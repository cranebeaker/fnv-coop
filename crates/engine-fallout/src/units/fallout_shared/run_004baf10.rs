//! `fallout shared/run_004baf10` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `004baf10` is a method of the object whose grid-cell array `004ba490`
//! (`GridCellArray::Get`, engine map) reads: the word at `+0x0c` is the
//! number of cells per side (`0084e3a0`), the cell pointers start at the
//! array at `+0x10`, and the word at `+0x24` is a smart pointer to a
//! rendered texture. The class itself is unconfirmed (the engine map has no
//! name for these functions).
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float`. A `float` copied through `FLD`/`FSTP` comes out
//! with a signalling NaN made quiet (bit 22 set); [`copied_float_bits`] does
//! that wherever the code copies one.

#[allow(unused_imports)]
use crate::prelude::*;

/// Scope guard constructor (`00404eb0`, `thiscall`, four stack words:
/// `0x20`, a flag, a source file name, a line number).
const GUARD_ENTER: u32 = 0x0040_4eb0;
/// Scope guard destructor (`00404ee0`, `fastcall`).
const GUARD_LEAVE: u32 = 0x0040_4ee0;
/// Smart pointer constructor from a pointer (`00633c90`, one stack word):
/// stores the pointer and adds a reference.
const POINTER_FROM: u32 = 0x0063_3c90;
/// Smart pointer assignment (`0066b0d0`, one stack word): releases the old
/// referent and references the new one.
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// Smart pointer release (`0045cec0`): drops the reference held, if any.
const POINTER_RELEASE: u32 = 0x0045_cec0;
/// Smart pointer read (`00559450`): the pointer held.
const POINTER_GET: u32 = 0x0055_9450;
/// Default constructor of a 12-byte point, also used as the element
/// constructor of the point arrays (`006815c0`: returns `this`).
const POINT_DEFAULT: u32 = 0x0068_15c0;
/// Constructor of a 16-byte object from four floats (`00414430`, `RET 0x10`).
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
/// `NiFrustum::NiFrustum` (engine map; `00a71b70`, one stack word).
const FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
/// Number of cells per side of the grid (`0084e3a0`: the word at `+0x0c`).
const GRID_SIDE: u32 = 0x0084_e3a0;
/// Reads the byte at `+0` of the object at `0x011c75a4`'s `00408d60` result
/// (`004bc400`): whether the offscreen mask can be built.
const MASK_BUILD_ENABLED: u32 = 0x004b_c400;
/// `00452dc0` (`thiscall` on a caller-provided 8-byte object, two floats):
/// writes the two floats into it and returns it.
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
/// `00416870` (`thiscall` on a caller-provided 12-byte object, three
/// floats): writes the three floats into it and returns it.
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// Stores the two words (`004bc420`, `cdecl`) into the globals at
/// `0x011fd874`/`0x011fd878` (as floats).
const SET_MASK_ORIGIN: u32 = 0x004b_c420;
/// Stores the float word (`004bc440`, `cdecl`) into the global at
/// `0x011fd880`.
const SET_MASK_SIZE: u32 = 0x004b_c440;
/// Reads the word at `0x011f91d8` (`004bc3e0`).
const GET_MASK_FLAGS: u32 = 0x004b_c3e0;
/// Writes the word at `0x011f91d8` (`004bc3d0`, `cdecl`).
const SET_MASK_FLAGS: u32 = 0x004b_c3d0;
/// The object held by the smart pointer global at `0x011c73b4` (`004bc3f0`);
/// the code calls slots `0xac` and `0xb4` of its vtable.
const GET_RENDERER: u32 = 0x004b_c3f0;
/// Stores the word in the smart pointer global at `0x011f95e8`
/// (`00451570`, `cdecl`): the mask texture.
const SET_MASK_TEXTURE: u32 = 0x0045_1570;
/// Allocator behind `operator new` for scene objects (`00aa13e0`, `cdecl`).
const ALLOCATE_OBJECT: u32 = 0x00aa_13e0;
/// The allocator `00401000` (`cdecl`) used for the vertex arrays.
const ALLOCATE_ARRAY: u32 = 0x0040_1000;
/// The allocator `00aa1070` (`cdecl`) used for the index arrays.
const ALLOCATE_INDICES: u32 = 0x00aa_1070;
/// `` `vector constructor iterator' `` (`00401050`, `stdcall`, four words:
/// the array, the element size, the element count, the element
/// constructor): calls the constructor on every element.
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// `NiNode` constructor (`00a5ecb0`, one stack word; engine map).
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `NiCamera` constructor (`00a712f0`; engine map).
const CAMERA_CONSTRUCT: u32 = 0x00a7_12f0;
/// `NiDirectionalLight::NiDirectionalLight` (engine map; `00a75c20`).
const LIGHT_CONSTRUCT: u32 = 0x00a7_5c20;
/// `NiMaterialProperty::NiMaterialProperty` (engine map; `00a75650`).
const MATERIAL_CONSTRUCT: u32 = 0x00a7_5650;
/// Constructor of a 0x24-byte object (`0049ec80`; engine map: in
/// `bgsautowater.cpp`), whose setter `0049ee30` gets 3.
const PROPERTY_CONSTRUCT: u32 = 0x0049_ec80;
/// The setter `0049ee30` (one stack word) the code calls with 3.
const PROPERTY_SET_MODE: u32 = 0x0049_ee30;
/// `BSShaderAccumulator::BSShaderAccumulator` (engine map; `00b660d0`,
/// three stack words).
const ACCUMULATOR_CONSTRUCT: u32 = 0x00b6_60d0;
/// `BSShaderNoLightingProperty::BSShaderNoLightingProperty` (`00b6fc90`).
const NO_LIGHTING_CONSTRUCT: u32 = 0x00b6_fc90;
/// `NiTriStrips::NiTriStrips_ov2` (engine map; `00a71cc0`, one stack word:
/// the geometry data).
const TRI_STRIPS_CONSTRUCT: u32 = 0x00a7_1cc0;
/// Geometry data constructor (`00a75dc0`, eleven stack words): vertex
/// count, vertices, normals, colours, texture coordinates, and so on.
const GEOMETRY_DATA_CONSTRUCT: u32 = 0x00a7_5dc0;
/// `GridCellArray::Get` (engine map, `004ba490`, two stack words): the
/// address of the cell pointer for (row, column), or 0 when out of range.
const GRID_CELL_GET: u32 = 0x004b_a490;
/// `005477f0` (`thiscall` on a cell, two out pointers): fills the two words
/// and returns a state (1: build a property from the first word, 2: the cell
/// has none).
const CELL_TEXTURE_STATE: u32 = 0x0054_77f0;
/// `D3DTexture_LockRect` (engine map; `0059bb30`): the Direct3D texture
/// object of a texture wrapper.
const TEXTURE_OBJECT: u32 = 0x0059_bb30;
/// `NiAVObject::AttachProperty` (engine map; `00439410`, one stack word).
const ATTACH_PROPERTY: u32 = 0x0043_9410;
/// `00438230`: sets the texture of a property (one stack word).
const PROPERTY_SET_TEXTURE: u32 = 0x0043_8230;
/// Sets the translation of a scene object (`00440460`, one stack word: the
/// address of three floats).
const SET_TRANSLATION: u32 = 0x0044_0460;
/// `BSShaderManager::PrepareObject` (engine map; `00b57e30`, `cdecl`, three
/// words).
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// Constructor of a 12-byte object from a float and two bytes (`0043d410`,
/// `RET 0xc`) that the code passes to `00a59c60`.
const ROTATION_CONSTRUCT: u32 = 0x0043_d410;
/// Sets the rotation of a scene object (`00a59c60`, one stack word).
const SET_ROTATION: u32 = 0x00a5_9c60;
/// `NiCamera::LookAtWorldPoint` (engine map; `00a701b0`, two stack words).
const LOOK_AT: u32 = 0x00a7_01b0;
/// `NiCamera::SetViewFrustum` (engine map; `00a6faf0`, one stack word).
const SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
/// Attaches a node to an effect (`00a89af0`, one stack word).
const ATTACH_AFFECTED_NODE: u32 = 0x00a8_9af0;
/// `004534f0` (one stack word: the address of three floats) on the material.
const MATERIAL_SET_COLOUR: u32 = 0x0045_34f0;
/// `NiAVObject::UpdateProperties` (engine map; `00a5a040`).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// A function returning the word at `0x011f4748` (`0043c4b0`).
const FIRST_STATIC_OBJECT: u32 = 0x0043_c4b0;
/// A function returning the word at `0x011f91a8` (`004a0ea0`).
const SECOND_STATIC_OBJECT: u32 = 0x004a_0ea0;
/// Builds an object from the second static object (`00b6e110`, `RET 0x14`,
/// five words: the first static object, `0x17` and three zeros).
const BUILD_FROM_STATIC_OBJECTS: u32 = 0x00b6_e110;
/// `00822510` on a smart pointer: compares the pointer held with the word.
const POINTER_EQUALS: u32 = 0x0082_2510;
/// `TESRegionDataLandscape::GetDefaultFullMaskTextureProp` (engine map;
/// `004f2ee0`).
const DEFAULT_MASK_PROPERTY: u32 = 0x004f_2ee0;
/// Constructor of the 0xcc-byte object the scene is accumulated with
/// (`004a0eb0`, one stack word).
const SCENE_STATE_CONSTRUCT: u32 = 0x004a_0eb0;
/// `BSCullingProcess::~BSCullingProcess` (engine map; `004a0f60`): the
/// destructor of the 0xcc-byte object.
const SCENE_STATE_DESTRUCT: u32 = 0x004a_0f60;
/// Sets the accumulator of the 0xcc-byte object (`004a0fd0`, one stack word).
const SCENE_STATE_SET_ACCUMULATOR: u32 = 0x004a_0fd0;
/// Sets the byte at `+0x84` of the accumulator (`004bc4a0`, one stack word).
const ACCUMULATOR_SET_FLAG: u32 = 0x004b_c4a0;
/// `BSRenderedTexture::StopOffscreen` (engine map; `00b6b260`).
const RENDERED_TEXTURE_STOP: u32 = 0x00b6_b260;
/// Takes the mode 7 and the stopped texture (`00b6b8d0`, `cdecl`).
const START_OFFSCREEN: u32 = 0x00b6_b8d0;
/// `BSShaderUtil::AccumulateScene` (engine map; `00b6bee0`, `cdecl`, three
/// words: camera, node, the 0xcc-byte object).
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
/// Renders with an accumulator (`00b6c0d0`, `cdecl`, three words: camera,
/// accumulator, 0).
const RENDER_ACCUMULATED: u32 = 0x00b6_c0d0;
/// Ends the offscreen pass (`00b6b790`).
const END_OFFSCREEN: u32 = 0x00b6_b790;
/// `BSRenderedTexture::GetTexture` (engine map; `004bc320`, one stack word).
const RENDERED_TEXTURE_GET: u32 = 0x004b_c320;
/// `BSShaderManager::GetAccumulator` (engine map; `00b4f5c0`: returns the
/// word at `0x011f95ec`). The game pushes a 1 that this function ignores and
/// the next call, `004bc4a0`, consumes.
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
/// `004bc450`: copies three floats to `+0x28` and calls `004bc480(1)`.
const MATERIAL_SET_AMBIENT: u32 = 0x004b_c450;

