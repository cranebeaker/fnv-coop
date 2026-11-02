//! `BSHavok/bhkconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkConstraint` keeps its Havok constraint instance at `+8` (`phkObject`). The PC class
//! reaches its construction info (`hkConstraintCinfo`, a second object whose address the
//! class hands out 4 bytes inside, so the code subtracts 4) through virtual slot `0xB4`;
//! slot `0xB4` takes one stack word, the address of a byte the callers leave unset.
//! The cinfo keeps the constraint data at `+4`, the priority at `+8` and the two entities
//! at `+0xC` and `+0x10`.
//!
//! The functions that allocate or release Havok objects raise the per-thread allocation
//! word at `TLS + 0x2b4` to 0x10 for their duration and put the old value back. The
//! exception-unwinding frames (`FS:[0]` registrations) and `/GS` cookie checks of the
//! functions marked `Translation incomplete` are not modeled.
//!
//! A pointer the code dereferences without testing it (a null `this` adjusted by -4, or a
//! virtual call on a null object) reads address 0 or a small offset in the exe; here that
//! reads unmapped memory and panics with the address.

#[allow(unused_imports)]
use crate::prelude::*;

/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;
/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator whose slot `+4`
/// allocates `(size)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `bhkSerializable` base methods this unit's overrides call.
const SERIALIZABLE_LOAD_BINARY: u32 = 0x00c8_7650;
const SERIALIZABLE_COPY_MEMBERS: u32 = 0x00c8_7600;
const SERIALIZABLE_LINK_OBJECT: u32 = 0x00c8_7670;
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const SERIALIZABLE_DESTRUCT: u32 = 0x00c8_77b0;
const SERIALIZABLE_GET_VIEWER_STRINGS: u32 = 0x00c8_78c0;
/// `bhkRefObject::SethkObject` (`00c87c50`) and `bhkRefObject::Link` (`00c87c90`).
const REF_OBJECT_SET_HK_OBJECT: u32 = 0x00c8_7c50;
const REF_OBJECT_LINK: u32 = 0x00c8_7c90;
/// `NiObject::IsEqual` (`00ba87d0`, a jump to `00a5d470`) and `NiObject::RegisterStreamables`
/// (`00a81fe0`).
const NI_OBJECT_IS_EQUAL: u32 = 0x00ba_87d0;
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a8_1fe0;
/// `NiStream` helpers: `00a63430` answers the next linked object, `00a63460` is called
/// first by `LinkObject`, `00a64670` reads the link ids.
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_BEFORE_LINKS: u32 = 0x00a6_3460;
const STREAM_READ_LINK_IDS: u32 = 0x00a6_4670;
/// The cloning process's map lookup `(key, &value)` (`00a53c30`, answers whether found).
const MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `hkpConstraintInstance` constructor (`00cf3ea0`, four stack words).
const CONSTRAINT_INSTANCE_CONSTRUCT: u32 = 0x00cf_3ea0;
/// `bhkWorld::Add_ov2` (`00c6b320`) and `bhkWorld::Remove_ov3` (`00c68d50`), one stack word.
const WORLD_ADD: u32 = 0x00c6_b320;
const WORLD_REMOVE: u32 = 0x00c6_8d50;
/// `00a29680`, called twice around a read of a field of `ESI->+8` by `ProcessEntityB`.
const PROCESS_ENTITY_GUARD: u32 = 0x00a2_9680;
/// `hkOstream::hkOstream` (`00cc0500`, `(buffer, size, flag)`), its `operator<<(const char*)`
/// (`00cc01c0`) and its destructor (`00cc00e0`).
const OSTREAM_CONSTRUCT: u32 = 0x00cc_0500;
const OSTREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The global with the report sink whose slot `0xC` takes `(1, 0x234f24fc, text, file, line)`.
const REPORT_SINK: u32 = 0x0126_8360;
/// Motor classes read and written by the cinfo: constructors, loaders and savers by
/// motor type 1, 2 and 3.
const MOTOR1_CONSTRUCT: u32 = 0x00cf_4190;
const MOTOR1_LOAD: u32 = 0x00cf_41c0;
const MOTOR1_SAVE: u32 = 0x00cf_4260;
const MOTOR2_CONSTRUCT: u32 = 0x00cf_43b0;
const MOTOR2_LOAD: u32 = 0x00cf_4400;
const MOTOR2_SAVE: u32 = 0x00cf_4490;
const MOTOR3_CONSTRUCT: u32 = 0x00cf_45d0;
const MOTOR3_LOAD: u32 = 0x00cf_4600;
const MOTOR3_SAVE: u32 = 0x00cf_4670;
/// `00cf4790`, the type-name text of a constraint data type (cdecl, one argument).
const CONSTRAINT_TYPE_NAME: u32 = 0x00cf_4790;
/// `NiGetViewerString` overloads (cdecl): `00a7d4d0` `(name, integer)`, `00a7d5e0`
/// `(name, text)`; both answer the new text.
const VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
/// `sprintf_s` (`00406d00`, cdecl `(buffer, size, format, ...)`).
const FORMAT_TEXT: u32 = 0x0040_6d00;
/// The `NiTArray` growth (`0096ad30`, one stack word) and store (`0096ae90`,
/// `(index, &value)`) the viewer-string list uses.
const ARRAY_GROW: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The count of live `bhkConstraint` objects (decremented by the destructor).
const CONSTRAINT_COUNT: u32 = 0x0126_8188;
/// The vtable of `bhkConstraint` (RTTI-confirmed: `.?AVbhkConstraint@@`).
const VTABLE_CONSTRAINT: u32 = 0x0108_febc;

layout! {
    /// `bhkConstraint` (Xbox PDB), 0x10 bytes there.
    pub struct BhkConstraint: 0x10 {
        /// `phkObject` (Xbox PDB): the Havok constraint instance.
        0x08 phkObject: Ptr,
    }

    /// `hkConstraintCinfo` (Xbox PDB), offsets confirmed against the PC code.
    pub struct HkConstraintCinfo: 0x14 {
        /// `pData` (Xbox PDB): `hkpConstraintData*`.
        0x04 pData: Ptr,
        /// `ePriority` (Xbox PDB).
        0x08 ePriority: u32,
        /// `pEntityA` (Xbox PDB): `hkpEntity*`.
        0x0C pEntityA: Ptr,
        /// `pEntityB` (Xbox PDB): `hkpEntity*`.
        0x10 pEntityB: Ptr,
    }
}

/// Raises the per-thread allocation word to 0x10 and answers the old value.
pub(crate) fn enter_scope(e: &mut Engine) -> u32 {
    let word = e.tls() + SCOPE_WORD;
    let old = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    old
}

/// Puts the per-thread allocation word back.
pub(crate) fn leave_scope(e: &mut Engine, old: u32) {
    let word = e.tls() + SCOPE_WORD;
    e.mem.set_u32(word, old);
}

/// Allocates `size` bytes from the Havok allocator (the memory router's allocator,
/// virtual slot `+4`) and stores the size as the block's 16-bit word at `+4`.
pub(crate) fn allocate_havok(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

/// A transfer through a stream's reader or writer object: the cdecl function pointer in
/// the object's `slot` (`+8` reads, `+0xC` writes) is called as `(object, buffer, size,
/// &size, 1)`, and its result is added to the object's byte count at `+4`.
pub(crate) fn stream_transfer(e: &mut Engine, object: u32, slot: u32, buffer: u32, size: u32) {
    let function = e.mem.u32(object + slot);
    let done = e.with_stack(4, |e, size_word| {
        e.mem.set_u32(size_word.addr(), size);
        e.call(function, &args![object, buffer, size, size_word, 1u32])
            .u32()
    });
    let count = e.mem.u32(object + 4);
    e.mem.set_u32(object + 4, count.wrapping_add(done));
}

/// The cinfo of a constraint: slot `0xB4` of the constraint answers a pointer 4 bytes into
/// it, or 0 (the code then keeps the null).
fn cinfo_of(e: &mut Engine, this: Ptr) -> u32 {
    let inner = e
        .with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32();
    if inner == 0 {
        0
    } else {
        inner.wrapping_sub(4)
    }
}

// Translated from 00c89720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::UnMarkRead` (map name, taken from a vtable; the body
/// is only this jump). A tail jump: `jmp [vtable + 0x94]` with `ECX` still
/// `this`, so the caller's own stack words (`forwarded`, none known) reach
/// the virtual function unchanged and its result is returned.
pub fn fn_00c89720(e: &mut Engine, this: Ptr, forwarded: &[u32]) -> Ret {
    e.vcall(this.addr(), 0x94, forwarded)
}

/// Uniform form of [`fn_00c89720`]: `this`, then the forwarded words.
fn fn_00c89720_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fn_00c89720(e, Ptr::new(words[0]), &words[1..])
}

// Translated from 00c89640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c89640 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraint::~bhkConstraint` (Xbox PDB): sets the `bhkConstraint` vtable, runs
/// `bhkSerializable::Kill`, decrements the live-object count at `01268188` and runs the
/// `bhkSerializable` destructor.
pub fn bhk_constraint_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CONSTRAINT);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.global::<u32>(CONSTRAINT_COUNT);
    e.set_global(CONSTRAINT_COUNT, count.wrapping_sub(1));
    e.call(SERIALIZABLE_DESTRUCT, &args![this]);
}

// Translated from 00c896b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::LoadBinary` (Xbox PDB): a tail jump to `bhkSerializable::LoadBinary`
/// (`00c87650`) with the same stream.
pub fn bhk_constraint_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(SERIALIZABLE_LOAD_BINARY, &args![this, stream]);
}

// Translated from 00c896c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::IsEqual` (Xbox PDB): the base comparison (`00ba87d0`) and then, when it
/// holds, whether virtual slots `0xCC` and `0xD0` answer the same on both objects (the
/// other object is asked first for each).
pub fn bhk_constraint_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let other_first = e.vcall(other.addr(), 0xcc, &args![]).u32();
    let this_first = e.vcall(this.addr(), 0xcc, &args![]).u32();
    let first_equal = this_first == other_first;
    let other_second = e.vcall(other.addr(), 0xd0, &args![]).u32();
    let this_second = e.vcall(this.addr(), 0xd0, &args![]).u32();
    first_equal & (this_second == other_second)
}

