//! Mapping a PE32 image at its preferred base, with import trap stubs and
//! optional memory snapshots.

use crate::call;
use nv_oracle_core::pe::{Import, ImportName, Pe};
use nv_oracle_core::sha256;
use nv_win as win;
use std::ffi::c_void;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

/// A mapped image and what was done to its import table.
pub struct MappedImage {
    pub pe: Pe,
    pub sha256: String,
    /// Display names (`dll!name`) indexed by the trap index.
    pub import_names: Vec<String>,
    pub imports_resolved: bool,
    /// Imports that could not be resolved even though resolving was asked for.
    pub unresolved: usize,
    /// How the address range was obtained.
    pub placement: Placement,
    /// The image has a TLS directory, and the copy of its headers in memory
    /// has that entry zeroed (see [`map`]).
    pub tls_cleared: bool,
}

/// The array that `nv-call.exe` reserves for the image, as the host reports
/// it: address and length. Only this array is free for the image to use; the
/// host's own sections around it are not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placeholder {
    pub start: u32,
    pub len: u32,
}

/// Where the image's address range came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Placement {
    /// The range was reserved by the host program: from the host's base to
    /// the end of its placeholder array (see nv-call). The host's own code in
    /// front of the array is replaced by the image; `array_start` is where
    /// the array itself begins.
    HostPlaceholder {
        base: u32,
        size: u32,
        array_start: u32,
    },
    /// The range was free and was allocated with `VirtualAlloc`.
    Allocated,
}

/// The system hands out address space in blocks of this size.
const GRANULARITY: u64 = 0x1_0000;

fn round_up(v: u64, unit: u64) -> u64 {
    (v + unit - 1) & !(unit - 1)
}

/// Sort ranges and join those that touch or overlap.
fn merge_ranges(mut v: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    v.retain(|(a, b)| b > a);
    v.sort_unstable();
    let mut out: Vec<(u64, u64)> = Vec::new();
    for (a, b) in v {
        match out.last_mut() {
            Some(last) if a <= last.1 => last.1 = last.1.max(b),
            _ => out.push((a, b)),
        }
    }
    out
}

impl MappedImage {
    /// The address ranges that belong to the mapped program: its own pages
    /// and, with the host placeholder, the rest of the placeholder array
    /// (zero pages that nothing else uses). Snapshot regions may be written
    /// there. The host's other sections (its imports, its TLS template and
    /// startup tables behind the array, or what is left of its code in front
    /// of a small image) are the harness's and are not part of it. An image
    /// that the engine allocated owns the 64 KiB blocks it was given.
    fn owned_ranges(&self) -> Vec<(u64, u64)> {
        let image = (
            u64::from(self.pe.image_base),
            u64::from(self.pe.image_base) + u64::from(self.pe.size_of_image),
        );
        match self.placement {
            Placement::HostPlaceholder {
                base,
                size,
                array_start,
            } => merge_ranges(vec![
                image,
                (u64::from(array_start), u64::from(base) + u64::from(size)),
            ]),
            Placement::Allocated => vec![(image.0, round_up(image.1, GRANULARITY))],
        }
    }
}

static TOOK_OVER: AtomicBool = AtomicBool::new(false);

/// Exit status of the process when an exception outside the harness's reach
/// ends it (see [`install_top_level_filter`]).
pub const EXIT_UNHANDLED: u32 = 4;

fn push_text(buf: &mut [u8], n: &mut usize, text: &[u8]) {
    let take = text.len().min(buf.len() - *n);
    buf[*n..*n + take].copy_from_slice(&text[..take]);
    *n += take;
}

fn push_hex(buf: &mut [u8], n: &mut usize, v: u32) {
    let mut digits = *b"0x00000000";
    for i in 0..8 {
        digits[2 + i] = b"0123456789ABCDEF"[((v >> (28 - 4 * i)) & 15) as usize];
    }
    push_text(buf, n, &digits);
}

