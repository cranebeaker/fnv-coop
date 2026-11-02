//! `fallout/ai/escortpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `EscortActorPackageData` (an `ActorPackageData`; vtable `01093188`, RTTI
//! name `.?AVEscortActorPackageData@@`) keeps the actors an escort package
//! leads in a `BSSimpleList<Actor *>` at +0x10. The list is walked node by
//! node: `006815c0` returns the node itself (its item is the first word),
//! `00726070` the next node. Functions that open a C++ exception frame
//! (`FS:[0]`) carry a `Translation incomplete` marker: the unwinding states
//! are not translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// Deallocator (cdecl `(block)`).
const FREE: u32 = 0x0040_1030;
/// `EscortActorPackageData` vtable (RTTI name confirmed).
const ESCORT_ACTOR_PACKAGE_DATA_VTABLE: u32 = 0x0109_3188;
/// The list's destructor steps `(list)` and the `ActorPackageData`
/// destructor body `(this)` (map: `packagedatauseweapon.cpp`).
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_DESTROY: u32 = 0x0046_ffb0;
const ACTOR_PACKAGE_DATA_DESTROY: u32 = 0x0067_d970;
/// List operations `(list, &item)`: contains, add, remove.
const LIST_CONTAINS: u32 = 0x005f_65d0;
const LIST_ADD: u32 = 0x005a_e3d0;
const LIST_REMOVE: u32 = 0x0090_5330;
/// Node walk: the node itself (its item word first), the next node.
const LIST_NODE: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
/// `(list)`: whether the list is empty (both words zero).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `(node, &item)`: stores the item in the node.
const LIST_NODE_SET_ITEM: u32 = 0x0072_6c60;
/// `(node)`: removes the head node's item (the next one moves up).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `(actor)`: a flag of the actor (map: `modelloader.cpp`); true drops it.
const ACTOR_FLAG: u32 = 0x0044_0d80;
/// `Actor::CouldBeFollowing` (Xbox PDB) `(this, actor)`.
const ACTOR_COULD_BE_FOLLOWING: u32 = 0x008b_c860;
/// `(reference, form)`: whether the reference matches the form.
const REFERENCE_MATCHES_FORM: u32 = 0x0055_d3b0;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (Xbox PDB) `(this, form, 0)`.
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (Xbox PDB) `(this)`.
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (Xbox PDB) `(this, value, start)`.
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
/// `BGSLoadGameBuffer::LoadFormID` (Xbox PDB) `(this)`: a form id.
const LOAD_FORM_ID: u32 = 0x0086_48a0;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (Xbox PDB) `(this, &word)`.
const LOAD_FORM_ID_INTO: u32 = 0x0086_48e0;
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (Xbox PDB) `(this)`.
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;
/// cdecl `(form id)`: the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` `(object, 0, source type, target type, 0)`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `TESForm`, `TESPackage`, `Actor` (names confirmed).
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TES_PACKAGE_TYPE: u32 = 0x0118_46a0;
const ACTOR_TYPE: u32 = 0x0118_46d4;
/// `TESPackage::GetEscortPackageData` (Xbox PDB) `(this)`.
const GET_ESCORT_PACKAGE_DATA: u32 = 0x0067_59b0;

layout! {
    /// `EscortActorPackageData` (Xbox PDB), 0x18 bytes; the PC code uses
    /// the same offsets.
    pub struct EscortActorPackageData: 0x18 {
        /// Virtual function table pointer.
        0x00 vfptr: u32,
        /// `pActor` (Xbox PDB): `Actor*`.
        0x04 pActor: Ptr,
        /// `pPackage` (Xbox PDB): `TESPackage*`.
        0x08 pPackage: Ptr,
        /// `pPackageData` (Xbox PDB): `const TESEscortPackageData*`.
        0x0C pPackageData: Ptr,
        /// `EscortedActors` (Xbox PDB): `BSSimpleList<Actor *>`, first item.
        0x10 EscortedActorsItem: u32,
        /// The list head's next node.
        0x14 EscortedActorsNext: u32,
    }
}

