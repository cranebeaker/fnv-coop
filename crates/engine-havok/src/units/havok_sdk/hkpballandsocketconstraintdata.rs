//! `Havok SDK/hkpballandsocketconstraintdata.cpp` (Xbox PDB source unit), subsystem
//! `Havok SDK`: its functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpBallAndSocketConstraintData` is `0x50` bytes: the vtable at `+0`, the atoms from `+0x10`
//! (`0x38` bytes of them), the two pivots (16 bytes each) at `+0x20` and `+0x30`, the 16-bit
//! atom word at `+0x40`, the notified-body bit at `+0x42` (a single bit: `1 << body`), an
//! `hkUFloat8` at `+0x43` and the maximum linear impulse at `+0x44`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the constructor installs.
const BALL_AND_SOCKET_VTABLE: u32 = 0x010d_0cd4;
/// `hkpConstraintData::getConstraintInfoUtil` (`00d21a60`, cdecl `(atoms, size, info)`).
const GET_CONSTRAINT_INFO_UTIL: u32 = 0x00d2_1a60;
/// `hkUFloat8::operator=` (`00ca9360`, `this` = the `hkUFloat8` byte, `(&float)`).
const UFLOAT8_ASSIGN: u32 = 0x00ca_9360;
/// The `float` the constructor stores as the maximum linear impulse.
const DEFAULT_MAX_LINEAR_IMPULSE: u32 = 0x0104_da48;
/// The `double` the runtime-info function compares the maximum linear impulse with.
const UNLIMITED_IMPULSE: u32 = 0x0104_e138;

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

// Translated from 00d541b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallAndSocketConstraintData::setInBodySpace` (named in the engine map): copies the 16
/// bytes at `pivot_a` to `this + 0x20` and the 16 bytes at `pivot_b` to `this + 0x30`.
pub fn hkp_ball_and_socket_constraint_data_set_in_body_space(
    e: &mut Engine,
    this: Ptr,
    pivot_a: Ptr,
    pivot_b: Ptr,
) {
    copy_vec(e, pivot_a.addr(), this.addr() + 0x20);
    copy_vec(e, pivot_b.addr(), this.addr() + 0x30);
}

// Translated from 00d541d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallAndSocketConstraintData::setMaxLinearImpulse` (named in the engine map): stores the
/// `float` at `this + 0x44` (through the x87, so a signalling NaN is quieted).
pub fn hkp_ball_and_socket_constraint_data_set_max_linear_impulse(
    e: &mut Engine,
    this: Ptr,
    impulse: f32,
) {
    e.mem.set_f32(this.addr() + 0x44, x87_quiet(impulse));
}

// Translated from 00d541e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallAndSocketConstraintData::getMaxLinearImpulse` (named in the engine map,
/// `__fastcall`): the `float` at `this + 0x44`, returned in `st0`.
pub fn hkp_ball_and_socket_constraint_data_get_max_linear_impulse(
    e: &mut Engine,
    this: Ptr,
) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x44))
}

// Translated from 00d541f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallAndSocketConstraintData::setBodyToNotify` (named in the engine map): stores the
/// byte `1 << (body & 0x1f)` at `this + 0x42` (an 8-bit shift: zero from body 8 on). Only the
/// low byte of the pushed dword is read.
pub fn hkp_ball_and_socket_constraint_data_set_body_to_notify(
    e: &mut Engine,
    this: Ptr,
    body: u32,
) {
    let bit = 1u32 << (body & 0x1f);
    e.mem.set_u8(this.addr() + 0x42, bit as u8);
}

// Translated from 00d54200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallAndSocketConstraintData::getNotifiedBodyIndex` (named in the engine map,
/// `__fastcall`): the byte at `this + 0x42` shifted right by one (so 0 for the bit of body 0
/// and 1 for body 1).
pub fn hkp_ball_and_socket_constraint_data_get_notified_body_index(
    e: &mut Engine,
    this: Ptr,
) -> u32 {
    u32::from(e.mem.u8(this.addr() + 0x42)) >> 1
}

