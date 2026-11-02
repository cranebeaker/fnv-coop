//! `fallout/ai/combat/combatprocedureswitchweapon.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009da7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `010920cc` (RTTI `.?AVCombatProcedureSwitchWeapon@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009da7a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_20cc);
    e.call(0x009c_a6c0, &args![this]);
}

// Translated from 009da760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: returns 5 (`thiscall`, `this` unread). The
/// body is shared by the linker (several vtables point to it).
pub fn fn_009da760(_e: &mut Engine, _this: Ptr) -> u32 {
    5
}

// ---- Batch n025 ---------------------------------------------------------------------------

/// `CombatProcedureSwitchWeapon` (Xbox PDB), 0x18 bytes: the `CombatProcedure` base
/// (vtable, `pCombatController` +0x4, `eStatus` +0x8, `pFailureReason` +0xc), then
/// `pWeapon` +0x10 and `bEquipQueued` +0x14; the PC offsets are the PDB's (constructor
/// `009da720`).
const SWITCH_WEAPON: u32 = 0x10;
const SWITCH_EQUIP_QUEUED: u32 = 0x14;
/// The `CombatProcedure` base constructor (`009ca680`, `thiscall`).
const PROCEDURE_CONSTRUCT: u32 = 0x009c_a680;
/// Ends the procedure (`009caae0`, `thiscall`).
const PROCEDURE_DONE: u32 = 0x009c_aae0;
/// `CombatProcedure::GetAttacker` (Xbox PDB, `00996de0`).
const GET_ATTACKER: u32 = 0x0099_6de0;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map name, `008d8520`) on the actor:
/// the object whose slots `+0x148` and `+0x454` the procedures call.
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// `008a6840(actor, value)` (unit `actor.cpp`).
const ACTOR_SET_008A6840: u32 = 0x008a_6840;
/// `CombatProcedure::LoadGame` (Xbox PDB, `00996ef0`).
const PROCEDURE_LOAD_GAME: u32 = 0x0099_6ef0;
/// `TESForm`'s and `TESObjectWEAP`'s RTTI type descriptors.
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TES_OBJECT_WEAP_TYPE: u32 = 0x0118_3998;

// Translated from 009da720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureSwitchWeapon::CombatProcedureSwitchWeapon` (Xbox PDB; `thiscall`,
/// `RET 4`): the base constructor (`009ca680`), the vtable `010920cc`, `pWeapon` = `weapon`
/// and no equip queued. Returns `this`.
pub fn combat_procedure_switch_weapon_construct(e: &mut Engine, this: Ptr, weapon: u32) -> Ptr {
    e.call(PROCEDURE_CONSTRUCT, &args![this]);
    let me = this.addr();
    e.mem.set_u32(me, 0x0109_20cc);
    e.mem.set_u32(me + SWITCH_WEAPON, weapon);
    e.mem.set_u8(me + SWITCH_EQUIP_QUEUED, 0);
    this
}