fn escorted_actors(this: Ptr<EscortActorPackageData>) -> u32 {
    this.addr() + 0x10
}

/// The item of list node `node` (`006815c0` then the first word).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(LIST_NODE, &args![node]).u32();
    e.mem.u32(node)
}

// Translated from 009f0980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EscortActorPackageData::__scalar_deleting_destructor` (Xbox PDB): runs
/// the destructor `009f09b0`, frees `this` when bit 0 of `flags` is set,
/// returns `this`.
pub fn escort_actor_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<EscortActorPackageData>,
    flags: u32,
) -> Ptr<EscortActorPackageData> {
    fn_009f09b0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 009f09b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f09b0 - C++ exception frame (FS:[0] unwinding states) not translated
/// The `EscortActorPackageData` destructor: sets its vtable, empties and
/// destroys the escorted actors list, then runs the `ActorPackageData`
/// destructor body (`0067d970`).
pub fn fn_009f09b0(e: &mut Engine, this: Ptr<EscortActorPackageData>) {
    e.set(
        this,
        EscortActorPackageData::vfptr,
        ESCORT_ACTOR_PACKAGE_DATA_VTABLE,
    );
    e.call(LIST_CLEAR, &args![escorted_actors(this)]);
    e.call(LIST_DESTROY, &args![escorted_actors(this)]);
    e.call(ACTOR_PACKAGE_DATA_DESTROY, &args![this]);
}

// Translated from 009f0a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `actor` to the escorted actors unless it is already there (the
/// list functions get the address of the argument word).
pub fn fn_009f0a20(e: &mut Engine, this: Ptr<EscortActorPackageData>, actor: Ptr) {
    let list = escorted_actors(this);
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), actor.addr());
        if !e.call(LIST_CONTAINS, &args![list, word]).bool() {
            e.call(LIST_ADD, &args![list, word]);
        }
    });
}

// Translated from 009f0a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `actor` from the escorted actors.
pub fn fn_009f0a60(e: &mut Engine, this: Ptr<EscortActorPackageData>, actor: Ptr) {
    let list = escorted_actors(this);
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), actor.addr());
        e.call(LIST_REMOVE, &args![list, word]);
    });
}

