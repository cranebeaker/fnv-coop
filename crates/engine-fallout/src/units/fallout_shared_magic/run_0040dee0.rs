//! `fallout shared/magic/run_0040dee0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `Setting` (Xbox PDB) base vtable, set by the base constructors.
const SETTING_VTABLE: u32 = 0x0101_2120;
/// The game-setting vtable `0040e0b0` sets after the base constructor.
const GAME_SETTING_VTABLE: u32 = 0x0101_2114;
/// The game-setting collection (`00404a70`: created on first use).
const GAME_SETTINGS: u32 = 0x0040_4a70;
/// Slot of the collection's vtable that adds a setting (`00404de0` for the
/// game settings).
const COLLECTION_ADD_SLOT: u32 = 0x4;

// Translated from 0040e0b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a float game setting (`thiscall`, `RET 8`): the base
/// constructor [`fn_0040e130`] with the name and the value (passed on through
/// the x87 stack), the game-setting vtable (`01012114`), then adds itself to
/// the game-setting collection (`00404a70`, its vtable slot +4). Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_0040e0b0(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    fn_0040e130(e, this, name, x87_quiet_bits(value));
    e.mem.set_u32(this.addr(), GAME_SETTING_VTABLE);
    let collection = e.call(GAME_SETTINGS, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

// Translated from 0040e130 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `Setting` (Xbox PDB) base constructor for a float value (`thiscall`,
/// `RET 8`): the base vtable (`01012120`), the name at +8 and the value at +4
/// (`FLD float`/`FSTP float`). Returns `this`.
pub fn fn_0040e130(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.mem.set_u32(this.addr(), SETTING_VTABLE);
    e.mem.set_u32(this.addr() + 8, name);
    e.mem.set_u32(this.addr() + 4, x87_quiet_bits(value));
    this
}

// Translated from 0040df90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 0 (cdecl; it reads no argument: one caller, `00804560`, pushes
/// one word and cleans it, others push none).
pub fn fn_0040df90(_e: &mut Engine) -> u32 {
    0
}

/// Thiscall on a float setting: the address of its value (+4; for a null
/// setting a zero stored at `01202800`) (`effectitem.cpp`).
const SETTING_FLOAT_ADDRESS_00403E20: u32 = 0x0040_3e20;
/// The float settings `0040df50` and `0040df70` read.
const FLOAT_SETTING_011C36D8: u32 = 0x011c_36d8;
const FLOAT_SETTING_011C3664: u32 = 0x011c_3664;
/// Thiscall on an effect item: its effect setting (`00825c00`, folded code).
const ITEM_EFFECT_SETTING: u32 = 0x0082_5c00;
/// Thiscall on an effect setting (flag mask), `RET 4`: `AL`.
const SETTING_HAS_FLAG: u32 = 0x0040_3e60;
/// Thiscall on an effect setting: its associated form (the word at +0x60).
const SETTING_ASSOCIATED_FORM: u32 = 0x005f_5f80;
/// `__RTDynamicCast` (CRT), cdecl, and the type descriptors of `TESForm`
/// and `TESBipedModelForm`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_BIPED_MODEL_FORM: u32 = 0x0118_3978;
/// Thiscall on a `TESBipedModelForm` (other form), `RET 4`: `AL`
/// (`tesbipedmodelform.cpp`).
const BIPED_FORM_00480AD0: u32 = 0x0048_0ad0;

// Translated from 0040dee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): the word at `011c3538`,
/// `011c353c`, `011c3540`, `011c3544`, `011c3548` or `011c354c` for `index`
/// 0 to 5 (unsigned; the switch's jump table), 0 for any other.
pub fn fn_0040dee0(e: &mut Engine, index: u32) -> u32 {
    let global = match index {
        0 => 0x011c_3538,
        1 => 0x011c_353c,
        2 => 0x011c_3540,
        3 => 0x011c_3544,
        4 => 0x011c_3548,
        5 => 0x011c_354c,
        _ => return 0,
    };
    e.mem.u32(global)
}

/// `FLD float` of the value of a float setting (`00403e20`).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e
        .call(SETTING_FLOAT_ADDRESS_00403E20, &args![setting])
        .u32();
    f32::from_bits(x87_quiet_bits(e.mem.u32(value)))
}

