//! `NiAnimation/nirollcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatController::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOAT_CONTROLLER_RTTI: u32 = 0x011f_4230;

/// `NiRollController::ms_RTTI` (the class's `NiRTTI` record).
const NI_ROLL_CONTROLLER_RTTI: u32 = 0x011f_3f58;

// Translated from 00a485a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_float_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOAT_CONTROLLER_RTTI)
}

// Translated from 00a485b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_roll_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_ROLL_CONTROLLER_RTTI)
}

/// `00a57970`: the jump target of `00a48550`.
const JUMP_TARGET_00A48550: u32 = 0x00a5_7970;

/// `00a57a70`: the jump target of `00a48560`.
const JUMP_TARGET_00A48560: u32 = 0x00a5_7a70;

/// `00a57ad0`: the jump target of `00a48570`.
const JUMP_TARGET_00A48570: u32 = 0x00a5_7ad0;

/// `00a57b00`: the jump target of `00a48580`.
const JUMP_TARGET_00A48580: u32 = 0x00a5_7b00;

/// `00a57b20`: the jump target of `00a48590`.
const JUMP_TARGET_00A48590: u32 = 0x00a5_7b20;

// Translated from 00a48550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::LoadBinary` (Xbox PDB): a 5-byte jump to `00a57970` (engine map: `NiFloatController::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiRollController@@UAAXAAVNiStream@@@Z`.
pub fn ni_roll_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A48550, &args![this, stream]);
}

// Translated from 00a48560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::LinkObject` (Xbox PDB): a 5-byte jump to `00a57a70` (engine map: `NiFloatController::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiRollController@@UAAXAAVNiStream@@@Z`.
pub fn ni_roll_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A48560, &args![this, stream]);
}

// Translated from 00a48570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a57ad0` (engine map: `NiFloatController::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiRollController@@UAA_NAAVNiStream@@@Z`.
pub fn ni_roll_controller_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A48570, &args![this, stream]).bool()
}

// Translated from 00a48580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::SaveBinary` (Xbox PDB): a 5-byte jump to `00a57b00` (engine map: `NiFloatController::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiRollController@@UAAXAAVNiStream@@@Z`.
pub fn ni_roll_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A48580, &args![this, stream]);
}

// Translated from 00a48590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::IsEqual` (Xbox PDB): a 5-byte jump to `00a57b20` (engine map: `NiFloatController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiRollController@@UAA_NPAVNiObject@@@Z`.
pub fn ni_roll_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A48590, &args![this, other]).bool()
}

/// `NiRollController`'s vtable.
const NI_ROLL_CONTROLLER_VTABLE: u32 = 0x0109_8634;
/// The `NiFloatController` destructor body (`00a57990`, `thiscall`), the sized delete
/// (`00aa1460`, `cdecl`, (block, size)) and `operator new` (`00aa13e0`, `cdecl`, size).
const FLOAT_CONTROLLER_DESTRUCT: u32 = 0x00a5_7990;
const SIZED_DELETE: u32 = 0x00aa_1460;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The `NiTimeController` constructor (`00a6d220`, `thiscall`), `DontDoUpdate` (`00a36250`,
/// `thiscall`, the time; a bool) and `GetViewerStrings` (`00a6d830`, `thiscall`, the array).
const TIME_CONTROLLER_CONSTRUCT: u32 = 0x00a6_d220;
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
const TIME_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a6_d830;
/// `NiFloatController::GetAnim` (`00a579f0`, `thiscall`, (&count, &key type, &key size);
/// answers the keys) and its member copy (`00a57b50`, `thiscall`, (clone, cloning)).
const FLOAT_CONTROLLER_GET_ANIM: u32 = 0x00a5_79f0;
const FLOAT_CONTROLLER_COPY_MEMBERS: u32 = 0x00a5_7b50;
/// `NiFloatKey::GenInterp` (`00a26b40`, `cdecl`, (time, keys, key type, key count, &last
/// index, key size); the value in ST0; it reads the low byte of the key size word).
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;
/// `NiMatrix3::operator*` (`0043f8d0`, `thiscall` on the left matrix, (result, right);
/// answers the result).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `NiGetViewerString` (`cdecl`, the text) and the viewer-string array's `SetSize`
/// (`0096ad30`) and `SetAt` (`0096ae90`, (index, address of the string)).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_SET_SIZE: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a485c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatController::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, the flags): the
/// destructor body (`00a57990`), and with bit 0 of `flags` the sized delete of 0x3C bytes.
/// Answers `this`.
pub fn ni_float_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(FLOAT_CONTROLLER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x3cu32]);
    }
    this
}

