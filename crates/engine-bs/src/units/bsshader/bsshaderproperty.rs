//! `BSShader/bsshaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAVObject::GetProperty` (Xbox PDB, `00a59d30`): `thiscall`, `RET 4`; the object's property
/// of the given type, or null.
const NI_AV_OBJECT_GET_PROPERTY: u32 = 0x00a5_9d30;
/// `BSShaderProperty::SetFlag` (Xbox PDB, `00441130`): `thiscall`, `RET 8` (flag index, value
/// byte).
const BS_SHADER_PROPERTY_SET_FLAG: u32 = 0x0044_1130;
/// The virtual slot (byte offset) that answers the object as a geometry, or null (`IsGeometry`).
const SLOT_IS_GEOMETRY: u32 = 0x18;
/// The virtual slot (byte offset) that answers the object as a node, or null (`IsNode`).
const SLOT_IS_NODE: u32 = 0x0c;
/// The property type `NiAVObject::GetProperty` is asked for (the shader property's slot).
const SHADER_PROPERTY_TYPE: u32 = 3;

// Translated from 00ba91a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::SetFlagRecurse` (Xbox PDB): `cdecl`, three stack words (an object, a flag
/// index, a value). For a non-null object: a geometry (virtual slot +0x18 non-null) gets the
/// flag set on its shader property (`GetProperty(3)` then `SetFlag(index, value)`), when it has
/// one; any other object that is a node (slot +0xc) passes the call on to each of its non-null
/// children (the `NiTArray` at node `+0x9c`: buffer at +0xa0, count at +0xa6).
pub fn bs_shader_property_set_flag_recurse(e: &mut Engine, object: Ptr, flag: u32, value: u32) {
    if object.is_null() {
        return;
    }
    if e.vcall(object.addr(), SLOT_IS_GEOMETRY, &args![]).u32() != 0 {
        let property = e
            .call(
                NI_AV_OBJECT_GET_PROPERTY,
                &args![object, SHADER_PROPERTY_TYPE],
            )
            .u32();
        if property != 0 {
            e.call(BS_SHADER_PROPERTY_SET_FLAG, &args![property, flag, value]);
        }
        return;
    }
    let node = e.vcall(object.addr(), SLOT_IS_NODE, &args![]).u32();
    if node != 0 {
        let count = u32::from(e.mem.u16(node.wrapping_add(0xa6)));
        let mut index = 0u32;
        if count != 0 {
            loop {
                let buffer = e.mem.u32(node.wrapping_add(0xa0));
                let child = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
                if child != 0 {
                    bs_shader_property_set_flag_recurse(e, Ptr::new(child), flag, value);
                }
                index = index.wrapping_add(1);
                if index >= count {
                    break;
                }
            }
        }
    }
}

layout! {
    /// `BSShaderProperty::RenderPass` (Xbox PDB), 0x10 bytes; the offsets
    /// match the PC constructor.
    pub struct RenderPass: 0x10 {
        /// `pGeometry` (Xbox PDB).
        0x00 pGeometry: u32,
        /// `usPassEnum` (Xbox PDB).
        0x04 usPassEnum: u16,
        /// `eAccumulationHint` (Xbox PDB).
        0x06 eAccumulationHint: u8,
        /// `bFirstPass` (Xbox PDB).
        0x07 bFirstPass: u8,
        /// `bLastPass` (Xbox PDB).
        0x08 bLastPass: u8,
        /// `ucNumLights` (Xbox PDB).
        0x09 ucNumLights: u8,
        /// `ucMaxNumLights` (Xbox PDB).
        0x0a ucMaxNumLights: u8,
        /// `ucExtraParam` (Xbox PDB).
        0x0b ucExtraParam: u8,
        /// `ppSceneLights` (Xbox PDB).
        0x0c ppSceneLights: u32,
    }
}

/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00ba8ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::RenderPass::RenderPass` (Xbox PDB): `cdecl` with a
/// variable argument list: the pass, its geometry, its pass enum (low half
/// of the word), its first-pass flag (low byte), the light count (low byte)
/// and then that many light words. Clears the hint, last-pass flag, maximum
/// light count, extra parameter and light array; with lights allocates
/// `count * 4` bytes, sets the maximum to the count and copies the light
/// words in. Returns the pass. The exe reads the light words from its
/// caller's stack; here they are the words after the count (a missing word
/// stops loudly).
pub fn bs_shader_property_render_pass_render_pass(
    e: &mut Engine,
    this: Ptr<RenderPass>,
    geometry: u32,
    pass_enum: u16,
    first_pass: u8,
    light_count: u8,
    lights: &[u32],
) -> Ptr<RenderPass> {
    e.set(this, RenderPass::pGeometry, geometry);
    e.set(this, RenderPass::usPassEnum, pass_enum);
    e.set(this, RenderPass::ucNumLights, light_count);
    e.set(this, RenderPass::ucMaxNumLights, 0);
    e.set(this, RenderPass::bFirstPass, first_pass);
    e.set(this, RenderPass::bLastPass, 0);
    e.set(this, RenderPass::ppSceneLights, 0);
    e.set(this, RenderPass::eAccumulationHint, 0);
    if light_count == 0 {
        e.set(this, RenderPass::ucExtraParam, 0);
        return this;
    }
    let count = u32::from(light_count);
    let size = count.checked_mul(4).unwrap_or(u32::MAX);
    let array = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    e.set(this, RenderPass::ppSceneLights, array);
    e.set(this, RenderPass::ucMaxNumLights, light_count);
    // `TEST EDI,EDI; JLE`: never taken (the count is at least 1 here); the
    // branch stores the zero in AL as the extra parameter.
    if (count as i32) <= 0 {
        e.set(this, RenderPass::ucExtraParam, 0);
        return this;
    }
    let mut index = 0u32;
    loop {
        let light = lights[index as usize];
        let array = e.get(this, RenderPass::ppSceneLights);
        e.mem.set_u32(array + index * 4, light);
        index += 1;
        if (index as i32) >= (count as i32) {
            break;
        }
    }
    e.set(this, RenderPass::ucExtraParam, 0);
    this
}

/// The registry entry of [`bs_shader_property_render_pass_render_pass`]:
/// the five fixed words, then the light words.
fn render_pass_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 5,
        "RenderPass::RenderPass needs 5 argument words, got {}",
        words.len()
    );
    bs_shader_property_render_pass_render_pass(
        e,
        Ptr::new(words[0]),
        words[1],
        words[2] as u16,
        words[3] as u8,
        words[4] as u8,
        &words[5..],
    )
    .addr()
    .into_ret()
}

use crate::types::NiTArray;

layout! {
    /// `BSShaderProperty` (Xbox PDB) on PC, 0x60 bytes (the constructor `00ba8e50` and the
    /// derived `TileShaderProperty`, whose own fields start at +0x60, show it): the PDB's
    /// fields up to `fDepthBias` at the same offsets; the PDB's `fLastClearFrame` and
    /// `fLastAccumFrame` (+0x60, +0x64) are not there on PC.
    pub struct BSShaderProperty: 0x60 {
        /// `NiShadeProperty::m_uFlags` (Xbox PDB).
        0x18 m_uFlags: u16,
        /// `NiShadeProperty::iShaderPropertyType` (Xbox PDB).
        0x1c iShaderPropertyType: i32,
        /// `ulFlags[0]` (Xbox PDB).
        0x20 ulFlags0: u32,
        /// `ulFlags[1]` (Xbox PDB).
        0x24 ulFlags1: u32,
        /// `fAlpha` (Xbox PDB).
        0x28 fAlpha: u32,
        /// `fFadeAlpha` (Xbox PDB).
        0x2c fFadeAlpha: u32,
        /// `fEnvmapScale` (Xbox PDB).
        0x30 fEnvmapScale: u32,
        /// `fCameraDistance` (Xbox PDB).
        0x34 fCameraDistance: u32,
        /// `iLastRenderPassState` (Xbox PDB).
        0x38 iLastRenderPassState: i32,
        /// `pRenderPassList` (Xbox PDB).
        0x3c pRenderPassList: u32,
        /// `pDepthMapRenderPassList` (Xbox PDB).
        0x40 pDepthMapRenderPassList: u32,
        /// `pConstAlphaRenderPassList` (Xbox PDB).
        0x44 pConstAlphaRenderPassList: u32,
        /// `pLocalMapRenderPassList` (Xbox PDB).
        0x48 pLocalMapRenderPassList: u32,
        /// `pSIBlockRenderPassList` (Xbox PDB).
        0x4c pSIBlockRenderPassList: u32,
        /// `pWaterFogRenderPassList` (Xbox PDB).
        0x50 pWaterFogRenderPassList: u32,
        /// `pSilhouettePassList` (Xbox PDB).
        0x54 pSilhouettePassList: u32,
        /// `iShader` (Xbox PDB).
        0x58 iShader: i32,
        /// `fDepthBias` (Xbox PDB).
        0x5c fDepthBias: u32,
    }

    /// `BSShaderProperty::RenderPassArray` (Xbox PDB), 0x14 bytes: the
    /// `NiTObjectArray<RenderPass *>` `pPassA` ([`NiTArray`] layout) at +0, then `iSize`
    /// (the number of passes in use).
    pub struct RenderPassArray: 0x14 {
        /// `iSize` (Xbox PDB).
        0x10 iSize: u32,
    }
}

/// The memory manager's deallocation (`00aa4060`, `thiscall`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `InterlockedDecrement` (KERNEL32 import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `InterlockedIncrement` (KERNEL32 import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The vtable `NiTArray<RenderPass *>`'s constructor and the render-pass array install
/// (`010b8480`).
const RENDER_PASS_ARRAY_VTABLE: u32 = 0x010b_8480;
/// The vtable `BSShaderProperty::BSShaderProperty` installs (`010b8488`).
const BS_SHADER_PROPERTY_VTABLE: u32 = 0x010b_8488;
/// The member copy `00a5d7a0` (`thiscall`, two stack words) that `00a81430` starts with.
const COPY_MEMBERS_00A5D7A0: u32 = 0x00a5_d7a0;
/// The base destructor `00a5c240` (`NiMain/niobjectnet.cpp`, `thiscall`).
const BASE_DESTRUCTOR_00A5C240: u32 = 0x00a5_c240;
/// The sized free `00aa1460` (`cdecl`, two words: the block and its size).
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;
/// The base constructor `00a5c200` (`NiMain/niobjectnet.cpp`, `thiscall`).
const BASE_CONSTRUCTOR_00A5C200: u32 = 0x00a5_c200;
/// `NiObjectNET::PostLinkObject` (`00a5c6a0`, `thiscall`, one word: the stream).
const NI_OBJECT_NET_POST_LINK_OBJECT: u32 = 0x00a5_c6a0;
/// The base `LoadBinary` `00a95190` (`thiscall`, one word: the stream).
const BASE_LOAD_BINARY_00A95190: u32 = 0x00a9_5190;
/// The array store `00ba3aa0` (`thiscall`, two stack words: index and the address of the
/// element), which `RenderPassArray::Add` uses to append a new pass.
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
/// The virtual slot (byte offset) of `IsTriBasedGeom` (Xbox PDB slot list).
const SLOT_IS_TRI_BASED_GEOM: u32 = 0x1c;

/// Releases a reference (`InterlockedDecrement` on `+4`; at zero the object's slot +4,
/// `DeleteThis`).
fn release_reference(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00a81430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` with two stack words (a destination and a second
/// word passed along). Calls `00a5d7a0` with both words, then copies `NiShadeProperty`'s
/// `m_uFlags` (`+0x18`) into the destination.
pub fn fn_00a81430(e: &mut Engine, this: Ptr<BSShaderProperty>, dest: u32, extra: u32) {
    e.call(COPY_MEMBERS_00A5D7A0, &args![this, dest, extra]);
    let flags = e.get(this, BSShaderProperty::m_uFlags);
    e.set(
        Ptr::<BSShaderProperty>::new(dest),
        BSShaderProperty::m_uFlags,
        flags,
    );
}

// Translated from 00a81460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShadeProperty::scalar deleting destructor` (engine map, from the vtable): runs the base
/// destructor `00a5c240`, frees the 0x20-byte object (`00aa1460(this, 0x20)`) when bit 0 of
/// the flags is set, and answers `this`.
pub fn ni_shade_property_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BASE_DESTRUCTOR_00A5C240, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x20u32]);
    }
    this
}

// Translated from 00ba8ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::SetAlpha` (engine map; the body matches): stores the new `fAlpha`
/// (`FLD`/`FSTP`: a signalling NaN is stored quiet) and clears `iLastRenderPassState` when
/// the alpha crosses 1.0: the old one at least 1 and the new one below, or the old one below
/// 1 and the new one at least 1 (NaN compares as neither).
pub fn bs_shader_property_set_alpha(e: &mut Engine, this: Ptr<BSShaderProperty>, alpha: u32) {
    let old = f32::from_bits(e.get(this, BSShaderProperty::fAlpha));
    let new = f32::from_bits(alpha);
    let crosses = if old >= 1.0 {
        new < 1.0
    } else if old < 1.0 {
        new >= 1.0
    } else {
        false
    };
    if crosses {
        e.set(this, BSShaderProperty::iLastRenderPassState, 0);
    }
    e.set(this, BSShaderProperty::fAlpha, x87_quiet_bits(alpha));
}

// Translated from 00ba8b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` with two stack words (a destination shader property
/// and a word passed along). Runs [`fn_00a81430`] with both, then copies both flag words,
/// `fAlpha`, `fFadeAlpha`, `iShader` and `fDepthBias` into the destination (the floats
/// through `FLD`/`FSTP`, so a signalling NaN arrives quiet) and clears the destination's
/// `iLastRenderPassState`.
pub fn fn_00ba8b10(e: &mut Engine, this: Ptr<BSShaderProperty>, dest: u32, extra: u32) {
    fn_00a81430(e, this, dest, extra);
    let d = Ptr::<BSShaderProperty>::new(dest);
    let v = e.get(this, BSShaderProperty::ulFlags0);
    e.set(d, BSShaderProperty::ulFlags0, v);
    let v = e.get(this, BSShaderProperty::ulFlags1);
    e.set(d, BSShaderProperty::ulFlags1, v);
    let v = e.get(this, BSShaderProperty::fAlpha);
    e.set(d, BSShaderProperty::fAlpha, x87_quiet_bits(v));
    let v = e.get(this, BSShaderProperty::fFadeAlpha);
    e.set(d, BSShaderProperty::iLastRenderPassState, 0);
    e.set(d, BSShaderProperty::fFadeAlpha, x87_quiet_bits(v));
    let v = e.get(this, BSShaderProperty::iShader);
    e.set(d, BSShaderProperty::iShader, v);
    let v = e.get(this, BSShaderProperty::fDepthBias);
    e.set(d, BSShaderProperty::fDepthBias, x87_quiet_bits(v));
}

// Translated from 00ba8b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<BSShaderProperty::RenderPass *>::NiTArray` (engine map; the body matches):
/// `thiscall` with the capacity and the grow step (the low halves of two words). Sets the
/// vtable `010b8480`, the capacity and grow step, clears the size and the element count, and
/// allocates `capacity * 4` bytes for the elements (none for a zero capacity). Answers
/// `this`.
pub fn ni_t_array_render_pass_ni_t_array(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.mem.set_u32(this.addr(), RENDER_PASS_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    if max_size == 0 {
        e.set(this, NiTArray::m_pBase, 0);
        return this;
    }
    let size = u32::from(max_size).checked_mul(4).unwrap_or(u32::MAX);
    let base = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    e.set(this, NiTArray::m_pBase, base);
    this
}

// Translated from 00ba8be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::PostLinkObject` (engine map; the body matches): `NiObjectNET`'s
/// (`00a5c6a0`) with the stream, then `fDepthBias` becomes 1.0 when bit 26 (`0x4000000`) of
/// the first flag word is set.
pub fn bs_shader_property_post_link_object(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    stream: u32,
) {
    e.call(NI_OBJECT_NET_POST_LINK_OBJECT, &args![this, stream]);
    if e.get(this, BSShaderProperty::ulFlags0) & 0x0400_0000 != 0 {
        e.set(this, BSShaderProperty::fDepthBias, 1.0f32.to_bits());
    }
}

// Translated from 00ba8c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` on a `RenderPass` with one word (a light count, its
/// low byte): allocates `count * 4` bytes and stores them as `ppSceneLights` (the old array
/// is not freed).
pub fn fn_00ba8c00(e: &mut Engine, this: Ptr<RenderPass>, count: u8) {
    let size = u32::from(count).checked_mul(4).unwrap_or(u32::MAX);
    let array = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    e.set(this, RenderPass::ppSceneLights, array);
}

// Translated from 00ba8c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` on a `RenderPass`: frees `ppSceneLights` and
/// clears it, when there is one.
pub fn fn_00ba8c30(e: &mut Engine, this: Ptr<RenderPass>) {
    let array = e.get(this, RenderPass::ppSceneLights);
    if array != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, array]);
        e.set(this, RenderPass::ppSceneLights, 0);
    }
}

