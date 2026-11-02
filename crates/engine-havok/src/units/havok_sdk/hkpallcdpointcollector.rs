//! `Havok SDK/hkpallcdpointcollector.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpAllCdPointCollector` (Xbox PDB; vtable `0104da28`) keeps every contact point of a
//! query in `m_hits`, an `hkArray` of `hkpRootCdPoint` (0x70 bytes each, `hkpRootCdPoint`
//! in the Xbox PDB) at +0x10 of the collector. A hit is the 32 bytes of contact data, then
//! for each of the two bodies: its root body (the end of the `hkpCdBody` parent chain, parent
//! at +0xC), its shape key (+4), and the shape keys of the parent chain below the root,
//! terminated by -1.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkarrayutil::HkArrayBase;

/// The thread's memory router (`00c85750`, no arguments); word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The array growth step (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkAlgorithm::quickSortRecursive<hkpRootCdPoint, less>` (`009b2ed0`, cdecl `(data, low,
/// high, comparison object)`).
const QUICK_SORT: u32 = 0x009b_2ed0;
/// `hkpRootCdPoint::operator=` (`006212c0`, `this` = destination, one stack argument).
const ROOT_CD_POINT_ASSIGN: u32 = 0x0062_12c0;

/// Size of an `hkpRootCdPoint`.
const HIT_SIZE: u32 = 0x70;
/// `hkpCdBody` fields (Xbox PDB): shape key and parent.
const CD_BODY_SHAPE_KEY: u32 = 0x04;
const CD_BODY_PARENT: u32 = 0x0c;

layout! {
    /// `hkpAllCdPointCollector` (Xbox PDB), as far as these functions read it. The size is the
    /// 0x3A0 bytes `fallout_shared_havok/cameracaster.rs` records for the collector.
    pub struct HkpAllCdPointCollector: 0x3A0 {
        /// `m_hits` (Xbox PDB): `hkArray<hkpRootCdPoint>`.
        0x10 m_hits: Inline<HkArrayBase>,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cab970,
            hkp_all_cd_point_collector_add_cd_point(Ptr<HkpAllCdPointCollector>, Ptr)
        ),
        entry!(
            0x00caba90,
            hkp_all_cd_point_collector_add_root_cd_point(Ptr<HkpAllCdPointCollector>, Ptr) -> u32
        ),
        entry!(
            0x00cabad0,
            hkp_all_cd_point_collector_sort_hits(Ptr<HkpAllCdPointCollector>)
        ),
    ]
}

/// Makes room for one more hit (growing the array by `00c90940` when it is full), bumps the
/// size and returns the address of the new slot.
fn append_hit(e: &mut Engine, hits: Ptr<HkArrayBase>) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let size = e.get(hits, HkArrayBase::m_size);
    let capacity = e.get(hits, HkArrayBase::m_capacityAndFlags) & 0x3fff_ffff;
    if size as u32 == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, hits, HIT_SIZE]);
    }
    let size = e.get(hits, HkArrayBase::m_size);
    let data = e.get(hits, HkArrayBase::m_data);
    let slot = (size as u32)
        .wrapping_mul(HIT_SIZE)
        .wrapping_add(data.addr());
    e.set(hits, HkArrayBase::m_size, size.wrapping_add(1));
    slot
}

