//! `fallout/interface/run_00701500` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00701500` calls on the member (`0045cec0`).
const CALLEE_0045CEC0: u32 = 0x0045_cec0;

// Translated from 00701500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0045cec0` with `ECX` = `this + 0x18` and passes on nothing (the callee returns no value).
pub fn fn_00701500(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0045CEC0, &args![this.byte_add(0x18)]);
}

/// The object at `011dea3c` whose inventory `fn_00701520` walks (it is also the second word
/// of `GetBipedTESModel`).
const OWNER_011DEA3C: u32 = 0x011d_ea3c;
/// `InventoryChanges::GetInventoryChanges` (engine map, `004bf220`, `cdecl`, one word).
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `004bf4b0` (`thiscall` on the inventory: the hot key): a new copy of the item change with
/// that hot key, or 0.
const ITEM_WITH_HOT_KEY: u32 = 0x004b_f4b0;
/// `0044ddc0` (`thiscall`, no stack words): the item change's form (its word at +8).
const ITEM_FORM: u32 = 0x0044_ddc0;
/// `TESBipedModelForm::GetBipedTESModel` (engine map, `00481090`, `cdecl`: form, owner).
const GET_BIPED_MODEL: u32 = 0x0048_1090;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, offset, source type, target type, by
/// reference) and the type descriptors `.?AVTESBoundObject@@` (`01183108`) and
/// `.?AVTESModelTextureSwap@@` (`011831c4`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_TES_MODEL_TEXTURE_SWAP: u32 = 0x0118_31c4;
/// The model loader at `011c3b3c` and its release by name (`0045a5e0`, `thiscall`: the
/// name).
const MODEL_LOADER_011C3B3C: u32 = 0x011c_3b3c;
const RELEASE_MODEL: u32 = 0x0045_a5e0;
/// The item change's scalar deleting destructor (`004459e0`, `thiscall`: flags).
const ITEM_DELETE: u32 = 0x0044_59e0;

