//! `BSShader/imagespaceeffectwaterfft.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays), the `NiTArray` set-size
/// `00a90180` (`thiscall` on the array at +0x28, one word) and this effect's vtable
/// (`010b7f10`; no RTTI locator before it).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const VTABLE_010B7F10: u32 = 0x010b_7f10;
/// The `float` constants it copies (values read in Ghidra: 5.0, 90.0, 0.5).
const CONSTANT_0101712C: u32 = 0x0101_712c;
const CONSTANT_010249D8: u32 = 0x0102_49d8;
const HALF_01016248: u32 = 0x0101_6248;
/// The effect's globals: the `float`s `0120005c` to `01200068`, the effect itself
/// (`0120006c`), the byte that picks the grid (`01200059`), the grid size (`01200054`) and its
/// base-2 logarithm (`01200050`), the byte `01200058` and the `float` reciprocal of the size
/// (`0120004c`).
const PARAMETER_0120005C: u32 = 0x0120_005c;
const PARAMETER_01200060: u32 = 0x0120_0060;
const PARAMETER_01200064: u32 = 0x0120_0064;
const PARAMETER_01200068: u32 = 0x0120_0068;
const EFFECT_0120006C: u32 = 0x0120_006c;
const LARGE_GRID_01200059: u32 = 0x0120_0059;
const GRID_SIZE_01200054: u32 = 0x0120_0054;
const GRID_SHIFT_01200050: u32 = 0x0120_0050;
const FLAG_01200058: u32 = 0x0120_0058;
const INVERSE_SIZE_0120004C: u32 = 0x0120_004c;

// Translated from 00ba15e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`): the water FFT image-space
/// effect's constructor, `thiscall`, no stack words. Runs the base constructor `00ba4080`,
/// stores the vtable `010b7f10`, clears six 16-byte blocks at +0x58 to +0x98 (three bytes,
/// then three words each), sizes the array at +0x28 to 10 (`00a90180`), copies the `float`s at
/// `0101712c`, `010249d8` and `01016248` to `01200068`, `01200064` and `01200060` (through the
/// FPU, a signalling NaN quieted) and 1.0 to `0120005c`, keeps the effect in `0120006c`, stores
/// 0.0 at +0xb0 and +0xb4, sets the grid size and its logarithm to 0x100 and 8 when the byte
/// `01200059` is set, else 0x80 and 7, sets the byte `01200058` and stores `1.0 / size`
/// (`FILD`, `FDIVRP`) at `0120004c`. Answers `this`.
// Translation incomplete: 00ba15e0 - the C++ exception frame (FS:[0] chain, unwind state 5 for the base and the blocks) is not translated.
pub fn fn_00ba15e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B7F10);
    for block in (0x58u32..=0x98).step_by(0x10) {
        for byte in 0..3u32 {
            e.mem.set_u8(base + block + byte, 0);
        }
        for word in [4u32, 8, 0xc] {
            e.mem.set_u32(base + block + word, 0);
        }
    }
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 10u32]);
    let five = x87_quiet_bits(e.global::<u32>(CONSTANT_0101712C));
    e.set_global(PARAMETER_01200068, five);
    e.set_global(EFFECT_0120006C, base);
    let ninety = x87_quiet_bits(e.global::<u32>(CONSTANT_010249D8));
    e.set_global(PARAMETER_01200064, ninety);
    let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
    e.set_global(PARAMETER_01200060, half);
    e.set_global(PARAMETER_0120005C, 0x3f80_0000u32);
    e.mem.set_u32(base + 0xb0, 0);
    e.mem.set_u32(base + 0xb4, 0);
    if e.global::<u8>(LARGE_GRID_01200059) != 0 {
        e.set_global(GRID_SIZE_01200054, 0x100u32);
        e.set_global(GRID_SHIFT_01200050, 8u32);
    } else {
        e.set_global(GRID_SIZE_01200054, 0x80u32);
        e.set_global(GRID_SHIFT_01200050, 7u32);
    }
    let size = f64::from(e.global::<i32>(GRID_SIZE_01200054));
    e.set_global(FLAG_01200058, 1u8);
    let inverse = x87_store32(x87_div(1.0, size));
    e.set_global(INVERSE_SIZE_0120004C, inverse);
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// `ImageSpaceTexture::GetTexture` (`00ba3780`), `::ReturnRenderedTexture` (`00ba39a0`) and
/// the destructor (`00ba3a00`) (unit `imagespacetexture.cpp`, `thiscall`).
const TEXTURE_GET_TEXTURE_00BA3780: u32 = 0x00ba_3780;
const TEXTURE_RETURN_RENDERED_00BA39A0: u32 = 0x00ba_39a0;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;

// Translated from 00ba1250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectWaterFFT::FreeUpTextureMemory` (Xbox PDB), `thiscall`, no stack words: each
/// of the effect's textures at +0x78, +0x88, +0x58, +0x68 and +0x98 (in that order) that has a
/// texture is returned to the texture manager (`ReturnRenderedTexture`); then the byte +0xb8
/// is cleared.
pub fn image_space_effect_water_fft_free_up_texture_memory(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [0x78u32, 0x88, 0x58, 0x68, 0x98] {
        let holder = base + offset;
        if e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![holder]).u32() != 0 {
            e.call(TEXTURE_RETURN_RENDERED_00BA39A0, &args![holder]);
        }
    }
    e.mem.set_u8(base + 0xb8, 0);
}

/// The array setter `00ba3aa0` (`thiscall` on an `NiTPrimitiveArray`: index, the address of
/// the word), `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`) and
/// `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the manager, one word).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;
const MEMORY_DEALLOCATE_00AA4060: u32 = 0x00aa_4060;

/// Element `index` of the `NiTPrimitiveArray` at `array` set to `value` through `00ba3aa0`
/// (the word in a local).
fn set_at(e: &mut Engine, array: u32, index: u32, value: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        e.call(ARRAY_SET_AT_00BA3AA0, &args![array, index, local.addr()]);
    });
}

// Translated from 00ba12d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba12d0 - the C++ exception frame (FS:[0] chain, unwind states 5 to 0 for the textures and the base) is not translated.
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`): the water FFT effect's
/// destructor, `thiscall`, no stack words. Stores the vtable `010b7f10`; each of the 12 effect
/// parameters (+0x1c elements) that is set is deleted through its slot +0 with 1 (the element
/// tested twice) and its element set to null; the textures at +0x98, +0x88, +0x78, +0x68 and
/// +0x58 are destroyed; then `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00ba12d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B7F10);
    for index in 0..0xcu32 {
        let element = e.mem.u32(base + 0x1c).wrapping_add(4 * index);
        if e.mem.u32(element) != 0 {
            let param = e.mem.u32(element);
            if param != 0 {
                e.vcall(param, 0, &args![1u32]);
            }
            set_at(e, base + 0x18, index, 0);
        }
    }
    for offset in [0x98u32, 0x88, 0x78, 0x68, 0x58] {
        e.call(TEXTURE_DESTRUCT_00BA3A00, &args![base + offset]);
    }
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00ba1750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`): the water FFT effect's
/// scalar deleting destructor, `thiscall`, one word: `00ba12d0`, then the memory freed when
/// bit 0 of `flags` is set. Answers `this`.
pub fn fn_00ba1750(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00ba12d0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The effect-array sizer `00ba3f90` (unnamed; `thiscall`, one word),
/// `ImageSpaceShaderParam`'s constructor (`00b8a9e0`) and `InitConstantMap` (`00b8ab10`, two
/// words), and the effect setter `00ba3c70` (unit `run_00ba3aa0`).
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;

// Translated from 00ba1770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba1770 - the C++ exception frame (FS:[0] chain, unwind states 0 to 6 for the parameter allocations) is not translated.
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`): the water FFT effect's
/// `Setup` slot, `thiscall`, two words (the manager; a word it does not read). Sizes the
/// arrays to 12 (`00ba3f90`) and `pTextures` to 10; then seven parameters (0x88 bytes each
/// from the memory manager; the constructor only on a block) with `InitConstantMap(2, 3)` and
/// the byte +0x55 cleared, parameter `i` stored at `i` and given with the manager's effect at
/// +0x174 + 4 `i` to effect `i` (output 0).
pub fn fn_00ba1770(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 0xcu32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 0xau32]);
    for index in 0..7u32 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
            .u32();
        let param = if block != 0 {
            e.call(PARAM_CONSTRUCT_00B8A9E0, &args![block]).u32()
        } else {
            0
        };
        e.call(PARAM_INIT_CONSTANT_MAP_00B8AB10, &args![param, 2u32, 3u32]);
        e.mem.set_u8(param + 0x55, 0);
        set_at(e, base + 0x18, index, param);
        let effect = e.mem.u32(e.mem.u32(manager + 8) + 0x174 + 4 * index);
        e.call(
            SET_EFFECT_00BA3C70,
            &args![base, index, effect, param, 0u32],
        );
    }
}

/// The `float` `011f940c` (a speed) and the `double` 25.0 (`0104f2f0`) `00ba13b0` reads.
const SPEED_011F940C: u32 = 0x011f_940c;
const DIVISOR_0104F2F0: u32 = 0x0104_f2f0;

// Translated from 00ba13b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 7 of the vtable `010b7f10`, `thiscall`, one
/// word it does not read: +0xb0 becomes `[011f940c] / 25.0 * [0120005c]` (x87, the `double`
/// at `0104f2f0`). Answers true.
pub fn fn_00ba13b0(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    let speed = x87_load32(e.global::<u32>(SPEED_011F940C));
    let divisor = f64::from_bits(e.global::<u64>(DIVISOR_0104F2F0));
    let scale = x87_m32(e.global::<u32>(PARAMETER_0120005C));
    let value = x87_mul(x87_div(speed, divisor), scale);
    e.mem.set_u32(this.addr() + 0xb0, x87_store32(value));
    true
}

/// The CRT `rand` (`00ecadb8`, `cdecl`), the `double` 32767.0 (`01029790`) and the table of
/// `float` pairs (value, threshold) `00ba13d0` searches: values at `011ada18 + 8 k`,
/// thresholds at `011ada24 + 8 k`.
const RAND_00ECADB8: u32 = 0x00ec_adb8;
const RAND_MAX_01029790: u32 = 0x0102_9790;
const TABLE_VALUES_011ADA18: u32 = 0x011a_da18;
const TABLE_THRESHOLDS_011ADA24: u32 = 0x011a_da24;

