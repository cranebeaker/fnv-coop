//! `fallout/ai/run_008d1b30` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The globals `008d1b30` works on: an object (`011dfecc`) and two
/// `float`s, the value it answers (`011dfed4`) and the time left
/// (`011dfed8`); the `double` 0.0 they are compared with (`01012060`).
const OBJECT: u32 = 0x011d_fecc;
const VALUE: u32 = 0x011d_fed4;
const TIME_LEFT: u32 = 0x011d_fed8;
const ZERO: u32 = 0x0101_2060;
/// The object's first entry (`0043b230`, `thiscall`), the float that entry
/// answers in ST0 (`0047c860`, `thiscall`; the engine map's name is a
/// folded body's), the 12-byte local's constructor (`0043d410`, `thiscall
/// (this, float, 1, 0)`), the object's setter given it (`00a59c60`,
/// `thiscall (object, &local)`), the object's rotation (`006a9540`,
/// `thiscall`, answers 9 words), `NiMatrix3::Inverse` (`004b45b0`,
/// `thiscall (matrix, &result)`), the matrix product (`0043f8d0`,
/// `thiscall (matrix, &result, other)`), the frame time (`0084d030` on
/// `011f6394`, ST0) and the setting whose `float` value bounds the time
/// (`00403e20` on `011d08b0`, answers its address).
const FIRST_ENTRY: u32 = 0x0043_b230;
const ENTRY_FLOAT: u32 = 0x0047_c860;
const LOCAL_CONSTRUCTOR: u32 = 0x0043_d410;
const OBJECT_SET: u32 = 0x00a5_9c60;
const OBJECT_ROTATION: u32 = 0x006a_9540;
const MATRIX_INVERSE: u32 = 0x004b_45b0;
const MATRIX_PRODUCT: u32 = 0x0043_f8d0;
const FRAME_TIME: u32 = 0x0084_d030;
const FRAME_TIME_OWNER: u32 = 0x011f_6394;
const SETTING_VALUE: u32 = 0x0040_3e20;
const TIME_SETTING: u32 = 0x011d_08b0;

fn global_float(e: &Engine, address: u32) -> f64 {
    x87_load32(e.global::<u32>(address))
}

/// Both globals zeroed (`FLDZ`/`FSTP`), the value answered.
fn clear_and_answer(e: &mut Engine) -> f32 {
    e.set_global(TIME_LEFT, 0.0f32);
    e.set_global(VALUE, 0.0f32);
    f32::from_bits(x87_quiet_bits(e.global::<u32>(VALUE)))
}

