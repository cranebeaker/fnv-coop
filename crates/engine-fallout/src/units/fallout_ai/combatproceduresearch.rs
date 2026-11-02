//! `fallout/ai/combat/combatproceduresearch.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d8c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01091db4` (RTTI `.?AVCombatProcedureSearch@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009d8c40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_1db4);
    e.call(0x009c_a6c0, &args![this]);
}

// ---- Batch n025 ---------------------------------------------------------------------------

/// `CombatProcedureSearch` (Xbox PDB), 0x40 bytes: the `CombatProcedure` base (vtable,
/// `pCombatController` +0x4, `eStatus` +0x8, `pFailureReason` +0xc), then `eState` +0x10,
/// `CheckDestinationTimer` +0x14, `LookingTimer` +0x1c, `CheckSearchDoorTimer` +0x24 (8-byte
/// `CombatTimer`s), `InvestigateLocation` +0x2c (a `BGSWorldLocation`) and `pSearchDoor`
/// +0x3c; the PC offsets are the PDB's (constructor `009d8b60`).
const SEARCH_STATE: u32 = 0x10;
const SEARCH_CHECK_DESTINATION_TIMER: u32 = 0x14;
const SEARCH_LOOKING_TIMER: u32 = 0x1c;
const SEARCH_CHECK_DOOR_TIMER: u32 = 0x24;
const SEARCH_INVESTIGATE_LOCATION: u32 = 0x2c;
const SEARCH_DOOR: u32 = 0x3c;
/// The `CombatProcedureSearch` vtable (RTTI `.?AVCombatProcedureSearch@@`).
const SEARCH_VTABLE: u32 = 0x0109_1db4;
/// The `CombatProcedure` base constructor (`009ca680`, `thiscall`).
const PROCEDURE_CONSTRUCT: u32 = 0x009c_a680;
/// `CombatTimer` constructor (`00906f60`, `thiscall`).
const TIMER_CONSTRUCT: u32 = 0x0090_6f60;
/// `CombatProcedure::GetAttacker` (Xbox PDB, `00996de0`).
const GET_ATTACKER: u32 = 0x0099_6de0;
/// The controller's combat group (`004fb070`, `CombatController` +0x80).
const CONTROLLER_GROUP: u32 = 0x004f_b070;
/// `CombatProcedure::SaveGame` / `LoadGame` (Xbox PDB, `00996ed0` / `00996ef0`).
const PROCEDURE_SAVE_GAME: u32 = 0x0099_6ed0;
const PROCEDURE_LOAD_GAME: u32 = 0x0099_6ef0;
/// `CombatTimer::SaveGame` / `LoadGame` (Xbox PDB, `009a5eb0` / `009a5f20`).
const TIMER_SAVE_GAME: u32 = 0x009a_5eb0;
const TIMER_LOAD_GAME: u32 = 0x009a_5f20;
/// `TESForm`'s and `TESObjectREFR`'s RTTI type descriptors.
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TES_OBJECT_REFR_TYPE: u32 = 0x0118_41cc;

// Translated from 009d8b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d8b60 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `CombatProcedureSearch::CombatProcedureSearch` (Xbox PDB; `thiscall`): the base
/// constructor (`009ca680`), the vtable `01091db4`, state 0, the three timers (`00906f60`),
/// the investigate location (`0097ede0`) and no search door. Returns `this`.
pub fn combat_procedure_search_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.call(PROCEDURE_CONSTRUCT, &args![this]);
    e.mem.set_u32(me, SEARCH_VTABLE);
    e.mem.set_u32(me + SEARCH_STATE, 0);
    e.call(TIMER_CONSTRUCT, &args![me + SEARCH_CHECK_DESTINATION_TIMER]);
    e.call(TIMER_CONSTRUCT, &args![me + SEARCH_LOOKING_TIMER]);
    e.call(TIMER_CONSTRUCT, &args![me + SEARCH_CHECK_DOOR_TIMER]);
    e.call(0x0097_ede0, &args![me + SEARCH_INVESTIGATE_LOCATION]);
    e.mem.set_u32(me + SEARCH_DOOR, 0);
    this
}

// Translated from 009d8c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatProcedureSearch` type (`thiscall`, `this` unread): 10.
pub fn fn_009d8c00(_e: &mut Engine, _this: Ptr) -> u32 {
    10
}

// Translated from 009d8c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatProcedureSearch` scalar deleting destructor (`thiscall`, `RET 4`): the
/// destructor `009d8c40`, then `operator delete` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_009d8c10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009d8c40(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 009d8c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureSearch::Shutdown` (Xbox PDB; `thiscall`): gives up the group's searcher
/// slot when the attacker holds it (`009d8d20`, `009d8d00` with 0), stops the attacker
/// (`Actor::StopMoving`) and clears the move mode (`00996e40`); in states 4 and 5 the
/// attacker fades in (`Actor::FadeIn`), and in state 2 the search door is unreserved in the
/// group (`CombatGroup::UnreserveSearchDoor`).
pub fn combat_procedure_search_shutdown(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let controller = e.mem.u32(me + 4);
    let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
    if group != 0 && fn_009d8d20(e, Ptr::new(group)) == attacker {
        fn_009d8d00(e, Ptr::new(group), 0);
    }
    e.call(0x008b_3ab0, &args![attacker]);
    e.call(0x0099_6e40, &args![this]);
    let state = e.mem.u32(me + SEARCH_STATE);
    if state == 5 || e.mem.u32(me + SEARCH_STATE) == 4 {
        e.call(0x008b_cd20, &args![attacker]);
    }
    if e.mem.u32(me + SEARCH_STATE) == 2 && group != 0 && e.mem.u32(me + SEARCH_DOOR) != 0 {
        let door = e.mem.u32(me + SEARCH_DOOR);
        e.call(0x0098_c780, &args![group, door]);
    }
}

// Translated from 009d8d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores `value` at +0xec of `this` (the
/// combat group the search procedure passes; the field is not confirmed).
pub fn fn_009d8d00(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xec, value);
}

// Translated from 009d8d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the word at +0xec of `this` (see `009d8d00`).
pub fn fn_009d8d20(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xec)
}

// Translated from 009d9f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): copies the four words at +0xf0 of
/// `this` (a `BGSWorldLocation`-sized block; the class is not confirmed) to `out` and
/// returns `out`.
pub fn fn_009d9f60(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let from = this.addr() + 0xf0;
    for k in 0..4u32 {
        let word = e.mem.u32(from + 4 * k);
        e.mem.set_u32(out.addr() + 4 * k, word);
    }
    out
}

// Translated from 009d9fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureSearch::ActivatedReference` (Xbox PDB; `thiscall`, `RET 0x10`): forgets
/// the search door when it is `reference`. The other three words are not read.
pub fn combat_procedure_search_activated_reference(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
    let me = this.addr();
    if e.mem.u32(me + SEARCH_DOOR) == reference {
        e.mem.set_u32(me + SEARCH_DOOR, 0);
    }
}

// Translated from 009da550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the word at +0xe8 of `this` in
/// `out` and returns `out` (the class is not confirmed).
pub fn fn_009da550(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let word = e.mem.u32(this.addr() + 0xe8);
    e.mem.set_u32(out.addr(), word);
    out
}

// Translated from 009da570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the distance travelled
/// (`VirtualActorPathHandler::GetDistanceTraveled` `004a7bd0`) by the handler in the
/// `NiPointer` at +0xd4 of `this`, rounded to a float; -1 (`01012054`) without one.
pub fn fn_009da570(e: &mut Engine, this: Ptr) -> f32 {
    let pointer = this.addr() + 0xd4;
    let bits = if e.call(0x0055_9450, &args![pointer]).u32() != 0 {
        let handler = e.call(0x0055_9450, &args![pointer]).u32();
        x87_store32(e.call(0x004a_7bd0, &args![handler]).f64())
    } else {
        x87_quiet_bits(e.mem.u32(0x0101_2054))
    };
    f32::from_bits(x87_quiet_bits(bits))
}

// Translated from 009da5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureSearch::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): the procedure's own
/// save (`00996ed0`), the state (4 bytes), the investigate location (`00527b70`), the
/// timers in the order looking, check destination, check door (`009a5eb0`) and the search
/// door's form id (`00865df0`).
pub fn combat_procedure_search_save_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(PROCEDURE_SAVE_GAME, &args![this, buffer]);
    e.call(0x0086_5e50, &args![buffer, me + SEARCH_STATE, 4u32, 0u32]);
    e.call(
        0x0052_7b70,
        &args![me + SEARCH_INVESTIGATE_LOCATION, buffer],
    );
    e.call(TIMER_SAVE_GAME, &args![me + SEARCH_LOOKING_TIMER, buffer]);
    e.call(
        TIMER_SAVE_GAME,
        &args![me + SEARCH_CHECK_DESTINATION_TIMER, buffer],
    );
    e.call(
        TIMER_SAVE_GAME,
        &args![me + SEARCH_CHECK_DOOR_TIMER, buffer],
    );
    let door = e.mem.u32(me + SEARCH_DOOR);
    e.call(0x0086_5df0, &args![buffer, door, 0u32]);
}