/// The first `k` whose threshold is not below `r` (`FCOMP`; a NaN stops the search).
fn table_index(e: &Engine, r: f64) -> u32 {
    let threshold = |k: u32| x87_load32(e.global::<u32>(TABLE_THRESHOLDS_011ADA24 + 8 * k));
    let mut k = 0u32;
    if threshold(0) < r {
        loop {
            k += 1;
            if !(threshold(k) < r) {
                break;
            }
        }
    }
    k
}

// Translated from 00ba13d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`), `cdecl`, two `float`s: a
/// random number from a table. `r = rand() / 32767.0` (stored as a `float`); at or above 0.5
/// the value of the first table pair whose threshold is not below `r`, below 0.5 (or NaN) one
/// minus the value found for `1 - r`. Answers `value * scale + offset` (x87, through a `float`).
pub fn fn_00ba13d0(e: &mut Engine, offset: f32, scale: f32) -> f32 {
    let random = e.call(RAND_00ECADB8, &args![]).i32();
    let divisor = f64::from_bits(e.global::<u64>(RAND_MAX_01029790));
    let r_bits = x87_store32(x87_div(f64::from(random), divisor));
    let half = x87_load32(e.global::<u32>(HALF_01016248));
    let r = x87_load32(r_bits);
    let value = if !(r >= half) {
        let q_bits = x87_store32(x87_sub(1.0, r));
        let k = table_index(e, x87_load32(q_bits));
        x87_sub(1.0, x87_m32(e.global::<u32>(TABLE_VALUES_011ADA18 + 8 * k)))
    } else {
        let k = table_index(e, r);
        x87_load32(e.global::<u32>(TABLE_VALUES_011ADA18 + 8 * k))
    };
    let value_bits = x87_store32(value);
    let result = x87_add(
        x87_mul(x87_load32(value_bits), x87_m32(scale.to_bits())),
        x87_m32(offset.to_bits()),
    );
    f32::from_bits(x87_quiet_bits(x87_store32(result)))
}

/// What `00ba1480` reads: the `double`s pi (`0101ff40`), 180.0 (`0104ed58`), 100000.0
/// (`0108c160`), 9.81 (`010b7f30`), 75.0 (`010731a8`), -1.0 (`0101a6b0`), 0.0 (`01012060`)
/// and 0.25 (`010290b0`), and the CRT's `cos` (`00ec9f70`), `sin` (`00eca0a0`) and `exp`
/// (`00ecae6c`) (argument and result in ST0; the names by their use here, unconfirmed).
const PI_0101FF40: u32 = 0x0101_ff40;
const DEGREES_0104ED58: u32 = 0x0104_ed58;
const AMPLITUDE_DIVISOR_0108C160: u32 = 0x0108_c160;
const GRAVITY_010B7F30: u32 = 0x010b_7f30;
const DIVISOR_010731A8: u32 = 0x0107_31a8;
const MINUS_ONE_0101A6B0: u32 = 0x0101_a6b0;
const ZERO_01012060: u32 = 0x0101_2060;
const QUARTER_010290B0: u32 = 0x0102_90b0;
const COS_00EC9F70: u32 = 0x00ec_9f70;
const SIN_00ECA0A0: u32 = 0x00ec_a0a0;
const EXP_00ECAE6C: u32 = 0x00ec_ae6c;

/// A `double` constant of the exe as an x87 memory operand.
fn double_at(e: &Engine, addr: u32) -> f64 {
    f64::from_bits(e.global::<u64>(addr))
}

// Translated from 00ba1480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`), `thiscall` (`this` not
/// read), two `float`s (a wave vector x, y): the Phillips spectrum of the water. With the
/// direction angle `[01200064] * pi / 180`, A = `[01200060] / 100000`, L = `[01200068]^2 /
/// 9.81` and l = L / 75, k2 = x^2 + y^2, d = cos(angle) x + sin(angle) y clamped to 0 when
/// below 0: answers `exp(-k2 l^2) * (A d^2 exp(-1 / (k2 L^2)) / k2^3)`, the second factor
/// quartered when d is below 0 (it never is after the clamp; the test is kept). Every x87
/// step in the exe's order, with its stores to `float` and `double`.
pub fn fn_00ba1480(e: &mut Engine, _this: Ptr, x: f32, y: f32) -> f32 {
    let ld = x87_load32;
    let angle_bits = x87_store32(x87_div(
        x87_mul(
            ld(e.global::<u32>(PARAMETER_01200064)),
            double_at(e, PI_0101FF40),
        ),
        double_at(e, DEGREES_0104ED58),
    ));
    let amplitude_bits = x87_store32(x87_div(
        ld(e.global::<u32>(PARAMETER_01200060)),
        double_at(e, AMPLITUDE_DIVISOR_0108C160),
    ));
    let wind = ld(e.global::<u32>(PARAMETER_01200068));
    let length_bits = x87_store32(x87_div(x87_mul(wind, wind), double_at(e, GRAVITY_010B7F30)));
    let small_bits = x87_store32(x87_div(ld(length_bits), double_at(e, DIVISOR_010731A8)));
    let a = ld(x.to_bits());
    let b = ld(y.to_bits());
    let bb = x87_mul(b, b);
    let aa = x87_mul(a, a);
    let k2_bits = x87_store32(x87_add(aa, bb));
    let cosine = e.call(COS_00EC9F70, &args![ld(angle_bits)]).f64();
    let cosine_bits = x87_store32(cosine);
    let along_x = x87_mul(ld(cosine_bits), x87_m32(x.to_bits()));
    let sine = e.call(SIN_00ECA0A0, &args![ld(angle_bits)]).f64();
    let sine_bits = x87_store32(sine);
    let dot = x87_add(x87_mul(ld(sine_bits), x87_m32(y.to_bits())), along_x);
    let mut dot_bits = x87_store32(dot);
    if ld(dot_bits) < double_at(e, ZERO_01012060) {
        dot_bits = x87_store32(0.0);
    }
    let length = ld(length_bits);
    let dot = ld(dot_bits);
    let k2 = ld(k2_bits);
    let product = x87_mul(x87_mul(length, length), k2);
    let exponent_bits = x87_store32(x87_div(double_at(e, MINUS_ONE_0101A6B0), product));
    let falloff = e.call(EXP_00ECAE6C, &args![ld(exponent_bits)]).f64();
    let falloff_bits = x87_store32(falloff);
    let spectrum = x87_mul(
        x87_mul(ld(falloff_bits), x87_mul(dot, dot)),
        x87_m32(amplitude_bits),
    );
    let cube = x87_mul(x87_mul(k2, k2), ld(k2_bits));
    let mut spectrum_bits = x87_store32(x87_div(spectrum, cube));
    let small = ld(small_bits);
    let damping_bits = x87_store32(x87_mul(x87_mul(-ld(k2_bits), small), small));
    let damping = e.call(EXP_00ECAE6C, &args![ld(damping_bits)]).f64();
    let damping_bits = x87_quiet_bits(x87_store32(damping));
    // `FLDZ`, `FCOMP` with the clamped dot product: below 0 it never is any more.
    if 0.0 > x87_m32(dot_bits) {
        spectrum_bits = x87_store32(x87_mul(ld(spectrum_bits), double_at(e, QUARTER_010290B0)));
    }
    let result = x87_mul(ld(damping_bits), x87_m32(spectrum_bits));
    f32::from_bits(x87_quiet_bits(x87_store32(result)))
}

/// The memory manager's `Deallocate` (`00aa4060`), `memset` (`00ec61c0`), `memcpy`
/// (`00ec44d0`) and `_ftol2_sse` (`00ec62c0`, the value in ST0), the `double`s 0.5
/// (`01011588`), 65535.0 (`010217d0`), -2 pi (`010b7f38`, a `float`'s precision) and -2.0
/// (`0106d4c8`), and the thread block's allocation context word (+0x2b4).
const MEMSET_00EC61C0: u32 = 0x00ec_61c0;
const MEMCPY_00EC44D0: u32 = 0x00ec_44d0;
const FTOL_00EC62C0: u32 = 0x00ec_62c0;
const HALF_DOUBLE_01011588: u32 = 0x0101_1588;
const WORD_SCALE_010217D0: u32 = 0x0102_17d0;
const MINUS_TWO_PI_010B7F38: u32 = 0x010b_7f38;
const MINUS_TWO_0106D4C8: u32 = 0x0106_d4c8;
const TLS_ALLOCATION_CONTEXT: u32 = 0x2b4;

/// A `count * size` byte request with the compiler's overflow check (all ones on overflow).
fn array_bytes(count: u32, size: u32) -> u32 {
    count.checked_mul(size).unwrap_or(u32::MAX)
}

/// The memory manager's block of `size` bytes.
fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32()
}

/// The Direct3D texture behind the image-space texture at `holder`: `GetTexture`, its +0x24,
/// that object's slot +0x9c.
fn d3d_texture(e: &mut Engine, holder: u32) -> u32 {
    let texture = e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![holder]).u32();
    let renderer_data = e.mem.u32(texture + 0x24);
    e.vcall(renderer_data, 0x9c, &args![]).u32()
}

/// `LockRect(0, &locked, NULL, 0)` (`IDirect3DTexture9` slot +0x4c), `body` with the bits
/// pointer (the locked rectangle's +4), then `UnlockRect(0)` (slot +0x50).
fn with_locked(e: &mut Engine, texture: u32, body: impl FnOnce(&mut Engine, u32)) {
    e.with_stack(8, |e, locked| {
        e.vcall(texture, 0x4c, &args![0u32, locked, 0u32, 0u32]);
        let bits = e.mem.u32(locked.addr() + 4);
        body(e, bits);
    });
    e.vcall(texture, 0x50, &args![0u32]);
}

