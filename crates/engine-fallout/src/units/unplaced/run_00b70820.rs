//! `(unplaced)/run_00b70820` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The zero colour (4 words, `011a9bd0`).
const ZERO_COLOR: u32 = 0x011a_9bd0;
/// The shader state (`011f91e0`, a pointer to a pointer; the object at its
/// +0xA8 is used when its word at +0x1C is 8 to 12).
const SHADER_STATE: u32 = 0x011f_91e0;
/// Flags and values the lighting setters read: three bytes (`011f941e`,
/// `011f91de`, `011f9427`), the threshold `float` (`011f947c`), the extra
/// scale `float` (`011f9190`) and the 16-bit pass (`011f91e4`).
const LIGHTING_FLAG: u32 = 0x011f_941e;
const SKIP_FLAG: u32 = 0x011f_91de;
const SCALE_FLAG: u32 = 0x011f_9427;
const THRESHOLD: u32 = 0x011f_947c;
const EXTRA_SCALE: u32 = 0x011f_9190;
const PASS: u32 = 0x011f_91e4;
/// The `double` constants: two luminance weights (`010784d0` on green,
/// `01026998` on red and added once more as it is), the brightness offset
/// (`0101ffa0`) and 1.0 (`01012070`).
const GREEN_WEIGHT: u32 = 0x0107_84d0;
const RED_WEIGHT: u32 = 0x0102_6998;
const BRIGHTNESS_OFFSET: u32 = 0x0101_ffa0;
const ONE: u32 = 0x0101_2070;
/// The globals `00b70820` fills: the ambient colour (`011fa0c0`, 4 words),
/// the per-light colours (`011fa0d0`, 16 bytes each), the per-light
/// directions (`011fa1f0`, 16 bytes each), the per-light kinds
/// (`011fa070`, a word each) and the last light's flag byte (`011f91a6`).
const AMBIENT_COLOR: u32 = 0x011f_a0c0;
const LIGHT_COLORS: u32 = 0x011f_a0d0;
const LIGHT_DIRECTIONS: u32 = 0x011f_a1f0;
const LIGHT_KINDS: u32 = 0x011f_a070;
const LAST_LIGHT_FLAG: u32 = 0x011f_91a6;
/// The colour a directional light gets below full strength (3 words,
/// `011f4998`).
const DIMMED_COLOR: u32 = 0x011f_4998;
/// The callees: the point's unitize (`004a0c10`, `thiscall`), the
/// constant setter `004e20c0` (`cdecl (register, 4 words)`), the minimum
/// `0040ebd0` (`cdecl (a, b)`, ST0), the point scale `00439180`
/// (`thiscall (point, float)`), and the lighting shader's setters
/// `00bb95b0` (`cdecl (index, 4 words)`), `00bb95e0` (`cdecl (index, 4
/// words, 4 words)`) and `00bbb790` (`cdecl (&colour, flag)`).
const UNITIZE: u32 = 0x004a_0c10;
const SET_CONSTANT: u32 = 0x004e_20c0;
const MINIMUM: u32 = 0x0040_ebd0;
const SCALE_POINT: u32 = 0x0043_9180;
const SET_LIGHT: u32 = 0x00bb_95b0;
const SET_LIGHT_COLORS: u32 = 0x00bb_95e0;
const ADJUST_COLOR: u32 = 0x00bb_b790;

