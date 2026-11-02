//! `(unplaced)/run_00be0870` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Four shader helpers the engine map does not name. `00be08c0` is
//! referenced from tables at `010af348` (slot +0x50 of a table starting at
//! `010af2f8`, which has no RTTI locator), `010af7b0`, `010afa30` and
//! `010afe68`; `00be08b0` sits 0x80 bytes after the last three. `00be08f0`
//! installs the vtable whose RTTI name is `NiD3DPixelShader`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00e80570`: the base initialization (a jump to `00e81a90`); its byte
/// result says whether to go on.
const BASE_INITIALIZE: u32 = 0x00e8_0570;
/// The three virtual slots `fn_00be0870` calls after it, in this order.
const SLOT_FIRST: u32 = 0x12c;
const SLOT_SECOND: u32 = 0x134;
const SLOT_THIRD: u32 = 0x130;
/// `00e814b0`: the function `00be08b0` jumps to.
const THUNK_TARGET: u32 = 0x00e8_14b0;
/// `00a59d30`: `NiAVObject::GetProperty` (engine map), one word.
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// The property index asked for.
const PROPERTY_INDEX: u32 = 3;
/// The property's virtual slot +0x9C, given the object.
const SLOT_PROPERTY_APPLY: u32 = 0x9c;
/// `00e897d0`: `NiD3DShaderProgram::NiD3DShaderProgram` (engine map): the
/// program and its kind.
const SHADER_PROGRAM_CONSTRUCT: u32 = 0x00e8_97d0;
/// The kind passed for a pixel shader.
const PIXEL_SHADER_KIND: u32 = 2;
/// `NiD3DPixelShader`'s vtable (RTTI).
const PIXEL_SHADER_VTABLE: u32 = 0x010e_f7d4;
/// The word cleared at +0x2C.
const PIXEL_SHADER_EXTRA: u32 = 0x2c;

// Translated from 00be0870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the base initialization (`00e80570`); when it succeeds, calls
/// virtual slots +0x12C, +0x134 and +0x130 and returns true.
pub fn fn_00be0870(e: &mut Engine, this: Ptr) -> bool {
    if !e.call(BASE_INITIALIZE, &args![this]).bool() {
        return false;
    }
    for slot in [SLOT_FIRST, SLOT_SECOND, SLOT_THIRD] {
        e.vcall(this.addr(), slot, &args![]);
    }
    true
}

// Translated from 00be08b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A thunk: jumps to `00e814b0` with the same `this`.
pub fn fn_00be08b0(e: &mut Engine, this: Ptr) {
    e.call(THUNK_TARGET, &args![this]);
}

// Translated from 00be08c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gets property 3 of `object` and calls its slot +0x9C with `object`.
/// The method never reads its `this` (it sits in a vtable next to
/// `this`-using methods, so callers pass one).
pub fn fn_00be08c0(e: &mut Engine, _unused_this: Ptr, object: Ptr) {
    let property = e.call(GET_PROPERTY, &args![object, PROPERTY_INDEX]).u32();
    e.vcall(property, SLOT_PROPERTY_APPLY, &args![object]);
}

// Translated from 00be08f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a `NiD3DPixelShader`: the shader-program base with kind 2,
/// then its vtable and a cleared word at +0x2C. Returns `this`.
pub fn fn_00be08f0(e: &mut Engine, this: Ptr, program: u32) -> Ptr {
    e.call(
        SHADER_PROGRAM_CONSTRUCT,
        &args![this, program, PIXEL_SHADER_KIND],
    );
    e.mem.set_u32(this.addr(), PIXEL_SHADER_VTABLE);
    e.mem.set_u32(this.addr() + PIXEL_SHADER_EXTRA, 0);
    this
}

/// `00e804d0` (`cdecl`, one word; unit `geometrydecalshader.cpp` in the engine map, which
/// names it `GeometryDecalShader::LoadStagesAndPasses`, unconfirmed for this body).
const PASS_STEP_00E804D0: u32 = 0x00e8_04d0;

