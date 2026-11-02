//! `NiMain/nirendertargetgroup.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ee87e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ee8560`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x28 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_render_target_group_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ee_8560;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x28u32]);
    }
    this
}

layout! {
    /// `NiRenderTargetGroup` (RTTI `.?AVNiRenderTargetGroup@@`), 0x28 bytes on PC (the sized
    /// delete frees 0x28). Names from the Xbox PDB, whose class is 0x30 bytes: from the
    /// buffers on the PC fields sit 8 lower.
    pub struct NiRenderTargetGroup: 0x28 {
        /// A byte the constructor clears (name not confirmed on PC).
        0x08 flag_08: u8,
        /// `m_aspBuffers` (Xbox PDB): four `NiPointer<Ni2DBuffer>`s.
        0x0c m_aspBuffers: u32,
        /// `m_uiNumBuffers` (Xbox PDB).
        0x1c m_uiNumBuffers: u32,
        /// `m_spDepthStencilBuffer` (Xbox PDB).
        0x20 m_spDepthStencilBuffer: u32,
        /// `m_pkRendererData` (Xbox PDB).
        0x24 m_pkRendererData: u32,
    }
}

/// `NiRenderTargetGroup`'s virtual table.
const NI_RENDER_TARGET_GROUP_VTABLE: u32 = 0x0110_10ec;
/// The group's virtuals (Xbox PDB slots): `AttachBuffer` (+0xa8), `AttachDepthStencilBuffer`
/// (+0xac), `SetRendererData` (+0xc4); the renderer data's scalar deleting destructor (+0x0).
const ATTACH_BUFFER_SLOT: u32 = 0xa8;
const ATTACH_DEPTH_STENCIL_BUFFER_SLOT: u32 = 0xac;
const SET_RENDERER_DATA_SLOT: u32 = 0xc4;
const DELETING_DESTRUCTOR_SLOT: u32 = 0x0;
/// The renderer's virtual +0xd8 (`__thiscall`, no words): the most buffers a group may have.
const RENDERER_MAX_BUFFERS_SLOT: u32 = 0xd8;
/// The most buffers a group holds.
const MAX_BUFFERS: u32 = 4;
/// `Ni2DBuffer`'s `m_uiWidth` (+0x08), `m_uiHeight` (+0x0c) and `m_spRendererData` (+0x10);
/// the renderer data's pixel format pointer (+0x0c).
const BUFFER_WIDTH: u32 = 0x08;
const BUFFER_HEIGHT: u32 = 0x0c;
const BUFFER_RENDERER_DATA: u32 = 0x10;
const BUFFER_DATA_PIXEL_FORMAT: u32 = 0x0c;
/// `NiRefObject::m_uiRefCount` (+0x4) and `DeleteThis` (virtual +0x4).
const REF_COUNT: u32 = 0x4;
const DELETE_THIS_SLOT: u32 = 0x4;

// Callees outside this unit.
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `00aa13e0` (cdecl, a size): the object allocation.
const NEW_OBJECT: u32 = 0x00aa_13e0;
/// `InterlockedIncrement`/`InterlockedDecrement` import slots (stdcall: the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// The reference release the code inlines: down by one, deleted at zero.
fn release(e: &mut Engine, object: u32) {
    if object != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object + REF_COUNT])
            .u32();
        if count == 0 {
            e.vcall(object, DELETE_THIS_SLOT, &args![]);
        }
    }
}

/// `NiPointer` assignment as the code inlines it, on the slot at `slot`.
fn assign(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    release(e, old);
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object + REF_COUNT]);
    }
}

/// The buffer slot `index` (`m_aspBuffers[index]`, not bounds-checked, as in the exe).
fn buffer_at(e: &Engine, this: Ptr<NiRenderTargetGroup>, index: u32) -> u32 {
    e.mem.u32(
        this.addr()
            .wrapping_add(0xc)
            .wrapping_add(index.wrapping_mul(4)),
    )
}

// Translated from 00ee8460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::SetRendererData` (Xbox PDB; `__thiscall`, one word): deletes the
/// current renderer data (its virtual +0 with 1) and stores the new one.
pub fn ni_render_target_group_set_renderer_data(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
    data: Ptr,
) {
    let old = e.get(this, NiRenderTargetGroup::m_pkRendererData);
    if old != 0 {
        e.vcall(old, DELETING_DESTRUCTOR_SLOT, &args![1u32]);
    }
    e.set(this, NiRenderTargetGroup::m_pkRendererData, data.addr());
}

