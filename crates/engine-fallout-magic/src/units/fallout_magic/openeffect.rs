//! `fallout/magic/openeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) and the type
/// descriptors the casts use: `011a0ef8` (`MagicTarget`), `0118444c`
/// (`NonActorMagicTarget`), `01183108` (`TESBoundObject`), `01186530`
/// (`TESObjectDOOR`) and `0118a674` (`TESObjectCONT`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_MAGIC_TARGET: u32 = 0x011a_0ef8;
const TYPE_NON_ACTOR_MAGIC_TARGET: u32 = 0x0118_444c;
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_TES_OBJECT_DOOR: u32 = 0x0118_6530;
const TYPE_TES_OBJECT_CONT: u32 = 0x0118_a674;
/// The non-actor magic target's +0xC, whose slot +4 answers its reference.
const TARGET_REFERENCE_BASE: u32 = 0xc;
/// `BGSSaveFormBuffer::GetForm` (the engine map's name for `007af430`,
/// `thiscall`: the word at +0x20, the reference's base form).
const BASE_FORM_007AF430: u32 = 0x007a_f430;
/// `operator new` (`00401000`, `cdecl`: the size) and `OpenEffect`'s
/// constructor (`008260b0`, `thiscall`, three words).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPEN_EFFECT_CONSTRUCT: u32 = 0x0082_60b0;
/// `0059bb30` (`thiscall`): the effect's magic target.
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `TESObjectREFR::GetLock` (engine map, `00569160`),
/// `TESObjectREFR::GetLockLevel` (`005691b0`) and `TESObjectREFR::UnLock`
/// (`005692a0`), all `thiscall` on the reference.
const GET_LOCK: u32 = 0x0056_9160;
const GET_LOCK_LEVEL: u32 = 0x0056_91b0;
const UNLOCK: u32 = 0x0056_92a0;
/// `00495460` (`thiscall`: the float at +0x1C, the effect's magnitude, in
/// `ST0`), `_ftol2` (`00ec62c0`) and `REFR_LOCK::NumericValueToEnum`
/// (engine map, `00430b40`, `cdecl`, one word).
const MAGNITUDE_00495460: u32 = 0x0049_5460;
const FTOL: u32 = 0x00ec_62c0;
const LOCK_LEVEL_FROM_VALUE: u32 = 0x0043_0b40;

/// The reference of a non-actor magic target (its +0xC's slot +4).
fn target_reference(e: &mut Engine, target: u32) -> u32 {
    e.vcall(target.wrapping_add(TARGET_REFERENCE_BASE), 4, &[])
        .u32()
}

// Translated from 0080f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `OpenEffect::CheckTarget` (engine map; `thiscall`, `RET 4`: the magic
/// target): false unless the target is a `NonActorMagicTarget` with a
/// reference; then true when the reference's base form is a door or a
/// container (`TESObjectDOOR`, `TESObjectCONT`).
pub fn open_effect_check_target(e: &mut Engine, _this: Ptr, target: u32) -> u8 {
    let target = e
        .call(
            DYNAMIC_CAST,
            &args![
                target,
                0u32,
                TYPE_MAGIC_TARGET,
                TYPE_NON_ACTOR_MAGIC_TARGET,
                0u32
            ],
        )
        .u32();
    if target == 0 || target_reference(e, target) == 0 {
        return 0;
    }
    let reference = target_reference(e, target);
    let form = e.call(BASE_FORM_007AF430, &args![reference]).u32();
    let door = e
        .call(
            DYNAMIC_CAST,
            &args![
                form,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_TES_OBJECT_DOOR,
                0u32
            ],
        )
        .u32();
    let reference = target_reference(e, target);
    let form = e.call(BASE_FORM_007AF430, &args![reference]).u32();
    let container = e
        .call(
            DYNAMIC_CAST,
            &args![
                form,
                0u32,
                TYPE_TES_BOUND_OBJECT,
                TYPE_TES_OBJECT_CONT,
                0u32
            ],
        )
        .u32();
    u8::from(door != 0 || container != 0)
}