/// Copies one body's part of a hit: the root of its parent chain into `hit + root_at`, its
/// own shape key into `hit + root_at + 4`, and the keys of the chain below the root, oldest
/// first, from `hit + keys_at`, ended by -1.
fn copy_body(e: &mut Engine, hit: u32, body: u32, root_at: u32, keys_at: u32) {
    let mut root = body;
    let mut depth = 0i32;
    let mut parent = e.mem.u32(body + CD_BODY_PARENT);
    while parent != 0 {
        root = parent;
        parent = e.mem.u32(parent + CD_BODY_PARENT);
        depth += 1;
    }
    let own_key = e.mem.u32(body + CD_BODY_SHAPE_KEY);
    e.mem.set_u32(hit + root_at, root);
    e.mem.set_u32(hit + root_at + 4, own_key);
    // The terminator sits after `depth` keys; the keys are written from the last slot down,
    // starting with the body's own key and then its parents' (the root's is not listed).
    e.mem.set_u32(hit + keys_at + 4 * depth as u32, 0xffff_ffff);
    let mut walker = body;
    for index in (0..depth).rev() {
        let key = e.mem.u32(walker + CD_BODY_SHAPE_KEY);
        e.mem.set_u32(hit + keys_at + 4 * index as u32, key);
        walker = e.mem.u32(walker + CD_BODY_PARENT);
    }
}

// Translated from 00cab970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAllCdPointCollector::addCdPoint` (Xbox PDB), `RET 4`: appends a hit built from the
/// contact point `point` (32 bytes of contact data, then the two `hkpCdBody` pointers at
/// +0x20 and +0x24).
pub fn hkp_all_cd_point_collector_add_cd_point(
    e: &mut Engine,
    this: Ptr<HkpAllCdPointCollector>,
    point: Ptr,
) {
    let hits = this.at(HkpAllCdPointCollector::m_hits);
    let hit = append_hit(e, hits);
    let contact = e.mem.bytes(point.addr(), 32);
    e.mem.write(hit, &contact);
    let body_a = e.mem.u32(point.addr() + 0x20);
    copy_body(e, hit, body_a, 0x20, 0x28);
    let body_b = e.mem.u32(point.addr() + 0x24);
    copy_body(e, hit, body_b, 0x48, 0x50);
}

// Translated from 00caba90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAllCdPointCollector::addRootCdPoint` (Xbox PDB), `RET 4`: appends a hit, copied from
/// the `hkpRootCdPoint` `point` with `hkpRootCdPoint::operator=` (`006212c0`); the exe jumps
/// to that function, so its result is this one's.
pub fn hkp_all_cd_point_collector_add_root_cd_point(
    e: &mut Engine,
    this: Ptr<HkpAllCdPointCollector>,
    point: Ptr,
) -> u32 {
    let hits = this.at(HkpAllCdPointCollector::m_hits);
    let hit = append_hit(e, hits);
    e.call(ROOT_CD_POINT_ASSIGN, &args![hit, point]).u32()
}