// Translated from 009da640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureSearch::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): reads what `009da5c0`
/// saved; the search door stays a form id until `009da6c0`.
pub fn combat_procedure_search_load_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(PROCEDURE_LOAD_GAME, &args![this, buffer]);
    e.call(0x0086_4980, &args![buffer, me + SEARCH_STATE, 4u32]);
    e.call(
        0x0052_7ba0,
        &args![me + SEARCH_INVESTIGATE_LOCATION, buffer],
    );
    e.call(TIMER_LOAD_GAME, &args![me + SEARCH_LOOKING_TIMER, buffer]);
    e.call(
        TIMER_LOAD_GAME,
        &args![me + SEARCH_CHECK_DESTINATION_TIMER, buffer],
    );
    e.call(
        TIMER_LOAD_GAME,
        &args![me + SEARCH_CHECK_DOOR_TIMER, buffer],
    );
    e.call(0x0086_48e0, &args![buffer, me + SEARCH_DOOR]);
}

// Translated from 009da6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureSearch::InitLoadGame` (Xbox PDB; `thiscall`, `RET 4`): the base's
/// `004534f0(this, buffer)`, then the search door's form id becomes its `TESObjectREFR`
/// (`004839c0`, cast `00ec43fb`; 0 stays 0).
pub fn combat_procedure_search_init_load_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(0x0045_34f0, &args![this, buffer]);
    let door = e.mem.u32(me + SEARCH_DOOR);
    let resolved = if door != 0 {
        let form = e.call(0x0048_39c0, &args![door]).u32();
        e.call(
            0x00ec_43fb,
            &args![form, 0u32, TES_FORM_TYPE, TES_OBJECT_REFR_TYPE, 0u32],
        )
        .u32()
    } else {
        0
    };
    e.mem.set_u32(me + SEARCH_DOOR, resolved);
}

/// The state names and formats `009d9fd0` prints (`.rdata`).
const INITIALIZING_TEXT: u32 = 0x0109_0b68;
const INVESTIGATING_TEXT: u32 = 0x0109_2078;
const POINT_FORMAT: u32 = 0x0109_205c;
const SPACE_FORMAT: u32 = 0x0108_cfe4;
const SEARCHING_DOOR_TEXT: u32 = 0x0109_204c;
const DOOR_FORMAT: u32 = 0x0109_202c;
const SEARCHING_TEXT: u32 = 0x0109_2020;
const LOOKING_DOOR_FADE_TEXT: u32 = 0x0109_200c;
const LOOKING_TIMER_FORMAT: u32 = 0x0109_1ff0;
const LOOKING_DOOR_TEXT: u32 = 0x0109_1fe0;
const LOOKING_TEXT: u32 = 0x0109_1fd8;
const LOOKING_AROUND_NEW_SPACE_TEXT: u32 = 0x0109_1fb0;
const SEARCH_AREA_TIMER_FORMAT: u32 = 0x0109_1f88;
const MOVING_TO_SEARCH_CENTER_TEXT: u32 = 0x0109_1f60;
const SEARCH_SUMMARY_FORMAT: u32 = 0x0109_1f40;

/// The three floats of the point `006815c0` returns for `location`, each widened to a
/// `double` argument (`FLD`, `FSTP qword`), in the order x, y, z (each through its own call,
/// z first as the exe pushes them).
fn point_doubles(e: &mut Engine, location: u32) -> [f64; 3] {
    let point = e.call(0x0068_15c0, &args![location]).u32();
    let z = x87_load32(e.mem.u32(point + 8));
    let point = e.call(0x0068_15c0, &args![location]).u32();
    let y = x87_load32(e.mem.u32(point + 4));
    let point = e.call(0x0068_15c0, &args![location]).u32();
    let x = x87_load32(e.mem.u32(point));
    [x, y, z]
}

/// `"Looking Timer: %.02f/%.02f"` with the looking timer's elapsed and total time.
fn print_looking_timer(e: &mut Engine, this: Ptr, printer: u32, first: u32, second: u32) {
    let timer = this.addr() + SEARCH_LOOKING_TIMER;
    let total = e.call(0x006b_9130, &args![timer]).f64();
    let elapsed = e.call(0x0043_5e00, &args![timer]).f64();
    e.vcall(
        printer,
        0x14,
        &args![first, second, LOOKING_TIMER_FORMAT, elapsed, total],
    );
}

/// The investigate location's point and the name (slot `+0x130`) of its space
/// (`0084e3a0`).
fn print_investigate_location(e: &mut Engine, this: Ptr, printer: u32, first: u32, second: u32) {
    let location = this.addr() + SEARCH_INVESTIGATE_LOCATION;
    let [x, y, z] = point_doubles(e, location);
    e.vcall(printer, 0x14, &args![first, second, POINT_FORMAT, x, y, z]);
    let space = e.call(0x0084_e3a0, &args![location]).u32();
    let name = e.vcall(space, 0x130, &args![]).u32();
    e.vcall(printer, 0x14, &args![first, second, SPACE_FORMAT, name]);
}

// Translated from 009d9fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureSearch::DisplayDebugText` (Xbox PDB; `thiscall`, `RET 0xc`): prints
/// through `printer`'s slot `+0x14` (`cdecl`, with `first` and `second`) the state's name
/// (the exe's jump table at `009da520`, here a `match` on states 0 to 8) and its details:
/// the investigate location (states 1 and 8), the search door's name and position (state
/// 2), the looking timer (states 4 to 7) and, in state 7 when the looking timer started
/// after the group's search-area timer (`006a7f50` of `009d9f40`) and the group's search
/// location (`009d9f60`) lies in another space than the attacker (`00575ca0`), the
/// search-area timer. Then, when the group's distance travelled (`009da570`) is at least 0:
/// `"Search: %i %.2f %.1f%% %.2fs"` with the group's `00458400`, `009442a0`, that distance
/// times 100 and `0097fa90` of the group's `009da550` value.
pub fn combat_procedure_search_display_debug_text(
    e: &mut Engine,
    this: Ptr,
    printer: Ptr,
    first: u32,
    second: u32,
) {
    let me = this.addr();
    let p = printer.addr();
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let controller = e.mem.u32(me + 4);
    let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
    match e.mem.u32(me + SEARCH_STATE) {
        0 => {
            e.vcall(p, 0x14, &args![first, second, INITIALIZING_TEXT]);
        }
        1 => {
            e.vcall(p, 0x14, &args![first, second, INVESTIGATING_TEXT]);
            print_investigate_location(e, this, p, first, second);
        }
        2 => {
            e.vcall(p, 0x14, &args![first, second, SEARCHING_DOOR_TEXT]);
            if e.mem.u32(me + SEARCH_DOOR) != 0 {
                let door = e.mem.u32(me + SEARCH_DOOR);
                let position = e.vcall(door, 0x1f4, &args![]).u32();
                let z = x87_load32(e.mem.u32(position + 8));
                let door = e.mem.u32(me + SEARCH_DOOR);
                let position = e.vcall(door, 0x1f4, &args![]).u32();
                let y = x87_load32(e.mem.u32(position + 4));
                let door = e.mem.u32(me + SEARCH_DOOR);
                let position = e.vcall(door, 0x1f4, &args![]).u32();
                let x = x87_load32(e.mem.u32(position));
                let door = e.mem.u32(me + SEARCH_DOOR);
                let name = e.vcall(door, 0x130, &args![]).u32();
                e.vcall(p, 0x14, &args![first, second, DOOR_FORMAT, name, x, y, z]);
            }
        }
        3 => {
            e.vcall(p, 0x14, &args![first, second, SEARCHING_TEXT]);
        }
        4 => {
            e.vcall(p, 0x14, &args![first, second, LOOKING_DOOR_FADE_TEXT]);
            print_looking_timer(e, this, p, first, second);
        }
        5 => {
            e.vcall(p, 0x14, &args![first, second, LOOKING_DOOR_TEXT]);
            print_looking_timer(e, this, p, first, second);
        }
        6 => {
            e.vcall(p, 0x14, &args![first, second, LOOKING_TEXT]);
            print_looking_timer(e, this, p, first, second);
        }
        7 => {
            e.vcall(
                p,
                0x14,
                &args![first, second, LOOKING_AROUND_NEW_SPACE_TEXT],
            );
            print_looking_timer(e, this, p, first, second);
            let timer = e.call(0x009d_9f40, &args![group]).u32();
            let area_start = e.call(0x006a_7f50, &args![timer]).f64();
            let looking_start = e.call(0x006a_7f50, &args![me + SEARCH_LOOKING_TIMER]).f64();
            if looking_start > area_start {
                let space = e.with_stack(0x10, |e, location| {
                    let location = fn_009d9f60(e, Ptr::new(group), location).addr();
                    e.call(0x0084_e3a0, &args![location]).u32()
                });
                let attacker_space = e.call(0x0057_5ca0, &args![attacker]).u32();
                if space != attacker_space {
                    let timer = e.call(0x009d_9f40, &args![group]).u32();
                    let total = e.call(0x006b_9130, &args![timer]).f64();
                    let timer = e.call(0x009d_9f40, &args![group]).u32();
                    let elapsed = e.call(0x0043_5e00, &args![timer]).f64();
                    e.vcall(
                        p,
                        0x14,
                        &args![first, second, SEARCH_AREA_TIMER_FORMAT, elapsed, total],
                    );
                }
            }
        }
        8 => {
            e.vcall(p, 0x14, &args![first, second, MOVING_TO_SEARCH_CENTER_TEXT]);
            print_investigate_location(e, this, p, first, second);
        }
        _ => {}
    }
    let travelled = fn_009da570(e, Ptr::new(group)).to_bits();
    let zero: f64 = e.global(0x0101_2060);
    if x87_load32(travelled) >= zero {
        let total = e.with_stack(4, |e, out| {
            let value = fn_009da550(e, Ptr::new(group), out).addr();
            e.call(0x0097_fa90, &args![value]).f64()
        });
        let hundred: f64 = e.global(0x0101_7a40);
        let percent = x87_mul(x87_load32(travelled), hundred);
        let area = e.call(0x0094_42a0, &args![group]).f64();
        let count = e.call(0x0045_8400, &args![group]).u32();
        e.vcall(
            p,
            0x14,
            &args![
                first,
                second,
                SEARCH_SUMMARY_FORMAT,
                count,
                area,
                percent,
                total
            ],
        );
    }
}

