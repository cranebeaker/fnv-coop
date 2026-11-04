//! `fallout shared/tesrecipe.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `.?AVTESForm@@` and `.?AVTESRecipeCategory@@`, the type descriptors of the casts.
const TES_FORM_TYPE: u32 = 0x0118_3028;
const RECIPE_CATEGORY_TYPE: u32 = 0x0118_6490;
/// "MASTERFILE: Recipe Category (%08X) not found in InitItem for Recipe (%08X) '%s'."
const MISSING_CATEGORY: u32 = 0x0103_6cc0;
/// "MASTERFILE: Recipe (sub)Category (%08X) not found in InitItem for Recipe (%08X) '%s'."
const MISSING_SUBCATEGORY: u32 = 0x0103_6c68;
/// `00430830` (`ECX` the recipe): the address of its data block (category ID at +8,
/// subcategory ID at +0xc).
const RECIPE_DATA: u32 = 0x0043_0830;
/// List walking: `006815c0` gives the node (its first word is the item), `00726070` the
/// word at +4 (of a node, the next node; of an item, its form ID).
const NODE_ITEM: u32 = 0x0068_15c0;
const WORD_AT_FOUR: u32 = 0x0072_6070;

/// Resolves the category ID at `offset` of the recipe's data block: fixed up in a stack
/// slot through `00485d50` (cdecl, the slot and the recipe's file from `00484e60` with
/// -1), looked up with `004839c0` and cast to `TESRecipeCategory`. A found category goes to
/// `setter` (`thiscall` on the recipe); a missing one prints `warning` with the ID, the
/// word `0084e3a0` gives for the recipe and its name (vtable slot +0x130).
fn resolve_category(e: &mut Engine, this: Ptr, offset: u32, setter: u32, warning: u32) {
    let data = e.call(RECIPE_DATA, &args![this]).u32();
    let loaded = e.mem.u32(data.wrapping_add(offset));
    if loaded == 0 {
        return;
    }
    let id = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), loaded);
        let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
        e.call(0x0048_5d50, &args![slot, file]);
        e.mem.u32(slot.addr())
    });
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let category = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, TES_FORM_TYPE, RECIPE_CATEGORY_TYPE, 0u32],
        )
        .u32();
    if category == 0 {
        let name = e.vcall(this.addr(), 0x130, &args![]).u32();
        let recipe_id = e.call(0x0084_e3a0, &args![this]).u32();
        e.call(0x005b_5e40, &args![warning, id, recipe_id, name]);
    } else {
        e.call(setter, &args![this, category]);
    }
}

/// Resolves each item of the list whose first node `first_node` is: walks until a null
/// node or a node without an item, and for each item gives `004839c0`'s form for the item's
/// word at +4 (read through `00726070`) to `006ecd40` on the item.
fn resolve_list(e: &mut Engine, first_node: u32) {
    let mut node = first_node;
    while node != 0 {
        let entry = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(entry) == 0 {
            break;
        }
        let entry = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(entry);
        node = e.call(WORD_AT_FOUR, &args![node]).u32();
        let id = e.call(WORD_AT_FOUR, &args![item]).u32();
        let form = e.call(0x0048_39c0, &args![id]).u32();
        e.call(0x006e_cd40, &args![item, form]);
    }
}

