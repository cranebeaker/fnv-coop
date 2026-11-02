//! `BSShader/tallgrassshaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's allocation context (`+0x2b4` of the `_tls_index`
/// block), 0x1c while the render passes are built.
const TLS_CONTEXT: u32 = 0x2b4;
/// `BSShaderProperty::CheckCreateRenderPassArray` (engine map), thiscall (property, 2).
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
/// `BSShaderProperty::RenderPassArray::Add` (engine map), thiscall (array, eight words).
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// `BSShaderProperty::RenderPass::SetLights` (engine map), cdecl (pass, 1, light).
const RENDER_PASS_SET_LIGHTS: u32 = 0x00ba_8c50;
/// `BSShaderLightingProperty::GetNumberofActiveNonShadowLights` (engine map), thiscall: 16 bits.
const ACTIVE_NON_SHADOW_LIGHT_COUNT: u32 = 0x00b7_07d0;
/// `BSShaderLightingProperty::GetFirstActiveNonShadowLight` (engine map), thiscall (property, &iterator).
const FIRST_ACTIVE_NON_SHADOW_LIGHT: u32 = 0x00b7_0600;
/// `BSShaderLightingProperty::GetFirstActiveLight` (engine map), thiscall (property, &iterator).
const FIRST_ACTIVE_LIGHT: u32 = 0x00b7_0590;
/// `BSShaderLightingProperty::GetNextActiveLight` (engine map), thiscall (property, &iterator).
const NEXT_ACTIVE_LIGHT: u32 = 0x00b7_0680;
/// The shader settings the pass choice reads.
const LIGHTS_ENABLED: u32 = 0x011f_9ef0;
const SETTING_011F9427: u32 = 0x011f_9427;
const SETTING_011F95E4: u32 = 0x011f_95e4;
const SETTING_FLAGS_011F91D8: u32 = 0x011f_91d8;
const SHADER_LEVEL: u32 = 0x011f_91c0;
const SETTING_011FD890: u32 = 0x011f_d890;
const SETTING_011F941E: u32 = 0x011f_941e;

fn add_pass(e: &mut Engine, list: u32, words: [u32; 8]) {
    let [a, b, c, d, f, g, h, i] = words;
    e.call(RENDER_PASS_ARRAY_ADD, &args![list, a, b, c, d, f, g, h, i]);
}

/// The base pass's offset over the first part (`0x246` with the setting at
/// `011f941e`, plus 0x14 with lights): `wide` is bit 0x400 of the flags,
/// `long` bit 0x1000, `extra` bit 0x2000 and `special` the shadow test.
fn base_pass_offset(wide: bool, long: bool, extra: bool, special: bool) -> u32 {
    match (wide, long, extra, special) {
        (true, true, _, false) => 0x19,
        (true, true, _, true) => 0x21,
        (true, false, _, false) => 0x17,
        (true, false, _, true) => 0x1f,
        (false, true, true, _) => 0x27,
        (false, true, false, false) => 0x18,
        (false, true, false, true) => 0x20,
        (false, false, true, _) => 0x26,
        (false, false, false, false) => 0x16,
        (false, false, false, true) => 0x1e,
    }
}

// Translated from 00b60790 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b60790 - the C++ exception-unwinding frame (FS:[0]) that restores the allocation context when a call throws is not translated
/// `TallGrassShaderProperty::GetRenderPasses` (Xbox PDB), thiscall
/// `(geometry, key, third, high, fifth, sixth)` (the third, fifth and sixth
/// words are never read): with the allocation context at 0x1c (restored at
/// the end), answers the pass list (`+0x3c`) unchanged when the kept key
/// (`+0x38`) is `key`. Otherwise makes sure the list exists (2), empties
/// it, adds the base pass (by the flags at `+0x24`, the light count and the
/// shadow settings), gives it the first non-shadow light when there are
/// lights, adds a light pass (0x284 to 0x287) for each active light with
/// its byte at `+0xec` set when the shader level is 3 or more and bit 0x10
/// of `011f91d8` is set, and keeps `(high << 8) | key` at `+0x38`.
#[allow(clippy::too_many_arguments)]
pub fn tall_grass_shader_property_get_render_passes(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    key: u32,
    _unused_3: u32,
    high: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let base = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, 0x1c);
    if e.mem.u32(base + 0x38) != key {
        build_passes(e, base, geometry);
        e.mem.set_u32(base + 0x38, (high << 8) | key);
    }
    let list = e.mem.u32(base + 0x3c);
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
    list
}

