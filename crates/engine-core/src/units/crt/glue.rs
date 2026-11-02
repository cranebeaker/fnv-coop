//! Small pieces of the C runtime (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe
//! 1.4.0.525) that the recorded game ran, translated from the exe: the start-up and exit
//! glue (`__cinit` `00eccd14` with `_initterm` `00ecccd3` and `_initterm_e` `00ecccf0`, the
//! first C initializer `00ec6522`, the C++ terminator runner `00eddb44`, the stdio terminator
//! `00ecdffb`, `_exit` `00eccec5`, `__crtCorExitProcess` `00eccc7e`), the image checks
//! (`_IsNonwritableInCurrentImage` `00eddcc0`, `_ValidateImageBase` `00eddc30`,
//! `_FindPESection` `00eddc70`), `__realloc_crt` (`00ed0d70`), the locale accessors
//! `___pctype_func` (`00ed08c8`) and `___lc_handle_func` (`00ed0cb9`), `srand` (`00ecada6`), `rand` (`00ecadb8`),
//! `_finite` (`00ec7595`), `_isnan` (`00ec75b1`), `_CIfmod` (`00ec913a`), `__strupr_s`
//! (`00ecbc5b`), `___ascii_strnicmp` (`00ed5bc0`), the 16-byte aligned stack probe
//! (`00ec5ec0`), the `eh vector copy constructor iterator` (`00ecaebb`) with its `__finally`
//! block (`00ecaf0c`), the word cleared before `_invoke_watson` (`00ed5bae`), and the
//! `__finally` blocks that give a lock back (`00ec9886`, `00ec9a3d`, `00ecb0fe`, `00ecceb0`,
//! `00ecf7ac`, `00ed1312`, `00ed131e`, `00ed3d7a`, `00ed81fd`, `00ed8630`, `00edc815`,
//! `00ee213b`, `00ee38da`), and `_setlocale` (`00ed036e`) with its last `__finally` block
//! (`00ed04d9`).
//!
//! A `__finally` block runs on its function's frame; the frame words it reads are its
//! arguments here, in the order it reads them. Every call is made by address, as the exe makes
//! it, so a test can double each one.

use super::*;

/// `_errno` (`00ec85e3`) and the invalid-parameter report (`00ec7c30`, five words), cdecl.
const ERRNO_ADDR: u32 = 0x00ec_85e3;
const INVALID_PARAMETER_ADDR: u32 = 0x00ec_7c30;
/// `_getptd` (`00ed11de`) and `___updatetlocinfo` (`00ecf742`), cdecl, no arguments.
const GETPTD_ADDR: u32 = 0x00ed_11de;
const UPDATETLOCINFO_ADDR: u32 = 0x00ec_f742;
/// `__ptlocinfo` (`011c23f8`, the global locale) and `__globallocalestatus` (`011c2310`).
const PTLOCINFO: u32 = 0x011c_23f8;
const GLOBAL_LOCALE_STATUS: u32 = 0x011c_2310;
/// `_unlock` (`00edde5f`), `__unlock_file` (`00ece08e`) and `_unlock_fhandle` (`00ee2f5c`),
/// cdecl, one word.
const UNLOCK_ADDR: u32 = 0x00ed_de5f;
const UNLOCK_FILE_ADDR: u32 = 0x00ec_e08e;
const UNLOCK_FHANDLE_ADDR: u32 = 0x00ee_2f5c;
/// `__ioinfo` table of pointers to the handle blocks (`01270960`; 32 entries of 0x40 bytes a
/// block).
const PIOINFO: u32 = 0x0127_0960;
/// `__alloca_probe` (`00ec8660`; `EAX` the size, then `ESP` at entry).
const ALLOCA_PROBE_ADDR: u32 = 0x00ec_8660;
/// `__calloc_crt` (`00ed0d24`), `__encode_pointer` (`00ed0f17`), `realloc` (`00eccf5d`),
/// `free` (`00ecd291`), `_flushall`'s body `flsall` called with 1 (`00ecdac3`), `_fcloseall`
/// (`00edf400`), `_atexit` (`00ec658f`), `doexit` (`00eccd99`; code, quick, return-to-caller),
/// `__initp_misc_cfltcvt_tab` (`00ed248e`), `__strupr_s_l` (`00ecbc26`), `__cintrindisp2`
/// (`00ed6070`; `EDX` the descriptor, then `ST1`, `ST0`) and `__ArrayUnwind` (`00ec5f70`,
/// stdcall: array, element size, count, destructor).
const CALLOC_CRT_ADDR: u32 = 0x00ed_0d24;
const ENCODE_POINTER_ADDR: u32 = 0x00ed_0f17;
const REALLOC_ADDR: u32 = 0x00ec_cf5d;
const FREE_ADDR: u32 = 0x00ec_d291;
const FLUSHALL_ADDR: u32 = 0x00ec_dac3;
const FCLOSEALL_ADDR: u32 = 0x00ed_f400;
const ATEXIT_ADDR: u32 = 0x00ec_658f;
const DOEXIT_ADDR: u32 = 0x00ec_cd99;
const INITP_MISC_CFLTCVT_TAB_ADDR: u32 = 0x00ed_248e;
const STRUPR_S_L_ADDR: u32 = 0x00ec_bc26;
const CINTRINDISP2_ADDR: u32 = 0x00ed_6070;
const ARRAY_UNWIND_ADDR: u32 = 0x00ec_5f70;
/// The functions of this file that others here call by address.
const INITTERM_ADDR: u32 = 0x00ec_ccd3;
const INITTERM_E_ADDR: u32 = 0x00ec_ccf0;
const IS_NONWRITABLE_ADDR: u32 = 0x00ed_dcc0;
const VALIDATE_IMAGE_BASE_ADDR: u32 = 0x00ed_dc30;
const FIND_PE_SECTION_ADDR: u32 = 0x00ed_dc70;
const CPP_TERMINATORS_ADDR: u32 = 0x00ed_db44;
const COPY_ITERATOR_FINALLY_ADDR: u32 = 0x00ec_af0c;
/// KERNEL32 import slots: `GetModuleHandleW`, `GetProcAddress` and `Sleep`.
const IMPORT_GET_MODULE_HANDLE_W: u32 = 0x00fd_f198;
const IMPORT_GET_PROC_ADDRESS: u32 = 0x00fd_f0ac;
const IMPORT_SLEEP: u32 = 0x00fd_f104;
/// `L"mscoree.dll"` (`010f2048`) and `"CorExitProcess"` (`010f2038`).
const TEXT_MSCOREE: u32 = 0x010f_2048;
const TEXT_COR_EXIT_PROCESS: u32 = 0x010f_2038;
/// `fmod`'s descriptor (`011c18e0`, the one `fmod` `00ec9130` passes too).
const FMOD_DESCRIPTOR: u32 = 0x011c_18e0;
/// The image base (`00400000`).
const IMAGE_BASE: u32 = 0x0040_0000;
/// `__cinit`'s tables and words: the floating-point initializer pointer (`010f1f1c`), the C
/// initializers (`010113c4..010113e8`), the C++ ones (`00fdf4a4..010113c0`), the TLS
/// callback pointer (`01271a94`) and the C++ terminators `00eddb44` runs (`0113ec3c..0113ec3c`,
/// empty).
const FPINIT_POINTER: u32 = 0x010f_1f1c;
const C_INIT: (u32, u32) = (0x0101_13c4, 0x0101_13e8);
const CPP_INIT: (u32, u32) = (0x00fd_f4a4, 0x0101_13c0);
const TLS_CALLBACK: u32 = 0x0127_1a94;
const CPP_TERMINATORS: (u32, u32) = (0x0113_ec3c, 0x0113_ec3c);
/// The exit table's start and end words (`01271a8c`, `01271a88`; the CRT's `__onexitbegin`
/// and `__onexitend`, names not confirmed in Ghidra).
const ONEXIT_BEGIN: u32 = 0x0127_1a8c;
const ONEXIT_END: u32 = 0x0127_1a88;
/// The byte `00ecdffb` tests before `_fcloseall` (`0126fde0`) and the stream table it frees
/// (`01270a70`; the CRT's `__piob`).
const EXIT_BYTE: u32 = 0x0126_fde0;
const PIOB: u32 = 0x0127_0a70;
/// `__realloc_crt`'s wait limit in milliseconds (`0126fdfc`; the CRT's `__crtMaxWait`).
const MAX_WAIT: u32 = 0x0126_fdfc;
/// The word `00ed5bae` clears (`01270a60`).
const WATSON_WORD: u32 = 0x0127_0a60;

// Translated from 00ec5ec0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_RtlCheckStack12` (Ghidra's label; the 16-byte aligned `alloca` probe): `EAX` the size,
/// then `ESP` at entry (it points at the return address). The size grows by the distance
/// from the caller's stack top (`esp + 4`) minus the size to the next lower 16-byte boundary
/// (`(esp + 4 - size) & 0xf`), all ones when that carries, and goes on to `__alloca_probe` with
/// the same `ESP` (a jump), whose answer (the caller's new `ESP`) it gives.
pub fn alloca_probe_16(e: &mut Engine, size: u32, esp: u32) -> u32 {
    let low = esp.wrapping_add(4).wrapping_sub(size) & 0xf;
    let (sum, carry) = size.overflowing_add(low);
    let size = if carry { 0xffff_ffff } else { sum };
    e.call(ALLOCA_PROBE_ADDR, &args![size, esp]).u32()
}

