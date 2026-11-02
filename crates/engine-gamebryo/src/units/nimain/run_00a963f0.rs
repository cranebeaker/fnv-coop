//! `NiMain/run_00a963f0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `NiParticlesData` constructor (the vtable it stores, `010a05e4`, has
//! the RTTI name `.?AVNiParticlesData@@`) and four of its array setters; the
//! engine map gives them no names. Field names are the Xbox PDB's, whose
//! offsets the PC code uses.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiParticlesData` (Xbox PDB, 0x70 bytes): the fields used so far.
    pub struct NiParticlesData: 0x70 {
        /// `m_bSaveVertexData` (Xbox PDB, `NiGeometryData`).
        0x3C m_bSaveVertexData: u8,
        /// `m_bIsOldRotatingParticlesObject` (Xbox PDB).
        0x40 m_bIsOldRotatingParticlesObject: u8,
        /// `m_pfRadii` (Xbox PDB).
        0x4C m_pfRadii: u32,
        /// `m_usActive` (Xbox PDB).
        0x50 m_usActive: u16,
        /// `m_pfSizes` (Xbox PDB).
        0x54 m_pfSizes: u32,
        /// `m_pkRotations` (Xbox PDB).
        0x58 m_pkRotations: u32,
        /// `m_pfRotationAngles` (Xbox PDB).
        0x5C m_pfRotationAngles: u32,
        /// `m_pkRotationAxes` (Xbox PDB).
        0x60 m_pkRotationAxes: u32,
        /// `m_pucTextureIndices` (Xbox PDB).
        0x64 m_pucTextureIndices: u32,
        /// `m_pkSubTextureOffsets` (Xbox PDB).
        0x68 m_pkSubTextureOffsets: u32,
        /// `m_ucSubTextureOffsetCount` (Xbox PDB).
        0x6C m_ucSubTextureOffsetCount: u8,
    }
}

/// The vtable of `NiParticlesData` (RTTI `.?AVNiParticlesData@@`).
const VTABLE: u32 = 0x010a_05e4;
/// `NiGeometryData::NiGeometryData` (`00a67c00`, engine map; `thiscall`).
const GEOMETRY_DATA_CONSTRUCTOR: u32 = 0x00a6_7c00;
/// `NiGeometryData::SetConsistency` (`00a67050`, engine map; `thiscall`,
/// one word).
const GEOMETRY_DATA_SET_CONSISTENCY: u32 = 0x00a6_7050;
/// A block free (`00aa10f0`, `cdecl`, one word; no Xbox PDB name).
const FREE_BLOCK: u32 = 0x00aa_10f0;

// Translated from 00a963f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiParticlesData` constructor (`thiscall`): the `NiGeometryData`
/// constructor, the vtable, every particle array and count cleared, not an
/// old rotating particles object, `SetConsistency(0)` and no saved vertex
/// data. Returns `this`.
pub fn fn_00a963f0(e: &mut Engine, this: Ptr<NiParticlesData>) -> Ptr<NiParticlesData> {
    e.call(GEOMETRY_DATA_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, NiParticlesData::m_pfRadii, 0);
    e.set(this, NiParticlesData::m_usActive, 0);
    e.set(this, NiParticlesData::m_pfSizes, 0);
    e.set(this, NiParticlesData::m_pkRotations, 0);
    e.set(this, NiParticlesData::m_pfRotationAngles, 0);
    e.set(this, NiParticlesData::m_pkRotationAxes, 0);
    e.set(this, NiParticlesData::m_pucTextureIndices, 0);
    e.set(this, NiParticlesData::m_pkSubTextureOffsets, 0);
    e.set(this, NiParticlesData::m_ucSubTextureOffsetCount, 0);
    e.set(this, NiParticlesData::m_bIsOldRotatingParticlesObject, 0);
    e.call(GEOMETRY_DATA_SET_CONSISTENCY, &args![this, 0u32]);
    e.set(this, NiParticlesData::m_bSaveVertexData, 0);
    this
}