// Translated from 00c89730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::CopyMembers` (Xbox PDB), `(this, source, process)` with the cloning
/// process third: when the process's float at `+0x10` is not exactly 1.0 (NaN included)
/// the cinfo's virtual slot 4 is called with `(0, process)`; then the cinfo's two
/// entities are replaced by their clones found through the process's map (the second
/// only when the first entity existed; a miss on the second asks the entity's own virtual
/// slot `0x48`, a miss on the first leaves the slot empty); then
/// `bhkSerializable::CopyMembers`.
pub fn bhk_constraint_copy_members(e: &mut Engine, this: Ptr, source: u32, process: u32) {
    let cinfo = cinfo_of(e, this);
    if e.mem.f32(process + 0x10) != 1.0 {
        e.vcall(cinfo, 4, &args![0u32, process]);
    }
    let owner_of = |e: &Engine, entity: u32| {
        if entity == 0 {
            0
        } else {
            e.mem.u32(entity + 0xc)
        }
    };
    let first = owner_of(e, e.mem.u32(cinfo + 0xc));
    let second = owner_of(e, e.mem.u32(cinfo + 0x10));
    e.mem.set_u32(cinfo + 0xc, 0);
    e.mem.set_u32(cinfo + 0x10, 0);
    if first != 0 {
        let map = e.mem.u32(process);
        let clone = e.with_stack(4, |e, found| {
            let hit = e.call(MAP_LOOKUP, &args![map, first, found]).bool();
            hit.then(|| e.mem.u32(found.addr()))
        });
        if let Some(clone) = clone {
            let stored = if clone == 0 { 0 } else { e.mem.u32(clone + 8) };
            e.mem.set_u32(cinfo + 0xc, stored);
        }
        if second != 0 {
            let map = e.mem.u32(process);
            let clone = e.with_stack(4, |e, found| {
                let hit = e.call(MAP_LOOKUP, &args![map, second, found]).bool();
                if hit {
                    e.mem.u32(found.addr())
                } else {
                    e.vcall(second, 0x48, &args![process]).u32()
                }
            });
            let stored = if clone == 0 { 0 } else { e.mem.u32(clone + 8) };
            e.mem.set_u32(cinfo + 0x10, stored);
        }
    }
    e.call(SERIALIZABLE_COPY_MEMBERS, &args![this, source, process]);
}

// Translated from 00c89810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::LoadCInfo` (Xbox PDB; an `int` there): asks the cinfo's virtual slot 8
/// to load from `source`, clears the word at `+8` of the object at cinfo `+4`, then calls
/// the constraint's own virtual slot `0xA8` and answers its value (the exe ends on that
/// call and leaves its `EAX`).
pub fn bhk_constraint_load_c_info(e: &mut Engine, this: Ptr, source: u32) -> i32 {
    let cinfo = cinfo_of(e, this);
    e.vcall(cinfo, 8, &args![source]);
    let data = e.mem.u32(cinfo + 4);
    e.mem.set_u32(data + 8, 0);
    e.vcall(this.addr(), 0xa8, &args![]).i32()
}

// Translated from 00c89860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::LinkObject` (Xbox PDB): reads two linked objects from the stream (after
/// `00a63460`); when the first exists its Havok object (`+8`) becomes the cinfo's entity A
/// and the second's (or 0) its entity B; then `bhkSerializable::LinkObject`.
pub fn bhk_constraint_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(STREAM_BEFORE_LINKS, &args![stream]);
    let first = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let second = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let cinfo = cinfo_of(e, this);
    if first != 0 {
        let entity_a = e.mem.u32(first + 8);
        e.mem.set_u32(cinfo + 0xc, entity_a);
        let entity_b = if second != 0 {
            e.mem.u32(second + 8)
        } else {
            0
        };
        e.mem.set_u32(cinfo + 0x10, entity_b);
    }
    e.call(SERIALIZABLE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00c898e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::RegisterStreamables` (Xbox PDB; a `bool` there): when virtual slot `0xD0`
/// answers an object whose `+0xC` is set, that one's virtual slot `0x54` registers with the
/// stream; then `NiObject::RegisterStreamables`, whose value it answers (the exe ends on that
/// call and leaves its `EAX`).
pub fn bhk_constraint_register_streamables(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    let first = e.vcall(this.addr(), 0xd0, &args![]).u32();
    if first != 0 {
        let owner = e.mem.u32(first + 0xc);
        if owner != 0 {
            e.vcall(owner, 0x54, &args![stream]);
        }
    }
    e.call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .u32()
}

// Translated from 00c89920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::SaveBinary` (Xbox PDB): the cinfo's virtual slot 3 (`+0xC`) saves to
/// the stream. With no cinfo the code calls through address 0 (reads unmapped memory).
pub fn bhk_constraint_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let cinfo = cinfo_of(e, this);
    e.vcall(cinfo, 0xc, &args![stream]);
}

// Translated from 00c89960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::GetViewerStrings` (Xbox PDB): `bhkSerializable::GetViewerStrings`, the
/// cinfo's virtual slot 5 (`+0x14`) with the same list, then the constraint's virtual
/// slot `0xA4` with 1.
pub fn bhk_constraint_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    e.call(SERIALIZABLE_GET_VIEWER_STRINGS, &args![this, strings]);
    let cinfo = cinfo_of(e, this);
    e.vcall(cinfo, 0x14, &args![strings]);
    e.vcall(this.addr(), 0xa4, &args![1u32]);
}

// Translated from 00c899b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::GethkEntityA` (Xbox PDB): the Havok constraint's entity A (`+0x14`), or
/// 0 without a Havok constraint.
pub fn bhk_constraint_gethk_entity_a(e: &mut Engine, this: Ptr<BhkConstraint>) -> u32 {
    let instance = e.get(this, BhkConstraint::phkObject);
    if instance.is_null() {
        0
    } else {
        e.mem.u32(instance.addr() + 0x14)
    }
}

// Translated from 00c899c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::GethkEntityB` (Xbox PDB): the Havok constraint's entity B (`+0x18`), or
/// 0 without a Havok constraint.
pub fn bhk_constraint_gethk_entity_b(e: &mut Engine, this: Ptr<BhkConstraint>) -> u32 {
    let instance = e.get(this, BhkConstraint::phkObject);
    if instance.is_null() {
        0
    } else {
        e.mem.u32(instance.addr() + 0x18)
    }
}

// Translated from 00c899d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::Link` (Xbox PDB): with a `this` and a Havok constraint, stores `this`
/// (when the low byte of `flag` is not 0) or 0 into the Havok constraint's word at
/// `+0x2C`; then a tail jump to `bhkRefObject::Link` (`00c87c90`) with the same word.
pub fn bhk_constraint_link(e: &mut Engine, this: Ptr, flag: u32) {
    if !this.is_null() {
        let instance = e.mem.u32(this.addr() + 8);
        if instance != 0 {
            let owner = if flag as u8 != 0 { this.addr() } else { 0 };
            e.mem.set_u32(instance + 0x2c, owner);
        }
    }
    e.call(REF_OBJECT_LINK, &args![this, flag]);
}

// Translated from 00c89a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::GethkWorld` (Xbox PDB): with a Havok constraint whose `+8` is set,
/// virtual slot `0xCC`'s answer's Havok object (`+8`), else 0.
pub fn bhk_constraint_gethk_world(e: &mut Engine, this: Ptr<BhkConstraint>) -> u32 {
    let instance = e.get(this, BhkConstraint::phkObject);
    if instance.is_null() || e.mem.u32(instance.addr() + 8) == 0 {
        return 0;
    }
    let owner = e.vcall(this.addr(), 0xcc, &args![]).u32();
    if owner == 0 {
        0
    } else {
        e.mem.u32(owner + 8)
    }
}

