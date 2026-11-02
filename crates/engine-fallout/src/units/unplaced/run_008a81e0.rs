//! `(unplaced)/run_008a81e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Four small `Actor` methods the engine map does not name. `008a8230` and
//! `008a8290` sit in the `Actor`, `Character` and `Creature` vtables
//! (`010843c0`, `01086bd8`, `01087218` hold `008a8230`; `010846c0` holds
//! `008a8290`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_ai::actor::Actor;

/// Slot +0x504 of the actor's process: the record it keeps about another
/// actor (two words: the actor, 0), or null.
const SLOT_PROCESS_RECORD_FOR: u32 = 0x504;
/// Default when there is no process or no record.
const NO_LEVEL: u32 = 0xffff_ff9c;
/// `00416870`: `NiPoint3::NiPoint3(x, y, z)`.
const NI_POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// `011dea3c`: the player character.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// `0087f800` (engine map: `actor.cpp`), called on every actor but the
/// player (thiscall, no arguments).
const NON_PLAYER_STEP: u32 = 0x0087_f800;

fn process_record(e: &mut Engine, this: Ptr<Actor>, other: Ptr) -> Option<u32> {
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() {
        return None;
    }
    let record = e
        .vcall(process.addr(), SLOT_PROCESS_RECORD_FOR, &args![other, 0u32])
        .u32();
    Some(record)
}

// Translated from 008a81e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +4 of the process's record about `other` (process slot
/// +0x504), 0 without a process or a record.
pub fn fn_008a81e0(e: &mut Engine, this: Ptr<Actor>, other: Ptr) -> u32 {
    match process_record(e, this, other) {
        Some(record) if record != 0 => e.mem.u32(record + 4),
        _ => 0,
    }
}

// Translated from 008a8230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +8 of the process's record about `other` (process slot
/// +0x504), -100 without a process or a record.
pub fn fn_008a8230(e: &mut Engine, this: Ptr<Actor>, other: Ptr) -> u32 {
    match process_record(e, this, other) {
        Some(record) if record != 0 => e.mem.u32(record + 8),
        _ => NO_LEVEL,
    }
}

// Translated from 008a8290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the point `(0, 0, fEditorLocZRot)` (built by `NiPoint3`'s
/// constructor in a local) to `result` and returns `result`.
pub fn fn_008a8290(e: &mut Engine, this: Ptr<Actor>, result: Ptr) -> Ptr {
    let z = x87_quiet_bits(e.get(this, Actor::fEditorLocZRot).to_bits());
    e.with_stack(12, |e, point| {
        e.call(NI_POINT3_CONSTRUCT, &args![point, 0u32, 0u32, z]);
        for i in 0..3 {
            let word = e.mem.u32(point.addr() + 4 * i);
            e.mem.set_u32(result.addr() + 4 * i, word);
        }
    });
    result
}

// Translated from 008a82e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `0087f800` on the actor unless it is the player character.
pub fn fn_008a82e0(e: &mut Engine, this: Ptr<Actor>) {
    if this.addr() != e.global::<u32>(PLAYER_CHARACTER) {
        e.call(NON_PLAYER_STEP, &args![this]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008a81e0, fn_008a81e0(Ptr<Actor>, Ptr) -> u32),
        entry!(0x008a8230, fn_008a8230(Ptr<Actor>, Ptr) -> u32),
        entry!(0x008a8290, fn_008a8290(Ptr<Actor>, Ptr) -> Ptr),
        entry!(0x008a82e0, fn_008a82e0(Ptr<Actor>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROCESS_VTABLE: u32 = 0x0002_0000;
    const RECORD_DOUBLE: u32 = 0x0001_0000;

    /// An actor whose process returns the record stored in the process's
    /// word at +4 (null when it is 0).
    fn engine(record: Option<(u32, u32)>) -> (Engine, Ptr<Actor>) {
        let mut e = Engine::new();
        e.register(RECORD_DOUBLE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.put_vtable(PROCESS_VTABLE, &[0; 0x142]);
        e.mem
            .set_u32(PROCESS_VTABLE + SLOT_PROCESS_RECORD_FOR, RECORD_DOUBLE);
        let actor: Ptr<Actor> = e.new_object();
        if let Some((at4, at8)) = record {
            let process = e.mem.alloc(8);
            e.mem.set_u32(process, PROCESS_VTABLE);
            let rec = e.mem.alloc(12);
            e.mem.set_u32(rec + 4, at4);
            e.mem.set_u32(rec + 8, at8);
            e.mem.set_u32(process + 4, rec);
            e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        }
        (e, actor)
    }

    #[test]
    fn record_word_4_or_zero() {
        let (mut e, actor) = engine(Some((7, 9)));
        assert_eq!(e.call(0x008a_81e0, &args![actor, 0x55u32]).u32(), 7);
        let (mut e, actor) = engine(None);
        assert_eq!(e.call(0x008a_81e0, &args![actor, 0x55u32]).u32(), 0);
    }

    #[test]
    fn record_word_8_or_minus_100() {
        let (mut e, actor) = engine(Some((7, 9)));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008a_8230, &args![actor, 0x55u32]).u32(), 9);
        let log = e.call_log.take().unwrap();
        let call = log.iter().find(|(a, _)| *a == RECORD_DOUBLE).unwrap();
        assert_eq!(&call.1[1..], &[0x55, 0]);
        let (mut e, actor) = engine(None);
        assert_eq!(e.call(0x008a_8230, &args![actor, 0x55u32]).i32(), -100);
    }

    #[test]
    fn missing_record_gives_the_defaults() {
        let (mut e, actor) = engine(Some((7, 9)));
        let process = e.get(actor, Actor::pCurrentProcess).addr();
        e.mem.set_u32(process + 4, 0);
        assert_eq!(e.call(0x008a_81e0, &args![actor, 0u32]).u32(), 0);
        assert_eq!(e.call(0x008a_8230, &args![actor, 0u32]).i32(), -100);
    }

    #[test]
    fn point_from_the_z_rotation() {
        let (mut e, actor) = engine(None);
        e.register(NI_POINT3_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.mem.set_u32(actor.addr() + 0x16c, 0x7f80_0001);
        let result: Ptr = Ptr::new(e.mem.alloc(12));
        let back = e.call(0x008a_8290, &args![actor, result]).ptr::<()>();
        assert_eq!(back, result);
        assert_eq!(e.mem.u32(result.addr()), 0);
        assert_eq!(e.mem.u32(result.addr() + 4), 0);
        assert_eq!(e.mem.u32(result.addr() + 8), 0x7fc0_0001);
    }

    #[test]
    fn non_player_step_skips_the_player() {
        let (mut e, actor) = engine(None);
        e.register(NON_PLAYER_STEP, |_, _| Ret::default());
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.call_log = Some(vec![]);
        e.call(0x008a_82e0, &args![actor]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == NON_PLAYER_STEP));
        e.set_global(PLAYER_CHARACTER, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_82e0, &args![actor]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(a, w)| *a == NON_PLAYER_STEP && w == &vec![actor.addr()]));
    }
}
