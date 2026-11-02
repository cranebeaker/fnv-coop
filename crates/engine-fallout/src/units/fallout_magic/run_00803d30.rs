//! `fallout/magic/run_00803d30` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ActiveEffect`'s constructor and destructor bodies and its flag setter
//! (the layout is `units/fallout_magic/activeeffect.rs`'s).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_magic::activeeffect::ActiveEffect;

/// `ActiveEffect`'s vtable (RTTI `.?AVActiveEffect@@`).
const ACTIVE_EFFECT_VTABLE: u32 = 0x0107_935c;
/// `PersistentSound` (Xbox PDB): the `BSSoundHandle` at +0x30.
const PERSISTENT_SOUND: u32 = 0x30;
/// `BSSoundHandle`'s constructor (`0041a250`), `IsValid` (`00ad8ce0`),
/// `Stop` (`00ad88f0`), `Release` (`00ad8d10`) and destructor
/// (`00483710`), `ECX` = the handle.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// `EffectItem::GetMagnitude` and `EffectItem::GetDuration` (Xbox PDB
/// names of `00404390` and `004044d0`), integers.
const EFFECT_ITEM_MAGNITUDE: u32 = 0x0040_4390;
const EFFECT_ITEM_DURATION: u32 = 0x0040_44d0;
/// The effect item's list the constructor tests (`00409140`, `ECX` =
/// effect item), its node's item address (`006815c0`) and the empty test
/// (`008256d0`).
const EFFECT_ITEM_LIST: u32 = 0x0040_9140;
const NODE_ITEM: u32 = 0x0068_15c0;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// The next node (`00726070`), the list's remove-all (`00470470`) and the
/// list's deleting destructor (`004702f0` with 1).
const NODE_NEXT: u32 = 0x0072_6070;
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
const LIST_DELETE: u32 = 0x0047_02f0;
/// The two calls made on each hit effect (`00461310` with 1, `00984f60`
/// with 0).
const HIT_EFFECT_FIRST: u32 = 0x0046_1310;
const HIT_EFFECT_SECOND: u32 = 0x0098_4f60;

// Translated from 00803d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect`'s constructor body (`this`; caster, magic item, effect
/// item): the vtable, the persistent sound handle, the spell and its type
/// (the item's slot +0x18), the effect, elapsed time 0, active, magnitude
/// and duration from the effect item (integers made floats), the caster,
/// cleared target, source, hit effects, flags and displacement spell; flag
/// 0x80 when the effect item's list (`00409140`) has a first node that is
/// not empty. Returns `this`.
// Translation incomplete: 00803d30 - SEH exception frame (FS:[0]) that destroys the sound handle on unwinding is not modelled.
pub fn fn_00803d30(
    e: &mut Engine,
    this: Ptr<ActiveEffect>,
    caster: u32,
    spell: u32,
    effect: u32,
) -> Ptr<ActiveEffect> {
    e.mem.set_u32(this.addr(), ACTIVE_EFFECT_VTABLE);
    e.call(
        SOUND_HANDLE_CONSTRUCT,
        &args![this.addr().wrapping_add(PERSISTENT_SOUND)],
    );
    e.set(this, ActiveEffect::pSpell, spell);
    let spell = e.get(this, ActiveEffect::pSpell);
    let kind = e.vcall(spell, 0x18, &[]).u32();
    e.set(this, ActiveEffect::eSpellType, kind);
    e.set(this, ActiveEffect::pEffect, effect);
    e.set(this, ActiveEffect::fElapsedSeconds, 0.0);
    e.set(this, ActiveEffect::bActive, 1);
    e.set(this, ActiveEffect::bStarted, 0);
    e.set(this, ActiveEffect::bFinished, 0);
    e.set(this, ActiveEffect::bDone, 0);
    let magnitude = e.call(EFFECT_ITEM_MAGNITUDE, &args![effect]).i32();
    let magnitude = f32::from_bits(x87_store32(f64::from(magnitude)));
    e.set(this, ActiveEffect::fMagnitude, magnitude);
    let duration = e.call(EFFECT_ITEM_DURATION, &args![effect]).i32();
    let duration = f32::from_bits(x87_store32(f64::from(duration)));
    e.set(this, ActiveEffect::fDuration, duration);
    e.set(this, ActiveEffect::pTarget, 0);
    e.set(this, ActiveEffect::pCaster, caster);
    e.set(this, ActiveEffect::pSource, 0);
    e.set(this, ActiveEffect::pHitEffects, 0);
    e.set(this, ActiveEffect::iFlags, 0);
    e.set(this, ActiveEffect::pDisplacementSpell, 0);
    let list = e.call(EFFECT_ITEM_LIST, &args![effect]).u32();
    let node = e.call(NODE_ITEM, &args![list]).u32();
    if effect != 0 && node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        fn_00803e80(e, this, 0x80);
    }
    this
}