// Translated from 00a485e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::Update` (Xbox PDB; `thiscall`, the update data, whose first float is
/// the time): with a target (`NiTimeController::m_pkTarget` +0x2C) and `DontDoUpdate` saying
/// to go on, the float keys (`GetAnim`) give the roll at the scaled time (+0x28;
/// `NiFloatKey::GenInterp`, the last index at +0x34); the target's rotation (+0x34, nine
/// floats) is multiplied (`NiMatrix3::operator*`) by the rotation about the z axis by minus
/// the roll (`FSINCOS`: rows (cos, sin, 0), (-sin, cos, 0), (0, 0, 1)) and stored back.
pub fn ni_roll_controller_update(e: &mut Engine, this: Ptr, update: Ptr) {
    let t = this.addr();
    if e.mem.u32(t + 0x2c) == 0 {
        return;
    }
    let time = x87_quiet_bits(e.mem.u32(update.addr()));
    if e.call(DONT_DO_UPDATE, &args![this, time]).bool() {
        return;
    }
    // Locals: the key count, the key size, the key type, the target's rotation, the
    // rotation matrix and the product.
    e.with_stack(0x78, |e, locals| {
        let base = locals.addr();
        let count_slot = base;
        let size_slot = base + 4;
        let type_slot = base + 8;
        let rotation = base + 0xc;
        let roll = base + 0x30;
        let product = base + 0x54;
        let keys = e
            .call(
                FLOAT_CONTROLLER_GET_ANIM,
                &args![this, count_slot, type_slot, size_slot],
            )
            .u32();
        let scaled_time = x87_quiet_bits(e.mem.u32(t + 0x28));
        let size = u32::from(e.mem.u8(size_slot));
        let count = e.mem.u32(count_slot);
        let key_type = e.mem.u32(type_slot);
        let value = e
            .call(
                FLOAT_KEY_GEN_INTERP,
                &args![scaled_time, keys, key_type, count, t + 0x34, size],
            )
            .f64();
        let value = x87_store32(value);
        let angle = x87_store32(-x87_load32(value));
        let target = e.mem.u32(t + 0x2c);
        for word in 0..9 {
            let w = e.mem.u32(target + 0x34 + 4 * word);
            e.mem.set_u32(rotation + 4 * word, w);
        }
        let (sin, cos) = x87_fsincos(X87Ext::load32(angle));
        let cos = cos.store32();
        let sin = sin.store32();
        let zero = x87_store32(0.0);
        let rows = [
            cos,
            sin,
            zero,
            x87_store32(-x87_load32(sin)),
            cos,
            zero,
            zero,
            zero,
            x87_store32(1.0),
        ];
        for (word, w) in rows.iter().enumerate() {
            e.mem.set_u32(roll + 4 * word as u32, *w);
        }
        let result = e
            .call(MATRIX_MULTIPLY, &args![rotation, product, roll])
            .u32();
        for word in 0..9 {
            let w = e.mem.u32(result + 4 * word);
            e.mem.set_u32(rotation + 4 * word, w);
        }
        let target = e.mem.u32(t + 0x2c);
        for word in 0..9 {
            let w = e.mem.u32(rotation + 4 * word);
            e.mem.set_u32(target + 0x34 + 4 * word, w);
        }
    });
}

/// The members `CreateClone` and the creation function give a new object after the
/// `NiTimeController` constructor: no float data, index 0, the class's vtable.
fn init_roll_controller(e: &mut Engine, object: u32) {
    e.call(TIME_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u32(object + 0x38, 0);
    e.mem.set_u32(object + 0x34, 0);
    e.mem.set_u32(object, NI_ROLL_CONTROLLER_VTABLE);
}

// Translated from 00a486f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::CreateClone` (Xbox PDB; `thiscall`, the cloning process): a new
/// 0x3C-byte controller (`operator new`, the `NiTimeController` constructor, no float data,
/// the vtable `01098634`) given this one's members (`00a57b50`, also run with a null clone
/// when the allocation fails). Answers the clone or null.
pub fn ni_roll_controller_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![0x3cu32]).u32();
    if object != 0 {
        init_roll_controller(e, object);
    }
    e.call(FLOAT_CONTROLLER_COPY_MEMBERS, &args![this, object, cloning]);
    Ptr::new(object)
}

