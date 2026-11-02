//! `BSShader/speedtreefrondshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    add_constant_entry, make_constant_map, CONSTANT_EMPTY_NAME_01011584,
};

/// The constant names (read in Ghidra) and the data outside the shader they point at.
const WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const AMBIENT_COLOR_010AF568: u32 = 0x010a_f568;
const DIFF_COLOR_0_010B92F8: u32 = 0x010b_92f8;
const DIFF_COLOR_1_010B92E8: u32 = 0x010b_92e8;
const DIRECTRONAL_LIGHT_DIR_010BC014: u32 = 0x010b_c014;
const POINT_LIGHT_POS_010BC004: u32 = 0x010b_c004;
const LIGHT_RADIUS_010BBFF8: u32 = 0x010b_bff8;
const FOG_PARAM_010AF60C: u32 = 0x010a_f60c;
const FOG_COLOR_010AF5E4: u32 = 0x010a_f5e4;
const TREE_DIMMER_010B92DC: u32 = 0x010b_92dc;
const AMBIENT_011FA0C0: u32 = 0x011f_a0c0;
const DIFFUSE_0_011FA0D0: u32 = 0x011f_a0d0;
const DIFFUSE_1_011FA0E0: u32 = 0x011f_a0e0;
const LIGHT_RADIUS_011FA1F0: u32 = 0x011f_a1f0;
const FOG_PARAM_011FA280: u32 = 0x011f_a280;
const FOG_COLOR_011FA290: u32 = 0x011f_a290;
const TREE_DIMMER_011F918C: u32 = 0x011f_918c;
/// The byte that adds `"Tree Dimmer"` (`011f941e`) and `SpeedTreeShader::SetShaderConstants`
/// (Xbox PDB, `00bb2f50`, `cdecl`: the map, the first register).
const DIMMER_SWITCH_011F941E: u32 = 0x011f_941e;
const SET_SHADER_CONSTANTS_00BB2F50: u32 = 0x00bb_2f50;

/// One `0x10000007` entry: one register of 0x10 bytes at `data`.
fn vector_entry(name: u32, register: u32, data: u32) -> [u32; 10] {
    [
        name,
        0x1000_0007,
        0,
        register,
        1,
        CONSTANT_EMPTY_NAME_01011584,
        0x10,
        4,
        data,
        0,
    ]
}

// Translated from 00bd3000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpeedTreeFrondShader::CreateConstantMaps` (Xbox PDB), no stack words. When the vertex
/// constant map (+0x34) is null: makes one (kind 0) and adds `"WorldViewProjTranspose"`
/// (0x20000009, 0, 0, 4), `"Ambient Color"` (`011fa0c0`, register 5), `"Diff Color 0"`
/// (`011fa0d0`, 6), `"Diff Color 1"` (`011fa0e0`, 7), `"DirectronalLightDir"` (+0x98, 8),
/// `"PointLightPos"` (+0x88, 9), `"LightRadius"` (`011fa1f0`, 0xa), `"FogParam |
/// ShadowVolumeFatness"` (`011fa280`, 0xe), `"FogColor | ShadowVolumeExtrudeDistance"`
/// (`011fa290`, 0xf), with the byte `011f941e` set `"Tree Dimmer"` (0x10000004, register 0x10,
/// 4 bytes at `011f918c`), then `SpeedTreeShader::SetShaderConstants` with the map and 0x11.
// Translation incomplete: 00bd3000 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn speed_tree_frond_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x34) != 0 {
        return;
    }
    make_constant_map(e, base, 0x34, 0);
    let entries = [
        [
            WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
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
        vector_entry(AMBIENT_COLOR_010AF568, 5, AMBIENT_011FA0C0),
        vector_entry(DIFF_COLOR_0_010B92F8, 6, DIFFUSE_0_011FA0D0),
        vector_entry(DIFF_COLOR_1_010B92E8, 7, DIFFUSE_1_011FA0E0),
        vector_entry(DIRECTRONAL_LIGHT_DIR_010BC014, 8, base + 0x98),
        vector_entry(POINT_LIGHT_POS_010BC004, 9, base + 0x88),
        vector_entry(LIGHT_RADIUS_010BBFF8, 0xa, LIGHT_RADIUS_011FA1F0),
        vector_entry(FOG_PARAM_010AF60C, 0xe, FOG_PARAM_011FA280),
        vector_entry(FOG_COLOR_010AF5E4, 0xf, FOG_COLOR_011FA290),
    ];
    for words in entries {
        add_constant_entry(e, base, 0x34, words);
    }
    if e.global::<u8>(DIMMER_SWITCH_011F941E) != 0 {
        add_constant_entry(
            e,
            base,
            0x34,
            [
                TREE_DIMMER_010B92DC,
                0x1000_0004,
                0,
                0x10,
                1,
                CONSTANT_EMPTY_NAME_01011584,
                4,
                4,
                TREE_DIMMER_011F918C,
                0,
            ],
        );
    }
    let map = e.mem.u32(base + 0x34);
    e.call(SET_SHADER_CONSTANTS_00BB2F50, &args![map, 0x11u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bd_3000,
        speed_tree_frond_shader_create_constant_maps(Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::{
        CONSTANT_MAP_CONSTRUCT_00B7E330, NI_ALLOCATE_00AA13E0,
    };
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const MAP_SLOTS: u32 = 0x0e30_0000;
    const MAP_VTABLE: u32 = 0x0e31_0000;

    fn run(dimmer: u8) -> (Engine, u32) {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global(DIMMER_SWITCH_011F941E, dimmer);
        let targets: Vec<u32> = (0..0x20u32).map(|i| MAP_SLOTS + 4 * i).collect();
        e.put_vtable(MAP_VTABLE, &targets);
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCT_00B7E330, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            a[0].into_ret()
        });
        e.register(MAP_SLOTS + 8, |_, _| Ret::default());
        e.register(SET_SHADER_CONSTANTS_00BB2F50, |_, _| Ret::default());
        let this = e.mem.alloc(0xa8);
        e.call(0x00bd_3000, &args![this]);
        (e, this)
    }

    #[test]
    fn create_constant_maps_adds_the_vertex_constants() {
        let (e, this) = run(0);
        let map = e.mem.u32(this + 0x34);
        let entries = calls(&e, MAP_SLOTS + 8);
        assert_eq!(entries.len(), 9);
        assert_eq!(
            entries[4],
            vec![
                map,
                DIRECTRONAL_LIGHT_DIR_010BC014,
                0x1000_0007,
                0,
                8,
                1,
                CONSTANT_EMPTY_NAME_01011584,
                0x10,
                4,
                this + 0x98,
                0
            ]
        );
        assert_eq!(entries[8][1], FOG_COLOR_010AF5E4);
        assert_eq!(
            calls(&e, SET_SHADER_CONSTANTS_00BB2F50),
            vec![vec![map, 0x11]]
        );
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }

    #[test]
    fn create_constant_maps_adds_the_dimmer_and_runs_once() {
        let (mut e, this) = run(1);
        let entries = calls(&e, MAP_SLOTS + 8);
        assert_eq!(entries.len(), 10);
        assert_eq!(
            &entries[9][1..],
            &[
                TREE_DIMMER_010B92DC,
                0x1000_0004,
                0,
                0x10,
                1,
                CONSTANT_EMPTY_NAME_01011584,
                4,
                4,
                TREE_DIMMER_011F918C,
                0
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00bd_3000, &args![this]);
        assert!(calls(&e, MAP_SLOTS + 8).is_empty());
        assert!(calls(&e, SET_SHADER_CONSTANTS_00BB2F50).is_empty());
    }
}