/// Frees the array at `offset` (`00aa10f0`) and stores `array` there.
fn replace_array(e: &mut Engine, this: Ptr<NiParticlesData>, offset: u32, array: u32) {
    let old = e.mem.u32(this.addr() + offset);
    e.call(FREE_BLOCK, &args![old]);
    e.mem.set_u32(this.addr() + offset, array);
}

// Translated from 00a96440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces `m_pfRadii` (`thiscall`, one word): the old array is freed.
pub fn fn_00a96440(e: &mut Engine, this: Ptr<NiParticlesData>, array: u32) {
    replace_array(e, this, 0x4c, array);
}

// Translated from 00a96460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces `m_pfSizes` (`thiscall`, one word): the old array is freed.
pub fn fn_00a96460(e: &mut Engine, this: Ptr<NiParticlesData>, array: u32) {
    replace_array(e, this, 0x54, array);
}

// Translated from 00a96480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces `m_pfRotationAngles` (`thiscall`, one word): the old array is
/// freed.
pub fn fn_00a96480(e: &mut Engine, this: Ptr<NiParticlesData>, array: u32) {
    replace_array(e, this, 0x5c, array);
}

// Translated from 00a964a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces `m_pucTextureIndices` (`thiscall`, one word): the old array is
/// freed.
pub fn fn_00a964a0(e: &mut Engine, this: Ptr<NiParticlesData>, array: u32) {
    replace_array(e, this, 0x64, array);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a9_63f0,
            fn_00a963f0(Ptr<NiParticlesData>) -> Ptr<NiParticlesData>
        ),
        entry!(0x00a9_6440, fn_00a96440(Ptr<NiParticlesData>, u32)),
        entry!(0x00a9_6460, fn_00a96460(Ptr<NiParticlesData>, u32)),
        entry!(0x00a9_6480, fn_00a96480(Ptr<NiParticlesData>, u32)),
        entry!(0x00a9_64a0, fn_00a964a0(Ptr<NiParticlesData>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_clears_the_particle_arrays() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.mem.write(this, &[0xcd; 0x70]);
        e.register(GEOMETRY_DATA_CONSTRUCTOR, |_, _| Ret::default());
        e.register(GEOMETRY_DATA_SET_CONSISTENCY, |e, a| {
            // Runs after the arrays are cleared, before the saved-data flag.
            assert_eq!(e.mem.u8(a[0] + 0x40), 0);
            assert_eq!(e.mem.u8(a[0] + 0x3c), 0xcd);
            assert_eq!(a[1], 0);
            Ret::default()
        });
        assert_eq!(e.call(0x00a9_63f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert_eq!(e.mem.u16(this + 0x50), 0);
        assert_eq!(e.mem.u16(this + 0x52), 0xcdcd, "padding after m_usActive");
        assert_eq!(e.mem.bytes(this + 0x54, 0x19), vec![0; 0x19]);
        assert_eq!(e.mem.u8(this + 0x3c), 0);
        assert_eq!(e.mem.u8(this + 0x40), 0);
        // The fields it does not touch.
        assert_eq!(e.mem.u32(this + 0x44), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(this + 0x48), 0xcdcd_cdcd);
    }

    #[test]
    fn setters_free_the_old_array() {
        for (address, offset) in [
            (0x00a9_6440u32, 0x4cu32),
            (0x00a9_6460, 0x54),
            (0x00a9_6480, 0x5c),
            (0x00a9_64a0, 0x64),
        ] {
            let mut e = Engine::new();
            let this = e.mem.alloc(0x70);
            e.mem.set_u32(this + offset, 0x1111);
            e.register(FREE_BLOCK, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(address, &args![this, 0x2222u32]);
            assert_eq!(e.mem.u32(this + offset), 0x2222);
            assert_eq!(e.call_log.take().unwrap()[1], (FREE_BLOCK, vec![0x1111]));
        }
    }
}
