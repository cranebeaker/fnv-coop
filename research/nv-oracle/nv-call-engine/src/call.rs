//! Calling a function on the real CPU and recovering from faults.
//!
//! `nv_call_raw` is a small assembly routine. Given a [`CallFrame`] it sets
//! the FPU control word and MXCSR, registers an exception handler record,
//! pushes the stack arguments, loads the registers, calls the target, and
//! copies back EAX, EDX, ST0 (when asked), XMM0, the FPU status word and the
//! stack pointer. The stack pointer is always restored from a saved copy
//! rather than computed, so it does not matter whether the callee or the
//! caller removes the arguments.
//!
//! Faults are caught by a frame-based handler: the harness's record is put on
//! the thread's handler chain (`fs:[0]`) before the call, so every record the
//! called code registers during the call is searched before it, and so are
//! the handlers inside system libraries it calls (`IsBadReadPtr` has one).
//! Those see an exception first, exactly as in the original program. Only an
//! exception that none of them handled reaches the harness's record, and every
//! such exception then becomes a recorded [`Fault`] (apart from the
//! informational ones a debugger swallows): the handler points the resumed
//! context at `nv_call_recover`, which puts the harness's own chain head back,
//! resets the FPU and returns to the Rust caller as if the call had ended.
//! Calls to unresolved imports go through a trap that does the same.
//!
//! The record is on the harness thread's chain only, so an exception on any
//! other thread that the called code creates never reaches it.

use nv_oracle_core::vectors::Fault;
use nv_win as win;
use std::arch::global_asm;
use std::ffi::c_void;
use std::mem::offset_of;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

/// Everything the assembly routine reads and writes. Field offsets are
/// passed to the assembly as constants, so the order here is free to change.
#[repr(C)]
pub struct CallFrame {
    // Inputs.
    pub target: u32,
    pub nstack: u32,
    pub stack: *const u32,
    /// EAX, ECX, EDX, EBX, ESI, EDI before the call.
    pub regs_in: [u32; 6],
    pub fpcw: u32,
    pub mxcsr: u32,
    pub xmm_in: [[u8; 16]; 4],
    pub want_st0: u32,
    // Outputs.
    pub eax: u32,
    pub edx: u32,
    pub fsw: u32,
    pub fcw: u32,
    pub mxcsr_out: u32,
    pub esp_before: u32,
    pub esp_after: u32,
    pub st0: [u8; 16],
    pub xmm0: [u8; 16],
}

impl CallFrame {
    pub fn new() -> CallFrame {
        CallFrame {
            target: 0,
            nstack: 0,
            stack: std::ptr::null(),
            regs_in: [0; 6],
            fpcw: 0x27f,
            mxcsr: 0x1f80,
            xmm_in: [[0; 16]; 4],
            want_st0: 0,
            eax: 0,
            edx: 0,
            fsw: 0,
            fcw: 0,
            mxcsr_out: 0,
            esp_before: 0,
            esp_after: 0,
            st0: [0; 16],
            xmm0: [0; 16],
        }
    }
}

// State shared between the assembly, the exception handler and Rust. The
// harness runs one call at a time on one thread, so plain atomics suffice.
static SAVED_ESP: AtomicU32 = AtomicU32::new(0);
static CALL_TARGET: AtomicU32 = AtomicU32::new(0);
static TRAP_HIT: AtomicU32 = AtomicU32::new(0);
static TRAP_INDEX: AtomicU32 = AtomicU32::new(0);
static TRAP_CALLER: AtomicU32 = AtomicU32::new(0);
static FAULT_HIT: AtomicU32 = AtomicU32::new(0);
static FAULT_CODE: AtomicU32 = AtomicU32::new(0);
static FAULT_EIP: AtomicU32 = AtomicU32::new(0);
static FAULT_ADDR: AtomicU32 = AtomicU32::new(0);
/// 0 none, 1 read, 2 write, 3 execute.
static FAULT_ACCESS: AtomicU32 = AtomicU32::new(0);

