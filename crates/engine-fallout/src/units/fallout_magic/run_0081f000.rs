//! `fallout/magic/run_0081f000` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c9c1d0` (engine map: `hkpEntity::activate`; `thiscall`, no stack words).
const ACTIVATE_00C9C1D0: u32 = 0x00c9_c1d0;
/// `009d9f40` (`thiscall`, no stack words): the address of the member at +0xe0.
const MEMBER_00E0: u32 = 0x009d_9f40;

// Translated from 0081f1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two stack words, `RET 8`): runs `00c9c1d0` on
/// `this`, then calls the vtable slot +0x50 of the object `009d9f40` answers (the member at
/// +0xe0) with `first` and `second`.
pub fn fn_0081f1e0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(ACTIVATE_00C9C1D0, &args![this]);
    let member = e.call(MEMBER_00E0, &args![this]).u32();
    e.vcall(member, 0x50, &args![first, second]);
}

/// The reference's vtable slot +0x98 (no stack words): an object the function brackets
/// with `00483710` (its scene object; not confirmed here).
const SLOT_98: u32 = 0x98;
/// The float the impulse scale starts at (`010546cc`).
const INITIAL_SCALE: u32 = 0x0105_46cc;
/// `0043b4f0` (`thiscall`, one word: an out word): writes the reference's collision
/// filter word; `0043b4d0` (`thiscall` on that word): its low 7 bits (the layer).
const COLLISION_FILTER: u32 = 0x0043_b4f0;
const FILTER_LAYER: u32 = 0x0043_b4d0;
/// `00403e20` (`thiscall`): the address of a setting's value (+4).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The settings the scale is multiplied by, by layer (see `fn_0081f000`), and the two
/// used after it.
const SETTING_011CF9D4: u32 = 0x011c_f9d4;
const SETTING_011CECAC: u32 = 0x011c_ecac;
const SETTING_011CE3E4: u32 = 0x011c_e3e4;
const SETTING_011CE798: u32 = 0x011c_e798;
const SETTING_011CF114: u32 = 0x011c_f114;
const SETTING_011CE224: u32 = 0x011c_e224;
/// `004b5400` (`thiscall`): a float of the reference in `ST0` (0.0 without the object
/// `004ae750` answers).
const REFERENCE_FLOAT: u32 = 0x004b_5400;
/// `006815c0` (`thiscall`): answers `this` (a point's default constructor).
const POINT_DEFAULT: u32 = 0x0068_15c0;
/// `004a3e00` (`cdecl`, two words: out vector, source point).
const VECTOR_FROM_POINT: u32 = 0x004a_3e00;
/// `0062aea0` (`thiscall` on a vector).
const VECTOR_0062AEA0: u32 = 0x0062_aea0;
/// `005dbf20` (`thiscall`, one float): builds a vector from the float.
const VECTOR_FROM_FLOAT: u32 = 0x005d_bf20;
/// `00627920` (`thiscall` on a vector, one word: another vector's address).
const VECTOR_00627920: u32 = 0x0062_7920;
/// `00483710` (`thiscall`): an empty body.
const EMPTY_00483710: u32 = 0x0048_3710;
/// `004ae750` (`thiscall`): the reference's rigid body object (or null).
const REFERENCE_BODY: u32 = 0x004a_e750;

/// The setting `fn_0081f000` multiplies the scale by for a collision layer (the exe's
/// jump table on `layer - 2`: byte map at `0081f1bc`, targets at `0081f1ac`).
fn layer_setting(layer: u32) -> u32 {
    match layer.wrapping_sub(2) {
        0 | 8 => SETTING_011CF9D4,
        6 | 0x1b => SETTING_011CECAC,
        0xc => SETTING_011CE3E4,
        _ => SETTING_011CE798,
    }
}

