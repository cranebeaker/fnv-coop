//! `fallout shared/sky/run_006368f0` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Functions the linker placed between the moon and the precipitation code: the moon-phase
//! test `Sky::Update` calls, the climate's phase length, the moons' texture unload and the
//! `Precipitation` constructor (whose vtable it stores; `Sky::Initialize` allocates it with
//! 0x18 bytes, the Xbox PDB's size).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_sky::skyobject::{
    pointer_assign, pointer_get, NI_POINTER_FROM_RAW,
};

/// The sky's current climate (`0084e3a0`, `thiscall`: `+0xc`).
const SKY_CURRENT_CLIMATE: u32 = 0x0084_e3a0;
/// The calendar (`011de7b8`) and its day count (`00867de0`, `thiscall`).
const CALENDAR: u32 = 0x011d_e7b8;
const CALENDAR_GET_DAYS: u32 = 0x0086_7de0;
/// The moon phase the last test saw (`011cca80`).
const LAST_MOON_PHASE: u32 = 0x011c_ca80;
/// `NiAVObject::GetProperty` (`00a59d30`, `thiscall`, one word), the cast `00653270`
/// (`cdecl`: the RTTI, the object) and `SkyShaderProperty::ms_RTTI` (`012005b8`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const DYNAMIC_CAST: u32 = 0x0065_3270;
const SKY_SHADER_PROPERTY_RTTI: u32 = 0x0120_05b8;
/// The `SkyShaderProperty` base map and file name setters of `clouds.cpp` (`006348e0`,
/// `00634900`; `thiscall`, one word) and the empty `NiFixedString` (`011f49e0`).
const SET_BASE_MAP: u32 = 0x0063_48e0;
const SET_FILE_NAME: u32 = 0x0063_4900;
const EMPTY_FIXED_STRING_OBJECT: u32 = 0x011f_49e0;
/// `Precipitation`'s vtable (`0104ed98`).
const PRECIPITATION_VTABLE: u32 = 0x0104_ed98;

layout! {
    /// `Precipitation` (Xbox PDB), 0x18 bytes.
    pub struct Precipitation: 0x18 {
        /// `spCurrentPrecipRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x04 spCurrentPrecipRoot: u32,
        /// `spLastPrecipRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x08 spLastPrecipRoot: u32,
        /// `pParent` (Xbox PDB).
        0x0C pParent: u32,
        /// `fBirthRate` (Xbox PDB).
        0x10 fBirthRate: f32,
        /// `pQueuedModel` (Xbox PDB).
        0x14 pQueuedModel: u32,
    }
}

// Translated from 006368f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word: the sky; `MOON_PHASE_APPLIES` in
/// `sky.rs`). With a current climate whose phase length (`00636980`) is not 0: the phase is
/// `(days % (8 * length)) / length` from the calendar's day count; answers 1 and remembers
/// it when it differs from the phase last seen (`011cca80`), else 0.
pub fn fn_006368f0(e: &mut Engine, sky: u32) -> bool {
    if e.call(SKY_CURRENT_CLIMATE, &args![sky]).u32() == 0 {
        return false;
    }
    let climate = e.call(SKY_CURRENT_CLIMATE, &args![sky]).u32();
    if fn_00636980(e, climate) == 0 {
        return false;
    }
    let days = e.call(CALENDAR_GET_DAYS, &args![CALENDAR]).u32();
    let climate = e.call(SKY_CURRENT_CLIMATE, &args![sky]).u32();
    let cycle = fn_00636980(e, climate) << 3;
    let rest = days % cycle;
    let climate = e.call(SKY_CURRENT_CLIMATE, &args![sky]).u32();
    let phase = rest / fn_00636980(e, climate);
    if phase == e.global::<u32>(LAST_MOON_PHASE) {
        return false;
    }
    e.set_global(LAST_MOON_PHASE, phase);
    true
}

// Translated from 00636980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `TESClimate` by its use, no stack words): the
/// low six bits of the byte at `+0x55` (the moon phase length).
pub fn fn_00636980(e: &mut Engine, climate: u32) -> u32 {
    e.mem.u8(climate + 0x55) as u32 & 0xffff_ff3f
}

// Translated from 006369a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `Moon`, no stack words; `MOON_UNLOAD_TEXTURES`
/// in `sky.rs`): for the moon mesh (`+0x10`) and then the shadow mesh (`+0x14`), a
/// `SkyShaderProperty` as property 3 loses its base map and takes the empty file name.
pub fn fn_006369a0(e: &mut Engine, moon: u32) {
    for slot in [0x10u32, 0x14] {
        let mesh = pointer_get(e, moon + slot);
        let property = e.call(GET_PROPERTY, &args![mesh, 3u32]).u32();
        let sky_property = e
            .call(DYNAMIC_CAST, &args![SKY_SHADER_PROPERTY_RTTI, property])
            .u32();
        if sky_property != 0 {
            e.call(SET_BASE_MAP, &args![sky_property, 0u32]);
            e.call(
                SET_FILE_NAME,
                &args![sky_property, EMPTY_FIXED_STRING_OBJECT],
            );
        }
    }
}

