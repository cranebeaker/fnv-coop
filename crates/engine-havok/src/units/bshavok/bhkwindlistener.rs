//! `BSHavok/bhkwindlistener.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The class is `bhkWindListener` (RTTI of vtable `010c4b2c`, whose slot 0 is `00c74780`); the
//! destructor body is shared with `bhkWaterListener` (vtable `010c88b8`, RTTI-confirmed, slot 0
//! the same function).

#[allow(unused_imports)]
use crate::prelude::*;

/// Runs `f` on an exe-shaped frame: a function that aligns `ESP` (`AND ESP,0xfffffff0`) and
/// then reserves `below` bytes (its `SUB ESP` and pushes) has its frame `below` bytes under a
/// 16-byte boundary; `f` gets the frame's lowest address.
fn with_aligned_frame<R>(e: &mut Engine, below: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(below + 0x10, |e, raw| {
        let boundary = (raw.addr() + below + 0xf) & !0xf;
        f(e, boundary - below)
    })
}

/// The frame base [`with_aligned_frame`] picks for a block at `raw` (tests).
#[cfg(test)]
fn aligned_frame_at(raw: u32, below: u32) -> u32 {
    ((raw + below + 0xf) & !0xf) - below
}

/// Vtable of `bhkWindListener`.
const VTABLE_WIND_LISTENER: u32 = 0x010c_4b2c;
/// The destructor body (`005545f0`, thiscall on `this`).
const DESTRUCTOR_BODY: u32 = 0x0055_45f0;
/// `MemoryManager::Deallocate` (`00aa4060`, thiscall `(block)`) on the manager `011f6238`.
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;

// Translated from 00c74760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWindListener` constructor (RTTI of vtable `010c4b2c`): clears `+0xc` and `+0x10`, sets
/// the word at `+4` and the byte at `+8` to 1, stores the vtable. Answers `this`.
pub fn bhk_wind_listener_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 4, 1);
    e.mem.set_u8(me + 8, 1);
    e.mem.set_u32(me, VTABLE_WIND_LISTENER);
    this
}

// Translated from 00c74780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWindListener` scalar deleting destructor (slot 0 of vtable `010c4b2c`, shared with
/// `bhkWaterListener`; the map names it `bhkWaterListener::_scalar_deleting_destructor_`): runs
/// the destructor body `005545f0` and, when bit 0 of `flags` is set, hands the block to
/// `MemoryManager::Deallocate`. Answers `this`.
pub fn bhk_wind_listener_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The wind listener globals: two floats (`01267c14` the strength, `01267c18` the phase), the
/// floats `011b02dc` (the strength limit) and `011b02e0` (the period the angle wraps at), the
/// float `01064f60` (the second noise sample position), the `double` `010c4b20` the time step is
/// divided by and the `double` `010290b0` (0.25).
const WIND_STRENGTH: u32 = 0x0126_7c14;
const WIND_PHASE: u32 = 0x0126_7c18;
const WIND_STRENGTH_LIMIT: u32 = 0x011b_02dc;
const WIND_ANGLE_PERIOD: u32 = 0x011b_02e0;
const WIND_SECOND_SAMPLE: u32 = 0x0106_4f60;
const WIND_TIME_DIVISOR: u32 = 0x010c_4b20;
const WIND_QUARTER: u32 = 0x0102_90b0;
/// `_ftol2` (`00ec62c0`, the value in `ST0`, a leading `f64`), `hkRotation::setAxisAngle`
/// (`00cb2e30`, `this` = the 0x30-byte rotation, `(axis, angle)`), `hkVector4::setRotatedDir`
/// (`00c7f700`, `this` = the result, `(rotation, vector)`) and `hkpEntity::activate`
/// (`00c9c1d0`).
const FTOL: u32 = 0x00ec_62c0;
const ROTATION_SET_AXIS_ANGLE: u32 = 0x00cb_2e30;
const VECTOR_SET_ROTATED_DIR: u32 = 0x00c7_f700;
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;

