//! `Havok SDK/hkpshapecontainer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpShapeContainer` (Xbox PDB) is the interface shape collections implement: vtable slot
//! `0x08` is `getFirstKey` (answers -1 when there is no child), `0x0C` is `getNextKey(key)`,
//! `0x14` is `getChildShape(key, buffer)`. `hkpSingleShapeContainer` holds one child shape
//! at +4.

#[allow(unused_imports)]
use crate::prelude::*;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1f1d0, hkp_shape_container_get_collision_filter_info(Ptr, u32) -> u32),
        entry!(0x00d1f1e0, hkp_shape_container_get_num_child_shapes(Ptr) -> i32),
        entry!(0x00d1f210, hkp_single_shape_container_get_child_shape(Ptr, u32, Ptr) -> u32),
    ]
}

// Translated from 00d1f1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeContainer::getCollisionFilterInfo` (Xbox PDB), `RET 4`: the default answer, 0
/// for every key.
pub fn hkp_shape_container_get_collision_filter_info(
    _e: &mut Engine,
    _this: Ptr,
    _key: u32,
) -> u32 {
    0
}

// Translated from 00d1f1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeContainer::getNumChildShapes` (Xbox PDB): counts the keys from `getFirstKey` (slot
/// `0x08`) following `getNextKey` (slot `0x0C`) until one is -1.
pub fn hkp_shape_container_get_num_child_shapes(e: &mut Engine, this: Ptr) -> i32 {
    let mut count = 0i32;
    let mut key = e.vcall(this.addr(), 0x8, &args![]).i32();
    while key != -1 {
        key = e.vcall(this.addr(), 0xc, &args![key]).i32();
        count = count.wrapping_add(1);
    }
    count
}

// Translated from 00d1f210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSingleShapeContainer::getChildShape` (Xbox PDB), `RET 8`: the one child shape (at
/// +4), whatever the key and buffer.
pub fn hkp_single_shape_container_get_child_shape(
    e: &mut Engine,
    this: Ptr,
    _key: u32,
    _buffer: Ptr,
) -> u32 {
    e.mem.u32(this.addr() + 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0200_0000;
    const FIRST_KEY: u32 = 0x0200_0100;
    const NEXT_KEY: u32 = 0x0200_0104;

    #[test]
    fn filter_info_is_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d1_f1d0, &args![0x1000u32, 7u32]).u32(), 0);
    }

    #[test]
    fn child_count_follows_the_keys_until_minus_one() {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.put_vtable(VTABLE, &[0, 0, FIRST_KEY, NEXT_KEY]);
        // Keys 5 -> 9 -> 2 -> end.
        e.register(FIRST_KEY, |_, _| 5u32.into_ret());
        e.register(NEXT_KEY, |_, a| {
            let next = match a[1] {
                5 => 9u32,
                9 => 2,
                _ => u32::MAX,
            };
            next.into_ret()
        });
        let container = e.mem.alloc(8);
        e.mem.set_u32(container, VTABLE);
        assert_eq!(e.call(0x00d1_f1e0, &args![container]).i32(), 3);
        let nexts: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == NEXT_KEY)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(nexts, vec![5, 9, 2]);

        // No child at all.
        e.register(FIRST_KEY, |_, _| u32::MAX.into_ret());
        assert_eq!(e.call(0x00d1_f1e0, &args![container]).i32(), 0);
    }

    #[test]
    fn single_container_returns_its_shape() {
        let mut e = Engine::new();
        let container = e.mem.alloc(8);
        e.mem.set_u32(container + 4, 0x7777);
        assert_eq!(
            e.call(0x00d1_f210, &args![container, 3u32, 0x9000u32])
                .u32(),
            0x7777
        );
    }
}
