//! Command line, header, and the vector loop.

use crate::call::{self, CallFrame, Stopped};
use crate::image::{self, Baseline, MappedImage};
use nv_oracle_core::abi::Place;
use nv_oracle_core::hex;
use nv_oracle_core::json::{self, ObjectWriter};
use nv_oracle_core::vectors::{CallResult, Fault, RegsOut, Vector, DEFAULT_FPCW, DEFAULT_MXCSR};
use nv_win as win;
use std::collections::HashMap;
use std::ffi::c_void;
use std::io::{BufRead, Write};
use std::path::PathBuf;

const USAGE: &str = "\
usage: nv-call <image.exe> <vectors.jsonl | -> [options]

Maps the PE32 image at its preferred base and runs each vector on the real
CPU, writing one JSON line per vector (after a header line).

options:
  --snapshot <dir>     apply <hexaddr>.bin region files after mapping
  --resolve-imports    fill the import table from this system's DLLs
                       (default: every import traps and ends the call)
  --keep-state         do not put the image's writable memory and the snapshot
                       regions back before each vector (default: every vector
                       starts from the state after mapping and snapshot, so a
                       result does not depend on the vectors before it)
  --fpcw <hex>         default x87 control word (default 0x027F)
  --mxcsr <hex>        default MXCSR (default 0x1F80)
  --out <file>         write results to a file instead of stdout
  --arena <hex>        preferred address of the buffer arena (default 0x30000000)
  -h, --help           this text
";

struct Options {
    image: PathBuf,
    vectors: String,
    snapshot: Option<PathBuf>,
    resolve_imports: bool,
    keep_state: bool,
    fpcw: u16,
    mxcsr: u32,
    out: Option<PathBuf>,
    arena: u32,
}

fn parse_args() -> Result<Options, String> {
    let mut args = std::env::args().skip(1);
    let mut positional = Vec::new();
    let mut o = Options {
        image: PathBuf::new(),
        vectors: String::new(),
        snapshot: None,
        resolve_imports: false,
        keep_state: false,
        fpcw: DEFAULT_FPCW,
        mxcsr: DEFAULT_MXCSR,
        out: None,
        arena: 0x3000_0000,
    };
    while let Some(a) = args.next() {
        let mut value = |name: &str| args.next().ok_or_else(|| format!("{name} needs a value"));
        match a.as_str() {
            "-h" | "--help" => {
                print!("{USAGE}");
                std::process::exit(0);
            }
            "--snapshot" => o.snapshot = Some(PathBuf::from(value("--snapshot")?)),
            "--resolve-imports" => o.resolve_imports = true,
            "--keep-state" => o.keep_state = true,
            "--fpcw" => {
                let v = hex::parse_u32(&value("--fpcw")?)?;
                o.fpcw = u16::try_from(v).map_err(|_| "--fpcw must fit 16 bits".to_string())?;
            }
            "--mxcsr" => o.mxcsr = hex::parse_u32(&value("--mxcsr")?)?,
            "--out" => o.out = Some(PathBuf::from(value("--out")?)),
            "--arena" => o.arena = hex::parse_u32(&value("--arena")?)?,
            s if s.starts_with("--") => return Err(format!("unknown option {s}")),
            _ => positional.push(a),
        }
    }
    if positional.len() != 2 {
        return Err("expected an image and a vectors file".into());
    }
    o.image = PathBuf::from(&positional[0]);
    o.vectors = positional[1].clone();
    if o.mxcsr & 0xffff_0000 != 0 {
        return Err("--mxcsr has reserved bits set".into());
    }
    Ok(o)
}

