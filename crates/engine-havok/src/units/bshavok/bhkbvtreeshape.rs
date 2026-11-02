//! `BSHavok/bhkbvtreeshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkBvTreeShape` (0x14 bytes, vtable `0104dbac`) is a `bhkShape` whose Havok object
//! (`phkObject`, `+8`) is a bounding-volume tree. Most of the unit's functions forward to the
//! shape wrapped by the tree's shape collection: they ask the Havok object for its container
//! (virtual `+0x10`), step back 0x10 bytes to the collection, and read the wrapping
//! `bhkShape` from the collection's `+8` (the user data).
//!
//! The exception-unwinding frames (`FS:[0]`) of the functions marked `Translation incomplete`
//! are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkSerializable::Kill` (engine map name, `00c876a0`).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
/// `bhkShape` base functions (engine map names): destructor body (`00c84c50`), `LinkObject`
/// (`00c84cc0`), `LoadBinary` (`00c84d00`), `SaveBinary` (`00c84d50`), `CopyMembers`
/// (`00c84df0`), `CopyCinfo` (`00c84e20`), `Link` (`00c84e50`), `CanShare` (`00c84ec0`) and
/// `GetViewerStrings` (`00c851c0`).
const SHAPE_DESTRUCT: u32 = 0x00c8_4c50;
const SHAPE_LINK_OBJECT: u32 = 0x00c8_4cc0;
const SHAPE_LOAD_BINARY: u32 = 0x00c8_4d00;
const SHAPE_SAVE_BINARY: u32 = 0x00c8_4d50;
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
const SHAPE_COPY_CINFO: u32 = 0x00c8_4e20;
const SHAPE_LINK: u32 = 0x00c8_4e50;
const SHAPE_CAN_SHARE: u32 = 0x00c8_4ec0;
const SHAPE_GET_VIEWER_STRINGS: u32 = 0x00c8_51c0;
/// The base `RegisterStreamables` the unit ends in (`00aa0ad0`, a jump to `00a81fe0`).
const BASE_REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
/// `NiStream` read at the start of `LoadBinary` (`00a645b0`, thiscall on the stream, no stack
/// arguments) and the stream's link-object lookup (`00a63430`, engine map name
/// `NiStream::GetObjectFromLinkID`).
const STREAM_READ: u32 = 0x00a6_45b0;
const STREAM_GET_LINK_OBJECT: u32 = 0x00a6_3430;
/// `NiGetViewerString` (`00a7d330`, cdecl: name) and the string array's resize (`0096ad30`)
/// and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `kernel32` `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The vtable of `bhkBvTreeShape`, the object counter the destructor lowers and
/// `bhkBvTreeShape::ms_RTTI` (its first word is the class name).
const BV_TREE_SHAPE_VTABLE: u32 = 0x0104_dbac;
const BV_TREE_SHAPE_COUNT: u32 = 0x0126_82cc;
const BV_TREE_SHAPE_RTTI: u32 = 0x0126_82d0;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

layout! {
    /// `bhkBvTreeShape` (Xbox PDB, 0x14 bytes): a `bhkShape`.
    pub struct BhkBvTreeShape: 0x14 {
        /// `phkObject` (Xbox PDB): the Havok bounding-volume tree.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB): the Cinfo, as `CreateCinfo` (virtual `+0xb4`) answers it.
        0x0C pInfo: Ptr,
        /// `eMaterial` (Xbox PDB).
        0x10 eMaterial: u32,
    }
}

layout! {
    /// `hkBvTreeShapeCinfo` (Xbox PDB, 8 bytes).
    pub struct HkBvTreeShapeCinfo: 0x08 {
        /// `m_userData` (Xbox PDB).
        0x00 m_userData: u32,
        /// `m_shapeCollection` (Xbox PDB): the Havok shape collection (`hkpShapeCollection*`).
        0x04 m_shapeCollection: Ptr,
    }
}

/// Sets the per-thread word `TLS + 0x2b4` to 0x10, answering the thread block and the old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    (thread, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

/// The container the tree's Havok object answers (virtual `+0x10`), or 0 when `this` or its
/// Havok object is null.
fn container_of(e: &mut Engine, this: u32) -> u32 {
    if this == 0 {
        return 0;
    }
    let tree = e.mem.u32(this + 8);
    if tree == 0 {
        return 0;
    }
    e.vcall(tree, 0x10, &args![]).u32()
}

