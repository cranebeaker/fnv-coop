//! `BSHavok/bhkbreakableconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit has three classes. `bhkBreakableConstraint` (0x10 bytes, vtable `010c98bc`) is a
//! `bhkConstraint`: `+8` the Havok constraint (`phkObject`), `+0xc` `pInfo`, the pointer to the
//! Cinfo it creates. The Cinfo (`hkBreakableConstraintCinfo`, 0x14 bytes, vtable `010c99e0`)
//! keeps its `hkpConstraintData` at `+4`. That data (`ahkpBreakableConstraintData`, 0x28 bytes,
//! vtable `010c999c`) holds the wrapped child constraint data at `+0x18`, the threshold at
//! `+0x20` and the remove-when-broken flag at `+0x24`.
//!
//! The exception-unwinding frames (`FS:[0]`) of the functions marked `Translation incomplete`
//! are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `operator new` (`00aa13e0`, cdecl size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The memory manager (`011f6238`), allocating through `00aa3e40` (thiscall, size) and freeing
/// through `00aa4060` (thiscall, pointer).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `bhkSerializable::Kill` (engine map name, `00c876a0`) and `~bhkConstraint` (`00c89640`).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CONSTRAINT_DESTRUCT: u32 = 0x00c8_9640;
/// `bhkConstraint::LoadBinary` (`00c896b0`, a jump to `bhkSerializable::LoadBinary`),
/// `LinkObject` (`00c89860`), `CopyMembers` (`00c89730`) and `CopyCinfo` (`00c89c80`).
const CONSTRAINT_LOAD_BINARY: u32 = 0x00c8_96b0;
const CONSTRAINT_LINK_OBJECT: u32 = 0x00c8_9860;
const CONSTRAINT_COPY_MEMBERS: u32 = 0x00c8_9730;
const CONSTRAINT_COPY_CINFO: u32 = 0x00c8_9c80;
/// The base constructor of the 0x10-byte serializable object (`00c87c30`).
const OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable::CloneCritLock` / `CloneCritUnlock` (engine map names).
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `hkConstraintCinfo` (engine map names): `CloneData` (`00c8a1b0`), `LoadBinary` (`00c8a230`),
/// `SaveBinary` (`00c8a280`), `GetViewerStrings` (`00c8a560`) and `SetData` (`00c89ac0`).
const CINFO_CLONE_DATA: u32 = 0x00c8_a1b0;
const CINFO_LOAD_BINARY: u32 = 0x00c8_a230;
const CINFO_SAVE_BINARY: u32 = 0x00c8_a280;
const CINFO_GET_VIEWER_STRINGS: u32 = 0x00c8_a560;
const CINFO_SET_DATA: u32 = 0x00c8_9ac0;
/// `bhkConstraintManager` (engine map names): `Clone` (`00cf4a30`, cdecl data and process),
/// `GetInfo` (`00cf4840`, cdecl type), `LoadBinary` (`00cf4af0`, cdecl stream, answers the
/// data); `00cf4bc0` (cdecl stream and data) writes a constraint data.
const MANAGER_CLONE: u32 = 0x00cf_4a30;
const MANAGER_GET_INFO: u32 = 0x00cf_4840;
const MANAGER_LOAD_BINARY: u32 = 0x00cf_4af0;
const MANAGER_SAVE_BINARY: u32 = 0x00cf_4bc0;
/// Reference count increment and decrement of a Havok object (`00c90510`, `00c905b0`; the
/// engine map does not name them).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpBridgeConstraintAtom::init` (engine map name, thiscall on the atom at `data + 0xc`, one
/// argument: the data).
const BRIDGE_ATOM_INIT: u32 = 0x00d2_1ce0;
/// `NiStream` (engine map names): `00a63460` (`GetNumberOfLinkIDs`) and `00a63430`
/// (`GetObjectFromLinkID`) each step one of the stream's link cursors.
const STREAM_STEP_LINK_ARRAY: u32 = 0x00a6_3460;
const STREAM_GET_LINK_OBJECT: u32 = 0x00a6_3430;
/// `NiGetViewerString` with a float (`00a7d550`) and with a bool (`00a7d370`), both cdecl
/// (name, value); the string array's resize (`0096ad30`) and store (`0096ae90`).
const GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"Threshold"` and `"RemIfBroke"`.
const THRESHOLD_LABEL: u32 = 0x010c_9a04;
const REMOVE_IF_BROKEN_LABEL: u32 = 0x010c_99f8;
/// The threshold the data starts with: a `float` (10.0) loaded through the x87.
const DEFAULT_THRESHOLD: u32 = 0x0101_7b78;
/// Vtables: the constraint, its serializable base, the Cinfo and its base, the data and its
/// two bases.
const CONSTRAINT_VTABLE: u32 = 0x010c_98bc;
const SERIALIZABLE_VTABLE: u32 = 0x0108_febc;
const CINFO_VTABLE: u32 = 0x010c_99e0;
const CINFO_BASE_VTABLE: u32 = 0x0108_ffb4;
const DATA_VTABLE: u32 = 0x010c_999c;
const DATA_BASE_VTABLE: u32 = 0x010d_0a1c;
const DATA_ROOT_VTABLE: u32 = 0x0102_e388;
/// Object counters: the serializable base's (`01268188`, incremented by its construction) and
/// the constraint's (`012685dc`).
const SERIALIZABLE_COUNT: u32 = 0x0126_8188;
const CONSTRAINT_COUNT: u32 = 0x0126_85dc;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

layout! {
    /// `bhkBreakableConstraint` (0x10 bytes) as a `bhkConstraint` (Xbox PDB field names).
    pub struct BhkBreakableConstraint: 0x10 {
        /// `phkObject` (Xbox PDB): the Havok constraint.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB): the Cinfo, stored as the address 4 bytes into its block.
        0x0C pInfo: Ptr,
    }
}

layout! {
    /// `hkBreakableConstraintCinfo` (0x14 bytes, an `hkConstraintCinfo`, Xbox PDB).
    pub struct HkBreakableConstraintCinfo: 0x14 {
        /// `pData` (Xbox PDB): the `hkpConstraintData*`.
        0x04 pData: Ptr,
    }
}

layout! {
    /// `ahkpBreakableConstraintData` (0x28 bytes, an `hkpConstraintData`). The fields after
    /// `m_userData` have unconfirmed names.
    pub struct AhkpBreakableConstraintData: 0x28 {
        /// `m_memSizeAndFlags` (Xbox PDB, `hkReferencedObject`).
        0x04 m_memSizeAndFlags: u16,
        /// `m_referenceCount` (Xbox PDB).
        0x06 m_referenceCount: u16,
        /// `m_userData` (Xbox PDB, `hkpConstraintData`).
        0x08 m_userData: u32,
        /// Unconfirmed: the first member of the embedded bridge atom (a 16-bit type, set to 1).
        0x0C atomType: u16,
        /// Unconfirmed: the wrapped child `hkpConstraintData*`.
        0x18 childData: Ptr,
        /// Unconfirmed: the first 16-bit value of the child's runtime info.
        0x1C runtimeInfoA: u16,
        /// Unconfirmed: the second 16-bit value of the child's runtime info.
        0x1E runtimeInfoB: u16,
        /// Unconfirmed: the break threshold (`GetViewerStrings` labels it `Threshold`).
        0x20 threshold: f32,
        /// Unconfirmed: the remove-when-broken flag (labelled `RemIfBroke`).
        0x24 removeWhenBroken: u8,
        /// Unconfirmed: a second flag byte, cleared with the first.
        0x25 flagB: u8,
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

/// `FLD dword` / `FSTP dword` copy of one float.
fn copy_float(e: &mut Engine, destination: u32, source: u32) {
    let bits = x87_quiet_bits(e.mem.u32(source));
    e.mem.set_u32(destination, bits);
}

/// One read through the stream's input object (`stream + 0x24c`, function pointer at `+8`
/// called as `(input, buffer, size, &size, 1)`), advancing the object's position (`+4`).
fn read_from_stream(e: &mut Engine, stream: u32, destination: u32, size: u32) {
    let input = e.mem.u32(stream + 0x24c);
    let reader = e.mem.u32(input + 8);
    e.with_stack(4, |e, size_slot| {
        e.mem.set_u32(size_slot.addr(), size);
        let read = e
            .call(reader, &args![input, destination, size, size_slot, 1u32])
            .u32();
        let position = e.mem.u32(input + 4);
        e.mem.set_u32(input + 4, position.wrapping_add(read));
    });
}

/// One write through the stream's output object (`stream + 0x250`, function pointer at
/// `+0xc`, the same argument shape).
fn write_to_stream(e: &mut Engine, stream: u32, source: u32, size: u32) {
    let output = e.mem.u32(stream + 0x250);
    let writer = e.mem.u32(output + 0xc);
    e.with_stack(4, |e, size_slot| {
        e.mem.set_u32(size_slot.addr(), size);
        let written = e
            .call(writer, &args![output, source, size, size_slot, 1u32])
            .u32();
        let position = e.mem.u32(output + 4);
        e.mem.set_u32(output + 4, position.wrapping_add(written));
    });
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

/// The inline construction of an `ahkpBreakableConstraintData` that `CloneData` and
/// `CreateData` share: a 0x28-byte block from the memory router's allocator (virtual slot 4)
/// with its 16-bit size stored at `+4`, the base constructor (`00d4df30`, no child), the
/// threshold set to the `float` at `01017b78`, the data vtable and both flag bytes cleared.
fn new_breakable_data(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let data = e.vcall(allocator, 4, &args![0x28u32]).u32();
    e.mem.set_u16(data + 4, 0x28);
    ahkp_breakable_constraint_data_constructor(e, Ptr::new(data), Ptr::new(0));
    copy_float(e, data + 0x20, DEFAULT_THRESHOLD);
    e.mem.set_u32(data, DATA_VTABLE);
    e.mem.set_u8(data + 0x24, 0);
    e.mem.set_u8(data + 0x25, 0);
    data
}

/// Allocates and constructs a `bhkBreakableConstraint`: 0x10 bytes, the base constructor, the
/// serializable vtable, `+0xc` cleared and both object counters raised, then the constraint
/// vtable. Answers 0 when the allocation fails.
fn new_breakable_constraint(e: &mut Engine) -> u32 {
    let object = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    e.call(OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, SERIALIZABLE_VTABLE);
    e.mem.set_u32(object + 0xc, 0);
    let count = e.mem.u32(SERIALIZABLE_COUNT);
    e.mem.set_u32(SERIALIZABLE_COUNT, count.wrapping_add(1));
    e.mem.set_u32(object, CONSTRAINT_VTABLE);
    let count = e.mem.u32(CONSTRAINT_COUNT);
    e.mem.set_u32(CONSTRAINT_COUNT, count.wrapping_add(1));
    object
}

// Translated from 00cc48b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_e000`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cc48b0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_e000, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00cc48e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBreakableConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc47f0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_breakable_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_47f0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc4860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBreakableConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_85e0.
pub fn bhk_breakable_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_85e0
}

