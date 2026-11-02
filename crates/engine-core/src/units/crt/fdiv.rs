//! The C runtime's Pentium `FDIV`/`FPREM` workaround (MSVC 2008 LIBCMT, statically linked into
//! FalloutNV.exe 1.4.0.525), translated from the exe: the division routine (`00ed62f0`) and the
//! register-form dispatcher inside it (`00ed6407`, a jump through the table at `011c259e` to
//! one of 64 fragments, `00ed6414` to `00ed6892`), the register entries `00ed68a6`, `00ed68b9`,
//! `00ed6acc` and `00ed6ae1`, the memory-operand entries (`00ed68cc` to `00ed6a98`), and the
//! partial remainder (`00ed6cfc` with its body `00ed6af6`). `exp`'s and `fmod`'s math handlers
//! (`math.rs`) call `00ed6ae1` and `00ed6cfc` instead of `FDIVRP` and `FPREM` when the word at
//! `0126fd94` (the Pentium flaw flag) is 1.
//!
//! The routines keep their operands in the x87 registers ([`Fpu`], extended values, every
//! instruction through `crate::x87`'s register arithmetic under the running thread's control
//! word) and in their stack frames, which are memory here as in the exe (`Engine::with_stack`):
//! the tests on an operand's words read the frame as the exe reads it. `FNSTCW`/`FLDCW` go
//! through the thread's control word (`x87_control_word`, `set_x87_control_word`). The routines
//! call each other directly with the register stack; the ones with a uniform form are also
//! registered (register inputs first: `EAX`, then `ST0`, `ST1`, ... as doubles, then the stack
//! words; the result `ST0` rounded to a double with the control word left, and for `00ed6cfc`
//! the status word's condition codes in `EAX`).
//!
//! The 16 fragments that only give the frame back and raise `INT 6` (the invalid-opcode
//! exception, which the engine cannot dispatch) are panic-only: they stop the run through an
//! unmarked helper and carry no translation marker (docs/ENGINE_CRATE.md).
//!
//! Not modelled: the x87 status word, which the engine does not keep. The stack-top field the
//! memory-operand entries test is taken from the register count of the model, and the
//! condition codes `00ed6cfc` leaves are the ones its `FPREM` sets (each function says so).

use super::math::{ext_of_double, load_extended, Fpu};
use super::{set_x87_control_word, x87_control_word};
use crate::prelude::*;
use crate::x87::{
    x87_fabs, x87_fchs, x87_fcom, x87_fdiv, x87_fmul, x87_fprem, x87_fsub, X87_STATUS_C0,
};

/// The divisor test table (`011c2540`, 16 bytes, by bits 59 to 62 of the divisor's significand:
/// a byte not 0 marks a divisor the routine scales first), the two scales (`011c2550`,
/// `011c2554`, floats: the second when the dividend's exponent is 1) and the scale of a
/// denormal divisor (`011c2558`, a float).
const FDIV_TABLE: u32 = 0x011c_2540;
const FDIV_SCALE: u32 = 0x011c_2550;
const FDIV_SCALE_EXPONENT_ONE: u32 = 0x011c_2554;
const FDIV_DENORMAL_SCALE: u32 = 0x011c_2558;
/// The register-form dispatcher's jump table (`011c259e`, 64 fragment addresses).
const FDIV_R_TABLE: u32 = 0x011c_259e;
/// The partial remainder's data: its divisor test table (`011c255c`, 16 bytes), the scale of
/// a denormal divisor (`011c2574`, a double), the scale back (`011c257c`, a double), the
/// halving factor (`011c258c`, a double) and the extended operand of its last `FPREM`
/// (`011c2594`).
const FPREM_TABLE: u32 = 0x011c_255c;
const FPREM_DENORMAL_SCALE: u32 = 0x011c_2574;
const FPREM_UNSCALE: u32 = 0x011c_257c;
const FPREM_HALF: u32 = 0x011c_258c;
const FPREM_OPERAND: u32 = 0x011c_2594;
/// Status word bit C1 (`crate::x87` names C0, C2 and C3).
const X87_STATUS_C1: u16 = 0x0200;
/// The condition codes C0, C1, C2 and C3: the part of the status word the model answers (the
/// exception flags and the stack top the instruction leaves are not kept).
const CONDITION_CODES: u16 = 0x4700;

/// `FLD tbyte` of the slot at `at`.
fn load(e: &Engine, at: u32) -> X87Ext {
    load_extended(e, at)
}

/// `FSTP tbyte` into the slot at `at` (the value the caller popped).
fn store(e: &mut Engine, at: u32, value: X87Ext) {
    e.mem.write(at, &value.0);
}

/// `FNSTCW word [at]`.
fn fnstcw(e: &mut Engine, at: u32) {
    let word = x87_control_word(e);
    e.mem.set_u16(at, word);
}

/// `FLDCW word [at]`.
fn fldcw(e: &mut Engine, at: u32) {
    let word = e.mem.u16(at);
    set_x87_control_word(e, word);
}

/// A float memory operand of an arithmetic instruction (`FMUL dword ptr [..]` and the like),
/// widened exactly (a signalling NaN still signalling).
fn float_operand(bits: u32) -> X87Ext {
    if bits & 0x7f80_0000 != 0x7f80_0000 {
        return X87Ext::load32(bits);
    }
    let sign_exponent = ((bits >> 16) as u16 & 0x8000) | 0x7fff;
    X87Ext::from_parts(
        sign_exponent,
        (1 << 63) | (u64::from(bits & 0x7f_ffff) << 40),
    )
}

/// A double memory operand of an arithmetic instruction, widened exactly.
fn double_operand(e: &Engine, at: u32) -> X87Ext {
    ext_of_double(e.mem.u64(at))
}

/// `FDIVP` (`DE F9`): `ST1 = ST1 / ST0`, then pop.
fn fdivp(fpu: &mut Fpu) {
    let divisor = fpu.pop();
    let quotient = x87_fdiv(fpu.st(0), divisor);
    fpu.set(0, quotient);
}

/// `FPREM` on `ST0` by `ST1`: `ST0` replaced, the condition codes answered.
fn fprem(fpu: &mut Fpu) -> u16 {
    let (remainder, status) = x87_fprem(fpu.st(0), fpu.st(1));
    fpu.set(0, remainder);
    status & CONDITION_CODES
}

/// The stack-top field (`TOP`, bits 11 to 13 of the status word, as `FNSTSW AX; AND EAX,
/// 0x3800` leaves it) as the model gives it: the engine keeps no status word, so it is taken
/// from the registers the model holds, counted down from an empty stack's 0 (8 registers: 0).
fn stack_top(fpu: &Fpu) -> u32 {
    (8u32.wrapping_sub(fpu.st.len() as u32) & 7) << 11
}

/// A uniform form's register words (`ST0`, `ST1`, ... as doubles, two words each) on a
/// register stack.
fn registers(words: &[u32]) -> Fpu {
    let mut fpu = Fpu::default();
    for pair in words.chunks(2).rev() {
        let bits = u64::from(pair[0]) | u64::from(pair[1]) << 32;
        fpu.push(X87Ext::load64(f64::from_bits(bits)));
    }
    fpu
}

/// A uniform form's answer: `ST0` rounded to a double.
fn st0_answer(fpu: &Fpu) -> Ret {
    Ret {
        st0: fpu.st(0).store64(),
        ..Ret::default()
    }
}

// ---- The division routine ----------------------------------------------------------------------

// Translated from 00ed62f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The division routine (no Ghidra name; `CALL`ed by every entry with its operands in the
/// caller's frame, here `frame`, the caller's `ESP`: the divisor's extended value at +0, the
/// dividend's at +0xc, +0x18 and +0x1c the control-word slots). Loads the dividend and the
/// divisor (`ST0`) and divides (`FDIVP`, `ST0` = the quotient) unless the divisor is one the
/// flaw can hit:
///
/// - a divisor with its integer bit set, significand bits 56 to 58 all set and bits 59 to 62
///   (`(high << 1) >> 28` after the test) marking a byte in the table `011c2540`, with an
///   exponent neither 0 nor 0x7fff: under the caller's control word with every exception
///   masked, 64-bit precision and rounding to nearest (`(word | 0x33f) & 0xf3ff`, kept at
///   +0x1c) both operands are multiplied by the float at `011c2550` (`011c2554` when the
///   dividend's exponent is 1), the caller's word comes back and they are divided;
/// - a denormal divisor (integer bit clear, exponent 0, significand not 0) with a dividend
///   whose integer bit is clear and exponent 0, or whose integer bit is set and exponent is
///   neither 0 nor 0x7fff: under that word the divisor times the float at `011c2558` replaces
///   the divisor in the frame (the registers keep the two operands as they were), the
///   caller's word comes back and the tests run again on the frame's divisor (`JMP
///   00ed62f8`).
///
/// Every other divisor is divided at once.
pub(super) fn fdiv_routine(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    // FLD tbyte [ESP+0x10]; FLD tbyte [ESP+4].
    fpu.push(load(e, frame + 0xc));
    fpu.push(load(e, frame));
    loop {
        // 00ed62f8: MOV EAX, [ESP+8]; ADD EAX, EAX; JNC 00ed638a.
        let high = e.mem.u32(frame + 4);
        let (doubled, integer) = high.overflowing_add(high);
        if integer {
            // XOR EAX, 0xe000000; TEST EAX, 0xe000000; JZ 00ed6313; SHR EAX, 0x1c.
            let bits = doubled ^ 0x0e00_0000;
            if bits & 0x0e00_0000 != 0 || e.mem.u8(FDIV_TABLE + (bits >> 28)) == 0 {
                fdivp(fpu);
                return;
            }
            // 00ed6322: the divisor's exponent neither 0 nor 0x7fff.
            let exponent = e.mem.u32(frame + 8) & 0x7fff;
            if exponent == 0 || exponent == 0x7fff {
                fdivp(fpu);
                return;
            }
            guarded_word(e, frame);
            let scale = if e.mem.u32(frame + 0x14) & 0x7fff == 1 {
                FDIV_SCALE_EXPONENT_ONE
            } else {
                FDIV_SCALE
            };
            let scale = float_operand(e.mem.u32(scale));
            // FMUL; FXCH; FMUL; FXCH; FLDCW [ESP+0x1c]; FDIVP.
            fpu.set(0, x87_fmul(fpu.st(0), scale));
            fpu.fxch(1);
            fpu.set(0, x87_fmul(fpu.st(0), scale));
            fpu.fxch(1);
            fldcw(e, frame + 0x18);
            fdivp(fpu);
            return;
        }
        // 00ed638a: a zero significand, or an exponent not 0 (an unnormal), divides at once.
        if e.mem.u32(frame) | e.mem.u32(frame + 4) == 0 || e.mem.u32(frame + 8) & 0x7fff != 0 {
            fdivp(fpu);
            return;
        }
        guarded_word(e, frame);
        // 00ed63bc: the dividend's exponent and integer bit.
        let exponent = e.mem.u32(frame + 0x14) & 0x7fff;
        let dividend_integer = e.mem.u32(frame + 0x10) & 0x8000_0000 != 0;
        let rescale = match exponent {
            0 => !dividend_integer,
            0x7fff => false,
            _ => dividend_integer,
        };
        if !rescale {
            // 00ed6400: FLDCW [ESP+0x1c]; FDIVP.
            fldcw(e, frame + 0x18);
            fdivp(fpu);
            return;
        }
        // 00ed63e0: FXCH; FSTP ST0; FLD ST0; FMUL dword [011c2558]; FSTP tbyte [ESP+4];
        // FLD tbyte [ESP+0x10]; FXCH; FLDCW [ESP+0x1c]; JMP 00ed62f8.
        fpu.fxch(1);
        fpu.pop();
        let scaled = x87_fmul(fpu.st(0), float_operand(e.mem.u32(FDIV_DENORMAL_SCALE)));
        store(e, frame, scaled);
        fpu.push(load(e, frame + 0xc));
        fpu.fxch(1);
        fldcw(e, frame + 0x18);
    }
}

