//! `fallout/magic/run_0080cc70` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `DemoralizeEffect` constructor `0080c890` (`thiscall`, three words).
const DEMORALIZE_EFFECT_CONSTRUCTOR: u32 = 0x0080_c890;
/// `ActiveEffect::GetSaveSize` (engine map, `00804f80`; `thiscall`, one word).
const ACTIVE_EFFECT_GET_SAVE_SIZE: u32 = 0x0080_4f80;
/// The base constructor `0082ba60` (unnamed in the engine map; `thiscall`, three words).
const BASE_CONSTRUCTOR: u32 = 0x0082_ba60;
/// The `DetectLifeEffect` vtable (`01079e7c`, RTTI `.?AVDetectLifeEffect@@`).
const DETECT_LIFE_EFFECT_VTABLE: u32 = 0x0107_9e7c;

// Translated from 0080cc70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080cc70 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map (`cdecl`, three words): allocates a 0x4c-byte
/// `DemoralizeEffect` (`00401000`) and constructs it with the three words (`0080c890`);
/// returns it, or null when the allocation fails.
pub fn fn_0080cc70(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        DEMORALIZE_EFFECT_CONSTRUCTOR,
        &args![block, caster, spell, effect],
    )
    .u32()
}

// Translated from 0080ccf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetSaveSize` slot (+0x10) of the `DemoralizeEffect` and `TurnUndeadEffect`
/// vtables (one shared body; `thiscall`, `RET 4`): the `ActiveEffect` save size
/// (`00804f80` with the same word) plus 1, as a 16-bit sum.
pub fn fn_0080ccf0(e: &mut Engine, this: Ptr, buffer: u32) -> u16 {
    let base = e
        .call(ACTIVE_EFFECT_GET_SAVE_SIZE, &args![this, buffer])
        .u16();
    base.wrapping_add(1)
}

// Translated from 0080cd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `DetectLifeEffect` constructor (by the vtable it stores; unnamed in the engine map;
/// `thiscall`, `RET 0xc`): runs the base constructor `0082ba60` with the three words, then
/// stores the vtable. Returns `this`.
pub fn fn_0080cd30(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this, caster, spell, effect]);
    e.mem.set_u32(this.addr(), DETECT_LIFE_EFFECT_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_cc70, fn_0080cc70(u32, u32, u32) -> u32),
        entry!(0x0080_ccf0, fn_0080ccf0(Ptr, u32) -> u16),
        entry!(0x0080_cd30, fn_0080cd30(Ptr, u32, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_0080cc70_allocates_and_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(DEMORALIZE_EFFECT_CONSTRUCTOR, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_cc70, &[7, 8, 9]).u32(), 0x5001);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x4c]]);
        assert_eq!(
            calls(&e, DEMORALIZE_EFFECT_CONSTRUCTOR),
            vec![vec![0x5000, 7, 8, 9]]
        );
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_cc70, &[7, 8, 9]).u32(), 0);
        assert!(calls(&e, DEMORALIZE_EFFECT_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn fn_0080ccf0_adds_one_in_16_bits() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_GET_SAVE_SIZE, |_, _| {
            0xabcd_0030u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_ccf0, &[0x100, 0x200]).u32(), 0x31);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_GET_SAVE_SIZE),
            vec![vec![0x100, 0x200]]
        );
        e.register(ACTIVE_EFFECT_GET_SAVE_SIZE, |_, _| 0xffffu32.into_ret());
        assert_eq!(e.call(0x0080_ccf0, &[0x100, 0x200]).u32(), 0);
    }

    #[test]
    fn fn_0080cd30_constructs_the_base_then_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_cd30, &[this, 1, 2, 3]).u32(), this);
        assert_eq!(calls(&e, BASE_CONSTRUCTOR), vec![vec![this, 1, 2, 3]]);
        assert_eq!(e.mem.u32(this), DETECT_LIFE_EFFECT_VTABLE);
    }
}
