//! `fallout shared/run_005ec040` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BGSQuestObjective`'s vtable (its RTTI names `BGSQuestObjective`).
const QUEST_OBJECTIVE_VTABLE: u32 = 0x0104_7088;
/// The string constructor (`004037b0`, `ECX` = string).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// The two-word list node constructor (`0096a2d0`, `ECX` = node).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The objective's field reset (`005ec1b0`, `ECX` = objective).
const OBJECTIVE_RESET: u32 = 0x005e_c1b0;

// Translated from 005ec040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of the byte at `+0` (unnamed in the map; `ECX`, no stack words).
/// `TESCondition` calls it on a condition item, whose first byte holds the
/// comparison flags; there bit 0 joins the item to the next by OR.
pub fn fn_005ec040(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr()) & 1 != 0
}

// Translated from 005ec090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ec090 - the exception-unwinding frame (FS:[0]) around the member constructors and the security-cookie bookkeeping are not translated.
/// The `BGSQuestObjective` constructor (unnamed in the map; `ECX`, no stack
/// words; the vtable it stores is `BGSQuestObjective`'s): stores the
/// vtable, constructs the display text at `+8` (`004037b0`) and the target
/// list at `+0x14` (`0096a2d0`), resets the fields (`005ec1b0`) and returns
/// `this`.
pub fn fn_005ec090(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), QUEST_OBJECTIVE_VTABLE);
    e.call(STRING_CONSTRUCT, &args![this.byte_add(8)]);
    e.call(LIST_CONSTRUCT, &args![this.byte_add(0x14)]);
    e.call(OBJECTIVE_RESET, &args![this]);
    this
}

// Translated from 005ec060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bits 5 to 7 of the byte at `+0` as a number 0 to 7 (unnamed in the map;
/// `ECX`, no stack words): the byte sign-extended, masked with 0xE0 and
/// shifted right arithmetically by 5. `TESCondition` items keep their
/// comparison operator there.
pub fn fn_005ec060(e: &mut Engine, this: Ptr) -> u32 {
    let flags = i32::from(e.mem.u8(this.addr()) as i8);
    ((flags & 0xe0) >> 5) as u32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005ec040, fn_005ec040(Ptr) -> bool),
        entry!(0x005ec090, fn_005ec090(Ptr) -> Ptr),
        entry!(0x005ec060, fn_005ec060(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_stores_the_vtable_and_builds_the_members_in_order() {
        let mut e = Engine::new();
        for addr in [STRING_CONSTRUCT, LIST_CONSTRUCT, OBJECTIVE_RESET] {
            e.register(addr, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x24);
        let back = e.call(0x005e_c090, &args![Ptr::<()>::new(this)]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), QUEST_OBJECTIVE_VTABLE);
        let calls: Vec<(u32, Vec<u32>)> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x005e_c090)
            .collect();
        assert_eq!(
            calls,
            vec![
                (STRING_CONSTRUCT, vec![this + 8]),
                (LIST_CONSTRUCT, vec![this + 0x14]),
                (OBJECTIVE_RESET, vec![this]),
            ]
        );
    }

    #[test]
    fn answers_bit_0_of_the_first_byte() {
        let mut e = Engine::new();
        let item = e.mem.alloc(4);
        for (byte, expected) in [(0u8, false), (1, true), (0xfe, false), (0x81, true)] {
            e.mem.set_u8(item, byte);
            let back = e.call(0x005e_c040, &args![Ptr::<()>::new(item)]).bool();
            assert_eq!(back, expected, "byte {byte:#x}");
        }
    }

    #[test]
    fn bits_5_to_7_of_the_first_byte_005ec060() {
        let mut e = Engine::new();
        let item = e.mem.alloc(4);
        for (byte, expected) in [(0x00u8, 0u32), (0x1f, 0), (0x20, 1), (0xa1, 5), (0xff, 7)] {
            e.mem.set_u8(item, byte);
            assert_eq!(e.call(0x005e_c060, &args![item]).u32(), expected);
        }
    }
}
