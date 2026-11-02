//! `NiMain/nitexturetransform.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTextureTransform::m_bMatrixDirty` (Xbox PDB) +0x1C and `m_kMatrix` +0x20.
const MATRIX_DIRTY: u32 = 0x1c;
const MATRIX: u32 = 0x20;
/// The matrix rebuild `00a88cd0` (unnamed in the engine map; thiscall on the
/// transform, no arguments).
const REBUILD_MATRIX_00A88CD0: u32 = 0x00a8_8cd0;

// Translated from 00bc5c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransform::UpdateMatrix` (Xbox PDB; thiscall, no arguments):
/// rebuilds the matrix through `00a88cd0` when the dirty byte (+0x1C) is set,
/// then returns the address of the matrix (+0x20).
pub fn ni_texture_transform_update_matrix(e: &mut Engine, this: Ptr) -> Ptr {
    if e.mem.u8(this.addr() + MATRIX_DIRTY) != 0 {
        e.call(REBUILD_MATRIX_00A88CD0, &args![this]);
    }
    Ptr::new(this.addr() + MATRIX)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bc_5c20,
        ni_texture_transform_update_matrix(Ptr) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rebuilds_only_a_dirty_matrix() {
        let mut e = Engine::new();
        e.register(REBUILD_MATRIX_00A88CD0, |e, a| {
            e.mem.set_u8(a[0] + MATRIX_DIRTY, 0);
            Ret::default()
        });
        let transform = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        let matrix = e.call(0x00bc_5c20, &args![transform]).u32();
        assert_eq!(matrix, transform + MATRIX);
        let rebuilds = |e: &Engine| {
            e.call_log
                .iter()
                .flatten()
                .filter(|(a, _)| *a == REBUILD_MATRIX_00A88CD0)
                .count()
        };
        assert_eq!(rebuilds(&e), 0);
        e.mem.set_u8(transform + MATRIX_DIRTY, 1);
        assert_eq!(
            e.call(0x00bc_5c20, &args![transform]).u32(),
            transform + MATRIX
        );
        assert_eq!(rebuilds(&e), 1);
    }
}
