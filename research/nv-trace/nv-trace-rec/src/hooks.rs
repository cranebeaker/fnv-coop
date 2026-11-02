//! The function hooks: each listed exe function's first instructions are replaced by a jump
//! to an entry stub ([`crate::stubs`]) that calls [`enter`]; the return address is swapped for
//! a return stub that calls [`leave`] (a per-thread shadow stack keeps the real one, as in
//! nv-probe's runtime). `enter` writes the call's words, `leave` its `EAX`, `EDX` and `ST0`,
//! both into the thread's ring ([`crate::ring`]).
//!
//! Also here: [`detour`], a plain jump to a Rust function with a trampoline for the original
//! (the guard's ntdll hooks).

use crate::ring::{self, Ring};
use crate::stubs::{self, fx, Region, SavedRegs, FRAME_SIZE};
use nv_oracle_core::decode;
use nv_oracle_core::x87::Ext80;
use nv_trace_core::format::{DEPTH_UNKNOWN, ENTER, LEAVE, STACK, THREAD};
use nv_trace_core::list::{Conv, FnSpec};
use nv_win as win;
use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, AtomicU32, AtomicU64, Ordering::*};

pub struct Hook {
    pub spec: FnSpec,
    pub ret_stub: u32,
    /// Calls seen (recorded or not, see `FnSpec::max` and `FnSpec::after`).
    pub calls: AtomicU64,
    /// Calls recorded.
    pub recorded: AtomicU64,
    /// The frame the recording opened at (`frames=N`); `u64::MAX` before.
    pub open_frame: AtomicU64,
}

pub struct Table {
    pub hooks: Vec<Hook>,
}

static TABLE: AtomicPtr<Table> = AtomicPtr::new(std::ptr::null_mut());
static TLS: AtomicU32 = AtomicU32::new(u32::MAX);
static SEQ: AtomicU32 = AtomicU32::new(0);
/// Calls of the list's `frame` function so far.
pub static FRAMES: AtomicU64 = AtomicU64::new(0);

fn table() -> Option<&'static Table> {
    // SAFETY: null or a leaked, never-freed table.
    unsafe { TABLE.load(Acquire).as_ref() }
}

struct Shadow {
    orig_ret: u32,
    hook: u32,
    entry_esp: u32,
    seq: u32,
    /// The call's words, kept for its return-time dumps (`rdump=`, `rhash=`).
    words: Vec<u32>,
}

struct ThreadState {
    ring: *mut Ring,
    shadow: Vec<Shadow>,
    busy: bool,
    record: Vec<u8>,
}

unsafe fn thread_state() -> *mut ThreadState {
    let idx = TLS.load(Acquire);
    if idx == u32::MAX {
        return std::ptr::null_mut();
    }
    let p = win::TlsGetValue(idx) as *mut ThreadState;
    if !p.is_null() {
        return p;
    }
    let ring = ring::new_ring();
    // The thread record: its start routine, so the diff can pair threads by what they run.
    let mut t = vec![THREAD, 0, 0, 0];
    t.extend_from_slice(&thread_start().to_le_bytes());
    t.extend_from_slice(&0u32.to_le_bytes());
    ring::push(&*ring, &t);
    let b = Box::into_raw(Box::new(ThreadState {
        ring,
        shadow: Vec::with_capacity(64),
        busy: false,
        record: Vec::with_capacity(512),
    }));
    win::TlsSetValue(idx, b as *mut c_void);
    b
}

fn read_u32(addr: u32) -> u32 {
    if win::is_readable(addr, 4) {
        // SAFETY: checked readable just now.
        unsafe { std::ptr::read_unaligned(addr as usize as *const u32) }
    } else {
        0xbad0_bad0
    }
}

/// Called by the entry stub.
///
/// # Safety
/// Only the generated stubs call this, with a frame they built.
pub unsafe extern "C" fn enter(id: u32, regs: *mut SavedRegs, _fx: *const u8) {
    let last_error = win::GetLastError();
    let ts = thread_state();
    if !ts.is_null() && !(*ts).busy {
        (*ts).busy = true;
        enter_impl(id, &mut *regs, &mut *ts);
        (*ts).busy = false;
    }
    win::SetLastError(last_error);
}

