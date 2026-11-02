//! `fallout shared/run_004e0540` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A run of `fallout shared` code the engine map names none of: the screen
//! splatter's set-up and tear-down (`004e0540` passes `ScreenSplatter.cpp`
//! to its scope guard), and the water system's constructor, destructor and
//! per-frame update (`004e1bc0` passes `TESWater.cpp`; the constructor makes
//! the `"WATER"` object), with the small setters and globals they use.

#[allow(unused_imports)]
use crate::prelude::*;

/// The scope guard's constructor (`00404eb0`, `thiscall(value, mode, source
/// file, line)`) and destructor (`00404ee0`).
const SCOPE_GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\ScreenSplatter.cpp"`.
const SCREEN_SPLATTER_FILE: u32 = 0x0102_2ab8;
/// `"D:\_Fallout3\...\Fallout Shared\TESWater.cpp"`.
const TES_WATER_FILE: u32 = 0x0102_301c;
/// `"WATER"`.
const WATER_NAME: u32 = 0x0102_3014;

/// Returns the address of a setting's value (`00408d60`, `thiscall`).
const SETTING_VALUE: u32 = 0x0040_8d60;
/// `NiPointer<T>` get (`00559450`, `thiscall`): the pointer it holds.
const POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<T>` assign (`0066b0d0`, `thiscall(object)`).
const POINTER_SET: u32 = 0x0066_b0d0;
/// The game's allocators: `00aa13e0` (`cdecl(size)`) and `operator new`
/// (`00401000`, `cdecl(size)`).
const ALLOCATE: u32 = 0x00aa_13e0;
const OPERATOR_NEW: u32 = 0x0040_1000;
/// A three-float constructor (`00416870`, `thiscall(x, y, z)`).
const POINT_CONSTRUCT: u32 = 0x0041_6870;
/// A four-float constructor (`00414430`, `thiscall(r, g, b, a)`).
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
/// The 12-byte value constructor `004e0540` passes to `00a59c60`
/// (`0043d410`, `thiscall(float, word, word)`).
const VALUE_CONSTRUCT: u32 = 0x0043_d410;
/// `_ftol2_sse` (`00ec62c0`): truncates ST0 (here an `f64` argument).
const FTOL: u32 = 0x00ec_62c0;
/// An empty `thiscall` body (`00483710`).
const EMPTY_BODY: u32 = 0x0048_3710;

/// The screen splatter's enable setting (`011c784c`) and its copy
/// (`011c7834`).
const SPLATTER_SETTING: u32 = 0x011c_784c;
const SPLATTER_ENABLED: u32 = 0x011c_7834;
/// The splatter's camera (`011c7840`) and second object (`011c786c`)
/// pointers, and the three texture pointers (`011c7860`, `011c7864`,
/// `011c7868`).
const SPLATTER_CAMERA: u32 = 0x011c_7840;
const SPLATTER_OBJECT: u32 = 0x011c_786c;
const SPLATTER_TEXTURES: u32 = 0x011c_7860;
/// A word the set-up clears (`011c7828`).
const SPLATTER_WORD: u32 = 0x011c_7828;
/// The two floats the set-up copies the frustum's right and top into.
const SPLATTER_FRUSTUM_RIGHT: u32 = 0x0118_9550;
const SPLATTER_FRUSTUM_TOP: u32 = 0x0118_9554;
/// The world point the camera looks at (`011f426c`) and its up vector
/// (`011a9478`).
const LOOK_AT_POINT: u32 = 0x011f_426c;
const LOOK_AT_UP: u32 = 0x011a_9478;
/// The three texture names (`011ce284`, `011ce918`, `011cede8`) and the byte
/// that enables the third (`011f941e`).
const TEXTURE_NAME_FIRST: u32 = 0x011c_e284;
const TEXTURE_NAME_SECOND: u32 = 0x011c_e918;
const TEXTURE_NAME_THIRD: u32 = 0x011c_ede8;
const THIRD_TEXTURE_ENABLED: u32 = 0x011f_941e;
/// The pending count `004e1610` flushes (`011c7870`).
const PENDING_COUNT: u32 = 0x011c_7870;

/// Float constants: 1.00001 (`01022960`), 0.5 (`01016248`), -0.5
/// (`0102295c`), 10000.0 (`01022958`), 0.1 (`0101e2bc`); doubles -0.5
/// (`01011598`), 0.5 (`01011588`), 100.0 (`01017a40`), 0.0 (`01012060`).
const CAMERA_Z: u32 = 0x0102_2960;
const HALF: u32 = 0x0101_6248;
const MINUS_HALF: u32 = 0x0102_295c;
const FAR_PLANE: u32 = 0x0102_2958;
const TENTH: u32 = 0x0101_e2bc;
const MINUS_HALF_DOUBLE: u32 = 0x0101_1598;
const HALF_DOUBLE: u32 = 0x0101_1588;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
const ZERO_DOUBLE: u32 = 0x0101_2060;

/// The water system's globals.
const WATER_SETTING_A: u32 = 0x011c_7bbc;
const WATER_SETTING_A_COPY: u32 = 0x011f_f100;
const WATER_BYTE_86E: u32 = 0x011a_d86e;
const WATER_POINTER_C28: u32 = 0x011c_7c28;
const WATER_OWNER: u32 = 0x011d_ea10;
const WATER_COLORS: u32 = 0x011f_f3b8;
const WATER_WORD_A3C: u32 = 0x011c_7a3c;
const WATER_PAIR: u32 = 0x011f_4980;
const WATER_OBJECT: u32 = 0x011c_7a54;
const WATER_LIST_BEC: u32 = 0x011c_7bec;
const WATER_LIST_AFC: u32 = 0x011c_7afc;
const WATER_WORD_A44: u32 = 0x011c_7a44;
const WATER_WORD_A40: u32 = 0x011c_7a40;
const WATER_BYTE_7B4: u32 = 0x011a_d7b4;
const WATER_UNDERWATER: u32 = 0x011c_7a59;
const WATER_BYTE_1C4: u32 = 0x011f_91c4;
const WATER_SETTING_B20: u32 = 0x011c_7b20;
const WATER_COLOR_TABLE: u32 = 0x011f_a0c0;
/// The update's globals: the cell source (`011dea3c`), the object whose
/// +0x88 pointer gets the under-water state (`011dea0c`), the skip byte
/// (`011c7a5b`), the forced-under-water byte (`011c7a58`), the enable byte
/// (`01189624`), the LOD/placeable flag (`01189625`) and the two bytes that
/// force the placeable update (`011c7a66`, `011c7a67`).
const WATER_CELLS: u32 = 0x011d_ea3c;
const WATER_STATE_OWNER: u32 = 0x011d_ea0c;
const WATER_SKIP: u32 = 0x011c_7a5b;
const WATER_FORCE_UNDER: u32 = 0x011c_7a58;
const WATER_ENABLED: u32 = 0x0118_9624;
const WATER_REFRESH: u32 = 0x0118_9625;
const WATER_FORCE_A: u32 = 0x011c_7a66;
const WATER_FORCE_B: u32 = 0x011c_7a67;
const WATER_SCOPE: u32 = 0x011c_7b78;

// Translated from 004e0540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004e0540 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Screen splatter set-up (by the source file its scope guard is given):
/// copies the enable setting (`011c784c`) to `011c7834`; when it is set,
/// computes the renderer's aspect ratio (`004dee10` / `004dee70` of
/// `0043c4b0`'s result), makes the camera (0x114 bytes, `00a712f0`) into
/// `011c7840`, places it (`00440460` at (0, 0, 1.00001)), gives it two
/// values (`00a59c60`), points it at `011f426c` (`00a701b0`, up
/// `011a9478`), sets its view frustum (`00a6faf0`: left/right = aspect *
/// -0.5/0.5, top 0.5, bottom -0.5, near 1, far 10000, not orthographic),
/// keeps right and top in `01189550`/`01189554`, makes the second object
/// (0xAC bytes, `00a5ecb0(0)`) into `011c786c`, clears `011c7828` and loads
/// the textures (`0063fb70` with the names `00403df0` returns; the third
/// only when `011f941e` is set).
pub fn fn_004e0540(e: &mut Engine) {
    // The frame from EBP-0x80: the guard at EBP-0x10 (+0x70), the point at
    // EBP-0x60 (+0x20), the frustum at EBP-0x48 (+0x38) and the two values
    // at EBP-0x28 (+0x58) and EBP-0x1c (+0x64).
    e.with_stack(0x80, |e, frame| {
        let at = |offset: u32| frame.addr() + offset;
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![at(0x70), 7u32, 1u32, SCREEN_SPLATTER_FILE, 0xb5u32],
        );
        let setting = e.call(SETTING_VALUE, &args![SPLATTER_SETTING]).u32();
        let enabled = e.mem.u8(setting);
        e.set_global(SPLATTER_ENABLED, enabled);
        if enabled != 0 {
            let renderer = e.call(0x0043_c4b0, &args![]).u32();
            let width = e.call(0x004d_ee10, &args![renderer]).u32();
            let height = e.call(0x004d_ee70, &args![renderer]).u32();
            let aspect = x87_store32(x87_div(f64::from(width), f64::from(height)));

            let block = e.call(ALLOCATE, &args![0x114u32]).u32();
            let camera = if block != 0 {
                e.call(0x00a7_12f0, &args![block]).u32()
            } else {
                0
            };
            e.call(POINTER_SET, &args![SPLATTER_CAMERA, camera]);
            let z = x87_quiet_bits(e.global::<u32>(CAMERA_Z));
            let point = e
                .call(POINT_CONSTRUCT, &args![at(0x20), 0u32, 0u32, z])
                .u32();
            let camera = e.call(POINTER_GET, &args![SPLATTER_CAMERA]).u32();
            e.call(0x0044_0460, &args![camera, point]);
            e.call(VALUE_CONSTRUCT, &args![at(0x58), 0u32, 0u32, 0u32]);
            let camera = e.call(POINTER_GET, &args![SPLATTER_CAMERA]).u32();
            e.call(0x00a5_9c60, &args![camera, at(0x58)]);
            let camera = e.call(POINTER_GET, &args![SPLATTER_CAMERA]).u32();
            e.call(0x00a7_01b0, &args![camera, LOOK_AT_POINT, LOOK_AT_UP]);

            // NiFrustum (`00a71b70(false)`): left, right, top, bottom,
            // near, far, orthographic byte.
            let frustum = at(0x38);
            e.call(0x00a7_1b70, &args![frustum, 0u32]);
            e.mem.set_u8(frustum + 0x18, 0);
            let top = x87_quiet_bits(e.global::<u32>(HALF));
            e.mem.set_u32(frustum + 0x08, top);
            let bottom = x87_quiet_bits(e.global::<u32>(MINUS_HALF));
            e.mem.set_u32(frustum + 0x0c, bottom);
            let minus_half: f64 = e.global(MINUS_HALF_DOUBLE);
            let half: f64 = e.global(HALF_DOUBLE);
            let left = x87_store32(x87_mul(x87_load32(aspect), minus_half));
            e.mem.set_u32(frustum, left);
            let right = x87_store32(x87_mul(x87_load32(aspect), half));
            e.mem.set_u32(frustum + 0x04, right);
            let far = x87_quiet_bits(e.global::<u32>(FAR_PLANE));
            e.mem.set_u32(frustum + 0x14, far);
            e.mem.set_u32(frustum + 0x10, 1.0f32.to_bits());
            let camera = e.call(POINTER_GET, &args![SPLATTER_CAMERA]).u32();
            e.call(0x00a6_faf0, &args![camera, frustum]);

            e.call(VALUE_CONSTRUCT, &args![at(0x64), 0u32, 0u32, 0u32]);
            let camera = e.call(POINTER_GET, &args![SPLATTER_CAMERA]).u32();
            e.call(0x00a5_9c60, &args![camera, at(0x64)]);
            let right = x87_quiet_bits(e.mem.u32(frustum + 0x04));
            e.set_global(SPLATTER_FRUSTUM_RIGHT, right);
            let top = x87_quiet_bits(e.mem.u32(frustum + 0x08));
            e.set_global(SPLATTER_FRUSTUM_TOP, top);

            let block = e.call(ALLOCATE, &args![0xacu32]).u32();
            let object = if block != 0 {
                e.call(0x00a5_ecb0, &args![block, 0u32]).u32()
            } else {
                0
            };
            e.call(POINTER_SET, &args![SPLATTER_OBJECT, object]);
            e.set_global(SPLATTER_WORD, 0u32);
            let name = e.call(0x0040_3df0, &args![TEXTURE_NAME_SECOND]).u32();
            e.call(0x0063_fb70, &args![SPLATTER_TEXTURES + 4, name]);
            let name = e.call(0x0040_3df0, &args![TEXTURE_NAME_FIRST]).u32();
            e.call(0x0063_fb70, &args![SPLATTER_TEXTURES, name]);
            if e.global::<u8>(THIRD_TEXTURE_ENABLED) != 0 {
                let name = e.call(0x0040_3df0, &args![TEXTURE_NAME_THIRD]).u32();
                e.call(0x0063_fb70, &args![SPLATTER_TEXTURES + 8, name]);
            }
        }
        e.call(SCOPE_GUARD_DESTRUCT, &args![at(0x70)]);
    });
}

// Translated from 004e0800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Screen splatter tear-down (no parameters; `ECX` is not read): when
/// `011c7834` is set, runs `004def90` on the object `011c786c` holds and
/// clears that pointer and the camera pointer `011c7840`; then clears
/// `011c7834` and the three texture pointers (`011c7860` + 4 * i).
pub fn fn_004e0800(e: &mut Engine) {
    if e.global::<u8>(SPLATTER_ENABLED) != 0 {
        let object = e.call(POINTER_GET, &args![SPLATTER_OBJECT]).u32();
        e.call(0x004d_ef90, &args![object]);
        e.call(POINTER_SET, &args![SPLATTER_OBJECT, 0u32]);
        e.call(POINTER_SET, &args![SPLATTER_CAMERA, 0u32]);
    }
    e.set_global(SPLATTER_ENABLED, 0u8);
    for i in 0..3u32 {
        e.call(POINTER_SET, &args![SPLATTER_TEXTURES + 4 * i, 0u32]);
    }
}

// Translated from 004e1610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flushes the pending count at `011c7870`: when it is not zero, caps it at
/// 20, passes it to `0040fca0` (`cdecl`) and clears it.
pub fn fn_004e1610(e: &mut Engine) {
    if e.global::<u32>(PENDING_COUNT) != 0 {
        if e.global::<u32>(PENDING_COUNT) > 0x14 {
            e.set_global(PENDING_COUNT, 0x14u32);
        }
        let count: u32 = e.global(PENDING_COUNT);
        e.call(0x0040_fca0, &args![count]);
        e.set_global(PENDING_COUNT, 0u32);
    }
}

/// Builds a colour at `slot` (`00414430`) and copies its four words to
/// `target`.
fn store_color(e: &mut Engine, slot: u32, rgba: [u32; 4], target: u32) {
    let color = e
        .call(
            COLOR_CONSTRUCT,
            &args![slot, rgba[0], rgba[1], rgba[2], rgba[3]],
        )
        .u32();
    for i in 0..4 {
        let word = e.mem.u32(color + 4 * i);
        e.set_global(target + 4 * i, word);
    }
}

// Translated from 004e1650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004e1650 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The water system's constructor (unnamed in the engine map; its object
/// is the one `004e1bc0` updates): builds the seven pointers at
/// +0x08..+0x20 (`00633c90(0)`), the members at +0x28 (`006815c0`), +0x3C
/// (`0048f200`), +0x4C/+0x5C (`004ed7a0(0x25)`), +0x6C (`004ed7d0(0x25)`),
/// +0x7C (`004ed800(0x25)`) and +0x8C (`0041a250`), clears the float at
/// +0x24, copies the setting `011c7bbc` to `011ff100`, clears `011ad86e`;
/// when `004e7ff0` returns 0 it makes an object (0xAC bytes, `00a5ecb0(0)`)
/// into `011c7c28` and hands it to slot 0xDC of what `0084e3a0` returns for
/// `011dea10` (with 1). Then sets the three colours at `011ff3b8` ((0,
/// 0.5, 0.5, 1), (0, 0, 0.1, 1), (1, 1, 1, 1)), clears `011c7a3c`, +0x30,
/// +0x38 (float), +0x34 (byte) and the pointer at +0x18, copies the pair at
/// `011f4980` to +0x28, clears +0x00, makes the `"WATER"` object (4 bytes,
/// `00438170`) into `011c7a54`, clears +0x48, +0x98 (float) and +0x9C
/// (byte), and copies the first words of `0043d4d0` of `011c7bec` and
/// `011c7afc` to `011c7a44` and `011c7a40`. Returns `this`.
pub fn fn_004e1650(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for offset in (0x08..=0x20).step_by(4) {
        e.call(0x0063_3c90, &args![base + offset, 0u32]);
    }
    e.call(0x0068_15c0, &args![base + 0x28]);
    e.call(0x0048_f200, &args![base + 0x3c]);
    e.call(0x004e_d7a0, &args![base + 0x4c, 0x25u32]);
    e.call(0x004e_d7a0, &args![base + 0x5c, 0x25u32]);
    e.call(0x004e_d7d0, &args![base + 0x6c, 0x25u32]);
    e.call(0x004e_d800, &args![base + 0x7c, 0x25u32]);
    e.call(0x0041_a250, &args![base + 0x8c]);
    e.mem.set_u32(base + 0x24, 0);
    let setting = e.call(SETTING_VALUE, &args![WATER_SETTING_A]).u32();
    let value = e.mem.u8(setting);
    e.set_global(WATER_SETTING_A_COPY, value);
    e.set_global(WATER_BYTE_86E, 0u8);
    if e.call(0x004e_7ff0, &args![]).u32() == 0 {
        let block = e.call(ALLOCATE, &args![0xacu32]).u32();
        let object = if block != 0 {
            e.call(0x00a5_ecb0, &args![block, 0u32]).u32()
        } else {
            0
        };
        e.call(POINTER_SET, &args![WATER_POINTER_C28, object]);
        let owner: u32 = e.global(WATER_OWNER);
        let target = e.call(0x0084_e3a0, &args![owner]).u32();
        let object = e.call(POINTER_GET, &args![WATER_POINTER_C28]).u32();
        e.vcall(target, 0xdc, &args![object, 1u32]);
    }
    e.with_stack(0x30, |e, colors| {
        let half = x87_quiet_bits(e.global::<u32>(HALF));
        let tenth = x87_quiet_bits(e.global::<u32>(TENTH));
        let one = 1.0f32.to_bits();
        store_color(e, colors.addr(), [0, half, half, one], WATER_COLORS);
        store_color(
            e,
            colors.addr() + 0x10,
            [0, 0, tenth, one],
            WATER_COLORS + 0x10,
        );
        store_color(
            e,
            colors.addr() + 0x20,
            [one, one, one, one],
            WATER_COLORS + 0x20,
        );
    });
    e.set_global(WATER_WORD_A3C, 0u32);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u32(base + 0x38, 0);
    e.mem.set_u8(base + 0x34, 0);
    e.call(POINTER_SET, &args![base + 0x18, 0u32]);
    let first: u32 = e.global(WATER_PAIR);
    e.mem.set_u32(base + 0x28, first);
    let second: u32 = e.global(WATER_PAIR + 4);
    e.mem.set_u32(base + 0x2c, second);
    e.mem.set_u32(base, 0);
    let block = e.call(OPERATOR_NEW, &args![4u32]).u32();
    let water = if block != 0 {
        e.call(0x0043_8170, &args![block, WATER_NAME]).u32()
    } else {
        0
    };
    e.set_global(WATER_OBJECT, water);
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u32(base + 0x98, 0);
    e.mem.set_u8(base + 0x9c, 0);
    let first = e.call(0x0043_d4d0, &args![WATER_LIST_BEC]).u32();
    let word = e.mem.u32(first);
    e.set_global(WATER_WORD_A44, word);
    let first = e.call(0x0043_d4d0, &args![WATER_LIST_AFC]).u32();
    let word = e.mem.u32(first);
    e.set_global(WATER_WORD_A40, word);
    this
}

/// Releases the texture a pointer holds: when it holds one,
/// `BSTextureManager::ReturnRenderedTexture` (`00b6da10`, Xbox PDB) on the
/// manager `004a0ea0` returns; then clears the pointer.
fn return_rendered_texture(e: &mut Engine, pointer: u32) {
    if e.call(POINTER_GET, &args![pointer]).u32() != 0 {
        let texture = e.call(POINTER_GET, &args![pointer]).u32();
        let manager = e.call(0x004a_0ea0, &args![]).u32();
        e.call(0x00b6_da10, &args![manager, texture]);
    }
    e.call(POINTER_SET, &args![pointer, 0u32]);
}

// Translated from 004e19c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004e19c0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The water system's destructor (it undoes `004e1650`): `004e6620(1, 1)`,
/// clears the pointers `011c7be8` and `011c7cb8`, returns the rendered
/// textures of `011c7c2c` and `011c7ad4`, clears `011c7b68`, empties the
/// lists `011c7bcc`, `011c7c30`, `011c7ce4` and `011c7b80` (`004ed900`),
/// deletes the `"WATER"` object (`004b9900(1)`) and clears `011c7a54`, then
/// destroys the members in reverse order (+0x8C `00483710`, +0x7C
/// `004edbd0`, +0x6C `004edad0`, +0x5C/+0x4C `004ed9d0`, +0x3C `004a1a30`,
/// the pointers +0x20..+0x08 `0045cec0`).
pub fn fn_004e19c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x004e_6620, &args![this, 1u32, 1u32]);
    e.call(POINTER_SET, &args![0x011c_7be8u32, 0u32]);
    e.call(POINTER_SET, &args![0x011c_7cb8u32, 0u32]);
    return_rendered_texture(e, 0x011c_7c2c);
    return_rendered_texture(e, 0x011c_7ad4);
    e.call(POINTER_SET, &args![0x011c_7b68u32, 0u32]);
    for list in [0x011c_7bccu32, 0x011c_7c30, 0x011c_7ce4, 0x011c_7b80] {
        e.call(0x004e_d900, &args![list]);
    }
    let water: u32 = e.global(WATER_OBJECT);
    if water != 0 {
        e.call(0x004b_9900, &args![water, 1u32]);
    }
    e.set_global(WATER_OBJECT, 0u32);
    e.call(EMPTY_BODY, &args![base + 0x8c]);
    e.call(0x004e_dbd0, &args![base + 0x7c]);
    e.call(0x004e_dad0, &args![base + 0x6c]);
    e.call(0x004e_d9d0, &args![base + 0x5c]);
    e.call(0x004e_d9d0, &args![base + 0x4c]);
    e.call(0x004a_1a30, &args![base + 0x3c]);
    for offset in (0x08..=0x20).rev().step_by(4) {
        e.call(0x0045_cec0, &args![base + offset]);
    }
}

/// The object `011dea0c` holds at +0x88 ([`fn_004e2190`]).
fn water_state(e: &mut Engine) -> Ptr {
    let owner: u32 = e.global(WATER_STATE_OWNER);
    fn_004e2190(e, Ptr::new(owner))
}

/// `TESObjectCELL::GetWaterHeight` (`005471e0`, Xbox PDB) of the cell
/// `008d6f30` gives for `011dea3c`.
fn water_height(e: &mut Engine) -> f64 {
    let cells: u32 = e.global(WATER_CELLS);
    let cell = e.call(0x008d_6f30, &args![cells]).u32();
    e.call(0x0054_71e0, &args![cell]).f64()
}

// Translated from 004e1bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004e1bc0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The water system's per-frame update (`thiscall(camera, unused)`, `RET 8`;
/// the second word is never read): clears +0x00/+0x04 and stops unless
/// `008d6f30` of `011dea3c` gives something and the skip byte `011c7a5b` is
/// clear (a set skip byte is cleared and the update stops). Then clears
/// `011f91c4` ([`fn_004e20f0`]), stores the camera's position (`0045bb80`)
/// as colour 0x1E ([`fn_004e20c0`], alpha 0) and the camera in
/// `011c7a50`; stops there when `01189624` is clear. Otherwise clears
/// `011ad7b4`, decides whether the camera is under water (`011c7a58`, or
/// its height below the cell's water height) into `011c7a59`/`011ff8c4`
/// (keeping the height in `011ff8a8`) and passes it, the water height
/// (truncated) and the enable byte to the object at `011dea0c` +0x88
/// ([`fn_004e2120`], [`fn_004e2140`], [`fn_004e2100`]); copies the sky's
/// direction (normalized by `004a0c10`) to `011ff398`, a clamped value
/// (`006428b0` * 100 limited to 0..1) to `011ff3a4` and `011ff3b4` (0 when
/// it equals 0.0 or under water), the sky's colour to `011ff3a8`, whether
/// `005f36f0` of `011dea10` is set to `011ff101`; updates the LOD water
/// (`004e3320`) and, unless `0076b610(this + 0x3C)` holds with both
/// `011c7a66`/`011c7a67` clear, the placeable water (`004e21b0`), the
/// displacement simulation (`004e8950`/`004e8a00`/`004e8ec0`, when
/// `004e2160` is set, `004dc060(0)` is at least 2 and `0076b610` is clear)
/// and the sounds (`004e6a80`). Finally clears `011c7a66`/`011c7a67` and
/// sets `011ad7b4`.
pub fn fn_004e1bc0(e: &mut Engine, this: Ptr, camera: u32, _unused_2: u32) {
    let base = this.addr();
    // The frame from EBP-0x48: the colour at EBP-0x44 (+0x04), the guard at
    // EBP-0x28 (+0x20), the direction at EBP-0x24 (+0x24).
    e.with_stack(0x48, |e, frame| {
        let at = |offset: u32| frame.addr() + offset;
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![at(0x20), 0x1du32, 1u32, TES_WATER_FILE, 0x117u32],
        );
        e.mem.set_u32(base, 0);
        e.mem.set_u32(base + 4, 0);
        let cells: u32 = e.global(WATER_CELLS);
        if e.call(0x008d_6f30, &args![cells]).u32() == 0 {
            e.call(SCOPE_GUARD_DESTRUCT, &args![at(0x20)]);
            return;
        }
        if e.global::<u8>(WATER_SKIP) != 0 {
            e.set_global(WATER_SKIP, 0u8);
            e.call(SCOPE_GUARD_DESTRUCT, &args![at(0x20)]);
            return;
        }
        fn_004e20f0(e, 0);
        e.call(EMPTY_BODY, &args![WATER_SCOPE]);
        let position = e.call(0x0045_bb80, &args![camera]).u32();
        let x = x87_quiet_bits(e.mem.u32(position));
        let y = x87_quiet_bits(e.mem.u32(position + 4));
        let z = x87_quiet_bits(e.mem.u32(position + 8));
        let color = e
            .call(COLOR_CONSTRUCT, &args![at(0x04), x, y, z, 0u32])
            .u32();
        let words = [0, 4, 8, 12].map(|i| e.mem.u32(color + i));
        fn_004e20c0(e, 0x1e, words[0], words[1], words[2], words[3]);
        e.set_global(0x011c_7a50, camera);
        if e.global::<u8>(WATER_ENABLED) == 0 {
            e.call(EMPTY_BODY, &args![WATER_SCOPE]);
            e.call(SCOPE_GUARD_DESTRUCT, &args![at(0x20)]);
            return;
        }
        fn_004e20b0(e, 0);
        let cells: u32 = e.global(WATER_CELLS);
        if cells != 0 && e.call(0x008d_6f30, &args![cells]).u32() != 0 {
            let under = if e.global::<u8>(WATER_FORCE_UNDER) != 0 {
                true
            } else {
                let position = e.call(0x0045_bb80, &args![camera]).u32();
                let height = water_height(e);
                // FLD [z]; FCOMPP; TEST AH,5; JP: above water when z >= the
                // height or unordered.
                let z = x87_load32(e.mem.u32(position + 8));
                z < height
            };
            if under {
                e.set_global(WATER_UNDERWATER, 1u8);
                let state = water_state(e);
                fn_004e2120(e, state, 1);
                e.set_global(0x011f_f8c4u32, 1u8);
                let position = e.call(0x0045_bb80, &args![camera]).u32();
                let z = x87_quiet_bits(e.mem.u32(position + 8));
                e.set_global(0x011f_f8a8, z);
            } else {
                e.set_global(WATER_UNDERWATER, 0u8);
                let state = water_state(e);
                fn_004e2120(e, state, 0);
                e.set_global(0x011f_f8c4u32, 0u8);
            }
            let underwater: u8 = e.global(WATER_UNDERWATER);
            e.set_global(0x011f_f104, underwater);
            let height = water_height(e);
            let height = e.call(FTOL, &args![height]).u32();
            let state = water_state(e);
            fn_004e2140(e, state, height);
            let state = water_state(e);
            fn_004e2100(e, state, 1);
        } else {
            let state = water_state(e);
            fn_004e2100(e, state, 0);
        }
        let underwater: u8 = e.global(WATER_UNDERWATER);
        e.set_global(0x011f_f104, underwater);

        let sky = e.call(0x0046_dd00, &args![]).u32();
        let first = e.call(0x0045_cd60, &args![sky]).u32();
        let second = e.call(0x0055_85e0, &args![first]).u32();
        let third = e.call(0x0096_11e0, &args![second]).u32();
        let source = e.call(0x0043_c490, &args![third]).u32();
        let direction = at(0x24);
        for i in [0, 4, 8] {
            let word = e.mem.u32(source + i);
            e.mem.set_u32(direction + i, word);
        }
        e.call(0x004a_0c10, &args![direction]);
        for i in [0, 4, 8] {
            let word = x87_quiet_bits(e.mem.u32(direction + i));
            e.set_global(0x011f_f398 + i, word);
        }

        let sky = e.call(0x0046_dd00, &args![]).u32();
        let first = e.call(0x0045_cd60, &args![sky]).u32();
        let value = e.call(0x0064_28b0, &args![first]).f64();
        let hundred: f64 = e.global(HUNDRED_DOUBLE);
        let scaled = x87_mul(value, hundred);
        let low = fn_004e2080(e, 0.0, scaled);
        let clamped = fn_004e2050(e, 1.0, low);
        e.set_global(0x011f_f3a4, x87_store32(clamped));

        let sky = e.call(0x0046_dd00, &args![]).u32();
        let color = e.call(0x0043_b5d0, &args![sky]).u32();
        for i in [0, 4, 8] {
            let word = x87_quiet_bits(e.mem.u32(color + i));
            e.set_global(0x011f_f3a8 + i, word);
        }
        // FCOMP against 0.0; TEST AH,0x44; JP: copies unless equal.
        let stored = x87_load32(e.global::<u32>(0x011f_f3a4));
        let zero: f64 = e.global(ZERO_DOUBLE);
        if stored == zero {
            e.set_global(0x011f_f3b4, 0u32);
        } else {
            let word = x87_quiet_bits(e.global::<u32>(0x011f_f3a4));
            e.set_global(0x011f_f3b4, word);
        }
        if fn_004e2180(e) != 0 {
            e.set_global(0x011f_f3b4, 0u32);
        }
        let owner: u32 = e.global(WATER_OWNER);
        let flag = e.call(0x005f_36f0, &args![owner]).u32() != 0;
        e.set_global(0x011f_f101, u8::from(flag));
        let refresh = u32::from(e.global::<u8>(WATER_REFRESH));
        e.call(0x004e_3320, &args![this, camera, refresh]);
        let settled = e.call(0x0076_b610, &args![base + 0x3c]).u8() != 0;
        if !settled || e.global::<u8>(WATER_FORCE_B) != 0 || e.global::<u8>(WATER_FORCE_A) != 0 {
            let refresh = u32::from(e.global::<u8>(WATER_REFRESH));
            e.call(0x004e_21b0, &args![this, camera, refresh]);
            e.set_global(WATER_REFRESH, 0u8);
            if fn_004e2160(e) != 0
                && e.call(0x004d_c060, &args![0u32]).i32() >= 2
                && e.call(0x0076_b610, &args![base + 0x3c]).u8() == 0
            {
                if e.call(0x004e_8950, &args![this]).u8() != 0 {
                    e.call(0x004e_8a00, &args![this]);
                }
                e.call(0x004e_8ec0, &args![this]);
            }
            e.call(0x004e_6a80, &args![this]);
        }
        e.set_global(WATER_FORCE_A, 0u8);
        e.set_global(WATER_FORCE_B, 0u8);
        fn_004e20b0(e, 1);
        e.call(EMPTY_BODY, &args![WATER_SCOPE]);
        e.call(SCOPE_GUARD_DESTRUCT, &args![at(0x20)]);
    });
}

// Translated from 004e2050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl(float a, double b)`, result in ST0: `b` when `a >= b` or the two
/// are unordered, otherwise `a` (the smaller of the two).
pub fn fn_004e2050(_e: &mut Engine, a: f32, b: f64) -> f64 {
    let first = x87_load32(a.to_bits());
    if !(first < b) {
        x87_load64(b.to_bits())
    } else {
        first
    }
}