// Translated from 009f0a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EscortActorPackageData::GetNumEscortedActorsOfType` (Xbox PDB): first
/// drops every escorted actor whose `00440d80` flag is set or that could not
/// be following `leader` (`Actor::CouldBeFollowing`); after a removal the
/// walk restarts at the head unless the list is now empty (then it goes on
/// from the node it had read next). Then counts the escorted actors that
/// match `form` (`0055d3b0`).
pub fn escort_actor_package_data_get_num_escorted_actors_of_type(
    e: &mut Engine,
    this: Ptr<EscortActorPackageData>,
    leader: Ptr,
    form: Ptr,
) -> i32 {
    let list = escorted_actors(this);
    let mut node = list;
    while node != 0 {
        let actor = node_item(e, node);
        node = e.call(LIST_NEXT, &args![node]).u32();
        if actor == 0 {
            continue;
        }
        let drop = e.call(ACTOR_FLAG, &args![actor]).bool()
            || !e
                .call(ACTOR_COULD_BE_FOLLOWING, &args![actor, leader])
                .bool();
        if drop {
            e.with_stack(4, |e, word| {
                e.mem.set_u32(word.addr(), actor);
                e.call(LIST_REMOVE, &args![list, word]);
            });
            if !e.call(LIST_IS_EMPTY, &args![list]).bool() {
                node = list;
            }
        }
    }

    let mut count = 0i32;
    let mut node = list;
    while node != 0 {
        let actor = node_item(e, node);
        if actor != 0 && e.call(REFERENCE_MATCHES_FORM, &args![actor, form]).bool() {
            count += 1;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    count
}

// Translated from 009f0b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EscortActorPackageData::HandleDeletedReference` (Xbox PDB): a deleted
/// reference that answers slot 0x100 (an actor) is removed from the
/// escorted actors.
pub fn escort_actor_package_data_handle_deleted_reference(
    e: &mut Engine,
    this: Ptr<EscortActorPackageData>,
    reference: Ptr,
) {
    if e.vcall(reference.addr(), 0x100, &args![]).bool() {
        fn_009f0a60(e, this, reference);
    }
}

// Translated from 009f0bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EscortActorPackageData::SaveGame` (Xbox PDB): the package's form id,
/// then the escorted actors' form ids as a variable-sized count followed by
/// the ids (null items skipped).
pub fn escort_actor_package_data_save_game(
    e: &mut Engine,
    this: Ptr<EscortActorPackageData>,
    buffer: Ptr,
) {
    let package = e.get(this, EscortActorPackageData::pPackage);
    e.call(SAVE_FORM_ID, &args![buffer, package, 0u32]);
    let mut count = 0u32;
    let start = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = escorted_actors(this);
    while node != 0 {
        let actor = node_item(e, node);
        if actor != 0 {
            e.call(SAVE_FORM_ID, &args![buffer, actor, 0u32]);
            count = count.wrapping_add(1);
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count, start]);
}

// Translated from 009f0c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EscortActorPackageData::LoadGame` (Xbox PDB): reads the package's form
/// id, looks the form up and casts it to `TESPackage` (`pPackage`), takes
/// its escort package data (null without a package), then reads the
/// escorted actors' form ids into the list (resolved later by
/// `InitLoadGame`).
pub fn escort_actor_package_data_load_game(
    e: &mut Engine,
    this: Ptr<EscortActorPackageData>,
    buffer: Ptr,
) {
    let id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let package = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TES_FORM_TYPE, TES_PACKAGE_TYPE, 0u32],
        )
        .u32();
    e.set(this, EscortActorPackageData::pPackage, Ptr::new(package));
    let data = if package == 0 {
        0
    } else {
        e.call(GET_ESCORT_PACKAGE_DATA, &args![package]).u32()
    };
    e.set(this, EscortActorPackageData::pPackageData, Ptr::new(data));
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let list = escorted_actors(this);
    for _ in 0..count {
        e.with_stack(4, |e, word| {
            e.call(LOAD_FORM_ID_INTO, &args![buffer, word]);
            e.call(LIST_ADD, &args![list, word]);
        });
    }
}

