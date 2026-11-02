//! `fallout/magic/lockeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_magic::activeeffect::ActiveEffect;

/// `00401000`: the game's `operator new` (`cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
/// The `LockEffect` constructor `0080f390` (`thiscall`, `RET 0xC`: caster, spell, effect
/// item).
const LOCK_EFFECT_CONSTRUCTOR: u32 = 0x0080_f390;
/// `0059bb30` (`thiscall`): the effect's magic target.
const GET_MAGIC_TARGET: u32 = 0x0059_bb30;
/// `__RTDynamicCast` and the RTTI type descriptors `.?AVMagicTarget@@` and
/// `.?AVNonActorMagicTarget@@`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_MAGIC_TARGET: u32 = 0x011a_0ef8;
const TYPE_NON_ACTOR_MAGIC_TARGET: u32 = 0x0118_444c;
/// `TESObjectREFR::GetLock` (`00569160`), `AddLock` (`005690a0`), `AddLockChange`
/// (`005789b0`) and `Lock` (`00569250`) (engine map names; `thiscall`, no stack words).
const GET_LOCK: u32 = 0x0056_9160;
const ADD_LOCK: u32 = 0x0056_90a0;
const ADD_LOCK_CHANGE: u32 = 0x0057_89b0;
const LOCK: u32 = 0x0056_9250;
/// `ActiveEffect::Dispel` (engine map, `00804210`, `thiscall`, `RET 4`).
const DISPEL: u32 = 0x0080_4210;
/// The active effect's magnitude (`00495460`: the `float` at +0x1C, in `ST0`).
const MAGNITUDE: u32 = 0x0049_5460;
/// `_ftol2` (`00ec62c0`): truncates `ST0` to an integer in `EAX` (`ST0` passed as an `f64`).
const FTOL: u32 = 0x00ec_62c0;

