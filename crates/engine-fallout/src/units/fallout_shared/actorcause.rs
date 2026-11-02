//! `fallout shared/actorcause.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ActorCause` (Xbox PDB), 0x1c bytes.
    pub struct ActorCause: 0x1c {
        /// `pActor` (Xbox PDB): `Actor*`.
        0x00 pActor: u32,
        /// `pWitnesses` (Xbox PDB): `BSSimpleList<DetectionState *>*`.
        0x04 pWitnesses: u32,
        /// `kOrigin` (Xbox PDB): an `NiPoint3`.
        0x08 kOrigin: u32,
        /// `iActorCauseID` (Xbox PDB).
        0x14 iActorCauseID: i32,
        /// `iRefCount` (Xbox PDB).
        0x18 iRefCount: i32,
    }
}

/// The word saved before the cause list (the Xbox PDB has the static
/// `ActorCause::iActorCauseIDCounter`; not tied to this address).
const CAUSE_ID_COUNTER: u32 = 0x011d_60cc;
/// The list of causes `0066e6d0` saves and `0066df40` (the destructor body) removes from (the Xbox
/// PDB has the static `ActorCause::kCauseList`; not tied to this address).
const CAUSE_LIST: u32 = 0x011d_6124;
/// The global whose `0042ce10` test decides whether a cause without an id
/// stays on the list.
const GAME_STATE: u32 = 0x011d_df38;
/// The zero point (`NiPoint3`, three words).
const ZERO_POINT: u32 = 0x011f_426c;
/// `FLT_MAX` (`float`); the constructor stores its negation.
const FLOAT_MAX: u32 = 0x0106_73a0;

/// `008256d0` (`thiscall`): whether a list is empty (`AL`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (`thiscall`): a list's first item address (also run on a
/// point before it is set).
const LIST_ITEM: u32 = 0x0068_15c0;
/// `0063f7b0` (`thiscall`): removes a list's head.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `00726070` (`thiscall`): the next list node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `004702f0` (`thiscall`, `RET 4`): the list's deleting destructor.
const LIST_DELETE: u32 = 0x0047_02f0;
/// `0042ce10` (`thiscall`, no stack words): a test on the global at
/// `011ddf38` (`AL`).
const GAME_STATE_TEST: u32 = 0x0042_ce10;
/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `004019a0` (`cdecl`, the engine map's name `WaitForSingleObjectEx` is a
/// folded body): run on the reference count; 0 means the last reference
/// went.
const RELEASE_REFERENCE: u32 = 0x0040_19a0;
/// `BGSSaveGameBuffer` raw save (`00865e50`, `thiscall`: data, size, 0).
const SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (engine map), `thiscall`: form, 0.
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (engine map), `thiscall`.
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (engine map),
/// `thiscall`: count, handle.
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
/// `DetectionState::SaveGame` (engine map), `thiscall`, `RET 4`.
const DETECTION_STATE_SAVE: u32 = 0x008d_7070;
/// `004300a0` (unit `run_00430000`): sets a float to 0.
const FLOAT_ZERO: u32 = 0x0043_00a0;
/// `00435de0` (`thiscall`, `RET 4`): stores the float argument at `this`.
const FLOAT_SET: u32 = 0x0043_5de0;

// Translated from 0066df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ActorCause` destructor body (no name in the engine map): clears the
/// actor, frees every witness (`00401030` on each item while the list is not
/// empty, removing the head with `0063f7b0`), deletes the list
/// (`004702f0` with 1) and clears it; then, unless `0042ce10` holds for the
/// global at `011ddf38` and the cause has no id (-1), hands the cause to
/// `00905330` on the cause list.
pub fn fn_0066df40(e: &mut Engine, this: Ptr<ActorCause>) {
    e.set(this, ActorCause::pActor, 0);
    loop {
        let witnesses = e.get(this, ActorCause::pWitnesses);
        if witnesses == 0 || e.call(LIST_IS_EMPTY, &args![witnesses]).bool() {
            break;
        }
        let witnesses = e.get(this, ActorCause::pWitnesses);
        let item_address = e.call(LIST_ITEM, &args![witnesses]).u32();
        let item = e.mem.u32(item_address);
        e.call(OPERATOR_DELETE, &args![item]);
        let witnesses = e.get(this, ActorCause::pWitnesses);
        e.call(LIST_REMOVE_HEAD, &args![witnesses]);
    }
    let witnesses = e.get(this, ActorCause::pWitnesses);
    if witnesses != 0 {
        e.call(LIST_DELETE, &args![witnesses, 1u32]);
    }
    e.set(this, ActorCause::pWitnesses, 0);
    let mut remove = true;
    let state = e.global::<u32>(GAME_STATE);
    if e.call(GAME_STATE_TEST, &args![state]).bool() && e.get(this, ActorCause::iActorCauseID) == -1
    {
        remove = false;
    }
    if remove {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), this.addr());
            e.call(LIST_REMOVE_ITEM, &args![CAUSE_LIST, slot]);
        });
    }
}

