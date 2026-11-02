//! `fallout shared/region system/run_004f3610` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `TESRegionDataManager` vtable (RTTI `.?AVTESRegionDataManager@@`).
const REGION_DATA_MANAGER_VTABLE: u32 = 0x0102_3ce4;

layout! {
    /// `TESRegionDataManager` (Xbox PDB), 8 bytes.
    pub struct TESRegionDataManager: 0x08 {
        /// `pLastLoadedRegion` (Xbox PDB): `TESRegion*`.
        0x04 pLastLoadedRegion: u32,
    }
}

// Translated from 004f3660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESRegionDataManager` constructor (no Xbox PDB name at this
/// address): stores the vtable, clears `pLastLoadedRegion` and returns
/// `this`.
pub fn fn_004f3660(e: &mut Engine, this: Ptr<TESRegionDataManager>) -> Ptr<TESRegionDataManager> {
    e.mem.set_u32(this.addr(), REGION_DATA_MANAGER_VTABLE);
    e.set(this, TESRegionDataManager::pLastLoadedRegion, 0);
    this
}

/// `006815c0` (`thiscall`, no stack word): the address of a node's item.
const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;

// Translated from 004f3610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): true for a null node or one
/// without an item; false when the item's vtable slot `+0x20` answers
/// false. The exe never moves to the next node: while the slot answers
/// true it asks the same node again (an endless loop unless the item
/// changes).
pub fn fn_004f3610(e: &mut Engine, node: u32) -> bool {
    loop {
        if node == 0 {
            return true;
        }
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            return true;
        }
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if !e.vcall(item, 0x20, &args![]).bool() {
            return false;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004f_3660,
            fn_004f3660(Ptr<TESRegionDataManager>) -> Ptr<TESRegionDataManager>
        ),
        entry!(0x004f_3610, fn_004f3610(u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_stores_the_vtable_and_clears_the_region() {
        let mut e = Engine::new();
        let this: Ptr<TESRegionDataManager> = e.new_object();
        e.set(this, TESRegionDataManager::pLastLoadedRegion, 0x1234);
        let back = e.call(0x004f_3660, &args![this]).u32();
        assert_eq!(back, this.addr());
        assert_eq!(e.mem.u32(this.addr()), REGION_DATA_MANAGER_VTABLE);
        assert_eq!(e.get(this, TESRegionDataManager::pLastLoadedRegion), 0);
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn fn_004f3610_answers_from_the_items_test() {
        let mut e = Engine::new();
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.put_vtable(0x0300_0000, &[0, 0, 0, 0, 0, 0, 0, 0, 0x0300_1000]);
        // The test answers true once and then clears the node's item, so
        // the second round ends on the empty item.
        e.register(0x0300_1000, |e, a| {
            let node = e.mem.u32(a[0] + 4);
            e.mem.set_u32(node, 0);
            1u32.into_ret()
        });
        e.register(0x0300_1004, |_, _| 0u32.into_ret());
        let node = e.mem.alloc(8);
        let item = e.mem.alloc(8);
        e.mem.set_u32(item, 0x0300_0000);
        e.mem.set_u32(item + 4, node);
        e.mem.set_u32(node, item);
        assert!(e.call(0x004f_3610, &args![node]).bool());
        assert!(e.call(0x004f_3610, &args![0u32]).bool());
        e.put_vtable(0x0300_0000, &[0, 0, 0, 0, 0, 0, 0, 0, 0x0300_1004]);
        e.mem.set_u32(node, item);
        assert!(!e.call(0x004f_3610, &args![node]).bool());
    }
}
