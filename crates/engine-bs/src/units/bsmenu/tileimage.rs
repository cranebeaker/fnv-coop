//! `BSMenu/tileimage.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TileImage` is the textured quad of the menu system: a one-quad
//! `NiTriShape` (a `BSScissorTriShape` when the tile clips) with a
//! `TileShaderProperty`, under an `NiNode`. The unit also finds the
//! textures for image files (`AddToTesTextures` and the path helper at
//! `00a1f7d0`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsmenu::font::{
    release_after_use, ALLOCATE, BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR,
    INTERFACE_ATTACH_DEFAULT_ALPHA_PROPERTY, INTERLOCKED_DECREMENT, MEMORY_MANAGER, NI_ALLOC,
    NI_AV_OBJECT_ATTACH_PROPERTY, NI_AV_OBJECT_UPDATE_PROPERTIES, NI_FIXED_STRING_CREATE,
    NI_FIXED_STRING_EMPTY, NI_NEW, NI_OBJECT_NET_SET_NAME, NI_SOURCE_TEXTURE_CREATE,
    TEXTURE_FORMAT_PREFS, TILE_SHADER_PROPERTY_CONSTRUCTOR,
};
use crate::units::bsmenu::tile::{
    Tile, TRAIT_ALPHA, TRAIT_BLUE, TRAIT_BRIGHTNESS, TRAIT_CLIPS, TRAIT_DEPTH, TRAIT_FILENAME,
    TRAIT_GREEN, TRAIT_HEIGHT, TRAIT_RED, TRAIT_TEX_ATLAS, TRAIT_VISIBLE, TRAIT_WIDTH, TRAIT_X,
    TRAIT_Y,
};

layout! {
    /// `TileImage` (Xbox PDB), 0x48 bytes on PC as on the Xbox.
    pub struct TileImage: 0x48 {
        /// `fScale` (Xbox PDB).
        0x38 fScale: f32,
        /// `spTexture` (Xbox PDB): `NiPointer<NiTexture>`.
        0x3C spTexture: Ptr,
        /// `spTSP` (Xbox PDB): `NiPointer<TileShaderProperty>`.
        0x40 spTSP: Ptr,
        /// `bIsScissor` (Xbox PDB).
        0x44 bIsScissor: bool,
    }
}

/// `Tile::Init` (`009ff600`, Xbox PDB name).
const TILE_INIT: u32 = 0x009f_f600;
/// Sets a float trait (`00a012d0`, thiscall: trait, value, flag).
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// Sets a string trait (`00a01350`, thiscall: trait, string, flag).
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `Tile::SetNeedsUpdate` (`00a074d0`) and `Tile::AddNeedsUpdate`
/// (`00a07530`, Xbox PDB names).
const TILE_SET_NEEDS_UPDATE: u32 = 0x00a0_74d0;
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// `Tile::GetValue` (`00a00f30`, Xbox PDB name).
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// `Tile::GetParentModel` (`00a089c0`), `Tile::IsTrue` (`00a01230`), the
/// float value of a trait (`00a011b0`, `ST0`) and `Tile::FinalPostParse`
/// (`00a037e0`).
const TILE_GET_PARENT_MODEL: u32 = 0x00a0_89c0;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_FINAL_POST_PARSE: u32 = 0x00a0_37e0;
/// `Tile::Extra::Extra` (`00a1f610`, Xbox PDB name, in `tilerect.cpp`).
const TILE_EXTRA_CONSTRUCTOR: u32 = 0x00a1_f610;
/// `NiPointer` assignment from a pointer (`0066b0d0`) and from another
/// `NiPointer` (`006e5cc0`, thiscall with the source slot's address).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_ASSIGN_POINTER: u32 = 0x006e_5cc0;
/// The `NiNode` constructor (`00a5ecb0`, thiscall with a child count).
const NI_NODE_CONSTRUCTOR: u32 = 0x00a5_ecb0;
/// `NiTriShape::NiTriShape` (`00a74410`, Xbox PDB name; nine words).
const NI_TRI_SHAPE_CONSTRUCTOR: u32 = 0x00a7_4410;
/// `NiGeometryData::SetConsistency` (`00a67050`, Xbox PDB name).
const NI_GEOMETRY_DATA_SET_CONSISTENCY: u32 = 0x00a6_7050;
/// `BSShaderManager::GetShader` (`00b55560`, Xbox PDB name, cdecl).
const BS_SHADER_MANAGER_GET_SHADER: u32 = 0x00b5_5560;
/// `NiObjectNET::AddExtraData` (`00a5bca0`, Xbox PDB name `_ov2`).
const NI_OBJECT_NET_ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
/// `BSTexturePalette::GetTexture` (`00a61b90`, Xbox PDB name `_ov2`, cdecl:
/// name, `NiPointer` slot) and `BSTexturePalette::SetTexture` (`00a61c50`,
/// cdecl).
const TEXTURE_PALETTE_GET_TEXTURE: u32 = 0x00a6_1b90;
const TEXTURE_PALETTE_SET_TEXTURE: u32 = 0x00a6_1c50;
/// The file finder's look-up (`00afe0d0`, BSSystem/filefinder.cpp,
/// thiscall: path, second path, 6 or 0, -1) on the finder at `011f81dc`.
const FILE_FINDER_FIND: u32 = 0x00af_e0d0;
const FILE_FINDER: u32 = 0x011f_81dc;
/// `Interface::GetInterfaceRoot` (`00706d30`), `GetScreenHeight`
/// (`00706df0`), `GetScreenWidth` (`00706e00`), `GetScreenAspectRatio`
/// (`00706e70`) and `SetNeedToUpdate` (`00707930`) (Xbox PDB names, cdecl).
const INTERFACE_GET_INTERFACE_ROOT: u32 = 0x0070_6d30;
const INTERFACE_GET_SCREEN_HEIGHT: u32 = 0x0070_6df0;
const INTERFACE_GET_SCREEN_WIDTH: u32 = 0x0070_6e00;
const INTERFACE_GET_SCREEN_ASPECT_RATIO: u32 = 0x0070_6e70;
const INTERFACE_SET_NEED_TO_UPDATE: u32 = 0x0070_7930;
/// `Error` (`0040fbe0`, Xbox PDB name, cdecl with a format).
const ERROR: u32 = 0x0040_fbe0;
/// `_strcpy_s` (`00ec65a6`), `_strcat_s` (`00ec6bd2`), `strstr`
/// (`00ec7750`) and `_memset` (`00ec61c0`), cdecl.
const STRCPY_S: u32 = 0x00ec_65a6;
const STRCAT_S: u32 = 0x00ec_6bd2;
const STRSTR: u32 = 0x00ec_7750;
const MEMSET: u32 = 0x00ec_61c0;
/// `_ftol2_sse` (`00ec62c0`): truncates the value in ST0 (an `f64` here).
const FTOL: u32 = 0x00ec_62c0;
/// The `InterlockedIncrement` import slot (`00fdf1c0`).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The `NiNode` virtual slot that attaches a child (`+0xDC`, second word 1).
const NODE_ATTACH_CHILD_SLOT: u32 = 0xdc;
/// `NiRefObject::DeleteThis` (virtual slot `+4`).
const DELETE_THIS_SLOT: u32 = 4;
/// The `TLS` word the allocations are tagged with, and the tag.
const ALLOCATION_TAG_OFFSET: u32 = 0x2b4;
const ALLOCATION_TAG: u32 = 0xd;