// Translated from 0066e2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a reference to the cause (no name in the engine map): when
/// `004019a0` on the reference count answers 0 and `this` is not null, the
/// cause deletes itself (`0066e300` with 1).
pub fn fn_0066e2b0(e: &mut Engine, this: Ptr<ActorCause>) {
    let count = this.addr().wrapping_add(0x18);
    if e.call(RELEASE_REFERENCE, &args![count]).u32() == 0 && !this.is_null() {
        fn_0066e300(e, this, 1);
    }
}

// Translated from 0066e300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ActorCause` scalar deleting destructor (no name in the engine map):
/// the destructor body (`0066df40`), then `00401030` when `flags & 1`.
/// Returns `this`.
pub fn fn_0066e300(e: &mut Engine, this: Ptr<ActorCause>, flags: u32) -> Ptr<ActorCause> {
    fn_0066df40(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0066e390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorCause::SaveGame` (Xbox PDB): the origin (12 bytes), the id and the
/// actor's form id, then a variable-sized value holding each witness's
/// detection state (`008d7070`) and their number.
pub fn actor_cause_save_game(e: &mut Engine, this: Ptr<ActorCause>, buffer: u32) {
    let base = this.addr();
    e.call(SAVE_DATA, &args![buffer, base + 8, 0xcu32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, base + 0x14, 4u32, 0u32]);
    let actor = e.get(this, ActorCause::pActor);
    e.call(SAVE_FORM_ID, &args![buffer, actor, 0u32]);
    let mut count = 0u32;
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = e.get(this, ActorCause::pWitnesses);
    while node != 0 {
        let item_address = e.call(LIST_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if item != 0 {
            e.call(DETECTION_STATE_SAVE, &args![item, buffer]);
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count, handle]);
}

// Translated from 0066e590 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor of a 0x24-byte detection record (the engine map's name
/// `BSSimpleList<DetectionState_P>::AddHead` is a folded body; the class is
/// unconfirmed): `006815c0` on the point at +0xc, `004300a0` on the float at
/// +0x18, then the words at +0 and +4 cleared, byte +0x1e 0, byte +0x1f 1,
/// -100 at +8, the zero point at +0xc, `-FLT_MAX` (built by `00435de0` in a
/// local) at +0x18, bytes +0x1c and +0x1d 0 and -100 at +0x20. Returns
/// `this`.
pub fn fn_0066e590(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(LIST_ITEM, &args![base + 0xc]);
    e.call(FLOAT_ZERO, &args![base + 0x18]);
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u8(base + 0x1e, 0);
    e.mem.set_u8(base + 0x1f, 1);
    e.mem.set_u32(base + 8, 0xffff_ff9c);
    for i in 0..3 {
        let word = e.global::<u32>(ZERO_POINT + 4 * i);
        e.mem.set_u32(base + 0xc + 4 * i, word);
    }
    let lowest = x87_quiet_bits(e.global::<u32>(FLOAT_MAX)) ^ 0x8000_0000;
    let stored = e.with_stack(4, |e, local| {
        e.call(FLOAT_SET, &args![local, lowest]);
        e.mem.u32(local.addr())
    });
    e.mem.set_u32(base + 0x18, stored);
    e.mem.set_u8(base + 0x1c, 0);
    e.mem.set_u8(base + 0x1d, 0);
    e.mem.set_u32(base + 0x20, 0xffff_ff9c);
    this
}

// Translated from 0066e6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorCause::SaveActorCausesList` (Xbox PDB; `cdecl`, the buffer): the
/// word at `011d60cc`, then a variable-sized value holding each cause of
/// the list at `011d6124` (`0066e390`) and their number.
pub fn actor_cause_save_actor_causes_list(e: &mut Engine, buffer: u32) {
    e.call(SAVE_DATA, &args![buffer, CAUSE_ID_COUNTER, 4u32, 0u32]);
    let mut count = 0u32;
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = CAUSE_LIST;
    while node != 0 {
        let item_address = e.call(LIST_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if item != 0 {
            actor_cause_save_game(e, Ptr::new(item), buffer);
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count, handle]);
}

/// `005ae3d0` (`thiscall`, `RET 4`): adds the item whose word is at the
/// given address to the list.
const LIST_APPEND: u32 = 0x005a_e3d0;
/// `00905330` (`thiscall`, `RET 4`): removes from the list the item whose
/// word is at the given address (the callers free the item right after).
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// `operator new` (`00401000`, `cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `BGSLoadGameBuffer` raw load (`00864980`, `thiscall`: destination,
/// size).
const LOAD_DATA: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (engine map), `thiscall`: the
/// destination word.
const LOAD_FORM_ID: u32 = 0x0086_48e0;
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (engine map), `thiscall`.
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;

// Translated from 0066deb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ActorCause` constructor (no name in the engine map): `006815c0` on
/// the origin, no actor or witnesses, the zero point (`011f426c`) as
/// origin, reference count 0, the id from the counter at `011d60cc`
/// (incremented), then the cause is added to the cause list (`005ae3d0`
/// with a stack word holding `this`). Returns `this`.
pub fn fn_0066deb0(e: &mut Engine, this: Ptr<ActorCause>) -> Ptr<ActorCause> {
    let base = this.addr();
    e.call(LIST_ITEM, &args![base + 8]);
    e.set(this, ActorCause::pActor, 0);
    e.set(this, ActorCause::pWitnesses, 0);
    for i in 0..3 {
        let word = e.global::<u32>(ZERO_POINT + 4 * i);
        e.mem.set_u32(base + 8 + 4 * i, word);
    }
    e.set(this, ActorCause::iRefCount, 0);
    let id = e.global::<u32>(CAUSE_ID_COUNTER);
    e.set(this, ActorCause::iActorCauseID, id as i32);
    let next = e.global::<u32>(CAUSE_ID_COUNTER).wrapping_add(1);
    e.mem.set_u32(CAUSE_ID_COUNTER, next);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), base);
        e.call(LIST_APPEND, &args![CAUSE_LIST, slot]);
    });
    this
}

