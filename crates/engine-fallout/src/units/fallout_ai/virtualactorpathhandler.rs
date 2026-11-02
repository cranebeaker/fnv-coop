//! `fallout/ai/movement/virtualactorpathhandler.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `VirtualActorPathHandler` (Xbox PDB), 0x20 bytes: an
    /// `ActorPathHandler` (0x14) and the walk along the path.
    pub struct VirtualActorPathHandler: 0x20 {
        /// `pActor` (Xbox PDB, `ActorPathHandler`): `Actor*`.
        0x04 pActor: u32,
        /// `pActorMover` (Xbox PDB, `ActorPathHandler`): `ActorMover*`.
        0x08 pActorMover: u32,
        /// `spPath` (Xbox PDB, `ActorPathHandler`):
        /// `NiPointer<PathingSolution>`.
        0x0c spPath: u32,
        /// `spRequest` (Xbox PDB, `ActorPathHandler`):
        /// `NiPointer<PathingRequest>`.
        0x10 spRequest: u32,
        /// `iCurrentNodeIndex` (Xbox PDB).
        0x14 iCurrentNodeIndex: u32,
        /// `fDistanceTraveled` (Xbox PDB).
        0x18 fDistanceTraveled: f32,
        /// `fPathLength` (Xbox PDB).
        0x1c fPathLength: f32,
    }
}

/// The `VirtualActorPathHandler` vtable.
const VIRTUAL_ACTOR_PATH_HANDLER_VTABLE: u32 = 0x0109_29fc;
/// The player character pointer.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// The calendar (`Calendar::GetTimeScale`'s `this`).
const CALENDAR: u32 = 0x011d_e7b8;
/// The zero point (`NiPoint3`, three words).
const ZERO_POINT: u32 = 0x011f_426c;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `3600.0` (`double`): seconds per hour.
const SECONDS_PER_HOUR: u32 = 0x0101_2640;
/// A `double` the update divides by the time scale when the actor is not
/// waiting (the update's time step).
const WAIT_STEP: u32 = 0x0108_48d0;
/// A `float` the update subtracts the acquire object's value from.
const ACQUIRE_TIME: u32 = 0x011e_0358;
/// The distance (`double`) beyond which a teleport door's arrival counts as
/// wrong.
const DOOR_ARRIVAL_TOLERANCE: u32 = 0x0102_f078;
/// `"PATHFINDING: Actor %s used teleport door %s in cell %s and was not
/// moved to the expected pathing location, check teleport marker"`.
const DOOR_WARNING: u32 = 0x0109_2a80;
/// The text used for a door without a cell.
const NO_CELL_TEXT: u32 = 0x0101_1584;