/// The source file name passed to the scope guard
/// (`D:\_Fallout3\Platforms\Common\Code\Fallout Shared\GridCell.cpp`).
const GUARD_SOURCE_FILE: u32 = 0x0102_06c0;
/// The byte the function clears at start (`0x01189184`).
const MASK_STATE_FLAG: u32 = 0x0118_9184;
/// `2.0` (`double`).
const TWO: u32 = 0x0101_1590;
/// `2048.0` (`double`).
const CENTRE_2048: u32 = 0x0101_6968;
/// `-1.0` (`double`).
const MINUS_ONE: u32 = 0x0101_a6b0;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// The frustum's far plane (`float`, `2.0` in the exe).
const FAR_PLANE: u32 = 0x0101_62c0;
/// Three `float`s (`0, 0, 1.0` in the exe) used as the normal of every
/// vertex.
const NORMAL: u32 = 0x011a_9484;
/// Three `float`s (`1.0` each in the exe): the light's colour.
const LIGHT_COLOUR: u32 = 0x011a_9b7c;
/// Three `float`s (`0.0` each in the exe): the renderer's new background.
const BACKGROUND_COLOUR: u32 = 0x011a_9bd0;
/// Three `float`s (zero in the exe): the point the camera looks at.
const LOOK_AT_TARGET: u32 = 0x011f_426c;
/// Three `float`s (`0, 1.0, 0` in the exe): the camera's up vector.
const LOOK_AT_UP: u32 = 0x011a_9478;

// Frame slots of `004baf10` (offsets from its `EBP`).
const GUARD: i32 = -0x148;
const POINTER_A: i32 = -0x60;
const POINTER_B: i32 = -0x170;
const POINTER_C: i32 = -0x134;
const POINTER_D: i32 = -0x16c;
const POINTER_E: i32 = -0x164;
const POINTER_F: i32 = -0x13c;
const POINTER_G: i32 = -0x174;
const SCENE_STATE: i32 = -0x12c;
const FRUSTUM: i32 = -0x44;
const FRUSTUM_ORTHO: i32 = -0x2c;
const POSITION_X: i32 = -0x1ac;
const POSITION_Y: i32 = -0x1a8;
const POSITION_Z: i32 = -0x1a4;
const SAVED_COLOUR: i32 = -0x1a0;
const HALF_SIDE: i32 = -0x130;
const CELL_TEXTURE: i32 = -0x138;
const CELL_TEXTURE_INFO: i32 = -0x140;
/// Scratch for the objects the point/rotation constructors fill in (the game
/// has one local for each call).
const SCRATCH: i32 = 0x20;

fn at(frame: u32, offset: i32) -> u32 {
    frame.wrapping_add_signed(offset)
}

/// A `float` copied through the x87 stack (`FLD` then `FSTP`): a signalling
/// NaN comes out quiet (bit 22 set).
fn copied_float_bits(value: f32) -> u32 {
    let bits = value.to_bits();
    if value.is_nan() {
        bits | 0x0040_0000
    } else {
        bits
    }
}

fn pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(POINTER_GET, &args![slot]).u32()
}