/// Stack frame of `009d8d40` (from `EBP-0x5e8`).
const SEARCH_UPDATE_FRAME: u32 = 0x5e8;
/// `CombatTimer`: start with a delay (`008d7f40`, `thiscall(delay)`) and the delay has
/// passed (`008d7f80`).
const TIMER_START: u32 = 0x008d_7f40;
const TIMER_EXPIRED: u32 = 0x008d_7f80;
/// Fails the procedure with a reason (`009caac0`, `thiscall(text)`).
const PROCEDURE_FAIL: u32 = 0x009c_aac0;
/// `PathingRequest` constructor (`006e2420`) and destructor (`006e2620`).
const PATHING_REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const PATHING_REQUEST_DESTRUCT: u32 = 0x006e_2620;
/// `Actor::BuildRequest` (`008b38d0`, `thiscall(request, point, space, radius, 0)`) and
/// `Actor::SetPathfindingGoal` (`008b3630`, `thiscall(request)`).
const BUILD_REQUEST: u32 = 0x008b_38d0;
const SET_PATHFINDING_GOAL: u32 = 0x008b_3630;
/// Path state of the attacker: failed (`008b3b90`), valid (`Actor::IsPathValid`
/// `008b3960`), complete (`Actor::IsPathingComplete` `008b3bb0`).
const PATH_FAILED: u32 = 0x008b_3b90;
const PATH_VALID: u32 = 0x008b_3960;
const PATH_COMPLETE: u32 = 0x008b_3bb0;
/// `CombatState::AddUnreachableLocation` (Xbox PDB, `009a0ec0`, unit `combatstate.cpp`).
const STATE_ADD_UNREACHABLE_LOCATION: u32 = 0x009a_0ec0;
/// `128.0` (`float`) and `512.0` (`float`), `2.0` (`float`), `3.0` (`float`).
const ONE_HUNDRED_TWENTY_EIGHT: u32 = 0x0101_e704;
const FIVE_HUNDRED_TWELVE: u32 = 0x0102_31a0;
const TWO_FLOAT: u32 = 0x0101_62c0;
const THREE_FLOAT: u32 = 0x0101_7718;
/// `30.0`, `3.1415927410125732` (the float nearest pi), `1.0` and `0.0` (`double`).
const THIRTY_DOUBLE: u32 = 0x0101_db88;
const PI_DOUBLE: u32 = 0x0101_ff40;
const ONE_DOUBLE: u32 = 0x0101_2070;
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `16384.0` (`double`, 128 squared).
const NEAR_CENTER_DOUBLE: u32 = 0x0106_cd60;
/// The failure texts of `009d8d40`.
const NO_PATH_TO_INVESTIGATE: u32 = 0x0109_1f18;
const NO_PATH_TO_DOOR: u32 = 0x0109_1ef4;
const NO_PATH_TO_CENTER: u32 = 0x0109_1ed0;
const NO_LOS_PATH: u32 = 0x0109_1eb0;
const MOVING_FAILED: u32 = 0x0109_1c54;
const BUILDING_FAILED: u32 = 0x0109_1c30;
const MOVING_TO_DOOR_FAILED: u32 = 0x0109_1e84;
const BUILDING_TO_DOOR_FAILED: u32 = 0x0109_1e60;
const NO_PATH_THROUGH_DOOR: u32 = 0x0109_1e38;
const SEARCHING_FAILED: u32 = 0x0109_1e18;
const BUILDING_SEARCH_FAILED: u32 = 0x0109_1df8;

/// What `009d8d40` reads once at its start.
struct SearchUpdate {
    this: Ptr,
    attacker: u32,
    process_type: u32,
    group: u32,
    grid: u32,
    state: u32,
    frame: Ptr,
}

impl SearchUpdate {
    fn local(&self, k: u32) -> u32 {
        self.frame.addr() + SEARCH_UPDATE_FRAME - k
    }

    fn field(&self, off: u32) -> u32 {
        self.this.addr() + off
    }

    /// Starts the timer at `off` with `delay` (bits).
    fn start(&self, e: &mut Engine, off: u32, delay: u32) {
        e.call(TIMER_START, &args![self.field(off), delay]);
    }

    /// Starts the timer at `off` with a setting's value.
    fn start_with_setting(&self, e: &mut Engine, off: u32, setting: u32) {
        let value = e.call(0x0040_3e20, &args![setting]).u32();
        let delay = x87_quiet_bits(e.mem.u32(value));
        self.start(e, off, delay);
    }

    fn set_state(&self, e: &mut Engine, state: u32) {
        e.mem.set_u32(self.field(SEARCH_STATE), state);
    }

    /// State 6 with the looking timer started for 1.
    fn look_around(&self, e: &mut Engine) {
        self.set_state(e, 6);
        self.start(e, SEARCH_LOOKING_TIMER, 1.0f32.to_bits());
    }

    fn fail(&self, e: &mut Engine, text: u32) {
        e.call(PROCEDURE_FAIL, &args![self.this, text]);
    }

    /// A pathing request (`006e2420`) in the frame at `k`, built for `point` in `space` with
    /// `radius` (`008b38d0`), and whether the attacker accepted it as its goal (`008b3630`).
    /// The request is left constructed.
    fn request_goal(
        &self,
        e: &mut Engine,
        k: u32,
        point: u32,
        space: u32,
        radius: u32,
        before_goal: impl FnOnce(&mut Engine, u32),
    ) -> bool {
        let request = self.local(k);
        e.call(
            BUILD_REQUEST,
            &args![self.attacker, request, point, space, radius, 0u32],
        );
        before_goal(e, request);
        e.call(SET_PATHFINDING_GOAL, &args![self.attacker, request])
            .bool()
    }
}