// Translated from 00701520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; `this` is not read): for each hot
/// key 0 to 7 of the inventory of the object at `011dea3c`, takes the item change with that
/// hot key (`004bf4b0`); when it has a form, finds its model (`GetBipedTESModel(form,
/// owner)`, else the form cast from `TESBoundObject` to `TESModelTextureSwap`) and, when there
/// is one, releases the model named by its vtable slot +0x14 from the model loader at
/// `011c3b3c`. Every item change found is deleted (`004459e0` with 1).
pub fn fn_00701520(e: &mut Engine, _unused_this: Ptr) {
    let owner = e.mem.u32(OWNER_011DEA3C);
    let inventory = e.call(GET_INVENTORY_CHANGES, &args![owner]).u32();
    for hot_key in 0..8u32 {
        let item = e.call(ITEM_WITH_HOT_KEY, &args![inventory, hot_key]).u32();
        if item != 0 && e.call(ITEM_FORM, &args![item]).u32() != 0 {
            let owner = e.mem.u32(OWNER_011DEA3C);
            let form = e.call(ITEM_FORM, &args![item]).u32();
            let mut model = e.call(GET_BIPED_MODEL, &args![form, owner]).u32();
            if model == 0 {
                let form = e.call(ITEM_FORM, &args![item]).u32();
                model = e
                    .call(
                        DYNAMIC_CAST,
                        &args![
                            form,
                            0u32,
                            TYPE_TES_BOUND_OBJECT,
                            TYPE_TES_MODEL_TEXTURE_SWAP,
                            0u32
                        ],
                    )
                    .u32();
            }
            if model != 0 {
                let name = e.vcall(model, 0x14, &args![]).u32();
                let loader = e.mem.u32(MODEL_LOADER_011C3B3C);
                e.call(RELEASE_MODEL, &args![loader, name]);
            }
        }
        if item != 0 {
            e.call(ITEM_DELETE, &args![item, 1u32]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070_1500, fn_00701500(Ptr)),
        entry!(0x0070_1520, fn_00701520(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00701500_passes_this_plus_18_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0045CEC0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0070_1500, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0045CEC0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x18]);
    }
}

#[cfg(test)]
mod hot_key_tests {
    use super::*;

    const NAME_SLOT: u32 = 0x0e10_0000;
    const INVENTORY: u32 = 0x0e20_0000;
    const OWNER: u32 = 0x0e30_0000;
    const LOADER: u32 = 0x0e40_0000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Item changes for hot keys 1 (form with a biped model), 2 (form cast to a model), 3
    /// (form without either) and 5 (no form); the models answer their address + 1 as the
    /// name. Answers the engine and the four item changes.
    fn engine() -> (Engine, [u32; 4]) {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(OWNER_011DEA3C, OWNER);
        e.set_global(MODEL_LOADER_011C3B3C, LOADER);
        let name_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(name_vtable + 0x14, NAME_SLOT);
        let biped_model = e.mem.alloc(8);
        e.mem.set_u32(biped_model, name_vtable);
        let swap_model = e.mem.alloc(8);
        e.mem.set_u32(swap_model, name_vtable);
        // A form: +0 the biped model it answers, +4 the cast's answer.
        let mut items = [0u32; 4];
        for (i, (biped, swap, has_form)) in [
            (biped_model, 0u32, true),
            (0, swap_model, true),
            (0, 0, true),
            (0, 0, false),
        ]
        .into_iter()
        .enumerate()
        {
            let form = e.mem.alloc(8);
            e.mem.set_u32(form, biped);
            e.mem.set_u32(form + 4, swap);
            let item = e.mem.alloc(0x10);
            e.mem.set_u32(item + 8, if has_form { form } else { 0 });
            items[i] = item;
        }
        let table = e.mem.alloc(0x20);
        for (hot_key, item) in [
            (1u32, items[0]),
            (2, items[1]),
            (3, items[2]),
            (5, items[3]),
        ] {
            e.mem.set_u32(table + 4 * hot_key, item);
        }
        e.map(INVENTORY, 0x1000);
        e.mem.set_u32(INVENTORY, table);
        e.register(GET_INVENTORY_CHANGES, |_, _| INVENTORY.into_ret());
        e.register(ITEM_WITH_HOT_KEY, |e, a| {
            let table = e.mem.u32(a[0]);
            e.mem.u32(table + 4 * a[1]).into_ret()
        });
        e.register(ITEM_FORM, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(GET_BIPED_MODEL, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(DYNAMIC_CAST, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NAME_SLOT, |_, a| (a[0] + 1).into_ret());
        e.register(RELEASE_MODEL, |_, _| Ret::default());
        e.register(ITEM_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, items)
    }

    #[test]
    fn fn_00701520_releases_the_hot_key_models_and_deletes_the_items() {
        let (mut e, items) = engine();
        e.call(0x0070_1520, &args![0u32]);
        assert_eq!(calls_to(&e, GET_INVENTORY_CHANGES), vec![vec![OWNER]]);
        assert_eq!(
            calls_to(&e, ITEM_WITH_HOT_KEY),
            (0..8).map(|key| vec![INVENTORY, key]).collect::<Vec<_>>()
        );
        let forms: Vec<u32> = items[..3].iter().map(|item| e.mem.u32(item + 8)).collect();
        assert_eq!(
            calls_to(&e, GET_BIPED_MODEL),
            forms
                .iter()
                .map(|form| vec![*form, OWNER])
                .collect::<Vec<_>>()
        );
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            forms[1..]
                .iter()
                .map(|form| vec![
                    *form,
                    0,
                    TYPE_TES_BOUND_OBJECT,
                    TYPE_TES_MODEL_TEXTURE_SWAP,
                    0
                ])
                .collect::<Vec<_>>()
        );
        let biped = e.mem.u32(forms[0]);
        let swap = e.mem.u32(forms[1] + 4);
        assert_eq!(
            calls_to(&e, RELEASE_MODEL),
            vec![vec![LOADER, biped + 1], vec![LOADER, swap + 1]]
        );
        assert_eq!(
            calls_to(&e, ITEM_DELETE),
            items.iter().map(|item| vec![*item, 1]).collect::<Vec<_>>()
        );
    }
}
