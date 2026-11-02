//! `BSHavok/bhkcharacterstatejumping.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::x87::rsqrt_estimate;

/// `0.0`, `3.0` and `0.5` (`float`) in the exe image: the `RSQRTSS` refinement constants.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
/// The global zero `hkVector4` the code reads when the controller has no proxy
/// (`01267e30`).
const ZERO_VECTOR: u32 = 0x0126_7e30;
/// The CRT square root taking its argument in ST0 (`00ec6040`).
const X87_SQRT: u32 = 0x00ec_6040;
/// `bhkCharacterController::UpdateState` (`00c6cba0`): applies the wanted state.
const UPDATE_STATE: u32 = 0x00c6_cba0;
/// `bhkCharacterProxy::UnMarkRead` (`00c89720`): a jump to the controller's virtual function at
/// offset 0x94.
const CONTROLLER_SLOT_94: u32 = 0x00c8_9720;
/// `hkpCharacterProxy::getLinearVelocity` (`00cac950`, `LEA EAX,[ECX+0x10]`).
const GET_LINEAR_VELOCITY: u32 = 0x00ca_c950;
/// `bhkCharacterController::GetPosition_ov2` (`00c6e300`, `(controller, out)`).
const GET_POSITION: u32 = 0x00c6_e300;
/// An angle extraction (`00458620`, cdecl `(out, input)`): three floats at `out`.
const EXTRACT_ANGLES: u32 = 0x0045_8620;
/// `bhkCharacterController::GetCurrentState` (`00cbc510`, `(manager, state)`).
const GET_CURRENT_STATE: u32 = 0x00cb_c510;

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (i, lane) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

/// `z*z + (y*y + x*x)`, the `MULPS` / `ADDSS` sum of squares.
fn sum_of_squares(v: [f32; 4]) -> f32 {
    let squares = v.map(|lane| lane * lane);
    squares[2] + (squares[1] + squares[0])
}

/// The `RSQRTSS` estimate refined once with the `CMPEQSS` / `ANDNPS` zero guard.
fn inverse_length(sum: f32, zero: f32, half: f32, three: f32) -> f32 {
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    if sum == zero {
        0.0
    } else {
        value
    }
}

