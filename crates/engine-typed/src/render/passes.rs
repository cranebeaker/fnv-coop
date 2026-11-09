//! The lighting shader property's render passes (`BSShaderPPLightingProperty::GetRenderPasses`), what
//! the accumulator asks each visible geometry's property for, with the small getters it calls: the
//! shader manager's accumulator, a property flag, the camera's position.
//!
//! Floats: the camera distance and the faded alpha are computed with the exact x87 helpers, every
//! load, operation and store the exe's, under either control word.

use super::FAMILY;
use crate::shader::{accumulator, layout::BSShaderProperty};
use engine_core::ptr::Ptr;
use engine_core::x87::{x87_load32, x87_m32, x87_mul, x87_quiet_bits, x87_store32, x87_sub};
use engine_core::{args, typed_call, Engine};

/// The shader manager's accumulator (`011f95ec`).
const ACCUMULATOR: u32 = 0x011f_95ec;
/// `00a59d30` (`thiscall(geometry, 0)`): the geometry's property of a kind.
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// The texture renderer (`011f4748`).
const RENDERER: u32 = 0x011f_4748;
/// The pass arrays' makers and the passes' adders for the special pass types.
const CHECK_CREATE_DEPTH_MAP_PASSES: u32 = 0x00ba_9660;
const ADD_DEPTH_MAP_PASS: u32 = 0x00bd_f650;
const CHECK_CREATE_CONST_ALPHA_PASSES: u32 = 0x00ba_9740;
const ADD_CONST_ALPHA_PASS: u32 = 0x00bd_a060;
const CHECK_CREATE_LOCAL_MAP_PASSES: u32 = 0x00ba_96d0;
const ADD_LOCAL_MAP_PASS: u32 = 0x00bd_e170;
const CHECK_CREATE_SILHOUETTE_PASSES: u32 = 0x00ba_9820;
const ADD_SILHOUETTE_PASS: u32 = 0x00bd_f6c0;
/// The fade distances (`float`s).
const FADE_FAR_9458: u32 = 0x011f_9458;
const FADE_FAR_9460: u32 = 0x011f_9460;
const FADE_FAR_9468: u32 = 0x011f_9468;
const FADE_NEAR_946C: u32 = 0x011f_946c;
const FADE_FAR_9470: u32 = 0x011f_9470;
/// The bound used for a geometry without one (`011f4288`).
const DEFAULT_BOUND: u32 = 0x011f_4288;
/// `004b3ab0` (`cdecl`, five words; the answer in ST0): the fade interpolation.
const INTERPOLATE: u32 = 0x004b_3ab0;
/// `00b70390` (`thiscall(property, bound)`): the lights resorted.
const RESORT_LIGHTS: u32 = 0x00b7_0390;
/// The shader level (`011f91c0`, signed).
const SHADER_LEVEL: u32 = 0x011f_91c0;
/// `005bf7b0` (no arguments, a bool): whether depth passes are wanted.
const DEPTH_PASSES_WANTED: u32 = 0x005b_f7b0;

/// The low byte of a word (a `bool` the exe passes or answers as a word).
#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
fn low8(word: u32) -> u8 {
    word as u8
}

// Typed from 00b4f5c0
/// `BSShaderManager::GetAccumulator` (Xbox PDB): the word at `011f95ec`.
pub fn shader_accumulator(e: &mut Engine) -> u32 {
    e.global::<u32>(ACCUMULATOR)
}

// Typed from 004a2020
/// `thiscall(this, bit)`: whether bit `bit` of the bit set whose words start at `this + 0x20` is set.
pub fn has_flag(e: &mut Engine, this: u32, bit: u32) -> bool {
    let word = e.mem.u32(
        this.wrapping_add(0x20)
            .wrapping_add((bit >> 5).wrapping_mul(4)),
    );
    word & (1u32 << (bit % 32)) != 0
}

// Typed from 005bb9b0
/// `thiscall(this, out)`: the three words at `this + 0x1f0` copied to `out`; answers `out`.
pub fn camera_position(e: &mut Engine, this: u32, out: u32) -> u32 {
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(this.wrapping_add(0x1f0).wrapping_add(offset));
        e.mem.set_u32(out.wrapping_add(offset), word);
    }
    out
}

// Typed from 00b66760
/// Whether a pass number is one of the two the passes lower by one (0xfc, 0x102).
pub fn lowered_pass_number(_e: &mut Engine, value: u32) -> u32 {
    u32::from(value == 0xfc || value == 0x102)
}