/// The `bhkShape` that wraps the tree's shape collection: the collection is the container
/// minus 0x10 and the wrapper is its `+8`. Answers 0 when any link of the chain is null.
fn wrapped_shape(e: &mut Engine, this: u32) -> u32 {
    let container = container_of(e, this);
    if container == 0 {
        return 0;
    }
    let collection = container.wrapping_sub(0x10);
    if collection == 0 {
        return 0;
    }
    e.mem.u32(collection + 8)
}

/// Stores the string pointer `text` in the string array: grows the array by its grow-by
/// (`0096ad30`) when the size (`+0xa`) has reached the capacity (`+8`), then stores through
/// `0096ae90` (array, index = the old size, address of the pointer).
fn store_text(e: &mut Engine, array: u32, text: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
}

/// Calls `CreateCinfo` (virtual `+0xb4`, with the address of a scratch bool) and answers the
/// Cinfo.
fn create_cinfo(e: &mut Engine, this: u32) -> Ptr<HkBvTreeShapeCinfo> {
    let cinfo = e.with_stack(4, |e, created| e.vcall(this, 0xb4, &args![created]).u32());
    Ptr::new(cinfo)
}

// Translated from 00ca4da0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca4da0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBvTreeShape::~bhkBvTreeShape` (Xbox PDB): sets the vtable, runs `Kill` (`00c876a0`),
/// lowers the object counter (`012682cc`) and runs the `bhkShape` destructor body
/// (`00c84c50`).
pub fn bhk_bv_tree_shape_destructor(e: &mut Engine, this: Ptr<BhkBvTreeShape>) {
    e.mem.set_u32(this.addr(), BV_TREE_SHAPE_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(BV_TREE_SHAPE_COUNT);
    e.mem.set_u32(BV_TREE_SHAPE_COUNT, count.wrapping_sub(1));
    e.call(SHAPE_DESTRUCT, &args![this]);
}

// Translated from 00ca4e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::CopyCinfo` (engine map name), `(cinfo)`: the base `bhkShape::CopyCinfo`
/// (`00c84e20`), then the Cinfo's shape collection (`+4`) is the tree's container minus 0x10,
/// or 0 when the object or its Havok object is null or the container is null.
pub fn bhk_bv_tree_shape_copy_cinfo(
    e: &mut Engine,
    this: Ptr<BhkBvTreeShape>,
    cinfo: Ptr<HkBvTreeShapeCinfo>,
) {
    e.call(SHAPE_COPY_CINFO, &args![this, cinfo]);
    let container = container_of(e, this.addr());
    let collection = if container != 0 {
        container.wrapping_sub(0x10)
    } else {
        0
    };
    e.set(
        cinfo,
        HkBvTreeShapeCinfo::m_shapeCollection,
        Ptr::new(collection),
    );
}

// Translated from 00ca4e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::CopyMembers` (engine map name), `(clone, process)`: takes the Cinfo
/// (`CreateCinfo`, virtual `+0xb4`); if its shape collection is set and wraps a shape
/// (`+8`) that answers false to `CanShare(process)` (virtual `+0xd0`), the Cinfo's collection
/// becomes the Havok object (`+8`) of that shape's clone (virtual `+0x48`, 0 when the clone is
/// null). Then the base `bhkShape::CopyMembers` (`00c84df0`) with `(this, clone, process)`.
pub fn bhk_bv_tree_shape_copy_members(
    e: &mut Engine,
    this: Ptr<BhkBvTreeShape>,
    clone: Ptr,
    process: Ptr,
) {
    let cinfo = create_cinfo(e, this.addr());
    let collection = e.get(cinfo, HkBvTreeShapeCinfo::m_shapeCollection).addr();
    if collection != 0 {
        let wrapped = e.mem.u32(collection + 8);
        if wrapped != 0 && !e.vcall(wrapped, 0xd0, &args![process]).bool() {
            let copy = e.vcall(wrapped, 0x48, &args![process]).u32();
            let havok = if copy != 0 { e.mem.u32(copy + 8) } else { 0 };
            e.set(
                cinfo,
                HkBvTreeShapeCinfo::m_shapeCollection,
                Ptr::new(havok),
            );
        }
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, clone, process]);
}