// Translated from 00ca98d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBreakableConstraint::LoadBinary` (Xbox PDB; the engine map names this address after
/// the ball-and-socket constraint, whose identical thunk the exe shares): a jump to
/// `bhkConstraint::LoadBinary` (`00c896b0`) with the same `(this, stream)`.
pub fn bhk_breakable_constraint_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(CONSTRAINT_LOAD_BINARY, &args![this, stream]);
}

// Translated from 00cc47f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc47f0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBreakableConstraint::~bhkBreakableConstraint` (Xbox PDB): sets the constraint vtable,
/// runs `Kill` (`00c876a0`), lowers the constraint object counter (`012685dc`) and runs
/// `~bhkConstraint` (`00c89640`).
pub fn bhk_breakable_constraint_destructor(e: &mut Engine, this: Ptr<BhkBreakableConstraint>) {
    e.mem.set_u32(this.addr(), CONSTRAINT_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(CONSTRAINT_COUNT);
    e.mem.set_u32(CONSTRAINT_COUNT, count.wrapping_sub(1));
    e.call(CONSTRAINT_DESTRUCT, &args![this]);
}

// Translated from 00cc4870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBreakableConstraint::LinkObject` (Xbox PDB): calls `CreateCinfo` (virtual `+0xb4`, with
/// the address of a scratch bool), `bhkConstraint::LinkObject` (`00c89860`), then steps the
/// stream's link cursors (`00a63460`, `00a63430` twice); their results are not used.
pub fn bhk_breakable_constraint_link_object(
    e: &mut Engine,
    this: Ptr<BhkBreakableConstraint>,
    stream: Ptr,
) {
    e.with_stack(4, |e, scratch| {
        e.vcall(this.addr(), 0xb4, &args![scratch.addr() + 3]);
    });
    e.call(CONSTRAINT_LINK_OBJECT, &args![this, stream]);
    e.call(STREAM_STEP_LINK_ARRAY, &args![stream]);
    e.call(STREAM_GET_LINK_OBJECT, &args![stream]);
    e.call(STREAM_GET_LINK_OBJECT, &args![stream]);
}

// Translated from 00cc4900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4900 - the FS:[0] exception-unwinding frame is not modeled.
/// Allocates and constructs a `bhkBreakableConstraint` (unconfirmed name; the Xbox PDB has a
/// static `CreateObject` for the class). Answers the object, or 0 when the allocation failed.
pub fn fn_00cc4900(e: &mut Engine) -> Ptr {
    Ptr::new(new_breakable_constraint(e))
}

// Translated from 00cc4990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the Cinfo block (unconfirmed name): clears `+4` (the data), sets `+8` to 1
/// and clears `+0xc` and `+0x10`, then stores the Cinfo vtable. Answers `this`.
pub fn fn_00cc4990(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 4, 0);
    e.mem.set_u32(me + 8, 1);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me, CINFO_VTABLE);
    this
}

