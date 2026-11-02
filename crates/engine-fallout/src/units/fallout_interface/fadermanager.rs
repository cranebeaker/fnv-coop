//! `fallout/interface/fadermanager.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The fader manager (`FaderManager`, 8 bytes, the pointer in the global at
//! `011d8804`) owns two scene graph roots and a static table of three
//! faders (the Xbox PDB's `FaderData`, only known by the name of its
//! initializer). A fader is a full-screen quad of a texture from
//! `Data\Textures\Interface\Faders\<name>` whose shader property's alpha
//! (`+0x78`) is driven from 0 to 1 or back over a time.
//!
//! The functions that open a profiling scope (`00404eb0` / `00404ee0`) and
//! hold temporaries have a C++ exception-unwinding frame, which is not
//! translated: `007006c0`, `00700960` and `007011d0` carry a `Translation
//! incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// The fader table: three entries of [`FaderData`] (0x1c bytes each).
const FADERS: u32 = 0x011d_8828;
/// Size of a table entry.
const FADER_SIZE: u32 = 0x1c;
/// Number of entries.
const FADER_COUNT: i32 = 3;
/// A signed byte counted down (in place of the time) while a fader fades
/// out; `CreateFader` sets it to 3 for a fader that starts opaque.
const FRAMES_TO_WAIT: u32 = 0x011d_8808;
/// A byte flag that makes `RemoveFader` remove at once.
const REMOVE_AT_ONCE: u32 = 0x011d_8909;
/// The object whose float at +0xc is the frame time (the divisor is the
/// float at `011ac3a0`).
const FRAME_TIME_OBJECT: u32 = 0x011f_6394;
const FRAME_TIME_DIVISOR: u32 = 0x011a_c3a0;
/// The object (pointer in `011dea10`) that `RemoveFader` calls `00457d70`
/// on after removing a fader.
const AFTER_REMOVE_OBJECT: u32 = 0x011d_ea10;

/// Constants: `1.0` and `0.0` (`double`), `20.0` (`double`), `0.5`
/// (`double`), `0.75` (`double`), `1280.0` (`double`), 4/3 (`float`), and
/// the `float` just below 1.0 that `RemoveFader` leaves a fully opaque
/// fader at.
const ONE: u32 = 0x0101_2070;
const ZERO: u32 = 0x0101_2060;
const TWENTY: u32 = 0x0102_fc70;
const HALF: u32 = 0x0101_1588;
const THREE_QUARTERS: u32 = 0x0101_de30;
const WIDE_WIDTH: u32 = 0x0106_e960;
const FOUR_THIRDS: u32 = 0x0106_e968;
const JUST_BELOW_ONE: u32 = 0x0106_ea1c;

/// The source file name `...\Fallout\Interface\FaderManager.cpp` the
/// profiling scope is opened with.
const SOURCE_FILE: u32 = 0x0106_e9b0;
/// `"FaderNode Below Menus"`, `"FaderNode Above Menus"`, `"FaderNodeLight"`.
const NAME_BELOW_MENUS: u32 = 0x0106_e994;
const NAME_ABOVE_MENUS: u32 = 0x0106_e97c;
const NAME_LIGHT: u32 = 0x0106_e96c;
/// `"Data\Textures\Interface\Faders\%s"`.
const TEXTURE_PATH_FORMAT: u32 = 0x0106_e9f8;
/// The address passed as the second argument of `NiSourceTexture::Create`.
const TEXTURE_PREFS: u32 = 0x011a_9598;

/// Profiling scope constructor (`ECX` = scope; 0xd, 1, file, line) and
/// destructor.
const SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_DESTROY: u32 = 0x0040_4ee0;
/// The allocator `operator new` (cdecl, size) and the allocators the faders'
/// arrays use.
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const ARRAY_ALLOC: u32 = 0x0040_1000;
const BYTES_ALLOC: u32 = 0x00aa_1070;
/// `_vector_constructor_iterator_` (pointer, element size, count, element
/// constructor) and the element constructor (`007815c0`-style: returns its
/// `this`).
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
const THIS_RETURNER: u32 = 0x0068_15c0;
/// `NiNode::NiNode(0)`.
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `NiDirectionalLight::NiDirectionalLight` (engine map).
const LIGHT_CONSTRUCT: u32 = 0x00a7_5c20;
/// `NiTriShape::NiTriShape` (engine map), nine arguments.
const TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
/// The constructor of the property attached to the root (`004391c0`) and
/// its setter (`0049ed90`, takes a flag).
const PROPERTY_CONSTRUCT: u32 = 0x0043_91c0;
const PROPERTY_SET_FLAG: u32 = 0x0049_ed90;
/// `NiAVObject::AttachProperty` (engine map).
const ATTACH_PROPERTY: u32 = 0x0043_9410;
/// `NiAVObject::UpdateProperties` (engine map) and the update call
/// (`00a59c60`, takes a `NiUpdateData`) with the update data's constructor
/// (`0043d410`, a float and two flags).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE: u32 = 0x00a5_9c60;
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// `NiObjectNET::SetName` (takes a `NiFixedString *`), the
/// `NiFixedString` constructor (from characters) and destructor.
const SET_NAME: u32 = 0x00a5_b950;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTROY: u32 = 0x0043_81b0;
/// `NiPoint3` constructor (x, y, z) and `NiPoint2` constructor (x, y); both
/// return `this`.
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
/// Copies a `NiPoint3` to its `this + 0xc8` and counts a change (`004bc280`,
/// used on the light), and the translation setter (`00440460`, copies to
/// `this + 0x58`) and scale setter (`00440490`, a float to `this + 0x64`)
/// of the root.
const LIGHT_SET_COLOR: u32 = 0x004b_c280;
const NODE_SET_TRANSLATION: u32 = 0x0044_0460;
const NODE_SET_SCALE: u32 = 0x0044_0490;
/// An empty function taking one argument (`RET 4`).
const EMPTY_ONE_ARGUMENT: u32 = 0x00a8_9af0;
/// The screen width and height getters (the words at `0118947c` and its
/// neighbour), and whether the screen is in a mode that forces scale 1.
const SCREEN_WIDTH: u32 = 0x004d_c1f0;
const SCREEN_HEIGHT: u32 = 0x004d_c200;
const FORCES_UNIT_SCALE: u32 = 0x004d_e070;
/// `BSStringT` constructor / destructor / `sprintf` / character getter.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const STRING_C_STR: u32 = 0x0055_9450;
/// `NiSourceTexture::Create` (engine map; four arguments) and the smart
/// pointer constructor (`633c90`, from a raw pointer) / destructor
/// (`0045cec0`) / assignment (`0066b0d0`) it is held in.
const TEXTURE_CREATE: u32 = 0x00a5_fd70;
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_DESTROY: u32 = 0x0045_cec0;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `TileShaderProperty::TileShaderProperty` (engine map), its
/// `SetTileTexture` (engine map), the setter of its byte (`008cc6d0`, takes
/// 3) and its alpha setter (`00821600`, a float to `this + 0x78`).
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00bb_7c30;
const SET_TILE_TEXTURE: u32 = 0x00bb_7a10;
const SHADER_PROPERTY_SET_BYTE: u32 = 0x008c_c6d0;
const SET_ALPHA: u32 = 0x0082_1600;
/// `BSShaderManager::PrepareObject` (engine map; cdecl, shape and two
/// flags).
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// Returns the bound of a shape (`this + something`, `00537b10`) and
/// `NiBound::ComputeFromData` (engine map; count and points).
const SHAPE_BOUND: u32 = 0x0053_7b10;
const COMPUTE_BOUND: u32 = 0x00a7_ee30;
/// Returns the word at +0x18 of its node: the node's parent.
const NODE_PARENT: u32 = 0x0096_11e0;
/// Called on the object in `011dea10` after a fader is removed.
const AFTER_REMOVE: u32 = 0x0045_7d70;
/// Returns the pointer a smart pointer (at its `this`) holds.
const POINTER_GET: u32 = 0x0055_9450;