// Translated from 00ba8c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::RenderPass::SetLights` (engine map; the body matches): `cdecl` with a
/// variable argument list: the pass, the light count (low byte) and that many light words.
/// A count above `ucMaxNumLights` frees the old light array and allocates `count * 4`
/// bytes, raising the maximum; `ucNumLights` becomes the count and the light words are
/// copied in. The exe reads the light words from its caller's stack; here they are the
/// words after the count (a missing word stops loudly).
pub fn bs_shader_property_render_pass_set_lights(
    e: &mut Engine,
    this: Ptr<RenderPass>,
    count: u8,
    lights: &[u32],
) {
    if count > e.get(this, RenderPass::ucMaxNumLights) {
        let array = e.get(this, RenderPass::ppSceneLights);
        if array != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, array]);
            e.set(this, RenderPass::ppSceneLights, 0);
        }
        let size = u32::from(count).checked_mul(4).unwrap_or(u32::MAX);
        let array = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
        e.set(this, RenderPass::ppSceneLights, array);
        e.set(this, RenderPass::ucMaxNumLights, count);
    }
    e.set(this, RenderPass::ucNumLights, count);
    if count == 0 {
        return;
    }
    let n = i32::from(count);
    // `TEST ECX,ECX; JLE`: never taken (the count is at least 1 here).
    if n <= 0 {
        return;
    }
    let mut index = 0i32;
    loop {
        let light = lights[index as usize];
        let array = e.get(this, RenderPass::ppSceneLights);
        e.mem
            .set_u32(array.wrapping_add((index as u32).wrapping_mul(4)), light);
        index += 1;
        if index >= n {
            break;
        }
    }
}

/// The registry entry of [`bs_shader_property_render_pass_set_lights`]: the pass, the count,
/// then the light words.
fn set_lights_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 2,
        "RenderPass::SetLights needs 2 argument words, got {}",
        words.len()
    );
    bs_shader_property_render_pass_set_lights(e, Ptr::new(words[0]), words[1] as u8, &words[2..]);
    Ret::default()
}

// Translated from 00ba8cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::RenderPass::~RenderPass` (engine map; the body matches): clears
/// `usPassEnum`, frees and clears the light array when there is one, and clears
/// `ucExtraParam`.
pub fn bs_shader_property_render_pass_destructor(e: &mut Engine, this: Ptr<RenderPass>) {
    e.set(this, RenderPass::usPassEnum, 0);
    let array = e.get(this, RenderPass::ppSceneLights);
    if array != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, array]);
        e.set(this, RenderPass::ppSceneLights, 0);
    }
    e.set(this, RenderPass::ucExtraParam, 0);
}

// Translated from 00ba8d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::SetMotionBlurred` (engine map; the body matches): sets (non-zero low
/// byte) or clears bit 4 (`0x10`) of the first flag word; when the bit changes,
/// `iLastRenderPassState` is cleared.
pub fn bs_shader_property_set_motion_blurred(e: &mut Engine, this: Ptr<BSShaderProperty>, on: u8) {
    let flags = e.get(this, BSShaderProperty::ulFlags0);
    if on != 0 {
        if flags & 0x10 == 0 {
            e.set(this, BSShaderProperty::iLastRenderPassState, 0);
        }
        let flags = e.get(this, BSShaderProperty::ulFlags0);
        e.set(this, BSShaderProperty::ulFlags0, flags | 0x10);
    } else {
        if flags & 0x10 != 0 {
            e.set(this, BSShaderProperty::iLastRenderPassState, 0);
        }
        let flags = e.get(this, BSShaderProperty::ulFlags0);
        e.set(this, BSShaderProperty::ulFlags0, flags & 0xffff_ffef);
    }
}

// Translated from 00ba8d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<BSShaderProperty::RenderPass *>::SetSize` (engine map; the body matches):
/// nothing when the word equals the capacity. A size below `m_usSize` clears the slots from
/// the new size up (each non-null one lowers `m_usESize`) and lowers `m_usSize`. The
/// capacity becomes the size (its low half); a zero size leaves no buffer, otherwise a new
/// buffer of `capacity * 4` bytes gets the used slots and zeros after them. The old buffer
/// is freed in every case.
pub fn ni_t_array_render_pass_set_size(e: &mut Engine, this: Ptr<NiTArray>, size: u32) {
    if size == u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        return;
    }
    let size16 = size as u16;
    let used = e.get(this, NiTArray::m_usSize);
    if size < u32::from(used) {
        if size16 < used {
            let mut index = size16;
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                let slot = base.wrapping_add(u32::from(index).wrapping_mul(4));
                if e.mem.u32(slot) != 0 {
                    e.mem.set_u32(slot, 0);
                    let count = e.get(this, NiTArray::m_usESize);
                    e.set(this, NiTArray::m_usESize, count.wrapping_add(0xffff));
                }
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usSize) {
                    break;
                }
            }
        }
        e.set(this, NiTArray::m_usSize, size16);
    }
    let old = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, size16);
    if size == 0 {
        e.set(this, NiTArray::m_pBase, 0);
    } else {
        let bytes = u32::from(size16).checked_mul(4).unwrap_or(u32::MAX);
        let base = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, bytes]).u32();
        e.set(this, NiTArray::m_pBase, base);
        let mut index = 0u16;
        if index < e.get(this, NiTArray::m_usSize) {
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                let offset = u32::from(index) * 4;
                let value = e.mem.u32(old.wrapping_add(offset));
                index = index.wrapping_add(1);
                e.mem.set_u32(base.wrapping_add(offset), value);
                if index >= e.get(this, NiTArray::m_usSize) {
                    break;
                }
            }
        }
        let mut index = e.get(this, NiTArray::m_usSize);
        if index < e.get(this, NiTArray::m_usMaxSize) {
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                e.mem.set_u32(base.wrapping_add(u32::from(index) * 4), 0);
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usMaxSize) {
                    break;
                }
            }
        }
    }
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, old]);
}

// Translated from 00ba8e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::BSShaderProperty` (engine map; the body matches): the base
/// constructor (`00a5c200`), then `m_uFlags` 1, `iShaderPropertyType` -1, the vtable
/// `010b8488`, both flag words, `iLastRenderPassState`, the render-pass lists at +0x3c to
/// +0x50 and `iShader` 0, `fAlpha`, `fFadeAlpha` and `fEnvmapScale` 1.0, `fDepthBias` and
/// `fCameraDistance` 0.0 (`pSilhouettePassList` at +0x54 is not touched). Answers `this`.
pub fn bs_shader_property_bs_shader_property(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
) -> Ptr<BSShaderProperty> {
    e.call(BASE_CONSTRUCTOR_00A5C200, &args![this]);
    let one = 1.0f32.to_bits();
    e.set(this, BSShaderProperty::m_uFlags, 1);
    e.set(this, BSShaderProperty::iShaderPropertyType, -1);
    e.mem.set_u32(this.addr(), BS_SHADER_PROPERTY_VTABLE);
    e.set(this, BSShaderProperty::ulFlags0, 0);
    e.set(this, BSShaderProperty::ulFlags1, 0);
    e.set(this, BSShaderProperty::fAlpha, one);
    e.set(this, BSShaderProperty::fFadeAlpha, one);
    e.set(this, BSShaderProperty::iLastRenderPassState, 0);
    e.set(this, BSShaderProperty::fEnvmapScale, one);
    e.set(this, BSShaderProperty::pRenderPassList, 0);
    e.set(this, BSShaderProperty::pDepthMapRenderPassList, 0);
    e.set(this, BSShaderProperty::fDepthBias, 0);
    e.set(this, BSShaderProperty::pLocalMapRenderPassList, 0);
    e.set(this, BSShaderProperty::pSIBlockRenderPassList, 0);
    e.set(this, BSShaderProperty::fCameraDistance, 0);
    e.set(this, BSShaderProperty::pConstAlphaRenderPassList, 0);
    e.set(this, BSShaderProperty::pWaterFogRenderPassList, 0);
    e.set(this, BSShaderProperty::iShader, 0);
    this
}

// Translated from 00ba8f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; vtable slot +0x98 (`CopyToMembers` in the Xbox PDB's slot
/// list) of the shader-property vtables. `thiscall` with one word, the destination: copies
/// both flag words, `fAlpha`, `fFadeAlpha` (through `FLD`/`FSTP`), `iShader` and
/// `fDepthBias` and clears the destination's `iLastRenderPassState`. When this object has a
/// reference-counted object at `+0xc` (`NiObjectNET::m_spControllers` in the Xbox PDB), the
/// destination takes a reference to it (releasing its own first, unless it is the same) and
/// that object's slot +0x98 is called with the destination.
pub fn fn_00ba8f60(e: &mut Engine, this: Ptr<BSShaderProperty>, dest: u32) {
    let d = Ptr::<BSShaderProperty>::new(dest);
    let v = e.get(this, BSShaderProperty::ulFlags0);
    e.set(d, BSShaderProperty::ulFlags0, v);
    let v = e.get(this, BSShaderProperty::ulFlags1);
    e.set(d, BSShaderProperty::ulFlags1, v);
    let v = e.get(this, BSShaderProperty::fAlpha);
    e.set(d, BSShaderProperty::fAlpha, x87_quiet_bits(v));
    let v = e.get(this, BSShaderProperty::fFadeAlpha);
    e.set(d, BSShaderProperty::iLastRenderPassState, 0);
    e.set(d, BSShaderProperty::fFadeAlpha, x87_quiet_bits(v));
    let v = e.get(this, BSShaderProperty::iShader);
    e.set(d, BSShaderProperty::iShader, v);
    let v = e.get(this, BSShaderProperty::fDepthBias);
    e.set(d, BSShaderProperty::fDepthBias, x87_quiet_bits(v));
    let source = e.mem.u32(this.addr().wrapping_add(0xc));
    if source == 0 {
        return;
    }
    let current = e.mem.u32(dest.wrapping_add(0xc));
    if current != source {
        if current != 0 {
            release_reference(e, current);
        }
        let source = e.mem.u32(this.addr().wrapping_add(0xc));
        e.mem.set_u32(dest.wrapping_add(0xc), source);
        if source != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![source.wrapping_add(4)]);
        }
    }
    let object = e.mem.u32(dest.wrapping_add(0xc));
    e.vcall(object, 0x98, &args![dest]);
}

// Translated from 00ba9120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::SetMotionBlurRecurse` (engine map; the body matches): `cdecl`, an
/// object and a value. For a non-null object: a tri-based geometry (slot +0x1c non-zero)
/// with a shader property (`GetProperty(3)`) gets [`bs_shader_property_set_motion_blurred`]
/// with the value; any other object that is a node (slot +0xc) passes the call on to each
/// of its non-null children (the array at node `+0xa0`, count at `+0xa6`).
pub fn bs_shader_property_set_motion_blur_recurse(e: &mut Engine, object: Ptr, value: u32) {
    if object.is_null() {
        return;
    }
    if e.vcall(object.addr(), SLOT_IS_TRI_BASED_GEOM, &args![])
        .u32()
        != 0
    {
        let property = e
            .call(
                NI_AV_OBJECT_GET_PROPERTY,
                &args![object, SHADER_PROPERTY_TYPE],
            )
            .u32();
        if property != 0 {
            bs_shader_property_set_motion_blurred(e, Ptr::new(property), value as u8);
        }
        return;
    }
    let node = e.vcall(object.addr(), SLOT_IS_NODE, &args![]).u32();
    if node == 0 {
        return;
    }
    let count = u32::from(e.mem.u16(node.wrapping_add(0xa6)));
    let mut index = 0u32;
    if count == 0 {
        return;
    }
    loop {
        let buffer = e.mem.u32(node.wrapping_add(0xa0));
        let child = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
        if child != 0 {
            bs_shader_property_set_motion_blur_recurse(e, Ptr::new(child), value);
        }
        index = index.wrapping_add(1);
        if index >= count {
            break;
        }
    }
}

/// One read through the stream's file (`[stream + 0x24c]`): its read function at file `+8`
/// (`cdecl`: file, buffer, 4, the address of a word holding 4, 1); the answer is added to
/// the file's word at `+4`.
pub(crate) fn stream_read_word(e: &mut Engine, stream: u32, buffer: u32) {
    let file = e.mem.u32(stream.wrapping_add(0x24c));
    let read = e.mem.u32(file.wrapping_add(8));
    let n = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read, &args![file, buffer, 4u32, size, 1u32]).u32()
    });
    let position = e.mem.u32(file.wrapping_add(4));
    e.mem
        .set_u32(file.wrapping_add(4), position.wrapping_add(n));
}

// Translated from 00ba92c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` with one word, a stream (a `LoadBinary`). Runs the
/// base load (`00a95190`), then reads `iShader`, the two flag words and `fEnvmapScale`, one
/// word each through the stream's file; between the flags and the scale, a set bit 13
/// (`0x2000`) of the second flag word also sets bit 15 (`0x8000`) and, when that bit was
/// clear, clears `iLastRenderPassState`.
pub fn fn_00ba92c0(e: &mut Engine, this: Ptr<BSShaderProperty>, stream: u32) {
    e.call(BASE_LOAD_BINARY_00A95190, &args![this, stream]);
    stream_read_word(e, stream, this.addr().wrapping_add(0x58));
    let mut flags = this.addr().wrapping_add(0x20);
    let mut remaining = 2u32;
    loop {
        stream_read_word(e, stream, flags);
        flags = flags.wrapping_add(4);
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
    if e.get(this, BSShaderProperty::ulFlags1) & 0x2000 != 0 {
        if e.get(this, BSShaderProperty::ulFlags1) & 0x8000 == 0 {
            e.set(this, BSShaderProperty::iLastRenderPassState, 0);
        }
        let v = e.get(this, BSShaderProperty::ulFlags1);
        e.set(this, BSShaderProperty::ulFlags1, v | 0x8000);
    }
    stream_read_word(e, stream, this.addr().wrapping_add(0x30));
}

/// `fn_00ba9420`'s globals: two guard bytes, the source object, the two 16-word copies, the
/// flag bytes and the three floats.
const GUARD_0120045C: u32 = 0x0120_045c;
const GUARD_0120045D: u32 = 0x0120_045d;
const SOURCE_011F9508: u32 = 0x011f_9508;
const MATRIX_01200470: u32 = 0x0120_0470;
const MATRIX_012004B0: u32 = 0x0120_04b0;
const FLAG_0120045E: u32 = 0x0120_045e;
const FLAG_011ADCDC: u32 = 0x011a_dcdc;
const FLOATS_012004F0: u32 = 0x0120_04f0;

// Translated from 00ba9420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `cdecl`, one object. Unless one of the bytes at `0120045c`
/// and `0120045d` is set: copies 16 words from `+0x980` and 16 from `+0x9c0` of the object
/// at `011f9508` to `01200470` and `012004b0`, sets the byte at `0120045e` and clears the
/// one at `011adcdc`, and copies the object's floats at `+0x68`, `+0x74` and `+0x80`
/// (through `FLD`/`FSTP`) to `012004f0`, `012004f4` and `012004f8`.
pub fn fn_00ba9420(e: &mut Engine, object: Ptr) {
    if e.mem.u8(GUARD_0120045C) != 0 || e.mem.u8(GUARD_0120045D) != 0 {
        return;
    }
    let source = e.mem.u32(SOURCE_011F9508);
    for i in 0..16u32 {
        let v = e.mem.u32(source.wrapping_add(0x980 + 4 * i));
        e.mem.set_u32(MATRIX_01200470 + 4 * i, v);
    }
    for i in 0..16u32 {
        let v = e.mem.u32(source.wrapping_add(0x9c0 + 4 * i));
        e.mem.set_u32(MATRIX_012004B0 + 4 * i, v);
    }
    e.mem.set_u8(FLAG_0120045E, 1);
    e.mem.set_u8(FLAG_011ADCDC, 0);
    for (i, offset) in [0x68u32, 0x74, 0x80].into_iter().enumerate() {
        let v = x87_quiet_bits(e.mem.u32(object.addr().wrapping_add(offset)));
        e.mem.set_u32(FLOATS_012004F0 + 4 * i as u32, v);
    }
}

// Translated from 00ba94b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::RenderPassArray::RenderPassArray` (engine map; the body matches):
/// constructs the pass array ([`ni_t_array_render_pass_ni_t_array`] with the size and a
/// grow step of 3), sets the vtable `010b8480`, clears `iSize` and sizes the array
/// ([`ni_t_array_render_pass_set_size`]). Answers `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba94b0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn bs_shader_property_render_pass_array_render_pass_array(
    e: &mut Engine,
    this: Ptr<RenderPassArray>,
    size: u32,
) -> Ptr<RenderPassArray> {
    let array = this.cast::<NiTArray>();
    ni_t_array_render_pass_ni_t_array(e, array, size as u16, 3);
    e.mem.set_u32(this.addr(), RENDER_PASS_ARRAY_VTABLE);
    e.set(this, RenderPassArray::iSize, 0);
    ni_t_array_render_pass_set_size(e, array, size);
    this
}

