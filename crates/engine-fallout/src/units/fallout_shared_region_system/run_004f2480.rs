//! `fallout shared/region system/run_004f2480` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f2770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the address of
/// the member at +0x764 of `this`.
pub fn fn_004f2770(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x764)
}

/// `00413f40` (`thiscall`, no stack word): a list's first node.
const LIST_HEAD: u32 = 0x0041_3f40;
/// `006815c0` (`thiscall`, no stack word): the address of a node's item.
const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`, no stack word): the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `00564f00` (`thiscall`): whether the reference is flagged as an imposter.
const IS_IMPOSTER: u32 = 0x0056_4f00;
/// `00564f40` (`thiscall`, `RET 4`): sets that flag (called with 1).
const SET_IMPOSTER: u32 = 0x0056_4f40;
/// `0084e3a0` (`thiscall`): the form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40` (cdecl, variable arguments): a warning line with a category.
const WARNING: u32 = 0x005b_5e40;
/// `"SetImposterVisible: ObjectREFR is not flagged as an imposter (%s:%X)"`.
const NOT_AN_IMPOSTER_FORMAT: u32 = 0x0102_3bf0;
/// `00450f90` (`thiscall`, `RET 4`): run on the reference's 3D (vtable
/// `+0x1d0`) with a flag.
const NODE_FLAG_00450F90: u32 = 0x0045_0f90;
/// `0043b370` (`thiscall`, `RET 8`): sets or clears a flag mask on a node.
const SET_NODE_FLAGS: u32 = 0x0043_b370;
/// The flag mask `004f2620` passes (`0x10000000`).
const HIDDEN_MASK: u32 = 0x1000_0000;
/// The player pointer (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `TESObjectREFR::GetWorldLocation` (`00575650`, `thiscall`, `RET 4`: out
/// location), `BGSWorldLocation::GetCell` (`00527900`) and
/// `TESObjectCELL::GetRegionList` (`00547110`, `RET 4`: a flag).
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
const LOCATION_CELL: u32 = 0x0052_7900;
const CELL_REGION_LIST: u32 = 0x0054_7110;
/// `009611e0` (`thiscall`): a region's data list.
const REGION_DATA_LIST: u32 = 0x0096_11e0;
/// `TESRegionDataList::Find` (`004f35b0`, `thiscall`, `RET 4`: the type).
const DATA_LIST_FIND: u32 = 0x004f_35b0;

