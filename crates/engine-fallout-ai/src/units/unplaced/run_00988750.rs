//! `(unplaced)/run_00988750` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Four `CombatGroup` methods the engine map does not name (their callers
//! are `CombatGroup` code: `MakeSmoothingData`, `UpdateCombatStrength`,
//! `FindSearchLocation`): three array accessors and the update of the
//! group's `CombatTarget` for one target from its members' detection states.
//!
//! x87 note: the game compares in extended precision; the translation
//! compares the same values as `f64` (every operand is a `float` or an
//! integer, exact in `f64`) with the exe's flag tests, NaN included.

// `!(a < b)` mirrors the exe's x87 flag tests, which are true for NaN.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_ai::combatgroup::{CombatGroup, CombatTarget};
use crate::units::fallout_ai::highprocess::DetectionState;

/// `0044ddc0`: a `BSSimpleArray`'s size (thiscall).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `0098ebc0` (engine map: `combatgroup.cpp`): the address of element `i`
/// of `SearchLocations`.
const SEARCH_LOCATION_AT: u32 = 0x0098_ebc0;
/// `006dad90` (engine map: `pathfind.cpp`): the address of element `i` of
/// `MemberArray`.
const MEMBER_AT: u32 = 0x006d_ad90;

/// `00435de0`: stores a `float` argument into the word at `this` (a
/// `CombatTimeStamp` built in a local).
const STORE_FLOAT: u32 = 0x0043_5de0;
/// `00435dd0`: the combat clock (`float` at `011f1bf0`, in ST0).
const COMBAT_TIME: u32 = 0x0043_5dd0;
/// `00435e00`: the combat clock minus the time stamp at `this` (ST0).
const TIME_SINCE: u32 = 0x0043_5e00;
/// `006a7f50`: the `float` at `this` (ST0).
const LOAD_FLOAT: u32 = 0x006a_7f50;
/// `009865d0` (engine map: `combatgroup.cpp`): the group's `CombatTarget`
/// for an actor, or null.
const TARGET_FOR: u32 = 0x0098_65d0;
/// `0084e3a0`: the word at +0x0C of a `BGSWorldLocation`.
const LOCATION_WORD_3: u32 = 0x0084_e3a0;
/// `0097ede0` (engine map: `BGSWorldLocation::CheckSpace`): run on each
/// fresh 16-byte location local before use.
const LOCATION_INIT: u32 = 0x0097_ede0;
/// `00990890` (engine map: `combatgroupstrategy.cpp`): the member count.
const MEMBER_COUNT: u32 = 0x0099_0890;
/// `00903600` (engine map: `highprocess.cpp`): member `i`'s actor.
const MEMBER_ACTOR: u32 = 0x0090_3600;
/// `008d8520`: `MiddleHighProcess::GetSavedAcquireObject` (engine map), as
/// called here: the actor's process.
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// Process slot +0x504: the `DetectionState` about an actor (two words:
/// the actor, 0), or null.
const SLOT_DETECTION_STATE: u32 = 0x504;
/// Process slot +0xF8: a detection event about an actor, or null.
const SLOT_DETECTION_EVENT: u32 = 0xf8;
/// `0066e590` (engine map name `BSSimpleList<DetectionState_P>::AddHead`,
/// folded code): initializes the local `DetectionState`.
const DETECTION_STATE_INIT: u32 = 0x0066_e590;
/// `00931850`: `MobileObject::GetCurrentProcessType` (engine map).
const PROCESS_TYPE: u32 = 0x0093_1850;
/// `00575650`: `TESObjectREFR::GetWorldLocation` (engine map), into a
/// 16-byte local.
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
/// `00527a40`: `BGSWorldLocation::GetDistanceSquared` (engine map).
const DISTANCE_SQUARED: u32 = 0x0052_7a40;
/// `00403e20`: the address of a `Setting`'s float value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `00ec62c0`: `_ftol2` (the value is passed as an `f64`).
const FTOL: u32 = 0x00ec_62c0;
/// Actor slot +0x428: the actor's combat controller (or null); slot
/// +0x42C (no arguments) is what `Script::SetActionFlag` gets.
const SLOT_COMBAT_CONTROLLER: u32 = 0x428;
const SLOT_ACTION_TARGET: u32 = 0x42c;
/// `004030b0`: the controller's target.
const CONTROLLER_TARGET: u32 = 0x0040_30b0;
/// `00586150`: the controller's combat state (thiscall, no arguments).
const CONTROLLER_STATE: u32 = 0x0058_6150;
/// `0099a360` (engine map: `combatstate.cpp`): asked with (actor, target).
const STATE_CHECK: u32 = 0x0099_a360;
/// `00575ca0`: `TESObjectREFR::GetSpace` (engine map).
const GET_SPACE: u32 = 0x0057_5ca0;
/// `00989010`: `BGSWorldLocation::SetLocation` (engine map): point, space.
const SET_LOCATION: u32 = 0x0098_9010;
/// `0097ef30`: whether `iSearchCount` is non-zero.
const IS_SEARCHING: u32 = 0x0097_ef30;
/// The `CombatDialogueManager` (`011f1708`) and `StartDialogue_ov2`
/// (`009839b0`, engine map).
const DIALOGUE_MANAGER: u32 = 0x011f_1708;
const START_DIALOGUE: u32 = 0x0098_39b0;
/// `005d43c0`: the actor's address + 0x44.
const ACTOR_PLUS_44: u32 = 0x005d_43c0;
/// `005ac750`: `Script::SetActionFlag` (engine map), cdecl.
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `0040ebd0`: the larger of two floats (cdecl, ST0).
const FLOAT_MAX: u32 = 0x0040_ebd0;
/// `0098bfb0` (engine map: `combatgroup.cpp`): (location, radius, 0).
const ADD_SEARCH_LOCATION: u32 = 0x0098_bfb0;
/// `011dea3c`: the player character.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;

