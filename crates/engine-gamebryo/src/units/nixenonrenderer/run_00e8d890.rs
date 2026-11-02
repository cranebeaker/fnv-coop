//! `NiXenonRenderer/run_00e8d890` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// What the stage set-up calls: `00a463e0` (`thiscall` on the texturing property, an index: its
/// shader map array's item, or 0), `00e7ddf0` and `00e8d3a0` (`thiscall` on the stage, no
/// stack words: two state reads through the stage's +0x10), `00e88fc0` (`thiscall` on the
/// stage's +0x10, three words), `00e8d3d0` (`cdecl`, four words: the light, the stage, a byte
/// and the shader's +0x18), `00e7de90` (`thiscall` on the stage: the clamp mode) and
/// `NiD3DTextureStage::SetFilterMode` (`00e7def0`, `thiscall`: the filter mode).
const SHADER_MAP_00A463E0: u32 = 0x00a4_63e0;
const STAGE_READ_00E7DDF0: u32 = 0x00e7_ddf0;
const STAGE_READ_00E8D3A0: u32 = 0x00e8_d3a0;
const STAGE_SET_00E88FC0: u32 = 0x00e8_8fc0;
const LIGHT_STAGE_00E8D3D0: u32 = 0x00e8_d3d0;
const SET_CLAMP_MODE_00E7DE90: u32 = 0x00e7_de90;
const SET_FILTER_MODE: u32 = 0x00e7_def0;

/// The light `00e8d890` takes from a list of the effect state (next +0, light +4): the light
/// that is switched on (+0x9c is 1) and has the given position among those, or 0.
fn nth_light_on(e: &Engine, list: u32, wanted: u32) -> u32 {
    let mut node = list;
    let mut seen = 0u32;
    while node != 0 {
        let light = e.mem.u32(node.wrapping_add(4));
        node = e.mem.u32(node);
        if light != 0 && e.mem.u8(light.wrapping_add(0x9c)) == 1 {
            let position = seen & 0xffff;
            seen += 1;
            if position == wanted {
                return light;
            }
        }
    }
    0
}

