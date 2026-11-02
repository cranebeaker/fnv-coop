//! `Havok SDK/run_00df6380` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_00df06a0::{
    addps, maxps, movaps, movhlps, movlhps, mulps, rcpps, shufps, splat0, subps, sum3, unpckhps,
    unpcklps,
};

// Translated from 00df6470 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x350-byte stack buffer by calling `00c93d10` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00df6470(e: &mut Engine) -> u32 {
    e.with_stack(0x350, |e, buffer| {
        e.call(0x00c9_3d10, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

/// The vector `010c7200`: the lower bound of the divisor.
const DIVISOR_FLOOR: u32 = 0x010c_7200;

// Translated from 00df6380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Thiscall with one stack word `pair` (two 16-byte
/// vectors `u` at `+0` and `v` at `+0x10`), answers a float in `ST0`. `this + 0x74` points at
/// three 16-byte rows `r0..r2` (the 3x3 matrix is used transposed): `s_j = (v1 * r_j1 + v0 *
/// r_j0) + v2 * r_j2` for `j = 0..2`; `m` is the vector at `this + 0x60` raised to at least the
/// vector `010c7200` (`MAXPS`); `R = 2r - (r * m) * r` with `r` the `RCPPS` estimate of `m`
/// (the host CPU's, tests allow one ulp); the answer is the lane sum `x2 + (x1 + x0)` of
/// `R3 * u^2 + s^2 * R` (`R3` lane 3 of `R`), lane by lane, quieted by the `FLD`.
pub fn fn_00df6380(e: &mut Engine, this: Ptr, pair: Ptr) -> f32 {
    let me = this.addr();
    let rows = e.mem.u32(me.wrapping_add(0x74));
    let r0 = movaps(e, rows);
    let r1 = movaps(e, rows.wrapping_add(0x10));
    let r2 = movaps(e, rows.wrapping_add(0x20));
    let low = unpcklps(r0, r1);
    let high = unpckhps(r0, r1);
    let x4 = movlhps(low, r2);
    let x2 = shufps(movhlps(x4, low), r2, 0xd4);
    let v = movaps(e, pair.addr().wrapping_add(0x10));
    let mut sum = mulps(shufps(v, v, 0x55), x2);
    let v2 = shufps(v, v, 0xaa);
    sum = addps(sum, mulps(shufps(v, v, 0), x4));
    let divisor = movaps(e, me.wrapping_add(0x60));
    let x3 = shufps(high, r2, 0xe4);
    let x3 = mulps(x3, v2);
    let floor = movaps(e, DIVISOR_FLOOR);
    let divisor = maxps(divisor, floor);
    sum = addps(sum, x3);
    let estimate = rcpps(divisor);
    let refined_term = mulps(mulps(estimate, divisor), estimate);
    let u = movaps(e, pair.addr());
    let refined = subps(addps(estimate, estimate), refined_term);
    let sum = mulps(sum, sum);
    let sum = mulps(sum, refined);
    let u = mulps(u, u);
    let scale = splat0(shufps(refined, refined, 0xff));
    let total = addps(mulps(scale, u), sum);
    x87_quiet(sum3(total))
}

// Translated from 00df6440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc0a2f): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00c93d10` (the `CALL` at 00df644d) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00df6440(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00c9_3d10, &args![object.addr(), finishing]);
    }
}

// Translated from 00df6460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc0a39): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00df6460(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00df64a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b94ec): cdecl with one stack word (never read); only returns.
pub fn fn_00df64a0(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00df6540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b9504): cdecl with one stack word (never read); only returns.
pub fn fn_00df6540(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00df64d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b94e8): cdecl with two stack words, `object` and `finishing`. When neither is 0, clears the 16-bit values at `+0x20` and `+0x22` and the words at `+0x24`, `+0x28` and `+0x10`, and stores 1 in the word at its start.
pub fn fn_00df64d0(e: &mut Engine, object: Ptr, finishing: u32) {
    let base = object.addr();
    if base == 0 || finishing == 0 {
        return;
    }
    e.mem.set_u16(base + 0x20, 0);
    e.mem.set_u16(base + 0x22, 0);
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 0x28, 0);
    e.mem.set_u32(base, 1);
    e.mem.set_u32(base + 0x10, 0);
}

