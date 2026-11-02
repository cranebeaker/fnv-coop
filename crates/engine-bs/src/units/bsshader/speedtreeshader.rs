//! `BSShader/speedtreeshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The constant names (read in Ghidra: `"TreeData"`, `"WindMatrices"`), the empty string and
/// the data the entries point at (`01200678`, one register; `01200688`, 0x10 registers).
const TREE_DATA_010B932C: u32 = 0x010b_932c;
const WIND_MATRICES_010B931C: u32 = 0x010b_931c;
const EMPTY_01011584: u32 = 0x0101_1584;
const TREE_DATA_01200678: u32 = 0x0120_0678;
const WIND_MATRICES_01200688: u32 = 0x0120_0688;

// Translated from 00bb2f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpeedTreeShader::SetShaderConstants` (Xbox PDB), `cdecl`: the constant map and the first
/// register. Adds through the map's vtable slot +8 `"TreeData"` (0x10000007, register
/// `register`, one register of 0x10 bytes at `01200678`) and `"WindMatrices"` (0x10000009,
/// register `register + 1`, 0x10 registers, 0x100 bytes at `01200688`).
pub fn speed_tree_shader_set_shader_constants(e: &mut Engine, map: Ptr, register: u32) {
    let map = map.addr();
    e.vcall(
        map,
        8,
        &args![
            TREE_DATA_010B932C,
            0x1000_0007u32,
            0u32,
            register,
            1u32,
            EMPTY_01011584,
            0x10u32,
            4u32,
            TREE_DATA_01200678,
            0u32
        ],
    );
    e.vcall(
        map,
        8,
        &args![
            WIND_MATRICES_010B931C,
            0x1000_0009u32,
            0u32,
            register.wrapping_add(1),
            0x10u32,
            EMPTY_01011584,
            0x100u32,
            4u32,
            WIND_MATRICES_01200688,
            0u32
        ],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bb_2f50,
        speed_tree_shader_set_shader_constants(Ptr, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_shader_constants_adds_the_tree_data_and_wind_matrices() {
        const ADD: u32 = 0x0e00_0008;
        let mut e = Engine::new();
        e.put_vtable(0x0e10_0000, &[0, 0, ADD]);
        e.register(ADD, |_, _| Ret::default());
        let map = e.mem.alloc(8);
        e.mem.set_u32(map, 0x0e10_0000);
        e.call_log = Some(vec![]);
        e.call(0x00bb_2f50, &args![map, 0x2au32]);
        let log = e.call_log.take().unwrap();
        let adds: Vec<_> = log
            .iter()
            .filter(|c| c.0 == ADD)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            adds,
            vec![
                vec![
                    map,
                    TREE_DATA_010B932C,
                    0x1000_0007,
                    0,
                    0x2a,
                    1,
                    EMPTY_01011584,
                    0x10,
                    4,
                    TREE_DATA_01200678,
                    0
                ],
                vec![
                    map,
                    WIND_MATRICES_010B931C,
                    0x1000_0009,
                    0,
                    0x2b,
                    0x10,
                    EMPTY_01011584,
                    0x100,
                    4,
                    WIND_MATRICES_01200688,
                    0
                ],
            ]
        );
    }
}
