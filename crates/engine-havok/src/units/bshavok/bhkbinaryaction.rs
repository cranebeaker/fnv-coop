//! `BSHavok/bhkbinaryaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkBinaryAction` (Xbox PDB) is a `bhkAction` (0x10 bytes: `+8` the Havok action, `+0xC` the
//! cinfo) whose Havok object is an `hkpBinaryAction` with two entities (`+0x18` and `+0x1C`
//! of it); `+0xC` of an entity is the `bhk` object that owns it. The cinfo built by virtual
//! slot 0xB4 keeps the two bodies at `+4` and `+8`.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame
//! that is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// Per-thread word (`TLS + 0x2b4`) that the allocating functions set to 0x10 around their work.
const SCOPE_WORD: u32 = 0x2b4;
/// `bhkSerializable::Kill` (Xbox PDB) and the `bhkAction` functions this class builds on:
/// destructor, `Add`, `Remove` and `GetViewerStrings`.
const KILL: u32 = 0x00c8_76a0;
const ACTION_DESTRUCTOR: u32 = 0x00c8_7970;
const ACTION_ADD: u32 = 0x00c8_7a70;
const ACTION_REMOVE: u32 = 0x00c8_7b40;
const ACTION_GET_VIEWER_STRINGS: u32 = 0x00c8_7be0;
/// `bhkSerializable` functions: `LoadBinary`, `LinkObject`, `CopyMembers` (`this, clone,
/// process`) and `SaveBinary`... (Xbox PDB names, `00c87850` unnamed in the engine map).
const SERIALIZABLE_LOAD_BINARY: u32 = 0x00c8_7650;
const SERIALIZABLE_LINK_OBJECT: u32 = 0x00c8_7670;
const SERIALIZABLE_COPY_MEMBERS: u32 = 0x00c8_7600;
const SERIALIZABLE_SAVE_BINARY: u32 = 0x00c8_7850;
/// `NiStream` (`nistream.cpp`): `00a645b0` reads a link id (`this` = the stream, no stack
/// word) and `NiStream::GetObjectFromLinkID` (`00a63430`, `this` = the stream) reads one and
/// answers the object.
const STREAM_LOAD_LINK_ID: u32 = 0x00a6_45b0;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// Lookup in the cloning process's map (`00a53c30`, `this` = the map, `(key, out)`, answers a
/// byte).
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// The base class's equality test (`00ba87d0`, a jump to `00a5d470`; `(this, other)`).
const BASE_IS_EQUAL: u32 = 0x00ba_87d0;
/// `hkpBinaryAction::setEntityA` (`00cf4ce0`, unconfirmed name) and `setEntityB` (Xbox PDB):
/// `(action, body)`.
const HK_SET_ENTITY_A: u32 = 0x00cf_4ce0;
const HK_SET_ENTITY_B: u32 = 0x00cf_4d30;
/// `NiObject::RegisterStreamables` thunk (`00a81fe0`, a jump to `00a5d2f0`; `(this, stream)`).
const BASE_REGISTER_STREAMABLES: u32 = 0x00a8_1fe0;
/// `NiGetViewerString` and the two array helpers of the viewer-string array.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The word the class passes to `NiGetViewerString`.
const CLASS_NAME_POINTER: u32 = 0x0126_8198;
/// Class vtable and object counter.
const VTABLE_CLASS: u32 = 0x010c_5ef4;
const COUNT_CLASS: u32 = 0x0126_8194;

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

/// The cinfo: virtual slot 0xB4 (`CreateCinfo`) given a scratch out-pointer.
fn cinfo_of(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32())
}

/// `this ? [this + 8] : 0`: the Havok action.
fn havok_action(e: &Engine, this: u32) -> u32 {
    if this == 0 {
        0
    } else {
        e.mem.u32(this + 8)
    }
}

/// The entity at `offset` (0x18 or 0x1C) of the Havok action of `this`, 0 when there is none.
fn entity_of(e: &Engine, this: u32, offset: u32) -> u32 {
    let action = havok_action(e, this);
    if action == 0 {
        0
    } else {
        e.mem.u32(action + offset)
    }
}

/// `[entity + 0xC]`, 0 for no entity.
fn entity_owner(e: &Engine, entity: u32) -> u32 {
    if entity == 0 {
        0
    } else {
        e.mem.u32(entity + 0xc)
    }
}