fn float_word(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// The shader state's object (`[[011f91e0]] + 0xA8`) when its word at
/// +0x1C is 8 to 12 (signed), else null.
fn shader_object(e: &Engine) -> u32 {
    let state = e.mem.u32(e.global::<u32>(SHADER_STATE));
    let object = e.mem.u32(state.wrapping_add(0xa8));
    if object == 0 {
        return 0;
    }
    let kind = e.mem.i32(object + 0x1c);
    if (8..=12).contains(&kind) {
        object
    } else {
        0
    }
}

/// The brightness step both colour setters share (`color` as loaded by
/// `FLD dword`): with the threshold above zero (else `None`), the
/// luminance `red * w2 + green * w1 + blue + w2` (stored as a `float`) at
/// or below zero makes all three the threshold; otherwise each component
/// plus the offset is scaled by threshold / luminance (1.0 when the
/// luminance is above the threshold), all in the exe's x87 operand order;
/// `blue_sum_first` picks the order of the blue component's two
/// operations, which the two callers write differently.
fn brighten(e: &Engine, color: [f64; 3], threshold: f64, blue_sum_first: bool) -> Option<[u32; 3]> {
    if !(threshold > 0.0) {
        return None;
    }
    let [x, y, z] = color;
    let green = x87_mul(x87_load64(e.global::<u64>(GREEN_WEIGHT)), y);
    let red_weight = x87_load64(e.global::<u64>(RED_WEIGHT));
    let red = x87_mul(x, red_weight);
    let sum = x87_add(x87_add(red, green), z);
    let luminance = x87_load32(x87_store32(x87_add(red_weight, sum)));
    if luminance <= 0.0 {
        let t = x87_store32(threshold);
        return Some([t, t, t]);
    }
    let ratio = if luminance > threshold {
        1.0
    } else {
        x87_div(threshold, luminance)
    };
    let ratio = x87_load32(x87_store32(ratio));
    let offset = x87_load64(e.global::<u64>(BRIGHTNESS_OFFSET));
    let red = x87_store32(x87_mul(x87_add(x, offset), ratio));
    let green = x87_store32(x87_mul(x87_add(y, offset), ratio));
    let blue = if blue_sum_first {
        x87_store32(x87_mul(x87_add(z, offset), ratio))
    } else {
        x87_store32(x87_mul(ratio, x87_add(offset, z)))
    };
    Some([red, green, blue])
}

/// The light's strength (`float` at +0xC4 of its data, through the FPU),
/// made 1.0 when above 1.0 unless the lighting flag is set.
fn capped_strength(e: &Engine, data: u32) -> f64 {
    let strength = x87_load32(e.mem.u32(data.wrapping_add(0xc4)));
    if e.mem.u8(LIGHTING_FLAG) == 0 && strength > f64::from_bits(e.global::<u64>(ONE)) {
        1.0
    } else {
        strength
    }
}

// Translated from 00b70820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets lighting-shader light `index` (below 8; the engine map has no
/// name; `cdecl (index, light, scale)`). Without a light the light's
/// colour slot gets the zero colour. With one (its data at +0xF8, its
/// flags at +0xF4 and +0xF5): a directional light (flag +0xF4) with data
/// sets constant `index + 0xB` to its colour (+0x8C, w 1.0); otherwise
/// data sets constant `index + 0x13` to its unitized direction (+0xF0, w
/// the shader object's vtable +0x10C or 1.0) and, unless the byte at
/// `011f91de` is set while the flag +0xF5 is clear, the ambient colour
/// `011fa0c0` to the data's +0xC8 colour times its strength (`0040ebd0`
/// against 1.0 unless the lighting flag), brightened (see `brighten`) and
/// times `scale` (w 1.0). Then the light's colour (+0xD4 times the capped
/// strength, times `scale`, times the light's +0xD0; a directional light
/// below full `scale` takes `011f4998`, else with the lighting flag and
/// not `011f9427` it is scaled by `011f9190`; in passes 0x154 to 0x177 the
/// shader object's vtable +0x10C scales it too) goes to its slot at
/// `011fa0d0` with the light's +0xD4 as w; a directional light's slot at
/// `011fa1f0` gets (+0xE0, 0, 0, 1). The flag byte `011f91a6` and the
/// kind word at `011fa070` (2 directional, else 1) are set.
pub fn fn_00b70820(e: &mut Engine, index: u32, light: Ptr, scale: f32) {
    if index >= 8 {
        return;
    }
    let light = light.addr();
    let slot = LIGHT_COLORS.wrapping_add(index << 4);
    if light == 0 {
        for i in 0..4 {
            let value = e.mem.u32(ZERO_COLOR + 4 * i);
            e.mem.set_u32(slot + 4 * i, value);
        }
        return;
    }
    let ambient_allowed = e.mem.u8(light + 0xf5);
    let data = e.mem.u32(light + 0xf8);
    let directional = e.mem.u8(light + 0xf4);
    let object = shader_object(e);
    let scale_value = x87_load32(scale.to_bits());
    if directional != 0 {
        if data != 0 {
            let color: [u32; 3] =
                std::array::from_fn(|i| x87_quiet_bits(e.mem.u32(data + 0x8c + 4 * i as u32)));
            e.call(
                SET_CONSTANT,
                &args![index + 0xb, color[0], color[1], color[2], 1.0f32],
            );
        }
    } else {
        if data != 0 {
            let direction = e.with_stack(0xc, |e, point| {
                let point = point.addr();
                for i in 0..3 {
                    let value = e.mem.u32(data + 0xf0 + 4 * i);
                    e.mem.set_u32(point + 4 * i, value);
                }
                e.call(UNITIZE, &args![point]);
                [
                    x87_quiet_bits(e.mem.u32(point)),
                    x87_quiet_bits(e.mem.u32(point + 4)),
                    x87_quiet_bits(e.mem.u32(point + 8)),
                ]
            });
            let w = if object != 0 {
                e.vcall(object, 0x10c, &args![]).f64()
            } else {
                1.0
            };
            let w = x87_store32(w);
            e.call(
                SET_CONSTANT,
                &args![index + 0x13, direction[0], direction[1], direction[2], w],
            );
        }
        if !(e.mem.u8(SKIP_FLAG) != 0 && ambient_allowed == 0) {
            let mut strength = x87_quiet_bits(e.mem.u32(data.wrapping_add(0xc4)));
            if e.mem.u8(LIGHTING_FLAG) == 0 {
                strength = x87_store32(e.call(MINIMUM, &args![1.0f32, strength]).f64());
            }
            let strength = x87_load32(strength);
            let base: [f64; 3] =
                std::array::from_fn(|i| float_word(e, data.wrapping_add(0xc8 + 4 * i as u32)));
            let mut color = [
                x87_store32(x87_mul(base[0], strength)),
                x87_store32(x87_mul(base[1], strength)),
                x87_store32(x87_mul(
                    strength,
                    x87_m32(e.mem.u32(data.wrapping_add(0xd0))),
                )),
            ];
            let threshold = x87_load32(x87_quiet_bits(e.global::<u32>(THRESHOLD)));
            let loaded = color.map(x87_load32);
            if let Some(bright) = brighten(e, loaded, threshold, false) {
                color = bright;
            }
            for (i, component) in color.iter().enumerate() {
                let scaled = x87_store32(x87_mul(x87_load32(*component), scale_value));
                e.set_global(AMBIENT_COLOR + 4 * i as u32, x87_quiet_bits(scaled));
            }
            e.set_global(AMBIENT_COLOR + 0xc, 1.0f32);
        }
    }
    let strength = capped_strength(e, data);
    let weight = x87_quiet_bits(e.mem.u32(light + 0xd0));
    let base: [f64; 3] =
        std::array::from_fn(|i| float_word(e, data.wrapping_add(0xd4 + 4 * i as u32)));
    let color = [
        x87_store32(x87_mul(base[0], strength)),
        x87_store32(x87_mul(base[1], strength)),
        x87_store32(x87_mul(
            strength,
            x87_m32(e.mem.u32(data.wrapping_add(0xdc))),
        )),
    ];
    let scaled = color.map(|component| x87_store32(x87_mul(x87_load32(component), scale_value)));
    let weight = x87_load32(weight);
    e.with_stack(0xc, |e, point| {
        let point = point.addr();
        let products = [
            x87_store32(x87_mul(x87_load32(scaled[0]), weight)),
            x87_store32(x87_mul(weight, x87_m32(scaled[1]))),
            x87_store32(x87_mul(weight, x87_m32(scaled[2]))),
        ];
        for (i, value) in products.iter().enumerate() {
            e.mem.set_u32(point + 4 * i as u32, *value);
        }
        if directional != 0 {
            if scale_value < 1.0 {
                for i in 0..3 {
                    let value = e.mem.u32(DIMMED_COLOR + 4 * i);
                    e.mem.set_u32(point + 4 * i, value);
                }
            }
        } else if e.mem.u8(LIGHTING_FLAG) != 0 && e.mem.u8(SCALE_FLAG) == 0 {
            let extra = x87_quiet_bits(e.global::<u32>(EXTRA_SCALE));
            e.call(SCALE_POINT, &args![point, extra]);
        }
        let pass = e.global::<u16>(PASS).wrapping_sub(0x154);
        let alpha = x87_quiet_bits(e.mem.u32(light + 0xd4));
        if pass <= 0x23 && object != 0 {
            let factor = x87_store32(e.vcall(object, 0x10c, &args![]).f64());
            e.call(SCALE_POINT, &args![point, factor]);
        }
        for i in 0..3 {
            let value = x87_quiet_bits(e.mem.u32(point + 4 * i));
            e.set_global(slot + 4 * i, value);
        }
        e.set_global(slot + 0xc, alpha);
    });
    if directional != 0 {
        let direction = LIGHT_DIRECTIONS.wrapping_add(index << 4);
        let first = x87_quiet_bits(e.mem.u32(data.wrapping_add(0xe0)));
        // The exe also reads +0xE4 and +0xE8 and overwrites them with 0.0.
        let _ = (
            e.mem.u32(data.wrapping_add(0xe4)),
            e.mem.u32(data.wrapping_add(0xe8)),
        );
        e.set_global(direction, first);
        e.set_global(direction + 4, 0.0f32);
        e.set_global(direction + 8, 0.0f32);
        e.set_global(direction + 0xc, 1.0f32);
    }
    e.set_global(LAST_LIGHT_FLAG, directional);
    e.set_global(LIGHT_KINDS + 4 * index, u32::from(directional != 0) + 1);
}

/// The four words of the zero colour.
fn zero_color(e: &Engine) -> [u32; 4] {
    std::array::from_fn(|i| e.mem.u32(ZERO_COLOR + 4 * i as u32))
}

/// The light's colour times `strength` (three `float`s from `at`: the
/// first two loaded and multiplied by `strength`, the third the memory
/// operand of `strength`'s `FMUL`), each stored as a `float`.
fn scaled_color(e: &Engine, at: u32, strength: f64) -> [u32; 3] {
    [
        x87_store32(x87_mul(float_word(e, at), strength)),
        x87_store32(x87_mul(float_word(e, at + 4), strength)),
        x87_store32(x87_mul(strength, x87_m32(e.mem.u32(at + 8)))),
    ]
}

// Translated from 00b70de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets point light `index` of the lighting shader (the engine map has no
/// name; `cdecl (index, light, scale)`): without a light, `00bb95b0` with
/// the zero colour. Otherwise the data's position (+0xF0, w the shader
/// object's vtable +0x10C or 1.0) and the light's colour go to
/// `00bb95e0`: the colour is the data's +0xD4 times its strength (capped at
/// 1.0 unless the lighting flag), times `scale`, times the light's +0xD0
/// (and times `011f9190` with the lighting flag and not `011f9427`), w the
/// light's +0xD4. On the way the data's +0xC8 colour times the capped
/// strength, brightened and times `scale` (w 1.0), goes to `00bbb790` with
/// whether `scale` is below 1.0 (the exe passes that byte in a word whose
/// other bytes are whatever its stack held; zero here).
pub fn fn_00b70de0(e: &mut Engine, index: u32, light: Ptr, scale: f32) {
    let light = light.addr();
    if light == 0 {
        let zero = zero_color(e);
        e.call(SET_LIGHT, &args![index, zero[0], zero[1], zero[2], zero[3]]);
        return;
    }
    let object = shader_object(e);
    let data = e.mem.u32(light + 0xf8);
    let position: [u32; 3] =
        std::array::from_fn(|i| x87_quiet_bits(e.mem.u32(data.wrapping_add(0xf0 + 4 * i as u32))));
    let w = if object != 0 {
        e.vcall(object, 0x10c, &args![]).f64()
    } else {
        1.0
    };
    let w = x87_store32(w);
    let threshold = x87_load32(x87_quiet_bits(e.global::<u32>(THRESHOLD)));
    // The exe first copies the zero colour into the colour local (all of
    // it is overwritten before use).
    let _ = zero_color(e);
    let strength = x87_store32(capped_strength(e, data));
    let mut color = scaled_color(e, data.wrapping_add(0xc8), x87_load32(strength));
    if let Some(bright) = brighten(e, color.map(x87_load32), threshold, true) {
        color = bright;
    }
    let scale_value = x87_load32(scale.to_bits());
    let scaled = color.map(|component| x87_store32(x87_mul(x87_load32(component), scale_value)));
    let below_full = u32::from(1.0 > scale_value);
    e.with_stack(0x10, |e, adjusted| {
        let adjusted = adjusted.addr();
        for (i, component) in scaled.iter().enumerate() {
            e.mem
                .set_u32(adjusted + 4 * i as u32, x87_quiet_bits(*component));
        }
        e.mem.set_f32(adjusted + 0xc, 1.0);
        e.call(ADJUST_COLOR, &args![adjusted, below_full]);
    });
    let strength = x87_store32(x87_mul(x87_load32(strength), x87_m32(scale.to_bits())));
    let mut strength = x87_store32(x87_mul(
        x87_load32(strength),
        x87_m32(e.mem.u32(light + 0xd0)),
    ));
    if e.mem.u8(LIGHTING_FLAG) != 0 && e.mem.u8(SCALE_FLAG) == 0 {
        strength = x87_store32(x87_mul(float_word(e, EXTRA_SCALE), x87_m32(strength)));
    }
    let color = scaled_color(e, data.wrapping_add(0xd4), x87_load32(strength));
    let alpha = x87_quiet_bits(e.mem.u32(light + 0xd4));
    e.call(
        SET_LIGHT_COLORS,
        &args![
            index,
            position[0],
            position[1],
            position[2],
            w,
            color[0],
            color[1],
            color[2],
            alpha
        ],
    );
}

// Translated from 00b71130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets light `index` (below 20) of the lighting shader's other set (the
/// engine map has no name; `cdecl (index, light, scale)`): without a light
/// `00bb95b0` with the zero colour; otherwise `00bb95e0` with the data's
/// +0x8C colour (w its +0xE0) and, for `scale` below 1.0, the zero colour,
/// else the data's +0xD4 colour times its strength (`0040ebd0` against
/// 1.0 unless the lighting flag), `scale` and the light's +0xD0; w the
/// light's +0xD4.
pub fn fn_00b71130(e: &mut Engine, index: u32, light: Ptr, scale: f32) {
    if index >= 0x14 {
        return;
    }
    let light = light.addr();
    if light == 0 {
        let zero = zero_color(e);
        e.call(SET_LIGHT, &args![index, zero[0], zero[1], zero[2], zero[3]]);
        return;
    }
    let data = e.mem.u32(light + 0xf8);
    let first: [u32; 4] = [
        x87_quiet_bits(e.mem.u32(data.wrapping_add(0x8c))),
        x87_quiet_bits(e.mem.u32(data.wrapping_add(0x90))),
        x87_quiet_bits(e.mem.u32(data.wrapping_add(0x94))),
        x87_quiet_bits(e.mem.u32(data.wrapping_add(0xe0))),
    ];
    let _ = (
        e.mem.u32(data.wrapping_add(0xe4)),
        e.mem.u32(data.wrapping_add(0xe8)),
    );
    let scale_value = x87_load32(scale.to_bits());
    let color = if 1.0 > scale_value {
        let zero = zero_color(e);
        [zero[0], zero[1], zero[2]]
    } else {
        let mut strength = x87_quiet_bits(e.mem.u32(data.wrapping_add(0xc4)));
        if e.mem.u8(LIGHTING_FLAG) == 0 {
            strength = x87_store32(e.call(MINIMUM, &args![1.0f32, strength]).f64());
        }
        let strength = x87_store32(x87_mul(x87_load32(strength), x87_m32(scale.to_bits())));
        let strength = x87_store32(x87_mul(
            x87_load32(strength),
            x87_m32(e.mem.u32(light + 0xd0)),
        ));
        scaled_color(e, data.wrapping_add(0xd4), x87_load32(strength))
    };
    let alpha = x87_quiet_bits(e.mem.u32(light + 0xd4));
    e.call(
        SET_LIGHT_COLORS,
        &args![index, first[0], first[1], first[2], first[3], color[0], color[1], color[2], alpha],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_0820, fn_00b70820(u32, Ptr, f32)),
        entry!(0x00b7_0de0, fn_00b70de0(u32, Ptr, f32)),
        entry!(0x00b7_1130, fn_00b71130(u32, Ptr, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shader_calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn bits(values: [f32; 4]) -> Vec<u32> {
        values.iter().map(|value| value.to_bits()).collect()
    }

    /// An engine with the lighting globals mapped (zero colour (0, 0, 0,
    /// 1), weights 0.34 and 0.33, offset 0.1, threshold `threshold`), the
    /// callees doubled (the minimum answers the smaller argument), and a
    /// light whose data holds: strength 0.5 (+0xC4), colours (1, 2, 3)
    /// (+0x8C), (2, 4, 6) (+0xC8), (1, 1, 1) (+0xD4), position (7, 8, 9)
    /// (+0xF0) and +0xE0 = 5; the light's +0xD0 is 3 and +0xD4 is 0.75.
    fn shader_engine(threshold: f32) -> (Engine, u32) {
        let mut e = Engine::new();
        for (address, size) in [
            (ZERO_COLOR, 0x10u32),
            (SHADER_STATE, 4),
            (LIGHTING_FLAG, 1),
            (SKIP_FLAG, 1),
            (SCALE_FLAG, 1),
            (THRESHOLD, 4),
            (EXTRA_SCALE, 4),
            (PASS, 2),
            (GREEN_WEIGHT, 8),
            (RED_WEIGHT, 8),
            (BRIGHTNESS_OFFSET, 8),
            (ONE, 8),
            (AMBIENT_COLOR, 0x10),
            (LIGHT_COLORS, 0x80),
            (LIGHT_DIRECTIONS, 0x80),
            (LIGHT_KINDS, 0x20),
            (LAST_LIGHT_FLAG, 1),
            (DIMMED_COLOR, 0xc),
        ] {
            e.map(address, size);
        }
        e.set_global(ZERO_COLOR + 0xc, 1.0f32);
        e.set_global(GREEN_WEIGHT, 0.34f64);
        e.set_global(RED_WEIGHT, 0.33f64);
        e.set_global(BRIGHTNESS_OFFSET, 0.1f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(THRESHOLD, threshold);
        e.set_global(PASS, 0x200u16);
        let state = e.mem.alloc(4);
        let inner = e.mem.alloc(0xac);
        e.mem.set_u32(state, inner);
        e.set_global(SHADER_STATE, state);
        for address in [
            UNITIZE,
            SET_CONSTANT,
            SCALE_POINT,
            SET_LIGHT,
            SET_LIGHT_COLORS,
            ADJUST_COLOR,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(MINIMUM, |_, a| {
            f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))).into_ret()
        });
        let data = e.mem.alloc(0x100);
        e.mem.set_f32(data + 0xc4, 0.5);
        for (offset, values) in [
            (0x8cu32, [1.0f32, 2.0, 3.0]),
            (0xc8, [2.0, 4.0, 6.0]),
            (0xd4, [1.0, 1.0, 1.0]),
            (0xf0, [7.0, 8.0, 9.0]),
        ] {
            for (i, value) in values.iter().enumerate() {
                e.mem.set_f32(data + offset + 4 * i as u32, *value);
            }
        }
        e.mem.set_f32(data + 0xe0, 5.0);
        let light = e.mem.alloc(0x100);
        e.mem.set_u32(light + 0xf8, data);
        e.mem.set_f32(light + 0xd0, 3.0);
        e.mem.set_f32(light + 0xd4, 0.75);
        e.call_log = Some(vec![]);
        (e, light)
    }

    fn global_words(e: &Engine, address: u32) -> Vec<u32> {
        (0..4).map(|i| e.global::<u32>(address + 4 * i)).collect()
    }

    #[test]
    fn light_without_object_or_beyond_the_table() {
        let (mut e, _) = shader_engine(0.0);
        e.call(0x00b7_0820, &args![8u32, 0u32, 1.0f32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.set_global(LIGHT_COLORS + 0x20, 9u32);
        e.call(0x00b7_0820, &args![2u32, 0u32, 1.0f32]);
        assert_eq!(
            global_words(&e, LIGHT_COLORS + 0x20),
            bits([0.0, 0.0, 0.0, 1.0])
        );
    }

    #[test]
    fn point_light_sets_direction_ambient_and_colour() {
        let (mut e, light) = shader_engine(0.0);
        e.call(0x00b7_0820, &args![1u32, light, 2.0f32]);
        assert_eq!(
            shader_calls(&e, SET_CONSTANT),
            vec![[vec![0x14], bits([7.0, 8.0, 9.0, 1.0])].concat()]
        );
        // (2, 4, 6) * min(1, 0.5) * 2.
        assert_eq!(global_words(&e, AMBIENT_COLOR), bits([2.0, 4.0, 6.0, 1.0]));
        // (1, 1, 1) * 0.5 * 2 * 3, w the light's 0.75.
        assert_eq!(
            global_words(&e, LIGHT_COLORS + 0x10),
            bits([3.0, 3.0, 3.0, 0.75])
        );
        assert_eq!(e.global::<u32>(LIGHT_KINDS + 4), 1);
        assert_eq!(e.global::<u8>(LAST_LIGHT_FLAG), 0);
        // With the skip flag and +0xF5 clear the ambient is left alone.
        e.set_global(AMBIENT_COLOR, 0u32);
        e.set_global(SKIP_FLAG, 1u8);
        e.call(0x00b7_0820, &args![1u32, light, 2.0f32]);
        assert_eq!(e.global::<u32>(AMBIENT_COLOR), 0);
    }

    #[test]
    fn ambient_is_brightened_up_to_the_threshold() {
        let (mut e, light) = shader_engine(10.0);
        e.call(0x00b7_0820, &args![1u32, light, 1.0f32]);
        // s = (1, 2, 3); L = 0.33 + (1 * 0.33 + 0.34 * 2) + 3.
        let luminance = (0.33 + ((1.0 * 0.33 + 0.34 * 2.0) + 3.0)) as f32;
        let ratio = (10.0 / luminance as f64) as f32 as f64;
        let expected = [
            ((1.0 + 0.1) * ratio) as f32,
            ((2.0 + 0.1) * ratio) as f32,
            (ratio * (0.1 + 3.0)) as f32,
            1.0,
        ];
        assert_eq!(global_words(&e, AMBIENT_COLOR), bits(expected));
        // A zero colour has no luminance: the threshold everywhere.
        let (mut e, light) = shader_engine(10.0);
        e.set_global(RED_WEIGHT, 0.0f64);
        let data = e.mem.u32(light + 0xf8);
        e.mem.set_f32(data + 0xc4, 0.0);
        e.call(0x00b7_0820, &args![1u32, light, 1.0f32]);
        assert_eq!(
            global_words(&e, AMBIENT_COLOR),
            bits([10.0, 10.0, 10.0, 1.0])
        );
    }

    #[test]
    fn directional_light_sets_its_colour_constant_and_direction() {
        let (mut e, light) = shader_engine(0.0);
        e.mem.set_u8(light + 0xf4, 1);
        for i in 0..3 {
            e.set_global(DIMMED_COLOR + 4 * i, 0x100 + i);
        }
        e.call(0x00b7_0820, &args![3u32, light, 0.5f32]);
        assert_eq!(
            shader_calls(&e, SET_CONSTANT),
            vec![[vec![0xe], bits([1.0, 2.0, 3.0, 1.0])].concat()]
        );
        // Below full scale the colour is the dimmed one.
        assert_eq!(
            global_words(&e, LIGHT_COLORS + 0x30),
            vec![0x100, 0x101, 0x102, 0.75f32.to_bits()]
        );
        assert_eq!(
            global_words(&e, LIGHT_DIRECTIONS + 0x30),
            bits([5.0, 0.0, 0.0, 1.0])
        );
        assert_eq!(e.global::<u32>(LIGHT_KINDS + 0xc), 2);
        assert_eq!(e.global::<u8>(LAST_LIGHT_FLAG), 1);
    }

    #[test]
    fn point_light_colours_go_to_the_shader() {
        let (mut e, light) = shader_engine(0.0);
        e.call(0x00b7_0de0, &args![4u32, 0u32, 1.0f32]);
        assert_eq!(
            shader_calls(&e, SET_LIGHT),
            vec![[vec![4], bits([0.0, 0.0, 0.0, 1.0])].concat()]
        );
        e.call(0x00b7_0de0, &args![4u32, light, 2.0f32]);
        // Strength 0.5 * 2 * 3 = 3 on (1, 1, 1).
        assert_eq!(
            shader_calls(&e, SET_LIGHT_COLORS),
            vec![[
                vec![4],
                bits([7.0, 8.0, 9.0, 1.0]),
                bits([3.0, 3.0, 3.0, 0.75])
            ]
            .concat()]
        );
        // The adjusted colour: (2, 4, 6) * 0.5 * 2, not below full scale.
        let adjust = shader_calls(&e, ADJUST_COLOR);
        assert_eq!(adjust[0][1], 0);
        e.call_log = Some(vec![]);
        e.call(0x00b7_0de0, &args![4u32, light, 0.5f32]);
        assert_eq!(shader_calls(&e, ADJUST_COLOR)[0][1], 1);
    }

    #[test]
    fn other_light_set_uses_zero_colour_below_full_scale() {
        let (mut e, light) = shader_engine(0.0);
        e.call(0x00b7_1130, &args![20u32, light, 1.0f32]);
        assert!(shader_calls(&e, SET_LIGHT_COLORS).is_empty());
        e.call(0x00b7_1130, &args![5u32, 0u32, 1.0f32]);
        assert_eq!(shader_calls(&e, SET_LIGHT).len(), 1);
        e.call(0x00b7_1130, &args![5u32, light, 0.5f32]);
        e.call(0x00b7_1130, &args![5u32, light, 2.0f32]);
        assert_eq!(
            shader_calls(&e, SET_LIGHT_COLORS),
            vec![
                [
                    vec![5],
                    bits([1.0, 2.0, 3.0, 5.0]),
                    bits([0.0, 0.0, 0.0, 0.75])
                ]
                .concat(),
                [
                    vec![5],
                    bits([1.0, 2.0, 3.0, 5.0]),
                    bits([3.0, 3.0, 3.0, 0.75])
                ]
                .concat(),
            ]
        );
    }
}
