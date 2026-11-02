//! More C runtime pieces (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe 1.4.0.525),
//! translated from the exe: the structured-exception (SEH) machinery the compiler's frames use.
//!
//! - The frame helpers `__SEH_prolog4` (`00ed1714`), `__SEH_epilog4` (`00ed1759`),
//!   `__EH_prolog3` (`00ece255`), `__EH_prolog3_catch` (`00ece288`), `__EH_epilog3`
//!   (`00ece2be`) and the 8-byte aligned `alloca` probe (`00ec5ed6`).
//! - The handlers `__except_handler4` (`00ecc8e0`) with `_EH4_CallFilterFunc` (`00ed872e`),
//!   `_EH4_TransferToHandler` (`00ed8745`), `_EH4_GlobalUnwind` (`00ed875e`),
//!   `_EH4_LocalUnwind` (`00ed8778`), `__local_unwind4` (`00ed863c`) and its handler
//!   (`00ed86cc`), the `longjmp` unwind `00ed8712`; `__except_handler3` (`00ece104`) with
//!   `__ValidateEH3RN` (`00edf6d0`) and `_ValidateScopeTableHandlers` (`00edf610`),
//!   `__global_unwind2` (`00edf4d8`), `__local_unwind2` (`00edf53d`) and its handler
//!   (`00edf4f8`), the `longjmp` unwind `00ece202`; the non-local-goto hooks `__NLG_Notify`
//!   (`00edf5ed`), `__NLG_Notify1` (`00edf5e4`) and the call `00edf60c`;
//!   `_CallSettingFrame@12` (`00ed1550`).
//! - The `__except` handler blocks of `__RTDynamicCast` (`00ec44b5`), `__ArrayUnwind`
//!   (`00ec5fbc`), `__tmainCRTStartup` (`00ecc49c`, `00ecc4c6`), `__DestructExceptionObject`
//!   (`00ece53c`), `___BuildCatchObject` (`00ece9e9`), the SSE probe `00ed18d7` (`00ed1910`),
//!   `___set_fpsr_sse2` (`00ed5b85`) and `__crtInitCritSecAndSpinCount` (`00ede1d4`).
//!
//! The engine runs translations without a machine stack, segment registers or a thread
//! environment block, and it dispatches no exceptions (`RaiseException` stops it,
//! `units/platform.rs`). So:
//!
//! - The frame helpers work on [`Regs`]: the eight integer registers and the head of the
//!   thread's exception chain (`FS:[0]`), as a value, with the stack they push to and pop from in
//!   game memory at `ESP`, instruction by instruction. Their uniform form takes the nine words of
//!   [`Regs::words`] and answers `EAX` and `EDX` after; the rest of the state comes back only
//!   through the Rust function (as `__aulldvrm`'s remainder in `crt.rs`).
//! - A funclet (a filter, a `__finally` or `__except` block, a catch block, a continuation) is
//!   entered with `EBP` set to its function's frame; its uniform form here is that one word. The
//!   other registers are zeroed or meaningless at the call. The older funclet translations
//!   (`glue.rs`, `misc5.rs`, ...) take the frame words they read instead; a call into one of them
//!   through these dispatchers would not fit, which each dispatcher marks as incomplete.
//! - Where a function links a registration node into `FS:[0]` for its own duration (the unwinds,
//!   `__ValidateEH3RN`), the node is not built: it only matters to an exception raised inside,
//!   which the engine does not have.
//!
//! Every call is made by address, as the exe makes it, so a test can double each one.

#[allow(unused_imports)]
use crate::prelude::*;

/// The security cookie (`011c16bc`).
const SECURITY_COOKIE: u32 = 0x011c_16bc;
/// `__security_check_cookie` (`00ec408c`, fastcall: the value in `ECX`, one word here).
const SECURITY_CHECK_COOKIE: u32 = 0x00ec_408c;
/// `__except_handler4` (`00ecc8e0`), the handler `__SEH_prolog4` registers.
const EXCEPT_HANDLER4: u32 = 0x00ec_c8e0;
/// The `RtlUnwind` thunk (`00ee8210`, stdcall: frame, target address, record, value).
const RTL_UNWIND: u32 = 0x00ee_8210;
/// The addresses `__global_unwind2` (`00edf4f0`) and `_EH4_GlobalUnwind` (`00ed8773`) give
/// `RtlUnwind` to come back to (the instruction after each call).
const GLOBAL_UNWIND2_TARGET: u32 = 0x00ed_f4f0;
const EH4_GLOBAL_UNWIND_TARGET: u32 = 0x00ed_8773;
/// The non-local-goto hooks: `__NLG_Notify` (`00edf5ed`: `EAX`, `EBP`, then the code),
/// `__NLG_Notify1` (`00edf5e4`: `EAX`, `ECX` the code, `EBP`, then a word it drops) and the
/// call `00edf60c` (`EAX` the funclet, `EBP`), and the block they fill (`011c2a88`: `+4` the
/// address, `+8` the code, `+0xc` the frame).
const NLG_NOTIFY: u32 = 0x00ed_f5ed;
const NLG_NOTIFY1: u32 = 0x00ed_f5e4;
const NLG_CALL: u32 = 0x00ed_f60c;
const NLG_DATA: u32 = 0x011c_2a88;
/// The unwinds: `__local_unwind2` (`00edf53d`: `EBP`, node, stop level), `__global_unwind2`
/// (`00edf4d8`: frame) and `__local_unwind4` (`00ed863c`: `EBP`, the cookie's address, node,
/// stop level).
const LOCAL_UNWIND2: u32 = 0x00ed_f53d;
const GLOBAL_UNWIND2: u32 = 0x00ed_f4d8;
const LOCAL_UNWIND4: u32 = 0x00ed_863c;
/// The `__except_handler4` helpers: `_EH4_CallFilterFunc` (`00ed872e`: `ECX` the filter, `EDX`
/// the frame), `_EH4_TransferToHandler` (`00ed8745`: `ECX` the handler, `EDX` the frame),
/// `_EH4_GlobalUnwind` (`00ed875e`: `ECX` the node) and `_EH4_LocalUnwind` (`00ed8778`: `ECX`
/// the node, `EDX` the stop level, then the frame and the cookie's address).
const EH4_CALL_FILTER_FUNC: u32 = 0x00ed_872e;
const EH4_TRANSFER_TO_HANDLER: u32 = 0x00ed_8745;
const EH4_GLOBAL_UNWIND: u32 = 0x00ed_875e;
const EH4_LOCAL_UNWIND: u32 = 0x00ed_8778;
/// `__ValidateEH3RN` (`00edf6d0`, cdecl: the node) and `_ValidateScopeTableHandlers`
/// (`00edf610`: `ECX` the highest level, `EDI` the image base, then the scope table).
const VALIDATE_EH3RN: u32 = 0x00ed_f6d0;
const VALIDATE_SCOPE_TABLE_HANDLERS: u32 = 0x00ed_f610;
/// `_CallDestructExceptionObject` (`00edf49f`, cdecl: record, throw flag), the pointer to
/// `__DestructExceptionObject` `__except_handler4` calls through (`010f20a0`) and
/// `_IsNonwritableInCurrentImage` (`00eddcc0`, cdecl: an address).
const CALL_DESTRUCT_EXCEPTION_OBJECT: u32 = 0x00ed_f49f;
const DESTRUCT_POINTER: u32 = 0x010f_20a0;
const IS_NONWRITABLE_IN_CURRENT_IMAGE: u32 = 0x00ed_dcc0;
/// `__ValidateImageBase` (`00eddc30`, cdecl: base) and `__FindPESection` (`00eddc70`, cdecl:
/// base, RVA).
const VALIDATE_IMAGE_BASE: u32 = 0x00ed_dc30;
const FIND_PE_SECTION: u32 = 0x00ed_dc70;
/// The import slots of `VirtualQuery` (`00fdf260`, stdcall: address, buffer, size) and
/// `SetLastError` (`00fdf1dc`, stdcall, one word).
const IMPORT_VIRTUAL_QUERY: u32 = 0x00fd_f260;
const IMPORT_SET_LAST_ERROR: u32 = 0x00fd_f1dc;
/// `__ValidateEH3RN`'s cache of checked scope-table pages: the count (`01270738`, at most 16),
/// the lock word (`0127073c`) and the entries (`01270740`: page, image base).
const EH3_CACHE_COUNT: u32 = 0x0127_0738;
const EH3_CACHE_LOCK: u32 = 0x0127_073c;
const EH3_CACHE: u32 = 0x0127_0740;
/// `__alloca_probe` (`00ec8660`; `EAX` the size, then `ESP` at entry).
const ALLOCA_PROBE: u32 = 0x00ec_8660;
/// `terminate` (`00ed14b4`), `__exit` (`00eccedb`, cdecl: the code) and `_c_exit` (`00eccf00`),
/// no arguments unless said.
const TERMINATE: u32 = 0x00ed_14b4;
const EXIT: u32 = 0x00ec_cedb;
const C_EXIT: u32 = 0x00ec_cf00;
/// `__non_rtti_object::__non_rtti_object(const char *)` (`00ec7a45`, thiscall, one word), the
/// text `__RTDynamicCast`'s block gives it (`010f1ee4`), `_CxxThrowException` (`00ec7894`, stdcall: object, throw
/// information) and `__non_rtti_object`'s throw information (`0117fc10`).
const NON_RTTI_OBJECT: u32 = 0x00ec_7a45;
const NON_RTTI_TEXT: u32 = 0x010f_1ee4;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
const NON_RTTI_THROW_INFO: u32 = 0x0117_fc10;
/// Whether the processor takes `MXCSR`'s denormals-are-zero bit (`011c2538`) and the one
/// `MXCSR` value the engine models (`crt.rs`).
const DAZ_SUPPORTED: u32 = 0x011c_2538;
const MXCSR: u32 = 0x1f80;
/// `STATUS_ACCESS_VIOLATION`, `STATUS_NO_MEMORY` and `ERROR_NOT_ENOUGH_MEMORY`.
const ACCESS_VIOLATION: u32 = 0xc000_0005;
const NO_MEMORY: u32 = 0xc000_0017;
const NOT_ENOUGH_MEMORY: u32 = 8;
/// The MSVC C++ exception code (`'msc' | 0xE0000000`).
const CXX_EXCEPTION: u32 = 0xe06d_7363;
/// `MEM_IMAGE`, a `VirtualQuery` region's type for a mapped image.
const MEM_IMAGE: u32 = 0x0100_0000;

/// The machine state the frame helpers work on: the integer registers and the head of the
/// running thread's exception chain (`FS:[0]`), which the engine keeps nowhere else. The stack
/// is game memory at `esp`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Regs {
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,
    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,
    pub fs0: u32,
}

impl Regs {
    /// The state from nine words: the registers in their encoding order (`EAX`, `ECX`, `EDX`,
    /// `EBX`, `ESP`, `EBP`, `ESI`, `EDI`), then `FS:[0]`.
    pub fn from_words(words: &[u32]) -> Self {
        let word = |i: usize| {
            words
                .get(i)
                .copied()
                .unwrap_or_else(|| panic!("call has {} argument words, needs 9", words.len()))
        };
        Regs {
            eax: word(0),
            ecx: word(1),
            edx: word(2),
            ebx: word(3),
            esp: word(4),
            ebp: word(5),
            esi: word(6),
            edi: word(7),
            fs0: word(8),
        }
    }

    /// The nine words of [`Regs::from_words`].
    pub fn words(&self) -> [u32; 9] {
        [
            self.eax, self.ecx, self.edx, self.ebx, self.esp, self.ebp, self.esi, self.edi,
            self.fs0,
        ]
    }

    /// `PUSH value`.
    fn push(&mut self, e: &mut Engine, value: u32) {
        self.esp = self.esp.wrapping_sub(4);
        e.mem.set_u32(self.esp, value);
    }

    /// `POP`: the word at `ESP`.
    fn pop(&mut self, e: &mut Engine) -> u32 {
        let value = e.mem.u32(self.esp);
        self.esp = self.esp.wrapping_add(4);
        value
    }

    /// `EAX` and `EDX`: what the uniform form answers.
    fn answer(&self) -> Ret {
        Ret {
            eax: self.eax,
            edx: self.edx,
            st0: 0.0,
        }
    }
}

// ---- The frame helpers ------------------------------------------------------------------------

// Translated from 00ed1714 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__SEH_prolog4` (LIBCMT/sehprolg4.asm), called with the scope table and the size of the
/// locals pushed (`[ESP+4]`, `[ESP+8]`). Pushes `__except_handler4` and the chain head; the
/// size's slot takes the caller's `EBP` and becomes the new `EBP`; the locals are made
/// (`ESP -= size`) and `EBX`, `ESI`, `EDI` pushed; the scope table's slot is XORed with the
/// cookie, the cookie XOR `EBP` pushed (the `/GS` word), `[EBP-0x18]` = `ESP`; the return
/// address is pushed again, its old slot (`EBP-8`) takes the encoded scope table and `EBP-4`
/// the try level -2; `FS:[0]` = `EBP-0x10` (also left in `EAX`); `RET` pops the return address.
pub fn seh_prolog4(e: &mut Engine, r: &mut Regs) {
    r.push(e, EXCEPT_HANDLER4);
    r.push(e, r.fs0);
    r.eax = e.mem.u32(r.esp.wrapping_add(0x10));
    e.mem.set_u32(r.esp.wrapping_add(0x10), r.ebp);
    r.ebp = r.esp.wrapping_add(0x10);
    r.esp = r.esp.wrapping_sub(r.eax);
    r.push(e, r.ebx);
    r.push(e, r.esi);
    r.push(e, r.edi);
    r.eax = e.mem.u32(SECURITY_COOKIE);
    let slot = r.ebp.wrapping_sub(4);
    let encoded = e.mem.u32(slot) ^ r.eax;
    e.mem.set_u32(slot, encoded);
    r.eax ^= r.ebp;
    r.push(e, r.eax);
    e.mem.set_u32(r.ebp.wrapping_sub(0x18), r.esp);
    let return_address = e.mem.u32(r.ebp.wrapping_sub(8));
    r.push(e, return_address);
    r.eax = e.mem.u32(slot);
    e.mem.set_u32(slot, 0xffff_fffe);
    e.mem.set_u32(r.ebp.wrapping_sub(8), r.eax);
    r.eax = r.ebp.wrapping_sub(0x10);
    r.fs0 = r.eax;
    r.pop(e);
}