/// `FLT_MAX` (`float`), negated for "never".
const FLT_MAX: u32 = 0x0108_d540;
/// `67108864.0` (`double`): squared distance under which a member counts
/// as detecting by proximity.
const PROXIMITY_SQUARED: u32 = 0x0108_a7a0;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `5.0` (`double`).
const EVENT_MAX_AGE: u32 = 0x0102_0998;
/// `5.0` (`float`).
const RADIUS_FLOOR: u32 = 0x0101_712c;
/// `20.0`, `55.0`, `2.0`, `40.0` (`double`s).
const LEVEL_DIVISOR: u32 = 0x0102_fc70;
const RADIUS_BASE: u32 = 0x0108_d6e0;
const LOW_LEVEL_DIVISOR: u32 = 0x0101_1590;
const LOW_RADIUS_BASE: u32 = 0x0103_5810;
/// Two `Setting`s: a detection level threshold and a time limit.
const LEVEL_SETTING: u32 = 0x011c_d45c;
const TIME_SETTING: u32 = 0x011c_e9b4;
/// "Not set" for `DetectionState::iDetectedEventLevel`.
const NO_EVENT_LEVEL: u32 = 0x7fff_ffff;

// The function's locals, in one stack block.
const LOCAL_SIZE: u32 = 0x80;
/// 16-byte location updated from the best-timed member's state.
const NEWEST_LOCATION: u32 = 0x00;
/// The target's world location.
const TARGET_LOCATION: u32 = 0x10;
/// The newest detection time seen (a `float` time stamp).
const NEWEST_TIME: u32 = 0x20;
/// A `DetectionState` built when a member is close enough.
const PROXIMITY_STATE: u32 = 0x24;
const MEMBER_LOCATION: u32 = 0x48;
const OTHER_LOCATION: u32 = 0x58;
const SEARCH_LOCATION: u32 = 0x68;
const FLOAT_TEMP: u32 = 0x78;

// Translated from 00988750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size of `SearchLocations` (+0x110).
pub fn fn_00988750(e: &mut Engine, this: Ptr<CombatGroup>) -> u32 {
    e.call(
        ARRAY_SIZE,
        &args![this.addr() + CombatGroup::SEARCH_LOCATIONS],
    )
    .u32()
}

// Translated from 00988770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Element `index` of `SearchLocations` (+0x110).
pub fn fn_00988770(e: &mut Engine, this: Ptr<CombatGroup>, index: u32) -> u32 {
    e.call(
        SEARCH_LOCATION_AT,
        &args![this.addr() + CombatGroup::SEARCH_LOCATIONS, index],
    )
    .u32()
}

// Translated from 00988790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Element `index` of `MemberArray` (+0x18).
pub fn fn_00988790(e: &mut Engine, this: Ptr<CombatGroup>, index: u32) -> u32 {
    e.call(
        MEMBER_AT,
        &args![this.addr() + CombatGroup::MEMBER_ARRAY, index],
    )
    .u32()
}

/// `00435de0` on a stack word: the word the game stores for `bits`.
fn stored_float(e: &mut Engine, frame: u32, bits: u32) -> u32 {
    e.call(STORE_FLOAT, &args![frame + FLOAT_TEMP, bits]);
    e.mem.u32(frame + FLOAT_TEMP)
}

/// `-FLT_MAX` as pushed through the x87 stack.
fn never(e: &Engine) -> u32 {
    x87_quiet_bits(e.global::<u32>(FLT_MAX) ^ 0x8000_0000)
}

