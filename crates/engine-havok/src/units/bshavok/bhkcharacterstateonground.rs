//! `BSHavok/bhkcharacterstateonground.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `float`s and a `double` in the exe image: the movement limit (`01013d84`) and the factor
/// pair that sets the distance bound (`012686e4`, a `float`, and `010240c0`, a `double`).
const MOVEMENT_LIMIT: u32 = 0x0101_3d84;
const DISTANCE_FACTOR: u32 = 0x0126_86e4;
const DISTANCE_FACTOR_SCALE: u32 = 0x0102_40c0;
/// `0.5` as a `double` (`01011588`).
const HALF_DOUBLE: u32 = 0x0101_1588;
/// `bhkCharacterController::UpdateState` (`00c6cba0`): applies the wanted state.
const UPDATE_STATE: u32 = 0x00c6_cba0;
/// `bhkCharacterController::GetPosition_ov2` (`00c6e300`, `(controller, out)`).
const GET_POSITION: u32 = 0x00c6_e300;
/// `hkpCharacterMovementUtil::calculateMovement` (`00d6aef0`, cdecl `(input, output)`).
const CALCULATE_MOVEMENT: u32 = 0x00d6_aef0;
/// `bhkCharacterController::ApplyVelocityMod` (`00c6d5c0`).
const APPLY_VELOCITY_MOD: u32 = 0x00c6_d5c0;
/// `bhkCharacterController::GetCurrentState` (`00cbc510`, `(manager, state)`).
const GET_CURRENT_STATE: u32 = 0x00cb_c510;
/// `00812b00(controller, out)`, which fills a vector (the height is read at `out + 8`);
/// it is translated in another unit and called by address.
const FALL_START: u32 = 0x0081_2b00;

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (i, lane) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