// Translated from 00c8a720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8a720 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBinaryAction::~bhkBinaryAction` (Xbox PDB): stores the class vtable, runs `Kill`
/// (`00c876a0`), decrements the class counter and runs the `bhkAction` destructor (`00c87970`).
pub fn bhk_binary_action_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    e.call(KILL, &args![this]);
    let count: u32 = e.global(COUNT_CLASS);
    e.set_global(COUNT_CLASS, count.wrapping_sub(1));
    e.call(ACTION_DESTRUCTOR, &args![this]);
}

// Translated from 00c8a7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBinaryAction::LoadBinary` (Xbox PDB): reads two link ids from the stream (`00a645b0`
/// twice), runs `bhkSerializable::LoadBinary` (`00c87650`) and clears the two bodies of the
/// cinfo (virtual slot 0xB4) at `+4` and `+8`.
pub fn bhk_binary_action_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(STREAM_LOAD_LINK_ID, &args![stream]);
    e.call(STREAM_LOAD_LINK_ID, &args![stream]);
    e.call(SERIALIZABLE_LOAD_BINARY, &args![this, stream]);
    let cinfo = cinfo_of(e, this);
    e.mem.set_u32(cinfo + 4, 0);
    e.mem.set_u32(cinfo + 8, 0);
}

// Translated from 00c8a850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBinaryAction::CopyMembers` (Xbox PDB): takes the cinfo (virtual slot 0xB4), the `bhk`
/// owners of its two bodies (`+4`, `+8`, through `+0xC`), clears both, and looks each owner
/// up in the cloning process's map (the map is the first word of `process`; `00a53c30`).
/// A found first owner stores `[found + 8]` at `+4`; the second owner, when found, or else
/// the result of its virtual slot 0x48 (clone, given `process`), stores `[object + 8]` at
/// `+8` (0 for a null object). The lookups' out word starts as `process`. It ends with
/// `bhkSerializable::CopyMembers` (`00c87600`) on `(this, clone, process)`.
pub fn bhk_binary_action_copy_members(e: &mut Engine, this: Ptr, clone: Ptr, process: Ptr) {
    let cinfo = cinfo_of(e, this);
    let first = e.mem.u32(cinfo + 4);
    let owner_a = entity_owner(e, first);
    let second = e.mem.u32(cinfo + 8);
    let owner_b = entity_owner(e, second);
    e.mem.set_u32(cinfo + 4, 0);
    e.mem.set_u32(cinfo + 8, 0);
    let map = e.mem.u32(process.addr());
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), process.addr());
        if e.call(CLONE_MAP_LOOKUP, &args![map, owner_a, slot]).bool() {
            let found = e.mem.u32(slot.addr());
            let body = if found == 0 { 0 } else { e.mem.u32(found + 8) };
            e.mem.set_u32(cinfo + 4, body);
        }
        if owner_b != 0 {
            let object = if e.call(CLONE_MAP_LOOKUP, &args![map, owner_b, slot]).bool() {
                e.mem.u32(slot.addr())
            } else {
                e.vcall(owner_b, 0x48, &args![process]).u32()
            };
            let body = if object == 0 {
                0
            } else {
                e.mem.u32(object + 8)
            };
            e.mem.set_u32(cinfo + 8, body);
        }
    });
    e.call(SERIALIZABLE_COPY_MEMBERS, &args![this, clone, process]);
}