// Translated from 005a7e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRecipe::InitItem` (Xbox PDB; `ECX` only, plain `RET`): unless `004013e0` says
/// otherwise: `00680a20` (the conditions at +0x34) with the recipe; the first word of the
/// data block (`00430830`) is read and not used; the category (data +8, set through
/// `005a8040`) and the subcategory (data +0xc, set through `005a8060`) are resolved (see
/// `resolve_category`); the items of the lists `005a8080` and `005d43c0` give are resolved
/// (see `resolve_list`); then `00484ab0` with 1.
pub fn tes_recipe_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    e.call(0x0068_0a20, &args![this.addr() + 0x34, this]);
    let data = e.call(RECIPE_DATA, &args![this]).u32();
    let _first_word = e.mem.u32(data);
    resolve_category(e, this, 8, 0x005a_8040, MISSING_CATEGORY);
    resolve_category(e, this, 0xc, 0x005a_8060, MISSING_SUBCATEGORY);
    let inputs = e.call(0x005a_8080, &args![this]).u32();
    resolve_list(e, inputs);
    let outputs = e.call(0x005d_43c0, &args![this]).u32();
    resolve_list(e, outputs);
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005a_7e00, tes_recipe_init_item(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A recipe with a data block (category IDs `category`, `subcategory`), a vtable whose
    /// slot +0x130 gives 0x7777, the lists at `inputs`/`outputs` and doubles for the callees;
    /// casts give `found` for IDs below 0x0100_1000 and 0 above.
    fn recipe_case(
        e: &mut Engine,
        category: u32,
        subcategory: u32,
        inputs: u32,
        outputs: u32,
    ) -> u32 {
        let recipe = e.mem.alloc(0x60);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, 0x55);
        e.mem.set_u32(data + 8, category);
        e.mem.set_u32(data + 0xc, subcategory);
        e.put_vtable(0x0300_0000, &[0; 0x50]);
        e.mem.set_u32(0x0300_0000 + 0x130, 0x0300_1130);
        e.mem.set_u32(recipe, 0x0300_0000);
        e.register(0x0300_1130, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register(0x0040_13e0, |_, _| Ret::default());
        e.register_double(RECIPE_DATA, move |_, _| Ret {
            eax: data,
            ..Ret::default()
        });
        e.register(0x0048_4e60, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        e.register(0x00ec_43fb, |_, a| Ret {
            eax: if a[0] < 0x0100_1000 { a[0] + 1 } else { 0 },
            ..Ret::default()
        });
        e.register(NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(WORD_AT_FOUR, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, _| Ret {
            eax: 0xbeef,
            ..Ret::default()
        });
        e.register_double(0x005a_8080, move |_, _| Ret {
            eax: inputs,
            ..Ret::default()
        });
        e.register_double(0x005d_43c0, move |_, _| Ret {
            eax: outputs,
            ..Ret::default()
        });
        for address in [
            0x0068_0a20u32,
            0x005b_5e40,
            0x005a_8040,
            0x005a_8060,
            0x006e_cd40,
            0x0048_4ab0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        recipe
    }

    /// A list of items with these form IDs (each item: an 8-byte block, ID at +4).
    fn item_list(e: &mut Engine, ids: &[u32]) -> (u32, Vec<u32>) {
        let mut items = vec![];
        let first = e.mem.alloc(8);
        let mut node = first;
        for (index, id) in ids.iter().enumerate() {
            let item = e.mem.alloc(8);
            e.mem.set_u32(item + 4, *id);
            items.push(item);
            e.mem.set_u32(node, item);
            let next = if index + 1 < ids.len() {
                e.mem.alloc(8)
            } else {
                0
            };
            e.mem.set_u32(node + 4, next);
            node = next;
        }
        (first, items)
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn recipe_init_item_stops_when_checked() {
        let mut e = Engine::new();
        let recipe = recipe_case(&mut e, 0, 0, 0, 0);
        e.register(0x0040_13e0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x005a_7e00, &args![recipe]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn recipe_init_item_sets_found_categories_and_resolves_the_lists() {
        let mut e = Engine::new();
        let (inputs, input_items) = item_list(&mut e, &[0x11, 0x12]);
        let (outputs, output_items) = item_list(&mut e, &[0x21]);
        let recipe = recipe_case(&mut e, 0x30, 0x40, inputs, outputs);
        e.call_log = Some(vec![]);
        e.call(0x005a_7e00, &args![recipe]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0068_0a20),
            vec![vec![recipe + 0x34, recipe]]
        );
        assert_eq!(calls_to(&log, 0x005a_8040), vec![vec![recipe, 0x0100_0041]]);
        assert_eq!(calls_to(&log, 0x005a_8060), vec![vec![recipe, 0x0100_0051]]);
        assert!(calls_to(&log, 0x005b_5e40).is_empty());
        assert_eq!(
            calls_to(&log, 0x006e_cd40),
            vec![
                vec![input_items[0], 0x21],
                vec![input_items[1], 0x22],
                vec![output_items[0], 0x31],
            ]
        );
        assert_eq!(log.last().unwrap(), &(0x0048_4ab0, vec![recipe, 1]));
    }

    #[test]
    fn recipe_init_item_warns_about_missing_categories_and_skips_zero_ids() {
        let mut e = Engine::new();
        let (empty, _) = item_list(&mut e, &[]);
        let recipe = recipe_case(&mut e, 0x2000, 0, empty, 0);
        e.call_log = Some(vec![]);
        e.call(0x005a_7e00, &args![recipe]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x005a_8040).is_empty());
        assert_eq!(
            calls_to(&log, 0x005b_5e40),
            vec![vec![MISSING_CATEGORY, 0x0100_2000, 0xbeef, 0x7777]]
        );
        // The subcategory ID is 0: no lookup for it.
        assert_eq!(calls_to(&log, 0x0048_39c0).len(), 1);
        assert!(calls_to(&log, 0x006e_cd40).is_empty());

        let mut e = Engine::new();
        let recipe = recipe_case(&mut e, 0, 0x3000, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x005a_7e00, &args![recipe]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x005b_5e40),
            vec![vec![MISSING_SUBCATEGORY, 0x0100_3000, 0xbeef, 0x7777]]
        );
    }
}
