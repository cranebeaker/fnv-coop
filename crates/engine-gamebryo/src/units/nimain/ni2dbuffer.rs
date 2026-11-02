//! `NiMain/ni2dbuffer.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `Ni2DBuffer::ms_RTTI` (the class's `NiRTTI` record).
const NI2_D_BUFFER_RTTI: u32 = 0x011f_5d00;

// Translated from 00a8f1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Ni2DBuffer::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni2_d_buffer_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI2_D_BUFFER_RTTI)
}

// Translated from 00a8f380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Ni2DBuffer::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a5d6e0`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f5d00`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni2_d_buffer_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
    const CLASS_NAME_POINTER: u32 = 0x011f_5d00;
    const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    e.call(PARENT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

layout! {
    /// `Ni2DBuffer` (Xbox PDB), 0x14 bytes (the sized delete frees 0x14).
    pub struct Ni2DBuffer: 0x14 {
        /// `m_uiWidth` (Xbox PDB).
        0x08 m_uiWidth: u32,
        /// `m_uiHeight` (Xbox PDB).
        0x0c m_uiHeight: u32,
        /// `m_spRendererData` (Xbox PDB): an `NiPointer<Ni2DBuffer::RendererData>`.
        0x10 m_spRendererData: u32,
    }
}

/// `Ni2DBuffer`'s virtual table (RTTI `.?AVNi2DBuffer@@`).
const NI2_D_BUFFER_VTABLE: u32 = 0x0109_f4dc;
/// `NiDepthStencilBuffer::ms_RTTI` (the record `nidepthstencilbuffer.rs` names).
const NI_DEPTH_STENCIL_BUFFER_RTTI: u32 = 0x011f_603c;
/// `NiRenderer::ms_pkRenderer` (`011f4748`) and the renderer's virtual +0x154 (`__thiscall`,
/// three words: the depth/stencil buffer and the two words `CreateRendererData` passes on).
const RENDERER_INSTANCE: u32 = 0x011f_4748;
const RENDERER_CREATE_DEPTH_STENCIL_DATA_SLOT: u32 = 0x154;
/// `NiRefObject::m_uiRefCount` (+0x4), `DeleteThis` (virtual +0x4), `NiObject::GetRTTI`
/// (virtual +0x8) and `NiRTTI`'s base-class pointer (+0x4).
const REF_COUNT: u32 = 0x4;
const DELETE_THIS_SLOT: u32 = 0x4;
const GET_RTTI_SLOT: u32 = 0x8;
const RTTI_BASE: u32 = 0x4;

// Callees outside this unit.
/// `00a5d3a0` and `00a5d3d0`: the `NiObject` constructor and destructor bodies.
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `00aa13e0` (cdecl, a size): the object allocation; `00aa1460` (cdecl: object, size): the
/// sized delete.
const NEW_OBJECT: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `00a5f860` (`__thiscall` on a pointer slot, one word): `NiPointer<T>::operator=`'s shape.
const POINTER_ASSIGN: u32 = 0x00a5_f860;
/// `InterlockedIncrement`/`InterlockedDecrement` import slots (stdcall: the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// The release of a reference as the code inlines it: the count goes down and at zero the
/// object deletes itself.
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

/// The `Ni2DBuffer` constructor as the code inlines it after `NiObject`'s.
fn construct_inline(e: &mut Engine, this: Ptr<Ni2DBuffer>) {
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), NI2_D_BUFFER_VTABLE);
    e.set(this, Ni2DBuffer::m_uiWidth, 0);
    e.set(this, Ni2DBuffer::m_uiHeight, 0);
    e.set(this, Ni2DBuffer::m_spRendererData, 0);
}