// Translated from 00ca4ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::LoadBinary` (engine map name): reads from the stream (`00a645b0`), runs
/// `bhkShape::LoadBinary` (`00c84d00`), then clears the Cinfo's shape collection.
pub fn bhk_bv_tree_shape_load_binary(e: &mut Engine, this: Ptr<BhkBvTreeShape>, stream: Ptr) {
    e.call(STREAM_READ, &args![stream]);
    e.call(SHAPE_LOAD_BINARY, &args![this, stream]);
    let cinfo = create_cinfo(e, this.addr());
    e.set(cinfo, HkBvTreeShapeCinfo::m_shapeCollection, Ptr::new(0));
}

// Translated from 00ca4f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::LinkObject` (engine map name): takes the Cinfo, stores into its shape
/// collection the Havok object (`+8`) of the next linked object the stream hands out
/// (`00a63430`, when not null), then runs `bhkShape::LinkObject` (`00c84cc0`).
pub fn bhk_bv_tree_shape_link_object(e: &mut Engine, this: Ptr<BhkBvTreeShape>, stream: Ptr) {
    let cinfo = create_cinfo(e, this.addr());
    let linked = e.call(STREAM_GET_LINK_OBJECT, &args![stream]).u32();
    if linked != 0 {
        let havok = e.mem.u32(linked + 8);
        e.set(
            cinfo,
            HkBvTreeShapeCinfo::m_shapeCollection,
            Ptr::new(havok),
        );
    }
    e.call(SHAPE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00ca4f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::RegisterStreamables` (engine map name; a `bool` in the Xbox PDB):
/// registers the wrapped shape (virtual `+0x54` on it) when there is one, then the base
/// registration (`00aa0ad0`), whose answer it answers (the exe ends on that call and leaves
/// its `EAX`).
pub fn bhk_bv_tree_shape_register_streamables(
    e: &mut Engine,
    this: Ptr<BhkBvTreeShape>,
    stream: Ptr,
) -> bool {
    let wrapped = wrapped_shape(e, this.addr());
    if wrapped != 0 {
        e.vcall(wrapped, 0x54, &args![stream]);
    }
    e.call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00ca4f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::SaveBinary` (engine map name): calls the stream's virtual `+0x30` with
/// the wrapped shape (0 when there is none), then `bhkShape::SaveBinary` (`00c84d50`).
pub fn bhk_bv_tree_shape_save_binary(e: &mut Engine, this: Ptr<BhkBvTreeShape>, stream: Ptr) {
    let wrapped = wrapped_shape(e, this.addr());
    e.vcall(stream.addr(), 0x30, &args![wrapped]);
    e.call(SHAPE_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00ca4fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::BuildDisplayGeometry` (engine map name), two words: forwards both to the
/// wrapped shape's virtual `+0xd4` when there is one (a tail jump), else does nothing.
pub fn bhk_bv_tree_shape_build_display_geometry(
    e: &mut Engine,
    this: Ptr<BhkBvTreeShape>,
    first: u32,
    second: u32,
) {
    let wrapped = wrapped_shape(e, this.addr());
    if wrapped != 0 {
        e.vcall(wrapped, 0xd4, &args![first, second]);
    }
}

// Translated from 00ca5010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::DestroyDisplayGeometry` (engine map name): the wrapped shape's virtual
/// `+0xd8` when there is one (a tail jump).
pub fn bhk_bv_tree_shape_destroy_display_geometry(e: &mut Engine, this: Ptr<BhkBvTreeShape>) {
    let wrapped = wrapped_shape(e, this.addr());
    if wrapped != 0 {
        e.vcall(wrapped, 0xd8, &args![]);
    }
}

// Translated from 00ca5040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::CanShare` (engine map name), `(other)`: the wrapped shape's virtual
/// `+0xd0` with `other` when there is one (a tail jump, its answer returned), else the base
/// `bhkShape::CanShare` (`00c84ec0`).
pub fn bhk_bv_tree_shape_can_share(e: &mut Engine, this: Ptr<BhkBvTreeShape>, other: Ptr) -> bool {
    let wrapped = wrapped_shape(e, this.addr());
    if wrapped != 0 {
        return e.vcall(wrapped, 0xd0, &args![other]).bool();
    }
    e.call(SHAPE_CAN_SHARE, &args![this, other]).bool()
}

// Translated from 00ca5080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca5080 - the FS:[0] exception-unwinding frame is not modeled.
/// Releases the wrapped shape (unconfirmed name; `Link` calls it): inside the
/// `TLS + 0x2b4 = 0x10` scope, `InterlockedDecrement` of the wrapped shape's reference count
/// (`+4`) and, when that reaches 0, its virtual `+4` (`DeleteThis`).
pub fn fn_00ca5080(e: &mut Engine, this: Ptr<BhkBvTreeShape>) {
    let scope = enter_scope(e);
    let wrapped = wrapped_shape(e, this.addr());
    if wrapped != 0 && e.call(INTERLOCKED_DECREMENT, &args![wrapped + 4]).u32() == 0 {
        e.vcall(wrapped, 4, &args![]);
    }
    leave_scope(e, scope);
}

// Translated from 00ca5130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBvTreeShape::Link` (Xbox PDB `Link`, virtual `+0x90`), one word of which the low byte is
/// tested: when set, the wrapped shape (if there is one) gets `InterlockedIncrement` on its
/// reference count (`+4`); when clear, `00ca5080` releases it. Then the base `bhkShape::Link`
/// (`00c84e50`) with the same word.
pub fn bhk_bv_tree_shape_link(e: &mut Engine, this: Ptr<BhkBvTreeShape>, link: u32) {
    if link & 0xff != 0 {
        let wrapped = wrapped_shape(e, this.addr());
        if wrapped != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![wrapped + 4]);
        }
    } else {
        fn_00ca5080(e, this);
    }
    e.call(SHAPE_LINK, &args![this, link]);
}

// Translated from 00ca5190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca5190 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBvTreeShape::GetViewerStrings` (Xbox PDB): inside the `TLS + 0x2b4 = 0x10` scope, the
/// base strings (`00c851c0`), the class name (`NiGetViewerString` of the first word of the
/// RTTI at `012682d0`) stored in the string array, then the wrapped shape's virtual `+0x60`
/// with the array when there is one.
pub fn bhk_bv_tree_shape_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkBvTreeShape>,
    strings: Ptr,
) {
    let scope = enter_scope(e);
    e.call(SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(BV_TREE_SHAPE_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    store_text(e, strings.addr(), text);
    let wrapped = wrapped_shape(e, this.addr());
    if wrapped != 0 {
        e.vcall(wrapped, 0x60, &args![strings]);
    }
    leave_scope(e, scope);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ca4da0,
            bhk_bv_tree_shape_destructor(Ptr<BhkBvTreeShape>)
        ),
        entry!(
            0x00ca4e10,
            bhk_bv_tree_shape_copy_cinfo(Ptr<BhkBvTreeShape>, Ptr<HkBvTreeShapeCinfo>)
        ),
        entry!(
            0x00ca4e50,
            bhk_bv_tree_shape_copy_members(Ptr<BhkBvTreeShape>, Ptr, Ptr)
        ),
        entry!(
            0x00ca4ec0,
            bhk_bv_tree_shape_load_binary(Ptr<BhkBvTreeShape>, Ptr)
        ),
        entry!(
            0x00ca4f00,
            bhk_bv_tree_shape_link_object(Ptr<BhkBvTreeShape>, Ptr)
        ),
        entry!(
            0x00ca4f40,
            bhk_bv_tree_shape_register_streamables(Ptr<BhkBvTreeShape>, Ptr) -> bool
        ),
        entry!(
            0x00ca4f80,
            bhk_bv_tree_shape_save_binary(Ptr<BhkBvTreeShape>, Ptr)
        ),
        entry!(
            0x00ca4fd0,
            bhk_bv_tree_shape_build_display_geometry(Ptr<BhkBvTreeShape>, u32, u32)
        ),
        entry!(
            0x00ca5010,
            bhk_bv_tree_shape_destroy_display_geometry(Ptr<BhkBvTreeShape>)
        ),
        entry!(
            0x00ca5040,
            bhk_bv_tree_shape_can_share(Ptr<BhkBvTreeShape>, Ptr) -> bool
        ),
        entry!(0x00ca5080, fn_00ca5080(Ptr<BhkBvTreeShape>)),
        entry!(0x00ca5130, bhk_bv_tree_shape_link(Ptr<BhkBvTreeShape>, u32)),
        entry!(
            0x00ca5190,
            bhk_bv_tree_shape_get_viewer_strings(Ptr<BhkBvTreeShape>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const HAVOK_SLOT_10: u32 = 0x7000_0010;
    const SHAPE_SLOT: u32 = 0x7000_0020;
    const CREATE_CINFO_SLOT: u32 = 0x7000_00b4;
    const STREAM_SLOT: u32 = 0x7000_0030;
    const DELETE_SLOT: u32 = 0x7000_0004;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    /// Writes a vtable at `address` whose slots (byte offsets) point at the targets.
    fn put_vtable_at(e: &mut Engine, address: u32, entries: &[(u32, u32)]) {
        let words = entries
            .iter()
            .map(|(offset, _)| offset / 4 + 1)
            .max()
            .unwrap_or(0);
        let mut table = vec![0u32; words as usize];
        for (offset, target) in entries {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(address, &table);
    }

    struct Tree {
        this: u32,
        collection: u32,
        wrapped: u32,
    }

    /// A tree shape whose Havok object answers `collection + 0x10` from its virtual `+0x10`,
    /// where the collection's `+8` is a wrapped shape with the given vtable slots, and whose
    /// virtual `+0xb4` answers a Cinfo at `this + 0x0c`'s target.
    fn tree(e: &mut Engine, wrapped_slots: &[(u32, u32)]) -> Tree {
        put_vtable_at(e, 0x7001_0000, &[(0x10, HAVOK_SLOT_10)]);
        put_vtable_at(e, 0x7001_1000, wrapped_slots);
        put_vtable_at(e, 0x7001_2000, &[(0xb4, CREATE_CINFO_SLOT)]);
        e.register(HAVOK_SLOT_10, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(CREATE_CINFO_SLOT, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        let this = e.mem.alloc(0x14);
        let havok = e.mem.alloc(0x10);
        let collection = e.mem.alloc(0x20);
        let wrapped = e.mem.alloc(0x20);
        let cinfo = e.mem.alloc(8);
        e.mem.set_u32(this, 0x7001_2000);
        e.mem.set_u32(this + 8, havok);
        e.mem.set_u32(this + 0xc, cinfo);
        e.mem.set_u32(havok, 0x7001_0000);
        e.mem.set_u32(havok + 8, collection + 0x10);
        e.mem.set_u32(collection + 8, wrapped);
        e.mem.set_u32(wrapped, 0x7001_1000);
        Tree {
            this,
            collection,
            wrapped,
        }
    }

    fn cinfo_of(e: &Engine, tree: &Tree) -> u32 {
        e.mem.u32(tree.this + 0xc)
    }

    #[test]
    fn bhk_bv_tree_shape_destructor_runs_kill_then_the_shape_destructor() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(SHAPE_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(BV_TREE_SHAPE_COUNT, 3);
        e.call(0x00ca_4da0, &args![this]);
        assert_eq!(e.mem.u32(this), BV_TREE_SHAPE_VTABLE);
        assert_eq!(e.mem.u32(BV_TREE_SHAPE_COUNT), 2);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, SHAPE_DESTRUCT), vec![vec![this]]);
        assert_eq!(
            order_of(&e, &[SERIALIZABLE_KILL, SHAPE_DESTRUCT]),
            vec![SERIALIZABLE_KILL, SHAPE_DESTRUCT]
        );
    }

    #[test]
    fn bhk_bv_tree_shape_copy_cinfo_stores_the_collection() {
        let mut e = logged_engine();
        e.register(SHAPE_COPY_CINFO, |_, _| Ret::default());
        let tree = tree(&mut e, &[]);
        let cinfo = e.mem.alloc(8);
        e.mem.set_u32(cinfo + 4, 0xffff);
        e.call(0x00ca_4e10, &args![tree.this, cinfo]);
        assert_eq!(calls_to(&e, SHAPE_COPY_CINFO), vec![vec![tree.this, cinfo]]);
        assert_eq!(e.mem.u32(cinfo + 4), tree.collection);
        // A null container clears it; so does a missing Havok object.
        let havok = e.mem.u32(tree.this + 8);
        e.mem.set_u32(havok + 8, 0);
        e.call(0x00ca_4e10, &args![tree.this, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        e.mem.set_u32(cinfo + 4, 0xffff);
        e.mem.set_u32(tree.this + 8, 0);
        e.call(0x00ca_4e10, &args![tree.this, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(calls_to(&e, SHAPE_COPY_CINFO).len(), 3);
    }

    #[test]
    fn bhk_bv_tree_shape_copy_members_clones_a_wrapped_shape_that_cannot_be_shared() {
        let mut e = logged_engine();
        e.register(SHAPE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(SHAPE_SLOT, |_, _| Ret::default());
        e.register(SHAPE_SLOT + 1, |_, _| 0x7300_0000u32.into_ret());
        let tree = tree(&mut e, &[(0xd0, SHAPE_SLOT), (0x48, SHAPE_SLOT + 1)]);
        e.map(0x7300_0000, 0x20);
        e.mem.set_u32(0x7300_0008, 0x7300_0010);
        let cinfo = cinfo_of(&e, &tree);
        e.mem.set_u32(cinfo + 4, tree.collection);
        // The wrapped shape is the collection's +8.
        e.mem.set_u32(tree.collection + 8, tree.wrapped);
        e.call(0x00ca_4e50, &args![tree.this, 0x1000u32, 0x2000u32]);
        assert_eq!(calls_to(&e, SHAPE_SLOT), vec![vec![tree.wrapped, 0x2000]]);
        assert_eq!(
            calls_to(&e, SHAPE_SLOT + 1),
            vec![vec![tree.wrapped, 0x2000]]
        );
        assert_eq!(e.mem.u32(cinfo + 4), 0x7300_0010);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![tree.this, 0x1000, 0x2000]]
        );
    }

    #[test]
    fn bhk_bv_tree_shape_copy_members_keeps_a_shareable_shape() {
        let mut e = logged_engine();
        e.register(SHAPE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(SHAPE_SLOT, |_, _| 1u32.into_ret());
        e.register(SHAPE_SLOT + 1, |_, _| 0x7300_0000u32.into_ret());
        let tree = tree(&mut e, &[(0xd0, SHAPE_SLOT), (0x48, SHAPE_SLOT + 1)]);
        let cinfo = cinfo_of(&e, &tree);
        e.mem.set_u32(cinfo + 4, tree.collection);
        e.call(0x00ca_4e50, &args![tree.this, 0x1000u32, 0x2000u32]);
        assert!(calls_to(&e, SHAPE_SLOT + 1).is_empty());
        assert_eq!(e.mem.u32(cinfo + 4), tree.collection);
        // Without a collection nothing is asked; a null clone clears the collection.
        e.mem.set_u32(cinfo + 4, 0);
        e.call(0x00ca_4e50, &args![tree.this, 0x1000u32, 0x2000u32]);
        assert_eq!(calls_to(&e, SHAPE_SLOT).len(), 1);
        e.register(SHAPE_SLOT, |_, _| Ret::default());
        e.register(SHAPE_SLOT + 1, |_, _| Ret::default());
        e.mem.set_u32(cinfo + 4, tree.collection);
        e.call(0x00ca_4e50, &args![tree.this, 0x1000u32, 0x2000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(calls_to(&e, SHAPE_COPY_MEMBERS).len(), 3);
    }

    #[test]
    fn bhk_bv_tree_shape_load_binary_reads_then_clears_the_collection() {
        let mut e = logged_engine();
        e.register(STREAM_READ, |_, _| Ret::default());
        e.register(SHAPE_LOAD_BINARY, |_, _| Ret::default());
        let tree = tree(&mut e, &[]);
        let cinfo = cinfo_of(&e, &tree);
        e.mem.set_u32(cinfo + 4, 0xffff);
        e.call(0x00ca_4ec0, &args![tree.this, 0x2000u32]);
        assert_eq!(calls_to(&e, STREAM_READ), vec![vec![0x2000]]);
        assert_eq!(
            calls_to(&e, SHAPE_LOAD_BINARY),
            vec![vec![tree.this, 0x2000]]
        );
        assert_eq!(
            order_of(&e, &[STREAM_READ, SHAPE_LOAD_BINARY, CREATE_CINFO_SLOT]),
            vec![STREAM_READ, SHAPE_LOAD_BINARY, CREATE_CINFO_SLOT]
        );
        assert_eq!(e.mem.u32(cinfo + 4), 0);
    }

    #[test]
    fn bhk_bv_tree_shape_link_object_takes_the_havok_object_of_the_linked_object() {
        let mut e = logged_engine();
        e.register(STREAM_GET_LINK_OBJECT, |_, _| 0x7300_0000u32.into_ret());
        e.register(SHAPE_LINK_OBJECT, |_, _| Ret::default());
        e.map(0x7300_0000, 0x20);
        e.mem.set_u32(0x7300_0008, 0x7300_0010);
        let tree = tree(&mut e, &[]);
        let cinfo = cinfo_of(&e, &tree);
        e.call(0x00ca_4f00, &args![tree.this, 0x2000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x7300_0010);
        assert_eq!(calls_to(&e, STREAM_GET_LINK_OBJECT), vec![vec![0x2000]]);
        assert_eq!(
            calls_to(&e, SHAPE_LINK_OBJECT),
            vec![vec![tree.this, 0x2000]]
        );
        // No linked object: the Cinfo is left alone.
        e.register(STREAM_GET_LINK_OBJECT, |_, _| Ret::default());
        e.mem.set_u32(cinfo + 4, 0x55);
        e.call(0x00ca_4f00, &args![tree.this, 0x2000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x55);
    }

    #[test]
    fn bhk_bv_tree_shape_register_streamables_registers_the_wrapped_shape() {
        let mut e = logged_engine();
        e.register(BASE_REGISTER_STREAMABLES, |_, _| Ret::default());
        e.register(SHAPE_SLOT, |_, _| Ret::default());
        let tree = tree(&mut e, &[(0x54, SHAPE_SLOT)]);
        e.call(0x00ca_4f40, &args![tree.this, 0x2000u32]);
        assert_eq!(calls_to(&e, SHAPE_SLOT), vec![vec![tree.wrapped, 0x2000]]);
        assert_eq!(
            calls_to(&e, BASE_REGISTER_STREAMABLES),
            vec![vec![tree.this, 0x2000]]
        );
        // Without a wrapped shape only the base runs.
        e.mem.set_u32(tree.collection + 8, 0);
        e.call(0x00ca_4f40, &args![tree.this, 0x2000u32]);
        assert_eq!(calls_to(&e, SHAPE_SLOT).len(), 1);
        assert_eq!(calls_to(&e, BASE_REGISTER_STREAMABLES).len(), 2);
    }

    #[test]
    fn bhk_bv_tree_shape_save_binary_hands_the_wrapped_shape_to_the_stream() {
        let mut e = logged_engine();
        e.register(SHAPE_SAVE_BINARY, |_, _| Ret::default());
        e.register(STREAM_SLOT, |_, _| Ret::default());
        put_vtable_at(&mut e, 0x7001_3000, &[(0x30, STREAM_SLOT)]);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream, 0x7001_3000);
        let tree = tree(&mut e, &[]);
        e.call(0x00ca_4f80, &args![tree.this, stream]);
        assert_eq!(calls_to(&e, STREAM_SLOT), vec![vec![stream, tree.wrapped]]);
        assert_eq!(
            calls_to(&e, SHAPE_SAVE_BINARY),
            vec![vec![tree.this, stream]]
        );
        e.mem.set_u32(tree.this + 8, 0);
        e.call(0x00ca_4f80, &args![tree.this, stream]);
        assert_eq!(calls_to(&e, STREAM_SLOT)[1], vec![stream, 0]);
    }

    #[test]
    fn bhk_bv_tree_shape_build_display_geometry_forwards_both_words() {
        let mut e = logged_engine();
        e.register(SHAPE_SLOT, |_, _| Ret::default());
        let tree = tree(&mut e, &[(0xd4, SHAPE_SLOT)]);
        e.call(0x00ca_4fd0, &args![tree.this, 5u32, 6u32]);
        assert_eq!(calls_to(&e, SHAPE_SLOT), vec![vec![tree.wrapped, 5, 6]]);
        e.mem.set_u32(tree.collection + 8, 0);
        e.call(0x00ca_4fd0, &args![tree.this, 5u32, 6u32]);
        assert_eq!(calls_to(&e, SHAPE_SLOT).len(), 1);
    }

    #[test]
    fn bhk_bv_tree_shape_destroy_display_geometry_calls_the_wrapped_shape() {
        let mut e = logged_engine();
        e.register(SHAPE_SLOT, |_, _| Ret::default());
        let tree = tree(&mut e, &[(0xd8, SHAPE_SLOT)]);
        e.call(0x00ca_5010, &args![tree.this]);
        assert_eq!(calls_to(&e, SHAPE_SLOT), vec![vec![tree.wrapped]]);
        e.mem.set_u32(tree.this + 8, 0);
        e.call(0x00ca_5010, &args![tree.this]);
        assert_eq!(calls_to(&e, SHAPE_SLOT).len(), 1);
    }

    #[test]
    fn bhk_bv_tree_shape_can_share_asks_the_wrapped_shape_or_the_base() {
        let mut e = logged_engine();
        e.register(SHAPE_SLOT, |_, _| 1u32.into_ret());
        e.register(SHAPE_CAN_SHARE, |_, _| 0u32.into_ret());
        let tree = tree(&mut e, &[(0xd0, SHAPE_SLOT)]);
        assert!(e.call(0x00ca_5040, &args![tree.this, 0x2000u32]).bool());
        assert_eq!(calls_to(&e, SHAPE_SLOT), vec![vec![tree.wrapped, 0x2000]]);
        assert!(calls_to(&e, SHAPE_CAN_SHARE).is_empty());
        e.mem.set_u32(tree.collection + 8, 0);
        assert!(!e.call(0x00ca_5040, &args![tree.this, 0x2000u32]).bool());
        assert_eq!(calls_to(&e, SHAPE_CAN_SHARE), vec![vec![tree.this, 0x2000]]);
        e.register(SHAPE_CAN_SHARE, |_, _| 1u32.into_ret());
        assert!(e.call(0x00ca_5040, &args![tree.this, 0x2000u32]).bool());
    }

    #[test]
    fn fn_00ca5080_deletes_the_wrapped_shape_when_its_count_reaches_zero() {
        let mut e = logged_engine();
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.register(DELETE_SLOT, |_, _| Ret::default());
        let tree = tree(&mut e, &[(4, DELETE_SLOT)]);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00ca_5080, &args![tree.this]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![tree.wrapped + 4]]
        );
        assert!(calls_to(&e, DELETE_SLOT).is_empty());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.call(0x00ca_5080, &args![tree.this]);
        assert_eq!(calls_to(&e, DELETE_SLOT), vec![vec![tree.wrapped]]);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // No wrapped shape: nothing happens.
        e.mem.set_u32(tree.collection + 8, 0);
        e.call(0x00ca_5080, &args![tree.this]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 2);
    }

    #[test]
    fn bhk_bv_tree_shape_link_counts_up_or_releases_then_runs_the_base() {
        let mut e = logged_engine();
        e.register(INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.register(SHAPE_LINK, |_, _| Ret::default());
        let tree = tree(&mut e, &[]);
        e.call(0x00ca_5130, &args![tree.this, 0x101u32]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![tree.wrapped + 4]]
        );
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(calls_to(&e, SHAPE_LINK), vec![vec![tree.this, 0x101]]);
        // A clear low byte releases (the full word is still passed to the base).
        e.call(0x00ca_5130, &args![tree.this, 0x100u32]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![tree.wrapped + 4]]
        );
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT).len(), 1);
        assert_eq!(calls_to(&e, SHAPE_LINK)[1], vec![tree.this, 0x100]);
        // Setting the link without a wrapped shape counts nothing.
        e.mem.set_u32(tree.collection + 8, 0);
        e.call(0x00ca_5130, &args![tree.this, 1u32]);
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT).len(), 1);
        assert_eq!(calls_to(&e, SHAPE_LINK).len(), 3);
    }

    #[test]
    fn bhk_bv_tree_shape_get_viewer_strings_adds_the_class_name_and_asks_the_wrapped_shape() {
        let mut e = logged_engine();
        e.register(SHAPE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        e.register(SHAPE_SLOT, |_, _| Ret::default());
        e.mem.set_u32(BV_TREE_SHAPE_RTTI, 0x4444);
        let tree = tree(&mut e, &[(0x60, SHAPE_SLOT)]);
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 4);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00ca_5190, &args![tree.this, array]);
        assert_eq!(
            calls_to(&e, SHAPE_GET_VIEWER_STRINGS),
            vec![vec![tree.this, array]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4444]]);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(calls_to(&e, SHAPE_SLOT), vec![vec![tree.wrapped, array]]);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // Without a wrapped shape only the name is stored.
        e.mem.set_u32(tree.collection + 8, 0);
        e.call(0x00ca_5190, &args![tree.this, array]);
        assert_eq!(calls_to(&e, SHAPE_SLOT).len(), 1);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING).len(), 2);
    }

    #[test]
    fn bhk_bv_tree_shape_register_streamables_answers_the_base_registration() {
        let mut e = logged_engine();
        e.register(BASE_REGISTER_STREAMABLES, |_, _| true.into_ret());
        e.register(SHAPE_SLOT, |_, _| Ret::default());
        let tree = tree(&mut e, &[(0x54, SHAPE_SLOT)]);
        assert!(e.call(0x00ca_4f40, &args![tree.this, 0x2000u32]).bool());
        e.register(BASE_REGISTER_STREAMABLES, |_, _| false.into_ret());
        assert!(!e.call(0x00ca_4f40, &args![tree.this, 0x2000u32]).bool());
    }
}
