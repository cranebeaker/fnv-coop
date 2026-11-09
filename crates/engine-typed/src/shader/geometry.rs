//! The shadow light shader's per-geometry constants (`ShadowLightShader::SetupGeometryConstants`,
//! `00b7dab0`), the set-up the batch renderer runs for nearly every lit geometry of the crowded street
//! (about 600 times a frame), with the constant set-ups it calls in the exe's order: the lights, the
//! ambient and emittance colours, the object-to-cube and shadow projection matrices, the eye position,
//! the toggles, the clip planes and the smaller entries. Then the pixel and vertex constant maps upload
//! their entries (`SetShaderConstants`, [`super::constants`], made direct when the map's slot holds it).
//!
//! Only `SetupGeometryConstants` is registered here: the set-ups it calls are plain Rust calls, as in
//! the translation (which keeps them registered at their own addresses for other callers). Everything they
//! write to the device goes through the same calls in the same order: the D3D9 stream is the
//! translation's. Floats use the exact x87 helpers, the same under either control word.

use super::{accumulator, constants, state, FAMILY};
use engine_core::prelude::*;

/// The rendering object and its pass type (`011f91e0`, `011f91e4`), the current pass (`0126f74c`) and
/// the position used without a geometry bound (`011f4288`).
const RENDERING_PASS: u32 = 0x011f_91e0;
const RENDERING_PASS_TYPE: u32 = 0x011f_91e4;
const CURRENT_PASS: u32 = 0x0126_f74c;
const DEFAULT_POSITION: u32 = 0x011f_4288;
/// The first word of the rendering pass's (`011f91e0`) +0xc.
/// A loop's small index as a word.
#[allow(clippy::cast_possible_truncation)]
fn idx(i: usize) -> u32 {
    i as u32
}

fn rendering_object(e: &Engine) -> u32 {
    let rendering = e.global::<u32>(RENDERING_PASS);
    let holder = e.mem.u32(rendering + 0xc);
    e.mem.u32(holder)
}

/// The renderer (`011f4748`), the HDR switch (`011f941e`), the colour row the lights scale (`011fa0d0`), and
/// the D3DX import thunks `D3DXVec3TransformNormal` (`00ee6df2`) and `D3DXVec3Normalize` (`00ee6dec`).
const RENDERER: u32 = 0x011f_4748;
const HDR: u32 = 0x011f_941e;
const COLOUR: u32 = 0x011f_a0d0;
const VEC3_TRANSFORM_NORMAL: u32 = 0x00ee_6df2;
const VEC3_NORMALIZE: u32 = 0x00ee_6dec;

/// The shader's vertex and pixel constant-map entry pointers (`ShadowLightShader::
/// pVertexConstantMapEntry` / `pPixelConstantMapEntry` (Xbox PDB), 22 words each from
/// `011febd8` and `011fec30` on PC).
pub(crate) const VERTEX_ENTRIES: u32 = 0x011f_ebd8;
pub(crate) const PIXEL_ENTRIES: u32 = 0x011f_ec30;

/// Whether the constant-map entry the pointer at `slot` holds is enabled (`m_bEnabled`, +8).
pub(crate) fn entry_enabled(e: &Engine, slot: u32) -> bool {
    let entry = e.mem.u32(slot);
    e.mem.u8(entry.wrapping_add(8)) != 0
}