layout! {
    /// `FaderManager` (Xbox PDB), 8 bytes.
    pub struct FaderManager: 0x8 {
        /// `pRootBelowMenus` (Xbox PDB): `NiNode *`.
        0x00 pRootBelowMenus: Ptr,
        /// `pRootAboveMenus` (Xbox PDB): `NiNode *`.
        0x04 pRootAboveMenus: Ptr,
    }

    /// One entry of the fader table (the Xbox PDB has no type of this name;
    /// `_dynamic_initializer_for_FaderData_` is the table's initializer), by
    /// the use the code makes of it.
    pub struct FaderData: 0x1c {
        /// Set: the fader hangs under the root below the menus.
        0x00 bBelowMenus: bool,
        /// The fader's name (characters); the texture is
        /// `Data\Textures\Interface\Faders\<name>`.
        0x04 pName: u32,
        /// Set: the alpha rises towards 1.0; clear: it falls towards 0.0.
        0x08 bFadingIn: bool,
        /// Seconds the fade takes.
        0x0C fDuration: f32,
        /// The fader's scene graph node (`NiNode *`).
        0x10 pNode: Ptr,
        /// The current alpha.
        0x14 fAlpha: f32,
        /// A smart pointer holding the shader property whose alpha is set.
        0x18 spShaderProperty: Ptr,
    }
}