/// `00559450` (`thiscall`): the word at `this` (the pointed-to object of a
/// smart pointer).
const WORD_AT: u32 = 0x0055_9450;
/// `005a4320` (`thiscall`): the path's node count.
const PATH_NODE_COUNT: u32 = 0x005a_4320;
/// `006e78b0` (`thiscall`, `RET 4`): the path node at an index.
const PATH_NODE: u32 = 0x006e_78b0;
/// `006e7800` (`thiscall`): the path's length, in `ST0`.
const PATH_LENGTH: u32 = 0x006e_7800;
/// `0066b0d0` (`thiscall`, `RET 4`): sets a smart pointer.
const SMART_POINTER_SET: u32 = 0x0066_b0d0;
/// `BGSSaveGameBuffer` raw save (`00865e50`, `thiscall`: data, size, 0).
const SAVE_DATA: u32 = 0x0086_5e50;
/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The `ActorPathHandler` constructor and destructor body (unit
/// `run_009dfe40`).
const BASE_CONSTRUCT: u32 = 0x009d_fe40;
const BASE_DESTRUCT: u32 = 0x009d_fed0;

/// `MiddleHighProcess::GetSavedAcquireObject` (engine map), on the actor.
const SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// `PlayerCharacter::IsSleepingorResting` (engine map).
const IS_SLEEPING_OR_RESTING: u32 = 0x0094_df60;
/// `Actor::IsFollowing_ov2` (engine map), `RET 4`.
const IS_FOLLOWING: u32 = 0x008b_c7f0;
/// `00493bb0` (`thiscall` on the actor): a test that keeps the given time
/// step (`AL`).
const KEEPS_TIME_STEP: u32 = 0x0049_3bb0;
/// `Calendar::GetTimeScale` (engine map), in `ST0`.
const GET_TIME_SCALE: u32 = 0x0086_7950;
/// `007df1f0` (`thiscall`): a value of the acquire object, in `ST0`.
const ACQUIRE_VALUE: u32 = 0x007d_f1f0;
/// `Actor::GetCurrentSpeed` (engine map), in `ST0`.
const CURRENT_SPEED: u32 = 0x008a_0b10;
/// `BaseProcess::GetActorPackageThatIsRunning` (engine map), on a node.
const NODE_PART: u32 = 0x0071_7e50;
/// `005c3420` (`thiscall`, `RET 4`): the node's point, into a local.
const NODE_POINT: u32 = 0x005c_3420;
/// `00439ef0` (`thiscall`, `RET 8`): `(point, result, other)`, the
/// difference of two points.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00589850` (`thiscall`): a point's length, in `ST0`.
const POINT_LENGTH: u32 = 0x0058_9850;
/// `GetZAngleFromVector` (engine map), `cdecl`: the point; in `ST0`.
const Z_ANGLE: u32 = 0x004b_13c0;
/// `006ca0c0` (`thiscall`): whether the node is a door (`AL`).
const NODE_IS_DOOR: u32 = 0x006c_a0c0;
/// `TESObjectREFR::GetWorldSpace` (engine map), on the actor.
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `008d6f30` (`thiscall`): the reference's parent cell.
const PARENT_CELL: u32 = 0x008d_6f30;
/// `008ad1c0` (`thiscall`, `RET 0x18`): moves the actor
/// (point, angle, cell, world space, 1, 0).
const MOVE_ACTOR: u32 = 0x008a_d1c0;
/// `MobileObject::GetDesiredProcessLevel` (engine map).
const DESIRED_PROCESS_LEVEL: u32 = 0x0093_34b0;
/// `00439180` (`thiscall`, `RET 4`): scales a point.
const POINT_SCALE: u32 = 0x0043_9180;
/// `0063c8a0` (`thiscall`, `RET 4`): `point += other`.
const POINT_ADD: u32 = 0x0063_c8a0;
/// `009b0bd0` (`thiscall`): the request's target radius, in `ST0`.
const TARGET_RADIUS: u32 = 0x009b_0bd0;
/// `008c7aa0` (no parameters): whether doors are taken through the
/// teleport path (`AL`).
const DOORS_TELEPORT: u32 = 0x008c_7aa0;
/// `004537b0` (no parameters): the object the two door calls run on.
const DOOR_MANAGER: u32 = 0x0045_37b0;
/// `0087b650` (`thiscall`, `RET 8`): (actor, point).
const DOOR_MOVE: u32 = 0x0087_b650;
/// `0087b190` (`thiscall`, `RET 4`): (actor).
const DOOR_FINISH: u32 = 0x0087_b190;
/// `TESObjectREFR::SetLocationOnReference` (engine map), `RET 4`.
const SET_LOCATION: u32 = 0x0057_5830;
/// `0055b980` (`thiscall`): the node's door reference.
const NODE_DOOR: u32 = 0x0055_b980;
/// `TESObjectREFR::Activate` (engine map), `RET 0x10`.
const ACTIVATE: u32 = 0x0057_3170;
/// `00595c80` (`thiscall`): a point's length, in `ST0`.
const POINT_LENGTH_2: u32 = 0x0059_5c80;
/// `00439090` (`thiscall`, `RET 4`): whether two points differ (`AL`).
const POINTS_DIFFER: u32 = 0x0043_9090;
/// `00474cb0` (`thiscall`): a test on a cell or reference, choosing how its
/// name is read.
const NAME_TEST: u32 = 0x0047_4cb0;
/// `MapMarkerData::GetLocationName` (engine map).
const LOCATION_NAME: u32 = 0x0040_8da0;
/// `0055d520` (`thiscall`): a reference's name.
const REFERENCE_NAME: u32 = 0x0055_d520;
/// `005b5e40` (`cdecl`, varargs): the debug print.
const DEBUG_PRINT: u32 = 0x005b_5e40;

// Translated from 004a7bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::GetDistanceTraveled` (Xbox PDB):
/// `fDistanceTraveled` (`FLD`).
pub fn virtual_actor_path_handler_get_distance_traveled(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
) -> f32 {
    x87_quiet(e.get(this, VirtualActorPathHandler::fDistanceTraveled))
}

// Translated from 009ea710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::GetMoveDelta` (Xbox PDB): writes the zero
/// point to `delta` and answers false.
pub fn virtual_actor_path_handler_get_move_delta(
    e: &mut Engine,
    _unused_0: Ptr<VirtualActorPathHandler>,
    delta: u32,
) -> bool {
    for i in 0..3 {
        let word = e.global::<u32>(ZERO_POINT + 4 * i);
        e.mem.set_u32(delta + 4 * i, word);
    }
    false
}

// Translated from 009ea740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler` scalar deleting destructor (Xbox PDB name from
/// the engine map): the destructor body (`009ea7c0`), then `00401030` when
/// `flags & 1`. Returns `this`.
pub fn virtual_actor_path_handler_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    flags: u32,
) -> Ptr<VirtualActorPathHandler> {
    fn_009ea7c0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009ea770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `VirtualActorPathHandler` constructor (no name in the engine map):
/// the `ActorPathHandler` constructor (`009dfe40`) with the four arguments,
/// the vtable, and the distance traveled and path length at 0. Returns
/// `this`.
pub fn fn_009ea770(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    actor: u32,
    mover: u32,
    path: u32,
    request: u32,
) -> Ptr<VirtualActorPathHandler> {
    e.call(BASE_CONSTRUCT, &args![this, actor, mover, path, request]);
    e.mem
        .set_u32(this.addr(), VIRTUAL_ACTOR_PATH_HANDLER_VTABLE);
    e.set(this, VirtualActorPathHandler::fDistanceTraveled, 0.0);
    e.set(this, VirtualActorPathHandler::fPathLength, 0.0);
    this
}

// Translated from 009ea7c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ea7c0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `VirtualActorPathHandler` destructor body (the decompiler's library
/// name for it does not apply): the vtable, `ClearMovement` (`009eb040`),
/// then the `ActorPathHandler` destructor body (`009dfed0`).
pub fn fn_009ea7c0(e: &mut Engine, this: Ptr<VirtualActorPathHandler>) {
    e.mem
        .set_u32(this.addr(), VIRTUAL_ACTOR_PATH_HANDLER_VTABLE);
    virtual_actor_path_handler_clear_movement(e, this);
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 009ea820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::Init` (Xbox PDB; `RET 8`): the current node
/// index is `start` truncated (`FISTP` with rounding toward zero, low
/// word), the distance traveled 0 and the path length the path's
/// (`006e7800`); when `start` equals 0.0 (`0101 2060`) the answer is the
/// handler's slot +0xc with `time`, otherwise 0.
pub fn virtual_actor_path_handler_init(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    start: f32,
    time: f32,
) -> u32 {
    let index = x87_fistp64_chop(X87Ext::load32(start.to_bits())) as u32;
    e.set(this, VirtualActorPathHandler::iCurrentNodeIndex, index);
    e.set(this, VirtualActorPathHandler::fDistanceTraveled, 0.0);
    let path = e.call(WORD_AT, &args![this.addr() + 0xc]).u32();
    let length = e.call(PATH_LENGTH, &args![path]).f64();
    e.mem.set_u32(this.addr() + 0x1c, x87_store32(length));
    if x87_load32(start.to_bits()) == e.global::<f64>(ZERO) {
        e.vcall(this.addr(), 0xc, &args![x87_quiet_bits(time.to_bits())])
            .u32()
    } else {
        0
    }
}

/// The current path's node count (`005a4320` on `00559450(spPath)`).
fn path_node_count(e: &mut Engine, this: Ptr<VirtualActorPathHandler>) -> u32 {
    let path = e.call(WORD_AT, &args![this.addr() + 0xc]).u32();
    e.call(PATH_NODE_COUNT, &args![path]).u32()
}

/// The path node at the current index (`006e78b0`).
fn current_node(e: &mut Engine, this: Ptr<VirtualActorPathHandler>) -> u32 {
    let index = e.get(this, VirtualActorPathHandler::iCurrentNodeIndex);
    let path = e.call(WORD_AT, &args![this.addr() + 0xc]).u32();
    e.call(PATH_NODE, &args![path, index]).u32()
}

/// Moves the actor to `point` facing `angle` (bits) in its world space and
/// cell (`008ad1c0`).
fn move_actor(e: &mut Engine, this: Ptr<VirtualActorPathHandler>, point: u32, angle: u32) {
    let actor = e.get(this, VirtualActorPathHandler::pActor);
    let world_space = e.call(GET_WORLD_SPACE, &args![actor]).u32();
    let actor = e.get(this, VirtualActorPathHandler::pActor);
    let cell = e.call(PARENT_CELL, &args![actor]).u32();
    let actor = e.get(this, VirtualActorPathHandler::pActor);
    e.call(
        MOVE_ACTOR,
        &args![
            actor,
            point,
            x87_quiet_bits(angle),
            cell,
            world_space,
            1u32,
            0u32
        ],
    );
}

/// Whether the actor has no process level and the player is not sleeping
/// or resting (the update's stop test).
fn actor_is_inactive(e: &mut Engine, this: Ptr<VirtualActorPathHandler>) -> bool {
    let actor = e.get(this, VirtualActorPathHandler::pActor);
    if e.call(DESIRED_PROCESS_LEVEL, &args![actor]).u32() != 0 {
        return false;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    !e.call(IS_SLEEPING_OR_RESTING, &args![player]).bool()
}

fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 009ea8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::Update` (Xbox PDB; `RET 4`): moves the actor
/// along the path without simulating it. Answers 0 when the actor's saved
/// acquire object is busy (its slot +0x4bc). The time step is `elapsed`, or
/// `3600 / time scale` when the player sleeps or rests or the actor follows
/// the player, or, unless `00493bb0` holds, `WAIT_STEP / time scale` and,
/// with an acquire object, `(ACQUIRE_TIME - 007df1f0) * 3600 / time scale`.
/// Answers 5 when the node index is past the path. Otherwise the actor walks
/// `speed * step` from its position: whole segments to the next nodes (each
/// adding to the distance traveled, turning the actor along the segment,
/// and either moving it (`008ad1c0`) and going to the next node, or for a
/// door node using the door, `009ead90`: answer 3 when that fails with -1 or
/// -2, 0 after a move when it answers 1), answering 5 when the path ends;
/// the rest of a segment is walked by scaling it, moving the actor, and
/// answering 5 (moving to the next node) when the last node is within the
/// target radius, otherwise 0. A stop test (no process level and the player
/// not sleeping) answers 0 after each move.
pub fn virtual_actor_path_handler_update(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    elapsed: f32,
) -> u32 {
    let mut step = x87_quiet_bits(elapsed.to_bits());
    let actor = e.get(this, VirtualActorPathHandler::pActor);
    let acquire = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
    if acquire != 0 && e.vcall(acquire, 0x4bc, &[]).u32() != 0 {
        return 0;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    let mut hour_step = e.call(IS_SLEEPING_OR_RESTING, &args![player]).bool();
    if !hour_step {
        let player = e.global::<u32>(PLAYER_POINTER);
        let actor = e.get(this, VirtualActorPathHandler::pActor);
        hour_step = e.call(IS_FOLLOWING, &args![actor, player]).bool();
    }
    if hour_step {
        let time_scale = e.call(GET_TIME_SCALE, &args![CALENDAR]).f64();
        step = x87_store32(x87_div(e.global::<f64>(SECONDS_PER_HOUR), time_scale));
    } else {
        let actor = e.get(this, VirtualActorPathHandler::pActor);
        if !e.call(KEEPS_TIME_STEP, &args![actor]).bool() {
            let time_scale = e.call(GET_TIME_SCALE, &args![CALENDAR]).f64();
            step = x87_store32(x87_div(e.global::<f64>(WAIT_STEP), time_scale));
            let actor = e.get(this, VirtualActorPathHandler::pActor);
            let acquire = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
            if acquire != 0 {
                let value = e.call(ACQUIRE_VALUE, &args![acquire]).f64();
                let left = x87_sub(x87_m32(e.global::<u32>(ACQUIRE_TIME)), value);
                let seconds = x87_mul(left, e.global::<f64>(SECONDS_PER_HOUR));
                let time_scale = e.call(GET_TIME_SCALE, &args![CALENDAR]).f64();
                step = x87_store32(x87_div(seconds, time_scale));
            }
        }
    }
    if e.get(this, VirtualActorPathHandler::iCurrentNodeIndex) >= path_node_count(e, this) {
        return 5;
    }
    e.with_stack(0x3c, |e, locals| {
        let position = locals.addr();
        let direction = position + 0xc;
        let next = position + 0x18;
        let node_point = position + 0x24;
        let difference = position + 0x30;
        let actor = e.get(this, VirtualActorPathHandler::pActor);
        let current = e.vcall(actor, 0x1f4, &[]).u32();
        copy_point(e, position, current);
        for i in 0..3 {
            let word = e.global::<u32>(ZERO_POINT + 4 * i);
            e.mem.set_u32(direction + 4 * i, word);
        }
        e.call(0x0068_15c0, &args![next]);
        let actor = e.get(this, VirtualActorPathHandler::pActor);
        let speed = e.call(CURRENT_SPEED, &args![actor]).f64();
        let mut remaining = x87_store32(x87_mul(speed, x87_m32(step)));
        let mut segment = 0f32.to_bits();
        loop {
            let left = x87_load32(remaining);
            let zero = e.global::<f64>(ZERO);
            if left < zero || left.is_nan() || zero.is_nan() {
                break;
            }
            let node = current_node(e, this);
            let part = e.call(NODE_PART, &args![node]).u32();
            let point = e.call(NODE_POINT, &args![part, node_point]).u32();
            copy_point(e, next, point);
            let delta = e
                .call(POINT_SUBTRACT, &args![next, difference, position])
                .u32();
            copy_point(e, direction, delta);
            segment = x87_store32(e.call(POINT_LENGTH, &args![direction]).f64());
            if !(x87_load32(segment) <= x87_load32(remaining)) {
                break;
            }
            let traveled = e.mem.u32(this.addr() + 0x18);
            let traveled = x87_add(x87_load32(traveled), x87_m32(segment));
            e.mem.set_u32(this.addr() + 0x18, x87_store32(traveled));
            remaining = x87_store32(x87_sub(x87_load32(remaining), x87_m32(segment)));
            copy_point(e, position, next);
            let angle = x87_store32(e.call(Z_ANGLE, &args![direction]).f64());
            let actor = e.get(this, VirtualActorPathHandler::pActor);
            e.vcall(actor, 0x2c4, &args![x87_quiet_bits(angle)]);
            let node = current_node(e, this);
            if e.call(NODE_IS_DOOR, &args![node]).bool() {
                let used = virtual_actor_path_handler_use_door_at_node(e, this, position);
                if used >= -2 {
                    if used < 0 {
                        return 3;
                    }
                    if used == 1 {
                        move_actor(e, this, position, angle);
                        return 0;
                    }
                }
            } else {
                move_actor(e, this, position, angle);
                if actor_is_inactive(e, this) {
                    return 0;
                }
                let index = e.get(this, VirtualActorPathHandler::iCurrentNodeIndex);
                e.set(
                    this,
                    VirtualActorPathHandler::iCurrentNodeIndex,
                    index.wrapping_add(1),
                );
            }
            if e.get(this, VirtualActorPathHandler::iCurrentNodeIndex) >= path_node_count(e, this) {
                return 5;
            }
        }
        // The part of a segment that is left.
        let length = x87_load32(segment);
        let fraction = if length == e.global::<f64>(ZERO) {
            0.0
        } else {
            let walked = x87_div(x87_sub(length, x87_m32(remaining)), x87_m32(segment));
            x87_sub(1.0, walked)
        };
        let fraction = x87_store32(fraction);
        e.call(POINT_SCALE, &args![direction, x87_quiet_bits(fraction)]);
        e.call(POINT_ADD, &args![position, direction]);
        let walked = e.call(POINT_LENGTH, &args![direction]).f64();
        let traveled = e.mem.u32(this.addr() + 0x18);
        e.mem.set_u32(
            this.addr() + 0x18,
            x87_store32(x87_add(walked, x87_m32(traveled))),
        );
        let angle = x87_store32(e.call(Z_ANGLE, &args![direction]).f64());
        let actor = e.get(this, VirtualActorPathHandler::pActor);
        e.vcall(actor, 0x2c4, &args![x87_quiet_bits(angle)]);
        move_actor(e, this, position, angle);
        if actor_is_inactive(e, this) {
            return 0;
        }
        if e.call(WORD_AT, &args![this.addr() + 0xc]).u32() != 0 {
            let next_index = e
                .get(this, VirtualActorPathHandler::iCurrentNodeIndex)
                .wrapping_add(1);
            if next_index == path_node_count(e, this) {
                let request = e.call(WORD_AT, &args![this.addr() + 0x10]).u32();
                let radius = e.call(TARGET_RADIUS, &args![request]).f64();
                let reach = x87_add(radius, x87_m32(remaining));
                if x87_load32(segment) <= reach {
                    let index = e.get(this, VirtualActorPathHandler::iCurrentNodeIndex);
                    e.set(
                        this,
                        VirtualActorPathHandler::iCurrentNodeIndex,
                        index.wrapping_add(1),
                    );
                    return 5;
                }
            }
        }
        0
    })
}

// Translated from 009ead90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::UseDoorAtNode` (Xbox PDB; `RET 4`): puts the
/// actor at `point` (through the door manager, `0087b650` and `0087b190`,
/// when `008c7aa0` says so, otherwise `00575830`), takes the current node's
/// door (`0055b980`) and moves to the next node. Answers -1 when there is a
/// door and activating it (`00573170`) fails. When the path goes on and the
/// actor's position is farther than the tolerance from the next node, the
/// actor was not moved where expected: when the point differs from the
/// position (`00439090`) a warning names the actor, the door and the door's
/// cell (`005b5e40`), and the answer is -2. Otherwise `point` becomes the
/// actor's position, and the answer is 1 (moving to the next node and
/// clearing the distance traveled) when the stop test holds, else 0.
pub fn virtual_actor_path_handler_use_door_at_node(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    point: u32,
) -> i32 {
    if e.call(DOORS_TELEPORT, &[]).bool() {
        let actor = e.get(this, VirtualActorPathHandler::pActor);
        let manager = e.call(DOOR_MANAGER, &[]).u32();
        e.call(DOOR_MOVE, &args![manager, actor, point]);
        let actor = e.get(this, VirtualActorPathHandler::pActor);
        let manager = e.call(DOOR_MANAGER, &[]).u32();
        e.call(DOOR_FINISH, &args![manager, actor]);
    } else {
        let actor = e.get(this, VirtualActorPathHandler::pActor);
        e.call(SET_LOCATION, &args![actor, point]);
    }
    let node = current_node(e, this);
    let door = e.call(NODE_DOOR, &args![node]).u32();
    let index = e.get(this, VirtualActorPathHandler::iCurrentNodeIndex);
    e.set(
        this,
        VirtualActorPathHandler::iCurrentNodeIndex,
        index.wrapping_add(1),
    );
    if door != 0 {
        let actor = e.get(this, VirtualActorPathHandler::pActor);
        if !e
            .call(ACTIVATE, &args![door, actor, 0u32, 0u32, 1u32])
            .bool()
        {
            return -1;
        }
    }
    if e.call(WORD_AT, &args![this.addr() + 0xc]).u32() != 0
        && e.get(this, VirtualActorPathHandler::iCurrentNodeIndex) < path_node_count(e, this)
    {
        let far = e.with_stack(0x30, |e, locals| {
            let position = locals.addr();
            let node_point = position + 0xc;
            let difference = position + 0x18;
            let actor = e.get(this, VirtualActorPathHandler::pActor);
            let current = e.vcall(actor, 0x1f4, &[]).u32();
            copy_point(e, position, current);
            let node = current_node(e, this);
            let part = e.call(NODE_PART, &args![node]).u32();
            let next = e.call(NODE_POINT, &args![part, node_point]).u32();
            let delta = e
                .call(POINT_SUBTRACT, &args![position, difference, next])
                .u32();
            let distance = x87_store32(e.call(POINT_LENGTH_2, &args![delta]).f64());
            if x87_load32(distance) > e.global::<f64>(DOOR_ARRIVAL_TOLERANCE) {
                if e.call(POINTS_DIFFER, &args![point, position]).bool() {
                    let cell = e.call(PARENT_CELL, &args![door]).u32();
                    let cell_name = if cell != 0 {
                        let cell = e.call(PARENT_CELL, &args![door]).u32();
                        if e.call(NAME_TEST, &args![cell]).u32() != 0 {
                            let cell = e.call(PARENT_CELL, &args![door]).u32();
                            e.call(LOCATION_NAME, &args![cell.wrapping_add(0x18)]).u32()
                        } else {
                            let cell = e.call(PARENT_CELL, &args![door]).u32();
                            e.vcall(cell, 0x130, &[]).u32()
                        }
                    } else {
                        NO_CELL_TEXT
                    };
                    let actor = e.get(this, VirtualActorPathHandler::pActor);
                    let actor_name = if e.call(NAME_TEST, &args![actor]).u32() != 0 {
                        let actor = e.get(this, VirtualActorPathHandler::pActor);
                        e.vcall(actor, 0x130, &[]).u32()
                    } else {
                        let actor = e.get(this, VirtualActorPathHandler::pActor);
                        e.call(REFERENCE_NAME, &args![actor]).u32()
                    };
                    let door_name = e.vcall(door, 0x130, &[]).u32();
                    e.call(
                        DEBUG_PRINT,
                        &args![DOOR_WARNING, actor_name, door_name, cell_name],
                    );
                }
                true
            } else {
                false
            }
        });
        if far {
            return -2;
        }
    }
    let actor = e.get(this, VirtualActorPathHandler::pActor);
    let current = e.vcall(actor, 0x1f4, &[]).u32();
    copy_point(e, point, current);
    if actor_is_inactive(e, this) {
        let index = e.get(this, VirtualActorPathHandler::iCurrentNodeIndex);
        e.set(
            this,
            VirtualActorPathHandler::iCurrentNodeIndex,
            index.wrapping_add(1),
        );
        e.set(this, VirtualActorPathHandler::fDistanceTraveled, 0.0);
        return 1;
    }
    0
}

// Translated from 009eb040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::ClearMovement` (Xbox PDB): clears `spPath` and
/// `spRequest` (`0066b0d0` with null) and the node index.
pub fn virtual_actor_path_handler_clear_movement(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
) {
    e.call(SMART_POINTER_SET, &args![this.addr() + 0xc, 0u32]);
    e.call(SMART_POINTER_SET, &args![this.addr() + 0x10, 0u32]);
    e.set(this, VirtualActorPathHandler::iCurrentNodeIndex, 0);
}

// Translated from 009eb390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::SaveGame` (Xbox PDB): the node index, the
/// distance traveled and the path length, four bytes each (`00865e50`).
pub fn virtual_actor_path_handler_save_game(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    buffer: u32,
) {
    for offset in [0x14u32, 0x18, 0x1c] {
        e.call(SAVE_DATA, &args![buffer, this.addr() + offset, 4u32, 0u32]);
    }
}

/// `00404010` (unnamed, `cdecl`, two `float`s; the larger in `ST0`).
const LARGER_OF: u32 = 0x0040_4010;
/// `008b6800` (unnamed, engine map unit `actor.cpp`, `thiscall` on the path, no stack words):
/// compared unsigned with 0.
const PATH_COUNT_008B6800: u32 = 0x008b_6800;
/// `009ddb30` (unnamed, engine map unit `actormover.cpp`, `thiscall` on the path, no stack
/// words): the node used past the path's last index.
const PATH_LAST_NODE_009DDB30: u32 = 0x009d_db30;
/// The actor's vtable slot `+0x1f4` (no stack words): the address of its position.
const ACTOR_POSITION_SLOT_1F4: u32 = 0x1f4;
/// `BGSLoadGameBuffer` raw load (`00864980`, `thiscall`: data, size).
const LOAD_DATA: u32 = 0x0086_4980;
/// The printer's slot `+0x14` (`cdecl`: printer, two words passed on, format, values).
const PRINT_SLOT: u32 = 0x14;
/// `"VirtualPathHandler: Actor's Positon %1.f..."` (the three coordinates).
const STATUS_POSITION_FORMAT: u32 = 0x0109_2b28;
/// `"Current path node %u of %u"`.
const STATUS_NODE_FORMAT: u32 = 0x0109_2b0c;
/// `"No path"`.
const STATUS_NO_PATH_FORMAT: u32 = 0x0109_2b04;

/// The actor's position (slot `+0x1f4`) copied to `out`.
fn copy_actor_position(e: &mut Engine, this: Ptr<VirtualActorPathHandler>, out: u32) {
    let actor = e.get(this, VirtualActorPathHandler::pActor);
    let position = e.vcall(actor, ACTOR_POSITION_SLOT_1F4, &args![]).u32();
    copy_point(e, out, position);
}

// Translated from 009ead60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::UpdateWhilePlayerWaits` (Xbox PDB; `thiscall`, one `float`,
/// `RET 4`): the handler's slot `+0xc` (`Update`) with the time (`FLD`/`FSTP` copy); its `EAX`
/// is the answer.
pub fn virtual_actor_path_handler_update_while_player_waits(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    time: f32,
) -> u32 {
    e.vcall(this.addr(), 0xc, &args![x87_quiet_bits(time.to_bits())])
        .u32()
}

// Translated from 009eb070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::GetDistanceRemaining` (Xbox PDB): `fPathLength -
/// fDistanceTraveled` (stored as a `float`), and the larger of it and 0.0 (`00404010`),
/// stored as a `float` and loaded into `ST0`.
pub fn virtual_actor_path_handler_get_distance_remaining(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
) -> f32 {
    let length = e.mem.u32(this.addr() + 0x1c);
    let traveled = e.mem.u32(this.addr() + 0x18);
    let remaining = x87_store32(x87_sub(x87_load32(length), x87_m32(traveled)));
    let larger = e
        .call(
            LARGER_OF,
            &args![x87_quiet_bits(remaining), x87_store32(0.0)],
        )
        .f64();
    x87_quiet(f32::from_bits(x87_store32(larger)))
}

// Translated from 009eb0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::GetPercentTraveled` (Xbox PDB): 0.0 without a path; 1.0 when the
/// node index is at or past the node count, or when `fPathLength` is below or equal to 0.0
/// (`FCOMP`, `TEST AH,0x41`, `JP`: greater and unordered divide); otherwise
/// `fDistanceTraveled / fPathLength`, stored as a `float`.
pub fn virtual_actor_path_handler_get_percent_traveled(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
) -> f32 {
    if e.call(WORD_AT, &args![this.addr() + 0xc]).u32() == 0 {
        return 0.0;
    }
    let count = path_node_count(e, this);
    if e.get(this, VirtualActorPathHandler::iCurrentNodeIndex) >= count {
        return 1.0;
    }
    let length = e.mem.u32(this.addr() + 0x1c);
    let zero = e.global::<f64>(ZERO);
    let length_value = x87_load32(length);
    let divides = length_value > zero || length_value.is_nan();
    if !divides {
        return 1.0;
    }
    let traveled = e.mem.u32(this.addr() + 0x18);
    let percent = x87_store32(x87_div(x87_load32(traveled), x87_m32(length)));
    x87_quiet(f32::from_bits(percent))
}

// Translated from 009eb120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::ComputeApproximateLocation` (Xbox PDB; `thiscall`, two stack
/// words, `RET 8`; the second is not read): the actor's position (slot `+0x1f4`) copied to
/// `out`; answers `out`.
pub fn virtual_actor_path_handler_compute_approximate_location(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    out: u32,
    _unused_2: u32,
) -> u32 {
    copy_actor_position(e, this, out);
    out
}

// Translated from 009eb160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::GetNextNodeLocation` (Xbox PDB; `thiscall`, one stack word,
/// `RET 4`): when `008b6800` of the path answers 0, the actor's position; otherwise the point
/// (`005c3420`) of the part (`00717e50`) of the node after the current one, or, when that index
/// is not below the node count, of `009ddb30`'s node. Written to `out`; answers `out`.
pub fn virtual_actor_path_handler_get_next_node_location(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    out: u32,
) -> u32 {
    let path = e.call(WORD_AT, &args![this.addr() + 0xc]).u32();
    if e.call(PATH_COUNT_008B6800, &args![path]).u32() == 0 {
        copy_actor_position(e, this, out);
        return out;
    }
    let next = e
        .get(this, VirtualActorPathHandler::iCurrentNodeIndex)
        .wrapping_add(1);
    let node = if next < path_node_count(e, this) {
        let next = e
            .get(this, VirtualActorPathHandler::iCurrentNodeIndex)
            .wrapping_add(1);
        let path = e.call(WORD_AT, &args![this.addr() + 0xc]).u32();
        e.call(PATH_NODE, &args![path, next]).u32()
    } else {
        let path = e.call(WORD_AT, &args![this.addr() + 0xc]).u32();
        e.call(PATH_LAST_NODE_009DDB30, &args![path]).u32()
    };
    let part = e.call(NODE_PART, &args![node]).u32();
    e.call(NODE_POINT, &args![part, out]);
    out
}

// Translated from 009eb230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::GetNodeLocation` (Xbox PDB; `thiscall`, two stack words: index
/// and out, `RET 8`): `false` without a path or when `index` is not below the node count;
/// otherwise the point of the node's part (`005c3420` into a local, through the address it
/// answers) copied to `out`, and `true`.
pub fn virtual_actor_path_handler_get_node_location(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    index: u32,
    out: u32,
) -> bool {
    if e.call(WORD_AT, &args![this.addr() + 0xc]).u32() == 0 {
        return false;
    }
    if index >= path_node_count(e, this) {
        return false;
    }
    e.with_stack(0xc, |e, local| {
        let path = e.call(WORD_AT, &args![this.addr() + 0xc]).u32();
        let node = e.call(PATH_NODE, &args![path, index]).u32();
        let part = e.call(NODE_PART, &args![node]).u32();
        let point = e.call(NODE_POINT, &args![part, local]).u32();
        copy_point(e, out, point);
    });
    true
}

// Translated from 009eb2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::PrintCurrentStatus` (Xbox PDB; `thiscall`, three stack words:
/// the printer and two words passed on, `RET 0xc`): through the printer's slot `+0x14`
/// (`cdecl`), the actor's position (slot `+0x1f4`, each coordinate as a `double`) with
/// `01092b28`'s format, then `"Current path node %u of %u"` with the node index and count,
/// or `"No path"` without a path.
pub fn virtual_actor_path_handler_print_current_status(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    printer: u32,
    first: u32,
    second: u32,
) {
    let actor = e.get(this, VirtualActorPathHandler::pActor);
    let position = e.vcall(actor, ACTOR_POSITION_SLOT_1F4, &args![]).u32();
    let x = e.mem.u32(position);
    let y = e.mem.u32(position + 4);
    let z = e.mem.u32(position + 8);
    e.vcall(
        printer,
        PRINT_SLOT,
        &args![
            first,
            second,
            STATUS_POSITION_FORMAT,
            x87_load32(x),
            x87_load32(y),
            x87_load32(z)
        ],
    );
    if e.call(WORD_AT, &args![this.addr() + 0xc]).u32() != 0 {
        let count = path_node_count(e, this);
        let index = e.get(this, VirtualActorPathHandler::iCurrentNodeIndex);
        e.vcall(
            printer,
            PRINT_SLOT,
            &args![first, second, STATUS_NODE_FORMAT, index, count],
        );
    } else {
        e.vcall(
            printer,
            PRINT_SLOT,
            &args![first, second, STATUS_NO_PATH_FORMAT],
        );
    }
}

// Translated from 009eb3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VirtualActorPathHandler::LoadGame` (Xbox PDB; `RET 4`): the node index, the distance
/// traveled and the path length, four bytes each (`00864980`).
pub fn virtual_actor_path_handler_load_game(
    e: &mut Engine,
    this: Ptr<VirtualActorPathHandler>,
    buffer: u32,
) {
    for offset in [0x14u32, 0x18, 0x1c] {
        e.call(LOAD_DATA, &args![buffer, this.addr() + offset, 4u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009e_ad60,
            virtual_actor_path_handler_update_while_player_waits(
                Ptr<VirtualActorPathHandler>,
                f32,
            ) -> u32
        ),
        entry!(
            0x009e_b070,
            virtual_actor_path_handler_get_distance_remaining(Ptr<VirtualActorPathHandler>) -> f32
        ),
        entry!(
            0x009e_b0b0,
            virtual_actor_path_handler_get_percent_traveled(Ptr<VirtualActorPathHandler>) -> f32
        ),
        entry!(
            0x009e_b120,
            virtual_actor_path_handler_compute_approximate_location(
                Ptr<VirtualActorPathHandler>,
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x009e_b160,
            virtual_actor_path_handler_get_next_node_location(
                Ptr<VirtualActorPathHandler>,
                u32,
            ) -> u32
        ),
        entry!(
            0x009e_b230,
            virtual_actor_path_handler_get_node_location(
                Ptr<VirtualActorPathHandler>,
                u32,
                u32,
            ) -> bool
        ),
        entry!(
            0x009e_b2b0,
            virtual_actor_path_handler_print_current_status(
                Ptr<VirtualActorPathHandler>,
                u32,
                u32,
                u32,
            )
        ),
        entry!(
            0x009e_b3e0,
            virtual_actor_path_handler_load_game(Ptr<VirtualActorPathHandler>, u32)
        ),
        entry!(
            0x004a_7bd0,
            virtual_actor_path_handler_get_distance_traveled(Ptr<VirtualActorPathHandler>) -> f32
        ),
        entry!(
            0x009e_a710,
            virtual_actor_path_handler_get_move_delta(Ptr<VirtualActorPathHandler>, u32) -> bool
        ),
        entry!(
            0x009e_a740,
            virtual_actor_path_handler_scalar_deleting_destructor(
                Ptr<VirtualActorPathHandler>,
                u32,
            )
                -> Ptr<VirtualActorPathHandler>
        ),
        entry!(
            0x009e_a770,
            fn_009ea770(
                Ptr<VirtualActorPathHandler>,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<VirtualActorPathHandler>
        ),
        entry!(0x009e_a7c0, fn_009ea7c0(Ptr<VirtualActorPathHandler>)),
        entry!(
            0x009e_a820,
            virtual_actor_path_handler_init(Ptr<VirtualActorPathHandler>, f32, f32) -> u32
        ),
        entry!(
            0x009e_a8a0,
            virtual_actor_path_handler_update(Ptr<VirtualActorPathHandler>, f32) -> u32
        ),
        entry!(
            0x009e_ad90,
            virtual_actor_path_handler_use_door_at_node(Ptr<VirtualActorPathHandler>, u32) -> i32
        ),
        entry!(
            0x009e_b040,
            virtual_actor_path_handler_clear_movement(Ptr<VirtualActorPathHandler>)
        ),
        entry!(
            0x009e_b390,
            virtual_actor_path_handler_save_game(Ptr<VirtualActorPathHandler>, u32)
        ),
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

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    const ACTOR_VTABLE: u32 = 0x0300_0000;
    const ACTOR_POSITION_SLOT: u32 = 0x0300_1000;
    const ACTOR_TURN_SLOT: u32 = 0x0300_2000;
    const ACTOR_NAME_SLOT: u32 = 0x0300_3000;

    /// A handler with an actor standing at the origin (slot +0x1f4), a path
    /// of `nodes` nodes along x, ten units apart, and the doubles the
    /// update and the door use.
    fn engine(nodes: u32) -> (Engine, Ptr<VirtualActorPathHandler>, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(SECONDS_PER_HOUR, 3600.0f64);
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLAYER_POINTER, 0x0aaa_0000u32);
        e.map(0x011f_4000, 0x1000);
        e.map(0x0102_f000, 0x1000);
        e.set_global(DOOR_ARRIVAL_TOLERANCE, 100.0f64);
        e.put_vtable(ACTOR_VTABLE, &[0; 0x130]);
        e.mem.set_u32(ACTOR_VTABLE + 0x1f4, ACTOR_POSITION_SLOT);
        e.mem.set_u32(ACTOR_VTABLE + 0x2c4, ACTOR_TURN_SLOT);
        e.mem.set_u32(ACTOR_VTABLE + 0x130, ACTOR_NAME_SLOT);
        let actor = e.mem.alloc(0x20);
        e.mem.set_u32(actor, ACTOR_VTABLE);
        e.register(ACTOR_POSITION_SLOT, |_, a| word(a[0] + 0x10));
        e.register(ACTOR_TURN_SLOT, nothing);
        e.register(ACTOR_NAME_SLOT, |_, a| word(a[0] + 0x1000));
        let this: Ptr<VirtualActorPathHandler> = e.new_object();
        e.set(this, VirtualActorPathHandler::pActor, actor);
        let path = e.mem.alloc(4);
        e.mem.set_u32(this.addr() + 0xc, path);
        let request = e.mem.alloc(4);
        e.mem.set_u32(this.addr() + 0x10, request);
        e.register(WORD_AT, |e, a| word(e.mem.u32(a[0])));
        e.register_double(PATH_NODE_COUNT, move |_, _| word(nodes));
        e.register(PATH_NODE, |_, a| word(0x100 + a[1]));
        e.register(NODE_PART, |_, a| word(a[0]));
        e.register(NODE_POINT, |e, a| {
            e.mem.set_f32(a[1], 10.0 * (a[0] - 0xff) as f32);
            e.mem.set_f32(a[1] + 4, 0.0);
            e.mem.set_f32(a[1] + 8, 0.0);
            word(a[1])
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            word(a[1])
        });
        e.register(POINT_LENGTH, |e, a| float(f64::from(e.mem.f32(a[0]).abs())));
        e.register(POINT_LENGTH_2, |e, a| {
            float(f64::from(e.mem.f32(a[0]).abs()))
        });
        e.register(POINT_SCALE, |e, a| {
            let factor = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * factor;
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        e.register(POINT_ADD, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        e.register(Z_ANGLE, |_, _| float(0.5));
        e.register(NODE_IS_DOOR, |_, _| word(0));
        e.register(GET_WORLD_SPACE, |_, _| word(0x33));
        e.register(PARENT_CELL, |_, _| word(0x44));
        e.register(MOVE_ACTOR, nothing);
        e.register(DESIRED_PROCESS_LEVEL, |_, _| word(1));
        e.register(IS_SLEEPING_OR_RESTING, |_, _| word(0));
        e.register(IS_FOLLOWING, |_, _| word(0));
        e.register(KEEPS_TIME_STEP, |_, _| word(1));
        e.register(SAVED_ACQUIRE_OBJECT, |_, _| word(0));
        e.register(CURRENT_SPEED, |_, _| float(25.0));
        e.register(0x0068_15c0, |_, a| word(a[0]));
        e.register(TARGET_RADIUS, |_, _| float(0.0));
        (e, this, actor)
    }

    #[test]
    fn get_distance_traveled_reads_the_field() {
        let (mut e, this, _) = engine(1);
        e.set(this, VirtualActorPathHandler::fDistanceTraveled, 12.5);
        assert_eq!(e.call(0x004a_7bd0, &args![this]).f64(), 12.5);
    }

    #[test]
    fn get_move_delta_writes_zero_and_answers_false() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.set_global(ZERO_POINT, 1.0f32);
        let delta = e.mem.alloc(0xc);
        e.mem.set_u32(delta + 4, 7);
        assert!(!e.call(0x009e_a710, &args![0u32, delta]).bool());
        assert_eq!(e.mem.f32(delta), 1.0);
        assert_eq!(e.mem.u32(delta + 4), 0);
    }

    fn destructor_doubles(e: &mut Engine) {
        e.register(SMART_POINTER_SET, nothing);
        e.register(BASE_DESTRUCT, nothing);
        e.register(OPERATOR_DELETE, nothing);
    }

    #[test]
    fn destructor_body_clears_then_runs_the_base() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 4);
        e.call_log = Some(vec![]);
        e.call(0x009e_a7c0, &args![this]);
        assert_eq!(e.mem.u32(this), VIRTUAL_ACTOR_PATH_HANDLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(
            calls_to(&e, SMART_POINTER_SET),
            vec![vec![this + 0xc, 0], vec![this + 0x10, 0]]
        );
        assert_eq!(calls_to(&e, BASE_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_zero() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_a740, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x009e_a740, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn constructor_runs_the_base_and_clears_the_floats() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, |_, a| word(a[0]));
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, 0x1234);
        e.mem.set_u32(this + 0x1c, 0x1234);
        e.call_log = Some(vec![]);
        let r = e.call(0x009e_a770, &args![this, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(r.u32(), this);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![this, 1, 2, 3, 4]]);
        assert_eq!(e.mem.u32(this), VIRTUAL_ACTOR_PATH_HANDLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
    }

    #[test]
    fn init_truncates_the_start_and_starts_at_zero() {
        let (mut e, this, _) = engine(4);
        e.register(PATH_LENGTH, |_, _| float(42.0));
        e.put_vtable(0x0310_0000, &[0, 0, 0, 0x0310_1000]);
        e.mem.set_u32(this.addr(), 0x0310_0000);
        e.register(0x0310_1000, |_, a| word(a[1]));
        e.set(this, VirtualActorPathHandler::fDistanceTraveled, 3.0);
        // A start of 2.9 is node 2 and answers 0.
        assert_eq!(e.call(0x009e_a820, &args![this, 2.9f32, 7.0f32]).u32(), 0);
        assert_eq!(e.get(this, VirtualActorPathHandler::iCurrentNodeIndex), 2);
        assert_eq!(e.get(this, VirtualActorPathHandler::fDistanceTraveled), 0.0);
        assert_eq!(e.get(this, VirtualActorPathHandler::fPathLength), 42.0);
        // A start of 0 answers slot +0xc with the time.
        assert_eq!(
            e.call(0x009e_a820, &args![this, 0.0f32, 7.0f32]).u32(),
            7f32.to_bits()
        );
        // -1.5 truncates to -1.
        e.call(0x009e_a820, &args![this, -1.5f32, 7.0f32]);
        assert_eq!(
            e.get(this, VirtualActorPathHandler::iCurrentNodeIndex),
            0xffff_ffff
        );
    }

    #[test]
    fn clear_movement_clears_both_pointers_and_the_index() {
        let mut e = Engine::new();
        e.register(SMART_POINTER_SET, nothing);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 9);
        e.call_log = Some(vec![]);
        e.call(0x009e_b040, &args![this]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(calls_to(&e, SMART_POINTER_SET).len(), 2);
    }

    #[test]
    fn save_game_saves_three_words() {
        let mut e = Engine::new();
        e.register(SAVE_DATA, nothing);
        e.call_log = Some(vec![]);
        e.call(0x009e_b390, &args![0x5000u32, 0x9000u32]);
        assert_eq!(
            calls_to(&e, SAVE_DATA),
            vec![
                vec![0x9000, 0x5014, 4, 0],
                vec![0x9000, 0x5018, 4, 0],
                vec![0x9000, 0x501c, 4, 0]
            ]
        );
    }

    #[test]
    fn update_answers_0_when_the_acquire_object_is_busy() {
        let (mut e, this, _) = engine(3);
        let acquire = e.mem.alloc(4);
        e.put_vtable(0x0320_0000, &[0; 0x130]);
        e.mem.set_u32(0x0320_0000 + 0x4bc, 0x0320_1000);
        e.mem.set_u32(acquire, 0x0320_0000);
        e.register(0x0320_1000, |_, _| word(1));
        e.register_double(SAVED_ACQUIRE_OBJECT, move |_, _| word(acquire));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_a8a0, &args![this, 1.0f32]).u32(), 0);
        assert!(calls_to(&e, CURRENT_SPEED).is_empty());
    }

    #[test]
    fn update_answers_5_past_the_last_node() {
        let (mut e, this, _) = engine(3);
        e.set(this, VirtualActorPathHandler::iCurrentNodeIndex, 3);
        assert_eq!(e.call(0x009e_a8a0, &args![this, 1.0f32]).u32(), 5);
    }

    #[test]
    fn update_walks_whole_segments_then_part_of_the_next() {
        let (mut e, this, actor) = engine(3);
        // The move double keeps the x it was given at actor + 0x1c.
        e.register(MOVE_ACTOR, |e, a| {
            let x = e.mem.f32(a[1]);
            e.mem.set_f32(a[0] + 0x1c, x);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        // 25 units: nodes at 10 and 20, then half of the segment to 30.
        assert_eq!(e.call(0x009e_a8a0, &args![this, 1.0f32]).u32(), 0);
        assert_eq!(e.get(this, VirtualActorPathHandler::iCurrentNodeIndex), 2);
        assert_eq!(
            e.get(this, VirtualActorPathHandler::fDistanceTraveled),
            25.0
        );
        let moves = calls_to(&e, MOVE_ACTOR);
        assert_eq!(moves.len(), 3);
        assert_eq!(moves[0][0], actor);
        assert_eq!(&moves[0][2..], &[0.5f32.to_bits(), 0x44, 0x33, 1, 0]);
        // The last move puts the actor at 25.
        assert_eq!(e.mem.f32(actor + 0x1c), 25.0);
        assert_eq!(calls_to(&e, ACTOR_TURN_SLOT).len(), 3);
        assert_eq!(calls_to(&e, POINT_SCALE)[0][1], 0.5f32.to_bits());
    }

    #[test]
    fn update_reaches_the_goal_within_the_target_radius() {
        let (mut e, this, _) = engine(3);
        e.register(TARGET_RADIUS, |_, _| float(6.0));
        assert_eq!(e.call(0x009e_a8a0, &args![this, 1.0f32]).u32(), 5);
        assert_eq!(e.get(this, VirtualActorPathHandler::iCurrentNodeIndex), 3);
    }

    #[test]
    fn update_uses_an_hour_step_while_the_player_rests() {
        let (mut e, this, _) = engine(3);
        e.register(IS_SLEEPING_OR_RESTING, |_, _| word(1));
        e.register(GET_TIME_SCALE, |_, _| float(3600.0));
        e.register(CURRENT_SPEED, |_, _| float(5.0));
        // A step of 1 second at speed 5: half of the first segment.
        assert_eq!(e.call(0x009e_a8a0, &args![this, 100.0f32]).u32(), 0);
        assert_eq!(e.get(this, VirtualActorPathHandler::fDistanceTraveled), 5.0);
        assert_eq!(e.get(this, VirtualActorPathHandler::iCurrentNodeIndex), 0);
    }

    #[test]
    fn update_answers_3_when_a_door_cannot_be_used() {
        let (mut e, this, _) = engine(3);
        e.register(NODE_IS_DOOR, |_, a| word(u32::from(a[0] == 0x100)));
        e.register(DOORS_TELEPORT, |_, _| word(0));
        e.register(SET_LOCATION, nothing);
        e.register(NODE_DOOR, |_, _| word(0x700));
        e.register(ACTIVATE, |_, _| word(0));
        assert_eq!(e.call(0x009e_a8a0, &args![this, 1.0f32]).u32(), 3);
        assert_eq!(e.get(this, VirtualActorPathHandler::iCurrentNodeIndex), 1);
    }

    #[test]
    fn use_door_through_the_manager_fails_when_activation_fails() {
        let (mut e, this, actor) = engine(3);
        e.register(DOORS_TELEPORT, |_, _| word(1));
        e.register(DOOR_MANAGER, |_, _| word(0x0bbb_0000));
        e.register(DOOR_MOVE, nothing);
        e.register(DOOR_FINISH, nothing);
        e.register(NODE_DOOR, |_, _| word(0x700));
        e.register(ACTIVATE, |_, _| word(0));
        let point = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_ad90, &args![this, point]).i32(), -1);
        assert_eq!(
            calls_to(&e, DOOR_MOVE),
            vec![vec![0x0bbb_0000, actor, point]]
        );
        assert_eq!(calls_to(&e, DOOR_FINISH), vec![vec![0x0bbb_0000, actor]]);
        assert_eq!(calls_to(&e, ACTIVATE), vec![vec![0x700, actor, 0, 0, 1]]);
        assert_eq!(e.get(this, VirtualActorPathHandler::iCurrentNodeIndex), 1);
    }

    #[test]
    fn use_door_warns_when_the_actor_is_far_from_the_next_node() {
        let (mut e, this, actor) = engine(3);
        e.register(DOORS_TELEPORT, |_, _| word(0));
        e.register(SET_LOCATION, nothing);
        e.register_double(NODE_DOOR, move |_, _| word(actor));
        e.register(ACTIVATE, |_, _| word(1));
        e.register(PARENT_CELL, |_, _| word(0));
        e.register(POINTS_DIFFER, |_, _| word(1));
        e.register(NAME_TEST, |_, _| word(0));
        e.register(REFERENCE_NAME, |_, _| word(0x5555));
        e.register(DEBUG_PRINT, nothing);
        e.set_global(DOOR_ARRIVAL_TOLERANCE, 5.0f64);
        let point = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        // The next node (index 1) is 20 units from the actor.
        assert_eq!(e.call(0x009e_ad90, &args![this, point]).i32(), -2);
        let printed = calls_to(&e, DEBUG_PRINT);
        assert_eq!(printed.len(), 1);
        assert_eq!(
            printed[0],
            vec![DOOR_WARNING, 0x5555, actor + 0x1000, NO_CELL_TEXT]
        );
        assert_eq!(calls_to(&e, SET_LOCATION), vec![vec![actor, point]]);
    }

    #[test]
    fn use_door_takes_the_actors_position_and_moves_on() {
        let (mut e, this, actor) = engine(3);
        e.register(DOORS_TELEPORT, |_, _| word(0));
        e.register(SET_LOCATION, nothing);
        e.register(NODE_DOOR, |_, _| word(0));
        e.register(DESIRED_PROCESS_LEVEL, |_, _| word(0));
        e.mem.set_f32(actor + 0x10, 3.0);
        let point = e.mem.alloc(0xc);
        e.set(this, VirtualActorPathHandler::fDistanceTraveled, 9.0);
        // Within the tolerance of the next node: the position is taken.
        e.mem.set_f32(actor + 0x10, 15.0);
        assert_eq!(e.call(0x009e_ad90, &args![this, point]).i32(), 1);
        assert_eq!(e.mem.f32(point), 15.0);
        assert_eq!(e.get(this, VirtualActorPathHandler::iCurrentNodeIndex), 2);
        assert_eq!(e.get(this, VirtualActorPathHandler::fDistanceTraveled), 0.0);
        // An active actor: 0, the index stays.
        e.register(DESIRED_PROCESS_LEVEL, |_, _| word(1));
        e.set(this, VirtualActorPathHandler::iCurrentNodeIndex, 0);
        assert_eq!(e.call(0x009e_ad90, &args![this, point]).i32(), 0);
        assert_eq!(e.get(this, VirtualActorPathHandler::iCurrentNodeIndex), 1);
    }
}

#[cfg(test)]
mod n027_tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    const ACTOR_VTABLE: u32 = 0x0300_0000;
    const ACTOR_POSITION: u32 = 0x0300_1000;
    const HANDLER_VTABLE: u32 = 0x0301_0000;
    const HANDLER_UPDATE: u32 = 0x0301_1000;
    const PRINTER_VTABLE: u32 = 0x0302_0000;
    const PRINTER_PRINT: u32 = 0x0302_1000;

    /// A handler whose actor stands at (1, 2, 3) (slot +0x1f4 answers actor + 0x10), with a
    /// path of `nodes` nodes (node i at x = 10 i) when `with_path`.
    fn rig(nodes: u32, with_path: bool) -> (Engine, Ptr<VirtualActorPathHandler>, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.put_vtable(ACTOR_VTABLE, &[0; 0x80]);
        e.mem.set_u32(ACTOR_VTABLE + 0x1f4, ACTOR_POSITION);
        let actor = e.mem.alloc(0x20);
        e.mem.set_u32(actor, ACTOR_VTABLE);
        e.mem.set_f32(actor + 0x10, 1.0);
        e.mem.set_f32(actor + 0x14, 2.0);
        e.mem.set_f32(actor + 0x18, 3.0);
        e.register(ACTOR_POSITION, |_, a| (a[0] + 0x10).into_ret());
        let this: Ptr<VirtualActorPathHandler> = e.new_object();
        e.put_vtable(HANDLER_VTABLE, &[0, 0, 0, HANDLER_UPDATE]);
        e.mem.set_u32(this.addr(), HANDLER_VTABLE);
        e.set(this, VirtualActorPathHandler::pActor, actor);
        let path = if with_path { e.mem.alloc(4) } else { 0 };
        e.mem.set_u32(this.addr() + 0xc, path);
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register_double(PATH_NODE_COUNT, move |_, _| nodes.into_ret());
        e.register_double(PATH_COUNT_008B6800, move |_, _| nodes.into_ret());
        e.register(PATH_NODE, |_, a| (0x100 + a[1]).into_ret());
        e.register(PATH_LAST_NODE_009DDB30, |_, _| 0x1ffu32.into_ret());
        e.register(NODE_PART, |_, a| a[0].into_ret());
        e.register(NODE_POINT, |e, a| {
            e.mem.set_f32(a[1], 10.0 * (a[0] - 0x100) as f32);
            e.mem.set_f32(a[1] + 4, 0.5);
            e.mem.set_f32(a[1] + 8, 0.25);
            a[1].into_ret()
        });
        e.call_log = Some(vec![]);
        (e, this, actor)
    }

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    #[test]
    fn update_while_player_waits_calls_update() {
        let (mut e, this, _) = rig(2, true);
        e.register(HANDLER_UPDATE, |_, a| (a[1] ^ 0x77).into_ret());
        let answer = e.call(0x009e_ad60, &args![this, 2.5f32]).u32();
        assert_eq!(answer, 2.5f32.to_bits() ^ 0x77);
        assert_eq!(
            calls(&e, HANDLER_UPDATE),
            vec![vec![this.addr(), 2.5f32.to_bits()]]
        );
    }

    #[test]
    fn distance_remaining_is_the_larger_of_the_rest_and_zero() {
        let (mut e, this, _) = rig(2, true);
        e.register(LARGER_OF, |_, a| {
            f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))).into_ret()
        });
        e.set(this, VirtualActorPathHandler::fPathLength, 30.0);
        e.set(this, VirtualActorPathHandler::fDistanceTraveled, 12.5);
        assert_eq!(e.call(0x009e_b070, &args![this]).f32(), 17.5);
        assert_eq!(calls(&e, LARGER_OF), vec![vec![17.5f32.to_bits(), 0]]);
        e.set(this, VirtualActorPathHandler::fDistanceTraveled, 40.0);
        assert_eq!(e.call(0x009e_b070, &args![this]).f32(), 0.0);
    }

    #[test]
    fn percent_traveled_by_path_and_index() {
        // No path.
        let (mut e, this, _) = rig(3, false);
        assert_eq!(e.call(0x009e_b0b0, &args![this]).f32(), 0.0);
        // Index at the node count.
        let (mut e, this, _) = rig(3, true);
        e.set(this, VirtualActorPathHandler::iCurrentNodeIndex, 3);
        assert_eq!(e.call(0x009e_b0b0, &args![this]).f32(), 1.0);
        // A path of length zero or below.
        e.set(this, VirtualActorPathHandler::iCurrentNodeIndex, 1);
        for length in [0.0f32, -4.0] {
            e.set(this, VirtualActorPathHandler::fPathLength, length);
            assert_eq!(e.call(0x009e_b0b0, &args![this]).f32(), 1.0);
        }
        // The ratio; a NaN length divides.
        e.set(this, VirtualActorPathHandler::fPathLength, 40.0);
        e.set(this, VirtualActorPathHandler::fDistanceTraveled, 10.0);
        assert_eq!(e.call(0x009e_b0b0, &args![this]).f32(), 0.25);
        e.set(this, VirtualActorPathHandler::fPathLength, f32::NAN);
        assert!(e.call(0x009e_b0b0, &args![this]).f32().is_nan());
    }

    #[test]
    fn approximate_location_is_the_actor_position() {
        let (mut e, this, _) = rig(3, true);
        let out = e.mem.alloc(0xc);
        assert_eq!(e.call(0x009e_b120, &args![this, out, 0x55u32]).u32(), out);
        assert_eq!(point(&e, out), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn next_node_location_by_index() {
        // Inside the path: the node after the current one.
        let (mut e, this, _) = rig(3, true);
        e.set(this, VirtualActorPathHandler::iCurrentNodeIndex, 1);
        let out = e.mem.alloc(0xc);
        assert_eq!(e.call(0x009e_b160, &args![this, out]).u32(), out);
        assert_eq!(point(&e, out), [20.0, 0.5, 0.25]);
        assert_eq!(calls(&e, PATH_NODE)[0][1], 2);
        // At the last node: the node of 009ddb30.
        e.set(this, VirtualActorPathHandler::iCurrentNodeIndex, 2);
        e.call(0x009e_b160, &args![this, out]);
        assert_eq!(point(&e, out), [10.0 * 0xff as f32, 0.5, 0.25]);
        assert_eq!(calls(&e, PATH_LAST_NODE_009DDB30).len(), 1);
        // 008b6800 answering 0: the actor position.
        let (mut e, this, _) = rig(0, true);
        e.call(0x009e_b160, &args![this, out]);
        assert_eq!(point(&e, out), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn node_location_checks_path_and_index() {
        let (mut e, this, _) = rig(3, true);
        let out = e.mem.alloc(0xc);
        assert!(e.call(0x009e_b230, &args![this, 2u32, out]).bool());
        assert_eq!(point(&e, out), [20.0, 0.5, 0.25]);
        e.mem.set_u32(out, 0x1234);
        assert!(!e.call(0x009e_b230, &args![this, 3u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0x1234);
        let (mut e, this, _) = rig(3, false);
        assert!(!e.call(0x009e_b230, &args![this, 0u32, out]).bool());
    }

    #[test]
    fn print_current_status_prints_position_then_nodes() {
        for (with_path, second) in [(true, STATUS_NODE_FORMAT), (false, STATUS_NO_PATH_FORMAT)] {
            let (mut e, this, _) = rig(3, with_path);
            e.set(this, VirtualActorPathHandler::iCurrentNodeIndex, 1);
            let printer = e.mem.alloc(4);
            e.put_vtable(PRINTER_VTABLE, &[0, 0, 0, 0, 0, PRINTER_PRINT]);
            e.mem.set_u32(printer, PRINTER_VTABLE);
            e.register(PRINTER_PRINT, |_, _| Ret::default());
            e.call(0x009e_b2b0, &args![this, printer, 7u32, 8u32]);
            let prints = calls(&e, PRINTER_PRINT);
            assert_eq!(prints.len(), 2);
            assert_eq!(
                prints[0],
                args![
                    printer,
                    7u32,
                    8u32,
                    STATUS_POSITION_FORMAT,
                    1.0f64,
                    2.0f64,
                    3.0f64
                ]
            );
            if with_path {
                assert_eq!(prints[1], vec![printer, 7, 8, second, 1, 3]);
            } else {
                assert_eq!(prints[1], vec![printer, 7, 8, second]);
            }
        }
    }

    #[test]
    fn load_game_loads_three_words() {
        let (mut e, this, _) = rig(3, true);
        e.register(LOAD_DATA, |_, _| Ret::default());
        e.call(0x009e_b3e0, &args![this, 0x4444u32]);
        let base = this.addr();
        assert_eq!(
            calls(&e, LOAD_DATA),
            vec![
                vec![0x4444, base + 0x14, 4],
                vec![0x4444, base + 0x18, 4],
                vec![0x4444, base + 0x1c, 4]
            ]
        );
    }
}