// Translated from 00df6500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b9500): cdecl with two stack words, `object` and `finishing`. When neither is 0, clears the words at `+0x44`, `+0x48` and `+0x30` and the 16-bit values at `+0x40` and `+0x42`, stores 1 in the word at `+0x20`, and stores in the byte at `+0x19` the low byte of `object` minus the low byte of `object + 0x14` (`SUB AL, CL` with `ECX = object + 0x14`), which is always 0xEC.
pub fn fn_00df6500(e: &mut Engine, object: Ptr, finishing: u32) {
    let base = object.addr();
    if base == 0 || finishing == 0 {
        return;
    }
    e.mem.set_u32(base + 0x44, 0);
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u16(base + 0x40, 0);
    e.mem.set_u16(base + 0x42, 0);
    e.mem.set_u32(base + 0x20, 1);
    let inner = base.wrapping_add(0x14);
    e.mem
        .set_u8(inner + 5, (base as u8).wrapping_sub(inner as u8));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_6380, fn_00df6380(Ptr, Ptr) -> f32),
        entry!(0x00df_6470, fn_00df6470() -> u32),
        entry!(0x00df6440, fn_00df6440(Ptr, u32)),
        entry!(0x00df6460, fn_00df6460(Ptr) -> u32),
        entry!(0x00df64a0, fn_00df64a0(u32)),
        entry!(0x00df6540, fn_00df6540(u32)),
        entry!(0x00df64d0, fn_00df64d0(Ptr, u32)),
        entry!(0x00df6500, fn_00df6500(Ptr, u32)),
    ]
}

#[cfg(test)]
mod a07_first_word_tests {
    use crate::prelude::*;

    const BUILT_WORD: u32 = 0x0abc_def0;

    /// A double over `callee` writes `BUILT_WORD` at the buffer it is given; `function`
    /// must call it once with that buffer and a zero flag word, and return the word.
    fn check(function: u32, callee: u32) {
        let mut e = Engine::new();
        e.register(callee, |e, a| {
            e.mem.set_u32(a[0], BUILT_WORD);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(function, &args![]).u32();
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == callee).collect();
        assert_eq!(calls.len(), 1);
        assert_ne!(calls[0].1[0], 0);
        assert_eq!(calls[0].1[1], 0);
        assert_eq!(result, BUILT_WORD);
    }

    macro_rules! first_word_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check($function, $callee);
            }
        };
    }

    first_word_test!(
        fn_00df6470_returns_first_word_of_built_object,
        0x00df_6470,
        0x00c9_3d10
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type ConstructorLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Double for a constructor: logs its argument words and writes `vtable` at the start of `this`.
    fn constructor_double(e: &mut Engine, address: u32, vtable: u32) -> ConstructorLog {
        let log: ConstructorLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0], vtable);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn fn_00df6470_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00c9_3d10, 0x0a0b_0c0d);
        let result = e.call(0x00df_6470, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }
}

#[cfg(test)]
mod quadratic_tests {
    use super::*;

    struct Rig {
        e: Engine,
        this: u32,
        pair: u32,
    }

