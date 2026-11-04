//! `fallout/interface/run_007001f0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiTPrimitiveArray` constructor `BGSMenuPacker`'s constructor runs
/// first (`00700480`, thiscall: two words).
const PACKED_MENUS_CONSTRUCTOR: u32 = 0x0070_0480;
/// Reads the word at `this` (`00559450`, thiscall).
const READ_WORD: u32 = 0x0055_9450;
/// Sets a tile's float trait (`00a012d0`, thiscall: trait, value, flag).
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;

// Translated from 007002d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSMenuPacker` constructor (no PDB name; the Xbox PDB's
/// `BGSMenuPacker` has `PackedMenus` at +0 and `bPackFailed` at +0x21):
/// constructs the packed-menu array (`00700480` with 0 and 1) and clears
/// `bPackFailed`. Returns `this`.
pub fn fn_007002d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PACKED_MENUS_CONSTRUCTOR, &args![this, 0u32, 1u32]);
    e.mem.set_u8(this.addr() + 0x21, 0);
    this
}

// Translated from 00700300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x20` (through `00559450`; for a `BGSMenuPacker`,
/// `dump_info_flag` and the bytes after it).
pub fn fn_00700300(e: &mut Engine, this: Ptr) -> u32 {
    e.call(READ_WORD, &args![this.byte_add(0x20)]).u32()
}

// Translated from 00700320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the tile `this`'s trait `trait_id` to the integer `value` as a
/// float (flag word 1).
pub fn fn_00700320(e: &mut Engine, this: Ptr, trait_id: i32, value: i32) {
    let value = x87_store32(f64::from(value));
    e.call(TILE_SET_FLOAT, &args![this, trait_id, value, 1u32]);
}

/// `007002a0` (unnamed; `thiscall`, one word; answers a byte in `AL`).
const TEST_007002A0: u32 = 0x0070_02a0;

// Translated from 00700270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the negation of `007002a0` on `this`
/// with `argument`: 1 in `EAX` when its byte answer is 0, else 0 (`MOVZX; NEG; SBB; ADD 1`).
pub fn fn_00700270(e: &mut Engine, this: Ptr, argument: u32) -> bool {
    e.call(TEST_007002A0, &args![this, argument]).u8() == 0
}

// Translated from 007001f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): of the two objects at +8 and +0xc, the word at +4 of the first
/// when both are there with equal words, or only the first is there; of the second when
/// only it is there; else 0.
pub fn fn_007001f0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let first = |e: &Engine| e.mem.u32(base + 8);
    let second = |e: &Engine| e.mem.u32(base + 0xc);
    if first(e) != 0 && second(e) != 0 && e.mem.u32(first(e) + 4) == e.mem.u32(second(e) + 4) {
        return e.mem.u32(first(e) + 4);
    }
    if first(e) != 0 && second(e) == 0 {
        return e.mem.u32(first(e) + 4);
    }
    if first(e) == 0 && second(e) != 0 {
        return e.mem.u32(second(e) + 4);
    }
    0
}

/// `BSStringT<char>::StrCmp` (`00408a80`, `thiscall`, `RET 8`: the other text and a
/// case flag).
const STRING_COMPARE: u32 = 0x0040_8a80;

// Translated from 007002a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): whether the string `this` equals `other` ignoring case
/// (`StrCmp(other, 1)` is 0).
pub fn fn_007002a0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    e.call(STRING_COMPARE, &args![this, other, 1u32]).u32() == 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070_02d0, fn_007002d0(Ptr) -> Ptr),
        entry!(0x0070_0300, fn_00700300(Ptr) -> u32),
        entry!(0x0070_0320, fn_00700320(Ptr, i32, i32)),
        entry!(0x0070_0270, fn_00700270(Ptr, u32) -> bool),
        entry!(0x0070_01f0, fn_007001f0(Ptr) -> u32),
        entry!(0x0070_02a0, fn_007002a0(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_007002d0_constructs_the_array_and_clears_the_failure() {
        let mut e = Engine::new();
        e.register(PACKED_MENUS_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x24);
        e.mem.set_u8(this + 0x21, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0070_02d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u8(this + 0x21), 0);
        assert_eq!(
            calls_to(&e, PACKED_MENUS_CONSTRUCTOR),
            vec![vec![this, 0, 1]]
        );
    }

    #[test]
    fn fn_00700300_reads_the_word_at_0x20() {
        let mut e = Engine::new();
        e.register(READ_WORD, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0070_0300, &args![0x5000u32]).u32(), 0x5021);
    }

    #[test]
    fn fn_00700320_sets_the_integer_as_a_float() {
        let mut e = Engine::new();
        e.register(TILE_SET_FLOAT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0070_0320, &args![0x5000u32, 0xfb1u32, -7i32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x5000, 0xfb1, (-7.0f32).to_bits(), 1]]
        );
    }

    #[test]
    fn fn_00700270_negates_the_low_byte() {
        let mut e = Engine::new();
        // The double answers the argument; only its low byte counts.
        e.register(TEST_007002A0, |_, a| a[1].into_ret());
        e.call_log = Some(vec![]);
        let got = e.call(0x0070_0270, &args![0x7000u32, 0x100u32]);
        assert_eq!(got.u32(), 1);
        assert_eq!(calls_to(&e, TEST_007002A0), vec![vec![0x7000, 0x100]]);
        assert_eq!(e.call(0x0070_0270, &args![0x7000u32, 0x101u32]).u32(), 0);
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn fn_007001f0_picks_the_shared_or_only_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first + 4, 7);
        e.mem.set_u32(second + 4, 9);
        let pick = |e: &mut Engine, a: u32, b: u32| {
            e.mem.set_u32(this + 8, a);
            e.mem.set_u32(this + 0xc, b);
            e.call(0x0070_01f0, &args![this]).u32()
        };
        assert_eq!(pick(&mut e, first, second), 0);
        assert_eq!(pick(&mut e, first, 0), 7);
        assert_eq!(pick(&mut e, 0, second), 9);
        assert_eq!(pick(&mut e, 0, 0), 0);
        e.mem.set_u32(second + 4, 7);
        assert_eq!(pick(&mut e, first, second), 7);
    }

    #[test]
    fn fn_007002a0_is_a_case_blind_equality() {
        let mut e = Engine::new();
        e.register(STRING_COMPARE, |_, a| (a[1] & 0xff).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0070_02a0, &args![0x5000u32, 0x100u32]).bool());
        assert!(!e.call(0x0070_02a0, &args![0x5000u32, 0x101u32]).bool());
        let calls: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == STRING_COMPARE)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(calls[0], vec![0x5000, 0x100, 1]);
    }
}