// Translated from 00c8a910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBinaryAction::LinkObject` (Xbox PDB): reads two objects from the stream's link ids
/// (`00a63430`); when the first exists, stores its `+8` at `+4` of the cinfo (virtual slot
/// 0xB4) and the second's `+8` (0 when there is none) at `+8`; then runs
/// `bhkSerializable::LinkObject` (`00c87670`).
pub fn bhk_binary_action_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let first = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let second = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let cinfo = cinfo_of(e, this);
    if first != 0 {
        let body = e.mem.u32(first + 8);
        e.mem.set_u32(cinfo + 4, body);
        let other = if second == 0 {
            0
        } else {
            e.mem.u32(second + 8)
        };
        e.mem.set_u32(cinfo + 8, other);
    }
    e.call(SERIALIZABLE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00c8a980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBinaryAction::IsEqual` (Xbox PDB): the base test (`00ba87d0`) and the two entities
/// (`+0x18`, `+0x1C` of the Havok action, 0 when `this` or the action is 0) equal to those
/// of `other`.
pub fn bhk_binary_action_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let same_a = entity_of(e, this.addr(), 0x18) == entity_of(e, other.addr(), 0x18);
    let same_b = entity_of(e, this.addr(), 0x1c) == entity_of(e, other.addr(), 0x1c);
    same_a && same_b
}

// Translated from 00c8aa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `bhk` owner (`+0xC`) of entity A (`+0x18` of the Havok action) when the low
/// byte of `which` is `'A'` or `'a'`, else of entity B (`+0x1C`); 0 when there is none.
pub fn fn_00c8aa00(e: &mut Engine, this: Ptr, which: u32) -> u32 {
    let offset = if which as u8 == b'A' || which as u8 == b'a' {
        0x18
    } else {
        0x1c
    };
    let entity = entity_of(e, this.addr(), offset);
    entity_owner(e, entity)
}

// Translated from 00c8aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBinaryAction::SetRigidBodyA` (Xbox PDB): when `this` and its Havok action exist, sets
/// entity A of the action to the Havok object (`+8`) of `body` (`00cf4ce0`).
pub fn bhk_binary_action_set_rigid_body_a(e: &mut Engine, this: Ptr, body: Ptr) {
    let action = havok_action(e, this.addr());
    if action != 0 {
        let hk_body = e.mem.u32(body.addr() + 8);
        e.call(HK_SET_ENTITY_A, &args![action, hk_body]);
    }
}

// Translated from 00c8aa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBinaryAction::SetRigidBodyB` (Xbox PDB): when `this` and its Havok action exist, sets
/// entity B of the action to the Havok object (`+8`) of `body` (`hkpBinaryAction::setEntityB`,
/// `00cf4d30`).
pub fn bhk_binary_action_set_rigid_body_b(e: &mut Engine, this: Ptr, body: Ptr) {
    let action = havok_action(e, this.addr());
    if action != 0 {
        let hk_body = e.mem.u32(body.addr() + 8);
        e.call(HK_SET_ENTITY_B, &args![action, hk_body]);
    }
}

// Translated from 00c8aa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBinaryAction::RegisterStreamables` (Xbox PDB): when entity B (`+0x1C` of the Havok
/// action) has an owner (`+0xC`), runs that owner's virtual slot 0x54 on the stream; then
/// answers the base class thunk `00a81fe0` on `(this, stream)`.
pub fn bhk_binary_action_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> u32 {
    let entity = entity_of(e, this.addr(), 0x1c);
    let owner = entity_owner(e, entity);
    if owner != 0 {
        e.vcall(owner, 0x54, &args![stream]);
    }
    e.call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .u32()
}

// Translated from 00c8aac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBinaryAction::SaveBinary` (Xbox PDB): writes the owners (`+0xC`) of entity A and
/// entity B (`+0x18`, `+0x1C` of the Havok action; 0 for a missing one) through the stream's
/// virtual slot 0x30, then runs `00c87850` on `(this, stream)`.
pub fn bhk_binary_action_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    for offset in [0x18u32, 0x1c] {
        let entity = entity_of(e, this.addr(), offset);
        let owner = entity_owner(e, entity);
        e.vcall(stream.addr(), 0x30, &args![owner]);
    }
    e.call(SERIALIZABLE_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00c8ab30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8ab30 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBinaryAction::Add` (Xbox PDB), with the allocation scope word set to 0x10: false
/// when `this` or its Havok action is 0. Without a `world` it runs virtual slot 0xA0
/// (`Remove`) and answers false. With one, entity B's owner (`+0xC`), when there is one,
/// is asked through its virtual slot 0x9C with the world and a false answer ends it;
/// otherwise the answer is that of `bhkAction::Add` (`00c87a70`, `(this, world)`).
pub fn bhk_binary_action_add(e: &mut Engine, this: Ptr, world: Ptr) -> bool {
    let (word, saved) = enter_scope(e);
    let mut added = false;
    if havok_action(e, this.addr()) != 0 {
        if world.addr() == 0 {
            e.vcall(this.addr(), 0xa0, &args![]);
        } else {
            let entity = entity_of(e, this.addr(), 0x1c);
            let owner = entity_owner(e, entity);
            let accepted = owner == 0 || e.vcall(owner, 0x9c, &args![world]).bool();
            if accepted {
                added = e.call(ACTION_ADD, &args![this, world]).bool();
            }
        }
    }
    e.mem.set_u32(word, saved);
    added
}

// Translated from 00c8ac00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8ac00 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBinaryAction::Remove` (Xbox PDB), with the allocation scope word set to 0x10: takes
/// the `bhkWorld` at `+0x350` of the Havok world from virtual slot 0x94; with none it answers
/// false. Otherwise runs `bhkAction::Remove` (`00c87b40`), and when entity B (`+0x1C` of the
/// Havok action) equals the word at `+0x24` of the object the world's virtual slot 0x94
/// answers (0 for none) and the Havok action exists, sets entity B to 0 (`00cf4d30`).
/// Answers the byte `bhkAction::Remove` answered.
pub fn bhk_binary_action_remove(e: &mut Engine, this: Ptr) -> bool {
    let (word, saved) = enter_scope(e);
    let hk_world = e.vcall(this.addr(), 0x94, &args![]).u32();
    let world = if hk_world == 0 {
        0
    } else {
        e.mem.u32(hk_world + 0x350)
    };
    let mut removed = false;
    if world != 0 {
        removed = e.call(ACTION_REMOVE, &args![this]).bool();
        let entity = entity_of(e, this.addr(), 0x1c);
        let fixed_world = e.vcall(world, 0x94, &args![]).u32();
        let fixed_entity = if fixed_world == 0 {
            0
        } else {
            e.mem.u32(fixed_world + 0x24)
        };
        if entity == fixed_entity {
            let action = havok_action(e, this.addr());
            if action != 0 {
                e.call(HK_SET_ENTITY_B, &args![action, 0u32]);
            }
        }
    }
    e.mem.set_u32(word, saved);
    removed
}

// Translated from 00c8ad40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8ad40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBinaryAction::GetViewerStrings` (Xbox PDB), with the allocation scope word set to
/// 0x10: runs `bhkAction::GetViewerStrings` (`00c87be0`) on `(this, strings)`, then appends
/// the string `NiGetViewerString` makes of the word at `01268198` to the array (grown by the
/// step at `+0xE` plus the size when the 16-bit size at `+0xA` has reached the capacity at
/// `+0x8`).
pub fn bhk_binary_action_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let (word, saved) = enter_scope(e);
    e.call(ACTION_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    let capacity = u32::from(e.mem.u16(strings.addr() + 8));
    if size >= capacity {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c8a720, bhk_binary_action_destructor(Ptr)),
        entry!(0x00c8a7a0, bhk_binary_action_load_binary(Ptr, Ptr)),
        entry!(0x00c8a850, bhk_binary_action_copy_members(Ptr, Ptr, Ptr)),
        entry!(0x00c8a910, bhk_binary_action_link_object(Ptr, Ptr)),
        entry!(0x00c8a980, bhk_binary_action_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c8aa00, fn_00c8aa00(Ptr, u32) -> u32),
        entry!(0x00c8aa40, bhk_binary_action_set_rigid_body_a(Ptr, Ptr)),
        entry!(0x00c8aa60, bhk_binary_action_set_rigid_body_b(Ptr, Ptr)),
        entry!(0x00c8aa80, bhk_binary_action_register_streamables(Ptr, Ptr) -> u32),
        entry!(0x00c8aac0, bhk_binary_action_save_binary(Ptr, Ptr)),
        entry!(0x00c8ab30, bhk_binary_action_add(Ptr, Ptr) -> bool),
        entry!(0x00c8ac00, bhk_binary_action_remove(Ptr) -> bool),
        entry!(0x00c8ad40, bhk_binary_action_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An object with a fresh vtable holding the given `(byte offset, target)` slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        object
    }

    /// A binary action (slot 0xB4 answers `cinfo`) with a Havok action at `+8` whose entities
    /// A and B are owned by `owner_a` and `owner_b` (0 for none). Returns the action object
    /// and the Havok action.
    fn action(
        e: &mut Engine,
        slots: &[(u32, u32)],
        owner_a: u32,
        owner_b: u32,
    ) -> (u32, u32, u32, u32) {
        let this = object(e, slots);
        let hk = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, hk);
        let entity_a = e.mem.alloc(0x20);
        let entity_b = e.mem.alloc(0x20);
        e.mem.set_u32(entity_a + 0xc, owner_a);
        e.mem.set_u32(entity_b + 0xc, owner_b);
        e.mem.set_u32(hk + 0x18, entity_a);
        e.mem.set_u32(hk + 0x1c, entity_b);
        (this, hk, entity_a, entity_b)
    }

    #[test]
    fn destructor_stores_the_vtable_kills_and_destroys_the_base() {
        let mut e = Engine::new();
        e.map(COUNT_CLASS, 4);
        e.set_global(COUNT_CLASS, 3u32);
        e.register(KILL, nothing);
        e.register(ACTION_DESTRUCTOR, nothing);
        let this = object(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00c8a720, &args![this]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x00c8a720, KILL, ACTION_DESTRUCTOR]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 2);
    }

    #[test]
    fn load_binary_reads_two_link_ids_then_clears_the_cinfo_bodies() {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 4, 7);
        e.mem.set_u32(cinfo + 8, 8);
        e.register_double(0x0900_00b4, move |_, _| Ret {
            eax: cinfo,
            ..Ret::default()
        });
        e.register(STREAM_LOAD_LINK_ID, nothing);
        e.register(SERIALIZABLE_LOAD_BINARY, nothing);
        let this = object(&mut e, &[(0xb4, 0x0900_00b4)]);
        e.call_log = Some(vec![]);
        e.call(0x00c8a7a0, &args![this, 0x5000u32]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00c8a7a0,
                STREAM_LOAD_LINK_ID,
                STREAM_LOAD_LINK_ID,
                SERIALIZABLE_LOAD_BINARY,
                0x0900_00b4
            ]
        );
        assert_eq!(calls(&e, STREAM_LOAD_LINK_ID), [[0x5000], [0x5000]]);
        assert_eq!(calls(&e, SERIALIZABLE_LOAD_BINARY), [[this, 0x5000]]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(e.mem.u32(cinfo + 8), 0);
    }

    #[test]
    fn copy_members_maps_the_bodies_to_their_clones() {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x20);
        e.register_double(0x0900_00b4, move |_, _| Ret {
            eax: cinfo,
            ..Ret::default()
        });
        // Owners A and B each own a Havok body; A is in the map, B is cloned on demand.
        let hk_a = e.mem.alloc(0x20);
        let hk_b = e.mem.alloc(0x20);
        let owner_a = object(&mut e, &[]);
        let owner_b = object(&mut e, &[(0x48, 0x0900_0048)]);
        e.mem.set_u32(hk_a + 0xc, owner_a);
        e.mem.set_u32(hk_b + 0xc, owner_b);
        e.mem.set_u32(cinfo + 4, hk_a);
        e.mem.set_u32(cinfo + 8, hk_b);
        let clone_a = e.mem.alloc(0x20);
        let clone_a_body = e.mem.alloc(0x20);
        e.mem.set_u32(clone_a + 8, clone_a_body);
        let clone_b = e.mem.alloc(0x20);
        let clone_b_body = e.mem.alloc(0x20);
        e.mem.set_u32(clone_b + 8, clone_b_body);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            if a[1] == owner_a {
                e.mem.set_u32(a[2], clone_a);
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            } else {
                Ret::default()
            }
        });
        e.register_double(0x0900_0048, move |_, _| Ret {
            eax: clone_b,
            ..Ret::default()
        });
        e.register(SERIALIZABLE_COPY_MEMBERS, nothing);
        let this = object(&mut e, &[(0xb4, 0x0900_00b4)]);
        let process = e.mem.alloc(0x10);
        e.mem.set_u32(process, 0x4242);
        e.call_log = Some(vec![]);
        e.call(0x00c8a850, &args![this, 0x6000u32, process]);
        assert_eq!(e.mem.u32(cinfo + 4), clone_a_body);
        assert_eq!(e.mem.u32(cinfo + 8), clone_b_body);
        assert_eq!(
            calls(&e, CLONE_MAP_LOOKUP)
                .iter()
                .map(|c| (c[0], c[1]))
                .collect::<Vec<_>>(),
            [(0x4242, owner_a), (0x4242, owner_b)]
        );
        assert_eq!(calls(&e, 0x0900_0048), [[owner_b, process]]);
        assert_eq!(
            calls(&e, SERIALIZABLE_COPY_MEMBERS),
            [[this, 0x6000, process]]
        );
    }

    #[test]
    fn copy_members_without_a_second_body_clears_both_and_skips_the_second_lookup() {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x20);
        e.register_double(0x0900_00b4, move |_, _| Ret {
            eax: cinfo,
            ..Ret::default()
        });
        e.register(CLONE_MAP_LOOKUP, nothing);
        e.register(SERIALIZABLE_COPY_MEMBERS, nothing);
        let this = object(&mut e, &[(0xb4, 0x0900_00b4)]);
        let process = e.mem.alloc(0x10);
        e.mem.set_u32(cinfo + 4, 1);
        e.mem.set_u32(cinfo + 8, 2);
        e.call_log = Some(vec![]);
        // Both bodies are 0-owned: the first lookup runs with key 0, the second is skipped.
        let hk = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 4, hk);
        e.mem.set_u32(cinfo + 8, 0);
        e.call(0x00c8a850, &args![this, 0x6000u32, process]);
        assert_eq!(calls(&e, CLONE_MAP_LOOKUP).len(), 1);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(e.mem.u32(cinfo + 8), 0);
    }

    #[test]
    fn link_object_stores_the_linked_bodies_in_the_cinfo() {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x20);
        e.register_double(0x0900_00b4, move |_, _| Ret {
            eax: cinfo,
            ..Ret::default()
        });
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first + 8, 0x1111);
        e.mem.set_u32(second + 8, 0x2222);
        let ids = std::rc::Rc::new(std::cell::RefCell::new(vec![second, first]));
        let queue = ids.clone();
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| Ret {
            eax: queue.borrow_mut().pop().unwrap(),
            ..Ret::default()
        });
        e.register(SERIALIZABLE_LINK_OBJECT, nothing);
        let this = object(&mut e, &[(0xb4, 0x0900_00b4)]);
        e.call_log = Some(vec![]);
        e.call(0x00c8a910, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x1111);
        assert_eq!(e.mem.u32(cinfo + 8), 0x2222);
        assert_eq!(calls(&e, SERIALIZABLE_LINK_OBJECT), [[this, 0x5000]]);
        // A missing first object leaves the cinfo alone.
        *ids.borrow_mut() = vec![second, 0];
        e.mem.set_u32(cinfo + 4, 5);
        e.call(0x00c8a910, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 5);
        assert_eq!(calls(&e, SERIALIZABLE_LINK_OBJECT).len(), 2);
        // A missing second object stores 0.
        *ids.borrow_mut() = vec![0, first];
        e.call(0x00c8a910, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x1111);
        assert_eq!(e.mem.u32(cinfo + 8), 0);
    }

    #[test]
    fn is_equal_needs_the_base_and_both_entities_to_match() {
        let mut e = Engine::new();
        e.register(BASE_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let (this, hk, entity_a, entity_b) = action(&mut e, &[], 1, 2);
        let (other, other_hk, _, _) = action(&mut e, &[], 1, 2);
        e.mem.set_u32(other_hk + 0x18, entity_a);
        e.mem.set_u32(other_hk + 0x1c, entity_b);
        assert_eq!(e.call(0x00c8a980, &args![this, other]).u32(), 1);
        e.mem.set_u32(other_hk + 0x1c, 0x1234);
        assert_eq!(e.call(0x00c8a980, &args![this, other]).u32(), 0);
        e.mem.set_u32(other_hk + 0x1c, entity_b);
        e.mem.set_u32(other_hk + 0x18, 0x1234);
        assert_eq!(e.call(0x00c8a980, &args![this, other]).u32(), 0);
        // Missing Havok actions on both sides compare equal (all entities 0).
        let bare = object(&mut e, &[]);
        let bare_other = object(&mut e, &[]);
        assert_eq!(e.call(0x00c8a980, &args![bare, bare_other]).u32(), 1);
        e.mem.set_u32(hk + 0x18, 0);
        e.mem.set_u32(hk + 0x1c, 0);
        assert_eq!(e.call(0x00c8a980, &args![this, bare_other]).u32(), 1);
        // A failing base test answers false without looking further.
        e.register(BASE_IS_EQUAL, |_, _| Ret::default());
        assert_eq!(e.call(0x00c8a980, &args![bare, bare_other]).u32(), 0);
    }

    #[test]
    fn fn_00c8aa00_answers_the_owner_of_entity_a_or_b() {
        let mut e = Engine::new();
        let (this, _, _, _) = action(&mut e, &[], 0xa0, 0xb0);
        assert_eq!(e.call(0x00c8aa00, &args![this, b'A' as u32]).u32(), 0xa0);
        assert_eq!(e.call(0x00c8aa00, &args![this, 0x7761u32]).u32(), 0xa0);
        assert_eq!(e.call(0x00c8aa00, &args![this, b'B' as u32]).u32(), 0xb0);
        assert_eq!(e.call(0x00c8aa00, &args![this, 0u32]).u32(), 0xb0);
        let bare = object(&mut e, &[]);
        assert_eq!(e.call(0x00c8aa00, &args![bare, b'A' as u32]).u32(), 0);
        assert_eq!(e.call(0x00c8aa00, &args![0u32, b'B' as u32]).u32(), 0);
    }

    #[test]
    fn set_rigid_body_a_and_b_pass_the_havok_body_to_the_action() {
        let mut e = Engine::new();
        e.register(HK_SET_ENTITY_A, nothing);
        e.register(HK_SET_ENTITY_B, nothing);
        let (this, hk, _, _) = action(&mut e, &[], 0, 0);
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body + 8, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x00c8aa40, &args![this, body]);
        e.call(0x00c8aa60, &args![this, body]);
        assert_eq!(calls(&e, HK_SET_ENTITY_A), [[hk, 0x7777]]);
        assert_eq!(calls(&e, HK_SET_ENTITY_B), [[hk, 0x7777]]);
        // Neither runs without a Havok action.
        let bare = object(&mut e, &[]);
        e.call(0x00c8aa40, &args![bare, body]);
        e.call(0x00c8aa60, &args![0u32, body]);
        assert_eq!(calls(&e, HK_SET_ENTITY_A).len(), 1);
        assert_eq!(calls(&e, HK_SET_ENTITY_B).len(), 1);
    }

    #[test]
    fn register_streamables_asks_the_owner_of_entity_b_first() {
        let mut e = Engine::new();
        e.register(0x0900_0054, nothing);
        e.register(BASE_REGISTER_STREAMABLES, |_, _| Ret {
            eax: 0x99,
            ..Ret::default()
        });
        let owner = object(&mut e, &[(0x54, 0x0900_0054)]);
        let (this, _, _, _) = action(&mut e, &[], 0, owner);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8aa80, &args![this, 0x5000u32]).u32(), 0x99);
        assert_eq!(calls(&e, 0x0900_0054), [[owner, 0x5000]]);
        let (without, _, _, _) = action(&mut e, &[], 0, 0);
        assert_eq!(e.call(0x00c8aa80, &args![without, 0x5000u32]).u32(), 0x99);
        assert_eq!(calls(&e, 0x0900_0054).len(), 1);
        assert_eq!(calls(&e, BASE_REGISTER_STREAMABLES).len(), 2);
    }

    #[test]
    fn save_binary_writes_both_owners_then_the_base() {
        let mut e = Engine::new();
        e.register(0x0900_0030, nothing);
        e.register(SERIALIZABLE_SAVE_BINARY, nothing);
        let stream = object(&mut e, &[(0x30, 0x0900_0030)]);
        let (this, _, _, _) = action(&mut e, &[], 0xa0, 0xb0);
        e.call_log = Some(vec![]);
        e.call(0x00c8aac0, &args![this, stream]);
        assert_eq!(calls(&e, 0x0900_0030), [[stream, 0xa0], [stream, 0xb0]]);
        assert_eq!(calls(&e, SERIALIZABLE_SAVE_BINARY), [[this, stream]]);
        let bare = object(&mut e, &[]);
        e.call(0x00c8aac0, &args![bare, stream]);
        assert_eq!(calls(&e, 0x0900_0030).len(), 4);
        assert_eq!(calls(&e, 0x0900_0030)[3], [stream, 0]);
    }

    #[test]
    fn add_asks_the_owner_of_entity_b_before_adding() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        e.register(ACTION_ADD, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(0x0900_00a0, nothing);
        e.register(0x0900_009c, |_, _| Ret {
            eax: 0,
            ..Ret::default()
        });
        let owner = object(&mut e, &[(0x9c, 0x0900_009c)]);
        let (this, _, _, _) = action(&mut e, &[(0xa0, 0x0900_00a0)], 0, owner);
        let world = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        // The owner refuses.
        assert_eq!(e.call(0x00c8ab30, &args![this, world]).u32(), 0);
        assert_eq!(calls(&e, 0x0900_009c), [[owner, world]]);
        assert!(calls(&e, ACTION_ADD).is_empty());
        // The owner accepts.
        e.register(0x0900_009c, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00c8ab30, &args![this, world]).u32(), 1);
        assert_eq!(calls(&e, ACTION_ADD), [[this, world]]);
        // No owner: added without asking.
        let (free, _, _, _) = action(&mut e, &[], 0, 0);
        assert_eq!(e.call(0x00c8ab30, &args![free, world]).u32(), 1);
        // No world: Remove only.
        assert_eq!(e.call(0x00c8ab30, &args![this, 0u32]).u32(), 0);
        assert_eq!(calls(&e, 0x0900_00a0), [[this]]);
        // No Havok action.
        let bare = object(&mut e, &[]);
        assert_eq!(e.call(0x00c8ab30, &args![bare, world]).u32(), 0);
        assert_eq!(e.mem.u32(word), 0x55);
    }

    #[test]
    fn remove_clears_entity_b_when_it_is_the_fixed_one() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        e.register(ACTION_REMOVE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(HK_SET_ENTITY_B, nothing);
        // The Havok world's bhkWorld answers (slot 0x94) another Havok object whose +0x24 is
        // the fixed entity.
        let fixed_holder = e.mem.alloc(0x40);
        e.register_double(0x0900_0194, move |_, _| Ret {
            eax: fixed_holder,
            ..Ret::default()
        });
        let bhk_world = object(&mut e, &[(0x94, 0x0900_0194)]);
        let hk_world = e.mem.alloc(0x400);
        e.mem.set_u32(hk_world + 0x350, bhk_world);
        e.register_double(0x0900_0094, move |_, _| Ret {
            eax: hk_world,
            ..Ret::default()
        });
        let (this, hk, _, entity_b) = action(&mut e, &[(0x94, 0x0900_0094)], 0, 0);
        e.call_log = Some(vec![]);
        // Not the fixed entity: only bhkAction::Remove's answer.
        e.mem.set_u32(fixed_holder + 0x24, 0x1234);
        assert_eq!(e.call(0x00c8ac00, &args![this]).u32(), 1);
        assert!(calls(&e, HK_SET_ENTITY_B).is_empty());
        // The fixed entity: entity B is cleared.
        e.mem.set_u32(fixed_holder + 0x24, entity_b);
        assert_eq!(e.call(0x00c8ac00, &args![this]).u32(), 1);
        assert_eq!(calls(&e, HK_SET_ENTITY_B), [[hk, 0]]);
        // No bhkWorld: false and nothing removed.
        e.mem.set_u32(hk_world + 0x350, 0);
        assert_eq!(e.call(0x00c8ac00, &args![this]).u32(), 0);
        assert_eq!(calls(&e, ACTION_REMOVE).len(), 2);
        assert_eq!(e.mem.u32(word), 0x55);
    }

    #[test]
    fn get_viewer_strings_grows_a_full_array_before_storing() {
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(CLASS_NAME_POINTER, 4);
            e.set_global(CLASS_NAME_POINTER, 0x5151_0001u32);
            let word = e.tls() + SCOPE_WORD;
            e.mem.set_u32(word, 0x66);
            e.register(ACTION_GET_VIEWER_STRINGS, nothing);
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(STRINGS_GROW, nothing);
            e.register(STRINGS_SET_AT, nothing);
            let this = object(&mut e, &[]);
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.call_log = Some(vec![]);
            e.call(0x00c8ad40, &args![this, strings]);
            assert_eq!(calls(&e, ACTION_GET_VIEWER_STRINGS), [[this, strings]]);
            assert_eq!(calls(&e, NI_GET_VIEWER_STRING), [[0x5151_0001]]);
            let grown = calls(&e, STRINGS_GROW);
            if size >= capacity {
                assert_eq!(grown, [[strings, 5 + u32::from(size)]]);
            } else {
                assert!(grown.is_empty());
            }
            assert_eq!(calls(&e, STRINGS_SET_AT).len(), 1);
            assert_eq!(e.mem.u32(word), 0x66);
        }
    }
}