// Translated from 00cc49b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc49b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `ahkpBreakableConstraintData::SetChild` (Xbox PDB): inside the `TLS + 0x2b4 = 0x10` scope,
/// adds a reference to the new child (`00c90510`, when not null), drops the old child's
/// (`00c905b0`), stores the child at `+0x18`, asks it for its runtime info through virtual
/// `+0x24` (arguments: the byte 1 and the address of an 8-byte result), keeps the low 16 bits
/// of the two result words at `+0x1c` and `+0x1e` and re-initialises the bridge atom at
/// `+0xc` (`00d21ce0`). The child is used without a null check.
pub fn ahkp_breakable_constraint_data_set_child(
    e: &mut Engine,
    this: Ptr<AhkpBreakableConstraintData>,
    child: Ptr,
) {
    let scope = enter_scope(e);
    if child.addr() != 0 {
        e.call(ADD_REFERENCE, &args![child]);
    }
    let old = e.get(this, AhkpBreakableConstraintData::childData);
    if old.addr() != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.set(this, AhkpBreakableConstraintData::childData, child);
    // The pushed word is the byte 1 followed by whatever the register held; only the byte is
    // read.
    let (first, second) = e.with_stack(8, |e, info| {
        e.vcall(child.addr(), 0x24, &args![1u32, info]);
        (e.mem.u16(info.addr()), e.mem.u16(info.addr() + 4))
    });
    e.set(this, AhkpBreakableConstraintData::runtimeInfoB, second);
    e.set(this, AhkpBreakableConstraintData::runtimeInfoA, first);
    e.call(BRIDGE_ATOM_INIT, &args![this.addr() + 0xc, this]);
    leave_scope(e, scope);
}

// Translated from 00cc4a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4a80 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkBreakableConstraintCinfo::CloneData` (Xbox PDB), `(data, process)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, takes the Cinfo's own data as the source and, when `data` is
/// null, builds a new one. Clones the source's child through the constraint manager
/// (`00cf4a30`) and sets it as the new data's child (releasing the clone's reference); copies
/// the threshold (x87 copy) and the remove-when-broken byte; then runs
/// `hkConstraintCinfo::CloneData` (`00c8a1b0`) with the new data.
pub fn hk_breakable_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr<HkBreakableConstraintCinfo>,
    data: Ptr,
    process: Ptr,
) {
    let scope = enter_scope(e);
    let source = e.get(this, HkBreakableConstraintCinfo::pData).addr();
    let mut data = data.addr();
    if data == 0 {
        data = new_breakable_data(e);
    }
    let child = e.mem.u32(source + 0x18);
    if child != 0 {
        let cloned = e.call(MANAGER_CLONE, &args![child, process]).u32();
        if cloned != 0 {
            ahkp_breakable_constraint_data_set_child(e, Ptr::new(data), Ptr::new(cloned));
            e.call(REMOVE_REFERENCE, &args![cloned]);
        }
    }
    copy_float(e, data + 0x20, source + 0x20);
    let flag = e.mem.u8(source + 0x24);
    e.mem.set_u8(data + 0x24, flag);
    e.call(CINFO_CLONE_DATA, &args![this, data, process]);
    leave_scope(e, scope);
}

// Translated from 00cc4b90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4b90 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkBreakableConstraintCinfo::CreateData` (Xbox PDB): inside the `TLS + 0x2b4 = 0x10`
/// scope, builds the data (as `CloneData` does) when the Cinfo has none. Answers 0x28, the
/// size of the data.
pub fn hk_breakable_constraint_cinfo_create_data(
    e: &mut Engine,
    this: Ptr<HkBreakableConstraintCinfo>,
) -> u32 {
    let scope = enter_scope(e);
    if e.get(this, HkBreakableConstraintCinfo::pData).addr() == 0 {
        let data = new_breakable_data(e);
        e.set(this, HkBreakableConstraintCinfo::pData, Ptr::new(data));
    }
    leave_scope(e, scope);
    0x28
}

// Translated from 00cc4c60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4c60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBreakableConstraint::CreateClone` (Xbox PDB): inside the `TLS + 0x2b4 = 0x10` scope
/// and the clone lock (`00c876f0`, `00c87700`), constructs a new constraint (as `00cc4900`)
/// and runs `bhkConstraint::CopyMembers` (`00c89730`, `(this, clone, process)`) on it.
/// Answers the clone (0 when the allocation failed; the copy still runs).
pub fn bhk_breakable_constraint_create_clone(
    e: &mut Engine,
    this: Ptr<BhkBreakableConstraint>,
    process: Ptr,
) -> Ptr {
    let scope = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = new_breakable_constraint(e);
    e.call(CONSTRAINT_COPY_MEMBERS, &args![this, clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, scope);
    Ptr::new(clone)
}

// Translated from 00cc4d40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4d40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBreakableConstraint::CreateCinfo` (Xbox PDB), `(created)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, writes whether there was no Cinfo yet to the byte at `created`;
/// if so, allocates a 0x14-byte Cinfo block from the memory manager (`00aa3e40`), initialises
/// it as `00cc4990` does and stores the address 4 bytes into the block at `+0xc`; then, when
/// the constraint has its Havok constraint (`+8`), runs `CopyCinfo` (`00c89c80`) with the
/// block's address (0 for none). Answers the stored Cinfo pointer.
pub fn bhk_breakable_constraint_create_cinfo(
    e: &mut Engine,
    this: Ptr<BhkBreakableConstraint>,
    created: Ptr,
) -> Ptr {
    let scope = enter_scope(e);
    let is_new = e.get(this, BhkBreakableConstraint::pInfo).addr() == 0;
    e.mem.set_u8(created.addr(), is_new as u8);
    if is_new {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
            .u32();
        let info = if block != 0 {
            e.mem.set_u32(block + 8, 1);
            e.mem.set_u32(block + 0xc, 0);
            e.mem.set_u32(block + 0x10, 0);
            e.mem.set_u32(block, CINFO_VTABLE);
            e.mem.set_u32(block + 4, 0);
            block + 4
        } else {
            0
        };
        e.set(this, BhkBreakableConstraint::pInfo, Ptr::new(info));
        if e.get(this, BhkBreakableConstraint::phkObject).addr() != 0 {
            let base = if info != 0 { info - 4 } else { 0 };
            e.call(CONSTRAINT_COPY_CINFO, &args![this, base]);
        }
    }
    let info = e.get(this, BhkBreakableConstraint::pInfo);
    leave_scope(e, scope);
    info
}

// Translated from 00cc4e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBreakableConstraintCinfo::LoadBinary` (Xbox PDB): the base `LoadBinary` (`00c8a230`),
/// then the child data read by the constraint manager (`00cf4af0`) becomes the data's child
/// (`00cc49b0`; the manager's reference is released), then the threshold (4 bytes at
/// `data + 0x20`) and the remove-when-broken byte (`data + 0x24`) are read through the
/// stream's input object (`stream + 0x24c`).
pub fn hk_breakable_constraint_cinfo_load_binary(
    e: &mut Engine,
    this: Ptr<HkBreakableConstraintCinfo>,
    stream: Ptr,
) {
    e.call(CINFO_LOAD_BINARY, &args![this, stream]);
    let data = e.get(this, HkBreakableConstraintCinfo::pData).addr();
    let child = e.call(MANAGER_LOAD_BINARY, &args![stream]).u32();
    ahkp_breakable_constraint_data_set_child(e, Ptr::new(data), Ptr::new(child));
    e.call(REMOVE_REFERENCE, &args![child]);
    read_from_stream(e, stream.addr(), data + 0x20, 4);
    read_from_stream(e, stream.addr(), data + 0x24, 1);
}

// Translated from 00cc4e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBreakableConstraintCinfo::SaveBinary` (Xbox PDB): the base `SaveBinary` (`00c8a280`),
/// the child data written by the constraint manager (`00cf4bc0`, `(stream, child)`), then the
/// threshold (4 bytes) and the remove-when-broken byte written through the stream's output
/// object (`stream + 0x250`).
pub fn hk_breakable_constraint_cinfo_save_binary(
    e: &mut Engine,
    this: Ptr<HkBreakableConstraintCinfo>,
    stream: Ptr,
) {
    e.call(CINFO_SAVE_BINARY, &args![this, stream]);
    let data = e.get(this, HkBreakableConstraintCinfo::pData).addr();
    let child = e.mem.u32(data + 0x18);
    e.call(MANAGER_SAVE_BINARY, &args![stream, child]);
    write_to_stream(e, stream.addr(), data + 0x20, 4);
    write_to_stream(e, stream.addr(), data + 0x24, 1);
}

// Translated from 00cc4f00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4f00 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkBreakableConstraintCinfo::GetViewerStrings` (Xbox PDB): inside the `TLS + 0x2b4 = 0x10`
/// scope, the base strings (`00c8a560`), `"Threshold"` with the data's threshold (0.0 without
/// data; x87 copy) and `"RemIfBroke"` with the flag are stored in the string array; then, if
/// the data has a child, the constraint manager's info for the child's type (virtual `+0x20`,
/// `00cf4840`) is bound to the child (`00c89ac0`), asked for its strings (virtual `+0x14`) and
/// deleted (virtual `+0`, flag 1).
pub fn hk_breakable_constraint_cinfo_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<HkBreakableConstraintCinfo>,
    strings: Ptr,
) {
    let scope = enter_scope(e);
    e.call(CINFO_GET_VIEWER_STRINGS, &args![this, strings]);
    let data = e.get(this, HkBreakableConstraintCinfo::pData).addr();
    let threshold = if data != 0 {
        x87_quiet_bits(e.mem.u32(data + 0x20))
    } else {
        0
    };
    let text = e
        .call(GET_VIEWER_STRING_FLOAT, &args![THRESHOLD_LABEL, threshold])
        .u32();
    store_text(e, strings.addr(), text);
    let data = e.get(this, HkBreakableConstraintCinfo::pData).addr();
    let flag = data != 0 && e.mem.u8(data + 0x24) != 0;
    let text = e
        .call(
            GET_VIEWER_STRING_BOOL,
            &args![REMOVE_IF_BROKEN_LABEL, flag as u32],
        )
        .u32();
    store_text(e, strings.addr(), text);
    let data = e.get(this, HkBreakableConstraintCinfo::pData).addr();
    let child = if data != 0 { e.mem.u32(data + 0x18) } else { 0 };
    if child != 0 {
        let kind = e.vcall(child, 0x20, &args![]).u32();
        let info = e.call(MANAGER_GET_INFO, &args![kind]).u32();
        if info != 0 {
            e.call(CINFO_SET_DATA, &args![info, child]);
            e.vcall(info, 0x14, &args![strings]);
            e.vcall(info, 0, &args![1u32]);
        }
    }
    leave_scope(e, scope);
}