/// The process's top-level exception filter while an image is mapped. It says
/// what happened and ends the process.
///
/// The host program's startup code registers a filter of its own, and that
/// function lives in pages that the image replaces. Without this one, an
/// exception that reaches the top level (on a thread that the called code
/// created, where the harness's handler record is not on the chain) would run
/// whatever bytes the image has at that address. No allocation and no locks:
/// the thread that got here may hold either.
unsafe extern "system" fn top_level_filter(info: *const win::ExceptionPointers) -> i32 {
    let mut buf = [0u8; 256];
    let mut n = 0;
    push_text(&mut buf, &mut n, b"nv-call: unhandled exception ");
    let rec = (*info).exception_record;
    if rec.is_null() {
        push_text(&mut buf, &mut n, b"(no record)");
    } else {
        push_hex(&mut buf, &mut n, (*rec).exception_code);
        push_text(&mut buf, &mut n, b" at ");
        push_hex(&mut buf, &mut n, (*rec).exception_address as usize as u32);
    }
    push_text(
        &mut buf,
        &mut n,
        b" outside the harness's reach (a thread the called code created, or a damaged handler chain); ending the process\r\n",
    );
    let mut written = 0u32;
    win::WriteFile(
        win::GetStdHandle(win::STD_ERROR_HANDLE),
        buf.as_ptr().cast(),
        n as u32,
        &mut written,
        std::ptr::null_mut(),
    );
    win::TerminateProcess(win::GetCurrentProcess(), EXIT_UNHANDLED);
    0
}

/// Replace the process's top-level exception filter with the engine's own.
fn install_top_level_filter() {
    // SAFETY: registers a function of this DLL, which stays loaded.
    unsafe { win::SetUnhandledExceptionFilter(Some(top_level_filter)) };
}

/// True once the host program's pages have been replaced by the image. From
/// then on the host's code no longer exists and the process must be ended
/// with `TerminateProcess`.
pub fn took_over_host() -> bool {
    TOOK_OVER.load(SeqCst)
}

/// Base and size of the running program's own image (read from its headers
/// in memory, before anything overwrites them).
fn host_image() -> Option<(u32, u32)> {
    // SAFETY: GetModuleHandleW(NULL) is the base of the running exe, whose
    // headers are mapped and were validated by the loader.
    unsafe {
        let base = win::GetModuleHandleW(std::ptr::null()) as usize;
        if base == 0 {
            return None;
        }
        let read32 = |off: usize| std::ptr::read_unaligned((base + off) as *const u32);
        if std::ptr::read_unaligned(base as *const u16) != 0x5a4d {
            return None;
        }
        let pe = read32(0x3c) as usize;
        if read32(pe) != 0x0000_4550 {
            return None;
        }
        let size_of_image = read32(pe + 4 + 20 + 56);
        Some((base as u32, size_of_image))
    }
}

/// Make `[base, base + size)` of the host's placeholder writable and wipe
/// it, so the image sees the zeroed memory a fresh mapping would have. Only
/// the part the image needs is touched.
fn take_over_host(base: u32, size: u32) -> Result<(), String> {
    // Before anything is overwritten: the host registered code in these
    // pages as the process's exception filter.
    install_top_level_filter();
    let mut cur = u64::from(base) & !0xfff;
    let end = (u64::from(base) + u64::from(size) + 0xfff) & !0xfff;
    while cur < end {
        let mut info = win::MemoryBasicInformation::zeroed();
        // SAFETY: valid output buffer of the right size.
        let n = unsafe {
            win::VirtualQuery(
                cur as usize as *const c_void,
                &mut info,
                std::mem::size_of::<win::MemoryBasicInformation>(),
            )
        };
        if n == 0 {
            return Err(format!("cannot query the host placeholder at {cur:#010x}"));
        }
        if info.state != win::MEM_COMMIT {
            return Err(format!(
                "the host placeholder is not committed at {cur:#010x}: {}",
                describe_region(cur as u32)
            ));
        }
        let region_end = (info.base_address as usize as u64 + info.region_size as u64).min(end);
        let mut old = 0u32;
        // SAFETY: changing protection of committed pages of this process.
        let ok = unsafe {
            win::VirtualProtect(
                cur as usize as *mut c_void,
                (region_end - cur) as usize,
                win::PAGE_EXECUTE_READWRITE,
                &mut old,
            )
        };
        if ok == 0 {
            return Err(format!(
                "cannot make the host placeholder writable at {cur:#010x}"
            ));
        }
        cur = region_end;
    }
    // SAFETY: the whole range was made writable above. The host's code in
    // it is never run again.
    unsafe { std::ptr::write_bytes(base as usize as *mut u8, 0, size as usize) };
    TOOK_OVER.store(true, SeqCst);
    Ok(())
}