// Translated from 009da770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatProcedureSwitchWeapon` scalar deleting destructor (`thiscall`, `RET 4`): the
/// destructor `009da7a0`, then `operator delete` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_009da770(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009da7a0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 009da7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureSwitchWeapon::Update` (Xbox PDB; `thiscall`): a fleeing controller
/// (`00981990`) ends the procedure (`009caae0`). Nothing happens while the attacker has an
/// animation action (`008a7570` not -1). The equipped weapon is the form (`0044ddc0`) of the
/// process object's slot `+0x148` entry (0 without one).
/// - Before the equip is queued and while the weapon differs: when the process object's slot
///   `+0x454` is set, `008a6840(attacker, 0)` and nothing else; otherwise the weapon is
///   queued (`Actor::QueueEquipObject` with its count `00575610` when `004c0bf0` says it is
///   counted, else 1, then 0, 1, 0, 1), or for no weapon the current one is unequipped
///   (`Actor::QueueUnEquipObject` with the entry's `00726070`, 0, 0, 0, 1), and the equip
///   is marked queued.
/// - Once the weapon is equipped: `008a6840(attacker, 1)` unless slot `+0x454` is set, the
///   power-attack deltas (`0097ff60`) for no weapon or one `006450c0` accepts, and the
///   procedure ends.
pub fn combat_procedure_switch_weapon_update(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let controller = e.mem.u32(me + 4);
    if e.call(0x0098_1990, &args![controller]).bool() {
        e.call(PROCEDURE_DONE, &args![this]);
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    if e.call(0x008a_7570, &args![attacker]).i32() != -1 {
        return;
    }
    let process = e.call(ACTOR_PROCESS, &args![attacker]).u32();
    let entry = e.vcall(process, 0x148, &args![]).u32();
    let current = if entry != 0 {
        e.call(0x0044_ddc0, &args![entry]).u32()
    } else {
        0
    };
    if e.mem.u8(me + SWITCH_EQUIP_QUEUED) == 0 && current != e.mem.u32(me + SWITCH_WEAPON) {
        let process = e.call(ACTOR_PROCESS, &args![attacker]).u32();
        if e.vcall(process, 0x454, &args![]).bool() {
            e.call(ACTOR_SET_008A6840, &args![attacker, 0u32]);
            return;
        }
        if e.mem.u32(me + SWITCH_WEAPON) != 0 {
            let mut count = 1u32;
            let weapon = e.mem.u32(me + SWITCH_WEAPON);
            if e.call(0x004c_0bf0, &args![weapon]).bool() {
                let weapon = e.mem.u32(me + SWITCH_WEAPON);
                count = e.call(0x0057_5610, &args![attacker, weapon]).u32();
            }
            let weapon = e.mem.u32(me + SWITCH_WEAPON);
            e.call(
                0x0088_c650,
                &args![attacker, weapon, count, 0u32, 1u32, 0u32, 1u32],
            );
        } else {
            let extra = e.call(0x0072_6070, &args![entry]).u32();
            e.call(
                0x0088_c790,
                &args![attacker, current, extra, 0u32, 0u32, 0u32, 1u32],
            );
        }
        e.mem.set_u8(me + SWITCH_EQUIP_QUEUED, 1);
        return;
    }
    if current != e.mem.u32(me + SWITCH_WEAPON) {
        return;
    }
    let process = e.call(ACTOR_PROCESS, &args![attacker]).u32();
    if !e.vcall(process, 0x454, &args![]).bool() {
        e.call(ACTOR_SET_008A6840, &args![attacker, 1u32]);
    }
    if current == 0 || e.call(0x0064_50c0, &args![current]).bool() {
        let controller = e.mem.u32(me + 4);
        e.call(0x0097_ff60, &args![controller]);
    }
    e.call(PROCEDURE_DONE, &args![this]);
}

// Translated from 009da9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureSwitchWeapon::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): the procedure's
/// own load (`00996ef0`), the weapon (`008648a0`'s form id through `004839c0` and a cast to
/// `TESObjectWEAP`, `00ec43fb`) and the equip byte (`00864980`).
pub fn combat_procedure_switch_weapon_load_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(PROCEDURE_LOAD_GAME, &args![this, buffer]);
    let id = e.call(0x0086_48a0, &args![buffer]).u32();
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let weapon = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, TES_FORM_TYPE, TES_OBJECT_WEAP_TYPE, 0u32],
        )
        .u32();
    e.mem.set_u32(me + SWITCH_WEAPON, weapon);
    e.call(0x0086_4980, &args![buffer, me + SWITCH_EQUIP_QUEUED, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009d_a720,
            combat_procedure_switch_weapon_construct(Ptr, u32) -> Ptr
        ),
        entry!(0x009d_a770, fn_009da770(Ptr, u32) -> Ptr),
        entry!(0x009d_a7c0, combat_procedure_switch_weapon_update(Ptr)),
        entry!(
            0x009d_a9b0,
            combat_procedure_switch_weapon_load_game(Ptr, u32)
        ),
        entry!(0x009d_a7a0, fn_009da7a0(Ptr)),
        entry!(0x009d_a760, fn_009da760(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod get_type_tests {
    use super::*;

    #[test]
    fn at_009da760_returns_five() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009d_a760, &args![0x1234u32]).u32(), 5);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009da7a0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x009c_a6c0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009d_a7a0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_20cc);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_20cc,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n025_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Every callee outside this file that the batch's translations call, and that of the
    /// destructor `009da7a0` (`009ca6c0`).
    const EXTERNAL: &[u32] = &[
        0x0040_1030,
        0x0044_ddc0,
        0x0048_39c0,
        0x004c_0bf0,
        0x0057_5610,
        0x0064_50c0,
        0x0072_6070,
        0x0086_48a0,
        0x0086_4980,
        0x0088_c650,
        0x0088_c790,
        0x008a_6840,
        0x008a_7570,
        0x008d_8520,
        0x0097_ff60,
        0x0098_1990,
        0x0099_6de0,
        0x0099_6ef0,
        0x009c_a680,
        0x009c_aae0,
        0x00ec_43fb,
        0x009c_a6c0,
    ];

    const VTABLE: u32 = 0x0200_0000;
    const SLOTS: u32 = 0x0300_0000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| ret(eax));
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn logged(e: &mut Engine, f: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        f(e);
        e.call_log.take().unwrap()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in EXTERNAL {
            stub(&mut e, addr, 0);
        }
        let slots: Vec<u32> = (0..0x200).map(|s| SLOTS + 4 * s).collect();
        for &slot in &slots {
            stub(&mut e, slot, 0);
        }
        e.put_vtable(VTABLE, &slots);
        e
    }

    #[test]
    fn constructor_and_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xcd; 0x18]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009d_a720, &args![this, 0x55u32]).u32(), this);
        });
        assert_eq!(calls(&log, PROCEDURE_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x0109_20cc);
        assert_eq!((e.mem.u32(this + 0x10), e.mem.u8(this + 0x14)), (0x55, 0));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009d_a770, &args![this, 1u32]).u32(), this);
            e.call(0x009d_a770, &args![this, 0u32]);
        });
        assert_eq!(calls(&log, 0x009c_a6c0).len(), 2);
        assert_eq!(calls(&log, 0x0040_1030), vec![vec![this]]);
    }

    #[test]
    fn update_queues_then_finishes() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 4, 0x700);
        e.mem.set_u32(this + 0x10, 0x55);
        let attacker = 0x800u32;
        stub(&mut e, GET_ATTACKER, attacker);
        stub(&mut e, 0x008a_7570, u32::MAX);
        let process = e.mem.alloc(8);
        e.mem.set_u32(process, VTABLE);
        stub(&mut e, ACTOR_PROCESS, process);
        // Fleeing: done at once.
        stub(&mut e, 0x0098_1990, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009d_a7c0, &args![this]);
        });
        assert_eq!(calls(&log, PROCEDURE_DONE), vec![vec![this]]);
        assert!(calls(&log, GET_ATTACKER).is_empty());
        stub(&mut e, 0x0098_1990, 0);
        // Busy animating: nothing.
        stub(&mut e, 0x008a_7570, 3);
        let log = logged(&mut e, |e| {
            e.call(0x009d_a7c0, &args![this]);
        });
        assert!(calls(&log, ACTOR_PROCESS).is_empty());
        stub(&mut e, 0x008a_7570, u32::MAX);
        // Another weapon equipped: queue the weapon with its count.
        stub(&mut e, 0x004c_0bf0, 1);
        stub(&mut e, 0x0057_5610, 3);
        let log = logged(&mut e, |e| {
            e.call(0x009d_a7c0, &args![this]);
        });
        assert_eq!(
            calls(&log, 0x0088_c650),
            vec![vec![attacker, 0x55, 3, 0, 1, 0, 1]]
        );
        assert_eq!(e.mem.u8(this + 0x14), 1);
        // Equipped now: power-attack deltas for an accepted weapon, then done.
        let entry = 0x900u32;
        stub(&mut e, SLOTS + 0x148, entry);
        stub(&mut e, 0x0044_ddc0, 0x55);
        stub(&mut e, 0x0064_50c0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009d_a7c0, &args![this]);
        });
        assert_eq!(calls(&log, ACTOR_SET_008A6840), vec![vec![attacker, 1]]);
        assert_eq!(calls(&log, 0x0097_ff60), vec![vec![0x700]]);
        assert_eq!(calls(&log, PROCEDURE_DONE), vec![vec![this]]);
        // Switching to no weapon: unequip the current one.
        e.mem.set_u8(this + 0x14, 0);
        e.mem.set_u32(this + 0x10, 0);
        stub(&mut e, 0x0072_6070, 0x77);
        let log = logged(&mut e, |e| {
            e.call(0x009d_a7c0, &args![this]);
        });
        assert_eq!(
            calls(&log, 0x0088_c790),
            vec![vec![attacker, 0x55, 0x77, 0, 0, 0, 1]]
        );
        // Slot 0x454 set while switching: `008a6840(0)` only.
        e.mem.set_u8(this + 0x14, 0);
        stub(&mut e, SLOTS + 0x454, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009d_a7c0, &args![this]);
        });
        assert_eq!(calls(&log, ACTOR_SET_008A6840), vec![vec![attacker, 0]]);
        assert_eq!(e.mem.u8(this + 0x14), 0);
    }

    #[test]
    fn load_game_resolves_the_weapon() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        stub(&mut e, 0x0086_48a0, 0x123);
        e.register(0x0048_39c0, |_, a| ret(a[0] + 1));
        e.register(0x00ec_43fb, |_, a| ret(a[0] + a[3]));
        let log = logged(&mut e, |e| {
            e.call(0x009d_a9b0, &args![this, 0x8000u32]);
        });
        assert_eq!(calls(&log, PROCEDURE_LOAD_GAME), vec![vec![this, 0x8000]]);
        assert_eq!(e.mem.u32(this + 0x10), 0x124 + TES_OBJECT_WEAP_TYPE);
        assert_eq!(calls(&log, 0x0086_4980), vec![vec![0x8000, this + 0x14, 1]]);
        assert_eq!(
            calls(&log, 0x00ec_43fb)[0][1..],
            [0, TES_FORM_TYPE, TES_OBJECT_WEAP_TYPE, 0]
        );
    }
}