// Translated from 00ba1ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba1ac0 - the C++ exception frame (FS:[0] chain, unwind state 0) and the security-cookie bookkeeping are not translated.
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`), `thiscall`, two words (the
/// grid size `n` and the number of passes): fills the FFT's utility textures, with the thread's
/// allocation context set to 0x1d. The texture of +0x88 gets the bit-reversal table: entry `i`
/// is `(i + 0.5) / n`, the entries permuted by bit reversal (swapped through the FPU), each
/// times 65535 and truncated (`_ftol2`) to a 16-bit word (`n` words copied in). The texture of
/// +0x78 gets the butterfly table: `n` entries of 16 bytes per pass (a buffer of `n * 0x100`
/// bytes); in pass `p` (span `2^(p+1)`, step `2^p`) for each `m` below the step the twiddle
/// `w = (wr, wi)` starts at (1, 0) and turns by the angle `-2 pi / -span` (the recurrence
/// `wr += wr alpha - wi beta`, `wi += wi alpha + beta wr` with `alpha = -2 sin^2(theta/2)`,
/// `beta = sin(theta)`); entries `k` and `k + step` (`k` from `m` by the span) get
/// `((k + step + 0.5) / n, (k + 0.5) / n, wr, wi)` and `(-(k + step + 0.5) / n, (k + 0.5) / n,
/// wr, wi)`. Every x87 step in the exe's order.
pub fn fn_00ba1ac0(e: &mut Engine, this: Ptr, size: u32, passes: u32) {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 0x1d);
    let n = size as i32;
    let positions = allocate(e, array_bytes(size, 4));
    let words = allocate(e, array_bytes(size, 2));
    e.call(
        MEMSET_00EC61C0,
        &args![positions, 0u32, size.wrapping_mul(4)],
    );
    let texture = d3d_texture(e, this.addr() + 0x88);
    let half = x87_load64(e.global::<u64>(HALF_DOUBLE_01011588));
    if n > 0 {
        let count_bits = x87_store32(f64::from(n));
        let count = x87_load32(count_bits);
        for index in 0..n {
            let shifted_bits = x87_store32(x87_add(f64::from(index), half));
            let position = x87_div(x87_load32(shifted_bits), count);
            e.mem
                .set_u32(positions + 4 * index as u32, x87_store32(position));
        }
    }
    if n > 0 {
        let half_count = n >> 1;
        let mut j = 0i32;
        for i in 0..n {
            if i > j {
                let first = positions.wrapping_add(4 * j as u32);
                let second = positions.wrapping_add(4 * i as u32);
                let kept = x87_quiet_bits(e.mem.u32(first));
                let moved = x87_quiet_bits(e.mem.u32(second));
                e.mem.set_u32(first, moved);
                e.mem.set_u32(second, x87_quiet_bits(kept));
            }
            let mut m = half_count;
            while j >= m && m >= 2 {
                j -= m;
                m >>= 1;
            }
            j = j.wrapping_add(m);
        }
    }
    if n > 0 {
        let scale = x87_load64(e.global::<u64>(WORD_SCALE_010217D0));
        for index in 0..n as u32 {
            let value = x87_load32(e.mem.u32(positions + 4 * index));
            let word = e.call(FTOL_00EC62C0, &args![x87_mul(value, scale)]).u32();
            e.mem.set_u16(words + 2 * index, word as u16);
        }
    }
    with_locked(e, texture, |e, bits| {
        e.call(
            MEMCPY_00EC44D0,
            &args![bits, words, size.wrapping_add(size)],
        );
    });
    e.call(
        MEMORY_DEALLOCATE_00AA4060,
        &args![MEMORY_MANAGER, positions],
    );
    e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, words]);
    let texture = d3d_texture(e, this.addr() + 0x78);
    let buffer = allocate(e, array_bytes(size << 6, 4));
    let bytes = size << 8;
    e.call(MEMSET_00EC61C0, &args![buffer, 0u32, bytes]);
    let mut step = 1i32;
    let mut base = 0i32;
    for _ in 0..(if (passes as i32) > 0 { passes } else { 0 }) {
        let span = step.wrapping_add(step);
        let theta_bits = x87_store32(x87_div(
            f64::from_bits(e.global::<u64>(MINUS_TWO_PI_010B7F38)),
            f64::from(span.wrapping_neg()),
        ));
        let half_angle_bits = x87_store32(x87_mul(half, x87_m32(theta_bits)));
        let sine = e
            .call(SIN_00ECA0A0, &args![x87_load32(half_angle_bits)])
            .f64();
        let sine_bits = x87_quiet_bits(x87_store32(sine));
        let s = x87_load32(sine_bits);
        let factor = x87_load64(e.global::<u64>(MINUS_TWO_0106D4C8));
        let alpha_bits = x87_store32(x87_mul(s, x87_mul(factor, s)));
        let beta = e.call(SIN_00ECA0A0, &args![x87_load32(theta_bits)]).f64();
        let beta_bits = x87_quiet_bits(x87_store32(beta));
        let mut real_bits = x87_store32(1.0);
        let mut imaginary_bits = x87_store32(0.0);
        let alpha = x87_load32(alpha_bits);
        let beta = x87_load32(beta_bits);
        for m in 0..step.max(0) {
            let real = x87_load32(real_bits);
            let imaginary = x87_load32(imaginary_bits);
            if m < n {
                let count = x87_load32(x87_store32(f64::from(n)));
                let mut k = m;
                let mut low = buffer.wrapping_add((m.wrapping_add(base) as u32) << 4);
                let mut high =
                    buffer.wrapping_add((m.wrapping_add(base).wrapping_add(step) as u32) << 4);
                loop {
                    let far = x87_div(x87_add(f64::from(step.wrapping_add(k)), half), count);
                    e.mem.set_u32(high, x87_store32(-far));
                    let near_bits = x87_store32(x87_div(x87_add(f64::from(k), half), count));
                    let near = x87_quiet_bits(near_bits);
                    e.mem.set_u32(high + 4, near);
                    e.mem.set_u32(low + 4, near);
                    e.mem.set_u32(low, x87_store32(far));
                    e.mem.set_u32(high + 8, x87_store32(real));
                    e.mem.set_u32(high + 0xc, x87_store32(imaginary));
                    e.mem.set_u32(low + 8, x87_store32(real));
                    e.mem.set_u32(low + 0xc, x87_store32(imaginary));
                    let stride = (span as u32) << 4;
                    high = high.wrapping_add(stride);
                    low = low.wrapping_add(stride);
                    k = k.wrapping_add(span);
                    if k >= n {
                        break;
                    }
                }
            }
            // The twiddle turns: `FST` of the real part first (read back as a `float`).
            real_bits = x87_store32(real);
            let turned = x87_add(
                real,
                x87_sub(x87_mul(real, alpha), x87_mul(imaginary, beta)),
            );
            let next_real_bits = x87_store32(turned);
            let next_imaginary = x87_add(
                imaginary,
                x87_add(x87_mul(imaginary, alpha), x87_mul(beta, x87_m32(real_bits))),
            );
            imaginary_bits = x87_store32(next_imaginary);
            real_bits = next_real_bits;
        }
        base = base.wrapping_add(n);
        step = span;
    }
    with_locked(e, texture, |e, bits| {
        e.call(MEMCPY_00EC44D0, &args![bits, buffer, bytes]);
    });
    e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, buffer]);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, saved);
}