// Translated from 00cabad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAllCdPointCollector::sortHits` (Xbox PDB): sorts the hits with the quick sort when
/// there are two or more. The comparison object is an empty `hkAlgorithm::less`, passed by
/// value: its stack word is `this` with the low byte cleared (the compiler stores the byte
/// zero into the pushed copy of ECX).
pub fn hkp_all_cd_point_collector_sort_hits(e: &mut Engine, this: Ptr<HkpAllCdPointCollector>) {
    let hits = this.at(HkpAllCdPointCollector::m_hits);
    let size = e.get(hits, HkArrayBase::m_size);
    if size > 1 {
        let data = e.get(hits, HkArrayBase::m_data);
        let comparison = this.addr() & 0xffff_ff00;
        e.call(QUICK_SORT, &args![data, 0i32, size - 1, comparison]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROUTER: u32 = 0x0300_0000;
    const ALLOCATOR: u32 = 0x0300_0040;

    fn rig() -> (Engine, Ptr<HkpAllCdPointCollector>) {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.map(ROUTER, 0x100);
        e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        e.register(MEMORY_ROUTER, |_, _| ROUTER.into_ret());
        // The growth step gives the array room for four hits.
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let block = e.mem.alloc(4 * HIT_SIZE);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        e.register(QUICK_SORT, |_, _| Ret::default());
        e.register(ROOT_CD_POINT_ASSIGN, |e, a| {
            let bytes = e.mem.bytes(a[1], 8);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        let collector = e.new_object::<HkpAllCdPointCollector>();
        (e, collector)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn body(e: &mut Engine, key: u32, parent: u32) -> u32 {
        let b = e.mem.alloc(0x20);
        e.mem.set_u32(b + CD_BODY_SHAPE_KEY, key);
        e.mem.set_u32(b + CD_BODY_PARENT, parent);
        b
    }

    #[test]
    fn add_cd_point_records_roots_and_key_chains() {
        let (mut e, collector) = rig();
        let root_a = body(&mut e, 0, 0);
        let mid_a = body(&mut e, 11, root_a);
        let leaf_a = body(&mut e, 10, mid_a);
        let lone_b = body(&mut e, 99, 0);
        let point = e.mem.alloc(0x40);
        for i in 0..8 {
            e.mem.set_u32(point + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(point + 0x20, leaf_a);
        e.mem.set_u32(point + 0x24, lone_b);
        e.call(0x00ca_b970, &args![collector, Ptr::<()>::new(point)]);

        // The array was empty with no room: the growth step ran once.
        assert_eq!(calls(&e, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(calls(&e, ARRAY_RESERVE_MORE)[0][0], ALLOCATOR);
        let hits = collector.at(HkpAllCdPointCollector::m_hits);
        assert_eq!(e.get(hits, HkArrayBase::m_size), 1);
        let hit = e.get(hits, HkArrayBase::m_data).addr();
        assert_eq!(e.mem.u32(hit), 0x100);
        assert_eq!(e.mem.u32(hit + 0x1c), 0x107);
        assert_eq!(e.mem.u32(hit + 0x20), root_a);
        assert_eq!(e.mem.u32(hit + 0x24), 10);
        assert_eq!(e.mem.u32(hit + 0x28), 11);
        assert_eq!(e.mem.u32(hit + 0x2c), 10);
        assert_eq!(e.mem.u32(hit + 0x30), 0xffff_ffff);
        assert_eq!(e.mem.u32(hit + 0x48), lone_b);
        assert_eq!(e.mem.u32(hit + 0x4c), 99);
        assert_eq!(e.mem.u32(hit + 0x50), 0xffff_ffff);

        // A second hit goes to the next slot without growing.
        e.call(0x00ca_b970, &args![collector, Ptr::<()>::new(point)]);
        assert_eq!(calls(&e, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(e.get(hits, HkArrayBase::m_size), 2);
        assert_eq!(e.mem.u32(hit + HIT_SIZE + 0x28), 11);
    }

    #[test]
    fn add_root_cd_point_copies_through_the_assignment() {
        let (mut e, collector) = rig();
        let source = e.mem.alloc(HIT_SIZE);
        e.mem.set_u32(source, 0xfeed);
        let result = e
            .call(0x00ca_ba90, &args![collector, Ptr::<()>::new(source)])
            .u32();
        let hits = collector.at(HkpAllCdPointCollector::m_hits);
        let hit = e.get(hits, HkArrayBase::m_data).addr();
        assert_eq!(result, hit);
        assert_eq!(e.mem.u32(hit), 0xfeed);
        assert_eq!(e.get(hits, HkArrayBase::m_size), 1);
        assert_eq!(calls(&e, ROOT_CD_POINT_ASSIGN), vec![vec![hit, source]]);
    }

    #[test]
    fn sort_hits_needs_two_hits() {
        let (mut e, collector) = rig();
        let hits = collector.at(HkpAllCdPointCollector::m_hits);
        e.set(hits, HkArrayBase::m_data, Ptr::new(0x7000));
        e.set(hits, HkArrayBase::m_size, 1);
        e.call(0x00ca_bad0, &args![collector]);
        assert!(calls(&e, QUICK_SORT).is_empty());
        e.set(hits, HkArrayBase::m_size, 5);
        e.call(0x00ca_bad0, &args![collector]);
        assert_eq!(
            calls(&e, QUICK_SORT),
            vec![vec![0x7000, 0, 4, collector.addr() & 0xffff_ff00]]
        );
    }
}