/// The routine's control word (`00ed6334`, `00ed63a2`): `FNSTCW [ESP+0x1c]`, the dword there
/// `OR 0x33f`, `AND 0xf3ff` (every exception masked, 64-bit precision, round to nearest) stored
/// at `[ESP+0x20]` and loaded (`FLDCW`): the caller's frame slots +0x18 and +0x1c.
fn guarded_word(e: &mut Engine, frame: u32) {
    fnstcw(e, frame + 0x18);
    let word = (e.mem.u32(frame + 0x18) | 0x33f) & 0xf3ff;
    e.mem.set_u32(frame + 0x1c, word);
    fldcw(e, frame + 0x1c);
}

/// The registered form of `00ed62f0`: the eight words above the return address (the divisor's
/// and the dividend's extended values, three words each, and the two control-word slots) on a
/// frame of its own; the quotient rounded to a double.
fn fdiv_routine_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let mut fpu = Fpu::default();
    e.with_stack(0x20, |e, frame| {
        let frame = frame.addr();
        for (i, word) in words.iter().take(8).enumerate() {
            e.mem.set_u32(frame + 4 * i as u32, *word);
        }
        fdiv_routine(e, &mut fpu, frame);
    });
    st0_answer(&fpu)
}

// ---- The register forms ------------------------------------------------------------------------

/// A fragment of the register-form dispatcher: the register stack and the dispatcher's frame.
type Fragment = fn(&mut Engine, &mut Fpu, u32);

// Translated from 00ed6407 (disassembled, FalloutNV.exe 1.4.0.525)
/// The register-form dispatcher (inside `00ed62f0`'s body; no Ghidra name; `EAX` the selector:
/// bits 0 to 2 the form, bits 3 to 5 the register `i`): a 0x2c-byte frame (`SUB ESP, 0x2c`),
/// then a jump through the table at `011c259e` by `EAX & 0x3f`. Forms 0 to 7 of a register:
/// `ST0 = ST0 / STi`, an invalid opcode, `ST0 = STi / ST0`, an invalid opcode, `STi = STi /
/// ST0`, the same popped, `STi = ST0 / STi`, the same popped; register 0's are plain
/// instructions on `ST0`. Each fragment gives the frame back and returns.
pub(super) fn fdiv_registers(e: &mut Engine, fpu: &mut Fpu, selector: u32) {
    e.with_stack(0x2c, |e, frame| {
        let target = e.mem.u32(FDIV_R_TABLE + (selector & 0x3f) * 4);
        let Some(&(_, fragment)) = FRAGMENTS.iter().find(|(addr, _)| *addr == target) else {
            panic!("{target:08x}: the FDIV dispatcher's table names no fragment there");
        };
        fragment(e, fpu, frame.addr());
    })
}

/// The registered form of `00ed6407`: `EAX`, then the registers `ST0`, `ST1`, ... as doubles
/// (as many as the words give); `ST0` afterwards, rounded to a double.
fn fdiv_registers_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let mut fpu = registers(&words[1..]);
    fdiv_registers(e, &mut fpu, words[0]);
    st0_answer(&fpu)
}

/// The fragments' sequences by form, for register `i` (1 to 7) in the dispatcher's frame
/// (`frame`: the dividend slot +0xc, the divisor slot +0, the saved register +0x20). The exe
/// writes register 1's without the general form's `FXCH ST0`s, which change nothing.
///
/// Form 0, `ST0 = ST0 / STi`: `FSTP tbyte [ESP+0xc]; FXCH ST(i-1); FLD ST0; FSTP tbyte [ESP];
/// FSTP tbyte [ESP+0x20]; CALL 00ed62f0; FXCH ST(i-1); FLD tbyte [ESP+0x20]; FXCH ST(i)`.
fn divide_st0_by(e: &mut Engine, fpu: &mut Fpu, frame: u32, i: usize) {
    store(e, frame + 0xc, fpu.pop());
    fpu.fxch(i - 1);
    fpu.push(fpu.st(0));
    store(e, frame, fpu.pop());
    store(e, frame + 0x20, fpu.pop());
    fdiv_routine(e, fpu, frame);
    fpu.fxch(i - 1);
    fpu.push(load(e, frame + 0x20));
    fpu.fxch(i);
}

/// Form 2, `ST0 = STi / ST0`: `FSTP tbyte [ESP]; FXCH ST(i-1); FSTP tbyte [ESP+0xc]; CALL
/// 00ed62f0; FXCH ST(i-1); FLD tbyte [ESP+0xc]; FXCH ST(i)`.
fn divide_into_st0(e: &mut Engine, fpu: &mut Fpu, frame: u32, i: usize) {
    store(e, frame, fpu.pop());
    fpu.fxch(i - 1);
    store(e, frame + 0xc, fpu.pop());
    fdiv_routine(e, fpu, frame);
    fpu.fxch(i - 1);
    fpu.push(load(e, frame + 0xc));
    fpu.fxch(i);
}

/// Form 4, `STi = STi / ST0`: `FXCH ST(i); FSTP tbyte [ESP+0xc]; FXCH ST(i-1); FLD ST0; FSTP
/// tbyte [ESP]; FSTP tbyte [ESP+0x20]; CALL 00ed62f0; FXCH ST(i-1); FLD tbyte [ESP+0x20]`.
fn divide_sti_by(e: &mut Engine, fpu: &mut Fpu, frame: u32, i: usize) {
    fpu.fxch(i);
    store(e, frame + 0xc, fpu.pop());
    fpu.fxch(i - 1);
    fpu.push(fpu.st(0));
    store(e, frame, fpu.pop());
    store(e, frame + 0x20, fpu.pop());
    fdiv_routine(e, fpu, frame);
    fpu.fxch(i - 1);
    fpu.push(load(e, frame + 0x20));
}

/// Form 5, `STi = STi / ST0` and pop: `FSTP tbyte [ESP]; FXCH ST(i-1); FSTP tbyte [ESP+0xc];
/// CALL 00ed62f0; FXCH ST(i-1)`.
fn divide_sti_by_and_pop(e: &mut Engine, fpu: &mut Fpu, frame: u32, i: usize) {
    store(e, frame, fpu.pop());
    fpu.fxch(i - 1);
    store(e, frame + 0xc, fpu.pop());
    fdiv_routine(e, fpu, frame);
    fpu.fxch(i - 1);
}

/// Form 6, `STi = ST0 / STi`: `FSTP tbyte [ESP+0xc]; FXCH ST(i-1); FSTP tbyte [ESP]; CALL
/// 00ed62f0; FXCH ST(i-1); FLD tbyte [ESP+0xc]`.
fn divide_into_sti(e: &mut Engine, fpu: &mut Fpu, frame: u32, i: usize) {
    store(e, frame + 0xc, fpu.pop());
    fpu.fxch(i - 1);
    store(e, frame, fpu.pop());
    fdiv_routine(e, fpu, frame);
    fpu.fxch(i - 1);
    fpu.push(load(e, frame + 0xc));
}

/// Form 7, `STi = ST0 / STi` and pop: `FSTP tbyte [ESP+0xc]; FXCH ST(i-1); FSTP tbyte [ESP];
/// CALL 00ed62f0; FXCH ST(i-1)`.
fn divide_into_sti_and_pop(e: &mut Engine, fpu: &mut Fpu, frame: u32, i: usize) {
    store(e, frame + 0xc, fpu.pop());
    fpu.fxch(i - 1);
    store(e, frame, fpu.pop());
    fdiv_routine(e, fpu, frame);
    fpu.fxch(i - 1);
}

/// `INT 6` after `ADD ESP, 0x2c`: the invalid-opcode exception, which the engine cannot
/// dispatch, so it stops (as `RaiseException` with any code but the thread-naming one does).
fn invalid_selector(addr: u32) -> ! {
    panic!(
        "{addr:08x}: INT 6 (invalid opcode) in the FDIV workaround's register forms: \
         structured exceptions are not provided"
    );
}

// ---- The register-form fragments (the table at 011c259e, in its order) ---------------------------

// Translated from 00ed6414 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x00: `FDIV ST0, ST0` (`D8 F0`): `ST0 = ST0 / ST0`, without the division routine.
fn fdiv_r_6414(_e: &mut Engine, fpu: &mut Fpu, _frame: u32) {
    fpu.set(0, x87_fdiv(fpu.st(0), fpu.st(0)));
}

/// Selector 0x01 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_641a(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_641a);
}

// Translated from 00ed641f (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x02: `FDIVR ST0, ST0` (`D8 F8`): `ST0 = ST0 / ST0`, without the division routine.
fn fdiv_r_641f(_e: &mut Engine, fpu: &mut Fpu, _frame: u32) {
    fpu.set(0, x87_fdiv(fpu.st(0), fpu.st(0)));
}

/// Selector 0x03 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_6425(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_6425);
}

// Translated from 00ed642a (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x04: `FDIV ST0, ST0` (`D8 F0`): `ST0 = ST0 / ST0`, without the division routine.
fn fdiv_r_642a(_e: &mut Engine, fpu: &mut Fpu, _frame: u32) {
    fpu.set(0, x87_fdiv(fpu.st(0), fpu.st(0)));
}