// Translated from 00803e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the bits `flags` in `iFlags` (+0x18).
pub fn fn_00803e80(e: &mut Engine, this: Ptr<ActiveEffect>, flags: u32) {
    let value = e.get(this, ActiveEffect::iFlags) | flags;
    e.set(this, ActiveEffect::iFlags, value);
}

// Translated from 00803ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect`'s destructor body: the vtable, then for each hit effect
/// of the `pHitEffects` list (until an empty node) its two calls
/// (`00461310` with 1, `00984f60` with 0); empties and deletes the list;
/// stops and releases the persistent sound when it is valid and destroys
/// the handle.
// Translation incomplete: 00803ed0 - SEH exception frame (FS:[0]) for the sound handle's unwinding is not modelled.
pub fn fn_00803ed0(e: &mut Engine, this: Ptr<ActiveEffect>) {
    e.mem.set_u32(this.addr(), ACTIVE_EFFECT_VTABLE);
    let mut node = e.get(this, ActiveEffect::pHitEffects);
    while node != 0 {
        if e.call(LIST_IS_EMPTY, &args![node]).bool() {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let hit = e.mem.u32(slot);
        e.call(HIT_EFFECT_FIRST, &args![hit, 1u32]);
        e.call(HIT_EFFECT_SECOND, &args![hit, 0u32]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if e.get(this, ActiveEffect::pHitEffects) != 0 {
        let list = e.get(this, ActiveEffect::pHitEffects);
        e.call(LIST_REMOVE_ALL, &args![list]);
    }
    let list = e.get(this, ActiveEffect::pHitEffects);
    if list != 0 {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
    let sound = this.addr().wrapping_add(PERSISTENT_SOUND);
    if e.call(SOUND_HANDLE_IS_VALID, &args![sound]).bool() {
        e.call(SOUND_HANDLE_STOP, &args![sound]);
        e.call(SOUND_HANDLE_RELEASE, &args![sound]);
    }
    e.call(SOUND_HANDLE_DESTRUCT, &args![sound]);
}

/// The free `00401030` (`cdecl`, one word).
const FREE: u32 = 0x0040_1030;

// Translated from 00803ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActiveEffect`'s scalar deleting destructor (`thiscall`, `RET 4`: the
/// flags): runs the destructor body (`00803ed0`) and frees `this`
/// (`00401030`) when bit 0 of the flags is set. Returns `this`.
pub fn fn_00803ea0(e: &mut Engine, this: Ptr<ActiveEffect>, flags: u32) -> Ptr<ActiveEffect> {
    fn_00803ed0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0080_3d30,
            fn_00803d30(Ptr<ActiveEffect>, u32, u32, u32) -> Ptr<ActiveEffect>
        ),
        entry!(0x0080_3e80, fn_00803e80(Ptr<ActiveEffect>, u32)),
        entry!(0x0080_3ed0, fn_00803ed0(Ptr<ActiveEffect>)),
        entry!(
            0x0080_3ea0,
            fn_00803ea0(Ptr<ActiveEffect>, u32) -> Ptr<ActiveEffect>
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn constructor_fills_the_fields() {
        let mut e = Engine::new();
        e.register(SOUND_HANDLE_CONSTRUCT, |_, _| Ret::default());
        e.register(EFFECT_ITEM_MAGNITUDE, |_, _| ret(25));
        e.register(EFFECT_ITEM_DURATION, |_, _| ret(-3i32 as u32));
        e.register(EFFECT_ITEM_LIST, |_, _| ret(0x700));
        e.register(NODE_ITEM, |_, a| ret(a[0] + 4));
        e.register(LIST_IS_EMPTY, |_, _| ret(0));
        e.register(0x00b3_0018, |_, _| ret(6));
        let spell = e.mem.alloc(8);
        e.put_vtable(0x00b3_1000, &[0, 0, 0, 0, 0, 0, 0x00b3_0018]);
        e.mem.set_u32(spell, 0x00b3_1000);
        let effect = e.new_object::<ActiveEffect>();
        e.mem.write(effect.addr(), &[0xffu8; 0x48]);
        e.call_log = Some(vec![]);
        let r = e.call(0x0080_3d30, &args![effect, 0x11u32, spell, 0x33u32]);
        assert_eq!(r.u32(), effect.addr());
        assert_eq!(e.mem.u32(effect.addr()), ACTIVE_EFFECT_VTABLE);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_CONSTRUCT),
            vec![vec![effect.addr() + 0x30]]
        );
        assert_eq!(e.get(effect, ActiveEffect::pSpell), spell);
        assert_eq!(e.get(effect, ActiveEffect::eSpellType), 6);
        assert_eq!(e.get(effect, ActiveEffect::pEffect), 0x33);
        assert_eq!(e.get(effect, ActiveEffect::fMagnitude), 25.0);
        assert_eq!(e.get(effect, ActiveEffect::fDuration), -3.0);
        assert_eq!(e.get(effect, ActiveEffect::pCaster), 0x11);
        assert_eq!(e.get(effect, ActiveEffect::bActive), 1);
        assert_eq!(e.get(effect, ActiveEffect::bDone), 0);
        assert_eq!(e.get(effect, ActiveEffect::iFlags), 0x80);
        assert_eq!(e.get(effect, ActiveEffect::pTarget), 0);
        assert_eq!(e.get(effect, ActiveEffect::bWornEnchantment), 0xff);
        // An empty list: no flag.
        e.register(LIST_IS_EMPTY, |_, _| ret(1));
        fn_00803d30(&mut e, effect, 0x11, spell, 0x33);
        assert_eq!(e.get(effect, ActiveEffect::iFlags), 0);
    }

    #[test]
    fn flags_are_or_ed() {
        let mut e = Engine::new();
        let effect = e.new_object::<ActiveEffect>();
        e.set(effect, ActiveEffect::iFlags, 0x5);
        e.call(0x0080_3e80, &args![effect, 0x12u32]);
        assert_eq!(e.get(effect, ActiveEffect::iFlags), 0x17);
    }

    #[test]
    fn destructor_releases_hits_and_sound() {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        for f in [
            HIT_EFFECT_FIRST,
            HIT_EFFECT_SECOND,
            LIST_REMOVE_ALL,
            LIST_DELETE,
            SOUND_HANDLE_STOP,
            SOUND_HANDLE_RELEASE,
            SOUND_HANDLE_DESTRUCT,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(SOUND_HANDLE_IS_VALID, |_, _| ret(1));
        let effect = e.new_object::<ActiveEffect>();
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, 0x41);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 0x42);
        e.set(effect, ActiveEffect::pHitEffects, first);
        e.call_log = Some(vec![]);
        e.call(0x0080_3ed0, &args![effect]);
        assert_eq!(
            calls_to(&e, HIT_EFFECT_FIRST),
            vec![vec![0x41, 1], vec![0x42, 1]]
        );
        assert_eq!(
            calls_to(&e, HIT_EFFECT_SECOND),
            vec![vec![0x41, 0], vec![0x42, 0]]
        );
        assert_eq!(calls_to(&e, LIST_DELETE), vec![vec![first, 1]]);
        let sound = effect.addr() + 0x30;
        assert_eq!(calls_to(&e, SOUND_HANDLE_STOP), vec![vec![sound]]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_DESTRUCT), vec![vec![sound]]);
        // Without hit effects or a valid sound: only the handle's destructor.
        e.set(effect, ActiveEffect::pHitEffects, 0);
        e.register(SOUND_HANDLE_IS_VALID, |_, _| ret(0));
        e.call_log = Some(vec![]);
        fn_00803ed0(&mut e, effect);
        assert!(calls_to(&e, LIST_REMOVE_ALL).is_empty());
        assert!(calls_to(&e, SOUND_HANDLE_STOP).is_empty());
        assert_eq!(calls_to(&e, SOUND_HANDLE_DESTRUCT).len(), 1);
    }
}

#[cfg(test)]
mod tests_n048 {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = Engine::new();
        for addr in [
            SOUND_HANDLE_IS_VALID,
            SOUND_HANDLE_DESTRUCT,
            LIST_REMOVE_ALL,
            LIST_DELETE,
            FREE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let effect = e.new_object::<ActiveEffect>();
        for flags in [1u32, 2] {
            e.call_log = Some(vec![]);
            let r = e.call(0x0080_3ea0, &args![effect, flags]);
            assert_eq!(r.u32(), effect.addr());
            let log = e.call_log.take().unwrap();
            let frees: Vec<_> = log.iter().filter(|(a, _)| *a == FREE).collect();
            let destructs = log
                .iter()
                .filter(|(a, _)| *a == SOUND_HANDLE_DESTRUCT)
                .count();
            assert_eq!(destructs, 1);
            if flags & 1 != 0 {
                assert_eq!(frees.len(), 1);
                assert_eq!(frees[0].1, vec![effect.addr()]);
            } else {
                assert!(frees.is_empty());
            }
            assert_eq!(e.mem.u32(effect.addr()), ACTIVE_EFFECT_VTABLE);
        }
    }
}