fn pointer_assign(e: &mut Engine, slot: u32, value: u32) {
    e.call(POINTER_ASSIGN, &args![slot, value]);
}

/// `operator new` through `00aa13e0`, then the constructor (the game uses its
/// result, so a constructor that returns something else is honoured); a null
/// allocation gives null.
fn new_object(e: &mut Engine, size: u32, constructor: u32, arguments: &[u32]) -> u32 {
    let object = e.call(ALLOCATE_OBJECT, &args![size]).u32();
    if object == 0 {
        return 0;
    }
    let mut words = vec![object];
    words.extend_from_slice(arguments);
    e.call(constructor, &words).u32()
}

/// A freshly allocated array of `count` default-constructed points of
/// `element` bytes (the allocation `00401000`, then `00401050`).
fn new_point_array(e: &mut Engine, bytes: u32, element: u32) -> u32 {
    let array = e.call(ALLOCATE_ARRAY, &args![bytes]).u32();
    if array != 0 {
        e.call(
            VECTOR_CONSTRUCTOR,
            &args![array, element, 4u32, POINT_DEFAULT],
        );
    }
    array
}

/// Copies the three words at `source` to `destination` (integer moves).
fn copy_words(e: &mut Engine, destination: u32, source: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(destination + 4 * i, word);
    }
}

/// Builds a point with `00416870` in the scratch object and stores its three
/// words at `destination`.
fn store_point3(e: &mut Engine, scratch: u32, destination: u32, x: f32, y: f32, z: f32) {
    let point = e
        .call(
            POINT3_CONSTRUCT,
            &args![scratch, x.to_bits(), y.to_bits(), z.to_bits()],
        )
        .u32();
    copy_words(e, destination, point, 3);
}

/// Builds a 2D point with `00452dc0` and stores its two words.
fn store_point2(e: &mut Engine, scratch: u32, destination: u32, x: f32, y: f32) {
    let point = e
        .call(POINT2_CONSTRUCT, &args![scratch, x.to_bits(), y.to_bits()])
        .u32();
    copy_words(e, destination, point, 2);
}

/// Releases the six smart pointers and the scope guard in the order every
/// exit of `004baf10` does.
fn release_locals(e: &mut Engine, frame: u32) {
    for offset in [
        POINTER_F, POINTER_E, POINTER_D, POINTER_C, POINTER_B, POINTER_A,
    ] {
        e.call(POINTER_RELEASE, &args![at(frame, offset)]);
    }
    e.call(GUARD_LEAVE, &args![at(frame, GUARD)]);
}

/// Clears bit 5 of the mask flags and empties the mask texture: the exit
/// `004baf10` takes when the grid has no cells or there is no renderer, and
/// when no cell had a texture state.
fn disable_mask(e: &mut Engine) {
    let flags = e.call(GET_MASK_FLAGS, &args![]).u32();
    e.call(SET_MASK_FLAGS, &args![flags & 0xffff_ffdf]);
    e.call(SET_MASK_TEXTURE, &args![0u32]);
}