// Translated from 00c89a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::Add` (Xbox PDB): false without a Havok constraint. With no world it
/// calls virtual slot `0xA0` and answers false. Otherwise, when the constraint's world
/// (slot `0x94`) differs from the target's: slot `0xA0`, slot `0xD4` with `(world, 1)`,
/// `bhkWorld::Add` with the Havok constraint, and true.
pub fn bhk_constraint_add(e: &mut Engine, this: Ptr, world: Ptr) -> bool {
    let instance = if this.is_null() {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    if instance == 0 {
        return false;
    }
    if world.is_null() {
        e.vcall(this.addr(), 0xa0, &args![]);
        return false;
    }
    let current = e.vcall(this.addr(), 0x94, &args![]).u32();
    let target = e.vcall(world.addr(), 0x94, &args![]).u32();
    if current == target {
        return false;
    }
    e.vcall(this.addr(), 0xa0, &args![]);
    e.vcall(this.addr(), 0xd4, &args![world, 1u32]);
    e.call(WORLD_ADD, &args![world, instance]);
    true
}

// Translated from 00c89ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c89ac0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkConstraintCinfo::SetData` (Xbox PDB): in the Havok allocation scope, releases the
/// current data (clearing its word at `+8` first), references the new data and points its
/// `+8` back at the cinfo, and stores it at `+4`.
pub fn hk_constraint_cinfo_set_data(e: &mut Engine, this: Ptr<HkConstraintCinfo>, data: Ptr) {
    let old = enter_scope(e);
    let previous = e.get(this, HkConstraintCinfo::pData);
    if !previous.is_null() {
        e.mem.set_u32(previous.addr() + 8, 0);
        let again = e.get(this, HkConstraintCinfo::pData);
        e.call(REMOVE_REFERENCE, &args![again]);
    }
    if !data.is_null() {
        e.call(ADD_REFERENCE, &args![data]);
        e.mem.set_u32(data.addr() + 8, this.addr());
    }
    e.set(this, HkConstraintCinfo::pData, data);
    leave_scope(e, old);
}

// Translated from 00c89b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c89b60 - the FS:[0] exception-unwinding frame is not modeled.
/// `ahkpConstraintInstance::setEntityA` (engine map name): in the Havok allocation scope,
/// references the new entity, releases the old one and stores the new one at `+0x14`.
pub fn ahkp_constraint_instance_set_entity_a(e: &mut Engine, this: Ptr, entity: Ptr) {
    let old = enter_scope(e);
    if !entity.is_null() {
        e.call(ADD_REFERENCE, &args![entity]);
    }
    let previous = e.mem.u32(this.addr() + 0x14);
    if previous != 0 {
        e.call(REMOVE_REFERENCE, &args![previous]);
    }
    e.mem.set_u32(this.addr() + 0x14, entity.addr());
    leave_scope(e, old);
}

// Translated from 00c89bf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c89bf0 - the FS:[0] exception-unwinding frame is not modeled.
/// `ahkpConstraintInstance::setEntityB` (engine map name): as `setEntityA`, for the word
/// at `+0x18`.
pub fn ahkp_constraint_instance_set_entity_b(e: &mut Engine, this: Ptr, entity: Ptr) {
    let old = enter_scope(e);
    if !entity.is_null() {
        e.call(ADD_REFERENCE, &args![entity]);
    }
    let previous = e.mem.u32(this.addr() + 0x18);
    if previous != 0 {
        e.call(REMOVE_REFERENCE, &args![previous]);
    }
    e.mem.set_u32(this.addr() + 0x18, entity.addr());
    leave_scope(e, old);
}

// Translated from 00c89c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::CopyCinfo` (Xbox PDB): with a Havok constraint, copies its data
/// (`+0xC`) into the cinfo through `SetData`, its entities (`+0x14`, `+0x18`) into the
/// cinfo's `+0xC`, `+0x10`, and its byte at `+0x1C` (zero-extended) into the cinfo's
/// priority; without one, `SetData(0)` and both entities cleared.
pub fn bhk_constraint_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: Ptr<HkConstraintCinfo>) {
    let instance = if this.is_null() {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    if instance != 0 {
        let data = e.mem.u32(instance + 0xc);
        hk_constraint_cinfo_set_data(e, cinfo, Ptr::new(data));
        let entity_a = e.mem.u32(instance + 0x14);
        e.set(cinfo, HkConstraintCinfo::pEntityA, Ptr::new(entity_a));
        let entity_b = e.mem.u32(instance + 0x18);
        e.set(cinfo, HkConstraintCinfo::pEntityB, Ptr::new(entity_b));
        let priority = e.mem.u8(instance + 0x1c);
        e.set(cinfo, HkConstraintCinfo::ePriority, priority as u32);
    } else {
        hk_constraint_cinfo_set_data(e, cinfo, Ptr::NULL);
        e.set(cinfo, HkConstraintCinfo::pEntityA, Ptr::NULL);
        e.set(cinfo, HkConstraintCinfo::pEntityB, Ptr::NULL);
    }
}

// Translated from 00c89cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::SetEntityA` (Xbox PDB): with a `this` whose Havok constraint exists, a
/// tail jump to `setEntityA` (`00c89b60`) on it with the same entity; otherwise nothing.
pub fn bhk_constraint_set_entity_a(e: &mut Engine, this: Ptr, entity: Ptr) {
    if this.is_null() {
        return;
    }
    let instance = e.mem.u32(this.addr() + 8);
    if instance != 0 {
        ahkp_constraint_instance_set_entity_a(e, Ptr::new(instance), entity);
    }
}

// Translated from 00c89cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint::SetEntityB` (Xbox PDB): as `SetEntityA`, through `setEntityB`
/// (`00c89bf0`).
pub fn bhk_constraint_set_entity_b(e: &mut Engine, this: Ptr, entity: Ptr) {
    if this.is_null() {
        return;
    }
    let instance = e.mem.u32(this.addr() + 8);
    if instance != 0 {
        ahkp_constraint_instance_set_entity_b(e, Ptr::new(instance), entity);
    }
}

// Translated from 00c89d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c89d10 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraint::SethkObject` (Xbox PDB): in the Havok allocation scope, when the new
/// object differs from the current one and the current one has a Havok object at `+8`,
/// raises the use count (`+0x94`) of the entity at `+0x14`'s object `+8` (if any) around
/// the Havok object's virtual slot 4 call with the current object, then
/// `bhkRefObject::SethkObject` stores the new one.
pub fn bhk_constraint_sethk_object(e: &mut Engine, this: Ptr, object: Ptr) {
    let old = enter_scope(e);
    let current = if this.is_null() {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    if object.addr() != current {
        if current != 0 {
            let hk = e.mem.u32(current + 8);
            if hk != 0 {
                let entity = e.mem.u32(current + 0x14);
                let holder = e.mem.u32(entity + 8);
                if holder != 0 {
                    let count = e.mem.u32(holder + 0x94);
                    e.mem.set_u32(holder + 0x94, count.wrapping_add(1));
                }
                e.vcall(hk, 0x10, &args![current]);
                if holder != 0 {
                    let count = e.mem.u32(holder + 0x94);
                    e.mem.set_u32(holder + 0x94, count.wrapping_sub(1));
                }
            }
        }
        e.call(REF_OBJECT_SET_HK_OBJECT, &args![this, object]);
    }
    leave_scope(e, old);
}

// Translated from 00c89dd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c89dd0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraint::Init` (Xbox PDB), `cinfo_inner` being the address 4 bytes into the
/// cinfo (or 0, which the code keeps): in the Havok allocation scope, builds a 0x38-byte
/// `hkpConstraintInstance` from the cinfo's entities (`+0xC`, `+0x10`), data (`+4`) and
/// priority (`+8`), hands it to virtual slot `0x8C`, releases it, then calls virtual
/// slot `0xBC` with `cinfo_inner`.
pub fn bhk_constraint_init(e: &mut Engine, this: Ptr, cinfo_inner: u32) {
    let old = enter_scope(e);
    let cinfo = if cinfo_inner == 0 {
        0
    } else {
        cinfo_inner.wrapping_sub(4)
    };
    let block = allocate_havok(e, 0x38);
    let entity_a = e.mem.u32(cinfo.wrapping_add(0xc));
    let entity_b = e.mem.u32(cinfo.wrapping_add(0x10));
    let data = e.mem.u32(cinfo.wrapping_add(4));
    let priority = e.mem.u32(cinfo.wrapping_add(8));
    let instance = e
        .call(
            CONSTRAINT_INSTANCE_CONSTRUCT,
            &args![block, entity_a, entity_b, data, priority],
        )
        .u32();
    e.vcall(this.addr(), 0x8c, &args![instance]);
    e.call(REMOVE_REFERENCE, &args![instance]);
    e.vcall(this.addr(), 0xbc, &args![cinfo_inner]);
    leave_scope(e, old);
}

// Translated from 00c89ec0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c89ec0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConstraint::Remove` (Xbox PDB): in the Havok allocation scope, takes the world
/// (slot `0x94`) and its Havok world (`+0x350`); when that exists, removes the Havok
/// constraint from it (`bhkWorld::Remove`), calls virtual slot `0xD4` with `(world, 0)`
/// and answers the removal's result; otherwise false.
pub fn bhk_constraint_remove(e: &mut Engine, this: Ptr) -> bool {
    let old = enter_scope(e);
    let world = e.vcall(this.addr(), 0x94, &args![]).u32();
    let hk_world = if world == 0 {
        0
    } else {
        e.mem.u32(world + 0x350)
    };
    let removed = if hk_world != 0 {
        let instance = e.mem.u32(this.addr() + 8);
        let removed = e.call(WORLD_REMOVE, &args![hk_world, instance]).bool();
        e.vcall(this.addr(), 0xd4, &args![hk_world, 0u32]);
        removed
    } else {
        false
    };
    leave_scope(e, old);
    removed
}

// Translated from 00c89f90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c89f90 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled; the third argument of the report stream's constructor keeps three bytes of whatever register the exe left there, here 0.
/// `bhkConstraint::ProcessEntityB` (Xbox PDB), `(this, entity, flag)`: in the Havok
/// allocation scope, takes the object of virtual slot `0xD0`. With `flag` clear, when that
/// object is the one the entity's world (slot `0x94`, `+0x24`) names, calls virtual slot
/// `0xC8` with 0. With `flag` set and no object (or no `+0xC` of it): answers true and
/// calls slot `0xC8` with the entity's world's `+0x24`. With `flag` set and an object,
/// unless the Havok object at `+8` of the linked one has a nonzero `+8`, rebuilds the
/// object's `+0x2C` with its low half and the high half of slot `0xCC`'s `+0x2C`; then
/// when the entity's `+0x18` is 0: slot `0x9C` of the linked one with the entity, and if
/// its slot `0x94` is 0 and its byte at `+0x10` has neither bit `0x30`, writes an error
/// report (`hkOstream` over a 0x200-byte buffer, the text at `010c5e30`, sent to the sink
/// at `01268360` with source `bhkConstraint.cpp` line 749) and calls the linked one's slot
/// `0xC8` with the entity.
pub fn bhk_constraint_process_entity_b(e: &mut Engine, this: Ptr, entity: Ptr, flag: bool) -> bool {
    let old = enter_scope(e);
    let mut result = false;
    let current = e.vcall(this.addr(), 0xd0, &args![]).u32();
    if !flag {
        let world = e.vcall(entity.addr(), 0x94, &args![]).u32();
        let name = if world == 0 {
            0
        } else {
            e.mem.u32(world + 0x24)
        };
        if current == name {
            e.vcall(this.addr(), 0xc8, &args![0u32]);
        }
    } else {
        let linked = if current == 0 {
            0
        } else {
            e.mem.u32(current + 0xc)
        };
        if linked == 0 {
            result = true;
            let world = e.vcall(entity.addr(), 0x94, &args![]).u32();
            let name = if world == 0 {
                0
            } else {
                e.mem.u32(world + 0x24)
            };
            e.vcall(this.addr(), 0xc8, &args![name]);
        } else {
            let hk_object = e.mem.u32(linked + 8);
            let mut keep_flags = false;
            if hk_object != 0 {
                e.call(PROCESS_ENTITY_GUARD, &args![linked]);
                keep_flags = e.mem.u32(hk_object + 8) != 0;
                e.call(PROCESS_ENTITY_GUARD, &args![linked]);
            }
            if !keep_flags {
                let other = e.vcall(this.addr(), 0xcc, &args![]).u32();
                let high = e.mem.u32(other + 0x2c) & 0xffff_0000;
                let low = e.mem.u16(current + 0x2c) as u32;
                e.mem.set_u32(current + 0x2c, high | low);
            }
            if e.mem.u32(entity.addr() + 0x18) == 0 {
                e.vcall(linked, 0x9c, &args![entity]);
                let world = e.vcall(linked, 0x94, &args![]).u32();
                if world == 0 && e.mem.u8(linked + 0x10) & 0x30 == 0 {
                    write_error_report(e);
                    e.vcall(linked, 0xc8, &args![entity]);
                }
            }
        }
    }
    leave_scope(e, old);
    result
}

/// The report `ProcessEntityB` writes: an `hkOstream` over a 0x200-byte buffer holding
/// the text at `010c5e30`, sent to the sink at `01268360` (slot `0xC`) with the file name
/// at `010c5df0` and line 0x2ED, then destroyed.
fn write_error_report(e: &mut Engine) {
    e.with_stack(0x210, |e, block| {
        let stream = block.addr();
        let buffer = stream + 0xc;
        e.call(OSTREAM_CONSTRUCT, &args![stream, buffer, 0x200u32, 1u32]);
        e.call(OSTREAM_WRITE_TEXT, &args![stream, 0x010c_5e30u32]);
        let sink = e.global::<u32>(REPORT_SINK);
        e.vcall(
            sink,
            0xc,
            &args![1u32, 0x234f_24fcu32, buffer, 0x010c_5df0u32, 0x2edu32],
        );
        e.call(OSTREAM_DESTRUCT, &args![stream]);
    });
}

// Translated from 00c8a1b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8a1b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkConstraintCinfo::CloneData` (Xbox PDB), `(this, data, _unused_1)`: in the Havok
/// allocation scope, `SetData(data)` and then releases `data` once. The second stack word
/// is never read.
pub fn hk_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr<HkConstraintCinfo>,
    data: Ptr,
    _unused_1: u32,
) {
    let old = enter_scope(e);
    hk_constraint_cinfo_set_data(e, this, data);
    e.call(REMOVE_REFERENCE, &args![data]);
    leave_scope(e, old);
}

// Translated from 00c8a230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkConstraintCinfo::LoadBinary` (Xbox PDB): reads the link ids from the stream
/// (`00a64670`), calls virtual slot 4 (`+0x10`, the data builder), then reads the
/// priority (`+8`, 4 bytes) through the stream's reader at `+0x24C`.
pub fn hk_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr<HkConstraintCinfo>, stream: u32) {
    e.call(STREAM_READ_LINK_IDS, &args![stream]);
    e.vcall(this.addr(), 0x10, &args![]);
    let reader = e.mem.u32(stream + 0x24c);
    stream_transfer(
        e,
        reader,
        8,
        this.addr() + HkConstraintCinfo::ePriority.off,
        4,
    );
}