// Translated from 004e2080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl(float a, double b)`, result in ST0: `b` when `a <= b` or the two
/// are unordered, otherwise `a` (the larger of the two).
pub fn fn_004e2080(_e: &mut Engine, a: f32, b: f64) -> f64 {
    let first = x87_load32(a.to_bits());
    if !(first > b) {
        x87_load64(b.to_bits())
    } else {
        first
    }
}

// Translated from 004e20b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in `011ad7b4` (`cdecl`).
pub fn fn_004e20b0(e: &mut Engine, value: u8) {
    e.set_global(WATER_BYTE_7B4, value);
}

// Translated from 004e20c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores four words as entry `index` of the 16-byte table at `011fa0c0`
/// (`cdecl(index, r, g, b, a)`).
pub fn fn_004e20c0(e: &mut Engine, index: u32, red: u32, green: u32, blue: u32, alpha: u32) {
    let entry = (index << 4).wrapping_add(WATER_COLOR_TABLE);
    e.mem.set_u32(entry, red);
    e.mem.set_u32(entry.wrapping_add(4), green);
    e.mem.set_u32(entry.wrapping_add(8), blue);
    e.mem.set_u32(entry.wrapping_add(12), alpha);
}

// Translated from 004e20f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in `011f91c4` (`cdecl`).
pub fn fn_004e20f0(e: &mut Engine, value: u8) {
    e.set_global(WATER_BYTE_1C4, value);
}

