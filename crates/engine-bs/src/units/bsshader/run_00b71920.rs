//! `BSShader/run_00b71920` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The three doubles `00b719e0` weighs the colour channels with
/// (`FMUL qword`): for +0x08, +0x00 and +0x04.
const WEIGHT_2_010AE8B0: u32 = 0x010a_e8b0;
const WEIGHT_0_010AE8B8: u32 = 0x010a_e8b8;
const WEIGHT_1_010AE8C0: u32 = 0x010a_e8c0;
/// Makes the object `00b71a10` keeps at +0x30 (`00e7f5d0`, no arguments).
const MAKE_OBJECT_00E7F5D0: u32 = 0x00e7_f5d0;
/// The object's method `00e7f430` (thiscall, three words).
const OBJECT_METHOD_00E7F430: u32 = 0x00e7_f430;
/// The point `00b71a40` subtracts from the translation (three floats).
const ORIGIN_X: u32 = 0x011f_474c;
const ORIGIN_Y: u32 = 0x011f_4750;
const ORIGIN_Z: u32 = 0x011f_4754;

// Translated from 00b719c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two doubles): the smaller of `a` and
/// `b` in `ST0`: `a` when `a < b`, otherwise `b` (also when either is NaN).
pub fn fn_00b719c0(_e: &mut Engine, a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

// Translated from 00b719e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on three floats): the weighted sum
/// `c[1]*w1 + c[0]*w0 + c[2]*w2` with the doubles at `010ae8c0`,
/// `010ae8b8` and `010ae8b0`, rounded to a float and returned in `ST0`.
pub fn fn_00b719e0(e: &mut Engine, this: Ptr) -> f32 {
    let base = this.addr();
    let channel = |e: &Engine, offset: u32| x87_load32(e.mem.u32(base + offset));
    let green = x87_mul(channel(e, 4), e.mem.f64(WEIGHT_1_010AE8C0));
    let red = x87_mul(channel(e, 0), e.mem.f64(WEIGHT_0_010AE8B8));
    let sum = x87_add(green, red);
    let blue = x87_mul(channel(e, 8), e.mem.f64(WEIGHT_2_010AE8B0));
    let sum = x87_add(sum, blue);
    f32::from_bits(x87_store32(sum))
}

// Translated from 00b71a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0xC`: three words, the third
/// never read): makes the object at +0x30 when it is null (`00e7f5d0`),
/// then calls its method `00e7f430(first, second, 1)`.
pub fn fn_00b71a10(e: &mut Engine, this: Ptr, first: u32, second: u32, _unused_2: u32) {
    let base = this.addr();
    if e.mem.u32(base + 0x30) == 0 {
        let object = e.call(MAKE_OBJECT_00E7F5D0, &[]).u32();
        e.mem.set_u32(base + 0x30, object);
    }
    let object = e.mem.u32(base + 0x30);
    e.call(OBJECT_METHOD_00E7F430, &[object, first, second, 1]);
}

// Translated from 00b71a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words): writes the 4x4 float
/// matrix `out` from the 3x3 float matrix `rotation` transposed and
/// multiplied by `scale` (row `i` of `out` is column `i` of `rotation`,
/// with 0 in its fourth float), and the translation row `point` minus the
/// floats at `011f474c` with 1 last.
pub fn fn_00b71a40(e: &mut Engine, out: Ptr, rotation: Ptr, point: Ptr, scale: f32) {
    let out = out.addr();
    let rotation = rotation.addr();
    let point = point.addr();
    let scale = x87_load32(scale.to_bits());
    let m = |e: &Engine, index: u32| x87_load32(e.mem.u32(rotation + index * 4));
    // FMULP ST2: the first entry is the destination, the scale the source.
    let value = x87_mul(m(e, 0), scale);
    e.mem.set_u32(out, x87_store32(value));
    for (slot, index) in [(1, 3), (2, 6)] {
        let value = x87_mul(m(e, index), scale);
        e.mem.set_u32(out + slot * 4, x87_store32(value));
    }
    let zero = x87_store32(0.0);
    e.mem.set_u32(out + 0xc, zero);
    for (slot, index) in [(4, 1), (5, 4), (6, 7)] {
        let value = x87_mul(m(e, index), scale);
        e.mem.set_u32(out + slot * 4, x87_store32(value));
    }
    e.mem.set_u32(out + 0x1c, zero);
    for (slot, index) in [(8, 2), (9, 5)] {
        let value = x87_mul(m(e, index), scale);
        e.mem.set_u32(out + slot * 4, x87_store32(value));
    }
    // FMULP ST2 again, now with the scale as the destination.
    let value = x87_mul(scale, m(e, 8));
    e.mem.set_u32(out + 0x28, x87_store32(value));
    e.mem.set_u32(out + 0x2c, zero);
    for (slot, origin) in [(0xc, ORIGIN_X), (0xd, ORIGIN_Y), (0xe, ORIGIN_Z)] {
        let coordinate = x87_load32(e.mem.u32(point + (slot - 0xc) * 4));
        let value = x87_sub(coordinate, x87_m32(e.mem.u32(origin)));
        e.mem.set_u32(out + slot * 4, x87_store32(value));
    }
    e.mem.set_u32(out + 0x3c, x87_store32(1.0));
}

/// The allocation `00aa13e0` (`cdecl`, one word), `BSShaderProperty`'s constructor
/// (`00ba8e50`, `thiscall`, no words), the light list's `RemoveAll` (`00b71300`, `thiscall` on
/// the list) and `BSShaderLightingProperty`'s vtable (`010ae7b0`).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00ba_8e50;
const LIGHT_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
const LIGHTING_PROPERTY_VTABLE: u32 = 0x010a_e7b0;
/// `memcpy` (`00ec44d0`, `cdecl`, three words).
const MEMCPY: u32 = 0x00ec_44d0;

// Translated from 00b71920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the create function `RegisterStreamables` (`00b4f1af`) records
/// for `BSShaderLightingProperty` (`cdecl`, no arguments). 0x7c bytes from `00aa13e0`, or null
/// when the allocation gave none; otherwise constructed inline: `BSShaderProperty`'s
/// constructor (`00ba8e50`), the vtable `010ae7b0`, the light list at `+0x60` emptied (count,
/// head and tail cleared, then `00b71300`), `fForcedDarkness` (`+0x6c`) 1.0 (`FLD1`/`FSTP`),
/// `uiReferenceID` (`+0x70`) 0, `bLightListChanged` (`+0x74`) false, `iShaderPropertyType`
/// (`+0x1c`) 1 and the fence word (`+0x78`) 0. Answers the object.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b71920 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b71920(e: &mut Engine) -> u32 {
    let object = e.call(ALLOCATE_00AA13E0, &args![0x7cu32]).u32();
    if object == 0 {
        return 0;
    }
    e.call(SHADER_PROPERTY_CONSTRUCT, &args![object]);
    let list = object + 0x60;
    e.mem.set_u32(object, LIGHTING_PROPERTY_VTABLE);
    e.mem.set_u32(list + 8, 0);
    e.mem.set_u32(list, 0);
    e.mem.set_u32(list + 4, 0);
    e.call(LIGHT_LIST_REMOVE_ALL, &args![list]);
    e.mem.set_u32(object + 0x6c, x87_store32(1.0));
    e.mem.set_u32(object + 0x70, 0);
    e.mem.set_u8(object + 0x74, 0);
    e.mem.set_u32(object + 0x1c, 1);
    e.mem.set_u32(object + 0x78, 0);
    object
}

