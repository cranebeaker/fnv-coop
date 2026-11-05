//! Lines in `rec.log` for C++ exceptions in the real game (ADR-0013): each `_CxxThrowException`
//! with its type, message and caller, and each catch block the search finds. The hooks are in
//! every recording (`nv_trace_core::exceptions::LIST`); a run where nothing is thrown writes
//! nothing, and costs nothing outside a throw.

use crate::stubs::{SavedRegs, FRAME_SIZE};
use nv_trace_core::exceptions as ex;
use nv_win as win;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

/// `0xE06D7363`, the code `_CxxThrowException` raises.
const CXX_CODE: u32 = 0xe06d_7363;
/// The flags of an exception record while unwinding: the handlers' second pass.
const UNWINDING: u32 = 0x66;
/// A run keeps at most this many lines of it.
const MAX_LINES: u32 = 20_000;

static LINES: AtomicU32 = AtomicU32::new(0);

struct Process;

impl ex::Reader for Process {
    fn word(&mut self, addr: u32) -> Option<u32> {
        // SAFETY: checked readable just now.
        win::is_readable(addr, 4)
            .then(|| unsafe { std::ptr::read_unaligned(addr as usize as *const u32) })
    }

    fn c_string(&mut self, addr: u32) -> Option<String> {
        let mut bytes = Vec::new();
        for offset in 0..512u32 {
            let at = addr.wrapping_add(offset);
            if !win::is_readable(at, 1) {
                return None;
            }
            // SAFETY: checked readable just now.
            let byte = unsafe { std::ptr::read(at as usize as *const u8) };
            if byte == 0 {
                break;
            }
            bytes.push(byte);
        }
        Some(String::from_utf8_lossy(&bytes).into_owned())
    }
}

fn line(text: &str) {
    if LINES.fetch_add(1, Relaxed) < MAX_LINES {
        crate::log(text);
    }
}

/// The running OS thread's id.
fn thread() -> u32 {
    // SAFETY: plain query.
    unsafe { win::GetCurrentThreadId() }
}

fn stack_word(entry_esp: u32, index: u32) -> u32 {
    use ex::Reader;
    Process
        .word(entry_esp.wrapping_add(4 + 4 * index))
        .unwrap_or(0)
}

/// The exception a catch is being looked for, on this thread: `throw;` carries no type of its own
/// and is the last one thrown here.
pub struct Current {
    chain: Vec<String>,
}

impl Current {
    pub const fn new() -> Current {
        Current { chain: Vec::new() }
    }
}

/// An exception hook was entered (from [`crate::hooks`]), before the function runs.
pub fn on_enter(addr: u32, regs: &SavedRegs, current: &mut Current) {
    let entry_esp = (regs as *const SavedRegs as u32).wrapping_add(FRAME_SIZE);
    match addr {
        ex::THROW => on_throw(entry_esp, regs.ret_addr, current),
        ex::INTERNAL_HANDLER => on_handler(entry_esp, current),
        _ => {}
    }
}

fn on_throw(entry_esp: u32, caller: u32, current: &mut Current) {
    let (object, throw_info) = (stack_word(entry_esp, 0), stack_word(entry_esp, 1));
    let thread = thread();
    if throw_info == 0 {
        line(&format!(
            "exception: throw; (rethrow of {}) from {caller:08x} on thread {thread}",
            current.chain.first().map_or("?".to_string(), |n| ex::readable_type(n))
        ));
        return;
    }
    current.chain = ex::type_chain(throw_info, &mut Process);
    let most_derived = current
        .chain
        .first()
        .map_or("?".to_string(), |n| ex::readable_type(n));
    let what = ex::message(object, &current.chain, &mut Process)
        .map(|text| format!(" \"{text}\""))
        .unwrap_or_default();
    line(&format!(
        "exception: throw {most_derived}{what} from {caller:08x}, object {object:08x}, \
         ThrowInfo {throw_info:08x}, on thread {thread}"
    ));
}

/// `___InternalCxxFrameHandler(record, node, context, dispatcher context, FuncInfo, ...)`: on the
/// first pass over a frame, the catch block it has for the exception, if any. The state word is
/// at `+8` of the registration node.
fn on_handler(entry_esp: u32, current: &Current) {
    use ex::Reader;
    let record = stack_word(entry_esp, 0);
    let node = stack_word(entry_esp, 1);
    let func_info = stack_word(entry_esp, 4);
    let mut process = Process;
    let code = process.word(record).unwrap_or(0);
    let flags = process.word(record.wrapping_add(4)).unwrap_or(0);
    if flags & UNWINDING != 0 {
        return;
    }
    if code != CXX_CODE {
        line(&format!(
            "exception: structured exception {code:08x} reached the C++ frame with FuncInfo \
             {func_info:08x} on thread {}",
            thread()
        ));
        return;
    }
    let state = process.word(node.wrapping_add(8)).unwrap_or(u32::MAX) as i32;
    if let Some(catch) = ex::find_catch(func_info, state, &current.chain, &mut process) {
        let function = catch
            .function
            .map_or("a function not in the table".to_string(), |f| format!("{f:08x}"));
        line(&format!(
            "exception: catch by {function}: block {:08x}, {}, FuncInfo {func_info:08x}, state \
             {state}, on thread {}",
            catch.block,
            catch
                .type_name
                .as_deref()
                .map_or("catch(...)".to_string(), ex::readable_type),
            thread()
        ));
    }
}