/// CPU vendor and brand string from CPUID.
fn cpu_info() -> (String, String) {
    use std::arch::x86::__cpuid;
    // SAFETY: CPUID exists on every CPU this program can run on.
    #[allow(unused_unsafe)]
    unsafe {
        let l0 = __cpuid(0);
        let mut vendor = Vec::new();
        for r in [l0.ebx, l0.edx, l0.ecx] {
            vendor.extend_from_slice(&r.to_le_bytes());
        }
        let mut brand = Vec::new();
        if __cpuid(0x8000_0000).eax >= 0x8000_0004 {
            for leaf in 0x8000_0002..=0x8000_0004u32 {
                let r = __cpuid(leaf);
                for x in [r.eax, r.ebx, r.ecx, r.edx] {
                    brand.extend_from_slice(&x.to_le_bytes());
                }
            }
        }
        let clean = |b: Vec<u8>| {
            String::from_utf8_lossy(&b)
                .trim_matches(|c: char| c == '\0' || c.is_whitespace())
                .to_string()
        };
        (clean(vendor), clean(brand))
    }
}

/// A bump allocator over a block of memory for vector buffers. Placed at a
/// fixed address when possible, so pointers stored in buffers repeat.
struct Arena {
    base: usize,
    size: usize,
    used: usize,
}

impl Arena {
    fn new(preferred: u32) -> Result<Arena, String> {
        const SIZE: usize = 64 * 1024 * 1024;
        let flags = win::MEM_RESERVE | win::MEM_COMMIT;
        // SAFETY: allocating fresh memory; results are checked.
        let mut p = unsafe {
            win::VirtualAlloc(
                preferred as usize as *mut c_void,
                SIZE,
                flags,
                win::PAGE_READWRITE,
            )
        };
        if p.is_null() {
            // SAFETY: as above, letting the system choose.
            p = unsafe {
                win::VirtualAlloc(std::ptr::null_mut(), SIZE, flags, win::PAGE_READWRITE)
            };
        }
        if p.is_null() {
            return Err("cannot allocate the buffer arena".into());
        }
        Ok(Arena {
            base: p as usize,
            size: SIZE,
            used: 0,
        })
    }

    fn reset(&mut self) {
        // SAFETY: zeroing the part that was handed out.
        unsafe { std::ptr::write_bytes(self.base as *mut u8, 0, self.used) };
        self.used = 0;
    }

    fn alloc(&mut self, len: usize) -> Result<u32, String> {
        let start = (self.used + 15) & !15;
        let end = start
            .checked_add(len)
            .filter(|&e| e <= self.size)
            .ok_or("buffers do not fit in the arena")?;
        self.used = end;
        Ok((self.base + start) as u32)
    }
}

fn header_line(
    opts: &Options,
    img: &MappedImage,
    snapshot: &image::Snapshot,
    arena: &Arena,
    baseline: &Baseline,
) -> String {
    let (vendor, brand) = cpu_info();
    let mut o = ObjectWriter::new();
    o.str("type", "header")
        .str("tool", "nv-call")
        .str("version", env!("CARGO_PKG_VERSION"));
    o.str("cpu_vendor", &vendor).str("cpu_brand", &brand);
    o.str("image", &opts.image.display().to_string())
        .str("image_sha256", &img.sha256);
    o.hex32("image_base", img.pe.image_base)
        .u64("size_of_image", u64::from(img.pe.size_of_image));
    o.hex32("fpcw", u32::from(opts.fpcw))
        .hex32("mxcsr", opts.mxcsr);
    o.u64("imports", img.import_names.len() as u64)
        .bool("imports_resolved", img.imports_resolved);
    o.u64("imports_unresolved", img.unresolved as u64);
    o.hex32("arena_base", arena.base as u32);
    o.bool("state_restored", !opts.keep_state)
        .u64("state_bytes", baseline.size() as u64);
    // True when the image has a TLS directory whose entry is zeroed in the
    // copy of the headers in memory (the only change to the image's bytes).
    o.bool("tls_directory_cleared", img.tls_cleared);
    match img.placement {
        image::Placement::HostPlaceholder {
            base,
            size,
            array_start,
        } => {
            // The range the host reserved for the image: from its own base to
            // the end of its placeholder array, which begins at `array_start`.
            let mut x = ObjectWriter::new();
            x.hex32("base", base)
                .u64("size", u64::from(size))
                .hex32("array_start", array_start);
            o.raw("placeholder", &x.finish());
        }
        image::Placement::Allocated => {
            o.null("placeholder");
        }
    }
    if opts.snapshot.is_some() {
        let regions: Vec<String> = snapshot
            .regions
            .iter()
            .map(|r| {
                let mut x = ObjectWriter::new();
                x.hex32("addr", r.addr)
                    .u64("size", r.size as u64)
                    .str("sha256", &r.sha256);
                x.finish()
            })
            .collect();
        o.raw("snapshot_regions", &json::array_of(&regions));
    } else {
        o.null("snapshot_regions");
    }
    o.finish()
}