// Translated from 009f0cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EscortActorPackageData::InitLoadGame` (Xbox PDB): turns each loaded form
/// id in the list into its `Actor` (lookup and checked cast); an item that
/// is null or not an actor is removed (the head through `0063f7b0`, a later
/// node through `00905330` on the previous node). Then `pActor` is
/// `owner`'s slot 0x0C.
pub fn escort_actor_package_data_init_load_game(
    e: &mut Engine,
    this: Ptr<EscortActorPackageData>,
    owner: Ptr,
) {
    let mut node = escorted_actors(this);
    let mut previous = 0u32;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let id = node_item(e, node);
        let remove = if id == 0 {
            true
        } else {
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let actor = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TES_FORM_TYPE, ACTOR_TYPE, 0u32],
                )
                .u32();
            if actor == 0 {
                true
            } else {
                e.with_stack(4, |e, word| {
                    e.mem.set_u32(word.addr(), actor);
                    e.call(LIST_NODE_SET_ITEM, &args![node, word]);
                });
                false
            }
        };
        if !remove {
            previous = node;
            node = e.call(LIST_NEXT, &args![node]).u32();
        } else if previous != 0 {
            // The node's own address is the address of its item word.
            let item_word = e.call(LIST_NODE, &args![node]).u32();
            e.call(LIST_REMOVE, &args![previous, item_word]);
            node = e.call(LIST_NEXT, &args![previous]).u32();
        } else {
            e.call(LIST_REMOVE_HEAD, &args![node]);
        }
    }
    let actor = e.vcall(owner.addr(), 0x0c, &args![]).u32();
    e.set(this, EscortActorPackageData::pActor, Ptr::new(actor));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009f0980,
            escort_actor_package_data_scalar_deleting_destructor(
                Ptr<EscortActorPackageData>,
                u32,
            )
                -> Ptr<EscortActorPackageData>
        ),
        entry!(0x009f09b0, fn_009f09b0(Ptr<EscortActorPackageData>)),
        entry!(0x009f0a20, fn_009f0a20(Ptr<EscortActorPackageData>, Ptr)),
        entry!(0x009f0a60, fn_009f0a60(Ptr<EscortActorPackageData>, Ptr)),
        entry!(
            0x009f0a80,
            escort_actor_package_data_get_num_escorted_actors_of_type(
                Ptr<EscortActorPackageData>,
                Ptr,
                Ptr,
            ) -> i32
        ),
        entry!(
            0x009f0b70,
            escort_actor_package_data_handle_deleted_reference(Ptr<EscortActorPackageData>, Ptr)
        ),
        entry!(
            0x009f0bb0,
            escort_actor_package_data_save_game(Ptr<EscortActorPackageData>, Ptr)
        ),
        entry!(
            0x009f0c40,
            escort_actor_package_data_load_game(Ptr<EscortActorPackageData>, Ptr)
        ),
        entry!(
            0x009f0cf0,
            escort_actor_package_data_init_load_game(Ptr<EscortActorPackageData>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Removes the first node (from `start`) whose item equals `item`: the
    /// head takes the second node's item and link (or is zeroed), a later
    /// node is unlinked.
    fn remove_item(e: &mut Engine, start: u32, item: u32) {
        let mut previous = 0;
        let mut node = start;
        while node != 0 {
            if e.mem.u32(node) == item {
                if previous == 0 {
                    let next = e.mem.u32(node + 4);
                    if next == 0 {
                        e.mem.set_u32(node, 0);
                    } else {
                        let next_item = e.mem.u32(next);
                        let next_next = e.mem.u32(next + 4);
                        e.mem.set_u32(node, next_item);
                        e.mem.set_u32(node + 4, next_next);
                    }
                } else {
                    let next = e.mem.u32(node + 4);
                    e.mem.set_u32(previous + 4, next);
                }
                return;
            }
            previous = node;
            node = e.mem.u32(node + 4);
        }
    }

    /// List doubles working on real nodes (item, next) and doubles for the
    /// other callees outside this file.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_NODE, |_, a| ret(a[0]));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(LIST_REMOVE, |e, a| {
            let item = e.mem.u32(a[1]);
            remove_item(e, a[0], item);
            Ret::default()
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let item = e.mem.u32(a[0]);
            remove_item(e, a[0], item);
            Ret::default()
        });
        e.register(LIST_NODE_SET_ITEM, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        // Records the item word's value at `SEEN`.
        e.map(SEEN, 0x10);
        e.register(LIST_ADD, |e, a| {
            let item = e.mem.u32(a[1]);
            let count = e.mem.u32(SEEN);
            e.mem.set_u32(SEEN + 4 + 4 * (count % 3), item);
            e.mem.set_u32(SEEN, count + 1);
            Ret::default()
        });
        for addr in [
            FREE,
            LIST_CLEAR,
            LIST_DESTROY,
            ACTOR_PACKAGE_DATA_DESTROY,
            LIST_CONTAINS,
            ACTOR_FLAG,
            ACTOR_COULD_BE_FOLLOWING,
            REFERENCE_MATCHES_FORM,
            SAVE_FORM_ID,
            START_VARIABLE_SIZED_VALUE,
            SAVE_VARIABLE_SIZED_VALUE,
            LOAD_FORM_ID,
            LOAD_FORM_ID_INTO,
            LOAD_VARIABLE_SIZED_VALUE,
            LOOKUP_FORM,
            DYNAMIC_CAST,
            GET_ESCORT_PACKAGE_DATA,
        ] {
            stub(&mut e, addr, 0);
        }
        e
    }

    const SEEN: u32 = 0x0300_0000;

    /// The package data with the given escorted actors (0 for a null item).
    fn data_with(e: &mut Engine, items: &[u32]) -> Ptr<EscortActorPackageData> {
        let this: Ptr<EscortActorPackageData> = e.new_object();
        let mut node = this.addr() + 0x10;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    fn items(e: &Engine, this: Ptr<EscortActorPackageData>) -> Vec<u32> {
        let mut out = vec![];
        let mut node = this.addr() + 0x10;
        while node != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    /// An object whose vtable slot `slot` answers `value`.
    fn object(e: &mut Engine, vtable: u32, slot: u32, value: u32) -> u32 {
        e.map(vtable, slot + 4);
        let target = 0x7000_0000 + vtable / 0x100 + slot;
        e.mem.set_u32(vtable + slot, target);
        stub(e, target, value);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn scalar_deleting_destructor_runs_the_destructor_and_frees_with_bit_0() {
        let mut e = engine();
        let this = data_with(&mut e, &[]);
        e.call_log = Some(vec![]);
        let back = e.call(0x009f_0980, &args![this, 1u32]).u32();
        assert_eq!(back, this.addr());
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![LIST_CLEAR, LIST_DESTROY, ACTOR_PACKAGE_DATA_DESTROY, FREE]
        );
        assert_eq!(calls(&log, LIST_CLEAR), vec![vec![this.addr() + 0x10]]);
        assert_eq!(
            e.get(this, EscortActorPackageData::vfptr),
            ESCORT_ACTOR_PACKAGE_DATA_VTABLE
        );
        e.call_log = Some(vec![]);
        e.call(0x009f_0980, &args![this, 0u32]);
        assert!(calls(&e.call_log.take().unwrap(), FREE).is_empty());
    }

    #[test]
    fn destructor_sets_the_vtable_and_destroys_the_list() {
        let mut e = engine();
        let this = data_with(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x009f_09b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, LIST_DESTROY), vec![vec![this.addr() + 0x10]]);
        assert_eq!(
            calls(&log, ACTOR_PACKAGE_DATA_DESTROY),
            vec![vec![this.addr()]]
        );
        assert_eq!(
            e.get(this, EscortActorPackageData::vfptr),
            ESCORT_ACTOR_PACKAGE_DATA_VTABLE
        );
    }

    #[test]
    fn add_escorted_actor_skips_actors_already_listed() {
        let mut e = engine();
        let this = data_with(&mut e, &[]);
        e.call(0x009f_0a20, &args![this, 0x1111u32]);
        assert_eq!(e.mem.u32(SEEN), 1);
        assert_eq!(e.mem.u32(SEEN + 4), 0x1111);
        stub(&mut e, LIST_CONTAINS, 1);
        e.call(0x009f_0a20, &args![this, 0x2222u32]);
        assert_eq!(e.mem.u32(SEEN), 1);
    }

    #[test]
    fn remove_escorted_actor_removes_it_from_the_list() {
        let mut e = engine();
        let this = data_with(&mut e, &[0x1111, 0x2222, 0x3333]);
        e.call(0x009f_0a60, &args![this, 0x2222u32]);
        assert_eq!(items(&e, this), vec![0x1111, 0x3333]);
    }

    #[test]
    fn count_drops_flagged_and_not_following_actors_then_counts_matches() {
        let mut e = engine();
        let this = data_with(&mut e, &[0xa, 0, 0xb, 0xc]);
        e.register(ACTOR_FLAG, |_, a| ret((a[0] == 0xa) as u32));
        e.register(ACTOR_COULD_BE_FOLLOWING, |_, a| {
            assert_eq!(a[1], 0x5150);
            ret((a[0] != 0xc) as u32)
        });
        e.register(REFERENCE_MATCHES_FORM, |_, a| {
            assert_eq!(a[1], 0xf0f0);
            ret((a[0] == 0xb) as u32)
        });
        let count = e
            .call(0x009f_0a80, &args![this, 0x5150u32, 0xf0f0u32])
            .i32();
        assert_eq!(count, 1);
        assert_eq!(items(&e, this), vec![0, 0xb]);
    }

    #[test]
    fn deleted_actor_references_are_removed() {
        let mut e = engine();
        let this = data_with(&mut e, &[0x1111]);
        let actor = object(&mut e, 0x0200_1000, 0x100, 1);
        e.call_log = Some(vec![]);
        e.call(0x009f_0b70, &args![this, actor]);
        assert_eq!(calls(&e.call_log.take().unwrap(), LIST_REMOVE).len(), 1);
        let other = object(&mut e, 0x0200_2000, 0x100, 0);
        e.call_log = Some(vec![]);
        e.call(0x009f_0b70, &args![this, other]);
        assert!(calls(&e.call_log.take().unwrap(), LIST_REMOVE).is_empty());
    }

    #[test]
    fn save_writes_the_package_and_the_escorted_actors() {
        let mut e = engine();
        let this = data_with(&mut e, &[0xa, 0, 0xc]);
        e.set(this, EscortActorPackageData::pPackage, Ptr::new(0x9999));
        stub(&mut e, START_VARIABLE_SIZED_VALUE, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x009f_0bb0, &args![this, 0xbbbbu32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SAVE_FORM_ID),
            vec![
                vec![0xbbbb, 0x9999, 0],
                vec![0xbbbb, 0xa, 0],
                vec![0xbbbb, 0xc, 0]
            ]
        );
        assert_eq!(
            calls(&log, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0xbbbb, 2, 0x40]]
        );
    }

    #[test]
    fn load_reads_the_package_and_the_escorted_actor_ids() {
        let mut e = engine();
        let this = data_with(&mut e, &[]);
        stub(&mut e, LOAD_FORM_ID, 0x0001_0001);
        stub(&mut e, LOOKUP_FORM, 0x8888);
        stub(&mut e, DYNAMIC_CAST, 0x8888);
        stub(&mut e, GET_ESCORT_PACKAGE_DATA, 0x7777);
        stub(&mut e, LOAD_VARIABLE_SIZED_VALUE, 2);
        e.register(LOAD_FORM_ID_INTO, |e, a| {
            e.mem.set_u32(a[1], 0x0002_0002);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009f_0c40, &args![this, 0xbbbbu32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, LOOKUP_FORM), vec![vec![0x0001_0001]]);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![vec![0x8888, 0, TES_FORM_TYPE, TES_PACKAGE_TYPE, 0]]
        );
        assert_eq!(e.get(this, EscortActorPackageData::pPackage).addr(), 0x8888);
        assert_eq!(
            e.get(this, EscortActorPackageData::pPackageData).addr(),
            0x7777
        );
        assert_eq!(calls(&log, LOAD_FORM_ID_INTO).len(), 2);
        assert_eq!(e.mem.u32(SEEN), 2);
        assert_eq!(e.mem.u32(SEEN + 8), 0x0002_0002);

        // No package: no package data.
        stub(&mut e, DYNAMIC_CAST, 0);
        stub(&mut e, LOAD_VARIABLE_SIZED_VALUE, 0);
        e.set(this, EscortActorPackageData::pPackageData, Ptr::new(1));
        e.call_log = Some(vec![]);
        e.call(0x009f_0c40, &args![this, 0xbbbbu32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, GET_ESCORT_PACKAGE_DATA).is_empty());
        assert!(e.get(this, EscortActorPackageData::pPackageData).is_null());
    }

    #[test]
    fn init_load_resolves_actors_and_drops_the_rest() {
        let mut e = engine();
        // Ids 0x10 and 0x30 are actors, 0x20 is not; one null item.
        e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x1000));
        e.register(DYNAMIC_CAST, |_, a| {
            assert_eq!(&a[1..], &[0, TES_FORM_TYPE, ACTOR_TYPE, 0]);
            ret(if a[0] == 0x1020 { 0 } else { a[0] })
        });
        let owner = object(&mut e, 0x0200_3000, 0x0c, 0xacac);
        let this = data_with(&mut e, &[0x10, 0, 0x20, 0x30]);
        e.call(0x009f_0cf0, &args![this, owner]);
        assert_eq!(items(&e, this), vec![0x1010, 0x1030]);
        assert_eq!(e.get(this, EscortActorPackageData::pActor).addr(), 0xacac);

        // A bad head item goes through the head removal.
        let this = data_with(&mut e, &[0x20, 0x10]);
        e.call_log = Some(vec![]);
        e.call(0x009f_0cf0, &args![this, owner]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, LIST_REMOVE_HEAD).len(), 1);
        assert_eq!(items(&e, this), vec![0x1010]);
    }
}