// Translated from 004e2100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at +0x166 of `this`.
pub fn fn_004e2100(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x166, value);
}

// Translated from 004e2120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at +0x167 of `this`.
pub fn fn_004e2120(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x167, value);
}

// Translated from 004e2140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word at +0x168 of `this`.
pub fn fn_004e2140(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x168, value);
}

// Translated from 004e2160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte value of the setting at `011c7b20` (`00408d60`).
pub fn fn_004e2160(e: &mut Engine) -> u8 {
    let value = e.call(SETTING_VALUE, &args![WATER_SETTING_B20]).u32();
    e.mem.u8(value)
}

// Translated from 004e2180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The under-water byte `011c7a59`.
pub fn fn_004e2180(e: &mut Engine) -> u8 {
    e.global(WATER_UNDERWATER)
}

// Translated from 004e2190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer held at +0x88 of `this` (`00559450` on it).
pub fn fn_004e2190(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(POINTER_GET, &args![this.addr() + 0x88]).ptr()
}

// Translated from 004e0870 (decompiled, FalloutNV.exe 1.4.0.525)
/// With the splatter enabled (`011c7834`), hands the object `011c786c` holds (`00559450`)
/// to `004def90` and clears `011c7828`.
pub fn fn_004e0870(e: &mut Engine) {
    if e.global::<u8>(SPLATTER_ENABLED) == 0 {
        return;
    }
    let object = e.call(POINTER_GET, &args![SPLATTER_OBJECT]).u32();
    e.call(0x004d_ef90, &args![object]);
    e.set_global::<u32>(SPLATTER_WORD, 0);
}

