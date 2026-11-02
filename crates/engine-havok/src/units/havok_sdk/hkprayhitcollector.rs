//! `Havok SDK/hkprayhitcollector.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! None of the functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cf0860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRayHitCollector::shapeKeysFromCdBody` (Xbox PDB), cdecl `(keys, max_keys, body)`: walks
/// from `body` up its parent chain (the pointer at `+0xC`; the topmost body, which has no
/// parent, is left out) and writes the shape key (`+4`) of each body from the one just below the
/// topmost down to `body`, at most `max_keys - 1` of them, then -1. Answers the number of words
/// written.
///
/// The exe keeps the chain in an eight-word array on its stack; a ninth body would overwrite its
/// return address, so such a chain panics here.
pub fn hkp_ray_hit_collector_shape_keys_from_cd_body(
    e: &mut Engine,
    keys: Ptr,
    max_keys: i32,
    body: Ptr,
) -> i32 {
    let mut chain: Vec<u32> = Vec::new();
    let mut node = body.addr();
    let mut parent = e.mem.u32(node + 0xc);
    while parent != 0 {
        assert!(
            chain.len() < 8,
            "00cf0860: a chain of more than eight bodies overruns the stack array"
        );
        chain.push(node);
        node = parent;
        parent = e.mem.u32(node + 0xc);
    }
    let mut index: i32 = 0;
    let mut left = chain.len();
    if left > 0 {
        let limit = max_keys.wrapping_sub(1);
        loop {
            if index >= limit {
                break;
            }
            left -= 1;
            let key = e.mem.u32(chain[left] + 4);
            e.mem
                .set_u32(keys.addr().wrapping_add((index as u32) << 2), key);
            index += 1;
            if left == 0 {
                break;
            }
        }
    }
    e.mem
        .set_u32(keys.addr().wrapping_add((index as u32) << 2), u32::MAX);
    index + 1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cf_0860,
        hkp_ray_hit_collector_shape_keys_from_cd_body(Ptr, i32, Ptr) -> i32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A body chain: element 0 is the topmost; each body's key is `0x100 + its index`.
    fn bodies(e: &mut Engine, depth: u32) -> Vec<u32> {
        let mut made: Vec<u32> = Vec::new();
        for index in 0..depth {
            let body = e.mem.alloc(0x20);
            e.mem.set_u32(body + 4, 0x100 + index);
            e.mem.set_u32(body + 0xc, made.last().copied().unwrap_or(0));
            made.push(body);
        }
        made
    }

    #[test]
    fn keys_run_from_below_the_topmost_body_down_to_the_given_one_00cf0860() {
        let mut e = Engine::new();
        let chain = bodies(&mut e, 4);
        let keys = e.mem.alloc(0x40);
        let count = e.call(0x00cf_0860, &args![keys, 8u32, chain[3]]).i32();
        assert_eq!(count, 4);
        assert_eq!(
            [0, 4, 8, 12].map(|at| e.mem.u32(keys + at)),
            [0x101, 0x102, 0x103, u32::MAX]
        );
    }

    #[test]
    fn a_body_without_a_parent_gives_only_the_terminator_00cf0860() {
        let mut e = Engine::new();
        let chain = bodies(&mut e, 1);
        let keys = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00cf_0860, &args![keys, 3u32, chain[0]]).i32(), 1);
        assert_eq!(e.mem.u32(keys), u32::MAX);
    }

    #[test]
    fn the_output_is_limited_to_one_less_than_the_room_00cf0860() {
        let mut e = Engine::new();
        let chain = bodies(&mut e, 5);
        let keys = e.mem.alloc(0x40);
        e.mem.write(keys, &[0xcd; 0x40]);
        // Room for 3 words: two keys and the terminator.
        let count = e.call(0x00cf_0860, &args![keys, 3u32, chain[4]]).i32();
        assert_eq!(count, 3);
        assert_eq!(
            [0, 4, 8].map(|at| e.mem.u32(keys + at)),
            [0x101, 0x102, u32::MAX]
        );
        assert_eq!(e.mem.u32(keys + 12), 0xcdcd_cdcd);
        // No room at all: the terminator goes first.
        e.mem.write(keys, &[0xcd; 0x40]);
        assert_eq!(e.call(0x00cf_0860, &args![keys, 0u32, chain[4]]).i32(), 1);
        assert_eq!(e.mem.u32(keys), u32::MAX);
        assert_eq!(e.mem.u32(keys + 4), 0xcdcd_cdcd);
    }

    #[test]
    fn eight_bodies_below_the_top_fit_the_stack_array_00cf0860() {
        let mut e = Engine::new();
        let chain = bodies(&mut e, 9);
        let keys = e.mem.alloc(0x80);
        assert_eq!(e.call(0x00cf_0860, &args![keys, 20u32, chain[8]]).i32(), 9);
        assert_eq!(e.mem.u32(keys), 0x101);
        assert_eq!(e.mem.u32(keys + 28), 0x108);
        assert_eq!(e.mem.u32(keys + 32), u32::MAX);
    }

    #[test]
    #[should_panic(expected = "00cf0860")]
    fn nine_bodies_below_the_top_overrun_the_stack_array_00cf0860() {
        let mut e = Engine::new();
        let chain = bodies(&mut e, 10);
        let keys = e.mem.alloc(0x80);
        e.call(0x00cf_0860, &args![keys, 20u32, chain[9]]);
    }
}
