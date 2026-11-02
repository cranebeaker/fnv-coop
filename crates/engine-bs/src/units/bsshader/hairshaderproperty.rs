//! `BSShader/hairshaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's allocation context word (`+0x2b4`) and the context the hair passes are
/// built in (0x28).
const TLS_CONTEXT: u32 = 0x2b4;
const CONTEXT_HAIR: u32 = 0x28;
/// The calls `GetRenderPasses` makes: `BSShaderPPLightingProperty::GetRenderPasses`
/// (`00b67be0`, `thiscall`, six words), `AddLocalMapPass_2x` (`00bde170`, three words),
/// `BSShaderProperty::CheckCreateRenderPassArray` (`00ba95e0`, one word),
/// `RenderPassArray::Add` (`00ba9ee0`, eight words), the accumulator (`00b4f5c0`), the light
/// walk (`00b707d0` count, `00b70600` first and `00b70700` next active non-shadow light, the
/// last two with the address of an iterator word), a colour's luminance (`00b719e0`,
/// `thiscall` on the colour, the answer in ST0), a vector length (`00457990`, `thiscall`, ST0),
/// the larger of two `float`s (`00404010`, `cdecl`, ST0), the flag setter (`00441130`), the
/// depth pass test (`005bf7b0`, no words, a byte), `GetProperty` (`00a59d30`),
/// `AddDepthMapPass_2x` (`00bdf650`, four words), `AddTextureEffectPass_1x` (`00bd9d00`, five
/// words) and the default bound (`011f4288`).
const PP_GET_RENDER_PASSES: u32 = 0x00b6_7be0;
const ADD_LOCAL_MAP_PASS_2X: u32 = 0x00bd_e170;
const CHECK_CREATE_RENDER_PASSES: u32 = 0x00ba_95e0;
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const ACTIVE_LIGHT_COUNT: u32 = 0x00b7_07d0;
const FIRST_LIGHT: u32 = 0x00b7_0600;
const NEXT_LIGHT: u32 = 0x00b7_0700;
const COLOR_LUMINANCE_00B719E0: u32 = 0x00b7_19e0;
const VECTOR_LENGTH_00457990: u32 = 0x0045_7990;
const FLOAT_MAX_00404010: u32 = 0x0040_4010;
const SET_FLAG_00441130: u32 = 0x0044_1130;
const DEPTH_PASSES_WANTED_005BF7B0: u32 = 0x005b_f7b0;
const GET_PROPERTY: u32 = 0x00a5_9d30;
const ADD_DEPTH_MAP_PASS_2X: u32 = 0x00bd_f650;
const ADD_TEXTURE_EFFECT_PASS_1X: u32 = 0x00bd_9d00;
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;

/// One of the light bits of the second flag word (`+0x24`) set: `iLastRenderPassState`
/// (`+0x38`) cleared first when it was clear.
fn set_light_bit(e: &mut Engine, this: u32, bit: u32) {
    if e.mem.u32(this + 0x24) & bit == 0 {
        e.mem.set_u32(this + 0x38, 0);
    }
    let flags = e.mem.u32(this + 0x24);
    e.mem.set_u32(this + 0x24, flags | bit);
}

/// One of the light bits cleared: `iLastRenderPassState` cleared first when it was set.
fn clear_light_bit(e: &mut Engine, this: u32, bit: u32) {
    if e.mem.u32(this + 0x24) & bit != 0 {
        e.mem.set_u32(this + 0x38, 0);
    }
    let flags = e.mem.u32(this + 0x24);
    e.mem.set_u32(this + 0x24, flags & !bit);
}

/// The luminance (`00b719e0`) of a light's colour (`+0xd4` to `+0xdc` of its `+0xf8`
/// object, copied into a local first).
fn light_luminance(e: &mut Engine, light: u32) -> f64 {
    let data = e.mem.u32(light + 0xf8);
    e.with_stack(0xc, |e, color| {
        for k in 0..3u32 {
            let v = e.mem.u32(data + 0xd4 + 4 * k);
            e.mem.set_u32(color.addr() + 4 * k, v);
        }
        e.call(COLOR_LUMINANCE_00B719E0, &args![color]).f64()
    })
}