// Translated from 00ba9520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the render-pass array's destructor
/// ([`bs_shader_property_free_render_passes`] runs it before freeing each 0x14-byte array).
/// For each slot below `m_usESize` (+0xc) that holds a pass, runs the pass's destructor
/// inline (`usPassEnum` 0, light array freed, `ucExtraParam` 0) and frees the pass; then
/// stores `0x00ffdead` in `iSize`, sets the vtable `010b8480` and frees the buffer.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba9520 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00ba9520(e: &mut Engine, this: Ptr<RenderPassArray>) {
    let array = this.cast::<NiTArray>();
    let mut index = 0u32;
    if e.get(array, NiTArray::m_usESize) != 0 {
        loop {
            let base = e.get(array, NiTArray::m_pBase);
            let pass = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if pass != 0 {
                let pass = Ptr::<RenderPass>::new(pass);
                let lights = e.get(pass, RenderPass::ppSceneLights);
                e.set(pass, RenderPass::usPassEnum, 0);
                if lights != 0 {
                    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, lights]);
                    e.set(pass, RenderPass::ppSceneLights, 0);
                }
                e.set(pass, RenderPass::ucExtraParam, 0);
                e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, pass]);
            }
            index = index.wrapping_add(1);
            if index >= u32::from(e.get(array, NiTArray::m_usESize)) {
                break;
            }
        }
    }
    let base = e.get(array, NiTArray::m_pBase);
    e.set(this, RenderPassArray::iSize, 0x00ff_dead);
    e.mem.set_u32(this.addr(), RENDER_PASS_ARRAY_VTABLE);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, base]);
}

/// Diagnostic only, not exe behaviour (a22, the p185/p191 race-menu stops:
/// `RenderPassArray::Add` reached a null pass in a slot below `m_usESize`, a hole the exe's
/// own pass-array code never leaves). Without `NV_PASS_CHECK` nothing is checked or watched.
///
/// With it, every render-pass array `Add` uses and every one `CheckCreateRenderPassArray`
/// keeps is checked ([`fault`]: a live 0x18 block with the vtable `010b8480`, a live element
/// buffer of at least `m_usMaxSize` words, `m_usESize <= m_usSize <= m_usMaxSize`, exactly
/// `m_usESize` slots holding a pass, none of them below `m_usESize` empty, each pass a live
/// 0x10 block) and from then on watched through the memory write watch: the array's 0x14
/// bytes (all but `iSize`), its element buffer, and the freeing of each listed pass. Every
/// change is kept with the calls it happened in (the last 32 per array), and at the next
/// call or return after a change the array is checked again, so a run stops right after
/// the call that broke it, naming it. Frees inside `FreeRenderPasses` (`00baa000`) and the
/// array destructor (`00ba9520`) are the arrays' own teardown and are not stops.
pub(crate) mod pass_check {
    use super::RENDER_PASS_ARRAY_VTABLE;
    use crate::mem::{WatchHit, WatchKind};
    use crate::prelude::*;
    use std::collections::{HashMap, VecDeque};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

    /// `Engine::call_hook`'s type.
    type CallHook = fn(&mut Engine, u32, bool);

    /// The arrays' own teardown: `RenderPassArray`'s destructor and `FreeRenderPasses`.
    const ARRAY_DESTRUCTOR_00BA9520: u32 = 0x00ba_9520;
    const FREE_RENDER_PASSES_00BAA000: u32 = 0x00ba_a000;
    /// Events kept per array, and calls named per event.
    const KEEP: usize = 32;
    /// The array functions whose inside may pass through half-done states (`Add`, `SetSize`,
    /// `SetAt`, the constructors, the `CheckCreate` family): an array they change is
    /// checked once none of them is running.
    const ARRAY_FUNCTIONS: [u32; 11] = [
        0x00ba_9ee0,
        0x00ba_8d30,
        0x00ba_3aa0,
        0x00ba_94b0,
        0x00ba_8b80,
        0x00ba_95e0,
        0x00ba_9660,
        0x00ba_9740,
        0x00ba_96d0,
        0x00ba_97b0,
        0x00ba_9820,
    ];
    const PATH_DEPTH: usize = 8;

    fn on() -> bool {
        static ON: OnceLock<bool> = OnceLock::new();
        *ON.get_or_init(|| std::env::var_os("NV_PASS_CHECK").is_some())
    }

    /// What a watched range is to its array.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Role {
        Header,
        Buffer,
        /// A listed pass (frees only), with the slot it was in.
        Pass(u32),
    }

    /// What is watched for one array.
    #[derive(Default)]
    struct Tracked {
        base: u32,
        buffer_len: u32,
        passes: Vec<u32>,
    }

    #[derive(Default)]
    struct State {
        arrays: HashMap<u32, Tracked>,
        /// Watched range start -> (array, role).
        owner: HashMap<u32, (u32, Role)>,
        events: HashMap<u32, VecDeque<String>>,
        seq: u64,
        /// Arrays changed since they were last checked.
        pending: Vec<u32>,
    }

    fn state() -> MutexGuard<'static, State> {
        static STATE: OnceLock<Mutex<State>> = OnceLock::new();
        STATE
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    static PREVIOUS: Mutex<Option<CallHook>> = Mutex::new(None);
    static HAS_PREVIOUS: AtomicBool = AtomicBool::new(false);
    static IN_HOOK: AtomicBool = AtomicBool::new(false);
    /// Whether arrays wait for their check (`State::pending`).
    static PENDING: AtomicBool = AtomicBool::new(false);

    /// What is wrong with the array at `array`, or `None` when it is sound.
    pub(crate) fn fault(e: &Engine, array: u32) -> Option<String> {
        match e.mem.block_size(array) {
            Some(0x18) => {}
            Some(size) => {
                return Some(format!(
                    "the array is a live block of {size:#x} bytes, not 0x18"
                ))
            }
            None => {
                return Some("the array is not a live heap block (freed or never allocated)".into())
            }
        }
        let vtable = e.mem.u32(array);
        if vtable != RENDER_PASS_ARRAY_VTABLE {
            return Some(format!("the array's vtable is {vtable:08x}, not 010b8480"));
        }
        let base = e.mem.u32(array + 4);
        let max = u32::from(e.mem.u16(array + 8));
        let size = u32::from(e.mem.u16(array + 0xa));
        let used = u32::from(e.mem.u16(array + 0xc));
        if base != 0 || max != 0 {
            match e.mem.block_size(base) {
                Some(bytes) if bytes >= max * 4 => {}
                Some(bytes) => {
                    return Some(format!(
                        "the element buffer {base:08x} is {bytes:#x} bytes for {max} slots"
                    ));
                }
                None => {
                    return Some(format!(
                        "the element buffer {base:08x} is not a live heap block"
                    ));
                }
            }
        }
        if used > size || size > max {
            return Some(format!(
                "the counts are out of order (ESize {used}, Size {size}, MaxSize {max})"
            ));
        }
        let slot = |i: u32| e.mem.u32(base.wrapping_add(i * 4));
        let held = (0..size).filter(|i| slot(*i) != 0).count() as u32;
        if held != used {
            return Some(format!(
                "{held} of the {size} slots hold a pass but m_usESize is {used}"
            ));
        }
        // `Add` reuses slots below `m_usESize` in order, so they must all hold a pass.
        if let Some(hole) = (0..used).find(|i| slot(*i) == 0) {
            return Some(format!("slot {hole} below m_usESize {used} is null"));
        }
        if let Some(i) =
            (0..size).find(|i| slot(*i) != 0 && e.mem.block_size(slot(*i)) != Some(0x10))
        {
            return Some(format!(
                "slot {i}'s pass {:08x} is not a live 0x10 block",
                slot(i)
            ));
        }
        None
    }

    /// The array's five words, its block sizes and its first slots, then (with the check
    /// on) the changes kept for it, for a stop message.
    pub(crate) fn describe(e: &Engine, array: u32) -> String {
        let mut text = describe_words(e, array);
        if on() {
            text += &events_text(&state(), array);
        }
        text
    }

    fn events_text(st: &State, array: u32) -> String {
        match st.events.get(&array) {
            Some(list) if !list.is_empty() => {
                let lines: Vec<&str> = list.iter().map(String::as_str).collect();
                format!("; changes seen (oldest first): {}", lines.join(" | "))
            }
            _ => "; no changes seen since it was first checked".into(),
        }
    }

    /// With `NV_PASS_CHECK`: stops at an unsound array (`owner`: its property, or 0), else
    /// watches it from now on.
    pub(crate) fn check(e: &mut Engine, array: u32, at: &str, owner: u32) {
        if on() && array != 0 {
            check_now(e, array, at, owner);
        }
    }

    /// [`check`] whether or not the variable is set (tests).
    pub(crate) fn check_now(e: &mut Engine, array: u32, at: &str, owner: u32) {
        let mut st = state();
        install(e);
        if let Some(why) = fault(e, array) {
            let events = events_text(&st, array);
            drop(st);
            let mut text = describe_words(e, array);
            text += &events;
            panic!(
                "pass check: render-pass array {at} (property {owner:08x}): {why}; {text} (NV_PASS_CHECK)"
            );
        }
        track(e, &mut st, array);
    }

    /// [`describe`] without the changes (for callers that hold the state).
    fn describe_words(e: &Engine, array: u32) -> String {
        let live = |a: u32| match e.mem.block_size(a) {
            Some(size) => format!("live {size:#x}"),
            None => "not live".to_string(),
        };
        let mut text = format!("array {array:08x} ({})", live(array));
        if !e.mem.is_mapped(array) || !e.mem.is_mapped(array.wrapping_add(0x13)) {
            return text + " unmapped";
        }
        let words: Vec<String> = (0..5)
            .map(|i| format!("{:08x}", e.mem.u32(array + i * 4)))
            .collect();
        text += &format!(" words [{}]", words.join(" "));
        let base = e.mem.u32(array + 4);
        let size = u32::from(e.mem.u16(array + 0xa)).min(8);
        text += &format!(" buffer {base:08x} ({})", live(base));
        if base != 0 && e.mem.is_mapped(base) {
            let slots: Vec<String> = (0..size)
                .filter(|i| e.mem.is_mapped(base.wrapping_add(i * 4)))
                .map(|i| format!("{:08x}", e.mem.u32(base + i * 4)))
                .collect();
            text += &format!(" slots [{}]", slots.join(" "));
        }
        text
    }

    /// Takes the call hook (calling on to one that was there first) the first time.
    fn install(e: &mut Engine) {
        if e.call_hook
            .is_some_and(|h| h as usize == on_call as CallHook as usize)
        {
            return;
        }
        let previous = e
            .call_hook
            .filter(|h| *h as usize != on_call as CallHook as usize);
        *PREVIOUS.lock().unwrap_or_else(PoisonError::into_inner) = previous;
        HAS_PREVIOUS.store(previous.is_some(), Ordering::Relaxed);
        e.call_hook = Some(on_call);
    }

    /// Watches the array's header, its current element buffer and the freeing of its passes
    /// (dropping what it no longer uses).
    fn track(e: &mut Engine, st: &mut State, array: u32) {
        let base = e.mem.u32(array + 4);
        let max = u32::from(e.mem.u16(array + 8));
        let size = u32::from(e.mem.u16(array + 0xa));
        let mut t = st.arrays.remove(&array).unwrap_or_default();
        if st.owner.get(&array) != Some(&(array, Role::Header)) {
            e.mem.watch(array, 0x14, false);
            st.owner.insert(array, (array, Role::Header));
        }
        if t.base != base || t.buffer_len != max * 4 {
            if t.base != 0 && st.owner.get(&t.base) == Some(&(array, Role::Buffer)) {
                e.mem.unwatch(t.base);
                st.owner.remove(&t.base);
            }
            if base != 0 && max != 0 {
                e.mem.watch(base, max * 4, false);
                st.owner.insert(base, (array, Role::Buffer));
            }
            t.base = base;
            t.buffer_len = max * 4;
        }
        let passes: Vec<u32> = if base != 0 && e.mem.block_size(base).is_some() {
            (0..size).map(|i| e.mem.u32(base + i * 4)).collect()
        } else {
            Vec::new()
        };
        if passes != t.passes {
            for &p in &t.passes {
                if p != 0
                    && !passes.contains(&p)
                    && matches!(st.owner.get(&p), Some((a, Role::Pass(_))) if *a == array)
                {
                    e.mem.unwatch(p);
                    st.owner.remove(&p);
                }
            }
            for (i, &p) in passes.iter().enumerate() {
                if p != 0 {
                    e.mem.watch(p, 0x10, true);
                    st.owner.insert(p, (array, Role::Pass(i as u32)));
                }
            }
            t.passes = passes;
        }
        st.arrays.insert(array, t);
    }

    /// Stops watching an array (its header was freed).
    fn untrack(e: &mut Engine, st: &mut State, array: u32) {
        let Some(t) = st.arrays.remove(&array) else {
            return;
        };
        let mut starts = vec![array, t.base];
        starts.extend(t.passes.iter().copied());
        for start in starts {
            if start != 0 && st.owner.get(&start).is_some_and(|(a, _)| *a == array) {
                e.mem.unwatch(start);
                st.owner.remove(&start);
            }
        }
    }

    /// The header field at byte `offset`.
    fn field(offset: u32) -> &'static str {
        match offset {
            0..=3 => "vtable",
            4..=7 => "m_pBase",
            8 | 9 => "m_usMaxSize",
            0xa | 0xb => "m_usSize",
            0xc | 0xd => "m_usESize",
            0xe | 0xf => "m_usGrowBy",
            _ => "iSize",
        }
    }

    /// Files the events and answers a stop message when one broke an array.
    fn process(
        e: &mut Engine,
        st: &mut State,
        hits: &[WatchHit],
        path: &str,
        teardown: bool,
        busy: bool,
    ) -> Option<String> {
        let mut touched = std::mem::take(&mut st.pending);
        let mut stop: Option<(u32, String)> = None;
        for hit in hits {
            let Some(&(array, role)) = st.owner.get(&hit.start) else {
                continue;
            };
            let offset = hit.addr - hit.start;
            if role == Role::Header
                && matches!(hit.kind, WatchKind::Write | WatchKind::Zero)
                && offset >= 0x10
            {
                continue; // `iSize`, rewritten every frame.
            }
            let what = match role {
                Role::Header => format!("{} (+{offset:x})", field(offset)),
                Role::Buffer => format!("buffer {:08x} slot {}", hit.start, offset / 4),
                Role::Pass(slot) => format!("pass {:08x} (slot {slot})", hit.start),
            };
            let width = (hit.len.min(8) * 2) as usize;
            st.seq += 1;
            let text = format!(
                "#{} {:?} {what}: {:0width$x} -> {:0width$x} in {path}",
                st.seq, hit.kind, hit.old, hit.new
            );
            let list = st.events.entry(array).or_default();
            if list.len() >= KEEP {
                list.pop_front();
            }
            list.push_back(text);
            match (role, hit.kind) {
                (Role::Header, WatchKind::Free) => {
                    untrack(e, st, array);
                    continue;
                }
                (Role::Pass(_), WatchKind::Free) if !teardown => {
                    let base = e.mem.u32(array + 4);
                    let size = u32::from(e.mem.u16(array + 0xa));
                    let listed = st.arrays.contains_key(&array)
                        && e.mem.block_size(base).is_some()
                        && (0..size).any(|i| e.mem.u32(base + i * 4) == hit.start);
                    if listed && stop.is_none() {
                        stop = Some((
                            array,
                            format!(
                                "pass {:08x} was freed while array {array:08x} still lists it",
                                hit.start
                            ),
                        ));
                    }
                }
                (Role::Buffer, WatchKind::Free) if !teardown => {
                    if st.arrays.contains_key(&array)
                        && e.mem.u32(array + 4) == hit.start
                        && stop.is_none()
                    {
                        stop = Some((
                            array,
                            format!("element buffer {:08x} was freed while array {array:08x} still uses it", hit.start),
                        ));
                    }
                }
                _ => {}
            }
            if !touched.contains(&array) {
                touched.push(array);
            }
        }
        if stop.is_none() && (teardown || busy) {
            st.pending = touched;
            return None;
        }
        if stop.is_none() {
            for &array in &touched {
                if !st.arrays.contains_key(&array) {
                    continue;
                }
                if let Some(why) = fault(e, array) {
                    stop = Some((array, format!("array {array:08x} broke: {why}")));
                    break;
                }
                track(e, st, array);
            }
        }
        stop.map(|(array, why)| {
            format!(
                "pass check: {why}, right after the change in {path}; {}{} (NV_PASS_CHECK)",
                describe_words(e, array),
                events_text(st, array)
            )
        })
    }

    /// `Engine::call_hook`: files the watched ranges' changes with the calls they happened in
    /// (the function returning, or the one calling, and its callers) and stops at a broken
    /// array; then calls on to the hook that was there first.
    fn on_call(e: &mut Engine, addr: u32, exit: bool) {
        if (e.mem.has_watch_hits() || PENDING.load(Ordering::Relaxed))
            && !IN_HOOK.swap(true, Ordering::Acquire)
        {
            let hits = e.mem.take_watch_hits();
            let mut frames: Vec<u32> = Vec::with_capacity(PATH_DEPTH);
            if exit {
                frames.push(addr);
            }
            frames.extend(e.call_stack.iter().rev().take(PATH_DEPTH));
            let teardown = frames
                .iter()
                .chain(e.call_stack.iter())
                .any(|a| *a == ARRAY_DESTRUCTOR_00BA9520 || *a == FREE_RENDER_PASSES_00BAA000);
            let busy = e.call_stack.iter().any(|a| ARRAY_FUNCTIONS.contains(a));
            frames.truncate(PATH_DEPTH);
            let path: Vec<String> = frames.iter().map(|a| format!("{a:08x}")).collect();
            let path = path.join(" < ");
            let stop = {
                let mut st = state();
                let stop = process(e, &mut st, &hits, &path, teardown, busy);
                PENDING.store(!st.pending.is_empty(), Ordering::Relaxed);
                stop
            };
            IN_HOOK.store(false, Ordering::Release);
            if let Some(text) = stop {
                panic!("{text}");
            }
        }
        if HAS_PREVIOUS.load(Ordering::Relaxed) {
            let previous = *PREVIOUS.lock().unwrap_or_else(PoisonError::into_inner);
            if let Some(hook) = previous {
                hook(e, addr, exit);
            }
        }
    }
}