/// `Update`'s frame under the 16-byte boundary: `SUB ESP,0x84` and three pushes.
const UPDATE_FRAME_BELOW: u32 = 0x90;

// Translated from 00c74570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c74570 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// `bhkWindListener::Update` (Xbox PDB), thiscall with one float stack word, the time step.
/// Does nothing when the byte at `+8` is 0 or the strength `01267c14` is 0. Otherwise, with
/// `k = 1 - trunc(dt / 0x010c4b20)` as a float, for each node of the list starting at `+0xc`
/// (the list ends at a node whose first word is 0; the second word links to the next): `body` is
/// the word at +8 of the object the node's first word points to; the strength plus the noise of
/// virtual slot 0x28 (called with the strength) times 0.25 is limited to `[0, 011b02dc]` and
/// multiplied by `k`; the phase `01267c18` plus the noise of slot 0x28 (called with the float at
/// `01064f60`) is wrapped into `[0, 011b02e0]` by one subtraction or addition of that period.
/// Then the rotation of that angle about (0, 0, 1) (`setAxisAngle`) turns the direction (0, the
/// limited strength, 0, 0) (`setRotatedDir`) and, when `body` is not null, the body is activated
/// and its motion (`body + 0xe0`) gets virtual slot 0x5c called with the time step and the
/// rotated direction.
pub fn bhk_wind_listener_update(e: &mut Engine, this: Ptr, dt: f32) {
    let this = this.addr();
    if e.mem.u8(this + 8) == 0 {
        return;
    }
    let strength = e.mem.u32(WIND_STRENGTH);
    // `FLDZ` / `FCOMP`: equal (and not a NaN) ends the update.
    if x87_m32(strength) == 0.0 {
        return;
    }
    let divisor: f64 = e.global(WIND_TIME_DIVISOR);
    let quotient = x87_div(x87_load32(dt.to_bits()), divisor);
    let truncated = e.call(FTOL, &args![quotient]).i32();
    let scale = x87_store32(f64::from(1i32.wrapping_sub(truncated)));
    let quarter: f64 = e.global(WIND_QUARTER);
    // The exe's frame (aligned: `SUB ESP,0x84` and three pushes under the boundary) holds the
    // axis at +0x20, the rotated direction at +0x30, the direction at +0x40 and the rotation at
    // +0x50 (00c746c1..00c7472e).
    with_aligned_frame(e, UPDATE_FRAME_BELOW, |e, frame| {
        let (axis, rotated, vector, rotation) =
            (frame + 0x20, frame + 0x30, frame + 0x40, frame + 0x50);
        let mut node = this + 0xc;
        while node != 0 && e.mem.u32(node) != 0 {
            let entry = e.mem.u32(node);
            let body = e.mem.u32(entry + 8);
            let noise = e.vcall(this, 0x28, &args![x87_quiet_bits(strength)]).f64();
            // `strength * 0.25 + noise` (`FADDP`), stored as a float.
            let mut limited = x87_store32(x87_add(noise, x87_mul(x87_load32(strength), quarter)));
            // `FLDZ` / `FCOM`: only an ordered "below zero" clamps.
            if 0.0 > x87_m32(limited) {
                limited = 0.0f32.to_bits();
            }
            let upper = x87_load32(e.mem.u32(WIND_STRENGTH_LIMIT));
            if upper < x87_load32(limited) {
                limited = x87_store32(upper);
            }
            let limited = x87_store32(x87_mul(x87_load32(limited), x87_m32(scale)));
            let sample = x87_quiet_bits(e.mem.u32(WIND_SECOND_SAMPLE));
            let noise = e.vcall(this, 0x28, &args![sample]).f64();
            let noise = x87_store32(noise);
            let mut angle = x87_store32(x87_add(x87_load32(e.mem.u32(WIND_PHASE)), x87_m32(noise)));
            let period = x87_load32(e.mem.u32(WIND_ANGLE_PERIOD));
            let current = x87_load32(angle);
            if period < current {
                angle = x87_store32(x87_sub(current, period));
            } else if 0.0 > current {
                angle = x87_store32(x87_add(current, period));
            }
            let angle = x87_store32(x87_load32(angle));
            for (lane, value) in [0.0f32, f32::from_bits(limited), 0.0, 0.0]
                .into_iter()
                .enumerate()
            {
                e.mem.set_f32(vector + 4 * lane as u32, value);
            }
            for (lane, value) in [0.0f32, 0.0, 1.0, 0.0].into_iter().enumerate() {
                e.mem.set_f32(axis + 4 * lane as u32, value);
            }
            e.call(ROTATION_SET_AXIS_ANGLE, &args![rotation, axis, angle]);
            e.call(VECTOR_SET_ROTATED_DIR, &args![rotated, rotation, vector]);
            if body != 0 {
                e.call(ENTITY_ACTIVATE, &args![body]);
                e.vcall(
                    body + 0xe0,
                    0x5c,
                    &args![x87_quiet_bits(dt.to_bits()), rotated],
                );
            }
            node = e.mem.u32(node + 4);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c74570, bhk_wind_listener_update(Ptr, f32)),
        entry!(0x00c74760, bhk_wind_listener_construct(Ptr) -> Ptr),
        entry!(0x00c74780, bhk_wind_listener_scalar_deleting_destructor(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn construct_sets_the_fields_and_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        for offset in 0..0x20 {
            e.mem.set_u8(this + offset, 0xee);
        }
        assert_eq!(e.call(0x00c74760, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_WIND_LISTENER);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(e.mem.u8(this + 8), 1);
        assert_eq!(e.mem.u8(this + 9), 0xee);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_on_bit_0() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<(u32, Vec<u32>)>>> = Rc::new(RefCell::new(Vec::new()));
        for address in [DESTRUCTOR_BODY, MEMORY_DEALLOCATE] {
            let kept = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                kept.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let this = 0x5000u32;
        assert_eq!(e.call(0x00c74780, &args![this, 0xfeu8]).u32(), this);
        assert_eq!(*log.borrow(), vec![(DESTRUCTOR_BODY, vec![this])]);
        assert_eq!(e.call(0x00c74780, &args![this, 1u8]).u32(), this);
        assert_eq!(
            log.borrow()[1..],
            [
                (DESTRUCTOR_BODY, vec![this]),
                (MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, this])
            ]
        );
    }
}

#[cfg(test)]
mod update_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>, Vec<f32>)>>>;

    const LISTENER_VTABLE: u32 = 0x0e00_0000;
    const NOISE: u32 = 0x0e00_0100;
    const MOTION_VTABLE: u32 = 0x0e00_0200;
    const MOTION_SLOT_5C: u32 = 0x0e00_0300;

    struct Scene {
        e: Engine,
        seen: Seen,
        listener: u32,
        body: u32,
    }

    fn scene(strength: f32, phase: f32) -> Scene {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(WIND_STRENGTH, strength);
        e.set_global(WIND_PHASE, phase);
        e.set_global(WIND_STRENGTH_LIMIT, 10.0f32);
        e.set_global(WIND_ANGLE_PERIOD, 6.0f32);
        e.set_global(WIND_SECOND_SAMPLE, 0.75f32);
        e.set_global(WIND_TIME_DIVISOR, 0.5f64);
        e.set_global(WIND_QUARTER, 0.25f64);
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        // The noise of the listener: 1.0 for the strength sample, 7.0 for the second sample.
        e.put_vtable(LISTENER_VTABLE, &[0; 0x0c]);
        e.mem.set_u32(LISTENER_VTABLE + 0x28, NOISE);
        e.register_double(NOISE, move |_, a| {
            let value = if f32::from_bits(a[1]) == 0.75 {
                7.0
            } else {
                1.0
            };
            Ret {
                st0: value,
                ..Ret::default()
            }
        });
        let listener = e.mem.alloc(0x40);
        e.mem.set_u32(listener, LISTENER_VTABLE);
        e.mem.set_u8(listener + 8, 1);
        let body = e.mem.alloc(0x300);
        e.put_vtable(MOTION_VTABLE, &[0; 0x18]);
        e.mem.set_u32(MOTION_VTABLE + 0x5c, MOTION_SLOT_5C);
        e.mem.set_u32(body + 0xe0, MOTION_VTABLE);
        let log = Rc::clone(&seen);
        e.register_double(MOTION_SLOT_5C, move |e, a| {
            let floats = (0..4).map(|lane| e.mem.f32(a[2] + 4 * lane)).collect();
            log.borrow_mut().push((MOTION_SLOT_5C, a.to_vec(), floats));
            Ret::default()
        });
        for (address, name) in [(ENTITY_ACTIVATE, 0), (FTOL, 1)] {
            let log = Rc::clone(&seen);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec(), Vec::new()));
                // `_ftol2`: the value truncated.
                let value =
                    f64::from_bits(u64::from(a[0]) | u64::from(*a.get(1).unwrap_or(&0)) << 32);
                Ret {
                    eax: if name == 1 { value as i32 as u32 } else { 0 },
                    ..Ret::default()
                }
            });
        }
        let log = Rc::clone(&seen);
        e.register_double(ROTATION_SET_AXIS_ANGLE, move |e, a| {
            let floats = (0..4).map(|lane| e.mem.f32(a[1] + 4 * lane)).collect();
            log.borrow_mut()
                .push((ROTATION_SET_AXIS_ANGLE, a.to_vec(), floats));
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(VECTOR_SET_ROTATED_DIR, move |e, a| {
            let floats: Vec<f32> = (0..4).map(|lane| e.mem.f32(a[2] + 4 * lane)).collect();
            log.borrow_mut()
                .push((VECTOR_SET_ROTATED_DIR, a.to_vec(), floats.clone()));
            // The rotated direction is the input scaled by 2.
            for (lane, value) in floats.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, value * 2.0);
            }
            Ret::default()
        });
        Scene {
            e,
            seen,
            listener,
            body,
        }
    }

    fn calls(seen: &Seen, address: u32) -> Vec<(Vec<u32>, Vec<f32>)> {
        seen.borrow()
            .iter()
            .filter(|(a, _, _)| *a == address)
            .map(|(_, words, floats)| (words.clone(), floats.clone()))
            .collect()
    }

    /// The first list node (inline at +0xc) holding an object whose +8 word is `body`.
    fn add_node(s: &mut Scene, body: u32) {
        let entry = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(entry + 8, body);

        // The first list node is inline in the listener: the element, then the link.
        s.e.mem.set_u32(s.listener + 0xc, entry);
    }

    #[test]
    fn each_node_gets_the_rotated_wind_direction() {
        let mut s = scene(2.0, 1.5);
        let body = s.body;
        add_node(&mut s, body);
        s.e.call(0x00c7_4570, &args![s.listener, 1.0f32]);
        // dt / 0.5 = 2, k = 1 - 2 = -1.
        assert_eq!(calls(&s.seen, FTOL)[0].0[0..2], [0, 0x4000_0000]);
        // strength 2 * 0.25 + noise 1 = 1.5, within [0, 10], times k = -1.5? The lower limit
        // is applied before the scaling, so the direction's y is -1.5.
        let rotation = &calls(&s.seen, VECTOR_SET_ROTATED_DIR)[0];
        assert_eq!(rotation.1, vec![0.0, -1.5, 0.0, 0.0]);
        // The phase 1.5 plus the second noise 7.0 = 8.5 is above the period 6: wrapped to 2.5.
        let axis_angle = &calls(&s.seen, ROTATION_SET_AXIS_ANGLE)[0];
        assert_eq!(axis_angle.1, vec![0.0, 0.0, 1.0, 0.0]);
        assert_eq!(f32::from_bits(axis_angle.0[2]), 2.5);
        // The body is activated and its motion gets the time step and the rotated direction.
        assert_eq!(calls(&s.seen, ENTITY_ACTIVATE)[0].0, vec![s.body]);
        let motion = &calls(&s.seen, MOTION_SLOT_5C)[0];
        assert_eq!(motion.0[0], s.body + 0xe0);
        assert_eq!(f32::from_bits(motion.0[1]), 1.0);
        assert_eq!(motion.1, vec![0.0, -3.0, 0.0, 0.0]);
    }

    #[test]
    fn the_wind_vectors_sit_at_the_exe_frame_offsets() {
        let mut s = scene(2.0, 1.5);
        let body = s.body;
        add_node(&mut s, body);
        // The frame block is the next stack local: the axis at +0x20, the rotated direction at
        // +0x30, the direction at +0x40 and the rotation at +0x50.
        let raw = s.e.next_stack_local(UPDATE_FRAME_BELOW + 0x10);
        let frame = aligned_frame_at(raw, UPDATE_FRAME_BELOW);
        s.e.call(0x00c7_4570, &args![s.listener, 1.0f32]);
        let axis_angle = &calls(&s.seen, ROTATION_SET_AXIS_ANGLE)[0];
        assert_eq!(axis_angle.0[..2], [frame + 0x50, frame + 0x20]);
        let rotated = &calls(&s.seen, VECTOR_SET_ROTATED_DIR)[0];
        assert_eq!(rotated.0, vec![frame + 0x30, frame + 0x50, frame + 0x40]);
        assert_eq!(calls(&s.seen, MOTION_SLOT_5C)[0].0[2], frame + 0x30);
    }

    #[test]
    fn a_zero_strength_or_a_disabled_listener_does_nothing() {
        let mut s = scene(0.0, 1.5);
        let body = s.body;
        add_node(&mut s, body);
        s.e.call(0x00c7_4570, &args![s.listener, 1.0f32]);
        assert!(s.seen.borrow().is_empty());
        let mut s = scene(2.0, 1.5);
        let body = s.body;
        add_node(&mut s, body);
        s.e.mem.set_u8(s.listener + 8, 0);
        s.e.call(0x00c7_4570, &args![s.listener, 1.0f32]);
        assert!(s.seen.borrow().is_empty());
    }

    #[test]
    fn the_strength_is_clamped_and_a_null_body_is_not_activated() {
        let mut s = scene(100.0, -1.0);
        add_node(&mut s, 0);
        s.e.call(0x00c7_4570, &args![s.listener, 0.0f32]);
        // k = 1 - 0 = 1; 100 * 0.25 + 1 = 26 is limited to 10.
        assert_eq!(
            calls(&s.seen, VECTOR_SET_ROTATED_DIR)[0].1,
            vec![0.0, 10.0, 0.0, 0.0]
        );
        // The phase -1 + 7 = 6 is not above the period 6 and not below zero: kept.
        assert_eq!(
            f32::from_bits(calls(&s.seen, ROTATION_SET_AXIS_ANGLE)[0].0[2]),
            6.0
        );
        assert!(calls(&s.seen, ENTITY_ACTIVATE).is_empty());
        assert!(calls(&s.seen, MOTION_SLOT_5C).is_empty());
    }

    #[test]
    fn a_negative_angle_wraps_up_by_the_period() {
        let mut s = scene(1.0, -20.0);
        let body = s.body;
        add_node(&mut s, body);
        s.e.call(0x00c7_4570, &args![s.listener, 0.0f32]);
        // -20 + 7 = -13, plus the period 6 once: -7.
        assert_eq!(
            f32::from_bits(calls(&s.seen, ROTATION_SET_AXIS_ANGLE)[0].0[2]),
            -7.0
        );
    }
}