// Translated from 00c8a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkConstraintCinfo::SaveBinary` (Xbox PDB): through the stream's writer (`+0x250`)
/// writes the dword 2 (a local), then entity A's and entity B's `+0xC` (or 0) through the
/// stream's virtual slot `0x30`, calls virtual slot 4 (`+0x10`) and writes the priority
/// (`+8`, 4 bytes) through the writer again.
pub fn hk_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr<HkConstraintCinfo>, stream: u32) {
    let writer = e.mem.u32(stream + 0x250);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 2);
        stream_transfer(e, writer, 0xc, local.addr(), 4);
    });
    for entity in [
        e.get(this, HkConstraintCinfo::pEntityA),
        e.get(this, HkConstraintCinfo::pEntityB),
    ] {
        let owner = if entity.is_null() {
            0
        } else {
            e.mem.u32(entity.addr() + 0xc)
        };
        e.vcall(stream, 0x30, &args![owner]);
    }
    e.vcall(this.addr(), 0x10, &args![]);
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(
        e,
        writer,
        0xc,
        this.addr() + HkConstraintCinfo::ePriority.off,
        4,
    );
}

// Translated from 00c8a330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8a330 - the FS:[0] exception-unwinding frame is not modeled.
/// Constraint-motor loader (unnamed in the engine map; the range is the unit's own):
/// in the Havok allocation scope reads one type byte (and its count) through the stream's
/// reader (`+0x24C`), then builds the motor of that type: type 1 a 0x24-byte object
/// (constructor `00cf4190`, loaded by `00cf41c0`), type 2 a 0x20-byte one (`00cf43b0`,
/// `00cf4400`), type 3 a 0x1C-byte one (`00cf45d0`, `00cf4600`). Any other byte answers 0.
/// `thiscall` with `RET 4` (`ECX`, the constraint info, is not read): the stream is its one
/// stack word, as its callers (00cc6e91, the limited hinge's) push it.
pub fn fn_00c8a330(e: &mut Engine, _this: u32, stream: u32) -> u32 {
    let old = enter_scope(e);
    let reader = e.mem.u32(stream + 0x24c);
    let kind = e.with_stack(4, |e, local| {
        stream_transfer(e, reader, 8, local.addr(), 1);
        e.mem.u8(local.addr())
    });
    let (size, construct, load) = match kind {
        1 => (0x24, MOTOR1_CONSTRUCT, MOTOR1_LOAD),
        2 => (0x20, MOTOR2_CONSTRUCT, MOTOR2_LOAD),
        3 => (0x1c, MOTOR3_CONSTRUCT, MOTOR3_LOAD),
        _ => {
            leave_scope(e, old);
            return 0;
        }
    };
    let block = allocate_havok(e, size);
    let motor = e.call(construct, &args![block]).u32();
    e.call(load, &args![motor, stream]);
    leave_scope(e, old);
    motor
}

// Translated from 00c8a4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkConstraintCinfo::SaveMotor` (Xbox PDB), `(motor, stream)` with no `this`: writes the
/// motor's type byte (`motor + 8`, or a 0 byte for no motor) through the stream's writer
/// (`+0x250`), then for type 1, 2 or 3 (the byte as signed) calls the motor's saver
/// (`00cf4260`, `00cf4490`, `00cf4670`) with the stream.
pub fn hk_constraint_cinfo_save_motor(e: &mut Engine, motor: Ptr, stream: u32) {
    let writer = e.mem.u32(stream + 0x250);
    if motor.is_null() {
        e.with_stack(4, |e, zero| {
            stream_transfer(e, writer, 0xc, zero.addr(), 1);
        });
        return;
    }
    stream_transfer(e, writer, 0xc, motor.addr() + 8, 1);
    match e.mem.i8(motor.addr() + 8) {
        1 => e.call(MOTOR1_SAVE, &args![motor, stream]),
        2 => e.call(MOTOR2_SAVE, &args![motor, stream]),
        3 => e.call(MOTOR3_SAVE, &args![motor, stream]),
        _ => Ret::default(),
    };
}

/// Appends `text` (an address) to the viewer-string list the way the code does: when the
/// list's count (`+0xA`) has reached its capacity (`+8`) it grows by the grow-by (`+0xE`)
/// plus the count (`0096ad30`), then stores at the count's index (`0096ae90`).
pub(crate) fn add_viewer_string(e: &mut Engine, list: u32, text: u32) {
    let index = e.mem.u16(list + 0xa) as u32;
    let capacity = e.mem.u16(list + 8) as u32;
    if index >= capacity {
        let grow_by = e.mem.u16(list + 0xe) as u32;
        e.call(ARRAY_GROW, &args![list, grow_by + index]);
    }
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), text);
        e.call(ARRAY_SET_AT, &args![list, index, value]);
    });
}

