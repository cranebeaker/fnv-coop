//! `fallout shared/region system/tesregiondatalist.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (`thiscall`, no stack words): the list node's item address
/// (the code reads the item word through it).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`, no stack words): the next node of a list node.
const NODE_NEXT: u32 = 0x0072_6070;

// Translated from 004f35b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataList::Find` (Xbox PDB): walks the list that starts at `this`
/// and returns the first non-null item whose virtual slot +0x10 (the region
/// data's type) answers `data_type`; null when none does.
pub fn tes_region_data_list_find(e: &mut Engine, this: Ptr, data_type: u32) -> u32 {
    let mut node = this.addr();
    while node != 0 {
        let item_address = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if item != 0 && e.vcall(item, 0x10, &[]).u32() == data_type {
            return item;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x004f_35b0,
        tes_region_data_list_find(Ptr, u32) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_returns_the_first_item_of_the_type() {
        let mut e = Engine::new();
        // Three nodes of 8 bytes: item, next.
        let nodes = e.mem.alloc(0x18);
        let items = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0, 0, 0, 0, 0x0300_1000]);
        e.mem.set_u32(items, 0x0300_0000);
        e.mem.set_u32(items + 4, 3);
        e.mem.set_u32(items + 8, 0x0300_0000);
        e.mem.set_u32(items + 0xc, 5);
        e.register(0x0300_1000, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        // First node empty, then the type-3 item, then the type-5 item.
        e.mem.set_u32(nodes, 0);
        e.mem.set_u32(nodes + 4, nodes + 8);
        e.mem.set_u32(nodes + 8, items);
        e.mem.set_u32(nodes + 0xc, nodes + 0x10);
        e.mem.set_u32(nodes + 0x10, items + 8);
        e.mem.set_u32(nodes + 0x14, 0);
        e.register(NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(NODE_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        assert_eq!(e.call(0x004f_35b0, &args![nodes, 5u32]).u32(), items + 8);
        assert_eq!(e.call(0x004f_35b0, &args![nodes, 3u32]).u32(), items);
        assert_eq!(e.call(0x004f_35b0, &args![nodes, 7u32]).u32(), 0);
        assert_eq!(e.call(0x004f_35b0, &args![0u32, 3u32]).u32(), 0);
    }
}