// Translated from 00ed1759 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__SEH_epilog4` (LIBCMT/sehprolg4.asm): `FS:[0]` = `[EBP-0x10]`; pops the return address
/// into `ECX`, the `/GS` word and `EDI`, `ESI`, `EBX`; `ESP` = `EBP`, `EBP` popped; the return
/// address pushed and `RET` to it (the guarded function's own `RET` follows there). `EAX` is
/// left as it is: the guarded function's answer.
pub fn seh_epilog4(e: &mut Engine, r: &mut Regs) {
    r.ecx = e.mem.u32(r.ebp.wrapping_sub(0x10));
    r.fs0 = r.ecx;
    epilog_pops(e, r);
}

/// The pops `__SEH_epilog4` and `__EH_epilog3` share (`POP ECX; POP EDI; POP EDI; POP ESI;
/// POP EBX; MOV ESP, EBP; POP EBP; PUSH ECX; RET`).
fn epilog_pops(e: &mut Engine, r: &mut Regs) {
    r.ecx = r.pop(e);
    r.edi = r.pop(e);
    r.edi = r.pop(e);
    r.esi = r.pop(e);
    r.ebx = r.pop(e);
    r.esp = r.ebp;
    r.ebp = r.pop(e);
    r.push(e, r.ecx);
    r.pop(e);
}

// Translated from 00ece255 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__EH_prolog3` (LIBCMT/ehprolg3.c), `EAX` the C++ unwind handler, the size of the locals
/// pushed (`[ESP+4]`). Pushes the handler and the chain head; the size's slot takes the
/// caller's `EBP` and becomes the new `EBP` (`EAX` = `ESP+0xc` before the locals); the locals
/// are made and `EBX`, `ESI`, `EDI` pushed; the cookie XOR `EBP` pushed; the return address
/// pushed again and its old slot (`EBP-4`) takes the state -1; `FS:[0]` = `EBP-0xc` (also left
/// in `EAX`); `RET`.
pub fn eh_prolog3(e: &mut Engine, r: &mut Regs) {
    eh_prolog3_body(e, r, false);
}

// Translated from 00ece288 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__EH_prolog3_catch` (LIBCMT/ehprolg3.c): [`eh_prolog3`] that also keeps the stack pointer
/// after the `/GS` word in `[EBP-0x10]`, for the catch blocks.
pub fn eh_prolog3_catch(e: &mut Engine, r: &mut Regs) {
    eh_prolog3_body(e, r, true);
}

/// The body of `__EH_prolog3` (`00ece255`) and `__EH_prolog3_catch` (`00ece288`).
fn eh_prolog3_body(e: &mut Engine, r: &mut Regs, catch: bool) {
    r.push(e, r.eax);
    r.push(e, r.fs0);
    r.eax = r.esp.wrapping_add(0xc);
    let size = e.mem.u32(r.esp.wrapping_add(0xc));
    r.esp = r.esp.wrapping_sub(size);
    r.push(e, r.ebx);
    r.push(e, r.esi);
    r.push(e, r.edi);
    e.mem.set_u32(r.eax, r.ebp);
    r.ebp = r.eax;
    r.eax = e.mem.u32(SECURITY_COOKIE) ^ r.ebp;
    r.push(e, r.eax);
    if catch {
        e.mem.set_u32(r.ebp.wrapping_sub(0x10), r.esp);
    }
    let return_address = e.mem.u32(r.ebp.wrapping_sub(4));
    r.push(e, return_address);
    e.mem.set_u32(r.ebp.wrapping_sub(4), 0xffff_ffff);
    r.eax = r.ebp.wrapping_sub(0xc);
    r.fs0 = r.eax;
    r.pop(e);
}

// Translated from 00ece2be (disassembled, FalloutNV.exe 1.4.0.525)
/// `__EH_epilog3` (LIBCMT/ehprolg3.c): `FS:[0]` = `[EBP-0xc]`, then the pops of
/// `__SEH_epilog4`.
pub fn eh_epilog3(e: &mut Engine, r: &mut Regs) {
    r.ecx = e.mem.u32(r.ebp.wrapping_sub(0xc));
    r.fs0 = r.ecx;
    epilog_pops(e, r);
}

/// The uniform form of a frame helper: the nine words of [`Regs`] in, `EAX` and `EDX` after.
fn regs_entry(e: &mut Engine, words: &[u32], f: fn(&mut Engine, &mut Regs)) -> Ret {
    let mut r = Regs::from_words(words);
    f(e, &mut r);
    r.answer()
}

fn seh_prolog4_entry(e: &mut Engine, words: &[u32]) -> Ret {
    regs_entry(e, words, seh_prolog4)
}

fn seh_epilog4_entry(e: &mut Engine, words: &[u32]) -> Ret {
    regs_entry(e, words, seh_epilog4)
}

fn eh_prolog3_entry(e: &mut Engine, words: &[u32]) -> Ret {
    regs_entry(e, words, eh_prolog3)
}

fn eh_prolog3_catch_entry(e: &mut Engine, words: &[u32]) -> Ret {
    regs_entry(e, words, eh_prolog3_catch)
}

fn eh_epilog3_entry(e: &mut Engine, words: &[u32]) -> Ret {
    regs_entry(e, words, eh_epilog3)
}

// Translated from 00ec5ed6 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__alloca_probe_8` (Ghidra's label; the 8-byte aligned `alloca` probe): `EAX` the size,
/// then `ESP` at entry. The size grows by `(esp + 4 - size) & 7`, all ones when that carries,
/// and goes on to `__alloca_probe` with the same `ESP` (a jump), whose answer (the caller's new
/// `ESP`) it gives. The 16-byte one is `glue.rs`'s `alloca_probe_16`.
pub fn alloca_probe_8(e: &mut Engine, size: u32, esp: u32) -> u32 {
    let low = esp.wrapping_add(4).wrapping_sub(size) & 7;
    let (sum, carry) = size.overflowing_add(low);
    let size = if carry { 0xffff_ffff } else { sum };
    e.call(ALLOCA_PROBE, &args![size, esp]).u32()
}

// ---- The non-local-goto hooks -----------------------------------------------------------------

// Translated from 00edf5ed (disassembled, FalloutNV.exe 1.4.0.525)
/// `__NLG_Notify` (LIBCMT/nlgsupp.asm; `EAX` the address, `EBP` the frame, then the code; `RET
/// 4`): the debugger's non-local-goto block (`011c2a88`) gets the code (`+8`), the address
/// (`+4`) and the frame (`+0xc`); the pushes and pops at `__NLG_Dispatch` that follow change
/// nothing. `EAX` is answered as it came.
pub fn nlg_notify(e: &mut Engine, eax: u32, ebp: u32, code: u32) -> u32 {
    nlg_fill(e, eax, code, ebp)
}

// Translated from 00edf5e4 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__NLG_Notify1` (LIBCMT/nlgsupp.asm; `EAX` the address, `ECX` the code, `EBP` the frame,
/// then a word that its `RET 4` drops): [`nlg_notify`] with the code from `ECX`.
pub fn nlg_notify1(e: &mut Engine, eax: u32, ecx: u32, ebp: u32, _dropped: u32) -> u32 {
    nlg_fill(e, eax, ecx, ebp)
}

/// The block `__NLG_Notify` and `__NLG_Notify1` fill (`011c2a88`).
fn nlg_fill(e: &mut Engine, eax: u32, code: u32, ebp: u32) -> u32 {
    e.mem.set_u32(NLG_DATA + 8, code);
    e.mem.set_u32(NLG_DATA + 4, eax);
    e.mem.set_u32(NLG_DATA + 0xc, ebp);
    eax
}

// Translated from 00edf60c (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00edf60c - the funclet is entered with EBP = the frame and called with that one word; the older __finally translations (glue.rs and others) take their frame words instead.
/// `__NLG_Call` (LIBCMT/nlgsupp.asm; `EAX` the funclet, `EBP` the frame): `CALL EAX`; the
/// funclet's `EAX` is answered.
pub fn nlg_call(e: &mut Engine, funclet: u32, ebp: u32) -> u32 {
    e.call(funclet, &args![ebp]).u32()
}

// Translated from 00ed1550 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed1550 - the funclet is entered with EBP = node + 0xc and called with that one word; the older funclet translations take their frame words instead, and the EBP the funclet leaves is taken as the one it got.
/// `_CallSettingFrame@12` (LIBCMT/lowhelpr.asm; stdcall: the funclet, the registration node,
/// the non-local-goto code). The funclet's frame is the node + 0xc; `__NLG_Notify1` with the
/// funclet and the code, the funclet called on that frame, `__NLG_Notify1` again with its
/// answer and the code (0x100 becomes 2) on the frame it leaves. Answers the funclet's `EAX`
/// (a catch block's continuation address).
pub fn call_setting_frame(e: &mut Engine, funclet: u32, node: u32, code: u32) -> u32 {
    let frame = node.wrapping_add(0xc);
    e.call(NLG_NOTIFY1, &args![funclet, code, frame, code]);
    let answer = e.call(funclet, &args![frame]).u32();
    let code = if code == 0x100 { 2 } else { code };
    e.call(NLG_NOTIFY1, &args![answer, code, frame, code]);
    answer
}

// ---- __except_handler4 ------------------------------------------------------------------------

// Translated from 00ed872e (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed872e - the filter is entered with EBP = the frame and called with that one word; the older filter translations take their frame words instead.
/// `_EH4_CallFilterFunc` (LIBCMT/exsup4.asm; `ECX` the filter, `EDX` the guarded function's
/// frame): the filter called with `EBP` = the frame and the other registers zeroed; its
/// answer (`EXCEPTION_EXECUTE_HANDLER` 1, `EXCEPTION_CONTINUE_SEARCH` 0,
/// `EXCEPTION_CONTINUE_EXECUTION` -1) given back.
pub fn eh4_call_filter_func(e: &mut Engine, filter: u32, frame: u32) -> u32 {
    e.call(filter, &args![frame]).u32()
}

// Translated from 00ed8745 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed8745 - the jump to the handler block (EBP = the frame, the stack left as the dispatcher had it) is a call with the frame as its one word; in the exe it never comes back.
/// `_EH4_TransferToHandler` (LIBCMT/exsup4.asm; `ECX` the `__except` block, `EDX` the frame):
/// `__NLG_Notify` with the block and code 1, then a jump to the block on the frame. Answers
/// what the block answers.
pub fn eh4_transfer_to_handler(e: &mut Engine, handler: u32, frame: u32) -> u32 {
    e.call(NLG_NOTIFY, &args![handler, frame, 1u32]);
    e.call(handler, &args![frame]).u32()
}

// Translated from 00ed875e (disassembled, FalloutNV.exe 1.4.0.525)
/// `_EH4_GlobalUnwind` (LIBCMT/exsup4.asm; `ECX` the registration node): `RtlUnwind(node,
/// 00ed8773, 0, 0)`, the instruction after the call as the target.
pub fn eh4_global_unwind(e: &mut Engine, node: u32) {
    e.call(
        RTL_UNWIND,
        &args![node, EH4_GLOBAL_UNWIND_TARGET, 0u32, 0u32],
    );
}

// Translated from 00ed8778 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_EH4_LocalUnwind` (LIBCMT/exsup4.asm; `ECX` the node, `EDX` the stop level, then the
/// guarded function's frame and the cookie's address; `RET 8`): `__local_unwind4` with `EBP` =
/// the frame.
pub fn eh4_local_unwind(e: &mut Engine, node: u32, stop: u32, frame: u32, cookie: u32) {
    e.call(LOCAL_UNWIND4, &args![frame, cookie, node, stop]);
}

// Translated from 00ed863c (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed863c - its own registration node (handler 00ed86cc) is not linked at FS:[0]; the termination handlers are called with EBP as their one word (the older __finally translations take their frame words).
/// `__local_unwind4` (LIBCMT/exsup4.asm; `EBP` the guarded function's frame, then the
/// cookie's address, the registration node and the stop level). The scope table is the node's
/// (`+8`) XOR the cookie; from the node's try level (`+0xc`) down to the stop level: unless it
/// is -2, or the stop level is not -2 and the level is at or below it (unsigned), the level's
/// entry (`table + 0x10 + level * 12`: enclosing level, filter, handler) gives the node its
/// enclosing level; an entry without a filter is a `__finally` block, run through
/// `__NLG_Notify` (code 0x101) and `__NLG_Call` on the frame.
pub fn local_unwind4(e: &mut Engine, ebp: u32, cookie: u32, node: u32, stop: u32) {
    loop {
        let table = e.mem.u32(node.wrapping_add(8)) ^ e.mem.u32(cookie);
        let level = e.mem.u32(node.wrapping_add(0xc));
        if level == 0xffff_fffe || (stop != 0xffff_fffe && level <= stop) {
            return;
        }
        let entry = table
            .wrapping_add(level.wrapping_mul(12))
            .wrapping_add(0x10);
        e.mem.set_u32(node.wrapping_add(0xc), e.mem.u32(entry));
        if e.mem.u32(entry.wrapping_add(4)) == 0 {
            let handler = e.mem.u32(entry.wrapping_add(8));
            e.call(NLG_NOTIFY, &args![handler, ebp, 0x101u32]);
            let handler = e.mem.u32(entry.wrapping_add(8));
            e.call(NLG_CALL, &args![handler, ebp]);
        }
    }
}

