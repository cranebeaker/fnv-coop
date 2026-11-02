//! `BSShader/shadowlightshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    ni_pointer_assign, ni_pointer_clear, set_up_sources, ShaderSource, COMPILE_PIXEL_SHADER,
    COMPILE_VERTEX_SHADER, FORMAT_00406D00, MAKE_PATH,
};

/// The shader's vertex and pixel constant-map entry pointers (`ShadowLightShader::
/// pVertexConstantMapEntry` / `pPixelConstantMapEntry` (Xbox PDB), 22 words each from
/// `011febd8` and `011fec30` on PC).
pub(crate) const VERTEX_ENTRIES_011FEBD8: u32 = 0x011f_ebd8;
pub(crate) const PIXEL_ENTRIES_011FEC30: u32 = 0x011f_ec30;

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
const MATRIX_011FEC88: u32 = 0x011f_ec88;

// Translated from 00b79050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with one word. With vertex
/// entry 12 enabled and the word not null: an identity matrix built on the stack (zeros from
/// `FLDZ`, ones from `FLD1`) is copied to `011fec88` (`memcpy`, 0x40 bytes).
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn fn_00b79050(e: &mut Engine, _this: Ptr, object: u32) {
    if !entry_enabled(e, VERTEX_ENTRIES_011FEBD8 + 4 * 12) || object == 0 {
        return;
    }
    e.with_stack(0x40, |e, matrix| {
        let matrix = matrix.addr();
        for index in 0..16u32 {
            let value = if index % 5 == 0 { 1.0f32 } else { 0.0 };
            e.mem.set_u32(matrix + 4 * index, value.to_bits());
        }
        e.call(MEMCPY, &args![MATRIX_011FEC88, matrix, 0x40u32]);
    });
}

/// The game time the shaders animate with (`011f940c`), `3600.0` (`double`, `01012640`), the
/// `double` `6.28318` (`0104eea0`) and the angle `00b790d0` stores (`011fd868`).
const GAME_TIME_011F940C: u32 = 0x011f_940c;
const SECONDS_PER_HOUR_01012640: u32 = 0x0101_2640;
const TWO_PI_0104EEA0: u32 = 0x0104_eea0;
const ANGLE_011FD868: u32 = 0x011f_d868;

// Translated from 00b790d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with one word, an object
/// with an integer at +0xe4. With vertex entry 15 enabled: `011fd868` = game time (`011f940c`)
/// / 3600.0 * 6.28318 * the integer (x87, in the exe's order).
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn fn_00b790d0(e: &mut Engine, _this: Ptr, object: Ptr) {
    if !entry_enabled(e, VERTEX_ENTRIES_011FEBD8 + 4 * 15) {
        return;
    }
    let time = x87_load32(e.global::<u32>(GAME_TIME_011F940C));
    let value = x87_div(time, e.global::<f64>(SECONDS_PER_HOUR_01012640));
    let value = x87_mul(value, e.global::<f64>(TWO_PI_0104EEA0));
    let count = e.mem.u32(object.addr() + 0xe4) as i32;
    let value = x87_mul(value, f64::from(count));
    e.set_global(ANGLE_011FD868, x87_store32(value));
}

/// The camera position the shaders subtract (`011f474c`, three floats), the `double` 15.0
/// (`01015a38`) and the four floats `00b79100` reads (`011f95f4`) and writes (`011fa2e0`).
const CAMERA_011F474C: u32 = 0x011f_474c;
const FIFTEEN_01015A38: u32 = 0x0101_5a38;
const SOURCE_011F95F4: u32 = 0x011f_95f4;
const TARGET_011FA2E0: u32 = 0x011f_a2e0;

// Translated from 00b79100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with one word it does not
/// read. With vertex entry 16 enabled: the four floats at `011fa2e0` become those at
/// `011f95f4` less the camera's x and y (`011f474c`) and less 15.0 twice (the last as `FSUBR`).
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn fn_00b79100(e: &mut Engine, _this: Ptr, _unused_0: u32) {
    if !entry_enabled(e, VERTEX_ENTRIES_011FEBD8 + 4 * 16) {
        return;
    }
    for index in 0..2u32 {
        let source = x87_load32(e.global::<u32>(SOURCE_011F95F4 + 4 * index));
        let camera = x87_m32(e.global::<u32>(CAMERA_011F474C + 4 * index));
        let value = x87_store32(x87_sub(source, camera));
        e.set_global(TARGET_011FA2E0 + 4 * index, value);
    }
    let third = x87_load32(e.global::<u32>(SOURCE_011F95F4 + 8));
    let fifteen = x87_load64(e.global::<u64>(FIFTEEN_01015A38));
    let value = x87_store32(x87_sub(third, fifteen));
    e.set_global(TARGET_011FA2E0 + 8, value);
    let fourth = x87_m32(e.global::<u32>(SOURCE_011F95F4 + 0xc));
    let value = x87_store32(x87_sub(fourth, fifteen));
    e.set_global(TARGET_011FA2E0 + 0xc, value);
}

/// The position used when an object has none (`011f4288`) and the three floats `00b79180`
/// writes (`011fd8b4`).
const DEFAULT_POSITION_011F4288: u32 = 0x011f_4288;
const OFFSET_011FD8B4: u32 = 0x011f_d8b4;

// Translated from 00b79180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with one word, an object
/// whose +0x20 points at a position (`011f4288` when null). With vertex entry 17 enabled: the
/// position less the camera (`011f474c`), each rounded to a float on the stack and copied to
/// `011fd8b4`.
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn fn_00b79180(e: &mut Engine, _this: Ptr, object: Ptr) {
    if !entry_enabled(e, VERTEX_ENTRIES_011FEBD8 + 4 * 17) {
        return;
    }
    let mut position = e.mem.u32(object.addr() + 0x20);
    if position == 0 {
        position = DEFAULT_POSITION_011F4288;
    }
    let mut values = [0u32; 3];
    for (index, value) in values.iter_mut().enumerate() {
        let offset = 4 * index as u32;
        let coordinate = x87_load32(e.mem.u32(position + offset));
        let camera = x87_m32(e.global::<u32>(CAMERA_011F474C + offset));
        *value = x87_store32(x87_sub(coordinate, camera));
    }
    for (index, value) in values.iter().enumerate() {
        e.set_global(OFFSET_011FD8B4 + 4 * index as u32, x87_quiet_bits(*value));
    }
}

/// The four floats `00b791f0` writes (`011fa0b0`), the float it uses for type 0xfe
/// (`011ad808`), the times and scale of its type 0x230 (`011f94ac`, `011ad838`; `double` 5.0
/// at `01020998`), the two pairs it lerps between (`011f962c`/`011f9634` and
/// `011f9630`/`011f9638`), the byte it clears when the factor is 1.0 (`011f94aa`), and the
/// float max / min helpers (`00404010`, `0040ebd0`; `cdecl`, two floats, the result in ST0).
const VECTOR_011FA0B0: u32 = 0x011f_a0b0;
const VALUE_011AD808: u32 = 0x011a_d808;
const START_TIME_011F94AC: u32 = 0x011f_94ac;
const DURATION_011AD838: u32 = 0x011a_d838;
const FIVE_01020998: u32 = 0x0102_0998;
const TO_X_011F962C: u32 = 0x011f_962c;
const TO_Y_011F9630: u32 = 0x011f_9630;
const FROM_X_011F9634: u32 = 0x011f_9634;
const FROM_Y_011F9638: u32 = 0x011f_9638;
const RUNNING_011F94AA: u32 = 0x011f_94aa;
const FLOAT_MAX_00404010: u32 = 0x0040_4010;
const FLOAT_MIN_0040EBD0: u32 = 0x0040_ebd0;

// Translated from 00b791f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with three words: the pass
/// type and two objects. With vertex entry 21 enabled, `011fa0b0` (four floats) becomes for
/// type 0xfe (the first object's +0x80, `011ad808`, 0, 0); for 0x61 (1, 0, 0, 0); for 0x230 the
/// first object's +0x94 to +0xa0 with the last two replaced by the lerps between the pairs at
/// `011f9634`/`011f962c` and `011f9638`/`011f9630` by the factor min(1, max(0, (time -
/// `011f94ac`) / (`011ad838` * 5))) less the second object's +0x8c and +0x90 (the factor 1.0
/// clears `011f94aa`). Every x87 step and copy is in the exe's order.
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn fn_00b791f0(e: &mut Engine, _this: Ptr, pass_type: u32, first: Ptr, second: Ptr) {
    if !entry_enabled(e, VERTEX_ENTRIES_011FEBD8 + 4 * 21) {
        return;
    }
    let first = first.addr();
    let second = second.addr();
    match pass_type {
        0xfe => {
            let x = copy_float(e, first + 0x80);
            e.set_global(VECTOR_011FA0B0, x);
            let y = copy_float(e, VALUE_011AD808);
            e.set_global(VECTOR_011FA0B0 + 4, y);
            e.set_global(VECTOR_011FA0B0 + 8, 0u32);
            e.set_global(VECTOR_011FA0B0 + 0xc, 0u32);
        }
        0x61 => {
            e.set_global(VECTOR_011FA0B0, 1.0f32.to_bits());
            for offset in [4u32, 8, 0xc] {
                e.set_global(VECTOR_011FA0B0 + offset, 0u32);
            }
        }
        0x230 => {
            let time = x87_load32(e.global::<u32>(GAME_TIME_011F940C));
            let elapsed = x87_sub(time, x87_m32(e.global::<u32>(START_TIME_011F94AC)));
            let scale = x87_load32(e.global::<u32>(DURATION_011AD838));
            let scale = x87_mul(scale, e.global::<f64>(FIVE_01020998));
            let x_offset = e.mem.u32(second + 0x8c);
            let y_offset = e.mem.u32(second + 0x90);
            // The exe also reads +0x94 into a stack slot nothing reads.
            let _unread = e.mem.u32(second + 0x94);
            let ratio = x87_store32(x87_div(elapsed, scale));
            let ratio = x87_quiet_bits(ratio);
            let raised = e.call(FLOAT_MAX_00404010, &args![0u32, ratio]).f64();
            let raised = x87_store32(raised);
            let factor = e
                .call(FLOAT_MIN_0040EBD0, &args![1.0f32.to_bits(), raised])
                .f64();
            let factor = x87_store32(factor);
            let to_x = x87_load32(e.global::<u32>(TO_X_011F962C));
            let span_x = x87_store32(x87_sub(to_x, x87_m32(e.global::<u32>(FROM_X_011F9634))));
            let to_y = x87_load32(e.global::<u32>(TO_Y_011F9630));
            let span_y = x87_store32(x87_sub(to_y, x87_m32(e.global::<u32>(FROM_Y_011F9638))));
            let from_x = e.global::<u32>(FROM_X_011F9634);
            let from_y = e.global::<u32>(FROM_Y_011F9638);
            let factor_value = x87_load32(factor);
            let scaled_x = x87_store32(x87_mul(x87_load32(span_x), factor_value));
            let scaled_y = x87_store32(x87_mul(x87_load32(span_y), factor_value));
            let lerp_x = x87_store32(x87_add(x87_load32(scaled_x), x87_m32(from_x)));
            let lerp_y = x87_store32(x87_add(x87_load32(scaled_y), x87_m32(from_y)));
            if factor_value == 1.0 {
                e.set_global(RUNNING_011F94AA, 0u8);
            }
            for (index, offset) in [0x94u32, 0x98, 0x9c, 0xa0].iter().enumerate() {
                let word = e.mem.u32(first + offset);
                e.set_global(VECTOR_011FA0B0 + 4 * index as u32, word);
            }
            let x = x87_store32(x87_sub(x87_load32(lerp_x), x87_m32(x_offset)));
            e.set_global(VECTOR_011FA0B0 + 8, x);
            let y = x87_store32(x87_sub(x87_load32(lerp_y), x87_m32(y_offset)));
            e.set_global(VECTOR_011FA0B0 + 0xc, y);
        }
        _ => {}
    }
}

/// The rows of four floats `00b79410` and `00b7c580` write (`011fa0c0`, 0x10 bytes each).
const ROWS_011FA0C0: u32 = 0x011f_a0c0;

// Translated from 00b79410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with two words: the pass
/// type and an object (bytes at +9 and +0xb). Types 0x93 and 0x1f1 to 0x1f5 use rows 2 and 3
/// of `011fa0c0` and set pixel entry 2's register count (+0x20) to max(3, byte +9 + 2,
/// signed); type 0x1f6 uses rows 0 and 0x1b; others leave everything. With the selector byte
/// +0xb the first row becomes (1 for 1, 1 for 2, 1 for 3, 1 for 4, else 0 each) and the second
/// (selector 5, 6, 7, 8), as the x87 stack juggling of 0.0 and 1.0 in the exe leaves them.
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn fn_00b79410(e: &mut Engine, _this: Ptr, pass_type: u32, object: Ptr) {
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
        let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 4 * 2);
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
    let first = ROWS_011FA0C0 + first_row * 0x10;
    for (index, wanted) in [1u32, 2, 3, 4].iter().enumerate() {
        e.mem.set_u32(first + 4 * index as u32, flag(*wanted));
    }
    let second = ROWS_011FA0C0.wrapping_add(second_row * 0x10);
    for (index, wanted) in [5u32, 6, 7, 8].iter().enumerate() {
        e.mem.set_u32(second + 4 * index as u32, flag(*wanted));
    }
}

/// `ShadowLightShader::UpdateTogglesConstantLand` (`00c02c30`, engine map; `thiscall`, four
/// words), `BSShaderManager::GetFogProperty` (`00b55520`, `cdecl`, one word), the fog
/// property index byte (`011f91c4`), the three floats `SetupGeometryConstants_Toggles` writes
/// (`011fa2d4`, `011fa2d8`, `011fa2dc`), `1.0` and `255.0` (`double`, `01012070`, `0101e568`) and
/// the float it uses for bit 0 (`010af2f0`).
const UPDATE_TOGGLES_LAND: u32 = 0x00c0_2c30;
const GET_FOG_PROPERTY: u32 = 0x00b5_5520;
const FOG_INDEX_011F91C4: u32 = 0x011f_91c4;
const TOGGLE_011FA2D4: u32 = 0x011f_a2d4;
const TOGGLE_011FA2D8: u32 = 0x011f_a2d8;
const TOGGLE_011FA2DC: u32 = 0x011f_a2dc;
const ONE_01012070: u32 = 0x0101_2070;
const BYTE_SCALE_0101E568: u32 = 0x0101_e568;
const TOGGLE_VALUE_010AF2F0: u32 = 0x010a_f2f0;

// Translated from 00b795b0 (decompiled, FalloutNV.exe 1.4.0.525)
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
pub fn shadow_light_shader_setup_geometry_constants_toggles(
    e: &mut Engine,
    this: Ptr,
    pass_type: u32,
    geometry: u32,
    property: Ptr,
    fog: u32,
    object: u32,
    flags_property: Ptr,
) {
    if !entry_enabled(e, PIXEL_ENTRIES_011FEC30 + 4 * 4) {
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
        let index = u32::from(e.global::<u8>(FOG_INDEX_011F91C4));
        let fog_property = e.call(GET_FOG_PROPERTY, &args![index]).u32();
        let target = e.mem.u32(vtable + 0x154);
        e.call(target, &args![base, geometry, fog_property, fog]);
    }
    if object != 0 && e.mem.u8(object + 8) == 0 {
        e.set_global(TOGGLE_011FA2D4, 0u32);
    }
    let land = e.mem.u32(property + 0x20) & 0x4000 != 0;
    if !land && !(0x248..=0x24b).contains(&pass_type) {
        let flags_property = flags_property.addr();
        let flags = e.mem.u16(flags_property + 0x18);
        if (flags >> 9) & 1 != 0 {
            let byte = f64::from(e.mem.u8(flags_property + 0x1a));
            let mut value = x87_load32(x87_store32(byte));
            if value < e.global::<f64>(ONE_01012070) {
                value = 1.0;
            }
            let value = x87_load32(x87_store32(value));
            let value = x87_div(value, e.global::<f64>(BYTE_SCALE_0101E568));
            e.set_global(TOGGLE_011FA2DC, x87_store32(value));
        } else if e.mem.u8(flags_property + 0x18) & 1 != 0 {
            let value = copy_float(e, TOGGLE_VALUE_010AF2F0);
            e.set_global(TOGGLE_011FA2DC, value);
        } else {
            e.set_global(TOGGLE_011FA2DC, 0u32);
        }
        return;
    }
    if pass_type.wrapping_sub(0x248) > 3 {
        return;
    }
    let lit = e.vcall(property, 0xf4, &args![5u32, 0u32]).u32();
    let value = if lit != 0 { 1.0f32.to_bits() } else { 0 };
    e.set_global(TOGGLE_011FA2DC, value);
    let word = copy_float(e, property + 0x30);
    e.set_global(TOGGLE_011FA2D8, word);
}

/// The two floats `00b796f0` writes (`011fd944`, `011fd948`) and its float for bit 4
/// (`010177e8`).
const VALUE_011FD944: u32 = 0x011f_d944;
const VALUE_011FD948: u32 = 0x011f_d948;
const VALUE_010177E8: u32 = 0x0101_77e8;

// Translated from 00b796f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with one word, an object.
/// With pixel entry 16 enabled: `011fd944` takes its +0xe0, `011fd948` the float at `010177e8`
/// when bit 4 of its byte +0x24 is set, else 1.0.
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn fn_00b796f0(e: &mut Engine, _this: Ptr, object: Ptr) {
    if !entry_enabled(e, PIXEL_ENTRIES_011FEC30 + 4 * 16) {
        return;
    }
    let object = object.addr();
    let value = copy_float(e, object + 0xe0);
    e.set_global(VALUE_011FD944, value);
    let value = if e.mem.u8(object + 0x24) & 0x10 != 0 {
        copy_float(e, VALUE_010177E8)
    } else {
        1.0f32.to_bits()
    };
    e.set_global(VALUE_011FD948, value);
}

/// The plane `00b79730` reads (`011f9604`, four floats), the vector it dots it with
/// (`011fa2a0`), the two distances it compares (`011f9614`, `011f9618`), its `-1.0`
/// (`01012054`) and what it writes (`011fd8e4`, four floats; `011fd8c4`, four floats).
const PLANE_011F9604: u32 = 0x011f_9604;
const VECTOR_011FA2A0: u32 = 0x011f_a2a0;
const NEAR_011F9614: u32 = 0x011f_9614;
const FAR_011F9618: u32 = 0x011f_9618;
const MINUS_ONE_01012054: u32 = 0x0101_2054;
const PLANE_011FD8E4: u32 = 0x011f_d8e4;
const RANGE_011FD8C4: u32 = 0x011f_d8c4;

// Translated from 00b79730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), no arguments. With pixel entry 19
/// enabled: `011fd8e4` takes the plane's first three floats (`011f9604`) and as the fourth the
/// dot product with `011fa2a0` ((y y' + x x') + z z', in that order) less the plane's fourth;
/// `011fd8c4` becomes (0, 0, -1.0, 0) when both distances (`011f9618`, `011f9614`) are 0, else
/// (far, far - near, 1.0, 1.0).
pub fn fn_00b79730(e: &mut Engine) {
    if !entry_enabled(e, PIXEL_ENTRIES_011FEC30 + 4 * 19) {
        return;
    }
    let vector: Vec<u32> = (0..3)
        .map(|index| copy_float(e, VECTOR_011FA2A0 + 4 * index))
        .collect();
    let plane: Vec<u32> = (0..4)
        .map(|index| e.global::<u32>(PLANE_011F9604 + 4 * index))
        .collect();
    let y = x87_mul(x87_load32(plane[1]), x87_m32(vector[1]));
    let x = x87_mul(x87_load32(plane[0]), x87_m32(vector[0]));
    for index in 0..3u32 {
        let copy = x87_quiet_bits(plane[index as usize]);
        e.set_global(PLANE_011FD8E4 + 4 * index, copy);
    }
    let sum = x87_add(y, x);
    let z = x87_mul(x87_load32(plane[2]), x87_m32(vector[2]));
    let dot = x87_store32(x87_add(sum, z));
    let distance = x87_store32(x87_sub(x87_load32(dot), x87_m32(plane[3])));
    e.set_global(PLANE_011FD8E4 + 0xc, distance);
    let far = copy_float(e, FAR_011F9618);
    let near = copy_float(e, NEAR_011F9614);
    let far_value = x87_load32(far);
    let near_value = x87_load32(near);
    if far_value == 0.0 && near_value == 0.0 {
        e.set_global(RANGE_011FD8C4, 0u32);
        e.set_global(RANGE_011FD8C4 + 4, 0u32);
        let minus_one = copy_float(e, MINUS_ONE_01012054);
        e.set_global(RANGE_011FD8C4 + 8, minus_one);
        e.set_global(RANGE_011FD8C4 + 0xc, 0u32);
        return;
    }
    e.set_global(RANGE_011FD8C4, far);
    let span = x87_store32(x87_sub(far_value, near_value));
    e.set_global(RANGE_011FD8C4 + 4, span);
    e.set_global(RANGE_011FD8C4 + 8, 1.0f32.to_bits());
    e.set_global(RANGE_011FD8C4 + 0xc, 1.0f32.to_bits());
}

/// The four floats `00b798c0` writes (`011fa300`).
const VECTOR_011FA300: u32 = 0x011f_a300;

// Translated from 00b798c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with one word, an object.
/// With pixel entry 20 enabled: `011fa300` takes its floats +0xf0, +0xf4, +0xfc, +0xf8 in that
/// order (each copied through the x87 stack).
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn fn_00b798c0(e: &mut Engine, _this: Ptr, object: Ptr) {
    if !entry_enabled(e, PIXEL_ENTRIES_011FEC30 + 4 * 20) {
        return;
    }
    let object = object.addr();
    for (index, offset) in [0xf0u32, 0xf4, 0xfc, 0xf8].iter().enumerate() {
        let value = copy_float(e, object + offset);
        e.set_global(VECTOR_011FA300 + 4 * index as u32, value);
    }
}

/// The pass being set up (`0126f74c`; its stages at +0x24), the stage's `00e7de90` (unnamed,
/// `thiscall`, one word) and `NiD3DTextureStage::SetFilterMode` (`00e7def0`).
const CURRENT_PASS_0126F74C: u32 = 0x0126_f74c;
const STAGE_MODE_00E7DE90: u32 = 0x00e7_de90;
const SET_FILTER_MODE: u32 = 0x00e7_def0;

/// Stage `index` of the current pass.
fn pass_stage(e: &Engine, index: u32) -> u32 {
    let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
    let stages = e.mem.u32(pass + 0x24);
    e.mem.u32(stages.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00b7c0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `stdcall` with three words: an object
/// (textures at +0xac, its vtable +0xe0 answers a mode), a stage index and a texture index.
/// The current pass's stage gets the texture at +8 and the object's mode (`00e7de90`).
pub fn fn_00b7c0a0(e: &mut Engine, object: Ptr, stage: u32, texture: u32) {
    let stage = pass_stage(e, stage);
    let object = object.addr();
    let textures = e.mem.u32(object + 0xac);
    let texture = e.mem.u32(textures.wrapping_add(texture.wrapping_mul(4)));
    e.mem.set_u32(stage + 8, texture);
    let mode = e.vcall(object, 0xe0, &args![]).u32();
    e.call(STAGE_MODE_00E7DE90, &args![stage, mode]);
}

/// The texture getter `00c03230` (unnamed, `stdcall`, two words).
const TEXTURE_00C03230: u32 = 0x00c0_3230;

// Translated from 00b7c0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `stdcall` with three words: an object,
/// a stage index and a word. The stage gets `00c03230(object, word)` at +8, and with an object
/// its mode (vtable +0xe0) through `00e7de90`.
pub fn fn_00b7c0e0(e: &mut Engine, object: Ptr, stage: u32, word: u32) {
    let stage = pass_stage(e, stage);
    let object = object.addr();
    let texture = e.call(TEXTURE_00C03230, &args![object, word]).u32();
    e.mem.set_u32(stage + 8, texture);
    if object != 0 {
        let mode = e.vcall(object, 0xe0, &args![]).u32();
        e.call(STAGE_MODE_00E7DE90, &args![stage, mode]);
    }
}

/// The pass and type being rendered (`011f91e0`) and `BSRenderedTexture::GetTexture`
/// (`004bc320`, engine map; `thiscall`, one word).
const RENDERING_PASS_011F91E0: u32 = 0x011f_91e0;
const RENDERED_TEXTURE_GET_TEXTURE: u32 = 0x004b_c320;

// Translated from 00b7c120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `stdcall` with one word, a stage
/// index. The stage gets `GetTexture(0)` of the rendered texture at +0x10c of the first word of
/// the rendering pass's +0xc.
pub fn fn_00b7c120(e: &mut Engine, stage: u32) {
    let stage = pass_stage(e, stage);
    let rendering = e.global::<u32>(RENDERING_PASS_011F91E0);
    let holder = e.mem.u32(rendering + 0xc);
    let object = e.mem.u32(holder);
    let rendered = e.mem.u32(object + 0x10c);
    let texture = e
        .call(RENDERED_TEXTURE_GET_TEXTURE, &args![rendered, 0u32])
        .u32();
    e.mem.set_u32(stage + 8, texture);
}

/// `00b7c190`'s choice byte (`011ad80c`), its two textures (`011f953c`, `011f9540`), its filter
/// byte (`011f9425`), the renderer (`011f4748`, device at +0x288) and the sampler value
/// (`011f9428`).
const CHOICE_011AD80C: u32 = 0x011a_d80c;
const TEXTURE_011F953C: u32 = 0x011f_953c;
const TEXTURE_011F9540: u32 = 0x011f_9540;
const FILTER_011F9425: u32 = 0x011f_9425;
const RENDERER_011F4748: u32 = 0x011f_4748;
const SAMPLER_VALUE_011F9428: u32 = 0x011f_9428;

// Translated from 00b7c190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `stdcall` with one word, a stage
/// index. The stage gets the texture `011f953c` (byte `011ad80c` set) or `011f9540`, the filter
/// mode 7 when `011f9425` is set, and the device (renderer `011f4748` +0x288) its vtable +0x114
/// with the index, 8 and `011f9428`.
pub fn fn_00b7c190(e: &mut Engine, index: u32) {
    let stage = pass_stage(e, index);
    let source = if e.global::<u8>(CHOICE_011AD80C) != 0 {
        TEXTURE_011F953C
    } else {
        TEXTURE_011F9540
    };
    let texture = e.mem.u32(source);
    e.mem.set_u32(stage + 8, texture);
    if e.global::<u8>(FILTER_011F9425) != 0 {
        e.call(SET_FILTER_MODE, &args![stage, 7u32]);
    }
    let value = e.global::<u32>(SAMPLER_VALUE_011F9428);
    let renderer = e.global::<u32>(RENDERER_011F4748);
    let device = e.mem.u32(renderer + 0x288);
    e.vcall(device, 0x114, &args![index, 8u32, value]);
}

/// The fallback texture of `00b7c1f0` and `00b7c230` (`011f951c`) and of `00b7c2c0`
/// (`011f9514`).
const FALLBACK_011F951C: u32 = 0x011f_951c;
const FALLBACK_011F9514: u32 = 0x011f_9514;

// Translated from 00b7c1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `stdcall` with four words: an object
/// (textures at +0xb4), a stage index, a texture index and a flag (low byte). The stage gets the
/// object's texture, or when it is null `011f951c` with the flag and null without.
pub fn fn_00b7c1f0(e: &mut Engine, object: Ptr, stage: u32, texture: u32, fallback: bool) {
    let stage = pass_stage(e, stage);
    let textures = e.mem.u32(object.addr() + 0xb4);
    let texture = e.mem.u32(textures.wrapping_add(texture.wrapping_mul(4)));
    let value = if texture != 0 {
        texture
    } else if fallback {
        e.global::<u32>(FALLBACK_011F951C)
    } else {
        0
    };
    e.mem.set_u32(stage + 8, value);
}

// Translated from 00b7c230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `stdcall` with four words: an object, a
/// stage index, a word and a flag (low byte). The stage gets what the object's vtable +0xf4
/// answers for (3, word) (asked twice when not null), else `011f951c` with the flag and null
/// without.
pub fn fn_00b7c230(e: &mut Engine, object: Ptr, stage: u32, word: u32, fallback: bool) {
    let stage = pass_stage(e, stage);
    let object = object.addr();
    if e.vcall(object, 0xf4, &args![3u32, word]).u32() != 0 {
        let texture = e.vcall(object, 0xf4, &args![3u32, word]).u32();
        e.mem.set_u32(stage + 8, texture);
        return;
    }
    let value = if fallback {
        e.global::<u32>(FALLBACK_011F951C)
    } else {
        0
    };
    e.mem.set_u32(stage + 8, value);
}

/// The texture getter `00c02f30` (unnamed, `stdcall`, two words).
const TEXTURE_00C02F30: u32 = 0x00c0_2f30;

// Translated from 00b7c290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `stdcall` with three words: an object,
/// a stage index and a word. The stage gets `00c02f30(object, word)` and the mode 0.
pub fn fn_00b7c290(e: &mut Engine, object: u32, stage: u32, word: u32) {
    let stage = pass_stage(e, stage);
    let texture = e.call(TEXTURE_00C02F30, &args![object, word]).u32();
    e.mem.set_u32(stage + 8, texture);
    e.call(STAGE_MODE_00E7DE90, &args![stage, 0u32]);
}

// Translated from 00b7c2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `stdcall` with three words: an object,
/// a stage index and a word. The stage gets what the object's vtable +0xf4 answers for (5,
/// word) (asked twice when not null), else `011f9514`.
pub fn fn_00b7c2c0(e: &mut Engine, object: Ptr, stage: u32, word: u32) {
    let stage = pass_stage(e, stage);
    let object = object.addr();
    if e.vcall(object, 0xf4, &args![5u32, word]).u32() != 0 {
        let texture = e.vcall(object, 0xf4, &args![5u32, word]).u32();
        e.mem.set_u32(stage + 8, texture);
        return;
    }
    let fallback = e.global::<u32>(FALLBACK_011F9514);
    e.mem.set_u32(stage + 8, fallback);
}

/// The pass type being rendered (`011f91e4`), the render-state setters of
/// `bsrenderstate_xenon.cpp` and the batch renderer (`cdecl`, two words each): `00b97e80`,
/// `00b98070`, `BSRenderState::SetZWriteEnable` (`00b97e30`) and `00b98380`, and the two
/// counters `00b7c320` lowers (`011ff9dc`, `011ffa18`).
const RENDERING_PASS_TYPE_011F91E4: u32 = 0x011f_91e4;
const RENDER_STATE_00B97E80: u32 = 0x00b9_7e80;
const RENDER_STATE_00B98070: u32 = 0x00b9_8070;
const SET_Z_WRITE_ENABLE: u32 = 0x00b9_7e30;
const RENDER_STATE_00B98380: u32 = 0x00b9_8380;
const COUNTER_011FF9DC: u32 = 0x011f_f9dc;
const COUNTER_011FFA18: u32 = 0x011f_fa18;

// Translated from 00b7c320 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (`shadowlightshader.cpp` by address), `thiscall` with one word:
/// a block whose +0x10 is a property with flags at +0x18 and whose +0xc is the shader property
/// (flags at +0x20). Pass types 0x250 and 0x251 set `00b97e80(3, 0)`; a property with bit 0 set
/// `00b98070(0, 0)`; for pass types 6 and 8 with bit 0x40000 of the shader property: the
/// counter `011ff9dc` lowered and `SetZWriteEnable(1, 0)`, the counter `011ffa18` lowered and
/// `00b98380(7, 0)`.
/// `this` (`ECX`) is not read, but it sits in vtables and is called with `ECX` set.
pub fn fn_00b7c320(e: &mut Engine, _this: Ptr, block: Ptr) {
    let block = block.addr();
    let pass_type = e.global::<u32>(RENDERING_PASS_TYPE_011F91E4);
    if pass_type == 0x250 || pass_type == 0x251 {
        e.call(RENDER_STATE_00B97E80, &args![3u32, 0u32]);
    }
    let property = e.mem.u32(block + 0x10);
    if property != 0 && e.mem.u8(property + 0x18) & 1 != 0 {
        e.call(RENDER_STATE_00B98070, &args![0u32, 0u32]);
    }
    let shader_property = e.mem.u32(block + 0xc);
    if (pass_type == 6 || pass_type == 8) && e.mem.u32(shader_property + 0x20) & 0x40000 != 0 {
        let count = e.global::<u32>(COUNTER_011FF9DC).wrapping_sub(1);
        e.set_global(COUNTER_011FF9DC, count);
        e.call(SET_Z_WRITE_ENABLE, &args![1u32, 0u32]);
        let count = e.global::<u32>(COUNTER_011FFA18).wrapping_sub(1);
        e.set_global(COUNTER_011FFA18, count);
        e.call(RENDER_STATE_00B98380, &args![7u32, 0u32]);
    }
}

/// The eight floats `00b7c3a0` builds (`011fa280`), its far distance (`500000.0` at `010ad6e0`)
/// and the fog colour it uses then (`011a9bd0`, four words).
const FOG_011FA280: u32 = 0x011f_a280;
const FAR_DISTANCE_010AD6E0: u32 = 0x010a_d6e0;
const FOG_COLOUR_011A9BD0: u32 = 0x011a_9bd0;

/// The vertex shader of the current pass's +0x5c: its device (+0x20) gets `count` float
/// constants from `register` at `data` (the device's vtable +0x178).
fn set_vertex_constants(e: &mut Engine, register: u32, data: u32, count: u32) {
    let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
    let shader = e.mem.u32(pass + 0x5c);
    let device = e.mem.u32(shader + 0x20);
    e.vcall(device, 0x178, &args![register, data, count]);
}

// Translated from 00b7c3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), no arguments. With a fog property
/// (`GetFogProperty` of the byte `011f91c4`): when its far (+0x30) and near (+0x2c) distances
/// are both 0, `011fa280` becomes (500000.0, 0, 0, 0) and the colour `011a9bd0`; else (far, far -
/// near, its +0x60, 0) and its +0x20, +0x24, +0x28, 0. Either way the vertex shader's device gets
/// the eight floats as constants 0xe and 0xf.
pub fn fn_00b7c3a0(e: &mut Engine, _this: Ptr) {
    let index = u32::from(e.global::<u8>(FOG_INDEX_011F91C4));
    let fog = e.call(GET_FOG_PROPERTY, &args![index]).u32();
    if fog != 0 {
        let near = copy_float(e, fog + 0x2c);
        let far = copy_float(e, fog + 0x30);
        let power = copy_float(e, fog + 0x60);
        let far_value = x87_load32(far);
        let near_value = x87_load32(near);
        if far_value == 0.0 && near_value == 0.0 {
            let distance = copy_float(e, FAR_DISTANCE_010AD6E0);
            e.set_global(FOG_011FA280, distance);
            for offset in [4u32, 8, 0xc] {
                e.set_global(FOG_011FA280 + offset, 0u32);
            }
            let colour: Vec<u32> = (0..4)
                .map(|index| e.global::<u32>(FOG_COLOUR_011A9BD0 + 4 * index))
                .collect();
            e.set_global(FOG_011FA280 + 0x10, colour[0]);
            e.set_global(FOG_011FA280 + 0x1c, colour[3]);
            e.set_global(FOG_011FA280 + 0x18, colour[2]);
            e.set_global(FOG_011FA280 + 0x14, colour[1]);
        } else {
            let red = e.mem.u32(fog + 0x20);
            let green = e.mem.u32(fog + 0x24);
            let span = x87_store32(x87_sub(far_value, near_value));
            let blue = e.mem.u32(fog + 0x28);
            e.set_global(FOG_011FA280, far);
            e.set_global(FOG_011FA280 + 4, span);
            e.set_global(FOG_011FA280 + 8, x87_quiet_bits(power));
            e.set_global(FOG_011FA280 + 0xc, 0u32);
            e.set_global(FOG_011FA280 + 0x10, x87_quiet_bits(red));
            e.set_global(FOG_011FA280 + 0x1c, 0u32);
            e.set_global(FOG_011FA280 + 0x18, x87_quiet_bits(blue));
            e.set_global(FOG_011FA280 + 0x14, x87_quiet_bits(green));
        }
    }
    set_vertex_constants(e, 0xe, FOG_011FA280, 2);
}

/// The table of enabled-constant masks by pass type (`011fc0a0`, one word per type).
const PIXEL_FLAGS_011FC0A0: u32 = 0x011f_c0a0;

// Translated from 00b7c510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` on the shader, no stack
/// words. Each of the 22 pixel entries (`011fec30`) that is set is enabled when its bit (entry
/// k: bit k + 1, as the exe rotates its mask) is set in the word of `011fc0a0` for the pass type
/// being rendered (`011f91e4`, read for each); then the pixel constant map's (+0x30)
/// `SortByEnabled` (vtable +0x24) runs and its result is answered.
pub fn fn_00b7c510(e: &mut Engine, this: Ptr) -> u32 {
    let mut mask = 1u32;
    for index in 0..22u32 {
        let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 4 * index);
        if entry != 0 {
            let pass_type = e.global::<u32>(RENDERING_PASS_TYPE_011F91E4);
            let flags = e
                .mem
                .u32(PIXEL_FLAGS_011FC0A0.wrapping_add(pass_type.wrapping_mul(4)));
            let enabled = flags & mask.rotate_left(1) != 0;
            e.mem.set_u8(entry + 8, u8::from(enabled));
        }
        mask = mask.rotate_left(1);
    }
    let map = e.mem.u32(this.addr() + 0x30);
    e.vcall(map, 0x24, &args![]).u32()
}

// Translated from 00b7c580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), no arguments. Row 0 of `011fa0c0`
/// becomes the three floats +0xc8 to +0xd0 of the object at +0xf8 of the first word of the
/// rendering pass's +0xc (copied through the x87 stack), then 1.0.
pub fn fn_00b7c580(e: &mut Engine, _this: Ptr) {
    let object = rendering_object(e);
    let values = e.mem.u32(object + 0xf8);
    for index in 0..3u32 {
        let value = copy_float(e, values + 0xc8 + 4 * index);
        e.set_global(ROWS_011FA0C0 + 4 * index, value);
    }
    e.set_global(ROWS_011FA0C0 + 0xc, 1.0f32.to_bits());
}

/// The first word of the rendering pass's (`011f91e0`) +0xc.
fn rendering_object(e: &Engine) -> u32 {
    let rendering = e.global::<u32>(RENDERING_PASS_011F91E0);
    let holder = e.mem.u32(rendering + 0xc);
    e.mem.u32(holder)
}

/// The HDR byte (`011f941e`), the byte that keeps the HDR scale off (`011f9427`), the HDR
/// scale (`011f9190`) and the four floats `00b7c600` writes (`011fa0d0`).
const HDR_011F941E: u32 = 0x011f_941e;
const HDR_SCALE_DISABLED_011F9427: u32 = 0x011f_9427;
const HDR_SCALE_011F9190: u32 = 0x011f_9190;
const COLOUR_011FA0D0: u32 = 0x011f_a0d0;

// Translated from 00b7c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), no arguments. With the rendering
/// object (first word of the rendering pass's +0xc) and its +0xf8: the strength +0xc4 (at most
/// 1.0 without HDR, `011f941e`), the object's +0xd0 and the colour +0xd4 to +0xdc give
/// `011fa0d0` = (colour * strength) * object +0xd0 for each, times the HDR scale (`011f9190`)
/// with HDR and `011f9427` clear, then the object's +0xd4; each x87 step and float store in the
/// exe's order and operand order.
pub fn fn_00b7c600(e: &mut Engine, _this: Ptr) {
    let object = rendering_object(e);
    let values = e.mem.u32(object + 0xf8);
    let strength_bits = copy_float(e, values + 0xc4);
    let mut strength = x87_load32(strength_bits);
    let hdr = e.global::<u8>(HDR_011F941E) != 0;
    if !hdr && strength > e.global::<f64>(ONE_01012070) {
        strength = x87_load32(1.0f32.to_bits());
    }
    let scale_bits = copy_float(e, object + 0xd0);
    let red = e.mem.u32(values + 0xd4);
    let green = e.mem.u32(values + 0xd8);
    let blue = e.mem.u32(values + 0xdc);
    let red_strength = x87_store32(x87_mul(x87_load32(red), strength));
    let green_strength = x87_store32(x87_mul(x87_load32(green), strength));
    let blue_strength = x87_store32(x87_mul(strength, x87_m32(blue)));
    let scale = x87_load32(scale_bits);
    let mut colour = [
        x87_store32(x87_mul(x87_load32(red_strength), scale)),
        x87_store32(x87_mul(scale, x87_m32(green_strength))),
        x87_store32(x87_mul(scale, x87_m32(blue_strength))),
    ];
    if hdr && e.global::<u8>(HDR_SCALE_DISABLED_011F9427) == 0 {
        let factor = x87_load32(e.global::<u32>(HDR_SCALE_011F9190));
        for value in colour.iter_mut() {
            *value = x87_store32(x87_mul(factor, x87_m32(*value)));
        }
    }
    for (index, value) in colour.iter().enumerate() {
        e.set_global(COLOUR_011FA0D0 + 4 * index as u32, x87_quiet_bits(*value));
    }
    let alpha = copy_float(e, object + 0xd4);
    e.set_global(COLOUR_011FA0D0 + 0xc, alpha);
}

/// The renderer method `00e6fbb0` (unnamed, `thiscall`, two words) and the D3DX import thunks
/// `D3DXMatrixMultiplyTranspose` (`00ee6dfe`), `D3DXVec3TransformNormal` (`00ee6df2`) and
/// `D3DXVec3Normalize` (`00ee6dec`) (decompiler names of their imports; `stdcall`).
const RENDERER_METHOD_00E6FBB0: u32 = 0x00e6_fbb0;
const MATRIX_MULTIPLY_TRANSPOSE: u32 = 0x00ee_6dfe;
const VEC3_TRANSFORM_NORMAL: u32 = 0x00ee_6df2;
const VEC3_NORMALIZE: u32 = 0x00ee_6dec;

// Translated from 00b7c750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with two stack words, the
/// first not read. The renderer at +0x18 gets `00e6fbb0(word, 0)`; the product of its matrices
/// at +0x940 and +0xa00, transposed (`D3DXMatrixMultiplyTranspose` into a 0x40-byte stack
/// matrix), goes to the vertex shader's device as constants 0 to 3.
pub fn fn_00b7c750(e: &mut Engine, this: Ptr, _unused_0: u32, word: u32) {
    let renderer = e.mem.u32(this.addr() + 0x18);
    e.call(RENDERER_METHOD_00E6FBB0, &args![renderer, word, 0u32]);
    let renderer = e.mem.u32(this.addr() + 0x18);
    e.with_stack(0x40, |e, matrix| {
        e.call(
            MATRIX_MULTIPLY_TRANSPOSE,
            &args![matrix, renderer + 0x940, renderer + 0xa00],
        );
        set_vertex_constants(e, 0, matrix.addr(), 4);
    });
}

/// `-1.0` (`double`, `0101a6b0`).
const MINUS_ONE_0101A6B0: u32 = 0x0101_a6b0;

// Translated from 00b7c7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `stdcall` with one word, a matrix. The
/// rendering object's vector (+0xf0 to +0xf8 of its +0xf8) negated (x87 multiplies by -1.0) is
/// transformed by the matrix (`D3DXVec3TransformNormal` into the frame's last three words) and
/// normalized into the negated copy (`D3DXVec3Normalize(copy, transformed)`); the vertex
/// shader's device then gets constant 0x19 from the transformed words, as the exe passes it.
// Translation incomplete: 00b7c7b0 - the constant's fourth float is the word above the exe's 0x20-byte frame (this function's return address); the engine's stack model has no such word and leaves it 0.
pub fn fn_00b7c7b0(e: &mut Engine, matrix: u32) {
    let object = rendering_object(e);
    let vector = e.mem.u32(object + 0xf8);
    let minus_one = x87_load64(e.global::<u64>(MINUS_ONE_0101A6B0));
    let y = x87_store32(x87_mul(x87_load32(e.mem.u32(vector + 0xf4)), minus_one));
    let z = x87_store32(x87_mul(x87_load32(e.mem.u32(vector + 0xf8)), minus_one));
    let x = x87_store32(x87_mul(minus_one, x87_m32(e.mem.u32(vector + 0xf0))));
    e.with_stack(0x24, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame, y);
        e.mem.set_u32(frame + 4, z);
        e.mem.set_u32(frame + 8, x);
        e.mem.set_u32(frame + 0xc, x87_quiet_bits(y));
        e.mem.set_u32(frame + 0x10, x87_quiet_bits(z));
        e.call(
            VEC3_TRANSFORM_NORMAL,
            &args![frame + 0x14, frame + 8, matrix],
        );
        e.call(VEC3_NORMALIZE, &args![frame + 8, frame + 0x14]);
        set_vertex_constants(e, 0x19, frame + 0x14, 1);
    });
}

