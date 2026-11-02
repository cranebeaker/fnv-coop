//! `BSShader/tileshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::particleshader::constant_maps::{
    add_entry, make_constant_map, EMPTY_STRING_01011584,
};

/// Strings of `TileShader::CreateConstantMaps` (read in Ghidra): `"tintcolor"` (`010bb56c`),
/// `"WorldViewProjTranspose"` (`010a1e50`) and `"TexScroll"` (`010bb560`).
const NAME_TINT_COLOR_010BB56C: u32 = 0x010b_b56c;
const NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const NAME_TEX_SCROLL_010BB560: u32 = 0x010b_b560;
/// The constants' data: the tint colour (`01202188`) and the texture scroll (`01202198`).
const TINT_COLOR_01202188: u32 = 0x0120_2188;
const TEX_SCROLL_01202198: u32 = 0x0120_2198;

// Translated from 00bcaff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileShader::CreateConstantMaps` (engine map), `thiscall`, no stack words. When the
/// `NiPointer` at +0x30 is null: a constant map (program type 2, the word at +0x18 as the
/// renderer) into it with the entry (vtable slot +0x8) `"tintcolor"` (0x10000007, register 0,
/// 0x10 bytes at `01202188`). When the `NiPointer` at +0x34 is null: a constant map of program
/// type 0 into it with `"WorldViewProjTranspose"` (0x20000009, 4 registers, no data) and
/// `"TexScroll"` (0x10000007, register 4, 0x10 bytes at `01202198`).
// Translation incomplete: 00bcaff0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn tile_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let pixel_map = base + 0x30;
    if e.mem.u32(pixel_map) == 0 {
        make_constant_map(e, pixel_map, base + 0x18, 2);
        add_entry(
            e,
            pixel_map,
            &[
                NAME_TINT_COLOR_010BB56C,
                0x1000_0007,
                0,
                0,
                1,
                EMPTY_STRING_01011584,
                0x10,
                4,
                TINT_COLOR_01202188,
                0,
            ],
        );
    }
    let vertex_map = base + 0x34;
    if e.mem.u32(vertex_map) == 0 {
        make_constant_map(e, vertex_map, base + 0x18, 0);
        add_entry(
            e,
            vertex_map,
            &[
                NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                0x2000_0009,
                0,
                0,
                4,
                0,
                0,
                0,
                0,
                0,
            ],
        );
        add_entry(
            e,
            vertex_map,
            &[
                NAME_TEX_SCROLL_010BB560,
                0x1000_0007,
                0,
                4,
                1,
                EMPTY_STRING_01011584,
                0x10,
                4,
                TEX_SCROLL_01202198,
                0,
            ],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bc_aff0, tile_shader_create_constant_maps(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;
    use crate::units::bsshader::particleshader::constant_map_rig::*;
    use crate::units::bsshader::particleshader::constant_maps::CONSTANT_MAP_CONSTRUCT;

    #[test]
    fn tile_create_constant_maps_adds_the_tint_and_the_scroll() {
        let mut e = map_rig();
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x18, 0x0e50_0000);
        e.call(0x00bc_aff0, &args![this]);
        let pixel = e.mem.u32(this + 0x30);
        let vertex = e.mem.u32(this + 0x34);
        assert_eq!(map_made_with(&e, pixel), (0x0e50_0000, 2));
        assert_eq!(map_made_with(&e, vertex), (0x0e50_0000, 0));
        assert_eq!(
            entries(&e, pixel),
            vec![vec![
                NAME_TINT_COLOR_010BB56C,
                0x1000_0007,
                0,
                0,
                1,
                EMPTY_STRING_01011584,
                0x10,
                4,
                TINT_COLOR_01202188,
                0
            ]]
        );
        let added = entries(&e, vertex);
        assert_eq!(added.len(), 2);
        assert_eq!(
            added[0][..5],
            [
                NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                0x2000_0009,
                0,
                0,
                4
            ]
        );
        assert_eq!(added[1][3], 4);
        assert_eq!(added[1][8], TEX_SCROLL_01202198);
        // Both maps there: nothing is made.
        e.call(0x00bc_aff0, &args![this]);
        assert_eq!(calls(&e, CONSTANT_MAP_CONSTRUCT).len(), 2);
    }
}