/// `255.0f` (`01023cd8`): `Init`'s brightness, colour and alpha.
const FULL: u32 = 0x0102_3cd8;
/// `-1.0f` (`01012054`): the quad's second coordinate.
const MINUS_ONE: u32 = 0x0101_2054;
/// `1.0` (`double`, `01012070`).
const ONE: u32 = 0x0101_2070;
/// `"Data\Textures\"` (`01033e9c`).
const TEXTURES_FOLDER: u32 = 0x0103_3e9c;
/// The global whose word at +4 is the path used when an image is not found
/// (`011d33f0`; the code tests the global's address, a constant, for
/// null, so its 0 branch never runs).
const MISSING_TEXTURE: u32 = 0x011d_33f0;
/// `"oading\load_"` (`01095700`): loading-screen images are not added to
/// the texture palette.
const LOADING_SCREEN_PART: u32 = 0x0109_5700;
/// `"*** ERROR: Could not create ptexture '%s'!"` (`010956d4`).
const COULD_NOT_CREATE_TEXTURE: u32 = 0x0109_56d4;
/// `"Interface: TileImage"` (`01095710`).
const TILE_IMAGE_NAME: u32 = 0x0109_5710;
/// The empty string (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// The size of the path buffers (`MAX_PATH`, 0x104).
const PATH_SIZE: u32 = 0x104;

/// Releases an `NiPointer` held in a local: drops the reference and runs
/// `DeleteThis` when it was the last one.
fn release_object(e: &mut Engine, object: u32) {
    if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, DELETE_THIS_SLOT, &args![]);
    }
}

/// A temporary `NiFixedString` of `text`, or the empty handle when `text`
/// is null.
fn fixed_string(e: &mut Engine, text: u32) -> u32 {
    if text == 0 {
        e.global::<u32>(NI_FIXED_STRING_EMPTY)
    } else {
        e.call(NI_FIXED_STRING_CREATE, &args![text]).u32()
    }
}

/// Drops a fixed string handle unless it is the empty one.
fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(NI_FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// The missing-image path: the word at +4 of the global at `011d33f0`.
fn missing_texture_path(e: &mut Engine) -> u32 {
    let global = std::hint::black_box(MISSING_TEXTURE);
    if global == 0 {
        0
    } else {
        e.mem.u32(global + 4)
    }
}

// Translated from 00a1f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileImage::Init` (Xbox PDB): `Tile::Init` with the same three words;
/// brightness, red, green, blue and alpha the float at `01023cd8`, depth
/// and visible 1 (each with flag word 1); the update bits 3; scale 1, no
/// texture and not a scissor image.
pub fn tile_image_init(
    e: &mut Engine,
    this: Ptr<TileImage>,
    parent: Ptr<Tile>,
    name: Ptr,
    set_parent_arg: u32,
) {
    e.call(TILE_INIT, &args![this, parent, name, set_parent_arg]);
    for trait_id in [
        TRAIT_BRIGHTNESS,
        TRAIT_RED,
        TRAIT_GREEN,
        TRAIT_BLUE,
        TRAIT_ALPHA,
    ] {
        let full = x87_quiet_bits(e.global::<u32>(FULL));
        e.call(TILE_SET_FLOAT, &args![this, trait_id, full, 1u32]);
    }
    for trait_id in [TRAIT_DEPTH, TRAIT_VISIBLE] {
        e.call(TILE_SET_FLOAT, &args![this, trait_id, 1.0f32, 1u32]);
    }
    e.call(TILE_SET_NEEDS_UPDATE, &args![this, 3u32]);
    e.set(this, TileImage::fScale, 1.0);
    e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x3c), 0u32]);
    e.set(this, TileImage::bIsScissor, false);
}