// Translated from 0066e100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorCause::ClearActor` (engine map), `cdecl`: for each cause of the
/// cause list (until an empty node) for which `005ca4f0` holds: when its
/// actor (`00559450`) is `actor` it is reset ([`fn_0066e200`]); then the
/// first of its witnesses (from the list at +4 by `00726070`, until a null
/// item) whose word +0 is `actor` is removed (`00905330`) and freed.
pub fn actor_cause_clear_actor(e: &mut Engine, actor: u32) {
    let mut node = CAUSE_LIST;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item = e.call(LIST_ITEM, &args![node]).u32();
        let cause = e.mem.u32(item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        if cause == 0 || !e.call(0x005c_a4f0, &args![cause]).bool() {
            continue;
        }
        if e.call(0x0055_9450, &args![cause]).u32() == actor {
            fn_0066e200(e, Ptr::new(cause));
        }
        let mut witness = e.call(NODE_NEXT, &args![cause]).u32();
        let mut done = false;
        while !done && witness != 0 {
            let item = e.call(LIST_ITEM, &args![witness]).u32();
            if e.mem.u32(item) == 0 {
                break;
            }
            let item = e.call(LIST_ITEM, &args![witness]).u32();
            let state = e.mem.u32(item);
            witness = e.call(NODE_NEXT, &args![witness]).u32();
            if e.mem.u32(state) == actor {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), state);
                    let list = e.call(NODE_NEXT, &args![cause]).u32();
                    e.call(LIST_REMOVE_ITEM, &args![list, slot]);
                    let state = e.mem.u32(slot.addr());
                    e.call(OPERATOR_DELETE, &args![state]);
                });
                done = true;
            }
        }
    }
}

// Translated from 0066e200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets a cause (no name in the engine map): clears the actor, frees
/// every witness (`00401030` on each head item, removed with `0063f7b0`,
/// while the list is not empty), deletes the list (`004702f0` with 1) and
/// clears it, and sets the origin to the zero point.
pub fn fn_0066e200(e: &mut Engine, this: Ptr<ActorCause>) {
    e.set(this, ActorCause::pActor, 0);
    loop {
        let witnesses = e.get(this, ActorCause::pWitnesses);
        if witnesses == 0 || e.call(LIST_IS_EMPTY, &args![witnesses]).bool() {
            break;
        }
        let witnesses = e.get(this, ActorCause::pWitnesses);
        let item_address = e.call(LIST_ITEM, &args![witnesses]).u32();
        let item = e.mem.u32(item_address);
        e.call(OPERATOR_DELETE, &args![item]);
        let witnesses = e.get(this, ActorCause::pWitnesses);
        e.call(LIST_REMOVE_HEAD, &args![witnesses]);
    }
    let witnesses = e.get(this, ActorCause::pWitnesses);
    if witnesses != 0 {
        e.call(LIST_DELETE, &args![witnesses, 1u32]);
    }
    e.set(this, ActorCause::pWitnesses, 0);
    let base = this.addr();
    for i in 0..3 {
        let word = e.global::<u32>(ZERO_POINT + 4 * i);
        e.mem.set_u32(base + 8 + 4 * i, word);
    }
}

