//! `Havok SDK/hkpsymmetricagentlinearcast.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The three collectors (names from the engine map) wrap another collector and pass what they
//! are given on with the two bodies swapped. The wrapped collector is a pointer at `+8` (the
//! flip collector, the flip body collector) or at `+0x20` (the flip cast collector); the byte
//! or word at `+4` of a collector is its early-out status, which the wrapper copies back from
//! the wrapped one after each call. A contact point is 0x28 bytes: the position at `+0`, the
//! normal and (in its fourth lane) the distance at `+0x10`, and two words at `+0x20` and `+0x24`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{add, mul, splat};
use crate::units::havok_sdk::hkpsphereshape::{load_bits, load_vec, store_bits, store_vec, Vec4};

/// The mask that turns a vector into the vector with its first three lanes negated.
const NEGATE_MASK: u32 = 0x010c_4c00;

/// Passes the contact point at `point` on to the collector at `target` (virtual `4`, `(point)`)
/// with the position replaced by `position`, the normal negated in its first three lanes (the
/// fourth lane, the distance, stays) and the two words at `+0x20` and `+0x24` swapped; then
/// copies the wrapped collector's status word (`+4`) through the x87, so a signalling NaN comes
/// out quiet.
fn pass_flipped_point(e: &mut Engine, this: u32, point: u32, position: Vec4, target: u32) {
    let mask = load_bits(e, NEGATE_MASK);
    let normal = load_bits(e, point + 0x10);
    let flipped: [u32; 4] = std::array::from_fn(|lane| normal[lane] ^ mask[lane]);
    let first_word = e.mem.u32(point + 0x24);
    let second_word = e.mem.u32(point + 0x20);
    e.with_stack(0x30, |e, block| {
        let block = block.addr();
        store_vec(e, block, position);
        e.mem.set_u32(block + 0x20, first_word);
        e.mem.set_u32(block + 0x24, second_word);
        store_bits(e, block + 0x10, flipped);
        e.vcall(target, 4, &args![block]);
    });
    let status = x87_quiet_bits(e.mem.u32(target + 4));
    e.mem.set_u32(this + 4, status);
}

// Translated from 00da7070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentFlipCollector::addCdPoint` (engine map; virtual): passes the point on to the
/// wrapped collector at `[this + 8]` with the position `position + normal.w * normal` (the
/// fourth lane of the normal vector is the distance, spread over the lanes), the normal
/// negated and the two words swapped, and copies that collector's status (`+4`) back.
pub fn hkp_symmetric_agent_flip_collector_add_cd_point(e: &mut Engine, this: Ptr, point: Ptr) {
    let this = this.addr();
    let point = point.addr();
    let position = load_vec(e, point);
    let normal = load_vec(e, point + 0x10);
    let moved = add(mul(splat(normal, 3), normal), position);
    let target = e.mem.u32(this + 8);
    pass_flipped_point(e, this, point, moved, target);
}

// Translated from 00da70e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentFlipCastCollector::addCdPoint` (engine map; virtual): as the flip
/// collector, except that the position is `position + normal.w * direction` with the cast
/// direction being the vector at `[this + 0x10]`, and that the wrapped collector is the
/// pointer at `[this + 0x20]`.
pub fn hkp_symmetric_agent_flip_cast_collector_add_cd_point(e: &mut Engine, this: Ptr, point: Ptr) {
    let this = this.addr();
    let point = point.addr();
    let normal = load_vec(e, point + 0x10);
    let direction = load_vec(e, this + 0x10);
    let position = load_vec(e, point);
    let moved = add(mul(splat(normal, 3), direction), position);
    let target = e.mem.u32(this + 0x20);
    pass_flipped_point(e, this, point, moved, target);
}

