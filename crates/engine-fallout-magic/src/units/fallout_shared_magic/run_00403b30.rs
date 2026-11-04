//! `fallout shared/magic/run_00403b30` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `EffectItem` constructor and destructor and the helpers they use
//! (the layout is `effectitem.rs`'s).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_magic::effectitem::{EffectItem, EffectItemData};

/// The `TESCondition` constructor, clean-up and destructor (thiscall, on
/// `EffectItem::Conditions`, +0x1c).
const CONDITION_CONSTRUCT: u32 = 0x0068_0890;
const CONDITION_CLEAR: u32 = 0x0068_0fe0;
const CONDITION_DESTRUCT: u32 = 0x0068_08b0;
/// Thiscall: `this + 0x1c` (`enchantmentitem.rs`); here the conditions.
const PLUS_1C: u32 = 0x0040_9140;
/// Thiscall on an effect setting: the word at +0x98 (`magicballprojectile.rs`;
/// the index `00403c80` looks up).
const WORD_AT_98: u32 = 0x0081_0fd0;
/// Thiscall on an effect setting: the word at +0x9c (`tesworldspace.cpp`,
/// folded code).
const WORD_AT_9C: u32 = 0x0058_6150;
/// The table `00403c80` reads: 16-byte rows, the flags word first.
const FLAG_TABLE: u32 = 0x0118_3328;
/// `-1.0` (`float`).
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;

// Translated from 00403b30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00403b30 - the C++ exception frame (FS:[0] chain, unwind state 0 after the conditions are built) is not translated.
/// The `EffectItem` constructor (`thiscall`, `RET 4`; unnamed in the engine
/// map): the data's start values (`00403ba0`), the conditions (+0x1c,
/// `00680890`), then `00403be0(setting)`. Returns `this`.
pub fn fn_00403b30(e: &mut Engine, this: Ptr<EffectItem>, setting: u32) -> Ptr<EffectItem> {
    fn_00403ba0(e, Ptr::new(this.addr()));
    e.call(CONDITION_CONSTRUCT, &args![this.addr() + 0x1c]);
    fn_00403be0(e, this, setting);
    this
}

// Translated from 00403ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `EffectItemData` start values: magnitude,
/// area, duration and range 0, actor value -1. Returns `this`.
pub fn fn_00403ba0(e: &mut Engine, this: Ptr<EffectItemData>) -> Ptr<EffectItemData> {
    e.set(this, EffectItemData::iMagnitude, 0);
    e.set(this, EffectItemData::iArea, 0);
    e.set(this, EffectItemData::iDuration, 0);
    e.set(this, EffectItemData::iRange, 0);
    e.set(this, EffectItemData::iActorValue, 0xffff_ffff);
    this
}

// Translated from 00403be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): makes `setting` the
/// item's `pEffectSetting`, clears area, duration and magnitude, takes the actor value from
/// the setting (`00586150`) when `00403c60` says it has one (else -1), and
/// marks the raw cost unknown (-1.0).
pub fn fn_00403be0(e: &mut Engine, this: Ptr<EffectItem>, setting: u32) {
    e.set(this, EffectItem::pEffectSetting, Ptr::new(setting));
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr(), 0);
    let setting = e.get(this, EffectItem::pEffectSetting);
    let actor_value = if fn_00403c60(e, setting) {
        let setting = e.get(this, EffectItem::pEffectSetting);
        e.call(WORD_AT_9C, &args![setting]).u32()
    } else {
        0xffff_ffff
    };
    e.mem.set_u32(this.addr() + 0x10, actor_value);
    let invalid = x87_quiet_bits(e.mem.u32(MINUS_ONE_FLOAT));
    e.mem.set_u32(this.addr() + 0x18, invalid);
}

// Translated from 00403c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an effect setting): whether its
/// row of the table at `01183328` (index `00810fd0`) has flag 2
/// (`00403c80`; the result passes through `EAX`).
pub fn fn_00403c60(e: &mut Engine, this: Ptr) -> bool {
    let index = e.call(WORD_AT_98, &args![this]).u32();
    fn_00403c80(e, index, 2)
}