// Translated from 00b71ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: a 0x60-byte block or null; the lighting
/// shader's clip-plane code `00bbef90` calls it): with a block,
/// copies its 0x60 bytes to `+0x1a8` (`memcpy`) and sets the byte `+0x208`; without one clears
/// the byte.
pub fn fn_00b71ad0(e: &mut Engine, this: Ptr, planes: u32) {
    let base = this.addr();
    if planes != 0 {
        e.call(MEMCPY, &args![base + 0x1a8, planes, 0x60u32]);
        e.mem.set_u8(base + 0x208, 1);
        return;
    }
    e.mem.set_u8(base + 0x208, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_1920, fn_00b71920() -> u32),
        entry!(0x00b7_1ad0, fn_00b71ad0(Ptr, u32)),
        entry!(0x00b7_19c0, fn_00b719c0(f64, f64) -> f64),
        entry!(0x00b7_19e0, fn_00b719e0(Ptr) -> f32),
        entry!(0x00b7_1a10, fn_00b71a10(Ptr, u32, u32, u32)),
        entry!(0x00b7_1a40, fn_00b71a40(Ptr, Ptr, Ptr, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smaller_double_comes_back() {
        let mut e = Engine::new();
        let call = |e: &mut Engine, a: f64, b: f64| e.call(0x00b7_19c0, &args![a, b]).st0;
        assert_eq!(call(&mut e, 1.5, 2.5), 1.5);
        assert_eq!(call(&mut e, 2.5, 1.5), 1.5);
        assert_eq!(call(&mut e, 2.0, 2.0), 2.0);
        assert!(call(&mut e, 1.0, f64::NAN).is_nan());
        assert_eq!(call(&mut e, f64::NAN, 3.0), 3.0);
    }

    #[test]
    fn weighted_sum_uses_the_three_doubles() {
        let mut e = Engine::new();
        e.map(0x010a_e000, 0x1000);
        e.mem.set_f64(WEIGHT_2_010AE8B0, 0.25);
        e.mem.set_f64(WEIGHT_0_010AE8B8, 0.5);
        e.mem.set_f64(WEIGHT_1_010AE8C0, 2.0);
        let colour = e.mem.alloc(12);
        e.mem.set_f32(colour, 1.0);
        e.mem.set_f32(colour + 4, 3.0);
        e.mem.set_f32(colour + 8, 8.0);
        let r = e.call(0x00b7_19e0, &[colour]);
        assert_eq!(r.st0, 8.5);
    }

    #[test]
    fn object_is_made_once_and_called() {
        let mut e = Engine::new();
        e.register(MAKE_OBJECT_00E7F5D0, |_, _| 0x4444u32.into_ret());
        e.register(OBJECT_METHOD_00E7F430, |_, _| Ret::default());
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x00b7_1a10, &[this, 7, 8, 9]);
        e.call(0x00b7_1a10, &[this, 1, 2, 3]);
        let log = e.call_log.as_ref().unwrap();
        let makes = log
            .iter()
            .filter(|(a, _)| *a == MAKE_OBJECT_00E7F5D0)
            .count();
        assert_eq!(makes, 1);
        let methods: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == OBJECT_METHOD_00E7F430)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(methods, vec![vec![0x4444, 7, 8, 1], vec![0x4444, 1, 2, 1]]);
        assert_eq!(e.mem.u32(this + 0x30), 0x4444);
    }

    #[test]
    fn matrix_is_transposed_scaled_and_translated() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.mem.set_f32(ORIGIN_X, 1.0);
        e.mem.set_f32(ORIGIN_Y, 2.0);
        e.mem.set_f32(ORIGIN_Z, 3.0);
        let out = e.mem.alloc(0x40);
        let rotation = e.mem.alloc(0x24);
        for i in 0..9 {
            e.mem.set_f32(rotation + i * 4, (i + 1) as f32);
        }
        let point = e.mem.alloc(12);
        e.mem.set_f32(point, 10.0);
        e.mem.set_f32(point + 4, 20.0);
        e.mem.set_f32(point + 8, 30.0);
        e.call(0x00b7_1a40, &args![out, rotation, point, 2.0f32]);
        let got: Vec<f32> = (0..16).map(|i| e.mem.f32(out + i * 4)).collect();
        assert_eq!(
            got,
            vec![
                2.0, 8.0, 14.0, 0.0, 4.0, 10.0, 16.0, 0.0, 6.0, 12.0, 18.0, 0.0, 9.0, 18.0, 27.0,
                1.0
            ]
        );
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00b71920_makes_a_lighting_property() {
        let mut e = Engine::new();
        e.register(ALLOCATE_00AA13E0, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in (0..a[0]).step_by(4) {
                e.mem.set_u32(block + offset, 0xdead_beef);
            }
            block.into_ret()
        });
        e.register(SHADER_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(LIGHT_LIST_REMOVE_ALL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let object = e.call(0x00b7_1920, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls(&e, ALLOCATE_00AA13E0), vec![vec![0x7c]]);
        assert_eq!(calls(&e, SHADER_PROPERTY_CONSTRUCT), vec![vec![object]]);
        assert_eq!(calls(&e, LIGHT_LIST_REMOVE_ALL), vec![vec![object + 0x60]]);
        assert_eq!(e.mem.u32(object), LIGHTING_PROPERTY_VTABLE);
        for offset in [0x60, 0x64, 0x68, 0x70, 0x78] {
            assert_eq!(e.mem.u32(object + offset), 0);
        }
        assert_eq!(e.mem.f32(object + 0x6c), 1.0);
        assert_eq!(e.mem.u8(object + 0x74), 0);
        assert_eq!(e.mem.u32(object + 0x1c), 1);
        // No block: null, nothing constructed.
        e.register(ALLOCATE_00AA13E0, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00b7_1920, &args![]).u32(), 0);
        assert_eq!(calls(&e, SHADER_PROPERTY_CONSTRUCT).len(), 1);
    }

    #[test]
    fn fn_00b71ad0_copies_or_clears() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x210);
        e.call(0x00b7_1ad0, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u8(this + 0x208), 1);
        assert_eq!(calls(&e, MEMCPY), vec![vec![this + 0x1a8, 0x5000, 0x60]]);
        e.call(0x00b7_1ad0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x208), 0);
        assert_eq!(calls(&e, MEMCPY).len(), 1);
    }
}