/// `00403e20` (`thiscall`): the address of a setting's value.
const SETTING_ADDRESS: u32 = 0x0040_3e20;
/// `RandomFloat` (Xbox PDB, `00476b70`, cdecl: low, high).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `004dfea0` (no arguments): a random float in `ST0`.
const RANDOM_UNIT: u32 = 0x004d_fea0;
/// The two-float constructor (`00452dc0`, `thiscall(x, y)`).
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;

/// An `operator new` block of `count` entries of `size` bytes (a size of `0xFFFFFFFF` when the
/// multiplication overflows) with each entry built by `006815c0` (the vector constructor
/// `00401050`), or null when the allocation fails.
fn new_splatter_array(e: &mut Engine, count: u32, size: u32) -> u32 {
    let bytes = count.checked_mul(size).unwrap_or(u32::MAX);
    let block = e.call(OPERATOR_NEW, &args![bytes]).u32();
    if block == 0 {
        return 0;
    }
    e.call(0x0040_1050, &args![block, size, count, 0x0068_15c0u32]);
    block
}

/// The value at a setting's address (`00403e20`), as `FLD`/`FSTP` copy it.
fn setting_float(e: &mut Engine, setting: u32) -> u32 {
    let address = e.call(SETTING_ADDRESS, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(address))
}

/// `RandomFloat` between the values of the settings `low` and `high` (the high one is read
/// first, as the exe pushes it first), stored as a float.
fn random_between(e: &mut Engine, low: u32, high: u32) -> u32 {
    let high = setting_float(e, high);
    let low = setting_float(e, low);
    x87_store32(e.call(RANDOM_FLOAT, &args![low, high]).f64())
}

/// 0.5 (the double at `01011588`) when `004dfea0` is not above the double at `01012060`
/// (`TEST AH,0x41`: below, equal or unordered), else 0.0, stored as a float.
fn random_half(e: &mut Engine) -> u32 {
    let value = e.call(RANDOM_UNIT, &args![]).f64();
    let zero = e.global::<f64>(ZERO_DOUBLE);
    if value > zero {
        0
    } else {
        x87_store32(x87_load64(e.global::<u64>(HALF_DOUBLE)))
    }
}

/// One splatter corner: the point (`x`, `y`, `z` as pushed) turned by the rotation at `matrix`
/// (`004b4500`) and moved by `center` (`00439e90`), its three words copied to `target`. The
/// temporaries are the exe's (corner `k` at EBP-0xE0, -0xEC, -0xF8, each 0x24 lower).
#[allow(clippy::too_many_arguments)]
fn splatter_corner(
    e: &mut Engine,
    ebp: u32,
    k: u32,
    matrix: u32,
    center: u32,
    point: [u32; 3],
    target: u32,
) {
    let local = ebp - 0xe0 - 0x24 * k;
    let turned = ebp - 0xec - 0x24 * k;
    let moved = ebp - 0xf8 - 0x24 * k;
    let made = e
        .call(POINT_CONSTRUCT, &args![local, point[0], point[1], point[2]])
        .u32();
    let turned = e.call(0x004b_4500, &args![matrix, turned, made]).u32();
    let moved = e.call(0x0043_9e90, &args![turned, moved, center]).u32();
    for word in 0..3 {
        let value = e.mem.u32(moved + 4 * word);
        e.mem.set_u32(target + 4 * word, value);
    }
}

/// The four corners of one quad (`a` the half size, `depth` the third word): (-a, a), (a, a),
/// (-a, -a), (a, -a), at `base` .. `base + 3` of the 12-byte `array`.
#[allow(clippy::too_many_arguments)]
fn splatter_quad(
    e: &mut Engine,
    ebp: u32,
    first: u32,
    matrix: u32,
    center: u32,
    half: u32,
    depth: u32,
    array: u32,
    base: u32,
) {
    let plus = x87_quiet_bits(half);
    let minus = plus ^ 0x8000_0000;
    let depth = x87_quiet_bits(depth);
    for (k, x, y) in [
        (0u32, minus, plus),
        (1, plus, plus),
        (2, minus, minus),
        (3, plus, minus),
    ] {
        let target = base.wrapping_add(k).wrapping_mul(0xc).wrapping_add(array);
        splatter_corner(e, ebp, first + k, matrix, center, [x, y, depth], target);
    }
}

/// The six indices of a quad's two triangles (`base`, +2, +1, +1, +2, +3) at entry `quad`*6
/// of the 16-bit `indices`.
fn splatter_triangles(e: &mut Engine, indices: u32, quad: u32, base: u32) {
    let at = indices.wrapping_add(quad.wrapping_mul(6).wrapping_mul(2));
    for (offset, step) in [(0u32, 0u32), (2, 2), (4, 1), (6, 1), (8, 2), (0xa, 3)] {
        e.mem.set_u16(at + offset, base.wrapping_add(step) as u16);
    }
}

