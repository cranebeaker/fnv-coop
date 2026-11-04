//! `(unplaced)/run_00407600` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004077c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 14 (`0x4000`) of the flag word at `+8` of `this` is set (a
/// plain flag getter).
pub fn fn_004077c0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) & 0x4000 != 0
}

// Translated from 004077e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at `+0xA0` of `this` (`this + 0xA0`); the
/// engine map has no name and the class is unconfirmed.
pub fn fn_004077e0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xa0)
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@` and `.?AVEffectSetting@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_EFFECT_SETTING: u32 = 0x0118_37c4;
/// `TESForm::CopyAllComponents` (Xbox PDB, `004851b0`, `ECX`, one stack word: the source).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
/// `0084e3a0` (`ECX`): a word of the form, tested for zero.
const RECORD_ID_0084E3A0: u32 = 0x0084_e3a0;
/// `TESDataHandler::GetNextID` (Xbox PDB, `00469800`, `ECX` = the data handler, no stack
/// word).
const GET_NEXT_ID: u32 = 0x0046_9800;
/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;

/// The word getter/setter pairs [`fn_00407600`] copies with, in its order: the getter
/// runs on the source (`ECX`, the word in `EAX`), the setter on `this` with that word.
const COPIED_WORDS: [(u32, u32); 14] = [
    (0x0062_86d0, 0x005a_8060),
    (0x0058_6150, 0x0081_0570),
    (0x0070_ec90, 0x008d_8500),
    (0x008d_8520, 0x0040_7800),
    (0x0081_0fd0, 0x0040_78a0),
    (0x005f_5f80, 0x009e_3360),
    (0x004f_d380, 0x0057_bd60),
    (0x0040_7820, 0x0044_2a80),
    (0x0040_7840, 0x004f_ebd0),
    (0x004f_b070, 0x008a_c890),
    (0x004f_d3c0, 0x004f_b090),
    (0x004f_d3e0, 0x0085_7230),
    (0x008c_e390, 0x008c_c6d0),
    (0x0040_77e0, 0x0040_8600),
];

/// Copies one word: `getter` on the source, `setter` on `this` with its `EAX`.
fn copy_word(e: &mut Engine, this: Ptr, source: u32, (getter, setter): (u32, u32)) {
    let value = if getter == 0x0040_77e0 {
        // Same unit: a direct call.
        fn_004077e0(e, Ptr::new(source)).addr()
    } else {
        e.call(getter, &args![source]).u32()
    };
    e.call(setter, &args![this, value]);
}

/// Copies one float: `getter` on the source (`ST0`), stored to a `float` stack word
/// (`FSTP float ptr [ESP]`) and handed to `setter` on `this`.
fn copy_float(e: &mut Engine, this: Ptr, source: u32, getter: u32, setter: u32) {
    let value = e.call(getter, &args![source]).f64();
    e.call(setter, &args![this, x87_store32(value)]);
}

// Translated from 00407600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSetting`'s `Copy` slot (+0x108 of its vtable `01012834`; unnamed in the engine
/// map, whose names for the getters and setters it calls come from folded code; `ECX` and
/// one stack word, `RET 4`). When `source` casts (`__RTDynamicCast` from `TESForm`) to an
/// `EffectSetting`: `TESForm::CopyAllComponents` from `source` (the uncast pointer), then
/// each field through a getter on the cast source and a setter on `this`, in the exe's
/// order: `006286d0`→`005a8060`, the float `00639b20`→`004ee4b0`, `00586150`→`00810570`,
/// `0070ec90`→`008d8500`, `008d8520`→`00407800`, the floats `006ca4e0`→`00407860`,
/// `00644930`→`00407880` and `009b0bd0`→`006d3b00`, then `00810fd0`→`004078a0`,
/// `005f5f80`→`009e3360`, `004fd380`→`0057bd60`, `00407820`→`00442a80`,
/// `00407840`→`004febd0`, `004fb070`→`008ac890`, `004fd3c0`→`004fb090`,
/// `004fd3e0`→`00857230`, `008ce390`→`008cc6d0` and [`fn_004077e0`]→`00408600`. Last,
/// when flag 0x4000 of `this` ([`fn_004077c0`]) is clear and `0084e3a0` answers 0 for
/// `this`, gives `this` a new id: `TESDataHandler::GetNextID` on the data handler, handed
/// to `this`'s vtable slot +0x128 with 1.
pub fn fn_00407600(e: &mut Engine, this: Ptr, source: Ptr) {
    let setting = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, TYPE_TES_FORM, TYPE_EFFECT_SETTING, 0u32],
        )
        .u32();
    if setting == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
    copy_word(e, this, setting, COPIED_WORDS[0]);
    copy_float(e, this, setting, 0x0063_9b20, 0x004e_e4b0);
    for pair in &COPIED_WORDS[1..4] {
        copy_word(e, this, setting, *pair);
    }
    copy_float(e, this, setting, 0x006c_a4e0, 0x0040_7860);
    copy_float(e, this, setting, 0x0064_4930, 0x0040_7880);
    copy_float(e, this, setting, 0x009b_0bd0, 0x006d_3b00);
    for pair in &COPIED_WORDS[4..] {
        copy_word(e, this, setting, *pair);
    }
    if fn_004077c0(e, this) {
        return;
    }
    if e.call(RECORD_ID_0084E3A0, &args![this]).u32() != 0 {
        return;
    }
    let handler: u32 = e.global(DATA_HANDLER);
    // 0040779e: `00469800` takes no stack word (plain RET); the pushed 1 is the
    // second word of the form's +0x128 (`00485c10`, RET 8).
    let id = e.call(GET_NEXT_ID, &args![handler]).u32();
    e.vcall(this.addr(), 0x128, &args![id, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_7600, fn_00407600(Ptr, Ptr)),
        entry!(0x004077c0, fn_004077c0(Ptr) -> bool),
        entry!(0x0040_77e0, fn_004077e0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_bit_0x4000_of_the_word_at_8() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x10));
        // Every other bit set: still false.
        e.mem.set_u32(object.addr() + 8, !0x4000);
        assert!(!e.call(0x0040_77c0, &args![object]).bool());
        e.mem.set_u32(object.addr() + 8, 0x4000);
        assert!(e.call(0x0040_77c0, &args![object]).bool());
    }
}

