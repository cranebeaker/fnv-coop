//! `(unplaced)/run_0097e490` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `0097e4b0` is a combat controller's update by its callees (`CombatController::
//! SetMovementProcedure` `009801b0`, `SetActionProcedure` `00980110`, unit
//! `combatcontroller.cpp`); its fields are unnamed here and read at their offsets: the
//! group +0x80, the procedures +0x84 and +0x88, the combat state +0x9C, a timer +0xB4,
//! the actor +0xBC, the target +0xC0, a byte +0xE4, the movement state +0x144 and a
//! float +0x148.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0097e490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): 1 when the word at +8 is not below
/// the word at +4 (unsigned), else 0.
pub fn fn_0097e490(e: &mut Engine, this: Ptr) -> u32 {
    let at = this.addr();
    u32::from(e.mem.u32(at + 8) >= e.mem.u32(at + 4))
}

/// `0097d990` (`thiscall`): the first-update set-up.
const INITIALIZE_0097D990: u32 = 0x0097_d990;
/// `008d7f40` (`thiscall`, one float) and `008d7f80` (`thiscall`, `bool`) on the timer at
/// +0xB4; `00435e00` (`thiscall`, ST0) reads it.
const TIMER_RESET_008D7F40: u32 = 0x008d_7f40;
const TIMER_DONE_008D7F80: u32 = 0x008d_7f80;
const TIMER_VALUE_00435E00: u32 = 0x0043_5e00;
/// `00408d60` (`thiscall` on `011f1840`): the address of a byte setting.
const BYTE_SETTING_00408D60: u32 = 0x0040_8d60;
const SETTING_011F1840: u32 = 0x011f_1840;
/// `00567d90` (`thiscall`; `tesobjectrefr.cpp`): compared for the actor and the target.
const REFERENCE_00567D90: u32 = 0x0056_7d90;
/// `00980830` (`thiscall`, one word): sets the target.
const SET_TARGET_00980830: u32 = 0x0098_0830;
/// `bool` checks of the controller (`thiscall`): `00980470`, `009804d0`.
const CHECK_00980470: u32 = 0x0098_0470;
const CHECK_009804D0: u32 = 0x0098_04d0;
/// `0097e310` (`thiscall` on a procedure, `bool`) and `0097f270` (`thiscall`, one word).
const PROCEDURE_CHECK_0097E310: u32 = 0x0097_e310;
const PROCEDURE_CALL_0097F270: u32 = 0x0097_f270;
/// `00980250` (`thiscall`): clears the movement procedure.
const CLEAR_00980250: u32 = 0x0098_0250;
/// `00986c60` (`thiscall` on the group, one word): a new target for the actor.
const GROUP_TARGET_00986C60: u32 = 0x0098_6c60;
/// `00997cb0` (`thiscall` on the combat state).
const STATE_UPDATE_00997CB0: u32 = 0x0099_7cb0;
/// `TESObjectREFR::GetWorldLocation` (`00575650`, engine map; `thiscall`, an out location).
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
/// `bool` checks of the combat state (`thiscall`): `009a1380`, `0097ee90`, `0097eee0`; and
/// `0097ef30` of the group.
const STATE_CHECK_009A1380: u32 = 0x009a_1380;
const STATE_CHECK_0097EE90: u32 = 0x0097_ee90;
const STATE_CHECK_0097EEE0: u32 = 0x0097_eee0;
const GROUP_CHECK_0097EF30: u32 = 0x0097_ef30;
/// `BGSWorldLocation::GetDistanceSquared` (`00527a40`, engine map; `thiscall`, one word;
/// ST0).
const GET_DISTANCE_SQUARED: u32 = 0x0052_7a40;
/// `00403e20` (`thiscall` on a float setting): the address of its value; the setting at
/// `011ce820` (squared by the code).
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_011CE820: u32 = 0x011c_e820;
/// `00662a80` (`thiscall`, a byte) and `0097ce70` (`thiscall`, one word) on the combat state
/// (the engine map's names, `BSFaceGenNiNode::GetApplyRotToParent`/`Set..`, are folded
/// bodies').
const STATE_FLAG_00662A80: u32 = 0x0066_2a80;
const STATE_SET_0097CE70: u32 = 0x0097_ce70;
/// The game's `operator new` (`00401000`, `cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// Procedure constructors (`thiscall`): `009cbad0`, `009d69f0` (a location, a float, a
/// word), `009d5ad0` (one word) and `CombatProcedureSearch::CombatProcedureSearch`
/// (`009d8b60`, engine map).
const PROCEDURE_009CBAD0: u32 = 0x009c_bad0;
const PROCEDURE_009D69F0: u32 = 0x009d_69f0;
const PROCEDURE_009D5AD0: u32 = 0x009d_5ad0;
const COMBAT_PROCEDURE_SEARCH: u32 = 0x009d_8b60;
/// `CombatController::SetActionProcedure` (`00980110`) and `SetMovementProcedure`
/// (`009801b0`) (engine map; `thiscall`, one word).
const SET_ACTION_PROCEDURE: u32 = 0x0098_0110;
const SET_MOVEMENT_PROCEDURE: u32 = 0x0098_01b0;
/// The location calls: `BGSWorldLocation::SetSpace` (`0097ee20`, the engine map's name for
/// the constructor-like call made on a fresh local), `CombatLocation::SetRef` (`00997400`),
/// `006815c0` (`thiscall`: the address it passes on), `BGSWorldLocation::CheckSpace`
/// (`0097ede0`), `0097ee60` (`thiscall`, one word) (engine map names).
const LOCATION_INIT_0097EE20: u32 = 0x0097_ee20;
const COMBAT_LOCATION_SET_REF: u32 = 0x0099_7400;
const LOCATION_ADDRESS_006815C0: u32 = 0x0068_15c0;
const LOCATION_INIT_0097EDE0: u32 = 0x0097_ede0;
const LOCATION_SET_0097EE60: u32 = 0x0097_ee60;
/// `CombatState::CheckMovement` (`009a02a0`, engine map; `thiscall`, three words).
const COMBAT_STATE_CHECK_MOVEMENT: u32 = 0x009a_02a0;
/// `009890e0` (`thiscall` on the group, two words, `bool`).
const GROUP_LOCATION_009890E0: u32 = 0x0098_90e0;
/// `CombatState::FindBestFleeLocation` (`0099ebe0`, engine map; `thiscall`, three words).
const FIND_BEST_FLEE_LOCATION: u32 = 0x0099_ebe0;
/// `0097be60` (`thiscall` on a location, one word, `bool`).
const LOCATION_CHECK_0097BE60: u32 = 0x0097_be60;
/// `Actor::IsPathing` (`008b3bd0`, engine map) and `00575770` (`thiscall`, one float) on
/// the actor.
const ACTOR_IS_PATHING: u32 = 0x008b_3bd0;
const ACTOR_SET_00575770: u32 = 0x0057_5770;
/// Constants: `double 0.0` (`01012060`), `double 100.0` (`01017a40`), the floats passed to
/// `009d69f0` (`011a5654`, `011a56b4`) and to `00575770` (`01016970`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
const VALUE_011A5654: u32 = 0x011a_5654;
const VALUE_011A56B4: u32 = 0x011a_56b4;
const VALUE_01016970: u32 = 0x0101_6970;