// Translated from 00a48750 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiRollController`): a 0x3C-byte controller built as `CreateClone` builds
/// one, or null when the allocation fails.
pub fn fn_00a48750(e: &mut Engine) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![0x3cu32]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    init_roll_controller(e, object);
    Ptr::new(object)
}

// Translated from 00a48790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::GetViewerStrings` (Xbox PDB; `thiscall`, the viewer-string array):
/// `NiTimeController::GetViewerStrings`, then the class name (the first word of the `NiRTTI`
/// record at `011f3f58`) appended: the array grown by its step when full, then `SetAt` at
/// the old size with the address of the string's stack word.
pub fn ni_roll_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    e.call(TIME_CONTROLLER_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(NI_ROLL_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_SET_SIZE, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a485a0, ni_float_controller_get_rtti() -> Ptr),
        entry!(0x00a485b0, ni_roll_controller_get_rtti() -> Ptr),
        entry!(0x00a4_8550, ni_roll_controller_load_binary(Ptr, Ptr)),
        entry!(0x00a4_8560, ni_roll_controller_link_object(Ptr, Ptr)),
        entry!(0x00a4_8570, ni_roll_controller_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a4_8580, ni_roll_controller_save_binary(Ptr, Ptr)),
        entry!(0x00a4_8590, ni_roll_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_85c0,
            ni_float_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a4_85e0, ni_roll_controller_update(Ptr, Ptr)),
        entry!(0x00a4_86f0, ni_roll_controller_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00a4_8750, fn_00a48750() -> Ptr),
        entry!(0x00a4_8790, ni_roll_controller_get_viewer_strings(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_float_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a485a0, &args![]).u32(), NI_FLOAT_CONTROLLER_RTTI);
    }

    #[test]
    fn ni_roll_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a485b0, &args![]).u32(), NI_ROLL_CONTROLLER_RTTI);
    }

    #[test]
    fn thunk_00a48550_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_8550;
        const TARGET: u32 = 0x00a5_7970;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a48560_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_8560;
        const TARGET: u32 = 0x00a5_7a70;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a48570_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_8570;
        const TARGET: u32 = 0x00a5_7ad0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a48580_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_8580;
        const TARGET: u32 = 0x00a5_7b00;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a48590_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_8590;
        const TARGET: u32 = 0x00a5_7b20;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}

