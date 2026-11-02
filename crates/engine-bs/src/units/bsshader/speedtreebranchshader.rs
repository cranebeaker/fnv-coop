//! `BSShader/speedtreebranchshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The pointer to the geometry being drawn (`011f91e0`).
const CURRENT_GEOMETRY: u32 = 0x011f_91e0;
/// The skin instance used when the geometry has none at `+0x20` (`011f4288`).
const DEFAULT_SKIN: u32 = 0x011f_4288;
/// The fog setting byte (`011f91c4`).
const FOG_SETTING: u32 = 0x011f_91c4;
/// The constant maps object (`0126f74c`): the maps at `+0x44` and `+0x5c`.
const CONSTANT_MAPS: u32 = 0x0126_f74c;
/// `ShadowLightShader::SetupGeometryConstants` (engine map), thiscall (shader, pass).
const SHADOW_LIGHT_SETUP_GEOMETRY_CONSTANTS: u32 = 0x00b7_dab0;
/// `BSShaderManager::GetFogProperty` (engine map), cdecl (setting).
const GET_FOG_PROPERTY: u32 = 0x00b5_5520;
/// `BSShaderManager::GetShader` (engine map), cdecl (index).
const GET_SHADER: u32 = 0x00b5_5560;
/// The shader's vtable slot called with `(geometry, fog, pass word +8)` (+0x154).
const SHADER_SLOT_154: u32 = 0x154;
/// The constant map's vtable slot that sets its constants (+0x78, 11 words).
const MAP_SLOT_SET_CONSTANTS: u32 = 0x78;

/// One constant map's update: `map.SetConstants(constants, geometry, +0xbc,
/// 0, +0x34 of the data, pass, 0, geometry + 0x68, skin, 0, 1)`.
#[allow(clippy::too_many_arguments)]
fn set_constants(
    e: &mut Engine,
    map: u32,
    constants: u32,
    geometry: u32,
    word_bc: u32,
    word_34: u32,
    pass: u32,
    skin: u32,
) {
    e.vcall(
        map,
        MAP_SLOT_SET_CONSTANTS,
        &args![
            constants,
            geometry,
            word_bc,
            0u32,
            word_34,
            pass,
            0u32,
            geometry + 0x68,
            skin,
            0u32,
            1u32
        ],
    );
}