    fn put(e: &mut Engine, at: u32, lanes: [f32; 4]) {
        for (lane, value) in lanes.iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *value);
        }
    }

    /// Rows `[1,2,3]`, `[4,5,6]`, `[7,8,9]`, `u = [1,2,3]`, `v = [1,0.5,2]`, the divisor
    /// `divisor` and the floor `floor`.
    fn rig(divisor: [f32; 4], floor: [f32; 4]) -> Rig {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let rows = e.mem.alloc(0x30);
        let pair = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x74, rows);
        put(&mut e, rows, [1.0, 2.0, 3.0, 0.0]);
        put(&mut e, rows + 0x10, [4.0, 5.0, 6.0, 0.0]);
        put(&mut e, rows + 0x20, [7.0, 8.0, 9.0, 0.0]);
        put(&mut e, this + 0x60, divisor);
        put(&mut e, pair, [1.0, 2.0, 3.0, 0.0]);
        put(&mut e, pair + 0x10, [1.0, 0.5, 2.0, 0.0]);
        e.map(0x0100_0000, 0x0030_0000);
        put(&mut e, 0x010c_7200, floor);
        Rig { e, this, pair }
    }

    fn close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() <= expected.abs() * 1e-5,
            "{actual} vs {expected}"
        );
    }

    #[test]
    fn the_sum_weights_the_squared_terms_by_the_reciprocals() {
        let mut r = rig([2.0, 4.0, 0.5, 8.0], [0.001; 4]);
        let answer = r.e.call(0x00df_6380, &args![r.this, r.pair]).f32();
        // s = [8, 18.5, 29]; 14 / 8 + 64 / 2 + 342.25 / 4 + 841 / 0.5.
        close(answer, 1.75 + 32.0 + 85.5625 + 1682.0);
    }

    #[test]
    fn a_small_divisor_is_raised_to_the_floor_first() {
        let mut r = rig([-1.0, 0.0, f32::NAN, 0.25], [0.5, 0.5, 0.5, 0.5]);
        let answer = r.e.call(0x00df_6380, &args![r.this, r.pair]).f32();
        // Every lane of the divisor becomes 0.5 (a NaN gives the floor), except lane 3: 0.5 too
        // (0.25 < 0.5): 14 * 2 + (64 + 342.25 + 841) * 2.
        close(answer, 28.0 + 2.0 * (64.0 + 342.25 + 841.0));
    }

    #[test]
    fn the_answer_is_a_float() {
        let mut r = rig([1.0; 4], [0.001; 4]);
        let direct = fn_00df6380(&mut r.e, Ptr::new(r.this), Ptr::new(r.pair));
        close(direct, 14.0 + 64.0 + 342.25 + 841.0);
    }
}

#[cfg(test)]
mod tests_a07_gap {
    use super::*;

    /// The argument words of every call to `address` in the engine's call log (which is taken).
    fn calls_to(e: &mut Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words)
            .collect()
    }

    /// The address the doubled virtual slot 0 of [`object_with_slot_0`] answers at.
    const SLOT_0_TARGET: u32 = 0x0900_0001;

    /// An object whose vtable's slot 0 is a double answering 0x1234.
    fn object_with_slot_0(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable, SLOT_0_TARGET);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.register(SLOT_0_TARGET, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        object
    }

    #[test]
    fn fn_00df6440_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00c9_3d10, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00df_6440, &args![0u32, 7u32]);
        e.call(0x00df_6440, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00c9_3d10), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00df6460_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_6460, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00df64a0_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_64a0, &args![0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00df6540_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_6540, &args![0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00df64d0_resets_the_fields_only_when_finishing_a_real_object() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        for offset in (0..0x30).step_by(4) {
            e.mem.set_u32(object + offset, 0x5a5a_5a5a);
        }
        e.call(0x00df_64d0, &args![0u32, 1u32]);
        e.call(0x00df_64d0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x5a5a_5a5a);
        e.call(0x00df_64d0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 1);
        assert_eq!(e.mem.u32(object + 0x10), 0);
        assert_eq!(e.mem.u32(object + 0x20), 0);
        assert_eq!(e.mem.u32(object + 0x24), 0);
        assert_eq!(e.mem.u32(object + 0x28), 0);
        assert_eq!(e.mem.u32(object + 0x14), 0x5a5a_5a5a);
    }

    #[test]
    fn fn_00df6500_resets_the_fields_and_writes_the_offset_byte() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x60);
        for offset in (0..0x50).step_by(4) {
            e.mem.set_u32(object + offset, 0x5a5a_5a5a);
        }
        e.call(0x00df_6500, &args![0u32, 1u32]);
        e.call(0x00df_6500, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object + 0x44), 0x5a5a_5a5a);
        e.call(0x00df_6500, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object + 0x44), 0);
        assert_eq!(e.mem.u32(object + 0x48), 0);
        assert_eq!(e.mem.u32(object + 0x30), 0);
        assert_eq!(e.mem.u32(object + 0x40), 0);
        assert_eq!(e.mem.u32(object + 0x20), 1);
        assert_eq!(e.mem.u8(object + 0x19), 0xec);
        assert_eq!(e.mem.u32(object + 0x34), 0x5a5a_5a5a);
    }
}