#[cfg(test)]
mod n005_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn float_controller_destructor_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.register(FLOAT_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_85c0, &[0x5000, 0]).u32(), 0x5000);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00a4_85c0, &[0x5000, 1]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![0x5000, 0x3c]]);
        assert_eq!(calls_to(&e, FLOAT_CONTROLLER_DESTRUCT).len(), 2);
    }

    /// A roll controller whose float keys give `roll`, with a target whose rotation is the
    /// identity.
    fn rolling(e: &mut Engine, roll: f32) -> (u32, u32, u32) {
        e.register(DONT_DO_UPDATE, |_, a| (a[1] == 0x7f80_0000).into_ret());
        e.register(FLOAT_CONTROLLER_GET_ANIM, |e, a| {
            e.mem.set_u32(a[1], 3);
            e.mem.set_u32(a[2], 1);
            e.mem.set_u8(a[3], 0x0c);
            0x4000u32.into_ret()
        });
        e.register_double(FLOAT_KEY_GEN_INTERP, move |_, _| f64::from(roll).into_ret());
        // The product: the right matrix itself (the left one is the identity).
        e.register(MATRIX_MULTIPLY, |e, a| {
            for word in 0..9 {
                let w = e.mem.u32(a[2] + 4 * word);
                e.mem.set_u32(a[1] + 4 * word, w);
            }
            a[1].into_ret()
        });
        let this = e.mem.alloc(0x3c);
        let target = e.mem.alloc(0x60);
        for i in 0..3 {
            e.mem.set_f32(target + 0x34 + 16 * i, 1.0);
        }
        e.mem.set_u32(this + 0x2c, target);
        e.mem.set_f32(this + 0x28, 0.75);
        let update = e.mem.alloc(4);
        e.mem.set_f32(update, 2.0);
        (this, target, update)
    }

    fn rotation(e: &Engine, target: u32) -> Vec<f32> {
        (0..9).map(|i| e.mem.f32(target + 0x34 + 4 * i)).collect()
    }

    #[test]
    fn update_turns_the_target_by_minus_the_roll() {
        let mut e = Engine::new();
        let (this, target, update) = rolling(&mut e, std::f32::consts::FRAC_PI_2);
        e.call_log = Some(vec![]);
        e.call(0x00a4_85e0, &[this, update]);
        assert_eq!(
            calls_to(&e, DONT_DO_UPDATE),
            vec![vec![this, 2.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, FLOAT_KEY_GEN_INTERP),
            vec![vec![0.75f32.to_bits(), 0x4000, 1, 3, this + 0x34, 0x0c]]
        );
        let angle = -std::f32::consts::FRAC_PI_2;
        let (sin, cos) = x87_fsincos(X87Ext::load32(angle.to_bits()));
        let (sin, cos) = (f32::from_bits(sin.store32()), f32::from_bits(cos.store32()));
        assert_eq!(
            rotation(&e, target),
            vec![cos, sin, 0.0, -sin, cos, 0.0, 0.0, 0.0, 1.0]
        );
        let multiply = calls_to(&e, MATRIX_MULTIPLY);
        assert_eq!(multiply.len(), 1);
        // The left matrix is the copy of the target's rotation.
        assert_ne!(multiply[0][0], target + 0x34);
    }

    #[test]
    fn update_skips_without_target_or_when_told() {
        let mut e = Engine::new();
        let (this, target, update) = rolling(&mut e, 1.0);
        e.mem.set_u32(update, 0x7f80_0000);
        e.call_log = Some(vec![]);
        e.call(0x00a4_85e0, &[this, update]);
        assert!(calls_to(&e, FLOAT_CONTROLLER_GET_ANIM).is_empty());
        assert_eq!(rotation(&e, target)[0], 1.0);
        e.mem.set_u32(this + 0x2c, 0);
        e.call(0x00a4_85e0, &[this, update]);
        assert_eq!(calls_to(&e, DONT_DO_UPDATE).len(), 1);
    }

    #[test]
    fn create_clone_and_creation_build_a_roll_controller() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in (0..a[0]).step_by(4) {
                e.mem.set_u32(block + offset, 0xffff_ffff);
            }
            block.into_ret()
        });
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(FLOAT_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a4_86f0, &[0x5000, 0x6000]).u32();
        assert_eq!(e.mem.u32(clone), NI_ROLL_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(clone + 0x34), 0);
        assert_eq!(e.mem.u32(clone + 0x38), 0);
        assert_eq!(
            calls_to(&e, FLOAT_CONTROLLER_COPY_MEMBERS),
            vec![vec![0x5000, clone, 0x6000]]
        );
        let made = e.call(0x00a4_8750, &[]).u32();
        assert_eq!(e.mem.u32(made), NI_ROLL_CONTROLLER_VTABLE);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_CONSTRUCT),
            vec![vec![clone], vec![made]]
        );
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_86f0, &[0x5000, 0x6000]).u32(), 0);
        assert_eq!(
            calls_to(&e, FLOAT_CONTROLLER_COPY_MEMBERS),
            vec![vec![0x5000, 0, 0x6000]]
        );
        assert_eq!(e.call(0x00a4_8750, &[]).u32(), 0);
        assert!(calls_to(&e, TIME_CONTROLLER_CONSTRUCT).is_empty());
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = Engine::new();
        e.map(NI_ROLL_CONTROLLER_RTTI & !0xfff, 0x1000);
        e.set_global(NI_ROLL_CONTROLLER_RTTI, 0x0444_0000u32);
        e.register(TIME_CONTROLLER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0444_0001);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a4_8790, &[0x5000, strings]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_GET_VIEWER_STRINGS),
            vec![vec![0x5000, strings]]
        );
        assert!(calls_to(&e, STRINGS_SET_SIZE).is_empty());
        assert_eq!(calls_to(&e, STRINGS_SET_AT)[0][..2], [strings, 1]);
    }
}
