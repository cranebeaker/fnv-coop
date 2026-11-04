//! `fallout/interface/run_007832a0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ItemChange::GetWorn` (`004bddd0`, `RET 4`), the item's form (`0044ddc0`), the
/// inventory menu (`011d9ea4`) whose list at +0x11c the form is added to (`005ae3d0`,
/// `RET 4`, the address of the word).
const ITEM_GET_WORN: u32 = 0x004b_ddd0;
const ITEM_FORM_0044DDC0: u32 = 0x0044_ddc0;
const INVENTORY_MENU: u32 = 0x011d_9ea4;
const LIST_ADD_005AE3D0: u32 = 0x005a_e3d0;

// Translated from 007832a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, two words: one not read and an item; a callback): when the item is
/// worn (`GetWorn(0)`), adds its form to the inventory menu's list at +0x11c through a
/// local holding it.
pub fn fn_007832a0(e: &mut Engine, _unused_1: u32, item: u32) {
    if item == 0 || !e.call(ITEM_GET_WORN, &args![item, 0u32]).bool() {
        return;
    }
    let form = e.call(ITEM_FORM_0044DDC0, &args![item]).u32();
    // The local at `EBP - 4`.
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), form);
        let menu = e.global::<u32>(INVENTORY_MENU);
        e.call(LIST_ADD_005AE3D0, &args![menu + 0x11c, local]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0078_32a0, fn_007832a0(u32, u32))]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn adds_worn_items_forms() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.set_global(INVENTORY_MENU, 0x7000u32);
        e.register(ITEM_GET_WORN, |_, a| u32::from(a[0] == 0x10).into_ret());
        e.register(ITEM_FORM_0044DDC0, |_, a| (a[0] + 0x100).into_ret());
        e.register(LIST_ADD_005AE3D0, |e, a| e.mem.u32(a[1]).into_ret());
        e.call_log = Some(vec![]);
        for item in [0u32, 0x20, 0x10] {
            e.call(0x0078_32a0, &args![0u32, item]);
        }
        let log = e.call_log.unwrap();
        let adds: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == LIST_ADD_005AE3D0)
            .collect();
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0].1[0], 0x711c);
        assert!(log.contains(&(ITEM_GET_WORN, vec![0x20, 0])));
    }
}