/// The pass building of [`tall_grass_shader_property_get_render_passes`].
fn build_passes(e: &mut Engine, base: u32, geometry: u32) {
    e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![base, 2u32]);
    let list = e.mem.u32(base + 0x3c);
    e.mem.set_u32(list + 0x10, 0);
    e.with_stack(4, |e, iterator| {
        let lights = if e.global::<u8>(LIGHTS_ENABLED) != 0 {
            e.call(ACTIVE_NON_SHADOW_LIGHT_COUNT, &args![base]).u16()
        } else {
            0
        };
        let flags = e.mem.u32(base + 0x24);
        let wide = flags & 0x400 != 0;
        let long = flags & 0x1000 != 0;
        let extra = flags & 0x2000 != 0;
        let special = e.global::<u8>(SETTING_011F9427) == 0
            && e.global::<u32>(SETTING_011F95E4) != 0
            && e.global::<u8>(SETTING_FLAGS_011F91D8) & 0x20 != 0
            && e.global::<i32>(SHADER_LEVEL) >= 2
            && {
                let value: u32 = e.global(SETTING_011FD890);
                crate::x87::x87_fistp64_chop(X87Ext::load32(value)) as u32 != 0
            };
        let mut pass = if e.global::<u8>(SETTING_011F941E) != 0 {
            0x246
        } else {
            0
        };
        if lights > 0 {
            pass += 0x14;
        }
        pass += base_pass_offset(wide, long, extra, special);
        let list = e.mem.u32(base + 0x3c);
        add_pass(e, list, [geometry, pass, 1, 0, 0, 0, 0, 0]);
        if lights > 0 {
            let light = e
                .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                .u32();
            let list = e.mem.u32(base + 0x3c);
            let count = e.mem.u32(list + 0x10);
            let passes = e.mem.u32(list + 4);
            let last = e
                .mem
                .u32(passes.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4));
            e.call(RENDER_PASS_SET_LIGHTS, &args![last, 1u32, light]);
        }
        if e.global::<i32>(SHADER_LEVEL) >= 3 && e.global::<u8>(SETTING_FLAGS_011F91D8) & 0x10 != 0
        {
            let mut light = e.call(FIRST_ACTIVE_LIGHT, &args![base, iterator]).u32();
            while light != 0 {
                if e.mem.u8(light + 0xec) != 0 {
                    let list = e.mem.u32(base + 0x3c);
                    let light_pass = match (long, extra) {
                        (true, true) => 0x287,
                        (true, false) => 0x285,
                        (false, true) => 0x286,
                        (false, false) => 0x284,
                    };
                    add_pass(e, list, [geometry, light_pass, 0, 1, light, 0, 0, 0]);
                }
                light = e.call(NEXT_ACTIVE_LIGHT, &args![base, iterator]).u32();
            }
        }
    });
}