// Translated from 008260e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `OpenEffect::Copy` (engine map; `thiscall`): a new 0x48-byte
/// `OpenEffect` (`00401000`, then `008260b0` with the caster +0x28, magic
/// item +8 and effect item +0xC, or 0 when the allocation fails), handed to
/// `this`'s slot +0x44 (`CopyData`). Returns the copy.
// Translation incomplete: 008260e0 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
pub fn open_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    let copy = if block != 0 {
        let base = this.addr();
        let caster = e.mem.u32(base.wrapping_add(0x28));
        let spell = e.mem.u32(base.wrapping_add(8));
        let effect = e.mem.u32(base.wrapping_add(0xc));
        e.call(OPEN_EFFECT_CONSTRUCT, &args![block, caster, spell, effect])
            .u32()
    } else {
        0
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 00826190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `OpenEffect::Start` (engine map; `thiscall`): when the effect's magic
/// target is a `NonActorMagicTarget` with a reference that is locked
/// (`GetLock`), and the lock level of the effect's magnitude (`_ftol2`,
/// `REFR_LOCK::NumericValueToEnum`) is at least the reference's lock
/// level, unlocks the reference.
pub fn open_effect_start(e: &mut Engine, this: Ptr) {
    let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
    let target = e
        .call(
            DYNAMIC_CAST,
            &args![
                target,
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
    let magnitude = e.call(MAGNITUDE_00495460, &args![this]).st0;
    let magnitude = e.call(FTOL, &args![magnitude]).u32();
    let strength = e.call(LOCK_LEVEL_FROM_VALUE, &args![magnitude]).i32();
    let reference = target_reference(e, target);
    let level = e.call(GET_LOCK_LEVEL, &args![reference]).u32();
    let level = e.call(LOCK_LEVEL_FROM_VALUE, &args![level]).i32();
    if lock != 0 && strength >= level {
        let reference = target_reference(e, target);
        e.call(UNLOCK, &args![reference]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_f580, open_effect_check_target(Ptr, u32) -> u8),
        entry!(0x0082_60e0, open_effect_copy(Ptr) -> u32),
        entry!(0x0082_6190, open_effect_start(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// A non-actor magic target whose +0xC answers `reference` through slot
    /// +4 (code `0x0300_0004`); the casts answer the target for the first
    /// cast and `door` / `container` for the others.
    fn target(e: &mut Engine, reference: u32, door: u32, container: u32) -> u32 {
        let target = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0300_0004);
        e.mem.set_u32(target + 0xc, vtable);
        e.register_double(0x0300_0004, move |_, _| ret(reference));
        e.register_double(DYNAMIC_CAST, move |_, a| {
            ret(match a[3] {
                TYPE_NON_ACTOR_MAGIC_TARGET => a[0],
                TYPE_TES_OBJECT_DOOR => door,
                _ => container,
            })
        });
        e.register(BASE_FORM_007AF430, |_, a| ret(a[0] + 0x20));
        target
    }

    #[test]
    fn check_target_wants_a_door_or_a_container() {
        let mut e = Engine::new();
        let t = target(&mut e, 0x500, 0, 0);
        let (r, log) = run(&mut e, 0x0080_f580, &args![0u32, t]);
        assert_eq!(r.u32(), 0);
        assert_eq!(
            calls(&log, DYNAMIC_CAST)[1],
            vec![0x520, 0, TYPE_TES_BOUND_OBJECT, TYPE_TES_OBJECT_DOOR, 0]
        );
        assert_eq!(
            calls(&log, DYNAMIC_CAST)[2],
            vec![0x520, 0, TYPE_TES_BOUND_OBJECT, TYPE_TES_OBJECT_CONT, 0]
        );
        let t = target(&mut e, 0x500, 1, 0);
        assert_eq!(e.call(0x0080_f580, &args![0u32, t]).u32(), 1);
        let t = target(&mut e, 0x500, 0, 1);
        assert_eq!(e.call(0x0080_f580, &args![0u32, t]).u32(), 1);
        // No reference: false without the form casts.
        let t = target(&mut e, 0, 1, 1);
        let (r, log) = run(&mut e, 0x0080_f580, &args![0u32, t]);
        assert_eq!(r.u32(), 0);
        assert_eq!(calls(&log, DYNAMIC_CAST).len(), 1);
    }

    #[test]
    fn copy_makes_an_open_effect() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, _| ret(0x9000));
        e.register(OPEN_EFFECT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(0x0300_0044, |_, _| Ret::default());
        let this = e.mem.alloc(0x48);
        let vtable = e.mem.alloc(0x48);
        e.mem.set_u32(vtable + 0x44, 0x0300_0044);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x28, 1);
        e.mem.set_u32(this + 8, 2);
        e.mem.set_u32(this + 0xc, 3);
        let (r, log) = run(&mut e, 0x0082_60e0, &args![this]);
        assert_eq!(r.u32(), 0x9000);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![0x48]]);
        assert_eq!(
            calls(&log, OPEN_EFFECT_CONSTRUCT),
            vec![vec![0x9000, 1, 2, 3]]
        );
        assert_eq!(calls(&log, 0x0300_0044), vec![vec![this, 0x9000]]);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        let (r, log) = run(&mut e, 0x0082_60e0, &args![this]);
        assert_eq!(r.u32(), 0);
        assert_eq!(calls(&log, 0x0300_0044), vec![vec![this, 0]]);
    }

    #[test]
    fn start_unlocks_up_to_the_magnitude() {
        let mut e = Engine::new();
        let t = target(&mut e, 0x500, 0, 0);
        e.register_double(EFFECT_TARGET_0059BB30, move |_, _| ret(t));
        e.register(GET_LOCK, |_, _| ret(0x1c0c));
        e.register(MAGNITUDE_00495460, |_, _| Ret {
            st0: 50.0,
            ..Ret::default()
        });
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.register(LOCK_LEVEL_FROM_VALUE, |_, a| ret(a[0] / 25));
        e.register(GET_LOCK_LEVEL, |_, _| ret(50));
        e.register(UNLOCK, |_, _| Ret::default());
        let (_, log) = run(&mut e, 0x0082_6190, &args![0x40u32]);
        assert_eq!(calls(&log, MAGNITUDE_00495460), vec![vec![0x40]]);
        assert_eq!(calls(&log, LOCK_LEVEL_FROM_VALUE), vec![vec![50], vec![50]]);
        assert_eq!(calls(&log, UNLOCK), vec![vec![0x500]]);
        e.register(GET_LOCK_LEVEL, |_, _| ret(75));
        let (_, log) = run(&mut e, 0x0082_6190, &args![0x40u32]);
        assert!(calls(&log, UNLOCK).is_empty());
        e.register(GET_LOCK_LEVEL, |_, _| ret(50));
        e.register(GET_LOCK, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0082_6190, &args![0x40u32]);
        assert!(calls(&log, UNLOCK).is_empty());
        let t = target(&mut e, 0, 0, 0);
        e.register_double(EFFECT_TARGET_0059BB30, move |_, _| ret(t));
        let (_, log) = run(&mut e, 0x0082_6190, &args![0x40u32]);
        assert!(calls(&log, GET_LOCK).is_empty());
    }
}