// Translated from 00ee8490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::GetWidth` (Xbox PDB; one word: the buffer index): the buffer's
/// width, 0 without a buffer.
pub fn ni_render_target_group_get_width(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
    index: u32,
) -> u32 {
    let buffer = buffer_at(e, this, index);
    if buffer == 0 {
        return 0;
    }
    e.mem.u32(buffer + BUFFER_WIDTH)
}

// Translated from 00ee84b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::GetHeight` (Xbox PDB; one word: the buffer index): the buffer's
/// height, 0 without a buffer.
pub fn ni_render_target_group_get_height(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
    index: u32,
) -> u32 {
    let buffer = buffer_at(e, this, index);
    if buffer == 0 {
        return 0;
    }
    e.mem.u32(buffer + BUFFER_HEIGHT)
}

// Translated from 00ee84f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::GetBuffer` (Xbox PDB; one word: the buffer index).
pub fn ni_render_target_group_get_buffer(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
    index: u32,
) -> u32 {
    buffer_at(e, this, index)
}

// Translated from 00ee8500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiRenderTargetGroup` constructor (unnamed in the map): the `NiObject` constructor,
/// the byte at +8 cleared, the class's virtual table, and the buffers, count, depth/stencil
/// buffer and renderer data cleared. Answers `this`.
pub fn fn_00ee8500(e: &mut Engine, this: Ptr<NiRenderTargetGroup>) -> Ptr<NiRenderTargetGroup> {
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.set(this, NiRenderTargetGroup::flag_08, 0);
    e.mem.set_u32(this.addr(), NI_RENDER_TARGET_GROUP_VTABLE);
    for offset in (0x0c..=0x24).step_by(4) {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    this
}

// Translated from 00ee8560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiRenderTargetGroup` destructor body (unnamed in the map): the class's virtual
/// table, the renderer data deleted (virtual +0 with 1), the depth/stencil buffer's reference
/// given up, then the four buffers' from the last to the first, and the `NiObject` destructor
/// (a tail jump).
pub fn fn_00ee8560(e: &mut Engine, this: Ptr<NiRenderTargetGroup>) {
    let data = e.get(this, NiRenderTargetGroup::m_pkRendererData);
    e.mem.set_u32(this.addr(), NI_RENDER_TARGET_GROUP_VTABLE);
    if data != 0 {
        e.vcall(data, DELETING_DESTRUCTOR_SLOT, &args![1u32]);
    }
    let depth_stencil = e.get(this, NiRenderTargetGroup::m_spDepthStencilBuffer);
    release(e, depth_stencil);
    for index in (0..MAX_BUFFERS).rev() {
        let buffer = buffer_at(e, this, index);
        release(e, buffer);
    }
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 00ee85e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::GetPixelFormat` (Xbox PDB; one word: the buffer index): the pixel
/// format of the buffer's renderer data, 0 without a buffer or data.
pub fn ni_render_target_group_get_pixel_format(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
    index: u32,
) -> u32 {
    let buffer = buffer_at(e, this, index);
    if buffer == 0 {
        return 0;
    }
    let data = e.mem.u32(buffer + BUFFER_RENDERER_DATA);
    if data == 0 {
        return 0;
    }
    e.mem.u32(data + BUFFER_DATA_PIXEL_FORMAT)
}

// Translated from 00ee8620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::AttachBuffer` (Xbox PDB; `__thiscall`, two words: buffer, index):
/// drops the renderer data (virtual `SetRendererData(0)`), then false for an index from the
/// count on; otherwise the slot takes the buffer as an `NiPointer` does and the answer is true.
pub fn ni_render_target_group_attach_buffer(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
    buffer: Ptr,
    index: u32,
) -> bool {
    e.vcall(this.addr(), SET_RENDERER_DATA_SLOT, &args![0u32]);
    if index >= e.get(this, NiRenderTargetGroup::m_uiNumBuffers) {
        return false;
    }
    let slot = this
        .addr()
        .wrapping_add(0xc)
        .wrapping_add(index.wrapping_mul(4));
    assign(e, slot, buffer.addr());
    true
}

// Translated from 00ee8690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::AttachDepthStencilBuffer` (Xbox PDB; `__thiscall`, one word): drops
/// the renderer data (virtual `SetRendererData(0)`) and takes the buffer as an `NiPointer`
/// does. Answers true.
pub fn ni_render_target_group_attach_depth_stencil_buffer(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
    buffer: Ptr,
) -> bool {
    e.vcall(this.addr(), SET_RENDERER_DATA_SLOT, &args![0u32]);
    assign(e, this.addr() + 0x20, buffer.addr());
    true
}

// Translated from 00ee86e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::GetBufferRendererData` (Xbox PDB; one word: the buffer index): the
/// buffer's renderer data, 0 without a buffer.
pub fn ni_render_target_group_get_buffer_renderer_data(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
    index: u32,
) -> u32 {
    let buffer = buffer_at(e, this, index);
    if buffer == 0 {
        return 0;
    }
    e.mem.u32(buffer + BUFFER_RENDERER_DATA)
}

// Translated from 00ee8700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::GetDepthStencilBufferRendererData` (Xbox PDB): the depth/stencil
/// buffer's renderer data, 0 without one.
pub fn ni_render_target_group_get_depth_stencil_buffer_renderer_data(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
) -> u32 {
    let buffer = e.get(this, NiRenderTargetGroup::m_spDepthStencilBuffer);
    if buffer == 0 {
        return 0;
    }
    e.mem.u32(buffer + BUFFER_RENDERER_DATA)
}

// Translated from 00ee8710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::Create`'s shape with a buffer count (unnamed in the map; cdecl, two
/// words: count, renderer): null without a renderer, or when the count is above the
/// renderer's maximum (its virtual +0xd8) or above 4. Otherwise a new 0x28-byte group
/// ([`fn_00ee8500`]) with that count (a failed allocation still writes the count at +0x1c of
/// address 0). Answers the group.
pub fn fn_00ee8710(e: &mut Engine, count: u32, renderer: Ptr) -> Ptr<NiRenderTargetGroup> {
    if renderer.is_null() {
        return Ptr::new(0);
    }
    let most = e
        .vcall(renderer.addr(), RENDERER_MAX_BUFFERS_SLOT, &args![])
        .u32();
    if most < count || count > MAX_BUFFERS {
        return Ptr::new(0);
    }
    let memory = e.call(NEW_OBJECT, &args![0x28u32]).u32();
    let group: Ptr<NiRenderTargetGroup> = if memory != 0 {
        fn_00ee8500(e, Ptr::new(memory))
    } else {
        Ptr::new(0)
    };
    e.set(group, NiRenderTargetGroup::m_uiNumBuffers, count);
    group
}

// Translated from 00ee8760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::Create`'s shape with one buffer (unnamed in the map; cdecl, three
/// words: buffer, renderer, depth/stencil buffer): null without a renderer or buffer, or when
/// the renderer allows no buffer (its virtual +0xd8 answers 0). Otherwise a new group
/// ([`fn_00ee8500`]; a failed allocation goes on with address 0) with a count of 1 attaches the
/// buffer at index 0 (virtual `AttachBuffer`) and, when given, the depth/stencil buffer
/// (virtual `AttachDepthStencilBuffer`). Answers the group.
pub fn fn_00ee8760(
    e: &mut Engine,
    buffer: Ptr,
    renderer: Ptr,
    depth_stencil: Ptr,
) -> Ptr<NiRenderTargetGroup> {
    if renderer.is_null() || buffer.is_null() {
        return Ptr::new(0);
    }
    let most = e
        .vcall(renderer.addr(), RENDERER_MAX_BUFFERS_SLOT, &args![])
        .u32();
    if most < 1 {
        return Ptr::new(0);
    }
    let memory = e.call(NEW_OBJECT, &args![0x28u32]).u32();
    let group: Ptr<NiRenderTargetGroup> = if memory != 0 {
        fn_00ee8500(e, Ptr::new(memory))
    } else {
        Ptr::new(0)
    };
    let vtable = e.mem.u32(group.addr());
    let attach = e.mem.u32(vtable + ATTACH_BUFFER_SLOT);
    e.set(group, NiRenderTargetGroup::m_uiNumBuffers, 1);
    e.call(attach, &args![group, buffer, 0u32]);
    if !depth_stencil.is_null() {
        e.vcall(
            group.addr(),
            ATTACH_DEPTH_STENCIL_BUFFER_SLOT,
            &args![depth_stencil],
        );
    }
    group
}

/// `NiRenderTargetGroup::ms_RTTI` (the class's `NiRTTI` record).
const NI_RENDER_TARGET_GROUP_RTTI: u32 = 0x0127_08ec;
/// `NiObject::GetViewerStrings` (Xbox PDB, `thiscall`, the string array) and
/// `NiGetViewerString` (Xbox PDB, `cdecl`, one word).
const OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// `0096ad30` (`thiscall`, new capacity) and `0096ae90` (`thiscall`, index, pointer to the
/// element): the viewer-string array's set-size and set-at.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;

// Translated from 00ee84d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::GetDepthStencilWidth` (Xbox PDB): the depth-stencil buffer's
/// width, 0 without one.
pub fn ni_render_target_group_get_depth_stencil_width(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
) -> u32 {
    let buffer = e.get(this, NiRenderTargetGroup::m_spDepthStencilBuffer);
    if buffer == 0 {
        return 0;
    }
    e.mem.u32(buffer + BUFFER_WIDTH)
}

// Translated from 00ee84e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::GetDepthStencilHeight` (Xbox PDB): the depth-stencil buffer's
/// height, 0 without one.
pub fn ni_render_target_group_get_depth_stencil_height(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
) -> u32 {
    let buffer = e.get(this, NiRenderTargetGroup::m_spDepthStencilBuffer);
    if buffer == 0 {
        return 0;
    }
    e.mem.u32(buffer + BUFFER_HEIGHT)
}

// Translated from 00ee8530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_render_target_group_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_RENDER_TARGET_GROUP_RTTI)
}

// Translated from 00ee8540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): the byte at +8, the one
/// [`ni_render_target_group_set_resolve_depth_buffer_mode`] stores (`AL` only).
pub fn fn_00ee8540(e: &mut Engine, this: Ptr<NiRenderTargetGroup>) -> u8 {
    e.get(this, NiRenderTargetGroup::flag_08)
}

// Translated from 00ee8550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::SetResolveDepthBufferMode` (Xbox PDB): stores the low byte of
/// `mode` at +8.
pub fn ni_render_target_group_set_resolve_depth_buffer_mode(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
    mode: u8,
) {
    e.set(this, NiRenderTargetGroup::flag_08, mode);
}

// Translated from 00ee8600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::GetDepthStencilPixelFormat` (Xbox PDB): the pixel format (+0xc)
/// of the depth-stencil buffer's renderer data (+0x10), 0 without either.
pub fn ni_render_target_group_get_depth_stencil_pixel_format(
    e: &mut Engine,
    this: Ptr<NiRenderTargetGroup>,
) -> u32 {
    let buffer = e.get(this, NiRenderTargetGroup::m_spDepthStencilBuffer);
    if buffer == 0 {
        return 0;
    }
    let data = e.mem.u32(buffer + BUFFER_RENDERER_DATA);
    if data == 0 {
        return 0;
    }
    e.mem.u32(data + BUFFER_DATA_PIXEL_FORMAT)
}

// Translated from 00ee8800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderTargetGroup::GetViewerStrings` (Xbox PDB): `NiObject::GetViewerStrings`, then
/// appends `NiGetViewerString` of the class name (the first word of
/// `NiRenderTargetGroup::ms_RTTI`, `012708ec`) to `strings`: when the count (+0xa) is not
/// below the capacity (+8) the array first grows by `m_usGrowBy` (+0xe, `0096ad30`), then
/// `0096ae90` stores the string held in the `strings` argument's stack word at the old
/// count.
pub fn ni_render_target_group_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(NI_RENDER_TARGET_GROUP_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        let count = u32::from(e.mem.u16(strings.addr() + 0xa));
        if count >= u32::from(e.mem.u16(strings.addr() + 8)) {
            let grow_by = u32::from(e.mem.u16(strings.addr() + 0xe));
            e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
        }
        e.call(ARRAY_SET_AT, &args![strings, count, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    type G = NiRenderTargetGroup;
    vec![
        entry!(
            0x00ee_84d0,
            ni_render_target_group_get_depth_stencil_width(Ptr<G>) -> u32
        ),
        entry!(
            0x00ee_84e0,
            ni_render_target_group_get_depth_stencil_height(Ptr<G>) -> u32
        ),
        entry!(0x00ee_8530, ni_render_target_group_get_rtti() -> Ptr),
        entry!(0x00ee_8540, fn_00ee8540(Ptr<G>) -> u8),
        entry!(
            0x00ee_8550,
            ni_render_target_group_set_resolve_depth_buffer_mode(Ptr<G>, u8)
        ),
        entry!(
            0x00ee_8600,
            ni_render_target_group_get_depth_stencil_pixel_format(Ptr<G>) -> u32
        ),
        entry!(
            0x00ee_8800,
            ni_render_target_group_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00ee87e0, ni_render_target_group_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00ee8460,
            ni_render_target_group_set_renderer_data(Ptr<NiRenderTargetGroup>, Ptr)
        ),
        entry!(
            0x00ee8490,
            ni_render_target_group_get_width(Ptr<NiRenderTargetGroup>, u32) -> u32
        ),
        entry!(
            0x00ee84b0,
            ni_render_target_group_get_height(Ptr<NiRenderTargetGroup>, u32) -> u32
        ),
        entry!(
            0x00ee84f0,
            ni_render_target_group_get_buffer(Ptr<NiRenderTargetGroup>, u32) -> u32
        ),
        entry!(
            0x00ee8500,
            fn_00ee8500(Ptr<NiRenderTargetGroup>) -> Ptr<NiRenderTargetGroup>
        ),
        entry!(0x00ee8560, fn_00ee8560(Ptr<NiRenderTargetGroup>)),
        entry!(
            0x00ee85e0,
            ni_render_target_group_get_pixel_format(Ptr<NiRenderTargetGroup>, u32) -> u32
        ),
        entry!(
            0x00ee8620,
            ni_render_target_group_attach_buffer(Ptr<NiRenderTargetGroup>, Ptr, u32) -> bool
        ),
        entry!(
            0x00ee8690,
            ni_render_target_group_attach_depth_stencil_buffer(
                Ptr<NiRenderTargetGroup>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00ee86e0,
            ni_render_target_group_get_buffer_renderer_data(Ptr<NiRenderTargetGroup>, u32) -> u32
        ),
        entry!(
            0x00ee8700,
            ni_render_target_group_get_depth_stencil_buffer_renderer_data(
                Ptr<NiRenderTargetGroup>,
            ) -> u32
        ),
        entry!(0x00ee8710, fn_00ee8710(u32, Ptr) -> Ptr<NiRenderTargetGroup>),
        entry!(0x00ee8760, fn_00ee8760(Ptr, Ptr, Ptr) -> Ptr<NiRenderTargetGroup>),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_render_target_group_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ee_8560, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00ee87e0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ee87e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x28]]);
        assert_eq!(calls_to(&e, 0x00ee_8560), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b007_tests {
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

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    const DELETE_DOUBLE: u32 = 0x0070_4000;
    const DATA_DELETE_DOUBLE: u32 = 0x0070_4010;
    const MAX_BUFFERS_DOUBLE: u32 = 0x0070_4020;

    fn doubles(e: &mut Engine) {
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(NI_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        e.register(DATA_DELETE_DOUBLE, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
    }

    /// An object with a reference count and a virtual table sending +0 to the data deleter
    /// and +4 (`DeleteThis`) to the delete double.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable, DATA_DELETE_DOUBLE);
        e.mem.set_u32(vtable + DELETE_THIS_SLOT, DELETE_DOUBLE);
        e.mem.set_u32(object + REF_COUNT, count);
        object
    }

    /// The class's virtual table (in the exe), mapped with this unit's `SetRendererData`,
    /// `AttachBuffer` and `AttachDepthStencilBuffer` in their slots.
    fn class_table(e: &mut Engine) {
        let table = NI_RENDER_TARGET_GROUP_VTABLE;
        e.map(table, 0x100);
        e.mem.set_u32(table + SET_RENDERER_DATA_SLOT, 0x00ee_8460);
        e.mem.set_u32(table + ATTACH_BUFFER_SLOT, 0x00ee_8620);
        e.mem
            .set_u32(table + ATTACH_DEPTH_STENCIL_BUFFER_SLOT, 0x00ee_8690);
    }

    fn group(e: &mut Engine) -> u32 {
        class_table(e);
        let g = e.mem.alloc(0x28);
        fn_00ee8500(e, Ptr::new(g));
        g
    }

    #[test]
    fn construct_attach_query_and_destroy() {
        let mut e = Engine::new();
        doubles(&mut e);
        let g = e.mem.alloc(0x28);
        for off in (0..0x28).step_by(4) {
            e.mem.set_u32(g + off, 0x6666_6666);
        }
        assert_eq!(e.call(0x00ee8500, &args![g]).u32(), g);
        assert_eq!(e.mem.u32(g), NI_RENDER_TARGET_GROUP_VTABLE);
        assert_eq!(e.mem.u32(g + 8), 0x6666_6600);
        assert!((0x0c..0x28).step_by(4).all(|off| e.mem.u32(g + off) == 0));
        let g = group(&mut e);
        e.mem.set_u32(g + 0x1c, 2);
        let a = counted(&mut e, 1);
        e.mem.set_u32(a + BUFFER_WIDTH, 640);
        e.mem.set_u32(a + BUFFER_HEIGHT, 480);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + BUFFER_DATA_PIXEL_FORMAT, 0xf00d);
        let old_data = counted(&mut e, 1);
        e.mem.set_u32(g + 0x24, old_data);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ee8620, &args![g, a, 1u32]).bool());
        assert_eq!(calls_to(&e, DATA_DELETE_DOUBLE), vec![vec![old_data, 1]]);
        assert_eq!(e.mem.u32(g + 0x24), 0);
        assert!(!e.call(0x00ee8620, &args![g, a, 2u32]).bool());
        assert_eq!((e.mem.u32(g + 0x10), e.mem.u32(a + 4)), (a, 2));
        assert_eq!(e.call(0x00ee84f0, &args![g, 1u32]).u32(), a);
        assert_eq!(e.call(0x00ee8490, &args![g, 1u32]).u32(), 640);
        assert_eq!(e.call(0x00ee84b0, &args![g, 1u32]).u32(), 480);
        assert_eq!(e.call(0x00ee8490, &args![g, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00ee84b0, &args![g, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00ee85e0, &args![g, 1u32]).u32(), 0);
        assert_eq!(e.call(0x00ee86e0, &args![g, 1u32]).u32(), 0);
        e.mem.set_u32(a + BUFFER_RENDERER_DATA, data);
        assert_eq!(e.call(0x00ee85e0, &args![g, 1u32]).u32(), 0xf00d);
        assert_eq!(e.call(0x00ee86e0, &args![g, 1u32]).u32(), data);
        assert_eq!(e.call(0x00ee85e0, &args![g, 0u32]).u32(), 0);
        let ds = counted(&mut e, 0);
        assert_eq!(e.call(0x00ee8700, &args![g]).u32(), 0);
        assert!(e.call(0x00ee8690, &args![g, ds]).bool());
        assert_eq!((e.mem.u32(g + 0x20), e.mem.u32(ds + 4)), (ds, 1));
        e.mem.set_u32(ds + BUFFER_RENDERER_DATA, 0xda7a);
        assert_eq!(e.call(0x00ee8700, &args![g]).u32(), 0xda7a);
        // The destructor: the renderer data deleted, the references given up (ds was the last).
        let new_data = counted(&mut e, 1);
        e.call(0x00ee8460, &args![g, new_data]);
        assert_eq!(e.mem.u32(g + 0x24), new_data);
        e.call_log = Some(vec![]);
        e.call(0x00ee8560, &args![g]);
        assert_eq!(e.mem.u32(g), NI_RENDER_TARGET_GROUP_VTABLE);
        assert_eq!(calls_to(&e, DATA_DELETE_DOUBLE), vec![vec![new_data, 1]]);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![ds]]);
        assert_eq!(e.mem.u32(a + 4), 1);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCTOR), vec![vec![g]]);
    }

    #[test]
    fn create_groups() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register_double(MAX_BUFFERS_DOUBLE, |_, _| word(4));
        let renderer = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(renderer, vtable);
        e.mem
            .set_u32(vtable + RENDERER_MAX_BUFFERS_SLOT, MAX_BUFFERS_DOUBLE);
        let fresh = e.mem.alloc(0x28);
        e.register_double(NEW_OBJECT, move |_, _| word(fresh));
        assert_eq!(e.call(0x00ee8710, &args![2u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00ee8710, &args![5u32, renderer]).u32(), 0);
        assert_eq!(e.call(0x00ee8710, &args![3u32, renderer]).u32(), fresh);
        assert_eq!(e.mem.u32(fresh + 0x1c), 3);
        e.register_double(MAX_BUFFERS_DOUBLE, |_, _| word(2));
        assert_eq!(e.call(0x00ee8710, &args![3u32, renderer]).u32(), 0);
        // One buffer.
        class_table(&mut e);
        e.register_double(MAX_BUFFERS_DOUBLE, |_, _| word(1));
        let buffer = counted(&mut e, 0);
        let ds = counted(&mut e, 0);
        assert_eq!(e.call(0x00ee8760, &args![0u32, renderer, ds]).u32(), 0);
        assert_eq!(e.call(0x00ee8760, &args![buffer, 0u32, ds]).u32(), 0);
        let fresh = e.mem.alloc(0x28);
        e.register_double(NEW_OBJECT, move |_, _| word(fresh));
        assert_eq!(
            e.call(0x00ee8760, &args![buffer, renderer, ds]).u32(),
            fresh
        );
        assert_eq!(e.mem.u32(fresh + 0x1c), 1);
        assert_eq!((e.mem.u32(fresh + 0xc), e.mem.u32(buffer + 4)), (buffer, 1));
        assert_eq!((e.mem.u32(fresh + 0x20), e.mem.u32(ds + 4)), (ds, 1));
        let other = e.mem.alloc(0x28);
        e.register_double(NEW_OBJECT, move |_, _| word(other));
        assert_eq!(
            e.call(0x00ee8760, &args![buffer, renderer, 0u32]).u32(),
            other
        );
        assert_eq!(e.mem.u32(other + 0x20), 0);
        e.register_double(MAX_BUFFERS_DOUBLE, |_, _| word(0));
        assert_eq!(e.call(0x00ee8760, &args![buffer, renderer, ds]).u32(), 0);
    }
}