// Translated from 00da7150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentFlipBodyCollector::addCdBodyPair` (engine map; virtual, `(first, second)`):
/// calls the wrapped collector at `[this + 8]` (virtual `4`) with the two bodies swapped and
/// copies its status byte (`+4`) back.
pub fn hkp_symmetric_agent_flip_body_collector_add_cd_body_pair(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
) {
    let this = this.addr();
    let target = e.mem.u32(this + 8);
    e.vcall(target, 4, &args![second, first]);
    let status = e.mem.u8(target + 4);
    e.mem.set_u8(this + 4, status);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00da_7070,
            hkp_symmetric_agent_flip_collector_add_cd_point(Ptr, Ptr)
        ),
        entry!(
            0x00da_70e0,
            hkp_symmetric_agent_flip_cast_collector_add_cd_point(Ptr, Ptr)
        ),
        entry!(
            0x00da_7150,
            hkp_symmetric_agent_flip_body_collector_add_cd_body_pair(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;
    use std::cell::RefCell;
    use std::rc::Rc;

    const INNER_SLOT_DOUBLE: u32 = 0x0900_0004;

    /// What the wrapped collector received: the address of the point and its ten words.
    type Received = Rc<RefCell<Vec<(u32, [u32; 10])>>>;

    /// A wrapped collector whose slot 4 keeps the words of the point it was given and whose
    /// status word at `+4` is `status`.
    fn inner_collector(e: &mut Engine, status: u32) -> (u32, Received) {
        let received: Received = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&received);
        e.register_double(INNER_SLOT_DOUBLE, move |e, a| {
            let words = std::array::from_fn(|index| e.mem.u32(a[1] + 4 * index as u32));
            sink.borrow_mut().push((a[1], words));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, INNER_SLOT_DOUBLE);
        let inner = e.mem.alloc(0x20);
        e.mem.set_u32(inner, vtable);
        e.mem.set_u32(inner + 4, status);
        (inner, received)
    }

    /// A contact point: position `(1, 2, 3, 4)`, normal `(0.5, 0.25, 2, 3)` (distance 3) and
    /// the words 7 and 8 at `+0x20`, `+0x24` (the words 8 and 9 of the array of floats).
    fn contact_point(e: &mut Engine) -> u32 {
        let point = e.mem.alloc(0x30);
        store_vec(e, point, [1.0, 2.0, 3.0, 4.0]);
        store_vec(e, point + 0x10, [0.5, 0.25, 2.0, 3.0]);
        e.mem.set_u32(point + 0x20, 7);
        e.mem.set_u32(point + 0x24, 8);
        point
    }

    fn engine_with_mask() -> Engine {
        let mut e = engine();
        for lane in 0..4u32 {
            let sign = if lane < 3 { 0x8000_0000 } else { 0 };
            e.mem.set_u32(NEGATE_MASK + 4 * lane, sign);
        }
        e
    }

    fn floats(words: &[u32]) -> Vec<f32> {
        words.iter().map(|word| f32::from_bits(*word)).collect()
    }

    #[test]
    fn the_flip_collector_moves_the_position_along_the_normal_and_negates_the_normal() {
        let mut e = engine_with_mask();
        let (inner, received) = inner_collector(&mut e, 0x7fa0_0001);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, inner);
        let point = contact_point(&mut e);
        e.call(0x00da_7070, &args![this, point]);
        let received = received.borrow();
        assert_eq!(received.len(), 1);
        let words = received[0].1;
        // The position is (1, 2, 3, 4) + 3 * (0.5, 0.25, 2, 3).
        assert_eq!(floats(&words[..4]), vec![2.5, 2.75, 9.0, 13.0]);
        // The first three lanes of the normal are negated; the distance stays.
        assert_eq!(floats(&words[4..8]), vec![-0.5, -0.25, -2.0, 3.0]);
        // The words at +0x20 and +0x24 are swapped.
        assert_eq!(words[8..], [8, 7]);
        // The status word is copied through the x87: a signalling NaN comes out quiet.
        assert_eq!(e.mem.u32(this + 4), 0x7fe0_0001);
    }

    #[test]
    fn the_flip_cast_collector_moves_the_position_along_the_cast_direction() {
        let mut e = engine_with_mask();
        let (inner, received) = inner_collector(&mut e, 0x1234);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x20, inner);
        store_vec(&mut e, this + 0x10, [10.0, 20.0, 30.0, 40.0]);
        let point = contact_point(&mut e);
        e.call(0x00da_70e0, &args![this, point]);
        let received = received.borrow();
        assert_eq!(received.len(), 1);
        let words = received[0].1;
        assert_eq!(floats(&words[..4]), vec![31.0, 62.0, 93.0, 124.0]);
        assert_eq!(floats(&words[4..8]), vec![-0.5, -0.25, -2.0, 3.0]);
        assert_eq!(words[8..], [8, 7]);
        assert_eq!(e.mem.u32(this + 4), 0x1234);
    }

    #[test]
    fn the_flip_body_collector_swaps_the_bodies_and_copies_the_status_byte() {
        let mut e = engine_with_mask();
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(INNER_SLOT_DOUBLE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, INNER_SLOT_DOUBLE);
        let inner = e.mem.alloc(0x20);
        e.mem.set_u32(inner, vtable);
        e.mem.set_u8(inner + 4, 0x5a);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, inner);
        e.mem.set_u32(this + 4, 0xffff_ff00);
        e.call(0x00da_7150, &args![this, 0x1111u32, 0x2222u32]);
        assert_eq!(*seen.borrow(), vec![vec![inner, 0x2222, 0x1111]]);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ff5a);
    }
}
