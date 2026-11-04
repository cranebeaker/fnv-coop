//! `fallout/ai/run_0087d5e0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004f8960`: thiscall, returns an `int`.
const GET_VALUE: u32 = 0x004f_8960;

// Translated from 0087d6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: whether `004f8960(this)` returns exactly 1.
pub fn fn_0087d6a0(e: &mut Engine, this: Ptr) -> bool {
    e.call(GET_VALUE, &args![this]).i32() == 1
}

// Translated from 0087d600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, `RET 4`): copies the three words at `this`
/// +0x160 (a point) to `result` and returns `result` (returned by value).
pub fn fn_0087d600(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    for i in 0..3 {
        let word = e.mem.u32(this.addr() + 0x160 + 4 * i);
        e.mem.set_u32(result.addr() + 4 * i, word);
    }
    result
}

// Translated from 0087d670 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: from a subobject at +0xA4 of an actor (the actor value
/// owner), calls `0087f9f0` on the actor; its `EAX` is the result.
pub fn fn_0087d670(e: &mut Engine, this: Ptr) -> u32 {
    let actor = this.addr().wrapping_sub(0xa4);
    e.call(0x0087_f9f0, &args![actor]).u32()
}

// Translated from 0087d6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`): answers 1.0 (`FLD1`) in ST0.
pub fn fn_0087d6c0(_e: &mut Engine, _this: Ptr) -> f64 {
    1.0
}

// Translated from 0087d6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `this` - 0xA4 (from the subobject at +0xA4 to its object;
/// no null check).
pub fn fn_0087d6d0(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_sub(0xa4)
}

// Translated from 0087d6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `this` + 0x88 (the subobject there), 0 for a null `this`.
pub fn fn_0087d6f0(_e: &mut Engine, this: Ptr) -> u32 {
    if this.addr() != 0 {
        this.addr() + 0x88
    } else {
        0
    }
}

// Translated from 0087d720 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `this` + 0x94 (the subobject there), 0 for a null `this`.
pub fn fn_0087d720(_e: &mut Engine, this: Ptr) -> u32 {
    if this.addr() != 0 {
        this.addr() + 0x94
    } else {
        0
    }
}

// Translated from 0087d750 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `TESActorBase::IsImmobile` (`005f0c80`) of what `004181e0`
/// answers for `this`; its `AL` is the result.
pub fn fn_0087d750(e: &mut Engine, this: Ptr) -> bool {
    let base = e.call(0x0041_81e0, &args![this]).u32();
    e.call(0x005f_0c80, &args![base]).bool()
}

// Translated from 0087d770 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the byte at `this` +0x145 (in `AL`).
pub fn fn_0087d770(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x145)
}

// Translated from 0087d790 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `this` - 0x88 (from the subobject at +0x88 to its object;
/// no null check).
pub fn fn_0087d790(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_sub(0x88)
}

// Translated from 0087d7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `this` - 0x94 (from the subobject at +0x94 to its object;
/// no null check).
pub fn fn_0087d7b0(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_sub(0x94)
}

/// `00931cc0` (unnamed, engine map unit `mobileobject.cpp`, `thiscall`, one stack word).
const MOBILE_OBJECT_00931CC0: u32 = 0x0093_1cc0;

// Translated from 0087d5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one stack word, `RET 4`): passes `this` and the word on to
/// `00931cc0` and its `EAX` back.
pub fn fn_0087d5e0(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    e.call(MOBILE_OBJECT_00931CC0, &args![this, word]).u32()
}

// Translated from 0087d630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one stack word, `RET 4`): stores the byte at `this + 0x144`.
pub fn fn_0087d630(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x144, value);
}

// Translated from 0087d650 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, no stack words): the byte at `this + 0x144` (in `AL`).
pub fn fn_0087d650(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x144)
}

