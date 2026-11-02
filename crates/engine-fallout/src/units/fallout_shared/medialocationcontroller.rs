//! `fallout shared/medialocationcontroller.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `MediaLocationController` (Xbox PDB; fields 0x10 lower on PC, where
    /// `TESForm` is 0x18 bytes): the fields used so far.
    pub struct MediaLocationController: 0xB8 {
        /// `pFaction` (Xbox PDB): `TESFaction*`, a form id until `InitItem`.
        0x60 pFaction: u32,
    }
}

/// The six `BSSimpleList<MediaSet*>` lists [`media_location_controller_init_item`]
/// resolves, in order: `lConditionalNeutralList`, `lConditionalAllyList`,
/// `lConditionalFriendList`, `lConditionalEnemyList`, `lLocationList`,
/// `lBattleList` (Xbox PDB names).
const MEDIA_SET_LISTS: [u32; 6] = [0x88, 0x90, 0x98, 0xa0, 0xa8, 0xb0];

/// `004013e0` (`thiscall`): whether bit 3 (0x8) of the form's flag word at
/// +8 is set.
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `00484ab0` (`thiscall`, one byte): sets (non-zero) or clears bit 3 of the
/// form's flag word at +8.
const FORM_SET_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (`00484e60`, engine map; `thiscall`, one word).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, engine map; `cdecl`: `&form_id`,
/// file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The form lookup by form id (`004839c0`, `cdecl`).
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor `.?AVTESForm@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// RTTI type descriptor `.?AVMediaSet@@`.
const TYPE_MEDIA_SET: u32 = 0x0118_6168;
/// `TESDataHandler::GetFaction` (`00461600`, engine map; `thiscall`, one
/// word: the form id).
const DATA_HANDLER_GET_FACTION: u32 = 0x0046_1600;
/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The warning printer (`005b5e40`, `cdecl`, format and arguments).
const PRINT_WARNING: u32 = 0x005b_5e40;
/// "MASTERFILE: Could not find reputation (%08X) for MediaLocationController"
const MISSING_REPUTATION: u32 = 0x0103_4248;
/// "MASTERFILE: Could not find MediaSet (%08X) for %s."
const MISSING_MEDIA_SET: u32 = 0x0103_4210;
/// `008256d0` (`thiscall`): whether a list node is empty (no item and no
/// next node).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (`thiscall`): returns `this`; here a node's item word.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`): the word at +4; here the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `00726c60` (`thiscall`, one word: the address of a word): stores that
/// word as the node's item when it is non-zero.
const NODE_SET_ITEM: u32 = 0x0072_6c60;
/// `0063f7b0` (`thiscall` on a list node): called on a first node whose
/// media set is missing.
const NODE_REMOVE_0063F7B0: u32 = 0x0063_f7b0;
/// `00905330` (`thiscall` on the node before, one word: the address of a
/// word holding the item): called for a later node whose media set is
/// missing.
const LIST_REMOVE_00905330: u32 = 0x0090_5330;
/// `BSStringT` constructor (`004037b0`) and destructor (`004037d0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
/// `00559450` (`thiscall`): here the text of the `BSStringT`.
const STRING_CHARS_00559450: u32 = 0x0055_9450;
/// The form's vtable slot +0x90: fills a `BSStringT` with the form's name
/// for the warning.
const NAME_STRING_SLOT: u32 = 0x90;

// Translated from 005962e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MediaLocationController::InitItem` (Xbox PDB), `thiscall`: unless the
/// form's flag bit 0x8 is set, resolves the media sets of its six lists
/// ([`media_location_controller_init_item_list`]) and its reputation
/// faction (`pFaction`, through `TESDataHandler::GetFaction`; a missing one
/// is reported), then sets the flag bit.
pub fn media_location_controller_init_item(e: &mut Engine, this: Ptr<MediaLocationController>) {
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    for offset in MEDIA_SET_LISTS {
        media_location_controller_init_item_list(e, this, this.addr() + offset);
    }
    if e.get(this, MediaLocationController::pFaction) != 0 {
        e.with_stack(4, |e, form_id| {
            let id = e.get(this, MediaLocationController::pFaction);
            e.mem.set_u32(form_id.addr(), id);
            let file = e.call(FORM_GET_FILE, &args![this, -1i32]).u32();
            e.call(FORM_ADD_COMPILE_INDEX, &args![form_id, file]);
            let id = e.mem.u32(form_id.addr());
            let handler: u32 = e.global(DATA_HANDLER);
            let faction = e.call(DATA_HANDLER_GET_FACTION, &args![handler, id]).u32();
            e.set(this, MediaLocationController::pFaction, faction);
            if e.get(this, MediaLocationController::pFaction) == 0 {
                let missing = e.get(this, MediaLocationController::pFaction);
                e.call(PRINT_WARNING, &args![MISSING_REPUTATION, missing]);
            }
        });
    }
    e.call(FORM_SET_FLAG_8, &args![this, 1u32]);
}