// Translated from 00bc2b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map's `TallGrassShaderProperty::SetClampNI`, a name the body does not confirm
/// (it may be folded code): thiscall, `RET 0xc` (three words, the last two unread), the first
/// word stored at +0xc.
pub fn fn_00bc2b20(e: &mut Engine, this: Ptr, value: u32, _unused_2: u32, _unused_3: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0xc), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b60790,
            tall_grass_shader_property_get_render_passes(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00bc_2b20, fn_00bc2b20(Ptr, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn base_pass_offsets_follow_the_flags() {
        assert_eq!(base_pass_offset(true, true, false, false), 0x19);
        assert_eq!(base_pass_offset(true, true, true, true), 0x21);
        assert_eq!(base_pass_offset(true, false, true, false), 0x17);
        assert_eq!(base_pass_offset(true, false, false, true), 0x1f);
        assert_eq!(base_pass_offset(false, true, true, true), 0x27);
        assert_eq!(base_pass_offset(false, true, false, false), 0x18);
        assert_eq!(base_pass_offset(false, true, false, true), 0x20);
        assert_eq!(base_pass_offset(false, false, true, false), 0x26);
        assert_eq!(base_pass_offset(false, false, false, false), 0x16);
        assert_eq!(base_pass_offset(false, false, false, true), 0x1e);
    }

    #[test]
    fn builds_the_base_and_light_passes_once_per_key() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x5000);
        e.set_global(LIGHTS_ENABLED, 1u8);
        e.set_global(SETTING_011F941E, 1u8);
        e.set_global(SHADER_LEVEL, 3u32);
        e.set_global(SETTING_FLAGS_011F91D8, 0x30u8);
        e.set_global(SETTING_011F95E4, 1u32);
        e.set_global(SETTING_011FD890, 1.5f32);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        e.register(CHECK_CREATE_RENDER_PASS_ARRAY, |_, _| Ret::default());
        e.register(ACTIVE_NON_SHADOW_LIGHT_COUNT, |_, _| Ret {
            eax: 0x1_0002,
            ..Ret::default()
        });
        let first = e.mem.alloc(0xf0);
        e.mem.set_u8(first + 0xec, 1);
        let second = e.mem.alloc(0xf0);
        e.register_double(FIRST_ACTIVE_NON_SHADOW_LIGHT, move |_, _| Ret {
            eax: first,
            ..Ret::default()
        });
        e.register_double(FIRST_ACTIVE_LIGHT, move |_, _| Ret {
            eax: first,
            ..Ret::default()
        });
        let mut handed_out = false;
        e.register_double(NEXT_ACTIVE_LIGHT, move |_, _| {
            let eax = if handed_out { 0 } else { second };
            handed_out = true;
            Ret {
                eax,
                ..Ret::default()
            }
        });
        e.register(RENDER_PASS_SET_LIGHTS, |_, _| Ret::default());
        // The list keeps the passes it is given.
        let passes = e.mem.alloc(0x40);
        let list = e.mem.alloc(0x14);
        e.mem.set_u32(list + 4, passes);
        e.register_double(RENDER_PASS_ARRAY_ADD, move |e, a| {
            let count = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(passes + 4 * count, 0x7000 + a[2]);
            e.mem.set_u32(a[0] + 0x10, count + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x3c, list);
        e.mem.set_u32(this + 0x24, 0x1400);
        e.mem.set_u32(this + 0x38, 0x99);
        let words = args![this, 0x5000u32, 3u32, 0u32, 2u32, 0u32, 0u32];
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_0790, &words).u32(), list);
        let added = calls(&e, RENDER_PASS_ARRAY_ADD);
        // 0x246 + 0x14 + 0x21 (wide, long, special).
        assert_eq!(added[0], vec![list, 0x5000, 0x27b, 1, 0, 0, 0, 0, 0]);
        assert_eq!(
            calls(&e, RENDER_PASS_SET_LIGHTS),
            vec![vec![0x727b, 1, first]]
        );
        assert_eq!(added[1], vec![list, 0x5000, 0x285, 0, 1, first, 0, 0, 0]);
        assert_eq!(added.len(), 2);
        assert_eq!(e.mem.u32(this + 0x38), 0x203);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
        // The same key again: nothing is rebuilt.
        e.call_log = Some(vec![]);
        let words = args![this, 0x5000u32, 0x203u32, 0u32, 2u32, 0u32, 0u32];
        assert_eq!(e.call(0x00b6_0790, &words).u32(), list);
        assert!(calls(&e, CHECK_CREATE_RENDER_PASS_ARRAY).is_empty());
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    #[test]
    fn set_clamp_stores_the_first_word_at_0xc() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.call(0x00bc_2b20, &args![this, 7u32, 8u32, 9u32]);
        assert_eq!(e.mem.u32(this + 0xc), 7);
        assert_eq!(e.mem.u32(this + 8), 0);
    }
}