/// The memory manager (`011f6238`) and its free (`00aa4060`, `thiscall`, one word), the
/// `eh vector destructor iterator` (`00ec5fce`, `stdcall`) and the element destructor the
/// pointer arrays pass it (`0045cec0`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;

// Translated from 00b7ce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`): `delete[]` of an array of `NiPointer`s,
/// `cdecl` with one word, the elements. With elements: their count (the word before them) runs
/// through the `eh vector destructor iterator` with `0045cec0`, then the block (from the count)
/// goes to the memory manager's free.
pub fn fn_00b7ce30(e: &mut Engine, elements: u32) {
    if elements == 0 {
        return;
    }
    let count = e.mem.u32(elements.wrapping_sub(4));
    e.call(
        VECTOR_DESTRUCT,
        &args![elements, 4u32, count, POINTER_DESTRUCT_0045CEC0],
    );
    e.call(
        MEMORY_FREE,
        &args![MEMORY_MANAGER, elements.wrapping_sub(4)],
    );
}

/// The shader's destructor body (`00b79de0`, unit `unplaced`, `thiscall`) and the sized Ni free
/// (`00aa1460`, `cdecl`: block, size).
const SHADER_DESTRUCT_00B79DE0: u32 = 0x00b7_9de0;
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00b7ce60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowLightShader`'s scalar deleting destructor (its vtable's slot 0, `010af2f8`),
/// `thiscall` with one stack word (flags): the destructor body `00b79de0`, then with bit 0 the
/// 0x8c bytes freed (`00aa1460`). Answers `this`.
pub fn shadow_light_shader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(SHADER_DESTRUCT_00B79DE0, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x8cu32]);
    }
    this
}

/// The eye position the shaders use (`011fa2a0`, three floats), the two vectors
/// `SetupGeometryConstants_EyePos` writes (`011fd894`, `011fd8a4`), the point unitize
/// `004a0c10` (unnamed, `thiscall` on the point) and the D3DX import thunk
/// `D3DXVec3TransformCoord` (`00ee6df8`, `stdcall`).
const EYE_011FA2A0: u32 = 0x011f_a2a0;
const EYE_011FD894: u32 = 0x011f_d894;
const EYE_DIRECTION_011FD8A4: u32 = 0x011f_d8a4;
const POINT_UNITIZE_004A0C10: u32 = 0x004a_0c10;
const VEC3_TRANSFORM_COORD: u32 = 0x00ee_6df8;

/// The uniform-form entry of `SetupGeometryConstants_EyePos`: the pass type, the object and
/// the sixteen words of the matrix.
fn eye_pos_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let mut matrix = [0u32; 16];
    matrix.copy_from_slice(&words[2..18]);
    shadow_light_shader_setup_geometry_constants_eye_pos(e, words[0], Ptr::new(words[1]), matrix);
    Ret::default()
}

// Translated from 00b78ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowLightShader::SetupGeometryConstants_EyePos` (engine map), `stdcall` with eighteen
/// words: the pass type, an object (position at +0x24, a float at +0x30) and a 4x4 matrix by
/// value. With vertex entry 9 or pixel entry 5 enabled: for pass types 0x244 to 0x24b the eye
/// (`011fa2a0`) less the object's position, unitized (`004a0c10`), goes to `011fd8a4` with 0, and
/// the eye less the camera (`011f474c`) to `011fd894` with 1.0; for other types the eye
/// transformed by the matrix (`D3DXVec3TransformCoord`) goes to `011fd894` with the object's
/// +0x30.
pub fn shadow_light_shader_setup_geometry_constants_eye_pos(
    e: &mut Engine,
    pass_type: u32,
    object: Ptr,
    matrix: [u32; 16],
) {
    if !entry_enabled(e, VERTEX_ENTRIES_011FEBD8 + 4 * 9)
        && !entry_enabled(e, PIXEL_ENTRIES_011FEC30 + 4 * 5)
    {
        return;
    }
    let object = object.addr();
    if (0x244..=0x24b).contains(&pass_type) {
        let eye: Vec<u32> = (0..3)
            .map(|index| copy_float(e, EYE_011FA2A0 + 4 * index))
            .collect();
        e.with_stack(0xc, |e, point| {
            let point = point.addr();
            for index in 0..3u32 {
                let value = x87_sub(
                    x87_load32(eye[index as usize]),
                    x87_m32(e.mem.u32(object + 0x24 + 4 * index)),
                );
                e.mem.set_u32(point + 4 * index, x87_store32(value));
            }
            e.call(POINT_UNITIZE_004A0C10, &args![point]);
            for index in 0..3u32 {
                let value = copy_float(e, point + 4 * index);
                e.set_global(EYE_DIRECTION_011FD8A4 + 4 * index, value);
            }
        });
        e.set_global(EYE_DIRECTION_011FD8A4 + 0xc, 0u32);
        let mut shifted = [0u32; 3];
        for (index, value) in shifted.iter_mut().enumerate() {
            let offset = 4 * index as u32;
            let eye = x87_load32(eye[index]);
            let camera = x87_m32(e.global::<u32>(CAMERA_011F474C + offset));
            *value = x87_store32(x87_sub(eye, camera));
        }
        for (index, value) in shifted.iter().enumerate() {
            e.set_global(EYE_011FD894 + 4 * index as u32, x87_quiet_bits(*value));
        }
        e.set_global(EYE_011FD894 + 0xc, 1.0f32.to_bits());
        return;
    }
    e.with_stack(0x58, |e, frame| {
        let frame = frame.addr();
        let vector = frame;
        let transformed = frame + 0xc;
        let by_value = frame + 0x18;
        for (index, word) in matrix.iter().enumerate() {
            e.mem.set_u32(by_value + 4 * index as u32, *word);
        }
        for index in 0..3u32 {
            let value = copy_float(e, EYE_011FA2A0 + 4 * index);
            e.mem.set_u32(vector + 4 * index, value);
        }
        e.call(VEC3_TRANSFORM_COORD, &args![transformed, vector, by_value]);
        for index in 0..3u32 {
            let value = copy_float(e, transformed + 4 * index);
            e.set_global(EYE_011FD894 + 4 * index, value);
        }
    });
    let value = copy_float(e, object + 0x30);
    e.set_global(EYE_011FD894 + 0xc, value);
}

/// `BSShaderManager::GetAccumulator` (`00b4f5c0`, engine map; no arguments) and the matrix
/// `SetupGeometryConstants_ShadowProj` writes (`011fd968`, 16 floats).
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const SHADOW_PROJECTION_011FD968: u32 = 0x011f_d968;

// Translated from 00b7b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowLightShader::SetupGeometryConstants_ShadowProj` (engine map), `thiscall` with three
/// words: a block whose +0xc is an object with a register at +0x1c, a flag and a transform (a
/// 3x3 rotation, a translation and a scale, 13 floats). With vertex entry 3 enabled, the object
/// set with a register other than -1, and the accumulator's +0x194 set: `011fd968` becomes,
/// without the flag, the rows (rotation row with translation x, y, z in the fourth column) and
/// (0, 0, 0, scale), each float copied through the x87 stack; with it, the identity with the
/// camera position (`011f474c`) in the fourth column.
/// `this` (`ECX`) is not read, but `SetupGeometryConstants` (`00b7dab0`) calls it with `ECX` set.
pub fn shadow_light_shader_setup_geometry_constants_shadow_proj(
    e: &mut Engine,
    _this: Ptr,
    block: Ptr,
    flag: u32,
    transform: Ptr,
) {
    if !entry_enabled(e, VERTEX_ENTRIES_011FEBD8 + 4 * 3) {
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
            let camera = e.global::<u32>(CAMERA_011F474C + 4 * row as u32);
            matrix[*index] = x87_quiet_bits(camera);
        }
    }
    for (index, word) in matrix.iter().enumerate() {
        e.set_global(SHADOW_PROJECTION_011FD968 + 4 * index as u32, *word);
    }
}

/// The two textures `00b7cfb0` puts on a pair of stages (`011f95e4`, `011f95e8`).
const PAIR_TEXTURE_011F95E4: u32 = 0x011f_95e4;
const PAIR_TEXTURE_011F95E8: u32 = 0x011f_95e8;

// Translated from 00b7cfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), a vtable slot (`thiscall`, `RET 4`; `this` is
/// not read; its callers 00bcc590 and 00bce800 pass it) with one word, the pass type.
/// The textures `011f95e4` and `011f95e8` go to a pair of the current pass's stages chosen by
/// the type (the exe's two jump tables): stages 2 and 3 for 0x94 and 0x1f3; 6 and 7 for 0xa0 to
/// 0xa2, 0xa6 to 0xa8, 0xb6 to 0xb8, 0xbc to 0xbe, 0x10a to 0x10c, 0x110 to 0x112, 0x115, 0x121,
/// 0x122, 0x126 to 0x128 and 0x12b; 5 and 6 for 0xe0, 0xe1, 0xe5, 0xe6, 0xf2, 0xf3, 0xf7 and
/// 0xf8; 4 and 5 for 0x160, 0x161, 0x163 and 0x164; 14 and 15 for every fourth type from 0x1f8
/// to 0x22c; none for other types.
pub fn fn_00b7cfb0(e: &mut Engine, _this: Ptr, pass_type: u32) {
    let first = match pass_type {
        0x94 | 0x1f3 => 2u32,
        0xa0..=0xa2
        | 0xa6..=0xa8
        | 0xb6..=0xb8
        | 0xbc..=0xbe
        | 0x10a..=0x10c
        | 0x110..=0x112
        | 0x115
        | 0x121
        | 0x122
        | 0x126..=0x128
        | 0x12b => 6,
        0xe0 | 0xe1 | 0xe5 | 0xe6 | 0xf2 | 0xf3 | 0xf7 | 0xf8 => 5,
        0x160 | 0x161 | 0x163 | 0x164 => 4,
        0x1f8..=0x22c if (pass_type - 0x1f8) & 3 == 0 => 14,
        _ => return,
    };
    let texture = e.global::<u32>(PAIR_TEXTURE_011F95E4);
    let stage = pass_stage(e, first);
    e.mem.set_u32(stage + 8, texture);
    let texture = e.global::<u32>(PAIR_TEXTURE_011F95E8);
    let stage = pass_stage(e, first + 1);
    e.mem.set_u32(stage + 8, texture);
}

/// `NiD3DUtility::GetD3DFromNi` (`00b799a0`, engine map; `cdecl`: matrix, transform), the D3DX
/// import thunks `D3DXMatrixMultiply` (`00ee6de0`), `D3DXMatrixTranspose` (`00ee6dd4`) and
/// `D3DXMatrixInverse` (`00ee6dda`) (engine map names; `stdcall`), and the matrix
/// `SetupGeometryConstants_ObjToCube` writes (`011fecc8`).
const GET_D3D_FROM_NI: u32 = 0x00b7_99a0;
const MATRIX_MULTIPLY: u32 = 0x00ee_6de0;
const MATRIX_TRANSPOSE: u32 = 0x00ee_6dd4;
const MATRIX_INVERSE: u32 = 0x00ee_6dda;
const OBJECT_TO_CUBE_011FECC8: u32 = 0x011f_ecc8;

// Translated from 00b7b730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowLightShader::SetupGeometryConstants_ObjToCube` (engine map), `thiscall` with four
/// stack words: a block (a byte at +9, an object at +0xc), the pass type, a flag and a
/// transform. With vertex entry 1 enabled: without the block's object, with the byte set and the
/// object's first word's byte +0xed set, or for the pass types 0x244 to 0x24b, 0x2d8 to 0x2ef,
/// 0xfe and 0x10b, `011fecc8` becomes the transpose of the renderer's (+0x18) matrix at +0x940
/// (copied to the stack first). Otherwise, with the rendering pass's +0xc set and the byte set:
/// an identity matrix (or the transform's, `GetD3DFromNi`, without the flag) gets the camera
/// (`011f474c`) added to its translation, is multiplied by the first light's matrix (+0x10 of
/// the first word of the rendering pass's +0xc), transposed in place and copied to `011fecc8`.
pub fn shadow_light_shader_setup_geometry_constants_obj_to_cube(
    e: &mut Engine,
    this: Ptr,
    block: Ptr,
    pass_type: u32,
    flag: u32,
    transform: u32,
) {
    if !entry_enabled(e, VERTEX_ENTRIES_011FEBD8 + 4) {
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
            e.call(MATRIX_TRANSPOSE, &args![OBJECT_TO_CUBE_011FECC8, product]);
            return;
        }
        let rendering = e.global::<u32>(RENDERING_PASS_011F91E0);
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
            let camera = x87_load32(e.global::<u32>(CAMERA_011F474C + 4 * index));
            let value = x87_add(camera, x87_m32(e.mem.u32(slot)));
            e.mem.set_u32(slot, x87_store32(value));
        }
        let lights = e.mem.u32(rendering + 0xc);
        let light = e.mem.u32(lights);
        e.call(MATRIX_MULTIPLY, &args![product, first, light + 0x10]);
        let bytes = e.mem.bytes(product, 0x40);
        e.mem.write(second, &bytes);
        e.call(MATRIX_TRANSPOSE, &args![second, second]);
        e.call(MEMCPY, &args![OBJECT_TO_CUBE_011FECC8, second, 0x40u32]);
    });
}

/// The scaled colour of a light as the shaders compute it, from the light object (its scale at
/// +0xd0) and its data (the strength +0xc4, at most 1.0 without HDR, and the colour +0xd4 to
/// +0xdc): (colour * strength) * scale for each channel, each x87 step and float store in the
/// exe's order and operand order.
fn light_colour(e: &Engine, light: u32, values: u32) -> [u32; 3] {
    let strength_bits = copy_float(e, values + 0xc4);
    let mut strength = x87_load32(strength_bits);
    if e.global::<u8>(HDR_011F941E) == 0 && strength > e.global::<f64>(ONE_01012070) {
        strength = x87_load32(1.0f32.to_bits());
    }
    let scale_bits = copy_float(e, light + 0xd0);
    let red = e.mem.u32(values + 0xd4);
    let green = e.mem.u32(values + 0xd8);
    let blue = e.mem.u32(values + 0xdc);
    let red_strength = x87_store32(x87_mul(x87_load32(red), strength));
    let green_strength = x87_store32(x87_mul(x87_load32(green), strength));
    let blue_strength = x87_store32(x87_mul(strength, x87_m32(blue)));
    let scale = x87_load32(scale_bits);
    [
        x87_store32(x87_mul(x87_load32(red_strength), scale)),
        x87_store32(x87_mul(scale, x87_m32(green_strength))),
        x87_store32(x87_mul(scale, x87_m32(blue_strength))),
    ]
}

/// The camera object (`011f91c8`; its position at +0x1e4) and the light vectors the light
/// loops write and upload (`011fd9a8`, 0x10 bytes per light).
const CAMERA_OBJECT_011F91C8: u32 = 0x011f_91c8;
const LIGHT_VECTORS_011FD9A8: u32 = 0x011f_d9a8;

/// One light of the light loops (`00b7c850`, `00b7cb00`): light `index` of the rendering pass
/// (`011f91e0` +0xc) and its data (+0xf8). Its position (+0x8c) plus the camera object's
/// (`011f91c8` +0x1e4), transformed by `matrix` (`D3DXVec3TransformCoord`), with its radius
/// (+0xe0) over `scale` goes to `011fd9a8` row `index` and is uploaded as vertex constant 0x19 +
/// index; row `index` of `011fa0c0` takes the scaled colour and `fourth`.
fn light_row(e: &mut Engine, rendering: u32, index: u32, matrix: u32, scale: u32, fourth: u32) {
    let lights = e.mem.u32(rendering + 0xc);
    let light = e.mem.u32(lights.wrapping_add(index * 4));
    let values = e.mem.u32(light + 0xf8);
    let camera = e.global::<u32>(CAMERA_OBJECT_011F91C8);
    e.with_stack(0x18, |e, frame| {
        let frame = frame.addr();
        let position = frame;
        let transformed = frame + 0xc;
        for axis in 0..3u32 {
            let coordinate = x87_load32(e.mem.u32(values + 0x8c + 4 * axis));
            let offset = x87_m32(e.mem.u32(camera + 0x1e4 + 4 * axis));
            let value = x87_store32(x87_add(coordinate, offset));
            e.mem.set_u32(position + 4 * axis, value);
        }
        e.call(VEC3_TRANSFORM_COORD, &args![transformed, position, matrix]);
        let radius = x87_load32(e.mem.u32(values + 0xe0));
        let range = x87_store32(x87_div(radius, x87_m32(scale)));
        let row = LIGHT_VECTORS_011FD9A8 + index * 0x10;
        for axis in 0..3u32 {
            let value = copy_float(e, transformed + 4 * axis);
            e.mem.set_u32(row + 4 * axis, value);
        }
        e.mem.set_u32(row + 0xc, x87_quiet_bits(range));
        set_vertex_constants(e, index + 0x19, row, 1);
    });
    let colour = light_colour(e, light, values);
    let row = ROWS_011FA0C0 + index * 0x10;
    for (axis, value) in colour.iter().enumerate() {
        e.mem.set_u32(row + 4 * axis as u32, x87_quiet_bits(*value));
    }
    e.mem.set_u32(row + 0xc, fourth);
}

/// `9.0` (`01050c98`), `99.0` (`010af450`) and `30.0` (`01018f5c`), the constants the light
/// loops use.
const NINE_01050C98: u32 = 0x0105_0c98;
const NINETY_NINE_010AF450: u32 = 0x010a_f450;
const THIRTY_01018F5C: u32 = 0x0101_8f5c;

/// The light count byte of the rendering pass (+9) less one, as a float (`FILD`, `FSUB` 1.0).
fn light_count_less_one(e: &Engine, rendering: u32) -> u32 {
    let count = f64::from(e.mem.u8(rendering + 9));
    x87_store32(x87_sub(count, e.global::<f64>(ONE_01012070)))
}

// Translated from 00b7c850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `stdcall` with two words: a matrix and a
/// float (the radius divisor). Vertex constant 0x19 becomes (9, 9, 9, n - 1) with n the
/// rendering pass's light count (+9); pixel entry 2's register count (+0x20) n - 1, and pixel
/// entry 7's when enabled. Lights 1 to n - 1 then each get their row (`light_row`) with n - 1
/// as the fourth colour word.
pub fn fn_00b7c850(e: &mut Engine, matrix: u32, scale: u32) {
    let rendering = e.global::<u32>(RENDERING_PASS_011F91E0);
    let less_one = light_count_less_one(e, rendering);
    e.with_stack(0x10, |e, constant| {
        let constant = constant.addr();
        let nine = copy_float(e, NINE_01050C98);
        for index in 0..3u32 {
            e.mem.set_u32(constant + 4 * index, nine);
        }
        e.mem.set_u32(constant + 0xc, x87_quiet_bits(less_one));
        set_vertex_constants(e, 0x19, constant, 1);
    });
    let count = u32::from(e.mem.u8(rendering + 9)).wrapping_sub(1);
    let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 4 * 2);
    e.mem.set_u32(entry + 0x20, count);
    let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 4 * 7);
    if e.mem.u8(entry + 8) != 0 {
        let count = u32::from(e.mem.u8(rendering + 9)).wrapping_sub(1);
        e.mem.set_u32(entry + 0x20, count);
    }
    let less_one = light_count_less_one(e, rendering);
    let fourth = x87_quiet_bits(less_one);
    let mut index: u8 = 1;
    if e.mem.u8(rendering + 9) > 1 {
        loop {
            light_row(e, rendering, u32::from(index), matrix, scale, fourth);
            index = index.wrapping_add(1);
            if index >= e.mem.u8(rendering + 9) {
                break;
            }
        }
    }
}

/// Row 2 of `011fa0c0` (`011fa0e0`), which the second light loop sets when there are only two
/// lights (the loop then fills only row 1).
const FAR_PLANE_011FA0E0: u32 = 0x011f_a0e0;

// Translated from 00b7cb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `stdcall` with three words: a matrix, a
/// float (the radius divisor) and an object (a float at +0x38) or null. Vertex constant 0x19
/// becomes (99, 99, 99, the float the rendering pass's first object's +0xa8 answers from its
/// vtable +0x10c); pixel entry 2's register count max(2, n - 1) and pixel entry 7's n - 1 when
/// enabled. Lights 1 to n - 1 each get their row (`light_row`): the first with n - 1 as the
/// fourth colour word, the later ones with the object's float (30.0 without one). With exactly
/// two lights row 2 (`011fa0e0`) becomes (0, 0, 0, that float).
pub fn fn_00b7cb00(e: &mut Engine, matrix: u32, scale: u32, object: u32) {
    let rendering = e.global::<u32>(RENDERING_PASS_011F91E0);
    let holder = e.mem.u32(rendering);
    let source = e.mem.u32(holder + 0xa8);
    let value = e.vcall(source, 0x10c, &args![]).f64();
    let value = x87_store32(value);
    e.with_stack(0x10, |e, constant| {
        let constant = constant.addr();
        let ninety_nine = copy_float(e, NINETY_NINE_010AF450);
        for index in 0..3u32 {
            e.mem.set_u32(constant + 4 * index, ninety_nine);
        }
        e.mem.set_u32(constant + 0xc, x87_quiet_bits(value));
        set_vertex_constants(e, 0x19, constant, 1);
    });
    let mut count = u32::from(e.mem.u8(rendering + 9)).wrapping_sub(1);
    if (count as i32) < 2 {
        count = 2;
    }
    let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 4 * 2);
    e.mem.set_u32(entry + 0x20, count);
    let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 4 * 7);
    if e.mem.u8(entry + 8) != 0 {
        let count = u32::from(e.mem.u8(rendering + 9)).wrapping_sub(1);
        e.mem.set_u32(entry + 0x20, count);
    }
    let far = if object != 0 {
        copy_float(e, object + 0x38)
    } else {
        copy_float(e, THIRTY_01018F5C)
    };
    let mut fourth = light_count_less_one(e, rendering);
    let mut index: u8 = 1;
    if e.mem.u8(rendering + 9) > 1 {
        loop {
            let word = x87_quiet_bits(fourth);
            light_row(e, rendering, u32::from(index), matrix, scale, word);
            fourth = x87_quiet_bits(far);
            index = index.wrapping_add(1);
            if index >= e.mem.u8(rendering + 9) {
                break;
            }
        }
    }
    if e.mem.u8(rendering + 9) == 2 {
        for offset in [0u32, 4, 8] {
            e.set_global(FAR_PLANE_011FA0E0 + offset, 0u32);
        }
        e.set_global(FAR_PLANE_011FA0E0 + 0xc, x87_quiet_bits(far));
    }
}

/// The D3DX import thunks the clip planes use (unnamed in the engine map; `stdcall`):
/// `00ee6dce` (plane normalize: out, plane) and `00ee6dc8` (plane transform: out, plane,
/// matrix), as the decompiler names their imports `D3DXPlaneNormalize` and
/// `D3DXPlaneTransform`.
const PLANE_NORMALIZE_00EE6DCE: u32 = 0x00ee_6dce;
const PLANE_TRANSFORM_00EE6DC8: u32 = 0x00ee_6dc8;

/// The six clip planes at `planes` set on the device (renderer `011f4748` +0x288, read for each;
/// its vtable +0xdc with the index and the plane).
fn set_clip_planes(e: &mut Engine, planes: u32) {
    for index in 0..6u32 {
        let renderer = e.global::<u32>(RENDERER_011F4748);
        let device = e.mem.u32(renderer + 0x288);
        e.vcall(device, 0xdc, &args![index, planes + 0x10 * index]);
    }
}