/// Allocates a 0x14-byte render-pass array and constructs it with `size` (null when the
/// allocation fails).
fn new_render_pass_array(e: &mut Engine, size: u32) -> u32 {
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x14]).u32();
    if block == 0 {
        return 0;
    }
    bs_shader_property_render_pass_array_render_pass_array(e, Ptr::new(block), size).addr()
}

// Translated from 00ba95e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::CheckCreateRenderPassArray` (engine map; the body matches): when
/// `pRenderPassList` is null, makes a new render-pass array of the given size (-1 means 5).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba95e0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn bs_shader_property_check_create_render_pass_array(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    size: i32,
) {
    let existing = e.get(this, BSShaderProperty::pRenderPassList);
    if existing != 0 {
        pass_check::check(
            e,
            existing,
            "kept by CheckCreateRenderPassArray",
            this.addr(),
        );
        return;
    }
    let size = if size == -1 { 5 } else { size };
    let array = new_render_pass_array(e, size as u32);
    e.set(this, BSShaderProperty::pRenderPassList, array);
}

// Translated from 00ba9660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::CheckCreateDepthMapRenderPassArray` (engine map; the body matches):
/// when `pDepthMapRenderPassList` is null, makes a new render-pass array of the given size.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba9660 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn bs_shader_property_check_create_depth_map_render_pass_array(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    size: u32,
) {
    if e.get(this, BSShaderProperty::pDepthMapRenderPassList) != 0 {
        return;
    }
    let array = new_render_pass_array(e, size);
    e.set(this, BSShaderProperty::pDepthMapRenderPassList, array);
}

// Translated from 00ba9ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::RenderPassArray::Add` (engine map; the body matches): `thiscall`, eight
/// words (geometry, pass enum, first-pass flag, light count, four light words). While
/// `iSize` is below `m_usESize` the pass in slot `iSize` is reused: its lights set
/// ([`bs_shader_property_render_pass_set_lights`] with the count and the four words), its
/// pass enum, first-pass flag and geometry stored and its last-pass flag, accumulation hint
/// and extra parameter cleared. Otherwise a new 0x10-byte pass is made
/// ([`bs_shader_property_render_pass_render_pass`] with all eight words; null when the
/// allocation fails), the array grows by `m_usGrowBy` when it is full, and the pass is
/// stored at index `m_usSize` (`00ba3aa0`). `iSize` goes up by one.
///
/// The light words are the four argument words; a light count above 4 makes the callee
/// (`cdecl`, variable arguments) read the words above the four copies the exe pushes: this
/// function's own frame (the cookie word, the saved `EDI`/`ESI`, a local, the exception
/// record, its return address, then its arguments again). The translation has no such frame and stops
/// loudly instead. The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba9ee0 - the FS:[0] exception frame (unwinding on a throw) is not translated; a light count above 4 reads the exe's own stack frame past the four light words (stops loudly here)
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_property_render_pass_array_add(
    e: &mut Engine,
    this: Ptr<RenderPassArray>,
    geometry: u32,
    pass_enum: u32,
    first_pass: u32,
    light_count: u32,
    light0: u32,
    light1: u32,
    light2: u32,
    light3: u32,
) {
    let array = this.cast::<NiTArray>();
    let lights = [light0, light1, light2, light3];
    pass_check::check(e, this.addr(), "at RenderPassArray::Add", 0);
    let in_use = e.get(this, RenderPassArray::iSize);
    if in_use < u32::from(e.get(array, NiTArray::m_usESize)) {
        let base = e.get(array, NiTArray::m_pBase);
        let pass = Ptr::<RenderPass>::new(e.mem.u32(base.wrapping_add(in_use.wrapping_mul(4))));
        if pass.addr() == 0 {
            // Not exe behaviour: the exe faults on the next read (the null pass's +0xa); the
            // stop says why (the p185 race-menu stop).
            panic!(
                "pass check: RenderPassArray::Add reuses slot {in_use} (below m_usESize) of \
                 {:08x} but the slot is null; {}",
                this.addr(),
                pass_check::describe(e, this.addr())
            );
        }
        bs_shader_property_render_pass_set_lights(e, pass, light_count as u8, &lights);
        e.set(pass, RenderPass::usPassEnum, pass_enum as u16);
        e.set(pass, RenderPass::bFirstPass, first_pass as u8);
        e.set(pass, RenderPass::pGeometry, geometry);
        e.set(pass, RenderPass::bLastPass, 0);
        e.set(pass, RenderPass::eAccumulationHint, 0);
        e.set(pass, RenderPass::ucExtraParam, 0);
    } else {
        let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x10]).u32();
        let pass = if block != 0 {
            bs_shader_property_render_pass_render_pass(
                e,
                Ptr::new(block),
                geometry,
                pass_enum as u16,
                first_pass as u8,
                light_count as u8,
                &lights,
            )
            .addr()
        } else {
            0
        };
        let used = u32::from(e.get(array, NiTArray::m_usSize));
        if used >= u32::from(e.get(array, NiTArray::m_usMaxSize)) {
            let grow = u32::from(e.get(array, NiTArray::m_usGrowBy));
            ni_t_array_render_pass_set_size(e, array, grow.wrapping_add(used));
        }
        e.with_stack(4, |e, element| {
            e.mem.set_u32(element.addr(), pass);
            e.call(ARRAY_SET_AT_00BA3AA0, &args![this, used, element]);
        });
    }
    let in_use = e.get(this, RenderPassArray::iSize);
    e.set(this, RenderPassArray::iSize, in_use.wrapping_add(1));
}

// Translated from 00baa000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::FreeRenderPasses` (engine map; the body matches): clears
/// `iLastRenderPassState`, then destroys ([`fn_00ba9520`]), frees and clears each of the
/// seven render-pass lists that exists, in the order +0x3c, +0x40, +0x48, +0x4c, +0x44,
/// +0x50, +0x54.
pub fn bs_shader_property_free_render_passes(e: &mut Engine, this: Ptr<BSShaderProperty>) {
    e.set(this, BSShaderProperty::iLastRenderPassState, 0);
    for offset in [0x3cu32, 0x40, 0x48, 0x4c, 0x44, 0x50, 0x54] {
        let slot = this.addr().wrapping_add(offset);
        let list = e.mem.u32(slot);
        if list != 0 {
            fn_00ba9520(e, Ptr::new(list));
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, list]);
            e.mem.set_u32(slot, 0);
        }
    }
}

/// `NiShadeProperty`'s run-time type record (`011f5ae0`), which `00a81450` answers.
const NI_SHADE_PROPERTY_RTTI: u32 = 0x011f_5ae0;
/// `BSShaderProperty`'s run-time type record (`01200464`); its first word is the class name.
const BS_SHADER_PROPERTY_RTTI: u32 = 0x0120_0464;
/// The object allocation `00aa13e0` (`cdecl`, one word: the size).
const NEW_OBJECT_00AA13E0: u32 = 0x00aa_13e0;
/// The base `SaveBinary` `00a815c0` (`thiscall`, one word: the stream).
const BASE_SAVE_BINARY_00A815C0: u32 = 0x00a8_15c0;
/// `NiShadeProperty::GetViewerStrings` (`00a81600`, `thiscall`, one word: the strings).
const NI_SHADE_PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00a8_1600;
/// `BSShaderManager::GetPassName` (`00b4f9d0`, `cdecl`, one word: the pass).
const GET_PASS_NAME: u32 = 0x00b4_f9d0;
/// The viewer-string makers (`NiGetViewerString` and its overloads, `cdecl`): of a string
/// (`00a7d330`), of a string and a flag (`00a7d370`), of a name and a signed integer
/// (`00a7d4d0`), an unsigned integer (`00a7d510`), a `float` (`00a7d550`) or a string
/// (`00a7d5e0`). Each answers the new string.
pub(crate) const VIEWER_STRING: u32 = 0x00a7_d330;
pub(crate) const VIEWER_STRING_FLAG: u32 = 0x00a7_d370;
pub(crate) const VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
pub(crate) const VIEWER_STRING_UINT: u32 = 0x00a7_d510;
pub(crate) const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
pub(crate) const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
/// The string array's `SetSize` (`0096ad30`, `thiscall`, one word) and `SetAt` (`0096ae90`,
/// `thiscall`, two words: index and the address of the element).
pub(crate) const STRINGS_SET_SIZE: u32 = 0x0096_ad30;
pub(crate) const STRINGS_SET_AT: u32 = 0x0096_ae90;

/// The `NiTArray<char *>` add the viewer-string functions inline: the index is `m_usSize`;
/// when it is not below `m_usMaxSize` the array is resized to `m_usGrowBy` more than it
/// (`0096ad30`), then the string is stored there through a local word (`0096ae90`).
pub(crate) fn viewer_strings_add(e: &mut Engine, strings: u32, string: u32) {
    let array = Ptr::<NiTArray>::new(strings);
    let index = u32::from(e.get(array, NiTArray::m_usSize));
    if index >= u32::from(e.get(array, NiTArray::m_usMaxSize)) {
        let grow = u32::from(e.get(array, NiTArray::m_usGrowBy));
        e.call(STRINGS_SET_SIZE, &args![strings, grow.wrapping_add(index)]);
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), string);
        e.call(STRINGS_SET_AT, &args![strings, index, local]);
    });
}

/// The same add with `SetAt` inlined too, as some of the viewer-string functions have it:
/// after the resize, an index at or past `m_usSize` makes `m_usSize` one more than it and a
/// non-null string counts in `m_usESize`; an index below it counts a non-null string over a
/// null slot up and a null string over a non-null slot down. Then the slot is written.
pub(crate) fn viewer_strings_add_inline(e: &mut Engine, strings: u32, string: u32) {
    let array = Ptr::<NiTArray>::new(strings);
    let index = u32::from(e.get(array, NiTArray::m_usSize));
    if index >= u32::from(e.get(array, NiTArray::m_usMaxSize)) {
        let grow = u32::from(e.get(array, NiTArray::m_usGrowBy));
        e.call(STRINGS_SET_SIZE, &args![strings, grow.wrapping_add(index)]);
    }
    if index >= u32::from(e.get(array, NiTArray::m_usSize)) {
        e.set(array, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if string != 0 {
            let used = e.get(array, NiTArray::m_usESize);
            e.set(array, NiTArray::m_usESize, used.wrapping_add(1));
        }
    } else {
        let base = e.get(array, NiTArray::m_pBase);
        let old = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        if string != 0 {
            if old == 0 {
                let used = e.get(array, NiTArray::m_usESize);
                e.set(array, NiTArray::m_usESize, used.wrapping_add(1));
            }
        } else if old != 0 {
            let used = e.get(array, NiTArray::m_usESize);
            e.set(array, NiTArray::m_usESize, used.wrapping_add(0xffff));
        }
    }
    let base = e.get(array, NiTArray::m_pBase);
    e.mem
        .set_u32(base.wrapping_add(index.wrapping_mul(4)), string);
}

// Translated from 00a81450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShadeProperty::GetRTTI` (engine map, from the vtable): answers the run-time type
/// record at `011f5ae0`.
pub fn ni_shade_property_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    NI_SHADE_PROPERTY_RTTI
}

// Translated from 00ba8b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `cdecl`, one byte, stored in the byte at `0120045c` (the first
/// of the two guards [`fn_00ba9420`] tests).
pub fn fn_00ba8b60(e: &mut Engine, value: u8) {
    e.mem.set_u8(GUARD_0120045C, value);
}

// Translated from 00ba8b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, no arguments: flips the byte at `0120045d` (the second guard
/// [`fn_00ba9420`] tests) between 0 and 1 and answers its new value.
pub fn fn_00ba8b70(e: &mut Engine) -> u8 {
    let value = u8::from(e.mem.u8(GUARD_0120045D) == 0);
    e.mem.set_u8(GUARD_0120045D, value);
    value
}

// Translated from 00ba8e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; slot 0 of the vtable `010b8480` (the one the render-pass
/// arrays carry, no run-time type record before it): the deleting destructor of that
/// `NiTArray`. Puts the vtable back, frees the buffer (`m_pBase`) and, when bit 0 of the
/// flags is set, the array itself (both through the memory manager). Answers `this`.
pub fn fn_00ba8e20(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    let base = e.get(this, NiTArray::m_pBase);
    e.mem.set_u32(this.addr(), RENDER_PASS_ARRAY_VTABLE);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, base]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00ba8eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; slot 2 (`GetRTTI`) of `BSShaderProperty`'s vtable `010b8488`:
/// answers the run-time type record at `01200464`.
pub fn fn_00ba8eb0(_e: &mut Engine, _this: Ptr) -> u32 {
    BS_SHADER_PROPERTY_RTTI
}

/// A new 0x60-byte object (`00aa13e0`) made a `BSShaderProperty`
/// ([`bs_shader_property_bs_shader_property`]), or null when the allocation fails.
fn new_shader_property(e: &mut Engine) -> u32 {
    let block = e.call(NEW_OBJECT_00AA13E0, &args![0x60u32]).u32();
    if block == 0 {
        return 0;
    }
    bs_shader_property_bs_shader_property(e, Ptr::new(block)).addr()
}