fn enter_impl(id: u32, regs: &mut SavedRegs, ts: &mut ThreadState) {
    let Some(hook) = table().and_then(|t| t.hooks.get(id as usize)) else {
        return;
    };
    let spec = &hook.spec;
    hook.calls.fetch_add(1, Relaxed);
    if spec.conv == Conv::Frame {
        crate::input::on_frame(FRAMES.fetch_add(1, Relaxed));
    }
    if crate::input::KEYS_AFTER
        .iter()
        .any(|k| k.load(Relaxed) == spec.addr)
    {
        crate::input::event(spec.addr);
    }
    // `after=ADDR:N`: nothing is recorded before the N-th call of ADDR was entered.
    if let Some((event, n)) = spec.after {
        let open = table()
            .and_then(|t| t.hooks.iter().find(|h| h.spec.addr == event))
            .is_some_and(|h| h.calls.load(Relaxed) >= n);
        if !open {
            return;
        }
    }
    // `frames=S..E`: only during frames S to E-1 counted from the first call let through.
    if let Some((first, end)) = spec.frames {
        let now = FRAMES.load(Relaxed);
        let _ = hook
            .open_frame
            .compare_exchange(u64::MAX, now, Relaxed, Relaxed);
        let rel = now.saturating_sub(hook.open_frame.load(Relaxed));
        if rel < first || rel >= end {
            return;
        }
    }
    let entry_esp = (regs as *mut SavedRegs as u32).wrapping_add(FRAME_SIZE);
    let n = spec.words.len();
    let regs_words = [regs.ecx, regs.edx];
    let words: Vec<u32> = (0..n)
        .map(|k| {
            if let Some(&w) = regs_words.get(k).filter(|_| k < spec.conv.reg_words()) {
                w
            } else {
                let slot = (k - spec.conv.reg_words()) as u32;
                read_u32(entry_esp.wrapping_add(4 + 4 * slot))
            }
        })
        .collect();
    // `when=`: only the calls that meet the conditions.
    if !spec.when.is_empty() {
        let mut read = |a: u32| win::is_readable(a, 4).then(|| read_u32(a));
        if !spec.when.iter().all(|c| c.holds(&words, &mut read)) {
            return;
        }
    }
    // `max=N` counts the calls recorded.
    let n_rec = hook.recorded.fetch_add(1, Relaxed);
    if spec.max.is_some_and(|m| n_rec >= m) {
        return;
    }
    let seq = SEQ.fetch_add(1, Relaxed);
    let r = &mut ts.record;
    r.clear();
    r.extend_from_slice(&[ENTER, n as u8, 0, 0]);
    r.extend_from_slice(&spec.addr.to_le_bytes());
    r.extend_from_slice(&seq.to_le_bytes());
    for w in &words {
        r.extend_from_slice(&w.to_le_bytes());
    }
    // SAFETY: the ring belongs to this thread.
    ring::push(unsafe { &*ts.ring }, r);
    // Words that point into this thread's stack: their offset from the caller's stack
    // pointer after its prologue (the entry stack pointer above the return address and the
    // stack arguments), so the diff can compare frame layouts.
    let (limit, base) = stack_bounds();
    let frame = entry_esp.wrapping_add(4 + 4 * spec.stack_words() as u32);
    let mut stack = Vec::new();
    for (k, &w) in words.iter().enumerate() {
        if w >= limit && w < base {
            stack.push((k as u8, w.wrapping_sub(frame) as i32));
        }
    }
    if !stack.is_empty() {
        let mut s = vec![STACK, stack.len() as u8, 0, 0];
        s.extend_from_slice(&seq.to_le_bytes());
        for (index, offset) in stack {
            s.extend_from_slice(&[index, DEPTH_UNKNOWN, 0, 0]);
            s.extend_from_slice(&offset.to_le_bytes());
        }
        // SAFETY: as above.
        ring::push(unsafe { &*ts.ring }, &s);
    }
    // The list's dumps at entry: memory at addresses computed from the words, pointers
    // followed.
    if spec.dumps.iter().any(|d| !d.at_return) {
        let dumps = take_dumps(spec, &words, false);
        let mut out = Vec::new();
        nv_trace_core::format::Record::Dump { seq, dumps }.encode(&mut out);
        // SAFETY: as above.
        ring::push(unsafe { &*ts.ring }, &out);
    }
    let keep_words = spec.dumps.iter().any(|d| d.at_return);
    if hook.ret_stub != 0 {
        while ts.shadow.last().is_some_and(|s| s.entry_esp < entry_esp) {
            ts.shadow.pop();
        }
        // SAFETY: the slot is the function's own return address on its stack.
        let orig = unsafe { std::ptr::read(entry_esp as usize as *const u32) };
        ts.shadow.push(Shadow {
            orig_ret: orig,
            hook: id,
            entry_esp,
            seq,
            words: if keep_words { words } else { Vec::new() },
        });
        // SAFETY: as above.
        unsafe { std::ptr::write(entry_esp as usize as *mut u32, hook.ret_stub) };
    }
}