// Translated from 00636a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Precipitation::Precipitation` (by the vtable it stores; `PRECIPITATION_CONSTRUCT` in
/// `sky.rs`; `thiscall`, no stack words): the vtable, both roots constructed from 0 and
/// assigned 0, birth rate 0, no queued model, no parent. Answers `this`.
// Translation incomplete: 00636a40 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the roots) is not translated.
pub fn fn_00636a40(e: &mut Engine, this: Ptr<Precipitation>) -> Ptr<Precipitation> {
    let base = this.addr();
    e.mem.set_u32(base, PRECIPITATION_VTABLE);
    e.call(NI_POINTER_FROM_RAW, &args![base + 4, 0u32]);
    e.call(NI_POINTER_FROM_RAW, &args![base + 8, 0u32]);
    pointer_assign(e, base + 4, 0);
    pointer_assign(e, base + 8, 0);
    e.set(this, Precipitation::fBirthRate, 0.0);
    e.set(this, Precipitation::pQueuedModel, 0);
    e.set(this, Precipitation::pParent, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0063_68f0, fn_006368f0(u32) -> bool),
        entry!(0x0063_6980, fn_00636980(u32) -> u32),
        entry!(0x0063_69a0, fn_006369a0(u32)),
        entry!(
            0x0063_6a40,
            fn_00636a40(Ptr<Precipitation>) -> Ptr<Precipitation>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::fallout_shared_sky::skyobject::test_support::*;
    use crate::units::fallout_shared_sky::skyobject::NI_POINTER_ASSIGN;

    fn phase_rig(length: u8, days: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let sky = e.mem.alloc(0x138);
        let climate = e.mem.alloc(0x60);
        e.mem.set_u32(sky + 0xc, climate);
        e.mem.set_u8(climate + 0x55, length);
        e.register(SKY_CURRENT_CLIMATE, |e, a| answer(e.mem.u32(a[0] + 0xc)));
        e.register_double(CALENDAR_GET_DAYS, move |_, _| answer(days));
        e.map(LAST_MOON_PHASE, 4);
        (e, sky)
    }

    #[test]
    fn moon_phase_changes_are_reported_once() {
        // Length 3 (the two high bits are ignored), day 20: 20 % 24 = 20, phase 6.
        let (mut e, sky) = phase_rig(0xc3, 20);
        assert!(e.call(0x0063_68f0, &args![sky]).bool());
        assert_eq!(e.global::<u32>(LAST_MOON_PHASE), 6);
        assert!(!e.call(0x0063_68f0, &args![sky]).bool());
        // A zero length or no climate never reports.
        let (mut e, sky) = phase_rig(0xc0, 20);
        assert!(!e.call(0x0063_68f0, &args![sky]).bool());
        e.mem.set_u32(sky + 0xc, 0);
        assert!(!e.call(0x0063_68f0, &args![sky]).bool());
        assert_eq!(e.global::<u32>(LAST_MOON_PHASE), 0);
    }

    #[test]
    fn moon_texture_unload_clears_both_meshes() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(&mut e, &[SET_BASE_MAP, SET_FILE_NAME]);
        e.register(GET_PROPERTY, |e, a| answer(e.mem.u32(a[0] + 0xa8)));
        e.register(DYNAMIC_CAST, |e, a| {
            if e.mem.u32(a[1] + 0x1c) == 0xd {
                answer(a[1])
            } else {
                answer(0)
            }
        });
        let sky_property = e.mem.alloc(0x90);
        e.mem.set_u32(sky_property + 0x1c, 0xd);
        let other = e.mem.alloc(0x90);
        let moon = e.mem.alloc(0x7c);
        for (slot, property) in [(0x10u32, sky_property), (0x14, other)] {
            let mesh = e.mem.alloc(0xc0);
            e.mem.set_u32(mesh + 0xa8, property);
            e.mem.set_u32(moon + slot, mesh);
        }
        e.call_log = Some(vec![]);
        e.call(0x0063_69a0, &args![moon]);
        assert_eq!(calls(&e, SET_BASE_MAP), vec![vec![sky_property, 0]]);
        assert_eq!(
            calls(&e, SET_FILE_NAME),
            vec![vec![sky_property, EMPTY_FIXED_STRING_OBJECT]]
        );
        assert_eq!(calls(&e, GET_PROPERTY).len(), 2);
    }

    #[test]
    fn precipitation_constructor_clears_everything() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        let this = e.mem.alloc(0x18);
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_6a40, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), PRECIPITATION_VTABLE);
        for offset in [4u32, 8, 0xc, 0x10, 0x14] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN),
            vec![vec![this + 4, 0], vec![this + 8, 0]]
        );
    }
}
