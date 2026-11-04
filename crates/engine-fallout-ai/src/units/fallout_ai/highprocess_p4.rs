//! `fallout/ai/highprocess.cpp` (Xbox PDB source unit), part 4: its functions from `008f3fe0` up to
//! (not including) `00903160` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::highprocess`]; anything public there may be used here.
//!
//! The compiler's exception-unwinding frames (`FS:[0]` chains) are not
//! translated anywhere in this file. Offsets of `HighProcess` and its bases
//! are those of the Xbox PDB (they agree with the PC code); fields the main
//! file's layout does not declare are read at their offset with a comment.

#[allow(unused_imports)]
use super::highprocess::*;
#[allow(unused_imports)]
use crate::prelude::*;

// ---- Callees and globals shared by the functions below -------------------------------

/// `operator new(size)` (`00401000`, cdecl) and `operator delete(block)`
/// (`00401030`, cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// A `BSSimpleList` node's own address (`006815c0`, returns `this`): the item
/// is the word at the result.
const LIST_ITEM_SLOT: u32 = 0x0068_15c0;
/// A `BSSimpleList` node's next pointer (`00726070`, the word at `+4`).
const LIST_NEXT: u32 = 0x0072_6070;
/// Appends the item whose address is passed: `(list, &item)` (`005ae3d0`).
const LIST_APPEND: u32 = 0x005a_e3d0;
/// Removes the first node's item by moving the second node's content into the
/// head and freeing the second node (`0063f7b0`, `this` is the head node).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// Removes the first node holding the item whose address is passed:
/// `(list, &item)` (`00905330`).
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// Empties a list (`00470470`, `this` is the list).
const LIST_CLEAR: u32 = 0x0047_0470;
/// `BSSimpleList::BSSimpleList` (`0096a2d0`, returns its object).
const LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
/// `DetectionState::DetectionState` (`0066e590`, 0x24 bytes, the engine map
/// names it `BSSimpleList<DetectionState_P>::AddHead`).
const DETECTION_STATE_CONSTRUCTOR: u32 = 0x0066_e590;
/// `CombatTimeStamp::CombatTimeStamp(float)` (`00435de0`, one stack word;
/// stores the time in the object) and the game clock (`00435dd0`, `float` in
/// ST0).
const TIME_STAMP_CONSTRUCTOR: u32 = 0x0043_5de0;
const CLOCK: u32 = 0x0043_5dd0;
/// Reads the `float` a `CombatTimeStamp` holds (`006a7f50`, returned in ST0).
const TIME_STAMP_VALUE: u32 = 0x006a_7f50;
/// The default `NiPoint3` the constructors copy (three words) and the
/// `FLT_MAX` constant.
const DEFAULT_POINT: u32 = 0x011f_426c;
const MAX_FLOAT: u32 = 0x0108_7820;
/// Virtual slot `0x504` of the process: `HighProcess::GetDetectionState`
/// (`008f6650`), `(actor, which list)`.
const PROCESS_SLOT_GET_DETECTION_STATE: u32 = 0x504;
/// Slot `0x1f4` of a reference: its location on reference (a pointer to
/// three `float`s; `Actor::GetLocationOnReference`).
const REFERENCE_SLOT_LOCATION: u32 = 0x1f4;
/// Slot `0x100` of a reference: true for an actor (all `Actor` classes share
/// one body), and slot `0x218`, a second boolean test (`008d0360`).
const REFERENCE_SLOT_IS_ACTOR: u32 = 0x100;
const REFERENCE_SLOT_IS_CREATURE: u32 = 0x218;
/// Slot `0x344` of an actor: the disposition towards another reference
/// (`GetEmotionsDispostion` passes `(reference, 0)`).
const ACTOR_SLOT_DISPOSITION: u32 = 0x344;
/// Slot `0x27c` of the process: the package it is running
/// (`MiddleHighProcess::GetPackageThatIsRunning`); slot `0x30c`: the
/// greeting flag (`HighProcess::GetGreetingFlag`).
const PROCESS_SLOT_RUNNING_PACKAGE: u32 = 0x27c;
const PROCESS_SLOT_GREETING_FLAG: u32 = 0x30c;

/// `HighProcess` list fields read at their Xbox PDB offsets.
const GROUPS_TO_HELP_LIST: u32 = 0x27c;
const THREAD_DETECT_LIST: u32 = 0x268;
const TEMP_WHO_DETECTS_ME_LIST: u32 = 0x26c;
const DETECTED_ACTOR_LIST: u32 = 0x25c;
const WHO_DETECTS_ME_LIST: u32 = 0x260;
/// `LowProcess::pTarget` (`TESObjectREFR*`).
const TARGET: u32 = 0x40;
/// `MiddleHighProcess::lastSeenPostion` (`NiPoint3`).
const LAST_SEEN_POSITION: u32 = 0xfc;

/// The item of a list node: `*LIST_ITEM_SLOT(node)`.
fn list_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
    e.mem.u32(slot)
}

/// The next node of a list node.
fn list_next(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NEXT, &args![node]).u32()
}

/// A new `DetectionState` (`operator new(0x24)` and its constructor; null
/// when the allocation fails, as the game's test does).
fn new_detection_state(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(DETECTION_STATE_CONSTRUCTOR, &args![block]).u32()
    }
}

/// The process's `DetectionState` for `(actor, which)` through its virtual
/// slot `0x504`.
fn detection_state(e: &mut Engine, this: Ptr, actor: u32, which: u32) -> u32 {
    e.vcall(
        this.addr(),
        PROCESS_SLOT_GET_DETECTION_STATE,
        &args![actor, which],
    )
    .u32()
}

/// Appends `item` to `list` (the list function takes the item's address).
fn list_append(e: &mut Engine, list: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_APPEND, &args![list, slot]);
    });
}

/// [`list_append`] with the caller's local `slot` holding the item.
fn list_append_at(e: &mut Engine, list: u32, item: u32, slot: u32) {
    e.mem.set_u32(slot, item);
    e.call(LIST_APPEND, &args![list, slot]);
}

/// `MiddleHighProcess::GetSavedAcquireObject` in the engine map, but the
/// body (shared by folding) returns the word at +0x68 of an actor: its
/// process (`008d8520`).
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// The type code of a package or object (`0041ca90`).
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// `TESPackage::GetPackageTarget` (`00671d10`: the word at +0x30).
const PACKAGE_GET_TARGET: u32 = 0x0067_1d10;
/// The 3D node of a reference (`0043fcd0`, also its slot `0x1d0`).
const GET_3D_NODE: u32 = 0x0043_fcd0;
/// The `Calendar` singleton (`011de7b8`) and its `GetHour` (`00867da0`) and
/// `GetTimeScale` (`00867950`).
const CALENDAR: u32 = 0x011d_e7b8;
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
const CALENDAR_GET_TIME_SCALE: u32 = 0x0086_7950;
/// An actor's extra data list (`005d43c0`: `this + 0x44`).
const EXTRA_DATA_LIST: u32 = 0x005d_43c0;

/// Number of entries of a list (`005ae380`, `VATS::GetCount` in the map, a
/// folded body).
const LIST_COUNT: u32 = 0x005a_e380;
/// The breath timer default the constructor and `Revert` copy (`01017868`).
const DEFAULT_BREATH_TIMER: u32 = 0x0101_7868;
/// `HighProcess::HeadTrackTargets`: six `TESObjectREFR*` at +0x3f8, with
/// their six flag bytes at +0x410.
const HEAD_TRACKING_TARGETS: u32 = 0x3f8;
const HEAD_TRACKING_TARGET_FLAGS: u32 = 0x410;
/// The word at +0x14 of an object (`00825c00`, the same body as the save
/// buffer position getter).
const WORD_AT_0X14: u32 = 0x0082_5c00;

/// Slot `0x428` of an actor (`008a02d0`): the package its process reports
/// through process slot `0x22c`, if that has type code `0x12`, else null.
const ACTOR_SLOT_PACKAGE_OF_TYPE_0X12: u32 = 0x428;

// Translated from 008f3fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: calls `LowProcess::ProcessAcquire(this, actor)`
/// (`00908f70`) and returns true. The second word the caller pushes (the
/// frame time) is not read.
pub fn fn_008f3fe0(e: &mut Engine, this: Ptr, actor: u32, _unused_0: u32) -> bool {
    e.call(0x0090_8f70, &args![this, actor]);
    true
}

// Translated from 008f4000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: calls `LowProcess::ProcessPatrol(this, actor)`
/// (`009121e0`) and returns true. The second word the caller pushes (the
/// frame time) is not read.
pub fn fn_008f4000(e: &mut Engine, this: Ptr, actor: u32, _unused_0: u32) -> bool {
    e.call(0x0091_21e0, &args![this, actor]);
    true
}

// Translated from 008f5410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether an entry of the process's
/// `GroupsToHelpList` (`BSSimpleList<StartCombatStates *>` at +0x27c, an
/// inline head node) has `pGroup` (+4) equal to `group`. The walk stops at the
/// first match or at a node without an item.
pub fn fn_008f5410(e: &mut Engine, this: Ptr, group: u32) -> bool {
    let mut found = false;
    let mut node = this.addr() + GROUPS_TO_HELP_LIST;
    while node != 0 {
        let entry = list_item(e, node);
        if entry == 0 || found {
            break;
        }
        // StartCombatStates::pGroup (Xbox PDB) +0x04
        if e.mem.u32(entry + 4) == group {
            found = true;
        }
        node = list_next(e, node);
    }
    found
}

// Translated from 008f6120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StartCombatStates` constructor (Xbox PDB `StartCombatStates`, size
/// 0x14; the engine map's `BSSimpleList<StartCombatStates_P>::AddHead` is the
/// same body): clears `pTarget`, `pGroup`, the six flag bytes
/// (`bMustEntercombat`, `bSendAlarm`, `bForceFlee`, `bFleeing`,
/// `bIgnoreDefensivePackage`, `bSpectatorOfCombat`) and `iPriority`. Returns
/// `this`.
pub fn fn_008f6120(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    for offset in [0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d] {
        e.mem.set_u8(base + offset, 0);
    }
    e.mem.set_u32(base + 0x10, 0);
    this
}

// Translated from 008f6180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetWhoDetectsMe` (Xbox PDB): a new list holding a copy
/// (nine words) of every `DetectionState` of `pActorsWhoDetectMeList`
/// (+0x260). The word the caller pushes is not read.
pub fn high_process_get_who_detects_me(e: &mut Engine, this: Ptr, _unused_0: u32) -> u32 {
    let mut node = e.mem.u32(this.addr() + WHO_DETECTS_ME_LIST);
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let result = if block == 0 {
        0
    } else {
        e.call(LIST_CONSTRUCTOR, &args![block]).u32()
    };
    while node != 0 {
        let source = list_item(e, node);
        if source == 0 {
            break;
        }
        node = list_next(e, node);
        let copy = new_detection_state(e);
        for word in 0..9 {
            let value = e.mem.u32(source + 4 * word);
            e.mem.set_u32(copy + 4 * word, value);
        }
        list_append(e, result, copy);
    }
    result
}

// Translated from 008f62b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::InsertIntoDetectionList` (Xbox PDB): builds a
/// `DetectionState` for `actor` and appends it to the thread list (+0x268;
/// `bEvaluated` is cleared) when `detects_me` is 0, else to the temporary
/// actors-who-detect-me list (+0x26c). It records the detection level word
/// (+4), the line-of-sight byte (+0x1e), `iLevel` (+8), `b360view` (+0x1c)
/// and `bcombat` (+0x1d); when `level` is above zero it also stores the
/// actor's location (slot `0x1f4`) and the current game time as a
/// `CombatTimeStamp`. Returns the state.
#[allow(clippy::too_many_arguments)]
pub fn high_process_insert_into_detection_list(
    e: &mut Engine,
    this: Ptr,
    actor: Ptr,
    detection_level: u32,
    line_of_sight: u8,
    level: i32,
    detects_me: i32,
    view_360: u8,
    combat: u8,
) -> Ptr<DetectionState> {
    let state = new_detection_state(e);
    if detects_me == 0 {
        e.mem.set_u8(state + 0x1f, 0);
        let list = e.mem.u32(this.addr() + THREAD_DETECT_LIST);
        list_append(e, list, state);
    } else {
        let list = e.mem.u32(this.addr() + TEMP_WHO_DETECTS_ME_LIST);
        list_append(e, list, state);
    }
    e.mem.set_u32(state + 4, detection_level);
    e.mem.set_u32(state, actor.addr());
    e.mem.set_u8(state + 0x1e, line_of_sight);
    e.mem.set_i32(state + 8, level);
    e.mem.set_u8(state + 0x1c, view_360);
    e.mem.set_u8(state + 0x1d, combat);
    if level > 0 {
        let location = e
            .vcall(actor.addr(), REFERENCE_SLOT_LOCATION, &args![])
            .u32();
        for word in 0..3 {
            let value = e.mem.u32(location + 4 * word);
            e.mem.set_u32(state + 0xc + 4 * word, value);
        }
        let now = e.call(CLOCK, &args![]).f32();
        let stamp = e.with_stack(4, |e, slot| {
            e.call(TIME_STAMP_CONSTRUCTOR, &args![slot, now]);
            e.mem.u32(slot.addr())
        });
        e.mem.set_u32(state + 0x18, stamp);
    }
    Ptr::new(state)
}

// Translated from 008f63e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: merges the two detection lists gathered by a
/// detection pass into the process's lists. Each state of the thread list
/// (+0x268) is copied (`008d6fc0`) into the `DetectionState` that
/// `GetDetectionState(actor, 0)` finds, or into a new one appended to
/// `pDetectedActorList` (+0x25c); the thread state is then freed and the list
/// emptied. If any state was merged, `owner`'s virtual slot `0x428` is called
/// and, when it returns an object, `008f6630(object, 1)` runs. The temporary
/// list (+0x26c) is merged the same way into `pActorsWhoDetectMeList`
/// (+0x260) using `GetDetectionState(actor, 1)`. The calls of the no-op
/// destructor `00483710` on a static object (`011e0245`) at entry and exit
/// are left out.
pub fn fn_008f63e0(e: &mut Engine, this: Ptr, owner: Ptr) {
    let mut merged = false;
    let mut node = e.mem.u32(this.addr() + THREAD_DETECT_LIST);
    while node != 0 {
        let source = list_item(e, node);
        if source == 0 {
            break;
        }
        let actor = e.mem.u32(source);
        let mut target = detection_state(e, this, actor, 0);
        if target == 0 {
            target = new_detection_state(e);
            let list = e.mem.u32(this.addr() + DETECTED_ACTOR_LIST);
            list_append(e, list, target);
        }
        e.call(0x008d_6fc0, &args![target, source]);
        merged = true;
        e.call(OPERATOR_DELETE, &args![source]);
        node = list_next(e, node);
    }
    let list = e.mem.u32(this.addr() + THREAD_DETECT_LIST);
    e.call(LIST_CLEAR, &args![list]);
    if merged {
        let package = e
            .vcall(owner.addr(), ACTOR_SLOT_PACKAGE_OF_TYPE_0X12, &args![])
            .u32();
        if package != 0 {
            e.call(0x008f_6630, &args![package, 1u32]);
        }
    }
    node = e.mem.u32(this.addr() + TEMP_WHO_DETECTS_ME_LIST);
    while node != 0 {
        let source = list_item(e, node);
        if source == 0 {
            break;
        }
        let actor = e.mem.u32(source);
        let mut target = detection_state(e, this, actor, 1);
        if target == 0 {
            target = new_detection_state(e);
            let list = e.mem.u32(this.addr() + WHO_DETECTS_ME_LIST);
            list_append(e, list, target);
        }
        e.call(0x008d_6fc0, &args![target, source]);
        e.call(OPERATOR_DELETE, &args![source]);
        node = list_next(e, node);
    }
    let list = e.mem.u32(this.addr() + TEMP_WHO_DETECTS_ME_LIST);
    e.call(LIST_CLEAR, &args![list]);
}

// Translated from 008f6630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores its argument byte at +0xc5 of `this`
/// (the package object `008f63e0` passes it).
pub fn fn_008f6630(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xc5, value);
}

// Translated from 008f6650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetDetectionState` (Xbox PDB): the first `DetectionState`
/// of `pDetectedActorList` (+0x25c, when `which` is 0) or of
/// `pActorsWhoDetectMeList` (+0x260, otherwise) whose `pActor` is `actor`;
/// null when there is none.
pub fn high_process_get_detection_state(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    which: i32,
) -> Ptr<DetectionState> {
    let mut found = 0;
    let mut node = if which == 0 {
        e.mem.u32(this.addr() + DETECTED_ACTOR_LIST)
    } else {
        e.mem.u32(this.addr() + WHO_DETECTS_ME_LIST)
    };
    while node != 0 {
        let state = list_item(e, node);
        if state == 0 || found != 0 {
            break;
        }
        if e.mem.u32(state) == actor {
            found = state;
        }
        node = list_next(e, node);
    }
    Ptr::new(found)
}

// Translated from 008f66e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::RemoveDetectionActor` (Xbox PDB): removes (and frees) every
/// `DetectionState` for `actor` from the list `which` names: 0 is
/// `pDetectedActorList`, 1 is `pActorsWhoDetectMeList`, 3 is both, the
/// first list and then, once it is exhausted, the second.
pub fn high_process_remove_detection_actor(e: &mut Engine, this: Ptr, actor: u32, which: i32) {
    let mut node = 0;
    let mut previous = 0;
    let mut both = false;
    if which == 0 || which == 3 {
        node = e.mem.u32(this.addr() + DETECTED_ACTOR_LIST);
    } else if which == 1 {
        node = e.mem.u32(this.addr() + WHO_DETECTS_ME_LIST);
    }
    if which == 3 {
        if node != 0 && list_item(e, node) != 0 {
            both = true;
        } else {
            previous = 0;
            node = e.mem.u32(this.addr() + WHO_DETECTS_ME_LIST);
        }
    }
    while node != 0 {
        let state = list_item(e, node);
        if state == 0 {
            break;
        }
        if e.mem.u32(state) == actor {
            if previous != 0 {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), state);
                    e.call(LIST_REMOVE_ITEM, &args![previous, slot]);
                });
                node = list_next(e, previous);
            } else {
                e.call(LIST_REMOVE_HEAD, &args![node]);
            }
            e.call(OPERATOR_DELETE, &args![state]);
        } else {
            previous = node;
            node = list_next(e, node);
        }
        if (node == 0 || list_item(e, node) == 0) && both {
            previous = 0;
            node = e.mem.u32(this.addr() + WHO_DETECTS_ME_LIST);
            both = false;
        }
    }
}

// Translated from 008f6820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetLastPositionDetected` (Xbox PDB): writes the
/// `LastPositionDetected` (+0xc) of the `DetectionState` for `(actor, which)`
/// to `out`, or the default point (`011f426c`) when there is none. Returns
/// `out`.
pub fn high_process_get_last_position_detected(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    actor: u32,
    which: u32,
) -> Ptr {
    let mut point = [
        e.mem.u32(DEFAULT_POINT),
        e.mem.u32(DEFAULT_POINT + 4),
        e.mem.u32(DEFAULT_POINT + 8),
    ];
    let state = detection_state(e, this, actor, which);
    if state != 0 {
        for (word, slot) in point.iter_mut().enumerate() {
            *slot = e.mem.u32(state + 0xc + 4 * word as u32);
        }
    }
    for (word, value) in point.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * word as u32, *value);
    }
    out
}

// Translated from 008f68a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetLastTimeDetected` (Xbox PDB): the time stamp
/// (`fLastTimeDetected`, +0x18, read through `006a7f50`) of the
/// `DetectionState` for `(actor, which)`, or `-FLT_MAX` when there is none.
pub fn high_process_get_last_time_detected(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    which: u32,
) -> f32 {
    let mut result = -e.global::<f32>(MAX_FLOAT);
    let state = detection_state(e, this, actor, which);
    if state != 0 {
        result = e.call(TIME_STAMP_VALUE, &args![state + 0x18]).f32();
    }
    result
}

// Translated from 008f68f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::Get360LineSightonActor` (Xbox PDB): `b360view` (+0x1c) of
/// the `DetectionState` for `(actor, which)`, 0 when there is none. The first
/// word the caller pushes is not read.
pub fn high_process_get_360_line_sighton_actor(
    e: &mut Engine,
    this: Ptr,
    _unused_0: u32,
    actor: u32,
    which: u32,
) -> u8 {
    let state = detection_state(e, this, actor, which);
    if state == 0 {
        0
    } else {
        e.mem.u8(state + 0x1c)
    }
}

// Translated from 008f6930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetLineSightonActor` (Xbox PDB): for the `DetectionState`
/// of `(actor, which)`, `b360view` (+0x1c) when `use_360_view` is non-zero,
/// else `bLineofSight` (+0x1e); 0 when there is none. The first word the
/// caller pushes is not read.
pub fn high_process_get_line_sighton_actor(
    e: &mut Engine,
    this: Ptr,
    _unused_0: u32,
    actor: u32,
    which: u32,
    use_360_view: u8,
) -> u8 {
    let state = detection_state(e, this, actor, which);
    if state == 0 {
        0
    } else if use_360_view == 0 {
        e.mem.u8(state + 0x1e)
    } else {
        e.mem.u8(state + 0x1c)
    }
}

// Translated from 008f6990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetDetectionActor` (Xbox PDB): `iLevel` (+8) of the
/// `DetectionState` for `(actor, which)`, `0x7fffffff` when there is none.
pub fn high_process_get_detection_actor(e: &mut Engine, this: Ptr, actor: u32, which: u32) -> i32 {
    let state = detection_state(e, this, actor, which);
    if state == 0 {
        0x7fff_ffff
    } else {
        e.mem.i32(state + 8)
    }
}

// Translated from 008f6ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetLastSeenLocation` (Xbox PDB): when `pTarget` (+0x40) is
/// set, copies its location (slot `0x1f4`, three words) to
/// `lastSeenPostion` (+0xfc).
pub fn high_process_set_last_seen_location(e: &mut Engine, this: Ptr) {
    let target = e.mem.u32(this.addr() + TARGET);
    if target != 0 {
        let location = e.vcall(target, REFERENCE_SLOT_LOCATION, &args![]).u32();
        for word in 0..3 {
            let value = e.mem.u32(location + 4 * word);
            e.mem
                .set_u32(this.addr() + LAST_SEEN_POSITION + 4 * word, value);
        }
    }
}

// Translated from 008f6ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetEmotionsDispostion` (Xbox PDB): the disposition
/// (`subject` slot `0x344`, called with `(reference, 0)`) of `subject`
/// towards `other`; 0x32 when neither case below applies. When `subject`
/// passes its slot-`0x218` test and no `other` is given, `pTarget` (+0x40) is
/// the reference if it is an actor (slot `0x100`) and `00440da0` is false for
/// it. When `other` is given and is an actor, it is the reference.
pub fn high_process_get_emotions_dispostion(
    e: &mut Engine,
    this: Ptr,
    subject: u32,
    other: u32,
) -> u32 {
    let mut result = 0x32;
    if e.vcall(subject, REFERENCE_SLOT_IS_CREATURE, &args![])
        .bool()
        && other == 0
    {
        let target = e.mem.u32(this.addr() + TARGET);
        if target != 0
            && e.vcall(target, REFERENCE_SLOT_IS_ACTOR, &args![]).bool()
            && !e.call(0x0044_0da0, &args![target]).bool()
        {
            return e
                .vcall(subject, ACTOR_SLOT_DISPOSITION, &args![target, 0u32])
                .u32();
        }
    }
    if other != 0 && e.vcall(other, REFERENCE_SLOT_IS_ACTOR, &args![]).bool() {
        result = e
            .vcall(subject, ACTOR_SLOT_DISPOSITION, &args![other, 0u32])
            .u32();
    }
    result
}

// Translated from 008f6fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::IsTalking` (Xbox PDB): with an `actor` that
/// `MobileObject::IsinDialogue` (`009336c0`) accepts, true when the package
/// the process runs (slot `0x27c`) has type code `0x1c` (`0041ca90`) and its
/// word at +0x94 is `actor`. Otherwise true when the process's sound handle
/// (+0x314) is valid (`BSSoundHandle::IsValid`) or its greeting flag (slot
/// `0x30c`) is set.
pub fn high_process_is_talking(e: &mut Engine, this: Ptr, actor: u32) -> bool {
    if actor != 0 && e.call(0x0093_36c0, &args![actor]).bool() {
        let package = e
            .vcall(this.addr(), PROCESS_SLOT_RUNNING_PACKAGE, &args![])
            .u32();
        let mut dialogue_package = 0;
        if package != 0 && e.call(0x0041_ca90, &args![package]).i32() == 0x1c {
            dialogue_package = package;
        }
        dialogue_package != 0 && e.mem.u32(dialogue_package + 0x94) == actor
    } else {
        e.call(0x00ad_8ce0, &args![this.addr() + 0x314]).bool()
            || e.vcall(this.addr(), PROCESS_SLOT_GREETING_FLAG, &args![])
                .bool()
    }
}

// Translated from 008f7710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetAlphaMult` (Xbox PDB): `fAlphaMult` at +0x170.
pub fn middle_high_process_get_alpha_mult(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x170)
}

// Translated from 008f9100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: returns the byte at +0xe of `this` (a field of
/// the `TESPackage` use-weapon data that `ProcessUseWeapon` reads).
pub fn fn_008f9100(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xe)
}

// Translated from 008f9120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: returns the byte at +0xf of `this` (a field of
/// the use-weapon data that `ProcessUseWeapon` reads).
pub fn fn_008f9120(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xf)
}

// Translated from 008f9140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: returns the 16-bit word at +0x14 of `this` (a
/// field of the use-weapon data that `ProcessUseWeapon` reads).
pub fn fn_008f9140(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x14)
}

// Translated from 008f9160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: returns the 16-bit word at +0x16 of `this` (a
/// field of the use-weapon data that `ProcessUseWeapon` reads).
pub fn fn_008f9160(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x16)
}

// Translated from 008fdb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `DetectionEvent` constructor (Xbox PDB
/// `DetectionEvent`, size 0x1c, with `iActionValue`, `Location`,
/// `fTimeStamp`, `eEventType` and `pRef`): the action value is 0, the
/// location the default point (`011f426c`), the time stamp `CombatTimeStamp(-FLT_MAX)`,
/// the event type -1 and the reference null. Returns `this`.
pub fn fn_008fdb10(e: &mut Engine, this: Ptr<DetectionEvent>) -> Ptr<DetectionEvent> {
    let base = this.addr();
    e.call(LIST_ITEM_SLOT, &args![base + 4]);
    e.call(0x0043_00a0, &args![base + 0x10]);
    e.mem.set_u32(base, 0);
    for word in 0..3 {
        let value = e.mem.u32(DEFAULT_POINT + 4 * word);
        e.mem.set_u32(base + 4 + 4 * word, value);
    }
    let never = -e.global::<f32>(MAX_FLOAT);
    let stamp = e.with_stack(4, |e, slot| {
        e.call(TIME_STAMP_CONSTRUCTOR, &args![slot, never]);
        e.mem.u32(slot.addr())
    });
    e.mem.set_u32(base + 0x10, stamp);
    e.mem.set_u32(base + 0x14, 0xffff_ffff);
    e.mem.set_u32(base + 0x18, 0);
    this
}

/// The global holding the `PlayerCharacter` pointer (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `Actor::GetCurrentWeapon` (`008a1710`, `this` = actor).
const GET_CURRENT_WEAPON: u32 = 0x008a_1710;
/// `Setting::GetFloat`-style accessor (`00403e20`, `this` = a game setting;
/// returns the address of its `float`).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// Process slots: `0x148` the current weapon (`ItemChange*`), `0x1b8` the
/// animation, `0x58` clear the face animation data, `0x470` clear the 3D
/// update flags, `0x478` read them, `0x580` set the shader-effect refresh
/// flags, `0x628` set the action head-track target, `0x654` clear the
/// use-weapon head-track target, `0x294` end the move message, `0x394`
/// free up the special idle, `0x794`/`0x79c`/`0x7a4` set the face node, the
/// skinned face node and the head animations, `0x280` the running procedure
/// index.
const PROCESS_SLOT_CURRENT_WEAPON: u32 = 0x148;
const PROCESS_SLOT_ANIMATION: u32 = 0x1b8;
const PROCESS_SLOT_CLEAR_FACE_ANIMATION_DATA: u32 = 0x58;
const PROCESS_SLOT_CLEAR_3D_UPDATE_FLAGS: u32 = 0x470;
const PROCESS_SLOT_3D_UPDATE_FLAGS: u32 = 0x478;
const PROCESS_SLOT_REFRESH_MAGIC_SHADERS: u32 = 0x580;
const PROCESS_SLOT_SET_ACTION_HEAD_TRACK_TARGET: u32 = 0x628;
const PROCESS_SLOT_CLEAR_USE_WEAPON_HEAD_TRACK_TARGET: u32 = 0x654;
const PROCESS_SLOT_END_MOVE_MESSAGE: u32 = 0x294;
const PROCESS_SLOT_FREE_UP_SPECIAL_IDLE: u32 = 0x394;
const PROCESS_SLOT_SET_FACE_NODE: u32 = 0x794;
const PROCESS_SLOT_SET_FACE_SKINNED_NODE: u32 = 0x79c;
const PROCESS_SLOT_SET_HEAD_ANIMS: u32 = 0x7a4;
const PROCESS_SLOT_PROCEDURE_INDEX: u32 = 0x280;
/// Actor (reference) slots: `0x1d0` the 3D node, `0x21c` a boolean test,
/// `0x1b0`/`0x1ac` the head and torso nodes (take a flag), `0x210` set an
/// object flag, `0x22c`, `0x384`, `0x1c0` and `0xd4` as used below.
const REFERENCE_SLOT_GET_3D: u32 = 0x1d0;
const REFERENCE_SLOT_TEST_0X21C: u32 = 0x21c;
const REFERENCE_SLOT_HEAD_NODE: u32 = 0x1b0;
const REFERENCE_SLOT_TORSO_NODE: u32 = 0x1ac;
/// The table of procedure types the follow check indexes
/// (`011a3ff0`, pointers to word tables).
const PROCEDURE_TABLE: u32 = 0x011a_3ff0;

/// The float a game setting holds (`00403e20(setting)` gives its address).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

// Translated from 008f7070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::CreateFollownoEscort` (Xbox PDB): gives `actor` a follow
/// package aimed at the player. It runs when the running package (slot
/// `0x27c`) is missing, when the actor's own process has its greeting flag
/// set (slot `0x30c`), when the package passes `008840f0` and `008840d0`, or
/// when the package's procedure table entry (the table at `011a3ff0`,
/// indexed by the package's word at +0x18 and the running procedure index,
/// slot `0x280`) is 1 for a package of type 5 or 0x36 for one of type 6 or 0.
/// It then clears the actor's slot-`0x214` object through slot `0x418`,
/// makes a package of type 1 (`TESPackage::CreatePackage`) with a new
/// `PackageTarget` (type 0, the player), a count of 3000 when `long_range`
/// is set and 80 otherwise, copies the second generic location and a flag
/// from the running package, adds it to the actor (slot `0x2f4`), and sets
/// the process's action head-track target to the player. With `long_range`
/// it also loads `fAwarePlayerTimer` (+0x34c) from the setting at `011cd210`.
pub fn high_process_create_followno_escort(e: &mut Engine, this: Ptr, actor: Ptr, long_range: u8) {
    let mut follow = false;
    let package = e
        .vcall(this.addr(), PROCESS_SLOT_RUNNING_PACKAGE, &args![])
        .u32();
    if package != 0 {
        let actor_process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        let greeting = e
            .vcall(actor_process, PROCESS_SLOT_GREETING_FLAG, &args![])
            .bool();
        if !greeting {
            if e.call(0x0088_40f0, &args![package]).bool()
                && e.call(0x0088_40d0, &args![package]).bool()
            {
                follow = true;
            } else if e.call(PACKAGE_TYPE, &args![package]).i32() == 5 {
                let kind = e.call(0x0096_11e0, &args![package]).u32();
                let index = e
                    .vcall(this.addr(), PROCESS_SLOT_PROCEDURE_INDEX, &args![])
                    .u32();
                let row = e.mem.u32(PROCEDURE_TABLE + 4 * kind);
                if e.mem.u32(row + 4 * index) == 1 {
                    follow = true;
                }
            } else if e.call(PACKAGE_TYPE, &args![package]).i32() == 6
                || e.call(PACKAGE_TYPE, &args![package]).i32() == 0
            {
                let kind = e.call(0x0096_11e0, &args![package]).u32();
                let index = e
                    .vcall(this.addr(), PROCESS_SLOT_PROCEDURE_INDEX, &args![])
                    .u32();
                let row = e.mem.u32(PROCEDURE_TABLE + 4 * kind);
                if e.mem.u32(row + 4 * index) == 0x36 {
                    follow = true;
                }
            }
        } else {
            follow = true;
        }
    } else {
        follow = true;
    }
    if !follow {
        return;
    }
    if e.vcall(actor.addr(), 0x214, &args![]).u32() != 0 {
        e.vcall(actor.addr(), 0x418, &args![]);
    }
    let created = e.call(0x0067_0b90, &args![1u32]).u32();
    e.call(0x0067_0fc0, &args![created, 1u32]);
    e.call(0x0082_6b40, &args![created, 1u32]);
    e.call(0x0082_6b90, &args![created, 1u32]);
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let target = if block == 0 {
        0
    } else {
        e.call(0x0067_ff70, &args![block]).u32()
    };
    e.call(0x0067_2fc0, &args![created, target]);
    if target != 0 {
        e.call(0x007b_3fa0, &args![target, 1u32]);
    }
    let package_target = e.call(PACKAGE_GET_TARGET, &args![created]).u32();
    e.call(0x0068_00b0, &args![package_target, 0u32]);
    let player = e.mem.u32(PLAYER);
    let package_target = e.call(PACKAGE_GET_TARGET, &args![created]).u32();
    e.call(0x0068_0110, &args![package_target, player]);
    let package_target = e.call(PACKAGE_GET_TARGET, &args![created]).u32();
    let count: u32 = if long_range != 0 { 3000 } else { 0x50 };
    e.call(0x0040_3550, &args![package_target, count]);
    if package != 0 {
        let second = e.call(0x0067_33e0, &args![package]).u32();
        e.call(0x0067_3400, &args![created, second]);
        let flag = e.call(0x0067_a460, &args![package]).u8();
        e.call(0x0067_1a20, &args![created, flag]);
    }
    e.call(0x0098_4f60, &args![created, 0x2du32]);
    e.vcall(actor.addr(), 0x2f4, &args![created, 1u32, 1u32]);
    let player = e.mem.u32(PLAYER);
    e.vcall(
        this.addr(),
        PROCESS_SLOT_SET_ACTION_HEAD_TRACK_TARGET,
        &args![player],
    );
    if long_range != 0 {
        let timer = setting_float(e, 0x011c_d210);
        e.mem.set_f32(this.addr() + 0x34c, timer);
    }
}

// Translated from 008f7350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::FinishDying` (Xbox PDB). `dead` starts as `00c6a440(3D node
/// of actor, 1, 0)`; if set, it becomes whether the hours since the process
/// last processed (+0x20, through `007df1f0`; wrapping past 24 hours: `last +
/// 23.0 - now` when the last hour is later than the clock hour) are less
/// than `Calendar::GetTimeScale * 0.0005`. When the process has an animation
/// (slot `0x1b8`) whose sequence `00491040(animation, 1)` is of group
/// `0xe0`, the animation is updated with the sequence length
/// (`00491090`) and the float at `01012054`. If not `dead`, the actor then
/// runs `DoDeathStuff`, slots `0x1c0` and `0xd4(1)`, and its extra data list
/// gets `SetRagDollData(actor)`.
pub fn high_process_finish_dying(e: &mut Engine, this: Ptr, actor: Ptr) {
    let node = e.call(GET_3D_NODE, &args![actor]).u32();
    let mut dead = e.call(0x00c6_a440, &args![node, 1u32, 0u32]).bool();
    if dead {
        let last_hour = e.call(0x007d_f1f0, &args![this]).f32();
        let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f32();
        let diff = if last_hour <= hour {
            (f64::from(hour) - f64::from(last_hour)) as f32
        } else {
            ((f64::from(last_hour) + e.global::<f64>(0x0103_5838)) - f64::from(hour)) as f32
        };
        let scale = f64::from(e.call(CALENDAR_GET_TIME_SCALE, &args![CALENDAR]).f32());
        let limit = scale * e.global::<f64>(0x0107_6f68);
        dead = f64::from(diff) < limit;
    }
    if e.vcall(this.addr(), PROCESS_SLOT_ANIMATION, &args![]).u32() != 0 {
        let animation = e.vcall(this.addr(), PROCESS_SLOT_ANIMATION, &args![]).u32();
        let sequence = e.call(0x0049_1040, &args![animation, 1u32]).u32();
        if sequence != 0 {
            let holder = e.call(0x0048_f7f0, &args![sequence]).u32();
            if e.call(0x005f_2420, &args![holder]).i32() == 0xe0 {
                let blend = e.global::<f32>(0x0101_2054);
                let length = e.call(0x0049_1090, &args![sequence]).f32();
                e.call(0x0049_1180, &args![animation, actor, length, blend]);
            }
        }
    }
    if !dead {
        e.call(0x008b_01c0, &args![actor]);
        e.vcall(actor.addr(), 0x1c0, &args![]);
        e.vcall(actor.addr(), 0xd4, &args![1u32]);
        let list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
        e.call(0x0041_d490, &args![list, actor]);
    }
}

// Translated from 008f74c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::AimAtTarget` (Xbox PDB): aims `attacker` at `target`;
/// returns false at once when there is no target. The aim point is the
/// target's firing location (`GetTargetFiringLocation(target, 1)`) when the
/// attacker's current weapon (`Actor::GetCurrentWeapon`) has no projectile
/// (`00525a90(weapon, 0)`), else the projected point for the target
/// (`GetProjectedPointForTarget`, with the projectile speed
/// `0069ef80` times 1.0 or, when `scaled` is set, the setting at `011cfb78`).
/// The attacker's firing location is computed (`005257a0` or
/// `GetAttackerFiringLocation`) but only stored in dead locals. The attacker
/// then looks at the point (`SetLookAtTarget`) and `bAimingTarget` (+0xe1) is
/// set. With a weapon for which `006450c0` is false, iron sights are set when
/// the squared range (`GetAlphaMult` of the weapon times the setting at
/// `011cebf8`) is below the squared distance (`004a7290`) between the
/// attacker's location (slot `0x1f4`) and the aim point, and the result is
/// `IsTargetWithinFiringArc(attacker, point, f, f)` with `f` the float at
/// `01088250`; otherwise it is the result of `009a6ae0(attacker, target, 0)`.
pub fn high_process_aim_at_target(
    e: &mut Engine,
    this: Ptr,
    attacker: Ptr,
    target: Ptr,
    scaled: u8,
) -> bool {
    if target.is_null() {
        return false;
    }
    let weapon = e.call(GET_CURRENT_WEAPON, &args![attacker]).u32();
    let projectile = if weapon != 0 {
        e.call(0x0052_5a90, &args![weapon, 0u32]).u32()
    } else {
        0
    };
    // The exe's frame (`SUB ESP,0x80` under `EBP`, `ESI`): 0x84 bytes. The firing location
    // lands at +0x3c (`EBP-0x48`) with a weapon or +0x30 (`EBP-0x54`) without; the aim point
    // comes out at +0x24 (`EBP-0x60`) with a projectile or +0x18 (`EBP-0x6c`) without and is
    // kept at +0x6c (`EBP-0x18`); the offset to it at +0x50 (`EBP-0x34`).
    e.with_stack(0x84, |e, frame| {
        aim_in_frame(
            e,
            this,
            attacker,
            target,
            scaled,
            weapon,
            projectile,
            frame.addr(),
        )
    })
}

/// [`high_process_aim_at_target`] past its first checks, in the exe's frame.
#[allow(clippy::too_many_arguments)]
fn aim_in_frame(
    e: &mut Engine,
    this: Ptr,
    attacker: Ptr,
    target: Ptr,
    scaled: u8,
    weapon: u32,
    projectile: u32,
    frame: u32,
) -> bool {
    // The attacker's firing location is only stored in locals nothing reads.
    if weapon != 0 {
        e.call(0x0052_57a0, &args![weapon, frame + 0x3c, attacker]);
    } else {
        e.call(0x009a_8050, &args![frame + 0x30, attacker]);
    }
    let point = frame + 0x6c;
    if projectile != 0 {
        let scale = if scaled != 0 {
            setting_float(e, 0x011c_fb78)
        } else {
            1.0
        };
        let speed = e.call(0x0069_ef80, &args![projectile]).f32();
        let speed = (f64::from(speed) * f64::from(scale)) as f32;
        let result = e
            .call(
                0x009a_8bf0,
                &args![
                    frame + 0x24,
                    attacker,
                    target,
                    0u32,
                    projectile,
                    speed,
                    scaled as u32
                ],
            )
            .u32();
        copy_point(e, result, point);
    } else {
        let result = e
            .call(0x009a_8600, &args![frame + 0x18, target, 1u32])
            .u32();
        copy_point(e, result, point);
    }
    let (x, y, z) = (e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8));
    e.call(0x008b_3cd0, &args![attacker, x, y, z]);
    e.mem.set_u8(this.addr() + 0xe1, 1);
    if weapon != 0 && !e.call(0x0064_50c0, &args![weapon]).bool() {
        let range = setting_float(e, 0x011c_ebf8);
        let alpha = middle_high_process_get_alpha_mult(e, Ptr::new(weapon));
        let reach = (f64::from(alpha) * f64::from(range)) as f32;
        let offset = frame + 0x50;
        let location = e
            .vcall(attacker.addr(), REFERENCE_SLOT_LOCATION, &args![])
            .u32();
        e.call(0x0043_9ef0, &args![location, offset, point]);
        let squared = e.call(0x004a_7290, &args![offset]).f32();
        let within = f64::from(reach) * f64::from(reach) < f64::from(squared);
        e.call(0x008b_b650, &args![attacker, within as u32, 0u32, 0u32]);
        let arc = e.global::<f32>(0x0108_8250);
        e.call(0x009a_6d70, &args![attacker, point, arc, arc])
            .bool()
    } else {
        e.call(0x009a_6ae0, &args![attacker, target, 0u32]).bool()
    }
}

/// Copies a point (three words) from `from` to `to`.
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for word in 0..3 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

// Translated from 008f9180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::CleanUpAfterProcessUseWeapon` (Xbox PDB). Takes the
/// use-weapon data of the running package (slot `0x27c`,
/// `TESPackage::GetUseWeaponPackageData`); when `00824060(data)` is false
/// and the bursts fired (+0x2c4) have reached the data's count (`008f21d0`),
/// it queues the current weapon (slot `0x148`) for unequipping, if that
/// resolves (`0044ddc0`) to a form of type `0x28` for which `004c0bf0` is
/// true (after `008a6840(actor, 0)`). It then clears the actor's look-at
/// target, looking and iron sights, calls `005ce9d0(actor, 0)`, slot
/// `0x394(actor)`, `ClearShootingAction(actor)`, slot `0x654(1)` and, when
/// the actor's pathing is not complete, slot `0x294(actor)`.
pub fn high_process_clean_up_after_process_use_weapon(e: &mut Engine, this: Ptr, actor: Ptr) {
    let package = e
        .vcall(this.addr(), PROCESS_SLOT_RUNNING_PACKAGE, &args![])
        .u32();
    let data = e.call(0x0067_58f0, &args![package]).u32();
    let mut finished = false;
    if !e.call(0x0082_4060, &args![data]).bool() {
        let bursts = i32::from(e.mem.i16(this.addr() + 0x2c4));
        let needed = e.call(0x008f_21d0, &args![data]).u16();
        if bursts >= i32::from(needed) {
            finished = true;
        }
    }
    if finished
        && e.vcall(this.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .u32()
            != 0
    {
        let mut weapon_form = 0;
        let current = e
            .vcall(this.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
            .u32();
        let item = e.call(0x0044_ddc0, &args![current]).u32();
        if item != 0 && e.call(0x0040_1170, &args![item]).i32() == 0x28 {
            let current = e
                .vcall(this.addr(), PROCESS_SLOT_CURRENT_WEAPON, &args![])
                .u32();
            weapon_form = e.call(0x0044_ddc0, &args![current]).u32();
        }
        if weapon_form != 0 && e.call(0x004c_0bf0, &args![weapon_form]).bool() {
            e.call(0x008a_6840, &args![actor, 0u32]);
            e.call(
                0x0088_c790,
                &args![actor, item, 1u32, 0u32, 0u32, 0u32, 1u32],
            );
        }
    }
    e.call(0x008b_3d30, &args![actor]);
    e.call(0x0093_1d90, &args![actor, 0.0f32]);
    e.call(0x008b_b650, &args![actor, 0u32, 0u32, 0u32]);
    e.call(0x005c_e9d0, &args![actor, 0u32]);
    e.vcall(
        this.addr(),
        PROCESS_SLOT_FREE_UP_SPECIAL_IDLE,
        &args![actor],
    );
    e.call(0x0092_bf20, &args![this, actor]);
    e.vcall(
        this.addr(),
        PROCESS_SLOT_CLEAR_USE_WEAPON_HEAD_TRACK_TARGET,
        &args![1u32],
    );
    if !e.call(0x008b_3bb0, &args![actor]).bool() {
        e.vcall(this.addr(), PROCESS_SLOT_END_MOVE_MESSAGE, &args![actor]);
    }
}

/// The byte `ShadowSceneNode` flag global the 3D update sets around
/// `005710c0` (`011c5cb4`).
const UPDATE_3D_GUARD: u32 = 0x011c_5cb4;
/// `00450b80(0)` (cdecl): the shadow scene node; its `RemoveObject`
/// (`00b5b1c0`) and `AddObject` (`00b5eeb0`) take a 3D node.
const SHADOW_SCENE_NODE: u32 = 0x0045_0b80;

// Translated from 008f69e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::Update3dModel` (Xbox PDB). `flags` is the process's 3D
/// update flags (slot `0x478`). Nothing but the tail runs if the actor's slot
/// `0x21c` test is true, `flags` is 0 or the actor has no 3D node (slot
/// `0x1d0`).
///
/// Flags 4 and 8 (head and body): when the node's slot `0x0c` answers, the
/// head (actor slot `0x1b0(0)`) and torso (`0x1ac(0)`) objects are removed
/// from the animation's object palette (`00a6e8e0`) and released through
/// their owner's slot `0xe8`, resetting the process's face nodes (slots
/// `0x79c`, `0x794`, `0x7a4`); then the actor's base form (`004181e0`) is
/// asked to `ClearHead` (flag 4 only), the process clears its face animation
/// data (slot `0x58`), and the head is rebuilt (`InitHead` and `00607420`);
/// for a form of type 7 (`0084e3a0`) `TESRace::KillEGTData` runs.
///
/// Flag 1 (reload the 3D): the actor's slot `0x210(0)` runs, the 3D node is
/// removed from the shadow scene node, `005710c0` runs (with the flag byte
/// `011c5cb4` set around it when flag 2 is on and it was clear), and the node
/// is added back. The face animation data is updated (slot `0x22c(0, 1)`
/// through its slot `0xd8`), the process refreshes the magic shaders (slot
/// `0x580(1, 1, 0)`), the actor's alpha is updated (`UpdateAlpha`) or, for a
/// refractive actor, set from its refraction extra data (slot `0x384`), and
/// unless the actor is the player with the inventory menu visible the
/// lighting property pointer is initialised (`InitLightingPropertyPtr`) and,
/// for other actors, the reference ID is set on the scene graph.
///
/// Flag 0x10 calls `00567490(actor, 00598040(actor))`. The tail clears the
/// flags (slot `0x470`) and, for any actor but the player, calls
/// `00936f50(actor, 0)`.
pub fn high_process_update_3d_model(e: &mut Engine, this: Ptr, actor: Ptr) {
    let flags = e
        .vcall(this.addr(), PROCESS_SLOT_3D_UPDATE_FLAGS, &args![])
        .u8();
    let player = e.mem.u32(PLAYER);
    let skip = e
        .vcall(actor.addr(), REFERENCE_SLOT_TEST_0X21C, &args![])
        .bool();
    let node = if skip || flags == 0 {
        0
    } else {
        e.vcall(actor.addr(), REFERENCE_SLOT_GET_3D, &args![]).u32()
    };
    let mut finished = false;
    if node != 0 {
        if flags & 4 != 0 || flags & 8 != 0 {
            if e.vcall(node, 0x0c, &args![]).u32() != 0 {
                let animation = e.vcall(this.addr(), PROCESS_SLOT_ANIMATION, &args![]).u32();
                let mut palette = 0;
                if animation != 0 && e.call(0x0049_6940, &args![animation]).u32() != 0 {
                    let owner = e.call(0x0049_6940, &args![animation]).u32();
                    palette = e.call(0x0053_7bd0, &args![owner]).u32();
                }
                let head = e
                    .vcall(actor.addr(), REFERENCE_SLOT_HEAD_NODE, &args![0u32])
                    .u32();
                if head != 0 && e.call(0x0096_11e0, &args![head]).u32() != 0 {
                    e.vcall(
                        this.addr(),
                        PROCESS_SLOT_SET_FACE_SKINNED_NODE,
                        &args![0u32],
                    );
                    e.call(0x00a6_e8e0, &args![head, palette]);
                    let owner = e.call(0x0096_11e0, &args![head]).u32();
                    e.vcall(owner, 0xe8, &args![head]);
                }
                let torso = e
                    .vcall(actor.addr(), REFERENCE_SLOT_TORSO_NODE, &args![0u32])
                    .u32();
                if torso != 0 && e.call(0x0096_11e0, &args![torso]).u32() != 0 {
                    e.vcall(this.addr(), PROCESS_SLOT_SET_FACE_NODE, &args![0u32]);
                    e.vcall(this.addr(), PROCESS_SLOT_SET_HEAD_ANIMS, &args![0u32]);
                    e.call(0x00a6_e8e0, &args![torso, palette]);
                    let owner = e.call(0x0096_11e0, &args![torso]).u32();
                    e.vcall(owner, 0xe8, &args![torso]);
                }
            }
            let base = e.call(0x0041_81e0, &args![actor]).u32();
            if flags & 4 != 0 {
                e.call(0x005d_d560, &args![base]);
            }
            e.vcall(
                this.addr(),
                PROCESS_SLOT_CLEAR_FACE_ANIMATION_DATA,
                &args![],
            );
            let face = e.call(0x005d_9f90, &args![actor]).u32();
            let (first, second) = e.with_stack(8, |e, out| {
                e.mem.set_u32(out.addr(), 0);
                e.mem.set_u32(out.addr() + 4, 0);
                e.call(0x0060_7370, &args![base, out, out.addr() + 4]);
                (e.mem.u32(out.addr()), e.mem.u32(out.addr() + 4))
            });
            e.call(0x0060_7420, &args![base, actor, face, first, second]);
            if e.call(0x0084_e3a0, &args![base]).i32() == 7 {
                let race = e.call(LIST_NEXT, &args![base + 0x10c]).u32();
                e.call(0x0061_3fd0, &args![race]);
            }
        }
        if flags & 1 != 0 {
            e.vcall(actor.addr(), 0x210, &args![0u32]);
            let current = e.vcall(actor.addr(), REFERENCE_SLOT_GET_3D, &args![]).u32();
            let scene = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
            e.call(0x00b5_b1c0, &args![scene, current]);
            let mut guard = false;
            if flags & 2 != 0 && e.mem.u8(UPDATE_3D_GUARD) == 0 {
                e.mem.set_u8(UPDATE_3D_GUARD, 1);
                guard = true;
            }
            e.call(0x0057_10c0, &args![actor]);
            if guard {
                e.mem.set_u8(UPDATE_3D_GUARD, 0);
            }
            let current = e.vcall(actor.addr(), REFERENCE_SLOT_GET_3D, &args![]).u32();
            let scene = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
            e.call(0x00b5_eeb0, &args![scene, current]);
            let mut save_form = 0;
            if e.vcall(actor.addr(), REFERENCE_SLOT_IS_CREATURE, &args![])
                .bool()
            {
                save_form = e.call(0x007a_f430, &args![actor]).u32();
            }
            let face_data = e.call(0x008a_dcb0, &args![actor]).u32();
            if face_data != 0 {
                // 008f6d02: slot 0x22c takes one word (0); the pushed 1 is the
                // second word of the call through +0xd8.
                let value = e.vcall(actor.addr(), 0x22c, &args![0u32]).u8();
                e.vcall(face_data, 0xd8, &args![value as u32, 1u32]);
            }
            if save_form != 0 {
                e.call(0x0060_56f0, &args![save_form, actor]);
            }
            e.vcall(
                this.addr(),
                PROCESS_SLOT_REFRESH_MAGIC_SHADERS,
                &args![1u32, 1u32, 0u32],
            );
            if e.call(0x005b_9b00, &args![]).bool() && e.call(0x008c_51c0, &args![actor]).bool() {
                let list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
                let extra = e.call(0x0042_2820, &args![list]).u32();
                let power = e.mem.f32(extra + 0xc);
                e.vcall(actor.addr(), 0x384, &args![1u32, power]);
            } else {
                e.call(0x008c_4640, &args![actor]);
            }
            if actor.addr() == player && e.call(0x0070_4ad0, &args![]).bool() {
                // The no-op destructor `00483710(player)` is left out.
                finished = true;
            }
            if !finished {
                let mut lit = e.vcall(actor.addr(), REFERENCE_SLOT_GET_3D, &args![]).u32();
                if actor.addr() == player && !e.call(0x004e_af60, &args![player]).bool() {
                    lit = e.call(0x0045_bc00, &args![lit, 0u32]).u32();
                }
                e.call(0x008b_0bd0, &args![actor, lit]);
                if actor.addr() != player {
                    let form_id = e.call(0x0084_e3a0, &args![actor]).u32();
                    let current = e.vcall(actor.addr(), REFERENCE_SLOT_GET_3D, &args![]).u32();
                    e.call(0x004b_6dc0, &args![current, form_id]);
                }
            }
        }
    }
    if flags & 0x10 != 0 {
        let value = e.call(0x0059_8040, &args![actor]).f32();
        e.call(0x0056_7490, &args![actor, value]);
    }
    e.vcall(this.addr(), PROCESS_SLOT_CLEAR_3D_UPDATE_FLAGS, &args![]);
    if actor.addr() != player {
        e.call(0x0093_6f50, &args![actor, 0u32]);
    }
}

/// The global `TESSaveLoadGame`-style pointer the save code passes as `this`
/// to `008df040` (the save version) and `UseSaveGameBlocks` (`011de45c`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `008df040(this)`: the save game version byte (the byte at +0x80).
const SAVE_VERSION: u32 = 0x008d_f040;
/// `TESSaveLoadGame::UseSaveGameBlocks` (`00862110`).
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// `Error(format, ...)` (`0040fbe0`, cdecl; the release build keeps the
/// calls but the function does nothing).
const ERROR_LOG: u32 = 0x0040_fbe0;
/// The source file name the error messages print (`01088358`).
const SOURCE_FILE_NAME: u32 = 0x0108_8358;

// Translated from 008fa630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetSaveSize` (Xbox PDB): the number of bytes `SaveGame`
/// writes, as a 16-bit sum. It starts with `MiddleHighProcess::GetSaveSize`
/// (`flags`, `context`); then every fixed field is added in the order the
/// save writes it (the comments give the widths), the fields newer than save
/// version 0x32, 0x3f, 0x42, 0x5a, 0x5d, 0x6a and 0x71 (from `008df040`) only
/// when the save is that new; the detection list counts 13 bytes per entry
/// (`pDetectedActorList` at +0x25c), flag `0x10000000` adds 1 and the
/// character controller size (`GetCharControllerSaveSize`) is added. When the
/// debug setting at `011de4e8` is set, the size is logged through `Error`
/// with the current world space's form.
pub fn high_process_get_save_size(e: &mut Engine, this: Ptr, flags: u32, context: u32) -> u16 {
    let mut size: u16 = 0;
    size = size.wrapping_add(e.call(0x0092_4220, &args![this, flags, context]).u16());
    let base = size;
    let tes = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![tes]).bool() {
        size = size.wrapping_add(4 + 2);
    }
    // 1 + 1 + 1 + 1, 4 + 2 + 2, then ten words of 4.
    size = size.wrapping_add(4 + 4 + 2 + 2 + 40);
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x32 {
        size = size.wrapping_add(4);
    }
    // 2 + 2 + 2 + 1 + 12 + six words of 4.
    size = size.wrapping_add(2 + 2 + 2 + 1 + 12 + 24);
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x3f {
        size = size.wrapping_add(1 + 4);
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x42 {
        size = size.wrapping_add(1 + 4);
    }
    size = size.wrapping_add(4 + 4 + 4 + 2);
    let detected = e.mem.u32(this.addr() + DETECTED_ACTOR_LIST);
    let count = e.call(LIST_COUNT, &args![detected]).u32();
    size = size.wrapping_add(count.wrapping_mul(13) as u16);
    if flags & 0x1000_0000 != 0 {
        size = size.wrapping_add(1);
    }
    size = size.wrapping_add(2);
    size = size.wrapping_add(e.call(0x0092_6350, &args![this, context]).u16());
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x5a {
        size = size.wrapping_add(4 + 1);
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x5d {
        size = size.wrapping_add(4 + 4);
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x6a {
        size = size.wrapping_add(4);
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x71 {
        size = size.wrapping_add(1 + 4 + 4 + 4 + 1 + 1 + 4 + 4 + 1);
    }
    let setting = e.call(0x0040_8d60, &args![0x011d_e4e8u32]).u32();
    if e.mem.u8(setting) != 0 {
        let world_space = e.call(0x004f_d3e0, &args![tes]).u32();
        let used = u32::from(size.wrapping_sub(base));
        if world_space != 0 {
            let form_id = e.mem.u32(world_space);
            let form = e.call(0x0048_39c0, &args![form_id]).u32();
            let name = e.vcall(form, 0x130, &args![]).u32();
            // `[world space + 5]` is read as the game does (an unaligned word).
            let word = e.mem.u32(world_space + 5);
            e.call(
                ERROR_LOG,
                &args![
                    0x0101_2cb0u32,
                    used,
                    form_id,
                    name,
                    word,
                    0x3152u32,
                    SOURCE_FILE_NAME
                ],
            );
        } else {
            e.call(
                ERROR_LOG,
                &args![0x0101_2c78u32, used, 0x3152u32, SOURCE_FILE_NAME],
            );
        }
    }
    size
}

/// `TESSaveLoadGame::Save(this, data, size)` (`008579b0`): appends `size`
/// bytes read from `data` to the save buffer.
const SAVE_BYTES_TO_BUFFER: u32 = 0x0085_79b0;
/// `TESSaveLoadGame::SaveNumericID(this, id, size)` (`00857a10`): writes a
/// form ID (read from `id`) in the save's numbering.
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
/// The save buffer's current write position (`00825c00`: the word at +0x14).
const SAVE_POSITION: u32 = 0x0082_5c00;
/// The debug-logging setting check (`00408d60(011de4e8)`, a pointer to a byte).
const LOGGING_SETTING: u32 = 0x0040_8d60;
const LOGGING_SETTING_OBJECT: u32 = 0x011d_e4e8;
/// `TES::GetWorldSpace` (`004fd3e0`).
const GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `TESForm::GetFormID` (`0084e3a0`: the word at +0x0c).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// Whether a list is empty (`008256d0`: no item and no next node).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;

/// Writes `size` bytes of `this + offset` to the save buffer.
fn save_field(e: &mut Engine, tes: u32, this: Ptr, offset: u32, size: u32) {
    e.call(
        SAVE_BYTES_TO_BUFFER,
        &args![tes, this.addr() + offset, size],
    );
}

/// Writes the numeric form ID of the form in `this + offset` (0 when the
/// pointer is null) as four bytes.
fn save_form_id_field(e: &mut Engine, tes: u32, this: Ptr, offset: u32) {
    let form = e.mem.u32(this.addr() + offset);
    let id = if form != 0 {
        e.call(GET_FORM_ID, &args![form]).u32()
    } else {
        0
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        e.call(SAVE_NUMERIC_ID, &args![tes, slot, 4u32]);
    });
}

/// Writes `size` bytes holding `value` (a local of the game's stack frame).
fn save_local(e: &mut Engine, tes: u32, value: u32, size: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(SAVE_BYTES_TO_BUFFER, &args![tes, slot, size]);
    });
}

// Translated from 008faa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SaveGame` (Xbox PDB): writes the process to the save
/// buffer. `MiddleHighProcess::SaveGame(flags, context)` goes first. With
/// save blocks the `BLOK` tag and a two-byte block size placeholder follow;
/// then the fixed fields in the order `GetSaveSize` counts them, the fields
/// newer than save versions 0x32, 0x3f, 0x42, 0x5a, 0x5d, 0x6a and 0x71 only
/// for a save at least that new, the form IDs of `pGreetActor` (+0x30c),
/// `plastDetected` (+0x2a4) and `pTeleportFadeRef` (+0x3f0), the detection
/// list (+0x25c, a counted run of: actor ID, level word, line-of-sight byte,
/// `iLevel`), the selected animation sequence index (flag `0x10000000`), the
/// character controller (size word then `SaveCharController` when the size
/// is not 0) and the later version fields. Afterwards the entry counts and
/// the block size are patched into the placeholders. The 4-byte field after
/// the third flag byte is written from a stack slot the game never sets (it
/// is written as zero here). Debug logging goes through `Error` when the
/// setting at `011de4e8` is set.
pub fn high_process_save_game(e: &mut Engine, this: Ptr, flags: u32, context: u32) {
    e.call(0x0092_4560, &args![this, flags, context]);
    let tes = e.mem.u32(SAVE_LOAD_GAME);
    let mut start = e.call(SAVE_POSITION, &args![tes]).u32();
    let setting = e
        .call(LOGGING_SETTING, &args![LOGGING_SETTING_OBJECT])
        .u32();
    if e.mem.u8(setting) != 0 {
        start = e.call(SAVE_POSITION, &args![tes]).u32();
    }
    let mut block_size_slot = 0;
    if e.call(USE_SAVE_GAME_BLOCKS, &args![tes]).bool() {
        save_local(e, tes, 0x424c_4f4b, 4);
        block_size_slot = e.call(SAVE_POSITION, &args![tes]).u32();
        save_local(e, tes, 0, 2);
    }
    save_field(e, tes, this, 0x32c, 1);
    save_field(e, tes, this, 0x340, 1);
    save_field(e, tes, this, 0x374, 1);
    save_field(e, tes, this, 0x375, 1);
    save_local(e, tes, 0, 4);
    save_field(e, tes, this, 0x2ec, 2);
    save_field(e, tes, this, 0x2fc, 2);
    for offset in [
        0x2e8, 0x2b4, 0x2f8, 0x310, 0x330, 0x334, 0x338, 0x34c, 0x294, 0x2b8, 0x2bc,
    ] {
        save_field(e, tes, this, offset, 4);
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x32 {
        save_field(e, tes, this, 0x298, 4);
    }
    save_field(e, tes, this, 0x2c0, 2);
    save_field(e, tes, this, 0x2c2, 2);
    save_field(e, tes, this, 0x2c4, 2);
    save_field(e, tes, this, 0x349, 1);
    save_field(e, tes, this, 0x300, 0xc);
    for offset in [0x36c, 0x3e8, 0x3ec, 0x33c, 0x2a8, 0x378] {
        save_field(e, tes, this, offset, 4);
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x3f {
        save_field(e, tes, this, 0x3a0, 1);
        save_field(e, tes, this, 0x39c, 4);
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x42 {
        save_field(e, tes, this, 0x3a8, 1);
        save_field(e, tes, this, 0x3a4, 4);
    }
    save_form_id_field(e, tes, this, 0x30c);
    save_form_id_field(e, tes, this, 0x2a4);
    save_form_id_field(e, tes, this, 0x3f0);
    let mut count: u16 = 0;
    let count_slot = e.call(SAVE_POSITION, &args![tes]).u32();
    save_local(e, tes, 0, 2);
    let mut node = e.mem.u32(this.addr() + DETECTED_ACTOR_LIST);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let state = list_item(e, node);
        let actor = e.mem.u32(state);
        let id = if actor != 0 {
            e.call(GET_FORM_ID, &args![actor]).u32()
        } else {
            0
        };
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), id);
            e.call(SAVE_NUMERIC_ID, &args![tes, slot, 4u32]);
        });
        e.call(SAVE_BYTES_TO_BUFFER, &args![tes, state + 4, 4u32]);
        e.call(SAVE_BYTES_TO_BUFFER, &args![tes, state + 0x1e, 1u32]);
        e.call(SAVE_BYTES_TO_BUFFER, &args![tes, state + 8, 4u32]);
        count = count.wrapping_add(1);
        node = list_next(e, node);
    }
    e.mem.set_u16(count_slot, count);
    if flags & 0x1000_0000 != 0 {
        let mut selected: u8 = 0xff;
        if e.vcall(this.addr(), PROCESS_SLOT_ANIMATION, &args![]).u32() != 0
            && e.mem.u32(this.addr() + 0x2f0) != 0
        {
            for index in 0..8u32 {
                let animation = e.vcall(this.addr(), PROCESS_SLOT_ANIMATION, &args![]).u32();
                let sequence = e.call(0x0049_1040, &args![animation, index]).u32();
                if sequence == e.mem.u32(this.addr() + 0x2f0) {
                    selected = index as u8;
                }
            }
        }
        save_local(e, tes, u32::from(selected), 1);
    }
    let controller_size = e.call(0x0092_6350, &args![this, context]).u16();
    save_local(e, tes, u32::from(controller_size), 2);
    if controller_size != 0 {
        e.call(0x0092_6490, &args![this, context]);
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x5a {
        save_form_id_field(e, tes, this, 0x41c);
        save_field(e, tes, this, 0x420, 1);
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x5d {
        save_field(e, tes, this, 0x3bc, 4);
        save_field(e, tes, this, 0x3c0, 4);
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x6a {
        save_form_id_field(e, tes, this, 0x370);
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x71 {
        save_field(e, tes, this, 0x2c6, 1);
        save_field(e, tes, this, 0x2d0, 4);
        save_field(e, tes, this, 0x2d4, 4);
        save_field(e, tes, this, 0x2d8, 4);
        save_field(e, tes, this, 0x3b8, 1);
        save_field(e, tes, this, 0x2dc, 1);
        save_field(e, tes, this, 0x2e0, 4);
        save_field(e, tes, this, 0x344, 4);
        save_field(e, tes, this, 0x3b8, 1);
    }
    let setting = e
        .call(LOGGING_SETTING, &args![LOGGING_SETTING_OBJECT])
        .u32();
    if e.mem.u8(setting) != 0 {
        let end = e.call(SAVE_POSITION, &args![tes]).u32();
        let world_space = e.call(GET_WORLD_SPACE, &args![tes]).u32();
        let used = end.wrapping_sub(start);
        if world_space != 0 {
            let form_id = e.mem.u32(world_space);
            let form = e.call(0x0048_39c0, &args![form_id]).u32();
            let name = e.vcall(form, 0x130, &args![]).u32();
            // `[world space + 5]` is read as the game does (an unaligned word).
            let word = e.mem.u32(world_space + 5);
            e.call(
                ERROR_LOG,
                &args![
                    0x0101_53a0u32,
                    used,
                    form_id,
                    name,
                    word,
                    0x3200u32,
                    SOURCE_FILE_NAME
                ],
            );
        } else {
            e.call(
                ERROR_LOG,
                &args![0x0101_536cu32, used, 0x3200u32, SOURCE_FILE_NAME],
            );
        }
    }
    if e.call(USE_SAVE_GAME_BLOCKS, &args![tes]).bool() {
        let end = e.call(SAVE_POSITION, &args![tes]).u32();
        let begin = block_size_slot;
        if end > begin.wrapping_add(0xffff) {
            e.call(
                0x005b_5e40,
                &args![0x0101_5318u32, SOURCE_FILE_NAME, 0x3200u32],
            );
        }
        e.mem
            .set_u16(block_size_slot, end.wrapping_sub(begin) as u16);
    }
}

/// `TESSaveLoadGame::Load(this, destination, size)` (`008579e0`).
const LOAD_BYTES_FROM_BUFFER: u32 = 0x0085_79e0;
/// `TESSaveLoadGame::LoadNumericID(this, destination, size)` (`00857aa0`):
/// reads a form ID and converts it from the save's numbering.
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// The load-side world space getter (`004fd3c0`, `this` is the save/load
/// object).
const LOAD_GET_WORLD_SPACE: u32 = 0x004f_d3c0;
/// `005b5e40(format, ...)` (cdecl): the error report the load code uses.
const REPORT_ERROR: u32 = 0x005b_5e40;
/// `TESSaveLoadGame` skip/discard (`00857bd0(this, 1)`).
const SKIP_BYTES: u32 = 0x0085_7bd0;
/// `TESSaveLoadGame` load of the character controller block
/// (`0085f420(this, actor, size)`).
const LOAD_CHARACTER_CONTROLLER: u32 = 0x0085_f420;

/// Reads `size` bytes from the save into `this + offset`.
fn load_field(e: &mut Engine, tes: u32, this: Ptr, offset: u32, size: u32) {
    e.call(
        LOAD_BYTES_FROM_BUFFER,
        &args![tes, this.addr() + offset, size],
    );
}

/// Reads `size` bytes from the save and returns them (zero-extended).
fn load_value(e: &mut Engine, tes: u32, size: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.call(LOAD_BYTES_FROM_BUFFER, &args![tes, slot, size]);
        match size {
            1 => u32::from(e.mem.u8(slot.addr())),
            2 => u32::from(e.mem.u16(slot.addr())),
            _ => e.mem.u32(slot.addr()),
        }
    })
}

/// Reads a numeric form ID (four bytes) from the save.
fn load_id(e: &mut Engine, tes: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.call(LOAD_NUMERIC_ID, &args![tes, slot, 4u32]);
        e.mem.u32(slot.addr())
    })
}

/// Frees every `DetectionState` of the list at `list` and empties it.
fn free_list_items(e: &mut Engine, list: u32) {
    let mut node = list;
    while node != 0 {
        let item = list_item(e, node);
        if item == 0 {
            break;
        }
        e.call(OPERATOR_DELETE, &args![item]);
        node = list_next(e, node);
    }
    e.call(LIST_CLEAR, &args![list]);
}

/// Reports a block problem through `005b5e40`: the format, the byte difference
/// (`amount`, when the format has one), the source file and line, then either
/// the current world space's form ID, name (slot `0x130` of the form), byte
/// at +9 and word at +5 or, without a world space, the save version.
fn report_block_mismatch(
    e: &mut Engine,
    tes: u32,
    world_space: u32,
    formats: (u32, u32),
    amount: Option<u32>,
    line: u32,
) {
    let (with_world_space, without_world_space) = formats;
    let mut words = vec![0u32];
    if world_space != 0 {
        let form_id = e.mem.u32(world_space);
        let form = e.call(0x0048_39c0, &args![form_id]).u32();
        let name = e.vcall(form, 0x130, &args![]).u32();
        let byte = u32::from(e.mem.u8(world_space + 9));
        let word = e.mem.u32(world_space + 5);
        words[0] = with_world_space;
        words.extend(amount);
        words.extend([SOURCE_FILE_NAME, line, form_id, name, byte, word]);
    } else {
        let version = u32::from(e.call(SAVE_VERSION, &args![tes]).u8());
        words[0] = without_world_space;
        words.extend(amount);
        words.extend([SOURCE_FILE_NAME, line, version]);
    }
    e.call(REPORT_ERROR, &words);
}

// Translated from 008fb330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::LoadGame` (Xbox PDB): the inverse of `SaveGame`.
/// `MiddleHighProcess::LoadGame(flags, context, actor)` goes first. With
/// save blocks the `BLOK` tag is checked (a mismatch is reported) and the
/// block size read; then every field `SaveGame` writes is read back into the
/// process (older saves lack some fields and have others the process no
/// longer keeps: those are read and dropped, and for versions below 0x4f the
/// item being used (+0x34) is rebuilt from a form ID, kept only when slot
/// `0xe4` of the form is true). The form IDs go into `pGreetActor`,
/// `plastDetected` and `pTeleportFadeRef`; the old detection list is freed
/// and rebuilt from the counted entries (and freed again when `actor` is the
/// player); the animation sequence index (+0x2f0) comes from the byte read
/// when flag `0x10000000` is set (`-1` is none, else the index plus 8). The
/// character controller block follows. At the end the position in the block
/// is compared with the announced size and a mismatch is reported.
pub fn high_process_load_game(e: &mut Engine, this: Ptr, flags: u32, context: u32, actor: Ptr) {
    e.call(0x0092_4b90, &args![this, flags, context, actor]);
    let tes = e.mem.u32(SAVE_LOAD_GAME);
    let mut block_size: u16 = 0;
    let mut block_start = 0;
    if e.call(USE_SAVE_GAME_BLOCKS, &args![tes]).bool() {
        let tag = load_value(e, tes, 4);
        if tag != 0x424c_4f4b {
            let world_space = e.call(LOAD_GET_WORLD_SPACE, &args![tes]).u32();
            report_block_mismatch(
                e,
                tes,
                world_space,
                (0x0101_5718, 0x0101_56a8),
                None,
                0x320d,
            );
        }
        block_start = e.call(SAVE_POSITION, &args![tes]).u32();
        block_size = load_value(e, tes, 2) as u16;
    }
    let version = |e: &mut Engine| -> u8 { e.call(SAVE_VERSION, &args![tes]).u8() };
    if version(e) < 0x3e {
        load_value(e, tes, 1);
    }
    load_field(e, tes, this, 0x32c, 1);
    load_field(e, tes, this, 0x340, 1);
    if version(e) < 0x19 {
        load_value(e, tes, 1);
    }
    load_field(e, tes, this, 0x374, 1);
    if version(e) >= 0x1b {
        load_field(e, tes, this, 0x375, 1);
    }
    load_value(e, tes, 4);
    load_field(e, tes, this, 0x2ec, 2);
    load_field(e, tes, this, 0x2fc, 2);
    for offset in [
        0x2e8, 0x2b4, 0x2f8, 0x310, 0x330, 0x334, 0x338, 0x34c, 0x294, 0x2b8, 0x2bc,
    ] {
        load_field(e, tes, this, offset, 4);
    }
    if version(e) >= 0x32 {
        load_field(e, tes, this, 0x298, 4);
    }
    load_field(e, tes, this, 0x2c0, 2);
    load_field(e, tes, this, 0x2c2, 2);
    load_field(e, tes, this, 0x2c4, 2);
    load_field(e, tes, this, 0x349, 1);
    load_field(e, tes, this, 0x300, 0xc);
    for offset in [0x36c, 0x3e8, 0x3ec, 0x33c, 0x2a8] {
        load_field(e, tes, this, offset, 4);
    }
    if version(e) >= 0x14 {
        load_field(e, tes, this, 0x378, 4);
    }
    if version(e) >= 0x3f {
        load_field(e, tes, this, 0x3a0, 1);
        load_field(e, tes, this, 0x39c, 4);
    }
    if version(e) >= 0x42 {
        load_field(e, tes, this, 0x3a8, 1);
        load_field(e, tes, this, 0x3a4, 4);
    }
    let id = load_id(e, tes);
    e.mem.set_u32(this.addr() + 0x30c, id);
    if version(e) < 0x71 {
        load_id(e, tes);
    }
    let id = load_id(e, tes);
    e.mem.set_u32(this.addr() + 0x2a4, id);
    if version(e) < 0x4f {
        let id = load_id(e, tes);
        let form = e.call(0x0048_39c0, &args![id]).u32();
        let keep = e.vcall(form, 0xe4, &args![]).bool();
        e.mem
            .set_u32(this.addr() + 0x34, if keep { form } else { 0 });
    }
    let id = load_id(e, tes);
    e.mem.set_u32(this.addr() + 0x3f0, id);
    let list = e.mem.u32(this.addr() + DETECTED_ACTOR_LIST);
    free_list_items(e, list);
    let mut entries: u16 = 0;
    if version(e) >= 0x4e {
        entries = load_value(e, tes, 2) as u16;
    }
    if version(e) < 0x4e {
        entries = load_value(e, tes, 1) as u16;
    }
    for _ in 0..entries {
        let state = new_detection_state(e);
        let actor_id = load_id(e, tes);
        e.call(LOAD_BYTES_FROM_BUFFER, &args![tes, state + 4, 4u32]);
        e.call(LOAD_BYTES_FROM_BUFFER, &args![tes, state + 0x1e, 1u32]);
        e.call(LOAD_BYTES_FROM_BUFFER, &args![tes, state + 8, 4u32]);
        e.mem.set_u32(state, actor_id);
        let list = e.mem.u32(this.addr() + DETECTED_ACTOR_LIST);
        list_append(e, list, state);
    }
    if actor.addr() == e.mem.u32(PLAYER) {
        let list = e.mem.u32(this.addr() + DETECTED_ACTOR_LIST);
        free_list_items(e, list);
    }
    e.mem.set_u32(this.addr() + 0x2f0, 0);
    if flags & 0x1000_0000 != 0 {
        let index = load_value(e, tes, 1) as u8 as i8;
        if index == -1 {
            e.mem.set_u32(this.addr() + 0x2f0, 0);
        } else {
            e.mem
                .set_u32(this.addr() + 0x2f0, (i32::from(index) + 8) as u32);
        }
    } else if version(e) < 0x1c
        && e.vcall(actor.addr(), REFERENCE_SLOT_IS_ACTOR, &args![])
            .bool()
    {
        e.call(SKIP_BYTES, &args![tes, 1u32]);
    }
    let controller_size = load_value(e, tes, 2);
    if controller_size != 0 {
        e.call(
            LOAD_CHARACTER_CONTROLLER,
            &args![tes, actor, controller_size],
        );
    }
    if version(e) >= 0x5a {
        let id = load_id(e, tes);
        e.mem.set_u32(this.addr() + 0x41c, id);
        load_field(e, tes, this, 0x420, 1);
    }
    if version(e) >= 0x5d {
        load_field(e, tes, this, 0x3bc, 4);
        load_field(e, tes, this, 0x3c0, 4);
    }
    if version(e) >= 0x6a {
        let id = load_id(e, tes);
        e.mem.set_u32(this.addr() + 0x370, id);
    }
    if version(e) >= 0x71 {
        load_field(e, tes, this, 0x2c6, 1);
        load_field(e, tes, this, 0x2d0, 4);
        load_field(e, tes, this, 0x2d4, 4);
        load_field(e, tes, this, 0x2d8, 4);
        load_field(e, tes, this, 0x3b8, 1);
        load_field(e, tes, this, 0x2dc, 1);
        load_field(e, tes, this, 0x2e0, 4);
        load_field(e, tes, this, 0x344, 4);
        load_field(e, tes, this, 0x3b8, 1);
        if version(e) < 0x7d {
            for _ in 0..5 {
                load_id(e, tes);
            }
        }
    }
    // The game keeps `actor` in a local when slot 0x100 accepts it; nothing
    // reads that local afterwards, but the slot is still called.
    if !actor.is_null() {
        e.vcall(actor.addr(), REFERENCE_SLOT_IS_ACTOR, &args![]);
    }
    if e.call(USE_SAVE_GAME_BLOCKS, &args![tes]).bool() {
        let position = e.call(SAVE_POSITION, &args![tes]).u32();
        let world_space = e.call(LOAD_GET_WORLD_SPACE, &args![tes]).u32();
        let expected = u32::from(block_size).wrapping_add(block_start);
        if position > expected {
            report_block_mismatch(
                e,
                tes,
                world_space,
                (0x0101_5588, 0x0101_54a0),
                Some(position.wrapping_sub(expected)),
                0x32ff,
            );
        } else if position < expected {
            report_block_mismatch(
                e,
                tes,
                world_space,
                (0x0101_5500, 0x0101_5440),
                Some(expected.wrapping_sub(position)),
                0x32ff,
            );
        }
    }
}

/// `__RTDynamicCast(form of form_id, 0, TESForm, target, 0)` for the form
/// with `form_id` (`004839c0` then `00ec43fb`).
fn form_cast_to(e: &mut Engine, form_id: u32, target: u32) -> u32 {
    let form = e.call(0x0048_39c0, &args![form_id]).u32();
    e.call(
        0x00ec_43fb,
        &args![form, 0u32, 0x0118_3028u32, target, 0u32],
    )
    .u32()
}

/// `TypeDescriptor` of `TESObjectREFR` (`011841cc`), of `Actor`
/// (`011846d4`) and the one `pDialogTarget` is resolved to (`01184920`).
const TYPE_REFERENCE: u32 = 0x0118_41cc;
const TYPE_ACTOR_CLASS: u32 = 0x0118_46d4;
const TYPE_DIALOG_TARGET: u32 = 0x0118_4920;

// Translated from 008fbfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::InitLoadGame` (Xbox PDB): after `MiddleHighProcess::
/// InitLoadGame(a, b, c)` the form IDs loaded into `pGreetActor` (+0x30c),
/// `plastDetected` (+0x2a4) and `pTeleportFadeRef` (+0x3f0) are replaced by
/// the `TESObjectREFR` they name (or null). Each entry of the detection list
/// (+0x25c) gets its actor ID resolved to an `Actor`; entries whose actor does
/// not resolve are removed and freed. For saves of version 0x5a and later
/// `pLastTarget` (+0x41c) is resolved to a reference too, and from 0x6a
/// `pDialogTarget` (+0x370) to the class at `01184920`.
pub fn high_process_init_load_game(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32) {
    e.call(0x0092_5490, &args![this, a, b, c]);
    for offset in [0x30c, 0x2a4, 0x3f0] {
        let id = e.mem.u32(this.addr() + offset);
        let cast = form_cast_to(e, id, TYPE_REFERENCE);
        e.mem.set_u32(this.addr() + offset, cast);
    }
    let mut node = e.mem.u32(this.addr() + DETECTED_ACTOR_LIST);
    let mut previous = 0;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let state = list_item(e, node);
        let actor_id = e.mem.u32(state);
        if actor_id != 0 {
            let actor = form_cast_to(e, actor_id, TYPE_ACTOR_CLASS);
            e.mem.set_u32(state, actor);
        }
        if e.mem.u32(state) == 0 {
            if previous == 0 {
                e.call(LIST_REMOVE_HEAD, &args![node]);
            } else {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), state);
                    e.call(LIST_REMOVE_ITEM, &args![previous, slot]);
                });
                node = list_next(e, previous);
            }
            e.call(OPERATOR_DELETE, &args![state]);
        } else {
            previous = node;
            node = list_next(e, node);
        }
    }
    let tes = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x5a {
        let id = e.mem.u32(this.addr() + 0x41c);
        if id != 0 {
            let cast = form_cast_to(e, id, TYPE_REFERENCE);
            e.mem.set_u32(this.addr() + 0x41c, cast);
        }
    }
    if e.call(SAVE_VERSION, &args![tes]).u8() >= 0x6a {
        let id = e.mem.u32(this.addr() + 0x370);
        if id != 0 {
            let cast = form_cast_to(e, id, TYPE_DIALOG_TARGET);
            e.mem.set_u32(this.addr() + 0x370, cast);
        }
    }
}

// Translated from 008fc210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::Revert` (Xbox PDB): runs `MiddleHighProcess::Revert(a,
/// actor)`; when `0055f5b0` (called on the save/load object) is true it then
/// resets the process to its start-of-game values (the cached actor values
/// object at +0x428 is told to reset through `008c6eb0`, the timers, flags,
/// shot counters, head-tracking targets and so on are cleared, the leveled
/// spell list +0x3b4 is emptied and nulled) and, if `actor` is an actor
/// (slot `0x100`), calls the process's slot `0x394` with it.
pub fn high_process_revert(e: &mut Engine, this: Ptr, a: u32, actor: Ptr) {
    let mut actor_ref = 0;
    if !actor.is_null()
        && e.vcall(actor.addr(), REFERENCE_SLOT_IS_ACTOR, &args![])
            .bool()
    {
        actor_ref = actor.addr();
    }
    e.call(0x0092_6000, &args![this, a, actor]);
    let tes = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(0x0055_f5b0, &args![tes]).bool() {
        let base = this.addr();
        let cache = e.mem.u32(base + 0x428);
        if cache != 0 {
            e.call(0x008c_6eb0, &args![cache]);
        }
        e.mem.set_u8(base + 0x364, 0);
        e.mem.set_u32(base + 0x3ac, 0xffff_ffff);
        e.mem.set_u32(base + 0x3b0, 0xffff_ffff);
        e.mem.set_f32(base + 0x2d0, 0.0);
        e.mem.set_u8(base + 0x2c6, 0);
        e.mem.set_u32(base + 0xb4, 0xffff_ffff);
        e.mem.set_f32(base + 0x310, 0.0);
        e.mem.set_u8(base + 0x32c, 0);
        e.mem.set_f32(base + 0x330, 0.0);
        e.mem.set_u8(base + 0x340, 1);
        e.mem.set_u8(base + 0x349, 0);
        e.mem.set_u32(base + 0x368, 0);
        e.mem.set_u8(base + 0x374, 0);
        e.mem.set_f32(base + 0x34c, 0.0);
        e.mem.set_f32(base + 0x338, 0.0);
        e.mem.set_f32(base + 0x2e0, 0.0);
        e.mem.set_f32(base + 0x2bc, 0.0);
        e.mem.set_f32(base + 0x2b8, 0.0);
        e.mem.set_u16(base + 0x2c0, 0);
        e.mem.set_u16(base + 0x2c2, 0xffff);
        e.mem.set_u16(base + 0x2c4, 0);
        let breath = e.global::<f32>(DEFAULT_BREATH_TIMER);
        e.mem.set_f32(base + 0x33c, breath);
        e.mem.set_f32(base + 0x2a8, 0.0);
        e.mem.set_f32(base + 0x2d8, 0.0);
        e.mem.set_f32(base + 0x344, 0.0);
        e.mem.set_u8(base + 0x2dc, 0);
        e.mem.set_u8(base + 0x375, 0);
        e.mem.set_f32(base + 0x378, 0.0);
        e.mem.set_f32(base + 0x37c, 0.0);
        e.mem.set_f32(base + 0x384, 1.0);
        e.mem.set_f32(base + 0x388, 0.0);
        e.mem.set_u32(base + 0x39c, 0);
        e.mem.set_u8(base + 0x3a0, 0);
        e.mem.set_f32(base + 0x3a4, 0.0);
        e.mem.set_u8(base + 0x3a8, 0);
        e.mem.set_u8(base + 0x3b8, 0);
        e.mem.set_u8(base + 0x3b9, 0);
        e.mem.set_u8(base + 0x3d0, 0);
        e.mem.set_u32(base + 0x3cc, 0);
        e.mem.set_u8(base + 0x3d1, 0);
        for index in 0..6 {
            e.mem.set_u32(base + 0x3f8 + 4 * index, 0);
            e.mem.set_u8(base + 0x410 + index, 0);
        }
        e.mem.set_u32(base + 0x41c, 0);
        e.mem.set_u8(base + 0x420, 0);
        e.mem.set_f32(base + 0x3bc, 0.0);
        e.mem.set_f32(base + 0x3c0, 0.0);
        let spells = e.mem.u32(base + 0x3b4);
        if spells != 0 {
            e.call(LIST_CLEAR, &args![spells]);
            e.mem.set_u32(base + 0x3b4, 0);
        }
        if actor_ref != 0 {
            e.vcall(base, PROCESS_SLOT_FREE_UP_SPECIAL_IDLE, &args![actor_ref]);
        }
    }
}

/// Process virtual slots `HighProcess::Update_ov2` uses (names from the
/// engine map; `BaseProcess` slots are the same on PC and Xbox below `0x100`).
const PROCESS_SLOT_COMPUTE_LAST_TIME_PROCESSED: u32 = 0x28;
const PROCESS_SLOT_CHECK_FOR_NEW_PACKAGE: u32 = 0x24;
const PROCESS_SLOT_PROCESS_ACTIVATE_ONE_HOUR: u32 = 0x74;
const PROCESS_SLOT_PROCESS_SANDMAN: u32 = 0x7c;
const PROCESS_SLOT_PROCESS_CANNIBAL: u32 = 0x80;
const PROCESS_SLOT_SET_TARGET_FOR_PACKAGE: u32 = 0x8c;
const PROCESS_SLOT_SET_CURRENT_ACTION_COMPLETE: u32 = 0x118;
const PROCESS_SLOT_SET_TARGET: u32 = 0x12c;
const PROCESS_SLOT_GET_LOCKED_LOCATION: u32 = 0x1bc;
const PROCESS_SLOT_SET_LOCKED_LOCATION: u32 = 0x1c0;
const PROCESS_SLOT_GET_RUN_ONCE_PACKAGE: u32 = 0x20c;
const PROCESS_SLOT_CLEAR_RUN_ONCE_PACKAGE: u32 = 0x214;
const PROCESS_SLOT_GET_CURRENT_PACKAGE: u32 = 0x22c;
const PROCESS_SLOT_CLEAR_CURRENT_PACKAGE: u32 = 0x234;
const PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX: u32 = 0x288;
const PROCESS_SLOT_GET_UP_ONE_HOUR: u32 = 0x2ac;
const PROCESS_SLOT_FINISH_DYING: u32 = 0x354;
const PROCESS_SLOT_SET_GREETING_TIMER: u32 = 0x4b4;
const PROCESS_SLOT_CONTINUING_PACKAGE_FOR_PC: u32 = 0x4e0;
const PROCESS_SLOT_GET_GENERIC_LOCATION: u32 = 0x514;
const PROCESS_SLOT_LOAD_PACKAGE_FROM_EXTRA_DATA: u32 = 0x714;
const PROCESS_SLOT_AMBUSH_WAIT: u32 = 0x7d4;
const PROCESS_SLOT_AVOID_AREA: u32 = 0x7e8;
const PROCESS_SLOT_COMBAT_ONE_HOUR: u32 = 0x81c;
const PROCESS_SLOT_RESET_TARGET: u32 = 0x82c;
const PROCESS_SLOT_0X848: u32 = 0x848;
/// Actor slots: `0x2e8` (true when the actor is dead or dying), `0x214` (the
/// word at +0x1ac), `0x28c(package, day)`, `0x2c4(angle)`, `0x2a8(point)`,
/// `0x16c(out)` (a pointer to a point; its +8 is read) and `0x2f4`.
const ACTOR_SLOT_IS_DYING: u32 = 0x2e8;
const ACTOR_SLOT_0X214: u32 = 0x214;
const ACTOR_SLOT_SET_PACKAGE_DAY: u32 = 0x28c;
const ACTOR_SLOT_SET_ANGLE_Z: u32 = 0x2c4;
const ACTOR_SLOT_SET_LOOK_POINT: u32 = 0x2a8;
const ACTOR_SLOT_GET_POINT: u32 = 0x16c;
/// Package slot `0x13c(actor, 0, -1.0, 0)` (a boolean test of the package for
/// the actor; the float is the global at `01012054`), `0x144(actor, mask)` and
/// `0x22c(1)`.
const PACKAGE_SLOT_TEST_0X13C: u32 = 0x13c;
const PACKAGE_SLOT_TEST_0X144: u32 = 0x144;
const PACKAGE_SLOT_TEST_0X22C: u32 = 0x22c;
/// The table of procedure kinds the package switch indexes, and the globals
/// `Update_ov2` reads.
const NO_TARGET_FLOAT: u32 = 0x0101_2054;
const REFERENCE_SEARCH_RADIUS: u32 = 0x0103_0020;
const CALENDAR_GET_DAY: u32 = 0x0086_7d60;
const DATA_HANDLER: u32 = 0x011c_3f2c;
const SAVED_FORM_GLOBAL: u32 = 0x011c_a248;
const PLAYER_LEVEL_SETTING: u32 = 0x011c_dad0;
/// Callbacks given to `TESDataHandler::EnumReferencesCloseToPoint`.
const ENUM_CALLBACK_FIRST: u32 = 0x0090_d480;
const ENUM_CALLBACK_SECOND: u32 = 0x0090_d570;

/// `TESDataHandler::EnumReferencesCloseToPoint` around `actor`: its location
/// is fetched twice (slot `0x1f4`), the radius is the float at `01030020`,
/// and `callback` runs for the references found.
fn enumerate_references_near(e: &mut Engine, actor: u32, callback: u32) {
    let radius = e.global::<f32>(REFERENCE_SEARCH_RADIUS);
    let first = e.vcall(actor, REFERENCE_SLOT_LOCATION, &args![]).u32();
    let second = e.vcall(actor, REFERENCE_SLOT_LOCATION, &args![]).u32();
    let cell = e.call(0x008d_6f30, &args![actor]).u32();
    let handler = e.mem.u32(DATA_HANDLER);
    e.call(
        0x0046_f280,
        &args![handler, cell, second, radius, first, radius, callback, actor],
    );
}

/// The height the package-location steps give to the actor (slot `0x2c4`):
/// the float at +8 of the location reference's `00430830` result or, without
/// a reference, of the point the actor's slot `0x16c` returns.
fn set_actor_facing(e: &mut Engine, actor: u32, location: u32) {
    let height = if location != 0 {
        let point = e.call(0x0043_0830, &args![location]).u32();
        e.mem.f32(point + 8)
    } else {
        e.with_stack(12, |e, out| {
            let point = e.vcall(actor, ACTOR_SLOT_GET_POINT, &args![out]).u32();
            e.mem.f32(point + 8)
        })
    };
    e.vcall(actor, ACTOR_SLOT_SET_ANGLE_Z, &args![height]);
}

// Translated from 008f4020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::Update_ov2` (Xbox PDB): the per-frame update of a process
/// whose `actor` (the argument, or null: nothing happens) is a high-process
/// actor. It clears `bGreetingFlag` (+0x32c) and `fGreetingTimer` (+0x330),
/// ends all sounds of type 4, zeroes `fDelayTimer` (+0x2d0) and takes the
/// running package. The actor is the argument when its slot `0x100` accepts
/// it. A dying actor (slot `0x2e8`) gets `DoDeathStuff` and `FinishDying`;
/// otherwise the greeting timer is reset (slot `0x4b4(0.0)`). The player's
/// greet flag is reset and the head-tracking targets updated. For a
/// persistent reference (`GetRefPersists`) the package is then run: when
/// `fScriptPackageEndTime` (+0x378) is set and passed the package is
/// re-evaluated; `CheckforNewPackage` is asked; and while the update has not
/// finished (`keep_going`) the running package's procedure kind (the table
/// at `011a3ff0` indexed by the package's word at +0x18 and the running
/// procedure index, slot `0x280`) selects what to do: the kinds 0xd, 0xe and
/// 0 position or path the actor, the others call the matching one-hour
/// process step (`ProcessFollowOneHour`, `ProcessSandman`, `ProcessCannibal`,
/// `ProcessAmbushWait`, `ProcessAvoidArea`, ...); unlisted kinds stop the
/// loop. Afterwards, for a process of type 0 that runs a procedure of kind
/// 0x36, the package is finished or abandoned (the actions are in the code
/// below). The actor's own process then gets slot `0x28`
/// (`ComputeLastTimeProcessed`).
///
/// One branch is dead in the game's code: the location holder that
/// case 0 consults is always null, so `PackageLocation::GetLocReference`
/// is never called from there; the call `0055b980` before it is kept.
pub fn high_process_update_ov2(e: &mut Engine, this: Ptr, actor: Ptr, delta: f32) {
    if actor.is_null() {
        return;
    }
    let base = this.addr();
    let mut package;
    e.mem.set_u8(base + 0x32c, 0);
    e.mem.set_f32(base + 0x330, 0.0);
    let audio = e.call(0x0045_3a70, &args![]).u32();
    e.call(0x00ad_8780, &args![audio, 4u32]);
    e.mem.set_f32(base + 0x2d0, 0.0);
    package = e.vcall(base, PROCESS_SLOT_RUNNING_PACKAGE, &args![]).u32();
    let mut actor_local = 0;
    if e.vcall(actor.addr(), REFERENCE_SLOT_IS_ACTOR, &args![])
        .bool()
    {
        actor_local = actor.addr();
    }
    if !e.vcall(actor_local, ACTOR_SLOT_IS_DYING, &args![]).bool() {
        e.vcall(base, PROCESS_SLOT_SET_GREETING_TIMER, &args![0.0f32]);
    } else {
        e.call(0x008b_01c0, &args![actor_local]);
        e.vcall(base, PROCESS_SLOT_FINISH_DYING, &args![actor_local]);
    }
    let player = e.mem.u32(PLAYER);
    e.call(0x0095_3ce0, &args![player]);
    let mut keep_going = true;
    e.call(0x0090_1550, &args![this]);
    if actor_local != 0 && e.call(0x0056_53d0, &args![actor_local]).bool() {
        if e.vcall(base, PROCESS_SLOT_GET_CURRENT_PACKAGE, &args![])
            .u32()
            != 0
        {
            let current = e
                .vcall(base, PROCESS_SLOT_GET_CURRENT_PACKAGE, &args![])
                .u32();
            e.call(0x0055_b980, &args![current]);
        }
        let mut location = 0;
        let mut rescheduled = false;
        let timer = e.mem.f32(base + 0x378);
        if f64::from(timer) != e.global::<f64>(0x0101_2060) {
            let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f32();
            let processed = e.call(0x0062_1b00, &args![base + 4]).f32();
            let difference = (f64::from(hour) - f64::from(processed)) as f32;
            let wrapped = e.call(0x0040_8840, &args![difference]).f32();
            let scaled = (f64::from(wrapped) * e.global::<f64>(0x0101_7a40)) as f32;
            if timer <= scaled {
                e.mem.set_f32(base + 0x2b4, 0.0);
                rescheduled = true;
                e.vcall(
                    base,
                    PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                    &args![actor_local, 3u32],
                );
                if package != 0 && e.call(0x0067_0f90, &args![package]).bool() {
                    let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8();
                    e.vcall(
                        actor_local,
                        ACTOR_SLOT_SET_PACKAGE_DAY,
                        &args![package, u32::from(day)],
                    );
                }
            }
        }
        let new_package = e
            .vcall(
                base,
                PROCESS_SLOT_CHECK_FOR_NEW_PACKAGE,
                &args![actor_local, u32::from(rescheduled)],
            )
            .u8();
        e.vcall(base, PROCESS_SLOT_RESET_TARGET, &args![]);
        package = e.vcall(base, PROCESS_SLOT_RUNNING_PACKAGE, &args![]).u32();
        if package != 0 && new_package != 0 {
            let count = e.call(0x0067_2930, &args![package]).u32();
            e.mem.set_u32(base + 0x58, count);
        }
        if package != 0 && e.call(0x0096_11e0, &args![package]).i32() != -1 {
            while keep_going {
                package = e.vcall(base, PROCESS_SLOT_RUNNING_PACKAGE, &args![]).u32();
                if package == 0 {
                    break;
                }
                let kind = e.call(0x0096_11e0, &args![package]).u32();
                let index = e.vcall(base, PROCESS_SLOT_PROCEDURE_INDEX, &args![]).u32();
                let row = e.mem.u32(PROCEDURE_TABLE + 4 * kind);
                let selector = e.mem.u32(row + 4 * index);
                match selector {
                    0 => {
                        // The location holder is never set (always null), so
                        // `PackageLocation::GetLocReference` is not reached.
                        if location == 0 {
                            location = e
                                .vcall(base, PROCESS_SLOT_GET_GENERIC_LOCATION, &args![])
                                .u32();
                        }
                        let done = e
                            .vcall(
                                package,
                                PACKAGE_SLOT_TEST_0X13C,
                                &args![actor_local, 0u32, e.global::<f32>(NO_TARGET_FLOAT), 0u32],
                            )
                            .bool();
                        if done {
                            e.vcall(
                                base,
                                PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                                &args![actor_local, 1u32],
                            );
                            keep_going = false;
                            let saved_form = if location != 0 {
                                e.call(0x007a_f430, &args![location]).u32()
                            } else {
                                0
                            };
                            let at_saved =
                                location != 0 && saved_form == e.mem.u32(SAVED_FORM_GLOBAL);
                            let mut set_height = at_saved;
                            if !at_saved {
                                let package_location = e.call(0x0055_b980, &args![package]).u32();
                                if package_location != 0 {
                                    let location_object =
                                        e.call(0x0055_b980, &args![package]).u32();
                                    if e.call(0x0067_8ca0, &args![location_object]).i32() == 3
                                        && !e.call(0x0091_5ef0, &args![this, actor_local]).bool()
                                    {
                                        set_height = true;
                                    }
                                }
                            }
                            if set_height {
                                set_actor_facing(e, actor_local, location);
                            }
                        } else if !e.call(0x008b_3bb0, &args![actor_local]).bool() {
                            keep_going = false;
                        } else {
                            update_case_0_travel(e, this, package, actor_local);
                            keep_going = false;
                        }
                    }
                    1 => {
                        e.call(0x008f_36c0, &args![this, actor_local, delta]);
                        keep_going = false;
                    }
                    2 => {
                        keep_going = e
                            .vcall(
                                base,
                                PROCESS_SLOT_PROCESS_ACTIVATE_ONE_HOUR,
                                &args![actor_local, delta],
                            )
                            .bool();
                    }
                    3 => {
                        get_up_if_needed(e, base, actor_local);
                        e.call(0x008f_3fe0, &args![this, actor_local, delta]);
                        keep_going = false;
                    }
                    4 => {
                        keep_going = e.call(0x008f_3550, &args![this, actor_local, delta]).bool();
                    }
                    5 => {
                        keep_going = e.call(0x008f_3600, &args![this, actor_local, delta]).bool();
                    }
                    6 => {
                        get_up_if_needed(e, base, actor_local);
                        keep_going = e
                            .call(0x008f_31d0, &args![this, actor_local, delta, 1u32])
                            .bool();
                    }
                    7 => {
                        get_up_if_needed(e, base, actor_local);
                        keep_going = e.call(0x008f_2610, &args![this, actor_local, delta]).bool();
                    }
                    8 => {
                        get_up_if_needed(e, base, actor_local);
                        keep_going = false;
                    }
                    9 => {
                        get_up_if_needed(e, base, actor_local);
                        keep_going = e
                            .vcall(
                                base,
                                PROCESS_SLOT_COMBAT_ONE_HOUR,
                                &args![actor_local, delta],
                            )
                            .bool();
                    }
                    0xd => {
                        if e.mem.u32(base + TARGET) == 0 {
                            e.vcall(
                                base,
                                PROCESS_SLOT_SET_TARGET_FOR_PACKAGE,
                                &args![actor_local],
                            );
                        }
                        if e.mem.u32(base + TARGET) == 0 {
                            e.vcall(
                                base,
                                PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                                &args![actor_local, 1u32],
                            );
                        } else if e.call(PACKAGE_TYPE, &args![package]).i32() == 1 {
                            let player = e.mem.u32(PLAYER);
                            let level = e.call(0x0096_2620, &args![player]).i32();
                            let setting = e.call(0x0043_d4d0, &args![PLAYER_LEVEL_SETTING]).u32();
                            if level < e.mem.i32(setting) {
                                e.vcall(
                                    base,
                                    PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                                    &args![actor_local, 1u32],
                                );
                            } else {
                                keep_going = false;
                            }
                        } else {
                            let target = e.mem.u32(base + TARGET);
                            if e.call(0x0044_0d80, &args![target]).bool()
                                || e.call(0x0044_0da0, &args![target]).bool()
                            {
                                if e.call(0x0044_0d80, &args![target]).bool() {
                                    e.call(0x0067_4e70, &args![package, target, 1u32]);
                                }
                                e.vcall(
                                    base,
                                    PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                                    &args![actor_local, 1u32],
                                );
                                return;
                            }
                            if e.vcall(target, PACKAGE_SLOT_TEST_0X22C, &args![1u32])
                                .bool()
                            {
                                e.call(0x0067_4e70, &args![package, target, 1u32]);
                                return;
                            }
                            let radius = e.call(0x0067_6280, &args![package, actor_local]).u32();
                            let mask: u32 = if radius == 0 { 0x190 } else { 0 };
                            if e.vcall(package, PACKAGE_SLOT_TEST_0X144, &args![actor_local, mask])
                                .bool()
                            {
                                e.vcall(
                                    base,
                                    PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                                    &args![actor_local, 1u32],
                                );
                            } else {
                                keep_going = false;
                            }
                        }
                    }
                    0xe => {
                        if e.mem.u32(base + TARGET) == 0 {
                            e.vcall(
                                base,
                                PROCESS_SLOT_SET_TARGET_FOR_PACKAGE,
                                &args![actor_local],
                            );
                        }
                        if e.mem.u32(base + TARGET) == 0 {
                            e.vcall(
                                base,
                                PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                                &args![actor_local, 1u32],
                            );
                            if !e.call(0x008b_3bb0, &args![actor_local]).bool() {
                                e.vcall(base, PROCESS_SLOT_END_MOVE_MESSAGE, &args![actor_local]);
                            }
                        }
                        e.vcall(
                            base,
                            PROCESS_SLOT_PROCESS_ACTIVATE_ONE_HOUR,
                            &args![actor_local, delta],
                        );
                        keep_going = false;
                    }
                    0xf => {
                        e.call(0x0090_dc60, &args![this, actor_local, 0u32, 1u32, 0x101u32]);
                        keep_going = false;
                    }
                    0x11 => {
                        e.call(0x008d_b4f0, &args![this, actor_local]);
                        keep_going = false;
                    }
                    0x16 => {
                        e.vcall(base, PROCESS_SLOT_GET_UP_ONE_HOUR, &args![actor_local]);
                        e.vcall(
                            base,
                            PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                            &args![actor_local, 1u32],
                        );
                    }
                    0x1b => {
                        e.call(0x008e_0ad0, &args![this, actor_local, delta]);
                        keep_going = false;
                    }
                    0x1d => {
                        e.vcall(base, PROCESS_SLOT_PROCESS_SANDMAN, &args![actor_local]);
                        keep_going = false;
                    }
                    0x1e => {
                        e.vcall(base, PROCESS_SLOT_AMBUSH_WAIT, &args![actor_local]);
                        keep_going = false;
                    }
                    0x1f => {
                        e.call(0x008e_eac0, &args![this, actor_local]);
                    }
                    0x23 => {
                        e.vcall(base, PROCESS_SLOT_0X848, &args![actor_local, 1u32, 0u32]);
                        keep_going = false;
                    }
                    0x27 => {
                        e.vcall(base, PROCESS_SLOT_AVOID_AREA, &args![actor_local]);
                        keep_going = false;
                    }
                    0x2c => {
                        get_up_if_needed(e, base, actor_local);
                        e.call(0x008f_4000, &args![this, actor_local, delta]);
                        keep_going = false;
                    }
                    0x34 => {
                        e.vcall(base, PROCESS_SLOT_PROCESS_CANNIBAL, &args![actor_local]);
                        keep_going = false;
                    }
                    // 0xc, 0x19, 0x1a, 0x1c, 0x21, 0x2f, 0x32, 0x36 and every
                    // kind without a case end the loop.
                    _ => keep_going = false,
                }
            }
        }
    }
    if e.call(0x0093_1850, &args![actor_local]).i32() == 0 {
        let package = e.vcall(base, PROCESS_SLOT_RUNNING_PACKAGE, &args![]).u32();
        if package != 0 {
            let kind = e.call(0x0096_11e0, &args![package]).u32();
            let index = e.vcall(base, PROCESS_SLOT_PROCEDURE_INDEX, &args![]).u32();
            let row = e.mem.u32(PROCEDURE_TABLE + 4 * kind);
            if e.mem.u32(row + 4 * index) == 0x36
                && !finish_package_procedure(e, this, package, actor_local)
            {
                return;
            }
        }
    }
    let actor_process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    e.vcall(
        actor_process,
        PROCESS_SLOT_COMPUTE_LAST_TIME_PROCESSED,
        &args![],
    );
}

/// Calls the process's slot `0x2ac(actor)` when the actor's slot `0x214`
/// returns an object (`ProcessGetUpOneHour` before the one-hour steps).
fn get_up_if_needed(e: &mut Engine, process: u32, actor: u32) {
    if e.vcall(actor, ACTOR_SLOT_0X214, &args![]).u32() != 0 {
        e.vcall(process, PROCESS_SLOT_GET_UP_ONE_HOUR, &args![actor]);
    }
}

/// The travel part of `Update_ov2` case 0 (from `008f44a2`), reached when the
/// package test (slot `0x13c`) is false and the actor's pathing is complete:
/// the destination reference is the package location's reference or the
/// process's `pGenericLocation` (+0x44) when the run-once package is unset;
/// an actor with a current package that fails the test gets slot `0x2ac`;
/// the pathfinding goal is set from the package (`SetPathfindingGoal_ov2`
/// with its coordinates, cell, world space and radius); then, if the process
/// has no saved acquire object (`0045cd60`) and the package test passes, the
/// reference search callbacks run for packages with flag `0x20` or `0x100`
/// (`00670f40`, `00670f60`), the locked location is set, the procedure index
/// advances and the actor's height or look point is set.
fn update_case_0_travel(e: &mut Engine, this: Ptr, package: u32, actor: u32) {
    let base = this.addr();
    let mut destination = 0;
    if e.call(0x0055_b980, &args![package]).u32() != 0 {
        let package_location = e.call(0x0055_b980, &args![package]).u32();
        destination = e.call(0x0067_f390, &args![package_location]).u32();
    }
    if e.mem.u32(base + 0x44) != 0
        && e.vcall(base, PROCESS_SLOT_GET_RUN_ONCE_PACKAGE, &args![])
            .u32()
            == 0
    {
        destination = e.mem.u32(base + 0x44);
    }
    let no_target = e.global::<f32>(NO_TARGET_FLOAT);
    if e.vcall(actor, ACTOR_SLOT_0X214, &args![]).u32() != 0 {
        let current = e.call(0x0093_44a0, &args![actor]).u32();
        if !e
            .vcall(
                current,
                PACKAGE_SLOT_TEST_0X13C,
                &args![actor, 0u32, no_target, 0u32],
            )
            .bool()
        {
            e.vcall(base, PROCESS_SLOT_GET_UP_ONE_HOUR, &args![actor]);
        }
    }
    let radius = e.call(0x0067_8670, &args![package, actor, 0u32]).f32();
    let world = e.call(0x0067_5a50, &args![package, actor]).u32();
    let cell = e.call(0x0067_5c20, &args![package, actor]).u32();
    // The frame of `008f4020` (`SUB ESP,0xb0` under `EBP`, then `ESI`): the goal at `EBP-0x64`
    // (+0x50).
    e.with_stack(0xb4, |e, frame| {
        let goal = frame.addr() + 0x50;
        let coordinates = e.call(0x0067_5de0, &args![package, goal, actor]).u32();
        e.call(
            0x008b_3690,
            &args![actor, coordinates, cell, world, radius, 0u32],
        );
    });
    let saved_acquire_object = e.call(ACTOR_PROCESS, &args![actor]).u32();
    if e.call(0x0045_cd60, &args![saved_acquire_object]).u32() != 0 {
        return;
    }
    if !e
        .vcall(
            package,
            PACKAGE_SLOT_TEST_0X13C,
            &args![actor, 0u32, no_target, 0u32],
        )
        .bool()
    {
        return;
    }
    if !e
        .vcall(base, PROCESS_SLOT_GET_LOCKED_LOCATION, &args![])
        .bool()
    {
        if package != 0 && e.call(0x0067_0f40, &args![package]).bool() {
            enumerate_references_near(e, actor, ENUM_CALLBACK_FIRST);
        }
        e.vcall(base, PROCESS_SLOT_SET_LOCKED_LOCATION, &args![1u32]);
    }
    if package != 0 && e.call(0x0067_0f60, &args![package]).bool() {
        enumerate_references_near(e, actor, ENUM_CALLBACK_SECOND);
    }
    e.vcall(
        base,
        PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
        &args![actor, 1u32],
    );
    let at_saved_form = destination != 0
        && e.call(0x007a_f430, &args![destination]).u32() == e.mem.u32(SAVED_FORM_GLOBAL);
    let mut reached = at_saved_form;
    if !at_saved_form && e.call(0x0055_b980, &args![package]).u32() != 0 {
        let package_location = e.call(0x0055_b980, &args![package]).u32();
        if e.call(0x0067_8ca0, &args![package_location]).i32() == 3 {
            reached = true;
        }
    }
    if reached && !e.call(0x0091_5ef0, &args![this, actor]).bool() {
        set_actor_facing(e, actor, destination);
        return;
    }
    if e.call(0x0067_6280, &args![package, actor]).u32() != 0 {
        let radius = e.call(0x0067_6280, &args![package, actor]).u32() as f32;
        // The same frame: the look point at `EBP-0x40` (+0x74), the coordinates at `EBP-0x7c`
        // (+0x38).
        e.with_stack(0xb4, |e, frame| {
            let look = frame.addr() + 0x74;
            let scratch = frame.addr() + 0x38;
            let coordinates = e.call(0x0067_5de0, &args![package, scratch, actor]).u32();
            let (x, y, z) = (
                e.mem.u32(coordinates),
                e.mem.u32(coordinates + 4),
                e.mem.u32(coordinates + 8),
            );
            if e.call(0x008e_3e90, &args![actor, x, y, z, radius, look])
                .bool()
            {
                e.vcall(actor, ACTOR_SLOT_SET_LOOK_POINT, &args![look]);
            }
        });
    }
}

/// The part of `Update_ov2` after the package loop for a process of type 0
/// that runs a procedure of kind 0x36 (`package` is the running package).
/// Returns false when the function returns at once, true when it goes on to
/// the final `ComputeLastTimeProcessed` call. It clears
/// `fScriptPackageEndTime` (+0x378) when no run-once package is set; ends the
/// procedure (index -1) when the package test fails for a kind-0 package, for
/// kind 3 and for type codes 3 and 4; ends pathing; with
/// `bContinuingPackageforPC` (+0x374) it also clears the action-complete flag
/// and returns; sets the day of a once-per-day package; and, unless the
/// package has a successor (the list at `0041d8a0`), clears the target,
/// replaces or clears the running package, resets the package timer
/// (+0x2b4) for once-per-day and similar packages, and releases the acquire
/// objects (+0x64, +0x34, the list at +0x5c), the generic location (+0x44) and
/// the list at +0x6c.
fn finish_package_procedure(e: &mut Engine, this: Ptr, package: u32, actor: u32) -> bool {
    let base = this.addr();
    let no_target = e.global::<f32>(NO_TARGET_FLOAT);
    if e.vcall(base, PROCESS_SLOT_GET_RUN_ONCE_PACKAGE, &args![])
        .u32()
        == 0
    {
        e.mem.set_f32(base + 0x378, 0.0);
    }
    if e.call(0x0096_11e0, &args![package]).u32() == 0 {
        if !e
            .vcall(
                package,
                PACKAGE_SLOT_TEST_0X13C,
                &args![actor, 0u32, no_target, 0u32],
            )
            .bool()
        {
            e.vcall(
                base,
                PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                &args![actor, 0xffff_ffffu32],
            );
            return false;
        }
    } else {
        if e.call(0x0096_11e0, &args![package]).u32() == 3 {
            e.vcall(
                base,
                PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                &args![actor, 0xffff_ffffu32],
            );
            return false;
        }
        if e.call(PACKAGE_TYPE, &args![package]).i32() == 3
            || e.call(PACKAGE_TYPE, &args![package]).i32() == 4
        {
            e.vcall(
                base,
                PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                &args![actor, 0xffff_ffffu32],
            );
            return false;
        }
    }
    e.vcall(base, PROCESS_SLOT_END_MOVE_MESSAGE, &args![actor]);
    if e.mem.u8(base + 0x374) != 0 {
        e.vcall(
            base,
            PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
            &args![actor, 0xffff_ffffu32],
        );
        e.vcall(base, PROCESS_SLOT_SET_CURRENT_ACTION_COMPLETE, &args![0u32]);
        return false;
    }
    if package != 0 && e.call(0x0067_0f90, &args![package]).bool() {
        let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8();
        e.vcall(
            actor,
            ACTOR_SLOT_SET_PACKAGE_DAY,
            &args![package, u32::from(day)],
        );
    }
    let successors = e.call(0x0041_d8a0, &args![package]).u32();
    if list_next(e, successors) != 0 {
        return true;
    }
    e.vcall(base, PROCESS_SLOT_SET_TARGET, &args![0u32]);
    if e.vcall(base, PROCESS_SLOT_GET_RUN_ONCE_PACKAGE, &args![])
        .u32()
        == 0
        || e.vcall(base, PROCESS_SLOT_CONTINUING_PACKAGE_FOR_PC, &args![])
            .bool()
    {
        let current = e
            .vcall(base, PROCESS_SLOT_GET_CURRENT_PACKAGE, &args![])
            .u32();
        if e.call(0x0067_4d40, &args![current]).bool() {
            let current = e
                .vcall(base, PROCESS_SLOT_GET_CURRENT_PACKAGE, &args![])
                .u32();
            if e.call(0x0067_8610, &args![current]).bool() {
                e.vcall(
                    base,
                    PROCESS_SLOT_LOAD_PACKAGE_FROM_EXTRA_DATA,
                    &args![actor],
                );
            } else {
                e.vcall(base, PROCESS_SLOT_CLEAR_CURRENT_PACKAGE, &args![]);
            }
            if !e.call(0x008b_3bb0, &args![actor]).bool() {
                e.vcall(base, PROCESS_SLOT_END_MOVE_MESSAGE, &args![actor]);
            }
        }
    } else {
        e.vcall(base, PROCESS_SLOT_CLEAR_RUN_ONCE_PACKAGE, &args![]);
    }
    if e.vcall(base, PROCESS_SLOT_GET_CURRENT_PACKAGE, &args![])
        .u32()
        != 0
    {
        let mut reset = false;
        let current = e
            .vcall(base, PROCESS_SLOT_GET_CURRENT_PACKAGE, &args![])
            .u32();
        if e.call(0x0067_0f90, &args![current]).bool() {
            reset = true;
        } else {
            let current = e
                .vcall(base, PROCESS_SLOT_GET_CURRENT_PACKAGE, &args![])
                .u32();
            if e.call(0x008b_1ff0, &args![current]).bool() {
                reset = true;
            } else {
                let current = e
                    .vcall(base, PROCESS_SLOT_GET_CURRENT_PACKAGE, &args![])
                    .u32();
                if e.call(0x0067_efd0, &args![current]).bool() {
                    reset = true;
                }
            }
        }
        if reset {
            e.mem.set_f32(base + 0x2b4, 0.0);
        }
    }
    let acquire = e.mem.u32(base + 0x64);
    if acquire != 0 {
        e.call(0x007b_3fa0, &args![acquire, 1u32]);
    }
    e.mem.set_u32(base + 0x64, 0);
    e.mem.set_u32(base + 0x34, 0);
    let objects = base + 0x5c;
    while !e.call(LIST_IS_EMPTY, &args![objects]).bool() {
        let item = list_item(e, objects);
        if item != 0 {
            e.call(0x007b_3fa0, &args![item, 1u32]);
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            e.call(LIST_REMOVE_ITEM, &args![objects, slot]);
        });
    }
    e.mem.set_f32(base + 0x294, 0.0);
    e.mem.set_u32(base + 0x44, 0);
    e.call(LIST_CLEAR, &args![base + 0x6c]);
    true
}

/// `BGSSaveGameBuffer::Save(buffer, data, size, 0)` (`00865e50`),
/// `SaveFormID_ov2(buffer, form, 0)` (`00865df0`) and the variable-sized
/// value pair `StartVariableSizedValue` (`00865f20`, returns the position)
/// and `SaveVariableSizedValue_ov2(buffer, count, position)` (`00865ff0`).
const BUFFER_SAVE_BYTES: u32 = 0x0086_5e50;
const BUFFER_SAVE_FORM_ID: u32 = 0x0086_5df0;
const BUFFER_START_SIZED_VALUE: u32 = 0x0086_5f20;
const BUFFER_END_SIZED_VALUE: u32 = 0x0086_5ff0;
/// `0084e3a0(buffer)`: the word at +0x0c of the buffer (its position).
const BUFFER_POSITION: u32 = 0x0084_e3a0;

/// The fixed fields `SaveGame_ov2` writes first, as (offset in the process,
/// size in bytes), in the order the code writes them.
const SAVE_OV2_FIELDS: [(u32, u32); 63] = [
    (0x32c, 1),
    (0x340, 1),
    (0x374, 1),
    (0x375, 1),
    (0x2fc, 2),
    (0x2b4, 4),
    (0x2f8, 4),
    (0x310, 4),
    (0x330, 4),
    (0x334, 4),
    (0x338, 4),
    (0x34c, 4),
    (0x294, 4),
    (0x2b8, 4),
    (0x2bc, 4),
    (0x298, 4),
    (0x2c0, 2),
    (0x2c2, 2),
    (0x2c4, 2),
    (0x349, 1),
    (0x300, 12),
    (0x36c, 4),
    (0x3e8, 4),
    (0x3ec, 4),
    (0x33c, 4),
    (0x2a8, 4),
    (0x378, 4),
    (0x3a0, 1),
    (0x39c, 4),
    (0x3a8, 1),
    (0x3a4, 4),
    (0x420, 1),
    (0x3bc, 4),
    (0x3c0, 4),
    (0x2c6, 1),
    (0x2d0, 4),
    (0x2d4, 4),
    (0x2d8, 4),
    (0x3b8, 1),
    (0x2dc, 1),
    (0x2e0, 4),
    (0x344, 4),
    (0x2dc, 1),
    (0x2dc, 1),
    (0x3d8, 4),
    (0x448, 4),
    (0x29d, 1),
    (0x2b0, 4),
    (0x2c8, 4),
    (0x418, 4),
    (0x43c, 4),
    (0x440, 4),
    (0x444, 1),
    (0x445, 1),
    (0x450, 4),
    (0x458, 1),
    (0x430, 4),
    (0x3e0, 1),
    (0x459, 1),
    (0x2a0, 4),
    (0x3d0, 1),
    (0x3d1, 1),
    (0x348, 1),
];

/// Writes the form IDs of a list's items as a counted run: the count is
/// patched in by `SaveVariableSizedValue_ov2`. `list` is the first node (or
/// 0); items that are null are skipped and not counted.
fn save_form_id_run(e: &mut Engine, buffer: u32, list: u32) {
    let start = e.call(BUFFER_START_SIZED_VALUE, &args![buffer]).u32();
    let mut count = 0u32;
    let mut node = list;
    while node != 0 {
        let form = list_item(e, node);
        if form != 0 {
            e.call(BUFFER_SAVE_FORM_ID, &args![buffer, form, 0u32]);
            count += 1;
        }
        node = list_next(e, node);
    }
    e.call(BUFFER_END_SIZED_VALUE, &args![buffer, count, start]);
}

/// Like [`save_form_id_run`] but each non-null item is written by the
/// function at `saver` (`(item, buffer)`).
fn save_item_run(e: &mut Engine, buffer: u32, list: u32, saver: u32) {
    let start = e.call(BUFFER_START_SIZED_VALUE, &args![buffer]).u32();
    let mut count = 0u32;
    let mut node = list;
    while node != 0 {
        let item = list_item(e, node);
        if item != 0 {
            e.call(saver, &args![item, buffer]);
            count += 1;
        }
        node = list_next(e, node);
    }
    e.call(BUFFER_END_SIZED_VALUE, &args![buffer, count, start]);
}

// Translated from 008fc4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SaveGame_ov2` (Xbox PDB): the save of the process through a
/// `BGSSaveGameBuffer`. `MiddleHighProcess::SaveGame_ov2(buffer)` goes first;
/// then the fixed fields (`SAVE_OV2_FIELDS`, one `Save(buffer, field, size,
/// 0)` each), the byte "`pLipSynicAnim` (+0x3cc) is set", the form IDs of
/// `pGreetActor` (+0x30c), `plastDetected` (+0x2a4), `pTeleportFadeRef`
/// (+0x3f0), `pLastTarget` (+0x41c), `pDialogTarget` (+0x370), `pIdleToPlay`
/// (+0x350) and `pPathLookAtTarget` (+0x2ac), the six head-tracking targets
/// (+0x3f8) each followed by its flag byte (+0x410), then five counted runs:
/// the form IDs of `AggroRadiusList` (+0x38c), `AvoidActorList` (+0x394) and of `pLastSpokeToList`
/// (+0x264), the byte "`pGreetTopic` (+0x368) is set" with that
/// `DialogueItem::SaveGame_ov2`, `pListOfAvoidAreas` (+0x44c, saved with
/// `008d73b0`), `pDetectedActorList` (+0x25c) and `pActorsWhoDetectMeList`
/// (+0x260) (both with `DetectionState::SaveGame`), the byte
/// "`pActorsGeneratedDetectionEvent` (+0x3dc) is set" with its
/// `DetectionEvent::SaveGame`, and the character controller
/// (`SaveCharController_ov2`) as a variable-sized value of its written length.
pub fn high_process_save_game_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0092_6a20, &args![this, buffer]);
    for (offset, size) in SAVE_OV2_FIELDS {
        e.call(
            BUFFER_SAVE_BYTES,
            &args![buffer, this.addr() + offset, size, 0u32],
        );
    }
    let lip_sync_set = e.mem.u32(this.addr() + 0x3cc) != 0;
    save_flag_byte(e, buffer, u8::from(lip_sync_set));
    for offset in [0x30c, 0x2a4, 0x3f0, 0x41c, 0x370, 0x350, 0x2ac] {
        let form = e.mem.u32(this.addr() + offset);
        e.call(BUFFER_SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
    for index in 0..6 {
        let form = e.mem.u32(this.addr() + HEAD_TRACKING_TARGETS + 4 * index);
        e.call(BUFFER_SAVE_FORM_ID, &args![buffer, form, 0u32]);
        e.call(
            BUFFER_SAVE_BYTES,
            &args![
                buffer,
                this.addr() + HEAD_TRACKING_TARGET_FLAGS + index,
                1u32,
                0u32
            ],
        );
    }
    save_form_id_run(e, buffer, this.addr() + 0x38c);
    save_form_id_run(e, buffer, this.addr() + 0x394);
    let spoken_to = e.mem.u32(this.addr() + 0x264);
    save_form_id_run(e, buffer, spoken_to);
    let topic = e.mem.u32(this.addr() + 0x368);
    save_flag_byte(e, buffer, u8::from(topic != 0));
    if topic != 0 {
        e.call(0x0083_ce40, &args![topic, buffer]);
    }
    let avoid_areas = e.mem.u32(this.addr() + 0x44c);
    save_item_run(e, buffer, avoid_areas, 0x008d_73b0);
    let detected = e.mem.u32(this.addr() + DETECTED_ACTOR_LIST);
    save_item_run(e, buffer, detected, 0x008d_7070);
    let who_detects_me = e.mem.u32(this.addr() + WHO_DETECTS_ME_LIST);
    save_item_run(e, buffer, who_detects_me, 0x008d_7070);
    let event = e.mem.u32(this.addr() + 0x3dc);
    save_flag_byte(e, buffer, u8::from(event != 0));
    if event != 0 {
        e.call(0x008d_7270, &args![event, buffer]);
    }
    let start = e.call(BUFFER_START_SIZED_VALUE, &args![buffer]).u32();
    let before = e.call(BUFFER_POSITION, &args![buffer]).u32();
    e.call(0x0092_8880, &args![this, buffer]);
    let after = e.call(BUFFER_POSITION, &args![buffer]).u32();
    e.call(
        BUFFER_END_SIZED_VALUE,
        &args![buffer, after.wrapping_sub(before), start],
    );
}

/// Writes one byte (a flag the game keeps in a stack local) to the buffer.
fn save_flag_byte(e: &mut Engine, buffer: u32, value: u8) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u8(slot.addr(), value);
        e.call(BUFFER_SAVE_BYTES, &args![buffer, slot, 1u32, 0u32]);
    });
}

/// `BGSLoadGameBuffer::Load(buffer, destination, size)` (`00864980`),
/// `LoadFormID(buffer)` (`008648a0`, the ID read), `LoadFormID_ov2(buffer,
/// destination)` (`008648e0`) and `LoadVariableSizedValue(buffer)`
/// (`00864a60`, the entry count).
const BUFFER_LOAD_BYTES: u32 = 0x0086_4980;
const BUFFER_LOAD_FORM_ID: u32 = 0x0086_48a0;
const BUFFER_LOAD_FORM_ID_OV2: u32 = 0x0086_48e0;
const BUFFER_LOAD_SIZED_VALUE: u32 = 0x0086_4a60;
/// `BGSSaveLoadGame::QueueSubBuffer(this, 1, buffer, 0)` (`0084a810`) on the
/// object at `011ddf38`.
const QUEUE_SUB_BUFFER: u32 = 0x0084_a810;
const SAVE_LOAD_QUEUE: u32 = 0x011d_df38;

/// The fixed fields `LoadGame_ov2` reads between the version-8 check and the
/// version-1 check, as (offset in the process, size in bytes), in order. The
/// word at +0x3d8 is reduced modulo 12 right after it is read; the function
/// does that, not this table.
const LOAD_OV2_FIELDS: [(u32, u32); 52] = [
    (0x2b4, 0x4),
    (0x2f8, 0x4),
    (0x310, 0x4),
    (0x330, 0x4),
    (0x334, 0x4),
    (0x338, 0x4),
    (0x34c, 0x4),
    (0x294, 0x4),
    (0x2b8, 0x4),
    (0x2bc, 0x4),
    (0x298, 0x4),
    (0x2c0, 0x2),
    (0x2c2, 0x2),
    (0x2c4, 0x2),
    (0x349, 0x1),
    (0x300, 0xc),
    (0x36c, 0x4),
    (0x3e8, 0x4),
    (0x3ec, 0x4),
    (0x33c, 0x4),
    (0x2a8, 0x4),
    (0x378, 0x4),
    (0x3a0, 0x1),
    (0x39c, 0x4),
    (0x3a8, 0x1),
    (0x3a4, 0x4),
    (0x420, 0x1),
    (0x3bc, 0x4),
    (0x3c0, 0x4),
    (0x2c6, 0x1),
    (0x2d0, 0x4),
    (0x2d4, 0x4),
    (0x2d8, 0x4),
    (0x3b8, 0x1),
    (0x2dc, 0x1),
    (0x2e0, 0x4),
    (0x344, 0x4),
    (0x2dc, 0x1),
    (0x2dc, 0x1),
    (0x3d8, 0x4),
    (0x448, 0x4),
    (0x29d, 0x1),
    (0x2b0, 0x4),
    (0x2c8, 0x4),
    (0x418, 0x4),
    (0x43c, 0x4),
    (0x440, 0x4),
    (0x444, 0x1),
    (0x445, 0x1),
    (0x450, 0x4),
    (0x458, 0x1),
    (0x430, 0x4),
];

/// Reads `size` bytes from the buffer into `this + offset`.
fn buffer_load_field(e: &mut Engine, buffer: u32, this: Ptr, offset: u32, size: u32) {
    e.call(
        BUFFER_LOAD_BYTES,
        &args![buffer, this.addr() + offset, size],
    );
}

/// Reads `size` bytes from the buffer into a scratch local and returns the
/// first byte (a flag or a value the game keeps in a stack local).
fn buffer_load_discard(e: &mut Engine, buffer: u32, size: u32) -> u8 {
    e.with_stack(4, |e, slot| {
        e.call(BUFFER_LOAD_BYTES, &args![buffer, slot, size]);
        e.mem.u8(slot.addr())
    })
}

/// Reads a form ID into `destination` (`LoadFormID_ov2`).
fn buffer_load_form_id_into(e: &mut Engine, buffer: u32, destination: u32) {
    e.call(BUFFER_LOAD_FORM_ID_OV2, &args![buffer, destination]);
}

/// Reads a counted run of form IDs and appends each to `list` (a node
/// address or the pointer the process keeps).
fn buffer_load_form_id_run(e: &mut Engine, buffer: u32, list: u32, slot: u32) {
    let count = e.call(BUFFER_LOAD_SIZED_VALUE, &args![buffer]).u32();
    for _ in 0..count {
        e.mem.set_u32(slot, 0);
        buffer_load_form_id_into(e, buffer, slot);
        e.call(LIST_APPEND, &args![list, slot]);
    }
}

/// Reads a counted run of items: for each, an object of `size` bytes is
/// allocated and built by `constructor` (a null allocation stays null),
/// loaded by `loader(item, buffer)` and appended to `list`.
fn buffer_load_item_run(
    e: &mut Engine,
    buffer: u32,
    list: u32,
    size: u32,
    constructor: u32,
    loader: u32,
) {
    let count = e.call(BUFFER_LOAD_SIZED_VALUE, &args![buffer]).u32();
    for _ in 0..count {
        let block = e.call(OPERATOR_NEW, &args![size]).u32();
        let item = if block == 0 {
            0
        } else {
            e.call(constructor, &args![block]).u32()
        };
        e.call(loader, &args![item, buffer]);
        list_append(e, list, item);
    }
}

// Translated from 008fce90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::LoadGame_ov2` (Xbox PDB): the inverse of `SaveGame_ov2`
/// (`BGSLoadGameBuffer`; the buffer's slot 0 gives the save version).
/// `MiddleHighProcess::LoadGame_ov2(buffer)` goes first; then the fields of
/// `LOAD_OV2_FIELDS` and the fields of newer versions: a 4-byte field that
/// versions below 8 carry is read and dropped; version 1 adds +0x3e0, version
/// 0xa +0x459, version 0x12 +0x2a0 and the `pLastSpokeToList` run, version
/// 0x13 the bytes +0x3d0, +0x3d1 and +0x348 and a flag which, like a set
/// +0x3d0, sets +0x3d1 and clears +0x3d0. The form IDs of +0x30c, +0x2a4,
/// +0x3f0, +0x41c and +0x370 follow, then `pIdleToPlay` (+0x350, resolved to
/// the class at `01186a18`), +0x2ac from version 0xe and the head-tracking
/// targets with their flags (five below version 4, else six). Then come the
/// counted runs of form IDs for the lists at +0x38c and +0x394, the
/// optional `DialogueItem` (+0x368), the avoid areas (+0x44c, whose list is
/// emptied or created when the run is not empty), the `DetectionState`s for
/// +0x25c and +0x260 and the optional `DetectionEvent` (+0x3dc). The buffer is
/// finally queued with `QueueSubBuffer`.
pub fn high_process_load_game_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0092_7000, &args![this, buffer]);
    let version = |e: &mut Engine| -> u8 { e.vcall(buffer, 0, &args![]).u8() };
    for (offset, size) in [(0x32c, 1), (0x340, 1), (0x374, 1), (0x375, 1), (0x2fc, 2)] {
        buffer_load_field(e, buffer, this, offset, size);
    }
    if version(e) < 8 {
        buffer_load_discard(e, buffer, 4);
    }
    for (offset, size) in LOAD_OV2_FIELDS {
        buffer_load_field(e, buffer, this, offset, size);
        if offset == 0x3d8 {
            let value = e.mem.u32(this.addr() + 0x3d8);
            e.mem.set_u32(this.addr() + 0x3d8, value % 12);
        }
    }
    if version(e) >= 1 {
        buffer_load_field(e, buffer, this, 0x3e0, 1);
    }
    if version(e) >= 0xa {
        buffer_load_field(e, buffer, this, 0x459, 1);
    }
    if version(e) >= 0x12 {
        buffer_load_field(e, buffer, this, 0x2a0, 4);
    }
    if version(e) >= 0x13 {
        buffer_load_field(e, buffer, this, 0x3d0, 1);
        buffer_load_field(e, buffer, this, 0x3d1, 1);
        buffer_load_field(e, buffer, this, 0x348, 1);
        let flag = buffer_load_discard(e, buffer, 1);
        if e.mem.u8(this.addr() + 0x3d0) != 0 || flag != 0 {
            e.mem.set_u8(this.addr() + 0x3d0, 0);
            e.mem.set_u8(this.addr() + 0x3d1, 1);
        }
    }
    for offset in [0x30c, 0x2a4, 0x3f0, 0x41c, 0x370] {
        buffer_load_form_id_into(e, buffer, this.addr() + offset);
    }
    let idle_id = e.call(BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
    let idle = form_cast_to(e, idle_id, 0x0118_6a18);
    e.mem.set_u32(this.addr() + 0x350, idle);
    if version(e) >= 0xe {
        buffer_load_form_id_into(e, buffer, this.addr() + 0x2ac);
    }
    let targets = if version(e) < 4 { 5 } else { 6 };
    for index in 0..targets {
        buffer_load_form_id_into(e, buffer, this.addr() + HEAD_TRACKING_TARGETS + 4 * index);
        buffer_load_field(e, buffer, this, HEAD_TRACKING_TARGET_FLAGS + index, 1);
    }
    // The exe's frame (`SUB ESP,0xa8` under the exception record, then the record link) has
    // one form word per list: `EBP-0x2c` (+0x8c), `EBP-0x38` (+0x80) and `EBP-0x44` (+0x74).
    e.with_stack(0xb8, |e, frame| {
        buffer_load_form_id_run(e, buffer, this.addr() + 0x38c, frame.addr() + 0x8c);
        buffer_load_form_id_run(e, buffer, this.addr() + 0x394, frame.addr() + 0x80);
        if version(e) >= 0x12 {
            let spoken_to = e.mem.u32(this.addr() + 0x264);
            buffer_load_form_id_run(e, buffer, spoken_to, frame.addr() + 0x74);
        }
    });
    if buffer_load_discard(e, buffer, 1) != 0 {
        let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
        let item = if block == 0 {
            0
        } else {
            e.call(0x0083_c4d0, &args![block]).u32()
        };
        e.mem.set_u32(this.addr() + 0x368, item);
        e.call(0x0083_cf40, &args![item, buffer]);
    }
    let avoid_count = e.call(BUFFER_LOAD_SIZED_VALUE, &args![buffer]).u32();
    if avoid_count != 0 {
        let existing = e.mem.u32(this.addr() + 0x44c);
        if existing != 0 {
            e.call(LIST_CLEAR, &args![existing]);
        } else {
            let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
            let list = if block == 0 {
                0
            } else {
                e.call(LIST_CONSTRUCTOR, &args![block]).u32()
            };
            e.mem.set_u32(this.addr() + 0x44c, list);
        }
    }
    for _ in 0..avoid_count {
        let block = e.call(OPERATOR_NEW, &args![0x34u32]).u32();
        let area = if block == 0 {
            0
        } else {
            e.call(0x008f_db90, &args![block]).u32()
        };
        e.call(0x008d_7420, &args![area, buffer]);
        let list = e.mem.u32(this.addr() + 0x44c);
        list_append(e, list, area);
    }
    let detected = e.mem.u32(this.addr() + DETECTED_ACTOR_LIST);
    buffer_load_item_run(
        e,
        buffer,
        detected,
        0x24,
        DETECTION_STATE_CONSTRUCTOR,
        0x008d_7140,
    );
    let who_detects_me = e.mem.u32(this.addr() + WHO_DETECTS_ME_LIST);
    buffer_load_item_run(
        e,
        buffer,
        who_detects_me,
        0x24,
        DETECTION_STATE_CONSTRUCTOR,
        0x008d_7140,
    );
    if buffer_load_discard(e, buffer, 1) != 0 {
        let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
        let event = if block == 0 {
            0
        } else {
            e.call(0x008f_db10, &args![block]).u32()
        };
        e.mem.set_u32(this.addr() + 0x3dc, event);
        e.call(0x008d_72e0, &args![event, buffer]);
    }
    let queue = e.mem.u32(SAVE_LOAD_QUEUE);
    e.call(QUEUE_SUB_BUFFER, &args![queue, 1u32, buffer, 0u32]);
}

/// `Actor::CalculateCombatStrength(actor, -1.0)` (`008acbe0`; the float is
/// the global at `01012054`, the result comes back in ST0).
const CALCULATE_COMBAT_STRENGTH: u32 = 0x008a_cbe0;
/// `ProcessLists` singleton (`011e0e80`) and the array accessor
/// (`00717e50`), the array's `GetCount(level)` (`005be5c0`) and
/// `GetActor(index)` (`00968670`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
const PROCESS_LISTS_GET_ARRAY: u32 = 0x0071_7e50;
const PROCESS_ARRAY_COUNT: u32 = 0x005b_e5c0;
const PROCESS_ARRAY_GET_ACTOR: u32 = 0x0096_8670;
/// `Actor::IsInCombatantFaction` (`008ac6f0`) and
/// `TESActorBaseData::IsInEvilFactionsOnly` (`0047d740`, `this` = base form
/// + 0x30).
const IS_IN_COMBATANT_FACTION: u32 = 0x008a_c6f0;
const IS_IN_EVIL_FACTIONS_ONLY: u32 = 0x0047_d740;
/// Actor slots `EvaluateDetection` uses: `0x22c(flag)`, `0x2e8` (dying),
/// `0x3f8` (the actor's group), `0x448`, `0x304`, `0x400(trespass package)`
/// and `0x48(0x80000000)`.
const ACTOR_SLOT_0X22C: u32 = 0x22c;
const ACTOR_SLOT_GROUP: u32 = 0x3f8;
const ACTOR_SLOT_0X448: u32 = 0x448;
const ACTOR_SLOT_0X304: u32 = 0x304;
const ACTOR_SLOT_0X400: u32 = 0x400;
const ACTOR_SLOT_0X48: u32 = 0x48;

// Translated from 008f5480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::EvaluateDetection` (Xbox PDB): one actor's detection of the
/// player. Returns false at once when the actor's slot `0x22c(0)` is true or
/// the detection switch (`011f1221`) is off. `bCheckDeadTalk` (+0x2c6) is cleared, the global list at `011e0280` is cleared and the
/// player's detection level of the actor is found (`GetDetectionState`,
/// slot `0x504`, `iLevel` or -100). The combat strengths of the actor and
/// the player decide whether the actor is the stronger one (then the player is
/// queued on `AvoidActorList`, +0x394); otherwise `GetShouldAttackActor`
/// may set the alert level to 100, and an actor with a positive base
/// aggro radius (`TESNPC` +0x90) that the player is within is "near". When the
/// player is not the actor: the detection level is compared with the
/// `011cd45c` setting (0 when the running package fails `0067a770`) and
/// `bEvaluated` (+0x1f) of the detection state is set; the player is queued
/// on `AggroList` (+0x274) or `AggroRadiusList` (+0x38c) according to the alert level,
/// the combatant-faction rules and the number of actors in combat (`011cf414`,
/// `011cdf10`); a group the player belongs to (slot `0x3f8`) that is not yet
/// listed and wants to help (`CombatManager::GetWanttoHelpGroup`) is queued in
/// `GroupsToHelpList` (+0x27c). A positive detection level of an owner-hostile
/// cell can then create or remove a `TrespassPackage` on the player and,
/// when `ProcessLists` finds no package and the actor cannot be trespassed
/// against, the lists are freed and the actor's slot `0x400` decides the
/// result. If the player is in combat `ReactToCombatSituation` runs.
/// Finally each actor on the process list of level 0 is handed to `008ff350`
/// and the process's slot `0x6c(actor)` runs; the result is whether
/// `plastDetected` (+0x2a4) was set (it is cleared).
pub fn high_process_evaluate_detection(e: &mut Engine, this: Ptr, actor: Ptr) -> bool {
    let base = this.addr();
    let actor_addr = actor.addr();
    if e.vcall(actor_addr, ACTOR_SLOT_0X22C, &args![0u32]).bool() {
        return false;
    }
    if e.mem.u8(0x011f_1221) == 0 {
        return false;
    }
    e.mem.set_u8(base + 0x2c6, 0);
    // The running package (slot 0x27c) is fetched but never used.
    e.vcall(base, PROCESS_SLOT_RUNNING_PACKAGE, &args![]);
    let current_package = e.call(0x0088_1510, &args![actor]).u32();
    // The exe's frame (`SUB ESP,0x11c` under the exception record, then `ESI` and the record
    // link): the flag byte at `EBP-0x19` (+0x117), the attack result at `EBP-0x48` (+0xe8), the
    // difference at `EBP-0x74` (+0xbc), the combat flag at `EBP-0x75` (+0xbb) and the queued
    // words at `EBP-0xd8`, `-0x8c`, `-0xe4` and `-0x94` (+0x58, +0xa4, +0x4c, +0x9c).
    e.with_stack(0x130, |e, frame| {
        let local = |offset: u32| frame.addr() + offset;
        let flag_slot = local(0x117);
        e.mem.set_u8(flag_slot, 0);
        e.call(LIST_CLEAR, &args![0x011e_0280u32]);
        let player = e.mem.u32(PLAYER);
        let mut alert_level = 0;
        let mut check_time = setting_float(e, 0x011c_d7d8);
        let no_target = e.global::<f32>(NO_TARGET_FLOAT);
        let own_strength = e
            .call(CALCULATE_COMBAT_STRENGTH, &args![actor, no_target])
            .f32();
        let own_group = e.vcall(actor_addr, ACTOR_SLOT_GROUP, &args![]).u32();
        let base_form = e.call(0x0041_81e0, &args![actor]).u32();
        let mut sight_distance: i32 = 0;
        if base_form != 0 && e.call(0x0070_5cf0, &args![base_form + 0x90]).bool() {
            sight_distance = e.call(WORD_AT_0X14, &args![base_form + 0x90]).i32();
        }
        let mut near = false;
        let attack_result = local(0xe8);
        e.mem.set_u32(attack_result, 0);
        let mut stronger = false;
        // `ActorValueOwner` embedded at +0xa4: vtable slot 8 (value, one
        // argument) and slot 0xc (float value).
        let owner = actor_addr + 0xa4;
        let owner_vtable = e.mem.u32(owner);
        let aggression_slot = e.mem.u32(owner_vtable + 8);
        let aggression = e.call(aggression_slot, &args![owner, 0u32]).i32();
        let confidence_slot = e.mem.u32(owner_vtable + 0xc);
        let confidence = e.call(confidence_slot, &args![owner, 1u32]).f32();
        let mut player_strength = e
            .call(CALCULATE_COMBAT_STRENGTH, &args![player, no_target])
            .f32();
        if player_strength == 0.0 {
            player_strength = 1.0;
        }
        let attack_value = e.mem.i32(attack_result);
        let ratio = f64::from(own_strength) / f64::from(player_strength);
        let confident = f64::from(confidence).partial_cmp(&ratio) == Some(std::cmp::Ordering::Less);
        if aggression != 0 || attack_value != 1 || !confident {
            let should_attack = e
                .call(
                    0x008b_06d0,
                    &args![actor, player, 0u32, attack_result, 0u32],
                )
                .bool();
            if should_attack {
                alert_level = 100;
            } else {
                let attack_value = e.mem.i32(attack_result);
                if sight_distance > 0 && (attack_value == 0 || attack_value == 1) {
                    let player_position = e.call(0x0043_6aa0, &args![player]).u32();
                    let actor_position = e.call(0x0043_6aa0, &args![actor]).u32();
                    let squared = {
                        let out = local(0xbc);
                        e.call(0x0043_9ef0, &args![actor_position, out, player_position]);
                        e.call(0x004a_7290, &args![out]).f32()
                    };
                    let range_squared = f64::from(sight_distance.wrapping_mul(sight_distance));
                    if range_squared >= f64::from(squared) {
                        near = true;
                    }
                }
            }
        } else {
            stronger = true;
        }
        if player != actor_addr {
            let mut level: i32 = -100;
            let state = detection_state(e, this, player, 0);
            if state != 0 {
                level = e.mem.i32(state + 8);
            }
            if e.vcall(player, ACTOR_SLOT_0X22C, &args![0u32]).bool()
                && !e.vcall(player, ACTOR_SLOT_IS_DYING, &args![]).bool()
            {
                let player_process = e.call(ACTOR_PROCESS, &args![player]).u32();
                if !e
                    .vcall(player_process, PROCESS_SLOT_GREETING_FLAG, &args![])
                    .bool()
                    && e.mem.f32(player + 0xc8) <= 0.0
                {
                    e.mem.set_u8(base + 0x2c6, 1);
                }
            }
            let mut threshold = setting_float(e, 0x011c_d45c);
            if current_package != 0 && !e.call(0x0067_a770, &args![current_package]).bool() {
                threshold = 0.0;
            }
            let evaluated = state != 0 && e.mem.u8(state + 0x1f) != 0;
            if f64::from(threshold) < f64::from(level) && !evaluated {
                if state != 0 {
                    e.mem.set_u8(state + 0x1f, 1);
                }
                if !e.vcall(player, ACTOR_SLOT_0X22C, &args![0u32]).bool() {
                    if stronger {
                        let list = base + 0x394;
                        let player_ref = e.mem.u32(PLAYER);
                        list_append_at(e, list, player_ref, local(0x58));
                    } else if alert_level > 0 {
                        // The game skips this when both the player and a second
                        // reference (a local it never sets) are in combatant factions;
                        // that local is always null, so the skip cannot happen. The
                        // first faction test is still made.
                        e.call(IS_IN_COMBATANT_FACTION, &args![player]);
                        {
                            let mut engage = true;
                            let in_combat = e.call(0x0095_3c20, &args![e.mem.u32(PLAYER)]).i32();
                            let limit_slot = e.call(0x0043_d4d0, &args![0x011c_f414u32]).u32();
                            if in_combat >= e.mem.i32(limit_slot) {
                                let threshold = setting_float(e, 0x011c_df10);
                                if f64::from(threshold) >= f64::from(level) {
                                    engage = false;
                                }
                            }
                            if engage {
                                let target_slot =
                                    e.vcall(actor_addr, ACTOR_SLOT_0X304, &args![]).bool();
                                let mut send_alarm = u8::from(!target_slot);
                                let base_form = e.call(0x0041_81e0, &args![actor]).u32();
                                if e.call(IS_IN_EVIL_FACTIONS_ONLY, &args![base_form + 0x30])
                                    .bool()
                                {
                                    send_alarm = 1;
                                }
                                let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
                                let record = if block == 0 {
                                    0
                                } else {
                                    fn_008f6120(e, Ptr::new(block)).addr()
                                };
                                e.mem.set_u32(record, player);
                                e.mem.set_u8(record + 9, send_alarm);
                                list_append_at(e, base + 0x274, record, local(0xa4));
                                let current_player = e.mem.u32(PLAYER);
                                e.mem.set_u32(base + 0x2a4, current_player);
                            }
                        }
                    } else if near && level > 0 && !e.call(0x008a_78f0, &args![actor, 6u32]).bool()
                    {
                        let player_ref = e.mem.u32(PLAYER);
                        list_append_at(e, base + 0x38c, player_ref, local(0x4c));
                    }
                    let player_group = e.vcall(player, ACTOR_SLOT_GROUP, &args![]).u32();
                    let help_threshold = setting_float(e, 0x011c_df10);
                    if f64::from(help_threshold) < f64::from(level)
                        && player_group != 0
                        && player_group != own_group
                        && e.call(0x005a_4320, &args![player_group]).u32() != 0
                        && !fn_008f5410(e, this, player_group)
                    {
                        let manager = e.mem.u32(0x011f_1958);
                        if e.call(0x0099_2530, &args![manager, actor, player]).bool() {
                            let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
                            let record = if block == 0 {
                                0
                            } else {
                                fn_008f6120(e, Ptr::new(block)).addr()
                            };
                            e.mem.set_u32(record + 4, player_group);
                            list_append_at(e, base + 0x27c, record, local(0x9c));
                        }
                    }
                }
            }
            if level > 0 {
                let player_value_ok = if player == e.mem.u32(PLAYER) {
                    true
                } else {
                    let owner = player + 0xa4;
                    let owner_vtable = e.mem.u32(owner);
                    let value_slot = e.mem.u32(owner_vtable + 8);
                    e.call(value_slot, &args![owner, 0x2au32]).i32() < 100
                };
                if player_value_ok
                    && e.vcall(player, ACTOR_SLOT_0X448, &args![]).bool()
                    && !e.call(0x008c_0050, &args![actor]).bool()
                {
                    if let Some(result) = evaluate_trespass(e, this, actor_addr, player) {
                        return result;
                    }
                }
            }
            let in_combat_flag = {
                let slot = local(0xbb);
                e.mem.set_u8(slot, 0);
                e.call(0x0095_3c50, &args![e.mem.u32(PLAYER), slot]).bool()
            };
            if in_combat_flag {
                e.call(0x008f_fe50, &args![this, actor, player]);
            }
        }
        e.mem.set_u32(base + 0x2a4, 0);
        let parent = e.call(0x008d_6f30, &args![actor]).u32();
        let doubles = if parent == 0 {
            true
        } else {
            let parent = e.call(0x008d_6f30, &args![actor]).u32();
            !e.call(0x0042_5fd0, &args![parent]).bool()
        };
        if doubles {
            check_time += check_time;
        }
        let _ = check_time;
        let mut index = 0u32;
        loop {
            let array = e.call(PROCESS_LISTS_GET_ARRAY, &args![PROCESS_LISTS]).u32();
            let count = e.call(PROCESS_ARRAY_COUNT, &args![array, 0u32]).u32();
            if index >= count {
                break;
            }
            let array = e.call(PROCESS_LISTS_GET_ARRAY, &args![PROCESS_LISTS]).u32();
            let other = e.call(PROCESS_ARRAY_GET_ACTOR, &args![array, index]).u32();
            e.call(
                0x008f_f350,
                &args![
                    this,
                    other,
                    actor,
                    flag_slot,
                    0u32,
                    aggression,
                    confidence,
                    own_strength
                ],
            );
            index += 1;
        }
        e.vcall(base, 0x6c, &args![actor]);
        e.mem.set_u8(base + 0x270, 0);
        let detected = e.mem.u32(base + 0x2a4) != 0;
        e.mem.set_u32(base + 0x2a4, 0);
        detected
    })
}

/// The trespass part of `EvaluateDetection` (from `008f5ba3`). Returns
/// `Some(result)` when the function returns at once (the lists were freed
/// and the actor's slot `0x400` answered), `None` to go on with the
/// combat reaction.
fn evaluate_trespass(e: &mut Engine, this: Ptr, actor: u32, player: u32) -> Option<bool> {
    let base = this.addr();
    let cell = e.call(0x008d_6f30, &args![player]).u32();
    let owner = e.call(0x0054_6a40, &args![cell]).u32();
    let mut owner_is_evil = false;
    if owner != 0 {
        let kind = e.call(0x0040_1170, &args![owner]).i32();
        if kind == 0x2a {
            if e.call(IS_IN_EVIL_FACTIONS_ONLY, &args![owner + 0x30])
                .bool()
            {
                owner_is_evil = true;
            }
        } else {
            let kind = e.call(0x0040_1170, &args![owner]).i32();
            if kind == 8 && e.call(0x0047_d7c0, &args![owner]).bool() {
                owner_is_evil = true;
            }
        }
    }
    let actor_base = e.call(0x0041_81e0, &args![actor]).u32();
    if owner_is_evil
        && !e
            .call(IS_IN_EVIL_FACTIONS_ONLY, &args![actor_base + 0x30])
            .bool()
    {
        return None;
    }
    let player_cell = e.call(0x008d_6f30, &args![player]).u32();
    if !e.vcall(player, ACTOR_SLOT_0X448, &args![]).bool() {
        if !e.vcall(player, ACTOR_SLOT_0X448, &args![]).bool() {
            let extra = e.call(EXTRA_DATA_LIST, &args![player]).u32();
            e.call(0x0041_cda0, &args![extra]);
        }
        return None;
    }
    let extra = e.call(EXTRA_DATA_LIST, &args![player]).u32();
    let mut trespass = e.call(0x0041_cd70, &args![extra]).u32();
    let mut trespass_owner = 0;
    if trespass != 0 && e.call(0x008c_e390, &args![trespass]).u32() != 0 {
        trespass_owner = e.call(0x008c_e390, &args![trespass]).u32();
        if e.call(0x0040_1170, &args![trespass_owner]).i32() == 0x39
            && player_cell != trespass_owner
        {
            let extra = e.call(EXTRA_DATA_LIST, &args![player]).u32();
            e.call(0x0041_cda0, &args![extra]);
            trespass = 0;
        }
    } else if trespass == 0 {
        let mut kind = 3u32;
        if e.call(0x0054_4490, &args![player_cell]).bool() {
            kind = 0;
        }
        let block = e.call(OPERATOR_NEW, &args![0x9cu32]).u32();
        let package = if block == 0 {
            0
        } else {
            let owner = e.call(0x0054_6a40, &args![player_cell]).u32();
            e.call(0x009f_9250, &args![block, player, owner, player_cell, kind])
                .u32()
        };
        trespass = package;
        let extra = e.call(EXTRA_DATA_LIST, &args![player]).u32();
        e.call(0x0041_cc90, &args![extra, trespass]);
        e.vcall(player, ACTOR_SLOT_0X48, &args![0x8000_0000u32]);
    }
    let _ = trespass_owner;
    let found = e
        .call(0x0097_2570, &args![0x011e_0e80u32, player, 0u32, 2u32])
        .u32();
    if found != 0 {
        return None;
    }
    if !e.vcall(actor, ACTOR_SLOT_0X448, &args![]).bool() {
        for offset in [0x274, 0x27c] {
            let mut node = base + offset;
            while node != 0 {
                let item = list_item(e, node);
                if item == 0 {
                    break;
                }
                e.call(OPERATOR_DELETE, &args![item]);
                node = list_next(e, node);
            }
            e.call(LIST_CLEAR, &args![base + offset]);
        }
        e.call(LIST_CLEAR, &args![base + 0x394]);
        let answer = e.vcall(actor, ACTOR_SLOT_0X400, &args![trespass]).bool();
        return Some(answer);
    }
    if !e.vcall(player, ACTOR_SLOT_0X448, &args![]).bool() {
        let extra = e.call(EXTRA_DATA_LIST, &args![player]).u32();
        e.call(0x0041_cda0, &args![extra]);
    }
    None
}

/// Process slots `ProcessUseItemAt` uses beyond those named earlier:
/// `0x44` `SetupSpecialIdle`, `0x1d0` `SetItemBeingUsed`, `0x338`
/// `SetIdleTimer(float)`, `0x644` `ClearActionHeadTrackTarget`, `0x688`
/// `GetForceRotate`, `0x7c8` `ProcessActivate(actor, flag)` and `0x80c`
/// `CheckIfHasObject(actor, flag)`.
const PROCESS_SLOT_SETUP_SPECIAL_IDLE: u32 = 0x44;
const PROCESS_SLOT_SET_ITEM_BEING_USED: u32 = 0x1d0;
const PROCESS_SLOT_SET_IDLE_TIMER: u32 = 0x338;
const PROCESS_SLOT_CLEAR_ACTION_HEAD_TRACK_TARGET: u32 = 0x644;
const PROCESS_SLOT_GET_FORCE_ROTATE: u32 = 0x688;
const PROCESS_SLOT_PROCESS_ACTIVATE: u32 = 0x7c8;
const PROCESS_SLOT_CHECK_IF_HAS_OBJECT: u32 = 0x80c;
/// Actor slots: `0x1e4` `GetAnimation`, `0x2bc` `GetHeading(flag)`.
const ACTOR_SLOT_GET_ANIMATION: u32 = 0x1e4;
const ACTOR_SLOT_GET_HEADING: u32 = 0x2bc;
/// Callees: `Actor::GetAnimAction` (`008a7570`), `Animation::
/// SpecialIdleDonePlaying` (`004985f0`), `Actor::StopMoving` (`008b3ab0`),
/// `Actor::IsPathingComplete` (`008b3bb0`), `Actor::QueueEquipObject`
/// (`0088c650`, `(object, 1, 0, 1, 0, 1)`), `00915ef0(process, actor)`,
/// `MiddleHighProcess::ClearShootingAction` (`0092bf20`) and the heading
/// helpers: `004b15e0(heading, target, &out)` (cdecl, the angle difference),
/// `00408840(angle)` (cdecl, wraps an angle) and the turn limit setting at
/// `011cda1c` (degrees, scaled by the double at `01023128`).
const GET_ANIM_ACTION: u32 = 0x008a_7570;
const SPECIAL_IDLE_DONE_PLAYING: u32 = 0x0049_85f0;
const STOP_MOVING: u32 = 0x008b_3ab0;
const IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
const QUEUE_EQUIP_OBJECT: u32 = 0x0088_c650;
const CLEAR_SHOOTING_ACTION: u32 = 0x0092_bf20;
const ANGLE_DIFFERENCE: u32 = 0x004b_15e0;
const WRAP_ANGLE: u32 = 0x0040_8840;
const TURN_LIMIT_SETTING: u32 = 0x011c_da1c;
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `008bb5c0(actor, angle, flag)`: turns the actor towards an angle (stack:
/// angle, flag).
const TURN_ACTOR_SET_ROTATION: u32 = 0x008b_b5c0;
/// `TESObjectREFR::GetForm`-like conversion `007af430` and the global the
/// location checks compare against (`011ca248`).
const FORM_OF_REFERENCE: u32 = 0x007a_f430;
/// `PackageTarget` accessors: type (`00519b00`), object (`00680050`),
/// reference (`00680020`), object type (`00680080`); `TESPackage::
/// GetObjectTypeFromForm` (`00679ba0`, cdecl).
const TARGET_TYPE: u32 = 0x0051_9b00;
const TARGET_OBJECT: u32 = 0x0068_0050;
const TARGET_REFERENCE: u32 = 0x0068_0020;
const TARGET_OBJECT_TYPE: u32 = 0x0068_0080;
const OBJECT_TYPE_FROM_FORM: u32 = 0x0067_9ba0;
/// `PackageLocation::GetLocReference` (`0067f390`), `GetLocType` (`00678ca0`)
/// and the location of a package (`0055b980`).
const LOCATION_REFERENCE: u32 = 0x0067_f390;
const LOCATION_TYPE: u32 = 0x0067_8ca0;
const PACKAGE_LOCATION: u32 = 0x0055_b980;

/// The facing angle the actor is set to: the float at +8 of the
/// reference's rotation (`00430830` result), or of the point the actor's slot `0x16c`
/// fills in when there is no reference.
fn facing_of(e: &mut Engine, actor: u32, reference: u32) -> f32 {
    if reference != 0 {
        let point = e.call(0x0043_0830, &args![reference]).u32();
        e.mem.f32(point + 8)
    } else {
        e.with_stack(12, |e, out| {
            let point = e.vcall(actor, ACTOR_SLOT_GET_POINT, &args![out]).u32();
            e.mem.f32(point + 8)
        })
    }
}

/// Whether the actor must turn before acting: its heading (slot `0x2bc(0)`)
/// is compared with `target_angle` (`004b15e0`), the difference is wrapped
/// (`00408840`) and must not exceed the limit setting at `011cda1c` times
/// the degrees-to-radians double. True when the limit is below the wrapped
/// difference.
fn must_turn(e: &mut Engine, actor: u32, target_angle: f32) -> bool {
    let heading = e.vcall(actor, ACTOR_SLOT_GET_HEADING, &args![0u32]).f32();
    let difference = e.with_stack(4, |e, out| {
        e.mem.set_f32(out.addr(), 0.0);
        e.call(ANGLE_DIFFERENCE, &args![heading, target_angle, out])
            .f32()
    });
    let setting = e.call(0x0043_d4d0, &args![TURN_LIMIT_SETTING]).u32();
    let degrees = e.mem.i32(setting);
    let limit = (f64::from(degrees) * e.global::<f64>(DEGREES_TO_RADIANS)) as f32;
    let wrapped = e.call(WRAP_ANGLE, &args![difference]).f32();
    f64::from(limit) < f64::from(wrapped)
}

/// The "target location is a saved marker" test the use-item cases share: the
/// location reference's form equals the global `011ca248`, or the package's
/// location (`0055b980`) has type 3.
fn at_marker_location(e: &mut Engine, package: u32, location: u32) -> bool {
    if location != 0
        && e.call(FORM_OF_REFERENCE, &args![location]).u32() == e.mem.u32(SAVED_FORM_GLOBAL)
    {
        return true;
    }
    let package_location = e.call(PACKAGE_LOCATION, &args![package]).u32();
    if package_location == 0 {
        return false;
    }
    let package_location = e.call(PACKAGE_LOCATION, &args![package]).u32();
    e.call(LOCATION_TYPE, &args![package_location]).i32() == 3
}

/// Common tail of the "action finished" branches: `AddToProcedureIndex(actor,
/// 2)`, optionally `ClearShootingAction`, slot `0x394` and, when pathing is
/// not complete, slot `0x294`.
fn finish_use_step(e: &mut Engine, process: u32, actor: u32, clear_shooting: bool) {
    e.vcall(
        process,
        PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
        &args![actor, 2u32],
    );
    if clear_shooting {
        e.call(CLEAR_SHOOTING_ACTION, &args![process, actor]);
    }
    e.vcall(process, PROCESS_SLOT_FREE_UP_SPECIAL_IDLE, &args![actor]);
    if !e.call(IS_PATHING_COMPLETE, &args![actor]).bool() {
        e.vcall(process, PROCESS_SLOT_END_MOVE_MESSAGE, &args![actor]);
    }
}

// Translated from 008f9320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ProcessUseItemAt` (Xbox PDB): one step of a "use item at"
/// package for `actor`. Returns true when the step is done (also when the
/// actor is farther than the `011cd674` setting from the player, doing
/// nothing), false when it should be called again. Steps: take the package
/// (slot `0x27c`), its target and location reference (the location's
/// reference, or `pGenericLocation` +0x44 when no run-once package is set);
/// if the package test (slot `0x13c`) fails, end the procedure (index -1).
/// For a marker location the actor is turned (`must_turn`, then
/// `008bb5c0`) or snapped to it. The target object type (+0x108) is
/// resolved from the package target (or `pTarget` +0x40). Without a
/// target and without the object (`CheckIfHasObject`) the procedure
/// advances. For furniture packages (`00674d20`) the process looks for a free
/// bed or chair (`FindBedChairs`, the list at +0xd0, the current furniture at
/// +0x140) and activates it. Then, by object type: 1, 4 and 0x13 run
/// the "use at" idle steps, types 12, 13, 21, 22 and 23 are reported as
/// errors (a weapon target belongs to UseWeapon), 14 and 15 stop, and every
/// other type uses the item (`SetupSpecialIdle`, `SetIdleTimer` from the
/// random generator, the used-item bookkeeping, equipping the item
/// afterwards).
pub fn high_process_process_use_item_at(e: &mut Engine, this: Ptr, actor: Ptr) -> bool {
    let base = this.addr();
    let actor = actor.addr();
    let package = e.vcall(base, PROCESS_SLOT_RUNNING_PACKAGE, &args![]).u32();
    let player = e.mem.u32(PLAYER);
    let distance = e.call(0x0057_23b0, &args![actor, player, 0u32, 0u32]).f64();
    let range = setting_float(e, 0x011c_d674);
    if f64::from(range) < distance {
        return true;
    }
    let mut item_form = 0;
    let mut location = 0;
    let steps_needed: i32 = 1;
    let package_location = e.call(PACKAGE_LOCATION, &args![package]).u32();
    // `InventoryChanges::GetInventoryChanges(actor)` is called; its result is never used.
    e.call(0x004b_f220, &args![actor]);
    let target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    if package_location != 0 && e.call(LOCATION_REFERENCE, &args![package_location]).u32() != 0 {
        location = e.call(LOCATION_REFERENCE, &args![package_location]).u32();
    }
    if location == 0
        && e.vcall(base, PROCESS_SLOT_GET_RUN_ONCE_PACKAGE, &args![])
            .u32()
            == 0
    {
        location = e.mem.u32(base + 0x44);
    }
    let no_target = e.global::<f32>(NO_TARGET_FLOAT);
    if !e
        .vcall(
            package,
            PACKAGE_SLOT_TEST_0X13C,
            &args![actor, 0u32, no_target, 0u32],
        )
        .bool()
    {
        e.vcall(
            base,
            PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
            &args![actor, 0xffff_ffffu32],
        );
        return false;
    }
    if location != 0
        && e.call(FORM_OF_REFERENCE, &args![location]).u32() == e.mem.u32(SAVED_FORM_GLOBAL)
        && e.mem.u8(base + 0xe1) == 0
    {
        let angle = facing_of(e, actor, location);
        if must_turn(e, actor, angle) {
            e.call(TURN_ACTOR_SET_ROTATION, &args![actor, angle, 1u32]);
            e.vcall(
                base,
                PROCESS_SLOT_SET_ACTION_HEAD_TRACK_TARGET,
                &args![0u32],
            );
            return false;
        }
        e.call(STOP_MOVING, &args![actor]);
        e.vcall(
            base,
            PROCESS_SLOT_CLEAR_ACTION_HEAD_TRACK_TARGET,
            &args![1u32],
        );
        let reference_location = e.vcall(location, REFERENCE_SLOT_LOCATION, &args![]).u32();
        e.call(0x0057_5830, &args![actor, reference_location]);
        let height = facing_of(e, actor, location);
        e.vcall(actor, ACTOR_SLOT_SET_ANGLE_Z, &args![height]);
        e.vcall(base, PROCESS_SLOT_END_MOVE_MESSAGE, &args![actor]);
    }
    if target == 0 {
        e.mem.set_u32(base + 0x108, 0);
    } else {
        if package_location != 0 && e.call(LOCATION_REFERENCE, &args![package_location]).u32() != 0
        {
            location = e.call(LOCATION_REFERENCE, &args![package_location]).u32();
        }
        if location == 0
            && e.vcall(base, PROCESS_SLOT_GET_RUN_ONCE_PACKAGE, &args![])
                .u32()
                == 0
        {
            location = e.mem.u32(base + 0x44);
        }
        if e.mem.u32(base + TARGET) == 0 {
            e.vcall(base, PROCESS_SLOT_SET_TARGET, &args![location]);
        }
        let process_target = e.mem.u32(base + TARGET);
        let target_type_a = if process_target != 0 {
            e.call(TARGET_TYPE, &args![target]).i32()
        } else {
            0
        };
        if process_target != 0
            && (target_type_a == 0 || e.call(TARGET_TYPE, &args![target]).i32() == 3)
        {
            let form = e.call(FORM_OF_REFERENCE, &args![process_target]).u32();
            item_form = form;
            let form = e.call(FORM_OF_REFERENCE, &args![process_target]).u32();
            let kind = e.call(OBJECT_TYPE_FROM_FORM, &args![form]).u32();
            e.mem.set_u32(base + 0x108, kind);
        } else {
            let kind = e.call(TARGET_TYPE, &args![target]).i32();
            if kind == 1 {
                item_form = e.call(TARGET_OBJECT, &args![target]).u32();
                let object_type = e.call(OBJECT_TYPE_FROM_FORM, &args![item_form]).u32();
                e.mem.set_u32(base + 0x108, object_type);
            } else if e.call(TARGET_TYPE, &args![target]).i32() == 2 {
                let object_type = e.call(TARGET_OBJECT_TYPE, &args![target]).u32();
                e.mem.set_u32(base + 0x108, object_type);
            }
        }
        if e.mem.u32(base + TARGET) != 0 && e.mem.u32(base + 0x108) == 0 {
            let process_target = e.mem.u32(base + TARGET);
            item_form = e.call(FORM_OF_REFERENCE, &args![process_target]).u32();
            let form = e.call(FORM_OF_REFERENCE, &args![process_target]).u32();
            let kind = e.call(OBJECT_TYPE_FROM_FORM, &args![form]).u32();
            e.mem.set_u32(base + 0x108, kind);
            let generic = e.mem.u32(base + 0x44);
            if generic == 0 {
                e.mem.set_u32(base + 0x108, 0);
            } else {
                e.vcall(base, PROCESS_SLOT_SET_TARGET, &args![generic]);
                item_form = e.call(FORM_OF_REFERENCE, &args![generic]).u32();
                let form = e.call(FORM_OF_REFERENCE, &args![generic]).u32();
                let kind = e.call(OBJECT_TYPE_FROM_FORM, &args![form]).u32();
                e.mem.set_u32(base + 0x108, kind);
            }
        }
    }
    if e.mem.u32(base + TARGET) == 0
        && !e
            .vcall(base, PROCESS_SLOT_CHECK_IF_HAS_OBJECT, &args![actor, 1u32])
            .bool()
    {
        e.vcall(
            base,
            PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
            &args![actor, 1u32],
        );
        return false;
    }
    let saved_target = e.mem.u32(base + TARGET);
    if e.call(0x0067_4d20, &args![package]).bool()
        && e.vcall(actor, ACTOR_SLOT_0X214, &args![]).i32() != 4
        && !use_furniture(e, this, package, actor, saved_target)
    {
        return false;
    }
    let object_type = e.mem.u32(base + 0x108);
    match object_type {
        1 => {
            return use_item_case_1(e, this, package, actor, location, steps_needed);
        }
        4 | 0x13 => {
            if !e
                .vcall(base, PROCESS_SLOT_CHECK_IF_HAS_OBJECT, &args![actor, 1u32])
                .bool()
            {
                e.vcall(
                    base,
                    PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                    &args![actor, 2u32],
                );
                e.vcall(base, PROCESS_SLOT_FREE_UP_SPECIAL_IDLE, &args![actor]);
                e.vcall(base, PROCESS_SLOT_END_MOVE_MESSAGE, &args![actor]);
                return false;
            }
            let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
            let mut selected = 0;
            let kind = e.call(TARGET_TYPE, &args![package_target]).i32();
            if kind == 1 {
                selected = e.call(TARGET_OBJECT, &args![package_target]).u32();
            } else if e.call(TARGET_TYPE, &args![package_target]).i32() == 0 {
                let reference = e.call(TARGET_REFERENCE, &args![package_target]).u32();
                selected = e.call(FORM_OF_REFERENCE, &args![reference]).u32();
            } else if e.call(TARGET_TYPE, &args![package_target]).i32() == 3 {
                let reference = e.call(0x0056_9b80, &args![actor]).u32();
                selected = if reference == 0 {
                    0
                } else {
                    e.call(FORM_OF_REFERENCE, &args![reference]).u32()
                };
            }
            if selected != 0 {
                // The form is kept only when its type is 0x18; the value is not used.
                e.call(0x0040_1170, &args![selected]);
            }
            return use_item_case_1(e, this, package, actor, location, steps_needed);
        }
        12 | 13 | 21 | 22 | 23 => {
            let name = e.vcall(package, 0x130, &args![]).u32();
            // The exe's frame (`SUB ESP,0x1cc` under `EBP`): the message at `EBP-0x150` (+0x7c).
            e.with_stack(0x1cc, |e, frame| {
                let buffer = frame.addr() + 0x7c;
                e.call(0x0040_6d00, &args![buffer, 0x100u32, 0x0108_8318u32, name]);
                e.call(0x005b_5e40, &args![buffer]);
            });
            return false;
        }
        14 | 15 => return false,
        _ => {}
    }
    use_item_default(e, this, package, actor, location, item_form, steps_needed)
}

/// The furniture search of `ProcessUseItemAt` (from `008f9810`): looks for a
/// free bed or chair when the current furniture (+0x140) is taken or cannot
/// be sat on, activates it through slot `0x7c8`, and restores the target
/// (`saved_target`). Returns false when the step ends (the furniture is
/// still unresolved).
fn use_furniture(e: &mut Engine, this: Ptr, package: u32, actor: u32, saved_target: u32) -> bool {
    let base = this.addr();
    let mut search = true;
    let furniture = e.mem.u32(base + 0x140);
    if furniture != 0 {
        let form = e.call(FORM_OF_REFERENCE, &args![furniture]).u32();
        search = !(e.call(0x0050_93f0, &args![form]).bool()
            && e.call(0x0056_8260, &args![furniture, 0u32]).bool());
    }
    if search {
        e.call(0x0092_2670, &args![this, actor, 1u32, 0u32]);
        let chairs = base + 0xd0;
        if e.call(LIST_COUNT, &args![chairs]).u32() != 0
            && e.call(0x0044_b130, &args![0x011e_043cu32]).bool()
        {
            let first = list_item(e, chairs);
            e.mem.set_u32(base + 0x140, first);
            e.call(0x0092_c680, &args![this]);
            loop {
                let current = e.mem.u32(base + 0x140);
                if current == 0 || e.call(0x0056_8260, &args![current, 0u32]).bool() {
                    break;
                }
                e.call(LIST_REMOVE_ITEM, &args![chairs, base + 0x140]);
                let next = list_item(e, chairs);
                e.mem.set_u32(base + 0x140, next);
                e.call(0x0092_c680, &args![this]);
            }
            let current = e.mem.u32(base + 0x140);
            if current != 0 && e.call(0x0056_7790, &args![current]).u32() == 0 {
                let count = e.call(LIST_COUNT, &args![chairs]).u32();
                let random = e.call(0x0048_7f50, &args![]).u32();
                let mut index = random % count;
                if (index as i32) >= (count as i32) {
                    index = count;
                }
                let mut node = chairs;
                for _ in 0..(index as i32) {
                    node = list_next(e, node);
                }
                let picked = list_item(e, node);
                e.mem.set_u32(base + 0x140, picked);
                e.call(0x0092_c680, &args![this]);
            }
            e.call(0x0082_f1f0, &args![0x011e_043cu32]);
        }
        e.call(LIST_CLEAR, &args![chairs]);
    }
    let furniture = e.mem.u32(base + 0x140);
    if furniture != 0 {
        e.vcall(base, PROCESS_SLOT_SET_TARGET, &args![furniture]);
        e.vcall(base, PROCESS_SLOT_PROCESS_ACTIVATE, &args![actor, 0u32]);
        e.vcall(base, PROCESS_SLOT_SET_TARGET, &args![0u32]);
    }
    if e.call(0x0091_5ef0, &args![this, actor]).bool() {
        e.mem.set_u32(base + 0x140, 0);
        e.call(0x0092_c680, &args![this]);
        e.vcall(base, PROCESS_SLOT_END_MOVE_MESSAGE, &args![actor]);
        e.vcall(base, PROCESS_SLOT_SET_IDLE_TIMER, &args![0.0f32]);
    }
    if e.mem.u32(base + 0x140) != 0 {
        return false;
    }
    let _ = package;
    e.vcall(base, PROCESS_SLOT_SET_TARGET, &args![saved_target]);
    true
}

/// Object type 1 (and, after its target checks, types 4 and 0x13) of
/// `ProcessUseItemAt` (from `008f9d41`).
fn use_item_case_1(
    e: &mut Engine,
    this: Ptr,
    package: u32,
    actor: u32,
    location: u32,
    steps_needed: i32,
) -> bool {
    let base = this.addr();
    let done_steps = i32::from(e.mem.i16(base + 0x2c0));
    if steps_needed > 0
        && done_steps >= steps_needed
        && e.call(GET_ANIM_ACTION, &args![actor]).i32() == -1
    {
        let animation = e.vcall(actor, ACTOR_SLOT_GET_ANIMATION, &args![]).u32();
        if e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).bool() {
            finish_use_step(e, base, actor, false);
            return true;
        }
    }
    if at_marker_location(e, package, location)
        && !e.call(0x0091_5ef0, &args![this, actor]).bool()
        && e.vcall(actor, ACTOR_SLOT_0X214, &args![]).u32() == 0
        && !e
            .vcall(base, PROCESS_SLOT_GET_FORCE_ROTATE, &args![])
            .bool()
    {
        let height = facing_of(e, actor, location);
        e.call(TURN_ACTOR_SET_ROTATION, &args![actor, height, 0u32]);
    }
    let target_angle = facing_of(e, actor, location);
    if must_turn(e, actor, target_angle) {
        e.vcall(
            base,
            PROCESS_SLOT_SET_ACTION_HEAD_TRACK_TARGET,
            &args![0u32],
        );
        e.call(TURN_ACTOR_SET_ROTATION, &args![actor, target_angle, 0u32]);
        return false;
    }
    e.call(STOP_MOVING, &args![actor]);
    if e.vcall(base, PROCESS_SLOT_PROCESS_ACTIVATE, &args![actor, 1u32])
        .bool()
    {
        let count = e.mem.i16(base + 0x2c0).wrapping_add(1);
        e.mem.set_i16(base + 0x2c0, count);
    }
    if e.call(0x0067_0f90, &args![package]).bool() {
        let successors = e.call(0x0041_d8a0, &args![package]).u32();
        if list_next(e, successors) == 0 {
            let animation = e.vcall(actor, ACTOR_SLOT_GET_ANIMATION, &args![]).u32();
            if e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).bool() {
                e.vcall(
                    base,
                    PROCESS_SLOT_CLEAR_ACTION_HEAD_TRACK_TARGET,
                    &args![1u32],
                );
                e.vcall(
                    base,
                    PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                    &args![actor, 2u32],
                );
            }
        }
    }
    let done_steps = i32::from(e.mem.i16(base + 0x2c0));
    if steps_needed > 0
        && done_steps >= steps_needed
        && e.call(GET_ANIM_ACTION, &args![actor]).i32() == -1
    {
        finish_use_step(e, base, actor, true);
        return true;
    }
    let delta = e.call(0x0084_d030, &args![0x011f_6394u32]).f32();
    let timer = e.mem.f32(base + 0x2b8);
    e.mem
        .set_f32(base + 0x2b8, (f64::from(timer) - f64::from(delta)) as f32);
    if e.call(0x0067_0f90, &args![package]).bool() {
        let successors = e.call(0x0041_d8a0, &args![package]).u32();
        if list_next(e, successors) == 0 {
            let animation = e.vcall(actor, ACTOR_SLOT_GET_ANIMATION, &args![]).u32();
            if e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).bool() {
                e.vcall(
                    base,
                    PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                    &args![actor, 2u32],
                );
                e.call(CLEAR_SHOOTING_ACTION, &args![this, actor]);
            }
        }
    }
    false
}

/// Every other object type of `ProcessUseItemAt` (from `008fa1b3`).
fn use_item_default(
    e: &mut Engine,
    this: Ptr,
    package: u32,
    actor: u32,
    location: u32,
    item_form: u32,
    steps_needed: i32,
) -> bool {
    let base = this.addr();
    if item_form != 0 {
        e.vcall(base, PROCESS_SLOT_SET_ITEM_BEING_USED, &args![item_form]);
    }
    let used_item = e.mem.u32(base + 0x34);
    let done_steps = i32::from(e.mem.i16(base + 0x2c0));
    if used_item != 0
        && steps_needed > 0
        && done_steps >= steps_needed
        && e.call(GET_ANIM_ACTION, &args![actor]).i32() == -1
    {
        let animation = e.vcall(actor, ACTOR_SLOT_GET_ANIMATION, &args![]).u32();
        if e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).bool() {
            e.call(
                QUEUE_EQUIP_OBJECT,
                &args![actor, used_item, 1u32, 0u32, 1u32, 0u32, 1u32],
            );
            finish_use_step(e, base, actor, true);
            return true;
        }
    }
    if at_marker_location(e, package, location) {
        let target_angle = facing_of(e, actor, location);
        if must_turn(e, actor, target_angle) {
            e.call(TURN_ACTOR_SET_ROTATION, &args![actor, target_angle, 0u32]);
            e.vcall(
                base,
                PROCESS_SLOT_SET_ACTION_HEAD_TRACK_TARGET,
                &args![0u32],
            );
            return false;
        }
        e.call(STOP_MOVING, &args![actor]);
        e.vcall(
            base,
            PROCESS_SLOT_CLEAR_ACTION_HEAD_TRACK_TARGET,
            &args![1u32],
        );
    }
    let animation = e.vcall(actor, ACTOR_SLOT_GET_ANIMATION, &args![]).u32();
    let used_item = e.mem.u32(base + 0x34);
    let object_type = e.mem.u32(base + 0x108);
    if used_item == 0 && object_type != 0 {
        e.vcall(
            base,
            PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
            &args![actor, 0xffff_fffeu32],
        );
        return false;
    }
    if (used_item != 0 || object_type == 0)
        && animation != 0
        && e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).bool()
    {
        e.call(0x0060_0900, &args![used_item]);
        e.vcall(
            base,
            PROCESS_SLOT_SETUP_SPECIAL_IDLE,
            &args![actor, 0u32, 2u32, 1u32, 0u32, 1u32],
        );
        let random = e.call(0x0048_7f50, &args![]).u32();
        let remainder = random % 5000;
        let delay = (f64::from(remainder) * e.global::<f64>(0x0101_6978)
            + e.global::<f64>(0x0102_0998)) as f32;
        e.vcall(base, PROCESS_SLOT_SET_IDLE_TIMER, &args![delay]);
        e.call(0x0060_0900, &args![0u32]);
        let count = e.mem.i16(base + 0x2c0).wrapping_add(1);
        e.mem.set_i16(base + 0x2c0, count);
    }
    if e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).bool() {
        let used_item = e.mem.u32(base + 0x34);
        if used_item != 0 {
            e.call(
                QUEUE_EQUIP_OBJECT,
                &args![actor, used_item, 1u32, 0u32, 1u32, 0u32, 1u32],
            );
        }
        if e.call(0x0067_0f90, &args![package]).bool() {
            let successors = e.call(0x0041_d8a0, &args![package]).u32();
            if list_next(e, successors) == 0
                && e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).bool()
            {
                e.vcall(
                    base,
                    PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                    &args![actor, 2u32],
                );
                e.call(CLEAR_SHOOTING_ACTION, &args![this, actor]);
            }
        }
    }
    false
}

/// Process slots `ProcessUseWeapon` uses beyond those named earlier:
/// `0x274` `GetInstanceDataThatIsRunning`, `0x278`
/// `GetActorPackageThatIsRunning`, `0x14c` `GetCurrentAmmo`, `0x618`
/// `GetPostAnimationActions`, `0x638` `SetUseWeaponHeadTrackTarget`, `0x434`
/// `ClearAutomaticShotsToFire`, `0x3e4` `GetAnimAction`, `0x454`
/// `GetWeaponDrawn`, `0x44c` `GetWantWeaponDrawn`, `0x1b0`
/// `IsCurrentWeaponGrenadeOrMine`.
const PROCESS_SLOT_INSTANCE_DATA: u32 = 0x274;
const PROCESS_SLOT_ACTOR_PACKAGE_ITEM: u32 = 0x278;
const PROCESS_SLOT_GET_CURRENT_AMMO: u32 = 0x14c;
const PROCESS_SLOT_GET_POST_ANIMATION_ACTIONS: u32 = 0x618;
const PROCESS_SLOT_SET_USE_WEAPON_HEAD_TRACK_TARGET: u32 = 0x638;
const PROCESS_SLOT_CLEAR_AUTOMATIC_SHOTS_TO_FIRE: u32 = 0x434;
const PROCESS_SLOT_GET_ANIM_ACTION: u32 = 0x3e4;
const PROCESS_SLOT_GET_WEAPON_DRAWN: u32 = 0x454;
const PROCESS_SLOT_GET_WANT_WEAPON_DRAWN: u32 = 0x44c;
const PROCESS_SLOT_IS_WEAPON_GRENADE_OR_MINE: u32 = 0x1b0;
/// Reference slot `0xfc` (true for actors; `008d0360`) and package slot
/// `0x140(target, actor, 0, -1.0, 0)`.
const REFERENCE_SLOT_0XFC: u32 = 0xfc;
const PACKAGE_SLOT_TEST_0X140: u32 = 0x140;
/// The distance setting that ends the update when the actor is too far from
/// the player (`011cdd3c`) and the half-angle global used by the line of fire
/// check (`01088258`, a double).
const USE_WEAPON_RANGE_SETTING: u32 = 0x011c_dd3c;
const FIRING_CONE_ANGLE: u32 = 0x0108_8258;
/// Delay globals: `01018204` after a block and `01030ff0` after an attack.
const BLOCK_DELAY: u32 = 0x0101_8204;
const ATTACK_DELAY: u32 = 0x0103_0ff0;
/// The attack animation groups the actor picks from, with their weights
/// (group, weight), in the order the code lists them.
const ATTACK_CHOICES: [(u32, i32); 6] = [
    (0x5e, 10),
    (0x60, 10),
    (0x5c, 10),
    (0x5f, 10),
    (0x1a, 30),
    (0x20, 30),
];

/// Current weapon of the process: slot `0x148` gives an inventory entry
/// (`ItemChange`, none: `None`); its object (`0044ddc0`) counts only when its
/// type code (`00401170`) is `0x28` (a weapon). Returns the weapon form or 0.
fn current_weapon_form(e: &mut Engine, process: u32) -> Option<u32> {
    if e.vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .u32()
        == 0
    {
        return None;
    }
    let entry = e
        .vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .u32();
    let entry = e.call(0x0044_ddc0, &args![entry]).u32();
    if entry == 0 {
        return Some(0);
    }
    let again = e
        .vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .u32();
    let object = e.call(0x0044_ddc0, &args![again]).u32();
    if e.call(0x0040_1170, &args![object]).i32() != 0x28 {
        return Some(0);
    }
    let third = e
        .vcall(process, PROCESS_SLOT_CURRENT_WEAPON, &args![])
        .u32();
    Some(e.call(0x0044_ddc0, &args![third]).u32())
}

/// `Actor::QueueEquipObject(actor, object, 1, 0, 1, 0, 1)`.
fn queue_equip(e: &mut Engine, actor: u32, object: u32) {
    e.call(
        QUEUE_EQUIP_OBJECT,
        &args![actor, object, 1u32, 0u32, 1u32, 0u32, 1u32],
    );
}

/// `Actor::QueueUnEquipObject(actor, object, 1, 0, 0, 0, 1)`.
fn queue_unequip(e: &mut Engine, actor: u32, object: u32) {
    e.call(
        0x0088_c790,
        &args![actor, object, 1u32, 0u32, 0u32, 0u32, 1u32],
    );
}

/// Whether the ammunition list of the process (slot `0x14c`) has at most one
/// further node and the weapon passes `00524b60`; the "can reload" test.
fn weapon_can_reload(e: &mut Engine, process: u32, weapon: u32) -> bool {
    let ammo = e
        .vcall(process, PROCESS_SLOT_GET_CURRENT_AMMO, &args![])
        .u32();
    ammo != 0
        && (list_next(e, ammo) as i32) <= 1
        && weapon != 0
        && e.call(0x0052_4b60, &args![weapon]).u8() != 0
}

/// The "weapon is being used" test shared by the reload checks:
/// `Actor` test `008a8870`, or a non-null animation that passes `008846c0`.
fn actor_is_reloading(e: &mut Engine, actor: u32, animation: u32) -> bool {
    e.call(0x008a_8870, &args![actor]).bool()
        || (animation != 0 && e.call(0x0088_46c0, &args![animation]).bool())
}

// Translated from 008f7730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ProcessUseWeapon` (Xbox PDB): one step of a "use weapon"
/// package for `actor`. Returns true when the step is finished or the actor
/// is farther from the player than the `011cdd3c` setting (then nothing
/// else happens), false when it should be called again. In order:
///
/// 1. The package's test (slot `0x13c`) failing cleans up
///    (`CleanUpAfterProcessUseWeapon`) and ends the procedure (index -1).
///    For a marker location the actor is turned and set to its facing, else
///    `004938e0` stops it.
/// 2. The attack target `X` is the instance data's next node or the location
///    reference, replaced by the use-weapon data's target (a reference, or
///    `00569b80(actor)` for type 3; other types are reported as errors);
///    `006ecd40` is then given it.
/// 3. The target object type (+0x108) is resolved as in `ProcessUseItemAt`;
///    only types 12, 13, 21, 22 and 23 are valid (an error is reported
///    otherwise). The actor equips the package's weapon when it holds a
///    different one (`QueueEquipObject`), or fetches a matching one from the
///    inventory.
/// 4. Shots to fire (+0x2c2) are initialised from the weapon's clip, bursts
///    are counted, the weapon is unequipped when the bursts are used up and
///    the hold-attack timer (+0x2b8) is randomised; the reload logic
///    (`QueueReload`) follows.
/// 5. If the package test still passes the actor aims (`AimAtTarget`),
///    checks that no closer detected actor stands in its line of fire and
///    attacks: a weapon of level 3 or more uses `FireAtObject`, others pick
///    an attack animation group by weight (with a block reaction when the
///    target is attacking) and `QueueAttack`. When the actor is not ready to
///    fire it clears its special idle, aims, and counts the timer down.
/// 6. At the end the head-track target (slot `0x638`) is set to `X`.
pub fn high_process_process_use_weapon(e: &mut Engine, this: Ptr, actor: Ptr) -> bool {
    let base = this.addr();
    let actor = actor.addr();
    let package = e.vcall(base, PROCESS_SLOT_RUNNING_PACKAGE, &args![]).u32();
    let player = e.mem.u32(PLAYER);
    let distance = e.call(0x0057_23b0, &args![actor, player, 0u32, 0u32]).f64();
    let range = setting_float(e, USE_WEAPON_RANGE_SETTING);
    if f64::from(range) < distance {
        return true;
    }
    // The roll is drawn (advancing the random generator) but never used.
    let _roll = e.call(0x0048_7f50, &args![]).u32() % 100;
    let mut weapon = 0;
    let mut item_form = 0;
    let mut location = 0;
    let package_location = e.call(PACKAGE_LOCATION, &args![package]).u32();
    let inventory = e.call(0x004b_f220, &args![actor]).u32();
    let mut target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    let data = e.call(0x0067_58f0, &args![package]).u32();
    e.vcall(base, PROCESS_SLOT_SET_TARGET_FOR_PACKAGE, &args![actor]);
    if package_location != 0 && e.call(LOCATION_REFERENCE, &args![package_location]).u32() != 0 {
        location = e.call(LOCATION_REFERENCE, &args![package_location]).u32();
    }
    if location == 0 {
        location = e.mem.u32(base + 0x44);
    }
    let no_target = e.global::<f32>(NO_TARGET_FLOAT);
    if !e
        .vcall(
            package,
            PACKAGE_SLOT_TEST_0X13C,
            &args![actor, 0u32, no_target, 0u32],
        )
        .bool()
    {
        high_process_clean_up_after_process_use_weapon(e, this, Ptr::new(actor));
        e.vcall(
            base,
            PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
            &args![actor, 0xffff_ffffu32],
        );
        return false;
    }
    if location != 0
        && e.call(FORM_OF_REFERENCE, &args![location]).u32() == e.mem.u32(SAVED_FORM_GLOBAL)
        && e.mem.u8(base + 0xe1) == 0
    {
        let angle = facing_of(e, actor, location);
        if must_turn(e, actor, angle) {
            e.call(TURN_ACTOR_SET_ROTATION, &args![actor, angle, 0u32]);
            e.vcall(
                base,
                PROCESS_SLOT_SET_ACTION_HEAD_TRACK_TARGET,
                &args![0u32],
            );
            return false;
        }
        e.call(STOP_MOVING, &args![actor]);
        e.vcall(
            base,
            PROCESS_SLOT_CLEAR_ACTION_HEAD_TRACK_TARGET,
            &args![1u32],
        );
        let angle = facing_of(e, actor, location);
        e.vcall(actor, ACTOR_SLOT_SET_ANGLE_Z, &args![angle]);
        e.vcall(base, PROCESS_SLOT_END_MOVE_MESSAGE, &args![actor]);
    } else if e.call(0x0049_38e0, &args![actor]).bool() {
        e.call(STOP_MOVING, &args![actor]);
        e.vcall(base, PROCESS_SLOT_END_MOVE_MESSAGE, &args![actor]);
    }
    // The attack target.
    let mut aim_target = 0;
    let instance_data = e.vcall(base, PROCESS_SLOT_INSTANCE_DATA, &args![]).u32();
    if instance_data != 0 {
        aim_target = list_next(e, instance_data);
    }
    if aim_target == 0 {
        aim_target = location;
        let data_target = list_next(e, data);
        if data_target != 0 {
            match e.call(TARGET_TYPE, &args![data_target]).u32() {
                0 => aim_target = e.call(TARGET_REFERENCE, &args![data_target]).u32(),
                3 => aim_target = e.call(0x0056_9b80, &args![actor]).u32(),
                1 | 2 => {
                    let name = e.vcall(package, 0x130, &args![]).u32();
                    // The exe's frame (`SUB ESP,0x370` under `EBP`, then `ESI`): the message at
                    // `EBP-0x148` (+0x22c).
                    e.with_stack(0x374, |e, frame| {
                        let buffer = frame.addr() + 0x22c;
                        e.call(0x0040_6d00, &args![buffer, 0x100u32, 0x0108_82a8u32, name]);
                        e.call(0x005b_5e40, &args![buffer]);
                    });
                }
                _ => {}
            }
        }
        if instance_data != 0 {
            e.call(0x006e_cd40, &args![instance_data, aim_target]);
        }
    }
    // The object type.
    if target == 0 {
        e.mem.set_u32(base + 0x108, 0);
    } else {
        let process_target = e.mem.u32(base + TARGET);
        if process_target != 0
            && (e.call(TARGET_TYPE, &args![target]).i32() == 0
                || e.call(TARGET_TYPE, &args![target]).i32() == 3)
        {
            item_form = e.call(FORM_OF_REFERENCE, &args![process_target]).u32();
            let form = e.call(FORM_OF_REFERENCE, &args![process_target]).u32();
            let kind = e.call(OBJECT_TYPE_FROM_FORM, &args![form]).u32();
            e.mem.set_u32(base + 0x108, kind);
        } else if e.call(TARGET_TYPE, &args![target]).i32() == 1 {
            item_form = e.call(TARGET_OBJECT, &args![target]).u32();
            let kind = e.call(OBJECT_TYPE_FROM_FORM, &args![item_form]).u32();
            e.mem.set_u32(base + 0x108, kind);
        } else if e.call(TARGET_TYPE, &args![target]).i32() == 2 {
            let kind = e.call(TARGET_OBJECT_TYPE, &args![target]).u32();
            e.mem.set_u32(base + 0x108, kind);
        }
        if e.mem.u32(base + TARGET) != 0 && e.mem.u32(base + 0x108) == 0 {
            let process_target = e.mem.u32(base + TARGET);
            item_form = e.call(FORM_OF_REFERENCE, &args![process_target]).u32();
            let form = e.call(FORM_OF_REFERENCE, &args![process_target]).u32();
            let kind = e.call(OBJECT_TYPE_FROM_FORM, &args![form]).u32();
            e.mem.set_u32(base + 0x108, kind);
            let generic = e.mem.u32(base + 0x44);
            if generic != 0 {
                e.vcall(base, PROCESS_SLOT_SET_TARGET, &args![generic]);
                item_form = e.call(FORM_OF_REFERENCE, &args![generic]).u32();
                let form = e.call(FORM_OF_REFERENCE, &args![generic]).u32();
                let kind = e.call(OBJECT_TYPE_FROM_FORM, &args![form]).u32();
                e.mem.set_u32(base + 0x108, kind);
            } else {
                e.mem.set_u32(base + 0x108, 0);
            }
        }
    }
    if let Some(form) = current_weapon_form(e, base) {
        weapon = form;
    }
    let object_type = e.mem.u32(base + 0x108);
    if !matches!(object_type, 12 | 13 | 21 | 22 | 23) {
        let name = e.vcall(package, 0x130, &args![]).u32();
        // The same frame: the message at `EBP-0x248` (+0x12c).
        e.with_stack(0x374, |e, frame| {
            let buffer = frame.addr() + 0x12c;
            e.call(0x0040_6d00, &args![buffer, 0x100u32, 0x0108_8260u32, name]);
            e.call(0x005b_5e40, &args![buffer]);
        });
        return false;
    }
    target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    if !e
        .vcall(base, PROCESS_SLOT_CHECK_IF_HAS_OBJECT, &args![actor, 1u32])
        .bool()
        && e.mem.u32(base + 0x108) != 0x15
    {
        high_process_clean_up_after_process_use_weapon(e, this, Ptr::new(actor));
        e.vcall(
            base,
            PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
            &args![actor, 2u32],
        );
        return false;
    }
    if weapon != 0 {
        match e.call(TARGET_TYPE, &args![target]).u32() {
            1 => {
                let wanted = e.call(TARGET_OBJECT, &args![target]).u32();
                if weapon != wanted {
                    let wanted = e.call(TARGET_OBJECT, &args![target]).u32();
                    queue_equip(e, actor, wanted);
                    return true;
                }
            }
            0 | 3 => {
                let entry = e
                    .vcall(base, PROCESS_SLOT_ACTOR_PACKAGE_ITEM, &args![])
                    .u32();
                let entry = e.call(0x0044_ddc0, &args![entry]).u32();
                if entry != 0 {
                    let form = e.call(FORM_OF_REFERENCE, &args![entry]).u32();
                    if weapon != form {
                        let form = e.call(FORM_OF_REFERENCE, &args![entry]).u32();
                        queue_equip(e, actor, form);
                        return true;
                    }
                }
            }
            2 => {
                let wanted_type = e.call(TARGET_OBJECT_TYPE, &args![target]).u32();
                if !e.call(0x0067_9e00, &args![weapon, wanted_type]).bool() {
                    let found = e.with_stack(4, |e, out| {
                        e.mem.set_u32(out.addr(), 0);
                        let object_type = e.mem.u32(base + 0x108);
                        e.call(0x004c_6a10, &args![inventory, object_type, out])
                            .u32()
                    });
                    if found != 0 {
                        queue_equip(e, actor, found);
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    // Shots to fire.
    if e.mem.i16(base + 0x2c2) <= 0 && weapon != 0 {
        let clip = e.call(0x004f_e160, &args![weapon, 0u32]).u32();
        let shots = e.call(0x0064_7b70, &args![1u32, clip]).u16();
        e.mem.set_u16(base + 0x2c2, shots);
        if data != 0 && e.call(0x0083_3c20, &args![data]).bool() {
            let maximum = e.call(0x008f_9160, &args![data]).u16();
            let minimum = e.call(0x008f_9140, &args![data]).u16();
            let chosen = e
                .call(0x0094_4460, &args![u32::from(minimum), u32::from(maximum)])
                .i32();
            let value = if chosen > 0 {
                chosen
            } else {
                i32::from(e.mem.i16(base + 0x2c2))
            };
            e.mem.set_u16(base + 0x2c2, value as u16);
        }
    }
    let shots_to_fire = i32::from(e.mem.i16(base + 0x2c2));
    if shots_to_fire > 0 && i32::from(e.mem.i16(base + 0x2c0)) >= shots_to_fire {
        let bursts = e.mem.i16(base + 0x2c4).wrapping_add(1);
        e.mem.set_i16(base + 0x2c4, bursts);
        e.mem.set_u16(base + 0x2c2, 0xffff);
        e.mem.set_u16(base + 0x2c0, 0);
        let mut bursts_used = false;
        if weapon != 0
            && e.call(0x004c_0bf0, &args![weapon]).bool()
            && !e.call(0x0082_4060, &args![data]).bool()
        {
            let needed = e.call(0x008f_21d0, &args![data]).u16();
            bursts_used = i32::from(e.mem.i16(base + 0x2c4)) >= i32::from(needed);
        }
        if bursts_used {
            e.call(0x008a_6840, &args![actor, 0u32]);
            queue_unequip(e, actor, weapon);
            e.mem.set_f32(base + 0x2b8, 0.0);
        } else if e.call(0x0083_3c20, &args![data]).bool() {
            let first = e.call(0x0049_5460, &args![data]).f32();
            let second = e.call(0x004a_7bd0, &args![data]).f32();
            let delay = e.call(0x0047_6b70, &args![second, first]).f32();
            e.mem.set_f32(base + 0x2b8, delay);
        }
    }
    // Reload and fire-mode checks.
    if data != 0
        && e.call(0x008f_9100, &args![data]).u8() != 0
        && !e.call(0x0067_a4f0, &args![package]).bool()
    {
        let animation = e.vcall(actor, ACTOR_SLOT_GET_ANIMATION, &args![]).u32();
        let can_reload = weapon_can_reload(e, base, weapon);
        let reloading = actor_is_reloading(e, actor, animation);
        let kind_word = e.call(0x0043_01b0, &args![animation, 4u32]).u16();
        let kind = e.call(0x005f_2440, &args![u32::from(kind_word)]).i32();
        let in_attack_range = (0x18..=0xa8).contains(&kind);
        let wants_aim = !in_attack_range && (can_reload || reloading);
        let move_word = e.call(0x0043_01b0, &args![animation, 0u32]).u16();
        let moving = e.call(0x005f_23c0, &args![u32::from(move_word)]).i32() == 1;
        e.call(0x005c_e9d0, &args![actor, u32::from(wants_aim)]);
        if can_reload {
            if in_attack_range {
                e.call(0x0049_94f0, &args![animation, 4u32, 0u32]);
            } else if moving && !reloading {
                let sequence = e.call(0x0049_1040, &args![animation, 0u32]).u32();
                if sequence != 0 && e.call(0x0080_41a0, &args![sequence]).i32() == 1 {
                    e.call(0x008a_8840, &args![actor]);
                }
            }
        }
        if wants_aim || moving || (can_reload && in_attack_range) {
            if aim_target != 0 {
                high_process_aim_at_target(e, this, Ptr::new(actor), Ptr::new(aim_target), 0);
            }
            if aim_target != 0 {
                e.vcall(
                    base,
                    PROCESS_SLOT_SET_USE_WEAPON_HEAD_TRACK_TARGET,
                    &args![aim_target],
                );
            }
            return false;
        }
    } else if weapon != 0 && !e.call(0x004c_0bf0, &args![weapon]).bool() {
        let animation = e.vcall(actor, ACTOR_SLOT_GET_ANIMATION, &args![]).u32();
        let can_reload = weapon_can_reload(e, base, weapon);
        let flags = e
            .vcall(base, PROCESS_SLOT_GET_POST_ANIMATION_ACTIONS, &args![])
            .u32();
        let reloading = actor_is_reloading(e, actor, animation) || (flags & 2) != 0;
        if can_reload && !reloading {
            e.call(0x008a_8840, &args![actor]);
            return false;
        }
    }
    use_weapon_attack(
        e, this, actor, package, inventory, data, location, item_form, aim_target, weapon,
    )
}

/// The second half of `ProcessUseWeapon` (from `008f853c`): the package test
/// again, then unequip / equip handling for object type 0x15, firing
/// readiness (the timer at +0x2b8 and the line-of-fire scan over the
/// detected actors), and the attack itself. `weapon` is the weapon form found
/// earlier (it is replaced when the process reports a current weapon),
/// `location` the package's location reference and `aim_target` the attack
/// target `X`. The result is the function's result.
#[allow(clippy::too_many_arguments)]
fn use_weapon_attack(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    package: u32,
    inventory: u32,
    data: u32,
    location: u32,
    item_form: u32,
    aim_target: u32,
    mut weapon: u32,
) -> bool {
    let base = this.addr();
    let no_target = e.global::<f32>(NO_TARGET_FLOAT);
    if !e
        .vcall(
            package,
            PACKAGE_SLOT_TEST_0X13C,
            &args![actor, 0u32, no_target, 0u32],
        )
        .bool()
    {
        e.call(CLEAR_SHOOTING_ACTION, &args![this, actor]);
        e.vcall(
            base,
            PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
            &args![actor, 0xffff_ffffu32],
        );
        return false;
    }
    if let Some(form) = current_weapon_form(e, base) {
        weapon = form;
    }
    let object_type = e.mem.u32(base + 0x108);
    if object_type == 0x15 {
        if weapon != 0 {
            queue_unequip(e, actor, weapon);
            return true;
        }
    } else if weapon == 0 {
        // Equip the package's weapon, or one of the type it asks for.
        let mut found = 0;
        let mut entry = 0;
        let process_target = e.mem.u32(base + TARGET);
        if process_target != 0 {
            let form = e.call(FORM_OF_REFERENCE, &args![process_target]).u32();
            entry = e.call(0x004d_0650, &args![inventory, form, 0u32]).u32();
        } else if item_form != 0 {
            entry = e
                .call(0x004d_0650, &args![inventory, item_form, 0u32])
                .u32();
        } else {
            found = e.with_stack(4, |e, out| {
                e.mem.set_u32(out.addr(), 0);
                e.call(0x004c_6a10, &args![inventory, object_type, out])
                    .u32()
            });
        }
        if found == 0 && entry != 0 {
            found = e.call(0x0044_ddc0, &args![entry]).u32();
        }
        if found != 0 {
            queue_equip(e, actor, found);
        } else {
            high_process_clean_up_after_process_use_weapon(e, this, Ptr::new(actor));
            e.vcall(
                base,
                PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                &args![actor, 2u32],
            );
        }
        if entry != 0 {
            let object = e.call(0x0044_ddc0, &args![entry]).u32();
            queue_equip(e, actor, object);
        }
        if entry != 0 {
            e.call(0x0044_59e0, &args![entry, 1u32]);
        }
        return true;
    }
    // Firing readiness.
    let mut ready = e.mem.f32(base + 0x2b8) <= 0.0;
    if ready && !e.call(0x0082_4060, &args![data]).bool() {
        let needed = e.call(0x008f_21d0, &args![data]).u16();
        if i32::from(e.mem.i16(base + 0x2c4)) >= i32::from(needed) {
            high_process_clean_up_after_process_use_weapon(e, this, Ptr::new(actor));
            e.vcall(
                base,
                PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                &args![actor, 2u32],
            );
            return true;
        }
    }
    if ready && aim_target != 0 && location != aim_target {
        ready = high_process_aim_at_target(e, this, Ptr::new(actor), Ptr::new(aim_target), 0);
    }
    if ready && e.call(0x0067_2dd0, &args![package]).u32() != 0 && aim_target != 0 {
        ready = if e.vcall(aim_target, REFERENCE_SLOT_0XFC, &args![]).bool() {
            e.vcall(
                package,
                PACKAGE_SLOT_TEST_0X140,
                &args![aim_target, actor, 0u32, no_target, 0u32],
            )
            .bool()
        } else {
            false
        };
    }
    if ready && data != 0 && e.call(0x008f_9120, &args![data]).u8() != 0 {
        ready = line_of_fire_clear(e, this, actor, aim_target);
    }
    if !e
        .vcall(base, PROCESS_SLOT_GET_WEAPON_DRAWN, &args![])
        .bool()
        && !e
            .vcall(base, PROCESS_SLOT_GET_WANT_WEAPON_DRAWN, &args![])
            .bool()
    {
        e.call(0x008a_6840, &args![actor, 1u32]);
        e.mem.set_u8(base + 0x349, 1);
    } else if ready {
        if attack_with_weapon(e, this, actor, aim_target) {
            return false;
        }
    } else {
        e.vcall(base, PROCESS_SLOT_FREE_UP_SPECIAL_IDLE, &args![actor]);
        e.vcall(
            base,
            PROCESS_SLOT_CLEAR_AUTOMATIC_SHOTS_TO_FIRE,
            &args![0u32],
        );
        if aim_target != 0 {
            high_process_aim_at_target(e, this, Ptr::new(actor), Ptr::new(aim_target), 0);
        }
        if !e
            .vcall(
                package,
                PACKAGE_SLOT_TEST_0X13C,
                &args![actor, 0u32, no_target, 0u32],
            )
            .bool()
            && e.call(GET_ANIM_ACTION, &args![actor]).i32() == -1
        {
            let animation = e.vcall(actor, ACTOR_SLOT_GET_ANIMATION, &args![]).u32();
            if e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).bool() {
                e.vcall(
                    base,
                    PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
                    &args![actor, 0xffff_ffffu32],
                );
            }
        }
        if e.mem.f32(base + 0x2bc) <= 0.0 {
            let delta = e.call(0x0084_d030, &args![0x011f_6394u32]).f32();
            let timer = e.mem.f32(base + 0x2b8);
            e.mem
                .set_f32(base + 0x2b8, (f64::from(timer) - f64::from(delta)) as f32);
        }
    }
    if aim_target != 0 {
        e.vcall(
            base,
            PROCESS_SLOT_SET_USE_WEAPON_HEAD_TRACK_TARGET,
            &args![aim_target],
        );
    }
    false
}

/// The line-of-fire scan (from `008f8903`): `false` when another actor in the
/// process's detected list (+0x25c) other than the target, alive, in line of
/// sight and with a positive level, is closer than the target and within the
/// angle at `01088258` of the actor's heading; `true` otherwise.
fn line_of_fire_clear(e: &mut Engine, this: Ptr, actor: u32, target: u32) -> bool {
    let base = this.addr();
    let distance = e.call(0x0057_23b0, &args![actor, target, 0u32, 0u32]).f32();
    let distance_squared = (f64::from(distance) * f64::from(distance)) as f32;
    let mut node = e.mem.u32(base + DETECTED_ACTOR_LIST);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let state = list_item(e, node);
        node = list_next(e, node);
        let other = e.mem.u32(state);
        if other == target {
            continue;
        }
        if e.vcall(other, ACTOR_SLOT_0X22C, &args![0u32]).bool() {
            continue;
        }
        if e.mem.u8(state + 0x1e) == 0 || e.mem.i32(state + 8) <= 0 {
            continue;
        }
        let blocked = e.with_stack(12, |e, offset| {
            for word in 0..3 {
                let value = e.mem.u32(state + 0xc + 4 * word);
                e.mem.set_u32(offset.addr() + 4 * word, value);
            }
            let location = e.vcall(actor, REFERENCE_SLOT_LOCATION, &args![]).u32();
            e.call(0x0045_78c0, &args![offset, location]);
            let length_squared = e.call(0x004a_7290, &args![offset]).f32();
            if !(f64::from(distance_squared) > f64::from(length_squared)
                || f64::from(distance_squared).is_nan()
                || f64::from(length_squared).is_nan())
            {
                return false;
            }
            let angle = e.call(0x004b_13c0, &args![offset]).f32();
            let heading = e.vcall(actor, ACTOR_SLOT_GET_HEADING, &args![0u32]).f32();
            let difference = e.with_stack(4, |e, out| {
                e.mem.set_f32(out.addr(), 0.0);
                e.call(ANGLE_DIFFERENCE, &args![heading, angle, out]).f32()
            });
            let wrapped = e.call(WRAP_ANGLE, &args![difference]).f32();
            f64::from(wrapped) < e.global::<f64>(FIRING_CONE_ANGLE)
        });
        if blocked {
            return false;
        }
    }
    true
}

/// The attack itself (from `008f8ad7`), for an actor that is ready to fire.
/// Returns true when `ProcessUseWeapon` returns at once (after a block
/// reaction), skipping its final head-track call.
fn attack_with_weapon(e: &mut Engine, this: Ptr, actor: u32, aim_target: u32) -> bool {
    let base = this.addr();
    let roll = e.call(0x0048_7f50, &args![]).u32() % 100;
    let mut chosen: i32 = 0xff;
    let weapon = current_weapon_form(e, base).unwrap_or(0);
    if weapon != 0 && e.call(0x0044_6390, &args![weapon]).i32() >= 3 {
        e.call(0x0090_37e0, &args![this, actor, weapon]);
        return false;
    }
    if !e.call(IS_PATHING_COMPLETE, &args![actor]).bool() {
        return false;
    }
    if e.vcall(base, PROCESS_SLOT_GET_ANIM_ACTION, &args![]).i32() != -1 {
        if e.call(0x0089_4d60, &args![actor]).bool() {
            e.call(0x0089_4cc0, &args![actor, 0u32]);
        }
        return false;
    }
    if aim_target != 0
        && e.vcall(aim_target, REFERENCE_SLOT_IS_ACTOR, &args![])
            .bool()
        && roll >= 0x28
    {
        let action = e.call(GET_ANIM_ACTION, &args![aim_target]).i32();
        if action == 2 || e.call(GET_ANIM_ACTION, &args![aim_target]).i32() == 4 {
            e.call(0x0089_4cc0, &args![actor, 1u32]);
            let delay = e.global::<f32>(BLOCK_DELAY);
            e.mem.set_f32(base + 0x2b8, delay);
            return true;
        }
    }
    // Pick an attack animation group by weight.
    let mut usable = [false; 6];
    let mut total: i32 = 0;
    for (index, (group, weight)) in ATTACK_CHOICES.into_iter().enumerate() {
        let animation_group = e
            .call(0x0089_7910, &args![actor, group, 0u32, 0u32, 0u32])
            .u16();
        usable[index] = e
            .call(0x005f_2540, &args![u32::from(animation_group)])
            .bool();
        total += if usable[index] { weight } else { 0 };
    }
    if total > 0 {
        let mut remaining = (e.call(0x0048_7f50, &args![]).u32() % total as u32) as i32;
        let mut index = 0;
        while remaining >= 0 && index < ATTACK_CHOICES.len() {
            if usable[index] {
                remaining -= ATTACK_CHOICES[index].1;
                if remaining <= 0 {
                    chosen = ATTACK_CHOICES[index].0 as i32;
                }
            }
            index += 1;
        }
    }
    if chosen != -1 {
        // The frame of `008f7730` (0x374 bytes): the attack data at `EBP-0x30c` (+0x68).
        e.with_stack(0x374, |e, frame| {
            let pointer = frame.addr() + 0x68;
            e.call(LIST_ITEM_SLOT, &args![pointer]);
            let accepted = e
                .call(0x0097_fd80, &args![actor, chosen as u32, pointer])
                .bool();
            if accepted {
                let second = e.mem.f32(pointer + 4);
                if second != 0.0 {
                    chosen = if roll > 0x14 { 0x20 } else { 0x1a };
                }
            }
        });
    }
    if e.call(0x0049_97b0, &args![actor]).bool()
        && !e
            .vcall(base, PROCESS_SLOT_IS_WEAPON_GRENADE_OR_MINE, &args![])
            .bool()
        && !e.call(0x005a_2030, &args![actor]).bool()
    {
        chosen = 0x5c;
    }
    if !e.call(0x0089_4d60, &args![actor]).bool() {
        if chosen != -1 && e.call(0x0089_35f0, &args![actor, chosen as u32]).bool() {
            let delay = e.global::<f32>(ATTACK_DELAY);
            e.mem.set_f32(base + 0x2b8, delay);
        }
    } else {
        let delay = e.global::<f32>(ATTACK_DELAY);
        e.mem.set_f32(base + 0x2b8, delay);
    }
    false
}

// ---- Second half of the unit: save/load hooks, fading, combat reactions, actor values ------

/// `TESObjectREFR::SetTargeted(this, flag)` (`00564db0`).
const SET_TARGETED_REFERENCE: u32 = 0x0056_4db0;
/// Replaces the item of a `BSSimpleList` node by the word the second
/// argument points to (`00726c60`, `this` is the node).
const LIST_SET_ITEM: u32 = 0x0072_6c60;
/// The constructor the `BSSimpleList<AvoidAreaStruct *>` entries share
/// (`00692710`, `this` is the entry) and the `float` constant it starts the
/// entry's distance with (`FLT_MAX`, `01016970`).
const AVOID_AREA_BASE_CONSTRUCTOR: u32 = 0x0069_2710;
const AVOID_AREA_START_DISTANCE: u32 = 0x0101_6970;
/// `MiddleHighProcess::InitLoadGame_ov2` (`009278d0`) and `::Revert_ov2`
/// (`009280f0`), each with the save buffer as its one stack word.
const MIDDLE_HIGH_INIT_LOAD_GAME_OV2: u32 = 0x0092_78d0;
const MIDDLE_HIGH_REVERT_OV2: u32 = 0x0092_80f0;
/// `DialogueItem::InitLoadGame` (`0083d0d0`, `this` is the topic, the buffer
/// is the stack word).
const DIALOGUE_ITEM_INIT_LOAD_GAME: u32 = 0x0083_d0d0;
/// Resolves the actor form ID of a `DetectionState` (`008d7220`, `this` is
/// the state); `008d74a0` does the same for the reference of an avoid-area
/// entry and `008d7340` for a `DetectionEvent`; the last two are called with
/// the buffer as a stack word they never read.
const DETECTION_STATE_RESOLVE: u32 = 0x008d_7220;
const AVOID_AREA_RESOLVE: u32 = 0x008d_74a0;
const DETECTION_EVENT_RESOLVE: u32 = 0x008d_7340;
/// `008d6f10(node, flag)` and `008e5730(process, actor)`: the pair
/// `InitLoadGame_ov2` calls for an actor whose process has a bone LOD
/// controller.
const NODE_UPDATE: u32 = 0x008d_6f10;
const PROCESS_REFRESH_BONE_LOD: u32 = 0x008e_5730;
/// Resets an `ActorValueCache` (`008c6eb0`, `this` is the cache).
const CACHE_RESET: u32 = 0x008c_6eb0;
/// `LipSynchAnim::~LipSynchAnim` (`004d5850`, one stack word: delete flag)
/// and the destructor of the greeting topic (`005c90d0`, one stack word:
/// delete flag).
const LIP_SYNCH_ANIM_DESTROY: u32 = 0x004d_5850;
const DIALOGUE_ITEM_DESTROY: u32 = 0x005c_90d0;
/// The `BSSimpleList` deleting destructor (`004702f0`, one stack word: delete
/// flag).
const LIST_DESTROY: u32 = 0x0047_02f0;
/// `HighProcess::ClearAvoidAreas` (`00904160`).
const CLEAR_AVOID_AREAS: u32 = 0x0090_4160;
/// Process slot `0x6c4`: `HighProcess::ClearMuzzleFlash`.
const PROCESS_SLOT_CLEAR_MUZZLE_FLASH: u32 = 0x6c4;
/// Slots of the save buffer object: `0` the save version (a byte) and `0xc`
/// the actor the process belongs to.
const BUFFER_SLOT_VERSION: u32 = 0x0;
const BUFFER_SLOT_ACTOR: u32 = 0xc;
/// The game setting `Revert_ov2` copies into `fClearTalkToListTimer`.
const CLEAR_TALK_TO_LIST_SETTING: u32 = 0x011c_dcd8;
/// `HighProcess::fHeadTrackTargetTimer`.
const HEAD_TRACK_TIMER: u32 = 0x418;

/// Replaces the form ID in the word at `slot` by the object it names (cast
/// to the class with the type descriptor `class`), or by null.
fn resolve_form_word(e: &mut Engine, slot: u32, class: u32) -> u32 {
    let form_id = e.mem.u32(slot);
    let object = if form_id == 0 {
        0
    } else {
        form_cast_to(e, form_id, class)
    };
    e.mem.set_u32(slot, object);
    object
}

/// Removes the node `node` of a list: the head node (`previous` is 0) by
/// `0063f7b0` (the next node's content moves into it, so `node` stays the
/// node to continue with), any other by item through its predecessor
/// `previous`, whose next node is the one to continue with.
fn remove_list_node(e: &mut Engine, node: u32, previous: u32) -> u32 {
    if previous == 0 {
        e.call(LIST_REMOVE_HEAD, &args![node]);
        node
    } else {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        e.call(LIST_REMOVE_ITEM, &args![previous, slot]);
        list_next(e, previous)
    }
}

/// One of the `BSSimpleList<Actor *>` loops of `InitLoadGame_ov2`: every
/// stored form ID is replaced by its `Actor`; a node whose item is null or
/// does not resolve is removed.
fn resolve_actor_list(e: &mut Engine, head: u32) {
    let mut node = head;
    let mut previous = 0;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let form_id = e.mem.u32(slot);
        let mut keep = false;
        if form_id != 0 {
            let actor = form_cast_to(e, form_id, TYPE_ACTOR_CLASS);
            if actor != 0 {
                e.with_stack(4, |e, cell| {
                    e.mem.set_u32(cell.addr(), actor);
                    e.call(LIST_SET_ITEM, &args![node, cell]);
                });
                keep = true;
            }
        }
        if keep {
            previous = node;
            node = list_next(e, node);
        } else {
            node = remove_list_node(e, node, previous);
        }
    }
}

/// One of the `DetectionState` list loops of `InitLoadGame_ov2`: every
/// state resolves its actor (`008d7220`); a null state, or one whose actor is
/// null, is removed (and a non-null state freed).
fn resolve_detection_list(e: &mut Engine, head: u32) {
    let mut node = head;
    let mut previous = 0;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let state = list_item(e, node);
        let mut remove = true;
        if state != 0 {
            e.call(DETECTION_STATE_RESOLVE, &args![state]);
            remove = e.mem.u32(state) == 0;
            if remove {
                e.call(OPERATOR_DELETE, &args![state]);
            }
        }
        if remove {
            node = remove_list_node(e, node, previous);
        } else {
            previous = node;
            node = list_next(e, node);
        }
    }
}

// Translated from 008fdb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed constructor; the engine map names the address
/// `BSSimpleList<AvoidAreaStruct_P>::AddHead` because identical-code folding
/// put that name on it. Its body constructs an avoid-area entry: the base
/// constructor `00692710`, then +0x24 = the `float` at `01016970`
/// (`FLT_MAX`), +0x28 = 0, +0x2c = 0.0, +0x30 = 0. Returns `this`.
pub fn fn_008fdb90(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(AVOID_AREA_BASE_CONSTRUCTOR, &args![this]);
    let start = e.global::<f32>(AVOID_AREA_START_DISTANCE);
    e.mem.set_f32(base + 0x24, start);
    e.mem.set_u32(base + 0x28, 0);
    e.mem.set_f32(base + 0x2c, 0.0);
    e.mem.set_u32(base + 0x30, 0);
    this
}

// Translated from 008fdbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::InitLoadGame_ov2` (Xbox PDB): after `buffer`'s actor (slot
/// `0xc`) is fetched, `MiddleHighProcess::InitLoadGame_ov2(buffer)` runs and
/// the form IDs the load stored in `pGreetActor` (+0x30c), `plastDetected`
/// (+0x2a4), `pTeleportFadeRef` (+0x3f0) and `pLastTarget` (+0x41c) become
/// `TESObjectREFR`s (null when unresolved), `pDialogTarget` (+0x370) the class
/// at `01184920`. From save version 0xe on `pPathLookAtTarget` (+0x2ac)
/// is resolved and marked targeted, and so is each of the six head-tracking
/// targets (+0x3f8). The actor lists at +0x38c and +0x394 (and from version
/// 0x12 the one behind +0x264) are resolved to `Actor`s with the
/// unresolved entries removed; the greeting topic (+0x368) and the
/// avoid-area entries (+0x44c) get their own load fix-ups; the detection
/// lists (+0x25c, +0x260) resolve their actors and drop entries without one;
/// the detection event (+0x3dc) is resolved. A process with a bone LOD
/// controller (+0x2e4) whose actor is neither null nor the player and has 3D
/// refreshes it, and the value cache (+0x428) is reset.
pub fn high_process_init_load_game_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    let actor = e.vcall(buffer, BUFFER_SLOT_ACTOR, &args![]).u32();
    e.call(MIDDLE_HIGH_INIT_LOAD_GAME_OV2, &args![this, buffer]);
    for offset in [0x30c, 0x2a4, 0x3f0, 0x41c] {
        resolve_form_word(e, base + offset, TYPE_REFERENCE);
    }
    resolve_form_word(e, base + 0x370, TYPE_DIALOG_TARGET);
    if e.vcall(buffer, BUFFER_SLOT_VERSION, &args![]).u8() > 0xd {
        let target = resolve_form_word(e, base + 0x2ac, TYPE_REFERENCE);
        if target != 0 {
            e.call(SET_TARGETED_REFERENCE, &args![target, 1u32]);
        }
    }
    for index in 0..6 {
        let slot = base + HEAD_TRACKING_TARGETS + 4 * index;
        let target = resolve_form_word(e, slot, TYPE_REFERENCE);
        if target != 0 {
            e.call(SET_TARGETED_REFERENCE, &args![target, 1u32]);
        }
    }
    // AggroRadiusList (+0x38c) and AvoidActorList (+0x394), inline lists.
    resolve_actor_list(e, base + 0x38c);
    resolve_actor_list(e, base + 0x394);
    if e.vcall(buffer, BUFFER_SLOT_VERSION, &args![]).u8() > 0x11 {
        let spoke_to = e.mem.u32(base + 0x264);
        resolve_actor_list(e, spoke_to);
    }
    let topic = e.mem.u32(base + 0x368);
    if topic != 0 {
        e.call(DIALOGUE_ITEM_INIT_LOAD_GAME, &args![topic, buffer]);
    }
    let mut node = e.mem.u32(base + 0x44c);
    while node != 0 {
        let entry = list_item(e, node);
        if entry != 0 {
            e.call(AVOID_AREA_RESOLVE, &args![entry, buffer]);
        }
        node = list_next(e, node);
    }
    let detected = e.mem.u32(base + DETECTED_ACTOR_LIST);
    resolve_detection_list(e, detected);
    let who_detects_me = e.mem.u32(base + WHO_DETECTS_ME_LIST);
    resolve_detection_list(e, who_detects_me);
    let event = e.mem.u32(base + 0x3dc);
    if event != 0 {
        e.call(DETECTION_EVENT_RESOLVE, &args![event, buffer]);
    }
    if e.mem.u32(base + 0x2e4) != 0 && actor != 0 && actor != e.mem.u32(PLAYER) {
        let has_3d = e.vcall(actor, REFERENCE_SLOT_GET_3D, &args![]).u32();
        if has_3d != 0 {
            let node_3d = e.vcall(actor, REFERENCE_SLOT_GET_3D, &args![]).u32();
            e.call(NODE_UPDATE, &args![node_3d, 0u32]);
            e.call(PROCESS_REFRESH_BONE_LOD, &args![this, actor]);
        }
    }
    let cache = e.mem.u32(base + 0x428);
    if cache != 0 {
        e.call(CACHE_RESET, &args![cache]);
    }
}

// Translated from 008fe420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::Revert_ov2` (Xbox PDB): `MiddleHighProcess::Revert_ov2(buffer)`
/// then the process returns to its start-of-game values: the value cache is
/// reset, the timers, counters, flags and shot counts are cleared, the lip
/// synch animation and the greeting topic are destroyed, the aggro, avoid and
/// last-spoke-to lists are emptied, the muzzle flash is cleared (slot
/// `0x6c4`), `ClearAvoidAreas` runs, the head-tracking targets are cleared,
/// the items of both detection lists are freed (and the lists emptied) and
/// the pending detection event (+0x3dc) is freed.
pub fn high_process_revert_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.call(MIDDLE_HIGH_REVERT_OV2, &args![this, buffer]);
    let cache = e.mem.u32(base + 0x428);
    if cache != 0 {
        e.call(CACHE_RESET, &args![cache]);
    }
    e.mem.set_u8(base + 0x364, 0);
    e.mem.set_u32(base + 0x3ac, 0xffff_ffff);
    e.mem.set_u32(base + 0x3b0, 0xffff_ffff);
    e.mem.set_f32(base + 0x2d0, 0.0);
    e.mem.set_u8(base + 0x2c6, 0);
    e.mem.set_u32(base + 0xb4, 0xffff_ffff);
    e.mem.set_f32(base + 0x310, 0.0);
    e.mem.set_u8(base + 0x32c, 0);
    e.mem.set_f32(base + 0x330, 0.0);
    e.mem.set_u8(base + 0x340, 1);
    e.mem.set_u8(base + 0x349, 0);
    e.mem.set_u8(base + 0x374, 0);
    e.mem.set_f32(base + 0x34c, 0.0);
    e.mem.set_f32(base + 0x338, 0.0);
    e.mem.set_f32(base + 0x2e0, 0.0);
    e.mem.set_f32(base + 0x2bc, 0.0);
    e.mem.set_f32(base + 0x2b8, 0.0);
    e.mem.set_u16(base + 0x2c0, 0);
    e.mem.set_u16(base + 0x2c2, 0xffff);
    e.mem.set_u16(base + 0x2c4, 0);
    let breath = e.global::<f32>(DEFAULT_BREATH_TIMER);
    e.mem.set_f32(base + 0x33c, breath);
    e.mem.set_f32(base + 0x2a8, 0.0);
    e.mem.set_f32(base + 0x2d8, 0.0);
    e.mem.set_f32(base + 0x344, 0.0);
    e.mem.set_u8(base + 0x2dc, 0);
    e.mem.set_u8(base + 0x375, 0);
    e.mem.set_f32(base + 0x378, 0.0);
    e.mem.set_f32(base + 0x37c, 0.0);
    e.mem.set_f32(base + 0x384, 1.0);
    e.mem.set_f32(base + 0x388, 0.0);
    e.mem.set_u32(base + 0x39c, 0);
    e.mem.set_u8(base + 0x3a0, 0);
    e.mem.set_f32(base + 0x3a4, 0.0);
    e.mem.set_u8(base + 0x3a8, 0);
    e.mem.set_u8(base + 0x3b8, 0);
    e.mem.set_u8(base + 0x3b9, 0);
    e.mem.set_u8(base + 0x3d0, 0);
    e.mem.set_u8(base + 0x348, 0);
    let lip_synch = e.mem.u32(base + 0x3cc);
    if lip_synch != 0 {
        e.call(LIP_SYNCH_ANIM_DESTROY, &args![lip_synch, 1u32]);
    }
    e.mem.set_u32(base + 0x3cc, 0);
    e.mem.set_u8(base + 0x3d1, 0);
    e.mem.set_u32(base + 0x2f0, 0);
    e.mem.set_u16(base + 0x2ec, 0xffff);
    let no_target = e.global::<f32>(NO_TARGET_FLOAT);
    e.mem.set_f32(base + 0x448, no_target);
    e.mem.set_u32(base + 0x350, 0);
    e.mem.set_u8(base + 0x29d, 0);
    e.mem.set_f32(base + 0x2b0, 1.0);
    e.mem.set_f32(base + 0x2c8, 0.0);
    e.mem.set_f32(base + HEAD_TRACK_TIMER, no_target);
    e.mem.set_f32(base + 0x43c, 0.0);
    e.mem.set_f32(base + 0x440, 0.0);
    e.mem.set_u8(base + 0x444, 0);
    e.mem.set_u8(base + 0x445, 0);
    e.mem.set_f32(base + 0x450, 0.0);
    e.mem.set_u8(base + 0x458, 0);
    e.mem.set_u32(base + 0x430, 2);
    e.mem.set_u8(base + 0x3e0, 0);
    e.mem.set_f32(base + 0x42c, 0.0);
    let clear_talk = setting_float(e, CLEAR_TALK_TO_LIST_SETTING);
    e.mem.set_f32(base + 0x2a0, clear_talk);
    e.mem.set_u32(base + 0x2ac, 0);
    let topic = e.mem.u32(base + 0x368);
    if topic != 0 {
        e.call(DIALOGUE_ITEM_DESTROY, &args![topic, 1u32]);
    }
    e.mem.set_u32(base + 0x368, 0);
    e.call(LIST_CLEAR, &args![base + 0x38c]);
    e.call(LIST_CLEAR, &args![base + 0x394]);
    let spoke_to = e.mem.u32(base + 0x264);
    e.call(LIST_CLEAR, &args![spoke_to]);
    e.vcall(base, PROCESS_SLOT_CLEAR_MUZZLE_FLASH, &args![]);
    e.mem.set_u32(base + 0x454, 0);
    e.call(CLEAR_AVOID_AREAS, &args![this]);
    for index in 0..6 {
        e.mem.set_u32(base + HEAD_TRACKING_TARGETS + 4 * index, 0);
        e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS + index, 0);
    }
    e.mem.set_u32(base + 0x41c, 0);
    e.mem.set_u8(base + 0x420, 0);
    e.mem.set_f32(base + 0x3bc, 0.0);
    e.mem.set_f32(base + 0x3c0, 0.0);
    for list_offset in [DETECTED_ACTOR_LIST, WHO_DETECTS_ME_LIST] {
        let list = e.mem.u32(base + list_offset);
        let mut node = list;
        while node != 0 {
            let item = list_item(e, node);
            if item != 0 {
                e.call(OPERATOR_DELETE, &args![item]);
            }
            node = list_next(e, node);
        }
        e.call(LIST_CLEAR, &args![list]);
    }
    let event = e.mem.u32(base + 0x3dc);
    e.call(OPERATOR_DELETE, &args![event]);
    e.mem.set_u32(base + 0x3dc, 0);
    e.mem.set_u32(base + 0x3e4, 0);
}

// ---- Fading --------------------------------------------------------------------------

/// The two `double` constants the fade code compares the alpha with: 0.0
/// (`01012060`) and 1.0 (`01012070`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `HighProcess::eFadeState` (+0x3e8), `fFadeAlpha` (+0x3ec),
/// `pTeleportFadeRef` (+0x3f0) and `pMoveToFadeStruct` (+0x3f4). The states
/// the code below distinguishes: 0 none, 1 and 3 fade in, 2 and 4 fade out
/// (4 then activates the reference), 5 fade out and disable, 6 fade out and
/// delete, 7 fade out and move.
const FADE_STATE: u32 = 0x3e8;
const FADE_ALPHA: u32 = 0x3ec;
const FADE_REFERENCE: u32 = 0x3f0;
const FADE_MOVE: u32 = 0x3f4;
/// `Actor::StopMoving` (`008b3ab0`), `Actor::PickAnimations(float, float)`
/// (`00895110`), `Actor::UpdateAlpha` (`008c4640`), all with the actor as
/// `this`.
const ACTOR_STOP_MOVING: u32 = 0x008b_3ab0;
const ACTOR_PICK_ANIMATIONS: u32 = 0x0089_5110;
const ACTOR_UPDATE_ALPHA: u32 = 0x008c_4640;
/// `PlayerCharacter::SetFirstPerson(flag)` (`00950110`) and the player's
/// boolean test `004eaf60` the fading code asks first.
const PLAYER_SET_FIRST_PERSON: u32 = 0x0095_0110;
const PLAYER_VIEW_TEST: u32 = 0x004e_af60;
/// The game clock object (`011f6394`) and its time getter (`0084d030`, a
/// `double` in `ST0`), and the two game settings that divide it into a fade
/// step: the player's (`011cd7c8`) and everyone else's (`011cdab8`).
const FADE_CLOCK: u32 = 0x011f_6394;
const FADE_CLOCK_GET_TIME: u32 = 0x0084_d030;
const FADE_PLAYER_SETTING: u32 = 0x011c_d7c8;
const FADE_ACTOR_SETTING: u32 = 0x011c_dab8;
/// `Script::MoveToFunctionBase(actor, cell, x, y, z)` (`005ccb20`, cdecl).
const SCRIPT_MOVE_TO: u32 = 0x005c_cb20;
/// `TESObjectREFR::Activate(this = target, activator, 0, 0, 1)`
/// (`00573170`) and `TESObjectREFR::Disable` (`00574400`).
const REFERENCE_ACTIVATE: u32 = 0x0057_3170;
const REFERENCE_DISABLE: u32 = 0x0057_4400;
/// `MobileObject::GetDesiredProcessLevel` (`009334b0`).
const GET_DESIRED_PROCESS_LEVEL: u32 = 0x0093_34b0;
/// Slot `0xc4` of an actor, called with 1 once a delete-fade has finished.
const ACTOR_SLOT_DELETE_FADE_DONE: u32 = 0xc4;

// Translated from 008fe8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::FadeIn` (Xbox PDB): unless the fade state is 5 or 6, sets
/// it to 1 (`flag` zero) or 3, clears the fade reference and, when the alpha
/// is at least 1.0, resets it to 0.0 (`FCOMP`, `TEST AH,0x1`: C0 is set for
/// below and for unordered, so a NaN alpha is kept). The first stack word (the
/// actor) is not read.
pub fn high_process_fade_in(e: &mut Engine, this: Ptr, _unused_0: u32, flag: u8) {
    let base = this.addr();
    let state = e.mem.u32(base + FADE_STATE);
    if state == 6 || state == 5 {
        return;
    }
    e.mem
        .set_u32(base + FADE_STATE, if flag != 0 { 3 } else { 1 });
    e.mem.set_u32(base + FADE_REFERENCE, 0);
    let one = e.global::<f64>(ONE_DOUBLE);
    if f64::from(e.mem.f32(base + FADE_ALPHA)) >= one {
        e.mem.set_f32(base + FADE_ALPHA, 0.0);
    }
}

/// Sets the alpha to 1.0 when it is not above the `double` 0.0 at
/// `01012060` (the compiled test is `alpha <= 0.0`, false for NaN).
fn restart_fade_alpha(e: &mut Engine, base: u32) {
    let zero = e.global::<f64>(ZERO_DOUBLE);
    if f64::from(e.mem.f32(base + FADE_ALPHA)) <= zero {
        e.mem.set_f32(base + FADE_ALPHA, 1.0);
    }
}

// Translated from 008fe960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::FadeOut` (Xbox PDB): unless the fade state is 5 or 6, sets
/// it to 2 (`flag` zero) or 4, remembers `reference` (when not null) in
/// `pTeleportFadeRef` and restarts the alpha at 1.0 when it is not above 0.
/// The first stack word (the actor) is not read.
pub fn high_process_fade_out(e: &mut Engine, this: Ptr, _unused_0: u32, reference: u32, flag: u8) {
    let base = this.addr();
    let state = e.mem.u32(base + FADE_STATE);
    if state == 6 || state == 5 {
        return;
    }
    e.mem
        .set_u32(base + FADE_STATE, if flag != 0 { 4 } else { 2 });
    if reference != 0 {
        e.mem.set_u32(base + FADE_REFERENCE, reference);
    }
    restart_fade_alpha(e, base);
}

// Translated from 008fe9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::FadeOutAndMove` (Xbox PDB): unless the fade state is 5 or 6,
/// sets it to 7, stops the actor, resets its animations (`PickAnimations(1.0,
/// 1.0)`), stores a new 16-byte block (`cell`, `x`, `y`, `z`) in
/// `pMoveToFadeStruct` (+0x3f4) and restarts the alpha at 1.0 when it is
/// not above 0.
pub fn high_process_fade_out_and_move(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    cell: u32,
    x: f32,
    y: f32,
    z: f32,
) {
    let base = this.addr();
    let state = e.mem.u32(base + FADE_STATE);
    if state == 6 || state == 5 {
        return;
    }
    e.mem.set_u32(base + FADE_STATE, 7);
    e.call(ACTOR_STOP_MOVING, &args![actor]);
    e.call(ACTOR_PICK_ANIMATIONS, &args![actor, 1.0f32, 1.0f32]);
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    e.mem.set_u32(base + FADE_MOVE, block);
    e.mem.set_u32(block, cell);
    e.mem.set_f32(block + 4, x);
    e.mem.set_f32(block + 8, y);
    e.mem.set_f32(block + 12, z);
    restart_fade_alpha(e, base);
}

/// The shared body of `FadeAndDelete` and `FadeAndDisable`: the state is set,
/// the fade reference cleared, the alpha restarted and the actor stopped with
/// its animations reset; when the actor is the player and the view test
/// `004eaf60` fails, the first-person flag is cleared around the reset and
/// set afterwards.
fn fade_and_stop(e: &mut Engine, this: Ptr, actor: u32, state: u32) {
    let base = this.addr();
    e.mem.set_u32(base + FADE_STATE, state);
    e.mem.set_u32(base + FADE_REFERENCE, 0);
    restart_fade_alpha(e, base);
    let player = e.mem.u32(PLAYER);
    let mut view_ok = true;
    if actor == player {
        view_ok = e.call(PLAYER_VIEW_TEST, &args![player]).bool();
        if !view_ok {
            e.call(PLAYER_SET_FIRST_PERSON, &args![player, 0u32]);
        }
    }
    e.call(ACTOR_STOP_MOVING, &args![actor]);
    e.call(ACTOR_PICK_ANIMATIONS, &args![actor, 1.0f32, 1.0f32]);
    if !view_ok {
        let player = e.mem.u32(PLAYER);
        e.call(PLAYER_SET_FIRST_PERSON, &args![player, 1u32]);
    }
}

// Translated from 008feab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::FadeAndDelete` (Xbox PDB): fade state 6 (see
/// `fade_and_stop`).
pub fn high_process_fade_and_delete(e: &mut Engine, this: Ptr, actor: u32) {
    fade_and_stop(e, this, actor, 6);
}

// Translated from 008feb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::FadeAndDisable` (Xbox PDB): fade state 5 (see
/// `fade_and_stop`).
pub fn high_process_fade_and_disable(e: &mut Engine, this: Ptr, actor: u32) {
    fade_and_stop(e, this, actor, 5);
}

// Translated from 008ff030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SkipFadeIn` (Xbox PDB): in fade state 1 or 3 the alpha jumps
/// to 1.0, the state to 0 and the actor's alpha is updated.
pub fn high_process_skip_fade_in(e: &mut Engine, this: Ptr, actor: u32) {
    let base = this.addr();
    let state = e.mem.u32(base + FADE_STATE);
    if state == 1 || state == 3 {
        e.mem.set_f32(base + FADE_ALPHA, 1.0);
        e.mem.set_u32(base + FADE_STATE, 0);
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
    }
}

// Translated from 008ff080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the fade state is 3 to 6 (compared as
/// signed numbers).
pub fn fn_008ff080(e: &mut Engine, this: Ptr) -> bool {
    let state = e.mem.i32(this.addr() + FADE_STATE);
    (3..=6).contains(&state)
}

// Translated from 008fec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::FadeUpdate` (Xbox PDB): steps the fade of `actor`. In state
/// 0 the alpha is forced to 1.0 (and the actor's alpha updated) unless it is
/// there already. Otherwise the step is the game clock divided by a setting
/// (the player's or another actor's) and: states 1 and 3 add it (above 1.0
/// the alpha is clamped and the state returns to 0); state 2 subtracts it
/// while the alpha is above 0 (below 0 the alpha and the reference are
/// cleared); state 7 subtracts it and, at or below 0, calls
/// `Script::MoveToFunctionBase` with the stored block, frees the block and
/// clears the reference; state 4 activates the reference once the alpha has
/// run out (at once for the player, returning true, and for other actors
/// returning true when they have a process level, else fading them in);
/// state 6 calls actor slot `0xc4(1)` and state 5 disables the
/// reference when the alpha has run out. After the step the actor's alpha is
/// updated and the result is `fn_008ff080` (fade state 3 to 6).
pub fn high_process_fade_update(e: &mut Engine, this: Ptr, actor: u32) -> bool {
    let base = this.addr();
    let state = e.mem.u32(base + FADE_STATE);
    if state == 0 {
        let one = e.global::<f64>(ONE_DOUBLE);
        let below_one = f64::from(e.mem.f32(base + FADE_ALPHA)) < one;
        if !below_one {
            return false;
        }
        e.mem.set_f32(base + FADE_ALPHA, 1.0);
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
        return false;
    }
    let player = e.mem.u32(PLAYER);
    let setting = if actor == player {
        FADE_PLAYER_SETTING
    } else {
        FADE_ACTOR_SETTING
    };
    let clock = e.call(FADE_CLOCK_GET_TIME, &args![FADE_CLOCK]).f64();
    let divisor = setting_float(e, setting);
    let step = (clock / f64::from(divisor)) as f32;
    let zero = e.global::<f64>(ZERO_DOUBLE);
    let one = e.global::<f64>(ONE_DOUBLE);
    let mut alpha = e.mem.f32(base + FADE_ALPHA);
    match state {
        1 | 3 => {
            alpha += step;
            e.mem.set_f32(base + FADE_ALPHA, alpha);
            if f64::from(alpha) > one {
                e.mem.set_f32(base + FADE_ALPHA, 1.0);
                e.mem.set_u32(base + FADE_STATE, 0);
            }
        }
        2 => {
            if f64::from(alpha) > zero {
                alpha -= step;
                e.mem.set_f32(base + FADE_ALPHA, alpha);
                if f64::from(alpha) < zero {
                    e.mem.set_f32(base + FADE_ALPHA, 0.0);
                    e.mem.set_u32(base + FADE_REFERENCE, 0);
                }
            }
        }
        7 => {
            if f64::from(alpha) > zero {
                alpha -= step;
                e.mem.set_f32(base + FADE_ALPHA, alpha);
                if f64::from(alpha) <= zero {
                    e.mem.set_f32(base + FADE_ALPHA, 0.0);
                    let block = e.mem.u32(base + FADE_MOVE);
                    if block != 0 {
                        // Script::MoveToFunctionBase(actor, cell, x, y, z), the
                        // block's words passed as stored.
                        let cell = e.mem.u32(block);
                        let x = e.mem.u32(block + 4);
                        let y = e.mem.u32(block + 8);
                        let z = e.mem.u32(block + 12);
                        e.call(SCRIPT_MOVE_TO, &args![actor, cell, x, y, z]);
                        let block = e.mem.u32(base + FADE_MOVE);
                        e.call(OPERATOR_DELETE, &args![block]);
                    }
                    e.mem.set_u32(base + FADE_MOVE, 0);
                    e.mem.set_u32(base + FADE_REFERENCE, 0);
                }
            }
        }
        4 => {
            if actor == player {
                e.mem.set_f32(base + FADE_ALPHA, 1.0);
                let reference = e.mem.u32(base + FADE_REFERENCE);
                e.mem.set_u32(base + FADE_REFERENCE, 0);
                if reference != 0 {
                    e.call(
                        REFERENCE_ACTIVATE,
                        &args![reference, actor, 0u32, 0u32, 1u32],
                    );
                }
                e.mem.set_u32(base + FADE_STATE, 0);
                return true;
            }
            alpha -= step;
            e.mem.set_f32(base + FADE_ALPHA, alpha);
            if f64::from(alpha) < zero {
                e.mem.set_f32(base + FADE_ALPHA, 0.0);
                let reference = e.mem.u32(base + FADE_REFERENCE);
                e.mem.set_u32(base + FADE_REFERENCE, 0);
                if reference != 0 {
                    e.call(
                        REFERENCE_ACTIVATE,
                        &args![reference, actor, 0u32, 0u32, 1u32],
                    );
                }
                if actor == player || e.call(GET_DESIRED_PROCESS_LEVEL, &args![actor]).u32() == 0 {
                    high_process_fade_in(e, this, actor, 1);
                } else {
                    e.mem.set_u32(base + FADE_STATE, 0);
                    return true;
                }
            }
        }
        6 => {
            alpha -= step;
            e.mem.set_f32(base + FADE_ALPHA, alpha);
            if f64::from(alpha) < zero {
                e.mem.set_f32(base + FADE_ALPHA, 0.0);
                e.vcall(actor, ACTOR_SLOT_DELETE_FADE_DONE, &args![1u32]);
            }
        }
        5 => {
            alpha -= step;
            e.mem.set_f32(base + FADE_ALPHA, alpha);
            if f64::from(alpha) < zero {
                e.mem.set_f32(base + FADE_ALPHA, 0.0);
                e.call(REFERENCE_DISABLE, &args![actor]);
            }
        }
        _ => {}
    }
    e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
    fn_008ff080(e, this)
}

// ---- Special idles and sounds ----------------------------------------------------------

/// `TESIdleManager::SetUsedItem(item)` (`00600900`),
/// `::SetUsedItemLevel(level)` (`00600920`) and `::SetUsedItemActivate(flag)`
/// (`00600940`), all cdecl with one argument.
const IDLE_MANAGER_SET_USED_ITEM: u32 = 0x0060_0900;
const IDLE_MANAGER_SET_USED_ITEM_LEVEL: u32 = 0x0060_0920;
const IDLE_MANAGER_SET_USED_ITEM_ACTIVATE: u32 = 0x0060_0940;
/// `Actor::GetEyeLevel` (`008be940`, a `double` in `ST0`).
const ACTOR_GET_EYE_LEVEL: u32 = 0x008b_e940;
/// Actor slot `0x380`, a number returned in `ST0`.
const ACTOR_SLOT_0X380: u32 = 0x380;
/// Reference slot `0x218`, a boolean test.
const REFERENCE_SLOT_0X218: u32 = 0x218;
/// `BSSoundHandle` members: `IsValid` (`00ad8ce0`), `IsPlaying` (`00ad8930`),
/// `Stop` (`00ad88f0`), `Release` (`00ad8d10`) and the assignment `00418900`
/// (the other handle's address is the stack word); handles are 12 bytes.
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_HANDLE_IS_PLAYING: u32 = 0x00ad_8930;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_NEW: u32 = 0x0041_a250;
const SOUND_HANDLE_DELETE: u32 = 0x0048_3710;
/// `HighProcess::SoundHandle` (+0x314, two 12-byte handles).
const SOUND_HANDLES: u32 = 0x314;
/// `TESDataHandler::GetSound_ov2(name)` (`004616c0`, `this` is the data
/// handler), the name of the torch sound (`01088398`, "ITMTorchHeldLP"),
/// the sound's word at +0xc (`0084e3a0`, no stack word) and the owner's
/// `00933150(out handle, sound word, 1, 2, 1)` (RET 0x14).
const DATA_HANDLER_GET_SOUND: u32 = 0x0046_16c0;
const TORCH_SOUND_NAME: u32 = 0x0108_8398;
const SOUND_PREPARE: u32 = 0x0084_e3a0;
const OWNER_START_SOUND: u32 = 0x0093_3150;

// Translated from 008ff0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::FindSpecialIdletoPlay` (Xbox PDB): publishes `idle` and the
/// activate flag to the idle manager, derives a level (1 when `reference`
/// is not null and either the actor's eye position, lowered by its slot
/// `0x380` value, is below the reference's height or the reference's slot
/// `0x218` is true; else 0), asks the process's slot `0x44` (set up the
/// special idle: `(actor, 0, 2, 1, 0, 1)`) and resets the manager. Returns
/// what slot `0x44` returned.
pub fn high_process_find_special_idleto_play(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    idle: u32,
    reference: u32,
) -> bool {
    e.call(IDLE_MANAGER_SET_USED_ITEM, &args![idle]);
    e.call(IDLE_MANAGER_SET_USED_ITEM_ACTIVATE, &args![1u32]);
    let mut level = 0u32;
    if reference != 0 {
        let eye = e.call(ACTOR_GET_EYE_LEVEL, &args![actor]).f64();
        let location = e.vcall(actor, REFERENCE_SLOT_LOCATION, &args![]).u32();
        let height = f64::from(e.mem.f32(location + 8)) + eye;
        let lowered = e.vcall(actor, ACTOR_SLOT_0X380, &args![]).f64();
        let limit = (height - lowered) as f32;
        let target_location = e.vcall(reference, REFERENCE_SLOT_LOCATION, &args![]).u32();
        let target_height = e.mem.f32(target_location + 8);
        if limit < target_height || e.vcall(reference, REFERENCE_SLOT_0X218, &args![]).bool() {
            level = 1;
        }
    }
    e.call(IDLE_MANAGER_SET_USED_ITEM_LEVEL, &args![level]);
    let played = e
        .vcall(
            this.addr(),
            PROCESS_SLOT_SETUP_SPECIAL_IDLE,
            &args![actor, 0u32, 2u32, 1u32, 0u32, 1u32],
        )
        .bool();
    e.call(IDLE_MANAGER_SET_USED_ITEM, &args![0u32]);
    e.call(IDLE_MANAGER_SET_USED_ITEM_ACTIVATE, &args![0u32]);
    e.call(IDLE_MANAGER_SET_USED_ITEM_LEVEL, &args![0xffff_ffffu32]);
    played
}

// Translated from 008ff1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::StopSoundHandle` (Xbox PDB): a valid sound handle `index`
/// (the 12-byte entry at +0x314) is stopped when it is playing, released and
/// overwritten with a fresh empty handle. C++ exception unwinding is not
/// translated.
pub fn high_process_stop_sound_handle(e: &mut Engine, this: Ptr, index: u32) {
    let handle = this.addr() + SOUND_HANDLES + index.wrapping_mul(12);
    if !e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        return;
    }
    if e.call(SOUND_HANDLE_IS_PLAYING, &args![handle]).bool() {
        e.call(SOUND_HANDLE_STOP, &args![handle]);
    }
    e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    e.with_stack(12, |e, empty| {
        let made = e.call(SOUND_HANDLE_NEW, &args![empty]).u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
        e.call(SOUND_HANDLE_DELETE, &args![empty]);
    });
}

// Translated from 008ff290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::StartTorchSound` (Xbox PDB): when the second sound handle
/// (+0x320) is not valid, looks up the sound named "ITMTorchHeldLP" and, if
/// `owner` and the sound exist, starts it through the owner's `00933150`
/// into a temporary handle that is assigned to +0x320 and destroyed. C++
/// exception unwinding is not translated.
pub fn high_process_start_torch_sound(e: &mut Engine, this: Ptr, owner: u32) {
    let handle = this.addr() + SOUND_HANDLES + 12;
    if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        return;
    }
    let data_handler = e.mem.u32(DATA_HANDLER);
    let sound = e
        .call(
            DATA_HANDLER_GET_SOUND,
            &args![data_handler, TORCH_SOUND_NAME],
        )
        .u32();
    if owner == 0 || sound == 0 {
        return;
    }
    // 008ff2f7: `0084e3a0` takes no stack word (plain RET); the pushed 1, 2, 1 are
    // the last three words of `00933150` (RET 0x14).
    let prepared = e.call(SOUND_PREPARE, &args![sound]).u32();
    e.with_stack(12, |e, temporary| {
        let made = e
            .call(
                OWNER_START_SOUND,
                &args![owner, temporary, prepared, 1u32, 2u32, 1u32],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
        e.call(SOUND_HANDLE_DELETE, &args![temporary]);
    });
}

// ---- Combat detection ----------------------------------------------------------------

/// The table indexed by `iDetectionCounter` (+0x3d8) that decides how often
/// and how each process runs its combat detection (`011a3780`, 8-byte rows:
/// a word `mode`, then bytes at +4, +5 and +6).
const DETECTION_MODE_TABLE: u32 = 0x011a_3780;
/// `Actor::GetCombatGroup`-like getter `008a16b0` (the engine map names it
/// `Actor::IsCombatGroupMember`; its caller compares its result with an
/// actor), `Actor::IsInCombatWithActor` (`008bc700`, `(other)`) and
/// `008c1680(other)`.
const ACTOR_COMPARED_TARGET: u32 = 0x008a_16b0;
const ACTOR_IS_IN_COMBAT_WITH_ACTOR: u32 = 0x008b_c700;
const ACTOR_TEST_008C1680: u32 = 0x008c_1680;
/// The report `005b5e40(format, value)` the unknown-mode case calls (the
/// format string is at `010883a8`).
const REPORT_UNKNOWN_MODE: u32 = 0x005b_5e40;
const UNKNOWN_MODE_FORMAT: u32 = 0x0108_83a8;
/// `Actor::IsPointInViewCone(point, cone)` (`0088c570`) and the cone
/// constant `0101ff38`; the position of a reference (`00436aa0`).
const ACTOR_IS_POINT_IN_VIEW_CONE: u32 = 0x0088_c570;
const VIEW_CONE: u32 = 0x0101_ff38;
const REFERENCE_POSITION: u32 = 0x0043_6aa0;
/// Reference/actor tests: `00437bd0(reference)` (a boolean on the extra data
/// of the reference), `00493bb0(actor)` (a boolean, true for the player-like
/// actors), `Actor::GetShouldHelp` (`008b0970`, `(other)`),
/// `Actor::IsAlarmed` (`008a61b0`) and
/// `Actor::ShouldSkipFallOutBehavior(kind)` (`008a78f0`).
const REFERENCE_TEST_00437BD0: u32 = 0x0043_7bd0;
const ACTOR_TEST_00493BB0: u32 = 0x0049_3bb0;
const ACTOR_GET_SHOULD_HELP: u32 = 0x008b_0970;
const ACTOR_IS_ALARMED: u32 = 0x008a_61b0;
const ACTOR_SHOULD_SKIP_FALLOUT_BEHAVIOR: u32 = 0x008a_78f0;
/// `Actor::GetShouldAttackActor(this, other, 0, &result, 0)` (`008b06d0`) and
/// `TESObjectREFR::GetDistanceFromReference(this, other, 0, 1)`
/// (`005723b0`, a `double` in `ST0`).
const ACTOR_GET_SHOULD_ATTACK_ACTOR: u32 = 0x008b_06d0;
const REFERENCE_GET_DISTANCE: u32 = 0x0057_23b0;
/// The setting `ReactToCombatSituation` compares the distance with.
const REACT_DISTANCE_SETTING: u32 = 0x011c_dfe4;
/// Actor slots `0x428`/`0x42c` (the combat controller and target words the
/// reaction needs).
const ACTOR_SLOT_0X428_WORD: u32 = 0x428;
const ACTOR_SLOT_0X42C_WORD: u32 = 0x42c;
/// `HighProcess::SpectatorList` (+0x28c), `AggroList` (+0x274),
/// `GroupsToHelpList` (+0x27c), `AggroRadiusList` (+0x38c) and
/// `AvoidActorList` (+0x394): inline `BSSimpleList<StartCombatStates *>` /
/// `BSSimpleList<Actor *>` heads.
const SPECTATOR_LIST: u32 = 0x28c;
const AGGRO_LIST: u32 = 0x274;
const AGGRO_RADIUS_LIST: u32 = 0x38c;
const AVOID_ACTOR_LIST: u32 = 0x394;
const DETECTION_COUNTER_OFFSET: u32 = 0x3d8;

/// `operator new(0x14)` then `StartCombatStates` construction (null when the
/// allocation fails, as the game tests).
fn new_start_combat_states(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        0
    } else {
        fn_008f6120(e, Ptr::new(block)).addr()
    }
}

/// `x87` truncation of a `float` to a 32-bit integer (`FISTP` with the
/// truncating rounding mode): out-of-range values and NaN give `i32::MIN`.
fn truncate_float(value: f32) -> i32 {
    if value.is_nan() || value >= 2_147_483_648.0 || value < -2_147_483_648.0 {
        i32::MIN
    } else {
        value as i32
    }
}

/// `TESActorBaseData::IsInEvilFactionsOnly` of an actor's base data
/// (`004181e0(actor) + 0x30`, then `0047d740`).
fn base_is_in_evil_factions_only(e: &mut Engine, actor: u32) -> bool {
    let form = e.call(0x0041_81e0, &args![actor]).u32();
    e.call(0x0047_d740, &args![form + 0x30]).bool()
}

// Translated from 008ff350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the per-pair part of the detection pass that
/// `EvaluateDetection` runs for every actor `other` on the process list,
/// against the evaluated `actor`. Nothing happens when `other` is null, is
/// deleted (`00440da0`), is not an actor (slot `0x100`), is `actor`, is in
/// the faction of default object 8, when its detection level (the state of
/// slot `0x504`) does not exceed the threshold (setting `011cd45c`, 0 when
/// the actor's current package fails `0067a770`) or was evaluated already
/// (the state's byte +0x1f, set here), when `other` fails slot `0x22c(0)` or
/// `00437bd0`. Otherwise: with a detection level above 0, an other that
/// answers slot `0x274` and detects its slot-`0x42c` target can queue its
/// group in `GroupsToHelpList` (+0x27c); a combat reaction value is worked
/// out (`GetShouldAttackActor` both ways: 100 when the actor would attack,
/// the other going into `AvoidActorList` (+0x394) when the trigger roll
/// `0047eeb0(trigger)` beats `strength` divided by the other's combat
/// strength; a hostile-enough other within the sight distance from the
/// base form is close, and goes into `AggroRadiusList` (+0x38c) when the
/// detection level is above 0); then the other is queued in `AggroList`
/// (+0x274) with `plastDetected` (+0x2a4) set, or just recorded, and
/// `ReactToCombatSituation` runs. `_unused_*` are stack words the function
/// never reads. C++ exception unwinding is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_008ff350(
    e: &mut Engine,
    this: Ptr,
    other: u32,
    actor: u32,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    trigger: f32,
    strength: f32,
) {
    let base = this.addr();
    let mut level: i32 = -100;
    let base_form = e.call(0x0041_81e0, &args![actor]).u32();
    let sight: i32 = if base_form != 0 && e.call(0x0070_5cf0, &args![base_form + 0x90]).bool() {
        e.call(0x0082_5c00, &args![base_form + 0x90]).i32()
    } else {
        0
    };
    let mut close = false;
    let player_like = e.call(ACTOR_TEST_00493BB0, &args![actor]).bool();
    let own_group = e.vcall(actor, ACTOR_SLOT_GROUP, &args![]).u32();
    if other == 0
        || e.call(0x0044_0da0, &args![other]).bool()
        || !e.vcall(other, REFERENCE_SLOT_IS_ACTOR, &args![]).bool()
        || other == actor
    {
        return;
    }
    // BGSDefaultObjectManager::GetDefaultObject(8): the singleton getter
    // `0058d680` it is preceded by only creates the manager that
    // `0058db10` creates itself.
    let default_faction = e.call(0x0058_db10, &args![8u32]).u32();
    if e.call(0x008b_8e90, &args![other, default_faction]).bool() {
        return;
    }
    let state = e
        .vcall(base, PROCESS_SLOT_GET_DETECTION_STATE, &args![other, 0u32])
        .u32();
    if state != 0 {
        level = e.mem.i32(state + 8);
    }
    let package = e.call(0x0088_1510, &args![actor]).u32();
    let mut threshold = setting_float(e, 0x011c_d45c);
    if package != 0 && !e.call(0x0067_a770, &args![package]).bool() {
        threshold = 0.0;
    }
    let exceeds = f64::from(threshold) < f64::from(level);
    if !exceeds {
        return;
    }
    if state != 0 && e.mem.u8(state + 0x1f) != 0 {
        return;
    }
    if state != 0 {
        e.mem.set_u8(state + 0x1f, 1);
    }
    if e.vcall(other, PACKAGE_SLOT_TEST_0X22C, &args![0u32]).bool()
        || e.call(REFERENCE_TEST_00437BD0, &args![other]).bool()
    {
        return;
    }
    let mut scheduler = 0;
    let mut reaction: i32 = 0;
    let mut wants_help = false;
    // The detection level gates this, not the sight distance (008ff545:
    // `CMP [EBP-0x18],0` / `JLE`).
    if level > 0
        && e.vcall(other, 0x274, &args![u32::from(!player_like)])
            .bool()
    {
        let desired = e.vcall(other, ACTOR_SLOT_0X42C_WORD, &args![]).u32();
        if desired != 0 {
            let process = e.call(ACTOR_PROCESS, &args![other]).u32();
            let other_state = e
                .vcall(
                    process,
                    PROCESS_SLOT_GET_DETECTION_STATE,
                    &args![desired, 0u32],
                )
                .u32();
            wants_help = other_state != 0 && e.mem.i32(other_state + 8) > 0;
        }
    }
    if wants_help {
        let help_setting = setting_float(e, 0x011c_df10);
        let below = f64::from(help_setting) < f64::from(level);
        if below && !e.call(IS_IN_COMBATANT_FACTION, &args![other]).bool() {
            let other_group = e.vcall(other, ACTOR_SLOT_GROUP, &args![]).u32();
            if other_group != 0
                && own_group != other_group
                && e.call(0x005a_4320, &args![other_group]).u32() != 0
                && !fn_008f5410(e, this, other_group)
            {
                let manager = e.mem.u32(0x011f_1958);
                if e.call(0x0099_2530, &args![manager, actor, other]).bool() {
                    let mut queue = true;
                    let in_combat = e.call(0x0095_3c20, &args![e.mem.u32(PLAYER)]).i32();
                    let limit = e.call(0x0043_d4d0, &args![0x011c_f414u32]).u32();
                    let limit = e.mem.i32(limit);
                    if in_combat >= limit {
                        let player = e.mem.u32(PLAYER);
                        if e.call(0x0098_65b0, &args![other_group, player]).bool() {
                            let own_group_targets_player = own_group != 0
                                && e.call(0x0098_65b0, &args![own_group, player]).bool();
                            if !own_group_targets_player {
                                queue = false;
                            }
                        }
                    }
                    if queue {
                        let node = new_start_combat_states(e);
                        e.mem.set_u32(node + 4, other_group);
                        list_append(e, base + GROUPS_TO_HELP_LIST, node);
                    }
                }
            }
        }
    }
    let no_target = e.global::<f32>(NO_TARGET_FLOAT);
    let mut own_strength = e
        .call(CALCULATE_COMBAT_STRENGTH, &args![other, no_target])
        .f32();
    let zero = e.global::<f64>(ZERO_DOUBLE);
    if f64::from(own_strength) == zero {
        own_strength = 1.0;
    }
    if e.call(0x008a_ce90, &args![actor]).bool()
        || e.call(0x008a_ce90, &args![other]).bool()
        || actor == other
    {
        reaction = 0;
    } else {
        let mut handled = false;
        let flag = e.vcall(actor + 0xa4, 8, &args![0u32]).u32();
        if flag == 0 {
            // The attack verdict word at +0x64 (`EBP-0x8c`) of the exe's 0xf0-byte frame.
            e.with_stack(0xf0, |e, frame| {
                let result = frame.addr() + 0x64;
                e.mem.set_u32(result, 0);
                if e.call(
                    ACTOR_GET_SHOULD_ATTACK_ACTOR,
                    &args![other, actor, 0u32, result, 0u32],
                )
                .bool()
                {
                    let scaled = f64::from(strength) / f64::from(own_strength);
                    let roll_input = truncate_float(trigger) as u32 & 0xff;
                    let roll = e.call(0x0047_eeb0, &args![roll_input]).f64();
                    if roll > scaled {
                        list_append(e, base + AVOID_ACTOR_LIST, other);
                        handled = true;
                    }
                }
            });
        }
        if !handled {
            // The verdict word at +0x5c (`EBP-0x94`) and the difference at +0x68 (`EBP-0x88`) of
            // the exe's 0xf0-byte frame.
            e.with_stack(0xf0, |e, frame| {
                let result = frame.addr() + 0x5c;
                e.mem.set_u32(result, 0);
                if e.call(
                    ACTOR_GET_SHOULD_ATTACK_ACTOR,
                    &args![actor, other, 0u32, result, 0u32],
                )
                .bool()
                {
                    reaction = 100;
                } else if sight > 0 {
                    let kind = e.mem.u32(result);
                    if kind == 0 || kind == 1 {
                        let other_position = e.call(REFERENCE_POSITION, &args![other]).u32();
                        let actor_position = e.call(REFERENCE_POSITION, &args![actor]).u32();
                        let difference = frame.addr() + 0x68;
                        e.call(
                            0x0043_9ef0,
                            &args![actor_position, difference, other_position],
                        );
                        let squared = e.call(0x004a_7290, &args![difference]).f64();
                        let radius = f64::from(sight.wrapping_mul(sight));
                        if squared <= radius {
                            close = true;
                        }
                    }
                }
            });
        }
    }
    let skip_help = e.vcall(actor, 0x21c, &args![]).bool()
        || !e.vcall(other, REFERENCE_SLOT_0X218, &args![]).bool()
        || {
            let value = e.vcall(other, 0x37c, &args![]).u32();
            !e.call(0x0059_f610, &args![value]).bool()
        }
        || base_is_in_evil_factions_only(e, other);
    if skip_help && reaction > 0 {
        e.vcall(actor, 0x304, &args![]);
        let evil = base_is_in_evil_factions_only(e, actor);
        let node = new_start_combat_states(e);
        e.mem.set_u32(node, other);
        e.mem.set_u8(node + 9, u8::from(evil));
        list_append(e, base + AGGRO_LIST, node);
    } else if reaction > 0 {
        if scheduler == 0 && e.vcall(other, ACTOR_SLOT_0X428_WORD, &args![]).u32() != 0 {
            let owner = e.vcall(other, ACTOR_SLOT_0X428_WORD, &args![]).u32();
            scheduler = e.call(0x0040_30b0, &args![owner]).u32();
        }
        let both_combatants = e.call(IS_IN_COMBATANT_FACTION, &args![other]).bool()
            && scheduler != 0
            && e.call(IS_IN_COMBATANT_FACTION, &args![scheduler]).bool();
        if !both_combatants {
            e.vcall(actor, 0x304, &args![]);
            if !base_is_in_evil_factions_only(e, actor) {
                base_is_in_evil_factions_only(e, other);
            }
            let node = new_start_combat_states(e);
            e.mem.set_u32(node, other);
            list_append(e, base + AGGRO_LIST, node);
            e.mem.set_u32(base + 0x2a4, other);
        }
    } else if close
        // The detection level, not the sight distance (008ffbd7:
        // `CMP [EBP-0x18],0` / `JLE`).
        && level > 0
        && !e
            .call(ACTOR_SHOULD_SKIP_FALLOUT_BEHAVIOR, &args![actor, 6u32])
            .bool()
    {
        list_append(e, base + AGGRO_RADIUS_LIST, other);
    }
    high_process_react_to_combat_situation(e, this, actor, other);
}

// Translated from 008ffc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::CheckTargetCombatDetectionState` (Xbox PDB): by the mode of
/// the table row for `iDetectionCounter` (+0x3d8) at `011a3780`: mode 0 is
/// true when `008a16b0(a)` is `b`; mode 1 also when `Actor::IsInCombatWith`
/// `(a, b)`; mode 2 is true when `008a16b0(a)` is `b`, or when neither that
/// test nor `008c1680(a, b)` holds. Any other mode is reported (`005b5e40`)
/// and gives false. The results are false when none of those hold.
pub fn high_process_check_target_combat_detection_state(
    e: &mut Engine,
    this: Ptr,
    a: u32,
    b: u32,
) -> bool {
    let counter = e.mem.u32(this.addr() + DETECTION_COUNTER_OFFSET);
    let row = DETECTION_MODE_TABLE.wrapping_add(counter.wrapping_mul(8));
    let mode = e.mem.u32(row);
    match mode {
        0 => e.call(ACTOR_COMPARED_TARGET, &args![a]).u32() == b,
        1 => {
            e.call(ACTOR_COMPARED_TARGET, &args![a]).u32() == b
                || e.call(ACTOR_IS_IN_COMBAT_WITH_ACTOR, &args![a, b]).bool()
        }
        2 => {
            if e.call(ACTOR_COMPARED_TARGET, &args![a]).u32() == b {
                return true;
            }
            !e.call(ACTOR_IS_IN_COMBAT_WITH_ACTOR, &args![a, b]).bool()
                && !e.call(ACTOR_TEST_008C1680, &args![a, b]).bool()
        }
        _ => {
            e.call(REPORT_UNKNOWN_MODE, &args![UNKNOWN_MODE_FORMAT, mode]);
            false
        }
    }
}

// Translated from 008ffd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ShouldRunCombatDetection` (Xbox PDB): false when `b` fails
/// slot `0x22c(0)` or `00437bd0`, or when `CheckTargetCombatDetectionState(a,
/// b)` fails, or when the row's byte +4 is set (never for the player as
/// `b`) and `b`'s position is outside `a`'s view cone (the constant at
/// `0101ff38`); else true.
pub fn high_process_should_run_combat_detection(e: &mut Engine, this: Ptr, a: u32, b: u32) -> bool {
    if e.vcall(b, PACKAGE_SLOT_TEST_0X22C, &args![0u32]).bool()
        || e.call(REFERENCE_TEST_00437BD0, &args![b]).bool()
    {
        return false;
    }
    let counter = e.mem.u32(this.addr() + DETECTION_COUNTER_OFFSET);
    let row = DETECTION_MODE_TABLE.wrapping_add(counter.wrapping_mul(8));
    let mut check_view = e.mem.u8(row + 4);
    if b == e.mem.u32(PLAYER) {
        check_view = 0;
    }
    if !high_process_check_target_combat_detection_state(e, this, a, b) {
        return false;
    }
    if check_view != 0 {
        let position = e.call(REFERENCE_POSITION, &args![b]).u32();
        let cone = e.global::<f32>(VIEW_CONE);
        if !e
            .call(ACTOR_IS_POINT_IN_VIEW_CONE, &args![a, position, cone])
            .bool()
        {
            return false;
        }
    }
    true
}

// Translated from 008ffdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ShouldRunCombatDetectionEventCheck` (Xbox PDB): false when
/// the row's byte +5 is zero, else `CheckTargetCombatDetectionState(a, b)`.
pub fn high_process_should_run_combat_detection_event_check(
    e: &mut Engine,
    this: Ptr,
    a: u32,
    b: u32,
) -> bool {
    let counter = e.mem.u32(this.addr() + DETECTION_COUNTER_OFFSET);
    let row = DETECTION_MODE_TABLE.wrapping_add(counter.wrapping_mul(8));
    if e.mem.u8(row + 5) == 0 {
        return false;
    }
    high_process_check_target_combat_detection_state(e, this, a, b)
}

// Translated from 008ffe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ShouldRunPlayerDetection` (Xbox PDB): true when
/// `00493bb0(actor)` is false, else whether the row's byte +6 is set.
pub fn high_process_should_run_player_detection(e: &mut Engine, this: Ptr, actor: u32) -> bool {
    if !e.call(ACTOR_TEST_00493BB0, &args![actor]).bool() {
        return true;
    }
    let counter = e.mem.u32(this.addr() + DETECTION_COUNTER_OFFSET);
    let row = DETECTION_MODE_TABLE.wrapping_add(counter.wrapping_mul(8));
    e.mem.u8(row + 6) != 0
}

// Translated from 008ffe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ReactToCombatSituation` (Xbox PDB): decides whether `a`
/// should join a fight `b` is in, and when it does appends a new
/// `StartCombatStates` (actor `b`, group from `b`'s slot `0x3f8`, flag byte
/// +0xd = 1) to `SpectatorList` (+0x28c). Nothing is queued when `a` would
/// help `b` already (`GetShouldHelp`) or would attack `b`'s target
/// (slot `0x42c`), when `b` (not the player) lacks slots `0x428`/`0x42c`,
/// when `a` skips the behaviour kind 2, is alarmed or fails `00493bb0`, or
/// when the distance between them is above the setting `011cdfe4`. Always
/// returns false.
pub fn high_process_react_to_combat_situation(e: &mut Engine, this: Ptr, a: u32, b: u32) -> bool {
    let target = e.vcall(b, ACTOR_SLOT_0X42C_WORD, &args![]).u32();
    if e.call(ACTOR_GET_SHOULD_HELP, &args![a, b]).bool() {
        return false;
    }
    if target != 0 {
        // The verdict word at +0x20 (`EBP-0x14`) of the exe's 0x34-byte frame.
        let attacks = e.with_stack(0x34, |e, frame| {
            let result = frame.addr() + 0x20;
            e.mem.set_u32(result, 0);
            e.call(
                ACTOR_GET_SHOULD_ATTACK_ACTOR,
                &args![a, target, 0u32, result, 0u32],
            )
            .bool()
        });
        if attacks {
            return false;
        }
    }
    if b != e.mem.u32(PLAYER)
        && (e.vcall(b, ACTOR_SLOT_0X428_WORD, &args![]).u32() == 0
            || e.vcall(b, ACTOR_SLOT_0X42C_WORD, &args![]).u32() == 0)
    {
        return false;
    }
    if e.call(ACTOR_SHOULD_SKIP_FALLOUT_BEHAVIOR, &args![a, 2u32])
        .bool()
        || e.call(ACTOR_IS_ALARMED, &args![a]).bool()
        || e.call(ACTOR_TEST_00493BB0, &args![a]).bool()
    {
        return false;
    }
    let distance = e
        .call(REFERENCE_GET_DISTANCE, &args![b, a, 0u32, 1u32])
        .f64();
    let limit = setting_float(e, REACT_DISTANCE_SETTING);
    let within = f64::from(limit) >= distance;
    if !within {
        return false;
    }
    let node = new_start_combat_states(e);
    e.mem.set_u32(node, b);
    let group = e.vcall(b, ACTOR_SLOT_GROUP, &args![]).u32();
    e.mem.set_u32(node + 4, group);
    e.mem.set_u8(node + 0xd, 1);
    list_append(e, this.addr() + SPECTATOR_LIST, node);
    false
}

// Translated from 00900000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a stub that takes three stack words and returns
/// false.
pub fn fn_00900000(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
) -> bool {
    false
}

// ---- Combat entry, actor values, followers, spells, head tracking --------------------------

/// `MiddleHighProcess::EnterCombat` (`00916880`, 13 stack words, the byte
/// parameters passed zero-extended).
const MIDDLE_HIGH_ENTER_COMBAT: u32 = 0x0091_6880;
/// Process slots: `0x324` `GetFinishingCombatPackage`, `0x848` the armour
/// re-equip call (`(actor, 0, 0)`), `0x33c` `EnterCombat`, `0x7cc` and
/// `0x298` (`ProcessFollow`) the summon-defend fallbacks.
const PROCESS_SLOT_GET_FINISHING_COMBAT_PACKAGE: u32 = 0x324;
const PROCESS_SLOT_ENTER_COMBAT: u32 = 0x33c;
const PROCESS_SLOT_SUMMON_FALLBACK: u32 = 0x7cc;
const PROCESS_SLOT_PROCESS_FOLLOW: u32 = 0x298;
/// Setting copied into `fReEquipArmorTimer` by `EnterCombat`.
const RE_EQUIP_ARMOR_SETTING: u32 = 0x011c_da64;
/// Package type code (`0041ca90`) of the run-once packages `EnterCombat`
/// does not interrupt.
const PACKAGE_TYPE_NO_COMBAT: u32 = 0x10;
/// `PlayerCharacter::IsPlayerCharacterInCombat(&flag)` (`00953c50`) and the
/// `ProcessLists` query `00971c30(player, 0x12, 1)` (`this` is the process
/// lists object `011e0e80`), a `BSSimpleList` of actors.
const PLAYER_IS_IN_COMBAT: u32 = 0x0095_3c50;
const PROCESS_LISTS_FIND_ACTORS: u32 = 0x0097_1c30;
/// The cached actor-value object (+0x428): size `0x268`, constructor
/// `008c6e90`, `008c6f40(index)` marks an entry out of date, `008c6f00(index,
/// value)` stores one.
const VALUE_CACHE_SIZE: u32 = 0x268;
const VALUE_CACHE_CONSTRUCTOR: u32 = 0x008c_6e90;
const VALUE_CACHE_INVALIDATE: u32 = 0x008c_6f40;
const VALUE_CACHE_STORE: u32 = 0x008c_6f00;
/// `MiddleLowProcess` actor-value members: `TempModActorValue`
/// (`0092ce70`), `_ov2` (`0092cec0`), `DamageModActorValue` (`00907430`),
/// `_ov2` (`0092cf30`) and `GetActorFloatValue` (`0092ce00`, the float in
/// `ST0`); the test `00406d70(index, 0x100)` after which the cache is not
/// touched.
const LOW_PROCESS_TEMP_MOD: u32 = 0x0092_ce70;
const LOW_PROCESS_TEMP_MOD_OV2: u32 = 0x0092_cec0;
const LOW_PROCESS_DAMAGE_MOD: u32 = 0x0090_7430;
const LOW_PROCESS_DAMAGE_MOD_OV2: u32 = 0x0092_cf30;
const LOW_PROCESS_GET_ACTOR_FLOAT_VALUE: u32 = 0x0092_ce00;
const ACTOR_VALUE_INDEX_TEST: u32 = 0x0040_6d70;
const ACTOR_VALUE_INDEX_LIMIT: u32 = 0x100;
/// Process slot `0x39c`: `GetActorFloatValue` (the float in `ST0`); the
/// rounding helper `00404040(float)` (a `double` in `ST0`) and `_ftol2`
/// (`00ec62c0`, takes the `double`).
const PROCESS_SLOT_GET_ACTOR_FLOAT_VALUE: u32 = 0x39c;
const FLOAT_FLOOR: u32 = 0x0040_4040;
const FLOAT_TO_INTEGER: u32 = 0x00ec_62c0;
/// `ExtraDataList` calls of `UpdateFollowers`: the follower data
/// (`00422700`), `RemoveFollower(item)` (`00422690`) and `RemoveFollowerExtra`
/// (`00422720`); the list add `00905820(list, &item)`; `Actor::CouldBeFollowing`
/// (`008bc860`, `(leader)`); `MobileObject::GetCurrentProcessType`
/// (`00931850`); `00693d50(process, hour)`.
const EXTRA_DATA_FOLLOWERS: u32 = 0x0042_2700;
const EXTRA_DATA_REMOVE_FOLLOWER: u32 = 0x0042_2690;
const EXTRA_DATA_REMOVE_FOLLOWER_EXTRA: u32 = 0x0042_2720;
const FOLLOWER_LIST_ADD: u32 = 0x0090_5820;
const ACTOR_COULD_BE_FOLLOWING: u32 = 0x008b_c860;
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
const PROCESS_SET_HOUR: u32 = 0x0069_3d50;
/// Follower update calls on the follower (actor slots): `0x25c(hours)`,
/// `0x2b0(accumulated)`, `0x2b4` (a number in `ST0`), `0x2b8`.
const FOLLOWER_SLOT_UPDATE: u32 = 0x25c;
const FOLLOWER_SLOT_WAIT: u32 = 0x2b0;
const FOLLOWER_SLOT_WAIT_TIME: u32 = 0x2b4;
const FOLLOWER_SLOT_STOP_WAIT: u32 = 0x2b8;
/// `float` used as the follower update length when none is given
/// (`01084838`) and the `double` 0.25 (`010290b0`).
const DEFAULT_FOLLOWER_HOURS: u32 = 0x0108_4838;
const FOLLOWER_WAIT_STEP: u32 = 0x0102_90b0;
/// `TESNPC` spell list at +0x7c of an actor base and the leveled-spell
/// expansion `0050c1d0(leveled, actor)` of an entry.
const BASE_SPELL_LIST: u32 = 0x0048_d150;
const EXPAND_LEVELED_SPELL: u32 = 0x0050_c1d0;
/// Head-tracking: `HighProcess::OnNewHeadTrackTarget` (`009014d0`).
const ON_NEW_HEAD_TRACK_TARGET: u32 = 0x0090_14d0;

/// A new empty `BSSimpleList` (`operator new(8)` and its constructor; null
/// when the allocation fails).
fn new_simple_list(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(LIST_CONSTRUCTOR, &args![block]).u32()
    }
}

/// The process's cached actor-value object (+0x428), created first when it
/// does not exist.
fn ensure_value_cache(e: &mut Engine, base: u32) -> u32 {
    if e.mem.u32(base + 0x428) == 0 {
        let block = e.call(OPERATOR_NEW, &args![VALUE_CACHE_SIZE]).u32();
        let cache = if block == 0 {
            0
        } else {
            e.call(VALUE_CACHE_CONSTRUCTOR, &args![block]).u32()
        };
        e.mem.set_u32(base + 0x428, cache);
    }
    e.mem.u32(base + 0x428)
}

/// What the actor-value modifiers do after the `MiddleLowProcess` call: when
/// the index passes `00406d70(index, 0x100)` nothing more happens; otherwise
/// the cache entry for `index` is marked out of date (`008c6f40`).
fn invalidate_cached_value(e: &mut Engine, base: u32, index: u32) {
    if e.call(
        ACTOR_VALUE_INDEX_TEST,
        &args![index, ACTOR_VALUE_INDEX_LIMIT],
    )
    .bool()
    {
        return;
    }
    let cache = ensure_value_cache(e, base);
    e.call(VALUE_CACHE_INVALIDATE, &args![cache, index]);
}

// Translated from 009001c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::EnterCombat` (Xbox PDB): returns false when
/// `GetFinishingCombatPackage` (slot `0x324`) is set or the run-once package
/// (slot `0x20c`) has type code `0x10`. Otherwise, when the armour was taken
/// off to swim (+0x3a8) it calls slot `0x848(actor, 0, 0)`, clears the flag
/// and restarts `fReEquipArmorTimer` (+0x3a4) from the setting `011cda64`.
/// Then returns `MiddleHighProcess::EnterCombat` (`00916880`) with all 13
/// words (the byte-sized ones zero-extended). The word named `target` is the
/// second one; the other names only number the words.
#[allow(clippy::too_many_arguments)]
pub fn high_process_enter_combat(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    target: u32,
    flag_2: u8,
    flag_3: u8,
    word_4: u32,
    flag_5: u8,
    flag_6: u8,
    flag_7: u8,
    flag_8: u8,
    flag_9: u8,
    flag_10: u8,
    flag_11: u8,
    word_12: u32,
) -> bool {
    let base = this.addr();
    let run_once = e
        .vcall(base, PROCESS_SLOT_GET_RUN_ONCE_PACKAGE, &args![])
        .u32();
    e.vcall(base, PROCESS_SLOT_GET_CURRENT_PACKAGE, &args![]);
    let finishing = e
        .vcall(base, PROCESS_SLOT_GET_FINISHING_COMBAT_PACKAGE, &args![])
        .bool();
    if finishing
        || (run_once != 0 && e.call(PACKAGE_TYPE, &args![run_once]).u32() == PACKAGE_TYPE_NO_COMBAT)
    {
        return false;
    }
    if e.mem.u8(base + 0x3a8) != 0 {
        e.vcall(base, PROCESS_SLOT_0X848, &args![actor, 0u32, 0u32]);
        e.mem.set_u8(base + 0x3a8, 0);
        let timer = setting_float(e, RE_EQUIP_ARMOR_SETTING);
        e.mem.set_f32(base + 0x3a4, timer);
    }
    e.call(
        MIDDLE_HIGH_ENTER_COMBAT,
        &args![
            this,
            actor,
            target,
            u32::from(flag_2),
            u32::from(flag_3),
            word_4,
            u32::from(flag_5),
            u32::from(flag_6),
            u32::from(flag_7),
            u32::from(flag_8),
            u32::from(flag_9),
            u32::from(flag_10),
            u32::from(flag_11),
            word_12
        ],
    )
    .bool()
}

// Translated from 00900020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ProcessSummonCreatureDefend` (Xbox PDB): a process with no
/// commanding actor (+0x158) adds `(actor, 1)` to the procedure index (slot
/// `0x288`). With a commander that satisfies `00493bb0` or is the player in
/// combat (`00953c50`): for the player each actor `ProcessLists` finds
/// (`00971c30(player, 0x12, 1)`) that slot `0x100` accepts is entered into
/// combat with `actor` through the process's slot `0x33c` (13 stack words);
/// another commander does nothing more. For any other commander slot
/// `0x7cc(actor)` runs, or for the player slot `0x298(actor, 0, 0x101, 0)`.
pub fn high_process_process_summon_creature_defend(e: &mut Engine, this: Ptr, actor: u32) {
    let base = this.addr();
    let commander = e.mem.u32(base + 0x158);
    if commander == 0 {
        e.vcall(
            base,
            PROCESS_SLOT_ADD_TO_PROCEDURE_INDEX,
            &args![actor, 1u32],
        );
        return;
    }
    let player = e.mem.u32(PLAYER);
    // The flag byte at +0x17 (`EBP-1`) of the exe's 0x18-byte frame.
    let in_summon_fight = e.with_stack(0x18, |e, frame| {
        let flag_cell = frame.addr() + 0x17;
        e.mem.set_u8(flag_cell, 0);
        e.call(ACTOR_TEST_00493BB0, &args![commander]).bool()
            || (commander == player
                && e.call(PLAYER_IS_IN_COMBAT, &args![player, flag_cell])
                    .bool())
    });
    if in_summon_fight {
        if commander != player {
            return;
        }
        let mut node = e
            .call(
                PROCESS_LISTS_FIND_ACTORS,
                &args![PROCESS_LISTS, player, 0x12u32, 1u32],
            )
            .u32();
        while node != 0 && list_item(e, node) != 0 {
            let candidate = list_item(e, node);
            if e.vcall(candidate, REFERENCE_SLOT_IS_ACTOR, &args![]).bool() {
                e.vcall(
                    base,
                    PROCESS_SLOT_ENTER_COMBAT,
                    &args![
                        actor, candidate, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32,
                        1u32, 0u32
                    ],
                );
            }
            node = list_next(e, node);
        }
    } else if commander != player {
        e.vcall(base, PROCESS_SLOT_SUMMON_FALLBACK, &args![actor]);
    } else {
        e.vcall(
            base,
            PROCESS_SLOT_PROCESS_FOLLOW,
            &args![actor, 0u32, 0x101u32, 0u32],
        );
    }
}

// Translated from 009002b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetActorValue` (Xbox PDB): the process's slot `0x39c(a, b,
/// c)` (`GetActorFloatValue`) rounded down (`00404040`) and truncated to an
/// integer (`_ftol2`).
pub fn high_process_get_actor_value(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32) -> i32 {
    let value = e
        .vcall(
            this.addr(),
            PROCESS_SLOT_GET_ACTOR_FLOAT_VALUE,
            &args![a, b, c],
        )
        .f32();
    let floor = e.call(FLOAT_FLOOR, &args![value]).f64();
    e.call(FLOAT_TO_INTEGER, &args![floor]).i32()
}

// Translated from 00900400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte of the 8-byte entry `index` of an
/// `ActorValueCache` (offset `index * 8`).
pub fn fn_00900400(e: &mut Engine, this: Ptr, index: u32) -> u8 {
    e.mem.u8(this.addr().wrapping_add(index.wrapping_mul(8)))
}

// Translated from 009003e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `float` of the 8-byte entry `index` of an
/// `ActorValueCache` (offset `index * 8 + 4`).
pub fn fn_009003e0(e: &mut Engine, this: Ptr, index: u32) -> f32 {
    e.mem
        .f32(this.addr().wrapping_add(index.wrapping_mul(8)) + 4)
}

// Translated from 009002f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetActorFloatValue` (Xbox PDB): creates the value cache
/// (+0x428, 0x268 bytes) when missing. When the cache entry `b` is flagged
/// (`fn_00900400`) the value is worked out by `MiddleLowProcess::
/// GetActorFloatValue(a, b, c)` (`0092ce00`) and stored (`008c6f00`). Returns
/// the cache's `float` for `b`.
pub fn high_process_get_actor_float_value(
    e: &mut Engine,
    this: Ptr,
    a: u32,
    b: u32,
    c: u32,
) -> f32 {
    let cache = ensure_value_cache(e, this.addr());
    if fn_00900400(e, Ptr::new(cache), b) != 0 {
        let value = e
            .call(LOW_PROCESS_GET_ACTOR_FLOAT_VALUE, &args![this, a, b, c])
            .f32();
        e.call(VALUE_CACHE_STORE, &args![cache, b, value]);
    }
    fn_009003e0(e, Ptr::new(cache), b)
}

// Translated from 00900420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::TempModActorValue` (Xbox PDB): `MiddleLowProcess::
/// TempModActorValue(a, b, c)` and then, unless `00406d70(b, 0x100)` holds,
/// the cache entry `b` is marked out of date. C++ exception unwinding is not
/// translated.
pub fn high_process_temp_mod_actor_value(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32) {
    e.call(LOW_PROCESS_TEMP_MOD, &args![this, a, b, c]);
    invalidate_cached_value(e, this.addr(), b);
}

// Translated from 009004f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::TempModActorValue_ov2` (Xbox PDB): as `TempModActorValue`
/// with a `float` third word (`MiddleLowProcess::TempModActorValue_ov2`).
pub fn high_process_temp_mod_actor_value_ov2(e: &mut Engine, this: Ptr, a: u32, b: u32, c: f32) {
    e.call(LOW_PROCESS_TEMP_MOD_OV2, &args![this, a, b, c]);
    invalidate_cached_value(e, this.addr(), b);
}

// Translated from 009005d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::DamageModActorValue` (Xbox PDB): `MiddleLowProcess::
/// DamageModActorValue(a, b, c)` and the same cache invalidation as
/// `TempModActorValue`.
pub fn high_process_damage_mod_actor_value(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32) {
    e.call(LOW_PROCESS_DAMAGE_MOD, &args![this, a, b, c]);
    invalidate_cached_value(e, this.addr(), b);
}

// Translated from 009006a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::DamageModActorValue_ov2` (Xbox PDB): as `DamageModActorValue`
/// with a `float` third word (`MiddleLowProcess::DamageModActorValue_ov2`).
pub fn high_process_damage_mod_actor_value_ov2(e: &mut Engine, this: Ptr, a: u32, b: u32, c: f32) {
    e.call(LOW_PROCESS_DAMAGE_MOD_OV2, &args![this, a, b, c]);
    invalidate_cached_value(e, this.addr(), b);
}

// Translated from 00900780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetCachedActorValueOutOfDate` (Xbox PDB): creates the value
/// cache when missing and marks its entry `index` out of date (`008c6f40`).
pub fn high_process_set_cached_actor_value_out_of_date(e: &mut Engine, this: Ptr, index: u32) {
    let cache = ensure_value_cache(e, this.addr());
    if cache != 0 {
        e.call(VALUE_CACHE_INVALIDATE, &args![cache, index]);
    }
}

// Translated from 00900830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::UpdateFollowers` (Xbox PDB): walks the follower data of
/// `actor`'s extra data list (nothing happens without one). Every follower
/// is copied to a work list; each one that is not the player and could still
/// follow `actor` (`008bc860`) is updated: a follower with a process level
/// gets slot `0x25c(hours)` (`hours` is replaced by the float at `01084838`
/// when it is not positive; level 3 first sets the hour `Calendar::GetHour -
/// 1.0` on its process through `00693d50`); a follower without one gets
/// `0x25c(0.0)` and, when it is an actor whose process type is 0, either
/// `0x2b0(accumulated)` (adding `clock * count * 0.25` to the running total)
/// or `0x2b8` depending on its wait time (slot `0x2b4`), `00437bd0` and slot
/// `0x22c`. Followers that cannot follow any more are removed from the extra
/// data afterwards, and the follower extra is removed when the list ends up
/// empty. C++ exception unwinding is not translated.
pub fn high_process_update_followers(e: &mut Engine, _this: Ptr, actor: u32, hours: f32) {
    let list_owner = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
    let followers = if list_owner != 0 {
        let list_owner = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
        e.call(EXTRA_DATA_FOLLOWERS, &args![list_owner]).u32()
    } else {
        0
    };
    if followers == 0 {
        return;
    }
    let to_remove = new_simple_list(e);
    let work = new_simple_list(e);
    let mut node = e.mem.u32(followers + 0xc);
    let mut accumulated = 0.0f32;
    while node != 0 {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        e.call(FOLLOWER_LIST_ADD, &args![work, slot]);
        node = list_next(e, node);
    }
    let player = e.mem.u32(PLAYER);
    let mut cursor = work;
    while cursor != 0 && list_item(e, cursor) != 0 {
        let array = e.call(PROCESS_LISTS_GET_ARRAY, &args![PROCESS_LISTS]).u32();
        let count = e.call(PROCESS_ARRAY_COUNT, &args![array, 0u32]).u32();
        let follower = list_item(e, cursor);
        if follower != 0 && follower != player {
            if !e
                .call(ACTOR_COULD_BE_FOLLOWING, &args![follower, actor])
                .bool()
            {
                list_append(e, to_remove, follower);
            } else {
                let process_type = e.call(GET_CURRENT_PROCESS_TYPE, &args![follower]).i32();
                if process_type != 0 {
                    let mut length = hours;
                    let zero = e.global::<f64>(ZERO_DOUBLE);
                    if f64::from(length) <= zero {
                        length = e.global::<f32>(DEFAULT_FOLLOWER_HOURS);
                    }
                    if process_type == 3 {
                        let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
                        let adjusted = (hour - e.global::<f64>(ONE_DOUBLE)) as f32;
                        let process = e.call(ACTOR_PROCESS, &args![follower]).u32();
                        e.call(PROCESS_SET_HOUR, &args![process, adjusted]);
                    }
                    e.vcall(follower, FOLLOWER_SLOT_UPDATE, &args![length]);
                } else {
                    e.vcall(follower, FOLLOWER_SLOT_UPDATE, &args![0.0f32]);
                    if e.vcall(follower, REFERENCE_SLOT_IS_ACTOR, &args![]).bool()
                        && e.call(GET_CURRENT_PROCESS_TYPE, &args![follower]).i32() == 0
                    {
                        let wait = e.vcall(follower, FOLLOWER_SLOT_WAIT_TIME, &args![]).f64();
                        let zero = e.global::<f64>(ZERO_DOUBLE);
                        let waiting = wait < zero
                            && !e.call(0x0043_7bd0, &args![follower]).bool()
                            && !e
                                .vcall(follower, PACKAGE_SLOT_TEST_0X22C, &args![0u32])
                                .bool();
                        if waiting {
                            e.vcall(follower, FOLLOWER_SLOT_WAIT, &args![accumulated]);
                            let step = e.global::<f64>(FOLLOWER_WAIT_STEP);
                            let clock = e.call(FADE_CLOCK_GET_TIME, &args![FADE_CLOCK]).f64();
                            accumulated =
                                (clock * (f64::from(count) * step) + f64::from(accumulated)) as f32;
                        } else {
                            e.vcall(follower, FOLLOWER_SLOT_STOP_WAIT, &args![]);
                        }
                    }
                }
            }
        }
        cursor = list_next(e, cursor);
    }
    let mut cursor = to_remove;
    while cursor != 0 && list_item(e, cursor) != 0 {
        let removed = list_item(e, cursor);
        let list_owner = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
        e.call(EXTRA_DATA_REMOVE_FOLLOWER, &args![list_owner, removed]);
        cursor = list_next(e, cursor);
    }
    e.call(LIST_CLEAR, &args![to_remove]);
    if to_remove != 0 {
        e.call(LIST_DESTROY, &args![to_remove, 1u32]);
    }
    e.call(LIST_CLEAR, &args![work]);
    if work != 0 {
        e.call(LIST_DESTROY, &args![work, 1u32]);
    }
    let follower_list = e.mem.u32(followers + 0xc);
    if e.call(LIST_IS_EMPTY, &args![follower_list]).bool() {
        let list_owner = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
        e.call(EXTRA_DATA_REMOVE_FOLLOWER_EXTRA, &args![list_owner]);
    }
}

// Translated from 00900c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetLeveledSpells` (Xbox PDB): the process's spell list
/// (+0x3b4). Null for a null actor or one without base data; an existing
/// list is returned as it is. Otherwise every entry of the base data's
/// spell list (`0048d150(base + 0x7c)`) is expanded for `actor`
/// (`0050c1d0`) and each resulting spell whose type (slot `0x18` of its
/// +0x18 sub-object) is not 4, and, for type 1, only when `flag` is set,
/// is appended to a list created on first use and stored at +0x3b4. The
/// expansion lists are emptied and freed. C++ exception unwinding is not
/// translated.
pub fn high_process_get_leveled_spells(e: &mut Engine, this: Ptr, actor: u32, flag: u8) -> Ptr {
    let base = this.addr();
    if actor == 0 || e.call(0x0041_81e0, &args![actor]).u32() == 0 {
        return Ptr::new(0);
    }
    if e.mem.u32(base + 0x3b4) != 0 {
        return Ptr::new(e.mem.u32(base + 0x3b4));
    }
    let form = e.call(0x0041_81e0, &args![actor]).u32();
    let mut outer = e.call(BASE_SPELL_LIST, &args![form + 0x7c]).u32();
    while outer != 0 && !e.call(LIST_IS_EMPTY, &args![outer]).bool() {
        let entry = list_item(e, outer);
        if entry != 0 {
            let expanded = e.call(EXPAND_LEVELED_SPELL, &args![entry, actor]).u32();
            let mut inner = expanded;
            while inner != 0 && !e.call(LIST_IS_EMPTY, &args![inner]).bool() {
                let spell = list_item(e, inner);
                if spell != 0 {
                    let kind = e.vcall(spell + 0x18, 0x18, &args![]).u32();
                    if kind != 4 {
                        let mut accept = true;
                        if e.vcall(spell + 0x18, 0x18, &args![]).u32() == 1 {
                            accept = flag != 0;
                        }
                        if accept {
                            if e.mem.u32(base + 0x3b4) == 0 {
                                let list = new_simple_list(e);
                                e.mem.set_u32(base + 0x3b4, list);
                            }
                            let list = e.mem.u32(base + 0x3b4);
                            list_append(e, list, spell);
                        }
                    }
                }
                inner = list_next(e, inner);
            }
            if expanded != 0 {
                e.call(LIST_CLEAR, &args![expanded]);
                e.call(LIST_DESTROY, &args![expanded, 1u32]);
            }
        }
        outer = list_next(e, outer);
    }
    Ptr::new(e.mem.u32(base + 0x3b4))
}

// Translated from 00900e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetHasHealingSpells` (Xbox PDB): stores `iHasHealingSpell`
/// (+0x3ac).
pub fn high_process_set_has_healing_spells(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x3ac, value);
}

// Translated from 00900e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetHasHealingSpells` (Xbox PDB): `iHasHealingSpell`
/// (+0x3ac).
pub fn high_process_get_has_healing_spells(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x3ac)
}

// Translated from 00900e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetHasHealingPotions` (Xbox PDB): stores `iHasHealingPotion`
/// (+0x3b0).
pub fn high_process_set_has_healing_potions(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x3b0, value);
}

// Translated from 00900ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetHasHealingPotions` (Xbox PDB): `iHasHealingPotion`
/// (+0x3b0).
pub fn high_process_get_has_healing_potions(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x3b0)
}

// Translated from 00900ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetDefaultHeadTrackTarget` (Xbox PDB): stores `target` as
/// the default head-tracking target (+0x3f8), sets its flag byte (+0x410),
/// runs `OnNewHeadTrackTarget` (`009014d0`) and marks a non-null target as
/// targeted.
pub fn high_process_set_default_head_track_target(e: &mut Engine, this: Ptr, target: u32) {
    let base = this.addr();
    e.mem.set_u32(base + HEAD_TRACKING_TARGETS, target);
    e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS, 1);
    e.call(ON_NEW_HEAD_TRACK_TARGET, &args![this]);
    if target != 0 {
        e.call(SET_TARGETED_REFERENCE, &args![target, 1u32]);
    }
}

// Translated from 00900f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetActionHeadTrackTarget` (Xbox PDB): stores `target` as the
/// action head-tracking target (+0x3fc). A non-null target sets its flag byte
/// (+0x411), runs `OnNewHeadTrackTarget` (`009014d0`) and is marked targeted;
/// a null target clears the flag byte.
pub fn high_process_set_action_head_track_target(e: &mut Engine, this: Ptr, target: u32) {
    let base = this.addr();
    e.mem.set_u32(base + HEAD_TRACKING_TARGETS + 4, target);
    if target != 0 {
        e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS + 1, 1);
        e.call(ON_NEW_HEAD_TRACK_TARGET, &args![this]);
        e.call(SET_TARGETED_REFERENCE, &args![target, 1u32]);
    } else {
        e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS + 1, 0);
    }
}

// ---- Head-track targets (+0x3f8, six words; flags at +0x410, six bytes), dialogue and guard ----
//
// The six head-track slots are, in order: default, action, script, combat, dialog and
// use-weapon (the engine map's setter names). The highest flagged slot is the current
// target.

/// Stores `target` in head-track slot `index`; a non-null target flags the slot,
/// tells the process (`009014d0`) and marks the reference as targeted
/// (`TESObjectREFR::SetTargeted`, `00564db0`), a null target unflags it.
fn set_head_track_slot(e: &mut Engine, this: Ptr, index: u32, target: u32) {
    let base = this.addr();
    e.mem
        .set_u32(base + HEAD_TRACKING_TARGETS + 4 * index, target);
    if target != 0 {
        e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS + index, 1);
        high_process_on_new_head_track_target(e, this);
        e.call(SET_TARGETED_REFERENCE, &args![target, 1u32]);
    } else {
        e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS + index, 0);
    }
}

/// Empties head-track slot `index`; with `restore_default` the old target becomes the
/// default target and the process is told through its slot `0x268` with the float of the
/// game setting at `011cdbb8`, otherwise it is told through `009014d0`.
fn clear_head_track_slot(e: &mut Engine, this: Ptr, index: u32, restore_default: bool) {
    let base = this.addr();
    let old = e.mem.u32(base + HEAD_TRACKING_TARGETS + 4 * index);
    e.mem.set_u32(base + HEAD_TRACKING_TARGETS + 4 * index, 0);
    e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS + index, 0);
    if restore_default {
        e.mem.set_u32(base + HEAD_TRACKING_TARGETS, old);
        let delay = setting_float(e, 0x011c_dbb8);
        e.vcall(base, 0x268, &args![delay]);
    } else {
        high_process_on_new_head_track_target(e, this);
    }
}

// Translated from 00900f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetScriptHeadTrackTarget` (Xbox PDB): slot 2.
pub fn high_process_set_script_head_track_target(e: &mut Engine, this: Ptr, target: u32) {
    set_head_track_slot(e, this, 2, target);
}

// Translated from 00900fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetCombatHeadTrackTarget` (Xbox PDB): slot 3.
pub fn high_process_set_combat_head_track_target(e: &mut Engine, this: Ptr, target: u32) {
    set_head_track_slot(e, this, 3, target);
}

// Translated from 00900ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetHeadTrackTarget` (Xbox PDB): the slot `index` is not range checked.
pub fn high_process_set_head_track_target(e: &mut Engine, this: Ptr, index: u32, target: u32) {
    set_head_track_slot(e, this, index, target);
}

// Translated from 00901050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetDialogHeadTrackTarget` (Xbox PDB): slot 4.
pub fn high_process_set_dialog_head_track_target(e: &mut Engine, this: Ptr, target: u32) {
    set_head_track_slot(e, this, 4, target);
}

// Translated from 009010a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetUseWeaponHeadTrackTarget` (Xbox PDB): slot 5.
pub fn high_process_set_use_weapon_head_track_target(e: &mut Engine, this: Ptr, target: u32) {
    set_head_track_slot(e, this, 5, target);
}

// Translated from 009010f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearDefaultHeadTrackTarget` (Xbox PDB): empties slot 0.
pub fn high_process_clear_default_head_track_target(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + HEAD_TRACKING_TARGETS, 0);
    e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS, 0);
    high_process_on_new_head_track_target(e, this);
}

// Translated from 00901120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearActionHeadTrackTarget` (Xbox PDB): empties slot 1;
/// `restore_default` hands the old target to slot 0.
pub fn high_process_clear_action_head_track_target(e: &mut Engine, this: Ptr, restore_default: u8) {
    clear_head_track_slot(e, this, 1, restore_default != 0);
}

// Translated from 00901190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearScriptHeadTrackTarget` (Xbox PDB): empties slot 2.
pub fn high_process_clear_script_head_track_target(e: &mut Engine, this: Ptr, restore_default: u8) {
    clear_head_track_slot(e, this, 2, restore_default != 0);
}

// Translated from 00901200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearCombatHeadTrackTarget` (Xbox PDB): empties slot 3.
pub fn high_process_clear_combat_head_track_target(e: &mut Engine, this: Ptr) {
    clear_head_track_slot(e, this, 3, false);
}

// Translated from 00901230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearDialogHeadTrackTarget` (Xbox PDB): empties slot 4.
pub fn high_process_clear_dialog_head_track_target(e: &mut Engine, this: Ptr, restore_default: u8) {
    clear_head_track_slot(e, this, 4, restore_default != 0);
}

// Translated from 009012a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearUseWeaponHeadTrackTarget` (Xbox PDB): empties slot 5.
pub fn high_process_clear_use_weapon_head_track_target(
    e: &mut Engine,
    this: Ptr,
    restore_default: u8,
) {
    clear_head_track_slot(e, this, 5, restore_default != 0);
}

// Translated from 00901310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearAllHeadTrackTargets` (Xbox PDB): empties the six slots.
pub fn high_process_clear_all_head_track_targets(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for index in 0..6 {
        e.mem.set_u32(base + HEAD_TRACKING_TARGETS + 4 * index, 0);
        e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS + index, 0);
    }
    high_process_on_new_head_track_target(e, this);
}

// Translated from 00901360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearPathLookTarget` (Xbox PDB): forgets the path look target (+0x2ac)
/// when it is `target`.
pub fn high_process_clear_path_look_target(e: &mut Engine, this: Ptr, target: u32) {
    if e.mem.u32(this.addr() + 0x2ac) == target {
        e.mem.set_u32(this.addr() + 0x2ac, 0);
    }
}

// Translated from 00901390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::RemoveHeadTrackTarget` (Xbox PDB): empties every slot holding `target`.
pub fn high_process_remove_head_track_target(e: &mut Engine, this: Ptr, target: u32) {
    let base = this.addr();
    for index in 0..6 {
        if e.mem.u32(base + HEAD_TRACKING_TARGETS + 4 * index) == target {
            e.mem.set_u32(base + HEAD_TRACKING_TARGETS + 4 * index, 0);
            e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS + index, 0);
        }
    }
    high_process_on_new_head_track_target(e, this);
}

// Translated from 00901400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::CanSetDefaultHeadTrackTarget` (Xbox PDB): false when any of slots 1 to 5
/// is flagged; otherwise true when the process's slot `0x264` (a float) is below the
/// double at `01012060` (zero).
pub fn high_process_can_set_default_head_track_target(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    for index in (1..6).rev() {
        if e.mem.u8(base + HEAD_TRACKING_TARGET_FLAGS + index) != 0 {
            return false;
        }
    }
    let value = e.vcall(base, 0x264, &args![]).f64();
    value < e.global::<f64>(0x0101_2060)
}

// Translated from 00901460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::CanSetActionHeadTrackTarget` (Xbox PDB): true when none of slots 1 to 5
/// is flagged.
pub fn high_process_can_set_action_head_track_target(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    (1..6).all(|index| e.mem.u8(base + HEAD_TRACKING_TARGET_FLAGS + index) == 0)
}

// Translated from 009014a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::CanSetCombatHeadTrackTarget` (Xbox PDB): true when slot 4 (dialog) is
/// not flagged.
pub fn high_process_can_set_combat_head_track_target(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + HEAD_TRACKING_TARGET_FLAGS + 4) == 0
}

// Translated from 009014d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::OnNewHeadTrackTarget` (Xbox PDB): the current target (the highest
/// flagged slot, or none) is cached at +0x41c; a change clears the byte at +0x420.
pub fn high_process_on_new_head_track_target(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut current = 0;
    for index in (0..6).rev() {
        if e.mem.u8(base + HEAD_TRACKING_TARGET_FLAGS + index) != 0 {
            current = e.mem.u32(base + HEAD_TRACKING_TARGETS + 4 * index);
            break;
        }
    }
    if e.mem.u32(base + 0x41c) != current {
        e.mem.set_u8(base + 0x420, 0);
        e.mem.set_u32(base + 0x41c, current);
    }
}

// Translated from 00901550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::UpdateHeadTrackTargets` (Xbox PDB): unflags and empties every flagged slot
/// whose target is null or for which `00440d80` is true; when any was emptied the process
/// is told through `009014d0`.
pub fn high_process_update_head_track_targets(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut changed = false;
    for index in (0..6).rev() {
        if e.mem.u8(base + HEAD_TRACKING_TARGET_FLAGS + index) == 0 {
            continue;
        }
        let target = e.mem.u32(base + HEAD_TRACKING_TARGETS + 4 * index);
        if target == 0 || e.call(0x0044_0d80, &args![target]).bool() {
            e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS + index, 0);
            e.mem.set_u32(base + HEAD_TRACKING_TARGETS + 4 * index, 0);
            changed = true;
        }
    }
    if changed {
        high_process_on_new_head_track_target(e, this);
    }
}

// Translated from 009015f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetCurrentHeadTrackTarget` (Xbox PDB): the target of the highest flagged
/// slot, or null.
pub fn high_process_get_current_head_track_target(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    for index in (0..6).rev() {
        if e.mem.u8(base + HEAD_TRACKING_TARGET_FLAGS + index) != 0 {
            return e.mem.u32(base + HEAD_TRACKING_TARGETS + 4 * index);
        }
    }
    0
}

// Translated from 00901640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetCurrentHeadTrackTarget_ov2` (Xbox PDB): the target stored in slot
/// `index` (not range checked).
pub fn high_process_get_current_head_track_target_ov2(
    e: &mut Engine,
    this: Ptr,
    index: u32,
) -> u32 {
    e.mem.u32(this.addr() + HEAD_TRACKING_TARGETS + 4 * index)
}

// Translated from 00901660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetCurrentHeadTrackType` (Xbox PDB): the index of the highest flagged
/// slot, or 0.
pub fn high_process_get_current_head_track_type(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    for index in (0..6).rev() {
        if e.mem.u8(base + HEAD_TRACKING_TARGET_FLAGS + index) != 0 {
            return index;
        }
    }
    0
}

// Translated from 009016a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetCurrentHeadTrackTypeString` (Xbox PDB): the string for the type the
/// process's slot `0x67c` returns (0 to 4: `0104e6a0`, `010883ec`, `010883e4`, `010883dc`,
/// `010883d4`), `0101a6c0` for any other.
pub fn high_process_get_current_head_track_type_string(e: &mut Engine, this: Ptr) -> u32 {
    match e.vcall(this.addr(), 0x67c, &args![]).u32() {
        0 => 0x0104_e6a0,
        1 => 0x0108_83ec,
        2 => 0x0108_83e4,
        3 => 0x0108_83dc,
        4 => 0x0108_83d4,
        _ => 0x0101_a6c0,
    }
}

// Translated from 00901710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetDetectionModifier` (Xbox PDB): calls the process's slot `0x4a8`
/// (no argument, result unused) and stores `modifier` at +0x3bc.
pub fn high_process_set_detection_modifier(e: &mut Engine, this: Ptr, modifier: f32) {
    e.vcall(this.addr(), 0x4a8, &args![]);
    e.mem.set_f32(this.addr() + 0x3bc, modifier);
}

// Translated from 00901740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the body ignores `this`): `0061a2d0(1, 2)` gives a value `item`;
/// `0057bd60(actor, player)` is called; with an `item` the actor's process (slot `0x2a4`)
/// gets `(actor, item, 0, 1, 0, 0)`. Then, unless the actor's slot `0x214` is nonzero or
/// `009344a0(actor)` has type 6, the actor's process slot `0x1dc` gets `(actor, 1)`.
pub fn fn_00901740(e: &mut Engine, _this: Ptr, actor: u32) {
    let item = e.call(0x0061_a2d0, &args![1u32, 2u32]).u32();
    let player = e.mem.u32(PLAYER);
    e.call(0x0057_bd60, &args![actor, player]);
    if item != 0 {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.vcall(process, 0x2a4, &args![actor, item, 0u32, 1u32, 0u32, 0u32]);
    }
    if e.vcall(actor, 0x214, &args![]).u32() == 0 {
        let form = e.call(0x0093_44a0, &args![actor]).u32();
        if form == 0 || e.call(PACKAGE_TYPE, &args![form]).u32() != 6 {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            e.vcall(process, 0x1dc, &args![actor, 1u32]);
        }
    }
}

// Translated from 00901800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ComputeCanInitiateDialogueWithNPC` (Xbox PDB): whether `actor` may start
/// a dialogue with `other`. False when the player is busy (`00969860`, `008defe0`), the
/// process runs no package (slot `0x27c`), `actor` is not an actor or is dead, the
/// process's wait (+0x2a8) is positive, `actor` is in a conversation (`008a78f0`), `other`
/// is the actor, the player or not a loaded actor, `other` cannot be seen (`00893190`) or
/// its process refuses (slot `0x1f8`). With the global byte `011f1221` clear the answer is
/// then true, otherwise it is whether the detection state's float (`00435e00`) is below the
/// double at `0101db88`.
pub fn high_process_compute_can_initiate_dialogue_with_npc(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    other: u32,
) -> bool {
    let base = this.addr();
    let player = e.mem.u32(PLAYER);
    if e.call(0x0096_9860, &args![player]).bool() || e.call(0x008d_efe0, &args![player]).bool() {
        return false;
    }
    if e.vcall(base, 0x27c, &args![]).u32() == 0 {
        return false;
    }
    if !e.call(0x0088_4480, &args![actor]).bool() || e.call(0x0057_9670, &args![actor]).bool() {
        return false;
    }
    if f64::from(e.mem.f32(base + 0x2a8)) > e.global::<f64>(0x0101_2060) {
        return false;
    }
    if e.call(0x008a_78f0, &args![actor, 1u32]).bool()
        || other == actor
        || other == player
        || !e.call(0x0088_4480, &args![other]).bool()
        || e.vcall(other, 0x1d0, &args![]).u32() == 0
        || !e.call(0x0089_3190, &args![other, actor]).bool()
    {
        return false;
    }
    e.call(0x0088_1510, &args![actor]);
    e.call(0x0088_1510, &args![other]);
    let other_process = e.call(ACTOR_PROCESS, &args![other]).u32();
    if other_process == 0 || !e.vcall(other_process, 0x1f8, &args![other, actor]).bool() {
        return false;
    }
    if e.mem.u8(0x011f_1221) == 0 {
        return true;
    }
    let state = e.vcall(base, 0x504, &args![other, 0u32]).u32();
    if state == 0 {
        return false;
    }
    let value = e.call(0x0043_5e00, &args![state + 0x18]).f64();
    value < e.global::<f64>(0x0101_db88)
}

// Translated from 00901990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ComputeCanRespondToDialogueFromNPC` (Xbox PDB): false when the process's
/// slot `0x52c` is nonzero, its greeting flag (slot `0x30c`) is set, `actor` is in a
/// conversation (`008a78f0`) or the object of its slot `0x22c` fails `00678610`.
pub fn high_process_compute_can_respond_to_dialogue_from_npc(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    _unused_2: u32,
) -> bool {
    let base = this.addr();
    if e.vcall(base, 0x52c, &args![]).u32() != 0
        || e.vcall(base, 0x30c, &args![]).bool()
        || e.call(0x008a_78f0, &args![actor, 1u32]).bool()
    {
        return false;
    }
    if e.vcall(base, 0x22c, &args![]).u32() != 0 {
        let object = e.vcall(base, 0x22c, &args![]).u32();
        if e.call(0x0067_8610, &args![object]).bool() {
            return false;
        }
    }
    true
}

// Translated from 00901a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ProcessSandBoxDialogue` (Xbox PDB): makes `actor` start a dialogue with
/// `other` (an actor, accepted by the process's slot `0x1f4`). The location used is the
/// package's (`0055b980`) unless the process's slot `0x274` is a form of type 12 giving
/// one; the actor's process slot `0x280` runs the dialogue. The exception-unwinding frame
/// is not translated.
pub fn high_process_process_sand_box_dialogue(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    other: u32,
) -> bool {
    let base = this.addr();
    if other == 0 || !e.vcall(other, 0x100, &args![]).bool() {
        return false;
    }
    if !e.vcall(base, 0x1f4, &args![actor, other]).bool() {
        return false;
    }
    let form = e.call(0x0093_44a0, &args![other]).u32();
    let mut flag = 0u32;
    if form != 0
        && e.call(0x0096_11e0, &args![form]).u32() == 0x25
        && fn_00901c50(e, Ptr::new(form))
    {
        flag = 1;
    }
    let package = e.vcall(base, 0x27c, &args![]).u32();
    let mut location = if package != 0 {
        e.call(PACKAGE_LOCATION, &args![package]).u32()
    } else {
        0
    };
    e.with_stack(12, |e, object| {
        e.call(0x0067_f030, &args![object]);
        let source = e.vcall(base, 0x274, &args![]).u32();
        if source != 0 && e.vcall(source, 0x8, &args![]).u32() == 0xc {
            let inner = e.call(0x0044_edb0, &args![source]).u32();
            if inner != 0 {
                e.call(0x0067_f3c0, &args![object, inner]);
                location = object.addr();
            }
        }
        let low = setting_float(e, 0x011c_d038);
        let high = setting_float(e, 0x011c_d328);
        let value = e.call(0x0064_65f0, &args![high, low]).f32();
        e.mem.set_f32(base + 0x2c8, value);
        e.vcall(base, 0x1f0, &args![other]);
        if e.call(ACTOR_PROCESS, &args![other]).u32() != 0 {
            let other_process = e.call(ACTOR_PROCESS, &args![other]).u32();
            e.vcall(other_process, 0x1f0, &args![actor]);
        }
        let result = e
            .vcall(
                actor,
                0x280,
                &args![other, location, location, 1u32, 0u32, 0u32, 0u32, 1u32, flag],
            )
            .bool();
        e.call(0x0067_f110, &args![object]);
        result
    })
}

// Translated from 00901c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: true when bit 2 of the word at +0x24 is clear.
pub fn fn_00901c50(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u16(this.addr() + 0x24) & 4 == 0
}

// Translated from 00901c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ProcessUseIdleMarker` (Xbox PDB): keeps `actor` at the idle marker
/// `marker` (or at the process's target, +0x40). A given marker goes to slot `0x12c`; with
/// neither it asks slot `0x8c` and gives up unless the target then has a form of type
/// 0x30. An actor that is not to move (`00915ef0`) ends its move (slots `0x294`, `0x288`
/// with -1). A pathing actor (`008b3bd0`) is steered through slots `0x34c` / `0x350` when
/// farther than the float at `0101e6ec` from the marker. Otherwise, when far from the
/// target (squared distance above the square of the float at `01017b78`, or height gap
/// above the float at `01016410`) it is told to move (`008b37c0`); near it, an actor not
/// at the target's height is turned (`008bb5c0`); else an idle is chosen from the
/// collection (`0047a030`) and started through slot `0x44`: mode 0 at random, mode 1 in
/// turn (the chosen index +0x190, the idle +0x194, its delay +0x198). The
/// exception-unwinding frame is not translated.
pub fn high_process_process_use_idle_marker(e: &mut Engine, this: Ptr, actor: u32, marker: u32) {
    let base = this.addr();
    let marker_form = e.call(FORM_OF_REFERENCE, &args![marker]).u32();
    let mode = e.call(0x0047_9e80, &args![marker_form + 0x30]).u32();
    if marker != 0 {
        e.vcall(base, 0x12c, &args![marker]);
    } else if e.mem.u32(base + TARGET) == 0 {
        e.vcall(base, 0x8c, &args![actor]);
        let target = e.mem.u32(base + TARGET);
        if target == 0 || e.call(FORM_OF_REFERENCE, &args![target]).u32() == 0 {
            return;
        }
        let form = e.call(FORM_OF_REFERENCE, &args![target]).u32();
        if e.call(0x0040_1170, &args![form]).u32() != 0x30 {
            return;
        }
    }
    if e.call(0x0091_5ef0, &args![base, actor]).bool() {
        e.vcall(base, 0x294, &args![actor]);
        e.vcall(base, 0x288, &args![actor, 0xffff_ffffu32]);
        return;
    }
    if e.call(0x008b_3bd0, &args![actor]).bool() {
        let package = e.vcall(base, 0x27c, &args![]).u32();
        let distance = e
            .call(REFERENCE_GET_DISTANCE, &args![actor, marker, 0u32, 0u32])
            .f32();
        let far = e.global::<f32>(0x0101_e6ec);
        let mut steps = 0x101u32;
        let outside = far < distance;
        if outside {
            let radius = e.call(0x0067_8670, &args![package, actor, 0u32]).f32();
            let double = (f64::from(radius) + f64::from(radius)) as f32;
            steps = e
                .vcall(
                    base,
                    0x34c,
                    &args![actor, distance, radius, double, 0u32, 0u32],
                )
                .u32();
        }
        e.vcall(base, 0x350, &args![actor, steps, outside as u32]);
        return;
    }
    let limit_height = e.global::<f32>(0x0101_6410);
    let limit_distance = e.global::<f32>(0x0101_7b78);
    let limit_squared = (f64::from(limit_distance) * f64::from(limit_distance)) as f32;
    let target = e.mem.u32(base + TARGET);
    let target_position = e.vcall(target, 0x1f4, &args![]).u32();
    let actor_position = e.vcall(actor, 0x1f4, &args![]).u32();
    let (squared, height) = e.with_stack(12, |e, out| {
        e.call(0x0043_9ef0, &args![target_position, out, actor_position]);
        let squared = e.call(0x0059_5c80, &args![out]).f64();
        (squared, e.mem.f32(out.addr() + 8))
    });
    let far_away = if f64::from(limit_squared) < squared {
        true
    } else {
        let gap = e.call(0x0040_8860, &args![height]).f64();
        f64::from(limit_height) < gap
    };
    if far_away {
        let target = e.mem.u32(base + TARGET);
        e.call(0x008b_37c0, &args![actor, target, limit_distance, 0u32]);
        return;
    }
    if e.mem.u32(base + TARGET) != 0
        && !e.call(0x0091_5ef0, &args![base, actor]).bool()
        && e.vcall(actor, 0x214, &args![]).u32() == 0
    {
        let target = e.mem.u32(base + TARGET);
        let target_node = e.call(0x0043_0830, &args![target]).u32();
        let target_height = e.mem.f32(target_node + 8);
        let actor_node = e.call(0x0043_0830, &args![actor]).u32();
        let actor_height = e.mem.f32(actor_node + 8);
        let angle = e.with_stack(4, |e, zero| {
            e.mem.set_f32(zero.addr(), 0.0);
            e.call(0x004b_15e0, &args![actor_height, target_height, zero])
                .f32()
        });
        let wrapped = e.call(0x0040_8840, &args![angle]).f64();
        if e.global::<f64>(0x0101_ffa0) < wrapped {
            e.call(0x008b_b5c0, &args![actor, target_height, 1u32]);
            return;
        }
    }
    e.vcall(base, 0x660, &args![]);
    e.with_stack(16, |e, idles| {
        e.call(0x0090_58e0, &args![idles, 10u32]);
        let target = e.mem.u32(base + TARGET);
        let count = e
            .call(
                0x0047_a030,
                &args![marker_form + 0x30, actor, target, idles],
            )
            .u32();
        if count > 0 {
            let owner = e.vcall(actor, 0x1e4, &args![]).u32();
            if owner != 0
                && (e.mem.u32(base + 0x194) == 0 || e.call(0x0049_85f0, &args![owner]).bool())
                && !e.vcall(actor, 0x234, &args![]).bool()
            {
                e.mem.set_u32(base + 0x194, 0);
                if mode == 0 {
                    let random = e.call(0x00ec_adb8, &args![]).u32();
                    let index = random % count;
                    let slot = e.call(0x0087_7a30, &args![idles, index]).u32();
                    let idle = e.mem.u32(slot);
                    e.mem.set_u32(base + 0x194, idle);
                    e.mem.set_u32(base + 0x190, index);
                    if idle != 0 {
                        let delay = e.global::<f32>(0x0103_0ff0);
                        e.mem.set_f32(base + 0x198, delay);
                    }
                } else if mode == 1 {
                    let last = e.mem.u32(base + 0x190);
                    let next = if last == 0xffff_ffff || last.wrapping_add(1) >= count {
                        0
                    } else {
                        last + 1
                    };
                    e.mem.set_u32(base + 0x190, next);
                    let slot = e.call(0x0087_7a30, &args![idles, next]).u32();
                    let idle = e.mem.u32(slot);
                    e.mem.set_u32(base + 0x194, idle);
                    let delay = e.global::<f32>(0x0101_2054);
                    e.mem.set_f32(base + 0x198, delay);
                }
                let idle = e.mem.u32(base + 0x194);
                if idle != 0 {
                    e.vcall(base, 0x44, &args![actor, idle, 2u32, 1u32, 0u32, 0u32]);
                }
            }
        }
        e.call(0x0090_5910, &args![idles]);
    });
}

// Translated from 00902190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::CreateMuzzleFlash` (Xbox PDB): when the process has none (+0x3d4), a
/// new 0x20-byte object built by `009bacb0(first, second)` (null if the allocation failed).
pub fn high_process_create_muzzle_flash(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let base = this.addr();
    if e.mem.u32(base + 0x3d4) != 0 {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
    let flash = if block != 0 {
        e.call(0x009b_acb0, &args![block, first, second]).u32()
    } else {
        0
    };
    e.mem.set_u32(base + 0x3d4, flash);
}

// Translated from 00902230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearMuzzleFlash` (Xbox PDB): destroys the muzzle flash (+0x3d4) with its
/// deleting destructor (`008d9f70`, flag 1) and forgets it.
pub fn high_process_clear_muzzle_flash(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let flash = e.mem.u32(base + 0x3d4);
    if flash != 0 {
        e.call(0x008d_9f70, &args![flash, 1u32]);
    }
    e.mem.set_u32(base + 0x3d4, 0);
}

// Translated from 00902290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ProcessGuard` (Xbox PDB): one step of the guard package for `actor`. The
/// guard state (`008d0430`; the process's slot `0x270` when slot `0x20c` has a type-14
/// object) selects the phase through its mode (`00726070`): 0 `fn_00902640`, 1
/// `fn_009026c0`, 2 `high_process_process_guard_wait_to_attack`. Without a state the
/// process ends (slot `0x284`, -1). Before the phase the guard reference (+0x128) is
/// brought in line with the package's, an actor of type 4 or 9 without combat state
/// (`0x35c`) is sent to melee idle, and an actor with no path move (`0x4b4`) gets a
/// distance check against the package target (or the package location) and the movement
/// slots `0x34c` / `0x350`.
pub fn high_process_process_guard(e: &mut Engine, this: Ptr, actor: u32) {
    let base = this.addr();
    let mut state = e.call(0x008d_0430, &args![base]).u32();
    if e.vcall(base, 0x20c, &args![]).u32() != 0 {
        let object = e.vcall(base, 0x20c, &args![]).u32();
        if e.call(PACKAGE_TYPE, &args![object]).u32() == 0xe {
            state = e.vcall(base, 0x270, &args![]).u32();
        }
    }
    if state == 0 {
        e.vcall(base, 0x284, &args![0xffff_ffffu32]);
        return;
    }
    e.mem.set_u8(base + 0x349, 0);
    let package = e.vcall(base, 0x27c, &args![]).u32();
    let initial = e.call(0x0067_59e0, &args![package, actor]).u32();
    let mut guard = if initial != 0 { initial } else { actor };
    let list = e.call(EXTRA_DATA_LIST, &args![guard]).u32();
    let reference = e.call(0x0041_c8d0, &args![list]).u32();
    if reference != 0 {
        guard = reference;
    }
    if e.vcall(base, 0x128, &args![]).u32() != guard {
        e.vcall(base, 0x12c, &args![guard]);
        let list = e.call(EXTRA_DATA_LIST, &args![guard]).u32();
        e.call(0x0042_1310, &args![list, actor]);
        e.vcall(guard, 0x48, &args![0x8000_0000u32]);
    }
    if !e.vcall(base, 0x35c, &args![]).bool() {
        let kind = e.vcall(actor, 0x214, &args![]).u32();
        if kind == 4 || e.vcall(actor, 0x214, &args![]).u32() == 9 {
            e.call(0x006e_cd40, &args![state, 0u32]);
            e.vcall(actor, 0x418, &args![]);
            return;
        }
    }
    let mut inside = true;
    if !e.vcall(actor, 0x4b4, &args![]).bool() {
        let radius;
        if e.call(PACKAGE_LOCATION, &args![package]).u32() != 0 {
            let minus_one = e.global::<f32>(0x0101_2054);
            inside = e
                .vcall(package, 0x13c, &args![actor, 0u32, minus_one, 0u32])
                .bool();
            radius = e.call(0x0067_6280, &args![package, actor]).u32() as f32;
        } else {
            let target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
            radius = if target != 0 {
                e.call(0x0044_ddc0, &args![target]).i32() as f32
            } else {
                e.global::<f32>(0x0101_8f5c)
            };
            let distance = e
                .call(REFERENCE_GET_DISTANCE, &args![actor, guard, 0u32, 0u32])
                .f64();
            inside = f64::from(radius) > distance;
        }
        if e.call(LIST_NEXT, &args![state]).u32() != 0
            && ((e.call(0x008b_3bb0, &args![actor]).bool() && !inside)
                || e.call(0x008b_3b90, &args![actor]).bool())
        {
            e.vcall(base, 0x7f8, &args![actor, state]);
        }
        let owner = e.mem.u32(actor + 0x190);
        let remaining = if !inside && owner != 0 {
            e.call(0x009d_cc50, &args![owner]).f32()
        } else {
            0.0
        };
        let double = (f64::from(radius) + f64::from(radius)) as f32;
        let steps = e
            .vcall(
                base,
                0x34c,
                &args![actor, remaining, radius, double, 0u32, 0u32],
            )
            .u32();
        e.call(0x0067_8670, &args![package, actor, 0u32]);
        e.vcall(base, 0x350, &args![actor, steps, 1u32]);
    }
    match e.call(LIST_NEXT, &args![state]).u32() {
        0 => fn_00902640(e, this, actor, state),
        1 => fn_009026c0(e, this, actor, state, inside as u8),
        2 => high_process_process_guard_wait_to_attack(e, this, actor, state),
        _ => {}
    }
}

// Translated from 00902640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the process's slot `0x7f8` gets `(actor, state)`, then the state's counter
/// (+0xc) and timer (+0x10) are reset.
pub fn fn_00902640(e: &mut Engine, this: Ptr, actor: u32, state: u32) {
    e.vcall(this.addr(), 0x7f8, &args![actor, state]);
    fn_009026a0(e, Ptr::new(state));
    fn_00902680(e, Ptr::new(state));
}

// Translated from 00902680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: clears the guard state's timer (the float at +0x10).
pub fn fn_00902680(e: &mut Engine, this: Ptr) {
    e.mem.set_f32(this.addr() + 0x10, 0.0);
}

// Translated from 009026a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: clears the guard state's counter (the word at +0xc).
pub fn fn_009026a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0xc, 0);
}

// Translated from 009026c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the guard phase that looks for an intruder. `inside` says whether the actor
/// is within the guard area (computed by the caller). The package's guard radius
/// (`00671d10` then `0044ddc0`) is squared and the list at +0x25c is searched, when the
/// package allows it (`0067a7e0`), for the first detected actor that is not the guard
/// reference, has a process with `0045cd60` false, is not dead or otherwise excluded
/// (slots `0x22c`, `0x2e8`, `0x230`, `0x234`, `00437bd0`), has a positive count (+8), is
/// closer to the guard reference than the radius, is not owned by the reference's owner
/// and that the actor should attack (`008b06d0`). Found: when `inside` the actor stops
/// moving (slot `0x294`) and turns to it (`008bb520`), the intruder is recorded
/// (`009f2970`), the actor is warned (`high_process_process_guard_warn`) and the state
/// moves to 2. None: head-track slots `0x644` / `0x654` are cleared, the look-at target is
/// cleared (`008b3d30`), and, when `inside`, the actor is walked (slot `0x84c`) back
/// towards the package location (or the guard reference when the radius is positive), else
/// turned to face the package location's or the process's reference (+0x44) height
/// (`008bb5c0`) when it differs by more than the double at `01023128`.
pub fn fn_009026c0(e: &mut Engine, this: Ptr, actor: u32, state: u32, inside: u8) {
    let base = this.addr();
    let package = e.vcall(base, 0x27c, &args![]).u32();
    let has_location = e.call(PACKAGE_LOCATION, &args![package]).u32() != 0;
    let guard = e.vcall(base, 0x128, &args![]).u32();
    // The game stores the result of this test and never reads it; the calls are kept.
    if has_location {
        let minus_one = e.global::<f32>(0x0101_2054);
        e.vcall(package, 0x13c, &args![actor, 0u32, minus_one, 0u32]);
    } else {
        e.call(REFERENCE_GET_DISTANCE, &args![actor, guard, 0u32, 0u32]);
        let target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
        e.call(0x0044_ddc0, &args![target]);
    }
    let package_now = e.vcall(base, 0x27c, &args![]).u32();
    let target = e.call(PACKAGE_GET_TARGET, &args![package_now]).u32();
    let radius = e.call(0x0044_ddc0, &args![target]).i32() as f32;
    let radius_squared = (f64::from(radius) * f64::from(radius)) as f32;
    let mut found = 0u32;
    let mut node = e.mem.u32(base + DETECTED_ACTOR_LIST);
    let package_now = e.vcall(base, 0x27c, &args![]).u32();
    if !e.call(0x0067_a7e0, &args![package_now]).bool() {
        node = 0;
    }
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let entry = list_item(e, node);
        node = list_next(e, node);
        let candidate = e.mem.u32(entry);
        if candidate == 0 || candidate == guard {
            continue;
        }
        if e.call(ACTOR_PROCESS, &args![candidate]).u32() == 0 {
            continue;
        }
        let process = e.call(ACTOR_PROCESS, &args![candidate]).u32();
        if e.call(0x0045_cd60, &args![process]).u32() != 0 {
            continue;
        }
        if e.vcall(candidate, 0x22c, &args![0u32]).bool()
            || e.vcall(candidate, 0x2e8, &args![]).bool()
            || e.vcall(candidate, 0x230, &args![]).bool()
            || e.vcall(candidate, 0x234, &args![]).bool()
            || e.call(REFERENCE_TEST_00437BD0, &args![candidate]).bool()
        {
            continue;
        }
        if (e.mem.u32(entry + 8) as i32) < 1 {
            continue;
        }
        let guard_now = e.vcall(base, 0x128, &args![]).u32();
        let guard_position = e.vcall(guard_now, 0x1f4, &args![]).u32();
        let candidate_position = e.vcall(candidate, 0x1f4, &args![]).u32();
        let length = e.with_stack(12, |e, out| {
            let result = e
                .call(0x0043_9ef0, &args![candidate_position, out, guard_position])
                .u32();
            e.call(0x004a_7290, &args![result]).f32()
        });
        if radius_squared <= length {
            continue;
        }
        if guard != 0 {
            if e.call(0x0056_7790, &args![guard]).u32() != 0
                && e.call(0x0057_85e0, &args![guard, candidate, 1u32]).bool()
            {
                continue;
            }
            let attack = e.with_stack(4, |e, flag| {
                e.mem.set_u32(flag.addr(), 0);
                e.call(0x008b_06d0, &args![actor, candidate, 1u32, flag, 0u32])
                    .bool()
            });
            if !attack {
                continue;
            }
        }
        found = candidate;
        break;
    }
    if found != 0 {
        if inside != 0 {
            e.vcall(base, 0x294, &args![actor]);
            // 009029c1: the reference's +0x1f4 takes no stack word (plain RET); the
            // pushed 0 is the last word of `008bb520` (RET 0x10), after the position.
            let position = e.vcall(found, 0x1f4, &args![]).u32();
            let (x, y, z) = (
                e.mem.u32(position),
                e.mem.u32(position + 4),
                e.mem.u32(position + 8),
            );
            e.call(0x008b_b520, &args![actor, x, y, z, 0u32]);
        }
        e.call(0x009f_2970, &args![state, found]);
        high_process_process_guard_warn(e, this, actor, state);
        e.call(0x006e_cd40, &args![state, 2u32]);
        return;
    }
    e.vcall(base, 0x644, &args![0u32]);
    e.vcall(base, 0x654, &args![0u32]);
    e.call(0x008b_3d30, &args![actor]);
    if inside == 0 {
        return;
    }
    let mut handled = false;
    if has_location {
        let package_now = e.vcall(base, 0x27c, &args![]).u32();
        if e.call(0x0067_6280, &args![package_now, actor]).u32() > 0 {
            let minus_one = e.global::<f32>(0x0101_2054);
            e.vcall(base, 0x84c, &args![actor, 0u32, minus_one]);
            handled = true;
        }
    } else if f64::from(radius_squared) > e.global::<f64>(0x0101_2060) {
        let root = e.call(0x0040_19d0, &args![radius_squared]).f32();
        let position = e.vcall(guard, 0x1f4, &args![]).u32();
        e.vcall(base, 0x84c, &args![actor, position, root]);
        handled = true;
    }
    if handled {
        return;
    }
    let mut location = 0u32;
    let location_object = e.call(PACKAGE_LOCATION, &args![package]).u32();
    if location_object != 0 {
        location = e.call(0x0067_f390, &args![location_object]).u32();
    }
    let override_reference = e.mem.u32(base + 0x44);
    if override_reference != 0 {
        location = override_reference;
    }
    if e.call(0x008b_3bd0, &args![actor]).bool() || e.vcall(actor, 0x214, &args![]).u32() != 0 {
        return;
    }
    let usable = (location != 0
        && e.call(FORM_OF_REFERENCE, &args![location]).u32() == e.mem.u32(0x011c_a248))
        || (location != 0
            && location_object != 0
            && e.call(0x0067_8ca0, &args![location_object]).u32() == 3);
    if !usable {
        return;
    }
    let location_node = e.call(0x0043_0830, &args![location]).u32();
    let location_height = e.mem.f32(location_node + 8);
    let actor_node = e.call(0x0043_0830, &args![actor]).u32();
    let difference = (f64::from(location_height) - f64::from(e.mem.f32(actor_node + 8))) as f32;
    let wrapped = e.call(WRAP_ANGLE, &args![difference]).f64();
    if e.global::<f64>(0x0102_3128) < wrapped {
        e.call(
            TURN_ACTOR_SET_ROTATION,
            &args![actor, location_height, 1u32],
        );
    }
}

// Translated from 00902bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ProcessGuardWarn` (Xbox PDB): reads the state's target (`0044ddc0`) and
/// the guard reference's `00567790` (slot `0x128`; neither result is used), draws a value
/// with `0061a2d0(2, 0x14)`, counts the warning (`fn_00902c60`), clears the timer and hands
/// the actor's process (slot `0x2a4`) `(actor, value, 0, 0, 1, 0)`.
pub fn high_process_process_guard_warn(e: &mut Engine, this: Ptr, actor: u32, state: u32) {
    let base = this.addr();
    e.call(0x0044_ddc0, &args![state]);
    let guard = e.vcall(base, 0x128, &args![]).u32();
    if guard != 0 {
        e.call(0x0056_7790, &args![guard]);
    }
    let value = e.call(0x0061_a2d0, &args![2u32, 0x14u32]).u32();
    fn_00902c60(e, Ptr::new(state));
    fn_00902680(e, Ptr::new(state));
    e.vcall(base, 0x2a4, &args![actor, value, 0u32, 0u32, 1u32, 0u32]);
}

// Translated from 00902c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: adds one to the guard state's counter (the word at +0xc).
pub fn fn_00902c60(e: &mut Engine, this: Ptr) {
    let counter = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(this.addr() + 0xc, counter.wrapping_add(1));
}

// Translated from 00902c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ProcessGuardWaitToAttack` (Xbox PDB): the guard phase in which the actor
/// waits to attack the state's target (`0044ddc0`). With the process flag +0x368 the actor
/// (if it has no path move, slot `0x4b4`) is told to go to the target (slot `0x2a4`). With
/// no guard reference (slot `0x128`) or a target that is gone or dead (slots `0x22c`,
/// `0x2e8`, `0x230`, `0x234`, `00437bd0`) the state returns to 0. Otherwise the target is
/// head-tracked (slot `0x628`) and, within the squared package radius (`00671d10` /
/// `0044ddc0`), aimed at (`high_process_aim_at_target`) and the weapon / animation slots
/// are set; beyond it the actor stands down (state 1, slots `0x400`, `0x654`). Within a
/// closer radius (the game setting at `011cd1f8` times the radius) the target is attacked
/// (slot `0x33c`); otherwise, when the state's time (`00621b00`) has reached the setting
/// at `011cd5fc`, the warning count (`0084e3a0`) against the package's limit decides
/// between attacking and warning. The exception-unwinding frame is not translated.
pub fn high_process_process_guard_wait_to_attack(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    state: u32,
) {
    let base = this.addr();
    let mut flagged = false;
    let target = e.call(0x0044_ddc0, &args![state]).u32();
    if e.mem.u32(base + 0x368) != 0 {
        flagged = true;
        if e.vcall(actor, 0x4b4, &args![]).bool() && target != 0 {
            e.vcall(base, 0x2a4, &args![target, 0u32, 0u32, 0u32, 0u32, 0u32]);
        }
    }
    let guard = e.vcall(base, 0x128, &args![]).u32();
    if guard == 0 {
        e.call(0x006e_cd40, &args![state, 0u32]);
        fn_00902640(e, this, actor, state);
        return;
    }
    if target == 0
        || e.vcall(target, 0x22c, &args![0u32]).bool()
        || e.vcall(target, 0x2e8, &args![]).bool()
        || e.vcall(target, 0x230, &args![]).bool()
        || e.vcall(target, 0x234, &args![]).bool()
        || e.call(REFERENCE_TEST_00437BD0, &args![target]).bool()
    {
        e.call(0x006e_cd40, &args![state, 0u32]);
        fn_00902640(e, this, actor, state);
        return;
    }
    let package = e.vcall(base, 0x27c, &args![]).u32();
    let guard_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    let radius = e.call(0x0044_ddc0, &args![guard_target]).i32() as f32;
    let scale = setting_float(e, 0x011c_d1f8);
    let near = (f64::from(radius) * f64::from(scale)) as f32;
    let radius_squared = (f64::from(radius) * f64::from(radius)) as f32;
    let near_squared = (f64::from(near) * f64::from(near)) as f32;
    let guard_position = e.vcall(guard, 0x1f4, &args![]).u32();
    let target_position = e.vcall(target, 0x1f4, &args![]).u32();
    let length = e.with_stack(12, |e, out| {
        let result = e
            .call(0x0043_9ef0, &args![target_position, out, guard_position])
            .u32();
        e.call(0x004a_7290, &args![result]).f32()
    });
    let in_range = length < radius_squared;
    let inner = in_range && length < near_squared;
    let chosen = if in_range { target } else { guard };
    e.vcall(base, 0x628, &args![chosen]);
    if in_range {
        high_process_aim_at_target(e, this, Ptr::new(actor), Ptr::new(target), 0);
    } else {
        e.call(0x008b_3d30, &args![actor]);
    }
    if !in_range {
        e.vcall(base, 0x400, &args![0u32]);
        let package = e.vcall(base, 0x27c, &args![]).u32();
        if package == 0 || !e.call(0x0067_a460, &args![package]).bool() {
            e.call(0x008a_6840, &args![actor, 0u32]);
        }
        e.call(0x006e_cd40, &args![state, 1u32]);
        fn_00902680(e, Ptr::new(state));
        fn_009026a0(e, Ptr::new(state));
        e.vcall(base, 0x654, &args![1u32]);
        return;
    }
    let animation = e.vcall(base, 0x1b8, &args![]).u32();
    let stand = setting_float(e, 0x011c_dd30);
    e.vcall(base, 0x338, &args![stand]);
    if !e.call(0x0049_85f0, &args![animation]).bool() {
        e.call(0x0049_8910, &args![animation, 1u32, 0u32]);
    }
    e.mem.set_u8(base + 0x349, 1);
    if !e.call(0x008a_16d0, &args![actor]).bool() {
        e.call(0x008a_6840, &args![actor, 1u32]);
    }
    if e.call(GET_CURRENT_WEAPON, &args![actor]).u32() != 0
        && e.call(0x008a_16d0, &args![actor]).bool()
    {
        let weapon = e.call(GET_CURRENT_WEAPON, &args![actor]).u32();
        if e.call(0x004c_0c60, &args![weapon]).bool() {
            e.vcall(base, 0x400, &args![1u32]);
        }
    }
    e.vcall(base, 0x638, &args![chosen]);
    if flagged {
        return;
    }
    let seconds = e.call(0x0084_d030, &args![0x011f_6394u32]).f32();
    e.call(0x0090_3160, &args![state, seconds]);
    let mut attack = inner;
    let mut warn = false;
    if !inner {
        let elapsed = e.call(0x0062_1b00, &args![state]).f64();
        if f64::from(setting_float(e, 0x011c_d5fc)) <= elapsed {
            let package = e.vcall(base, 0x27c, &args![]).u32();
            let warnings = if package != 0 {
                e.call(0x0067_5980, &args![package]).u32()
            } else {
                0
            };
            let limit = if warnings != 0 {
                e.call(0x0044_ddc0, &args![warnings]).i32()
            } else {
                2
            };
            let count = e.call(GET_FORM_ID, &args![state]).i32();
            attack = count > limit;
            warn = !attack;
        }
    }
    if attack {
        let weapon_state = e.call(0x0044_ddc0, &args![state]).u32();
        let engaged = e
            .vcall(
                base,
                0x33c,
                &args![
                    actor,
                    weapon_state,
                    0u32,
                    0u32,
                    0u32,
                    0u32,
                    0u32,
                    0u32,
                    0u32,
                    0u32,
                    0u32,
                    1u32,
                    0u32
                ],
            )
            .bool();
        if engaged && e.call(LIST_NEXT, &args![base + 4]).u32() == state {
            e.call(0x006e_cd40, &args![state, 0u32]);
        }
    } else if warn {
        high_process_process_guard_warn(e, this, actor, state);
    }
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008f3fe0, fn_008f3fe0(Ptr, u32, u32) -> bool),
        entry!(0x008f4000, fn_008f4000(Ptr, u32, u32) -> bool),
        entry!(0x008f5410, fn_008f5410(Ptr, u32) -> bool),
        entry!(0x008f6120, fn_008f6120(Ptr) -> Ptr),
        entry!(0x008f6180, high_process_get_who_detects_me(Ptr, u32) -> u32),
        entry!(0x008f62b0, high_process_insert_into_detection_list(Ptr, Ptr, u32, u8, i32, i32, u8, u8) -> Ptr<DetectionState>),
        entry!(0x008f63e0, fn_008f63e0(Ptr, Ptr)),
        entry!(0x008f6630, fn_008f6630(Ptr, u8)),
        entry!(0x008f6650, high_process_get_detection_state(Ptr, u32, i32) -> Ptr<DetectionState>),
        entry!(
            0x008f66e0,
            high_process_remove_detection_actor(Ptr, u32, i32)
        ),
        entry!(0x008f6820, high_process_get_last_position_detected(Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x008f68a0, high_process_get_last_time_detected(Ptr, u32, u32) -> f32),
        entry!(0x008f68f0, high_process_get_360_line_sighton_actor(Ptr, u32, u32, u32) -> u8),
        entry!(0x008f6930, high_process_get_line_sighton_actor(Ptr, u32, u32, u32, u8) -> u8),
        entry!(0x008f6990, high_process_get_detection_actor(Ptr, u32, u32) -> i32),
        entry!(0x008f69e0, high_process_update_3d_model(Ptr, Ptr)),
        entry!(0x008f6ea0, high_process_set_last_seen_location(Ptr)),
        entry!(0x008f6ef0, high_process_get_emotions_dispostion(Ptr, u32, u32) -> u32),
        entry!(0x008f6fc0, high_process_is_talking(Ptr, u32) -> bool),
        entry!(
            0x008f7070,
            high_process_create_followno_escort(Ptr, Ptr, u8)
        ),
        entry!(0x008f7350, high_process_finish_dying(Ptr, Ptr)),
        entry!(0x008f74c0, high_process_aim_at_target(Ptr, Ptr, Ptr, u8) -> bool),
        entry!(0x008f7710, middle_high_process_get_alpha_mult(Ptr) -> f32),
        entry!(0x008f9100, fn_008f9100(Ptr) -> u8),
        entry!(0x008f9120, fn_008f9120(Ptr) -> u8),
        entry!(0x008f9140, fn_008f9140(Ptr) -> u16),
        entry!(0x008f9160, fn_008f9160(Ptr) -> u16),
        entry!(0x008f9320, high_process_process_use_item_at(Ptr, Ptr) -> bool),
        entry!(0x008f7730, high_process_process_use_weapon(Ptr, Ptr) -> bool),
        entry!(
            0x008f9180,
            high_process_clean_up_after_process_use_weapon(Ptr, Ptr)
        ),
        entry!(
            0x008fdb10,
            fn_008fdb10(Ptr<DetectionEvent>) -> Ptr<DetectionEvent>
        ),
        entry!(0x008f5480, high_process_evaluate_detection(Ptr, Ptr) -> bool),
        entry!(0x008f4020, high_process_update_ov2(Ptr, Ptr, f32)),
        entry!(0x008fa630, high_process_get_save_size(Ptr, u32, u32) -> u16),
        entry!(0x008faa50, high_process_save_game(Ptr, u32, u32)),
        entry!(0x008fb330, high_process_load_game(Ptr, u32, u32, Ptr)),
        entry!(0x008fbfd0, high_process_init_load_game(Ptr, u32, u32, u32)),
        entry!(0x008fc210, high_process_revert(Ptr, u32, Ptr)),
        entry!(0x008fc4f0, high_process_save_game_ov2(Ptr, u32)),
        entry!(0x008fce90, high_process_load_game_ov2(Ptr, u32)),
        entry!(0x008fdb90, fn_008fdb90(Ptr) -> Ptr),
        entry!(0x008fdbd0, high_process_init_load_game_ov2(Ptr, u32)),
        entry!(0x008fe420, high_process_revert_ov2(Ptr, u32)),
        entry!(0x008fe8f0, high_process_fade_in(Ptr, u32, u8)),
        entry!(0x008fe960, high_process_fade_out(Ptr, u32, u32, u8)),
        entry!(
            0x008fe9e0,
            high_process_fade_out_and_move(Ptr, u32, u32, f32, f32, f32)
        ),
        entry!(0x008feab0, high_process_fade_and_delete(Ptr, u32)),
        entry!(0x008feb60, high_process_fade_and_disable(Ptr, u32)),
        entry!(0x008fec10, high_process_fade_update(Ptr, u32) -> bool),
        entry!(0x008ff030, high_process_skip_fade_in(Ptr, u32)),
        entry!(0x008ff080, fn_008ff080(Ptr) -> bool),
        entry!(0x008ff0b0, high_process_find_special_idleto_play(Ptr, u32, u32, u32) -> bool),
        entry!(0x008ff1b0, high_process_stop_sound_handle(Ptr, u32)),
        entry!(0x008ff290, high_process_start_torch_sound(Ptr, u32)),
        entry!(
            0x008ff350,
            fn_008ff350(Ptr, u32, u32, u32, u32, u32, f32, f32)
        ),
        entry!(0x008ffc30, high_process_check_target_combat_detection_state(Ptr, u32, u32) -> bool),
        entry!(0x008ffd10, high_process_should_run_combat_detection(Ptr, u32, u32) -> bool),
        entry!(0x008ffdc0, high_process_should_run_combat_detection_event_check(Ptr, u32, u32) -> bool),
        entry!(0x008ffe10, high_process_should_run_player_detection(Ptr, u32) -> bool),
        entry!(0x008ffe50, high_process_react_to_combat_situation(Ptr, u32, u32) -> bool),
        entry!(0x00900000, fn_00900000(Ptr, u32, u32, u32) -> bool),
        entry!(
            0x00900020,
            high_process_process_summon_creature_defend(Ptr, u32)
        ),
        entry!(0x009001c0, high_process_enter_combat(Ptr, u32, u32, u8, u8, u32, u8, u8, u8, u8, u8, u8, u8, u32) -> bool),
        entry!(0x009002b0, high_process_get_actor_value(Ptr, u32, u32, u32) -> i32),
        entry!(0x009002f0, high_process_get_actor_float_value(Ptr, u32, u32, u32) -> f32),
        entry!(0x009003e0, fn_009003e0(Ptr, u32) -> f32),
        entry!(0x00900400, fn_00900400(Ptr, u32) -> u8),
        entry!(
            0x00900420,
            high_process_temp_mod_actor_value(Ptr, u32, u32, u32)
        ),
        entry!(
            0x009004f0,
            high_process_temp_mod_actor_value_ov2(Ptr, u32, u32, f32)
        ),
        entry!(
            0x009005d0,
            high_process_damage_mod_actor_value(Ptr, u32, u32, u32)
        ),
        entry!(
            0x009006a0,
            high_process_damage_mod_actor_value_ov2(Ptr, u32, u32, f32)
        ),
        entry!(
            0x00900780,
            high_process_set_cached_actor_value_out_of_date(Ptr, u32)
        ),
        entry!(0x00900830, high_process_update_followers(Ptr, u32, f32)),
        entry!(0x00900c20, high_process_get_leveled_spells(Ptr, u32, u8) -> Ptr),
        entry!(0x00900e40, high_process_set_has_healing_spells(Ptr, u32)),
        entry!(0x00900e60, high_process_get_has_healing_spells(Ptr) -> u32),
        entry!(0x00900e80, high_process_set_has_healing_potions(Ptr, u32)),
        entry!(0x00900ea0, high_process_get_has_healing_potions(Ptr) -> u32),
        entry!(
            0x00900ec0,
            high_process_set_default_head_track_target(Ptr, u32)
        ),
        entry!(
            0x00900f00,
            high_process_set_action_head_track_target(Ptr, u32)
        ),
        entry!(
            0x00900f50,
            high_process_set_script_head_track_target(Ptr, u32)
        ),
        entry!(
            0x00900fa0,
            high_process_set_combat_head_track_target(Ptr, u32)
        ),
        entry!(
            0x00900ff0,
            high_process_set_head_track_target(Ptr, u32, u32)
        ),
        entry!(
            0x00901050,
            high_process_set_dialog_head_track_target(Ptr, u32)
        ),
        entry!(
            0x009010a0,
            high_process_set_use_weapon_head_track_target(Ptr, u32)
        ),
        entry!(
            0x009010f0,
            high_process_clear_default_head_track_target(Ptr)
        ),
        entry!(
            0x00901120,
            high_process_clear_action_head_track_target(Ptr, u8)
        ),
        entry!(
            0x00901190,
            high_process_clear_script_head_track_target(Ptr, u8)
        ),
        entry!(0x00901200, high_process_clear_combat_head_track_target(Ptr)),
        entry!(
            0x00901230,
            high_process_clear_dialog_head_track_target(Ptr, u8)
        ),
        entry!(
            0x009012a0,
            high_process_clear_use_weapon_head_track_target(Ptr, u8)
        ),
        entry!(0x00901310, high_process_clear_all_head_track_targets(Ptr)),
        entry!(0x00901360, high_process_clear_path_look_target(Ptr, u32)),
        entry!(0x00901390, high_process_remove_head_track_target(Ptr, u32)),
        entry!(0x00901400, high_process_can_set_default_head_track_target(Ptr) -> bool),
        entry!(0x00901460, high_process_can_set_action_head_track_target(Ptr) -> bool),
        entry!(0x009014a0, high_process_can_set_combat_head_track_target(Ptr) -> bool),
        entry!(0x009014d0, high_process_on_new_head_track_target(Ptr)),
        entry!(0x00901550, high_process_update_head_track_targets(Ptr)),
        entry!(0x009015f0, high_process_get_current_head_track_target(Ptr) -> u32),
        entry!(0x00901640, high_process_get_current_head_track_target_ov2(Ptr, u32) -> u32),
        entry!(0x00901660, high_process_get_current_head_track_type(Ptr) -> u32),
        entry!(0x009016a0, high_process_get_current_head_track_type_string(Ptr) -> u32),
        entry!(0x00901710, high_process_set_detection_modifier(Ptr, f32)),
        entry!(0x00901740, fn_00901740(Ptr, u32)),
        entry!(0x00901800, high_process_compute_can_initiate_dialogue_with_npc(Ptr, u32, u32) -> bool),
        entry!(0x00901990, high_process_compute_can_respond_to_dialogue_from_npc(Ptr, u32, u32) -> bool),
        entry!(0x00901a20, high_process_process_sand_box_dialogue(Ptr, u32, u32) -> bool),
        entry!(0x00901c50, fn_00901c50(Ptr) -> bool),
        entry!(
            0x00901c70,
            high_process_process_use_idle_marker(Ptr, u32, u32)
        ),
        entry!(0x00902190, high_process_create_muzzle_flash(Ptr, u32, u32)),
        entry!(0x00902230, high_process_clear_muzzle_flash(Ptr)),
        entry!(0x00902290, high_process_process_guard(Ptr, u32)),
        entry!(0x00902640, fn_00902640(Ptr, u32, u32)),
        entry!(0x00902680, fn_00902680(Ptr)),
        entry!(0x009026a0, fn_009026a0(Ptr)),
        entry!(0x009026c0, fn_009026c0(Ptr, u32, u32, u8)),
        entry!(0x00902bd0, high_process_process_guard_warn(Ptr, u32, u32)),
        entry!(0x00902c60, fn_00902c60(Ptr)),
        entry!(
            0x00902c80,
            high_process_process_guard_wait_to_attack(Ptr, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// Registers a double that returns `eax`.
    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| ret(eax));
    }

    /// Registers a double that returns the float `value` in ST0.
    fn stub_float(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| ret_float(value));
    }

    /// The argument lists of the logged calls to `addr`.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .expect("the call log is on")
            .iter()
            .filter(|(called, _)| *called == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Maps `addr` and stores a word there.
    fn global(e: &mut Engine, addr: u32, value: u32) {
        e.map(addr, 4);
        e.mem.set_u32(addr, value);
    }

    /// A vtable with the given (slot offset, target) entries.
    fn vtable(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let size = slots.iter().map(|slot| slot.0).max().unwrap_or(0) + 4;
        let table = e.mem.alloc(size);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        table
    }

    /// A zeroed object of `size` bytes whose first word is `vtable`.
    fn object(e: &mut Engine, size: u32, vtable: u32) -> u32 {
        let block = e.mem.alloc(size);
        e.mem.set_u32(block, vtable);
        block
    }

    /// A process-sized object (0x500 bytes) with the given vtable slots.
    fn process(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = vtable(e, slots);
        object(e, 0x500, table)
    }

    /// Doubles for the list helpers and the allocator that the detection
    /// code uses: the node itself (`006815c0`), the next pointer
    /// (`00726070`), append (`005ae3d0`), `operator new` / `delete`, the
    /// `DetectionState` and list constructors, head removal, item removal and
    /// clear.
    fn list_doubles(e: &mut Engine) {
        e.register_double(0x0068_15c0, |_, a| ret(a[0]));
        e.register_double(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register_double(0x005a_e3d0, |e, a| {
            let list = a[0];
            let item = e.mem.u32(a[1]);
            if e.mem.u32(list) == 0 {
                e.mem.set_u32(list, item);
            } else {
                let mut last = list;
                while e.mem.u32(last + 4) != 0 {
                    last = e.mem.u32(last + 4);
                }
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, item);
                e.mem.set_u32(last + 4, node);
            }
            ret(0)
        });
        e.register_double(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register_double(0x0040_1030, |_, _| ret(0));
        e.register_double(0x0066_e590, |e, a| {
            e.mem.set_u8(a[0] + 0x1f, 1);
            ret(a[0])
        });
        e.register_double(0x0096_a2d0, |_, a| ret(a[0]));
        e.register_double(0x0063_f7b0, |e, a| {
            let node = e.mem.u32(a[0] + 4);
            if node != 0 {
                let item = e.mem.u32(node);
                let next = e.mem.u32(node + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, next);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            ret(0)
        });
        e.register_double(0x0090_5330, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut previous = 0;
            let mut node = a[0];
            while node != 0 && e.mem.u32(node) != item {
                previous = node;
                node = e.mem.u32(node + 4);
            }
            if node != 0 {
                if previous == 0 {
                    let next = e.mem.u32(node + 4);
                    if next != 0 {
                        let next_item = e.mem.u32(next);
                        let after = e.mem.u32(next + 4);
                        e.mem.set_u32(node, next_item);
                        e.mem.set_u32(node + 4, after);
                    } else {
                        e.mem.set_u32(node, 0);
                    }
                } else {
                    let next = e.mem.u32(node + 4);
                    e.mem.set_u32(previous + 4, next);
                }
            }
            ret(0)
        });
        e.register_double(0x0047_0470, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            ret(0)
        });
        e.register_double(0x0082_56d0, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
    }

    /// A chain of list nodes `[item, next]` holding `items`; the first node.
    fn make_list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        if next == 0 {
            next = e.mem.alloc(8);
        }
        next
    }

    /// The items of a node chain.
    fn list_items(e: &Engine, list: u32) -> Vec<u32> {
        let mut items = Vec::new();
        let mut node = list;
        while node != 0 && e.mem.u32(node) != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    /// A `DetectionState` (0x24 bytes) for `actor`.
    fn detection_state_for(e: &mut Engine, actor: u32, level: i32) -> u32 {
        let state = e.mem.alloc(0x24);
        e.mem.set_u32(state, actor);
        e.mem.set_i32(state + 8, level);
        state
    }

    #[test]
    fn test_fn_008f3fe0() {
        let mut e = Engine::new();
        stub(&mut e, 0x0090_8f70, 0);
        e.call_log = Some(vec![]);
        let r = e.call(0x008f_3fe0, &args![0x1000u32, 0x2000u32, 0u32]);
        assert!(r.bool());
        assert_eq!(calls_to(&e, 0x0090_8f70), vec![vec![0x1000, 0x2000]]);
    }

    #[test]
    fn test_fn_008f4000() {
        let mut e = Engine::new();
        stub(&mut e, 0x0091_21e0, 0);
        e.call_log = Some(vec![]);
        let r = e.call(0x008f_4000, &args![0x1000u32, 0x2000u32, 0u32]);
        assert!(r.bool());
        assert_eq!(calls_to(&e, 0x0091_21e0), vec![vec![0x1000, 0x2000]]);
    }

    #[test]
    fn test_fn_008f5410() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let this = e.mem.alloc(0x500);
        let first = e.mem.alloc(0x14);
        let second = e.mem.alloc(0x14);
        e.mem.set_u32(first + 4, 0x11);
        e.mem.set_u32(second + 4, 0x22);
        // The head node is inline at +0x27c.
        e.mem.set_u32(this + 0x27c, first);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, second);
        e.mem.set_u32(this + 0x280, node);
        assert!(e.call(0x008f_5410, &args![this, 0x22u32]).bool());
        assert!(e.call(0x008f_5410, &args![this, 0x11u32]).bool());
        assert!(!e.call(0x008f_5410, &args![this, 0x33u32]).bool());
        // A node without an item ends the walk.
        e.mem.set_u32(this + 0x27c, 0);
        assert!(!e.call(0x008f_5410, &args![this, 0x22u32]).bool());
    }

    #[test]
    fn test_fn_008f6120() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x14);
        for offset in (0..0x14).step_by(4) {
            e.mem.set_u32(block + offset, 0xffff_ffff);
        }
        let r = e.call(0x008f_6120, &args![block]);
        assert_eq!(r.u32(), block);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u16(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
    }

    #[test]
    fn test_high_process_get_who_detects_me() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let this = e.mem.alloc(0x500);
        let first = detection_state_for(&mut e, 0xa1, 5);
        let second = detection_state_for(&mut e, 0xa2, 7);
        let list = make_list(&mut e, &[first, second]);
        e.mem.set_u32(this + 0x260, list);
        let result = e.call(0x008f_6180, &args![this, 0u32]).u32();
        let copies = list_items(&e, result);
        assert_eq!(copies.len(), 2);
        assert_ne!(copies[0], first);
        assert_eq!(e.mem.u32(copies[0]), 0xa1);
        assert_eq!(e.mem.i32(copies[0] + 8), 5);
        assert_eq!(e.mem.u32(copies[1]), 0xa2);
        assert_eq!(e.mem.i32(copies[1] + 8), 7);
    }

    #[test]
    fn test_high_process_insert_into_detection_list() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        // The actor's slot 0x1f4 returns a location of three floats.
        let location = e.mem.alloc(12);
        e.mem.set_f32(location, 1.0);
        e.mem.set_f32(location + 4, 2.0);
        e.mem.set_f32(location + 8, 3.0);
        e.register_double(0x00aa_aa10, move |_, _| ret(location));
        let actor_vtable = vtable(&mut e, &[(0x1f4, 0x00aa_aa10)]);
        let actor = object(&mut e, 0x40, actor_vtable);
        stub_float(&mut e, 0x0043_5dd0, 12.5);
        e.register_double(0x0043_5de0, |e, a| {
            e.mem.set_f32(a[0], f32::from_bits(a[1]));
            ret(0)
        });
        let this = e.mem.alloc(0x500);
        let thread_list = e.mem.alloc(8);
        let who_list = e.mem.alloc(8);
        e.mem.set_u32(this + 0x268, thread_list);
        e.mem.set_u32(this + 0x26c, who_list);
        // detects_me = 0: the thread list, bEvaluated cleared, time stamped.
        let state = e
            .call(
                0x008f_62b0,
                &args![this, actor, 3u32, 1u8, 4i32, 0i32, 1u8, 1u8],
            )
            .u32();
        assert_eq!(list_items(&e, thread_list), vec![state]);
        assert_eq!(e.mem.u32(state), actor);
        assert_eq!(e.mem.u32(state + 4), 3);
        assert_eq!(e.mem.u8(state + 0x1e), 1);
        assert_eq!(e.mem.i32(state + 8), 4);
        assert_eq!(e.mem.u8(state + 0x1c), 1);
        assert_eq!(e.mem.u8(state + 0x1d), 1);
        assert_eq!(e.mem.u8(state + 0x1f), 0);
        assert_eq!(e.mem.f32(state + 0xc), 1.0);
        assert_eq!(e.mem.f32(state + 0x14), 3.0);
        assert_eq!(e.mem.f32(state + 0x18), 12.5);
        // detects_me != 0 with a level of 0: the other list, no location.
        let other = e
            .call(
                0x008f_62b0,
                &args![this, actor, 0u32, 0u8, 0i32, 1i32, 0u8, 0u8],
            )
            .u32();
        assert_eq!(list_items(&e, who_list), vec![other]);
        assert_eq!(e.mem.u8(other + 0x1f), 1);
        assert_eq!(e.mem.f32(other + 0x18), 0.0);
    }

    #[test]
    fn test_fn_008f63e0() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        // The copy function `008d6fc0` and the state lookup slot are doubles.
        e.register_double(0x008d_6fc0, |e, a| {
            let actor = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], actor);
            ret(0)
        });
        e.register_double(0x00aa_aa20, |_, _| ret(0));
        let process_vtable = vtable(&mut e, &[(0x504, 0x00aa_aa20)]);
        let this = object(&mut e, 0x500, process_vtable);
        let owner_vtable = vtable(&mut e, &[(0x428, 0x00aa_aa30)]);
        let package = e.mem.alloc(0x100);
        e.register_double(0x00aa_aa30, move |_, _| ret(package));
        let owner = object(&mut e, 0x20, owner_vtable);
        e.register_double(0x008f_6630, |e, a| {
            e.mem.set_u8(a[0] + 0xc5, a[1] as u8);
            ret(0)
        });
        let state_a = detection_state_for(&mut e, 0xb1, 0);
        let state_b = detection_state_for(&mut e, 0xb2, 0);
        let thread_list = make_list(&mut e, &[state_a]);
        let who_list = make_list(&mut e, &[state_b]);
        let detected = e.mem.alloc(8);
        let who = e.mem.alloc(8);
        e.mem.set_u32(this + 0x268, thread_list);
        e.mem.set_u32(this + 0x26c, who_list);
        e.mem.set_u32(this + 0x25c, detected);
        e.mem.set_u32(this + 0x260, who);
        e.call_log = Some(vec![]);
        e.call(0x008f_63e0, &args![this, owner]);
        let detected_items = list_items(&e, detected);
        assert_eq!(detected_items.len(), 1);
        assert_eq!(e.mem.u32(detected_items[0]), 0xb1);
        let who_items = list_items(&e, who);
        assert_eq!(who_items.len(), 1);
        assert_eq!(e.mem.u32(who_items[0]), 0xb2);
        // Both source lists are emptied, the package flagged once.
        assert_eq!(e.mem.u32(thread_list), 0);
        assert_eq!(e.mem.u32(who_list), 0);
        assert_eq!(calls_to(&e, 0x008f_6630), vec![vec![package, 1]]);
        assert_eq!(e.mem.u8(package + 0xc5), 1);
    }

    #[test]
    fn test_fn_008f6630() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.call(0x008f_6630, &args![this, 1u8]);
        assert_eq!(e.mem.u8(this + 0xc5), 1);
        e.call(0x008f_6630, &args![this, 0u8]);
        assert_eq!(e.mem.u8(this + 0xc5), 0);
    }

    /// A process whose slot 0x504 returns the state of a scripted lookup.
    fn detection_process(e: &mut Engine, result: u32) -> u32 {
        e.register_double(0x00aa_aa40, move |_, _| ret(result));
        process(e, &[(0x504, 0x00aa_aa40)])
    }

    #[test]
    fn test_high_process_get_detection_state() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let this = e.mem.alloc(0x500);
        let first = detection_state_for(&mut e, 0xc1, 0);
        let second = detection_state_for(&mut e, 0xc2, 0);
        let list_a = make_list(&mut e, &[first, second]);
        let list_b = make_list(&mut e, &[second]);
        e.mem.set_u32(this + 0x25c, list_a);
        e.mem.set_u32(this + 0x260, list_b);
        assert_eq!(
            e.call(0x008f_6650, &args![this, 0xc2u32, 0i32]).u32(),
            second
        );
        assert_eq!(e.call(0x008f_6650, &args![this, 0xc1u32, 1i32]).u32(), 0);
        assert_eq!(
            e.call(0x008f_6650, &args![this, 0xc2u32, 1i32]).u32(),
            second
        );
        assert_eq!(e.call(0x008f_6650, &args![this, 0xc9u32, 0i32]).u32(), 0);
    }

    #[test]
    fn test_high_process_remove_detection_actor() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let this = e.mem.alloc(0x500);
        let a = detection_state_for(&mut e, 0xd1, 0);
        let b = detection_state_for(&mut e, 0xd2, 0);
        let c = detection_state_for(&mut e, 0xd1, 0);
        let d = detection_state_for(&mut e, 0xd1, 0);
        let first = make_list(&mut e, &[a, b, c]);
        let second = make_list(&mut e, &[d, b]);
        e.mem.set_u32(this + 0x25c, first);
        e.mem.set_u32(this + 0x260, second);
        e.call_log = Some(vec![]);
        // Only the first list (which = 0): both 0xd1 entries go.
        e.call(0x008f_66e0, &args![this, 0xd1u32, 0i32]);
        assert_eq!(list_items(&e, first), vec![b]);
        assert_eq!(list_items(&e, second), vec![d, b]);
        let freed: Vec<u32> = calls_to(&e, 0x0040_1030)
            .into_iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(freed, vec![a, c]);
        // Both lists (which = 3): the entry in the second list goes too.
        e.call(0x008f_66e0, &args![this, 0xd1u32, 3i32]);
        assert_eq!(list_items(&e, second), vec![b]);
    }

    #[test]
    fn test_high_process_get_last_position_detected() {
        let mut e = Engine::new();
        for (index, value) in [7.0f32, 8.0, 9.0].into_iter().enumerate() {
            global(&mut e, 0x011f_426c + 4 * index as u32, value.to_bits());
        }
        let state = e.mem.alloc(0x24);
        e.mem.set_f32(state + 0xc, 1.0);
        e.mem.set_f32(state + 0x10, 2.0);
        e.mem.set_f32(state + 0x14, 3.0);
        let found = detection_process(&mut e, state);
        let out = e.mem.alloc(12);
        let r = e.call(0x008f_6820, &args![found, out, 0x55u32, 0u32]);
        assert_eq!(r.u32(), out);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 8), 3.0);
        let missing = detection_process(&mut e, 0);
        e.call(0x008f_6820, &args![missing, out, 0x55u32, 0u32]);
        assert_eq!(e.mem.f32(out), 7.0);
        assert_eq!(e.mem.f32(out + 4), 8.0);
        assert_eq!(e.mem.f32(out + 8), 9.0);
    }

    #[test]
    fn test_high_process_get_last_time_detected() {
        let mut e = Engine::new();
        global(&mut e, 0x0108_7820, 1.0e10f32.to_bits());
        stub_float(&mut e, 0x006a_7f50, 42.5);
        let state = e.mem.alloc(0x24);
        let found = detection_process(&mut e, state);
        assert_eq!(e.call(0x008f_68a0, &args![found, 1u32, 0u32]).f32(), 42.5);
        let missing = detection_process(&mut e, 0);
        assert_eq!(
            e.call(0x008f_68a0, &args![missing, 1u32, 0u32]).f32(),
            -1.0e10
        );
    }

    #[test]
    fn test_high_process_get_360_line_sighton_actor() {
        let mut e = Engine::new();
        let state = e.mem.alloc(0x24);
        e.mem.set_u8(state + 0x1c, 1);
        let found = detection_process(&mut e, state);
        assert_eq!(e.call(0x008f_68f0, &args![found, 0u32, 1u32, 0u32]).u8(), 1);
        let missing = detection_process(&mut e, 0);
        assert_eq!(
            e.call(0x008f_68f0, &args![missing, 0u32, 1u32, 0u32]).u8(),
            0
        );
    }

    #[test]
    fn test_high_process_get_line_sighton_actor() {
        let mut e = Engine::new();
        let state = e.mem.alloc(0x24);
        e.mem.set_u8(state + 0x1c, 1);
        e.mem.set_u8(state + 0x1e, 0);
        let found = detection_process(&mut e, state);
        // Flag 0 reads bLineofSight (+0x1e), else b360view (+0x1c).
        assert_eq!(
            e.call(0x008f_6930, &args![found, 0u32, 1u32, 0u32, 0u8])
                .u8(),
            0
        );
        assert_eq!(
            e.call(0x008f_6930, &args![found, 0u32, 1u32, 0u32, 1u8])
                .u8(),
            1
        );
        let missing = detection_process(&mut e, 0);
        assert_eq!(
            e.call(0x008f_6930, &args![missing, 0u32, 1u32, 0u32, 1u8])
                .u8(),
            0
        );
    }

    #[test]
    fn test_high_process_get_detection_actor() {
        let mut e = Engine::new();
        let state = e.mem.alloc(0x24);
        e.mem.set_i32(state + 8, 33);
        let found = detection_process(&mut e, state);
        assert_eq!(e.call(0x008f_6990, &args![found, 1u32, 0u32]).i32(), 33);
        let missing = detection_process(&mut e, 0);
        assert_eq!(
            e.call(0x008f_6990, &args![missing, 1u32, 0u32]).i32(),
            0x7fff_ffff
        );
    }

    #[test]
    fn test_high_process_set_last_seen_location() {
        let mut e = Engine::new();
        let location = e.mem.alloc(12);
        e.mem.set_f32(location, 4.0);
        e.mem.set_f32(location + 4, 5.0);
        e.mem.set_f32(location + 8, 6.0);
        e.register_double(0x00aa_aa50, move |_, _| ret(location));
        let target_vtable = vtable(&mut e, &[(0x1f4, 0x00aa_aa50)]);
        let target = object(&mut e, 0x40, target_vtable);
        let this = e.mem.alloc(0x500);
        e.call(0x008f_6ea0, &args![this]);
        assert_eq!(e.mem.u32(this + 0xfc), 0);
        e.mem.set_u32(this + 0x40, target);
        e.call(0x008f_6ea0, &args![this]);
        assert_eq!(e.mem.f32(this + 0xfc), 4.0);
        assert_eq!(e.mem.f32(this + 0x100), 5.0);
        assert_eq!(e.mem.f32(this + 0x104), 6.0);
    }

    #[test]
    fn test_high_process_get_emotions_dispostion() {
        let mut e = Engine::new();
        e.register_double(0x00aa_aa60, |_, _| ret(1));
        e.register_double(0x00aa_aa70, |_, a| ret(a[1] + 100));
        e.register_double(0x00aa_aa80, |_, _| ret(0));
        e.register_double(0x0044_0da0, |_, a| ret(u32::from(a[0] == 0x7777)));
        let subject_vtable = vtable(&mut e, &[(0x218, 0x00aa_aa60), (0x344, 0x00aa_aa70)]);
        let subject = object(&mut e, 0x40, subject_vtable);
        let actor_vtable = vtable(&mut e, &[(0x100, 0x00aa_aa60)]);
        let target = object(&mut e, 0x40, actor_vtable);
        let blocked = object(&mut e, 0x40, actor_vtable);
        let this = e.mem.alloc(0x500);
        // No other, no target: the default.
        assert_eq!(e.call(0x008f_6ef0, &args![this, subject, 0u32]).u32(), 0x32);
        // The process target answers when the other is missing.
        e.mem.set_u32(this + 0x40, target);
        assert_eq!(
            e.call(0x008f_6ef0, &args![this, subject, 0u32]).u32(),
            target + 100
        );
        // `00440da0` true for the target: it is skipped.
        e.register_double(0x0044_0da0, move |_, _| ret(1));
        assert_eq!(e.call(0x008f_6ef0, &args![this, subject, 0u32]).u32(), 0x32);
        // An explicit other that is an actor.
        assert_eq!(
            e.call(0x008f_6ef0, &args![this, subject, blocked]).u32(),
            blocked + 100
        );
        let plain_vtable = vtable(&mut e, &[(0x100, 0x00aa_aa80)]);
        let plain = object(&mut e, 0x40, plain_vtable);
        assert_eq!(
            e.call(0x008f_6ef0, &args![this, subject, plain]).u32(),
            0x32
        );
    }

    #[test]
    fn test_high_process_is_talking() {
        let mut e = Engine::new();
        e.register_double(0x0093_36c0, |_, a| ret(u32::from(a[0] == 0x4001)));
        e.register_double(0x0041_ca90, |_, a| {
            ret(if a[0] == 0x5000 { 0x1c } else { 0 })
        });
        e.register_double(0x00ad_8ce0, |_, _| ret(0));
        e.register_double(0x00aa_aa90, |_, _| ret(0x5000));
        e.register_double(0x00aa_aaa0, |_, _| ret(0));
        let process_vtable = vtable(&mut e, &[(0x27c, 0x00aa_aa90), (0x30c, 0x00aa_aaa0)]);
        let this = object(&mut e, 0x500, process_vtable);
        let package = 0x5000;
        global(&mut e, package + 0x94, 0x4001);
        // The dialogue package names the actor.
        assert!(e.call(0x008f_6fc0, &args![this, 0x4001u32]).bool());
        // Another actor in dialogue: not talking to them.
        e.mem.set_u32(package + 0x94, 0x4002);
        e.register_double(0x0093_36c0, |_, _| ret(1));
        assert!(!e.call(0x008f_6fc0, &args![this, 0x4001u32]).bool());
        // Without an actor the sound handle (at +0x314) and slot 0x30c answer.
        e.register_double(0x00ad_8ce0, |_, _| ret(0));
        assert!(!e.call(0x008f_6fc0, &args![this, 0u32]).bool());
        e.register_double(0x00aa_aaa0, |_, _| ret(1));
        assert!(e.call(0x008f_6fc0, &args![this, 0u32]).bool());
        e.register_double(0x00aa_aaa0, |_, _| ret(0));
        e.register_double(0x00ad_8ce0, move |_, a| {
            ret(u32::from(a[0] == this + 0x314))
        });
        assert!(e.call(0x008f_6fc0, &args![this, 0u32]).bool());
    }

    #[test]
    fn test_high_process_update_3d_model() {
        let mut e = Engine::new();
        global(&mut e, 0x011d_ea3c, 0x0abc_0000);
        global(&mut e, 0x011c_5cb4, 0);
        for addr in [
            0x0093_6f50u32,
            0x0059_8040,
            0x0056_7490,
            0x0057_10c0,
            0x008c_4640,
            0x008b_0bd0,
            0x004b_6dc0,
            0x00b5_b1c0,
            0x00b5_eeb0,
            0x007a_f430,
            0x0060_56f0,
            0x005b_9b00,
        ] {
            stub(&mut e, addr, 0);
        }
        stub(&mut e, 0x0045_0b80, 0x0e0e_0000);
        stub(&mut e, 0x0084_e3a0, 0x1234);
        stub(&mut e, 0x008a_dcb0, 0);
        stub_float(&mut e, 0x0059_8040, 2.5);
        let flags = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let flag_source = flags.clone();
        e.register_double(0x00aa_ab00, move |_, _| ret(flag_source.get()));
        let node = e.mem.alloc(0x10);
        e.register_double(0x00aa_ab10, move |_, _| ret(node));
        stub(&mut e, 0x00aa_ab20, 0);
        let process = process(
            &mut e,
            &[
                (0x478, 0x00aa_ab00),
                (0x470, 0x00aa_ab20),
                (0x580, 0x00aa_ab20),
            ],
        );
        let actor_vtable = vtable(
            &mut e,
            &[
                (0x21c, 0x00aa_ab20),
                (0x1d0, 0x00aa_ab10),
                (0x210, 0x00aa_ab20),
                (0x218, 0x00aa_ab20),
            ],
        );
        let actor = object(&mut e, 0x100, actor_vtable);
        e.call_log = Some(vec![]);
        // No flags: only the tail runs, the other actor is not the player.
        e.call(0x008f_69e0, &args![process, actor]);
        assert_eq!(calls_to(&e, 0x0093_6f50), vec![vec![actor, 0]]);
        assert!(calls_to(&e, 0x00b5_b1c0).is_empty());
        // Flag 0x10: the refraction-style update with the float from 00598040.
        flags.set(0x10);
        e.call_log = Some(vec![]);
        e.call(0x008f_69e0, &args![process, actor]);
        assert_eq!(
            calls_to(&e, 0x0056_7490),
            vec![vec![actor, 2.5f32.to_bits()]]
        );
        // Flag 1 with flag 2: the guard byte is set around 005710c0 and reset.
        flags.set(3);
        e.call_log = Some(vec![]);
        e.call(0x008f_69e0, &args![process, actor]);
        assert_eq!(calls_to(&e, 0x00b5_b1c0), vec![vec![0x0e0e_0000, node]]);
        assert_eq!(calls_to(&e, 0x00b5_eeb0), vec![vec![0x0e0e_0000, node]]);
        assert_eq!(calls_to(&e, 0x0057_10c0).len(), 1);
        assert_eq!(e.mem.u8(0x011c_5cb4), 0);
        assert_eq!(calls_to(&e, 0x008c_4640), vec![vec![actor]]);
        assert_eq!(calls_to(&e, 0x008b_0bd0), vec![vec![actor, node]]);
        assert_eq!(calls_to(&e, 0x004b_6dc0), vec![vec![node, 0x1234]]);
        // The actor's slot 0x21c test stops everything but the tail.
        e.register_double(0x00aa_ab20, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x008f_69e0, &args![process, actor]);
        assert!(calls_to(&e, 0x00b5_b1c0).is_empty());
    }

    #[test]
    fn test_high_process_create_followno_escort() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        global(&mut e, 0x011d_ea3c, 0x0abc_0000);
        global(&mut e, 0x011c_d210, 0);
        e.map(0x011c_d210, 4);
        e.mem.set_f32(0x011c_d210, 0.0);
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 9.5);
        e.register_double(0x0040_3e20, move |_, _| ret(setting));
        // No running package: the follow package is always created.
        stub(&mut e, 0x00aa_ac00, 0);
        let actor_vtable = vtable(
            &mut e,
            &[
                (0x214, 0x00aa_ac10),
                (0x418, 0x00aa_ac20),
                (0x2f4, 0x00aa_ac20),
            ],
        );
        stub(&mut e, 0x00aa_ac10, 1);
        stub(&mut e, 0x00aa_ac20, 0);
        let actor = object(&mut e, 0x100, actor_vtable);
        let created = e.mem.alloc(0x100);
        let target_slot = e.mem.alloc(0x40);
        e.mem.set_u32(created + 0x30, target_slot);
        stub(&mut e, 0x0067_0b90, created);
        for addr in [
            0x0067_0fc0u32,
            0x0082_6b40,
            0x0082_6b90,
            0x0067_2fc0,
            0x007b_3fa0,
            0x0068_00b0,
            0x0068_0110,
            0x0040_3550,
            0x0098_4f60,
            0x0067_3400,
            0x0067_a460,
            0x0067_1a20,
        ] {
            stub(&mut e, addr, 0);
        }
        stub(&mut e, 0x0067_ff70, 0x0cc0_0000);
        stub(&mut e, 0x0067_1d10, target_slot);
        let process = process(&mut e, &[(0x27c, 0x00aa_ac00), (0x628, 0x00aa_ac20)]);
        e.call_log = Some(vec![]);
        e.call(0x008f_7070, &args![process, actor, 1u8]);
        assert_eq!(calls_to(&e, 0x0067_0b90), vec![vec![1]]);
        // The count is 3000 for the long-range case, and the player is the target.
        assert_eq!(calls_to(&e, 0x0040_3550), vec![vec![target_slot, 3000]]);
        assert_eq!(
            calls_to(&e, 0x0068_0110),
            vec![vec![target_slot, 0x0abc_0000]]
        );
        assert_eq!(calls_to(&e, 0x00aa_ac20).len(), 3);
        assert_eq!(e.mem.f32(process + 0x34c), 9.5);
        // Short range: 80, no timer.
        e.mem.set_f32(process + 0x34c, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x008f_7070, &args![process, actor, 0u8]);
        assert_eq!(calls_to(&e, 0x0040_3550), vec![vec![target_slot, 0x50]]);
        assert_eq!(e.mem.f32(process + 0x34c), 0.0);
    }

    #[test]
    fn test_high_process_finish_dying() {
        let mut e = Engine::new();
        global(&mut e, 0x0103_5838, 0);
        e.map(0x0103_5838, 8);
        e.mem.set_f64(0x0103_5838, 23.0);
        e.map(0x0107_6f68, 8);
        e.mem.set_f64(0x0107_6f68, 0.0005);
        global(&mut e, 0x0101_2054, (-1.0f32).to_bits());
        stub(&mut e, 0x0043_fcd0, 0x3d00);
        let dead = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let dead_source = dead.clone();
        e.register_double(0x00c6_a440, move |_, _| ret(dead_source.get()));
        stub_float(&mut e, 0x007d_f1f0, 22.0);
        stub_float(&mut e, 0x0086_7da0, 2.0);
        stub_float(&mut e, 0x0086_7950, 20.0);
        stub(&mut e, 0x0048_f7f0, 0x7700);
        stub(&mut e, 0x005f_2420, 0xe0);
        stub(&mut e, 0x0049_1040, 0x6600);
        stub_float(&mut e, 0x0049_1090, 3.0);
        stub(&mut e, 0x0049_1180, 0);
        stub(&mut e, 0x008b_01c0, 0);
        stub(&mut e, 0x005d_43c0, 0x9900);
        stub(&mut e, 0x0041_d490, 0);
        stub(&mut e, 0x00aa_ad00, 0x5500);
        stub(&mut e, 0x00aa_ad10, 0);
        let process = process(&mut e, &[(0x1b8, 0x00aa_ad00)]);
        let actor_vtable = vtable(&mut e, &[(0x1c0, 0x00aa_ad10), (0xd4, 0x00aa_ad10)]);
        let actor = object(&mut e, 0x100, actor_vtable);
        e.call_log = Some(vec![]);
        // Hours since 22:00 at 02:00 are 4 (22 -> 2 wraps: 22 + 23 - 2 = 43 hmm no: 2 <= 22 so 22 - 2... see below).
        e.call(0x008f_7350, &args![process, actor]);
        // The animation is updated once with the sequence length and the global float.
        assert_eq!(
            calls_to(&e, 0x0049_1180),
            vec![vec![0x5500, actor, 3.0f32.to_bits(), (-1.0f32).to_bits()]]
        );
        // last (22.0) > now (2.0): (22 + 23) - 2 = 43 is not below 20 * 0.0005.
        assert_eq!(calls_to(&e, 0x008b_01c0), vec![vec![actor]]);
        assert_eq!(calls_to(&e, 0x0041_d490), vec![vec![0x9900, actor]]);
        // Close to the time of death: the actor is left as it is.
        stub_float(&mut e, 0x007d_f1f0, 2.0);
        stub_float(&mut e, 0x0086_7950, 20_000.0);
        e.call_log = Some(vec![]);
        e.call(0x008f_7350, &args![process, actor]);
        assert!(calls_to(&e, 0x008b_01c0).is_empty());
        // The first test fails: the death stuff runs without the time check.
        dead.set(0);
        e.call_log = Some(vec![]);
        e.call(0x008f_7350, &args![process, actor]);
        assert_eq!(calls_to(&e, 0x008b_01c0).len(), 1);
    }

    #[test]
    fn test_high_process_aim_at_target() {
        let mut e = Engine::new();
        for (index, value) in [0.0f32, 0.0, 0.0].into_iter().enumerate() {
            global(&mut e, 0x011f_426c + 4 * index as u32, value.to_bits());
        }
        global(&mut e, 0x0108_8250, 0.5f32.to_bits());
        let attacker = e.mem.alloc(0x100);
        let target = e.mem.alloc(0x100);
        let this = e.mem.alloc(0x500);
        // No target: false and nothing is called.
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x008f_74c0, &args![this, attacker, 0u32, 0u8])
            .bool());
        assert!(e.call_log.as_ref().unwrap().len() == 1);
        // Without a weapon: the plain target firing location, then the look
        // target, and bAimingTarget is set; 009a6ae0 answers.
        stub(&mut e, 0x008a_1710, 0);
        let attacker_location = e.mem.alloc(12);
        let target_location = e.mem.alloc(12);
        e.mem.set_f32(target_location, 1.5);
        e.mem.set_f32(target_location + 4, 2.5);
        e.mem.set_f32(target_location + 8, 3.5);
        stub(&mut e, 0x009a_8050, attacker_location);
        stub(&mut e, 0x009a_8600, target_location);
        stub(&mut e, 0x008b_3cd0, 0);
        stub(&mut e, 0x009a_6ae0, 1);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x008f_74c0, &args![this, attacker, target, 0u8])
            .bool());
        assert_eq!(e.mem.u8(this + 0xe1), 1);
        assert_eq!(
            calls_to(&e, 0x008b_3cd0),
            vec![vec![
                attacker,
                1.5f32.to_bits(),
                2.5f32.to_bits(),
                3.5f32.to_bits()
            ]]
        );
        assert_eq!(calls_to(&e, 0x009a_6ae0), vec![vec![attacker, target, 0]]);
        // With a thrown-style weapon: 006450c0 false, the iron sights and the
        // firing arc test decide.
        let weapon = e.mem.alloc(0x200);
        e.mem.set_f32(weapon + 0x170, 4.0);
        stub(&mut e, 0x008a_1710, weapon);
        stub(&mut e, 0x0052_5a90, 0);
        stub(&mut e, 0x0052_57a0, attacker_location);
        stub(&mut e, 0x0064_50c0, 0);
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 2.0);
        e.register_double(0x0040_3e20, move |_, _| ret(setting));
        let attacker_vtable = vtable(&mut e, &[(0x1f4, 0x00aa_ae00)]);
        stub(&mut e, 0x00aa_ae00, attacker_location);
        e.mem.set_u32(attacker, attacker_vtable);
        stub(&mut e, 0x0043_9ef0, 0);
        stub_float(&mut e, 0x004a_7290, 100.0);
        stub(&mut e, 0x008b_b650, 0);
        stub(&mut e, 0x009a_6d70, 1);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x008f_74c0, &args![this, attacker, target, 0u8])
            .bool());
        // range = 4.0 * 2.0 = 8.0; 64 < 100: iron sights are set.
        assert_eq!(calls_to(&e, 0x008b_b650), vec![vec![attacker, 1, 0, 0]]);
        assert_eq!(calls_to(&e, 0x009a_6d70).len(), 1);
    }

    #[test]
    fn test_middle_high_process_get_alpha_mult() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x500);
        e.mem.set_f32(this + 0x170, 0.75);
        assert_eq!(e.call(0x008f_7710, &args![this]).f32(), 0.75);
    }

    fn use_weapon_data(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(0x40);
        e.mem.set_u8(data + 0xe, 0x11);
        e.mem.set_u8(data + 0xf, 0x22);
        e.mem.set_u16(data + 0x14, 0x3344);
        e.mem.set_u16(data + 0x16, 0x5566);
        data
    }

    #[test]
    fn test_fn_008f9100() {
        let mut e = Engine::new();
        let data = use_weapon_data(&mut e);
        assert_eq!(e.call(0x008f_9100, &args![data]).u8(), 0x11);
    }

    #[test]
    fn test_fn_008f9120() {
        let mut e = Engine::new();
        let data = use_weapon_data(&mut e);
        assert_eq!(e.call(0x008f_9120, &args![data]).u8(), 0x22);
    }

    #[test]
    fn test_fn_008f9140() {
        let mut e = Engine::new();
        let data = use_weapon_data(&mut e);
        assert_eq!(e.call(0x008f_9140, &args![data]).u16(), 0x3344);
    }

    #[test]
    fn test_fn_008f9160() {
        let mut e = Engine::new();
        let data = use_weapon_data(&mut e);
        assert_eq!(e.call(0x008f_9160, &args![data]).u16(), 0x5566);
    }

    #[test]
    fn test_high_process_clean_up_after_process_use_weapon() {
        let mut e = Engine::new();
        let package = 0x4_0000u32;
        global(&mut e, package, 0);
        stub(&mut e, 0x0067_58f0, 0x4_1000);
        let finished = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let finished_source = finished.clone();
        e.register_double(0x0082_4060, move |_, _| ret(finished_source.get()));
        stub(&mut e, 0x008f_21d0, 3);
        stub(&mut e, 0x0044_ddc0, 0x4_2000);
        stub(&mut e, 0x0040_1170, 0x28);
        stub(&mut e, 0x004c_0bf0, 1);
        for addr in [
            0x008a_6840u32,
            0x0088_c790,
            0x008b_3d30,
            0x0093_1d90,
            0x008b_b650,
            0x005c_e9d0,
            0x0092_bf20,
        ] {
            stub(&mut e, addr, 0);
        }
        stub(&mut e, 0x008b_3bb0, 0);
        stub(&mut e, 0x00aa_af00, package);
        stub(&mut e, 0x00aa_af10, 0x4_3000);
        stub(&mut e, 0x00aa_af20, 0);
        let process = process(
            &mut e,
            &[
                (0x27c, 0x00aa_af00),
                (0x148, 0x00aa_af10),
                (0x394, 0x00aa_af20),
                (0x654, 0x00aa_af20),
                (0x294, 0x00aa_af20),
            ],
        );
        e.mem.set_u16(process + 0x2c4, 3);
        let actor = 0x4_4000u32;
        e.call_log = Some(vec![]);
        // finished = 1 (00824060 true): the weapon is kept; the rest cleans up.
        e.call(0x008f_9180, &args![process, actor]);
        assert!(calls_to(&e, 0x0088_c790).is_empty());
        assert_eq!(calls_to(&e, 0x008b_3d30), vec![vec![actor]]);
        assert_eq!(calls_to(&e, 0x0093_1d90), vec![vec![actor, 0]]);
        assert_eq!(calls_to(&e, 0x00aa_af20).len(), 3);
        // 00824060 false and enough bursts: the weapon is unequipped.
        finished.set(0);
        e.call_log = Some(vec![]);
        e.call(0x008f_9180, &args![process, actor]);
        assert_eq!(
            calls_to(&e, 0x0088_c790),
            vec![vec![actor, 0x4_2000, 1, 0, 0, 0, 1]]
        );
        // Too few bursts: kept.
        e.mem.set_u16(process + 0x2c4, 2);
        e.call_log = Some(vec![]);
        e.call(0x008f_9180, &args![process, actor]);
        assert!(calls_to(&e, 0x0088_c790).is_empty());
    }

    #[test]
    fn test_fn_008fdb10() {
        let mut e = Engine::new();
        for (index, value) in [7.0f32, 8.0, 9.0].into_iter().enumerate() {
            global(&mut e, 0x011f_426c + 4 * index as u32, value.to_bits());
        }
        global(&mut e, 0x0108_7820, 5.0e9f32.to_bits());
        e.register_double(0x0068_15c0, |_, a| ret(a[0]));
        stub(&mut e, 0x0043_00a0, 0);
        e.register_double(0x0043_5de0, |e, a| {
            e.mem.set_f32(a[0], f32::from_bits(a[1]));
            ret(0)
        });
        let block = e.mem.alloc(0x1c);
        let r = e.call(0x008f_db10, &args![block]);
        assert_eq!(r.u32(), block);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.f32(block + 4), 7.0);
        assert_eq!(e.mem.f32(block + 0xc), 9.0);
        assert_eq!(e.mem.f32(block + 0x10), -5.0e9);
        assert_eq!(e.mem.u32(block + 0x14), 0xffff_ffff);
        assert_eq!(e.mem.u32(block + 0x18), 0);
    }

    /// The save/load object: its +0x14 word is the stream position
    /// (`00825c00`). `Save`/`SaveNumericID` copy bytes to the stream and
    /// `Load`/`LoadNumericID` copy them back; `version` is the save version.
    fn stream_doubles(e: &mut Engine, version: u32, blocks: bool) -> (u32, u32) {
        let tes = e.mem.alloc(0x100);
        let stream = e.mem.alloc(0x2000);
        e.mem.set_u32(tes + 0x14, stream);
        global(e, 0x011d_e45c, tes);
        e.register_double(0x0082_5c00, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register_double(0x0086_2110, move |_, _| ret(u32::from(blocks)));
        e.register_double(0x008d_f040, move |_, _| ret(version));
        e.register_double(0x0084_e3a0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        for addr in [0x0085_79b0u32, 0x0085_7a10] {
            e.register_double(addr, |e, a| {
                let position = e.mem.u32(a[0] + 0x14);
                for byte in 0..a[2] {
                    let value = e.mem.u8(a[1] + byte);
                    e.mem.set_u8(position + byte, value);
                }
                e.mem.set_u32(a[0] + 0x14, position + a[2]);
                ret(0)
            });
        }
        for addr in [0x0085_79e0u32, 0x0085_7aa0] {
            e.register_double(addr, |e, a| {
                let position = e.mem.u32(a[0] + 0x14);
                for byte in 0..a[2] {
                    let value = e.mem.u8(position + byte);
                    e.mem.set_u8(a[1] + byte, value);
                }
                e.mem.set_u32(a[0] + 0x14, position + a[2]);
                ret(0)
            });
        }
        // The debug-logging setting is off (a byte at the returned address).
        let flag = e.mem.alloc(4);
        e.register_double(0x0040_8d60, move |_, _| ret(flag));
        e.register_double(0x005b_5e40, |_, _| ret(0));
        e.register_double(0x0040_fbe0, |_, _| ret(0));
        stub(e, 0x004f_d3e0, 0);
        stub(e, 0x004f_d3c0, 0);
        (tes, stream)
    }

    #[test]
    fn test_high_process_get_save_size() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let (tes, _) = stream_doubles(&mut e, 0x71, true);
        let _ = tes;
        stub(&mut e, 0x0092_4220, 100);
        stub(&mut e, 0x0092_6350, 7);
        let counted = std::rc::Rc::new(std::cell::Cell::new(2u32));
        let counted_source = counted.clone();
        e.register_double(0x005a_e380, move |_, _| ret(counted_source.get()));
        let this = e.mem.alloc(0x500);
        // The newest version, with save blocks and the flag: every field.
        let size = e
            .call(0x008f_a630, &args![this, 0x1000_0000u32, 0u32])
            .u16();
        assert_eq!(size, 306);
        // An old version without blocks: only the base fields.
        e.register_double(0x008d_f040, |_, _| ret(0x10));
        e.register_double(0x0086_2110, |_, _| ret(0));
        counted.set(0);
        let size = e.call(0x008f_a630, &args![this, 0u32, 0u32]).u16();
        assert_eq!(size, 218);
        // With logging on and no world space the size is logged.
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        e.register_double(0x0040_8d60, move |_, _| ret(flag));
        stub(&mut e, 0x004f_d3e0, 0);
        e.call_log = Some(vec![]);
        e.call(0x008f_a630, &args![this, 0u32, 0u32]);
        let logged = calls_to(&e, 0x0040_fbe0);
        assert_eq!(logged.len(), 1);
        assert_eq!(logged[0][0], 0x0101_2c78);
        assert_eq!(logged[0][1], 218 - 100);
    }

    /// Gives the process distinct values in the fields the save writes.
    fn fill_saved_fields(e: &mut Engine, this: u32) {
        for offset in (0x290..0x460).step_by(4) {
            e.mem.set_u32(this + offset, 0x0100_0000 + offset);
        }
        // Flag bytes the save treats as booleans.
        for offset in [
            0x2c6u32, 0x2dc, 0x32c, 0x340, 0x349, 0x374, 0x375, 0x3a0, 0x3a8, 0x3b8, 0x420,
        ] {
            e.mem.set_u8(this + offset, 1);
        }
    }

    /// Pointer fields set to forms whose IDs (word at +0xc) are known.
    fn set_form_fields(e: &mut Engine, this: u32, offsets: &[u32]) -> Vec<u32> {
        let mut ids = Vec::new();
        for (index, offset) in offsets.iter().enumerate() {
            let form = e.mem.alloc(0x20);
            let id = 0x00ff_0000 + index as u32;
            e.mem.set_u32(form + 0xc, id);
            e.mem.set_u32(this + offset, form);
            ids.push(id);
        }
        ids
    }

    #[test]
    fn test_high_process_save_game_and_load_game_round_trip() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let (tes, stream) = stream_doubles(&mut e, 0x7d, true);
        stub(&mut e, 0x0092_4560, 0);
        stub(&mut e, 0x0092_4b90, 0);
        stub(&mut e, 0x0092_6350, 0);
        stub(&mut e, 0x0085_7bd0, 0);
        global(&mut e, 0x011d_ea3c, 0x0abc_0000);
        let this = e.mem.alloc(0x500);
        fill_saved_fields(&mut e, this);
        let ids = set_form_fields(&mut e, this, &[0x30c, 0x2a4, 0x3f0, 0x41c, 0x370]);
        // The saved detection list: one state with an actor form.
        let actor_form = e.mem.alloc(0x20);
        e.mem.set_u32(actor_form + 0xc, 0x00ee_0001);
        let state = detection_state_for(&mut e, actor_form, 3);
        e.mem.set_u32(state + 4, 2);
        e.mem.set_u8(state + 0x1e, 1);
        let list = make_list(&mut e, &[state]);
        e.mem.set_u32(this + 0x25c, list);
        e.mem.set_u32(this + 0x2f0, 0);
        // A reference form lookup for the loaded IDs.
        e.register_double(0x0048_39c0, |_, a| ret(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x008f_aa50, &args![this, 0u32, 0u32]);
        // The block tag and the patched sizes are in the stream.
        assert_eq!(e.mem.u32(stream), 0x424c_4f4b);
        let end = e.mem.u32(tes + 0x14);
        assert_eq!(u32::from(e.mem.u16(stream + 4)), end - (stream + 4));
        assert_eq!(calls_to(&e, 0x0092_4560), vec![vec![this, 0, 0]]);
        // Load into a fresh process.
        e.mem.set_u32(tes + 0x14, stream);
        let fresh = e.mem.alloc(0x500);
        let fresh_list = e.mem.alloc(8);
        e.mem.set_u32(fresh + 0x25c, fresh_list);
        e.call_log = Some(vec![]);
        e.call(0x008f_b330, &args![fresh, 0u32, 0u32, 0u32]);
        assert!(calls_to(&e, 0x005b_5e40).is_empty());
        for offset in [
            0x2e8u32, 0x2b4, 0x2f8, 0x310, 0x330, 0x294, 0x298, 0x378, 0x39c, 0x3bc,
        ] {
            assert_eq!(
                e.mem.u32(fresh + offset),
                0x0100_0000 + offset,
                "{offset:x}"
            );
        }
        for offset in [0x32cu32, 0x340, 0x374, 0x375, 0x349, 0x420] {
            assert_eq!(e.mem.u8(fresh + offset), 1, "{offset:x}");
        }
        assert_eq!(e.mem.u32(fresh + 0x30c), ids[0]);
        assert_eq!(e.mem.u32(fresh + 0x2a4), ids[1]);
        assert_eq!(e.mem.u32(fresh + 0x370), ids[4]);
        // One detection state was rebuilt with the saved actor ID.
        let loaded = list_items(&e, fresh_list);
        assert_eq!(loaded.len(), 1);
        assert_eq!(e.mem.u32(loaded[0]), 0x00ee_0001);
        assert_eq!(e.mem.u32(loaded[0] + 4), 2);
        assert_eq!(e.mem.u8(loaded[0] + 0x1e), 1);
        // The stream was consumed exactly.
        assert_eq!(e.mem.u32(tes + 0x14), end);
    }

    #[test]
    fn test_high_process_load_game_reports_a_missing_block_tag() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let (tes, stream) = stream_doubles(&mut e, 0x71, true);
        stub(&mut e, 0x0092_4b90, 0);
        stub(&mut e, 0x0092_6350, 0);
        global(&mut e, 0x011d_ea3c, 0x0abc_0000);
        stub(&mut e, 0x004f_d3c0, 0);
        e.mem.set_u32(stream, 0x1234_5678);
        let this = e.mem.alloc(0x500);
        e.mem.set_u32(this + 0x25c, e.mem.u32(this + 0x25c));
        let detected = e.mem.alloc(8);
        e.mem.set_u32(this + 0x25c, detected);
        e.register_double(0x0048_39c0, |_, a| ret(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x008f_b330, &args![this, 0u32, 0u32, 0u32]);
        let errors = calls_to(&e, 0x005b_5e40);
        assert!(!errors.is_empty());
        // The first report is the tag mismatch without a world space.
        assert_eq!(errors[0][0], 0x0101_56a8);
        assert_eq!(errors[0][2], 0x320d);
        let _ = tes;
    }

    #[test]
    fn test_high_process_init_load_game() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let (_, _) = stream_doubles(&mut e, 0x71, false);
        stub(&mut e, 0x0092_5490, 0);
        // The cast: lookups answer the ID itself, the cast keeps the form
        // unless it is the "unresolvable" one.
        e.register_double(0x0048_39c0, |_, a| ret(a[0]));
        e.register_double(0x00ec_43fb, |_, a| ret(if a[0] == 0x66 { 0 } else { a[0] }));
        global(&mut e, 0x0118_3028, 0);
        let this = e.mem.alloc(0x500);
        e.mem.set_u32(this + 0x30c, 0x11);
        e.mem.set_u32(this + 0x2a4, 0x66);
        e.mem.set_u32(this + 0x3f0, 0x33);
        e.mem.set_u32(this + 0x41c, 0x44);
        e.mem.set_u32(this + 0x370, 0x55);
        // Two detection states: one resolvable, one not.
        let good = e.mem.alloc(0x24);
        e.mem.set_u32(good, 0x77);
        let bad = e.mem.alloc(0x24);
        e.mem.set_u32(bad, 0x66);
        let list = make_list(&mut e, &[bad, good]);
        e.mem.set_u32(this + 0x25c, list);
        e.call(0x008f_bfd0, &args![this, 0u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0x30c), 0x11);
        assert_eq!(e.mem.u32(this + 0x2a4), 0);
        assert_eq!(e.mem.u32(this + 0x3f0), 0x33);
        assert_eq!(e.mem.u32(this + 0x41c), 0x44);
        assert_eq!(e.mem.u32(this + 0x370), 0x55);
        // The unresolved state is gone.
        assert_eq!(list_items(&e, list).len(), 1);
        assert_eq!(e.mem.u32(e.mem.u32(list)), 0x77);
    }

    #[test]
    fn test_high_process_revert() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        global(&mut e, 0x011d_e45c, 0x1234);
        global(&mut e, 0x0101_7868, 3.5f32.to_bits());
        stub(&mut e, 0x0092_6000, 0);
        stub(&mut e, 0x0055_f5b0, 1);
        stub(&mut e, 0x008c_6eb0, 0);
        stub(&mut e, 0x00aa_b000, 0);
        let process = process(&mut e, &[(0x394, 0x00aa_b000)]);
        let cache = e.mem.alloc(0x10);
        e.mem.set_u32(process + 0x428, cache);
        let spells = make_list(&mut e, &[0x99]);
        e.mem.set_u32(process + 0x3b4, spells);
        e.mem.set_u32(process + 0x3f8, 0xffff);
        e.mem.set_u8(process + 0x410, 1);
        e.mem.set_u32(process + 0x41c, 0x1234);
        e.mem.set_u32(process + 0x3a4, 5);
        let actor_vtable = vtable(&mut e, &[(0x100, 0x00aa_b010)]);
        stub(&mut e, 0x00aa_b010, 1);
        let actor = object(&mut e, 0x40, actor_vtable);
        e.call_log = Some(vec![]);
        e.call(0x008f_c210, &args![process, 0u32, actor]);
        assert_eq!(calls_to(&e, 0x008c_6eb0), vec![vec![cache]]);
        assert_eq!(e.mem.u8(process + 0x340), 1);
        assert_eq!(e.mem.u32(process + 0xb4), 0xffff_ffff);
        assert_eq!(e.mem.f32(process + 0x33c), 3.5);
        assert_eq!(e.mem.f32(process + 0x384), 1.0);
        assert_eq!(e.mem.u32(process + 0x3f8), 0);
        assert_eq!(e.mem.u8(process + 0x410), 0);
        assert_eq!(e.mem.u32(process + 0x41c), 0);
        assert_eq!(e.mem.u32(process + 0x3a4), 0);
        assert_eq!(e.mem.u32(process + 0x3b4), 0);
        assert_eq!(calls_to(&e, 0x00aa_b000), vec![vec![process, actor]]);
        // When the save/load object says no, only the base revert runs.
        stub(&mut e, 0x0055_f5b0, 0);
        e.mem.set_u8(process + 0x340, 0);
        e.call(0x008f_c210, &args![process, 0u32, actor]);
        assert_eq!(e.mem.u8(process + 0x340), 0);
    }

    /// Serialisation doubles for the `BGSSaveGameBuffer` / `BGSLoadGameBuffer`
    /// functions: the buffer's +0xc word is the stream position; forms are
    /// stored as their own address.
    fn buffer_doubles(e: &mut Engine, version: u32) -> (u32, u32) {
        let stream = e.mem.alloc(0x4000);
        e.register_double(0x00aa_b100, move |_, _| ret(version));
        let buffer_vtable = vtable(e, &[(0, 0x00aa_b100)]);
        let buffer = object(e, 0x40, buffer_vtable);
        e.mem.set_u32(buffer + 0xc, stream);
        fn put(e: &mut Engine, buffer: u32, bytes: &[u8]) {
            let position = e.mem.u32(buffer + 0xc);
            e.mem.write(position, bytes);
            e.mem.set_u32(buffer + 0xc, position + bytes.len() as u32);
        }
        fn take(e: &mut Engine, buffer: u32, size: u32) -> Vec<u8> {
            let position = e.mem.u32(buffer + 0xc);
            let bytes = e.mem.bytes(position, size);
            e.mem.set_u32(buffer + 0xc, position + size);
            bytes
        }
        e.register_double(0x0086_5e50, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            put(e, a[0], &bytes);
            ret(0)
        });
        e.register_double(0x0086_5df0, |e, a| {
            put(e, a[0], &a[1].to_le_bytes());
            ret(0)
        });
        e.register_double(0x0086_5f20, |e, a| {
            let position = e.mem.u32(a[0] + 0xc);
            put(e, a[0], &0u32.to_le_bytes());
            ret(position)
        });
        e.register_double(0x0086_5ff0, |e, a| {
            e.mem.set_u32(a[2], a[1]);
            ret(0)
        });
        e.register_double(0x0084_e3a0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register_double(0x0086_4980, |e, a| {
            let bytes = take(e, a[0], a[2]);
            e.mem.write(a[1], &bytes);
            ret(0)
        });
        e.register_double(0x0086_48e0, |e, a| {
            let bytes = take(e, a[0], 4);
            e.mem.write(a[1], &bytes);
            ret(0)
        });
        e.register_double(0x0086_48a0, |e, a| {
            let bytes = take(e, a[0], 4);
            ret(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        });
        e.register_double(0x0086_4a60, |e, a| {
            let bytes = take(e, a[0], 4);
            ret(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        });
        // The item savers write the item's first word, the loaders read it.
        for addr in [0x0083_ce40u32, 0x008d_7270] {
            e.register_double(addr, |e, a| {
                let word = e.mem.u32(a[0]);
                put(e, a[1], &word.to_le_bytes());
                ret(0)
            });
        }
        for addr in [0x008d_73b0u32, 0x008d_7070] {
            e.register_double(addr, |e, a| {
                let word = e.mem.u32(a[0]);
                put(e, a[1], &word.to_le_bytes());
                ret(0)
            });
        }
        for addr in [0x0083_cf40u32, 0x008d_72e0, 0x008d_7420, 0x008d_7140] {
            e.register_double(addr, |e, a| {
                let bytes = take(e, a[1], 4);
                e.mem.write(a[0], &bytes);
                ret(0)
            });
        }
        for addr in [0x0083_c4d0u32, 0x008f_db90, 0x008f_db10, 0x0066_e590] {
            e.register_double(addr, |_, a| ret(a[0]));
        }
        e.register_double(0x0092_8880, |e, a| {
            put(e, a[1], &0xcc_u32.to_le_bytes());
            ret(0)
        });
        stub(e, 0x0092_6a20, 0);
        stub(e, 0x0092_7000, 0);
        stub(e, 0x0084_a810, 0);
        global(e, 0x011d_df38, 0x5555);
        e.register_double(0x0048_39c0, |_, a| ret(a[0]));
        e.register_double(0x00ec_43fb, |_, a| ret(a[0]));
        (buffer, stream)
    }

    #[test]
    fn test_high_process_save_game_ov2_and_load_game_ov2_round_trip() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let (buffer, stream) = buffer_doubles(&mut e, 0x71);
        let this = e.mem.alloc(0x500);
        for offset in (0x290..0x460).step_by(4) {
            e.mem.set_u32(this + offset, 0x0200_0000 + offset);
        }
        // Pointer fields, lists and optional objects.
        let forms: Vec<u32> = (0..8).map(|_| e.mem.alloc(0x20)).collect();
        for (index, offset) in [0x30cu32, 0x2a4, 0x3f0, 0x41c, 0x370, 0x350, 0x2ac]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(this + offset, forms[index]);
        }
        for offset in [0x3f8u32, 0x3fc, 0x400, 0x404, 0x408, 0x40c] {
            e.mem.set_u32(this + offset, forms[7]);
        }
        e.mem.set_u32(this + 0x3cc, 0);
        // The two inline lists (+0x38c, +0x394) and the pointer list (+0x264).
        e.mem.set_u32(this + 0x38c, forms[0]);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, forms[1]);
        e.mem.set_u32(this + 0x390, second);
        e.mem.set_u32(this + 0x394, forms[2]);
        e.mem.set_u32(this + 0x398, 0);
        let spoken = make_list(&mut e, &[forms[3], forms[4]]);
        e.mem.set_u32(this + 0x264, spoken);
        let topic = e.mem.alloc(0x1c);
        e.mem.set_u32(topic, 0x7001);
        e.mem.set_u32(this + 0x368, topic);
        let area = e.mem.alloc(0x34);
        e.mem.set_u32(area, 0x7002);
        let avoid = make_list(&mut e, &[area]);
        e.mem.set_u32(this + 0x44c, avoid);
        let state_a = detection_state_for(&mut e, 0x7003, 0);
        let state_b = detection_state_for(&mut e, 0x7004, 0);
        let detected = make_list(&mut e, &[state_a]);
        let who = make_list(&mut e, &[state_b]);
        e.mem.set_u32(this + 0x25c, detected);
        e.mem.set_u32(this + 0x260, who);
        let event = e.mem.alloc(0x1c);
        e.mem.set_u32(event, 0x7005);
        e.mem.set_u32(this + 0x3dc, event);
        e.call(0x008f_c4f0, &args![this, buffer]);
        let written = e.mem.u32(buffer + 0xc) - stream;
        assert!(written > 0x100);
        // Load the stream into a fresh process.
        e.mem.set_u32(buffer + 0xc, stream);
        let fresh = e.mem.alloc(0x500);
        for offset in [0x264u32, 0x25c, 0x260] {
            let head = e.mem.alloc(8);
            e.mem.set_u32(fresh + offset, head);
        }
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0xb8);
        e.call(0x008f_ce90, &args![fresh, buffer]);
        // The 0xb8-byte frame: the form words at EBP-0x2c (+0x8c) and EBP-0x38 (+0x80).
        let appends = calls_to(&e, LIST_APPEND);
        assert!(appends
            .iter()
            .filter(|c| c[0] == fresh + 0x38c)
            .all(|c| c[1] == frame + 0x8c));
        assert!(appends
            .iter()
            .filter(|c| c[0] == fresh + 0x394)
            .all(|c| c[1] == frame + 0x80));
        assert!(appends.iter().any(|c| c[0] == fresh + 0x38c));
        // The character controller block (a 4-byte size and its 4 bytes) is read
        // by the base class load, which is a double here.
        assert_eq!(e.mem.u32(buffer + 0xc) - stream, written - 8);
        for offset in [
            0x2b4u32, 0x2f8, 0x310, 0x330, 0x298, 0x378, 0x39c, 0x3a4, 0x3bc, 0x344,
        ] {
            assert_eq!(
                e.mem.u32(fresh + offset),
                0x0200_0000 + offset,
                "{offset:x}"
            );
        }
        // The word at +0x3d8 is taken modulo 12.
        assert_eq!(e.mem.u32(fresh + 0x3d8), (0x0200_0000 + 0x3d8) % 12);
        for (index, offset) in [0x30cu32, 0x2a4, 0x3f0, 0x41c, 0x370, 0x350, 0x2ac]
            .into_iter()
            .enumerate()
        {
            assert_eq!(e.mem.u32(fresh + offset), forms[index], "{offset:x}");
        }
        assert_eq!(e.mem.u32(fresh + 0x3f8), forms[7]);
        assert_eq!(list_items(&e, fresh + 0x38c), vec![forms[0], forms[1]]);
        assert_eq!(list_items(&e, fresh + 0x394), vec![forms[2]]);
        let loaded_spoken = e.mem.u32(fresh + 0x264);
        assert_eq!(list_items(&e, loaded_spoken), vec![forms[3], forms[4]]);
        assert_eq!(e.mem.u32(e.mem.u32(fresh + 0x368)), 0x7001);
        let loaded_avoid = e.mem.u32(fresh + 0x44c);
        let avoid_items = list_items(&e, loaded_avoid);
        assert_eq!(e.mem.u32(avoid_items[0]), 0x7002);
        let loaded_detected = e.mem.u32(fresh + 0x25c);
        let detected_items = list_items(&e, loaded_detected);
        assert_eq!(e.mem.u32(detected_items[0]), 0x7003);
        let loaded_who = e.mem.u32(fresh + 0x260);
        let who_items = list_items(&e, loaded_who);
        assert_eq!(e.mem.u32(who_items[0]), 0x7004);
        assert_eq!(e.mem.u32(e.mem.u32(fresh + 0x3dc)), 0x7005);
        assert_eq!(calls_to(&e, 0x0084_a810), vec![vec![0x5555, 1, buffer, 0]]);
    }

    #[test]
    fn test_high_process_load_game_ov2_old_version_discards_a_field() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let (buffer, stream) = buffer_doubles(&mut e, 7);
        let this = e.mem.alloc(0x500);
        for offset in [0x264u32, 0x25c, 0x260] {
            let head = e.mem.alloc(8);
            e.mem.set_u32(this + offset, head);
        }
        // Version 7 has a 4-byte field after +0x2fc, and none of the newer
        // ones: bytes 0.. are 1, 2, 3, 4 (flags), 2 bytes, then the dropped
        // four bytes, then the first float word.
        let bytes = [
            1u8, 2, 3, 4, 5, 6, 0xde, 0xad, 0xbe, 0xef, 0x11, 0x22, 0x33, 0x44,
        ];
        e.mem.write(stream, &bytes);
        e.call(0x008f_ce90, &args![this, buffer]);
        assert_eq!(e.mem.u8(this + 0x32c), 1);
        assert_eq!(e.mem.u8(this + 0x340), 2);
        assert_eq!(e.mem.u8(this + 0x374), 3);
        assert_eq!(e.mem.u8(this + 0x375), 4);
        assert_eq!(e.mem.u16(this + 0x2fc), 0x0605);
        assert_eq!(e.mem.u32(this + 0x2b4), 0x4433_2211);
    }

    /// Doubles for the game-setting accessors: `00403e20(setting)` gives the
    /// address of a float, `0043d4d0(setting)` the address of an int.
    fn float_settings(e: &mut Engine, settings: &[(u32, f32)]) {
        let table: Vec<(u32, u32)> = settings
            .iter()
            .map(|(addr, value)| {
                let cell = e.mem.alloc(4);
                e.mem.set_f32(cell, *value);
                (*addr, cell)
            })
            .collect();
        e.register_double(0x0040_3e20, move |_, a| {
            ret(table
                .iter()
                .find(|(addr, _)| *addr == a[0])
                .map_or(0, |(_, cell)| *cell))
        });
    }

    fn int_settings(e: &mut Engine, settings: &[(u32, i32)]) {
        let table: Vec<(u32, u32)> = settings
            .iter()
            .map(|(addr, value)| {
                let cell = e.mem.alloc(4);
                e.mem.set_i32(cell, *value);
                (*addr, cell)
            })
            .collect();
        e.register_double(0x0043_d4d0, move |_, a| {
            ret(table
                .iter()
                .find(|(addr, _)| *addr == a[0])
                .map_or(0, |(_, cell)| *cell))
        });
    }

    /// Maps the table of procedure kinds so that `package kind -> row`.
    fn procedure_table(e: &mut Engine, kind: u32, selectors: &[u32]) {
        e.map(0x011a_3ff0, 0x100);
        let row = e.mem.alloc(4 * selectors.len() as u32 + 4);
        for (index, selector) in selectors.iter().enumerate() {
            e.mem.set_u32(row + 4 * index as u32, *selector);
        }
        e.mem.set_u32(0x011a_3ff0 + 4 * kind, row);
    }

    /// A scene for `Update_ov2`: every double the common path needs.
    struct UpdateScene {
        process: u32,
        actor: u32,
        package: u32,
        actor_process: u32,
    }

    fn update_scene(e: &mut Engine, selectors: &[u32]) -> UpdateScene {
        list_doubles(e);
        global(e, 0x011d_ea3c, 0x0abc_0000);
        global(e, 0x011d_e7b8, 0);
        for addr in [
            0x0045_3a70u32,
            0x00ad_8780,
            0x0095_3ce0,
            0x0090_1550,
            0x0055_b980,
            0x008b_01c0,
            0x00aa_b200,
            0x00aa_b210,
        ] {
            stub(e, addr, 0);
        }
        stub(e, 0x0056_53d0, 1);
        stub(e, 0x0096_11e0, 2);
        e.map(0x0101_2060, 8);
        global(e, 0x0101_2054, (-1.0f32).to_bits());
        procedure_table(e, 2, selectors);
        let package_vtable = vtable(e, &[(0x13c, 0x00aa_b220)]);
        stub(e, 0x00aa_b220, 1);
        let package = object(e, 0x100, package_vtable);
        e.register_double(0x00aa_b230, move |_, _| ret(package));
        stub(e, 0x00aa_b240, 1);
        let actor_process_vtable = vtable(e, &[(0x28, 0x00aa_b210)]);
        let actor_process = object(e, 0x40, actor_process_vtable);
        e.register_double(0x008d_8520, move |_, _| ret(actor_process));
        let process = process(
            e,
            &[
                (0x27c, 0x00aa_b230),
                (0x280, 0x00aa_b240),
                (0x22c, 0x00aa_b200),
                (0x20c, 0x00aa_b240),
                (0x514, 0x00aa_b280),
                (0x24, 0x00aa_b200),
                (0x82c, 0x00aa_b210),
                (0x4b4, 0x00aa_b210),
                (0x288, 0x00aa_b210),
                (0x2ac, 0x00aa_b210),
                (0x354, 0x00aa_b210),
                (0x74, 0x00aa_b210),
                (0x7c, 0x00aa_b210),
                (0x80, 0x00aa_b210),
            ],
        );
        let actor_vtable = vtable(
            e,
            &[
                (0x100, 0x00aa_b240),
                (0x2e8, 0x00aa_b200),
                (0x214, 0x00aa_b200),
                (0x28c, 0x00aa_b210),
                (0x2c4, 0x00aa_b210),
            ],
        );
        let actor = object(e, 0x100, actor_vtable);
        stub(e, 0x00aa_b280, 0);
        // The process type is not 0, so the tail only runs the final slot.
        stub(e, 0x0093_1850, 1);
        UpdateScene {
            process,
            actor,
            package,
            actor_process,
        }
    }

    #[test]
    fn test_high_process_update_ov2_runs_the_selected_step() {
        let mut e = Engine::new();
        // Procedure index 1 selects kind 1 (`008f36c0`).
        let scene = update_scene(&mut e, &[0, 1]);
        stub(&mut e, 0x008f_36c0, 0);
        e.mem.set_u8(scene.process + 0x32c, 1);
        e.mem.set_f32(scene.process + 0x330, 4.0);
        e.call_log = Some(vec![]);
        e.call(0x008f_4020, &args![scene.process, scene.actor, 0.25f32]);
        assert_eq!(
            calls_to(&e, 0x008f_36c0),
            vec![vec![scene.process, scene.actor, 0.25f32.to_bits()]]
        );
        // The greeting flag and timer are reset and the actor's process updated.
        assert_eq!(e.mem.u8(scene.process + 0x32c), 0);
        assert_eq!(e.mem.f32(scene.process + 0x330), 0.0);
        assert_eq!(
            calls_to(&e, 0x00aa_b210).last().unwrap()[0],
            scene.actor_process
        );
        // A null actor does nothing at all.
        e.call_log = Some(vec![]);
        e.call(0x008f_4020, &args![scene.process, 0u32, 0.25f32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_high_process_update_ov2_dying_actor() {
        let mut e = Engine::new();
        let scene = update_scene(&mut e, &[0, 0xc]);
        let dying_vtable = vtable(
            &mut e,
            &[
                (0x100, 0x00aa_b240),
                (0x2e8, 0x00aa_b240),
                (0x214, 0x00aa_b250),
                (0x28c, 0x00aa_b210),
            ],
        );
        stub(&mut e, 0x00aa_b250, 0);
        e.mem.set_u32(scene.actor, dying_vtable);
        e.call_log = Some(vec![]);
        e.call(0x008f_4020, &args![scene.process, scene.actor, 0.0f32]);
        assert_eq!(calls_to(&e, 0x008b_01c0), vec![vec![scene.actor]]);
        // Kind 0xc ends the loop without a call; FinishDying (slot 0x354) ran.
        assert!(calls_to(&e, 0x00aa_b210)
            .iter()
            .any(|words| words.len() == 2 && words[1] == scene.actor));
    }

    #[test]
    fn test_high_process_update_ov2_case_0_package_done() {
        let mut e = Engine::new();
        let scene = update_scene(&mut e, &[0, 0]);
        global(&mut e, 0x011c_a248, 0x1234);
        // The package test passes: the procedure index advances, loop ends.
        stub(&mut e, 0x007a_f430, 0);
        let location = e.mem.alloc(0x40);
        e.register_double(0x00aa_b280, move |_, _| ret(location));
        e.call_log = Some(vec![]);
        e.call(0x008f_4020, &args![scene.process, scene.actor, 0.0f32]);
        let advanced = calls_to(&e, 0x00aa_b210)
            .iter()
            .filter(|words| words.len() == 3 && words[1] == scene.actor && words[2] == 1)
            .count();
        assert_eq!(advanced, 1);
        // The reference of the generic location was asked for.
        assert_eq!(calls_to(&e, 0x00aa_b280).len(), 1);
    }

    #[test]
    fn test_high_process_update_ov2_tail_ends_the_procedure() {
        let mut e = Engine::new();
        let scene = update_scene(&mut e, &[0, 0x36]);
        // Not a persistent reference: the loop is skipped. The process type is
        // 0 and the running procedure is of kind 0x36: the package test fails.
        stub(&mut e, 0x0056_53d0, 0);
        stub(&mut e, 0x0093_1850, 0);
        stub(&mut e, 0x0096_11e0, 0);
        procedure_table(&mut e, 0, &[0, 0x36]);
        let package_vtable = e.mem.u32(scene.package);
        stub(&mut e, 0x00aa_b270, 0);
        e.mem.set_u32(package_vtable + 0x13c, 0x00aa_b270);
        e.mem.set_u32(scene.process + 0x378, 7);
        e.call_log = Some(vec![]);
        e.call(0x008f_4020, &args![scene.process, scene.actor, 0.0f32]);
        // The function returned through the "end the procedure" branch: the
        // index was moved by -1 and the final slot was not called.
        assert!(calls_to(&e, 0x00aa_b210)
            .iter()
            .any(|words| words.len() == 3 && words[2] == 0xffff_ffff));
        assert!(calls_to(&e, 0x00aa_b210)
            .iter()
            .all(|words| words[0] != scene.actor_process));
    }

    #[test]
    fn test_high_process_evaluate_detection() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let player = e.mem.alloc(0x100);
        global(&mut e, 0x011d_ea3c, player);
        global(&mut e, 0x011f_1221, 1);
        e.map(0x011e_0280, 8);
        e.map(0x011f_1958, 4);
        stub(&mut e, 0x0088_1510, 0);
        stub(&mut e, 0x0041_81e0, 0);
        global(&mut e, 0x0101_2054, (-1.0f32).to_bits());
        stub_float(&mut e, 0x008a_cbe0, 1.0);
        float_settings(
            &mut e,
            &[(0x011c_d7d8, 5.0), (0x011c_d45c, 0.0), (0x011c_df10, 10.0)],
        );
        int_settings(&mut e, &[(0x011c_f414, 3)]);
        // The owner sub-object at +0xa4 of the actor: slot 8 gives 0, slot 0xc 0.5.
        stub(&mut e, 0x00aa_b300, 0);
        e.register_double(0x00aa_b310, |_, _| ret_float(0.5));
        let owner_vtable = vtable(&mut e, &[(8, 0x00aa_b300), (0xc, 0x00aa_b310)]);
        let actor_vtable = vtable(
            &mut e,
            &[
                (0x22c, 0x00aa_b300),
                (0x3f8, 0x00aa_b300),
                (0x304, 0x00aa_b300),
            ],
        );
        let actor = object(&mut e, 0x100, actor_vtable);
        e.mem.set_u32(actor + 0xa4, owner_vtable);
        let player_vtable = vtable(
            &mut e,
            &[
                (0x22c, 0x00aa_b300),
                (0x3f8, 0x00aa_b300),
                (0x448, 0x00aa_b300),
                (0x2e8, 0x00aa_b300),
            ],
        );
        e.mem.set_u32(player, player_vtable);
        // The player's detection state of the actor: level 50, not evaluated.
        let state = detection_state_for(&mut e, player, 50);
        e.mem.set_u8(state + 0x1f, 0);
        e.register_double(0x00aa_b320, move |_, _| ret(state));
        stub(&mut e, 0x00aa_b330, 0);
        let process = process(
            &mut e,
            &[
                (0x504, 0x00aa_b320),
                (0x27c, 0x00aa_b300),
                (0x6c, 0x00aa_b330),
            ],
        );
        // GetShouldAttackActor says yes: the alert level becomes 100.
        e.register_double(0x008b_06d0, |e, a| {
            e.mem.set_u32(a[3], 2);
            ret(1)
        });
        stub(&mut e, 0x008a_c6f0, 0);
        stub(&mut e, 0x0095_3c20, 0);
        stub(&mut e, 0x0047_d740, 0);
        stub(&mut e, 0x0095_3c50, 0);
        stub(&mut e, 0x008d_6f30, 0);
        stub(&mut e, 0x0071_7e50, 0x0ddd_0000);
        stub(&mut e, 0x005b_e5c0, 0);
        let heads = [0x274u32, 0x27c, 0x38c, 0x394];
        for offset in heads {
            // Inline list heads: node 0 is part of the process itself.
            e.mem.set_u32(process + offset, 0);
        }
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x130);
        let detected = e.call(0x008f_5480, &args![process, actor]).bool();
        // The 0x130-byte frame: the aggressor record word at EBP-0x8c (+0xa4).
        assert!(calls_to(&e, LIST_APPEND).contains(&vec![process + 0x274, frame + 0xa4]));
        assert!(!detected);
        // The player was queued as an aggressor (+0x274) with the alarm byte.
        let aggressors = list_items(&e, process + 0x274);
        assert_eq!(aggressors.len(), 1);
        assert_eq!(e.mem.u32(aggressors[0]), player);
        assert_eq!(e.mem.u8(aggressors[0] + 9), 1);
        // The state was marked evaluated and the process told to finish.
        assert_eq!(e.mem.u8(state + 0x1f), 1);
        assert_eq!(calls_to(&e, 0x00aa_b330), vec![vec![process, actor]]);
        assert_eq!(e.mem.u32(process + 0x2a4), 0);
        // An actor that is already handled stops at once.
        stub(&mut e, 0x00aa_b300, 1);
        assert!(!e.call(0x008f_5480, &args![process, actor]).bool());
    }

    /// A scene shared by the `ProcessUseItemAt` and `ProcessUseWeapon` tests.
    struct UseScene {
        process: u32,
        actor: u32,
        package: u32,
        target: u32,
        data: u32,
        /// The result of the package's slot `0x13c` test.
        package_ok: std::rc::Rc<std::cell::Cell<u32>>,
        /// The process's `GetWeaponDrawn` answer.
        drawn: std::rc::Rc<std::cell::Cell<u32>>,
        /// The distance the player is away.
        distance: std::rc::Rc<std::cell::Cell<f64>>,
        /// `Animation::SpecialIdleDonePlaying`.
        idle_done: std::rc::Rc<std::cell::Cell<u32>>,
    }

    fn use_scene(e: &mut Engine) -> UseScene {
        use std::cell::Cell;
        use std::rc::Rc;
        list_doubles(e);
        global(e, 0x011d_ea3c, 0x0abc_0000);
        global(e, 0x011c_a248, 0x1234);
        global(e, 0x0101_2054, (-1.0f32).to_bits());
        global(e, 0x0103_0ff0, 0.8f32.to_bits());
        e.map(0x0101_2060, 8);
        float_settings(e, &[(0x011c_d674, 100.0), (0x011c_dd3c, 100.0)]);
        let distance = Rc::new(Cell::new(10.0));
        let distance_source = distance.clone();
        e.register_double(0x0057_23b0, move |_, _| ret_float(distance_source.get()));
        let idle_done = Rc::new(Cell::new(1));
        let idle_source = idle_done.clone();
        e.register_double(0x0049_85f0, move |_, _| ret(idle_source.get()));
        e.register_double(0x0048_7f50, |_, _| ret(2500));
        for addr in [
            0x0055_b980u32,
            0x004b_f220,
            0x0067_f390,
            0x0049_38e0,
            0x0067_0f90,
            0x0067_4d20,
            0x0067_2dd0,
            0x008a_6840,
            0x0092_bf20,
            0x0060_0900,
            0x0088_c650,
            0x0088_c790,
            0x008b_3ab0,
            0x008b_3d30,
            0x0093_1d90,
            0x008b_b650,
            0x005c_e9d0,
            0x008b_b5c0,
            0x007a_f430,
            0x0040_6d00,
            0x005b_5e40,
            0x0089_4d60,
            0x0089_4cc0,
            0x0097_fd80,
            0x0049_97b0,
            0x005a_2030,
            0x0056_9b80,
            0x006e_cd40,
            0x0083_3c20,
            0x0067_a4f0,
        ] {
            stub(e, addr, 0);
        }
        stub(e, 0x008b_3bb0, 1);
        stub(e, 0x0089_35f0, 1);
        stub(e, 0x005f_2540, 1);
        stub(e, 0x0089_7910, 0x77);
        let target = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x40);
        let weapon = 0x6200u32;
        stub(e, 0x0067_1d10, target);
        stub(e, 0x0067_58f0, data);
        stub(e, 0x0051_9b00, 1);
        stub(e, 0x0068_0050, weapon);
        stub(e, 0x0067_9ba0, 12);
        stub(e, 0x0044_ddc0, weapon);
        stub(e, 0x0040_1170, 0x28);
        stub(e, 0x0082_4060, 1);
        stub(e, 0x008f_21d0, 3);
        stub(e, 0x004c_0bf0, 1);
        stub(e, 0x0044_6390, 3);
        stub(e, 0x0090_37e0, 0);
        let package_ok = Rc::new(Cell::new(1));
        let package_source = package_ok.clone();
        e.register_double(0x00aa_b400, move |_, _| ret(package_source.get()));
        stub(e, 0x00aa_b410, 0x7700);
        stub(e, 0x00aa_b420, 1);
        let package_vtable = vtable(
            e,
            &[
                (0x13c, 0x00aa_b400),
                (0x130, 0x00aa_b410),
                (0x140, 0x00aa_b420),
            ],
        );
        let package = object(e, 0x100, package_vtable);
        let drawn = Rc::new(Cell::new(0));
        let drawn_source = drawn.clone();
        e.register_double(0x00aa_b430, move |_, _| ret(drawn_source.get()));
        e.register_double(0x00aa_b440, move |_, _| ret(package));
        stub(e, 0x00aa_b450, 0);
        stub(e, 0x00aa_b460, 0x6100);
        stub(e, 0x00aa_b470, 1);
        stub(e, 0x00aa_b480, 0xffff_ffff);
        let process = process(
            e,
            &[
                (0x27c, 0x00aa_b440),
                (0x8c, 0x00aa_b450),
                (0x274, 0x00aa_b450),
                (0x148, 0x00aa_b460),
                (0x80c, 0x00aa_b470),
                (0x288, 0x00aa_b450),
                (0x14c, 0x00aa_b450),
                (0x618, 0x00aa_b450),
                (0x638, 0x00aa_b450),
                (0x454, 0x00aa_b430),
                (0x44c, 0x00aa_b450),
                (0x3e4, 0x00aa_b480),
                (0x1b0, 0x00aa_b450),
                (0x394, 0x00aa_b450),
                (0x434, 0x00aa_b450),
                (0x294, 0x00aa_b450),
                (0x654, 0x00aa_b450),
                (0x278, 0x00aa_b450),
                (0x628, 0x00aa_b450),
                (0x644, 0x00aa_b450),
                (0x12c, 0x00aa_b450),
                (0x20c, 0x00aa_b470),
                (0x44, 0x00aa_b450),
                (0x338, 0x00aa_b450),
                (0x1d0, 0x00aa_b450),
                (0x7c8, 0x00aa_b450),
                (0x688, 0x00aa_b450),
            ],
        );
        stub(e, 0x00aa_b490, 0x3000);
        let actor_vtable = vtable(
            e,
            &[
                (0x1e4, 0x00aa_b490),
                (0x100, 0x00aa_b470),
                (0x22c, 0x00aa_b450),
                (0x214, 0x00aa_b450),
                (0x2bc, 0x00aa_b450),
                (0x1f4, 0x00aa_b450),
                (0x2c4, 0x00aa_b450),
            ],
        );
        let actor = object(e, 0x100, actor_vtable);
        // Three shots are still to be fired.
        e.mem.set_i16(process + 0x2c2, 3);
        UseScene {
            process,
            actor,
            package,
            target,
            data,
            package_ok,
            drawn,
            distance,
            idle_done,
        }
    }

    #[test]
    fn test_high_process_process_use_item_at_far_actor_does_nothing() {
        let mut e = Engine::new();
        let scene = use_scene(&mut e);
        scene.distance.set(500.0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x008f_9320, &args![scene.process, scene.actor])
            .bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn test_high_process_process_use_item_at_failed_package_ends_the_procedure() {
        let mut e = Engine::new();
        let scene = use_scene(&mut e);
        scene.package_ok.set(0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x008f_9320, &args![scene.process, scene.actor])
            .bool());
        assert_eq!(calls_to(&e, 0x00aa_b450).len(), 1);
        assert_eq!(
            calls_to(&e, 0x00aa_b450)[0],
            vec![scene.process, scene.actor, 0xffff_ffff]
        );
    }

    #[test]
    fn test_high_process_process_use_item_at_uses_the_item() {
        let mut e = Engine::new();
        let scene = use_scene(&mut e);
        // No package target: the object type is 0 and the default arm uses
        // the special idle.
        stub(&mut e, 0x0067_1d10, 0);
        e.map(0x0101_6978, 8);
        e.mem.set_f64(0x0101_6978, 0.001);
        e.map(0x0102_0998, 8);
        e.mem.set_f64(0x0102_0998, 5.0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x008f_9320, &args![scene.process, scene.actor])
            .bool());
        // SetupSpecialIdle (slot 0x44), SetIdleTimer(5 + 2500 * 0.001).
        let calls = calls_to(&e, 0x00aa_b450);
        assert!(calls.contains(&vec![scene.process, scene.actor, 0, 2, 1, 0, 1]));
        assert!(calls.contains(&vec![scene.process, 7.5f32.to_bits()]));
        assert_eq!(e.mem.u16(scene.process + 0x2c0), 1);
    }

    #[test]
    fn test_high_process_process_use_item_at_reports_a_weapon_target() {
        let mut e = Engine::new();
        let scene = use_scene(&mut e);
        // The package target is an object type of 12 (a weapon): an error.
        stub(&mut e, 0x0051_9b00, 2);
        stub(&mut e, 0x0068_0080, 12);
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x1cc);
        assert!(!e
            .call(0x008f_9320, &args![scene.process, scene.actor])
            .bool());
        // The 0x1cc-byte frame: the message at EBP-0x150 (+0x7c).
        assert_eq!(calls_to(&e, 0x0040_6d00)[0][0], frame + 0x7c);
        assert_eq!(e.mem.u32(scene.process + 0x108), 12);
        assert_eq!(calls_to(&e, 0x0040_6d00).len(), 1);
        assert_eq!(calls_to(&e, 0x0040_6d00)[0][2], 0x0108_8318);
        assert_eq!(calls_to(&e, 0x005b_5e40).len(), 1);
    }

    #[test]
    fn test_high_process_process_use_weapon_far_actor_does_nothing() {
        let mut e = Engine::new();
        let scene = use_scene(&mut e);
        scene.distance.set(500.0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x008f_7730, &args![scene.process, scene.actor])
            .bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn test_high_process_process_use_weapon_failed_package_cleans_up() {
        let mut e = Engine::new();
        let scene = use_scene(&mut e);
        scene.package_ok.set(0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x008f_7730, &args![scene.process, scene.actor])
            .bool());
        // The clean-up ran (it looked at the use-weapon data) and the
        // procedure index moved by -1.
        assert_eq!(calls_to(&e, 0x008b_3d30), vec![vec![scene.actor]]);
        assert!(calls_to(&e, 0x00aa_b450).contains(&vec![scene.process, scene.actor, 0xffff_ffff]));
    }

    #[test]
    fn test_high_process_process_use_weapon_reports_a_non_weapon_target() {
        let mut e = Engine::new();
        let scene = use_scene(&mut e);
        // No package target: the object type is 0, which is not valid.
        stub(&mut e, 0x0067_1d10, 0);
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x374);
        assert!(!e
            .call(0x008f_7730, &args![scene.process, scene.actor])
            .bool());
        // The 0x374-byte frame: the message at EBP-0x248 (+0x12c).
        assert_eq!(calls_to(&e, 0x0040_6d00)[0][0], frame + 0x12c);
        assert_eq!(calls_to(&e, 0x0040_6d00).len(), 1);
        assert_eq!(calls_to(&e, 0x0040_6d00)[0][2], 0x0108_8260);
    }

    #[test]
    fn test_high_process_process_use_weapon_equips_the_package_weapon() {
        let mut e = Engine::new();
        let scene = use_scene(&mut e);
        // The package wants another weapon than the one in hand.
        stub(&mut e, 0x0068_0050, 0x6300);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x008f_7730, &args![scene.process, scene.actor])
            .bool());
        assert_eq!(
            calls_to(&e, 0x0088_c650),
            vec![vec![scene.actor, 0x6300, 1, 0, 1, 0, 1]]
        );
    }

    #[test]
    fn test_high_process_process_use_weapon_draws_the_weapon_first() {
        let mut e = Engine::new();
        let scene = use_scene(&mut e);
        scene.drawn.set(0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x008f_7730, &args![scene.process, scene.actor])
            .bool());
        assert_eq!(calls_to(&e, 0x008a_6840), vec![vec![scene.actor, 1]]);
        assert_eq!(e.mem.u8(scene.process + 0x349), 1);
        let _ = (scene.target, scene.data, scene.package, scene.idle_done);
    }

    #[test]
    fn test_high_process_process_use_weapon_fires_a_high_level_weapon() {
        let mut e = Engine::new();
        let scene = use_scene(&mut e);
        scene.drawn.set(1);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x008f_7730, &args![scene.process, scene.actor])
            .bool());
        assert_eq!(
            calls_to(&e, 0x0090_37e0),
            vec![vec![scene.process, scene.actor, 0x6200]]
        );
        assert!(calls_to(&e, 0x0089_35f0).is_empty());
    }

    #[test]
    fn test_high_process_process_use_weapon_picks_an_attack_by_weight() {
        let mut e = Engine::new();
        let scene = use_scene(&mut e);
        scene.drawn.set(1);
        // A low-level weapon: the attack animation group is chosen by weight.
        stub(&mut e, 0x0044_6390, 2);
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x374);
        assert!(!e
            .call(0x008f_7730, &args![scene.process, scene.actor])
            .bool());
        // The 0x374-byte frame: the attack data at EBP-0x30c (+0x68).
        assert_eq!(calls_to(&e, 0x0097_fd80)[0][2], frame + 0x68);
        // The random roll (2500) % 100 = 0 and 2500 % 100 = 0 picks the first
        // usable group... the weights are 10, 10, 10, 10, 30, 30 and the
        // remainder 2500 % 100 = 0 ends on the very first entry.
        assert!(calls_to(&e, 0x0090_37e0).is_empty());
        assert_eq!(calls_to(&e, 0x0089_35f0), vec![vec![scene.actor, 0x5e]]);
        assert_eq!(e.mem.f32(scene.process + 0x2b8), 0.8);
    }

    // ---- Tests for the second half of the part (008fdb90 to 00900f00) --------------------

    /// Maps `addr` and stores a `double` there.
    fn global_f64(e: &mut Engine, addr: u32, value: f64) {
        e.map(addr, 8);
        e.mem.set_f64(addr, value);
    }

    /// An object of `size` bytes whose virtual slot `offset` is a double at
    /// `target` returning `eax`, for each `(offset, target, eax)`.
    fn slots_object(e: &mut Engine, size: u32, slots: &[(u32, u32, u32)]) -> u32 {
        let pairs: Vec<(u32, u32)> = slots
            .iter()
            .map(|(offset, target, _)| (*offset, *target))
            .collect();
        for (_, target, eax) in slots {
            stub(e, *target, *eax);
        }
        let table = vtable(e, &pairs);
        object(e, size, table)
    }

    /// The logged calls to any of `addrs`, in order.
    fn calls_in_order(e: &Engine, addrs: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .expect("the call log is on")
            .iter()
            .filter(|(called, _)| addrs.contains(called))
            .cloned()
            .collect()
    }

    /// A `BSSimpleList` whose head node sits inline at `head`: the first item
    /// in the head, the rest in heap nodes.
    fn inline_list(e: &mut Engine, head: u32, items: &[u32]) {
        if let Some((first, rest)) = items.split_first() {
            e.mem.set_u32(head, *first);
            let next = if rest.is_empty() {
                0
            } else {
                make_list(e, rest)
            };
            e.mem.set_u32(head + 4, next);
        }
    }

    /// The `double` constants 0.0 and 1.0 the fade code compares with.
    fn fade_constants(e: &mut Engine) {
        global_f64(e, 0x0101_2060, 0.0);
        global_f64(e, 0x0101_2070, 1.0);
    }

    #[test]
    fn test_fn_008fdb90() {
        let mut e = Engine::new();
        stub(&mut e, 0x0069_2710, 0);
        global(&mut e, 0x0101_6970, f32::MAX.to_bits());
        let entry = e.mem.alloc(0x40);
        for offset in [0x24, 0x28, 0x2c, 0x30] {
            e.mem.set_u32(entry + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        let r = e.call(0x008f_db90, &args![entry]);
        assert_eq!(r.u32(), entry);
        assert_eq!(calls_to(&e, 0x0069_2710), vec![vec![entry]]);
        assert_eq!(e.mem.f32(entry + 0x24), f32::MAX);
        assert_eq!(e.mem.u32(entry + 0x28), 0);
        assert_eq!(e.mem.f32(entry + 0x2c), 0.0);
        assert_eq!(e.mem.u32(entry + 0x30), 0);
    }

    /// A process, a save buffer of `version` and an actor for
    /// `InitLoadGame_ov2`. Form IDs below 0x66 are resolvable (they cast to
    /// themselves), 0x66 is not.
    fn init_scene(e: &mut Engine, version: u32) -> (u32, u32, u32) {
        list_doubles(e);
        for addr in [
            0x0092_78d0,
            0x0056_4db0,
            0x0083_d0d0,
            0x008d_74a0,
            0x008d_7220,
            0x008d_7340,
            0x008d_6f10,
            0x008e_5730,
            0x008c_6eb0,
        ] {
            stub(e, addr, 0);
        }
        e.register_double(0x0048_39c0, |_, a| ret(a[0]));
        e.register_double(0x00ec_43fb, |_, a| ret(if a[0] == 0x66 { 0 } else { a[0] }));
        e.register_double(0x0072_6c60, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            ret(0)
        });
        global(e, 0x011d_ea3c, 0x5000);
        let node_3d = e.mem.alloc(8);
        let actor = slots_object(e, 0x40, &[(0x1d0, 0x00aa_c000, node_3d)]);
        let buffer = slots_object(
            e,
            8,
            &[(0, 0x00aa_c010, version), (0xc, 0x00aa_c020, actor)],
        );
        let this = e.mem.alloc(0x500);
        (this, buffer, actor)
    }

    #[test]
    fn test_high_process_init_load_game_ov2_resolves_everything() {
        let mut e = Engine::new();
        let (this, buffer, actor) = init_scene(&mut e, 0x12);
        e.mem.set_u32(this + 0x30c, 0x11);
        e.mem.set_u32(this + 0x2a4, 0x66);
        e.mem.set_u32(this + 0x3f0, 0x33);
        e.mem.set_u32(this + 0x41c, 0x44);
        e.mem.set_u32(this + 0x370, 0x55);
        e.mem.set_u32(this + 0x2ac, 0x22);
        for (index, id) in [0x61u32, 0, 0x66, 0, 0, 0x62].into_iter().enumerate() {
            e.mem.set_u32(this + 0x3f8 + 4 * index as u32, id);
        }
        inline_list(&mut e, this + 0x38c, &[0x71, 0x66, 0x72]);
        let spoke_to = make_list(&mut e, &[0x81, 0x66]);
        e.mem.set_u32(this + 0x264, spoke_to);
        e.mem.set_u32(this + 0x368, 0x7000);
        let avoid = make_list(&mut e, &[0x9100]);
        e.mem.set_u32(this + 0x44c, avoid);
        let good = detection_state_for(&mut e, 0x77, 0);
        let bad = detection_state_for(&mut e, 0, 0);
        let detected = make_list(&mut e, &[bad, good]);
        e.mem.set_u32(this + 0x25c, detected);
        let other_bad = detection_state_for(&mut e, 0, 0);
        let who_detects = make_list(&mut e, &[other_bad]);
        e.mem.set_u32(this + 0x260, who_detects);
        e.mem.set_u32(this + 0x3dc, 0x6000);
        e.mem.set_u32(this + 0x2e4, 1);
        let cache = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x428, cache);
        e.call_log = Some(vec![]);
        e.call(0x008f_dbd0, &args![this, buffer]);
        assert_eq!(calls_to(&e, 0x0092_78d0), vec![vec![this, buffer]]);
        assert_eq!(e.mem.u32(this + 0x30c), 0x11);
        assert_eq!(e.mem.u32(this + 0x2a4), 0);
        assert_eq!(e.mem.u32(this + 0x3f0), 0x33);
        assert_eq!(e.mem.u32(this + 0x41c), 0x44);
        assert_eq!(e.mem.u32(this + 0x370), 0x55);
        assert_eq!(e.mem.u32(this + 0x2ac), 0x22);
        assert_eq!(e.mem.u32(this + 0x3f8 + 8), 0);
        // Resolved references are marked targeted: the path look-at target,
        // then the head-tracking targets in order.
        assert_eq!(
            calls_to(&e, 0x0056_4db0),
            vec![vec![0x22, 1], vec![0x61, 1], vec![0x62, 1]]
        );
        // The unresolvable entries leave the actor lists.
        assert_eq!(list_items(&e, this + 0x38c), vec![0x71, 0x72]);
        assert_eq!(list_items(&e, spoke_to), vec![0x81]);
        assert_eq!(calls_to(&e, 0x0083_d0d0), vec![vec![0x7000, buffer]]);
        assert_eq!(calls_to(&e, 0x008d_74a0), vec![vec![0x9100, buffer]]);
        assert_eq!(calls_to(&e, 0x008d_7340), vec![vec![0x6000, buffer]]);
        // Detection states without an actor are removed and freed.
        assert_eq!(list_items(&e, detected), vec![good]);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![bad], vec![other_bad]]);
        assert!(list_items(&e, who_detects).is_empty());
        // The bone LOD refresh and the cache reset.
        let refresh = calls_to(&e, 0x008d_6f10);
        assert_eq!(refresh.len(), 1);
        assert_ne!(refresh[0][0], 0);
        assert_eq!(refresh[0][1], 0);
        assert_eq!(calls_to(&e, 0x008e_5730), vec![vec![this, actor]]);
        assert_eq!(calls_to(&e, 0x008c_6eb0), vec![vec![cache]]);
    }

    #[test]
    fn test_high_process_init_load_game_ov2_old_save_and_player_actor() {
        let mut e = Engine::new();
        let (this, buffer, _) = init_scene(&mut e, 0x0d);
        e.mem.set_u32(this + 0x2ac, 0x22);
        inline_list(&mut e, this + 0x38c, &[]);
        let spoke_to = make_list(&mut e, &[0x66]);
        e.mem.set_u32(this + 0x264, spoke_to);
        e.mem.set_u32(this + 0x2e4, 1);
        // The actor is the player: no 3D refresh.
        let player_buffer = slots_object(
            &mut e,
            8,
            &[(0, 0x00aa_c030, 0x0d), (0xc, 0x00aa_c040, 0x5000)],
        );
        e.call_log = Some(vec![]);
        e.call(0x008f_dbd0, &args![this, player_buffer]);
        let _ = buffer;
        // Versions up to 0xd keep the path look-at target and the spoke-to list.
        assert_eq!(e.mem.u32(this + 0x2ac), 0x22);
        assert!(calls_to(&e, 0x0056_4db0).is_empty());
        assert_eq!(list_items(&e, spoke_to), vec![0x66]);
        assert!(calls_to(&e, 0x008d_6f10).is_empty());
        assert!(calls_to(&e, 0x008e_5730).is_empty());
        assert!(calls_to(&e, 0x008c_6eb0).is_empty());
    }

    #[test]
    fn test_high_process_revert_ov2() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        for addr in [
            0x0092_80f0,
            0x008c_6eb0,
            0x004d_5850,
            0x005c_90d0,
            0x0090_4160,
        ] {
            stub(&mut e, addr, 0);
        }
        global(&mut e, 0x0101_7868, 3.5f32.to_bits());
        global(&mut e, 0x0101_2054, (-1.0f32).to_bits());
        float_settings(&mut e, &[(0x011c_dcd8, 2.5)]);
        let process = process(&mut e, &[(0x6c4, 0x00aa_c050)]);
        stub(&mut e, 0x00aa_c050, 0);
        let cache = e.mem.alloc(0x10);
        e.mem.set_u32(process + 0x428, cache);
        e.mem.set_u32(process + 0x3cc, 0x1111);
        e.mem.set_u32(process + 0x368, 0x2222);
        inline_list(&mut e, process + 0x38c, &[0x41]);
        inline_list(&mut e, process + 0x394, &[0x42]);
        let spoke_to = make_list(&mut e, &[0x43]);
        e.mem.set_u32(process + 0x264, spoke_to);
        let detected = make_list(&mut e, &[0xd1, 0xd2]);
        e.mem.set_u32(process + 0x25c, detected);
        let who = make_list(&mut e, &[0xd3]);
        e.mem.set_u32(process + 0x260, who);
        e.mem.set_u32(process + 0x3dc, 0x4444);
        e.mem.set_u32(process + 0x3e4, 0x5555);
        e.mem.set_u32(process + 0x3f8, 0x61);
        e.mem.set_u8(process + 0x410, 1);
        e.mem.set_u32(process + 0x41c, 0x62);
        e.mem.set_u32(process + 0x3ac, 7);
        e.mem.set_f32(process + 0x384, 9.0);
        e.call_log = Some(vec![]);
        let buffer = 0x1234u32;
        e.call(0x008f_e420, &args![process, buffer]);
        assert_eq!(calls_to(&e, 0x0092_80f0), vec![vec![process, buffer]]);
        assert_eq!(calls_to(&e, 0x008c_6eb0), vec![vec![cache]]);
        assert_eq!(calls_to(&e, 0x004d_5850), vec![vec![0x1111, 1]]);
        assert_eq!(calls_to(&e, 0x005c_90d0), vec![vec![0x2222, 1]]);
        assert_eq!(calls_to(&e, 0x0090_4160), vec![vec![process]]);
        assert_eq!(calls_to(&e, 0x00aa_c050), vec![vec![process]]);
        assert_eq!(
            calls_to(&e, 0x0047_0470),
            vec![
                vec![process + 0x38c],
                vec![process + 0x394],
                vec![spoke_to],
                vec![detected],
                vec![who]
            ]
        );
        // The detection list items and the event are freed.
        assert_eq!(
            calls_to(&e, 0x0040_1030),
            vec![vec![0xd1], vec![0xd2], vec![0xd3], vec![0x4444]]
        );
        assert_eq!(e.mem.u32(process + 0x3dc), 0);
        assert_eq!(e.mem.u32(process + 0x3e4), 0);
        assert_eq!(e.mem.u32(process + 0x3cc), 0);
        assert_eq!(e.mem.u32(process + 0x368), 0);
        assert_eq!(e.mem.u32(process + 0x3f8), 0);
        assert_eq!(e.mem.u8(process + 0x410), 0);
        assert_eq!(e.mem.u32(process + 0x41c), 0);
        assert_eq!(e.mem.u32(process + 0x3ac), 0xffff_ffff);
        assert_eq!(e.mem.u32(process + 0xb4), 0xffff_ffff);
        assert_eq!(e.mem.f32(process + 0x384), 1.0);
        assert_eq!(e.mem.f32(process + 0x33c), 3.5);
        assert_eq!(e.mem.f32(process + 0x2a0), 2.5);
        assert_eq!(e.mem.f32(process + 0x448), -1.0);
        assert_eq!(e.mem.f32(process + 0x418), -1.0);
        assert_eq!(e.mem.f32(process + 0x2b0), 1.0);
        assert_eq!(e.mem.u16(process + 0x2c2), 0xffff);
        assert_eq!(e.mem.u8(process + 0x340), 1);
        assert_eq!(e.mem.u32(process + 0x430), 2);
    }

    #[test]
    fn test_high_process_fade_in() {
        let mut e = Engine::new();
        fade_constants(&mut e);
        let process = e.mem.alloc(0x500);
        e.mem.set_f32(process + 0x3ec, 0.5);
        e.mem.set_u32(process + 0x3f0, 0x77);
        e.call(0x008f_e8f0, &args![process, 0u32, 0u32]);
        assert_eq!(e.mem.u32(process + 0x3e8), 1);
        assert_eq!(e.mem.u32(process + 0x3f0), 0);
        assert_eq!(e.mem.f32(process + 0x3ec), 0.5);
        // A non-zero flag fades in with state 3; an alpha of 1.0 or more is reset.
        e.mem.set_f32(process + 0x3ec, 1.0);
        e.call(0x008f_e8f0, &args![process, 0u32, 1u32]);
        assert_eq!(e.mem.u32(process + 0x3e8), 3);
        assert_eq!(e.mem.f32(process + 0x3ec), 0.0);
        // States 5 and 6 are left alone.
        for state in [5u32, 6] {
            e.mem.set_u32(process + 0x3e8, state);
            e.mem.set_f32(process + 0x3ec, 0.25);
            e.call(0x008f_e8f0, &args![process, 0u32, 1u32]);
            assert_eq!(e.mem.u32(process + 0x3e8), state);
            assert_eq!(e.mem.f32(process + 0x3ec), 0.25);
        }
    }

    #[test]
    fn test_high_process_fade_out() {
        let mut e = Engine::new();
        fade_constants(&mut e);
        let process = e.mem.alloc(0x500);
        e.call(0x008f_e960, &args![process, 0u32, 0x1234u32, 0u32]);
        assert_eq!(e.mem.u32(process + 0x3e8), 2);
        assert_eq!(e.mem.u32(process + 0x3f0), 0x1234);
        // An alpha of 0.0 restarts at 1.0.
        assert_eq!(e.mem.f32(process + 0x3ec), 1.0);
        // The flag selects state 4; a null reference keeps the old one; a
        // positive alpha stays.
        e.mem.set_f32(process + 0x3ec, 0.5);
        e.call(0x008f_e960, &args![process, 0u32, 0u32, 1u32]);
        assert_eq!(e.mem.u32(process + 0x3e8), 4);
        assert_eq!(e.mem.u32(process + 0x3f0), 0x1234);
        assert_eq!(e.mem.f32(process + 0x3ec), 0.5);
        e.mem.set_u32(process + 0x3e8, 6);
        e.call(0x008f_e960, &args![process, 0u32, 0x99u32, 1u32]);
        assert_eq!(e.mem.u32(process + 0x3e8), 6);
        assert_eq!(e.mem.u32(process + 0x3f0), 0x1234);
    }

    #[test]
    fn test_high_process_fade_out_and_move() {
        let mut e = Engine::new();
        fade_constants(&mut e);
        stub(&mut e, 0x008b_3ab0, 0);
        stub(&mut e, 0x0089_5110, 0);
        e.register_double(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        let process = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        e.call(
            0x008f_e9e0,
            &args![process, 0xa0u32, 0xcc00u32, 1.5f32, 2.5f32, 3.5f32],
        );
        assert_eq!(e.mem.u32(process + 0x3e8), 7);
        assert_eq!(calls_to(&e, 0x008b_3ab0), vec![vec![0xa0]]);
        assert_eq!(
            calls_to(&e, 0x0089_5110),
            vec![vec![0xa0, 1.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        let block = e.mem.u32(process + 0x3f4);
        assert_eq!(calls_to(&e, 0x0040_1000), vec![vec![0x10]]);
        assert_eq!(e.mem.u32(block), 0xcc00);
        assert_eq!(e.mem.f32(block + 4), 1.5);
        assert_eq!(e.mem.f32(block + 8), 2.5);
        assert_eq!(e.mem.f32(block + 12), 3.5);
        assert_eq!(e.mem.f32(process + 0x3ec), 1.0);
        // Fade state 5 or 6: nothing happens.
        e.mem.set_u32(process + 0x3e8, 5);
        e.call_log = Some(vec![]);
        e.call(
            0x008f_e9e0,
            &args![process, 0xa0u32, 0u32, 0f32, 0f32, 0f32],
        );
        assert!(calls_to(&e, 0x008b_3ab0).is_empty());
        assert_eq!(e.mem.u32(process + 0x3e8), 5);
    }

    /// The doubles and objects for the fade-and-stop functions: the player
    /// (`011dea3c`), whether the view test passes, and the logged calls.
    fn fade_stop_scene(e: &mut Engine, view_ok: u32) -> (u32, u32) {
        fade_constants(e);
        for addr in [0x008b_3ab0, 0x0089_5110, 0x0095_0110] {
            stub(e, addr, 0);
        }
        stub(e, 0x004e_af60, view_ok);
        let player = e.mem.alloc(0x40);
        global(e, 0x011d_ea3c, player);
        let process = e.mem.alloc(0x500);
        e.mem.set_u32(process + 0x3f0, 0x55);
        (process, player)
    }

    #[test]
    fn test_high_process_fade_and_delete_for_the_player() {
        let mut e = Engine::new();
        let (process, player) = fade_stop_scene(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x008f_eab0, &args![process, player]);
        assert_eq!(e.mem.u32(process + 0x3e8), 6);
        assert_eq!(e.mem.u32(process + 0x3f0), 0);
        assert_eq!(e.mem.f32(process + 0x3ec), 1.0);
        // The first-person flag is cleared around the animation reset.
        assert_eq!(
            calls_in_order(&e, &[0x004e_af60, 0x0095_0110, 0x008b_3ab0, 0x0089_5110]),
            vec![
                (0x004e_af60, vec![player]),
                (0x0095_0110, vec![player, 0]),
                (0x008b_3ab0, vec![player]),
                (
                    0x0089_5110,
                    vec![player, 1.0f32.to_bits(), 1.0f32.to_bits()]
                ),
                (0x0095_0110, vec![player, 1]),
            ]
        );
    }

    #[test]
    fn test_high_process_fade_and_delete_for_other_actors_and_a_passing_view() {
        let mut e = Engine::new();
        let (process, player) = fade_stop_scene(&mut e, 1);
        let other = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x008f_eab0, &args![process, other]);
        assert!(calls_to(&e, 0x004e_af60).is_empty());
        assert!(calls_to(&e, 0x0095_0110).is_empty());
        assert_eq!(calls_to(&e, 0x008b_3ab0), vec![vec![other]]);
        // The player with a passing view test: no first-person toggling.
        e.call_log = Some(vec![]);
        e.call(0x008f_eab0, &args![process, player]);
        assert_eq!(calls_to(&e, 0x004e_af60), vec![vec![player]]);
        assert!(calls_to(&e, 0x0095_0110).is_empty());
    }

    #[test]
    fn test_high_process_fade_and_disable() {
        let mut e = Engine::new();
        let (process, _) = fade_stop_scene(&mut e, 1);
        let other = e.mem.alloc(0x40);
        e.mem.set_f32(process + 0x3ec, 0.5);
        e.call(0x008f_eb60, &args![process, other]);
        assert_eq!(e.mem.u32(process + 0x3e8), 5);
        assert_eq!(e.mem.u32(process + 0x3f0), 0);
        // A positive alpha stays.
        assert_eq!(e.mem.f32(process + 0x3ec), 0.5);
    }

    #[test]
    fn test_high_process_skip_fade_in() {
        let mut e = Engine::new();
        stub(&mut e, 0x008c_4640, 0);
        let process = e.mem.alloc(0x500);
        e.mem.set_u32(process + 0x3e8, 3);
        e.mem.set_f32(process + 0x3ec, 0.25);
        e.call_log = Some(vec![]);
        e.call(0x008f_f030, &args![process, 0xa0u32]);
        assert_eq!(e.mem.u32(process + 0x3e8), 0);
        assert_eq!(e.mem.f32(process + 0x3ec), 1.0);
        assert_eq!(calls_to(&e, 0x008c_4640), vec![vec![0xa0]]);
        // Other states are left alone.
        e.mem.set_u32(process + 0x3e8, 2);
        e.mem.set_f32(process + 0x3ec, 0.25);
        e.call_log = Some(vec![]);
        e.call(0x008f_f030, &args![process, 0xa0u32]);
        assert_eq!(e.mem.u32(process + 0x3e8), 2);
        assert_eq!(e.mem.f32(process + 0x3ec), 0.25);
        assert!(calls_to(&e, 0x008c_4640).is_empty());
    }

    #[test]
    fn test_fn_008ff080() {
        let mut e = Engine::new();
        let process = e.mem.alloc(0x500);
        for (state, expected) in [
            (0u32, false),
            (2, false),
            (3, true),
            (5, true),
            (6, true),
            (7, false),
        ] {
            e.mem.set_u32(process + 0x3e8, state);
            assert_eq!(
                e.call(0x008f_f080, &args![process]).bool(),
                expected,
                "state {state}"
            );
        }
    }

    /// A scene for `FadeUpdate`: a process in `state` with `alpha`, an actor
    /// that is the player or not, a clock of 1.0 and the two fade settings
    /// (2.0 for the player, 4.0 for others).
    fn fade_update_scene(e: &mut Engine, state: u32, alpha: f32, is_player: bool) -> (u32, u32) {
        fade_constants(e);
        for addr in [
            0x008c_4640,
            0x005c_cb20,
            0x0040_1030,
            0x0057_3170,
            0x0057_4400,
            0x0093_34b0,
        ] {
            stub(e, addr, 0);
        }
        stub_float(e, 0x0084_d030, 1.0);
        float_settings(e, &[(0x011c_d7c8, 2.0), (0x011c_dab8, 4.0)]);
        let actor = slots_object(e, 0x40, &[(0xc4, 0x00aa_c060, 0)]);
        let player = if is_player { actor } else { e.mem.alloc(0x40) };
        global(e, 0x011d_ea3c, player);
        let process = e.mem.alloc(0x500);
        e.mem.set_u32(process + 0x3e8, state);
        e.mem.set_f32(process + 0x3ec, alpha);
        (process, actor)
    }

    #[test]
    fn test_high_process_fade_update_idle_state() {
        let mut e = Engine::new();
        let (process, actor) = fade_update_scene(&mut e, 0, 0.5, false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008f_ec10, &args![process, actor]).bool());
        assert_eq!(e.mem.f32(process + 0x3ec), 1.0);
        assert_eq!(calls_to(&e, 0x008c_4640), vec![vec![actor]]);
        // Already at full alpha: nothing more to do.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008f_ec10, &args![process, actor]).bool());
        assert!(calls_to(&e, 0x008c_4640).is_empty());
    }

    #[test]
    fn test_high_process_fade_update_fading_in_and_out() {
        let mut e = Engine::new();
        // Fading in (state 1) by clock / setting = 1 / 4 for another actor.
        let (process, actor) = fade_update_scene(&mut e, 1, 0.5, false);
        assert!(!e.call(0x008f_ec10, &args![process, actor]).bool());
        assert_eq!(e.mem.f32(process + 0x3ec), 0.75);
        assert_eq!(e.mem.u32(process + 0x3e8), 1);
        // Past 1.0 the alpha is clamped and the state returns to 0.
        e.mem.set_f32(process + 0x3ec, 0.875);
        e.call(0x008f_ec10, &args![process, actor]);
        assert_eq!(e.mem.f32(process + 0x3ec), 1.0);
        assert_eq!(e.mem.u32(process + 0x3e8), 0);
        // State 3 is a fade in as well, and counts as fading for the result.
        e.mem.set_u32(process + 0x3e8, 3);
        e.mem.set_f32(process + 0x3ec, 0.25);
        assert!(e.call(0x008f_ec10, &args![process, actor]).bool());
        assert_eq!(e.mem.f32(process + 0x3ec), 0.5);
        // Fading out (state 2) for the player: step 1 / 2.
        let (process, player) = fade_update_scene(&mut e, 2, 0.75, true);
        e.mem.set_u32(process + 0x3f0, 0x99);
        assert!(!e.call(0x008f_ec10, &args![process, player]).bool());
        assert_eq!(e.mem.f32(process + 0x3ec), 0.25);
        assert_eq!(e.mem.u32(process + 0x3f0), 0x99);
        e.call(0x008f_ec10, &args![process, player]);
        assert_eq!(e.mem.f32(process + 0x3ec), 0.0);
        assert_eq!(e.mem.u32(process + 0x3f0), 0);
    }

    #[test]
    fn test_high_process_fade_update_fade_out_and_move() {
        let mut e = Engine::new();
        let (process, actor) = fade_update_scene(&mut e, 7, 0.25, false);
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, 0xcc00);
        e.mem.set_f32(block + 4, 1.5);
        e.mem.set_f32(block + 8, 2.5);
        e.mem.set_f32(block + 12, 3.5);
        e.mem.set_u32(process + 0x3f4, block);
        e.mem.set_u32(process + 0x3f0, 0x66);
        e.call_log = Some(vec![]);
        // 0.25 - 1/4 = 0: the move is carried out and the block freed.
        assert!(!e.call(0x008f_ec10, &args![process, actor]).bool());
        assert_eq!(
            calls_to(&e, 0x005c_cb20),
            vec![vec![
                actor,
                0xcc00,
                1.5f32.to_bits(),
                2.5f32.to_bits(),
                3.5f32.to_bits()
            ]]
        );
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![block]]);
        assert_eq!(e.mem.u32(process + 0x3f4), 0);
        assert_eq!(e.mem.u32(process + 0x3f0), 0);
        assert_eq!(e.mem.f32(process + 0x3ec), 0.0);
        // With alpha already 0 nothing happens in this state.
        e.call_log = Some(vec![]);
        e.call(0x008f_ec10, &args![process, actor]);
        assert!(calls_to(&e, 0x005c_cb20).is_empty());
    }

    #[test]
    fn test_high_process_fade_update_state_4_activates_the_reference() {
        let mut e = Engine::new();
        // The player: alpha is set to 1.0, the reference activated at once.
        let (process, player) = fade_update_scene(&mut e, 4, 0.5, true);
        e.mem.set_u32(process + 0x3f0, 0x77);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008f_ec10, &args![process, player]).bool());
        assert_eq!(calls_to(&e, 0x0057_3170), vec![vec![0x77, player, 0, 0, 1]]);
        assert_eq!(e.mem.u32(process + 0x3e8), 0);
        assert_eq!(e.mem.u32(process + 0x3f0), 0);
        assert_eq!(e.mem.f32(process + 0x3ec), 1.0);
        // Another actor with a process level: activates, goes back to state 0
        // and the result is true.
        let (process, actor) = fade_update_scene(&mut e, 4, 0.125, false);
        stub(&mut e, 0x0093_34b0, 2);
        e.mem.set_u32(process + 0x3f0, 0x78);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008f_ec10, &args![process, actor]).bool());
        assert_eq!(calls_to(&e, 0x0057_3170), vec![vec![0x78, actor, 0, 0, 1]]);
        assert_eq!(e.mem.u32(process + 0x3e8), 0);
        // Without a process level the actor fades in.
        let (process, actor) = fade_update_scene(&mut e, 4, 0.125, false);
        e.mem.set_u32(process + 0x3f0, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008f_ec10, &args![process, actor]).bool());
        assert!(calls_to(&e, 0x0057_3170).is_empty());
        assert_eq!(e.mem.u32(process + 0x3e8), 3);
        // Still fading out: the alpha only shrinks.
        let (process, actor) = fade_update_scene(&mut e, 4, 0.75, false);
        e.call(0x008f_ec10, &args![process, actor]);
        assert_eq!(e.mem.u32(process + 0x3e8), 4);
        assert_eq!(e.mem.f32(process + 0x3ec), 0.5);
    }

    #[test]
    fn test_high_process_fade_update_delete_and_disable() {
        let mut e = Engine::new();
        let (process, actor) = fade_update_scene(&mut e, 6, 0.125, false);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008f_ec10, &args![process, actor]).bool());
        assert_eq!(calls_to(&e, 0x00aa_c060), vec![vec![actor, 1]]);
        assert_eq!(e.mem.f32(process + 0x3ec), 0.0);
        assert_eq!(calls_to(&e, 0x008c_4640), vec![vec![actor]]);
        let (process, actor) = fade_update_scene(&mut e, 5, 0.125, false);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008f_ec10, &args![process, actor]).bool());
        assert_eq!(calls_to(&e, 0x0057_4400), vec![vec![actor]]);
        assert!(calls_to(&e, 0x00aa_c060).is_empty());
        // An unknown state only updates the alpha.
        let (process, actor) = fade_update_scene(&mut e, 9, 0.25, false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008f_ec10, &args![process, actor]).bool());
        assert_eq!(calls_to(&e, 0x008c_4640), vec![vec![actor]]);
    }

    /// Objects for `FindSpecialIdletoPlay`: a process whose slot `0x44`
    /// returns 1, an actor at height `actor_z` whose slot `0x380` gives
    /// `lowered` and whose eye level is `eye`, and a reference at height
    /// `reference_z` whose slot `0x218` gives `reference_flag`.
    fn idle_scene(
        e: &mut Engine,
        eye: f64,
        actor_z: f32,
        lowered: f64,
        reference_z: f32,
        reference_flag: u32,
    ) -> (u32, u32, u32) {
        for addr in [0x0060_0900, 0x0060_0920, 0x0060_0940] {
            stub(e, addr, 0);
        }
        stub_float(e, 0x008b_e940, eye);
        let process = slots_object(e, 0x500, &[(0x44, 0x00aa_c070, 1)]);
        let actor_location = e.mem.alloc(12);
        e.mem.set_f32(actor_location + 8, actor_z);
        stub(e, 0x00aa_c080, actor_location);
        stub_float(e, 0x00aa_c090, lowered);
        let table = vtable(e, &[(0x1f4, 0x00aa_c080), (0x380, 0x00aa_c090)]);
        let actor = object(e, 0x40, table);
        let reference_location = e.mem.alloc(12);
        e.mem.set_f32(reference_location + 8, reference_z);
        let reference = slots_object(
            e,
            0x240,
            &[
                (0x1f4, 0x00aa_c0a0, reference_location),
                (0x218, 0x00aa_c0b0, reference_flag),
            ],
        );
        (process, actor, reference)
    }

    #[test]
    fn test_high_process_find_special_idleto_play_without_a_reference() {
        let mut e = Engine::new();
        let (process, actor, _) = idle_scene(&mut e, 1.5, 10.0, 0.5, 12.0, 0);
        e.call_log = Some(vec![]);
        let played = e.call(0x008f_f0b0, &args![process, actor, 0x1d1eu32, 0u32]);
        assert!(played.bool());
        assert_eq!(
            calls_in_order(&e, &[0x0060_0900, 0x0060_0920, 0x0060_0940, 0x00aa_c070]),
            vec![
                (0x0060_0900, vec![0x1d1e]),
                (0x0060_0940, vec![1]),
                (0x0060_0920, vec![0]),
                (0x00aa_c070, vec![process, actor, 0, 2, 1, 0, 1]),
                (0x0060_0900, vec![0]),
                (0x0060_0940, vec![0]),
                (0x0060_0920, vec![0xffff_ffff]),
            ]
        );
    }

    #[test]
    fn test_high_process_find_special_idleto_play_levels() {
        // Lowered limit 10 + 1.5 - 0.5 = 11 against a reference at 12: level 1.
        let mut e = Engine::new();
        let (process, actor, reference) = idle_scene(&mut e, 1.5, 10.0, 0.5, 12.0, 0);
        e.call_log = Some(vec![]);
        e.call(0x008f_f0b0, &args![process, actor, 1u32, reference]);
        assert_eq!(calls_to(&e, 0x0060_0920), vec![vec![1], vec![0xffff_ffff]]);
        // A lower reference: level 0 unless slot 0x218 says otherwise.
        let mut e = Engine::new();
        let (process, actor, reference) = idle_scene(&mut e, 1.5, 10.0, 0.5, 5.0, 0);
        e.call_log = Some(vec![]);
        e.call(0x008f_f0b0, &args![process, actor, 1u32, reference]);
        assert_eq!(calls_to(&e, 0x0060_0920), vec![vec![0], vec![0xffff_ffff]]);
        let mut e = Engine::new();
        let (process, actor, reference) = idle_scene(&mut e, 1.5, 10.0, 0.5, 5.0, 1);
        e.call_log = Some(vec![]);
        e.call(0x008f_f0b0, &args![process, actor, 1u32, reference]);
        assert_eq!(calls_to(&e, 0x0060_0920), vec![vec![1], vec![0xffff_ffff]]);
    }

    /// Doubles for the sound handle members; `valid` and `playing` are what
    /// `IsValid` and `IsPlaying` answer. `00418900` is logged only.
    fn sound_doubles(e: &mut Engine, valid: u32, playing: u32) {
        stub(e, 0x00ad_8ce0, valid);
        stub(e, 0x00ad_8930, playing);
        for addr in [0x00ad_88f0, 0x00ad_8d10, 0x0041_8900, 0x0048_3710] {
            stub(e, addr, 0);
        }
        e.register_double(0x0041_a250, |_, a| ret(a[0]));
    }

    #[test]
    fn test_high_process_stop_sound_handle() {
        let mut e = Engine::new();
        sound_doubles(&mut e, 1, 1);
        let process = e.mem.alloc(0x500);
        let handle = process + 0x314 + 12;
        e.call_log = Some(vec![]);
        e.call(0x008f_f1b0, &args![process, 1u32]);
        let log = calls_in_order(
            &e,
            &[
                0x00ad_8ce0,
                0x00ad_8930,
                0x00ad_88f0,
                0x00ad_8d10,
                0x0041_a250,
                0x0041_8900,
                0x0048_3710,
            ],
        );
        let temporary = log[4].1[0];
        assert_eq!(
            log,
            vec![
                (0x00ad_8ce0, vec![handle]),
                (0x00ad_8930, vec![handle]),
                (0x00ad_88f0, vec![handle]),
                (0x00ad_8d10, vec![handle]),
                (0x0041_a250, vec![temporary]),
                (0x0041_8900, vec![handle, temporary]),
                (0x0048_3710, vec![temporary]),
            ]
        );
        // Valid but not playing: no stop. Not valid: nothing.
        let mut e = Engine::new();
        sound_doubles(&mut e, 1, 0);
        let process = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        e.call(0x008f_f1b0, &args![process, 0u32]);
        assert!(calls_to(&e, 0x00ad_88f0).is_empty());
        assert_eq!(calls_to(&e, 0x00ad_8d10), vec![vec![process + 0x314]]);
        let mut e = Engine::new();
        sound_doubles(&mut e, 0, 1);
        let process = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        e.call(0x008f_f1b0, &args![process, 0u32]);
        assert!(calls_to(&e, 0x00ad_8d10).is_empty());
        assert!(calls_to(&e, 0x0041_8900).is_empty());
    }

    #[test]
    fn test_high_process_start_torch_sound() {
        let mut e = Engine::new();
        sound_doubles(&mut e, 0, 0);
        global(&mut e, 0x011c_3f2c, 0x4000);
        e.register_double(0x0046_16c0, |_, _| ret(0x5678));
        stub(&mut e, 0x0084_e3a0, 0x9999);
        e.register_double(0x0093_3150, |_, a| ret(a[1]));
        let process = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        e.call(0x008f_f290, &args![process, 0x7000u32]);
        assert_eq!(calls_to(&e, 0x0046_16c0), vec![vec![0x4000, 0x0108_8398]]);
        assert_eq!(calls_to(&e, 0x0084_e3a0), vec![vec![0x5678]]);
        let started = calls_to(&e, 0x0093_3150);
        let temporary = started[0][1];
        assert_eq!(started, vec![vec![0x7000, temporary, 0x9999, 1, 2, 1]]);
        assert_eq!(
            calls_to(&e, 0x0041_8900),
            vec![vec![process + 0x320, temporary]]
        );
        assert_eq!(calls_to(&e, 0x0048_3710), vec![vec![temporary]]);
        // No owner: the sound is looked up but not started.
        e.call_log = Some(vec![]);
        e.call(0x008f_f290, &args![process, 0u32]);
        assert!(calls_to(&e, 0x0093_3150).is_empty());
        // No sound.
        e.register_double(0x0046_16c0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x008f_f290, &args![process, 0x7000u32]);
        assert!(calls_to(&e, 0x0093_3150).is_empty());
        // The second handle is valid already: nothing is looked up.
        stub(&mut e, 0x00ad_8ce0, 1);
        e.call_log = Some(vec![]);
        e.call(0x008f_f290, &args![process, 0x7000u32]);
        assert!(calls_to(&e, 0x0046_16c0).is_empty());
    }

    /// The detection mode table (`011a3780`): `(mode, byte 4, byte 5, byte 6)`.
    fn detection_table(e: &mut Engine, rows: &[(u32, u8, u8, u8)]) {
        e.map(0x011a_3780, 8 * rows.len() as u32);
        for (index, (mode, b4, b5, b6)) in rows.iter().enumerate() {
            let row = 0x011a_3780 + 8 * index as u32;
            e.mem.set_u32(row, *mode);
            e.mem.set_u8(row + 4, *b4);
            e.mem.set_u8(row + 5, *b5);
            e.mem.set_u8(row + 6, *b6);
        }
    }

    #[test]
    fn test_high_process_check_target_combat_detection_state() {
        let mut e = Engine::new();
        detection_table(
            &mut e,
            &[(0, 0, 0, 0), (1, 0, 0, 0), (2, 0, 0, 0), (9, 0, 0, 0)],
        );
        let process = e.mem.alloc(0x500);
        // 008a16b0(a) gives 0xb1.
        stub(&mut e, 0x008a_16b0, 0xb1);
        stub(&mut e, 0x008b_c700, 0);
        stub(&mut e, 0x008c_1680, 0);
        stub(&mut e, 0x005b_5e40, 0);
        let check = |e: &mut Engine, row: u32, b: u32| {
            e.mem.set_u32(process + 0x3d8, row);
            e.call(0x008f_fc30, &args![process, 0xa1u32, b]).bool()
        };
        // Mode 0: only the compared target counts.
        assert!(check(&mut e, 0, 0xb1));
        assert!(!check(&mut e, 0, 0xb2));
        // Mode 1: also being in combat with the actor.
        assert!(check(&mut e, 1, 0xb1));
        assert!(!check(&mut e, 1, 0xb2));
        stub(&mut e, 0x008b_c700, 1);
        assert!(check(&mut e, 1, 0xb2));
        // Mode 2: true for the target; false when in combat with the actor,
        // or when 008c1680 holds; otherwise true.
        assert!(check(&mut e, 2, 0xb1));
        assert!(!check(&mut e, 2, 0xb2));
        stub(&mut e, 0x008b_c700, 0);
        assert!(check(&mut e, 2, 0xb2));
        stub(&mut e, 0x008c_1680, 1);
        assert!(!check(&mut e, 2, 0xb2));
        // An unknown mode is reported and gives false.
        e.call_log = Some(vec![]);
        assert!(!check(&mut e, 3, 0xb1));
        assert_eq!(calls_to(&e, 0x005b_5e40), vec![vec![0x0108_83a8, 9]]);
    }

    #[test]
    fn test_high_process_should_run_combat_detection() {
        let mut e = Engine::new();
        // Row 0: mode 0, view check on; row 1: mode 0, view check off.
        detection_table(&mut e, &[(0, 1, 0, 0), (0, 0, 0, 0)]);
        let process = e.mem.alloc(0x500);
        let player = e.mem.alloc(0x40);
        global(&mut e, 0x011d_ea3c, player);
        global(&mut e, 0x0101_ff38, 0.5f32.to_bits());
        stub(&mut e, 0x0043_7bd0, 0);
        e.register_double(0x0043_6aa0, |_, a| ret(a[0] + 0x30));
        stub(&mut e, 0x0088_c570, 1);
        let target = slots_object(&mut e, 0x240, &[(0x22c, 0x00aa_c0c0, 0)]);
        let stranger = slots_object(&mut e, 0x240, &[(0x22c, 0x00aa_c0c8, 0)]);
        // 008a16b0 answers `target`, so the combat check passes for it only.
        stub(&mut e, 0x008a_16b0, target);
        let run = |e: &mut Engine, row: u32, b: u32| {
            e.mem.set_u32(process + 0x3d8, row);
            e.call(0x008f_fd10, &args![process, 0xa1u32, b]).bool()
        };
        e.call_log = Some(vec![]);
        assert!(run(&mut e, 0, target));
        // The view cone is asked with b's position and the cone constant.
        assert_eq!(
            calls_to(&e, 0x0088_c570),
            vec![vec![0xa1, target + 0x30, 0.5f32.to_bits()]]
        );
        // Outside the cone: false.
        stub(&mut e, 0x0088_c570, 0);
        assert!(!run(&mut e, 0, target));
        // View check off: true without asking.
        e.call_log = Some(vec![]);
        assert!(run(&mut e, 1, target));
        assert!(calls_to(&e, 0x0088_c570).is_empty());
        // The combat check fails for another target.
        assert!(!run(&mut e, 1, stranger));
        // 00437bd0 or slot 0x22c true: false.
        stub(&mut e, 0x0043_7bd0, 1);
        assert!(!run(&mut e, 1, target));
        stub(&mut e, 0x0043_7bd0, 0);
        stub(&mut e, 0x00aa_c0c0, 1);
        assert!(!run(&mut e, 1, target));
    }

    #[test]
    fn test_high_process_should_run_combat_detection_player_skips_the_cone() {
        let mut e = Engine::new();
        detection_table(&mut e, &[(0, 1, 0, 0)]);
        let process = e.mem.alloc(0x500);
        let player = slots_object(&mut e, 0x240, &[(0x22c, 0x00aa_c0d0, 0)]);
        global(&mut e, 0x011d_ea3c, player);
        global(&mut e, 0x0101_ff38, 0.5f32.to_bits());
        stub(&mut e, 0x0043_7bd0, 0);
        stub(&mut e, 0x008a_16b0, player);
        stub(&mut e, 0x0088_c570, 0);
        e.call_log = Some(vec![]);
        // The player is never tested against the view cone.
        assert!(e.call(0x008f_fd10, &args![process, 0xa1u32, player]).bool());
        assert!(calls_to(&e, 0x0088_c570).is_empty());
        // A check that fails gives false.
        stub(&mut e, 0x008a_16b0, 0);
        assert!(!e.call(0x008f_fd10, &args![process, 0xa1u32, player]).bool());
    }

    #[test]
    fn test_high_process_should_run_combat_detection_event_check() {
        let mut e = Engine::new();
        detection_table(&mut e, &[(0, 0, 0, 0), (0, 0, 1, 0)]);
        let process = e.mem.alloc(0x500);
        stub(&mut e, 0x008a_16b0, 0xb1);
        e.mem.set_u32(process + 0x3d8, 0);
        assert!(!e
            .call(0x008f_fdc0, &args![process, 0xa1u32, 0xb1u32])
            .bool());
        e.mem.set_u32(process + 0x3d8, 1);
        assert!(e
            .call(0x008f_fdc0, &args![process, 0xa1u32, 0xb1u32])
            .bool());
        assert!(!e
            .call(0x008f_fdc0, &args![process, 0xa1u32, 0xb2u32])
            .bool());
    }

    #[test]
    fn test_high_process_should_run_player_detection() {
        let mut e = Engine::new();
        detection_table(&mut e, &[(0, 0, 0, 0), (0, 0, 0, 1)]);
        let process = e.mem.alloc(0x500);
        stub(&mut e, 0x0049_3bb0, 0);
        e.mem.set_u32(process + 0x3d8, 0);
        // Not a player-like actor: always true.
        assert!(e.call(0x008f_fe10, &args![process, 0xa1u32]).bool());
        stub(&mut e, 0x0049_3bb0, 1);
        assert!(!e.call(0x008f_fe10, &args![process, 0xa1u32]).bool());
        e.mem.set_u32(process + 0x3d8, 1);
        assert!(e.call(0x008f_fe10, &args![process, 0xa1u32]).bool());
    }

    /// A scene for `ReactToCombatSituation(a, b)`: the actors, with every
    /// test passing by default.
    fn react_scene(e: &mut Engine) -> (u32, u32, u32) {
        list_doubles(e);
        fade_constants(e);
        for addr in [
            0x008b_0970,
            0x008b_06d0,
            0x008a_78f0,
            0x008a_61b0,
            0x0049_3bb0,
        ] {
            stub(e, addr, 0);
        }
        stub_float(e, 0x0057_23b0, 50.0);
        float_settings(e, &[(0x011c_dfe4, 100.0)]);
        global(e, 0x011d_ea3c, 0x5000);
        let process = e.mem.alloc(0x500);
        let b = slots_object(
            e,
            0x40,
            &[
                (0x42c, 0x00aa_c0e0, 0x6000),
                (0x428, 0x00aa_c0f0, 0x6100),
                (0x3f8, 0x00aa_c100, 0x6200),
            ],
        );
        (process, 0xa1, b)
    }

    #[test]
    fn test_high_process_react_to_combat_situation_queues_a_spectator() {
        let mut e = Engine::new();
        let (process, a, b) = react_scene(&mut e);
        e.call_log = Some(vec![]);
        // Always false, but a spectator is queued.
        assert!(!e.call(0x008f_fe50, &args![process, a, b]).bool());
        assert_eq!(calls_to(&e, 0x008b_0970), vec![vec![a, b]]);
        assert_eq!(calls_to(&e, 0x008b_06d0).len(), 1);
        assert_eq!(calls_to(&e, 0x0057_23b0), vec![vec![b, a, 0, 1]]);
        let queued = list_items(&e, process + 0x28c);
        assert_eq!(queued.len(), 1);
        let node = queued[0];
        assert_eq!(e.mem.u32(node), b);
        assert_eq!(e.mem.u32(node + 4), 0x6200);
        assert_eq!(e.mem.u8(node + 0xd), 1);
    }

    #[test]
    fn test_high_process_react_to_combat_situation_declines() {
        // Already helping.
        let mut e = Engine::new();
        let (process, a, b) = react_scene(&mut e);
        stub(&mut e, 0x008b_0970, 1);
        e.call(0x008f_fe50, &args![process, a, b]);
        assert!(list_items(&e, process + 0x28c).is_empty());
        // Would attack b's target.
        let mut e = Engine::new();
        let (process, a, b) = react_scene(&mut e);
        stub(&mut e, 0x008b_06d0, 1);
        e.call(0x008f_fe50, &args![process, a, b]);
        assert!(list_items(&e, process + 0x28c).is_empty());
        // Too far away.
        let mut e = Engine::new();
        let (process, a, b) = react_scene(&mut e);
        stub_float(&mut e, 0x0057_23b0, 150.0);
        e.call(0x008f_fe50, &args![process, a, b]);
        assert!(list_items(&e, process + 0x28c).is_empty());
        // Behaviour kind 2 skipped, alarmed, or player-like.
        for addr in [0x008a_78f0, 0x008a_61b0, 0x0049_3bb0] {
            let mut e = Engine::new();
            let (process, a, b) = react_scene(&mut e);
            stub(&mut e, addr, 1);
            e.call(0x008f_fe50, &args![process, a, b]);
            assert!(list_items(&e, process + 0x28c).is_empty(), "{addr:08x}");
        }
        // b has no slot 0x428 word (and is not the player).
        let mut e = Engine::new();
        let (process, a, _) = react_scene(&mut e);
        let bare = slots_object(
            &mut e,
            0x40,
            &[
                (0x42c, 0x00aa_c110, 0x6000),
                (0x428, 0x00aa_c120, 0),
                (0x3f8, 0x00aa_c130, 0x6200),
            ],
        );
        e.call(0x008f_fe50, &args![process, a, bare]);
        assert!(list_items(&e, process + 0x28c).is_empty());
        // The player as b needs neither word.
        global(&mut e, 0x011d_ea3c, bare);
        e.call(0x008f_fe50, &args![process, a, bare]);
        assert_eq!(list_items(&e, process + 0x28c).len(), 1);
    }

    #[test]
    fn test_fn_00900000() {
        let mut e = Engine::new();
        assert!(!e
            .call(0x0090_0000, &args![0x1000u32, 1u32, 2u32, 3u32])
            .bool());
    }

    /// A pair scene for `fn_008ff350`: the process, the evaluated `actor` and
    /// the `other` actor, with every test passing by default (the other has a
    /// detection level of 20 against a threshold of 10, nobody attacks, there
    /// is no sight distance).
    struct PairScene {
        process: u32,
        actor: u32,
        other: u32,
        state: u32,
    }

    fn pair_scene(e: &mut Engine) -> PairScene {
        list_doubles(e);
        fade_constants(e);
        for addr in [
            0x0070_5cf0,
            0x0082_5c00,
            0x0049_3bb0,
            0x0044_0da0,
            0x008b_8e90,
            0x0088_1510,
            0x0067_a770,
            0x0043_7bd0,
            0x005a_4320,
            0x008a_c6f0,
            0x0099_2530,
            0x0095_3c20,
            0x0098_65b0,
            0x008a_ce90,
            0x0047_d740,
            0x0059_f610,
            0x0040_30b0,
            0x008a_78f0,
            0x0043_9ef0,
            0x0043_6aa0,
        ] {
            stub(e, addr, 0);
        }
        stub(e, 0x0041_81e0, 0x6000);
        stub(e, 0x0058_db10, 0x7777);
        // GetShouldHelp answers true so that ReactToCombatSituation stops at once.
        stub(e, 0x008b_0970, 1);
        stub(e, 0x008b_06d0, 0);
        stub_float(e, 0x008a_cbe0, 2.0);
        stub_float(e, 0x0047_eeb0, 0.5);
        stub_float(e, 0x004a_7290, 0.0);
        float_settings(e, &[(0x011c_d45c, 10.0), (0x011c_df10, 5.0)]);
        int_settings(e, &[(0x011c_f414, 3)]);
        global(e, 0x011d_ea3c, 0x5000);
        global(e, 0x011f_1958, 0x5500);
        global(e, 0x0101_2054, (-1.0f32).to_bits());
        let other = slots_object(
            e,
            0x100,
            &[
                (0x100, 0x00aa_d000, 1),
                (0x3f8, 0x00aa_d010, 0x8100),
                (0x218, 0x00aa_d020, 0),
                (0x274, 0x00aa_d030, 0),
                (0x42c, 0x00aa_d040, 0),
                (0x37c, 0x00aa_d050, 0),
                (0x22c, 0x00aa_d060, 0),
                (0x428, 0x00aa_d070, 0),
            ],
        );
        let actor = slots_object(
            e,
            0x100,
            &[
                (0x100, 0x00aa_d0d0, 1),
                (0x3f8, 0x00aa_d080, 0x8200),
                (0x21c, 0x00aa_d090, 0),
                (0x304, 0x00aa_d0a0, 0),
            ],
        );
        // The interface at actor + 0xa4 whose slot 8 gives 0.
        let interface = vtable(e, &[(8, 0x00aa_d0b0)]);
        stub(e, 0x00aa_d0b0, 0);
        e.mem.set_u32(actor + 0xa4, interface);
        let state = detection_state_for(e, other, 20);
        let process = slots_object(e, 0x500, &[(0x504, 0x00aa_d0c0, state)]);
        // The process of `other` is the process itself.
        stub(e, 0x008d_8520, process);
        PairScene {
            process,
            actor,
            other,
            state,
        }
    }

    /// Runs `fn_008ff350` with a trigger of 3.0 and a strength of 1.0.
    fn run_pair(e: &mut Engine, scene: &PairScene) {
        e.call(
            0x008f_f350,
            &args![
                scene.process,
                scene.other,
                scene.actor,
                0u32,
                0u32,
                0u32,
                3.0f32,
                1.0f32
            ],
        );
    }

    #[test]
    fn test_fn_008ff350_stops_early() {
        // No other actor.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        e.call_log = Some(vec![]);
        e.call(
            0x008f_f350,
            &args![
                scene.process,
                0u32,
                scene.actor,
                0u32,
                0u32,
                0u32,
                3.0f32,
                1.0f32
            ],
        );
        assert!(calls_to(&e, 0x008b_8e90).is_empty());
        // The other is the evaluated actor, deleted, not an actor, or in the
        // faction of default object 8.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        e.call_log = Some(vec![]);
        e.call(
            0x008f_f350,
            &args![
                scene.process,
                scene.actor,
                scene.actor,
                0u32,
                0u32,
                0u32,
                3.0f32,
                1.0f32
            ],
        );
        assert!(calls_to(&e, 0x008b_8e90).is_empty());
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        stub(&mut e, 0x0044_0da0, 1);
        e.call_log = Some(vec![]);
        run_pair(&mut e, &scene);
        assert!(calls_to(&e, 0x008b_8e90).is_empty());
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        stub(&mut e, 0x008b_8e90, 1);
        e.call_log = Some(vec![]);
        run_pair(&mut e, &scene);
        assert_eq!(calls_to(&e, 0x008b_8e90), vec![vec![scene.other, 0x7777]]);
        assert_eq!(e.mem.u8(scene.state + 0x1f), 0);
        // The level does not exceed the threshold.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        float_settings(&mut e, &[(0x011c_d45c, 25.0)]);
        run_pair(&mut e, &scene);
        assert_eq!(e.mem.u8(scene.state + 0x1f), 0);
        // A package that fails 0067a770 drops the threshold to 0.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        float_settings(&mut e, &[(0x011c_d45c, 25.0)]);
        stub(&mut e, 0x0088_1510, 0x4242);
        run_pair(&mut e, &scene);
        assert_eq!(e.mem.u8(scene.state + 0x1f), 1);
        // Already evaluated.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        e.mem.set_u8(scene.state + 0x1f, 1);
        e.call_log = Some(vec![]);
        run_pair(&mut e, &scene);
        assert!(calls_to(&e, 0x0043_7bd0).is_empty());
        // Slot 0x22c or 00437bd0 true: marked evaluated, then nothing.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        stub(&mut e, 0x0043_7bd0, 1);
        e.call_log = Some(vec![]);
        run_pair(&mut e, &scene);
        assert_eq!(e.mem.u8(scene.state + 0x1f), 1);
        assert!(calls_to(&e, 0x008a_cbe0).is_empty());
    }

    #[test]
    fn test_fn_008ff350_queues_the_other_for_an_attacker() {
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        let (actor, other) = (scene.actor, scene.other);
        // The actor would attack the other: reaction 100.
        e.register_double(0x008b_06d0, move |e, a| {
            e.mem.set_u32(a[3], 0);
            ret(u32::from(a[0] == actor && a[1] == other))
        });
        e.call_log = Some(vec![]);
        run_pair(&mut e, &scene);
        assert_eq!(e.mem.u8(scene.state + 0x1f), 1);
        // The other fails slot 0x218, so it is queued without being recorded
        // as `plastDetected`.
        let queued = list_items(&e, scene.process + 0x274);
        assert_eq!(queued.len(), 1);
        assert_eq!(e.mem.u32(queued[0]), other);
        assert_eq!(e.mem.u8(queued[0] + 9), 0);
        assert_eq!(e.mem.u32(scene.process + 0x2a4), 0);
        // ReactToCombatSituation(actor, other) ran.
        assert_eq!(calls_to(&e, 0x008b_0970), vec![vec![actor, other]]);
        // The combat strength was asked of the other with -1.0.
        assert_eq!(
            calls_to(&e, 0x008a_cbe0),
            vec![vec![other, (-1.0f32).to_bits()]]
        );
    }

    #[test]
    fn test_fn_008ff350_records_the_other_as_last_detected() {
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        let (actor, other) = (scene.actor, scene.other);
        e.register_double(0x008b_06d0, move |e, a| {
            e.mem.set_u32(a[3], 0);
            ret(u32::from(a[0] == actor && a[1] == other))
        });
        // The other passes every test of the help branch: slot 0x218, 0059f610
        // and a base that is not evil-only.
        stub(&mut e, 0x00aa_d020, 1);
        stub(&mut e, 0x0059_f610, 1);
        run_pair(&mut e, &scene);
        let queued = list_items(&e, scene.process + 0x274);
        assert_eq!(queued.len(), 1);
        assert_eq!(e.mem.u32(queued[0]), other);
        assert_eq!(e.mem.u32(scene.process + 0x2a4), other);
        // With a scheduler (slot 0x428) that is a combatant as well, the other
        // is dropped.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        let (actor, other) = (scene.actor, scene.other);
        e.register_double(0x008b_06d0, move |e, a| {
            e.mem.set_u32(a[3], 0);
            ret(u32::from(a[0] == actor && a[1] == other))
        });
        stub(&mut e, 0x00aa_d020, 1);
        stub(&mut e, 0x0059_f610, 1);
        stub(&mut e, 0x00aa_d070, 0x9000);
        stub(&mut e, 0x0040_30b0, 0x9100);
        stub(&mut e, 0x008a_c6f0, 1);
        e.call_log = Some(vec![]);
        run_pair(&mut e, &scene);
        assert_eq!(calls_to(&e, 0x0040_30b0), vec![vec![0x9000]]);
        assert!(list_items(&e, scene.process + 0x274).is_empty());
        assert_eq!(e.mem.u32(scene.process + 0x2a4), 0);
    }

    #[test]
    fn test_fn_008ff350_avoids_and_aggro_radius() {
        // The other would attack the actor, and the roll beats the ratio
        // strength / combat strength = 1.0 / 2.0: the other joins AvoidActorList.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        let (actor, other) = (scene.actor, scene.other);
        e.register_double(0x008b_06d0, move |e, a| {
            e.mem.set_u32(a[3], 0);
            ret(u32::from(a[0] == other && a[1] == actor))
        });
        stub_float(&mut e, 0x0047_eeb0, 0.75);
        e.call_log = Some(vec![]);
        run_pair(&mut e, &scene);
        assert_eq!(calls_to(&e, 0x0047_eeb0), vec![vec![3]]);
        assert_eq!(list_items(&e, scene.process + 0x394), vec![other]);
        assert!(list_items(&e, scene.process + 0x274).is_empty());
        // A lower roll: no avoid entry.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        let (actor, other) = (scene.actor, scene.other);
        e.register_double(0x008b_06d0, move |e, a| {
            e.mem.set_u32(a[3], 0);
            ret(u32::from(a[0] == other && a[1] == actor))
        });
        stub_float(&mut e, 0x0047_eeb0, 0.25);
        run_pair(&mut e, &scene);
        assert!(list_items(&e, scene.process + 0x394).is_empty());
        // Nobody attacks, a sight distance of 5 and the squared distance 20
        // within 25: the other joins AggroRadiusList.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        stub(&mut e, 0x0070_5cf0, 1);
        stub(&mut e, 0x0082_5c00, 5);
        stub_float(&mut e, 0x004a_7290, 20.0);
        e.call_log = Some(vec![]);
        run_pair(&mut e, &scene);
        assert_eq!(list_items(&e, scene.process + 0x38c), vec![scene.other]);
        assert_eq!(calls_to(&e, 0x008a_78f0), vec![vec![scene.actor, 6]]);
        // Too far, or the behaviour skipped: not added.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        stub(&mut e, 0x0070_5cf0, 1);
        stub(&mut e, 0x0082_5c00, 5);
        stub_float(&mut e, 0x004a_7290, 30.0);
        run_pair(&mut e, &scene);
        assert!(list_items(&e, scene.process + 0x38c).is_empty());
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        stub(&mut e, 0x0070_5cf0, 1);
        stub(&mut e, 0x0082_5c00, 5);
        stub_float(&mut e, 0x004a_7290, 20.0);
        stub(&mut e, 0x008a_78f0, 1);
        run_pair(&mut e, &scene);
        assert!(list_items(&e, scene.process + 0x38c).is_empty());
    }

    #[test]
    fn test_fn_008ff350_queues_the_group_to_help() {
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        // No sight distance (the detection level of 20 gates this, 008ff545);
        // the other (slot 0x274, with a desired target in slot 0x42c) has a
        // level above the help setting; group, 0005a4320 and the manager's
        // GetWanttoHelpGroup all agree.
        stub(&mut e, 0x00aa_d030, 1);
        stub(&mut e, 0x00aa_d040, 0x9300);
        stub(&mut e, 0x005a_4320, 1);
        stub(&mut e, 0x0099_2530, 1);
        e.call_log = Some(vec![]);
        run_pair(&mut e, &scene);
        assert_eq!(calls_to(&e, 0x00aa_d030), vec![vec![scene.other, 1]]);
        assert_eq!(
            calls_to(&e, 0x0099_2530),
            vec![vec![0x5500, scene.actor, scene.other]]
        );
        let queued = list_items(&e, scene.process + 0x27c);
        assert_eq!(queued.len(), 1);
        assert_eq!(e.mem.u32(queued[0] + 4), 0x8100);
        // The player is in more fights than allowed and the group targets the
        // player while the actor's group does not: not queued.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        stub(&mut e, 0x00aa_d030, 1);
        stub(&mut e, 0x00aa_d040, 0x9300);
        stub(&mut e, 0x005a_4320, 1);
        stub(&mut e, 0x0099_2530, 1);
        stub(&mut e, 0x0095_3c20, 5);
        e.register_double(0x0098_65b0, |_, a| ret(u32::from(a[0] == 0x8100)));
        run_pair(&mut e, &scene);
        assert!(list_items(&e, scene.process + 0x27c).is_empty());
        // The group is queued already: not queued again.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        stub(&mut e, 0x00aa_d030, 1);
        stub(&mut e, 0x00aa_d040, 0x9300);
        stub(&mut e, 0x005a_4320, 1);
        stub(&mut e, 0x0099_2530, 1);
        let existing = e.mem.alloc(0x14);
        e.mem.set_u32(existing + 4, 0x8100);
        e.mem.set_u32(scene.process + 0x27c, existing);
        run_pair(&mut e, &scene);
        assert_eq!(list_items(&e, scene.process + 0x27c), vec![existing]);
    }

    #[test]
    fn test_fn_008ff350_level_not_sight_gates_help_and_aggro_radius() {
        // A threshold of -10 lets a detection level of 0 through. A sight
        // distance of 5 and the squared distance 20 make the other close, and
        // it answers slot 0x274, but the level of 0 skips the wants-help
        // check (008ff545) and the AggroRadiusList entry (008ffbd7).
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        float_settings(&mut e, &[(0x011c_d45c, -10.0)]);
        e.mem.set_i32(scene.state + 8, 0);
        stub(&mut e, 0x0070_5cf0, 1);
        stub(&mut e, 0x0082_5c00, 5);
        stub_float(&mut e, 0x004a_7290, 20.0);
        stub(&mut e, 0x00aa_d030, 1);
        e.call_log = Some(vec![]);
        run_pair(&mut e, &scene);
        assert_eq!(e.mem.u8(scene.state + 0x1f), 1);
        assert!(calls_to(&e, 0x00aa_d030).is_empty());
        assert!(calls_to(&e, 0x008a_78f0).is_empty());
        assert!(list_items(&e, scene.process + 0x38c).is_empty());
        // A level of 1 under the same threshold: both run.
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        float_settings(&mut e, &[(0x011c_d45c, -10.0)]);
        e.mem.set_i32(scene.state + 8, 1);
        stub(&mut e, 0x0070_5cf0, 1);
        stub(&mut e, 0x0082_5c00, 5);
        stub_float(&mut e, 0x004a_7290, 20.0);
        stub(&mut e, 0x00aa_d030, 1);
        e.call_log = Some(vec![]);
        run_pair(&mut e, &scene);
        assert_eq!(calls_to(&e, 0x00aa_d030), vec![vec![scene.other, 1]]);
        assert_eq!(calls_to(&e, 0x008a_78f0), vec![vec![scene.actor, 6]]);
        assert_eq!(list_items(&e, scene.process + 0x38c), vec![scene.other]);
    }

    /// A process for `EnterCombat`: slot `0x20c` gives `run_once`, slot
    /// `0x324` `finishing`; `00916880` answers 1.
    fn enter_combat_scene(e: &mut Engine, run_once: u32, finishing: u32, package_type: u32) -> u32 {
        stub(e, 0x0041_ca90, package_type);
        stub(e, 0x0091_6880, 1);
        float_settings(e, &[(0x011c_da64, 6.0)]);
        slots_object(
            e,
            0x500,
            &[
                (0x20c, 0x00aa_e000, run_once),
                (0x22c, 0x00aa_e010, 0),
                (0x324, 0x00aa_e020, finishing),
                (0x848, 0x00aa_e030, 0),
            ],
        )
    }

    #[test]
    fn test_high_process_enter_combat_forwards_all_words() {
        let mut e = Engine::new();
        let process = enter_combat_scene(&mut e, 0, 0, 0);
        e.mem.set_u8(process + 0x3a8, 1);
        e.mem.set_f32(process + 0x3a4, 1.0);
        e.call_log = Some(vec![]);
        let words: Vec<u32> = (1..=13).collect();
        let mut arguments = vec![process];
        arguments.extend(&words);
        let r = e.call(0x0090_01c0, &arguments);
        assert!(r.bool());
        // The armour is re-equipped (slot 0x848) and the timer restarted.
        assert_eq!(calls_to(&e, 0x00aa_e030), vec![vec![process, 1, 0, 0]]);
        assert_eq!(e.mem.u8(process + 0x3a8), 0);
        assert_eq!(e.mem.f32(process + 0x3a4), 6.0);
        // All 13 words reach MiddleHighProcess::EnterCombat.
        assert_eq!(calls_to(&e, 0x0091_6880), vec![arguments]);
    }

    #[test]
    fn test_high_process_enter_combat_declines() {
        // Finishing a combat package: nothing happens.
        let mut e = Engine::new();
        let process = enter_combat_scene(&mut e, 0, 1, 0);
        e.call_log = Some(vec![]);
        let mut arguments = vec![process];
        arguments.extend(1..=13u32);
        assert!(!e.call(0x0090_01c0, &arguments).bool());
        assert!(calls_to(&e, 0x0091_6880).is_empty());
        // A run-once package of type 0x10.
        let mut e = Engine::new();
        let process = enter_combat_scene(&mut e, 0x9000, 0, 0x10);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0090_01c0, &arguments_for(process)).bool());
        assert!(calls_to(&e, 0x0091_6880).is_empty());
        assert_eq!(calls_to(&e, 0x0041_ca90), vec![vec![0x9000]]);
        // A run-once package of another type does not stop it; the armour flag
        // being clear, slot 0x848 is not called.
        let mut e = Engine::new();
        let process = enter_combat_scene(&mut e, 0x9000, 0, 0x11);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0090_01c0, &arguments_for(process)).bool());
        assert!(calls_to(&e, 0x00aa_e030).is_empty());
        assert_eq!(calls_to(&e, 0x0091_6880).len(), 1);
    }

    /// The 14 argument words of `EnterCombat` for `process`.
    fn arguments_for(process: u32) -> Vec<u32> {
        let mut arguments = vec![process];
        arguments.extend(1..=13u32);
        arguments
    }

    /// A process for `ProcessSummonCreatureDefend` with its four slots.
    fn summon_process(e: &mut Engine, commander: u32) -> u32 {
        let process = slots_object(
            e,
            0x500,
            &[
                (0x288, 0x00aa_e040, 0),
                (0x33c, 0x00aa_e050, 0),
                (0x7cc, 0x00aa_e060, 0),
                (0x298, 0x00aa_e070, 0),
            ],
        );
        e.mem.set_u32(process + 0x158, commander);
        process
    }

    #[test]
    fn test_high_process_process_summon_creature_defend() {
        // No commander: the procedure index is advanced.
        let mut e = Engine::new();
        list_doubles(&mut e);
        let player = e.mem.alloc(0x40);
        global(&mut e, 0x011d_ea3c, player);
        stub(&mut e, 0x0049_3bb0, 0);
        stub(&mut e, 0x0095_3c50, 0);
        let process = summon_process(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_0020, &args![process, 0xa1u32]);
        assert_eq!(calls_to(&e, 0x00aa_e040), vec![vec![process, 0xa1, 1]]);
        // A commander that is neither the player nor player-like: slot 0x7cc.
        let commander = e.mem.alloc(0x40);
        e.mem.set_u32(process + 0x158, commander);
        e.call_log = Some(vec![]);
        e.call(0x0090_0020, &args![process, 0xa1u32]);
        assert_eq!(calls_to(&e, 0x00aa_e060), vec![vec![process, 0xa1]]);
        // The player, not in combat: slot 0x298.
        e.mem.set_u32(process + 0x158, player);
        e.call_log = Some(vec![]);
        e.call(0x0090_0020, &args![process, 0xa1u32]);
        assert_eq!(
            calls_to(&e, 0x00aa_e070),
            vec![vec![process, 0xa1, 0, 0x101, 0]]
        );
        assert!(calls_to(&e, 0x00aa_e050).is_empty());
        // A player-like commander other than the player does nothing more.
        stub(&mut e, 0x0049_3bb0, 1);
        e.mem.set_u32(process + 0x158, commander);
        e.call_log = Some(vec![]);
        e.call(0x0090_0020, &args![process, 0xa1u32]);
        assert!(calls_to(&e, 0x00aa_e060).is_empty());
        assert!(calls_to(&e, 0x00aa_e050).is_empty());
    }

    #[test]
    fn test_high_process_process_summon_creature_defend_enters_combat_for_the_player() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let player = e.mem.alloc(0x40);
        global(&mut e, 0x011d_ea3c, player);
        stub(&mut e, 0x0049_3bb0, 0);
        // The player is in combat.
        stub(&mut e, 0x0095_3c50, 1);
        let actor_one = slots_object(&mut e, 0x120, &[(0x100, 0x00aa_e080, 1)]);
        let not_actor = slots_object(&mut e, 0x120, &[(0x100, 0x00aa_e090, 0)]);
        let actor_two = slots_object(&mut e, 0x120, &[(0x100, 0x00aa_e0a0, 1)]);
        let found = make_list(&mut e, &[actor_one, not_actor, actor_two]);
        e.register_double(0x0097_1c30, move |_, _| ret(found));
        let process = summon_process(&mut e, player);
        e.call_log = Some(vec![]);
        e.call(0x0090_0020, &args![process, 0xa1u32]);
        assert_eq!(
            calls_to(&e, 0x0097_1c30),
            vec![vec![0x011e_0e80, player, 0x12, 1]]
        );
        assert_eq!(
            calls_to(&e, 0x00aa_e050),
            vec![
                vec![process, 0xa1, actor_one, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
                vec![process, 0xa1, actor_two, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
            ]
        );
        assert!(calls_to(&e, 0x00aa_e060).is_empty());
        assert!(calls_to(&e, 0x00aa_e070).is_empty());
    }

    #[test]
    fn test_high_process_get_actor_value() {
        let mut e = Engine::new();
        let table = vtable(&mut e, &[(0x39c, 0x00aa_e0b0)]);
        let process = object(&mut e, 0x500, table);
        stub_float(&mut e, 0x00aa_e0b0, 3.75);
        stub_float(&mut e, 0x0040_4040, 3.0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0090_02b0, &args![process, 1u32, 2u32, 3u32]).i32(),
            3
        );
        assert_eq!(calls_to(&e, 0x00aa_e0b0), vec![vec![process, 1, 2, 3]]);
        assert_eq!(calls_to(&e, 0x0040_4040), vec![vec![3.75f32.to_bits()]]);
        // Rounded down: a negative value goes to the next lower integer.
        stub_float(&mut e, 0x0040_4040, -4.0);
        assert_eq!(
            e.call(0x0090_02b0, &args![process, 1u32, 2u32, 3u32]).i32(),
            -4
        );
    }

    #[test]
    fn test_fn_00900400_and_fn_009003e0_read_the_cache_entries() {
        let mut e = Engine::new();
        let cache = e.mem.alloc(0x40);
        e.mem.set_u8(cache + 3 * 8, 1);
        e.mem.set_f32(cache + 3 * 8 + 4, 2.5);
        assert_eq!(e.call(0x0090_0400, &args![cache, 3u32]).u8(), 1);
        assert_eq!(e.call(0x0090_0400, &args![cache, 2u32]).u8(), 0);
        assert_eq!(e.call(0x0090_03e0, &args![cache, 3u32]).f32(), 2.5);
    }

    /// Doubles for the value cache: `operator new`, the cache constructor,
    /// the store `008c6f00(index, value)` that clears the entry's flag and
    /// keeps the value, and the invalidation `008c6f40(index)` that sets it.
    fn cache_doubles(e: &mut Engine) {
        e.register_double(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register_double(0x008c_6e90, |_, a| ret(a[0]));
        e.register_double(0x008c_6f00, |e, a| {
            e.mem.set_u8(a[0] + a[1] * 8, 0);
            e.mem.set_u32(a[0] + a[1] * 8 + 4, a[2]);
            ret(0)
        });
        e.register_double(0x008c_6f40, |e, a| {
            e.mem.set_u8(a[0] + a[1] * 8, 1);
            ret(0)
        });
    }

    #[test]
    fn test_high_process_get_actor_float_value() {
        let mut e = Engine::new();
        cache_doubles(&mut e);
        stub_float(&mut e, 0x0092_ce00, 7.5);
        let process = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        // The cache does not exist: it is created (0x268 bytes) and, its entries
        // being zero, the value is returned from it without being computed.
        let value = e
            .call(0x0090_02f0, &args![process, 0xa1u32, 4u32, 0u32])
            .f32();
        assert_eq!(value, 0.0);
        assert_eq!(calls_to(&e, 0x0040_1000), vec![vec![0x268]]);
        let cache = e.mem.u32(process + 0x428);
        assert_ne!(cache, 0);
        assert!(calls_to(&e, 0x0092_ce00).is_empty());
        // A flagged entry is computed, stored and returned.
        e.mem.set_u8(cache + 4 * 8, 1);
        let value = e
            .call(0x0090_02f0, &args![process, 0xa1u32, 4u32, 9u32])
            .f32();
        assert_eq!(value, 7.5);
        assert_eq!(calls_to(&e, 0x0092_ce00), vec![vec![process, 0xa1, 4, 9]]);
        assert_eq!(
            calls_to(&e, 0x008c_6f00),
            vec![vec![cache, 4, 7.5f32.to_bits()]]
        );
        // Now cached: no second computation, no second allocation.
        let value = e
            .call(0x0090_02f0, &args![process, 0xa1u32, 4u32, 9u32])
            .f32();
        assert_eq!(value, 7.5);
        assert_eq!(calls_to(&e, 0x0092_ce00).len(), 1);
        assert_eq!(calls_to(&e, 0x0040_1000).len(), 1);
    }

    #[test]
    fn test_actor_value_modifiers_invalidate_the_cache() {
        // (entry address, test double result, ...): each modifier runs its
        // MiddleLowProcess function and then marks the cache entry.
        let entries: [(u32, u32, bool); 4] = [
            (0x0090_0420, 0x0092_ce70, false),
            (0x0090_04f0, 0x0092_cec0, true),
            (0x0090_05d0, 0x0090_7430, false),
            (0x0090_06a0, 0x0092_cf30, true),
        ];
        for (entry, low_level, float_arg) in entries {
            let mut e = Engine::new();
            cache_doubles(&mut e);
            stub(&mut e, low_level, 0);
            stub(&mut e, 0x0040_6d70, 0);
            let process = e.mem.alloc(0x500);
            e.call_log = Some(vec![]);
            let third = if float_arg { 2.5f32.to_bits() } else { 9 };
            e.call(entry, &args![process, 1u32, 5u32, third]);
            assert_eq!(calls_to(&e, low_level), vec![vec![process, 1, 5, third]]);
            assert_eq!(calls_to(&e, 0x0040_6d70), vec![vec![5, 0x100]]);
            let cache = e.mem.u32(process + 0x428);
            assert_ne!(cache, 0, "{entry:08x}");
            assert_eq!(calls_to(&e, 0x008c_6f40), vec![vec![cache, 5]]);
            // When the index test holds, the cache is not touched.
            let mut e = Engine::new();
            cache_doubles(&mut e);
            stub(&mut e, low_level, 0);
            stub(&mut e, 0x0040_6d70, 1);
            let process = e.mem.alloc(0x500);
            e.call_log = Some(vec![]);
            e.call(entry, &args![process, 1u32, 5u32, third]);
            assert_eq!(e.mem.u32(process + 0x428), 0);
            assert!(calls_to(&e, 0x008c_6f40).is_empty());
        }
    }

    #[test]
    fn test_high_process_set_cached_actor_value_out_of_date() {
        let mut e = Engine::new();
        cache_doubles(&mut e);
        let process = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        e.call(0x0090_0780, &args![process, 6u32]);
        let cache = e.mem.u32(process + 0x428);
        assert_ne!(cache, 0);
        assert_eq!(calls_to(&e, 0x008c_6f40), vec![vec![cache, 6]]);
        assert_eq!(e.mem.u8(cache + 6 * 8), 1);
        // The existing cache is reused.
        e.call(0x0090_0780, &args![process, 7u32]);
        assert_eq!(e.mem.u32(process + 0x428), cache);
        assert_eq!(calls_to(&e, 0x0040_1000).len(), 1);
    }

    /// A follower object: its slots `0x25c`, `0x100`, `0x2b4`, `0x22c`,
    /// `0x2b0` and `0x2b8` are doubles at `base`, `base + 4`, ... (`0x2b4`
    /// answers `wait` in `ST0`).
    fn follower(e: &mut Engine, base: u32, is_actor: u32, wait: f64) -> u32 {
        let table = vtable(
            e,
            &[
                (0x25c, base),
                (0x100, base + 4),
                (0x2b4, base + 8),
                (0x22c, base + 12),
                (0x2b0, base + 16),
                (0x2b8, base + 20),
            ],
        );
        stub(e, base, 0);
        stub(e, base + 4, is_actor);
        stub_float(e, base + 8, wait);
        stub(e, base + 12, 0);
        stub(e, base + 16, 0);
        stub(e, base + 20, 0);
        object(e, 0x40, table)
    }

    /// Doubles for `UpdateFollowers` and the leader's extra data list; the
    /// follower data's list holds `followers`. Returns `(leader, data)`.
    fn followers_scene(e: &mut Engine, followers: &[u32]) -> (u32, u32) {
        list_doubles(e);
        fade_constants(e);
        global(e, 0x011d_ea3c, 0);
        let leader = e.mem.alloc(0x100);
        let extra = 0x7100u32;
        e.register_double(0x005d_43c0, move |_, _| ret(extra));
        let data = e.mem.alloc(0x20);
        let list = make_list(e, followers);
        e.mem.set_u32(data + 0xc, list);
        e.register_double(0x0042_2700, move |_, _| ret(data));
        // The work list takes the item the slot holds.
        e.register_double(0x0090_5820, |e, a| {
            let item = e.mem.u32(a[1]);
            let list = a[0];
            if e.mem.u32(list) == 0 {
                e.mem.set_u32(list, item);
            } else {
                let mut last = list;
                while e.mem.u32(last + 4) != 0 {
                    last = e.mem.u32(last + 4);
                }
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, item);
                e.mem.set_u32(last + 4, node);
            }
            ret(0)
        });
        for addr in [
            0x0042_2690,
            0x0042_2720,
            0x0047_02f0,
            0x0043_7bd0,
            0x0069_3d50,
        ] {
            stub(e, addr, 0);
        }
        stub(e, 0x0071_7e50, 0x9000);
        stub(e, 0x005b_e5c0, 8);
        stub_float(e, 0x0084_d030, 2.0);
        stub_float(e, 0x0086_7da0, 10.0);
        stub(e, 0x008d_8520, 0x9200);
        global(e, 0x0108_4838, 3600.0f32.to_bits());
        global_f64(e, 0x0102_90b0, 0.25);
        (leader, data)
    }

    #[test]
    fn test_high_process_update_followers() {
        let mut e = Engine::new();
        let dropped = follower(&mut e, 0x00aa_e100, 1, 0.0);
        let traveller = follower(&mut e, 0x00aa_e140, 1, 0.0);
        let waiter = follower(&mut e, 0x00aa_e180, 1, -1.0);
        let second_waiter = follower(&mut e, 0x00aa_e1c0, 1, -2.0);
        let idle = follower(&mut e, 0x00aa_e200, 1, 5.0);
        let player = follower(&mut e, 0x00aa_e240, 1, 0.0);
        let (leader, _) = followers_scene(
            &mut e,
            &[dropped, traveller, waiter, second_waiter, idle, player],
        );
        global(&mut e, 0x011d_ea3c, player);
        e.register_double(0x008b_c860, move |_, a| ret(u32::from(a[0] != dropped)));
        e.register_double(0x0093_1850, move |_, a| {
            ret(if a[0] == traveller { 3 } else { 0 })
        });
        let process = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        // A non-positive length is replaced by the float at 01084838.
        e.call(0x0090_0830, &args![process, leader, 0.0f32]);
        // The follower that could not follow is removed from the extra data.
        assert_eq!(calls_to(&e, 0x0042_2690), vec![vec![0x7100, dropped]]);
        assert!(calls_to(&e, 0x00aa_e100).is_empty());
        // The traveller (process type 3) gets the hour - 1 on its process and
        // the replaced length.
        assert_eq!(
            calls_to(&e, 0x0069_3d50),
            vec![vec![0x9200, 9.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x00aa_e140),
            vec![vec![traveller, 3600.0f32.to_bits()]]
        );
        // The waiters get the accumulated wait: 0, then clock (2.0) * (8 *
        // 0.25) = 4, then 8.
        assert_eq!(
            calls_to(&e, 0x00aa_e180),
            vec![vec![waiter, 0.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x00aa_e190),
            vec![vec![waiter, 0.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x00aa_e1d0),
            vec![vec![second_waiter, 4.0f32.to_bits()]]
        );
        // A follower that is not waiting is told to stop waiting.
        assert_eq!(calls_to(&e, 0x00aa_e214), vec![vec![idle]]);
        assert!(calls_to(&e, 0x00aa_e1d4).is_empty());
        // The player is skipped; the work lists are cleared and freed; the
        // follower list is not empty, so the follower extra stays.
        assert!(calls_to(&e, 0x00aa_e240).is_empty());
        assert_eq!(calls_to(&e, 0x0047_02f0).len(), 2);
        assert!(calls_to(&e, 0x0042_2720).is_empty());
    }

    #[test]
    fn test_high_process_update_followers_empty_list_and_missing_data() {
        // No followers left: the follower extra is removed.
        let mut e = Engine::new();
        let (leader, _) = followers_scene(&mut e, &[]);
        let process = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        e.call(0x0090_0830, &args![process, leader, 1.0f32]);
        assert_eq!(calls_to(&e, 0x0042_2720), vec![vec![0x7100]]);
        // No extra data list: nothing at all.
        let mut e = Engine::new();
        let (leader, _) = followers_scene(&mut e, &[]);
        stub(&mut e, 0x005d_43c0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_0830, &args![process, leader, 1.0f32]);
        assert!(calls_to(&e, 0x0042_2700).is_empty());
        assert!(calls_to(&e, 0x0040_1000).is_empty());
        // Extra data without follower data: nothing either.
        let mut e = Engine::new();
        let (leader, _) = followers_scene(&mut e, &[]);
        stub(&mut e, 0x0042_2700, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_0830, &args![process, leader, 1.0f32]);
        assert!(calls_to(&e, 0x0040_1000).is_empty());
        assert!(calls_to(&e, 0x0042_2720).is_empty());
    }

    /// A spell object whose sub-object at +0x18 answers `kind` to its slot
    /// `0x18` (a double at `target`).
    fn spell(e: &mut Engine, target: u32, kind: u32) -> u32 {
        stub(e, target, kind);
        let table = vtable(e, &[(0x18, target)]);
        let spell = e.mem.alloc(0x40);
        e.mem.set_u32(spell + 0x18, table);
        spell
    }

    #[test]
    fn test_high_process_get_leveled_spells() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let skipped = spell(&mut e, 0x00aa_e300, 4);
        let flagged = spell(&mut e, 0x00aa_e310, 1);
        let plain = spell(&mut e, 0x00aa_e320, 2);
        stub(&mut e, 0x0041_81e0, 0x6000);
        let entries = make_list(&mut e, &[0xe1]);
        stub(&mut e, 0x0048_d150, entries);
        let expanded = make_list(&mut e, &[skipped, flagged, plain]);
        stub(&mut e, 0x0050_c1d0, expanded);
        stub(&mut e, 0x0047_02f0, 0);
        let process = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        let list = e.call(0x0090_0c20, &args![process, 0xa1u32, 0u32]).u32();
        assert_eq!(e.mem.u32(process + 0x3b4), list);
        // Type 4 is dropped and type 1 only with the flag.
        assert_eq!(list_items(&e, list), vec![plain]);
        assert_eq!(calls_to(&e, 0x0048_d150), vec![vec![0x6000 + 0x7c]]);
        assert_eq!(calls_to(&e, 0x0050_c1d0), vec![vec![0xe1, 0xa1]]);
        // The expansion list is cleared and freed.
        assert_eq!(calls_to(&e, 0x0047_0470), vec![vec![expanded]]);
        assert_eq!(calls_to(&e, 0x0047_02f0), vec![vec![expanded, 1]]);
        // The list exists now: it is returned as it is.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0090_0c20, &args![process, 0xa1u32, 1u32]).u32(),
            list
        );
        assert!(calls_to(&e, 0x0048_d150).is_empty());
    }

    #[test]
    fn test_high_process_get_leveled_spells_with_the_flag_and_without_data() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let flagged = spell(&mut e, 0x00aa_e330, 1);
        let plain = spell(&mut e, 0x00aa_e340, 2);
        stub(&mut e, 0x0041_81e0, 0x6000);
        let entries = make_list(&mut e, &[0xe1]);
        stub(&mut e, 0x0048_d150, entries);
        let expanded = make_list(&mut e, &[flagged, plain]);
        stub(&mut e, 0x0050_c1d0, expanded);
        stub(&mut e, 0x0047_02f0, 0);
        let process = e.mem.alloc(0x500);
        let list = e.call(0x0090_0c20, &args![process, 0xa1u32, 1u32]).u32();
        assert_eq!(list_items(&e, list), vec![flagged, plain]);
        // A null actor, or one without base data, gives null.
        let process = e.mem.alloc(0x500);
        assert_eq!(e.call(0x0090_0c20, &args![process, 0u32, 1u32]).u32(), 0);
        stub(&mut e, 0x0041_81e0, 0);
        assert_eq!(e.call(0x0090_0c20, &args![process, 0xa1u32, 1u32]).u32(), 0);
        // A process without matching spells keeps a null list.
        stub(&mut e, 0x0041_81e0, 0x6000);
        let empty = make_list(&mut e, &[]);
        stub(&mut e, 0x0048_d150, empty);
        assert_eq!(e.call(0x0090_0c20, &args![process, 0xa1u32, 1u32]).u32(), 0);
    }

    #[test]
    fn test_healing_flag_accessors() {
        let mut e = Engine::new();
        let process = e.mem.alloc(0x500);
        e.call(0x0090_0e40, &args![process, 7u32]);
        e.call(0x0090_0e80, &args![process, 9u32]);
        assert_eq!(e.mem.u32(process + 0x3ac), 7);
        assert_eq!(e.mem.u32(process + 0x3b0), 9);
        assert_eq!(e.call(0x0090_0e60, &args![process]).u32(), 7);
        assert_eq!(e.call(0x0090_0ea0, &args![process]).u32(), 9);
    }

    #[test]
    fn test_high_process_set_default_head_track_target() {
        let mut e = Engine::new();
        stub(&mut e, 0x0090_14d0, 0);
        stub(&mut e, 0x0056_4db0, 0);
        let process = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        e.call(0x0090_0ec0, &args![process, 0x7001u32]);
        assert_eq!(e.mem.u32(process + 0x3f8), 0x7001);
        assert_eq!(e.mem.u8(process + 0x410), 1);
        assert_eq!(calls_to(&e, 0x0090_14d0), vec![vec![process]]);
        assert_eq!(calls_to(&e, 0x0056_4db0), vec![vec![0x7001, 1]]);
        // A null target is stored and flagged without being marked.
        e.call_log = Some(vec![]);
        e.call(0x0090_0ec0, &args![process, 0u32]);
        assert_eq!(e.mem.u32(process + 0x3f8), 0);
        assert_eq!(e.mem.u8(process + 0x410), 1);
        assert_eq!(calls_to(&e, 0x0090_14d0), vec![vec![process]]);
        assert!(calls_to(&e, 0x0056_4db0).is_empty());
    }

    #[test]
    fn test_high_process_set_action_head_track_target() {
        let mut e = Engine::new();
        stub(&mut e, 0x0090_14d0, 0);
        stub(&mut e, 0x0056_4db0, 0);
        let process = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        e.call(0x0090_0f00, &args![process, 0x7002u32]);
        assert_eq!(e.mem.u32(process + 0x3fc), 0x7002);
        assert_eq!(e.mem.u8(process + 0x411), 1);
        assert_eq!(calls_to(&e, 0x0090_14d0), vec![vec![process]]);
        assert_eq!(calls_to(&e, 0x0056_4db0), vec![vec![0x7002, 1]]);
        // A null target clears the flag and calls nothing.
        e.call_log = Some(vec![]);
        e.call(0x0090_0f00, &args![process, 0u32]);
        assert_eq!(e.mem.u32(process + 0x3fc), 0);
        assert_eq!(e.mem.u8(process + 0x411), 0);
        assert!(calls_to(&e, 0x0090_14d0).is_empty());
        assert!(calls_to(&e, 0x0056_4db0).is_empty());
    }

    // ---- Head-track targets, dialogue and guard (008f3fe0 range, third part) ----------

    /// Registers the game-setting accessor `00403e20` as the game does (the value is the
    /// float at `this + 4`) and maps a setting object at `object`.
    fn put_setting(e: &mut Engine, object: u32, value: f32) {
        e.map(object, 8);
        e.mem.set_f32(object + 4, value);
        e.register_double(0x0040_3e20, |_, a| ret(a[0] + 4));
    }

    /// Maps a `double` constant.
    fn put_double(e: &mut Engine, addr: u32, value: f64) {
        e.map(addr, 8);
        e.mem.set_f64(addr, value);
    }

    /// Maps a `float` constant.
    fn put_float(e: &mut Engine, addr: u32, value: f32) {
        global(e, addr, value.to_bits());
    }

    const HEAD_WORDS: u32 = 0x3f8;
    const HEAD_FLAGS: u32 = 0x410;

    fn check_head_track_setter(setter: u32, index: u32) {
        let mut e = Engine::new();
        stub(&mut e, 0x0056_4db0, 0);
        let proc = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        e.call(setter, &args![proc, 0x7001u32]);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS + 4 * index), 0x7001);
        assert_eq!(e.mem.u8(proc + HEAD_FLAGS + index), 1);
        assert_eq!(e.mem.u32(proc + 0x41c), 0x7001);
        assert_eq!(calls_to(&e, 0x0056_4db0), vec![vec![0x7001, 1]]);
        e.call(setter, &args![proc, 0u32]);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS + 4 * index), 0);
        assert_eq!(e.mem.u8(proc + HEAD_FLAGS + index), 0);
        assert_eq!(calls_to(&e, 0x0056_4db0).len(), 1);
    }

    #[test]
    fn test_high_process_set_script_head_track_target() {
        check_head_track_setter(0x0090_0f50, 2);
    }

    #[test]
    fn test_high_process_set_combat_head_track_target() {
        check_head_track_setter(0x0090_0fa0, 3);
    }

    #[test]
    fn test_high_process_set_dialog_head_track_target() {
        check_head_track_setter(0x0090_1050, 4);
    }

    #[test]
    fn test_high_process_set_use_weapon_head_track_target() {
        check_head_track_setter(0x0090_10a0, 5);
    }

    #[test]
    fn test_high_process_set_head_track_target() {
        let mut e = Engine::new();
        stub(&mut e, 0x0056_4db0, 0);
        let proc = e.mem.alloc(0x500);
        e.call(0x0090_0ff0, &args![proc, 3u32, 0x7003u32]);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS + 12), 0x7003);
        assert_eq!(e.mem.u8(proc + HEAD_FLAGS + 3), 1);
        e.call(0x0090_0ff0, &args![proc, 3u32, 0u32]);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS + 12), 0);
        assert_eq!(e.mem.u8(proc + HEAD_FLAGS + 3), 0);
    }

    #[test]
    fn test_high_process_clear_default_head_track_target() {
        let mut e = Engine::new();
        let proc = e.mem.alloc(0x500);
        e.mem.set_u32(proc + HEAD_WORDS, 0x7000);
        e.mem.set_u8(proc + HEAD_FLAGS, 1);
        e.mem.set_u32(proc + 0x41c, 0x7000);
        e.call(0x0090_10f0, &args![proc]);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS), 0);
        assert_eq!(e.mem.u8(proc + HEAD_FLAGS), 0);
        assert_eq!(e.mem.u32(proc + 0x41c), 0);
    }

    /// A clear function with a restore flag: slot `index` is emptied; with the flag the old
    /// target moves to slot 0 and the process's slot `0x268` gets the setting's float,
    /// without it the current target is recomputed.
    fn check_head_track_clear(clear: u32, index: u32) {
        let mut e = Engine::new();
        put_setting(&mut e, 0x011c_dbb8, 2.5);
        let table = vtable(&mut e, &[(0x268, 0x00aa_f268)]);
        let proc = object(&mut e, 0x500, table);
        stub(&mut e, 0x00aa_f268, 0);
        e.mem.set_u32(proc + HEAD_WORDS + 4 * index, 0x7002);
        e.mem.set_u8(proc + HEAD_FLAGS + index, 1);
        e.call_log = Some(vec![]);
        e.call(clear, &args![proc, 1u32]);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS + 4 * index), 0);
        assert_eq!(e.mem.u8(proc + HEAD_FLAGS + index), 0);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS), 0x7002);
        assert_eq!(
            calls_to(&e, 0x00aa_f268),
            vec![vec![proc, 2.5f32.to_bits()]]
        );
        // Without the flag nothing is handed on and the cached target follows.
        e.mem.set_u32(proc + HEAD_WORDS + 4 * index, 0x7003);
        e.mem.set_u8(proc + HEAD_FLAGS + index, 1);
        e.mem.set_u32(proc + 0x41c, 0x7003);
        e.call_log = Some(vec![]);
        e.call(clear, &args![proc, 0u32]);
        assert!(calls_to(&e, 0x00aa_f268).is_empty());
        assert_eq!(e.mem.u32(proc + 0x41c), 0);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS), 0x7002);
    }

    #[test]
    fn test_high_process_clear_action_head_track_target() {
        check_head_track_clear(0x0090_1120, 1);
    }

    #[test]
    fn test_high_process_clear_script_head_track_target() {
        check_head_track_clear(0x0090_1190, 2);
    }

    #[test]
    fn test_high_process_clear_dialog_head_track_target() {
        check_head_track_clear(0x0090_1230, 4);
    }

    #[test]
    fn test_high_process_clear_use_weapon_head_track_target() {
        check_head_track_clear(0x0090_12a0, 5);
    }

    #[test]
    fn test_high_process_clear_combat_head_track_target() {
        let mut e = Engine::new();
        let proc = e.mem.alloc(0x500);
        e.mem.set_u32(proc + HEAD_WORDS + 12, 0x7004);
        e.mem.set_u8(proc + HEAD_FLAGS + 3, 1);
        e.mem.set_u32(proc + 0x41c, 0x7004);
        e.call(0x0090_1200, &args![proc]);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS + 12), 0);
        assert_eq!(e.mem.u8(proc + HEAD_FLAGS + 3), 0);
        assert_eq!(e.mem.u32(proc + 0x41c), 0);
    }

    #[test]
    fn test_high_process_clear_all_head_track_targets() {
        let mut e = Engine::new();
        let proc = e.mem.alloc(0x500);
        for index in 0..6 {
            e.mem.set_u32(proc + HEAD_WORDS + 4 * index, 0x7000 + index);
            e.mem.set_u8(proc + HEAD_FLAGS + index, 1);
        }
        e.mem.set_u32(proc + 0x41c, 0x7005);
        e.call(0x0090_1310, &args![proc]);
        for index in 0..6 {
            assert_eq!(e.mem.u32(proc + HEAD_WORDS + 4 * index), 0);
            assert_eq!(e.mem.u8(proc + HEAD_FLAGS + index), 0);
        }
        assert_eq!(e.mem.u32(proc + 0x41c), 0);
    }

    #[test]
    fn test_high_process_clear_path_look_target() {
        let mut e = Engine::new();
        let proc = e.mem.alloc(0x500);
        e.mem.set_u32(proc + 0x2ac, 0x7001);
        e.call(0x0090_1360, &args![proc, 0x7002u32]);
        assert_eq!(e.mem.u32(proc + 0x2ac), 0x7001);
        e.call(0x0090_1360, &args![proc, 0x7001u32]);
        assert_eq!(e.mem.u32(proc + 0x2ac), 0);
    }

    #[test]
    fn test_high_process_remove_head_track_target() {
        let mut e = Engine::new();
        let proc = e.mem.alloc(0x500);
        for index in 0..6 {
            e.mem.set_u8(proc + HEAD_FLAGS + index, 1);
            e.mem.set_u32(proc + HEAD_WORDS + 4 * index, 0x7000);
        }
        e.mem.set_u32(proc + HEAD_WORDS + 8, 0x7777);
        e.mem.set_u32(proc + 0x41c, 0x7000);
        e.call(0x0090_1390, &args![proc, 0x7000u32]);
        for index in [0u32, 1, 3, 4, 5] {
            assert_eq!(e.mem.u32(proc + HEAD_WORDS + 4 * index), 0);
            assert_eq!(e.mem.u8(proc + HEAD_FLAGS + index), 0);
        }
        assert_eq!(e.mem.u32(proc + HEAD_WORDS + 8), 0x7777);
        assert_eq!(e.mem.u8(proc + HEAD_FLAGS + 2), 1);
        assert_eq!(e.mem.u32(proc + 0x41c), 0x7777);
    }

    #[test]
    fn test_high_process_can_set_default_head_track_target() {
        let mut e = Engine::new();
        put_double(&mut e, 0x0101_2060, 0.0);
        let table = vtable(&mut e, &[(0x264, 0x00aa_f264)]);
        let proc = object(&mut e, 0x500, table);
        stub_float(&mut e, 0x00aa_f264, -1.0);
        assert!(e.call(0x0090_1400, &args![proc]).bool());
        stub_float(&mut e, 0x00aa_f264, 0.0);
        assert!(!e.call(0x0090_1400, &args![proc]).bool());
        // A flagged slot 1 to 5 refuses without asking; slot 0 does not count.
        stub_float(&mut e, 0x00aa_f264, -1.0);
        e.mem.set_u8(proc + HEAD_FLAGS, 1);
        assert!(e.call(0x0090_1400, &args![proc]).bool());
        e.mem.set_u8(proc + HEAD_FLAGS + 1, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0090_1400, &args![proc]).bool());
        assert!(calls_to(&e, 0x00aa_f264).is_empty());
    }

    #[test]
    fn test_high_process_can_set_action_head_track_target() {
        let mut e = Engine::new();
        let proc = e.mem.alloc(0x500);
        e.mem.set_u8(proc + HEAD_FLAGS, 1);
        assert!(e.call(0x0090_1460, &args![proc]).bool());
        e.mem.set_u8(proc + HEAD_FLAGS + 5, 1);
        assert!(!e.call(0x0090_1460, &args![proc]).bool());
    }

    #[test]
    fn test_high_process_can_set_combat_head_track_target() {
        let mut e = Engine::new();
        let proc = e.mem.alloc(0x500);
        assert!(e.call(0x0090_14a0, &args![proc]).bool());
        e.mem.set_u8(proc + HEAD_FLAGS + 4, 1);
        assert!(!e.call(0x0090_14a0, &args![proc]).bool());
    }

    #[test]
    fn test_high_process_on_new_head_track_target() {
        let mut e = Engine::new();
        let proc = e.mem.alloc(0x500);
        e.mem.set_u8(proc + HEAD_FLAGS + 1, 1);
        e.mem.set_u32(proc + HEAD_WORDS + 4, 0x7001);
        e.mem.set_u8(proc + HEAD_FLAGS + 3, 1);
        e.mem.set_u32(proc + HEAD_WORDS + 12, 0x7003);
        e.mem.set_u8(proc + 0x420, 1);
        e.call(0x0090_14d0, &args![proc]);
        assert_eq!(e.mem.u32(proc + 0x41c), 0x7003);
        assert_eq!(e.mem.u8(proc + 0x420), 0);
        // Unchanged: the byte stays.
        e.mem.set_u8(proc + 0x420, 1);
        e.call(0x0090_14d0, &args![proc]);
        assert_eq!(e.mem.u8(proc + 0x420), 1);
        // Nothing flagged: the cache is emptied.
        e.mem.set_u8(proc + HEAD_FLAGS + 1, 0);
        e.mem.set_u8(proc + HEAD_FLAGS + 3, 0);
        e.call(0x0090_14d0, &args![proc]);
        assert_eq!(e.mem.u32(proc + 0x41c), 0);
        assert_eq!(e.mem.u8(proc + 0x420), 0);
    }

    #[test]
    fn test_high_process_update_head_track_targets() {
        let mut e = Engine::new();
        e.register_double(0x0044_0d80, |_, a| ret((a[0] == 0x7002) as u32));
        let proc = e.mem.alloc(0x500);
        // Slot 1 holds a target that 00440d80 rejects, slot 2 one it accepts, slot 3 is
        // flagged with a null target, slot 4 holds a word but is not flagged.
        e.mem.set_u8(proc + HEAD_FLAGS + 1, 1);
        e.mem.set_u32(proc + HEAD_WORDS + 4, 0x7001);
        e.mem.set_u8(proc + HEAD_FLAGS + 2, 1);
        e.mem.set_u32(proc + HEAD_WORDS + 8, 0x7002);
        e.mem.set_u8(proc + HEAD_FLAGS + 3, 1);
        e.mem.set_u32(proc + HEAD_WORDS + 16, 0x7004);
        e.mem.set_u32(proc + 0x41c, 0x7002);
        e.call(0x0090_1550, &args![proc]);
        assert_eq!(e.mem.u8(proc + HEAD_FLAGS + 1), 1);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS + 4), 0x7001);
        assert_eq!(e.mem.u8(proc + HEAD_FLAGS + 2), 0);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS + 8), 0);
        assert_eq!(e.mem.u8(proc + HEAD_FLAGS + 3), 0);
        assert_eq!(e.mem.u32(proc + HEAD_WORDS + 16), 0x7004);
        assert_eq!(e.mem.u32(proc + 0x41c), 0x7001);
    }

    #[test]
    fn test_high_process_get_current_head_track_target_and_type() {
        let mut e = Engine::new();
        let proc = e.mem.alloc(0x500);
        assert_eq!(e.call(0x0090_15f0, &args![proc]).u32(), 0);
        assert_eq!(e.call(0x0090_1660, &args![proc]).u32(), 0);
        e.mem.set_u8(proc + HEAD_FLAGS + 2, 1);
        e.mem.set_u32(proc + HEAD_WORDS + 8, 0x7002);
        e.mem.set_u32(proc + HEAD_WORDS + 12, 0x7003);
        assert_eq!(e.call(0x0090_15f0, &args![proc]).u32(), 0x7002);
        assert_eq!(e.call(0x0090_1660, &args![proc]).u32(), 2);
        assert_eq!(e.call(0x0090_1640, &args![proc, 3u32]).u32(), 0x7003);
    }

    #[test]
    fn test_high_process_get_current_head_track_type_string() {
        let mut e = Engine::new();
        let table = vtable(&mut e, &[(0x67c, 0x00aa_f67c)]);
        let proc = object(&mut e, 0x500, table);
        for (kind, text) in [
            (0u32, 0x0104_e6a0u32),
            (1, 0x0108_83ec),
            (2, 0x0108_83e4),
            (3, 0x0108_83dc),
            (4, 0x0108_83d4),
            (5, 0x0101_a6c0),
        ] {
            stub(&mut e, 0x00aa_f67c, kind);
            assert_eq!(e.call(0x0090_16a0, &args![proc]).u32(), text);
        }
    }

    #[test]
    fn test_high_process_set_detection_modifier() {
        let mut e = Engine::new();
        let table = vtable(&mut e, &[(0x4a8, 0x00aa_f4a8)]);
        let proc = object(&mut e, 0x500, table);
        stub(&mut e, 0x00aa_f4a8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_1710, &args![proc, 1.25f32]);
        assert_eq!(e.mem.f32(proc + 0x3bc), 1.25);
        assert_eq!(calls_to(&e, 0x00aa_f4a8), vec![vec![proc]]);
    }

    #[test]
    fn test_fn_00901740() {
        let mut e = Engine::new();
        let player = 0x7f00u32;
        global(&mut e, 0x011d_ea3c, player);
        stub(&mut e, 0x0061_a2d0, 0x9001);
        stub(&mut e, 0x0057_bd60, 0);
        let process_table = vtable(&mut e, &[(0x2a4, 0x00aa_f2a4), (0x1dc, 0x00aa_f1dc)]);
        let their_process = object(&mut e, 0x500, process_table);
        stub(&mut e, 0x00aa_f2a4, 0);
        stub(&mut e, 0x00aa_f1dc, 0);
        stub(&mut e, 0x008d_8520, their_process);
        stub(&mut e, 0x0093_44a0, 0x5555);
        stub(&mut e, 0x0041_ca90, 6);
        let actor_table = vtable(&mut e, &[(0x214, 0x00aa_f214)]);
        let actor = object(&mut e, 0x200, actor_table);
        stub(&mut e, 0x00aa_f214, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_1740, &args![0u32, actor]);
        assert_eq!(calls_to(&e, 0x0061_a2d0), vec![vec![1, 2]]);
        assert_eq!(calls_to(&e, 0x0057_bd60), vec![vec![actor, player]]);
        assert_eq!(
            calls_to(&e, 0x00aa_f2a4),
            vec![vec![their_process, actor, 0x9001, 0, 1, 0, 0]]
        );
        // Type 6: the second process call is skipped.
        assert!(calls_to(&e, 0x00aa_f1dc).is_empty());
        stub(&mut e, 0x0041_ca90, 5);
        e.call(0x0090_1740, &args![0u32, actor]);
        assert_eq!(
            calls_to(&e, 0x00aa_f1dc),
            vec![vec![their_process, actor, 1]]
        );
        // A nonzero slot 0x214 skips it too, and a null item skips the first call.
        stub(&mut e, 0x00aa_f214, 1);
        stub(&mut e, 0x0061_a2d0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_1740, &args![0u32, actor]);
        assert!(calls_to(&e, 0x00aa_f1dc).is_empty());
        assert!(calls_to(&e, 0x00aa_f2a4).is_empty());
    }

    /// Everything `ComputeCanInitiateDialogueWithNPC` asks, answering "yes" throughout.
    /// Returns (process, actor, other, other's process).
    fn initiate_setup(e: &mut Engine) -> (u32, u32, u32) {
        let player = e.mem.alloc(0x10);
        global(e, 0x011d_ea3c, player);
        put_double(e, 0x0101_2060, 0.0);
        put_double(e, 0x0101_db88, 1.0);
        e.map(0x011f_1221, 4);
        e.mem.set_u8(0x011f_1221, 0);
        stub(e, 0x0096_9860, 0);
        stub(e, 0x008d_efe0, 0);
        stub(e, 0x0088_4480, 1);
        stub(e, 0x0057_9670, 0);
        stub(e, 0x008a_78f0, 0);
        stub(e, 0x0089_3190, 1);
        stub(e, 0x0088_1510, 0);
        let other_process_table = vtable(e, &[(0x1f8, 0x00aa_f1f8)]);
        let other_process = object(e, 0x500, other_process_table);
        stub(e, 0x00aa_f1f8, 1);
        stub(e, 0x008d_8520, other_process);
        let other_table = vtable(e, &[(0x1d0, 0x00aa_f1d0)]);
        let other = object(e, 0x200, other_table);
        stub(e, 0x00aa_f1d0, 0x6001);
        let actor = e.mem.alloc(0x200);
        let table = vtable(e, &[(0x27c, 0x00aa_f27c), (0x504, 0x00aa_f504)]);
        let proc = object(e, 0x500, table);
        stub(e, 0x00aa_f27c, 0x6002);
        let state = e.mem.alloc(0x40);
        stub(e, 0x00aa_f504, state);
        stub_float(e, 0x0043_5e00, 0.5);
        (proc, actor, other)
    }

    #[test]
    fn test_high_process_compute_can_initiate_dialogue_with_npc() {
        let mut e = Engine::new();
        let (proc, actor, other) = initiate_setup(&mut e);
        let run = |e: &mut Engine| e.call(0x0090_1800, &args![proc, actor, other]).bool();
        assert!(run(&mut e));
        // The player is busy.
        stub(&mut e, 0x0096_9860, 1);
        assert!(!run(&mut e));
        stub(&mut e, 0x0096_9860, 0);
        // A positive wait in the process.
        e.mem.set_f32(proc + 0x2a8, 1.0);
        assert!(!run(&mut e));
        e.mem.set_f32(proc + 0x2a8, 0.0);
        // The actor is already in a conversation.
        stub(&mut e, 0x008a_78f0, 1);
        assert!(!run(&mut e));
        stub(&mut e, 0x008a_78f0, 0);
        // The other is the actor itself.
        assert!(!e.call(0x0090_1800, &args![proc, actor, actor]).bool());
        // Its process refuses.
        stub(&mut e, 0x00aa_f1f8, 0);
        assert!(!run(&mut e));
        stub(&mut e, 0x00aa_f1f8, 1);
        // With the global byte set the detection state's float decides.
        e.mem.set_u8(0x011f_1221, 1);
        assert!(run(&mut e));
        stub_float(&mut e, 0x0043_5e00, 2.0);
        assert!(!run(&mut e));
        stub(&mut e, 0x00aa_f504, 0);
        assert!(!run(&mut e));
    }

    #[test]
    fn test_high_process_compute_can_respond_to_dialogue_from_npc() {
        let mut e = Engine::new();
        let table = vtable(
            &mut e,
            &[
                (0x52c, 0x00aa_f52c),
                (0x30c, 0x00aa_f30c),
                (0x22c, 0x00aa_f22c),
            ],
        );
        let proc = object(&mut e, 0x500, table);
        stub(&mut e, 0x00aa_f52c, 0);
        stub(&mut e, 0x00aa_f30c, 0);
        stub(&mut e, 0x00aa_f22c, 0);
        stub(&mut e, 0x008a_78f0, 0);
        stub(&mut e, 0x0067_8610, 0);
        let actor = 0x6001u32;
        let run = |e: &mut Engine| e.call(0x0090_1990, &args![proc, actor, 0u32]).bool();
        assert!(run(&mut e));
        stub(&mut e, 0x00aa_f22c, 0x6100);
        assert!(run(&mut e));
        stub(&mut e, 0x0067_8610, 1);
        assert!(!run(&mut e));
        stub(&mut e, 0x00aa_f22c, 0);
        assert!(run(&mut e));
        stub(&mut e, 0x00aa_f52c, 1);
        assert!(!run(&mut e));
        stub(&mut e, 0x00aa_f52c, 0);
        stub(&mut e, 0x00aa_f30c, 1);
        assert!(!run(&mut e));
        stub(&mut e, 0x00aa_f30c, 0);
        stub(&mut e, 0x008a_78f0, 1);
        assert!(!run(&mut e));
    }

    #[test]
    fn test_fn_00901c50() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x40);
        assert!(e.call(0x0090_1c50, &args![block]).bool());
        e.mem.set_u16(block + 0x24, 0x0004);
        assert!(!e.call(0x0090_1c50, &args![block]).bool());
        e.mem.set_u16(block + 0x24, 0xfffb);
        assert!(e.call(0x0090_1c50, &args![block]).bool());
    }

    #[test]
    fn test_high_process_process_sand_box_dialogue() {
        let mut e = Engine::new();
        put_setting(&mut e, 0x011c_d038, 1.0);
        put_setting(&mut e, 0x011c_d328, 2.0);
        let table = vtable(
            &mut e,
            &[
                (0x27c, 0x00aa_f27c),
                (0x274, 0x00aa_f274),
                (0x1f4, 0x00aa_f1f4),
                (0x1f0, 0x00aa_f1f0),
            ],
        );
        let proc = object(&mut e, 0x500, table);
        stub(&mut e, 0x00aa_f27c, 0x6002);
        stub(&mut e, 0x00aa_f274, 0);
        stub(&mut e, 0x00aa_f1f4, 1);
        stub(&mut e, 0x00aa_f1f0, 0);
        let other_process_table = vtable(&mut e, &[(0x1f0, 0x00aa_f2f0)]);
        let other_process = object(&mut e, 0x500, other_process_table);
        stub(&mut e, 0x00aa_f2f0, 0);
        stub(&mut e, 0x008d_8520, other_process);
        let other_table = vtable(&mut e, &[(0x100, 0x00aa_f100)]);
        let other = object(&mut e, 0x200, other_table);
        stub(&mut e, 0x00aa_f100, 1);
        let actor_table = vtable(&mut e, &[(0x280, 0x00aa_f280)]);
        let actor = object(&mut e, 0x200, actor_table);
        stub(&mut e, 0x00aa_f280, 1);
        stub(&mut e, 0x0093_44a0, 0x7100);
        stub(&mut e, 0x0096_11e0, 0x25);
        let form_flags = e.mem.alloc(0x40);
        stub(&mut e, 0x0093_44a0, form_flags);
        stub(&mut e, 0x0055_b980, 0x5100);
        stub(&mut e, 0x0067_f030, 0);
        stub(&mut e, 0x0067_f110, 0);
        e.register_double(0x0064_65f0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]) * 10.0 + f32::from_bits(a[1])),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0090_1a20, &args![proc, actor, other]).bool());
        // The form has the flag clear: the last argument is 1; the package's location is
        // used; the random value is 2 * 10 + 1.
        assert_eq!(
            calls_to(&e, 0x00aa_f280),
            vec![vec![actor, other, 0x5100, 0x5100, 1, 0, 0, 0, 1, 1]]
        );
        assert_eq!(e.mem.f32(proc + 0x2c8), 21.0);
        assert_eq!(calls_to(&e, 0x00aa_f1f0), vec![vec![proc, other]]);
        assert_eq!(calls_to(&e, 0x00aa_f2f0), vec![vec![other_process, actor]]);
        assert_eq!(calls_to(&e, 0x0067_f110).len(), 1);
        // The flag bit set: the last argument is 0. A form of another type too.
        e.mem.set_u16(form_flags + 0x24, 4);
        e.call_log = Some(vec![]);
        e.call(0x0090_1a20, &args![proc, actor, other]);
        assert_eq!(calls_to(&e, 0x00aa_f280)[0][9], 0);
        // A slot 0x274 object of type 12 supplies its own location.
        let source_table = vtable(&mut e, &[(0x8, 0x00aa_f008)]);
        let source = object(&mut e, 0x40, source_table);
        stub(&mut e, 0x00aa_f008, 0xc);
        stub(&mut e, 0x00aa_f274, source);
        stub(&mut e, 0x0044_edb0, 0x5300);
        stub(&mut e, 0x0067_f3c0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_1a20, &args![proc, actor, other]);
        let call = &calls_to(&e, 0x00aa_f280)[0];
        assert_ne!(call[2], 0x5100);
        assert_eq!(call[2], call[3]);
        assert_eq!(calls_to(&e, 0x0067_f3c0).len(), 1);
        assert_eq!(calls_to(&e, 0x0067_f3c0)[0][1], 0x5300);
        // Not an actor, or refused by the process: false, nothing run.
        stub(&mut e, 0x00aa_f100, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0090_1a20, &args![proc, actor, other]).bool());
        assert!(!e.call(0x0090_1a20, &args![proc, actor, 0u32]).bool());
        stub(&mut e, 0x00aa_f100, 1);
        stub(&mut e, 0x00aa_f1f4, 0);
        assert!(!e.call(0x0090_1a20, &args![proc, actor, other]).bool());
        assert!(calls_to(&e, 0x00aa_f280).is_empty());
    }

    /// The doubles the idle-marker function needs; the actor and the process record their
    /// slot calls. Returns (process, actor, target).
    fn idle_setup(e: &mut Engine, mode: u32) -> (u32, u32, u32) {
        let process_table = vtable(
            e,
            &[
                (0x12c, 0x00aa_f12c),
                (0x8c, 0x00aa_f08c),
                (0x294, 0x00aa_f294),
                (0x288, 0x00aa_f288),
                (0x27c, 0x00aa_f27c),
                (0x34c, 0x00aa_f34c),
                (0x350, 0x00aa_f350),
                (0x660, 0x00aa_f660),
                (0x44, 0x00aa_f044),
            ],
        );
        let proc = object(e, 0x500, process_table);
        for slot in [
            0x00aa_f12c,
            0x00aa_f08c,
            0x00aa_f294,
            0x00aa_f288,
            0x00aa_f660,
            0x00aa_f044,
            0x00aa_f350,
        ] {
            stub(e, slot, 0);
        }
        stub(e, 0x00aa_f27c, 0x5100);
        stub(e, 0x00aa_f34c, 0x77);
        let position = e.mem.alloc(12);
        let target_table = vtable(e, &[(0x1f4, 0x00aa_f1f4)]);
        let target = object(e, 0x200, target_table);
        stub(e, 0x00aa_f1f4, position);
        let owner = e.mem.alloc(0x10);
        let actor_table = vtable(
            e,
            &[
                (0x1f4, 0x00aa_f1f4),
                (0x214, 0x00aa_f214),
                (0x1e4, 0x00aa_f1e4),
                (0x234, 0x00aa_f234),
            ],
        );
        let actor = object(e, 0x200, actor_table);
        stub(e, 0x00aa_f214, 0);
        stub(e, 0x00aa_f1e4, owner);
        stub(e, 0x00aa_f234, 0);
        let marker_form = e.mem.alloc(0x40);
        stub(e, 0x007a_f430, marker_form);
        stub(e, 0x0047_9e80, mode);
        stub(e, 0x0040_1170, 0x30);
        stub(e, 0x0091_5ef0, 0);
        stub(e, 0x008b_3bd0, 0);
        put_float(e, 0x0101_e6ec, 64.0);
        put_float(e, 0x0101_6410, 100.0);
        put_float(e, 0x0101_7b78, 10.0);
        put_float(e, 0x0103_0ff0, 0.75);
        put_float(e, 0x0101_2054, -1.0);
        put_double(e, 0x0101_ffa0, 0.1);
        stub(e, 0x0043_9ef0, 0);
        e.mem.set_u32(proc + 0x40, target);
        (proc, actor, target)
    }

    #[test]
    fn test_high_process_process_use_idle_marker_stops_moving_and_paths() {
        let mut e = Engine::new();
        let (proc, actor, _target) = idle_setup(&mut e, 0);
        // A marker is passed to slot 0x12c; an actor that may not move is stopped.
        stub(&mut e, 0x0091_5ef0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0090_1c70, &args![proc, actor, 0x6600u32]);
        assert_eq!(calls_to(&e, 0x00aa_f12c), vec![vec![proc, 0x6600]]);
        assert_eq!(calls_to(&e, 0x00aa_f294), vec![vec![proc, actor]]);
        assert_eq!(
            calls_to(&e, 0x00aa_f288),
            vec![vec![proc, actor, 0xffff_ffff]]
        );
        // A pathing actor farther than 64 from the marker: slot 0x34c, then 0x350.
        stub(&mut e, 0x0091_5ef0, 0);
        stub(&mut e, 0x008b_3bd0, 1);
        stub_float(&mut e, 0x0057_23b0, 100.0);
        stub_float(&mut e, 0x0067_8670, 5.0);
        e.call_log = Some(vec![]);
        e.call(0x0090_1c70, &args![proc, actor, 0x6600u32]);
        assert_eq!(
            calls_to(&e, 0x00aa_f34c),
            vec![vec![
                proc,
                actor,
                100.0f32.to_bits(),
                5.0f32.to_bits(),
                10.0f32.to_bits(),
                0,
                0
            ]]
        );
        assert_eq!(calls_to(&e, 0x00aa_f350), vec![vec![proc, actor, 0x77, 1]]);
        // Closer than 64: the plain step value 0x101 and a clear flag.
        stub_float(&mut e, 0x0057_23b0, 10.0);
        e.call_log = Some(vec![]);
        e.call(0x0090_1c70, &args![proc, actor, 0x6600u32]);
        assert!(calls_to(&e, 0x00aa_f34c).is_empty());
        assert_eq!(calls_to(&e, 0x00aa_f350), vec![vec![proc, actor, 0x101, 0]]);
    }

    #[test]
    fn test_high_process_process_use_idle_marker_target_far_or_turning() {
        let mut e = Engine::new();
        let (proc, actor, target) = idle_setup(&mut e, 0);
        // No marker, a target whose form has type 0x30: the squared distance 1000 is
        // above 10 * 10, so the actor is told to move.
        stub_float(&mut e, 0x0059_5c80, 1000.0);
        stub(&mut e, 0x008b_37c0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_1c70, &args![proc, actor, 0u32]);
        assert_eq!(
            calls_to(&e, 0x008b_37c0),
            vec![vec![actor, target, 10.0f32.to_bits(), 0]]
        );
        // Near, but the height gap is above 100.
        stub_float(&mut e, 0x0059_5c80, 1.0);
        e.register_double(0x0040_8860, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        // The z value is read from the out buffer: write it through 00439ef0.
        e.register_double(0x0043_9ef0, |e, a| {
            e.mem.set_f32(a[1] + 8, 250.0);
            ret(a[1])
        });
        e.call_log = Some(vec![]);
        e.call(0x0090_1c70, &args![proc, actor, 0u32]);
        assert_eq!(calls_to(&e, 0x008b_37c0).len(), 1);
        // Near and level, but the heights differ: the actor is turned to the target's.
        e.register_double(0x0043_9ef0, |e, a| {
            e.mem.set_f32(a[1] + 8, 1.0);
            ret(a[1])
        });
        let target_node = e.mem.alloc(0x20);
        e.mem.set_f32(target_node + 8, 3.0);
        let actor_node = e.mem.alloc(0x20);
        e.mem.set_f32(actor_node + 8, 2.0);
        e.register_double(0x0043_0830, move |_, a| {
            ret(if a[0] == target {
                target_node
            } else {
                actor_node
            })
        });
        stub_float(&mut e, 0x004b_15e0, 0.5);
        e.register_double(0x0040_8840, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        stub(&mut e, 0x008b_b5c0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_1c70, &args![proc, actor, 0u32]);
        assert_eq!(calls_to(&e, 0x008b_37c0).len(), 0);
        assert_eq!(
            calls_to(&e, 0x004b_15e0)[0][..2],
            [2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(
            calls_to(&e, 0x008b_b5c0),
            vec![vec![actor, 3.0f32.to_bits(), 1]]
        );
        assert!(calls_to(&e, 0x00aa_f044).is_empty());
    }

    #[test]
    fn test_high_process_process_use_idle_marker_picks_an_idle() {
        let mut e = Engine::new();
        let (proc, actor, target) = idle_setup(&mut e, 0);
        stub_float(&mut e, 0x0059_5c80, 1.0);
        e.register_double(0x0043_9ef0, |e, a| {
            e.mem.set_f32(a[1] + 8, 1.0);
            ret(a[1])
        });
        e.register_double(0x0040_8860, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        let node = e.mem.alloc(0x20);
        e.register_double(0x0043_0830, move |_, _| ret(node));
        stub_float(&mut e, 0x004b_15e0, 0.0);
        e.register_double(0x0040_8840, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        stub(&mut e, 0x0090_58e0, 0);
        stub(&mut e, 0x0090_5910, 0);
        stub(&mut e, 0x0047_a030, 3);
        stub(&mut e, 0x00ec_adb8, 7);
        let table = e.mem.alloc(12);
        for (index, idle) in [0x1010u32, 0x1020, 0x1030].into_iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, idle);
        }
        e.register_double(0x0087_7a30, move |_, a| ret(table + 4 * a[1]));
        // Mode 0: random, 7 % 3 = 1.
        e.call_log = Some(vec![]);
        e.call(0x0090_1c70, &args![proc, actor, 0u32]);
        assert_eq!(e.mem.u32(proc + 0x194), 0x1020);
        assert_eq!(e.mem.u32(proc + 0x190), 1);
        assert_eq!(e.mem.f32(proc + 0x198), 0.75);
        assert_eq!(calls_to(&e, 0x0047_a030)[0][1..3], [actor, target]);
        assert_eq!(
            calls_to(&e, 0x00aa_f044),
            vec![vec![proc, actor, 0x1020, 2, 1, 0, 0]]
        );
        assert_eq!(calls_to(&e, 0x00aa_f660).len(), 1);
        assert_eq!(calls_to(&e, 0x0090_5910).len(), 1);
        // A current idle that is still playing (004985f0 false) keeps it.
        stub(&mut e, 0x0049_85f0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_1c70, &args![proc, actor, 0u32]);
        assert!(calls_to(&e, 0x00aa_f044).is_empty());
        assert_eq!(e.mem.u32(proc + 0x194), 0x1020);
        // Mode 1: in turn, wrapping to 0 after the last one; the delay is -1.
        stub(&mut e, 0x0047_9e80, 1);
        stub(&mut e, 0x0049_85f0, 1);
        e.mem.set_u32(proc + 0x190, 1);
        e.call(0x0090_1c70, &args![proc, actor, 0u32]);
        assert_eq!(e.mem.u32(proc + 0x190), 2);
        assert_eq!(e.mem.u32(proc + 0x194), 0x1030);
        assert_eq!(e.mem.f32(proc + 0x198), -1.0);
        e.call(0x0090_1c70, &args![proc, actor, 0u32]);
        assert_eq!(e.mem.u32(proc + 0x190), 0);
        assert_eq!(e.mem.u32(proc + 0x194), 0x1010);
        // The actor is busy (slot 0x234): nothing changes.
        stub(&mut e, 0x00aa_f234, 1);
        e.mem.set_u32(proc + 0x190, 0);
        e.call(0x0090_1c70, &args![proc, actor, 0u32]);
        assert_eq!(e.mem.u32(proc + 0x190), 0);
        assert_eq!(e.mem.u32(proc + 0x194), 0x1010);
    }

    #[test]
    fn test_high_process_process_use_idle_marker_gives_up_without_a_target() {
        let mut e = Engine::new();
        let (proc, actor, _target) = idle_setup(&mut e, 0);
        e.mem.set_u32(proc + 0x40, 0);
        e.call_log = Some(vec![]);
        // No marker and no target: slot 0x8c is asked, and the function ends.
        e.call(0x0090_1c70, &args![proc, actor, 0u32]);
        assert_eq!(calls_to(&e, 0x00aa_f08c), vec![vec![proc, actor]]);
        assert!(calls_to(&e, 0x0091_5ef0).is_empty());
        // Slot 0x8c finds a target whose form has another type: also over.
        let target = e.mem.alloc(0x40);
        e.register_double(0x00aa_f08c, move |e, a| {
            e.mem.set_u32(a[0] + 0x40, target);
            ret(0)
        });
        stub(&mut e, 0x0040_1170, 0x31);
        e.call(0x0090_1c70, &args![proc, actor, 0u32]);
        assert!(calls_to(&e, 0x0091_5ef0).is_empty());
    }

    #[test]
    fn test_high_process_create_muzzle_flash() {
        let mut e = Engine::new();
        e.register_double(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        stub(&mut e, 0x009b_acb0, 0x9100);
        let proc = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        e.call(0x0090_2190, &args![proc, 0x11u32, 0x22u32]);
        assert_eq!(e.mem.u32(proc + 0x3d4), 0x9100);
        let call = &calls_to(&e, 0x009b_acb0)[0];
        assert_eq!((call[1], call[2]), (0x11, 0x22));
        assert_eq!(calls_to(&e, 0x0040_1000), vec![vec![0x20]]);
        // Already present: nothing happens. A failed allocation stores null.
        e.call_log = Some(vec![]);
        e.call(0x0090_2190, &args![proc, 1u32, 2u32]);
        assert!(calls_to(&e, 0x0040_1000).is_empty());
        e.mem.set_u32(proc + 0x3d4, 0);
        stub(&mut e, 0x0040_1000, 0);
        e.call(0x0090_2190, &args![proc, 1u32, 2u32]);
        assert_eq!(e.mem.u32(proc + 0x3d4), 0);
        assert!(calls_to(&e, 0x009b_acb0).is_empty());
    }

    #[test]
    fn test_high_process_clear_muzzle_flash() {
        let mut e = Engine::new();
        stub(&mut e, 0x008d_9f70, 0);
        let proc = e.mem.alloc(0x500);
        e.call_log = Some(vec![]);
        e.call(0x0090_2230, &args![proc]);
        assert!(calls_to(&e, 0x008d_9f70).is_empty());
        e.mem.set_u32(proc + 0x3d4, 0x9100);
        e.call(0x0090_2230, &args![proc]);
        assert_eq!(calls_to(&e, 0x008d_9f70), vec![vec![0x9100, 1]]);
        assert_eq!(e.mem.u32(proc + 0x3d4), 0);
    }

    #[test]
    fn test_guard_state_helpers() {
        let mut e = Engine::new();
        let state = e.mem.alloc(0x20);
        e.mem.set_u32(state + 0xc, 5);
        e.mem.set_f32(state + 0x10, 3.0);
        e.call(0x0090_2c60, &args![state]);
        assert_eq!(e.mem.u32(state + 0xc), 6);
        e.call(0x0090_26a0, &args![state]);
        assert_eq!(e.mem.u32(state + 0xc), 0);
        e.call(0x0090_2680, &args![state]);
        assert_eq!(e.mem.f32(state + 0x10), 0.0);
    }

    #[test]
    fn test_fn_00902640() {
        let mut e = Engine::new();
        let table = vtable(&mut e, &[(0x7f8, 0x00aa_f7f8)]);
        let proc = object(&mut e, 0x500, table);
        stub(&mut e, 0x00aa_f7f8, 0);
        let state = e.mem.alloc(0x20);
        e.mem.set_u32(state + 0xc, 5);
        e.mem.set_f32(state + 0x10, 3.0);
        e.call_log = Some(vec![]);
        e.call(0x0090_2640, &args![proc, 0x6001u32, state]);
        assert_eq!(calls_to(&e, 0x00aa_f7f8), vec![vec![proc, 0x6001, state]]);
        assert_eq!(e.mem.u32(state + 0xc), 0);
        assert_eq!(e.mem.f32(state + 0x10), 0.0);
    }

    #[test]
    fn test_high_process_process_guard_warn() {
        let mut e = Engine::new();
        let table = vtable(&mut e, &[(0x128, 0x00aa_f128), (0x2a4, 0x00aa_f2a4)]);
        let proc = object(&mut e, 0x500, table);
        stub(&mut e, 0x00aa_f128, 0x6100);
        stub(&mut e, 0x00aa_f2a4, 0);
        stub(&mut e, 0x0044_ddc0, 0);
        stub(&mut e, 0x0056_7790, 0);
        stub(&mut e, 0x0061_a2d0, 0x9001);
        let state = e.mem.alloc(0x20);
        e.mem.set_u32(state + 0xc, 1);
        e.mem.set_f32(state + 0x10, 4.0);
        e.call_log = Some(vec![]);
        e.call(0x0090_2bd0, &args![proc, 0x6001u32, state]);
        assert_eq!(calls_to(&e, 0x0061_a2d0), vec![vec![2, 0x14]]);
        assert_eq!(e.mem.u32(state + 0xc), 2);
        assert_eq!(e.mem.f32(state + 0x10), 0.0);
        assert_eq!(
            calls_to(&e, 0x00aa_f2a4),
            vec![vec![proc, 0x6001, 0x9001, 0, 0, 1, 0]]
        );
        assert_eq!(calls_to(&e, 0x0056_7790), vec![vec![0x6100]]);
    }

    /// A guard process and actor with the doubles `ProcessGuard` needs; the state's mode
    /// is the word at +4 (`00726070`).
    fn guard_setup(e: &mut Engine, mode: u32) -> (u32, u32, u32) {
        let process_table = vtable(
            e,
            &[
                (0x20c, 0x00aa_f20c),
                (0x270, 0x00aa_f270),
                (0x284, 0x00aa_f284),
                (0x27c, 0x00aa_f27c),
                (0x128, 0x00aa_f128),
                (0x12c, 0x00aa_f12c),
                (0x35c, 0x00aa_f35c),
                (0x34c, 0x00aa_f34c),
                (0x350, 0x00aa_f350),
                (0x7f8, 0x00aa_f7f8),
                (0x644, 0x00aa_f644),
                (0x654, 0x00aa_f654),
            ],
        );
        let proc = object(e, 0x500, process_table);
        for slot in [
            0x00aa_f20c,
            0x00aa_f270,
            0x00aa_f284,
            0x00aa_f12c,
            0x00aa_f35c,
            0x00aa_f350,
            0x00aa_f7f8,
            0x00aa_f644,
            0x00aa_f654,
        ] {
            stub(e, slot, 0);
        }
        stub(e, 0x00aa_f34c, 0x77);
        let package_table = vtable(e, &[(0x13c, 0x00aa_f13c)]);
        let package = object(e, 0x80, package_table);
        stub(e, 0x00aa_f27c, package);
        stub(e, 0x00aa_f13c, 1);
        let actor_table = vtable(
            e,
            &[
                (0x214, 0x00aa_f214),
                (0x4b4, 0x00aa_f4b4),
                (0x418, 0x00aa_f418),
                (0x1f4, 0x00aa_f1f4),
            ],
        );
        let actor = object(e, 0x200, actor_table);
        stub(e, 0x00aa_f214, 0);
        stub(e, 0x00aa_f4b4, 0);
        stub(e, 0x00aa_f418, 0);
        let state = e.mem.alloc(0x40);
        e.mem.set_u32(state + 4, mode);
        stub(e, 0x008d_0430, state);
        e.register_double(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        stub(e, 0x0067_59e0, 0);
        stub(e, 0x005d_43c0, 0x8800);
        stub(e, 0x0041_c8d0, 0);
        stub(e, 0x00aa_f128, actor);
        stub(e, 0x0041_ca90, 1);
        stub(e, 0x0055_b980, 0);
        stub(e, 0x0067_1d10, 0);
        stub(e, 0x0067_a7e0, 0);
        stub(e, 0x008b_3d30, 0);
        stub(e, 0x0044_ddc0, 0);
        stub(e, 0x0067_8670, 0);
        stub(e, 0x008b_3bb0, 0);
        stub(e, 0x008b_3b90, 0);
        stub(e, 0x006e_cd40, 0);
        put_float(e, 0x0101_8f5c, 5.0);
        put_float(e, 0x0101_2054, -1.0);
        stub_float(e, 0x0057_23b0, 100.0);
        (proc, actor, state)
    }

    #[test]
    fn test_high_process_process_guard() {
        let mut e = Engine::new();
        let (proc, actor, state) = guard_setup(&mut e, 3);
        // No state at all: slot 0x284 with -1.
        stub(&mut e, 0x008d_0430, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_2290, &args![proc, actor]);
        assert_eq!(calls_to(&e, 0x00aa_f284), vec![vec![proc, 0xffff_ffff]]);
        // The state comes from slot 0x270 when slot 0x20c has an object of type 14.
        stub(&mut e, 0x00aa_f20c, 0x5001);
        stub(&mut e, 0x0041_ca90, 0xe);
        stub(&mut e, 0x00aa_f270, state);
        e.call_log = Some(vec![]);
        e.call(0x0090_2290, &args![proc, actor]);
        assert!(calls_to(&e, 0x00aa_f284).is_empty());
        assert_eq!(e.mem.u8(proc + 0x349), 0);
        stub(&mut e, 0x00aa_f20c, 0);
        stub(&mut e, 0x008d_0430, state);
        // The guard reference differs from the process's: it is installed and flagged.
        let guard_table = vtable(&mut e, &[(0x48, 0x00aa_f048)]);
        let guard = object(&mut e, 0x100, guard_table);
        stub(&mut e, 0x00aa_f048, 0);
        stub(&mut e, 0x0067_59e0, guard);
        stub(&mut e, 0x0042_1310, 0);
        stub(&mut e, 0x00aa_f128, 0x6666);
        e.call_log = Some(vec![]);
        e.call(0x0090_2290, &args![proc, actor]);
        assert_eq!(calls_to(&e, 0x00aa_f12c), vec![vec![proc, guard]]);
        assert_eq!(calls_to(&e, 0x0042_1310), vec![vec![0x8800, actor]]);
        assert_eq!(calls_to(&e, 0x00aa_f048), vec![vec![guard, 0x8000_0000]]);
        stub(&mut e, 0x0067_59e0, 0);
        stub(&mut e, 0x00aa_f128, actor);
        // An actor of type 4 with no combat state is sent to melee idle.
        stub(&mut e, 0x00aa_f214, 4);
        e.call_log = Some(vec![]);
        e.call(0x0090_2290, &args![proc, actor]);
        assert_eq!(calls_to(&e, 0x006e_cd40), vec![vec![state, 0]]);
        assert_eq!(calls_to(&e, 0x00aa_f418), vec![vec![actor]]);
        assert!(calls_to(&e, 0x00aa_f34c).is_empty());
        stub(&mut e, 0x00aa_f214, 0);
        // Mode 3 runs no phase. The package target gives no radius: the global 5.0; the
        // distance 100 puts the actor outside, so slot 0x34c gets (actor, 0, 5, 10, 0, 0).
        e.call_log = Some(vec![]);
        e.call(0x0090_2290, &args![proc, actor]);
        assert_eq!(
            calls_to(&e, 0x00aa_f34c),
            vec![vec![
                proc,
                actor,
                0,
                5.0f32.to_bits(),
                10.0f32.to_bits(),
                0,
                0
            ]]
        );
        assert_eq!(calls_to(&e, 0x00aa_f350), vec![vec![proc, actor, 0x77, 1]]);
        assert!(calls_to(&e, 0x00aa_f7f8).is_empty());
        // An actor that is pathing (slot 0x4b4) skips all of it.
        stub(&mut e, 0x00aa_f4b4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0090_2290, &args![proc, actor]);
        assert!(calls_to(&e, 0x00aa_f34c).is_empty());
        stub(&mut e, 0x00aa_f4b4, 0);
        // Mode 0 runs fn_00902640; the nonzero mode of the check lets 008b3b90 call 0x7f8.
        e.mem.set_u32(state + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_2290, &args![proc, actor]);
        assert_eq!(calls_to(&e, 0x00aa_f7f8), vec![vec![proc, actor, state]]);
        e.mem.set_u32(state + 4, 1);
        stub(&mut e, 0x008b_3b90, 1);
        e.call_log = Some(vec![]);
        e.call(0x0090_2290, &args![proc, actor]);
        assert_eq!(calls_to(&e, 0x00aa_f7f8), vec![vec![proc, actor, state]]);
        // Mode 1 (fn_009026c0 with the actor outside): the head-track slots are cleared.
        assert_eq!(calls_to(&e, 0x00aa_f644), vec![vec![proc, 0]]);
        assert_eq!(calls_to(&e, 0x00aa_f654), vec![vec![proc, 0]]);
    }

    #[test]
    fn test_high_process_process_guard_has_a_location() {
        let mut e = Engine::new();
        let (proc, actor, state) = guard_setup(&mut e, 3);
        // With a package location the answer of the package's slot 0x13c and the unsigned
        // result of 00676280 give "inside" and the radius.
        stub(&mut e, 0x0055_b980, 0x5100);
        stub(&mut e, 0x0067_6280, 7);
        e.call_log = Some(vec![]);
        e.call(0x0090_2290, &args![proc, actor]);
        assert_eq!(
            calls_to(&e, 0x00aa_f34c),
            vec![vec![
                proc,
                actor,
                0,
                7.0f32.to_bits(),
                14.0f32.to_bits(),
                0,
                0
            ]]
        );
        // Mode 2 is the wait-to-attack phase: with no target it returns to state 0.
        e.mem.set_u32(state + 4, 2);
        stub(&mut e, 0x0044_ddc0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_2290, &args![proc, actor]);
        assert!(calls_to(&e, 0x006e_cd40).contains(&vec![state, 0]));
    }

    /// Entries for the detected-actor list: one candidate at +0x25c with the doubles
    /// `fn_009026c0` needs. Returns (process, actor, state, candidate).
    fn intruder_setup(e: &mut Engine) -> (u32, u32, u32, u32) {
        let (proc, actor, state) = guard_setup(e, 1);
        e.register_double(0x0068_15c0, |_, a| ret(a[0]));
        stub(e, 0x0082_56d0, 0);
        let candidate_table = vtable(
            e,
            &[
                (0x22c, 0x00aa_f22c),
                (0x2e8, 0x00aa_f2e8),
                (0x230, 0x00aa_f230),
                (0x234, 0x00aa_f234),
                (0x1f4, 0x00aa_f1f4),
            ],
        );
        let candidate = object(e, 0x200, candidate_table);
        for slot in [0x00aa_f22c, 0x00aa_f2e8, 0x00aa_f230, 0x00aa_f234] {
            stub(e, slot, 0);
        }
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        stub(e, 0x00aa_f1f4, position);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry, candidate);
        e.mem.set_u32(entry + 8, 5);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, entry);
        e.mem.set_u32(proc + 0x25c, node);
        stub(e, 0x0067_a7e0, 1);
        stub(e, 0x0044_ddc0, 10);
        let their_process = e.mem.alloc(0x40);
        stub(e, 0x008d_8520, their_process);
        stub(e, 0x0045_cd60, 0);
        stub(e, 0x0043_7bd0, 0);
        stub(e, 0x0043_9ef0, 0);
        stub_float(e, 0x004a_7290, 4.0);
        stub(e, 0x0056_7790, 0);
        stub(e, 0x008b_06d0, 1);
        stub(e, 0x008b_b520, 0);
        stub(e, 0x009f_2970, 0);
        stub(e, 0x0061_a2d0, 0x9001);
        let table = vtable(e, &[(0x294, 0x00aa_f294), (0x2a4, 0x00aa_f2a4)]);
        let _ = table;
        (proc, actor, state, candidate)
    }

    #[test]
    fn test_fn_009026c0_finds_an_intruder() {
        let mut e = Engine::new();
        let (proc, actor, state, candidate) = intruder_setup(&mut e);
        // The process's slots 0x294 and 0x2a4 are not in guard_setup's table: add them.
        let table = e.mem.u32(proc);
        e.mem.set_u32(table + 0x294, 0x00aa_f294);
        e.mem.set_u32(table + 0x2a4, 0x00aa_f2a4);
        stub(&mut e, 0x00aa_f294, 0);
        stub(&mut e, 0x00aa_f2a4, 0);
        stub(&mut e, 0x0067_f390, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 1u8]);
        assert_eq!(calls_to(&e, 0x009f_2970), vec![vec![state, candidate]]);
        assert_eq!(calls_to(&e, 0x006e_cd40), vec![vec![state, 2]]);
        assert_eq!(calls_to(&e, 0x00aa_f294), vec![vec![proc, actor]]);
        assert_eq!(
            calls_to(&e, 0x008b_b520),
            vec![vec![
                actor,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(calls_to(&e, 0x008b_06d0).len(), 1);
        assert_eq!(calls_to(&e, 0x008b_06d0)[0][1..3], [candidate, 1]);
        // The warning ran (the actor's process got slot 0x2a4 and the counter moved).
        assert_eq!(calls_to(&e, 0x00aa_f2a4).len(), 1);
        assert_eq!(e.mem.u32(state + 0xc), 1);
        // Not inside: the actor is not stopped or turned.
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 0u8]);
        assert!(calls_to(&e, 0x00aa_f294).is_empty());
        assert!(calls_to(&e, 0x008b_b520).is_empty());
        assert_eq!(calls_to(&e, 0x006e_cd40), vec![vec![state, 2]]);
        // Too far (4 against a squared radius of 100: closer; make it 400), refused by the
        // checks, or not to be attacked: none is found.
        stub_float(&mut e, 0x004a_7290, 400.0);
        stub(&mut e, 0x008b_3d30, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 0u8]);
        assert!(calls_to(&e, 0x009f_2970).is_empty());
        assert_eq!(calls_to(&e, 0x00aa_f644), vec![vec![proc, 0]]);
        assert_eq!(calls_to(&e, 0x008b_3d30), vec![vec![actor]]);
        stub_float(&mut e, 0x004a_7290, 4.0);
        stub(&mut e, 0x008b_06d0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 0u8]);
        assert!(calls_to(&e, 0x009f_2970).is_empty());
        stub(&mut e, 0x008b_06d0, 1);
        stub(&mut e, 0x0045_cd60, 1);
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 0u8]);
        assert!(calls_to(&e, 0x009f_2970).is_empty());
        stub(&mut e, 0x0045_cd60, 0);
        let candidate_table = e.mem.u32(candidate);
        stub(&mut e, 0x00aa_f230, 1);
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 0u8]);
        assert!(calls_to(&e, 0x009f_2970).is_empty());
        let _ = candidate_table;
        stub(&mut e, 0x00aa_f230, 0);
        // The package does not allow the search (0067a7e0 false): nothing is found.
        stub(&mut e, 0x0067_a7e0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 0u8]);
        assert!(calls_to(&e, 0x009f_2970).is_empty());
    }

    #[test]
    fn test_fn_009026c0_walks_back_when_nothing_is_found() {
        let mut e = Engine::new();
        let (proc, actor, state, _candidate) = intruder_setup(&mut e);
        stub(&mut e, 0x0067_a7e0, 0);
        stub(&mut e, 0x008b_3d30, 0);
        let table = e.mem.u32(proc);
        e.mem.set_u32(table + 0x84c, 0x00aa_f84c);
        stub(&mut e, 0x00aa_f84c, 0);
        // Without a package location: the radius 10 squared (100) is positive, so the
        // actor walks (slot 0x84c) to the guard reference with the root 10.
        let guard_table = vtable(&mut e, &[(0x1f4, 0x00aa_f1f4)]);
        let guard = object(&mut e, 0x100, guard_table);
        stub(&mut e, 0x00aa_f128, guard);
        stub_float(&mut e, 0x0040_19d0, 10.0);
        let position = e.mem.alloc(12);
        stub(&mut e, 0x00aa_f1f4, position);
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 1u8]);
        assert_eq!(
            calls_to(&e, 0x00aa_f84c),
            vec![vec![proc, actor, position, 10.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0040_19d0), vec![vec![100.0f32.to_bits()]]);
        // With a package location and a positive 00676280: slot 0x84c gets (actor, 0, -1).
        stub(&mut e, 0x0055_b980, 0x5100);
        stub(&mut e, 0x0067_6280, 3);
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 1u8]);
        assert_eq!(
            calls_to(&e, 0x00aa_f84c),
            vec![vec![proc, actor, 0, (-1.0f32).to_bits()]]
        );
        // Neither: the radius is 0, there is no location; the actor is turned to the
        // override reference's height when it is the form 011ca248 and the height differs.
        stub(&mut e, 0x0055_b980, 0);
        stub(&mut e, 0x0044_ddc0, 0);
        let reference = e.mem.alloc(0x40);
        e.mem.set_u32(proc + 0x44, reference);
        let form = 0x4400u32;
        global(&mut e, 0x011c_a248, form);
        stub(&mut e, 0x007a_f430, form);
        stub(&mut e, 0x008b_3bd0, 0);
        let reference_node = e.mem.alloc(0x20);
        e.mem.set_f32(reference_node + 8, 5.0);
        let actor_node = e.mem.alloc(0x20);
        e.mem.set_f32(actor_node + 8, 1.0);
        e.register_double(0x0043_0830, move |_, a| {
            ret(if a[0] == reference {
                reference_node
            } else {
                actor_node
            })
        });
        e.register_double(0x0040_8840, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        put_double(&mut e, 0x0102_3128, 0.5);
        put_double(&mut e, 0x0101_2060, 0.0);
        stub(&mut e, 0x008b_b5c0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 1u8]);
        assert_eq!(
            calls_to(&e, 0x008b_b5c0),
            vec![vec![actor, 5.0f32.to_bits(), 1]]
        );
        // A gap that is not above 0.5 leaves the actor alone; so does a moving actor.
        e.mem.set_f32(actor_node + 8, 4.8);
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 1u8]);
        assert!(calls_to(&e, 0x008b_b5c0).is_empty());
        e.mem.set_f32(actor_node + 8, 1.0);
        stub(&mut e, 0x008b_3bd0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0090_26c0, &args![proc, actor, state, 1u8]);
        assert!(calls_to(&e, 0x008b_b5c0).is_empty());
    }

    /// Setup for `ProcessGuardWaitToAttack`: a target in the guard's reach.
    /// Returns (process, actor, state, target).
    fn wait_setup(e: &mut Engine) -> (u32, u32, u32, u32) {
        let (proc, actor, state) = guard_setup(e, 2);
        let table = e.mem.u32(proc);
        for (slot, double) in [
            (0x628u32, 0x00aa_f628u32),
            (0x400, 0x00aa_f400),
            (0x638, 0x00aa_f638),
            (0x338, 0x00aa_f338),
            (0x33c, 0x00aa_f33c),
            (0x1b8, 0x00aa_f1b8),
            (0x2a4, 0x00aa_f2a4),
        ] {
            e.mem.set_u32(table + slot, double);
            stub(e, double, 0);
        }
        let animation = e.mem.alloc(0x40);
        stub(e, 0x00aa_f1b8, animation);
        let guard_table = vtable(e, &[(0x1f4, 0x00aa_f1f4)]);
        let guard = object(e, 0x100, guard_table);
        stub(e, 0x00aa_f128, guard);
        let position = e.mem.alloc(12);
        stub(e, 0x00aa_f1f4, position);
        let target_table = vtable(
            e,
            &[
                (0x22c, 0x00aa_f22c),
                (0x2e8, 0x00aa_f2e8),
                (0x230, 0x00aa_f230),
                (0x234, 0x00aa_f234),
                (0x1f4, 0x00aa_f1f4),
            ],
        );
        let target = object(e, 0x200, target_table);
        for slot in [0x00aa_f22c, 0x00aa_f2e8, 0x00aa_f230, 0x00aa_f234] {
            stub(e, slot, 0);
        }
        e.register_double(0x0044_ddc0, move |_, a| {
            // The state's target (0x0044ddc0 is also the package-radius reader).
            ret(if a[0] == state { target } else { 10 })
        });
        stub(e, 0x0043_7bd0, 0);
        stub(e, 0x0043_9ef0, 0);
        put_setting(e, 0x011c_d1f8, 0.5);
        put_setting(e, 0x011c_dd30, 1.5);
        put_setting(e, 0x011c_d5fc, 3.0);
        stub(e, 0x0049_85f0, 1);
        stub(e, 0x0049_8910, 0);
        stub(e, 0x008a_16d0, 1);
        stub(e, 0x008a_6840, 0);
        stub(e, 0x008a_1710, 0);
        stub(e, 0x008b_3d30, 0);
        stub(e, 0x0067_a460, 1);
        // The aim function, run for real, needs these.
        stub(e, 0x009a_8600, position);
        stub(e, 0x008b_3cd0, 0);
        stub(e, 0x009a_8050, position);
        stub(e, 0x009a_6ae0, 1);
        stub_float(e, 0x0084_d030, 0.25);
        stub(e, 0x0090_3160, 0);
        stub_float(e, 0x0062_1b00, 5.0);
        stub(e, 0x0067_5980, 0);
        stub(e, 0x0084_e3a0, 0);
        stub(e, 0x0061_a2d0, 0x9001);
        stub(e, 0x0056_7790, 0);
        (proc, actor, state, target)
    }

    #[test]
    fn test_high_process_process_guard_wait_to_attack_resets_the_state() {
        let mut e = Engine::new();
        let (proc, actor, state, _target) = wait_setup(&mut e);
        // No guard reference: the state returns to 0 and fn_00902640 runs.
        stub(&mut e, 0x00aa_f128, 0);
        e.mem.set_u32(state + 0xc, 4);
        e.call_log = Some(vec![]);
        e.call(0x0090_2c80, &args![proc, actor, state]);
        assert_eq!(calls_to(&e, 0x006e_cd40), vec![vec![state, 0]]);
        assert_eq!(calls_to(&e, 0x00aa_f7f8), vec![vec![proc, actor, state]]);
        assert_eq!(e.mem.u32(state + 0xc), 0);
        // A dead target does the same.
        let guard = e.mem.alloc(0x100);
        let guard_table = vtable(&mut e, &[(0x1f4, 0x00aa_f1f4)]);
        e.mem.set_u32(guard, guard_table);
        stub(&mut e, 0x00aa_f128, guard);
        stub(&mut e, 0x00aa_f230, 1);
        e.call_log = Some(vec![]);
        e.call(0x0090_2c80, &args![proc, actor, state]);
        assert_eq!(calls_to(&e, 0x006e_cd40), vec![vec![state, 0]]);
        // The flag at +0x368 sends a pathless actor to the target first.
        stub(&mut e, 0x00aa_f230, 0);
        e.mem.set_u32(proc + 0x368, 1);
        stub(&mut e, 0x00aa_f128, 0);
        let actor_table = e.mem.u32(actor);
        e.mem.set_u32(actor_table + 0x4b4, 0x00aa_f4b4);
        stub(&mut e, 0x00aa_f4b4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0090_2c80, &args![proc, actor, state]);
        assert_eq!(
            calls_to(&e, 0x00aa_f2a4)[0],
            vec![
                proc,
                e.call(0x0044_ddc0, &args![state]).u32(),
                0,
                0,
                0,
                0,
                0
            ]
        );
    }

    #[test]
    fn test_high_process_process_guard_wait_to_attack_in_range() {
        let mut e = Engine::new();
        let (proc, actor, state, target) = wait_setup(&mut e);
        // Radius 10: 100 squared, the near radius is 5 (25 squared). A length of 50 is in
        // range but not near: the elapsed time 5 is above the setting 3, the count 0 is
        // not above the limit 2: the warning runs.
        stub_float(&mut e, 0x004a_7290, 50.0);
        e.call_log = Some(vec![]);
        e.call(0x0090_2c80, &args![proc, actor, state]);
        assert_eq!(calls_to(&e, 0x00aa_f628), vec![vec![proc, target]]);
        assert_eq!(calls_to(&e, 0x00aa_f638), vec![vec![proc, target]]);
        assert_eq!(
            calls_to(&e, 0x00aa_f338),
            vec![vec![proc, 1.5f32.to_bits()]]
        );
        assert_eq!(e.mem.u8(proc + 0x349), 1);
        assert_eq!(
            calls_to(&e, 0x0090_3160),
            vec![vec![state, 0.25f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(state + 0xc), 1);
        assert!(calls_to(&e, 0x00aa_f33c).is_empty());
        // A count above the limit attacks, and the state resets when the process's
        // own list head is the state.
        stub(&mut e, 0x0084_e3a0, 3);
        e.call_log = Some(vec![]);
        e.call(0x0090_2c80, &args![proc, actor, state]);
        let call = &calls_to(&e, 0x00aa_f33c)[0];
        assert_eq!(call.len(), 14);
        assert_eq!(call[..3], [proc, actor, target]);
        assert_eq!(call[12], 1);
        // Near: attack at once.
        stub(&mut e, 0x0084_e3a0, 0);
        stub_float(&mut e, 0x004a_7290, 10.0);
        e.call_log = Some(vec![]);
        e.call(0x0090_2c80, &args![proc, actor, state]);
        assert_eq!(calls_to(&e, 0x00aa_f33c).len(), 1);
        // The time has not reached the setting: neither attack nor warning.
        stub_float(&mut e, 0x004a_7290, 50.0);
        stub_float(&mut e, 0x0062_1b00, 1.0);
        e.mem.set_u32(state + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0090_2c80, &args![proc, actor, state]);
        assert!(calls_to(&e, 0x00aa_f33c).is_empty());
        assert_eq!(e.mem.u32(state + 0xc), 0);
        // With the process flag nothing after the weapon set-up runs.
        e.mem.set_u32(proc + 0x368, 1);
        e.call_log = Some(vec![]);
        e.call(0x0090_2c80, &args![proc, actor, state]);
        assert!(calls_to(&e, 0x0090_3160).is_empty());
        assert_eq!(calls_to(&e, 0x00aa_f638).len(), 1);
    }

    #[test]
    fn test_high_process_process_guard_wait_to_attack_out_of_range() {
        let mut e = Engine::new();
        let (proc, actor, state, _target) = wait_setup(&mut e);
        stub_float(&mut e, 0x004a_7290, 500.0);
        stub(&mut e, 0x0067_a460, 0);
        e.mem.set_u32(state + 0xc, 4);
        e.mem.set_f32(state + 0x10, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x0090_2c80, &args![proc, actor, state]);
        let guard = e.vcall(proc, 0x128, &args![]).u32();
        assert_eq!(calls_to(&e, 0x00aa_f628), vec![vec![proc, guard]]);
        assert_eq!(calls_to(&e, 0x00aa_f400), vec![vec![proc, 0]]);
        assert_eq!(calls_to(&e, 0x008a_6840), vec![vec![actor, 0]]);
        assert_eq!(calls_to(&e, 0x006e_cd40), vec![vec![state, 1]]);
        assert_eq!(e.mem.u32(state + 0xc), 0);
        assert_eq!(e.mem.f32(state + 0x10), 0.0);
        assert_eq!(calls_to(&e, 0x00aa_f654), vec![vec![proc, 1]]);
        assert_eq!(calls_to(&e, 0x008b_3d30), vec![vec![actor]]);
        // The package allows the stance (0067a460 true): the weapon is left alone.
        stub(&mut e, 0x0067_a460, 1);
        e.call_log = Some(vec![]);
        e.call(0x0090_2c80, &args![proc, actor, state]);
        assert!(calls_to(&e, 0x008a_6840).is_empty());
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on this thread takes (the frame of a function the
    /// test calls directly, outside any other local), with the call log switched on.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.call_log = Some(vec![]);
        e.next_stack_local(size)
    }

    #[test]
    fn aiming_keeps_its_points_at_the_exe_frame_offsets() {
        let mut e = Engine::new();
        for (index, value) in [0.0f32, 0.0, 0.0].into_iter().enumerate() {
            global(&mut e, 0x011f_426c + 4 * index as u32, value.to_bits());
        }
        let attacker = e.mem.alloc(0x100);
        let target = e.mem.alloc(0x100);
        let this = e.mem.alloc(0x500);
        stub(&mut e, 0x008a_1710, 0);
        let attacker_location = e.mem.alloc(12);
        let target_location = e.mem.alloc(12);
        stub(&mut e, 0x009a_8050, attacker_location);
        stub(&mut e, 0x009a_8600, target_location);
        stub(&mut e, 0x008b_3cd0, 0);
        stub(&mut e, 0x009a_6ae0, 1);
        let frame = next_frame(&mut e, 0x84);
        e.call(0x008f_74c0, &args![this, attacker, target, 0u8]);
        // Without a weapon: the firing location at +0x30, the aim point out at +0x18.
        assert_eq!(
            calls_to(&e, 0x009a_8050),
            vec![vec![frame + 0x30, attacker]]
        );
        assert_eq!(calls_to(&e, 0x009a_8600)[0][0], frame + 0x18);
    }

    #[test]
    fn the_combat_reaction_verdict_sits_at_frame_offset_0x20() {
        let mut e = Engine::new();
        let (process, a, b) = react_scene(&mut e);
        let frame = next_frame(&mut e, 0x34);
        e.call(0x008f_fe50, &args![process, a, b]);
        assert_eq!(
            calls_to(&e, ACTOR_GET_SHOULD_ATTACK_ACTOR)[0][3],
            frame + 0x20
        );
    }

    #[test]
    fn the_summon_defence_combat_flag_sits_at_frame_offset_0x17() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let player = e.mem.alloc(0x40);
        global(&mut e, 0x011d_ea3c, player);
        stub(&mut e, 0x0049_3bb0, 0);
        stub(&mut e, 0x0095_3c50, 0);
        let process = summon_process(&mut e, player);
        let frame = next_frame(&mut e, 0x18);
        e.call(0x0090_0020, &args![process, 0xa1u32]);
        assert_eq!(
            calls_to(&e, PLAYER_IS_IN_COMBAT),
            vec![vec![player, frame + 0x17]]
        );
    }

    #[test]
    fn the_pair_evaluation_verdicts_sit_at_the_exe_frame_offsets() {
        let mut e = Engine::new();
        let scene = pair_scene(&mut e);
        let frame = next_frame(&mut e, 0xf0);
        run_pair(&mut e, &scene);
        // The first verdict at +0x64, the second at +0x5c.
        let verdicts: Vec<u32> = calls_to(&e, ACTOR_GET_SHOULD_ATTACK_ACTOR)
            .iter()
            .map(|call| call[3])
            .collect();
        assert_eq!(verdicts, vec![frame + 0x64, frame + 0x5c]);
    }

    #[test]
    fn test_high_process_update_ov2_travel_keeps_its_goal_in_the_frame() {
        let mut e = Engine::new();
        let scene = update_scene(&mut e, &[0, 0]);
        global(&mut e, 0x011c_a248, 0x1234);
        // The package test fails and the pathing is complete: the travel part runs and sets
        // the pathfinding goal from the package coordinates at EBP-0x64 (+0x50).
        stub(&mut e, 0x00aa_b220, 0);
        stub(&mut e, 0x008b_3bb0, 1);
        for addr in [
            0x0067_8670u32,
            0x0067_5a50,
            0x0067_5c20,
            0x0067_5de0,
            0x008b_3690,
        ] {
            stub(&mut e, addr, 0);
        }
        // A saved acquire object ends the travel after the goal.
        stub(&mut e, 0x0045_cd60, 1);
        let frame = next_frame(&mut e, 0xb4);
        e.call_log = Some(vec![]);
        e.call(0x008f_4020, &args![scene.process, scene.actor, 0.0f32]);
        assert_eq!(calls_to(&e, 0x0067_5de0)[0][1], frame + 0x50);
    }
}
