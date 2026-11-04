//! `fallout/magic/run_0080bbf0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `ActiveEffect` base constructor `00803d30` (`thiscall`, three words).
const ACTIVE_EFFECT_CONSTRUCTOR: u32 = 0x0080_3d30;
/// `ConcussionEffect`'s vtable (`01079bdc`).
const CONCUSSION_EFFECT_VTABLE: u32 = 0x0107_9bdc;

// Translated from 0080bc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ConcussionEffect` constructor (unnamed in the engine map; `ConcussionEffect::Copy`
/// and `0080bff0` call it on 0x4c-byte blocks; `thiscall`, `RET 0xc`: three words passed
/// on to the `ActiveEffect` constructor `00803d30`): runs that constructor, stores the
/// vtable `01079bdc` and sets `fTimer` (+0x48, Xbox PDB) to 0.0. Returns `this`.
pub fn fn_0080bc70(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(
        ACTIVE_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), CONCUSSION_EFFECT_VTABLE);
    e.mem.set_f32(this.addr() + 0x48, 0.0);
    this
}

/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `CommandHumanoidEffect` constructor `0080ba90` (`thiscall`, three words).
const COMMAND_HUMANOID_EFFECT_CONSTRUCTOR: u32 = 0x0080_ba90;

// Translated from 0080bbf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080bbf0 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map (`cdecl`, three words): allocates a 0x48-byte
/// `CommandHumanoidEffect` (`00401000`) and constructs it with the three words
/// (`0080ba90`); returns it, or null when the allocation fails.
pub fn fn_0080bbf0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x48u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        COMMAND_HUMANOID_EFFECT_CONSTRUCTOR,
        &args![block, caster, spell, effect],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_bc70, fn_0080bc70(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0080_bbf0, fn_0080bbf0(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod n049_tests {
    use super::*;

    #[test]
    fn fn_0080bbf0_allocates_and_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(COMMAND_HUMANOID_EFFECT_CONSTRUCTOR, |_, a| {
            (a[0] + 1).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_bbf0, &[7, 8, 9]).u32(), 0x5001);
        let log = e.call_log.take().unwrap();
        let allocs: Vec<_> = log.iter().filter(|(a, _)| *a == ALLOCATE).collect();
        assert_eq!(allocs[0].1, vec![0x48]);
        let ctors: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == COMMAND_HUMANOID_EFFECT_CONSTRUCTOR)
            .collect();
        assert_eq!(ctors[0].1, vec![0x5000, 7, 8, 9]);

        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_bbf0, &[7, 8, 9]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log
            .iter()
            .any(|(a, _)| *a == COMMAND_HUMANOID_EFFECT_CONSTRUCTOR));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_base_then_sets_vtable_and_timer() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            e.mem.set_u32(a[0] + 0x48, 0xffff_ffff);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_bc70, &[this, 1, 2, 3]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ACTIVE_EFFECT_CONSTRUCTOR, vec![this, 1, 2, 3]));
        assert_eq!(e.mem.u32(this), CONCUSSION_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x48), 0);
    }
}