// Translated from 00ed6430 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x05: `FDIVP ST0, ST0` (`DE F8`): `ST0 = ST0 / ST0`, then popped, without the division routine.
fn fdiv_r_6430(_e: &mut Engine, fpu: &mut Fpu, _frame: u32) {
    fpu.set(0, x87_fdiv(fpu.st(0), fpu.st(0)));
    fpu.pop();
}

// Translated from 00ed6436 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x06: `FDIVR ST0, ST0` (`D8 F8`): `ST0 = ST0 / ST0`, without the division routine.
fn fdiv_r_6436(_e: &mut Engine, fpu: &mut Fpu, _frame: u32) {
    fpu.set(0, x87_fdiv(fpu.st(0), fpu.st(0)));
}

// Translated from 00ed643c (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x07: `FDIVRP ST0, ST0` (`DE F0`): `ST0 = ST0 / ST0`, then popped, without the division routine.
fn fdiv_r_643c(_e: &mut Engine, fpu: &mut Fpu, _frame: u32) {
    fpu.set(0, x87_fdiv(fpu.st(0), fpu.st(0)));
    fpu.pop();
}

// Translated from 00ed6442 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x08: `ST0 = ST0 / ST1` through the division routine ([`divide_st0_by`], register 1).
fn fdiv_r_6442(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_st0_by(e, fpu, frame, 1);
}

/// Selector 0x09 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_645e(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_645e);
}

// Translated from 00ed6463 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x0a: `ST0 = ST1 / ST0` through the division routine ([`divide_into_st0`], register 1).
fn fdiv_r_6463(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_st0(e, fpu, frame, 1);
}

/// Selector 0x0b (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_6479(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_6479);
}

// Translated from 00ed647e (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x0c: `ST1 = ST1 / ST0` through the division routine ([`divide_sti_by`], register 1).
fn fdiv_r_647e(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by(e, fpu, frame, 1);
}

// Translated from 00ed649a (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x0d: `ST1 = ST1 / ST0`, then popped through the division routine ([`divide_sti_by_and_pop`], register 1).
fn fdiv_r_649a(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by_and_pop(e, fpu, frame, 1);
}

// Translated from 00ed64aa (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x0e: `ST1 = ST0 / ST1` through the division routine ([`divide_into_sti`], register 1).
fn fdiv_r_64aa(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti(e, fpu, frame, 1);
}

// Translated from 00ed64be (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x0f: `ST1 = ST0 / ST1`, then popped through the division routine ([`divide_into_sti_and_pop`], register 1).
fn fdiv_r_64be(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti_and_pop(e, fpu, frame, 1);
}

// Translated from 00ed64ce (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x10: `ST0 = ST0 / ST2` through the division routine ([`divide_st0_by`], register 2).
fn fdiv_r_64ce(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_st0_by(e, fpu, frame, 2);
}

/// Selector 0x11 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_64ee(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_64ee);
}

// Translated from 00ed64f3 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x12: `ST0 = ST2 / ST0` through the division routine ([`divide_into_st0`], register 2).
fn fdiv_r_64f3(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_st0(e, fpu, frame, 2);
}

/// Selector 0x13 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_650d(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_650d);
}

// Translated from 00ed6512 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x14: `ST2 = ST2 / ST0` through the division routine ([`divide_sti_by`], register 2).
fn fdiv_r_6512(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by(e, fpu, frame, 2);
}

// Translated from 00ed6532 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x15: `ST2 = ST2 / ST0`, then popped through the division routine ([`divide_sti_by_and_pop`], register 2).
fn fdiv_r_6532(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by_and_pop(e, fpu, frame, 2);
}

// Translated from 00ed6546 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x16: `ST2 = ST0 / ST2` through the division routine ([`divide_into_sti`], register 2).
fn fdiv_r_6546(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti(e, fpu, frame, 2);
}

// Translated from 00ed655e (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x17: `ST2 = ST0 / ST2`, then popped through the division routine ([`divide_into_sti_and_pop`], register 2).
fn fdiv_r_655e(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti_and_pop(e, fpu, frame, 2);
}

// Translated from 00ed6572 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x18: `ST0 = ST0 / ST3` through the division routine ([`divide_st0_by`], register 3).
fn fdiv_r_6572(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_st0_by(e, fpu, frame, 3);
}

/// Selector 0x19 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_6592(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_6592);
}

// Translated from 00ed6597 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x1a: `ST0 = ST3 / ST0` through the division routine ([`divide_into_st0`], register 3).
fn fdiv_r_6597(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_st0(e, fpu, frame, 3);
}

/// Selector 0x1b (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_65b1(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_65b1);
}

// Translated from 00ed65b6 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x1c: `ST3 = ST3 / ST0` through the division routine ([`divide_sti_by`], register 3).
fn fdiv_r_65b6(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by(e, fpu, frame, 3);
}

// Translated from 00ed65d6 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x1d: `ST3 = ST3 / ST0`, then popped through the division routine ([`divide_sti_by_and_pop`], register 3).
fn fdiv_r_65d6(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by_and_pop(e, fpu, frame, 3);
}

// Translated from 00ed65ea (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x1e: `ST3 = ST0 / ST3` through the division routine ([`divide_into_sti`], register 3).
fn fdiv_r_65ea(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti(e, fpu, frame, 3);
}

// Translated from 00ed6602 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x1f: `ST3 = ST0 / ST3`, then popped through the division routine ([`divide_into_sti_and_pop`], register 3).
fn fdiv_r_6602(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti_and_pop(e, fpu, frame, 3);
}

// Translated from 00ed6616 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x20: `ST0 = ST0 / ST4` through the division routine ([`divide_st0_by`], register 4).
fn fdiv_r_6616(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_st0_by(e, fpu, frame, 4);
}

/// Selector 0x21 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_6636(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_6636);
}

// Translated from 00ed663b (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x22: `ST0 = ST4 / ST0` through the division routine ([`divide_into_st0`], register 4).
fn fdiv_r_663b(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_st0(e, fpu, frame, 4);
}

/// Selector 0x23 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_6655(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_6655);
}

// Translated from 00ed665a (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x24: `ST4 = ST4 / ST0` through the division routine ([`divide_sti_by`], register 4).
fn fdiv_r_665a(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by(e, fpu, frame, 4);
}

// Translated from 00ed667a (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x25: `ST4 = ST4 / ST0`, then popped through the division routine ([`divide_sti_by_and_pop`], register 4).
fn fdiv_r_667a(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by_and_pop(e, fpu, frame, 4);
}

// Translated from 00ed668e (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x26: `ST4 = ST0 / ST4` through the division routine ([`divide_into_sti`], register 4).
fn fdiv_r_668e(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti(e, fpu, frame, 4);
}

// Translated from 00ed66a6 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x27: `ST4 = ST0 / ST4`, then popped through the division routine ([`divide_into_sti_and_pop`], register 4).
fn fdiv_r_66a6(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti_and_pop(e, fpu, frame, 4);
}

// Translated from 00ed66ba (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x28: `ST0 = ST0 / ST5` through the division routine ([`divide_st0_by`], register 5).
fn fdiv_r_66ba(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_st0_by(e, fpu, frame, 5);
}

/// Selector 0x29 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_66da(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_66da);
}

// Translated from 00ed66df (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x2a: `ST0 = ST5 / ST0` through the division routine ([`divide_into_st0`], register 5).
fn fdiv_r_66df(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_st0(e, fpu, frame, 5);
}

/// Selector 0x2b (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_66f9(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_66f9);
}

// Translated from 00ed66fe (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x2c: `ST5 = ST5 / ST0` through the division routine ([`divide_sti_by`], register 5).
fn fdiv_r_66fe(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by(e, fpu, frame, 5);
}

// Translated from 00ed671e (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x2d: `ST5 = ST5 / ST0`, then popped through the division routine ([`divide_sti_by_and_pop`], register 5).
fn fdiv_r_671e(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by_and_pop(e, fpu, frame, 5);
}

// Translated from 00ed6732 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x2e: `ST5 = ST0 / ST5` through the division routine ([`divide_into_sti`], register 5).
fn fdiv_r_6732(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti(e, fpu, frame, 5);
}

// Translated from 00ed674a (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x2f: `ST5 = ST0 / ST5`, then popped through the division routine ([`divide_into_sti_and_pop`], register 5).
fn fdiv_r_674a(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti_and_pop(e, fpu, frame, 5);
}

// Translated from 00ed675e (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x30: `ST0 = ST0 / ST6` through the division routine ([`divide_st0_by`], register 6).
fn fdiv_r_675e(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_st0_by(e, fpu, frame, 6);
}

/// Selector 0x31 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_677e(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_677e);
}

// Translated from 00ed6783 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x32: `ST0 = ST6 / ST0` through the division routine ([`divide_into_st0`], register 6).
fn fdiv_r_6783(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_st0(e, fpu, frame, 6);
}

/// Selector 0x33 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_679d(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_679d);
}

// Translated from 00ed67a2 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x34: `ST6 = ST6 / ST0` through the division routine ([`divide_sti_by`], register 6).
fn fdiv_r_67a2(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by(e, fpu, frame, 6);
}

// Translated from 00ed67c2 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x35: `ST6 = ST6 / ST0`, then popped through the division routine ([`divide_sti_by_and_pop`], register 6).
fn fdiv_r_67c2(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by_and_pop(e, fpu, frame, 6);
}

// Translated from 00ed67d6 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x36: `ST6 = ST0 / ST6` through the division routine ([`divide_into_sti`], register 6).
fn fdiv_r_67d6(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti(e, fpu, frame, 6);
}

// Translated from 00ed67ee (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x37: `ST6 = ST0 / ST6`, then popped through the division routine ([`divide_into_sti_and_pop`], register 6).
fn fdiv_r_67ee(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti_and_pop(e, fpu, frame, 6);
}

// Translated from 00ed6802 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x38: `ST0 = ST0 / ST7` through the division routine ([`divide_st0_by`], register 7).
fn fdiv_r_6802(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_st0_by(e, fpu, frame, 7);
}

/// Selector 0x39 (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_6822(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_6822);
}

// Translated from 00ed6827 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x3a: `ST0 = ST7 / ST0` through the division routine ([`divide_into_st0`], register 7).
fn fdiv_r_6827(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_st0(e, fpu, frame, 7);
}

/// Selector 0x3b (`ADD ESP, 0x2c; INT 6`): no division for this selector; the frame is given
/// back and the invalid-opcode exception raised ([`invalid_selector`]).
fn fdiv_r_6841(_e: &mut Engine, _fpu: &mut Fpu, _frame: u32) {
    invalid_selector(0x00ed_6841);
}

