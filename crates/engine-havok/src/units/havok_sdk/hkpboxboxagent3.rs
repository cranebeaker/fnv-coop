//! `Havok SDK/hkpboxboxagent3.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SSE code computes lane by lane in `f32`; floats copied through `FLD`/`FSTP` go through
//! [`x87_quiet`]. The profiling records (`RDTSC` time stamps appended to the thread's monitor
//! stream) are those of [`profile_record`](super::hkpagent1nmachine).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpagent1nmachine::profile_record;

/// `hkpBoxBoxCollisionDetection::calcManifold` (named in the Ghidra listing, `00db2960`,
/// thiscall, one stack argument).
const CALC_MANIFOLD: u32 = 0x00db_2960;
/// Label of the first profiling record ("TtBoxBox3") and of the second (the bytes `Et`).
const START_LABEL: u32 = 0x010c_f1b4;
const END_LABEL: u32 = 0x010c_41f8;
/// A `float` constant (0.01) stored in the detection object.
const SKIN_WIDTH: u32 = 0x0101_3ea4;

/// Four `float` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store, or four word moves), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

// Translated from 00d3fe90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBoxBoxAgent3::process` (named in the engine map, cdecl): the agent function of two boxes,
/// called with the agent input record `input` (words `+0` and `+4` the two collision bodies,
/// `+0xc` the collision input whose float at `+8` is a tolerance, `+0x10` a word passed on, the
/// relative transform at `+0x20`), the agent entry `entry`, the manifold area `output`, an
/// unused word and `state`. It builds a `hkpBoxBoxCollisionDetection` object in its frame (the
/// bodies, their motions, the transform, the boxes' half extents grown by their convex radii and
/// by the tolerance, the tolerance and a constant 0.01), has it compute the manifold into
/// `output`, copies the manifold's byte `+0x21` to the entry's byte `+2` and returns
/// `output + 0x40`. The thread's monitor stream gets a record before and after.
///
/// The object's word at `+0x1c` is never written (a stack slot of the exe).
pub fn hk_box_box_agent3_process(
    e: &mut Engine,
    input: Ptr,
    entry: Ptr,
    output: Ptr,
    _unused_4: u32,
    state: Ptr,
) -> Ptr {
    profile_record(e, START_LABEL);
    let (input, entry, output) = (input.addr(), entry.addr(), output.addr());
    e.with_stack(0x1a0, |e, frame| {
        // The detection object (the exe's `s + 0x70`).
        let this = frame.addr() + 0x70;
        let body_a = e.mem.u32(input);
        let shape_a = e.mem.u32(body_a);
        let radius_a = x87_quiet(e.mem.f32(shape_a + 0x10));
        let half_a = load_vec(e, shape_a + 0x20);
        let motion_a = e.mem.u32(body_a + 8);
        let body_b = e.mem.u32(input + 4);
        let shape_b = e.mem.u32(body_b);
        let half_b = load_vec(e, shape_b + 0x20);
        let radius_b = x87_quiet(e.mem.f32(shape_b + 0x10));
        let collision_input = e.mem.u32(input + 0xc);
        let motion_b = e.mem.u32(body_b + 8);
        let tolerance = x87_quiet(e.mem.f32(collision_input + 8));
        e.mem.set_u32(this, body_a);
        e.mem.set_u32(this + 8, collision_input);
        let word = e.mem.u32(input + 0x10);
        e.mem.set_u32(this + 0xc, word);
        e.mem.set_u32(this + 0x10, state.addr());
        for quarter in 0..4 {
            copy_vec(
                e,
                input + 0x20 + 0x10 * quarter,
                this + 0x20 + 0x10 * quarter,
            );
        }
        let grown_a: Vec4 = std::array::from_fn(|lane| radius_a + half_a[lane]);
        let grown_b: Vec4 = std::array::from_fn(|lane| radius_b + half_b[lane]);
        store_vec(e, this + 0x60, grown_a);
        e.mem.set_u32(this + 4, body_b);
        e.mem.set_u32(this + 0x14, motion_a);
        e.mem.set_u32(this + 0x18, motion_b);
        store_vec(e, this + 0x70, grown_b);
        store_vec(e, this + 0x80, [tolerance; 4]);
        e.mem.set_f32(this + 0xb0, tolerance);
        let skin = x87_quiet(e.mem.f32(SKIN_WIDTH));
        e.mem.set_f32(this + 0xb4, skin);
        let widened_a: Vec4 = std::array::from_fn(|lane| tolerance + grown_a[lane]);
        let widened_b: Vec4 = std::array::from_fn(|lane| tolerance + grown_b[lane]);
        store_vec(e, this + 0x90, widened_a);
        store_vec(e, this + 0xa0, widened_b);
        e.call(CALC_MANIFOLD, &args![this, output]);
    });
    let flag = e.mem.u8(output + 0x21);
    e.mem.set_u8(entry + 2, flag);
    profile_record(e, END_LABEL);
    Ptr::new(output + 0x40)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d3_fe90,
        hk_box_box_agent3_process(Ptr, Ptr, Ptr, u32, Ptr) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::super::hkpagent1nmachine::{MONITOR_SLOT, TLS_GET_VALUE};

    /// The words of the detection object when the manifold double ran.
    type Objects = Rc<RefCell<Vec<(Vec<u32>, u32)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// The monitor stream (cursor at `+4`, limit at `+0xc`) and the TLS slot.
    fn monitor_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0xf0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        stream
    }

    /// A body whose shape has convex radius `radius` and half extents `half`, and a motion.
    fn box_body(e: &mut Engine, radius: f32, half: [f32; 4]) -> (u32, u32) {
        let shape = e.mem.alloc(0x40);
        e.mem.set_f32(shape + 0x10, radius);
        for (lane, value) in half.into_iter().enumerate() {
            e.mem.set_f32(shape + 0x20 + 4 * lane as u32, value);
        }
        let motion = e.mem.alloc(0x20);
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, motion);
        (body, motion)
    }

    /// Registers the manifold double: it records the words of the object (0xb8 bytes) and the
    /// output pointer and sets the manifold byte `+0x21`.
    fn install_manifold(e: &mut Engine) -> Objects {
        let seen: Objects = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(CALC_MANIFOLD, move |e, a| {
            let words = (0..0x2e).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            log.borrow_mut().push((words, a[1]));
            e.mem.set_u8(a[1] + 0x21, 0x5a);
            Ret::default()
        });
        seen
    }

    #[test]
    fn process_builds_the_detection_object_and_returns_the_end_of_the_manifold() {
        let mut e = engine();
        let stream = monitor_stream(&mut e);
        let seen = install_manifold(&mut e);
        e.mem.set_f32(SKIN_WIDTH, 0.01);
        let (a, motion_a) = box_body(&mut e, 0.5, [1.0, 2.0, 3.0, 4.0]);
        let (b, motion_b) = box_body(&mut e, 0.25, [10.0, 20.0, 30.0, 40.0]);
        let collision_input = e.mem.alloc(0x20);
        e.mem.set_f32(collision_input + 8, 0.125);
        let input = e.mem.alloc(0x80);
        e.mem.set_u32(input, a);
        e.mem.set_u32(input + 4, b);
        e.mem.set_u32(input + 0xc, collision_input);
        e.mem.set_u32(input + 0x10, 0x4444);
        for word in 8..0x18 {
            e.mem.set_u32(input + 4 * word, 0x2000 + word);
        }
        let entry = e.mem.alloc(0x40);
        let output = e.mem.alloc(0x80);
        let end = e
            .call(0x00d3_fe90, &args![input, entry, output, 0u32, 0x5757u32])
            .u32();
        assert_eq!(end, output + 0x40);
        assert_eq!(e.mem.u8(entry + 2), 0x5a);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (words, manifold) = &seen[0];
        assert_eq!(*manifold, output);
        let float = |word: usize| f32::from_bits(words[word]);
        assert_eq!(words[0..2], [a, b]);
        assert_eq!(words[2..6], [collision_input, 0x4444, 0x5757, motion_a]);
        assert_eq!(words[6], motion_b);
        // The transform words are copied unchanged.
        let expected: Vec<u32> = (8..0x18).map(|word| 0x2000 + word).collect();
        assert_eq!(words[8..0x18], expected[..]);
        // Half extents grown by the radii, then by the tolerance 0.125.
        let grown_a: Vec<f32> = (0x18..0x1c).map(float).collect();
        let grown_b: Vec<f32> = (0x1c..0x20).map(float).collect();
        assert_eq!(grown_a, vec![1.5, 2.5, 3.5, 4.5]);
        assert_eq!(grown_b, vec![10.25, 20.25, 30.25, 40.25]);
        let tolerance: Vec<f32> = (0x20..0x24).map(float).collect();
        assert_eq!(tolerance, vec![0.125; 4]);
        let widened_a: Vec<f32> = (0x24..0x28).map(float).collect();
        let widened_b: Vec<f32> = (0x28..0x2c).map(float).collect();
        assert_eq!(widened_a, vec![1.625, 2.625, 3.625, 4.625]);
        assert_eq!(widened_b, vec![10.375, 20.375, 30.375, 40.375]);
        assert_eq!(float(0x2c), 0.125);
        assert_eq!(float(0x2d), 0.01);
        // A record before and one after the manifold, each (label, time stamp, unused).
        let end_of_records = e.mem.u32(stream + 4);
        assert_eq!(end_of_records, stream + 0x40 + 24);
        assert_eq!(e.mem.u32(stream + 0x40), START_LABEL);
        assert_eq!(e.mem.u32(stream + 0x4c), END_LABEL);
    }

    #[test]
    fn process_quiets_signalling_nans_it_copies_through_the_fpu() {
        let mut e = engine();
        monitor_stream(&mut e);
        let seen = install_manifold(&mut e);
        e.mem.set_u32(SKIN_WIDTH, 0x7f80_0001);
        let (a, _) = box_body(&mut e, 0.0, [1.0; 4]);
        let (b, _) = box_body(&mut e, 0.0, [1.0; 4]);
        let collision_input = e.mem.alloc(0x20);
        e.mem.set_u32(collision_input + 8, 0x7f80_0002);
        let input = e.mem.alloc(0x80);
        e.mem.set_u32(input, a);
        e.mem.set_u32(input + 4, b);
        e.mem.set_u32(input + 0xc, collision_input);
        let entry = e.mem.alloc(0x40);
        let output = e.mem.alloc(0x80);
        e.call(0x00d3_fe90, &args![input, entry, output, 0u32, 0u32]);
        let seen = seen.borrow();
        let words = &seen[0].0;
        // The tolerance is quieted (and so is every lane that sums with it), the constant too.
        assert_eq!(words[0x2c], 0x7fc0_0002);
        assert_eq!(words[0x20], 0x7fc0_0002);
        assert_eq!(words[0x2d], 0x7fc0_0001);
    }
}