/// [`has_flag`] by `typed_call!`.
#[inline(always)]
fn flag(e: &mut Engine, this: u32, bit: u32) -> bool {
    typed_call!(FAMILY, 0x004a_2020, has_flag(e, this, bit))
}

/// The flag word with its low byte set to bit 9 of the 16-bit `+0x18` of the geometry's property
/// (`00a59d30(geometry, 0)`, else the geometry's `+0x9c` word), 0 without one.
fn depth_flag(e: &mut Engine, geometry: u32, flag_word: u32) -> u32 {
    let mut property = e.call(GET_PROPERTY, &args![geometry, 0u32]).u32();
    if property == 0 {
        property = e.mem.u32(geometry.wrapping_add(0x9c));
    }
    let set = property != 0 && (e.mem.u16(property.wrapping_add(0x18)) >> 9) & 1 != 0;
    (flag_word & 0xffff_ff00) | u32::from(set)
}

/// The first pass of a pass array (`+4`, count `+0x10`) gets the renderer's `+8` object's word
/// `+0x1a0` at its `+0xc` target when the pass's byte `+9` is set.
fn refresh_first_pass(e: &mut Engine, array: u32) {
    let pass = e.mem.u32(e.mem.u32(array.wrapping_add(4)));
    if e.mem.u8(pass.wrapping_add(9)) != 0 {
        let renderer = e.global::<u32>(RENDERER);
        let value = e
            .mem
            .u32(e.mem.u32(renderer.wrapping_add(8)).wrapping_add(0x1a0));
        let target = e.mem.u32(pass.wrapping_add(0xc));
        e.mem.set_u32(target, value);
    }
}

/// The state `+0x38` cleared when the camera distance `+0x34` crossed `limit` since the last call
/// (every compare ordered).
fn clear_on_crossing(e: &mut Engine, this: u32, limit: f32, previous: f32) {
    let current = e.mem.f32(this.wrapping_add(0x34));
    if limit > current {
        if limit <= previous {
            e.mem.set_u32(this.wrapping_add(0x38), 0);
        }
        return;
    }
    if current >= limit && limit > previous {
        e.mem.set_u32(this.wrapping_add(0x38), 0);
    }
}