/// `operator new(size)`, then `construct` on the block (0 when the allocation fails).
fn new_object(e: &mut Engine, size: u32, construct: impl FnOnce(&mut Engine, u32) -> u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block != 0 {
        construct(e, block)
    } else {
        0
    }
}

/// The setting `011ce820` squared (`FLD`, `FMUL` of the same value).
fn setting_squared(e: &mut Engine) -> f64 {
    let first = e.call(FLOAT_SETTING_VALUE, &args![SETTING_011CE820]).u32();
    let second = e.call(FLOAT_SETTING_VALUE, &args![SETTING_011CE820]).u32();
    x87_mul(x87_load32(e.mem.u32(first)), x87_m32(e.mem.u32(second)))
}

// Translated from 0097e4b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097e4b0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f19304, the state variable that frees a procedure block when its constructor throws) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): the combat controller's update.
/// On the first run `0097d990` (byte +0xE4). With the float +0x148 below 0.0 the timer
/// restarts (1.0), +0x148 is cleared and, with the byte setting `011f1840` set, a target
/// whose `00567d90` differs from the actor's drops (`00980830(0)`). When the timer is done
/// (`008d7f80`): the procedures +0x84/+0x88 get `0097f270` (when `0097e310`) and the target
/// drops when `00980470` answers true, else `009804d0` clears the movement procedure and
/// state +0x144 (remembered); without a target the group picks one (`00986c60`). With a
/// target the movement state is chosen: 5 (`009a1380`), 1 or 2 (`0097ee90`: whether the
/// squared setting `011ce820` passes the squared distance), 3 (`0097eee0` false), 4
/// (`0097ef30` of the group) or 0; a new state clears the procedure and starts its own
/// (1: action `009cbad0`; 2: movement `009d69f0` toward the target when `CheckMovement`
/// allows; 3: a group location beyond 100 (`009890e0`, `009d69f0`), else state 0; 4:
/// `CombatProcedureSearch`; 5: the combat state's flag 2 when its flag was set after the
/// clear, else flag 1 and a flee location (`FindBestFleeLocation`: `009d5ad0` or
/// `009d69f0`) when the setting passes the distance, else flag 2). Then the procedures'
/// slot +4 runs, a pathing actor gets the timer's value into +0x148 and `00575770`
/// (`01016970`), and the timer restarts (1.0).
pub fn fn_0097e4b0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if e.mem.u8(at + 0xe4) == 0 {
        e.call(INITIALIZE_0097D990, &args![at]);
        e.mem.set_u8(at + 0xe4, 1);
    }
    let one = 1.0f32.to_bits();
    if x87_load32(e.mem.u32(at + 0x148)) < e.mem.f64(ZERO_DOUBLE) {
        e.call(TIMER_RESET_008D7F40, &args![at + 0xb4, one]);
        e.mem.set_u32(at + 0x148, 0.0f32.to_bits());
        if e.mem.u32(at + 0xc0) != 0 {
            let setting = e
                .call(BYTE_SETTING_00408D60, &args![SETTING_011F1840])
                .u32();
            if e.mem.u8(setting) != 0 {
                let target = e.mem.u32(at + 0xc0);
                let target = e.call(REFERENCE_00567D90, &args![target]).u32();
                let actor = e.mem.u32(at + 0xbc);
                let actor = e.call(REFERENCE_00567D90, &args![actor]).u32();
                if target != actor {
                    e.call(SET_TARGET_00980830, &args![at, 0u32]);
                }
            }
        }
    }
    if !e.call(TIMER_DONE_008D7F80, &args![at + 0xb4]).bool() {
        return;
    }
    let finish = e.with_stack(0x120, |e, frame| {
        // The frame's locals at their offsets from EBP.
        let ebp = frame.addr() + 0x120;
        let mut cleared = false;
        if e.call(CHECK_00980470, &args![at]).bool() {
            for field in [0x84u32, 0x88] {
                let procedure = e.mem.u32(at + field);
                if procedure != 0 && e.call(PROCEDURE_CHECK_0097E310, &args![procedure]).bool() {
                    let procedure = e.mem.u32(at + field);
                    e.call(PROCEDURE_CALL_0097F270, &args![at, procedure]);
                }
            }
            e.call(SET_TARGET_00980830, &args![at, 0u32]);
        } else if e.call(CHECK_009804D0, &args![at]).bool() {
            e.call(CLEAR_00980250, &args![at]);
            e.mem.set_u32(at + 0x144, 0);
            cleared = true;
        }
        if e.mem.u32(at + 0xc0) == 0 {
            let actor = e.mem.u32(at + 0xbc);
            let group = e.mem.u32(at + 0x80);
            let target = e.call(GROUP_TARGET_00986C60, &args![group, actor]).u32();
            e.call(SET_TARGET_00980830, &args![at, target]);
            e.call(CLEAR_00980250, &args![at]);
            e.mem.set_u32(at + 0x144, 0);
        }
        if e.mem.u32(at + 0xc0) == 0 {
            return false;
        }
        let state_object = e.mem.u32(at + 0x9c);
        e.call(STATE_UPDATE_00997CB0, &args![state_object]);
        let actor_location = ebp - 0x34;
        let target_location = ebp - 0x20;
        let actor = e.mem.u32(at + 0xbc);
        e.call(GET_WORLD_LOCATION, &args![actor, actor_location]);
        let target = e.mem.u32(at + 0xc0);
        e.call(GET_WORLD_LOCATION, &args![target, target_location]);
        let state_object = e.mem.u32(at + 0x9c);
        let state = if e.call(STATE_CHECK_009A1380, &args![state_object]).bool() {
            5
        } else if e.call(STATE_CHECK_0097EE90, &args![state_object]).bool() {
            let distance = e
                .call(
                    GET_DISTANCE_SQUARED,
                    &args![actor_location, target_location],
                )
                .f64();
            if setting_squared(e) > distance {
                1
            } else {
                2
            }
        } else if !e.call(STATE_CHECK_0097EEE0, &args![state_object]).bool() {
            3
        } else {
            let group = e.mem.u32(at + 0x80);
            if e.call(GROUP_CHECK_0097EF30, &args![group]).bool() {
                4
            } else {
                0
            }
        };
        if state == e.mem.u32(at + 0x144) {
            return true;
        }
        e.call(CLEAR_00980250, &args![at]);
        let state_object = e.mem.u32(at + 0x9c);
        let flag_was_set = e.call(STATE_FLAG_00662A80, &args![state_object]).bool();
        e.call(STATE_SET_0097CE70, &args![state_object, 0u32]);
        let mut state = state;
        match state {
            1 => {
                let procedure = new_object(e, 0x20, |e, block| {
                    e.call(PROCEDURE_009CBAD0, &args![block]).u32()
                });
                e.call(SET_ACTION_PROCEDURE, &args![at, procedure]);
            }
            2 => {
                let location = ebp - 0x7c;
                e.call(LOCATION_INIT_0097EE20, &args![location]);
                let target = e.mem.u32(at + 0xc0);
                e.call(COMBAT_LOCATION_SET_REF, &args![location, target]);
                let address = e.call(LOCATION_ADDRESS_006815C0, &args![location]).u32();
                let state_object = e.mem.u32(at + 0x9c);
                if e.call(
                    COMBAT_STATE_CHECK_MOVEMENT,
                    &args![state_object, address, 1u32, 1u32],
                )
                .bool()
                {
                    let procedure = new_object(e, 0x7c, |e, block| {
                        let value = x87_quiet_bits(e.mem.u32(VALUE_011A5654));
                        e.call(PROCEDURE_009D69F0, &args![block, location, value, 0u32])
                            .u32()
                    });
                    e.call(SET_MOVEMENT_PROCEDURE, &args![at, procedure]);
                }
            }
            3 => {
                state = 0;
                let destination = ebp - 0x9c;
                let candidate = ebp - 0xac;
                e.call(LOCATION_INIT_0097EE20, &args![destination]);
                e.call(LOCATION_INIT_0097EDE0, &args![candidate]);
                let group = e.mem.u32(at + 0x80);
                let target = e.mem.u32(at + 0xc0);
                let state_object = e.mem.u32(at + 0x9c);
                if e.call(GROUP_LOCATION_009890E0, &args![group, target, candidate])
                    .bool()
                    && e.call(
                        COMBAT_STATE_CHECK_MOVEMENT,
                        &args![state_object, candidate, 1u32, 1u32],
                    )
                    .bool()
                {
                    let here = ebp - 0xbc;
                    let actor = e.mem.u32(at + 0xbc);
                    e.call(GET_WORLD_LOCATION, &args![actor, here]);
                    let distance = e.call(GET_DISTANCE_SQUARED, &args![here, candidate]).f64();
                    if distance > e.mem.f64(HUNDRED_DOUBLE) {
                        e.call(LOCATION_SET_0097EE60, &args![destination, candidate]);
                        let procedure = new_object(e, 0x7c, |e, block| {
                            let value = x87_quiet_bits(e.mem.u32(VALUE_011A5654));
                            e.call(
                                PROCEDURE_009D69F0,
                                &args![block, destination, value, 0xfu32],
                            )
                            .u32()
                        });
                        e.call(SET_MOVEMENT_PROCEDURE, &args![at, procedure]);
                        state = 3;
                    }
                }
            }
            4 => {
                let procedure = new_object(e, 0x40, |e, block| {
                    e.call(COMBAT_PROCEDURE_SEARCH, &args![block]).u32()
                });
                e.call(SET_MOVEMENT_PROCEDURE, &args![at, procedure]);
            }
            5 => {
                let state_object = e.mem.u32(at + 0x9c);
                if cleared && flag_was_set {
                    e.call(STATE_SET_0097CE70, &args![state_object, 2u32]);
                } else {
                    let distance = e
                        .call(
                            GET_DISTANCE_SQUARED,
                            &args![actor_location, target_location],
                        )
                        .f64();
                    if setting_squared(e) > distance {
                        let state_object = e.mem.u32(at + 0x9c);
                        e.call(STATE_SET_0097CE70, &args![state_object, 1u32]);
                        let location = ebp - 0x58;
                        e.call(LOCATION_INIT_0097EE20, &args![location]);
                        let state_object = e.mem.u32(at + 0x9c);
                        if e.call(
                            FIND_BEST_FLEE_LOCATION,
                            &args![state_object, 0u32, location, 0xcu32],
                        )
                        .bool()
                        {
                            let procedure = if e
                                .call(LOCATION_CHECK_0097BE60, &args![location, 1u32])
                                .bool()
                            {
                                new_object(e, 0x18, |e, block| {
                                    e.call(PROCEDURE_009D5AD0, &args![block, 0xcu32]).u32()
                                })
                            } else {
                                new_object(e, 0x7c, |e, block| {
                                    let value = x87_quiet_bits(e.mem.u32(VALUE_011A56B4));
                                    e.call(
                                        PROCEDURE_009D69F0,
                                        &args![block, location, value, 0xcu32],
                                    )
                                    .u32()
                                })
                            };
                            e.call(SET_MOVEMENT_PROCEDURE, &args![at, procedure]);
                        }
                    } else {
                        let state_object = e.mem.u32(at + 0x9c);
                        e.call(STATE_SET_0097CE70, &args![state_object, 2u32]);
                    }
                }
            }
            // State 0 (the jump table covers 1 to 5): nothing.
            _ => {}
        }
        e.mem.set_u32(at + 0x144, state);
        true
    });
    // Without a target the exe leaves before the tail.
    if finish {
        finish_update(e, at);
    }
}