/// Called by the return stub. Always fills in the original return address.
///
/// # Safety
/// Only the generated stubs call this, with a frame they built.
pub unsafe extern "C" fn leave(id: u32, regs: *mut SavedRegs, fxsave: *const u8) {
    let last_error = win::GetLastError();
    let regs = &mut *regs;
    let ts = thread_state();
    if ts.is_null() {
        fatal("return stub reached without thread state");
    }
    let ts = &mut *ts;
    let Some(hook) = table().and_then(|t| t.hooks.get(id as usize)) else {
        fatal("return stub for an unknown hook")
    };
    let stub_esp = (regs as *mut SavedRegs as u32).wrapping_add(FRAME_SIZE + 4);
    let pop = if matches!(hook.spec.conv, Conv::Cdecl) {
        0
    } else {
        4 * hook.spec.stack_words() as u32
    };
    let find = |exact: bool| {
        ts.shadow.iter().rposition(|s| {
            s.hook == id
                && if exact {
                    s.entry_esp.wrapping_add(4 + pop) == stub_esp
                } else {
                    s.entry_esp.wrapping_add(4) <= stub_esp
                }
        })
    };
    let Some(idx) = find(true).or_else(|| find(false)) else {
        fatal("no shadow stack entry for a return")
    };
    ts.shadow.truncate(idx + 1);
    let entry = ts.shadow.pop().expect("entry found above");
    regs.ret_addr = entry.orig_ret;

    if !ts.busy {
        ts.busy = true;
        let image = std::slice::from_raw_parts(fxsave, fx::SIZE);
        let st0 = if hook.spec.ret.x87() {
            let fsw = u16::from_le_bytes([image[fx::FSW], image[fx::FSW + 1]]);
            let top = (fsw >> 11) & 7;
            (image[fx::FTW] & (1 << top) != 0).then(|| {
                let mut raw = [0u8; 10];
                raw.copy_from_slice(&image[fx::ST0..fx::ST0 + 10]);
                Ext80::from_bytes(raw).to_f64_bits()
            })
        } else {
            None
        };
        let r = &mut ts.record;
        r.clear();
        r.extend_from_slice(&[LEAVE, u8::from(st0.is_some()), 0, 0]);
        r.extend_from_slice(&hook.spec.addr.to_le_bytes());
        r.extend_from_slice(&entry.seq.to_le_bytes());
        r.extend_from_slice(&regs.eax.to_le_bytes());
        r.extend_from_slice(&regs.edx.to_le_bytes());
        r.extend_from_slice(&st0.unwrap_or(0).to_le_bytes());
        ring::push(&*ts.ring, r);
        // The list's dumps at return, from the words the call was entered with.
        if hook.spec.dumps.iter().any(|d| d.at_return) {
            let dumps = take_dumps(&hook.spec, &entry.words, true);
            let mut out = Vec::new();
            nv_trace_core::format::Record::RDump {
                seq: entry.seq,
                dumps,
            }
            .encode(&mut out);
            ring::push(&*ts.ring, &out);
        }
        ts.busy = false;
    }
    win::SetLastError(last_error);
}