// Translated from 00bd4ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpeedTreeBranchShader::SetupGeometryConstants` (Xbox PDB), thiscall
/// `(pass)`: for the geometry at `011f91e0` (its skin at `+0x20`, or
/// `011f4288` without one), runs `ShadowLightShader::SetupGeometryConstants`,
/// gives the shader's vtable +0x154 the geometry, the fog property for the
/// byte at `011f91c4` and the pass's word at `+8`, then sets the constants
/// of shader 1's maps at `+0x30` (constants `+0x44` of `0126f74c`) and
/// `+0x34` (constants `+0x5c`), and of this shader's maps at `+0x30` and
/// `+0x34` when they and the constants exist.
pub fn speed_tree_branch_shader_setup_geometry_constants(e: &mut Engine, this: Ptr, pass: u32) {
    let base = this.addr();
    let current: u32 = e.global(CURRENT_GEOMETRY);
    let geometry = e.mem.u32(current);
    let mut skin = e.mem.u32(geometry + 0x20);
    if skin == 0 {
        skin = DEFAULT_SKIN;
    }
    let data = e.mem.u32(geometry + 0xb8);
    let word = e.mem.u32(pass.wrapping_add(8));
    let word_34 = e.mem.u32(data.wrapping_add(0x34));
    let word_bc = e.mem.u32(geometry + 0xbc);
    e.call(SHADOW_LIGHT_SETUP_GEOMETRY_CONSTANTS, &args![base, pass]);
    let fog_setting = u32::from(e.global::<u8>(FOG_SETTING));
    let fog = e.call(GET_FOG_PROPERTY, &args![fog_setting]).u32();
    e.vcall(base, SHADER_SLOT_154, &args![geometry, fog, word]);
    let shader = e.call(GET_SHADER, &args![1u32]).u32();
    for (map_offset, constants_offset) in [(0x30u32, 0x44u32), (0x34, 0x5c)] {
        let map = e.mem.u32(shader + map_offset);
        let maps: u32 = e.global(CONSTANT_MAPS);
        let constants = e.mem.u32(maps + constants_offset);
        set_constants(e, map, constants, geometry, word_bc, word_34, pass, skin);
    }
    for (map_offset, constants_offset) in [(0x30u32, 0x44u32), (0x34, 0x5c)] {
        let map = e.mem.u32(base + map_offset);
        if map == 0 {
            continue;
        }
        let maps: u32 = e.global(CONSTANT_MAPS);
        let constants = e.mem.u32(maps + constants_offset);
        if constants != 0 {
            set_constants(e, map, constants, geometry, word_bc, word_34, pass, skin);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bd4ba0,
        speed_tree_branch_shader_setup_geometry_constants(Ptr, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOT_154_DOUBLE: u32 = 0xdead_9000;
    const MAP_DOUBLE: u32 = 0xdead_9010;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn map(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + MAP_SLOT_SET_CONSTANTS, MAP_DOUBLE);
        let map = e.mem.alloc(8);
        e.mem.set_u32(map, vtable);
        map
    }

    #[test]
    fn sets_the_constants_of_both_shaders_maps() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e.set_global(FOG_SETTING, 3u8);
        let data = e.mem.alloc(0x38);
        e.mem.set_u32(data + 0x34, 0x34);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u32(geometry + 0xbc, 0xbc);
        let current = e.mem.alloc(4);
        e.mem.set_u32(current, geometry);
        e.set_global(CURRENT_GEOMETRY, current);
        let constants = e.mem.alloc(0x60);
        e.mem.set_u32(constants + 0x44, 0x44);
        e.mem.set_u32(constants + 0x5c, 0x5c);
        e.set_global(CONSTANT_MAPS, constants);
        let (shader_a, shader_b, own_a) = (map(&mut e), map(&mut e), map(&mut e));
        let shader = e.mem.alloc(0x38);
        e.mem.set_u32(shader + 0x30, shader_a);
        e.mem.set_u32(shader + 0x34, shader_b);
        e.register_double(GET_SHADER, move |_, _| Ret {
            eax: shader,
            ..Ret::default()
        });
        e.register(GET_FOG_PROPERTY, |_, a| Ret {
            eax: 0xf000 + a[0],
            ..Ret::default()
        });
        e.register(SHADOW_LIGHT_SETUP_GEOMETRY_CONSTANTS, |_, _| Ret::default());
        e.register(SLOT_154_DOUBLE, |_, _| Ret::default());
        e.register(MAP_DOUBLE, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x160);
        e.mem.set_u32(vtable + SHADER_SLOT_154, SLOT_154_DOUBLE);
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x30, own_a);
        let pass = e.mem.alloc(0xc);
        e.mem.set_u32(pass + 8, 0x88);
        e.call_log = Some(vec![]);
        e.call(0x00bd_4ba0, &args![this, pass]);
        assert_eq!(
            calls(&e, SHADOW_LIGHT_SETUP_GEOMETRY_CONSTANTS),
            vec![vec![this, pass]]
        );
        assert_eq!(calls(&e, GET_FOG_PROPERTY), vec![vec![3]]);
        assert_eq!(
            calls(&e, SLOT_154_DOUBLE),
            vec![vec![this, geometry, 0xf003, 0x88]]
        );
        let tail = |map: u32, constants: u32| {
            vec![
                map,
                constants,
                geometry,
                0xbc,
                0,
                0x34,
                pass,
                0,
                geometry + 0x68,
                DEFAULT_SKIN,
                0,
                1,
            ]
        };
        assert_eq!(
            calls(&e, MAP_DOUBLE),
            vec![
                tail(shader_a, 0x44),
                tail(shader_b, 0x5c),
                tail(own_a, 0x44)
            ]
        );
        // Without the constants the own maps are skipped.
        e.mem.set_u32(constants + 0x44, 0);
        e.mem.set_u32(geometry + 0x20, 0x2020);
        e.call_log = Some(vec![]);
        e.call(0x00bd_4ba0, &args![this, pass]);
        let maps = calls(&e, MAP_DOUBLE);
        assert_eq!(maps.len(), 2);
        assert_eq!(maps[0][9], 0x2020);
    }
}
