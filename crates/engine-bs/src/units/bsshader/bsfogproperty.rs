//! `BSShader/bsfogproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BSFogProperty` (Xbox PDB), an `NiFogProperty`; 0x64 bytes on PC as on the Xbox
    /// (`ShadowSceneNode`'s constructor allocates 0x64 for it). The fields the constructor sets.
    pub struct BSFogProperty: 0x64 {
        /// `m_uFlags` (Xbox PDB, `NiFogProperty`).
        0x18 m_uFlags: u16,
        /// `m_fDepth` (Xbox PDB, `NiFogProperty`).
        0x1C m_fDepth: f32,
        /// `fStartDistance` (Xbox PDB).
        0x2C fStartDistance: f32,
        /// `fEndDistance` (Xbox PDB).
        0x30 fEndDistance: f32,
        /// `fPower` (Xbox PDB).
        0x60 fPower: f32,
    }
}

/// The base constructor `00a5c200` (`thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00a5_c200;
/// `NiFogProperty`'s vtable (`010a0e0c`, RTTI `.?AVNiFogProperty@@`) and this class's
/// (`010b9e38`, no RTTI locator before it).
const NI_FOG_PROPERTY_VTABLE: u32 = 0x010a_0e0c;
const BS_FOG_PROPERTY_VTABLE: u32 = 0x010b_9e38;
/// The three words the colour `m_kColor` (+0x20) is copied from (`011f4998`).
const COLOR_011F4998: u32 = 0x011f_4998;
/// `NiPlane::NiPlane` (engine map, `00a69940`), on `kPlane` (+0x3c).
const NI_PLANE_CONSTRUCT: u32 = 0x00a6_9940;
/// The `double`s of the inlined distance computation (`00bb8140`'s): 0.83 (`010b9e30`) and
/// 163840.0 (`0103a480`) in the exe, and the `float` end distance (`010317c4`).
const DEPTH_FACTOR: u32 = 0x010b_9e30;
const DISTANCE_SCALE: u32 = 0x0103_a480;
const END_DISTANCE: u32 = 0x0103_17c4;

// Translated from 00bb8180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFogProperty::BSFogProperty` (Xbox PDB), `thiscall`, no stack words; answers `this`.
/// Runs the base constructor (`00a5c200`), then as `NiFogProperty`: colour zero, flags 0,
/// depth 1.0, colour from the three words at `011f4998`; then as `BSFogProperty`: constructs
/// `kPlane` (+0x3c), clears `kWaterColor` (+0x54), computes the start distance from the depth
/// read back from `+0x1c` (the inlined `00bb8140`: `(1 - depth * 0.83) * 163840` from the
/// exe's `double`s) and copies the end distance (`010317c4`), clears the plane's four floats
/// and sets `fPower` to 1.0.
// Translation incomplete: 00bb8180 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
pub fn bs_fog_property_bs_fog_property(
    e: &mut Engine,
    this: Ptr<BSFogProperty>,
) -> Ptr<BSFogProperty> {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, NI_FOG_PROPERTY_VTABLE);
    for offset in [0x20u32, 0x24, 0x28] {
        e.mem.set_u32(base + offset, 0);
    }
    e.set(this, BSFogProperty::m_uFlags, 0);
    e.set(this, BSFogProperty::m_fDepth, 1.0);
    for i in 0..3 {
        let word = e.global::<u32>(COLOR_011F4998 + 4 * i);
        e.mem.set_u32(base + 0x20 + 4 * i, word);
    }
    e.mem.set_u32(base, BS_FOG_PROPERTY_VTABLE);
    e.call(NI_PLANE_CONSTRUCT, &args![base + 0x3c]);
    for offset in [0x54u32, 0x58, 0x5c] {
        e.mem.set_u32(base + offset, 0);
    }
    let factor: f64 = e.global(DEPTH_FACTOR);
    let scale: f64 = e.global(DISTANCE_SCALE);
    let depth = e.mem.u32(base + 0x1c);
    let product = x87_mul(x87_load32(depth), factor);
    let start = x87_mul(x87_sub(1.0, product), scale);
    e.mem.set_u32(base + 0x2c, x87_store32(start));
    let end = x87_quiet_bits(e.global::<u32>(END_DISTANCE));
    e.mem.set_u32(base + 0x30, end);
    for offset in [0x3cu32, 0x40, 0x44, 0x48] {
        e.mem.set_u32(base + offset, 0);
    }
    e.set(this, BSFogProperty::fPower, 1.0);
    this
}

