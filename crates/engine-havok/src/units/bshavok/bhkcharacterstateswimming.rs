//! `BSHavok/bhkcharacterstateswimming.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `float`s and `double`s in the exe image: the movement limit (`01016410`), `0.5` as a
/// `float` (`01016248`) and as a `double` (`01011588`), the factor pair that sets the water
/// bound (`012686e0` and `01020758`, a `float` and a `double`).
const MOVEMENT_LIMIT: u32 = 0x0101_6410;
const HALF: u32 = 0x0101_6248;
const HALF_DOUBLE: u32 = 0x0101_1588;
const WATER_FACTOR: u32 = 0x0126_86e0;
const WATER_FACTOR_SCALE: u32 = 0x0102_0758;
/// The global zero `hkVector4` the code reads when the controller has no proxy (`01267e30`).
const ZERO_VECTOR: u32 = 0x0126_7e30;
/// `bhkCharacterController::UpdateState` (`00c6cba0`): applies the wanted state.
const UPDATE_STATE: u32 = 0x00c6_cba0;
/// `bhkCharacterProxy::UnMarkRead` (`00c89720`): a jump to the controller's virtual function at
/// offset 0x94.
const CONTROLLER_SLOT_94: u32 = 0x00c8_9720;
/// `hkpCharacterProxy::getPosition` (`00cac910`).
const GET_PROXY_POSITION: u32 = 0x00ca_c910;
/// `hkpCharacterMovementUtil::calculateMovement` (`00d6aef0`, cdecl `(input, output)`).
const CALCULATE_MOVEMENT: u32 = 0x00d6_aef0;
/// `bhkCharacterController::ApplyVelocityMod` (`00c6d5c0`).
const APPLY_VELOCITY_MOD: u32 = 0x00c6_d5c0;

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (i, lane) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

/// `OutVelocity += (gravity * vector) * step` with the step already signed.
fn add_gravity(e: &mut Engine, c: u32, vector: [f32; 4], step: f32) {
    let gravity = x87_quiet(e.mem.f32(c + 0x54c));
    let out = lanes(e, c + 0x4f0);
    let moved = [0, 1, 2, 3].map(|i| (gravity * vector[i]) * step + out[i]);
    set_lanes(e, c + 0x4f0, moved);
}

