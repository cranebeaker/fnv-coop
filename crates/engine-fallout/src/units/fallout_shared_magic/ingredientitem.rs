//! `fallout shared/magic/ingredientitem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004013e0` (thiscall on the form, returns `AL`; unnamed): when true, nothing is
/// initialized.
const FORM_TEST_004013E0: u32 = 0x0040_13e0;
/// `TESScriptableForm::InitItem` (`0048cd90`, thiscall: the owner form).
const TES_SCRIPTABLE_FORM_INIT_ITEM: u32 = 0x0048_cd90;
/// `BGSDestructibleObjectForm::InitItem` (`00478be0`, thiscall: the owner form).
const BGS_DESTRUCTIBLE_OBJECT_FORM_INIT_ITEM: u32 = 0x0047_8be0;
/// `TESModelTextureSwap::InitItem` (`0048aa80`, thiscall: the owner form).
const TES_MODEL_TEXTURE_SWAP_INIT_ITEM: u32 = 0x0048_aa80;
/// `MagicItemObject::InitItem` (`0040c9d0`).
const MAGIC_ITEM_OBJECT_INIT_ITEM: u32 = 0x0040_c9d0;

// Translated from 00409d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IngredientItem::InitItem` (Xbox PDB): unless `004013e0` says otherwise, initializes
/// the scriptable form component (+0x78), the destructible object component (+0x94)
/// and the model texture swap component (+0x4c), each with the form, then the
/// `MagicItemObject` part.
pub fn ingredient_item_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_TEST_004013E0, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(TES_SCRIPTABLE_FORM_INIT_ITEM, &args![base + 0x78, this]);
    e.call(
        BGS_DESTRUCTIBLE_OBJECT_FORM_INIT_ITEM,
        &args![base + 0x94, this],
    );
    e.call(TES_MODEL_TEXTURE_SWAP_INIT_ITEM, &args![base + 0x4c, this]);
    e.call(MAGIC_ITEM_OBJECT_INIT_ITEM, &args![this]);
}

/// `TESModelTextureSwap::Save` (`0048a520`, thiscall: the three chunk tags,
/// `RET 0xc`), `TESTexture::Save` (`0048e370`, thiscall: the chunk tag, `RET
/// 4`), `TESScriptableForm::Save` (`0048cca0`), `BGSDestructibleObjectForm::Save`
/// (`00477f20`), `BGSEquipType::Save` (`00479350`) and `TESForm::SaveData`
/// (`00485e00`) (engine map), thiscall on the component.
const TES_MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
const TES_TEXTURE_SAVE: u32 = 0x0048_e370;
const TES_SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
const BGS_DESTRUCTIBLE_OBJECT_FORM_SAVE: u32 = 0x0047_7f20;
const BGS_EQUIP_TYPE_SAVE: u32 = 0x0047_9350;
const TES_FORM_SAVE_DATA: u32 = 0x0048_5e00;
/// The chunk tags `"MODL"`, `"MODT"`, `"MODS"` and `"ICON"`.
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_ICON: u32 = 0x4e4f_4349;
/// No arguments: `AL`, the byte at `011c54ba` (whether saved data has its
/// byte order swapped).
const SWAP_ON_SAVE_00401500: u32 = 0x0040_1500;
/// Thiscall: swaps the byte order of the word at `this` (`00503210`).
const SWAP_BYTES_00503210: u32 = 0x0050_3210;
/// `TESForm::__AddChunkData` (engine map), cdecl (chunk tag, data, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;