/// How the translations model a caller's `_alloca16`: the `ESP` the caller continues with
/// after `CALL 00ec5ec0` with `EAX = size` when its own `ESP` was `esp` before the call, i.e.
/// [`alloca_probe_16`] and `__alloca_probe` (`00ec8660`) composed as plain arithmetic:
/// `(esp - size) & !0xf`, with the probes' carry (all ones) and borrow (address 0) cases. The
/// block lies from the answer up to `esp`, inside the caller's exe-shaped `with_stack` frame;
/// the probes' page reads and the return address they leave under the new top are not modelled
/// (nothing reads them).
pub fn alloca16_esp(esp: u32, size: u32) -> u32 {
    let low = esp.wrapping_sub(size) & 0xf;
    let (sum, carry) = size.overflowing_add(low);
    let size = if carry { 0xffff_ffff } else { sum };
    esp.wrapping_sub(4).saturating_sub(size).wrapping_add(4)
}

// Translated from 00ec6522 (disassembled, FalloutNV.exe 1.4.0.525)
/// The first C initializer (unnamed; the CRT's `__onexitinit`): the exit table is made from
/// `__calloc_crt(32, 4)`, its `__encode_pointer`'d address stored as both its start and its
/// end; no block answers 24 (`_RT_SPACEINIT`), else its first word is cleared and 0 answered.
pub fn onexit_init(e: &mut Engine) -> u32 {
    let block = e.call(CALLOC_CRT_ADDR, &args![32u32, 4u32]).u32();
    let encoded = e.call(ENCODE_POINTER_ADDR, &args![block]).u32();
    e.mem.set_u32(ONEXIT_BEGIN, encoded);
    e.mem.set_u32(ONEXIT_END, encoded);
    if block == 0 {
        return 0x18;
    }
    e.mem.set_u32(block, 0);
    0
}

// Translated from 00ec7595 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_finite` (Ghidra's function ID), cdecl, a double (low word, high word): 1 unless its
/// exponent bits (`0x7ff0` of the top half word) are all set.
pub fn finite(_e: &mut Engine, _low: u32, high: u32) -> u32 {
    ((high >> 16) as u16 & 0x7ff0 != 0x7ff0) as u32
}

// Translated from 00ec75b1 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_isnan` (Ghidra's function ID), cdecl, a double (low word, high word). The dword at
/// `[EBP+0xe]` is read (the top half word of the double and the two bytes above it, which the
/// `AND 0x7ff8` drops). Exponent all ones with the quiet bit clear (`0x7ff0`): 1 when any
/// other fraction bit is set (`high & 0x7ffff`, then the low word), else 0. Otherwise 1 when
/// the bits are `0x7ff8` (a quiet NaN or the indefinite), else 0.
pub fn isnan(_e: &mut Engine, low: u32, high: u32) -> u32 {
    let top = (high >> 16) as u16 & 0x7ff8;
    if top == 0x7ff0 && (high & 0x7ffff != 0 || low != 0) {
        return 1;
    }
    (top == 0x7ff8) as u32
}

// Translated from 00ec913a (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CIfmod` (the register form of `fmod`; no label in the map), `y` in `ST1`, `x` in `ST0`
/// (registered `y` first): `MOV EDX, 011c18e0` (`fmod`'s descriptor) and a jump to
/// `__cintrindisp2`.
pub fn ci_fmod(e: &mut Engine, y: f64, x: f64) -> f64 {
    e.call(CINTRINDISP2_ADDR, &args![FMOD_DESCRIPTOR, y, x])
        .f64()
}

// Translated from 00ec9886 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_fprintf` (`00ec9774`): `__unlock_file` of its first argument
/// (`[EBP+8]`, the stream).
pub fn fprintf_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE_ADDR, &args![stream]);
}

// Translated from 00ec9a3d (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_fsopen` (`00ec9983`): `__unlock_file` of the word at `[EBP+8]`
/// (the stream `_fsopen` keeps in its first argument's slot).
pub fn fsopen_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE_ADDR, &args![stream]);
}

// Translated from 00ecada6 (disassembled, FalloutNV.exe 1.4.0.525)
/// `srand` (Ghidra's function ID), cdecl: the seed is stored in the thread's data (`_getptd`
/// +0x14, the `rand` state).
pub fn srand(e: &mut Engine, seed: u32) {
    let ptd = e.call(GETPTD_ADDR, &args![]).u32();
    e.mem.set_u32(ptd.wrapping_add(0x14), seed);
}

// Translated from 00ecadb8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `rand` (Ghidra's function ID), no arguments: the thread's state (`_getptd` +0x14, which
/// `_initptd` starts at 1 and `srand` sets) becomes `state * 0x343fd + 0x269ec3` (32-bit), and
/// its bits 16 to 30 are answered.
pub fn rand(e: &mut Engine) -> i32 {
    let ptd = e.call(GETPTD_ADDR, &args![]).u32();
    let state = e
        .mem
        .u32(ptd.wrapping_add(0x14))
        .wrapping_mul(0x343fd)
        .wrapping_add(0x269ec3);
    e.mem.set_u32(ptd.wrapping_add(0x14), state);
    ((state >> 16) & 0x7fff) as i32
}

// Translated from 00ecaebb (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecaebb - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117fd88) is not translated; its __finally block (00ecaf0c) is called as the exe calls it on the way out.
/// `eh vector copy constructor iterator` (MSVC runtime, Ghidra library match), stdcall (`RET
/// 0x18`): destination array, source array, element size, count, copy constructor, destructor.
/// While the index (from 0) is below the count (signed), the copy constructor runs (`thiscall`:
/// the destination element in `ECX`, the source element pushed) and both cursors move up by the
/// size. Then the "done" local is 1 and the `__finally` block runs with the frame as the loop
/// left it.
pub fn eh_vector_copy_constructor_iterator(
    e: &mut Engine,
    destination: u32,
    source: u32,
    size: u32,
    count: i32,
    constructor: u32,
    destructor: u32,
) {
    let (mut destination, mut source) = (destination, source);
    let mut index: i32 = 0;
    while index < count {
        e.call(constructor, &args![destination, source]);
        destination = destination.wrapping_add(size);
        source = source.wrapping_add(size);
        index = index.wrapping_add(1);
    }
    e.call(
        COPY_ITERATOR_FINALLY_ADDR,
        &args![1u32, destination, size, index as u32, destructor],
    );
}

// Translated from 00ecaf0c (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of the `eh vector copy constructor iterator` (`00ecaebb`), on its
/// frame: the "done" local (`EBP-0x20`), the destination cursor (`EBP+8`), the element size
/// (`EBP+0x10`), the index reached (`EBP-0x1c`) and the destructor (`EBP+0x1c`), in that
/// order. When the loop did not finish (a constructor raised an exception),
/// `__ArrayUnwind(destination, size, index, destructor)` destroys the elements made.
pub fn eh_vector_copy_constructor_finally(
    e: &mut Engine,
    done: u32,
    destination: u32,
    size: u32,
    index: u32,
    destructor: u32,
) {
    if done == 0 {
        e.call(
            ARRAY_UNWIND_ADDR,
            &args![destination, size, index, destructor],
        );
    }
}

// Translated from 00ecb0fe (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `fwrite` (`00ecb086`): `__unlock_file` of its fourth argument
/// (`[EBP+0x14]`, the stream).
pub fn fwrite_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE_ADDR, &args![stream]);
}

// Translated from 00ecbc5b (disassembled, FalloutNV.exe 1.4.0.525)
/// `__strupr_s` (Ghidra's function ID), cdecl: string, size; `__strupr_s_l` with a null
/// locale answers.
pub fn strupr_s(e: &mut Engine, text: u32, size: u32) -> u32 {
    e.call(STRUPR_S_L_ADDR, &args![text, size, 0u32]).u32()
}

// Translated from 00eccc7e (disassembled, FalloutNV.exe 1.4.0.525)
/// `___crtCorExitProcess` (Ghidra's function ID), cdecl: the exit code. When
/// `GetModuleHandleW(L"mscoree.dll")` finds the module and `GetProcAddress(it,
/// "CorExitProcess")` the function, that is called with the code (stdcall).
pub fn crt_cor_exit_process(e: &mut Engine, code: u32) {
    let module = e
        .call(IMPORT_GET_MODULE_HANDLE_W, &args![TEXT_MSCOREE])
        .u32();
    if module == 0 {
        return;
    }
    let function = e
        .call(
            IMPORT_GET_PROC_ADDRESS,
            &args![module, TEXT_COR_EXIT_PROCESS],
        )
        .u32();
    if function != 0 {
        e.call(function, &args![code]);
    }
}

// Translated from 00ecccd3 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__initterm` (Ghidra's function ID): `EAX` the table's start (here the first word), then
/// cdecl its end. Each nonzero pointer below the end (unsigned) is called, in order.
pub fn initterm(e: &mut Engine, start: u32, end: u32) {
    let mut at = start;
    while at < end {
        let function = e.mem.u32(at);
        if function != 0 {
            e.call(function, &args![]);
        }
        at = at.wrapping_add(4);
    }
}

// Translated from 00ecccf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__initterm_e` (Ghidra's function ID), cdecl: the table's start and end. Each nonzero
/// pointer below the end (unsigned) is called in order until one answers nonzero, which is
/// answered; 0 when all ran.
pub fn initterm_e(e: &mut Engine, start: u32, end: u32) -> u32 {
    let mut at = start;
    let mut result = 0;
    while at < end {
        if result != 0 {
            break;
        }
        let function = e.mem.u32(at);
        if function != 0 {
            result = e.call(function, &args![]).u32();
        }
        at = at.wrapping_add(4);
    }
    result
}