// Translated from 008d1b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates a timed rotation of the object at `011dfecc` and answers a
/// value (the engine map has no name; `cdecl (angle, &rotation)`). With
/// the object and both globals nonzero: the object is set from its first
/// entry's float (through `0043d410` with 1 and 0) and its rotation kept,
/// then set from `angle` the same way; the inverse of the kept rotation
/// times the new rotation is written to `rotation` (9 words). The time
/// left loses the frame time; below zero both globals become 0 and 0 is
/// answered; when the bounding setting's value is greater than the time
/// left, `value * time left / setting` (x87, the product kept as a
/// `double`) is answered, else the value. Without the object, or with
/// either global zero: both globals become 0 unless both already are; the
/// value is answered.
pub fn fn_008d1b30(e: &mut Engine, angle: f32, rotation: Ptr) -> f32 {
    let zero = f64::from_bits(e.global::<u64>(ZERO));
    let object = e.global::<u32>(OBJECT);
    if object == 0 || global_float(e, VALUE) == zero || global_float(e, TIME_LEFT) == zero {
        if global_float(e, TIME_LEFT) != zero || global_float(e, VALUE) != zero {
            return clear_and_answer(e);
        }
        return f32::from_bits(x87_quiet_bits(e.global::<u32>(VALUE)));
    }
    e.with_stack(0x98, |e, frame| {
        // The exe's locals from `EBP - 0x98`: the second product +0xC,
        // the inverse +0x30, the kept rotation +0x54, the two 12-byte
        // locals +0x80 and +0x8C.
        let frame = frame.addr();
        let product = frame + 0xc;
        let inverse = frame + 0x30;
        let kept = frame + 0x54;
        let first_local = frame + 0x80;
        let second_local = frame + 0x8c;
        let entry = e.call(FIRST_ENTRY, &args![object]).u32();
        let entry_value = x87_store32(e.call(ENTRY_FLOAT, &args![entry]).f64());
        e.call(
            LOCAL_CONSTRUCTOR,
            &args![first_local, entry_value, 1u32, 0u32],
        );
        let object = e.global::<u32>(OBJECT);
        e.call(OBJECT_SET, &args![object, first_local]);
        let object = e.global::<u32>(OBJECT);
        let current = e.call(OBJECT_ROTATION, &args![object]).u32();
        for i in 0..9 {
            let value = e.mem.u32(current + 4 * i);
            e.mem.set_u32(kept + 4 * i, value);
        }
        e.call(
            LOCAL_CONSTRUCTOR,
            &args![second_local, x87_quiet_bits(angle.to_bits()), 1u32, 0u32],
        );
        let object = e.global::<u32>(OBJECT);
        e.call(OBJECT_SET, &args![object, second_local]);
        let object = e.global::<u32>(OBJECT);
        let current = e.call(OBJECT_ROTATION, &args![object]).u32();
        let inverted = e.call(MATRIX_INVERSE, &args![kept, inverse]).u32();
        let result = e
            .call(MATRIX_PRODUCT, &args![inverted, product, current])
            .u32();
        for i in 0..9 {
            let value = e.mem.u32(result + 4 * i);
            e.mem.set_u32(rotation.addr().wrapping_add(4 * i), value);
        }
    });
    let frame_time = e.call(FRAME_TIME, &args![FRAME_TIME_OWNER]).f64();
    let left = x87_store32(x87_sub(x87_m32(e.global::<u32>(TIME_LEFT)), frame_time));
    e.set_global(TIME_LEFT, left);
    let left = global_float(e, TIME_LEFT);
    if left < zero {
        e.set_global(TIME_LEFT, 0.0f32);
        e.set_global(VALUE, 0.0f32);
        return f32::from_bits(x87_quiet_bits(e.global::<u32>(VALUE)));
    }
    let bound = e.call(SETTING_VALUE, &args![TIME_SETTING]).u32();
    let bound = x87_load32(e.mem.u32(bound));
    if bound > left {
        let scaled = x87_mul(global_float(e, VALUE), x87_m32(e.global::<u32>(TIME_LEFT)));
        let bound = e.call(SETTING_VALUE, &args![TIME_SETTING]).u32();
        let bound = x87_load32(e.mem.u32(bound));
        let answer = x87_store32(x87_div(scaled, bound));
        return f32::from_bits(x87_quiet_bits(answer));
    }
    f32::from_bits(x87_quiet_bits(e.global::<u32>(VALUE)))
}

// Translated from 008d1cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at `011dfed4` (the engine map has no name).
pub fn fn_008d1cf0(e: &mut Engine) -> f32 {
    f32::from_bits(x87_quiet_bits(e.global::<u32>(VALUE)))
}

// Translated from 008d1d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at `011dfed8` (the engine map has no name).
pub fn fn_008d1d00(e: &mut Engine) -> f32 {
    f32::from_bits(x87_quiet_bits(e.global::<u32>(TIME_LEFT)))
}

