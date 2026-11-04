//! `fallout/magic/run_0080bff0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `ConcussionEffect` constructor `0080bc70` (`thiscall`, three words).
const CONCUSSION_EFFECT_CONSTRUCTOR: u32 = 0x0080_bc70;

// Translated from 0080bff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words): allocates a 0x4c-byte
/// `ConcussionEffect` (`00401000`) and constructs it with the three words (`0080bc70`);
/// returns it, or null when the allocation fails.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0080bff0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_0080bff0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        CONCUSSION_EFFECT_CONSTRUCTOR,
        &args![block, caster, spell, effect],
    )
    .u32()
}

/// The `ActiveEffect` constructor `00803d30` (`thiscall`, three words).
const ACTIVE_EFFECT_CONSTRUCTOR: u32 = 0x0080_3d30;
/// The `CureEffect` vtable (`01079c8c`, RTTI `.?AVCureEffect@@`).
const CURE_EFFECT_VTABLE: u32 = 0x0107_9c8c;
/// `CureEffect::eSpellType` (Xbox PDB) +0x48.
const CURE_SPELL_TYPE: u32 = 0x48;
/// `CureEffect::eArchetypeID` (Xbox PDB) +0x4C.
const CURE_ARCHETYPE: u32 = 0x4c;

// Translated from 0080c070 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `CureEffect` constructor (by the vtable it stores; unnamed in the engine map;
/// `thiscall`, `RET 0x10`): runs the `ActiveEffect` constructor `00803d30` with the first
/// three words, stores the vtable, `eSpellType` (+0x48) from the fourth word and
/// `eArchetypeID` (+0x4C) = -1. Returns `this`.
pub fn fn_0080c070(
    e: &mut Engine,
    this: Ptr,
    caster: u32,
    spell: u32,
    effect: u32,
    spell_type: u32,
) -> Ptr {
    e.call(
        ACTIVE_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), CURE_EFFECT_VTABLE);
    e.mem.set_u32(this.addr() + CURE_SPELL_TYPE, spell_type);
    e.mem.set_u32(this.addr() + CURE_ARCHETYPE, 0xffff_ffff);
    this
}

// Translated from 0080c0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `CureEffect` constructor (by the vtable it stores; unnamed in the engine map;
/// `thiscall`, `RET 0x10`): runs the `ActiveEffect` constructor `00803d30` with the first
/// three words, stores the vtable, `eSpellType` (+0x48) = 0 and `eArchetypeID` (+0x4C)
/// from the fourth word. Returns `this`.
pub fn fn_0080c0b0(
    e: &mut Engine,
    this: Ptr,
    caster: u32,
    spell: u32,
    effect: u32,
    archetype: u32,
) -> Ptr {
    e.call(
        ACTIVE_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), CURE_EFFECT_VTABLE);
    e.mem.set_u32(this.addr() + CURE_SPELL_TYPE, 0);
    e.mem.set_u32(this.addr() + CURE_ARCHETYPE, archetype);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_bff0, fn_0080bff0(u32, u32, u32) -> u32),
        entry!(0x0080_c070, fn_0080c070(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x0080_c0b0, fn_0080c0b0(Ptr, u32, u32, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod n049_tests {
    use super::*;

    fn constructed(address: u32, fourth: u32) -> (u32, u32, u32, Vec<u32>) {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x48, 0xaaaa_aaaa);
            e.mem.set_u32(a[0] + 0x4c, 0xbbbb_bbbb);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &[this, 1, 2, 3, fourth]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == ACTIVE_EFFECT_CONSTRUCTOR)
            .collect();
        assert_eq!(base[0].1, vec![this, 1, 2, 3]);
        (
            e.mem.u32(this),
            e.mem.u32(this + 0x48),
            e.mem.u32(this + 0x4c),
            base[0].1.clone(),
        )
    }

    #[test]
    fn fn_0080c070_takes_the_spell_type() {
        let (vtable, spell_type, archetype, _) = constructed(0x0080_c070, 5);
        assert_eq!(vtable, CURE_EFFECT_VTABLE);
        assert_eq!(spell_type, 5);
        assert_eq!(archetype, 0xffff_ffff);
    }

    #[test]
    fn fn_0080c0b0_takes_the_archetype() {
        let (vtable, spell_type, archetype, _) = constructed(0x0080_c0b0, 0x18);
        assert_eq!(vtable, CURE_EFFECT_VTABLE);
        assert_eq!(spell_type, 0);
        assert_eq!(archetype, 0x18);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_and_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(CONCUSSION_EFFECT_CONSTRUCTOR, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_bff0, &[7, 8, 9]).u32(), 0x5001);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ALLOCATE, vec![0x4c]));
        assert_eq!(
            log[2],
            (CONCUSSION_EFFECT_CONSTRUCTOR, vec![0x5000, 7, 8, 9])
        );

        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_bff0, &[7, 8, 9]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == CONCUSSION_EFFECT_CONSTRUCTOR));
    }
}