// Translated from 00ba1f20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba1f20 - the C++ exception frame (FS:[0] chain, unwind state 0) and the security-cookie bookkeeping are not translated.
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`), `thiscall`, one word (the
/// address of a byte): copies the spectrum rows into the textures, with the thread's
/// allocation context set to 0x1d. With the byte set, the texture of +0x58 is locked and the
/// `n x n` heights (rows +0xa8, through the FPU) written one after the other, and the byte
/// cleared. The texture of +0x68 always gets, for row `j` and column `i`, four `float`s: the
/// amplitude at `(n - j, n - i)` twice and at `(j, i)` twice (rows +0xac). `n` is `01200054`,
/// read again after each element.
pub fn fn_00ba1f20(e: &mut Engine, this: Ptr, refill: Ptr) {
    let base = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 0x1d);
    let grid = |e: &Engine| e.global::<u32>(GRID_SIZE_01200054) as i32;
    let element = |e: &Engine, rows: u32, row: i32, column: i32| {
        let row = e
            .mem
            .u32(e.mem.u32(base + rows).wrapping_add(4 * row as u32));
        x87_quiet_bits(e.mem.u32(row.wrapping_add(4 * column as u32)))
    };
    if e.mem.u8(refill.addr()) != 0 {
        let texture = d3d_texture(e, base + 0x58);
        with_locked(e, texture, |e, bits| {
            let mut out = bits;
            let mut row = 0;
            while row < grid(e) {
                let mut column = 0;
                while column < grid(e) {
                    let word = element(e, 0xa8, row, column);
                    column += 1;
                    e.mem.set_u32(out, word);
                    out = out.wrapping_add(4);
                }
                row += 1;
            }
        });
        e.mem.set_u8(refill.addr(), 0);
    }
    let texture = d3d_texture(e, base + 0x68);
    with_locked(e, texture, |e, bits| {
        let mut out = bits;
        let mut row = 0;
        while row < grid(e) {
            let mut column = 0;
            while column < grid(e) {
                for _ in 0..2 {
                    let n = grid(e);
                    let word = element(e, 0xac, n - row, n - column);
                    e.mem.set_u32(out, word);
                    out = out.wrapping_add(4);
                }
                for _ in 0..2 {
                    let word = element(e, 0xac, row, column);
                    e.mem.set_u32(out, word);
                    out = out.wrapping_add(4);
                }
                column += 1;
            }
            row += 1;
        }
    });
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, saved);
}

/// The wave-vector step (`010b7f40`, 2 pi / 128 as a `double` of a `float`'s precision).
const WAVE_STEP_010B7F40: u32 = 0x010b_7f40;

/// `((i - n * 0.5) * step)` stored as a `float` (`FILD`, the `double`s 0.5 and the step).
fn wave_number(e: &Engine, index: i32, n: i32) -> u32 {
    let half = f64::from_bits(e.global::<u64>(HALF_DOUBLE_01011588));
    let step = f64::from_bits(e.global::<u64>(WAVE_STEP_010B7F40));
    x87_store32(x87_mul(
        x87_sub(f64::from(index), x87_mul(f64::from(n), half)),
        step,
    ))
}

/// The dispersion `sqrt(sqrt(kx^2 + ky^2) * 9.81)` of a wave vector (each step stored as a
/// `float`; `kx^2` kept as a `double`).
fn dispersion(e: &mut Engine, kx_squared: f64, ky_bits: u32) -> u32 {
    let ky = x87_load32(ky_bits);
    let k2_bits = x87_store32(x87_add(x87_mul(ky, ky), kx_squared));
    let k = e.call(SQRT_00EC6040, &args![x87_load32(k2_bits)]).f64();
    let k_bits = x87_store32(k);
    let gravity = f64::from_bits(e.global::<u64>(GRAVITY_010B7F30));
    let scaled_bits = x87_store32(x87_mul(x87_load32(k_bits), gravity));
    let omega = e.call(SQRT_00EC6040, &args![x87_load32(scaled_bits)]).f64();
    x87_quiet_bits(x87_store32(omega))
}

/// A wave's random amplitude: with `kx` and `ky` both 0 (`FUCOM`) the spectrum term is 0 (and
/// `ky`'s slot becomes 0.0), else the square root of the Phillips spectrum (`00ba1480`); times
/// a table random number (`00ba13d0(0, 1)`) and `sin(2 pi rand() / 32767)` (each through a
/// `float`). Answers the amplitude's bits; `ky_slot` gets the spectrum term's bits.
fn wave_amplitude(e: &mut Engine, this: Ptr, kx_bits: u32, ky_slot: &mut u32) -> u32 {
    let kx = x87_load32(kx_bits);
    let zero_ky = x87_m32(*ky_slot) == 0.0;
    if kx == 0.0 && zero_ky {
        *ky_slot = x87_store32(0.0);
    } else {
        let ky = f32::from_bits(x87_quiet_bits(*ky_slot));
        let kx = f32::from_bits(x87_quiet_bits(kx_bits));
        let spectrum = fn_00ba1480(e, this, kx, ky);
        let root = e.call(SQRT_00EC6040, &args![f64::from(spectrum)]).f64();
        *ky_slot = x87_quiet_bits(x87_store32(root));
    }
    let random = fn_00ba13d0(e, 0.0, 1.0);
    let random_bits = x87_store32(f64::from(random));
    let value = e.call(RAND_00ECADB8, &args![]).i32();
    let divisor = f64::from_bits(e.global::<u64>(RAND_MAX_01029790));
    let unit_bits = x87_store32(x87_div(f64::from(value), divisor));
    let unit = x87_load32(unit_bits);
    let pi = f64::from_bits(e.global::<u64>(PI_0101FF40));
    let angle_bits = x87_store32(x87_mul(x87_add(unit, unit), pi));
    let sine = e.call(SIN_00ECA0A0, &args![x87_load32(angle_bits)]).f64();
    let sine_bits = x87_quiet_bits(x87_store32(sine));
    let product = x87_mul(
        x87_mul(x87_load32(random_bits), x87_m32(*ky_slot)),
        x87_m32(sine_bits),
    );
    x87_quiet_bits(x87_store32(product))
}

/// The CRT square root (`00ec6040`, argument and result in ST0).
const SQRT_00EC6040: u32 = 0x00ec_6040;

/// Row `row`, column `column` of the rows at `this + rows`.
fn cell(e: &Engine, this: u32, rows: u32, row: i32, column: i32) -> u32 {
    let row = e
        .mem
        .u32(e.mem.u32(this + rows).wrapping_add(4 * row as u32));
    row.wrapping_add(4 * column as u32)
}

// Translated from 00ba20e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba20e0 - the C++ exception frame (FS:[0] chain, unwind state 0) and the security-cookie bookkeeping are not translated.
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`), `thiscall`, no stack words:
/// builds the wave spectrum, with the thread's allocation context set to 0x1d. For each of the
/// `n + 1` rows (`n` = `01200054`) the rows at +0xa8 and +0xac are cleared (`n * 4 + 1` bytes;
/// +0xa8's again, `n * 4` bytes, below `n`). Then for each `(x, y)` in `0..=n` with the wave
/// vector `k = ((x - n/2) step, (y - n/2) step)`: the height row +0xa8 gets the dispersion
/// `sqrt(9.81 |k|)`, and the amplitude row +0xac the wave's random amplitude. With the byte
/// `01200059` set only the corner 32 x 32 blocks are computed; the edges copy them (rows
/// `x > n - 32` with `y < 32` from row `x - n + 64`, columns `y > n - 32` with `x < 32` from
/// column `y - n + 64`, both beyond from `(x + 64 - n, y + 64 - n)`), the inner cells stay
/// cleared. Last `00ba1f20` with a byte set to 1.
pub fn fn_00ba20e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 0x1d);
    let grid = |e: &Engine| e.global::<u32>(GRID_SIZE_01200054) as i32;
    let mut n = grid(e);
    let mut row = 0i32;
    while row < n.wrapping_add(1) {
        let heights = e
            .mem
            .u32(e.mem.u32(base + 0xa8).wrapping_add(4 * row as u32));
        e.call(
            MEMSET_00EC61C0,
            &args![heights, 0u32, (n as u32).wrapping_mul(4).wrapping_add(1)],
        );
        let n2 = grid(e);
        let amplitudes = e
            .mem
            .u32(e.mem.u32(base + 0xac).wrapping_add(4 * row as u32));
        e.call(
            MEMSET_00EC61C0,
            &args![
                amplitudes,
                0u32,
                (n2 as u32).wrapping_mul(4).wrapping_add(1)
            ],
        );
        n = grid(e);
        if row < n {
            let heights = e
                .mem
                .u32(e.mem.u32(base + 0xa8).wrapping_add(4 * row as u32));
            e.call(
                MEMSET_00EC61C0,
                &args![heights, 0u32, (n as u32).wrapping_mul(4)],
            );
            n = grid(e);
        }
        row += 1;
    }
    let large = e.global::<u8>(LARGE_GRID_01200059) != 0;
    if n >= 0 {
        let mut x = 0i32;
        loop {
            let kx_bits = wave_number(e, x, grid(e));
            let mut y = 0i32;
            if n >= 0 {
                let kx = x87_load32(kx_bits);
                let kx_squared = x87_mul(kx, kx);
                loop {
                    let mut ky_slot = wave_number(e, y, grid(e));
                    let omega = dispersion(e, kx_squared, ky_slot);
                    e.mem.set_u32(cell(e, base, 0xa8, x, y), omega);
                    let computed = if !large {
                        true
                    } else if x < 0x20 && y < 0x20 {
                        false
                    } else {
                        let size = grid(e);
                        let edge = size - 0x20;
                        if x > edge && y < 0x20 {
                            let from = cell(e, base, 0xac, x - size + 0x40, y);
                            let word = x87_quiet_bits(e.mem.u32(from));
                            e.mem.set_u32(cell(e, base, 0xac, x, y), word);
                            false
                        } else if x < 0x20 && y > edge {
                            let from = cell(e, base, 0xac, x, y - size + 0x40);
                            let word = x87_quiet_bits(e.mem.u32(from));
                            e.mem.set_u32(cell(e, base, 0xac, x, y), word);
                            false
                        } else if x > edge && y > edge {
                            let shift = 0x40 - size;
                            let from = cell(e, base, 0xac, shift + x, shift + y);
                            let word = x87_quiet_bits(e.mem.u32(from));
                            e.mem.set_u32(cell(e, base, 0xac, x, y), word);
                            false
                        } else {
                            true
                        }
                    };
                    if computed {
                        let amplitude = wave_amplitude(e, this, kx_bits, &mut ky_slot);
                        e.mem.set_u32(cell(e, base, 0xac, x, y), amplitude);
                    }
                    n = grid(e);
                    y += 1;
                    if y > n {
                        break;
                    }
                }
            }
            x += 1;
            if x > n {
                break;
            }
        }
    }
    e.with_stack(4, |e, refill| {
        e.mem.set_u8(refill.addr(), 1);
        fn_00ba1f20(e, this, refill);
    });
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, saved);
}

// Translated from 00ba2590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffectwaterfft.cpp`), `thiscall`, no stack words:
/// makes the row arrays +0xa8 and +0xac (`n + 1` pointers each, `n` = `01200054`, the size
/// checked for overflow) and their `n + 1` rows of `n + 1` `float`s from the memory manager,
/// builds the spectrum (`00ba20e0`) and the utility textures (`00ba1ac0` with `n` and
/// `01200050`), frees every row and both arrays, clears +0xa8 and +0xac and sets the byte
/// +0xb8.
pub fn fn_00ba2590(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let grid = |e: &Engine| e.global::<u32>(GRID_SIZE_01200054);
    let heights = allocate(e, array_bytes(grid(e).wrapping_add(1), 4));
    e.mem.set_u32(base + 0xa8, heights);
    let amplitudes = allocate(e, array_bytes(grid(e).wrapping_add(1), 4));
    e.mem.set_u32(base + 0xac, amplitudes);
    let mut row = 0i32;
    while row < grid(e).wrapping_add(1) as i32 {
        let block = allocate(e, array_bytes(grid(e).wrapping_add(1), 4));
        let rows = e.mem.u32(base + 0xa8);
        e.mem.set_u32(rows.wrapping_add(4 * row as u32), block);
        let block = allocate(e, array_bytes(grid(e).wrapping_add(1), 4));
        let rows = e.mem.u32(base + 0xac);
        e.mem.set_u32(rows.wrapping_add(4 * row as u32), block);
        row += 1;
    }
    fn_00ba20e0(e, this);
    let size = grid(e);
    let shift = e.global::<u32>(GRID_SHIFT_01200050);
    fn_00ba1ac0(e, this, size, shift);
    let mut row = 0i32;
    while row < grid(e).wrapping_add(1) as i32 {
        for rows in [0xa8u32, 0xac] {
            let block = e
                .mem
                .u32(e.mem.u32(base + rows).wrapping_add(4 * row as u32));
            e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, block]);
        }
        row += 1;
    }
    for rows in [0xa8u32, 0xac] {
        let block = e.mem.u32(base + rows);
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, block]);
    }
    e.mem.set_u32(base + 0xa8, 0);
    e.mem.set_u32(base + 0xac, 0);
    e.mem.set_u8(base + 0xb8, 1);
}

/// `ImageSpaceTexture::BorrowRenderedTexture` (engine map, `00ba3840`; `thiscall`, one word).
const TEXTURE_BORROW_00BA3840: u32 = 0x00ba_3840;

// Translated from 00ba26f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectWaterFFT::InitializeUtilityBuffers` (Xbox PDB), `thiscall`, no stack words:
/// each of the textures at +0x88, +0x78, +0x58 and +0x68 without a texture borrows a rendered
/// texture of kind 0xd, 0xe, 0xf and 0x10; then `00ba2590` (a tail jump).
pub fn image_space_effect_water_fft_initialize_utility_buffers(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, kind) in [(0x88u32, 0xdu32), (0x78, 0xe), (0x58, 0xf), (0x68, 0x10)] {
        let holder = base + offset;
        if e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![holder]).u32() == 0 {
            e.call(TEXTURE_BORROW_00BA3840, &args![holder, kind]);
        }
    }
    fn_00ba2590(e, this);
}