// Translated from 00a1f7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The image path helper of `AddToTesTextures` (no PDB name, cdecl): for a
/// `path` that is not empty and does not start with a space, sets `*scale`
/// to 1 and writes `"Data\Textures\" + path` into `buffer` (0x104 bytes).
/// When the texture palette has no texture of that name and the file
/// finder (`(buffer, buffer, 6, -1)`) does not find it either, `buffer`
/// becomes the missing-image path (the word at `011d33f0 + 4`); were
/// `*scale` not 1 there, the path would be built and looked for again
/// (`(buffer, 0, 6, -1)`), setting the scale to 1 when found. The screen
/// aspect ratio is asked for and not used, and so is the third word.
// Translation incomplete: 00a1f7d0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fn_00a1f7d0(e: &mut Engine, path: Ptr, scale: Ptr, _unused_2: u32, buffer: Ptr) {
    if path.is_null() {
        return;
    }
    let first = e.mem.i8(path.addr());
    if first == 0 || first == 0x20 {
        return;
    }
    e.call(INTERFACE_GET_SCREEN_ASPECT_RATIO, &args![]);
    e.mem.set_f32(scale.addr(), 1.0);
    e.call(STRCPY_S, &args![buffer, PATH_SIZE, TEXTURES_FOLDER]);
    e.call(STRCAT_S, &args![buffer, PATH_SIZE, path]);
    e.with_stack(4, |e, texture| {
        // The `NiPointer<NiTexture>` local starts null; its add-reference
        // for a non-null start never runs.
        let start = std::hint::black_box(0u32);
        if start != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![start + 4]);
        }
        e.mem.set_u32(texture.addr(), start);
        let handle = fixed_string(e, buffer.addr());
        e.call(TEXTURE_PALETTE_GET_TEXTURE, &args![handle, texture]);
        if e.mem.u32(texture.addr()) == 0 {
            let finder = e.global::<u32>(FILE_FINDER);
            let found = e
                .call(
                    FILE_FINDER_FIND,
                    &args![finder, buffer, buffer, 6u32, -1i32],
                )
                .u32();
            if found == 0 {
                let current = f64::from(e.mem.f32(scale.addr()));
                if current == e.global::<f64>(ONE) {
                    let missing = missing_texture_path(e);
                    e.call(STRCPY_S, &args![buffer, PATH_SIZE, missing]);
                } else {
                    e.call(STRCPY_S, &args![buffer, PATH_SIZE, TEXTURES_FOLDER]);
                    e.call(STRCAT_S, &args![buffer, PATH_SIZE, path]);
                    let finder = e.global::<u32>(FILE_FINDER);
                    let found = e
                        .call(FILE_FINDER_FIND, &args![finder, buffer, 0u32, 6u32, -1i32])
                        .u32();
                    if found != 0 {
                        e.mem.set_f32(scale.addr(), 1.0);
                    } else {
                        let missing = missing_texture_path(e);
                        e.call(STRCPY_S, &args![buffer, PATH_SIZE, missing]);
                    }
                }
            }
        }
        release_fixed_string(e, handle);
        let texture = e.mem.u32(texture.addr());
        release_object(e, texture);
    });
}

// Translated from 00a1fa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileImage::AddToTesTextures` (Xbox PDB, cdecl): with no `source`
/// texture, builds the image's path ([`fn_00a1f7d0`] into a zeroed
/// 0x400-byte buffer, passing `hint` as its unread third word) and, for a
/// non-empty path, asks the texture palette for it into `*texture`; when
/// the palette has none and the file finder finds the file (`(path, 0, 0,
/// -1)`), creates an `NiSourceTexture` of it (with the allocation tag
/// `0xD` in `TLS + 0x2b4`) into `*texture`, and adds it to the palette
/// unless it is a loading-screen image, or reports
/// `"*** ERROR: Could not create ptexture '%s'!"` when that failed. With
/// a `source`, `*texture` takes it. The reference `source` holds (an
/// `NiPointer` passed by value) is dropped at the end.
// Translation incomplete: 00a1fa20 - the compiler's exception frame (FS:[0] chain) and the /GS stack cookie check (00ec408c) are not translated
pub fn tile_image_add_to_tes_textures(
    e: &mut Engine,
    path: Ptr,
    scale: Ptr,
    texture: Ptr,
    hint: f32,
    source: Ptr,
) {
    e.with_stack(4, |e, source_slot| {
        e.mem.set_u32(source_slot.addr(), source.addr());
        if source.is_null() {
            e.with_stack(0x400, |e, buffer| {
                e.mem.set_u8(buffer.addr(), 0);
                e.call(MEMSET, &args![buffer.addr() + 1, 0u32, 0x3ffu32]);
                fn_00a1f7d0(e, path, scale, hint.to_bits(), buffer);
                if e.mem.i8(buffer.addr()) != 0 {
                    let mut handle = fixed_string(e, buffer.addr());
                    e.call(TEXTURE_PALETTE_GET_TEXTURE, &args![handle, texture]);
                    if e.mem.u32(texture.addr()) == 0 {
                        let finder = e.global::<u32>(FILE_FINDER);
                        let found = e
                            .call(FILE_FINDER_FIND, &args![finder, buffer, 0u32, 0u32, -1i32])
                            .u32();
                        if found != 0 {
                            // `NiFixedString = const char *`: replaced
                            // unless it already is that text.
                            if handle != buffer.addr() {
                                let old = handle;
                                handle = e.call(NI_FIXED_STRING_CREATE, &args![buffer]).u32();
                                release_fixed_string(e, old);
                            }
                            let tls = e.tls();
                            let saved = e.mem.u32(tls + ALLOCATION_TAG_OFFSET);
                            e.mem.set_u32(tls + ALLOCATION_TAG_OFFSET, ALLOCATION_TAG);
                            let created = e.with_stack(4, |e, name| {
                                e.mem.set_u32(name.addr(), handle);
                                let created = e
                                    .call(
                                        NI_SOURCE_TEXTURE_CREATE,
                                        &args![name, TEXTURE_FORMAT_PREFS, 1u32, 0u32],
                                    )
                                    .u32();
                                handle = e.mem.u32(name.addr());
                                created
                            });
                            e.call(NI_POINTER_ASSIGN, &args![texture, created]);
                            let made = e.mem.u32(texture.addr());
                            if made != 0 {
                                let loading =
                                    e.call(STRSTR, &args![buffer, LOADING_SCREEN_PART]).u32();
                                if loading == 0 {
                                    let made = e.mem.u32(texture.addr());
                                    e.call(TEXTURE_PALETTE_SET_TEXTURE, &args![made, 0u32]);
                                }
                            } else {
                                e.call(ERROR, &args![COULD_NOT_CREATE_TEXTURE, buffer]);
                            }
                            let tls = e.tls();
                            e.mem.set_u32(tls + ALLOCATION_TAG_OFFSET, saved);
                        }
                    }
                    release_fixed_string(e, handle);
                }
            });
        } else {
            e.call(NI_POINTER_ASSIGN_POINTER, &args![texture, source_slot]);
        }
        let source = e.mem.u32(source_slot.addr());
        release_object(e, source);
    });
}