// Translated from 0040df50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments, a float in `ST0`): the value
/// of the float setting at `011c36d8`.
pub fn fn_0040df50(e: &mut Engine) -> f32 {
    setting_float(e, FLOAT_SETTING_011C36D8)
}

// Translated from 0040df70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments, a float in `ST0`): the value
/// of the float setting at `011c3664`.
pub fn fn_0040df70(e: &mut Engine) -> f32 {
    setting_float(e, FLOAT_SETTING_011C3664)
}

/// Whether the setting of the effect item `item` has a flag of `mask`.
fn item_setting_has_flag(e: &mut Engine, item: u32, mask: u32) -> bool {
    let setting = e.call(ITEM_EFFECT_SETTING, &args![item]).u32();
    e.call(SETTING_HAS_FLAG, &args![setting, mask]).bool()
}

/// The associated form of the setting of the effect item `item`.
fn item_associated_form(e: &mut Engine, item: u32) -> u32 {
    let setting = e.call(ITEM_EFFECT_SETTING, &args![item]).u32();
    e.call(SETTING_ASSOCIATED_FORM, &args![setting]).u32()
}

// Translated from 0040dfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two effect items; its caller is
/// `CheckAddEffect`): false when either is null. When both items'
/// settings have flag 0x20000 and both associated forms are
/// `TESBipedModelForm`s (`__RTDynamicCast` from `TESForm`), the answer is
/// `00480ad0` on the first with the second; otherwise whether both
/// settings have flag 0x10000.
pub fn fn_0040dfa0(e: &mut Engine, first: u32, second: u32) -> bool {
    if first == 0 || second == 0 {
        return false;
    }
    if item_setting_has_flag(e, first, 0x0002_0000) && item_setting_has_flag(e, second, 0x0002_0000)
    {
        let first_form = item_associated_form(e, first);
        let second_form = item_associated_form(e, second);
        let first_biped = e
            .call(
                DYNAMIC_CAST,
                &args![
                    first_form,
                    0u32,
                    TYPE_TES_FORM,
                    TYPE_TES_BIPED_MODEL_FORM,
                    0u32
                ],
            )
            .u32();
        let second_biped = e
            .call(
                DYNAMIC_CAST,
                &args![
                    second_form,
                    0u32,
                    TYPE_TES_FORM,
                    TYPE_TES_BIPED_MODEL_FORM,
                    0u32
                ],
            )
            .u32();
        if first_biped != 0 && second_biped != 0 {
            return e
                .call(BIPED_FORM_00480AD0, &args![first_biped, second_biped])
                .bool();
        }
    }
    item_setting_has_flag(e, first, 0x0001_0000) && item_setting_has_flag(e, second, 0x0001_0000)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_dee0, fn_0040dee0(u32) -> u32),
        entry!(0x0040_df50, fn_0040df50() -> f32),
        entry!(0x0040_df70, fn_0040df70() -> f32),
        entry!(0x0040_dfa0, fn_0040dfa0(u32, u32) -> bool),
        entry!(0x0040e0b0, fn_0040e0b0(Ptr, u32, u32) -> Ptr),
        entry!(0x0040e130, fn_0040e130(Ptr, u32, u32) -> Ptr),
        entry!(0x0040_df90, fn_0040df90() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_base_keeps_name_and_value_and_quiets_a_signaling_nan() {
        let mut e = Engine::new();
        let s = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0040e130, &[s, 0x1234, 0x4040_0000]).u32(), s);
        assert_eq!(e.mem.u32(s), SETTING_VTABLE);
        assert_eq!(e.mem.u32(s + 8), 0x1234);
        assert_eq!(e.mem.u32(s + 4), 0x4040_0000);
        e.call(0x0040e130, &[s, 0x1234, 0x7f80_0001]);
        assert_eq!(e.mem.u32(s + 4), 0x7fc0_0001);
        e.call(0x0040e130, &[s, 0x1234, 0xffc0_0000]);
        assert_eq!(e.mem.u32(s + 4), 0xffc0_0000);
    }

    #[test]
    fn a_game_setting_adds_itself_to_the_collection() {
        let mut e = Engine::new();
        let collection = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(collection, 0x0090_0000);
        e.register_double(GAME_SETTINGS, move |_, _| collection.into_ret());
        e.register_double(0x0090_1000, |e, a| {
            assert_eq!(e.mem.u32(a[1]), GAME_SETTING_VTABLE);
            assert_eq!(e.mem.u32(a[1] + 4), 0x3f80_0000);
            Ret::default()
        });
        let s = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040e0b0, &[s, 0x55, 0x3f80_0000]).u32(), s);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, GAME_SETTINGS);
        assert_eq!(log[2], (0x0090_1000, vec![collection, s]));
        assert_eq!(e.mem.u32(s + 8), 0x55);
    }

    #[test]
    fn zero_with_or_without_a_word() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0040_df90, &[]).u32(), 0);
        assert_eq!(e.call(0x0040_df90, &[7]).u32(), 0);
    }
}