// Translated from 00c8a560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8a560 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `hkConstraintCinfo::GetViewerStrings` (Xbox PDB): in the Havok allocation scope appends
/// to the viewer-string list: when the data (`+4`) exists, its type name (virtual slot 8,
/// `+0x20`, through `00cf4790`) under the name at `01097e6c`; the entity A text
/// (`%`-formatted by `sprintf_s` with the format at `010c4430` into a 0x20-byte buffer)
/// under the name at `010c5ed4`; likewise entity B under `010c5ec8`; and the priority
/// under `010c5ebc`.
pub fn hk_constraint_cinfo_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<HkConstraintCinfo>,
    list: u32,
) {
    let old = enter_scope(e);
    let data = e.get(this, HkConstraintCinfo::pData);
    if !data.is_null() {
        let kind = e.vcall(data.addr(), 0x20, &args![]).u32();
        let name = e.call(CONSTRAINT_TYPE_NAME, &args![kind]).u32();
        let text = e
            .call(VIEWER_STRING_TEXT, &args![0x0109_7e6cu32, name])
            .u32();
        add_viewer_string(e, list, text);
    }
    for (entity, label) in [
        (e.get(this, HkConstraintCinfo::pEntityA), 0x010c_5ed4u32),
        (e.get(this, HkConstraintCinfo::pEntityB), 0x010c_5ec8u32),
    ] {
        let text = e.with_stack(0x20, |e, buffer| {
            e.call(FORMAT_TEXT, &args![buffer, 0x20u32, 0x010c_4430u32, entity]);
            e.call(VIEWER_STRING_TEXT, &args![label, buffer]).u32()
        });
        add_viewer_string(e, list, text);
    }
    let priority = e.get(this, HkConstraintCinfo::ePriority);
    let text = e
        .call(VIEWER_STRING_INT, &args![0x010c_5ebcu32, priority])
        .u32();
    add_viewer_string(e, list, text);
    leave_scope(e, old);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00c89720, fn_00c89720_entry as AbiFn),
        entry!(0x00c89640, bhk_constraint_destructor(Ptr)),
        entry!(0x00c896b0, bhk_constraint_load_binary(Ptr, u32)),
        entry!(0x00c896c0, bhk_constraint_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c89730, bhk_constraint_copy_members(Ptr, u32, u32)),
        entry!(0x00c89810, bhk_constraint_load_c_info(Ptr, u32) -> i32),
        entry!(0x00c89860, bhk_constraint_link_object(Ptr, u32)),
        entry!(0x00c898e0, bhk_constraint_register_streamables(Ptr, u32) -> u32),
        entry!(0x00c89920, bhk_constraint_save_binary(Ptr, u32)),
        entry!(0x00c89960, bhk_constraint_get_viewer_strings(Ptr, u32)),
        entry!(
            0x00c899b0,
            bhk_constraint_gethk_entity_a(Ptr<BhkConstraint>) -> u32
        ),
        entry!(
            0x00c899c0,
            bhk_constraint_gethk_entity_b(Ptr<BhkConstraint>) -> u32
        ),
        entry!(0x00c899d0, bhk_constraint_link(Ptr, u32)),
        entry!(
            0x00c89a00,
            bhk_constraint_gethk_world(Ptr<BhkConstraint>) -> u32
        ),
        entry!(0x00c89a30, bhk_constraint_add(Ptr, Ptr) -> bool),
        entry!(
            0x00c89ac0,
            hk_constraint_cinfo_set_data(Ptr<HkConstraintCinfo>, Ptr)
        ),
        entry!(0x00c89b60, ahkp_constraint_instance_set_entity_a(Ptr, Ptr)),
        entry!(0x00c89bf0, ahkp_constraint_instance_set_entity_b(Ptr, Ptr)),
        entry!(
            0x00c89c80,
            bhk_constraint_copy_cinfo(Ptr, Ptr<HkConstraintCinfo>)
        ),
        entry!(0x00c89cd0, bhk_constraint_set_entity_a(Ptr, Ptr)),
        entry!(0x00c89cf0, bhk_constraint_set_entity_b(Ptr, Ptr)),
        entry!(0x00c89d10, bhk_constraint_sethk_object(Ptr, Ptr)),
        entry!(0x00c89dd0, bhk_constraint_init(Ptr, u32)),
        entry!(0x00c89ec0, bhk_constraint_remove(Ptr) -> bool),
        entry!(0x00c89f90, bhk_constraint_process_entity_b(Ptr, Ptr, bool) -> bool),
        entry!(
            0x00c8a1b0,
            hk_constraint_cinfo_clone_data(Ptr<HkConstraintCinfo>, Ptr, u32)
        ),
        entry!(
            0x00c8a230,
            hk_constraint_cinfo_load_binary(Ptr<HkConstraintCinfo>, u32)
        ),
        entry!(
            0x00c8a280,
            hk_constraint_cinfo_save_binary(Ptr<HkConstraintCinfo>, u32)
        ),
        entry!(0x00c8a330, fn_00c8a330(u32, u32) -> u32),
        entry!(0x00c8a4b0, hk_constraint_cinfo_save_motor(Ptr, u32)),
        entry!(
            0x00c8a560,
            hk_constraint_cinfo_get_viewer_strings(Ptr<HkConstraintCinfo>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    /// An object whose vtable (at `vtable`) has the given `(byte offset, target)` slots.
    fn object(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        let mut table = vec![0u32; 0x80];
        for (offset, target) in slots {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(vtable, &table);
        let object = e.mem.alloc(0x400);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Doubles that only answer 0, for every address given.
    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// The arguments of every logged call to `addr`.
    fn calls_to(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    /// The logged addresses in order.
    fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    fn word(e: &Engine, addr: u32) -> u32 {
        e.mem.u32(addr)
    }

    /// The per-thread allocation word.
    fn scope(e: &mut Engine) -> u32 {
        let word = e.tls() + SCOPE_WORD;
        e.mem.u32(word)
    }

    /// A constraint object whose slot `0xB4` answers `cinfo + 4`, read from `this + 0x30`.
    fn constraint_with_cinfo(e: &mut Engine, cinfo: u32, extra: &[(u32, u32)]) -> u32 {
        let mut slots = vec![(0xb4, 0x0900_0001)];
        slots.extend_from_slice(extra);
        let this = object(e, 0x0200_1000, &slots);
        e.mem.set_u32(this + 0x30, cinfo);
        e.register(0x0900_0001, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30) + 4,
            ..Ret::default()
        });
        this
    }

    #[test]
    fn destructor_sets_the_vtable_kills_counts_down_and_destructs() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(CONSTRAINT_COUNT, 3u32);
        quiet(&mut e, &[SERIALIZABLE_KILL, SERIALIZABLE_DESTRUCT]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00c89640, &args![this]);
        assert_eq!(word(&e, this), VTABLE_CONSTRAINT);
        assert_eq!(e.global::<u32>(CONSTRAINT_COUNT), 2);
        assert_eq!(
            order(&e),
            vec![0x00c89640, SERIALIZABLE_KILL, SERIALIZABLE_DESTRUCT]
        );
    }

    #[test]
    fn load_binary_forwards_to_the_serializable_load() {
        let mut e = Engine::new();
        quiet(&mut e, &[SERIALIZABLE_LOAD_BINARY]);
        e.call_log = Some(vec![]);
        e.call(0x00c896b0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_LOAD_BINARY),
            vec![vec![0x1000, 0x2000]]
        );
    }

    #[test]
    fn is_equal_needs_the_base_and_both_virtual_answers() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_IS_EQUAL, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x40),
            ..Ret::default()
        });
        e.register(0x0900_0010, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x20),
            ..Ret::default()
        });
        e.register(0x0900_0011, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x24),
            ..Ret::default()
        });
        let make = |e: &mut Engine, base: u32, first: u32, second: u32| {
            let o = object(e, 0x0200_1100, &[(0xcc, 0x0900_0010), (0xd0, 0x0900_0011)]);
            e.mem.set_u32(o + 0x40, base);
            e.mem.set_u32(o + 0x20, first);
            e.mem.set_u32(o + 0x24, second);
            o
        };
        let a = make(&mut e, 1, 5, 6);
        let b = make(&mut e, 1, 5, 6);
        let c = make(&mut e, 1, 5, 7);
        let d = make(&mut e, 1, 4, 6);
        let no_base = make(&mut e, 0, 5, 6);
        let is_equal = |e: &mut Engine, x: u32, y: u32| e.call(0x00c896c0, &args![x, y]).bool();
        assert!(is_equal(&mut e, a, b));
        assert!(!is_equal(&mut e, a, c));
        assert!(!is_equal(&mut e, a, d));
        assert!(!is_equal(&mut e, no_base, b));
    }

    #[test]
    fn copy_members_replaces_the_cinfo_entities_with_their_clones() {
        let mut e = Engine::new();
        let cinfo = object(&mut e, 0x0200_1200, &[(4, 0x0900_0002)]);
        let this = constraint_with_cinfo(&mut e, cinfo, &[]);
        // Entities A and B, each owned by a bhk object; B's owner clones itself on a miss.
        let hk_a = e.mem.alloc(0x20);
        let hk_b = e.mem.alloc(0x20);
        let clone_a = e.mem.alloc(0x20);
        e.mem.set_u32(clone_a + 8, hk_a);
        let clone_b = e.mem.alloc(0x20);
        e.mem.set_u32(clone_b + 8, hk_b);
        let owner_a = e.mem.alloc(0x20);
        let owner_b = object(&mut e, 0x0200_1300, &[(0x48, 0x0900_0003)]);
        e.mem.set_u32(owner_b + 0x30, clone_b);
        let entity_a = e.mem.alloc(0x20);
        e.mem.set_u32(entity_a + 0xc, owner_a);
        let entity_b = e.mem.alloc(0x20);
        e.mem.set_u32(entity_b + 0xc, owner_b);
        e.mem.set_u32(cinfo + 0xc, entity_a);
        e.mem.set_u32(cinfo + 0x10, entity_b);
        quiet(&mut e, &[0x0900_0002, SERIALIZABLE_COPY_MEMBERS]);
        e.register(0x0900_0003, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        // The map: only owner A is found, and its clone is at the map's word.
        e.register(MAP_LOOKUP, |e, a| {
            let found = e.mem.u32(a[0] + 4) == a[1];
            if found {
                let clone = e.mem.u32(a[0] + 8);
                e.mem.set_u32(a[2], clone);
            }
            Ret {
                eax: found as u32,
                ..Ret::default()
            }
        });
        let map = e.mem.alloc(0x20);
        e.mem.set_u32(map + 4, owner_a);
        e.mem.set_u32(map + 8, clone_a);
        let process = e.mem.alloc(0x40);
        e.mem.set_u32(process, map);
        e.mem.set_f32(process + 0x10, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x00c89730, &args![this, 0x7777u32, process]);
        assert_eq!(calls_to(&e, 0x0900_0002), vec![vec![cinfo, 0, process]]);
        assert_eq!(word(&e, cinfo + 0xc), hk_a);
        assert_eq!(word(&e, cinfo + 0x10), hk_b);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_COPY_MEMBERS),
            vec![vec![this, 0x7777, process]]
        );
        assert_eq!(calls_to(&e, 0x0900_0003), vec![vec![owner_b, process]]);
    }

    #[test]
    fn copy_members_keeps_the_cinfo_untouched_at_one_and_clears_missing_clones() {
        let mut e = Engine::new();
        let cinfo = object(&mut e, 0x0200_1200, &[(4, 0x0900_0002)]);
        let this = constraint_with_cinfo(&mut e, cinfo, &[]);
        let owner = e.mem.alloc(0x20);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 0xc, owner);
        e.mem.set_u32(cinfo + 0xc, entity);
        quiet(
            &mut e,
            &[0x0900_0002, SERIALIZABLE_COPY_MEMBERS, MAP_LOOKUP],
        );
        let process = e.mem.alloc(0x40);
        e.mem.set_f32(process + 0x10, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00c89730, &args![this, 0u32, process]);
        assert!(calls_to(&e, 0x0900_0002).is_empty());
        // The lookup missed: the entity slot stays cleared.
        assert_eq!(word(&e, cinfo + 0xc), 0);
        // NaN is not 1.0, so the cinfo is asked.
        e.mem.set_f32(process + 0x10, f32::NAN);
        e.call(0x00c89730, &args![this, 0u32, process]);
        assert_eq!(calls_to(&e, 0x0900_0002).len(), 1);
    }

    #[test]
    fn load_c_info_loads_clears_and_calls_slot_0xa8() {
        let mut e = Engine::new();
        let cinfo = object(&mut e, 0x0200_1200, &[(8, 0x0900_0002)]);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data + 8, 0x99);
        e.mem.set_u32(cinfo + 4, data);
        let this = constraint_with_cinfo(&mut e, cinfo, &[(0xa8, 0x0900_0004)]);
        quiet(&mut e, &[0x0900_0002, 0x0900_0004]);
        e.call_log = Some(vec![]);
        e.call(0x00c89810, &args![this, 0x4242u32]);
        assert_eq!(calls_to(&e, 0x0900_0002), vec![vec![cinfo, 0x4242]]);
        assert_eq!(word(&e, data + 8), 0);
        assert_eq!(calls_to(&e, 0x0900_0004), vec![vec![this]]);
    }

    #[test]
    fn link_object_sets_the_entities_only_when_the_first_exists() {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x20);
        let this = constraint_with_cinfo(&mut e, cinfo, &[]);
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(first + 8, 0xaaa);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(second + 8, 0xbbb);
        quiet(&mut e, &[STREAM_BEFORE_LINKS, SERIALIZABLE_LINK_OBJECT]);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, {
            let mut queue = vec![second, first];
            move |_, _| Ret {
                eax: queue.pop().unwrap(),
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00c89860, &args![this, 0x5000u32]);
        assert_eq!(word(&e, cinfo + 0xc), 0xaaa);
        assert_eq!(word(&e, cinfo + 0x10), 0xbbb);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_LINK_OBJECT),
            vec![vec![this, 0x5000]]
        );
        // No first object: nothing is stored, even though a second exists.
        e.mem.set_u32(cinfo + 0xc, 1);
        e.mem.set_u32(cinfo + 0x10, 2);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, {
            let mut queue = vec![second, 0];
            move |_, _| Ret {
                eax: queue.pop().unwrap(),
                ..Ret::default()
            }
        });
        e.call(0x00c89860, &args![this, 0x5000u32]);
        assert_eq!((word(&e, cinfo + 0xc), word(&e, cinfo + 0x10)), (1, 2));
    }

    #[test]
    fn register_streamables_registers_the_owner_then_the_base() {
        let mut e = Engine::new();
        let owner_target = object(&mut e, 0x0200_1300, &[(0x54, 0x0900_0005)]);
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0xc, owner_target);
        let this = object(&mut e, 0x0200_1400, &[(0xd0, 0x0900_0006)]);
        e.mem.set_u32(this + 0x30, first);
        e.register(0x0900_0006, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        quiet(&mut e, &[0x0900_0005, NI_OBJECT_REGISTER_STREAMABLES]);
        e.call_log = Some(vec![]);
        e.call(0x00c898e0, &args![this, 0x6000u32]);
        assert_eq!(calls_to(&e, 0x0900_0005), vec![vec![owner_target, 0x6000]]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_REGISTER_STREAMABLES),
            vec![vec![this, 0x6000]]
        );
        // Without a first object only the base runs.
        e.mem.set_u32(this + 0x30, 0);
        e.call(0x00c898e0, &args![this, 0x6000u32]);
        assert_eq!(calls_to(&e, 0x0900_0005).len(), 1);
        assert_eq!(calls_to(&e, NI_OBJECT_REGISTER_STREAMABLES).len(), 2);
    }

    #[test]
    fn save_binary_calls_the_cinfo_slot_3() {
        let mut e = Engine::new();
        let cinfo = object(&mut e, 0x0200_1200, &[(0xc, 0x0900_0002)]);
        let this = constraint_with_cinfo(&mut e, cinfo, &[]);
        quiet(&mut e, &[0x0900_0002]);
        e.call_log = Some(vec![]);
        e.call(0x00c89920, &args![this, 0x7000u32]);
        assert_eq!(calls_to(&e, 0x0900_0002), vec![vec![cinfo, 0x7000]]);
    }

    #[test]
    #[should_panic]
    fn save_binary_without_a_cinfo_reads_address_zero() {
        let mut e = Engine::new();
        let this = constraint_with_cinfo(&mut e, 0, &[]);
        // The cinfo getter answers 4 (cinfo 0 + 4); make it answer 0 instead.
        e.register(0x0900_0001, |_, _| Ret::default());
        e.call(0x00c89920, &args![this, 0x7000u32]);
    }

    #[test]
    fn get_viewer_strings_runs_base_cinfo_and_slot_0xa4() {
        let mut e = Engine::new();
        let cinfo = object(&mut e, 0x0200_1200, &[(0x14, 0x0900_0002)]);
        let this = constraint_with_cinfo(&mut e, cinfo, &[(0xa4, 0x0900_0004)]);
        quiet(
            &mut e,
            &[0x0900_0002, 0x0900_0004, SERIALIZABLE_GET_VIEWER_STRINGS],
        );
        e.call_log = Some(vec![]);
        e.call(0x00c89960, &args![this, 0x8000u32]);
        assert_eq!(
            order(&e),
            vec![
                0x00c89960,
                SERIALIZABLE_GET_VIEWER_STRINGS,
                0x0900_0001,
                0x0900_0002,
                0x0900_0004
            ]
        );
        assert_eq!(calls_to(&e, 0x0900_0002), vec![vec![cinfo, 0x8000]]);
        assert_eq!(calls_to(&e, 0x0900_0004), vec![vec![this, 1]]);
    }

    #[test]
    fn entity_getters_read_the_havok_constraint() {
        let mut e = Engine::new();
        let this: Ptr<BhkConstraint> = e.new_object();
        assert_eq!(e.call(0x00c899b0, &args![this]).u32(), 0);
        assert_eq!(e.call(0x00c899c0, &args![this]).u32(), 0);
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance + 0x14, 0xa1);
        e.mem.set_u32(instance + 0x18, 0xb2);
        e.set(this, BhkConstraint::phkObject, Ptr::new(instance));
        assert_eq!(e.call(0x00c899b0, &args![this]).u32(), 0xa1);
        assert_eq!(e.call(0x00c899c0, &args![this]).u32(), 0xb2);
    }

    #[test]
    fn link_stores_this_by_the_low_byte_then_jumps_to_the_base() {
        let mut e = Engine::new();
        quiet(&mut e, &[REF_OBJECT_LINK]);
        let this = e.mem.alloc(0x40);
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, instance);
        e.call_log = Some(vec![]);
        e.call(0x00c899d0, &args![this, 0x100u32]);
        assert_eq!(word(&e, instance + 0x2c), 0);
        e.call(0x00c899d0, &args![this, 0x0101u32]);
        assert_eq!(word(&e, instance + 0x2c), this);
        assert_eq!(
            calls_to(&e, REF_OBJECT_LINK),
            vec![vec![this, 0x100], vec![this, 0x101]]
        );
        // No Havok constraint: only the jump.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c899d0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, REF_OBJECT_LINK).len(), 3);
    }

    #[test]
    fn gethk_world_follows_slot_0xcc_to_its_havok_object() {
        let mut e = Engine::new();
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 8, 0xcafe);
        let this = object(&mut e, 0x0200_1500, &[(0xcc, 0x0900_0007)]);
        e.register(0x0900_0007, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        e.mem.set_u32(this + 0x30, owner);
        let instance = e.mem.alloc(0x20);
        let this_typed: Ptr<BhkConstraint> = Ptr::new(this);
        assert_eq!(e.call(0x00c89a00, &args![this_typed]).u32(), 0);
        e.set(this_typed, BhkConstraint::phkObject, Ptr::new(instance));
        assert_eq!(e.call(0x00c89a00, &args![this_typed]).u32(), 0);
        e.mem.set_u32(instance + 8, 1);
        assert_eq!(e.call(0x00c89a00, &args![this_typed]).u32(), 0xcafe);
        e.mem.set_u32(this + 0x30, 0);
        assert_eq!(e.call(0x00c89a00, &args![this_typed]).u32(), 0);
    }

    #[test]
    fn add_moves_the_constraint_into_a_different_world() {
        let mut e = Engine::new();
        let vt = [
            (0x94, 0x0900_0008),
            (0xa0, 0x0900_0009),
            (0xd4, 0x0900_000a),
        ];
        let this = object(&mut e, 0x0200_1600, &vt);
        let world = object(&mut e, 0x0200_1700, &vt);
        e.register(0x0900_0008, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        quiet(&mut e, &[0x0900_0009, 0x0900_000a, WORLD_ADD]);
        let instance = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, instance);
        e.mem.set_u32(this + 0x30, 0x111);
        e.mem.set_u32(world + 0x30, 0x222);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c89a30, &args![this, world]).bool());
        assert_eq!(
            order(&e),
            vec![
                0x00c89a30,
                0x0900_0008,
                0x0900_0008,
                0x0900_0009,
                0x0900_000a,
                WORLD_ADD
            ]
        );
        assert_eq!(calls_to(&e, 0x0900_000a), vec![vec![this, world, 1]]);
        assert_eq!(calls_to(&e, WORLD_ADD), vec![vec![world, instance]]);
        // The same world: nothing happens.
        e.mem.set_u32(world + 0x30, 0x111);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c89a30, &args![this, world]).bool());
        assert_eq!(calls_to(&e, WORLD_ADD).len(), 0);
        // No world: slot 0xA0 only. No Havok constraint: false at once.
        assert!(!e.call(0x00c89a30, &args![this, 0u32]).bool());
        assert_eq!(calls_to(&e, 0x0900_0009), vec![vec![this]]);
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c89a30, &args![this, world]).bool());
        assert_eq!(order(&e), vec![0x00c89a30]);
    }

    /// `AddReference`/`RemoveReference` doubles and the scope word, shared by the
    /// reference-handling tests.
    fn reference_doubles(e: &mut Engine) {
        quiet(e, &[ADD_REFERENCE, REMOVE_REFERENCE]);
        // The scope word starts at a recognisable value so its restore is visible.
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x77);
    }

    #[test]
    fn set_data_swaps_the_data_with_references() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        let cinfo: Ptr<HkConstraintCinfo> = e.new_object();
        let old_data = e.mem.alloc(0x20);
        e.mem.set_u32(old_data + 8, 0x55);
        e.set(cinfo, HkConstraintCinfo::pData, Ptr::new(old_data));
        let new_data = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00c89ac0, &args![cinfo, new_data]);
        assert_eq!(word(&e, old_data + 8), 0);
        assert_eq!(word(&e, new_data + 8), cinfo.addr());
        assert_eq!(e.get(cinfo, HkConstraintCinfo::pData).addr(), new_data);
        assert_eq!(order(&e), vec![0x00c89ac0, REMOVE_REFERENCE, ADD_REFERENCE]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![old_data]]);
        assert_eq!(scope(&mut e), 0x77);
        // Clearing the data releases without referencing.
        e.call_log = Some(vec![]);
        e.call(0x00c89ac0, &args![cinfo, 0u32]);
        assert_eq!(order(&e), vec![0x00c89ac0, REMOVE_REFERENCE]);
        assert!(e.get(cinfo, HkConstraintCinfo::pData).is_null());
    }

    #[test]
    fn set_entity_a_and_b_swap_the_words_at_0x14_and_0x18() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance + 0x14, 0xa0);
        e.mem.set_u32(instance + 0x18, 0xb0);
        e.call_log = Some(vec![]);
        e.call(0x00c89b60, &args![instance, 0xa1u32]);
        e.call(0x00c89bf0, &args![instance, 0u32]);
        assert_eq!(word(&e, instance + 0x14), 0xa1);
        assert_eq!(word(&e, instance + 0x18), 0);
        assert_eq!(
            order(&e),
            vec![
                0x00c89b60,
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                0x00c89bf0,
                REMOVE_REFERENCE
            ]
        );
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![0xa1]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0xa0], vec![0xb0]]);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn copy_cinfo_copies_or_clears() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        let this = e.mem.alloc(0x20);
        let instance = e.mem.alloc(0x40);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, instance);
        e.mem.set_u32(instance + 0xc, data);
        e.mem.set_u32(instance + 0x14, 0xa1);
        e.mem.set_u32(instance + 0x18, 0xb2);
        e.mem.set_u32(instance + 0x1c, 0x1234_5603);
        let cinfo: Ptr<HkConstraintCinfo> = e.new_object();
        e.mem.set_u32(cinfo.addr() + 8, 0xffff);
        e.call(0x00c89c80, &args![this, cinfo]);
        assert_eq!(e.get(cinfo, HkConstraintCinfo::pData).addr(), data);
        assert_eq!(e.get(cinfo, HkConstraintCinfo::pEntityA).addr(), 0xa1);
        assert_eq!(e.get(cinfo, HkConstraintCinfo::pEntityB).addr(), 0xb2);
        assert_eq!(e.get(cinfo, HkConstraintCinfo::ePriority), 3);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c89c80, &args![this, cinfo]);
        assert!(e.get(cinfo, HkConstraintCinfo::pData).is_null());
        assert!(e.get(cinfo, HkConstraintCinfo::pEntityA).is_null());
        assert!(e.get(cinfo, HkConstraintCinfo::pEntityB).is_null());
        assert_eq!(e.get(cinfo, HkConstraintCinfo::ePriority), 3);
    }

    #[test]
    fn set_entity_a_and_b_forward_to_the_havok_constraint() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        let this = e.mem.alloc(0x20);
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, instance);
        e.call(0x00c89cd0, &args![this, 0x11u32]);
        e.call(0x00c89cf0, &args![this, 0x22u32]);
        assert_eq!(
            (word(&e, instance + 0x14), word(&e, instance + 0x18)),
            (0x11, 0x22)
        );
        // Without a Havok constraint, or without this, nothing happens.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c89cd0, &args![this, 0x33u32]);
        e.call(0x00c89cf0, &args![0u32, 0x44u32]);
        assert_eq!(
            (word(&e, instance + 0x14), word(&e, instance + 0x18)),
            (0x11, 0x22)
        );
    }

    #[test]
    fn sethk_object_brackets_the_old_object_with_a_use_count() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        quiet(&mut e, &[REF_OBJECT_SET_HK_OBJECT]);
        let holder = e.mem.alloc(0x100);
        e.mem.set_u32(holder + 0x94, 5);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 8, holder);
        let hk = object(&mut e, 0x0200_1800, &[(0x10, 0x0900_000b)]);
        let current = e.mem.alloc(0x40);
        e.mem.set_u32(current + 8, hk);
        e.mem.set_u32(current + 0x14, entity);
        e.register(0x0900_000b, |e, a| {
            // During the call the use count is one higher.
            let holder = e.mem.u32(e.mem.u32(a[1] + 0x14) + 8);
            let count = e.mem.u32(holder + 0x94);
            e.mem.set_u32(a[1] + 0x38, count);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, current);
        e.call_log = Some(vec![]);
        e.call(0x00c89d10, &args![this, 0x1234u32]);
        assert_eq!(word(&e, current + 0x38), 6);
        assert_eq!(word(&e, holder + 0x94), 5);
        assert_eq!(
            calls_to(&e, REF_OBJECT_SET_HK_OBJECT),
            vec![vec![this, 0x1234]]
        );
        // The same object: nothing is called.
        e.call_log = Some(vec![]);
        e.call(0x00c89d10, &args![this, current]);
        assert_eq!(order(&e), vec![0x00c89d10]);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn init_builds_the_constraint_instance_from_the_cinfo() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        // The allocator the memory router hands out: slot 4 allocates.
        let allocator = object(&mut e, 0x0200_1900, &[(4, 0x0900_000c)]);
        e.register(0x0900_000c, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        e.register(MEMORY_ROUTER, |e, _| {
            // The test stores the router at a fixed place.
            Ret {
                eax: e.mem.u32(0x0200_2000),
                ..Ret::default()
            }
        });
        e.map(0x0200_2000, 0x1000);
        e.mem.set_u32(0x0200_2000, router);
        e.register(CONSTRAINT_INSTANCE_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = object(
            &mut e,
            0x0200_1a00,
            &[(0x8c, 0x0900_000d), (0xbc, 0x0900_000e)],
        );
        quiet(&mut e, &[0x0900_000d, 0x0900_000e]);
        let cinfo = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 4, 0xda7a);
        e.mem.set_u32(cinfo + 8, 3);
        e.mem.set_u32(cinfo + 0xc, 0xe1);
        e.mem.set_u32(cinfo + 0x10, 0xe2);
        e.call_log = Some(vec![]);
        e.call(0x00c89dd0, &args![this, cinfo + 4]);
        let built = calls_to(&e, CONSTRAINT_INSTANCE_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(&built[0][1..], &[0xe1, 0xe2, 0xda7a, 3]);
        let instance = built[0][0];
        assert_eq!(e.mem.block_size(instance), Some(0x38));
        assert_eq!(e.mem.u16(instance + 4), 0x38);
        assert_eq!(calls_to(&e, 0x0900_000d), vec![vec![this, instance]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![instance]]);
        assert_eq!(calls_to(&e, 0x0900_000e), vec![vec![this, cinfo + 4]]);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn remove_takes_the_constraint_out_of_the_havok_world() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        let this = object(
            &mut e,
            0x0200_1b00,
            &[(0x94, 0x0900_000f), (0xd4, 0x0900_0010)],
        );
        e.register(0x0900_000f, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        quiet(&mut e, &[0x0900_0010]);
        e.register(WORLD_REMOVE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0x350, 0xbeef);
        e.mem.set_u32(this + 0x30, world);
        e.mem.set_u32(this + 8, 0x1c1c);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c89ec0, &args![this]).bool());
        assert_eq!(calls_to(&e, WORLD_REMOVE), vec![vec![0xbeef, 0x1c1c]]);
        assert_eq!(calls_to(&e, 0x0900_0010), vec![vec![this, 0xbeef, 0]]);
        assert_eq!(scope(&mut e), 0x77);
        // No world: false, nothing removed.
        e.mem.set_u32(this + 0x30, 0);
        assert!(!e.call(0x00c89ec0, &args![this]).bool());
        assert_eq!(calls_to(&e, WORLD_REMOVE).len(), 1);
    }

    /// An `ProcessEntityB` fixture: `this` (slots 0xC8, 0xCC, 0xD0), the entity (slot
    /// 0x94 answering a world whose `+0x24` is 0x77), and the guard doubles.
    fn entity_fixture(e: &mut Engine) -> (u32, u32) {
        reference_doubles(e);
        let this = object(
            e,
            0x0200_1c00,
            &[
                (0xc8, 0x0900_0011),
                (0xcc, 0x0900_0012),
                (0xd0, 0x0900_0013),
            ],
        );
        e.register(0x0900_0012, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x34),
            ..Ret::default()
        });
        e.register(0x0900_0013, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        let world = e.mem.alloc(0x40);
        e.mem.set_u32(world + 0x24, 0x77);
        let entity = object(e, 0x0200_1d00, &[(0x94, 0x0900_0014)]);
        e.register(0x0900_0014, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        e.mem.set_u32(entity + 0x30, world);
        quiet(e, &[0x0900_0011, PROCESS_ENTITY_GUARD]);
        (this, entity)
    }

    #[test]
    fn process_entity_b_without_the_flag_compares_the_names() {
        let mut e = Engine::new();
        let (this, entity) = entity_fixture(&mut e);
        e.mem.set_u32(this + 0x30, 0x77);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c89f90, &args![this, entity, false]).bool());
        assert_eq!(calls_to(&e, 0x0900_0011), vec![vec![this, 0]]);
        e.mem.set_u32(this + 0x30, 0x78);
        e.call(0x00c89f90, &args![this, entity, false]);
        assert_eq!(calls_to(&e, 0x0900_0011).len(), 1);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn process_entity_b_with_the_flag_and_no_linked_object_answers_true() {
        let mut e = Engine::new();
        let (this, entity) = entity_fixture(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c89f90, &args![this, entity, true]).bool());
        assert_eq!(calls_to(&e, 0x0900_0011), vec![vec![this, 0x77]]);
    }

    #[test]
    fn process_entity_b_with_the_flag_repairs_flags_and_reports_through_the_sink() {
        let mut e = Engine::new();
        let (this, entity) = entity_fixture(&mut e);
        // current -> linked (slot 0xC8 and 0x94/0x9C live on linked) -> hk object.
        let linked = object(
            &mut e,
            0x0200_1e00,
            &[
                (0x94, 0x0900_0015),
                (0x9c, 0x0900_0016),
                (0xc8, 0x0900_0017),
            ],
        );
        quiet(&mut e, &[0x0900_0015, 0x0900_0016, 0x0900_0017]);
        let hk = e.mem.alloc(0x20);
        e.mem.set_u32(linked + 8, hk);
        let current = e.mem.alloc(0x40);
        e.mem.set_u32(current + 0xc, linked);
        e.mem.set_u16(current + 0x2c, 0x1234);
        e.mem.set_u32(current + 0x2c, 0xabcd_1234);
        e.mem.set_u32(this + 0x30, current);
        let donor = e.mem.alloc(0x40);
        e.mem.set_u32(donor + 0x2c, 0x5678_9abc);
        e.mem.set_u32(this + 0x34, donor);
        // The report machinery.
        quiet(
            &mut e,
            &[OSTREAM_CONSTRUCT, OSTREAM_WRITE_TEXT, OSTREAM_DESTRUCT],
        );
        let sink = object(&mut e, 0x0200_1f00, &[(0xc, 0x0900_0018)]);
        quiet(&mut e, &[0x0900_0018]);
        e.map(REPORT_SINK, 4);
        e.set_global(REPORT_SINK, sink);
        e.call_log = Some(vec![]);
        // hk +8 is 0, so the flags are rebuilt; entity +0x18 is 0 and the linked object's
        // world answers 0 with no 0x30 bits: the report is written.
        assert!(!e.call(0x00c89f90, &args![this, entity, true]).bool());
        assert_eq!(word(&e, current + 0x2c), 0x5678_1234);
        assert_eq!(calls_to(&e, PROCESS_ENTITY_GUARD).len(), 2);
        assert_eq!(calls_to(&e, 0x0900_0016), vec![vec![linked, entity]]);
        let construct = calls_to(&e, OSTREAM_CONSTRUCT);
        assert_eq!(construct.len(), 1);
        assert_eq!(&construct[0][2..], &[0x200, 1]);
        assert_eq!(calls_to(&e, OSTREAM_WRITE_TEXT)[0][1], 0x010c_5e30);
        assert_eq!(
            calls_to(&e, 0x0900_0018),
            vec![vec![
                sink,
                1,
                0x234f_24fc,
                construct[0][1],
                0x010c_5df0,
                0x2ed
            ]]
        );
        assert_eq!(calls_to(&e, OSTREAM_DESTRUCT), vec![vec![construct[0][0]]]);
        assert_eq!(calls_to(&e, 0x0900_0017), vec![vec![linked, entity]]);
        // The Havok object has a nonzero +8: the flags are kept; entity +0x18 set: stop.
        e.mem.set_u32(hk + 8, 1);
        e.mem.set_u32(entity + 0x18, 1);
        e.mem.set_u32(current + 0x2c, 0xabcd_1234);
        e.call_log = Some(vec![]);
        e.call(0x00c89f90, &args![this, entity, true]);
        assert_eq!(word(&e, current + 0x2c), 0xabcd_1234);
        assert!(calls_to(&e, 0x0900_0016).is_empty());
    }

    #[test]
    fn clone_data_sets_the_data_and_releases_it_once() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        let cinfo: Ptr<HkConstraintCinfo> = e.new_object();
        let data = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00c8a1b0, &args![cinfo, data, 0xdeadu32]);
        assert_eq!(e.get(cinfo, HkConstraintCinfo::pData).addr(), data);
        assert_eq!(order(&e), vec![0x00c8a1b0, ADD_REFERENCE, REMOVE_REFERENCE]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![data]]);
        assert_eq!(scope(&mut e), 0x77);
    }

    /// A stream whose reader (`+0x24C`) and writer (`+0x250`) objects keep a log: their
    /// function pointers at `+8` and `+0xC` are doubles that copy to or from a script.
    fn stream_fixture(e: &mut Engine) -> (u32, u32, u32) {
        let stream = object(e, 0x0200_2100, &[(0x30, 0x0900_0020)]);
        let reader = e.mem.alloc(0x20);
        let writer = e.mem.alloc(0x20);
        e.mem.set_u32(reader + 8, 0x0900_0021);
        e.mem.set_u32(writer + 0xc, 0x0900_0022);
        e.mem.set_u32(stream + 0x24c, reader);
        e.mem.set_u32(stream + 0x250, writer);
        quiet(e, &[0x0900_0020]);
        // Reads fill the destination from the reader's byte at +0x10 repeated; they
        // answer the size.
        e.register(0x0900_0021, |e, a| {
            let value = e.mem.u32(a[0] + 0x10);
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, (value >> (8 * i)) as u8);
            }
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        // Writes append the source to the writer's log at +0x100.
        e.register(0x0900_0022, |e, a| {
            let at = e.mem.u32(a[0] + 0x14);
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + 0x100 + at + i, byte);
            }
            e.mem.set_u32(a[0] + 0x14, at + a[2]);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        (stream, reader, writer)
    }

    #[test]
    fn cinfo_load_binary_reads_the_priority_through_the_reader() {
        let mut e = Engine::new();
        let (stream, reader, _) = stream_fixture(&mut e);
        quiet(&mut e, &[STREAM_READ_LINK_IDS]);
        let cinfo = object(&mut e, 0x0200_2200, &[(0x10, 0x0900_0023)]);
        quiet(&mut e, &[0x0900_0023]);
        e.mem.set_u32(reader + 0x10, 0x0102_0304);
        e.call_log = Some(vec![]);
        e.call(0x00c8a230, &args![cinfo, stream]);
        assert_eq!(word(&e, cinfo + 8), 0x0102_0304);
        assert_eq!(word(&e, reader + 4), 4);
        assert_eq!(
            order(&e),
            vec![0x00c8a230, STREAM_READ_LINK_IDS, 0x0900_0023, 0x0900_0021]
        );
        let read = calls_to(&e, 0x0900_0021);
        assert_eq!(
            (read[0][0], read[0][1], read[0][2], read[0][4]),
            (reader, cinfo + 8, 4, 1)
        );
    }

    #[test]
    fn cinfo_save_binary_writes_version_entities_data_and_priority() {
        let mut e = Engine::new();
        let (stream, _, writer) = stream_fixture(&mut e);
        let cinfo = object(&mut e, 0x0200_2200, &[(0x10, 0x0900_0023)]);
        quiet(&mut e, &[0x0900_0023]);
        let entity_a = e.mem.alloc(0x20);
        e.mem.set_u32(entity_a + 0xc, 0xaa);
        e.mem.set_u32(cinfo + 0xc, entity_a);
        e.mem.set_u32(cinfo + 8, 0x0a0b_0c0d);
        e.call_log = Some(vec![]);
        e.call(0x00c8a280, &args![cinfo, stream]);
        assert_eq!(
            calls_to(&e, 0x0900_0020),
            vec![vec![stream, 0xaa], vec![stream, 0]]
        );
        // The writer received 2, then the priority.
        assert_eq!(word(&e, writer + 0x100), 2);
        assert_eq!(word(&e, writer + 0x104), 0x0a0b_0c0d);
        assert_eq!(word(&e, writer + 4), 8);
        assert_eq!(
            order(&e),
            vec![
                0x00c8a280,
                0x0900_0022,
                0x0900_0020,
                0x0900_0020,
                0x0900_0023,
                0x0900_0022
            ]
        );
    }

    #[test]
    fn motor_loader_builds_the_motor_of_the_byte_read() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        let (stream, reader, _) = stream_fixture(&mut e);
        let allocator = object(&mut e, 0x0200_2300, &[(4, 0x0900_0024)]);
        e.register(0x0900_0024, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        e.map(0x0200_3000, 0x1000);
        e.mem.set_u32(0x0200_3000, router);
        e.register(MEMORY_ROUTER, |e, _| Ret {
            eax: e.mem.u32(0x0200_3000),
            ..Ret::default()
        });
        for construct in [MOTOR1_CONSTRUCT, MOTOR2_CONSTRUCT, MOTOR3_CONSTRUCT] {
            e.register(construct, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
        quiet(&mut e, &[MOTOR1_LOAD, MOTOR2_LOAD, MOTOR3_LOAD]);
        for (kind, size, construct, load) in [
            (1u32, 0x24u32, MOTOR1_CONSTRUCT, MOTOR1_LOAD),
            (2, 0x20, MOTOR2_CONSTRUCT, MOTOR2_LOAD),
            (3, 0x1c, MOTOR3_CONSTRUCT, MOTOR3_LOAD),
        ] {
            e.mem.set_u32(reader + 0x10, kind);
            e.call_log = Some(vec![]);
            let motor = e.call(0x00c8a330, &args![0u32, stream]).u32();
            assert_eq!(e.mem.block_size(motor), Some(size.div_ceil(8) * 8));
            assert_eq!(e.mem.u16(motor + 4), size as u16);
            assert_eq!(calls_to(&e, construct), vec![vec![motor]]);
            assert_eq!(calls_to(&e, load), vec![vec![motor, stream]]);
        }
        e.mem.set_u32(reader + 0x10, 4);
        assert_eq!(e.call(0x00c8a330, &args![0u32, stream]).u32(), 0);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn save_motor_writes_the_type_byte_and_the_motor() {
        let mut e = Engine::new();
        let (stream, _, writer) = stream_fixture(&mut e);
        quiet(&mut e, &[MOTOR1_SAVE, MOTOR2_SAVE, MOTOR3_SAVE]);
        let motor = e.mem.alloc(0x40);
        for (kind, save) in [(1u8, MOTOR1_SAVE), (2, MOTOR2_SAVE), (3, MOTOR3_SAVE)] {
            e.mem.set_u8(motor + 8, kind);
            e.call_log = Some(vec![]);
            e.call(0x00c8a4b0, &args![motor, stream]);
            assert_eq!(calls_to(&e, save), vec![vec![motor, stream]]);
        }
        assert_eq!(e.mem.u8(writer + 0x100), 1);
        assert_eq!(e.mem.u8(writer + 0x101), 2);
        assert_eq!(e.mem.u8(writer + 0x102), 3);
        // An unknown type writes only its byte; no motor writes a zero byte.
        e.mem.set_u8(motor + 8, 9);
        e.call_log = Some(vec![]);
        e.call(0x00c8a4b0, &args![motor, stream]);
        e.call(0x00c8a4b0, &args![0u32, stream]);
        assert_eq!((e.mem.u8(writer + 0x103), e.mem.u8(writer + 0x104)), (9, 0));
        assert_eq!(word(&e, writer + 4), 5);
        assert_eq!(
            order(&e),
            vec![0x00c8a4b0, 0x0900_0022, 0x00c8a4b0, 0x0900_0022]
        );
    }

    #[test]
    fn viewer_strings_list_the_type_entities_and_priority() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        let data = object(&mut e, 0x0200_2400, &[(0x20, 0x0900_0025)]);
        e.register(0x0900_0025, |_, _| Ret {
            eax: 0x5151,
            ..Ret::default()
        });
        e.register(CONSTRAINT_TYPE_NAME, |_, a| Ret {
            eax: 0x6000 + a[0],
            ..Ret::default()
        });
        e.register(VIEWER_STRING_TEXT, |_, a| Ret {
            eax: a[0] ^ a[1],
            ..Ret::default()
        });
        e.register(VIEWER_STRING_INT, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        quiet(&mut e, &[FORMAT_TEXT, ARRAY_GROW]);
        e.register(ARRAY_SET_AT, |e, a| {
            // The list is a log: store the value at the index in the array at +0x100.
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x100 + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let list = e.mem.alloc(0x200);
        e.mem.set_u16(list + 8, 2);
        e.mem.set_u16(list + 0xe, 3);
        let cinfo: Ptr<HkConstraintCinfo> = e.new_object();
        e.set(cinfo, HkConstraintCinfo::pData, Ptr::new(data));
        e.set(cinfo, HkConstraintCinfo::ePriority, 7);
        e.mem.set_u32(cinfo.addr() + 0xc, 0xe1);
        e.mem.set_u32(cinfo.addr() + 0x10, 0xe2);
        e.call_log = Some(vec![]);
        e.call(0x00c8a560, &args![cinfo, list]);
        let at = |i: u32| word(&e, list + 0x100 + 4 * i);
        assert_eq!(at(0), 0x0109_7e6c ^ (0x6000 + 0x5151));
        assert_eq!(at(3), 0x010c_5ebc + 7);
        assert_eq!(word(&e, list + 0xa), 4);
        // Entity text is formatted into a 0x20-byte buffer: (buffer, 0x20, format, entity).
        let formats = calls_to(&e, FORMAT_TEXT);
        assert_eq!(
            formats
                .iter()
                .map(|c| (c[1], c[2], c[3]))
                .collect::<Vec<_>>(),
            vec![(0x20, 0x010c_4430, 0xe1), (0x20, 0x010c_4430, 0xe2)]
        );
        // The list grew when its count reached its capacity: (list, grow-by + count).
        assert_eq!(calls_to(&e, ARRAY_GROW), vec![vec![list, 5], vec![list, 6]]);
        // Without data the first line is left out.
        e.mem.set_u16(list + 0xa, 0);
        e.set(cinfo, HkConstraintCinfo::pData, Ptr::NULL);
        e.call(0x00c8a560, &args![cinfo, list]);
        assert_eq!(word(&e, list + 0xa), 3);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn fn_00c89720_jumps_to_vtable_slot_0x94_with_the_forwarded_words() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x26];
        slots[0x94 / 4] = 0x0090_5000;
        e.put_vtable(0x0090_2000, &slots);
        e.register(0x0090_5000, |_, a| Ret {
            eax: a.iter().sum(),
            ..Ret::default()
        });
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x0090_2000);
        assert_eq!(
            e.call(0x00c8_9720, &args![this, 5u32, 7u32]).u32(),
            this + 12
        );
    }

    #[test]
    fn load_c_info_and_register_streamables_answer_their_last_calls() {
        let mut e = Engine::new();
        let cinfo = object(&mut e, 0x0200_1200, &[(8, 0x0900_0002)]);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 4, data);
        let this =
            constraint_with_cinfo(&mut e, cinfo, &[(0xa8, 0x0900_0004), (0xd0, 0x0900_0006)]);
        quiet(&mut e, &[0x0900_0002, 0x0900_0006]);
        e.register(0x0900_0004, |_, _| 7u32.into_ret());
        assert_eq!(e.call(0x00c89810, &args![this, 0x4242u32]).i32(), 7);
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00c898e0, &args![this, 0x5000u32]).u32(), 1);
    }
}