/// A `float` copied through `FLD` / `FSTP`: a signaling NaN comes out quiet.
fn through_x87(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

/// The table entry for `index` (the code multiplies as a signed 32-bit
/// integer, so any index reaches memory).
fn fader(index: i32) -> Ptr<FaderData> {
    Ptr::new(FADERS).index(index as u32, FADER_SIZE)
}

/// `NiNode::SetName` with a temporary `NiFixedString` built from `text` in the caller's local
/// `name`.
fn set_name(e: &mut Engine, name: u32, object: u32, text: u32) {
    e.call(FIXED_STRING_CONSTRUCT, &args![name, text]);
    e.call(SET_NAME, &args![object, name]);
    e.call(FIXED_STRING_DESTROY, &args![name]);
}

/// Builds a `NiPoint3` in the caller's local `temp` and copies its three words to
/// `destination`.
fn store_point3(e: &mut Engine, temp: u32, destination: u32, x: f32, y: f32, z: f32) {
    let point = e.call(POINT3_CONSTRUCT, &args![temp, x, y, z]).u32();
    for word in 0..3 {
        let value = e.mem.u32(point + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
}

/// Builds a `NiPoint2` in the caller's local `temp` and copies its two words.
fn store_point2(e: &mut Engine, temp: u32, destination: u32, x: f32, y: f32) {
    let point = e.call(POINT2_CONSTRUCT, &args![temp, x, y]).u32();
    for word in 0..2 {
        let value = e.mem.u32(point + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
}

/// A new array of four elements of `element_size` bytes, each constructed
/// by `THIS_RETURNER` (`0` when the allocation fails).
fn new_element_array(e: &mut Engine, element_size: u32) -> u32 {
    let memory = e.call(ARRAY_ALLOC, &args![element_size * 4]).u32();
    if memory != 0 {
        e.call(
            VECTOR_CONSTRUCTOR,
            &args![memory, element_size, 4u32, THIS_RETURNER],
        );
    }
    memory
}

// Translated from 007006c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FaderManager::AddRoot` (Xbox PDB): makes a root node with a white
/// directional light below it, scaled to the screen (1.0 at 4:3, 4/3 at
/// other ratios, times `1280 / width` when the width is not 1280, and 1.0
/// when `004de070` says so), moved to `distance` along y, and attaches it
/// to `graph`. With `below_menus` the root is `pRootBelowMenus` (named
/// `"FaderNode Below Menus"`); otherwise `pRootAboveMenus` (named
/// `"FaderNode Above Menus"`) and `distance` is first reduced by 20.0.
// Translation incomplete: 007006c0 - the C++ exception-unwinding frame (FS:[0]) that releases the allocations when a constructor throws is not translated
pub fn fader_manager_add_root(
    e: &mut Engine,
    this: Ptr<FaderManager>,
    graph: Ptr,
    mut distance: f32,
    below_menus: bool,
) {
    // The exe's frame (`SUB ESP,0x78` under the exception record, then the record link): the
    // scope at `EBP-0x1c` (+0x6c), the root's name at `EBP-0x30` (+0x58, below the menus) or
    // `EBP-0x34` (+0x54), the light's name at `EBP-0x40` (+0x48), the colour at `EBP-0x4c`
    // (+0x3c) and the position at `EBP-0x58` (+0x30).
    e.with_stack(0x88, |e, frame| {
        let local = |offset: u32| frame.addr() + offset;
        let guard = local(0x6c);
        e.call(
            SCOPE_CONSTRUCT,
            &args![guard, 0xdu32, 1u32, SOURCE_FILE, 0x50u32],
        );

        let memory = e.call(OPERATOR_NEW, &args![0xacu32]).u32();
        let root = if memory != 0 {
            e.call(NODE_CONSTRUCT, &args![memory, 0u32]).u32()
        } else {
            0
        };
        if below_menus {
            e.set(this, FaderManager::pRootBelowMenus, Ptr::new(root));
            set_name(e, local(0x58), root, NAME_BELOW_MENUS);
        } else {
            e.set(this, FaderManager::pRootAboveMenus, Ptr::new(root));
            set_name(e, local(0x54), root, NAME_ABOVE_MENUS);
            let twenty: f64 = e.global(TWENTY);
            distance = (distance as f64 - twenty) as f32;
        }

        let memory = e.call(OPERATOR_NEW, &args![0xfcu32]).u32();
        let light = if memory != 0 {
            e.call(LIGHT_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        set_name(e, local(0x48), light, NAME_LIGHT);
        let color = e
            .call(
                POINT3_CONSTRUCT,
                &args![local(0x3c), 1.0f32, 1.0f32, 1.0f32],
            )
            .u32();
        e.call(LIGHT_SET_COLOR, &args![light, color]);
        e.call(EMPTY_ONE_ARGUMENT, &args![light, root]);
        e.vcall(root, 0xdc, &args![light, 1u32]);

        let width = e.call(SCREEN_WIDTH, &args![]).i32() as f32;
        let height = e.call(SCREEN_HEIGHT, &args![]).i32() as f32;
        let mut scale = 1.0f32;
        let three_quarters: f64 = e.global(THREE_QUARTERS);
        if height as f64 / width as f64 != three_quarters {
            scale = through_x87(e.global(FOUR_THIRDS));
        }
        let wide_width: f64 = e.global(WIDE_WIDTH);
        if width as f64 != wide_width {
            scale = (wide_width / width as f64 * scale as f64) as f32;
        }
        if e.call(FORCES_UNIT_SCALE, &args![]).bool() {
            scale = 1.0;
        }
        e.call(NODE_SET_SCALE, &args![root, through_x87(scale)]);
        let position = e
            .call(
                POINT3_CONSTRUCT,
                &args![local(0x30), 0.0f32, through_x87(distance), 0.0f32],
            )
            .u32();
        e.call(NODE_SET_TRANSLATION, &args![root, position]);
        e.vcall(graph.addr(), 0xdc, &args![root, 1u32]);
        e.call(SCOPE_DESTROY, &args![guard]);
    });
}

// Translated from 00700960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FaderManager::CreateFader` (Xbox PDB): sets up fader `index` for
/// `duration` seconds. `immediate` starts it opaque (alpha 1.0 and the fade
/// then runs down), as does a duration of exactly 0.0; either also sets the
/// frame wait counter to 3.
///
/// A fader that is already fading in is left alone; any other existing one
/// is removed first. The fader is a node (named after the table entry)
/// holding a four-vertex quad (a `NiTriShape` of two triangles, `0 1 2` and
/// `2 1 3`, as wide and high as the screen, in the xz plane, with the
/// corner texture coordinates) with a `NiProperty` made by `004391c0` and a
/// `TileShaderProperty` textured from
/// `Data\Textures\Interface\Faders\<name>` (kept in the table), attached
/// under the root below the menus when the entry's flag is set and above
/// them otherwise.
// Translation incomplete: 00700960 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporaries and releases the allocations when a step throws is not translated
pub fn fader_manager_create_fader(
    e: &mut Engine,
    this: Ptr<FaderManager>,
    index: i32,
    duration: f32,
    immediate: bool,
) {
    // The exe's frame (`SUB ESP,0x10c` under the exception record, then the record link): the
    // scope at `EBP-0x20` (+0xfc), the corner temporaries at `EBP-0x6c`, `-0x78`, `-0x84` and
    // `-0x90` (+0xb0, +0xa4, +0x98, +0x8c), the texture coordinate temporaries at `EBP-0x98`,
    // `-0xa0`, `-0xa8` and `-0xb0` (+0x84, +0x7c, +0x74, +0x6c), the node's name at `EBP-0xbc`
    // (+0x60), the path at `EBP-0x38` (+0xe4), the shape's name at `EBP-0xd0` (+0x4c), the
    // texture name at `EBP-0xd4` (+0x48), the texture pointer at `EBP-0x48` (+0xd4) and the
    // update data at `EBP-0x44` (+0xd8).
    e.with_stack(0x11c, |e, frame| {
        let local = |offset: u32| frame.addr() + offset;
        let guard = local(0xfc);
        e.call(
            SCOPE_CONSTRUCT,
            &args![guard, 0xdu32, 1u32, SOURCE_FILE, 0x87u32],
        );
        let slot = fader(index);
        if !e.get(slot, FaderData::pNode).is_null() {
            if e.get(slot, FaderData::bFadingIn) {
                e.call(SCOPE_DESTROY, &args![guard]);
                return;
            }
            fader_manager_remove_fader(e, this, index, true);
        }

        let vertices = new_element_array(e, 0xc);
        let texture_coordinates = new_element_array(e, 8);
        let triangles = e.call(BYTES_ALLOC, &args![0xcu32]).u32();
        let half: f64 = e.global(HALF);
        let half_width = (e.call(SCREEN_WIDTH, &args![]).i32() as f64 * half) as f32;
        let half_height = (e.call(SCREEN_HEIGHT, &args![]).i32() as f64 * half) as f32;
        store_point3(e, local(0xb0), vertices, -half_width, 0.0, half_height);
        store_point3(
            e,
            local(0xa4),
            vertices + 0xc,
            -half_width,
            0.0,
            -half_height,
        );
        store_point3(
            e,
            local(0x98),
            vertices + 0x18,
            half_width,
            0.0,
            half_height,
        );
        store_point3(
            e,
            local(0x8c),
            vertices + 0x24,
            half_width,
            0.0,
            -half_height,
        );
        store_point2(e, local(0x84), texture_coordinates, 0.0, 0.0);
        store_point2(e, local(0x7c), texture_coordinates + 8, 0.0, 1.0);
        store_point2(e, local(0x74), texture_coordinates + 0x10, 1.0, 0.0);
        store_point2(e, local(0x6c), texture_coordinates + 0x18, 1.0, 1.0);
        for (offset, value) in [0u16, 1, 2, 2, 1, 3].into_iter().enumerate() {
            e.mem.set_u16(triangles + 2 * offset as u32, value);
        }

        let memory = e.call(OPERATOR_NEW, &args![0xacu32]).u32();
        let node = if memory != 0 {
            e.call(NODE_CONSTRUCT, &args![memory, 0u32]).u32()
        } else {
            0
        };
        e.set(slot, FaderData::pNode, Ptr::new(node));
        let name = e.get(slot, FaderData::pName);
        let node = e.get(slot, FaderData::pNode).addr();
        set_name(e, local(0x60), node, name);

        let memory = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
        let property = if memory != 0 {
            e.call(PROPERTY_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        e.call(PROPERTY_SET_FLAG, &args![property, 1u32]);
        let node = e.get(slot, FaderData::pNode).addr();
        e.call(ATTACH_PROPERTY, &args![node, property]);

        let memory = e.call(OPERATOR_NEW, &args![0xc4u32]).u32();
        let shape = if memory != 0 {
            e.call(
                TRI_SHAPE_CONSTRUCT,
                &args![
                    memory,
                    4u32,
                    vertices,
                    0u32,
                    0u32,
                    texture_coordinates,
                    1u32,
                    0u32,
                    2u32,
                    triangles
                ],
            )
            .u32()
        } else {
            0
        };
        let node = e.get(slot, FaderData::pNode).addr();
        e.vcall(node, 0xdc, &args![shape, 1u32]);

        let path = local(0xe4);
        e.call(STRING_CONSTRUCT, &args![path]);
        let name = e.get(slot, FaderData::pName);
        e.call(STRING_FORMAT, &args![path, TEXTURE_PATH_FORMAT, name]);
        let characters = e.call(STRING_C_STR, &args![path]).u32();
        set_name(e, local(0x4c), shape, characters);
        let texture_pointer = local(0xd4);
        let characters = e.call(STRING_C_STR, &args![path]).u32();
        let texture_name = local(0x48);
        e.call(FIXED_STRING_CONSTRUCT, &args![texture_name, characters]);
        let texture = e
            .call(
                TEXTURE_CREATE,
                &args![texture_name, TEXTURE_PREFS, 1u32, 0u32],
            )
            .u32();
        e.call(POINTER_CONSTRUCT, &args![texture_pointer, texture]);
        e.call(FIXED_STRING_DESTROY, &args![texture_name]);

        let mut start_alpha = 0.0f32;
        let zero: f64 = e.global(ZERO);
        if immediate || duration as f64 == zero {
            start_alpha = 1.0;
            e.set_global(FRAMES_TO_WAIT, 3u8);
        }

        let memory = e.call(OPERATOR_NEW, &args![0xb0u32]).u32();
        let shader_property = if memory != 0 {
            e.call(SHADER_PROPERTY_CONSTRUCT, &args![memory, 0u32])
                .u32()
        } else {
            0
        };
        let texture = e.call(POINTER_GET, &args![texture_pointer]).u32();
        e.call(SET_TILE_TEXTURE, &args![shader_property, texture]);
        e.call(SHADER_PROPERTY_SET_BYTE, &args![shader_property, 3u32]);
        e.call(SET_ALPHA, &args![shader_property, through_x87(start_alpha)]);
        e.call(ATTACH_PROPERTY, &args![shape, shader_property]);
        e.call(
            POINTER_ASSIGN,
            &args![
                slot.addr() + FaderData::spShaderProperty.off,
                shader_property
            ],
        );
        e.call(PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
        let bound = e.call(SHAPE_BOUND, &args![shape]).u32();
        e.call(COMPUTE_BOUND, &args![bound, 4u32, vertices]);

        let mut root = e.get(this, FaderManager::pRootAboveMenus).addr();
        if e.get(slot, FaderData::bBelowMenus) {
            root = e.get(this, FaderManager::pRootBelowMenus).addr();
        }
        let node = e.get(slot, FaderData::pNode).addr();
        e.vcall(root, 0xdc, &args![node, 1u32]);
        e.call(UPDATE_PROPERTIES, &args![root]);
        let update_data = local(0xd8);
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![update_data, 0.0f32, 0u32, 0u32],
        );
        e.call(UPDATE, &args![root, update_data]);
        e.set(slot, FaderData::fDuration, through_x87(duration));
        e.set(slot, FaderData::bFadingIn, !immediate);
        e.set(slot, FaderData::fAlpha, start_alpha);

        e.call(POINTER_DESTROY, &args![texture_pointer]);
        e.call(STRING_DESTROY, &args![path]);
        e.call(SCOPE_DESTROY, &args![guard]);
    });
}

// Translated from 007010e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FaderManager::RemoveFader` (Xbox PDB): for a fader with a node, either
/// (when `force` or the byte at `011d8909` is set) detaches the node from
/// its parent (virtual slot `+0xe8` of the parent), clears the node, the
/// alpha and the shader property pointer and calls `00457d70` on the
/// object in `011dea10`; or lets it fade out: clears the fading-in flag and
/// moves an alpha of exactly 1.0 to the `float` just below it.
pub fn fader_manager_remove_fader(
    e: &mut Engine,
    _this: Ptr<FaderManager>,
    index: i32,
    force: bool,
) {
    let slot = fader(index);
    let node = e.get(slot, FaderData::pNode);
    if node.is_null() {
        return;
    }
    if force || e.global::<u8>(REMOVE_AT_ONCE) != 0 {
        let parent = e.call(NODE_PARENT, &args![node]).u32();
        let node = e.get(slot, FaderData::pNode);
        e.vcall(parent, 0xe8, &args![node]);
        e.set(slot, FaderData::pNode, Ptr::NULL);
        e.set(slot, FaderData::fAlpha, 0.0);
        e.call(
            POINTER_ASSIGN,
            &args![slot.addr() + FaderData::spShaderProperty.off, 0u32],
        );
        let object: u32 = e.global(AFTER_REMOVE_OBJECT);
        e.call(AFTER_REMOVE, &args![object, 0u32, 0u32, 0u32]);
    } else {
        e.set(slot, FaderData::bFadingIn, false);
        let one: f64 = e.global(ONE);
        if e.get(slot, FaderData::fAlpha) as f64 == one {
            let below_one = through_x87(e.global(JUST_BELOW_ONE));
            e.set(slot, FaderData::fAlpha, below_one);
        }
    }
}

// Translated from 007011d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FaderManager::UpdateFaders`: steps the three faders by the frame time.
/// A fading-in fader with alpha below 1.0 gains `frame time / duration`
/// (limited to 1.0). A fading-out one whose alpha is 0.0 or less (ordered)
/// is detached from its parent and cleared; otherwise it waits while the
/// byte at `011d8808` counts down, then loses `frame time / duration`
/// (limited to 0.0). A shader property then gets the alpha. The second
/// word is not read (the caller pushes 1).
// Translation incomplete: 007011d0 - the C++ exception-unwinding frame (FS:[0]) that closes the profiling scope when a step throws is not translated
pub fn fader_manager_update_faders(e: &mut Engine, _this: Ptr<FaderManager>, _unused_1: u32) {
    // The exe's frame (`SUB ESP,0x14` under the exception record, then the record link): the
    // scope at `EBP-0x10` (+0x14).
    e.with_stack(0x24, |e, frame| {
        let guard = frame.addr() + 0x14;
        e.call(
            SCOPE_CONSTRUCT,
            &args![guard, 0xdu32, 1u32, SOURCE_FILE, 0x10au32],
        );
        let one: f64 = e.global(ONE);
        let zero: f64 = e.global(ZERO);
        for index in 0..FADER_COUNT {
            let slot = fader(index);
            if e.get(slot, FaderData::pNode).is_null() {
                continue;
            }
            if e.get(slot, FaderData::bFadingIn) {
                if (e.get(slot, FaderData::fAlpha) as f64) < one {
                    let frame_time = frame_time(e);
                    let duration = e.get(slot, FaderData::fDuration);
                    let alpha = e.get(slot, FaderData::fAlpha);
                    let mut alpha = (frame_time as f64 / duration as f64 + alpha as f64) as f32;
                    if alpha as f64 > one {
                        alpha = 1.0;
                    }
                    e.set(slot, FaderData::fAlpha, alpha);
                    set_shader_alpha(e, slot);
                }
            } else if e.get(slot, FaderData::fAlpha) as f64 <= zero {
                let node = e.get(slot, FaderData::pNode);
                if !node.is_null() && e.call(NODE_PARENT, &args![node]).u32() != 0 {
                    let parent = e.call(NODE_PARENT, &args![node]).u32();
                    e.vcall(parent, 0xe8, &args![node]);
                }
                e.set(slot, FaderData::pNode, Ptr::NULL);
                e.set(slot, FaderData::fAlpha, 0.0);
            } else {
                let wait = e.global::<u8>(FRAMES_TO_WAIT);
                if wait as i8 != 0 {
                    e.set_global(FRAMES_TO_WAIT, wait.wrapping_sub(1));
                } else {
                    let frame_time = frame_time(e);
                    let duration = e.get(slot, FaderData::fDuration);
                    let alpha = e.get(slot, FaderData::fAlpha);
                    let alpha = (alpha as f64 - frame_time as f64 / duration as f64) as f32;
                    e.set(slot, FaderData::fAlpha, alpha);
                }
                if (e.get(slot, FaderData::fAlpha) as f64) < zero {
                    e.set(slot, FaderData::fAlpha, 0.0);
                }
                set_shader_alpha(e, slot);
            }
        }
        e.call(SCOPE_DESTROY, &args![guard]);
    });
}

/// Hands the fader's alpha to its shader property, when it has one.
fn set_shader_alpha(e: &mut Engine, slot: Ptr<FaderData>) {
    let pointer = slot.addr() + FaderData::spShaderProperty.off;
    if e.call(POINTER_GET, &args![pointer]).u32() != 0 {
        let alpha = through_x87(e.get(slot, FaderData::fAlpha));
        let property = e.call(POINTER_GET, &args![pointer]).u32();
        e.call(SET_ALPHA, &args![property, alpha]);
    }
}

/// The frame time of `007013e0` on its object.
fn frame_time(e: &mut Engine) -> f32 {
    fn_007013e0(e, Ptr::new(FRAME_TIME_OBJECT))
}

// Translated from 007013e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at +0xc of the object (`ECX`) divided by the float at
/// `011ac3a0`, rounded to a `float`, in `ST0`. `UpdateFaders` and the frame
/// code call it on the object at `011f6394` for the frame time.
pub fn fn_007013e0(e: &mut Engine, this: Ptr) -> f32 {
    let numerator = e.mem.f32(this.addr() + 0xc);
    let divisor: f32 = e.global(FRAME_TIME_DIVISOR);
    (numerator as f64 / divisor as f64) as f32
}

// Translated from 00701400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether fader `index` has a node, is fading in, and its alpha is below
/// 1.0 (ordered: a NaN alpha is false).
pub fn fn_00701400(e: &mut Engine, _this: Ptr<FaderManager>, index: i32) -> bool {
    let slot = fader(index);
    let one: f64 = e.global(ONE);
    !e.get(slot, FaderData::pNode).is_null()
        && e.get(slot, FaderData::bFadingIn)
        && (e.get(slot, FaderData::fAlpha) as f64) < one
}

// Translated from 00701450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether fader `index` has a node, is fading in, and its alpha is exactly
/// 1.0 (a NaN alpha is false).
pub fn fn_00701450(e: &mut Engine, _this: Ptr<FaderManager>, index: i32) -> bool {
    let slot = fader(index);
    let one: f64 = e.global(ONE);
    !e.get(slot, FaderData::pNode).is_null()
        && e.get(slot, FaderData::bFadingIn)
        && e.get(slot, FaderData::fAlpha) as f64 == one
}

// Translated from 007014a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FaderManager::IsFaderActive` (Xbox PDB): fader `index` has a node and
/// is fading in.
pub fn fader_manager_is_fader_active(e: &mut Engine, _this: Ptr<FaderManager>, index: i32) -> bool {
    let slot = fader(index);
    !e.get(slot, FaderData::pNode).is_null() && e.get(slot, FaderData::bFadingIn)
}

// Translated from 007014e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FaderManager::GetFaderAlpha` (Xbox PDB): the alpha of fader `index`,
/// in `ST0` (the `FLD` quiets a signaling NaN).
pub fn fader_manager_get_fader_alpha(e: &mut Engine, _this: Ptr<FaderManager>, index: i32) -> f32 {
    through_x87(e.get(fader(index), FaderData::fAlpha))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x007006c0,
            fader_manager_add_root(Ptr<FaderManager>, Ptr, f32, bool)
        ),
        entry!(
            0x00700960,
            fader_manager_create_fader(Ptr<FaderManager>, i32, f32, bool)
        ),
        entry!(
            0x007010e0,
            fader_manager_remove_fader(Ptr<FaderManager>, i32, bool)
        ),
        entry!(
            0x007011d0,
            fader_manager_update_faders(Ptr<FaderManager>, u32)
        ),
        entry!(0x007013e0, fn_007013e0(Ptr) -> f32),
        entry!(0x00701400, fn_00701400(Ptr<FaderManager>, i32) -> bool),
        entry!(0x00701450, fn_00701450(Ptr<FaderManager>, i32) -> bool),
        entry!(
            0x007014a0,
            fader_manager_is_fader_active(Ptr<FaderManager>, i32) -> bool
        ),
        entry!(
            0x007014e0,
            fader_manager_get_fader_alpha(Ptr<FaderManager>, i32) -> f32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// A vtable for the test nodes: slot `+0xdc` attaches a child, slot
    /// `+0xe8` detaches one.
    const VTABLE: u32 = 0x0f30_0000;
    const ATTACH: u32 = 0x0f30_10dc;
    const DETACH: u32 = 0x0f30_10e8;
    const MANAGER_GLOBAL: u32 = 0x011d_8804;
    const PROPERTY_ALPHA_OBJECT: u32 = 0x7a00;

    /// Names given by `NiObjectNET::SetName`: (object, text).
    type Names = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    struct World {
        e: Engine,
        names: Names,
        manager: Ptr<FaderManager>,
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn world() -> World {
        let mut e = Engine::new();
        for page in [
            0x011d_8000u32,
            0x011a_c000,
            0x011f_6000,
            0x0101_2000,
            0x0102_f000,
            0x0101_1000,
            0x0101_d000,
            0x0106_e000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE, 1.0f64);
        e.set_global(ZERO, 0.0f64);
        e.set_global(TWENTY, 20.0f64);
        e.set_global(HALF, 0.5f64);
        e.set_global(THREE_QUARTERS, 0.75f64);
        e.set_global(WIDE_WIDTH, 1280.0f64);
        e.set_global(FOUR_THIRDS, 1.333_333_4f32);
        e.set_global(JUST_BELOW_ONE, f32::from_bits(0x3f7f_f972));
        e.set_global(FRAME_TIME_DIVISOR, 4.0f32);
        e.set_global(AFTER_REMOVE_OBJECT, 0x7b00u32);
        e.mem.set_cstr(NAME_BELOW_MENUS, b"FaderNode Below Menus");
        e.mem.set_cstr(NAME_ABOVE_MENUS, b"FaderNode Above Menus");
        e.mem.set_cstr(NAME_LIGHT, b"FaderNodeLight");
        let manager: Ptr<FaderManager> = e.new_object();
        e.set_global(MANAGER_GLOBAL, manager.addr());
        e.put_vtable(VTABLE, &[0u32; 60]);
        e.mem.set_u32(VTABLE + 0xdc, ATTACH);
        e.mem.set_u32(VTABLE + 0xe8, DETACH);
        for addr in [
            SCOPE_CONSTRUCT,
            SCOPE_DESTROY,
            VECTOR_CONSTRUCTOR,
            ATTACH,
            DETACH,
            PROPERTY_SET_FLAG,
            ATTACH_PROPERTY,
            UPDATE_PROPERTIES,
            UPDATE,
            UPDATE_DATA_CONSTRUCT,
            LIGHT_SET_COLOR,
            NODE_SET_TRANSLATION,
            NODE_SET_SCALE,
            EMPTY_ONE_ARGUMENT,
            STRING_CONSTRUCT,
            STRING_DESTROY,
            FIXED_STRING_DESTROY,
            POINTER_DESTROY,
            SET_TILE_TEXTURE,
            SHADER_PROPERTY_SET_BYTE,
            SET_ALPHA,
            PREPARE_OBJECT,
            COMPUTE_BOUND,
            AFTER_REMOVE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        // Allocators.
        e.register(OPERATOR_NEW, |e, a| word(e.mem.alloc(a[0])));
        e.register(ARRAY_ALLOC, |e, a| word(e.mem.alloc(a[0])));
        e.register(BYTES_ALLOC, |e, a| word(e.mem.alloc(a[0])));
        // Constructors that return the object, with a vtable for the nodes.
        for addr in [NODE_CONSTRUCT, LIGHT_CONSTRUCT] {
            e.register(addr, |e, a| {
                e.mem.set_u32(a[0], VTABLE);
                word(a[0])
            });
        }
        e.register(PROPERTY_CONSTRUCT, |_, a| word(a[0]));
        e.register(SHADER_PROPERTY_CONSTRUCT, |_, a| word(a[0]));
        e.register(TRI_SHAPE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            word(a[0])
        });
        e.register(POINT3_CONSTRUCT, |e, a| {
            for k in 1..4 {
                e.mem.set_u32(a[0] + 4 * (k - 1), a[k as usize]);
            }
            word(a[0])
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            for k in 1..3 {
                e.mem.set_u32(a[0] + 4 * (k - 1), a[k as usize]);
            }
            word(a[0])
        });
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            word(a[0])
        });
        let names: Names = Rc::new(RefCell::new(vec![]));
        let log = names.clone();
        e.register_double(SET_NAME, move |e, a| {
            let text = e.mem.u32(a[1]);
            log.borrow_mut().push((a[0], e.mem.cstr(text)));
            Ret::default()
        });
        e.register(STRING_FORMAT, |e, a| {
            // The texture path: "Data\Textures\Interface\Faders\<name>".
            let name = e.mem.cstr(a[2]);
            let mut path = b"Data\\Textures\\Interface\\Faders\\".to_vec();
            path.extend(name);
            let buffer = e.mem.alloc(80);
            e.mem.set_cstr(buffer, &path);
            e.mem.set_u32(a[0], buffer);
            Ret::default()
        });
        e.register(STRING_C_STR, |e, a| word(e.mem.u32(a[0])));
        e.register(TEXTURE_CREATE, |_, _| word(0x7e00));
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            word(a[0])
        });
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(SHAPE_BOUND, |_, a| word(a[0] + 0x40));
        e.register(NODE_PARENT, |e, a| word(e.mem.u32(a[0] + 0x18)));
        e.register(SCREEN_WIDTH, |_, _| word(1280));
        e.register(SCREEN_HEIGHT, |_, _| word(960));
        e.register(FORCES_UNIT_SCALE, |_, _| word(0));
        e.call_log = Some(vec![]);
        World { e, names, manager }
    }

    fn slot_of(index: i32) -> Ptr<FaderData> {
        fader(index)
    }

    // ----- 007013e0 -----

    #[test]
    fn frame_time_divides_the_float_at_0xc() {
        let mut w = world();
        let object = w.e.mem.alloc(0x20);
        w.e.mem.set_f32(object + 0xc, 0.5);
        assert_eq!(w.e.call(0x0070_13e0, &args![object]).f32(), 0.125);
        // The quotient is rounded to a float.
        w.e.mem.set_f32(object + 0xc, 1.0);
        w.e.set_global(FRAME_TIME_DIVISOR, 3.0f32);
        assert_eq!(w.e.call(0x0070_13e0, &args![object]).f32(), 1.0f32 / 3.0f32);
    }

    // ----- 00701400, 00701450, 007014a0, 007014e0 -----

    fn set_slot(e: &mut Engine, index: i32, node: u32, fading_in: bool, alpha: f32) {
        let slot = slot_of(index);
        e.set(slot, FaderData::pNode, Ptr::new(node));
        e.set(slot, FaderData::bFadingIn, fading_in);
        e.set(slot, FaderData::fAlpha, alpha);
    }

    #[test]
    fn fading_in_below_one_is_reported_by_00701400() {
        let mut w = world();
        let cases = [
            (0x100u32, true, 0.5f32, true),
            (0x100, true, 1.0, false),
            (0x100, true, 1.5, false),
            (0x100, true, f32::NAN, false),
            (0x100, false, 0.5, false),
            (0, true, 0.5, false),
        ];
        for (node, fading_in, alpha, expected) in cases {
            set_slot(&mut w.e, 2, node, fading_in, alpha);
            assert_eq!(
                w.e.call(0x0070_1400, &args![w.manager, 2i32]).bool(),
                expected,
                "{node:#x} {fading_in} {alpha}"
            );
        }
    }

    #[test]
    fn fading_in_at_exactly_one_is_reported_by_00701450() {
        let mut w = world();
        let cases = [
            (0x100u32, true, 1.0f32, true),
            (0x100, true, 0.999_999, false),
            (0x100, true, 1.5, false),
            (0x100, true, f32::NAN, false),
            (0x100, false, 1.0, false),
            (0, true, 1.0, false),
        ];
        for (node, fading_in, alpha, expected) in cases {
            set_slot(&mut w.e, 1, node, fading_in, alpha);
            assert_eq!(
                w.e.call(0x0070_1450, &args![w.manager, 1i32]).bool(),
                expected,
                "{node:#x} {fading_in} {alpha}"
            );
        }
    }

    #[test]
    fn a_fader_is_active_with_a_node_while_fading_in() {
        let mut w = world();
        for (node, fading_in, expected) in [
            (0x100u32, true, true),
            (0x100, false, false),
            (0, true, false),
        ] {
            set_slot(&mut w.e, 0, node, fading_in, 0.25);
            assert_eq!(
                w.e.call(0x0070_14a0, &args![w.manager, 0i32]).bool(),
                expected
            );
        }
    }

    #[test]
    fn the_alpha_getter_reads_the_entry_and_quiets_nan() {
        let mut w = world();
        set_slot(&mut w.e, 2, 0x100, true, 0.375);
        assert_eq!(w.e.call(0x0070_14e0, &args![w.manager, 2i32]).f32(), 0.375);
        // Index 1 is the entry 0x1c bytes after index 0.
        set_slot(&mut w.e, 1, 0x100, true, 0.5);
        assert_eq!(w.e.call(0x0070_14e0, &args![w.manager, 1i32]).f32(), 0.5);
        // A signaling NaN comes out quiet (the FLD quiets it).
        w.e.mem.set_u32(FADERS + 2 * FADER_SIZE + 0x14, 0x7f80_0001);
        let alpha = w.e.call(0x0070_14e0, &args![w.manager, 2i32]).f32();
        assert!(alpha.is_nan());
        assert_eq!(alpha.to_bits() & 0x0040_0000, 0x0040_0000);
    }

    // ----- 007010e0 -----

    /// A node whose parent (at +0x18) is a fresh object with the test
    /// vtable.
    fn node_with_parent(e: &mut Engine) -> (u32, u32) {
        let parent = e.mem.alloc(0x40);
        e.mem.set_u32(parent, VTABLE);
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node, VTABLE);
        e.mem.set_u32(node + 0x18, parent);
        (node, parent)
    }

    #[test]
    fn removing_an_empty_slot_does_nothing() {
        let mut w = world();
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_10e0, &args![w.manager, 1i32, 1u32]);
        assert_eq!(w.e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn a_forced_removal_detaches_the_node_and_clears_the_entry() {
        let mut w = world();
        let (node, parent) = node_with_parent(&mut w.e);
        set_slot(&mut w.e, 1, node, true, 0.75);
        w.e.mem.set_u32(FADERS + FADER_SIZE + 0x18, 0x5555);
        w.e.mem.set_u32(0x011d_ea10, 0x7b00);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_10e0, &args![w.manager, 1i32, 1u32]);
        assert_eq!(calls(&w.e, DETACH), vec![vec![parent, node]]);
        let slot = slot_of(1);
        assert!(w.e.get(slot, FaderData::pNode).is_null());
        assert_eq!(w.e.get(slot, FaderData::fAlpha), 0.0);
        assert_eq!(
            calls(&w.e, POINTER_ASSIGN),
            vec![vec![slot.addr() + 0x18, 0]]
        );
        assert_eq!(w.e.mem.u32(slot.addr() + 0x18), 0);
        assert_eq!(calls(&w.e, AFTER_REMOVE), vec![vec![0x7b00, 0, 0, 0]]);
    }

    #[test]
    fn the_remove_at_once_byte_forces_a_removal_too() {
        let mut w = world();
        let (node, parent) = node_with_parent(&mut w.e);
        set_slot(&mut w.e, 0, node, true, 0.75);
        w.e.set_global(REMOVE_AT_ONCE, 1u8);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_10e0, &args![w.manager, 0i32, 0u32]);
        assert_eq!(calls(&w.e, DETACH), vec![vec![parent, node]]);
    }

    #[test]
    fn an_unforced_removal_lets_the_fader_fade_out() {
        let mut w = world();
        let (node, _) = node_with_parent(&mut w.e);
        set_slot(&mut w.e, 2, node, true, 1.0);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_10e0, &args![w.manager, 2i32, 0u32]);
        let slot = slot_of(2);
        assert!(!w.e.get(slot, FaderData::bFadingIn));
        // An alpha of exactly 1.0 moves to the float just below it.
        assert_eq!(w.e.get(slot, FaderData::fAlpha).to_bits(), 0x3f7f_f972);
        assert_eq!(w.e.get(slot, FaderData::pNode).addr(), node);
        assert!(calls(&w.e, DETACH).is_empty());
        // Another alpha is left alone.
        set_slot(&mut w.e, 2, node, true, 0.5);
        w.e.call(0x0070_10e0, &args![w.manager, 2i32, 0u32]);
        assert_eq!(w.e.get(slot, FaderData::fAlpha), 0.5);
        assert!(!w.e.get(slot, FaderData::bFadingIn));
    }

    // ----- 007011d0 -----

    /// A world with the frame time 0.5 (an object of 2.0 over the divisor
    /// 4.0), a shader property pointer on the slot, and the node of slot
    /// `index` set.
    fn update_world(index: i32, fading_in: bool, alpha: f32, duration: f32) -> (World, u32, u32) {
        let mut w = world();
        w.e.mem.set_f32(FRAME_TIME_OBJECT + 0xc, 2.0);
        let (node, parent) = node_with_parent(&mut w.e);
        set_slot(&mut w.e, index, node, fading_in, alpha);
        let slot = slot_of(index);
        w.e.set(slot, FaderData::fDuration, duration);
        w.e.mem.set_u32(slot.addr() + 0x18, PROPERTY_ALPHA_OBJECT);
        w.e.call_log = Some(vec![]);
        (w, node, parent)
    }

    #[test]
    fn a_fading_in_fader_gains_frame_time_over_duration() {
        let (mut w, _, _) = update_world(0, true, 0.25, 2.0);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        // 0.5 / 2.0 = 0.25 is added.
        assert_eq!(w.e.get(slot_of(0), FaderData::fAlpha), 0.5);
        assert_eq!(
            calls(&w.e, SET_ALPHA),
            vec![vec![PROPERTY_ALPHA_OBJECT, 0.5f32.to_bits()]]
        );
        // The profiling scope is opened and closed around the loop.
        assert_eq!(
            calls(&w.e, SCOPE_CONSTRUCT)[0][1..],
            [0xd, 1, SOURCE_FILE, 0x10a]
        );
        assert_eq!(calls(&w.e, SCOPE_DESTROY).len(), 1);
    }

    #[test]
    fn a_fading_in_fader_is_limited_to_one() {
        let (mut w, _, _) = update_world(1, true, 0.75, 0.25);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        assert_eq!(w.e.get(slot_of(1), FaderData::fAlpha), 1.0);
        assert_eq!(
            calls(&w.e, SET_ALPHA),
            vec![vec![PROPERTY_ALPHA_OBJECT, 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn a_fader_already_at_one_is_not_stepped() {
        let (mut w, _, _) = update_world(2, true, 1.0, 1.0);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        assert_eq!(w.e.get(slot_of(2), FaderData::fAlpha), 1.0);
        assert!(calls(&w.e, SET_ALPHA).is_empty());
    }

    #[test]
    fn fading_in_without_a_shader_property_changes_only_the_alpha() {
        let (mut w, _, _) = update_world(0, true, 0.25, 2.0);
        w.e.mem.set_u32(FADERS + 0x18, 0);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        assert_eq!(w.e.get(slot_of(0), FaderData::fAlpha), 0.5);
        assert!(calls(&w.e, SET_ALPHA).is_empty());
    }

    #[test]
    fn a_fading_out_fader_loses_frame_time_over_duration() {
        let (mut w, _, _) = update_world(1, false, 0.75, 2.0);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        assert_eq!(w.e.get(slot_of(1), FaderData::fAlpha), 0.5);
        assert_eq!(
            calls(&w.e, SET_ALPHA),
            vec![vec![PROPERTY_ALPHA_OBJECT, 0.5f32.to_bits()]]
        );
    }

    #[test]
    fn a_fading_out_fader_is_limited_to_zero() {
        let (mut w, _, _) = update_world(1, false, 0.125, 1.0);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        assert_eq!(w.e.get(slot_of(1), FaderData::fAlpha), 0.0);
        assert_eq!(calls(&w.e, SET_ALPHA), vec![vec![PROPERTY_ALPHA_OBJECT, 0]]);
        // The node is still there: it goes on the next step.
        assert!(!w.e.get(slot_of(1), FaderData::pNode).is_null());
    }

    #[test]
    fn a_fading_out_fader_waits_while_the_frame_counter_runs() {
        let (mut w, _, _) = update_world(1, false, 1.0, 2.0);
        w.e.set_global(FRAMES_TO_WAIT, 2u8);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        assert_eq!(w.e.global::<u8>(FRAMES_TO_WAIT), 1);
        assert_eq!(w.e.get(slot_of(1), FaderData::fAlpha), 1.0);
        // The alpha is still handed to the shader property.
        assert_eq!(
            calls(&w.e, SET_ALPHA),
            vec![vec![PROPERTY_ALPHA_OBJECT, 1.0f32.to_bits()]]
        );
        // A counter of -1 (a signed byte) also counts as running.
        w.e.set_global(FRAMES_TO_WAIT, 0xffu8);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        assert_eq!(w.e.global::<u8>(FRAMES_TO_WAIT), 0xfe);
    }

    #[test]
    fn a_faded_out_fader_is_detached_and_cleared() {
        let (mut w, node, parent) = update_world(2, false, 0.0, 2.0);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        assert_eq!(calls(&w.e, DETACH), vec![vec![parent, node]]);
        let slot = slot_of(2);
        assert!(w.e.get(slot, FaderData::pNode).is_null());
        assert_eq!(w.e.get(slot, FaderData::fAlpha), 0.0);
        assert!(calls(&w.e, SET_ALPHA).is_empty());
    }

    #[test]
    fn a_nan_alpha_fading_out_takes_the_countdown_branch() {
        let (mut w, _, _) = update_world(0, false, f32::NAN, 2.0);
        w.e.set_global(FRAMES_TO_WAIT, 1u8);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        assert!(calls(&w.e, DETACH).is_empty());
        assert_eq!(w.e.global::<u8>(FRAMES_TO_WAIT), 0);
        // The NaN is passed on, quieted.
        let alpha = calls(&w.e, SET_ALPHA)[0][1];
        assert!(f32::from_bits(alpha).is_nan());
    }

    #[test]
    fn empty_slots_are_skipped() {
        let mut w = world();
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        assert!(calls(&w.e, SET_ALPHA).is_empty());
        assert!(calls(&w.e, NODE_PARENT).is_empty());
    }

    // ----- 007006c0 -----

    fn scale_for(width: i32, height: i32, unit: bool) -> f32 {
        let mut w = world();
        w.e.register_double(SCREEN_WIDTH, move |_, _| word(width as u32));
        w.e.register_double(SCREEN_HEIGHT, move |_, _| word(height as u32));
        w.e.register_double(FORCES_UNIT_SCALE, move |_, _| word(unit as u32));
        let graph = w.e.mem.alloc(0x20);
        w.e.mem.set_u32(graph, VTABLE);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_06c0, &args![w.manager, graph, 100.0f32, true]);
        f32::from_bits(calls(&w.e, NODE_SET_SCALE)[0][1])
    }

    #[test]
    fn the_root_scale_follows_the_screen() {
        // 4:3 at the standard width: 1.0.
        assert_eq!(scale_for(1280, 960, false), 1.0);
        // 4:3 at another width: 1280 / width.
        assert_eq!(scale_for(1600, 1200, false), (1280.0f64 / 1600.0) as f32);
        // Another ratio at the standard width: 4/3.
        assert_eq!(scale_for(1280, 720, false), 1.333_333_4f32);
        // Both: (1280 / width) * 4/3.
        assert_eq!(
            scale_for(1920, 1080, false),
            (1280.0f64 / 1920.0 * 1.333_333_4f32 as f64) as f32
        );
        // The unit scale override wins.
        assert_eq!(scale_for(1920, 1080, true), 1.0);
        // A zero width: the ratio is NaN (not 0.75) and 1280 / 0 is infinite.
        assert!(scale_for(0, 0, false).is_infinite());
    }

    #[test]
    fn a_root_below_the_menus_is_named_and_attached() {
        let mut w = world();
        let graph = w.e.mem.alloc(0x20);
        w.e.mem.set_u32(graph, VTABLE);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_06c0, &args![w.manager, graph, 100.0f32, true]);
        let root = w.e.get(w.manager, FaderManager::pRootBelowMenus).addr();
        assert_ne!(root, 0);
        assert!(w.e.get(w.manager, FaderManager::pRootAboveMenus).is_null());
        let names = w.names.borrow();
        assert_eq!(names[0], (root, b"FaderNode Below Menus".to_vec()));
        assert_eq!(names[1].1, b"FaderNodeLight".to_vec());
        let light = names[1].0;
        // The scope, the root's size and the light.
        assert_eq!(
            calls(&w.e, SCOPE_CONSTRUCT)[0][1..],
            [0xd, 1, SOURCE_FILE, 0x50]
        );
        assert_eq!(calls(&w.e, SCOPE_DESTROY).len(), 1);
        assert_eq!(calls(&w.e, OPERATOR_NEW), vec![vec![0xac], vec![0xfc]]);
        assert_eq!(
            calls(&w.e, POINT3_CONSTRUCT)[0][1..],
            [1.0f32.to_bits(), 1.0f32.to_bits(), 1.0f32.to_bits()]
        );
        assert_eq!(calls(&w.e, LIGHT_SET_COLOR)[0][0], light);
        assert_eq!(calls(&w.e, EMPTY_ONE_ARGUMENT), vec![vec![light, root]]);
        // The light is attached to the root, the root to the graph.
        assert_eq!(
            calls(&w.e, ATTACH),
            vec![vec![root, light, 1], vec![graph, root, 1]]
        );
        // The translation is (0, distance, 0).
        let translation = calls(&w.e, POINT3_CONSTRUCT)[1].clone();
        assert_eq!(translation[1..], [0, 100.0f32.to_bits(), 0]);
        assert_eq!(calls(&w.e, NODE_SET_TRANSLATION)[0][0], root);
    }

    #[test]
    fn a_root_above_the_menus_is_twenty_nearer() {
        let mut w = world();
        let graph = w.e.mem.alloc(0x20);
        w.e.mem.set_u32(graph, VTABLE);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_06c0, &args![w.manager, graph, 100.0f32, false]);
        let root = w.e.get(w.manager, FaderManager::pRootAboveMenus).addr();
        assert!(w.e.get(w.manager, FaderManager::pRootBelowMenus).is_null());
        assert_eq!(
            w.names.borrow()[0],
            (root, b"FaderNode Above Menus".to_vec())
        );
        let translation = calls(&w.e, POINT3_CONSTRUCT)[1].clone();
        assert_eq!(translation[2], 80.0f32.to_bits());
    }

    // ----- 00700960 -----

    fn named_slot(w: &mut World, index: i32, below_menus: bool) {
        let name = w.e.mem.alloc(16);
        w.e.mem.set_cstr(name, b"Black");
        let slot = slot_of(index);
        w.e.set(slot, FaderData::pName, name);
        w.e.set(slot, FaderData::bBelowMenus, below_menus);
    }

    fn roots(w: &mut World) -> (u32, u32) {
        let below = w.e.mem.alloc(0x40);
        let above = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(below, VTABLE);
        w.e.mem.set_u32(above, VTABLE);
        w.e.set(w.manager, FaderManager::pRootBelowMenus, Ptr::new(below));
        w.e.set(w.manager, FaderManager::pRootAboveMenus, Ptr::new(above));
        (below, above)
    }

    #[test]
    fn creating_a_fader_builds_the_quad_and_the_table_entry() {
        let mut w = world();
        let (below, above) = roots(&mut w);
        named_slot(&mut w, 1, false);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_0960, &args![w.manager, 1i32, 2.0f32, false]);
        let slot = slot_of(1);
        let node = w.e.get(slot, FaderData::pNode).addr();
        assert_ne!(node, 0);
        // The scope, and a quad as wide and high as the screen.
        assert_eq!(
            calls(&w.e, SCOPE_CONSTRUCT)[0][1..],
            [0xd, 1, SOURCE_FILE, 0x87]
        );
        // NiTriShape(4 vertices, vertices, no normals, no colors, texture
        // coordinates, 1 set, 0, 2 triangles, triangle table).
        let shape = calls(&w.e, TRI_SHAPE_CONSTRUCT)[0].clone();
        assert_eq!(shape[1], 4);
        let (vertices, uvs, triangles) = (shape[2], shape[5], shape[9]);
        assert_eq!(shape[3..5], [0, 0]);
        assert_eq!(shape[6..9], [1, 0, 2]);
        let floats = |address: u32, count: u32| -> Vec<f32> {
            (0..count).map(|i| w.e.mem.f32(address + 4 * i)).collect()
        };
        assert_eq!(
            floats(vertices, 12),
            [-640.0, 0.0, 480.0, -640.0, 0.0, -480.0, 640.0, 0.0, 480.0, 640.0, 0.0, -480.0]
        );
        assert_eq!(floats(uvs, 8), [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]);
        let indices: Vec<u16> = (0..6).map(|i| w.e.mem.u16(triangles + 2 * i)).collect();
        assert_eq!(indices, [0, 1, 2, 2, 1, 3]);
        // Allocations: two element arrays, the triangle table, the node, the
        // property, the shape and the shader property.
        assert_eq!(calls(&w.e, ARRAY_ALLOC), vec![vec![0x30], vec![0x20]]);
        assert_eq!(calls(&w.e, BYTES_ALLOC), vec![vec![0xc]]);
        assert_eq!(
            calls(&w.e, OPERATOR_NEW),
            vec![vec![0xac], vec![0x1c], vec![0xc4], vec![0xb0]]
        );
        assert_eq!(
            calls(&w.e, VECTOR_CONSTRUCTOR)
                .iter()
                .map(|c| c[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![0xc, 4, THIS_RETURNER], vec![8, 4, THIS_RETURNER]]
        );
        // Names: the node by the entry's name, the shape by the texture path.
        let names = w.names.borrow().clone();
        assert_eq!(names[0], (node, b"Black".to_vec()));
        let shape_object = shape[0];
        assert_eq!(
            names[1],
            (
                shape_object,
                b"Data\\Textures\\Interface\\Faders\\Black".to_vec()
            )
        );
        // The node holds the shape and hangs under the root above the menus.
        assert_eq!(calls(&w.e, ATTACH)[0], vec![node, shape_object, 1]);
        assert_eq!(calls(&w.e, ATTACH)[1], vec![above, node, 1]);
        assert_ne!(above, below);
        // The table entry.
        assert_eq!(w.e.get(slot, FaderData::fDuration), 2.0);
        assert!(w.e.get(slot, FaderData::bFadingIn));
        assert_eq!(w.e.get(slot, FaderData::fAlpha), 0.0);
        assert_eq!(calls(&w.e, SET_ALPHA)[0][1], 0);
        assert_eq!(w.e.global::<u8>(FRAMES_TO_WAIT), 0);
        let shader_property = w.e.mem.u32(slot.addr() + 0x18);
        assert_ne!(shader_property, 0);
        assert_eq!(
            calls(&w.e, SET_TILE_TEXTURE),
            vec![vec![shader_property, 0x7e00]]
        );
        assert_eq!(
            calls(&w.e, SHADER_PROPERTY_SET_BYTE),
            vec![vec![shader_property, 3]]
        );
        assert_eq!(
            calls(&w.e, ATTACH_PROPERTY)[1],
            vec![shape_object, shader_property]
        );
        assert_eq!(calls(&w.e, PREPARE_OBJECT), vec![vec![shape_object, 0, 0]]);
        assert_eq!(
            calls(&w.e, COMPUTE_BOUND),
            vec![vec![shape_object + 0x40, 4, vertices]]
        );
        // The root is updated with an update data of time 0.
        assert_eq!(calls(&w.e, UPDATE_PROPERTIES), vec![vec![above]]);
        assert_eq!(calls(&w.e, UPDATE_DATA_CONSTRUCT)[0][1..], [0, 0, 0]);
        assert_eq!(calls(&w.e, UPDATE)[0][0], above);
        assert_eq!(calls(&w.e, SCOPE_DESTROY).len(), 1);
        assert_eq!(calls(&w.e, TEXTURE_CREATE)[0][1..], [TEXTURE_PREFS, 1, 0]);
    }

    #[test]
    fn an_entry_below_the_menus_hangs_under_the_other_root() {
        let mut w = world();
        let (below, _) = roots(&mut w);
        named_slot(&mut w, 0, true);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_0960, &args![w.manager, 0i32, 1.0f32, false]);
        let node = w.e.get(slot_of(0), FaderData::pNode).addr();
        assert_eq!(calls(&w.e, ATTACH)[1], vec![below, node, 1]);
    }

    #[test]
    fn an_immediate_fader_starts_opaque() {
        let mut w = world();
        roots(&mut w);
        named_slot(&mut w, 2, false);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_0960, &args![w.manager, 2i32, 2.0f32, true]);
        let slot = slot_of(2);
        assert_eq!(w.e.get(slot, FaderData::fAlpha), 1.0);
        assert!(!w.e.get(slot, FaderData::bFadingIn));
        assert_eq!(w.e.global::<u8>(FRAMES_TO_WAIT), 3);
        assert_eq!(calls(&w.e, SET_ALPHA)[0][1], 1.0f32.to_bits());
    }

    #[test]
    fn a_zero_duration_also_starts_opaque_but_still_fades_in() {
        let mut w = world();
        roots(&mut w);
        named_slot(&mut w, 0, false);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_0960, &args![w.manager, 0i32, 0.0f32, false]);
        let slot = slot_of(0);
        assert_eq!(w.e.get(slot, FaderData::fAlpha), 1.0);
        assert!(w.e.get(slot, FaderData::bFadingIn));
        assert_eq!(w.e.global::<u8>(FRAMES_TO_WAIT), 3);
        // A NaN duration is not zero.
        let mut w = world();
        roots(&mut w);
        named_slot(&mut w, 0, false);
        w.e.call(0x0070_0960, &args![w.manager, 0i32, f32::NAN, false]);
        assert_eq!(w.e.get(slot_of(0), FaderData::fAlpha), 0.0);
        assert_eq!(w.e.global::<u8>(FRAMES_TO_WAIT), 0);
    }

    #[test]
    fn a_fader_already_fading_in_is_left_alone() {
        let mut w = world();
        roots(&mut w);
        named_slot(&mut w, 1, false);
        let (node, _) = node_with_parent(&mut w.e);
        set_slot(&mut w.e, 1, node, true, 0.5);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_0960, &args![w.manager, 1i32, 2.0f32, false]);
        assert_eq!(w.e.get(slot_of(1), FaderData::pNode).addr(), node);
        assert!(calls(&w.e, OPERATOR_NEW).is_empty());
        assert_eq!(calls(&w.e, SCOPE_DESTROY).len(), 1);
    }

    #[test]
    fn a_fading_out_fader_is_removed_before_the_new_one_is_made() {
        let mut w = world();
        roots(&mut w);
        named_slot(&mut w, 1, false);
        let (node, parent) = node_with_parent(&mut w.e);
        set_slot(&mut w.e, 1, node, false, 0.5);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_0960, &args![w.manager, 1i32, 2.0f32, false]);
        assert_eq!(calls(&w.e, DETACH), vec![vec![parent, node]]);
        assert_ne!(w.e.get(slot_of(1), FaderData::pNode).addr(), node);
        assert_eq!(calls(&w.e, OPERATOR_NEW).len(), 4);
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on this thread takes (asked of the engine), with
    /// the call log switched on.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.call_log = Some(vec![]);
        e.next_stack_local(size)
    }

    fn first_words(e: &Engine, addr: u32) -> Vec<u32> {
        calls(e, addr).iter().map(|w| w[0]).collect()
    }

    #[test]
    fn the_fader_frames_keep_their_locals_at_the_exe_offsets() {
        // 007006c0 (0x88 bytes), below the menus: the scope at +0x6c, the names at +0x58 and
        // +0x48, the colour at +0x3c, the position at +0x30.
        let mut w = world();
        let graph = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(graph, VTABLE);
        let frame = next_frame(&mut w.e, 0x88);
        w.e.call(0x0070_06c0, &args![w.manager, graph, 100.0f32, true]);
        assert_eq!(first_words(&w.e, SCOPE_CONSTRUCT), vec![frame + 0x6c]);
        assert_eq!(
            first_words(&w.e, FIXED_STRING_CONSTRUCT),
            vec![frame + 0x58, frame + 0x48]
        );
        assert_eq!(
            first_words(&w.e, POINT3_CONSTRUCT),
            vec![frame + 0x3c, frame + 0x30]
        );
        // Above the menus the root's name is at +0x54.
        let frame = next_frame(&mut w.e, 0x88);
        w.e.call(0x0070_06c0, &args![w.manager, graph, 100.0f32, false]);
        assert_eq!(first_words(&w.e, FIXED_STRING_CONSTRUCT)[0], frame + 0x54);

        // 00700960 (0x11c bytes).
        let mut w = world();
        roots(&mut w);
        named_slot(&mut w, 1, false);
        let frame = next_frame(&mut w.e, 0x11c);
        w.e.call(0x0070_0960, &args![w.manager, 1i32, 2.0f32, false]);
        let at = |offsets: &[u32]| offsets.iter().map(|o| frame + o).collect::<Vec<u32>>();
        assert_eq!(first_words(&w.e, SCOPE_CONSTRUCT), at(&[0xfc]));
        assert_eq!(
            first_words(&w.e, POINT3_CONSTRUCT),
            at(&[0xb0, 0xa4, 0x98, 0x8c])
        );
        assert_eq!(
            first_words(&w.e, POINT2_CONSTRUCT),
            at(&[0x84, 0x7c, 0x74, 0x6c])
        );
        assert_eq!(
            first_words(&w.e, FIXED_STRING_CONSTRUCT),
            at(&[0x60, 0x4c, 0x48])
        );
        assert_eq!(first_words(&w.e, STRING_CONSTRUCT), at(&[0xe4]));
        assert_eq!(first_words(&w.e, POINTER_CONSTRUCT), at(&[0xd4]));
        assert_eq!(first_words(&w.e, UPDATE_DATA_CONSTRUCT), at(&[0xd8]));

        // 007011d0 (0x24 bytes): the scope at +0x14.
        let frame = next_frame(&mut w.e, 0x24);
        w.e.call(0x0070_11d0, &args![w.manager, 1u32]);
        assert_eq!(first_words(&w.e, SCOPE_CONSTRUCT), vec![frame + 0x14]);
    }
}