#[cfg(test)]
mod n018_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn depth_stencil_size_and_format() {
        let mut e = Engine::new();
        let group = e.mem.alloc(0x28);
        assert_eq!(e.call(0x00ee_84d0, &args![group]).u32(), 0);
        assert_eq!(e.call(0x00ee_84e0, &args![group]).u32(), 0);
        assert_eq!(e.call(0x00ee_8600, &args![group]).u32(), 0);
        let buffer = e.mem.alloc(0x14);
        e.mem.set_u32(buffer + 8, 640);
        e.mem.set_u32(buffer + 0xc, 480);
        e.mem.set_u32(group + 0x20, buffer);
        assert_eq!(e.call(0x00ee_84d0, &args![group]).u32(), 640);
        assert_eq!(e.call(0x00ee_84e0, &args![group]).u32(), 480);
        // A buffer without renderer data has no pixel format.
        assert_eq!(e.call(0x00ee_8600, &args![group]).u32(), 0);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 0xc, 0x7777);
        e.mem.set_u32(buffer + 0x10, data);
        assert_eq!(e.call(0x00ee_8600, &args![group]).u32(), 0x7777);
    }

    #[test]
    fn rtti_and_the_resolve_mode_byte() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00ee_8530, &args![]).u32(),
            NI_RENDER_TARGET_GROUP_RTTI
        );
        let group = e.mem.alloc(0x28);
        e.mem.set_u32(group + 8, 0xaabb_cc00);
        e.call(0x00ee_8550, &args![group, 0x1234_5601u32]);
        assert_eq!(e.mem.u32(group + 8), 0xaabb_cc01);
        assert_eq!(e.call(0x00ee_8540, &args![group]).u8(), 1);
    }

    #[test]
    fn viewer_strings_append_the_class_name() {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x1000);
        e.mem.set_u32(NI_RENDER_TARGET_GROUP_RTTI, 0x0127_0100);
        e.register(OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x5000u32.into_ret());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x5000);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 3);
        e.call(0x00ee_8800, &args![0x7777u32, strings]);
        assert_eq!(
            calls_to(&e, OBJECT_GET_VIEWER_STRINGS),
            vec![vec![0x7777, strings]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0127_0100]]);
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![strings, 5]]);
        assert_eq!(calls_to(&e, ARRAY_SET_AT)[0][..2], [strings, 2]);
    }
}