// Translated from 00ed86cc (disassembled, FalloutNV.exe 1.4.0.525)
/// The handler of `__local_unwind4`'s registration node (cdecl: record, node, context,
/// dispatcher context): outside an unwind (record flags `& 6` clear) 1
/// (`ExceptionContinueSearch`). Otherwise the node's cookie (`+8` XOR the node) is checked, the
/// interrupted unwind goes on (`__local_unwind4` with `EBP` = the node's `+0x18`, its cookie
/// address `+0x14`, node `+0x10` and stop level `+0xc`), the dispatcher context gets the node,
/// and the answer is 3 (`ExceptionCollidedUnwind`).
pub fn local_unwind4_handler(
    e: &mut Engine,
    record: u32,
    node: u32,
    _context: u32,
    dispatcher: u32,
) -> u32 {
    if e.mem.u32(record.wrapping_add(4)) & 6 == 0 {
        return 1;
    }
    let cookie = e.mem.u32(node.wrapping_add(8)) ^ node;
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
    let frame = e.mem.u32(node.wrapping_add(0x18));
    let words = [
        frame,
        e.mem.u32(node.wrapping_add(0x14)),
        e.mem.u32(node.wrapping_add(0x10)),
        e.mem.u32(node.wrapping_add(0xc)),
    ];
    e.call(LOCAL_UNWIND4, &words);
    e.mem.set_u32(dispatcher, node);
    3
}

// Translated from 00ed8712 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__local_unwind4` call of `longjmp` (stdcall: the `jmp_buf`; `_seh_longjmp_unwind4@4` by
/// its use): `EBP` = the buffer's `+0`, then the cookie's address `+0x28`, the node `+0x18` and
/// the level `+0x1c`.
pub fn seh_longjmp_unwind4(e: &mut Engine, buffer: u32) {
    let words = [
        e.mem.u32(buffer),
        e.mem.u32(buffer.wrapping_add(0x28)),
        e.mem.u32(buffer.wrapping_add(0x18)),
        e.mem.u32(buffer.wrapping_add(0x1c)),
    ];
    e.call(LOCAL_UNWIND4, &words);
}

/// The two `/GS` checks `__except_handler4` makes on a scope table (`table`) of the frame
/// `frame`: unless the table's `/GS` cookie offset (`+0`) is -2, `[frame + gs offset]` XOR
/// (`frame` + the `+4` XOR offset); then `[frame + eh offset (+8)]` XOR (`frame` + the `+0xc`
/// XOR offset), each through `__security_check_cookie`.
fn check_scope_cookies(e: &mut Engine, table: u32, frame: u32) {
    let gs = e.mem.u32(table);
    if gs != 0xffff_fffe {
        let value = e.mem.u32(table.wrapping_add(4)).wrapping_add(frame)
            ^ e.mem.u32(gs.wrapping_add(frame));
        e.call(SECURITY_CHECK_COOKIE, &args![value]);
    }
    let eh = e.mem.u32(table.wrapping_add(8));
    let value =
        e.mem.u32(table.wrapping_add(0xc)).wrapping_add(frame) ^ e.mem.u32(eh.wrapping_add(frame));
    e.call(SECURITY_CHECK_COOKIE, &args![value]);
}

// Translated from 00ecc8e0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecc8e0 - the jump into the __except block never comes back in the exe; here _EH4_TransferToHandler returns and the run stops.
/// `__except_handler4` (LIBCMT/chandler4.c; cdecl: record, registration node, context,
/// dispatcher context). The scope table is the node's `+8` XOR the cookie, the guarded
/// function's frame the node + 0x10; its cookies are checked. Outside an unwind (record flags
/// `& 0x66` clear) the node's `-4` gets the address of a local `EXCEPTION_POINTERS` (record,
/// context) and the try levels are walked from the node's (`+0xc`) out through each entry's
/// enclosing level until -2: an entry with a filter runs it (`_EH4_CallFilterFunc`); a negative
/// answer gives 0 (`ExceptionContinueExecution`), a positive one runs the handler: a C++
/// exception first gets `__DestructExceptionObject(record, 1)` when the pointer at `010f20a0`
/// is set and in the image, then `_EH4_GlobalUnwind`, `_EH4_LocalUnwind` to the level unless
/// the node is already at it, the node's level set to the entry's enclosing one, the cookies
/// checked and `_EH4_TransferToHandler` of the entry's handler. When a filter ran, the cookies
/// are checked again on the way out. During an unwind the node is unwound to -2
/// (`_EH4_LocalUnwind`) unless it is there, then the cookies are checked. Answers 1
/// (`ExceptionContinueSearch`) otherwise.
pub fn except_handler4(
    e: &mut Engine,
    record: u32,
    node: u32,
    context: u32,
    _dispatcher: u32,
) -> u32 {
    let table = e.mem.u32(node.wrapping_add(8)) ^ e.mem.u32(SECURITY_COOKIE);
    let frame = node.wrapping_add(0x10);
    check_scope_cookies(e, table, frame);
    if e.mem.u8(record.wrapping_add(4)) & 0x66 != 0 {
        if e.mem.u32(node.wrapping_add(0xc)) != 0xffff_fffe {
            e.call(
                EH4_LOCAL_UNWIND,
                &args![node, 0xffff_fffeu32, frame, SECURITY_COOKIE],
            );
            check_scope_cookies(e, table, frame);
        }
        return 1;
    }
    e.with_stack(8, |e, pointers| {
        let pointers = pointers.addr();
        e.mem.set_u32(pointers, record);
        e.mem.set_u32(pointers + 4, context);
        e.mem.set_u32(node.wrapping_sub(4), pointers);
        let mut answer = 1u32;
        let mut filtered = false;
        let mut level = e.mem.u32(node.wrapping_add(0xc));
        while level != 0xffff_fffe {
            let entry = table
                .wrapping_add(level.wrapping_mul(12))
                .wrapping_add(0x10);
            let filter = e.mem.u32(entry.wrapping_add(4));
            let enclosing = e.mem.u32(entry);
            if filter != 0 {
                let verdict = e.call(EH4_CALL_FILTER_FUNC, &args![filter, frame]).i32();
                filtered = true;
                if verdict < 0 {
                    answer = 0;
                    break;
                }
                if verdict > 0 {
                    run_handler4(e, record, node, table, frame, level, entry, enclosing);
                    panic!("00ecc8e0: _EH4_TransferToHandler returned");
                }
            }
            level = enclosing;
        }
        if filtered {
            check_scope_cookies(e, table, frame);
        }
        answer
    })
}

/// `__except_handler4`'s handler path (`00ecc9bf`): the entry at `level` took the exception.
#[allow(clippy::too_many_arguments)]
fn run_handler4(
    e: &mut Engine,
    record: u32,
    node: u32,
    table: u32,
    frame: u32,
    level: u32,
    entry: u32,
    enclosing: u32,
) {
    if e.mem.u32(record) == CXX_EXCEPTION && e.mem.u32(DESTRUCT_POINTER) != 0 {
        let image = e
            .call(IS_NONWRITABLE_IN_CURRENT_IMAGE, &args![DESTRUCT_POINTER])
            .u32();
        if image != 0 {
            let destruct = e.mem.u32(DESTRUCT_POINTER);
            e.call(destruct, &args![record, 1u32]);
        }
    }
    e.call(EH4_GLOBAL_UNWIND, &args![node]);
    if e.mem.u32(node.wrapping_add(0xc)) != level {
        e.call(
            EH4_LOCAL_UNWIND,
            &args![node, level, frame, SECURITY_COOKIE],
        );
    }
    e.mem.set_u32(node.wrapping_add(0xc), enclosing);
    check_scope_cookies(e, table, frame);
    let handler = e.mem.u32(entry.wrapping_add(8));
    e.call(EH4_TRANSFER_TO_HANDLER, &args![handler, frame]);
}

// ---- __except_handler3 ------------------------------------------------------------------------

// Translated from 00edf4d8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__global_unwind2` (LIBCMT/exsup.asm; cdecl: the frame): `RtlUnwind(frame, 00edf4f0, 0,
/// 0)`, the instruction after the call as the target.
pub fn global_unwind2(e: &mut Engine, frame: u32) {
    e.call(RTL_UNWIND, &args![frame, GLOBAL_UNWIND2_TARGET, 0u32, 0u32]);
}

// Translated from 00edf53d (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00edf53d - its own registration node (handler 00edf4f8, with its /GS word) is not linked at FS:[0]; the termination handlers are called with EBP as their one word (the older __finally translations take their frame words).
/// `__local_unwind2` (LIBCMT/exsup.asm; `EBP` the guarded function's frame, then the node and
/// the stop level). From the node's try level (`+0xc`) down: unless it is -1, or the stop level
/// is not -1 and the level is at or below it (unsigned), the level's entry in the node's scope
/// table (`+8`, 12-byte entries: enclosing level, filter, handler) gives the node its enclosing
/// level; an entry without a filter is a `__finally` block, run through `__NLG_Notify` (code
/// 0x101) and `__NLG_Call` on the frame.
pub fn local_unwind2(e: &mut Engine, ebp: u32, node: u32, stop: u32) {
    loop {
        let table = e.mem.u32(node.wrapping_add(8));
        let level = e.mem.u32(node.wrapping_add(0xc));
        if level == 0xffff_ffff || (stop != 0xffff_ffff && level <= stop) {
            return;
        }
        let entry = table.wrapping_add(level.wrapping_mul(12));
        e.mem.set_u32(node.wrapping_add(0xc), e.mem.u32(entry));
        if e.mem.u32(entry.wrapping_add(4)) == 0 {
            let handler = e.mem.u32(entry.wrapping_add(8));
            e.call(NLG_NOTIFY, &args![handler, ebp, 0x101u32]);
            let handler = e.mem.u32(entry.wrapping_add(8));
            e.call(NLG_CALL, &args![handler, ebp]);
        }
    }
}

// Translated from 00edf4f8 (disassembled, FalloutNV.exe 1.4.0.525)
/// The handler of `__local_unwind2`'s registration node (cdecl: record, node, context,
/// dispatcher context; it also reads the word above them, `[ESP+0x14]`, as the node whose
/// unwind was interrupted). Outside an unwind (record flags `& 6` clear) 1. Otherwise that
/// node's `/GS` word (`-4` XOR the node) is checked, the unwind goes on (`__local_unwind2` with
/// `EBP` = its `+0x10`, node `+0x24` and level `+0x28`), the dispatcher context gets the node
/// argument, and the answer is 3 (`ExceptionCollidedUnwind`).
pub fn local_unwind2_handler(
    e: &mut Engine,
    record: u32,
    node: u32,
    _context: u32,
    dispatcher: u32,
    interrupted: u32,
) -> u32 {
    if e.mem.u32(record.wrapping_add(4)) & 6 == 0 {
        return 1;
    }
    let cookie = e.mem.u32(interrupted.wrapping_sub(4)) ^ interrupted;
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
    let words = [
        e.mem.u32(interrupted.wrapping_add(0x10)),
        e.mem.u32(interrupted.wrapping_add(0x24)),
        e.mem.u32(interrupted.wrapping_add(0x28)),
    ];
    e.call(LOCAL_UNWIND2, &words);
    e.mem.set_u32(dispatcher, node);
    3
}

// Translated from 00ece202 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__local_unwind2` call of `longjmp` (stdcall: the `jmp_buf`; `_seh_longjmp_unwind@4` by
/// its use): `EBP` = the buffer's `+0`, then the node `+0x18` and the level `+0x1c`.
pub fn seh_longjmp_unwind(e: &mut Engine, buffer: u32) {
    let words = [
        e.mem.u32(buffer),
        e.mem.u32(buffer.wrapping_add(0x18)),
        e.mem.u32(buffer.wrapping_add(0x1c)),
    ];
    e.call(LOCAL_UNWIND2, &words);
}