// Translated from 00ed6846 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x3c: `ST7 = ST7 / ST0` through the division routine ([`divide_sti_by`], register 7).
fn fdiv_r_6846(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by(e, fpu, frame, 7);
}

// Translated from 00ed6866 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x3d: `ST7 = ST7 / ST0`, then popped through the division routine ([`divide_sti_by_and_pop`], register 7).
fn fdiv_r_6866(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_sti_by_and_pop(e, fpu, frame, 7);
}

// Translated from 00ed687a (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x3e: `ST7 = ST0 / ST7` through the division routine ([`divide_into_sti`], register 7).
fn fdiv_r_687a(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti(e, fpu, frame, 7);
}

// Translated from 00ed6892 (disassembled, FalloutNV.exe 1.4.0.525)
/// Selector 0x3f: `ST7 = ST0 / ST7`, then popped through the division routine ([`divide_into_sti_and_pop`], register 7).
fn fdiv_r_6892(e: &mut Engine, fpu: &mut Fpu, frame: u32) {
    divide_into_sti_and_pop(e, fpu, frame, 7);
}

/// The register-form dispatcher's table: each fragment by the address `011c259e` holds for it.
const FRAGMENTS: [(u32, Fragment); 64] = [
    (0x00ed_6414, fdiv_r_6414),
    (0x00ed_641a, fdiv_r_641a),
    (0x00ed_641f, fdiv_r_641f),
    (0x00ed_6425, fdiv_r_6425),
    (0x00ed_642a, fdiv_r_642a),
    (0x00ed_6430, fdiv_r_6430),
    (0x00ed_6436, fdiv_r_6436),
    (0x00ed_643c, fdiv_r_643c),
    (0x00ed_6442, fdiv_r_6442),
    (0x00ed_645e, fdiv_r_645e),
    (0x00ed_6463, fdiv_r_6463),
    (0x00ed_6479, fdiv_r_6479),
    (0x00ed_647e, fdiv_r_647e),
    (0x00ed_649a, fdiv_r_649a),
    (0x00ed_64aa, fdiv_r_64aa),
    (0x00ed_64be, fdiv_r_64be),
    (0x00ed_64ce, fdiv_r_64ce),
    (0x00ed_64ee, fdiv_r_64ee),
    (0x00ed_64f3, fdiv_r_64f3),
    (0x00ed_650d, fdiv_r_650d),
    (0x00ed_6512, fdiv_r_6512),
    (0x00ed_6532, fdiv_r_6532),
    (0x00ed_6546, fdiv_r_6546),
    (0x00ed_655e, fdiv_r_655e),
    (0x00ed_6572, fdiv_r_6572),
    (0x00ed_6592, fdiv_r_6592),
    (0x00ed_6597, fdiv_r_6597),
    (0x00ed_65b1, fdiv_r_65b1),
    (0x00ed_65b6, fdiv_r_65b6),
    (0x00ed_65d6, fdiv_r_65d6),
    (0x00ed_65ea, fdiv_r_65ea),
    (0x00ed_6602, fdiv_r_6602),
    (0x00ed_6616, fdiv_r_6616),
    (0x00ed_6636, fdiv_r_6636),
    (0x00ed_663b, fdiv_r_663b),
    (0x00ed_6655, fdiv_r_6655),
    (0x00ed_665a, fdiv_r_665a),
    (0x00ed_667a, fdiv_r_667a),
    (0x00ed_668e, fdiv_r_668e),
    (0x00ed_66a6, fdiv_r_66a6),
    (0x00ed_66ba, fdiv_r_66ba),
    (0x00ed_66da, fdiv_r_66da),
    (0x00ed_66df, fdiv_r_66df),
    (0x00ed_66f9, fdiv_r_66f9),
    (0x00ed_66fe, fdiv_r_66fe),
    (0x00ed_671e, fdiv_r_671e),
    (0x00ed_6732, fdiv_r_6732),
    (0x00ed_674a, fdiv_r_674a),
    (0x00ed_675e, fdiv_r_675e),
    (0x00ed_677e, fdiv_r_677e),
    (0x00ed_6783, fdiv_r_6783),
    (0x00ed_679d, fdiv_r_679d),
    (0x00ed_67a2, fdiv_r_67a2),
    (0x00ed_67c2, fdiv_r_67c2),
    (0x00ed_67d6, fdiv_r_67d6),
    (0x00ed_67ee, fdiv_r_67ee),
    (0x00ed_6802, fdiv_r_6802),
    (0x00ed_6822, fdiv_r_6822),
    (0x00ed_6827, fdiv_r_6827),
    (0x00ed_6841, fdiv_r_6841),
    (0x00ed_6846, fdiv_r_6846),
    (0x00ed_6866, fdiv_r_6866),
    (0x00ed_687a, fdiv_r_687a),
    (0x00ed_6892, fdiv_r_6892),
];

// ---- The register and memory-operand entries ---------------------------------------------------

/// `SUB ESP, 0x2c`, the two registers into the frame (`first` the slot `ST0` goes to), `CALL
/// 00ed62f0`, `ADD ESP, 0x2c`.
fn divide_registers(e: &mut Engine, fpu: &mut Fpu, st0_slot: u32, st1_slot: u32) {
    e.with_stack(0x2c, |e, frame| {
        let frame = frame.addr();
        store(e, frame + st0_slot, fpu.pop());
        store(e, frame + st1_slot, fpu.pop());
        fdiv_routine(e, fpu, frame);
    })
}

// Translated from 00ed68a6 (disassembled, FalloutNV.exe 1.4.0.525)
/// The memory-operand entries' division (no Ghidra name): `ST0` into the divisor slot, `ST1`
/// into the dividend slot (`FSTP tbyte [ESP]; FSTP tbyte [ESP+0xc]`) and the routine:
/// `ST0 = ST1 / ST0` with both popped.
pub(super) fn divide_st1_by_st0(e: &mut Engine, fpu: &mut Fpu) {
    divide_registers(e, fpu, 0, 0xc);
}

// Translated from 00ed68b9 (disassembled, FalloutNV.exe 1.4.0.525)
/// The reversed memory-operand entries' division (no Ghidra name): `ST0` into the dividend
/// slot, `ST1` into the divisor slot and the routine: `ST0 = ST0 / ST1` with both popped.
pub(super) fn divide_st0_by_st1(e: &mut Engine, fpu: &mut Fpu) {
    divide_registers(e, fpu, 0xc, 0);
}

// Translated from 00ed6acc (disassembled, FalloutNV.exe 1.4.0.525)
/// A register entry (no Ghidra name; `EAX` kept, `PUSH EAX` .. `POP EAX`): as `00ed68a6`,
/// `ST0 = ST1 / ST0` with both popped.
pub(super) fn divide_st1_by_st0_kept(e: &mut Engine, fpu: &mut Fpu) {
    divide_registers(e, fpu, 0, 0xc);
}

// Translated from 00ed6ae1 (disassembled, FalloutNV.exe 1.4.0.525)
/// A register entry (no Ghidra name; `EAX` kept; `exp`'s handler `00ed9301` calls it for
/// `1 / x` with the flaw flag set): as `00ed68b9`, `ST0 = ST0 / ST1` with both popped.
pub(super) fn divide_st0_by_st1_kept(e: &mut Engine, fpu: &mut Fpu) {
    divide_registers(e, fpu, 0xc, 0);
}

/// The memory-operand entries' common part (`00ed68dd` and its like): with the stack top not 0
/// (`FNSTSW AX; AND EAX, 0x3800`, [`stack_top`]) the operand is pushed and `divide` runs;
/// with it 0 (a full stack) `ST0` and `ST1` change places, `ST0` is kept in a 12-byte frame
/// (`FXCH; SUB ESP, 0xc; FSTP tbyte [ESP]`), the operand is pushed, `divide` runs, the kept
/// register comes back and the two change places again (`FLD tbyte [ESP]; FXCH`).
fn memory_form(e: &mut Engine, fpu: &mut Fpu, operand: X87Ext, divide: fn(&mut Engine, &mut Fpu)) {
    if stack_top(fpu) != 0 {
        fpu.push(operand);
        divide(e, fpu);
        return;
    }
    fpu.fxch(1);
    e.with_stack(0xc, |e, spill| {
        let spill = spill.addr();
        store(e, spill, fpu.pop());
        fpu.push(operand);
        divide(e, fpu);
        fpu.push(load(e, spill));
        fpu.fxch(1);
    })
}

// Translated from 00ed68cc (disassembled, FalloutNV.exe 1.4.0.525)
/// The `FDIV dword` entry (no Ghidra name; stdcall, the float on the stack, `RET 4`; `EAX`
/// kept): a float whose exponent is all ones (an infinity or a NaN) divides at once (`FDIV
/// dword ptr [ESP+8]`, `ST0 = ST0 / m`); otherwise `ST0 = ST0 / m` through `00ed68a6`
/// ([`memory_form`], the float loaded with `FLD dword`).
pub(super) fn fdiv_m32(e: &mut Engine, fpu: &mut Fpu, operand: u32) {
    if operand & 0x7f80_0000 == 0x7f80_0000 {
        fpu.set(0, x87_fdiv(fpu.st(0), float_operand(operand)));
        return;
    }
    memory_form(e, fpu, X87Ext::load32(operand), divide_st1_by_st0);
}

// Translated from 00ed6918 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `FDIV qword` entry (no Ghidra name; stdcall, the double on the stack, `RET 8`; `EAX`
/// kept): a double whose exponent is all ones divides at once (`FDIV qword ptr [ESP+8]`);
/// otherwise `ST0 = ST0 / m` through `00ed68a6` (the double loaded with `FLD qword`).
pub(super) fn fdiv_m64(e: &mut Engine, fpu: &mut Fpu, operand: u64) {
    if (operand >> 32) as u32 & 0x7ff0_0000 == 0x7ff0_0000 {
        fpu.set(0, x87_fdiv(fpu.st(0), ext_of_double(operand)));
        return;
    }
    let value = X87Ext::load64(f64::from_bits(operand));
    memory_form(e, fpu, value, divide_st1_by_st0);
}

// Translated from 00ed6964 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `FIDIV word` entry (no Ghidra name; stdcall, one word whose low half is the integer,
/// `RET 4`; `EAX` kept): `ST0 = ST0 / m` through `00ed68a6` (`FILD word`).
pub(super) fn fdiv_m16i(e: &mut Engine, fpu: &mut Fpu, operand: u32) {
    let value = X87Ext::load64(f64::from(operand as u16 as i16));
    memory_form(e, fpu, value, divide_st1_by_st0);
}