/// `new NiPoint3[4]` / `new NiPoint2[4]` through the memory manager (the
/// element loops construct nothing).
fn allocate_points(e: &mut Engine, bytes: u32) -> u32 {
    e.call(ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32()
}

/// Writes the `float` words at `address`.
fn put_floats(e: &mut Engine, address: u32, values: &[u32]) {
    for (i, value) in values.iter().enumerate() {
        e.mem.set_u32(address + 4 * i as u32, *value);
    }
}

// Translated from 00a1fd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileImage::MakeNode` (Xbox PDB): builds the image's quad: four vertices
/// `(0,0,0) (0,0,-1) (1,0,0) (1,0,-1)` (the `-1` read from `01012054`),
/// texture coordinates `(0,0) (0,1) (1,0) (1,1)`, triangles `0 1 2` and
/// `2 1 3`, no normals; as a `BSScissorTriShape` (0xD4 bytes, with the
/// screen width and height truncated to integers) when the clips trait is
/// true, else an `NiTriShape` (0xC4 bytes), and records which in
/// `bIsScissor`. Names it `"Interface: TileImage"`, sets its data's
/// consistency to 0, gives it a new `TileShaderProperty` (also kept in
/// `spTSP`) and the shader `BSShaderManager::GetShader(0x20)` (geometry
/// +0xC0), puts it under a new `NiNode` that becomes the model, attaches
/// the model to the nearest ancestor's model or the interface root, names
/// the model after the tile, attaches the default alpha property to the
/// geometry, updates the model's properties, tells the interface it needs
/// an update, adds the tile's `Tile::Extra`, runs `FinalPostParse` on the
/// depth, queues the scissor update (`0x100`) for a clipping image and the
/// updates `0x3D`, and returns the model. Failed allocations give null
/// objects, as in the exe.
// Translation incomplete: 00a1fd50 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn tile_image_make_node(e: &mut Engine, this: Ptr<TileImage>) -> Ptr {
    let tile = this.cast::<Tile>();
    let mut parent = e.call(TILE_GET_PARENT_MODEL, &args![this]).u32();
    e.with_stack(4, |e, geometry_slot| {
        // The `NiPointer<NiGeometry>` local starts null (its add-reference
        // for a non-null start never runs).
        let start = std::hint::black_box(0u32);
        if start != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![start + 4]);
        }
        e.mem.set_u32(geometry_slot.addr(), start);

        let vertices = allocate_points(e, 0x30);
        let uvs = allocate_points(e, 0x20);
        let triangles = e.call(NI_ALLOC, &args![0x0cu32]).u32();
        let normals = 0u32;
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        let zero = 0.0f32.to_bits();
        let one = 1.0f32.to_bits();
        put_floats(e, vertices, &[zero, zero, zero]);
        put_floats(e, vertices + 0x0c, &[zero, zero, minus_one]);
        put_floats(e, vertices + 0x18, &[one, zero, zero]);
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        put_floats(e, vertices + 0x24, &[one, zero, minus_one]);
        for (i, index) in [0u16, 1, 2, 2, 1, 3].iter().enumerate() {
            e.mem.set_u16(triangles + 2 * i as u32, *index);
        }
        put_floats(e, uvs, &[zero, zero, zero, one, one, zero, one, one]);

        if e.call(TILE_IS_TRUE, &args![this, TRAIT_CLIPS]).bool() {
            let block = e.call(NI_NEW, &args![0xd4u32]).u32();
            let shape = if block == 0 {
                0
            } else {
                let height = e.call(INTERFACE_GET_SCREEN_HEIGHT, &args![]).f64();
                let height = e.call(FTOL, &args![height]).u32();
                let width = e.call(INTERFACE_GET_SCREEN_WIDTH, &args![]).f64();
                let width = e.call(FTOL, &args![width]).u32();
                e.call(
                    BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR,
                    &args![
                        block, 4u32, vertices, normals, 0u32, uvs, 1u32, 0u32, 2u32, triangles,
                        0u32, 0u32, width, height
                    ],
                )
                .u32()
            };
            e.call(NI_POINTER_ASSIGN, &args![geometry_slot, shape]);
            e.set(this, TileImage::bIsScissor, true);
        } else {
            let block = e.call(NI_NEW, &args![0xc4u32]).u32();
            let shape = if block == 0 {
                0
            } else {
                e.call(
                    NI_TRI_SHAPE_CONSTRUCTOR,
                    &args![block, 4u32, vertices, normals, 0u32, uvs, 1u32, 0u32, 2u32, triangles],
                )
                .u32()
            };
            e.call(NI_POINTER_ASSIGN, &args![geometry_slot, shape]);
            e.set(this, TileImage::bIsScissor, false);
        }

        // The name's address is a constant, so its empty-handle branch
        // never runs; it is kept as the exe has it.
        let name = std::hint::black_box(TILE_IMAGE_NAME);
        let handle = fixed_string(e, name);
        release_after_use(e, handle, |e, local| {
            let geometry = e.mem.u32(geometry_slot.addr());
            e.call(NI_OBJECT_NET_SET_NAME, &args![geometry, local]);
        });
        // `NiGeometry::m_spModelData` (+0xB8).
        let geometry = e.mem.u32(geometry_slot.addr());
        let data = e.mem.u32(geometry + 0xb8);
        e.call(NI_GEOMETRY_DATA_SET_CONSISTENCY, &args![data, 0u32]);

        let block = e.call(NI_NEW, &args![0xb0u32]).u32();
        let property = if block == 0 {
            0
        } else {
            e.call(TILE_SHADER_PROPERTY_CONSTRUCTOR, &args![block, 0u32])
                .u32()
        };
        e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x40), property]);
        let property = e.get(this, TileImage::spTSP);
        let geometry = e.mem.u32(geometry_slot.addr());
        e.call(NI_AV_OBJECT_ATTACH_PROPERTY, &args![geometry, property]);
        let shader = e.call(BS_SHADER_MANAGER_GET_SHADER, &args![0x20u32]).u32();
        // The geometry's shader (word +0xC0).
        let geometry = e.mem.u32(geometry_slot.addr());
        e.mem.set_u32(geometry + 0xc0, shader);

        let block = e.call(NI_NEW, &args![0xacu32]).u32();
        let node = if block == 0 {
            0
        } else {
            e.call(NI_NODE_CONSTRUCTOR, &args![block, 0u32]).u32()
        };
        e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x2c), node]);
        let model = e.get(tile, Tile::spModel);
        let geometry = e.mem.u32(geometry_slot.addr());
        e.vcall(model.addr(), NODE_ATTACH_CHILD_SLOT, &args![geometry, 1u32]);
        if parent == 0 {
            parent = e.call(INTERFACE_GET_INTERFACE_ROOT, &args![]).u32();
        }
        let model = e.get(tile, Tile::spModel);
        e.vcall(parent, NODE_ATTACH_CHILD_SLOT, &args![model, 1u32]);

        // `Tile::xName`'s text (`BSStringT` +0).
        let name = e.mem.u32(this.addr() + 0x20);
        let handle = fixed_string(e, name);
        release_after_use(e, handle, |e, local| {
            let model = e.get(tile, Tile::spModel);
            e.call(NI_OBJECT_NET_SET_NAME, &args![model, local]);
        });
        let geometry = e.mem.u32(geometry_slot.addr());
        e.call(INTERFACE_ATTACH_DEFAULT_ALPHA_PROPERTY, &args![geometry]);
        let model = e.get(tile, Tile::spModel);
        e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![model]);
        e.call(INTERFACE_SET_NEED_TO_UPDATE, &args![1u32]);

        let block = e.call(NI_NEW, &args![0x14u32]).u32();
        let extra = if block == 0 {
            0
        } else {
            let model = e.get(tile, Tile::spModel);
            e.call(TILE_EXTRA_CONSTRUCTOR, &args![block, this, model])
                .u32()
        };
        let model = e.get(tile, Tile::spModel);
        e.call(NI_OBJECT_NET_ADD_EXTRA_DATA, &args![model, extra]);
        let depth = e.call(TILE_GET_FLOAT, &args![this, TRAIT_DEPTH]).f32();
        e.call(
            TILE_FINAL_POST_PARSE,
            &args![this, TRAIT_DEPTH, depth, 0u32],
        );
        if e.call(TILE_IS_TRUE, &args![this, TRAIT_CLIPS]).bool() {
            e.call(TILE_ADD_NEEDS_UPDATE, &args![this, 0x100u32]);
        }
        e.call(TILE_ADD_NEEDS_UPDATE, &args![this, 0x3du32]);
        let model = e.get(tile, Tile::spModel);
        let geometry = e.mem.u32(geometry_slot.addr());
        release_object(e, geometry);
        model
    })
}