// Translated from 0081f000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0081f000 - the /GS stack-cookie check (00ec408c) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0x1c`: two points of three floats and a
/// reference; `this` is not read; the frame is 16-byte aligned). With a reference whose
/// slot +0x98 answers an object, it starts from the float at `010546cc`, multiplies it by
/// a setting chosen by the reference's collision layer (`0043b4f0`, `0043b4d0`: layers 2
/// and 10 `011cf9d4`, 8 and 29 `011cecac`, 14 `011ce3e4`, else `011ce798`), then, when the
/// setting `011cf114` is greater than the reference's float (`004b5400`), by that float
/// divided by the setting, then by the setting `011ce224` (x87, rounded to `float` at each
/// store). It builds a vector from `second` (`004a3e00`), another from `first`, runs
/// `0062aea0` on the first, scales it by the result (`005dbf20`, `00627920`) and hands both
/// to the reference's body (`004ae750`, `0081f1e0`), between two calls of `00483710` on
/// the object.
#[allow(clippy::too_many_arguments)]
pub fn fn_0081f000(
    e: &mut Engine,
    _this: Ptr,
    first_x: f32,
    first_y: f32,
    first_z: f32,
    second_x: f32,
    second_y: f32,
    second_z: f32,
    reference: u32,
) {
    if reference == 0 {
        return;
    }
    let object = e.vcall(reference, SLOT_98, &args![]).u32();
    if object == 0 {
        return;
    }
    e.with_stack(0xa0, |e, frame| {
        // The exe aligns its frame to 16 bytes (`AND ESP,0xfffffff0`).
        let aligned = (frame.addr() + 0xf) & !0xf;
        let filter = aligned;
        let vector = aligned + 0x10;
        let other = aligned + 0x20;
        let scaled = aligned + 0x30;
        // The two points as the caller passed them on the stack.
        let first = aligned + 0x40;
        let second = aligned + 0x4c;
        for (i, value) in [first_x, first_y, first_z, second_x, second_y, second_z]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(first + 4 * i as u32, value.to_bits());
        }
        let mut scale = x87_quiet_bits(e.global::<u32>(INITIAL_SCALE));
        e.call(COLLISION_FILTER, &args![reference, filter]);
        let layer = e.call(FILTER_LAYER, &args![filter]).u32();
        let value = e.call(SETTING_VALUE, &args![layer_setting(layer)]).u32();
        scale = x87_store32(x87_mul(x87_load32(scale), x87_m32(e.mem.u32(value))));
        let float = x87_store32(e.call(REFERENCE_FLOAT, &args![reference]).f64());
        let value = e.call(SETTING_VALUE, &args![SETTING_011CF114]).u32();
        // FLD float; FLD setting; FCOMPP; TEST AH,0x41; JNZ: on only when the setting is
        // greater (ordered).
        if x87_load32(e.mem.u32(value)) > x87_load32(float) {
            let value = e.call(SETTING_VALUE, &args![SETTING_011CF114]).u32();
            let ratio = x87_div(x87_load32(float), x87_m32(e.mem.u32(value)));
            scale = x87_store32(x87_mul(ratio, x87_m32(scale)));
        }
        let value = e.call(SETTING_VALUE, &args![SETTING_011CE224]).u32();
        scale = x87_store32(x87_mul(x87_load32(scale), x87_m32(e.mem.u32(value))));
        e.call(POINT_DEFAULT, &args![vector]);
        e.call(VECTOR_FROM_POINT, &args![vector, second]);
        e.call(POINT_DEFAULT, &args![other]);
        e.call(VECTOR_FROM_POINT, &args![other, first]);
        e.call(VECTOR_0062AEA0, &args![vector]);
        e.call(VECTOR_FROM_FLOAT, &args![scaled, scale]);
        e.call(VECTOR_00627920, &args![vector, scaled]);
        e.call(EMPTY_00483710, &args![object]);
        let body = e.call(REFERENCE_BODY, &args![reference]).u32();
        fn_0081f1e0(e, Ptr::new(body), vector, other);
        e.call(EMPTY_00483710, &args![object]);
    });
}

/// The `MagicHitEffect` constructor `0081b650` (`thiscall`, no stack words).
const MAGIC_HIT_EFFECT_CONSTRUCTOR: u32 = 0x0081_b650;
/// The `MagicShaderHitEffect` vtable (`0107b70c`, RTTI `.?AVMagicShaderHitEffect@@`).
const MAGIC_SHADER_HIT_EFFECT_VTABLE: u32 = 0x0107_b70c;
/// `00822530` and `00822580` (`magicshaderhiteffect.cpp` in the engine map; `thiscall`):
/// construct the arrays at +0x38 and +0x50.
const ARRAY_38_CONSTRUCT: u32 = 0x0082_2530;
const ARRAY_50_CONSTRUCT: u32 = 0x0082_2580;
/// `00633c90` (`thiscall`, one word): constructs a reference-counted pointer from the word.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `0066b0d0` (`thiscall`, one word): assigns the word to a reference-counted pointer.
const POINTER_ASSIGN: u32 = 0x0066_b0d0;