// Translated from 00cd44e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd44e0 - the /GS cookie check 00ec408c is not modeled
/// `bhkCharacterStateSwimming::Update` (Xbox PDB), virtual `(this, controller)`; `this` (ECX)
/// is not read. Controller fields (`bhkCharacterController`, Xbox PDB): `iFlags` +0x414,
/// `eWantState` +0x520, `fGravity` +0x54c, `StepDeltaTime` +0x4e8, `OutVelocity` +0x4f0,
/// `UpVec` +0x4c0, `ForwardVec` +0x4d0, `Direction` +0x510, `hkSupportNorm` +0x450,
/// `m_surfaceVelocity` +0x4a0, `RotCenter` +0x570; +0x53c, +0x560 and +0x564 are floats the
/// PDB names `fCenter`-area fields (unconfirmed). Runs `calculateMovement` (`00d6aef0`) on a
/// local input record (speed 1, the forward and up vectors, the support normal when flag
/// bit 8 is set or the up vector, the old velocity, the direction as `(y, x, z, 0)`, the limit
/// constant and the surface velocity), then measures the depth of the proxy's position below
/// `RotCenter.z` (plus a share of the height at +0x564 when +0x560 is at least 0.5), sets flag
/// 0x400 and, by comparing the depth with the float at +0x53c, either updates the state,
/// zeroes an upward velocity, or adds `gravity * vector * step time` to `OutVelocity` (the
/// vector is at +0x10 of what the controller's virtual function at 0x94 returns, second
/// call; unconfirmed meaning); the last case uses a negated step. Ends with
/// `ApplyVelocityMod` and a state update when the wanted state is 0.
pub fn bhk_character_state_swimming_update(e: &mut Engine, _unused_0: Ptr, controller: Ptr) {
    let c = controller.addr();
    let flags = e.mem.u32(c + 0x414);
    let forward = lanes(e, c + 0x4d0);
    let up = lanes(e, c + 0x4c0);
    let velocity = lanes(e, c + 0x4f0);
    let direction = lanes(e, c + 0x510);
    let surface = lanes(e, c + 0x4a0);
    let across = x87_quiet(direction[0]);
    let depth_axis = x87_quiet(direction[2]);
    let limit = x87_quiet(e.mem.f32(MOVEMENT_LIMIT));
    let support = if (flags >> 8) & 1 != 0 {
        lanes(e, c + 0x450)
    } else {
        up
    };
    e.with_stack(0xb0, |e, frame| {
        let x = frame.addr();
        e.mem.set_f32(x + 0x30, 1.0);
        set_lanes(e, x + 0x40, forward);
        set_lanes(e, x + 0x50, up);
        set_lanes(e, x + 0x60, support);
        set_lanes(e, x + 0x70, velocity);
        set_lanes(e, x + 0x80, [direction[1], across, depth_axis, 0.0]);
        e.mem.set_f32(x + 0x90, limit);
        set_lanes(e, x + 0xa0, surface);
        e.call(CALCULATE_MOVEMENT, &args![x + 0x30, c + 0x4f0]);
    });
    let proxy = e.mem.u32(c + 8);
    let mut position_at = ZERO_VECTOR;
    if proxy != 0 {
        e.call(CONTROLLER_SLOT_94, &args![controller]);
        position_at = e.call(GET_PROXY_POSITION, &args![proxy]).u32();
        e.call(CONTROLLER_SLOT_94, &args![controller]);
    }
    let height = e.mem.f32(position_at + 8);
    let rotation_centre = e.mem.f32(c + 0x578);
    let mut depth = (f64::from(height) - f64::from(rotation_centre)) as f32;
    let half = e.mem.f32(HALF);
    let share = e.mem.f32(c + 0x560);
    if f64::from(half) <= f64::from(share) {
        let actor = f64::from(e.mem.f32(c + 0x564));
        let part = (actor * e.mem.f64(HALF_DOUBLE)) * f64::from(share);
        depth = (part + f64::from(depth)) as f32;
    }
    e.vcall(c, 0x94, &args![]);
    let second = e.vcall(c, 0x94, &args![]).u32();
    let vector = lanes(e, second + 0x10);
    e.vcall(c, 0x94, &args![]);
    let flags = e.mem.u32(c + 0x414);
    e.mem.set_u32(c + 0x414, flags | 0x400);
    let water = e.mem.f32(c + 0x53c);
    let step = x87_quiet(e.mem.f32(c + 0x4e8));
    if f64::from(water) < f64::from(depth) {
        if e.mem.u32(c + 0x520) == 1 {
            e.call(UPDATE_STATE, &args![controller]);
        } else {
            let rising = e.mem.f32(c + 0x4f8);
            if 0.0 < f64::from(rising) {
                e.mem.set_f32(c + 0x4f8, 0.0);
            }
            add_gravity(e, c, vector, step);
        }
    } else {
        let narrow = f64::from(half) > f64::from(share);
        if !narrow {
            let gravity_is_one = f64::from(e.mem.f32(c + 0x54c)) == 1.0;
            let flags = e.mem.u32(c + 0x414);
            if !gravity_is_one && (flags >> 8) & 1 == 0 && (flags >> 9) & 1 == 0 {
                add_gravity(e, c, vector, step);
            }
        } else if e.mem.u32(c + 0x520) == 1 && (e.mem.u32(c + 0x414) >> 10) & 1 != 0 {
            e.call(UPDATE_STATE, &args![controller]);
        } else {
            let factor = e.mem.f32(WATER_FACTOR);
            let bound = (f64::from(factor) * e.mem.f64(WATER_FACTOR_SCALE)) as f32;
            let lower = f64::from(water) - f64::from(bound);
            if lower > f64::from(depth) {
                add_gravity(e, c, vector, -step);
            }
        }
    }
    e.call(APPLY_VELOCITY_MOD, &args![controller]);
    if e.mem.u32(c + 0x520) == 0 {
        e.call(UPDATE_STATE, &args![controller]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cd_44e0,
        bhk_character_state_swimming_update(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A controller with no proxy (position from the global vector), a gravity vector of
    /// `(0, 0, -1, 0)` from the virtual function 0x94 and logging doubles.
    fn rig() -> (Engine, Log, u32) {
        let mut e = test_engine();
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_f32(MOVEMENT_LIMIT, 3.0);
        e.mem.set_f32(WATER_FACTOR, 4.0);
        e.mem.set_f64(WATER_FACTOR_SCALE, 0.5);
        set_lanes(&mut e, ZERO_VECTOR, [0.0; 4]);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for address in [UPDATE_STATE, CALCULATE_MOVEMENT, APPLY_VELOCITY_MOD] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let block = e.mem.alloc(0x40);
        set_lanes(&mut e, block + 0x10, [0.0, 0.0, -1.0, 0.0]);
        e.register_double(0x0900_0094, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x94, 0x0900_0094);
        let c = e.mem.alloc(0x660);
        e.mem.set_u32(c, vtable);
        e.mem.set_f32(c + 0x54c, 10.0);
        e.mem.set_f32(c + 0x4e8, 0.5);
        (e, log, c)
    }

    #[test]
    fn deep_water_zeroes_an_upward_velocity_and_applies_gravity() {
        let (mut e, log, c) = rig();
        // depth = 0 - RotCenter.z; water level below it.
        e.mem.set_f32(c + 0x578, -2.0);
        e.mem.set_f32(c + 0x53c, 1.0);
        e.mem.set_f32(c + 0x560, 0.0);
        e.mem.set_u32(c + 0x520, 5);
        set_lanes(&mut e, c + 0x4f0, [1.0, 2.0, 3.0, 0.0]);
        e.call(0x00cd_44e0, &args![0u32, c]);
        assert_eq!(e.mem.u32(c + 0x414) & 0x400, 0x400);
        // The upward component is cleared, then gravity * (0, 0, -1) * 0.5 is added.
        assert_eq!(lanes(&e, c + 0x4f0), [1.0, 2.0, -5.0, 0.0]);
        let calls = log.borrow();
        assert_eq!(calls[0].0, CALCULATE_MOVEMENT);
        assert_eq!(calls[0].1[1], c + 0x4f0);
        assert_eq!(calls[1], (APPLY_VELOCITY_MOD, vec![c]));
        assert_eq!(calls.len(), 2);
    }

    #[test]
    fn shallow_water_in_state_one_just_updates_the_state() {
        let (mut e, log, c) = rig();
        e.mem.set_f32(c + 0x578, 5.0);
        e.mem.set_f32(c + 0x53c, 1.0);
        e.mem.set_f32(c + 0x560, 0.0);
        e.mem.set_u32(c + 0x520, 1);
        e.mem.set_f32(c + 0x54c, 1.0);
        e.call(0x00cd_44e0, &args![0u32, c]);
        // Depth -5 is not above the water level, share 0 < 0.5 and state 1 with flag 0x400
        // (just set): update the state, then ApplyVelocityMod.
        let calls = log.borrow();
        assert_eq!(calls[1], (UPDATE_STATE, vec![c]));
        assert_eq!(calls[2], (APPLY_VELOCITY_MOD, vec![c]));
    }

    #[test]
    fn high_share_applies_gravity_unless_it_is_one() {
        let (mut e, _, c) = rig();
        e.mem.set_f32(c + 0x578, 5.0);
        e.mem.set_f32(c + 0x53c, 1.0);
        e.mem.set_f32(c + 0x560, 1.0);
        e.mem.set_f32(c + 0x564, 2.0);
        e.mem.set_u32(c + 0x520, 5);
        set_lanes(&mut e, c + 0x4f0, [0.0; 4]);
        e.call(0x00cd_44e0, &args![0u32, c]);
        assert_eq!(lanes(&e, c + 0x4f0), [0.0, 0.0, -5.0, 0.0]);
        // Gravity of exactly one is skipped.
        set_lanes(&mut e, c + 0x4f0, [0.0; 4]);
        e.mem.set_f32(c + 0x54c, 1.0);
        e.call(0x00cd_44e0, &args![0u32, c]);
        assert_eq!(lanes(&e, c + 0x4f0), [0.0; 4]);
    }

    #[test]
    fn the_wanted_state_zero_updates_at_the_end() {
        let (mut e, log, c) = rig();
        e.mem.set_f32(c + 0x578, 5.0);
        e.mem.set_f32(c + 0x53c, 1.0);
        e.mem.set_f32(c + 0x560, 1.0);
        e.mem.set_f32(c + 0x54c, 1.0);
        e.mem.set_u32(c + 0x520, 0);
        e.call(0x00cd_44e0, &args![0u32, c]);
        let calls = log.borrow();
        assert_eq!(calls.last().unwrap(), &(UPDATE_STATE, vec![c]));
    }
}