// Translated from 00ed6998 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `FIDIV dword` entry (no Ghidra name; stdcall, `RET 4`; `EAX` kept): `ST0 = ST0 / m`
/// through `00ed68a6` (`FILD dword`).
pub(super) fn fdiv_m32i(e: &mut Engine, fpu: &mut Fpu, operand: u32) {
    let value = X87Ext::load64(f64::from(operand as i32));
    memory_form(e, fpu, value, divide_st1_by_st0);
}

// Translated from 00ed69cc (disassembled, FalloutNV.exe 1.4.0.525)
/// The `FDIVR dword` entry (no Ghidra name; stdcall, `RET 4`; `EAX` kept): a float whose
/// exponent is all ones divides at once (`FDIVR dword ptr [ESP+8]`, `ST0 = m / ST0`);
/// otherwise `ST0 = m / ST0` through `00ed68b9`.
pub(super) fn fdivr_m32(e: &mut Engine, fpu: &mut Fpu, operand: u32) {
    if operand & 0x7f80_0000 == 0x7f80_0000 {
        fpu.set(0, x87_fdiv(float_operand(operand), fpu.st(0)));
        return;
    }
    memory_form(e, fpu, X87Ext::load32(operand), divide_st0_by_st1);
}

// Translated from 00ed6a18 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `FDIVR qword` entry (no Ghidra name; stdcall, `RET 8`; `EAX` kept): a double whose
/// exponent is all ones divides at once (`FDIVR qword ptr [ESP+8]`); otherwise `ST0 = m / ST0`
/// through `00ed68b9`.
pub(super) fn fdivr_m64(e: &mut Engine, fpu: &mut Fpu, operand: u64) {
    if (operand >> 32) as u32 & 0x7ff0_0000 == 0x7ff0_0000 {
        fpu.set(0, x87_fdiv(ext_of_double(operand), fpu.st(0)));
        return;
    }
    let value = X87Ext::load64(f64::from_bits(operand));
    memory_form(e, fpu, value, divide_st0_by_st1);
}

// Translated from 00ed6a64 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `FIDIVR word` entry (no Ghidra name; stdcall, `RET 4`; `EAX` kept): `ST0 = m / ST0`
/// through `00ed68b9` (`FILD word`).
pub(super) fn fdivr_m16i(e: &mut Engine, fpu: &mut Fpu, operand: u32) {
    let value = X87Ext::load64(f64::from(operand as u16 as i16));
    memory_form(e, fpu, value, divide_st0_by_st1);
}

// Translated from 00ed6a98 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `FIDIVR dword` entry (no Ghidra name; stdcall, `RET 4`; `EAX` kept): `ST0 = m / ST0`
/// through `00ed68b9` (`FILD dword`).
pub(super) fn fdivr_m32i(e: &mut Engine, fpu: &mut Fpu, operand: u32) {
    let value = X87Ext::load64(f64::from(operand as i32));
    memory_form(e, fpu, value, divide_st0_by_st1);
}

/// The registered forms of the register entries: `ST0`, `ST1`; the quotient.
fn register_entry(e: &mut Engine, words: &[u32], divide: fn(&mut Engine, &mut Fpu)) -> Ret {
    let mut fpu = registers(&words[..4]);
    divide(e, &mut fpu);
    st0_answer(&fpu)
}

fn divide_st1_by_st0_entry(e: &mut Engine, words: &[u32]) -> Ret {
    register_entry(e, words, divide_st1_by_st0)
}

fn divide_st0_by_st1_entry(e: &mut Engine, words: &[u32]) -> Ret {
    register_entry(e, words, divide_st0_by_st1)
}

fn divide_st1_by_st0_kept_entry(e: &mut Engine, words: &[u32]) -> Ret {
    register_entry(e, words, divide_st1_by_st0_kept)
}

fn divide_st0_by_st1_kept_entry(e: &mut Engine, words: &[u32]) -> Ret {
    register_entry(e, words, divide_st0_by_st1_kept)
}

/// The registered forms of the memory-operand entries with a one-word operand: `ST0`, then the
/// operand word; the quotient.
fn word_operand_entry(e: &mut Engine, words: &[u32], entry: fn(&mut Engine, &mut Fpu, u32)) -> Ret {
    let mut fpu = registers(&words[..2]);
    entry(e, &mut fpu, words[2]);
    st0_answer(&fpu)
}

/// The registered forms of the double entries: `ST0`, then the double (low word first).
fn double_operand_entry(
    e: &mut Engine,
    words: &[u32],
    entry: fn(&mut Engine, &mut Fpu, u64),
) -> Ret {
    let mut fpu = registers(&words[..2]);
    entry(e, &mut fpu, u64::from(words[2]) | u64::from(words[3]) << 32);
    st0_answer(&fpu)
}

fn fdiv_m32_entry(e: &mut Engine, words: &[u32]) -> Ret {
    word_operand_entry(e, words, fdiv_m32)
}

fn fdiv_m64_entry(e: &mut Engine, words: &[u32]) -> Ret {
    double_operand_entry(e, words, fdiv_m64)
}

fn fdiv_m16i_entry(e: &mut Engine, words: &[u32]) -> Ret {
    word_operand_entry(e, words, fdiv_m16i)
}

fn fdiv_m32i_entry(e: &mut Engine, words: &[u32]) -> Ret {
    word_operand_entry(e, words, fdiv_m32i)
}

fn fdivr_m32_entry(e: &mut Engine, words: &[u32]) -> Ret {
    word_operand_entry(e, words, fdivr_m32)
}

fn fdivr_m64_entry(e: &mut Engine, words: &[u32]) -> Ret {
    double_operand_entry(e, words, fdivr_m64)
}

fn fdivr_m16i_entry(e: &mut Engine, words: &[u32]) -> Ret {
    word_operand_entry(e, words, fdivr_m16i)
}

fn fdivr_m32i_entry(e: &mut Engine, words: &[u32]) -> Ret {
    word_operand_entry(e, words, fdivr_m32i)
}

// ---- The partial remainder ---------------------------------------------------------------------

// Translated from 00ed6cfc (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed6cfc - the engine keeps no x87 status word: the answer is the condition codes as 00ed6af6's model leaves them
/// The partial remainder (no Ghidra name; `fmod`'s handler `00ec9144` calls it instead of
/// `FPREM` with the flaw flag set; `ST0` the dividend, `ST1` the divisor; `EDX` kept): both
/// into a 0x30-byte frame (the dividend at +0x18, the divisor at +0), `EDX` = 0. A divisor with
/// an exponent not 0 goes to the body `00ed6af6` at once. Otherwise both are loaded again; a
/// zero divisor (significand 0) gets a plain `FPREM`; a denormal one is kept at +0xc, `EDX`
/// bit 1 set, and under the caller's control word with every exception masked and 64-bit
/// precision (`word | 0x33f`): a dividend exponent up to 0x7fbe sets `EDX` bit 0 and both are
/// multiplied by the double at `011c2574` into their slots; a larger one has the word stored
/// again (the changed one) with 64-bit precision (`| 0x300`) loaded, and only the divisor
/// multiplied. The word at +0x24 comes back (in the second case the changed one) and the body
/// runs. Leaves `ST0` the remainder and `ST1` the divisor; answers the condition codes.
pub(super) fn fprem_adjusted(e: &mut Engine, fpu: &mut Fpu) -> u16 {
    e.with_stack(0x30, |e, frame| {
        let f = frame.addr();
        // FSTP tbyte [ESP+0x18]; FSTP tbyte [ESP]; XOR EDX, EDX.
        store(e, f + 0x18, fpu.pop());
        store(e, f, fpu.pop());
        let mut edx = 0u32;
        // MOV EAX, [ESP+6]; TEST EAX, 0x7fff0000.
        if e.mem.u32(f + 6) & 0x7fff_0000 != 0 {
            return fprem_body(e, fpu, f, edx);
        }
        // 00ed6d1e: FLD tbyte [ESP]; FLD tbyte [ESP+0x18].
        fpu.push(load(e, f));
        fpu.push(load(e, f + 0x18));
        if e.mem.u32(f) | e.mem.u32(f + 4) == 0 {
            // 00ed6da7: FPREM.
            return fprem(fpu);
        }
        // FXCH; FSTP tbyte [ESP+0xc]; FLD tbyte [ESP]; FXCH; OR EDX, 2.
        fpu.fxch(1);
        store(e, f + 0xc, fpu.pop());
        fpu.push(load(e, f));
        fpu.fxch(1);
        edx |= 2;
        fnstcw(e, f + 0x24);
        let word = e.mem.u32(f + 0x24) | 0x33f;
        e.mem.set_u32(f + 0x28, word);
        fldcw(e, f + 0x28);
        let scale = double_operand(e, FPREM_DENORMAL_SCALE);
        if e.mem.u32(f + 0x20) & 0x7fff <= 0x7fbe {
            // OR EDX, 1; FMUL; FSTP tbyte [ESP+0x18]; FMUL; FSTP tbyte [ESP].
            edx |= 1;
            fpu.set(0, x87_fmul(fpu.st(0), scale));
            store(e, f + 0x18, fpu.pop());
            fpu.set(0, x87_fmul(fpu.st(0), scale));
            store(e, f, fpu.pop());
        } else {
            // 00ed6d79: FNSTCW; OR 0x300; FLDCW; FSTP ST0; FMUL; FSTP tbyte [ESP].
            fnstcw(e, f + 0x24);
            let word = e.mem.u32(f + 0x24) | 0x300;
            e.mem.set_u32(f + 0x28, word);
            fldcw(e, f + 0x28);
            fpu.pop();
            fpu.set(0, x87_fmul(fpu.st(0), scale));
            store(e, f, fpu.pop());
        }
        // 00ed6d99: FLDCW [ESP+0x24]; CALL 00ed6af6.
        fldcw(e, f + 0x24);
        fprem_body(e, fpu, f, edx)
    })
}