// Translated from 004baf10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004baf10 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Builds the mask texture of the grid (the source file the scope guard
/// names is `GridCell.cpp`, line 0x220). The engine map has no name for it.
///
/// `this` (`ECX`, no stack words) is the grid object. When the mask can be
/// built (`004bc400`) the function sets the mask origin and size from the
/// grid, sets bit 5 of the mask flags, and, if the grid has cells and a
/// renderer exists, builds a unit quad (four points, normals, texture
/// coordinates and a four-vertex strip) and, for every cell, asks
/// `005477f0` for its texture state: state 1 with a texture makes a
/// `BSShaderNoLightingProperty` for the cell (after unlocking the cell's
/// Direct3D texture when its info word is set), which is attached to a
/// `NiTriStrips` placed at the cell's position under a root node; state 2
/// only counts as a texture state. Afterwards: if the first flag is still set
/// (every cell kept the defaults; the code clears it for every cell of a
/// grid that has cells, so this branch cannot run) the default mask
/// property is used (`004f2ee0`, `004bc240`, `00451570`); if no cell had a
/// texture state of 1 (with a texture) or 2 the mask is cleared; otherwise the scene is rendered offscreen: a camera
/// looking at the origin with an orthographic frustum of the grid's size, a
/// directional light, a material and a second property are attached to the
/// root, the renderer's background colour is saved and set, the scene is
/// accumulated and rendered into the grid's texture (created at `+0x24` when
/// empty), and the background colour and accumulator flag are restored.
///
/// The smart pointers and the scope guard are released on every exit. The
/// exception-unwinding frame is not translated. The decompiler hangs the
/// words pushed for `00b6e110` on `0043c4b0`/`004a0ea0`, which take none.
pub fn fn_004baf10(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let frame = e.mem.alloc(0x200);
    let ebp = frame + 0x1c0;
    e.call(
        GUARD_ENTER,
        &args![at(ebp, GUARD), 0x20u32, 1u32, GUARD_SOURCE_FILE, 0x220u32],
    );
    for offset in [POINTER_A, POINTER_B, POINTER_C, POINTER_D, POINTER_E] {
        e.call(POINTER_FROM, &args![at(ebp, offset), 0u32]);
    }
    e.call(FRUSTUM_CONSTRUCT, &args![at(ebp, FRUSTUM), 0u32]);
    e.call(POINTER_FROM, &args![at(ebp, POINTER_F), 0u32]);
    e.call(POINT_DEFAULT, &args![at(ebp, POSITION_X)]);
    e.call(
        COLOR_CONSTRUCT,
        &args![at(ebp, SAVED_COLOUR), 0u32, 0u32, 0u32, 0u32],
    );
    let count = e.call(GRID_SIDE, &args![this]).u32();
    let mut everything_default = true;
    let mut no_cell_state = true;
    e.mem.set_u8(MASK_STATE_FLAG, 0);
    if !e.call(MASK_BUILD_ENABLED, &args![]).bool() {
        release_locals(e, ebp);
        e.mem.free(frame);
        return;
    }

    // The grid's size in world units and its corner.
    let side = e.call(GRID_SIDE, &args![this]).u32();
    let size = ((side << 12) as f64) as f32;
    let two = e.global::<f64>(TWO);
    let corner = (e.global::<f64>(CENTRE_2048) - size as f64 / two) as f32;
    let origin_y = (e.mem.f32(this + 0x18) as f64 + corner as f64) as f32;
    let origin_x = (e.mem.f32(this + 0x14) as f64 + corner as f64) as f32;
    let origin = e
        .call(
            POINT2_CONSTRUCT,
            &args![
                at(ebp, SCRATCH),
                copied_float_bits(origin_x),
                copied_float_bits(origin_y)
            ],
        )
        .u32();
    let origin_words = (e.mem.u32(origin), e.mem.u32(origin + 4));
    e.call(SET_MASK_ORIGIN, &args![origin_words.0, origin_words.1]);
    e.call(SET_MASK_SIZE, &args![copied_float_bits(size)]);
    let flags = e.call(GET_MASK_FLAGS, &args![]).u32();
    e.call(SET_MASK_FLAGS, &args![flags | 0x20]);
    if count == 0 || e.call(GET_RENDERER, &args![]).u32() == 0 {
        disable_mask(e);
        release_locals(e, ebp);
        e.mem.free(frame);
        return;
    }

    // The root node, the quad and the per-cell objects.
    let half = (count as f64 / two) as f32;
    e.mem.set_f32(at(ebp, HALF_SIDE), half);
    let minus_one = e.global::<f64>(MINUS_ONE);
    let one = e.global::<f64>(ONE);
    let low = (half as f64 * minus_one) as f32;
    e.mem.set_f32(at(ebp, POSITION_Y), low);
    e.mem.set_u32(at(ebp, POSITION_X), copied_float_bits(low));
    e.mem.set_f32(at(ebp, POSITION_Z), 0.0);
    let root = new_object(e, 0xac, NODE_CONSTRUCT, &[0]);
    pointer_assign(e, at(ebp, POINTER_A), root);

    let vertices = new_point_array(e, 0x30, 0xc);
    store_point3(e, at(ebp, SCRATCH), vertices, 0.0, 0.0, 0.0);
    store_point3(e, at(ebp, SCRATCH), vertices + 0xc, 1.0, 0.0, 0.0);
    store_point3(e, at(ebp, SCRATCH), vertices + 0x18, 1.0, 1.0, 0.0);
    store_point3(e, at(ebp, SCRATCH), vertices + 0x24, 0.0, 1.0, 0.0);
    let normals = new_point_array(e, 0x30, 0xc);
    copy_words(e, normals + 0x24, NORMAL, 3);
    copy_words(e, normals + 0x18, normals + 0x24, 3);
    copy_words(e, normals + 0xc, normals + 0x18, 3);
    copy_words(e, normals, normals + 0xc, 3);
    let texture_coordinates = new_point_array(e, 0x20, 8);
    store_point2(e, at(ebp, SCRATCH), texture_coordinates, 0.0, 0.0);
    store_point2(e, at(ebp, SCRATCH), texture_coordinates + 8, 1.0, 0.0);
    store_point2(e, at(ebp, SCRATCH), texture_coordinates + 0x10, 1.0, 1.0);
    store_point2(e, at(ebp, SCRATCH), texture_coordinates + 0x18, 0.0, 1.0);
    let indices = e.call(ALLOCATE_INDICES, &args![8u32]).u32();
    for (i, index) in [1u16, 0, 2, 3].into_iter().enumerate() {
        e.mem.set_u16(indices + 2 * i as u32, index);
    }
    let strip_lengths = e.call(ALLOCATE_INDICES, &args![2u32]).u32();
    e.mem.set_u16(strip_lengths, 4);
    let data = new_object(
        e,
        0x50,
        GEOMETRY_DATA_CONSTRUCT,
        &[
            4,
            vertices,
            normals,
            0,
            texture_coordinates,
            1,
            0,
            2,
            1,
            strip_lengths,
            indices,
        ],
    );
    pointer_assign(e, at(ebp, POINTER_D), data);

    for row in 0..count {
        for column in 0..count {
            let mut property = 0u32;
            let slot = e.call(GRID_CELL_GET, &args![this, row, column]).u32();
            let cell = e.mem.u32(slot);
            if cell != 0 {
                let state = e
                    .call(
                        CELL_TEXTURE_STATE,
                        &args![cell, at(ebp, CELL_TEXTURE), at(ebp, CELL_TEXTURE_INFO)],
                    )
                    .u32();
                let texture = e.mem.u32(at(ebp, CELL_TEXTURE));
                let info = e.mem.u32(at(ebp, CELL_TEXTURE_INFO));
                if texture != 0 && info != 0 && e.mem.u32(info + 4) != 0 {
                    // Unlock the Direct3D texture: `texture->GetTexture()`
                    // slot 0x9c, then the COM method at slot 0x50 with level 0.
                    let wrapper = e.call(TEXTURE_OBJECT, &args![texture]).u32();
                    let direct3d = e.vcall(wrapper, 0x9c, &args![]).u32();
                    let method = e.mem.u32(e.mem.u32(direct3d) + 0x50);
                    e.call(method, &args![direct3d, 0u32]);
                    e.mem.set_u32(info + 4, 0);
                }
                if state == 1 && e.mem.u32(at(ebp, CELL_TEXTURE)) != 0 {
                    property = new_object(e, 0x80, NO_LIGHTING_CONSTRUCT, &[]);
                    let texture = e.mem.u32(at(ebp, CELL_TEXTURE));
                    e.call(PROPERTY_SET_TEXTURE, &args![property, texture]);
                    e.vcall(property, 0xd0, &args![0u32]);
                    no_cell_state = false;
                    everything_default = false;
                } else if state == 2 {
                    no_cell_state = false;
                }
            }
            if property == 0 {
                let y = e.mem.f32(at(ebp, POSITION_Y));
                e.mem.set_f32(at(ebp, POSITION_Y), (y as f64 + one) as f32);
                everything_default = false;
                continue;
            }
            let data = pointer_get(e, at(ebp, POINTER_D));
            let strips = new_object(e, 0xc4, TRI_STRIPS_CONSTRUCT, &[data]);
            pointer_assign(e, at(ebp, POINTER_C), strips);
            let strips = pointer_get(e, at(ebp, POINTER_C));
            e.call(SET_TRANSLATION, &args![strips, at(ebp, POSITION_X)]);
            let strips = pointer_get(e, at(ebp, POINTER_C));
            e.call(ATTACH_PROPERTY, &args![strips, property]);
            let strips = pointer_get(e, at(ebp, POINTER_C));
            e.call(PREPARE_OBJECT, &args![strips, 0u32, 0u32]);
            let root = pointer_get(e, at(ebp, POINTER_A));
            let strips = pointer_get(e, at(ebp, POINTER_C));
            // Attach the strips to the root (vtable slot 0xdc: child, flag).
            e.vcall(root, 0xdc, &args![strips, 1u32]);
            let y = e.mem.f32(at(ebp, POSITION_Y));
            e.mem.set_f32(at(ebp, POSITION_Y), (y as f64 + one) as f32);
        }
        let x = e.mem.f32(at(ebp, POSITION_X));
        e.mem.set_f32(at(ebp, POSITION_X), (x as f64 + one) as f32);
        let reset = (e.mem.f32(at(ebp, HALF_SIDE)) as f64 * minus_one) as f32;
        e.mem.set_f32(at(ebp, POSITION_Y), reset);
    }

    if everything_default {
        let holder = e.call(DEFAULT_MASK_PROPERTY, &args![]).u32();
        let mask = fn_004bc240(e, Ptr::new(holder));
        e.call(SET_MASK_TEXTURE, &args![mask]);
    } else if no_cell_state {
        // No cell had a texture state: clear the mask.
        disable_mask(e);
    } else {
        render_mask(e, this, ebp);
    }
    release_locals(e, ebp);
    e.mem.free(frame);
}