// Translated from 00a8f140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Ni2DBuffer::SetRendererData`'s shape (unnamed in the map; `__thiscall`, one word): the
/// renderer data pointer takes `data` as an `NiPointer` does (nothing when it is the same; the
/// old one's reference given up, the new one's taken).
pub fn fn_00a8f140(e: &mut Engine, this: Ptr<Ni2DBuffer>, data: Ptr) {
    let old = e.get(this, Ni2DBuffer::m_spRendererData);
    if old == data.addr() {
        return;
    }
    release(e, old);
    e.set(this, Ni2DBuffer::m_spRendererData, data.addr());
    if !data.is_null() {
        e.call(INTERLOCKED_INCREMENT, &args![data.addr() + REF_COUNT]);
    }
}

// Translated from 00a8f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `Ni2DBuffer` constructor (unnamed in the map): the `NiObject` constructor, no sides,
/// the class's virtual table and no renderer data. Answers `this`.
pub fn fn_00a8f190(e: &mut Engine, this: Ptr<Ni2DBuffer>) -> Ptr<Ni2DBuffer> {
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.set(this, Ni2DBuffer::m_uiWidth, 0);
    e.set(this, Ni2DBuffer::m_uiHeight, 0);
    e.mem.set_u32(this.addr(), NI2_D_BUFFER_VTABLE);
    e.set(this, Ni2DBuffer::m_spRendererData, 0);
    this
}

// Translated from 00a8f1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `Ni2DBuffer` destructor body (unnamed in the map): the class's virtual table, the
/// renderer data's reference given up, then the `NiObject` destructor (a tail jump).
pub fn fn_00a8f1c0(e: &mut Engine, this: Ptr<Ni2DBuffer>) {
    e.mem.set_u32(this.addr(), NI2_D_BUFFER_VTABLE);
    let data = e.get(this, Ni2DBuffer::m_spRendererData);
    release(e, data);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 00a8f200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Ni2DBuffer::CreateRendererData` (Xbox PDB; `__thiscall`, two words): true when there is
/// renderer data already. Otherwise only a depth/stencil buffer (its RTTI chain reaches
/// `NiDepthStencilBuffer`'s) gets data: true without a renderer, else what the renderer's
/// virtual +0x154 answers for `(this, first, second)`. Anything else answers false.
pub fn ni2_d_buffer_create_renderer_data(
    e: &mut Engine,
    this: Ptr<Ni2DBuffer>,
    first: u32,
    second: u32,
) -> bool {
    if e.get(this, Ni2DBuffer::m_spRendererData) != 0 {
        return true;
    }
    let mut rtti = e.vcall(this.addr(), GET_RTTI_SLOT, &args![]).u32();
    loop {
        if rtti == 0 {
            return false;
        }
        if rtti == NI_DEPTH_STENCIL_BUFFER_RTTI {
            break;
        }
        rtti = e.mem.u32(rtti + RTTI_BASE);
    }
    let renderer = e.mem.u32(RENDERER_INSTANCE);
    if renderer == 0 {
        return true;
    }
    e.vcall(
        renderer,
        RENDERER_CREATE_DEPTH_STENCIL_DATA_SLOT,
        &args![this, first, second],
    )
    .bool()
}

// Translated from 00a8f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Ni2DBuffer::Create`'s shape (unnamed in the map; cdecl, two words: width, height): a new
/// 0x14-byte buffer (`00aa13e0`) constructed inline, with the given sides. When the
/// allocation fails the code still writes the sides at +0x8 and +0xc of address 0. Answers the
/// buffer.
pub fn fn_00a8f260(e: &mut Engine, width: u32, height: u32) -> Ptr<Ni2DBuffer> {
    let buffer: Ptr<Ni2DBuffer> = e.call(NEW_OBJECT, &args![0x14u32]).ptr();
    if !buffer.is_null() {
        construct_inline(e, buffer);
    }
    e.set(buffer, Ni2DBuffer::m_uiWidth, width);
    e.set(buffer, Ni2DBuffer::m_uiHeight, height);
    buffer
}

// Translated from 00a8f2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Ni2DBuffer::Create`'s shape with renderer data (unnamed in the map; cdecl, three words:
/// width, height, renderer data): null without data or when the data's word at +0xc is 0.
/// Otherwise a new buffer (`00aa13e0`, constructed inline; a failed allocation goes on with
/// address 0) takes the sides, the data through `00a5f860` on its pointer, and then through
/// [`fn_00a8f140`] (which finds it already there). Answers the buffer.
pub fn fn_00a8f2c0(e: &mut Engine, width: u32, height: u32, data: Ptr) -> Ptr<Ni2DBuffer> {
    if data.is_null() || e.mem.u32(data.addr() + 0xc) == 0 {
        return Ptr::new(0);
    }
    let buffer: Ptr<Ni2DBuffer> = e.call(NEW_OBJECT, &args![0x14u32]).ptr();
    if !buffer.is_null() {
        construct_inline(e, buffer);
    }
    e.set(buffer, Ni2DBuffer::m_uiHeight, height);
    e.set(buffer, Ni2DBuffer::m_uiWidth, width);
    e.call(POINTER_ASSIGN, &args![buffer.addr() + 0x10, data]);
    fn_00a8f140(e, buffer, data);
    buffer
}

// Translated from 00a8f330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Ni2DBuffer::_scalar_deleting_destructor_` (Xbox PDB): the destructor body as
/// [`fn_00a8f1c0`] inlines it, then the sized delete of 0x14 bytes when bit 0 of `flags` is
/// set. Answers `this`.
pub fn ni2_d_buffer_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Ni2DBuffer>,
    flags: u32,
) -> Ptr<Ni2DBuffer> {
    e.mem.set_u32(this.addr(), NI2_D_BUFFER_VTABLE);
    let data = e.get(this, Ni2DBuffer::m_spRendererData);
    release(e, data);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

/// `NiObject::IsEqual` (`00a5d470`, thiscall: other object).
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
/// The renderer's virtual slots +0x114 (`FastCopy`) and +0x118 (`Copy`), each taking the
/// source buffer, the destination and the three words the buffer's method passes on.
const RENDERER_FAST_COPY_SLOT: u32 = 0x114;
const RENDERER_COPY_SLOT: u32 = 0x118;

// Translated from 00a88a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Ni2DBuffer::IsEqual` (Xbox PDB; thiscall, `RET 4`): `NiObject::IsEqual` (`00a5d470`),
/// its answer as a boolean.
pub fn ni2_d_buffer_is_equal(e: &mut Engine, this: Ptr<Ni2DBuffer>, other: Ptr) -> bool {
    e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).u8() != 0
}

