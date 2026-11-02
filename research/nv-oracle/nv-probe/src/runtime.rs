//! What runs when a hooked function is entered or returns: argument and
//! memory capture, the per-thread shadow stack and the log records.

use crate::log;
use crate::memory;
use crate::stubs::{fx, SavedRegs, FRAME_SIZE};
use nv_oracle_core::abi::{ArgType, Layout, Place, RetKind};
use nv_oracle_core::json::{self, ObjectWriter};
use nv_oracle_core::manifest::{EvalContext, HookSpec, Reg};
use nv_oracle_core::x87::Ext80;
use nv_win as win;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, AtomicU64, Ordering::*};

/// Everything the handlers need to know about one installed hook.
pub struct HookRt {
    pub spec: HookSpec,
    pub layout: Layout,
    pub trampoline: u32,
    pub entry_stub: u32,
    /// Zero when the return is not captured.
    pub ret_stub: u32,
    pub max: u64,
    /// Times the function was entered.
    pub calls: AtomicU64,
    pub patch: std::sync::Mutex<Option<memory::Patch>>,
    pub installed: AtomicBool,
}

pub struct Table {
    pub hooks: Vec<HookRt>,
}

static TABLE: AtomicPtr<Table> = AtomicPtr::new(std::ptr::null_mut());
static TLS_INDEX: AtomicU32 = AtomicU32::new(u32::MAX);

pub fn publish(table: Table) -> &'static Table {
    let p = Box::into_raw(Box::new(table));
    TABLE.store(p, Release);
    // SAFETY: leaked on purpose; the table lives until the process ends.
    unsafe { &*p }
}

pub fn table() -> Option<&'static Table> {
    let p = TABLE.load(Acquire);
    // SAFETY: either null or a leaked, never-freed table.
    unsafe { p.as_ref() }
}

pub fn init_tls() -> bool {
    // SAFETY: plain TLS allocation.
    let idx = unsafe { win::TlsAlloc() };
    if idx == u32::MAX {
        return false;
    }
    TLS_INDEX.store(idx, Release);
    true
}

struct Shadow {
    orig_ret: u32,
    hook: u32,
    entry_esp: u32,
    call_seq: u64,
    /// Dump addresses resolved at entry, re-read at return.
    dumps: Vec<Option<(u32, usize)>>,
}

struct ThreadState {
    shadow: Vec<Shadow>,
    busy: bool,
}

/// This thread's state, created on first use. Null if TLS is unavailable.
unsafe fn thread_state() -> *mut ThreadState {
    let idx = TLS_INDEX.load(Acquire);
    if idx == u32::MAX {
        return std::ptr::null_mut();
    }
    let p = win::TlsGetValue(idx) as *mut ThreadState;
    if !p.is_null() {
        return p;
    }
    let b = Box::into_raw(Box::new(ThreadState {
        shadow: Vec::with_capacity(64),
        busy: false,
    }));
    win::TlsSetValue(idx, b as *mut _);
    b
}

/// Free the calling thread's state (from DLL_THREAD_DETACH).
pub fn free_thread_state() {
    let idx = TLS_INDEX.load(Acquire);
    if idx == u32::MAX {
        return;
    }
    // SAFETY: only this thread's own slot is touched.
    unsafe {
        let p = win::TlsGetValue(idx) as *mut ThreadState;
        if !p.is_null() {
            win::TlsSetValue(idx, std::ptr::null_mut());
            drop(Box::from_raw(p));
        }
    }
}

fn float_json(o: &mut ObjectWriter, bits32: Option<u32>, bits64: Option<u64>) {
    if let Some(b) = bits32 {
        o.hex32("bits", b).f64("v", f64::from(f32::from_bits(b)));
    }
    if let Some(b) = bits64 {
        o.raw("bits", &format!("\"0x{b:016X}\""))
            .f64("v", f64::from_bits(b));
    }
}