// Translated from 00eccd14 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cinit` (Ghidra's function ID), cdecl: the floating-point flag. When the floating-point
/// initializer pointer (`010f1f1c`) is set and lies in a read-only section of the image
/// (`_IsNonwritableInCurrentImage`), it is called with the flag. `__initp_misc_cfltcvt_tab`;
/// `_initterm_e` over the C initializers (`010113c4..010113e8`), whose nonzero answer is
/// answered; `_atexit` of the C++ terminator runner (`00eddb44`); `_initterm` over the C++
/// initializers (`00fdf4a4..010113c0`); and when the TLS callback pointer (`01271a94`) is set
/// and read-only, it is called with `(0, 2, 0)` (stdcall: `DLL_THREAD_ATTACH`). Answers 0.
///
/// The host runs the initializers through `crate::static_init::run` instead, which skips
/// those not translated yet; this is the exe's own order.
pub fn cinit(e: &mut Engine, float_flag: u32) -> u32 {
    if e.mem.u32(FPINIT_POINTER) != 0
        && e.call(IS_NONWRITABLE_ADDR, &args![FPINIT_POINTER]).u32() != 0
    {
        let function = e.mem.u32(FPINIT_POINTER);
        e.call(function, &args![float_flag]);
    }
    e.call(INITP_MISC_CFLTCVT_TAB_ADDR, &args![]);
    let result = e.call(INITTERM_E_ADDR, &args![C_INIT.0, C_INIT.1]).u32();
    if result != 0 {
        return result;
    }
    e.call(ATEXIT_ADDR, &args![CPP_TERMINATORS_ADDR]);
    e.call(INITTERM_ADDR, &args![CPP_INIT.0, CPP_INIT.1]);
    if e.mem.u32(TLS_CALLBACK) != 0 && e.call(IS_NONWRITABLE_ADDR, &args![TLS_CALLBACK]).u32() != 0
    {
        let function = e.mem.u32(TLS_CALLBACK);
        e.call(function, &args![0u32, 2u32, 0u32]);
    }
    0
}

// Translated from 00ecceb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `doexit` (`00eccd99`): with its third argument (`[EBP+0x10]`,
/// return to the caller) nonzero, `_unlock(8)`.
pub fn doexit_finally(e: &mut Engine, retcaller: u32) {
    if retcaller != 0 {
        e.call(UNLOCK_ADDR, &args![8u32]);
    }
}

// Translated from 00eccec5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_exit` (Ghidra's function ID), cdecl: the code; `doexit(code, 0, 0)`.
pub fn exit(e: &mut Engine, code: u32) {
    e.call(DOEXIT_ADDR, &args![code, 0u32, 0u32]);
}

// Translated from 00ecdffb (disassembled, FalloutNV.exe 1.4.0.525)
/// The stdio terminator (unnamed; the CRT's `__endstdio`, a C terminator): `_flushall`
/// (`00ecdac3`); `_fcloseall` when the byte at `0126fde0` is set; the stream table
/// (`01270a70`) freed.
pub fn end_stdio(e: &mut Engine) {
    e.call(FLUSHALL_ADDR, &args![]);
    if e.mem.u8(EXIT_BYTE) != 0 {
        e.call(FCLOSEALL_ADDR, &args![]);
    }
    let table = e.mem.u32(PIOB);
    e.call(FREE_ADDR, &args![table]);
}

// Translated from 00ecf7ac (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `___updatetlocinfo` (`00ecf742`): `_unlock(0xc)`, then `ESI` is
/// loaded from the frame local at `EBP-0x1c` (the locale its function answers), which it
/// gives back here.
pub fn updatetlocinfo_finally(e: &mut Engine, info: u32) -> u32 {
    e.call(UNLOCK_ADDR, &args![0xcu32]);
    info
}

/// The thread's locale as the accessors read it: `_getptd` +0x6C, refreshed through
/// `___updatetlocinfo` when it is not the global one (`011c23f8`) and the thread's +0x70 has
/// none of the bits of `__globallocalestatus` (`011c2310`).
fn thread_locale(e: &mut Engine) -> u32 {
    let ptd = e.call(GETPTD_ADDR, &args![]).u32();
    let info = e.mem.u32(ptd.wrapping_add(0x6c));
    if info != e.mem.u32(PTLOCINFO)
        && e.mem.u32(ptd.wrapping_add(0x70)) & e.mem.u32(GLOBAL_LOCALE_STATUS) == 0
    {
        return e.call(UPDATETLOCINFO_ADDR, &args![]).u32();
    }
    info
}

// Translated from 00ed08c8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___pctype_func` (Ghidra's function ID), no arguments: the thread's locale (refreshed as
/// `___lc_codepage_func` does) and its character-type table pointer (+0xC8).
pub fn pctype_func(e: &mut Engine) -> u32 {
    let info = thread_locale(e);
    e.mem.u32(info.wrapping_add(0xc8))
}

// Translated from 00ed0cb9 (disassembled, FalloutNV.exe 1.4.0.525)
/// `____lc_handle_func` (Ghidra's function ID), no arguments: the thread's locale (refreshed
/// as `___lc_codepage_func` does) plus 0xC, the address of its `lc_handle` array.
pub fn lc_handle_func(e: &mut Engine) -> u32 {
    thread_locale(e).wrapping_add(0xc)
}

// Translated from 00ed0d70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__realloc_crt` (Ghidra's function ID), cdecl: block, size. `realloc(block, size)`; while
/// that gives null for a nonzero size and the wait limit (`0126fdfc`) is above 0, `Sleep` for
/// the time waited so far (0 first) and retry with the wait grown by 1000 ms, until the grown
/// wait would pass the limit (unsigned; it becomes -1 and the loop ends). Answers the block or
/// null.
pub fn realloc_crt(e: &mut Engine, block: u32, size: u32) -> u32 {
    let mut waited: u32 = 0;
    loop {
        let result = e.call(REALLOC_ADDR, &args![block, size]).u32();
        if result != 0 || size == 0 || e.mem.u32(MAX_WAIT) == 0 {
            return result;
        }
        e.call(IMPORT_SLEEP, &args![waited]);
        let next = waited.wrapping_add(1000);
        waited = if next > e.mem.u32(MAX_WAIT) {
            0xffff_ffff
        } else {
            next
        };
        if waited == 0xffff_ffff {
            return result;
        }
    }
}

// Translated from 00ed1312 (disassembled, FalloutNV.exe 1.4.0.525)
/// The first `__finally` block of `_freefls` (`00ed11f8`; it uses nothing of the frame):
/// `_unlock(0xd)` (the multibyte lock).
pub fn unlock_mbc_finally(e: &mut Engine) {
    e.call(UNLOCK_ADDR, &args![0xdu32]);
}

// Translated from 00ed131e (disassembled, FalloutNV.exe 1.4.0.525)
/// The second `__finally` block of `_freefls` (`00ed11f8`; it uses nothing of the frame):
/// `_unlock(0xc)` (the locale lock).
pub fn unlock_locale_finally(e: &mut Engine) {
    e.call(UNLOCK_ADDR, &args![0xcu32]);
}

// Translated from 00ed3d7a (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `___updatetmbcinfo` (`00ed3cdf`; it uses nothing of the frame):
/// `_unlock(0xd)`.
pub fn updatetmbcinfo_finally(e: &mut Engine) {
    e.call(UNLOCK_ADDR, &args![0xdu32]);
}

// Translated from 00ed5bae (disassembled, FalloutNV.exe 1.4.0.525)
/// The word `_invalid_parameter` and the unhandled-exception path clear (`AND [01270a60],
/// 0`), whatever argument they push (cdecl; unnamed).
pub fn clear_watson_word(e: &mut Engine, _unused: u32) {
    e.mem.set_u32(WATSON_WORD, 0);
}

// Translated from 00ed5bc0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___ascii_strnicmp` (Ghidra's function ID), cdecl: two strings and a count. A count of 0
/// answers 0. Otherwise byte by byte, each folded to lower case when in `'A'..='Z'`: the first
/// difference answers -1 (first below, unsigned) or 1; a zero byte in either ends the
/// comparison (the folded pair compared once more: equal 0, else -1 or 1); `count` equal
/// bytes answer 0. A zero byte stops before the fold, so the bytes compared then are the raw
/// ones.
pub fn ascii_strnicmp(e: &mut Engine, first: u32, second: u32, count: u32) -> i32 {
    if count == 0 {
        return 0;
    }
    let (mut a_at, mut b_at, mut left) = (first, second, count);
    let fold = |c: u8| {
        if (0x41..=0x5a).contains(&c) {
            c + 0x20
        } else {
            c
        }
    };
    let (mut ah, mut al);
    loop {
        ah = e.mem.u8(a_at);
        al = e.mem.u8(b_at);
        if ah == 0 || al == 0 {
            break;
        }
        a_at = a_at.wrapping_add(1);
        b_at = b_at.wrapping_add(1);
        ah = fold(ah);
        al = fold(al);
        if ah != al {
            return if ah < al { -1 } else { 1 };
        }
        left -= 1;
        if left == 0 {
            break;
        }
    }
    // 00ed5c0b: XOR ECX, ECX; CMP AH, AL.
    if ah == al {
        0
    } else if ah < al {
        -1
    } else {
        1
    }
}

// Translated from 00ed81fd (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_close` (`00ed813a`): `_unlock_fhandle` of its first argument
/// (`[EBP+8]`, the handle).
pub fn close_finally(e: &mut Engine, handle: u32) {
    e.call(UNLOCK_FHANDLE_ADDR, &args![handle]);
}

// Translated from 00ed8630 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_getstream` (`00ed8501`; it uses nothing of the frame):
/// `_unlock(1)`.
pub fn getstream_finally(e: &mut Engine) {
    e.call(UNLOCK_ADDR, &args![1u32]);
}

// Translated from 00edc815 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_write` (`00edc743`): `_unlock_fhandle` of its first argument
/// (`[EBP+8]`, the handle).
pub fn write_finally(e: &mut Engine, handle: u32) {
    e.call(UNLOCK_FHANDLE_ADDR, &args![handle]);
}

// Translated from 00ee213b (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_lseeki64` (`00ee202c`): `_unlock_fhandle` of its first argument
/// (`[EBP+8]`, the handle).
pub fn lseeki64_finally(e: &mut Engine, handle: u32) {
    e.call(UNLOCK_FHANDLE_ADDR, &args![handle]);
}