// Translated from 0066e330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorCause::GetActorCauseByID` (engine map), `cdecl`: null for a
/// negative id; otherwise the first cause of the cause list (every node,
/// null items skipped) with that id, or null.
pub fn actor_cause_get_actor_cause_by_id(e: &mut Engine, id: i32) -> u32 {
    if id < 0 {
        return 0;
    }
    let mut node = CAUSE_LIST;
    while node != 0 {
        let item = e.call(LIST_ITEM, &args![node]).u32();
        let cause = e.mem.u32(item);
        if cause != 0 && e.get(Ptr::<ActorCause>::new(cause), ActorCause::iActorCauseID) == id {
            return cause;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

// Translated from 0066e440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066e440 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `ActorCause::LoadGame` (engine map), `RET 4`: the origin (12 bytes), the
/// id (4) and the actor's form id from `buffer`; then a variable-sized
/// count. With a count, the witness list is cleared (`00470470`) or made
/// (8 bytes, `0096a2d0`; null when the allocation fails), and each witness
/// is a new detection record (0x24 bytes, [`fn_0066e590`]) loaded
/// (`008d7140`) and added to it (`005ae3d0`).
pub fn actor_cause_load_game(e: &mut Engine, this: Ptr<ActorCause>, buffer: u32) {
    let base = this.addr();
    e.call(LOAD_DATA, &args![buffer, base + 8, 0xcu32]);
    e.call(LOAD_DATA, &args![buffer, base + 0x14, 4u32]);
    e.call(LOAD_FORM_ID, &args![buffer, this]);
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    if count == 0 {
        return;
    }
    let witnesses = e.get(this, ActorCause::pWitnesses);
    if witnesses != 0 {
        e.call(0x0047_0470, &args![witnesses]);
    } else {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = if block == 0 {
            0
        } else {
            e.call(0x0096_a2d0, &args![block]).u32()
        };
        e.set(this, ActorCause::pWitnesses, list);
    }
    for _ in 0..count {
        let block = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
        let state = if block == 0 {
            0
        } else {
            fn_0066e590(e, Ptr::new(block)).addr()
        };
        e.call(0x008d_7140, &args![state, buffer]);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), state);
            let witnesses = e.get(this, ActorCause::pWitnesses);
            e.call(LIST_APPEND, &args![witnesses, slot]);
        });
    }
}

