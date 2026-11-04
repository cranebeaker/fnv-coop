//! `(unplaced)/run_00ca9580` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Two floats the constructor copies from `.rdata` (`FLD`/`FSTP`, so a
/// signalling NaN comes out quiet): stored at +0x20 and +0x1C.
const FLOAT_AT_0X20: u32 = 0x0101_7868;
const FLOAT_AT_0X1C: u32 = 0x010c_8190;
/// The vtable the constructor installs.
const VTABLE: u32 = 0x010c_8198;
/// `__thiscall` call the constructor ends with, on the object stored at +0xC
/// (`00c90510`, `hkreferencedobject.cpp`, no arguments).
const REFERENCED_OBJECT_ADD_REFERENCE: u32 = 0x00c9_0510;

// Translated from 00ca9580 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Constructor (`__thiscall`, two arguments): sets the word at
/// +6 to 1, +0xC to `owner`, the vtable, +0x10 and +0x14 to `value`, +8 to 0,
/// the byte at +0x18 to 1, the float at +0x20 from `01017868`, the float at
/// +0x1C from `010c8190` and the float at +0x24 to 1.0; then calls `00c90510`
/// on the object at +0xC (`owner`). Returns `this`.
pub fn fn_00ca9580(e: &mut Engine, this: Ptr, owner: u32, value: u32) -> Ptr {
    let base = this.addr();
    let first = x87_quiet_bits(e.global::<u32>(FLOAT_AT_0X20));
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 0xc, owner);
    e.mem.set_u32(base, VTABLE);
    e.mem.set_u32(base + 0x10, value);
    e.mem.set_u32(base + 0x14, value);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u8(base + 0x18, 1);
    e.mem.set_u32(base + 0x20, first);
    let second = x87_quiet_bits(e.global::<u32>(FLOAT_AT_0X1C));
    let referenced = e.mem.u32(base + 0xc);
    e.mem.set_u32(base + 0x1c, second);
    e.mem.set_u32(base + 0x24, 1.0f32.to_bits());
    e.call(REFERENCED_OBJECT_ADD_REFERENCE, &args![referenced]);
    this
}

// Translated from 00ca9600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall` getter: the word at `this + 0x10`.
pub fn fn_00ca9600(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x10)
}

/// `hkBaseObject`'s vtable (`0102e388`; RTTI `.?AVhkBaseObject@@`), which
/// the destructor leaves in place.
const VTABLE_HK_BASE_OBJECT: u32 = 0x0102_e388;
/// `00c905b0`: `__thiscall` on the object at +0xC, no arguments (the
/// counterpart of [`REFERENCED_OBJECT_ADD_REFERENCE`] in `hkreferencedobject.cpp`).
const REFERENCED_OBJECT_REMOVE_REFERENCE: u32 = 0x00c9_05b0;

// Translated from 00ca95e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. The destructor of the object [`fn_00ca9580`] constructs
/// (its vtable's RTTI is `.?AVhkpCharacterContext@@`): sets that vtable,
/// calls `00c905b0` on the object at +0xC, then sets `hkBaseObject`'s
/// vtable.
pub fn fn_00ca95e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let referenced = e.mem.u32(base + 0xc);
    e.mem.set_u32(base, VTABLE);
    e.call(REFERENCED_OBJECT_REMOVE_REFERENCE, &args![referenced]);
    e.mem.set_u32(base, VTABLE_HK_BASE_OBJECT);
}

/// The state-manager lookup `00cbc510` (named `bhkCharacterController::GetCurrentState` in
/// the engine map; thiscall on the manager: a state type; returns the state object).
const STATE_MANAGER_GET_STATE: u32 = 0x00cb_c510;