// Translated from 00d54210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallAndSocketConstraintData::getConstraintInfo` (named in the engine map): fills `info`
/// from the `0x38` bytes of atoms at `this + 0x10` (`getConstraintInfoUtil`).
pub fn hkp_ball_and_socket_constraint_data_get_constraint_info(
    e: &mut Engine,
    this: Ptr,
    info: u32,
) {
    e.call(
        GET_CONSTRAINT_INFO_UTIL,
        &args![this.addr() + 0x10, 0x38u32, info],
    );
}

// Translated from 00d54230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallAndSocketConstraintData::getRuntimeInfo` (named in the engine map): `(size, count)`
/// is written to `info` as `(0, 0)` when `flag` (the low byte of the first word) is zero and
/// the maximum linear impulse equals the exe's `double` at `0104e138` (the `float` maximum,
/// "unlimited"), and as `(0x18, 3)` otherwise (also for a NaN impulse).
pub fn hkp_ball_and_socket_constraint_data_get_runtime_info(
    e: &mut Engine,
    this: Ptr,
    flag: u8,
    info: Ptr,
) {
    let unlimited =
        flag == 0 && f64::from(e.mem.f32(this.addr() + 0x44)) == e.mem.f64(UNLIMITED_IMPULSE);
    let (size, count) = if unlimited { (0, 0) } else { (0x18, 3) };
    e.mem.set_u32(info.addr() + 4, count);
    e.mem.set_u32(info.addr(), size);
}

// Translated from 00d54270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallAndSocketConstraintData::isValid` (named in the engine map): stores 1 in the byte
/// at `valid`.
/// Answers `valid`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_ball_and_socket_constraint_data_is_valid(e: &mut Engine, _this: Ptr, valid: Ptr) -> Ptr {
    e.mem.set_u8(valid.addr(), 1);
    valid
}