/// The list's dumps of one side (entry or return) for these words: bytes, or an FNV-1a hash
/// with the length hashed (`hash=`, `shader=`).
fn take_dumps(spec: &FnSpec, words: &[u32], at_return: bool) -> Vec<nv_trace_core::dump::Dumped> {
    use nv_trace_core::dump::{self, Dumped, Kind};
    let mut read = |a: u32| win::is_readable(a, 4).then(|| read_u32(a));
    spec.dumps
        .iter()
        .filter(|d| d.at_return == at_return)
        .map(|d| {
            let Some(addr) = d.expr.eval(words, &mut read) else {
                return Dumped {
                    addr: 0,
                    bytes: None,
                };
            };
            let bytes = match d.kind {
                Kind::Bytes => d
                    .len
                    .eval(words, dump::MAX_DUMP)
                    .and_then(|l| crate::memory_bytes(addr, l as usize)),
                Kind::Hash => d
                    .len
                    .eval(words, dump::MAX_HASH)
                    .and_then(|l| crate::memory_bytes(addr, l as usize))
                    .map(|b| dump::hash_payload(&b)),
                Kind::Shader => dump::shader_len(addr, &mut read)
                    .and_then(|l| crate::memory_bytes(addr, l as usize))
                    .map(|b| dump::shader_payload(&b)),
            };
            Dumped { addr, bytes }
        })
        .collect()
}

fn fatal(msg: &str) -> ! {
    crate::log(&format!("fatal: {msg}"));
    ring::finish(false);
    // SAFETY: ending this process.
    unsafe { win::TerminateProcess(win::GetCurrentProcess(), 0x4e56) };
    std::process::abort();
}

/// Writes `jmp target` over the first 5 bytes at `at`.
fn write_jump(at: u32, target: u32) -> Result<(), String> {
    let rel = target.wrapping_sub(at.wrapping_add(5));
    let mut jmp = [0xe9u8, 0, 0, 0, 0];
    jmp[1..].copy_from_slice(&rel.to_le_bytes());
    let mut old = 0u32;
    // SAFETY: this process's own code pages; restored right after.
    unsafe {
        if win::VirtualProtect(
            at as usize as *mut c_void,
            5,
            win::PAGE_EXECUTE_READWRITE,
            &mut old,
        ) == 0
        {
            return Err(format!("VirtualProtect at {at:#010x} failed"));
        }
        std::ptr::copy_nonoverlapping(jmp.as_ptr(), at as usize as *mut u8, 5);
        let mut ignored = 0;
        win::VirtualProtect(at as usize as *mut c_void, 5, old, &mut ignored);
        win::FlushInstructionCache(win::GetCurrentProcess(), at as usize as *const c_void, 5);
    }
    Ok(())
}

fn trampoline(at: u32, region: &mut Region) -> Result<u32, String> {
    if !win::is_readable(at, 16) {
        return Err(format!("{at:#010x} is not readable"));
    }
    let mut live = [0u8; 16];
    // SAFETY: checked readable.
    unsafe { std::ptr::copy_nonoverlapping(at as usize as *const u8, live.as_mut_ptr(), 16) };
    let steal = decode::auto_steal(&live).map_err(|e| e.to_string())?;
    let tramp = region.alloc(steal + decode::JMP_LEN)?;
    let code = decode::build_trampoline(&live, steal, at, tramp).map_err(|e| e.to_string())?;
    region.write(tramp, &code);
    Ok(tramp)
}

