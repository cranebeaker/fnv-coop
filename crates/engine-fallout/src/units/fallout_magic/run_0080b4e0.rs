//! `fallout/magic/run_0080b4e0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `CommandCreatureEffect` constructor `0080b370` (`thiscall`, three words).
const COMMAND_CREATURE_EFFECT_CONSTRUCTOR: u32 = 0x0080_b370;
/// The `ActiveEffect` constructor `00803d30` (`thiscall`, three words).
const ACTIVE_EFFECT_CONSTRUCTOR: u32 = 0x0080_3d30;
/// The `CommandEffect` vtable (`01079a9c`, RTTI `.?AVCommandEffect@@`).
const COMMAND_EFFECT_VTABLE: u32 = 0x0107_9a9c;

// Translated from 0080b4e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080b4e0 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map (`cdecl`, three words): allocates a 0x48-byte
/// `CommandCreatureEffect` (`00401000`) and constructs it with the three words
/// (`0080b370`); returns it, or null when the allocation fails.
pub fn fn_0080b4e0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x48u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        COMMAND_CREATURE_EFFECT_CONSTRUCTOR,
        &args![block, caster, spell, effect],
    )
    .u32()
}

// Translated from 0080b560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CommandEffect` constructor (by the vtable it stores; unnamed in the engine map;
/// `thiscall`, `RET 0xc`): runs the `ActiveEffect` constructor `00803d30` with the three
/// words, then stores the vtable. Returns `this`.
pub fn fn_0080b560(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(
        ACTIVE_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), COMMAND_EFFECT_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_b4e0, fn_0080b4e0(u32, u32, u32) -> u32),
        entry!(0x0080_b560, fn_0080b560(Ptr, u32, u32, u32) -> Ptr),
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
    fn fn_0080b4e0_allocates_and_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(COMMAND_CREATURE_EFFECT_CONSTRUCTOR, |_, a| {
            (a[0] + 1).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_b4e0, &[7, 8, 9]).u32(), 0x5001);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x48]]);
        assert_eq!(
            calls(&e, COMMAND_CREATURE_EFFECT_CONSTRUCTOR),
            vec![vec![0x5000, 7, 8, 9]]
        );
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_b4e0, &[7, 8, 9]).u32(), 0);
        assert!(calls(&e, COMMAND_CREATURE_EFFECT_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn fn_0080b560_constructs_the_base_then_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_b560, &[this, 1, 2, 3]).u32(), this);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_CONSTRUCTOR),
            vec![vec![this, 1, 2, 3]]
        );
        assert_eq!(e.mem.u32(this), COMMAND_EFFECT_VTABLE);
    }
}
