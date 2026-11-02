//! `fallout/misc/run_00846b60` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSSaveLoadFormIDMap` (Xbox PDB), checked against PC 00846b60.
    pub struct BGSSaveLoadFormIDMap: 0x24 {
        0x20 iCurrentIndex: u32,
    }
}

// Translated from 00846b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs both maps and reserves the index for the null form ID.
// Translation incomplete: 00846b60 - compiler exception-unwind cleanup remains omitted.
pub fn construct(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadFormIDMap>,
    buckets: u32,
) -> Ptr<BGSSaveLoadFormIDMap> {
    e.call(0x006e2080, &args![this, buckets]);
    e.call(0x006e2080, &args![this.addr().wrapping_add(0x10), buckets]);
    e.set(this, BGSSaveLoadFormIDMap::iCurrentIndex, 0);
    e.call(0x00846c90, &args![this, 0u32]);
    this
}

/// `00438af0` (`thiscall`, no stack word): empties a map.
const EMPTY_MAP: u32 = 0x0043_8af0;
/// `006e2150` (`thiscall`, no stack word; unit `fallout shared/pathfinding/pathinglosmap.cpp`):
/// a map's destructor.
const MAP_DESTRUCTOR: u32 = 0x006e_2150;

// Translated from 00846be0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00846be0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f12a63, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): the destructor of the two-map
/// registry `construct` builds. Empties the maps at +0 and +0x10 (`00438af0`), then destroys
/// the one at +0x10 and the one at +0 (`006e2150`).
pub fn fn_00846be0(e: &mut Engine, this: Ptr) {
    e.call(EMPTY_MAP, &args![this]);
    e.call(EMPTY_MAP, &args![this.addr() + 0x10]);
    e.call(MAP_DESTRUCTOR, &args![this.addr() + 0x10]);
    e.call(MAP_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00846b60,
            construct(Ptr<BGSSaveLoadFormIDMap>, u32) -> Ptr<BGSSaveLoadFormIDMap>
        ),
        entry!(0x0084_6be0, fn_00846be0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_registry_map_keeps_both_directions_and_numeric_id_encoding() {
        let mut e = Engine::new();
        e.map(0x011f6000, 0x1000);
        e.map(0x011c3f2c, 4);
        e.map(0x011ddf38, 4);
        e.put_vtable(0x010a2528, &[0, 0x00aa2240, 0, 0, 0, 0, 0, 0x00a29680]);
        e.put_vtable(
            0x0106cf68,
            &[
                0, 0x006c6a60, 0x006b81d0, 0x00863840, 0, 0x00844a70, 0x00853240,
            ],
        );
        let allocator = e.mem.alloc(4);
        let source = e.mem.alloc(8);
        e.call(0x00aa2230, &[allocator]);
        e.call(0x00aa21d0, &[source, allocator]);
        e.mem.set_u32(0x011f6118, source);
        e.call(0x00aa2020, &[]);
        let map = e.new_object::<BGSSaveLoadFormIDMap>();
        assert_eq!(e.call(0x00846b60, &args![map, 1u32]).u32(), map.addr());
        // One bucket deliberately forces all keys into a collision chain.
        assert_eq!(e.get(map, BGSSaveLoadFormIDMap::iCurrentIndex), 1);
        let manager = e.mem.alloc(12);
        e.mem.set_u32(manager + 8, map.addr());
        e.mem.set_u32(0x011ddf38, manager);
        let index = e.mem.alloc(4);
        for (id, encoded) in [
            (0u32, [0, 0, 0]),
            (0x1234, [0, 0, 1]),
            (0x101234, [0, 0, 2]),
            (0x1234, [0, 0, 1]),
            (0xff012345, [0x81, 0x23, 0x45]),
        ] {
            e.mem.write(index, &[0xaa; 4]);
            e.call(0x00853570, &args![index, id]);
            assert_eq!(
                e.mem.bytes(index, 4),
                [encoded[0], encoded[1], encoded[2], 0xaa]
            );
            assert_eq!(e.call(0x00853500, &args![index]).u32(), id);
        }
        assert_eq!(e.get(map, BGSSaveLoadFormIDMap::iCurrentIndex), 3);
        assert_eq!(e.mem.u32(map.addr() + 12), 3);
        assert_eq!(e.mem.u32(map.addr() + 0x1c), 3);
        assert_eq!(e.call(0x00846d20, &args![map, 99u32]).u32(), 0);
        // Replacement changes no count and resolves through the same bucket.
        e.call(0x00844700, &args![map.addr() + 0x10, 2u32, 0x778899u32]);
        assert_eq!(e.call(0x00846d20, &args![map, 2u32]).u32(), 0x778899);
        assert_eq!(e.mem.u32(map.addr() + 0x1c), 3);
        let node = e.call(0x00844a70, &args![map]).u32();
        assert!(e.mem.block_size(node).is_some());
        e.mem.set_u32(node + 8, 0xdeadbeef);
        e.call(0x00853240, &args![map, node]);
        assert_eq!(e.mem.block_size(node), None);
        e.call(0x006b8310, &args![0u32, 0u32]);
    }

    #[test]
    fn item_allocation_and_delete_forward_exact_receiver_and_clear_before_free() {
        let mut e = Engine::new();
        let map = e.mem.alloc(16);
        let node = e.mem.alloc(12);
        e.mem.write(node, &[0xaa; 12]);
        e.register_double(0x00401000, move |_, a| {
            assert_eq!(a, &[12]);
            node.into_ret()
        });
        assert_eq!(e.call(0x00844a70, &args![map]).u32(), node);
        assert_eq!(e.mem.bytes(node, 12), [0xaa; 12]);
        e.register_double(0x00401030, move |e, a| {
            assert_eq!(a, &[node]);
            assert_eq!(e.mem.bytes(node, 8), [0xaa; 8]);
            assert_eq!(e.mem.u32(node + 8), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00853240, &args![map, node]);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .contains(&(0x006b8310, vec![map + 12, node])));
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00846be0_empties_both_maps_then_destroys_them() {
        let mut e = Engine::new();
        e.register(EMPTY_MAP, |_, _| Ret::default());
        e.register(MAP_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0084_6be0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (EMPTY_MAP, vec![this]),
                (EMPTY_MAP, vec![this + 0x10]),
                (MAP_DESTRUCTOR, vec![this + 0x10]),
                (MAP_DESTRUCTOR, vec![this]),
            ]
        );
    }
}
