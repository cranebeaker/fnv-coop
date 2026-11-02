//! `(unplaced)/run_0040e600` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0040e210` (`spellitem.cpp`; thiscall, one word).
const CALLEE_0040E210: u32 = 0x0040_e210;

// Translated from 0040e650 (decompiled, FalloutNV.exe 1.4.0.525)
/// A base-adjusting forwarder (unnamed in the engine map): calls
/// `0040e210(this - 0x18, 0x20)`, the object 0x18 bytes before `this`, and answers its `AL`
/// (the exe ends on that call). It is slot `+0x1c` of `SpellItem`'s `MagicItem` vtable
/// (`01013f44`, RTTI), a `bool` virtual: `MagicItem`'s other implementation (`0047c850`)
/// answers `XOR AL,AL`.
pub fn fn_0040e650(e: &mut Engine, this: Ptr) -> bool {
    let object = this.addr().wrapping_sub(0x18);
    e.call(CALLEE_0040E210, &args![object, 0x20u32]).bool()
}

/// `00639b40` (`precipitation.cpp` in the engine map; thiscall, no stack words).
const CALLEE_00639B40: u32 = 0x0063_9b40;
/// `006491d0` (`magicformulas.cpp` in the engine map; `cdecl`, one `float` word).
const CALLEE_006491D0: u32 = 0x0064_91d0;

// Translated from 0040e600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot of the vtable at `01013f38` (unnamed in the engine map; `ECX` is a sub-object 0x24
/// bytes into its object, no stack words, plain `RET`): when `0040e210(object, 1)` answers
/// true in `AL`, runs `00639b40` on the object; otherwise calls the sub-object's own vtable
/// slot +8 with 0 and hands its `ST0`, stored as a `float`, to `006491d0` (`cdecl`).
/// Answers the `EAX` of whichever call ran last (`00639b40` at 0040e621, or `006491d0` at
/// 0040e63d; the RET 0040e648 follows both): the vtable `01013f28` is SpellItem's
/// EffectItemList sub-object (RTTI, sub-object +0x24) and its slot +0x10 is
/// `EffectItemList::GetLevel` (Xbox PDB layout), which returns `SkillLevel::Value` (Xbox PDB
/// `?GetLevel@SpellItem@@UAA?AW4Value@SkillLevel@@XZ`); the other implementation of the slot,
/// `009da760`, answers its value too. No exe reader of the slot was found.
pub fn fn_0040e600(e: &mut Engine, this: Ptr) -> u32 {
    let object = this.addr().wrapping_sub(0x24);
    if e.call(CALLEE_0040E210, &args![object, 1u32]).u8() != 0 {
        e.call(CALLEE_00639B40, &args![object]).u32()
    } else {
        let value = e.vcall(this.addr(), 8, &args![0u32]).f64();
        e.call(CALLEE_006491D0, &args![x87_store32(value)]).u32()
    }
}

// Translated from 0040e670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot of the vtable at `01013f64` (unnamed in the engine map; `ECX` is a sub-object 0x18
/// bytes into its object, plain `RET`): `0040e210(object, 0x40)`, whose `EAX` it leaves as
/// its own result.
pub fn fn_0040e670(e: &mut Engine, this: Ptr) -> u32 {
    let object = this.addr().wrapping_sub(0x18);
    e.call(CALLEE_0040E210, &args![object, 0x40u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_e650, fn_0040e650(Ptr) -> bool),
        entry!(0x0040_e600, fn_0040e600(Ptr) -> u32),
        entry!(0x0040_e670, fn_0040e670(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_the_adjusted_object_and_0x20() {
        let mut e = Engine::new();
        e.register(CALLEE_0040E210, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_e650, &args![0x0500_0018u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == CALLEE_0040E210).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x0500_0000, 0x20]);
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    fn engine(answer: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(CALLEE_0040E210, move |_, _| answer.into_ret());
        e.register(CALLEE_00639B40, |_, _| Ret::default());
        e.register(CALLEE_006491D0, |_, _| Ret::default());
        e.register(0x0060_0008, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.put_vtable(0x0070_0000, &[0, 0, 0x0060_0008]);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_0040e600_answers_the_level_the_last_call_answered() {
        let mut e = engine(0x0000_0101);
        e.register(CALLEE_00639B40, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register(CALLEE_006491D0, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        let object = e.mem.alloc(0x40);
        let sub = object + 0x24;
        e.mem.set_u32(sub, 0x0070_0000);
        // EAX at the RET 0040e648 is 00639b40's (0040e621) when 0040e210 answers true.
        assert_eq!(e.call(0x0040_e600, &args![sub]).u32(), 3);
        let mut e2 = engine(0x0000_0100);
        e2.register(CALLEE_006491D0, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        let object = e2.mem.alloc(0x40);
        let sub = object + 0x24;
        e2.mem.set_u32(sub, 0x0070_0000);
        // Otherwise it is 006491d0's (0040e63d).
        assert_eq!(e2.call(0x0040_e600, &args![sub]).u32(), 2);
    }

    #[test]
    fn fn_0040e600_runs_00639b40_when_flag_one_answers_true() {
        let mut e = engine(0x0000_0101);
        let object = e.mem.alloc(0x40);
        let sub = object + 0x24;
        e.mem.set_u32(sub, 0x0070_0000);
        e.call(0x0040_e600, &args![sub]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (CALLEE_0040E210, vec![object, 1]),
                (CALLEE_00639B40, vec![object]),
            ]
        );
    }

    #[test]
    fn fn_0040e600_passes_the_slot_8_float_otherwise() {
        let mut e = engine(0x0000_0100);
        let object = e.mem.alloc(0x40);
        let sub = object + 0x24;
        e.mem.set_u32(sub, 0x0070_0000);
        e.call(0x0040_e600, &args![sub]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (CALLEE_0040E210, vec![object, 1]),
                (0x0060_0008, vec![sub, 0]),
                (CALLEE_006491D0, vec![0.5f32.to_bits()]),
            ]
        );
    }

    #[test]
    fn fn_0040e670_tests_mask_0x40_of_the_adjusted_object() {
        let mut e = engine(0x1234_5601);
        assert_eq!(
            e.call(0x0040_e670, &args![0x0500_0018u32]).u32(),
            0x1234_5601
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (CALLEE_0040E210, vec![0x0500_0000, 0x40]));
    }
}

#[cfg(test)]
mod dropret_tests {
    use super::*;

    #[test]
    fn fn_0040e650_answers_the_callee_al() {
        for answer in [0x0000_0101u32, 0x0000_0100] {
            let mut e = Engine::new();
            e.register_double(CALLEE_0040E210, move |_, _| answer.into_ret());
            let result = e.call(0x0040_e650, &args![0x0500_0018u32]).bool();
            assert_eq!(result, answer & 0xff != 0);
        }
    }
}