/// What `Render` calls: `ImageSpaceShaderParam::SetPixelConstant` (`00b8a790`; `thiscall`:
/// index, four `float`s), `ImageSpaceEffect::SetTexture` (`00ba3cb0`; `thiscall`: index,
/// texture, filter mode) and `ImageSpaceTexture::operator=` (`00ba3940`; `thiscall`, one word)
/// (engine map), and the `double`s 0.0625 (`010b7e98`) and 0.03125 (`010b7f48`) and the
/// `float` 4.0 (`0101e570`).
const SET_PIXEL_CONSTANT_00B8A790: u32 = 0x00b8_a790;
const SET_TEXTURE_00BA3CB0: u32 = 0x00ba_3cb0;
const TEXTURE_ASSIGN_00BA3940: u32 = 0x00ba_3940;
const PASS_STEP_010B7E98: u32 = 0x010b_7e98;
const PASS_OFFSET_010B7F48: u32 = 0x010b_7f48;
const FOUR_0101E570: u32 = 0x0101_e570;

/// Effect `index` of the array at +0xc (null for a null or 0x6c pointer: the compiler's
/// adjustment of a null base pointer) and parameter `index` of the array at +0x1c.
fn effect_and_param(e: &Engine, this: u32, index: u32) -> (u32, u32) {
    let effect = e.mem.u32(e.mem.u32(this + 0xc) + 4 * index);
    let effect = if effect == 0 || effect.wrapping_sub(0x6c) == 0 {
        0
    } else {
        effect
    };
    (effect, e.mem.u32(e.mem.u32(this + 0x1c) + 4 * index))
}

/// The effect's three textures (filter mode 0 each).
fn set_textures(e: &mut Engine, effect: u32, textures: [u32; 3]) {
    for (index, texture) in textures.iter().enumerate() {
        e.call(
            SET_TEXTURE_00BA3CB0,
            &args![effect, index as u32, *texture, 0u32],
        );
    }
}

/// The ping-pong of three stack textures through a fourth: `d = b; b = c; c = d`
/// (`ImageSpaceTexture::operator=`).
fn rotate(e: &mut Engine, b: u32, c: u32, d: u32) {
    e.call(TEXTURE_ASSIGN_00BA3940, &args![d, b]);
    e.call(TEXTURE_ASSIGN_00BA3940, &args![b, c]);
    e.call(TEXTURE_ASSIGN_00BA3940, &args![c, d]);
}

/// Pixel constant 1 of `param` for pass `index`: `index * 0.0625 + 0.03125` (stored as a
/// `float`, also at +0xb4), 4.0, 0, 0.
fn set_pass_constant(e: &mut Engine, this: u32, param: u32, index: i32) {
    let step = f64::from_bits(e.global::<u64>(PASS_STEP_010B7E98));
    let offset = f64::from_bits(e.global::<u64>(PASS_OFFSET_010B7F48));
    let value_bits = x87_store32(x87_add(x87_mul(f64::from(index), step), offset));
    let value = x87_quiet_bits(value_bits);
    e.mem.set_u32(this + 0xb4, value);
    let four = x87_quiet_bits(e.global::<u32>(FOUR_0101E570));
    e.call(
        SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 1u32, value, four, 0u32, 0u32],
    );
}

