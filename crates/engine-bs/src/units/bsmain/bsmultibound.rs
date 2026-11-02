//! `BSMain/bsmultibound.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `BSMultiBound` is a `NiObject` that holds a reference-counted
//! `BSMultiBoundShape` (`spShape`, +0xC) and the frame number its bound was
//! last updated (`uiBoundFrameCount`, +8). It is 0x10 bytes; its vtable is
//! at `010c0f7c`.
//!
//! The constructor, the destructor, `CreateClone` and the factory
//! (`fn_00c361a0`) carry the compiler's C++ exception-unwinding frame, which
//! is not translated; they are marked `Translation incomplete`.

#[allow(unused_imports)]
use crate::prelude::*;

/// Address of the `BSMultiBound` vtable.
pub(crate) const BS_MULTI_BOUND_VTABLE: u32 = 0x010c_0f7c;
/// Address of `BSMultiBound`'s run-time type information (`GetRTTI`'s result).
const BS_MULTI_BOUND_RTTI: u32 = 0x0120_2810;
/// Size of a `BSMultiBound`.
pub(crate) const BS_MULTI_BOUND_SIZE: u32 = 0x10;

/// `NiObject::NiObject()`, thiscall, no arguments.
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The `NiObject` destructor body the `BSMultiBound` destructor ends with
/// (thiscall, no arguments).
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `NiObject::CopyMembers(this, target, cloneProcess)`.
const NI_OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
/// `NiObject::LoadBinary(this, stream)` (Xbox PDB).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `NiObject::RegisterStreamables(this, stream)` (Xbox PDB): bool.
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
/// `NiObject::IsEqual(this, other)` (Xbox PDB): bool.
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
/// `NiObject::GetViewerStrings(this, strings)` (Xbox PDB).
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// A one-word method of the stream (`this` = the stream) the engine map has
/// no name for; `LoadBinary` calls it after the base class.
const STREAM_AFTER_LOAD: u32 = 0x00a6_45b0;
/// `NiStream::GetObjectFromLinkID()` (Xbox PDB), `this` the stream.
const NI_STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// The empty stub (`RET 4`) that `LinkObject` and `SaveBinary` call first
/// (`this`, stream).
const LINK_STUB: u32 = 0x00a8_9af0;
/// The `NiPointer` assignment (`this` = the holder, then the new pointer;
/// returns the holder).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `operator new(size)` (cdecl) and `operator delete(block, size)` (cdecl).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const OPERATOR_DELETE_SIZED: u32 = 0x00aa_1460;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Vtable slots (byte offsets) used by this unit.
/// `NiObject::CreateClone` (slot 0x48), `RegisterStreamables` (0x54) and the
/// stream's slot 0x30, as the code indexes them.
const SLOT_CREATE_CLONE: u32 = 0x48;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_STREAM_SAVE_LINK: u32 = 0x30;
/// The shape's `QPointWithin` (slot 0xB4).
const SLOT_SHAPE_POINT_WITHIN: u32 = 0xb4;
/// `NiRefObject::DeleteThis` (slot 4), called when a reference count falls
/// to zero.
const SLOT_DELETE_THIS: u32 = 0x04;

layout! {
    /// `BSMultiBound` (Xbox PDB), 0x10 bytes on PC.
    pub struct BSMultiBound: 0x10 {
        /// `uiBoundFrameCount` (Xbox PDB).
        0x08 uiBoundFrameCount: u32,
        /// `spShape` (Xbox PDB): `NiPointer<BSMultiBoundShape>`.
        0x0C spShape: Ptr,
    }
}

/// Drops one reference to `object` (the body of the `NiPointer` destructor):
/// the count is at +4 and `DeleteThis` (vtable slot 4) runs when it reaches
/// zero.
fn release(e: &mut Engine, object: Ptr) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![object.addr() + 4])
        .u32();
    if count == 0 {
        e.vcall(object.addr(), SLOT_DELETE_THIS, &args![]);
    }
}

