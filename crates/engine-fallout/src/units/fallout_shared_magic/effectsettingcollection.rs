//! `fallout shared/magic/effectsettingcollection.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The effect setting list `Reset` clears and `00408df0` walks (a
/// `BSSimpleList` of `EffectSetting *` at `011c3400`).
const EFFECT_SETTINGS: u32 = 0x011c_3400;
/// The usage monitor effect setting (`CreateDefaultEffects` stores the last
/// effect it makes here).
const USAGE_MONITOR_EFFECT: u32 = 0x011c_33f4;
/// Thiscall: returns `this` (on a list node: the address of its item).
const SELF: u32 = 0x0068_15c0;
/// Thiscall: the word at +4 (on a list node: the next node).
const NEXT_NODE: u32 = 0x0072_6070;
/// Thiscall on a list: removes its items.
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// List destructor, thiscall.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// Cdecl (form id): the form with that id, or 0 (`tesform.cpp`).
const LOOK_UP_FORM_004839C0: u32 = 0x0048_39c0;
/// The allocator, cdecl (size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `EffectSetting` constructor (0xb0 bytes), thiscall.
const EFFECT_SETTING_CONSTRUCT: u32 = 0x0040_7020;
/// The six setters `00408e40` calls on the new effect setting, each thiscall
/// with one word (`RET 4`): `LowProcess::SetNumberOfItemsActivated`
/// (`005a8060`, engine map; folded code), `00408570` (`effectsetting.cpp`),
/// `004ee4b0` (a float), `00810570`, `MiddleHighProcess::SetAcquireObject`
/// (`008d8500`, folded code), `MiddleHighProcess::SetSavedAcquireObject`
/// (`00407800`, folded code) and `004078a0`.
const SETTER_005A8060: u32 = 0x005a_8060;
const SETTER_00408570: u32 = 0x0040_8570;
const SETTER_004EE4B0: u32 = 0x004e_e4b0;
const SETTER_00810570: u32 = 0x0081_0570;
const SETTER_008D8500: u32 = 0x008d_8500;
const SETTER_00407800: u32 = 0x0040_7800;
const SETTER_004078A0: u32 = 0x0040_78a0;
/// Cdecl (effect setting): adds it to the list (`run_00409060.rs`).
const ADD_EFFECT_SETTING: u32 = 0x0040_9060;

// Translated from 00408b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSettingCollection::GetUsageMonitorEffectSetting` (Xbox PDB): the
/// word at `011c33f4`.
pub fn effect_setting_collection_get_usage_monitor_effect_setting(e: &mut Engine) -> u32 {
    e.mem.u32(USAGE_MONITOR_EFFECT)
}

// Translated from 00408b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00408b60 - the C++ exception frame (FS:[0] chain, unwind state 0 around ClearData) is not translated.
/// Unnamed in the engine map (the decompiler's library match is wrong): the
/// collection's destructor shape: `ClearData`, then the list destroyed
/// (`0046ffb0`).
pub fn fn_00408b60(e: &mut Engine, this: Ptr) {
    effect_setting_collection_clear_data(e, this);
    e.call(LIST_DESTRUCT, &args![this]);
}