// Translated from 004f2480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: two bytes): runs
/// `004f24e0` with both bytes on each non-empty item of the list `this`
/// (from its first node, `00413f40`).
pub fn fn_004f2480(e: &mut Engine, this: Ptr, visible: u8, force: u8) {
    let mut node = e.call(LIST_HEAD, &args![this]).u32();
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(item_address) != 0 {
            let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
            let item = e.mem.u32(item_address);
            fn_004f24e0(e, Ptr::new(item), visible, force);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 004f24e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: reference, two bytes; the warning
/// names it `SetImposterVisible`). Nothing for a null reference. A
/// reference not flagged as an imposter (`00564f00`) is flagged
/// (`00564f40` with 1) and a warning (category 0xe) names it (vtable
/// `+0x130`) and its form id. The node is the reference's 3D (vtable
/// `+0x1d0`) through its slot `+0x10` (none without 3D); without it `force`
/// counts as set. With `force`: when the 3D (`+0x1d0`) exists, `00450f90`
/// on it with `!visible`, then `004f2620` on it with `visible`. Without:
/// `visible` runs `00450f90` with 0 on the 3D and `004f2620` with 1 on the
/// node; otherwise `004f2620` with 0 on the node.
pub fn fn_004f24e0(e: &mut Engine, reference: Ptr, visible: u8, force: u8) {
    if reference.is_null() {
        return;
    }
    let r = reference.addr();
    if !e.call(IS_IMPOSTER, &args![r]).bool() {
        e.call(SET_IMPOSTER, &args![r, 1u32]);
        let id = e.call(FORM_ID, &args![r]).u32();
        let name = e.vcall(r, 0x130, &args![]).u32();
        e.call(WARNING, &args![0xeu32, NOT_AN_IMPOSTER_FORMAT, name, id]);
    }
    let mut node = 0;
    if e.vcall(r, 0x1d0, &args![]).u32() != 0 {
        let root = e.vcall(r, 0x1d0, &args![]).u32();
        node = e.vcall(root, 0x10, &args![]).u32();
    }
    let force = if node == 0 { 1 } else { force };
    if force != 0 {
        if e.vcall(r, 0x1d0, &args![]).u32() != 0 {
            let hidden = u32::from(visible == 0);
            let root = e.vcall(r, 0x1d0, &args![]).u32();
            e.call(NODE_FLAG_00450F90, &args![root, hidden]);
            let root = e.vcall(r, 0x1d0, &args![]).u32();
            fn_004f2620(e, Ptr::new(root), visible);
        }
    } else if visible != 0 {
        let root = e.vcall(r, 0x1d0, &args![]).u32();
        e.call(NODE_FLAG_00450F90, &args![root, 0u32]);
        fn_004f2620(e, Ptr::new(node), 1);
    } else {
        fn_004f2620(e, Ptr::new(node), 0);
    }
}

// Translated from 004f2620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a byte): `0043b370` on
/// `this` with the byte and the mask `0x10000000`.
pub fn fn_004f2620(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(SET_NODE_FLAGS, &args![this, u32::from(flag), HIDDEN_MASK]);
}

// Translated from 004f2640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): whether `form` is among the
/// items of the type-8 data (`TESRegionDataList::Find`) of a region in the
/// player's region list (`004f2770` on the player, list at +4), or, when
/// that is missing or empty, of the regions of the player's cell
/// (`GetWorldLocation` with 1, `GetCell`, `GetRegionList`, +4).
pub fn fn_004f2640(e: &mut Engine, form: u32) -> bool {
    let player = e.global::<u32>(PLAYER);
    let regions = fn_004f2770(e, Ptr::new(player));
    let mut node = if regions == 0 {
        0
    } else {
        regions.wrapping_add(4)
    };
    let empty = node == 0 || {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        e.mem.u32(item_address) == 0
    };
    if empty {
        let player = e.global::<u32>(PLAYER);
        let regions = e.with_stack(0x10, |e, location| {
            // 004f2691: `00575650` takes one word (RET 4); the pushed 1 is the word of
            // `00547110` (RET 4).
            let location = e.call(GET_WORLD_LOCATION, &args![player, location]).u32();
            let cell = e.call(LOCATION_CELL, &args![location]).u32();
            e.call(CELL_REGION_LIST, &args![cell, 1u32]).u32()
        });
        node = if regions == 0 {
            0
        } else {
            regions.wrapping_add(4)
        };
    }
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            break;
        }
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let region = e.mem.u32(item_address);
        if region != 0 {
            let list = e.call(REGION_DATA_LIST, &args![region]).u32();
            let data = e.call(DATA_LIST_FIND, &args![list, 8u32]).u32();
            if data != 0 {
                let mut entry = e.call(LIST_HEAD, &args![data]).u32();
                while entry != 0 {
                    let item_address = e.call(NODE_ITEM_ADDRESS, &args![entry]).u32();
                    if e.mem.u32(item_address) != 0 {
                        let item_address = e.call(NODE_ITEM_ADDRESS, &args![entry]).u32();
                        if e.mem.u32(item_address) == form {
                            return true;
                        }
                    }
                    entry = e.call(NODE_NEXT, &args![entry]).u32();
                }
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f2770, fn_004f2770(Ptr) -> u32),
        entry!(0x004f_2480, fn_004f2480(Ptr, u8, u8)),
        entry!(0x004f_24e0, fn_004f24e0(Ptr, u8, u8)),
        entry!(0x004f_2620, fn_004f2620(Ptr, u8)),
        entry!(0x004f_2640, fn_004f2640(u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_member_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x004f_2770, &args![0x1000u32]).u32(), 0x1764);
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    /// Two-word nodes (item, next) for the list doubles.
    fn chain(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    fn lists(e: &mut Engine) {
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_004f2480_runs_each_item() {
        let mut e = Engine::new();
        lists(&mut e);
        let reference = e.mem.alloc(8);
        e.put_vtable(0x0150_0000, &[0; 0x80]);
        e.mem.set_u32(reference, 0x0150_0000);
        e.mem.set_u32(0x0150_0000 + 0x1d0, 0x0150_1000);
        e.register(0x0150_1000, |_, _| 0u32.into_ret());
        e.register(IS_IMPOSTER, |_, _| 1u32.into_ret());
        let head = chain(&mut e, &[reference, 0, reference]);
        e.register_double(LIST_HEAD, move |_, _| head.into_ret());
        e.register(SET_NODE_FLAGS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_2480, &args![0x10u32, 1u32, 0u32]);
        // No 3D: force, and nothing to do with no 3D, so each item asks
        // twice for its 3D.
        assert_eq!(calls_to(&e, IS_IMPOSTER).len(), 2);
        assert_eq!(calls_to(&e, 0x0150_1000).len(), 4);
    }

    fn reference_with_3d(e: &mut Engine, imposter: bool) -> (u32, u32, u32) {
        let reference = e.mem.alloc(8);
        let root = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        let mut slots = vec![0u32; 0x80];
        slots[0x130 / 4] = 0x0150_1100;
        slots[0x1d0 / 4] = 0x0150_1000;
        e.put_vtable(0x0150_0000, &slots);
        e.put_vtable(0x0151_0000, &[0, 0, 0, 0, 0x0151_1000]);
        e.mem.set_u32(reference, 0x0150_0000);
        e.mem.set_u32(root, 0x0151_0000);
        e.register_double(0x0150_1000, move |_, _| root.into_ret());
        e.register_double(0x0151_1000, move |_, _| node.into_ret());
        e.register(0x0150_1100, |_, _| 0x0160_0000u32.into_ret());
        if imposter {
            e.register(IS_IMPOSTER, |_, _| 1u32.into_ret());
        } else {
            e.register(IS_IMPOSTER, |_, _| 0u32.into_ret());
        }
        e.register(SET_IMPOSTER, |_, _| Ret::default());
        e.register(FORM_ID, |_, _| 0x1234u32.into_ret());
        e.register(WARNING, |_, _| Ret::default());
        e.register(NODE_FLAG_00450F90, |_, _| Ret::default());
        e.register(SET_NODE_FLAGS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (reference, root, node)
    }

    #[test]
    fn fn_004f24e0_warns_and_flags_a_non_imposter() {
        let mut e = Engine::new();
        let (reference, _, node) = reference_with_3d(&mut e, false);
        e.call(0x004f_24e0, &args![reference, 0u32, 0u32]);
        assert_eq!(calls_to(&e, SET_IMPOSTER), vec![vec![reference, 1]]);
        assert_eq!(
            calls_to(&e, WARNING),
            vec![vec![0xe, NOT_AN_IMPOSTER_FORMAT, 0x0160_0000, 0x1234]]
        );
        assert_eq!(
            calls_to(&e, SET_NODE_FLAGS),
            vec![vec![node, 0, HIDDEN_MASK]]
        );
        assert!(calls_to(&e, NODE_FLAG_00450F90).is_empty());
    }

    #[test]
    fn fn_004f24e0_shows_the_node() {
        let mut e = Engine::new();
        let (reference, root, node) = reference_with_3d(&mut e, true);
        e.call(0x004f_24e0, &args![reference, 1u32, 0u32]);
        assert!(calls_to(&e, WARNING).is_empty());
        assert_eq!(calls_to(&e, NODE_FLAG_00450F90), vec![vec![root, 0]]);
        assert_eq!(
            calls_to(&e, SET_NODE_FLAGS),
            vec![vec![node, 1, HIDDEN_MASK]]
        );
    }

    #[test]
    fn fn_004f24e0_forced_works_on_the_3d() {
        let mut e = Engine::new();
        let (reference, root, _) = reference_with_3d(&mut e, true);
        e.call(0x004f_24e0, &args![reference, 0u32, 1u32]);
        assert_eq!(calls_to(&e, NODE_FLAG_00450F90), vec![vec![root, 1]]);
        assert_eq!(
            calls_to(&e, SET_NODE_FLAGS),
            vec![vec![root, 0, HIDDEN_MASK]]
        );
        e.call_log = Some(vec![]);
        e.call(0x004f_24e0, &args![0u32, 0u32, 1u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_004f2620_passes_the_mask() {
        let mut e = Engine::new();
        e.register(SET_NODE_FLAGS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_2620, &args![0x40u32, 0x101u32]);
        assert_eq!(
            calls_to(&e, SET_NODE_FLAGS),
            vec![vec![0x40, 1, HIDDEN_MASK]]
        );
    }

    fn region_with_items(e: &mut Engine, items: &[u32]) -> u32 {
        let region = e.mem.alloc(8);
        let entries = chain(e, items);
        e.mem.set_u32(region, entries);
        region
    }

    fn data_doubles(e: &mut Engine) {
        e.register(REGION_DATA_LIST, |_, a| a[0].into_ret());
        e.register(DATA_LIST_FIND, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_HEAD, |_, a| a[0].into_ret());
    }

    #[test]
    fn fn_004f2640_finds_the_form_in_the_players_regions() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        lists(&mut e);
        data_doubles(&mut e);
        let player = e.mem.alloc(0x800);
        e.set_global::<u32>(PLAYER, player);
        let empty_region = e.mem.alloc(8);
        let region = region_with_items(&mut e, &[0, 0x55, 0x77]);
        let regions = chain(&mut e, &[empty_region, region]);
        e.mem.set_u32(player + 0x768, e.mem.u32(regions));
        e.mem.set_u32(player + 0x76c, e.mem.u32(regions + 4));
        assert!(e.call(0x004f_2640, &args![0x77u32]).bool());
        assert!(!e.call(0x004f_2640, &args![0x66u32]).bool());
    }

    #[test]
    fn fn_004f2640_falls_back_to_the_cells_regions() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        lists(&mut e);
        data_doubles(&mut e);
        let player = e.mem.alloc(0x800);
        e.set_global::<u32>(PLAYER, player);
        let region = region_with_items(&mut e, &[0x99]);
        let holder = e.mem.alloc(12);
        e.mem.set_u32(holder + 4, region);
        e.register(GET_WORLD_LOCATION, |_, a| a[1].into_ret());
        e.register(LOCATION_CELL, |_, _| 0x4444u32.into_ret());
        e.register_double(CELL_REGION_LIST, move |_, _| holder.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_2640, &args![0x99u32]).bool());
        assert_eq!(calls_to(&e, GET_WORLD_LOCATION)[0][0], player);
        assert_eq!(calls_to(&e, GET_WORLD_LOCATION)[0].len(), 2);
        assert_eq!(calls_to(&e, CELL_REGION_LIST), vec![vec![0x4444, 1]]);
    }
}