/// The copy both `FastCopy` and `Copy` make: false unless this buffer has renderer data,
/// there is a renderer and the destination has renderer data; else the renderer's slot
/// with (this, destination, the three words), its byte answer.
fn copy_through_renderer(
    e: &mut Engine,
    slot: u32,
    this: Ptr<Ni2DBuffer>,
    destination: Ptr<Ni2DBuffer>,
    words: [u32; 3],
) -> bool {
    if e.get(this, Ni2DBuffer::m_spRendererData) == 0 {
        return false;
    }
    let renderer = e.mem.u32(RENDERER_INSTANCE);
    if renderer == 0 {
        return false;
    }
    if e.get(destination, Ni2DBuffer::m_spRendererData) == 0 {
        return false;
    }
    e.vcall(
        renderer,
        slot,
        &args![this, destination, words[0], words[1], words[2]],
    )
    .u8()
        != 0
}

// Translated from 00a8f0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Ni2DBuffer::FastCopy` (Xbox PDB; thiscall, `RET 0x10`: destination and three words
/// passed on as they are): the renderer's virtual slot +0x114 when both buffers have
/// renderer data and there is a renderer, else false.
pub fn ni2_d_buffer_fast_copy(
    e: &mut Engine,
    this: Ptr<Ni2DBuffer>,
    destination: Ptr<Ni2DBuffer>,
    first: u32,
    second: u32,
    third: u32,
) -> bool {
    copy_through_renderer(
        e,
        RENDERER_FAST_COPY_SLOT,
        this,
        destination,
        [first, second, third],
    )
}

