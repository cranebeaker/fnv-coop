//! `(unplaced)/run_005f9820` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A creature form's copy from another form (`__RTDynamicCast` from
//! `TESForm` to `TESCreature`, the RTTI type descriptors `01183028` and
//! `01183a00`; vtable entry at `01049064`) and four field accessors it
//! uses: the byte at +0x130, the float at +0x138 and the word at +0x14C.

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast` (`00ec43fb`, `cdecl` (object, vfptr delta, source
/// type, target type, is reference)).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptor of `TESForm` (`01183028`).
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// The RTTI type descriptor of `TESCreature` (`01183a00`).
const TES_CREATURE_TYPE: u32 = 0x0118_3a00;
/// `005f91d0` (`thiscall`): the word at +0x128 when `00461580` on +0x30
/// with 0x100 says so, else 0.
const WORD_AT_128_IF_FLAGGED: u32 = 0x005f_91d0;
/// `005f7bf0` (`thiscall`, `RET 4`), called with 1 on the old word at
/// +0x128.
const RELEASE_WORD_AT_128: u32 = 0x005f_7bf0;
/// The component copies, in call order: the member's offset in the
/// creature and its `CopyComponent`-style function (`thiscall`, `RET 4`,
/// the source form). Xbox PDB names where the map has one:
/// `TESAIForm` (+0x90), `TESSpellList` (+0x7C), `0047bdd0` (+0x70),
/// `TESActorBaseData` (+0x30), `TESHealthForm` (+0xB0), `TESAttributes`
/// (+0xB8), `004870f0` (+0xD0), `TESModel` (+0xDC), `TESModelList`
/// (+0x114), `0048cce0` (+0xF4), `TESContainer` (+0x64), `0047fb50`
/// (+0xC4), `004786e0` (+0x104).
const COMPONENT_COPIES: [(u32, u32); 13] = [
    (0x90, 0x0047_f190),
    (0x7c, 0x0048_d630),
    (0x70, 0x0047_bdd0),
    (0x30, 0x0047_d220),
    (0xb0, 0x0048_7240),
    (0xb8, 0x0048_0000),
    (0xd0, 0x0048_70f0),
    (0xdc, 0x0048_9430),
    (0x114, 0x0048_9db0),
    (0xf4, 0x0048_cce0),
    (0x64, 0x0048_1c80),
    (0xc4, 0x0047_fb50),
    (0x104, 0x0047_86e0),
];
/// `005f9b70` (`thiscall`): the address of the member at +0x12C.
const MEMBER_AT_12C: u32 = 0x005f_9b70;
/// `00401460` (`cdecl` (dest, source, size), a `memcpy` wrapper).
const COPY_BYTES: u32 = 0x0040_1460;
/// `00717e50` (`BaseProcess::GetActorPackageThatIsRunning` in the map, a
/// folded body; `thiscall`): an address, called on the source's +0x30.
const POINTER_FROM_30: u32 = 0x0071_7e50;
/// Float getter (`00821640`, the float at +0x134) and its setter (`00507780`).
const GET_FLOAT_134: u32 = 0x0082_1640;
const SET_FLOAT_134: u32 = 0x0050_7780;
/// Float getter (`00567470`, the float at +0x13C) and its setter (`00803a90`).
const GET_FLOAT_13C: u32 = 0x0056_7470;
const SET_FLOAT_13C: u32 = 0x0080_3a90;
/// Float getter of +0x138 (`00821660`).
const GET_FLOAT_138: u32 = 0x0082_1660;
/// `005f9210` (`thiscall`): an object for `005f0640`.
const OBJECT_FOR_SET: u32 = 0x005f_9210;
/// `005f0640` (`thiscall`, `RET 4`).
const SET_ON_OBJECT: u32 = 0x005f_0640;
/// `005f9100` (`thiscall`): a word for `005f9140`.
const OTHER_WORD: u32 = 0x005f_9100;
/// `005f9140` (`thiscall`, `RET 4`).
const SET_OTHER_WORD: u32 = 0x005f_9140;
/// `009c0070` (`thiscall`): the word stored at +0x154.
const WORD_FOR_154: u32 = 0x009c_0070;
/// `005f8bb0` (`thiscall`, `RET 4`): takes [`fn_005f9b30`]'s word.
const SET_WORD_14C: u32 = 0x005f_8bb0;
/// `MiddleHighProcess::GetCommandingActor` / `SetCommandingActor` by the
/// map (`005f9b50` / `005f8bd0`; folded bodies, used as a getter/setter
/// pair here).
const GET_PAIRED: u32 = 0x005f_9b50;
const SET_PAIRED: u32 = 0x005f_8bd0;

// Translated from 005f9820 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Copies a creature from `source` (`thiscall`, `RET 4`). Nothing happens
/// unless `source` casts to `TESCreature`. When `005f91d0` gives this
/// creature a word at +0x128, `005f7bf0(1)` runs on it; +0x128 = 0. Then
/// the thirteen component copies with `source`, the bytes of +0x12C (4),
/// the word at +0x34 (read through `00717e50` on the source's +0x30), the
/// byte at +0x130 and the floats at +0x134, +0x13C and +0x138 (getters on
/// the source, setters on `this`, floats passed as their bits), slot 0x58
/// of the source's +0x30 into slot 0x5C of this +0x30; with a source word
/// from `005f91d0`, `005f0640` on what `005f9210` returns, else
/// `005f9140(005f9100(source))`; +0x154 from `009c0070`; the word at
/// +0x14C and the `005f9b50` / `005f8bd0` pair; slot 0x188 of the source
/// into slot 0x18C, slot 0x180 into slot 0x184.
pub fn fn_005f9820(e: &mut Engine, this: Ptr, source: u32) {
    let t = this.addr();
    let creature = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, TES_FORM_TYPE, TES_CREATURE_TYPE, 0u32],
        )
        .u32();
    if creature == 0 {
        return;
    }
    if e.call(WORD_AT_128_IF_FLAGGED, &args![this]).u32() != 0 {
        let word = e.mem.u32(t + 0x128);
        if word != 0 {
            e.call(RELEASE_WORD_AT_128, &args![word, 1u32]);
        }
    }
    e.mem.set_u32(t + 0x128, 0);
    for (offset, copy) in COMPONENT_COPIES {
        e.call(copy, &args![t + offset, source]);
    }
    let from = e.call(MEMBER_AT_12C, &args![creature]).u32();
    e.call(COPY_BYTES, &args![t + 0x12c, from, 4u32]);
    let pointer = e.call(POINTER_FROM_30, &args![creature + 0x30]).u32();
    let word = e.mem.u32(pointer);
    e.mem.set_u32(t + 0x34, word);
    let byte = fn_005f9ad0(e, Ptr::new(creature));
    fn_005f9af0(e, this, byte);
    let value = e.call(GET_FLOAT_134, &args![creature]).f32();
    e.call(SET_FLOAT_134, &args![this, value.to_bits()]);
    let value = e.call(GET_FLOAT_13C, &args![creature]).f32();
    e.call(SET_FLOAT_13C, &args![this, value.to_bits()]);
    let value = e.call(GET_FLOAT_138, &args![creature]).f32();
    fn_005f9b10(e, this, value);
    let value = e.vcall(creature + 0x30, 0x58, &[]).u32();
    e.vcall(t + 0x30, 0x5c, &args![value]);
    if e.call(WORD_AT_128_IF_FLAGGED, &args![creature]).u32() != 0 {
        let object = e.call(OBJECT_FOR_SET, &args![this]).u32();
        let word = e.call(WORD_AT_128_IF_FLAGGED, &args![creature]).u32();
        e.call(SET_ON_OBJECT, &args![object, word]);
    } else {
        let word = e.call(OTHER_WORD, &args![creature]).u32();
        e.call(SET_OTHER_WORD, &args![this, word]);
    }
    let word = e.call(WORD_FOR_154, &args![creature]).u32();
    e.mem.set_u32(t + 0x154, word);
    let word = fn_005f9b30(e, Ptr::new(creature));
    e.call(SET_WORD_14C, &args![this, word]);
    let word = e.call(GET_PAIRED, &args![creature]).u32();
    e.call(SET_PAIRED, &args![this, word]);
    let value = e.vcall(creature, 0x188, &[]).u32();
    e.vcall(t, 0x18c, &args![value]);
    let value = e.vcall(creature, 0x180, &[]).u32();
    e.vcall(t, 0x184, &args![value]);
}

// Translated from 005f9ad0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The byte at +0x130 (`AL`).
pub fn fn_005f9ad0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x130)
}

// Translated from 005f9af0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at +0x130 (`thiscall`, `RET 4`).
pub fn fn_005f9af0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x130, value);
}

// Translated from 005f9b10 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores `value` as the float at +0x138 (`thiscall`, `RET 4`;
/// `FLD`/`FSTP`, so a signalling NaN is stored quiet).
pub fn fn_005f9b10(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x138, x87_quiet_bits(value.to_bits()));
}

// Translated from 005f9b30 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The word at +0x14C.
pub fn fn_005f9b30(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x14c)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f9820, fn_005f9820(Ptr, u32)),
        entry!(0x005f9ad0, fn_005f9ad0(Ptr) -> u8),
        entry!(0x005f9af0, fn_005f9af0(Ptr, u8)),
        entry!(0x005f9b10, fn_005f9b10(Ptr, f32)),
        entry!(0x005f9b30, fn_005f9b30(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const VTABLE: u32 = 0x0300_0000;
    const INNER_VTABLE: u32 = 0x0300_1000;
    const SLOT_188: u32 = 0x0300_2188;
    const SLOT_18C: u32 = 0x0300_218c;
    const SLOT_180: u32 = 0x0300_2180;
    const SLOT_184: u32 = 0x0300_2184;
    const SLOT_58: u32 = 0x0300_2058;
    const SLOT_5C: u32 = 0x0300_205c;

    fn creature(e: &mut Engine) -> u32 {
        let c = e.mem.alloc(0x160);
        e.mem.set_u32(c, VTABLE);
        e.mem.set_u32(c + 0x30, INNER_VTABLE);
        c
    }

    /// Doubles for every callee; `flagged` is the word `005f91d0` gives
    /// (the same for both creatures).
    fn doubles(e: &mut Engine, cast_ok: bool, flagged: u32) {
        let mut table = vec![0u32; 0x190 / 4];
        table[0x188 / 4] = SLOT_188;
        table[0x18c / 4] = SLOT_18C;
        table[0x180 / 4] = SLOT_180;
        table[0x184 / 4] = SLOT_184;
        e.put_vtable(VTABLE, &table);
        let mut inner = vec![0u32; 0x60 / 4];
        inner[0x58 / 4] = SLOT_58;
        inner[0x5c / 4] = SLOT_5C;
        e.put_vtable(INNER_VTABLE, &inner);
        if cast_ok {
            e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        } else {
            e.register(DYNAMIC_CAST, |_, _| Ret::default());
        }
        e.register_double(WORD_AT_128_IF_FLAGGED, move |_, _| flagged.into_ret());
        e.register(MEMBER_AT_12C, |_, a| (a[0] + 0x12c).into_ret());
        e.register(POINTER_FROM_30, |_, a| (a[0] + 0x10).into_ret());
        e.register(GET_FLOAT_134, |_, _| 1.5f32.into_ret());
        e.register(GET_FLOAT_13C, |_, _| (-3.0f32).into_ret());
        e.register(GET_FLOAT_138, |_, _| 0.25f32.into_ret());
        e.register(SLOT_58, |_, _| 0x58u32.into_ret());
        e.register(SLOT_188, |_, _| 0x188u32.into_ret());
        e.register(SLOT_180, |_, _| 0x180u32.into_ret());
        e.register(OBJECT_FOR_SET, |_, _| 0x0b1u32.into_ret());
        e.register(OTHER_WORD, |_, _| 0x0b2u32.into_ret());
        e.register(WORD_FOR_154, |_, _| 0x154u32.into_ret());
        e.register(GET_PAIRED, |_, _| 0x0b3u32.into_ret());
        for a in [
            RELEASE_WORD_AT_128,
            COPY_BYTES,
            SET_FLOAT_134,
            SET_FLOAT_13C,
            SLOT_5C,
            SLOT_18C,
            SLOT_184,
            SET_ON_OBJECT,
            SET_OTHER_WORD,
            SET_WORD_14C,
            SET_PAIRED,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        for (_, copy) in COMPONENT_COPIES {
            e.register(copy, |_, _| Ret::default());
        }
    }

    #[test]
    fn nothing_happens_for_a_form_that_is_not_a_creature() {
        let mut e = Engine::new();
        doubles(&mut e, false, 0);
        let this = creature(&mut e);
        e.mem.set_u32(this + 0x128, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x005f_9820, &[this, 0x1234]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, DYNAMIC_CAST),
            vec![vec![0x1234, 0, TES_FORM_TYPE, TES_CREATURE_TYPE, 0]]
        );
        assert_eq!(log.len(), 2);
        assert_eq!(e.mem.u32(this + 0x128), 0x77);
    }

    #[test]
    fn copies_every_part_without_the_flagged_word() {
        let mut e = Engine::new();
        doubles(&mut e, true, 0);
        let this = creature(&mut e);
        let source = creature(&mut e);
        e.mem.set_u32(this + 0x128, 0x77);
        e.mem.set_u8(source + 0x130, 0x5a);
        e.mem.set_u32(source + 0x14c, 0x14c);
        e.mem.set_u32(source + 0x40, 0xdead);
        e.call_log = Some(vec![]);
        e.call(0x005f_9820, &[this, source]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, RELEASE_WORD_AT_128).is_empty());
        assert_eq!(e.mem.u32(this + 0x128), 0);
        let copies: Vec<_> = log
            .iter()
            .filter(|c| COMPONENT_COPIES.iter().any(|(_, f)| *f == c.0))
            .cloned()
            .collect();
        let expected: Vec<_> = COMPONENT_COPIES
            .iter()
            .map(|(off, f)| (*f, vec![this + off, source]))
            .collect();
        assert_eq!(copies, expected);
        assert_eq!(
            calls_to(&log, COPY_BYTES),
            vec![vec![this + 0x12c, source + 0x12c, 4]]
        );
        assert_eq!(calls_to(&log, POINTER_FROM_30), vec![vec![source + 0x30]]);
        assert_eq!(e.mem.u32(this + 0x34), 0xdead);
        assert_eq!(e.mem.u8(this + 0x130), 0x5a);
        assert_eq!(
            calls_to(&log, SET_FLOAT_134),
            vec![vec![this, 1.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, SET_FLOAT_13C),
            vec![vec![this, (-3.0f32).to_bits()]]
        );
        assert_eq!(e.mem.f32(this + 0x138), 0.25);
        assert_eq!(calls_to(&log, SLOT_58), vec![vec![source + 0x30]]);
        assert_eq!(calls_to(&log, SLOT_5C), vec![vec![this + 0x30, 0x58]]);
        assert_eq!(calls_to(&log, OTHER_WORD), vec![vec![source]]);
        assert_eq!(calls_to(&log, SET_OTHER_WORD), vec![vec![this, 0x0b2]]);
        assert!(calls_to(&log, SET_ON_OBJECT).is_empty());
        assert_eq!(e.mem.u32(this + 0x154), 0x154);
        assert_eq!(calls_to(&log, SET_WORD_14C), vec![vec![this, 0x14c]]);
        assert_eq!(calls_to(&log, GET_PAIRED), vec![vec![source]]);
        assert_eq!(calls_to(&log, SET_PAIRED), vec![vec![this, 0x0b3]]);
        assert_eq!(calls_to(&log, SLOT_18C), vec![vec![this, 0x188]]);
        assert_eq!(calls_to(&log, SLOT_184), vec![vec![this, 0x180]]);
        let last = log.last().unwrap();
        assert_eq!(last.0, SLOT_184);
    }

    #[test]
    fn a_flagged_word_is_released_and_passed_on() {
        let mut e = Engine::new();
        doubles(&mut e, true, 0x99);
        let this = creature(&mut e);
        let source = creature(&mut e);
        e.mem.set_u32(this + 0x128, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x005f_9820, &[this, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RELEASE_WORD_AT_128), vec![vec![0x77, 1]]);
        assert_eq!(e.mem.u32(this + 0x128), 0);
        assert_eq!(calls_to(&log, OBJECT_FOR_SET), vec![vec![this]]);
        assert_eq!(calls_to(&log, SET_ON_OBJECT), vec![vec![0x0b1, 0x99]]);
        assert!(calls_to(&log, SET_OTHER_WORD).is_empty());

        // A flagged creature whose word at +0x128 is already 0 releases
        // nothing.
        e.mem.set_u32(this + 0x128, 0);
        e.call_log = Some(vec![]);
        e.call(0x005f_9820, &[this, source]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, RELEASE_WORD_AT_128).is_empty());
    }

    #[test]
    fn the_byte_accessors_use_130() {
        let mut e = Engine::new();
        let o = e.mem.alloc(0x160);
        e.mem.set_u32(o + 0x12c, 0xffff_ffff);
        e.call(0x005f_9af0, &[o, 0x1ab]);
        assert_eq!(e.mem.u32(o + 0x12c), 0xffff_ffff);
        assert_eq!(e.mem.u8(o + 0x130), 0xab);
        assert_eq!(e.mem.u8(o + 0x131), 0);
        assert_eq!(e.call(0x005f_9ad0, &[o]).u8(), 0xab);
    }

    #[test]
    fn the_float_setter_stores_quiet_floats() {
        let mut e = Engine::new();
        let o = e.mem.alloc(0x160);
        e.call(0x005f_9b10, &args![o, 4.5f32]);
        assert_eq!(e.mem.f32(o + 0x138), 4.5);
        e.call(0x005f_9b10, &[o, 0x7fa0_0000]);
        assert_eq!(e.mem.u32(o + 0x138), 0x7fe0_0000);
    }

    #[test]
    fn the_word_getter_reads_14c() {
        let mut e = Engine::new();
        let o = e.mem.alloc(0x160);
        e.mem.set_u32(o + 0x14c, 0xcafe);
        assert_eq!(e.call(0x005f_9b30, &[o]).u32(), 0xcafe);
    }
}