// Translated from 0080f3c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080f3c0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `LockEffect::Copy` (Xbox PDB): a new `LockEffect` (0x48 bytes, `00401000`, constructed by
/// `0080f390`) with this effect's caster (+0x28), spell (+8) and effect item (+0xC), or
/// null when the allocation failed; this effect's slot +0x44 (`CopyData`) copies itself into
/// it. Returns it.
pub fn lock_effect_copy(e: &mut Engine, this: Ptr<ActiveEffect>) -> u32 {
    let block = e.call(ALLOCATE, &args![0x48u32]).u32();
    let copy = if block != 0 {
        let caster = e.get(this, ActiveEffect::pCaster);
        let spell = e.get(this, ActiveEffect::pSpell);
        let effect = e.get(this, ActiveEffect::pEffect);
        e.call(
            LOCK_EFFECT_CONSTRUCTOR,
            &args![block, caster, spell, effect],
        )
        .u32()
    } else {
        0
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

/// The reference of a `NonActorMagicTarget`: its `MagicTarget` part (+0xC) slot +4
/// (`GetTargetStatsObject`).
fn target_reference(e: &mut Engine, target: u32) -> u32 {
    e.vcall(target.wrapping_add(0xc), 4, &[]).u32()
}

// Translated from 0080f470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockEffect::Start` (Xbox PDB). Only for a non-actor target (the magic target cast to
/// `NonActorMagicTarget`) with a reference: when the reference has a lock whose flags byte
/// (+8) lacks bit 1 or has bit 0, the effect dispels itself (`ActiveEffect::Dispel(0)`).
/// Otherwise it adds a lock (`AddLock`) whose level byte is the magnitude truncated
/// (`_ftol2`), key word (+4) 0 and flags 2, then records the change (`AddLockChange`) and
/// locks the reference (`Lock`).
pub fn lock_effect_start(e: &mut Engine, this: Ptr) {
    let magic_target = e.call(GET_MAGIC_TARGET, &args![this]).u32();
    let target = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                magic_target,
                0u32,
                TYPE_MAGIC_TARGET,
                TYPE_NON_ACTOR_MAGIC_TARGET,
                0u32
            ],
        )
        .u32();
    if target == 0 || target_reference(e, target) == 0 {
        return;
    }
    let reference = target_reference(e, target);
    let lock = e.call(GET_LOCK, &args![reference]).u32();
    if lock != 0 {
        let flags = e.mem.u8(lock.wrapping_add(8));
        if flags & 2 == 0 || flags & 1 != 0 {
            e.call(DISPEL, &args![this, 0u32]);
            return;
        }
    }
    let reference = target_reference(e, target);
    let lock = e.call(ADD_LOCK, &args![reference]).u32();
    let magnitude = e.call(MAGNITUDE, &args![this]).f64();
    let level = e.call(FTOL, &args![magnitude]).u32();
    e.mem.set_u8(lock, level as u8);
    e.mem.set_u32(lock.wrapping_add(4), 0);
    e.mem.set_u8(lock.wrapping_add(8), 2);
    let reference = target_reference(e, target);
    e.call(ADD_LOCK_CHANGE, &args![reference]);
    let reference = target_reference(e, target);
    e.call(LOCK, &args![reference]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_f3c0, lock_effect_copy(Ptr<ActiveEffect>) -> u32),
        entry!(0x0080_f470, lock_effect_start(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn copy_constructs_a_lock_effect_or_answers_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0x0aa0u32.into_ret());
        e.register(LOCK_EFFECT_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(0x0310_0000, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x48 / 4];
        slots[0x44 / 4] = 0x0310_0000;
        e.put_vtable(0x0300_0000, &slots);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this, 0x0300_0000);
        e.mem.set_u32(this + 8, 0x22);
        e.mem.set_u32(this + 0xc, 0x33);
        e.mem.set_u32(this + 0x28, 0x11);
        assert_eq!(e.call(0x0080_f3c0, &args![this]).u32(), 0x0aa0);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x48]]);
        assert_eq!(
            calls(&e, LOCK_EFFECT_CONSTRUCTOR),
            vec![vec![0x0aa0, 0x11, 0x22, 0x33]]
        );
        assert_eq!(calls(&e, 0x0310_0000), vec![vec![this, 0x0aa0]]);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0080_f3c0, &args![this]).u32(), 0);
        assert_eq!(calls(&e, 0x0310_0000).last(), Some(&vec![this, 0]));
    }

    /// A non-actor target whose reference is 0x7000, and a lock block for `AddLock`.
    fn rig(existing_flags: Option<u8>) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let target = e.mem.alloc(0x28);
        e.mem.set_u32(target + 0xc, 0x0300_0000);
        e.put_vtable(0x0300_0000, &[0, 0x0310_0000]);
        e.register(0x0310_0000, |_, _| 0x7000u32.into_ret());
        e.register(GET_MAGIC_TARGET, |_, _| 0x6000u32.into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, _| target.into_ret());
        let old = e.mem.alloc(0xc);
        let lock = e.mem.alloc(0xc);
        e.mem.set_u32(lock + 4, 0x5555);
        match existing_flags {
            Some(flags) => {
                e.mem.set_u8(old + 8, flags);
                e.register_double(GET_LOCK, move |_, _| old.into_ret());
            }
            None => e.register(GET_LOCK, |_, _| 0u32.into_ret()),
        }
        e.register_double(ADD_LOCK, move |_, _| lock.into_ret());
        e.register(MAGNITUDE, |_, _| Ret {
            st0: 37.75,
            ..Ret::default()
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32).into_ret()
        });
        for addr in [ADD_LOCK_CHANGE, LOCK, DISPEL] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x48);
        (e, this, lock)
    }

    #[test]
    fn start_locks_the_reference_at_the_magnitude() {
        for flags in [None, Some(2u8)] {
            let (mut e, this, lock) = rig(flags);
            e.call(0x0080_f470, &args![this]);
            assert_eq!(e.mem.u8(lock), 37);
            assert_eq!(e.mem.u32(lock + 4), 0);
            assert_eq!(e.mem.u8(lock + 8), 2);
            assert_eq!(calls(&e, ADD_LOCK), vec![vec![0x7000]]);
            assert_eq!(calls(&e, ADD_LOCK_CHANGE), vec![vec![0x7000]]);
            assert_eq!(calls(&e, LOCK), vec![vec![0x7000]]);
            assert!(calls(&e, DISPEL).is_empty());
            assert_eq!(
                calls(&e, RT_DYNAMIC_CAST),
                vec![vec![
                    0x6000,
                    0,
                    TYPE_MAGIC_TARGET,
                    TYPE_NON_ACTOR_MAGIC_TARGET,
                    0
                ]]
            );
        }
    }

    #[test]
    fn start_dispels_on_a_lock_it_cannot_take() {
        for flags in [0u8, 1, 3] {
            let (mut e, this, _) = rig(Some(flags));
            e.call(0x0080_f470, &args![this]);
            assert_eq!(calls(&e, DISPEL), vec![vec![this, 0]]);
            assert!(calls(&e, ADD_LOCK).is_empty());
        }
        // No non-actor target, or no reference: nothing.
        let (mut e, this, _) = rig(None);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call(0x0080_f470, &args![this]);
        assert!(calls(&e, GET_LOCK).is_empty());
        let (mut e, this, _) = rig(None);
        e.register(0x0310_0000, |_, _| 0u32.into_ret());
        e.call(0x0080_f470, &args![this]);
        assert!(calls(&e, GET_LOCK).is_empty());
    }
}