// Translated from 00bc3e40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bc3e40 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `HairShaderProperty::GetRenderPasses` (engine map; the body matches): `thiscall`, `RET 0x18`
/// (geometry, a flag word, the pass counter, a mode, the accumulator, the pass count), in the
/// thread's allocation context 0x28. Refraction (bits 15 or 16 of the first flag word) goes to
/// `BSShaderPPLightingProperty::GetRenderPasses`; modes 7 and 8 answer the lists at `+0x40`
/// and `+0x44`; mode 9 adds the local map pass (`00bde170`) and answers `+0x48`. Otherwise,
/// unless the passes for this mode and flag word (`mode << 8 | flags`, kept at `+0x38`) are
/// already made, the list is made or emptied (pass count 1; the counter cleared otherwise), and
/// the hair pass is chosen by the lights: with no active non-shadow light pass `0xc1` with the
/// sun (light bits 7 to 9 cleared); else pass `0xc2` with three lights picked by comparing the
/// first light's luminance attenuated by its distance to the geometry's bound (`00457990`,
/// `00404010`) with the sun's, setting light bits 7 to 9 to match. Then the depth pass
/// (`00bdf650`) when `005bf7b0` asks for it and the pass count is not 0, the texture effect pass
/// with texture effect data (`+0xdc`), and the render-pass list (`+0x3c`) is the answer.
#[allow(clippy::too_many_arguments)]
pub fn hair_shader_property_get_render_passes(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    pass_flags: u32,
    counter: u32,
    mode: u32,
    accumulator: u32,
    pass_count: u32,
) -> u32 {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, CONTEXT_HAIR);
    let result = render_passes(
        e,
        this,
        geometry,
        pass_flags,
        counter,
        mode,
        accumulator,
        pass_count,
    );
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
    result
}

