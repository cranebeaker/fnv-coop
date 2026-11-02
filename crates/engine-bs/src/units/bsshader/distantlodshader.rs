//! `BSShader/distantlodshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    add_constant_entry, make_constant_map, CONSTANT_EMPTY_NAME_01011584,
};

/// The constant names (read in Ghidra) and the data they point at outside the shader.
const WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const DIFFUSE_LIGHT_DIRECTION_010B9C00: u32 = 0x010b_9c00;
const DIFFUSE_LIGHT_COLOR_010B9BEC: u32 = 0x010b_9bec;
const AMBIENT_COLOR_010B8FE4: u32 = 0x010b_8fe4;
const EYE_VECTOR_010BC3CC: u32 = 0x010b_c3cc;
const EYE_RIGHT_VECTOR_010BC3B8: u32 = 0x010b_c3b8;
const ALPHA_PARAM_PARAM_010BC3A4: u32 = 0x010b_c3a4;
const FOG_PARAM_010B8C54: u32 = 0x010b_8c54;
const FOG_COLOR_010B8C48: u32 = 0x010b_8c48;
const INSTANCE_DATA_010B8C24: u32 = 0x010b_8c24;
const ALPHA_TEST_REF_010B8BEC: u32 = 0x010b_8bec;
const ALPHA_PARAM_011AD860: u32 = 0x011a_d860;
const FOG_PARAM_011FA280: u32 = 0x011f_a280;
const FOG_COLOR_011FA290: u32 = 0x011f_a290;

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

// Translated from 00bd5a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DistantLODShader::CreateConstantMaps` (Xbox PDB), no stack words. When the vertex
/// constant map (+0x34) is null, makes one (kind 0) and adds `"WorldViewProjTranspose"`
/// (0x20000009, 0, 0, 4), `"Diffuse Light direction"` (+0xb0, register 4), `"Diffuse Light
/// color"` (+0xc0, 5), `"ambient color"` (+0xa0, 0xd), `"eye vector"` (+0xe0, 0xe), `"eye
/// right vector"` (+0xf0, 9), `"alpha param param"` (`011ad860`, 0xc), `"fogparam"`
/// (`011fa280`, 0xb), `"fogcolor"` (`011fa290`, 0xa) and `"instance data"` (register 0x14,
/// the `u16` count +0x9c, 0x10 bytes each, the array +0x94), and keeps the map's slot +0x70
/// answer for `"instance data"` in +0x98; when the pixel constant map (+0x30) is null, makes
/// one (kind 2) and adds `"alpha test ref"` (+0xd0, register 3).
// Translation incomplete: 00bd5a60 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn distant_lod_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x34) == 0 {
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
            vector_entry(DIFFUSE_LIGHT_DIRECTION_010B9C00, 4, base + 0xb0),
            vector_entry(DIFFUSE_LIGHT_COLOR_010B9BEC, 5, base + 0xc0),
            vector_entry(AMBIENT_COLOR_010B8FE4, 0xd, base + 0xa0),
            vector_entry(EYE_VECTOR_010BC3CC, 0xe, base + 0xe0),
            vector_entry(EYE_RIGHT_VECTOR_010BC3B8, 9, base + 0xf0),
            vector_entry(ALPHA_PARAM_PARAM_010BC3A4, 0xc, ALPHA_PARAM_011AD860),
            vector_entry(FOG_PARAM_010B8C54, 0xb, FOG_PARAM_011FA280),
            vector_entry(FOG_COLOR_010B8C48, 0xa, FOG_COLOR_011FA290),
        ];
        for words in entries {
            add_constant_entry(e, base, 0x34, words);
        }
        let count = u32::from(e.mem.u16(base + 0x9c));
        let instances = e.mem.u32(base + 0x94);
        add_constant_entry(
            e,
            base,
            0x34,
            [
                INSTANCE_DATA_010B8C24,
                0x1000_0009,
                0,
                0x14,
                count,
                CONSTANT_EMPTY_NAME_01011584,
                count << 4,
                4,
                instances,
                0,
            ],
        );
        let map = e.mem.u32(base + 0x34);
        let entry = e.vcall(map, 0x70, &args![INSTANCE_DATA_010B8C24]).u32();
        e.mem.set_u32(base + 0x98, entry);
    }
    if e.mem.u32(base + 0x30) == 0 {
        make_constant_map(e, base, 0x30, 2);
        add_constant_entry(
            e,
            base,
            0x30,
            vector_entry(ALPHA_TEST_REF_010B8BEC, 3, base + 0xd0),
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bd_5a60,
        distant_lod_shader_create_constant_maps(Ptr)
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

    #[test]
    fn create_constant_maps_adds_the_entries_once() {
        let mut e = rig();
        let targets: Vec<u32> = (0..0x20u32).map(|i| MAP_SLOTS + 4 * i).collect();
        e.put_vtable(MAP_VTABLE, &targets);
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCT_00B7E330, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            a[0].into_ret()
        });
        e.register(MAP_SLOTS + 8, |_, _| Ret::default());
        e.register(MAP_SLOTS + 0x70, |_, _| 0x7171u32.into_ret());
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x18, 0x5151);
        e.mem.set_u16(this + 0x9c, 0xe4);
        e.mem.set_u32(this + 0x94, 0x9000);
        e.call(0x00bd_5a60, &args![this]);
        let maps = calls(&e, CONSTANT_MAP_CONSTRUCT_00B7E330);
        assert_eq!(maps.len(), 2);
        assert_eq!(&maps[0][1..], &[0x5151, 0]);
        assert_eq!(&maps[1][1..], &[0x5151, 2]);
        let vertex = e.mem.u32(this + 0x34);
        let pixel = e.mem.u32(this + 0x30);
        let entries = calls(&e, MAP_SLOTS + 8);
        assert_eq!(entries.len(), 11);
        assert!(entries[..10].iter().all(|a| a[0] == vertex));
        assert_eq!(
            entries[1],
            vec![
                vertex,
                DIFFUSE_LIGHT_DIRECTION_010B9C00,
                0x1000_0007,
                0,
                4,
                1,
                CONSTANT_EMPTY_NAME_01011584,
                0x10,
                4,
                this + 0xb0,
                0
            ]
        );
        assert_eq!(
            entries[9],
            vec![
                vertex,
                INSTANCE_DATA_010B8C24,
                0x1000_0009,
                0,
                0x14,
                0xe4,
                CONSTANT_EMPTY_NAME_01011584,
                0xe40,
                4,
                0x9000,
                0
            ]
        );
        assert_eq!(entries[10][0], pixel);
        assert_eq!(entries[10][1], ALPHA_TEST_REF_010B8BEC);
        assert_eq!(entries[10][9], this + 0xd0);
        assert_eq!(e.mem.u32(this + 0x98), 0x7171);
        e.call_log = Some(vec![]);
        e.call(0x00bd_5a60, &args![this]);
        assert!(calls(&e, MAP_SLOTS + 8).is_empty());
    }
}