/// The part of `004baf10` that renders the mask: every cell had a texture
/// state and at least one made a property.
fn render_mask(e: &mut Engine, this: u32, ebp: u32) {
    let root_node = new_object(e, 0xac, NODE_CONSTRUCT, &[0]);
    pointer_assign(e, at(ebp, POINTER_B), root_node);
    let camera = new_object(e, 0x114, CAMERA_CONSTRUCT, &[]);
    pointer_assign(e, at(ebp, POINTER_E), camera);

    // The camera hangs under the second node, which sits one unit above.
    let second_node = pointer_get(e, at(ebp, POINTER_B));
    let camera = pointer_get(e, at(ebp, POINTER_E));
    e.vcall(second_node, 0xdc, &args![camera, 1u32]);
    let second_node = pointer_get(e, at(ebp, POINTER_B));
    fn_004bc1f0(e, Ptr::new(second_node), 0.0, 0.0, 1.0);
    let rotation = |e: &mut Engine, slot: i32, target: i32| {
        let scratch = at(ebp, slot);
        e.call(ROTATION_CONSTRUCT, &args![scratch, 0u32, 0u32, 0u32]);
        let object = pointer_get(e, at(ebp, target));
        e.call(SET_ROTATION, &args![object, scratch]);
    };
    rotation(e, -0x24, POINTER_B);
    let camera = pointer_get(e, at(ebp, POINTER_E));
    e.call(LOOK_AT, &args![camera, LOOK_AT_TARGET, LOOK_AT_UP]);
    rotation(e, -0x18, POINTER_E);
    rotation(e, -0x54, POINTER_B);

    // The orthographic frustum: the grid size on both axes.
    let frustum = at(ebp, FRUSTUM);
    e.mem.set_u8(at(ebp, FRUSTUM_ORTHO), 1);
    let half = e.mem.f32(at(ebp, HALF_SIDE));
    let minus_one = e.global::<f64>(MINUS_ONE);
    let left = (half as f64 * minus_one) as f32;
    e.mem.set_f32(frustum, left);
    e.mem.set_u32(frustum + 0xc, copied_float_bits(left));
    e.mem.set_u32(frustum + 4, copied_float_bits(half));
    let right = e.mem.f32(frustum + 4);
    e.mem.set_u32(frustum + 8, copied_float_bits(right));
    e.mem.set_f32(frustum + 0x10, 1.0);
    let far_plane = e.global::<f32>(FAR_PLANE);
    e.mem.set_u32(frustum + 0x14, copied_float_bits(far_plane));
    let camera = pointer_get(e, at(ebp, POINTER_E));
    e.call(SET_VIEW_FRUSTUM, &args![camera, frustum]);

    // The light.
    let light = new_object(e, 0xfc, LIGHT_CONSTRUCT, &[]);
    pointer_assign(e, at(ebp, POINTER_F), light);
    let light = pointer_get(e, at(ebp, POINTER_F));
    fn_004bc280(e, Ptr::new(light), Ptr::new(LIGHT_COLOUR));
    let light = pointer_get(e, at(ebp, POINTER_F));
    fn_004bc2e0(e, Ptr::new(light), Ptr::new(LIGHT_COLOUR));
    let root = pointer_get(e, at(ebp, POINTER_A));
    let light = pointer_get(e, at(ebp, POINTER_F));
    e.call(ATTACH_AFFECTED_NODE, &args![light, root]);

    // The material and the second property on the root.
    let material = new_object(e, 0x4c, MATERIAL_CONSTRUCT, &[]);
    e.call(MATERIAL_SET_COLOUR, &args![material, LIGHT_COLOUR]);
    e.call(MATERIAL_SET_COLOUR, &args![material, LIGHT_COLOUR]);
    e.call(MATERIAL_SET_AMBIENT, &args![material, LIGHT_COLOUR]);
    let root = pointer_get(e, at(ebp, POINTER_A));
    e.call(ATTACH_PROPERTY, &args![root, material]);
    let property = new_object(e, 0x24, PROPERTY_CONSTRUCT, &[]);
    e.call(PROPERTY_SET_MODE, &args![property, 3u32]);
    let root = pointer_get(e, at(ebp, POINTER_A));
    e.call(ATTACH_PROPERTY, &args![root, property]);

    // The grid texture is created when it is empty.
    let texture_slot = this + 0x24;
    if e.call(POINTER_EQUALS, &args![texture_slot, 0u32]).bool() {
        let first = e.call(FIRST_STATIC_OBJECT, &args![]).u32();
        let second = e.call(SECOND_STATIC_OBJECT, &args![]).u32();
        let texture = e
            .call(
                BUILD_FROM_STATIC_OBJECTS,
                &args![second, first, 0x17u32, 0u32, 0u32, 0u32],
            )
            .u32();
        pointer_assign(e, texture_slot, texture);
    }

    // Save the background colour of the renderer and set the mask one.
    let renderer = e.call(GET_RENDERER, &args![]).u32();
    e.vcall(renderer, 0xb4, &args![at(ebp, SAVED_COLOUR)]);
    let renderer = e.call(GET_RENDERER, &args![]).u32();
    e.vcall(renderer, 0xac, &args![BACKGROUND_COLOUR]);
    let root = pointer_get(e, at(ebp, POINTER_A));
    e.call(UPDATE_PROPERTIES, &args![root]);
    rotation(e, -0x180, POINTER_A);
    rotation(e, -0x15c, POINTER_B);
    rotation(e, -0x18c, POINTER_E);

    // Render.
    e.call(SCENE_STATE_CONSTRUCT, &args![at(ebp, SCENE_STATE), 0u32]);
    let accumulator = new_object(e, 0x280, ACCUMULATOR_CONSTRUCT, &[0x63, 1, 0x2f7]);
    e.call(POINTER_FROM, &args![at(ebp, POINTER_G), accumulator]);
    let accumulator = pointer_get(e, at(ebp, POINTER_G));
    e.call(
        SCENE_STATE_SET_ACCUMULATOR,
        &args![at(ebp, SCENE_STATE), accumulator],
    );
    let accumulator = pointer_get(e, at(ebp, POINTER_G));
    e.call(ACCUMULATOR_SET_FLAG, &args![accumulator, 0u32]);
    let texture = pointer_get(e, texture_slot);
    let stopped = e.call(RENDERED_TEXTURE_STOP, &args![texture]).u32();
    e.call(START_OFFSCREEN, &args![7u32, stopped]);
    let root = pointer_get(e, at(ebp, POINTER_A));
    let camera = pointer_get(e, at(ebp, POINTER_E));
    e.call(ACCUMULATE_SCENE, &args![camera, root, at(ebp, SCENE_STATE)]);
    let accumulator = pointer_get(e, at(ebp, POINTER_G));
    let camera = pointer_get(e, at(ebp, POINTER_E));
    e.call(RENDER_ACCUMULATED, &args![camera, accumulator, 0u32]);
    e.call(END_OFFSCREEN, &args![]);
    e.call(
        SCENE_STATE_SET_ACCUMULATOR,
        &args![at(ebp, SCENE_STATE), 0u32],
    );
    pointer_assign(e, at(ebp, POINTER_G), 0);

    // Hand the rendered texture over and restore.
    let texture = pointer_get(e, texture_slot);
    let rendered = e.call(RENDERED_TEXTURE_GET, &args![texture, 0u32]).u32();
    e.call(SET_MASK_TEXTURE, &args![rendered]);
    let default_accumulator = e.call(GET_ACCUMULATOR, &args![1u32]).u32();
    e.call(ACCUMULATOR_SET_FLAG, &args![default_accumulator, 1u32]);
    let renderer = e.call(GET_RENDERER, &args![]).u32();
    e.vcall(renderer, 0xac, &args![at(ebp, SAVED_COLOUR)]);
    e.call(POINTER_RELEASE, &args![at(ebp, POINTER_G)]);
    e.call(SCENE_STATE_DESTRUCT, &args![at(ebp, SCENE_STATE)]);
}

