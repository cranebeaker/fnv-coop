//! `fallout/magic/run_0080c260` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Four `cdecl (caster, spell, effect)` factories of `CureEffect` (0x50 bytes): each
//! allocates the block (`00401000`) and runs one of the two `CureEffect` constructors with
//! a fixed fourth word (the spell type for `0080c070`, the archetype for `0080c0b0`; what
//! the numbers stand for is not confirmed here). Each returns the effect, or null when the
//! allocation fails. Their exception frames (`FS:[0]`) are not translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `CureEffect` constructor that takes the spell type (`0080c070`, `thiscall`, four
/// words).
const CURE_EFFECT_WITH_SPELL_TYPE: u32 = 0x0080_c070;
/// The `CureEffect` constructor that takes the archetype (`0080c0b0`, `thiscall`, four
/// words).
const CURE_EFFECT_WITH_ARCHETYPE: u32 = 0x0080_c0b0;

fn make_cure_effect(
    e: &mut Engine,
    constructor: u32,
    caster: u32,
    spell: u32,
    effect: u32,
    fourth: u32,
) -> u32 {
    let block = e.call(ALLOCATE, &args![0x50u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(constructor, &args![block, caster, spell, effect, fourth])
        .u32()
}

// Translated from 0080c260 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080c260 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map: a `CureEffect` through `0080c070` with spell type 1.
pub fn fn_0080c260(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    make_cure_effect(e, CURE_EFFECT_WITH_SPELL_TYPE, caster, spell, effect, 1)
}

// Translated from 0080c2e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080c2e0 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map: a `CureEffect` through `0080c0b0` with archetype 0x18.
pub fn fn_0080c2e0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    make_cure_effect(e, CURE_EFFECT_WITH_ARCHETYPE, caster, spell, effect, 0x18)
}

// Translated from 0080c360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080c360 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map: a `CureEffect` through `0080c070` with spell type 0xA.
pub fn fn_0080c360(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    make_cure_effect(e, CURE_EFFECT_WITH_SPELL_TYPE, caster, spell, effect, 0xa)
}

// Translated from 0080c3e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080c3e0 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map: a `CureEffect` through `0080c070` with spell type 5.
pub fn fn_0080c3e0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    make_cure_effect(e, CURE_EFFECT_WITH_SPELL_TYPE, caster, spell, effect, 5)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_c260, fn_0080c260(u32, u32, u32) -> u32),
        entry!(0x0080_c2e0, fn_0080c2e0(u32, u32, u32) -> u32),
        entry!(0x0080_c360, fn_0080c360(u32, u32, u32) -> u32),
        entry!(0x0080_c3e0, fn_0080c3e0(u32, u32, u32) -> u32),
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

    fn check(factory: u32, constructor: u32, fourth: u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(CURE_EFFECT_WITH_SPELL_TYPE, |_, a| (a[0] + 1).into_ret());
        e.register(CURE_EFFECT_WITH_ARCHETYPE, |_, a| (a[0] + 2).into_ret());
        e.call_log = Some(vec![]);
        let expected = if constructor == CURE_EFFECT_WITH_SPELL_TYPE {
            0x5001
        } else {
            0x5002
        };
        assert_eq!(e.call(factory, &[7, 8, 9]).u32(), expected);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x50]]);
        assert_eq!(calls(&e, constructor), vec![vec![0x5000, 7, 8, 9, fourth]]);

        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(factory, &[7, 8, 9]).u32(), 0);
        assert!(calls(&e, constructor).is_empty());
    }

    #[test]
    fn fn_0080c260_spell_type_1() {
        check(0x0080_c260, CURE_EFFECT_WITH_SPELL_TYPE, 1);
    }

    #[test]
    fn fn_0080c2e0_archetype_0x18() {
        check(0x0080_c2e0, CURE_EFFECT_WITH_ARCHETYPE, 0x18);
    }

    #[test]
    fn fn_0080c360_spell_type_0xa() {
        check(0x0080_c360, CURE_EFFECT_WITH_SPELL_TYPE, 0xa);
    }

    #[test]
    fn fn_0080c3e0_spell_type_5() {
        check(0x0080_c3e0, CURE_EFFECT_WITH_SPELL_TYPE, 5);
    }
}