// Typed from 00b67be0
/// `BSShaderPPLightingProperty::GetRenderPasses` (Xbox PDB), `thiscall` with six words: the geometry,
/// a pass word, the address of a 16-bit count, the pass type, the accumulator and a flag word (its low
/// byte). Answers a render pass array.
///
/// Types 7 (depth map, `+0x40`), 8 (constant alpha, `+0x44`), 9 (local map, `+0x48`) and 0xf (`+0x54`)
/// make their array and add their pass. Any other type rebuilds the main array (`+0x3c`, key
/// `type << 8 | word`): with a fade on and the accumulator's bytes `+0x164`/`+0x165` clear, the camera
/// distance `+0x34` is measured again (the bound centre's distance to the camera less its radius,
/// scaled by the accumulator shader's `+0x110`) and the state `+0x38` cleared when a fade distance was
/// crossed; the alpha is set (with the material's alpha and the `011f946c`/`011f9470` fade, `004b3ab0`);
/// then unless the state matches the key, the lights are resorted when needed and the passes added
/// through slot 0x114 (level 1) or 0x118 (2 or more), a depth pass when `005bf7b0` asks, the last pass
/// marked, and one 0xfc/0x102 pass number lowered by one.
#[allow(clippy::too_many_arguments)]
pub fn render_passes(
    e: &mut Engine,
    this: u32,
    geometry: u32,
    pass_word: u32,
    count_out: u32,
    pass_type: u32,
    accumulator: u32,
    flag_word: u32,
) -> u32 {
    let base = this;
    let mut flag_word = flag_word;
    let properties = geometry.wrapping_add(0x9c);
    let measured_elsewhere = e.mem.u8(accumulator.wrapping_add(0x165)) != 0
        || e.mem.u8(accumulator.wrapping_add(0x164)) != 0;
    let key = (pass_type << 8) | pass_word;
    let property = Ptr::<BSShaderProperty>::new(base);
    if pass_type == 7 {
        let flags = e.mem.u32(base.wrapping_add(0x20));
        if flags & 0x8000 != 0 || flags & 0x0001_0000 != 0 {
            return 0;
        }
        e.call(CHECK_CREATE_DEPTH_MAP_PASSES, &args![base, 1u32]);
        let array = e.mem.u32(base.wrapping_add(0x40));
        if e.mem.u32(array.wrapping_add(0x10)) != 0 {
            refresh_first_pass(e, array);
        } else if low8(flag_word) != 0 && properties != 0 {
            flag_word = depth_flag(e, geometry, flag_word);
            let has_skin = u32::from(e.mem.u32(geometry.wrapping_add(0xbc)) != 0);
            e.call(
                ADD_DEPTH_MAP_PASS,
                &args![base, geometry, has_skin, flag_word, accumulator],
            );
        }
        return e.mem.u32(base.wrapping_add(0x40));
    }
    if pass_type == 8 {
        let flags = e.mem.u32(base.wrapping_add(0x20));
        if flags & 0x8000 == 0
            && flags & 0x0001_0000 == 0
            && e.mem.u32(base.wrapping_add(0x24)) & 0x10 == 0
        {
            e.call(CHECK_CREATE_CONST_ALPHA_PASSES, &args![base, 1u32]);
            let array = e.mem.u32(base.wrapping_add(0x44));
            if e.mem.u32(array.wrapping_add(0x10)) != 0 {
                refresh_first_pass(e, array);
            }
            let array = e.mem.u32(base.wrapping_add(0x44));
            if e.mem.u32(array.wrapping_add(0x10)) == 0 && low8(flag_word) != 0 && properties != 0 {
                flag_word = depth_flag(e, geometry, flag_word);
                let has_skin = u32::from(e.mem.u32(geometry.wrapping_add(0xbc)) != 0);
                e.call(
                    ADD_CONST_ALPHA_PASS,
                    &args![base, geometry, has_skin, flag_word],
                );
            }
        }
        return e.mem.u32(base.wrapping_add(0x44));
    }
    if pass_type == 9 {
        e.call(CHECK_CREATE_LOCAL_MAP_PASSES, &args![base, 1u32]);
        let flags = e.mem.u32(base.wrapping_add(0x20));
        let two = u32::from(flags & 2 != 0);
        let empty = u32::from(flags & 0x0010_0000 != 0);
        e.call(ADD_LOCAL_MAP_PASS, &args![base, geometry, two, empty]);
        return e.mem.u32(base.wrapping_add(0x48));
    }
    if pass_type == 0xf {
        e.call(CHECK_CREATE_SILHOUETTE_PASSES, &args![base, 1u32]);
        let array = e.mem.u32(base.wrapping_add(0x54));
        if e.mem.u32(array.wrapping_add(0x10)) == 0 {
            let has_skin = u32::from(e.mem.u32(geometry.wrapping_add(0xbc)) != 0);
            e.call(ADD_SILHOUETTE_PASS, &args![base, geometry, has_skin]);
        }
        return e.mem.u32(base.wrapping_add(0x54));
    }
    typed_call!(
        crate::shader::FAMILY,
        0x00ba_95e0,
        accumulator::check_create_pass_array(e, property, -1i32)
    );
    let previous = f32::from_bits(x87_quiet_bits(e.mem.u32(base.wrapping_add(0x34))));
    let flags = e.mem.u32(base.wrapping_add(0x20));
    let second_fade = flags & 0x80 != 0 && 0.0 < e.global::<f32>(FADE_FAR_9460);
    let first_fade = flags & 0x0002_0000 != 0 && 0.0 < e.global::<f32>(FADE_FAR_9468);
    let main_fade = flags & 1 != 0 && 0.0 < e.global::<f32>(FADE_FAR_9458);
    let lod_on = flags & 0x0400_0000 != 0 || flag(e, base, 0x1b);
    let lod_fade = lod_on && 0.0 < e.global::<f32>(FADE_FAR_9470) && !flag(e, base, 0x23);
    let mut lod_fade_kept = lod_fade;
    let mut bound = e.mem.u32(geometry.wrapping_add(0x20));
    if bound == 0 {
        bound = DEFAULT_BOUND;
    }
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        // The bound copy (centre and radius) at `frame + 0x20`, the camera position at `+0`, the
        // difference at `+0xc`.
        for word in 0..4u32 {
            let value = e.mem.u32(bound.wrapping_add(word * 4));
            e.mem.set_u32(frame.wrapping_add(0x20 + word * 4), value);
        }
        if (first_fade || second_fade || main_fade || lod_fade) && !measured_elsewhere {
            let camera = e.mem.u32(accumulator.wrapping_add(0x194));
            typed_call!(FAMILY, 0x005b_b9b0, camera_position(e, camera, frame));
            for axis in 0..3u32 {
                let centre = e.mem.u32(frame.wrapping_add(0x20 + axis * 4));
                let eye = e.mem.u32(frame.wrapping_add(axis * 4));
                let difference = x87_store32(x87_sub(x87_load32(centre), x87_m32(eye)));
                e.mem
                    .set_u32(frame.wrapping_add(0xc + axis * 4), difference);
            }
            let length: f32 = typed_call!(
                crate::pathing::FAMILY,
                0x0045_7990,
                crate::pathing::point::length(e, Ptr::new(frame.wrapping_add(0xc)))
            );
            let radius = e.mem.u32(frame.wrapping_add(0x2c));
            let distance = x87_store32(x87_sub(f64::from(length), x87_m32(radius)));
            e.mem.set_u32(base.wrapping_add(0x34), distance);
            let accumulated: u32 = typed_call!(FAMILY, 0x00b4_f5c0, shader_accumulator(e));
            if e.mem.u32(accumulated.wrapping_add(8)) != 0 {
                let accumulated: u32 = typed_call!(FAMILY, 0x00b4_f5c0, shader_accumulator(e));
                let shader = e.mem.u32(accumulated.wrapping_add(8));
                let scale = e.mem.u32(shader.wrapping_add(0x110));
                let distance = x87_store32(x87_mul(
                    x87_load32(scale),
                    x87_m32(e.mem.u32(base.wrapping_add(0x34))),
                ));
                e.mem.set_u32(base.wrapping_add(0x34), distance);
            }
            let current = e.mem.f32(base.wrapping_add(0x34));
            if previous != current {
                if first_fade {
                    let limit = e.global::<f32>(FADE_FAR_9468);
                    clear_on_crossing(e, base, limit, previous);
                } else if second_fade {
                    let limit = e.global::<f32>(FADE_FAR_9460);
                    clear_on_crossing(e, base, limit, previous);
                } else if lod_fade {
                    let limit = e.global::<f32>(FADE_FAR_9470);
                    clear_on_crossing(e, base, limit, previous);
                }
                if main_fade {
                    let limit = e.global::<f32>(FADE_FAR_9458);
                    clear_on_crossing(e, base, limit, previous);
                }
            }
        }
        if properties != 0 {
            let material = e.mem.u32(geometry.wrapping_add(0xa4));
            if material == 0 {
                let alpha = x87_quiet_bits(e.mem.u32(base.wrapping_add(0x2c)));
                typed_call!(
                    crate::shader::FAMILY,
                    0x00ba_8ab0,
                    accumulator::set_alpha(e, property, f32::from_bits(alpha))
                );
            } else {
                let alpha = x87_store32(x87_mul(
                    x87_load32(e.mem.u32(material.wrapping_add(0x3c))),
                    x87_m32(e.mem.u32(base.wrapping_add(0x2c))),
                ));
                let mut chosen = alpha;
                if lod_fade {
                    let far = e.global::<u32>(FADE_FAR_9470);
                    let current = e.mem.f32(base.wrapping_add(0x34));
                    if f32::from_bits(far) != 0.0 && current >= f32::from_bits(far) {
                        lod_fade_kept = false;
                    } else {
                        let near = e.global::<u32>(FADE_NEAR_946C);
                        let (near_value, far_value) = (f32::from_bits(near), f32::from_bits(far));
                        if near_value != 0.0
                            && 0.0 != far_value
                            && current >= near_value
                            && current < far_value
                        {
                            let current_bits = x87_quiet_bits(e.mem.u32(base.wrapping_add(0x34)));
                            let faded = e
                                .call(
                                    INTERPOLATE,
                                    &args![
                                        0u32,
                                        1.0f32,
                                        x87_quiet_bits(far),
                                        x87_quiet_bits(near),
                                        current_bits
                                    ],
                                )
                                .st0;
                            chosen = x87_store32(faded);
                        }
                    }
                }
                typed_call!(
                    crate::shader::FAMILY,
                    0x00ba_8ab0,
                    accumulator::set_alpha(e, property, f32::from_bits(x87_quiet_bits(chosen)))
                );
            }
        }
        if e.mem.u32(base.wrapping_add(0x38)) == key
            && low8(flag_word) != 0
            && e.mem.u8(base.wrapping_add(0x74)) == 0
        {
            return;
        }
        e.mem.set_u8(base.wrapping_add(0x74), 0);
        let resort = ((e.mem.u32(base.wrapping_add(0x20)) & 0x0400_0000 != 0 || flag(e, base, 5))
            && (flag(e, base, 0xb) || flag(e, base, 0x1c)))
            || flag(e, base, 5)
            || e.mem
                .u8(e.mem.u32(geometry.wrapping_add(0x9c)).wrapping_add(0x18))
                & 1
                != 0;
        if resort {
            e.call(RESORT_LIGHTS, &args![base, frame.wrapping_add(0x20)]);
        }
        let low = low8(flag_word);
        if low == 1 {
            let passes = e.mem.u32(base.wrapping_add(0x3c));
            e.mem.set_u32(passes.wrapping_add(0x10), 0);
            if e.mem.f32(base.wrapping_add(0x28)) == 0.0 {
                return;
            }
            if lod_fade && !lod_fade_kept {
                return;
            }
            e.mem.set_u32(base.wrapping_add(0x38), key);
        } else {
            e.mem.set_u16(count_out, 0);
        }
        let level = e.global::<i32>(SHADER_LEVEL);
        let slot = if level == 1 {
            Some(0x114u32)
        } else if level >= 2 {
            Some(0x118)
        } else {
            None
        };
        if let Some(slot) = slot {
            let lights = e.mem.u32(accumulator.wrapping_add(0x19c));
            e.vcall(
                base,
                slot,
                &args![
                    geometry,
                    pass_word,
                    count_out,
                    lights,
                    accumulator,
                    flag_word
                ],
            );
        }
        let depth = e.call(DEPTH_PASSES_WANTED, &args![]).u8() != 0;
        let mut mark_last = true;
        if depth {
            if low == 0 {
                mark_last = false;
            } else if properties != 0 {
                let depth_word = depth_flag(e, geometry, flag_word);
                let skin = e.mem.u32(geometry.wrapping_add(0xbc));
                e.call(CHECK_CREATE_DEPTH_MAP_PASSES, &args![base, 1u32]);
                let array = e.mem.u32(base.wrapping_add(0x40));
                e.mem.set_u32(array.wrapping_add(0x10), 0);
                e.call(
                    ADD_DEPTH_MAP_PASS,
                    &args![
                        base,
                        geometry,
                        u32::from(skin != 0),
                        depth_word,
                        accumulator
                    ],
                );
            }
        }
        if mark_last && low != 0 {
            let passes = e.mem.u32(base.wrapping_add(0x3c));
            let count = e.mem.u32(passes.wrapping_add(0x10));
            if count > 0 {
                let list = e.mem.u32(passes.wrapping_add(4));
                let last = e
                    .mem
                    .u32(list.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4));
                if last != 0 {
                    e.mem.set_u8(last.wrapping_add(8), 1);
                }
            }
        }
        let passes = e.mem.u32(base.wrapping_add(0x3c));
        let count = e.mem.u32(passes.wrapping_add(0x10));
        if count > 1 {
            let first = e.mem.u32(e.mem.u32(passes.wrapping_add(4)));
            let number = u32::from(e.mem.u16(first.wrapping_add(4)));
            let lowered: u32 = typed_call!(FAMILY, 0x00b6_6760, lowered_pass_number(e, number));
            if low8(lowered) != 0 {
                let value = e.mem.u16(first.wrapping_add(4)).wrapping_add(0xffff);
                e.mem.set_u16(first.wrapping_add(4), value);
                return;
            }
        }
        if count > 2 {
            let mut list_owner = passes;
            let mut index = 0u32;
            loop {
                let pass = e.mem.u32(
                    e.mem
                        .u32(list_owner.wrapping_add(4))
                        .wrapping_add(index.wrapping_mul(4)),
                );
                let number = e.mem.u16(pass.wrapping_add(4));
                if number == 0xfc || number == 0x102 {
                    e.mem.set_u16(pass.wrapping_add(4), number.wrapping_sub(1));
                    list_owner = e.mem.u32(base.wrapping_add(0x3c));
                    index = e.mem.u32(list_owner.wrapping_add(0x10));
                }
                index = index.wrapping_add(1);
                let count = e
                    .mem
                    .u32(e.mem.u32(base.wrapping_add(0x3c)).wrapping_add(0x10));
                if index >= count {
                    break;
                }
            }
        }
    });
    e.mem.u32(base.wrapping_add(0x3c))
}