// Translated from 00403c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): whether the flags word of row `index`
/// of the 16-byte table at `01183328` has any bit of `mask`.
pub fn fn_00403c80(e: &mut Engine, index: u32, mask: u32) -> bool {
    let row = FLAG_TABLE.wrapping_add(index.wrapping_shl(4));
    e.mem.u32(row) & mask != 0
}

// Translated from 00403d50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00403d50 - the C++ exception frame (FS:[0] chain, unwind state 0 around the conditions' clean-up) is not translated.
/// The `EffectItem` destructor body (unnamed in the engine map): the
/// conditions cleared (`00680fe0` on `00409140(this)`, +0x1c) and destroyed
/// (`006808b0`).
pub fn fn_00403d50(e: &mut Engine, this: Ptr<EffectItem>) {
    let conditions = e.call(PLUS_1C, &args![this]).u32();
    e.call(CONDITION_CLEAR, &args![conditions]);
    e.call(CONDITION_DESTRUCT, &args![this.addr() + 0x1c]);
}

/// Thiscall on an effect item: its effect setting (`00825c00`, folded code).
const ITEM_EFFECT_SETTING_00825C00: u32 = 0x0082_5c00;
/// Thiscall on an effect item (source item), `RET 4` (`effectitem.cpp`):
/// copies the source's values into it.
const ITEM_COPY_VALUES_00404210: u32 = 0x0040_4210;

// Translated from 00403ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00403ca0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the conditions are built) is not translated.
/// The `EffectItem` copy constructor (`thiscall`, `RET 4`; unnamed in the
/// engine map): the data's start values (`00403ba0`), the conditions
/// (+0x1c, `00680890`), `00403be0` with `source`'s effect setting
/// (`00825c00`), then `pEffectSetting` (+0x14) cleared and
/// `00404210(source)`. Returns `this`.
pub fn fn_00403ca0(e: &mut Engine, this: Ptr<EffectItem>, source: u32) -> Ptr<EffectItem> {
    fn_00403ba0(e, Ptr::new(this.addr()));
    e.call(CONDITION_CONSTRUCT, &args![this.addr() + 0x1c]);
    let setting = e.call(ITEM_EFFECT_SETTING_00825C00, &args![source]).u32();
    fn_00403be0(e, this, setting);
    e.set(this, EffectItem::pEffectSetting, Ptr::new(0));
    e.call(ITEM_COPY_VALUES_00404210, &args![this, source]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0040_3b30,
            fn_00403b30(Ptr<EffectItem>, u32) -> Ptr<EffectItem>
        ),
        entry!(
            0x0040_3ba0,
            fn_00403ba0(Ptr<EffectItemData>) -> Ptr<EffectItemData>
        ),
        entry!(0x0040_3be0, fn_00403be0(Ptr<EffectItem>, u32)),
        entry!(0x0040_3c60, fn_00403c60(Ptr) -> bool),
        entry!(0x0040_3c80, fn_00403c80(u32, u32) -> bool),
        entry!(0x0040_3d50, fn_00403d50(Ptr<EffectItem>)),
        entry!(
            0x0040_3ca0,
            fn_00403ca0(Ptr<EffectItem>, u32) -> Ptr<EffectItem>
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

    /// The table and the constant mapped: row 3 has flag 2, row 4 not.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0118_3000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.set_global(FLAG_TABLE + 3 * 16, 0x0000_0006u32);
        e.set_global(FLAG_TABLE + 4 * 16, 0x0000_0001u32);
        e
    }

    #[test]
    fn table_flags() {
        let mut e = engine();
        assert!(e.call(0x0040_3c80, &args![3u32, 2u32]).bool());
        assert!(e.call(0x0040_3c80, &args![3u32, 4u32]).bool());
        assert!(!e.call(0x0040_3c80, &args![4u32, 2u32]).bool());
        e.register(WORD_AT_98, |_, a| Ret {
            eax: a[0] & 0xf,
            ..Ret::default()
        });
        assert!(e.call(0x0040_3c60, &args![0x5003u32]).bool());
        assert!(!e.call(0x0040_3c60, &args![0x5004u32]).bool());
    }

    #[test]
    fn data_start_values() {
        let mut e = engine();
        let data = e.mem.alloc(0x14);
        for word in 0..5 {
            e.mem.set_u32(data + 4 * word, 7);
        }
        assert_eq!(e.call(0x0040_3ba0, &args![data]).u32(), data);
        assert_eq!(e.mem.u32(data), 0);
        assert_eq!(e.mem.u32(data + 0xc), 0);
        assert_eq!(e.mem.u32(data + 0x10), 0xffff_ffff);
    }

    #[test]
    fn set_effect_setting_takes_the_actor_value_when_flagged() {
        let mut e = engine();
        e.register(WORD_AT_98, |_, a| Ret {
            eax: a[0] & 0xf,
            ..Ret::default()
        });
        e.register(WORD_AT_9C, |_, _| Ret {
            eax: 0x2a,
            ..Ret::default()
        });
        let item: Ptr<EffectItem> = e.new_object();
        e.mem.set_u32(item.addr() + 8, 9);
        e.call(0x0040_3be0, &args![item, 0x5003u32]);
        assert_eq!(e.get(item, EffectItem::pEffectSetting).addr(), 0x5003);
        assert_eq!(e.mem.u32(item.addr() + 8), 0);
        assert_eq!(e.mem.u32(item.addr() + 0x10), 0x2a);
        assert_eq!(e.get(item, EffectItem::fRawCost), -1.0);
        e.call(0x0040_3be0, &args![item, 0x5004u32]);
        assert_eq!(e.mem.u32(item.addr() + 0x10), 0xffff_ffff);
    }

    #[test]
    fn constructor_and_destructor() {
        let mut e = engine();
        e.register(WORD_AT_98, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        e.register(CONDITION_CONSTRUCT, |_, _| Ret::default());
        e.register(CONDITION_CLEAR, |_, _| Ret::default());
        e.register(CONDITION_DESTRUCT, |_, _| Ret::default());
        e.register(PLUS_1C, |_, a| Ret {
            eax: a[0] + 0x1c,
            ..Ret::default()
        });
        let item: Ptr<EffectItem> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0040_3b30, &args![item, 0x5004u32]).u32(),
            item.addr()
        );
        assert_eq!(
            calls_to(&e, CONDITION_CONSTRUCT),
            vec![vec![item.addr() + 0x1c]]
        );
        assert_eq!(e.mem.u32(item.addr() + 0x10), 0xffff_ffff);
        assert_eq!(e.get(item, EffectItem::pEffectSetting).addr(), 0x5004);
        e.call(0x0040_3d50, &args![item]);
        assert_eq!(
            calls_to(&e, CONDITION_CLEAR),
            vec![vec![item.addr() + 0x1c]]
        );
        assert_eq!(
            calls_to(&e, CONDITION_DESTRUCT),
            vec![vec![item.addr() + 0x1c]]
        );
    }
}