// Translated from 009d8d40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d8d40 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `CombatProcedureSearch::Update` (Xbox PDB; `thiscall`): the search state machine (the
/// exe's jump table at `009d9e98`, here a `match` on the state; 8 runs as 1, states above 8
/// do nothing).
/// - 0 (start): a low-process attacker whose controller's `009d9f00` is 4 just looks
///   around (state 7, looking timer 2). Otherwise, when the group may search (`0097ef30`):
///   to a search location more than 30 away (`CombatGroup::FindSearchLocation`,
///   `00985bb0`) as an investigation (state 1); else to a search door
///   (`CombatGroup::FindSearchDoor` within a setting by process type) facing away from its
///   teleport rotation (state 2, the door reserved with `0098c750`); else, when the group has
///   a line-of-sight grid (`009d9f20`) and its search location (`009d9f60`) is in the
///   attacker's space, as the group's searcher a line-of-sight search (`PathingRequestLOS`,
///   state 3); else to the search location (state 8) unless already within 128 of it (state
///   6). Each path that cannot be set up fails the procedure (`009caac0`); a started move
///   sets the move mode (`00996e20` with 1).
/// - 1 and 8 (moving): a failed path marks the destination unreachable (unless the path
///   was valid) and fails the procedure; while investigating, the investigation point
///   within `setting(011cf2a0)` and in sight ends the move early (`Actor::StopMoving`); at
///   the end the group is told (`0098bd70`) and the searcher looks around (state 6).
/// - 2 (to the door): a failed path marks the door unreachable (`0098c710`); at the door
///   the group is told (`0098c6e0`), a low-process attacker fades out (`008bcd60`, state
///   4), any other walks through it (state 2 again).
/// - 3 (line-of-sight search): a failed path blocks the failed destination in the grid
///   (`006e0bc0`); while searching, a door is tried again on the door timer; the searcher
///   stops when a seen grid point is near enough for its navmesh kind (settings `011cf3c0`,
///   `011ceb68`, `011cf2e0`), and looks around at the end.
/// - 4 to 7: fading out and turning to the door's rotation (state 5), fading in (state 0),
///   looking around (state 0 when the looking timer passes), and looking around in a new
///   space (state 0 once the group's search-area timer has run out in the same space).
pub fn combat_procedure_search_update(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    e.call(0x0099_6e00, &args![this]);
    let process_type = e.call(0x0093_1850, &args![attacker]).u32();
    let controller = e.mem.u32(me + 4);
    let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
    let grid = e.call(0x009d_9f20, &args![group]).u32();
    let controller = e.mem.u32(me + 4);
    let state = e.call(0x0058_6150, &args![controller]).u32();
    e.with_stack(SEARCH_UPDATE_FRAME, |e, frame| {
        let update = SearchUpdate {
            this,
            attacker,
            process_type,
            group,
            grid,
            state,
            frame,
        };
        match e.mem.u32(me + SEARCH_STATE) {
            0 => search_start(e, &update),
            1 | 8 => search_moving(e, &update),
            2 => search_to_door(e, &update),
            3 => search_line_of_sight(e, &update),
            4 => {
                let value = e.call(0x008b_cda0, &args![attacker]).f64();
                let zero: f64 = e.global(ZERO_DOUBLE);
                if value <= zero {
                    let door = e.mem.u32(me + SEARCH_DOOR);
                    if door != 0 {
                        let source = e.call(0x0056_9000, &args![door]).u32();
                        let rotation = update.local(0x560);
                        for k in 0..3u32 {
                            let word = e.mem.u32(source + 4 * k);
                            e.mem.set_u32(rotation + 4 * k, word);
                        }
                        let z = x87_quiet_bits(e.mem.u32(rotation + 8));
                        let heading = e.call(0x004b_1480, &args![z]).f64();
                        e.call(0x0057_57d0, &args![attacker, x87_store32(heading)]);
                    }
                    update.set_state(e, 5);
                }
            }
            5 => {
                if e.call(TIMER_EXPIRED, &args![me + SEARCH_LOOKING_TIMER])
                    .bool()
                {
                    let value = e.call(0x008b_cda0, &args![attacker]).f64();
                    let one: f64 = e.global(ONE_DOUBLE);
                    if value < one {
                        e.call(0x008b_cd20, &args![attacker]);
                    }
                    update.set_state(e, 0);
                }
            }
            6 => {
                if e.call(TIMER_EXPIRED, &args![me + SEARCH_LOOKING_TIMER])
                    .bool()
                {
                    update.set_state(e, 0);
                }
            }
            7 => {
                let timer = e.call(0x009d_9f40, &args![group]).u32();
                let area_start = e.call(0x006a_7f50, &args![timer]).f64();
                let looking_start = e.call(0x006a_7f50, &args![me + SEARCH_LOOKING_TIMER]).f64();
                if looking_start > area_start {
                    let location = fn_009d9f60(e, Ptr::new(group), Ptr::new(update.local(0x5b8)));
                    let space = e.call(0x0084_e3a0, &args![location]).u32();
                    if space != e.call(0x0057_5ca0, &args![attacker]).u32() {
                        return;
                    }
                }
                if e.call(TIMER_EXPIRED, &args![me + SEARCH_LOOKING_TIMER])
                    .bool()
                {
                    update.set_state(e, 0);
                }
            }
            _ => {}
        }
    });
}

/// State 0 of `009d8d40`.
fn search_start(e: &mut Engine, update: &SearchUpdate) {
    let this = update.this;
    let attacker = update.attacker;
    let group = update.group;
    if update.process_type == 0 {
        let controller = e.mem.u32(update.field(4));
        if e.call(0x009d_9f00, &args![controller]).u32() == 4 {
            update.set_state(e, 7);
            let delay = x87_quiet_bits(e.mem.u32(TWO_FLOAT));
            update.start(e, SEARCH_LOOKING_TIMER, delay);
            return;
        }
    }
    e.mem.set_u32(update.field(SEARCH_DOOR), 0);
    if !e.call(0x0097_ef30, &args![group]).bool() {
        return;
    }
    let location = e
        .call(0x0098_bc00, &args![group, attacker, 0u32, 0u32])
        .u32();
    let far_enough = location != 0 && {
        let distance = e.call(0x0098_5bb0, &args![location]).f64();
        let limit: f64 = e.global(THIRTY_DOUBLE);
        distance > limit
    };
    if far_enough {
        e.call(PATHING_REQUEST_CONSTRUCT, &args![update.local(0xdc)]);
        let radius = x87_quiet_bits(e.mem.u32(ONE_HUNDRED_TWENTY_EIGHT));
        let space = e.call(0x0084_e3a0, &args![location]).u32();
        let point = e.call(0x0068_15c0, &args![location]).u32();
        let accepted = update.request_goal(e, 0xdc, point, space, radius, |e, request| {
            e.call(0x008e_7c50, &args![request, 0u32]);
        });
        if !accepted {
            update.fail(e, NO_PATH_TO_INVESTIGATE);
            e.call(PATHING_REQUEST_DESTRUCT, &args![update.local(0xdc)]);
            return;
        }
        update.set_state(e, 1);
        update.start_with_setting(e, SEARCH_CHECK_DESTINATION_TIMER, 0x011c_e9c0);
        for k in 0..4u32 {
            let word = e.mem.u32(location + 4 * k);
            e.mem
                .set_u32(update.field(SEARCH_INVESTIGATE_LOCATION) + 4 * k, word);
        }
        e.call(PATHING_REQUEST_DESTRUCT, &args![update.local(0xdc)]);
        e.call(0x0099_6e20, &args![this, 1u32]);
        return;
    }
    let setting = if update.process_type == 0 {
        0x011c_f938
    } else {
        0x011c_ef5c
    };
    let value = e.call(0x0040_3e20, &args![setting]).u32();
    let range = x87_quiet_bits(e.mem.u32(value));
    let door = e
        .call(0x0098_c3d0, &args![group, attacker, range, 1u32])
        .u32();
    if door != 0 {
        e.call(PATHING_REQUEST_CONSTRUCT, &args![update.local(0x19c)]);
        let space = e.call(0x0057_5ca0, &args![door]).u32();
        let point = e.call(0x0056_8fa0, &args![door]).u32();
        let accepted = update.request_goal(e, 0x19c, point, space, 0, |e, request| {
            let source = e.call(0x0056_9000, &args![door]).u32();
            let rotation = update.local(0xec);
            for k in 0..3u32 {
                let word = e.mem.u32(source + 4 * k);
                e.mem.set_u32(rotation + 4 * k, word);
            }
            let pi: f64 = e.global(PI_DOUBLE);
            let back = x87_store32(x87_sub(x87_load32(e.mem.u32(rotation + 8)), pi));
            let heading = e.call(0x004b_1480, &args![x87_quiet_bits(back)]).f64();
            e.call(0x008e_7c20, &args![request, x87_store32(heading)]);
        });
        if !accepted {
            update.fail(e, NO_PATH_TO_DOOR);
            e.call(PATHING_REQUEST_DESTRUCT, &args![update.local(0x19c)]);
            return;
        }
        update.set_state(e, 2);
        e.mem.set_u32(update.field(SEARCH_DOOR), door);
        e.call(0x0098_c750, &args![group, door]);
        e.call(PATHING_REQUEST_DESTRUCT, &args![update.local(0x19c)]);
        e.call(0x0099_6e20, &args![this, 1u32]);
        return;
    }
    let center = fn_009d9f60(e, Ptr::new(group), Ptr::new(update.local(0x1ac))).addr();
    if update.grid != 0 {
        let space = e.call(0x0084_e3a0, &args![center]).u32();
        if space == e.call(0x0057_5ca0, &args![attacker]).u32() {
            search_with_line_of_sight(e, update, location);
            return;
        }
    }
    let own = update.local(0x1bc);
    e.call(GET_WORLD_LOCATION_FOR_SEARCH, &args![attacker, own]);
    let squared = e.call(0x0052_7a40, &args![own, center]).f64();
    let near: f64 = e.global(NEAR_CENTER_DOUBLE);
    if squared <= near || squared.is_nan() {
        update.look_around(e);
    } else {
        let request = update.local(0x26c);
        e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
        if e.call(0x009a_02a0, &args![update.state, center, 1u32, 0u32])
            .bool()
        {
            let radius = x87_quiet_bits(e.mem.u32(ONE_HUNDRED_TWENTY_EIGHT));
            let space = e.call(0x0084_e3a0, &args![center]).u32();
            let point = e.call(0x0068_15c0, &args![center]).u32();
            if !update.request_goal(e, 0x26c, point, space, radius, |_, _| {}) {
                update.fail(e, NO_PATH_TO_CENTER);
                e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                return;
            }
            for k in 0..4u32 {
                let word = e.mem.u32(center + 4 * k);
                e.mem
                    .set_u32(update.field(SEARCH_INVESTIGATE_LOCATION) + 4 * k, word);
            }
            update.set_state(e, 8);
        }
        e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
    }
    e.call(0x0099_6e20, &args![this, 1u32]);
}

/// `TESObjectREFR::GetWorldLocation` (Xbox PDB, `00575650`).
const GET_WORLD_LOCATION_FOR_SEARCH: u32 = 0x0057_5650;