// Translated from 00c35e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBound::BSMultiBound` (Xbox PDB): builds the `NiObject` base, sets
/// the vtable, clears the shape pointer (the `NiPointer` constructor's
/// increment of a null pointer never runs) and the frame count, and assigns
/// null to the holder. Returns `this`.
// Translation incomplete: 00c35e00 - C++ exception unwinding frame is not translated.
pub fn bs_multi_bound_bs_multi_bound(e: &mut Engine, this: Ptr<BSMultiBound>) -> Ptr<BSMultiBound> {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BS_MULTI_BOUND_VTABLE);
    e.set(this, BSMultiBound::spShape, Ptr::NULL);
    let shape = e.get(this, BSMultiBound::spShape);
    if !shape.is_null() {
        e.call(INTERLOCKED_INCREMENT, &args![shape.addr() + 4]);
    }
    e.set(this, BSMultiBound::uiBoundFrameCount, 0);
    e.call(NI_POINTER_ASSIGN, &args![this.addr() + 0xc, 0u32]);
    this
}

// Translated from 00c35eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMultiBound` destructor body (the engine map has no name for it):
/// restores the vtable, releases the shape, then runs the `NiObject`
/// destructor body.
// Translation incomplete: 00c35eb0 - C++ exception unwinding frame is not translated.
pub fn fn_00c35eb0(e: &mut Engine, this: Ptr<BSMultiBound>) {
    e.mem.set_u32(this.addr(), BS_MULTI_BOUND_VTABLE);
    e.call(NI_POINTER_ASSIGN, &args![this.addr() + 0xc, 0u32]);
    let shape = e.get(this, BSMultiBound::spShape);
    if !shape.is_null() {
        release(e, shape);
    }
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00c35f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBound::CreateClone` (Xbox PDB): allocates and constructs a new
/// `BSMultiBound`, copies this one's members into it and returns it (null if
/// the allocation failed; `CopyMembers` still runs, as in the game).
// Translation incomplete: 00c35f50 - C++ exception unwinding frame is not translated.
pub fn bs_multi_bound_create_clone(
    e: &mut Engine,
    this: Ptr<BSMultiBound>,
    clone_process: u32,
) -> Ptr<BSMultiBound> {
    let block = e
        .call(OPERATOR_NEW, &args![BS_MULTI_BOUND_SIZE])
        .ptr::<BSMultiBound>();
    let clone = if block.is_null() {
        Ptr::NULL
    } else {
        bs_multi_bound_bs_multi_bound(e, block)
    };
    bs_multi_bound_copy_members(e, this, clone, clone_process);
    clone
}

// Translated from 00c35fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBound::CopyMembers` (Xbox PDB): copies the base members into
/// `target`, then gives it a clone of this one's shape (`CreateClone`, vtable
/// slot 0x48, with the clone process).
pub fn bs_multi_bound_copy_members(
    e: &mut Engine,
    this: Ptr<BSMultiBound>,
    target: Ptr<BSMultiBound>,
    clone_process: u32,
) {
    e.call(NI_OBJECT_COPY_MEMBERS, &args![this, target, clone_process]);
    let shape = e.get(this, BSMultiBound::spShape);
    let shape_clone = e
        .vcall(shape.addr(), SLOT_CREATE_CLONE, &args![clone_process])
        .u32();
    e.call(NI_POINTER_ASSIGN, &args![target.addr() + 0xc, shape_clone]);
}

// Translated from 00c36040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBound::LoadBinary` (Xbox PDB): loads the `NiObject` base, then
/// calls the stream method `00a645b0` (unnamed in the engine map) on the
/// stream.
pub fn bs_multi_bound_load_binary(e: &mut Engine, this: Ptr<BSMultiBound>, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_AFTER_LOAD, &args![stream]);
}

// Translated from 00c36070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBound::LinkObject` (Xbox PDB): calls the empty stub `00a89af0`
/// and takes the next object from the stream's link IDs into the shape
/// holder.
pub fn bs_multi_bound_link_object(e: &mut Engine, this: Ptr<BSMultiBound>, stream: Ptr) {
    e.call(LINK_STUB, &args![this, stream]);
    let object = e
        .call(NI_STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream])
        .u32();
    e.call(NI_POINTER_ASSIGN, &args![this.addr() + 0xc, object]);
}