// Translated from 00a20580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileImage::PostParse` (Xbox PDB): queues the position update (1) for x
/// or y unless it is pending, the geometry update (`0x10`) for width or
/// height unless pending, and the geometry update for the file name when
/// the tile has a texture-atlas trait. Returns 0; the value and the third
/// word are not read.
// The exe's last two branches queue the same update.
#[allow(clippy::if_same_then_else)]
pub fn tile_image_post_parse(
    e: &mut Engine,
    this: Ptr<TileImage>,
    trait_id: i32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    let flags = e.get(this.cast::<Tile>(), Tile::uiFlags);
    if (trait_id == TRAIT_X || trait_id == TRAIT_Y) && flags & 1 == 0 {
        e.call(TILE_ADD_NEEDS_UPDATE, &args![this, 1u32]);
    } else if (trait_id == TRAIT_WIDTH || trait_id == TRAIT_HEIGHT) && flags & 0x10 == 0 {
        e.call(TILE_ADD_NEEDS_UPDATE, &args![this, 0x10u32]);
    } else if trait_id == TRAIT_FILENAME
        && e.call(TILE_GET_VALUE, &args![this, TRAIT_TEX_ATLAS]).u32() != 0
    {
        e.call(TILE_ADD_NEEDS_UPDATE, &args![this, 0x10u32]);
    }
    0
}