#[cfg(test)]
mod member_tests {
    use super::*;

    #[test]
    fn fn_004077e0_adds_0xa0_to_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0040_77e0, &args![0x1000u32]).u32(), 0x10a0);
        assert_eq!(e.call(0x0040_77e0, &args![0xffff_ff80u32]).u32(), 0x20);
    }
}

#[cfg(test)]
mod copy_tests {
    use super::*;

    const FLOAT_GETTERS: [u32; 4] = [0x0063_9b20, 0x006c_a4e0, 0x0064_4930, 0x009b_0bd0];
    const FLOAT_SETTERS: [u32; 4] = [0x004e_e4b0, 0x0040_7860, 0x0040_7880, 0x006d_3b00];

    fn engine(cast: bool, word: u32) -> (Engine, Ptr, u32) {
        let mut e = Engine::new();
        e.register(
            DYNAMIC_CAST,
            if cast {
                |_, a| (a[0] + 0x100).into_ret()
            } else {
                |_, _| 0u32.into_ret()
            },
        );
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        for (getter, setter) in COPIED_WORDS {
            if getter != 0x0040_77e0 {
                e.register(getter, |_, a| (a[0] ^ 0x5000).into_ret());
            }
            e.register(setter, |_, _| Ret::default());
        }
        for getter in FLOAT_GETTERS {
            e.register(getter, |_, _| Ret {
                st0: 1.25,
                ..Ret::default()
            });
        }
        for setter in FLOAT_SETTERS {
            e.register(setter, |_, _| Ret::default());
        }
        e.register_double(RECORD_ID_0084E3A0, move |_, _| word.into_ret());
        e.register(GET_NEXT_ID, |_, _| 0x0012_3456u32.into_ret());
        e.register(0x0060_0128, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x4b];
        slots[0x128 / 4] = 0x0060_0128;
        e.put_vtable(0x0070_0000, &slots);
        e.map(DATA_HANDLER, 4);
        e.set_global(DATA_HANDLER, 0x0090_0000u32);
        let this = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(this.addr(), 0x0070_0000);
        let source = e.mem.alloc(0x200);
        e.call_log = Some(vec![]);
        (e, this, source)
    }

    #[test]
    fn fn_00407600_copies_every_field_and_assigns_an_id() {
        let (mut e, this, source) = engine(true, 0);
        e.call(0x0040_7600, &args![this, source]);
        let log = e.call_log.take().unwrap();
        let setting = source + 0x100;
        let setters: Vec<_> = log
            .iter()
            .filter(|(a, _)| COPIED_WORDS.iter().any(|(_, s)| s == a) || FLOAT_SETTERS.contains(a))
            .map(|(a, w)| (*a, w.clone()))
            .collect();
        let mut expected = vec![(0x005a_8060, vec![this.addr(), setting ^ 0x5000])];
        expected.push((0x004e_e4b0, vec![this.addr(), 1.25f32.to_bits()]));
        for (_, s) in &COPIED_WORDS[1..4] {
            expected.push((*s, vec![this.addr(), setting ^ 0x5000]));
        }
        for s in &FLOAT_SETTERS[1..] {
            expected.push((*s, vec![this.addr(), 1.25f32.to_bits()]));
        }
        for (g, s) in &COPIED_WORDS[4..] {
            let value = if *g == 0x0040_77e0 {
                setting + 0xa0
            } else {
                setting ^ 0x5000
            };
            expected.push((*s, vec![this.addr(), value]));
        }
        assert_eq!(setters, expected);
        assert!(log.contains(&(COPY_ALL_COMPONENTS, vec![this.addr(), source])));
        assert!(log.contains(&(GET_NEXT_ID, vec![0x0090_0000])));
        assert!(log.contains(&(0x0060_0128, vec![this.addr(), 0x0012_3456, 1])));
    }

    #[test]
    fn fn_00407600_keeps_the_id_when_flagged_or_set() {
        let (mut e, this, source) = engine(true, 0);
        e.mem.set_u32(this.addr() + 8, 0x4000);
        e.call(0x0040_7600, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert!(!log
            .iter()
            .any(|(a, _)| *a == RECORD_ID_0084E3A0 || *a == GET_NEXT_ID));
        let (mut e, this, source) = engine(true, 7);
        e.call(0x0040_7600, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(a, _)| *a == RECORD_ID_0084E3A0));
        assert!(!log.iter().any(|(a, _)| *a == GET_NEXT_ID));
    }

    #[test]
    fn fn_00407600_ignores_other_forms() {
        let (mut e, this, source) = engine(false, 0);
        e.call(0x0040_7600, &args![this, source]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