fn describe_region(addr: u32) -> String {
    let mut info = win::MemoryBasicInformation::zeroed();
    // SAFETY: valid output buffer of the right size.
    let n = unsafe {
        win::VirtualQuery(
            addr as usize as *const c_void,
            &mut info,
            std::mem::size_of::<win::MemoryBasicInformation>(),
        )
    };
    if n == 0 {
        return "the region could not be queried".into();
    }
    let state = match info.state {
        win::MEM_COMMIT => "committed",
        win::MEM_RESERVE => "reserved",
        win::MEM_FREE => "free",
        _ => "unknown state",
    };
    format!(
        "region at {:#010x}, {:#x} bytes, {} (type {:#x})",
        info.base_address as usize, info.region_size, state, info.kind
    )
}

/// Reserve and commit `[base, base + size)` exactly, as read-write-execute.
fn alloc_at(base: u32, size: usize) -> Result<(), String> {
    // SAFETY: asking the OS for a specific address range; the result is checked.
    let got = unsafe {
        win::VirtualAlloc(
            base as usize as *mut c_void,
            size,
            win::MEM_RESERVE | win::MEM_COMMIT,
            win::PAGE_EXECUTE_READWRITE,
        )
    };
    if got as usize == base as usize {
        return Ok(());
    }
    if !got.is_null() {
        // SAFETY: freeing the block just allocated.
        unsafe { win::VirtualFree(got, 0, win::MEM_RELEASE) };
    }
    // SAFETY: plain query.
    let code = unsafe { win::GetLastError() };
    Err(format!(
        "cannot reserve {size:#x} bytes at {base:#010x}: {} ({}). Another allocation is in the way: {}. \
         On a normal run nv-call's own placeholder reserves this range; getting here means the image's \
         base is outside the placeholder (see the README: NV_CALL_RESERVE_MB), or the range is taken by \
         something else.",
        win::error_text(code),
        code,
        describe_region(base)
    ))
}

fn inside(ranges: &[(u64, u64)], start: u64, end: u64) -> bool {
    ranges.iter().any(|r| r.0 <= start && end <= r.1)
}

