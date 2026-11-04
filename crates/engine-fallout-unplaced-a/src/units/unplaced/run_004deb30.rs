//! `(unplaced)/run_004deb30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The screen splatter effect: `004deb30` names
//! `Fallout Shared\ScreenCustomSplatter.cpp` in its scope guard, and the
//! globals these functions share match the Xbox PDB's static members of
//! `ScreenCustomSplatter` (`spRoot`, `spCamera`, `fWRadius`, `fHRadius`,
//! `bEnabled`, `bSecondsForFade`, `fFadeTime`, `uiCount`, `spTextures`) by
//! their use: which address holds which member is read from the code (the
//! node and camera by their constructors), not confirmed by a symbol. The
//! PC build keeps three texture pointers where the Xbox PDB has twelve.
//!
//! x87 note: products are computed in `f64` and rounded to `f32` where the
//! code stores a `float`. A `float` copied with `FLD`/`FSTP` (into memory or
//! onto the stack as an argument) comes out with a signalling NaN quieted.

#[allow(unused_imports)]
use crate::prelude::*;

/// The enabled flag (`ScreenCustomSplatter::bEnabled` by use).
const ENABLED: u32 = 0x011c_77e8;
/// The byte cleared with the fade time (`bSecondsForFade` by use).
const FADE_FLAG: u32 = 0x011c_77e9;
/// The fade time (`fFadeTime` by use).
const FADE_TIME: u32 = 0x011c_77ec;
/// The counter cleared on reset (`uiCount` by use).
const COUNT: u32 = 0x011c_77dc;
/// Three texture pointers (`spTextures` by use), 4 bytes apart.
const TEXTURES: u32 = 0x011c_7804;
/// The node pointer (`spRoot` by use; an `NiNode` from `00a5ecb0`).
const ROOT: u32 = 0x011c_7810;
/// The camera pointer (`spCamera` by use; constructed by `00a712f0` in
/// `NiMain/nicamera.cpp`).
const CAMERA: u32 = 0x011c_781c;
/// Half width of the view (`fWRadius` by use), set from the frustum.
const WIDTH_RADIUS: u32 = 0x0118_9500;
/// Half height of the view (`fHRadius` by use), set from the frustum.
const HEIGHT_RADIUS: u32 = 0x0118_9504;
/// A byte flag that adds the third texture and the second shape
/// (meaning unconfirmed).
const SECOND_LAYER: u32 = 0x011f_941e;