#[cfg(test)]
mod copy_tests {
    use super::*;

    #[test]
    fn copy_constructor_starts_from_the_source_setting_then_copies() {
        let mut e = Engine::new();
        e.map(0x0118_3000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.set_global(FLAG_TABLE + 3 * 16, 2u32);
        e.register(CONDITION_CONSTRUCT, |_, _| Ret::default());
        e.register(WORD_AT_98, |_, _| 3u32.into_ret());
        e.register(WORD_AT_9C, |_, _| 0x2au32.into_ret());
        e.register(ITEM_EFFECT_SETTING_00825C00, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(ITEM_COPY_VALUES_00404210, |e, a| {
            // The setting is cleared before the copy runs.
            assert_eq!(e.mem.u32(a[0] + 0x14), 0);
            Ret::default()
        });
        let source = e.mem.alloc(0x24);
        e.mem.set_u32(source + 0x14, 0x5000);
        let this = e.mem.alloc(0x24);
        e.mem.write(this, &[0xcc; 0x24]);
        e.call_log = Some(vec![]);
        let result = e.call(0x0040_3ca0, &args![this, source]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this + 0x10), 0x2a);
        assert_eq!(e.mem.u32(this + 0x18), (-1.0f32).to_bits());
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let log = e.call_log.take().unwrap();
        let watched = [
            CONDITION_CONSTRUCT,
            ITEM_EFFECT_SETTING_00825C00,
            ITEM_COPY_VALUES_00404210,
        ];
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| watched.contains(a))
            .collect();
        assert_eq!(order, watched.to_vec());
        let copies: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == ITEM_COPY_VALUES_00404210)
            .collect();
        assert_eq!(copies[0].1, vec![this, source]);
    }
}