// Translated from 004bc1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the three floats at `+0x58` of `this` (the engine map has no name; on
/// a scene node that is its local translation): builds a point with
/// `00416870` from the three arguments, which the game copies through the
/// x87 stack (a signalling NaN comes out quiet), and copies its three words.
/// `__thiscall`, three stack words (`RET 0xc`).
pub fn fn_004bc1f0(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32) {
    let scratch = e.mem.alloc(12);
    let point = e
        .call(
            POINT3_CONSTRUCT,
            &args![
                scratch,
                copied_float_bits(x),
                copied_float_bits(y),
                copied_float_bits(z)
            ],
        )
        .u32();
    copy_words(e, this.addr() + 0x58, point, 3);
    e.mem.free(scratch);
}

// Translated from 004bc240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the first entry of the array at `+0x1c` of `this` (`00877a30` with
/// index 0 gives its address) and, when that entry is not null, returns what
/// `005585e0` makes of it; otherwise 0. `__thiscall`, no stack words.
pub fn fn_004bc240(e: &mut Engine, this: Ptr) -> u32 {
    let entry = e.call(0x0087_7a30, &args![this.addr() + 0x1c, 0u32]).u32();
    let element = e.mem.u32(entry);
    if element == 0 {
        return 0;
    }
    e.call(0x0055_85e0, &args![element]).u32()
}

// Translated from 004bc280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three words at `colour` to `+0xc8` of `this`, then bumps the
/// change counter (`004bc2c0`). `__thiscall`, one stack word.
pub fn fn_004bc280(e: &mut Engine, this: Ptr, colour: Ptr) {
    copy_words(e, this.addr() + 0xc8, colour.addr(), 3);
    fn_004bc2c0(e, this);
}

// Translated from 004bc2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds one to the counter at `+0xa8` of `this` (wrapping, as the `ADD`).
/// `__fastcall`.
pub fn fn_004bc2c0(e: &mut Engine, this: Ptr) {
    let counter = e.mem.u32(this.addr() + 0xa8);
    e.mem.set_u32(this.addr() + 0xa8, counter.wrapping_add(1));
}