/// A float copied through the x87 stack (`FLD dword`, `FSTP dword`): a signalling NaN comes
/// out quiet.
fn copy_float(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

/// CRT `_memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// The 4x4 matrix `00b79050` makes the identity (`011fec88`).
const MATRIX: u32 = 0x011f_ec88;

// Typed from 00b79050
/// Unnamed in the engine map, `thiscall` with one word. With vertex
/// entry 12 enabled and the word not null: an identity matrix built on the stack (zeros from
/// `FLDZ`, ones from `FLD1`) is copied to `011fec88` (`memcpy`, 0x40 bytes).
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn vertex_entry_12(e: &mut Engine, _this: Ptr, object: u32) {
    if !entry_enabled(e, VERTEX_ENTRIES + 4 * 12) || object == 0 {
        return;
    }
    e.with_stack(0x40, |e, matrix| {
        let matrix = matrix.addr();
        for index in 0..16u32 {
            let value = if index % 5 == 0 { 1.0f32 } else { 0.0 };
            e.mem.set_u32(matrix + 4 * index, value.to_bits());
        }
        e.call(MEMCPY, &args![MATRIX, matrix, 0x40u32]);
    });
}

/// The game time the shaders animate with (`011f940c`), `3600.0` (`double`, `01012640`), the
/// `double` `6.28318` (`0104eea0`) and the angle `00b790d0` stores (`011fd868`).
const GAME_TIME: u32 = 0x011f_940c;
const SECONDS_PER_HOUR: u32 = 0x0101_2640;
const TWO_PI: u32 = 0x0104_eea0;
const ANGLE: u32 = 0x011f_d868;

// Typed from 00b790d0
/// Unnamed in the engine map, `thiscall` with one word, an object
/// with an integer at +0xe4. With vertex entry 15 enabled: `011fd868` = game time (`011f940c`)
/// / 3600.0 * 6.28318 * the integer (x87, in the exe's order).
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn vertex_entry_15(e: &mut Engine, _this: Ptr, object: Ptr) {
    if !entry_enabled(e, VERTEX_ENTRIES + 4 * 15) {
        return;
    }
    let time = x87_load32(e.global::<u32>(GAME_TIME));
    let value = x87_div(time, e.global::<f64>(SECONDS_PER_HOUR));
    let value = x87_mul(value, e.global::<f64>(TWO_PI));
    let count = e.mem.u32(object.addr() + 0xe4) as i32;
    let value = x87_mul(value, f64::from(count));
    e.set_global(ANGLE, x87_store32(value));
}

/// The camera position the shaders subtract (`011f474c`, three floats), the `double` 15.0
/// (`01015a38`) and the four floats `00b79100` reads (`011f95f4`) and writes (`011fa2e0`).
const CAMERA: u32 = 0x011f_474c;
const FIFTEEN: u32 = 0x0101_5a38;
const SOURCE: u32 = 0x011f_95f4;
const TARGET: u32 = 0x011f_a2e0;

// Typed from 00b79100
/// Unnamed in the engine map, `thiscall` with one word it does not
/// read. With vertex entry 16 enabled: the four floats at `011fa2e0` become those at
/// `011f95f4` less the camera's x and y (`011f474c`) and less 15.0 twice (the last as `FSUBR`).
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn vertex_entry_16(e: &mut Engine, _this: Ptr, _unused_0: u32) {
    if !entry_enabled(e, VERTEX_ENTRIES + 4 * 16) {
        return;
    }
    for index in 0..2u32 {
        let source = x87_load32(e.global::<u32>(SOURCE + 4 * index));
        let camera = x87_m32(e.global::<u32>(CAMERA + 4 * index));
        let value = x87_store32(x87_sub(source, camera));
        e.set_global(TARGET + 4 * index, value);
    }
    let third = x87_load32(e.global::<u32>(SOURCE + 8));
    let fifteen = x87_load64(e.global::<u64>(FIFTEEN));
    let value = x87_store32(x87_sub(third, fifteen));
    e.set_global(TARGET + 8, value);
    let fourth = x87_m32(e.global::<u32>(SOURCE + 0xc));
    let value = x87_store32(x87_sub(fourth, fifteen));
    e.set_global(TARGET + 0xc, value);
}

/// The three floats `00b79180` writes (`011fd8b4`).
const OFFSET: u32 = 0x011f_d8b4;

// Typed from 00b79180
/// Unnamed in the engine map, `thiscall` with one word, an object
/// whose +0x20 points at a position (`011f4288` when null). With vertex entry 17 enabled: the
/// position less the camera (`011f474c`), each rounded to a float on the stack and copied to
/// `011fd8b4`.
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn vertex_entry_17(e: &mut Engine, _this: Ptr, object: Ptr) {
    if !entry_enabled(e, VERTEX_ENTRIES + 4 * 17) {
        return;
    }
    let mut position = e.mem.u32(object.addr() + 0x20);
    if position == 0 {
        position = DEFAULT_POSITION;
    }
    let mut values = [0u32; 3];
    for (index, value) in values.iter_mut().enumerate() {
        let offset = 4 * idx(index);
        let coordinate = x87_load32(e.mem.u32(position + offset));
        let camera = x87_m32(e.global::<u32>(CAMERA + offset));
        *value = x87_store32(x87_sub(coordinate, camera));
    }
    for (index, value) in values.iter().enumerate() {
        e.set_global(OFFSET + 4 * idx(index), x87_quiet_bits(*value));
    }
}

/// The four floats `00b791f0` writes (`011fa0b0`), the float it uses for type 0xfe
/// (`011ad808`), the times and scale of its type 0x230 (`011f94ac`, `011ad838`; `double` 5.0
/// at `01020998`), the two pairs it lerps between (`011f962c`/`011f9634` and
/// `011f9630`/`011f9638`), the byte it clears when the factor is 1.0 (`011f94aa`), and the
/// float max / min helpers (`00404010`, `0040ebd0`; `cdecl`, two floats, the result in ST0).
const VECTOR: u32 = 0x011f_a0b0;
const VALUE_DEFAULT: u32 = 0x011a_d808;
const START_TIME: u32 = 0x011f_94ac;
const DURATION: u32 = 0x011a_d838;
const FIVE: u32 = 0x0102_0998;
const TO_X: u32 = 0x011f_962c;
const TO_Y: u32 = 0x011f_9630;
const FROM_X: u32 = 0x011f_9634;
const FROM_Y: u32 = 0x011f_9638;
const RUNNING: u32 = 0x011f_94aa;
const FLOAT_MAX: u32 = 0x0040_4010;
const FLOAT_MIN: u32 = 0x0040_ebd0;

// Typed from 00b791f0
/// Unnamed in the engine map, `thiscall` with three words: the pass
/// type and two objects. With vertex entry 21 enabled, `011fa0b0` (four floats) becomes for
/// type 0xfe (the first object's +0x80, `011ad808`, 0, 0); for 0x61 (1, 0, 0, 0); for 0x230 the
/// first object's +0x94 to +0xa0 with the last two replaced by the lerps between the pairs at
/// `011f9634`/`011f962c` and `011f9638`/`011f9630` by the factor min(1, max(0, (time -
/// `011f94ac`) / (`011ad838` * 5))) less the second object's +0x8c and +0x90 (the factor 1.0
/// clears `011f94aa`). Every x87 step and copy is in the exe's order.
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn vertex_entry_21(e: &mut Engine, _this: Ptr, pass_type: u32, first: Ptr, second: Ptr) {
    if !entry_enabled(e, VERTEX_ENTRIES + 4 * 21) {
        return;
    }
    let first = first.addr();
    let second = second.addr();
    match pass_type {
        0xfe => {
            let x = copy_float(e, first + 0x80);
            e.set_global(VECTOR, x);
            let y = copy_float(e, VALUE_DEFAULT);
            e.set_global(VECTOR + 4, y);
            e.set_global(VECTOR + 8, 0u32);
            e.set_global(VECTOR + 0xc, 0u32);
        }
        0x61 => {
            e.set_global(VECTOR, 1.0f32.to_bits());
            for offset in [4u32, 8, 0xc] {
                e.set_global(VECTOR + offset, 0u32);
            }
        }
        0x230 => {
            let time = x87_load32(e.global::<u32>(GAME_TIME));
            let elapsed = x87_sub(time, x87_m32(e.global::<u32>(START_TIME)));
            let scale = x87_load32(e.global::<u32>(DURATION));
            let scale = x87_mul(scale, e.global::<f64>(FIVE));
            let x_offset = e.mem.u32(second + 0x8c);
            let y_offset = e.mem.u32(second + 0x90);
            // The exe also reads +0x94 into a stack slot nothing reads.
            let _unread = e.mem.u32(second + 0x94);
            let ratio = x87_store32(x87_div(elapsed, scale));
            let ratio = x87_quiet_bits(ratio);
            let raised = e.call(FLOAT_MAX, &args![0u32, ratio]).f64();
            let raised = x87_store32(raised);
            let factor = e.call(FLOAT_MIN, &args![1.0f32.to_bits(), raised]).f64();
            let factor = x87_store32(factor);
            let to_x = x87_load32(e.global::<u32>(TO_X));
            let span_x = x87_store32(x87_sub(to_x, x87_m32(e.global::<u32>(FROM_X))));
            let to_y = x87_load32(e.global::<u32>(TO_Y));
            let span_y = x87_store32(x87_sub(to_y, x87_m32(e.global::<u32>(FROM_Y))));
            let from_x = e.global::<u32>(FROM_X);
            let from_y = e.global::<u32>(FROM_Y);
            let factor_value = x87_load32(factor);
            let scaled_x = x87_store32(x87_mul(x87_load32(span_x), factor_value));
            let scaled_y = x87_store32(x87_mul(x87_load32(span_y), factor_value));
            let lerp_x = x87_store32(x87_add(x87_load32(scaled_x), x87_m32(from_x)));
            let lerp_y = x87_store32(x87_add(x87_load32(scaled_y), x87_m32(from_y)));
            if factor_value == 1.0 {
                e.set_global(RUNNING, 0u8);
            }
            for (index, offset) in [0x94u32, 0x98, 0x9c, 0xa0].iter().enumerate() {
                let word = e.mem.u32(first + offset);
                e.set_global(VECTOR + 4 * idx(index), word);
            }
            let x = x87_store32(x87_sub(x87_load32(lerp_x), x87_m32(x_offset)));
            e.set_global(VECTOR + 8, x);
            let y = x87_store32(x87_sub(x87_load32(lerp_y), x87_m32(y_offset)));
            e.set_global(VECTOR + 0xc, y);
        }
        _ => {}
    }
}

/// The rows of four floats `00b79410` and `00b7c580` write (`011fa0c0`, 0x10 bytes each).
const ROWS: u32 = 0x011f_a0c0;

// Typed from 00b79410
/// Unnamed in the engine map, `thiscall` with two words: the pass
/// type and an object (bytes at +9 and +0xb). Types 0x93 and 0x1f1 to 0x1f5 use rows 2 and 3
/// of `011fa0c0` and set pixel entry 2's register count (+0x20) to max(3, byte +9 + 2,
/// signed); type 0x1f6 uses rows 0 and 0x1b; others leave everything. With the selector byte
/// +0xb the first row becomes (1 for 1, 1 for 2, 1 for 3, 1 for 4, else 0 each) and the second
/// (selector 5, 6, 7, 8), as the x87 stack juggling of 0.0 and 1.0 in the exe leaves them.
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn pixel_entry_2(e: &mut Engine, _this: Ptr, pass_type: u32, object: Ptr) {
    let object = object.addr();
    let (first_row, second_row);
    if pass_type > 0x1f5 {
        if pass_type != 0x1f6 {
            return;
        }
        first_row = 0u32;
        second_row = 0x1bu32;
    } else {
        if pass_type < 0x1f1 && pass_type != 0x93 {
            return;
        }
        first_row = 2;
        second_row = 3;
        let mut count = u32::from(e.mem.u8(object + 9)) + 2;
        if (count as i32) <= 3 {
            count = 3;
        }
        let entry = e.global::<u32>(PIXEL_ENTRIES + 4 * 2);
        e.mem.set_u32(entry + 0x20, count);
    }
    let selector = u32::from(e.mem.u8(object + 0xb));
    let flag = |wanted: u32| {
        if selector == wanted {
            1.0f32.to_bits()
        } else {
            0
        }
    };
    let first = ROWS + first_row * 0x10;
    for (index, wanted) in [1u32, 2, 3, 4].iter().enumerate() {
        e.mem.set_u32(first + 4 * idx(index), flag(*wanted));
    }
    let second = ROWS.wrapping_add(second_row * 0x10);
    for (index, wanted) in [5u32, 6, 7, 8].iter().enumerate() {
        e.mem.set_u32(second + 4 * idx(index), flag(*wanted));
    }
}

/// `ShadowLightShader::UpdateTogglesConstantLand` (`00c02c30`, engine map; `thiscall`, four
/// words), `BSShaderManager::GetFogProperty` (`00b55520`, `cdecl`, one word), the fog
/// property index byte (`011f91c4`), the three floats `SetupGeometryConstants_Toggles` writes
/// (`011fa2d4`, `011fa2d8`, `011fa2dc`), `1.0` and `255.0` (`double`, `01012070`, `0101e568`) and
/// the float it uses for bit 0 (`010af2f0`).
const UPDATE_TOGGLES_LAND: u32 = 0x00c0_2c30;
const GET_FOG_PROPERTY: u32 = 0x00b5_5520;
const FOG_INDEX: u32 = 0x011f_91c4;
const TOGGLE_FIRST: u32 = 0x011f_a2d4;
const TOGGLE_SECOND: u32 = 0x011f_a2d8;
const TOGGLE_THIRD: u32 = 0x011f_a2dc;
const ONE: u32 = 0x0101_2070;
const BYTE_SCALE: u32 = 0x0101_e568;
const TOGGLE_VALUE: u32 = 0x010a_f2f0;

// Typed from 00b795b0
/// `ShadowLightShader::SetupGeometryConstants_Toggles` (engine map), `thiscall` with six stack
/// words: the pass type, a geometry word, the shader property, a fog word, an object with a
/// byte at +8 and a property with flags at +0x18. With pixel entry 4 enabled:
/// `UpdateTogglesConstantLand(geometry, 0, property, 0)` for types 0x1f7 to 0x22e, else the
/// shader's `UpdateTogglesConstant` (vtable +0x154) with the geometry word, the fog property
/// (`GetFogProperty` of the byte at `011f91c4`) and the fog word. A fifth word with its byte +8
/// clear zeroes `011fa2d4`. Without bit 0x4000 of the shader property's +0x20 and outside the
/// types 0x248 to 0x24b, `011fa2dc` becomes max(1, byte +0x1a) / 255 when bit 9 of the flags is
/// set, else `010af2f0` with bit 0 and 0.0 without; otherwise, for the types 0x248 to 0x24b,
/// 1.0 or 0.0 by the property's vtable +0xf4 (5, 0), and `011fa2d8` its +0x30.
#[allow(clippy::too_many_arguments)]
pub fn toggles(
    e: &mut Engine,
    this: Ptr,
    pass_type: u32,
    geometry: u32,
    property: Ptr,
    fog: u32,
    object: u32,
    flags_property: Ptr,
) {
    if !entry_enabled(e, PIXEL_ENTRIES + 4 * 4) {
        return;
    }
    let base = this.addr();
    let property = property.addr();
    if pass_type.wrapping_sub(0x1f7) <= 0x37 {
        e.call(
            UPDATE_TOGGLES_LAND,
            &args![base, geometry, 0u32, property, 0u32],
        );
    } else {
        let vtable = e.mem.u32(base);
        let index = u32::from(e.global::<u8>(FOG_INDEX));
        let fog_property = e.call(GET_FOG_PROPERTY, &args![index]).u32();
        let target = e.mem.u32(vtable + 0x154);
        e.call(target, &args![base, geometry, fog_property, fog]);
    }
    if object != 0 && e.mem.u8(object + 8) == 0 {
        e.set_global(TOGGLE_FIRST, 0u32);
    }
    let land = e.mem.u32(property + 0x20) & 0x4000 != 0;
    if !land && !(0x248..=0x24b).contains(&pass_type) {
        let flags_property = flags_property.addr();
        let flags = e.mem.u16(flags_property + 0x18);
        if (flags >> 9) & 1 != 0 {
            let byte = f64::from(e.mem.u8(flags_property + 0x1a));
            let mut value = x87_load32(x87_store32(byte));
            if value < e.global::<f64>(ONE) {
                value = 1.0;
            }
            let value = x87_load32(x87_store32(value));
            let value = x87_div(value, e.global::<f64>(BYTE_SCALE));
            e.set_global(TOGGLE_THIRD, x87_store32(value));
        } else if e.mem.u8(flags_property + 0x18) & 1 != 0 {
            let value = copy_float(e, TOGGLE_VALUE);
            e.set_global(TOGGLE_THIRD, value);
        } else {
            e.set_global(TOGGLE_THIRD, 0u32);
        }
        return;
    }
    if pass_type.wrapping_sub(0x248) > 3 {
        return;
    }
    let lit = e.vcall(property, 0xf4, &args![5u32, 0u32]).u32();
    let value = if lit != 0 { 1.0f32.to_bits() } else { 0 };
    e.set_global(TOGGLE_THIRD, value);
    let word = copy_float(e, property + 0x30);
    e.set_global(TOGGLE_SECOND, word);
}

/// The two floats `00b796f0` writes (`011fd944`, `011fd948`) and its float for bit 4
/// (`010177e8`).
const VALUE_FIRST: u32 = 0x011f_d944;
const VALUE_SECOND: u32 = 0x011f_d948;
const VALUE_CONSTANT: u32 = 0x0101_77e8;

// Typed from 00b796f0
/// Unnamed in the engine map, `thiscall` with one word, an object.
/// With pixel entry 16 enabled: `011fd944` takes its +0xe0, `011fd948` the float at `010177e8`
/// when bit 4 of its byte +0x24 is set, else 1.0.
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn pixel_entry_16(e: &mut Engine, _this: Ptr, object: Ptr) {
    if !entry_enabled(e, PIXEL_ENTRIES + 4 * 16) {
        return;
    }
    let object = object.addr();
    let value = copy_float(e, object + 0xe0);
    e.set_global(VALUE_FIRST, value);
    let value = if e.mem.u8(object + 0x24) & 0x10 != 0 {
        copy_float(e, VALUE_CONSTANT)
    } else {
        1.0f32.to_bits()
    };
    e.set_global(VALUE_SECOND, value);
}

/// The plane `00b79730` reads (`011f9604`, four floats), the vector it dots it with
/// (`011fa2a0`), the two distances it compares (`011f9614`, `011f9618`), its `-1.0`
/// (`01012054`) and what it writes (`011fd8e4`, four floats; `011fd8c4`, four floats).
const PLANE: u32 = 0x011f_9604;
const EYE_POSITION: u32 = 0x011f_a2a0;
const NEAR: u32 = 0x011f_9614;
const FAR: u32 = 0x011f_9618;
const MINUS_ONE: u32 = 0x0101_2054;
const PLANE_CONSTANT: u32 = 0x011f_d8e4;
const RANGE: u32 = 0x011f_d8c4;

// Typed from 00b79730
/// Unnamed in the engine map, no arguments. With pixel entry 19
/// enabled: `011fd8e4` takes the plane's first three floats (`011f9604`) and as the fourth the
/// dot product with `011fa2a0` ((y y' + x x') + z z', in that order) less the plane's fourth;
/// `011fd8c4` becomes (0, 0, -1.0, 0) when both distances (`011f9618`, `011f9614`) are 0, else
/// (far, far - near, 1.0, 1.0).
pub fn pixel_entry_19(e: &mut Engine) {
    if !entry_enabled(e, PIXEL_ENTRIES + 4 * 19) {
        return;
    }
    let vector: Vec<u32> = (0..3)
        .map(|index| copy_float(e, EYE_POSITION + 4 * index))
        .collect();
    let plane: Vec<u32> = (0..4)
        .map(|index| e.global::<u32>(PLANE + 4 * index))
        .collect();
    let y = x87_mul(x87_load32(plane[1]), x87_m32(vector[1]));
    let x = x87_mul(x87_load32(plane[0]), x87_m32(vector[0]));
    for index in 0..3u32 {
        let copy = x87_quiet_bits(plane[index as usize]);
        e.set_global(PLANE_CONSTANT + 4 * index, copy);
    }
    let sum = x87_add(y, x);
    let z = x87_mul(x87_load32(plane[2]), x87_m32(vector[2]));
    let dot = x87_store32(x87_add(sum, z));
    let distance = x87_store32(x87_sub(x87_load32(dot), x87_m32(plane[3])));
    e.set_global(PLANE_CONSTANT + 0xc, distance);
    let far = copy_float(e, FAR);
    let near = copy_float(e, NEAR);
    let far_value = x87_load32(far);
    let near_value = x87_load32(near);
    if far_value == 0.0 && near_value == 0.0 {
        e.set_global(RANGE, 0u32);
        e.set_global(RANGE + 4, 0u32);
        let minus_one = copy_float(e, MINUS_ONE);
        e.set_global(RANGE + 8, minus_one);
        e.set_global(RANGE + 0xc, 0u32);
        return;
    }
    e.set_global(RANGE, far);
    let span = x87_store32(x87_sub(far_value, near_value));
    e.set_global(RANGE + 4, span);
    e.set_global(RANGE + 8, 1.0f32.to_bits());
    e.set_global(RANGE + 0xc, 1.0f32.to_bits());
}

/// The four floats `00b798c0` writes (`011fa300`).
const VECTOR_LAST: u32 = 0x011f_a300;

// Typed from 00b798c0
/// Unnamed in the engine map, `thiscall` with one word, an object.
/// With pixel entry 20 enabled: `011fa300` takes its floats +0xf0, +0xf4, +0xfc, +0xf8 in that
/// order (each copied through the x87 stack).
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn pixel_entry_20(e: &mut Engine, _this: Ptr, object: Ptr) {
    if !entry_enabled(e, PIXEL_ENTRIES + 4 * 20) {
        return;
    }
    let object = object.addr();
    for (index, offset) in [0xf0u32, 0xf4, 0xfc, 0xf8].iter().enumerate() {
        let value = copy_float(e, object + offset);
        e.set_global(VECTOR_LAST + 4 * idx(index), value);
    }
}

/// The eye position the shaders use (`011fa2a0`, three floats), the two vectors
/// `SetupGeometryConstants_EyePos` writes (`011fd894`, `011fd8a4`), the point unitize
/// `004a0c10` (unnamed, `thiscall` on the point) and the D3DX import thunk
/// `D3DXVec3TransformCoord` (`00ee6df8`, `stdcall`).
const EYE: u32 = 0x011f_a2a0;
const EYE_VECTOR: u32 = 0x011f_d894;
const EYE_DIRECTION: u32 = 0x011f_d8a4;
const POINT_UNITIZE: u32 = 0x004a_0c10;
const VEC3_TRANSFORM_COORD: u32 = 0x00ee_6df8;

// Typed from 00b78ef0
/// `eye_pos` as the exe calls it (`stdcall`, eighteen words: the pass type, the object, the matrix).
pub fn eye_pos_words(e: &mut Engine, words: &[u32]) -> Ret {
    let mut matrix = [0u32; 16];
    matrix.copy_from_slice(&words[2..18]);
    eye_pos(e, words[0], Ptr::new(words[1]), matrix);
    Ret::default()
}

/// `ShadowLightShader::SetupGeometryConstants_EyePos` (engine map), `stdcall` with eighteen
/// words: the pass type, an object (position at +0x24, a float at +0x30) and a 4x4 matrix by
/// value. With vertex entry 9 or pixel entry 5 enabled: for pass types 0x244 to 0x24b the eye
/// (`011fa2a0`) less the object's position, unitized (`004a0c10`), goes to `011fd8a4` with 0, and
/// the eye less the camera (`011f474c`) to `011fd894` with 1.0; for other types the eye
/// transformed by the matrix (`D3DXVec3TransformCoord`) goes to `011fd894` with the object's
/// +0x30.
pub fn eye_pos(e: &mut Engine, pass_type: u32, object: Ptr, matrix: [u32; 16]) {
    if !entry_enabled(e, VERTEX_ENTRIES + 4 * 9) && !entry_enabled(e, PIXEL_ENTRIES + 4 * 5) {
        return;
    }
    let object = object.addr();
    if (0x244..=0x24b).contains(&pass_type) {
        let eye: Vec<u32> = (0..3).map(|index| copy_float(e, EYE + 4 * index)).collect();
        e.with_stack(0xc, |e, point| {
            let point = point.addr();
            for index in 0..3u32 {
                let value = x87_sub(
                    x87_load32(eye[index as usize]),
                    x87_m32(e.mem.u32(object + 0x24 + 4 * index)),
                );
                e.mem.set_u32(point + 4 * index, x87_store32(value));
            }
            e.call(POINT_UNITIZE, &args![point]);
            for index in 0..3u32 {
                let value = copy_float(e, point + 4 * index);
                e.set_global(EYE_DIRECTION + 4 * index, value);
            }
        });
        e.set_global(EYE_DIRECTION + 0xc, 0u32);
        let mut shifted = [0u32; 3];
        for (index, value) in shifted.iter_mut().enumerate() {
            let offset = 4 * idx(index);
            let eye = x87_load32(eye[index]);
            let camera = x87_m32(e.global::<u32>(CAMERA + offset));
            *value = x87_store32(x87_sub(eye, camera));
        }
        for (index, value) in shifted.iter().enumerate() {
            e.set_global(EYE_VECTOR + 4 * idx(index), x87_quiet_bits(*value));
        }
        e.set_global(EYE_VECTOR + 0xc, 1.0f32.to_bits());
        return;
    }
    e.with_stack(0x58, |e, frame| {
        let frame = frame.addr();
        let vector = frame;
        let transformed = frame + 0xc;
        let by_value = frame + 0x18;
        for (index, word) in matrix.iter().enumerate() {
            e.mem.set_u32(by_value + 4 * idx(index), *word);
        }
        for index in 0..3u32 {
            let value = copy_float(e, EYE + 4 * index);
            e.mem.set_u32(vector + 4 * index, value);
        }
        e.call(VEC3_TRANSFORM_COORD, &args![transformed, vector, by_value]);
        for index in 0..3u32 {
            let value = copy_float(e, transformed + 4 * index);
            e.set_global(EYE_VECTOR + 4 * index, value);
        }
    });
    let value = copy_float(e, object + 0x30);
    e.set_global(EYE_VECTOR + 0xc, value);
}

/// `BSShaderManager::GetAccumulator` (`00b4f5c0`, engine map; no arguments) and the matrix
/// `SetupGeometryConstants_ShadowProj` writes (`011fd968`, 16 floats).
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const SHADOW_PROJECTION: u32 = 0x011f_d968;

// Typed from 00b7b930
/// `ShadowLightShader::SetupGeometryConstants_ShadowProj` (engine map), `thiscall` with three
/// words: a block whose +0xc is an object with a register at +0x1c, a flag and a transform (a
/// 3x3 rotation, a translation and a scale, 13 floats). With vertex entry 3 enabled, the object
/// set with a register other than -1, and the accumulator's +0x194 set: `011fd968` becomes,
/// without the flag, the rows (rotation row with translation x, y, z in the fourth column) and
/// (0, 0, 0, scale), each float copied through the x87 stack; with it, the identity with the
/// camera position (`011f474c`) in the fourth column.
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn shadow_proj(e: &mut Engine, _this: Ptr, block: Ptr, flag: u32, transform: Ptr) {
    if !entry_enabled(e, VERTEX_ENTRIES + 4 * 3) {
        return;
    }
    let object = e.mem.u32(block.addr() + 0xc);
    if object == 0 || e.mem.u32(object + 0x1c) == 0xffff_ffff {
        return;
    }
    let accumulator = e.call(GET_ACCUMULATOR, &args![]).u32();
    if e.mem.u32(accumulator + 0x194) == 0 {
        return;
    }
    let mut matrix = [0u32; 16];
    if flag == 0 {
        let transform = transform.addr();
        let sources = [0u32, 1, 2, 9, 3, 4, 5, 10, 6, 7, 8, 11];
        for (index, source) in sources.iter().enumerate() {
            matrix[index] = copy_float(e, transform + 4 * source);
        }
        matrix[15] = copy_float(e, transform + 0x30);
    } else {
        let one = 1.0f32.to_bits();
        for index in [0usize, 5, 10, 15] {
            matrix[index] = one;
        }
        for (row, index) in [3usize, 7, 11].iter().enumerate() {
            let camera = e.global::<u32>(CAMERA + 4 * idx(row));
            matrix[*index] = x87_quiet_bits(camera);
        }
    }
    for (index, word) in matrix.iter().enumerate() {
        e.set_global(SHADOW_PROJECTION + 4 * idx(index), *word);
    }
}

/// `NiD3DUtility::GetD3DFromNi` (`00b799a0`, engine map; `cdecl`: matrix, transform), the D3DX
/// import thunks `D3DXMatrixMultiply` (`00ee6de0`), `D3DXMatrixTranspose` (`00ee6dd4`) and
/// `D3DXMatrixInverse` (`00ee6dda`) (engine map names; `stdcall`), and the matrix
/// `SetupGeometryConstants_ObjToCube` writes (`011fecc8`).
const GET_D3D_FROM_NI: u32 = 0x00b7_99a0;
const MATRIX_MULTIPLY: u32 = 0x00ee_6de0;
const MATRIX_TRANSPOSE: u32 = 0x00ee_6dd4;
const MATRIX_INVERSE: u32 = 0x00ee_6dda;
const OBJECT_TO_CUBE: u32 = 0x011f_ecc8;

// Typed from 00b7b730
/// `ShadowLightShader::SetupGeometryConstants_ObjToCube` (engine map), `thiscall` with four
/// stack words: a block (a byte at +9, an object at +0xc), the pass type, a flag and a
/// transform. With vertex entry 1 enabled: without the block's object, with the byte set and the
/// object's first word's byte +0xed set, or for the pass types 0x244 to 0x24b, 0x2d8 to 0x2ef,
/// 0xfe and 0x10b, `011fecc8` becomes the transpose of the renderer's (+0x18) matrix at +0x940
/// (copied to the stack first). Otherwise, with the rendering pass's +0xc set and the byte set:
/// an identity matrix (or the transform's, `GetD3DFromNi`, without the flag) gets the camera
/// (`011f474c`) added to its translation, is multiplied by the first light's matrix (+0x10 of
/// the first word of the rendering pass's +0xc), transposed in place and copied to `011fecc8`.
pub fn obj_to_cube(
    e: &mut Engine,
    this: Ptr,
    block: Ptr,
    pass_type: u32,
    flag: u32,
    transform: u32,
) {
    if !entry_enabled(e, VERTEX_ENTRIES + 4) {
        return;
    }
    let block = block.addr();
    let object = e.mem.u32(block + 0xc);
    let byte = e.mem.u8(block + 9);
    let mut direct = object == 0;
    if !direct && byte != 0 && e.mem.u32(object) != 0 {
        let first = e.mem.u32(object);
        direct = e.mem.u8(first + 0xed) != 0;
    }
    if !direct {
        direct = (0x244..=0x24b).contains(&pass_type)
            || (0x2d8..=0x2ef).contains(&pass_type)
            || pass_type == 0xfe
            || pass_type == 0x10b;
    }
    e.with_stack(0x100, |e, frame| {
        let frame = frame.addr();
        let first = frame;
        let second = frame + 0x40;
        let product = frame + 0x80;
        if direct {
            let renderer = e.mem.u32(this.addr() + 0x18);
            let bytes = e.mem.bytes(renderer + 0x940, 0x40);
            e.mem.write(product, &bytes);
            e.call(MATRIX_TRANSPOSE, &args![OBJECT_TO_CUBE, product]);
            return;
        }
        let rendering = e.global::<u32>(RENDERING_PASS);
        if e.mem.u32(rendering + 0xc) == 0 || byte == 0 {
            return;
        }
        for matrix in [first, second] {
            for index in 0..16u32 {
                let value = if index % 5 == 0 { 1.0f32 } else { 0.0 };
                e.mem.set_u32(matrix + 4 * index, value.to_bits());
            }
        }
        if flag == 0 {
            e.call(GET_D3D_FROM_NI, &args![first, transform]);
        }
        for index in 0..3u32 {
            let slot = first + 0x30 + 4 * index;
            let camera = x87_load32(e.global::<u32>(CAMERA + 4 * index));
            let value = x87_add(camera, x87_m32(e.mem.u32(slot)));
            e.mem.set_u32(slot, x87_store32(value));
        }
        let lights = e.mem.u32(rendering + 0xc);
        let light = e.mem.u32(lights);
        e.call(MATRIX_MULTIPLY, &args![product, first, light + 0x10]);
        let bytes = e.mem.bytes(product, 0x40);
        e.mem.write(second, &bytes);
        e.call(MATRIX_TRANSPOSE, &args![second, second]);
        e.call(MEMCPY, &args![OBJECT_TO_CUBE, second, 0x40u32]);
    });
}

/// The camera object (`011f91c8`; its position at +0x1e4) and the light vectors the light
/// loops write and upload (`011fd9a8`, 0x10 bytes per light).
const CAMERA_OBJECT: u32 = 0x011f_91c8;
const LIGHT_VECTORS: u32 = 0x011f_d9a8;

/// The D3DX import thunks the clip planes use (unnamed in the engine map; `stdcall`):
/// `00ee6dce` (plane normalize: out, plane) and `00ee6dc8` (plane transform: out, plane,
/// matrix), as the decompiler names their imports `D3DXPlaneNormalize` and
/// `D3DXPlaneTransform`.
const PLANE_NORMALIZE: u32 = 0x00ee_6dce;
const PLANE_TRANSFORM: u32 = 0x00ee_6dc8;

/// The six clip planes at `planes` set on the device (renderer `011f4748` +0x288, read for each;
/// its vtable +0xdc with the index and the plane).
fn set_clip_planes(e: &mut Engine, planes: u32) {
    for index in 0..6u32 {
        let renderer = e.global::<u32>(RENDERER);
        let device = e.mem.u32(renderer + 0x288);
        e.vcall(device, 0xdc, &args![index, planes + 0x10 * index]);
    }
}

// Typed from 00b7be00
/// `ShadowLightShader::SetupGeometryConstants_SimpleShadowClipPlanes` (engine map), `thiscall`
/// with one stack word, the pass type; only the types 0x231 to 0x234 do anything. The first
/// light (the first word of the rendering pass's +0xc) keeps transformed clip planes at +0x1a8
/// and a byte at +0x208 that says they are made; when it is set the device gets them. Else: the
/// renderer's (+0x18) view matrix (+0x980) on the stack gets the translation -(camera . column)
/// for its three columns (the camera `011f474c`, x87 in the exe's order), is multiplied by the
/// renderer's matrix at +0x9c0, inverted and transposed; each of the light's six planes (+0x144;
/// the fourth word negated) is normalized and transformed by it; the device gets the six, they
/// are copied to +0x1a8 and the byte is set.
pub fn simple_shadow_clip_planes(e: &mut Engine, this: Ptr, pass_type: u32) {
    if pass_type.wrapping_sub(0x231) > 3 {
        return;
    }
    let light = rendering_object(e);
    if e.mem.u8(light + 0x208) != 0 {
        set_clip_planes(e, light + 0x1a8);
        return;
    }
    let renderer = e.mem.u32(this.addr() + 0x18);
    e.with_stack(0x1a0, |e, frame| {
        let frame = frame.addr();
        let view = frame;
        let product = frame + 0x40;
        let inverse = frame + 0x80;
        let planes = frame + 0xc0;
        let plane = frame + 0x120;
        let moved = frame + 0x130;
        let bytes = e.mem.bytes(renderer + 0x980, 0x40);
        e.mem.write(view, &bytes);
        let m = |e: &Engine, index: u32| e.mem.u32(view + 4 * index);
        let x = x87_load32(e.global::<u32>(CAMERA));
        let y = x87_load32(e.global::<u32>(CAMERA + 4));
        let z = x87_load32(e.global::<u32>(CAMERA + 8));
        let first = x87_add(x87_mul(y, x87_m32(m(e, 4))), x87_mul(x, x87_m32(m(e, 0))));
        let first = x87_store32(x87_add(first, x87_mul(z, x87_m32(m(e, 8)))));
        e.mem.set_u32(view + 0x30, x87_store32(-x87_load32(first)));
        let second = x87_add(x87_mul(y, x87_m32(m(e, 5))), x87_mul(x, x87_m32(m(e, 1))));
        let second = x87_store32(x87_add(second, x87_mul(z, x87_m32(m(e, 9)))));
        e.mem.set_u32(view + 0x34, x87_store32(-x87_load32(second)));
        let y_term = x87_mul(y, x87_load32(m(e, 6)));
        let x_term = x87_mul(x, x87_m32(m(e, 2)));
        let third = x87_add(y_term, x_term);
        let z_term = x87_mul(z, x87_m32(m(e, 10)));
        let third = x87_store32(x87_add(third, z_term));
        e.mem.set_u32(view + 0x38, x87_store32(-x87_load32(third)));
        let bytes = e.mem.bytes(renderer + 0x9c0, 0x40);
        e.mem.write(moved, &bytes);
        e.call(MATRIX_MULTIPLY, &args![product, view, moved]);
        e.call(MATRIX_INVERSE, &args![inverse, 0u32, product]);
        e.call(MATRIX_TRANSPOSE, &args![product, inverse]);
        for index in 0..6u32 {
            let source = light + 0x144 + 0x10 * index;
            for word in 0..3u32 {
                let value = copy_float(e, source + 4 * word);
                e.mem.set_u32(plane + 4 * word, value);
            }
            let distance = x87_store32(-x87_load32(e.mem.u32(source + 0xc)));
            e.mem.set_u32(plane + 0xc, distance);
            e.call(PLANE_NORMALIZE, &args![plane, plane]);
            let out = frame + 0x170;
            e.call(PLANE_TRANSFORM, &args![out, plane, product]);
            for word in 0..4u32 {
                let value = copy_float(e, out + 4 * word);
                e.mem.set_u32(planes + 0x10 * index + 4 * word, value);
            }
        }
        set_clip_planes(e, planes);
        e.call(MEMCPY, &args![light + 0x1a8, planes, 0x60u32]);
    });
    e.mem.set_u8(light + 0x208, 1);
}

/// The shader mode the shaders switch on (`011f91b8`), the shader manager's object `00b4f5d0`
/// answers (unnamed, no arguments; its colour at +0x154), the per-light set-up `00b70820`
/// (unnamed, `cdecl`: index, light, a float), `BSShaderLightingProperty::GetFirstActiveLight`
/// (`00b70590`, engine map; `thiscall`, one word), the light positions and directions the
/// set-up leaves (`011fa170` and `011fa1f0`, 0x10 bytes per light), `0.5` (`01016248`), the
/// HDR light scale (`011f9184`) and the colour rows `SetupGeometryConstants_Lights` writes
/// (`011fa0d0`).
const SHADER_MODE: u32 = 0x011f_91b8;
const MANAGER_OBJECT: u32 = 0x00b4_f5d0;
const LIGHT_SETUP: u32 = 0x00b7_0820;
const GET_FIRST_ACTIVE_LIGHT: u32 = 0x00b7_0590;
const LIGHT_POSITIONS: u32 = 0x011f_a170;
const LIGHT_DIRECTIONS: u32 = 0x011f_a1f0;
const HALF: u32 = 0x0101_6248;
const HDR_LIGHT_SCALE: u32 = 0x011f_9184;

// Typed from 00b78a90
/// `lights` as the exe calls it (`stdcall`, 21 words: the property, the rendering object, the
/// matrix, the scale object, the pass type, the flag).
pub fn lights_words(e: &mut Engine, words: &[u32]) -> Ret {
    let mut matrix = [0u32; 16];
    matrix.copy_from_slice(&words[2..18]);
    lights(
        e,
        Ptr::new(words[0]),
        Ptr::new(words[1]),
        matrix,
        Ptr::new(words[18]),
        words[19],
        words[20],
    );
    Ret::default()
}

/// `ShadowLightShader::SetupGeometryConstants_Lights` (engine map), `stdcall` with 21 words: the
/// shader property (a float at +0x6c, flags at +0x20), the rendering object (type +4, light count
/// +9, lights +0xc), a 4x4 matrix by value, an object with a scale at +0x30, the pass type and a
/// flag. In shader mode 0xf (`011f91b8`) row 0 of `011fa0c0` takes the manager object's colour
/// (`00b4f5d0` +0x154) and that is all. Else the register counts of vertex entry 6 (1 for types
/// 0x10 to 0x13, else the light count) and pixel entries 2 (the count) and 7 (count - 1) are set
/// when enabled. For each light up to 10, after its set-up (`00b70820` with the property's
/// float): for types 0x10 to 0x13 `011fd9a8` row i takes the position row and as its fourth word
/// the direction row's first; else, for a light with its byte +0xf4 set (a point light), the
/// position plus the camera object's (`011f91c8` +0x1e4) transformed by the matrix, its fourth
/// word the direction row's first, scaled by the object's +0x30 for types 0x231 to 0x233 or the
/// fourth divided by it otherwise (neither with the flag); for another light the negated
/// direction transformed as a normal and normalized, with the direction row's fourth word. With
/// HDR (`011f941e`), bit 0x400 of the property and a first light that is not a point light, row
/// 1 of the colours (`011fa0d0`) is multiplied by `011f9184`. For pass type 0x24b without an
/// active point light (`GetFirstActiveLight`) row 1 becomes (0.5, 0.5, 0.5, 1.0) in shader mode 10,
/// else a copy of row 0.
pub fn lights(
    e: &mut Engine,
    property: Ptr,
    rendering: Ptr,
    matrix: [u32; 16],
    scale_object: Ptr,
    pass_type: u32,
    flag: u32,
) {
    let property = property.addr();
    let rendering = rendering.addr();
    let light_float = copy_float(e, property + 0x6c);
    if e.global::<u32>(SHADER_MODE) == 0xf {
        let object = e.call(MANAGER_OBJECT, &args![]).u32();
        for index in 0..4u32 {
            let word = e.mem.u32(object + 0x154 + 4 * index);
            e.set_global(ROWS + 4 * index, word);
        }
        return;
    }
    let count = u32::from(e.mem.u8(rendering + 9));
    let kind = e.mem.u16(rendering + 4);
    let entry = e.global::<u32>(VERTEX_ENTRIES + 4 * 6);
    if e.mem.u8(entry + 8) != 0 {
        let registers = if (0x10..=0x13).contains(&kind) {
            1
        } else {
            count
        };
        e.mem.set_u32(entry + 0x20, registers);
    }
    let entry = e.global::<u32>(PIXEL_ENTRIES + 4 * 2);
    if e.mem.u8(entry + 8) != 0 {
        e.mem.set_u32(entry + 0x20, count);
    }
    let entry = e.global::<u32>(PIXEL_ENTRIES + 4 * 7);
    if e.mem.u8(entry + 8) != 0 {
        e.mem.set_u32(entry + 0x20, count.wrapping_sub(1));
    }
    let mut index: u8 = 0;
    if e.mem.u8(rendering + 9) > 0 {
        e.with_stack(0x80, |e, frame| {
            let frame = frame.addr();
            let by_value = frame;
            let vector = frame + 0x40;
            let out = frame + 0x50;
            for (word, value) in matrix.iter().enumerate() {
                e.mem.set_u32(by_value + 4 * idx(word), *value);
            }
            loop {
                if index >= 10 {
                    break;
                }
                let i = u32::from(index);
                let lights = e.mem.u32(rendering + 0xc);
                let light = e.mem.u32(lights.wrapping_add(i * 4));
                e.call(LIGHT_SETUP, &args![i, light, x87_quiet_bits(light_float)]);
                let row = LIGHT_VECTORS + 0x10 * i;
                let position = LIGHT_POSITIONS + 0x10 * i;
                let direction = LIGHT_DIRECTIONS + 0x10 * i;
                let kind = e.mem.u16(rendering + 4);
                if (0x10..=0x13).contains(&kind) {
                    for word in 0..4u32 {
                        let value = e.mem.u32(position + 4 * word);
                        e.mem.set_u32(row + 4 * word, value);
                    }
                    let fourth = copy_float(e, direction);
                    e.mem.set_u32(row + 0xc, fourth);
                } else if light != 0 {
                    if e.mem.u8(light + 0xf4) != 0 {
                        for word in 0..3u32 {
                            let value = copy_float(e, position + 4 * word);
                            e.mem.set_u32(vector + 4 * word, value);
                        }
                        let camera = e.global::<u32>(CAMERA_OBJECT);
                        for word in 0..3u32 {
                            let offset = x87_load32(e.mem.u32(camera + 0x1e4 + 4 * word));
                            let value = x87_add(offset, x87_m32(e.mem.u32(vector + 4 * word)));
                            e.mem.set_u32(vector + 4 * word, x87_store32(value));
                        }
                        e.call(VEC3_TRANSFORM_COORD, &args![out, vector, by_value]);
                        let mut fourth = copy_float(e, direction);
                        if flag == 0 {
                            let scale_object = scale_object.addr();
                            let kind = e.mem.u16(rendering + 4);
                            if (0x231..=0x233).contains(&kind) {
                                let scale_bits = copy_float(e, scale_object + 0x30);
                                let scale = x87_load32(scale_bits);
                                for word in 0..3u32 {
                                    let slot = out + 4 * word;
                                    let value = x87_mul(scale, x87_m32(e.mem.u32(slot)));
                                    e.mem.set_u32(slot, x87_store32(value));
                                }
                            } else {
                                let value = x87_div(
                                    x87_load32(fourth),
                                    x87_m32(e.mem.u32(scale_object + 0x30)),
                                );
                                fourth = x87_store32(value);
                            }
                        }
                        for word in 0..3u32 {
                            let value = copy_float(e, out + 4 * word);
                            e.mem.set_u32(row + 4 * word, value);
                        }
                        e.mem.set_u32(row + 0xc, x87_quiet_bits(fourth));
                    } else {
                        for word in 0..3u32 {
                            let value = x87_load32(e.mem.u32(direction + 4 * word));
                            e.mem.set_u32(vector + 4 * word, x87_store32(-value));
                        }
                        e.call(VEC3_TRANSFORM_NORMAL, &args![out, vector, by_value]);
                        e.call(VEC3_NORMALIZE, &args![vector, out]);
                        for word in 0..3u32 {
                            let value = copy_float(e, vector + 4 * word);
                            e.mem.set_u32(row + 4 * word, value);
                        }
                        let fourth = copy_float(e, direction + 0xc);
                        e.mem.set_u32(row + 0xc, fourth);
                    }
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u8(rendering + 9) {
                    break;
                }
            }
        });
    }
    let mut scale_colour = e.global::<u8>(HDR) != 0
        && e.mem.u32(property + 0x20) & 0x400 != 0
        && e.mem.u8(rendering + 9) != 0;
    if scale_colour {
        let lights = e.mem.u32(rendering + 0xc);
        let first = e.mem.u32(lights);
        scale_colour = e.mem.u8(first + 0xf4) == 0;
    }
    if scale_colour {
        let scale = x87_load32(e.global::<u32>(HDR_LIGHT_SCALE));
        let red = x87_load32(e.global::<u32>(COLOUR));
        let green = e.global::<u32>(COLOUR + 4);
        let blue = e.global::<u32>(COLOUR + 8);
        let alpha = e.global::<u32>(COLOUR + 0xc);
        e.set_global(COLOUR, x87_store32(x87_mul(red, scale)));
        let value = x87_store32(x87_mul(x87_load32(green), scale));
        e.set_global(COLOUR + 4, value);
        let value = x87_store32(x87_mul(x87_load32(blue), scale));
        e.set_global(COLOUR + 8, value);
        let value = x87_store32(x87_mul(scale, x87_m32(alpha)));
        e.set_global(COLOUR + 0xc, value);
    }
    if pass_type != 0x24b {
        return;
    }
    let mode = e.global::<u32>(SHADER_MODE);
    e.with_stack(4, |e, slot| {
        let light = e.call(GET_FIRST_ACTIVE_LIGHT, &args![property, slot]).u32();
        if light != 0 && e.mem.u8(light + 0xf4) != 0 {
            return;
        }
        if mode == 10 {
            let half = copy_float(e, HALF);
            for offset in [0u32, 4, 8] {
                e.set_global(COLOUR + offset, half);
            }
            e.set_global(COLOUR + 0xc, 1.0f32.to_bits());
        } else {
            for offset in [0u32, 4, 8, 0xc] {
                let word = e.global::<u32>(ROWS + offset);
                e.set_global(COLOUR + offset, word);
            }
        }
    });
}

/// The ambient helpers (`stdcall`, one word: the shader's word): `00c03180` with pixel entry 3
/// enabled outside types 0xc3 to 0xd6, else `00c03010` (both unnamed).
const AMBIENT_SET_UP: u32 = 0x00c0_3180;
const ACCUMULATE_LIGHT_COLOR: u32 = 0x00c0_3010;
/// The two floats `SetupGeometryConstants_AmbientEmittance` sets from the light count for
/// types 0xc3 to 0xd6 (`011fd95c`, `011fa27c`), the byte that makes the ambient row white
/// (`011f91a7`), `BSShaderLightingProperty::GetFirstActiveNonShadowLight` (`00b70600`, engine
/// map; `thiscall`, one word), the three luminance weights (`double`s at `010ae8c0`, `010ae8b8`,
/// `010ae8b0` for the second, first and third channel) and the helpers it calls (unnamed):
/// `00439ef0` (`thiscall` on a point: out, other point), `00633b40` (`thiscall` on a colour:
/// out, a float), `00457990` (`thiscall` on a point, the result in ST0), `0068b050` (`cdecl`: a
/// `double` and a float, the result in ST0), `00b719c0` (`cdecl`: two `double`s, the result in
/// ST0) and `00b719e0` (`thiscall` on a colour, the result in ST0).
const LIGHT_COUNT: u32 = 0x011f_d95c;
const LIGHT_COUNT_CONSTANT: u32 = 0x011f_a27c;
const WHITE_AMBIENT: u32 = 0x011f_91a7;
const GET_FIRST_ACTIVE_NON_SHADOW_LIGHT: u32 = 0x00b7_0600;
const WEIGHT_SECOND: u32 = 0x010a_e8c0;
const WEIGHT_FIRST: u32 = 0x010a_e8b8;
const WEIGHT_THIRD: u32 = 0x010a_e8b0;
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const COLOUR_SCALE: u32 = 0x0063_3b40;
const POINT_LENGTH: u32 = 0x0045_7990;
const FLOAT_HELPER: u32 = 0x0068_b050;
const DOUBLE_HELPER: u32 = 0x00b7_19c0;
const COLOUR_LUMINANCE: u32 = 0x00b7_19e0;

// Typed from 00b7ba70
/// `ShadowLightShader::SetupGeometryConstants_AmbientEmittance` (engine map), `thiscall` with
/// four words: the shader property (flags at +0x20 and +0x24, a float at +0x28, its vtable
/// +0x110 answering a float), the rendering object (type +4, light count +9), a block whose
/// +0x9c is a property with flags at +0x18, and a word for the ambient helpers. Row 0 of
/// `011fa0c0` keeps its colour (copied) with the property's +0x28 as the fourth word; the ambient
/// helper runs (`00c03180` or `00c03010`); types 0xc3 to 0xd6 put the light count into
/// `011fd95c` and `011fa27c`; `011f91a7` makes the row white. With bit 0x8000 of +0x24, without
/// bit 0x20000 of +0x20 and for rendering types 0x244 to 0x24b, the fourth word is multiplied by
/// the larger of the camera light luminance and, for the first active non-shadow light, its
/// luminance scaled by 1 - min(1, max(0, distance / radius)). In shader mode 8 the row becomes
/// (0, 0, 1 or 0 by the block property bits 0 and 9, 1 or 0 by bit 0x20000 of +0x24). For pass
/// types 0x244 to 0x24b outside mode 10 the fourth word is multiplied by the property vtable
/// +0x110. Every x87 step, copy and comparison follows the exe.
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn ambient_emittance(
    e: &mut Engine,
    _this: Ptr,
    property: Ptr,
    rendering: Ptr,
    block: Ptr,
    word: u32,
) {
    let property = property.addr();
    let rendering = rendering.addr();
    let current = e.global::<u32>(RENDERING_PASS);
    let colour: Vec<u32> = (0..3)
        .map(|index| copy_float(e, ROWS + 4 * index))
        .collect();
    let alpha = copy_float(e, property + 0x28);
    for (index, value) in colour.iter().enumerate() {
        e.set_global(ROWS + 4 * idx(index), *value);
    }
    e.set_global(ROWS + 0xc, alpha);
    let kind = e.mem.u16(rendering + 4);
    let helper = if entry_enabled(e, PIXEL_ENTRIES + 4 * 3) && !(0xc3..=0xd6).contains(&kind) {
        AMBIENT_SET_UP
    } else {
        ACCUMULATE_LIGHT_COLOR
    };
    if helper == ACCUMULATE_LIGHT_COLOR {
        typed_call!(
            FAMILY,
            0x00c0_3010,
            accumulator::accumulate_light_color(e, Ptr::new(word))
        );
    } else {
        e.call(helper, &args![word]);
    }
    let kind = e.mem.u16(rendering + 4);
    if (0xc3..=0xd6).contains(&kind) {
        let count = x87_store32(f64::from(e.mem.u8(rendering + 9)));
        e.set_global(LIGHT_COUNT, count);
        let count = x87_store32(f64::from(e.mem.u8(rendering + 9)));
        e.set_global(LIGHT_COUNT_CONSTANT, count);
    }
    if e.global::<u8>(WHITE_AMBIENT) != 0 {
        for offset in [0u32, 4, 8, 0xc] {
            e.set_global(ROWS + offset, 1.0f32.to_bits());
        }
    }
    let current_kind = e.mem.u16(current + 4);
    if e.mem.u32(property + 0x24) & 0x8000 != 0
        && e.mem.u32(property + 0x20) & 0x20000 == 0
        && (0x244..=0x24b).contains(&current_kind)
    {
        e.with_stack(0x24, |e, frame| {
            let frame = frame.addr();
            let slot = frame;
            let difference = frame + 4;
            let scaled = frame + 0x10;
            let light = e
                .call(GET_FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![property, slot])
                .u32();
            let camera = e.global::<u32>(CAMERA_OBJECT);
            let camera_light = e.mem.u32(camera + 0xe0);
            let values = e.mem.u32(camera_light + 0xf8);
            let second = x87_mul(
                x87_load32(e.mem.u32(values + 0xd8)),
                e.global::<f64>(WEIGHT_SECOND),
            );
            let first = x87_mul(
                x87_load32(e.mem.u32(values + 0xd4)),
                e.global::<f64>(WEIGHT_FIRST),
            );
            let sum = x87_add(second, first);
            let third = x87_mul(
                x87_load32(e.mem.u32(values + 0xdc)),
                e.global::<f64>(WEIGHT_THIRD),
            );
            let luminance = x87_store32(x87_add(sum, third));
            let mut light_part = 0.0f32.to_bits();
            if light != 0 {
                let current = e.global::<u32>(RENDERING_PASS);
                let first_word = e.mem.u32(current);
                let mut position = e.mem.u32(first_word + 0x20);
                if position == 0 {
                    position = DEFAULT_POSITION;
                }
                let values = e.mem.u32(light + 0xf8);
                e.call(
                    POINT_DIFFERENCE,
                    &args![values + 0x8c, difference, position],
                );
                let strength = copy_float(e, values + 0xc4);
                e.call(COLOUR_SCALE, &args![values + 0xd4, scaled, strength]);
                let length = e.call(POINT_LENGTH, &args![difference]).f64();
                let ratio = x87_store32(x87_div(length, x87_m32(e.mem.u32(values + 0xe0))));
                let raised = e
                    .call(FLOAT_HELPER, &args![0.0f64, x87_quiet_bits(ratio)])
                    .f64();
                let limited = e.call(DOUBLE_HELPER, &args![1.0f64, raised]).f64();
                let limited = x87_store32(limited);
                let falloff = x87_store32(x87_sub(1.0, x87_load32(limited)));
                let falloff = x87_quiet_bits(falloff);
                let brightness = e.call(COLOUR_LUMINANCE, &args![scaled]).f64();
                light_part = x87_store32(x87_mul(brightness, x87_m32(falloff)));
            }
            let light_value = x87_load32(light_part);
            let camera_value = x87_load32(luminance);
            let chosen = if camera_value < light_value {
                light_value
            } else {
                camera_value
            };
            let chosen = x87_store32(chosen);
            let fourth = e.global::<u32>(ROWS + 0xc);
            let value = x87_mul(x87_load32(chosen), x87_m32(fourth));
            e.set_global(ROWS + 0xc, x87_store32(value));
        });
    }
    let mode = e.global::<u32>(SHADER_MODE);
    if mode == 8 {
        let one = 1.0f32.to_bits();
        let alpha = if e.mem.u32(property + 0x24) & 0x20000 != 0 {
            0
        } else {
            one
        };
        let flags_property = e.mem.u32(block.addr() + 0x9c);
        let flags = e.mem.u16(flags_property + 0x18);
        let blue = if flags & 1 != 0 || (flags >> 9) & 1 != 0 {
            0
        } else {
            one
        };
        e.set_global(ROWS, 0u32);
        e.set_global(ROWS + 4, 0u32);
        e.set_global(ROWS + 8, blue);
        e.set_global(ROWS + 0xc, alpha);
    }
    let pass_type = e.global::<u32>(RENDERING_PASS_TYPE);
    if pass_type.wrapping_sub(0x244) > 7 || mode == 10 {
        return;
    }
    let factor = e.vcall(property, 0x110, &args![]).f64();
    let fourth = e.global::<u32>(ROWS + 0xc);
    let value = x87_mul(factor, x87_m32(fourth));
    e.set_global(ROWS + 0xc, x87_store32(value));
}

/// The model-to-world helper `00c4c2d0` (unnamed, `cdecl`: transform, skin, matrix out) and the
/// two pass set-ups `SetupGeometryConstants` runs for types 4 and 5 (`00b7ad00`) and 0x250 and
/// 0x251 (`00b7ce90`) (unit `thiscall`: geometry, property, rendering
/// object).
const MODEL_MATRIX: u32 = 0x00c4_c2d0;
const PASS_SETUP_SHADOW: u32 = 0x00b7_ad00;
const PASS_SETUP_SPECIAL: u32 = 0x00b7_ce90;

/// A constant map's `SetShaderConstants` (its vtable +0x78, `00e826d0`).
const SLOT_SET_SHADER_CONSTANTS: u32 = 0x78;
const SET_SHADER_CONSTANTS: u32 = 0x00e8_26d0;

// Typed from 00b7dab0
/// `ShadowLightShader::SetupGeometryConstants` (engine map), `thiscall` with one stack word, the
/// block (its first word a property, +8 a word, +0xc the shader property, +0x10 another
/// property). With the rendering object (`011f91e0`), its geometry (first word; transform at
/// +0x68, position at +0x20 or `011f4288`, +0xb8 and skin +0xbc) and the pass type (`011f91e4`):
/// pass types 6 and 8 with bit 0x40000 of the shader property and bit 0 of the property set
/// `SetZWriteEnable(0, 1)` and `00b98380(0, 1)`; the model matrix is made (`00c4c2d0`) and
/// without a skin uploaded (`00b7c750`). Types 0x250 and 0x251 run `00b7ce90`, types 4 and 5
/// `00b7ad00`; every other type the constant set-ups in the exe's order (lights, ambient,
/// object-to-cube, shadow projection, eye, `00b79050`, `00b790d0`, `00b79180`, `00b79100`,
/// `00b791f0`, `00b79410`, toggles, `00b796f0`, `00b79730`, clip planes, `00b798c0`). The pixel
/// (+0x30) and vertex (+0x34) constant maps then set their constants (`SetShaderConstants`,
/// vtable +0x78) for the pass's pixel (+0x44) and vertex (+0x5c) program; with the block's +0x10
/// property's bit 0, the shader's +0x1c gets it through its vtable +0x24.
pub fn setup_geometry_constants(e: &mut Engine, this: Ptr, block: Ptr) {
    let base = this.addr();
    let block = block.addr();
    let rendering = e.global::<u32>(RENDERING_PASS);
    let geometry = e.mem.u32(rendering);
    let pass_type = e.global::<u32>(RENDERING_PASS_TYPE);
    let mut position = e.mem.u32(geometry + 0x20);
    if position == 0 {
        position = DEFAULT_POSITION;
    }
    let data = e.mem.u32(geometry + 0xb8);
    let geometry_data = e.mem.u32(data + 0x34);
    let fog_word = e.mem.u32(block + 8);
    let skin = e.mem.u32(geometry + 0xbc);
    let property = e.mem.u32(block + 0xc);
    let first_property = e.mem.u32(block);
    if (pass_type == 6 || pass_type == 8)
        && e.mem.u32(property + 0x20) & 0x40000 != 0
        && e.mem.u8(first_property + 0x18) & 1 != 0
    {
        typed_call!(FAMILY, 0x00b9_7e30, state::set_z_write_enable(e, 0u8, 1u32));
        typed_call!(FAMILY, 0x00b9_8380, state::set_color_write(e, 0u32, 1u32));
    }
    let transform = geometry + 0x68;
    e.with_stack(0x40, |e, matrix| {
        let matrix_addr = matrix.addr();
        e.call(MODEL_MATRIX, &args![transform, skin, matrix_addr]);
        if skin == 0 {
            typed_call!(
                FAMILY,
                0x00b7_c750,
                accumulator::set_world_view_constants(e, base, geometry, transform)
            );
        }
        let mut words = [0u32; 16];
        for (index, word) in words.iter_mut().enumerate() {
            *word = e.mem.u32(matrix_addr + 4 * idx(index));
        }
        if pass_type == 0x250 || pass_type == 0x251 {
            e.call(
                PASS_SETUP_SPECIAL,
                &args![base, geometry, property, rendering],
            );
        } else if pass_type == 4 || pass_type == 5 {
            e.call(
                PASS_SETUP_SHADOW,
                &args![base, geometry, property, rendering],
            );
        } else {
            let p = Ptr::new(property);
            let r = Ptr::new(rendering);
            lights(e, p, r, words, Ptr::new(transform), pass_type, skin);
            ambient_emittance(e, this, p, r, Ptr::new(geometry), fog_word);
            obj_to_cube(e, this, r, pass_type, skin, transform);
            shadow_proj(e, this, Ptr::new(block), skin, Ptr::new(transform));
            eye_pos(e, pass_type, Ptr::new(transform), words);
            vertex_entry_12(e, this, skin);
            vertex_entry_15(e, this, p);
            vertex_entry_17(e, this, Ptr::new(geometry));
            vertex_entry_16(e, this, geometry);
            vertex_entry_21(e, this, pass_type, p, Ptr::new(geometry));
            pixel_entry_2(e, this, pass_type, r);
            toggles(
                e,
                this,
                pass_type,
                geometry,
                p,
                fog_word,
                rendering,
                Ptr::new(first_property),
            );
            pixel_entry_16(e, this, p);
            pixel_entry_19(e);
            simple_shadow_clip_planes(e, this, pass_type);
            pixel_entry_20(e, this, p);
        }
    });
    for (map_offset, program_offset) in [(0x30u32, 0x44u32), (0x34, 0x5c)] {
        let map = e.mem.u32(base + map_offset);
        if map == 0 {
            continue;
        }
        let pass = e.global::<u32>(CURRENT_PASS);
        let program = e.mem.u32(pass + program_offset);
        if program == 0 {
            continue;
        }
        if crate::slot_target(e, map, SLOT_SET_SHADER_CONSTANTS) == SET_SHADER_CONSTANTS {
            let _: bool = typed_call!(
                FAMILY,
                0x00e8_26d0,
                constants::set_shader_constants(
                    e,
                    Ptr::new(map),
                    program,
                    geometry,
                    skin,
                    0,
                    geometry_data,
                    block,
                    0,
                    transform,
                    position,
                    0,
                    1
                )
            );
        } else {
            let words = args![
                program,
                geometry,
                skin,
                0u32,
                geometry_data,
                block,
                0u32,
                transform,
                position,
                0u32,
                1u32
            ];
            e.vcall(map, SLOT_SET_SHADER_CONSTANTS, &words);
        }
    }
    let other = e.mem.u32(block + 0x10);
    if other != 0 && e.mem.u8(other + 0x18) & 1 != 0 {
        let object = e.mem.u32(base + 0x1c);
        e.vcall(object, 0x24, &args![other]);
    }
}