// Translated from 00ed6af6 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed6af6 - the engine keeps no x87 status word: the answer is FPREM's condition codes, C1 cleared by the FSTP/FLD/FCHS after an FPREM, and C0/C2/C3, which the SDM leaves undefined after FMUL, FXCH, FLD, FSTP and FCHS, taken as unchanged (not checked on a CPU); the FNSTENV/FLDENV block below the frame is not kept
/// The partial remainder's body (no Ghidra name; `EAX`, `EBX`, `ECX` kept; `EDX` the flags of
/// `00ed6cfc`: bit 0 both operands scaled, bit 1 the divisor kept at +0xc; `frame` its frame:
/// the divisor at +0, the dividend at +0x18). A divisor whose significand bits 56 to 58 are all
/// set, whose bits 59 to 62 mark a byte in the table `011c255c`, with an exponent not 0x7fff,
/// a dividend exponent neither 0 nor 0x7fff, and both significands' high words zero but for
/// the top bit, takes the long way: while the dividend's exponent is at most the divisor's plus
/// 0x3f, `FPREM`s with the divisor's exponent brought up (to the dividend's minus 4 to 11)
/// reduce the dividend in its slot until its exponent is below the divisor's plus 10 (then the
/// plain `FPREM`); beyond that, under the word with every exception masked and 64-bit
/// precision, a copy of the divisor kept at +0xc (unless bit 1 says it is there), the divisor
/// given the dividend's exponent, a shift-and-subtract loop of `0x21 + (difference & 0x3f)`
/// rounds on their absolute values halves the divisor each round (the double at `011c258c`),
/// an `FPREM` of the extended at `011c2594` by the kept divisor, and the remainder with the
/// dividend's sign. Every other case is one `FPREM` of the dividend by the divisor. Then, with
/// a flag of `EDX` set: with bit 0 the remainder is scaled back (the double at `011c257c`,
/// 64-bit precision), the kept divisor replaces `ST1`, and C3, C1 and C0 come back as they
/// were after the `FPREM` (`FNSTENV`, `AND 0xbcff`, `OR`, `FLDENV`). Leaves `ST0` the
/// remainder and `ST1` the divisor; answers the condition codes.
fn fprem_body(e: &mut Engine, fpu: &mut Fpu, f: u32, edx: u32) -> u16 {
    let divisor_top = e.mem.u32(f + 6) ^ 0x700;
    let long_way = divisor_top & 0x700 == 0
        && e.mem.u8(FPREM_TABLE + ((divisor_top >> 11) & 0xf)) != 0
        && e.mem.u32(f + 6) & 0x7fff_0000 != 0x7fff_0000
        && matches!(e.mem.u32(f + 0x1e) & 0x7fff_0000, x if x != 0 && x != 0x7fff_0000)
        && e.mem.u32(f + 0x1c).wrapping_add(e.mem.u32(f + 0x1c)) == 0
        && e.mem.u32(f + 4).wrapping_add(e.mem.u32(f + 4)) == 0;
    let mut status = 0u16;
    let mut plain = !long_way;
    if long_way {
        // 00ed6b66: the dividend's exponent above the divisor's plus 0x3f (JA)?
        let divisor_exponent = e.mem.u32(f + 8) & 0x7fff;
        let dividend_exponent = e.mem.u32(f + 0x20) & 0x7fff;
        if dividend_exponent <= divisor_exponent + 0x3f {
            // 00ed6b80: reduce until the dividend's exponent is below the divisor's plus 10.
            loop {
                let low = (e.mem.u32(f + 8) & 0x7fff) + 0xa;
                let dividend_exponent = e.mem.u32(f + 0x20) & 0x7fff;
                if (dividend_exponent.wrapping_sub(low) as i32) < 0 {
                    plain = true;
                    break;
                }
                fpu.push(load(e, f + 0x18));
                let divisor_word = e.mem.u32(f + 8);
                let exponent = e.mem.u32(f + 0x20) & 0x7fff;
                let step = (exponent.wrapping_sub(divisor_word) & 7) | 4;
                let raised = exponent.wrapping_sub(step) | (divisor_word & 0x8000);
                e.mem.set_u32(f + 8, raised);
                fpu.push(load(e, f));
                e.mem.set_u32(f + 8, divisor_word);
                fpu.fxch(1);
                fprem(fpu);
                store(e, f + 0x18, fpu.pop());
                fpu.pop();
            }
        } else {
            // 00ed6bde.
            if edx & 2 == 0 {
                let divisor = load(e, f);
                store(e, f + 0xc, divisor);
            }
            fnstcw(e, f + 0x24);
            let word = e.mem.u32(f + 0x24) | 0x33f;
            e.mem.set_u32(f + 0x28, word);
            fldcw(e, f + 0x28);
            let difference = (e.mem.u32(f + 0x20) & 0x7fff).wrapping_sub(e.mem.u32(f + 8) & 0x7fff);
            let mut rounds = ((difference & 0x3f) | 0x20) + 1;
            let sign = e.mem.u32(f + 8) & 0x8000;
            e.mem.set_u32(f + 8, (e.mem.u32(f + 0x20) & 0x7fff) | sign);
            fpu.push(x87_fabs(load(e, f)));
            fpu.push(x87_fabs(load(e, f + 0x18)));
            let half = double_operand(e, FPREM_HALF);
            loop {
                // FCOM; FNSTSW AX; AND EAX, 0x100; JNZ: subtract unless below.
                if x87_fcom(fpu.st(0), fpu.st(1)) & X87_STATUS_C0 == 0 {
                    fpu.set(0, x87_fsub(fpu.st(0), fpu.st(1)));
                }
                fpu.fxch(1);
                fpu.set(0, x87_fmul(fpu.st(0), half));
                fpu.fxch(1);
                rounds -= 1;
                if rounds == 0 {
                    break;
                }
            }
            let dividend_word = e.mem.u32(f + 0x20);
            store(e, f + 0x18, fpu.pop());
            fpu.pop();
            fpu.push(load(e, f + 0xc));
            fpu.push(load(e, FPREM_OPERAND));
            status = fprem(fpu);
            fpu.pop();
            fpu.push(load(e, f + 0x18));
            status &= !X87_STATUS_C1;
            fldcw(e, f + 0x24);
            if dividend_word & 0x8000 != 0 {
                fpu.set(0, x87_fchs(fpu.st(0)));
            }
        }
    }
    if plain {
        // 00ed6c90: FLD tbyte [ESP+0x10]; FLD tbyte [ESP+0x28]; FPREM.
        fpu.push(load(e, f));
        fpu.push(load(e, f + 0x18));
        status = fprem(fpu);
    }
    // 00ed6c9a.
    if edx & 3 != 0 {
        e.mem.set_u16(f + 0x2c, status);
        if edx & 1 != 0 {
            fnstcw(e, f + 0x24);
            let word = e.mem.u32(f + 0x24) | 0x300;
            e.mem.set_u32(f + 0x28, word);
            fldcw(e, f + 0x28);
            fpu.set(0, x87_fmul(fpu.st(0), double_operand(e, FPREM_UNSCALE)));
            fldcw(e, f + 0x24);
        }
        let saved = e.mem.u16(f + 0x2c);
        // FXCH; FSTP ST0; FLD tbyte [ESP+0x1c]; FXCH; the status word's C3, C1, C0 put back.
        fpu.fxch(1);
        fpu.pop();
        fpu.push(load(e, f + 0xc));
        fpu.fxch(1);
        status = (status & 0xbcff) | (saved & 0x4300);
    }
    status
}