/// The combat clock as pushed through the x87 stack.
fn now_bits(e: &mut Engine) -> u32 {
    let now = e.call(COMBAT_TIME, &args![]).f64() as f32;
    x87_quiet_bits(now.to_bits())
}

fn setting(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

fn ftol(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

fn level_setting_int(e: &mut Engine) -> i32 {
    let value = setting(e, LEVEL_SETTING);
    ftol(e, value as f64)
}

fn load_float(e: &mut Engine, addr: u32) -> f64 {
    e.call(LOAD_FLOAT, &args![addr]).f64()
}

fn dialogue_manager(e: &Engine) -> u32 {
    e.global(DIALOGUE_MANAGER)
}

fn copy_location(e: &mut Engine, to: u32, from: u32) {
    for i in 0..4 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 009887b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the group's `CombatTarget` for `target` and reports it:
/// `*detection_level` gets its `iDetectionLevel` (-100 when the group has
/// no such target), `*detected_time` its `fLastDetectedTimeStamp`
/// (`-FLT_MAX` without a target), and `*line_of_sight` / `*view_360` the
/// `DetectionState` flags of `actor` about the target.
///
/// When the target's detection was updated after the clock
/// (`fDetectionLevelUpdateTimeStamp` not before now), only `actor`'s own
/// state is read. Otherwise every member's state about the target is
/// visited (or a fresh state when member and target both have processes
/// and are within `sqrt(67108864)`): line-of-sight and 360° counts, the
/// highest detection level (raised to the event level when it is one above
/// the level setting; a member whose combat controller targets the target
/// and whose combat state `0099a360` refuses keeps the old level), and the
/// newest detected position when the old one has no space. The level is
/// then forced above the setting for a target noticed within a second,
/// search dialogue is started for an undetected target (`fLastDetected`
/// never set: members get `SetActionFlag(0x8000)` and topic 3, else topic
/// 9), and time stamps and locations are refreshed; a strong detection
/// event younger than 5 seconds adds a search location whose radius
/// grows with the event level.
#[allow(clippy::too_many_arguments)]
pub fn fn_009887b0(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    actor: Ptr,
    target: Ptr,
    detection_level: Ptr,
    detected_time: Ptr,
    line_of_sight: Ptr,
    view_360: Ptr,
) {
    e.with_stack(LOCAL_SIZE, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u8(line_of_sight.addr(), 0);
        e.mem.set_u8(view_360.addr(), 0);
        e.mem.set_u32(detection_level.addr(), 0xffff_ff9c);
        let bits = never(e);
        let never_word = stored_float(e, frame, bits);
        e.mem.set_u32(detected_time.addr(), never_word);

        let entry: Ptr<CombatTarget> = e.call(TARGET_FOR, &args![this, target]).ptr();
        if entry.is_null() {
            return;
        }
        let base = entry.addr();
        let entry_level = e.get(entry, CombatTarget::iDetectionLevel);
        let updated = load_float(e, base + 0x54);
        let now = e.call(COMBAT_TIME, &args![]).f64();
        if now > updated {
            update_target(
                e,
                this,
                actor,
                target,
                entry,
                entry_level,
                frame,
                line_of_sight,
                view_360,
            );
        } else {
            let state = if actor.is_null() {
                0
            } else {
                let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                e.vcall(process, SLOT_DETECTION_STATE, &args![target, 0u32])
                    .u32()
            };
            if state != 0 {
                let state: Ptr<DetectionState> = Ptr::new(state);
                let los = e.get(state, DetectionState::bLineofSight);
                e.mem.set_u8(line_of_sight.addr(), los);
                let view = e.get(state, DetectionState::b360view);
                e.mem.set_u8(view_360.addr(), view);
            }
        }
        let level = e.get(entry, CombatTarget::iDetectionLevel);
        e.mem.set_u32(detection_level.addr(), level as u32);
        let detected = e.mem.u32(base + 0x50);
        e.mem.set_u32(detected_time.addr(), detected);
    });
}

/// The main path of [`fn_009887b0`] (the target's level is stale).
#[allow(clippy::too_many_arguments)]
fn update_target(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    actor: Ptr,
    target: Ptr,
    entry: Ptr<CombatTarget>,
    entry_level: i32,
    frame: u32,
    line_of_sight: Ptr,
    view_360: Ptr,
) {
    let base = entry.addr();
    let mut best_level: i32 = -100;
    let mut max_event_level: i32 = -100;
    let detected_location = base + CombatTarget::LAST_DETECTED_LOCATION;
    let track_newest = e.call(LOCATION_WORD_3, &args![detected_location]).u32() == 0;
    let mut best_actor = 0u32;
    e.mem.set_u8(base + 0x65, 0);
    e.mem.set_u8(base + 0x64, 0);
    e.call(LOCATION_INIT, &args![frame + NEWEST_LOCATION]);
    let bits = never(e);
    e.call(STORE_FLOAT, &args![frame + NEWEST_TIME, bits]);
    let count = e.call(MEMBER_COUNT, &args![this]).u32();

    for i in 0..count {
        let member = e.call(MEMBER_ACTOR, &args![this, i]).u32();
        let process = e.call(ACTOR_PROCESS, &args![member]).u32();
        let mut state = e
            .vcall(process, SLOT_DETECTION_STATE, &args![target, 0u32])
            .u32();
        let local_state = frame + PROXIMITY_STATE;
        e.call(DETECTION_STATE_INIT, &args![local_state]);
        if e.call(PROCESS_TYPE, &args![member]).u32() != 0
            && e.call(PROCESS_TYPE, &args![target]).u32() != 0
        {
            e.call(GET_WORLD_LOCATION, &args![member, frame + MEMBER_LOCATION]);
            e.call(GET_WORLD_LOCATION, &args![target, frame + OTHER_LOCATION]);
            let distance = e
                .call(
                    DISTANCE_SQUARED,
                    &args![frame + MEMBER_LOCATION, frame + OTHER_LOCATION],
                )
                .f64();
            if distance < e.global::<f64>(PROXIMITY_SQUARED) {
                e.mem.set_u32(local_state + 8, 1);
                state = local_state;
            }
        }
        if state == 0 {
            continue;
        }
        let ptr: Ptr<DetectionState> = Ptr::new(state);
        if e.get(ptr, DetectionState::bLineofSight) != 0 {
            let n = e.mem.u8(base + 0x64);
            e.mem.set_u8(base + 0x64, n.wrapping_add(1));
        }
        if e.get(ptr, DetectionState::b360view) != 0 {
            let n = e.mem.u8(base + 0x65);
            e.mem.set_u8(base + 0x65, n.wrapping_add(1));
        }
        if member == actor.addr() {
            let los = e.get(ptr, DetectionState::bLineofSight);
            e.mem.set_u8(line_of_sight.addr(), los);
            let view = e.get(ptr, DetectionState::b360view);
            e.mem.set_u8(view_360.addr(), view);
        }
        let mut level = e.get(ptr, DetectionState::iLevel);
        let event_level = e.get(ptr, DetectionState::iDetectedEventLevel);
        if event_level != NO_EVENT_LEVEL {
            let event_level = event_level as i32;
            if max_event_level < event_level {
                max_event_level = event_level;
            }
            let threshold = level_setting_int(e);
            if level == threshold.wrapping_add(1) {
                let threshold = setting(e, LEVEL_SETTING);
                if (threshold as f64) < event_level as f64 {
                    let threshold = setting(e, LEVEL_SETTING);
                    level = ftol(e, threshold as f64);
                }
            }
        }
        let controller = e.vcall(member, SLOT_COMBAT_CONTROLLER, &args![]).u32();
        let refused = controller != 0
            && e.call(CONTROLLER_TARGET, &args![controller]).u32() == target.addr()
            && {
                let combat_state = e.call(CONTROLLER_STATE, &args![controller]).u32();
                !e.call(STATE_CHECK, &args![combat_state, member, target])
                    .bool()
            };
        if refused {
            if entry_level > best_level {
                best_level = entry_level;
                best_actor = member;
            }
        } else if level > best_level {
            best_level = level;
            best_actor = member;
        }
        if track_newest {
            let state_time = load_float(e, state + 0x18);
            let newest = load_float(e, frame + NEWEST_TIME);
            if newest < state_time {
                let space = e.call(GET_SPACE, &args![member]).u32();
                e.call(
                    SET_LOCATION,
                    &args![frame + NEWEST_LOCATION, state + 0x0c, space],
                );
                let time = e.mem.u32(state + 0x18);
                e.mem.set_u32(frame + NEWEST_TIME, time);
            }
        }
    }

    let threshold = level_setting_int(e);
    if best_level <= threshold {
        let since = e.call(TIME_SINCE, &args![base + 0x60]).f64();
        let one: f64 = e.global(ONE);
        if since < one {
            let value = setting(e, LEVEL_SETTING);
            best_level = ftol(e, value as f64 + one);
        }
    }

    if e.call(IS_SEARCHING, &args![this]).bool() && entry_level <= 0 && best_level > 0 {
        let last_detected = load_float(e, base + 0x50);
        let never_time = -(e.global::<f32>(FLT_MAX) as f64);
        if never_time == last_detected {
            for j in 0..count {
                let member = e.call(MEMBER_ACTOR, &args![this, j]).u32();
                if member != e.global::<u32>(PLAYER_CHARACTER) {
                    let flags = e.call(ACTOR_PLUS_44, &args![member]).u32();
                    let action = e.vcall(member, SLOT_ACTION_TARGET, &args![]).u32();
                    e.call(SET_ACTION_FLAG, &args![action, flags, 0x8000u32]);
                }
            }
            e.call(
                START_DIALOGUE,
                &args![
                    dialogue_manager(e),
                    best_actor,
                    target,
                    4u32,
                    3u32,
                    1u32,
                    0u32
                ],
            );
        } else {
            e.call(
                START_DIALOGUE,
                &args![
                    dialogue_manager(e),
                    best_actor,
                    target,
                    4u32,
                    9u32,
                    1u32,
                    0u32
                ],
            );
        }
    }

    e.set(entry, CombatTarget::iDetectionLevel, best_level);
    let bits = now_bits(e);
    let word = stored_float(e, frame, bits);
    e.mem.set_u32(base + 0x54, word);
    e.call(GET_WORLD_LOCATION, &args![target, frame + TARGET_LOCATION]);
    if best_level > 0 {
        let bits = now_bits(e);
        let word = stored_float(e, frame, bits);
        e.mem.set_u32(base + 0x50, word);
    }
    let since_detected = e.call(TIME_SINCE, &args![base + 0x50]).f64();
    let limit = setting(e, TIME_SETTING);
    if limit as f64 > since_detected {
        copy_location(e, detected_location, frame + TARGET_LOCATION);
    } else if track_newest {
        copy_location(e, detected_location, frame + NEWEST_LOCATION);
    }
    let threshold = level_setting_int(e);
    if best_level > threshold {
        let bits = now_bits(e);
        let word = stored_float(e, frame, bits);
        e.mem.set_u32(base + 0x58, word);
        copy_location(
            e,
            base + CombatTarget::LAST_NOTICED_LOCATION,
            frame + TARGET_LOCATION,
        );
        let search_count = e.mem.u16(this.addr() + 0xd0);
        e.set(entry, CombatTarget::sLastSearchNoticed, search_count);
    }
    if e.mem.u8(base + 0x65) != 0 {
        copy_location(
            e,
            base + CombatTarget::LAST_SEEN_LOCATION,
            frame + TARGET_LOCATION,
        );
        let bits = now_bits(e);
        let word = stored_float(e, frame, bits);
        e.mem.set_u32(base + 0x4c, word);
    }

    let threshold = setting(e, LEVEL_SETTING);
    if (threshold as f64) < max_event_level as f64 && e.call(IS_SEARCHING, &args![this]).bool() {
        let process = e.call(ACTOR_PROCESS, &args![target]).u32();
        let event = e.vcall(process, SLOT_DETECTION_EVENT, &args![target]).u32();
        if event == 0 {
            return;
        }
        let age = e.call(TIME_SINCE, &args![event + 0x10]).f64();
        if !(age < e.global::<f64>(EVENT_MAX_AGE)) {
            return;
        }
        let event_time = load_float(e, event + 0x10);
        let handled = load_float(e, base + 0x5c);
        if !(handled < event_time) {
            return;
        }
        let radius: f32 = if max_event_level > 0 {
            let floor = x87_quiet_bits(e.global::<u32>(RADIUS_FLOOR));
            let scaled = (max_event_level as f64 / e.global::<f64>(LEVEL_DIVISOR)) as f32;
            let larger = e
                .call(FLOAT_MAX, &args![x87_quiet_bits(scaled.to_bits()), floor])
                .f64();
            (larger + e.global::<f64>(RADIUS_BASE)) as f32
        } else {
            let threshold = level_setting_int(e);
            let difference = max_event_level.wrapping_sub(threshold);
            (difference as f64 / e.global::<f64>(LOW_LEVEL_DIVISOR)
                + e.global::<f64>(LOW_RADIUS_BASE)) as f32
        };
        e.call(LOCATION_INIT, &args![frame + SEARCH_LOCATION]);
        let space = e.call(GET_SPACE, &args![target]).u32();
        e.call(
            SET_LOCATION,
            &args![frame + SEARCH_LOCATION, event + 4, space],
        );
        e.call(
            ADD_SEARCH_LOCATION,
            &args![
                this,
                frame + SEARCH_LOCATION,
                x87_quiet_bits(radius.to_bits()),
                0u32
            ],
        );
        let stamp = e.mem.u32(event + 0x10);
        e.mem.set_u32(base + 0x5c, stamp);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00988750, fn_00988750(Ptr<CombatGroup>) -> u32),
        entry!(0x00988770, fn_00988770(Ptr<CombatGroup>, u32) -> u32),
        entry!(0x00988790, fn_00988790(Ptr<CombatGroup>, u32) -> u32),
        entry!(
            0x009887b0,
            fn_009887b0(Ptr<CombatGroup>, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const MEMBER_VTABLE: u32 = 0x0002_0000;
    const STATE_DOUBLE: u32 = 0x0001_0000;
    const CONTROLLER_DOUBLE: u32 = 0x0001_0010;
    const ACTION_DOUBLE: u32 = 0x0001_0020;
    const EVENT_DOUBLE: u32 = 0x0001_0030;
    /// Where the doubles find the combat clock, the target entry and the
    /// member list.
    const CLOCK: u32 = 0x011f_1bf0;
    const ENTRY_CELL: u32 = 0x0003_0000;
    const MEMBERS: u32 = 0x0003_0010;

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn st0_f32(value: f32) -> Ret {
        st0(value as f64)
    }

    /// A "member": vtable at +0; the doubles read its detection state at
    /// +4, its controller at +8, its event at +0xC and its process type at
    /// +0x14. The member is its own process.
    fn member(e: &mut Engine) -> u32 {
        let m = e.mem.alloc(0x20);
        e.mem.set_u32(m, MEMBER_VTABLE);
        m
    }

    fn state(e: &mut Engine, level: i32, los: u8, view: u8, event: u32, time: f32) -> u32 {
        let s = e.mem.alloc(0x24);
        e.mem.set_i32(s + 8, level);
        e.mem.set_u8(s + 0x1e, los);
        e.mem.set_u8(s + 0x1c, view);
        e.mem.set_u32(s + 0x20, event);
        e.mem.set_f32(s + 0x18, time);
        s
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0108_d000,
            0x0108_a000,
            0x0101_2000,
            0x0102_0000,
            0x0101_7000,
            0x0102_f000,
            0x0101_1000,
            0x0103_5000,
            0x011c_d000,
            0x011c_e000,
            0x011f_1000,
            0x011d_e000,
            0x0003_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(FLT_MAX, f32::MAX);
        e.set_global(PROXIMITY_SQUARED, 67_108_864.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(EVENT_MAX_AGE, 5.0f64);
        e.set_global(RADIUS_FLOOR, 5.0f32);
        e.set_global(LEVEL_DIVISOR, 20.0f64);
        e.set_global(RADIUS_BASE, 55.0f64);
        e.set_global(LOW_LEVEL_DIVISOR, 2.0f64);
        e.set_global(LOW_RADIUS_BASE, 40.0f64);
        e.set_global(LEVEL_SETTING + 4, 49.0f32);
        e.set_global(TIME_SETTING + 4, 30.0f32);
        e.set_global(DIALOGUE_MANAGER, 0x7700u32);
        e.set_global(CLOCK, 100.0f32);

        e.register(STORE_FLOAT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(COMBAT_TIME, |e, _| st0_f32(e.mem.f32(CLOCK)));
        e.register(TIME_SINCE, |e, a| {
            st0_f32((e.mem.f32(CLOCK) as f64 - e.mem.f32(a[0]) as f64) as f32)
        });
        e.register(LOAD_FLOAT, |e, a| st0_f32(e.mem.f32(a[0])));
        e.register(TARGET_FOR, |e, _| e.mem.u32(ENTRY_CELL).into_ret());
        e.register(LOCATION_WORD_3, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(MEMBER_COUNT, |e, _| e.mem.u32(MEMBERS).into_ret());
        e.register(MEMBER_ACTOR, |e, a| {
            e.mem.u32(MEMBERS + 4 + 4 * a[1]).into_ret()
        });
        e.register(ACTOR_PROCESS, |_, a| a[0].into_ret());
        e.register(STATE_DOUBLE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(CONTROLLER_DOUBLE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ACTION_DOUBLE, |_, a| (a[0] + 0x100).into_ret());
        e.register(EVENT_DOUBLE, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(PROCESS_TYPE, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(GET_WORLD_LOCATION, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[1] + 4 * i, a[0] + i);
            }
            Ret::default()
        });
        e.register(DISTANCE_SQUARED, |_, _| st0(100.0));
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        e.register(CONTROLLER_TARGET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CONTROLLER_STATE, |_, a| a[0].into_ret());
        e.register(STATE_CHECK, |_, _| Ret::default());
        e.register(GET_SPACE, |_, _| 0x5000u32.into_ret());
        e.register(IS_SEARCHING, |e, a| {
            (e.mem.u32(a[0] + 0xd0) != 0).into_ret()
        });
        e.register(ACTOR_PLUS_44, |_, a| (a[0] + 0x44).into_ret());
        e.register(FLOAT_MAX, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            st0_f32(if x > y { x } else { y })
        });
        for addr in [
            LOCATION_INIT,
            DETECTION_STATE_INIT,
            SET_LOCATION,
            START_DIALOGUE,
            SET_ACTION_FLAG,
            ADD_SEARCH_LOCATION,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.put_vtable(MEMBER_VTABLE, &[0; 0x142]);
        e.mem
            .set_u32(MEMBER_VTABLE + SLOT_DETECTION_STATE, STATE_DOUBLE);
        e.mem
            .set_u32(MEMBER_VTABLE + SLOT_COMBAT_CONTROLLER, CONTROLLER_DOUBLE);
        e.mem
            .set_u32(MEMBER_VTABLE + SLOT_ACTION_TARGET, ACTION_DOUBLE);
        e.mem
            .set_u32(MEMBER_VTABLE + SLOT_DETECTION_EVENT, EVENT_DOUBLE);
        e
    }

    fn set_members(e: &mut Engine, members: &[u32]) {
        e.mem.set_u32(MEMBERS, members.len() as u32);
        for (i, m) in members.iter().enumerate() {
            e.mem.set_u32(MEMBERS + 4 + 4 * i as u32, *m);
        }
    }

    struct Outputs {
        level: Ptr,
        time: Ptr,
        los: Ptr,
        view: Ptr,
    }

    fn outputs(e: &mut Engine) -> Outputs {
        let block = e.mem.alloc(16);
        e.mem.write(block, &[0xcc; 16]);
        Outputs {
            level: Ptr::new(block),
            time: Ptr::new(block + 4),
            los: Ptr::new(block + 8),
            view: Ptr::new(block + 12),
        }
    }

    fn run(e: &mut Engine, group: Ptr<CombatGroup>, actor: u32, target: u32, out: &Outputs) {
        e.call(
            0x0098_87b0,
            &args![group, actor, target, out.level, out.time, out.los, out.view],
        );
    }

    #[test]
    fn accessors_pass_the_arrays() {
        let mut e = Engine::new();
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(SEARCH_LOCATION_AT, |_, a| (a[0] + a[1]).into_ret());
        e.register(MEMBER_AT, |_, a| (a[0] + 2 * a[1]).into_ret());
        let group: Ptr<CombatGroup> = e.new_object();
        e.mem.set_u32(group.addr() + 0x118, 6);
        assert_eq!(e.call(0x0098_8750, &args![group]).u32(), 6);
        assert_eq!(
            e.call(0x0098_8770, &args![group, 3u32]).u32(),
            group.addr() + 0x113
        );
        assert_eq!(
            e.call(0x0098_8790, &args![group, 3u32]).u32(),
            group.addr() + 0x1e
        );
    }

    #[test]
    fn without_a_target_entry_only_the_defaults_are_written() {
        let mut e = engine();
        let group: Ptr<CombatGroup> = e.new_object();
        let out = outputs(&mut e);
        run(&mut e, group, 0, 0x4444, &out);
        assert_eq!(e.mem.i32(out.level.addr()), -100);
        assert_eq!(e.mem.f32(out.time.addr()), -f32::MAX);
        assert_eq!(e.mem.u8(out.los.addr()), 0);
        assert_eq!(e.mem.u8(out.view.addr()), 0);
    }

    #[test]
    fn fresh_entry_reads_only_the_actor_state() {
        let mut e = engine();
        let group: Ptr<CombatGroup> = e.new_object();
        let entry: Ptr<CombatTarget> = e.new_object();
        e.mem.set_u32(ENTRY_CELL, entry.addr());
        e.set(entry, CombatTarget::iDetectionLevel, 12);
        e.mem.set_f32(entry.addr() + 0x50, 42.0);
        e.mem.set_f32(entry.addr() + 0x54, 100.0);
        let actor = member(&mut e);
        let s = state(&mut e, 0, 1, 1, NO_EVENT_LEVEL, 0.0);
        e.mem.set_u32(actor + 4, s);
        let out = outputs(&mut e);
        e.call_log = Some(vec![]);
        run(&mut e, group, actor, 0x4444, &out);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, MEMBER_COUNT).is_empty());
        assert_eq!(calls(&log, STATE_DOUBLE), vec![vec![actor, 0x4444, 0]]);
        assert_eq!(e.mem.i32(out.level.addr()), 12);
        assert_eq!(e.mem.f32(out.time.addr()), 42.0);
        assert_eq!(e.mem.u8(out.los.addr()), 1);
        assert_eq!(e.mem.u8(out.view.addr()), 1);
    }

    #[test]
    fn stale_entry_takes_the_best_member_and_starts_search_dialogue() {
        let mut e = engine();
        let group: Ptr<CombatGroup> = e.new_object();
        e.mem.set_u32(group.addr() + 0xd0, 2);
        let entry: Ptr<CombatTarget> = e.new_object();
        e.mem.set_u32(ENTRY_CELL, entry.addr());
        e.mem.set_f32(entry.addr() + 0x50, -f32::MAX);
        e.mem.set_f32(entry.addr() + 0x54, 50.0);
        e.mem.set_f32(entry.addr() + 0x60, 99.5);
        let m1 = member(&mut e);
        let s1 = state(&mut e, 30, 1, 0, NO_EVENT_LEVEL, 20.0);
        e.mem.set_u32(m1 + 4, s1);
        let m2 = member(&mut e);
        let s2 = state(&mut e, 50, 0, 1, 80, 90.0);
        e.mem.set_u32(m2 + 4, s2);
        set_members(&mut e, &[m1, m2]);
        // The target's detection event, 2 seconds old.
        let target = member(&mut e);
        let event = e.mem.alloc(0x14);
        e.mem.set_f32(event + 0x10, 98.0);
        e.mem.set_u32(target + 0xc, event);
        let out = outputs(&mut e);
        e.call_log = Some(vec![]);
        run(&mut e, group, m2, target, &out);
        let log = e.call_log.take().unwrap();

        // m2's level 50 is one above the setting and its event level 80 is
        // above the setting: the level becomes 49; noticed within a
        // second: 50.
        assert_eq!(e.get(entry, CombatTarget::iDetectionLevel), 50);
        assert_eq!(e.mem.u8(entry.addr() + 0x64), 1);
        assert_eq!(e.mem.u8(entry.addr() + 0x65), 1);
        let set_location = calls(&log, SET_LOCATION);
        assert_eq!(set_location.len(), 3);
        assert_eq!(set_location[0][1..], [s1 + 0xc, 0x5000]);
        assert_eq!(set_location[1][1..], [s2 + 0xc, 0x5000]);
        assert_eq!(set_location[0][0], set_location[1][0]);
        assert_eq!(set_location[2][1..], [event + 4, 0x5000]);
        assert_eq!(
            calls(&log, SET_ACTION_FLAG),
            vec![
                vec![m1 + 0x100, m1 + 0x44, 0x8000],
                vec![m2 + 0x100, m2 + 0x44, 0x8000]
            ]
        );
        assert_eq!(
            calls(&log, START_DIALOGUE),
            vec![vec![0x7700, m2, target, 4, 3, 1, 0]]
        );
        for off in [0x4c, 0x50, 0x54, 0x58] {
            assert_eq!(e.mem.f32(entry.addr() + off), 100.0, "+{off:x}");
        }
        for location in [0x08, 0x18, 0x28] {
            assert_eq!(e.mem.u32(entry.addr() + location), target);
            assert_eq!(e.mem.u32(entry.addr() + location + 0xc), target + 3);
        }
        assert_eq!(e.get(entry, CombatTarget::sLastSearchNoticed), 2);
        let add = &calls(&log, ADD_SEARCH_LOCATION)[0];
        assert_eq!(add[0], group.addr());
        assert_eq!(add[1], set_location[2][0]);
        assert_eq!(f32::from_bits(add[2]), 60.0);
        assert_eq!(add[3], 0);
        assert_eq!(e.mem.f32(entry.addr() + 0x5c), 98.0);
        assert_eq!(e.mem.i32(out.level.addr()), 50);
        assert_eq!(e.mem.f32(out.time.addr()), 100.0);
        assert_eq!(e.mem.u8(out.los.addr()), 0);
        assert_eq!(e.mem.u8(out.view.addr()), 1);
    }

    #[test]
    fn close_member_whose_combat_state_refuses_keeps_the_old_level() {
        let mut e = engine();
        let group: Ptr<CombatGroup> = e.new_object();
        let entry: Ptr<CombatTarget> = e.new_object();
        e.mem.set_u32(ENTRY_CELL, entry.addr());
        e.set(entry, CombatTarget::iDetectionLevel, 7);
        e.mem.set_f32(entry.addr() + 0x54, 50.0);
        // The old detected location has a space: no newest-location tracking.
        e.mem.set_u32(entry.addr() + 0x18 + 0xc, 0x99);
        let target = member(&mut e);
        e.mem.set_u32(target + 0x14, 1);
        let m1 = member(&mut e);
        e.mem.set_u32(m1 + 0x14, 1);
        // Its controller targets the target.
        let controller = e.mem.alloc(4);
        e.mem.set_u32(controller, target);
        e.mem.set_u32(m1 + 8, controller);
        set_members(&mut e, &[m1]);
        let out = outputs(&mut e);
        e.call_log = Some(vec![]);
        run(&mut e, group, 0, target, &out);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, STATE_CHECK), vec![vec![controller, m1, target]]);
        assert_eq!(e.get(entry, CombatTarget::iDetectionLevel), 7);
        assert!(calls(&log, SET_LOCATION).is_empty());
        assert!(calls(&log, START_DIALOGUE).is_empty());
        assert!(calls(&log, ADD_SEARCH_LOCATION).is_empty());
        assert_eq!(e.mem.i32(out.level.addr()), 7);
    }
}
