//! `Havok SDK/hkpallrayhitcollector.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpAllRayHitCollector` keeps every ray hit in an `hkArray` of 0x60-byte records (data
//! `+0x10`, size `+0x14`, capacity word `+0x18` whose low 30 bits are the capacity).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkpRayHitCollector::shapeKeysFromCdBody` (`00cf0860`, cdecl `(keys, capacity, body)`).
const SHAPE_KEYS_FROM_CD_BODY: u32 = 0x00cf_0860;
/// Bytes of one stored hit.
const HIT_SIZE: u32 = 0x60;

// Translated from 00c85520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAllRayHitCollector::addRayHit` (named in the engine map): appends a copy of the hit.
/// The array grows by one record first when it is full (`_reserveMore`, element size `0x60`;
/// the memory router is asked before the size is compared). The new record's fraction,
/// shape-key and body words are initialised (1.0 at `+0x10`, -1 at `+0x14` and `+0x20`, 0 at
/// `+0x40` and `+0x50`; skipped when the address is null), then filled from `hit_info`: the
/// 16-byte normal at `+0`, the fraction through the x87 at `+0x10` (a signalling NaN is
/// quieted) and the words `+0x14`, `+0x18`, `+0x1c`. The shape keys of `cd_body` go to
/// `+0x20` (8 of them, `hkpRayHitCollector::shapeKeysFromCdBody`); `+0x50` gets the last body
/// of the `cd_body` parent chain (the word at `+0xc`), or `cd_body` itself without parent.
pub fn hkp_all_ray_hit_collector_add_ray_hit(
    e: &mut Engine,
    this: Ptr,
    cd_body: u32,
    hit_info: u32,
) {
    let hits = this.addr() + 0x10;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(hits + 8) & 0x3fff_ffff;
    if e.mem.u32(hits + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, hits, HIT_SIZE]);
    }
    let size = e.mem.u32(hits + 4);
    let element = size.wrapping_mul(HIT_SIZE).wrapping_add(e.mem.u32(hits));
    if element != 0 {
        e.mem.set_u32(element + 0x10, 1.0f32.to_bits());
        e.mem.set_u32(element + 0x14, 0xffff_ffff);
        e.mem.set_u32(element + 0x40, 0);
        e.mem.set_u32(element + 0x20, 0xffff_ffff);
        e.mem.set_u32(element + 0x50, 0);
    }
    let size = e.mem.u32(hits + 4);
    e.mem.set_u32(hits + 4, size.wrapping_add(1));
    let record = size.wrapping_mul(HIT_SIZE).wrapping_add(e.mem.u32(hits));
    for lane in 0..4 {
        let word = e.mem.u32(hit_info + 4 * lane);
        e.mem.set_u32(record + 4 * lane, word);
    }
    let fraction = x87_quiet_bits(e.mem.u32(hit_info + 0x10));
    e.mem.set_u32(record + 0x10, fraction);
    for offset in [0x14, 0x18, 0x1c] {
        let word = e.mem.u32(hit_info + offset);
        e.mem.set_u32(record + offset, word);
    }
    e.call(
        SHAPE_KEYS_FROM_CD_BODY,
        &args![record + 0x20, 8u32, cd_body],
    );
    let mut last = cd_body;
    let mut parent = e.mem.u32(cd_body + 0xc);
    while parent != 0 {
        last = parent;
        parent = e.mem.u32(last + 0xc);
    }
    e.mem.set_u32(record + 0x50, last);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c8_5520,
        hkp_all_ray_hit_collector_add_ray_hit(Ptr, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument lists of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A router (allocator `allocator`), a collector whose hit array has `capacity` records of
    /// room and `size` filled, and a hit record `(normal 1, 2, 3, 4; fraction; words 5, 6, 7)`.
    fn setup(e: &mut Engine, size: u32, capacity: u32) -> (u32, u32, u32, u32) {
        let allocator = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(SHAPE_KEYS_FROM_CD_BODY, |_, _| Ret::default());
        let collector = e.mem.alloc(0x40);
        let data = e.mem.alloc(HIT_SIZE * (capacity + 1));
        e.mem.set_u32(collector + 0x10, data);
        e.mem.set_u32(collector + 0x14, size);
        e.mem.set_u32(collector + 0x18, capacity);
        let hit = e.mem.alloc(0x20);
        for (i, bits) in [1.0f32, 2.0, 3.0, 4.0, 0.25]
            .iter()
            .map(|v| v.to_bits())
            .enumerate()
        {
            e.mem.set_u32(hit + 4 * i as u32, bits);
        }
        for (i, word) in [5u32, 6, 7].into_iter().enumerate() {
            e.mem.set_u32(hit + 0x14 + 4 * i as u32, word);
        }
        (allocator, collector, data, hit)
    }

    #[test]
    fn add_ray_hit_appends_a_copy_with_the_shape_keys_and_the_root_body() {
        let mut e = engine();
        let (_, collector, data, hit) = setup(&mut e, 1, 4);
        // The body chain: body -> parent -> root.
        let root = e.mem.alloc(0x20);
        let parent = e.mem.alloc(0x20);
        e.mem.set_u32(parent + 0xc, root);
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body + 0xc, parent);
        e.call(0x00c8_5520, &args![collector, body, hit]);
        assert_eq!(e.mem.u32(collector + 0x14), 2);
        let record = data + HIT_SIZE;
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(record + 4 * i)).collect();
        let expected: Vec<u32> = [1.0f32, 2.0, 3.0, 4.0, 0.25]
            .iter()
            .map(|v| v.to_bits())
            .chain([5, 6, 7])
            .collect();
        assert_eq!(words, expected);
        assert_eq!(e.mem.u32(record + 0x20), 0xffff_ffff);
        assert_eq!(e.mem.u32(record + 0x40), 0);
        assert_eq!(e.mem.u32(record + 0x50), root);
        assert_eq!(
            calls(&e, SHAPE_KEYS_FROM_CD_BODY),
            vec![vec![record + 0x20, 8, body]]
        );
        assert!(calls(&e, ARRAY_RESERVE_MORE).is_empty());
    }

    #[test]
    fn add_ray_hit_without_a_parent_stores_the_body_itself() {
        let mut e = engine();
        let (_, collector, data, hit) = setup(&mut e, 0, 4);
        let body = e.mem.alloc(0x20);
        e.call(0x00c8_5520, &args![collector, body, hit]);
        assert_eq!(e.mem.u32(data + 0x50), body);
        assert_eq!(e.mem.u32(collector + 0x14), 1);
    }

    #[test]
    fn add_ray_hit_grows_a_full_array_first() {
        let mut e = engine();
        let (allocator, collector, data, hit) = setup(&mut e, 2, 2);
        // The growth double moves the array into a block with room for four records.
        let bigger = e.mem.alloc(HIT_SIZE * 4);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            e.mem.set_u32(a[1], bigger);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let body = e.mem.alloc(0x20);
        e.call(0x00c8_5520, &args![collector, body, hit]);
        assert_eq!(
            calls(&e, ARRAY_RESERVE_MORE),
            vec![vec![allocator, collector + 0x10, HIT_SIZE]]
        );
        assert_ne!(bigger, data);
        assert_eq!(e.mem.u32(collector + 0x14), 3);
        assert_eq!(e.mem.f32(bigger + 2 * HIT_SIZE + 0x10), 0.25);
        assert_eq!(e.mem.u32(bigger + 2 * HIT_SIZE + 0x50), body);
    }

    #[test]
    fn add_ray_hit_quiets_the_fraction_but_copies_the_other_words_bit_for_bit() {
        let mut e = engine();
        let (_, collector, data, hit) = setup(&mut e, 0, 2);
        e.mem.set_u32(hit, 0x7fa0_0001);
        e.mem.set_u32(hit + 0x10, 0x7fa0_0000);
        e.mem.set_u32(hit + 0x14, 0x7fa0_0002);
        let body = e.mem.alloc(0x20);
        e.call(0x00c8_5520, &args![collector, body, hit]);
        assert_eq!(e.mem.u32(data), 0x7fa0_0001);
        assert_eq!(e.mem.u32(data + 0x10), 0x7fe0_0000);
        assert_eq!(e.mem.u32(data + 0x14), 0x7fa0_0002);
    }
}