// Translated from 00ece104 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ece104 - filters and the __except block are entered with EBP = the node + 0x10 and called with that one word (the older funclet translations take their frame words); the jump into the block never comes back in the exe, here the run stops if it does.
/// `__except_handler3` (LIBCMT/exsup3.asm; cdecl: record, registration node, context,
/// dispatcher context). During an unwind (record flags `& 6`) the node is unwound to -1
/// (`__local_unwind2` on the frame node + 0x10) and the answer is 1. Otherwise the node's `-4`
/// gets the address of a local `EXCEPTION_POINTERS` (record, context) and the node is checked
/// (`__ValidateEH3RN`): not valid (0 or less) sets the record's flag 8
/// (`EXCEPTION_STACK_INVALID`) and answers 1. The try levels are walked from the node's
/// (`+0xc`) out through each entry's enclosing level until -1 (answer 1): a filter (entry
/// `+4`) runs on the frame; 0 goes on, negative answers 0 (`ExceptionContinueExecution`),
/// positive runs `_CallDestructExceptionObject(record, 1)`, `__global_unwind2(node)`,
/// `__local_unwind2` to the level, `__NLG_Notify` (code 1) and the node's level set to the
/// entry's enclosing one, then the entry's handler (`+8`) on the frame.
pub fn except_handler3(
    e: &mut Engine,
    record: u32,
    node: u32,
    context: u32,
    _dispatcher: u32,
) -> u32 {
    let frame = node.wrapping_add(0x10);
    if e.mem.u32(record.wrapping_add(4)) & 6 != 0 {
        e.call(LOCAL_UNWIND2, &args![frame, node, 0xffff_ffffu32]);
        return 1;
    }
    e.with_stack(8, |e, pointers| {
        let pointers = pointers.addr();
        e.mem.set_u32(pointers, record);
        e.mem.set_u32(pointers + 4, context);
        e.mem.set_u32(node.wrapping_sub(4), pointers);
        let mut level = e.mem.u32(node.wrapping_add(0xc));
        let mut table = e.mem.u32(node.wrapping_add(8));
        if (e.call(VALIDATE_EH3RN, &args![node]).i32()) <= 0 {
            let flags = e.mem.u32(record.wrapping_add(4));
            e.mem.set_u32(record.wrapping_add(4), flags | 8);
            return 1;
        }
        while level != 0xffff_ffff {
            let entry = table.wrapping_add(level.wrapping_mul(12));
            let filter = e.mem.u32(entry.wrapping_add(4));
            if filter != 0 {
                let verdict = e.call(filter, &args![frame]).i32();
                if verdict < 0 {
                    return 0;
                }
                if verdict > 0 {
                    e.call(CALL_DESTRUCT_EXCEPTION_OBJECT, &args![record, 1u32]);
                    table = e.mem.u32(node.wrapping_add(8));
                    e.call(GLOBAL_UNWIND2, &args![node]);
                    e.call(LOCAL_UNWIND2, &args![frame, node, level]);
                    let entry = table.wrapping_add(level.wrapping_mul(12));
                    let handler = e.mem.u32(entry.wrapping_add(8));
                    e.call(NLG_NOTIFY, &args![handler, frame, 1u32]);
                    e.mem.set_u32(node.wrapping_add(0xc), e.mem.u32(entry));
                    let handler = e.mem.u32(entry.wrapping_add(8));
                    e.call(handler, &args![frame]);
                    panic!("00ece104: the __except block returned");
                }
            }
            table = e.mem.u32(node.wrapping_add(8));
            level = e.mem.u32(table.wrapping_add(level.wrapping_mul(12)));
        }
        1
    })
}

/// The running thread's stack range as its thread environment block holds it (`FS:[0x18]`,
/// then `+8` the limit and `+4` the base). The engine keeps no thread environment block and its
/// threads keep no stack in game memory, so there is no range to answer: the run stops here
/// rather than guess one. [`validate_eh3rn_with_stack`] takes the range as an input instead.
fn thread_stack_range() -> (u32, u32) {
    panic!(
        "00edf6d0: the thread's stack range (the TEB's StackLimit and StackBase, FS:[0x18] +8 \
         and +4) is not modelled"
    );
}

// Translated from 00edf6d0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00edf6d0 - its own SEH frame (scope table 011804e8, __except_handler4) is not built and its __except path (filter 00edf8aa, block 00edf8be) is not reached; the TEB's stack range is not modelled: the registered form stops where the exe reads it (thread_stack_range), validate_eh3rn_with_stack takes it as an input.
/// `__ValidateEH3RN` (LIBCMT/eh3valid.c; cdecl: the registration node). Answers 0 for a node
/// that must not be used: a scope table (`+8`) not 4-byte aligned or on the thread's stack, an
/// entry whose enclosing level is not -1 and not below its own index, or (when any entry has a
/// filter) a saved stack pointer (`-8`) outside the stack below the node. A scope table in a
/// page the cache (`01270740`, 16 entries of page and image base, count `01270738`) knows, in
/// a valid image whose handlers lie in executable sections (`_ValidateScopeTableHandlers`) and
/// whose handler (`+4`) is in a section, answers 1 and moves to the front of the cache (under
/// the lock word `0127073c`, skipped when it is taken). Otherwise `VirtualQuery` decides: no
/// region answers 1; not an image, or not a valid image base, answers -1; a writable region
/// (`Protect & 0xcc`) whose table is outside a section or in a writable one, handlers outside
/// executable sections, or a node handler outside a section answers 0; else the page goes
/// into the cache (its image base updated if it is there, else inserted at the front) and the
/// answer is 1.
pub fn validate_eh3rn(e: &mut Engine, node: u32) -> u32 {
    let table = e.mem.u32(node.wrapping_add(8));
    if table & 3 != 0 {
        return 0;
    }
    let (limit, base) = thread_stack_range();
    validate_eh3rn_rest(e, node, table, limit, base)
}

/// [`validate_eh3rn`] with the thread's stack range given (`limit`, `base`: the TEB's
/// `StackLimit` and `StackBase`), as [`Regs`] gives the frame helpers `FS:[0]`.
pub fn validate_eh3rn_with_stack(e: &mut Engine, node: u32, limit: u32, base: u32) -> u32 {
    let table = e.mem.u32(node.wrapping_add(8));
    if table & 3 != 0 {
        return 0;
    }
    validate_eh3rn_rest(e, node, table, limit, base)
}

/// `__ValidateEH3RN` from the stack-range test on (`00edf727`).
fn validate_eh3rn_rest(e: &mut Engine, node: u32, table: u32, limit: u32, base: u32) -> u32 {
    if table >= limit && table < base {
        return 0;
    }
    let highest = e.mem.u32(node.wrapping_add(0xc));
    if highest == 0xffff_ffff {
        return 1;
    }
    let mut any_filter = false;
    let mut index = 0u32;
    let mut entry = table;
    loop {
        let enclosing = e.mem.u32(entry);
        if enclosing != 0xffff_ffff && enclosing >= index {
            return 0;
        }
        if e.mem.u32(entry.wrapping_add(4)) != 0 {
            any_filter = true;
        }
        index = index.wrapping_add(1);
        entry = entry.wrapping_add(0xc);
        if index > highest {
            break;
        }
    }
    if any_filter {
        let saved_esp = e.mem.u32(node.wrapping_sub(8));
        if saved_esp < limit || saved_esp >= node {
            return 0;
        }
    }
    let page = table & 0xffff_f000;
    let mut slot = 0i32;
    let count = e.mem.i32(EH3_CACHE_COUNT);
    while slot < count {
        let at = EH3_CACHE.wrapping_add((slot as u32).wrapping_mul(8));
        let (cached_page, image) = (e.mem.u32(at), e.mem.u32(at.wrapping_add(4)));
        if cached_page == page {
            if cached_image_is_valid(e, node, table, highest, image) {
                return move_to_front(e, slot, page, cached_page, image);
            }
            break;
        }
        slot += 1;
    }
    query_region(e, node, table, highest, page)
}

/// `__ValidateEH3RN`'s checks of a cached page (`00edf7b2`..`00edf7f5`, inside its `__try`):
/// the image base is valid, the handlers lie in executable sections, the node's handler is in
/// a section.
fn cached_image_is_valid(e: &mut Engine, node: u32, table: u32, highest: u32, image: u32) -> bool {
    if e.call(VALIDATE_IMAGE_BASE, &args![image]).u32() == 0 {
        return false;
    }
    if e.call(VALIDATE_SCOPE_TABLE_HANDLERS, &args![highest, image, table])
        .u32()
        == 0
    {
        return false;
    }
    let handler = e.mem.u32(node.wrapping_add(4));
    e.call(FIND_PE_SECTION, &args![image, handler.wrapping_sub(image)])
        .u32()
        != 0
}

/// `XCHG [0127073c], value`: the lock word's old value.
fn swap_lock(e: &mut Engine, value: u32) -> u32 {
    let old = e.mem.u32(EH3_CACHE_LOCK);
    e.mem.set_u32(EH3_CACHE_LOCK, value);
    old
}

/// `__ValidateEH3RN` for a page found in the cache at `slot` (`00edf802`): the entry moves to
/// the front under the lock (found again from the end if another thread moved it; added at the
/// end, the count grown up to 16, when it is gone). Answers 1.
fn move_to_front(e: &mut Engine, slot: i32, page: u32, mut first: u32, mut second: u32) -> u32 {
    if slot <= 0 || swap_lock(e, 1) != 0 {
        return 1;
    }
    let entry = |i: i32| EH3_CACHE.wrapping_add((i as u32).wrapping_mul(8));
    let mut last = slot;
    if e.mem.u32(entry(slot)) != page {
        let mut count = e.mem.i32(EH3_CACHE_COUNT);
        last = count - 1;
        while last >= 0 && e.mem.u32(entry(last)) != page {
            last -= 1;
        }
        if last >= 0 {
            first = e.mem.u32(entry(last));
            second = e.mem.u32(entry(last).wrapping_add(4));
        } else {
            if count < 0x10 {
                count += 1;
                e.mem.set_u32(EH3_CACHE_COUNT, count as u32);
            }
            last = count - 1;
        }
    }
    if last > 0 {
        for i in 0..=last {
            let at = entry(i);
            let (old_first, old_second) = (e.mem.u32(at), e.mem.u32(at.wrapping_add(4)));
            e.mem.set_u32(at, first);
            e.mem.set_u32(at.wrapping_add(4), second);
            first = old_first;
            second = old_second;
        }
    }
    swap_lock(e, 0);
    1
}

/// `__ValidateEH3RN` past the cache (`00edf8c1`): `VirtualQuery` of the scope table into a
/// local `MEMORY_BASIC_INFORMATION` (0x1c bytes: `+4` the allocation base, `+0x14` the
/// protection, `+0x18` the type) and the checks of its region.
fn query_region(e: &mut Engine, node: u32, table: u32, highest: u32, page: u32) -> u32 {
    e.with_stack(0x1c, |e, info| {
        let info = info.addr();
        if e.call(IMPORT_VIRTUAL_QUERY, &args![table, info, 0x1cu32])
            .u32()
            == 0
        {
            return 1;
        }
        if e.mem.u32(info + 0x18) != MEM_IMAGE {
            return 0xffff_ffff;
        }
        let image = e.mem.u32(info + 4);
        if e.call(VALIDATE_IMAGE_BASE, &args![image]).u32() == 0 {
            return 0xffff_ffff;
        }
        if e.mem.u8(info + 0x14) & 0xcc != 0 {
            let section = e
                .call(FIND_PE_SECTION, &args![image, table.wrapping_sub(image)])
                .u32();
            if section == 0 || e.mem.u32(section.wrapping_add(0x24)) & 0x8000_0000 != 0 {
                return 0;
            }
        }
        if e.call(VALIDATE_SCOPE_TABLE_HANDLERS, &args![highest, image, table])
            .u32()
            == 0
        {
            return 0;
        }
        let handler = e.mem.u32(node.wrapping_add(4));
        if e.call(FIND_PE_SECTION, &args![image, handler.wrapping_sub(image)])
            .u32()
            == 0
        {
            return 0;
        }
        if swap_lock(e, 1) != 0 {
            return 1;
        }
        let count = e.mem.i32(EH3_CACHE_COUNT);
        let mut found = count;
        if count > 0 {
            let mut at = EH3_CACHE_COUNT.wrapping_add((count as u32).wrapping_mul(8));
            while e.mem.u32(at) != page {
                found -= 1;
                at = at.wrapping_sub(8);
                if found <= 0 {
                    break;
                }
            }
        }
        if found != 0 {
            let at = EH3_CACHE_LOCK.wrapping_add((found as u32).wrapping_mul(8));
            e.mem.set_u32(at, image);
        } else {
            let last = if count > 0xf { 0xf } else { count };
            let mut count = count;
            if last >= 0 {
                let (mut first, mut second) = (page, image);
                let mut at = EH3_CACHE;
                for _ in 0..=last {
                    let (old_first, old_second) = (e.mem.u32(at), e.mem.u32(at + 4));
                    e.mem.set_u32(at, first);
                    e.mem.set_u32(at + 4, second);
                    first = old_first;
                    second = old_second;
                    at += 8;
                }
                count = e.mem.i32(EH3_CACHE_COUNT);
            }
            if count < 0x10 {
                e.mem.set_u32(EH3_CACHE_COUNT, (count + 1) as u32);
            }
        }
        swap_lock(e, 0);
        1
    })
}

// Translated from 00edf8aa (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` filter of `__ValidateEH3RN` (part of `00edf6d0`, entered with `EBP` = its
/// frame): 1 (`EXCEPTION_EXECUTE_HANDLER`) when the exception (`[EBP-0x14]`, the
/// `EXCEPTION_POINTERS`) is an access violation, else 0.
pub fn validate_eh3rn_filter(e: &mut Engine, ebp: u32) -> u32 {
    let pointers = e.mem.u32(ebp.wrapping_sub(0x14));
    let record = e.mem.u32(pointers);
    u32::from(e.mem.u32(record) == ACCESS_VIOLATION)
}