/// Make `[addr, addr + len)` committed and writable, allocating at that
/// exact address where nothing is there yet. Used for snapshot regions.
///
/// Memory that is already in use is only touched when it belongs to the
/// mapped program (`owned`) or was allocated for an earlier snapshot region
/// (`made`, which this adds to). Anything else is the harness's own memory
/// (its code and data, heaps, stacks, the buffer arena) or something the
/// called code could reach through another route: overwriting it would
/// corrupt the harness, so it is an error that names what is there.
fn ensure_writable(
    addr: u32,
    len: usize,
    owned: &[(u64, u64)],
    made: &mut Vec<(u64, u64)>,
) -> Result<(), String> {
    let end = u64::from(addr) + len as u64;
    if end > 0x1_0000_0000 {
        return Err(format!(
            "region {addr:#010x}+{len:#x} runs past the 4 GiB address space"
        ));
    }
    let mut cur = u64::from(addr);
    while cur < end {
        let mut info = win::MemoryBasicInformation::zeroed();
        // SAFETY: valid output buffer of the right size.
        let n = unsafe {
            win::VirtualQuery(
                cur as usize as *const c_void,
                &mut info,
                std::mem::size_of::<win::MemoryBasicInformation>(),
            )
        };
        if n == 0 {
            return Err(format!("VirtualQuery failed at {cur:#010x}"));
        }
        let region_end = (info.base_address as usize as u64 + info.region_size as u64).min(end);
        let page_start = cur & !0xfff;
        let span = (region_end - page_start + 0xfff) & !0xfff;
        if info.state != win::MEM_FREE
            && !inside(owned, page_start, page_start + span)
            && !inside(made, page_start, page_start + span)
        {
            return Err(format!(
                "snapshot region {addr:#010x}+{len:#x} overlaps memory that is not part of the mapped image \
                 or free address space: {}. A snapshot region may lie inside the image, in the unused rest \
                 of the placeholder array, in free address space, or in pages that earlier snapshot files \
                 allocated.",
                describe_region(cur as u32)
            ));
        }
        match info.state {
            win::MEM_FREE => {
                // Allocations start on 64 KiB boundaries. Take the whole block
                // (as far as the free range reaches), not just the pages
                // needed: a later file in the same block then finds it ours,
                // where a smaller allocation would leave the rest of the
                // block unusable.
                let aligned = cur & !(GRANULARITY - 1);
                let free_start = info.base_address as usize as u64;
                if aligned < free_start {
                    // The free pages begin partway through a block whose first
                    // part belongs to something else. The system will not
                    // allocate there, although the pages are reported free.
                    return Err(format!(
                        "snapshot region {addr:#010x}+{len:#x} starts in the unused tail of a 64 KiB block: the pages from \
                         {free_start:#010x} are free, but address space is handed out in 64 KiB units and the block at \
                         {aligned:#010x} already belongs to another allocation ({}). Start the region at \
                         {:#010x} or later.",
                        describe_region(aligned as u32),
                        aligned + GRANULARITY
                    ));
                }
                let free_end = free_start + info.region_size as u64;
                let span = round_up(region_end, GRANULARITY).min(free_end) - aligned;
                // SAFETY: allocating a free range; result is checked.
                let got = unsafe {
                    win::VirtualAlloc(
                        aligned as usize as *mut c_void,
                        span as usize,
                        win::MEM_RESERVE | win::MEM_COMMIT,
                        win::PAGE_EXECUTE_READWRITE,
                    )
                };
                if got.is_null() {
                    return Err(format!(
                        "cannot allocate {span:#x} bytes at {aligned:#010x}: {}",
                        describe_region(aligned as u32)
                    ));
                }
                made.push((aligned, aligned + span));
            }
            win::MEM_RESERVE => {
                // SAFETY: committing inside a reservation of the mapped program.
                let got = unsafe {
                    win::VirtualAlloc(
                        page_start as usize as *mut c_void,
                        span as usize,
                        win::MEM_COMMIT,
                        win::PAGE_EXECUTE_READWRITE,
                    )
                };
                if got.is_null() {
                    return Err(format!(
                        "cannot commit {span:#x} bytes at {page_start:#010x}"
                    ));
                }
            }
            _ => {
                if info.protect != win::PAGE_EXECUTE_READWRITE {
                    let mut old = 0u32;
                    // SAFETY: changing protection on committed pages of this process.
                    let ok = unsafe {
                        win::VirtualProtect(
                            page_start as usize as *mut c_void,
                            span as usize,
                            win::PAGE_EXECUTE_READWRITE,
                            &mut old,
                        )
                    };
                    if ok == 0 {
                        return Err(format!("cannot make {page_start:#010x} writable"));
                    }
                }
            }
        }
        cur = region_end;
    }
    Ok(())
}

