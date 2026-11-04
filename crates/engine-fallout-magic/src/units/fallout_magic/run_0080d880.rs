//! `fallout/magic/run_0080d880` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0080d930 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00803ed0`, a destructor with an exception frame
/// that stores the vtable `0107935c` and tears down the sound handle and the list at
/// `+0x40`, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00803ed0`'s disassembly ends
/// with its exception-frame unlink and returns no value (Ghidra types it void).
pub fn fn_0080d930(e: &mut Engine, this: Ptr) {
    e.call(0x0080_3ed0, &args![this]);
}

/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `DisintegrateWeaponEffect` constructor `0080d6b0` (`thiscall`, three words).
const DISINTEGRATE_WEAPON_EFFECT_CONSTRUCTOR: u32 = 0x0080_d6b0;
/// The `ActiveEffect` constructor `00803d30` (`thiscall`, three words).
const ACTIVE_EFFECT_CONSTRUCTOR: u32 = 0x0080_3d30;
/// The `DispelEffect` vtable (`0107a0cc`, RTTI `.?AVDispelEffect@@`).
const DISPEL_EFFECT_VTABLE: u32 = 0x0107_a0cc;

// Translated from 0080d880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080d880 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map (`cdecl`, three words): allocates a 0x48-byte
/// `DisintegrateWeaponEffect` (`00401000`) and constructs it with the three words
/// (`0080d6b0`); returns it, or null when the allocation fails.
pub fn fn_0080d880(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x48u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        DISINTEGRATE_WEAPON_EFFECT_CONSTRUCTOR,
        &args![block, caster, spell, effect],
    )
    .u32()
}

// Translated from 0080d900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `DispelEffect` constructor (by the vtable it stores; unnamed in the engine map;
/// `thiscall`, `RET 0xc`): runs the `ActiveEffect` constructor `00803d30` with the three
/// words, then stores the vtable. Returns `this`.
pub fn fn_0080d900(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(
        ACTIVE_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), DISPEL_EFFECT_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_d930, fn_0080d930(Ptr)),
        entry!(0x0080_d880, fn_0080d880(u32, u32, u32) -> u32),
        entry!(0x0080_d900, fn_0080d900(Ptr, u32, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod n049_tests {
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
    fn fn_0080d880_allocates_and_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(DISINTEGRATE_WEAPON_EFFECT_CONSTRUCTOR, |_, a| {
            (a[0] + 1).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_d880, &[7, 8, 9]).u32(), 0x5001);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x48]]);
        assert_eq!(
            calls(&e, DISINTEGRATE_WEAPON_EFFECT_CONSTRUCTOR),
            vec![vec![0x5000, 7, 8, 9]]
        );
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_d880, &[7, 8, 9]).u32(), 0);
        assert!(calls(&e, DISINTEGRATE_WEAPON_EFFECT_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn fn_0080d900_constructs_the_base_then_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_d900, &[this, 1, 2, 3]).u32(), this);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_CONSTRUCTOR),
            vec![vec![this, 1, 2, 3]]
        );
        assert_eq!(e.mem.u32(this), DISPEL_EFFECT_VTABLE);
    }
}

#[cfg(test)]
mod thunk_tests {
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
    fn fwd_a03_0080d930_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0080_3ed0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0080_d930, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0080_3ed0), vec![vec![0x5000]]);
    }
}
