//! `NiMain/nisourcecubemap.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSourceCubeMap::ms_RTTI` (the class's `NiRTTI` record).
const NI_SOURCE_CUBE_MAP_RTTI: u32 = 0x011f_4454;

// Translated from 00a60cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_source_cube_map_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SOURCE_CUBE_MAP_RTTI)
}

/// `00a60600`: the jump target of `00a60870`.
const JUMP_TARGET_00A60870: u32 = 0x00a6_0600;

// Translated from 00a60870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::SaveBinary` (Xbox PDB): a 5-byte jump to `00a60600` (engine map: `NiSourceTexture::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiSourceCubeMap@@UAAXAAVNiStream@@@Z`.
pub fn ni_source_cube_map_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A60870, &args![this, stream]);
}

/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiSourceTexture::NiSourceTexture` (engine map, `00a5fc10`, `thiscall`, no words).
const NI_SOURCE_TEXTURE_CONSTRUCT: u32 = 0x00a5_fc10;
/// `NiSourceCubeMap`'s vtable (`0109babc`, named by its RTTI locator).
const NI_SOURCE_CUBE_MAP_VTABLE: u32 = 0x0109_babc;
/// `NiSourceCubeMap::LoadFromFiles` (engine map, `00a60cc0`, `thiscall`, seven words: the
/// six file names and the search-path object).
const NI_SOURCE_CUBE_MAP_LOAD_FROM_FILES: u32 = 0x00a6_0cc0;
/// The set-up `00aa24d0` and clean-up `00aa24f0` of the 0x210-byte stack object passed to
/// `LoadFromFiles` (both `thiscall`, no words; they set the vtable whose RTTI locator names
/// `NiSearchPath`).
const SEARCH_PATH_SET_UP_00AA24D0: u32 = 0x00aa_24d0;
const SEARCH_PATH_CLEAN_UP_00AA24F0: u32 = 0x00aa_24f0;
const SEARCH_PATH_BYTES: u32 = 0x210;

// Translated from 00a61120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `NiMain/nisourcecubemap.cpp`): makes an `NiSourceCubeMap`
/// from six files. `cdecl`, nine words: the six file names, the renderer, an unread word and
/// the address of three words of format preferences. Allocates 0x4c bytes (`00aa13e0`); when
/// that succeeds runs `NiSourceTexture::NiSourceTexture` (`00a5fc10`), sets the vtable
/// `0109babc` and clears +0x48 (a null allocation is used as it is). Copies the three
/// preference words to +0x18..+0x20, sets up a stack `NiSearchPath` (`00aa24d0`) and runs
/// `NiSourceCubeMap::LoadFromFiles` (`00a60cc0`) with the six names and it; clears +0x48 when
/// it is set. With a renderer and a non-zero +0x3c, the renderer's vtable slot +0x144 is
/// called with the map; when there is no renderer, or that answers true (`AL`), the map is
/// answered. Otherwise (no word at +0x3c, or the renderer answered false) the map is deleted
/// (slot 0 with 1) and 0 answered. The search path is cleaned up (`00aa24f0`) on both paths.
// Translation incomplete: 00a61120 - the /GS cookie check (00ec408c) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a61120(
    e: &mut Engine,
    name_0: Ptr,
    name_1: Ptr,
    name_2: Ptr,
    name_3: Ptr,
    name_4: Ptr,
    name_5: Ptr,
    renderer: Ptr,
    _unused_7: u32,
    preferences: Ptr,
) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x4cu32]).u32();
    let map = if block != 0 {
        e.call(NI_SOURCE_TEXTURE_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, NI_SOURCE_CUBE_MAP_VTABLE);
        e.mem.set_u32(block.wrapping_add(0x48), 0);
        block
    } else {
        0
    };
    for word in 0..3u32 {
        let value = e.mem.u32(preferences.addr().wrapping_add(4 * word));
        e.mem.set_u32(map.wrapping_add(0x18 + 4 * word), value);
    }
    e.with_stack(SEARCH_PATH_BYTES, |e, search_path| {
        e.call(SEARCH_PATH_SET_UP_00AA24D0, &args![search_path]);
        e.call(
            NI_SOURCE_CUBE_MAP_LOAD_FROM_FILES,
            &args![
                map,
                name_0,
                name_1,
                name_2,
                name_3,
                name_4,
                name_5,
                search_path
            ],
        );
        if e.mem.u32(map.wrapping_add(0x48)) != 0 {
            e.mem.set_u32(map.wrapping_add(0x48), 0);
        }
        let keep = renderer.is_null()
            || (e.mem.u32(map.wrapping_add(0x3c)) != 0
                && e.vcall(renderer.addr(), 0x144, &args![map]).bool());
        if keep {
            e.call(SEARCH_PATH_CLEAN_UP_00AA24F0, &args![search_path]);
            Ptr::new(map)
        } else {
            e.vcall(map, 0, &args![1u32]);
            e.call(SEARCH_PATH_CLEAN_UP_00AA24F0, &args![search_path]);
            Ptr::NULL
        }
    })
}

/// The word an empty `NiFixedString` holds (`0109b220`); names equal to it are not counted.
const NULL_NAME_0109B220: u32 = 0x0109_b220;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiStandardizeFilePath` (`00a57500`, `cdecl`, the name) and
/// `NiImageConverter::ConvertFilenameToPlatformSpecific_ov2` (`00a76960`, `cdecl`: the name,
/// the converted name) (engine map).
const STANDARDIZE_FILE_PATH: u32 = 0x00a5_7500;
const CONVERT_FILE_NAME: u32 = 0x00a7_6960;
/// The byte that makes the texture load at once (`011a9594`).
const LOAD_AT_ONCE_011A9594: u32 = 0x011a_9594;

// Translated from 00a60f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `NiMain/nisourcecubemap.cpp`): makes an `NiSourceCubeMap`
/// from one file (the cube-map path of `BSShaderManager::GetTexture`). `cdecl`: the address of
/// the name (an `NiFixedString`), the renderer and the address of three words of format
/// preferences. Allocates 0x4c bytes (`00aa13e0`); when that succeeds runs
/// `NiSourceTexture::NiSourceTexture` (`00a5fc10`), sets the vtable `0109babc` and clears +0x48
/// (a null allocation is used as it is). Copies the preferences to +0x18..+0x20, assigns the
/// name to the `NiFixedString` at +0x30 (the new one's count at -8 incremented and the old
/// one's decremented, each unless it is the empty word at `0109b220`), standardizes it
/// (`00a57500`) and converts it into +0x34 (`00a76960`), clears +0x48 when it is set. When the
/// byte at `011a9594` is set: the map's vtable slot +0xa4, then with a renderer its slot +0x144
/// with the map; when that answers a clear byte the map is deleted (slot 0 with 1) and 0
/// answered. Otherwise answers the map.
pub fn fn_00a60f60(e: &mut Engine, name: Ptr, renderer: Ptr, preferences: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x4cu32]).u32();
    let map = if block != 0 {
        e.call(NI_SOURCE_TEXTURE_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, NI_SOURCE_CUBE_MAP_VTABLE);
        e.mem.set_u32(block.wrapping_add(0x48), 0);
        block
    } else {
        0
    };
    for word in 0..3u32 {
        let value = e.mem.u32(preferences.addr().wrapping_add(4 * word));
        e.mem.set_u32(map.wrapping_add(0x18 + 4 * word), value);
    }
    let file = map.wrapping_add(0x30);
    let new_name = e.mem.u32(name.addr());
    if e.mem.u32(file) != new_name {
        if new_name != e.global::<u32>(NULL_NAME_0109B220) {
            e.call(
                IMPORT_INTERLOCKED_INCREMENT,
                &args![new_name.wrapping_sub(8)],
            );
        }
        let old_name = e.mem.u32(file);
        if old_name != e.global::<u32>(NULL_NAME_0109B220) {
            e.call(
                IMPORT_INTERLOCKED_DECREMENT,
                &args![old_name.wrapping_sub(8)],
            );
        }
        e.mem.set_u32(file, new_name);
    }
    e.call(STANDARDIZE_FILE_PATH, &args![file]);
    e.call(CONVERT_FILE_NAME, &args![file, map.wrapping_add(0x34)]);
    if e.mem.u32(map.wrapping_add(0x48)) != 0 {
        e.mem.set_u32(map.wrapping_add(0x48), 0);
    }
    if e.global::<u8>(LOAD_AT_ONCE_011A9594) != 0 {
        e.vcall(map, 0xa4, &args![]);
        if !renderer.is_null() && !e.vcall(renderer.addr(), 0x144, &args![map]).bool() {
            e.vcall(map, 0, &args![1u32]);
            return Ptr::NULL;
        }
    }
    Ptr::new(map)
}
/// Callees of `LoadFromFiles` (engine map names): `NiImageConverter::GetImageConverter`
/// (`00a76640`, no arguments), the allocation `00aa1070` (`cdecl`, size), `_strcpy_s`
/// (`00ec65a6`, `cdecl`), `NiPath::Standardize` (`00aa2a00`, `cdecl`),
/// `NiImageConverter::ConvertFilenameToPlatformSpecific` (`00a768b0`, `cdecl`; answers a new
/// name), `NiSearchPath::SetFilePath` (`00aa2540`, `thiscall`, one word), `NiFile::Access`
/// (`00aa1a30`, `cdecl`: path, mode; answers a byte), the free `00aa10f0` (`cdecl`) and
/// `NiSourceCubeMap::LoadFromPixelData` (`00a60940`, `thiscall`, six words).
const GET_IMAGE_CONVERTER: u32 = 0x00a7_6640;
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const STRCPY_S: u32 = 0x00ec_65a6;
const PATH_STANDARDIZE: u32 = 0x00aa_2a00;
const CONVERT_FILE_NAME_TO_PLATFORM: u32 = 0x00a7_68b0;
const SEARCH_PATH_SET_FILE_PATH: u32 = 0x00aa_2540;
const FILE_ACCESS: u32 = 0x00aa_1a30;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const LOAD_FROM_PIXEL_DATA: u32 = 0x00a6_0940;

/// The six face pointers of `LoadFromFiles` released in reverse order (the `NiPointer`
/// destructors), without clearing them.
fn release_faces(e: &mut Engine, faces: u32) {
    for index in (0..6u32).rev() {
        let face = e.mem.u32(faces + 4 * index);
        if face != 0 {
            let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![face + 4]).u32();
            if left == 0 {
                e.vcall(face, 4, &args![]);
            }
        }
    }
}