/// Replaces the start of the function at `at` with a jump to `target` (a Rust function with
/// the same signature). `ready` gets the trampoline (the original function) before the jump is
/// written, so a call arriving at once finds it.
pub fn detour(
    at: u32,
    target: u32,
    region: &mut Region,
    ready: impl FnOnce(u32),
) -> Result<(), String> {
    let tramp = trampoline(at, region)?;
    ready(tramp);
    write_jump(at, target)
}

/// Device methods waiting for the device: (hook id, vtable slot).
static PENDING_DEVICE: std::sync::Mutex<Vec<(u32, u32)>> = std::sync::Mutex::new(Vec::new());

/// Whether the list names device methods still to install.
pub fn device_hooks_pending() -> bool {
    !PENDING_DEVICE
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .is_empty()
}

/// Where a listed address is hooked.
enum Target {
    /// Exe code: its first instructions jump to the entry stub, which continues in a
    /// trampoline.
    Code,
    /// One of the exe's import slots: the slot points at the entry stub, which jumps to the
    /// imported function.
    ImportSlot,
    /// A Direct3D device method (the port's address for it, [`crate::d3d`]): its vtable slot
    /// is patched once the device exists.
    DeviceSlot(u32),
}

fn target_of(addr: u32) -> Target {
    if let Some(slot) = crate::d3d::device_slot(addr) {
        return Target::DeviceSlot(slot);
    }
    if crate::iat::SLOTS.get().is_some_and(|s| s.contains(&addr)) {
        return Target::ImportSlot;
    }
    Target::Code
}

/// An entry stub for hook `id` that continues at `next`.
fn entry_stub(region: &mut Region, id: u32, next: u32) -> Result<u32, String> {
    let entry = region.alloc(96)?;
    region.write(
        entry,
        &stubs::entry_stub(
            entry,
            id,
            enter as *const () as usize as u32,
            region.mxcsr_const(),
            next,
        ),
    );
    Ok(entry)
}

/// Builds and installs the hooks of `list`. Returns a status line per function. Device
/// methods are installed later, by [`install_device`].
pub fn install(list: &[FnSpec], region: &mut Region) -> Vec<String> {
    // SAFETY: plain TLS allocation.
    let idx = unsafe { win::TlsAlloc() };
    if idx == u32::MAX {
        return vec!["no TLS slot: no hooks installed".into()];
    }
    TLS.store(idx, Release);
    let leave_fn = leave as *const () as usize as u32;
    let mut hooks = Vec::new();
    // (address, entry stub) of code hooks to patch, and (slot address, entry stub) of imports.
    let mut code: Vec<(u32, u32)> = Vec::new();
    let mut slots: Vec<(u32, u32)> = Vec::new();
    let mut status = Vec::new();
    for spec in list {
        let id = hooks.len() as u32;
        let target = target_of(spec.addr);
        let built = (|| -> Result<u32, String> {
            match target {
                Target::Code => {
                    let tramp = trampoline(spec.addr, region)?;
                    code.push((spec.addr, entry_stub(region, id, tramp)?));
                }
                Target::ImportSlot => {
                    // SAFETY: an import slot of the exe, resolved by the loader.
                    let imported = unsafe { std::ptr::read(spec.addr as usize as *const u32) };
                    slots.push((spec.addr, entry_stub(region, id, imported)?));
                }
                Target::DeviceSlot(slot) => {
                    PENDING_DEVICE
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .push((id, slot));
                }
            }
            let ret = region.alloc(96)?;
            region.write(ret, &stubs::return_stub(id, leave_fn, region.mxcsr_const()));
            Ok(ret)
        })();
        match built {
            Ok(ret_stub) => {
                hooks.push(Hook {
                    spec: spec.clone(),
                    ret_stub,
                    calls: AtomicU64::new(0),
                    recorded: AtomicU64::new(0),
                    open_frame: AtomicU64::new(u64::MAX),
                });
                if let Target::DeviceSlot(slot) = target {
                    status.push(format!(
                        "hook={:08x} waits for the device (vtable slot {slot})",
                        spec.addr
                    ));
                }
            }
            Err(e) => status.push(format!("hook={:08x} refused: {e}", spec.addr)),
        }
    }
    TABLE.store(Box::into_raw(Box::new(Table { hooks })), Release);
    for (addr, entry) in code {
        match write_jump(addr, entry) {
            Ok(()) => status.push(format!("hook={addr:08x} installed")),
            Err(e) => status.push(format!("hook={addr:08x} refused: {e}")),
        }
    }
    for (slot, entry) in slots {
        // SAFETY: the exe's own import slot.
        let ok = unsafe {
            let mut old = 0u32;
            let p = slot as usize as *mut u32;
            let ok = win::VirtualProtect(p as *mut c_void, 4, win::PAGE_READWRITE, &mut old) != 0;
            if ok {
                *p = entry;
                let mut ignored = 0;
                win::VirtualProtect(p as *mut c_void, 4, old, &mut ignored);
            }
            ok
        };
        status.push(if ok {
            format!("hook={slot:08x} installed (import slot)")
        } else {
            format!("hook={slot:08x} refused: cannot unprotect the import slot")
        });
    }
    status
}