// Translated from 00ee38da (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `__sopen_helper` (`00ee383c`), on its frame (`ESI` is 0 on both
/// ways in): the unlock flag `__tsopen_nolock` set (`EBP-0x1c`), its answer (`EBP-0x20`, an
/// error) and the result pointer (`EDI`, `[EBP+0x18]`). With the flag set: an error first
/// clears the handle's `FOPEN` bit (bit 0 of the `osfile` byte, +4 in its `ioinfo` entry,
/// `__pioinfo[fh >> 5] + (fh & 0x1f) * 0x40`), then `_unlock_fhandle(fh)`.
pub fn sopen_helper_finally(e: &mut Engine, unlock: u32, error: u32, result: u32) {
    if unlock == 0 {
        return;
    }
    if error != 0 {
        let handle = e.mem.u32(result);
        let block = e
            .mem
            .u32(PIOINFO.wrapping_add(((handle as i32 >> 5) as u32).wrapping_mul(4)));
        let osfile = block.wrapping_add((handle & 0x1f) << 6).wrapping_add(4);
        let flags = e.mem.u8(osfile);
        e.mem.set_u8(osfile, flags & 0xfe);
    }
    let handle = e.mem.u32(result);
    e.call(UNLOCK_FHANDLE_ADDR, &args![handle]);
}

// Translated from 00eddb44 (disassembled, FalloutNV.exe 1.4.0.525)
/// The C++ terminator runner `__cinit` gives to `_atexit` (unnamed; the CRT's `_RTC_Terminate`
/// shape): each nonzero pointer of `0113ec3c..0113ec3c` is called in order (the table is empty
/// in this exe, so nothing runs).
pub fn run_cpp_terminators(e: &mut Engine) {
    let mut at = CPP_TERMINATORS.0;
    while at < CPP_TERMINATORS.1 {
        let function = e.mem.u32(at);
        if function != 0 {
            e.call(function, &args![]);
        }
        at = at.wrapping_add(4);
    }
}

// Translated from 00eddc30 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__ValidateImageBase` (Ghidra's function ID), cdecl: the image base. 1 when it starts with
/// `MZ`, the word at +0x3c leads to `PE\0\0` and the optional header's magic (+0x18 there) is
/// 0x10b (PE32); else 0.
pub fn validate_image_base(e: &mut Engine, base: u32) -> u32 {
    if e.mem.u16(base) != 0x5a4d {
        return 0;
    }
    let header = e.mem.u32(base.wrapping_add(0x3c)).wrapping_add(base);
    if e.mem.u32(header) != 0x4550 {
        return 0;
    }
    (e.mem.u16(header.wrapping_add(0x18)) == 0x10b) as u32
}

// Translated from 00eddc70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__FindPESection` (Ghidra's function ID), cdecl: the image base and an offset into the
/// image. The section headers follow the optional header (its size at +0x14 of the PE header,
/// + 0x18); the first (of the count at +6) whose virtual address (+0xc) is at or below the
/// offset and whose end (+ the virtual size at +8) is above it (unsigned) is answered (its
/// header's address); none, 0.
pub fn find_pe_section(e: &mut Engine, base: u32, offset: u32) -> u32 {
    let header = e.mem.u32(base.wrapping_add(0x3c)).wrapping_add(base);
    let optional_size = u32::from(e.mem.u16(header.wrapping_add(0x14)));
    let count = u32::from(e.mem.u16(header.wrapping_add(6)));
    let mut section = optional_size.wrapping_add(header).wrapping_add(0x18);
    let mut index = 0;
    while index < count {
        let start = e.mem.u32(section.wrapping_add(0xc));
        if offset >= start {
            let end = e.mem.u32(section.wrapping_add(8)).wrapping_add(start);
            if offset < end {
                return section;
            }
        }
        index += 1;
        section = section.wrapping_add(0x28);
    }
    0
}

// Translated from 00eddcc0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00eddcc0 - the SEH frame (FS:[0] registration with the scope table 011803e8 and handler 00ecc8e0) and its exception filter (00eddd4b: an access violation answers 0) are not translated.
/// `__IsNonwritableInCurrentImage` (Ghidra's function ID), cdecl: an address. With a valid
/// image at `00400000` (`__ValidateImageBase`) and a section holding the address
/// (`__FindPESection(00400000, address - 00400000)`), 1 when that section is not writable
/// (bit 31 of its characteristics, +0x24, clear), else 0; no image or no section, 0.
pub fn is_nonwritable_in_current_image(e: &mut Engine, address: u32) -> u32 {
    if e.call(VALIDATE_IMAGE_BASE_ADDR, &args![IMAGE_BASE]).u32() == 0 {
        return 0;
    }
    let section = e
        .call(
            FIND_PE_SECTION_ADDR,
            &args![IMAGE_BASE, address.wrapping_sub(IMAGE_BASE)],
        )
        .u32();
    if section == 0 {
        return 0;
    }
    !(e.mem.u32(section.wrapping_add(0x24)) >> 31) & 1
}

/// `_setlocale`'s calls: `__copytlocinfo_nolock` (`00ecf6de`; registers: `EAX` the new info,
/// `ECX` the thread's, passed as these two words), `__setlocale_nolock` (`00ed0153`; `EDX` the
/// new info, `ECX` the locale text, then the category on the stack: three words in that
/// order), `strcmp` (`00ec6da0`), `__updatetlocinfoEx_nolock` (`00ecf704`; `EAX` the slot,
/// `EDI` the info, as two words), `___removelocaleref` (`00ecf645`), `___freetlocinfo`
/// (`00ecf46d`), `memcpy` (`00ec44d0`), `sync_legacy_variables_lk` (`00ecf7b8`), `_lock`
/// (`00eddf39`) and its `__finally` block (`00ed04d9`).
const COPYTLOCINFO_NOLOCK: u32 = 0x00ec_f6de;
const SETLOCALE_NOLOCK: u32 = 0x00ed_0153;
const STRCMP_ADDR: u32 = 0x00ec_6da0;
const UPDATETLOCINFO_EX_NOLOCK: u32 = 0x00ec_f704;
const REMOVELOCALEREF: u32 = 0x00ec_f645;
const FREETLOCINFO: u32 = 0x00ec_f46d;
const MEMCPY_ADDR: u32 = 0x00ec_44d0;
const SYNC_LEGACY_VARIABLES: u32 = 0x00ec_f7b8;
const LOCK_ADDR: u32 = 0x00ed_df39;
const SETLOCALE_FINALLY_ADDR: u32 = 0x00ed_04d9;
/// The C locale's name (`011c2318`, `"C"`), `__locale_changed` (`0126fdf4`) and the legacy
/// copy of the global locale's `lc_handle` words (`01270484`, 0x18 bytes).
const C_LOCALE_NAME: u32 = 0x011c_2318;
const LOCALE_CHANGED: u32 = 0x0126_fdf4;
const LEGACY_HANDLES: u32 = 0x0127_0484;