// Translated from 00a60cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::LoadFromFiles` (Xbox PDB), `thiscall` with seven stack words: the six
/// face file names and the search path. With six face `NiPointer`s on the stack (cleared) and
/// the image converter (`GetImageConverter`), for each face in order: copies its name into a
/// new block (`00aa1070` of the length + 1, `_strcpy_s`), standardizes it
/// (`NiPath::Standardize`), converts it (`ConvertFilenameToPlatformSpecific`), gives that to
/// the search path (`SetFilePath`), resets the search path (its vtable slot +4) and asks it
/// for paths (slot +8 with a 0x104-byte buffer and 0x104) until one exists
/// (`NiFile::Access(path, 0)`): then the converter's slot +0xc reads it (with 0, 0) into the
/// face's pointer, and the converted name and the copy are freed (`00aa10f0`). A face left
/// null ends the load (the six pointers released, nothing freed); after the sixth,
/// `LoadFromPixelData` gets the six faces and the pointers are released.
// Translation incomplete: 00a60cc0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ni_source_cube_map_load_from_files(
    e: &mut Engine,
    this: Ptr,
    name_0: Ptr,
    name_1: Ptr,
    name_2: Ptr,
    name_3: Ptr,
    name_4: Ptr,
    name_5: Ptr,
    search_path: Ptr,
) {
    let names = [name_0, name_1, name_2, name_3, name_4, name_5];
    e.with_stack(0x148, |e, frame| {
        let frame = frame.addr();
        let faces = frame + 4;
        let buffer = frame + 0x40;
        let search = search_path.addr();
        let converter = e.call(GET_IMAGE_CONVERTER, &args![]).u32();
        for index in 0..6u32 {
            let name = names[index as usize].addr();
            let size = e.mem.cstr(name).len() as u32 + 1;
            let copy = e.call(ALLOCATE_00AA1070, &args![size]).u32();
            e.call(STRCPY_S, &args![copy, size, name]);
            e.call(PATH_STANDARDIZE, &args![copy]);
            let converted = e.call(CONVERT_FILE_NAME_TO_PLATFORM, &args![copy]).u32();
            e.call(SEARCH_PATH_SET_FILE_PATH, &args![search, converted]);
            e.vcall(search, 4, &args![]);
            let mut more = e.vcall(search, 8, &args![buffer, 0x104u32]).bool();
            while more {
                if e.call(FILE_ACCESS, &args![buffer, 0u32]).bool() {
                    let texture = e.vcall(converter, 0xc, &args![buffer, 0u32, 0u32]).u32();
                    let slot = faces + 4 * index;
                    let old = e.mem.u32(slot);
                    if old != texture {
                        if old != 0 {
                            let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old + 4]).u32();
                            if left == 0 {
                                e.vcall(old, 4, &args![]);
                            }
                        }
                        e.mem.set_u32(slot, texture);
                        if texture != 0 {
                            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![texture + 4]);
                        }
                    }
                    e.call(FREE_00AA10F0, &args![converted]);
                    e.call(FREE_00AA10F0, &args![copy]);
                    break;
                }
                more = e.vcall(search, 8, &args![buffer, 0x104u32]).bool();
            }
            if e.mem.u32(faces + 4 * index) == 0 {
                release_faces(e, faces);
                return;
            }
        }
        let face = |e: &Engine, index: u32| e.mem.u32(faces + 4 * index);
        let words = [
            this.addr(),
            face(e, 0),
            face(e, 1),
            face(e, 2),
            face(e, 3),
            face(e, 4),
            face(e, 5),
        ];
        e.call(LOAD_FROM_PIXEL_DATA, &words);
        release_faces(e, faces);
    });
}

use crate::units::nimain::nipixeldata::NiPixelData;

/// `NiPixelData::NiPixelData` (`00a7c190`, the engine map's `_ov2`; `thiscall`, `RET 0x14`:
/// width, height, the address of a pixel format, mipmap levels, faces) and `_memcpy`
/// (`00ec44d0`, `cdecl`: destination, source, size).
const NI_PIXEL_DATA_CONSTRUCT_00A7C190: u32 = 0x00a7_c190;
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiSourceTexture::m_spSrcPixelData` (Xbox PDB, +0x3c; the same on PC).
const SOURCE_PIXEL_DATA: u32 = 0x3c;

/// `NiPointer<T>::operator=` expanded inline on the pointer at `slot`: nothing when `object`
/// is already there; else the old object's reference given up (deleted through its vtable
/// slot +4 at zero), `object` stored and, when not null, a reference taken on it.
fn assign_ni_pointer(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

// Translated from 00a60940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::LoadFromPixelData` (Xbox PDB), `thiscall`, six stack words: the six
/// faces' `NiPixelData`. Makes the cube map's pixel data (0x74 bytes from `00aa13e0`;
/// `NiPixelData::NiPixelData` `00a7c190` with the first face's width and height, the first
/// word of each of its arrays, the address of its pixel format at +8, its mipmap level count
/// and 6 faces; null when the allocation fails) and assigns it to `m_spSrcPixelData` (+0x3c,
/// the `NiPointer` assignment inline). Then, with a reference on each face held in six stack
/// `NiPointer`s (cleared, then assigned in order), copies for each face f and each of its
/// mipmap levels m (none when its level count is 0; the count read again after each level)
/// the level's bytes (offset m + 1 minus offset m of the face) from the face's pixels plus
/// offset m to the cube map's pixels (read again for each level) plus its offset m plus f
/// times its offset at its own level count (one face's size) with `_memcpy`. Finally the six
/// stack pointers are released, the last first. A null face is read through as the exe does.
pub fn ni_source_cube_map_load_from_pixel_data(
    e: &mut Engine,
    this: Ptr,
    face_0: Ptr<NiPixelData>,
    face_1: Ptr<NiPixelData>,
    face_2: Ptr<NiPixelData>,
    face_3: Ptr<NiPixelData>,
    face_4: Ptr<NiPixelData>,
    face_5: Ptr<NiPixelData>,
) {
    let base = this.addr();
    let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
    let data = if block != 0 {
        let levels = e.get(face_0, NiPixelData::m_uiMipmapLevels);
        let heights = e.get(face_0, NiPixelData::m_puiHeight);
        let height = e.mem.u32(heights);
        let widths = e.get(face_0, NiPixelData::m_puiWidth);
        let width = e.mem.u32(widths);
        let format = face_0.addr().wrapping_add(8);
        e.call(
            NI_PIXEL_DATA_CONSTRUCT_00A7C190,
            &args![block, width, height, format, levels, 6u32],
        )
        .u32()
    } else {
        0
    };
    assign_ni_pointer(e, base.wrapping_add(SOURCE_PIXEL_DATA), data);
    let faces = [face_0, face_1, face_2, face_3, face_4, face_5];
    e.with_stack(0x18, |e, held| {
        let held = held.addr();
        for index in 0..6u32 {
            e.mem.set_u32(held + 4 * index, 0);
        }
        for (index, face) in faces.iter().enumerate() {
            assign_ni_pointer(e, held + 4 * index as u32, face.addr());
        }
        for index in 0..6u32 {
            let source = Ptr::<NiPixelData>::new(e.mem.u32(held + 4 * index));
            if e.get(source, NiPixelData::m_uiMipmapLevels) == 0 {
                continue;
            }
            let mut level = 0u32;
            loop {
                let target =
                    Ptr::<NiPixelData>::new(e.mem.u32(base.wrapping_add(SOURCE_PIXEL_DATA)));
                let offsets = e.get(source, NiPixelData::m_puiOffsetInBytes);
                let next = e.mem.u32(offsets.wrapping_add(4 * level + 4));
                let start = e.mem.u32(offsets.wrapping_add(4 * level));
                let size = next.wrapping_sub(start);
                let from = start.wrapping_add(e.get(source, NiPixelData::m_pucPixels));
                let target_offsets = e.get(target, NiPixelData::m_puiOffsetInBytes);
                let target_levels = e.get(target, NiPixelData::m_uiMipmapLevels);
                let face_size = e.mem.u32(target_offsets.wrapping_add(4 * target_levels));
                let to = face_size
                    .wrapping_mul(index)
                    .wrapping_add(e.mem.u32(target_offsets.wrapping_add(4 * level)))
                    .wrapping_add(e.get(target, NiPixelData::m_pucPixels));
                e.call(MEMCPY, &args![to, from, size]);
                level += 1;
                if level >= e.get(source, NiPixelData::m_uiMipmapLevels) {
                    break;
                }
            }
        }
        for index in (0..6u32).rev() {
            let face = e.mem.u32(held + 4 * index);
            if face != 0 {
                let left = e
                    .call(IMPORT_INTERLOCKED_DECREMENT, &args![face.wrapping_add(4)])
                    .u32();
                if left == 0 {
                    e.vcall(face, 4, &args![]);
                }
            }
        }
    });
}

/// The `NiPointer` release expanded inline on the pointer at `slot`: when an
/// object is there, `InterlockedDecrement` of its count (`+4`), its vtable
/// slot `+4` (delete) at zero, and the pointer cleared.
fn release_ni_pointer(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(object, 4, &args![]);
        }
        e.mem.set_u32(slot, 0);
    }
}

// Translated from 00a60910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::DestroyAppPixelData` (engine map name), `thiscall`, no
/// stack words: releases the source pixel data (`+0x3c`) and clears it.
pub fn ni_source_cube_map_destroy_app_pixel_data(e: &mut Engine, this: Ptr) {
    release_ni_pointer(e, this.addr() + SOURCE_PIXEL_DATA);
}