// Translated from 00e8d890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8d890`; the `NiDX9FragmentShader` unit by the RTTI of the
/// constructor's table): sets a texture stage's texture, clamp and filter modes up (`thiscall`
/// on the shader, three words: the stage, the texturing property and the dynamic effect
/// state). The texture, clamp and filter start at 0. With the stage's 16-bit +0x5c at 0 its
/// word +0xc picks a map of the property: bits 20..23 2 to 9 one of the eight maps at
/// `[property +0x20]` +0 to +0x1c, bit 24 a decal map (+0x20 + 4 * the low 16 bits, below the
/// property's 16-bit +0x26 minus 8), bit 25 a shader map (`00a463e0`, when the property has the
/// array +0x2c and the index is below its 16-bit +0xa); a map gives its texture (+8), clamp
/// (bits 12..13 of its 16-bit +4) and filter (bits 8..11), and with the top nibble 1 of the
/// word the stage state 0xb is set (`00e88fc0`) from the map's low byte, the upper half of
/// `00e7ddf0` and the byte of `00e8d3a0`. Otherwise its high byte 5 to 8 picks a light of the
/// effect state: +0x18 (5) or +0x1c (8) when switched on, or the switched-on light at the low
/// byte's position in the lists +0x14 (6) or +0x10 (7); a light gives its +0x124, +0x12c and
/// +0x128, and with the top nibble 1 runs `00e8d3d0`. Then the stage's +8 gets the texture and
/// the clamp (`00e7de90`) and filter (`SetFilterMode`) modes are set.
///
/// The byte of `00e8d3a0` in the map case is stored in the low byte of the property's stack
/// word, and that whole word is pushed: its upper bytes are the property address's (the
/// callee reads only the byte).
pub fn fn_00e8d890(e: &mut Engine, this: Ptr, stage: u32, property: u32, effects: u32) {
    let kind = e.mem.u16(stage.wrapping_add(0x5c));
    let word = e.mem.u32(stage.wrapping_add(0xc));
    let mut texture = 0u32;
    let mut clamp = 0u32;
    let mut filter = 0u32;
    if kind == 0 {
        if word != 0 {
            let maps = |e: &Engine| e.mem.u32(property.wrapping_add(0x20));
            let index = word & 0xffff;
            let map = match word & 0x0f00_0000 {
                0 => match word & 0x00f0_0000 {
                    0x0020_0000 => Some(e.mem.u32(maps(e))),
                    0x0030_0000 => Some(e.mem.u32(maps(e).wrapping_add(4))),
                    0x0040_0000 => Some(e.mem.u32(maps(e).wrapping_add(8))),
                    0x0050_0000 => Some(e.mem.u32(maps(e).wrapping_add(0xc))),
                    0x0060_0000 => Some(e.mem.u32(maps(e).wrapping_add(0x10))),
                    0x0070_0000 => Some(e.mem.u32(maps(e).wrapping_add(0x14))),
                    0x0080_0000 => Some(e.mem.u32(maps(e).wrapping_add(0x18))),
                    0x0090_0000 => Some(e.mem.u32(maps(e).wrapping_add(0x1c))),
                    _ => None,
                },
                0x0100_0000 => {
                    let decals = u32::from(e.mem.u16(property.wrapping_add(0x26))).wrapping_sub(8);
                    if index >= decals {
                        None
                    } else {
                        Some(e.mem.u32(maps(e).wrapping_add(index * 4 + 0x20)))
                    }
                }
                0x0200_0000 => {
                    let array = e.mem.u32(property.wrapping_add(0x2c));
                    if array == 0 || index >= u32::from(e.mem.u16(array.wrapping_add(0xa))) {
                        None
                    } else {
                        Some(e.call(SHADER_MAP_00A463E0, &args![property, index]).u32())
                    }
                }
                _ => None,
            };
            if let Some(map) = map {
                if map != 0 {
                    texture = e.mem.u32(map.wrapping_add(8));
                    let bits = u32::from(e.mem.u16(map.wrapping_add(4)));
                    clamp = (bits >> 12) & 3;
                    filter = (bits >> 8) & 0xf;
                    if word & 0xf000_0000 == 0x1000_0000 {
                        let low = u32::from(e.mem.u16(map.wrapping_add(4))) & 0xff;
                        let high = e.call(STAGE_READ_00E7DDF0, &args![stage]).u32();
                        let value = low | (high & 0xffff_0000);
                        let flag = e.call(STAGE_READ_00E8D3A0, &args![stage]).u8();
                        let target = e.mem.u32(stage.wrapping_add(0x10));
                        let pushed = (property & 0xffff_ff00) | u32::from(flag);
                        e.call(STAGE_SET_00E88FC0, &args![target, 0xbu32, value, pushed]);
                    }
                }
            }
        }
    } else if effects != 0 && (u32::from(kind) >> 8).wrapping_sub(5) <= 3 {
        let wanted = u32::from(kind) & 0xff;
        // The compiler's jump table at `00e8db7c` on the high byte minus 5.
        let light = match u32::from(kind) >> 8 {
            5 | 8 => {
                let offset = if kind >> 8 == 5 { 0x18 } else { 0x1c };
                let light = e.mem.u32(effects.wrapping_add(offset));
                if light != 0 && e.mem.u8(light.wrapping_add(0x9c)) != 0 {
                    light
                } else {
                    0
                }
            }
            6 => nth_light_on(e, e.mem.u32(effects.wrapping_add(0x14)), wanted),
            _ => nth_light_on(e, e.mem.u32(effects.wrapping_add(0x10)), wanted),
        };
        if light != 0 {
            texture = e.mem.u32(light.wrapping_add(0x124));
            clamp = e.mem.u32(light.wrapping_add(0x12c));
            filter = e.mem.u32(light.wrapping_add(0x128));
            if word & 0xf000_0000 == 0x1000_0000 {
                let flag = e.call(STAGE_READ_00E8D3A0, &args![stage]).u8();
                let extra = e.mem.u32(this.addr().wrapping_add(0x18));
                e.call(
                    LIGHT_STAGE_00E8D3D0,
                    &args![light, stage, u32::from(flag), extra],
                );
            }
        }
    }
    e.mem.set_u32(stage.wrapping_add(8), texture);
    e.call(SET_CLAMP_MODE_00E7DE90, &args![stage, clamp]);
    e.call(SET_FILTER_MODE, &args![stage, filter]);
}

/// The base constructor `00e818a0` (`thiscall`, no stack words), the class's two tables (RTTI
/// `NiDX9FragmentShader` at +0, `NiFragmentShaderInstanceDescriptor` at +0x3c) and the
/// member at +0x40's constructor `00ee89f0` (`thiscall`) and set-up `00ee8a10` (`thiscall`,
/// one word).
const BASE_CONSTRUCT_00E818A0: u32 = 0x00e8_18a0;
const FRAGMENT_SHADER_VTABLE: u32 = 0x010e_ff44;
const INSTANCE_DESCRIPTOR_VTABLE: u32 = 0x010e_fefc;
const MEMBER_CONSTRUCT_00EE89F0: u32 = 0x00ee_89f0;
const MEMBER_SET_00EE8A10: u32 = 0x00ee_8a10;