// Translated from 0066e640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorCause::InitLoadGame` (engine map), `RET 4` (the word is not
/// read): the actor's form id becomes the form (`004839c0`) cast from
/// `TESForm` to `Actor` (`__RTDynamicCast`), null without one; then each
/// witness (null items skipped) runs `008d7220`.
pub fn actor_cause_init_load_game(e: &mut Engine, this: Ptr<ActorCause>, _unused_1: u32) {
    let id = e.get(this, ActorCause::pActor);
    let actor = if id != 0 {
        let form = e.call(0x0048_39c0, &args![id]).u32();
        e.call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_3028u32, 0x0118_46d4u32, 0u32],
        )
        .u32()
    } else {
        0
    };
    e.set(this, ActorCause::pActor, actor);
    let mut node = e.get(this, ActorCause::pWitnesses);
    while node != 0 {
        let item = e.call(LIST_ITEM, &args![node]).u32();
        let state = e.mem.u32(item);
        if state != 0 {
            e.call(0x008d_7220, &args![state]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0066e750 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066e750 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `ActorCause::LoadActorCausesList` (engine map), `cdecl`: the counter at
/// `011d60cc` (4 bytes), then a variable-sized count of new causes (0x1c
/// bytes, [`fn_0066deb0`]; null when the allocation fails), each loaded
/// ([`actor_cause_load_game`]).
pub fn actor_cause_load_actor_causes_list(e: &mut Engine, buffer: u32) {
    e.call(LOAD_DATA, &args![buffer, CAUSE_ID_COUNTER, 4u32]);
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    for _ in 0..count {
        let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
        let cause = if block == 0 {
            Ptr::NULL
        } else {
            fn_0066deb0(e, Ptr::new(block))
        };
        actor_cause_load_game(e, cause, buffer);
    }
}

// Translated from 0066e810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorCause::InitLoadActorCausesList` (engine map), `cdecl`: walks the
/// cause list (until an empty node). A cause whose `0047d3d0` (16 bits)
/// is not 0 runs [`actor_cause_init_load_game`] with the word; any other
/// cause gets id -1 and deletes itself ([`fn_0066e300`] with 1), and it is
/// removed from the list like a null item: after the previous node
/// (`00905330`, continuing at the previous node's next) or as the head
/// (`0063f7b0`).
pub fn actor_cause_init_load_actor_causes_list(e: &mut Engine, word: u32) {
    let mut node = CAUSE_LIST;
    let mut previous = 0u32;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item = e.call(LIST_ITEM, &args![node]).u32();
        let cause = e.mem.u32(item);
        let mut remove = false;
        if cause != 0 {
            let cause: Ptr<ActorCause> = Ptr::new(cause);
            if i32::from(e.call(0x0047_d3d0, &args![cause]).u16()) > 0 {
                actor_cause_init_load_game(e, cause, word);
            } else {
                e.set(cause, ActorCause::iActorCauseID, -1);
                if !cause.is_null() {
                    fn_0066e300(e, cause, 1);
                }
                remove = true;
            }
        } else {
            remove = true;
        }
        if remove {
            if previous != 0 {
                let item = e.call(LIST_ITEM, &args![node]).u32();
                e.call(LIST_REMOVE_ITEM, &args![previous, item]);
                node = e.call(NODE_NEXT, &args![previous]).u32();
            } else {
                e.call(LIST_REMOVE_HEAD, &args![node]);
            }
        } else {
            previous = node;
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_df40, fn_0066df40(Ptr<ActorCause>)),
        entry!(0x0066_e2b0, fn_0066e2b0(Ptr<ActorCause>)),
        entry!(
            0x0066_e300,
            fn_0066e300(Ptr<ActorCause>, u32) -> Ptr<ActorCause>
        ),
        entry!(0x0066_e390, actor_cause_save_game(Ptr<ActorCause>, u32)),
        entry!(0x0066_e590, fn_0066e590(Ptr) -> Ptr),
        entry!(0x0066_e6d0, actor_cause_save_actor_causes_list(u32)),
        entry!(0x0066_deb0, fn_0066deb0(Ptr<ActorCause>) -> Ptr<ActorCause>),
        entry!(0x0066_e100, actor_cause_clear_actor(u32)),
        entry!(0x0066_e200, fn_0066e200(Ptr<ActorCause>)),
        entry!(0x0066_e330, actor_cause_get_actor_cause_by_id(i32) -> u32),
        entry!(0x0066_e440, actor_cause_load_game(Ptr<ActorCause>, u32)),
        entry!(
            0x0066_e640,
            actor_cause_init_load_game(Ptr<ActorCause>, u32)
        ),
        entry!(0x0066_e750, actor_cause_load_actor_causes_list(u32)),
        entry!(0x0066_e810, actor_cause_init_load_actor_causes_list(u32)),
    ]
}

#[cfg(test)]
mod tests {
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

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// A witness list modelled as [count, first item, second item]; the
    /// head removal drops the first.
    fn witness_list(e: &mut Engine, items: &[u32]) -> u32 {
        let list = e.mem.alloc(4 + 4 * items.len() as u32);
        e.mem.set_u32(list, items.len() as u32);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(list + 4 + 4 * i as u32, *item);
        }
        e.register(LIST_IS_EMPTY, |e, a| word(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(LIST_ITEM, |_, a| word(a[0] + 4));
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let count = e.mem.u32(a[0]);
            for i in 1..count {
                let next = e.mem.u32(a[0] + 4 + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, next);
            }
            e.mem.set_u32(a[0], count - 1);
            Ret::default()
        });
        list
    }

    fn destructor_engine(state_test: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_d000, 0x1000);
        e.set_global(GAME_STATE, 0x4444u32);
        e.register(OPERATOR_DELETE, nothing);
        e.register(LIST_DELETE, nothing);
        e.register_double(GAME_STATE_TEST, move |_, _| word(u32::from(state_test)));
        e.register(LIST_REMOVE_ITEM, |e, a| word(e.mem.u32(a[1])));
        e
    }

    #[test]
    fn destructor_body_frees_the_witnesses_and_returns_the_cause() {
        let mut e = destructor_engine(false);
        let list = witness_list(&mut e, &[0x71, 0x72]);
        let cause = e.mem.alloc(0x1c);
        e.mem.set_u32(cause, 0x99);
        e.mem.set_u32(cause + 4, list);
        e.mem.set_i32(cause + 0x14, -1);
        e.call_log = Some(vec![]);
        e.call(0x0066_df40, &args![cause]);
        assert_eq!(e.mem.u32(cause), 0);
        assert_eq!(e.mem.u32(cause + 4), 0);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x71], vec![0x72]]);
        assert_eq!(calls_to(&e, LIST_DELETE), vec![vec![list, 1]]);
        assert_eq!(calls_to(&e, GAME_STATE_TEST), vec![vec![0x4444]]);
        let removes = calls_to(&e, LIST_REMOVE_ITEM);
        assert_eq!(removes.len(), 1);
        assert_eq!(removes[0][0], CAUSE_LIST);
    }

    #[test]
    fn destructor_body_drops_a_cause_without_an_id_when_the_test_holds() {
        let mut e = destructor_engine(true);
        let cause = e.mem.alloc(0x1c);
        e.mem.set_i32(cause + 0x14, -1);
        e.call_log = Some(vec![]);
        e.call(0x0066_df40, &args![cause]);
        assert!(calls_to(&e, LIST_DELETE).is_empty());
        assert!(calls_to(&e, LIST_REMOVE_ITEM).is_empty());
        // With an id it is taken off the cause list.
        e.mem.set_i32(cause + 0x14, 5);
        e.call(0x0066_df40, &args![cause]);
        assert_eq!(calls_to(&e, LIST_REMOVE_ITEM).len(), 1);
    }

    #[test]
    fn release_deletes_on_the_last_reference() {
        let mut e = destructor_engine(true);
        let cause = e.mem.alloc(0x1c);
        e.mem.set_i32(cause + 0x14, -1);
        e.register(RELEASE_REFERENCE, |e, a| word(e.mem.u32(a[0])));
        e.mem.set_u32(cause + 0x18, 1);
        e.call_log = Some(vec![]);
        e.call(0x0066_e2b0, &args![cause]);
        assert_eq!(calls_to(&e, RELEASE_REFERENCE), vec![vec![cause + 0x18]]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.mem.set_u32(cause + 0x18, 0);
        e.call(0x0066_e2b0, &args![cause]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![cause]]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_zero() {
        let mut e = destructor_engine(true);
        let cause = e.mem.alloc(0x1c);
        e.mem.set_i32(cause + 0x14, -1);
        e.mem.set_u32(cause, 0x99);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_e300, &args![cause, 0u32]).u32(), cause);
        assert_eq!(e.mem.u32(cause), 0);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x0066_e300, &args![cause, 1u32]).u32(), cause);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![cause]]);
    }

    fn save_engine() -> Engine {
        let mut e = Engine::new();
        e.register(SAVE_DATA, nothing);
        e.register(SAVE_FORM_ID, nothing);
        e.register(START_VARIABLE_SIZED_VALUE, |_, _| word(0x55));
        e.register(SAVE_VARIABLE_SIZED_VALUE, nothing);
        e.register(DETECTION_STATE_SAVE, nothing);
        // Nodes of two words: item, next.
        e.register(LIST_ITEM, |_, a| word(a[0]));
        e.register(NODE_NEXT, |e, a| word(e.mem.u32(a[0] + 4)));
        e
    }

    #[test]
    fn save_game_saves_the_witnesses_that_exist() {
        let mut e = save_engine();
        let cause = e.mem.alloc(0x1c);
        e.mem.set_u32(cause, 0x99);
        let nodes = e.mem.alloc(0x18);
        e.mem.set_u32(nodes, 0x71);
        e.mem.set_u32(nodes + 4, nodes + 8);
        e.mem.set_u32(nodes + 8, 0);
        e.mem.set_u32(nodes + 0xc, nodes + 0x10);
        e.mem.set_u32(nodes + 0x10, 0x72);
        e.mem.set_u32(nodes + 0x14, 0);
        e.mem.set_u32(cause + 4, nodes);
        e.call_log = Some(vec![]);
        e.call(0x0066_e390, &args![cause, 0x9000u32]);
        assert_eq!(
            calls_to(&e, SAVE_DATA),
            vec![
                vec![0x9000, cause + 8, 0xc, 0],
                vec![0x9000, cause + 0x14, 4, 0]
            ]
        );
        assert_eq!(calls_to(&e, SAVE_FORM_ID), vec![vec![0x9000, 0x99, 0]]);
        assert_eq!(
            calls_to(&e, DETECTION_STATE_SAVE),
            vec![vec![0x71, 0x9000], vec![0x72, 0x9000]]
        );
        assert_eq!(
            calls_to(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0x9000, 2, 0x55]]
        );
    }

    #[test]
    fn save_actor_causes_list_saves_every_cause() {
        let mut e = save_engine();
        e.map(0x011d_6000, 0x1000);
        let cause = e.mem.alloc(0x1c);
        e.mem.set_u32(CAUSE_LIST, cause);
        e.mem.set_u32(CAUSE_LIST + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0066_e6d0, &args![0x9000u32]);
        assert_eq!(
            calls_to(&e, SAVE_DATA)[0],
            vec![0x9000, CAUSE_ID_COUNTER, 4, 0]
        );
        assert_eq!(calls_to(&e, SAVE_FORM_ID).len(), 1);
        assert_eq!(
            calls_to(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0x9000, 0, 0x55], vec![0x9000, 1, 0x55]]
        );
    }

    #[test]
    fn detection_record_constructor_sets_the_defaults() {
        let mut e = Engine::new();
        e.map(0x0106_7000, 0x1000);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.map(0x011f_4000, 0x1000);
        e.set_global(ZERO_POINT, 2.0f32);
        e.register(LIST_ITEM, nothing);
        e.register(FLOAT_ZERO, nothing);
        e.register(FLOAT_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = e.mem.alloc(0x24);
        for i in 0..9 {
            e.mem.set_u32(this + 4 * i, 0xcccc_cccc);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_e590, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, LIST_ITEM), vec![vec![this + 0xc]]);
        assert_eq!(calls_to(&e, FLOAT_ZERO), vec![vec![this + 0x18]]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.i32(this + 8), -100);
        assert_eq!(e.mem.f32(this + 0xc), 2.0);
        assert_eq!(e.mem.f32(this + 0x18), -f32::MAX);
        assert_eq!(e.mem.u32(this + 0x1c), 0x0100_0000);
        assert_eq!(e.mem.i32(this + 0x20), -100);
    }
}