/// `NiSourceTexture::~NiSourceTexture`'s body (`00a5fca0`, `thiscall`, no
/// stack words) and the sized delete (`00aa1460`, `cdecl`: block, size).
const NI_SOURCE_TEXTURE_DESTRUCT: u32 = 0x00a5_fca0;
const SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00a61250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::_scalar_deleting_destructor_` (engine map name),
/// `thiscall`, `RET 4` (the flags): stores the vtable `0109babc`, releases the
/// source pixel data (`+0x3c`), runs `NiSourceTexture`'s destructor body
/// (`00a5fca0`) and, when bit 0 of the flags is set, frees the 0x4c bytes
/// (`00aa1460`). Answers `this`.
pub fn ni_source_cube_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_SOURCE_CUBE_MAP_VTABLE);
    release_ni_pointer(e, this.addr() + SOURCE_PIXEL_DATA);
    e.call(NI_SOURCE_TEXTURE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x4cu32]);
    }
    this
}

/// `00a5f860` (`thiscall` on an `NiPointer`, `RET 4`: the object): assigns it.
const NI_POINTER_ASSIGN_00A5F860: u32 = 0x00a5_f860;

// Translated from 00a608c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::LoadPixelDataFromFile` (engine map name), `thiscall`, no
/// stack words. Only without source pixel data (`+0x3c`) and with a platform
/// file name (`+0x34`, not the empty name `0109b220`): asks the image
/// converter (`GetImageConverter`) through its vtable slot `+0xc` with the
/// name, 0 and the word at `+0x44`, assigns the answer to `+0x3c`
/// (`00a5f860`) and clears `+0x44`.
pub fn ni_source_cube_map_load_pixel_data_from_file(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    if e.mem.u32(t + SOURCE_PIXEL_DATA) != 0 {
        return;
    }
    if e.mem.u32(t + 0x34) == e.global::<u32>(NULL_NAME_0109B220) {
        return;
    }
    let converter = e.call(GET_IMAGE_CONVERTER, &args![]).u32();
    let extra = e.mem.u32(t + 0x44);
    let name = e.mem.u32(t + 0x34);
    let data = e.vcall(converter, 0xc, &args![name, 0u32, extra]).u32();
    e.call(
        NI_POINTER_ASSIGN_00A5F860,
        &args![t + SOURCE_PIXEL_DATA, data],
    );
    e.mem.set_u32(t + 0x44, 0);
}

/// `00a5b690` (`cdecl`, the text): a new `NiFixedString` handle for it (with
/// one reference).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;

// Translated from 00a5fe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::Create` (engine map name; the object it makes is an
/// `NiSourceTexture` of 0x48 bytes), `cdecl`, four words: a word for `+0x44`,
/// a file name (text), the address of three words of format preferences and a
/// byte for `+0x41`. Allocates 0x48 bytes (`00aa13e0`) and runs
/// `NiSourceTexture::NiSourceTexture` on them (a null allocation is used as
/// it is). Copies the preferences to `+0x18..+0x20`, stores the byte and the
/// word. With a name: makes it an `NiFixedString` (`00a5b690`), assigns it to
/// `+0x30` (new one counted, old one released, each unless it is the empty
/// name `0109b220`), standardizes it (`00a57500`), converts it into `+0x34`
/// (`00a76960`) and releases the temporary handle. When the byte at
/// `011a9594` is set and the texture's vtable slot `+0xac` answers false, the
/// texture is deleted (slot 0 with 1) and 0 answered. Answers the texture.
pub fn ni_source_cube_map_create(
    e: &mut Engine,
    extra: u32,
    name: Ptr,
    preferences: Ptr,
    flag: u8,
) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x48u32]).u32();
    let texture = if block != 0 {
        e.call(NI_SOURCE_TEXTURE_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    for word in 0..3u32 {
        let value = e.mem.u32(preferences.addr().wrapping_add(4 * word));
        e.mem.set_u32(texture.wrapping_add(0x18 + 4 * word), value);
    }
    e.mem.set_u8(texture.wrapping_add(0x41), flag);
    e.mem.set_u32(texture.wrapping_add(0x44), extra);
    if !name.is_null() {
        let handle = e.call(FIXED_STRING_CREATE, &args![name]).u32();
        let file = texture.wrapping_add(0x30);
        if e.mem.u32(file) != handle {
            if handle != e.global::<u32>(NULL_NAME_0109B220) {
                e.call(IMPORT_INTERLOCKED_INCREMENT, &args![handle.wrapping_sub(8)]);
            }
            let old = e.mem.u32(file);
            if old != e.global::<u32>(NULL_NAME_0109B220) {
                e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
            }
            e.mem.set_u32(file, handle);
        }
        e.call(STANDARDIZE_FILE_PATH, &args![file]);
        e.call(CONVERT_FILE_NAME, &args![file, texture.wrapping_add(0x34)]);
        if handle != e.global::<u32>(NULL_NAME_0109B220) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    }
    if e.global::<u8>(LOAD_AT_ONCE_011A9594) != 0 && !e.vcall(texture, 0xac, &args![]).bool() {
        e.vcall(texture, 0, &args![1u32]);
        return Ptr::NULL;
    }
    Ptr::new(texture)
}

// Translated from 00a61040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nisourcecubemap.cpp`): makes an
/// `NiSourceCubeMap` from a name, like `00a60f60` with one more word.
/// `cdecl`, four words: the address of the name (an `NiFixedString`), a word
/// for `+0x44`, the renderer and the address of three words of format
/// preferences. Allocates 0x4c bytes (`00aa13e0`); when that succeeds runs
/// `NiSourceTexture::NiSourceTexture`, sets the vtable `0109babc` and clears
/// `+0x48` (a null allocation is used as it is). Copies the preferences to
/// `+0x18..+0x20`, assigns the name to `+0x30` (counted as in `00a60f60`),
/// standardizes and converts it into `+0x34`, clears `+0x48` when set, stores
/// the word at `+0x44` and runs the map's vtable slot `+0xa4`. With a
/// renderer whose slot `+0x144` answers false for the map, the map is deleted
/// (slot 0 with 1) and 0 answered. Answers the map.
pub fn fn_00a61040(e: &mut Engine, name: Ptr, extra: u32, renderer: Ptr, preferences: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x4cu32]).u32();
    let map = if block != 0 {
        e.call(NI_SOURCE_TEXTURE_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, NI_SOURCE_CUBE_MAP_VTABLE);
        e.mem.set_u32(block.wrapping_add(0x48), 0);
        block
    } else {
        0
    };
    for word in 0..3u32 {
        let value = e.mem.u32(preferences.addr().wrapping_add(4 * word));
        e.mem.set_u32(map.wrapping_add(0x18 + 4 * word), value);
    }
    let new_name = e.mem.u32(name.addr());
    let file = map.wrapping_add(0x30);
    if e.mem.u32(file) != new_name {
        if new_name != e.global::<u32>(NULL_NAME_0109B220) {
            e.call(
                IMPORT_INTERLOCKED_INCREMENT,
                &args![new_name.wrapping_sub(8)],
            );
        }
        let old_name = e.mem.u32(file);
        if old_name != e.global::<u32>(NULL_NAME_0109B220) {
            e.call(
                IMPORT_INTERLOCKED_DECREMENT,
                &args![old_name.wrapping_sub(8)],
            );
        }
        e.mem.set_u32(file, new_name);
    }
    e.call(STANDARDIZE_FILE_PATH, &args![file]);
    e.call(CONVERT_FILE_NAME, &args![file, map.wrapping_add(0x34)]);
    if e.mem.u32(map.wrapping_add(0x48)) != 0 {
        e.mem.set_u32(map.wrapping_add(0x48), 0);
    }
    e.mem.set_u32(map.wrapping_add(0x44), extra);
    e.vcall(map, 0xa4, &args![]);
    if !renderer.is_null() && !e.vcall(renderer.addr(), 0x144, &args![map]).bool() {
        e.vcall(map, 0, &args![1u32]);
        return Ptr::NULL;
    }
    Ptr::new(map)
}

/// `NiRenderer::ms_pkRenderer` (`011f4748`); the renderer's virtual +0x144 makes a cube
/// map's renderer data (one word: the map; answers a byte).
const RENDERER_INSTANCE: u32 = 0x011f_4748;
const RENDERER_CREATE_CUBE_MAP_DATA_SLOT: u32 = 0x144;
/// `NiSourceTexture::m_pFile` (Xbox PDB, +0x44 on PC).
const SOURCE_FILE: u32 = 0x44;
/// `NiSourceTexture`'s `RegisterStreamables` (`00a5fa90`), `IsEqual` (`00a5fae0`),
/// `LinkObject` (`00a5f9a0`), `LoadBinary` (`00a60180`) and `GetViewerStrings` (`00a60770`),
/// `__thiscall` with one word.
const NI_SOURCE_TEXTURE_REGISTER_STREAMABLES: u32 = 0x00a5_fa90;
const NI_SOURCE_TEXTURE_IS_EQUAL: u32 = 0x00a5_fae0;
const NI_SOURCE_TEXTURE_LINK_OBJECT: u32 = 0x00a5_f9a0;
const NI_SOURCE_TEXTURE_LOAD_BINARY: u32 = 0x00a6_0180;
const NI_SOURCE_TEXTURE_GET_VIEWER_STRINGS: u32 = 0x00a6_0770;
/// The base class's `LoadBinary` (`00a47350`) and `LinkObject` (`00a47360`), `__thiscall` with
/// the stream (the map names the folded bodies after `NiSequenceStreamHelper`).
const BASE_LOAD_BINARY: u32 = 0x00a4_7350;
const BASE_LINK_OBJECT: u32 = 0x00a4_7360;
/// `NiStream`'s file version (+0xd8), the word at +0x1e8 that `LoadBinary` passes to
/// `LoadFromFiles` as the search path, and its input stream (+0x24c; read function at +0x8,
/// cdecl: stream, buffer, bytes, component sizes, component count; position at +0x4).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_SEARCH_PATH: u32 = 0x1e8;
const STREAM_INPUT: u32 = 0x24c;
/// The first stream version whose cube maps load and link as plain source textures
/// (10.3.0.6).
const VERSION_10_3_0_6: u32 = 0x0a03_0006;
/// `NiStream` methods (`__thiscall` on the stream): `GetNumberOfLinkIDs` (`00a63460`),
/// `GetObjectFromLinkID` (`00a63430`), `LoadCString` (`00a64ce0`, one word: the address of the
/// string pointer), `SetNumberOfLinkIDs` (`00a64700`, one word) and the link-id read
/// `00a645b0` (no words).
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_LOAD_C_STRING: u32 = 0x00a6_4ce0;
const STREAM_SET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_4700;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiGetViewerString` (`00a7d330`, cdecl: a name) and the string array's `SetSize`
/// (`0096ad30`, `__thiscall`: the new size) and `SetAt` (`0096ae90`, `__thiscall`: index,
/// address of the element).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;

/// Deletes the texture's file (its virtual 0 with 1) and clears `m_pFile`, when there is one.
fn delete_file(e: &mut Engine, this: u32) {
    let file = e.mem.u32(this + SOURCE_FILE);
    if file != 0 {
        e.vcall(file, 0, &args![1u32]);
        e.mem.set_u32(this + SOURCE_FILE, 0);
    }
}

// Translated from 00a60800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::CreateRendererData` (Xbox PDB): with a renderer (`011f4748`) its virtual
/// +0x144 makes the data; when that fails the file is deleted and cleared and the answer is
/// false. Otherwise (no renderer, or it succeeded) the file is deleted and cleared and the
/// answer is true.
pub fn ni_source_cube_map_create_renderer_data(e: &mut Engine, this: Ptr) -> bool {
    let renderer = e.mem.u32(RENDERER_INSTANCE);
    if renderer != 0
        && !e
            .vcall(renderer, RENDERER_CREATE_CUBE_MAP_DATA_SLOT, &args![this])
            .bool()
    {
        delete_file(e, this.addr());
        return false;
    }
    delete_file(e, this.addr());
    true
}