// Translated from 0087d690 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, no stack words): answers 0x4d.
pub fn fn_0087d690(_e: &mut Engine, _this: Ptr) -> u32 {
    0x4d
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0087d6a0, fn_0087d6a0(Ptr) -> bool),
        entry!(0x0087d600, fn_0087d600(Ptr, Ptr) -> Ptr),
        entry!(0x0087d670, fn_0087d670(Ptr) -> u32),
        entry!(0x0087d6c0, fn_0087d6c0(Ptr) -> f64),
        entry!(0x0087d6d0, fn_0087d6d0(Ptr) -> u32),
        entry!(0x0087d6f0, fn_0087d6f0(Ptr) -> u32),
        entry!(0x0087d720, fn_0087d720(Ptr) -> u32),
        entry!(0x0087d750, fn_0087d750(Ptr) -> bool),
        entry!(0x0087d770, fn_0087d770(Ptr) -> u8),
        entry!(0x0087d790, fn_0087d790(Ptr) -> u32),
        entry!(0x0087d7b0, fn_0087d7b0(Ptr) -> u32),
        entry!(0x0087_d5e0, fn_0087d5e0(Ptr, u32) -> u32),
        entry!(0x0087_d630, fn_0087d630(Ptr, u8)),
        entry!(0x0087_d650, fn_0087d650(Ptr) -> u8),
        entry!(0x0087_d690, fn_0087d690(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod b019_tests {
    use super::*;

    #[test]
    fn point_at_0x160_is_copied_out() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x170);
        for i in 0..3 {
            e.mem.set_u32(this + 0x160 + 4 * i, 0x10 + i);
        }
        let out = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0087_d600, &args![this, out]).u32(), out);
        let words: Vec<u32> = (0..3).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![0x10, 0x11, 0x12]);
    }

    #[test]
    fn owner_forwards_to_the_actor() {
        let mut e = Engine::new();
        e.register(0x0087_f9f0, |_, a| Ret {
            eax: a[0] ^ 0xff,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0087_d670, &args![0x10a4u32]).u32(), 0x1000 ^ 0xff);
    }

    #[test]
    fn constant_one_and_subobject_offsets() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0087_d6c0, &args![0x1000u32]).f64(), 1.0);
        assert_eq!(e.call(0x0087_d6d0, &args![0x10a4u32]).u32(), 0x1000);
        assert_eq!(e.call(0x0087_d6f0, &args![0x1000u32]).u32(), 0x1088);
        assert_eq!(e.call(0x0087_d6f0, &args![0u32]).u32(), 0);
        assert_eq!(e.call(0x0087_d720, &args![0x1000u32]).u32(), 0x1094);
        assert_eq!(e.call(0x0087_d720, &args![0u32]).u32(), 0);
        assert_eq!(e.call(0x0087_d790, &args![0x1088u32]).u32(), 0x1000);
        assert_eq!(e.call(0x0087_d7b0, &args![0x1094u32]).u32(), 0x1000);
        assert_eq!(e.call(0x0087_d7b0, &args![0u32]).u32(), 0xffff_ff6c);
    }

    #[test]
    fn immobile_asks_the_base_form() {
        let mut e = Engine::new();
        e.register(0x0041_81e0, |_, a| Ret {
            eax: a[0] + 0x20,
            ..Ret::default()
        });
        e.register(0x005f_0c80, |_, a| Ret {
            eax: (a[0] == 0x1020) as u32,
            ..Ret::default()
        });
        assert!(e.call(0x0087_d750, &args![0x1000u32]).bool());
        assert!(!e.call(0x0087_d750, &args![0x2000u32]).bool());
    }

    #[test]
    fn flag_byte_at_0x145() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x150);
        e.mem.set_u8(this + 0x145, 0xa5);
        assert_eq!(e.call(0x0087_d770, &args![this]).u32() & 0xff, 0xa5);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_only_for_exactly_one() {
        for (value, expected) in [(1u32, true), (0, false), (2, false), (u32::MAX, false)] {
            let mut e = Engine::new();
            e.register_double(GET_VALUE, move |_, _| Ret {
                eax: value,
                ..Ret::default()
            });
            assert_eq!(
                e.call(0x0087_d6a0, &args![Ptr::<()>::new(0x10)]).bool(),
                expected
            );
        }
    }
}

#[cfg(test)]
mod n027_tests {
    use super::*;

    #[test]
    fn fn_0087d5e0_forwards_to_the_mobile_object_call() {
        let mut e = Engine::new();
        e.register(MOBILE_OBJECT_00931CC0, |_, a| (a[0] ^ a[1]).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0087_d5e0, &args![0x1000u32, 0x22u32]).u32(),
            0x1022
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MOBILE_OBJECT_00931CC0)
            .collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x1000, 0x22]);
    }

    #[test]
    fn byte_at_0x144_round_trips() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x150);
        e.mem.set_u8(this + 0x145, 0x77);
        e.call(0x0087_d630, &args![this, 0x1ffu32]);
        assert_eq!(e.mem.u8(this + 0x144), 0xff);
        assert_eq!(e.mem.u8(this + 0x145), 0x77);
        assert_eq!(e.call(0x0087_d650, &args![this]).u8(), 0xff);
    }

    #[test]
    fn fn_0087d690_answers_0x4d() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0087_d690, &args![0x1000u32]).u32(), 0x4d);
    }
}
