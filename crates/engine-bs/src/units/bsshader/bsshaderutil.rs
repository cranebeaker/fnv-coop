//! `BSShader/bsshaderutil.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSShaderProperty::SetFlagRecurse` (Xbox PDB, `00ba91a0`): `cdecl`, three stack words (the
/// object, the flag index, the value).
const SHADER_PROPERTY_SET_FLAG_RECURSE: u32 = 0x00ba_91a0;

// Translated from 00b6b910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::RecursiveSetPropertyWireframe` (Xbox PDB): `cdecl`, two stack words (the
/// object, the value): `BSShaderProperty::SetFlagRecurse` with flag index `0x30`.
pub fn bs_shader_util_recursive_set_property_wireframe(e: &mut Engine, object: Ptr, value: u32) {
    e.call(
        SHADER_PROPERTY_SET_FLAG_RECURSE,
        &args![object, 0x30u32, value],
    );
}

/// The lock `006503b0` holds while it walks its map: `00652140` takes it, `00652190` gives
/// it back (no arguments; both work on the lock at `011d5a9c` and the count at `011d59dc`).
const MAP_LOCK_ENTER: u32 = 0x0065_2140;
const MAP_LOCK_LEAVE: u32 = 0x0065_2190;
/// `004b9ba0` (`thiscall`, no stack words): the map's first position (0 when empty).
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
/// `00633c90` (`thiscall`, one word): `NiPointer` constructor from a raw pointer.
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiTMapBase<..>::GetNext` (`00559120`, `thiscall`, three words: &position, &key, &value).
const MAP_GET_NEXT: u32 = 0x0055_9120;
/// `00559450` (`thiscall`, no stack words): the word at `this` (a smart pointer's object).
const POINTER_GET: u32 = 0x0055_9450;
/// `00726070` (`thiscall`, no stack words): the word at `this + 4`.
const WORD_AT_4: u32 = 0x0072_6070;
/// `NiTMapBase<..>::RemoveAt` (`00405430`, `thiscall`, one word: the key).
const MAP_REMOVE_AT: u32 = 0x0040_5430;
/// `0066b0d0` (`thiscall`, one word): `NiPointer` assignment from a raw pointer.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `0045cec0` (`thiscall`, no stack words): `NiPointer` destructor.
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;

// Translated from 006503b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map's `BSShaderUtil::UpdateDebugBoundVis` (a call-graph guess the body does not
/// confirm), so named by address: `thiscall`, no stack words. Under the lock `00652140` /
/// `00652190`, walks the map at `this + 4` from its first position with `GetNext` (position,
/// key and an `NiPointer` value in locals) and removes (`RemoveAt`, by key) every entry whose
/// value's object at +8 has the word 1 at +4; the value pointer is cleared after each entry
/// and destroyed at the end.
// Translation incomplete: 006503b0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the NiPointer local) is not translated.
pub fn fn_006503b0(e: &mut Engine, this: Ptr) {
    let map = this.addr().wrapping_add(4);
    e.call(MAP_LOCK_ENTER, &args![]);
    let first = e.call(MAP_FIRST_POSITION, &args![map]).u32();
    e.with_stack(0xc, |e, locals| {
        // EBP-0x10: position, EBP-0x14: key, EBP-0x18: the NiPointer value.
        let position = locals.addr();
        let key = position + 4;
        let value = position + 8;
        e.mem.set_u32(position, first);
        e.call(NI_POINTER_CONSTRUCT, &args![value, 0u32]);
        while e.mem.u32(position) != 0 {
            e.call(MAP_GET_NEXT, &args![map, position, key, value]);
            let object = e.call(POINTER_GET, &args![value]).u32();
            let inner = e.call(POINTER_GET, &args![object.wrapping_add(8)]).u32();
            if e.call(WORD_AT_4, &args![inner]).u32() == 1 {
                let k = e.mem.u32(key);
                e.call(MAP_REMOVE_AT, &args![map, k]);
            }
            e.call(NI_POINTER_ASSIGN, &args![value, 0u32]);
        }
        e.call(MAP_LOCK_LEAVE, &args![]);
        e.call(NI_POINTER_DESTRUCT, &args![value]);
    });
}

// Translated from 00b6b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSShader/bsshaderutil.cpp`): `cdecl`, three stack words (its
/// one caller, `00873200`, passes an animation data pointer, an object and 1). Jumps to the
/// object's virtual at +0xAC with the caller's stack as it is, so that method sees the three
/// words as its stack arguments.
pub fn fn_00b6b930(e: &mut Engine, first: u32, object: Ptr, third: u32) {
    e.vcall(object.addr(), 0xac, &args![first, object, third]);
}

/// The global `00b6ba20`, `AccumulateScene` and `AccumulateSceneList` test first
/// (`011f4748`).
const SCENE_GLOBAL_011F4748: u32 = 0x011f_4748;
/// The global `00b6ba20` stores its first argument in (`011f917c`).
const LAST_OBJECT_011F917C: u32 = 0x011f_917c;
/// `004e9bb0` (`thiscall` on the global at `011f4748`, one word).
const SCENE_SET_OBJECT_004E9BB0: u32 = 0x004e_9bb0;
/// `00b54b10` (`cdecl`, one word).
const PREPARE_OBJECT_00B54B10: u32 = 0x00b5_4b10;

// Translated from 00b6ba20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSShader/bsshaderutil.cpp`): `cdecl`, three stack words.
/// With the global at `011f4748` set and a first argument: calls `004e9bb0` on the global
/// with it, stores it at `011f917c`, calls `00b54b10` with the second argument, then the
/// second argument's virtual +0x8C with the first, and jumps to its virtual +0xA8 with the
/// caller's stack as it is (the three words as that method's stack arguments).
pub fn fn_00b6ba20(e: &mut Engine, first: u32, object: Ptr, third: u32) {
    let scene = e.global::<u32>(SCENE_GLOBAL_011F4748);
    if scene == 0 || first == 0 {
        return;
    }
    e.call(SCENE_SET_OBJECT_004E9BB0, &args![scene, first]);
    e.set_global(LAST_OBJECT_011F917C, first);
    e.call(PREPARE_OBJECT_00B54B10, &args![object]);
    e.vcall(object.addr(), 0x8c, &args![first]);
    e.vcall(object.addr(), 0xa8, &args![first, object, third]);
}

/// `MemoryManager` singleton and `MemoryManager::Allocate` (`thiscall`, one word: size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `00a5b690` (`cdecl`, one word: a C string): an `NiFixedString` handle for the string.
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
/// `BSShaderUtil::pConvertedTextureName` and `pBGConvertedTextureName` (Xbox PDB statics):
/// `NiFixedString*`.
const CONVERTED_TEXTURE_NAME: u32 = 0x011f_a050;
const BG_CONVERTED_TEXTURE_NAME: u32 = 0x011f_a054;
/// `"ConvertedTexture"` and `"BackgroundConvertedTexture"`.
const CONVERTED_TEXTURE_TEXT: u32 = 0x010a_e658;
const BG_CONVERTED_TEXTURE_TEXT: u32 = 0x010a_e63c;

/// A 4-byte `NiFixedString` from the allocator holding `text`'s handle (0 when the
/// allocation failed).
fn new_fixed_string(e: &mut Engine, text: u32) -> u32 {
    let string = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    if string != 0 {
        let handle = e.call(FIXED_STRING_FROM_TEXT, &args![text]).u32();
        e.mem.set_u32(string, handle);
    }
    string
}

// Translated from 00b6ba70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::InitSDM` (Xbox PDB): no arguments. Allocates the two texture-name strings
/// (`"ConvertedTexture"`, `"BackgroundConvertedTexture"`) into `pConvertedTextureName` and
/// `pBGConvertedTextureName` (0 where the allocation failed).
// Translation incomplete: 00b6ba70 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around each allocation) is not translated.
pub fn bs_shader_util_init_sdm(e: &mut Engine) {
    let name = new_fixed_string(e, CONVERTED_TEXTURE_TEXT);
    e.set_global(CONVERTED_TEXTURE_NAME, name);
    let background = new_fixed_string(e, BG_CONVERTED_TEXTURE_TEXT);
    e.set_global(BG_CONVERTED_TEXTURE_NAME, background);
}

// Translated from 00b6bb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::RecursiveSetPropertyFadeAlpha` (Xbox PDB): `cdecl`, two stack words (the
/// node, the alpha as a float). Only for a node with a world bound (+0x20) whose radius
/// (+0xC) is not 0 (or is NaN): for each child slot (`m_kChildren`, elements at +0xA0, count
/// `m_usSize` at +0xA6, read again each step) holding a child, a child that is geometry
/// (virtual +0x18) gets the alpha in its shade property's `fFadeAlpha` (+0x2C) when the
/// property (`m_PropertyState` slot 3, +0xA8) is set and its word at +0x1C is not -1; then the
/// function recurses on what the child's virtual +0xC answers (its node, or 0).
pub fn bs_shader_util_recursive_set_property_fade_alpha(e: &mut Engine, node: Ptr, alpha: f32) {
    if node.is_null() {
        return;
    }
    let bound = e.mem.u32(node.addr() + 0x20);
    if bound == 0 {
        return;
    }
    // FLDZ; FCOMP [bound + 0xC]; TEST AH,0x44; JNP: leave when the radius equals 0.
    let radius = x87_m32(e.mem.u32(bound + 0xc));
    if radius == 0.0 {
        return;
    }
    let mut index = 0u32;
    if e.mem.u16(node.addr() + 0xa6) == 0 {
        return;
    }
    loop {
        let children = e.mem.u32(node.addr() + 0xa0);
        let child = e.mem.u32(children.wrapping_add(index * 4));
        if child != 0 {
            let geometry = e.vcall(child, 0x18, &args![]).u32();
            // LEA ECX,[EAX+0x9C]; TEST ECX,ECX: the property state's address.
            if geometry != 0 && geometry.wrapping_add(0x9c) != 0 {
                let property = e.mem.u32(geometry.wrapping_add(0xa8));
                if property != 0 && e.mem.u32(property.wrapping_add(0x1c)).wrapping_add(1) != 0 {
                    e.mem
                        .set_u32(property.wrapping_add(0x2c), x87_quiet_bits(alpha.to_bits()));
                }
            }
            let next = e.vcall(child, 0xc, &args![]).ptr();
            bs_shader_util_recursive_set_property_fade_alpha(e, next, alpha);
        }
        index += 1;
        if index >= u32::from(e.mem.u16(node.addr() + 0xa6)) {
            break;
        }
    }
}

/// `004dc540` (`thiscall` on the global at `011f4748`, one word).
const SCENE_SET_ACCUMULATOR_004DC540: u32 = 0x004d_c540;
/// `BSShaderManager::SetAccumulator` (`00b54ac0`, `cdecl`, one word).
const SHADER_MANAGER_SET_ACCUMULATOR: u32 = 0x00b5_4ac0;
/// The thread's TLS word `AccumulateScene` tests (`+0x2BC` in the block of `_tls_index`).
const TLS_ACCUMULATE_WORD: u32 = 0x2bc;
/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00b6bee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::AccumulateScene` (Xbox PDB): `cdecl`, three stack words (two objects and
/// the culling process). With the global at `011f4748` and both objects set: unless the
/// thread's TLS word +0x2BC is set, holds a reference to the global's object at +8 (an
/// `NiPointer` local), calls `004dc540` on the global with the process's accumulator
/// (+0xC4) and releases the reference (`0045cec0`); then `BSShaderManager::SetAccumulator`
/// with the accumulator and the process's virtual +0x48 with (first, second, 0).
// Translation incomplete: 00b6bee0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the NiPointer local) is not translated.
pub fn bs_shader_util_accumulate_scene(e: &mut Engine, first: u32, second: u32, process: Ptr) {
    let scene = e.global::<u32>(SCENE_GLOBAL_011F4748);
    if scene == 0 || first == 0 || second == 0 {
        return;
    }
    let tls = e.tls();
    if e.mem.u32(tls + TLS_ACCUMULATE_WORD) == 0 {
        let held = e.mem.u32(scene.wrapping_add(8));
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), held);
            if held != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![held.wrapping_add(4)]);
            }
            let scene = e.global::<u32>(SCENE_GLOBAL_011F4748);
            let accumulator = e.mem.u32(process.addr().wrapping_add(0xc4));
            e.call(SCENE_SET_ACCUMULATOR_004DC540, &args![scene, accumulator]);
            e.call(NI_POINTER_DESTRUCT, &args![local]);
        });
    }
    let accumulator = e.mem.u32(process.addr().wrapping_add(0xc4));
    e.call(SHADER_MANAGER_SET_ACCUMULATOR, &args![accumulator]);
    e.vcall(process.addr(), 0x48, &args![first, second, 0u32]);
}

// Translated from 00b6bfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::AccumulateSceneList` (Xbox PDB): `cdecl`, three stack words (an object, a
/// list whose first word is its head node, the culling process). With the global at
/// `011f4748` and the object set: holds a reference to the global's object at +8, calls
/// `004dc540` on the global and `BSShaderManager::SetAccumulator` with the process's
/// accumulator (+0xC4); for a non-empty list calls the process's virtual +0x48 with (object,
/// first node's element, 0), then its virtual +0x44 with each following node's element
/// (nodes: next at +0, element at +8), stopping after the element before a null one; then
/// releases the reference (`InterlockedDecrement`, `DeleteThis` at 0).
// Translation incomplete: 00b6bfc0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the held reference) is not translated.
pub fn bs_shader_util_accumulate_scene_list(e: &mut Engine, object: u32, list: Ptr, process: Ptr) {
    let scene = e.global::<u32>(SCENE_GLOBAL_011F4748);
    if scene == 0 || object == 0 {
        return;
    }
    let held = e.mem.u32(scene.wrapping_add(8));
    if held != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![held.wrapping_add(4)]);
    }
    let scene = e.global::<u32>(SCENE_GLOBAL_011F4748);
    let accumulator = e.mem.u32(process.addr().wrapping_add(0xc4));
    e.call(SCENE_SET_ACCUMULATOR_004DC540, &args![scene, accumulator]);
    let accumulator = e.mem.u32(process.addr().wrapping_add(0xc4));
    e.call(SHADER_MANAGER_SET_ACCUMULATOR, &args![accumulator]);
    let head = e.mem.u32(list.addr());
    if head != 0 {
        let second = e.mem.u32(head);
        let element = e.mem.u32(head.wrapping_add(8));
        e.vcall(process.addr(), 0x48, &args![object, element, 0u32]);
        if second != 0 {
            let mut node = e.mem.u32(second);
            let mut element = e.mem.u32(second.wrapping_add(8));
            loop {
                let current = element;
                if node != 0 {
                    element = e.mem.u32(node.wrapping_add(8));
                    node = e.mem.u32(node);
                } else {
                    element = 0;
                }
                e.vcall(process.addr(), 0x44, &args![current]);
                if element == 0 {
                    break;
                }
            }
        }
    }
    if held != 0
        && e.call(INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(held, 4, &args![]);
    }
}