/// The tail of `0097e4b0` after the movement state (from `0097eced`): each procedure's
/// vtable slot +4, then for a pathing actor the timer's value into +0x148 and
/// `00575770(01016970)`, and the timer restarted with 1.0.
fn finish_update(e: &mut Engine, at: u32) {
    for field in [0x88u32, 0x84] {
        if e.mem.u32(at + field) != 0 {
            let procedure = e.mem.u32(at + field);
            e.vcall(procedure, 0x4, &[]);
        }
    }
    let actor = e.mem.u32(at + 0xbc);
    if e.call(ACTOR_IS_PATHING, &args![actor]).bool() {
        let value = e.call(TIMER_VALUE_00435E00, &args![at + 0xb4]).f64();
        e.mem.set_u32(at + 0x148, x87_store32(value));
        let actor = e.mem.u32(at + 0xbc);
        let value = x87_quiet_bits(e.mem.u32(VALUE_01016970));
        e.call(ACTOR_SET_00575770, &args![actor, value]);
    }
    e.call(TIMER_RESET_008D7F40, &args![at + 0xb4, 1.0f32.to_bits()]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0097_e490, fn_0097e490(Ptr) -> u32),
        entry!(0x0097_e4b0, fn_0097e4b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = [(u32, Vec<u32>)];

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            INITIALIZE_0097D990,
            TIMER_RESET_008D7F40,
            TIMER_DONE_008D7F80,
            SET_TARGET_00980830,
            CHECK_00980470,
            CHECK_009804D0,
            PROCEDURE_CHECK_0097E310,
            PROCEDURE_CALL_0097F270,
            CLEAR_00980250,
            GROUP_TARGET_00986C60,
            STATE_UPDATE_00997CB0,
            GET_WORLD_LOCATION,
            STATE_CHECK_009A1380,
            STATE_CHECK_0097EE90,
            STATE_CHECK_0097EEE0,
            GROUP_CHECK_0097EF30,
            STATE_FLAG_00662A80,
            STATE_SET_0097CE70,
            LOCATION_INIT_0097EE20,
            COMBAT_LOCATION_SET_REF,
            LOCATION_INIT_0097EDE0,
            LOCATION_SET_0097EE60,
            COMBAT_STATE_CHECK_MOVEMENT,
            GROUP_LOCATION_009890E0,
            FIND_BEST_FLEE_LOCATION,
            LOCATION_CHECK_0097BE60,
            ACTOR_IS_PATHING,
            ACTOR_SET_00575770,
            SET_ACTION_PROCEDURE,
            SET_MOVEMENT_PROCEDURE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for address in [
            PROCEDURE_009CBAD0,
            PROCEDURE_009D69F0,
            PROCEDURE_009D5AD0,
            COMBAT_PROCEDURE_SEARCH,
        ] {
            e.register(address, |_, a| a[0].into_ret());
        }
        e.register(LOCATION_ADDRESS_006815C0, |_, a| (a[0] + 4).into_ret());
        e.register(FLOAT_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(BYTE_SETTING_00408D60, |_, a| (a[0] + 4).into_ret());
        e.register(REFERENCE_00567D90, |_, a| a[0].into_ret());
        e.register(GET_DISTANCE_SQUARED, |_, _| Ret {
            st0: 50.0,
            ..Ret::default()
        });
        e.register(TIMER_VALUE_00435E00, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_6000, 0x2000);
        e.mem.set_f64(ZERO_DOUBLE, 0.0);
        e.mem.set_f64(HUNDRED_DOUBLE, 100.0);
        e.mem.set_f32(VALUE_01016970, 3.0);
        e.map(0x011a_5000, 0x1000);
        e.mem.set_f32(VALUE_011A5654, 64.0);
        e.mem.set_f32(VALUE_011A56B4, 10.0);
        e.map(0x011c_e000, 0x1000);
        e.mem.set_f32(SETTING_011CE820 + 4, 10.0);
        e.map(0x011f_1000, 0x1000);
        e
    }

    /// A controller already initialized, with a group, a combat state, an actor and a
    /// target, and no procedures.
    fn controller(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x150);
        e.mem.set_u8(this + 0xe4, 1);
        e.mem.set_u32(this + 0x80, 0x8080);
        e.mem.set_u32(this + 0x9c, 0x9c9c);
        e.mem.set_u32(this + 0xbc, 0xbcbc);
        e.mem.set_u32(this + 0xc0, 0xc0c0);
        this
    }

    #[test]
    fn fn_0097e490_compares_the_two_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 4, 5);
        e.mem.set_u32(this + 8, 5);
        assert_eq!(e.call(0x0097_e490, &args![this]).u32(), 1);
        e.mem.set_u32(this + 8, 4);
        assert_eq!(e.call(0x0097_e490, &args![this]).u32(), 0);
    }

    #[test]
    fn first_run_and_a_negative_timer_value_before_the_timer_is_done() {
        let mut e = engine();
        let this = controller(&mut e);
        e.mem.set_u8(this + 0xe4, 0);
        e.mem.set_f32(this + 0x148, -1.0);
        e.mem.set_u8(SETTING_011F1840 + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0097_e4b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, INITIALIZE_0097D990), vec![vec![this]]);
        assert_eq!(e.mem.u8(this + 0xe4), 1);
        assert_eq!(
            calls_to(&log, TIMER_RESET_008D7F40),
            vec![vec![this + 0xb4, bits(1.0)]]
        );
        assert_eq!(e.mem.f32(this + 0x148), 0.0);
        assert_eq!(calls_to(&log, SET_TARGET_00980830), vec![vec![this, 0]]);
        assert!(calls_to(&log, CHECK_00980470).is_empty());
    }

    #[test]
    fn close_target_starts_the_action_procedure_and_runs_the_tail() {
        let mut e = engine();
        answer(&mut e, TIMER_DONE_008D7F80, 1);
        answer(&mut e, STATE_CHECK_0097EE90, 1);
        answer(&mut e, ACTOR_IS_PATHING, 1);
        let this = controller(&mut e);
        let procedure = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(procedure, vtable);
        e.mem.set_u32(vtable + 4, vtable + 4);
        e.register(vtable + 4, |_, _| Ret::default());
        e.mem.set_u32(this + 0x84, procedure);
        e.call_log = Some(vec![]);
        e.call(0x0097_e4b0, &args![this]);
        let log = e.call_log.take().unwrap();
        let locations = calls_to(&log, GET_WORLD_LOCATION);
        assert_eq!((locations[0][0], locations[1][0]), (0xbcbc, 0xc0c0));
        assert_eq!(
            calls_to(&log, GET_DISTANCE_SQUARED),
            vec![vec![locations[0][1], locations[1][1]]]
        );
        let action = calls_to(&log, SET_ACTION_PROCEDURE);
        assert_eq!(action.len(), 1);
        assert_eq!(calls_to(&log, PROCEDURE_009CBAD0), vec![vec![action[0][1]]]);
        assert_eq!(calls_to(&log, STATE_SET_0097CE70), vec![vec![0x9c9c, 0]]);
        assert_eq!(e.mem.u32(this + 0x144), 1);
        assert_eq!(calls_to(&log, vtable + 4), vec![vec![procedure]]);
        assert_eq!(e.mem.f32(this + 0x148), 2.5);
        assert_eq!(
            calls_to(&log, ACTOR_SET_00575770),
            vec![vec![0xbcbc, bits(3.0)]]
        );
        assert_eq!(
            calls_to(&log, TIMER_RESET_008D7F40),
            vec![vec![this + 0xb4, bits(1.0)]]
        );
        // The same state again: only the tail.
        e.call_log = Some(vec![]);
        e.call(0x0097_e4b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, CLEAR_00980250).is_empty());
        assert_eq!(calls_to(&log, TIMER_RESET_008D7F40).len(), 1);
    }

    #[test]
    fn without_a_target_the_update_leaves_before_the_tail() {
        let mut e = engine();
        answer(&mut e, TIMER_DONE_008D7F80, 1);
        let this = controller(&mut e);
        e.mem.set_u32(this + 0xc0, 0);
        e.mem.set_u32(this + 0x144, 4);
        e.call_log = Some(vec![]);
        e.call(0x0097_e4b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, GROUP_TARGET_00986C60),
            vec![vec![0x8080, 0xbcbc]]
        );
        assert_eq!(calls_to(&log, SET_TARGET_00980830), vec![vec![this, 0]]);
        assert_eq!(e.mem.u32(this + 0x144), 0);
        assert!(calls_to(&log, STATE_UPDATE_00997CB0).is_empty());
        assert!(calls_to(&log, TIMER_RESET_008D7F40).is_empty());
    }

    #[test]
    fn a_cleared_state_with_its_flag_set_keeps_flag_2() {
        let mut e = engine();
        answer(&mut e, TIMER_DONE_008D7F80, 1);
        answer(&mut e, CHECK_009804D0, 1);
        answer(&mut e, STATE_CHECK_009A1380, 1);
        answer(&mut e, STATE_FLAG_00662A80, 1);
        let this = controller(&mut e);
        e.mem.set_u32(this + 0x144, 2);
        e.call_log = Some(vec![]);
        e.call(0x0097_e4b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, STATE_SET_0097CE70),
            vec![vec![0x9c9c, 0], vec![0x9c9c, 2]]
        );
        assert_eq!(e.mem.u32(this + 0x144), 5);
        assert!(calls_to(&log, FIND_BEST_FLEE_LOCATION).is_empty());
    }

    #[test]
    fn a_close_threat_flees() {
        let mut e = engine();
        answer(&mut e, TIMER_DONE_008D7F80, 1);
        answer(&mut e, STATE_CHECK_009A1380, 1);
        answer(&mut e, FIND_BEST_FLEE_LOCATION, 1);
        answer(&mut e, LOCATION_CHECK_0097BE60, 1);
        let this = controller(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0097_e4b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, STATE_SET_0097CE70),
            vec![vec![0x9c9c, 0], vec![0x9c9c, 1]]
        );
        let flee = calls_to(&log, FIND_BEST_FLEE_LOCATION);
        assert_eq!((flee[0][0], flee[0][1], flee[0][3]), (0x9c9c, 0, 0xc));
        let made = calls_to(&log, PROCEDURE_009D5AD0);
        assert_eq!(made[0][1], 0xc);
        assert_eq!(
            calls_to(&log, SET_MOVEMENT_PROCEDURE),
            vec![vec![this, made[0][0]]]
        );
        // The flee location check fails: the other procedure, with 10.0.
        answer(&mut e, LOCATION_CHECK_0097BE60, 0);
        e.mem.set_u32(this + 0x144, 0);
        e.call_log = Some(vec![]);
        e.call(0x0097_e4b0, &args![this]);
        let log = e.call_log.take().unwrap();
        let made = calls_to(&log, PROCEDURE_009D69F0);
        assert_eq!(made[0][1..], [flee[0][2], bits(10.0), 0xc]);
        // Far away: flag 2 only.
        e.mem.set_f32(SETTING_011CE820 + 4, 5.0);
        e.mem.set_u32(this + 0x144, 0);
        e.call_log = Some(vec![]);
        e.call(0x0097_e4b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, STATE_SET_0097CE70),
            vec![vec![0x9c9c, 0], vec![0x9c9c, 2]]
        );
    }

    #[test]
    fn a_far_target_is_approached_when_the_movement_is_allowed() {
        let mut e = engine();
        answer(&mut e, TIMER_DONE_008D7F80, 1);
        answer(&mut e, STATE_CHECK_0097EE90, 1);
        answer(&mut e, COMBAT_STATE_CHECK_MOVEMENT, 1);
        e.mem.set_f32(SETTING_011CE820 + 4, 5.0);
        let this = controller(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0097_e4b0, &args![this]);
        let log = e.call_log.take().unwrap();
        let location = calls_to(&log, LOCATION_INIT_0097EE20)[0][0];
        assert_eq!(
            calls_to(&log, COMBAT_LOCATION_SET_REF),
            vec![vec![location, 0xc0c0]]
        );
        assert_eq!(
            calls_to(&log, COMBAT_STATE_CHECK_MOVEMENT),
            vec![vec![0x9c9c, location + 4, 1, 1]]
        );
        let made = calls_to(&log, PROCEDURE_009D69F0);
        assert_eq!(made[0][1..], [location, bits(64.0), 0]);
        assert_eq!(e.mem.u32(this + 0x144), 2);
    }

    #[test]
    fn an_unseen_target_is_searched_for_from_a_group_location() {
        let mut e = engine();
        answer(&mut e, TIMER_DONE_008D7F80, 1);
        answer(&mut e, GROUP_LOCATION_009890E0, 1);
        answer(&mut e, COMBAT_STATE_CHECK_MOVEMENT, 1);
        e.register(GET_DISTANCE_SQUARED, |_, _| Ret {
            st0: 200.0,
            ..Ret::default()
        });
        let this = controller(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0097_e4b0, &args![this]);
        let log = e.call_log.take().unwrap();
        let candidate = calls_to(&log, LOCATION_INIT_0097EDE0)[0][0];
        let destination = calls_to(&log, LOCATION_INIT_0097EE20)[0][0];
        assert_eq!(
            calls_to(&log, GROUP_LOCATION_009890E0),
            vec![vec![0x8080, 0xc0c0, candidate]]
        );
        assert_eq!(
            calls_to(&log, LOCATION_SET_0097EE60),
            vec![vec![destination, candidate]]
        );
        let made = calls_to(&log, PROCEDURE_009D69F0);
        assert_eq!(made[0][1..], [destination, bits(64.0), 0xf]);
        assert_eq!(e.mem.u32(this + 0x144), 3);
        // Not beyond 100: state 0.
        e.register(GET_DISTANCE_SQUARED, |_, _| Ret {
            st0: 100.0,
            ..Ret::default()
        });
        e.mem.set_u32(this + 0x144, 7);
        e.call(0x0097_e4b0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x144), 0);
        // A group search instead.
        answer(&mut e, STATE_CHECK_0097EEE0, 1);
        answer(&mut e, GROUP_CHECK_0097EF30, 1);
        e.call_log = Some(vec![]);
        e.call(0x0097_e4b0, &args![this]);
        let log = e.call_log.take().unwrap();
        let made = calls_to(&log, COMBAT_PROCEDURE_SEARCH);
        assert_eq!(
            calls_to(&log, SET_MOVEMENT_PROCEDURE),
            vec![vec![this, made[0][0]]]
        );
        assert_eq!(e.mem.u32(this + 0x144), 4);
    }
}