// Translated from 00409c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IngredientItem::SaveMagicItemComponents` (engine map; `thiscall` on the
/// component at +0x30 of the form): saves the model texture swap (+0x1c,
/// `"MODL"`, `"MODT"`, `"MODS"`), the icon (+0x3c, `"ICON"`), the script
/// (+0x48), the destructible data (+0x64) and the equip type (+0x5c), then
/// the form's data (`TESForm::SaveData` on the form, `this - 0x30`).
pub fn ingredient_item_save_magic_item_components(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(
        TES_MODEL_TEXTURE_SWAP_SAVE,
        &args![base + 0x1c, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
    );
    e.call(TES_TEXTURE_SAVE, &args![base + 0x3c, CHUNK_ICON]);
    e.call(TES_SCRIPTABLE_FORM_SAVE, &args![base + 0x48]);
    e.call(BGS_DESTRUCTIBLE_OBJECT_FORM_SAVE, &args![base + 0x64]);
    e.call(BGS_EQUIP_TYPE_SAVE, &args![base + 0x5c]);
    e.call(TES_FORM_SAVE_DATA, &args![base.wrapping_sub(0x30)]);
}

// Translated from 00409ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): adds the 8 bytes at
/// +0x6c as a chunk (`TESForm::__AddChunkData`) tagged with what the vtable
/// slot +0x24 returns; when `00401500` says saved data is byte-swapped, the
/// word at +0x6c is swapped (`00503210`) before and swapped back after.
pub fn fn_00409ce0(e: &mut Engine, this: Ptr) {
    let data = this.addr() + 0x6c;
    if e.call(SWAP_ON_SAVE_00401500, &args![]).bool() {
        e.call(SWAP_BYTES_00503210, &args![data]);
    }
    let tag = e.vcall(this.addr(), 0x24, &args![]).u32();
    e.call(ADD_CHUNK_DATA, &args![tag, data, 8u32]);
    if e.call(SWAP_ON_SAVE_00401500, &args![]).bool() {
        e.call(SWAP_BYTES_00503210, &args![data]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_9d40, ingredient_item_init_item(Ptr)),
        entry!(0x0040_9c70, ingredient_item_save_magic_item_components(Ptr)),
        entry!(0x0040_9ce0, fn_00409ce0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_item_initializes_the_components_unless_told_not_to() {
        for (skip, expected) in [(1u32, 0usize), (0, 4)] {
            let mut e = Engine::new();
            e.register_double(FORM_TEST_004013E0, move |_, _| Ret {
                eax: skip,
                ..Ret::default()
            });
            for callee in [
                TES_SCRIPTABLE_FORM_INIT_ITEM,
                BGS_DESTRUCTIBLE_OBJECT_FORM_INIT_ITEM,
                TES_MODEL_TEXTURE_SWAP_INIT_ITEM,
                MAGIC_ITEM_OBJECT_INIT_ITEM,
            ] {
                e.register(callee, |_, _| Ret::default());
            }
            e.call_log = Some(vec![]);
            e.call(0x0040_9d40, &args![0x5000u32]);
            let log: Vec<(u32, Vec<u32>)> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|c| c.0 != 0x0040_9d40 && c.0 != FORM_TEST_004013E0)
                .collect();
            assert_eq!(log.len(), expected);
            if expected != 0 {
                assert_eq!(
                    log,
                    vec![
                        (TES_SCRIPTABLE_FORM_INIT_ITEM, vec![0x5078, 0x5000]),
                        (BGS_DESTRUCTIBLE_OBJECT_FORM_INIT_ITEM, vec![0x5094, 0x5000]),
                        (TES_MODEL_TEXTURE_SWAP_INIT_ITEM, vec![0x504c, 0x5000]),
                        (MAGIC_ITEM_OBJECT_INIT_ITEM, vec![0x5000]),
                    ]
                );
            }
        }
    }
}

#[cfg(test)]
mod save_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn components_save_in_order() {
        let mut e = Engine::new();
        let saves = [
            TES_MODEL_TEXTURE_SWAP_SAVE,
            TES_TEXTURE_SAVE,
            TES_SCRIPTABLE_FORM_SAVE,
            BGS_DESTRUCTIBLE_OBJECT_FORM_SAVE,
            BGS_EQUIP_TYPE_SAVE,
            TES_FORM_SAVE_DATA,
        ];
        for addr in saves {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0040_9c70, &args![0x1030u32]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| saves.contains(a))
            .collect();
        assert_eq!(
            log,
            vec![
                (
                    TES_MODEL_TEXTURE_SWAP_SAVE,
                    vec![0x104c, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS]
                ),
                (TES_TEXTURE_SAVE, vec![0x106c, CHUNK_ICON]),
                (TES_SCRIPTABLE_FORM_SAVE, vec![0x1078]),
                (BGS_DESTRUCTIBLE_OBJECT_FORM_SAVE, vec![0x1094]),
                (BGS_EQUIP_TYPE_SAVE, vec![0x108c]),
                (TES_FORM_SAVE_DATA, vec![0x1000]),
            ]
        );
    }

    #[test]
    fn data_chunk_with_and_without_swapping() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x24, 0x0055_0000);
        e.register(0x0055_0000, |_, _| 0x4154_4144u32.into_ret());
        e.register(ADD_CHUNK_DATA, |_, _| Ret::default());
        e.register(SWAP_BYTES_00503210, |_, _| Ret::default());
        e.register(SWAP_ON_SAVE_00401500, |_, _| Ret::default());
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this, vtable);
        e.call_log = Some(vec![]);
        e.call(0x0040_9ce0, &args![this]);
        assert_eq!(
            calls_to(&e, ADD_CHUNK_DATA),
            vec![vec![0x4154_4144, this + 0x6c, 8]]
        );
        assert!(calls_to(&e, SWAP_BYTES_00503210).is_empty());
        e.register(SWAP_ON_SAVE_00401500, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        fn_00409ce0(&mut e, Ptr::new(this));
        let order: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|(a, _)| a)
            .filter(|a| *a == ADD_CHUNK_DATA || *a == SWAP_BYTES_00503210)
            .collect();
        assert_eq!(
            order,
            vec![SWAP_BYTES_00503210, ADD_CHUNK_DATA, SWAP_BYTES_00503210]
        );
    }
}
