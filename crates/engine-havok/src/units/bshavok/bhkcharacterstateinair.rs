//! `BSHavok/bhkcharacterstateinair.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `float`s and a `double` in the exe image: the acrobatics scale and offset (`011b0140`,
/// `01267bbc`), the vector whose first lane is the target of the velocity projection
/// (`010c7160`) and the fall distance bound (`0101ffa0`, `double`).
const ACROBATICS_SCALE: u32 = 0x011b_0140;
const ACROBATICS_OFFSET: u32 = 0x0126_7bbc;
const PROJECTION_VECTOR: u32 = 0x010c_7160;
const FALL_DISTANCE_BOUND: u32 = 0x0101_ffa0;
/// `bhkCharacterController::UpdateState` (`00c6cba0`): applies the wanted state.
const UPDATE_STATE: u32 = 0x00c6_cba0;
/// `bhkCharacterController::GetFallDistance` (`00c70550`): returns the distance in ST0.
const GET_FALL_DISTANCE: u32 = 0x00c7_0550;
/// `bhkCharacterController::GetPosition_ov2` (`00c6e300`, `(controller, out)`).
const GET_POSITION: u32 = 0x00c6_e300;
/// `hkpCharacterMovementUtil::calculateMovement` (`00d6aef0`, cdecl `(input, output)`).
const CALCULATE_MOVEMENT: u32 = 0x00d6_aef0;
/// Adds the step time to the fall time (`00c6e2e0`, `FLD [ECX+0x4E8]; FADD [ECX+0x548]`).
const ADD_FALL_TIME: u32 = 0x00c6_e2e0;

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (i, lane) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