// Translated from 00cc8920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the Cinfo (slot 0 of the vtable at `010c99e0`; the engine map
/// names it after `hkBallAndSocketConstraintCinfo`, whose `??_G` shares the Xbox address
/// `82b840c0` with `hkBreakableConstraintCinfo`'s): stores the base Cinfo vtable, drops the
/// data (`00c89ac0` with null) and, when bit 0 of `flags` is set, frees the block through the
/// memory manager (`00aa4060`). Answers `this`.
pub fn hk_breakable_constraint_cinfo_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), CINFO_BASE_VTABLE);
    e.call(CINFO_SET_DATA, &args![this, 0u32]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00d4df30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ahkpBreakableConstraintData::ahkpBreakableConstraintData` (Xbox PDB), `(child)`: sets the
/// 16-bit reference count and atom type to 1, the user data to 0, the vtable `010d0a1c`, the
/// threshold to the `float` at `01017b78` (x87 copy) and both flag bytes to 0, stores `child`
/// at `+0x18`, and for a non-null child adds a reference to it and reads its runtime info
/// (virtual `+0x24`, arguments the byte 1 and the address of an 8-byte result), keeping the
/// low 16 bits of both words at `+0x1c` and `+0x1e`. Always initialises the bridge atom at
/// `+0xc` (`00d21ce0`). Answers `this`.
pub fn ahkp_breakable_constraint_data_constructor(
    e: &mut Engine,
    this: Ptr<AhkpBreakableConstraintData>,
    child: Ptr,
) -> Ptr {
    let threshold = x87_quiet_bits(e.mem.u32(DEFAULT_THRESHOLD));
    e.set(this, AhkpBreakableConstraintData::m_referenceCount, 1);
    e.set(this, AhkpBreakableConstraintData::m_userData, 0);
    e.mem.set_u32(this.addr(), DATA_BASE_VTABLE);
    e.set(this, AhkpBreakableConstraintData::atomType, 1);
    e.mem.set_u32(this.addr() + 0x20, threshold);
    e.set(this, AhkpBreakableConstraintData::childData, child);
    e.set(this, AhkpBreakableConstraintData::removeWhenBroken, 0);
    e.set(this, AhkpBreakableConstraintData::flagB, 0);
    if child.addr() != 0 {
        e.call(ADD_REFERENCE, &args![child]);
        let (first, second) = e.with_stack(8, |e, info| {
            e.vcall(child.addr(), 0x24, &args![1u32, info]);
            (e.mem.u16(info.addr()), e.mem.u16(info.addr() + 4))
        });
        e.set(this, AhkpBreakableConstraintData::runtimeInfoA, first);
        e.set(this, AhkpBreakableConstraintData::runtimeInfoB, second);
    }
    e.call(BRIDGE_ATOM_INIT, &args![this.addr() + 0xc, this]);
    this.cast()
}

// Translated from 00d4e000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the breakable constraint data (unconfirmed name): stores the data base
/// vtable (`010d0a1c`), drops the child (`00c905b0`, called even for null) and stores the
/// root vtable (`0102e388`).
pub fn fn_00d4e000(e: &mut Engine, this: Ptr<AhkpBreakableConstraintData>) {
    let child = e.get(this, AhkpBreakableConstraintData::childData);
    e.mem.set_u32(this.addr(), DATA_BASE_VTABLE);
    e.call(REMOVE_REFERENCE, &args![child]);
    e.mem.set_u32(this.addr(), DATA_ROOT_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca98d0, bhk_breakable_constraint_load_binary(Ptr, Ptr)),
        entry!(
            0x00cc47f0,
            bhk_breakable_constraint_destructor(Ptr<BhkBreakableConstraint>)
        ),
        entry!(0x00cc4860, bhk_breakable_constraint_get_r_t_t_i() -> u32),
        entry!(
            0x00cc4870,
            bhk_breakable_constraint_link_object(Ptr<BhkBreakableConstraint>, Ptr)
        ),
        entry!(0x00cc_48b0, deleting_destructor_00cc48b0(Ptr, u8) -> Ptr),
        entry!(0x00cc48e0, bhk_breakable_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc4900, fn_00cc4900() -> Ptr),
        entry!(0x00cc4990, fn_00cc4990(Ptr) -> Ptr),
        entry!(
            0x00cc49b0,
            ahkp_breakable_constraint_data_set_child(Ptr<AhkpBreakableConstraintData>, Ptr)
        ),
        entry!(
            0x00cc4a80,
            hk_breakable_constraint_cinfo_clone_data(Ptr<HkBreakableConstraintCinfo>, Ptr, Ptr)
        ),
        entry!(
            0x00cc4b90,
            hk_breakable_constraint_cinfo_create_data(Ptr<HkBreakableConstraintCinfo>) -> u32
        ),
        entry!(
            0x00cc4c60,
            bhk_breakable_constraint_create_clone(Ptr<BhkBreakableConstraint>, Ptr) -> Ptr
        ),
        entry!(
            0x00cc4d40,
            bhk_breakable_constraint_create_cinfo(Ptr<BhkBreakableConstraint>, Ptr) -> Ptr
        ),
        entry!(
            0x00cc4e10,
            hk_breakable_constraint_cinfo_load_binary(Ptr<HkBreakableConstraintCinfo>, Ptr)
        ),
        entry!(
            0x00cc4e90,
            hk_breakable_constraint_cinfo_save_binary(Ptr<HkBreakableConstraintCinfo>, Ptr)
        ),
        entry!(
            0x00cc4f00,
            hk_breakable_constraint_cinfo_get_viewer_strings(Ptr<HkBreakableConstraintCinfo>, Ptr)
        ),
        entry!(0x00cc8920, hk_breakable_constraint_cinfo_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00d4df30,
            ahkp_breakable_constraint_data_constructor(
                Ptr<AhkpBreakableConstraintData>,
                Ptr,
            ) -> Ptr
        ),
        entry!(0x00d4e000, fn_00d4e000(Ptr<AhkpBreakableConstraintData>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00cc48b0_frees_only_when_bit_zero_is_set() {
        check(0x00cc_48b0, 0x00d4_e000);
    }

    #[test]
    fn bhk_breakable_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_47f0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cc48e0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc48e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_47f0), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_breakable_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_4860, &args![]).u32(), 0x0126_85e0);
    }

    // ---- Tests of the functions translated from the exe ----

    const WRITER: u32 = 0x7000_0010;
    const READER: u32 = 0x7000_0011;
    const CHILD_SLOT_24: u32 = 0x7000_0024;
    const CHILD_SLOT_20: u32 = 0x7000_0020;
    const ALLOC_SLOT: u32 = 0x7000_0004;
    const CREATE_CINFO_SLOT: u32 = 0x7000_00b4;
    const INFO_DESTRUCTOR_SLOT: u32 = 0x7000_0030;
    const INFO_STRINGS_SLOT: u32 = 0x7000_0031;
    const ROUTER_OBJECT: u32 = 0x7100_0000;
    const CHILD_OBJECT: u32 = 0x7200_0000;

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

    /// The order in which the given addresses were called.
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
            .unwrap();
        let mut table = vec![0u32; words as usize];
        for (offset, target) in entries {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(address, &table);
    }

    /// The doubles most tests need: the reference counting, the bridge atom and the memory
    /// router whose allocator (virtual slot 4) hands out zeroed blocks; the threshold constant.
    fn install_common(e: &mut Engine) {
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(BRIDGE_ATOM_INIT, |_, _| Ret::default());
        put_vtable_at(e, 0x7001_1000, &[(4, ALLOC_SLOT)]);
        e.map(ROUTER_OBJECT, 0x40);
        e.mem.set_u32(ROUTER_OBJECT + 0x10, ROUTER_OBJECT + 0x20);
        e.mem.set_u32(ROUTER_OBJECT + 0x20, 0x7001_1000);
        e.register(MEMORY_ROUTER, |_, _| ROUTER_OBJECT.into_ret());
        e.register(ALLOC_SLOT, |e, a| e.mem.alloc(a[1]).into_ret());
        e.set_global(DEFAULT_THRESHOLD, 10.0f32);
    }

    /// An object whose virtual `+0x24` writes the two words `0x1234_5678` and `0x9abc_def0`
    /// to the result it is given, and whose virtual `+0x20` answers 7.
    fn child_at(e: &mut Engine, address: u32) {
        put_vtable_at(
            e,
            0x7001_2000,
            &[(0x20, CHILD_SLOT_20), (0x24, CHILD_SLOT_24)],
        );
        e.map(address, 0x40);
        e.mem.set_u32(address, 0x7001_2000);
        e.register(CHILD_SLOT_24, |e, a| {
            e.mem.set_u32(a[2], 0x1234_5678);
            e.mem.set_u32(a[2] + 4, 0x9abc_def0);
            Ret::default()
        });
        e.register(CHILD_SLOT_20, |_, _| 7u32.into_ret());
    }

    #[test]
    fn bhk_breakable_constraint_load_binary_jumps_to_the_constraint_load() {
        let mut e = logged_engine();
        e.register(CONSTRAINT_LOAD_BINARY, |_, _| Ret::default());
        e.call(0x00ca_98d0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls_to(&e, CONSTRAINT_LOAD_BINARY),
            vec![vec![0x1000, 0x2000]]
        );
    }

    #[test]
    fn bhk_breakable_constraint_destructor_runs_kill_then_the_base_destructor() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(CONSTRAINT_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(CONSTRAINT_COUNT, 5);
        e.call(0x00cc_47f0, &args![this]);
        assert_eq!(e.mem.u32(this), CONSTRAINT_VTABLE);
        assert_eq!(e.mem.u32(CONSTRAINT_COUNT), 4);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, CONSTRAINT_DESTRUCT), vec![vec![this]]);
        assert_eq!(
            order_of(&e, &[SERIALIZABLE_KILL, CONSTRAINT_DESTRUCT]),
            vec![SERIALIZABLE_KILL, CONSTRAINT_DESTRUCT]
        );
    }

    #[test]
    fn bhk_breakable_constraint_link_object_creates_the_cinfo_and_steps_the_stream() {
        let mut e = logged_engine();
        e.register(CREATE_CINFO_SLOT, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        put_vtable_at(&mut e, 0x7001_3000, &[(0xb4, CREATE_CINFO_SLOT)]);
        e.register(CONSTRAINT_LINK_OBJECT, |_, _| Ret::default());
        e.register(STREAM_STEP_LINK_ARRAY, |_, _| Ret::default());
        e.register(STREAM_GET_LINK_OBJECT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x7001_3000);
        e.call(0x00cc_4870, &args![this, 0x2000u32]);
        let creates = calls_to(&e, CREATE_CINFO_SLOT);
        assert_eq!(creates.len(), 1);
        assert_eq!(creates[0][0], this);
        assert_eq!(creates[0][1] % 4, 3);
        assert_eq!(
            calls_to(&e, CONSTRAINT_LINK_OBJECT),
            vec![vec![this, 0x2000]]
        );
        assert_eq!(calls_to(&e, STREAM_STEP_LINK_ARRAY), vec![vec![0x2000]]);
        assert_eq!(calls_to(&e, STREAM_GET_LINK_OBJECT), vec![vec![0x2000]; 2]);
        assert_eq!(
            order_of(
                &e,
                &[
                    CREATE_CINFO_SLOT,
                    CONSTRAINT_LINK_OBJECT,
                    STREAM_STEP_LINK_ARRAY,
                    STREAM_GET_LINK_OBJECT
                ]
            ),
            vec![
                CREATE_CINFO_SLOT,
                CONSTRAINT_LINK_OBJECT,
                STREAM_STEP_LINK_ARRAY,
                STREAM_GET_LINK_OBJECT,
                STREAM_GET_LINK_OBJECT
            ]
        );
    }

    #[test]
    fn fn_00cc4900_allocates_and_constructs_a_constraint() {
        let mut e = logged_engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 0xdead);
            Ret::default()
        });
        e.mem.set_u32(SERIALIZABLE_COUNT, 7);
        e.mem.set_u32(CONSTRAINT_COUNT, 9);
        let object = e.call(0x00cc_4900, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), CONSTRAINT_VTABLE);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(SERIALIZABLE_COUNT), 8);
        assert_eq!(e.mem.u32(CONSTRAINT_COUNT), 10);
        // A failed allocation answers 0 and constructs nothing.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_4900, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT).len(), 1);
        assert_eq!(e.mem.u32(CONSTRAINT_COUNT), 10);
    }

    #[test]
    fn fn_00cc4990_initialises_the_cinfo_block() {
        let mut e = logged_engine();
        let this = e.mem.alloc(0x14);
        for offset in [4, 8, 0xc, 0x10] {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00cc_4990, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), CINFO_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn ahkp_breakable_constraint_data_set_child_swaps_the_child_and_reads_its_info() {
        let mut e = logged_engine();
        install_common(&mut e);
        child_at(&mut e, CHILD_OBJECT);
        let seen = Rc::new(Cell::new(0));
        let seen_in_double = Rc::clone(&seen);
        e.register_double(BRIDGE_ATOM_INIT, move |e, _| {
            let thread = e.tls();
            seen_in_double.set(e.mem.u32(thread + SCOPE_WORD));
            Ret::default()
        });
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x18, 0x6000);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00cc_49b0, &args![this, CHILD_OBJECT]);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![CHILD_OBJECT]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0x6000]]);
        assert_eq!(e.mem.u32(this + 0x18), CHILD_OBJECT);
        let infos = calls_to(&e, CHILD_SLOT_24);
        assert_eq!(infos.len(), 1);
        assert_eq!(infos[0][..2], [CHILD_OBJECT, 1]);
        assert_eq!(e.mem.u16(this + 0x1c), 0x5678);
        assert_eq!(e.mem.u16(this + 0x1e), 0xdef0);
        assert_eq!(calls_to(&e, BRIDGE_ATOM_INIT), vec![vec![this + 0xc, this]]);
        assert_eq!(seen.get(), 0x10);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // No old child: nothing is released.
        e.mem.set_u32(this + 0x18, 0);
        e.call(0x00cc_49b0, &args![this, CHILD_OBJECT]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 1);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 2);
    }

    /// A Cinfo whose data holds `child`, a threshold word and a flag.
    fn cinfo_with_data(e: &mut Engine, child: u32, threshold: u32, flag: u8) -> (u32, u32) {
        let cinfo = e.mem.alloc(0x14);
        let data = e.mem.alloc(0x28);
        e.mem.set_u32(cinfo + 4, data);
        e.mem.set_u32(data + 0x18, child);
        e.mem.set_u32(data + 0x20, threshold);
        e.mem.set_u8(data + 0x24, flag);
        (cinfo, data)
    }

    #[test]
    fn hk_breakable_constraint_cinfo_clone_data_builds_the_data_and_clones_the_child() {
        let mut e = logged_engine();
        install_common(&mut e);
        child_at(&mut e, CHILD_OBJECT);
        e.register(MANAGER_CLONE, |_, _| CHILD_OBJECT.into_ret());
        e.register(CINFO_CLONE_DATA, |_, _| Ret::default());
        let (cinfo, _) = cinfo_with_data(&mut e, 0x6000, 0x7f80_0001, 1);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00cc_4a80, &args![cinfo, 0u32, 0x3000u32]);
        assert_eq!(calls_to(&e, MANAGER_CLONE), vec![vec![0x6000, 0x3000]]);
        let clones = calls_to(&e, CINFO_CLONE_DATA);
        assert_eq!(clones.len(), 1);
        let new_data = clones[0][1];
        assert_eq!(clones[0], vec![cinfo, new_data, 0x3000]);
        assert_ne!(new_data, 0);
        assert_eq!(e.mem.u32(new_data), DATA_VTABLE);
        assert_eq!(e.mem.u16(new_data + 4), 0x28);
        assert_eq!(e.mem.u32(new_data + 0x18), CHILD_OBJECT);
        assert_eq!(e.mem.u32(new_data + 0x20), 0x7fc0_0001);
        assert_eq!(e.mem.u8(new_data + 0x24), 1);
        assert_eq!(e.mem.u8(new_data + 0x25), 0);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![CHILD_OBJECT]]);
        assert_eq!(calls_to(&e, ALLOC_SLOT).len(), 1);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn hk_breakable_constraint_cinfo_clone_data_fills_a_given_data_without_a_child() {
        let mut e = logged_engine();
        install_common(&mut e);
        e.register(MANAGER_CLONE, |_, _| Ret::default());
        e.register(CINFO_CLONE_DATA, |_, _| Ret::default());
        let (cinfo, _) = cinfo_with_data(&mut e, 0, 0x4048_0000, 0);
        let given = e.mem.alloc(0x28);
        e.mem.set_u8(given + 0x24, 1);
        e.call(0x00cc_4a80, &args![cinfo, given, 0x3000u32]);
        assert!(calls_to(&e, ALLOC_SLOT).is_empty());
        assert!(calls_to(&e, MANAGER_CLONE).is_empty());
        assert_eq!(e.mem.f32(given + 0x20), 3.125);
        assert_eq!(e.mem.u8(given + 0x24), 0);
        assert_eq!(
            calls_to(&e, CINFO_CLONE_DATA),
            vec![vec![cinfo, given, 0x3000]]
        );
        // A child the manager cannot clone is not set.
        let (cinfo, _) = cinfo_with_data(&mut e, 0x6000, 0, 0);
        e.call(0x00cc_4a80, &args![cinfo, given, 0x3000u32]);
        assert_eq!(calls_to(&e, MANAGER_CLONE).len(), 1);
        assert!(calls_to(&e, ADD_REFERENCE).is_empty());
        assert!(calls_to(&e, REMOVE_REFERENCE).is_empty());
    }

    #[test]
    fn hk_breakable_constraint_cinfo_create_data_builds_the_data_once() {
        let mut e = logged_engine();
        install_common(&mut e);
        let seen = Rc::new(Cell::new(0));
        let seen_in_double = Rc::clone(&seen);
        e.register_double(ALLOC_SLOT, move |e, a| {
            let thread = e.tls();
            seen_in_double.set(e.mem.u32(thread + SCOPE_WORD));
            e.mem.alloc(a[1]).into_ret()
        });
        let cinfo = e.mem.alloc(0x14);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        assert_eq!(e.call(0x00cc_4b90, &args![cinfo]).u32(), 0x28);
        let data = e.mem.u32(cinfo + 4);
        assert_ne!(data, 0);
        assert_eq!(seen.get(), 0x10);
        assert_eq!(e.mem.u32(data), DATA_VTABLE);
        assert_eq!(e.mem.u16(data + 4), 0x28);
        assert_eq!(e.mem.u32(data + 0x18), 0);
        assert_eq!(e.mem.f32(data + 0x20), 10.0);
        assert_eq!(e.mem.u8(data + 0x24), 0);
        assert_eq!(e.mem.u8(data + 0x25), 0);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        assert_eq!(calls_to(&e, BRIDGE_ATOM_INIT), vec![vec![data + 0xc, data]]);
        // The second call finds the data and builds nothing.
        assert_eq!(e.call(0x00cc_4b90, &args![cinfo]).u32(), 0x28);
        assert_eq!(e.mem.u32(cinfo + 4), data);
        assert_eq!(calls_to(&e, BRIDGE_ATOM_INIT).len(), 1);
    }

    #[test]
    fn bhk_breakable_constraint_create_clone_copies_the_members_under_the_clone_lock() {
        let mut e = logged_engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(CONSTRAINT_COPY_MEMBERS, |_, _| Ret::default());
        e.mem.set_u32(SERIALIZABLE_COUNT, 1);
        e.mem.set_u32(CONSTRAINT_COUNT, 2);
        let this = e.mem.alloc(0x10);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let clone = e.call(0x00cc_4c60, &args![this, 0x3000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), CONSTRAINT_VTABLE);
        assert_eq!(e.mem.u32(SERIALIZABLE_COUNT), 2);
        assert_eq!(e.mem.u32(CONSTRAINT_COUNT), 3);
        assert_eq!(
            calls_to(&e, CONSTRAINT_COPY_MEMBERS),
            vec![vec![this, clone, 0x3000]]
        );
        assert_eq!(
            order_of(
                &e,
                &[
                    CLONE_LOCK,
                    OPERATOR_NEW,
                    OBJECT_CONSTRUCT,
                    CONSTRAINT_COPY_MEMBERS,
                    CLONE_UNLOCK
                ]
            ),
            vec![
                CLONE_LOCK,
                OPERATOR_NEW,
                OBJECT_CONSTRUCT,
                CONSTRAINT_COPY_MEMBERS,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // A failed allocation still runs the copy, with a null clone.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_4c60, &args![this, 0x3000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, CONSTRAINT_COPY_MEMBERS)[1],
            vec![this, 0, 0x3000]
        );
    }

    #[test]
    fn bhk_breakable_constraint_create_cinfo_builds_the_cinfo_block_once() {
        let mut e = logged_engine();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(CONSTRAINT_COPY_CINFO, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x5000);
        let created = e.mem.alloc(4);
        e.mem.set_u8(created, 9);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let info = e.call(0x00cc_4d40, &args![this, created]).u32();
        let block = info - 4;
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14]]
        );
        assert_eq!(e.mem.u32(this + 0xc), info);
        assert_eq!(e.mem.u32(block), CINFO_VTABLE);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(calls_to(&e, CONSTRAINT_COPY_CINFO), vec![vec![this, block]]);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // The second call finds the Cinfo.
        assert_eq!(e.call(0x00cc_4d40, &args![this, created]).u32(), info);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE).len(), 1);
        assert_eq!(calls_to(&e, CONSTRAINT_COPY_CINFO).len(), 1);
        // Without the Havok constraint no copy is made; a failed allocation stores 0.
        let bare = e.mem.alloc(0x10);
        e.call(0x00cc_4d40, &args![bare, created]);
        assert_eq!(calls_to(&e, CONSTRAINT_COPY_CINFO).len(), 1);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        let failing = e.mem.alloc(0x10);
        e.mem.set_u32(failing + 8, 0x5000);
        assert_eq!(e.call(0x00cc_4d40, &args![failing, created]).u32(), 0);
        assert_eq!(calls_to(&e, CONSTRAINT_COPY_CINFO)[1], vec![failing, 0]);
    }

    /// A stream whose output object (`+0x250`) writes through a double that appends the bytes
    /// it is given to a log (`output + 0x10`).
    fn writer_stream(e: &mut Engine) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let output = e.mem.alloc(0x20);
        let log = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, WRITER);
        e.mem.set_u32(output + 0x10, log);
        e.register(WRITER, |e, a| {
            let used = e.mem.u32(a[0] + 0x14);
            let log = e.mem.u32(a[0] + 0x10);
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(log + used, &bytes);
            e.mem.set_u32(a[0] + 0x14, used + a[2]);
            a[2].into_ret()
        });
        (stream, output)
    }

    /// A stream whose input object (`+0x24c`) reads `bytes` through a double.
    fn reader_stream(e: &mut Engine, bytes: &[u8]) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x20);
        let source = e.mem.alloc(0x100);
        e.mem.write(source, bytes);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, READER);
        e.mem.set_u32(input + 0x10, source);
        e.register(READER, |e, a| {
            let used = e.mem.u32(a[0] + 0x14);
            let source = e.mem.u32(a[0] + 0x10);
            let bytes = e.mem.bytes(source + used, a[2]);
            e.mem.write(a[1], &bytes);
            e.mem.set_u32(a[0] + 0x14, used + a[2]);
            a[2].into_ret()
        });
        (stream, input)
    }

    #[test]
    fn hk_breakable_constraint_cinfo_load_binary_reads_the_child_threshold_and_flag() {
        let mut e = logged_engine();
        install_common(&mut e);
        child_at(&mut e, CHILD_OBJECT);
        e.register(CINFO_LOAD_BINARY, |_, _| Ret::default());
        e.register(MANAGER_LOAD_BINARY, |_, _| CHILD_OBJECT.into_ret());
        let mut bytes = std::f32::consts::PI.to_le_bytes().to_vec();
        bytes.push(1);
        let (stream, input) = reader_stream(&mut e, &bytes);
        let (cinfo, data) = cinfo_with_data(&mut e, 0, 0, 0);
        e.call(0x00cc_4e10, &args![cinfo, stream]);
        assert_eq!(calls_to(&e, CINFO_LOAD_BINARY), vec![vec![cinfo, stream]]);
        assert_eq!(calls_to(&e, MANAGER_LOAD_BINARY), vec![vec![stream]]);
        assert_eq!(e.mem.u32(data + 0x18), CHILD_OBJECT);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![CHILD_OBJECT]]);
        assert_eq!(e.mem.f32(data + 0x20), std::f32::consts::PI);
        assert_eq!(e.mem.u8(data + 0x24), 1);
        assert_eq!(e.mem.u32(input + 4), 5);
        let reads = calls_to(&e, READER);
        assert_eq!(reads.len(), 2);
        assert_eq!(
            (reads[0][..3].to_vec(), reads[0][4]),
            (vec![input, data + 0x20, 4], 1)
        );
        assert_eq!(
            (reads[1][..3].to_vec(), reads[1][4]),
            (vec![input, data + 0x24, 1], 1)
        );
    }

    #[test]
    fn hk_breakable_constraint_cinfo_save_binary_writes_the_child_threshold_and_flag() {
        let mut e = logged_engine();
        e.register(CINFO_SAVE_BINARY, |_, _| Ret::default());
        e.register(MANAGER_SAVE_BINARY, |_, _| Ret::default());
        let (stream, output) = writer_stream(&mut e);
        let (cinfo, data) = cinfo_with_data(&mut e, 0x6000, 2.5f32.to_bits(), 1);
        e.call(0x00cc_4e90, &args![cinfo, stream]);
        assert_eq!(calls_to(&e, CINFO_SAVE_BINARY), vec![vec![cinfo, stream]]);
        assert_eq!(
            calls_to(&e, MANAGER_SAVE_BINARY),
            vec![vec![stream, 0x6000]]
        );
        let log = e.mem.u32(output + 0x10);
        let mut expected = 2.5f32.to_le_bytes().to_vec();
        expected.push(1);
        assert_eq!(e.mem.bytes(log, 5), expected);
        assert_eq!(e.mem.u32(output + 4), 5);
        let writes = calls_to(&e, WRITER);
        assert_eq!(writes.len(), 2);
        assert_eq!(writes[0][..3], [output, data + 0x20, 4]);
        assert_eq!(writes[1][..3], [output, data + 0x24, 1]);
    }

    /// A string array with room for 16 entries (grow-by 4) and the doubles that resize and
    /// store into it.
    fn string_array(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x100);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 16);
        e.mem.set_u16(array + 0xe, 4);
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        array
    }

    #[test]
    fn hk_breakable_constraint_cinfo_get_viewer_strings_lists_the_fields_and_the_child() {
        let mut e = logged_engine();
        e.register(CINFO_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING_FLOAT, |_, _| 0x1111u32.into_ret());
        e.register(GET_VIEWER_STRING_BOOL, |_, _| 0x2222u32.into_ret());
        e.register(MANAGER_GET_INFO, |_, _| ROUTER_OBJECT.into_ret());
        e.register(CINFO_SET_DATA, |_, _| Ret::default());
        e.register(INFO_STRINGS_SLOT, |_, _| Ret::default());
        e.register(INFO_DESTRUCTOR_SLOT, |_, _| Ret::default());
        put_vtable_at(
            &mut e,
            0x7001_4000,
            &[(0, INFO_DESTRUCTOR_SLOT), (0x14, INFO_STRINGS_SLOT)],
        );
        e.map(ROUTER_OBJECT, 0x40);
        e.mem.set_u32(ROUTER_OBJECT, 0x7001_4000);
        child_at(&mut e, CHILD_OBJECT);
        let array = string_array(&mut e);
        let (cinfo, _) = cinfo_with_data(&mut e, CHILD_OBJECT, 0x7f80_0001, 1);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00cc_4f00, &args![cinfo, array]);
        assert_eq!(
            calls_to(&e, CINFO_GET_VIEWER_STRINGS),
            vec![vec![cinfo, array]]
        );
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_FLOAT),
            vec![vec![THRESHOLD_LABEL, 0x7fc0_0001]]
        );
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_BOOL),
            vec![vec![REMOVE_IF_BROKEN_LABEL, 1]]
        );
        let base = e.mem.u32(array + 4);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(e.mem.u32(base + 4), 0x2222);
        assert_eq!(calls_to(&e, MANAGER_GET_INFO), vec![vec![7]]);
        assert_eq!(
            calls_to(&e, CINFO_SET_DATA),
            vec![vec![ROUTER_OBJECT, CHILD_OBJECT]]
        );
        assert_eq!(
            calls_to(&e, INFO_STRINGS_SLOT),
            vec![vec![ROUTER_OBJECT, array]]
        );
        assert_eq!(
            calls_to(&e, INFO_DESTRUCTOR_SLOT),
            vec![vec![ROUTER_OBJECT, 1]]
        );
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // No info for the child: nothing is bound or deleted. A full array grows first.
        e.register(MANAGER_GET_INFO, |_, _| Ret::default());
        e.mem.set_u16(array + 0xa, 0);
        e.mem.set_u16(array + 8, 0);
        e.call(0x00cc_4f00, &args![cinfo, array]);
        assert_eq!(calls_to(&e, CINFO_SET_DATA).len(), 1);
        assert_eq!(
            calls_to(&e, STRING_ARRAY_SET_SIZE),
            vec![vec![array, 4], vec![array, 5]]
        );
    }

    #[test]
    fn hk_breakable_constraint_cinfo_get_viewer_strings_without_data_uses_zero() {
        let mut e = logged_engine();
        e.register(CINFO_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING_FLOAT, |_, _| 0x1111u32.into_ret());
        e.register(GET_VIEWER_STRING_BOOL, |_, _| 0x2222u32.into_ret());
        let array = string_array(&mut e);
        let cinfo = e.mem.alloc(0x14);
        e.call(0x00cc_4f00, &args![cinfo, array]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_FLOAT),
            vec![vec![THRESHOLD_LABEL, 0]]
        );
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_BOOL),
            vec![vec![REMOVE_IF_BROKEN_LABEL, 0]]
        );
        assert!(calls_to(&e, MANAGER_GET_INFO).is_empty());
    }

    #[test]
    fn hk_breakable_constraint_cinfo_scalar_deleting_destructor_frees_only_on_bit_0() {
        let mut e = logged_engine();
        e.register(CINFO_SET_DATA, |_, _| Ret::default());
        e.register(MEMORY_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this, CINFO_VTABLE);
        assert_eq!(e.call(0x00cc_8920, &args![this, 0xfffe_u32]).u32(), this);
        assert_eq!(e.mem.u32(this), CINFO_BASE_VTABLE);
        assert_eq!(calls_to(&e, CINFO_SET_DATA), vec![vec![this, 0]]);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        assert_eq!(e.call(0x00cc_8920, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, this]]);
    }

    #[test]
    fn ahkp_breakable_constraint_data_constructor_sets_the_defaults_and_reads_the_child() {
        let mut e = logged_engine();
        install_common(&mut e);
        child_at(&mut e, CHILD_OBJECT);
        e.mem.set_u32(DEFAULT_THRESHOLD, 0x7f80_0001);
        let this = e.mem.alloc(0x28);
        for offset in [4, 8, 0xc, 0x18, 0x1c, 0x20, 0x24] {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        let result = e.call(0x00d4_df30, &args![this, CHILD_OBJECT]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), DATA_BASE_VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 0x18), CHILD_OBJECT);
        assert_eq!(e.mem.u16(this + 0x1c), 0x5678);
        assert_eq!(e.mem.u16(this + 0x1e), 0xdef0);
        assert_eq!(e.mem.u32(this + 0x20), 0x7fc0_0001);
        assert_eq!(e.mem.u8(this + 0x24), 0);
        assert_eq!(e.mem.u8(this + 0x25), 0);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![CHILD_OBJECT]]);
        assert_eq!(calls_to(&e, BRIDGE_ATOM_INIT), vec![vec![this + 0xc, this]]);
        // Without a child it only initialises the atom.
        let bare = e.mem.alloc(0x28);
        e.call(0x00d4_df30, &args![bare, 0u32]);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 1);
        assert_eq!(calls_to(&e, CHILD_SLOT_24).len(), 1);
        assert_eq!(calls_to(&e, BRIDGE_ATOM_INIT).len(), 2);
        assert_eq!(e.mem.u32(bare + 0x18), 0);
    }

    #[test]
    fn fn_00d4e000_drops_the_child_and_stores_the_root_vtable() {
        let mut e = logged_engine();
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x18, 0x6000);
        e.call(0x00d4_e000, &args![this]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0x6000]]);
        assert_eq!(e.mem.u32(this), DATA_ROOT_VTABLE);
        e.mem.set_u32(this + 0x18, 0);
        e.call(0x00d4_e000, &args![this]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE)[1], vec![0]);
    }
}