/// The body of [`hair_shader_property_get_render_passes`] between setting and restoring the
/// allocation context.
#[allow(clippy::too_many_arguments)]
fn render_passes(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    pass_flags: u32,
    counter: u32,
    mode: u32,
    accumulator: u32,
    pass_count: u32,
) -> u32 {
    let base = this.addr();
    let flags0 = e.mem.u32(base + 0x20);
    if flags0 & 0x8000 != 0 || flags0 & 0x1_0000 != 0 {
        return e
            .call(
                PP_GET_RENDER_PASSES,
                &args![
                    this,
                    geometry,
                    pass_flags,
                    counter,
                    mode,
                    accumulator,
                    pass_count
                ],
            )
            .u32();
    }
    let key = mode << 8 | pass_flags;
    if mode == 7 {
        return e.mem.u32(base + 0x40);
    }
    if mode == 8 {
        return e.mem.u32(base + 0x44);
    }
    if mode == 9 {
        let skinned = u32::from(e.mem.u32(geometry.wrapping_add(0xbc)) != 0);
        e.call(ADD_LOCAL_MAP_PASS_2X, &args![this, geometry, skinned, 0u32]);
        return e.mem.u32(base + 0x48);
    }
    let count = pass_count as u8;
    let made = e.mem.u32(base + 0x38) == key && count != 0;
    if !made {
        if count == 1 {
            e.call(CHECK_CREATE_RENDER_PASSES, &args![this, 0xffff_ffffu32]);
            let list = e.mem.u32(base + 0x3c);
            e.mem.set_u32(list + 0x10, 0);
            e.mem.set_u32(base + 0x38, key);
        } else {
            e.mem.set_u16(counter, 0);
        }
        let shader_accumulator = e.call(GET_ACCUMULATOR, &args![]).u32();
        let sun = e.mem.u32(
            e.mem
                .u32(shader_accumulator.wrapping_add(0x194))
                .wrapping_add(0xe0),
        );
        let lights = e.call(ACTIVE_LIGHT_COUNT, &args![this]).u32() as u16;
        if lights == 0 {
            if count == 1 {
                let list = e.mem.u32(base + 0x3c);
                e.call(
                    RENDER_PASS_ARRAY_ADD,
                    &args![list, geometry, 0xc1u32, 1u32, 1u32, sun, 0u32, 0u32, 0u32],
                );
                clear_light_bit(e, base, 0x80);
                clear_light_bit(e, base, 0x100);
                clear_light_bit(e, base, 0x200);
            } else {
                let n = e.mem.u16(counter);
                e.mem.set_u16(counter, n.wrapping_add(1));
            }
        } else {
            let (first, second, third) = e.with_stack(4, |e, iterator| {
                let iterator = iterator.addr();
                e.mem.set_u32(iterator, 0);
                let light = e.call(FIRST_LIGHT, &args![this, iterator]).u32();
                let sun_luminance = x87_store32(light_luminance(e, sun));
                let mut bound = e.mem.u32(geometry.wrapping_add(0x20));
                if bound == 0 {
                    bound = DEFAULT_BOUND_011F4288;
                }
                let data = e.mem.u32(light + 0xf8);
                let distance = e.with_stack(0xc, |e, offset| {
                    for k in 0..3u32 {
                        let d = x87_sub(
                            x87_m32(e.mem.u32(data + 0x8c + 4 * k)),
                            x87_m32(e.mem.u32(bound + 4 * k)),
                        );
                        e.mem.set_u32(offset.addr() + 4 * k, x87_store32(d));
                    }
                    let length = e.call(VECTOR_LENGTH_00457990, &args![offset]).f64();
                    x87_store32(x87_sub(length, x87_m32(e.mem.u32(bound + 0xc))))
                });
                let larger = e
                    .call(
                        FLOAT_MAX_00404010,
                        &args![x87_store32(0.0), x87_quiet_bits(distance)],
                    )
                    .f64();
                let distance = x87_store32(larger);
                let luminance = light_luminance(e, light);
                let d = x87_load32(distance);
                let inverse = x87_div(1.0, d);
                let attenuated = x87_store32(x87_mul(luminance, x87_mul(d, inverse)));
                let sun_luminance_64 = x87_load32(sun_luminance);
                if x87_load32(attenuated) < sun_luminance_64 {
                    clear_light_bit(e, base, 0x80);
                    clear_light_bit(e, base, 0x100);
                    clear_light_bit(e, base, 0x200);
                    set_light_bit(e, base, 0x100);
                    let next = e.call(NEXT_LIGHT, &args![this, iterator]).u32();
                    if next != 0 {
                        set_light_bit(e, base, 0x200);
                    }
                    (sun, light, next)
                } else {
                    set_light_bit(e, base, 0x80);
                    if 0.0 < x87_m32(sun_luminance) {
                        let next = e.call(NEXT_LIGHT, &args![this, iterator]).u32();
                        if next != 0 {
                            set_light_bit(e, base, 0x100);
                            set_light_bit(e, base, 0x200);
                            (light, sun, next)
                        } else {
                            set_light_bit(e, base, 0x100);
                            clear_light_bit(e, base, 0x200);
                            (light, sun, 0)
                        }
                    } else {
                        let next = e.call(NEXT_LIGHT, &args![this, iterator]).u32();
                        if next != 0 {
                            let third = e.call(NEXT_LIGHT, &args![this, iterator]).u32();
                            set_light_bit(e, base, 0x100);
                            e.call(
                                SET_FLAG_00441130,
                                &args![this, 0x29u32, u32::from(third != 0)],
                            );
                            (light, next, third)
                        } else {
                            clear_light_bit(e, base, 0x100);
                            clear_light_bit(e, base, 0x200);
                            (light, 0, 0)
                        }
                    }
                }
            });
            if count == 1 {
                let list = e.mem.u32(base + 0x3c);
                e.call(
                    RENDER_PASS_ARRAY_ADD,
                    &args![list, geometry, 0xc2u32, 1u32, 3u32, first, second, third, 0u32],
                );
            } else {
                let n = e.mem.u16(counter);
                e.mem.set_u16(counter, n.wrapping_add(1));
            }
        }
        if e.call(DEPTH_PASSES_WANTED_005BF7B0, &args![]).u8() != 0 && count != 0 {
            let mut alpha = e.call(GET_PROPERTY, &args![geometry, 0u32]).u32();
            if alpha == 0 {
                alpha = e.mem.u32(geometry.wrapping_add(0x9c));
            }
            let tested = alpha != 0 && (e.mem.u16(alpha + 0x18) >> 9) & 1 != 0;
            // The byte is stored over the low byte of the geometry argument's stack word.
            let word = (geometry & 0xffff_ff00) | u32::from(tested);
            let skinned = u32::from(e.mem.u32(geometry.wrapping_add(0xbc)) != 0);
            e.call(
                ADD_DEPTH_MAP_PASS_2X,
                &args![this, geometry, skinned, word, accumulator],
            );
        }
    }
    if e.mem.u32(base + 0xdc) != 0 {
        e.with_stack(4, |e, first| {
            e.mem.set_u32(first.addr(), (geometry & 0xffff_ff00) | 1);
            e.call(
                ADD_TEXTURE_EFFECT_PASS_1X,
                &args![this, geometry, counter, pass_count, first, 0u32],
            );
        });
    }
    e.mem.u32(base + 0x3c)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bc_3e40,
        hair_shader_property_get_render_passes(Ptr, u32, u32, u32, u32, u32, u32) -> u32
    )]
}