// Translated from 004e08a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004e08a0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Builds the screen splatter's geometry for `count` splats (cdecl) and returns its node
/// (`00aa13e0(0xAC)`, constructor `00a5ecb0` with 0; null when the allocation fails): for
/// each splat a quad of 4 corners (12 bytes each), texture coordinates (8 bytes each) and 6
/// 16-bit indices, and with the third texture enabled (`011f941e`) a second set. Each splat's
/// centre is two random values (`004dfea0`) times the frustum floats `01189550`/`01189554`
/// (`00416870` with 0.0); its half size, depth and alternative depth are `RandomFloat`
/// between the settings `011cf9a4`/`011cfaa0`, `011cfae0`/`011cf980` and
/// `011ceca0`/`011cf920`, the alternative used when `004dff00` passes for the setting
/// `011cefe4`; its rotation is the local matrix `004dfdd0` fills; the texture offset is 0.5
/// or 0.0 per axis by two more random values (`00452dc0`), added (`004dfe30`) to (0.5, 0.5),
/// (0, 0.5), (0.5, 0), (0, 0); the second set's half size is the setting `011cf2c4` and its
/// coordinates (1, 1), (0, 1), (1, 0), (0, 0). Each set becomes a `NiTriShape` (`00a74410`,
/// `00aa13e0(0xC4)`) with a texturing property (`00aa13e0(0x70)`, `00b81200`) holding the
/// textures `011c7864` and `011c7860` (the second set: `011c7868`) through `004dfe80`,
/// attached (`00439410`), prepared (`00b57e30` with 0, 0) and added to the node (vtable slot
/// 0xDC with 1).
pub fn fn_004e08a0(e: &mut Engine, count: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0xacu32]).u32();
    let node = if block == 0 {
        0
    } else {
        e.call(0x00a5_ecb0, &args![block, 0u32]).u32()
    };
    let vertices = new_splatter_array(e, count << 2, 0xc);
    let coordinates = new_splatter_array(e, count << 2, 8);
    let index_bytes = count.wrapping_mul(6).checked_mul(2).unwrap_or(u32::MAX);
    let indices = e.call(OPERATOR_NEW, &args![index_bytes]).u32();
    let mut second_vertices = 0;
    let mut second_coordinates = 0;
    let mut second_indices = 0;
    if e.global::<u8>(THIRD_TEXTURE_ENABLED) != 0 {
        second_vertices = new_splatter_array(e, count << 2, 0xc);
        second_coordinates = new_splatter_array(e, count << 2, 8);
        let index_bytes = count.wrapping_mul(6).checked_mul(2).unwrap_or(u32::MAX);
        second_indices = e.call(OPERATOR_NEW, &args![index_bytes]).u32();
    }
    // The frame below EBP: the rotation at EBP-0x7C, the centre at EBP-0x88, the texture
    // offset at EBP-0x50 and the temporaries below them.
    e.with_stack(0x2b0, |e, frame| {
        let ebp = frame.addr() + 0x2b0;
        let matrix = ebp - 0x7c;
        let center = ebp - 0x88;
        let offset = ebp - 0x50;
        let mut splat = 0u32;
        while splat < count {
            let random = e.call(RANDOM_UNIT, &args![]).f64();
            let right = x87_m32(e.global::<u32>(SPLATTER_FRUSTUM_RIGHT));
            let x = x87_store32(x87_mul(random, right));
            let random = e.call(RANDOM_UNIT, &args![]).f64();
            let top = x87_m32(e.global::<u32>(SPLATTER_FRUSTUM_TOP));
            let y = x87_store32(x87_mul(random, top));
            e.call(
                POINT_CONSTRUCT,
                &args![center, x87_quiet_bits(x), x87_quiet_bits(y), 0u32],
            );
            let half = random_between(e, 0x011c_f9a4, 0x011c_faa0);
            let mut depth = random_between(e, 0x011c_fae0, 0x011c_f980);
            let other_depth = random_between(e, 0x011c_eca0, 0x011c_f920);
            e.call(0x0068_15c0, &args![matrix]);
            e.call(0x004d_fdd0, &args![matrix]);
            let chance = setting_float(e, 0x011c_efe4);
            if e.call(0x004d_ff00, &args![chance]).bool() {
                depth = x87_quiet_bits(other_depth);
            }
            let u = random_half(e);
            let v = random_half(e);
            e.call(
                POINT2_CONSTRUCT,
                &args![offset, x87_quiet_bits(u), x87_quiet_bits(v)],
            );
            let second_half = setting_float(e, 0x011c_f2c4);
            let base = splat << 2;
            splatter_quad(e, ebp, 0, matrix, center, half, depth, vertices, base);
            if e.global::<u8>(THIRD_TEXTURE_ENABLED) != 0 {
                splatter_quad(
                    e,
                    ebp,
                    4,
                    matrix,
                    center,
                    second_half,
                    depth,
                    second_vertices,
                    base,
                );
            }
            let half_value = x87_quiet_bits(e.global::<u32>(HALF));
            for (k, (cu, cv)) in [
                (half_value, half_value),
                (0, half_value),
                (half_value, 0),
                (0, 0),
            ]
            .into_iter()
            .enumerate()
            {
                let k = k as u32;
                let local = ebp - 0x1fc - 0x10 * k;
                let sum = ebp - 0x204 - 0x10 * k;
                let made = e.call(POINT2_CONSTRUCT, &args![local, cu, cv]).u32();
                let moved = e.call(0x004d_fe30, &args![made, sum, offset]).u32();
                let target = coordinates
                    .wrapping_add(base.wrapping_mul(8))
                    .wrapping_add(8 * k);
                let first = e.mem.u32(moved);
                let second = e.mem.u32(moved + 4);
                e.mem.set_u32(target, first);
                e.mem.set_u32(target + 4, second);
            }
            if e.global::<u8>(THIRD_TEXTURE_ENABLED) != 0 {
                let one = 0x3f80_0000u32;
                for (k, (cu, cv)) in [(one, one), (0, one), (one, 0), (0, 0)]
                    .into_iter()
                    .enumerate()
                {
                    let k = k as u32;
                    let local = ebp - 0x23c - 8 * k;
                    let made = e.call(POINT2_CONSTRUCT, &args![local, cu, cv]).u32();
                    let target = second_coordinates
                        .wrapping_add(base.wrapping_mul(8))
                        .wrapping_add(8 * k);
                    let first = e.mem.u32(made);
                    let second = e.mem.u32(made + 4);
                    e.mem.set_u32(target, first);
                    e.mem.set_u32(target + 4, second);
                }
            }
            splatter_triangles(e, indices, splat, base);
            if e.global::<u8>(THIRD_TEXTURE_ENABLED) != 0 {
                splatter_triangles(e, second_indices, splat, base);
            }
            splat += 1;
        }
    });
    add_splatter_shape(
        e,
        node,
        count,
        [vertices, coordinates, indices],
        &[SPLATTER_TEXTURES + 4, SPLATTER_TEXTURES],
    );
    if e.global::<u8>(THIRD_TEXTURE_ENABLED) != 0 {
        add_splatter_shape(
            e,
            node,
            count,
            [second_vertices, second_coordinates, second_indices],
            &[SPLATTER_TEXTURES + 8],
        );
    }
    node
}