/// `NiPixelData` fields `00b6b940` and `00b6bbd0` use: the pixels (`+0x50`), the per-level
/// widths (`+0x54`), heights (`+0x58`) and offsets (`+0x5c`), the level count (`+0x60`) and
/// the revision (`+0x68`).
const PIXELS: u32 = 0x50;
const WIDTHS: u32 = 0x54;
const HEIGHTS: u32 = 0x58;
const OFFSETS: u32 = 0x5c;
const LEVELS: u32 = 0x60;
const REVISION: u32 = 0x68;

/// The bytes of a 4-byte pixel `00b6b940` sets to 0xff for mipmap level `level` (1 to 6;
/// the arms of its switch's jump table at `00b6ba04`, read in Ghidra), none for later levels.
fn level_bytes(level: u32) -> &'static [u32] {
    match level.wrapping_sub(1) {
        0 => &[0],
        1 => &[1],
        2 => &[2],
        3 => &[1, 0],
        4 => &[1, 2],
        5 => &[2, 0],
        _ => &[],
    }
}

// Translated from 00b6b940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `cdecl` with one word (an `NiPixelData`; a debug colouring of
/// the mipmaps): for a non-null pixel data, every 4-byte pixel of mipmap levels 1 to 6 gets
/// 0xff in the bytes of its level's colour (level 1: byte 0; 2: byte 1; 3: byte 2; 4: bytes
/// 1 and 0; 5: bytes 1 and 2; 6: bytes 2 and 0), level 7 and later are walked unchanged, and
/// the revision is raised by one.
pub fn fn_00b6b940(e: &mut Engine, pixel_data: u32) {
    if pixel_data == 0 {
        return;
    }
    let levels = e.mem.u32(pixel_data + LEVELS);
    let mut level = 1u32;
    while level < levels {
        let width = e
            .mem
            .u32(e.mem.u32(pixel_data + WIDTHS).wrapping_add(level * 4));
        let height = e
            .mem
            .u32(e.mem.u32(pixel_data + HEIGHTS).wrapping_add(level * 4));
        let start = e
            .mem
            .u32(e.mem.u32(pixel_data + OFFSETS).wrapping_add(level * 4))
            .wrapping_add(e.mem.u32(pixel_data + PIXELS));
        let mut row = 0u32;
        for _ in 0..height {
            let mut pixel = row;
            for _ in 0..width {
                for byte in level_bytes(level) {
                    e.mem
                        .set_u8(pixel.wrapping_add(start).wrapping_add(*byte), 0xff);
                }
                pixel = pixel.wrapping_add(4);
            }
            row = row.wrapping_add(width.wrapping_mul(4));
        }
        level += 1;
    }
    let revision = e.mem.u32(pixel_data + REVISION);
    e.mem
        .set_u32(pixel_data + REVISION, revision.wrapping_add(1));
}

