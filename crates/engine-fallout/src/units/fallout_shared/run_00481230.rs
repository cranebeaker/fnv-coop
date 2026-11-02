//! `fallout shared/run_00481230` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_004812d0` calls on the member (`0048cee0`).
const CALLEE_0048CEE0: u32 = 0x0048_cee0;

// Translated from 004812d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0048cee0` with `ECX` = `this + 0x4` and passes on its result.
pub fn fn_004812d0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0048CEE0, &args![this.byte_add(0x4)]).u32()
}

/// `00ec7750` (`cdecl`, two words; unnamed in the engine map): the CRT
/// routine that searches the first string for the second (its body is
/// `strstr`'s).
const FIND_SUBSTRING_00EC7750: u32 = 0x00ec_7750;

// Translated from 004812f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` wrapper (no Xbox PDB name): `00ec7750(text, pattern)` and its
/// result.
pub fn fn_004812f0(e: &mut Engine, text: u32, pattern: u32) -> u32 {
    e.call(FIND_SUBSTRING_00EC7750, &args![text, pattern]).u32()
}

/// `00408da0` (`thiscall`, no stack words; the map's `MapMarkerData::GetLocationName`, a
/// shared body): the text of the string pointer at `this`, `""` (`01011584`) when null.
const STRING_TEXT_OR_EMPTY: u32 = 0x0040_8da0;

// Translated from 00481230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: the sex; `RET 4`): the 0xC-byte
/// component `this + 0x8C + sex * 0xC` (by `TESBipedModelForm`'s PDB layout its
/// `inventoryIcon[sex]`; not confirmed for this caller), sex 1 falling back to 0 when that
/// component has no file name (`0048cee0`); answers `00408da0` on the chosen component
/// (its file name's text, or `""`).
pub fn fn_00481230(e: &mut Engine, this: Ptr, sex: u32) -> u32 {
    let mut sex = sex;
    let icon = this
        .addr()
        .wrapping_add(sex.wrapping_mul(0xc))
        .wrapping_add(0x8c);
    if e.call(CALLEE_0048CEE0, &args![icon]).u32() == 0 && sex == 1 {
        sex = 0;
    }
    let icon = this
        .addr()
        .wrapping_add(sex.wrapping_mul(0xc))
        .wrapping_add(0x8c);
    e.call(STRING_TEXT_OR_EMPTY, &args![icon]).u32()
}

/// `0047a300` (`thiscall`, no stack words; unnamed in the engine map), on
/// the 16-byte member `fn_00481280` picks.
const CALLEE_0047A300: u32 = 0x0047_a300;

// Translated from 00481280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: an index; `RET 4`):
/// the 16-byte member `this + 0xA4 + index * 16`, index 1 falling back to 0
/// when [`fn_004812d0`] answers 0 for it; answers `0047a300` on the chosen
/// member (the function ends in that call, so its `EAX` is the answer).
pub fn fn_00481280(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let mut index = index;
    let member = this
        .addr()
        .wrapping_add(index.wrapping_shl(4))
        .wrapping_add(0xa4);
    if fn_004812d0(e, Ptr::new(member)) == 0 && index == 1 {
        index = 0;
    }
    let member = this
        .addr()
        .wrapping_add(index.wrapping_shl(4))
        .wrapping_add(0xa4);
    e.call(CALLEE_0047A300, &args![member]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_1280, fn_00481280(Ptr, u32) -> u32),
        entry!(0x0048_1230, fn_00481230(Ptr, u32) -> u32),
        entry!(0x0048_12d0, fn_004812d0(Ptr) -> u32),
        entry!(0x0048_12f0, fn_004812f0(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_004812d0_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0048CEE0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_12d0, &args![0x4100u32]).u32(), 0x1451);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0048CEE0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }
}

#[cfg(test)]
mod wrapper_tests {
    use super::*;

    #[test]
    fn fn_004812f0_passes_both_words_on() {
        let mut e = Engine::new();
        e.register(FIND_SUBSTRING_00EC7750, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_12f0, &args![0x100u32, 0x20u32]).u32(), 0x120);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (FIND_SUBSTRING_00EC7750, vec![0x100, 0x20])
        );
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_00481230_answers_the_icon_text_with_the_fallback() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.register(CALLEE_0048CEE0, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(STRING_TEXT_OR_EMPTY, |_, a| (a[0] | 0x8000_0000).into_ret());
        let text = |e: &mut Engine, sex: u32| e.call(0x0048_1230, &args![this, sex]).u32();
        assert_eq!(text(&mut e, 0), (this + 0x8c) | 0x8000_0000);
        assert_eq!(text(&mut e, 1), (this + 0x8c) | 0x8000_0000);
        e.mem.set_u32(this + 0x98 + 4, 1);
        assert_eq!(text(&mut e, 1), (this + 0x98) | 0x8000_0000);
    }
}

#[cfg(test)]
mod n085_tests {
    use super::*;

    #[test]
    fn member_choice_falls_back_to_zero_and_passes_the_answer_on() {
        let mut e = Engine::new();
        // `0048cee0` (through fn_004812d0 on member + 4): the word there.
        e.register(CALLEE_0048CEE0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CALLEE_0047A300, |_, a| (a[0] ^ 0x8000_0000).into_ret());
        let this = e.mem.alloc(0xc0);
        let answer = |e: &mut Engine, index: u32| e.call(0x0048_1280, &args![this, index]).u32();
        // Member 1 without a name: member 0.
        assert_eq!(answer(&mut e, 1), (this + 0xa4) ^ 0x8000_0000);
        e.mem.set_u32(this + 0xb4 + 4, 5);
        assert_eq!(answer(&mut e, 1), (this + 0xb4) ^ 0x8000_0000);
        // Index 0 keeps index 0 whatever the name.
        assert_eq!(answer(&mut e, 0), (this + 0xa4) ^ 0x8000_0000);
    }
}