// Translated from 008d1d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the two floats (cdecl) in the globals at `011dfed4` and `011dfed8`
/// (`FLD`/`FSTP`, so a signalling NaN is stored quiet).
pub fn fn_008d1d10(e: &mut Engine, value: f32, time_left: f32) {
    e.mem.set_u32(VALUE, x87_quiet_bits(value.to_bits()));
    e.mem
        .set_u32(TIME_LEFT, x87_quiet_bits(time_left.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_1d10, fn_008d1d10(f32, f32)),
        entry!(0x008d_1b30, fn_008d1b30(f32, Ptr) -> f32),
        entry!(0x008d_1cf0, fn_008d1cf0() -> f32),
        entry!(0x008d_1d00, fn_008d1d00() -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timed_engine(object: u32, value: f32, left: f32) -> Engine {
        let mut e = Engine::new();
        for address in [OBJECT, VALUE, TIME_LEFT] {
            e.map(address, 4);
        }
        e.map(ZERO, 8);
        e.set_global(ZERO, 0.0f64);
        e.set_global(OBJECT, object);
        e.set_global(VALUE, value);
        e.set_global(TIME_LEFT, left);
        e
    }

    /// Doubles for the full path: the frame time `frame` and the bounding
    /// setting's value `bound`.
    fn timed_doubles(e: &mut Engine, frame: f64, bound: f32) {
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, bound);
        let rotation = e.mem.alloc(0x24);
        e.register(FIRST_ENTRY, |_, _| 0x4000u32.into_ret());
        e.register(ENTRY_FLOAT, |_, _| 0.5f64.into_ret());
        e.register(LOCAL_CONSTRUCTOR, |_, _| Ret::default());
        e.register(OBJECT_SET, |_, _| Ret::default());
        e.register_double(OBJECT_ROTATION, move |_, _| rotation.into_ret());
        e.register(MATRIX_INVERSE, |_, a| a[1].into_ret());
        e.register(MATRIX_PRODUCT, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[1] + 4 * i, 0x100 + i);
            }
            a[1].into_ret()
        });
        e.register_double(FRAME_TIME, move |_, _| frame.into_ret());
        e.register_double(SETTING_VALUE, move |_, _| setting.into_ret());
    }

    #[test]
    fn without_the_object_the_globals_are_cleared_unless_already_zero() {
        let mut e = timed_engine(0, 2.0, 0.0);
        let out = e.mem.alloc(0x24);
        assert_eq!(e.call(0x008d_1b30, &args![1.0f32, out]).f32(), 0.0);
        assert_eq!(e.global::<f32>(VALUE), 0.0);
        let mut e = timed_engine(0, 0.0, 0.0);
        assert_eq!(e.call(0x008d_1b30, &args![1.0f32, out]).f32(), 0.0);
        // A zero value with the object: also cleared.
        let mut e = timed_engine(0x9000, 0.0, 3.0);
        assert_eq!(e.call(0x008d_1b30, &args![1.0f32, out]).f32(), 0.0);
        assert_eq!(e.global::<f32>(TIME_LEFT), 0.0);
    }

    #[test]
    fn running_time_scales_the_value_and_writes_the_rotation() {
        let mut e = timed_engine(0x9000, 2.0, 1.0);
        timed_doubles(&mut e, 0.25, 1.5);
        e.call_log = Some(vec![]);
        let out = e.mem.alloc(0x24);
        // 2 * 0.75 / 1.5.
        assert_eq!(e.call(0x008d_1b30, &args![3.0f32, out]).f32(), 1.0);
        assert_eq!(e.global::<f32>(TIME_LEFT), 0.75);
        assert_eq!(e.global::<f32>(VALUE), 2.0);
        for i in 0..9 {
            assert_eq!(e.mem.u32(out + 4 * i), 0x100 + i);
        }
        let log = e.call_log.take().unwrap();
        let locals: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOCAL_CONSTRUCTOR)
            .collect();
        assert_eq!(locals[0].1[1..], [0.5f32.to_bits(), 1, 0]);
        assert_eq!(locals[1].1[1..], [3.0f32.to_bits(), 1, 0]);
        assert!(log
            .iter()
            .any(|call| call.0 == FRAME_TIME && call.1 == vec![FRAME_TIME_OWNER]));
    }

    #[test]
    fn time_past_the_bound_answers_the_value_and_running_out_clears() {
        let mut e = timed_engine(0x9000, 2.0, 1.0);
        timed_doubles(&mut e, 0.25, 0.5);
        let out = e.mem.alloc(0x24);
        assert_eq!(e.call(0x008d_1b30, &args![3.0f32, out]).f32(), 2.0);
        let mut e = timed_engine(0x9000, 2.0, 1.0);
        timed_doubles(&mut e, 2.0, 0.5);
        let out = e.mem.alloc(0x24);
        assert_eq!(e.call(0x008d_1b30, &args![3.0f32, out]).f32(), 0.0);
        assert_eq!(e.global::<f32>(TIME_LEFT), 0.0);
        assert_eq!(e.global::<f32>(VALUE), 0.0);
    }

    #[test]
    fn the_two_getters_answer_the_globals() {
        let mut e = timed_engine(0, 1.25, 2.5);
        assert_eq!(e.call(0x008d_1cf0, &args![]).f32(), 1.25);
        assert_eq!(e.call(0x008d_1d00, &args![]).f32(), 2.5);
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    #[test]
    fn fn_008d1d10_stores_both_floats_quiet() {
        let mut e = Engine::new();
        e.map(VALUE, 8);
        e.call(0x008d_1d10, &[1.5f32.to_bits(), 0x7f80_0001]);
        assert_eq!(e.mem.f32(VALUE), 1.5);
        assert_eq!(e.mem.u32(TIME_LEFT), 0x7fc0_0001);
    }
}