// Translated from 00b7be00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowLightShader::SetupGeometryConstants_SimpleShadowClipPlanes` (engine map), `thiscall`
/// with one stack word, the pass type; only the types 0x231 to 0x234 do anything. The first
/// light (the first word of the rendering pass's +0xc) keeps transformed clip planes at +0x1a8
/// and a byte at +0x208 that says they are made; when it is set the device gets them. Else: the
/// renderer's (+0x18) view matrix (+0x980) on the stack gets the translation -(camera . column)
/// for its three columns (the camera `011f474c`, x87 in the exe's order), is multiplied by the
/// renderer's matrix at +0x9c0, inverted and transposed; each of the light's six planes (+0x144;
/// the fourth word negated) is normalized and transformed by it; the device gets the six, they
/// are copied to +0x1a8 and the byte is set.
pub fn shadow_light_shader_setup_geometry_constants_simple_shadow_clip_planes(
    e: &mut Engine,
    this: Ptr,
    pass_type: u32,
) {
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
        let x = x87_load32(e.global::<u32>(CAMERA_011F474C));
        let y = x87_load32(e.global::<u32>(CAMERA_011F474C + 4));
        let z = x87_load32(e.global::<u32>(CAMERA_011F474C + 8));
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
            e.call(PLANE_NORMALIZE_00EE6DCE, &args![plane, plane]);
            let out = frame + 0x170;
            e.call(PLANE_TRANSFORM_00EE6DC8, &args![out, plane, product]);
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

/// The table of enabled vertex constants by pass type (`011fcc80`).
const VERTEX_FLAGS_011FCC80: u32 = 0x011f_cc80;

/// The 22 entries at `entries` enabled by the bits (entry k: bit k + 1) of `flags`.
fn enable_entries(e: &mut Engine, entries: u32, flags_table: u32, pass_type: u32) {
    let mut mask = 1u32;
    for index in 0..22u32 {
        let entry = e.global::<u32>(entries + 4 * index);
        if entry != 0 {
            let flags = e
                .mem
                .u32(flags_table.wrapping_add(pass_type.wrapping_mul(4)));
            let enabled = flags & mask.rotate_left(1) != 0;
            e.mem.set_u8(entry + 8, u8::from(enabled));
        }
        mask = mask.rotate_left(1);
    }
}

/// The fog constants of `00b7c3a0` and `SetupPassConstants`: with a fog property
/// (`GetFogProperty` of the byte `011f91c4`), `011fa280` becomes (500000.0, 0, 0, 0) and the
/// colour `011a9bd0` when its far (+0x30) and near (+0x2c) distances are both 0, else (far,
/// far - near, its +0x60, 0) and its +0x20, +0x24, +0x28, 0.
fn fog_constants(e: &mut Engine) {
    let index = u32::from(e.global::<u8>(FOG_INDEX_011F91C4));
    let fog = e.call(GET_FOG_PROPERTY, &args![index]).u32();
    if fog == 0 {
        return;
    }
    let near = copy_float(e, fog + 0x2c);
    let far = copy_float(e, fog + 0x30);
    let power = copy_float(e, fog + 0x60);
    let far_value = x87_load32(far);
    let near_value = x87_load32(near);
    if far_value == 0.0 && near_value == 0.0 {
        let distance = copy_float(e, FAR_DISTANCE_010AD6E0);
        e.set_global(FOG_011FA280, distance);
        for offset in [4u32, 8, 0xc] {
            e.set_global(FOG_011FA280 + offset, 0u32);
        }
        let colour: Vec<u32> = (0..4)
            .map(|index| e.global::<u32>(FOG_COLOUR_011A9BD0 + 4 * index))
            .collect();
        e.set_global(FOG_011FA280 + 0x10, colour[0]);
        e.set_global(FOG_011FA280 + 0x1c, colour[3]);
        e.set_global(FOG_011FA280 + 0x18, colour[2]);
        e.set_global(FOG_011FA280 + 0x14, colour[1]);
    } else {
        let red = e.mem.u32(fog + 0x20);
        let green = e.mem.u32(fog + 0x24);
        let span = x87_store32(x87_sub(far_value, near_value));
        let blue = e.mem.u32(fog + 0x28);
        e.set_global(FOG_011FA280, far);
        e.set_global(FOG_011FA280 + 4, span);
        e.set_global(FOG_011FA280 + 8, x87_quiet_bits(power));
        e.set_global(FOG_011FA280 + 0xc, 0u32);
        e.set_global(FOG_011FA280 + 0x10, x87_quiet_bits(red));
        e.set_global(FOG_011FA280 + 0x1c, 0u32);
        e.set_global(FOG_011FA280 + 0x18, x87_quiet_bits(blue));
        e.set_global(FOG_011FA280 + 0x14, x87_quiet_bits(green));
    }
}

/// The three copies of the fog colour `SetupPassConstants` makes (`011fd9b8`, 0x30 bytes).
const FOG_COLOURS_011FD9B8: u32 = 0x011f_d9b8;

// Translated from 00b7b450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowLightShader::SetupPassConstants` (engine map), `thiscall` with one stack word, the
/// pass type (0x250 and 0x251 do nothing). The vertex entries are enabled by the type's word of
/// `011fcc80` and the pixel entries by its word of `011fc0a0` (entry k: bit k + 1), then the
/// vertex (+0x34) and pixel (+0x30) constant maps sorted (`SortByEnabled`, vtable +0x24). With
/// vertex entry 8 enabled the fog constants are made (as `00b7c3a0` makes them). When the
/// rendering pass's first word has +0xbc set, the renderer's (+0x18) matrix at +0xa00 is
/// transposed onto the stack and uploaded as vertex constants 1 to 4. With vertex entry 8 enabled
/// the fog colour `011a9bd0` is copied three times to `011fd9b8`.
pub fn shadow_light_shader_setup_pass_constants(e: &mut Engine, this: Ptr, pass_type: u32) {
    if pass_type == 0x250 || pass_type == 0x251 {
        return;
    }
    let base = this.addr();
    enable_entries(e, VERTEX_ENTRIES_011FEBD8, VERTEX_FLAGS_011FCC80, pass_type);
    enable_entries(e, PIXEL_ENTRIES_011FEC30, PIXEL_FLAGS_011FC0A0, pass_type);
    let vertex_map = e.mem.u32(base + 0x34);
    e.vcall(vertex_map, 0x24, &args![]);
    let pixel_map = e.mem.u32(base + 0x30);
    e.vcall(pixel_map, 0x24, &args![]);
    if entry_enabled(e, VERTEX_ENTRIES_011FEBD8 + 4 * 8) {
        fog_constants(e);
    }
    let rendering = e.global::<u32>(RENDERING_PASS_011F91E0);
    let first = e.mem.u32(rendering);
    if e.mem.u32(first + 0xbc) != 0 {
        let renderer = e.mem.u32(base + 0x18);
        e.with_stack(0x40, |e, matrix| {
            e.call(MATRIX_TRANSPOSE, &args![matrix, renderer + 0xa00]);
            set_vertex_constants(e, 1, matrix.addr(), 4);
        });
    }
    if entry_enabled(e, VERTEX_ENTRIES_011FEBD8 + 4 * 8) {
        let colour: Vec<u32> = (0..4)
            .map(|index| e.global::<u32>(FOG_COLOUR_011A9BD0 + 4 * index))
            .collect();
        for copy in 0..3u32 {
            for (index, word) in colour.iter().enumerate() {
                let addr = FOG_COLOURS_011FD9B8 + 0x10 * copy + 4 * index as u32;
                e.set_global(addr, *word);
            }
        }
    }
}

/// The shader mode the shaders switch on (`011f91b8`), the shader manager's object `00b4f5d0`
/// answers (unnamed, no arguments; its colour at +0x154), the per-light set-up `00b70820`
/// (unnamed, `cdecl`: index, light, a float), `BSShaderLightingProperty::GetFirstActiveLight`
/// (`00b70590`, engine map; `thiscall`, one word), the light positions and directions the
/// set-up leaves (`011fa170` and `011fa1f0`, 0x10 bytes per light), `0.5` (`01016248`), the
/// HDR light scale (`011f9184`) and the colour rows `SetupGeometryConstants_Lights` writes
/// (`011fa0d0`).
const SHADER_MODE_011F91B8: u32 = 0x011f_91b8;
const MANAGER_OBJECT_00B4F5D0: u32 = 0x00b4_f5d0;
const LIGHT_SETUP_00B70820: u32 = 0x00b7_0820;
const GET_FIRST_ACTIVE_LIGHT: u32 = 0x00b7_0590;
const LIGHT_POSITIONS_011FA170: u32 = 0x011f_a170;
const LIGHT_DIRECTIONS_011FA1F0: u32 = 0x011f_a1f0;
const HALF_01016248: u32 = 0x0101_6248;
const HDR_LIGHT_SCALE_011F9184: u32 = 0x011f_9184;

/// The uniform-form entry of `SetupGeometryConstants_Lights`: the property, the rendering
/// object, the sixteen words of the matrix, the scale object, the pass type and the flag.
fn lights_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let mut matrix = [0u32; 16];
    matrix.copy_from_slice(&words[2..18]);
    shadow_light_shader_setup_geometry_constants_lights(
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

// Translated from 00b78a90 (decompiled, FalloutNV.exe 1.4.0.525)
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
pub fn shadow_light_shader_setup_geometry_constants_lights(
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
    if e.global::<u32>(SHADER_MODE_011F91B8) == 0xf {
        let object = e.call(MANAGER_OBJECT_00B4F5D0, &args![]).u32();
        for index in 0..4u32 {
            let word = e.mem.u32(object + 0x154 + 4 * index);
            e.set_global(ROWS_011FA0C0 + 4 * index, word);
        }
        return;
    }
    let count = u32::from(e.mem.u8(rendering + 9));
    let kind = e.mem.u16(rendering + 4);
    let entry = e.global::<u32>(VERTEX_ENTRIES_011FEBD8 + 4 * 6);
    if e.mem.u8(entry + 8) != 0 {
        let registers = if (0x10..=0x13).contains(&kind) {
            1
        } else {
            count
        };
        e.mem.set_u32(entry + 0x20, registers);
    }
    let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 4 * 2);
    if e.mem.u8(entry + 8) != 0 {
        e.mem.set_u32(entry + 0x20, count);
    }
    let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 4 * 7);
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
                e.mem.set_u32(by_value + 4 * word as u32, *value);
            }
            loop {
                if index >= 10 {
                    break;
                }
                let i = u32::from(index);
                let lights = e.mem.u32(rendering + 0xc);
                let light = e.mem.u32(lights.wrapping_add(i * 4));
                e.call(
                    LIGHT_SETUP_00B70820,
                    &args![i, light, x87_quiet_bits(light_float)],
                );
                let row = LIGHT_VECTORS_011FD9A8 + 0x10 * i;
                let position = LIGHT_POSITIONS_011FA170 + 0x10 * i;
                let direction = LIGHT_DIRECTIONS_011FA1F0 + 0x10 * i;
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
                        let camera = e.global::<u32>(CAMERA_OBJECT_011F91C8);
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
    let mut scale_colour = e.global::<u8>(HDR_011F941E) != 0
        && e.mem.u32(property + 0x20) & 0x400 != 0
        && e.mem.u8(rendering + 9) != 0;
    if scale_colour {
        let lights = e.mem.u32(rendering + 0xc);
        let first = e.mem.u32(lights);
        scale_colour = e.mem.u8(first + 0xf4) == 0;
    }
    if scale_colour {
        let scale = x87_load32(e.global::<u32>(HDR_LIGHT_SCALE_011F9184));
        let red = x87_load32(e.global::<u32>(COLOUR_011FA0D0));
        let green = e.global::<u32>(COLOUR_011FA0D0 + 4);
        let blue = e.global::<u32>(COLOUR_011FA0D0 + 8);
        let alpha = e.global::<u32>(COLOUR_011FA0D0 + 0xc);
        e.set_global(COLOUR_011FA0D0, x87_store32(x87_mul(red, scale)));
        let value = x87_store32(x87_mul(x87_load32(green), scale));
        e.set_global(COLOUR_011FA0D0 + 4, value);
        let value = x87_store32(x87_mul(x87_load32(blue), scale));
        e.set_global(COLOUR_011FA0D0 + 8, value);
        let value = x87_store32(x87_mul(scale, x87_m32(alpha)));
        e.set_global(COLOUR_011FA0D0 + 0xc, value);
    }
    if pass_type != 0x24b {
        return;
    }
    let mode = e.global::<u32>(SHADER_MODE_011F91B8);
    e.with_stack(4, |e, slot| {
        let light = e.call(GET_FIRST_ACTIVE_LIGHT, &args![property, slot]).u32();
        if light != 0 && e.mem.u8(light + 0xf4) != 0 {
            return;
        }
        if mode == 10 {
            let half = copy_float(e, HALF_01016248);
            for offset in [0u32, 4, 8] {
                e.set_global(COLOUR_011FA0D0 + offset, half);
            }
            e.set_global(COLOUR_011FA0D0 + 0xc, 1.0f32.to_bits());
        } else {
            for offset in [0u32, 4, 8, 0xc] {
                let word = e.global::<u32>(ROWS_011FA0C0 + offset);
                e.set_global(COLOUR_011FA0D0 + offset, word);
            }
        }
    });
}

/// The ambient helpers (`stdcall`, one word: the shader's word): `00c03180` with pixel entry 3
/// enabled outside types 0xc3 to 0xd6, else `00c03010` (both unnamed).
const AMBIENT_00C03180: u32 = 0x00c0_3180;
const AMBIENT_00C03010: u32 = 0x00c0_3010;
/// The two floats `SetupGeometryConstants_AmbientEmittance` sets from the light count for
/// types 0xc3 to 0xd6 (`011fd95c`, `011fa27c`), the byte that makes the ambient row white
/// (`011f91a7`), `BSShaderLightingProperty::GetFirstActiveNonShadowLight` (`00b70600`, engine
/// map; `thiscall`, one word), the three luminance weights (`double`s at `010ae8c0`, `010ae8b8`,
/// `010ae8b0` for the second, first and third channel) and the helpers it calls (unnamed):
/// `00439ef0` (`thiscall` on a point: out, other point), `00633b40` (`thiscall` on a colour:
/// out, a float), `00457990` (`thiscall` on a point, the result in ST0), `0068b050` (`cdecl`: a
/// `double` and a float, the result in ST0), `00b719c0` (`cdecl`: two `double`s, the result in
/// ST0) and `00b719e0` (`thiscall` on a colour, the result in ST0).
const LIGHT_COUNT_011FD95C: u32 = 0x011f_d95c;
const LIGHT_COUNT_011FA27C: u32 = 0x011f_a27c;
const WHITE_AMBIENT_011F91A7: u32 = 0x011f_91a7;
const GET_FIRST_ACTIVE_NON_SHADOW_LIGHT: u32 = 0x00b7_0600;
const WEIGHT_SECOND_010AE8C0: u32 = 0x010a_e8c0;
const WEIGHT_FIRST_010AE8B8: u32 = 0x010a_e8b8;
const WEIGHT_THIRD_010AE8B0: u32 = 0x010a_e8b0;
const POINT_DIFFERENCE_00439EF0: u32 = 0x0043_9ef0;
const COLOUR_SCALE_00633B40: u32 = 0x0063_3b40;
const POINT_LENGTH_00457990: u32 = 0x0045_7990;
const FLOAT_HELPER_0068B050: u32 = 0x0068_b050;
const DOUBLE_HELPER_00B719C0: u32 = 0x00b7_19c0;
const COLOUR_LUMINANCE_00B719E0: u32 = 0x00b7_19e0;

// Translated from 00b7ba70 (decompiled, FalloutNV.exe 1.4.0.525)
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
pub fn shadow_light_shader_setup_geometry_constants_ambient_emittance(
    e: &mut Engine,
    _this: Ptr,
    property: Ptr,
    rendering: Ptr,
    block: Ptr,
    word: u32,
) {
    let property = property.addr();
    let rendering = rendering.addr();
    let current = e.global::<u32>(RENDERING_PASS_011F91E0);
    let colour: Vec<u32> = (0..3)
        .map(|index| copy_float(e, ROWS_011FA0C0 + 4 * index))
        .collect();
    let alpha = copy_float(e, property + 0x28);
    for (index, value) in colour.iter().enumerate() {
        e.set_global(ROWS_011FA0C0 + 4 * index as u32, *value);
    }
    e.set_global(ROWS_011FA0C0 + 0xc, alpha);
    let kind = e.mem.u16(rendering + 4);
    let helper =
        if entry_enabled(e, PIXEL_ENTRIES_011FEC30 + 4 * 3) && !(0xc3..=0xd6).contains(&kind) {
            AMBIENT_00C03180
        } else {
            AMBIENT_00C03010
        };
    e.call(helper, &args![word]);
    let kind = e.mem.u16(rendering + 4);
    if (0xc3..=0xd6).contains(&kind) {
        let count = x87_store32(f64::from(e.mem.u8(rendering + 9)));
        e.set_global(LIGHT_COUNT_011FD95C, count);
        let count = x87_store32(f64::from(e.mem.u8(rendering + 9)));
        e.set_global(LIGHT_COUNT_011FA27C, count);
    }
    if e.global::<u8>(WHITE_AMBIENT_011F91A7) != 0 {
        for offset in [0u32, 4, 8, 0xc] {
            e.set_global(ROWS_011FA0C0 + offset, 1.0f32.to_bits());
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
            let camera = e.global::<u32>(CAMERA_OBJECT_011F91C8);
            let camera_light = e.mem.u32(camera + 0xe0);
            let values = e.mem.u32(camera_light + 0xf8);
            let second = x87_mul(
                x87_load32(e.mem.u32(values + 0xd8)),
                e.global::<f64>(WEIGHT_SECOND_010AE8C0),
            );
            let first = x87_mul(
                x87_load32(e.mem.u32(values + 0xd4)),
                e.global::<f64>(WEIGHT_FIRST_010AE8B8),
            );
            let sum = x87_add(second, first);
            let third = x87_mul(
                x87_load32(e.mem.u32(values + 0xdc)),
                e.global::<f64>(WEIGHT_THIRD_010AE8B0),
            );
            let luminance = x87_store32(x87_add(sum, third));
            let mut light_part = 0.0f32.to_bits();
            if light != 0 {
                let current = e.global::<u32>(RENDERING_PASS_011F91E0);
                let first_word = e.mem.u32(current);
                let mut position = e.mem.u32(first_word + 0x20);
                if position == 0 {
                    position = DEFAULT_POSITION_011F4288;
                }
                let values = e.mem.u32(light + 0xf8);
                e.call(
                    POINT_DIFFERENCE_00439EF0,
                    &args![values + 0x8c, difference, position],
                );
                let strength = copy_float(e, values + 0xc4);
                e.call(
                    COLOUR_SCALE_00633B40,
                    &args![values + 0xd4, scaled, strength],
                );
                let length = e.call(POINT_LENGTH_00457990, &args![difference]).f64();
                let ratio = x87_store32(x87_div(length, x87_m32(e.mem.u32(values + 0xe0))));
                let raised = e
                    .call(FLOAT_HELPER_0068B050, &args![0.0f64, x87_quiet_bits(ratio)])
                    .f64();
                let limited = e.call(DOUBLE_HELPER_00B719C0, &args![1.0f64, raised]).f64();
                let limited = x87_store32(limited);
                let falloff = x87_store32(x87_sub(1.0, x87_load32(limited)));
                let falloff = x87_quiet_bits(falloff);
                let brightness = e.call(COLOUR_LUMINANCE_00B719E0, &args![scaled]).f64();
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
            let fourth = e.global::<u32>(ROWS_011FA0C0 + 0xc);
            let value = x87_mul(x87_load32(chosen), x87_m32(fourth));
            e.set_global(ROWS_011FA0C0 + 0xc, x87_store32(value));
        });
    }
    let mode = e.global::<u32>(SHADER_MODE_011F91B8);
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
        e.set_global(ROWS_011FA0C0, 0u32);
        e.set_global(ROWS_011FA0C0 + 4, 0u32);
        e.set_global(ROWS_011FA0C0 + 8, blue);
        e.set_global(ROWS_011FA0C0 + 0xc, alpha);
    }
    let pass_type = e.global::<u32>(RENDERING_PASS_TYPE_011F91E4);
    if pass_type.wrapping_sub(0x244) > 7 || mode == 10 {
        return;
    }
    let factor = e.vcall(property, 0x110, &args![]).f64();
    let fourth = e.global::<u32>(ROWS_011FA0C0 + 0xc);
    let value = x87_mul(factor, x87_m32(fourth));
    e.set_global(ROWS_011FA0C0 + 0xc, x87_store32(value));
}

/// The texture the stage of `00b7d200` body 1 gets without one (`011f952c`) and the stage
/// helper it calls then (`00b71b10`, unit `shadowlightshader.cpp`, `thiscall`: stage, property).
const DEFAULT_TEXTURE_011F952C: u32 = 0x011f_952c;
const STAGE_HELPER_00B71B10: u32 = 0x00b7_1b10;

/// The texture the stage `index` of the pass's stages (`stages`, the pass's +0x24 array) gets
/// in `00b7d200`: the property's first texture (`[+0xb4][0]`), `011f951c` when it is null.
fn first_texture_or_fallback(e: &mut Engine, stages: u32, index: u32, property: u32) {
    let textures = e.mem.u32(property + 0xb4);
    let texture = e.mem.u32(textures);
    let stage = e.mem.u32(stages + 4 * index);
    let value = if texture != 0 {
        texture
    } else {
        e.global::<u32>(FALLBACK_011F951C)
    };
    e.mem.set_u32(stage + 8, value);
}

// Translated from 00b7d200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`): the shader's vtable +0x78 (the slot the
/// Xbox PDB calls `SetupGeometryTextures`), `thiscall` with one stack word, a block whose +0xc is
/// the shader property. By the rendering object's type (`011f91e0`, +4; the exe's two-level jump
/// table on type - 2, translated as match arms) it puts the property's textures on the current
/// pass's stages through `00b7c0a0`, `00b7c0e0`, `00b7c290`, `00b7c2c0`, `00b7c120` and `00b7c190`,
/// stores first textures directly, or copies the property's texture arrays (+0xac, and +0xb0
/// without bit 0x100000 of its +0x24) onto stages 0 to 6 and 7 to 13; then, for every type in
/// the table, the shader's vtable +0x108. Types outside the table do nothing.
pub fn fn_00b7d200(e: &mut Engine, this: Ptr, block: Ptr) {
    let base = this.addr();
    let rendering = e.global::<u32>(RENDERING_PASS_011F91E0);
    let kind = u32::from(e.mem.u16(rendering + 4));
    let property = e.mem.u32(block.addr() + 0xc);
    let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
    let second_set = e.mem.u32(property + 0x24) & 0x100000 == 0;
    let p = Ptr::new(property);
    let body = match kind {
        0x2
        | 0x6..=0x8
        | 0x11
        | 0x13
        | 0x58
        | 0x5a
        | 0x5c
        | 0x5e
        | 0x62..=0x63
        | 0x14a
        | 0x14d..=0x14f
        | 0x253
        | 0x255
        | 0x2de..=0x2df
        | 0x2ea..=0x2eb => 0,
        0x4..=0x5 => 1,
        0x59 | 0x5b | 0x5d | 0x5f | 0x1f1 | 0x1f4 => 2,
        0x64..=0x65
        | 0x69..=0x6a
        | 0x6f..=0x70
        | 0x74..=0x75
        | 0x7f..=0x80
        | 0x83..=0x84
        | 0x89..=0x8a
        | 0x8d..=0x8e
        | 0xc3
        | 0xc6
        | 0xc9
        | 0xcd
        | 0xd7
        | 0xdc
        | 0xe0
        | 0xe5
        | 0xe9
        | 0xee
        | 0xf2
        | 0xf7
        | 0xfb
        | 0x105
        | 0x10a
        | 0x111
        | 0x116
        | 0x11c
        | 0x121
        | 0x127
        | 0x235 => 3,
        0x66..=0x67
        | 0x6b..=0x6c
        | 0x71..=0x72
        | 0x76..=0x77
        | 0x81..=0x82
        | 0x86..=0x87
        | 0x8b..=0x8c
        | 0x90..=0x91 => 4,
        0x68 | 0x6d | 0x73 | 0x78 | 0x85 | 0x8f => 5,
        0x7c..=0x7d
        | 0x12c..=0x12f
        | 0x132
        | 0x135
        | 0x13e
        | 0x141
        | 0x154
        | 0x156
        | 0x1e2..=0x1e4 => 6,
        0x93..=0x94 | 0x1ef..=0x1f0 | 0x1f2..=0x1f3 | 0x1f5..=0x1f6 => 7,
        0x95 | 0x9b | 0xa0 | 0xa6 | 0xab | 0xb1 | 0xb6 | 0xbc | 0xd0 | 0xd4 | 0x10b => 8,
        0x96 | 0x9c | 0xa1 | 0xa7 | 0xac | 0xb2 | 0xb7 | 0xbd | 0xd2 | 0xd5 => 9,
        0x97 | 0x9d | 0xa2 | 0xa8 | 0xad | 0xb3 | 0xb8 | 0xbe | 0x104 | 0x109 | 0x110 | 0x115
        | 0x11b | 0x120 | 0x126 | 0x12b => 10,
        0xc4 | 0xc7 | 0xcb | 0xce | 0xff | 0x106 | 0x10c | 0x112 | 0x117 | 0x11d | 0x122
        | 0x128 => 11,
        0xd8 | 0xdd | 0xe1 | 0xe6 | 0xea | 0xef | 0xf3 | 0xf8 => 12,
        0xfe => 13,
        0x14b | 0x150 => 14,
        0x14c | 0x152 => 15,
        0x155
        | 0x157
        | 0x15a..=0x15b
        | 0x15d..=0x15e
        | 0x160..=0x161
        | 0x163..=0x164
        | 0x166..=0x167
        | 0x169..=0x16a
        | 0x16c..=0x16d
        | 0x16f..=0x170
        | 0x172..=0x173
        | 0x175..=0x176
        | 0x24b => 16,
        0x1f7..=0x1fe => 17,
        0x1ff..=0x206 => 18,
        0x207..=0x20e => 19,
        0x20f..=0x216 => 20,
        0x217..=0x21e => 21,
        0x21f..=0x226 => 22,
        0x227..=0x22e => 23,
        0x230 => 24,
        0x231..=0x233 => 25,
        0x244..=0x247 => 26,
        0x248..=0x24a => 27,
        0x2d6..=0x2d7 => 28,
        _ => return,
    };
    let stages = e.mem.u32(pass + 0x24);
    match body {
        0 => fn_00b7c0a0(e, p, 0, 0),
        1 => {
            let stage = e.mem.u32(stages);
            let textures = e.mem.u32(property + 0xdc);
            let mut texture = 0;
            if textures != 0 {
                texture = e.mem.u32(textures + 0x6c);
            }
            if texture == 0 {
                texture = e.global::<u32>(DEFAULT_TEXTURE_011F952C);
            }
            e.mem.set_u32(stage + 8, texture);
            e.call(STAGE_HELPER_00B71B10, &args![base, stage, property]);
        }
        2 => {
            fn_00b7c0a0(e, p, 0, 0);
            let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
            let stages = e.mem.u32(pass + 0x24);
            first_texture_or_fallback(e, stages, 1, property);
        }
        3 => {
            fn_00b7c0e0(e, p, 1, 0);
            fn_00b7c0a0(e, p, 0, 0);
        }
        4 => {
            first_texture_or_fallback(e, stages, 2, property);
            fn_00b7c0e0(e, p, 1, 0);
            fn_00b7c0a0(e, p, 0, 0);
        }
        5 => {
            fn_00b7c0a0(e, p, 0, 0);
            fn_00b7c0e0(e, p, 1, 0);
            fn_00b7c0a0(e, p, 2, 1);
            fn_00b7c0e0(e, p, 3, 1);
        }
        6 | 16 => fn_00b7c0e0(e, p, 0, 0),
        7 | 24 => {
            let texture = u32::from(e.mem.u8(rendering + 0xb));
            fn_00b7c0a0(e, p, 0, texture);
            fn_00b7c0e0(e, p, 1, texture);
            if body == 24 {
                fn_00b7c190(e, 2);
            }
        }
        8 => {
            fn_00b7c0a0(e, p, 0, 0);
            fn_00b7c0e0(e, p, 1, 0);
        }
        9 => {
            first_texture_or_fallback(e, stages, 4, property);
            fn_00b7c0a0(e, p, 0, 0);
            fn_00b7c0e0(e, p, 1, 0);
        }
        10 => {
            fn_00b7c0a0(e, p, 0, 0);
            fn_00b7c0e0(e, p, 1, 0);
            fn_00b7c0a0(e, p, 4, 1);
        }
        11 | 12 => {
            fn_00b7c0a0(e, p, 0, 0);
            fn_00b7c0e0(e, p, 1, 0);
            let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
            let stages = e.mem.u32(pass + 0x24);
            let index = if body == 11 { 4 } else { 3 };
            first_texture_or_fallback(e, stages, index, property);
        }
        13 => {
            fn_00b7c0a0(e, p, 0, 0);
            fn_00b7c0e0(e, p, 1, 0);
            fn_00b7c0a0(e, p, 4, 1);
            fn_00b7c0e0(e, p, 6, 1);
            fn_00b7c190(e, 7);
        }
        14 => {
            fn_00b7c0a0(e, p, 0, 0);
            fn_00b7c0a0(e, p, 1, 1);
            fn_00b7c0e0(e, p, 2, 1);
        }
        15 => {
            fn_00b7c0a0(e, p, 0, 0);
            fn_00b7c0a0(e, p, 1, 1);
        }
        17..=23 => {
            for index in (0..=(body - 17)).rev() {
                let first = e.mem.u32(property + 0xac);
                let stage = e.mem.u32(stages + 4 * index);
                let texture = e.mem.u32(first + 4 * index);
                e.mem.set_u32(stage + 8, texture);
                if second_set {
                    let second = e.mem.u32(property + 0xb0);
                    let stage = e.mem.u32(stages + 4 * (index + 7));
                    let texture = e.mem.u32(second + 4 * index);
                    e.mem.set_u32(stage + 8, texture);
                }
            }
        }
        25 => {
            fn_00b7c0a0(e, p, 0, 0);
            fn_00b7c120(e, 6);
        }
        26 | 27 => {
            let (first, second) = if body == 26 { (2, 3) } else { (3, 1) };
            fn_00b7c2c0(e, p, first, 0);
            fn_00b7c290(e, property, second, 0);
            fn_00b7c0e0(e, p, 0, 0);
        }
        // Body 28.
        _ => first_texture_or_fallback(e, stages, 0, property),
    }
    e.vcall(base, 0x108, &args![]);
}

/// The model-to-world helper `00c4c2d0` (unnamed, `cdecl`: transform, skin, matrix out) and the
/// two pass set-ups `SetupGeometryConstants` runs for types 4 and 5 (`00b7ad00`) and 0x250 and
/// 0x251 (`00b7ce90`) (unit `shadowlightshader.cpp`, `thiscall`: geometry, property, rendering
/// object).
const MODEL_MATRIX_00C4C2D0: u32 = 0x00c4_c2d0;
const PASS_SETUP_00B7AD00: u32 = 0x00b7_ad00;
const PASS_SETUP_00B7CE90: u32 = 0x00b7_ce90;

// Translated from 00b7dab0 (decompiled, FalloutNV.exe 1.4.0.525)
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
pub fn shadow_light_shader_setup_geometry_constants(e: &mut Engine, this: Ptr, block: Ptr) {
    let base = this.addr();
    let block = block.addr();
    let rendering = e.global::<u32>(RENDERING_PASS_011F91E0);
    let geometry = e.mem.u32(rendering);
    let pass_type = e.global::<u32>(RENDERING_PASS_TYPE_011F91E4);
    let mut position = e.mem.u32(geometry + 0x20);
    if position == 0 {
        position = DEFAULT_POSITION_011F4288;
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
        e.call(SET_Z_WRITE_ENABLE, &args![0u32, 1u32]);
        e.call(RENDER_STATE_00B98380, &args![0u32, 1u32]);
    }
    let transform = geometry + 0x68;
    e.with_stack(0x40, |e, matrix| {
        let matrix_addr = matrix.addr();
        e.call(MODEL_MATRIX_00C4C2D0, &args![transform, skin, matrix_addr]);
        if skin == 0 {
            fn_00b7c750(e, this, geometry, transform);
        }
        let mut words = [0u32; 16];
        for (index, word) in words.iter_mut().enumerate() {
            *word = e.mem.u32(matrix_addr + 4 * index as u32);
        }
        if pass_type == 0x250 || pass_type == 0x251 {
            e.call(
                PASS_SETUP_00B7CE90,
                &args![base, geometry, property, rendering],
            );
        } else if pass_type == 4 || pass_type == 5 {
            e.call(
                PASS_SETUP_00B7AD00,
                &args![base, geometry, property, rendering],
            );
        } else {
            let p = Ptr::new(property);
            let r = Ptr::new(rendering);
            shadow_light_shader_setup_geometry_constants_lights(
                e,
                p,
                r,
                words,
                Ptr::new(transform),
                pass_type,
                skin,
            );
            shadow_light_shader_setup_geometry_constants_ambient_emittance(
                e,
                this,
                p,
                r,
                Ptr::new(geometry),
                fog_word,
            );
            shadow_light_shader_setup_geometry_constants_obj_to_cube(
                e, this, r, pass_type, skin, transform,
            );
            shadow_light_shader_setup_geometry_constants_shadow_proj(
                e,
                this,
                Ptr::new(block),
                skin,
                Ptr::new(transform),
            );
            shadow_light_shader_setup_geometry_constants_eye_pos(
                e,
                pass_type,
                Ptr::new(transform),
                words,
            );
            fn_00b79050(e, this, skin);
            fn_00b790d0(e, this, p);
            fn_00b79180(e, this, Ptr::new(geometry));
            fn_00b79100(e, this, geometry);
            fn_00b791f0(e, this, pass_type, p, Ptr::new(geometry));
            fn_00b79410(e, this, pass_type, r);
            shadow_light_shader_setup_geometry_constants_toggles(
                e,
                this,
                pass_type,
                geometry,
                p,
                fog_word,
                rendering,
                Ptr::new(first_property),
            );
            fn_00b796f0(e, this, p);
            fn_00b79730(e);
            shadow_light_shader_setup_geometry_constants_simple_shadow_clip_planes(
                e, this, pass_type,
            );
            fn_00b798c0(e, this, p);
        }
    });
    for (map_offset, program_offset) in [(0x30u32, 0x44u32), (0x34, 0x5c)] {
        let map = e.mem.u32(base + map_offset);
        if map == 0 {
            continue;
        }
        let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
        let program = e.mem.u32(pass + program_offset);
        if program == 0 {
            continue;
        }
        e.vcall(
            map,
            0x78,
            &args![
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
            ],
        );
    }
    let other = e.mem.u32(block + 0x10);
    if other != 0 && e.mem.u8(other + 0x18) & 1 != 0 {
        let object = e.mem.u32(base + 0x1c);
        e.vcall(object, 0x24, &args![other]);
    }
}