// Translated from 00ba2760 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba2760 - the C++ exception frame (FS:[0] chain, unwind states 3 to 0 for the four stack textures) and the security-cookie bookkeeping are not translated.
/// `ImageSpaceEffectWaterFFT::Render` (Xbox PDB), `thiscall`, four words (two passed on to the
/// effects' slot +4, one to `this`'s slot +0x1c, one not read). Without the utility buffers
/// (byte +0xb8) they are made (`InitializeUtilityBuffers`); with the byte +5 set it becomes
/// what slot +0x1c answers for the third word. Three stack textures A, C, B borrow rendered
/// textures of kind 7 (a fourth, D, takes part in the swaps). Effect 0: constant 0 = (+0xb0, 0,
/// 0, 0); textures A, +0x68, +0x58. Effect 3: C, +0x88, A. Then `[01200050]` passes of effect
/// 1 (constant 1 per pass; C, +0x78, B), effect 4 (C, +0x88, B), and `[01200050]` passes of
/// effect 2 (C, or `pTextures[1]` on the last pass, +0x78, B); after each effect but the last
/// pass B and C swap through D. Last effect 5 (with the byte `01200058`: constant 2 =
/// (`0120004c`, 4.0, 0, 0)) or 6 on `pTextures[0]` and `[1]`. Each effect's slot +4 gets the
/// first two words, its parameter and 1. The borrowed textures are given back and the four
/// destroyed.
pub fn image_space_effect_water_fft_render(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    active: u32,
    _unused_3: u32,
) {
    let base = this.addr();
    if e.mem.u8(base + 0xb8) == 0 {
        image_space_effect_water_fft_initialize_utility_buffers(e, this);
    }
    let passes = e.global::<u32>(GRID_SHIFT_01200050) as i32;
    if e.mem.u8(base + 5) != 0 {
        let value = e.vcall(base, 0x1c, &args![active]).u8();
        e.mem.set_u8(base + 5, value);
    }
    e.with_stack(0x40, |e, frame| {
        let c = frame.addr();
        let b = c + 0x10;
        let d = c + 0x20;
        let a = c + 0x30;
        let clear = |e: &mut Engine, texture: u32| {
            for offset in 0..3 {
                e.mem.set_u8(texture + offset, 0);
            }
            for offset in [4u32, 8, 0xc] {
                e.mem.set_u32(texture + offset, 0);
            }
        };
        clear(e, a);
        clear(e, b);
        clear(e, c);
        for texture in [a, c, b] {
            e.call(TEXTURE_BORROW_00BA3840, &args![texture, 7u32]);
        }
        clear(e, d);
        let render = |e: &mut Engine, effect: u32, param: u32| {
            e.vcall(effect, 4, &args![first, second, param, 1u32]);
        };
        let (effect, param) = effect_and_param(e, base, 0);
        let scale = x87_quiet_bits(e.mem.u32(base + 0xb0));
        e.call(
            SET_PIXEL_CONSTANT_00B8A790,
            &args![param, 0u32, scale, 0u32, 0u32, 0u32],
        );
        set_textures(e, effect, [a, base + 0x68, base + 0x58]);
        render(e, effect, param);
        let (effect, param) = effect_and_param(e, base, 3);
        set_textures(e, effect, [c, base + 0x88, a]);
        render(e, effect, param);
        rotate(e, b, c, d);
        let (effect, param) = effect_and_param(e, base, 1);
        let mut pass = 0;
        while pass < passes {
            set_pass_constant(e, base, param, pass);
            set_textures(e, effect, [c, base + 0x78, b]);
            render(e, effect, param);
            rotate(e, b, c, d);
            pass += 1;
        }
        let (effect, param) = effect_and_param(e, base, 4);
        set_textures(e, effect, [c, base + 0x88, b]);
        render(e, effect, param);
        rotate(e, b, c, d);
        let (effect, param) = effect_and_param(e, base, 2);
        let mut pass = 0;
        while pass < passes {
            set_pass_constant(e, base, param, pass);
            let last = pass == passes - 1;
            let first_texture = if last {
                e.mem.u32(e.mem.u32(base + 0x2c) + 4)
            } else {
                c
            };
            set_textures(e, effect, [first_texture, base + 0x78, b]);
            render(e, effect, param);
            if !last {
                rotate(e, b, c, d);
            }
            pass += 1;
        }
        let (effect, param) = if e.global::<u8>(FLAG_01200058) != 0 {
            let (effect, param) = effect_and_param(e, base, 5);
            let four = x87_quiet_bits(e.global::<u32>(FOUR_0101E570));
            let inverse = x87_quiet_bits(e.global::<u32>(INVERSE_SIZE_0120004C));
            e.call(
                SET_PIXEL_CONSTANT_00B8A790,
                &args![param, 2u32, inverse, four, 0u32, 0u32],
            );
            (effect, param)
        } else {
            effect_and_param(e, base, 6)
        };
        let textures = e.mem.u32(base + 0x2c);
        let texture_0 = e.mem.u32(textures);
        e.call(SET_TEXTURE_00BA3CB0, &args![effect, 0u32, texture_0, 0u32]);
        let texture_1 = e.mem.u32(e.mem.u32(base + 0x2c) + 4);
        e.call(SET_TEXTURE_00BA3CB0, &args![effect, 1u32, texture_1, 0u32]);
        render(e, effect, param);
        for texture in [a, c, b] {
            if e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![texture]).u32() != 0 {
                e.call(TEXTURE_RETURN_RENDERED_00BA39A0, &args![texture]);
            }
        }
        for texture in [d, c, b, a] {
            e.call(TEXTURE_DESTRUCT_00BA3A00, &args![texture]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_13b0, fn_00ba13b0(Ptr, u32) -> bool),
        entry!(0x00ba_13d0, fn_00ba13d0(f32, f32) -> f32),
        entry!(0x00ba_1480, fn_00ba1480(Ptr, f32, f32) -> f32),
        entry!(0x00ba_1ac0, fn_00ba1ac0(Ptr, u32, u32)),
        entry!(0x00ba_1f20, fn_00ba1f20(Ptr, Ptr)),
        entry!(0x00ba_20e0, fn_00ba20e0(Ptr)),
        entry!(0x00ba_2590, fn_00ba2590(Ptr)),
        entry!(
            0x00ba_26f0,
            image_space_effect_water_fft_initialize_utility_buffers(Ptr)
        ),
        entry!(
            0x00ba_2760,
            image_space_effect_water_fft_render(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00ba_15e0, fn_00ba15e0(Ptr) -> Ptr),
        entry!(
            0x00ba_1250,
            image_space_effect_water_fft_free_up_texture_memory(Ptr)
        ),
        entry!(0x00ba_12d0, fn_00ba12d0(Ptr)),
        entry!(0x00ba_1750, fn_00ba1750(Ptr, u32) -> Ptr),
        entry!(0x00ba_1770, fn_00ba1770(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct(large: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_SIZE_00A90180, |_, a| a[0].into_ret());
        for page in [0x0101_7000u32, 0x0102_4000, 0x0101_6000, 0x0120_0000] {
            e.map(page, 0x1000);
        }
        e.set_global(CONSTANT_0101712C, 5.0f32);
        e.set_global(CONSTANT_010249D8, 0x7f80_0002u32);
        e.set_global(HALF_01016248, 0.5f32);
        e.set_global(LARGE_GRID_01200059, large);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xb8);
        e.mem.write(this, &[0xcc; 0xb8]);
        assert_eq!(e.call(0x00ba_15e0, &args![this]).u32(), this);
        (e, this)
    }

    #[test]
    fn fn_00ba15e0_builds_the_effect() {
        let (mut e, this) = construct(0);
        assert_eq!(e.mem.u32(this), VTABLE_010B7F10);
        for block in (0x58u32..=0x98).step_by(0x10) {
            assert_eq!(e.mem.bytes(this + block, 3), vec![0; 3]);
            assert_eq!(e.mem.u8(this + block + 3), 0xcc);
            assert_eq!(e.mem.bytes(this + block + 4, 0xc), vec![0; 0xc]);
        }
        assert_eq!(e.mem.u32(this + 0xa8), 0xcccc_cccc);
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        assert_eq!(e.mem.u32(this + 0xb4), 0);
        assert_eq!(e.global::<f32>(PARAMETER_01200068), 5.0);
        assert_eq!(e.global::<u32>(PARAMETER_01200064), 0x7fc0_0002);
        assert_eq!(e.global::<f32>(PARAMETER_01200060), 0.5);
        assert_eq!(e.global::<f32>(PARAMETER_0120005C), 1.0);
        assert_eq!(e.global::<u32>(EFFECT_0120006C), this);
        assert_eq!(e.global::<u32>(GRID_SIZE_01200054), 0x80);
        assert_eq!(e.global::<u32>(GRID_SHIFT_01200050), 7);
        assert_eq!(e.global::<u8>(FLAG_01200058), 1);
        assert_eq!(e.global::<f32>(INVERSE_SIZE_0120004C), 1.0 / 128.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 10]),
            ]
        );
    }

    #[test]
    fn fn_00ba1250_returns_the_held_textures() {
        let mut e = Engine::new();
        e.register(TEXTURE_GET_TEXTURE_00BA3780, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(TEXTURE_RETURN_RENDERED_00BA39A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xbc);
        for offset in [0x88u32, 0x58] {
            e.mem.set_u32(this + offset + 4, 0x77);
        }
        e.mem.set_u8(this + 0xb8, 1);
        e.call(0x00ba_1250, &args![this]);
        let log = e.call_log.take().unwrap();
        let asked: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == TEXTURE_GET_TEXTURE_00BA3780)
            .map(|c| c.1[0] - this)
            .collect();
        assert_eq!(asked, vec![0x78, 0x88, 0x58, 0x68, 0x98]);
        let returned: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == TEXTURE_RETURN_RENDERED_00BA39A0)
            .map(|c| c.1[0] - this)
            .collect();
        assert_eq!(returned, vec![0x88, 0x58]);
        assert_eq!(e.mem.u8(this + 0xb8), 0);
    }

    #[test]
    fn fn_00ba12d0_and_00ba1750_delete_the_parameters_and_textures() {
        let mut e = Engine::new();
        for callee in [
            ARRAY_SET_AT_00BA3AA0,
            EFFECT_DESTRUCT_00BA3D30,
            MEMORY_DEALLOCATE_00AA4060,
            TEXTURE_DESTRUCT_00BA3A00,
            0x0e6e_0000,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(0x0e6f_0000, &[0x0e6e_0000]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xbc);
        let list = e.mem.alloc(0x30);
        let param = e.mem.alloc(8);
        e.mem.set_u32(param, 0x0e6f_0000);
        e.mem.set_u32(list + 4 * 0xb, param);
        e.mem.set_u32(this + 0x1c, list);
        assert_eq!(e.call(0x00ba_1750, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B7F10);
        let log = e.call_log.take().unwrap();
        let of = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(of(0x0e6e_0000), vec![vec![param, 1]]);
        assert_eq!(of(ARRAY_SET_AT_00BA3AA0)[0][..2], [this + 0x18, 0xb]);
        let destroyed: Vec<u32> = of(TEXTURE_DESTRUCT_00BA3A00)
            .iter()
            .map(|c| c[0] - this)
            .collect();
        assert_eq!(destroyed, vec![0x98, 0x88, 0x78, 0x68, 0x58]);
        assert_eq!(of(EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert_eq!(
            of(MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn fn_00ba1770_sets_up_seven_parameters() {
        let mut e = Engine::new();
        e.map(0x0e70_0000, 0x1000);
        e.mem.write(0x0e70_0000, &[0xcc; 0x1000]);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let mut next = 0x0e70_0000u32;
        e.register_double(PARAM_CONSTRUCT_00B8A9E0, move |_, _| {
            next += 0x100;
            next.into_ret()
        });
        for callee in [
            SIZE_ARRAYS_00BA3F90,
            ARRAY_SET_SIZE_00A90180,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            ARRAY_SET_AT_00BA3AA0,
            SET_EFFECT_00BA3C70,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xbc);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0x190);
        for index in 0..7u32 {
            e.mem
                .set_u32(effects + 0x174 + 4 * index, 0x0e71_0000 + index);
        }
        e.mem.set_u32(manager + 8, effects);
        e.call(0x00ba_1770, &args![this, manager, 0u32]);
        let log = e.call_log.take().unwrap();
        let of = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(of(SIZE_ARRAYS_00BA3F90), vec![vec![this, 0xc]]);
        assert_eq!(of(ARRAY_SET_SIZE_00A90180), vec![vec![this + 0x28, 0xa]]);
        let expected: Vec<Vec<u32>> = (0..7u32)
            .map(|i| vec![this, i, 0x0e71_0000 + i, 0x0e70_0100 + 0x100 * i, 0])
            .collect();
        assert_eq!(of(SET_EFFECT_00BA3C70), expected);
        let maps: Vec<Vec<u32>> = (0..7u32)
            .map(|i| vec![0x0e70_0100 + 0x100 * i, 2, 3])
            .collect();
        assert_eq!(of(PARAM_INIT_CONSTANT_MAP_00B8AB10), maps);
        for i in 0..7u32 {
            let param = 0x0e70_0100 + 0x100 * i;
            assert_eq!(e.mem.u8(param + 0x55), 0);
            assert_eq!(e.mem.u8(param + 0x56), 0xcc);
        }
        let indexes: Vec<u32> = of(ARRAY_SET_AT_00BA3AA0).iter().map(|c| c[1]).collect();
        assert_eq!(indexes, (0..7).collect::<Vec<u32>>());
    }

    #[test]
    fn fn_00ba15e0_large_grid() {
        let (e, _) = construct(1);
        assert_eq!(e.global::<u32>(GRID_SIZE_01200054), 0x100);
        assert_eq!(e.global::<u32>(GRID_SHIFT_01200050), 8);
        assert_eq!(e.global::<f32>(INVERSE_SIZE_0120004C), 1.0 / 256.0);
    }
}

#[cfg(test)]
mod n070_tests {
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

    fn f64_arg(a: &[u32]) -> f64 {
        f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32)
    }

    const LOCK_SLOT: u32 = 0x0e70_0000;
    const UNLOCK_SLOT: u32 = 0x0e70_0004;
    const D3D_SLOT: u32 = 0x0e70_0008;
    const RENDER_SLOT: u32 = 0x0e70_000c;
    const ACTIVE_SLOT: u32 = 0x0e70_0010;

    /// The exe's constants at their addresses, the CRT doubles (`sqrt`, `sin`, `cos`, `exp`
    /// as the host's, `rand` 16383, `_ftol2` truncation, `memset`, `memcpy`), the memory
    /// manager, and the random table with every threshold 2.0 and value `k` = `k + 1`.
    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_6000,
            0x0101_a000,
            0x0101_e000,
            0x0101_f000,
            0x0102_1000,
            0x0102_9000,
            0x0104_e000,
            0x0104_f000,
            0x0106_d000,
            0x0107_3000,
            0x0108_c000,
            0x010b_7000,
            0x011a_d000,
            0x011f_9000,
            0x0120_0000,
        ] {
            e.map(page, 0x1000);
        }
        for (addr, value) in [
            (HALF_DOUBLE_01011588, 0.5f64),
            (ZERO_01012060, 0.0),
            (MINUS_ONE_0101A6B0, -1.0),
            (PI_0101FF40, f64::from(std::f32::consts::PI)),
            (WORD_SCALE_010217D0, 65535.0),
            (RAND_MAX_01029790, 32767.0),
            (QUARTER_010290B0, 0.25),
            (DEGREES_0104ED58, 180.0),
            (DIVISOR_0104F2F0, 25.0),
            (MINUS_TWO_0106D4C8, -2.0),
            (DIVISOR_010731A8, 75.0),
            (AMPLITUDE_DIVISOR_0108C160, 100000.0),
            (GRAVITY_010B7F30, f64::from(9.81f32)),
            (MINUS_TWO_PI_010B7F38, -f64::from(std::f32::consts::TAU)),
            (PASS_STEP_010B7E98, 0.0625),
            (WAVE_STEP_010B7F40, f64::from(std::f32::consts::TAU / 128.0)),
            (PASS_OFFSET_010B7F48, 0.03125),
        ] {
            e.set_global(addr, value);
        }
        e.set_global(HALF_01016248, 0.5f32);
        e.set_global(FOUR_0101E570, 4.0f32);
        for k in 0..8u32 {
            e.set_global(TABLE_VALUES_011ADA18 + 8 * k, (k + 1) as f32);
            e.set_global(TABLE_THRESHOLDS_011ADA24 + 8 * k, 2.0f32);
        }
        e.register(SQRT_00EC6040, |_, a| f64_arg(a).sqrt().into_ret());
        e.register(SIN_00ECA0A0, |_, a| f64_arg(a).sin().into_ret());
        e.register(COS_00EC9F70, |_, a| f64_arg(a).cos().into_ret());
        e.register(EXP_00ECAE6C, |_, a| f64_arg(a).exp().into_ret());
        e.register(RAND_00ECADB8, |_, _| 16383u32.into_ret());
        e.register(FTOL_00EC62C0, |_, a| (f64_arg(a) as i32).into_ret());
        e.register(MEMSET_00EC61C0, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register(MEMCPY_00EC44D0, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_DEALLOCATE_00AA4060, |_, _| Ret::default());
        // `GetTexture` answers the holder's +4; the texture's +0x24 has slot +0x9c answering
        // its +8, a Direct3D texture whose lock hands out its +8 as the bits.
        e.register(TEXTURE_GET_TEXTURE_00BA3780, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(D3D_SLOT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(LOCK_SLOT, |e, a| {
            let bits = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[2] + 4, bits);
            Ret::default()
        });
        e.register(UNLOCK_SLOT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    /// An image-space texture at `holder` whose Direct3D texture locks to `bytes` new bytes;
    /// answers the bits.
    fn locked_texture(e: &mut Engine, holder: u32, bytes: u32) -> u32 {
        let d3d_vtable = e.mem.alloc(0x60);
        e.mem.set_u32(d3d_vtable + 0x4c, LOCK_SLOT);
        e.mem.set_u32(d3d_vtable + 0x50, UNLOCK_SLOT);
        let d3d = e.mem.alloc(0xc);
        e.mem.set_u32(d3d, d3d_vtable);
        let bits = e.mem.alloc(bytes);
        e.mem.set_u32(d3d + 8, bits);
        let data_vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(data_vtable + 0x9c, D3D_SLOT);
        let data = e.mem.alloc(0xc);
        e.mem.set_u32(data, data_vtable);
        e.mem.set_u32(data + 8, d3d);
        let texture = e.mem.alloc(0x28);
        e.mem.set_u32(texture + 0x24, data);
        e.mem.set_u32(holder + 4, texture);
        bits
    }

    #[test]
    fn fn_00ba13b0_scales_the_speed() {
        let mut e = rig();
        e.set_global(SPEED_011F940C, 50.0f32);
        e.set_global(PARAMETER_0120005C, 3.0f32);
        let this = e.mem.alloc(0xc0);
        assert!(e.call(0x00ba_13b0, &args![this, 0u32]).bool());
        assert_eq!(e.mem.f32(this + 0xb0), 6.0);
    }

    #[test]
    fn fn_00ba13d0_picks_from_the_table() {
        let mut e = rig();
        // r = 16383 / 32767 (just below 0.5): 1 - the value for 1 - r (k = 0 here: 1.0).
        let r = (16383.0f64 / 32767.0) as f32;
        assert!(r < 0.5);
        let got = e.call(0x00ba_13d0, &args![10.0f32, 2.0f32]).f32();
        assert_eq!(got, 10.0);
        // r = 1.0: thresholds 2.0, 2.0, then 0.5 at k = 2: value 3.0.
        e.register(RAND_00ECADB8, |_, _| 32767u32.into_ret());
        e.set_global(TABLE_THRESHOLDS_011ADA24, 0.25f32);
        e.set_global(TABLE_THRESHOLDS_011ADA24 + 8, 0.75f32);
        let got = e.call(0x00ba_13d0, &args![1.0f32, 2.0f32]).f32();
        assert_eq!(got, 1.0 + 2.0 * 3.0);
        // Below 0.5: the search runs on 1 - r (r = 0: 1.0 -> k = 2), 1 - 3.0.
        e.register(RAND_00ECADB8, |_, _| 0u32.into_ret());
        let got = e.call(0x00ba_13d0, &args![0.0f32, 1.0f32]).f32();
        assert_eq!(got, -2.0);
    }

    #[test]
    fn fn_00ba1480_phillips_spectrum() {
        let mut e = rig();
        // Angle 0 (cos 1, sin 0), A = 1, wind 0 (L = 0: exp(-inf) = 0 ... with exp doubled).
        e.set_global(PARAMETER_01200064, 0.0f32);
        e.set_global(PARAMETER_01200060, 100000.0f32);
        e.set_global(PARAMETER_01200068, 3.0f32);
        e.register(EXP_00ECAE6C, |_, _| 0.5f64.into_ret());
        let this = e.mem.alloc(4);
        // k = (1, 0): d = 1, k2 = 1: 0.5 * (1 * 1 * 0.5 / 1) = 0.25.
        let got = e.call(0x00ba_1480, &args![this, 1.0f32, 0.0f32]).f32();
        assert_eq!(got, 0.25);
        let exps = calls(&e, EXP_00ECAE6C);
        let length = (9.0f64 / f64::from(9.81f32)) as f32;
        let small = (f64::from(length) / 75.0) as f32;
        let first = (-1.0 / (f64::from(length) * f64::from(length))) as f32;
        assert_eq!(f64_arg(&exps[0]) as f32, first);
        let second = ((-1.0f64 * f64::from(small)) * f64::from(small)) as f32;
        assert_eq!(f64_arg(&exps[1]) as f32, second);
        // A wave against the wind: d clamped to 0, the spectrum 0.
        let got = e.call(0x00ba_1480, &args![this, -1.0f32, 0.0f32]).f32();
        assert_eq!(got, 0.0);
        // k = (0, 2) with sin 1: d = 2, k2 = 4: 0.5 * (4 * 0.5 / 64).
        e.register(COS_00EC9F70, |_, _| 0.0f64.into_ret());
        e.register(SIN_00ECA0A0, |_, _| 1.0f64.into_ret());
        let got = e.call(0x00ba_1480, &args![this, 0.0f32, 2.0f32]).f32();
        assert_eq!(got, 0.5 * (4.0 * 0.5 / 64.0));
    }

    #[test]
    fn fn_00ba1ac0_fills_the_bit_reversal_and_butterfly_tables() {
        let mut e = rig();
        let this = e.mem.alloc(0xc0);
        let reversal = locked_texture(&mut e, this + 0x88, 8);
        let butterfly = locked_texture(&mut e, this + 0x78, 4 << 8);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 5);
        e.call(0x00ba_1ac0, &args![this, 4u32, 2u32]);
        let words: Vec<u16> = (0..4).map(|i| e.mem.u16(reversal + 2 * i)).collect();
        // (i + 0.5) / 4 in bit-reversed order (0, 2, 1, 3), times 65535, truncated.
        assert_eq!(words, vec![8191, 40959, 24575, 57343]);
        let entry = |e: &Engine, index: u32| -> [f32; 4] {
            [0, 4, 8, 0xc].map(|offset| e.mem.f32(butterfly + 16 * index + offset))
        };
        // Pass 0 (span 2): pairs (0, 1) and (2, 3), w = (1, 0).
        assert_eq!(entry(&e, 0), [0.375, 0.125, 1.0, 0.0]);
        assert_eq!(entry(&e, 1), [-0.375, 0.125, 1.0, 0.0]);
        assert_eq!(entry(&e, 2), [0.875, 0.625, 1.0, 0.0]);
        assert_eq!(entry(&e, 3), [-0.875, 0.625, 1.0, 0.0]);
        // Pass 1 (span 4): pairs (4, 6) with w = (1, 0) and (5, 7) with w turned by pi/2.
        assert_eq!(entry(&e, 4), [0.625, 0.125, 1.0, 0.0]);
        assert_eq!(entry(&e, 6), [-0.625, 0.125, 1.0, 0.0]);
        let turned = entry(&e, 5);
        assert_eq!(turned[..2], [0.875, 0.375]);
        assert!(turned[2].abs() < 1e-6);
        assert!((turned[3] - 1.0).abs() < 1e-6);
        assert_eq!(entry(&e, 7)[0], -0.875);
        assert_eq!(e.mem.u32(tls + TLS_ALLOCATION_CONTEXT), 5);
        // Three blocks taken and freed.
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE)
                .iter()
                .map(|a| a[1])
                .collect::<Vec<_>>(),
            vec![16, 8, 4 << 8]
        );
        assert_eq!(calls(&e, MEMORY_DEALLOCATE_00AA4060).len(), 3);
    }

    /// `n + 1` rows of `n + 1` `float`s at +0xa8 and +0xac holding `row * 10 + column` and
    /// `100 + row * 10 + column`.
    fn rows(e: &mut Engine, this: u32, n: u32) {
        for (offset, start) in [(0xa8u32, 0.0f32), (0xac, 100.0)] {
            let array = e.mem.alloc(4 * (n + 1));
            for row in 0..=n {
                let block = e.mem.alloc(4 * (n + 1));
                for column in 0..=n {
                    let value = start + (row * 10 + column) as f32;
                    e.mem.set_f32(block + 4 * column, value);
                }
                e.mem.set_u32(array + 4 * row, block);
            }
            e.mem.set_u32(this + offset, array);
        }
    }

    #[test]
    fn fn_00ba1f20_copies_heights_and_amplitudes() {
        let mut e = rig();
        e.set_global(GRID_SIZE_01200054, 2u32);
        let this = e.mem.alloc(0xc0);
        rows(&mut e, this, 2);
        let heights = locked_texture(&mut e, this + 0x58, 16);
        let amplitudes = locked_texture(&mut e, this + 0x68, 64);
        let refill = e.mem.alloc(4);
        e.mem.set_u8(refill, 1);
        e.call(0x00ba_1f20, &args![this, refill]);
        let floats = |e: &Engine, bits: u32, count: u32| -> Vec<f32> {
            (0..count).map(|i| e.mem.f32(bits + 4 * i)).collect()
        };
        assert_eq!(floats(&e, heights, 4), vec![0.0, 1.0, 10.0, 11.0]);
        assert_eq!(e.mem.u8(refill), 0);
        // (0, 0): (2, 2) twice, (0, 0) twice; (0, 1): (2, 1) twice, (0, 1) twice.
        assert_eq!(
            floats(&e, amplitudes, 8),
            vec![122.0, 122.0, 100.0, 100.0, 121.0, 121.0, 101.0, 101.0]
        );
        // Without the byte only the amplitudes are written.
        e.mem.set_f32(heights, 7.0);
        e.call(0x00ba_1f20, &args![this, refill]);
        assert_eq!(e.mem.f32(heights), 7.0);
    }

    /// The water effect for `00ba20e0` with grid `n` and both textures of `00ba1f20`.
    fn spectrum_rig(n: u32, large: u8) -> (Engine, u32) {
        let mut e = rig();
        e.set_global(GRID_SIZE_01200054, n);
        e.set_global(LARGE_GRID_01200059, large);
        e.set_global(PARAMETER_01200064, 0.0f32);
        e.set_global(PARAMETER_01200060, 100000.0f32);
        e.set_global(PARAMETER_01200068, 10.0f32);
        // The table random is then 1 - 3 = -2.
        e.set_global(TABLE_VALUES_011ADA18, 3.0f32);
        let this = e.mem.alloc(0xc0);
        rows(&mut e, this, n);
        locked_texture(&mut e, this + 0x58, 4 * n * n);
        locked_texture(&mut e, this + 0x68, 16 * n * n);
        (e, this)
    }

    #[test]
    fn fn_00ba20e0_builds_the_spectrum() {
        let (mut e, this) = spectrum_rig(2, 0);
        e.call(0x00ba_20e0, &args![this]);
        let at = |e: &Engine, rows: u32, row: u32, column: u32| {
            e.mem
                .f32(e.mem.u32(e.mem.u32(this + rows) + 4 * row) + 4 * column)
        };
        // Grid 2: k = (i - 1) * step; the centre (1, 1) has k = 0: no spectrum term.
        assert_eq!(at(&e, 0xa8, 1, 1), 0.0);
        assert_eq!(at(&e, 0xac, 1, 1), 0.0);
        let step = f64::from(std::f32::consts::TAU / 128.0);
        let k2 = (step * step) as f32;
        let k = f64::from(k2).sqrt() as f32;
        let scaled = (f64::from(k) * f64::from(9.81f32)) as f32;
        let omega = f64::from(scaled).sqrt() as f32;
        assert_eq!(at(&e, 0xa8, 1, 2), omega);
        // (2, 1): k = (step, 0) along the wind: a spectrum term, a nonzero amplitude.
        assert_ne!(at(&e, 0xac, 2, 1), 0.0);
        // (0, 0): against the wind: 0.
        assert_eq!(at(&e, 0xac, 0, 0), 0.0);
        // Nine table randoms and nine `rand` angles (one per cell).
        assert_eq!(calls(&e, RAND_00ECADB8).len(), 18);
        // Rows cleared first: `n * 4 + 1` bytes each, +0xa8 again with `n * 4`.
        let sizes: Vec<u32> = calls(&e, MEMSET_00EC61C0).iter().map(|a| a[2]).collect();
        assert_eq!(sizes, vec![9, 9, 8, 9, 9, 8, 9, 9]);
        // `00ba1f20` refilled the heights texture.
        assert!(calls(&e, LOCK_SLOT).len() == 2);
    }

    #[test]
    fn fn_00ba20e0_large_grid_computes_only_the_cross() {
        let (mut e, this) = spectrum_rig(64, 1);
        e.call(0x00ba_20e0, &args![this]);
        // With n = 64 every quadrant is copied from itself: only row or column 32 computed.
        assert_eq!(calls(&e, RAND_00ECADB8).len(), 2 * 129);
        let at = |e: &Engine, row: u32, column: u32| {
            e.mem
                .f32(e.mem.u32(e.mem.u32(this + 0xac) + 4 * row) + 4 * column)
        };
        assert_eq!(at(&e, 10, 10), 0.0);
        assert_eq!(at(&e, 40, 40), 0.0);
        assert_ne!(at(&e, 50, 32), 0.0);
    }

    #[test]
    fn fn_00ba2590_makes_and_frees_the_rows() {
        let (mut e, this) = spectrum_rig(2, 0);
        locked_texture(&mut e, this + 0x88, 8);
        locked_texture(&mut e, this + 0x78, 2 << 8);
        e.set_global(GRID_SHIFT_01200050, 1u32);
        e.mem.set_u32(this + 0xa8, 0);
        e.mem.set_u32(this + 0xac, 0);
        e.call(0x00ba_2590, &args![this]);
        let sizes: Vec<u32> = calls(&e, MEMORY_ALLOCATE).iter().map(|a| a[1]).collect();
        // Two arrays of 3 pointers, then 2 x 3 rows of 3 floats, then the tables' blocks.
        assert_eq!(sizes[..8], [12, 12, 12, 12, 12, 12, 12, 12]);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE_00AA4060).len(), 8 + 3);
        assert_eq!(e.mem.u32(this + 0xa8), 0);
        assert_eq!(e.mem.u32(this + 0xac), 0);
        assert_eq!(e.mem.u8(this + 0xb8), 1);
    }

    #[test]
    fn initialize_utility_buffers_borrows_missing_textures() {
        let (mut e, this) = spectrum_rig(2, 0);
        locked_texture(&mut e, this + 0x88, 8);
        locked_texture(&mut e, this + 0x78, 2 << 8);
        e.set_global(GRID_SHIFT_01200050, 1u32);
        // +0x88, +0x78, +0x58 and +0x68 have textures; take +0x58's away.
        let kept = e.mem.u32(this + 0x58 + 4);
        e.mem.set_u32(this + 0x58 + 4, 0);
        e.set_global(0x0120_0f00, kept);
        e.register(TEXTURE_BORROW_00BA3840, |e, a| {
            let texture = e.mem.u32(0x0120_0f00);
            e.mem.set_u32(a[0] + 4, texture);
            Ret::default()
        });
        e.call(0x00ba_26f0, &args![this]);
        assert_eq!(
            calls(&e, TEXTURE_BORROW_00BA3840),
            vec![vec![this + 0x58, 0xf]]
        );
        assert_eq!(e.mem.u8(this + 0xb8), 1);
    }

    /// `Render`'s rig: seven effects (slot +4 `RENDER_SLOT`) and parameters, `pTextures` with
    /// two textures, the utility buffers made (+0xb8 set), slot +0x1c `ACTIVE_SLOT`.
    fn render_rig(passes: u32, flag: u8) -> (Engine, u32, Vec<u32>) {
        let mut e = rig();
        e.set_global(GRID_SHIFT_01200050, passes);
        e.set_global(FLAG_01200058, flag);
        e.set_global(INVERSE_SIZE_0120004C, 0.25f32);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x1c, ACTIVE_SLOT);
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this, vtable);
        e.mem.set_u8(this + 0xb8, 1);
        e.mem.set_f32(this + 0xb0, 1.5);
        let effect_vtable = e.mem.alloc(8);
        e.mem.set_u32(effect_vtable + 4, RENDER_SLOT);
        let effects_list = e.mem.alloc(0x1c);
        let params = e.mem.alloc(0x1c);
        let mut effects = vec![];
        for index in 0..7u32 {
            let effect = e.mem.alloc(0x100) + 0x6c;
            e.mem.set_u32(effect, effect_vtable);
            e.mem.set_u32(effects_list + 4 * index, effect);
            e.mem.set_u32(params + 4 * index, 0x700 + index);
            effects.push(effect);
        }
        e.mem.set_u32(this + 0xc, effects_list);
        e.mem.set_u32(this + 0x1c, params);
        let textures = e.mem.alloc(8);
        e.mem.set_u32(textures, 0x800);
        e.mem.set_u32(textures + 4, 0x804);
        e.mem.set_u32(this + 0x2c, textures);
        e.register(ACTIVE_SLOT, |_, _| 0x2au32.into_ret());
        e.register(TEXTURE_GET_TEXTURE_00BA3780, |_, _| 1u32.into_ret());
        for target in [
            RENDER_SLOT,
            TEXTURE_BORROW_00BA3840,
            TEXTURE_RETURN_RENDERED_00BA39A0,
            TEXTURE_DESTRUCT_00BA3A00,
            TEXTURE_ASSIGN_00BA3940,
            SET_PIXEL_CONSTANT_00B8A790,
            SET_TEXTURE_00BA3CB0,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        (e, this, effects)
    }

    #[test]
    fn render_runs_the_passes() {
        let (mut e, this, effects) = render_rig(2, 1);
        e.mem.set_u8(this + 5, 1);
        e.call(0x00ba_2760, &args![this, 0x11u32, 0x22u32, 0x33u32, 0u32]);
        assert_eq!(calls(&e, ACTIVE_SLOT), vec![vec![this, 0x33]]);
        assert_eq!(e.mem.u8(this + 5), 0x2a);
        let borrows = calls(&e, TEXTURE_BORROW_00BA3840);
        assert_eq!(borrows.len(), 3);
        assert!(borrows.iter().all(|a| a[1] == 7));
        let (a, c, b) = (borrows[0][0], borrows[1][0], borrows[2][0]);
        let order: Vec<u32> = calls(&e, RENDER_SLOT).iter().map(|w| w[0]).collect();
        assert_eq!(
            order,
            vec![
                effects[0], effects[3], effects[1], effects[1], effects[4], effects[2], effects[2],
                effects[5],
            ]
        );
        assert!(calls(&e, RENDER_SLOT)
            .iter()
            .all(|w| w[1] == 0x11 && w[2] == 0x22 && w[4] == 1));
        let constants = calls(&e, SET_PIXEL_CONSTANT_00B8A790);
        let f = |v: f32| v.to_bits();
        assert_eq!(constants[0], vec![0x700, 0, f(1.5), 0, 0, 0]);
        assert_eq!(constants[1], vec![0x701, 1, f(0.03125), f(4.0), 0, 0]);
        assert_eq!(constants[2], vec![0x701, 1, f(0.09375), f(4.0), 0, 0]);
        assert_eq!(constants[4], vec![0x702, 1, f(0.09375), f(4.0), 0, 0]);
        assert_eq!(constants[5], vec![0x705, 2, f(0.25), f(4.0), 0, 0]);
        assert_eq!(e.mem.f32(this + 0xb4), 0.09375);
        let textures = calls(&e, SET_TEXTURE_00BA3CB0);
        assert_eq!(textures[0][1..4], [0, a, 0]);
        assert_eq!(textures[3][1..4], [0, c, 0]);
        // The last pass of effect 2 starts from `pTextures[1]`.
        let last = &textures[textures.len() - 5];
        assert_eq!(last[..4], [effects[2], 0, 0x804, 0]);
        assert_eq!(textures[textures.len() - 2][..3], [effects[5], 0, 0x800]);
        // Swaps after effects 3, 1, 1, 4 and the first pass of 2.
        assert_eq!(calls(&e, TEXTURE_ASSIGN_00BA3940).len(), 3 * 5);
        assert_eq!(calls(&e, TEXTURE_RETURN_RENDERED_00BA39A0).len(), 3);
        let destroyed: Vec<u32> = calls(&e, TEXTURE_DESTRUCT_00BA3A00)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(destroyed[1..], [c, b, a]);
    }

    #[test]
    fn render_without_the_flag_uses_effect_6() {
        let (mut e, this, effects) = render_rig(0, 0);
        e.call(0x00ba_2760, &args![this, 0u32, 0u32, 0u32, 0u32]);
        let order: Vec<u32> = calls(&e, RENDER_SLOT).iter().map(|w| w[0]).collect();
        assert_eq!(order, vec![effects[0], effects[3], effects[4], effects[6]]);
        assert!(calls(&e, ACTIVE_SLOT).is_empty());
        assert_eq!(calls(&e, SET_PIXEL_CONSTANT_00B8A790).len(), 1);
    }
}