// Translated from 00ba8ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba8ff0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map: `thiscall` with one word passed on (a clone): makes a new
/// `BSShaderProperty` (0x60 bytes, [`bs_shader_property_bs_shader_property`]; null when the
/// allocation fails), runs [`fn_00a81430`] on it with the word, then copies both flag words,
/// `fAlpha` and `fFadeAlpha` (`FLD`/`FSTP`, so a signalling NaN arrives quiet), `iShader`
/// and `fDepthBias` into it and clears its `iLastRenderPassState`. Answers the new object.
/// With a failed allocation the exe goes on with null and faults on the first store, as
/// the translation does.
pub fn fn_00ba8ff0(e: &mut Engine, this: Ptr<BSShaderProperty>, clone: u32) -> u32 {
    let copy = new_shader_property(e);
    fn_00a81430(e, this, copy, clone);
    let d = Ptr::<BSShaderProperty>::new(copy);
    let v = e.get(this, BSShaderProperty::ulFlags0);
    e.set(d, BSShaderProperty::ulFlags0, v);
    let v = e.get(this, BSShaderProperty::ulFlags1);
    e.set(d, BSShaderProperty::ulFlags1, v);
    let v = e.get(this, BSShaderProperty::fAlpha);
    e.set(d, BSShaderProperty::fAlpha, x87_quiet_bits(v));
    let v = e.get(this, BSShaderProperty::fFadeAlpha);
    e.set(d, BSShaderProperty::iLastRenderPassState, 0);
    e.set(d, BSShaderProperty::fFadeAlpha, x87_quiet_bits(v));
    let v = e.get(this, BSShaderProperty::iShader);
    e.set(d, BSShaderProperty::iShader, v);
    let v = e.get(this, BSShaderProperty::fDepthBias);
    e.set(d, BSShaderProperty::fDepthBias, x87_quiet_bits(v));
    copy
}

// Translated from 00ba90a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::ToggleMotionBlurRecurse` (engine map; the body matches): `cdecl`, one
/// object. For a non-null object: a tri-based geometry (slot +0x1c non-zero) with a shader
/// property (`GetProperty(3)`) gets [`bs_shader_property_set_motion_blurred`] with 1 when
/// bit 4 of the property's first flag word is clear and 0 when it is set; any other object
/// that is a node (slot +0xc) passes the call on to each of its non-null children (the array
/// at node `+0xa0`, the count at `+0xa6` read once).
pub fn bs_shader_property_toggle_motion_blur_recurse(e: &mut Engine, object: Ptr) {
    if object.is_null() {
        return;
    }
    if e.vcall(object.addr(), SLOT_IS_TRI_BASED_GEOM, &args![])
        .u32()
        != 0
    {
        let property = e
            .call(
                NI_AV_OBJECT_GET_PROPERTY,
                &args![object, SHADER_PROPERTY_TYPE],
            )
            .u32();
        if property != 0 {
            let property = Ptr::<BSShaderProperty>::new(property);
            let on = u8::from(e.mem.u8(property.addr().wrapping_add(0x20)) & 0x10 == 0);
            bs_shader_property_set_motion_blurred(e, property, on);
        }
        return;
    }
    let node = e.vcall(object.addr(), SLOT_IS_NODE, &args![]).u32();
    if node == 0 {
        return;
    }
    let count = u32::from(e.mem.u16(node.wrapping_add(0xa6)));
    let mut index = 0u32;
    while index < count {
        let buffer = e.mem.u32(node.wrapping_add(0xa0));
        let child = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
        if child != 0 {
            bs_shader_property_toggle_motion_blur_recurse(e, Ptr::new(child));
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00ba9250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba9250 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, no arguments (registered with `NiStream` as the
/// `BSShaderProperty` creator, `bsshadermanager.cpp`'s table at `010a93a0`): a new 0x60-byte
/// object (`00aa13e0`) made a `BSShaderProperty`, or null when the allocation fails.
pub fn fn_00ba9250(e: &mut Engine) -> u32 {
    new_shader_property(e)
}

/// One write through the stream's file (`[stream + 0x250]`): its write function at file
/// `+0xc` (`cdecl`: file, buffer, 4, the address of a word holding 4, 1); the answer is
/// added to the file's word at `+4`.
fn stream_write_word(e: &mut Engine, stream: u32, buffer: u32) {
    let file = e.mem.u32(stream.wrapping_add(0x250));
    let write = e.mem.u32(file.wrapping_add(0xc));
    let n = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(write, &args![file, buffer, 4u32, size, 1u32]).u32()
    });
    let position = e.mem.u32(file.wrapping_add(4));
    e.mem
        .set_u32(file.wrapping_add(4), position.wrapping_add(n));
}

// Translated from 00ba9380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` with one word, a stream (the `SaveBinary` that
/// pairs with [`fn_00ba92c0`]). Runs the base save (`00a815c0`), then writes `iShader`, the
/// two flag words and `fEnvmapScale`, one word each through the stream's file.
pub fn fn_00ba9380(e: &mut Engine, this: Ptr<BSShaderProperty>, stream: u32) {
    e.call(BASE_SAVE_BINARY_00A815C0, &args![this, stream]);
    stream_write_word(e, stream, this.addr().wrapping_add(0x58));
    let mut flags = this.addr().wrapping_add(0x20);
    let mut remaining = 2u32;
    loop {
        stream_write_word(e, stream, flags);
        flags = flags.wrapping_add(4);
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
    stream_write_word(e, stream, this.addr().wrapping_add(0x30));
}

/// The four `CheckCreate...RenderPassArray` bodies: when the list at `offset` is null, a new
/// render-pass array of `size` passes (null when the allocation fails) is stored there.
fn check_create_list(e: &mut Engine, this: Ptr<BSShaderProperty>, offset: u32, size: u32) {
    let slot = this.addr().wrapping_add(offset);
    if e.mem.u32(slot) != 0 {
        return;
    }
    let array = new_render_pass_array(e, size);
    e.mem.set_u32(slot, array);
}

// Translated from 00ba96d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba96d0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`CheckCreateLocalMapRenderPassArray` by its field): when
/// `pLocalMapRenderPassList` is null, makes a new render-pass array of the given size.
pub fn fn_00ba96d0(e: &mut Engine, this: Ptr<BSShaderProperty>, size: u32) {
    check_create_list(e, this, 0x48, size);
}

// Translated from 00ba9740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba9740 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `BSShaderProperty::CheckCreateConstAlphaRenderPassArray` (engine map; the body matches):
/// when `pConstAlphaRenderPassList` is null, makes a new render-pass array of the given size.
pub fn bs_shader_property_check_create_const_alpha_render_pass_array(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    size: u32,
) {
    check_create_list(e, this, 0x44, size);
}

// Translated from 00ba97b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba97b0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map: when `pWaterFogRenderPassList` is null, makes a new
/// render-pass array of the given size.
pub fn fn_00ba97b0(e: &mut Engine, this: Ptr<BSShaderProperty>, size: u32) {
    check_create_list(e, this, 0x50, size);
}

// Translated from 00ba9820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba9820 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map: when `pSilhouettePassList` is null, makes a new render-pass
/// array of the given size.
pub fn fn_00ba9820(e: &mut Engine, this: Ptr<BSShaderProperty>, size: u32) {
    check_create_list(e, this, 0x54, size);
}

/// The 56 flag names `GetViewerStrings` builds on its stack (immediates of the code, read
/// in Ghidra): bit `n` of the two flag words (`ulFlags[n / 32]`) is named by entry `n`
/// (`"specular"`, `"skinned"`, ... `"stinger prop"`).
const FLAG_NAMES: [u32; 56] = [
    0x010b_8950,
    0x010b_8948,
    0x010b_893c,
    0x010b_892c,
    0x010b_8920,
    0x010b_8914,
    0x010b_890c,
    0x010b_88f8,
    0x010b_88e8,
    0x010b_88dc,
    0x010b_88d4,
    0x010b_88c8,
    0x010b_88b4,
    0x010b_889c,
    0x010b_8890,
    0x010b_8884,
    0x010b_8874,
    0x010b_885c,
    0x010b_8854,
    0x010b_8844,
    0x010b_882c,
    0x010b_8810,
    0x010b_8800,
    0x010b_87f0,
    0x010b_87dc,
    0x010b_87c8,
    0x010b_87c0,
    0x010b_87b0,
    0x010b_879c,
    0x010b_8788,
    0x010b_877c,
    0x010b_876c,
    0x010b_875c,
    0x010b_874c,
    0x010b_873c,
    0x010b_8734,
    0x010b_8724,
    0x010b_8714,
    0x010b_8708,
    0x010b_86ec,
    0x010b_86e0,
    0x010b_86d4,
    0x010b_86c4,
    0x010b_86b4,
    0x010b_86a8,
    0x010b_869c,
    0x010b_8688,
    0x010b_8674,
    0x010b_8668,
    0x010b_8658,
    0x010b_8644,
    0x010b_8634,
    0x010b_8620,
    0x010b_8614,
    0x010b_85f4,
    0x010b_85e4,
];
/// The labels: `"fAlpha"`, `"fFadeAlpha"`, `"fDepthBias"`, `"Envmap Scale"`,
/// `"Shader Index"`, `"iLastRenderPassState"`, `"pRenderPassList not found"`,
/// `"number of passes"`, `" Fpass"` and `"  pass"`.
const LABEL_ALPHA: u32 = 0x010b_85dc;
const LABEL_FADE_ALPHA: u32 = 0x010b_85d0;
const LABEL_DEPTH_BIAS: u32 = 0x010b_85c4;
const LABEL_ENVMAP_SCALE: u32 = 0x010b_85b4;
const LABEL_SHADER_INDEX: u32 = 0x010b_85a4;
const LABEL_LAST_PASS_STATE: u32 = 0x010b_858c;
const LABEL_NO_PASS_LIST: u32 = 0x010b_8570;
const LABEL_PASS_COUNT: u32 = 0x010b_855c;
const LABEL_FIRST_PASS: u32 = 0x010b_8554;
const LABEL_PASS: u32 = 0x010b_854c;

// Translated from 00ba9890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::GetViewerStrings` (engine map; the body matches): `thiscall`, one word
/// (the `NiTArray<char *>` of strings). The `NiShadeProperty` strings (`00a81600`), the class
/// name, the name of each set flag bit (bits 0 to 55 of the two flag words), then `fAlpha`
/// and `fFadeAlpha` when below 1.0, `fDepthBias` when above 0.0 (NaN shows none of them),
/// `fEnvmapScale` when bit 7 of the first flag word is set (each `float` through
/// `FLD`/`FSTP`), `iShader` and `iLastRenderPassState`; then, without a pass list, the
/// not-found string, or else the pass count and a line per pass whose enum is below 0x2f7
/// (the pass name, labelled by its first-pass flag). The two integer lines and the pass
/// lines use the inlined `SetAt` ([`viewer_strings_add_inline`]).
pub fn bs_shader_property_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    strings: u32,
) {
    e.call(NI_SHADE_PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(BS_SHADER_PROPERTY_RTTI);
    let text = e.call(VIEWER_STRING, &args![class_name]).u32();
    viewer_strings_add(e, strings, text);
    let mut bit = 1u32;
    for (index, name) in FLAG_NAMES.iter().enumerate() {
        let word = this.addr().wrapping_add(0x20 + 4 * (index as u32 >> 5));
        if e.mem.u32(word) & bit != 0 && *name != 0 {
            let text = e.call(VIEWER_STRING_FLAG, &args![*name, 1u32]).u32();
            viewer_strings_add(e, strings, text);
        }
        bit = bit.rotate_left(1);
    }
    let alpha = e.get(this, BSShaderProperty::fAlpha);
    if 1.0 > f32::from_bits(alpha) {
        let text = e
            .call(
                VIEWER_STRING_FLOAT,
                &args![LABEL_ALPHA, x87_quiet_bits(alpha)],
            )
            .u32();
        viewer_strings_add(e, strings, text);
    }
    let fade = e.get(this, BSShaderProperty::fFadeAlpha);
    if 1.0 > f32::from_bits(fade) {
        let text = e
            .call(
                VIEWER_STRING_FLOAT,
                &args![LABEL_FADE_ALPHA, x87_quiet_bits(fade)],
            )
            .u32();
        viewer_strings_add(e, strings, text);
    }
    let bias = e.get(this, BSShaderProperty::fDepthBias);
    if 0.0 < f32::from_bits(bias) {
        let text = e
            .call(
                VIEWER_STRING_FLOAT,
                &args![LABEL_DEPTH_BIAS, x87_quiet_bits(bias)],
            )
            .u32();
        viewer_strings_add(e, strings, text);
    }
    if e.get(this, BSShaderProperty::ulFlags0) & 0x80 != 0 {
        let scale = x87_quiet_bits(e.get(this, BSShaderProperty::fEnvmapScale));
        let text = e
            .call(
                VIEWER_STRING_FLOAT,
                &args![LABEL_ENVMAP_SCALE, x87_quiet_bits(scale)],
            )
            .u32();
        viewer_strings_add(e, strings, text);
    }
    let shader = e.get(this, BSShaderProperty::iShader);
    let text = e
        .call(VIEWER_STRING_INT, &args![LABEL_SHADER_INDEX, shader])
        .u32();
    viewer_strings_add_inline(e, strings, text);
    let state = e.get(this, BSShaderProperty::iLastRenderPassState);
    let text = e
        .call(VIEWER_STRING_INT, &args![LABEL_LAST_PASS_STATE, state])
        .u32();
    viewer_strings_add_inline(e, strings, text);
    let list = e.get(this, BSShaderProperty::pRenderPassList);
    if list == 0 {
        let text = e.call(VIEWER_STRING, &args![LABEL_NO_PASS_LIST]).u32();
        viewer_strings_add(e, strings, text);
        return;
    }
    let count = e.get(Ptr::<RenderPassArray>::new(list), RenderPassArray::iSize);
    let text = e
        .call(VIEWER_STRING_UINT, &args![LABEL_PASS_COUNT, count])
        .u32();
    viewer_strings_add(e, strings, text);
    let mut index = 0u32;
    while index < count {
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        let base = e.get(Ptr::<NiTArray>::new(list), NiTArray::m_pBase);
        let pass = Ptr::<RenderPass>::new(e.mem.u32(base.wrapping_add(index.wrapping_mul(4))));
        let pass_enum = e.get(pass, RenderPass::usPassEnum);
        if pass_enum < 0x2f7 {
            let label = if e.get(pass, RenderPass::bFirstPass) != 0 {
                LABEL_FIRST_PASS
            } else {
                LABEL_PASS
            };
            let name = e.call(GET_PASS_NAME, &args![u32::from(pass_enum)]).u32();
            let text = e.call(VIEWER_STRING_TEXT, &args![label, name]).u32();
            viewer_strings_add_inline(e, strings, text);
        }
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ba_91a0,
            bs_shader_property_set_flag_recurse(Ptr, u32, u32)
        ),
        (0x00ba_8ec0, render_pass_entry as AbiFn),
        entry!(0x00a8_1430, fn_00a81430(Ptr<BSShaderProperty>, u32, u32)),
        entry!(
            0x00a8_1460,
            ni_shade_property_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00ba_8ab0,
            bs_shader_property_set_alpha(Ptr<BSShaderProperty>, u32)
        ),
        entry!(0x00ba_8b10, fn_00ba8b10(Ptr<BSShaderProperty>, u32, u32)),
        entry!(
            0x00ba_8b80,
            ni_t_array_render_pass_ni_t_array(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x00ba_8be0,
            bs_shader_property_post_link_object(Ptr<BSShaderProperty>, u32)
        ),
        entry!(0x00ba_8c00, fn_00ba8c00(Ptr<RenderPass>, u8)),
        entry!(0x00ba_8c30, fn_00ba8c30(Ptr<RenderPass>)),
        (0x00ba_8c50, set_lights_entry as AbiFn),
        entry!(
            0x00ba_8cd0,
            bs_shader_property_render_pass_destructor(Ptr<RenderPass>)
        ),
        entry!(
            0x00ba_8d00,
            bs_shader_property_set_motion_blurred(Ptr<BSShaderProperty>, u8)
        ),
        entry!(
            0x00ba_8d30,
            ni_t_array_render_pass_set_size(Ptr<NiTArray>, u32)
        ),
        entry!(
            0x00ba_8e50,
            bs_shader_property_bs_shader_property(Ptr<BSShaderProperty>) -> Ptr<BSShaderProperty>
        ),
        entry!(0x00ba_8f60, fn_00ba8f60(Ptr<BSShaderProperty>, u32)),
        entry!(
            0x00ba_9120,
            bs_shader_property_set_motion_blur_recurse(Ptr, u32)
        ),
        entry!(0x00ba_92c0, fn_00ba92c0(Ptr<BSShaderProperty>, u32)),
        entry!(0x00ba_9420, fn_00ba9420(Ptr)),
        entry!(
            0x00ba_94b0,
            bs_shader_property_render_pass_array_render_pass_array(
                Ptr<RenderPassArray>,
                u32,
            )
                -> Ptr<RenderPassArray>
        ),
        entry!(0x00ba_9520, fn_00ba9520(Ptr<RenderPassArray>)),
        entry!(
            0x00ba_95e0,
            bs_shader_property_check_create_render_pass_array(Ptr<BSShaderProperty>, i32)
        ),
        entry!(
            0x00ba_9660,
            bs_shader_property_check_create_depth_map_render_pass_array(Ptr<BSShaderProperty>, u32)
        ),
        entry!(
            0x00ba_9ee0,
            bs_shader_property_render_pass_array_add(
                Ptr<RenderPassArray>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            )
        ),
        entry!(
            0x00ba_a000,
            bs_shader_property_free_render_passes(Ptr<BSShaderProperty>)
        ),
        entry!(0x00a8_1450, ni_shade_property_get_rtti(Ptr) -> u32),
        entry!(0x00ba_8b60, fn_00ba8b60(u8)),
        entry!(0x00ba_8b70, fn_00ba8b70() -> u8),
        entry!(
            0x00ba_8e20,
            fn_00ba8e20(Ptr<NiTArray>, u32) -> Ptr<NiTArray>
        ),
        entry!(0x00ba_8eb0, fn_00ba8eb0(Ptr) -> u32),
        entry!(0x00ba_8ff0, fn_00ba8ff0(Ptr<BSShaderProperty>, u32) -> u32),
        entry!(
            0x00ba_90a0,
            bs_shader_property_toggle_motion_blur_recurse(Ptr)
        ),
        entry!(0x00ba_9250, fn_00ba9250() -> u32),
        entry!(0x00ba_9380, fn_00ba9380(Ptr<BSShaderProperty>, u32)),
        entry!(0x00ba_96d0, fn_00ba96d0(Ptr<BSShaderProperty>, u32)),
        entry!(
            0x00ba_9740,
            bs_shader_property_check_create_const_alpha_render_pass_array(
                Ptr<BSShaderProperty>,
                u32,
            )
        ),
        entry!(0x00ba_97b0, fn_00ba97b0(Ptr<BSShaderProperty>, u32)),
        entry!(0x00ba_9820, fn_00ba9820(Ptr<BSShaderProperty>, u32)),
        entry!(
            0x00ba_9890,
            bs_shader_property_get_viewer_strings(Ptr<BSShaderProperty>, u32)
        ),
    ]
}

#[cfg(test)]
mod render_pass_tests {
    use super::*;

    #[test]
    fn render_pass_without_lights_clears_the_pass() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for off in (0..0x10).step_by(4) {
            e.mem.set_u32(this + off, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        // The accumulator's call: geometry 0, pass 0x2f5, first pass, no
        // lights, and one more pushed word.
        let result = e.call(0x00ba_8ec0, &[this, 0, 0x0001_02f5, 0x0101, 0, 0]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u16(this + 4), 0x2f5);
        assert_eq!(e.mem.u8(this + 6), 0);
        assert_eq!(e.mem.u8(this + 7), 1);
        assert_eq!(e.mem.bytes(this + 8, 4), vec![0, 0, 0, 0]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let allocations = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMORY_ALLOCATE)
            .count();
        assert_eq!(allocations, 0);
    }

    #[test]
    fn render_pass_with_lights_copies_them_into_a_new_array() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00ba_8ec0, &[this, 0x1234, 7, 0, 3, 0xa, 0xb, 0xc]);
        assert_eq!(e.mem.u32(this), 0x1234);
        assert_eq!(e.mem.u16(this + 4), 7);
        assert_eq!(e.mem.u8(this + 7), 0);
        assert_eq!(e.mem.u8(this + 9), 3);
        assert_eq!(e.mem.u8(this + 0xa), 3);
        assert_eq!(e.mem.u8(this + 0xb), 0);
        let array = e.mem.u32(this + 0xc);
        assert_eq!(
            (0..3).map(|i| e.mem.u32(array + i * 4)).collect::<Vec<_>>(),
            vec![0xa, 0xb, 0xc]
        );
        let sizes: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMORY_ALLOCATE)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(sizes, vec![12]);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
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

    /// Objects share one vtable: slot +0x18 answers the dword at +0x10 (non-zero: geometry),
    /// slot +0xc answers the dword at +0x14 (the node view, non-null: a node).
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let mut slots = [0u32; 8];
        slots[SLOT_IS_NODE as usize / 4] = 0x0f00_e000;
        slots[SLOT_IS_GEOMETRY as usize / 4] = 0x0f00_e010;
        e.put_vtable(0x0f00_e100, &slots);
        e.register(0x0f00_e000, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(0x0f00_e010, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(NI_AV_OBJECT_GET_PROPERTY, |e, a| {
            e.mem.u32(a[0] + 0x18).into_ret()
        });
        e.register(BS_SHADER_PROPERTY_SET_FLAG, |_, _| Ret::default());
        e
    }

    fn geometry(e: &mut Engine, property: u32) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, 0x0f00_e100);
        e.mem.set_u32(object + 0x10, 1);
        e.mem.set_u32(object + 0x18, property);
        object
    }

    fn node(e: &mut Engine, children: &[u32]) -> u32 {
        let object = e.mem.alloc(0xb0);
        e.mem.set_u32(object, 0x0f00_e100);
        e.mem.set_u32(object + 0x14, object);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0xa0, buffer);
        e.mem.set_u16(object + 0xa6, children.len() as u16);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * index as u32, *child);
        }
        object
    }

    #[test]
    fn at_00ba91a0_sets_the_flag_on_a_geometrys_shader_property() {
        let mut e = rig();
        let geometry = geometry(&mut e, 0x7000);
        e.call(0x00ba_91a0, &args![geometry, 0x30u32, 1u32]);
        assert_eq!(
            calls(&e, NI_AV_OBJECT_GET_PROPERTY),
            vec![vec![geometry, 3]]
        );
        assert_eq!(
            calls(&e, BS_SHADER_PROPERTY_SET_FLAG),
            vec![vec![0x7000, 0x30, 1]]
        );
    }

    #[test]
    fn at_00ba91a0_skips_a_geometry_without_the_property_and_a_null_object() {
        let mut e = rig();
        let geometry = geometry(&mut e, 0);
        e.call(0x00ba_91a0, &args![geometry, 0x30u32, 1u32]);
        e.call(0x00ba_91a0, &args![0u32, 0x30u32, 1u32]);
        assert!(calls(&e, BS_SHADER_PROPERTY_SET_FLAG).is_empty());
        assert_eq!(calls(&e, NI_AV_OBJECT_GET_PROPERTY).len(), 1);
    }

    #[test]
    fn at_00ba91a0_walks_the_children_of_nodes_skipping_null_slots() {
        let mut e = rig();
        let first = geometry(&mut e, 0x7001);
        let second = geometry(&mut e, 0x7002);
        let inner = node(&mut e, &[second]);
        let outer = node(&mut e, &[first, 0, inner]);
        e.call(0x00ba_91a0, &args![outer, 0x30u32, 0u32]);
        assert_eq!(
            calls(&e, BS_SHADER_PROPERTY_SET_FLAG),
            vec![vec![0x7001, 0x30, 0], vec![0x7002, 0x30, 0]]
        );
    }

    #[test]
    fn at_00ba91a0_does_nothing_for_an_object_that_is_neither() {
        let mut e = rig();
        let plain = e.mem.alloc(0x20);
        e.mem.set_u32(plain, 0x0f00_e100);
        e.call(0x00ba_91a0, &args![plain, 0x30u32, 1u32]);
        assert!(calls(&e, BS_SHADER_PROPERTY_SET_FLAG).is_empty());
        assert!(calls(&e, NI_AV_OBJECT_GET_PROPERTY).is_empty());
    }

    #[test]
    fn at_00ba91a0_a_node_without_children_has_nothing_to_visit() {
        let mut e = rig();
        let empty = node(&mut e, &[]);
        e.call(0x00ba_91a0, &args![empty, 0x30u32, 1u32]);
        assert!(calls(&e, BS_SHADER_PROPERTY_SET_FLAG).is_empty());
    }
}