fn load_symbol(imp: &Import) -> Option<u32> {
    let dll: Vec<u8> = imp.dll.bytes().chain(std::iter::once(0)).collect();
    // SAFETY: NUL-terminated name passed to the loader.
    let module = unsafe { win::LoadLibraryA(dll.as_ptr()) };
    if module.is_null() {
        return None;
    }
    let addr = match &imp.name {
        ImportName::Name(n) => {
            let name: Vec<u8> = n.bytes().chain(std::iter::once(0)).collect();
            // SAFETY: valid module handle and NUL-terminated name.
            unsafe { win::GetProcAddress(module, name.as_ptr()) }
        }
        // SAFETY: an ordinal is passed as a pointer-sized integer.
        ImportName::Ordinal(o) => unsafe {
            win::GetProcAddress(module, usize::from(*o) as *const u8)
        },
    };
    (!addr.is_null()).then_some(addr as usize as u32)
}

/// Map the image at `path` at its preferred base.
///
/// Imports are left unresolved: every IAT slot points at a trap stub that
/// ends the current call and reports the import's name. With
/// `resolve_imports`, slots are filled with the real addresses from the
/// running system's DLLs instead (those that cannot be found keep a stub).
///
/// The image's own TLS directory is never given to the loader: the loader reads
/// an executable's TLS directory from the headers in memory each time a
/// thread starts or ends, and the image's callbacks would then run inside the
/// harness whenever the called code (or the system) starts a thread. So the
/// copy of the headers in memory has that directory entry zeroed
/// ([`MappedImage::tls_cleared`]); no other byte of the image is changed.
pub fn map(
    path: &Path,
    resolve_imports: bool,
    placeholder: Placeholder,
) -> Result<MappedImage, String> {
    let data = std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let digest = sha256::hex_digest(&data);
    let pe = Pe::parse(&data).map_err(|e| format!("{}: {e}", path.display()))?;
    let base = pe.image_base;
    let image_end = u64::from(base) + u64::from(pe.size_of_image);
    let array_end = u64::from(placeholder.start) + u64::from(placeholder.len);
    let placement = match host_image() {
        // The host reserved [its base, the end of its array) for the image.
        Some((hb, hs))
            if hb <= placeholder.start
                && array_end <= u64::from(hb) + u64::from(hs)
                && base >= hb
                && image_end <= array_end =>
        {
            take_over_host(base, pe.size_of_image)?;
            Placement::HostPlaceholder {
                base: hb,
                size: (array_end - u64::from(hb)) as u32,
                array_start: placeholder.start,
            }
        }
        Some((hb, hs))
            if u64::from(base) < u64::from(hb) + u64::from(hs) && image_end > u64::from(hb) =>
        {
            return Err(format!(
                "the image wants {base:#010x}-{image_end:#010x}, which overlaps the program's own image \
                 ({hb:#010x}-{:#010x}) without fitting inside the placeholder ({hb:#010x}-{array_end:#010x}). \
                 Rebuild nv-call with a larger NV_CALL_RESERVE_MB (it is {} MiB now).",
                u64::from(hb) + u64::from(hs),
                placeholder.len >> 20
            ));
        }
        _ => {
            // Whole 64 KiB blocks, so the pages after the image are not left
            // as an unusable tail of its block.
            alloc_at(
                base,
                round_up(u64::from(pe.size_of_image), GRANULARITY) as usize,
            )?;
            Placement::Allocated
        }
    };
    let mem = base as usize as *mut u8;

    // SAFETY: the whole range was just committed; every copy is checked
    // against the image size and the file length by `Pe::parse`.
    let mut tls_cleared = false;
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), mem, pe.size_of_headers as usize);
        if let Some(at) = pe.tls_dir_entry {
            if pe.tls_dir != (0, 0) && at + 8 <= pe.size_of_headers as usize {
                std::ptr::write_bytes(mem.add(at), 0, 8);
                tls_cleared = true;
            }
        }
        for s in &pe.sections {
            let n = if s.virtual_size == 0 {
                s.raw_size
            } else {
                s.raw_size.min(s.virtual_size)
            } as usize;
            if n > 0 {
                std::ptr::copy_nonoverlapping(
                    data.as_ptr().add(s.raw_offset as usize),
                    mem.add(s.virtual_address as usize),
                    n,
                );
            }
        }
    }

    let imports = pe.imports(&data)?;
    let mut import_names = Vec::with_capacity(imports.len());
    let mut unresolved = 0;
    if !imports.is_empty() {
        // One 16-byte stub per import: mov eax, index / mov ecx, trap / jmp ecx.
        let stub_size = imports.len() * 16;
        // SAFETY: fresh allocation anywhere.
        let stubs = unsafe {
            win::VirtualAlloc(
                std::ptr::null_mut(),
                stub_size,
                win::MEM_RESERVE | win::MEM_COMMIT,
                win::PAGE_EXECUTE_READWRITE,
            )
        } as *mut u8;
        if stubs.is_null() {
            return Err("cannot allocate import stubs".into());
        }
        let trap = call::trap_address();
        for (i, imp) in imports.iter().enumerate() {
            import_names.push(imp.to_string());
            // SAFETY: `i * 16 + 12 <= stub_size`.
            let stub = unsafe { stubs.add(i * 16) };
            let mut code = [0u8; 12];
            code[0] = 0xb8;
            code[1..5].copy_from_slice(&(i as u32).to_le_bytes());
            code[5] = 0xb9;
            code[6..10].copy_from_slice(&trap.to_le_bytes());
            code[10] = 0xff;
            code[11] = 0xe1;
            // SAFETY: inside the stub allocation.
            unsafe { std::ptr::copy_nonoverlapping(code.as_ptr(), stub, code.len()) };
            let target = if resolve_imports {
                load_symbol(imp).unwrap_or_else(|| {
                    unresolved += 1;
                    stub as usize as u32
                })
            } else {
                stub as usize as u32
            };
            if imp.iat_rva as u64 + 4 > u64::from(pe.size_of_image) {
                return Err(format!(
                    "import {imp} has its address-table slot outside the image"
                ));
            }
            // SAFETY: the slot is inside the committed image.
            unsafe { std::ptr::write_unaligned(mem.add(imp.iat_rva as usize) as *mut u32, target) };
        }
    }
    Ok(MappedImage {
        pe,
        sha256: digest,
        import_names,
        imports_resolved: resolve_imports,
        unresolved,
        placement,
        tls_cleared,
    })
}