/// The line-of-sight search of state 0 of `009d8d40` (`location` is the group's search
/// location from `CombatGroup::FindSearchLocation`, possibly 0).
fn search_with_line_of_sight(e: &mut Engine, update: &SearchUpdate, location: u32) {
    let this = update.this;
    let attacker = update.attacker;
    let group = update.group;
    let holder = fn_009d8d20(e, Ptr::new(group));
    if holder != 0 && holder != attacker {
        return;
    }
    let request = update.local(0x35c);
    e.call(0x006e_57e0, &args![request]);
    e.call(0x006e_29f0, &args![request, attacker]);
    if e.vcall(attacker, 0x21c, &args![]).bool() {
        let extra = e.call(0x0041_81e0, &args![attacker]).u32() + 0x30;
        let value = e.vcall(extra, 0x28, &args![]).u8();
        e.call(0x006e_2b50, &args![request, u32::from(value)]);
    }
    e.call(0x008d_f000, &args![request]);
    let pointer = update.local(0x564);
    e.call(0x0063_3c90, &args![pointer, update.grid]);
    e.call(0x009d_9ec0, &args![request, pointer]);
    e.call(0x0045_cec0, &args![pointer]);
    let target = e.call(0x0070_58c0, &args![update.state]).u32();
    if target != 0 && matches!(e.mem.u8(target), 1..=3) {
        let distance = e.call(0x009a_0b00, &args![update.state]).f64();
        let point = e.call(0x0068_15c0, &args![target + 8]).u32();
        let (x, y, z) = (e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8));
        e.call(0x006e_5a40, &args![request, x, y, z, x87_store32(distance)]);
    }
    if location != 0 {
        let space = e.call(0x0084_e3a0, &args![location]).u32();
        if space == e.call(0x0057_5ca0, &args![attacker]).u32() {
            let point = e.call(0x0068_15c0, &args![location]).u32();
            let goal = update.local(0x58c);
            let goal = e.call(0x006d_ce10, &args![goal, point, attacker]).u32();
            e.call(0x006d_1bc0, &args![request, goal]);
            e.call(0x004f_f7e0, &args![update.local(0x58c)]);
        }
    }
    let range = x87_quiet_bits(e.mem.u32(FIVE_HUNDRED_TWELVE));
    e.call(0x006e_4340, &args![request, range]);
    e.call(0x008a_d8f0, &args![request, 0u32]);
    let ignored = update.local(0x27c);
    e.call(0x0049_f1c0, &args![ignored]);
    if e.call(0x0098_c7b0, &args![group, attacker, ignored]).bool() {
        e.call(0x009d_9ee0, &args![request, ignored]);
    }
    if !e.call(0x008b_b630, &args![attacker, request]).bool() {
        update.fail(e, NO_LOS_PATH);
        e.call(0x0049_f1f0, &args![ignored]);
        e.call(0x006e_58d0, &args![request]);
        return;
    }
    fn_009d8d00(e, Ptr::new(group), attacker);
    update.start_with_setting(e, SEARCH_CHECK_DESTINATION_TIMER, 0x011c_ecc4);
    update.set_state(e, 3);
    update.start_with_setting(e, SEARCH_CHECK_DOOR_TIMER, 0x011c_e8f4);
    e.call(0x0049_f1f0, &args![ignored]);
    e.call(0x006e_58d0, &args![request]);
    e.call(0x0099_6e20, &args![this, 1u32]);
}

/// States 1 and 8 of `009d8d40`.
fn search_moving(e: &mut Engine, update: &SearchUpdate) {
    let attacker = update.attacker;
    if e.call(PATH_FAILED, &args![attacker]).bool() {
        if !e.call(PATH_VALID, &args![attacker]).bool() {
            e.call(
                STATE_ADD_UNREACHABLE_LOCATION,
                &args![update.state, update.field(SEARCH_INVESTIGATE_LOCATION)],
            );
        }
        let text = if e.call(PATH_VALID, &args![attacker]).bool() {
            MOVING_FAILED
        } else {
            BUILDING_FAILED
        };
        update.fail(e, text);
        return;
    }
    let mut seen = false;
    if e.mem.u32(update.field(SEARCH_STATE)) == 1
        && e.call(
            TIMER_EXPIRED,
            &args![update.field(SEARCH_CHECK_DESTINATION_TIMER)],
        )
        .bool()
    {
        let own = update.local(0x374);
        e.call(GET_WORLD_LOCATION_FOR_SEARCH, &args![attacker, own]);
        let squared = e
            .call(
                0x0052_7a40,
                &args![own, update.field(SEARCH_INVESTIGATE_LOCATION)],
            )
            .f64();
        let first = e.call(0x0040_3e20, &args![0x011c_f2a0u32]).u32();
        let second = e.call(0x0040_3e20, &args![0x011c_f2a0u32]).u32();
        let range = x87_mul(x87_load32(e.mem.u32(first)), x87_m32(e.mem.u32(second)));
        if range > squared {
            let location = update.local(0x39c);
            e.call(
                0x006d_d220,
                &args![location, update.field(SEARCH_INVESTIGATE_LOCATION)],
            );
            let point = e.call(0x0068_15c0, &args![own]).u32();
            if e.call(0x006d_7490, &args![location, point, 0u32, 0u32])
                .bool()
            {
                seen = true;
            }
            e.call(0x004f_f7e0, &args![location]);
        }
        update.start_with_setting(e, SEARCH_CHECK_DESTINATION_TIMER, 0x011c_fad4);
    }
    if seen || e.call(PATH_COMPLETE, &args![attacker]).bool() {
        if seen {
            e.call(0x008b_3ab0, &args![attacker]);
        }
        e.call(
            0x0098_bd70,
            &args![update.group, update.field(SEARCH_INVESTIGATE_LOCATION)],
        );
        update.look_around(e);
    }
}

/// State 2 of `009d8d40`.
fn search_to_door(e: &mut Engine, update: &SearchUpdate) {
    let attacker = update.attacker;
    if e.call(PATH_FAILED, &args![attacker]).bool() {
        if !e.call(PATH_VALID, &args![attacker]).bool() && e.mem.u32(update.field(SEARCH_DOOR)) != 0
        {
            let door = e.mem.u32(update.field(SEARCH_DOOR));
            let location = e
                .call(
                    GET_WORLD_LOCATION_FOR_SEARCH,
                    &args![door, update.local(0x59c)],
                )
                .u32();
            e.call(
                STATE_ADD_UNREACHABLE_LOCATION,
                &args![update.state, location],
            );
            let door = e.mem.u32(update.field(SEARCH_DOOR));
            e.call(0x0098_c710, &args![update.group, door]);
        }
        let text = if e.call(PATH_VALID, &args![attacker]).bool() {
            MOVING_TO_DOOR_FAILED
        } else {
            BUILDING_TO_DOOR_FAILED
        };
        update.fail(e, text);
        return;
    }
    if !e.call(PATH_COMPLETE, &args![attacker]).bool() {
        return;
    }
    if e.mem.u32(update.field(SEARCH_DOOR)) == 0 {
        update.look_around(e);
        return;
    }
    let door = e.mem.u32(update.field(SEARCH_DOOR));
    e.call(0x0098_c6e0, &args![update.group, door]);
    if update.process_type == 0 {
        e.call(0x008b_cd60, &args![attacker]);
        update.set_state(e, 4);
        let delay = x87_quiet_bits(e.mem.u32(THREE_FLOAT));
        update.start(e, SEARCH_LOOKING_TIMER, delay);
    } else {
        let request = update.local(0x454);
        e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
        let door = e.mem.u32(update.field(SEARCH_DOOR));
        let teleport = e.call(0x0056_8e50, &args![door]).u32();
        if teleport == 0 {
            e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
            return;
        }
        let space = e.call(0x0043_a3f0, &args![teleport]).u32();
        let point = e.call(0x0071_7e50, &args![teleport]).u32();
        if !update.request_goal(e, 0x454, point, space, 0, |_, _| {}) {
            update.fail(e, NO_PATH_THROUGH_DOOR);
            e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
            return;
        }
        update.set_state(e, 2);
        e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
    }
    e.mem.set_u32(update.field(SEARCH_DOOR), 0);
}