#[cfg(test)]
mod n012_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A hair property with a pass list, a sun and the doubles of the pass making; the light
    /// luminance double answers the first colour channel, the length double the first vector
    /// component, the larger-of-two double its second `float`.
    fn rig(lights: u32) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        let this = e.mem.alloc(0x100);
        let list = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x3c, list);
        let sun = e.mem.alloc(0x100);
        let sun_data = e.mem.alloc(0x100);
        e.mem.set_u32(sun + 0xf8, sun_data);
        let accumulator = e.mem.alloc(0x200);
        let holder = e.mem.alloc(0x100);
        e.mem.set_u32(holder + 0xe0, sun);
        e.mem.set_u32(accumulator + 0x194, holder);
        e.register_double(GET_ACCUMULATOR, move |_, _| accumulator.into_ret());
        e.register_double(ACTIVE_LIGHT_COUNT, move |_, _| lights.into_ret());
        e.register(CHECK_CREATE_RENDER_PASSES, |_, _| Ret::default());
        e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
        e.register(COLOR_LUMINANCE_00B719E0, |e, a| {
            f64::from(e.mem.f32(a[0])).into_ret()
        });
        e.register(VECTOR_LENGTH_00457990, |e, a| {
            f64::from(e.mem.f32(a[0])).into_ret()
        });
        e.register(FLOAT_MAX_00404010, |_, a| {
            f64::from(f32::from_bits(a[1])).into_ret()
        });
        e.register(DEPTH_PASSES_WANTED_005BF7B0, |_, _| Ret::default());
        e.register(SET_FLAG_00441130, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, this, sun, sun_data)
    }

    fn light(e: &mut Engine, luminance: f32, x: f32) -> u32 {
        let light = e.mem.alloc(0x100);
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(light + 0xf8, data);
        e.mem.set_f32(data + 0xd4, luminance);
        e.mem.set_f32(data + 0x8c, x);
        light
    }

    #[test]
    fn refraction_and_fixed_modes_answer_other_lists() {
        let (mut e, this, _, _) = rig(0);
        e.register(PP_GET_RENDER_PASSES, |_, _| 0x4321u32.into_ret());
        e.register(ADD_LOCAL_MAP_PASS_2X, |_, _| Ret::default());
        e.mem.set_u32(this + 0x40, 0x40);
        e.mem.set_u32(this + 0x44, 0x44);
        e.mem.set_u32(this + 0x48, 0x48);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry + 0xbc, 1);
        let call = |e: &mut Engine, mode: u32| {
            e.call(
                0x00bc_3e40,
                &args![this, geometry, 3u32, 0x900u32, mode, 0x500u32, 1u32],
            )
            .u32()
        };
        assert_eq!(call(&mut e, 7), 0x40);
        assert_eq!(call(&mut e, 8), 0x44);
        assert_eq!(call(&mut e, 9), 0x48);
        assert_eq!(
            calls(&e, ADD_LOCAL_MAP_PASS_2X),
            vec![vec![this, geometry, 1, 0]]
        );
        e.mem.set_u32(this + 0x20, 0x8000);
        assert_eq!(call(&mut e, 0), 0x4321);
        assert_eq!(
            calls(&e, PP_GET_RENDER_PASSES),
            vec![vec![this, geometry, 3, 0x900, 0, 0x500, 1]]
        );
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 0);
    }

    #[test]
    fn without_lights_the_sun_pass_and_bits_cleared() {
        let (mut e, this, sun, _) = rig(0);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0x24, 0x380);
        e.mem.set_u32(this + 0x38, 5);
        let list = e
            .call(
                0x00bc_3e40,
                &args![this, geometry, 2u32, 0u32, 1u32, 0u32, 1u32],
            )
            .u32();
        assert_eq!(list, e.mem.u32(this + 0x3c));
        assert_eq!(
            calls(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![list, geometry, 0xc1, 1, 1, sun, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        // The same key with a pass count: already made, nothing added.
        e.mem.set_u32(this + 0x38, 0x102);
        e.call(
            0x00bc_3e40,
            &args![this, geometry, 2u32, 0u32, 1u32, 0u32, 1u32],
        );
        assert_eq!(calls(&e, RENDER_PASS_ARRAY_ADD).len(), 1);
        // Counting only: the counter cleared, then one counted.
        let counter = e.mem.alloc(4);
        e.mem.set_u16(counter, 7);
        e.call(
            0x00bc_3e40,
            &args![this, geometry, 2u32, counter, 1u32, 0u32, 0u32],
        );
        assert_eq!(e.mem.u16(counter), 1);
    }

    #[test]
    fn a_dim_light_goes_after_the_sun() {
        let (mut e, this, sun, sun_data) = rig(2);
        e.mem.set_f32(sun_data + 0xd4, 4.0);
        let first = light(&mut e, 1.0, 3.0);
        let second = light(&mut e, 1.0, 0.0);
        e.register_double(FIRST_LIGHT, move |_, _| first.into_ret());
        let mut handed = vec![second, 0];
        e.register_double(NEXT_LIGHT, move |_, _| handed.remove(0).into_ret());
        let geometry = e.mem.alloc(0xc0);
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(geometry + 0x20, bound);
        e.call(
            0x00bc_3e40,
            &args![this, geometry, 2u32, 0u32, 1u32, 0u32, 1u32],
        );
        let list = e.mem.u32(this + 0x3c);
        assert_eq!(
            calls(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![list, geometry, 0xc2, 1, 3, sun, first, second, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x24), 0x300);
    }

    #[test]
    fn a_bright_light_leads_and_the_texture_effect_pass_follows() {
        let (mut e, this, sun, sun_data) = rig(1);
        e.mem.set_f32(sun_data + 0xd4, 0.5);
        let first = light(&mut e, 2.0, 1.0);
        e.register_double(FIRST_LIGHT, move |_, _| first.into_ret());
        e.register(NEXT_LIGHT, |_, _| Ret::default());
        e.register(ADD_TEXTURE_EFFECT_PASS_1X, |e, a| {
            assert_eq!(e.mem.u8(a[4]), 1);
            Ret::default()
        });
        e.mem.set_u32(this + 0xdc, 0x7777);
        e.mem.set_u32(this + 0x24, 0x200);
        let geometry = e.mem.alloc(0xc0);
        let counter = e.mem.alloc(4);
        e.call(
            0x00bc_3e40,
            &args![this, geometry, 2u32, counter, 1u32, 0u32, 1u32],
        );
        let list = e.mem.u32(this + 0x3c);
        assert_eq!(
            calls(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![list, geometry, 0xc2, 1, 3, first, sun, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x24), 0x180);
        let effect = calls(&e, ADD_TEXTURE_EFFECT_PASS_1X);
        assert_eq!(effect[0][..4], [this, geometry, counter, 1]);
    }
}