// Translated from 00a20610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileImage::SetSourceTexture` (Xbox PDB): empties the file name trait
/// (flag word 1), takes `texture` and queues the geometry and texture
/// updates (`0x30`).
pub fn tile_image_set_source_texture(e: &mut Engine, this: Ptr<TileImage>, texture: Ptr) {
    e.call(
        TILE_SET_STRING,
        &args![this, TRAIT_FILENAME, EMPTY_STRING, 1u32],
    );
    e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x3c), texture]);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![this, 0x30u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a1_f6e0,
            tile_image_init(Ptr<TileImage>, Ptr<Tile>, Ptr, u32)
        ),
        entry!(0x00a1_f7d0, fn_00a1f7d0(Ptr, Ptr, u32, Ptr)),
        entry!(
            0x00a1_fa20,
            tile_image_add_to_tes_textures(Ptr, Ptr, Ptr, f32, Ptr)
        ),
        entry!(0x00a1_fd50, tile_image_make_node(Ptr<TileImage>) -> Ptr),
        entry!(
            0x00a2_0580,
            tile_image_post_parse(Ptr<TileImage>, i32, u32, u32) -> u32
        ),
        entry!(
            0x00a2_0610,
            tile_image_set_source_texture(Ptr<TileImage>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ATTACH: u32 = 0x0000_1dc0;
    const DELETE_THIS: u32 = 0x0000_1d04;
    const NODE_VTABLE: u32 = 0x0003_0000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// `_strcpy_s` / `_strcat_s` doubles that really copy, and a reference
    /// count decrement that really decrements.
    fn strings(e: &mut Engine) {
        e.register(STRCPY_S, |e, a| {
            let text = if a[2] == 0 { vec![] } else { e.mem.cstr(a[2]) };
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
    }

    /// A reference-counted object with `count` references whose
    /// `DeleteThis` is [`DELETE_THIS`].
    fn object(e: &mut Engine, count: u32) -> u32 {
        e.put_vtable(0x0003_1000, &[0, DELETE_THIS]);
        quiet(e, &[DELETE_THIS]);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0003_1000);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let address = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(address, s);
        address
    }

    #[test]
    fn tile_image_init_sets_the_default_traits() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                TILE_INIT,
                TILE_SET_FLOAT,
                TILE_SET_NEEDS_UPDATE,
                NI_POINTER_ASSIGN,
            ],
        );
        e.map(FULL, 4);
        e.set_global(FULL, 255.0f32);
        let this = e.new_object::<TileImage>();
        e.set(this, TileImage::bIsScissor, true);
        e.call_log = Some(vec![]);
        e.call(0x00a1_f6e0, &args![this, 1u32, 2u32, 3u32]);
        let t = this.addr();
        let full = 255.0f32.to_bits();
        let one = 1.0f32.to_bits();
        assert_eq!(calls_to(&e, TILE_INIT), vec![vec![t, 1, 2, 3]]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![t, 0xff5, full, 1],
                vec![t, 0xfb2, full, 1],
                vec![t, 0xfb3, full, 1],
                vec![t, 0xfb4, full, 1],
                vec![t, 0xfa9, full, 1],
                vec![t, 0xfad, one, 1],
                vec![t, 0xfa3, one, 1],
            ]
        );
        assert_eq!(calls_to(&e, TILE_SET_NEEDS_UPDATE), vec![vec![t, 3]]);
        assert_eq!(e.get(this, TileImage::fScale), 1.0);
        assert_eq!(calls_to(&e, NI_POINTER_ASSIGN), vec![vec![t + 0x3c, 0]]);
        assert!(!e.get(this, TileImage::bIsScissor));
    }

    /// The path helper's doubles: the palette answers `texture` (0 for
    /// none), the finder answers from `finds`.
    fn path_engine(texture: u32, finds: Vec<u32>) -> Engine {
        let mut e = Engine::new();
        strings(&mut e);
        e.map(NI_FIXED_STRING_EMPTY, 4);
        e.map(FILE_FINDER, 4);
        e.set_global(FILE_FINDER, 0x0000_7000u32);
        e.map(MISSING_TEXTURE, 8);
        let missing = text(&mut e, b"Missing.dds");
        e.mem.set_u32(MISSING_TEXTURE + 4, missing);
        e.map(ONE, 8);
        e.set_global(ONE, 1.0f64);
        e.map(TEXTURES_FOLDER, 16);
        e.mem.set_cstr(TEXTURES_FOLDER, b"Data\\Textures\\");
        let handle = e.mem.alloc(0x10) + 8;
        e.register_double(NI_FIXED_STRING_CREATE, move |_, _| Ret {
            eax: handle,
            ..Ret::default()
        });
        e.register_double(TEXTURE_PALETTE_GET_TEXTURE, move |e, a| {
            e.mem.set_u32(a[1], texture);
            Ret::default()
        });
        let finds = RefCell::new(finds.into_iter());
        e.register_double(FILE_FINDER_FIND, move |_, _| Ret {
            eax: finds.borrow_mut().next().unwrap(),
            ..Ret::default()
        });
        quiet(&mut e, &[INTERFACE_GET_SCREEN_ASPECT_RATIO]);
        e
    }

    #[test]
    fn fn_00a1f7d0_builds_the_path_and_falls_back_to_the_missing_image() {
        let mut e = path_engine(0, vec![0]);
        let path = text(&mut e, b"menus\\a.dds");
        let scale = e.mem.alloc(4);
        let buffer = e.mem.alloc(0x104);
        e.call_log = Some(vec![]);
        e.call(0x00a1_f7d0, &args![path, scale, 0u32, buffer]);
        assert_eq!(e.mem.f32(scale), 1.0);
        assert_eq!(e.mem.cstr(buffer), b"Missing.dds");
        let looked = calls_to(&e, FILE_FINDER_FIND);
        assert_eq!(looked, vec![vec![0x7000, buffer, buffer, 6, u32::MAX]]);
        assert_eq!(calls_to(&e, INTERFACE_GET_SCREEN_ASPECT_RATIO).len(), 1);
        // The name's handle was released.
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn fn_00a1f7d0_keeps_a_found_path_and_releases_the_texture() {
        let mut e = path_engine(0, vec![1]);
        let path = text(&mut e, b"a.dds");
        let scale = e.mem.alloc(4);
        let buffer = e.mem.alloc(0x104);
        e.call(0x00a1_f7d0, &args![path, scale, 0u32, buffer]);
        assert_eq!(e.mem.cstr(buffer), b"Data\\Textures\\a.dds");

        let mut e = path_engine(0, vec![]);
        let texture = object(&mut e, 2);
        e.register_double(TEXTURE_PALETTE_GET_TEXTURE, move |e, a| {
            e.mem.set_u32(a[1], texture);
            Ret::default()
        });
        let path = text(&mut e, b"a.dds");
        let scale = e.mem.alloc(4);
        let buffer = e.mem.alloc(0x104);
        e.call_log = Some(vec![]);
        e.call(0x00a1_f7d0, &args![path, scale, 0u32, buffer]);
        assert!(calls_to(&e, FILE_FINDER_FIND).is_empty());
        assert_eq!(e.mem.u32(texture + 4), 1);
        assert!(calls_to(&e, DELETE_THIS).is_empty());
    }

    #[test]
    fn fn_00a1f7d0_ignores_empty_and_space_paths() {
        let mut e = path_engine(0, vec![]);
        let scale = e.mem.alloc(4);
        let buffer = e.mem.alloc(0x104);
        e.mem.set_f32(scale, 5.0);
        for path in [0, text(&mut e, b""), text(&mut e, b" x")] {
            e.call(0x00a1_f7d0, &args![path, scale, 0u32, buffer]);
        }
        assert_eq!(e.mem.f32(scale), 5.0);
        assert_eq!(e.mem.u8(buffer), 0);
    }

    fn textures_engine(palette: u32, found: u32, created: u32, loading: u32) -> Engine {
        let mut e = path_engine(palette, vec![1, found]);
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register_double(NI_SOURCE_TEXTURE_CREATE, move |_, _| Ret {
            eax: created,
            ..Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register_double(STRSTR, move |_, _| Ret {
            eax: loading,
            ..Ret::default()
        });
        quiet(&mut e, &[TEXTURE_PALETTE_SET_TEXTURE, ERROR]);
        e
    }

    #[test]
    fn tile_image_add_to_tes_textures_creates_and_registers_the_texture() {
        let mut e = textures_engine(0, 1, 0x0000_8800, 0);
        let path = text(&mut e, b"a.dds");
        let scale = e.mem.alloc(4);
        let slot = e.mem.alloc(4);
        let tags = Rc::new(RefCell::new(vec![]));
        let seen = tags.clone();
        e.register_double(NI_SOURCE_TEXTURE_CREATE, move |e, a| {
            let tls = e.tls();
            seen.borrow_mut()
                .push((e.mem.u32(tls + ALLOCATION_TAG_OFFSET), a[1], a[2], a[3]));
            Ret {
                eax: 0x8800,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00a1_fa20, &args![path, scale, slot, 2.0f32, 0u32]);
        assert_eq!(e.mem.u32(slot), 0x8800);
        assert_eq!(*tags.borrow(), vec![(0xd, TEXTURE_FORMAT_PREFS, 1, 0)]);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + ALLOCATION_TAG_OFFSET), 0);
        assert_eq!(
            calls_to(&e, TEXTURE_PALETTE_SET_TEXTURE),
            vec![vec![0x8800, 0]]
        );
        let finds = calls_to(&e, FILE_FINDER_FIND);
        assert_eq!(finds[1][2..], [0, 0, u32::MAX]);
        assert!(calls_to(&e, ERROR).is_empty());
    }

    #[test]
    fn tile_image_add_to_tes_textures_reports_and_skips_loading_screens() {
        let mut e = textures_engine(0, 1, 0, 0);
        let path = text(&mut e, b"a.dds");
        let (scale, slot) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        e.call(0x00a1_fa20, &args![path, scale, slot, 0u32, 0u32]);
        let errors = calls_to(&e, ERROR);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0][0], COULD_NOT_CREATE_TEXTURE);

        let mut e = textures_engine(0, 1, 0x8800, 1);
        let path = text(&mut e, b"loading\\load_a.dds");
        let (scale, slot) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        e.call(0x00a1_fa20, &args![path, scale, slot, 0u32, 0u32]);
        assert!(calls_to(&e, TEXTURE_PALETTE_SET_TEXTURE).is_empty());

        // Not found by the finder: nothing is created.
        let mut e = textures_engine(0, 0, 0x8800, 0);
        let path = text(&mut e, b"a.dds");
        let (scale, slot) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        e.call(0x00a1_fa20, &args![path, scale, slot, 0u32, 0u32]);
        assert!(calls_to(&e, NI_SOURCE_TEXTURE_CREATE).is_empty());
    }

    #[test]
    fn tile_image_add_to_tes_textures_takes_and_releases_a_source() {
        let mut e = textures_engine(0, 1, 0, 0);
        let source = object(&mut e, 1);
        e.register(NI_POINTER_ASSIGN_POINTER, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let slot = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00a1_fa20, &args![0u32, 0u32, slot, 0u32, source]);
        assert_eq!(e.mem.u32(slot), source);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![source]]);
        assert!(calls_to(&e, MEMSET).is_empty());
    }

    fn make_node_engine(clips: bool, parent: u32) -> (Engine, Ptr<TileImage>) {
        let mut e = Engine::new();
        e.map(NI_FIXED_STRING_EMPTY, 4);
        e.set_global(NI_FIXED_STRING_EMPTY, 0x0000_7770u32);
        e.map(MINUS_ONE, 4);
        e.set_global(MINUS_ONE, -1.0f32);
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(NI_ALLOC, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        for constructor in [
            BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR,
            NI_TRI_SHAPE_CONSTRUCTOR,
            TILE_SHADER_PROPERTY_CONSTRUCTOR,
            NI_NODE_CONSTRUCTOR,
            TILE_EXTRA_CONSTRUCTOR,
        ] {
            e.register(constructor, |e, a| {
                // Objects: the test node vtable, one reference, data at +0xB8.
                e.mem.set_u32(a[0], NODE_VTABLE);
                e.mem.set_u32(a[0] + 4, 1);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register_double(TILE_GET_PARENT_MODEL, move |_, _| Ret {
            eax: parent,
            ..Ret::default()
        });
        e.register(INTERFACE_GET_INTERFACE_ROOT, |_, _| Ret {
            eax: 0x0004_0000,
            ..Ret::default()
        });
        e.register_double(TILE_IS_TRUE, move |_, _| Ret {
            eax: clips as u32,
            ..Ret::default()
        });
        e.register(INTERFACE_GET_SCREEN_HEIGHT, |_, _| Ret {
            st0: 720.75,
            ..Ret::default()
        });
        e.register(INTERFACE_GET_SCREEN_WIDTH, |_, _| Ret {
            st0: 1280.5,
            ..Ret::default()
        });
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32,
            ..Ret::default()
        });
        e.register(BS_SHADER_MANAGER_GET_SHADER, |_, _| Ret {
            eax: 0x0000_5e00,
            ..Ret::default()
        });
        e.register(TILE_GET_FLOAT, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        e.register(NI_FIXED_STRING_CREATE, |_, _| Ret {
            eax: 0x9008,
            ..Ret::default()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            if a[0] == 0x9000 {
                return Ret::default();
            }
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        quiet(
            &mut e,
            &[
                ATTACH,
                DELETE_THIS,
                NI_OBJECT_NET_SET_NAME,
                NI_GEOMETRY_DATA_SET_CONSISTENCY,
                NI_AV_OBJECT_ATTACH_PROPERTY,
                INTERFACE_ATTACH_DEFAULT_ALPHA_PROPERTY,
                NI_AV_OBJECT_UPDATE_PROPERTIES,
                INTERFACE_SET_NEED_TO_UPDATE,
                NI_OBJECT_NET_ADD_EXTRA_DATA,
                TILE_FINAL_POST_PARSE,
                TILE_ADD_NEEDS_UPDATE,
            ],
        );
        let mut slots = vec![0u32; 0x38];
        slots[1] = DELETE_THIS;
        slots[(NODE_ATTACH_CHILD_SLOT / 4) as usize] = ATTACH;
        e.put_vtable(NODE_VTABLE, &slots);
        for parent in [0x0004_0000u32, 0x0004_1000] {
            e.map(parent, 4);
            e.mem.set_u32(parent, NODE_VTABLE);
        }
        let this = e.new_object::<TileImage>();
        (e, this)
    }

    #[test]
    fn tile_image_make_node_builds_a_scissor_quad() {
        let (mut e, this) = make_node_engine(true, 0x0004_1000);
        e.call_log = Some(vec![]);
        let model = e.call(0x00a1_fd50, &args![this]).u32();
        let scissor = calls_to(&e, BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR);
        assert_eq!(scissor.len(), 1);
        let s = &scissor[0];
        assert_eq!(s[1], 4);
        assert_eq!(s[3..5], [0, 0]);
        assert_eq!(s[6..14], [1, 0, 2, s[9], 0, 0, 1280, 720]);
        let (vertices, uvs, triangles) = (s[2], s[5], s[9]);
        let floats =
            |e: &Engine, at: u32, n: u32| (0..n).map(|i| e.mem.f32(at + 4 * i)).collect::<Vec<_>>();
        assert_eq!(
            floats(&e, vertices, 12),
            vec![0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, -1.0]
        );
        assert_eq!(
            floats(&e, uvs, 8),
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]
        );
        let indices: Vec<u16> = (0..6).map(|i| e.mem.u16(triangles + 2 * i)).collect();
        assert_eq!(indices, vec![0, 1, 2, 2, 1, 3]);
        assert!(e.get(this, TileImage::bIsScissor));
        let geometry = s[0];
        assert_eq!(e.mem.u32(geometry + 0xc0), 0x5e00);
        assert!(!e.get(this, TileImage::spTSP).is_null());
        assert_eq!(
            calls_to(&e, ATTACH),
            vec![vec![model, geometry, 1], vec![0x0004_1000, model, 1]]
        );
        assert_eq!(
            calls_to(&e, TILE_ADD_NEEDS_UPDATE),
            vec![vec![this.addr(), 0x100], vec![this.addr(), 0x3d]]
        );
        assert_eq!(
            calls_to(&e, TILE_FINAL_POST_PARSE),
            vec![vec![this.addr(), 0xfad, 3.0f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, INTERFACE_ATTACH_DEFAULT_ALPHA_PROPERTY),
            vec![vec![geometry]]
        );
        // The local's reference on the geometry is dropped: 1 -> 0.
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![geometry]]);
        assert_eq!(
            calls_to(&e, NI_FIXED_STRING_CREATE),
            vec![vec![TILE_IMAGE_NAME]]
        );
    }

    #[test]
    fn tile_image_make_node_builds_a_plain_quad_under_the_root() {
        let (mut e, this) = make_node_engine(false, 0);
        e.call_log = Some(vec![]);
        let model = e.call(0x00a1_fd50, &args![this]).u32();
        assert!(calls_to(&e, BS_SCISSOR_TRI_SHAPE_CONSTRUCTOR).is_empty());
        assert!(calls_to(&e, INTERFACE_GET_SCREEN_WIDTH).is_empty());
        let shape = calls_to(&e, NI_TRI_SHAPE_CONSTRUCTOR);
        assert_eq!(shape.len(), 1);
        assert_eq!(shape[0].len(), 10);
        assert!(!e.get(this, TileImage::bIsScissor));
        assert_eq!(calls_to(&e, ATTACH)[1], vec![0x0004_0000, model, 1]);
        assert_eq!(
            calls_to(&e, TILE_ADD_NEEDS_UPDATE),
            vec![vec![this.addr(), 0x3d]]
        );
    }

    #[test]
    fn tile_image_post_parse_queues_the_updates() {
        let mut e = Engine::new();
        quiet(&mut e, &[TILE_ADD_NEEDS_UPDATE]);
        e.register(TILE_GET_VALUE, |e, _| Ret {
            eax: e.mem.u32(0x0000_6000),
            ..Ret::default()
        });
        e.map(0x0000_6000, 4);
        let this = e.new_object::<TileImage>();
        let run = |e: &mut Engine, trait_id: u32, flags: u32| {
            e.set(this.cast::<Tile>(), Tile::uiFlags, flags);
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x00a2_0580, &args![this, trait_id, 0u32, 0u32])
                    .u32(),
                0
            );
            calls_to(e, TILE_ADD_NEEDS_UPDATE)
        };
        let t = this.addr();
        assert_eq!(run(&mut e, 0xfa1, 0), vec![vec![t, 1]]);
        assert_eq!(run(&mut e, 0xfa2, 1), Vec::<Vec<u32>>::new());
        assert_eq!(run(&mut e, 0xfb1, 0), vec![vec![t, 0x10]]);
        assert_eq!(run(&mut e, 0xfb0, 0x10), Vec::<Vec<u32>>::new());
        assert_eq!(run(&mut e, 0xfcc, 0), Vec::<Vec<u32>>::new());
        e.mem.set_u32(0x0000_6000, 0x1234);
        assert_eq!(run(&mut e, 0xfcc, 0x10), vec![vec![t, 0x10]]);
        assert_eq!(run(&mut e, 0xfb2, 0), Vec::<Vec<u32>>::new());
    }

    #[test]
    fn tile_image_set_source_texture_clears_the_file_and_takes_the_texture() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[TILE_SET_STRING, NI_POINTER_ASSIGN, TILE_ADD_NEEDS_UPDATE],
        );
        e.call_log = Some(vec![]);
        e.call(0x00a2_0610, &args![0x5000u32, 0x8800u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x5000, 0xfcc, EMPTY_STRING, 1]]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_ASSIGN),
            vec![vec![0x5000 + 0x3c, 0x8800]]
        );
        assert_eq!(
            calls_to(&e, TILE_ADD_NEEDS_UPDATE),
            vec![vec![0x5000, 0x30]]
        );
    }
}