// Translated from 00596190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00596190 - the C++ exception unwinding frame (FS:[0], handler 00ef4558) is not translated
/// `MediaLocationController::InitItemList` (Xbox PDB), `thiscall (this;
/// list)`: walks the list's nodes until a null or empty node, resolving each
/// item's form id (the form's file compile index, looked up and cast to
/// `MediaSet`). A found set is stored in the node (`00726c60`) and the walk
/// moves on. A missing one is reported with the form's name (a temporary
/// `BSStringT` its vtable +0x90 fills) and taken out: after an earlier
/// node through `00905330` on that node (the walk continues after it),
/// else through `0063f7b0` on the node itself (the walk looks at it again).
/// The exception-unwinding frame is not translated.
pub fn media_location_controller_init_item_list(
    e: &mut Engine,
    this: Ptr<MediaLocationController>,
    list: u32,
) {
    let mut node = list;
    let mut previous = 0u32;
    // The exe's locals: +0 the form id, +4 the found set, +8 the string (8
    // bytes), +0x10 the copy of the id handed to 00905330.
    e.with_stack(0x14, |e, locals| {
        let base = locals.addr();
        let form_id = base;
        let found = base + 4;
        let string = base + 8;
        let copy = base + 0x10;
        while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            e.mem.set_u32(form_id, e.mem.u32(slot));
            let file = e.call(FORM_GET_FILE, &args![this, -1i32]).u32();
            e.call(FORM_ADD_COMPILE_INDEX, &args![form_id, file]);
            let id = e.mem.u32(form_id);
            let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
            let set = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_MEDIA_SET, 0u32],
                )
                .u32();
            e.mem.set_u32(found, set);
            if set == 0 {
                e.call(STRING_CONSTRUCT, &args![string]);
                e.vcall(this.addr(), NAME_STRING_SLOT, &args![string]);
                let text = e.call(STRING_CHARS_00559450, &args![string]).u32();
                let id = e.mem.u32(form_id);
                e.call(PRINT_WARNING, &args![MISSING_MEDIA_SET, id, text]);
                if previous != 0 {
                    e.mem.set_u32(copy, e.mem.u32(form_id));
                    e.call(LIST_REMOVE_00905330, &args![previous, copy]);
                    node = e.call(NODE_NEXT, &args![previous]).u32();
                } else {
                    e.call(NODE_REMOVE_0063F7B0, &args![node]);
                }
                e.call(STRING_DESTROY, &args![string]);
            } else {
                e.call(NODE_SET_ITEM, &args![node, found]);
                previous = node;
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0059_62e0,
            media_location_controller_init_item(Ptr<MediaLocationController>)
        ),
        entry!(
            0x0059_6190,
            media_location_controller_init_item_list(Ptr<MediaLocationController>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME_STRING: u32 = 0x0001_0010;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Lists are (item, next) nodes; ids with bit 23 set are not found.
    fn engine() -> (Engine, Ptr<MediaLocationController>) {
        let mut e = Engine::new();
        let this = e.new_object::<MediaLocationController>();
        let mut slots = vec![0u32; 0x90 / 4 + 1];
        slots[0x90 / 4] = NAME_STRING;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(this.addr(), 0x0002_0000);
        e.register(FORM_GET_FILE, |_, _| word(0x7000));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM_BY_ID, |_, a| {
            word(if a[0] & 0x0080_0000 != 0 {
                0
            } else {
                a[0] + 0x10
            })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| word(a[0]));
        e.register(NODE_IS_EMPTY, |e, a| {
            word(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e.register(NODE_ITEM, |_, a| word(a[0]));
        e.register(NODE_NEXT, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(NODE_SET_ITEM, |e, a| {
            let value = e.mem.u32(a[1]);
            if value != 0 {
                e.mem.set_u32(a[0], value);
            }
            Ret::default()
        });
        // Unlinks the node after the given one.
        e.register(LIST_REMOVE_00905330, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        // Copies the next node in.
        e.register(NODE_REMOVE_0063F7B0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_DESTROY, |_, _| Ret::default());
        e.register(NAME_STRING, |_, _| Ret::default());
        e.register(STRING_CHARS_00559450, |_, _| word(0x9000));
        e.register(PRINT_WARNING, |_, _| Ret::default());
        e.register(FORM_FLAG_8_SET, |_, _| word(0));
        e.register(FORM_SET_FLAG_8, |_, _| Ret::default());
        (e, this)
    }

    fn list(e: &mut Engine, items: &[u32]) -> (u32, Vec<u32>) {
        let mut nodes = vec![];
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
            nodes.push(node);
        }
        nodes.reverse();
        (next, nodes)
    }

    #[test]
    fn init_item_list_resolves_and_drops_missing_sets() {
        let (mut e, this) = engine();
        // First missing (removed in place), then found, missing (removed
        // after the found one), found.
        let (head, nodes) = list(&mut e, &[0x0080_0001, 0x2, 0x0080_0003, 0x4]);
        e.call_log = Some(vec![]);
        e.call(0x0059_6190, &args![this, head]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(nodes[0]), 0x0100_0012);
        assert_eq!(e.mem.u32(nodes[0] + 4), nodes[3]);
        assert_eq!(e.mem.u32(nodes[3]), 0x0100_0014);
        assert_eq!(calls_to(&log, NODE_REMOVE_0063F7B0), vec![vec![nodes[0]]]);
        assert_eq!(calls_to(&log, LIST_REMOVE_00905330).len(), 1);
        assert_eq!(calls_to(&log, LIST_REMOVE_00905330)[0][0], nodes[0]);
        assert_eq!(
            calls_to(&log, PRINT_WARNING),
            vec![
                vec![MISSING_MEDIA_SET, 0x0180_0001, 0x9000],
                vec![MISSING_MEDIA_SET, 0x0180_0003, 0x9000]
            ]
        );
        assert_eq!(calls_to(&log, STRING_DESTROY).len(), 2);
    }

    #[test]
    fn init_item_resolves_the_lists_and_the_faction() {
        let (mut e, this) = engine();
        e.map(0x011c_3000, 0x1000);
        e.set_global(DATA_HANDLER, 0x0000_5000u32);
        let mut heads = vec![];
        for offset in MEDIA_SET_LISTS {
            // Each list is inline at its offset: one node, item `offset`.
            e.mem.set_u32(this.addr() + offset, offset);
            heads.push(this.addr() + offset);
        }
        e.set(this, MediaLocationController::pFaction, 0x55);
        e.register(DATA_HANDLER_GET_FACTION, |_, a| {
            assert_eq!(a[0], 0x5000);
            word(a[1] + 0x20)
        });
        e.call_log = Some(vec![]);
        e.call(0x0059_62e0, &args![this]);
        let log = e.call_log.take().unwrap();
        for offset in MEDIA_SET_LISTS {
            assert_eq!(e.mem.u32(this.addr() + offset), 0x0100_0010 + offset);
        }
        assert_eq!(e.get(this, MediaLocationController::pFaction), 0x0100_0075);
        assert!(calls_to(&log, PRINT_WARNING).is_empty());
        assert_eq!(
            log.last().unwrap(),
            &(FORM_SET_FLAG_8, vec![this.addr(), 1])
        );

        // A missing faction is reported (with the null it now holds).
        e.set(this, MediaLocationController::pFaction, 0x55);
        e.register(DATA_HANDLER_GET_FACTION, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x0059_62e0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PRINT_WARNING),
            vec![vec![MISSING_REPUTATION, 0]]
        );

        // An initialized form is left alone.
        e.register(FORM_FLAG_8_SET, |_, _| word(1));
        e.call_log = Some(vec![]);
        e.call(0x0059_62e0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