global_asm!(
    ".text",
    ".globl _nv_call_raw",
    "_nv_call_raw:",
    "    push ebp",
    "    mov ebp, esp",
    "    push ebx",
    "    push esi",
    "    push edi",
    // Locals: [esp] control word, [esp+4] MXCSR, [esp+8] the frame,
    // [esp+12] the thread's handler chain head to put back afterwards.
    "    sub esp, 16",
    "    mov ebx, [ebp + 8]",
    "    mov [esp + 8], ebx",
    "    fnstcw word ptr [esp]",
    "    stmxcsr dword ptr [esp + 4]",
    "    mov dword ptr [{saved_esp}], esp",
    "    mov eax, dword ptr fs:[0]",
    "    mov [esp + 12], eax",
    "    mov eax, [ebx + {off_target}]",
    "    mov dword ptr [{call_target}], eax",
    "    fninit",
    "    fldcw word ptr [ebx + {off_fpcw}]",
    "    ldmxcsr dword ptr [ebx + {off_mxcsr}]",
    "    movups xmm0, xmmword ptr [ebx + {off_xmm}]",
    "    movups xmm1, xmmword ptr [ebx + {off_xmm} + 16]",
    "    movups xmm2, xmmword ptr [ebx + {off_xmm} + 32]",
    "    movups xmm3, xmmword ptr [ebx + {off_xmm} + 48]",
    // Register the handler record above the arguments (a callee that
    // removes its own arguments must find nothing but them below its return
    // address): handler, then the previous head, then fs:[0] points here.
    "    mov eax, offset {seh_handler}",
    "    push eax",
    "    push dword ptr fs:[0]",
    "    mov dword ptr fs:[0], esp",
    "    mov ecx, [ebx + {off_nstack}]",
    "    mov esi, [ebx + {off_stack}]",
    "2:",
    "    test ecx, ecx",
    "    jz 3f",
    "    push dword ptr [esi + ecx*4 - 4]",
    "    dec ecx",
    "    jmp 2b",
    "3:",
    "    mov [ebx + {off_esp_before}], esp",
    "    mov eax, [ebx + {off_regs}]",
    "    mov ecx, [ebx + {off_regs} + 4]",
    "    mov edx, [ebx + {off_regs} + 8]",
    "    mov esi, [ebx + {off_regs} + 16]",
    "    mov edi, [ebx + {off_regs} + 20]",
    "    mov ebx, [ebx + {off_regs} + 12]",
    "    call dword ptr [{call_target}]",
    // The callee returned. Find the frame again through the saved stack
    // pointer, because the callee may have used every register.
    "    mov ecx, dword ptr [{saved_esp}]",
    "    mov ecx, [ecx + 8]",
    "    mov [ecx + {off_eax}], eax",
    "    mov [ecx + {off_edx}], edx",
    "    mov [ecx + {off_esp_after}], esp",
    "    fnstsw ax",
    "    movzx eax, ax",
    "    mov [ecx + {off_fsw}], eax",
    "    fnstcw word ptr [ecx + {off_fcw}]",
    "    stmxcsr dword ptr [ecx + {off_mxcsr_out}]",
    "    movups xmmword ptr [ecx + {off_xmm0}], xmm0",
    "    cmp dword ptr [ecx + {off_want_st0}], 0",
    "    je 4f",
    // Only read ST0 if the stack is not empty (TOP is not zero).
    "    test eax, 0x3800",
    "    jz 4f",
    "    fstp tbyte ptr [ecx + {off_st0}]",
    "4:",
    ".globl _nv_call_recover",
    "_nv_call_recover:",
    "    cld",
    "    mov esp, dword ptr [{saved_esp}]",
    // Take the record (and any the callee left behind) off the chain.
    "    mov eax, [esp + 12]",
    "    mov dword ptr fs:[0], eax",
    "    fninit",
    "    fldcw word ptr [esp]",
    "    ldmxcsr dword ptr [esp + 4]",
    "    add esp, 16",
    "    pop edi",
    "    pop esi",
    "    pop ebx",
    "    pop ebp",
    "    ret",
    // Every IAT slot points at a small stub that loads its import index into
    // EAX and jumps here.
    ".globl _nv_call_trap",
    "_nv_call_trap:",
    "    mov dword ptr [{trap_index}], eax",
    "    mov eax, [esp]",
    "    mov dword ptr [{trap_caller}], eax",
    "    mov dword ptr [{trap_hit}], 1",
    "    jmp _nv_call_recover",
    saved_esp = sym SAVED_ESP,
    call_target = sym CALL_TARGET,
    seh_handler = sym seh_handler,
    trap_index = sym TRAP_INDEX,
    trap_caller = sym TRAP_CALLER,
    trap_hit = sym TRAP_HIT,
    off_target = const offset_of!(CallFrame, target),
    off_nstack = const offset_of!(CallFrame, nstack),
    off_stack = const offset_of!(CallFrame, stack),
    off_regs = const offset_of!(CallFrame, regs_in),
    off_fpcw = const offset_of!(CallFrame, fpcw),
    off_mxcsr = const offset_of!(CallFrame, mxcsr),
    off_xmm = const offset_of!(CallFrame, xmm_in),
    off_want_st0 = const offset_of!(CallFrame, want_st0),
    off_eax = const offset_of!(CallFrame, eax),
    off_edx = const offset_of!(CallFrame, edx),
    off_fsw = const offset_of!(CallFrame, fsw),
    off_fcw = const offset_of!(CallFrame, fcw),
    off_mxcsr_out = const offset_of!(CallFrame, mxcsr_out),
    off_esp_before = const offset_of!(CallFrame, esp_before),
    off_esp_after = const offset_of!(CallFrame, esp_after),
    off_st0 = const offset_of!(CallFrame, st0),
    off_xmm0 = const offset_of!(CallFrame, xmm0),
);