// Translated from 00d54280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallAndSocketConstraintData::getType` (named in the engine map): 0.
pub fn hkp_ball_and_socket_constraint_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// Translated from 00d54290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBallAndSocketConstraintData::hkpBallAndSocketConstraintData` (named in the engine map,
/// `__fastcall`): sets the words at `+6` (16 bits, 1), `+8` (0), the vtable, `+0x10` (16 bits,
/// 3), `+0x40` (16 bits, 5) and the notified-body byte `+0x42` (0); stores 1.0 in the
/// `hkUFloat8` at `+0x43` (`00ca9360`, handed the address of a local `float`), copies the
/// default maximum linear impulse (through the x87) to `+0x44` and zeroes both pivots.
/// Returns `this`.
pub fn hkp_ball_and_socket_constraint_data_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0xc`, one push): 0x10 bytes from a
    // 16-byte boundary, the 1.0 at +0xc (00d5429e).
    e.with_stack(0x10 + 0x10, |e, raw| {
        let one = ((raw.addr() + 0xf) & !0xf) + 0xc;
        e.mem.set_f32(one, 1.0);
        e.mem.set_u16(base + 6, 1);
        e.mem.set_u32(base + 8, 0);
        e.mem.set_u32(base, BALL_AND_SOCKET_VTABLE);
        e.mem.set_u16(base + 0x10, 3);
        e.mem.set_u8(base + 0x42, 0);
        e.mem.set_u16(base + 0x40, 5);
        e.call(UFLOAT8_ASSIGN, &args![base + 0x43, one]);
    });
    let impulse = x87_quiet(e.mem.f32(DEFAULT_MAX_LINEAR_IMPULSE));
    e.mem.set_f32(base + 0x44, impulse);
    for offset in [0x20, 0x30] {
        for lane in 0..4 {
            e.mem.set_u32(base + offset + 4 * lane, 0);
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5_41b0,
            hkp_ball_and_socket_constraint_data_set_in_body_space(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d5_41d0,
            hkp_ball_and_socket_constraint_data_set_max_linear_impulse(Ptr, f32)
        ),
        entry!(
            0x00d5_41e0,
            hkp_ball_and_socket_constraint_data_get_max_linear_impulse(Ptr) -> f32
        ),
        entry!(
            0x00d5_41f0,
            hkp_ball_and_socket_constraint_data_set_body_to_notify(Ptr, u32)
        ),
        entry!(
            0x00d5_4200,
            hkp_ball_and_socket_constraint_data_get_notified_body_index(Ptr) -> u32
        ),
        entry!(
            0x00d5_4210,
            hkp_ball_and_socket_constraint_data_get_constraint_info(Ptr, u32)
        ),
        entry!(
            0x00d5_4230,
            hkp_ball_and_socket_constraint_data_get_runtime_info(Ptr, u8, Ptr)
        ),
        entry!(
            0x00d5_4270,
            hkp_ball_and_socket_constraint_data_is_valid(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d5_4280,
            hkp_ball_and_socket_constraint_data_get_type(Ptr) -> u32
        ),
        entry!(
            0x00d5_4290,
            hkp_ball_and_socket_constraint_data_construct(Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument lists of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn set_in_body_space_copies_both_pivots_bit_for_bit() {
        let mut e = engine();
        let data = e.mem.alloc(0x50);
        let pivots = e.mem.alloc(0x20);
        for (i, word) in [1u32, 0x7fa0_0000, 3, 4, 5, 6, 0xffff_ffff, 8]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(pivots + 4 * i as u32, word);
        }
        e.call(0x00d5_41b0, &args![data, pivots, pivots + 0x10]);
        let copied: Vec<u32> = (0..8).map(|i| e.mem.u32(data + 0x20 + 4 * i)).collect();
        assert_eq!(copied, vec![1, 0x7fa0_0000, 3, 4, 5, 6, 0xffff_ffff, 8]);
    }

    #[test]
    fn max_linear_impulse_round_trips_and_quiets_a_signalling_nan() {
        let mut e = engine();
        let data = e.mem.alloc(0x50);
        e.call(0x00d5_41d0, &args![data, 12.5f32]);
        assert_eq!(e.mem.f32(data + 0x44), 12.5);
        assert_eq!(e.call(0x00d5_41e0, &args![data]).st0, 12.5);
        e.call(0x00d5_41d0, &args![data, f32::from_bits(0x7fa0_0000)]);
        assert_eq!(e.mem.u32(data + 0x44), 0x7fe0_0000);
        assert!(e.call(0x00d5_41e0, &args![data]).st0.is_nan());
    }

    #[test]
    fn set_body_to_notify_stores_the_bit_and_the_index_reads_it_back_shifted() {
        let mut e = engine();
        let data = e.mem.alloc(0x50);
        for (body, byte, index) in [(0u32, 1u8, 0u32), (1, 2, 1), (3, 8, 4), (7, 0x80, 0x40)] {
            e.call(0x00d5_41f0, &args![data, body]);
            assert_eq!(e.mem.u8(data + 0x42), byte);
            assert_eq!(e.call(0x00d5_4200, &args![data]).u32(), index);
        }
        // An 8-bit shift: the bit falls off from body 8 on, and the count is taken modulo 32.
        e.call(0x00d5_41f0, &args![data, 8u32]);
        assert_eq!(e.mem.u8(data + 0x42), 0);
        e.call(0x00d5_41f0, &args![data, 33u32]);
        assert_eq!(e.mem.u8(data + 0x42), 2);
    }

    #[test]
    fn get_constraint_info_passes_the_atoms_and_their_size() {
        let mut e = engine();
        e.register_double(GET_CONSTRAINT_INFO_UTIL, |_, _| Ret::default());
        let data = e.mem.alloc(0x50);
        let info = e.mem.alloc(0x10);
        e.call(0x00d5_4210, &args![data, info]);
        assert_eq!(
            calls(&e, GET_CONSTRAINT_INFO_UTIL),
            vec![vec![data + 0x10, 0x38, info]]
        );
    }

    #[test]
    fn get_runtime_info_reports_nothing_only_for_flag_zero_and_the_unlimited_impulse() {
        let mut e = engine();
        e.mem.set_f64(UNLIMITED_IMPULSE, f64::from(f32::MAX));
        let data = e.mem.alloc(0x50);
        let info = e.mem.alloc(8);
        let words = |e: &Engine| (e.mem.u32(info), e.mem.u32(info + 4));
        for (impulse, flag, expected) in [
            (f32::MAX, 0u8, (0, 0)),
            (f32::MAX, 1, (0x18, 3)),
            (f32::MAX, 0x80, (0x18, 3)),
            (10.0, 0, (0x18, 3)),
            (f32::NAN, 0, (0x18, 3)),
        ] {
            e.mem.set_f32(data + 0x44, impulse);
            e.mem.set_u32(info, 0xdead);
            e.mem.set_u32(info + 4, 0xbeef);
            e.call(0x00d5_4230, &args![data, u32::from(flag), info]);
            assert_eq!(words(&e), expected, "impulse {impulse} flag {flag}");
        }
    }

    #[test]
    fn is_valid_stores_one_in_the_byte_only() {
        let mut e = engine();
        let data = e.mem.alloc(0x50);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xaabb_ccdd);
        e.call(0x00d5_4270, &args![data, out]);
        assert_eq!(e.mem.u32(out), 0xaabb_cc01);
    }

    #[test]
    fn fn_00d54270_answers_its_result_pointer() {
        let mut e = engine();
        let data = e.mem.alloc(0x50);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xaabb_ccdd);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(e.call(0x00d5_4270, &args![data, out]).u32(), args![out][0]);
    }

    #[test]
    fn get_type_is_zero() {
        let mut e = engine();
        let data = e.mem.alloc(0x50);
        assert_eq!(e.call(0x00d5_4280, &args![data]).u32(), 0);
    }

    #[test]
    fn construct_sets_the_defaults() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let recorded = Rc::clone(&seen);
        e.register_double(UFLOAT8_ASSIGN, move |e, a| {
            recorded.borrow_mut().push(e.mem.f32(a[1]));
            Ret::default()
        });
        e.mem.set_u32(DEFAULT_MAX_LINEAR_IMPULSE, 0x7fa0_0001);
        let data = e.mem.alloc(0x50);
        for offset in (0..0x50).step_by(4) {
            e.mem.set_u32(data + offset, 0xffff_ffff);
        }
        // The 1.0 sits at +0xc of the exe's aligned 0x10-byte frame (read after the call: the
        // ended frame's bytes stay on the thread's stack).
        let block = e.next_stack_local(0x20);
        let result = e.call(0x00d5_4290, &args![data]);
        assert_eq!(e.mem.f32(((block + 0xf) & !0xf) + 0xc), 1.0);
        assert_eq!(result.u32(), data);
        assert_eq!(e.mem.u16(data + 6), 1);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u32(data), BALL_AND_SOCKET_VTABLE);
        assert_eq!(e.mem.u16(data + 0x10), 3);
        assert_eq!(e.mem.u16(data + 0x40), 5);
        assert_eq!(e.mem.u8(data + 0x42), 0);
        assert_eq!(e.mem.u32(data + 0x44), 0x7fe0_0001);
        for offset in (0x20..0x40).step_by(4) {
            assert_eq!(e.mem.u32(data + offset), 0);
        }
        let assigns = calls(&e, UFLOAT8_ASSIGN);
        assert_eq!(assigns.len(), 1);
        assert_eq!(assigns[0][0], data + 0x43);
        assert_eq!(*seen.borrow(), vec![1.0]);
    }
}