// Translated from 00be08e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; it sits in shader vtables, `thiscall`, no stack words):
/// calls `00e804d0` with the address of `this + 0x5c` and answers true (only `AL` is set).
pub fn fn_00be08e0(e: &mut Engine, this: Ptr) -> bool {
    e.call(PASS_STEP_00E804D0, &args![this.addr().wrapping_add(0x5c)]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00be_08e0, fn_00be08e0(Ptr) -> bool),
        entry!(0x00be0870, fn_00be0870(Ptr) -> bool),
        entry!(0x00be08b0, fn_00be08b0(Ptr)),
        entry!(0x00be08c0, fn_00be08c0(Ptr, Ptr)),
        entry!(0x00be08f0, fn_00be08f0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0002_0000;

    fn shader(e: &mut Engine) -> Ptr {
        e.put_vtable(VTABLE, &[0; 0x4e]);
        for (i, slot) in [SLOT_FIRST, SLOT_SECOND, SLOT_THIRD].iter().enumerate() {
            e.mem.set_u32(VTABLE + slot, 0x0001_0000 + 0x10 * i as u32);
            e.register(0x0001_0000 + 0x10 * i as u32, |_, _| Ret::default());
        }
        let p = e.mem.alloc(0x30);
        e.mem.set_u32(p, VTABLE);
        Ptr::new(p)
    }

    #[test]
    fn initialize_calls_three_slots_on_success() {
        let mut e = Engine::new();
        e.register(BASE_INITIALIZE, |_, _| 1u32.into_ret());
        let this = shader(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00be_0870, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![BASE_INITIALIZE, 0x0001_0000, 0x0001_0010, 0x0001_0020]
        );
    }

    #[test]
    fn initialize_stops_when_the_base_fails() {
        let mut e = Engine::new();
        e.register(BASE_INITIALIZE, |_, _| 0x100u32.into_ret());
        let this = shader(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00be_0870, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn thunk_forwards_this() {
        let mut e = Engine::new();
        e.register(THUNK_TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00be_08b0, &args![0x1234u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (THUNK_TARGET, vec![0x1234]));
    }

    #[test]
    fn property_3_gets_the_object() {
        let mut e = Engine::new();
        let property: Ptr = Ptr::new(e.mem.alloc(4));
        e.put_vtable(0x0003_0000, &[0; 0x28]);
        e.mem
            .set_u32(0x0003_0000 + SLOT_PROPERTY_APPLY, 0x0001_0100);
        e.mem.set_u32(property.addr(), 0x0003_0000);
        e.register(0x0001_0100, |_, _| Ret::default());
        e.register_double(GET_PROPERTY, move |_, _| property.addr().into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00be_08c0, &args![0x9999u32, 0x4444u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (GET_PROPERTY, vec![0x4444, 3]));
        assert_eq!(log[2], (0x0001_0100, vec![property.addr(), 0x4444]));
    }

    #[test]
    fn pixel_shader_constructor() {
        let mut e = Engine::new();
        e.register(SHADER_PROGRAM_CONSTRUCT, |_, a| a[0].into_ret());
        let this: Ptr = Ptr::new(e.mem.alloc(0x30));
        e.mem.set_u32(this.addr() + 0x2c, 7);
        e.call_log = Some(vec![]);
        let back = e.call(0x00be_08f0, &args![this, 0x55u32]).ptr::<()>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, this);
        assert_eq!(
            log[1],
            (SHADER_PROGRAM_CONSTRUCT, vec![this.addr(), 0x55, 2])
        );
        assert_eq!(e.mem.u32(this.addr()), PIXEL_SHADER_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x2c), 0);
    }
}

#[cfg(test)]
mod batch_b011_tests {
    use super::*;

    #[test]
    fn fn_00be08e0_passes_this_plus_0x5c() {
        let mut e = Engine::new();
        e.register(PASS_STEP_00E804D0, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00be_08e0, &args![0x1000u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (PASS_STEP_00E804D0, vec![0x105c]));
        assert_eq!(log.len(), 2);
    }
}