#[cfg(test)]
mod n050_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn word_by_index() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        for i in 0..6u32 {
            e.set_global(0x011c_3538 + 4 * i, 0x100 + i);
        }
        e.set_global(0x011c_3550u32, 0x777u32);
        for i in 0..6u32 {
            assert_eq!(e.call(0x0040_dee0, &args![i]).u32(), 0x100 + i);
        }
        assert_eq!(fn_0040dee0(&mut e, 6), 0);
        assert_eq!(fn_0040dee0(&mut e, 0xffff_ffff), 0);
    }

    #[test]
    fn setting_values_as_floats() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(FLOAT_SETTING_011C36D8 + 4, 2.5f32);
        e.set_global(FLOAT_SETTING_011C3664 + 4, 0x7f80_0001u32);
        e.register(SETTING_FLOAT_ADDRESS_00403E20, |_, a| (a[0] + 4).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_df50, &args![]).f32(), 2.5);
        assert_eq!(
            calls(&e, SETTING_FLOAT_ADDRESS_00403E20),
            vec![vec![FLOAT_SETTING_011C36D8]]
        );
        // `FLD float` quiets a signalling NaN.
        assert_eq!(fn_0040df70(&mut e).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn effect_items_share_a_biped_form_or_both_flag_10000() {
        let mut e = Engine::new();
        // Item = setting = form: first word the flags, +0x60 the associated
        // form, +0x64 nonzero when it is a biped model form, +0x68 the
        // answer of `00480ad0`.
        e.register(ITEM_EFFECT_SETTING, |_, a| a[0].into_ret());
        e.register(SETTING_ASSOCIATED_FORM, |_, a| a[0].into_ret());
        e.register(SETTING_HAS_FLAG, |e, a| {
            u32::from(e.mem.u32(a[0]) & a[1] != 0).into_ret()
        });
        e.register(DYNAMIC_CAST, |e, a| {
            if e.mem.u32(a[0] + 0x64) != 0 {
                a[0].into_ret()
            } else {
                Ret::default()
            }
        });
        e.register(BIPED_FORM_00480AD0, |e, a| {
            e.mem.u32(a[0] + 0x68).into_ret()
        });
        let mut item = |flags: u32, biped: u32, answer: u32| {
            let item = e.mem.alloc(0x70);
            e.mem.set_u32(item, flags);
            e.mem.set_u32(item + 0x64, biped);
            e.mem.set_u32(item + 0x68, answer);
            item
        };
        let biped_yes = item(0x2_0000, 1, 1);
        let biped_no = item(0x2_0000, 1, 0);
        let not_biped = item(0x3_0000, 0, 1);
        let flag_10000 = item(0x1_0000, 0, 0);
        let plain = item(0, 0, 0);
        assert!(!e.call(0x0040_dfa0, &args![0u32, biped_yes]).bool());
        assert!(!fn_0040dfa0(&mut e, biped_yes, 0));
        e.call_log = Some(vec![]);
        assert!(fn_0040dfa0(&mut e, biped_yes, biped_no));
        assert_eq!(
            calls(&e, BIPED_FORM_00480AD0),
            vec![vec![biped_yes, biped_no]]
        );
        assert!(!fn_0040dfa0(&mut e, biped_no, biped_yes));
        // A cast fails: the 0x10000 test decides.
        assert!(!fn_0040dfa0(&mut e, biped_yes, not_biped));
        assert!(fn_0040dfa0(&mut e, not_biped, flag_10000));
        assert!(!fn_0040dfa0(&mut e, flag_10000, plain));
    }
}