fn arg_json(ty: ArgType, lo: Result<u32, ()>, hi: Result<u32, ()>) -> String {
    let mut o = ObjectWriter::new();
    o.str("t", ty.name());
    let lo = match lo {
        Ok(v) => v,
        Err(()) => {
            o.str("error", "argument slot unreadable");
            return o.finish();
        }
    };
    match ty {
        ArgType::I8 => {
            o.i64("v", i64::from(lo as u8 as i8));
        }
        ArgType::U8 => {
            o.u64("v", u64::from(lo as u8));
        }
        ArgType::I16 => {
            o.i64("v", i64::from(lo as u16 as i16));
        }
        ArgType::U16 => {
            o.u64("v", u64::from(lo as u16));
        }
        ArgType::I32 => {
            o.i64("v", i64::from(lo as i32));
        }
        ArgType::U32 => {
            o.u64("v", u64::from(lo));
        }
        ArgType::Bool => {
            o.bool("v", lo as u8 != 0);
        }
        ArgType::Ptr => {
            o.hex32("v", lo);
        }
        ArgType::F32 => float_json(&mut o, Some(lo), None),
        ArgType::F64 | ArgType::I64 | ArgType::U64 => match hi {
            Ok(hi) => {
                let bits = (u64::from(hi) << 32) | u64::from(lo);
                match ty {
                    ArgType::F64 => float_json(&mut o, None, Some(bits)),
                    ArgType::I64 => {
                        o.i64("v", bits as i64);
                    }
                    _ => {
                        o.raw("v", &format!("\"0x{bits:016X}\""));
                    }
                }
            }
            Err(()) => {
                o.str("error", "argument slot unreadable");
            }
        },
    }
    o.finish()
}

/// Evaluation context for dump expressions at function entry.
struct EntryCtx<'a> {
    regs: &'a SavedRegs,
    entry_esp: u32,
    /// First dword of each declared argument.
    args: &'a [Option<u32>],
}

impl EvalContext for EntryCtx<'_> {
    fn reg(&self, r: Reg) -> u32 {
        match r {
            Reg::Eax => self.regs.eax,
            Reg::Ecx => self.regs.ecx,
            Reg::Edx => self.regs.edx,
            Reg::Ebx => self.regs.ebx,
            Reg::Esp => self.entry_esp,
            Reg::Ebp => self.regs.ebp,
            Reg::Esi => self.regs.esi,
            Reg::Edi => self.regs.edi,
        }
    }

    fn arg(&self, index: usize) -> Result<u32, String> {
        match self.args.get(index) {
            Some(Some(v)) => Ok(*v),
            Some(None) => Err(format!("argument {index} is unreadable")),
            None => Err(format!("no argument {index} declared")),
        }
    }

    fn read_u32(&self, addr: u32) -> Result<u32, String> {
        memory::read_u32(addr)
    }
}

fn regs_json(r: &SavedRegs, esp: u32) -> String {
    let mut o = ObjectWriter::new();
    o.hex32("eax", r.eax)
        .hex32("ecx", r.ecx)
        .hex32("edx", r.edx)
        .hex32("ebx", r.ebx);
    o.hex32("esp", esp)
        .hex32("ebp", r.ebp)
        .hex32("esi", r.esi)
        .hex32("edi", r.edi);
    o.finish()
}

fn dump_json(text: &str, addr: Result<(u32, usize), String>) -> String {
    let mut o = ObjectWriter::new();
    o.str("expr", text);
    match addr {
        Err(e) => {
            o.str("error", &e);
        }
        Ok((a, len)) => {
            o.hex32("addr", a);
            match memory::read_bytes(a, len) {
                Ok(bytes) => {
                    o.hex_bytes("hex", &bytes);
                }
                Err(e) => {
                    o.str("error", &e);
                }
            }
        }
    }
    o.finish()
}

fn tick() -> u32 {
    // SAFETY: plain query.
    unsafe { win::GetTickCount() }
}