#[cfg(test)]
mod n036_tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Plain list nodes (item +0, next +4) with the list helpers, and the
    /// globals mapped.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_6000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.map(0x011d_d000, 0x1000);
        e.map(0x0106_7000, 0x1000);
        e.register(LIST_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        for callee in [OPERATOR_DELETE, LIST_DELETE, LIST_REMOVE_ITEM, LIST_APPEND] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn constructor_takes_the_next_id_and_joins_the_list() {
        let mut e = list_engine();
        e.mem.set_u32(CAUSE_ID_COUNTER, 41);
        e.mem.set_u32(ZERO_POINT, 1);
        e.mem.set_u32(ZERO_POINT + 8, 3);
        let cause = e.new_object::<ActorCause>();
        e.mem.write(cause.addr(), &[0xcc; 0x1c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_deb0, &args![cause]).u32(), cause.addr());
        assert_eq!(e.get(cause, ActorCause::iActorCauseID), 41);
        assert_eq!(e.mem.u32(CAUSE_ID_COUNTER), 42);
        assert_eq!(
            (
                e.get(cause, ActorCause::pActor),
                e.get(cause, ActorCause::pWitnesses)
            ),
            (0, 0)
        );
        assert_eq!(e.get(cause, ActorCause::iRefCount), 0);
        assert_eq!(
            (e.mem.u32(cause.addr() + 8), e.mem.u32(cause.addr() + 0x10)),
            (1, 3)
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LIST_APPEND)[0][0], CAUSE_LIST);
    }

    #[test]
    fn reset_frees_the_witnesses() {
        let mut e = list_engine();
        e.mem.set_u32(ZERO_POINT + 4, 2);
        let cause = e.new_object::<ActorCause>();
        e.set(cause, ActorCause::pActor, 0x5000);
        let witnesses = e.mem.alloc(8);
        e.mem.set_u32(witnesses, 0x7000);
        e.set(cause, ActorCause::pWitnesses, witnesses);
        // Removing the head empties the node.
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0066_e200, &args![cause]);
        assert_eq!(e.get(cause, ActorCause::pActor), 0);
        assert_eq!(e.get(cause, ActorCause::pWitnesses), 0);
        assert_eq!(e.mem.u32(cause.addr() + 0xc), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![0x7000]]);
        assert_eq!(calls_to(&log, LIST_DELETE), vec![vec![witnesses, 1]]);
    }

    #[test]
    fn get_by_id_walks_the_list() {
        let mut e = list_engine();
        let a = e.new_object::<ActorCause>();
        e.set(a, ActorCause::iActorCauseID, 5);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, a.addr());
        e.mem.set_u32(CAUSE_LIST + 4, second);
        assert_eq!(e.call(0x0066_e330, &args![5u32]).u32(), a.addr());
        assert_eq!(e.call(0x0066_e330, &args![6u32]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_e330, &args![-1i32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn clear_actor_resets_its_causes_and_drops_its_witness_states() {
        let mut e = list_engine();
        e.register(0x005c_a4f0, |_, _| true.into_ret());
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        let cause = e.new_object::<ActorCause>();
        e.set(cause, ActorCause::pActor, 0x5000);
        // Witness states: the first for another actor, the second for 0x5000.
        let other = e.mem.alloc(8);
        e.mem.set_u32(other, 0x6000);
        let mine = e.mem.alloc(8);
        e.mem.set_u32(mine, 0x5000);
        let witness2 = e.mem.alloc(8);
        e.mem.set_u32(witness2, mine);
        let witnesses = e.mem.alloc(8);
        e.mem.set_u32(witnesses, other);
        e.mem.set_u32(witnesses + 4, witness2);
        e.set(cause, ActorCause::pWitnesses, witnesses);
        e.mem.set_u32(CAUSE_LIST, cause.addr());
        // The reset (0066e200) is direct: keep its list deletion visible.
        e.call_log = Some(vec![]);
        e.call(0x0066_e100, &args![0x6000u32]);
        let log = e.call_log.take().unwrap();
        // Not this cause's actor: no reset; the other actor's state goes.
        assert_eq!(e.get(cause, ActorCause::pActor), 0x5000);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![other]]);
        assert_eq!(calls_to(&log, LIST_REMOVE_ITEM)[0][0], witnesses);

        e.call_log = Some(vec![]);
        e.call(0x0066_e100, &args![0x5000u32]);
        assert_eq!(e.get(cause, ActorCause::pActor), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LIST_DELETE), vec![vec![witnesses, 1]]);
    }

    #[test]
    fn load_game_reads_fields_and_witnesses() {
        let mut e = list_engine();
        e.register(LOAD_DATA, |_, _| Ret::default());
        e.register(LOAD_FORM_ID, |_, _| Ret::default());
        e.register(LOAD_VARIABLE_SIZED_VALUE, |_, _| 2u32.into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0096_a2d0, |_, a| a[0].into_ret());
        for callee in [0x008d_7140, 0x0047_0470, 0x0043_00a0, 0x0043_5de0] {
            e.register(callee, |_, _| Ret::default());
        }
        e.mem.set_u32(0x0106_73a0, 0);
        let cause = e.new_object::<ActorCause>();
        e.call_log = Some(vec![]);
        e.call(0x0066_e440, &args![cause, 0x9000u32]);
        let log = e.call_log.take().unwrap();
        let base = cause.addr();
        assert_eq!(
            calls_to(&log, LOAD_DATA),
            vec![vec![0x9000, base + 8, 0xc], vec![0x9000, base + 0x14, 4]]
        );
        assert_eq!(calls_to(&log, LOAD_FORM_ID), vec![vec![0x9000, base]]);
        let list = e.get(cause, ActorCause::pWitnesses);
        assert_ne!(list, 0);
        assert_eq!(calls_to(&log, 0x008d_7140).len(), 2);
        let appended = calls_to(&log, LIST_APPEND);
        assert_eq!(appended.len(), 2);
        assert!(appended.iter().all(|w| w[0] == list));
        // With a list it is cleared instead.
        e.call_log = Some(vec![]);
        e.call(0x0066_e440, &args![cause, 0x9000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0047_0470), vec![vec![list]]);
    }

    #[test]
    fn load_actor_causes_list_makes_and_loads_each_cause() {
        let mut e = list_engine();
        e.register(LOAD_DATA, |_, _| Ret::default());
        e.register(LOAD_FORM_ID, |_, _| Ret::default());
        e.register_double(LOAD_VARIABLE_SIZED_VALUE, {
            let mut calls = 0;
            move |_, _| {
                calls += 1;
                u32::from(calls == 1).wrapping_mul(2).into_ret()
            }
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0066_e750, &args![0x9000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOAD_DATA)[0],
            vec![0x9000, CAUSE_ID_COUNTER, 4]
        );
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x1c], vec![0x1c]]);
        assert_eq!(calls_to(&log, LOAD_FORM_ID).len(), 2);
    }

    #[test]
    fn init_load_game_resolves_the_actor_and_witnesses() {
        let mut e = list_engine();
        e.register(0x0048_39c0, |_, a| (a[0] + 0x100).into_ret());
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.register(0x008d_7220, |_, _| Ret::default());
        let cause = e.new_object::<ActorCause>();
        e.set(cause, ActorCause::pActor, 0x14);
        let witnesses = e.mem.alloc(8);
        e.mem.set_u32(witnesses, 0x7000);
        e.set(cause, ActorCause::pWitnesses, witnesses);
        e.call_log = Some(vec![]);
        e.call(0x0066_e640, &args![cause, 0u32]);
        assert_eq!(e.get(cause, ActorCause::pActor), 0x114);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x00ec_43fb),
            vec![vec![0x114, 0, 0x0118_3028, 0x0118_46d4, 0]]
        );
        assert_eq!(calls_to(&log, 0x008d_7220), vec![vec![0x7000]]);
        e.set(cause, ActorCause::pActor, 0);
        e.call_log = Some(vec![]);
        e.call(0x0066_e640, &args![cause, 0u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0048_39c0).is_empty());
    }

    #[test]
    fn init_load_list_keeps_referenced_causes_and_drops_the_rest() {
        let mut e = list_engine();
        e.register(0x0047_d3d0, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(0x0048_39c0, |_, a| a[0].into_ret());
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.register(0x0042_ce10, |_, _| true.into_ret());
        e.mem.set_u32(GAME_STATE, 0);
        let kept = e.new_object::<ActorCause>();
        e.set(kept, ActorCause::iRefCount, 1);
        let dropped = e.new_object::<ActorCause>();
        let node2 = e.mem.alloc(8);
        e.mem.set_u32(node2, dropped.addr());
        e.mem.set_u32(CAUSE_LIST, kept.addr());
        e.mem.set_u32(CAUSE_LIST + 4, node2);
        // Removing after a node unlinks the next one.
        e.register(LIST_REMOVE_ITEM, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0066_e810, &args![0x33u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(dropped, ActorCause::iActorCauseID), -1);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![dropped.addr()]]);
        assert_eq!(
            calls_to(&log, LIST_REMOVE_ITEM),
            vec![vec![CAUSE_LIST, node2]]
        );
        assert_eq!(e.mem.u32(CAUSE_LIST + 4), 0);
    }
}
