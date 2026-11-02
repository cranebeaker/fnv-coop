//! `BSShader/shadowlightshader_update_2x.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The four words the toggles start from (`011a9bd0`..`011a9bdc`); the
/// first, third and fourth are overwritten before they are stored.
const TOGGLE_DEFAULTS: u32 = 0x011a_9bd0;
/// The four floats the function stores (`011fa2d0`..`011fa2dc`).
const LAND_TOGGLES: u32 = 0x011f_a2d0;
/// The `double` 1.0 added to the count (`01012070`).
const ONE: u32 = 0x0101_2070;
/// The geometry's vtable slot answering a count (+0xEC, no words; 16 bits).
const GEOMETRY_SLOT_COUNT: u32 = 0xec;

// Translated from 00c02c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowLightShader::UpdateTogglesConstantLand` (Xbox PDB), thiscall
/// `(pass, flag, geometry, index)`; neither `this` nor the first word is
/// read. Stores four floats at `011fa2d0`: 1.0; the second word of the
/// defaults (`011a9bd4`), or 1.0 when `flag` is set; the byte at `index` of
/// the geometry's table at `+0xc4`, as a float; and the geometry's count
/// (vtable +0xEC, 16 bits) plus 1.0.
pub fn shadow_light_shader_update_toggles_constant_land(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    flag: u32,
    geometry: u32,
    index: u32,
) {
    let second = if flag != 0 {
        x87_store32(1.0)
    } else {
        e.global::<u32>(TOGGLE_DEFAULTS + 4)
    };
    let table = e.mem.u32(geometry.wrapping_add(0xc4));
    let byte = e.mem.u8(table.wrapping_add(index));
    let third = x87_store32(f64::from(byte));
    let count = e.vcall(geometry, GEOMETRY_SLOT_COUNT, &args![]).u16();
    e.set_global(LAND_TOGGLES + 4, second);
    e.set_global(LAND_TOGGLES, x87_store32(1.0));
    e.set_global(LAND_TOGGLES + 8, third);
    let one: f64 = e.global(ONE);
    let fourth = x87_store32(x87_add(f64::from(count), one));
    e.set_global(LAND_TOGGLES + 0xc, fourth);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c02c30,
        shadow_light_shader_update_toggles_constant_land(Ptr, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const COUNT_DOUBLE: u32 = 0xdead_7000;

    #[test]
    fn stores_the_land_toggles() {
        let mut e = Engine::new();
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_a000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ONE, 1.0f64);
        e.set_global(TOGGLE_DEFAULTS + 4, 0.25f32);
        let vtable = e.mem.alloc(0xf0);
        e.mem.set_u32(vtable + GEOMETRY_SLOT_COUNT, COUNT_DOUBLE);
        let table = e.mem.alloc(8);
        e.mem.set_u8(table + 3, 7);
        let geometry = e.mem.alloc(0xc8);
        e.mem.set_u32(geometry, vtable);
        e.mem.set_u32(geometry + 0xc4, table);
        e.register(COUNT_DOUBLE, |_, _| Ret {
            eax: 0x1_0004,
            ..Ret::default()
        });
        e.call(0x00c0_2c30, &args![0x100u32, 0x55u32, 0u32, geometry, 3u32]);
        let toggles: Vec<f32> = (0..4)
            .map(|word| e.mem.f32(LAND_TOGGLES + 4 * word))
            .collect();
        assert_eq!(toggles, vec![1.0, 0.25, 7.0, 5.0]);
        e.call(0x00c0_2c30, &args![0x100u32, 0x55u32, 1u32, geometry, 3u32]);
        assert_eq!(e.mem.f32(LAND_TOGGLES + 4), 1.0);
    }
}