fn record_header(o: &mut ObjectWriter, kind: &str, seq: u64, hook: &HookRt) {
    // SAFETY: plain query.
    let tid = unsafe { win::GetCurrentThreadId() };
    o.str("type", kind)
        .u64("seq", seq)
        .u64("tid", u64::from(tid))
        .str("hook", &hook.spec.name)
        .u64("tick", u64::from(tick()));
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
    let Some(table) = table() else { return };
    let Some(hook) = table.hooks.get(id as usize) else {
        return;
    };
    let n = hook.calls.fetch_add(1, Relaxed);
    if n >= hook.max {
        return;
    }
    let entry_esp = (regs as *mut SavedRegs as u32).wrapping_add(FRAME_SIZE);

    // Declared arguments: the first dword of each, for dump expressions,
    // and the JSON for each.
    let mut firsts: Vec<Option<u32>> = Vec::with_capacity(hook.spec.args.len());
    let mut args_json: Vec<String> = Vec::with_capacity(hook.spec.args.len());
    for (ty, place) in hook.spec.args.iter().zip(&hook.layout.places) {
        let read = |dword: usize| -> Result<u32, ()> {
            match place {
                Place::Ecx if dword == 0 => Ok(regs.ecx),
                Place::Edx if dword == 0 => Ok(regs.edx),
                Place::Stack(off) => {
                    memory::read_u32(entry_esp.wrapping_add(4 + 4 * (*off + dword) as u32))
                        .map_err(|_| ())
                }
                _ => Err(()),
            }
        };
        let lo = read(0);
        let hi = if ty.dwords() == 2 { read(1) } else { Ok(0) };
        firsts.push(lo.ok());
        args_json.push(arg_json(*ty, lo, hi));
    }

    let ctx = EntryCtx {
        regs,
        entry_esp,
        args: &firsts,
    };
    let mut dump_addrs: Vec<Option<(u32, usize)>> = Vec::with_capacity(hook.spec.dumps.len());
    let mut dumps_json: Vec<String> = Vec::with_capacity(hook.spec.dumps.len());
    for d in &hook.spec.dumps {
        let addr = d.expr.eval(&ctx).map(|a| (a, d.len));
        dump_addrs.push(addr.clone().ok());
        dumps_json.push(dump_json(&d.text, addr));
    }

    let regs_text = hook.spec.regs.then(|| regs_json(regs, entry_esp));
    let seq = log::append_with(|seq| {
        let mut o = ObjectWriter::new();
        record_header(&mut o, "call", seq, hook);
        o.raw("args", &json::array_of(&args_json));
        if let Some(r) = &regs_text {
            o.raw("regs", r);
        }
        o.raw("dumps", &json::array_of(&dumps_json));
        o.finish()
    });
    if hook.ret_stub != 0 {
        // Replace the return address so the function comes back through
        // the return stub. First drop shadow entries from calls that never
        // returned: their frames are below this one, which is only possible
        // once the stack has unwound past them. An entry at the very same
        // position is not stale. It is the hooked function that just jumped
        // here (a jump thunk, or a tail call), whose return stub is the
        // return address now in the slot, and it must stay for that stub.
        while ts.shadow.last().is_some_and(|s| s.entry_esp < entry_esp) {
            ts.shadow.pop();
        }
        // SAFETY: the slot is the function's own return address on its stack.
        let orig = unsafe { std::ptr::read(entry_esp as usize as *const u32) };
        ts.shadow.push(Shadow {
            orig_ret: orig,
            hook: id,
            entry_esp,
            call_seq: seq,
            dumps: dump_addrs,
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

    // The return stub was entered with the stack pointer just above the
    // slot it pushed: that is where the callee's `ret` left it.
    let stub_esp = (regs as *mut SavedRegs as u32).wrapping_add(FRAME_SIZE + 4);
    let pop = if hook.spec.cc.callee_cleans() {
        hook.layout.stack_bytes() as u32
    } else {
        0
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
    // Anything above it is a call that never returned (unwound past).
    ts.shadow.truncate(idx + 1);
    let entry = ts.shadow.pop().expect("entry found above");
    regs.ret_addr = entry.orig_ret;

    if !ts.busy {
        ts.busy = true;
        leave_impl(hook, regs, fxsave, &entry);
        ts.busy = false;
    }
    win::SetLastError(last_error);
}

fn leave_impl(hook: &HookRt, regs: &SavedRegs, fxsave: *const u8, entry: &Shadow) {
    log::append_with(|seq| leave_line(hook, regs, fxsave, entry, seq));
}

fn leave_line(
    hook: &HookRt,
    regs: &SavedRegs,
    fxsave: *const u8,
    entry: &Shadow,
    seq: u64,
) -> String {
    let mut o = ObjectWriter::new();
    record_header(&mut o, "ret", seq, hook);
    o.u64("call", entry.call_seq);
    o.hex32("eax", regs.eax).hex32("edx", regs.edx);
    if hook.spec.regs {
        // The stack pointer the caller sees after the return.
        o.raw(
            "regs",
            &regs_json(
                regs,
                (regs as *const SavedRegs as u32).wrapping_add(FRAME_SIZE + 4),
            ),
        );
    }
    // SAFETY: the stub passes a pointer to a 512-byte FXSAVE image.
    let image = unsafe { std::slice::from_raw_parts(fxsave, fx::SIZE) };
    match hook.spec.ret {
        Some(RetKind::F32X87 | RetKind::F64X87) => {
            let fsw = u16::from_le_bytes([image[fx::FSW], image[fx::FSW + 1]]);
            let top = (fsw >> 11) & 7;
            // The abridged tag byte has one bit per physical register; ST0
            // is the register TOP points at, and FXSAVE stores ST0 first.
            if image[fx::FTW] & (1 << top) != 0 {
                let mut raw = [0u8; 10];
                raw.copy_from_slice(&image[fx::ST0..fx::ST0 + 10]);
                let e = Ext80::from_bytes(raw);
                let mut s = ObjectWriter::new();
                s.str("raw", &e.to_hex());
                s.hex32("f32_bits", e.to_f32_bits())
                    .f64("f32", f64::from(e.to_f32()));
                s.raw("f64_bits", &format!("\"0x{:016X}\"", e.to_f64_bits()))
                    .f64("f64", e.to_f64());
                o.raw("st0", &s.finish());
            } else {
                o.null("st0");
            }
        }
        Some(RetKind::Xmm0) => {
            o.hex_bytes("xmm0", &image[fx::XMM0..fx::XMM0 + 16]);
        }
        Some(RetKind::I32) => {
            o.i64("ret", i64::from(regs.eax as i32));
        }
        Some(RetKind::U32) => {
            o.u64("ret", u64::from(regs.eax));
        }
        Some(RetKind::Ptr) => {
            o.hex32("ret", regs.eax);
        }
        Some(RetKind::I64) => {
            let v = (u64::from(regs.edx) << 32) | u64::from(regs.eax);
            o.i64("ret", v as i64);
        }
        Some(RetKind::Void) | None => {}
    }
    if !entry.dumps.is_empty() {
        let items: Vec<String> = entry
            .dumps
            .iter()
            .zip(&hook.spec.dumps)
            .map(|(a, d)| {
                dump_json(
                    &d.text,
                    a.ok_or_else(|| "address could not be resolved at entry".to_string()),
                )
            })
            .collect();
        o.raw("dumps_after", &json::array_of(&items));
    }
    o.finish()
}

/// A state the probe cannot continue from (the original return address is
/// lost): record why, then end the process rather than jump somewhere wrong.
fn fatal(msg: &str) -> ! {
    let mut o = ObjectWriter::new();
    o.str("type", "fatal").str("error", msg);
    log::append(&o.finish());
    log::flush_now(false);
    // SAFETY: ending this process.
    unsafe { win::TerminateProcess(win::GetCurrentProcess(), 0x4e56) };
    std::process::abort();
}

/// Resolve and describe a hook for the header line.
pub fn hook_summary(h: &HookRt) -> String {
    let mut o = ObjectWriter::new();
    o.str("name", &h.spec.name)
        .hex32("addr", h.spec.addr)
        .str("status", "installed");
    o.u64("steal", h.spec.steal as u64)
        .str("cc", h.spec.cc.name());
    o.bool("capture_return", h.ret_stub != 0);
    o.finish()
}