/// The destructor body `00a5c240` (unit `niobjectnet.cpp`, unnamed in the engine map;
/// `thiscall`, no stack words).
const DESTRUCT_00A5C240: u32 = 0x00a5_c240;
/// The sized Ni free `00aa1460` (`cdecl`: block, size).
const NI_DELETE: u32 = 0x00aa_1460;

// Translated from 00a99f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFogProperty::_scalar_deleting_destructor_` (engine map; the Xbox PDB's `NiFogProperty`
/// is 0x2c bytes, the size freed here): `thiscall`, one stack word, `RET 4`. Runs the
/// destructor body `00a5c240`, then frees the 0x2c bytes of `this` when bit 0 of `flags` is
/// set. Answers `this`.
pub fn ni_fog_property_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_00A5C240, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x2cu32]);
    }
    this
}

/// `NiFogProperty::ms_RTTI` (`011f5f34`).
const NI_FOG_PROPERTY_RTTI: u32 = 0x011f_5f34;

// Translated from 00a99f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFogProperty::GetRTTI` (Xbox PDB): answers `&ms_RTTI` (`011f5f34`).
pub fn ni_fog_property_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    NI_FOG_PROPERTY_RTTI
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00bb_8180,
            bs_fog_property_bs_fog_property(Ptr<BSFogProperty>) -> Ptr<BSFogProperty>
        ),
        entry!(
            0x00a9_9f40,
            ni_fog_property_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_9f20, ni_fog_property_get_rtti(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_sets_the_vtables_colour_plane_and_distances() {
        let mut e = Engine::new();
        for page in [0x0103_1000, 0x0103_a000, 0x010b_9000, 0x011f_4000] {
            e.map(page, 0x1000);
        }
        e.set_global(DEPTH_FACTOR, 0.83f64);
        e.set_global(DISTANCE_SCALE, 163_840.0f64);
        e.set_global(END_DISTANCE, 163_840.0f32);
        for i in 0..3u32 {
            e.set_global(COLOR_011F4998 + 4 * i, 0x300 + i);
        }
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        // The plane constructor leaves garbage; the constructor clears it afterwards.
        e.register(NI_PLANE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xdddd_dddd);
            Ret::default()
        });
        let this: Ptr<BSFogProperty> = e.new_object();
        let base = this.addr();
        for offset in (4..0x64).step_by(4) {
            e.mem.set_u32(base + offset, 0xeeee_eeee);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bb_8180, &args![this]).u32(), base);
        let log = e.call_log.take().unwrap();
        let plane: Vec<_> = log.iter().filter(|c| c.0 == NI_PLANE_CONSTRUCT).collect();
        assert_eq!(plane[0].1, vec![base + 0x3c]);
        assert_eq!(e.mem.u32(base), BS_FOG_PROPERTY_VTABLE);
        assert_eq!(e.get(this, BSFogProperty::m_uFlags), 0);
        assert_eq!(e.get(this, BSFogProperty::m_fDepth), 1.0);
        for i in 0..3u32 {
            assert_eq!(e.mem.u32(base + 0x20 + 4 * i), 0x300 + i);
            assert_eq!(e.mem.u32(base + 0x54 + 4 * i), 0);
        }
        let start = ((1.0 - 0.83f64) * 163_840.0) as f32;
        assert_eq!(e.get(this, BSFogProperty::fStartDistance), start);
        assert_eq!(e.get(this, BSFogProperty::fEndDistance), 163_840.0);
        for offset in [0x3cu32, 0x40, 0x44, 0x48] {
            assert_eq!(e.mem.u32(base + offset), 0);
        }
        assert_eq!(e.get(this, BSFogProperty::fPower), 1.0);
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::*;

    #[test]
    fn ni_fog_property_scalar_deleting_destructor_frees_on_bit_zero() {
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            let mut e = Engine::new();
            e.register(DESTRUCT_00A5C240, |_, _| Ret::default());
            e.register(NI_DELETE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x00a9_9f40, &args![0x0e00_2000u32, flags]).u32(),
                0x0e00_2000
            );
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(DESTRUCT_00A5C240, vec![0x0e00_2000])];
            if frees {
                expected.push((NI_DELETE, vec![0x0e00_2000, 0x2c]));
            }
            assert_eq!(log[1..].to_vec(), expected);
        }
    }
}

#[cfg(test)]
mod n011_tests {
    use super::*;

    #[test]
    fn ni_fog_property_get_rtti_answers_the_class_record() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a9_9f20, &args![0x0e00_1000u32]).u32(),
            0x011f_5f34
        );
    }
}