/// One step of `CreateConstantMaps` on one of the shader's constant maps: a vtable call on the
/// map (the slot and the words pushed for it; slot +8 `AddEntry`, +0x68 `AddPredefinedEntry` in
/// the Xbox PDB order), or `GetEntry` (+0x70) of a name whose answer is assigned to an
/// `NiPointer` global (a constant-map entry pointer).
pub(crate) enum MapStep {
    Call(u32, &'static [u32]),
    Entry(u32, u32),
}

/// The Ni allocation `00aa13e0` (`cdecl`, size) and the constant map constructor `00b7e330`
/// (unnamed, `thiscall`: the renderer, a program type).
const NI_ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const MAP_CONSTRUCT_00B7E330: u32 = 0x00b7_e330;

/// A new constant map of `program_type` (0x44 bytes, `00b7e330` with the shader's renderer at
/// +0x18; null when the allocation fails) assigned to the `NiPointer` at `this + offset`.
fn new_constant_map(e: &mut Engine, base: u32, offset: u32, program_type: u32) {
    let block = e.call(NI_ALLOCATE_00AA13E0, &args![0x44u32]).u32();
    let map = if block != 0 {
        let renderer = e.mem.u32(base + 0x18);
        e.call(
            MAP_CONSTRUCT_00B7E330,
            &args![block, renderer, program_type],
        )
        .u32()
    } else {
        0
    };
    ni_pointer_assign(e, base + offset, map);
}

/// The steps run in order on the map at `this + offset` (read again for each step).
fn run_map_steps(e: &mut Engine, base: u32, offset: u32, steps: &[MapStep]) {
    for step in steps {
        let map = e.mem.u32(base + offset);
        match *step {
            MapStep::Call(slot, words) => {
                e.vcall(map, slot, words);
            }
            MapStep::Entry(name, global) => {
                let entry = e.vcall(map, 0x70, &args![name]).u32();
                ni_pointer_assign(e, global, entry);
            }
        }
    }
}

/// Section 1: the steps on the map at +0x34, in the exe order.
const MAP_STEPS_1: &[MapStep] = &[
    MapStep::Call(0x68, &[0x10a1e50, 0x20000009, 0, 0x1011584, 0, 0, 0, 0]),
    MapStep::Call(0x68, &[0x10a1d20, 0x20000009, 0x1, 0x1011584, 0, 0, 0, 0]),
    MapStep::Call(
        0x8,
        &[
            0x10af66c, 0x10000009, 0, 0x12, 0x4, 0x1011584, 0x40, 0x4, 0x11fd968, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10a1728, 0x10000009, 0, 0x9, 0x4, 0x1011584, 0x40, 0x4, 0x11fec88, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10a1f08, 0x10000009, 0, 0x8, 0x4, 0x1011584, 0x40, 0x4, 0x11fecc8, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af65c, 0x10000007, 0, 0x16, 0x1, 0x1011584, 0x10, 0x4, 0x11fd874, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af648, 0x10000007, 0, 0x17, 0x1, 0x1011584, 0x10, 0x4, 0x11fd884, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af638, 0x10000007, 0, 0xc, 0x1, 0x1011584, 0x10, 0x4, 0x11fa2e0, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af62c, 0x10000009, 0, 0x19, 0xa, 0x1011584, 0xa0, 0x4, 0x11fd9a8, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af60c, 0x10000007, 0, 0xe, 0x1, 0x1011584, 0x10, 0x4, 0x11fa280, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af5e4, 0x10000007, 0, 0xf, 0x1, 0x1011584, 0x10, 0x4, 0x11fa290, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af5d8, 0x10000007, 0, 0x10, 0x1, 0x1011584, 0x10, 0x4, 0x11fd894, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af5cc, 0x10000007, 0, 0x18, 0x1, 0x1011584, 0x10, 0x4, 0x11fd8b4, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af5bc, 0x10000007, 0, 0x13, 0x1, 0x1011584, 0x10, 0x4, 0x11fa0b0, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af5b0, 0x10000007, 0, 0x11, 0x1, 0x1011584, 0x10, 0x4, 0x11fd954, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af5a0, 0x10000009, 0, 0x23, 0x4, 0x1011584, 0x40, 0x4, 0x11fed08, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af590, 0x10000009, 0, 0x27, 0x4, 0x1011584, 0x40, 0x4, 0x11fed48, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[0x10a1cac, 0x20000009, 0x120000, 0x2c, 0x36, 0, 0, 0, 0, 0],
    ),
    MapStep::Call(0x68, &[0x10a1ee0, 0x20000009, 0x4, 0x1011584, 0, 0, 0, 0]),
    MapStep::Call(0x68, &[0x10a1db8, 0x20000009, 0x5, 0x1011584, 0, 0, 0, 0]),
    MapStep::Call(
        0x8,
        &[
            0x1097e74, 0x10000004, 0, 0xd, 0x1, 0x1011584, 0x4, 0x4, 0x11fd868, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af580, 0x10000009, 0, 0x62, 0x36, 0x1011584, 0x360, 0x4, 0x11fa580, 0,
        ],
    ),
    MapStep::Entry(0x10a1e50, 0x11febd8),
    MapStep::Entry(0x10a1f08, 0x11febdc),
    MapStep::Entry(0x10a1728, 0x11fec08),
    MapStep::Entry(0x10a1d20, 0x11febe0),
    MapStep::Entry(0x10af66c, 0x11febe4),
    MapStep::Entry(0x10af65c, 0x11febe8),
    MapStep::Entry(0x10af648, 0x11febec),
    MapStep::Entry(0x10af638, 0x11fec18),
    MapStep::Entry(0x10af62c, 0x11febf0),
    MapStep::Entry(0x10af60c, 0x11febf4),
    MapStep::Entry(0x10af5e4, 0x11febf8),
    MapStep::Entry(0x10af5d8, 0x11febfc),
    MapStep::Entry(0x10a1cac, 0x11fec00),
    MapStep::Entry(0x10a1ee0, 0x11fec0c),
    MapStep::Entry(0x10a1db8, 0x11fec10),
    MapStep::Entry(0x1097e74, 0x11fec14),
    MapStep::Entry(0x10af5cc, 0x11fec1c),
    MapStep::Entry(0x10af5bc, 0x11fec2c),
    MapStep::Entry(0x10af5b0, 0x11fec20),
    MapStep::Entry(0x10af5a0, 0x11fec24),
    MapStep::Entry(0x10af590, 0x11fec28),
    MapStep::Entry(0x10af580, 0x11fec04),
];

/// Section 2: the steps on the map at +0x30, in the exe order.
const MAP_STEPS_2: &[MapStep] = &[
    MapStep::Call(
        0x8,
        &[
            0x10af578, 0x10000007, 0, 0, 0x1, 0x1011584, 0x10, 0x4, 0x11fd8a4, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af568, 0x10000007, 0, 0x1, 0x1, 0x1011584, 0x10, 0x4, 0x11fa0c0, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af55c, 0x10000009, 0, 0x3, 0xa, 0x1011584, 0xa0, 0x4, 0x11fa0d0, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af54c, 0x10000007, 0, 0x12, 0x1, 0x1011584, 0x10, 0x4, 0x11fd9a8, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af53c, 0x10000007, 0, 0x13, 0x1, 0x1011584, 0x10, 0x4, 0x11fd9b8, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af52c, 0x10000007, 0, 0x14, 0x1, 0x1011584, 0x10, 0x4, 0x11fd9c8, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af51c, 0x10000007, 0, 0x15, 0x1, 0x1011584, 0x10, 0x4, 0x11fd9d8, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af50c, 0x10000007, 0, 0x16, 0x1, 0x1011584, 0x10, 0x4, 0x11fd9e8, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af4fc, 0x10000007, 0, 0x2, 0x1, 0x1011584, 0x10, 0x4, 0x11fa270, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af5d8, 0x10000007, 0, 0x11, 0x1, 0x1011584, 0x10, 0x4, 0x11fd894, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af4f4, 0x10000007, 0, 0x1b, 0x1, 0x1011584, 0x10, 0x4, 0x11fa2d0, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x1043608, 0x10000007, 0, 0x1c, 0x1, 0x1011584, 0x10, 0x4, 0x11fa090, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af4e0, 0x10000007, 0, 0x1e, 0x1, 0x1011584, 0x10, 0x4, 0x11fa2f0, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af4d0, 0x10000007, 0, 0x1f, 0x1, 0x1011584, 0x10, 0x4, 0x11fa300, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af4bc, 0x10000004, 0, 0x1d, 0x1, 0x1011584, 0x4, 0x4, 0x11fd944, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af4b0, 0x10000004, 0, 0xe, 0x1, 0x1011584, 0x4, 0x4, 0x11fd8c4, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10a16ec, 0x10000004, 0, 0xf, 0x1, 0x1011584, 0x4, 0x4, 0x11fd8d4, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af4a4, 0x10000004, 0, 0x10, 0x1, 0x1011584, 0x4, 0x4, 0x11fd8e4, 0,
        ],
    ),
    MapStep::Entry(0x1043608, 0x11fec6c),
    MapStep::Entry(0x10af578, 0x11fec30),
    MapStep::Entry(0x10af568, 0x11fec34),
    MapStep::Entry(0x10af55c, 0x11fec38),
    MapStep::Entry(0x10af54c, 0x11fec48),
    MapStep::Entry(0x10af53c, 0x11fec4c),
    MapStep::Entry(0x10af52c, 0x11fec50),
    MapStep::Entry(0x10af51c, 0x11fec54),
    MapStep::Entry(0x10af50c, 0x11fec58),
    MapStep::Entry(0x10af4fc, 0x11fec3c),
    MapStep::Entry(0x10af5d8, 0x11fec44),
    MapStep::Entry(0x10af4f4, 0x11fec40),
    MapStep::Entry(0x10af4bc, 0x11fec70),
    MapStep::Entry(0x10af4b0, 0x11fec74),
    MapStep::Entry(0x10a16ec, 0x11fec78),
    MapStep::Entry(0x10af4a4, 0x11fec7c),
    MapStep::Entry(0x10af4d0, 0x11fec80),
    MapStep::Entry(0x10af4e0, 0x11fec84),
];

/// Section 3: the steps on the map at +0x80, in the exe order.
const MAP_STEPS_3: &[MapStep] = &[
    MapStep::Call(0x68, &[0x10a1e50, 0x20000009, 0, 0x1011584, 0, 0, 0, 0]),
    MapStep::Call(0x68, &[0x10a1d20, 0x20000009, 0x4, 0x1011584, 0, 0, 0, 0]),
    MapStep::Call(
        0x8,
        &[0x10a1cac, 0x20000009, 0x120000, 0xe, 0x36, 0, 0, 0, 0, 0],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af5d8, 0x10000007, 0, 0x8, 0x1, 0x1011584, 0x10, 0x4, 0x11fd894, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af498, 0x10000004, 0, 0x9, 0x1, 0x1011584, 0x4, 0x4, 0x11fd85c, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af48c, 0x10000004, 0, 0xa, 0x1, 0x1011584, 0x4, 0x4, 0x11fd860, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af480, 0x10000004, 0, 0xb, 0x1, 0x1011584, 0x4, 0x4, 0x11fd864, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af474, 0x10000007, 0, 0xc, 0x1, 0x1011584, 0x10, 0x4, 0x11fd934, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10a16ec, 0x10000007, 0, 0xd, 0x1, 0x1011584, 0x10, 0x4, 0x11fd924, 0,
        ],
    ),
];

/// Section 4: the steps on the map at +0x7c, in the exe order.
const MAP_STEPS_4: &[MapStep] = &[
    MapStep::Call(
        0x8,
        &[
            0x10af468, 0x10000007, 0, 0, 0x1, 0x1011584, 0x10, 0x4, 0x11fd8f4, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af45c, 0x10000007, 0, 0x1, 0x1, 0x1011584, 0x10, 0x4, 0x11fd904, 0,
        ],
    ),
    MapStep::Call(
        0x8,
        &[
            0x10af454, 0x10000007, 0, 0x2, 0x1, 0x1011584, 0x10, 0x4, 0x11fd914, 0,
        ],
    ),
];

// Translated from 00b7e430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowLightShader::CreateConstantMaps` (engine map), `thiscall`, no stack words. Each of the
/// four maps that is not there yet is made and filled: the vertex map (+0x34, program type 0;
/// 22 entries added, the 22 vertex entry pointers `011febd8` taken by name, then kept also at
/// +0x88), the pixel map (+0x30, type 2; 18 entries, the pixel entry pointers, kept also at
/// +0x84), and the two texture-effect maps (+0x80, type 0, nine entries; +0x7c, type 2, three
/// entries). The steps, their words and their order are the exe's (`MAP_STEPS_1` to `_4`).
// Translation incomplete: 00b7e430 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the four allocations) is not translated.
pub fn shadow_light_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x34) == 0 {
        new_constant_map(e, base, 0x34, 0);
        run_map_steps(e, base, 0x34, MAP_STEPS_1);
        let map = e.mem.u32(base + 0x34);
        ni_pointer_assign(e, base + 0x88, map);
    }
    if e.mem.u32(base + 0x30) == 0 {
        new_constant_map(e, base, 0x30, 2);
        run_map_steps(e, base, 0x30, MAP_STEPS_2);
        let map = e.mem.u32(base + 0x30);
        ni_pointer_assign(e, base + 0x84, map);
    }
    if e.mem.u32(base + 0x80) == 0 {
        new_constant_map(e, base, 0x80, 0);
        run_map_steps(e, base, 0x80, MAP_STEPS_3);
    }
    if e.mem.u32(base + 0x7c) == 0 {
        new_constant_map(e, base, 0x7c, 2);
        run_map_steps(e, base, 0x7c, MAP_STEPS_4);
    }
}

/// One loop of a shader loader over consecutive 0x4c-byte source blocks of its frame: the
/// first block's frame offset, the block count, the compiled-name format (formatted with the
/// index), the `NiPointer` array of the shaders, and whether a block without a file name clears
/// its shader (else it is skipped).
struct CompileLoop {
    first: u32,
    count: u32,
    format: u32,
    shaders: u32,
    clear_empty: bool,
}

/// The frame offsets of a loader's path and compiled-name buffers (0x104 bytes each), the
/// profile and whether it compiles pixel shaders.
struct LoaderFrame {
    path: u32,
    name: u32,
    profile: u32,
    pixel: bool,
}

/// One block of a loader loop: `MakePath` of its file, the name formatted with the index
/// (`00406d00(name, 0x104, format, index)`), the compile (`this`, path, the block's macros,
/// profile, name) and its result assigned to the shader's `NiPointer`.
fn compile_block(
    e: &mut Engine,
    base: u32,
    frame: u32,
    layout: &LoaderFrame,
    block: u32,
    format: u32,
    index: u32,
    shader: u32,
) {
    let file = e.mem.u32(block);
    let path = frame + layout.path;
    let name = frame + layout.name;
    e.call(MAKE_PATH, &args![file, path]);
    e.call(FORMAT_00406D00, &args![name, 0x104u32, format, index]);
    let compile = if layout.pixel {
        COMPILE_PIXEL_SHADER
    } else {
        COMPILE_VERTEX_SHADER
    };
    let compiled = e
        .call(compile, &args![base, path, block + 4, layout.profile, name])
        .u32();
    ni_pointer_assign(e, shader, compiled);
}

/// A loader loop: each block with a file name is compiled into its shader; a block without one
/// is skipped, or with `clear_empty` its shader is released and cleared.
fn run_compile_loop(
    e: &mut Engine,
    base: u32,
    frame: u32,
    layout: &LoaderFrame,
    step: &CompileLoop,
) {
    for index in 0..step.count {
        let block = frame + step.first + 0x4c * index;
        let shader = step.shaders + 4 * index;
        if e.mem.u32(block) == 0 {
            if step.clear_empty {
                ni_pointer_clear(e, shader);
            }
            continue;
        }
        compile_block(e, base, frame, layout, block, step.format, index, shader);
    }
}

/// The shader model setting (`011f91c0`).
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

