//! `SpeedTree/camera.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `CIdvCamera` (Xbox PDB), 0x10 bytes: a vtable and the camera
    /// position (`CVec3`, three `float`s).
    pub struct CIdvCamera: 0x10 {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_cPosition.x` (Xbox PDB `m_cPosition`, a `CVec3`).
        0x04 m_cPosition_x: u32,
        /// `m_cPosition.y`.
        0x08 m_cPosition_y: u32,
        /// `m_cPosition.z`.
        0x0C m_cPosition_z: u32,
    }
}

// Translated from 00b3f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIdvCamera::operator=` (Xbox PDB): copies the position (three words, as
/// the code moves them) from `other` unless it is the same object. Returns
/// `this`.
pub fn cidv_camera_assign(
    e: &mut Engine,
    this: Ptr<CIdvCamera>,
    other: Ptr<CIdvCamera>,
) -> Ptr<CIdvCamera> {
    if other != this {
        let x = e.get(other, CIdvCamera::m_cPosition_x);
        e.set(this, CIdvCamera::m_cPosition_x, x);
        let y = e.get(other, CIdvCamera::m_cPosition_y);
        e.set(this, CIdvCamera::m_cPosition_y, y);
        let z = e.get(other, CIdvCamera::m_cPosition_z);
        e.set(this, CIdvCamera::m_cPosition_z, z);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00b3f0c0,
        cidv_camera_assign(Ptr<CIdvCamera>, Ptr<CIdvCamera>) -> Ptr<CIdvCamera>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_copies_the_position_and_returns_this() {
        let mut e = Engine::new();
        let this: Ptr<CIdvCamera> = e.new_object();
        let other: Ptr<CIdvCamera> = e.new_object();
        e.set(this, CIdvCamera::vtable, 0x1111);
        e.set(other, CIdvCamera::vtable, 0x2222);
        e.set(other, CIdvCamera::m_cPosition_x, 1.5f32.to_bits());
        e.set(other, CIdvCamera::m_cPosition_y, 2.5f32.to_bits());
        e.set(other, CIdvCamera::m_cPosition_z, 0x7f80_0001);
        let back = e.call(0x00b3_f0c0, &args![this, other]).ptr::<CIdvCamera>();
        assert_eq!(back, this);
        assert_eq!(e.get(this, CIdvCamera::m_cPosition_x), 1.5f32.to_bits());
        assert_eq!(e.get(this, CIdvCamera::m_cPosition_y), 2.5f32.to_bits());
        // The words move unchanged (a signalling NaN stays signalling).
        assert_eq!(e.get(this, CIdvCamera::m_cPosition_z), 0x7f80_0001);
        // The vtable stays.
        assert_eq!(e.get(this, CIdvCamera::vtable), 0x1111);
    }

    #[test]
    fn assign_to_itself_changes_nothing() {
        let mut e = Engine::new();
        let this: Ptr<CIdvCamera> = e.new_object();
        e.set(this, CIdvCamera::m_cPosition_x, 7);
        assert_eq!(
            e.call(0x00b3_f0c0, &args![this, this]).ptr::<CIdvCamera>(),
            this
        );
        assert_eq!(e.get(this, CIdvCamera::m_cPosition_x), 7);
    }
}