// Translated from 00cd4280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd4280 - the /GS cookie check 00ec408c is not modeled
/// `bhkCharacterStateJumping::Update` (Xbox PDB), virtual `(this, controller)`; `this` (ECX)
/// is not read. Controller fields (`bhkCharacterController`, Xbox PDB): `eWantState` +0x520,
/// `iFlags` +0x414, `fGravity` +0x54c, `fJumpHeight` +0x540, `OutVelocity` +0x4f0,
/// `fFallStartHeight` +0x544, `phkObject` +0x008. While the wanted state is 0 or 3 it just
/// updates the state (`00c6cba0`). Otherwise it sets flag 0x2000, takes the vector at +0x10 of what
/// the controller's virtual function at 0x94 returns (second call; unconfirmed meaning), scales it
/// by the gravity and gives the character a launch velocity of `-sqrt(2 * |jump height| * L)`
/// times the unit vector (`L` the scaled vector's length); the new `OutVelocity` is the proxy's
/// linear velocity (z cleared) plus that launch velocity. It records the height from the
/// angle extraction of the controller's position in `fFallStartHeight`, sets the wanted state
/// to 2, updates the state, clears flags 0x20000 and 0x100, and calls the new state's
/// virtual function at 0x20 with the controller.
pub fn bhk_character_state_jumping_update(e: &mut Engine, _unused_0: Ptr, controller: Ptr) {
    let c = controller.addr();
    let wanted = e.mem.u32(c + 0x520);
    if wanted != 0xb && (wanted == 0 || wanted == 3) {
        e.call(UPDATE_STATE, &args![controller]);
        return;
    }
    let flags = e.mem.u32(c + 0x414);
    e.mem.set_u32(c + 0x414, flags | 0x2000);
    e.vcall(c, 0x94, &args![]);
    let second = e.vcall(c, 0x94, &args![]).u32();
    let direction = lanes(e, second + 0x10);
    e.vcall(c, 0x94, &args![]);
    let (zero, three, half) = (e.mem.f32(ZERO), e.mem.f32(THREE), e.mem.f32(HALF));
    let gravity = x87_quiet(e.mem.f32(c + 0x54c));
    let height = f32::from_bits(x87_quiet_bits(e.mem.f32(c + 0x540).to_bits()) & 0x7fff_ffff);
    let scaled = direction.map(|lane| gravity * lane);
    let sum = sum_of_squares(scaled);
    let scale = inverse_length(sum, zero, half, three);
    let length = sum * scale;
    let product = (f64::from(height) * (f64::from(length) + f64::from(length))) as f32;
    let root = x87_quiet(e.call(X87_SQRT, &args![f64::from(product)]).f32());
    let speed = -root;
    let launch = scaled.map(|lane| speed * (scale * lane));
    let proxy = e.mem.u32(c + 8);
    let mut velocity_at = ZERO_VECTOR;
    if proxy != 0 {
        e.call(CONTROLLER_SLOT_94, &args![controller]);
        velocity_at = e.call(GET_LINEAR_VELOCITY, &args![proxy]).u32();
        e.call(CONTROLLER_SLOT_94, &args![controller]);
    }
    let mut velocity = lanes(e, velocity_at);
    velocity[2] = 0.0;
    let out = [0, 1, 2, 3].map(|i| velocity[i] + launch[i]);
    set_lanes(e, c + 0x4f0, out);
    let fall_start = e.with_stack(0x50, |e, frame| {
        let x = frame.addr();
        set_lanes(e, x + 0x20, velocity);
        e.call(GET_POSITION, &args![controller, x + 0x30]);
        e.call(EXTRACT_ANGLES, &args![x + 0x20, x + 0x30]);
        x87_quiet(e.mem.f32(x + 0x28))
    });
    e.mem.set_f32(c + 0x544, fall_start);
    e.mem.set_u32(c + 0x520, 2);
    e.call(UPDATE_STATE, &args![controller]);
    let flags = e.mem.u32(c + 0x414);
    e.mem.set_u32(c + 0x414, flags & 0xfffd_feff);
    let current = e.mem.u32(c + 0x3f0);
    let manager = e.mem.u32(c + 0x3ec);
    let state = e.call(GET_CURRENT_STATE, &args![manager, current]).u32();
    e.vcall(state, 0x20, &args![controller]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cd_4280,
        bhk_character_state_jumping_update(Ptr, Ptr)
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

    fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn waiting_states_only_update_the_state() {
        for wanted in [0u32, 3] {
            let mut e = test_engine();
            let log: Log = Rc::new(RefCell::new(Vec::new()));
            log_double(&mut e, &log, UPDATE_STATE, 0);
            let c = e.mem.alloc(0x660);
            e.mem.set_u32(c + 0x520, wanted);
            e.call(0x00cd_4280, &args![0u32, c]);
            assert_eq!(*log.borrow(), vec![(UPDATE_STATE, vec![c])]);
            assert_eq!(e.mem.u32(c + 0x414), 0);
        }
    }

    #[test]
    fn the_jump_launches_the_character_upwards() {
        let mut e = test_engine();
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, UPDATE_STATE, 0);
        log_double(&mut e, &log, CONTROLLER_SLOT_94, 0);
        log_double(&mut e, &log, GET_POSITION, 0);
        log_double(&mut e, &log, EXTRACT_ANGLES, 0);
        e.register_double(X87_SQRT, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: x.sqrt(),
                ..Ret::default()
            }
        });
        // The proxy's linear velocity lives at proxy + 0x10.
        let proxy = e.mem.alloc(0x40);
        set_lanes(&mut e, proxy + 0x10, [1.0, 2.0, 3.0, 4.0]);
        e.register_double(GET_LINEAR_VELOCITY, move |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        // The controller's virtual function 0x94 returns a block with the direction at +0x10.
        let direction_block = e.mem.alloc(0x40);
        set_lanes(&mut e, direction_block + 0x10, [0.0, 0.0, 1.0, 0.0]);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x94, 0x0900_0094);
        e.register_double(0x0900_0094, move |_, _| Ret {
            eax: direction_block,
            ..Ret::default()
        });
        // The new state's virtual function 0x20.
        let state_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(state_vtable + 0x20, 0x0900_0020);
        let state = e.mem.alloc(0x10);
        e.mem.set_u32(state, state_vtable);
        log_double(&mut e, &log, 0x0900_0020, 0);
        let manager = e.mem.alloc(0x40);
        e.mem.set_u32(manager + 8 + 4 * 5, state);
        log_double(&mut e, &log, GET_CURRENT_STATE, state);
        let c = e.mem.alloc(0x660);
        e.mem.set_u32(c, vtable);
        e.mem.set_u32(c + 8, proxy);
        e.mem.set_u32(c + 0x3ec, manager);
        e.mem.set_u32(c + 0x3f0, 5);
        e.mem.set_u32(c + 0x520, 1);
        e.mem.set_u32(c + 0x414, 0xffff_ffff);
        e.mem.set_f32(c + 0x54c, 2.0);
        e.mem.set_f32(c + 0x540, -8.0);
        e.call(0x00cd_4280, &args![0u32, c]);
        let out = lanes(&e, c + 0x4f0);
        // Launch speed is -sqrt(2 * 8 * 2): the direction scaled by gravity has length 2.
        let speed = -(32.0f32.sqrt());
        assert!((out[0] - 1.0).abs() < 1e-5);
        assert!((out[1] - 2.0).abs() < 1e-5);
        assert!((out[2] - speed).abs() < 1e-3);
        assert!((out[3] - 4.0).abs() < 1e-5);
        assert_eq!(e.mem.u32(c + 0x520), 2);
        assert_eq!(e.mem.u32(c + 0x414), 0xfffd_feff);
        let calls = log.borrow();
        assert!(calls.contains(&(UPDATE_STATE, vec![c])));
        assert!(calls.contains(&(GET_CURRENT_STATE, vec![manager, 5])));
        assert!(calls.contains(&(0x0900_0020, vec![state, c])));
    }
}