/// One applied snapshot region.
pub struct SnapshotRegion {
    pub addr: u32,
    pub size: usize,
    pub sha256: String,
}

/// What applying a snapshot folder did.
#[derive(Default)]
pub struct Snapshot {
    pub regions: Vec<SnapshotRegion>,
    /// Address ranges allocated for the regions (outside the image).
    allocated: Vec<(u64, u64)>,
}

/// Apply every `<hexaddr>.bin` file in `dir` to memory. Regions outside the
/// image are allocated at their exact addresses; see [`ensure_writable`] for
/// what a region may overlap. Two files must not cover the same bytes.
pub fn apply_snapshot(dir: &Path, img: &MappedImage) -> Result<Snapshot, String> {
    let mut files = Vec::new();
    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("cannot read snapshot folder {}: {e}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("bin") {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let digits = stem
            .strip_prefix("0x")
            .or_else(|| stem.strip_prefix("0X"))
            .unwrap_or(stem);
        let addr = u32::from_str_radix(digits, 16).map_err(|_| {
            format!(
                "snapshot file {} is not named <hexaddr>.bin",
                path.display()
            )
        })?;
        files.push((addr, path));
    }
    files.sort();
    let owned = img.owned_ranges();
    let mut made: Vec<(u64, u64)> = Vec::new();
    let mut applied: Vec<SnapshotRegion> = Vec::new();
    for (addr, path) in files {
        let bytes =
            std::fs::read(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        if bytes.is_empty() {
            continue;
        }
        let end = u64::from(addr) + bytes.len() as u64;
        if let Some(prev) = applied.iter().find(|r| {
            u64::from(addr) < u64::from(r.addr) + r.size as u64 && end > u64::from(r.addr)
        }) {
            return Err(format!(
                "snapshot file {} ({addr:#010x}+{:#x}) overlaps the region {:#010x}+{:#x} of another file",
                path.display(),
                bytes.len(),
                prev.addr,
                prev.size
            ));
        }
        ensure_writable(addr, bytes.len(), &owned, &mut made)?;
        // SAFETY: the range was just made committed and writable.
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as usize as *mut u8, bytes.len())
        };
        applied.push(SnapshotRegion {
            addr,
            size: bytes.len(),
            sha256: sha256::hex_digest(&bytes),
        });
    }
    Ok(Snapshot {
        regions: applied,
        allocated: made,
    })
}