/// The calls `00b6bbd0` makes: the format conversion (`00e7bd20`, `cdecl`, the address of a
/// pixel format: a `D3DFORMAT`), the `NiPixelData` constructor (`00a7c190`, `thiscall`: width,
/// height, the address of the format, levels, 1), `D3DXLoadSurfaceFromSurface` (`00ee6de6`,
/// eight words), `memcpy` (`00ec44d0`), `NiSourceTexture::Create` (`00a5ff10`, `cdecl`: the
/// pixel data, a name handle, the format preferences at `011a9598`), the two bytes it sets
/// around it (`011f4448`, kept and cleared, and `011a9595`, "more than one level") and the
/// `D3DFORMAT` codes it handles (`D3DFMT_A8R8G8B8` 0x15, 0x20 handled as 0x15, `DXT1`,
/// `DXT3`, `DXT5`).
const PIXEL_FORMAT_TO_D3D_00E7BD20: u32 = 0x00e7_bd20;
const PIXEL_DATA_CONSTRUCT_00A7C190: u32 = 0x00a7_c190;
const D3DX_LOAD_SURFACE_FROM_SURFACE: u32 = 0x00ee_6de6;
const MEMCPY: u32 = 0x00ec_44d0;
const SOURCE_TEXTURE_CREATE: u32 = 0x00a5_ff10;
const FORMAT_PREFS_011A9598: u32 = 0x011a_9598;
const SWITCH_011F4448: u32 = 0x011f_4448;
const MIPMAPPED_011A9595: u32 = 0x011a_9595;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const D3DFMT_A8R8G8B8: u32 = 0x15;
const D3DFMT_A8B8G8R8: u32 = 0x20;
const D3DFMT_DXT1: u32 = 0x3154_5844;
const D3DFMT_DXT3: u32 = 0x3354_5844;
const D3DFMT_DXT5: u32 = 0x3554_5844;