// Translated from 00ed036e (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed036e - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 011801c0) is not translated; its __finally blocks (the unlocks at 00ed049c and 00ed04a8, and 00ed04d9) are run as the exe runs them on the way out.
/// `_setlocale` (Ghidra's function ID), cdecl: category, locale text. A category above 5 sets
/// `errno` to `EINVAL`, reports and answers null. Otherwise, with the thread's data (`_getptd`)
/// refreshed (`___updatetlocinfo`) and bit 0x10 of its `_ownlocale` (+0x70) set: a 0xd8-byte
/// info from `__calloc_crt`; none, null. Under lock 0xc the thread's info (+0x6c) is copied
/// into it (`__copytlocinfo_nolock`), then `__setlocale_nolock` sets the category. A null
/// answer releases and frees the new info. Otherwise a locale other than `"C"` sets
/// `__locale_changed`, and under lock 0xc the new info becomes the thread's
/// (`__updatetlocinfoEx_nolock`) and loses the creator's reference; unless the thread keeps
/// its own locale (`_ownlocale` bit 1) or the global status says so (`__globallocalestatus`
/// bit 0) it also becomes the global one (`__ptlocinfo`), its `lc_handle` words (+0xc, 0x18
/// bytes) are copied to `01270484` and `sync_legacy_variables_lk` runs. Bit 0x10 is cleared
/// again (`00ed04d9`) and the answer of `__setlocale_nolock` (or null) is answered.
pub fn setlocale(e: &mut Engine, category: u32, locale: u32) -> u32 {
    if category > 5 {
        let slot = e.call(ERRNO_ADDR, &args![]).u32();
        e.mem.set_u32(slot, 0x16);
        e.call(INVALID_PARAMETER_ADDR, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        return 0;
    }
    let ptd = e.call(GETPTD_ADDR, &args![]).u32();
    e.call(UPDATETLOCINFO_ADDR, &args![]);
    let own = e.mem.u32(ptd.wrapping_add(0x70));
    e.mem.set_u32(ptd.wrapping_add(0x70), own | 0x10);
    let mut result = 0;
    let info = e.call(CALLOC_CRT_ADDR, &args![0xd8u32, 1u32]).u32();
    if info != 0 {
        e.call(LOCK_ADDR, &args![0xcu32]);
        let current = e.mem.u32(ptd.wrapping_add(0x6c));
        e.call(COPYTLOCINFO_NOLOCK, &args![info, current]);
        // 00ed049c: the first __finally block.
        e.call(UNLOCK_ADDR, &args![0xcu32]);
        result = e
            .call(SETLOCALE_NOLOCK, &args![info, locale, category])
            .u32();
        if result == 0 {
            e.call(REMOVELOCALEREF, &args![info]);
            e.call(FREETLOCINFO, &args![info]);
        } else {
            if locale != 0 && e.call(STRCMP_ADDR, &args![locale, C_LOCALE_NAME]).u32() != 0 {
                e.mem.set_u32(LOCALE_CHANGED, 1);
            }
            e.call(LOCK_ADDR, &args![0xcu32]);
            let slot = ptd.wrapping_add(0x6c);
            e.call(UPDATETLOCINFO_EX_NOLOCK, &args![slot, info]);
            e.call(REMOVELOCALEREF, &args![info]);
            if e.mem.u8(ptd.wrapping_add(0x70)) & 2 == 0 && e.mem.u8(GLOBAL_LOCALE_STATUS) & 1 == 0
            {
                let thread_info = e.mem.u32(slot);
                e.call(UPDATETLOCINFO_EX_NOLOCK, &args![PTLOCINFO, thread_info]);
                let global = e.mem.u32(PTLOCINFO);
                e.call(
                    MEMCPY_ADDR,
                    &args![LEGACY_HANDLES, global.wrapping_add(0xc), 0x18u32],
                );
                e.call(SYNC_LEGACY_VARIABLES, &args![]);
            }
            // 00ed04a8: the second __finally block.
            e.call(UNLOCK_ADDR, &args![0xcu32]);
        }
    }
    e.call(SETLOCALE_FINALLY_ADDR, &args![ptd]);
    result
}

// Translated from 00ed04d9 (disassembled, FalloutNV.exe 1.4.0.525)
/// The last `__finally` block of `_setlocale` (`00ed036e`): bit 0x10 of the thread's
/// `_ownlocale` (+0x70 of its data, `ESI` from the frame) cleared.
pub fn setlocale_finally(e: &mut Engine, ptd: u32) {
    let own = e.mem.u32(ptd.wrapping_add(0x70));
    e.mem.set_u32(ptd.wrapping_add(0x70), own & 0xffff_ffef);
}

/// The registry entries of the functions that take a register: `EAX` first.
fn alloca_probe_16_entry(e: &mut Engine, words: &[u32]) -> Ret {
    alloca_probe_16(e, words[0], words[1]).into_ret()
}

fn initterm_entry(e: &mut Engine, words: &[u32]) -> Ret {
    initterm(e, words[0], words[1]);
    Ret::default()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00ec_5ec0, alloca_probe_16_entry as AbiFn),
        entry!(0x00ec_6522, onexit_init() -> u32),
        entry!(0x00ec_7595, finite(u32, u32) -> u32),
        entry!(0x00ec_75b1, isnan(u32, u32) -> u32),
        entry!(0x00ec_913a, ci_fmod(f64, f64) -> f64),
        entry!(0x00ec_9886, fprintf_finally(u32)),
        entry!(0x00ec_9a3d, fsopen_finally(u32)),
        entry!(0x00ec_ada6, srand(u32)),
        entry!(0x00ec_adb8, rand() -> i32),
        entry!(
            0x00ec_aebb,
            eh_vector_copy_constructor_iterator(u32, u32, u32, i32, u32, u32)
        ),
        entry!(
            0x00ec_af0c,
            eh_vector_copy_constructor_finally(u32, u32, u32, u32, u32)
        ),
        entry!(0x00ec_b0fe, fwrite_finally(u32)),
        entry!(0x00ec_bc5b, strupr_s(u32, u32) -> u32),
        entry!(0x00ec_cc7e, crt_cor_exit_process(u32)),
        (INITTERM_ADDR, initterm_entry as AbiFn),
        entry!(0x00ec_ccf0, initterm_e(u32, u32) -> u32),
        entry!(0x00ec_cd14, cinit(u32) -> u32),
        entry!(0x00ec_ceb0, doexit_finally(u32)),
        entry!(0x00ec_cec5, exit(u32)),
        entry!(0x00ec_dffb, end_stdio()),
        entry!(0x00ec_f7ac, updatetlocinfo_finally(u32) -> u32),
        entry!(0x00ed_08c8, pctype_func() -> u32),
        entry!(0x00ed_0cb9, lc_handle_func() -> u32),
        entry!(0x00ed_0d70, realloc_crt(u32, u32) -> u32),
        entry!(0x00ed_1312, unlock_mbc_finally()),
        entry!(0x00ed_131e, unlock_locale_finally()),
        entry!(0x00ed_3d7a, updatetmbcinfo_finally()),
        entry!(0x00ed_5bae, clear_watson_word(u32)),
        entry!(0x00ed_5bc0, ascii_strnicmp(u32, u32, u32) -> i32),
        entry!(0x00ed_81fd, close_finally(u32)),
        entry!(0x00ed_8630, getstream_finally()),
        entry!(0x00ed_c815, write_finally(u32)),
        entry!(0x00ee_213b, lseeki64_finally(u32)),
        entry!(0x00ee_38da, sopen_helper_finally(u32, u32, u32)),
        entry!(0x00ed_db44, run_cpp_terminators()),
        entry!(0x00ed_dc30, validate_image_base(u32) -> u32),
        entry!(0x00ed_dc70, find_pe_section(u32, u32) -> u32),
        entry!(0x00ed_dcc0, is_nonwritable_in_current_image(u32) -> u32),
        entry!(0x00ed_036e, setlocale(u32, u32) -> u32),
        entry!(0x00ed_04d9, setlocale_finally(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;
    type Section = (u32, u32, u32);

    fn calls(e: &Engine, addr: u32) -> Calls {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The addresses called, in order, among `watched`.
    fn order(e: &Engine, watched: &[u32]) -> Vec<u32> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .map(|c| c.0)
            .filter(|a| watched.contains(a))
            .collect()
    }

    /// An engine with this file's functions registered, the named addresses doubled to answer
    /// 0 and do nothing, and the call log on.
    fn engine(doubled: &[u32]) -> Engine {
        let mut e = Engine::new();
        for (addr, f) in funcs() {
            e.register(addr, f);
        }
        for &addr in doubled {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    #[test]
    fn alloca16_esp_is_the_two_probes_composed() {
        let mut e = engine(&[]);
        e.map(0x0010_0000, 0x4000);
        e.register_double(ALLOCA_PROBE_ADDR, |e, a| {
            alloca_probe(e, a[0], a[1]).into_ret()
        });
        for esp in [
            0x0010_3000u32,
            0x0010_3004,
            0x0010_3008,
            0x0010_300c,
            0x0010_3ffc,
        ] {
            for size in [0x190u32, 0x24, 0xc, 0x100, 1] {
                // The call pushes the return address: the probe sees `esp - 4`.
                e.mem.set_u32(esp - 4, 0x00aa_bbcc);
                let probed = alloca_probe_16(&mut e, size, esp - 4);
                let modelled = alloca16_esp(esp, size);
                assert_eq!(modelled, probed, "esp {esp:x} size {size:x}");
                assert_eq!(modelled & 0xf, 0);
                assert!(esp - modelled >= size && esp - modelled < size + 0x10);
            }
        }
        // A size past the stack top: the probe's top is address 0, the answer 4.
        assert_eq!(alloca16_esp(0x1000, 0x2000), 4);
    }

    #[test]
    fn alloca_probe_16_rounds_to_the_lower_16_byte_boundary() {
        let mut e = engine(&[]);
        e.register_double(ALLOCA_PROBE_ADDR, |_, a| (a[0] ^ 0x5000_0000).into_ret());
        // esp + 4 = 0x1008; 0x1008 - 0x100 = 0xf08 -> 8 more bytes.
        assert_eq!(alloca_probe_16(&mut e, 0x100, 0x1004), 0x5000_0108);
        // Already aligned: nothing added.
        assert_eq!(alloca_probe_16(&mut e, 0x104, 0x1000), 0x5000_0104);
        // A carry saturates to all ones.
        assert_eq!(alloca_probe_16(&mut e, 0xffff_fffe, 0x1003), 0xafff_ffff);
        assert_eq!(
            calls(&e, ALLOCA_PROBE_ADDR),
            vec![
                vec![0x108, 0x1004],
                vec![0x104, 0x1000],
                vec![0xffff_ffff, 0x1003]
            ]
        );
        assert_eq!(
            e.call(0x00ec_5ec0, &args![0x100u32, 0x1004u32]).u32(),
            0x5000_0108
        );
    }

    #[test]
    fn onexit_init_makes_the_table_or_answers_24() {
        let mut e = engine(&[]);
        e.mem.map(ONEXIT_END, 8);
        let block = e.mem.alloc(0x80);
        e.mem.set_u32(block, 0xdead);
        answer(&mut e, CALLOC_CRT_ADDR, block);
        e.register_double(ENCODE_POINTER_ADDR, |_, a| (a[0] ^ 0x55).into_ret());
        assert_eq!(onexit_init(&mut e), 0);
        assert_eq!(calls(&e, CALLOC_CRT_ADDR), vec![vec![32, 4]]);
        assert_eq!(e.mem.u32(ONEXIT_BEGIN), block ^ 0x55);
        assert_eq!(e.mem.u32(ONEXIT_END), block ^ 0x55);
        assert_eq!(e.mem.u32(block), 0);
        answer(&mut e, CALLOC_CRT_ADDR, 0);
        assert_eq!(onexit_init(&mut e), 0x18);
        assert_eq!(e.mem.u32(ONEXIT_BEGIN), 0x55);
        assert_eq!(e.mem.u32(ONEXIT_END), 0x55);
    }

    #[test]
    fn finite_and_isnan_read_the_exponent_bits() {
        let mut e = engine(&[]);
        let cases: [(u64, u32, u32); 10] = [
            (1.5f64.to_bits(), 1, 0),
            (0, 1, 0),
            (f64::MAX.to_bits(), 1, 0),
            (f64::INFINITY.to_bits(), 0, 0),
            (f64::NEG_INFINITY.to_bits(), 0, 0),
            (0x7ff8_0000_0000_0000, 0, 1),
            (0xfff8_0000_0000_0000, 0, 1),
            // Signalling NaNs: a bit in the high word's fraction, or only in the low word.
            (0x7ff4_0000_0000_0000, 0, 1),
            (0x7ff0_0000_0000_0001, 0, 1),
            (0xfff0_0000_8000_0000, 0, 1),
        ];
        for (bits, finite_want, nan_want) in cases {
            let (low, high) = (bits as u32, (bits >> 32) as u32);
            assert_eq!(finite(&mut e, low, high), finite_want, "{bits:016x}");
            assert_eq!(isnan(&mut e, low, high), nan_want, "{bits:016x}");
        }
        let bits = f64::NAN.to_bits();
        let (low, high) = (bits as u32, (bits >> 32) as u32);
        assert_eq!(e.call(0x00ec_75b1, &args![low, high]).u32(), 1);
        assert_eq!(e.call(0x00ec_7595, &args![low, high]).u32(), 0);
    }

    #[test]
    fn ci_fmod_dispatches_with_fmods_descriptor() {
        let mut e = engine(&[]);
        e.register_double(CINTRINDISP2_ADDR, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[1]) | u64::from(a[2]) << 32)
                - f64::from_bits(u64::from(a[3]) | u64::from(a[4]) << 32),
            ..Ret::default()
        });
        assert_eq!(ci_fmod(&mut e, 7.0, 2.0), 5.0);
        let c = calls(&e, CINTRINDISP2_ADDR);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0][0], FMOD_DESCRIPTOR);
    }

    #[test]
    fn finally_blocks_give_their_locks_back() {
        let unlocks = [UNLOCK_ADDR, UNLOCK_FILE_ADDR, UNLOCK_FHANDLE_ADDR];
        let mut e = engine(&unlocks);
        fprintf_finally(&mut e, 0x11);
        fsopen_finally(&mut e, 0x12);
        fwrite_finally(&mut e, 0x13);
        close_finally(&mut e, 0x21);
        write_finally(&mut e, 0x22);
        lseeki64_finally(&mut e, 0x23);
        doexit_finally(&mut e, 0);
        doexit_finally(&mut e, 1);
        assert_eq!(updatetlocinfo_finally(&mut e, 0x4444), 0x4444);
        unlock_mbc_finally(&mut e);
        unlock_locale_finally(&mut e);
        updatetmbcinfo_finally(&mut e);
        getstream_finally(&mut e);
        assert_eq!(
            calls(&e, UNLOCK_FILE_ADDR),
            vec![vec![0x11], vec![0x12], vec![0x13]]
        );
        assert_eq!(
            calls(&e, UNLOCK_FHANDLE_ADDR),
            vec![vec![0x21], vec![0x22], vec![0x23]]
        );
        assert_eq!(
            calls(&e, UNLOCK_ADDR),
            vec![vec![8], vec![0xc], vec![0xd], vec![0xc], vec![0xd], vec![1]]
        );
    }

    #[test]
    fn sopen_helper_finally_clears_fopen_on_error() {
        let mut e = engine(&[UNLOCK_FHANDLE_ADDR]);
        e.mem.map(PIOINFO, 0x80);
        let blocks = e.mem.alloc(0x800);
        e.mem.set_u32(PIOINFO + 4, blocks);
        let result = e.mem.alloc(4);
        // Handle 0x23: block 1, entry 3; osfile at +4.
        e.mem.set_u32(result, 0x23);
        let osfile = blocks + 3 * 0x40 + 4;
        e.mem.set_u8(osfile, 0x81);
        sopen_helper_finally(&mut e, 0, 1, result);
        assert!(calls(&e, UNLOCK_FHANDLE_ADDR).is_empty());
        assert_eq!(e.mem.u8(osfile), 0x81);
        sopen_helper_finally(&mut e, 1, 0, result);
        assert_eq!(e.mem.u8(osfile), 0x81);
        sopen_helper_finally(&mut e, 1, 0x16, result);
        assert_eq!(e.mem.u8(osfile), 0x80);
        assert_eq!(calls(&e, UNLOCK_FHANDLE_ADDR), vec![vec![0x23], vec![0x23]]);
    }

    #[test]
    fn rand_steps_the_thread_seed_srand_sets() {
        let mut e = engine(&[]);
        let ptd = e.mem.alloc(0x100);
        answer(&mut e, GETPTD_ADDR, ptd);
        // The seed `_initptd` leaves: MSVC's sequence 41, 18467, 6334.
        e.mem.set_u32(ptd + 0x14, 1);
        let r: Vec<i32> = (0..3)
            .map(|_| e.call(0x00ec_adb8, &args![]).i32())
            .collect();
        assert_eq!(r, vec![41, 18467, 6334]);
        srand(&mut e, 1);
        assert_eq!(rand(&mut e), 41);
        // The state wraps at 32 bits.
        e.mem.set_u32(ptd + 0x14, 0xffff_ffff);
        let state = 0xffff_ffff_u32.wrapping_mul(0x343fd).wrapping_add(0x269ec3);
        assert_eq!(rand(&mut e), ((state >> 16) & 0x7fff) as i32);
        assert_eq!(e.mem.u32(ptd + 0x14), state);
    }

    #[test]
    fn srand_stores_the_seed_in_the_thread_data() {
        let mut e = engine(&[]);
        let ptd = e.mem.alloc(0x100);
        answer(&mut e, GETPTD_ADDR, ptd);
        srand(&mut e, 1234);
        assert_eq!(e.mem.u32(ptd + 0x14), 1234);
    }

    #[test]
    fn copy_constructor_iterator_constructs_each_and_runs_its_finally() {
        const CTOR: u32 = 0x0070_0000;
        let mut e = engine(&[ARRAY_UNWIND_ADDR]);
        e.register(CTOR, |_, _| Ret::default());
        eh_vector_copy_constructor_iterator(&mut e, 0x1000, 0x2000, 8, 3, CTOR, 0x77);
        assert_eq!(
            calls(&e, CTOR),
            vec![
                vec![0x1000, 0x2000],
                vec![0x1008, 0x2008],
                vec![0x1010, 0x2010]
            ]
        );
        assert_eq!(
            calls(&e, COPY_ITERATOR_FINALLY_ADDR),
            vec![vec![1, 0x1018, 8, 3, 0x77]]
        );
        assert!(calls(&e, ARRAY_UNWIND_ADDR).is_empty());
        // A count of 0 or below constructs nothing.
        e.call_log = Some(vec![]);
        eh_vector_copy_constructor_iterator(&mut e, 0x1000, 0x2000, 8, -1, CTOR, 0x77);
        assert!(calls(&e, CTOR).is_empty());
        // The finally block unwinds only an unfinished loop.
        eh_vector_copy_constructor_finally(&mut e, 0, 0x1010, 8, 2, 0x77);
        assert_eq!(calls(&e, ARRAY_UNWIND_ADDR), vec![vec![0x1010, 8, 2, 0x77]]);
    }

    #[test]
    fn strupr_s_and_exit_pass_their_words_on() {
        let mut e = engine(&[DOEXIT_ADDR]);
        answer(&mut e, STRUPR_S_L_ADDR, 0x22);
        assert_eq!(strupr_s(&mut e, 0x100, 9), 0x22);
        assert_eq!(calls(&e, STRUPR_S_L_ADDR), vec![vec![0x100, 9, 0]]);
        exit(&mut e, 3);
        assert_eq!(calls(&e, DOEXIT_ADDR), vec![vec![3, 0, 0]]);
    }

    #[test]
    fn cor_exit_process_calls_the_runtime_when_found() {
        const COR: u32 = 0x0070_0000;
        let mut e = engine(&[]);
        e.register(COR, |_, _| Ret::default());
        answer(&mut e, IMPORT_GET_MODULE_HANDLE_W, 0);
        crt_cor_exit_process(&mut e, 5);
        assert!(calls(&e, IMPORT_GET_PROC_ADDRESS).is_empty());
        answer(&mut e, IMPORT_GET_MODULE_HANDLE_W, 0x1000_0000);
        answer(&mut e, IMPORT_GET_PROC_ADDRESS, 0);
        crt_cor_exit_process(&mut e, 5);
        assert!(calls(&e, COR).is_empty());
        answer(&mut e, IMPORT_GET_PROC_ADDRESS, COR);
        crt_cor_exit_process(&mut e, 5);
        assert_eq!(calls(&e, IMPORT_GET_MODULE_HANDLE_W)[0], vec![TEXT_MSCOREE]);
        assert_eq!(
            calls(&e, IMPORT_GET_PROC_ADDRESS)[0],
            vec![0x1000_0000, TEXT_COR_EXIT_PROCESS]
        );
        assert_eq!(calls(&e, COR), vec![vec![5]]);
    }

    #[test]
    fn initterm_calls_nonzero_entries_and_initterm_e_stops_at_an_error() {
        const A: u32 = 0x0070_0000;
        const B: u32 = 0x0070_0010;
        let mut e = engine(&[]);
        answer(&mut e, A, 0);
        answer(&mut e, B, 7);
        let table = e.mem.alloc(16);
        for (i, f) in [A, 0, B, A].iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *f);
        }
        initterm(&mut e, table, table + 16);
        assert_eq!(order(&e, &[A, B]), vec![A, B, A]);
        e.call_log = Some(vec![]);
        assert_eq!(initterm_e(&mut e, table, table + 16), 7);
        assert_eq!(order(&e, &[A, B]), vec![A, B]);
        e.call_log = Some(vec![]);
        assert_eq!(initterm_e(&mut e, table, table + 4), 0);
        assert_eq!(order(&e, &[A, B]), vec![A]);
        // An empty or inverted range calls nothing.
        e.call_log = Some(vec![]);
        initterm(&mut e, table + 8, table);
        assert_eq!(initterm_e(&mut e, table + 8, table + 8), 0);
        assert!(order(&e, &[A, B]).is_empty());
        // Registered with EAX (the start) first.
        e.call(INITTERM_ADDR, &args![table, table + 4]);
        assert_eq!(order(&e, &[A, B]), vec![A]);
    }

    #[test]
    fn cinit_runs_the_tables_in_the_exes_order() {
        const FP: u32 = 0x0070_0000;
        const TLS: u32 = 0x0070_0020;
        let watched = [
            IS_NONWRITABLE_ADDR,
            FP,
            INITP_MISC_CFLTCVT_TAB_ADDR,
            INITTERM_E_ADDR,
            ATEXIT_ADDR,
            INITTERM_ADDR,
        ];
        let mut e = Engine::new();
        for addr in watched {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(TLS, |_, _| Ret::default());
        answer(&mut e, IS_NONWRITABLE_ADDR, 1);
        e.mem.map(FPINIT_POINTER, 4);
        e.mem.map(TLS_CALLBACK, 4);
        e.mem.set_u32(FPINIT_POINTER, FP);
        e.call_log = Some(vec![]);
        assert_eq!(cinit(&mut e, 1), 0);
        assert_eq!(order(&e, &watched), watched.to_vec());
        assert_eq!(calls(&e, FP), vec![vec![1]]);
        assert_eq!(calls(&e, INITTERM_E_ADDR), vec![vec![C_INIT.0, C_INIT.1]]);
        assert_eq!(calls(&e, ATEXIT_ADDR), vec![vec![CPP_TERMINATORS_ADDR]]);
        assert_eq!(calls(&e, INITTERM_ADDR), vec![vec![CPP_INIT.0, CPP_INIT.1]]);
        // A C initializer's error is answered before the C++ table; a writable pointer is
        // not called.
        answer(&mut e, INITTERM_E_ADDR, 0x18);
        answer(&mut e, IS_NONWRITABLE_ADDR, 0);
        e.call_log = Some(vec![]);
        assert_eq!(cinit(&mut e, 0), 0x18);
        assert!(calls(&e, FP).is_empty());
        assert!(calls(&e, INITTERM_ADDR).is_empty());
        // The TLS callback gets (0, 2, 0).
        e.mem.set_u32(TLS_CALLBACK, TLS);
        e.mem.set_u32(FPINIT_POINTER, 0);
        answer(&mut e, INITTERM_E_ADDR, 0);
        answer(&mut e, IS_NONWRITABLE_ADDR, 1);
        e.call_log = Some(vec![]);
        assert_eq!(cinit(&mut e, 0), 0);
        assert_eq!(calls(&e, IS_NONWRITABLE_ADDR), vec![vec![TLS_CALLBACK]]);
        assert_eq!(calls(&e, TLS), vec![vec![0, 2, 0]]);
    }

    #[test]
    fn end_stdio_flushes_closes_and_frees() {
        let watched = [FLUSHALL_ADDR, FCLOSEALL_ADDR, FREE_ADDR];
        let mut e = engine(&watched);
        e.mem.map(EXIT_BYTE, 1);
        e.mem.map(PIOB, 4);
        e.mem.set_u32(PIOB, 0x1234);
        end_stdio(&mut e);
        assert_eq!(order(&e, &watched), vec![FLUSHALL_ADDR, FREE_ADDR]);
        e.mem.set_u8(EXIT_BYTE, 1);
        e.call_log = Some(vec![]);
        end_stdio(&mut e);
        assert_eq!(order(&e, &watched), watched.to_vec());
        assert_eq!(calls(&e, FREE_ADDR)[0], vec![0x1234]);
    }

    #[test]
    fn locale_accessors_refresh_a_stale_thread_locale() {
        let mut e = engine(&[]);
        e.mem.map(PTLOCINFO, 4);
        e.mem.map(GLOBAL_LOCALE_STATUS, 4);
        let ptd = e.mem.alloc(0x100);
        let global = e.mem.alloc(0x100);
        let own = e.mem.alloc(0x100);
        let fresh = e.mem.alloc(0x100);
        e.mem.set_u32(global + 0xc8, 0xaaaa);
        e.mem.set_u32(own + 0xc8, 0xbbbb);
        e.mem.set_u32(fresh + 0xc8, 0xcccc);
        e.mem.set_u32(PTLOCINFO, global);
        answer(&mut e, GETPTD_ADDR, ptd);
        answer(&mut e, UPDATETLOCINFO_ADDR, fresh);
        // The global locale: read as it is.
        e.mem.set_u32(ptd + 0x6c, global);
        assert_eq!(pctype_func(&mut e), 0xaaaa);
        assert_eq!(lc_handle_func(&mut e), global + 0xc);
        assert!(calls(&e, UPDATETLOCINFO_ADDR).is_empty());
        // Another one with an own-locale bit: kept.
        e.mem.set_u32(ptd + 0x6c, own);
        e.mem.set_u32(GLOBAL_LOCALE_STATUS, 2);
        e.mem.set_u32(ptd + 0x70, 2);
        assert_eq!(pctype_func(&mut e), 0xbbbb);
        // Without it: refreshed.
        e.mem.set_u32(ptd + 0x70, 1);
        assert_eq!(pctype_func(&mut e), 0xcccc);
        assert_eq!(lc_handle_func(&mut e), fresh + 0xc);
        assert_eq!(calls(&e, UPDATETLOCINFO_ADDR).len(), 2);
    }

    #[test]
    fn realloc_crt_retries_with_growing_sleeps() {
        let mut e = engine(&[IMPORT_SLEEP]);
        e.mem.map(MAX_WAIT, 4);
        answer(&mut e, REALLOC_ADDR, 0);
        // No wait limit: one try.
        assert_eq!(realloc_crt(&mut e, 0x10, 8), 0);
        assert_eq!(calls(&e, REALLOC_ADDR).len(), 1);
        assert!(calls(&e, IMPORT_SLEEP).is_empty());
        // A limit of 2500 ms: sleeps 0, 1000, 2000, then 3000 passes it.
        e.mem.set_u32(MAX_WAIT, 2500);
        e.call_log = Some(vec![]);
        assert_eq!(realloc_crt(&mut e, 0x10, 8), 0);
        assert_eq!(
            calls(&e, IMPORT_SLEEP),
            vec![vec![0], vec![1000], vec![2000]]
        );
        assert_eq!(calls(&e, REALLOC_ADDR).len(), 3);
        // A size of 0, and a success, answer at once.
        e.call_log = Some(vec![]);
        assert_eq!(realloc_crt(&mut e, 0x10, 0), 0);
        answer(&mut e, REALLOC_ADDR, 0x44);
        assert_eq!(realloc_crt(&mut e, 0x10, 8), 0x44);
        assert!(calls(&e, IMPORT_SLEEP).is_empty());
    }

    #[test]
    fn clear_watson_word_clears_it() {
        let mut e = engine(&[]);
        e.mem.map(WATSON_WORD, 4);
        e.mem.set_u32(WATSON_WORD, 9);
        clear_watson_word(&mut e, 2);
        assert_eq!(e.mem.u32(WATSON_WORD), 0);
    }

    fn text(e: &mut Engine, t: &[u8]) -> u32 {
        let at = e.mem.alloc(t.len() as u32 + 1);
        e.mem.write(at, t);
        e.mem.set_u8(at + t.len() as u32, 0);
        at
    }

    #[test]
    fn ascii_strnicmp_folds_ascii_letters_only() {
        type Case<'a> = (&'a [u8], &'a [u8], u32, i32);
        let mut e = engine(&[]);
        let cases: [Case; 10] = [
            (b"Hello", b"hELLO", 5, 0),
            (b"abc", b"abd", 3, -1),
            (b"abd", b"ABC", 3, 1),
            (b"abc", b"abd", 2, 0),
            (b"abc", b"ab", 9, 1),
            (b"ab", b"abc", 9, -1),
            (b"", b"", 4, 0),
            (b"x", b"y", 0, 0),
            // '[' (0x5b) is not folded: above 'Z' but below 'a'.
            (b"[", b"a", 1, -1),
            (b"\xc4", b"\xe4", 1, -1),
        ];
        for (a, b, n, want) in cases {
            let (pa, pb) = (text(&mut e, a), text(&mut e, b));
            assert_eq!(ascii_strnicmp(&mut e, pa, pb, n), want, "{a:?} {b:?} {n}");
        }
    }

    /// A minimal PE image at `base`: `MZ`, the PE header at +0x80 with two sections
    /// (read-only at RVA 0x1000 of 0x1000 bytes, writable at 0x2000 of 0x800).
    fn image(e: &mut Engine, base: u32) {
        e.mem.map(base, 0x400);
        e.mem.set_u16(base, 0x5a4d);
        e.mem.set_u32(base + 0x3c, 0x80);
        let pe = base + 0x80;
        e.mem.set_u32(pe, 0x4550);
        e.mem.set_u16(pe + 6, 2);
        e.mem.set_u16(pe + 0x14, 0xe0);
        e.mem.set_u16(pe + 0x18, 0x10b);
        let sections = pe + 0x18 + 0xe0;
        let table: [Section; 2] = [(0x1000, 0x1000, 0x6000_0020), (0x2000, 0x800, 0xc000_0040)];
        for (i, (rva, size, flags)) in table.iter().enumerate() {
            let s = sections + 0x28 * i as u32;
            e.mem.set_u32(s + 8, *size);
            e.mem.set_u32(s + 0xc, *rva);
            e.mem.set_u32(s + 0x24, *flags);
        }
    }

    #[test]
    fn image_checks_find_the_section_and_its_write_bit() {
        let mut e = engine(&[]);
        image(&mut e, IMAGE_BASE);
        assert_eq!(validate_image_base(&mut e, IMAGE_BASE), 1);
        let pe = IMAGE_BASE + 0x80;
        let first = pe + 0x18 + 0xe0;
        assert_eq!(find_pe_section(&mut e, IMAGE_BASE, 0x1000), first);
        assert_eq!(find_pe_section(&mut e, IMAGE_BASE, 0x1fff), first);
        assert_eq!(find_pe_section(&mut e, IMAGE_BASE, 0x2000), first + 0x28);
        assert_eq!(find_pe_section(&mut e, IMAGE_BASE, 0x2800), 0);
        assert_eq!(find_pe_section(&mut e, IMAGE_BASE, 0x10), 0);
        assert_eq!(
            is_nonwritable_in_current_image(&mut e, IMAGE_BASE + 0x1234),
            1
        );
        assert_eq!(
            is_nonwritable_in_current_image(&mut e, IMAGE_BASE + 0x2004),
            0
        );
        assert_eq!(
            is_nonwritable_in_current_image(&mut e, IMAGE_BASE + 0x3000),
            0
        );
        // Another magic, signature or `MZ`: not an image.
        e.mem.set_u16(pe + 0x18, 0x20b);
        assert_eq!(validate_image_base(&mut e, IMAGE_BASE), 0);
        assert_eq!(
            is_nonwritable_in_current_image(&mut e, IMAGE_BASE + 0x1234),
            0
        );
        e.mem.set_u16(pe + 0x18, 0x10b);
        e.mem.set_u32(pe, 0x4551);
        assert_eq!(validate_image_base(&mut e, IMAGE_BASE), 0);
        e.mem.set_u16(IMAGE_BASE, 0x5a4e);
        assert_eq!(validate_image_base(&mut e, IMAGE_BASE), 0);
    }

    #[test]
    fn cpp_terminator_runner_calls_nothing_for_the_empty_table() {
        let mut e = engine(&[]);
        run_cpp_terminators(&mut e);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn glue_functions_are_registered() {
        let table = funcs();
        for addr in [
            0x00ec_5ec0u32,
            0x00ec_6522,
            0x00ec_7595,
            0x00ec_75b1,
            0x00ec_913a,
            0x00ec_9886,
            0x00ec_9a3d,
            0x00ec_ada6,
            0x00ec_aebb,
            0x00ec_b0fe,
            0x00ec_bc5b,
            0x00ec_cc7e,
            0x00ec_ccd3,
            0x00ec_cd14,
            0x00ec_ceb0,
            0x00ec_cec5,
            0x00ec_dffb,
            0x00ec_f7ac,
            0x00ed_08c8,
            0x00ed_0cb9,
            0x00ed_0d70,
            0x00ed_1312,
            0x00ed_131e,
            0x00ed_3d7a,
            0x00ed_5bae,
            0x00ed_5bc0,
            0x00ed_81fd,
            0x00ed_8630,
            0x00ed_c815,
            0x00ee_213b,
            0x00ee_38da,
            0x00ed_dcc0,
        ] {
            assert!(table.iter().any(|f| f.0 == addr), "{addr:08x}");
        }
    }
}

#[cfg(test)]
mod setlocale_tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    /// The calls among `watched`, in order.
    fn among(e: &Engine, watched: &[u32]) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| watched.contains(&c.0))
            .cloned()
            .collect()
    }

    const WATCHED: [u32; 12] = [
        LOCK_ADDR,
        UNLOCK_ADDR,
        COPYTLOCINFO_NOLOCK,
        SETLOCALE_NOLOCK,
        STRCMP_ADDR,
        UPDATETLOCINFO_EX_NOLOCK,
        REMOVELOCALEREF,
        FREETLOCINFO,
        MEMCPY_ADDR,
        SYNC_LEGACY_VARIABLES,
        SETLOCALE_FINALLY_ADDR,
        CALLOC_CRT_ADDR,
    ];

    /// Every callee doubled (the new info `0x0700_0000`, `__setlocale_nolock` answering
    /// `answer`, `strcmp` answering `compare`), the thread data at `0x0800_0000` (its info
    /// `0x0900_0000`) and the globals mapped.
    fn engine(answer: u32, compare: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let ptd = 0x0800_0000;
        e.map(ptd, 0x100);
        e.mem.set_u32(ptd + 0x6c, 0x0900_0000);
        e.map(PTLOCINFO, 4);
        e.mem.set_u32(PTLOCINFO, 0x0a00_0000);
        e.map(GLOBAL_LOCALE_STATUS, 4);
        e.map(LOCALE_CHANGED, 4);
        e.register_double(GETPTD_ADDR, move |_, _| ptd.into_ret());
        e.register(CALLOC_CRT_ADDR, |_, _| 0x0700_0000u32.into_ret());
        e.register_double(SETLOCALE_NOLOCK, move |_, _| answer.into_ret());
        e.register_double(STRCMP_ADDR, move |_, _| compare.into_ret());
        for addr in [
            UPDATETLOCINFO_ADDR,
            LOCK_ADDR,
            UNLOCK_ADDR,
            COPYTLOCINFO_NOLOCK,
            UPDATETLOCINFO_EX_NOLOCK,
            REMOVELOCALEREF,
            FREETLOCINFO,
            MEMCPY_ADDR,
            SYNC_LEGACY_VARIABLES,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, ptd)
    }

    #[test]
    fn setlocale_makes_the_new_info_the_threads_and_the_globals() {
        let (mut e, ptd) = engine(0x1234, 1);
        let locale = e.mem.alloc(8);
        assert_eq!(setlocale(&mut e, 0, locale), 0x1234);
        let info = 0x0700_0000;
        assert_eq!(
            among(&e, &WATCHED),
            vec![
                (CALLOC_CRT_ADDR, vec![0xd8, 1]),
                (LOCK_ADDR, vec![0xc]),
                (COPYTLOCINFO_NOLOCK, vec![info, 0x0900_0000]),
                (UNLOCK_ADDR, vec![0xc]),
                (SETLOCALE_NOLOCK, vec![info, locale, 0]),
                (STRCMP_ADDR, vec![locale, C_LOCALE_NAME]),
                (LOCK_ADDR, vec![0xc]),
                (UPDATETLOCINFO_EX_NOLOCK, vec![ptd + 0x6c, info]),
                (REMOVELOCALEREF, vec![info]),
                (UPDATETLOCINFO_EX_NOLOCK, vec![PTLOCINFO, 0x0900_0000]),
                (MEMCPY_ADDR, vec![LEGACY_HANDLES, 0x0a00_000c, 0x18]),
                (SYNC_LEGACY_VARIABLES, vec![]),
                (UNLOCK_ADDR, vec![0xc]),
                (SETLOCALE_FINALLY_ADDR, vec![ptd]),
            ]
        );
        assert_eq!(e.mem.u32(LOCALE_CHANGED), 1);
        // Bit 0x10 set on the way in and cleared by the finally block.
        assert_eq!(e.mem.u32(ptd + 0x70), 0);
    }

    #[test]
    fn setlocale_keeps_the_global_for_an_own_locale_and_c() {
        let (mut e, ptd) = engine(0x1234, 0);
        e.mem.set_u32(ptd + 0x70, 2);
        let locale = e.mem.alloc(8);
        assert_eq!(setlocale(&mut e, 5, locale), 0x1234);
        let calls = among(&e, &WATCHED);
        assert_eq!(
            calls
                .iter()
                .filter(|c| c.0 == UPDATETLOCINFO_EX_NOLOCK)
                .count(),
            1
        );
        assert!(calls.iter().all(|c| c.0 != MEMCPY_ADDR));
        assert_eq!(e.mem.u32(LOCALE_CHANGED), 0);
        assert_eq!(e.mem.u32(ptd + 0x70), 2);
        // A null locale skips the compare; the global status bit 0 also keeps the global.
        let (mut e, _) = engine(0x1234, 1);
        e.mem.set_u8(GLOBAL_LOCALE_STATUS, 1);
        assert_eq!(setlocale(&mut e, 1, 0), 0x1234);
        let calls = among(&e, &WATCHED);
        assert!(calls
            .iter()
            .all(|c| c.0 != STRCMP_ADDR && c.0 != MEMCPY_ADDR));
    }

    #[test]
    fn setlocale_frees_the_info_when_the_category_is_not_set() {
        let (mut e, ptd) = engine(0, 1);
        assert_eq!(setlocale(&mut e, 2, 0), 0);
        let info = 0x0700_0000;
        let calls = among(&e, &WATCHED);
        assert_eq!(
            calls[calls.len() - 3..].to_vec(),
            vec![
                (REMOVELOCALEREF, vec![info]),
                (FREETLOCINFO, vec![info]),
                (SETLOCALE_FINALLY_ADDR, vec![ptd]),
            ]
        );
        // No memory: no lock, only the finally block.
        let (mut e, ptd) = engine(0x1234, 1);
        e.register(CALLOC_CRT_ADDR, |_, _| 0u32.into_ret());
        assert_eq!(setlocale(&mut e, 2, 0), 0);
        assert_eq!(
            among(&e, &WATCHED),
            vec![
                (CALLOC_CRT_ADDR, vec![0xd8, 1]),
                (SETLOCALE_FINALLY_ADDR, vec![ptd])
            ]
        );
    }

    #[test]
    fn setlocale_refuses_a_category_above_5() {
        let (mut e, _) = engine(0x1234, 1);
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO_ADDR, move |_, _| slot.into_ret());
        e.register(INVALID_PARAMETER_ADDR, |_, _| Ret::default());
        assert_eq!(setlocale(&mut e, 6, 0), 0);
        assert_eq!(e.mem.u32(slot), 0x16);
        assert!(among(&e, &WATCHED).is_empty());
    }

    #[test]
    fn setlocale_finally_clears_bit_0x10() {
        let mut e = Engine::new();
        let ptd = e.mem.alloc(0x80);
        e.mem.set_u32(ptd + 0x70, 0xff);
        setlocale_finally(&mut e, ptd);
        assert_eq!(e.mem.u32(ptd + 0x70), 0xef);
    }
}