/// The registered form of `00ed6cfc`: `ST0`, `ST1`; the remainder, and the condition codes in
/// `EAX` (the status word the caller's `FSTSW AX` reads; the routine's own `EAX` is not kept).
fn fprem_adjusted_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let mut fpu = registers(&words[..4]);
    let status = fprem_adjusted(e, &mut fpu);
    Ret {
        eax: u32::from(status),
        st0: fpu.st(0).store64(),
        ..Ret::default()
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00ed_62f0, fdiv_routine_entry as AbiFn),
        (0x00ed_6407, fdiv_registers_entry as AbiFn),
        (0x00ed_68a6, divide_st1_by_st0_entry as AbiFn),
        (0x00ed_68b9, divide_st0_by_st1_entry as AbiFn),
        (0x00ed_68cc, fdiv_m32_entry as AbiFn),
        (0x00ed_6918, fdiv_m64_entry as AbiFn),
        (0x00ed_6964, fdiv_m16i_entry as AbiFn),
        (0x00ed_6998, fdiv_m32i_entry as AbiFn),
        (0x00ed_69cc, fdivr_m32_entry as AbiFn),
        (0x00ed_6a18, fdivr_m64_entry as AbiFn),
        (0x00ed_6a64, fdivr_m16i_entry as AbiFn),
        (0x00ed_6a98, fdivr_m32i_entry as AbiFn),
        (0x00ed_6acc, divide_st1_by_st0_kept_entry as AbiFn),
        (0x00ed_6ae1, divide_st0_by_st1_kept_entry as AbiFn),
        (0x00ed_6cfc, fprem_adjusted_entry as AbiFn),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::x87::{set_control_word, DEFAULT_CONTROL_WORD, X87_STATUS_C2};

    /// An engine with the workaround's data area mapped (all zero: no divisor marked) and the
    /// default control word.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_2000, 0x1000);
        set_x87_control_word(&mut e, DEFAULT_CONTROL_WORD);
        e
    }

    fn ext(x: f64) -> X87Ext {
        X87Ext::load64(x)
    }

    /// A register stack from `ST0` on.
    fn stack(values: &[X87Ext]) -> Fpu {
        let mut fpu = Fpu::default();
        for v in values.iter().rev() {
            fpu.push(*v);
        }
        fpu
    }

    /// The register stack from `ST0` on.
    fn read(fpu: &Fpu) -> Vec<X87Ext> {
        fpu.st.iter().rev().copied().collect()
    }

    /// The three words of an extended value as it sits in a frame slot (the last half word 0).
    fn words(x: X87Ext) -> [u32; 3] {
        let b = x.0;
        [
            u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            u32::from(u16::from_le_bytes([b[8], b[9]])),
        ]
    }

    /// The routine called by address with `dividend / divisor` in its frame.
    fn routine(e: &mut Engine, dividend: X87Ext, divisor: X87Ext) -> u64 {
        let mut w = Vec::new();
        w.extend_from_slice(&words(divisor));
        w.extend_from_slice(&words(dividend));
        w.extend_from_slice(&[0, 0]);
        e.call(0x00ed_62f0, &w).f64().to_bits()
    }

    /// Small pseudo-random doubles of both signs.
    struct Spread(u64);

    impl Spread {
        fn next(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let unit = (self.0 >> 11) as f64 / (1u64 << 53) as f64;
            (unit - 0.5) * 1e6
        }
    }

    #[test]
    fn the_routine_divides_unmarked_divisors_at_once() {
        let mut e = engine();
        let mut spread = Spread(0x5eed);
        for _ in 0..200 {
            let (a, b) = (spread.next(), spread.next());
            let want = x87_fdiv(ext(a), ext(b)).store64().to_bits();
            assert_eq!(routine(&mut e, ext(a), ext(b)), want, "{a:e} / {b:e}");
        }
        // NaNs, infinities and zeros go through FDIVP as they are.
        for (a, b) in [
            (1.0, 0.0),
            (0.0, 0.0),
            (f64::INFINITY, 2.0),
            (f64::NAN, 3.0),
            (5.0, f64::NEG_INFINITY),
        ] {
            let want = x87_fdiv(ext(a), ext(b)).store64().to_bits();
            assert_eq!(routine(&mut e, ext(a), ext(b)), want, "{a:e} / {b:e}");
        }
        assert_eq!(x87_control_word(&mut e), DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn a_marked_divisor_is_scaled_under_the_guarded_word() {
        let mut e = engine();
        // Significand bits 56 to 58 set and bits 59 to 62 = 3: the table's byte 3.
        let divisor = X87Ext::from_parts(0x3fff, 0x9f12_3456_789a_bcdeu64);
        e.mem.set_u8(FDIV_TABLE + 3, 1);
        e.mem.set_u32(FDIV_SCALE, 0.75f32.to_bits());
        e.mem.set_u32(FDIV_SCALE_EXPONENT_ONE, 0.5f32.to_bits());
        for (dividend, scale) in [
            (ext(1.0 / 3.0), 0.75f32),
            (ext(-12345.678), 0.75),
            (X87Ext::from_parts(0x0001, 0xc000_0000_0000_0001), 0.5),
        ] {
            set_control_word(0x037f);
            let a = x87_fmul(dividend, X87Ext::load32(scale.to_bits()));
            let b = x87_fmul(divisor, X87Ext::load32(scale.to_bits()));
            set_control_word(DEFAULT_CONTROL_WORD);
            let want = x87_fdiv(a, b);
            let mut fpu = Fpu::default();
            e.with_stack(0x2c, |e, frame| {
                let frame = frame.addr();
                store(e, frame, divisor);
                store(e, frame + 0xc, dividend);
                fdiv_routine(e, &mut fpu, frame);
                // The guarded word in its slot: every exception masked, 64-bit, nearest.
                assert_eq!(e.mem.u16(frame + 0x1c), 0x037f);
            });
            assert_eq!(read(&fpu), vec![want]);
            assert_eq!(x87_control_word(&mut e), DEFAULT_CONTROL_WORD);
        }
        // Without the table's byte the same divisor divides at once.
        e.mem.set_u8(FDIV_TABLE + 3, 0);
        let want = x87_fdiv(ext(1.0 / 3.0), divisor).store64().to_bits();
        assert_eq!(routine(&mut e, ext(1.0 / 3.0), divisor), want);
    }

    #[test]
    fn a_denormal_divisor_is_normalised_in_the_frame_and_tested_again() {
        let mut e = engine();
        e.mem.set_u32(FDIV_DENORMAL_SCALE, 0x5f00_0000); // 2^63
        let divisor = X87Ext::from_parts(0x0000, 0x0000_0001_0000_0000);
        let dividend = X87Ext::from_parts(0x0001, 0x8000_0000_0000_0000);
        let mut fpu = Fpu::default();
        e.with_stack(0x2c, |e, frame| {
            let frame = frame.addr();
            store(e, frame, divisor);
            store(e, frame + 0xc, dividend);
            fdiv_routine(e, &mut fpu, frame);
            // The frame's divisor scaled by 2^63; the registers divided as they were.
            let scaled = load(e, frame);
            assert_eq!(scaled.sign_exponent(), 0x21);
            assert_eq!(scaled.significand(), 0x8000_0000_0000_0000);
        });
        assert_eq!(read(&fpu), vec![x87_fdiv(dividend, divisor)]);
        assert_eq!(x87_control_word(&mut e), DEFAULT_CONTROL_WORD);
        // A dividend whose integer bit is clear with an exponent not 0 is not rescaled.
        let dividend = X87Ext::from_parts(0x0005, 0x4000_0000_0000_0000);
        let want = x87_fdiv(dividend, divisor).store64().to_bits();
        assert_eq!(routine(&mut e, dividend, divisor), want);
    }

    /// The table of the register forms as the exe holds it (`011c259e`).
    fn with_table(e: &mut Engine) {
        for (i, (addr, _)) in FRAGMENTS.iter().enumerate() {
            e.mem.set_u32(FDIV_R_TABLE + 4 * i as u32, *addr);
        }
    }

    #[test]
    fn every_register_form_divides_the_registers_it_names() {
        let mut e = engine();
        with_table(&mut e);
        let values: Vec<X87Ext> = [3.0, -7.0, 11.5, 0.25, 1e10, -2e-5, 17.0, 5.0]
            .iter()
            .map(|v| ext(*v))
            .collect();
        for i in 1..8usize {
            for form in [0u32, 2, 4, 5, 6, 7] {
                let mut want = values.clone();
                match form {
                    0 => want[0] = x87_fdiv(values[0], values[i]),
                    2 => want[0] = x87_fdiv(values[i], values[0]),
                    4 => want[i] = x87_fdiv(values[i], values[0]),
                    5 => {
                        want[i] = x87_fdiv(values[i], values[0]);
                        want.remove(0);
                    }
                    6 => want[i] = x87_fdiv(values[0], values[i]),
                    _ => {
                        want[i] = x87_fdiv(values[0], values[i]);
                        want.remove(0);
                    }
                }
                let mut fpu = stack(&values);
                fdiv_registers(&mut e, &mut fpu, 8 * i as u32 + form);
                assert_eq!(read(&fpu), want, "register {i}, form {form}");
            }
        }
        // Register 0: ST0 / ST0, popped by forms 5 and 7.
        for form in [0u32, 2, 4, 5, 6, 7] {
            let mut fpu = stack(&values[..2]);
            fdiv_registers(&mut e, &mut fpu, form);
            let mut want = vec![x87_fdiv(values[0], values[0]), values[1]];
            if form == 5 || form == 7 {
                want.remove(0);
            }
            assert_eq!(read(&fpu), want, "register 0, form {form}");
        }
        // The registered form: EAX, then ST0, ST1, ...; the selector's upper bits ignored.
        let got = e
            .call(0x00ed_6407, &args![0xc0u32 + 8 * 2, 6.0f64, 1.0f64, 4.0f64])
            .f64();
        assert_eq!(got, 1.5);
    }

    #[test]
    #[should_panic(expected = "INT 6 (invalid opcode)")]
    fn an_invalid_register_form_stops_the_engine() {
        let mut e = engine();
        with_table(&mut e);
        let mut fpu = stack(&[ext(1.0), ext(2.0), ext(3.0), ext(4.0), ext(5.0)]);
        fdiv_registers(&mut e, &mut fpu, 8 * 4 + 3);
    }

    #[test]
    fn the_register_and_memory_entries_divide_by_address() {
        let mut e = engine();
        let q = |a: f64, b: f64| x87_fdiv(ext(a), ext(b)).store64().to_bits();
        // ST0 first: 00ed68a6 and 00ed6acc give ST1 / ST0, 00ed68b9 and 00ed6ae1 ST0 / ST1.
        for addr in [0x00ed_68a6u32, 0x00ed_6acc] {
            let got = e.call(addr, &args![4.0f64, 10.0f64]).f64().to_bits();
            assert_eq!(got, q(10.0, 4.0), "{addr:08x}");
        }
        for addr in [0x00ed_68b9u32, 0x00ed_6ae1] {
            let got = e.call(addr, &args![1.0f64, 3.0f64]).f64().to_bits();
            assert_eq!(got, q(1.0, 3.0), "{addr:08x}");
        }
        let st0 = 7.0f64;
        for (addr, operand, m) in [
            (0x00ed_68ccu32, args![st0, 0.5f32], 0.5f64),
            (0x00ed_6918, args![st0, -3.0f64], -3.0),
            (0x00ed_6964, args![st0, 0xabcd_fffdu32], -3.0),
            (0x00ed_6998, args![st0, -40i32], -40.0),
        ] {
            let got = e.call(addr, &operand).f64().to_bits();
            assert_eq!(got, q(st0, m), "{addr:08x}");
        }
        for (addr, operand, m) in [
            (0x00ed_69ccu32, args![st0, 0.5f32], 0.5f64),
            (0x00ed_6a18, args![st0, -3.0f64], -3.0),
            (0x00ed_6a64, args![st0, 0x0000_0009u32], 9.0),
            (0x00ed_6a98, args![st0, 100_000i32], 100_000.0),
        ] {
            let got = e.call(addr, &operand).f64().to_bits();
            assert_eq!(got, q(m, st0), "{addr:08x}");
        }
        // An operand with an exponent of all ones divides at once: a signalling NaN comes back
        // quiet with its payload, infinity gives zero.
        let snan = 0x7f80_0001u32;
        let got = e.call(0x00ed_68cc, &args![st0, snan]).f64().to_bits();
        assert_eq!(
            got,
            x87_fdiv(ext(st0), float_operand(snan)).store64().to_bits()
        );
        assert_eq!(got, 0x7ff8_0000_2000_0000);
        let got = e.call(0x00ed_6a18, &args![st0, f64::INFINITY]).f64();
        assert_eq!(got, f64::INFINITY);
    }

    #[test]
    fn a_full_stack_keeps_its_second_register_aside() {
        let mut e = engine();
        let values: Vec<X87Ext> = (1..=8).map(|v| ext(f64::from(v))).collect();
        let mut fpu = stack(&values);
        fdiv_m32i(&mut e, &mut fpu, 3);
        let mut want = values.clone();
        want[0] = x87_fdiv(values[0], ext(3.0));
        assert_eq!(read(&fpu), want);
        let mut fpu = stack(&values);
        fdivr_m64(&mut e, &mut fpu, 2.0f64.to_bits());
        want[0] = x87_fdiv(ext(2.0), values[0]);
        assert_eq!(read(&fpu), want);
    }

    #[test]
    fn the_partial_remainder_matches_fprem_for_normal_divisors() {
        let mut e = engine();
        for (a, b) in [
            (9.5f64, 2.0f64),
            (-17.25, 4.0),
            (1e300, 3.0),
            (5.0, -0.75),
            (1.0, 0.0),
        ] {
            let (r, status) = x87_fprem(ext(a), ext(b));
            let got = e.call(0x00ed_6cfc, &args![a, b]);
            assert_eq!(got.st0.to_bits(), r.store64().to_bits(), "{a:e} rem {b:e}");
            assert_eq!(got.eax as u16, status & CONDITION_CODES, "{a:e} rem {b:e}");
        }
        // Incomplete for a large exponent difference (C2).
        let got = e.call(0x00ed_6cfc, &args![1e300f64, 3.0f64]);
        assert_ne!(got.eax as u16 & X87_STATUS_C2, 0);
    }

    #[test]
    fn a_denormal_divisor_is_scaled_up_and_the_remainder_back() {
        let mut e = engine();
        e.mem.set_u64(FPREM_DENORMAL_SCALE, 2f64.powi(64).to_bits());
        e.mem.set_u64(FPREM_UNSCALE, 2f64.powi(-64).to_bits());
        let divisor = X87Ext::from_parts(0x0000, 0x0000_0003_0000_0007);
        let dividend = X87Ext::from_parts(0x0005, 0x8000_0000_0000_0001);
        let (r, status) = x87_fprem(dividend, divisor);
        let mut fpu = stack(&[dividend, divisor, ext(42.0)]);
        let got = fprem_adjusted(&mut e, &mut fpu);
        assert_eq!(read(&fpu), vec![r, divisor, ext(42.0)]);
        assert_eq!(got, status & CONDITION_CODES);
        assert_eq!(x87_control_word(&mut e), DEFAULT_CONTROL_WORD);
        // A dividend exponent above 0x7fbe: only the divisor scaled, the remainder not scaled
        // back, and the word the exe loads at the end is the changed one (0x037f).
        let dividend = X87Ext::from_parts(0x7fc0, 0x8000_0000_0000_0000);
        set_control_word(0x037f);
        let scaled = x87_fmul(divisor, ext(2f64.powi(64)));
        let (r, status) = x87_fprem(dividend, scaled);
        set_control_word(DEFAULT_CONTROL_WORD);
        let mut fpu = stack(&[dividend, divisor]);
        let got = fprem_adjusted(&mut e, &mut fpu);
        assert_eq!(read(&fpu), vec![r, divisor]);
        assert_eq!(got, status & CONDITION_CODES);
        assert_eq!(x87_control_word(&mut e), 0x037f);
        set_x87_control_word(&mut e, DEFAULT_CONTROL_WORD);
        // A zero divisor: one FPREM, the invalid result.
        let zero = X87Ext::from_parts(0x0000, 0);
        let (r, status) = x87_fprem(dividend, zero);
        let mut fpu = stack(&[dividend, zero]);
        assert_eq!(fprem_adjusted(&mut e, &mut fpu), status & CONDITION_CODES);
        assert_eq!(read(&fpu), vec![r, zero]);
    }

    /// Recorded from the exe (`nv-call`, FalloutNV.exe 1.4.0.525 unpacked, on an AMD Ryzen AI 9
    /// HX 370): `00ed62f0` with the divisor's and the dividend's words (significand low, high,
    /// sign and exponent), the control word, and the significand and sign-and-exponent of `ST0`
    /// afterwards. Divisors marked in the exe's table (high words `8f..`, `a7..`, `bf..`, `d7..`,
    /// `ef..`), one not marked (`97..`), dividends with the exponent 1, and denormal divisors.
    const ROUTINE_RECORDED: &[([u32; 3], [u32; 3], u16, u64, u16)] = &[
        (
            [0x789abcde, 0x8f123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x027f,
            0x98b0_4ab0_6423_6800,
            0x3ffd,
        ),
        (
            [0x789abcde, 0x8f123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x027f,
            0x02af_1950_1b1a_2800,
            0x0000,
        ),
        (
            [0x789abcde, 0xa7123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x027f,
            0x82c1_3424_ed90_a000,
            0x3ffd,
        ),
        (
            [0x789abcde, 0xa7123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x027f,
            0x024c_656a_a753_4000,
            0x0000,
        ),
        (
            [0x789abcde, 0xbf123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x027f,
            0xe4a9_6c92_28b1_d800,
            0x3ffc,
        ),
        (
            [0x789abcde, 0xbf123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x027f,
            0x0202_7d34_49dc_d000,
            0x0000,
        ),
        (
            [0x789abcde, 0xd7123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x027f,
            0xcb25_2cac_adc0_b000,
            0x3ffc,
        ),
        (
            [0x789abcde, 0xd7123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x027f,
            0x01c9_13a4_856b_7800,
            0x0000,
        ),
        (
            [0x789abcde, 0xef123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x027f,
            0xb6c0_736f_d23c_6800,
            0x3ffc,
        ),
        (
            [0x789abcde, 0xef123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x027f,
            0x019b_3103_bc66_a000,
            0x0000,
        ),
        (
            [0x789abcde, 0x97123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x027f,
            0x909a_5d1a_2530_a800,
            0x3ffd,
        ),
        (
            [0x789abcde, 0x97123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x027f,
            0x028a_b6a2_f6ec_b800,
            0x0000,
        ),
        (
            [0x00000000, 0x00000001, 0x0000],
            [0x00000000, 0x80000000, 0x0001],
            0x027f,
            0x8000_0000_0000_0000,
            0x401e,
        ),
        (
            [0x12345678, 0x00000003, 0x0000],
            [0x00000000, 0x40000000, 0x0000],
            0x027f,
            0xa6b7_0413_649c_1000,
            0x401b,
        ),
        (
            [0x00000000, 0x0f000000, 0x0000],
            [0x9abcdef0, 0x91234567, 0x3fff],
            0x027f,
            0x8000_0000_0000_0000,
            0x7fff,
        ),
        (
            [0x789abcde, 0x8f123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x037f,
            0x98b0_4ab0_6423_679a,
            0x3ffd,
        ),
        (
            [0x789abcde, 0x8f123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x037f,
            0x02af_1950_1b1a_2bfa,
            0x0000,
        ),
        (
            [0x789abcde, 0xa7123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x037f,
            0x82c1_3424_ed90_a081,
            0x3ffd,
        ),
        (
            [0x789abcde, 0xa7123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x037f,
            0x024c_656a_a753_3d88,
            0x0000,
        ),
        (
            [0x789abcde, 0xbf123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x037f,
            0xe4a9_6c92_28b1_d50c,
            0x3ffc,
        ),
        (
            [0x789abcde, 0xbf123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x037f,
            0x0202_7d34_49dc_ceba,
            0x0000,
        ),
        (
            [0x789abcde, 0xd7123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x037f,
            0xcb25_2cac_adc0_ad04,
            0x3ffc,
        ),
        (
            [0x789abcde, 0xd7123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x037f,
            0x01c9_13a4_856b_7b58,
            0x0000,
        ),
        (
            [0x789abcde, 0xef123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x037f,
            0xb6c0_736f_d23c_64cd,
            0x3ffc,
        ),
        (
            [0x789abcde, 0xef123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x037f,
            0x019b_3103_bc66_a065,
            0x0000,
        ),
        (
            [0x789abcde, 0x97123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x037f,
            0x909a_5d1a_2530_a911,
            0x3ffd,
        ),
        (
            [0x789abcde, 0x97123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x037f,
            0x028a_b6a2_f6ec_b64a,
            0x0000,
        ),
        (
            [0x00000000, 0x00000001, 0x0000],
            [0x00000000, 0x80000000, 0x0001],
            0x037f,
            0x8000_0000_0000_0000,
            0x401e,
        ),
        (
            [0x12345678, 0x00000003, 0x0000],
            [0x00000000, 0x40000000, 0x0000],
            0x037f,
            0xa6b7_0413_649c_0e24,
            0x401b,
        ),
        (
            [0x00000000, 0x0f000000, 0x0000],
            [0x9abcdef0, 0x91234567, 0x3fff],
            0x037f,
            0x8000_0000_0000_0000,
            0x7fff,
        ),
        (
            [0x789abcde, 0x8f123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x007f,
            0x98b0_4b00_0000_0000,
            0x3ffd,
        ),
        (
            [0x789abcde, 0x8f123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x007f,
            0x02af_1900_0000_0000,
            0x0000,
        ),
        (
            [0x789abcde, 0xa7123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x007f,
            0x82c1_3400_0000_0000,
            0x3ffd,
        ),
        (
            [0x789abcde, 0xa7123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x007f,
            0x024c_6500_0000_0000,
            0x0000,
        ),
        (
            [0x789abcde, 0xbf123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x007f,
            0xe4a9_6d00_0000_0000,
            0x3ffc,
        ),
        (
            [0x789abcde, 0xbf123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x007f,
            0x0202_7d00_0000_0000,
            0x0000,
        ),
        (
            [0x789abcde, 0xd7123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x007f,
            0xcb25_2d00_0000_0000,
            0x3ffc,
        ),
        (
            [0x789abcde, 0xd7123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x007f,
            0x01c9_1400_0000_0000,
            0x0000,
        ),
        (
            [0x789abcde, 0xef123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x007f,
            0xb6c0_7300_0000_0000,
            0x3ffc,
        ),
        (
            [0x789abcde, 0xef123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x007f,
            0x019b_3100_0000_0000,
            0x0000,
        ),
        (
            [0x789abcde, 0x97123456, 0x3fff],
            [0x55555555, 0xaaaaaaaa, 0x3ffd],
            0x007f,
            0x909a_5d00_0000_0000,
            0x3ffd,
        ),
        (
            [0x789abcde, 0x97123456, 0x4005],
            [0x00000001, 0xc0000000, 0x0001],
            0x007f,
            0x028a_b700_0000_0000,
            0x0000,
        ),
        (
            [0x00000000, 0x00000001, 0x0000],
            [0x00000000, 0x80000000, 0x0001],
            0x007f,
            0x8000_0000_0000_0000,
            0x401e,
        ),
        (
            [0x12345678, 0x00000003, 0x0000],
            [0x00000000, 0x40000000, 0x0000],
            0x007f,
            0xa6b7_0400_0000_0000,
            0x401b,
        ),
        (
            [0x00000000, 0x0f000000, 0x0000],
            [0x9abcdef0, 0x91234567, 0x3fff],
            0x007f,
            0x8000_0000_0000_0000,
            0x7fff,
        ),
    ];

    #[test]
    #[ignore = "needs NV_EXE: the tables and scales are the exe's"]
    fn the_routine_matches_the_recorded_results_on_the_exes_tables() {
        let Some(path) = std::env::var("NV_EXE").ok().filter(|p| !p.is_empty()) else {
            return;
        };
        let mut e = Engine::with_exe(std::path::Path::new(&path)).expect("NV_EXE");
        for &(divisor, dividend, word, significand, sign_exponent) in ROUTINE_RECORDED {
            set_x87_control_word(&mut e, word);
            let mut fpu = Fpu::default();
            e.with_stack(0x20, |e, frame| {
                let frame = frame.addr();
                for (i, w) in divisor.iter().chain(dividend.iter()).enumerate() {
                    e.mem.set_u32(frame + 4 * i as u32, *w);
                }
                fdiv_routine(e, &mut fpu, frame);
            });
            let got = fpu.pop();
            assert_eq!(
                (got.significand(), got.sign_exponent()),
                (significand, sign_exponent),
                "{divisor:x?} / {dividend:x?} under {word:04x}"
            );
            assert_eq!(x87_control_word(&mut e), word);
        }
        set_x87_control_word(&mut e, DEFAULT_CONTROL_WORD);
    }
}