// Translated from 00a60850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::RegisterStreamables` (Xbox PDB): `NiSourceTexture::RegisterStreamables`
/// (`00a5fa90`), its byte made 0 or 1.
pub fn ni_source_cube_map_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(NI_SOURCE_TEXTURE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00a60880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::IsEqual` (Xbox PDB): `NiSourceTexture::IsEqual` (`00a5fae0`), then the
/// same pixel layout (+0x18), mipmap (+0x20) and alpha (+0x1c) words and static byte (+0x40).
pub fn ni_source_cube_map_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e
        .call(NI_SOURCE_TEXTURE_IS_EQUAL, &args![this, other])
        .bool()
    {
        return false;
    }
    let (a, b) = (this.addr(), other.addr());
    e.mem.u32(a + 0x18) == e.mem.u32(b + 0x18)
        && e.mem.u32(a + 0x20) == e.mem.u32(b + 0x20)
        && e.mem.u32(a + 0x1c) == e.mem.u32(b + 0x1c)
        && e.mem.u8(a + 0x40) == e.mem.u8(b + 0x40)
}

// Translated from 00a60ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::LinkObject` (Xbox PDB): from stream version 10.3.0.6 on,
/// `NiSourceTexture::LinkObject` (`00a5f9a0`). Before it: the base class's link
/// (`00a47360`), then six stack `NiPointer`s (cleared) get the six linked faces when the
/// stream has exactly six link ids (each `GetObjectFromLinkID` assigned in order); without
/// pixel data (+0x3c) the map is made from them (`LoadFromPixelData`), and the six pointers
/// are released, the last first.
pub fn ni_source_cube_map_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_10_3_0_6 {
        e.call(NI_SOURCE_TEXTURE_LINK_OBJECT, &args![this, stream]);
        return;
    }
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    e.with_stack(0x18, |e, faces| {
        let faces = faces.addr();
        if e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32() == 6 {
            for index in 0..6u32 {
                let face = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
                assign_ni_pointer(e, faces + 4 * index, face);
            }
        }
        if e.mem.u32(this.addr() + SOURCE_PIXEL_DATA) == 0 {
            let face = |e: &Engine, index: u32| Ptr::new(e.mem.u32(faces + 4 * index));
            let (f0, f1, f2) = (face(e, 0), face(e, 1), face(e, 2));
            let (f3, f4, f5) = (face(e, 3), face(e, 4), face(e, 5));
            ni_source_cube_map_load_from_pixel_data(e, this, f0, f1, f2, f3, f4, f5);
        }
        release_faces(e, faces);
    });
}

// Translated from 00a60f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; translated from the disassembled bytes), cdecl, no
/// words: a new `NiSourceCubeMap` (0x4c bytes from `00aa13e0`; `NiSourceTexture`'s
/// constructor `00a5fc10`, the vtable `0109babc`, +0x48 cleared), or 0 when the allocation
/// fails. `fn_00a89cc0` (`nimainsdm.rs`) registers it as the loader for the class name
/// `"NiSourceCubeMap"` (`0109bb6c`).
pub fn fn_00a60f30(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x4cu32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(NI_SOURCE_TEXTURE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, NI_SOURCE_CUBE_MAP_VTABLE);
    e.mem.set_u32(block + 0x48, 0);
    Ptr::new(block)
}

/// One read through the stream's input: `bytes` into `buffer`, with the component size word
/// at `sizes` (one component); the position moves on by what was read.
fn read_input(e: &mut Engine, stream: u32, buffer: u32, bytes: u32, sizes: u32) {
    let input = e.mem.u32(stream + STREAM_INPUT);
    let function = e.mem.u32(input + 8);
    let got = e
        .call(function, &args![input, buffer, bytes, sizes, 1u32])
        .u32();
    let position = e.mem.u32(input + 4).wrapping_add(got);
    e.mem.set_u32(input + 4, position);
}

// Translated from 00a612a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::LoadBinary` (Xbox PDB): from stream version 10.3.0.6 on,
/// `NiSourceTexture::LoadBinary` (`00a60180`). Before it, with a reference held on the map:
/// the base class's load (`00a47350`), the "external" byte (read into the low byte of the
/// stream argument's stack word, cleared first), six face names (`LoadCString` into six
/// cleared stack pointers), six link ids (`SetNumberOfLinkIDs(6)`, then six reads); an
/// external map loads its faces from the files (`LoadFromFiles` with the stream's +0x1e8 as
/// the search path); the six names are freed (`00aa10f0`). Then the pixel layout (+0x18),
/// mipmap (+0x20) and alpha (+0x1c) words (each stored before the next read, through the
/// same two stack words the code alternates between buffer and size) and the static byte
/// (+0x40, read over its old value, stored as 0 or 1). The held reference is given up (the
/// map deleted through its virtual +4 at the last one).
pub fn ni_source_cube_map_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    let stream_addr = stream.addr();
    if e.mem.u32(stream_addr + STREAM_VERSION) >= VERSION_10_3_0_6 {
        e.call(NI_SOURCE_TEXTURE_LOAD_BINARY, &args![this, stream]);
        return;
    }
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![base + 4]);
    // The frame below the return address (`ESP` at entry - 0x24): the static byte (+0x03),
    // two words used as buffer and size (+0x04, +0x08) and the six names (+0x0c); the
    // argument's stack word holds the stream pointer, its low byte the external byte.
    e.with_stack(0x28, |e, frame| {
        let frame = frame.addr();
        let static_byte = frame + 0x03;
        let word_04 = frame + 0x04;
        let word_08 = frame + 0x08;
        let names = frame + 0x0c;
        e.with_stack(4, |e, argument| {
            let external = argument.addr();
            e.mem.set_u32(external, stream_addr);
            e.mem.set_u8(external, 0);
            e.mem.set_u32(word_04, 1);
            read_input(e, stream_addr, external, 1, word_04);
            for index in 0..6u32 {
                e.mem.set_u32(names + 4 * index, 0);
            }
            for index in 0..6u32 {
                e.call(STREAM_LOAD_C_STRING, &args![stream, names + 4 * index]);
            }
            e.call(STREAM_SET_NUMBER_OF_LINK_IDS, &args![stream, 6u32]);
            for _ in 0..6 {
                e.call(STREAM_READ_LINK_ID, &args![stream]);
            }
            if e.mem.u8(external) != 0 {
                let search_path = Ptr::new(e.mem.u32(stream_addr + STREAM_SEARCH_PATH));
                let name = |e: &Engine, index: u32| Ptr::new(e.mem.u32(names + 4 * index));
                let (n0, n1, n2) = (name(e, 0), name(e, 1), name(e, 2));
                let (n3, n4, n5) = (name(e, 3), name(e, 4), name(e, 5));
                ni_source_cube_map_load_from_files(e, this, n0, n1, n2, n3, n4, n5, search_path);
            }
        });
        for index in 0..6u32 {
            let name = e.mem.u32(names + 4 * index);
            e.call(FREE_00AA10F0, &args![name]);
        }
        e.mem.set_u32(word_04, 4);
        read_input(e, stream_addr, word_08, 4, word_04);
        let layout = e.mem.u32(word_08);
        e.mem.set_u32(base + 0x18, layout);
        e.mem.set_u32(word_08, 4);
        read_input(e, stream_addr, word_04, 4, word_08);
        let mipmap = e.mem.u32(word_04);
        e.mem.set_u32(base + 0x20, mipmap);
        e.mem.set_u32(word_08, 4);
        read_input(e, stream_addr, word_04, 4, word_08);
        let alpha = e.mem.u32(word_04);
        let old_static = e.mem.u8(base + 0x40);
        e.mem.set_u32(base + 0x1c, alpha);
        e.mem.set_u8(static_byte, old_static);
        e.mem.set_u32(word_08, 1);
        read_input(e, stream_addr, static_byte, 1, word_08);
        let value = e.mem.u8(static_byte) != 0;
        e.mem.set_u8(base + 0x40, u8::from(value));
    });
    let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![base + 4]).u32();
    if left == 0 {
        e.vcall(base, 4, &args![]);
    }
}