/// Allocator for game objects (`00aa13e0`, cdecl: size).
const OBJECT_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiNode` constructor (`00a5ecb0`): this, one word.
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// Smart pointer assignment (`0066b0d0`): this (the pointer), new object.
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// Smart pointer read (`00559450`): this (the pointer); the object.
const POINTER_GET: u32 = 0x0055_9450;
/// String setting read (`00403df0`): this (the setting); its value.
const SETTING_STRING: u32 = 0x0040_3df0;
/// Float setting read (`00403e20`): this (the setting); a pointer to the
/// value.
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// Sets a texture pointer from a file name (`0063fb70`, cdecl: pointer,
/// name).
const TEXTURE_LOAD: u32 = 0x0063_fb70;
/// Point constructor (`00416870`): this, x, y, z; returns this.
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// Two-float constructor (`00452dc0`): this, x, y; returns this.
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
/// The random number generator object (`00476c00`, no arguments).
const RANDOM_GENERATOR: u32 = 0x0047_6c00;
/// Its next 32-bit value (`00476be0`, this).
const RANDOM_NEXT: u32 = 0x0047_6be0;
/// `RandomFloat` (Xbox PDB, `00476b70`, cdecl: two floats; float in ST0).
const RANDOM_FLOAT: u32 = 0x0047_6b70;

/// `2^-31` (`double`).
const SCALE_SIGNED_010229B8: u32 = 0x0102_29b8;
/// `2^-32` (`double`).
const SCALE_UNSIGNED_010229C0: u32 = 0x0102_29c0;
/// `1.0` (`double`).
const ONE_01012070: u32 = 0x0101_2070;
/// `0.0` (`double`).
const ZERO_01012060: u32 = 0x0101_2060;
/// `0.5` (`double`).
const HALF_01011588: u32 = 0x0101_1588;
/// `-0.5` (`double`).
const MINUS_HALF_01011598: u32 = 0x0101_1598;
/// `0.5` (`float`).
const HALF_01016248: u32 = 0x0101_6248;
/// `-0.5` (`float`).
const MINUS_HALF_0102295C: u32 = 0x0102_295c;
/// `10000.0` (`float`).
const FAR_01022958: u32 = 0x0102_2958;
/// `1.00001` (`float`, bits `3f800054`).
const CAMERA_Z_01022960: u32 = 0x0102_2960;

/// `FLD`/`FSTP` of a `float`: a signalling NaN comes out quiet.
fn fld_copy(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

/// The `float` a float setting points at (`00403e20`, then `FLD`).
fn float_setting(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_FLOAT, &args![setting]).u32();
    fld_copy(e.mem.f32(value))
}

fn second_layer(e: &Engine) -> bool {
    e.global::<u8>(SECOND_LAYER) != 0
}

/// Allocates and constructs an `NiNode` (`00a5ecb0` with 0); null when the
/// allocation fails.
fn new_node(e: &mut Engine) -> u32 {
    let block = e.call(OBJECT_ALLOCATE, &args![0xacu32]).u32();
    if block == 0 {
        0
    } else {
        e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
    }
}

/// Loads the two (three with the second layer) textures from their string
/// settings: `011ce918` into the second pointer, `011ce284` into the
/// first, `011cede8` into the third.
fn load_textures(e: &mut Engine) {
    let name = e.call(SETTING_STRING, &args![0x011c_e918u32]).u32();
    e.call(TEXTURE_LOAD, &args![TEXTURES + 4, name]);
    let name = e.call(SETTING_STRING, &args![0x011c_e284u32]).u32();
    e.call(TEXTURE_LOAD, &args![TEXTURES, name]);
    if second_layer(e) {
        let name = e.call(SETTING_STRING, &args![0x011c_ede8u32]).u32();
        e.call(TEXTURE_LOAD, &args![TEXTURES + 8, name]);
    }
}

/// Size of `004deb30`'s locals that it passes by address (`EBP-0x80` up).
const INIT_FRAME: u32 = 0x80;

// Translated from 004deb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the effect (called from `main`): reads the enabled flag
/// from the setting at `011c77f4` (`00408d60`); when it is set, makes the
/// camera (`00a712f0`) with a frustum of the screen's aspect ratio
/// (`004dee10` / `004dee70` of the object from `0043c4b0`), places it
/// (`00440460`, `00a59c60`, `NiCamera::LookAtWorldPoint`), sets the width
/// and height radii from the frustum, the fade time from the setting at
/// `011cfc2c`, makes the node, clears the counter and loads the textures.
/// The whole body runs inside a scope guard (`00404eb0` / `00404ee0`).
// Translation incomplete: 004deb30 - the exception frame (FS:[0]) that runs the scope guard's destructor and frees the camera and node blocks during unwinding is not translated
pub fn fn_004deb30(e: &mut Engine) {
    e.with_stack(INIT_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + INIT_FRAME - offset;
        e.call(
            0x0040_4eb0,
            &args![at(0x10), 7u32, 1u32, 0x0102_2968u32, 0xbdu32],
        );
        let enabled = e.call(0x0040_8d60, &args![0x011c_77f4u32]).u32();
        let enabled = e.mem.u8(enabled);
        e.set_global(ENABLED, enabled);
        if e.global::<u8>(ENABLED) != 0 {
            let screen = e.call(0x0043_c4b0, &[]).u32();
            let width = fn_004dee10(e, Ptr::new(screen));
            let height = fn_004dee70(e, Ptr::new(screen));
            let aspect = (width as f64 / height as f64) as f32;

            let block = e.call(OBJECT_ALLOCATE, &args![0x114u32]).u32();
            let camera = if block == 0 {
                0
            } else {
                e.call(0x00a7_12f0, &args![block]).u32()
            };
            e.call(POINTER_ASSIGN, &args![CAMERA, camera]);

            let camera_z = fld_copy(e.global::<f32>(CAMERA_Z_01022960));
            let point = e
                .call(POINT3_CONSTRUCT, &args![at(0x60), 0.0f32, 0.0f32, camera_z])
                .u32();
            let camera = e.call(POINTER_GET, &args![CAMERA]).u32();
            e.call(0x0044_0460, &args![camera, point]);

            e.call(0x0043_d410, &args![at(0x28), 0.0f32, 0u32, 0u32]);
            let camera = e.call(POINTER_GET, &args![CAMERA]).u32();
            e.call(0x00a5_9c60, &args![camera, at(0x28)]);

            let camera = e.call(POINTER_GET, &args![CAMERA]).u32();
            // `NiCamera::LookAtWorldPoint` (Xbox PDB).
            e.call(0x00a7_01b0, &args![camera, 0x011f_426cu32, 0x011a_9478u32]);

            // `NiFrustum` (Xbox PDB) at EBP-0x48: left, right, top, bottom,
            // near, far, ortho.
            let frustum = at(0x48);
            e.call(0x00a7_1b70, &args![frustum, 0u32]);
            e.mem.set_u8(frustum + 0x18, 0);
            let top = fld_copy(e.global::<f32>(HALF_01016248));
            e.mem.set_f32(frustum + 0x08, top);
            let bottom = fld_copy(e.global::<f32>(MINUS_HALF_0102295C));
            e.mem.set_f32(frustum + 0x0c, bottom);
            let left = aspect as f64 * e.global::<f64>(MINUS_HALF_01011598);
            e.mem.set_f32(frustum, left as f32);
            let right = aspect as f64 * e.global::<f64>(HALF_01011588);
            e.mem.set_f32(frustum + 0x04, right as f32);
            let far = fld_copy(e.global::<f32>(FAR_01022958));
            e.mem.set_f32(frustum + 0x14, far);
            e.mem.set_f32(frustum + 0x10, 1.0);
            let camera = e.call(POINTER_GET, &args![CAMERA]).u32();
            // `NiCamera::SetViewFrustum` (Xbox PDB).
            e.call(0x00a6_faf0, &args![camera, frustum]);

            e.call(0x0043_d410, &args![at(0x1c), 0.0f32, 0u32, 0u32]);
            let camera = e.call(POINTER_GET, &args![CAMERA]).u32();
            e.call(0x00a5_9c60, &args![camera, at(0x1c)]);

            let right = fld_copy(e.mem.f32(frustum + 0x04));
            e.set_global(WIDTH_RADIUS, right);
            let top = fld_copy(e.mem.f32(frustum + 0x08));
            e.set_global(HEIGHT_RADIUS, top);

            let fade_time = float_setting(e, 0x011c_fc2c);
            e.set_global(FADE_TIME, fade_time);
            e.set_global(FADE_FLAG, 0u8);

            let node = new_node(e);
            e.call(POINTER_ASSIGN, &args![ROOT, node]);
            e.set_global(COUNT, 0u32);
            load_textures(e);
        }
        e.call(0x0040_4ee0, &args![at(0x10)]);
    });
}

// Translated from 004dee10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The width the effect sizes its frustum by: `this`'s virtual +0xC8, then
/// that object's virtual +0xB0 with 0, then `0044ddc0` on the result; 0
/// when either virtual returns null.
pub fn fn_004dee10(e: &mut Engine, this: Ptr) -> u32 {
    let first = e.vcall(this.addr(), 0xc8, &[]).u32();
    if first == 0 {
        return 0;
    }
    let second = e.vcall(first, 0xb0, &args![0u32]).u32();
    if second == 0 {
        return 0;
    }
    e.call(0x0044_ddc0, &args![second]).u32()
}

// Translated from 004dee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The height the effect sizes its frustum by: as `004dee10`, ending in
/// `0084e3a0`.
pub fn fn_004dee70(e: &mut Engine, this: Ptr) -> u32 {
    let first = e.vcall(this.addr(), 0xc8, &[]).u32();
    if first == 0 {
        return 0;
    }
    let second = e.vcall(first, 0xb0, &args![0u32]).u32();
    if second == 0 {
        return 0;
    }
    e.call(0x0084_e3a0, &args![second]).u32()
}

// Translated from 004deed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads texture `index` (0 to 2) from the file `name` (`0063fb70`); false
/// for a larger index.
pub fn fn_004deed0(e: &mut Engine, name: Ptr, index: u8) -> bool {
    if index > 2 {
        return false;
    }
    e.call(TEXTURE_LOAD, &args![TEXTURES + index as u32 * 4, name]);
    true
}

// Translated from 004def00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the fade time and the byte next to the enabled flag; returns true.
pub fn fn_004def00(e: &mut Engine, fade_time: f32, flag: u8) -> bool {
    e.set_global(FADE_TIME, fld_copy(fade_time));
    e.set_global(FADE_FLAG, flag);
    true
}

// Translated from 004def20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shuts the effect down (called from `main`): when enabled, empties the
/// node's children (`004def90`) and releases the node and the camera; then
/// clears the enabled flag and releases the three textures.
pub fn fn_004def20(e: &mut Engine) {
    if e.global::<u8>(ENABLED) != 0 {
        let node = e.call(POINTER_GET, &args![ROOT]).u32();
        fn_004def90(e, Ptr::new(node));
        e.call(POINTER_ASSIGN, &args![ROOT, 0u32]);
        e.call(POINTER_ASSIGN, &args![CAMERA, 0u32]);
    }
    e.set_global(ENABLED, 0u8);
    for i in 0..3u32 {
        e.call(POINTER_ASSIGN, &args![TEXTURES + i * 4, 0u32]);
    }
}

// Translated from 004def90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the array at +0x9C of a node (`004dffa0`, which the engine map
/// names `NiTArray<NiPointer<BSTempEffect>>::RemoveAll`).
pub fn fn_004def90(e: &mut Engine, this: Ptr) {
    e.call(0x004d_ffa0, &args![this.addr() + 0x9c]);
}

// Translated from 004defb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the effect when enabled: empties the node's children, clears the
/// counter, the fade time and the byte next to the enabled flag, and loads
/// the textures again.
pub fn fn_004defb0(e: &mut Engine) {
    if e.global::<u8>(ENABLED) == 0 {
        return;
    }
    let node = e.call(POINTER_GET, &args![ROOT]).u32();
    fn_004def90(e, Ptr::new(node));
    e.set_global(COUNT, 0u32);
    e.set_global(FADE_TIME, 0.0f32);
    e.set_global(FADE_FLAG, 0u8);
    load_textures(e);
}

/// `n * size` as the allocation size, or `0xFFFFFFFF` when the product
/// does not fit (`MUL`, `SETO`, `NEG`, `OR`).
fn array_size(n: u32, size: u32) -> u32 {
    u32::try_from(n as u64 * size as u64).unwrap_or(u32::MAX)
}

/// `new T[n]` for 12- and 8-byte elements: allocates (`00401000`) and runs
/// the vector constructor iterator (`00401050`) with `006815c0` on each
/// element; null when the allocation fails.
fn new_array(e: &mut Engine, n: u32, size: u32) -> u32 {
    let block = e.call(0x0040_1000, &args![array_size(n, size)]).u32();
    if block != 0 {
        e.call(0x0040_1050, &args![block, size, n, 0x0068_15c0u32]);
    }
    block
}

/// Allocates and constructs an `NiTriShape` (Xbox PDB constructor
/// `00a74410`): `corners` vertices at `positions`, texture coordinates at
/// `uvs`, `2 * count` triangles at `triangles`; null when the allocation
/// fails.
fn new_tri_shape(e: &mut Engine, count: u32, positions: u32, uvs: u32, triangles: u32) -> u32 {
    let block = e.call(OBJECT_ALLOCATE, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        0x00a7_4410,
        &args![
            block,
            count << 2,
            positions,
            0u32,
            0u32,
            uvs,
            1u32,
            0u32,
            count << 1,
            triangles
        ],
    )
    .u32()
}

/// Allocates 0x70 bytes and constructs them with `00b81200` (a
/// `BSShader` unit property); null when the allocation fails.
fn new_property(e: &mut Engine) -> u32 {
    let block = e.call(OBJECT_ALLOCATE, &args![0x70u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(0x00b8_1200, &args![block]).u32()
    }
}

/// Attaches `property` to `shape` (`NiAVObject::AttachProperty`, Xbox
/// PDB), prepares it (`BSShaderManager::PrepareObject`, Xbox PDB, with
/// 0, 0) and adds it to `node` through the node's virtual +0xDC (shape, 1).
fn attach_shape(e: &mut Engine, node: u32, shape: u32, property: u32) {
    e.call(0x0043_9410, &args![shape, property]);
    e.call(0x00b5_7e30, &args![shape, 0u32, 0u32]);
    e.vcall(node, 0xdc, &args![shape, 1u32]);
}

/// Size of `004df040`'s locals that it passes by address (`EBP-0x2A0` up).
const SHAPE_FRAME: u32 = 0x2a0;

/// One corner of a quad: whether x and y are negated (`FCHS`), and the
/// stack temporaries (point, rotated point, translated point) the code
/// uses for it, as `EBP` offsets.
const CORNERS: [(bool, bool, [u32; 3]); 4] = [
    (true, false, [0xe0, 0xec, 0xf8]),
    (false, false, [0x104, 0x110, 0x11c]),
    (true, true, [0x128, 0x134, 0x140]),
    (false, true, [0x14c, 0x158, 0x164]),
];

/// `value`, sign flipped when `negate` (`FCHS`).
fn signed(value: f32, negate: bool) -> f32 {
    if negate {
        -value
    } else {
        value
    }
}
/// The same for the second layer's quad.
const SECOND_CORNERS: [[u32; 3]; 4] = [
    [0x170, 0x17c, 0x188],
    [0x194, 0x1a0, 0x1ac],
    [0x1b8, 0x1c4, 0x1d0],
    [0x1dc, 0x1e8, 0x1f4],
];
/// Stack temporaries (pair, sum) for the four texture coordinates.
const UV_TEMPORARIES: [[u32; 2]; 4] = [
    [0x1fc, 0x204],
    [0x20c, 0x214],
    [0x21c, 0x224],
    [0x22c, 0x234],
];
/// Stack temporaries for the second layer's texture coordinates.
const SECOND_UV_TEMPORARIES: [u32; 4] = [0x23c, 0x244, 0x24c, 0x254];

/// Copies `words` words from `from` to `to` (`MOV` pairs).
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for i in 0..words {
        let w = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to.wrapping_add(4 * i), w);
    }
}

// Translated from 004df040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the splatter geometry for `count` splats: a new `NiNode` and one
/// `NiTriShape` of `count` quads (two with the second layer). Each quad is
/// placed at a random point of the screen (`004dfea0` times the width and
/// height radii), sized by `RandomFloat` of two settings times
/// `width_scale`, at a depth of `RandomFloat` times `depth_scale` (one of
/// two depth ranges, chosen by `004dff00` with the setting at `011cefe4`);
/// its corners go through an identity matrix (`004b4500`) and the
/// translation (`00439e90`), and its texture coordinates are one quarter
/// of the texture, the quarter chosen by two more random signs. The second
/// layer's quad uses the half size from the setting at `011cf2c4` and the
/// whole texture. The shapes get a property (`00b81200`) with the textures
/// and are added to the node, which is returned.
// Translation incomplete: 004df040 - the exception frame (FS:[0]) that frees the node, array and shape blocks during unwinding is not translated
pub fn fn_004df040(e: &mut Engine, count: u32, width_scale: f32, depth_scale: f32) -> Ptr {
    let node = new_node(e);
    let positions = new_array(e, count << 2, 0xc);
    let uvs = new_array(e, count << 2, 8);
    let triangles = e
        .call(0x0040_1000, &args![array_size(count.wrapping_mul(6), 2)])
        .u32();
    let (second_positions, second_uvs, second_triangles) = if second_layer(e) {
        let positions = new_array(e, count << 2, 0xc);
        let uvs = new_array(e, count << 2, 8);
        let triangles = e
            .call(0x0040_1000, &args![array_size(count.wrapping_mul(6), 2)])
            .u32();
        (positions, uvs, triangles)
    } else {
        (0, 0, 0)
    };

    e.with_stack(SHAPE_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + SHAPE_FRAME - offset;
        let translation = at(0x88);
        let matrix = at(0x7c);
        let uv_offset = at(0x50);
        for i in 0..count {
            let random = fn_004dfea0(e);
            let x = (random as f64 * e.global::<f32>(WIDTH_RADIUS) as f64) as f32;
            let random = fn_004dfea0(e);
            let y = (random as f64 * e.global::<f32>(HEIGHT_RADIUS) as f64) as f32;
            e.call(
                POINT3_CONSTRUCT,
                &args![translation, fld_copy(x), fld_copy(y), 0.0f32],
            );

            let high = float_setting(e, 0x011c_faa0);
            let low = float_setting(e, 0x011c_f9a4);
            let size = e.call(RANDOM_FLOAT, &args![low, high]).f32();
            let size = (size as f64 * width_scale as f64) as f32;
            let high = float_setting(e, 0x011c_f980);
            let low = float_setting(e, 0x011c_fae0);
            let mut depth = e.call(RANDOM_FLOAT, &args![low, high]).f32();
            let high = float_setting(e, 0x011c_f920);
            let low = float_setting(e, 0x011c_eca0);
            let other_depth = e.call(RANDOM_FLOAT, &args![low, high]).f32();
            depth = (depth as f64 * depth_scale as f64) as f32;
            let other_depth = (other_depth as f64 * depth_scale as f64) as f32;

            e.call(0x0068_15c0, &args![matrix]);
            fn_004dfdd0(e, Ptr::new(matrix));
            let chance = float_setting(e, 0x011c_efe4);
            if fn_004dff00(e, chance) {
                depth = fld_copy(other_depth);
            }

            // Which quarter of the texture: 0.5 when the random value is
            // not above 0.0 (NaN included), else 0.0.
            let zero: f64 = e.global(ZERO_01012060);
            let random = fn_004dfea0(e);
            let u = if random as f64 > zero {
                0.0
            } else {
                e.global::<f64>(HALF_01011588) as f32
            };
            let random = fn_004dfea0(e);
            let v = if random as f64 > zero {
                0.0
            } else {
                e.global::<f64>(HALF_01011588) as f32
            };
            e.call(
                POINT2_CONSTRUCT,
                &args![uv_offset, fld_copy(u), fld_copy(v)],
            );
            let half_size = float_setting(e, 0x011c_f2c4);

            let first = i << 2;
            let corner = |e: &mut Engine, x: f32, y: f32, temps: [u32; 3], to: u32| {
                let point = e
                    .call(
                        POINT3_CONSTRUCT,
                        &args![at(temps[0]), fld_copy(x), fld_copy(y), fld_copy(depth)],
                    )
                    .u32();
                let rotated = e
                    .call(0x004b_4500, &args![matrix, at(temps[1]), point])
                    .u32();
                let moved = e
                    .call(0x0043_9e90, &args![rotated, at(temps[2]), translation])
                    .u32();
                copy_words(e, moved, to, 3);
            };
            for (j, (negate_x, negate_y, temps)) in CORNERS.iter().enumerate() {
                let to = first
                    .wrapping_add(j as u32)
                    .wrapping_mul(12)
                    .wrapping_add(positions);
                let x = signed(size, *negate_x);
                let y = signed(size, *negate_y);
                corner(e, x, y, *temps, to);
            }
            if second_layer(e) {
                for (j, temps) in SECOND_CORNERS.iter().enumerate() {
                    let (negate_x, negate_y, _) = CORNERS[j];
                    let to = first
                        .wrapping_add(j as u32)
                        .wrapping_mul(12)
                        .wrapping_add(second_positions);
                    let x = signed(half_size, negate_x);
                    let y = signed(half_size, negate_y);
                    corner(e, x, y, *temps, to);
                }
            }

            let half = fld_copy(e.global::<f32>(HALF_01016248));
            let quarter_corners = [(half, half), (0.0, half), (half, 0.0), (0.0, 0.0)];
            let uv_base = first.wrapping_mul(8).wrapping_add(uvs);
            for (j, ((cu, cv), temps)) in quarter_corners.iter().zip(UV_TEMPORARIES).enumerate() {
                let pair = e
                    .call(POINT2_CONSTRUCT, &args![at(temps[0]), *cu, *cv])
                    .u32();
                let sum = fn_004dfe30(
                    e,
                    Ptr::new(pair),
                    Ptr::new(at(temps[1])),
                    Ptr::new(uv_offset),
                );
                copy_words(e, sum.addr(), uv_base.wrapping_add(8 * j as u32), 2);
            }
            if second_layer(e) {
                let whole_corners = [(1.0f32, 1.0f32), (0.0, 1.0), (1.0, 0.0), (0.0, 0.0)];
                let uv_base = first.wrapping_mul(8).wrapping_add(second_uvs);
                for (j, ((cu, cv), temp)) in
                    whole_corners.iter().zip(SECOND_UV_TEMPORARIES).enumerate()
                {
                    let pair = e.call(POINT2_CONSTRUCT, &args![at(temp), *cu, *cv]).u32();
                    copy_words(e, pair, uv_base.wrapping_add(8 * j as u32), 2);
                }
            }

            // Two triangles: (0, 2, 1) and (1, 2, 3) of the quad.
            let order = [0u32, 2, 1, 1, 2, 3];
            let base = i.wrapping_mul(6).wrapping_mul(2);
            for (j, k) in order.iter().enumerate() {
                let index = first.wrapping_add(*k) as u16;
                e.mem.set_u16(
                    triangles.wrapping_add(base).wrapping_add(2 * j as u32),
                    index,
                );
            }
            if second_layer(e) {
                for (j, k) in order.iter().enumerate() {
                    let index = first.wrapping_add(*k) as u16;
                    e.mem.set_u16(
                        second_triangles
                            .wrapping_add(base)
                            .wrapping_add(2 * j as u32),
                        index,
                    );
                }
            }
        }
    });

    let shape = new_tri_shape(e, count, positions, uvs, triangles);
    let property = new_property(e);
    let texture = e.call(POINTER_GET, &args![TEXTURES + 4]).u32();
    fn_004dfe80(e, Ptr::new(property), 0, Ptr::new(texture));
    let texture = e.call(POINTER_GET, &args![TEXTURES]).u32();
    fn_004dfe80(e, Ptr::new(property), 1, Ptr::new(texture));
    attach_shape(e, node, shape, property);

    if second_layer(e) {
        let shape = new_tri_shape(e, count, second_positions, second_uvs, second_triangles);
        let property = new_property(e);
        let texture = e.call(POINTER_GET, &args![TEXTURES + 8]).u32();
        fn_004dfe80(e, Ptr::new(property), 0, Ptr::new(texture));
        attach_shape(e, node, shape, property);
    }
    Ptr::new(node)
}

// Translated from 004dfdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the 3x3 `float` matrix at `this` to the identity.
pub fn fn_004dfdd0(e: &mut Engine, this: Ptr) {
    for i in 0..9u32 {
        let value = if i % 4 == 0 { 1.0 } else { 0.0 };
        e.mem.set_f32(this.addr() + 4 * i, value);
    }
}

// Translated from 004dfe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Two-float sum: `result = this + other`, componentwise; returns `result`.
pub fn fn_004dfe30(e: &mut Engine, this: Ptr, result: Ptr, other: Ptr) -> Ptr {
    let x = e.mem.f32(this.addr());
    let y = e.mem.f32(this.addr() + 4);
    let x = (x as f64 + e.mem.f32(other.addr()) as f64) as f32;
    let y = (y as f64 + e.mem.f32(other.addr() + 4) as f64) as f32;
    e.mem.set_f32(result.addr(), x);
    e.mem.set_f32(result.addr() + 4, y);
    result
}

// Translated from 004dfe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `value` to the pointer slot `index` of the array at +0x60 of
/// `this` (`0066b0d0`); used for the property's textures.
pub fn fn_004dfe80(e: &mut Engine, this: Ptr, index: u32, value: Ptr) {
    let slot = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(0x60);
    e.call(POINTER_ASSIGN, &args![slot, value]);
}

// Translated from 004dfea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random `float` in [-1, 1) from the generator (`004dfec0`).
pub fn fn_004dfea0(e: &mut Engine) -> f32 {
    let generator = e.call(RANDOM_GENERATOR, &[]).u32();
    fn_004dfec0(e, Ptr::new(generator))
}

// Translated from 004dfec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The generator's next value (`00476be0`, unsigned) times `2^-31`, minus
/// 1.0, as a `float`.
pub fn fn_004dfec0(e: &mut Engine, this: Ptr) -> f32 {
    let value = e.call(RANDOM_NEXT, &args![this]).u32();
    let scale: f64 = e.global(SCALE_SIGNED_010229B8);
    let one: f64 = e.global(ONE_01012070);
    (value as f64 * scale - one) as f32
}

// Translated from 004dff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random chance: true when `chance` is at least a random value in
/// [0, 1) (`004dff20` on the generator).
pub fn fn_004dff00(e: &mut Engine, chance: f32) -> bool {
    let generator = e.call(RANDOM_GENERATOR, &[]).u32();
    fn_004dff20(e, Ptr::new(generator), fld_copy(chance))
}

// Translated from 004dff20 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `chance` is not below the generator's next value in [0, 1)
/// (`004dff60`); false when the comparison is unordered (NaN).
pub fn fn_004dff20(e: &mut Engine, this: Ptr, chance: f32) -> bool {
    let random = fn_004dff60(e, this);
    // FCOMPP of `chance` with the random value: C0 (below or unordered)
    // gives false, so the result is an ordered `>=`.
    chance as f64 >= random as f64
}

// Translated from 004dff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The generator's next value (`00476be0`, unsigned) times `2^-32`, as a
/// `float`.
pub fn fn_004dff60(e: &mut Engine, this: Ptr) -> f32 {
    let value = e.call(RANDOM_NEXT, &args![this]).u32();
    let scale: f64 = e.global(SCALE_UNSIGNED_010229C0);
    (value as f64 * scale) as f32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004deb30, fn_004deb30()),
        entry!(0x004dee10, fn_004dee10(Ptr) -> u32),
        entry!(0x004dee70, fn_004dee70(Ptr) -> u32),
        entry!(0x004deed0, fn_004deed0(Ptr, u8) -> bool),
        entry!(0x004def00, fn_004def00(f32, u8) -> bool),
        entry!(0x004def20, fn_004def20()),
        entry!(0x004def90, fn_004def90(Ptr)),
        entry!(0x004defb0, fn_004defb0()),
        entry!(0x004df040, fn_004df040(u32, f32, f32) -> Ptr),
        entry!(0x004dfdd0, fn_004dfdd0(Ptr)),
        entry!(0x004dfe30, fn_004dfe30(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x004dfe80, fn_004dfe80(Ptr, u32, Ptr)),
        entry!(0x004dfea0, fn_004dfea0() -> f32),
        entry!(0x004dfec0, fn_004dfec0(Ptr) -> f32),
        entry!(0x004dff00, fn_004dff00(f32) -> bool),
        entry!(0x004dff20, fn_004dff20(Ptr, f32) -> bool),
        entry!(0x004dff60, fn_004dff60(Ptr) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn returns_this(_: &mut Engine, a: &[u32]) -> Ret {
        Ret {
            eax: a[0],
            ..Ret::default()
        }
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    /// The pages holding the globals and constants these functions read,
    /// with the exe's constant values.
    fn splatter_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0102_2000,
            0x0118_9000,
            0x011c_7000,
            0x011c_e000,
            0x011c_f000,
            0x011f_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(SCALE_SIGNED_010229B8, 2f64.powi(-31));
        e.set_global(SCALE_UNSIGNED_010229C0, 2f64.powi(-32));
        e.set_global(ONE_01012070, 1.0f64);
        e.set_global(ZERO_01012060, 0.0f64);
        e.set_global(HALF_01011588, 0.5f64);
        e.set_global(MINUS_HALF_01011598, -0.5f64);
        e.set_global(HALF_01016248, 0.5f32);
        e.set_global(MINUS_HALF_0102295C, -0.5f32);
        e.set_global(FAR_01022958, 10000.0f32);
        e.set_global(CAMERA_Z_01022960, f32::from_bits(0x3f80_0054));
        e.register(POINTER_ASSIGN, nothing);
        e.register(POINTER_GET, |e, a| eax(e.mem.u32(a[0])));
        e.register(SETTING_STRING, |_, a| eax(a[0] + 1));
        // A float setting's value sits at the setting's own address here.
        e.register(SETTING_FLOAT, returns_this);
        e.register(TEXTURE_LOAD, nothing);
        e
    }

    /// Random values handed out by `00476be0`, in order.
    fn random_sequence(e: &mut Engine, values: Vec<u32>) {
        e.register(RANDOM_GENERATOR, |_, _| eax(0x7777));
        let mut values = values.into_iter();
        e.register_double(RANDOM_NEXT, move |_, a| {
            assert_eq!(a[0], 0x7777);
            eax(values.next().expect("more random values"))
        });
    }

    #[test]
    fn init_builds_camera_frustum_and_node_when_enabled() {
        let mut e = splatter_engine();
        let enabled = e.mem.alloc(4);
        e.mem.set_u8(enabled, 1);
        e.register_double(0x0040_8d60, move |_, _| eax(enabled));
        for addr in [
            0x0040_4eb0,
            0x0040_4ee0,
            0x0044_0460,
            0x0043_d410,
            0x00a5_9c60,
            0x00a7_01b0,
            0x00a7_1b70,
            0x00a6_faf0,
        ] {
            e.register(addr, nothing);
        }
        e.register(OBJECT_ALLOCATE, |e, a| eax(e.mem.alloc(a[0])));
        e.register(0x00a7_12f0, returns_this);
        e.register(NODE_CONSTRUCT, returns_this);
        e.register(POINT3_CONSTRUCT, returns_this);
        // Screen object: virtual +0xC8 gives the second object, whose
        // virtual +0xB0 gives the display; width 1280, height 720.
        let vtable = 0x0100_0000;
        e.put_vtable(vtable, &[0; 0x33]);
        e.mem.set_u32(vtable + 0xc8, 0x0090_0001);
        e.mem.set_u32(vtable + 0xb0, 0x0090_0002);
        let screen = e.mem.alloc(4);
        e.mem.set_u32(screen, vtable);
        e.register_double(0x0043_c4b0, move |_, _| eax(screen));
        e.register(0x0090_0001, returns_this);
        e.register(0x0090_0002, returns_this);
        e.register(0x0044_ddc0, |_, _| eax(1280));
        e.register(0x0084_e3a0, |_, _| eax(720));
        e.set_global(0x011c_fc2cu32, 3.5f32);
        e.set_global(FADE_FLAG, 1u8);
        e.set_global(COUNT, 9u32);
        e.set_global(SECOND_LAYER, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x004d_eb30, &[]);
        let log = e.call_log.take().unwrap();

        assert_eq!(e.global::<u8>(ENABLED), 1);
        let guard = calls_to(&log, 0x0040_4eb0);
        assert_eq!(&guard[0][1..], &[7, 1, 0x0102_2968, 0xbd]);
        assert_eq!(calls_to(&log, 0x0040_4ee0), vec![vec![guard[0][0]]]);
        let assigns = calls_to(&log, POINTER_ASSIGN);
        assert_eq!(assigns.len(), 2);
        assert_eq!(assigns[0][0], CAMERA);
        assert_eq!(assigns[1][0], ROOT);
        let point = calls_to(&log, POINT3_CONSTRUCT);
        assert_eq!(&point[0][1..], &[0, 0, 0x3f80_0054]);
        assert_eq!(
            &calls_to(&log, 0x00a7_01b0)[0][1..],
            &[0x011f_426c, 0x011a_9478]
        );
        let aspect = (1280.0f64 / 720.0) as f32;
        let right = (aspect as f64 * 0.5) as f32;
        assert_eq!(e.global::<f32>(WIDTH_RADIUS), right);
        assert_eq!(e.global::<f32>(HEIGHT_RADIUS), 0.5);
        assert_eq!(e.global::<f32>(FADE_TIME), 3.5);
        assert_eq!(e.global::<u8>(FADE_FLAG), 0);
        assert_eq!(e.global::<u32>(COUNT), 0);
        let loads = calls_to(&log, TEXTURE_LOAD);
        assert_eq!(
            loads,
            vec![
                vec![TEXTURES + 4, 0x011c_e919],
                vec![TEXTURES, 0x011c_e285],
                vec![TEXTURES + 8, 0x011c_ede9]
            ]
        );
    }

    #[test]
    fn init_frustum_values() {
        // The frustum the camera gets, read inside the SetViewFrustum
        // double.
        let mut e = splatter_engine();
        let enabled = e.mem.alloc(4);
        e.mem.set_u8(enabled, 1);
        e.register_double(0x0040_8d60, move |_, _| eax(enabled));
        for addr in [
            0x0040_4eb0,
            0x0040_4ee0,
            0x0044_0460,
            0x0043_d410,
            0x00a5_9c60,
            0x00a7_01b0,
            0x00a7_1b70,
        ] {
            e.register(addr, nothing);
        }
        e.register(OBJECT_ALLOCATE, |_, _| eax(0));
        e.register(POINT3_CONSTRUCT, returns_this);
        let vtable = 0x0100_0000;
        e.put_vtable(vtable, &[0; 0x33]);
        e.mem.set_u32(vtable + 0xc8, 0x0090_0001);
        e.mem.set_u32(vtable + 0xb0, 0x0090_0002);
        let screen = e.mem.alloc(4);
        e.mem.set_u32(screen, vtable);
        e.register_double(0x0043_c4b0, move |_, _| eax(screen));
        e.register(0x0090_0001, returns_this);
        e.register(0x0090_0002, returns_this);
        e.register(0x0044_ddc0, |_, _| eax(800));
        e.register(0x0084_e3a0, |_, _| eax(600));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let seen_in = seen.clone();
        e.register_double(0x00a6_faf0, move |e, a| {
            *seen_in.borrow_mut() = (0..6).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            seen_in.borrow_mut().push(e.mem.u8(a[1] + 0x18) as f32);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x004d_eb30, &[]);
        let log = e.call_log.take().unwrap();
        let aspect = (800.0f64 / 600.0) as f32;
        assert_eq!(
            *seen.borrow(),
            vec![
                (aspect as f64 * -0.5) as f32,
                (aspect as f64 * 0.5) as f32,
                0.5,
                -0.5,
                1.0,
                10000.0,
                0.0
            ]
        );
        // Failed allocations: null camera and node, no constructor calls.
        assert!(calls_to(&log, 0x00a7_12f0).is_empty());
        assert!(calls_to(&log, NODE_CONSTRUCT).is_empty());
        assert_eq!(
            calls_to(&log, POINTER_ASSIGN),
            vec![vec![CAMERA, 0], vec![ROOT, 0]]
        );
        // Without the second layer, two textures.
        assert_eq!(calls_to(&log, TEXTURE_LOAD).len(), 2);
    }

    #[test]
    fn init_does_nothing_more_when_disabled() {
        let mut e = splatter_engine();
        let enabled = e.mem.alloc(4);
        e.register_double(0x0040_8d60, move |_, _| eax(enabled));
        e.register(0x0040_4eb0, nothing);
        e.register(0x0040_4ee0, nothing);
        e.set_global(ENABLED, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x004d_eb30, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u8>(ENABLED), 0);
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![0x004d_eb30, 0x0040_4eb0, 0x0040_8d60, 0x0040_4ee0]
        );
    }

    #[test]
    fn screen_size_queries_follow_the_virtual_chain() {
        let mut e = Engine::new();
        let vtable = 0x0100_0000;
        e.put_vtable(vtable, &[0; 0x33]);
        e.mem.set_u32(vtable + 0xc8, 0x0090_0001);
        e.mem.set_u32(vtable + 0xb0, 0x0090_0002);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        e.register(0x0090_0001, returns_this);
        e.register(0x0090_0002, |_, a| {
            assert_eq!(a[1], 0);
            eax(a[0])
        });
        e.register(0x0044_ddc0, |_, _| eax(1920));
        e.register(0x0084_e3a0, |_, _| eax(1080));
        assert_eq!(e.call(0x004d_ee10, &args![object]).u32(), 1920);
        assert_eq!(e.call(0x004d_ee70, &args![object]).u32(), 1080);
        // The second virtual returns null.
        e.register(0x0090_0002, |_, _| eax(0));
        assert_eq!(e.call(0x004d_ee10, &args![object]).u32(), 0);
        assert_eq!(e.call(0x004d_ee70, &args![object]).u32(), 0);
        // The first virtual returns null.
        e.register(0x0090_0001, |_, _| eax(0));
        assert_eq!(e.call(0x004d_ee10, &args![object]).u32(), 0);
        assert_eq!(e.call(0x004d_ee70, &args![object]).u32(), 0);
    }

    #[test]
    fn load_texture_checks_the_index() {
        let mut e = splatter_engine();
        e.call_log = Some(vec![]);
        assert!(e.call(0x004d_eed0, &args![0x1234u32, 2u32]).bool());
        assert!(!e.call(0x004d_eed0, &args![0x1234u32, 3u32]).bool());
        // Only the low byte of the index counts.
        assert!(e.call(0x004d_eed0, &args![0x1234u32, 0x100u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, TEXTURE_LOAD),
            vec![vec![TEXTURES + 8, 0x1234], vec![TEXTURES, 0x1234]]
        );
    }

    #[test]
    fn set_fade_stores_both_and_quiets_nan() {
        let mut e = splatter_engine();
        assert!(e.call(0x004d_ef00, &args![2.5f32, 0x101u32]).bool());
        assert_eq!(e.global::<f32>(FADE_TIME), 2.5);
        assert_eq!(e.global::<u8>(FADE_FLAG), 1);
        e.call(0x004d_ef00, &args![0x7f80_0001u32, 0u32]);
        assert_eq!(e.global::<u32>(FADE_TIME), 0x7fc0_0001);
    }

    #[test]
    fn shutdown_releases_node_camera_and_textures() {
        let mut e = splatter_engine();
        e.register(0x004d_ffa0, nothing);
        e.set_global(ROOT, 0x0055_0000u32);
        e.set_global(ENABLED, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x004d_ef20, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u8>(ENABLED), 0);
        assert_eq!(calls_to(&log, 0x004d_ffa0), vec![vec![0x0055_009c]]);
        assert_eq!(
            calls_to(&log, POINTER_ASSIGN),
            vec![
                vec![ROOT, 0],
                vec![CAMERA, 0],
                vec![TEXTURES, 0],
                vec![TEXTURES + 4, 0],
                vec![TEXTURES + 8, 0]
            ]
        );

        // Disabled: only the textures.
        e.call_log = Some(vec![]);
        e.call(0x004d_ef20, &[]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x004d_ffa0).is_empty());
        assert_eq!(calls_to(&log, POINTER_ASSIGN).len(), 3);
    }

    #[test]
    fn remove_children_empties_the_array_at_9c() {
        let mut e = Engine::new();
        e.register(0x004d_ffa0, nothing);
        e.call_log = Some(vec![]);
        e.call(0x004d_ef90, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x004d_ffa0), vec![vec![0x109c]]);
    }

    #[test]
    fn reset_clears_state_and_reloads_textures() {
        let mut e = splatter_engine();
        e.register(0x004d_ffa0, nothing);
        e.set_global(ENABLED, 1u8);
        e.set_global(COUNT, 5u32);
        e.set_global(FADE_TIME, 2.0f32);
        e.set_global(FADE_FLAG, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x004d_efb0, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u32>(COUNT), 0);
        assert_eq!(e.global::<f32>(FADE_TIME), 0.0);
        assert_eq!(e.global::<u8>(FADE_FLAG), 0);
        assert_eq!(calls_to(&log, 0x004d_ffa0).len(), 1);
        assert_eq!(calls_to(&log, TEXTURE_LOAD).len(), 2);

        // Disabled: nothing changes.
        e.set_global(ENABLED, 0u8);
        e.set_global(COUNT, 5u32);
        e.call_log = Some(vec![]);
        e.call(0x004d_efb0, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(e.global::<u32>(COUNT), 5);
    }

    /// Doubles for the geometry builder: allocations are real blocks, the
    /// constructors fill their objects, the rotation copies the point and
    /// the translation adds.
    fn geometry_engine(second_layer: bool) -> Engine {
        let mut e = splatter_engine();
        e.register(OBJECT_ALLOCATE, |e, a| eax(e.mem.alloc(a[0])));
        e.register(0x0040_1000, |e, a| eax(e.mem.alloc(a[0])));
        e.register(0x0040_1050, nothing);
        e.register(0x0068_15c0, returns_this);
        // The node gets a vtable whose +0xDC is a recorded double.
        e.put_vtable(0x0100_0000, &[0; 0x38]);
        e.mem.set_u32(0x0100_00dc, 0x0090_00dc);
        e.register(0x0090_00dc, nothing);
        e.register(NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0100_0000);
            eax(a[0])
        });
        e.register(0x00a7_4410, returns_this);
        e.register(0x00b8_1200, returns_this);
        e.register(0x0043_9410, nothing);
        e.register(0x00b5_7e30, nothing);
        e.register(POINT3_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            eax(a[0])
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            eax(a[0])
        });
        e.register(0x004b_4500, |e, a| {
            copy_words(e, a[2], a[1], 3);
            eax(a[1])
        });
        e.register(0x0043_9e90, |e, a| {
            for i in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, sum);
            }
            eax(a[1])
        });
        e.register(RANDOM_FLOAT, |_, a| Ret {
            st0: (f32::from_bits(a[0]) + f32::from_bits(a[1])) as f64,
            ..Ret::default()
        });
        e.set_global(WIDTH_RADIUS, 2.0f32);
        e.set_global(HEIGHT_RADIUS, 4.0f32);
        for (setting, value) in [
            (0x011c_faa0u32, 1.0f32),
            (0x011c_f9a4, 2.0),
            (0x011c_f980, 1.0),
            (0x011c_fae0, 1.0),
            (0x011c_f920, 0.25),
            (0x011c_eca0, 0.25),
            (0x011c_efe4, 1.0),
            (0x011c_f2c4, 0.75),
        ] {
            e.set_global(setting, value);
        }
        e.set_global(TEXTURES, 0x0000_7e01u32);
        e.set_global(TEXTURES + 4, 0x0000_7e02u32);
        e.set_global(TEXTURES + 8, 0x0000_7e03u32);
        e.set_global(SECOND_LAYER, second_layer as u8);
        // x = 0.5, y = -0.5, chance roll 0.0, u roll -1.0, v roll 0.5.
        random_sequence(&mut e, vec![0xc000_0000, 0x4000_0000, 0, 0, 0xc000_0000]);
        e
    }

    fn floats(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    fn u16s(e: &Engine, at: u32, n: u32) -> Vec<u16> {
        (0..n).map(|i| e.mem.u16(at + 2 * i)).collect()
    }

    #[test]
    fn geometry_builds_one_quad() {
        let mut e = geometry_engine(false);
        e.call_log = Some(vec![]);
        let node = e.call(0x004d_f040, &args![1u32, 2.0f32, 3.0f32]).u32();
        let log = e.call_log.take().unwrap();

        let arrays = calls_to(&log, 0x0040_1000);
        assert_eq!(
            arrays.iter().map(|w| w[0]).collect::<Vec<_>>(),
            vec![48, 32, 12]
        );
        let shapes = calls_to(&log, 0x00a7_4410);
        assert_eq!(shapes.len(), 1);
        let (positions, uvs, triangles) = (shapes[0][2], shapes[0][5], shapes[0][9]);
        assert_eq!(
            &shapes[0][1..],
            &[4, positions, 0, 0, uvs, 1, 0, 2, triangles]
        );
        // Size 3 * 2 = 6; the chance roll picks the second depth, 0.5 * 3;
        // translation (1, -2).
        assert_eq!(
            floats(&e, positions, 12),
            vec![-5.0, 4.0, 1.5, 7.0, 4.0, 1.5, -5.0, -8.0, 1.5, 7.0, -8.0, 1.5]
        );
        // Quarter (0.5, 0.0) of the texture.
        assert_eq!(
            floats(&e, uvs, 8),
            vec![1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.0]
        );
        assert_eq!(u16s(&e, triangles, 6), vec![0, 2, 1, 1, 2, 3]);

        // The random size and depth ranges, low bound first.
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT),
            vec![bits(&[2.0, 1.0]), bits(&[1.0, 1.0]), bits(&[0.25, 0.25])]
        );
        let shape = shapes[0][0];
        let property = calls_to(&log, 0x00b8_1200)[0][0];
        assert_eq!(
            calls_to(&log, POINTER_ASSIGN),
            vec![vec![property + 0x60, 0x7e02], vec![property + 0x64, 0x7e01]]
        );
        assert_eq!(calls_to(&log, 0x0043_9410), vec![vec![shape, property]]);
        assert_eq!(calls_to(&log, 0x00b5_7e30), vec![vec![shape, 0, 0]]);
        assert_eq!(calls_to(&log, 0x0090_00dc), vec![vec![node, shape, 1]]);
    }

    #[test]
    fn geometry_with_second_layer_builds_two_shapes() {
        let mut e = geometry_engine(true);
        e.call_log = Some(vec![]);
        let node = e.call(0x004d_f040, &args![1u32, 2.0f32, 3.0f32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0040_1000).len(), 6);
        let shapes = calls_to(&log, 0x00a7_4410);
        assert_eq!(shapes.len(), 2);
        let (positions, uvs, triangles) = (shapes[1][2], shapes[1][5], shapes[1][9]);
        assert_eq!(
            floats(&e, positions, 12),
            vec![0.25, -1.25, 1.5, 1.75, -1.25, 1.5, 0.25, -2.75, 1.5, 1.75, -2.75, 1.5]
        );
        assert_eq!(
            floats(&e, uvs, 8),
            vec![1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0]
        );
        assert_eq!(u16s(&e, triangles, 6), vec![0, 2, 1, 1, 2, 3]);
        let properties = calls_to(&log, 0x00b8_1200);
        assert_eq!(
            calls_to(&log, POINTER_ASSIGN)[2],
            vec![properties[1][0] + 0x60, 0x7e03]
        );
        assert_eq!(
            calls_to(&log, 0x0090_00dc),
            vec![vec![node, shapes[0][0], 1], vec![node, shapes[1][0], 1]]
        );
    }

    #[test]
    fn geometry_with_no_splats_still_adds_an_empty_shape() {
        let mut e = geometry_engine(false);
        e.call_log = Some(vec![]);
        e.call(0x004d_f040, &args![0u32, 1.0f32, 1.0f32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, RANDOM_NEXT).is_empty());
        assert_eq!(calls_to(&log, 0x00a7_4410)[0][1], 0);
        assert_eq!(calls_to(&log, 0x0090_00dc).len(), 1);
    }

    #[test]
    fn array_size_saturates_on_overflow() {
        assert_eq!(array_size(4, 12), 48);
        assert_eq!(array_size(0x2000_0000, 8), u32::MAX);
    }

    #[test]
    fn identity_matrix() {
        let mut e = Engine::new();
        let m = e.mem.alloc(36);
        for i in 0..9 {
            e.mem.set_f32(m + 4 * i, 7.0);
        }
        e.call(0x004d_fdd0, &args![m]);
        assert_eq!(
            floats(&e, m, 9),
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
    }

    #[test]
    fn pair_sum() {
        let mut e = Engine::new();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_f32(a, 1.5);
        e.mem.set_f32(a + 4, -2.0);
        e.mem.set_f32(b, 0.25);
        e.mem.set_f32(b + 4, 3.0);
        // The result may be the first operand.
        assert_eq!(e.call(0x004d_fe30, &args![a, a, b]).u32(), a);
        assert_eq!(floats(&e, a, 2), vec![1.75, 1.0]);
    }

    #[test]
    fn property_texture_slot() {
        let mut e = Engine::new();
        e.register(POINTER_ASSIGN, nothing);
        e.call_log = Some(vec![]);
        e.call(0x004d_fe80, &args![0x1000u32, 2u32, 0x4444u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, POINTER_ASSIGN), vec![vec![0x1068, 0x4444]]);
    }

    #[test]
    fn signed_random() {
        let mut e = splatter_engine();
        random_sequence(&mut e, vec![0, 0x8000_0000, 0xffff_ffff]);
        assert_eq!(e.call(0x004d_fea0, &[]).f32(), -1.0);
        assert_eq!(e.call(0x004d_fec0, &args![0x7777u32]).f32(), 0.0);
        // 2^32 - 1 scaled rounds to 1.0 as a float.
        assert_eq!(e.call(0x004d_fea0, &[]).f32(), 1.0);
    }

    #[test]
    fn unsigned_random() {
        let mut e = splatter_engine();
        random_sequence(&mut e, vec![0x4000_0000, 0xffff_ffff]);
        assert_eq!(e.call(0x004d_ff60, &args![0x7777u32]).f32(), 0.25);
        assert_eq!(e.call(0x004d_ff60, &args![0x7777u32]).f32(), 1.0);
    }

    #[test]
    fn chance_compares_with_the_roll() {
        let mut e = splatter_engine();
        // Rolls 0.5, 0.5, 0.5, 0.5.
        random_sequence(&mut e, vec![0x8000_0000; 4]);
        assert!(e.call(0x004d_ff00, &args![0.5f32]).bool());
        assert!(!e.call(0x004d_ff00, &args![0.25f32]).bool());
        assert!(e.call(0x004d_ff20, &args![0x7777u32, 0.75f32]).bool());
        assert!(!e.call(0x004d_ff20, &args![0x7777u32, f32::NAN]).bool());
    }
}