// Translated from 00edf610 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_ValidateScopeTableHandlers` (LIBCMT/eh3valid.c; `ECX` the highest try level, `EDI` the
/// image base, then the scope table). From the highest level out through each entry's
/// enclosing level until -1: the page (RVA `& !0xfff`) of the entry's handler (`+8`) and of
/// its filter (`+4`, when set) must lie in a section with `IMAGE_SCN_MEM_EXECUTE` (0x20000000)
/// set; a page equal to the last one checked, or inside the last section found (`+0xc` its
/// RVA, `+8` its size), is not looked up again (`__FindPESection`). Answers 1 when all do, 0
/// at the first that does not.
pub fn validate_scope_table_handlers(e: &mut Engine, highest: u32, image: u32, table: u32) -> u32 {
    let mut section = 0u32;
    let mut last_page = 0xffff_ffffu32;
    let mut level = highest;
    // In the section found last (`[EAX+0xc] <= page < [EAX+0xc] + [EAX+8]`), or looked up.
    let check = |e: &mut Engine, page: u32, section: &mut u32| -> bool {
        if *section != 0 {
            let start = e.mem.u32(section.wrapping_add(0xc));
            if page >= start && page < e.mem.u32(section.wrapping_add(8)).wrapping_add(start) {
                return true;
            }
        }
        *section = e.call(FIND_PE_SECTION, &args![image, page]).u32();
        *section != 0 && e.mem.u32(section.wrapping_add(0x24)) & 0x2000_0000 != 0
    };
    while level != 0xffff_ffff {
        let entry = table.wrapping_add(level.wrapping_mul(12));
        let page = e.mem.u32(entry.wrapping_add(8)).wrapping_sub(image) & 0xffff_f000;
        if page != last_page {
            if !check(e, page, &mut section) {
                return 0;
            }
            last_page = page;
        }
        let filter = e.mem.u32(entry.wrapping_add(4));
        if filter != 0 {
            let page = filter.wrapping_sub(image) & 0xffff_f000;
            if page != last_page {
                // The exe tests the section found last without checking it is set (one always
                // is by here: the first handler's page differs from the starting -1).
                let start = e.mem.u32(section.wrapping_add(0xc));
                let inside =
                    page >= start && page < e.mem.u32(section.wrapping_add(8)).wrapping_add(start);
                if !inside {
                    section = e.call(FIND_PE_SECTION, &args![image, page]).u32();
                    if section == 0 || e.mem.u32(section.wrapping_add(0x24)) & 0x2000_0000 == 0 {
                        return 0;
                    }
                }
                last_page = page;
            }
        }
        level = e.mem.u32(entry);
    }
    1
}

// ---- The __except handler blocks --------------------------------------------------------------
//
// Each is entered with `EBP` = its function's frame (its one word); it takes back the stack the
// frame kept at `[EBP-0x18]`, does its work, and leaves through `__SEH_epilog4` (whose `CALL`
// pushes the address after it) and its own `RET`, which return from the guarded function. The
// answer is `EAX` at that point.

/// The registers a handler block starts with: `ESP` from `[EBP-0x18]`, `EBP` the frame.
pub(super) fn block_regs(e: &Engine, ebp: u32) -> Regs {
    Regs {
        esp: e.mem.u32(ebp.wrapping_sub(0x18)),
        ebp,
        ..Regs::default()
    }
}

/// `CALL __SEH_epilog4` (pushing `next`, the address after the call), then the block's `RET`
/// (`pop` bytes more for `RET n`). Answers `EAX`.
pub(super) fn leave_block(e: &mut Engine, r: &mut Regs, next: u32, pop: u32) -> u32 {
    r.push(e, next);
    seh_epilog4(e, r);
    r.pop(e);
    r.esp = r.esp.wrapping_add(pop);
    r.eax
}

// Translated from 00ec44b5 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` block of `__RTDynamicCast` (`00ec43fb`; entered with `EBP` = its frame): the
/// result (`[EBP-0x1c]`) cleared, a `__non_rtti_object` made in the local at `EBP-0x28` from the
/// text at `010f1ee4` ("Access violation - no RTTI data!") and thrown (`_CxxThrowException` with its throw information `0117fc10`),
/// which does not return in the exe; if it does, the run stops.
pub fn rt_dynamic_cast_except(e: &mut Engine, ebp: u32) -> u32 {
    let mut r = block_regs(e, ebp);
    e.mem.set_u32(ebp.wrapping_sub(0x1c), 0);
    r.push(e, NON_RTTI_TEXT);
    let object = ebp.wrapping_sub(0x28);
    e.call(NON_RTTI_OBJECT, &args![object, NON_RTTI_TEXT]);
    e.call(CXX_THROW_EXCEPTION, &args![object, NON_RTTI_THROW_INFO]);
    panic!("00ec44b5: _CxxThrowException returned");
}

// Translated from 00ec5fbc (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` block of `__ArrayUnwind` (`00ec5f70`; entered with `EBP` = its frame): out of
/// the try block and out through `__SEH_epilog4` and `RET 0x10` (its four words).
pub fn array_unwind_except(e: &mut Engine, ebp: u32) -> u32 {
    let mut r = block_regs(e, ebp);
    e.mem.set_u32(ebp.wrapping_sub(4), 0xffff_fffe);
    leave_block(e, &mut r, 0x00ec_5fcb, 0x10)
}

// Translated from 00ecc49c (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` block of `__tmainCRTStartup` (`00ecc35d`; entered with `EBP` = its frame):
/// the exception code (`[EBP-0x24]`, kept by the filter) becomes the answer (`[EBP-0x20]`); an
/// unmanaged image (`[EBP-0x1c]` 0) calls `__exit` with it (its word pushed), then `_c_exit`,
/// and leaves with the answer through `__SEH_epilog4`.
pub fn tmain_crt_startup_except(e: &mut Engine, ebp: u32) -> u32 {
    let mut r = block_regs(e, ebp);
    r.eax = e.mem.u32(ebp.wrapping_sub(0x24));
    e.mem.set_u32(ebp.wrapping_sub(0x20), r.eax);
    if e.mem.u32(ebp.wrapping_sub(0x1c)) == 0 {
        r.push(e, r.eax);
        e.call(EXIT, &args![r.eax]);
    }
    e.call(C_EXIT, &args![]);
    e.mem.set_u32(ebp.wrapping_sub(4), 0xffff_fffe);
    r.eax = e.mem.u32(ebp.wrapping_sub(0x20));
    leave_block(e, &mut r, 0x00ec_c4da, 0)
}

// Translated from 00ecc4c6 (disassembled, FalloutNV.exe 1.4.0.525)
/// The second `__except` block of `__tmainCRTStartup` (`00ecc35d`; entered with `EBP` = its
/// frame): leaves with 0xff through `__SEH_epilog4`.
pub fn tmain_crt_startup_except_ff(e: &mut Engine, ebp: u32) -> u32 {
    let mut r = block_regs(e, ebp);
    e.mem.set_u32(ebp.wrapping_sub(4), 0xffff_fffe);
    r.eax = 0xff;
    leave_block(e, &mut r, 0x00ec_c4da, 0)
}

// Translated from 00ece53c (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` block of `__DestructExceptionObject` (`00ece4f0`; entered with `EBP` = its
/// frame): `terminate`, which does not return in the exe (the code after it is
/// `___AdjustPointer`'s); if it does, the run stops.
pub fn destruct_exception_object_except(e: &mut Engine, _ebp: u32) -> u32 {
    e.call(TERMINATE, &args![]);
    panic!("00ece53c: terminate returned");
}

// Translated from 00ece9e9 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` block of `___BuildCatchObject` (`00ece960`; entered with `EBP` = its frame):
/// `terminate`, which does not return in the exe (the code after it is `CatchIt`'s); if it
/// does, the run stops.
pub fn build_catch_object_except(e: &mut Engine, _ebp: u32) -> u32 {
    e.call(TERMINATE, &args![]);
    panic!("00ece9e9: terminate returned");
}

// Translated from 00ed1910 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` block of the SSE probe `00ed18d7` (entered with `EBP` = its frame): the
/// result (`[EBP-0x1c]`) cleared and answered through `__SEH_epilog4` (the probe's body in
/// `math.rs` takes this path as 0).
pub fn os_sse_probe_except(e: &mut Engine, ebp: u32) -> u32 {
    let mut r = block_regs(e, ebp);
    e.mem.set_u32(ebp.wrapping_sub(0x1c), 0);
    e.mem.set_u32(ebp.wrapping_sub(4), 0xffff_fffe);
    r.eax = e.mem.u32(ebp.wrapping_sub(0x1c));
    leave_block(e, &mut r, 0x00ed_1926, 0)
}

// Translated from 00ed5b85 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed5b85 - MXCSR values other than 0x1f80 are not modelled (as in ___set_fpsr_sse2).
/// The `__except` block of `___set_fpsr_sse2` (`00ed5b3c`; entered with `EBP` = its frame): the
/// processor does not take the denormals-are-zero bit after all, so `011c2538` is cleared and
/// the value (`[EBP+8]`) loaded without it (`LDMXCSR`); out through `__SEH_epilog4`. `EAX` is
/// as the block found it (0 here).
pub fn set_fpsr_sse2_except(e: &mut Engine, ebp: u32) -> u32 {
    let mut r = block_regs(e, ebp);
    e.mem.set_u32(DAZ_SUPPORTED, 0);
    let value = e.mem.u32(ebp.wrapping_add(8)) & 0xffff_ffbf;
    e.mem.set_u32(ebp.wrapping_add(8), value);
    assert!(
        value == MXCSR,
        "LDMXCSR {value:#x}: MXCSR values other than {MXCSR:#x} are not modelled"
    );
    e.mem.set_u32(ebp.wrapping_sub(4), 0xffff_fffe);
    leave_block(e, &mut r, 0x00ed_5bad, 0)
}