// Translated from 00b6bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b6bbd0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, `cdecl` with two words (a texture, the address of an
/// `NiPixelFormat`, copied into a 0x44-byte local first): reads a texture back into a new
/// source texture. Null without a texture or renderer (`011f4748`), or when the format's
/// `D3DFORMAT` (`00e7bd20`) is none of 0x15, 0x20 (taken as 0x15), DXT1, DXT3 and DXT5. The
/// width and height (texture slots +0x94, +0x98), the level count (`+0x60` of the texture's
/// pixel data at `+0x3c`, 1 without one) and the renderer data's D3D texture (slot +0x9c of
/// `+0x24`) are read; a system-memory texture of that size and format is made (the device
/// `+0x288` of the renderer, slot +0x5c; null when that fails). A new `NiPixelData` (0x74
/// bytes, `00a7c190`; null when the allocation fails) gets each level: both surfaces (slot
/// +0x48), the copy (`D3DXLoadSurfaceFromSurface`), the lock (slot +0x34) and the bytes:
/// for 0x15 `height * pitch` bytes with bytes 0 and 2 of each pixel swapped, for the DXT
/// formats `max(height, 4) * pitch / 4` bytes; then unlock (+0x38) and both surfaces released
/// (+8). After the loop the system texture is released, and the answer is
/// `NiSourceTexture::Create` of the pixel data with the converted-texture name
/// (`pConvertedTextureName`), the byte at `011f4448` cleared around it and the byte at
/// `011a9595` set to whether there is more than one level.
pub fn fn_00b6bbd0(e: &mut Engine, texture: u32, format: u32) -> u32 {
    let renderer = e.mem.u32(SCENE_GLOBAL_011F4748);
    e.with_stack(0x5c, |e, frame| {
        // frame + 0: the format code, + 4: the system texture, + 8: the source surface,
        // + 0xc: the D3D texture, + 0x10: the pitch and + 0x14 the bits of the locked
        // rectangle, + 0x18: the copied pixel format (0x44 bytes).
        let f = frame.addr();
        let code_at = f;
        let system_at = f + 4;
        let source_at = f + 8;
        let rect = f + 0x10;
        let copy = f + 0x18;
        for k in 0..0x11u32 {
            let v = e.mem.u32(format.wrapping_add(4 * k));
            e.mem.set_u32(copy + 4 * k, v);
        }
        if texture == 0 || renderer == 0 {
            return 0;
        }
        let code = e.call(PIXEL_FORMAT_TO_D3D_00E7BD20, &args![copy]).u32();
        e.mem.set_u32(code_at, code);
        if (code as i32) > (D3DFMT_DXT1 as i32) {
            if code != D3DFMT_DXT3 && code != D3DFMT_DXT5 {
                return 0;
            }
        } else if code != D3DFMT_DXT1 && code != D3DFMT_A8R8G8B8 {
            if code != D3DFMT_A8B8G8R8 {
                return 0;
            }
            e.mem.set_u32(code_at, D3DFMT_A8R8G8B8);
        }
        let device = e.mem.u32(renderer + 0x288);
        let width = e.vcall(texture, 0x94, &args![]).u32();
        let height = e.vcall(texture, 0x98, &args![]).u32();
        let source_pixels = e.mem.u32(texture + 0x3c);
        let levels = if source_pixels != 0 {
            e.mem.u32(source_pixels + LEVELS)
        } else {
            1
        };
        let renderer_data = e.mem.u32(texture + 0x24);
        let d3d_texture = e.vcall(renderer_data, 0x9c, &args![]).u32();
        let code = e.mem.u32(code_at);
        let failed = e
            .vcall(
                device,
                0x5c,
                &args![width, height, levels, 0u32, code, 2u32, system_at, 0u32],
            )
            .u32();
        if failed != 0 || e.mem.u32(system_at) == 0 {
            return 0;
        }
        let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
        let pixel_data = if block != 0 {
            e.call(
                PIXEL_DATA_CONSTRUCT_00A7C190,
                &args![block, width, height, copy, levels, 1u32],
            )
            .u32()
        } else {
            0
        };
        e.with_stack(4, |e, target_at| {
            let mut level = 0u32;
            while level < levels {
                e.vcall(d3d_texture, 0x48, &args![level, source_at]);
                let system = e.mem.u32(system_at);
                e.vcall(system, 0x48, &args![level, target_at]);
                let source = e.mem.u32(source_at);
                let target = e.mem.u32(target_at.addr());
                e.call(
                    D3DX_LOAD_SURFACE_FROM_SURFACE,
                    &args![target, 0u32, 0u32, source, 0u32, 0u32, 0xffff_ffffu32, 0u32],
                );
                let target = e.mem.u32(target_at.addr());
                e.vcall(target, 0x34, &args![rect, 0u32, 0u32]);
                let code = e.mem.u32(code_at);
                let pitch = e.mem.u32(rect);
                let bits = e.mem.u32(rect + 4);
                let at = |e: &Engine, field: u32| {
                    e.mem
                        .u32(e.mem.u32(pixel_data + field).wrapping_add(level * 4))
                };
                if code == D3DFMT_A8R8G8B8 {
                    let size = at(e, HEIGHTS).wrapping_mul(pitch);
                    let dest = at(e, OFFSETS).wrapping_add(e.mem.u32(pixel_data + PIXELS));
                    e.call(MEMCPY, &args![dest, bits, size]);
                    let bytes = at(e, WIDTHS).wrapping_mul(at(e, HEIGHTS)).wrapping_mul(4);
                    let mut pixel = at(e, OFFSETS).wrapping_add(e.mem.u32(pixel_data + PIXELS));
                    if bytes != 0 {
                        let count = (bytes.wrapping_sub(1) >> 2).wrapping_add(1);
                        for _ in 0..count {
                            let low = e.mem.u8(pixel);
                            let high = e.mem.u8(pixel.wrapping_add(2));
                            e.mem.set_u8(pixel, high);
                            e.mem.set_u8(pixel.wrapping_add(2), low);
                            pixel = pixel.wrapping_add(4);
                        }
                    }
                } else if code == D3DFMT_DXT1 || code == D3DFMT_DXT3 || code == D3DFMT_DXT5 {
                    let dest = at(e, OFFSETS).wrapping_add(e.mem.u32(pixel_data + PIXELS));
                    let rows = at(e, HEIGHTS).max(4);
                    let size = rows.wrapping_mul(pitch) >> 2;
                    e.call(MEMCPY, &args![dest, bits, size]);
                }
                let target = e.mem.u32(target_at.addr());
                e.vcall(target, 0x38, &args![]);
                let target = e.mem.u32(target_at.addr());
                e.vcall(target, 8, &args![]);
                let source = e.mem.u32(source_at);
                e.vcall(source, 8, &args![]);
                level += 1;
            }
        });
        let system = e.mem.u32(system_at);
        e.vcall(system, 8, &args![]);
        let kept = e.mem.u8(SWITCH_011F4448);
        let name = e.mem.u32(e.mem.u32(CONVERTED_TEXTURE_NAME));
        e.mem.set_u8(SWITCH_011F4448, 0);
        let created = e
            .call(
                SOURCE_TEXTURE_CREATE,
                &args![pixel_data, name, FORMAT_PREFS_011A9598],
            )
            .u32();
        e.mem.set_u8(MIPMAPPED_011A9595, u8::from(levels > 1));
        e.mem.set_u8(SWITCH_011F4448, kept);
        created
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b6_b910,
            bs_shader_util_recursive_set_property_wireframe(Ptr, u32)
        ),
        entry!(0x0065_03b0, fn_006503b0(Ptr)),
        entry!(0x00b6_b930, fn_00b6b930(u32, Ptr, u32)),
        entry!(0x00b6_ba20, fn_00b6ba20(u32, Ptr, u32)),
        entry!(0x00b6_ba70, bs_shader_util_init_sdm()),
        entry!(
            0x00b6_bb30,
            bs_shader_util_recursive_set_property_fade_alpha(Ptr, f32)
        ),
        entry!(0x00b6_bee0, bs_shader_util_accumulate_scene(u32, u32, Ptr)),
        entry!(
            0x00b6_bfc0,
            bs_shader_util_accumulate_scene_list(u32, Ptr, Ptr)
        ),
        entry!(0x00b6_b940, fn_00b6b940(u32)),
        entry!(0x00b6_bbd0, fn_00b6bbd0(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00b6b910_sets_flag_0x30_recursively() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ba_91a0, |_, _| Ret::default());
        e.call(0x00b6_b910, &args![0x8000u32, 1u32]);
        e.call(0x00b6_b910, &args![0x9000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let sets: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x00ba_91a0)
            .map(|c| &c.1)
            .collect();
        assert_eq!(sets, vec![&vec![0x8000, 0x30, 1], &vec![0x9000, 0x30, 0]]);
    }
}

#[cfg(test)]
mod b048_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// A vtable at `table` whose slot `n` calls the double at `base + 4 * n`.
    fn vtable(e: &mut Engine, table: u32, base: u32, slots: u32) {
        let targets: Vec<u32> = (0..slots).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
    }

    #[test]
    fn fn_006503b0_removes_entries_whose_value_word_is_one() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        // Two values: objects whose +8 points at a block with word +4 = 1 and = 0.
        let removable = e.mem.alloc(0x10);
        let kept = e.mem.alloc(0x10);
        let flag_one = e.mem.alloc(8);
        let flag_zero = e.mem.alloc(8);
        e.mem.set_u32(flag_one + 4, 1);
        e.mem.set_u32(removable + 8, flag_one);
        e.mem.set_u32(kept + 8, flag_zero);
        e.register(MAP_LOCK_ENTER, nop);
        e.register(MAP_LOCK_LEAVE, nop);
        e.register(MAP_FIRST_POSITION, |_, _| 0x77u32.into_ret());
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_DESTRUCT, nop);
        let entries = Rc::new(RefCell::new(vec![
            (0x88u32, 11u32, removable),
            (0, 12, kept),
        ]));
        let feed = entries.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let (next, key, value) = feed.borrow_mut().remove(0);
            e.mem.set_u32(a[1], next);
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            Ret::default()
        });
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(WORD_AT_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(MAP_REMOVE_AT, nop);
        e.call_log = Some(vec![]);
        e.call(0x0065_03b0, &args![this]);
        assert!(entries.borrow().is_empty());
        assert_eq!(calls(&e, MAP_REMOVE_AT), vec![vec![this + 4, 11]]);
        assert_eq!(calls(&e, MAP_FIRST_POSITION), vec![vec![this + 4]]);
        assert_eq!(calls(&e, MAP_LOCK_LEAVE).len(), 1);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 2);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_00b6b930_passes_the_callers_stack_to_virtual_0xac() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        vtable(&mut e, 0x6000, 0x6100_0000, 0x40);
        e.mem.set_u32(object, 0x6000);
        e.register(0x6100_00ac, nop);
        e.call_log = Some(vec![]);
        e.call(0x00b6_b930, &args![0x1234u32, object, 1u32]);
        assert_eq!(
            calls(&e, 0x6100_00ac),
            vec![vec![object, 0x1234, object, 1]]
        );
    }

    #[test]
    fn fn_00b6ba20_needs_the_global_and_the_first_argument() {
        let mut e = Engine::new();
        e.map(SCENE_GLOBAL_011F4748, 4);
        e.map(LAST_OBJECT_011F917C, 4);
        let object = e.mem.alloc(8);
        vtable(&mut e, 0x6000, 0x6100_0000, 0x40);
        e.mem.set_u32(object, 0x6000);
        for a in [
            SCENE_SET_OBJECT_004E9BB0,
            PREPARE_OBJECT_00B54B10,
            0x6100_008c,
            0x6100_00a8,
        ] {
            e.register(a, nop);
        }
        e.call_log = Some(vec![]);
        e.call(0x00b6_ba20, &args![0x1234u32, object, 3u32]);
        e.set_global(SCENE_GLOBAL_011F4748, 0x9000u32);
        e.call(0x00b6_ba20, &args![0u32, object, 3u32]);
        assert!(calls(&e, SCENE_SET_OBJECT_004E9BB0).is_empty());
        e.call(0x00b6_ba20, &args![0x1234u32, object, 3u32]);
        assert_eq!(
            calls(&e, SCENE_SET_OBJECT_004E9BB0),
            vec![vec![0x9000, 0x1234]]
        );
        assert_eq!(e.global::<u32>(LAST_OBJECT_011F917C), 0x1234);
        assert_eq!(calls(&e, PREPARE_OBJECT_00B54B10), vec![vec![object]]);
        assert_eq!(calls(&e, 0x6100_008c), vec![vec![object, 0x1234]]);
        assert_eq!(
            calls(&e, 0x6100_00a8),
            vec![vec![object, 0x1234, object, 3]]
        );
    }

    #[test]
    fn init_sdm_allocates_both_texture_names() {
        let mut e = Engine::new();
        e.map(CONVERTED_TEXTURE_NAME, 8);
        let first = Rc::new(RefCell::new(0u32));
        let seen = first.clone();
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            assert_eq!(a, &[MEMORY_MANAGER, 4]);
            let mut b = seen.borrow_mut();
            // The first allocation succeeds, the second fails.
            let p = if *b == 0 { e.mem.alloc(4) } else { 0 };
            if *b == 0 {
                *b = p;
            }
            p.into_ret()
        });
        e.register(FIXED_STRING_FROM_TEXT, |_, a| (a[0] + 1).into_ret());
        e.set_global(BG_CONVERTED_TEXTURE_NAME, 0x55u32);
        e.call(0x00b6_ba70, &args![]);
        let first = *first.borrow();
        assert_eq!(e.global::<u32>(CONVERTED_TEXTURE_NAME), first);
        assert_eq!(e.mem.u32(first), CONVERTED_TEXTURE_TEXT + 1);
        assert_eq!(e.global::<u32>(BG_CONVERTED_TEXTURE_NAME), 0);
    }

    #[test]
    fn fade_alpha_reaches_geometry_properties_and_recurses() {
        let mut e = Engine::new();
        vtable(&mut e, 0x6000, 0x6100_0000, 0x10);
        // Slot 0x18 answers the object itself when its word +0x40 is 1 (geometry);
        // slot 0x0c answers the word +0x44 (its node, for the recursion).
        e.register(0x6100_0018, |e, a| {
            if e.mem.u32(a[0] + 0x40) == 1 { a[0] } else { 0 }.into_ret()
        });
        e.register(0x6100_000c, |e, a| e.mem.u32(a[0] + 0x44).into_ret());
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(bound + 0xc, 2.0f32.to_bits());
        let make_node = |e: &mut Engine, children: &[u32]| {
            let node = e.mem.alloc(0xb0);
            e.mem.set_u32(node, 0x6000);
            e.mem.set_u32(node + 0x20, bound);
            let array = e.mem.alloc(16);
            for (i, c) in children.iter().enumerate() {
                e.mem.set_u32(array + 4 * i as u32, *c);
            }
            e.mem.set_u32(node + 0xa0, array);
            e.mem.set_u16(node + 0xa6, children.len() as u16);
            node
        };
        let property = e.mem.alloc(0x30);
        let skipped = e.mem.alloc(0x30);
        e.mem.set_u32(skipped + 0x1c, 0xffff_ffff);
        let geometry = e.mem.alloc(0xb0);
        e.mem.set_u32(geometry, 0x6000);
        e.mem.set_u32(geometry + 0x40, 1);
        e.mem.set_u32(geometry + 0xa8, property);
        let other = e.mem.alloc(0xb0);
        e.mem.set_u32(other, 0x6000);
        e.mem.set_u32(other + 0x40, 1);
        e.mem.set_u32(other + 0xa8, skipped);
        let deep = e.mem.alloc(0x30);
        let deep_geometry = e.mem.alloc(0xb0);
        e.mem.set_u32(deep_geometry, 0x6000);
        e.mem.set_u32(deep_geometry + 0x40, 1);
        e.mem.set_u32(deep_geometry + 0xa8, deep);
        let inner = make_node(&mut e, &[other, deep_geometry]);
        let holder = e.mem.alloc(0xb0);
        e.mem.set_u32(holder, 0x6000);
        e.mem.set_u32(holder + 0x44, inner);
        let root = make_node(&mut e, &[geometry, 0, holder]);
        e.call(0x00b6_bb30, &args![root, 0.25f32]);
        assert_eq!(e.mem.u32(property + 0x2c), 0.25f32.to_bits());
        assert_eq!(e.mem.u32(deep + 0x2c), 0.25f32.to_bits());
        assert_eq!(e.mem.u32(skipped + 0x2c), 0);
        // A zero radius (also -0) stops at the root.
        e.mem.set_u32(property + 0x2c, 0);
        e.mem.set_u32(bound + 0xc, 0x8000_0000);
        e.call(0x00b6_bb30, &args![root, 0.5f32]);
        assert_eq!(e.mem.u32(property + 0x2c), 0);
        // A NaN radius does not.
        e.mem.set_u32(bound + 0xc, 0x7fc0_0000);
        e.call(0x00b6_bb30, &args![root, 0.5f32]);
        assert_eq!(e.mem.u32(property + 0x2c), 0.5f32.to_bits());
    }

    fn accumulate_rig(e: &mut Engine) -> (u32, u32) {
        e.map(SCENE_GLOBAL_011F4748, 4);
        let scene = e.mem.alloc(0x10);
        let held = e.mem.alloc(8);
        e.mem.set_u32(held, 0x6000);
        e.mem.set_u32(scene + 8, held);
        e.set_global(SCENE_GLOBAL_011F4748, scene);
        vtable(e, 0x6000, 0x6100_0000, 0x20);
        let process = e.mem.alloc(0xd0);
        e.mem.set_u32(process, 0x6000);
        e.mem.set_u32(process + 0xc4, 0xacc);
        for a in [
            SCENE_SET_ACCUMULATOR_004DC540,
            SHADER_MANAGER_SET_ACCUMULATOR,
            NI_POINTER_DESTRUCT,
            0x6100_0048,
            0x6100_0044,
            0x6100_0004,
        ] {
            e.register(a, nop);
        }
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        (scene, process)
    }

    #[test]
    fn accumulate_scene_skips_the_scene_call_when_the_tls_word_is_set() {
        let mut e = Engine::new();
        let (scene, process) = accumulate_rig(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00b6_bee0, &args![1u32, 0u32, process]);
        assert!(calls(&e, 0x6100_0048).is_empty());
        e.call(0x00b6_bee0, &args![1u32, 2u32, process]);
        assert_eq!(
            calls(&e, SCENE_SET_ACCUMULATOR_004DC540),
            vec![vec![scene, 0xacc]]
        );
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 1);
        assert_eq!(calls(&e, SHADER_MANAGER_SET_ACCUMULATOR), vec![vec![0xacc]]);
        assert_eq!(calls(&e, 0x6100_0048), vec![vec![process, 1, 2, 0]]);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_ACCUMULATE_WORD, 1);
        e.call(0x00b6_bee0, &args![1u32, 2u32, process]);
        assert_eq!(calls(&e, SCENE_SET_ACCUMULATOR_004DC540).len(), 1);
        assert_eq!(calls(&e, 0x6100_0048).len(), 2);
    }

    #[test]
    fn accumulate_scene_list_walks_until_a_null_element() {
        let mut e = Engine::new();
        let (scene, process) = accumulate_rig(&mut e);
        let held = e.mem.u32(scene + 8);
        // Nodes: next at +0, element at +8. Elements 0xa, 0xb, 0xc, then a node with 0.
        let n4 = e.mem.alloc(12);
        let n3 = e.mem.alloc(12);
        let n2 = e.mem.alloc(12);
        let n1 = e.mem.alloc(12);
        e.mem.set_u32(n1, n2);
        e.mem.set_u32(n1 + 8, 0xa);
        e.mem.set_u32(n2, n3);
        e.mem.set_u32(n2 + 8, 0xb);
        e.mem.set_u32(n3, n4);
        e.mem.set_u32(n3 + 8, 0xc);
        let list = e.mem.alloc(12);
        e.mem.set_u32(list, n1);
        e.call_log = Some(vec![]);
        e.call(0x00b6_bfc0, &args![7u32, list, process]);
        assert_eq!(calls(&e, 0x6100_0048), vec![vec![process, 7, 0xa, 0]]);
        assert_eq!(
            calls(&e, 0x6100_0044),
            vec![vec![process, 0xb], vec![process, 0xc]]
        );
        // The reference went up and back down to 0: DeleteThis ran.
        assert_eq!(e.mem.u32(held + 4), 0);
        assert_eq!(calls(&e, 0x6100_0004), vec![vec![held]]);
        // An empty list only sets the accumulator.
        e.mem.set_u32(list, 0);
        e.call(0x00b6_bfc0, &args![7u32, list, process]);
        assert_eq!(calls(&e, 0x6100_0048).len(), 1);
        assert_eq!(calls(&e, SHADER_MANAGER_SET_ACCUMULATOR).len(), 2);
    }
}