// Translated from 004bc2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three words at `colour` to `+0xd4` of `this`, then bumps the
/// change counter (`004bc2c0`). `__thiscall`, one stack word.
pub fn fn_004bc2e0(e: &mut Engine, this: Ptr, colour: Ptr) {
    copy_words(e, this.addr() + 0xd4, colour.addr(), 3);
    fn_004bc2c0(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004baf10, fn_004baf10(Ptr)),
        entry!(0x004bc1f0, fn_004bc1f0(Ptr, f32, f32, f32)),
        entry!(0x004bc240, fn_004bc240(Ptr) -> u32),
        entry!(0x004bc280, fn_004bc280(Ptr, Ptr)),
        entry!(0x004bc2c0, fn_004bc2c0(Ptr)),
        entry!(0x004bc2e0, fn_004bc2e0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Test memory: +0 whether the mask can be built, +4 the renderer.
    const CONFIG: u32 = 0x0b00_0000;
    const VTABLE: u32 = 0x0a00_0000;
    const DIRECT3D_VTABLE: u32 = 0x0a00_1000;
    /// Every vtable slot `s` of an object points at `SLOT_TARGET + s`.
    const SLOT_TARGET: u32 = 0x0900_0000;
    const MASK_FLAGS_WORD: u32 = 0x011f_91d8;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// What the doubles saw: the translation of every strip, the frustum
    /// words, the geometry data arguments and the last scene state.
    #[derive(Default)]
    struct Seen {
        translations: Vec<[f32; 3]>,
        frustum: Vec<f32>,
        frustum_ortho: u8,
        geometry: Vec<u32>,
        vertices: Vec<f32>,
        normals: Vec<f32>,
        coordinates: Vec<f32>,
        indices: Vec<u16>,
    }

    /// Registers a double for every function `004baf10` calls outside this
    /// file; they return what the game's would for the test world.
    fn install(e: &mut Engine) -> Rc<RefCell<Seen>> {
        let seen = Rc::new(RefCell::new(Seen::default()));
        for (address, length) in [
            (CONFIG, 8),
            (VTABLE, 0x100),
            (DIRECT3D_VTABLE, 0x100),
            (MASK_FLAGS_WORD, 4),
            (MASK_STATE_FLAG, 1),
            (TWO, 8),
            (CENTRE_2048, 8),
            (MINUS_ONE, 8),
            (ONE, 8),
            (FAR_PLANE, 4),
            (NORMAL, 12),
            (LIGHT_COLOUR, 12),
        ] {
            e.map(address, length);
        }
        e.mem.set_f64(TWO, 2.0);
        e.mem.set_f64(CENTRE_2048, 2048.0);
        e.mem.set_f64(MINUS_ONE, -1.0);
        e.mem.set_f64(ONE, 1.0);
        e.mem.set_f32(FAR_PLANE, 2.0);
        e.mem.set_f32(NORMAL + 8, 1.0);
        e.mem.set_u32(LIGHT_COLOUR, 0x3f80_0000);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(VTABLE + slot, SLOT_TARGET + slot);
            e.mem
                .set_u32(DIRECT3D_VTABLE + slot, SLOT_TARGET + 0x1000 + slot);
            e.register(SLOT_TARGET + slot, |_, a| ret(a[0]));
            e.register(SLOT_TARGET + 0x1000 + slot, |_, a| ret(a[0]));
        }
        // Slot 0x9c of an object gives the Direct3D texture.
        e.register(SLOT_TARGET + 0x9c, |e, _| {
            let texture = e.mem.alloc(8);
            e.mem.set_u32(texture, DIRECT3D_VTABLE);
            ret(texture)
        });

        for address in [
            POINT_DEFAULT,
            COLOR_CONSTRUCT,
            FRUSTUM_CONSTRUCT,
            NODE_CONSTRUCT,
            CAMERA_CONSTRUCT,
            LIGHT_CONSTRUCT,
            MATERIAL_CONSTRUCT,
            PROPERTY_CONSTRUCT,
            ACCUMULATOR_CONSTRUCT,
            NO_LIGHTING_CONSTRUCT,
            TRI_STRIPS_CONSTRUCT,
            GUARD_ENTER,
            RENDERED_TEXTURE_STOP,
        ] {
            e.register(address, |_, a| ret(a[0]));
        }
        for address in [
            GUARD_LEAVE,
            POINTER_RELEASE,
            SET_MASK_ORIGIN,
            SET_MASK_SIZE,
            SET_MASK_TEXTURE,
            PROPERTY_SET_TEXTURE,
            ATTACH_PROPERTY,
            PREPARE_OBJECT,
            ROTATION_CONSTRUCT,
            SET_ROTATION,
            LOOK_AT,
            ATTACH_AFFECTED_NODE,
            MATERIAL_SET_COLOUR,
            MATERIAL_SET_AMBIENT,
            UPDATE_PROPERTIES,
            PROPERTY_SET_MODE,
            SCENE_STATE_CONSTRUCT,
            SCENE_STATE_DESTRUCT,
            SCENE_STATE_SET_ACCUMULATOR,
            ACCUMULATOR_SET_FLAG,
            START_OFFSCREEN,
            ACCUMULATE_SCENE,
            RENDER_ACCUMULATED,
            END_OFFSCREEN,
            VECTOR_CONSTRUCTOR,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(POINTER_FROM, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(POINTER_EQUALS, |e, a| ret((e.mem.u32(a[0]) == a[1]) as u32));
        e.register(GRID_SIDE, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(GRID_CELL_GET, |e, a| {
            let side = e.mem.u32(a[0] + 0xc);
            if a[1] < side && a[2] < side {
                ret(e.mem.u32(a[0] + 0x10) + (side * a[1] + a[2]) * 4)
            } else {
                ret(0)
            }
        });
        // A cell: +0 its state, +4 its texture, +8 its info.
        e.register(CELL_TEXTURE_STATE, |e, a| {
            e.mem.set_u32(a[1], e.mem.u32(a[0] + 4));
            e.mem.set_u32(a[2], e.mem.u32(a[0] + 8));
            ret(e.mem.u32(a[0]))
        });
        e.register(TEXTURE_OBJECT, |e, _| {
            let wrapper = e.mem.alloc(8);
            e.mem.set_u32(wrapper, VTABLE);
            ret(wrapper)
        });
        e.register(MASK_BUILD_ENABLED, |e, _| ret(e.mem.u32(CONFIG)));
        e.register(GET_RENDERER, |e, _| ret(e.mem.u32(CONFIG + 4)));
        e.register(GET_MASK_FLAGS, |e, _| ret(e.mem.u32(MASK_FLAGS_WORD)));
        e.register(SET_MASK_FLAGS, |e, a| {
            e.mem.set_u32(MASK_FLAGS_WORD, a[0]);
            Ret::default()
        });
        e.register(POINT3_CONSTRUCT, |e, a| {
            for i in 1..4 {
                e.mem.set_u32(a[0] + 4 * (i - 1), a[i as usize]);
            }
            ret(a[0])
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        for address in [ALLOCATE_OBJECT, ALLOCATE_ARRAY, ALLOCATE_INDICES] {
            e.register(address, |e, a| {
                let block = e.mem.alloc(a[0]);
                if a[0] >= 0x20 {
                    e.mem.set_u32(block, VTABLE);
                }
                ret(block)
            });
        }
        e.register(FIRST_STATIC_OBJECT, |_, _| ret(0x11));
        e.register(SECOND_STATIC_OBJECT, |_, _| ret(0x22));
        e.register(BUILD_FROM_STATIC_OBJECTS, |_, _| ret(0x33));
        e.register(RENDERED_TEXTURE_GET, |_, _| ret(0x77));
        e.register(GET_ACCUMULATOR, |_, _| ret(0x44));

        let log = seen.clone();
        e.register_double(SET_TRANSLATION, move |e, a| {
            log.borrow_mut().translations.push([
                e.mem.f32(a[1]),
                e.mem.f32(a[1] + 4),
                e.mem.f32(a[1] + 8),
            ]);
            Ret::default()
        });
        let log = seen.clone();
        e.register_double(SET_VIEW_FRUSTUM, move |e, a| {
            let mut log = log.borrow_mut();
            log.frustum = (0..6).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            log.frustum_ortho = e.mem.u8(a[1] + 0x18);
            Ret::default()
        });
        let log = seen.clone();
        e.register_double(GEOMETRY_DATA_CONSTRUCT, move |e, a| {
            let mut log = log.borrow_mut();
            log.geometry = a.to_vec();
            log.vertices = (0..12).map(|i| e.mem.f32(a[2] + 4 * i)).collect();
            log.normals = (0..12).map(|i| e.mem.f32(a[3] + 4 * i)).collect();
            log.coordinates = (0..8).map(|i| e.mem.f32(a[5] + 4 * i)).collect();
            log.indices = (0..4).map(|i| e.mem.u16(a[11] + 2 * i)).collect();
            ret(a[0])
        });
        seen
    }

    /// The grid object: `side` cells per side, origin (10, 20), no texture.
    fn grid(e: &mut Engine, side: u32) -> (Ptr, u32) {
        let object = e.mem.alloc(0x40);
        let cells = e.mem.alloc(4 * side.max(1) * side.max(1));
        e.mem.set_u32(object + 0xc, side);
        e.mem.set_u32(object + 0x10, cells);
        e.mem.set_f32(object + 0x14, 10.0);
        e.mem.set_f32(object + 0x18, 20.0);
        (Ptr::new(object), cells)
    }

    fn calls(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .collect()
    }

    fn words(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn nothing_is_built_when_the_mask_is_disabled() {
        let mut e = Engine::new();
        install(&mut e);
        let (object, _) = grid(&mut e, 2);
        e.mem.set_u32(CONFIG, 0);
        e.mem.set_u32(MASK_FLAGS_WORD, 0x21);
        e.call_log = Some(vec![]);
        e.call(0x004b_af10, &args![object]);
        let log = calls(&e);
        assert_eq!(log[0], 0x004b_af10);
        assert_eq!(log[1], GUARD_ENTER);
        assert_eq!(log.iter().filter(|a| **a == POINTER_FROM).count(), 6);
        assert_eq!(log.iter().filter(|a| **a == POINTER_RELEASE).count(), 6);
        assert_eq!(*log.last().unwrap(), GUARD_LEAVE);
        assert!(!log.contains(&SET_MASK_TEXTURE));
        assert!(!log.contains(&GRID_CELL_GET));
        assert_eq!(e.mem.u32(MASK_FLAGS_WORD), 0x21);
        assert_eq!(e.mem.u8(MASK_STATE_FLAG), 0);
    }

    #[test]
    fn no_cells_or_no_renderer_clears_the_flag_and_the_texture() {
        for (side, renderer) in [(0, 0x1234), (2, 0)] {
            let mut e = Engine::new();
            install(&mut e);
            let (object, _) = grid(&mut e, side);
            e.mem.set_u32(CONFIG, 1);
            e.mem.set_u32(CONFIG + 4, renderer);
            e.mem.set_u32(MASK_FLAGS_WORD, 0x01);
            e.call_log = Some(vec![]);
            e.call(0x004b_af10, &args![object]);
            // Bit 5 was set for the build, then cleared again.
            assert_eq!(e.mem.u32(MASK_FLAGS_WORD), 0x01);
            assert_eq!(words(&e, SET_MASK_FLAGS), vec![vec![0x21], vec![0x01]]);
            assert_eq!(words(&e, SET_MASK_TEXTURE), vec![vec![0]]);
            let size = (side << 12) as f32;
            assert_eq!(words(&e, SET_MASK_SIZE), vec![vec![size.to_bits()]]);
            let corner = 2048.0 - size / 2.0;
            assert_eq!(
                words(&e, SET_MASK_ORIGIN),
                vec![vec![(10.0 + corner).to_bits(), (20.0 + corner).to_bits()]]
            );
            assert!(!calls(&e).contains(&NODE_CONSTRUCT));
            assert_eq!(*calls(&e).last().unwrap(), GUARD_LEAVE);
        }
    }

    #[test]
    fn cells_without_a_texture_state_clear_the_mask_after_building_the_quad() {
        let mut e = Engine::new();
        let seen = install(&mut e);
        let (object, _) = grid(&mut e, 2);
        e.mem.set_u32(CONFIG, 1);
        e.mem.set_u32(CONFIG + 4, 0x1234);
        e.mem.set_u32(MASK_FLAGS_WORD, 0);
        e.call_log = Some(vec![]);
        e.call(0x004b_af10, &args![object]);
        // Four cells were looked at; none had a cell.
        assert_eq!(words(&e, GRID_CELL_GET).len(), 4);
        assert_eq!(words(&e, GRID_CELL_GET)[1][1..], [0, 1]);
        assert!(!calls(&e).contains(&TRI_STRIPS_CONSTRUCT));
        assert!(!calls(&e).contains(&ACCUMULATE_SCENE));
        assert_eq!(words(&e, SET_MASK_TEXTURE), vec![vec![0]]);
        assert_eq!(e.mem.u32(MASK_FLAGS_WORD), 0);
        // The quad: unit square, normals (0, 0, 1) each, texture coordinates.
        let seen = seen.borrow();
        assert_eq!(
            seen.vertices,
            [0., 0., 0., 1., 0., 0., 1., 1., 0., 0., 1., 0.]
        );
        assert_eq!(
            seen.normals,
            [0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1.]
        );
        assert_eq!(seen.coordinates, [0., 0., 1., 0., 1., 1., 0., 1.]);
        assert_eq!(seen.indices, [1, 0, 2, 3]);
        assert_eq!(seen.geometry[1], 4);
        assert_eq!(seen.geometry[4], 0);
        assert_eq!(seen.geometry[6..10], [1, 0, 2, 1]);
    }

    /// A cell with the given state, texture and info (a word pair).
    fn cell(e: &mut Engine, state: u32, texture: u32, info: u32) -> u32 {
        let cell = e.mem.alloc(12);
        e.mem.set_u32(cell, state);
        e.mem.set_u32(cell + 4, texture);
        e.mem.set_u32(cell + 8, info);
        cell
    }

    #[test]
    fn a_textured_cell_is_unlocked_drawn_and_rendered() {
        let mut e = Engine::new();
        let seen = install(&mut e);
        let (object, cells) = grid(&mut e, 2);
        e.mem.set_u32(CONFIG, 1);
        let renderer = e.mem.alloc(8);
        e.mem.set_u32(renderer, VTABLE);
        e.mem.set_u32(CONFIG + 4, renderer);
        e.mem.set_u32(MASK_FLAGS_WORD, 0);
        let info = e.mem.alloc(8);
        e.mem.set_u32(info + 4, 1);
        let first = cell(&mut e, 1, 0x5555, info);
        e.mem.set_u32(cells, first);
        // The cell at (1, 1) has state 2: no property, but the mask is kept.
        let last = cell(&mut e, 2, 0, 0);
        e.mem.set_u32(cells + 12, last);
        e.call_log = Some(vec![]);
        e.call(0x004b_af10, &args![object]);

        // The texture of the first cell was unlocked once and its info word
        // cleared.
        assert_eq!(e.mem.u32(info + 4), 0);
        assert_eq!(words(&e, SLOT_TARGET + 0x9c).len(), 1);
        let unlock = words(&e, SLOT_TARGET + 0x1000 + 0x50);
        assert_eq!(unlock.len(), 1);
        assert_eq!(unlock[0][1], 0);
        // One strip, placed at the corner of the grid.
        assert_eq!(seen.borrow().translations, vec![[-1.0, -1.0, 0.0]]);
        assert_eq!(words(&e, TRI_STRIPS_CONSTRUCT).len(), 1);
        assert_eq!(words(&e, NO_LIGHTING_CONSTRUCT).len(), 1);
        assert_eq!(words(&e, PROPERTY_SET_TEXTURE)[0][1], 0x5555);
        assert_eq!(
            words(&e, SLOT_TARGET + 0xd0),
            vec![vec![words(&e, NO_LIGHTING_CONSTRUCT)[0][0], 0]]
        );
        // The orthographic frustum has the grid's size.
        let seen = seen.borrow();
        assert_eq!(seen.frustum, [-1.0, 1.0, 1.0, -1.0, 1.0, 2.0]);
        assert_eq!(seen.frustum_ortho, 1);
        // The scene is rendered once with the 0x63/1/0x2f7 accumulator and
        // the rendered texture handed to the mask.
        assert_eq!(words(&e, ACCUMULATE_SCENE).len(), 1);
        assert_eq!(words(&e, ACCUMULATOR_CONSTRUCT)[0][1..], [0x63, 1, 0x2f7]);
        assert_eq!(
            words(&e, BUILD_FROM_STATIC_OBJECTS),
            vec![vec![0x22, 0x11, 0x17, 0, 0, 0]]
        );
        assert_eq!(words(&e, SET_MASK_TEXTURE), vec![vec![0x77]]);
        assert_eq!(e.mem.u32(object.addr() + 0x24), 0x33);
        assert_eq!(words(&e, ACCUMULATOR_SET_FLAG)[1][1], 1);
        assert_eq!(*calls(&e).last().unwrap(), GUARD_LEAVE);
    }

    #[test]
    fn a_signalling_nan_translation_is_made_quiet() {
        let mut e = Engine::new();
        install(&mut e);
        let node = e.mem.alloc(0x80);
        e.call(
            0x004b_c1f0,
            &args![Ptr::<()>::new(node), 0x7fa0_0000u32, 1.0f32, -0.0f32],
        );
        assert_eq!(e.mem.u32(node + 0x58), 0x7fe0_0000);
        assert_eq!(e.mem.f32(node + 0x5c), 1.0);
        assert_eq!(e.mem.u32(node + 0x60), 0x8000_0000);
    }

    #[test]
    fn the_colour_setters_copy_three_words_and_bump_the_counter() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x100);
        let colour = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_u32(colour + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(object + 0xa8, u32::MAX);
        e.call(
            0x004b_c280,
            &args![Ptr::<()>::new(object), Ptr::<()>::new(colour)],
        );
        assert_eq!(e.mem.u32(object + 0xc8), 0x100);
        assert_eq!(e.mem.u32(object + 0xd0), 0x102);
        assert_eq!(e.mem.u32(object + 0xa8), 0);
        e.call(
            0x004b_c2e0,
            &args![Ptr::<()>::new(object), Ptr::<()>::new(colour)],
        );
        assert_eq!(e.mem.u32(object + 0xd4), 0x100);
        assert_eq!(e.mem.u32(object + 0xdc), 0x102);
        assert_eq!(e.mem.u32(object + 0xa8), 1);
        e.call(0x004b_c2c0, &args![Ptr::<()>::new(object)]);
        assert_eq!(e.mem.u32(object + 0xa8), 2);
    }

    #[test]
    fn the_first_entry_result_needs_a_non_null_entry() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        let entry = e.mem.alloc(4);
        e.register_double(0x0087_7a30, move |_, a| {
            assert_eq!(a[1], 0);
            ret(entry)
        });
        e.register(0x0055_85e0, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x004b_c240, &args![Ptr::<()>::new(object)]).u32(), 0);
        e.mem.set_u32(entry, 0x500);
        assert_eq!(
            e.call(0x004b_c240, &args![Ptr::<()>::new(object)]).u32(),
            0x501
        );
    }
}