extern "C" {
    fn nv_call_raw(frame: *mut CallFrame);
    fn nv_call_recover();
    fn nv_call_trap();
}

// On x86 with the MSVC linker, an exception handler must be listed in the
// image's safe handler table or Windows refuses to call it. (The MinGW
// linker does not build such a table, and does not need the entry.)
#[cfg(target_env = "msvc")]
global_asm!(".safeseh {seh_handler}", seh_handler = sym seh_handler);

/// Address of the trap routine, for building import stubs.
pub fn trap_address() -> u32 {
    nv_call_trap as *const () as usize as u32
}

const STATUS_STACK_OVERFLOW: u32 = 0xC000_00FD;

/// `ExceptionFlags` bits.
const EXCEPTION_NONCONTINUABLE: u32 = 0x1;
const EXCEPTION_UNWINDING: u32 = 0x2;
const EXCEPTION_EXIT_UNWIND: u32 = 0x4;

/// Exceptions a program raises to announce something and then carries on from
/// (text for `OutputDebugString`, a thread name for the debugger). They are
/// continuable and a debugger swallows them, so they are not faults.
fn is_informational(code: u32) -> bool {
    matches!(
        code,
        0x4001_0006 // DBG_PRINTEXCEPTION_C
            | 0x4001_000A // DBG_PRINTEXCEPTION_WIDE_C
            | 0x406D_1388 // MS_VC_EXCEPTION (thread name)
    )
}

/// `EFLAGS` bits that must not survive the jump to the recovery code: the
/// trap flag (a callee that set it would be single-stepped there for ever) and
/// the alignment check flag.
const EFLAGS_TF: u32 = 0x0100;
const EFLAGS_AC: u32 = 0x0004_0000;

/// The handler record's function, called by the system as
/// `handler(record, frame, context, dispatcher)` with the C convention. It
/// runs only for exceptions that no handler of the called code accepted, and
/// every one of those is a fault, whatever its code: a C++ `throw`, a code the
/// called program raises itself, a single step, an invalid disposition. Left
/// alone, such an exception goes on to the process's unhandled-exception
/// path, which on some systems carries on as if nothing happened (a silently
/// wrong result) and on others ends the harness and the rest of the batch.
/// The informational exceptions of [`is_informational`] are the only ones
/// that are continued instead.
unsafe extern "C" fn seh_handler(
    rec: *mut win::ExceptionRecord,
    _frame: *mut c_void,
    ctx: *mut win::Context,
    _dispatcher: *mut c_void,
) -> i32 {
    const CONTINUE_EXECUTION: i32 = 0;
    const CONTINUE_SEARCH: i32 = 1;
    let rec = &mut *rec;
    // Unwinding calls every record being removed; this one has nothing to
    // clean up.
    if rec.exception_flags & (EXCEPTION_UNWINDING | EXCEPTION_EXIT_UNWIND) != 0 {
        return CONTINUE_SEARCH;
    }
    if is_informational(rec.exception_code) {
        return CONTINUE_EXECUTION;
    }
    let ctx = &mut *ctx;
    FAULT_CODE.store(rec.exception_code, Relaxed);
    FAULT_EIP.store(ctx.eip, Relaxed);
    if rec.exception_code == 0xC000_0005 && rec.number_parameters >= 2 {
        FAULT_ACCESS.store(
            match rec.exception_information[0] {
                0 => 1,
                1 => 2,
                _ => 3,
            },
            Relaxed,
        );
        FAULT_ADDR.store(rec.exception_information[1] as u32, Relaxed);
    } else {
        FAULT_ACCESS.store(0, Relaxed);
        FAULT_ADDR.store(0, Relaxed);
    }
    FAULT_HIT.store(1, Relaxed);
    // The faulting code is abandoned, not continued, so a record that was
    // raised as non-continuable must not turn this into another exception.
    rec.exception_flags &= !EXCEPTION_NONCONTINUABLE;
    ctx.eflags &= !(EFLAGS_TF | EFLAGS_AC);
    ctx.eip = nv_call_recover as *const () as usize as u32;
    CONTINUE_EXECUTION
}