// Translated from 00cd3fb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd3fb0 - the /GS cookie check 00ec408c is not modeled
/// `bhkCharacterStateInAir::Update` (Xbox PDB), virtual `(this, controller)`. `this` (ECX)
/// is read for a float at +8 (the limit given to `calculateMovement`). Controller fields
/// (`bhkCharacterController`, Xbox PDB): `iFlags` +0x414, `eWantState` +0x520, `fFallTime`
/// +0x548, `fAcrobatics` +0x534, `fGravity` +0x54c, `UpVec` +0x4c0, `ForwardVec` +0x4d0,
/// `OutVelocity` +0x4f0, `Direction` +0x510, `m_surfaceVelocity` +0x4a0, `StepDeltaTime`
/// +0x4e8. Flag bit 8 resets the wanted state to 0 and sets flag 0x80. In state 0xb, once the
/// fall time exceeds 1 and `GetFallDistance` is below the bound at `0101ffa0`, the
/// state becomes 0, byte +0x610 is set and the position (`GetPosition_ov2`) is saved at
/// +0x620. State 1 becomes 0xb; any state but 0xb ends with the fall time cleared and the
/// state updated (`00c6cba0`). With positive acrobatics, `calculateMovement` (`00d6aef0`) is
/// run on a local input record (speed 1 when flags 0x1800 are set, else
/// `acrobatics * scale + offset`) and its output velocity is made orthogonal to the up
/// vector and then given back the up component it had before. Finally the vector at +0x10 of
/// what the controller's virtual function at 0x94 returns (second call; unconfirmed meaning)
/// times gravity times the step time is added to `OutVelocity`, and the fall time advances
/// (`00c6e2e0`).
pub fn bhk_character_state_in_air_update(e: &mut Engine, this: Ptr, controller: Ptr) {
    let c = controller.addr();
    let s = this.addr();
    let flags = e.mem.u32(c + 0x414);
    if (flags >> 8) & 1 != 0 {
        e.mem.set_u32(c + 0x520, 0);
        let flags = e.mem.u32(c + 0x414);
        e.mem.set_u32(c + 0x414, flags | 0x80);
    }
    if e.mem.u32(c + 0x520) == 0xb {
        let fall_time = e.mem.f32(c + 0x548);
        if 1.0 < f64::from(fall_time) {
            let distance = e.call(GET_FALL_DISTANCE, &args![controller]).f32();
            if f64::from(distance) < e.mem.f64(FALL_DISTANCE_BOUND) {
                e.mem.set_u32(c + 0x520, 0);
                e.mem.set_u8(c + 0x610, 1);
                e.with_stack(0x10, |e, position| {
                    e.call(GET_POSITION, &args![controller, position]);
                    let saved = lanes(e, position.addr());
                    set_lanes(e, c + 0x620, saved);
                });
            }
        }
    }
    let state = e.mem.u32(c + 0x520);
    if state != 0xb {
        if state != 1 {
            e.mem.set_f32(c + 0x548, 0.0);
            e.call(UPDATE_STATE, &args![controller]);
            return;
        }
        e.mem.set_u32(c + 0x520, 0xb);
    }
    let acrobatics = e.mem.f32(c + 0x534);
    if 0.0 < f64::from(acrobatics) {
        let flags = e.mem.u32(c + 0x414);
        let speed = if flags & 0x1800 != 0 {
            1.0
        } else {
            let scaled = f64::from(acrobatics) * f64::from(e.mem.f32(ACROBATICS_SCALE));
            (scaled + f64::from(e.mem.f32(ACROBATICS_OFFSET))) as f32
        };
        let limit = x87_quiet(e.mem.f32(s + 8));
        let forward = lanes(e, c + 0x4d0);
        let up = lanes(e, c + 0x4c0);
        let surface = lanes(e, c + 0x4a0);
        let velocity = lanes(e, c + 0x4f0);
        let direction = lanes(e, c + 0x510);
        let across = x87_quiet(direction[0]);
        e.with_stack(0xa0, |e, frame| {
            let x = frame.addr();
            e.mem.set_f32(x + 0x20, speed);
            set_lanes(e, x + 0x30, forward);
            set_lanes(e, x + 0x40, up);
            set_lanes(e, x + 0x50, up);
            set_lanes(e, x + 0x60, velocity);
            set_lanes(e, x + 0x70, [direction[1], across, 0.0, 0.0]);
            e.mem.set_f32(x + 0x80, limit);
            set_lanes(e, x + 0x90, surface);
            e.call(CALCULATE_MOVEMENT, &args![x + 0x20, c + 0x4f0]);
            let up = lanes(e, c + 0x4c0);
            let out = lanes(e, c + 0x4f0);
            let product = [0, 1, 2, 3].map(|i| up[i] * out[i]);
            let dot = product[2] + (product[1] + product[0]);
            let target = e.mem.f32(PROJECTION_VECTOR);
            let gap = target - dot;
            let out = lanes(e, c + 0x4f0);
            let adjusted = [0, 1, 2, 3].map(|i| gap * up[i] + out[i]);
            set_lanes(e, c + 0x4f0, adjusted);
            let up = lanes(e, c + 0x4c0);
            let before = lanes(e, x + 0x60);
            let product = [0, 1, 2, 3].map(|i| up[i] * before[i]);
            let dot = product[2] + (product[1] + product[0]);
            let out = lanes(e, c + 0x4f0);
            let restored = [0, 1, 2, 3].map(|i| dot * up[i] + out[i]);
            set_lanes(e, c + 0x4f0, restored);
        });
    }
    e.vcall(c, 0x94, &args![]);
    let second = e.vcall(c, 0x94, &args![]).u32();
    let vector = lanes(e, second + 0x10);
    e.vcall(c, 0x94, &args![]);
    let gravity = x87_quiet(e.mem.f32(c + 0x54c));
    let step = x87_quiet(e.mem.f32(c + 0x4e8));
    let out = lanes(e, c + 0x4f0);
    let moved = [0, 1, 2, 3].map(|i| (gravity * vector[i]) * step + out[i]);
    set_lanes(e, c + 0x4f0, moved);
    e.call(ADD_FALL_TIME, &args![controller]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cd_3fb0,
        bhk_character_state_in_air_update(Ptr, Ptr)
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

    fn log_double(e: &mut Engine, log: &Log, address: u32, ret: Ret) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            ret
        });
    }

    /// A controller whose virtual function 0x94 returns a block holding `(0, 0, -1, 0)`.
    fn rig() -> (Engine, Log, u32, u32) {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for address in [
            UPDATE_STATE,
            GET_POSITION,
            ADD_FALL_TIME,
            CALCULATE_MOVEMENT,
        ] {
            log_double(&mut e, &log, address, Ret::default());
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
        let this = e.mem.alloc(0x20);
        (e, log, this, c)
    }

    #[test]
    fn a_state_other_than_falling_resets_the_fall_time_and_updates() {
        let (mut e, log, this, c) = rig();
        e.mem.set_u32(c + 0x414, 0x100);
        e.mem.set_u32(c + 0x520, 5);
        e.mem.set_f32(c + 0x548, 3.0);
        e.call(0x00cd_3fb0, &args![this, c]);
        assert_eq!(e.mem.u32(c + 0x520), 0);
        assert_eq!(e.mem.u32(c + 0x414), 0x180);
        assert_eq!(e.mem.f32(c + 0x548), 0.0);
        assert_eq!(*log.borrow(), vec![(UPDATE_STATE, vec![c])]);
    }

    #[test]
    fn landing_saves_the_position_and_gravity_moves_the_velocity() {
        let (mut e, log, this, c) = rig();
        e.mem.set_f64(FALL_DISTANCE_BOUND, 100.0);
        e.register_double(GET_FALL_DISTANCE, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        e.register_double(GET_POSITION, |e, a| {
            for (i, v) in [7.0f32, 8.0, 9.0, 1.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, v);
            }
            Ret::default()
        });
        e.mem.set_u32(c + 0x520, 0xb);
        e.mem.set_f32(c + 0x548, 2.0);
        e.mem.set_f32(c + 0x534, 0.0);
        e.mem.set_f32(c + 0x54c, 10.0);
        e.mem.set_f32(c + 0x4e8, 0.5);
        set_lanes(&mut e, c + 0x4f0, [1.0, 1.0, 1.0, 1.0]);
        e.call(0x00cd_3fb0, &args![this, c]);
        // Landing resets the state to 0, which then updates the state and stops.
        assert_eq!(e.mem.u32(c + 0x520), 0);
        assert_eq!(e.mem.u8(c + 0x610), 1);
        assert_eq!(lanes(&e, c + 0x620), [7.0, 8.0, 9.0, 1.0]);
        // State 1 turns into 0xb and the velocity gains gravity * vector * step.
        e.mem.set_u32(c + 0x520, 1);
        log.borrow_mut().clear();
        e.call(0x00cd_3fb0, &args![this, c]);
        assert_eq!(e.mem.u32(c + 0x520), 0xb);
        assert_eq!(lanes(&e, c + 0x4f0), [1.0, 1.0, -4.0, 1.0]);
        assert_eq!(*log.borrow(), vec![(ADD_FALL_TIME, vec![c])]);
    }

    #[test]
    fn positive_acrobatics_runs_the_movement_calculation() {
        let (mut e, log, this, c) = rig();
        e.mem.set_u32(c + 0x520, 0xb);
        e.mem.set_u32(c + 0x414, 0x1800);
        e.mem.set_f32(c + 0x534, 1.0);
        e.mem.set_f32(PROJECTION_VECTOR, 1.0);
        e.mem.set_f32(this + 8, 5.5);
        set_lanes(&mut e, c + 0x4c0, [0.0, 1.0, 0.0, 0.0]);
        set_lanes(&mut e, c + 0x4d0, [0.0, 0.0, 1.0, 0.0]);
        set_lanes(&mut e, c + 0x4f0, [2.0, 3.0, 4.0, 0.0]);
        set_lanes(&mut e, c + 0x510, [0.25, 0.5, 0.75, 0.0]);
        let seen: Rc<RefCell<Vec<(f32, f32, [f32; 4])>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(CALCULATE_MOVEMENT, move |e, a| {
            sink.borrow_mut().push((
                e.mem.f32(a[0]),
                e.mem.f32(a[0] + 0x60),
                lanes(e, a[0] + 0x50),
            ));
            // The movement result: out = (10, 20, 30, 0).
            set_lanes(e, a[1], [10.0, 20.0, 30.0, 0.0]);
            Ret::default()
        });
        e.call(0x00cd_3fb0, &args![this, c]);
        assert_eq!(*seen.borrow(), vec![(1.0, 5.5, [0.5, 0.25, 0.0, 0.0])]);
        // out' = out + (1 - dot(up, out)) * up = (10, 20 + 1 - 20, 30, 0); then the saved
        // old velocity's up component (3) is added back along up: (10, 4, 30, 0).
        let out = lanes(&e, c + 0x4f0);
        assert_eq!(out[0], 10.0);
        assert_eq!(out[1], 4.0);
        assert!(log.borrow().iter().any(|(a, _)| *a == ADD_FALL_TIME));
    }
}