// Translated from 00a61480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::GetViewerStrings` (Xbox PDB): `NiSourceTexture`'s strings (`00a60770`),
/// then the class name (the first word of `ms_RTTI`, through `NiGetViewerString`) appended
/// (the array grown to its size plus `m_usGrowBy` when full, `SetAt` at the old size).
pub fn ni_source_cube_map_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let strings = strings.addr();
    e.call(NI_SOURCE_TEXTURE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(NI_SOURCE_CUBE_MAP_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    let capacity = u32::from(e.mem.u16(strings + 0x8));
    if size >= capacity {
        let grow_by = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow_by + size]);
    }
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![strings, size, element]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_0f60, fn_00a60f60(Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00a6_0cc0,
            ni_source_cube_map_load_from_files(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00a6_1120,
            fn_00a61120(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u32, Ptr) -> Ptr
        ),
        entry!(0x00a60cb0, ni_source_cube_map_get_rtti() -> Ptr),
        entry!(0x00a6_0870, ni_source_cube_map_save_binary(Ptr, Ptr)),
        entry!(
            0x00a6_0940,
            ni_source_cube_map_load_from_pixel_data(
                Ptr,
                Ptr<NiPixelData>,
                Ptr<NiPixelData>,
                Ptr<NiPixelData>,
                Ptr<NiPixelData>,
                Ptr<NiPixelData>,
                Ptr<NiPixelData>,
            )
        ),
        entry!(0x00a6_0910, ni_source_cube_map_destroy_app_pixel_data(Ptr)),
        entry!(0x00a6_1250, ni_source_cube_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a6_08c0,
            ni_source_cube_map_load_pixel_data_from_file(Ptr)
        ),
        entry!(0x00a5_fe30, ni_source_cube_map_create(u32, Ptr, Ptr, u8) -> Ptr),
        entry!(0x00a6_1040, fn_00a61040(Ptr, u32, Ptr, Ptr) -> Ptr),
        entry!(0x00a6_0800, ni_source_cube_map_create_renderer_data(Ptr) -> bool),
        entry!(0x00a6_0850, ni_source_cube_map_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a6_0880, ni_source_cube_map_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a6_0ba0, ni_source_cube_map_link_object(Ptr, Ptr)),
        entry!(0x00a6_0f30, fn_00a60f30() -> Ptr),
        entry!(0x00a6_12a0, ni_source_cube_map_load_binary(Ptr, Ptr)),
        entry!(0x00a6_1480, ni_source_cube_map_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_source_cube_map_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a60cb0, &args![]).u32(), NI_SOURCE_CUBE_MAP_RTTI);
    }

    #[test]
    fn thunk_00a60870_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a6_0870;
        const TARGET: u32 = 0x00a6_0600;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const DELETE: u32 = 0x0f00_3000;
    const RENDERER_PREPARE: u32 = 0x0f00_4144;

    /// The map's vtable `0109babc` (slot 0 at `DELETE`); `LoadFromFiles` sets +0x48 and the
    /// word at +0x3c to `loaded`; a renderer whose slot +0x144 answers `accepts`.
    fn cube_rig(loaded: u32, accepts: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.put_vtable(NI_SOURCE_CUBE_MAP_VTABLE, &[DELETE]);
        e.register(DELETE, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SOURCE_TEXTURE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x48, 0x99);
            Ret::default()
        });
        e.register(SEARCH_PATH_SET_UP_00AA24D0, |_, _| Ret::default());
        e.register(SEARCH_PATH_CLEAN_UP_00AA24F0, |_, _| Ret::default());
        e.register_double(NI_SOURCE_CUBE_MAP_LOAD_FROM_FILES, move |e, a| {
            assert_eq!(e.mem.u32(a[0] + 0x48), 0);
            e.mem.set_u32(a[0] + 0x48, 1);
            e.mem.set_u32(a[0] + 0x3c, loaded);
            Ret::default()
        });
        let renderer = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x52];
        slots[0x51] = RENDERER_PREPARE;
        e.put_vtable(0x0f00_8000, &slots);
        e.mem.set_u32(renderer, 0x0f00_8000);
        e.register_double(RENDERER_PREPARE, move |_, _| u32::from(accepts).into_ret());
        let preferences = e.mem.alloc(12);
        e.mem.set_u32(preferences, 0xa);
        e.mem.set_u32(preferences + 4, 0xb);
        e.mem.set_u32(preferences + 8, 0xc);
        (e, renderer, preferences)
    }

    fn make(e: &mut Engine, renderer: u32, preferences: u32) -> u32 {
        e.call(
            0x00a6_1120,
            &args![
                0x101u32,
                0x102u32,
                0x103u32,
                0x104u32,
                0x105u32,
                0x106u32,
                renderer,
                0x77u32,
                preferences
            ],
        )
        .u32()
    }

    #[test]
    fn fn_00a61120_keeps_the_map_the_renderer_accepts() {
        let (mut e, renderer, preferences) = cube_rig(1, true);
        e.call_log = Some(vec![]);
        let map = make(&mut e, renderer, preferences);
        assert_ne!(map, 0);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x4c]]);
        assert_eq!(e.mem.u32(map), NI_SOURCE_CUBE_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 0x48), 0);
        assert_eq!(
            [
                e.mem.u32(map + 0x18),
                e.mem.u32(map + 0x1c),
                e.mem.u32(map + 0x20)
            ],
            [0xa, 0xb, 0xc]
        );
        let load = calls(&e, NI_SOURCE_CUBE_MAP_LOAD_FROM_FILES);
        assert_eq!(load.len(), 1);
        assert_eq!(
            &load[0][..7],
            [map, 0x101, 0x102, 0x103, 0x104, 0x105, 0x106]
        );
        let search_path = load[0][7];
        assert_eq!(
            calls(&e, SEARCH_PATH_SET_UP_00AA24D0),
            vec![vec![search_path]]
        );
        assert_eq!(
            calls(&e, SEARCH_PATH_CLEAN_UP_00AA24F0),
            vec![vec![search_path]]
        );
        assert_eq!(calls(&e, RENDERER_PREPARE), vec![vec![renderer, map]]);
        assert!(calls(&e, DELETE).is_empty());
    }

    #[test]
    fn fn_00a61120_without_a_renderer_keeps_the_map() {
        let (mut e, _, preferences) = cube_rig(0, false);
        e.call_log = Some(vec![]);
        let map = make(&mut e, 0, preferences);
        assert_ne!(map, 0);
        assert!(calls(&e, DELETE).is_empty());
        assert_eq!(calls(&e, SEARCH_PATH_CLEAN_UP_00AA24F0).len(), 1);
    }

    #[test]
    fn fn_00a61120_deletes_a_map_that_failed_or_was_refused() {
        // Nothing loaded (+0x3c is 0): the renderer is not asked.
        let (mut e, renderer, preferences) = cube_rig(0, true);
        e.call_log = Some(vec![]);
        assert_eq!(make(&mut e, renderer, preferences), 0);
        assert!(calls(&e, RENDERER_PREPARE).is_empty());
        let map = calls(&e, NI_SOURCE_CUBE_MAP_LOAD_FROM_FILES)[0][0];
        assert_eq!(calls(&e, DELETE), vec![vec![map, 1]]);
        assert_eq!(calls(&e, SEARCH_PATH_CLEAN_UP_00AA24F0).len(), 1);
        // Loaded but refused by the renderer.
        let (mut e, renderer, preferences) = cube_rig(1, false);
        e.call_log = Some(vec![]);
        assert_eq!(make(&mut e, renderer, preferences), 0);
        assert_eq!(calls(&e, RENDERER_PREPARE).len(), 1);
        assert_eq!(calls(&e, DELETE).len(), 1);
    }
}

#[cfg(test)]
mod b000h_tests {
    use super::*;

    const DELETE: u32 = 0x0ea0_0000;
    const MAP_SLOT_A4: u32 = 0x0ea0_0010;
    const RENDERER_SLOT_144: u32 = 0x0ea0_0020;
    const SEARCH_RESET: u32 = 0x0ea0_0030;
    const SEARCH_NEXT: u32 = 0x0ea0_0040;
    const CONVERTER_READ: u32 = 0x0ea0_0050;
    const MAP_VTABLE: u32 = 0x0ea1_0000;
    const OBJECT_VTABLE: u32 = 0x0ea2_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
    }

    /// An engine for `00a60f60`: the allocation answers `allocation`, the map's vtable is
    /// `MAP_VTABLE` (the constructor double sets it; the function then stores the real one,
    /// which the test maps to the same slots).
    fn cube_engine(load_at_once: bool, accepted: u32) -> Engine {
        let mut e = Engine::new();
        counting(&mut e);
        e.map(0x0109_b000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        e.set_global::<u8>(LOAD_AT_ONCE_011A9594, u8::from(load_at_once));
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SOURCE_TEXTURE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(STANDARDIZE_FILE_PATH, |_, _| Ret::default());
        e.register(CONVERT_FILE_NAME, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.register(MAP_SLOT_A4, |_, _| Ret::default());
        e.register_double(RENDERER_SLOT_144, move |_, _| accepted.into_ret());
        let mut slots = vec![DELETE; 0x52];
        slots[0xa4 / 4] = MAP_SLOT_A4;
        slots[0x144 / 4] = RENDERER_SLOT_144;
        e.put_vtable(NI_SOURCE_CUBE_MAP_VTABLE, &slots);
        e.put_vtable(MAP_VTABLE, &slots);
        e.call_log = Some(vec![]);
        e
    }

    fn name(e: &mut Engine, count: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, count);
        block + 8
    }

    #[test]
    fn fn_00a60f60_makes_the_map_and_keeps_the_name() {
        let mut e = cube_engine(false, 1);
        let new_name = name(&mut e, 1);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, new_name);
        let preferences = e.mem.alloc(0xc);
        for word in 0..3u32 {
            e.mem.set_u32(preferences + 4 * word, 0x10 + word);
        }
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, MAP_VTABLE);
        let map = e
            .call(0x00a6_0f60, &args![holder, renderer, preferences])
            .u32();
        assert_ne!(map, 0);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x4c]]);
        assert_eq!(e.mem.u32(map), NI_SOURCE_CUBE_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 0x18), 0x10);
        assert_eq!(e.mem.u32(map + 0x20), 0x12);
        assert_eq!(e.mem.u32(map + 0x30), new_name);
        assert_eq!(e.mem.u32(new_name - 8), 2);
        assert_eq!(calls(&e, STANDARDIZE_FILE_PATH), vec![vec![map + 0x30]]);
        assert_eq!(
            calls(&e, CONVERT_FILE_NAME),
            vec![vec![map + 0x30, map + 0x34]]
        );
        // Without the load byte nothing else runs.
        assert!(calls(&e, MAP_SLOT_A4).is_empty());
        assert!(calls(&e, RENDERER_SLOT_144).is_empty());
    }

    #[test]
    fn fn_00a60f60_loads_at_once_and_drops_a_refused_map() {
        let mut e = cube_engine(true, 0x100);
        let holder = e.mem.alloc(4);
        let preferences = e.mem.alloc(0xc);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, MAP_VTABLE);
        let map = e
            .call(0x00a6_0f60, &args![holder, renderer, preferences])
            .u32();
        assert_eq!(map, 0);
        let made = calls(&e, NI_SOURCE_TEXTURE_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, MAP_SLOT_A4), vec![vec![made]]);
        assert_eq!(calls(&e, RENDERER_SLOT_144), vec![vec![renderer, made]]);
        assert_eq!(calls(&e, DELETE), vec![vec![made, 1]]);
        // The empty name (0, the word at 0109b220) is neither counted nor released.
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        // Without a renderer the map is kept.
        let mut e = cube_engine(true, 0);
        let holder = e.mem.alloc(4);
        let preferences = e.mem.alloc(0xc);
        let map = e.call(0x00a6_0f60, &args![holder, 0u32, preferences]).u32();
        assert_ne!(map, 0);
        assert!(calls(&e, DELETE).is_empty());
    }

    /// An engine for `LoadFromFiles`: the search path offers `tries` paths per face, the
    /// last of which exists unless `missing_face` is that face; the converter reads a new
    /// object (count 0) for an existing path.
    fn load_engine(tries: u32, missing_face: Option<u32>) -> (Engine, u32, u32, Vec<u32>) {
        let mut e = Engine::new();
        counting(&mut e);
        e.map(0x0eb0_0000, 0x100);
        e.register(GET_IMAGE_CONVERTER, |e, _| {
            let converter = e.mem.alloc(4);
            e.mem.set_u32(converter, MAP_VTABLE);
            converter.into_ret()
        });
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(PATH_STANDARDIZE, |_, _| Ret::default());
        e.register(CONVERT_FILE_NAME_TO_PLATFORM, |e, a| {
            let converted = e.mem.alloc(8);
            e.mem.set_u32(converted, a[0]);
            converted.into_ret()
        });
        e.register(SEARCH_PATH_SET_FILE_PATH, |_, _| Ret::default());
        // 0eb00000: the face being searched; 0eb00004: paths offered so far for it.
        e.register(SEARCH_RESET, |e, _| {
            e.mem.set_u32(0x0eb0_0004, 0);
            Ret::default()
        });
        e.register_double(SEARCH_NEXT, move |e, a| {
            let offered = e.mem.u32(0x0eb0_0004) + 1;
            e.mem.set_u32(0x0eb0_0004, offered);
            e.mem.set_u32(a[1], offered);
            u32::from(offered <= tries).into_ret()
        });
        e.register_double(FILE_ACCESS, move |e, a| {
            let face = e.mem.u32(0x0eb0_0000);
            let exists = e.mem.u32(a[0]) == tries && missing_face != Some(face);
            u32::from(exists).into_ret()
        });
        e.register(CONVERTER_READ, |e, _| {
            let face = e.mem.u32(0x0eb0_0000);
            e.mem.set_u32(0x0eb0_0000, face + 1);
            let texture = e.mem.alloc(8);
            e.mem.set_u32(texture, OBJECT_VTABLE);
            texture.into_ret()
        });
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.register(LOAD_FROM_PIXEL_DATA, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        let mut slots = vec![DELETE; 4];
        slots[1] = SEARCH_RESET;
        slots[2] = SEARCH_NEXT;
        slots[3] = CONVERTER_READ;
        e.put_vtable(MAP_VTABLE, &slots);
        e.put_vtable(OBJECT_VTABLE, &[DELETE, DELETE]);
        let search = e.mem.alloc(4);
        e.mem.set_u32(search, MAP_VTABLE);
        let this = e.mem.alloc(0x4c);
        let names: Vec<u32> = (0..6)
            .map(|index| {
                let text = e.mem.alloc(8);
                e.mem.set_cstr(text, format!("face{index}").as_bytes());
                text
            })
            .collect();
        e.call_log = Some(vec![]);
        (e, this, search, names)
    }

    fn load(e: &mut Engine, this: u32, names: &[u32], search: u32) {
        let mut words = vec![this];
        words.extend_from_slice(names);
        words.push(search);
        e.call(0x00a6_0cc0, &words);
    }

    #[test]
    fn ni_source_cube_map_load_from_files_loads_six_faces() {
        let (mut e, this, search, names) = load_engine(2, None);
        load(&mut e, this, &names, search);
        let copies = calls(&e, STRCPY_S);
        assert_eq!(copies.len(), 6);
        for (index, copy) in copies.iter().enumerate() {
            assert_eq!(copy[1], 6);
            assert_eq!(copy[2], names[index]);
        }
        let loaded = calls(&e, LOAD_FROM_PIXEL_DATA);
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0][0], this);
        let faces = &loaded[0][1..];
        for face in faces {
            // Held by the stack pointer only, released at the end: count 0, deleted.
            assert_eq!(e.mem.u32(face + 4), 0);
        }
        assert_eq!(calls(&e, DELETE).len(), 6);
        assert_eq!(calls(&e, FREE_00AA10F0).len(), 12);
        assert_eq!(calls(&e, FILE_ACCESS).len(), 12);
    }

    #[test]
    fn ni_source_cube_map_load_from_files_stops_at_a_missing_face() {
        let (mut e, this, search, names) = load_engine(2, Some(2));
        load(&mut e, this, &names, search);
        assert_eq!(calls(&e, STRCPY_S).len(), 3);
        assert!(calls(&e, LOAD_FROM_PIXEL_DATA).is_empty());
        // Two faces were read and released; the missing face's names are not freed.
        assert_eq!(calls(&e, DELETE).len(), 2);
        assert_eq!(calls(&e, FREE_00AA10F0).len(), 4);
    }
}