#[cfg(test)]
mod shader_property_tests {
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

    /// Allocation from the test heap, frees recorded only, interlocked operations on the
    /// word, and the call log on.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1].max(4)).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00a81430_copies_the_shade_flags_after_the_base_copy() {
        let mut e = rig();
        e.register(COPY_MEMBERS_00A5D7A0, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        let dest = e.mem.alloc(0x60);
        e.mem.set_u16(this + 0x18, 0x1234);
        e.call(0x00a8_1430, &args![this, dest, 7u32]);
        assert_eq!(calls(&e, COPY_MEMBERS_00A5D7A0), vec![vec![this, dest, 7]]);
        assert_eq!(e.mem.u16(dest + 0x18), 0x1234);
    }

    #[test]
    fn ni_shade_property_deleting_destructor_frees_only_with_bit_0() {
        let mut e = rig();
        e.register(BASE_DESTRUCTOR_00A5C240, |_, _| Ret::default());
        e.register(FREE_SIZED_00AA1460, |_, _| Ret::default());
        assert_eq!(e.call(0x00a8_1460, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert!(calls(&e, FREE_SIZED_00AA1460).is_empty());
        e.call(0x00a8_1460, &args![0x5000u32, 3u32]);
        assert_eq!(calls(&e, FREE_SIZED_00AA1460), vec![vec![0x5000, 0x20]]);
        assert_eq!(calls(&e, BASE_DESTRUCTOR_00A5C240).len(), 2);
    }

    #[test]
    fn set_alpha_resets_the_pass_state_when_crossing_one() {
        let mut e = rig();
        let this = e.mem.alloc(0x60);
        let cases: [(f32, u32, bool); 6] = [
            (1.0, 0.5f32.to_bits(), true),
            (0.5, 1.0f32.to_bits(), true),
            (0.5, 0.25f32.to_bits(), false),
            (2.0, 1.0f32.to_bits(), false),
            (f32::NAN, 0.5f32.to_bits(), false),
            (1.0, 0x7fc0_0000, false),
        ];
        for (old, new, resets) in cases {
            e.mem.set_f32(this + 0x28, old);
            e.mem.set_u32(this + 0x38, 9);
            e.call(0x00ba_8ab0, &args![this, new]);
            assert_eq!(e.mem.u32(this + 0x28), new);
            assert_eq!(e.mem.u32(this + 0x38) == 0, resets, "{old} -> {new:#x}");
        }
        e.call(0x00ba_8ab0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0x28), 0x7fc0_0001);
    }

    #[test]
    fn fn_00ba8b10_copies_the_shader_fields() {
        let mut e = rig();
        e.register(COPY_MEMBERS_00A5D7A0, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        let dest = e.mem.alloc(0x60);
        e.mem.set_u16(this + 0x18, 5);
        e.mem.set_u32(this + 0x20, 0x11);
        e.mem.set_u32(this + 0x24, 0x22);
        e.mem.set_u32(this + 0x28, 0x7f80_0001);
        e.mem.set_f32(this + 0x2c, 0.5);
        e.mem.set_u32(this + 0x30, 0x33);
        e.mem.set_u32(this + 0x58, 0x44);
        e.mem.set_f32(this + 0x5c, 2.0);
        e.mem.set_u32(dest + 0x38, 9);
        e.call(0x00ba_8b10, &args![this, dest, 1u32]);
        assert_eq!(calls(&e, COPY_MEMBERS_00A5D7A0), vec![vec![this, dest, 1]]);
        assert_eq!(e.mem.u16(dest + 0x18), 5);
        assert_eq!(e.mem.u32(dest + 0x20), 0x11);
        assert_eq!(e.mem.u32(dest + 0x24), 0x22);
        assert_eq!(e.mem.u32(dest + 0x28), 0x7fc0_0001);
        assert_eq!(e.mem.f32(dest + 0x2c), 0.5);
        assert_eq!(e.mem.u32(dest + 0x30), 0);
        assert_eq!(e.mem.u32(dest + 0x38), 0);
        assert_eq!(e.mem.u32(dest + 0x58), 0x44);
        assert_eq!(e.mem.f32(dest + 0x5c), 2.0);
    }

    #[test]
    fn ni_t_array_constructor_allocates_the_capacity() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0xdead);
        assert_eq!(
            e.call(0x00ba_8b80, &args![this, 0x1_0000u32, 3u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), RENDER_PASS_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 0xe), 3);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        e.call(0x00ba_8b80, &args![this, 5u32, 2u32]);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 20]]);
        assert_ne!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 8), 5);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
    }

    #[test]
    fn post_link_object_sets_the_depth_bias_for_bit_26() {
        let mut e = rig();
        e.register(NI_OBJECT_NET_POST_LINK_OBJECT, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.call(0x00ba_8be0, &args![this, 0x77u32]);
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        e.mem.set_u32(this + 0x20, 0x0400_0000);
        e.call(0x00ba_8be0, &args![this, 0x77u32]);
        assert_eq!(e.mem.f32(this + 0x5c), 1.0);
        assert_eq!(
            calls(&e, NI_OBJECT_NET_POST_LINK_OBJECT)[0],
            vec![this, 0x77]
        );
    }

    #[test]
    fn fn_00ba8c00_and_fn_00ba8c30_make_and_free_the_light_array() {
        let mut e = rig();
        let pass = e.mem.alloc(0x10);
        e.call(0x00ba_8c00, &args![pass, 0x103u32]);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 12]]);
        let array = e.mem.u32(pass + 0xc);
        assert_ne!(array, 0);
        e.call(0x00ba_8c30, &args![pass]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array]]
        );
        assert_eq!(e.mem.u32(pass + 0xc), 0);
        e.call(0x00ba_8c30, &args![pass]);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), 1);
    }

    #[test]
    fn set_lights_grows_the_array_only_past_the_maximum() {
        let mut e = rig();
        let pass = e.mem.alloc(0x10);
        e.call(0x00ba_8c50, &[pass, 2, 0xa, 0xb]);
        let first = e.mem.u32(pass + 0xc);
        assert_eq!(e.mem.u8(pass + 9), 2);
        assert_eq!(e.mem.u8(pass + 0xa), 2);
        assert_eq!((e.mem.u32(first), e.mem.u32(first + 4)), (0xa, 0xb));
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00ba_8c50, &[pass, 1, 0xc]);
        assert_eq!(e.mem.u32(pass + 0xc), first);
        assert_eq!(e.mem.u8(pass + 9), 1);
        assert_eq!(e.mem.u32(first), 0xc);
        e.call(0x00ba_8c50, &[pass, 3, 1, 2, 3]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, first]]
        );
        let second = e.mem.u32(pass + 0xc);
        assert_eq!(e.mem.u8(pass + 0xa), 3);
        assert_eq!(e.mem.u32(second + 8), 3);
        e.call(0x00ba_8c50, &[pass, 0]);
        assert_eq!(e.mem.u8(pass + 9), 0);
    }

    #[test]
    fn render_pass_destructor_clears_and_frees() {
        let mut e = rig();
        let pass = e.mem.alloc(0x10);
        e.mem.set_u16(pass + 4, 7);
        e.mem.set_u8(pass + 0xb, 1);
        e.mem.set_u32(pass + 0xc, 0x9000);
        e.call(0x00ba_8cd0, &args![pass]);
        assert_eq!(e.mem.u16(pass + 4), 0);
        assert_eq!(e.mem.u8(pass + 0xb), 0);
        assert_eq!(e.mem.u32(pass + 0xc), 0);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x9000]]
        );
    }

    #[test]
    fn set_motion_blurred_resets_the_state_only_on_a_change() {
        let mut e = rig();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x38, 9);
        e.call(0x00ba_8d00, &args![this, 0x100u32]);
        assert_eq!((e.mem.u32(this + 0x20), e.mem.u32(this + 0x38)), (0, 9));
        e.call(0x00ba_8d00, &args![this, 1u32]);
        assert_eq!((e.mem.u32(this + 0x20), e.mem.u32(this + 0x38)), (0x10, 0));
        e.mem.set_u32(this + 0x38, 9);
        e.call(0x00ba_8d00, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 0x38), 9);
        e.call(0x00ba_8d00, &args![this, 0u32]);
        assert_eq!((e.mem.u32(this + 0x20), e.mem.u32(this + 0x38)), (0, 0));
    }

    fn array_with(e: &mut Engine, max: u16, items: &[u32]) -> u32 {
        let this = e.mem.alloc(0x14);
        let base = e.mem.alloc(4 * u32::from(max).max(1));
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *item);
        }
        e.mem.set_u32(this + 4, base);
        e.mem.set_u16(this + 8, max);
        e.mem.set_u16(this + 0xa, items.len() as u16);
        e.mem
            .set_u16(this + 0xc, items.iter().filter(|i| **i != 0).count() as u16);
        e.mem.set_u16(this + 0xe, 2);
        this
    }

    #[test]
    fn set_size_shrinks_and_copies() {
        let mut e = rig();
        let this = array_with(&mut e, 4, &[1, 0, 3, 4]);
        let old = e.mem.u32(this + 4);
        e.call(0x00ba_8d30, &args![this, 4u32]);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        e.call(0x00ba_8d30, &args![this, 2u32]);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u16(this + 8), 2);
        let base = e.mem.u32(this + 4);
        assert_ne!(base, old);
        assert_eq!((e.mem.u32(base), e.mem.u32(base + 4)), (1, 0));
        assert_eq!(e.mem.u32(old + 8), 0);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, old]]
        );
    }

    #[test]
    fn set_size_grows_with_zeros_and_zero_drops_the_buffer() {
        let mut e = rig();
        // New buffers come filled with 0xee bytes, so the zeros are the translation's.
        e.register(MEMORY_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[1]);
            for i in 0..a[1] {
                e.mem.set_u8(block + i, 0xee);
            }
            block.into_ret()
        });
        let grown = array_with(&mut e, 2, &[5, 6]);
        let old = e.mem.u32(grown + 4);
        e.mem.set_u32(old + 8, 0x1234);
        e.call(0x00ba_8d30, &args![grown, 3u32]);
        let base = e.mem.u32(grown + 4);
        assert_eq!(
            (0..3).map(|i| e.mem.u32(base + 4 * i)).collect::<Vec<_>>(),
            vec![5, 6, 0]
        );
        e.call(0x00ba_8d30, &args![grown, 0u32]);
        assert_eq!(e.mem.u32(grown + 4), 0);
        assert_eq!(e.mem.u16(grown + 0xa), 0);
        assert_eq!(e.mem.u16(grown + 0xc), 0);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).last().unwrap()[1], base);
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = rig();
        e.register(BASE_CONSTRUCTOR_00A5C200, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        for off in (0..0x60).step_by(4) {
            e.mem.set_u32(this + off, 0xdead_beef);
        }
        assert_eq!(e.call(0x00ba_8e50, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BS_SHADER_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(this + 0x18), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 0xffff_ffff);
        for off in [0x28, 0x2c, 0x30] {
            assert_eq!(e.mem.f32(this + off), 1.0);
        }
        for off in [
            0x20, 0x24, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50, 0x58, 0x5c,
        ] {
            assert_eq!(e.mem.u32(this + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x54), 0xdead_beef);
    }

    #[test]
    fn fn_00ba8f60_copies_and_shares_the_object_at_0xc() {
        let mut e = rig();
        e.put_vtable(0x0f00_1000, &[0; 0x27]);
        e.mem.set_u32(0x0f00_1000 + 4, 0x0f00_0004);
        e.mem.set_u32(0x0f00_1000 + 0x98, 0x0f00_0098);
        e.register(0x0f00_0004, |_, _| Ret::default());
        e.register(0x0f00_0098, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        let dest = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x28, 0x7f80_0001);
        e.mem.set_u32(this + 0x58, 3);
        e.call(0x00ba_8f60, &args![this, dest]);
        assert_eq!(e.mem.u32(dest + 0x28), 0x7fc0_0001);
        assert_eq!(e.mem.u32(dest + 0x58), 3);
        assert!(calls(&e, 0x0f00_0098).is_empty());
        let shared = e.mem.alloc(8);
        e.mem.set_u32(shared, 0x0f00_1000);
        e.mem.set_u32(shared + 4, 1);
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, 0x0f00_1000);
        e.mem.set_u32(old + 4, 1);
        e.mem.set_u32(this + 0xc, shared);
        e.mem.set_u32(dest + 0xc, old);
        e.call(0x00ba_8f60, &args![this, dest]);
        assert_eq!(e.mem.u32(dest + 0xc), shared);
        assert_eq!(e.mem.u32(shared + 4), 2);
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![old]]);
        assert_eq!(calls(&e, 0x0f00_0098), vec![vec![shared, dest]]);
        // The same object: no reference changes.
        e.call(0x00ba_8f60, &args![this, dest]);
        assert_eq!(e.mem.u32(shared + 4), 2);
        assert_eq!(calls(&e, 0x0f00_0098).len(), 2);
    }

    #[test]
    fn set_motion_blur_recurse_reaches_geometry_through_nodes() {
        let mut e = rig();
        let mut slots = [0u32; 8];
        slots[SLOT_IS_NODE as usize / 4] = 0x0f00_e000;
        slots[SLOT_IS_TRI_BASED_GEOM as usize / 4] = 0x0f00_e010;
        e.put_vtable(0x0f00_e100, &slots);
        e.register(0x0f00_e000, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(0x0f00_e010, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(NI_AV_OBJECT_GET_PROPERTY, |e, a| {
            assert_eq!(a[1], 3);
            e.mem.u32(a[0] + 0x18).into_ret()
        });
        let property = e.mem.alloc(0x60);
        let geometry = e.mem.alloc(0x20);
        e.mem.set_u32(geometry, 0x0f00_e100);
        e.mem.set_u32(geometry + 0x10, 1);
        e.mem.set_u32(geometry + 0x18, property);
        let bare = e.mem.alloc(0x20);
        e.mem.set_u32(bare, 0x0f00_e100);
        e.mem.set_u32(bare + 0x10, 1);
        let node = e.mem.alloc(0xb0);
        e.mem.set_u32(node, 0x0f00_e100);
        e.mem.set_u32(node + 0x14, node);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer, geometry);
        e.mem.set_u32(buffer + 8, bare);
        e.mem.set_u32(node + 0xa0, buffer);
        e.mem.set_u16(node + 0xa6, 3);
        e.call(0x00ba_9120, &args![node, 1u32]);
        assert_eq!(e.mem.u32(property + 0x20), 0x10);
        assert_eq!(calls(&e, NI_AV_OBJECT_GET_PROPERTY).len(), 2);
        e.call(0x00ba_9120, &args![0u32, 1u32]);
        e.call(0x00ba_9120, &args![geometry, 0u32]);
        assert_eq!(e.mem.u32(property + 0x20), 0);
    }

    #[test]
    fn fn_00ba92c0_reads_shader_flags_and_scale() {
        let mut e = rig();
        e.register(BASE_LOAD_BINARY_00A95190, |_, _| Ret::default());
        // The read function writes the next value of a counter at +0x10 of the file.
        e.register(0x0f00_0100, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            let next = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[1], e.mem.u32(a[0] + 0x14 + 4 * next));
            e.mem.set_u32(a[0] + 0x10, next + 1);
            4u32.into_ret()
        });
        let file = e.mem.alloc(0x40);
        e.mem.set_u32(file + 8, 0x0f00_0100);
        for (i, v) in [0x21u32, 0x1, 0x2000, 0x3f80_0000].iter().enumerate() {
            e.mem.set_u32(file + 0x14 + 4 * i as u32, *v);
        }
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x38, 9);
        e.call(0x00ba_92c0, &args![this, stream]);
        assert_eq!(
            calls(&e, BASE_LOAD_BINARY_00A95190),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(this + 0x58), 0x21);
        assert_eq!(e.mem.u32(this + 0x20), 1);
        assert_eq!(e.mem.u32(this + 0x24), 0xa000);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.f32(this + 0x30), 1.0);
        assert_eq!(e.mem.u32(file + 4), 16);
        let reads: Vec<u32> = calls(&e, 0x0f00_0100).iter().map(|w| w[1]).collect();
        assert_eq!(
            reads,
            vec![this + 0x58, this + 0x20, this + 0x24, this + 0x30]
        );
    }

    #[test]
    fn fn_00ba9420_copies_the_matrices_unless_guarded() {
        let mut e = rig();
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        let source = e.mem.alloc(0xa00);
        for i in 0..32 {
            e.mem.set_u32(source + 0x980 + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(SOURCE_011F9508, source);
        e.mem.set_u8(FLAG_011ADCDC, 1);
        let object = e.mem.alloc(0x90);
        e.mem.set_f32(object + 0x68, 1.0);
        e.mem.set_u32(object + 0x74, 0x7f80_0001);
        e.mem.set_f32(object + 0x80, 3.0);
        e.mem.set_u8(GUARD_0120045D, 1);
        e.call(0x00ba_9420, &args![object]);
        assert_eq!(e.mem.u8(FLAG_0120045E), 0);
        e.mem.set_u8(GUARD_0120045D, 0);
        e.call(0x00ba_9420, &args![object]);
        assert_eq!(e.mem.u32(MATRIX_01200470), 0x100);
        assert_eq!(e.mem.u32(MATRIX_01200470 + 60), 0x10f);
        assert_eq!(e.mem.u32(MATRIX_012004B0 + 60), 0x11f);
        assert_eq!(e.mem.u8(FLAG_0120045E), 1);
        assert_eq!(e.mem.u8(FLAG_011ADCDC), 0);
        assert_eq!(e.mem.f32(FLOATS_012004F0), 1.0);
        assert_eq!(e.mem.u32(FLOATS_012004F0 + 4), 0x7fc0_0001);
        assert_eq!(e.mem.f32(FLOATS_012004F0 + 8), 3.0);
    }

    #[test]
    fn render_pass_array_constructor_sizes_the_array() {
        let mut e = rig();
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 9);
        assert_eq!(e.call(0x00ba_94b0, &args![this, 5u32]).u32(), this);
        assert_eq!(e.mem.u32(this), RENDER_PASS_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u16(this + 8), 5);
        assert_eq!(e.mem.u16(this + 0xe), 3);
        // The constructor allocates the capacity; SetSize sees the same size and stops.
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
    }

    #[test]
    fn render_pass_array_destructor_frees_the_passes() {
        let mut e = rig();
        let pass = e.mem.alloc(0x10);
        e.mem.set_u32(pass + 0xc, 0x9100);
        e.mem.set_u16(pass + 4, 3);
        let this = array_with(&mut e, 3, &[0, pass]);
        e.mem.set_u16(this + 0xc, 2);
        let base = e.mem.u32(this + 4);
        e.call(0x00ba_9520, &args![this]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![
                vec![MEMORY_MANAGER, 0x9100],
                vec![MEMORY_MANAGER, pass],
                vec![MEMORY_MANAGER, base]
            ]
        );
        assert_eq!(e.mem.u16(pass + 4), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x00ff_dead);
        assert_eq!(e.mem.u32(this), RENDER_PASS_ARRAY_VTABLE);
    }

    #[test]
    fn check_create_render_pass_arrays() {
        let mut e = rig();
        let this = e.mem.alloc(0x60);
        e.call(0x00ba_95e0, &args![this, -1i32]);
        let list = e.mem.u32(this + 0x3c);
        assert_eq!(e.mem.u16(list + 8), 5);
        e.call(0x00ba_95e0, &args![this, 7i32]);
        assert_eq!(e.mem.u32(this + 0x3c), list);
        e.call(0x00ba_9660, &args![this, 2u32]);
        let depth = e.mem.u32(this + 0x40);
        assert_eq!(e.mem.u16(depth + 8), 2);
        e.call(0x00ba_9660, &args![this, 4u32]);
        assert_eq!(e.mem.u32(this + 0x40), depth);
        // A failed allocation leaves the list null.
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        let other = e.mem.alloc(0x60);
        e.call(0x00ba_95e0, &args![other, 3i32]);
        e.call(0x00ba_9660, &args![other, 3u32]);
        assert_eq!((e.mem.u32(other + 0x3c), e.mem.u32(other + 0x40)), (0, 0));
    }

    #[test]
    fn add_reuses_a_pass_then_appends_new_ones() {
        let mut e = rig();
        e.register(ARRAY_SET_AT_00BA3AA0, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            e.mem.set_u16(a[0] + 0xc, size + 1);
            Ret::default()
        });
        let pass = e.mem.alloc(0x10);
        e.mem.set_u8(pass + 6, 1);
        e.mem.set_u8(pass + 8, 1);
        e.mem.set_u8(pass + 0xb, 1);
        let this = array_with(&mut e, 1, &[pass]);
        e.call(
            0x00ba_9ee0,
            &args![
                this,
                0x1234u32,
                0x2_0007u32,
                0x101u32,
                1u32,
                0xau32,
                0u32,
                0u32,
                0u32
            ],
        );
        assert_eq!(e.mem.u32(pass), 0x1234);
        assert_eq!(e.mem.u16(pass + 4), 7);
        assert_eq!(e.mem.u8(pass + 7), 1);
        assert_eq!(e.mem.bytes(pass + 8, 1), vec![0]);
        assert_eq!((e.mem.u8(pass + 6), e.mem.u8(pass + 0xb)), (0, 0));
        assert_eq!(e.mem.u8(pass + 9), 1);
        assert_eq!(e.mem.u32(e.mem.u32(pass + 0xc)), 0xa);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        // The array is full: it grows by 2 and the new pass goes to index 1.
        e.call(
            0x00ba_9ee0,
            &args![this, 0x5678u32, 9u32, 0u32, 2u32, 0xbu32, 0xcu32, 0u32, 0u32],
        );
        assert_eq!(e.mem.u16(this + 8), 3);
        let set_at = calls(&e, ARRAY_SET_AT_00BA3AA0);
        assert_eq!(set_at.len(), 1);
        assert_eq!(set_at[0][1], 1);
        let added = e.mem.u32(e.mem.u32(this + 4) + 4);
        assert_eq!(e.mem.u32(added), 0x5678);
        assert_eq!(e.mem.u16(added + 4), 9);
        let lights = e.mem.u32(added + 0xc);
        assert_eq!((e.mem.u32(lights), e.mem.u32(lights + 4)), (0xb, 0xc));
        assert_eq!(e.mem.u32(this + 0x10), 2);
    }

    /// The p185 stop: a slot below `m_usESize` that is null (a hole the exe's own code never
    /// leaves) stops `Add` with the array described, where the exe would fault on the null
    /// pass's `+0xa`.
    #[test]
    #[should_panic(expected = "pass check: RenderPassArray::Add reuses slot 0")]
    fn add_stops_on_a_null_slot_below_the_used_count() {
        let mut e = rig();
        let pass = e.mem.alloc(0x10);
        let this = array_with(&mut e, 2, &[0, pass]);
        e.mem.set_u32(this, RENDER_PASS_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        e.call(
            0x00ba_9ee0,
            &args![this, 0x1234u32, 7u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
    }

    /// The pass check's reading of an array: sound, a hole below `m_usESize`, a wrong vtable,
    /// and a freed (dangling) array.
    #[test]
    fn pass_check_tells_sound_holed_and_dangling_arrays_apart() {
        let mut e = rig();
        let pass = e.mem.alloc(0x10);
        let sound = array_with(&mut e, 5, &[pass, pass]);
        e.mem.set_u32(sound, RENDER_PASS_ARRAY_VTABLE);
        assert_eq!(pass_check::fault(&e, sound), None);

        let holed = array_with(&mut e, 5, &[0, pass]);
        e.mem.set_u32(holed, RENDER_PASS_ARRAY_VTABLE);
        let why = pass_check::fault(&e, holed).unwrap();
        assert!(why.contains("slot 0 below m_usESize 1 is null"), "{why}");

        let other = array_with(&mut e, 5, &[pass]);
        e.mem.set_u32(other, 0x0102_0304);
        let why = pass_check::fault(&e, other).unwrap();
        assert!(why.contains("vtable is 01020304"), "{why}");

        let freed = array_with(&mut e, 5, &[pass]);
        e.mem.set_u32(freed, RENDER_PASS_ARRAY_VTABLE);
        e.mem.free(freed);
        let why = pass_check::fault(&e, freed).unwrap();
        assert!(why.contains("not a live heap block"), "{why}");
        let text = pass_check::describe(&e, sound);
        assert!(
            text.contains("010b8480") && text.contains("live 0x18"),
            "{text}"
        );
    }

    /// Once checked, an array is watched: a call that empties slot 0 of its element buffer
    /// (as the p191 stop found: m_usESize 6, five passes, slot 0 null) stops the run as that
    /// call returns, naming it, with the change in the kept events.
    #[test]
    #[should_panic(
        expected = "broke: 1 of the 2 slots hold a pass but m_usESize is 2, right after the change in 0f000100"
    )]
    fn pass_check_names_the_call_that_empties_a_slot() {
        let mut e = rig();
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        let this = array_with(&mut e, 5, &[first, second]);
        e.mem.set_u32(this, RENDER_PASS_ARRAY_VTABLE);
        pass_check::check_now(&mut e, this, "in the test", 0);
        e.register(0x0f00_0100, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base, 0);
            Ret::default()
        });
        // `iSize` changes are not events; a sound array calls on quietly.
        e.mem.set_u32(this + 0x10, 3);
        e.call(0x0f00_0100, &[this]);
    }

    #[test]
    fn free_render_passes_frees_each_list_in_order() {
        let mut e = rig();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x38, 9);
        let mut lists = vec![];
        for offset in [0x3cu32, 0x44, 0x48, 0x54] {
            let list = array_with(&mut e, 1, &[]);
            e.mem.set_u32(this + offset, list);
            lists.push(list);
        }
        e.call(0x00ba_a000, &args![this]);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        for offset in (0x3c..=0x54).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        let freed: Vec<u32> = calls(&e, MEMORY_DEALLOCATE)
            .iter()
            .map(|w| w[1])
            .filter(|a| lists.contains(a))
            .collect();
        // +0x48 before +0x44.
        assert_eq!(freed, vec![lists[0], lists[2], lists[1], lists[3]]);
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

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1].max(4)).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(NEW_OBJECT_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCTOR_00A5C200, |_, _| Ret::default());
        e.register(COPY_MEMBERS_00A5D7A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    /// A string array of capacity `max` (grow step 2), and doubles for the size change
    /// (recorded only) and the store (writes the slot, raises `m_usSize`).
    fn strings(e: &mut Engine, max: u16) -> u32 {
        let this = e.mem.alloc(0x10);
        let base = e.mem.alloc(4 * 64);
        e.mem.set_u32(this + 4, base);
        e.mem.set_u16(this + 8, max);
        e.mem.set_u16(this + 0xe, 2);
        e.register(STRINGS_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRINGS_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let v = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], v);
            if a[1] >= u32::from(e.mem.u16(a[0] + 0xa)) {
                e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            }
            Ret::default()
        });
        this
    }

    fn slots(e: &Engine, strings: u32) -> Vec<u32> {
        let base = e.mem.u32(strings + 4);
        (0..u32::from(e.mem.u16(strings + 0xa)))
            .map(|i| e.mem.u32(base + 4 * i))
            .collect()
    }

    #[test]
    fn rtti_getters_answer_their_records() {
        let mut e = rig();
        assert_eq!(e.call(0x00a8_1450, &args![0x10u32]).u32(), 0x011f_5ae0);
        assert_eq!(e.call(0x00ba_8eb0, &args![0x10u32]).u32(), 0x0120_0464);
    }

    #[test]
    fn guard_setters_store_and_flip_the_bytes() {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.call(0x00ba_8b60, &args![0x1234_5601u32]);
        assert_eq!(e.mem.u8(GUARD_0120045C), 1);
        assert_eq!(e.call(0x00ba_8b70, &args![]).u8(), 1);
        assert_eq!(e.mem.u8(GUARD_0120045D), 1);
        assert_eq!(e.call(0x00ba_8b70, &args![]).u8(), 0);
        assert_eq!(e.mem.u8(GUARD_0120045D), 0);
    }

    #[test]
    fn array_deleting_destructor_frees_the_array_only_with_bit_0() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x7000);
        assert_eq!(e.call(0x00ba_8e20, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), RENDER_PASS_ARRAY_VTABLE);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x7000]]
        );
        e.call(0x00ba_8e20, &args![this, 1u32]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE)[1..],
            [vec![MEMORY_MANAGER, 0x7000], vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn fn_00ba8ff0_makes_a_copy_of_the_shader_fields() {
        let mut e = rig();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x20, 0x11);
        e.mem.set_u32(this + 0x24, 0x22);
        e.mem.set_u32(this + 0x28, 0x7f80_0001);
        e.mem.set_f32(this + 0x2c, 0.25);
        e.mem.set_u32(this + 0x58, 7);
        e.mem.set_f32(this + 0x5c, 2.5);
        e.mem.set_u16(this + 0x18, 0x55);
        let copy = e.call(0x00ba_8ff0, &args![this, 0x99u32]).u32();
        assert_eq!(calls(&e, NEW_OBJECT_00AA13E0), vec![vec![0x60]]);
        assert_eq!(
            calls(&e, COPY_MEMBERS_00A5D7A0),
            vec![vec![this, copy, 0x99]]
        );
        assert_eq!(e.mem.u32(copy), BS_SHADER_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(copy + 0x18), 0x55);
        assert_eq!(e.mem.u32(copy + 0x20), 0x11);
        assert_eq!(e.mem.u32(copy + 0x24), 0x22);
        assert_eq!(e.mem.u32(copy + 0x28), 0x7fc0_0001);
        assert_eq!(e.mem.f32(copy + 0x2c), 0.25);
        assert_eq!(e.mem.u32(copy + 0x38), 0);
        assert_eq!(e.mem.u32(copy + 0x58), 7);
        assert_eq!(e.mem.f32(copy + 0x5c), 2.5);
    }

    #[test]
    fn toggle_motion_blur_recurse_flips_each_geometry_through_nodes() {
        let mut e = rig();
        let mut slots = [0u32; 8];
        slots[SLOT_IS_NODE as usize / 4] = 0x0f00_e000;
        slots[SLOT_IS_TRI_BASED_GEOM as usize / 4] = 0x0f00_e010;
        e.put_vtable(0x0f00_e100, &slots);
        e.register(0x0f00_e000, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(0x0f00_e010, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(NI_AV_OBJECT_GET_PROPERTY, |e, a| {
            assert_eq!(a[1], 3);
            e.mem.u32(a[0] + 0x18).into_ret()
        });
        let blurred = e.mem.alloc(0x60);
        e.mem.set_u32(blurred + 0x20, 0x11);
        e.mem.set_u32(blurred + 0x38, 5);
        let sharp = e.mem.alloc(0x60);
        e.mem.set_u32(sharp + 0x20, 0x1);
        let geometry = |e: &mut Engine, property: u32| {
            let g = e.mem.alloc(0x20);
            e.mem.set_u32(g, 0x0f00_e100);
            e.mem.set_u32(g + 0x10, 1);
            e.mem.set_u32(g + 0x18, property);
            g
        };
        let first = geometry(&mut e, blurred);
        let second = geometry(&mut e, sharp);
        let bare = geometry(&mut e, 0);
        let node = e.mem.alloc(0xb0);
        e.mem.set_u32(node, 0x0f00_e100);
        e.mem.set_u32(node + 0x14, node);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer, first);
        e.mem.set_u32(buffer + 4, bare);
        e.mem.set_u32(buffer + 0xc, second);
        e.mem.set_u32(node + 0xa0, buffer);
        e.mem.set_u16(node + 0xa6, 4);
        e.call(0x00ba_90a0, &args![node]);
        assert_eq!(e.mem.u32(blurred + 0x20), 0x1);
        assert_eq!(e.mem.u32(blurred + 0x38), 0);
        assert_eq!(e.mem.u32(sharp + 0x20), 0x11);
        assert_eq!(calls(&e, NI_AV_OBJECT_GET_PROPERTY).len(), 3);
        e.call(0x00ba_90a0, &args![0u32]);
        assert_eq!(calls(&e, NI_AV_OBJECT_GET_PROPERTY).len(), 3);
    }

    #[test]
    fn fn_00ba9250_creates_a_shader_property_or_null() {
        let mut e = rig();
        let made = e.call(0x00ba_9250, &args![]).u32();
        assert_eq!(e.mem.u32(made), BS_SHADER_PROPERTY_VTABLE);
        assert_eq!(e.mem.f32(made + 0x28), 1.0);
        e.register(NEW_OBJECT_00AA13E0, |_, _| Ret::default());
        assert_eq!(e.call(0x00ba_9250, &args![]).u32(), 0);
    }

    #[test]
    fn fn_00ba9380_writes_shader_flags_and_scale() {
        let mut e = rig();
        e.register(BASE_SAVE_BINARY_00A815C0, |_, _| Ret::default());
        e.register(0x0f00_0200, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            3u32.into_ret()
        });
        let file = e.mem.alloc(0x20);
        e.mem.set_u32(file + 0xc, 0x0f00_0200);
        e.mem.set_u32(file + 4, 10);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x250, file);
        let this = e.mem.alloc(0x60);
        e.call(0x00ba_9380, &args![this, stream]);
        assert_eq!(
            calls(&e, BASE_SAVE_BINARY_00A815C0),
            vec![vec![this, stream]]
        );
        let written: Vec<u32> = calls(&e, 0x0f00_0200).iter().map(|w| w[1]).collect();
        assert_eq!(
            written,
            vec![this + 0x58, this + 0x20, this + 0x24, this + 0x30]
        );
        assert_eq!(e.mem.u32(file + 4), 22);
    }

    #[test]
    fn check_create_lists_fill_only_empty_slots() {
        let mut e = rig();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x44, 0x1234);
        e.call(0x00ba_96d0, &args![this, 2u32]);
        e.call(0x00ba_9740, &args![this, 2u32]);
        e.call(0x00ba_97b0, &args![this, 3u32]);
        e.call(0x00ba_9820, &args![this, 4u32]);
        assert_eq!(e.mem.u32(this + 0x44), 0x1234);
        for (offset, size) in [(0x48u32, 2u16), (0x50, 3), (0x54, 4)] {
            let list = e.mem.u32(this + offset);
            assert_eq!(e.mem.u32(list), RENDER_PASS_ARRAY_VTABLE);
            assert_eq!(e.mem.u16(list + 8), size);
            assert_eq!(e.mem.u32(list + 0x10), 0);
        }
        let allocations = calls(&e, MEMORY_ALLOCATE)
            .iter()
            .filter(|w| w[1] == 0x14)
            .count();
        assert_eq!(allocations, 3);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.mem.set_u32(this + 0x50, 0);
        e.call(0x00ba_97b0, &args![this, 3u32]);
        assert_eq!(e.mem.u32(this + 0x50), 0);
    }

    #[test]
    fn get_viewer_strings_lists_flags_values_and_passes() {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.mem.set_u32(BS_SHADER_PROPERTY_RTTI, 0x0cc0_0000);
        e.register(NI_SHADE_PROPERTY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, a| (0x1000_0000 | a[0]).into_ret());
        e.register(VIEWER_STRING_FLAG, |_, a| {
            assert_eq!(a[1], 1);
            (0x2000_0000 | a[0]).into_ret()
        });
        e.register(VIEWER_STRING_FLOAT, |_, a| {
            (0x3000_0000 | (a[0] & 0xfff) | (f32::from_bits(a[1]) as u32) << 12).into_ret()
        });
        e.register(VIEWER_STRING_INT, |_, a| {
            (0x4000_0000 | (a[0] & 0xfff) | a[1] << 12).into_ret()
        });
        e.register(VIEWER_STRING_UINT, |_, a| {
            (0x5000_0000 | (a[0] & 0xfff) | a[1] << 12).into_ret()
        });
        e.register(VIEWER_STRING_TEXT, |_, a| {
            (0x6000_0000 | (a[0] & 0xfff) | a[1] << 12).into_ret()
        });
        e.register(GET_PASS_NAME, |_, a| (a[0] + 0x100).into_ret());
        let list = e.mem.alloc(0x14);
        let passes = e.mem.alloc(0xc);
        let pass = |e: &mut Engine, pass_enum: u16, first: u8| {
            let p = e.mem.alloc(0x10);
            e.mem.set_u16(p + 4, pass_enum);
            e.mem.set_u8(p + 7, first);
            p
        };
        let first = pass(&mut e, 5, 1);
        let skipped = pass(&mut e, 0x2f7, 0);
        let last = pass(&mut e, 0x2f6, 0);
        e.mem.set_u32(passes, first);
        e.mem.set_u32(passes + 4, skipped);
        e.mem.set_u32(passes + 8, last);
        e.mem.set_u32(list + 4, passes);
        e.mem.set_u32(list + 0x10, 3);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x20, 0x8000_0081);
        e.mem.set_u32(this + 0x24, 0x0080_0000);
        e.mem.set_f32(this + 0x28, 0.5);
        e.mem.set_f32(this + 0x2c, 1.0);
        e.mem.set_u32(this + 0x5c, 0x7fc0_0000);
        e.mem.set_f32(this + 0x30, 3.0);
        e.mem.set_u32(this + 0x58, 2);
        e.mem.set_u32(this + 0x38, 9);
        e.mem.set_u32(this + 0x3c, list);
        let strings = strings(&mut e, 4);
        e.call(0x00ba_9890, &args![this, strings]);
        assert_eq!(
            slots(&e, strings),
            vec![
                0x1cc0_0000,
                0x2000_0000 | FLAG_NAMES[0],
                0x2000_0000 | FLAG_NAMES[7],
                0x2000_0000 | FLAG_NAMES[31],
                0x2000_0000 | FLAG_NAMES[55],
                0x3000_0000 | (LABEL_ALPHA & 0xfff),
                0x3000_3000 | (LABEL_ENVMAP_SCALE & 0xfff),
                0x4000_2000 | (LABEL_SHADER_INDEX & 0xfff),
                0x4000_9000 | (LABEL_LAST_PASS_STATE & 0xfff),
                0x5000_3000 | (LABEL_PASS_COUNT & 0xfff),
                0x6010_5000 | (LABEL_FIRST_PASS & 0xfff),
                0x603f_6000 | (LABEL_PASS & 0xfff),
            ]
        );
        assert_eq!(e.mem.u16(strings + 0xc), 4);
        // Every add past the capacity first asks for more room.
        let grows = calls(&e, STRINGS_SET_SIZE);
        assert_eq!(grows[0], vec![strings, 6]);
        assert_eq!(calls(&e, GET_PASS_NAME), vec![vec![5], vec![0x2f6]]);
        // Without a pass list: the not-found line.
        e.mem.set_u32(this + 0x3c, 0);
        let strings = self::strings(&mut e, 16);
        e.call(0x00ba_9890, &args![this, strings]);
        assert_eq!(
            slots(&e, strings).last(),
            Some(&(0x1000_0000 | LABEL_NO_PASS_LIST))
        );
    }

    #[test]
    fn inline_add_counts_used_slots() {
        let mut e = rig();
        let strings = strings(&mut e, 2);
        viewer_strings_add_inline(&mut e, strings, 5);
        viewer_strings_add_inline(&mut e, strings, 0);
        assert_eq!(slots(&e, strings), vec![5, 0]);
        assert_eq!(e.mem.u16(strings + 0xc), 1);
        // A resize that leaves the size above the index: the slot's old value decides.
        let base = e.mem.u32(strings + 4);
        e.mem.set_u32(base + 8, 7);
        e.register(STRINGS_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, 8);
            e.mem.set_u16(a[0] + 0xa, 4);
            Ret::default()
        });
        viewer_strings_add_inline(&mut e, strings, 0);
        assert_eq!(e.mem.u32(base + 8), 0);
        assert_eq!(e.mem.u16(strings + 0xc), 0);
        assert_eq!(e.mem.u16(strings + 0xa), 4);
        assert_eq!(calls(&e, STRINGS_SET_SIZE), vec![vec![strings, 4]]);
    }
}