// Translated from 00ede1d4 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` block of `__crtInitCritSecAndSpinCount` (`00ede19c`; entered with `EBP` = its
/// frame): an out-of-memory exception (`[EBP-0x20]`, the code the filter kept, is
/// `STATUS_NO_MEMORY`) sets the last error to `ERROR_NOT_ENOUGH_MEMORY`; the result
/// (`[EBP-0x1c]`) is cleared and answered through `__SEH_epilog4`.
pub fn init_crit_sec_except(e: &mut Engine, ebp: u32) -> u32 {
    let mut r = block_regs(e, ebp);
    if e.mem.u32(ebp.wrapping_sub(0x20)) == NO_MEMORY {
        r.push(e, NOT_ENOUGH_MEMORY);
        e.call(IMPORT_SET_LAST_ERROR, &args![NOT_ENOUGH_MEMORY]);
        r.esp = r.esp.wrapping_add(4);
    }
    e.mem.set_u32(ebp.wrapping_sub(0x1c), 0);
    e.mem.set_u32(ebp.wrapping_sub(4), 0xffff_fffe);
    r.eax = e.mem.u32(ebp.wrapping_sub(0x1c));
    leave_block(e, &mut r, 0x00ed_e1fb, 0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00ed_1714, seh_prolog4_entry as AbiFn),
        (0x00ed_1759, seh_epilog4_entry as AbiFn),
        (0x00ec_e255, eh_prolog3_entry as AbiFn),
        (0x00ec_e288, eh_prolog3_catch_entry as AbiFn),
        (0x00ec_e2be, eh_epilog3_entry as AbiFn),
        entry!(0x00ec_5ed6, alloca_probe_8(u32, u32) -> u32),
        entry!(0x00ed_f5ed, nlg_notify(u32, u32, u32) -> u32),
        entry!(0x00ed_f5e4, nlg_notify1(u32, u32, u32, u32) -> u32),
        entry!(0x00ed_f60c, nlg_call(u32, u32) -> u32),
        entry!(0x00ed_1550, call_setting_frame(u32, u32, u32) -> u32),
        entry!(0x00ed_872e, eh4_call_filter_func(u32, u32) -> u32),
        entry!(0x00ed_8745, eh4_transfer_to_handler(u32, u32) -> u32),
        entry!(0x00ed_875e, eh4_global_unwind(u32)),
        entry!(0x00ed_8778, eh4_local_unwind(u32, u32, u32, u32)),
        entry!(0x00ed_863c, local_unwind4(u32, u32, u32, u32)),
        entry!(0x00ed_86cc, local_unwind4_handler(u32, u32, u32, u32) -> u32),
        entry!(0x00ed_8712, seh_longjmp_unwind4(u32)),
        entry!(0x00ec_c8e0, except_handler4(u32, u32, u32, u32) -> u32),
        entry!(0x00ed_f4d8, global_unwind2(u32)),
        entry!(0x00ed_f53d, local_unwind2(u32, u32, u32)),
        entry!(0x00ed_f4f8, local_unwind2_handler(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ec_e202, seh_longjmp_unwind(u32)),
        entry!(0x00ec_e104, except_handler3(u32, u32, u32, u32) -> u32),
        entry!(0x00ed_f6d0, validate_eh3rn(u32) -> u32),
        entry!(0x00ed_f8aa, validate_eh3rn_filter(u32) -> u32),
        entry!(0x00ed_f610, validate_scope_table_handlers(u32, u32, u32) -> u32),
        entry!(0x00ec_44b5, rt_dynamic_cast_except(u32) -> u32),
        entry!(0x00ec_5fbc, array_unwind_except(u32) -> u32),
        entry!(0x00ec_c49c, tmain_crt_startup_except(u32) -> u32),
        entry!(0x00ec_c4c6, tmain_crt_startup_except_ff(u32) -> u32),
        entry!(0x00ec_e53c, destruct_exception_object_except(u32) -> u32),
        entry!(0x00ec_e9e9, build_catch_object_except(u32) -> u32),
        entry!(0x00ed_1910, os_sse_probe_except(u32) -> u32),
        entry!(0x00ed_5b85, set_fpsr_sse2_except(u32) -> u32),
        entry!(0x00ed_e1d4, init_crit_sec_except(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The order the logged calls to `addrs` were made in.
    fn order(e: &Engine, addrs: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addrs.contains(a))
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    /// An engine with the cookie (`011c16bc`), the non-local-goto block and `__ValidateEH3RN`'s
    /// cache mapped, the cookie set, and a 0x1000-byte block at `0x0e00_0000` for stacks and
    /// structures.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x2000);
        e.map(0x010f_2000, 0x1000);
        e.map(0x0127_0000, 0x1000);
        e.map(0x0e00_0000, 0x1000);
        e.mem.set_u32(SECURITY_COOKIE, 0x1234_5678);
        e.call_log = Some(vec![]);
        e
    }

    const STACK: u32 = 0x0e00_0800;

    #[test]
    fn the_seh_prolog_builds_the_frame_and_the_epilog_takes_it_down() {
        let mut e = engine();
        // `PUSH 0x10; PUSH scope; CALL __SEH_prolog4` from a function whose caller left its
        // return address at STACK.
        let esp0 = STACK - 0xc;
        e.mem.set_u32(esp0, 0x0040_1234);
        e.mem.set_u32(esp0 + 4, 0x0118_0000);
        e.mem.set_u32(esp0 + 8, 0x10);
        let before = Regs {
            eax: 7,
            ecx: 8,
            edx: 9,
            ebx: 0xb0,
            esp: esp0,
            ebp: 0xe0,
            esi: 0x51,
            edi: 0xd1,
            fs0: 0x0e00_0f00,
        };
        let mut r = before;
        seh_prolog4(&mut e, &mut r);
        let ebp = esp0 + 8;
        assert_eq!(r.ebp, ebp);
        assert_eq!(r.fs0, ebp - 0x10);
        assert_eq!(r.eax, ebp - 0x10);
        // Handler, chain head, locals, three registers, the /GS word (the return address
        // pushed again is popped by the RET).
        assert_eq!(r.esp, esp0 - 8 - 0x10 - 0xc - 4);
        assert_eq!(e.mem.u32(ebp), 0xe0);
        assert_eq!(e.mem.u32(ebp - 4), 0xffff_fffe);
        assert_eq!(e.mem.u32(ebp - 8), 0x0118_0000 ^ 0x1234_5678);
        assert_eq!(e.mem.u32(ebp - 0xc), EXCEPT_HANDLER4);
        assert_eq!(e.mem.u32(ebp - 0x10), 0x0e00_0f00);
        assert_eq!(e.mem.u32(ebp - 0x18), r.esp);
        assert_eq!(e.mem.u32(r.esp), 0x1234_5678 ^ ebp);
        assert_eq!(e.mem.u32(r.esp + 4), 0xd1);
        assert_eq!(e.mem.u32(r.esp + 8), 0x51);
        assert_eq!(e.mem.u32(r.esp + 0xc), 0xb0);
        assert_eq!(e.mem.u32(r.esp - 4), 0x0040_1234);
        // The uniform form answers EAX.
        let mut other = engine();
        other.mem.set_u32(esp0, 0x0040_1234);
        other.mem.set_u32(esp0 + 4, 0x0118_0000);
        other.mem.set_u32(esp0 + 8, 0x10);
        assert_eq!(other.call(0x00ed_1714, &before.words()).eax, ebp - 0x10);
        // The body calls __SEH_epilog4 (its return address pushed) with its answer in EAX.
        r.eax = 0x42;
        r.ebx = 0;
        r.esi = 0;
        r.edi = 0;
        r.push(&mut e, 0x0040_5555);
        seh_epilog4(&mut e, &mut r);
        assert_eq!(r.fs0, 0x0e00_0f00);
        assert_eq!((r.ebx, r.esi, r.edi, r.ebp), (0xb0, 0x51, 0xd1, 0xe0));
        assert_eq!(r.ecx, 0x0040_5555);
        assert_eq!(r.eax, 0x42);
        // Back in the function, whose own RET then pops the caller's return address.
        assert_eq!(r.esp, STACK);
        assert_eq!(e.mem.u32(STACK - 4), 0x0040_5555);
        // The uniform form of the epilog answers EAX as it came.
        let mut words = r.words();
        words[0] = 0x43;
        words[4] = STACK - 0x50;
        words[5] = STACK;
        assert_eq!(e.call(0x00ed_1759, &words).eax, 0x43);
    }

    #[test]
    fn the_eh_prolog3_frames_and_their_epilog() {
        for catch in [false, true] {
            let mut e = engine();
            // `PUSH 0x8; MOV EAX, handler; CALL __EH_prolog3[_catch]`.
            let esp0 = STACK - 8;
            e.mem.set_u32(esp0, 0x0040_1234);
            e.mem.set_u32(esp0 + 4, 8);
            let mut r = Regs {
                eax: 0x00f3_4e90,
                ebx: 0xb0,
                esp: esp0,
                ebp: 0xe0,
                esi: 0x51,
                edi: 0xd1,
                fs0: 0x0e00_0f00,
                ..Regs::default()
            };
            let addr = if catch { 0x00ec_e288 } else { 0x00ec_e255 };
            let ebp = esp0 + 4;
            assert_eq!(e.call(addr, &r.words()).eax, ebp - 0xc);
            e.mem.set_u32(esp0, 0x0040_1234);
            e.mem.set_u32(esp0 + 4, 8);
            if catch {
                eh_prolog3_catch(&mut e, &mut r);
            } else {
                eh_prolog3(&mut e, &mut r);
            }
            assert_eq!(r.ebp, ebp);
            assert_eq!(r.fs0, ebp - 0xc);
            assert_eq!(e.mem.u32(ebp), 0xe0);
            assert_eq!(e.mem.u32(ebp - 4), 0xffff_ffff);
            assert_eq!(e.mem.u32(ebp - 8), 0x00f3_4e90);
            assert_eq!(e.mem.u32(ebp - 0xc), 0x0e00_0f00);
            // Locals of 8 bytes, three registers, the /GS word.
            assert_eq!(r.esp, ebp - 0xc - 8 - 0xc - 4);
            assert_eq!(e.mem.u32(r.esp), 0x1234_5678 ^ ebp);
            if catch {
                assert_eq!(e.mem.u32(ebp - 0x10), r.esp);
            }
            r.push(&mut e, 0x0040_5555);
            eh_epilog3(&mut e, &mut r);
            assert_eq!(r.fs0, 0x0e00_0f00);
            assert_eq!((r.ebx, r.esi, r.edi, r.ebp), (0xb0, 0x51, 0xd1, 0xe0));
            assert_eq!(r.esp, ebp + 4);
            let mut words = r.words();
            words[4] = ebp - 0x40;
            words[5] = ebp;
            e.call(0x00ec_e2be, &words);
        }
    }

    #[test]
    fn the_8_byte_probe_rounds_the_size_and_goes_on_to_the_probe() {
        let mut e = engine();
        answer(&mut e, ALLOCA_PROBE, 0x77);
        assert_eq!(e.call(0x00ec_5ed6, &args![0x10u32, 0x1000u32]).u32(), 0x77);
        assert_eq!(alloca_probe_8(&mut e, 0xffff_ffff, 0x1000), 0x77);
        assert_eq!(alloca_probe_8(&mut e, 0x18, 0x1004), 0x77);
        // (0x1004 - 0x10) & 7 = 4; the carry gives all ones; (0x1008 - 0x18) & 7 = 0.
        assert_eq!(
            calls(&e, ALLOCA_PROBE),
            vec![
                vec![0x14, 0x1000],
                vec![0xffff_ffff, 0x1000],
                vec![0x18, 0x1004]
            ]
        );
    }

    #[test]
    fn the_non_local_goto_hooks_fill_the_block_and_call_the_funclet() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00ed_f5ed, &args![0x10u32, 0x20u32, 0x101u32])
                .u32(),
            0x10
        );
        assert_eq!(e.mem.u32(NLG_DATA + 4), 0x10);
        assert_eq!(e.mem.u32(NLG_DATA + 8), 0x101);
        assert_eq!(e.mem.u32(NLG_DATA + 0xc), 0x20);
        e.call(0x00ed_f5e4, &args![0x11u32, 2u32, 0x21u32, 9u32]);
        assert_eq!(e.mem.u32(NLG_DATA + 4), 0x11);
        assert_eq!(e.mem.u32(NLG_DATA + 8), 2);
        assert_eq!(e.mem.u32(NLG_DATA + 0xc), 0x21);
        answer(&mut e, 0x0090_0000, 0x55);
        assert_eq!(
            e.call(0x00ed_f60c, &args![0x0090_0000u32, 0x30u32]).u32(),
            0x55
        );
        assert_eq!(calls(&e, 0x0090_0000), vec![vec![0x30]]);
    }

    #[test]
    fn call_setting_frame_runs_the_funclet_on_the_node_s_frame() {
        let mut e = engine();
        answer(&mut e, 0x0090_0000, 0x0040_7777);
        assert_eq!(
            e.call(0x00ed_1550, &args![0x0090_0000u32, 0x2000u32, 0x100u32])
                .u32(),
            0x0040_7777
        );
        assert_eq!(calls(&e, 0x0090_0000), vec![vec![0x200c]]);
        assert_eq!(
            calls(&e, NLG_NOTIFY1),
            vec![
                vec![0x0090_0000, 0x100, 0x200c, 0x100],
                vec![0x0040_7777, 2, 0x200c, 2]
            ]
        );
        assert_eq!(e.mem.u32(NLG_DATA + 8), 2);
    }

    /// A scope table of three levels at `table` (entries of 12 bytes from `table + base`): level
    /// 2 a `__finally` (enclosing 1), level 1 an `__except` with a filter (enclosing 0), level 0
    /// a `__finally` (enclosing `outer`).
    fn scope_table(e: &mut Engine, table: u32, base: u32, outer: u32) {
        let rows = [
            (outer, 0, 0x0090_0000),
            (0, 0x0090_1001, 0x0090_1000),
            (1, 0, 0x0090_2000),
        ];
        for (i, (enclosing, filter, handler)) in rows.into_iter().enumerate() {
            let at = table + base + i as u32 * 12;
            e.mem.set_u32(at, enclosing);
            e.mem.set_u32(at + 4, filter);
            e.mem.set_u32(at + 8, handler);
        }
    }

    #[test]
    fn local_unwind2_runs_the_finally_blocks_down_to_the_stop_level() {
        let mut e = engine();
        for handler in [0x0090_0000, 0x0090_1000, 0x0090_2000] {
            answer(&mut e, handler, 0);
        }
        let node = 0x0e00_0100;
        let table = 0x0e00_0200;
        scope_table(&mut e, table, 0, 0xffff_ffff);
        e.mem.set_u32(node + 8, table);
        e.mem.set_u32(node + 0xc, 2);
        e.call(0x00ed_f53d, &args![0x0e00_0110u32, node, 0u32]);
        assert_eq!(e.mem.u32(node + 0xc), 0);
        assert_eq!(calls(&e, NLG_CALL), vec![vec![0x0090_2000, 0x0e00_0110]]);
        assert_eq!(
            calls(&e, NLG_NOTIFY),
            vec![vec![0x0090_2000, 0x0e00_0110, 0x101]]
        );
        // All the way (-1): level 0's block runs, level 1's (a filter) does not.
        local_unwind2(&mut e, 0x0e00_0110, node, 0xffff_ffff);
        assert_eq!(e.mem.u32(node + 0xc), 0xffff_ffff);
        assert_eq!(
            order(&e, &[0x0090_0000, 0x0090_1000, 0x0090_2000]),
            [0x0090_2000, 0x0090_0000]
        );
        // The longjmp form reads EBP, node and level from the buffer.
        let buffer = 0x0e00_0300;
        e.mem.set_u32(buffer, 0x0e00_0110);
        e.mem.set_u32(buffer + 0x18, node);
        e.mem.set_u32(buffer + 0x1c, 5);
        e.call(0x00ec_e202, &args![buffer]);
        assert_eq!(
            calls(&e, LOCAL_UNWIND2).last().unwrap(),
            &vec![0x0e00_0110, node, 5]
        );
    }

    #[test]
    fn local_unwind4_decodes_the_table_and_stops_at_minus_two() {
        let mut e = engine();
        for handler in [0x0090_0000, 0x0090_1000, 0x0090_2000] {
            answer(&mut e, handler, 0);
        }
        let node = 0x0e00_0100;
        let table = 0x0e00_0200;
        scope_table(&mut e, table, 0x10, 0xffff_fffe);
        e.mem.set_u32(node + 8, table ^ 0x1234_5678);
        e.mem.set_u32(node + 0xc, 2);
        e.call(
            0x00ed_863c,
            &args![0x0e00_0110u32, SECURITY_COOKIE, node, 0xffff_fffeu32],
        );
        assert_eq!(e.mem.u32(node + 0xc), 0xffff_fffe);
        assert_eq!(
            calls(&e, NLG_CALL),
            vec![
                vec![0x0090_2000, 0x0e00_0110],
                vec![0x0090_0000, 0x0e00_0110]
            ]
        );
        // _EH4_LocalUnwind: EBP from its first stack word, the cookie's address from the second.
        e.mem.set_u32(node + 0xc, 2);
        e.call(
            0x00ed_8778,
            &args![node, 1u32, 0x0e00_0110u32, SECURITY_COOKIE],
        );
        assert_eq!(e.mem.u32(node + 0xc), 1);
        assert_eq!(
            calls(&e, LOCAL_UNWIND4).last().unwrap(),
            &vec![0x0e00_0110, SECURITY_COOKIE, node, 1]
        );
        // The longjmp form.
        let buffer = 0x0e00_0300;
        e.mem.set_u32(buffer, 0x0e00_0110);
        e.mem.set_u32(buffer + 0x18, node);
        e.mem.set_u32(buffer + 0x1c, 0);
        e.mem.set_u32(buffer + 0x28, SECURITY_COOKIE);
        e.call(0x00ed_8712, &args![buffer]);
        assert_eq!(
            calls(&e, LOCAL_UNWIND4).last().unwrap(),
            &vec![0x0e00_0110, SECURITY_COOKIE, node, 0]
        );
    }

    #[test]
    fn the_unwind_handlers_go_on_with_an_interrupted_unwind() {
        let mut e = engine();
        e.register(SECURITY_CHECK_COOKIE, |_, _| Ret::default());
        e.register(LOCAL_UNWIND4, |_, _| Ret::default());
        e.register(LOCAL_UNWIND2, |_, _| Ret::default());
        let record = 0x0e00_0000;
        let dispatcher = 0x0e00_0010;
        let node = 0x0e00_0100;
        // Outside an unwind: continue the search.
        e.mem.set_u32(record + 4, 1);
        assert_eq!(
            e.call(0x00ed_86cc, &args![record, node, 0u32, dispatcher])
                .u32(),
            1
        );
        assert_eq!(
            e.call(0x00ed_f4f8, &args![record, node, 0u32, dispatcher, node])
                .u32(),
            1
        );
        e.mem.set_u32(record + 4, 2);
        for (i, w) in [0x11u32, 0x22, 0x33, 0x44, 0x55].into_iter().enumerate() {
            e.mem.set_u32(node + 8 + 4 * i as u32, w);
        }
        assert_eq!(
            e.call(0x00ed_86cc, &args![record, node, 0u32, dispatcher])
                .u32(),
            3
        );
        assert_eq!(calls(&e, SECURITY_CHECK_COOKIE), vec![vec![0x11 ^ node]]);
        assert_eq!(calls(&e, LOCAL_UNWIND4), vec![vec![0x55, 0x44, 0x33, 0x22]]);
        assert_eq!(e.mem.u32(dispatcher), node);
        // __local_unwind2's: the interrupted node is the word above the four arguments.
        let other = 0x0e00_0200;
        e.mem.set_u32(other - 4, 0x99);
        e.mem.set_u32(other + 0x10, 0xeb);
        e.mem.set_u32(other + 0x24, 0x4e);
        e.mem.set_u32(other + 0x28, 3);
        e.mem.set_u32(dispatcher, 0);
        assert_eq!(
            e.call(0x00ed_f4f8, &args![record, node, 0u32, dispatcher, other])
                .u32(),
            3
        );
        assert_eq!(calls(&e, SECURITY_CHECK_COOKIE)[1], vec![0x99 ^ other]);
        assert_eq!(calls(&e, LOCAL_UNWIND2), vec![vec![0xeb, 0x4e, 3]]);
        assert_eq!(e.mem.u32(dispatcher), node);
    }

    #[test]
    fn the_eh4_helpers_call_with_the_frame() {
        let mut e = engine();
        answer(&mut e, 0x0090_0000, 0xffff_ffff);
        answer(&mut e, 0x0090_1000, 0x66);
        e.register(RTL_UNWIND, |_, _| Ret::default());
        assert_eq!(
            e.call(0x00ed_872e, &args![0x0090_0000u32, 0x0e00_0110u32])
                .i32(),
            -1
        );
        assert_eq!(
            e.call(0x00ed_8745, &args![0x0090_1000u32, 0x0e00_0110u32])
                .u32(),
            0x66
        );
        assert_eq!(
            calls(&e, NLG_NOTIFY),
            vec![vec![0x0090_1000, 0x0e00_0110, 1]]
        );
        assert_eq!(calls(&e, 0x0090_1000), vec![vec![0x0e00_0110]]);
        e.call(0x00ed_875e, &args![0x0e00_0100u32]);
        e.call(0x00ed_f4d8, &args![0x0e00_0200u32]);
        assert_eq!(
            calls(&e, RTL_UNWIND),
            vec![
                vec![0x0e00_0100, EH4_GLOBAL_UNWIND_TARGET, 0, 0],
                vec![0x0e00_0200, GLOBAL_UNWIND2_TARGET, 0, 0]
            ]
        );
    }

    /// `__except_handler4`'s set-up: a node at `0x0e00_0100` (frame `0x0e00_0110`) with the
    /// scope table at `0x0e00_0200` (no /GS cookie, the EH cookie at frame - 0x20 matching),
    /// levels as [`scope_table`] with -2 outermost, at level `level`; the filter answers
    /// `filter`.
    fn handler4_engine(level: u32, filter: i32) -> (Engine, u32, u32) {
        let mut e = engine();
        let node = 0x0e00_0100;
        let frame = node + 0x10;
        let table = 0x0e00_0200;
        e.mem.set_u32(table, 0xffff_fffe);
        e.mem.set_u32(table + 8, 0xffff_ffe0);
        e.mem.set_u32(table + 0xc, 0);
        scope_table(&mut e, table, 0x10, 0xffff_fffe);
        e.mem.set_u32(frame - 0x20, 0);
        e.mem.set_u32(node + 8, table ^ 0x1234_5678);
        e.mem.set_u32(node + 0xc, level);
        e.register(SECURITY_CHECK_COOKIE, |_, _| Ret::default());
        answer(&mut e, EH4_CALL_FILTER_FUNC, filter as u32);
        e.register(EH4_LOCAL_UNWIND, |_, _| Ret::default());
        e.register(EH4_GLOBAL_UNWIND, |_, _| Ret::default());
        e.mem.set_u32(DESTRUCT_POINTER, 0);
        (e, node, frame)
    }

    #[test]
    fn except_handler4_walks_the_levels_and_checks_the_cookies() {
        let record = 0x0e00_0000;
        // No filter accepts (the filter answers 0): continue the search, the EH cookie checked
        // (frame + 0 XOR [frame - 0x20]) before and after.
        let (mut e, node, frame) = handler4_engine(2, 0);
        e.mem.set_u32(record + 4, 0);
        assert_eq!(
            e.call(0x00ec_c8e0, &args![record, node, 0x0e00_0020u32, 0u32])
                .u32(),
            1
        );
        assert_eq!(
            calls(&e, EH4_CALL_FILTER_FUNC),
            vec![vec![0x0090_1001, frame]]
        );
        assert_eq!(
            calls(&e, SECURITY_CHECK_COOKIE),
            vec![vec![frame], vec![frame]]
        );
        assert_ne!(e.mem.u32(node - 4), 0);
        // A negative filter answer: continue execution.
        let (mut e, node, _) = handler4_engine(1, -1);
        assert_eq!(except_handler4(&mut e, record, node, 0, 0), 0);
        // Unwinding: the node goes to -2, the cookies are checked again; at -2 nothing more.
        let (mut e, node, frame) = handler4_engine(2, 0);
        e.mem.set_u32(record + 4, 2);
        assert_eq!(except_handler4(&mut e, record, node, 0, 0), 1);
        assert_eq!(
            calls(&e, EH4_LOCAL_UNWIND),
            vec![vec![node, 0xffff_fffe, frame, SECURITY_COOKIE]]
        );
        assert_eq!(calls(&e, SECURITY_CHECK_COOKIE).len(), 2);
        let (mut e, node, _) = handler4_engine(0xffff_fffe, 0);
        e.mem.set_u32(record + 4, 2);
        assert_eq!(except_handler4(&mut e, record, node, 0, 0), 1);
        assert!(calls(&e, EH4_LOCAL_UNWIND).is_empty());
        assert_eq!(calls(&e, SECURITY_CHECK_COOKIE).len(), 1);
    }

    #[test]
    #[should_panic(expected = "_EH4_TransferToHandler returned")]
    fn except_handler4_unwinds_and_transfers_to_the_accepting_level() {
        let record = 0x0e00_0000;
        let (mut e, node, frame) = handler4_engine(2, 1);
        e.mem.set_u32(record, CXX_EXCEPTION);
        e.mem.set_u32(record + 4, 0);
        e.mem.set_u32(DESTRUCT_POINTER, 0x00ec_e4f0);
        answer(&mut e, IS_NONWRITABLE_IN_CURRENT_IMAGE, 1);
        e.register(0x00ec_e4f0, |_, _| Ret::default());
        e.register_double(EH4_TRANSFER_TO_HANDLER, move |e, a| {
            assert_eq!(a, [0x0090_1000, frame]);
            assert_eq!(calls(e, 0x00ec_e4f0), vec![vec![record, 1]]);
            assert_eq!(calls(e, EH4_GLOBAL_UNWIND), vec![vec![node]]);
            assert_eq!(
                calls(e, EH4_LOCAL_UNWIND),
                vec![vec![node, 1, frame, SECURITY_COOKIE]]
            );
            // The node now stands at the accepting level's enclosing one.
            assert_eq!(e.mem.u32(node + 0xc), 0);
            Ret::default()
        });
        except_handler4(&mut e, record, node, 0, 0);
    }

    /// `__except_handler3`'s set-up: a node at `0x0e00_0100` with the scope table at
    /// `0x0e00_0200` as [`scope_table`] with -1 outermost, at level `level`; `__ValidateEH3RN`
    /// answers `valid` and the filter `filter`.
    fn handler3_engine(level: u32, valid: i32, filter: i32) -> (Engine, u32) {
        let mut e = engine();
        let node = 0x0e00_0100;
        scope_table(&mut e, 0x0e00_0200, 0, 0xffff_ffff);
        e.mem.set_u32(node + 8, 0x0e00_0200);
        e.mem.set_u32(node + 0xc, level);
        answer(&mut e, VALIDATE_EH3RN, valid as u32);
        answer(&mut e, 0x0090_1001, filter as u32);
        e.register(LOCAL_UNWIND2, |_, _| Ret::default());
        e.register(GLOBAL_UNWIND2, |_, _| Ret::default());
        e.register(CALL_DESTRUCT_EXCEPTION_OBJECT, |_, _| Ret::default());
        (e, node)
    }

    #[test]
    fn except_handler3_validates_and_walks_the_levels() {
        let record = 0x0e00_0000;
        let (mut e, node) = handler3_engine(2, 1, 0);
        assert_eq!(
            e.call(0x00ec_e104, &args![record, node, 0u32, 0u32]).u32(),
            1
        );
        assert_eq!(calls(&e, 0x0090_1001), vec![vec![node + 0x10]]);
        // An invalid node: the stack-invalid flag.
        let (mut e, node) = handler3_engine(2, 0, 0);
        e.mem.set_u32(record + 4, 1);
        assert_eq!(except_handler3(&mut e, record, node, 0, 0), 1);
        assert_eq!(e.mem.u32(record + 4), 9);
        // A negative filter answer.
        let (mut e, node) = handler3_engine(1, 1, -1);
        e.mem.set_u32(record + 4, 0);
        assert_eq!(except_handler3(&mut e, record, node, 0, 0), 0);
        // Unwinding: everything down to -1 on the node's frame.
        let (mut e, node) = handler3_engine(2, 1, 0);
        e.mem.set_u32(record + 4, 2);
        assert_eq!(except_handler3(&mut e, record, node, 0, 0), 1);
        assert_eq!(
            calls(&e, LOCAL_UNWIND2),
            vec![vec![node + 0x10, node, 0xffff_ffff]]
        );
    }

    #[test]
    #[should_panic(expected = "the __except block returned")]
    fn except_handler3_runs_the_accepting_level_s_block() {
        let record = 0x0e00_0000;
        let (mut e, node) = handler3_engine(2, 1, 1);
        e.register_double(0x0090_1000, move |e, a| {
            assert_eq!(a, [node + 0x10]);
            assert_eq!(
                calls(e, CALL_DESTRUCT_EXCEPTION_OBJECT),
                vec![vec![record, 1]]
            );
            assert_eq!(calls(e, GLOBAL_UNWIND2), vec![vec![node]]);
            assert_eq!(calls(e, LOCAL_UNWIND2), vec![vec![node + 0x10, node, 1]]);
            assert_eq!(
                calls(e, NLG_NOTIFY),
                vec![vec![0x0090_1000, node + 0x10, 1]]
            );
            assert_eq!(e.mem.u32(node + 0xc), 0);
            Ret::default()
        });
        e.mem.set_u32(record + 4, 0);
        except_handler3(&mut e, record, node, 0, 0);
    }

    /// A section header at `at` (RVA `start`, size `size`, the characteristics).
    fn section(e: &mut Engine, at: u32, start: u32, size: u32, flags: u32) {
        e.mem.set_u32(at + 8, size);
        e.mem.set_u32(at + 0xc, start);
        e.mem.set_u32(at + 0x24, flags);
    }

    #[test]
    fn scope_table_handlers_must_lie_in_executable_sections() {
        let mut e = engine();
        let image = 0x0040_0000;
        let text = 0x0e00_0400;
        let data = 0x0e00_0500;
        section(&mut e, text, 0x1000, 0x5000, 0x6000_0020);
        section(&mut e, data, 0x6000, 0x1000, 0xc000_0040);
        e.register_double(FIND_PE_SECTION, move |_, a| {
            let rva = a[1];
            if (0x1000..0x6000).contains(&rva) {
                text.into_ret()
            } else if (0x6000..0x7000).contains(&rva) {
                data.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let table = 0x0e00_0200;
        let rows = [
            (0xffff_ffffu32, 0u32, image + 0x1100),
            (0, image + 0x1200, image + 0x2300),
        ];
        for (i, (enclosing, filter, handler)) in rows.into_iter().enumerate() {
            let at = table + i as u32 * 12;
            e.mem.set_u32(at, enclosing);
            e.mem.set_u32(at + 4, filter);
            e.mem.set_u32(at + 8, handler);
        }
        assert_eq!(e.call(0x00ed_f610, &args![1u32, image, table]).u32(), 1);
        // Pages 0x2000 (looked up), 0x1000 (in the section found), 0x1000 again (the last).
        assert_eq!(calls(&e, FIND_PE_SECTION), vec![vec![image, 0x2000]]);
        // A handler in a data section fails.
        e.mem.set_u32(table + 8, image + 0x6100);
        assert_eq!(validate_scope_table_handlers(&mut e, 0, image, table), 0);
        // No levels: valid.
        assert_eq!(
            validate_scope_table_handlers(&mut e, 0xffff_ffff, image, table),
            1
        );
    }

    /// `__ValidateEH3RN`'s set-up: a node at `0x0e00_0100` (handler `00ecc8e0`) with a
    /// two-level table at `0x0e00_0200` (level 1 has a filter) and the saved stack pointer
    /// below the node.
    fn eh3_engine() -> (Engine, u32) {
        let mut e = engine();
        let node = 0x0e00_0100;
        let table = 0x0e00_0200;
        e.mem.set_u32(table, 0xffff_ffff);
        e.mem.set_u32(table + 4, 0);
        e.mem.set_u32(table + 12, 0);
        e.mem.set_u32(table + 16, 0x0090_0001);
        e.mem.set_u32(node - 8, 0x0e00_0080);
        e.mem.set_u32(node + 4, EXCEPT_HANDLER4);
        e.mem.set_u32(node + 8, table);
        e.mem.set_u32(node + 0xc, 1);
        answer(&mut e, VALIDATE_IMAGE_BASE, 1);
        answer(&mut e, VALIDATE_SCOPE_TABLE_HANDLERS, 1);
        answer(&mut e, FIND_PE_SECTION, 0x0e00_0400);
        (e, node)
    }

    /// A thread stack from `0x0d00_0000` up to `0x0e00_0100` (limit, base): the test's nodes
    /// at `0x0e00_0100` sit just above it, their scope tables outside it.
    const STACK_LIMIT: u32 = 0x0d00_0000;
    const STACK_BASE: u32 = 0x0e00_0100;

    fn validate(e: &mut Engine, node: u32) -> u32 {
        validate_eh3rn_with_stack(e, node, STACK_LIMIT, STACK_BASE)
    }

    #[test]
    #[should_panic(expected = "the thread's stack range")]
    fn validate_eh3rn_stops_where_it_needs_the_thread_s_stack() {
        let (mut e, node) = eh3_engine();
        e.call(0x00ed_f6d0, &args![node]);
    }

    #[test]
    fn validate_eh3rn_rejects_bad_tables() {
        let (mut e, node) = eh3_engine();
        e.mem.set_u32(node + 8, 0x0e00_0202);
        assert_eq!(e.call(0x00ed_f6d0, &args![node]).u32(), 0);
        // A scope table on the thread's stack.
        let (mut e, node) = eh3_engine();
        assert_eq!(
            validate_eh3rn_with_stack(&mut e, node, 0x0e00_0000, 0x0e00_1000),
            0
        );
        assert!(calls(&e, VALIDATE_IMAGE_BASE).is_empty());
        // A filter and a saved stack pointer below the stack's limit.
        let (mut e, node) = eh3_engine();
        assert_eq!(
            validate_eh3rn_with_stack(&mut e, node, 0x0e00_0090, 0x0e00_0100),
            0
        );
        // An enclosing level not below its own.
        let (mut e, node) = eh3_engine();
        e.mem.set_u32(0x0e00_0200 + 12, 1);
        assert_eq!(validate(&mut e, node), 0);
        // A filter and a saved stack pointer at or above the node.
        let (mut e, node) = eh3_engine();
        e.mem.set_u32(node - 8, node);
        assert_eq!(validate(&mut e, node), 0);
        // No try level: valid without looking further.
        let (mut e, node) = eh3_engine();
        e.mem.set_u32(node + 0xc, 0xffff_ffff);
        assert_eq!(validate(&mut e, node), 1);
        assert!(calls(&e, VALIDATE_IMAGE_BASE).is_empty());
    }

    #[test]
    fn validate_eh3rn_queries_the_region_and_caches_the_page() {
        let (mut e, node) = eh3_engine();
        e.register_double(IMPORT_VIRTUAL_QUERY, |e, a| {
            e.mem.set_u32(a[1] + 4, 0x0040_0000);
            e.mem.set_u32(a[1] + 0x14, 2);
            e.mem.set_u32(a[1] + 0x18, MEM_IMAGE);
            0x1cu32.into_ret()
        });
        e.mem.set_u32(EH3_CACHE_COUNT, 1);
        e.mem.set_u32(EH3_CACHE, 0x0e00_5000);
        e.mem.set_u32(EH3_CACHE + 4, 0x0050_0000);
        assert_eq!(validate(&mut e, node), 1);
        assert_eq!(
            calls(&e, VALIDATE_SCOPE_TABLE_HANDLERS),
            vec![vec![1, 0x0040_0000, 0x0e00_0200]]
        );
        assert_eq!(
            calls(&e, FIND_PE_SECTION),
            vec![vec![0x0040_0000, EXCEPT_HANDLER4 - 0x0040_0000]]
        );
        // Inserted at the front, the count grown, the lock given back.
        assert_eq!(e.mem.u32(EH3_CACHE_COUNT), 2);
        assert_eq!(e.mem.u32(EH3_CACHE), 0x0e00_0000);
        assert_eq!(e.mem.u32(EH3_CACHE + 4), 0x0040_0000);
        assert_eq!(e.mem.u32(EH3_CACHE + 8), 0x0e00_5000);
        assert_eq!(e.mem.u32(EH3_CACHE_LOCK), 0);
        // Now cached at slot 0: no query, nothing moves.
        e.call_log = Some(vec![]);
        assert_eq!(validate(&mut e, node), 1);
        assert!(calls(&e, IMPORT_VIRTUAL_QUERY).is_empty());
        // Cached at slot 1: moves to the front.
        e.mem.set_u32(EH3_CACHE, 0x0e00_5000);
        e.mem.set_u32(EH3_CACHE + 4, 0x0050_0000);
        e.mem.set_u32(EH3_CACHE + 8, 0x0e00_0000);
        e.mem.set_u32(EH3_CACHE + 12, 0x0040_0000);
        assert_eq!(validate(&mut e, node), 1);
        assert_eq!(e.mem.u32(EH3_CACHE), 0x0e00_0000);
        assert_eq!(e.mem.u32(EH3_CACHE + 8), 0x0e00_5000);
        assert_eq!(e.mem.u32(EH3_CACHE_LOCK), 0);
        // Not an image: -1. No region: 1.
        e.mem.set_u32(EH3_CACHE_COUNT, 0);
        e.register_double(IMPORT_VIRTUAL_QUERY, |e, a| {
            e.mem.set_u32(a[1] + 0x18, 0x20000);
            0x1cu32.into_ret()
        });
        assert_eq!(validate(&mut e, node), 0xffff_ffff);
        answer(&mut e, IMPORT_VIRTUAL_QUERY, 0);
        assert_eq!(validate(&mut e, node), 1);
        // The filter: an access violation is handled.
        let frame = 0x0e00_0700;
        e.mem.set_u32(frame - 0x14, 0x0e00_0600);
        e.mem.set_u32(0x0e00_0600, 0x0e00_0610);
        e.mem.set_u32(0x0e00_0610, ACCESS_VIOLATION);
        assert_eq!(e.call(0x00ed_f8aa, &args![frame]).u32(), 1);
        e.mem.set_u32(0x0e00_0610, NO_MEMORY);
        assert_eq!(validate_eh3rn_filter(&mut e, frame), 0);
    }

    /// A guarded function's frame at `STACK` as `__SEH_prolog4` leaves it (saved `EBP` 0xe0,
    /// `EBX`/`ESI`/`EDI` 0xb0/0x51/0xd1, chain head 0x0e00_0f00), its locals 0x20 bytes, and
    /// `[EBP-0x18]` the stack pointer after the `/GS` word.
    fn guarded_frame(e: &mut Engine) -> u32 {
        let ebp = STACK;
        e.mem.set_u32(ebp, 0xe0);
        e.mem.set_u32(ebp + 4, 0x0040_1234);
        e.mem.set_u32(ebp - 0x10, 0x0e00_0f00);
        let esp = ebp - 0x10 - 0x20 - 0x10;
        e.mem.set_u32(esp, 0x1234_5678 ^ ebp);
        e.mem.set_u32(esp + 4, 0xd1);
        e.mem.set_u32(esp + 8, 0x51);
        e.mem.set_u32(esp + 0xc, 0xb0);
        e.mem.set_u32(ebp - 0x18, esp);
        ebp
    }

    #[test]
    fn the_except_blocks_set_their_results_and_leave() {
        let mut e = engine();
        let ebp = guarded_frame(&mut e);
        e.mem.set_u32(ebp - 4, 0);
        e.mem.set_u32(ebp - 0x1c, 1);
        assert_eq!(e.call(0x00ed_1910, &args![ebp]).u32(), 0);
        assert_eq!(e.mem.u32(ebp - 0x1c), 0);
        assert_eq!(e.mem.u32(ebp - 4), 0xffff_fffe);
        // The epilog's pushed return address lands over the saved EBP's slot.
        assert_eq!(e.mem.u32(ebp), 0x00ed_1926);
        let ebp = guarded_frame(&mut e);
        assert_eq!(e.call(0x00ec_c4c6, &args![ebp]).u32(), 0xff);
        let ebp = guarded_frame(&mut e);
        assert_eq!(e.call(0x00ec_5fbc, &args![ebp]).u32(), 0);
        assert_eq!(e.mem.u32(ebp), 0x00ec_5fcb);
        // __crtInitCritSecAndSpinCount's: out of memory sets the last error.
        e.register(IMPORT_SET_LAST_ERROR, |_, _| Ret::default());
        let ebp = guarded_frame(&mut e);
        e.mem.set_u32(ebp - 0x20, NO_MEMORY);
        e.mem.set_u32(ebp - 0x1c, 1);
        assert_eq!(e.call(0x00ed_e1d4, &args![ebp]).u32(), 0);
        assert_eq!(calls(&e, IMPORT_SET_LAST_ERROR), vec![vec![8]]);
        let ebp = guarded_frame(&mut e);
        e.mem.set_u32(ebp - 0x20, ACCESS_VIOLATION);
        init_crit_sec_except(&mut e, ebp);
        assert_eq!(calls(&e, IMPORT_SET_LAST_ERROR).len(), 1);
        // ___set_fpsr_sse2's: the denormals-are-zero bit dropped.
        let ebp = guarded_frame(&mut e);
        e.mem.set_u32(DAZ_SUPPORTED, 1);
        e.mem.set_u32(ebp + 8, MXCSR | 0x40);
        assert_eq!(e.call(0x00ed_5b85, &args![ebp]).u32(), 0);
        assert_eq!(e.mem.u32(DAZ_SUPPORTED), 0);
        assert_eq!(e.mem.u32(ebp + 8), MXCSR);
    }

    #[test]
    fn the_start_up_s_except_block_exits_with_the_code() {
        let mut e = engine();
        e.register(EXIT, |_, _| Ret::default());
        e.register(C_EXIT, |_, _| Ret::default());
        let ebp = guarded_frame(&mut e);
        e.mem.set_u32(ebp - 0x24, 0xc000_0094);
        e.mem.set_u32(ebp - 0x1c, 0);
        assert_eq!(e.call(0x00ec_c49c, &args![ebp]).u32(), 0xc000_0094);
        assert_eq!(e.mem.u32(ebp - 0x20), 0xc000_0094);
        assert_eq!(calls(&e, EXIT), vec![vec![0xc000_0094]]);
        assert_eq!(calls(&e, C_EXIT).len(), 1);
        // A managed image skips __exit.
        let ebp = guarded_frame(&mut e);
        e.mem.set_u32(ebp - 0x1c, 1);
        tmain_crt_startup_except(&mut e, ebp);
        assert_eq!(calls(&e, EXIT).len(), 1);
        assert_eq!(calls(&e, C_EXIT).len(), 2);
    }

    #[test]
    #[should_panic(expected = "00ece53c: terminate returned")]
    fn destruct_exception_object_s_block_terminates() {
        let mut e = engine();
        e.register(TERMINATE, |_, _| Ret::default());
        e.call(0x00ec_e53c, &args![0u32]);
    }

    #[test]
    #[should_panic(expected = "00ece9e9: terminate returned")]
    fn build_catch_object_s_block_terminates() {
        let mut e = engine();
        e.register(TERMINATE, |_, _| Ret::default());
        e.call(0x00ec_e9e9, &args![0u32]);
    }

    #[test]
    #[should_panic(expected = "_CxxThrowException returned")]
    fn the_dynamic_cast_block_throws_a_non_rtti_object() {
        let mut e = engine();
        let ebp = guarded_frame(&mut e);
        e.mem.set_u32(ebp - 0x1c, 5);
        e.register(NON_RTTI_OBJECT, |_, _| Ret::default());
        e.register_double(CXX_THROW_EXCEPTION, move |e, a| {
            assert_eq!(a, [ebp - 0x28, NON_RTTI_THROW_INFO]);
            assert_eq!(
                calls(e, NON_RTTI_OBJECT),
                vec![vec![ebp - 0x28, NON_RTTI_TEXT]]
            );
            assert_eq!(e.mem.u32(ebp - 0x1c), 0);
            Ret::default()
        });
        e.call(0x00ec_44b5, &args![ebp]);
    }
}