// Translated from 0081f220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0081f220 - the FS:[0] exception frame (destroying the members built so far when a constructor throws) is not translated
/// The `MagicShaderHitEffect` constructor (by the vtable it stores; unnamed in the engine
/// map; `thiscall`, plain `RET`): the `MagicHitEffect` constructor `0081b650`, the vtable,
/// `ParticlePropertyArray` (+0x38, `00822530`), null `spParticleNode` (+0x48,
/// `00633c90`), `AddOnObjects` (+0x50, `00822580`), null `spTexEffectData` (+0x68), then
/// `pEffectData` (+0x30) cleared, +0x68 assigned null (`0066b0d0`), `fAlphaTimer` (+0x34)
/// 0.0, +0x48 assigned null, `bWeaponEnchantment` (+0x28), `pWornObject` (+0x2C) and
/// `b3rdPerson` (+0x4C) cleared, `fAddonAlpha` (+0x60) and `fAddonScale` (+0x64) 1.0 (Xbox
/// PDB names). Returns `this`.
pub fn fn_0081f220(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MAGIC_HIT_EFFECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, MAGIC_SHADER_HIT_EFFECT_VTABLE);
    e.call(ARRAY_38_CONSTRUCT, &args![base + 0x38]);
    e.call(POINTER_CONSTRUCT, &args![base + 0x48, 0u32]);
    e.call(ARRAY_50_CONSTRUCT, &args![base + 0x50]);
    e.call(POINTER_CONSTRUCT, &args![base + 0x68, 0u32]);
    e.mem.set_u32(base + 0x30, 0);
    e.call(POINTER_ASSIGN, &args![base + 0x68, 0u32]);
    e.mem.set_f32(base + 0x34, 0.0);
    e.call(POINTER_ASSIGN, &args![base + 0x48, 0u32]);
    e.mem.set_u8(base + 0x28, 0);
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_u8(base + 0x4c, 0);
    e.mem.set_f32(base + 0x60, 1.0);
    e.mem.set_f32(base + 0x64, 1.0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_f1e0, fn_0081f1e0(Ptr, u32, u32)),
        entry!(
            0x0081_f000,
            fn_0081f000(Ptr, f32, f32, f32, f32, f32, f32, u32)
        ),
        entry!(0x0081_f220, fn_0081f220(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod n049_tests {
    use super::*;

    const SLOT_98_DOUBLE: u32 = 0x0e50_0000;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Settings with the value `value` at each of `011cxxxx` (their value word at +4).
    fn engine(layer: u32, float: f64) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0105_4000, 0x1000);
        e.map(0x011c_e000, 0x2000);
        e.set_global::<f32>(INITIAL_SCALE, 2.0);
        for (setting, value) in [
            (SETTING_011CF9D4, 3.0f32),
            (SETTING_011CECAC, 5.0),
            (SETTING_011CE3E4, 7.0),
            (SETTING_011CE798, 11.0),
            (SETTING_011CF114, 4.0),
            (SETTING_011CE224, 0.5),
        ] {
            e.set_global::<f32>(setting + 4, value);
        }
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SLOT_98, SLOT_98_DOUBLE);
        let reference = e.mem.alloc(0x10);
        e.mem.set_u32(reference, vtable);
        e.register(SLOT_98_DOUBLE, |_, _| 0x3300u32.into_ret());
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(COLLISION_FILTER, |_, a| a[1].into_ret());
        e.register_double(FILTER_LAYER, move |_, _| layer.into_ret());
        e.register_double(REFERENCE_FLOAT, move |_, _| float.into_ret());
        e.register(POINT_DEFAULT, |_, a| a[0].into_ret());
        e.register(VECTOR_FROM_POINT, |_, _| Ret::default());
        e.register(VECTOR_0062AEA0, |_, _| Ret::default());
        e.register(VECTOR_FROM_FLOAT, |_, a| a[0].into_ret());
        e.register(VECTOR_00627920, |_, _| Ret::default());
        e.register(EMPTY_00483710, |_, _| Ret::default());
        e.register(REFERENCE_BODY, |_, _| 0x4400u32.into_ret());
        e.register(ACTIVATE_00C9C1D0, |_, _| Ret::default());
        e.register(MEMBER_00E0, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        (e, reference)
    }

    fn run(layer: u32, float: f64) -> (Engine, u32) {
        let (mut e, reference) = engine(layer, float);
        // The member double answers 0: stop before its vtable call by registering the
        // slot read on a real object.
        let member_vtable = e.mem.alloc(0x60);
        e.mem.set_u32(member_vtable + 0x50, 0x0e50_0100);
        let member = e.mem.alloc(8);
        e.mem.set_u32(member, member_vtable);
        e.register_double(MEMBER_00E0, move |_, _| member.into_ret());
        e.register(0x0e50_0100, |_, _| Ret::default());
        let words = args![0x10u32, 1.0f32, 2.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, reference];
        e.call(0x0081_f000, &words);
        (e, reference)
    }

    #[test]
    fn fn_0081f000_without_reference_or_object_does_nothing() {
        let (mut e, reference) = engine(2, 1.0);
        e.call(0x0081_f000, &[0x10, 0, 0, 0, 0, 0, 0, 0]);
        assert!(calls(&e, COLLISION_FILTER).is_empty());
        e.register(SLOT_98_DOUBLE, |_, _| 0u32.into_ret());
        e.call(0x0081_f000, &[0x10, 0, 0, 0, 0, 0, 0, reference]);
        assert!(calls(&e, COLLISION_FILTER).is_empty());
    }

    #[test]
    fn fn_0081f000_layer_settings_and_ratio() {
        // Layer 2: 2.0 * 3.0; the setting 4.0 > float 2.0: * (2.0 / 4.0); * 0.5.
        let (e, _) = run(2, 2.0);
        let scaled = calls(&e, VECTOR_FROM_FLOAT);
        assert_eq!(scaled[0][1], 1.5f32.to_bits());
        let points = calls(&e, VECTOR_FROM_POINT);
        // The second point first, then the first.
        let second = points[0][1];
        let first = points[1][1];
        assert_eq!(e.mem.f32(second), 4.0);
        assert_eq!(e.mem.f32(second + 8), 6.0);
        assert_eq!(e.mem.f32(first), 1.0);
        assert_eq!(e.mem.f32(first + 8), 3.0);
        // The body gets both vectors.
        assert_eq!(calls(&e, ACTIVATE_00C9C1D0), vec![vec![0x4400]]);
        assert_eq!(calls(&e, 0x0e50_0100)[0][1..], [points[0][0], points[1][0]]);
        assert_eq!(calls(&e, EMPTY_00483710), vec![vec![0x3300], vec![0x3300]]);
    }

    #[test]
    fn fn_0081f000_other_layers() {
        // Layer 10 (`2 + 8`) shares layer 2's setting; float 4.0 is not below 4.0.
        let (e, _) = run(10, 4.0);
        assert_eq!(calls(&e, VECTOR_FROM_FLOAT)[0][1], 3.0f32.to_bits());
        // Layers 8 and 29: 5.0; layer 14: 7.0; anything else (also 0 and 1): 11.0.
        for (layer, value) in [(8, 5.0f32), (29, 5.0), (14, 7.0), (3, 11.0), (1, 11.0)] {
            let (e, _) = run(layer, 9.0);
            assert_eq!(
                calls(&e, VECTOR_FROM_FLOAT)[0][1],
                (2.0 * value * 0.5).to_bits()
            );
        }
    }

    #[test]
    fn fn_0081f220_constructs_the_shader_hit_effect() {
        let mut e = Engine::new();
        for callee in [
            MAGIC_HIT_EFFECT_CONSTRUCTOR,
            ARRAY_38_CONSTRUCT,
            ARRAY_50_CONSTRUCT,
            POINTER_CONSTRUCT,
            POINTER_ASSIGN,
        ] {
            e.register(callee, |_, a| a[0].into_ret());
        }
        let this = e.mem.alloc(0x6c);
        for i in 0..0x1b {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_f220, &[this]).u32(), this);
        assert_eq!(calls(&e, MAGIC_HIT_EFFECT_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(calls(&e, ARRAY_38_CONSTRUCT), vec![vec![this + 0x38]]);
        assert_eq!(calls(&e, ARRAY_50_CONSTRUCT), vec![vec![this + 0x50]]);
        assert_eq!(
            calls(&e, POINTER_CONSTRUCT),
            vec![vec![this + 0x48, 0], vec![this + 0x68, 0]]
        );
        assert_eq!(
            calls(&e, POINTER_ASSIGN),
            vec![vec![this + 0x68, 0], vec![this + 0x48, 0]]
        );
        assert_eq!(e.mem.u32(this), MAGIC_SHADER_HIT_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x28), 0xffff_ff00);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x4c), 0xffff_ff00);
        assert_eq!(e.mem.f32(this + 0x60), 1.0);
        assert_eq!(e.mem.f32(this + 0x64), 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOT_50: u32 = 0x0e10_0000;

    #[test]
    fn fn_0081f1e0_activates_then_calls_the_member_slot() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + 0x50, SLOT_50);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0xe0, vtable);
        e.register(ACTIVATE_00C9C1D0, |_, _| Ret::default());
        e.register(MEMBER_00E0, |_, a| (a[0] + 0xe0).into_ret());
        e.register(SLOT_50, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0081_f1e0, &args![this, 5u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (ACTIVATE_00C9C1D0, vec![this]),
                (MEMBER_00E0, vec![this]),
                (SLOT_50, vec![this + 0xe0, 5, 6]),
            ]
        );
    }
}