/// The end of `004e08a0` for one set: the `NiTriShape` (`00aa13e0(0xC4)`, `00a74410` with
/// `count`*4 vertices, no normals or colours, one texture set, `count`*2 triangles; null when
/// the allocation fails), the texturing property (`00aa13e0(0x70)`, `00b81200`) with the
/// textures the pointers in `textures` hold in slots 0, 1, ... (`004dfe80`), attached
/// (`00439410`), prepared (`00b57e30` with 0, 0) and added to the node (vtable slot 0xDC
/// with 1).
fn add_splatter_shape(e: &mut Engine, node: u32, count: u32, set: [u32; 3], textures: &[u32]) {
    let [vertices, coordinates, indices] = set;
    let block = e.call(ALLOCATE, &args![0xc4u32]).u32();
    let shape = if block == 0 {
        0
    } else {
        e.call(
            0x00a7_4410,
            &args![
                block,
                count << 2,
                vertices,
                0u32,
                0u32,
                coordinates,
                1u32,
                0u32,
                count << 1,
                indices
            ],
        )
        .u32()
    };
    let block = e.call(ALLOCATE, &args![0x70u32]).u32();
    let property = if block == 0 {
        0
    } else {
        e.call(0x00b8_1200, &args![block]).u32()
    };
    for (slot, &holder) in textures.iter().enumerate() {
        let texture = e.call(POINTER_GET, &args![holder]).u32();
        e.call(0x004d_fe80, &args![property, slot as u32, texture]);
    }
    e.call(0x0043_9410, &args![shape, property]);
    e.call(0x00b5_7e30, &args![shape, 0u32, 0u32]);
    e.vcall(node, 0xdc, &args![shape, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004e_0540, fn_004e0540()),
        entry!(0x004e_0800, fn_004e0800()),
        entry!(0x004e_1610, fn_004e1610()),
        entry!(0x004e_1650, fn_004e1650(Ptr) -> Ptr),
        entry!(0x004e_19c0, fn_004e19c0(Ptr)),
        entry!(0x004e_1bc0, fn_004e1bc0(Ptr, u32, u32)),
        entry!(0x004e_2050, fn_004e2050(f32, f64) -> f64),
        entry!(0x004e_2080, fn_004e2080(f32, f64) -> f64),
        entry!(0x004e_20b0, fn_004e20b0(u8)),
        entry!(0x004e_20c0, fn_004e20c0(u32, u32, u32, u32, u32)),
        entry!(0x004e_20f0, fn_004e20f0(u8)),
        entry!(0x004e_2100, fn_004e2100(Ptr, u8)),
        entry!(0x004e_2120, fn_004e2120(Ptr, u8)),
        entry!(0x004e_2140, fn_004e2140(Ptr, u32)),
        entry!(0x004e_2160, fn_004e2160() -> u8),
        entry!(0x004e_2180, fn_004e2180() -> u8),
        entry!(0x004e_2190, fn_004e2190(Ptr) -> Ptr),
        entry!(0x004e_0870, fn_004e0870()),
        entry!(0x004e_08a0, fn_004e08a0(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGES: [u32; 17] = [
        0x0101_1000,
        0x0101_2000,
        0x0101_6000,
        0x0101_7000,
        0x0101_e000,
        0x0102_2000,
        0x0118_9000,
        0x011a_9000,
        0x011a_d000,
        0x011c_7000,
        0x011c_e000,
        0x011d_e000,
        0x011f_4000,
        0x011f_9000,
        0x011f_a000,
        0x011f_f000,
        0x011f_8000,
    ];

    /// Every page of globals the unit touches mapped, the constants set,
    /// and every callee in `stubs` doubled to return 0.
    fn engine(stubs: &[u32]) -> Engine {
        let mut e = Engine::new();
        for page in PAGES {
            e.map(page, 0x1000);
        }
        e.set_global(CAMERA_Z, 1.00001f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(MINUS_HALF, -0.5f32);
        e.set_global(FAR_PLANE, 10000.0f32);
        e.set_global(TENTH, 0.1f32);
        e.set_global(MINUS_HALF_DOUBLE, -0.5f64);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        for &addr in stubs {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A setting whose value byte is `value`; `00408d60` returns its
    /// address.
    fn setting(e: &mut Engine, value: u8) {
        let cell = e.mem.alloc(4);
        e.mem.set_u8(cell, value);
        e.register_double(SETTING_VALUE, move |_, _| cell.into_ret());
    }

    /// `NiPointer` doubles: get reads the word, set writes it.
    fn pointers(e: &mut Engine) {
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
    }

    #[test]
    fn at_004e2050_and_004e2080_pick_the_smaller_and_larger() {
        let mut e = Engine::new();
        let min = |e: &mut Engine, a: f32, b: f64| e.call(0x004e_2050, &args![a, b]).f64();
        let max = |e: &mut Engine, a: f32, b: f64| e.call(0x004e_2080, &args![a, b]).f64();
        assert_eq!(min(&mut e, 1.0, 0.25), 0.25);
        assert_eq!(min(&mut e, 0.5, 2.0), 0.5);
        assert_eq!(min(&mut e, 2.0, 2.0), 2.0);
        assert!(min(&mut e, 1.0, f64::NAN).is_nan());
        assert!(min(&mut e, f32::NAN, 1.0) == 1.0);
        assert_eq!(max(&mut e, 0.0, 0.25), 0.25);
        assert_eq!(max(&mut e, 0.5, -2.0), 0.5);
        assert!(max(&mut e, 0.0, f64::NAN).is_nan());
        assert!(max(&mut e, f32::NAN, -1.0) == -1.0);
        // A signalling NaN `b` comes back quiet.
        let signalling = f64::from_bits(0x7ff0_0000_0000_0001);
        let back = max(&mut e, 0.0, signalling).to_bits();
        assert_eq!(back, 0x7ff8_0000_0000_0001);
    }

    #[test]
    fn small_setters_store_their_values() {
        let mut e = engine(&[]);
        e.call(0x004e_20b0, &args![7u32]);
        assert_eq!(e.global::<u8>(WATER_BYTE_7B4), 7);
        e.call(0x004e_20f0, &args![3u32]);
        assert_eq!(e.global::<u8>(WATER_BYTE_1C4), 3);
        e.call(0x004e_20c0, &args![2u32, 10u32, 11u32, 12u32, 13u32]);
        for (i, v) in [10u32, 11, 12, 13].iter().enumerate() {
            assert_eq!(e.global::<u32>(WATER_COLOR_TABLE + 0x20 + 4 * i as u32), *v);
        }
        let this = e.mem.alloc(0x170);
        e.call(0x004e_2100, &args![this, 5u32]);
        e.call(0x004e_2120, &args![this, 6u32]);
        e.call(0x004e_2140, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u8(this + 0x166), 5);
        assert_eq!(e.mem.u8(this + 0x167), 6);
        assert_eq!(e.mem.u32(this + 0x168), 0x1234);
        e.set_global(WATER_UNDERWATER, 9u8);
        assert_eq!(e.call(0x004e_2180, &args![]).u8(), 9);
    }

    #[test]
    fn at_004e2160_and_004e2190_read_through_their_callees() {
        let mut e = engine(&[]);
        setting(&mut e, 4);
        pointers(&mut e);
        assert_eq!(e.call(0x004e_2160, &args![]).u8(), 4);
        assert_eq!(calls(&e, SETTING_VALUE), vec![vec![WATER_SETTING_B20]]);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x88, 0x5150);
        assert_eq!(e.call(0x004e_2190, &args![this]).u32(), 0x5150);
        assert_eq!(calls(&e, POINTER_GET), vec![vec![this + 0x88]]);
    }

    #[test]
    fn at_004e1610_flushes_a_capped_count() {
        let mut e = engine(&[0x0040_fca0]);
        e.call(0x004e_1610, &args![]);
        assert!(calls(&e, 0x0040_fca0).is_empty());
        e.set_global(PENDING_COUNT, 5u32);
        e.call(0x004e_1610, &args![]);
        e.set_global(PENDING_COUNT, 30u32);
        e.call(0x004e_1610, &args![]);
        assert_eq!(calls(&e, 0x0040_fca0), vec![vec![5], vec![20]]);
        assert_eq!(e.global::<u32>(PENDING_COUNT), 0);
    }

    #[test]
    fn at_004e0800_tears_the_splatter_down() {
        let mut e = engine(&[0x004d_ef90]);
        pointers(&mut e);
        e.set_global(SPLATTER_OBJECT, 0x7000u32);
        e.set_global(SPLATTER_CAMERA, 0x7100u32);
        e.set_global(SPLATTER_TEXTURES, 1u32);
        e.call(0x004e_0800, &args![]);
        // Disabled: the object and camera stay.
        assert!(calls(&e, 0x004d_ef90).is_empty());
        assert_eq!(e.global::<u32>(SPLATTER_OBJECT), 0x7000);
        assert_eq!(e.global::<u32>(SPLATTER_TEXTURES), 0);
        e.set_global(SPLATTER_ENABLED, 1u8);
        e.call(0x004e_0800, &args![]);
        assert_eq!(calls(&e, 0x004d_ef90), vec![vec![0x7000]]);
        assert_eq!(e.global::<u32>(SPLATTER_OBJECT), 0);
        assert_eq!(e.global::<u32>(SPLATTER_CAMERA), 0);
        assert_eq!(e.global::<u8>(SPLATTER_ENABLED), 0);
        assert_eq!(calls(&e, POINTER_SET).len(), 3 + 5);
    }

    const SPLATTER_STUBS: [u32; 12] = [
        SCOPE_GUARD_CONSTRUCT,
        SCOPE_GUARD_DESTRUCT,
        0x0043_c4b0,
        0x0044_0460,
        VALUE_CONSTRUCT,
        0x00a5_9c60,
        0x00a7_01b0,
        0x00a7_1b70,
        0x00a6_faf0,
        0x0063_fb70,
        0x00a7_12f0,
        0x00a5_ecb0,
    ];

    #[test]
    fn at_004e0540_does_nothing_when_disabled() {
        let mut e = engine(&SPLATTER_STUBS);
        setting(&mut e, 0);
        e.call(0x004e_0540, &args![]);
        let guard = calls(&e, SCOPE_GUARD_CONSTRUCT);
        assert_eq!(&guard[0][1..], &[7, 1, SCREEN_SPLATTER_FILE, 0xb5]);
        assert_eq!(calls(&e, SCOPE_GUARD_DESTRUCT), vec![vec![guard[0][0]]]);
        assert!(calls(&e, 0x0043_c4b0).is_empty());
        assert_eq!(e.global::<u8>(SPLATTER_ENABLED), 0);
    }

    #[test]
    fn at_004e0540_builds_the_camera_and_frustum() {
        let mut e = engine(&SPLATTER_STUBS);
        setting(&mut e, 1);
        pointers(&mut e);
        e.register(0x004d_ee10, |_, _| 1280u32.into_ret());
        e.register(0x004d_ee70, |_, _| 720u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00a7_12f0, |_, a| (a[0] + 1).into_ret());
        e.register(0x00a5_ecb0, |_, a| (a[0] + 2).into_ret());
        e.register(POINT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(0x0040_3df0, |_, a| (a[0] + 0x10).into_ret());
        // The frustum the camera is given, copied when it is set.
        e.register(0x00a6_faf0, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x1c);
            e.mem.write(0x011f_f800, &bytes);
            Ret::default()
        });
        e.set_global(THIRD_TEXTURE_ENABLED, 1u8);
        e.call(0x004e_0540, &args![]);
        assert_eq!(e.global::<u8>(SPLATTER_ENABLED), 1);
        let camera = e.global::<u32>(SPLATTER_CAMERA);
        assert_eq!(calls(&e, 0x00a7_12f0), vec![vec![camera - 1]]);
        assert_eq!(e.global::<u32>(SPLATTER_OBJECT) & 3, 2);
        let point = calls(&e, POINT_CONSTRUCT);
        assert_eq!(&point[0][1..], &[0, 0, 1.00001f32.to_bits()]);
        assert_eq!(calls(&e, 0x0044_0460), vec![vec![camera, point[0][0]]]);
        assert_eq!(
            calls(&e, 0x00a7_01b0),
            vec![vec![camera, LOOK_AT_POINT, LOOK_AT_UP]]
        );
        let aspect = 1280.0f32 / 720.0;
        let f = |i: u32| e.mem.f32(0x011f_f800 + 4 * i);
        assert_eq!(f(0), aspect * -0.5);
        assert_eq!(f(1), aspect * 0.5);
        assert_eq!(f(2), 0.5);
        assert_eq!(f(3), -0.5);
        assert_eq!(f(4), 1.0);
        assert_eq!(f(5), 10000.0);
        assert_eq!(e.mem.u8(0x011f_f818), 0);
        assert_eq!(e.global::<f32>(SPLATTER_FRUSTUM_RIGHT), aspect * 0.5);
        assert_eq!(e.global::<f32>(SPLATTER_FRUSTUM_TOP), 0.5);
        assert_eq!(calls(&e, 0x00a5_9c60).len(), 2);
        assert_eq!(
            calls(&e, 0x0063_fb70),
            vec![
                vec![SPLATTER_TEXTURES + 4, TEXTURE_NAME_SECOND + 0x10],
                vec![SPLATTER_TEXTURES, TEXTURE_NAME_FIRST + 0x10],
                vec![SPLATTER_TEXTURES + 8, TEXTURE_NAME_THIRD + 0x10],
            ]
        );
    }

    #[test]
    fn at_004e1650_constructs_the_water_system() {
        let stubs = [
            0x0063_3c90,
            0x0068_15c0,
            0x0048_f200,
            0x004e_d7a0,
            0x004e_d7d0,
            0x004e_d800,
            0x0041_a250,
        ];
        let mut e = engine(&stubs);
        setting(&mut e, 3);
        pointers(&mut e);
        e.register(0x004e_7ff0, |_, _| 0u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00a5_ecb0, |_, a| a[0].into_ret());
        e.register(0x0043_8170, |_, a| a[0].into_ret());
        e.register(COLOR_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        let target = e.mem.alloc(4);
        let vtable = e.mem.alloc(0xe0);
        e.mem.set_u32(target, vtable);
        e.mem.set_u32(vtable + 0xdc, 0x0066_0000);
        e.register(0x0066_0000, |_, _| Ret::default());
        e.register_double(0x0084_e3a0, move |_, _| target.into_ret());
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, 0x4444);
        e.register_double(0x0043_d4d0, move |_, _| list.into_ret());
        e.set_global(WATER_PAIR, 0x11u32);
        e.set_global(WATER_PAIR + 4, 0x22u32);
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this, 0xffff);
        assert_eq!(e.call(0x004e_1650, &args![this]).u32(), this);
        let pointers_built: Vec<u32> = calls(&e, 0x0063_3c90).iter().map(|w| w[0] - this).collect();
        assert_eq!(
            pointers_built,
            vec![0x08, 0x0c, 0x10, 0x14, 0x18, 0x1c, 0x20]
        );
        assert_eq!(
            calls(&e, 0x004e_d7a0),
            vec![vec![this + 0x4c, 0x25], vec![this + 0x5c, 0x25]]
        );
        assert_eq!(e.global::<u8>(WATER_SETTING_A_COPY), 3);
        let object = e.global::<u32>(WATER_POINTER_C28);
        assert_ne!(object, 0);
        assert_eq!(calls(&e, 0x0066_0000), vec![vec![target, object, 1]]);
        let color = |i: u32| e.global::<f32>(WATER_COLORS + 4 * i);
        assert_eq!(
            (0..12).map(color).collect::<Vec<_>>(),
            vec![0.0, 0.5, 0.5, 1.0, 0.0, 0.0, 0.1, 1.0, 1.0, 1.0, 1.0, 1.0]
        );
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0x11);
        assert_eq!(e.mem.u32(this + 0x2c), 0x22);
        let water = e.global::<u32>(WATER_OBJECT);
        assert_eq!(calls(&e, 0x0043_8170), vec![vec![water, WATER_NAME]]);
        assert_eq!(e.global::<u32>(WATER_WORD_A44), 0x4444);
        assert_eq!(e.global::<u32>(WATER_WORD_A40), 0x4444);
        assert_eq!(
            calls(&e, 0x0043_d4d0),
            vec![vec![WATER_LIST_BEC], vec![WATER_LIST_AFC]]
        );
    }

    #[test]
    fn at_004e19c0_destroys_the_water_system() {
        let stubs = [
            0x004e_6620,
            0x00b6_da10,
            0x004e_d900,
            0x004b_9900,
            EMPTY_BODY,
            0x004e_dbd0,
            0x004e_dad0,
            0x004e_d9d0,
            0x004a_1a30,
            0x0045_cec0,
        ];
        let mut e = engine(&stubs);
        pointers(&mut e);
        e.register(0x004a_0ea0, |_, _| 0x9900u32.into_ret());
        e.set_global(0x011c_7c2cu32, 0x5000u32);
        e.set_global(WATER_OBJECT, 0x6000u32);
        let this = e.mem.alloc(0xa0);
        e.call(0x004e_19c0, &args![this]);
        assert_eq!(calls(&e, 0x004e_6620), vec![vec![this, 1, 1]]);
        // Only the first texture pointer held a texture.
        assert_eq!(calls(&e, 0x00b6_da10), vec![vec![0x9900, 0x5000]]);
        assert_eq!(e.global::<u32>(0x011c_7c2c), 0);
        assert_eq!(calls(&e, 0x004e_d900).len(), 4);
        assert_eq!(calls(&e, 0x004b_9900), vec![vec![0x6000, 1]]);
        assert_eq!(e.global::<u32>(WATER_OBJECT), 0);
        let destroyed: Vec<u32> = calls(&e, 0x0045_cec0).iter().map(|w| w[0] - this).collect();
        assert_eq!(destroyed, vec![0x20, 0x1c, 0x18, 0x14, 0x10, 0x0c, 0x08]);
        // Without the object, nothing is deleted.
        e.call_log = Some(vec![]);
        e.call(0x004e_19c0, &args![this]);
        assert!(calls(&e, 0x004b_9900).is_empty());
    }

    /// The update's world: cell source, camera at height `z`, water height
    /// `height`, sky values, state object at `011dea0c` +0x88.
    fn water_world(e: &mut Engine, z: f32, height: f32, cell: u32) -> (u32, u32, u32) {
        for addr in [
            SCOPE_GUARD_CONSTRUCT,
            SCOPE_GUARD_DESTRUCT,
            EMPTY_BODY,
            0x004a_0c10,
            0x004e_3320,
            0x004e_21b0,
            0x004e_8a00,
            0x004e_8ec0,
            0x004e_6a80,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        pointers(e);
        setting(e, 1);
        e.set_global(WATER_CELLS, 0x3000u32);
        e.register_double(0x008d_6f30, move |_, _| cell.into_ret());
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, z);
        e.register_double(0x0045_bb80, move |_, _| position.into_ret());
        e.register(COLOR_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register_double(0x0054_71e0, move |_, _| height.into_ret());
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32 as u32).into_ret());
        let state = e.mem.alloc(0x170);
        let owner = e.mem.alloc(0x90);
        e.mem.set_u32(owner + 0x88, state);
        e.set_global(WATER_STATE_OWNER, owner);
        let vector = e.mem.alloc(12);
        e.mem.set_f32(vector, 0.25);
        e.mem.set_f32(vector + 4, 0.5);
        e.mem.set_f32(vector + 8, 0.75);
        for addr in [0x0046_dd00, 0x0045_cd60, 0x0055_85e0, 0x0096_11e0] {
            e.register(addr, |_, _| 0x10u32.into_ret());
        }
        e.register_double(0x0043_c490, move |_, _| vector.into_ret());
        e.register_double(0x0043_b5d0, move |_, _| vector.into_ret());
        e.register(0x0064_28b0, |_, _| 0.004f32.into_ret());
        e.register(0x005f_36f0, |_, _| 1u32.into_ret());
        e.register(0x0076_b610, |_, _| 0u32.into_ret());
        e.register(0x004d_c060, |_, _| 2u32.into_ret());
        e.register(0x004e_8950, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, 0xaaaa);
        (this, position, state)
    }

    #[test]
    fn at_004e1bc0_stops_without_a_cell_or_when_skipped() {
        let mut e = engine(&[]);
        let (this, _, _) = water_world(&mut e, 0.0, 0.0, 0);
        e.call(0x004e_1bc0, &args![this, 0x77u32, 0u32]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(calls(&e, SCOPE_GUARD_DESTRUCT).len(), 1);
        assert!(calls(&e, 0x0045_bb80).is_empty());
        let mut e = engine(&[]);
        let (this, _, _) = water_world(&mut e, 0.0, 0.0, 0x3100);
        e.set_global(WATER_SKIP, 1u8);
        e.call(0x004e_1bc0, &args![this, 0x77u32, 0u32]);
        assert_eq!(e.global::<u8>(WATER_SKIP), 0);
        assert!(calls(&e, 0x0045_bb80).is_empty());
        // Water disabled: the colour and camera are stored, nothing more.
        let mut e = engine(&[]);
        let (this, _, _) = water_world(&mut e, 0.0, 0.0, 0x3100);
        e.set_global(WATER_BYTE_1C4, 9u8);
        e.call(0x004e_1bc0, &args![this, 0x77u32, 0u32]);
        assert_eq!(e.global::<u8>(WATER_BYTE_1C4), 0);
        assert_eq!(e.global::<u32>(0x011c_7a50), 0x77);
        let entry = WATER_COLOR_TABLE + 0x1e * 16;
        assert_eq!(e.global::<f32>(entry), 1.0);
        assert_eq!(e.global::<f32>(entry + 4), 2.0);
        assert_eq!(e.global::<u32>(entry + 12), 0);
        assert!(calls(&e, 0x0054_71e0).is_empty());
        assert_eq!(calls(&e, EMPTY_BODY).len(), 2);
    }

    #[test]
    fn at_004e1bc0_under_water() {
        let mut e = engine(&[]);
        let (this, _, state) = water_world(&mut e, 5.0, 10.5, 0x3100);
        e.set_global(WATER_ENABLED, 1u8);
        e.set_global(WATER_REFRESH, 1u8);
        e.call(0x004e_1bc0, &args![this, 0x77u32, 0u32]);
        assert_eq!(e.global::<u8>(WATER_UNDERWATER), 1);
        assert_eq!(e.global::<u8>(0x011f_f8c4), 1);
        assert_eq!(e.global::<f32>(0x011f_f8a8), 5.0);
        assert_eq!(e.mem.u8(state + 0x167), 1);
        assert_eq!(e.mem.u32(state + 0x168), 10);
        assert_eq!(e.mem.u8(state + 0x166), 1);
        assert_eq!(e.global::<u8>(0x011f_f104), 1);
        assert_eq!(e.global::<f32>(0x011f_f398), 0.25);
        assert_eq!(e.global::<f32>(0x011f_f3b0), 0.75);
        let expected = f32::from_bits(x87_store32(x87_mul(f64::from(0.004f32), 100.0)));
        assert_eq!(e.global::<f32>(0x011f_f3a4), expected);
        // Under water, `011ff3b4` is 0.
        assert_eq!(e.global::<f32>(0x011f_f3b4), 0.0);
        assert_eq!(e.global::<u8>(0x011f_f101), 1);
        assert_eq!(calls(&e, 0x004e_3320), vec![vec![this, 0x77, 1]]);
        assert_eq!(calls(&e, 0x004e_21b0), vec![vec![this, 0x77, 1]]);
        assert_eq!(e.global::<u8>(WATER_REFRESH), 0);
        assert_eq!(calls(&e, 0x004e_8a00), vec![vec![this]]);
        assert_eq!(calls(&e, 0x004e_8ec0), vec![vec![this]]);
        assert_eq!(calls(&e, 0x004e_6a80), vec![vec![this]]);
        assert_eq!(e.global::<u8>(WATER_BYTE_7B4), 1);
    }

    #[test]
    fn at_004e1bc0_above_water_and_settled() {
        let mut e = engine(&[]);
        let (this, _, state) = water_world(&mut e, 12.0, 10.5, 0x3100);
        e.register(0x0076_b610, |_, _| 1u32.into_ret());
        e.register(0x0064_28b0, |_, _| 0.02f32.into_ret());
        e.set_global(WATER_ENABLED, 1u8);
        e.call(0x004e_1bc0, &args![this, 0x77u32, 0u32]);
        assert_eq!(e.global::<u8>(WATER_UNDERWATER), 0);
        assert_eq!(e.mem.u8(state + 0x167), 0);
        assert_eq!(e.mem.u8(state + 0x166), 1);
        // 0.02 * 100 is clamped to 1.
        assert_eq!(e.global::<f32>(0x011f_f3a4), 1.0);
        assert_eq!(e.global::<f32>(0x011f_f3b4), 1.0);
        // Settled and not forced: no placeable update.
        assert!(calls(&e, 0x004e_21b0).is_empty());
        assert!(calls(&e, 0x004e_6a80).is_empty());
        // Forced by 011c7a66: the placeable update runs, and the flags clear.
        e.set_global(WATER_FORCE_A, 1u8);
        e.call(0x004e_1bc0, &args![this, 0x77u32, 0u32]);
        assert_eq!(calls(&e, 0x004e_21b0).len(), 1);
        assert!(calls(&e, 0x004e_8ec0).is_empty());
        assert_eq!(e.global::<u8>(WATER_FORCE_A), 0);
    }

    #[test]
    fn at_004e1bc0_without_cells_disables_the_state() {
        let mut e = engine(&[]);
        let (this, _, state) = water_world(&mut e, 12.0, 10.5, 0x3100);
        e.set_global(WATER_ENABLED, 1u8);
        e.mem.set_u8(state + 0x166, 1);
        // The cell source answers only the first question.
        let mut first = true;
        e.register_double(0x008d_6f30, move |_, _| {
            let cell = if first { 0x3100u32 } else { 0 };
            first = false;
            cell.into_ret()
        });
        e.call(0x004e_1bc0, &args![this, 0x77u32, 0u32]);
        assert_eq!(e.mem.u8(state + 0x166), 0);
        assert!(calls(&e, 0x0054_71e0).is_empty());
    }
}

#[cfg(test)]
mod n039_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn float_at(e: &Engine, address: u32) -> f32 {
        f32::from_bits(e.mem.u32(address))
    }

    #[test]
    fn fn_004e0870_flushes_only_when_enabled() {
        let mut e = Engine::new();
        e.map(0x011c_7800, 0x80);
        e.register(POINTER_GET, |_, _| 0x4321u32.into_ret());
        e.register(0x004d_ef90, |_, _| Ret::default());
        e.set_global::<u32>(SPLATTER_WORD, 7);
        e.call_log = Some(vec![]);
        e.call(0x004e_0870, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.global::<u32>(SPLATTER_WORD), 7);
        e.set_global::<u8>(SPLATTER_ENABLED, 1);
        e.call_log = Some(vec![]);
        e.call(0x004e_0870, &args![]);
        assert_eq!(calls_to(&e, POINTER_GET), vec![vec![SPLATTER_OBJECT]]);
        assert_eq!(calls_to(&e, 0x004d_ef90), vec![vec![0x4321]]);
        assert_eq!(e.global::<u32>(SPLATTER_WORD), 0);
    }

    /// Doubles for `004e08a0`: points and sums are computed in the test, the rotation is the
    /// identity, `RandomFloat` gives low + high and the settings hold the given floats.
    fn splatter_rig(e: &mut Engine, third: bool) -> u32 {
        e.map(0x0118_9550, 8);
        e.set_global::<f32>(SPLATTER_FRUSTUM_RIGHT, 2.0);
        e.set_global::<f32>(SPLATTER_FRUSTUM_TOP, 4.0);
        e.map(0x0101_6248, 4);
        e.set_global::<f32>(HALF, 0.5);
        e.map(0x0101_2060, 8);
        e.set_global::<f64>(ZERO_DOUBLE, 0.0);
        e.map(0x0101_1588, 8);
        e.set_global::<f64>(HALF_DOUBLE, 0.5);
        e.map(0x011f_941e, 1);
        e.set_global::<u8>(THIRD_TEXTURE_ENABLED, u8::from(third));
        e.map(SPLATTER_TEXTURES, 12);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.put_vtable(0x00d9_0000, &[0x00e6_0000; 0x40]);
        e.register(0x00e6_0000, |_, _| Ret::default());
        e.register(0x00a5_ecb0, |e, a| {
            e.mem.set_u32(a[0], 0x00d9_0000);
            a[0].into_ret()
        });
        for address in [
            0x0040_1050,
            0x0068_15c0,
            0x004d_fdd0,
            0x00b5_7e30,
            0x0043_9410,
            0x004d_fe80,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(POINTER_GET, |_, a| (a[0] + 0x1000).into_ret());
        e.register(0x00a7_4410, |_, a| a[0].into_ret());
        e.register(0x00b8_1200, |_, a| a[0].into_ret());
        let mut randoms = vec![0.5f32, 0.25, -1.0, 1.0];
        e.register_double(RANDOM_UNIT, move |_, _| randoms.remove(0).into_ret());
        let settings = e.mem.alloc(0x40);
        let table = [
            (0x011c_f9a4u32, 1.0f32),
            (0x011c_faa0, 2.0),
            (0x011c_fae0, 0.5),
            (0x011c_f980, 0.5),
            (0x011c_eca0, 0.0),
            (0x011c_f920, 0.0),
            (0x011c_efe4, 0.0),
            (0x011c_f2c4, 0.25),
        ];
        for (i, (_, value)) in table.iter().enumerate() {
            e.mem.set_u32(settings + 4 * i as u32, value.to_bits());
        }
        e.register_double(SETTING_ADDRESS, move |_, a| {
            let i = table.iter().position(|(s, _)| *s == a[0]).unwrap() as u32;
            (settings + 4 * i).into_ret()
        });
        e.register(RANDOM_FLOAT, |_, a| {
            (f32::from_bits(a[0]) + f32::from_bits(a[1])).into_ret()
        });
        e.register(0x004d_ff00, |_, _| false.into_ret());
        e.register(POINT_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e.register(0x004b_4500, |e, a| {
            for i in 0..3 {
                let v = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(0x0043_9e90, |e, a| {
            for i in 0..3 {
                let sum = f32::from_bits(e.mem.u32(a[0] + 4 * i))
                    + f32::from_bits(e.mem.u32(a[2] + 4 * i));
                e.mem.set_u32(a[1] + 4 * i, sum.to_bits());
            }
            a[1].into_ret()
        });
        e.register(0x004d_fe30, |e, a| {
            for i in 0..2 {
                let sum = f32::from_bits(e.mem.u32(a[0] + 4 * i))
                    + f32::from_bits(e.mem.u32(a[2] + 4 * i));
                e.mem.set_u32(a[1] + 4 * i, sum.to_bits());
            }
            a[1].into_ret()
        });
        settings
    }

    #[test]
    fn fn_004e08a0_builds_one_quad() {
        let mut e = Engine::new();
        splatter_rig(&mut e, false);
        e.call_log = Some(vec![]);
        let node = e.call(0x004e_08a0, &args![1u32]).u32();
        let shapes = calls_to(&e, 0x00a7_4410);
        assert_eq!(shapes.len(), 1);
        let shape = &shapes[0];
        assert_eq!(
            (shape[1], shape[3], shape[4], shape[6], shape[7], shape[8]),
            (4, 0, 0, 1, 0, 2)
        );
        let (vertices, coordinates, indices) = (shape[2], shape[5], shape[9]);
        // Centre (1, 1, 0), half size 3, depth 1.
        let corners: Vec<f32> = (0..12).map(|i| float_at(&e, vertices + 4 * i)).collect();
        assert_eq!(
            corners,
            vec![-2.0, 4.0, 1.0, 4.0, 4.0, 1.0, -2.0, -2.0, 1.0, 4.0, -2.0, 1.0]
        );
        // Offset (0.5, 0.0).
        let uv: Vec<f32> = (0..8).map(|i| float_at(&e, coordinates + 4 * i)).collect();
        assert_eq!(uv, vec![1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.0]);
        let triangles: Vec<u16> = (0..6).map(|i| e.mem.u16(indices + 2 * i)).collect();
        assert_eq!(triangles, vec![0, 2, 1, 1, 2, 3]);
        let property = calls_to(&e, 0x00b8_1200)[0][0];
        assert_eq!(
            calls_to(&e, 0x004d_fe80),
            vec![
                vec![property, 0, 0x011c_8864],
                vec![property, 1, 0x011c_8860]
            ]
        );
        assert_eq!(calls_to(&e, 0x00e6_0000), vec![vec![node, shape[0], 1]]);
        assert_eq!(calls_to(&e, 0x00b5_7e30), vec![vec![shape[0], 0, 0]]);
    }

    #[test]
    fn fn_004e08a0_builds_the_second_set_with_the_third_texture() {
        let mut e = Engine::new();
        splatter_rig(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x004e_08a0, &args![1u32]);
        let shapes = calls_to(&e, 0x00a7_4410);
        assert_eq!(shapes.len(), 2);
        let (vertices, coordinates, indices) = (shapes[1][2], shapes[1][5], shapes[1][9]);
        let corners: Vec<f32> = (0..3).map(|i| float_at(&e, vertices + 4 * i)).collect();
        assert_eq!(corners, vec![0.75, 1.25, 1.0]);
        let uv: Vec<f32> = (0..8).map(|i| float_at(&e, coordinates + 4 * i)).collect();
        assert_eq!(uv, vec![1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u16(indices + 0xa), 3);
        let property = calls_to(&e, 0x00b8_1200)[1][0];
        assert_eq!(calls_to(&e, 0x004d_fe80)[2], vec![property, 0, 0x011c_8868]);
        assert_eq!(calls_to(&e, OPERATOR_NEW).len(), 6);
    }
}