// Translated from 00e8db90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiDX9FragmentShader` constructor (the RTTI of its table),
/// `thiscall` with one word. The base constructor (`00e818a0`), the bytes +0x24 and +0x25 and
/// the words +0x28 to +0x38 cleared, its two tables (+0, +0x3c), then the member at +0x40
/// constructed (`00ee89f0`) and given the word (`00ee8a10`). Answers `this`.
pub fn fn_00e8db90(e: &mut Engine, this: Ptr, word: u32) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00E818A0, &args![base]);
    e.mem.set_u8(base.wrapping_add(0x24), 0);
    e.mem.set_u8(base.wrapping_add(0x25), 0);
    for offset in [0x28u32, 0x2c, 0x30, 0x34, 0x38] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u32(base, FRAGMENT_SHADER_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x3c), INSTANCE_DESCRIPTOR_VTABLE);
    e.call(MEMBER_CONSTRUCT_00EE89F0, &args![base.wrapping_add(0x40)]);
    e.call(MEMBER_SET_00EE8A10, &args![base.wrapping_add(0x40), word]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_d890, fn_00e8d890(Ptr, u32, u32, u32)),
        entry!(0x00e8_db90, fn_00e8db90(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            SET_CLAMP_MODE_00E7DE90,
            SET_FILTER_MODE,
            STAGE_SET_00E88FC0,
            LIGHT_STAGE_00E8D3D0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(STAGE_READ_00E7DDF0, |_, _| word(0x1234_5678));
        e.register(STAGE_READ_00E8D3A0, |_, _| word(0xabcd_ef01));
        e.call_log = Some(vec![]);
        e
    }

    fn stage(e: &mut Engine, kind: u16, word: u32) -> u32 {
        let stage = e.mem.alloc(0x60);
        e.mem.set_u16(stage + 0x5c, kind);
        e.mem.set_u32(stage + 0xc, word);
        e.mem.set_u32(stage + 0x10, 0x0e0f_0000);
        stage
    }

    fn map(e: &mut Engine, bits: u16, texture: u32) -> u32 {
        let map = e.mem.alloc(0xc);
        e.mem.set_u16(map + 4, bits);
        e.mem.set_u32(map + 8, texture);
        map
    }

    /// The (texture, clamp, filter) the stage ends with.
    fn result(e: &Engine, stage: u32) -> (u32, u32, u32) {
        (
            e.mem.u32(stage + 8),
            calls(e, SET_CLAMP_MODE_00E7DE90)[0][1],
            calls(e, SET_FILTER_MODE)[0][1],
        )
    }

    #[test]
    fn fn_00e8d890_takes_a_property_map_and_sets_the_stage_state() {
        let mut e = engine();
        let shader = e.mem.alloc(0x40);
        let property = e.mem.alloc(0x30);
        let maps = e.mem.alloc(0x40);
        e.mem.set_u32(property + 0x20, maps);
        let m = map(&mut e, 0x2a5c, 0x0e0e_0000);
        e.mem.set_u32(maps + 0xc, m);
        let s = stage(&mut e, 0, 0x1050_0000);
        e.call(0x00e8_d890, &args![shader, s, property, 0u32]);
        assert_eq!(result(&e, s), (0x0e0e_0000, 2, 0xa));
        assert_eq!(
            calls(&e, STAGE_SET_00E88FC0),
            vec![vec![
                0x0e0f_0000,
                0xb,
                0x1234_005c,
                (property & 0xffff_ff00) | 1
            ]]
        );
    }

    #[test]
    fn fn_00e8d890_decal_and_shader_maps_check_their_bounds() {
        let mut e = engine();
        let shader = e.mem.alloc(0x40);
        let property = e.mem.alloc(0x30);
        let maps = e.mem.alloc(0x40);
        e.mem.set_u32(property + 0x20, maps);
        e.mem.set_u16(property + 0x26, 10);
        let decal = map(&mut e, 0x1000, 0x0e0d_0000);
        e.mem.set_u32(maps + 0x20 + 4, decal);
        // Decal 1 of 2: taken (no state set without the top nibble).
        let s = stage(&mut e, 0, 0x0100_0001);
        e.call(0x00e8_d890, &args![shader, s, property, 0u32]);
        assert_eq!(result(&e, s), (0x0e0d_0000, 1, 0));
        assert!(calls(&e, STAGE_SET_00E88FC0).is_empty());
        // Decal 2 of 2: out of range, everything 0.
        e.call_log = Some(vec![]);
        let s = stage(&mut e, 0, 0x0100_0002);
        e.call(0x00e8_d890, &args![shader, s, property, 0u32]);
        assert_eq!(result(&e, s), (0, 0, 0));
        // A shader map: only with the array and an index below its count.
        e.call_log = Some(vec![]);
        let shader_map = map(&mut e, 0x0300, 0x0e0c_0000);
        e.register_double(SHADER_MAP_00A463E0, move |_, _| word(shader_map));
        let s = stage(&mut e, 0, 0x0200_0000);
        e.call(0x00e8_d890, &args![shader, s, property, 0u32]);
        assert!(calls(&e, SHADER_MAP_00A463E0).is_empty());
        let array = e.mem.alloc(0x10);
        e.mem.set_u16(array + 0xa, 1);
        e.mem.set_u32(property + 0x2c, array);
        e.call_log = Some(vec![]);
        e.call(0x00e8_d890, &args![shader, s, property, 0u32]);
        assert_eq!(calls(&e, SHADER_MAP_00A463E0), vec![vec![property, 0]]);
        assert_eq!(result(&e, s), (0x0e0c_0000, 0, 3));
    }

    fn light(e: &mut Engine, on: u8, texture: u32) -> u32 {
        let light = e.mem.alloc(0x130);
        e.mem.set_u8(light + 0x9c, on);
        e.mem.set_u32(light + 0x124, texture);
        e.mem.set_u32(light + 0x128, 5);
        e.mem.set_u32(light + 0x12c, 6);
        light
    }

    #[test]
    fn fn_00e8d890_takes_a_light_of_the_effect_state() {
        let mut e = engine();
        let shader = e.mem.alloc(0x40);
        e.mem.set_u32(shader + 0x18, 0x77);
        let effects = e.mem.alloc(0x20);
        let first = light(&mut e, 1, 0x0e01_0000);
        let off = light(&mut e, 0, 0x0e02_0000);
        let second = light(&mut e, 1, 0x0e03_0000);
        let mut list = 0;
        for l in [second, off, first] {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, list);
            e.mem.set_u32(node + 4, l);
            list = node;
        }
        e.mem.set_u32(effects + 0x10, list);
        e.mem.set_u32(effects + 0x18, off);
        // Kind 7, position 1: the second light switched on.
        let s = stage(&mut e, 0x0701, 0x1000_0000);
        e.call(0x00e8_d890, &args![shader, s, 0u32, effects]);
        assert_eq!(result(&e, s), (0x0e03_0000, 6, 5));
        assert_eq!(
            calls(&e, LIGHT_STAGE_00E8D3D0),
            vec![vec![second, s, 1, 0x77]]
        );
        // Kind 5: the light at +0x18 is off, nothing.
        e.call_log = Some(vec![]);
        let s = stage(&mut e, 0x0500, 0);
        e.call(0x00e8_d890, &args![shader, s, 0u32, effects]);
        assert_eq!(result(&e, s), (0, 0, 0));
        // Kind 9 is out of the table; no effect state: nothing either.
        e.call_log = Some(vec![]);
        let s = stage(&mut e, 0x0900, 0);
        e.call(0x00e8_d890, &args![shader, s, 0u32, effects]);
        assert_eq!(result(&e, s), (0, 0, 0));
    }

    #[test]
    fn fn_00e8db90_builds_the_fragment_shader() {
        let mut e = Engine::new();
        for addr in [
            BASE_CONSTRUCT_00E818A0,
            MEMBER_CONSTRUCT_00EE89F0,
            MEMBER_SET_00EE8A10,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x60);
        for offset in (0x24..0x3c).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_db90, &args![this, 0x42u32]).u32(), this);
        assert_eq!(e.mem.u32(this), FRAGMENT_SHADER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x3c), INSTANCE_DESCRIPTOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0x24), 0xffff_0000);
        for offset in (0x28..0x3c).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(calls(&e, BASE_CONSTRUCT_00E818A0), vec![vec![this]]);
        assert_eq!(
            calls(&e, MEMBER_SET_00EE8A10),
            vec![vec![this + 0x40, 0x42]]
        );
    }
}