// Translated from 00c360a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBound::RegisterStreamables` (Xbox PDB): registers the base class;
/// when that succeeds, registers the shape too (`RegisterStreamables`, vtable
/// slot 0x54). Returns whether the base class registered.
pub fn bs_multi_bound_register_streamables(
    e: &mut Engine,
    this: Ptr<BSMultiBound>,
    stream: Ptr,
) -> bool {
    if !e
        .call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let shape = e.get(this, BSMultiBound::spShape);
    e.vcall(shape.addr(), SLOT_REGISTER_STREAMABLES, &args![stream]);
    true
}

// Translated from 00c360f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBound::SaveBinary` (Xbox PDB): calls the empty stub `00a89af0`,
/// then writes the shape's link through the stream's vtable slot 0x30.
pub fn bs_multi_bound_save_binary(e: &mut Engine, this: Ptr<BSMultiBound>, stream: Ptr) {
    e.call(LINK_STUB, &args![this, stream]);
    let shape = e.get(this, BSMultiBound::spShape);
    e.vcall(stream.addr(), SLOT_STREAM_SAVE_LINK, &args![shape]);
}

// Translated from 00c36130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBound::IsEqual` (Xbox PDB): true when both the shape pointers and
/// the bound frame counts are equal. The base class `IsEqual` runs first but
/// its result is overwritten, as in the game.
pub fn bs_multi_bound_is_equal(
    e: &mut Engine,
    this: Ptr<BSMultiBound>,
    other: Ptr<BSMultiBound>,
) -> bool {
    e.call(NI_OBJECT_IS_EQUAL, &args![this, other]);
    e.get(this, BSMultiBound::spShape) == e.get(other, BSMultiBound::spShape)
        && e.get(this, BSMultiBound::uiBoundFrameCount)
            == e.get(other, BSMultiBound::uiBoundFrameCount)
}

// Translated from 00c361a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Factory for the run-time type information: allocates a `BSMultiBound` and
/// constructs it (null if the allocation failed).
// Translation incomplete: 00c361a0 - C++ exception unwinding frame is not translated.
pub fn fn_00c361a0(e: &mut Engine) -> Ptr<BSMultiBound> {
    let block = e
        .call(OPERATOR_NEW, &args![BS_MULTI_BOUND_SIZE])
        .ptr::<BSMultiBound>();
    if block.is_null() {
        Ptr::NULL
    } else {
        bs_multi_bound_bs_multi_bound(e, block)
    }
}

// Translated from 00c36220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMultiBound` vtable's slot 0x60 (`GetViewerStrings`): forwards to
/// `NiObject::GetViewerStrings`.
pub fn fn_00c36220(e: &mut Engine, this: Ptr<BSMultiBound>, strings: Ptr) {
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
}

// Translated from 00c36240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBound::QPointWithin` (Xbox PDB): false without a shape, otherwise
/// the shape's `QPointWithin` (vtable slot 0xB4) on the point.
pub fn bs_multi_bound_q_point_within(e: &mut Engine, this: Ptr<BSMultiBound>, point: Ptr) -> bool {
    let shape = e.get(this, BSMultiBound::spShape);
    if shape.is_null() {
        return false;
    }
    e.vcall(shape.addr(), SLOT_SHAPE_POINT_WITHIN, &args![point])
        .bool()
}

// Translated from 00c36290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBound::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body and, when bit 0 of `flags` is set, frees the 0x10 bytes.
/// Returns `this`.
pub fn bs_multi_bound_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSMultiBound>,
    flags: u32,
) -> Ptr<BSMultiBound> {
    fn_00c35eb0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_SIZED, &args![this, BS_MULTI_BOUND_SIZE]);
    }
    this
}