// Translated from 00cd4800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd4800 - the /GS cookie check 00ec408c is not modeled
/// `bhkCharacterStateOnGround::Update` (Xbox PDB), virtual `(this, controller)`. `this` (ECX)
/// is read for a byte at +8. Controller fields (`bhkCharacterController`, Xbox PDB): `iFlags`
/// +0x414, `eWantState` +0x520, `fFallStartHeight` +0x544, `fGravity` +0x54c, `fSpeedPct`
/// +0x568, `OutVelocity` +0x4f0, `UpVec` +0x4c0, `ForwardVec` +0x4d0, `hkSupportNorm` +0x450,
/// `Direction` +0x510, `m_surfaceVelocity` +0x4a0, `StepDeltaTime` +0x4e8; the byte at +0x610
/// and the vector at +0x620 are an airborne marker and the position saved with it
/// (`bhkCharacterStateInAir`). With flag bit 8 clear and the byte clear, the character has
/// left the ground: the up velocity is cleared when `this + 8` is set, `00812b00` fills a
/// vector whose z becomes `fFallStartHeight`, the wanted state becomes 2, the state is updated
/// (`00c6cba0`) and the current state's virtual function at 0x20 gets the controller. When the
/// byte is set, the byte is cleared once the position has moved by the bound
/// (`float 012686e4 * double 010240c0`) from the saved one, or unless the wanted state is
/// 0xb. Then `calculateMovement` (`00d6aef0`) runs on a local input record (speed 1, the
/// forward, up and support vectors, the old velocity, the direction as `(y, x, z, 0)` with z
/// only when flag bit 11 is set, the limit constant and the surface velocity). Without flag
/// bit 11 the old z velocity is restored and half the gravity times the speed percentage times
/// the step time times the vector at +0x10 of what the controller's virtual function at 0x94
/// returns (unconfirmed meaning) is added to `OutVelocity`. Ends with `ApplyVelocityMod` and a
/// state update unless the wanted state is 0xb, or is 1 with flag bit 10 clear.
pub fn bhk_character_state_on_ground_update(e: &mut Engine, this: Ptr, controller: Ptr) {
    let c = controller.addr();
    let s = this.addr();
    let flags = e.mem.u32(c + 0x414);
    let airborne = e.mem.u8(c + 0x610) != 0;
    if (flags >> 8) & 1 == 0 && !airborne {
        if e.mem.u8(s + 8) != 0 {
            e.mem.set_f32(c + 0x4f8, 0.0);
        }
        let height = e.with_stack(0x20, |e, out| {
            e.call(FALL_START, &args![controller, out]);
            x87_quiet(e.mem.f32(out.addr() + 8))
        });
        e.mem.set_f32(c + 0x544, height);
        e.mem.set_u32(c + 0x520, 2);
        e.call(UPDATE_STATE, &args![controller]);
        let current = e.mem.u32(c + 0x3f0);
        let manager = e.mem.u32(c + 0x3ec);
        let state = e.call(GET_CURRENT_STATE, &args![manager, current]).u32();
        e.vcall(state, 0x20, &args![controller]);
        return;
    }
    if airborne {
        let moved = e.with_stack(0x30, |e, frame| {
            let x = frame.addr();
            let saved = lanes(e, c + 0x620);
            set_lanes(e, x + 0x20, saved);
            e.call(GET_POSITION, &args![controller, x + 0x10]);
            let position = lanes(e, x + 0x10);
            let difference = [0, 1, 2, 3].map(|i| position[i] - saved[i]);
            let squares = difference.map(|lane| lane * lane);
            let sum = squares[2] + (squares[1] + squares[0]);
            sum.sqrt()
        });
        let factor = e.mem.f32(DISTANCE_FACTOR);
        let bound = (f64::from(factor) * e.mem.f64(DISTANCE_FACTOR_SCALE)) as f32;
        if f64::from(bound) < f64::from(moved) || e.mem.u32(c + 0x520) != 0xb {
            e.mem.set_u8(c + 0x610, 0);
        }
    }
    let flags = e.mem.u32(c + 0x414);
    let forward = lanes(e, c + 0x4d0);
    let up = lanes(e, c + 0x4c0);
    let support = lanes(e, c + 0x450);
    let velocity = lanes(e, c + 0x4f0);
    let direction = lanes(e, c + 0x510);
    let surface = lanes(e, c + 0x4a0);
    let across = x87_quiet(direction[0]);
    let depth_axis = if (flags >> 11) & 1 != 0 {
        x87_quiet(direction[2])
    } else {
        0.0
    };
    let limit = x87_quiet(e.mem.f32(MOVEMENT_LIMIT));
    let old_z = e.with_stack(0xb0, |e, frame| {
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
        x87_quiet(e.mem.f32(x + 0x78))
    });
    if (e.mem.u32(c + 0x414) >> 11) & 1 == 0 {
        e.mem.set_f32(c + 0x4f8, old_z);
        let vector_at = e.vcall(c, 0x94, &args![]).u32();
        let gravity = f64::from(e.mem.f32(c + 0x54c));
        let share = f64::from(e.mem.f32(c + 0x568));
        let scale = ((gravity * e.mem.f64(HALF_DOUBLE)) * share) as f32;
        let step = x87_quiet(e.mem.f32(c + 0x4e8));
        let vector = lanes(e, vector_at + 0x10);
        let out = lanes(e, c + 0x4f0);
        let added = [0, 1, 2, 3].map(|i| (scale * vector[i]) * step + out[i]);
        set_lanes(e, c + 0x4f0, added);
    }
    e.call(APPLY_VELOCITY_MOD, &args![controller]);
    let state = e.mem.u32(c + 0x520);
    let flags = e.mem.u32(c + 0x414);
    if state != 0xb && (state != 1 || (flags >> 10) & 1 != 0) {
        e.call(UPDATE_STATE, &args![controller]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cd_4800,
        bhk_character_state_on_ground_update(Ptr, Ptr)
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

    /// A controller whose virtual function 0x94 returns `(0, 0, 1, 0)` at +0x10.
    fn rig() -> (Engine, Log, u32, u32) {
        let mut e = test_engine();
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_f32(MOVEMENT_LIMIT, 2.0);
        e.mem.set_f32(DISTANCE_FACTOR, 4.0);
        e.mem.set_f64(DISTANCE_FACTOR_SCALE, 0.5);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for address in [
            UPDATE_STATE,
            CALCULATE_MOVEMENT,
            APPLY_VELOCITY_MOD,
            FALL_START,
        ] {
            log_double(&mut e, &log, address, Ret::default());
        }
        let block = e.mem.alloc(0x40);
        set_lanes(&mut e, block + 0x10, [0.0, 0.0, 1.0, 0.0]);
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
    fn leaving_the_ground_starts_the_fall() {
        let (mut e, log, this, c) = rig();
        e.mem.set_u8(this + 8, 1);
        set_lanes(&mut e, c + 0x4f0, [1.0, 2.0, 3.0, 4.0]);
        e.register_double(FALL_START, |e, a| {
            e.mem.set_f32(a[1] + 8, 42.0);
            Ret::default()
        });
        let state_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(state_vtable + 0x20, 0x0900_0020);
        let state = e.mem.alloc(0x10);
        e.mem.set_u32(state, state_vtable);
        log_double(&mut e, &log, 0x0900_0020, Ret::default());
        log_double(
            &mut e,
            &log,
            GET_CURRENT_STATE,
            Ret {
                eax: state,
                ..Ret::default()
            },
        );
        e.mem.set_u32(c + 0x3ec, 0x7000);
        e.mem.set_u32(c + 0x3f0, 3);
        e.call(0x00cd_4800, &args![this, c]);
        assert_eq!(e.mem.f32(c + 0x4f8), 0.0);
        assert_eq!(e.mem.f32(c + 0x544), 42.0);
        assert_eq!(e.mem.u32(c + 0x520), 2);
        let calls = log.borrow();
        assert_eq!(calls[0].0, UPDATE_STATE);
        assert_eq!(calls[1], (GET_CURRENT_STATE, vec![0x7000, 3]));
        assert_eq!(calls[2], (0x0900_0020, vec![state, c]));
        assert_eq!(calls.len(), 3);
    }

    #[test]
    fn the_marker_clears_after_the_position_moved_far_enough() {
        let (mut e, _, this, c) = rig();
        e.mem.set_u32(c + 0x414, 0x100);
        e.mem.set_u8(c + 0x610, 1);
        e.mem.set_u32(c + 0x520, 0xb);
        set_lanes(&mut e, c + 0x620, [0.0, 0.0, 0.0, 0.0]);
        e.register_double(GET_POSITION, |e, a| {
            for (i, v) in [3.0f32, 4.0, 0.0, 1.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, v);
            }
            Ret::default()
        });
        // Bound is 4 * 0.5 = 2; the position moved by 5, so the marker is cleared.
        e.call(0x00cd_4800, &args![this, c]);
        assert_eq!(e.mem.u8(c + 0x610), 0);
        // A small bound keeps it while the wanted state is 0xb.
        e.mem.set_u8(c + 0x610, 1);
        e.mem.set_f32(DISTANCE_FACTOR, 100.0);
        e.call(0x00cd_4800, &args![this, c]);
        assert_eq!(e.mem.u8(c + 0x610), 1);
        // A different wanted state clears it regardless.
        e.mem.set_u32(c + 0x520, 5);
        e.call(0x00cd_4800, &args![this, c]);
        assert_eq!(e.mem.u8(c + 0x610), 0);
    }

    #[test]
    fn walking_adds_gravity_along_the_vector_and_keeps_the_old_z() {
        let (mut e, log, this, c) = rig();
        e.mem.set_u32(c + 0x414, 0x100 | 0x400);
        e.mem.set_u32(c + 0x520, 1);
        e.mem.set_f32(c + 0x54c, 8.0);
        e.mem.set_f32(c + 0x568, 0.5);
        e.mem.set_f32(c + 0x4e8, 0.25);
        set_lanes(&mut e, c + 0x4f0, [1.0, 2.0, 3.0, 0.0]);
        set_lanes(&mut e, c + 0x510, [0.1, 0.2, 0.3, 0.0]);
        let seen: Rc<RefCell<Vec<(f32, [f32; 4])>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(CALCULATE_MOVEMENT, move |e, a| {
            sink.borrow_mut()
                .push((e.mem.f32(a[0] + 0x60), lanes(e, a[0] + 0x50)));
            // The calculation replaces the velocity.
            set_lanes(e, a[1], [9.0, 9.0, 9.0, 0.0]);
            Ret::default()
        });
        e.call(0x00cd_4800, &args![this, c]);
        assert_eq!(*seen.borrow(), vec![(2.0, [0.2, 0.1, 0.0, 0.0])]);
        // z is the old 3.0, then (8 * 0.5 * 0.5) * (0, 0, 1) * 0.25 = 0.5 is added.
        assert_eq!(lanes(&e, c + 0x4f0), [9.0, 9.0, 3.5, 0.0]);
        // State 1 with flag bit 10 set updates the state at the end.
        assert!(log.borrow().iter().any(|(a, _)| *a == UPDATE_STATE));
        // Flag bit 11 keeps the calculated z and skips gravity.
        e.mem.set_u32(c + 0x414, 0x100 | 0x800);
        e.mem.set_u32(c + 0x520, 0xb);
        e.call(0x00cd_4800, &args![this, c]);
        assert_eq!(lanes(&e, c + 0x4f0), [9.0, 9.0, 9.0, 0.0]);
        assert_eq!(seen.borrow()[1].1, [0.2, 0.1, 0.3, 0.0]);
    }
}