// Translated from 00a8f0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Ni2DBuffer::Copy` (Xbox PDB; thiscall, `RET 0x10`): as [`ni2_d_buffer_fast_copy`],
/// through the renderer's virtual slot +0x118.
pub fn ni2_d_buffer_copy(
    e: &mut Engine,
    this: Ptr<Ni2DBuffer>,
    destination: Ptr<Ni2DBuffer>,
    first: u32,
    second: u32,
    third: u32,
) -> bool {
    copy_through_renderer(
        e,
        RENDERER_COPY_SLOT,
        this,
        destination,
        [first, second, third],
    )
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a8_8a50,
            ni2_d_buffer_is_equal(Ptr<Ni2DBuffer>, Ptr) -> bool
        ),
        entry!(
            0x00a8_f0a0,
            ni2_d_buffer_fast_copy(Ptr<Ni2DBuffer>, Ptr<Ni2DBuffer>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00a8_f0f0,
            ni2_d_buffer_copy(Ptr<Ni2DBuffer>, Ptr<Ni2DBuffer>, u32, u32, u32) -> bool
        ),
        entry!(0x00a8f140, fn_00a8f140(Ptr<Ni2DBuffer>, Ptr)),
        entry!(0x00a8f190, fn_00a8f190(Ptr<Ni2DBuffer>) -> Ptr<Ni2DBuffer>),
        entry!(0x00a8f1c0, fn_00a8f1c0(Ptr<Ni2DBuffer>)),
        entry!(
            0x00a8f200,
            ni2_d_buffer_create_renderer_data(Ptr<Ni2DBuffer>, u32, u32) -> bool
        ),
        entry!(0x00a8f260, fn_00a8f260(u32, u32) -> Ptr<Ni2DBuffer>),
        entry!(0x00a8f2c0, fn_00a8f2c0(u32, u32, Ptr) -> Ptr<Ni2DBuffer>),
        entry!(
            0x00a8f330,
            ni2_d_buffer_scalar_deleting_destructor(Ptr<Ni2DBuffer>, u32) -> Ptr<Ni2DBuffer>
        ),
        entry!(0x00a8f1b0, ni2_d_buffer_get_rtti() -> Ptr),
        entry!(0x00a8f380, ni2_d_buffer_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni2_d_buffer_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8f1b0, &args![]).u32(), NI2_D_BUFFER_RTTI);
    }

    #[test]
    fn ni2_d_buffer_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a5_d6e0;
        const NAME_POINTER: u32 = 0x011f_5d00;
        const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
        const GROW: u32 = 0x0096_ad30;
        const SET_AT: u32 = 0x0096_ae90;
        // (capacity, size): room left, then a full array.
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(NAME_POINTER, 4);
            e.set_global(NAME_POINTER, 0x5151_0001u32);
            let this = Ptr::new(e.mem.alloc(0x40));
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.register(PARENT, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(GROW, |_, _| Ret::default());
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            ni2_d_buffer_get_viewer_strings(&mut e, this, Ptr::new(strings));
            let log = e.call_log.take().unwrap();
            let order: Vec<u32> = log.iter().map(|c| c.0).collect();
            let size = u32::from(size);
            if capacity > 3 {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, SET_AT]);
            } else {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, GROW, SET_AT]);
                let grow = log.iter().find(|c| c.0 == GROW).unwrap();
                assert_eq!(grow.1, [strings, 5 + size]);
            }
            let parent = log.iter().find(|c| c.0 == PARENT).unwrap();
            assert_eq!(parent.1, [this.addr(), strings]);
            let name = log.iter().find(|c| c.0 == NI_GET_VIEWER_STRING).unwrap();
            assert_eq!(name.1, [0x5151_0001]);
            let set = log.iter().find(|c| c.0 == SET_AT).unwrap();
            assert_eq!(&set.1[..2], [strings, size]);
            assert_eq!(stored.get(), 0x7777_0001);
        }
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

    const DELETE_DOUBLE: u32 = 0x0070_3000;
    const GET_RTTI_DOUBLE: u32 = 0x0070_3010;
    const RENDERER_DOUBLE: u32 = 0x0070_3020;

    fn doubles(e: &mut Engine) {
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(NI_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
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

    /// A reference-counted object with `count` references.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + DELETE_THIS_SLOT, DELETE_DOUBLE);
        e.mem.set_u32(object + REF_COUNT, count);
        object
    }

    #[test]
    fn construct_set_data_and_destroy() {
        let mut e = Engine::new();
        doubles(&mut e);
        let buffer = e.mem.alloc(0x14);
        for off in (0..0x14).step_by(4) {
            e.mem.set_u32(buffer + off, 0x4444_4444);
        }
        assert_eq!(e.call(0x00a8f190, &args![buffer]).u32(), buffer);
        assert_eq!(e.mem.u32(buffer), NI2_D_BUFFER_VTABLE);
        assert_eq!(e.mem.u32(buffer + 4), 0x4444_4444);
        assert_eq!(
            (
                e.mem.u32(buffer + 8),
                e.mem.u32(buffer + 0xc),
                e.mem.u32(buffer + 0x10)
            ),
            (0, 0, 0)
        );
        let a = counted(&mut e, 1);
        let b = counted(&mut e, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a8f140, &args![buffer, a]);
        assert_eq!((e.mem.u32(buffer + 0x10), e.mem.u32(a + 4)), (a, 2));
        e.call(0x00a8f140, &args![buffer, a]);
        assert_eq!(e.mem.u32(a + 4), 2);
        e.call(0x00a8f140, &args![buffer, b]);
        assert_eq!((e.mem.u32(a + 4), e.mem.u32(b + 4)), (1, 2));
        // The destructor gives up b's reference; at 1 left nothing is deleted.
        e.call(0x00a8f1c0, &args![buffer]);
        assert_eq!(e.mem.u32(b + 4), 1);
        assert!(calls_to(&e, DELETE_DOUBLE).is_empty());
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCTOR), vec![vec![buffer]]);
        // The deleting destructor: the last reference deletes b, bit 0 frees 0x14 bytes.
        assert_eq!(e.call(0x00a8f330, &args![buffer, 1u32]).u32(), buffer);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![b]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![buffer, 0x14]]);
        e.mem.set_u32(buffer + 0x10, 0);
        e.call(0x00a8f330, &args![buffer, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }

    #[test]
    fn create_renderer_data_only_for_depth_stencil_buffers() {
        let mut e = Engine::new();
        e.map(RENDERER_INSTANCE, 4);
        let base_rtti = e.mem.alloc(8);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, base_rtti);
        e.register_double(GET_RTTI_DOUBLE, move |_, _| word(rtti));
        let buffer = e.mem.alloc(0x14);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(buffer, vtable);
        e.mem.set_u32(vtable + GET_RTTI_SLOT, GET_RTTI_DOUBLE);
        assert!(!e.call(0x00a8f200, &args![buffer, 1u32, 2u32]).bool());
        e.mem.set_u32(base_rtti + 4, NI_DEPTH_STENCIL_BUFFER_RTTI);
        assert!(e.call(0x00a8f200, &args![buffer, 1u32, 2u32]).bool());
        let renderer = e.mem.alloc(8);
        let renderer_vtable = e.mem.alloc(0x200);
        e.mem.set_u32(renderer, renderer_vtable);
        e.mem.set_u32(
            renderer_vtable + RENDERER_CREATE_DEPTH_STENCIL_DATA_SLOT,
            RENDERER_DOUBLE,
        );
        e.mem.set_u32(RENDERER_INSTANCE, renderer);
        e.register(RENDERER_DOUBLE, |_, a| word(u32::from(a[2] == 1) | 0x100));
        e.call_log = Some(vec![]);
        // The answer is the low byte: 0x100 is false.
        assert!(!e.call(0x00a8f200, &args![buffer, 2u32, 3u32]).bool());
        assert!(e.call(0x00a8f200, &args![buffer, 1u32, 3u32]).bool());
        assert_eq!(
            calls_to(&e, RENDERER_DOUBLE)[0],
            vec![renderer, buffer, 2, 3]
        );
        // Data already there: true without asking.
        e.mem.set_u32(buffer + 0x10, 9);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a8f200, &args![buffer, 2u32, 3u32]).bool());
        assert!(calls_to(&e, GET_RTTI_DOUBLE).is_empty());
    }

    #[test]
    fn create_buffers() {
        let mut e = Engine::new();
        doubles(&mut e);
        let fresh = e.mem.alloc(0x14);
        e.register_double(NEW_OBJECT, move |_, _| word(fresh));
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8f260, &args![640u32, 480u32]).u32(), fresh);
        assert_eq!(calls_to(&e, NEW_OBJECT), vec![vec![0x14]]);
        assert_eq!(e.mem.u32(fresh), NI2_D_BUFFER_VTABLE);
        assert_eq!((e.mem.u32(fresh + 8), e.mem.u32(fresh + 0xc)), (640, 480));
        let data = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00a8f2c0, &args![32u32, 16u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00a8f2c0, &args![32u32, 16u32, data]).u32(), 0);
        e.mem.set_u32(data + 0xc, 1);
        assert_eq!(e.call(0x00a8f2c0, &args![32u32, 16u32, data]).u32(), fresh);
        assert_eq!((e.mem.u32(fresh + 8), e.mem.u32(fresh + 0xc)), (32, 16));
        assert_eq!(calls_to(&e, POINTER_ASSIGN), vec![vec![fresh + 0x10, data]]);
        assert_eq!(e.mem.u32(fresh + 0x10), data);
        // The second assignment finds the data in place: no reference taken.
        assert_eq!(e.mem.u32(data + 4), 0);
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    const SLOT_TARGET: u32 = 0x0390_0000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn is_equal_answers_the_base_byte() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_IS_EQUAL, |_, a| (0x100 | (a[1] & 1)).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a8_8a50, &args![0x1000u32, 0x2001u32]).bool());
        assert!(!e.call(0x00a8_8a50, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(calls_to(&e, NI_OBJECT_IS_EQUAL)[0], vec![0x1000, 0x2001]);
    }

    fn copy_case(address: u32, slot: u32) {
        let mut e = Engine::new();
        e.register(SLOT_TARGET, |_, _| 0x301u32.into_ret());
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + slot, SLOT_TARGET);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, vtable);
        e.mem.map(RENDERER_INSTANCE, 4);
        e.mem.set_u32(RENDERER_INSTANCE, renderer);
        let source = e.mem.alloc(0x14);
        let target = e.mem.alloc(0x14);
        e.mem.set_u32(source + 0x10, 0x4444);
        e.mem.set_u32(target + 0x10, 0x5555);
        e.call_log = Some(vec![]);
        let words = args![source, target, 7u32, 8u32, 9u32];
        assert!(e.call(address, &words).bool());
        assert_eq!(
            calls_to(&e, SLOT_TARGET),
            vec![vec![renderer, source, target, 7, 8, 9]]
        );
        e.mem.set_u32(target + 0x10, 0);
        assert!(!e.call(address, &words).bool());
        e.mem.set_u32(target + 0x10, 0x5555);
        e.mem.set_u32(RENDERER_INSTANCE, 0);
        assert!(!e.call(address, &words).bool());
        e.mem.set_u32(RENDERER_INSTANCE, renderer);
        e.mem.set_u32(source + 0x10, 0);
        assert!(!e.call(address, &words).bool());
        assert_eq!(calls_to(&e, SLOT_TARGET).len(), 1);
    }

    #[test]
    fn fast_copy_goes_through_the_renderer_when_both_have_data() {
        copy_case(0x00a8_f0a0, RENDERER_FAST_COPY_SLOT);
    }

    #[test]
    fn copy_goes_through_the_renderer_when_both_have_data() {
        copy_case(0x00a8_f0f0, RENDERER_COPY_SLOT);
    }
}