// Translated from 00c362c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPointer` destructor for a `BSMultiBoundShape` holder (fastcall:
/// `holder` is the address of the pointer): releases the held object, if
/// any; the holder keeps its value.
pub fn fn_00c362c0(e: &mut Engine, holder: Ptr) {
    let object = Ptr::<()>::new(e.mem.u32(holder.addr()));
    if !object.is_null() {
        release(e, object);
    }
}

// Translated from 00c36300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBound::GetRTTI` (Xbox PDB): the address of the class's run-time
/// type information.
pub fn bs_multi_bound_get_rtti(_e: &mut Engine, _this: Ptr<BSMultiBound>) -> u32 {
    BS_MULTI_BOUND_RTTI
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c35e00,
            bs_multi_bound_bs_multi_bound(Ptr<BSMultiBound>) -> Ptr<BSMultiBound>
        ),
        entry!(0x00c35eb0, fn_00c35eb0(Ptr<BSMultiBound>)),
        entry!(
            0x00c35f50,
            bs_multi_bound_create_clone(Ptr<BSMultiBound>, u32) -> Ptr<BSMultiBound>
        ),
        entry!(
            0x00c35fe0,
            bs_multi_bound_copy_members(Ptr<BSMultiBound>, Ptr<BSMultiBound>, u32)
        ),
        entry!(
            0x00c36040,
            bs_multi_bound_load_binary(Ptr<BSMultiBound>, Ptr)
        ),
        entry!(
            0x00c36070,
            bs_multi_bound_link_object(Ptr<BSMultiBound>, Ptr)
        ),
        entry!(
            0x00c360a0,
            bs_multi_bound_register_streamables(Ptr<BSMultiBound>, Ptr) -> bool
        ),
        entry!(
            0x00c360f0,
            bs_multi_bound_save_binary(Ptr<BSMultiBound>, Ptr)
        ),
        entry!(
            0x00c36130,
            bs_multi_bound_is_equal(Ptr<BSMultiBound>, Ptr<BSMultiBound>) -> bool
        ),
        entry!(0x00c361a0, fn_00c361a0() -> Ptr<BSMultiBound>),
        entry!(0x00c36220, fn_00c36220(Ptr<BSMultiBound>, Ptr)),
        entry!(
            0x00c36240,
            bs_multi_bound_q_point_within(Ptr<BSMultiBound>, Ptr) -> bool
        ),
        entry!(
            0x00c36290,
            bs_multi_bound_scalar_deleting_destructor(Ptr<BSMultiBound>, u32) -> Ptr<BSMultiBound>
        ),
        entry!(0x00c362c0, fn_00c362c0(Ptr)),
        entry!(
            0x00c36300,
            bs_multi_bound_get_rtti(Ptr<BSMultiBound>) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Address of a fake vtable the tests give shapes and the stream.
    const FAKE_VTABLE: u32 = 0x7000_0000;

    fn test_engine() -> Engine {
        let mut e = Engine::new();
        // The callees outside this file.
        for addr in [
            NI_OBJECT_CONSTRUCT,
            NI_OBJECT_DESTRUCT,
            NI_OBJECT_COPY_MEMBERS,
            NI_OBJECT_LOAD_BINARY,
            NI_OBJECT_GET_VIEWER_STRINGS,
            STREAM_AFTER_LOAD,
            LINK_STUB,
            NI_STREAM_GET_OBJECT_FROM_LINK_ID,
            NI_OBJECT_REGISTER_STREAMABLES,
            NI_OBJECT_IS_EQUAL,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        // The `NiPointer` assignment: stores the pointer into the holder.
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(OPERATOR_DELETE_SIZED, |_, _| Ret::default());
        // The imports are only resolved with the exe mapped: stand in for the
        // two interlocked counters (they return the new value).
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        e
    }

    /// A fake shape object whose vtable slot `slot` runs `target`.
    fn shape_with(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr {
        let mut table = vec![0u32; 0x40];
        for (slot, target) in slots {
            table[(*slot / 4) as usize] = *target;
        }
        e.put_vtable(FAKE_VTABLE, &table);
        let shape = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(shape.addr(), FAKE_VTABLE);
        e.mem.set_u32(shape.addr() + 4, 1);
        shape
    }

    #[test]
    fn constructor_sets_vtable_and_clears_the_members() {
        let mut e = test_engine();
        let object: Ptr<BSMultiBound> = e.new_object();
        e.set(object, BSMultiBound::uiBoundFrameCount, 9);
        e.set(object, BSMultiBound::spShape, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        let back = e.call(0x00c3_5e00, &args![object]).ptr::<BSMultiBound>();
        assert_eq!(back, object);
        assert_eq!(e.mem.u32(object.addr()), BS_MULTI_BOUND_VTABLE);
        assert_eq!(e.get(object, BSMultiBound::uiBoundFrameCount), 0);
        assert!(e.get(object, BSMultiBound::spShape).is_null());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(NI_OBJECT_CONSTRUCT, vec![object.addr()])));
        assert!(log.contains(&(NI_POINTER_ASSIGN, vec![object.addr() + 0xc, 0])));
        // The null shape is never counted.
        assert!(!log.iter().any(|(addr, _)| *addr == INTERLOCKED_INCREMENT));
    }

    #[test]
    fn destructor_releases_nothing_when_the_holder_was_cleared() {
        let mut e = test_engine();
        let object: Ptr<BSMultiBound> = e.new_object();
        let shape = shape_with(&mut e, &[]);
        e.set(object, BSMultiBound::spShape, shape);
        e.call_log = Some(vec![]);
        e.call(0x00c3_5eb0, &args![object]);
        // The assignment of null replaced the shape first, so nothing is
        // decremented, and the base destructor body runs last.
        assert!(e.get(object, BSMultiBound::spShape).is_null());
        assert_eq!(e.mem.u32(object.addr()), BS_MULTI_BOUND_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(addr, _)| *addr == INTERLOCKED_DECREMENT));
        assert_eq!(
            log.last().unwrap(),
            &(NI_OBJECT_DESTRUCT, vec![object.addr()])
        );
    }

    #[test]
    fn destructor_releases_a_shape_the_assignment_left_in_place() {
        let mut e = test_engine();
        // An assignment double that leaves the holder alone, to reach the
        // release branch the game's real assignment makes unreachable.
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        let object: Ptr<BSMultiBound> = e.new_object();
        let shape = shape_with(&mut e, &[(SLOT_DELETE_THIS, 0x00de_0001)]);
        e.set(object, BSMultiBound::spShape, shape);
        e.register(0x00de_0001, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_5eb0, &args![object]);
        let log = e.call_log.take().unwrap();
        // The count of 1 fell to zero: the shape's DeleteThis ran.
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![shape.addr() + 4])));
        assert!(log.contains(&(0x00de_0001, vec![shape.addr()])));
    }

    #[test]
    fn create_clone_builds_and_copies() {
        let mut e = test_engine();
        let source: Ptr<BSMultiBound> = e.new_object();
        e.call_log = Some(vec![]);
        // The source has a shape whose CreateClone returns a fixed object.
        let shape = shape_with(&mut e, &[(SLOT_CREATE_CLONE, 0x00de_0002)]);
        e.register(0x00de_0002, |_, _| Ret {
            eax: 0xCAFE_0000,
            ..Ret::default()
        });
        e.set(source, BSMultiBound::spShape, shape);
        let clone = e
            .call(0x00c3_5f50, &args![source, 77u32])
            .ptr::<BSMultiBound>();
        assert!(!clone.is_null());
        assert_eq!(e.mem.u32(clone.addr()), BS_MULTI_BOUND_VTABLE);
        assert_eq!(e.get(clone, BSMultiBound::spShape), Ptr::new(0xCAFE_0000));
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(OPERATOR_NEW, vec![0x10])));
        assert!(log.contains(&(
            NI_OBJECT_COPY_MEMBERS,
            vec![source.addr(), clone.addr(), 77]
        )));
        assert!(log.contains(&(0x00de_0002, vec![shape.addr(), 77])));
    }

    #[test]
    fn create_clone_with_a_failed_allocation_still_copies_members() {
        let mut e = test_engine();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        let source: Ptr<BSMultiBound> = e.new_object();
        let shape = shape_with(&mut e, &[(SLOT_CREATE_CLONE, 0x00de_0002)]);
        e.register(0x00de_0002, |_, _| Ret::default());
        e.set(source, BSMultiBound::spShape, shape);
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c3_5f50, &args![source, 5u32])
            .ptr::<BSMultiBound>();
        assert!(clone.is_null());
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(addr, _)| *addr == NI_OBJECT_CONSTRUCT));
        assert!(log.contains(&(NI_OBJECT_COPY_MEMBERS, vec![source.addr(), 0, 5])));
    }

    #[test]
    fn load_binary_runs_the_base_then_the_stream_method() {
        let mut e = test_engine();
        let object: Ptr<BSMultiBound> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c3_6040, &args![object, 0x4000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            &log[1..],
            &[
                (NI_OBJECT_LOAD_BINARY, vec![object.addr(), 0x4000]),
                (STREAM_AFTER_LOAD, vec![0x4000]),
            ]
        );
    }

    #[test]
    fn link_object_stores_the_linked_object_as_the_shape() {
        let mut e = test_engine();
        e.register(NI_STREAM_GET_OBJECT_FROM_LINK_ID, |_, _| Ret {
            eax: 0x5500,
            ..Ret::default()
        });
        let object: Ptr<BSMultiBound> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c3_6070, &args![object, 0x4000u32]);
        assert_eq!(e.get(object, BSMultiBound::spShape), Ptr::new(0x5500));
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (LINK_STUB, vec![object.addr(), 0x4000]));
        assert_eq!(log[2], (NI_STREAM_GET_OBJECT_FROM_LINK_ID, vec![0x4000]));
    }

    #[test]
    fn register_streamables_registers_the_shape_only_after_the_base() {
        let mut e = test_engine();
        let object: Ptr<BSMultiBound> = e.new_object();
        let shape = shape_with(&mut e, &[(SLOT_REGISTER_STREAMABLES, 0x00de_0003)]);
        e.register(0x00de_0003, |_, _| Ret::default());
        e.set(object, BSMultiBound::spShape, shape);

        // The base class refuses: no shape call, result false.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c3_60a0, &args![object, 0x4000u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(addr, _)| *addr == 0x00de_0003));

        // The base class accepts.
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c3_60a0, &args![object, 0x4000u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x00de_0003, vec![shape.addr(), 0x4000])));
    }

    #[test]
    fn save_binary_writes_the_shape_through_the_stream() {
        let mut e = test_engine();
        let object: Ptr<BSMultiBound> = e.new_object();
        e.set(object, BSMultiBound::spShape, Ptr::new(0x6600));
        // The stream object, with its own vtable.
        let stream = shape_with(&mut e, &[(SLOT_STREAM_SAVE_LINK, 0x00de_0004)]);
        e.register(0x00de_0004, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_60f0, &args![object, stream]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (LINK_STUB, vec![object.addr(), stream.addr()]));
        assert_eq!(log[2], (0x00de_0004, vec![stream.addr(), 0x6600]));
    }

    #[test]
    fn is_equal_compares_shape_and_frame_count_ignoring_the_base_result() {
        let mut e = test_engine();
        // The base reports "not equal"; the result is overwritten anyway.
        e.register(NI_OBJECT_IS_EQUAL, |_, _| Ret::default());
        let a: Ptr<BSMultiBound> = e.new_object();
        let b: Ptr<BSMultiBound> = e.new_object();
        for object in [a, b] {
            e.set(object, BSMultiBound::spShape, Ptr::new(0x100));
            e.set(object, BSMultiBound::uiBoundFrameCount, 4);
        }
        assert!(e.call(0x00c3_6130, &args![a, b]).bool());
        e.set(b, BSMultiBound::uiBoundFrameCount, 5);
        assert!(!e.call(0x00c3_6130, &args![a, b]).bool());
        e.set(b, BSMultiBound::uiBoundFrameCount, 4);
        e.set(b, BSMultiBound::spShape, Ptr::new(0x200));
        assert!(!e.call(0x00c3_6130, &args![a, b]).bool());
    }

    #[test]
    fn factory_allocates_and_constructs() {
        let mut e = test_engine();
        e.call_log = Some(vec![]);
        let object = e.call(0x00c3_61a0, &args![]).ptr::<BSMultiBound>();
        assert!(!object.is_null());
        assert_eq!(e.mem.u32(object.addr()), BS_MULTI_BOUND_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(OPERATOR_NEW, vec![0x10])));

        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert!(e
            .call(0x00c3_61a0, &args![])
            .ptr::<BSMultiBound>()
            .is_null());
    }

    #[test]
    fn viewer_strings_forwards_to_the_base_class() {
        let mut e = test_engine();
        let object: Ptr<BSMultiBound> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c3_6220, &args![object, 0x8000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (NI_OBJECT_GET_VIEWER_STRINGS, vec![object.addr(), 0x8000])
        );
    }

    #[test]
    fn point_within_asks_the_shape() {
        let mut e = test_engine();
        let object: Ptr<BSMultiBound> = e.new_object();
        // No shape: false, without a virtual call.
        assert!(!e.call(0x00c3_6240, &args![object, 0x9000u32]).bool());

        let shape = shape_with(&mut e, &[(SLOT_SHAPE_POINT_WITHIN, 0x00de_0005)]);
        e.register_double(0x00de_0005, |_, a| Ret {
            eax: (a[1] == 0x9000) as u32,
            ..Ret::default()
        });
        e.set(object, BSMultiBound::spShape, shape);
        assert!(e.call(0x00c3_6240, &args![object, 0x9000u32]).bool());
        assert!(!e.call(0x00c3_6240, &args![object, 0x9004u32]).bool());
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = test_engine();
        let object: Ptr<BSMultiBound> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e
            .call(0x00c3_6290, &args![object, 0u32])
            .ptr::<BSMultiBound>();
        assert_eq!(back, object);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(addr, _)| *addr == OPERATOR_DELETE_SIZED));

        e.call_log = Some(vec![]);
        e.call(0x00c3_6290, &args![object, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.last().unwrap(),
            &(OPERATOR_DELETE_SIZED, vec![object.addr(), 0x10])
        );
        // Bit 0 alone decides.
        e.call_log = Some(vec![]);
        e.call(0x00c3_6290, &args![object, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(addr, _)| *addr == OPERATOR_DELETE_SIZED));
    }

    #[test]
    fn pointer_release_decrements_and_deletes_at_zero() {
        let mut e = test_engine();
        let holder: Ptr = Ptr::new(e.mem.alloc(4));
        // Empty holder: nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x00c3_62c0, &args![holder]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);

        let shape = shape_with(&mut e, &[(SLOT_DELETE_THIS, 0x00de_0006)]);
        e.register(0x00de_0006, |_, _| Ret::default());
        e.mem.set_u32(holder.addr(), shape.addr());
        // Count 2: decrement only.
        e.mem.set_u32(shape.addr() + 4, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c3_62c0, &args![holder]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(addr, _)| *addr == 0x00de_0006));
        assert_eq!(e.mem.u32(shape.addr() + 4), 1);
        // Count 1: deleted, and the holder keeps its value.
        e.call_log = Some(vec![]);
        e.call(0x00c3_62c0, &args![holder]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x00de_0006, vec![shape.addr()])));
        assert_eq!(e.mem.u32(holder.addr()), shape.addr());
    }

    #[test]
    fn rtti_is_the_class_information_address() {
        let mut e = test_engine();
        let object: Ptr<BSMultiBound> = e.new_object();
        assert_eq!(e.call(0x00c3_6300, &args![object]).u32(), 0x0120_2810);
    }
}