/// The source blocks `LoadVertexShaders` writes into its frame, in the exe order (file name,
/// macro name/value pairs, the null name that ends them, the rest of the 0x4c bytes cleared).
const VERTEX_SOURCES: &[ShaderSource] = &[
    ShaderSource {
        offset: 0x2534,
        words: &[0x10aeda8, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2580,
        words: &[0, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x25cc,
        words: &[0, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2618,
        words: &[0x10aed88, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2664,
        words: &[0x10aed68, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x26b0,
        words: &[0x10aeda8, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x26fc,
        words: &[0x10aeda8, 0x10aed64, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2748,
        words: &[0x10aeda8, 0x10aed54, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2794,
        words: &[0x10aed34, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x27e0,
        words: &[0x10aed10, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x282c,
        words: &[0x10aece4, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2878,
        words: &[0x10aece4, 0x10aece0, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x28c4,
        words: &[0x10aece4, 0x10aed64, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2910,
        words: &[0x10aece4, 0x10aece0, 0x1011584, 0x10aed64, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x295c,
        words: &[0x10aece4, 0x10aed54, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x29a8,
        words: &[0x10aece4, 0x10aed54, 0x1011584, 0x10aece0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x29f4,
        words: &[0x10aecb8, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2a40,
        words: &[0x10aeda8, 0x10aece0, 0x1011584, 0x10aecb0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2a8c,
        words: &[0x10aeda8, 0x10aece0, 0x1011584, 0x10aecac, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2ad8,
        words: &[0x10aec90, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2b24,
        words: &[0x10aec70, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2b70,
        words: &[0x10aec70, 0x10aed64, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2bbc,
        words: &[0x10aec4c, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2c08,
        words: &[0x10aec4c, 0x10aed64, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2c54,
        words: &[0x10aeda8, 0x10aec48, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2ca0,
        words: &[0x10aec2c, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2cec,
        words: &[0x10aec2c, 0x10aec20, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2d38,
        words: &[0x10aec2c, 0x10aec20, 0x1011584, 0x10aec48, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2d84,
        words: &[0x10aec2c, 0x10aec18, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2dd0,
        words: &[0x10aec2c, 0x10aec20, 0x1011584, 0x10aec18, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2e1c,
        words: &[0, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x1eac,
        words: &[0x10aeda8, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1ef8,
        words: &[
            0x10aeda8, 0x10aec10, 0x1011584, 0x10aece0, 0x1011584, 0x10aecb0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1f44,
        words: &[
            0x10aeda8, 0x10aec10, 0x1011584, 0x10aece0, 0x1011584, 0x10aecac, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1f90,
        words: &[0x10aed88, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1fdc,
        words: &[0x10aed68, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2028,
        words: &[0x10aeda8, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2074,
        words: &[0x10aeda8, 0x10aec10, 0x1011584, 0x10aed64, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x20c0,
        words: &[0x10aeda8, 0x10aec10, 0x1011584, 0x10aed54, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x210c,
        words: &[0x10aed34, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2158,
        words: &[0x10aed10, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x21a4,
        words: &[0x10aece4, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x21f0,
        words: &[0x10aece4, 0x10aec10, 0x1011584, 0x10aed64, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x223c,
        words: &[0x10aece4, 0x10aec10, 0x1011584, 0x10aece0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2288,
        words: &[
            0x10aece4, 0x10aec10, 0x1011584, 0x10aece0, 0x1011584, 0x10aed64, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x22d4,
        words: &[0x10aece4, 0x10aec10, 0x1011584, 0x10aed54, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2320,
        words: &[
            0x10aece4, 0x10aec10, 0x1011584, 0x10aed54, 0x1011584, 0x10aece0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x236c,
        words: &[0x10aecb8, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x23b8,
        words: &[0x10aec90, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2404,
        words: &[0, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2450,
        words: &[0, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x249c,
        words: &[0x10aeda8, 0x10aec10, 0x1011584, 0x10aec48, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x24e8,
        words: &[0, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x18,
        words: &[0x10aebf4, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x64,
        words: &[0x10aebf4, 0x1039578, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xb0,
        words: &[
            0x10aebd4, 0x10aebc4, 0x1011584, 0x10aebb8, 0x1011584, 0x10aebac, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0xfc,
        words: &[0x10aebf4, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x148,
        words: &[0x10aebf4, 0x10aeba0, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x194,
        words: &[
            0x10aebd4, 0x10aebc4, 0x1011584, 0x10aeba0, 0x1011584, 0x10aebb8, 0x1011584, 0x10aebac,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aebf4, 0x10aec10, 0x1011584, 0x10aeba0,
            0x1011584, 0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x22c,
        words: &[0x10aebf4, 0x102bf34, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x278,
        words: &[0x10aebf4, 0x10aeb98, 0x1063f3c, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2c4,
        words: &[0x10aebf4, 0x10aeb98, 0x1063f3c, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x310,
        words: &[0x10aebf4, 0x10aeb98, 0x1063f3c, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x35c,
        words: &[
            0x10aebf4, 0x10aeb98, 0x1063f3c, 0x10aec10, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x3a8,
        words: &[0x10aebf4, 0x10aeb8c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3f4,
        words: &[0x10aebf4, 0x10aeb8c, 0x1011584, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x440,
        words: &[0x10aebf4, 0x10aeb8c, 0x1011584, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x48c,
        words: &[
            0x10aebf4, 0x10aeb8c, 0x1011584, 0x10aec10, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x4d8,
        words: &[0x10aebf4, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f3c, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x524,
        words: &[
            0x10aebf4, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f3c, 0x10aec10, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x570,
        words: &[
            0x10aebf4, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f3c, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x5bc,
        words: &[
            0x10aebf4, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f3c, 0x10aec10, 0x1011584, 0x10aeba0,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeb70, 0x10aeb98, 0x1063ee8, 0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x654,
        words: &[0x10aeb70, 0x10aeb98, 0x1063ee8, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x6a0,
        words: &[0x10aeb70, 0x10aeb98, 0x1063f24, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x6ec,
        words: &[0x10aeb70, 0x10aeb6c, 0x1011584, 0x10aeb98, 0x1063f24, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x738,
        words: &[0x10aeb70, 0x10aeb98, 0x1063f24, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x784,
        words: &[0x10aeb70, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f24, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x7d0,
        words: &[
            0x10aeb70, 0x10aeb6c, 0x1011584, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f24, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x81c,
        words: &[
            0x10aeb70, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f24, 0x10aec10, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x868,
        words: &[0x10aeb54, 0x10aeb98, 0x1063f3c, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x8b4,
        words: &[0x10aeb54, 0x10aeb98, 0x1063f3c, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x900,
        words: &[0x10aeb54, 0x10aeb98, 0x1063f3c, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x94c,
        words: &[
            0x10aeb54, 0x10aeb98, 0x1063f3c, 0x10aec10, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x998,
        words: &[0x10aeb54, 0x10aeb98, 0x1063f30, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x9e4,
        words: &[0x10aeb54, 0x10aeb98, 0x1063f30, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xa30,
        words: &[0x10aeb54, 0x10aeb98, 0x1063f30, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xa7c,
        words: &[
            0x10aeb54, 0x10aeb98, 0x1063f30, 0x10aec10, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0xac8,
        words: &[0x10aeb34, 0x10aeb98, 0x1063f3c, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xb14,
        words: &[0x10aeb34, 0x10aeb98, 0x1063f3c, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xb60,
        words: &[0x10aeb34, 0x10aeb98, 0x1063f30, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xbac,
        words: &[0x10aeb34, 0x10aeb98, 0x1063f30, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xbf8,
        words: &[0x10aeb14, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0xc44,
        words: &[0x10aeb14, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xc90,
        words: &[0x10aeb14, 0x10aeba0, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xcdc,
        words: &[0x10aeb14, 0x10aec10, 0x1011584, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xd28,
        words: &[0x10aeb14, 0x10aeb0c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xd74,
        words: &[0x10aeb14, 0x10aec10, 0x1011584, 0x10aeb0c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xdc0,
        words: &[0x10aeb14, 0x10aeafc, 0x1063f3c, 0x10aeb0c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xe0c,
        words: &[
            0x10aeb14, 0x10aeafc, 0x1063f3c, 0x10aec10, 0x1011584, 0x10aeb0c, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0xe58,
        words: &[0x10aeb14, 0x10aeafc, 0x1063f30, 0x10aeb0c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xea4,
        words: &[
            0x10aeb14, 0x10aeafc, 0x1063f30, 0x10aec10, 0x1011584, 0x10aeb0c, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0xef0,
        words: &[0x10aeae0, 0x10aead8, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xf3c,
        words: &[0x10aeae0, 0x10aec10, 0x1011584, 0x10aead8, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xf88,
        words: &[0x10aeae0, 0x10aead8, 0x1011584, 0x10aead4, 0, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xfd4,
        words: &[0x10aeab8, 0x10aeba0, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1020,
        words: &[0x10aeab8, 0x10aec20, 0x1011584, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x106c,
        words: &[0x10aea94, 0x1098a04, 0x1011584, 0x10aec18, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x10b8,
        words: &[0x10aea94, 0x10aeb8c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1104,
        words: &[0],
        cleared: 0x48,
    },
    ShaderSource {
        offset: 0x1150,
        words: &[
            0x10aea94, 0x10aec20, 0x1011584, 0x1098a04, 0x1011584, 0x10aec18, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x119c,
        words: &[0x10aea94, 0x10aec20, 0x1011584, 0x10aeb8c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x11e8,
        words: &[0],
        cleared: 0x48,
    },
    ShaderSource {
        offset: 0x1234,
        words: &[0x10aea94, 0x10aeb8c, 0x1011584, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1280,
        words: &[
            0x10aea94, 0x10aec20, 0x1011584, 0x10aeb8c, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x12cc,
        words: &[0x10aea74, 0x10aea68, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1318,
        words: &[0x10aea74, 0x10aea68, 0x1011584, 0x10aea5c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1364,
        words: &[0x10aea74, 0x10aea68, 0x1011584, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x13b0,
        words: &[
            0x10aea74, 0x10aea68, 0x1011584, 0x10aec10, 0x1011584, 0x10aea5c, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x13fc,
        words: &[0x10aea74, 0x10aea68, 0x1011584, 0x10aea50, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1448,
        words: &[0x10aea2c, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x1494,
        words: &[0x10aea2c, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x14e0,
        words: &[0x10aea2c, 0x10aea24, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x152c,
        words: &[0x10aea2c, 0x10aea1c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1578,
        words: &[0x10aea2c, 0x10aea1c, 0x1011584, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x15c4,
        words: &[0x10ae9f8, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x1610,
        words: &[0x10ae9f8, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x165c,
        words: &[0x10ae9f8, 0x10aebc4, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x16a8,
        words: &[0x10ae9d4, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x16f4,
        words: &[0x10ae9d4, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1740,
        words: &[0x10aeab8, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x178c,
        words: &[0x10aeab8, 0x10aec20, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x17d8,
        words: &[0x10aeab8, 0x10ae9cc, 0x1011584, 0x10aece0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1824,
        words: &[0x10aeab8, 0x10aec20, 0x1011584, 0x10aec48, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1870,
        words: &[0x10aeab8, 0x10aec18, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x18bc,
        words: &[0x10aeab8, 0x10aec20, 0x1011584, 0x10aec18, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1908,
        words: &[0x10ae9ac, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x1954,
        words: &[0x10ae9ac, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x19a0,
        words: &[0x10ae9ac, 0x10aea1c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x19ec,
        words: &[0x10ae98c, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x1a38,
        words: &[0x10ae98c, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1a84,
        words: &[0x10ae968, 0x10ae95c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1ad0,
        words: &[0x10ae968, 0x10ae95c, 0x1011584, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1b1c,
        words: &[
            0x10ae968, 0x10ae95c, 0x1011584, 0x101c45c, 0x1011584, 0x10aed64, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1b68,
        words: &[0x10ae93c, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x1bb4,
        words: &[0x10ae93c, 0x10aec10, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1c00,
        words: &[0x10ae93c, 0x10ae930, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1c4c,
        words: &[0x10ae93c, 0x10aec10, 0x1011584, 0x10ae930, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1c98,
        words: &[0x10ae93c, 0x10aed64, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1ce4,
        words: &[0x10ae93c, 0x10aed64, 0x1011584, 0x10aec10, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1d30,
        words: &[0x10ae93c, 0x10aed64, 0x1011584, 0x10ae930, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1d7c,
        words: &[
            0x10ae93c, 0x10aed64, 0x1011584, 0x10aec10, 0x1011584, 0x10ae930, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1dc8,
        words: &[0x10ae914, 0x10aece0, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1e14,
        words: &[0x10ae914, 0x10aece0, 0x1011584, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1e60,
        words: &[0x10ae8f8, 0x10aeb98, 0x1020760, 0],
        cleared: 0x3c,
    },
];

/// The three loops of `LoadVertexShaders`: 31 shaders into `011fdd88`, 22 into `011fde04` and,
/// from shader model 2, 103 into `011fde5c` (where an empty block clears its shader); the vertex
/// profile `010a93e4`.
const VERTEX_LOOPS: [CompileLoop; 3] = [
    CompileLoop {
        first: 0x2534,
        count: 0x1f,
        format: 0x010a_e8e8,
        shaders: 0x011f_dd88,
        clear_empty: false,
    },
    CompileLoop {
        first: 0x1eac,
        count: 0x16,
        format: 0x010a_e8d8,
        shaders: 0x011f_de04,
        clear_empty: false,
    },
    CompileLoop {
        first: 0x18,
        count: 0x67,
        format: 0x010a_e8c8,
        shaders: 0x011f_de5c,
        clear_empty: true,
    },
];
const VERTEX_LAYOUT: LoaderFrame = LoaderFrame {
    path: 0x2e68,
    name: 0x2f6c,
    profile: 0x010a_93e4,
    pixel: false,
};

// Translated from 00b71bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowLightShader::LoadVertexShaders` (engine map), `thiscall`, no stack words. On a
/// 0x3074-byte frame (the shader kept at +0x14) it writes 154 source blocks of 0x4c bytes
/// (`VERTEX_SOURCES`: file name, macros, the rest cleared through `memset`, in the exe order; a
/// run of blocks may be written whole before the one whose rest is cleared),
/// then compiles them in three loops (`VERTEX_LOOPS`: `MakePath`, the name formatted with the
/// index, `CompileVertexShaderHLSL`, the result kept in the shader array), the third only from
/// shader model 2 (`011f91c0`). The stack probe `__alloca_probe` (`00ec8660`) of the frame is
/// left out (the frame is a stack block of the engine).
// Translation incomplete: 00b71bf0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn shadow_light_shader_load_vertex_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x3074, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x14, base);
        set_up_sources(e, frame, VERTEX_SOURCES);
        let base = e.mem.u32(frame + 0x14);
        run_compile_loop(e, base, frame, &VERTEX_LAYOUT, &VERTEX_LOOPS[0]);
        run_compile_loop(e, base, frame, &VERTEX_LAYOUT, &VERTEX_LOOPS[1]);
        if e.global::<i32>(SHADER_MODEL_011F91C0) >= 2 {
            run_compile_loop(e, base, frame, &VERTEX_LAYOUT, &VERTEX_LOOPS[2]);
        }
    });
}

/// The source blocks `LoadPixelShaders` writes into its frame, in the exe order (a run of
/// blocks may be written whole before the one whose rest is cleared). The four words that hold
/// frame addresses (the two `__gcvt` strings) are 0 here and set by `PIXEL_FRAME_WORDS`.
const PIXEL_SOURCES: &[ShaderSource] = &[
    ShaderSource {
        offset: 0x2fa0,
        words: &[0x10af2d4, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x2fec,
        words: &[0x10af2b4, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x3038,
        words: &[0x10af294, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x3084,
        words: &[0x10af2d4, 0x10aecac, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x30d0,
        words: &[0x10af2d4, 0x10aecac, 0x1011584, 0x10aed64, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x311c,
        words: &[0x10af2d4, 0x10aecac, 0x1011584, 0x10aed54, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3168,
        words: &[0x10af2d4, 0x10aecac, 0x1011584, 0x101c61c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x31b4,
        words: &[0x10af268, 0x10af288, 0, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3200,
        words: &[0x10af244, 0x10af288, 0, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x324c,
        words: &[0x10af21c, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x3298,
        words: &[0x10af21c, 0x10aece0, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x32e4,
        words: &[0x10af21c, 0x10af214, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3330,
        words: &[0x10af21c, 0x10af214, 0x1011584, 0x10aece0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x337c,
        words: &[0x10af21c, 0x10aed64, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x33c8,
        words: &[0x10af21c, 0x10aed64, 0x1011584, 0x10aece0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3414,
        words: &[0x10af21c, 0x10aed64, 0x1011584, 0x10af214, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3460,
        words: &[
            0x10af21c, 0x10aed64, 0x1011584, 0x10af214, 0x1011584, 0x10aece0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x34ac,
        words: &[0x10af21c, 0x10aed54, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x34f8,
        words: &[0x10af21c, 0x10aed54, 0x1011584, 0x10aece0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3544,
        words: &[0x10af21c, 0x10aed54, 0x1011584, 0x10af214, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3590,
        words: &[
            0x10af21c, 0x10aed54, 0x1011584, 0x10aece0, 0x1011584, 0x10af214, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x35dc,
        words: &[0x10af21c, 0x10aec48, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3628,
        words: &[0x10af21c, 0x10aec48, 0x1011584, 0x10aece0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3674,
        words: &[0x10af21c, 0x10aec48, 0x1011584, 0x10af214, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x36c0,
        words: &[
            0x10af21c, 0x10aec48, 0x1011584, 0x10af214, 0x1011584, 0x10aece0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x370c,
        words: &[0x10af21c, 0x10aec48, 0x1011584, 0x10aed64, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3758,
        words: &[
            0x10af21c, 0x10aec48, 0x1011584, 0x10aed64, 0x1011584, 0x10aece0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x37a4,
        words: &[
            0x10af21c, 0x10aec48, 0x1011584, 0x10aed64, 0x1011584, 0x10af214, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x37f0,
        words: &[
            0x10af21c, 0x10aec48, 0x1011584, 0x10aed64, 0x1011584, 0x10af214, 0x1011584, 0x10aece0,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10af1ec, 0,
        ],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x3888,
        words: &[0x10af2d4, 0x10aece0, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x38d4,
        words: &[0x10af2d4, 0x10aece0, 0x1011584, 0x10aecac, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3920,
        words: &[0x10af1d0, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x396c,
        words: &[0x10af1d0, 0x10aed64, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x39b8,
        words: &[0x10af1b0, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x3a04,
        words: &[0x10af18c, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x3a50,
        words: &[0x10af174, 0x10af16c, 0x1011584, 0x10af160, 0, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3a9c,
        words: &[0x10af174, 0x10af160, 0, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3ae8,
        words: &[0x10af174, 0x10af158, 0, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3b34,
        words: &[0x10af2d4, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x3b80,
        words: &[0x10af13c, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x3bcc,
        words: &[0x10af13c, 0x10aec20, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3c18,
        words: &[0x10af13c, 0x10aec48, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3c64,
        words: &[0x10af13c, 0x10aec20, 0x1011584, 0x10aec48, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3cb0,
        words: &[0x10af13c, 0x10aec18, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3cfc,
        words: &[0x10af13c, 0x10aec20, 0x1011584, 0x10aec18, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3d48,
        words: &[0],
        cleared: 0x48,
    },
    ShaderSource {
        offset: 0x3d94,
        words: &[0x10af2d4, 0x10af130, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x20,
        words: &[0x10af114, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x6c,
        words: &[0x10af114, 0x10aeb6c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xb8,
        words: &[0x10af114, 0x10aeb6c, 0x1011584, 0x1039578, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x104,
        words: &[
            0x10af0f4, 0x10aebc4, 0x1011584, 0x10aebb8, 0x1011584, 0x10af0e4, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x150,
        words: &[0x10af114, 0x10aec48, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x19c,
        words: &[0x10af114, 0x10aeba0, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1e8,
        words: &[
            0x10af0f4, 0x10aebc4, 0x1011584, 0x10aeba0, 0x1011584, 0x10aebb8, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x234,
        words: &[0x10af114, 0x10aec48, 0x1011584, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x280,
        words: &[0x10af114, 0x102bf34, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2cc,
        words: &[0x10af114, 0x101c61c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x318,
        words: &[0x10af114, 0x101c61c, 0x1011584, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x364,
        words: &[0x10af114, 0x10aeb98, 0x1063f3c, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x3b0,
        words: &[0x10af114, 0x10aeb98, 0x1063f3c, 0x10aec48, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x3fc,
        words: &[0x10af114, 0x10aeb98, 0x1063f3c, 0x101c61c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x448,
        words: &[0x10af114, 0x10aeb98, 0x1063f3c, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x494,
        words: &[
            0x10af114, 0x10aeb98, 0x1063f3c, 0x10aec48, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x4e0,
        words: &[
            0x10af114, 0x10aeb98, 0x1063f3c, 0x101c61c, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x52c,
        words: &[0x10af114, 0x10aeb8c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x578,
        words: &[0x10af114, 0x10aeb8c, 0x1011584, 0x10aec48, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x5c4,
        words: &[0x10af114, 0x10aeb8c, 0x1011584, 0x101c61c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x610,
        words: &[0x10af114, 0x10aeb8c, 0x1011584, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x65c,
        words: &[
            0x10af114, 0x10aeb8c, 0x1011584, 0x10aec48, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x6a8,
        words: &[
            0x10af114, 0x10aeb8c, 0x1011584, 0x101c61c, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x6f4,
        words: &[0x10af114, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f3c, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x740,
        words: &[
            0x10af114, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f3c, 0x10aec48, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x78c,
        words: &[0],
        cleared: 0x48,
    },
    ShaderSource {
        offset: 0x7d8,
        words: &[
            0x10af114, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f3c, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x824,
        words: &[
            0x10af114, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f3c, 0x10aec48, 0x1011584, 0x10aeba0,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        cleared: 0x48,
    },
    ShaderSource {
        offset: 0x8bc,
        words: &[0x10af0c8, 0x10aeb98, 0x1063ee8, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x908,
        words: &[0x10af0c8, 0x10aeb98, 0x1063ee8, 0x10aec48, 0, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x954,
        words: &[0x10af0c8, 0x10aeb98, 0x1063f24, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x9a0,
        words: &[0x10af0c8, 0x10aeb6c, 0x1011584, 0x10aeb98, 0x1063f24, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x9ec,
        words: &[0x10af0c8, 0x10aeb98, 0x1063f24, 0x10aec48, 0, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xa38,
        words: &[0x10af0c8, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f24, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xa84,
        words: &[
            0x10af0c8, 0x10aeb6c, 0x1011584, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f24, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0xad0,
        words: &[
            0x10af0c8, 0x10aeb8c, 0x1011584, 0x10aeb98, 0x1063f24, 0x10aec48, 0, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0xb1c,
        words: &[0x10af0b0, 0x10aeb98, 0x1063f3c, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xb68,
        words: &[0x10af0b0, 0x10aeb98, 0x1063f3c, 0x10aec48, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xbb4,
        words: &[0x10af0b0, 0x10aeb98, 0x1063f3c, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xc00,
        words: &[
            0x10af0b0, 0x10aeb98, 0x1063f3c, 0x10aec48, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0xc4c,
        words: &[0x10af0b0, 0x10aeb98, 0x1063f30, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xc98,
        words: &[0x10af0b0, 0x10aeb98, 0x1063f30, 0x10aec48, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xce4,
        words: &[0x10af0b0, 0x10aeb98, 0x1063f30, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xd30,
        words: &[
            0x10af0b0, 0x10aeb98, 0x1063f30, 0x10aec48, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0xd7c,
        words: &[0x10af090, 0x10aeb98, 0x1063f3c, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xdc8,
        words: &[0x10af090, 0x10aeb98, 0x1063f30, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xe14,
        words: &[0x10af070, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0xe60,
        words: &[0x10af070, 0x101c61c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xeac,
        words: &[0x10af070, 0x10aeba0, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xef8,
        words: &[0x10af070, 0x10aeba0, 0x1011584, 0x101c61c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xf44,
        words: &[0x10af070, 0x10aeb0c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0xf90,
        words: &[0x10af070, 0x10aeb0c, 0x1011584, 0x101c61c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0xfdc,
        words: &[0x10af070, 0x10aeafc, 0x1063f3c, 0x10aeb0c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1028,
        words: &[
            0x10af070, 0x10aeafc, 0x1063f3c, 0x10aeb0c, 0x1011584, 0x101c61c, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1074,
        words: &[0x10af070, 0x10aeafc, 0x1063f30, 0x10aeb0c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x10c0,
        words: &[
            0x10af070, 0x10aeafc, 0x1063f30, 0x10aeb0c, 0x1011584, 0x101c61c, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x110c,
        words: &[0x10af054, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x1158,
        words: &[0x10af054, 0x10af04c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x11a4,
        words: &[0x10af054, 0x10aead4, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x11f0,
        words: &[0x10af030, 0x10aeba0, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x123c,
        words: &[0x10af030, 0x10aeba0, 0x1011584, 0x10aec20, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1288,
        words: &[0x10af00c, 0x1098a04, 0x1011584, 0x10aec18, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x12d4,
        words: &[0x10af00c, 0x10aeb8c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1320,
        words: &[0],
        cleared: 0x48,
    },
    ShaderSource {
        offset: 0x136c,
        words: &[
            0x10af00c, 0x10aec20, 0x1011584, 0x1098a04, 0x1011584, 0x10aec18, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x13b8,
        words: &[0x10af00c, 0x10aec20, 0x1011584, 0x10aeb8c, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1404,
        words: &[0],
        cleared: 0x48,
    },
    ShaderSource {
        offset: 0x1450,
        words: &[0x10af00c, 0x10aeb8c, 0x1011584, 0x10aeba0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x149c,
        words: &[
            0x10af00c, 0x10aec20, 0x1011584, 0x10aeb8c, 0x1011584, 0x10aeba0, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x14e8,
        words: &[0x10aefec, 0x10aea68, 0x1011584, 0x10aefe0, 0x1063f30, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1534,
        words: &[
            0x10aefec, 0x10aea68, 0x1011584, 0x10aefe0, 0x1063f30, 0x10aea5c, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1580,
        words: &[
            0x10aefec, 0x10aea68, 0x1011584, 0x10aefe0, 0x1063f30, 0x10aec10, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x15cc,
        words: &[
            0x10aefec, 0x10aea68, 0x1011584, 0x10aefe0, 0x1063f30, 0x10aec10, 0x1011584, 0x10aea5c,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aefec, 0x10aea68, 0x1011584, 0x10aea50,
            0x1011584, 0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x1664,
        words: &[0x10aefbc, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x16b0,
        words: &[0x10aefbc, 0x10aea24, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x16fc,
        words: &[0x10aefbc, 0x10aea1c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1748,
        words: &[0x10aef98, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x1794,
        words: &[0x10aef74, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x17e0,
        words: &[0x10af030, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x182c,
        words: &[0x10af030, 0x10aec20, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1878,
        words: &[0x10af030, 0x10ae9cc, 0x1011584, 0x10aece0, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x18c4,
        words: &[0x10af030, 0x10aec48, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1910,
        words: &[0x10af030, 0x10aec20, 0x1011584, 0x10aec48, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x195c,
        words: &[0x10af030, 0x10aec18, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x19a8,
        words: &[0x10af030, 0x10aec20, 0x1011584, 0x10aec18, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x19f4,
        words: &[0x10aef54, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x1a40,
        words: &[0x10aef54, 0x10aea1c, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1a8c,
        words: &[0x10aef34, 0x10aef30, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1ad8,
        words: &[0x10aef10, 0],
        cleared: 0x44,
    },
    ShaderSource {
        offset: 0x1b24,
        words: &[0x10aef10, 0x10aef00, 0x1011584, 0],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x1b70,
        words: &[
            0x10aeed8, 0x10aeecc, 0x1063f48, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1bbc,
        words: &[
            0x10aeed8, 0x10aeecc, 0x1063f48, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeba0,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f48, 0x10aece0,
            0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f48, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f48, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f48, 0x10aece0, 0x1011584, 0x10aeef4,
            0x1011584, 0x10aeeb0, 0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f48, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0,
            0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f48, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f3c,
            0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x1e1c,
        words: &[
            0x10aeed8, 0x10aeecc, 0x1063f3c, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeba0,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f3c, 0x10aece0,
            0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f3c, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f3c, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f3c, 0x10aece0, 0x1011584, 0x10aeef4,
            0x1011584, 0x10aeeb0, 0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f3c, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0,
            0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f3c, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f30,
            0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x207c,
        words: &[
            0x10aeed8, 0x10aeecc, 0x1063f30, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeba0,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f30, 0x10aece0,
            0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f30, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f30, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f30, 0x10aece0, 0x1011584, 0x10aeef4,
            0x1011584, 0x10aeeb0, 0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f30, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0,
            0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f30, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f24,
            0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x22dc,
        words: &[
            0x10aeed8, 0x10aeecc, 0x1063f24, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeba0,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f24, 0x10aece0,
            0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f24, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f24, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f24, 0x10aece0, 0x1011584, 0x10aeef4,
            0x1011584, 0x10aeeb0, 0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f24, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0,
            0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f24, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f18,
            0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x253c,
        words: &[
            0x10aeed8, 0x10aeecc, 0x1063f18, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeba0,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f18, 0x10aece0,
            0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f18, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f18, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f18, 0x10aece0, 0x1011584, 0x10aeef4,
            0x1011584, 0x10aeeb0, 0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f18, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0,
            0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f18, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f0c,
            0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x279c,
        words: &[
            0x10aeed8, 0x10aeecc, 0x1063f0c, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeba0,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f0c, 0x10aece0,
            0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f0c, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f0c, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f0c, 0x10aece0, 0x1011584, 0x10aeef4,
            0x1011584, 0x10aeeb0, 0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f0c, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0,
            0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f0c, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f00,
            0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0,
        ],
        cleared: 0x2c,
    },
    ShaderSource {
        offset: 0x29fc,
        words: &[
            0x10aeed8, 0x10aeecc, 0x1063f00, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeba0,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f00, 0x10aece0,
            0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f00, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f00, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc, 0x1063f00, 0x10aece0, 0x1011584, 0x10aeef4,
            0x1011584, 0x10aeeb0, 0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aeed8, 0x10aeecc, 0x1063f00, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0,
            0x1011584, 0x10aeebc, 0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aeed8, 0x10aeecc,
            0x1063f00, 0x10aece0, 0x1011584, 0x10aeef4, 0x1011584, 0x10aeeb0, 0x1011584, 0x10aeebc,
            0x1011584, 0x10aeba0, 0x1011584, 0, 0, 0, 0, 0, 0, 0x10aee88, 0x10aece0, 0x1011584,
            0x10aeea4, 0x1011584,
        ],
        cleared: 0x38,
    },
    ShaderSource {
        offset: 0x2c5c,
        words: &[
            0x10af0f4, 0x10aebc4, 0x1011584, 0x10aebb8, 0x1011584, 0x10af0e4, 0x1011584, 0x10aeea4,
            0x1011584, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10af2d4, 0x10aece0, 0x1011584, 0x10aee78,
            0x1011584, 0,
        ],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2cf4,
        words: &[0x10aee5c, 0x10aeb98, 0x1020760, 0x10aeea4, 0x1011584, 0],
        cleared: 0x34,
    },
    ShaderSource {
        offset: 0x2d40,
        words: &[
            0x10aee30, 0x10ae95c, 0x1011584, 0x10aee24, 0x10aee1c, 0x10aee54, 0x1063f48, 0x10aee14,
            0x1063f48, 0x10aefe0, 0x1063f30, 0, 0, 0, 0, 0, 0, 0, 0, 0x10aee30, 0x10ae95c,
            0x1011584, 0x10aee24, 0x10aee1c, 0x10aee54, 0x1063f48, 0x10aee14, 0x1063f48, 0x10aea5c,
            0x1011584, 0x10aefe0, 0x1063f30, 0, 0, 0, 0, 0, 0, 0x10aee30, 0x10ae95c, 0x1011584,
            0x10aee24, 0x10aee1c, 0x10aee54, 0x1063f24, 0x10aee14, 0x1063f48, 0x10aefe0, 0x1063f30,
            0, 0, 0, 0, 0, 0, 0, 0, 0x10aee30, 0x10ae95c, 0x1011584, 0x10aee24, 0x10aee1c,
            0x10aee54, 0x1063f24, 0x10aee14, 0x1063f48, 0x10aea5c, 0x1011584, 0x10aefe0, 0x1063f30,
            0, 0, 0, 0, 0, 0, 0x10aee30, 0x10ae95c, 0x1011584, 0x10aee24, 0x10aee1c, 0x10aee54,
            0x1063f24, 0x10aee14, 0x1063f3c, 0x10aefe0, 0x1063f30, 0, 0, 0, 0, 0, 0, 0, 0,
            0x10aee30, 0x10ae95c, 0x1011584, 0x10aee24, 0x10aee1c, 0x10aee54, 0x1063f24, 0x10aee14,
            0x1063f3c, 0x10aea5c, 0x1011584, 0x10aefe0, 0x1063f30, 0, 0, 0, 0, 0, 0, 0x10aef10,
            0x10aee0c, 0x1011584, 0,
        ],
        cleared: 0x3c,
    },
    ShaderSource {
        offset: 0x2f54,
        words: &[0x10aef10, 0x10aee00, 0x1011584, 0],
        cleared: 0x3c,
    },
];

/// The words of `PIXEL_SOURCES` that hold frame addresses: (word offset, frame offset).
const PIXEL_FRAME_WORDS: [(u32, u32); 4] = [
    (0x31bc, 0x4150),
    (0x3208, 0x4150),
    (0x3a60, 0x3ff0),
    (0x3aa4, 0x3ff0),
];

/// CRT `__gcvt` (`00ecdd16`, `cdecl`: a `double`, the digit count, the buffer) and `sprintf`
/// (`00ec623a`, `cdecl`), the two floats `LoadPixelShaders` writes as macro values (`011adbcc`,
/// `011adbd0`), the setting it checks (`011f91bc`) and the number it formats (`011f91b0`, as
/// `"%i"` at `01020774`), the five-byte string it copies into the frame (`010aede8`), the
/// pixel profile (`010a93cc`), the two name formats of the second loop (`010aedd8`, and
/// `010aedc4` for the shaders 0x46 to 0x49 and 0x98), and the two shader arrays (`011fda48`,
/// 48 shaders; `011fdb08`, 160).
const GCVT: u32 = 0x00ec_dd16;
const SPRINTF: u32 = 0x00ec_623a;
const PIXEL_VALUE_011ADBCC: u32 = 0x011a_dbcc;
const PIXEL_VALUE_011ADBD0: u32 = 0x011a_dbd0;
const SETTING_011F91BC: u32 = 0x011f_91bc;
const NUMBER_011F91B0: u32 = 0x011f_91b0;
const NUMBER_FORMAT_01020774: u32 = 0x0102_0774;
const STRING_010AEDE8: u32 = 0x010a_ede8;
const PIXEL_FORMAT_010AEDD8: u32 = 0x010a_edd8;
const PIXEL_FORMAT_010AEDC4: u32 = 0x010a_edc4;
const PIXEL_SHADERS_011FDB08: u32 = 0x011f_db08;

const PIXEL_FIRST_LOOP: CompileLoop = CompileLoop {
    first: 0x2fa0,
    count: 0x30,
    format: 0x010a_edf0,
    shaders: 0x011f_da48,
    clear_empty: false,
};
const PIXEL_LAYOUT: LoaderFrame = LoaderFrame {
    path: 0x3eec,
    name: 0x3de8,
    profile: 0x010a_93cc,
    pixel: true,
};

// Translated from 00b74210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowLightShader::LoadPixelShaders` (the shader's vtable +0x14c, the slot the Xbox PDB
/// names so; Ghidra made no function there), `thiscall`, no stack words. On a 0x42b4-byte
/// frame (the shader kept at +0x14): the floats `011adbcc` and `011adbd0` written as strings
/// (`__gcvt`, 12 digits, at +0x4150 and +0x3ff0); 149 source runs (`PIXEL_SOURCES`, then the
/// four words that point at those strings); the first 48 shaders compiled into `011fda48`. From
/// shader model 2 (`011f91c0`, kept at +0x1c): the string `010aede8` copied to +0x3de0 and, for
/// setting `011f91bc` 4 or 6, six macro values pointed at it; `011f91b0` formatted at +0x10
/// (`"%i"`) and six macro values pointed at that; then 160 shaders into `011fdb08`: a block
/// without a file clears its shader, shaders past 0x99 need shader model 5, and the shaders
/// 0x46 to 0x49 and 0x98 get their name formatted again with the index and `011f91b0`. The stack
/// probe of the frame is left out (the frame is a stack block of the engine).
// Translation incomplete: 00b74210 - the /GS stack cookie check (00ec408c) is not translated.
pub fn shadow_light_shader_load_pixel_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x42b4, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x14, base);
        let first = x87_load32(e.global::<u32>(PIXEL_VALUE_011ADBCC));
        e.call(GCVT, &args![first, 0xcu32, frame + 0x4150]);
        let second = x87_load32(e.global::<u32>(PIXEL_VALUE_011ADBD0));
        e.call(GCVT, &args![second, 0xcu32, frame + 0x3ff0]);
        set_up_sources(e, frame, PIXEL_SOURCES);
        for (word, target) in PIXEL_FRAME_WORDS {
            e.mem.set_u32(frame + word, frame + target);
        }
        let base = e.mem.u32(frame + 0x14);
        run_compile_loop(e, base, frame, &PIXEL_LAYOUT, &PIXEL_FIRST_LOOP);
        let model = e.global::<i32>(SHADER_MODEL_011F91C0);
        e.mem.set_u32(frame + 0x1c, model as u32);
        if model < 2 {
            return;
        }
        let setting = e.global::<u32>(SETTING_011F91BC);
        let text = e.global::<u32>(STRING_010AEDE8);
        let last = e.global::<u8>(STRING_010AEDE8 + 4);
        e.mem.set_u32(frame + 0x3de0, text);
        e.mem.set_u8(frame + 0x3de4, last);
        if setting == 6 || setting == 4 {
            for offset in [0x2d50u32, 0x2d9c, 0x2de8, 0x2e34, 0x2e80, 0x2ecc] {
                e.mem.set_u32(frame + offset, frame + 0x3de0);
            }
        }
        let number = e.global::<u32>(NUMBER_011F91B0);
        e.call(
            SPRINTF,
            &args![frame + 0x10, NUMBER_FORMAT_01020774, number],
        );
        for offset in [0x2d68u32, 0x2dbc, 0x14f8, 0x1544, 0x1590, 0x15dc] {
            e.mem.set_u32(frame + offset, frame + 0x10);
        }
        e.mem.set_u32(frame + 0x18, frame + 0x20);
        for index in 0..0xa0u32 {
            let block = frame + 0x20 + 0x4c * index;
            let shader = PIXEL_SHADERS_011FDB08 + 4 * index;
            let file = e.mem.u32(block);
            if file == 0 {
                ni_pointer_clear(e, shader);
                continue;
            }
            let model = e.mem.u32(frame + 0x1c) as i32;
            if index > 0x99 && model < 5 {
                continue;
            }
            let path = frame + PIXEL_LAYOUT.path;
            let name = frame + PIXEL_LAYOUT.name;
            e.call(MAKE_PATH, &args![file, path]);
            e.call(
                FORMAT_00406D00,
                &args![name, 0x104u32, PIXEL_FORMAT_010AEDD8, index],
            );
            if index == 0x98 || (0x46..=0x49).contains(&index) {
                let number = e.global::<u32>(NUMBER_011F91B0);
                e.call(
                    FORMAT_00406D00,
                    &args![name, 0x104u32, PIXEL_FORMAT_010AEDC4, index, number],
                );
            }
            let base = e.mem.u32(frame + 0x14);
            let compiled = e
                .call(
                    COMPILE_PIXEL_SHADER,
                    &args![base, path, block + 4, PIXEL_LAYOUT.profile, name],
                )
                .u32();
            ni_pointer_assign(e, shader, compiled);
        }
    });
}

// Translated from 00b71b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with two stack words (its
/// caller `00b7d200` sets `ECX`, the body never reads it): a texture stage and a shader
/// property. With both non-null, the stage gets the property's mode (vtable +0xe0) through
/// `00e7de90`.
pub fn fn_00b71b10(e: &mut Engine, _this: Ptr, stage: u32, property: u32) {
    if stage != 0 && property != 0 {
        let mode = e.vcall(property, 0xe0, &args![]).u32();
        e.call(STAGE_MODE_00E7DE90, &args![stage, mode]);
    }
}

/// The shader-model global (`011f91c0`) is `SHADER_MODEL_011F91C0`. The pass creators
/// `00b78980` runs, in its order (each `thiscall` on the shader, no stack words): the ten it
/// always runs, the thirteen it runs from shader model 2 on (the last a tail jump), and the
/// one it runs below (`00bea040`, also a tail jump). They take no words (`ECX` is set, unread).
const PASS_CREATORS_ALWAYS: [u32; 10] = [
    0x00be_b070,
    0x00be_22b0,
    0x00be_39e0,
    0x00be_7420,
    0x00be_78e0,
    0x00be_7f40,
    0x00be_8960,
    0x00be_8fb0,
    0x00be_9920,
    0x00be_ac70,
];
const PASS_CREATORS_MODEL_2: [u32; 13] = [
    0x00bf_66c0,
    0x00be_bd20,
    0x00bf_0720,
    0x00bf_8f70,
    0x00bf_96b0,
    0x00bf_bf60,
    0x00bf_c860,
    0x00c0_1020,
    0x00c0_17c0,
    0x00c0_1c20,
    0x00c0_1f60,
    0x00c0_0be0,
    0x00c0_2340,
];
const PASS_CREATOR_LOW_MODEL_00BEA040: u32 = 0x00be_a040;

// Translated from 00b78980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`): the shader's vtable entry at
/// `010af418`, `thiscall`, no stack words. Calls the shader's own vtable +0x144, then the ten
/// pass creators every shader model runs; with the shader model (`011f91c0`) at 2 or more
/// (signed) the thirteen further creators, below it `00bea040`.
pub fn fn_00b78980(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.vcall(base, 0x144, &args![]);
    for creator in PASS_CREATORS_ALWAYS {
        e.call(creator, &args![]);
    }
    if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
        e.call(PASS_CREATOR_LOW_MODEL_00BEA040, &args![]);
        return;
    }
    for creator in PASS_CREATORS_MODEL_2 {
        e.call(creator, &args![]);
    }
}

/// The shader's pass table (`011fdff8`, words up to `011febd4`).
const PASS_TABLE_011FDFF8: u32 = 0x011f_dff8;
const PASS_TABLE_END_011FEBD4: u32 = 0x011f_ebd4;

// Translated from 00b78a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`): the shader's vtable entry at
/// `010af41c`, `thiscall`, no stack words. Calls the shader's vtable +0x128 with every word of
/// the pass table `011fdff8`..`011febd4` (the loop tests after each call, signed).
pub fn fn_00b78a40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut slot = PASS_TABLE_011FDFF8;
    loop {
        let pass = e.mem.u32(slot);
        e.vcall(base, 0x128, &args![pass]);
        slot += 4;
        if (slot as i32) >= (PASS_TABLE_END_011FEBD4 as i32) {
            break;
        }
    }
}

/// `00b78a70`'s divisor (`012023e0`) and its two results (`011fd954`, `011fd958`).
const DIVISOR_012023E0: u32 = 0x0120_23e0;
const RECIPROCAL_011FD954: u32 = 0x011f_d954;
const ZERO_011FD958: u32 = 0x011f_d958;

// Translated from 00b78a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), no arguments: stores `1 / [012023e0]`
/// (`FLD1`, `FDIVRP`) at `011fd954` and 0.0 at `011fd958`.
pub fn fn_00b78a70(e: &mut Engine) {
    let divisor = x87_load32(e.global::<u32>(DIVISOR_012023E0));
    let value = x87_div(1.0, divisor);
    e.set_global(RECIPROCAL_011FD954, x87_store32(value));
    e.set_global(ZERO_011FD958, 0.0f32);
}

/// `00b97ed0` and `00b97f20` (unit `bsrenderstate_xenon.cpp` range, unnamed, `cdecl` with two
/// and three words).
const RENDER_STATE_00B97ED0: u32 = 0x00b9_7ed0;
const RENDER_STATE_00B97F20: u32 = 0x00b9_7f20;

/// The pass of the pass table for the rendering object's pass type (`u16` at +4).
fn pass_for(e: &Engine, rendering: u32) -> u32 {
    let kind = u32::from(e.mem.u16(rendering + 4));
    e.mem.u32(PASS_TABLE_011FDFF8 + 4 * kind)
}

// Translated from 00b7ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with three stack words (the
/// first unread, the shader property, the rendering object); `SetupGeometryConstants` calls it
/// for pass types 4 and 5. With a rendering object and a property whose +0xdc is set: the first
/// stage of the pass for the object's type gets that block's +0x6c texture (`011f952c` when it
/// is null) and the property's mode (`00e7de90`), then `00b97ed0(1, 0)`, `00b97f20(4, block
/// +0x70, 0)`, the shader's vtable +0x108, and the pass becomes the current one (`0126f74c`).
pub fn fn_00b7ad00(e: &mut Engine, this: Ptr, _unused_0: u32, property: u32, rendering: u32) {
    if rendering == 0 {
        return;
    }
    let block = e.mem.u32(property + 0xdc);
    let pass = pass_for(e, rendering);
    if block == 0 {
        return;
    }
    let mut texture = e.mem.u32(block + 0x6c);
    let stages = e.mem.u32(pass + 0x24);
    let stage = e.mem.u32(stages);
    if texture == 0 {
        texture = e.global::<u32>(DEFAULT_TEXTURE_011F952C);
    }
    e.mem.set_u32(stage + 8, texture);
    let mode = e.vcall(property, 0xe0, &args![]).u32();
    e.call(STAGE_MODE_00E7DE90, &args![stage, mode]);
    e.call(RENDER_STATE_00B97ED0, &args![1u32, 0u32]);
    let value = e.mem.u32(block + 0x70);
    e.call(RENDER_STATE_00B97F20, &args![4u32, value, 0u32]);
    e.vcall(this.addr(), 0x108, &args![]);
    e.set_global(CURRENT_PASS_0126F74C, pass);
}

/// The globals `00b7ad80` fills: eleven words from the property's +0xdc block (`011fd8f4` to
/// `011fd910`, `011fd85c`, `011fd860`, `011fd914`), 1.0 at `011fd918`, and the fog values
/// (`011fd924` to `011fd940`).
const BLOCK_WORDS_011FD8F4: u32 = 0x011f_d8f4;
const BLOCK_VALUE_011FD85C: u32 = 0x011f_d85c;
const BLOCK_VALUE_011FD860: u32 = 0x011f_d860;
const BLOCK_VALUE_011FD914: u32 = 0x011f_d914;
const ONE_011FD918: u32 = 0x011f_d918;
const FOG_COLOUR_011FD924: u32 = 0x011f_d924;
const FOG_FAR_011FD934: u32 = 0x011f_d934;
const FOG_RANGE_011FD938: u32 = 0x011f_d938;
const FOG_POWER_011FD93C: u32 = 0x011f_d93c;
const FOG_LAST_011FD940: u32 = 0x011f_d940;

// Translated from 00b7ad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with two stack words (a
/// word tested for zero, the shader property; `ECX` unread). With a property: its +0xdc block
/// (when set) gives the words +0xc..+0x28 (copied as they are) to `011fd8f4`..`011fd910`, the
/// floats +0x4c, +0x50, +0x54 (through `FLD`/`FSTP`) to `011fd85c`, `011fd860`, `011fd914`,
/// and 1.0 to `011fd918`. The fog values `011fd924`..`011fd940` are cleared to 0.0; with the
/// first word non-zero and a fog property (`BSShaderManager::GetFogProperty` of the byte
/// `011f91c4`) whose far (+0x30) or near (+0x2c) distance is not 0.0 (a NaN counts as not
/// equal), they become the fog colour (+0x20, +0x24, +0x28, then 0.0), the far distance, far
/// minus near, the fog's +0x60 and 0.0.
pub fn fn_00b7ad80(e: &mut Engine, _this: Ptr, enabled: u32, property: u32) {
    if property == 0 {
        return;
    }
    let block = e.mem.u32(property + 0xdc);
    if block != 0 {
        for index in 0..8 {
            let word = e.mem.u32(block + 0xc + 4 * index);
            e.set_global(BLOCK_WORDS_011FD8F4 + 4 * index, word);
        }
        for (offset, target) in [
            (0x4cu32, BLOCK_VALUE_011FD85C),
            (0x50, BLOCK_VALUE_011FD860),
            (0x54, BLOCK_VALUE_011FD914),
        ] {
            let value = copy_float(e, block + offset);
            e.set_global(target, value);
        }
        e.set_global(ONE_011FD918, 1.0f32);
    }
    for index in 0..8 {
        e.set_global(FOG_COLOUR_011FD924 + 4 * index, 0.0f32);
    }
    if enabled == 0 {
        return;
    }
    let index = u32::from(e.global::<u8>(FOG_INDEX_011F91C4));
    let fog = e.call(GET_FOG_PROPERTY, &args![index]).u32();
    if fog == 0 {
        return;
    }
    let near = copy_float(e, fog + 0x2c);
    let far = copy_float(e, fog + 0x30);
    let power = copy_float(e, fog + 0x60);
    let far_value = x87_load32(far);
    let near_value = x87_load32(near);
    // `FUCOM` against 0.0, `TEST AH,0x44`, `JP`: unequal or unordered writes.
    if far_value == 0.0 && near_value == 0.0 {
        return;
    }
    let range = x87_store32(x87_sub(far_value, near_value));
    let colour = [
        copy_float(e, fog + 0x20),
        copy_float(e, fog + 0x24),
        copy_float(e, fog + 0x28),
    ];
    e.set_global(FOG_FAR_011FD934, far);
    e.set_global(FOG_RANGE_011FD938, range);
    e.set_global(FOG_POWER_011FD93C, power);
    e.set_global(FOG_LAST_011FD940, 0.0f32);
    for (index, word) in colour.iter().enumerate() {
        e.set_global(FOG_COLOUR_011FD924 + 4 * index as u32, *word);
    }
    e.set_global(FOG_COLOUR_011FD924 + 0xc, 0.0f32);
}

// Translated from 00b7c150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with one stack word (its
/// callers set `ECX`, the body never reads it): stage `index` of the current pass (`0126f74c`)
/// gets the texture `011f95e4` at +8.
pub fn fn_00b7c150(e: &mut Engine, _this: Ptr, index: u32) {
    let stage = pass_stage(e, index);
    let texture = e.global::<u32>(PAIR_TEXTURE_011F95E4);
    e.mem.set_u32(stage + 8, texture);
}

// Translated from 00b7c170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with one stack word (`ECX`
/// unread): stage `index` of the current pass gets the texture `011f95e8` at +8.
pub fn fn_00b7c170(e: &mut Engine, _this: Ptr, index: u32) {
    let stage = pass_stage(e, index);
    let texture = e.global::<u32>(PAIR_TEXTURE_011F95E8);
    e.mem.set_u32(stage + 8, texture);
}

/// `00e7f7c0` (unnamed, `thiscall` on a pass, no stack words; answers an object whose vtable
/// +0x5c and +0x68 take four words).
const PASS_OBJECT_00E7F7C0: u32 = 0x00e7_f7c0;

/// What `00e7f7c0` answers for the current pass (`0126f74c`).
fn pass_object(e: &mut Engine) -> u32 {
    let current = e.global::<u32>(CURRENT_PASS_0126F74C);
    e.call(PASS_OBJECT_00E7F7C0, &args![current]).u32()
}

// Translated from 00b7ce90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowlightshader.cpp`), `thiscall` with three stack words (a
/// word passed on to `00b7ad80`, the shader property, the rendering object);
/// `SetupGeometryConstants` calls it for pass types 0x250 and 0x251. With a rendering object
/// and a property whose +0xdc block is set: the pass for the object's type has its first stage
/// get `00c03230(property, 0)` and its second the block's +8 texture (`011f952c` when null),
/// each with the property's mode (`00e7de90`); then, each time on the object `00e7f7c0` answers
/// for the current pass (`0126f74c`, still the old one), vtable +0x5c with (block +0x5c, +0x60,
/// 0, 0), +0x68 with (0xab, block +0x64, 0, 0) and +0x68 with (0x17, block +0x68, 0, 0); then
/// `00b7ad80(first word, property)`, the shader's vtable +0x108, and the pass becomes current.
pub fn fn_00b7ce90(e: &mut Engine, this: Ptr, word: u32, property: u32, rendering: u32) {
    if rendering == 0 {
        return;
    }
    let pass = pass_for(e, rendering);
    let block = e.mem.u32(property + 0xdc);
    if block == 0 {
        return;
    }
    let stages = e.mem.u32(pass + 0x24);
    let first = e.mem.u32(stages);
    let texture = e.call(TEXTURE_00C03230, &args![property, 0u32]).u32();
    e.mem.set_u32(first + 8, texture);
    let mode = e.vcall(property, 0xe0, &args![]).u32();
    e.call(STAGE_MODE_00E7DE90, &args![first, mode]);
    let stages = e.mem.u32(pass + 0x24);
    let texture = e.mem.u32(block + 8);
    let second = e.mem.u32(stages + 4);
    let texture = if texture == 0 {
        e.global::<u32>(DEFAULT_TEXTURE_011F952C)
    } else {
        texture
    };
    e.mem.set_u32(second + 8, texture);
    let mode = e.vcall(property, 0xe0, &args![]).u32();
    e.call(STAGE_MODE_00E7DE90, &args![second, mode]);
    let object = pass_object(e);
    let first_value = e.mem.u32(block + 0x5c);
    let second_value = e.mem.u32(block + 0x60);
    e.vcall(object, 0x5c, &args![first_value, second_value, 0u32, 0u32]);
    let object = pass_object(e);
    let value = e.mem.u32(block + 0x64);
    e.vcall(object, 0x68, &args![0xabu32, value, 0u32, 0u32]);
    let object = pass_object(e);
    let value = e.mem.u32(block + 0x68);
    e.vcall(object, 0x68, &args![0x17u32, value, 0u32, 0u32]);
    fn_00b7ad80(e, this, word, property);
    e.vcall(this.addr(), 0x108, &args![]);
    e.set_global(CURRENT_PASS_0126F74C, pass);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_1b10, fn_00b71b10(Ptr, u32, u32)),
        entry!(0x00b7_8980, fn_00b78980(Ptr)),
        entry!(0x00b7_8a40, fn_00b78a40(Ptr)),
        entry!(0x00b7_8a70, fn_00b78a70()),
        entry!(0x00b7_ad00, fn_00b7ad00(Ptr, u32, u32, u32)),
        entry!(0x00b7_ad80, fn_00b7ad80(Ptr, u32, u32)),
        entry!(0x00b7_c150, fn_00b7c150(Ptr, u32)),
        entry!(0x00b7_c170, fn_00b7c170(Ptr, u32)),
        entry!(0x00b7_ce90, fn_00b7ce90(Ptr, u32, u32, u32)),
        entry!(0x00b7_9050, fn_00b79050(Ptr, u32)),
        entry!(0x00b7_90d0, fn_00b790d0(Ptr, Ptr)),
        entry!(0x00b7_9100, fn_00b79100(Ptr, u32)),
        entry!(0x00b7_9180, fn_00b79180(Ptr, Ptr)),
        entry!(0x00b7_91f0, fn_00b791f0(Ptr, u32, Ptr, Ptr)),
        entry!(0x00b7_9410, fn_00b79410(Ptr, u32, Ptr)),
        entry!(
            0x00b7_95b0,
            shadow_light_shader_setup_geometry_constants_toggles(Ptr, u32, u32, Ptr, u32, u32, Ptr)
        ),
        entry!(0x00b7_96f0, fn_00b796f0(Ptr, Ptr)),
        entry!(0x00b7_9730, fn_00b79730()),
        entry!(0x00b7_98c0, fn_00b798c0(Ptr, Ptr)),
        entry!(0x00b7_c0a0, fn_00b7c0a0(Ptr, u32, u32)),
        entry!(0x00b7_c0e0, fn_00b7c0e0(Ptr, u32, u32)),
        entry!(0x00b7_c120, fn_00b7c120(u32)),
        entry!(0x00b7_c190, fn_00b7c190(u32)),
        entry!(0x00b7_c1f0, fn_00b7c1f0(Ptr, u32, u32, bool)),
        entry!(0x00b7_c230, fn_00b7c230(Ptr, u32, u32, bool)),
        entry!(0x00b7_c290, fn_00b7c290(u32, u32, u32)),
        entry!(0x00b7_c2c0, fn_00b7c2c0(Ptr, u32, u32)),
        entry!(0x00b7_c320, fn_00b7c320(Ptr, Ptr)),
        entry!(0x00b7_c3a0, fn_00b7c3a0(Ptr)),
        entry!(0x00b7_c510, fn_00b7c510(Ptr) -> u32),
        entry!(0x00b7_c580, fn_00b7c580(Ptr)),
        entry!(0x00b7_c600, fn_00b7c600(Ptr)),
        entry!(0x00b7_c750, fn_00b7c750(Ptr, u32, u32)),
        entry!(0x00b7_c7b0, fn_00b7c7b0(u32)),
        entry!(0x00b7_ce30, fn_00b7ce30(u32)),
        entry!(
            0x00b7_ce60,
            shadow_light_shader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        (0x00b7_8ef0, eye_pos_entry as AbiFn),
        entry!(
            0x00b7_b930,
            shadow_light_shader_setup_geometry_constants_shadow_proj(Ptr, Ptr, u32, Ptr)
        ),
        entry!(0x00b7_cfb0, fn_00b7cfb0(Ptr, u32)),
        entry!(
            0x00b7_b730,
            shadow_light_shader_setup_geometry_constants_obj_to_cube(Ptr, Ptr, u32, u32, u32)
        ),
        entry!(0x00b7_c850, fn_00b7c850(u32, u32)),
        entry!(0x00b7_cb00, fn_00b7cb00(u32, u32, u32)),
        entry!(
            0x00b7_be00,
            shadow_light_shader_setup_geometry_constants_simple_shadow_clip_planes(Ptr, u32)
        ),
        entry!(
            0x00b7_b450,
            shadow_light_shader_setup_pass_constants(Ptr, u32)
        ),
        (0x00b7_8a90, lights_entry as AbiFn),
        entry!(0x00b7_d200, fn_00b7d200(Ptr, Ptr)),
        entry!(0x00b7_4210, shadow_light_shader_load_pixel_shaders(Ptr)),
        entry!(0x00b7_1bf0, shadow_light_shader_load_vertex_shaders(Ptr)),
        entry!(0x00b7_e430, shadow_light_shader_create_constant_maps(Ptr)),
        entry!(
            0x00b7_dab0,
            shadow_light_shader_setup_geometry_constants(Ptr, Ptr)
        ),
        entry!(
            0x00b7_ba70,
            shadow_light_shader_setup_geometry_constants_ambient_emittance(Ptr, Ptr, Ptr, Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An entry (0x38 bytes) enabled or not.
    pub(super) fn constant_entry(e: &mut Engine, enabled: bool) -> u32 {
        let entry = e.mem.alloc(0x38);
        e.mem.set_u8(entry + 8, u8::from(enabled));
        entry
    }

    /// An engine with the shader's globals mapped, every vertex and pixel entry set (enabled),
    /// the given doubles and the call log on.
    pub(super) fn rig(doubles: &[u32]) -> Engine {
        let mut e = Engine::new();
        for target in doubles {
            e.register(*target, |_, _| Ret::default());
        }
        for page in [
            0x0040_4000u32,
            0x0101_2000,
            0x0101_5000,
            0x0101_7000,
            0x0101_a000,
            0x0101_e000,
            0x0102_0000,
            0x0104_e000,
            0x010a_d000,
            0x010a_f000,
            0x011a_9000,
            0x011a_d000,
            0x011f_4000,
            0x011f_9000,
            0x011f_a000,
            0x011f_c000,
            0x011f_d000,
            0x011f_e000,
            0x011f_f000,
            0x0126_f000,
        ] {
            e.map(page, 0x1000);
        }
        for index in 0..22 {
            let entry = constant_entry(&mut e, true);
            e.set_global(VERTEX_ENTRIES_011FEBD8 + 4 * index, entry);
            let entry = constant_entry(&mut e, true);
            e.set_global(PIXEL_ENTRIES_011FEC30 + 4 * index, entry);
        }
        e.call_log = Some(vec![]);
        e
    }

    pub(super) fn disable(e: &mut Engine, slot: u32) {
        let entry = e.global::<u32>(slot);
        e.mem.set_u8(entry + 8, 0);
    }

    pub(super) fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    pub(super) fn floats(e: &Engine, addr: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|index| e.mem.f32(addr + 4 * index))
            .collect()
    }

    pub(super) fn set_floats(e: &mut Engine, addr: u32, values: &[f32]) {
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(addr + 4 * index as u32, *value);
        }
    }

    #[test]
    fn fn_00b79050_copies_an_identity_matrix() {
        let mut e = rig(&[]);
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.call(0x00b7_9050, &args![0x5555u32, 0u32]);
        assert!(calls(&e, MEMCPY).is_empty());
        e.call(0x00b7_9050, &args![0x5555u32, 1u32]);
        let matrix = floats(&e, MATRIX_011FEC88, 16);
        for (index, value) in matrix.iter().enumerate() {
            let expected = if index % 5 == 0 { 1.0 } else { 0.0 };
            assert_eq!(*value, expected);
        }
        disable(&mut e, VERTEX_ENTRIES_011FEBD8 + 4 * 12);
        e.call(0x00b7_9050, &args![0x5555u32, 1u32]);
        assert_eq!(calls(&e, MEMCPY).len(), 1);
    }

    #[test]
    fn fn_00b790d0_turns_the_time_into_an_angle() {
        let mut e = rig(&[]);
        e.set_global(GAME_TIME_011F940C, 1800.0f32);
        e.set_global(SECONDS_PER_HOUR_01012640, 3600.0f64);
        e.set_global(TWO_PI_0104EEA0, 6.5f64);
        let object = e.mem.alloc(0xe8);
        e.mem.set_u32(object + 0xe4, 2);
        e.call(0x00b7_90d0, &args![0x5555u32, object]);
        assert_eq!(e.global::<f32>(ANGLE_011FD868), 6.5);
        disable(&mut e, VERTEX_ENTRIES_011FEBD8 + 4 * 15);
        e.set_global(ANGLE_011FD868, 0u32);
        e.call(0x00b7_90d0, &args![0x5555u32, object]);
        assert_eq!(e.global::<u32>(ANGLE_011FD868), 0);
    }

    #[test]
    fn fn_00b79100_subtracts_the_camera_and_fifteen() {
        let mut e = rig(&[]);
        set_floats(&mut e, SOURCE_011F95F4, &[10.0, 20.0, 30.0, 40.0]);
        set_floats(&mut e, CAMERA_011F474C, &[1.0, 2.0]);
        e.set_global(FIFTEEN_01015A38, 15.0f64);
        e.call(0x00b7_9100, &args![0x5555u32, 0u32]);
        assert_eq!(floats(&e, TARGET_011FA2E0, 4), vec![9.0, 18.0, 15.0, 25.0]);
    }

    #[test]
    fn fn_00b79180_uses_the_default_position_without_one() {
        let mut e = rig(&[]);
        set_floats(&mut e, CAMERA_011F474C, &[1.0, 2.0, 3.0]);
        set_floats(&mut e, DEFAULT_POSITION_011F4288, &[5.0, 5.0, 5.0]);
        let object = e.mem.alloc(0x24);
        e.call(0x00b7_9180, &args![0x5555u32, object]);
        assert_eq!(floats(&e, OFFSET_011FD8B4, 3), vec![4.0, 3.0, 2.0]);
        let position = e.mem.alloc(0xc);
        set_floats(&mut e, position, &[2.0, 4.0, 8.0]);
        e.mem.set_u32(object + 0x20, position);
        e.call(0x00b7_9180, &args![0x5555u32, object]);
        assert_eq!(floats(&e, OFFSET_011FD8B4, 3), vec![1.0, 2.0, 5.0]);
    }

    #[test]
    fn fn_00b791f0_writes_the_vector_by_type() {
        let mut e = rig(&[]);
        e.register(FLOAT_MAX_00404010, |_, a| {
            f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))).into_ret()
        });
        e.register(FLOAT_MIN_0040EBD0, |_, a| {
            f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))).into_ret()
        });
        let first = e.mem.alloc(0xa4);
        e.mem.set_f32(first + 0x80, 3.0);
        set_floats(&mut e, first + 0x94, &[1.0, 2.0, 9.0, 9.0]);
        let second = e.mem.alloc(0x98);
        set_floats(&mut e, second + 0x8c, &[0.5, 0.25]);
        e.set_global(VALUE_011AD808, 4.0f32);
        e.call(0x00b7_91f0, &args![0x5555u32, 0xfeu32, first, second]);
        assert_eq!(floats(&e, VECTOR_011FA0B0, 4), vec![3.0, 4.0, 0.0, 0.0]);
        e.call(0x00b7_91f0, &args![0x5555u32, 0x61u32, first, second]);
        assert_eq!(floats(&e, VECTOR_011FA0B0, 4), vec![1.0, 0.0, 0.0, 0.0]);
        // Halfway: time 15, start 10, duration 2 (times 5).
        e.set_global(GAME_TIME_011F940C, 15.0f32);
        e.set_global(START_TIME_011F94AC, 10.0f32);
        e.set_global(DURATION_011AD838, 2.0f32);
        e.set_global(FIVE_01020998, 5.0f64);
        set_floats(&mut e, TO_X_011F962C, &[10.0, 20.0, 2.0, 4.0]);
        e.set_global(RUNNING_011F94AA, 1u8);
        e.call(0x00b7_91f0, &args![0x5555u32, 0x230u32, first, second]);
        assert_eq!(floats(&e, VECTOR_011FA0B0, 4), vec![1.0, 2.0, 5.5, 11.75]);
        assert_eq!(e.global::<u8>(RUNNING_011F94AA), 1);
        assert_eq!(calls(&e, FLOAT_MAX_00404010)[0], vec![0, 0.5f32.to_bits()]);
        // Past the end the factor is 1.0, which clears the running byte.
        e.set_global(GAME_TIME_011F940C, 40.0f32);
        e.call(0x00b7_91f0, &args![0x5555u32, 0x230u32, first, second]);
        assert_eq!(floats(&e, VECTOR_011FA0B0 + 8, 2), vec![9.5, 19.75]);
        assert_eq!(e.global::<u8>(RUNNING_011F94AA), 0);
    }

    #[test]
    fn fn_00b79410_writes_the_selector_rows() {
        let mut e = rig(&[]);
        let object = e.mem.alloc(0xc);
        e.mem.set_u8(object + 9, 5);
        e.mem.set_u8(object + 0xb, 3);
        e.call(0x00b7_9410, &args![0x5555u32, 0x93u32, object]);
        let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 8);
        assert_eq!(e.mem.u32(entry + 0x20), 7);
        assert_eq!(
            floats(&e, ROWS_011FA0C0 + 0x20, 4),
            vec![0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(floats(&e, ROWS_011FA0C0 + 0x30, 4), vec![0.0; 4]);
        e.mem.set_u8(object + 9, 0);
        e.mem.set_u8(object + 0xb, 6);
        e.call(0x00b7_9410, &args![0x5555u32, 0x1f3u32, object]);
        assert_eq!(e.mem.u32(entry + 0x20), 3);
        assert_eq!(
            floats(&e, ROWS_011FA0C0 + 0x30, 4),
            vec![0.0, 1.0, 0.0, 0.0]
        );
        e.mem.set_u8(object + 0xb, 1);
        e.call(0x00b7_9410, &args![0x5555u32, 0x1f6u32, object]);
        assert_eq!(floats(&e, ROWS_011FA0C0, 4), vec![1.0, 0.0, 0.0, 0.0]);
        e.mem.set_u8(object + 0xb, 8);
        e.call(0x00b7_9410, &args![0x5555u32, 0x1f6u32, object]);
        assert_eq!(
            floats(&e, ROWS_011FA0C0 + 0x1b0, 4),
            vec![0.0, 0.0, 0.0, 1.0]
        );
        // Other types leave everything.
        e.mem.set_u8(object + 0xb, 2);
        e.call(0x00b7_9410, &args![0x5555u32, 0x1f0u32, object]);
        assert_eq!(floats(&e, ROWS_011FA0C0, 4), vec![0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn setup_geometry_constants_toggles_sets_the_toggles() {
        let mut e = rig(&[UPDATE_TOGGLES_LAND]);
        e.register(GET_FOG_PROPERTY, |_, _| 0x0e00_0f00u32.into_ret());
        let shader = e.mem.alloc(0x8c);
        e.put_vtable(0x0e10_0000, &[0; 0x56]);
        e.mem.set_u32(0x0e10_0000 + 0x154, 0x0e11_0154);
        e.mem.set_u32(shader, 0x0e10_0000);
        e.register(0x0e11_0154, |_, _| Ret::default());
        let property = e.mem.alloc(0x34);
        e.mem.set_f32(property + 0x30, 0.75);
        e.put_vtable(0x0e12_0000, &[0; 0x3e]);
        e.mem.set_u32(0x0e12_0000 + 0xf4, 0x0e13_00f4);
        e.mem.set_u32(property, 0x0e12_0000);
        e.register(0x0e13_00f4, |_, _| 1u32.into_ret());
        let object = e.mem.alloc(0xc);
        let flags = e.mem.alloc(0x1c);
        e.mem.set_u16(flags + 0x18, 0x200);
        e.set_global(ONE_01012070, 1.0f64);
        e.set_global(BYTE_SCALE_0101E568, 255.0f64);
        e.set_global(TOGGLE_011FA2D4, 5.0f32);
        let run = |e: &mut Engine, pass_type: u32| {
            e.call(
                0x00b7_95b0,
                &args![shader, pass_type, 0x77u32, property, 0x88u32, object, flags],
            );
        };
        // Bit 9: max(1, byte) / 255; the object's byte +8 clear zeroes 011fa2d4.
        e.mem.set_u8(flags + 0x1a, 0);
        run(&mut e, 0x10);
        assert_eq!(
            calls(&e, 0x0e11_0154),
            vec![vec![shader, 0x77, 0x0e00_0f00, 0x88]]
        );
        assert_eq!(e.global::<f32>(TOGGLE_011FA2DC), (1.0f64 / 255.0) as f32);
        assert_eq!(e.global::<u32>(TOGGLE_011FA2D4), 0);
        e.mem.set_u8(flags + 0x1a, 51);
        run(&mut e, 0x10);
        assert_eq!(e.global::<f32>(TOGGLE_011FA2DC), (51.0f64 / 255.0) as f32);
        // Bit 0 without bit 9: 010af2f0; neither: 0.
        e.mem.set_u16(flags + 0x18, 1);
        e.set_global(TOGGLE_VALUE_010AF2F0, 0.5f32);
        run(&mut e, 0x1f7);
        assert_eq!(
            calls(&e, UPDATE_TOGGLES_LAND),
            vec![vec![shader, 0x77, 0, property, 0]]
        );
        assert_eq!(e.global::<f32>(TOGGLE_011FA2DC), 0.5);
        e.mem.set_u16(flags + 0x18, 0);
        run(&mut e, 0x10);
        assert_eq!(e.global::<f32>(TOGGLE_011FA2DC), 0.0);
        // Types 0x248 to 0x24b: the property answers 1.0 and its +0x30.
        run(&mut e, 0x249);
        assert_eq!(e.global::<f32>(TOGGLE_011FA2DC), 1.0);
        assert_eq!(e.global::<f32>(TOGGLE_011FA2D8), 0.75);
        // Disabled: nothing at all.
        disable(&mut e, PIXEL_ENTRIES_011FEC30 + 16);
        e.call_log = Some(vec![]);
        run(&mut e, 0x10);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00b796f0_writes_the_two_floats() {
        let mut e = rig(&[]);
        let object = e.mem.alloc(0xe4);
        e.mem.set_f32(object + 0xe0, 2.5);
        e.set_global(VALUE_010177E8, 0.9f32);
        e.call(0x00b7_96f0, &args![0x5555u32, object]);
        assert_eq!(floats(&e, VALUE_011FD944, 2), vec![2.5, 1.0]);
        e.mem.set_u8(object + 0x24, 0x10);
        e.call(0x00b7_96f0, &args![0x5555u32, object]);
        assert_eq!(floats(&e, VALUE_011FD944, 2), vec![2.5, 0.9]);
    }

    #[test]
    fn fn_00b79730_writes_the_plane_and_range() {
        let mut e = rig(&[]);
        set_floats(&mut e, PLANE_011F9604, &[1.0, 2.0, 3.0, 4.0]);
        set_floats(&mut e, VECTOR_011FA2A0, &[5.0, 6.0, 7.0]);
        e.set_global(MINUS_ONE_01012054, -1.0f32);
        e.call(0x00b7_9730, &args![]);
        assert_eq!(floats(&e, PLANE_011FD8E4, 4), vec![1.0, 2.0, 3.0, 34.0]);
        assert_eq!(floats(&e, RANGE_011FD8C4, 4), vec![0.0, 0.0, -1.0, 0.0]);
        e.set_global(FAR_011F9618, 10.0f32);
        e.set_global(NEAR_011F9614, 4.0f32);
        e.call(0x00b7_9730, &args![]);
        assert_eq!(floats(&e, RANGE_011FD8C4, 4), vec![10.0, 6.0, 1.0, 1.0]);
    }

    #[test]
    fn fn_00b798c0_copies_four_floats() {
        let mut e = rig(&[]);
        let object = e.mem.alloc(0x100);
        set_floats(&mut e, object + 0xf0, &[1.0, 2.0, 3.0, 4.0]);
        e.call(0x00b7_98c0, &args![0x5555u32, object]);
        assert_eq!(floats(&e, VECTOR_011FA300, 4), vec![1.0, 2.0, 4.0, 3.0]);
    }

    /// The current pass with `count` stages (0x10 bytes each), answered.
    pub(super) fn pass_with_stages(e: &mut Engine, count: u32) -> Vec<u32> {
        let stages: Vec<u32> = (0..count).map(|_| e.mem.alloc(0x10)).collect();
        let block = e.mem.alloc(4 * count);
        for (index, stage) in stages.iter().enumerate() {
            e.mem.set_u32(block + 4 * index as u32, *stage);
        }
        let pass = e.mem.alloc(0x60);
        e.mem.set_u32(pass + 0x24, block);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        stages
    }

    /// An object whose vtable +0xe0 answers 9 and +0xf4 answers its +4.
    pub(super) fn texture_object(e: &mut Engine) -> u32 {
        e.put_vtable(0x0e20_0000, &[0; 0x3e]);
        e.mem.set_u32(0x0e20_0000 + 0xe0, 0x0e21_00e0);
        e.mem.set_u32(0x0e20_0000 + 0xf4, 0x0e21_00f4);
        e.register(0x0e21_00e0, |_, _| 9u32.into_ret());
        e.register(0x0e21_00f4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let object = e.mem.alloc(0xb8);
        e.mem.set_u32(object, 0x0e20_0000);
        object
    }

    #[test]
    fn fn_00b7c0a0_sets_the_texture_and_mode() {
        let mut e = rig(&[STAGE_MODE_00E7DE90]);
        let stages = pass_with_stages(&mut e, 2);
        let object = texture_object(&mut e);
        let textures = e.mem.alloc(8);
        e.mem.set_u32(textures + 4, 0x0e30_0000);
        e.mem.set_u32(object + 0xac, textures);
        e.call(0x00b7_c0a0, &args![object, 1u32, 1u32]);
        assert_eq!(e.mem.u32(stages[1] + 8), 0x0e30_0000);
        assert_eq!(calls(&e, STAGE_MODE_00E7DE90), vec![vec![stages[1], 9]]);
    }

    #[test]
    fn fn_00b7c0e0_sets_the_texture_and_mode_with_an_object() {
        let mut e = rig(&[STAGE_MODE_00E7DE90]);
        e.register(TEXTURE_00C03230, |_, a| (a[1] + 0x100).into_ret());
        let stages = pass_with_stages(&mut e, 1);
        let object = texture_object(&mut e);
        e.call(0x00b7_c0e0, &args![object, 0u32, 5u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x105);
        assert_eq!(calls(&e, STAGE_MODE_00E7DE90), vec![vec![stages[0], 9]]);
        e.call(0x00b7_c0e0, &args![0u32, 0u32, 6u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x106);
        assert_eq!(calls(&e, STAGE_MODE_00E7DE90).len(), 1);
    }

    #[test]
    fn fn_00b7c120_takes_the_rendered_texture() {
        let mut e = rig(&[]);
        e.register(RENDERED_TEXTURE_GET_TEXTURE, |_, _| {
            0x0e40_0000u32.into_ret()
        });
        let stages = pass_with_stages(&mut e, 1);
        let object = e.mem.alloc(0x110);
        e.mem.set_u32(object + 0x10c, 0x0e41_0000);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, object);
        let rendering = e.mem.alloc(0x10);
        e.mem.set_u32(rendering + 0xc, holder);
        e.set_global(RENDERING_PASS_011F91E0, rendering);
        e.call(0x00b7_c120, &args![0u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x0e40_0000);
        assert_eq!(
            calls(&e, RENDERED_TEXTURE_GET_TEXTURE),
            vec![vec![0x0e41_0000, 0]]
        );
    }

    #[test]
    fn fn_00b7c190_sets_the_texture_filter_and_sampler() {
        let mut e = rig(&[SET_FILTER_MODE]);
        let stages = pass_with_stages(&mut e, 2);
        e.set_global(TEXTURE_011F953C, 0x0e50_0000u32);
        e.set_global(TEXTURE_011F9540, 0x0e50_0001u32);
        e.set_global(SAMPLER_VALUE_011F9428, 3u32);
        let device = e.mem.alloc(4);
        e.put_vtable(0x0e51_0000, &[0; 0x46]);
        e.mem.set_u32(0x0e51_0000 + 0x114, 0x0e52_0114);
        e.mem.set_u32(device, 0x0e51_0000);
        e.register(0x0e52_0114, |_, _| Ret::default());
        let renderer = e.mem.alloc(0x28c);
        e.mem.set_u32(renderer + 0x288, device);
        e.set_global(RENDERER_011F4748, renderer);
        e.call(0x00b7_c190, &args![1u32]);
        assert_eq!(e.mem.u32(stages[1] + 8), 0x0e50_0001);
        assert!(calls(&e, SET_FILTER_MODE).is_empty());
        assert_eq!(calls(&e, 0x0e52_0114), vec![vec![device, 1, 8, 3]]);
        e.set_global(CHOICE_011AD80C, 1u8);
        e.set_global(FILTER_011F9425, 1u8);
        e.call(0x00b7_c190, &args![1u32]);
        assert_eq!(e.mem.u32(stages[1] + 8), 0x0e50_0000);
        assert_eq!(calls(&e, SET_FILTER_MODE), vec![vec![stages[1], 7]]);
    }

    #[test]
    fn fn_00b7c1f0_falls_back_with_the_flag() {
        let mut e = rig(&[]);
        let stages = pass_with_stages(&mut e, 1);
        e.set_global(FALLBACK_011F951C, 0x0e60_0000u32);
        let textures = e.mem.alloc(8);
        e.mem.set_u32(textures, 0x0e61_0000);
        let object = e.mem.alloc(0xb8);
        e.mem.set_u32(object + 0xb4, textures);
        e.call(0x00b7_c1f0, &args![object, 0u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x0e61_0000);
        e.call(0x00b7_c1f0, &args![object, 0u32, 1u32, 0x100u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0);
        e.call(0x00b7_c1f0, &args![object, 0u32, 1u32, 1u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x0e60_0000);
    }

    #[test]
    fn fn_00b7c230_asks_the_object_twice() {
        let mut e = rig(&[]);
        let stages = pass_with_stages(&mut e, 1);
        e.set_global(FALLBACK_011F951C, 0x0e60_0000u32);
        let object = texture_object(&mut e);
        e.mem.set_u32(object + 4, 0x0e70_0000);
        e.call(0x00b7_c230, &args![object, 0u32, 4u32, 0u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x0e70_0000);
        assert_eq!(
            calls(&e, 0x0e21_00f4),
            vec![vec![object, 3, 4], vec![object, 3, 4]]
        );
        e.mem.set_u32(object + 4, 0);
        e.call(0x00b7_c230, &args![object, 0u32, 4u32, 1u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x0e60_0000);
        e.call(0x00b7_c230, &args![object, 0u32, 4u32, 0u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0);
    }

    #[test]
    fn fn_00b7c290_sets_the_texture_and_mode_0() {
        let mut e = rig(&[STAGE_MODE_00E7DE90]);
        e.register(TEXTURE_00C02F30, |_, a| (a[0] + a[1]).into_ret());
        let stages = pass_with_stages(&mut e, 1);
        e.call(0x00b7_c290, &args![0x100u32, 0u32, 2u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x102);
        assert_eq!(calls(&e, STAGE_MODE_00E7DE90), vec![vec![stages[0], 0]]);
    }

    #[test]
    fn fn_00b7c2c0_falls_back_to_its_texture() {
        let mut e = rig(&[]);
        let stages = pass_with_stages(&mut e, 1);
        e.set_global(FALLBACK_011F9514, 0x0e80_0000u32);
        let object = texture_object(&mut e);
        e.call(0x00b7_c2c0, &args![object, 0u32, 1u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x0e80_0000);
        e.mem.set_u32(object + 4, 0x0e81_0000);
        e.call(0x00b7_c2c0, &args![object, 0u32, 1u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x0e81_0000);
        assert_eq!(calls(&e, 0x0e21_00f4)[2], vec![object, 5, 1]);
    }

    #[test]
    fn fn_00b7c320_sets_the_states_by_pass_type() {
        let states = [
            RENDER_STATE_00B97E80,
            RENDER_STATE_00B98070,
            SET_Z_WRITE_ENABLE,
            RENDER_STATE_00B98380,
        ];
        let mut e = rig(&states);
        let property = e.mem.alloc(0x1c);
        e.mem.set_u8(property + 0x18, 1);
        let shader_property = e.mem.alloc(0x24);
        e.mem.set_u32(shader_property + 0x20, 0x40000);
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block + 0x10, property);
        e.mem.set_u32(block + 0xc, shader_property);
        e.set_global(COUNTER_011FF9DC, 2u32);
        e.set_global(COUNTER_011FFA18, 2u32);
        e.set_global(RENDERING_PASS_TYPE_011F91E4, 0x251u32);
        e.call(0x00b7_c320, &args![0x5555u32, block]);
        assert_eq!(calls(&e, RENDER_STATE_00B97E80), vec![vec![3, 0]]);
        assert_eq!(calls(&e, RENDER_STATE_00B98070), vec![vec![0, 0]]);
        assert!(calls(&e, SET_Z_WRITE_ENABLE).is_empty());
        e.set_global(RENDERING_PASS_TYPE_011F91E4, 8u32);
        e.call(0x00b7_c320, &args![0x5555u32, block]);
        assert_eq!(calls(&e, SET_Z_WRITE_ENABLE), vec![vec![1, 0]]);
        assert_eq!(calls(&e, RENDER_STATE_00B98380), vec![vec![7, 0]]);
        assert_eq!(e.global::<u32>(COUNTER_011FF9DC), 1);
        assert_eq!(e.global::<u32>(COUNTER_011FFA18), 1);
        assert_eq!(calls(&e, RENDER_STATE_00B97E80).len(), 1);
    }

    /// The current pass with a vertex shader whose device's vtable +0x178 is a double that
    /// keeps the constants it gets (register, count, then the floats) at `0x0e90_0000`.
    pub(super) fn pass_with_device(e: &mut Engine) -> u32 {
        let device = e.mem.alloc(4);
        e.put_vtable(0x0e91_0000, &[0; 0x5f]);
        e.mem.set_u32(0x0e91_0000 + 0x178, 0x0e92_0178);
        e.mem.set_u32(device, 0x0e91_0000);
        e.map(0x0e90_0000, 0x100);
        e.register(0x0e92_0178, |e, a| {
            e.mem.set_u32(0x0e90_0000, a[1]);
            e.mem.set_u32(0x0e90_0004, a[3]);
            for index in 0..4 * a[3] {
                let word = e.mem.u32(a[2] + 4 * index);
                e.mem.set_u32(0x0e90_0008 + 4 * index, word);
            }
            Ret::default()
        });
        let shader = e.mem.alloc(0x24);
        e.mem.set_u32(shader + 0x20, device);
        let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
        let pass = if pass == 0 { e.mem.alloc(0x60) } else { pass };
        e.mem.set_u32(pass + 0x5c, shader);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        device
    }

    #[test]
    fn fn_00b7c3a0_uploads_the_fog() {
        let mut e = rig(&[]);
        pass_with_device(&mut e);
        let fog = e.mem.alloc(0x64);
        set_floats(&mut e, fog + 0x20, &[0.25, 0.5, 0.75]);
        set_floats(&mut e, fog + 0x2c, &[100.0, 300.0]);
        e.mem.set_f32(fog + 0x60, 2.0);
        e.register_double(GET_FOG_PROPERTY, move |_, _| fog.into_ret());
        e.call(0x00b7_c3a0, &args![0u32]);
        assert_eq!(
            floats(&e, FOG_011FA280, 8),
            vec![300.0, 200.0, 2.0, 0.0, 0.25, 0.5, 0.75, 0.0]
        );
        assert_eq!(e.mem.u32(0x0e90_0000), 0xe);
        assert_eq!(e.mem.u32(0x0e90_0004), 2);
        assert_eq!(e.mem.f32(0x0e90_0008), 300.0);
        // Both distances 0: the far distance and the default colour.
        set_floats(&mut e, fog + 0x2c, &[0.0, 0.0]);
        e.set_global(FAR_DISTANCE_010AD6E0, 500000.0f32);
        set_floats(&mut e, FOG_COLOUR_011A9BD0, &[0.1, 0.2, 0.3, 0.4]);
        e.call(0x00b7_c3a0, &args![0u32]);
        assert_eq!(
            floats(&e, FOG_011FA280, 8),
            vec![500000.0, 0.0, 0.0, 0.0, 0.1, 0.2, 0.3, 0.4]
        );
    }

    #[test]
    fn fn_00b7c510_enables_the_pixel_entries_by_mask() {
        let mut e = rig(&[]);
        let shader = e.mem.alloc(0x34);
        let map = e.mem.alloc(4);
        e.put_vtable(0x0ea0_0000, &[0; 0xa]);
        e.mem.set_u32(0x0ea0_0000 + 0x24, 0x0ea1_0024);
        e.mem.set_u32(map, 0x0ea0_0000);
        e.register(0x0ea1_0024, |_, _| 0x55u32.into_ret());
        e.mem.set_u32(shader + 0x30, map);
        e.set_global(RENDERING_PASS_TYPE_011F91E4, 3u32);
        e.set_global(PIXEL_FLAGS_011FC0A0 + 12, 0b1_0010u32);
        e.set_global(PIXEL_ENTRIES_011FEC30 + 8, 0u32);
        assert_eq!(e.call(0x00b7_c510, &args![shader]).u32(), 0x55);
        let enabled: Vec<bool> = [0u32, 1, 3, 4]
            .iter()
            .map(|index| entry_enabled(&e, PIXEL_ENTRIES_011FEC30 + 4 * index))
            .collect();
        assert_eq!(enabled, vec![true, false, true, false]);
        assert_eq!(calls(&e, 0x0ea1_0024), vec![vec![map]]);
    }

    /// The rendering object (the first word of the rendering pass's +0xc) with its +0xf8 block.
    pub(super) fn rendering_object_with_values(e: &mut Engine) -> (u32, u32) {
        let values = e.mem.alloc(0x100);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0xf8, values);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, object);
        let rendering = e.mem.alloc(0x10);
        e.mem.set_u32(rendering + 0xc, holder);
        e.set_global(RENDERING_PASS_011F91E0, rendering);
        (object, values)
    }

    #[test]
    fn fn_00b7c580_copies_the_colour() {
        let mut e = rig(&[]);
        let (_, values) = rendering_object_with_values(&mut e);
        set_floats(&mut e, values + 0xc8, &[0.1, 0.2, 0.3]);
        e.call(0x00b7_c580, &args![0u32]);
        assert_eq!(floats(&e, ROWS_011FA0C0, 4), vec![0.1, 0.2, 0.3, 1.0]);
    }

    #[test]
    fn fn_00b7c600_scales_the_colour() {
        let mut e = rig(&[]);
        let (object, values) = rendering_object_with_values(&mut e);
        e.set_global(ONE_01012070, 1.0f64);
        e.mem.set_f32(values + 0xc4, 4.0);
        set_floats(&mut e, values + 0xd4, &[0.5, 0.25, 1.0]);
        e.mem.set_f32(object + 0xd0, 2.0);
        e.mem.set_f32(object + 0xd4, 0.75);
        e.call(0x00b7_c600, &args![0u32]);
        assert_eq!(floats(&e, COLOUR_011FA0D0, 4), vec![1.0, 0.5, 2.0, 0.75]);
        // With HDR the strength stays and the HDR scale applies.
        e.set_global(HDR_011F941E, 1u8);
        e.set_global(HDR_SCALE_011F9190, 0.5f32);
        e.call(0x00b7_c600, &args![0u32]);
        assert_eq!(floats(&e, COLOUR_011FA0D0, 4), vec![2.0, 1.0, 4.0, 0.75]);
        e.set_global(HDR_SCALE_DISABLED_011F9427, 1u8);
        e.call(0x00b7_c600, &args![0u32]);
        assert_eq!(floats(&e, COLOUR_011FA0D0, 3), vec![4.0, 2.0, 8.0]);
    }

    #[test]
    fn fn_00b7c750_uploads_the_transposed_product() {
        let mut e = rig(&[RENDERER_METHOD_00E6FBB0]);
        pass_with_device(&mut e);
        e.register(MATRIX_MULTIPLY_TRANSPOSE, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        let renderer = e.mem.alloc(0xa40);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, renderer);
        e.call(0x00b7_c750, &args![this, 0u32, 7u32]);
        assert_eq!(
            calls(&e, RENDERER_METHOD_00E6FBB0),
            vec![vec![renderer, 7, 0]]
        );
        assert_eq!(e.mem.u32(0x0e90_0000), 0);
        assert_eq!(e.mem.u32(0x0e90_0004), 4);
        assert_eq!(e.mem.u32(0x0e90_0008), renderer + 0x940);
        assert_eq!(e.mem.u32(0x0e90_000c), renderer + 0xa00);
    }

    #[test]
    fn fn_00b7c7b0_transforms_the_negated_vector() {
        let mut e = rig(&[]);
        pass_with_device(&mut e);
        let (_, values) = rendering_object_with_values(&mut e);
        set_floats(&mut e, values + 0xf0, &[1.0, 2.0, 3.0]);
        e.set_global(MINUS_ONE_0101A6B0, -1.0f64);
        let seen = e.mem.alloc(0x10);
        e.register_double(VEC3_TRANSFORM_NORMAL, move |e, a| {
            for index in 0..3 {
                let word = e.mem.u32(a[1] + 4 * index);
                e.mem.set_u32(seen + 4 * index, word);
                let doubled = e.mem.f32(a[1] + 4 * index) * 2.0;
                e.mem.set_f32(a[0] + 4 * index, doubled);
            }
            e.mem.set_u32(seen + 0xc, a[2]);
            a[0].into_ret()
        });
        e.register(VEC3_NORMALIZE, |e, a| {
            e.mem.set_f32(a[0], 99.0);
            a[0].into_ret()
        });
        e.call(0x00b7_c7b0, &args![0x0eb0_0000u32]);
        assert_eq!(floats(&e, seen, 3), vec![-1.0, -2.0, -3.0]);
        assert_eq!(e.mem.u32(seen + 0xc), 0x0eb0_0000);
        assert_eq!(e.mem.u32(0x0e90_0000), 0x19);
        assert_eq!(e.mem.u32(0x0e90_0004), 1);
        assert_eq!(floats(&e, 0x0e90_0008, 3), vec![-2.0, -4.0, -6.0]);
    }

    #[test]
    fn fn_00b7ce30_deletes_the_array() {
        let mut e = rig(&[VECTOR_DESTRUCT, MEMORY_FREE]);
        e.call(0x00b7_ce30, &args![0u32]);
        assert!(calls(&e, MEMORY_FREE).is_empty());
        let block = e.mem.alloc(0xc);
        e.mem.set_u32(block, 2);
        e.call(0x00b7_ce30, &args![block + 4]);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![vec![block + 4, 4, 2, POINTER_DESTRUCT_0045CEC0]]
        );
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, block]]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = rig(&[SHADER_DESTRUCT_00B79DE0, NI_FREE_SIZED_00AA1460]);
        let this = e.mem.alloc(0x8c);
        assert_eq!(e.call(0x00b7_ce60, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, NI_FREE_SIZED_00AA1460).is_empty());
        assert_eq!(e.call(0x00b7_ce60, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0x8c]]);
        assert_eq!(calls(&e, SHADER_DESTRUCT_00B79DE0).len(), 2);
    }
}

#[cfg(test)]
mod batch2_tests {
    use super::tests::{
        calls, disable, floats, pass_with_device, pass_with_stages, rendering_object_with_values,
        rig, set_floats,
    };
    use super::*;

    #[test]
    fn eye_pos_unitizes_for_the_land_types_and_transforms_otherwise() {
        let mut e = rig(&[]);
        e.register(POINT_UNITIZE_004A0C10, |e, a| {
            e.mem.set_f32(a[0] + 8, 0.5);
            Ret::default()
        });
        e.register(VEC3_TRANSFORM_COORD, |e, a| {
            let first_column = e.mem.f32(a[2]);
            for index in 0..3 {
                let value = e.mem.f32(a[1] + 4 * index) * first_column;
                e.mem.set_f32(a[0] + 4 * index, value);
            }
            a[0].into_ret()
        });
        set_floats(&mut e, EYE_011FA2A0, &[10.0, 20.0, 30.0]);
        set_floats(&mut e, CAMERA_011F474C, &[1.0, 2.0, 3.0]);
        let object = e.mem.alloc(0x34);
        set_floats(&mut e, object + 0x24, &[4.0, 5.0, 6.0]);
        e.mem.set_f32(object + 0x30, 7.0);
        let mut words = vec![0x244u32, object];
        words.extend((0..16).map(|index| if index == 0 { 2.0f32.to_bits() } else { 0 }));
        e.call(0x00b7_8ef0, &words);
        assert_eq!(
            floats(&e, EYE_DIRECTION_011FD8A4, 4),
            vec![6.0, 15.0, 0.5, 0.0]
        );
        assert_eq!(floats(&e, EYE_011FD894, 4), vec![9.0, 18.0, 27.0, 1.0]);
        words[0] = 0x10;
        e.call(0x00b7_8ef0, &words);
        assert_eq!(floats(&e, EYE_011FD894, 4), vec![20.0, 40.0, 60.0, 7.0]);
        // Both entries off: nothing.
        disable(&mut e, VERTEX_ENTRIES_011FEBD8 + 4 * 9);
        disable(&mut e, PIXEL_ENTRIES_011FEC30 + 4 * 5);
        e.mem.set_f32(object + 0x30, 8.0);
        e.call(0x00b7_8ef0, &words);
        assert_eq!(e.global::<f32>(EYE_011FD894 + 0xc), 7.0);
    }

    #[test]
    fn shadow_proj_builds_the_matrix_by_the_flag() {
        let mut e = rig(&[]);
        let accumulator = e.mem.alloc(0x198);
        e.mem.set_u32(accumulator + 0x194, 1);
        e.register_double(GET_ACCUMULATOR, move |_, _| accumulator.into_ret());
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0x1c, 3);
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + 0xc, object);
        let transform = e.mem.alloc(0x34);
        let values: Vec<f32> = (1..=13).map(|value| value as f32).collect();
        set_floats(&mut e, transform, &values);
        e.call(0x00b7_b930, &args![0x5555u32, block, 0u32, transform]);
        assert_eq!(
            floats(&e, SHADOW_PROJECTION_011FD968, 16),
            vec![
                1.0, 2.0, 3.0, 10.0, 4.0, 5.0, 6.0, 11.0, 7.0, 8.0, 9.0, 12.0, 0.0, 0.0, 0.0, 13.0
            ]
        );
        set_floats(&mut e, CAMERA_011F474C, &[5.0, 6.0, 7.0]);
        e.call(0x00b7_b930, &args![0x5555u32, block, 1u32, transform]);
        assert_eq!(
            floats(&e, SHADOW_PROJECTION_011FD968, 16),
            vec![1.0, 0.0, 0.0, 5.0, 0.0, 1.0, 0.0, 6.0, 0.0, 0.0, 1.0, 7.0, 0.0, 0.0, 0.0, 1.0]
        );
        // A register of -1 stops it.
        e.mem.set_u32(object + 0x1c, 0xffff_ffff);
        e.call(0x00b7_b930, &args![0x5555u32, block, 0u32, transform]);
        assert_eq!(e.global::<f32>(SHADOW_PROJECTION_011FD968 + 0xc), 5.0);
        assert_eq!(calls(&e, GET_ACCUMULATOR).len(), 2);
    }

    #[test]
    fn fn_00b7cfb0_picks_the_stage_pair_by_type() {
        let mut e = rig(&[]);
        let stages = pass_with_stages(&mut e, 16);
        e.set_global(PAIR_TEXTURE_011F95E4, 0x0e00_00e4u32);
        e.set_global(PAIR_TEXTURE_011F95E8, 0x0e00_00e8u32);
        for (pass_type, first) in [
            (0x94u32, 2usize),
            (0x1f3, 2),
            (0xa7, 6),
            (0x12b, 6),
            (0xf7, 5),
            (0x163, 4),
            (0x1fc, 14),
        ] {
            for stage in &stages {
                e.mem.set_u32(stage + 8, 0);
            }
            e.call(0x00b7_cfb0, &args![0u32, pass_type]);
            assert_eq!(e.mem.u32(stages[first] + 8), 0x0e00_00e4, "{pass_type:#x}");
            assert_eq!(e.mem.u32(stages[first + 1] + 8), 0x0e00_00e8);
        }
        for stage in &stages {
            e.mem.set_u32(stage + 8, 0);
        }
        for pass_type in [0x95u32, 0x1fd, 0x22d, 0x162] {
            e.call(0x00b7_cfb0, &args![0u32, pass_type]);
        }
        assert!(stages.iter().all(|stage| e.mem.u32(stage + 8) == 0));
    }

    #[test]
    fn obj_to_cube_transposes_or_builds_the_matrix() {
        let mut e = rig(&[GET_D3D_FROM_NI]);
        e.register(MATRIX_TRANSPOSE, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x40);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(MATRIX_MULTIPLY, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x40);
            e.mem.write(a[0], &bytes);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        let renderer = e.mem.alloc(0x980);
        e.mem.set_f32(renderer + 0x940, 3.0);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, renderer);
        let block = e.mem.alloc(0x10);
        // No object: the renderer's matrix.
        e.call(0x00b7_b730, &args![this, block, 0x10u32, 0u32, 0u32]);
        assert_eq!(e.global::<f32>(OBJECT_TO_CUBE_011FECC8), 3.0);
        // With an object and the byte: identity plus the camera, times the light's matrix.
        let object = e.mem.alloc(4);
        e.mem.set_u32(block + 0xc, object);
        e.mem.set_u8(block + 9, 1);
        let (light, _) = rendering_object_with_values(&mut e);
        set_floats(&mut e, CAMERA_011F474C, &[1.0, 2.0, 3.0]);
        e.call(0x00b7_b730, &args![this, block, 0x10u32, 1u32, 0u32]);
        let matrix = floats(&e, OBJECT_TO_CUBE_011FECC8, 16);
        assert_eq!(matrix[0], 1.0);
        assert_eq!(e.global::<u32>(OBJECT_TO_CUBE_011FECC8 + 4), light + 0x10);
        assert_eq!(matrix[12..15].to_vec(), vec![1.0, 2.0, 3.0]);
        assert!(calls(&e, GET_D3D_FROM_NI).is_empty());
        e.call(
            0x00b7_b730,
            &args![this, block, 0x10u32, 0u32, 0x0e10_0000u32],
        );
        assert_eq!(calls(&e, GET_D3D_FROM_NI).len(), 1);
        assert_eq!(calls(&e, GET_D3D_FROM_NI)[0][1], 0x0e10_0000);
        // The land types take the renderer's matrix.
        e.call(0x00b7_b730, &args![this, block, 0x245u32, 1u32, 0u32]);
        assert_eq!(e.global::<f32>(OBJECT_TO_CUBE_011FECC8), 3.0);
    }

    /// The rendering pass with `count` lights, each a light object (scale +0xd0) and data
    /// (position +0x8c, strength +0xc4, colour +0xd4, radius +0xe0); and the camera object.
    fn rendering_with_lights(e: &mut Engine, count: u8) -> Vec<u32> {
        let camera = e.mem.alloc(0x1f0);
        set_floats(e, camera + 0x1e4, &[1.0, 1.0, 1.0]);
        e.set_global(CAMERA_OBJECT_011F91C8, camera);
        let lights: Vec<u32> = (0..u32::from(count))
            .map(|index| {
                let values = e.mem.alloc(0xe4);
                set_floats(e, values + 0x8c, &[index as f32, 0.0, 0.0]);
                e.mem.set_f32(values + 0xc4, 0.5);
                set_floats(e, values + 0xd4, &[1.0, 2.0, 4.0]);
                e.mem.set_f32(values + 0xe0, 8.0);
                let light = e.mem.alloc(0x100);
                e.mem.set_u32(light + 0xf8, values);
                e.mem.set_f32(light + 0xd0, 2.0);
                light
            })
            .collect();
        let block = e.mem.alloc(4 * u32::from(count).max(1));
        for (index, light) in lights.iter().enumerate() {
            e.mem.set_u32(block + 4 * index as u32, *light);
        }
        let rendering = e.mem.alloc(0x10);
        e.mem.set_u8(rendering + 9, count);
        e.mem.set_u32(rendering + 0xc, block);
        e.set_global(RENDERING_PASS_011F91E0, rendering);
        lights
    }

    fn light_rig() -> Engine {
        let mut e = rig(&[]);
        e.register(VEC3_TRANSFORM_COORD, |e, a| {
            let bytes = e.mem.bytes(a[1], 0xc);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.map(0x0105_0000, 0x1000);
        e.map(0x0101_8000, 0x1000);
        e.set_global(ONE_01012070, 1.0f64);
        e.set_global(NINE_01050C98, 9.0f32);
        e.set_global(NINETY_NINE_010AF450, 99.0f32);
        e.set_global(THIRTY_01018F5C, 30.0f32);
        // The device keeps every upload's register and first float at 0x0e90_0000 + 8 * call.
        let device = pass_with_device(&mut e);
        let uploads = e.mem.alloc(4);
        e.mem.set_u32(device, 0x0e95_0000);
        e.put_vtable(0x0e95_0000, &[0; 0x5f]);
        e.mem.set_u32(0x0e95_0000 + 0x178, 0x0e96_0178);
        e.register_double(0x0e96_0178, move |e, a| {
            let used = e.mem.u32(uploads);
            e.mem.set_u32(0x0e90_0000 + 8 * used, a[1]);
            let word = e.mem.u32(a[2] + 0xc);
            e.mem.set_u32(0x0e90_0004 + 8 * used, word);
            e.mem.set_u32(uploads, used + 1);
            Ret::default()
        });
        e
    }

    #[test]
    fn fn_00b7c850_uploads_each_light() {
        let mut e = light_rig();
        rendering_with_lights(&mut e, 3);
        e.call(0x00b7_c850, &args![0x0e70_0000u32, 2.0f32.to_bits()]);
        // Constant 0x19 (9, 9, 9, n - 1), then lights 1 and 2 as 0x1a and 0x1b.
        assert_eq!(e.mem.u32(0x0e90_0000), 0x19);
        assert_eq!(e.mem.f32(0x0e90_0004), 2.0);
        assert_eq!(e.mem.u32(0x0e90_0008), 0x1a);
        assert_eq!(e.mem.f32(0x0e90_000c), 4.0);
        assert_eq!(e.mem.u32(0x0e90_0010), 0x1b);
        assert_eq!(
            floats(&e, LIGHT_VECTORS_011FD9A8 + 0x20, 4),
            vec![3.0, 1.0, 1.0, 4.0]
        );
        assert_eq!(
            floats(&e, ROWS_011FA0C0 + 0x10, 4),
            vec![1.0, 2.0, 4.0, 2.0]
        );
        let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 8);
        assert_eq!(e.mem.u32(entry + 0x20), 2);
    }

    /// The rendering pass's first word: an object whose +0xa8 answers 0.25 from its vtable
    /// +0x10c.
    fn answering_holder(e: &mut Engine) {
        let rendering = e.global::<u32>(RENDERING_PASS_011F91E0);
        let holder = e.mem.alloc(0xac);
        let source = e.mem.alloc(4);
        e.put_vtable(0x0e97_0000, &[0; 0x44]);
        e.mem.set_u32(0x0e97_0000 + 0x10c, 0x0e98_010c);
        e.mem.set_u32(source, 0x0e97_0000);
        e.register(0x0e98_010c, |_, _| 0.25f64.into_ret());
        e.mem.set_u32(holder + 0xa8, source);
        e.mem.set_u32(rendering, holder);
    }

    #[test]
    fn fn_00b7cb00_uses_the_object_float_after_the_first_light() {
        let mut e = light_rig();
        rendering_with_lights(&mut e, 3);
        answering_holder(&mut e);
        let object = e.mem.alloc(0x3c);
        e.mem.set_f32(object + 0x38, 5.0);
        e.call(
            0x00b7_cb00,
            &args![0x0e70_0000u32, 2.0f32.to_bits(), object],
        );
        assert_eq!(e.mem.f32(0x0e90_0004), 0.25);
        assert_eq!(e.global::<f32>(ROWS_011FA0C0 + 0x1c), 2.0);
        assert_eq!(e.global::<f32>(ROWS_011FA0C0 + 0x2c), 5.0);
        let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 8);
        assert_eq!(e.mem.u32(entry + 0x20), 2);
        // Two lights: the far plane gets 30.0 without an object.
        let mut e = light_rig();
        rendering_with_lights(&mut e, 2);
        answering_holder(&mut e);
        e.call(0x00b7_cb00, &args![0x0e70_0000u32, 2.0f32.to_bits(), 0u32]);
        assert_eq!(floats(&e, FAR_PLANE_011FA0E0, 4), vec![0.0, 0.0, 0.0, 30.0]);
        let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 8);
        assert_eq!(e.mem.u32(entry + 0x20), 2);
    }

    #[test]
    fn simple_shadow_clip_planes_makes_the_planes_once() {
        let mut e = rig(&[PLANE_NORMALIZE_00EE6DCE]);
        for (target, copy) in [
            (MATRIX_MULTIPLY, 1u32),
            (MATRIX_INVERSE, 2),
            (MATRIX_TRANSPOSE, 1),
        ] {
            e.register_double(target, move |e, a| {
                let bytes = e.mem.bytes(a[copy as usize], 0x40);
                e.mem.write(a[0], &bytes);
                a[0].into_ret()
            });
        }
        e.register(PLANE_TRANSFORM_00EE6DC8, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x10);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        let device = e.mem.alloc(4);
        e.put_vtable(0x0ea0_0000, &[0; 0x38]);
        e.mem.set_u32(0x0ea0_0000 + 0xdc, 0x0ea1_00dc);
        e.mem.set_u32(device, 0x0ea0_0000);
        e.register(0x0ea1_00dc, |_, _| Ret::default());
        let renderer_object = e.mem.alloc(0x28c);
        e.mem.set_u32(renderer_object + 0x288, device);
        e.set_global(RENDERER_011F4748, renderer_object);
        let renderer = e.mem.alloc(0xa00);
        // An identity view matrix with the camera at (1, 2, 3).
        for index in [0u32, 5, 10, 15] {
            e.mem.set_f32(renderer + 0x980 + 4 * index, 1.0);
        }
        set_floats(&mut e, CAMERA_011F474C, &[1.0, 2.0, 3.0]);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, renderer);
        let light = e.mem.alloc(0x210);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, light);
        let rendering = e.mem.alloc(0x10);
        e.mem.set_u32(rendering + 0xc, holder);
        e.set_global(RENDERING_PASS_011F91E0, rendering);
        for plane in 0..6u32 {
            set_floats(
                &mut e,
                light + 0x144 + 0x10 * plane,
                &[1.0, 0.0, 0.0, plane as f32],
            );
        }
        e.call(0x00b7_be00, &args![this, 0x10u32]);
        assert!(calls(&e, 0x0ea1_00dc).is_empty());
        e.call(0x00b7_be00, &args![this, 0x232u32]);
        assert_eq!(e.mem.u8(light + 0x208), 1);
        assert_eq!(
            floats(&e, light + 0x1a8 + 0x50, 4),
            vec![1.0, 0.0, 0.0, -5.0]
        );
        let planes = calls(&e, 0x0ea1_00dc);
        assert_eq!(planes.len(), 6);
        assert_eq!(planes[2][1], 2);
        // Made: the stored planes go to the device directly.
        e.call(0x00b7_be00, &args![this, 0x234u32]);
        let planes = calls(&e, 0x0ea1_00dc);
        assert_eq!(planes.len(), 12);
        assert_eq!(planes[7], vec![device, 1, light + 0x1b8]);
        assert_eq!(calls(&e, MATRIX_MULTIPLY).len(), 1);
    }

    #[test]
    fn setup_pass_constants_enables_sorts_and_copies() {
        let mut e = rig(&[]);
        e.register(GET_FOG_PROPERTY, |_, _| 0u32.into_ret());
        e.register(MATRIX_TRANSPOSE, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        pass_with_device(&mut e);
        let map_vtable = 0x0eb0_0000;
        e.put_vtable(map_vtable, &[0; 0xa]);
        e.mem.set_u32(map_vtable + 0x24, 0x0eb1_0024);
        e.register(0x0eb1_0024, |_, _| Ret::default());
        let vertex_map = e.mem.alloc(4);
        let pixel_map = e.mem.alloc(4);
        e.mem.set_u32(vertex_map, map_vtable);
        e.mem.set_u32(pixel_map, map_vtable);
        let renderer = e.mem.alloc(0xa40);
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0x18, renderer);
        e.mem.set_u32(this + 0x30, pixel_map);
        e.mem.set_u32(this + 0x34, vertex_map);
        let first = e.mem.alloc(0xc0);
        e.mem.set_u32(first + 0xbc, 1);
        let rendering = e.mem.alloc(0x10);
        e.mem.set_u32(rendering, first);
        e.set_global(RENDERING_PASS_011F91E0, rendering);
        e.set_global(VERTEX_FLAGS_011FCC80 + 4 * 5, 1u32 << 9);
        e.set_global(PIXEL_FLAGS_011FC0A0 + 4 * 5, 0u32);
        set_floats(&mut e, FOG_COLOUR_011A9BD0, &[0.1, 0.2, 0.3, 0.4]);
        e.call(0x00b7_b450, &args![this, 5u32]);
        assert!(entry_enabled(&e, VERTEX_ENTRIES_011FEBD8 + 4 * 8));
        assert!(!entry_enabled(&e, VERTEX_ENTRIES_011FEBD8));
        assert!(!entry_enabled(&e, PIXEL_ENTRIES_011FEC30));
        assert_eq!(
            calls(&e, 0x0eb1_0024),
            vec![vec![vertex_map], vec![pixel_map]]
        );
        assert_eq!(calls(&e, GET_FOG_PROPERTY).len(), 1);
        assert_eq!(e.mem.u32(0x0e90_0000), 1);
        assert_eq!(e.mem.u32(0x0e90_0004), 4);
        assert_eq!(e.mem.u32(0x0e90_0008), renderer + 0xa00);
        assert_eq!(
            floats(&e, FOG_COLOURS_011FD9B8 + 0x20, 4),
            vec![0.1, 0.2, 0.3, 0.4]
        );
        // Types 0x250 and 0x251 do nothing.
        e.call_log = Some(vec![]);
        e.call(0x00b7_b450, &args![this, 0x251u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod batch3_tests {
    use super::tests::{calls, disable, floats, rig, set_floats};
    use super::*;

    /// A rendering object of `kind` with the given lights (null allowed).
    fn rendering_object(e: &mut Engine, kind: u16, lights: &[u32]) -> u32 {
        let block = e.mem.alloc(4 * lights.len().max(1) as u32);
        for (index, light) in lights.iter().enumerate() {
            e.mem.set_u32(block + 4 * index as u32, *light);
        }
        let rendering = e.mem.alloc(0x10);
        e.mem.set_u16(rendering + 4, kind);
        e.mem.set_u8(rendering + 9, lights.len() as u8);
        e.mem.set_u32(rendering + 0xc, block);
        rendering
    }

    fn light(e: &mut Engine, point: bool) -> u32 {
        let light = e.mem.alloc(0xf8);
        e.mem.set_u8(light + 0xf4, u8::from(point));
        light
    }

    fn lights_words(
        property: u32,
        rendering: u32,
        scale: u32,
        pass_type: u32,
        flag: u32,
    ) -> Vec<u32> {
        let mut words = vec![property, rendering];
        words.extend((0..16).map(|index| if index == 0 { 2.0f32.to_bits() } else { 0 }));
        words.extend([scale, pass_type, flag]);
        words
    }

    fn lights_rig() -> Engine {
        let mut e = rig(&[LIGHT_SETUP_00B70820]);
        e.map(0x0101_6000, 0x1000);
        e.register(VEC3_TRANSFORM_COORD, |e, a| {
            let factor = e.mem.f32(a[2]);
            for index in 0..3 {
                let value = e.mem.f32(a[1] + 4 * index) * factor;
                e.mem.set_f32(a[0] + 4 * index, value);
            }
            a[0].into_ret()
        });
        e.register(VEC3_TRANSFORM_NORMAL, |e, a| {
            let bytes = e.mem.bytes(a[1], 0xc);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(VEC3_NORMALIZE, |e, a| {
            for index in 0..3 {
                let value = e.mem.f32(a[1] + 4 * index) / 2.0;
                e.mem.set_f32(a[0] + 4 * index, value);
            }
            a[0].into_ret()
        });
        let camera = e.mem.alloc(0x1f0);
        set_floats(&mut e, camera + 0x1e4, &[1.0, 1.0, 1.0]);
        e.set_global(CAMERA_OBJECT_011F91C8, camera);
        for index in 0..8u32 {
            let row = LIGHT_POSITIONS_011FA170 + 0x10 * index;
            set_floats(&mut e, row, &[index as f32, 2.0, 3.0, 4.0]);
            let row = LIGHT_DIRECTIONS_011FA1F0 + 0x10 * index;
            set_floats(&mut e, row, &[8.0, 6.0, 4.0, 9.0]);
        }
        e
    }

    #[test]
    fn lights_in_mode_15_copies_the_manager_colour() {
        let mut e = rig(&[]);
        let object = e.mem.alloc(0x164);
        set_floats(&mut e, object + 0x154, &[0.1, 0.2, 0.3, 0.4]);
        e.register_double(MANAGER_OBJECT_00B4F5D0, move |_, _| object.into_ret());
        e.set_global(SHADER_MODE_011F91B8, 0xfu32);
        let property = e.mem.alloc(0x70);
        e.call(0x00b7_8a90, &lights_words(property, 0, 0, 0, 0));
        assert_eq!(floats(&e, ROWS_011FA0C0, 4), vec![0.1, 0.2, 0.3, 0.4]);
    }

    #[test]
    fn lights_fill_the_light_rows() {
        let mut e = lights_rig();
        let property = e.mem.alloc(0x70);
        e.mem.set_f32(property + 0x6c, 0.75);
        let point = light(&mut e, true);
        let directional = light(&mut e, false);
        let rendering = rendering_object(&mut e, 0x40, &[point, directional, 0]);
        let scale = e.mem.alloc(0x34);
        e.mem.set_f32(scale + 0x30, 2.0);
        e.call(0x00b7_8a90, &lights_words(property, rendering, scale, 0, 0));
        // The point light: (position + camera) * 2, its direction x over the scale.
        assert_eq!(
            floats(&e, LIGHT_VECTORS_011FD9A8, 4),
            vec![2.0, 6.0, 8.0, 4.0]
        );
        // The directional light: the negated direction halved, its fourth word.
        assert_eq!(
            floats(&e, LIGHT_VECTORS_011FD9A8 + 0x10, 4),
            vec![-4.0, -3.0, -2.0, 9.0]
        );
        assert_eq!(
            calls(&e, LIGHT_SETUP_00B70820),
            vec![
                vec![0, point, 0.75f32.to_bits()],
                vec![1, directional, 0.75f32.to_bits()],
                vec![2, 0, 0.75f32.to_bits()]
            ]
        );
        let entry = e.global::<u32>(VERTEX_ENTRIES_011FEBD8 + 4 * 6);
        assert_eq!(e.mem.u32(entry + 0x20), 3);
        let entry = e.global::<u32>(PIXEL_ENTRIES_011FEC30 + 4 * 7);
        assert_eq!(e.mem.u32(entry + 0x20), 2);
        // Types 0x231 to 0x233 scale the position instead; the flag skips both.
        e.mem.set_u16(rendering + 4, 0x232);
        e.call(0x00b7_8a90, &lights_words(property, rendering, scale, 0, 0));
        assert_eq!(
            floats(&e, LIGHT_VECTORS_011FD9A8, 4),
            vec![4.0, 12.0, 16.0, 8.0]
        );
        e.call(0x00b7_8a90, &lights_words(property, rendering, scale, 0, 1));
        assert_eq!(
            floats(&e, LIGHT_VECTORS_011FD9A8, 4),
            vec![2.0, 6.0, 8.0, 8.0]
        );
        // Types 0x10 to 0x13 copy the position rows.
        e.mem.set_u16(rendering + 4, 0x11);
        e.call(0x00b7_8a90, &lights_words(property, rendering, scale, 0, 0));
        assert_eq!(
            floats(&e, LIGHT_VECTORS_011FD9A8 + 0x20, 4),
            vec![2.0, 2.0, 3.0, 8.0]
        );
        let entry = e.global::<u32>(VERTEX_ENTRIES_011FEBD8 + 4 * 6);
        assert_eq!(e.mem.u32(entry + 0x20), 1);
    }

    #[test]
    fn lights_scale_the_colour_with_hdr_and_fill_it_for_type_0x24b() {
        let mut e = lights_rig();
        let property = e.mem.alloc(0x70);
        e.mem.set_u32(property + 0x20, 0x400);
        let directional = light(&mut e, false);
        let rendering = rendering_object(&mut e, 0x40, &[directional]);
        e.set_global(HDR_011F941E, 1u8);
        e.set_global(HDR_LIGHT_SCALE_011F9184, 2.0f32);
        set_floats(&mut e, COLOUR_011FA0D0, &[1.0, 2.0, 3.0, 4.0]);
        e.register(GET_FIRST_ACTIVE_LIGHT, |_, _| 0u32.into_ret());
        e.call(0x00b7_8a90, &lights_words(property, rendering, 0, 0, 0));
        assert_eq!(floats(&e, COLOUR_011FA0D0, 4), vec![2.0, 4.0, 6.0, 8.0]);
        // Pass type 0x24b without an active point light: a copy of row 0, or 0.5s in mode 10.
        e.set_global(HDR_011F941E, 0u8);
        set_floats(&mut e, ROWS_011FA0C0, &[0.1, 0.2, 0.3, 0.4]);
        e.call(0x00b7_8a90, &lights_words(property, rendering, 0, 0x24b, 0));
        assert_eq!(floats(&e, COLOUR_011FA0D0, 4), vec![0.1, 0.2, 0.3, 0.4]);
        e.set_global(SHADER_MODE_011F91B8, 10u32);
        e.set_global(HALF_01016248, 0.5f32);
        e.call(0x00b7_8a90, &lights_words(property, rendering, 0, 0x24b, 0));
        assert_eq!(floats(&e, COLOUR_011FA0D0, 4), vec![0.5, 0.5, 0.5, 1.0]);
        assert_eq!(calls(&e, GET_FIRST_ACTIVE_LIGHT)[0][0], property);
        // An active point light leaves the row.
        let point = light(&mut e, true);
        e.register_double(GET_FIRST_ACTIVE_LIGHT, move |_, _| point.into_ret());
        set_floats(&mut e, COLOUR_011FA0D0, &[7.0, 7.0, 7.0, 7.0]);
        e.call(0x00b7_8a90, &lights_words(property, rendering, 0, 0x24b, 0));
        assert_eq!(e.global::<f32>(COLOUR_011FA0D0), 7.0);
    }

    fn ambient_rig() -> Engine {
        let mut e = rig(&[AMBIENT_00C03180, AMBIENT_00C03010]);
        for page in [0x010a_e000u32, 0x0101_6000] {
            e.map(page, 0x1000);
        }
        e.set_global(WEIGHT_SECOND_010AE8C0, 0.5f64);
        e.set_global(WEIGHT_FIRST_010AE8B8, 0.25f64);
        e.set_global(WEIGHT_THIRD_010AE8B0, 0.25f64);
        let values = e.mem.alloc(0xe4);
        set_floats(&mut e, values + 0xd4, &[0.4, 0.2, 0.8]);
        let camera_light = e.mem.alloc(0xfc);
        e.mem.set_u32(camera_light + 0xf8, values);
        let camera = e.mem.alloc(0xe4);
        e.mem.set_u32(camera + 0xe0, camera_light);
        e.set_global(CAMERA_OBJECT_011F91C8, camera);
        e
    }

    /// A property with the given flags at +0x20 and +0x24, +0x28 = 0.5 and a vtable whose
    /// +0x110 answers 3.0.
    fn ambient_property(e: &mut Engine, flags_20: u32, flags_24: u32) -> u32 {
        e.put_vtable(0x0ec0_0000, &[0; 0x45]);
        e.mem.set_u32(0x0ec0_0000 + 0x110, 0x0ec1_0110);
        e.register(0x0ec1_0110, |_, _| 3.0f64.into_ret());
        let property = e.mem.alloc(0x2c);
        e.mem.set_u32(property, 0x0ec0_0000);
        e.mem.set_u32(property + 0x20, flags_20);
        e.mem.set_u32(property + 0x24, flags_24);
        e.mem.set_f32(property + 0x28, 0.5);
        property
    }

    #[test]
    fn ambient_emittance_keeps_the_row_and_picks_the_helper() {
        let mut e = ambient_rig();
        let property = ambient_property(&mut e, 0, 0);
        set_floats(&mut e, ROWS_011FA0C0, &[0.1, 0.2, 0.3, 9.0]);
        let rendering = rendering_object(&mut e, 0xc5, &[0, 0]);
        let current = rendering_object(&mut e, 0x10, &[]);
        e.set_global(RENDERING_PASS_011F91E0, current);
        e.call(
            0x00b7_ba70,
            &args![0x5555u32, property, rendering, 0u32, 7u32],
        );
        assert_eq!(floats(&e, ROWS_011FA0C0, 4), vec![0.1, 0.2, 0.3, 0.5]);
        assert_eq!(calls(&e, AMBIENT_00C03010), vec![vec![7]]);
        assert_eq!(e.global::<f32>(LIGHT_COUNT_011FD95C), 2.0);
        assert_eq!(e.global::<f32>(LIGHT_COUNT_011FA27C), 2.0);
        e.mem.set_u16(rendering + 4, 0xd7);
        e.call(
            0x00b7_ba70,
            &args![0x5555u32, property, rendering, 0u32, 8u32],
        );
        assert_eq!(calls(&e, AMBIENT_00C03180), vec![vec![8]]);
        disable(&mut e, PIXEL_ENTRIES_011FEC30 + 4 * 3);
        e.call(
            0x00b7_ba70,
            &args![0x5555u32, property, rendering, 0u32, 9u32],
        );
        assert_eq!(calls(&e, AMBIENT_00C03010).len(), 2);
        // White ambient.
        e.set_global(WHITE_AMBIENT_011F91A7, 1u8);
        e.call(
            0x00b7_ba70,
            &args![0x5555u32, property, rendering, 0u32, 9u32],
        );
        assert_eq!(floats(&e, ROWS_011FA0C0, 4), vec![1.0; 4]);
    }

    #[test]
    fn ambient_emittance_scales_by_the_brighter_light() {
        let mut e = ambient_rig();
        let property = ambient_property(&mut e, 0, 0x8000);
        let rendering = rendering_object(&mut e, 0x10, &[]);
        let current = rendering_object(&mut e, 0x245, &[]);
        let first_word = e.mem.alloc(0x24);
        e.mem.set_u32(current, first_word);
        e.set_global(RENDERING_PASS_011F91E0, current);
        // Without a light: the camera light luminance (0.2 * 0.5 + 0.4 * 0.25 + 0.8 * 0.25).
        e.register(GET_FIRST_ACTIVE_NON_SHADOW_LIGHT, |_, _| 0u32.into_ret());
        e.call(
            0x00b7_ba70,
            &args![0x5555u32, property, rendering, 0u32, 0u32],
        );
        let luminance = (0.2f32 as f64 * 0.5 + 0.4f32 as f64 * 0.25) + 0.8f32 as f64 * 0.25;
        let expected = (luminance as f32 as f64 * 0.5) as f32;
        assert_eq!(e.global::<f32>(ROWS_011FA0C0 + 0xc), expected);
        // With a light whose scaled luminance is 1.0 at distance 0: it wins.
        let light = e.mem.alloc(0xfc);
        let values = e.mem.alloc(0xe4);
        e.mem.set_f32(values + 0xe0, 10.0);
        e.mem.set_f32(values + 0xc4, 1.0);
        e.mem.set_u32(light + 0xf8, values);
        e.register_double(GET_FIRST_ACTIVE_NON_SHADOW_LIGHT, move |_, _| {
            light.into_ret()
        });
        e.register(POINT_DIFFERENCE_00439EF0, |_, _| Ret::default());
        e.register(COLOUR_SCALE_00633B40, |_, _| Ret::default());
        e.register(POINT_LENGTH_00457990, |_, _| 0.0f64.into_ret());
        e.register(FLOAT_HELPER_0068B050, |_, a| {
            f64::from(f32::from_bits(a[2])).max(0.0).into_ret()
        });
        e.register(DOUBLE_HELPER_00B719C0, |_, a| {
            let second = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            second.min(1.0).into_ret()
        });
        e.register(COLOUR_LUMINANCE_00B719E0, |_, _| 1.0f64.into_ret());
        e.call(
            0x00b7_ba70,
            &args![0x5555u32, property, rendering, 0u32, 0u32],
        );
        assert_eq!(e.global::<f32>(ROWS_011FA0C0 + 0xc), 0.5);
        let scale_call = &calls(&e, COLOUR_SCALE_00633B40)[0];
        assert_eq!(scale_call[0], values + 0xd4);
        assert_eq!(scale_call[2], 1.0f32.to_bits());
        assert_eq!(
            calls(&e, POINT_DIFFERENCE_00439EF0)[0][2],
            DEFAULT_POSITION_011F4288
        );
    }

    #[test]
    fn ambient_emittance_mode_8_and_the_pass_factor() {
        let mut e = ambient_rig();
        let property = ambient_property(&mut e, 0, 0x20000);
        let rendering = rendering_object(&mut e, 0x10, &[]);
        let current = rendering_object(&mut e, 0x10, &[]);
        e.set_global(RENDERING_PASS_011F91E0, current);
        let flags_property = e.mem.alloc(0x1c);
        let block = e.mem.alloc(0xa0);
        e.mem.set_u32(block + 0x9c, flags_property);
        e.set_global(SHADER_MODE_011F91B8, 8u32);
        e.call(
            0x00b7_ba70,
            &args![0x5555u32, property, rendering, block, 0u32],
        );
        assert_eq!(floats(&e, ROWS_011FA0C0, 4), vec![0.0, 0.0, 1.0, 0.0]);
        e.mem.set_u16(flags_property + 0x18, 0x200);
        e.mem.set_u32(property + 0x24, 0);
        e.call(
            0x00b7_ba70,
            &args![0x5555u32, property, rendering, block, 0u32],
        );
        assert_eq!(floats(&e, ROWS_011FA0C0, 4), vec![0.0, 0.0, 0.0, 1.0]);
        // Pass types 0x244 to 0x24b multiply the fourth word by the property's factor.
        e.set_global(RENDERING_PASS_TYPE_011F91E4, 0x246u32);
        e.call(
            0x00b7_ba70,
            &args![0x5555u32, property, rendering, block, 0u32],
        );
        assert_eq!(e.global::<f32>(ROWS_011FA0C0 + 0xc), 3.0);
        e.set_global(SHADER_MODE_011F91B8, 10u32);
        e.call(
            0x00b7_ba70,
            &args![0x5555u32, property, rendering, block, 0u32],
        );
        assert_eq!(e.global::<f32>(ROWS_011FA0C0 + 0xc), 0.5);
    }
}

#[cfg(test)]
mod batch4_tests {
    use super::tests::{calls, pass_with_stages, rig};
    use super::*;

    /// The shader (vtable slot +0x108 a double) and the rendering object of `kind`.
    fn shader_and_rendering(e: &mut Engine, kind: u16) -> (u32, u32) {
        e.put_vtable(0x0ed0_0000, &[0; 0x43]);
        e.mem.set_u32(0x0ed0_0000 + 0x108, 0x0ed1_0108);
        e.register(0x0ed1_0108, |_, _| Ret::default());
        let shader = e.mem.alloc(0x40);
        e.mem.set_u32(shader, 0x0ed0_0000);
        let rendering = e.mem.alloc(0x10);
        e.mem.set_u16(rendering + 4, kind);
        e.mem.set_u8(rendering + 0xb, 5);
        e.set_global(RENDERING_PASS_011F91E0, rendering);
        (shader, rendering)
    }

    /// A shader property: texture arrays at +0xac, +0xb0 and +0xb4 (words 0x100 + i, 0x200 + i,
    /// 0x300 + i), its vtable +0xe0 answering 9.
    fn property(e: &mut Engine) -> u32 {
        e.put_vtable(0x0ed2_0000, &[0; 0x3e]);
        e.mem.set_u32(0x0ed2_0000 + 0xe0, 0x0ed3_00e0);
        e.register(0x0ed3_00e0, |_, _| 9u32.into_ret());
        let property = e.mem.alloc(0xe0);
        e.mem.set_u32(property, 0x0ed2_0000);
        for (offset, base) in [(0xacu32, 0x100u32), (0xb0, 0x200), (0xb4, 0x300)] {
            let textures = e.mem.alloc(0x20);
            for index in 0..8 {
                e.mem.set_u32(textures + 4 * index, base + index);
            }
            e.mem.set_u32(property + offset, textures);
        }
        property
    }

    fn block(e: &mut Engine, property: u32) -> u32 {
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block + 0xc, property);
        block
    }

    fn textures(e: &Engine, stages: &[u32]) -> Vec<u32> {
        stages.iter().map(|stage| e.mem.u32(stage + 8)).collect()
    }

    #[test]
    fn fn_00b7d200_sets_the_stage_textures_by_type() {
        let mut e = rig(&[STAGE_MODE_00E7DE90, STAGE_HELPER_00B71B10, TEXTURE_00C03230]);
        let stages = pass_with_stages(&mut e, 14);
        let (shader, rendering) = shader_and_rendering(&mut e, 2);
        let property = property(&mut e);
        let block = block(&mut e, property);
        // Body 0: the first texture of +0xac on stage 0.
        e.call(0x00b7_d200, &args![shader, block]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x100);
        assert_eq!(calls(&e, 0x0ed1_0108), vec![vec![shader]]);
        // Body 2 (type 0x59): stage 1 gets +0xb4's first texture.
        e.mem.set_u16(rendering + 4, 0x59);
        e.call(0x00b7_d200, &args![shader, block]);
        assert_eq!(e.mem.u32(stages[1] + 8), 0x300);
        // Body 7 (type 0x93): textures by the rendering object's byte +0xb.
        e.mem.set_u16(rendering + 4, 0x93);
        e.call(0x00b7_d200, &args![shader, block]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x105);
        // Bodies 17 to 23 copy both sets; bit 0x100000 keeps the second.
        e.mem.set_u16(rendering + 4, 0x22a);
        e.call(0x00b7_d200, &args![shader, block]);
        let all = textures(&e, &stages);
        assert_eq!(all[..7].to_vec(), (0x100..0x107).collect::<Vec<u32>>());
        assert_eq!(all[7..14].to_vec(), (0x200..0x207).collect::<Vec<u32>>());
        for stage in &stages {
            e.mem.set_u32(stage + 8, 0);
        }
        e.mem.set_u32(property + 0x24, 0x100000);
        e.mem.set_u16(rendering + 4, 0x1f8);
        e.call(0x00b7_d200, &args![shader, block]);
        assert_eq!(textures(&e, &stages)[..2].to_vec(), vec![0x100, 0]);
        // Body 28 falls back to 011f951c without a first texture.
        let textures_b4 = e.mem.u32(property + 0xb4);
        e.mem.set_u32(textures_b4, 0);
        e.set_global(FALLBACK_011F951C, 0x0e00_951cu32);
        e.mem.set_u16(rendering + 4, 0x2d7);
        e.call(0x00b7_d200, &args![shader, block]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x0e00_951c);
        // Body 1: the default texture and the stage helper.
        e.set_global(DEFAULT_TEXTURE_011F952C, 0x0e00_952cu32);
        e.mem.set_u16(rendering + 4, 4);
        e.call(0x00b7_d200, &args![shader, block]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x0e00_952c);
        assert_eq!(
            calls(&e, STAGE_HELPER_00B71B10),
            vec![vec![shader, stages[0], property]]
        );
        // Types outside the table do nothing, not even the vtable call.
        let before = calls(&e, 0x0ed1_0108).len();
        for kind in [3u16, 0x2ec, 0x300] {
            e.mem.set_u16(rendering + 4, kind);
            e.call(0x00b7_d200, &args![shader, block]);
        }
        assert_eq!(calls(&e, 0x0ed1_0108).len(), before);
    }

    #[test]
    fn setup_geometry_constants_dispatches_and_sets_the_maps() {
        let mut e = rig(&[
            MODEL_MATRIX_00C4C2D0,
            PASS_SETUP_00B7AD00,
            PASS_SETUP_00B7CE90,
            AMBIENT_00C03010,
            AMBIENT_00C03180,
            RENDERER_METHOD_00E6FBB0,
            MATRIX_MULTIPLY_TRANSPOSE,
        ]);
        // Every constant entry off, so the set-ups stop early.
        for index in 0..22 {
            for entries in [VERTEX_ENTRIES_011FEBD8, PIXEL_ENTRIES_011FEC30] {
                let entry = e.global::<u32>(entries + 4 * index);
                e.mem.set_u8(entry + 8, 0);
            }
        }
        // The maps' SetShaderConstants and the shader's +0x1c object.
        e.put_vtable(0x0ee0_0000, &[0; 0x20]);
        e.mem.set_u32(0x0ee0_0000 + 0x78, 0x0ee1_0078);
        e.register(0x0ee1_0078, |_, _| Ret::default());
        e.mem.set_u32(0x0ee0_0000 + 0x24, 0x0ee1_0024);
        e.register(0x0ee1_0024, |_, _| Ret::default());
        let pixel_map = e.mem.alloc(4);
        let vertex_map = e.mem.alloc(4);
        let other = e.mem.alloc(4);
        for object in [pixel_map, vertex_map, other] {
            e.mem.set_u32(object, 0x0ee0_0000);
        }
        let shader = e.mem.alloc(0x38);
        e.mem.set_u32(shader + 0x30, pixel_map);
        e.mem.set_u32(shader + 0x34, vertex_map);
        e.mem.set_u32(shader + 0x1c, other);
        // The pass with only a vertex program, and a device for the model matrix upload.
        let program = e.mem.alloc(0x24);
        let pass = e.mem.alloc(0x60);
        e.mem.set_u32(pass + 0x5c, program);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        let device = e.mem.alloc(4);
        e.put_vtable(0x0ee3_0000, &[0; 0x5f]);
        e.mem.set_u32(0x0ee3_0000 + 0x178, 0x0ee4_0178);
        e.register(0x0ee4_0178, |_, _| Ret::default());
        e.mem.set_u32(device, 0x0ee3_0000);
        e.mem.set_u32(program + 0x20, device);
        let renderer = e.mem.alloc(0xa40);
        e.mem.set_u32(shader + 0x18, renderer);
        let data = e.mem.alloc(0x38);
        e.mem.set_u32(data + 0x34, 0x0ee5_0000);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry + 0xb8, data);
        let rendering = e.mem.alloc(0x10);
        e.mem.set_u16(rendering + 4, 0x10);
        e.mem.set_u32(rendering, geometry);
        e.set_global(RENDERING_PASS_011F91E0, rendering);
        let property = e.mem.alloc(0x70);
        let first = e.mem.alloc(0x1c);
        let second = e.mem.alloc(0x1c);
        e.mem.set_u8(second + 0x18, 1);
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block, first);
        e.mem.set_u32(block + 8, 0x77);
        e.mem.set_u32(block + 0xc, property);
        e.mem.set_u32(block + 0x10, second);
        e.set_global(RENDERING_PASS_TYPE_011F91E4, 0x10u32);
        e.call(0x00b7_dab0, &args![shader, block]);
        assert_eq!(
            calls(&e, MODEL_MATRIX_00C4C2D0)[0][..2],
            [geometry + 0x68, 0]
        );
        assert_eq!(calls(&e, AMBIENT_00C03010), vec![vec![0x77]]);
        assert_eq!(
            calls(&e, 0x0ee1_0078),
            vec![vec![
                vertex_map,
                program,
                geometry,
                0,
                0,
                0x0ee5_0000,
                block,
                0,
                geometry + 0x68,
                DEFAULT_POSITION_011F4288,
                0,
                1
            ]]
        );
        assert_eq!(calls(&e, 0x0ee1_0024), vec![vec![other, second]]);
        // Types 0x250 and 4 run their pass set-ups instead.
        e.set_global(RENDERING_PASS_TYPE_011F91E4, 0x250u32);
        e.call(0x00b7_dab0, &args![shader, block]);
        assert_eq!(
            calls(&e, PASS_SETUP_00B7CE90),
            vec![vec![shader, geometry, property, rendering]]
        );
        e.set_global(RENDERING_PASS_TYPE_011F91E4, 5u32);
        e.call(0x00b7_dab0, &args![shader, block]);
        assert_eq!(
            calls(&e, PASS_SETUP_00B7AD00),
            vec![vec![shader, geometry, property, rendering]]
        );
        assert_eq!(calls(&e, AMBIENT_00C03010).len(), 1);
    }
}

#[cfg(test)]
mod batch5_tests {
    use super::tests::{calls, rig};
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::MEMSET;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::{compiled, loader_rig};

    /// The constant maps the constructor double makes: vtable slots +8, +0x68 and +0x70 are
    /// doubles; +0x70 answers a new entry (count 0) for each name.
    fn map_rig() -> Engine {
        let mut e = rig(&[0x0ef1_0008, 0x0ef1_0068]);
        e.register(IMPORT_INTERLOCKED_INCREMENT_TEST, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT_TEST, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let slots: Vec<u32> = (0..0x20).map(|slot| 0x0ef1_0000 + 4 * slot).collect();
        e.put_vtable(0x0ef0_0000, &slots);
        e.register(0x0ef1_0070, |e, a| {
            let entry = e.mem.alloc(0xc);
            e.mem.set_u32(entry + 8, a[1]);
            entry.into_ret()
        });
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MAP_CONSTRUCT_00B7E330, |e, a| {
            e.mem.set_u32(a[0], 0x0ef0_0000);
            e.mem.set_u32(a[0] + 8, a[2]);
            a[0].into_ret()
        });
        e
    }

    const IMPORT_INTERLOCKED_INCREMENT_TEST: u32 = 0x00fd_f1c0;
    const IMPORT_INTERLOCKED_DECREMENT_TEST: u32 = 0x00fd_f1c4;

    #[test]
    fn create_constant_maps_builds_the_four_maps() {
        let mut e = map_rig();
        let this = e.mem.alloc(0x8c);
        e.mem.set_u32(this + 0x18, 0x0ef2_0000);
        e.call(0x00b7_e430, &args![this]);
        let vertex = e.mem.u32(this + 0x34);
        let pixel = e.mem.u32(this + 0x30);
        assert_eq!(e.mem.u32(vertex + 8), 0);
        assert_eq!(e.mem.u32(pixel + 8), 2);
        assert_eq!(e.mem.u32(this + 0x88), vertex);
        assert_eq!(e.mem.u32(this + 0x84), pixel);
        // Held by the shader twice.
        assert_eq!(e.mem.u32(vertex + 4), 2);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0x80) + 8), 0);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0x7c) + 8), 2);
        assert_eq!(
            calls(&e, MAP_CONSTRUCT_00B7E330)[0][1..].to_vec(),
            vec![0x0ef2_0000, 0]
        );
        let adds = calls(&e, 0x0ef1_0008);
        assert_eq!(adds.len(), 18 + 18 + 7 + 3);
        assert_eq!(calls(&e, 0x0ef1_0068).len(), 4 + 2);
        assert_eq!(
            adds[0],
            vec![
                vertex,
                0x010a_f66c,
                0x1000_0009,
                0,
                0x12,
                4,
                0x0101_1584,
                0x40,
                4,
                0x011f_d968,
                0
            ]
        );
        // The first vertex entry pointer is `WorldViewProjTranspose`'s entry.
        let first = e.global::<u32>(VERTEX_ENTRIES_011FEBD8);
        assert_eq!(e.mem.u32(first + 8), 0x010a_1e50);
        assert_eq!(e.mem.u32(first + 4), 1);
        assert_eq!(calls(&e, 0x0ef1_0070).len(), 22 + 18);
        // Existing maps are left alone.
        e.call_log = Some(vec![]);
        e.call(0x00b7_e430, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    /// The first word of each 0x4c-byte block the sources write.
    fn first_words(sources: &[ShaderSource]) -> std::collections::BTreeMap<u32, u32> {
        let mut words = std::collections::BTreeMap::new();
        for source in sources {
            for (index, word) in source.words.iter().enumerate() {
                let offset = source.offset + 4 * index as u32;
                words.insert(offset, *word);
            }
        }
        words
    }

    fn compiles_expected(sources: &[ShaderSource], first: u32, count: u32) -> usize {
        let words = first_words(sources);
        (0..count)
            .filter(|index| words.get(&(first + 0x4c * index)).copied().unwrap_or(0) != 0)
            .count()
    }

    #[test]
    fn load_vertex_shaders_compiles_the_three_tables() {
        let mut e = loader_rig();
        for page in [0x011f_9000u32, 0x011f_d000, 0x011f_e000] {
            e.map(page, 0x1000);
        }
        e.set_global(SHADER_MODEL_011F91C0, 1i32);
        let this = e.mem.alloc(0x8c);
        e.call(0x00b7_1bf0, &args![this]);
        let first = compiles_expected(VERTEX_SOURCES, 0x2534, 0x1f);
        let second = compiles_expected(VERTEX_SOURCES, 0x1eac, 0x16);
        let vertex_compiles = calls(&e, COMPILE_VERTEX_SHADER);
        assert_eq!(vertex_compiles.len(), first + second);
        assert!(vertex_compiles
            .iter()
            .all(|a| a[0] == this && a[3] == 0x010a_93e4));
        let shader = e.global::<u32>(0x011f_dd88);
        let seen = compiled(&e, shader);
        assert_eq!(seen.file, 0x010a_eda8);
        assert_eq!(seen.output, 0x010a_e8e8);
        assert!(!seen.pixel);
        // The formatter got the index as its fourth word (blocks 1 and 2 have no file).
        let formats = calls(&e, FORMAT_00406D00);
        assert_eq!(formats[1][3], 3);
        // From shader model 2 the third table runs; its block 0x39 (no file) clears its shader.
        let held = e.mem.alloc(8);
        e.mem.set_u32(held, 0x0ef3_0000);
        e.mem.set_u32(held + 4, 1);
        e.put_vtable(0x0ef3_0000, &[0, 0x0ef4_0004]);
        e.register(0x0ef4_0004, |_, _| Ret::default());
        e.set_global(0x011f_de5c + 4 * 0x39, held);
        e.set_global(SHADER_MODEL_011F91C0, 2i32);
        e.call_log = Some(vec![]);
        e.call(0x00b7_1bf0, &args![this]);
        let third = compiles_expected(VERTEX_SOURCES, 0x18, 0x67);
        assert_eq!(
            calls(&e, COMPILE_VERTEX_SHADER).len(),
            first + second + third
        );
        assert_eq!(e.global::<u32>(0x011f_de5c + 4 * 0x39), 0);
        assert_eq!(calls(&e, 0x0ef4_0004), vec![vec![held]]);
        assert_eq!(calls(&e, MEMSET).len(), VERTEX_SOURCES.len());
    }

    #[test]
    fn load_pixel_shaders_compiles_both_tables() {
        let mut e = loader_rig();
        for page in [
            0x011a_d000u32,
            0x011f_9000,
            0x011f_d000,
            0x010a_e000,
            0x0102_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(GCVT, |e, a| {
            e.mem.set_u32(a[3], 0x0047_4347);
            Ret::default()
        });
        e.register(SPRINTF, |e, a| {
            e.mem.set_u32(a[0], 0x0000_0033);
            Ret::default()
        });
        e.set_global(PIXEL_VALUE_011ADBCC, 1.5f32);
        e.set_global(SHADER_MODEL_011F91C0, 1i32);
        let this = e.mem.alloc(0x8c);
        e.call(0x00b7_4210, &args![this]);
        let first = compiles_expected(PIXEL_SOURCES, 0x2fa0, 0x30);
        assert_eq!(calls(&e, COMPILE_PIXEL_SHADER).len(), first);
        let gcvt = calls(&e, GCVT);
        let bits = 1.5f64.to_bits();
        assert_eq!(gcvt[0][..3], [bits as u32, (bits >> 32) as u32, 0xc]);
        // Shader 7 of the first table takes its third macro word from the first string.
        let shader = e.global::<u32>(0x011f_da48 + 4 * 7);
        let seen = compiled(&e, shader);
        assert_eq!(seen.macros[1], gcvt[0][3]);
        assert_eq!(seen.profile, 0x010a_93cc);
        assert!(seen.pixel);
        // Shader model 5, setting 4: the second table, its second names and the string words.
        e.set_global(SHADER_MODEL_011F91C0, 5i32);
        e.set_global(SETTING_011F91BC, 4u32);
        e.set_global(NUMBER_011F91B0, 3u32);
        e.call_log = Some(vec![]);
        e.call(0x00b7_4210, &args![this]);
        let second = compiles_expected(PIXEL_SOURCES, 0x20, 0xa0);
        assert_eq!(calls(&e, COMPILE_PIXEL_SHADER).len(), first + second);
        let sprintf = calls(&e, SPRINTF);
        assert_eq!(sprintf[0][1..].to_vec(), vec![NUMBER_FORMAT_01020774, 3]);
        let renamed = e.global::<u32>(PIXEL_SHADERS_011FDB08 + 4 * 0x46);
        if renamed != 0 {
            assert_eq!(compiled(&e, renamed).output, PIXEL_FORMAT_010AEDC4);
        }
        let plain = e.global::<u32>(PIXEL_SHADERS_011FDB08 + 4 * 0x45);
        if plain != 0 {
            assert_eq!(compiled(&e, plain).output, PIXEL_FORMAT_010AEDD8);
        }
        // Below shader model 5 the shaders past 0x99 are not compiled.
        e.set_global(SHADER_MODEL_011F91C0, 4i32);
        e.call_log = Some(vec![]);
        e.call(0x00b7_4210, &args![this]);
        let high = (0x9a..0xa0u32)
            .filter(|index| {
                first_words(PIXEL_SOURCES)
                    .get(&(0x20 + 0x4c * index))
                    .copied()
                    .unwrap_or(0)
                    != 0
            })
            .count();
        assert_eq!(calls(&e, COMPILE_PIXEL_SHADER).len(), first + second - high);
    }
}

/// Test objects shared by the n071 tests of the `BSShader` units: objects whose vtable slots
/// are doubles answering fixed words, and the logged calls to an address.
#[cfg(test)]
pub(crate) mod n071_rig {
    use crate::prelude::*;

    /// An object of `size` bytes whose vtable (0x100 slots) has a double at every slot: the
    /// slots listed answer their word, the others 0. The double of slot `s` sits at
    /// [`slot_target`]`(object, s)`.
    pub(crate) fn vobject(e: &mut Engine, size: u32, answers: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for slot in (0..0x400).step_by(4) {
            let target = 0x4000_0000 + vtable + slot;
            e.mem.set_u32(vtable + slot, target);
            let answer = answers
                .iter()
                .find(|(s, _)| *s == slot)
                .map_or(0, |(_, word)| *word);
            e.register_double(target, move |_, _| answer.into_ret());
        }
        let object = e.mem.alloc(size.max(4));
        e.mem.set_u32(object, vtable);
        object
    }

    /// The address of the double at vtable slot `slot` of `object`.
    pub(crate) fn slot_target(e: &Engine, object: u32, slot: u32) -> u32 {
        let vtable = e.mem.u32(object);
        e.mem.u32(vtable + slot)
    }

    /// The argument words of every logged call to `addr`, in order.
    pub(crate) fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The addresses of every logged call, in order.
    pub(crate) fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    /// An engine with the call log on and the given addresses doubled (answering 0).
    pub(crate) fn engine(doubles: &[u32]) -> Engine {
        let mut e = Engine::new();
        for target in doubles {
            e.register(*target, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }
}

#[cfg(test)]
mod n071_tests {
    use super::n071_rig::{calls, order, slot_target, vobject};
    use super::tests::{pass_with_stages, rig};
    use super::*;

    #[test]
    fn fn_00b71b10_sets_the_mode_only_with_both() {
        let mut e = rig(&[STAGE_MODE_00E7DE90]);
        let property = vobject(&mut e, 0x10, &[(0xe0, 9)]);
        e.call(0x00b7_1b10, &args![0u32, 0u32, property]);
        e.call(0x00b7_1b10, &args![0u32, 0x500u32, 0u32]);
        assert!(calls(&e, STAGE_MODE_00E7DE90).is_empty());
        e.call(0x00b7_1b10, &args![0u32, 0x500u32, property]);
        assert_eq!(calls(&e, STAGE_MODE_00E7DE90), vec![vec![0x500, 9]]);
    }

    #[test]
    fn fn_00b78980_runs_the_creators_by_shader_model() {
        let mut doubles = PASS_CREATORS_ALWAYS.to_vec();
        doubles.extend(PASS_CREATORS_MODEL_2);
        doubles.push(PASS_CREATOR_LOW_MODEL_00BEA040);
        let mut e = rig(&doubles);
        let shader = vobject(&mut e, 0x10, &[]);
        e.set_global(SHADER_MODEL_011F91C0, 1i32);
        e.call_log = Some(vec![]);
        e.call(0x00b7_8980, &args![shader]);
        let mut expected = vec![0x00b7_8980, slot_target(&e, shader, 0x144)];
        expected.extend(PASS_CREATORS_ALWAYS);
        expected.push(PASS_CREATOR_LOW_MODEL_00BEA040);
        assert_eq!(order(&e), expected);
        assert_eq!(calls(&e, 0x00be_b070), vec![Vec::<u32>::new()]);
        e.call_log = Some(vec![]);
        e.set_global(SHADER_MODEL_011F91C0, 2i32);
        e.call(0x00b7_8980, &args![shader]);
        let mut expected = vec![0x00b7_8980, slot_target(&e, shader, 0x144)];
        expected.extend(PASS_CREATORS_ALWAYS);
        expected.extend(PASS_CREATORS_MODEL_2);
        assert_eq!(order(&e), expected);
    }

    #[test]
    fn fn_00b78a40_releases_every_pass_of_the_table() {
        let mut e = rig(&[]);
        let shader = vobject(&mut e, 0x10, &[]);
        e.set_global(PASS_TABLE_011FDFF8, 0x11u32);
        e.set_global(PASS_TABLE_END_011FEBD4 - 4, 0x22u32);
        e.call(0x00b7_8a40, &args![shader]);
        let seen = calls(&e, slot_target(&e, shader, 0x128));
        assert_eq!(seen.len(), 0x2f7);
        assert_eq!(seen[0], vec![shader, 0x11]);
        assert_eq!(seen[0x2f6], vec![shader, 0x22]);
    }

    #[test]
    fn fn_00b78a70_stores_the_reciprocal() {
        let mut e = rig(&[]);
        e.map(0x0120_2000, 0x1000);
        e.set_global(DIVISOR_012023E0, 4.0f32);
        e.set_global(ZERO_011FD958, 7.0f32);
        e.call(0x00b7_8a70, &args![]);
        assert_eq!(e.global::<f32>(RECIPROCAL_011FD954), 0.25);
        assert_eq!(e.global::<f32>(ZERO_011FD958), 0.0);
    }

    /// A rendering object of pass type `kind` and the stages of the pass the table gives for
    /// it (the current pass is left as it was).
    fn rendering_with_pass(e: &mut Engine, kind: u16) -> (u32, Vec<u32>) {
        let previous = e.global::<u32>(CURRENT_PASS_0126F74C);
        let stages = pass_with_stages(e, 2);
        let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
        e.set_global(CURRENT_PASS_0126F74C, previous);
        e.set_global(PASS_TABLE_011FDFF8 + 4 * u32::from(kind), pass);
        let rendering = e.mem.alloc(0x10);
        e.mem.set_u16(rendering + 4, kind);
        (rendering, stages)
    }

    #[test]
    fn fn_00b7ad00_sets_up_the_first_stage() {
        let mut e = rig(&[
            STAGE_MODE_00E7DE90,
            RENDER_STATE_00B97ED0,
            RENDER_STATE_00B97F20,
        ]);
        let shader = vobject(&mut e, 0x10, &[]);
        let property = vobject(&mut e, 0xe0, &[(0xe0, 9)]);
        let (rendering, stages) = rendering_with_pass(&mut e, 4);
        let pass = e.global::<u32>(PASS_TABLE_011FDFF8 + 16);
        // No block, or no rendering object: nothing happens.
        e.call(0x00b7_ad00, &args![shader, 0u32, property, rendering]);
        e.call(0x00b7_ad00, &args![shader, 0u32, property, 0u32]);
        assert!(calls(&e, STAGE_MODE_00E7DE90).is_empty());
        let block = e.mem.alloc(0x80);
        e.mem.set_u32(property + 0xdc, block);
        e.mem.set_u32(block + 0x70, 0x77);
        e.set_global(DEFAULT_TEXTURE_011F952C, 0x952cu32);
        e.call(0x00b7_ad00, &args![shader, 0u32, property, rendering]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x952c);
        assert_eq!(calls(&e, STAGE_MODE_00E7DE90), vec![vec![stages[0], 9]]);
        assert_eq!(calls(&e, RENDER_STATE_00B97ED0), vec![vec![1, 0]]);
        assert_eq!(calls(&e, RENDER_STATE_00B97F20), vec![vec![4, 0x77, 0]]);
        assert_eq!(calls(&e, slot_target(&e, shader, 0x108)).len(), 1);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), pass);
        e.mem.set_u32(block + 0x6c, 0x6c6c);
        e.call(0x00b7_ad00, &args![shader, 0u32, property, rendering]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x6c6c);
    }

    #[test]
    fn fn_00b7ad80_copies_the_block_and_the_fog() {
        let mut e = rig(&[GET_FOG_PROPERTY]);
        let property = e.mem.alloc(0xe0);
        // No property: nothing is written.
        e.set_global(FOG_FAR_011FD934, 5.0f32);
        e.call(0x00b7_ad80, &args![0u32, 1u32, 0u32]);
        assert_eq!(e.global::<f32>(FOG_FAR_011FD934), 5.0);
        let block = e.mem.alloc(0x60);
        for index in 0..8 {
            e.mem.set_u32(block + 0xc + 4 * index, 0x100 + index);
        }
        e.mem.set_u32(block + 0x4c, 0x7fa0_0000);
        e.mem.set_f32(block + 0x50, 2.0);
        e.mem.set_f32(block + 0x54, 3.0);
        e.mem.set_u32(property + 0xdc, block);
        e.call(0x00b7_ad80, &args![0u32, 0u32, property]);
        assert_eq!(e.global::<u32>(BLOCK_WORDS_011FD8F4 + 0x1c), 0x107);
        assert_eq!(e.global::<u32>(BLOCK_VALUE_011FD85C), 0x7fe0_0000);
        assert_eq!(e.global::<f32>(BLOCK_VALUE_011FD860), 2.0);
        assert_eq!(e.global::<f32>(BLOCK_VALUE_011FD914), 3.0);
        assert_eq!(e.global::<f32>(ONE_011FD918), 1.0);
        assert_eq!(e.global::<f32>(FOG_FAR_011FD934), 0.0);
        assert!(calls(&e, GET_FOG_PROPERTY).is_empty());
        // A fog with both distances 0.0 keeps the cleared values.
        let fog = e.mem.alloc(0x70);
        e.register_double(GET_FOG_PROPERTY, move |_, _| fog.into_ret());
        e.set_global(FOG_INDEX_011F91C4, 3u8);
        e.call(0x00b7_ad80, &args![0u32, 1u32, property]);
        assert_eq!(calls(&e, GET_FOG_PROPERTY), vec![vec![3]]);
        assert_eq!(e.global::<f32>(FOG_FAR_011FD934), 0.0);
        // Near 100, far 600: the colour, far, range and power.
        e.mem.set_f32(fog + 0x20, 0.25);
        e.mem.set_f32(fog + 0x24, 0.5);
        e.mem.set_f32(fog + 0x28, 0.75);
        e.mem.set_f32(fog + 0x2c, 100.0);
        e.mem.set_f32(fog + 0x30, 600.0);
        e.mem.set_f32(fog + 0x60, 4.0);
        e.call(0x00b7_ad80, &args![0u32, 1u32, property]);
        let fogs: Vec<f32> = (0..8)
            .map(|index| e.global::<f32>(FOG_COLOUR_011FD924 + 4 * index))
            .collect();
        assert_eq!(fogs, vec![0.25, 0.5, 0.75, 0.0, 600.0, 500.0, 4.0, 0.0]);
        // Far 0.0 but a NaN near distance still writes.
        e.mem.set_f32(fog + 0x30, 0.0);
        e.mem.set_u32(fog + 0x2c, 0x7fc0_0000);
        e.call(0x00b7_ad80, &args![0u32, 1u32, property]);
        assert_eq!(e.global::<u32>(FOG_RANGE_011FD938), 0x7fc0_0000);
    }

    #[test]
    fn fn_00b7c150_and_00b7c170_set_the_pair_textures() {
        let mut e = rig(&[]);
        let stages = pass_with_stages(&mut e, 3);
        e.set_global(PAIR_TEXTURE_011F95E4, 0x95e4u32);
        e.set_global(PAIR_TEXTURE_011F95E8, 0x95e8u32);
        e.call(0x00b7_c150, &args![0u32, 2u32]);
        e.call(0x00b7_c170, &args![0u32, 1u32]);
        assert_eq!(e.mem.u32(stages[2] + 8), 0x95e4);
        assert_eq!(e.mem.u32(stages[1] + 8), 0x95e8);
        assert_eq!(e.mem.u32(stages[0] + 8), 0);
    }

    #[test]
    fn fn_00b7ce90_sets_up_both_stages_and_the_states() {
        let mut e = rig(&[STAGE_MODE_00E7DE90, GET_FOG_PROPERTY]);
        let shader = vobject(&mut e, 0x10, &[]);
        let property = vobject(&mut e, 0xe0, &[(0xe0, 9)]);
        let device = vobject(&mut e, 0x10, &[]);
        e.register_double(PASS_OBJECT_00E7F7C0, move |_, _| device.into_ret());
        e.register(TEXTURE_00C03230, |_, _| 0x3230u32.into_ret());
        let (rendering, stages) = rendering_with_pass(&mut e, 0x250);
        let pass = e.global::<u32>(PASS_TABLE_011FDFF8 + 4 * 0x250);
        e.call(0x00b7_ce90, &args![shader, 1u32, property, rendering]);
        assert!(calls(&e, TEXTURE_00C03230).is_empty());
        let block = e.mem.alloc(0x80);
        for offset in [0x5cu32, 0x60, 0x64, 0x68] {
            e.mem.set_u32(block + offset, offset);
        }
        e.mem.set_u32(property + 0xdc, block);
        e.set_global(DEFAULT_TEXTURE_011F952C, 0x952cu32);
        e.set_global(CURRENT_PASS_0126F74C, 0x0bad_0000u32);
        e.call(0x00b7_ce90, &args![shader, 0u32, property, rendering]);
        assert_eq!(calls(&e, TEXTURE_00C03230), vec![vec![property, 0]]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x3230);
        assert_eq!(e.mem.u32(stages[1] + 8), 0x952c);
        assert_eq!(
            calls(&e, STAGE_MODE_00E7DE90),
            vec![vec![stages[0], 9], vec![stages[1], 9]]
        );
        assert_eq!(calls(&e, PASS_OBJECT_00E7F7C0), vec![vec![0x0bad_0000]; 3]);
        assert_eq!(
            calls(&e, slot_target(&e, device, 0x5c)),
            vec![vec![device, 0x5c, 0x60, 0, 0]]
        );
        assert_eq!(
            calls(&e, slot_target(&e, device, 0x68)),
            vec![
                vec![device, 0xab, 0x64, 0, 0],
                vec![device, 0x17, 0x68, 0, 0]
            ]
        );
        // The first word 0 skips the fog in 00b7ad80.
        assert!(calls(&e, GET_FOG_PROPERTY).is_empty());
        assert_eq!(calls(&e, slot_target(&e, shader, 0x108)).len(), 1);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), pass);
        e.mem.set_u32(block + 8, 0x88);
        e.call(0x00b7_ce90, &args![shader, 0u32, property, rendering]);
        assert_eq!(e.mem.u32(stages[1] + 8), 0x88);
    }
}