/// A stack overflow uses up the thread's guard page. Put it back on the
/// lowest committed page of the stack, so that the next overflow is reported
/// as one as well instead of ending the process. (`_resetstkoflw` does the
/// same.) Called after the stack has been unwound.
fn rearm_stack_guard() {
    let here = 0u8;
    let here_addr = std::ptr::addr_of!(here) as usize;
    let mut info = win::MemoryBasicInformation::zeroed();
    let query = |addr: usize, info: &mut win::MemoryBasicInformation| {
        // SAFETY: valid output buffer of the right size.
        unsafe {
            win::VirtualQuery(
                addr as *const c_void,
                info,
                std::mem::size_of::<win::MemoryBasicInformation>(),
            )
        }
    };
    if query(here_addr, &mut info) == 0 {
        return;
    }
    // Walk up from the base of the stack's reservation to its lowest usable
    // page: the first one that is committed and not marked no-access (some
    // systems keep a no-access page at the very bottom).
    let mut cur = info.allocation_base as usize;
    while cur < here_addr {
        if query(cur, &mut info) == 0 {
            return;
        }
        if info.state == win::MEM_COMMIT && info.protect != win::PAGE_NOACCESS {
            if info.protect & win::PAGE_GUARD == 0 {
                let mut old = 0u32;
                // SAFETY: protecting one page of this thread's own stack,
                // well below the part in use.
                unsafe {
                    win::VirtualProtect(
                        cur as *mut c_void,
                        0x1000,
                        win::PAGE_READWRITE | win::PAGE_GUARD,
                        &mut old,
                    )
                };
            }
            return;
        }
        let next = info.base_address as usize + info.region_size;
        if next <= cur {
            return;
        }
        cur = next;
    }
}

/// What stopped a call early.
pub enum Stopped {
    Exception(Fault),
    /// Index of the import that was called.
    Import {
        index: usize,
        caller: u32,
    },
}

/// Run one call. Returns `None` if the callee returned normally, in which
/// case the output fields of `frame` are valid.
///
/// # Safety
/// `frame.target` must be an address in this process, and `frame.stack`
/// must point at `frame.nstack` readable dwords. The called code can do
/// anything the original program could; faults are caught, other damage is
/// not.
pub unsafe fn run(frame: &mut CallFrame) -> Option<Stopped> {
    FAULT_HIT.store(0, Relaxed);
    TRAP_HIT.store(0, Relaxed);
    nv_call_raw(frame);
    if TRAP_HIT.load(Relaxed) != 0 {
        return Some(Stopped::Import {
            index: TRAP_INDEX.load(Relaxed) as usize,
            caller: TRAP_CALLER.load(Relaxed),
        });
    }
    if FAULT_HIT.load(Relaxed) != 0 {
        let code = FAULT_CODE.load(Relaxed);
        if code == STATUS_STACK_OVERFLOW {
            rearm_stack_guard();
        }
        let access = match FAULT_ACCESS.load(Relaxed) {
            1 => Some("read"),
            2 => Some("write"),
            3 => Some("execute"),
            _ => None,
        };
        return Some(Stopped::Exception(Fault::Exception {
            code,
            eip: FAULT_EIP.load(Relaxed),
            address: access.map(|_| FAULT_ADDR.load(Relaxed)),
            access,
        }));
    }
    None
}