/// `IMAGE_SCN_MEM_WRITE`.
const SECTION_WRITABLE: u32 = 0x8000_0000;

/// A copy of the memory the called code can change, taken after the image is
/// mapped and the snapshot applied: the image's writable sections, the pages
/// of every snapshot region and everything that was allocated for the
/// snapshot. Putting it back before each vector makes a result independent of
/// the vectors that ran before it (a counter, or a run-once initialiser flag,
/// would otherwise carry over).
///
/// Memory that the called code allocates itself (through resolved imports)
/// and read-only parts of the image are not covered.
pub struct Baseline {
    ranges: Vec<(u32, Vec<u8>)>,
}

impl Baseline {
    pub fn capture(img: &MappedImage, snapshot: &Snapshot) -> Baseline {
        let image_end = u64::from(img.pe.image_base) + u64::from(img.pe.size_of_image);
        let mut spans: Vec<(u64, u64)> = Vec::new();
        for s in &img.pe.sections {
            if s.characteristics & SECTION_WRITABLE == 0 {
                continue;
            }
            let len = if s.virtual_size == 0 {
                s.raw_size
            } else {
                s.virtual_size
            };
            let start = u64::from(img.pe.image_base) + u64::from(s.virtual_address);
            let end = ((start + u64::from(len) + 0xfff) & !0xfff).min(image_end);
            spans.push((start, end));
        }
        for r in &snapshot.regions {
            let start = u64::from(r.addr) & !0xfff;
            let end = (u64::from(r.addr) + r.size as u64 + 0xfff) & !0xfff;
            spans.push((start, end));
        }
        spans.extend(snapshot.allocated.iter().copied());
        // Merge overlapping spans, so no byte is copied twice.
        spans.retain(|(a, b)| b > a);
        spans.sort_unstable();
        let mut merged: Vec<(u64, u64)> = Vec::new();
        for (a, b) in spans {
            match merged.last_mut() {
                Some(last) if a <= last.1 => last.1 = last.1.max(b),
                _ => merged.push((a, b)),
            }
        }
        let ranges = merged
            .into_iter()
            .map(|(a, b)| {
                // SAFETY: the pages were committed and made writable when the
                // image was mapped or the snapshot applied.
                let bytes = unsafe {
                    std::slice::from_raw_parts(a as usize as *const u8, (b - a) as usize)
                };
                (a as u32, bytes.to_vec())
            })
            .collect();
        Baseline { ranges }
    }

    /// Write every saved range back.
    pub fn restore(&self) {
        for (addr, bytes) in &self.ranges {
            // SAFETY: the same pages that were read when the copy was taken.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    *addr as usize as *mut u8,
                    bytes.len(),
                )
            };
        }
    }

    /// Bytes held (and written back before each vector).
    pub fn size(&self) -> usize {
        self.ranges.iter().map(|(_, b)| b.len()).sum()
    }
}