#[cfg(test)]
mod n012_tests {
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

    /// A pixel data with `levels` levels of the given sizes, packed one after the other.
    fn pixel_data(e: &mut Engine, sizes: &[(u32, u32)]) -> u32 {
        let this = e.mem.alloc(0x74);
        let n = sizes.len() as u32;
        let widths = e.mem.alloc(4 * n);
        let heights = e.mem.alloc(4 * n);
        let offsets = e.mem.alloc(4 * n);
        let mut total = 0u32;
        for (i, (w, h)) in sizes.iter().enumerate() {
            e.mem.set_u32(widths + 4 * i as u32, *w);
            e.mem.set_u32(heights + 4 * i as u32, *h);
            e.mem.set_u32(offsets + 4 * i as u32, total);
            total += w * h * 4;
        }
        let pixels = e.mem.alloc(total.max(4));
        e.mem.set_u32(this + PIXELS, pixels);
        e.mem.set_u32(this + WIDTHS, widths);
        e.mem.set_u32(this + HEIGHTS, heights);
        e.mem.set_u32(this + OFFSETS, offsets);
        e.mem.set_u32(this + LEVELS, n);
        this
    }

    #[test]
    fn fn_00b6b940_colours_the_mipmaps() {
        let mut e = Engine::new();
        let sizes = [
            (2, 2),
            (2, 1),
            (1, 2),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
        ];
        let data = pixel_data(&mut e, &sizes);
        e.call(0x00b6_b940, &args![data]);
        let pixels = e.mem.u32(data + PIXELS);
        let pixel = |e: &Engine, level: u32, k: u32| {
            let offset = e.mem.u32(e.mem.u32(data + OFFSETS) + 4 * level);
            e.mem.u32(pixels + offset + 4 * k)
        };
        assert_eq!(pixel(&e, 0, 0), 0);
        assert_eq!((pixel(&e, 1, 0), pixel(&e, 1, 1)), (0xff, 0xff));
        assert_eq!((pixel(&e, 2, 0), pixel(&e, 2, 1)), (0xff00, 0xff00));
        assert_eq!(pixel(&e, 3, 0), 0x00ff_0000);
        assert_eq!(pixel(&e, 4, 0), 0xffff);
        assert_eq!(pixel(&e, 5, 0), 0x00ff_ff00);
        assert_eq!(pixel(&e, 6, 0), 0x00ff_00ff);
        assert_eq!(pixel(&e, 7, 0), 0);
        assert_eq!(e.mem.u32(data + REVISION), 1);
        e.call(0x00b6_b940, &args![0u32]);
    }

