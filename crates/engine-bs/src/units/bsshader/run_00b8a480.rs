//! `BSShader/run_00b8a480` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    add_constant_entry, make_constant_map, CONSTANT_EMPTY_NAME_01011584,
};

/// The constant names (read in Ghidra) and the data outside the shader they point at.
const PARAMS_010AFF78: u32 = 0x010a_ff78;
const WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const WORLD_TRANSPOSE_010A1F08: u32 = 0x010a_1f08;
const COLOR0_010AFF70: u32 = 0x010a_ff70;
const COLOR1_010AFF68: u32 = 0x010a_ff68;
const COLOR2_010AFF60: u32 = 0x010a_ff60;
const EYEPOS_010AFF58: u32 = 0x010a_ff58;
const COLOR0_011FF910: u32 = 0x011f_f910;
const COLOR1_011FF920: u32 = 0x011f_f920;
const COLOR2_011FF930: u32 = 0x011f_f930;
const EYEPOS_011FF8C8: u32 = 0x011f_f8c8;

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

// Translated from 00b8a480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b8a480`; the sky shader's vtable `010afe18` slot +0x11c,
/// its `CreateConstantMaps`), no stack words. When the pixel constant map (+0x30) is null,
/// makes one (kind 2) with `"Params"` (0x10000005, register 4, 8 bytes at +0xe8); when the
/// vertex one (+0x34) is null, makes one (kind 0) with `"WorldViewProjTranspose"`
/// (0x20000009, 0, 0, 4), `"WorldTranspose"` (0x20000009, 0, 8, 4), `"Color0"` (`011ff910`,
/// 4), `"Color1"` (`011ff920`, 5), `"Color2"` (`011ff930`, 6), `"eyepos"` (`011ff8c8`, 7) and
/// `"Params"` (0x10000004, register 0xc, 4 bytes at +0xe4).
// Translation incomplete: 00b8a480 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn fn_00b8a480(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let empty = CONSTANT_EMPTY_NAME_01011584;
    if e.mem.u32(base + 0x30) == 0 {
        make_constant_map(e, base, 0x30, 2);
        add_constant_entry(
            e,
            base,
            0x30,
            [
                PARAMS_010AFF78,
                0x1000_0005,
                0,
                4,
                1,
                empty,
                8,
                4,
                base + 0xe8,
                0,
            ],
        );
    }
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
        [
            WORLD_TRANSPOSE_010A1F08,
            0x2000_0009,
            0,
            8,
            4,
            0,
            0,
            0,
            0,
            0,
        ],
        vector_entry(COLOR0_010AFF70, 4, COLOR0_011FF910),
        vector_entry(COLOR1_010AFF68, 5, COLOR1_011FF920),
        vector_entry(COLOR2_010AFF60, 6, COLOR2_011FF930),
        vector_entry(EYEPOS_010AFF58, 7, EYEPOS_011FF8C8),
        [
            PARAMS_010AFF78,
            0x1000_0004,
            0,
            0xc,
            1,
            empty,
            4,
            4,
            base + 0xe4,
            0,
        ],
    ];
    for words in entries {
        add_constant_entry(e, base, 0x34, words);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b8_a480, fn_00b8a480(Ptr))]
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
    fn fn_00b8a480_makes_the_sky_maps() {
        let mut e = rig();
        let targets: Vec<u32> = (0..0x20u32).map(|i| MAP_SLOTS + 4 * i).collect();
        e.put_vtable(MAP_VTABLE, &targets);
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCT_00B7E330, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            a[0].into_ret()
        });
        e.register(MAP_SLOTS + 8, |_, _| Ret::default());
        let this = e.mem.alloc(0xf0);
        e.call(0x00b8_a480, &args![this]);
        let pixel = e.mem.u32(this + 0x30);
        let vertex = e.mem.u32(this + 0x34);
        let entries = calls(&e, MAP_SLOTS + 8);
        assert_eq!(entries.len(), 8);
        assert_eq!(
            entries[0],
            vec![
                pixel,
                PARAMS_010AFF78,
                0x1000_0005,
                0,
                4,
                1,
                CONSTANT_EMPTY_NAME_01011584,
                8,
                4,
                this + 0xe8,
                0
            ]
        );
        assert_eq!(
            entries[2][..5],
            [vertex, WORLD_TRANSPOSE_010A1F08, 0x2000_0009, 0, 8]
        );
        assert_eq!(entries[6][9], EYEPOS_011FF8C8);
        assert_eq!(
            entries[7][1..],
            [
                PARAMS_010AFF78,
                0x1000_0004,
                0,
                0xc,
                1,
                CONSTANT_EMPTY_NAME_01011584,
                4,
                4,
                this + 0xe4,
                0
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00b8_a480, &args![this]);
        assert!(calls(&e, MAP_SLOTS + 8).is_empty());
    }
}
