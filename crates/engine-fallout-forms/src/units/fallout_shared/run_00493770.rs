//! `fallout shared/run_00493770` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The two `thiscall` getters `00493770` subtracts (each answers in ST0;
/// the engine map files them under `sky/precipitation.cpp` and
/// `computersmenu.cpp`), the word getter `00559450` (`thiscall`, answers
/// the word at `this`) and `TESAnimGroup::IsAimAction` (`005f2630`,
/// `cdecl (group)`).
const FIRST_VALUE: u32 = 0x0063_9aa0;
const SECOND_VALUE: u32 = 0x0075_9450;
const FIRST_WORD: u32 = 0x0055_9450;
const IS_AIM_ACTION: u32 = 0x005f_2630;

// Translated from 00493770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The difference of two of the object's values (the engine map has no
/// name): `00639aa0`'s answer (kept as a `double`) minus `00759450`'s
/// (x87 `FSUBR`), stored as a `float`.
pub fn fn_00493770(e: &mut Engine, this: Ptr) -> f32 {
    let first = e.call(FIRST_VALUE, &args![this]).f64();
    let second = e.call(SECOND_VALUE, &args![this]).f64();
    f32::from_bits(x87_store32(x87_sub(first, second)))
}

// Translated from 004937a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first word of element `index` of the 16-byte array at +0x14 (the
/// engine map has no name), through `00559450`.
pub fn fn_004937a0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let element = (index << 4).wrapping_add(e.mem.u32(this.addr() + 0x14));
    e.call(FIRST_WORD, &args![element]).u32()
}

// Translated from 004937c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimGroup::IsAimAction` of the 16-bit group at +0x10 (the engine
/// map has no name); its `EAX` is answered as it is.
pub fn fn_004937c0(e: &mut Engine, this: Ptr) -> u32 {
    let group = u32::from(e.mem.u16(this.addr() + 0x10));
    e.call(IS_AIM_ACTION, &args![group]).u32()
}

/// `TESAnimGroup::IsAttackAction` (engine map, `005f2540`, `cdecl
/// (group)`).
const IS_ATTACK_ACTION: u32 = 0x005f_2540;

// Translated from 004937e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimGroup::IsAttackAction` of the 16-bit group at +0x10 (the engine
/// map has no name; no stack word); it answers its callee's `AL` (the callee is a `bool` test: the
/// upper bytes of `EAX` are whatever that test left, and callers read `AL`).
pub fn fn_004937e0(e: &mut Engine, this: Ptr) -> bool {
    let group = u32::from(e.mem.u16(this.addr() + 0x10));
    e.call(IS_ATTACK_ACTION, &args![group]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0049_37e0, fn_004937e0(Ptr) -> bool),
        entry!(0x0049_3770, fn_00493770(Ptr) -> f32),
        entry!(0x0049_37a0, fn_004937a0(Ptr, u32) -> u32),
        entry!(0x0049_37c0, fn_004937c0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difference_of_the_two_values_is_stored_as_a_float() {
        let mut e = Engine::new();
        e.register(FIRST_VALUE, |_, _| 5.5f64.into_ret());
        e.register(SECOND_VALUE, |_, _| 2.25f64.into_ret());
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x0049_3770, &args![this]).f32(), 3.25);
    }

    #[test]
    fn element_word_indexes_sixteen_byte_elements() {
        let mut e = Engine::new();
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        let array = e.mem.alloc(0x30);
        e.mem.set_u32(array + 0x20, 0x1234);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x14, array);
        assert_eq!(e.call(0x0049_37a0, &args![this, 2u32]).u32(), 0x1234);
    }

    #[test]
    fn aim_action_passes_the_sixteen_bit_group() {
        let mut e = Engine::new();
        e.register(IS_AIM_ACTION, |_, a| (a[0] + 0x100).into_ret());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0xffff_0042);
        assert_eq!(e.call(0x0049_37c0, &args![this]).u32(), 0x142);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_004937e0_asks_about_the_group_at_10() {
        let mut e = Engine::new();
        e.register(IS_ATTACK_ACTION, |_, a| Ret {
            eax: 0x1200 | a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0xffff_0034);
        assert!(e.call(0x0049_37e0, &args![this]).bool());
        e.mem.set_u32(this + 0x10, 0x0034);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0049_37e0, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (IS_ATTACK_ACTION, vec![0x34]));
        // Only the low byte counts: the callee's upper bytes are not the answer.
        e.mem.set_u32(this + 0x10, 0x0000);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0049_37e0, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (IS_ATTACK_ACTION, vec![0]));
    }
}