    /// A COM-style object: a vtable of `count` doubles at `base + 4 * slot` (each answering
    /// 0 unless registered otherwise).
    fn com(e: &mut Engine, base: u32, count: u32) -> u32 {
        let slots: Vec<u32> = (0..count).map(|k| base + 4 * k).collect();
        let table = e.mem.alloc(4 * count);
        for (i, s) in slots.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *s);
            e.register(*s, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0x300);
        e.mem.set_u32(object, table);
        object
    }

    #[test]
    fn fn_00b6bbd0_reads_each_level_back() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_a000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        let format = e.mem.alloc(0x44);
        e.mem.set_u32(format, 0xf0f0);
        // No renderer: null.
        assert_eq!(e.call(0x00b6_bbd0, &args![0x10u32, format]).u32(), 0);
        let device = com(&mut e, 0x0f10_0000, 0x30);
        let renderer = e.mem.alloc(0x290);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(SCENE_GLOBAL_011F4748, renderer);
        let texture = com(&mut e, 0x0f20_0000, 0x30);
        e.register(0x0f20_0000 + 0x94, |_, _| 2u32.into_ret());
        e.register(0x0f20_0000 + 0x98, |_, _| 1u32.into_ret());
        let renderer_data = com(&mut e, 0x0f30_0000, 0x30);
        e.mem.set_u32(texture + 0x24, renderer_data);
        let d3d_texture = com(&mut e, 0x0f40_0000, 0x30);
        e.register_double(0x0f30_0000 + 0x9c, move |_, _| d3d_texture.into_ret());
        let system = com(&mut e, 0x0f50_0000, 0x30);
        e.register_double(0x0f10_0000 + 0x5c, move |e, a| {
            e.mem.set_u32(a[7], system);
            Ret::default()
        });
        let surface = com(&mut e, 0x0f60_0000, 0x30);
        let source = com(&mut e, 0x0f70_0000, 0x30);
        e.register_double(0x0f40_0000 + 0x48, move |e, a| {
            e.mem.set_u32(a[2], source);
            Ret::default()
        });
        e.register_double(0x0f50_0000 + 0x48, move |e, a| {
            e.mem.set_u32(a[2], surface);
            Ret::default()
        });
        let bits = e.mem.alloc(8);
        e.mem.set_u32(bits, 0x4433_2211);
        e.mem.set_u32(bits + 4, 0x8877_6655);
        e.register_double(0x0f60_0000 + 0x34, move |e, a| {
            e.mem.set_u32(a[1], 8);
            e.mem.set_u32(a[1] + 4, bits);
            Ret::default()
        });
        e.register(D3DX_LOAD_SURFACE_FROM_SURFACE, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let made = pixel_data(&mut e, &[(2, 1)]);
        e.register_double(PIXEL_DATA_CONSTRUCT_00A7C190, move |_, _| made.into_ret());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(SOURCE_TEXTURE_CREATE, |e, _| {
            assert_eq!(e.mem.u8(SWITCH_011F4448), 0);
            0x5050u32.into_ret()
        });
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0xaaaa);
        e.mem.set_u32(CONVERTED_TEXTURE_NAME, name);
        e.mem.set_u8(SWITCH_011F4448, 7);
        // An unsupported format: null.
        e.register(PIXEL_FORMAT_TO_D3D_00E7BD20, |_, _| 0x16u32.into_ret());
        assert_eq!(e.call(0x00b6_bbd0, &args![texture, format]).u32(), 0);
        // 0x20 is read as A8R8G8B8: the bytes copied and swapped.
        e.register(PIXEL_FORMAT_TO_D3D_00E7BD20, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0xf0f0);
            0x20u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_bbd0, &args![texture, format]).u32(), 0x5050);
        let created = calls(&e, 0x0f10_0000 + 0x5c);
        assert_eq!(created[0][1..7], [2, 1, 1, 0, 0x15, 2]);
        let pixels = e.mem.u32(made + PIXELS);
        assert_eq!(e.mem.u32(pixels), 0x4411_2233);
        assert_eq!(e.mem.u32(pixels + 4), 0x8855_6677);
        assert_eq!(
            calls(&e, D3DX_LOAD_SURFACE_FROM_SURFACE),
            vec![vec![surface, 0, 0, source, 0, 0, 0xffff_ffff, 0]]
        );
        assert_eq!(
            calls(&e, SOURCE_TEXTURE_CREATE),
            vec![vec![made, 0xaaaa, FORMAT_PREFS_011A9598]]
        );
        assert_eq!(e.mem.u8(SWITCH_011F4448), 7);
        assert_eq!(e.mem.u8(MIPMAPPED_011A9595), 0);
        assert_eq!(calls(&e, 0x0f50_0000 + 8), vec![vec![system]]);
        assert_eq!(calls(&e, 0x0f60_0000 + 0x38), vec![vec![surface]]);
        assert_eq!(calls(&e, 0x0f70_0000 + 8), vec![vec![source]]);
        // DXT1: max(height, 4) * pitch / 4 bytes, no swap.
        e.register(PIXEL_FORMAT_TO_D3D_00E7BD20, |_, _| D3DFMT_DXT1.into_ret());
        e.call(0x00b6_bbd0, &args![texture, format]);
        assert_eq!(calls(&e, MEMCPY).last().unwrap()[2], 8);
        assert_eq!(e.mem.u32(pixels), 0x4433_2211);
    }
}