#[cfg(test)]
mod load_from_pixel_data_tests {
    use super::*;

    const DELETE: u32 = 0x0e80_0000;
    const VTABLE: u32 = 0x0e81_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A pixel data object of `levels` levels, the level sizes 0x40, 0x10, 0x4 (offsets 0,
    /// 0x40, 0x50, 0x54 per face), `faces` faces and one reference.
    fn pixel_data(e: &mut Engine, levels: u32, faces: u32) -> u32 {
        let data = e.mem.alloc(0x74);
        e.mem.set_u32(data, VTABLE);
        e.mem.set_u32(data + 4, 1);
        let widths = e.mem.alloc(0xc);
        let heights = e.mem.alloc(0xc);
        e.mem.set_u32(widths, 8);
        e.mem.set_u32(heights, 2);
        let offsets = e.mem.alloc(0x10);
        for (index, offset) in [0u32, 0x40, 0x50, 0x54].iter().enumerate() {
            e.mem.set_u32(offsets + 4 * index as u32, *offset);
        }
        let pixels = e.mem.alloc(0x54 * faces);
        e.mem.set_u32(data + 0x50, pixels);
        e.mem.set_u32(data + 0x54, widths);
        e.mem.set_u32(data + 0x58, heights);
        e.mem.set_u32(data + 0x5c, offsets);
        e.mem.set_u32(data + 0x60, levels);
        data
    }

    fn engine(allocate: bool) -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.register_double(NI_ALLOCATE, move |e, a| {
            if allocate {
                e.mem.alloc(a[0]).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        // The constructor makes the block a three-level, six-face pixel data object.
        e.register(NI_PIXEL_DATA_CONSTRUCT_00A7C190, |e, a| {
            let made = pixel_data(e, 3, 6);
            let block = a[0];
            let words = e.mem.bytes(made, 0x74);
            e.mem.write(block, &words);
            e.mem.set_u32(block + 4, 0);
            block.into_ret()
        });
        e.register(MEMCPY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn ni_source_cube_map_load_from_pixel_data_copies_every_level_of_every_face() {
        let mut e = engine(true);
        let this = e.mem.alloc(0x48);
        let old = pixel_data(&mut e, 1, 1);
        e.mem.set_u32(this + SOURCE_PIXEL_DATA, old);
        let faces: Vec<u32> = (0..6).map(|_| pixel_data(&mut e, 3, 1)).collect();
        let mut words = vec![this];
        words.extend_from_slice(&faces);
        e.call(0x00a6_0940, &words);
        let made = e.mem.u32(this + SOURCE_PIXEL_DATA);
        assert_ne!(made, old);
        assert_eq!(e.mem.u32(made + 4), 1);
        // The old pixel data lost its only reference.
        assert_eq!(calls(&e, DELETE)[0], vec![old]);
        assert_eq!(
            calls(&e, NI_PIXEL_DATA_CONSTRUCT_00A7C190),
            vec![vec![made, 8, 2, faces[0] + 8, 3, 6]]
        );
        let copies = calls(&e, MEMCPY);
        assert_eq!(copies.len(), 18);
        let target = e.mem.u32(made + 0x50);
        let sizes = [0x40u32, 0x10, 0x4];
        let starts = [0u32, 0x40, 0x50];
        for face in 0..6u32 {
            let source = e.mem.u32(faces[face as usize] + 0x50);
            for level in 0..3u32 {
                let copy = &copies[(face * 3 + level) as usize];
                assert_eq!(
                    copy,
                    &vec![
                        target + 0x54 * face + starts[level as usize],
                        source + starts[level as usize],
                        sizes[level as usize]
                    ]
                );
            }
        }
        // The faces keep their own reference: taken and given back once each.
        for face in &faces {
            assert_eq!(e.mem.u32(face + 4), 1);
        }
        assert_eq!(calls(&e, DELETE).len(), 1);
        let released: Vec<u32> = calls(&e, IMPORT_INTERLOCKED_DECREMENT)
            .iter()
            .map(|a| a[0] - 4)
            .collect();
        let mut last_first = faces.clone();
        last_first.reverse();
        assert_eq!(released[1..], last_first[..]);
    }

    #[test]
    fn ni_source_cube_map_load_from_pixel_data_without_memory_reads_through_null() {
        let mut e = engine(false);
        // The copies then read the null pixel data's fields, as the exe does.
        e.map(0, 0x1000);
        let this = e.mem.alloc(0x48);
        let faces: Vec<u32> = (0..6)
            .map(|index| pixel_data(&mut e, u32::from(index == 2), 1))
            .collect();
        let mut words = vec![this];
        words.extend_from_slice(&faces);
        e.call(0x00a6_0940, &words);
        assert_eq!(e.mem.u32(this + SOURCE_PIXEL_DATA), 0);
        assert!(calls(&e, NI_PIXEL_DATA_CONSTRUCT_00A7C190).is_empty());
        // Only face 2 has a level; faces without levels copy nothing.
        let copies = calls(&e, MEMCPY);
        assert_eq!(copies.len(), 1);
        let source = e.mem.u32(faces[2] + 0x50);
        assert_eq!(copies[0], vec![0, source, 0x40]);
        assert!(calls(&e, DELETE).is_empty());
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    const DELETE_SLOT: u32 = 0x0125_0000;

    /// An object with a count at `+4` whose vtable slot `+4` is `DELETE_SLOT`.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_SLOT);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let left = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], left);
            left.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(DELETE_SLOT, |_, _| Ret::default());
        e
    }

    fn deletes(e: &mut Engine) -> usize {
        e.call_log
            .take()
            .unwrap()
            .iter()
            .filter(|c| c.0 == DELETE_SLOT)
            .count()
    }

    #[test]
    fn ni_source_cube_map_destroy_app_pixel_data_releases_the_data() {
        let mut e = engine();
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        e.call(0x00a6_0910, &args![this]);
        assert_eq!(deletes(&mut e), 0);
        let data = counted(&mut e, 2);
        e.mem.set_u32(this + 0x3c, data);
        e.call_log = Some(vec![]);
        e.call(0x00a6_0910, &args![this]);
        assert_eq!(deletes(&mut e), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        e.mem.set_u32(this + 0x3c, data);
        e.call_log = Some(vec![]);
        e.call(0x00a6_0910, &args![this]);
        assert_eq!(deletes(&mut e), 1);
    }

    #[test]
    fn ni_source_cube_map_scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = engine();
        e.register(NI_SOURCE_TEXTURE_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x4c);
        let data = counted(&mut e, 1);
        e.mem.set_u32(this + 0x3c, data);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_1250, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let names: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            names,
            vec![
                0x00a6_1250,
                IMPORT_INTERLOCKED_DECREMENT,
                DELETE_SLOT,
                NI_SOURCE_TEXTURE_DESTRUCT,
                SIZED_DELETE
            ]
        );
        assert_eq!(log[4].1, vec![this, 0x4c]);
        assert_eq!(e.mem.u32(this), NI_SOURCE_CUBE_MAP_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x00a6_1250, &args![this, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != SIZED_DELETE));
    }

    #[test]
    fn ni_source_cube_map_load_pixel_data_from_file_only_when_needed() {
        const CONVERT_SLOT: u32 = 0x0125_1000;
        let mut e = engine();
        e.map(NULL_NAME_0109B220, 4);
        e.set_global::<u32>(NULL_NAME_0109B220, 0x4444);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 0xc, CONVERT_SLOT);
        let converter = e.mem.alloc(4);
        e.mem.set_u32(converter, vtable);
        e.register_double(GET_IMAGE_CONVERTER, move |_, _| converter.into_ret());
        e.register(CONVERT_SLOT, |_, _| 0x7777u32.into_ret());
        e.register(NI_POINTER_ASSIGN_00A5F860, |_, _| Ret::default());
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 0x34, 0x4444);
        e.mem.set_u32(this + 0x44, 5);
        e.call_log = Some(vec![]);
        e.call(0x00a6_08c0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(this + 0x34, 0x8008);
        e.mem.set_u32(this + 0x3c, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a6_08c0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(this + 0x3c, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a6_08c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (GET_IMAGE_CONVERTER, vec![]),
                (CONVERT_SLOT, vec![converter, 0x8008, 0, 5]),
                (NI_POINTER_ASSIGN_00A5F860, vec![this + 0x3c, 0x7777]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x44), 0);
    }

    const KEEP_SLOT: u32 = 0x0125_2000;
    const LOAD_SLOT: u32 = 0x0125_2010;

    /// A texture block whose vtable has `+0xac` = `KEEP_SLOT`, `+0xa4` =
    /// `LOAD_SLOT` and `+0` = `DELETE_SLOT`.
    fn texture_engine(keep: u32) -> (Engine, u32) {
        let mut e = engine();
        e.map(NULL_NAME_0109B220, 4);
        e.set_global::<u32>(NULL_NAME_0109B220, 0x4444);
        e.map(LOAD_AT_ONCE_011A9594, 4);
        let vtable = e.mem.alloc(0x150);
        e.mem.set_u32(vtable, DELETE_SLOT);
        e.mem.set_u32(vtable + 0xac, KEEP_SLOT);
        e.mem.set_u32(vtable + 0xa4, LOAD_SLOT);
        // The cube map gets its class's own vtable: give it the same slots.
        e.map(NI_SOURCE_CUBE_MAP_VTABLE, 0x150);
        e.mem.set_u32(NI_SOURCE_CUBE_MAP_VTABLE, DELETE_SLOT);
        e.mem.set_u32(NI_SOURCE_CUBE_MAP_VTABLE + 0xac, KEEP_SLOT);
        e.mem.set_u32(NI_SOURCE_CUBE_MAP_VTABLE + 0xa4, LOAD_SLOT);
        let block = e.mem.alloc(0x4c);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.register_double(NI_SOURCE_TEXTURE_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], vtable);
            a[0].into_ret()
        });
        e.register_double(KEEP_SLOT, move |_, _| keep.into_ret());
        e.register(LOAD_SLOT, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.register(FIXED_STRING_CREATE, |_, _| 0x8008u32.into_ret());
        e.register(STANDARDIZE_FILE_PATH, |_, _| Ret::default());
        e.register(CONVERT_FILE_NAME, |_, _| Ret::default());
        (e, block)
    }

    #[test]
    fn ni_source_cube_map_create_names_and_keeps_the_texture() {
        let (mut e, block) = texture_engine(0);
        let preferences = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_u32(preferences + 4 * i, 0x10 + i);
        }
        let name = e.mem.alloc(8);
        e.mem.set_u32(block + 0x30, 0x6008);
        e.call_log = Some(vec![]);
        let made = e
            .call(0x00a5_fe30, &args![0x55u32, name, preferences, 1u32])
            .u32();
        assert_eq!(made, block);
        assert_eq!(e.mem.u32(block + 0x18), 0x10);
        assert_eq!(e.mem.u32(block + 0x20), 0x12);
        assert_eq!(e.mem.u8(block + 0x41), 1);
        assert_eq!(e.mem.u32(block + 0x44), 0x55);
        assert_eq!(e.mem.u32(block + 0x30), 0x8008);
        let log = e.call_log.take().unwrap();
        let counts: Vec<(u32, u32)> = log
            .iter()
            .filter(|c| c.0 == IMPORT_INTERLOCKED_INCREMENT || c.0 == IMPORT_INTERLOCKED_DECREMENT)
            .map(|c| (c.0, c.1[0]))
            .collect();
        assert_eq!(
            counts,
            vec![
                (IMPORT_INTERLOCKED_INCREMENT, 0x8000),
                (IMPORT_INTERLOCKED_DECREMENT, 0x6000),
                (IMPORT_INTERLOCKED_DECREMENT, 0x8000),
            ]
        );
        assert!(log.iter().all(|c| c.0 != KEEP_SLOT));
    }

    #[test]
    fn ni_source_cube_map_create_deletes_a_refused_texture() {
        let (mut e, block) = texture_engine(0);
        e.set_global::<u8>(LOAD_AT_ONCE_011A9594, 1);
        let preferences = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        let made = e
            .call(0x00a5_fe30, &args![0u32, 0u32, preferences, 0u32])
            .u32();
        assert_eq!(made, 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != FIXED_STRING_CREATE));
        assert!(log.contains(&(DELETE_SLOT, vec![block, 1])));
        let (mut e, block) = texture_engine(1);
        e.set_global::<u8>(LOAD_AT_ONCE_011A9594, 1);
        let preferences = e.mem.alloc(12);
        let made = e
            .call(0x00a5_fe30, &args![0u32, 0u32, preferences, 0u32])
            .u32();
        assert_eq!(made, block);
    }