fn error_line(line: usize, id: Option<&str>, msg: &str) -> String {
    let mut o = ObjectWriter::new();
    o.str("type", "error").u64("line", line as u64);
    if let Some(id) = id {
        o.str("id", id);
    }
    o.str("error", msg);
    o.finish()
}

fn execute(
    v: &Vector,
    opts: &Options,
    img: &MappedImage,
    arena: &mut Arena,
    baseline: Option<&Baseline>,
) -> Result<CallResult, String> {
    let layout = v.layout()?;
    if let Some(b) = baseline {
        b.restore();
    }
    arena.reset();
    let mut addrs: HashMap<String, u32> = HashMap::new();
    for b in &v.buffers {
        let a = arena.alloc(b.size)?;
        // SAFETY: freshly allocated arena bytes, `init.len() <= size`.
        unsafe {
            std::ptr::copy_nonoverlapping(b.init.as_ptr(), a as usize as *mut u8, b.init.len())
        };
        addrs.insert(b.name.clone(), a);
    }
    let resolve = |name: &str| addrs.get(name).copied();

    let mut stack = vec![0u32; layout.stack_dwords];
    let mut frame = CallFrame::new();
    for (arg, place) in v.args.iter().zip(&layout.places) {
        let dwords = arg.dwords(&resolve)?;
        match place {
            Place::Ecx => frame.regs_in[1] = dwords[0],
            Place::Edx => frame.regs_in[2] = dwords[0],
            Place::Stack(off) => stack[*off..*off + dwords.len()].copy_from_slice(&dwords),
        }
    }
    let uses_ecx = layout.places.contains(&Place::Ecx);
    let uses_edx = layout.places.contains(&Place::Edx);
    let presets = [
        (0, v.regs.eax, false, "eax"),
        (1, v.regs.ecx, uses_ecx, "ecx"),
        (2, v.regs.edx, uses_edx, "edx"),
        (3, v.regs.ebx, false, "ebx"),
        (4, v.regs.esi, false, "esi"),
        (5, v.regs.edi, false, "edi"),
    ];
    for (slot, preset, taken, name) in presets {
        if let Some(x) = preset {
            if taken {
                return Err(format!(
                    "regs.{name} conflicts with the {} argument registers",
                    v.cc
                ));
            }
            frame.regs_in[slot] = x;
        }
    }
    for (i, x) in v.regs.xmm.iter().enumerate() {
        if let Some(bytes) = x {
            frame.xmm_in[i] = *bytes;
        }
    }
    let fpcw = v.fpcw.unwrap_or(opts.fpcw);
    let mxcsr = v.mxcsr.unwrap_or(opts.mxcsr);
    if mxcsr & 0xffff_0000 != 0 {
        return Err("mxcsr has reserved bits set".into());
    }
    frame.target = v.addr;
    frame.nstack = stack.len() as u32;
    frame.stack = stack.as_ptr();
    frame.fpcw = u32::from(fpcw);
    frame.mxcsr = mxcsr;
    frame.want_st0 = u32::from(v.ret.uses_st0());

    // SAFETY: the target is an address in this process chosen by the user;
    // the stack vector outlives the call. Faults are recovered by the handler.
    let stopped = unsafe { call::run(&mut frame) };

    let (fault, regs) = match stopped {
        Some(Stopped::Exception(f)) => (Some(f), None),
        Some(Stopped::Import { index, caller }) => {
            let name = img
                .import_names
                .get(index)
                .cloned()
                .unwrap_or_else(|| format!("import #{index}"));
            (
                Some(Fault::ImportTrap {
                    index,
                    name,
                    caller,
                }),
                None,
            )
        }
        None => {
            let top = (frame.fsw >> 11) & 7;
            let depth = (8 - top) & 7;
            let st0 = (v.ret.uses_st0() && depth > 0).then(|| {
                let mut raw = [0u8; 10];
                raw.copy_from_slice(&frame.st0[..10]);
                raw
            });
            (
                None,
                Some(RegsOut {
                    eax: frame.eax,
                    edx: frame.edx,
                    st0,
                    fpu_depth: depth,
                    xmm0: frame.xmm0,
                    fsw: frame.fsw as u16,
                    fcw: frame.fcw as u16,
                    mxcsr: frame.mxcsr_out,
                    callee_popped: frame.esp_after.wrapping_sub(frame.esp_before),
                }),
            )
        }
    };
    let mut buffers = Vec::new();
    for b in &v.buffers {
        let a = addrs[&b.name] as usize as *const u8;
        // SAFETY: the buffer lives in the arena, `size` bytes.
        let bytes = unsafe { std::slice::from_raw_parts(a, b.size) }.to_vec();
        buffers.push((b.name.clone(), bytes));
    }
    Ok(CallResult {
        id: v.id.clone(),
        addr: v.addr,
        cc: v.cc,
        ret: v.ret,
        fpcw,
        mxcsr,
        fault,
        regs,
        stack_bytes: (stack.len() * 4) as u32,
        buffers,
    })
}