// Translated from 00408bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSettingCollection::ClearData` (Xbox PDB): for every node with an
/// item, the item's deleting destructor (vtable +0x10 with 1); stops at the
/// first node without one. Then the list's items are removed (`00470470`).
pub fn effect_setting_collection_clear_data(e: &mut Engine, this: Ptr) {
    let mut node = this.addr();
    while node != 0 {
        let slot = e.call(SELF, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item == 0 {
            break;
        }
        // The code tests the item for null again before the call.
        if item != 0 {
            e.vcall(item, 0x10, &args![1u32]);
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    e.call(LIST_REMOVE_ALL, &args![this]);
}

// Translated from 00408de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSettingCollection::Reset` (Xbox PDB): `ClearData` on the list at
/// `011c3400`.
pub fn effect_setting_collection_reset(e: &mut Engine) {
    effect_setting_collection_clear_data(e, Ptr::new(EFFECT_SETTINGS));
}

// Translated from 00408df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: walks the list at `011c3400` and calls vtable
/// +0x88 on each item, stopping at the first node without one.
pub fn fn_00408df0(e: &mut Engine) {
    let mut node = EFFECT_SETTINGS;
    while node != 0 {
        let slot = e.call(SELF, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item == 0 {
            break;
        }
        e.vcall(item, 0x88, &args![]);
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
}

// Translated from 00408e40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00408e40 - the C++ exception frame (FS:[0] chain, unwind state 0 around the allocation) is not translated.
/// Unnamed in the engine map (cdecl, nine words): when no form has the id
/// `form_id` (`004839c0`), makes an `EffectSetting` (0xb0 bytes, `00407020`;
/// the code calls on through a null pointer when the allocation fails),
/// gives it the id (vtable +0x128 with `form_id`, 1) and the editor id
/// (vtable +0x134), the seven values through their setters, and adds it to
/// the list (`00409060`). Returns it, or 0 when the id is taken.
pub fn fn_00408e40(
    e: &mut Engine,
    form_id: u32,
    editor_id: u32,
    name: u32,
    acquire: u32,
    value: f32,
    value_00810570: u32,
    items_activated: u32,
    saved_acquire: u32,
    value_004078a0: u32,
) -> u32 {
    if e.call(LOOK_UP_FORM_004839C0, &args![form_id]).u32() != 0 {
        return 0;
    }
    let block = e.call(ALLOCATE, &args![0xb0u32]).u32();
    let setting = if block != 0 {
        e.call(EFFECT_SETTING_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.vcall(setting, 0x128, &args![form_id, 1u32]);
    e.vcall(setting, 0x134, &args![editor_id]);
    e.call(SETTER_005A8060, &args![setting, items_activated]);
    e.call(SETTER_00408570, &args![setting, name]);
    e.call(
        SETTER_004EE4B0,
        &args![setting, x87_quiet_bits(value.to_bits())],
    );
    e.call(SETTER_00810570, &args![setting, value_00810570]);
    e.call(SETTER_008D8500, &args![setting, acquire]);
    e.call(SETTER_00407800, &args![setting, saved_acquire]);
    e.call(SETTER_004078A0, &args![setting, value_004078a0]);
    e.call(ADD_EFFECT_SETTING, &args![setting]);
    setting
}

// Translated from 00408f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSettingCollection::CreateDefaultEffects` (Xbox PDB): makes the six
/// built-in effect settings (`00408e40`): `"ScriptEffect"` (0x14a),
/// `"FireDamage"` (0x14b), `"FrostDamage"` (0x14c), `"ShockDamage"`
/// (0x14d), `"RestoreHealth"` (0x14e) and the usage monitor effect (0x14f,
/// `"UMON"`), which it keeps at `011c33f4`.
pub fn effect_setting_collection_create_default_effects(e: &mut Engine) {
    const NONE: u32 = 0xffff_ffff;
    let zero = 0.0f32;
    // (form id, editor id, name, then the words after the float).
    let effects: [(u32, u32, u32, u32, u32, u32, u32); 6] = [
        (0x14a, 0x0101_2dec, 0x0101_2dfc, NONE, 0x170, NONE, 1),
        (0x14b, 0x0101_2dd4, 0x0101_2de0, 0x10, 0x2000_0475, 0x34, 0),
        (0x14c, 0x0101_2db8, 0x0101_2dc4, 0x10, 0x2000_0475, NONE, 0),
        (0x14d, 0x0101_2d9c, 0x0101_2da8, 0x10, 0x2000_0475, NONE, 0),
        (0x14e, 0x0101_2d7c, 0x0101_2d8c, 0x10, 0x70, NONE, 0),
        (0x14f, 0x0101_2d5c, 0x0101_2d64, NONE, 0x12, NONE, 1),
    ];
    let mut last = 0;
    for (form_id, editor_id, name, sixth, seventh, eighth, ninth) in effects {
        last = fn_00408e40(
            e, form_id, editor_id, name, NONE, zero, sixth, seventh, eighth, ninth,
        );
    }
    e.mem.set_u32(USAGE_MONITOR_EFFECT, last);
}

/// The cdecl entry of `00408e40` (nine words, the fifth a float).
fn fn_00408e40_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let result = fn_00408e40(
        e,
        a[0],
        a[1],
        a[2],
        a[3],
        f32::from_bits(a[4]),
        a[5],
        a[6],
        a[7],
        a[8],
    );
    Ret {
        eax: result,
        ..Ret::default()
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0040_8b50,
            effect_setting_collection_get_usage_monitor_effect_setting() -> u32
        ),
        entry!(0x0040_8b60, fn_00408b60(Ptr)),
        entry!(0x0040_8bc0, effect_setting_collection_clear_data(Ptr)),
        entry!(0x0040_8de0, effect_setting_collection_reset()),
        entry!(0x0040_8df0, fn_00408df0()),
        (0x0040_8e40, fn_00408e40_entry as AbiFn),
        entry!(
            0x0040_8f60,
            effect_setting_collection_create_default_effects()
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// List doubles: `006815c0` returns the node, `00726070` its +4 word.
    fn list_doubles(e: &mut Engine) {
        e.register(SELF, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(NEXT_NODE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
    }

    /// A two-item list at `head` (items `0x5000`, `0x5100` with the vtable
    /// at `0107_9000`).
    fn two_items(e: &mut Engine, head: u32) {
        e.put_vtable(0x0107_9000, &[0x0050_0000; 0x30]);
        for item in [0x5000u32, 0x5100] {
            e.map(item, 0x100);
            e.mem.set_u32(item, 0x0107_9000);
        }
        let second = e.mem.alloc(8);
        e.mem.set_u32(head, 0x5000);
        e.mem.set_u32(head + 4, second);
        e.mem.set_u32(second, 0x5100);
    }

    #[test]
    fn usage_monitor_effect_setting_is_the_global() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(USAGE_MONITOR_EFFECT, 0x1234);
        assert_eq!(e.call(0x0040_8b50, &args![]).u32(), 0x1234);
    }

    #[test]
    fn clear_data_deletes_each_item_then_empties_the_list() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        quiet(&mut e, &[LIST_REMOVE_ALL, 0x0050_0010]);
        e.put_vtable(0x0107_9000, &[0x0050_0000; 0x30]);
        let head = e.mem.alloc(8);
        two_items(&mut e, head);
        e.mem.set_u32(0x0107_9010, 0x0050_0010);
        e.call_log = Some(vec![]);
        e.call(0x0040_8bc0, &args![head]);
        assert_eq!(
            calls_to(&e, 0x0050_0010),
            vec![vec![0x5000, 1], vec![0x5100, 1]]
        );
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL), vec![vec![head]]);
        // An empty head node stops the walk at once.
        let empty = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0040_8bc0, &args![empty]);
        assert!(calls_to(&e, 0x0050_0010).is_empty());
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL), vec![vec![empty]]);
    }

    #[test]
    fn destructor_shape_clears_then_destroys_the_list() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        quiet(&mut e, &[LIST_REMOVE_ALL, LIST_DESTRUCT]);
        let head = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0040_8b60, &args![head]);
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL), vec![vec![head]]);
        assert_eq!(calls_to(&e, LIST_DESTRUCT), vec![vec![head]]);
    }

    #[test]
    fn reset_clears_the_global_list() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        list_doubles(&mut e);
        quiet(&mut e, &[LIST_REMOVE_ALL]);
        e.call_log = Some(vec![]);
        e.call(0x0040_8de0, &args![]);
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL), vec![vec![EFFECT_SETTINGS]]);
    }

    #[test]
    fn walk_calls_slot_88_on_each_item() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        list_doubles(&mut e);
        two_items(&mut e, EFFECT_SETTINGS);
        e.mem.set_u32(0x0107_9088, 0x0050_0088);
        quiet(&mut e, &[0x0050_0088]);
        e.call_log = Some(vec![]);
        e.call(0x0040_8df0, &args![]);
        assert_eq!(calls_to(&e, 0x0050_0088), vec![vec![0x5000], vec![0x5100]]);
    }

    /// Doubles for `00408e40`: the new effect setting is `0x5000` with the
    /// vtable at `0107_9000`.
    fn create_doubles(e: &mut Engine, taken: u32) {
        e.register_double(LOOK_UP_FORM_004839C0, move |_, _| Ret {
            eax: taken,
            ..Ret::default()
        });
        e.put_vtable(0x0107_9000, &[0x0050_0000; 0x50]);
        e.mem.set_u32(0x0107_9128, 0x0050_0128);
        e.mem.set_u32(0x0107_9134, 0x0050_0134);
        e.map(0x5000, 0x100);
        e.mem.set_u32(0x5000, 0x0107_9000);
        e.register(ALLOCATE, |_, _| Ret {
            eax: 0x5000,
            ..Ret::default()
        });
        e.register(EFFECT_SETTING_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        quiet(
            e,
            &[
                0x0050_0128,
                0x0050_0134,
                SETTER_005A8060,
                SETTER_00408570,
                SETTER_004EE4B0,
                SETTER_00810570,
                SETTER_008D8500,
                SETTER_00407800,
                SETTER_004078A0,
                ADD_EFFECT_SETTING,
            ],
        );
    }

    #[test]
    fn create_effect_setting_sets_every_value() {
        let mut e = Engine::new();
        create_doubles(&mut e, 0);
        e.call_log = Some(vec![]);
        let words =
            args![0x14bu32, 0x11u32, 0x22u32, 0x33u32, 1.5f32, 0x66u32, 0x77u32, 0x88u32, 0x99u32];
        assert_eq!(e.call(0x0040_8e40, &words).u32(), 0x5000);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0xb0]]);
        assert_eq!(calls_to(&e, 0x0050_0128), vec![vec![0x5000, 0x14b, 1]]);
        assert_eq!(calls_to(&e, 0x0050_0134), vec![vec![0x5000, 0x11]]);
        assert_eq!(calls_to(&e, SETTER_005A8060), vec![vec![0x5000, 0x77]]);
        assert_eq!(calls_to(&e, SETTER_00408570), vec![vec![0x5000, 0x22]]);
        assert_eq!(
            calls_to(&e, SETTER_004EE4B0),
            vec![vec![0x5000, 1.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, SETTER_00810570), vec![vec![0x5000, 0x66]]);
        assert_eq!(calls_to(&e, SETTER_008D8500), vec![vec![0x5000, 0x33]]);
        assert_eq!(calls_to(&e, SETTER_00407800), vec![vec![0x5000, 0x88]]);
        assert_eq!(calls_to(&e, SETTER_004078A0), vec![vec![0x5000, 0x99]]);
        assert_eq!(calls_to(&e, ADD_EFFECT_SETTING), vec![vec![0x5000]]);
        // A taken id makes nothing.
        create_doubles(&mut e, 0x4000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_8e40, &words).u32(), 0);
        assert!(calls_to(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn default_effects_are_made_in_order_and_the_last_kept() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        create_doubles(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0040_8f60, &args![]);
        let ids: Vec<u32> = calls_to(&e, 0x0050_0128).iter().map(|c| c[1]).collect();
        assert_eq!(ids, vec![0x14a, 0x14b, 0x14c, 0x14d, 0x14e, 0x14f]);
        let activated: Vec<u32> = calls_to(&e, SETTER_005A8060).iter().map(|c| c[1]).collect();
        assert_eq!(
            activated,
            vec![0x170, 0x2000_0475, 0x2000_0475, 0x2000_0475, 0x70, 0x12]
        );
        assert_eq!(calls_to(&e, SETTER_00407800)[1], vec![0x5000, 0x34]);
        assert_eq!(calls_to(&e, 0x0050_0134)[5], vec![0x5000, 0x0101_2d5c]);
        assert_eq!(e.mem.u32(USAGE_MONITOR_EFFECT), 0x5000);
    }
}