/// Fields of `hkpCharacterContext` (Xbox PDB, 0x28 bytes; offsets as the PC code uses them).
const CONTEXT_STATE_MANAGER: u32 = 0x0c;
const CONTEXT_CURRENT_STATE: u32 = 0x10;
const CONTEXT_PREVIOUS_STATE: u32 = 0x14;
const CONTEXT_FILTER_ENABLE: u32 = 0x18;
const CONTEXT_MAX_LINEAR_ACCELERATION: u32 = 0x1c;
const CONTEXT_MAX_LINEAR_VELOCITY: u32 = 0x20;
const CONTEXT_GAIN: u32 = 0x24;
/// `hkpCharacterInput::m_stepInfo.m_deltaTime` and `m_invDeltaTime` (Xbox PDB `hkStepInfo`
/// at +0x80) and `m_velocity` (+0xa0).
const INPUT_DELTA_TIME: u32 = 0x88;
const INPUT_INV_DELTA_TIME: u32 = 0x8c;
const INPUT_VELOCITY: u32 = 0xa0;

/// The four lanes of the 16-byte vector at `addr` (`MOVAPS` load).
fn load_vector(e: &Engine, addr: u32) -> crate::sse::Xmm {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

/// Stores four lanes at `addr` (`MOVAPS` store, bits kept).
fn store_vector(e: &mut Engine, addr: u32, v: crate::sse::Xmm) {
    for (lane, x) in v.iter().enumerate() {
        e.mem.set_u32(addr + 4 * lane as u32, x.to_bits());
    }
}

/// The exe's inlined length of the first three lanes: `MULPS v,v`, then
/// `SQRTSS(z² + (y² + x²))` with `ADDSS` in that order.
fn length3(v: crate::sse::Xmm) -> f32 {
    use crate::sse::{add, mulps, sqrt};
    let squares = mulps(v, v);
    sqrt(add(squares[2], add(squares[1], squares[0])))
}

/// One float broadcast to four lanes (`SHUFPS x,x,0`).
fn broadcast(x: f32) -> crate::sse::Xmm {
    [x; 4]
}

// Translated from 00ca9610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; on `hkpCharacterContext` (the class of the vtable its
/// constructor `00ca9580` installs): changes the current state. The old state's object
/// (`00cbc510` on the state manager, +0xc) gets vtable slot +0x14 with `this`, `state` and
/// the two other words; `m_currentState` (+0x10) becomes `state`; then the new state's object
/// gets slot +0x10 with `this`, the old state and the same two words. `EAX` is left as the
/// last call's (passed on).
pub fn fn_00ca9610(e: &mut Engine, this: Ptr, state: u32, first: u32, second: u32) -> u32 {
    let this = this.addr();
    let manager = e.mem.u32(this + CONTEXT_STATE_MANAGER);
    let old = e.mem.u32(this + CONTEXT_CURRENT_STATE);
    let leaving = e.call(STATE_MANAGER_GET_STATE, &args![manager, old]).u32();
    e.vcall(leaving, 0x14, &args![this, state, first, second]);
    let manager = e.mem.u32(this + CONTEXT_STATE_MANAGER);
    e.mem.set_u32(this + CONTEXT_CURRENT_STATE, state);
    let entering = e
        .call(STATE_MANAGER_GET_STATE, &args![manager, state])
        .u32();
    e.vcall(entering, 0x10, &args![this, old, first, second])
        .u32()
}

// Translated from 00ca96a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; on `hkpCharacterContext`: computes the output velocity (the
/// 16-byte vector at `output`) from an `hkpCharacterInput`. The output starts as the input's
/// velocity (+0xa0); the current state's object (`00cbc510`) gets vtable slot +0x1c with
/// `this`, `input`, `output`, and when the current state equals the previous one (both
/// re-read) also slot +0x18; the previous state becomes the current one. With
/// `m_filterEnable` (+0x18) set, the result is filtered: a length (first three lanes, SSE)
/// above `m_maxLinearVelocity` (+0x20, x87 compare, NaN never above) scales all four lanes
/// by `max / length` (x87 `FDIVR`, stored as a float); the acceleration
/// `(output - input velocity) * invDeltaTime` (+0x8c) gets its length divided by
/// `m_maxLinearAcceleration` (+0x1c, x87); the output becomes the input velocity plus that
/// acceleration times `deltaTime * m_gain` (+0x88, +0x24, x87), divided further by the
/// ratio when the ratio is above 1 (x87 `FCOM`, NaN never above) and the current state is
/// not 1.
pub fn fn_00ca96a0(e: &mut Engine, this: Ptr, input: Ptr, output: Ptr) {
    use crate::sse::{addps, mulps, subps};
    let this = this.addr();
    let input = input.addr();
    let output = output.addr();
    let velocity = load_vector(e, input + INPUT_VELOCITY);
    store_vector(e, output, velocity);
    let manager = e.mem.u32(this + CONTEXT_STATE_MANAGER);
    let current = e.mem.u32(this + CONTEXT_CURRENT_STATE);
    let state = e
        .call(STATE_MANAGER_GET_STATE, &args![manager, current])
        .u32();
    e.vcall(state, 0x1c, &args![this, input, output]);
    let current = e.mem.u32(this + CONTEXT_CURRENT_STATE);
    if current == e.mem.u32(this + CONTEXT_PREVIOUS_STATE) {
        let manager = e.mem.u32(this + CONTEXT_STATE_MANAGER);
        let state = e
            .call(STATE_MANAGER_GET_STATE, &args![manager, current])
            .u32();
        e.vcall(state, 0x18, &args![this, input, output]);
    }
    let filter = e.mem.u8(this + CONTEXT_FILTER_ENABLE);
    let current = e.mem.u32(this + CONTEXT_CURRENT_STATE);
    e.mem.set_u32(this + CONTEXT_PREVIOUS_STATE, current);
    if filter == 0 {
        return;
    }
    // Clamp the speed.
    let wanted = load_vector(e, output);
    let length = x87_load32(length3(wanted).to_bits());
    let max_velocity_bits = e.mem.u32(this + CONTEXT_MAX_LINEAR_VELOCITY);
    let max_velocity = x87_load32(max_velocity_bits);
    if max_velocity < length {
        let scale = x87_div(x87_m32(max_velocity_bits), length);
        let scale = f32::from_bits(x87_store32(scale));
        store_vector(e, output, mulps(broadcast(scale), wanted));
    }
    // Limit the acceleration.
    let inv_delta = x87_quiet(e.mem.f32(input + INPUT_INV_DELTA_TIME));
    let velocity = load_vector(e, input + INPUT_VELOCITY);
    let filtered = load_vector(e, output);
    let acceleration = mulps(broadcast(inv_delta), subps(filtered, velocity));
    let accel_length = x87_load32(length3(acceleration).to_bits());
    let max_accel = e.mem.u32(this + CONTEXT_MAX_LINEAR_ACCELERATION);
    let ratio = x87_div(accel_length, x87_m32(max_accel));
    store_vector(e, output, velocity);
    let ratio = x87_load32(x87_store32(ratio));
    let delta = x87_load32(e.mem.u32(input + INPUT_DELTA_TIME));
    let gain = x87_m32(e.mem.u32(this + CONTEXT_GAIN));
    let step = if ratio > 1.0 && e.mem.u32(this + CONTEXT_CURRENT_STATE) != 1 {
        x87_div(x87_mul(delta, gain), ratio)
    } else {
        x87_mul(delta, gain)
    };
    let step = f32::from_bits(x87_store32(step));
    let result = addps(mulps(broadcast(step), acceleration), velocity);
    store_vector(e, output, result);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca9580, fn_00ca9580(Ptr, u32, u32) -> Ptr),
        entry!(0x00ca9600, fn_00ca9600(Ptr) -> u32),
        entry!(0x00ca_95e0, fn_00ca95e0(Ptr)),
        entry!(0x00ca_9610, fn_00ca9610(Ptr, u32, u32, u32) -> u32),
        entry!(0x00ca_96a0, fn_00ca96a0(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(e: &Engine, addr: u32) -> Log {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .cloned()
            .collect()
    }

    /// A state object for each state type `t` at `0x0050_0000 + t`, whose vtable slots
    /// +0x10..+0x1c are doubles at `0x0079_0010..`.
    fn states(e: &mut Engine) {
        let vtable = e.mem.alloc(0x20);
        for slot in [0x10u32, 0x14, 0x18, 0x1c] {
            e.mem.set_u32(vtable + slot, 0x0079_0000 + slot);
            e.register(0x0079_0000 + slot, |_, a| {
                (0x100 + a.len() as u32).into_ret()
            });
        }
        e.map(0x0050_0000, 0x10);
        for t in 0..4 {
            e.mem.set_u32(0x0050_0000 + 4 * t, vtable);
        }
        e.register(STATE_MANAGER_GET_STATE, |_, a| {
            (0x0050_0000 + 4 * a[1]).into_ret()
        });
    }

    #[test]
    fn set_state_leaves_the_old_and_enters_the_new() {
        let mut e = Engine::new();
        states(&mut e);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0xc, 0x6000);
        e.mem.set_u32(this + 0x10, 1);
        e.call_log = Some(vec![]);
        let r = e
            .call(0x00ca_9610, &args![this, 2u32, 0x77u32, 0x88u32])
            .u32();
        assert_eq!(r, 0x105);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        assert_eq!(
            calls_to(&e, STATE_MANAGER_GET_STATE),
            vec![
                (STATE_MANAGER_GET_STATE, vec![0x6000, 1]),
                (STATE_MANAGER_GET_STATE, vec![0x6000, 2])
            ]
        );
        assert_eq!(
            calls_to(&e, 0x0079_0014),
            vec![(0x0079_0014, vec![0x0050_0004, this, 2, 0x77, 0x88])]
        );
        assert_eq!(
            calls_to(&e, 0x0079_0010),
            vec![(0x0079_0010, vec![0x0050_0008, this, 1, 0x77, 0x88])]
        );
    }

    fn context(e: &mut Engine, filter: bool, current: u32, previous: u32) -> u32 {
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0xc, 0x6000);
        e.mem.set_u32(this + 0x10, current);
        e.mem.set_u32(this + 0x14, previous);
        e.mem.set_u8(this + 0x18, u8::from(filter));
        e.mem.set_f32(this + 0x1c, 10.0);
        e.mem.set_f32(this + 0x20, 5.0);
        e.mem.set_f32(this + 0x24, 2.0);
        this
    }

    fn input(e: &mut Engine, velocity: [f32; 4]) -> u32 {
        let input = e.mem.alloc(0xd0);
        e.mem.set_f32(input + 0x88, 0.5);
        e.mem.set_f32(input + 0x8c, 2.0);
        store_vector(e, input + 0xa0, velocity);
        input
    }

    #[test]
    fn update_without_filter_copies_the_velocity_and_runs_the_states() {
        for (previous, both) in [(0u32, true), (3, false)] {
            let mut e = Engine::new();
            states(&mut e);
            let this = context(&mut e, false, 0, previous);
            let input = input(&mut e, [1.0, 2.0, 3.0, 4.0]);
            let output = e.mem.alloc(16);
            e.call_log = Some(vec![]);
            e.call(0x00ca_96a0, &args![this, input, output]);
            assert_eq!(load_vector(&e, output), [1.0, 2.0, 3.0, 4.0]);
            assert_eq!(
                calls_to(&e, 0x0079_001c),
                vec![(0x0079_001c, vec![0x0050_0000, this, input, output])]
            );
            assert_eq!(calls_to(&e, 0x0079_0018).len(), usize::from(both));
            assert_eq!(e.mem.u32(this + 0x14), 0);
        }
    }

    #[test]
    fn update_with_filter_limits_speed_and_acceleration() {
        // The state's slot +0x1c writes a wanted velocity of (6, 8, 0, 0): length 10 > 5,
        // scaled by 0.5 to (3, 4, 0, 0). From a velocity of 0 and invDeltaTime 2 the
        // acceleration is (6, 8, 0, 0), length 10, ratio 10 / 10 = 1 (not above 1), so the
        // step is deltaTime * gain = 1 and the output (6, 8, 0, 0).
        let mut e = Engine::new();
        states(&mut e);
        e.register(0x0079_001c, |e, a| {
            store_vector(e, a[3], [6.0, 8.0, 0.0, 0.0]);
            Ret::default()
        });
        let this = context(&mut e, true, 2, 9);
        let input_block = input(&mut e, [0.0; 4]);
        let output = e.mem.alloc(16);
        e.call(0x00ca_96a0, &args![this, input_block, output]);
        assert_eq!(load_vector(&e, output), [6.0, 8.0, 0.0, 0.0]);

        // Above the acceleration limit (max 2.5: ratio 4) and not state 1: the step is
        // 1 / 4, so (1.5, 2, 0, 0); in state 1 the step stays 1.
        for (state, want) in [(2u32, [1.5f32, 2.0, 0.0, 0.0]), (1, [6.0, 8.0, 0.0, 0.0])] {
            let this = context(&mut e, true, state, 9);
            e.mem.set_f32(this + 0x1c, 2.5);
            e.call(0x00ca_96a0, &args![this, input_block, output]);
            assert_eq!(load_vector(&e, output), want);
        }

        // A NaN speed limit never clamps: the acceleration (12, 16) has length 20, ratio 2.
        let this = context(&mut e, true, 2, 9);
        e.mem.set_u32(this + 0x20, 0x7fc0_0000);
        e.call(0x00ca_96a0, &args![this, input_block, output]);
        assert_eq!(load_vector(&e, output), [6.0, 8.0, 0.0, 0.0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ca9580_fills_the_fields_and_calls_the_owner() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x010c_8000, 0x1000);
        // A signalling NaN comes out quiet; the other value is kept.
        e.set_global(FLOAT_AT_0X20, 0x7f80_0001u32);
        e.set_global(FLOAT_AT_0X1C, 624.0f32.to_bits());
        e.register(REFERENCED_OBJECT_ADD_REFERENCE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x28);
        for offset in 0..10 {
            e.mem.set_u32(this + 4 * offset, 0xffff_ffff);
        }
        let result = e.call(
            0x00ca_9580,
            &args![Ptr::<()>::new(this), 0x6000u32, 0x7000u32],
        );
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x6000);
        assert_eq!(e.mem.u32(this + 0x10), 0x7000);
        assert_eq!(e.mem.u32(this + 0x14), 0x7000);
        assert_eq!(e.mem.u8(this + 0x18), 1);
        assert_eq!(e.mem.u8(this + 0x19), 0xff);
        assert_eq!(e.mem.u32(this + 0x1c), 624.0f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x20), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x24), 1.0f32.to_bits());
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(REFERENCED_OBJECT_ADD_REFERENCE, vec![0x6000])));
    }

    #[test]
    fn fn_00ca9600_reads_the_word_at_0x10() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0xabcd_1234);
        assert_eq!(
            e.call(0x00ca_9600, &args![Ptr::<()>::new(this)]).u32(),
            0xabcd_1234
        );
    }
}

#[cfg(test)]
mod b040_tests {
    use super::*;

    #[test]
    fn fn_00ca95e0_releases_the_owner_and_leaves_the_base_vtable() {
        let mut e = Engine::new();
        e.register(REFERENCED_OBJECT_REMOVE_REFERENCE, |e, _| {
            // The class's own vtable is in place while the owner is released.
            assert_eq!(e.mem.u32(e.mem.u32(0x7ff0)), VTABLE);
            Ret::default()
        });
        let this = e.mem.alloc(0x28);
        e.map(0x7000, 0x1000);
        e.mem.set_u32(0x7ff0, this);
        e.mem.set_u32(this + 0xc, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x00ca_95e0, &args![Ptr::<()>::new(this)]);
        assert_eq!(e.mem.u32(this), VTABLE_HK_BASE_OBJECT);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log
            .iter()
            .filter(|c| c.0 == REFERENCED_OBJECT_REMOVE_REFERENCE)
            .collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x6000]);
    }
}