/// State 3 of `009d8d40`.
fn search_line_of_sight(e: &mut Engine, update: &SearchUpdate) {
    let attacker = update.attacker;
    let group = update.group;
    if e.call(PATH_FAILED, &args![attacker]).bool() {
        if !e.call(PATH_VALID, &args![attacker]).bool() {
            let failed = e.call(0x008b_39d0, &args![attacker]).u32();
            if failed != 0 && update.grid != 0 {
                e.call(0x006d_d6f0, &args![failed, 0u32]);
                e.call(0x006e_0bc0, &args![update.grid, failed, 1u32]);
                e.call(0x0091_5d20, &args![group, 1u32]);
            }
        }
        let text = if e.call(PATH_VALID, &args![attacker]).bool() {
            SEARCHING_FAILED
        } else {
            BUILDING_SEARCH_FAILED
        };
        update.fail(e, text);
        return;
    }
    if e.call(TIMER_EXPIRED, &args![update.field(SEARCH_CHECK_DOOR_TIMER)])
        .bool()
        && update.process_type == 0
    {
        update.start_with_setting(e, SEARCH_CHECK_DOOR_TIMER, 0x011c_e8f4);
        let value = e.call(0x0040_3e20, &args![0x011c_e0f0u32]).u32();
        let range = x87_quiet_bits(e.mem.u32(value));
        let door = e
            .call(0x0098_c3d0, &args![group, attacker, range, 1u32])
            .u32();
        if door != 0 {
            let request = update.local(0x514);
            e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
            let space = e.call(0x0057_5ca0, &args![door]).u32();
            let point = e.call(0x0056_8fa0, &args![door]).u32();
            if update.request_goal(e, 0x514, point, space, 0, |_, _| {}) {
                update.set_state(e, 2);
                e.mem.set_u32(update.field(SEARCH_DOOR), door);
                e.call(0x0098_c750, &args![group, door]);
                e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                return;
            }
            e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
        }
    }
    let mut arrived = false;
    if update.grid != 0
        && e.call(
            TIMER_EXPIRED,
            &args![update.field(SEARCH_CHECK_DESTINATION_TIMER)],
        )
        .bool()
    {
        let location = update.local(0x53c);
        e.call(0x006d_cbb0, &args![location]);
        if e.call(0x008b_3860, &args![attacker, location]).bool() {
            let point = e
                .call(0x005c_3420, &args![location, update.local(0x5a8)])
                .u32();
            let position = e.vcall(attacker, 0x1f4, &args![]).u32();
            let offset = update.local(0x548);
            e.call(0x0043_9ef0, &args![position, offset, point]);
            let squared = x87_store32(e.call(0x004a_7290, &args![offset]).f64());
            let first = e.call(0x0040_3e20, &args![0x011c_f3c0u32]).u32();
            let second = e.call(0x0040_3e20, &args![0x011c_f3c0u32]).u32();
            let range = x87_mul(x87_load32(e.mem.u32(first)), x87_m32(e.mem.u32(second)));
            if x87_load32(squared) < range {
                e.call(0x006d_d6f0, &args![location, 0u32]);
                if e.call(0x006d_d6a0, &args![location]).bool()
                    && e.call(0x006e_0b50, &args![update.grid, location]).bool()
                {
                    arrived = true;
                    let mesh = update.local(0x550);
                    e.call(0x0042_fb00, &args![mesh]);
                    if e.call(0x006d_d610, &args![location, mesh]).bool() {
                        let triangle = u32::from(e.call(0x006d_f760, &args![location]).u16());
                        let navmesh = e.call(0x0055_9450, &args![mesh]).u32();
                        let kind = e.call(0x006d_8910, &args![navmesh, triangle]).u8() as i8;
                        let too_far = |e: &mut Engine, setting: u32| {
                            let first = e.call(0x0040_3e20, &args![setting]).u32();
                            let second = e.call(0x0040_3e20, &args![setting]).u32();
                            let range =
                                x87_mul(x87_load32(e.mem.u32(first)), x87_m32(e.mem.u32(second)));
                            x87_load32(squared) > range
                        };
                        if (kind == 1 && too_far(e, 0x011c_eb68))
                            || (kind == 2 && too_far(e, 0x011c_f2e0))
                        {
                            arrived = false;
                        }
                    }
                    if arrived {
                        e.call(0x008b_3ab0, &args![attacker]);
                    }
                    e.call(0x0042_fa40, &args![mesh]);
                }
            }
        }
        update.start_with_setting(e, SEARCH_CHECK_DESTINATION_TIMER, 0x011c_ecc4);
        e.call(0x004f_f7e0, &args![location]);
    }
    if arrived || e.call(PATH_COMPLETE, &args![attacker]).bool() {
        update.look_around(e);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009d_8b60, combat_procedure_search_construct(Ptr) -> Ptr),
        entry!(0x009d_8c00, fn_009d8c00(Ptr) -> u32),
        entry!(0x009d_8c10, fn_009d8c10(Ptr, u32) -> Ptr),
        entry!(0x009d_8c60, combat_procedure_search_shutdown(Ptr)),
        entry!(0x009d_8d00, fn_009d8d00(Ptr, u32)),
        entry!(0x009d_8d20, fn_009d8d20(Ptr) -> u32),
        entry!(0x009d_9f60, fn_009d9f60(Ptr, Ptr) -> Ptr),
        entry!(
            0x009d_9fa0,
            combat_procedure_search_activated_reference(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x009d_a550, fn_009da550(Ptr, Ptr) -> Ptr),
        entry!(0x009d_a570, fn_009da570(Ptr) -> f32),
        entry!(0x009d_a5c0, combat_procedure_search_save_game(Ptr, u32)),
        entry!(0x009d_a640, combat_procedure_search_load_game(Ptr, u32)),
        entry!(
            0x009d_a6c0,
            combat_procedure_search_init_load_game(Ptr, u32)
        ),
        entry!(0x009d_8d40, combat_procedure_search_update(Ptr)),
        entry!(
            0x009d_9fd0,
            combat_procedure_search_display_debug_text(Ptr, Ptr, u32, u32)
        ),
        entry!(0x009d_8c40, fn_009d8c40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009d8c40_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x009c_a6c0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009d_8c40, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_1db4);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_1db4,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n025_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Every callee outside this file that the batch's translations call, and that of the
    /// destructor `009d8c40` (`009ca6c0`).
    const EXTERNAL: &[u32] = &[
        0x0040_1030,
        0x0040_3e20,
        0x0041_81e0,
        0x0042_fa40,
        0x0042_fb00,
        0x0043_5e00,
        0x0043_9ef0,
        0x0043_a3f0,
        0x0045_34f0,
        0x0045_8400,
        0x0045_cec0,
        0x0048_39c0,
        0x0049_f1c0,
        0x0049_f1f0,
        0x004a_7290,
        0x004a_7bd0,
        0x004b_1480,
        0x004f_b070,
        0x004f_f7e0,
        0x0052_7a40,
        0x0052_7b70,
        0x0052_7ba0,
        0x0055_9450,
        0x0056_8e50,
        0x0056_8fa0,
        0x0056_9000,
        0x0057_5650,
        0x0057_57d0,
        0x0057_5ca0,
        0x0058_6150,
        0x005c_3420,
        0x0063_3c90,
        0x0068_15c0,
        0x006a_7f50,
        0x006b_9130,
        0x006d_1bc0,
        0x006d_7490,
        0x006d_8910,
        0x006d_cbb0,
        0x006d_ce10,
        0x006d_d220,
        0x006d_d610,
        0x006d_d6a0,
        0x006d_d6f0,
        0x006d_f760,
        0x006e_0b50,
        0x006e_0bc0,
        0x006e_2420,
        0x006e_2620,
        0x006e_29f0,
        0x006e_2b50,
        0x006e_4340,
        0x006e_57e0,
        0x006e_58d0,
        0x006e_5a40,
        0x0070_58c0,
        0x0071_7e50,
        0x0084_e3a0,
        0x0086_48e0,
        0x0086_4980,
        0x0086_5df0,
        0x0086_5e50,
        0x008a_d8f0,
        0x008b_3630,
        0x008b_3860,
        0x008b_38d0,
        0x008b_3960,
        0x008b_39d0,
        0x008b_3ab0,
        0x008b_3b90,
        0x008b_3bb0,
        0x008b_b630,
        0x008b_cd20,
        0x008b_cd60,
        0x008b_cda0,
        0x008d_7f40,
        0x008d_7f80,
        0x008d_f000,
        0x008e_7c20,
        0x008e_7c50,
        0x0090_6f60,
        0x0091_5d20,
        0x0093_1850,
        0x0094_42a0,
        0x0097_ede0,
        0x0097_ef30,
        0x0097_fa90,
        0x0098_5bb0,
        0x0098_bc00,
        0x0098_bd70,
        0x0098_c3d0,
        0x0098_c6e0,
        0x0098_c710,
        0x0098_c750,
        0x0098_c780,
        0x0098_c7b0,
        0x0099_6de0,
        0x0099_6e00,
        0x0099_6e20,
        0x0099_6e40,
        0x0099_6ed0,
        0x0099_6ef0,
        0x009a_02a0,
        0x009a_0b00,
        0x009a_0ec0,
        0x009a_5eb0,
        0x009a_5f20,
        0x009c_a680,
        0x009c_aac0,
        0x009d_9ec0,
        0x009d_9ee0,
        0x009d_9f00,
        0x009d_9f20,
        0x009d_9f40,
        0x00ec_43fb,
        0x009c_a6c0,
    ];

    const VTABLE: u32 = 0x0200_0000;
    const SLOTS: u32 = 0x0300_0000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| ret(eax));
    }

    fn stub_st0(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| st0(value));
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn logged(e: &mut Engine, f: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        f(e);
        e.call_log.take().unwrap()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in EXTERNAL {
            stub(&mut e, addr, 0);
        }
        let slots: Vec<u32> = (0..0x200).map(|s| SLOTS + 4 * s).collect();
        for &slot in &slots {
            stub(&mut e, slot, 0);
        }
        e.put_vtable(VTABLE, &slots);
        e.map(0x0101_0000, 0x0008_4000);
        e.map(0x011c_0000, 0x0002_0000);
        for (addr, value) in [
            (ONE_HUNDRED_TWENTY_EIGHT, 128.0f32),
            (FIVE_HUNDRED_TWELVE, 512.0),
            (TWO_FLOAT, 2.0),
            (THREE_FLOAT, 3.0),
            (0x0101_2054, -1.0),
        ] {
            e.mem.set_f32(addr, value);
        }
        for (addr, value) in [
            (THIRTY_DOUBLE, 30.0f64),
            (PI_DOUBLE, f64::from(std::f32::consts::PI)),
            (ONE_DOUBLE, 1.0),
            (ZERO_DOUBLE, 0.0),
            (NEAR_CENTER_DOUBLE, 16384.0),
            (0x0101_7a40, 100.0),
        ] {
            e.mem.set_f64(addr, value);
        }
        e.register(0x0040_3e20, |_, a| ret(a[0] + 4));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x005c_3420, |_, a| ret(a[1]));
        e.register(0x0057_5650, |_, a| ret(a[1]));
        e.register(0x0043_9ef0, |_, a| ret(a[1]));
        e.register(TIMER_START, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            ret(0)
        });
        e
    }

    /// A search procedure with its controller, attacker (process type `process_type`) and
    /// group.
    struct Search {
        this: u32,
        attacker: u32,
        group: u32,
        state: u32,
    }

    fn search(e: &mut Engine, process_type: u32) -> Search {
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, SEARCH_VTABLE);
        e.mem.set_u32(this + 4, 0x700);
        let attacker = e.mem.alloc(0x200);
        e.mem.set_u32(attacker, VTABLE);
        let group = e.mem.alloc(0x200);
        stub(e, GET_ATTACKER, attacker);
        stub(e, CONTROLLER_GROUP, group);
        stub(e, 0x0093_1850, process_type);
        stub(e, 0x0058_6150, 0x900);
        Search {
            this,
            attacker,
            group,
            state: 0x900,
        }
    }

    fn state_of(e: &Engine, s: &Search) -> u32 {
        e.mem.u32(s.this + SEARCH_STATE)
    }

    #[test]
    fn constructor_type_and_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xcd; 0x40]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009d_8b60, &args![this]).u32(), this);
        });
        assert_eq!(e.mem.u32(this), SEARCH_VTABLE);
        assert_eq!((e.mem.u32(this + 0x10), e.mem.u32(this + 0x3c)), (0, 0));
        assert_eq!(
            calls(&log, TIMER_CONSTRUCT),
            vec![vec![this + 0x14], vec![this + 0x1c], vec![this + 0x24]]
        );
        assert_eq!(calls(&log, 0x0097_ede0), vec![vec![this + 0x2c]]);
        assert_eq!(e.call(0x009d_8c00, &args![this]).u32(), 10);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009d_8c10, &args![this, 1u32]).u32(), this);
            e.call(0x009d_8c10, &args![this, 0u32]);
        });
        assert_eq!(calls(&log, 0x009c_a6c0).len(), 2);
        assert_eq!(calls(&log, 0x0040_1030), vec![vec![this]]);
    }

    #[test]
    fn shutdown_releases_the_group_slot_and_door() {
        let mut e = engine();
        let s = search(&mut e, 0);
        e.mem.set_u32(s.group + 0xec, s.attacker);
        e.mem.set_u32(s.this + SEARCH_STATE, 2);
        e.mem.set_u32(s.this + SEARCH_DOOR, 0x55);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8c60, &args![s.this]);
        });
        assert_eq!(e.mem.u32(s.group + 0xec), 0);
        assert_eq!(calls(&log, 0x008b_3ab0), vec![vec![s.attacker]]);
        assert_eq!(calls(&log, 0x0099_6e40), vec![vec![s.this]]);
        assert_eq!(calls(&log, 0x0098_c780), vec![vec![s.group, 0x55]]);
        assert!(calls(&log, 0x008b_cd20).is_empty());
        // States 4 and 5 fade back in; another searcher keeps the slot.
        e.mem.set_u32(s.group + 0xec, 0x1234);
        e.mem.set_u32(s.this + SEARCH_STATE, 5);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8c60, &args![s.this]);
        });
        assert_eq!(e.mem.u32(s.group + 0xec), 0x1234);
        assert_eq!(calls(&log, 0x008b_cd20), vec![vec![s.attacker]]);
        assert!(calls(&log, 0x0098_c780).is_empty());
    }

    #[test]
    fn group_getters_and_copies() {
        let mut e = engine();
        let group = e.mem.alloc(0x100);
        e.call(0x009d_8d00, &args![group, 0x77u32]);
        assert_eq!(e.call(0x009d_8d20, &args![group]).u32(), 0x77);
        for k in 0..4u32 {
            e.mem.set_u32(group + 0xf0 + 4 * k, 0x10 + k);
        }
        e.mem.set_u32(group + 0xe8, 0x99);
        let out = e.mem.alloc(16);
        assert_eq!(e.call(0x009d_9f60, &args![group, out]).u32(), out);
        assert_eq!(e.mem.u32(out + 0xc), 0x13);
        assert_eq!(e.call(0x009d_a550, &args![group, out]).u32(), out);
        assert_eq!(e.mem.u32(out), 0x99);
        // The distance travelled, or -1 without a handler.
        assert_eq!(e.call(0x009d_a570, &args![group]).f64(), -1.0);
        stub(&mut e, 0x0055_9450, 0x500);
        stub_st0(&mut e, 0x004a_7bd0, 12.5);
        assert_eq!(e.call(0x009d_a570, &args![group]).f64(), 12.5);
    }

    #[test]
    fn activated_reference_forgets_the_door() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + SEARCH_DOOR, 0x55);
        e.call(0x009d_9fa0, &args![this, 0x56u32, 0u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + SEARCH_DOOR), 0x55);
        e.call(0x009d_9fa0, &args![this, 0x55u32, 0u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + SEARCH_DOOR), 0);
    }

    #[test]
    fn save_load_and_init_load() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + SEARCH_DOOR, 0x66);
        e.register(0x0048_39c0, |_, a| ret(a[0] + 1));
        e.register(0x00ec_43fb, |_, a| ret(a[0] + a[3]));
        let log = logged(&mut e, |e| {
            e.call(0x009d_a5c0, &args![this, 0x8000u32]);
            e.call(0x009d_a640, &args![this, 0x8000u32]);
            e.call(0x009d_a6c0, &args![this, 0x8000u32]);
        });
        assert_eq!(calls(&log, PROCEDURE_SAVE_GAME), vec![vec![this, 0x8000]]);
        assert_eq!(
            calls(&log, TIMER_SAVE_GAME),
            vec![
                vec![this + 0x1c, 0x8000],
                vec![this + 0x14, 0x8000],
                vec![this + 0x24, 0x8000]
            ]
        );
        assert_eq!(calls(&log, 0x0086_5df0), vec![vec![0x8000, 0x66, 0]]);
        assert_eq!(calls(&log, TIMER_LOAD_GAME).len(), 3);
        assert_eq!(calls(&log, 0x0086_48e0), vec![vec![0x8000, this + 0x3c]]);
        assert_eq!(calls(&log, 0x0045_34f0), vec![vec![this, 0x8000]]);
        assert_eq!(e.mem.u32(this + SEARCH_DOOR), 0x67 + TES_OBJECT_REFR_TYPE);
        e.mem.set_u32(this + SEARCH_DOOR, 0);
        e.call(0x009d_a6c0, &args![this, 0x8000u32]);
        assert_eq!(e.mem.u32(this + SEARCH_DOOR), 0);
    }

    #[test]
    fn display_debug_text_prints_the_state() {
        let mut e = engine();
        let s = search(&mut e, 0);
        let printer = e.mem.alloc(8);
        e.mem.set_u32(printer, VTABLE);
        let p = printer;
        let run = |e: &mut Engine| {
            let log = logged(e, |e| {
                e.call(0x009d_9fd0, &args![s.this, p, 1u32, 2u32]);
            });
            calls(&log, SLOTS + 0x14)
        };
        assert_eq!(run(&mut e), vec![vec![p, 1, 2, INITIALIZING_TEXT]]);
        // Looking: the timer's elapsed and total time.
        e.mem.set_u32(s.this + SEARCH_STATE, 6);
        stub_st0(&mut e, 0x006b_9130, 4.0);
        stub_st0(&mut e, 0x0043_5e00, 1.0);
        let printed = run(&mut e);
        assert_eq!(printed[0], vec![p, 1, 2, LOOKING_TEXT]);
        let (elapsed, total) = (1.0f64.to_bits(), 4.0f64.to_bits());
        assert_eq!(
            printed[1],
            vec![
                p,
                1,
                2,
                LOOKING_TIMER_FORMAT,
                elapsed as u32,
                (elapsed >> 32) as u32,
                total as u32,
                (total >> 32) as u32
            ]
        );
        // Investigating: the point and its space's name.
        e.mem.set_u32(s.this + SEARCH_STATE, 1);
        e.mem.set_f32(s.this + 0x2c, 3.0);
        let space = e.mem.alloc(8);
        e.mem.set_u32(space, VTABLE);
        stub(&mut e, 0x0084_e3a0, space);
        stub(&mut e, SLOTS + 0x130, 0x4444);
        let printed = run(&mut e);
        assert_eq!(printed[1][3], POINT_FORMAT);
        assert_eq!(printed[1][4..6], [0, 0x4008_0000]);
        assert_eq!(printed[2], vec![p, 1, 2, SPACE_FORMAT, 0x4444]);
        // A non-negative distance travelled adds the summary.
        stub(&mut e, 0x0055_9450, 0x500);
        stub(&mut e, 0x0045_8400, 7);
        let printed = run(&mut e);
        assert_eq!(printed[3][3..5], [SEARCH_SUMMARY_FORMAT, 7]);
    }

    #[test]
    fn update_start_investigates_or_searches_doors() {
        let mut e = engine();
        let s = search(&mut e, 1);
        // The group may not search: nothing (and no move mode).
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert!(calls(&log, 0x0099_6e20).is_empty());
        stub(&mut e, 0x0097_ef30, 1);
        // An investigation point 40 away.
        let point = e.mem.alloc(16);
        e.mem.set_u32(point + 0xc, 0x77);
        stub(&mut e, 0x0098_bc00, point);
        stub_st0(&mut e, 0x0098_5bb0, 40.0);
        stub(&mut e, 0x008b_3630, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(state_of(&e, &s), 1);
        assert_eq!(e.mem.u32(s.this + 0x38), 0x77);
        assert_eq!(calls(&log, 0x008e_7c50).len(), 1);
        assert_eq!(calls(&log, 0x008b_38d0)[0][4], 128.0f32.to_bits());
        assert_eq!(calls(&log, 0x0099_6e20), vec![vec![s.this, 1]]);
        // Refused path: the procedure fails.
        e.mem.set_u32(s.this + SEARCH_STATE, 0);
        stub(&mut e, 0x008b_3630, 0);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(
            calls(&log, PROCEDURE_FAIL),
            vec![vec![s.this, NO_PATH_TO_INVESTIGATE]]
        );
        assert!(calls(&log, 0x0099_6e20).is_empty());
        // Near point, a door instead: state 2 and the door reserved.
        stub_st0(&mut e, 0x0098_5bb0, 10.0);
        stub(&mut e, 0x0098_c3d0, 0x88);
        stub(&mut e, 0x008b_3630, 1);
        let rotation = e.mem.alloc(12);
        stub(&mut e, 0x0056_9000, rotation);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(state_of(&e, &s), 2);
        assert_eq!(e.mem.u32(s.this + SEARCH_DOOR), 0x88);
        assert_eq!(calls(&log, 0x0098_c750), vec![vec![s.group, 0x88]]);
        assert_eq!(calls(&log, 0x008e_7c20).len(), 1);
        assert_eq!(calls(&log, 0x0098_c3d0)[0][..2], [s.group, s.attacker]);
        // A low-process attacker whose controller says 4 looks around (state 7).
        let low = search(&mut e, 0);
        stub(&mut e, 0x009d_9f00, 4);
        e.call(0x009d_8d40, &args![low.this]);
        assert_eq!(state_of(&e, &low), 7);
        assert_eq!(e.mem.f32(low.this + 0x1c + 4), 2.0);
    }

    #[test]
    fn update_start_moves_to_the_center_or_searches_by_sight() {
        let mut e = engine();
        let s = search(&mut e, 1);
        stub(&mut e, 0x0097_ef30, 1);
        // No search point, no door: the group's search location, 200 away.
        stub_st0(&mut e, 0x0052_7a40, 40000.0);
        stub(&mut e, 0x009a_02a0, 1);
        stub(&mut e, 0x008b_3630, 1);
        e.mem.set_u32(s.group + 0xfc, 0x99);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(state_of(&e, &s), 8);
        assert_eq!(e.mem.u32(s.this + 0x38), 0x99);
        assert_eq!(calls(&log, 0x009a_02a0)[0][0], s.state);
        assert_eq!(calls(&log, 0x0099_6e20).len(), 1);
        // Already there: look around.
        e.mem.set_u32(s.this + SEARCH_STATE, 0);
        stub_st0(&mut e, 0x0052_7a40, 100.0);
        e.call(0x009d_8d40, &args![s.this]);
        assert_eq!(state_of(&e, &s), 6);
        // A line-of-sight grid in the same space: the group's searcher searches by sight.
        e.mem.set_u32(s.this + SEARCH_STATE, 0);
        stub(&mut e, 0x009d_9f20, 0x600);
        stub(&mut e, 0x008b_b630, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(state_of(&e, &s), 3);
        assert_eq!(e.mem.u32(s.group + 0xec), s.attacker);
        assert_eq!(calls(&log, 0x0063_3c90)[0][1], 0x600);
        assert_eq!(calls(&log, 0x006e_4340)[0][1], 512.0f32.to_bits());
        assert_eq!(calls(&log, 0x006e_58d0).len(), 1);
        // Another searcher holds the slot: nothing.
        e.mem.set_u32(s.this + SEARCH_STATE, 0);
        e.mem.set_u32(s.group + 0xec, 0x1234);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(state_of(&e, &s), 0);
        assert!(calls(&log, 0x006e_57e0).is_empty());
    }

    #[test]
    fn update_moving_and_door_states() {
        let mut e = engine();
        let s = search(&mut e, 0);
        // Moving with a failed, invalid path: unreachable, then failure.
        e.mem.set_u32(s.this + SEARCH_STATE, 8);
        stub(&mut e, 0x008b_3b90, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(calls(&log, 0x009a_0ec0), vec![vec![s.state, s.this + 0x2c]]);
        assert_eq!(
            calls(&log, PROCEDURE_FAIL),
            vec![vec![s.this, BUILDING_FAILED]]
        );
        // Arrived: the group is told and the searcher looks around.
        stub(&mut e, 0x008b_3b90, 0);
        stub(&mut e, 0x008b_3bb0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(calls(&log, 0x0098_bd70), vec![vec![s.group, s.this + 0x2c]]);
        assert_eq!(state_of(&e, &s), 6);
        // At the door with a low-process attacker: fade (state 4, timer 3).
        e.mem.set_u32(s.this + SEARCH_STATE, 2);
        e.mem.set_u32(s.this + SEARCH_DOOR, 0x55);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(calls(&log, 0x0098_c6e0), vec![vec![s.group, 0x55]]);
        assert_eq!(state_of(&e, &s), 4);
        assert_eq!(e.mem.f32(s.this + 0x1c + 4), 3.0);
        assert_eq!(e.mem.u32(s.this + SEARCH_DOOR), 0);
        // State 4 ends when faded out (5); state 5 fades in on the timer (0).
        e.call(0x009d_8d40, &args![s.this]);
        assert_eq!(state_of(&e, &s), 5);
        stub(&mut e, TIMER_EXPIRED, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(calls(&log, 0x008b_cd20), vec![vec![s.attacker]]);
        assert_eq!(state_of(&e, &s), 0);
        // State 6 ends on the timer too.
        e.mem.set_u32(s.this + SEARCH_STATE, 6);
        e.call(0x009d_8d40, &args![s.this]);
        assert_eq!(state_of(&e, &s), 0);
    }

    #[test]
    fn update_line_of_sight_state() {
        let mut e = engine();
        let s = search(&mut e, 1);
        stub(&mut e, 0x009d_9f20, 0x600);
        e.mem.set_u32(s.this + SEARCH_STATE, 3);
        // A failed path blocks its destination in the grid.
        stub(&mut e, 0x008b_3b90, 1);
        stub(&mut e, 0x008b_39d0, 0x650);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(calls(&log, 0x006e_0bc0), vec![vec![0x600, 0x650, 1]]);
        assert_eq!(calls(&log, 0x0091_5d20), vec![vec![s.group, 1]]);
        assert_eq!(
            calls(&log, PROCEDURE_FAIL),
            vec![vec![s.this, BUILDING_SEARCH_FAILED]]
        );
        // A seen grid point near enough stops the search (state 6).
        stub(&mut e, 0x008b_3b90, 0);
        stub(&mut e, TIMER_EXPIRED, 1);
        stub(&mut e, 0x008b_3860, 1);
        let position = e.mem.alloc(12);
        stub(&mut e, SLOTS + 0x1f4, position);
        e.register(0x0040_3e20, |e, a| {
            if a[0] == 0x011c_f3c0 {
                e.mem.set_f32(a[0] + 4, 10.0);
            }
            ret(a[0] + 4)
        });
        stub_st0(&mut e, 0x004a_7290, 50.0);
        stub(&mut e, 0x006d_d6a0, 1);
        stub(&mut e, 0x006e_0b50, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009d_8d40, &args![s.this]);
        });
        assert_eq!(calls(&log, 0x008b_3ab0), vec![vec![s.attacker]]);
        assert_eq!(state_of(&e, &s), 6);
        assert_eq!(calls(&log, 0x0042_fa40).len(), 1);
        // State 7 waits for the group's search-area timer in a new space.
        e.mem.set_u32(s.this + SEARCH_STATE, 7);
        e.register(0x006a_7f50, |_, a| st0(if a[0] == 0 { 1.0 } else { 2.0 }));
        stub(&mut e, 0x0084_e3a0, 0x10);
        stub(&mut e, 0x0057_5ca0, 0x20);
        e.call(0x009d_8d40, &args![s.this]);
        assert_eq!(state_of(&e, &s), 7);
        stub(&mut e, 0x0057_5ca0, 0x10);
        e.call(0x009d_8d40, &args![s.this]);
        assert_eq!(state_of(&e, &s), 0);
    }
}