    #[test]
    fn fn_00a61040_makes_and_checks_the_map() {
        const CHECK_SLOT: u32 = 0x0125_3000;
        let (mut e, block) = texture_engine(1);
        let preferences = e.mem.alloc(12);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x4444);
        e.mem.set_u32(block + 0x30, 0x4444);
        e.call_log = Some(vec![]);
        let made = e
            .call(0x00a6_1040, &args![name, 0x66u32, 0u32, preferences])
            .u32();
        assert_eq!(made, block);
        assert_eq!(e.mem.u32(block), NI_SOURCE_CUBE_MAP_VTABLE);
        assert_eq!(e.mem.u32(block + 0x44), 0x66);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .all(|c| c.0 != IMPORT_INTERLOCKED_INCREMENT && c.0 != IMPORT_INTERLOCKED_DECREMENT));

        let (mut e, block) = texture_engine(1);
        e.register(CHECK_SLOT, |_, _| Ret::default());
        let renderer_vtable = e.mem.alloc(0x150);
        e.mem.set_u32(renderer_vtable + 0x144, CHECK_SLOT);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, renderer_vtable);
        let preferences = e.mem.alloc(12);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x8008);
        e.call_log = Some(vec![]);
        let made = e
            .call(0x00a6_1040, &args![name, 0u32, renderer, preferences])
            .u32();
        assert_eq!(made, 0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(CHECK_SLOT, vec![renderer, block])));
        assert!(log.contains(&(DELETE_SLOT, vec![block, 1])));
        assert!(log.contains(&(IMPORT_INTERLOCKED_INCREMENT, vec![0x8000])));
    }
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An object whose virtual table has `slots` (byte offset, target).
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        object
    }

    fn reference_counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
    }

    const RENDERER_DOUBLE: u32 = 0x0070_9000;
    const DELETE_DOUBLE: u32 = 0x0070_9010;
    const READ_DOUBLE: u32 = 0x0070_9020;
    const SEARCH_DOUBLE: u32 = 0x0070_9030;

    #[test]
    fn create_renderer_data_deletes_the_file_either_way() {
        let mut e = Engine::new();
        e.map(RENDERER_INSTANCE, 4);
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        let map = e.mem.alloc(0x4c);
        let file = object(&mut e, 0x10, &[(0, DELETE_DOUBLE)]);
        e.mem.set_u32(map + 0x44, file);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a6_0800, &args![map]).bool());
        assert_eq!(e.mem.u32(map + 0x44), 0);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![file, 1]]);
        let renderer = object(&mut e, 0x10, &[(0x144, RENDERER_DOUBLE)]);
        e.mem.set_u32(RENDERER_INSTANCE, renderer);
        e.register(RENDERER_DOUBLE, |_, _| word(0x100));
        e.mem.set_u32(map + 0x44, file);
        assert!(!e.call(0x00a6_0800, &args![map]).bool());
        assert_eq!(e.mem.u32(map + 0x44), 0);
        assert_eq!(calls_to(&e, RENDERER_DOUBLE), vec![vec![renderer, map]]);
        assert_eq!(calls_to(&e, DELETE_DOUBLE).len(), 2);
        e.register(RENDERER_DOUBLE, |_, _| word(1));
        // Without a file nothing is deleted.
        assert!(e.call(0x00a6_0800, &args![map]).bool());
        assert_eq!(calls_to(&e, DELETE_DOUBLE).len(), 2);
    }

    #[test]
    fn register_streamables_and_is_equal_build_on_the_source_texture() {
        let mut e = Engine::new();
        e.register(NI_SOURCE_TEXTURE_REGISTER_STREAMABLES, |_, a| {
            word(u32::from(a[1] == 0x100))
        });
        assert!(e.call(0x00a6_0850, &args![0x10u32, 0x100u32]).bool());
        assert!(!e.call(0x00a6_0850, &args![0x10u32, 0x200u32]).bool());
        e.register(NI_SOURCE_TEXTURE_IS_EQUAL, |_, _| word(1));
        let a = e.mem.alloc(0x4c);
        let b = e.mem.alloc(0x4c);
        assert!(e.call(0x00a6_0880, &args![a, b]).bool());
        for (offset, wide) in [(0x18u32, true), (0x20, true), (0x1c, true), (0x40, false)] {
            if wide {
                e.mem.set_u32(b + offset, 7);
            } else {
                e.mem.set_u8(b + offset, 1);
            }
            assert!(!e.call(0x00a6_0880, &args![a, b]).bool());
            e.mem.set_u32(b + offset, 0);
        }
        e.register(NI_SOURCE_TEXTURE_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a6_0880, &args![a, b]).bool());
    }

    #[test]
    fn link_object_makes_the_map_from_six_linked_faces() {
        let mut e = Engine::new();
        reference_counting(&mut e);
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        e.register(BASE_LINK_OBJECT, |_, _| Ret::default());
        e.register(NI_SOURCE_TEXTURE_LINK_OBJECT, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |_, _| word(0));
        let faces: Vec<u32> = (0..6)
            .map(|_| {
                let face = object(&mut e, 0x80, &[(4, DELETE_DOUBLE)]);
                e.mem.set_u32(face + 4, 1);
                face
            })
            .collect();
        let queue = Rc::new(RefCell::new(VecDeque::from(faces.clone())));
        let next = queue.clone();
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            word(next.borrow_mut().pop_front().unwrap_or(0))
        });
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| word(6));
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a03_0005);
        let map = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        e.call(0x00a6_0ba0, &args![map, stream]);
        assert_eq!(calls_to(&e, BASE_LINK_OBJECT), vec![vec![map, stream]]);
        assert_eq!(calls_to(&e, STREAM_GET_OBJECT_FROM_LINK_ID).len(), 6);
        // `LoadFromPixelData` ran (its allocation), and every face is back to one reference.
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x74]]);
        for face in &faces {
            assert_eq!(e.mem.u32(face + 4), 1);
        }
        // The last face is released first.
        let releases = calls_to(&e, IMPORT_INTERLOCKED_DECREMENT);
        assert_eq!(releases[releases.len() - 1], vec![faces[0] + 4]);
        assert_eq!(releases[releases.len() - 6], vec![faces[5] + 4]);
        // With pixel data, and with another link count, no map is made.
        e.mem.set_u32(map + 0x3c, 0x5555);
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| word(5));
        e.call(0x00a6_0ba0, &args![map, stream]);
        assert_eq!(calls_to(&e, STREAM_GET_OBJECT_FROM_LINK_ID).len(), 6);
        assert_eq!(calls_to(&e, NI_ALLOCATE).len(), 1);
        // From 10.3.0.6 on the source texture links.
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a03_0006);
        e.call(0x00a6_0ba0, &args![map, stream]);
        assert_eq!(
            calls_to(&e, NI_SOURCE_TEXTURE_LINK_OBJECT),
            vec![vec![map, stream]]
        );
        assert_eq!(calls_to(&e, BASE_LINK_OBJECT).len(), 2);
    }

    #[test]
    fn fn_00a60f30_makes_a_cube_map() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x4c);
        e.mem.set_u32(block + 0x48, 9);
        e.register_double(NI_ALLOCATE, move |_, _| word(block));
        e.register(NI_SOURCE_TEXTURE_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_0f30, &args![]).u32(), block);
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x4c]]);
        assert_eq!(calls_to(&e, NI_SOURCE_TEXTURE_CONSTRUCT), vec![vec![block]]);
        assert_eq!(e.mem.u32(block), NI_SOURCE_CUBE_MAP_VTABLE);
        assert_eq!(e.mem.u32(block + 0x48), 0);
        e.register(NI_ALLOCATE, |_, _| word(0));
        assert_eq!(e.call(0x00a6_0f30, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, NI_SOURCE_TEXTURE_CONSTRUCT).len(), 1);
    }

    type Reads = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// A stream of `version` whose input serves `bytes` in order and records each read (the
    /// buffer, the byte count and the size word).
    fn input_stream(e: &mut Engine, version: u32, bytes: &[u8]) -> (u32, Reads) {
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, READ_DOUBLE);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        let mut queue = VecDeque::from(bytes.to_vec());
        let reads: Reads = Rc::new(RefCell::new(vec![]));
        let log = reads.clone();
        e.register_double(READ_DOUBLE, move |e, a| {
            log.borrow_mut().push((a[1], a[2], e.mem.u32(a[3])));
            for i in 0..a[2] {
                let byte = queue.pop_front().unwrap();
                e.mem.set_u8(a[1] + i, byte);
            }
            word(a[2])
        });
        (stream, reads)
    }

    fn load_doubles(e: &mut Engine) {
        reference_counting(e);
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(NI_SOURCE_TEXTURE_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_SET_NUMBER_OF_LINK_IDS, |_, _| Ret::default());
        e.register(STREAM_READ_LINK_ID, |_, _| Ret::default());
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        // Each name is a new one-letter string.
        e.register(STREAM_LOAD_C_STRING, |e, a| {
            let name = e.mem.alloc(2);
            e.mem.set_u8(name, b'f');
            e.mem.set_u32(a[1], name);
            Ret::default()
        });
    }

    fn values(layout: u32, mipmap: u32, alpha: u32, fixed: u8) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&layout.to_le_bytes());
        bytes.extend_from_slice(&mipmap.to_le_bytes());
        bytes.extend_from_slice(&alpha.to_le_bytes());
        bytes.push(fixed);
        bytes
    }

    #[test]
    fn load_binary_of_an_old_internal_cube_map() {
        let mut e = Engine::new();
        load_doubles(&mut e);
        let mut bytes = vec![0u8];
        bytes.extend(values(3, 1, 2, 5));
        let (stream, reads) = input_stream(&mut e, 0x0a03_0005, &bytes);
        let map = object(&mut e, 0x4c, &[(4, DELETE_DOUBLE)]);
        e.mem.set_u32(map + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a6_12a0, &args![map, stream]);
        assert_eq!(calls_to(&e, BASE_LOAD_BINARY), vec![vec![map, stream]]);
        assert_eq!(calls_to(&e, STREAM_LOAD_C_STRING).len(), 6);
        assert_eq!(
            calls_to(&e, STREAM_SET_NUMBER_OF_LINK_IDS),
            vec![vec![stream, 6]]
        );
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID).len(), 6);
        let names: Vec<u32> = calls_to(&e, STREAM_LOAD_C_STRING)
            .iter()
            .map(|c| e.mem.u32(c[1]))
            .collect();
        let freed: Vec<u32> = calls_to(&e, FREE_00AA10F0).iter().map(|c| c[0]).collect();
        assert_eq!(freed.len(), 6);
        assert!(freed.iter().all(|&f| f == 0 || names.contains(&f)));
        assert_eq!(e.mem.u32(map + 0x18), 3);
        assert_eq!(e.mem.u32(map + 0x20), 1);
        assert_eq!(e.mem.u32(map + 0x1c), 2);
        assert_eq!(e.mem.u8(map + 0x40), 1);
        let sizes: Vec<(u32, u32)> = reads.borrow().iter().map(|r| (r.1, r.2)).collect();
        assert_eq!(sizes, vec![(1, 1), (4, 4), (4, 4), (4, 4), (1, 1)]);
        // The held reference is given up without deleting.
        assert_eq!(e.mem.u32(map + 4), 1);
        assert_eq!(calls_to(&e, DELETE_DOUBLE).len(), 0);
        assert_eq!(calls_to(&e, GET_IMAGE_CONVERTER).len(), 0);
    }

    #[test]
    fn load_binary_of_an_old_external_cube_map_loads_the_files() {
        let mut e = Engine::new();
        load_doubles(&mut e);
        // `LoadFromFiles` runs for real: its search path finds no file.
        for callee in [
            GET_IMAGE_CONVERTER,
            STRCPY_S,
            PATH_STANDARDIZE,
            CONVERT_FILE_NAME_TO_PLATFORM,
            SEARCH_PATH_SET_FILE_PATH,
            SEARCH_DOUBLE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(ALLOCATE_00AA1070, |e, a| word(e.mem.alloc(a[0])));
        let search = object(&mut e, 0x10, &[(4, SEARCH_DOUBLE), (8, SEARCH_DOUBLE)]);
        let mut bytes = vec![1u8];
        bytes.extend(values(4, 0, 1, 0));
        let (stream, _) = input_stream(&mut e, 0x0a03_0005, &bytes);
        e.mem.set_u32(stream + STREAM_SEARCH_PATH, search);
        let map = object(&mut e, 0x4c, &[(4, DELETE_DOUBLE)]);
        e.mem.set_u32(map + 0x40, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a6_12a0, &args![map, stream]);
        let first_name = e.mem.u32(calls_to(&e, STREAM_LOAD_C_STRING)[0][1]);
        assert_eq!(calls_to(&e, STRCPY_S)[0][2], first_name);
        assert_eq!(calls_to(&e, SEARCH_DOUBLE)[0], vec![search]);
        assert_eq!(e.mem.u32(map + 0x18), 4);
        assert_eq!(e.mem.u8(map + 0x40), 0);
        // The count went from 0 to 1 and back: the map is deleted.
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![map]]);
    }

    #[test]
    fn load_binary_from_10_3_0_6_is_the_source_texture_load() {
        let mut e = Engine::new();
        load_doubles(&mut e);
        let (stream, reads) = input_stream(&mut e, 0x0a03_0006, &[]);
        let map = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        e.call(0x00a6_12a0, &args![map, stream]);
        assert_eq!(
            calls_to(&e, NI_SOURCE_TEXTURE_LOAD_BINARY),
            vec![vec![map, stream]]
        );
        assert_eq!(calls_to(&e, BASE_LOAD_BINARY).len(), 0);
        assert!(reads.borrow().is_empty());
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = Engine::new();
        e.map(NI_SOURCE_CUBE_MAP_RTTI, 8);
        e.mem.set_u32(NI_SOURCE_CUBE_MAP_RTTI, 0x0109_bb6c);
        e.register(NI_SOURCE_TEXTURE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, a| word(a[0] + 1));
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x20, value);
            Ret::default()
        });
        let strings = e.mem.alloc(0x40);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 2);
        let map = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        e.call(0x00a6_1480, &args![map, strings]);
        assert_eq!(
            calls_to(&e, NI_SOURCE_TEXTURE_GET_VIEWER_STRINGS),
            vec![vec![map, strings]]
        );
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_bb6c]]);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE).len(), 0);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_AT)[0][1], 2);
        assert_eq!(e.mem.u32(strings + 0x20), 0x0109_bb6d);
        // Full: grown by `m_usGrowBy`.
        e.mem.set_u16(strings + 0xa, 4);
        e.mem.set_u16(strings + 0xe, 2);
        e.call(0x00a6_1480, &args![map, strings]);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![strings, 6]]);
    }
}