/// Installs the waiting device-method hooks in the device's vtable (once). Returns a status
/// line per method.
pub fn install_device(vtable: *mut usize) -> Vec<String> {
    let pending = std::mem::take(&mut *PENDING_DEVICE.lock().unwrap_or_else(|e| e.into_inner()));
    if pending.is_empty() {
        return Vec::new();
    }
    let mut region = match Region::new(256 * 1024) {
        Ok(r) => r,
        Err(e) => return vec![format!("device hooks refused: {e}")],
    };
    let mut status = Vec::new();
    for (id, slot) in pending {
        // SAFETY: the device's vtable has 119 slots; `slot` is below that.
        let original = unsafe { *vtable.add(slot as usize) } as u32;
        let line = match entry_stub(&mut region, id, original) {
            // SAFETY: as above.
            Ok(entry) => match unsafe { crate::d3d::swap_slot(vtable, slot, entry as usize) } {
                Some(_) => format!("hook=device slot {slot} installed"),
                None => format!("hook=device slot {slot} refused: cannot unprotect the vtable"),
            },
            Err(e) => format!("hook=device slot {slot} refused: {e}"),
        };
        status.push(line);
    }
    status
}

/// The calling thread's start address (`NtQueryInformationThread`,
/// `ThreadQuerySetWin32StartAddress`); 0 for the exe's entry point (the main thread, as the
/// port names it) or when it can't be read.
fn thread_start() -> u32 {
    type Query = unsafe extern "system" fn(isize, u32, *mut u32, u32, *mut u32) -> i32;
    let f = crate::sys::proc_address("ntdll.dll", "NtQueryInformationThread");
    if f == 0 {
        return 0;
    }
    let mut start = 0u32;
    // SAFETY: ntdll's NtQueryInformationThread on the current-thread pseudo handle, with a
    // 4-byte output (class 9 answers a pointer); the exe's own headers are read for its entry.
    unsafe {
        let query: Query = std::mem::transmute(f);
        if query(-2, 9, &mut start, 4, std::ptr::null_mut()) < 0 {
            return 0;
        }
        let base = win::GetModuleHandleW(std::ptr::null()) as usize;
        let pe = std::ptr::read_unaligned((base + 0x3c) as *const u32) as usize;
        let entry = std::ptr::read_unaligned((base + pe + 0x28) as *const u32) as usize;
        if start as usize == base + entry {
            return 0;
        }
    }
    start
}

/// The calling thread's stack: (limit, base) from its TEB (`NT_TIB` +8 and +4).
fn stack_bounds() -> (u32, u32) {
    let teb: u32;
    // SAFETY: FS:[0x18] is the TEB's own address on 32-bit Windows; reading the TIB's words.
    unsafe {
        std::arch::asm!("mov {}, fs:[0x18]", out(reg) teb, options(nostack, readonly, preserves_flags));
        let base = std::ptr::read((teb + 4) as usize as *const u32);
        let limit = std::ptr::read((teb + 8) as usize as *const u32);
        (limit, base)
    }
}