pub fn main(placeholder: image::Placeholder) -> i32 {
    let opts = match parse_args() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("nv-call: {e}\n\n{USAGE}");
            return 2;
        }
    };
    let img = match image::map(&opts.image, opts.resolve_imports, placeholder) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("nv-call: {e}");
            return 1;
        }
    };
    let snapshot = match &opts.snapshot {
        Some(dir) => match image::apply_snapshot(dir, &img) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("nv-call: {e}");
                return 1;
            }
        },
        None => image::Snapshot::default(),
    };
    // Everything the called code can change, as it is now. Taken before the
    // arena is allocated, so the arena is not part of it (it is reset on its own).
    let baseline = Baseline::capture(&img, &snapshot);
    let mut arena = match Arena::new(opts.arena) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("nv-call: {e}");
            return 1;
        }
    };
    let input: Box<dyn BufRead> = if opts.vectors == "-" {
        Box::new(std::io::BufReader::new(std::io::stdin()))
    } else {
        match std::fs::File::open(&opts.vectors) {
            Ok(f) => Box::new(std::io::BufReader::new(f)),
            Err(e) => {
                eprintln!("nv-call: cannot open {}: {e}", opts.vectors);
                return 1;
            }
        }
    };
    let mut out: Box<dyn Write> = match &opts.out {
        Some(p) => match std::fs::File::create(p) {
            Ok(f) => Box::new(std::io::BufWriter::new(f)),
            Err(e) => {
                eprintln!("nv-call: cannot create {}: {e}", p.display());
                return 1;
            }
        },
        None => Box::new(std::io::stdout()),
    };
    let mut emit = |line: &str| {
        let _ = writeln!(out, "{line}");
        let _ = out.flush();
    };
    emit(&header_line(&opts, &img, &snapshot, &arena, &baseline));
    let restore = (!opts.keep_state).then_some(&baseline);

    let mut errors = 0;
    for (i, line) in input.lines().enumerate() {
        let line_no = i + 1;
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                emit(&error_line(
                    line_no,
                    None,
                    &format!("cannot read input: {e}"),
                ));
                errors += 1;
                break;
            }
        };
        let text = line.trim();
        if text.is_empty() || text.starts_with('#') {
            continue;
        }
        match Vector::parse(text) {
            Err(e) => {
                emit(&error_line(line_no, None, &e));
                errors += 1;
            }
            Ok(v) => match execute(&v, &opts, &img, &mut arena, restore) {
                Ok(r) => emit(&r.to_json()),
                Err(e) => {
                    emit(&error_line(line_no, Some(&v.id), &e));
                    errors += 1;
                }
            },
        }
    }
    if errors > 0 {
        2
    } else {
        0
    }
}
