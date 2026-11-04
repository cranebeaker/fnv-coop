//! `fallout/ai/combat/combatprocedureattacklow.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CombatProcedureAttackLow` is the combat procedure of low-detail combat:
//! the attacker hits the target by chance (`fCombatLowTargetHitPercent`,
//! `fCombatLowFleeingTargetHitPercent`) with its weapon's damage per second
//! and uses up ammunition at the weapon's rate of fire.
//!
//! Conventions: `ECX` receivers of `__thiscall` callees are the first
//! argument word, stack arguments follow in the order the callee reads them
//! (every argument list comes from the disassembly). Floats pushed as
//! arguments are passed as their bit pattern.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `CombatProcedureAttackLow` (Xbox PDB), 0x20 bytes; the offsets are
    /// the ones the PC code uses.
    pub struct CombatProcedureAttackLow: 0x20 {
        /// `pCombatController` (Xbox PDB, `CombatProcedure`): `CombatController*`.
        0x04 pCombatController: Ptr,
        /// `eState` (Xbox PDB): 0 initializing, 1 attacking (the debug texts).
        0x10 eState: u32,
        /// `fWeaponDPS` (Xbox PDB).
        0x14 fWeaponDPS: f32,
        /// `fWeaponCondition` (Xbox PDB).
        0x18 fWeaponCondition: f32,
        /// `pWeapon` (Xbox PDB): `TESObjectWEAP*`.
        0x1C pWeapon: Ptr,
    }
}

/// `CombatProcedure::GetAttacker` (Xbox PDB).
const GET_ATTACKER: u32 = 0x0099_6de0;
/// `CombatProcedure::GetTarget` (Xbox PDB).
const GET_TARGET: u32 = 0x0099_6e00;
/// `CombatProcedure::SaveGame` and `LoadGame` (Xbox PDB).
const BASE_SAVE_GAME: u32 = 0x0099_6ed0;
const BASE_LOAD_GAME: u32 = 0x0099_6ef0;
/// `00586150` (`__thiscall` on the `CombatController`): the word at `+0x9c`
/// (the controller's combat state).
const GET_COMBAT_STATE: u32 = 0x0058_6150;
/// `TESObjectREFR::GetSpace` (Xbox PDB).
const GET_SPACE: u32 = 0x0057_5ca0;
/// `009caac0` (`__thiscall` on the procedure, one word): stores the reason
/// string at `+0xc` and sets the status at `+8` to 1.
const FAIL_PROCEDURE: u32 = 0x009c_aac0;
/// `0099a2d0` (`__thiscall` on the combat state): the weapon the procedure
/// attacks with.
const COMBAT_STATE_WEAPON: u32 = 0x0099_a2d0;
/// `TESObjectWEAP::GetCombatWeaponType` (Xbox PDB).
const GET_COMBAT_WEAPON_TYPE: u32 = 0x0052_2c80;
/// `0097ac90` (`__thiscall` on the combat state, one word): the float at
/// `+0x3c + 4 * type`, in `ST0`.
const COMBAT_STATE_WEAPON_DPS: u32 = 0x0097_ac90;
/// `0097c150` (`__thiscall` on the combat state, one word): the word at
/// `+0xc + 4 * type`.
const COMBAT_STATE_WEAPON_OF_TYPE: u32 = 0x0097_c150;
/// `0097f450` (`__thiscall` on the combat state, one byte): stores the byte
/// at `+0x77`.
const COMBAT_STATE_SET_BYTE_77: u32 = 0x0097_f450;
/// `TESObjectREFR::GetInventoryItem` (Xbox PDB): `__thiscall`, two words.
const GET_INVENTORY_ITEM: u32 = 0x0057_6260;
/// `ItemChange::GetItemHealth` (Xbox PDB): `__thiscall`, one word, `ST0`.
const GET_ITEM_HEALTH: u32 = 0x004b_cdb0;
/// `004459e0` (`__thiscall` on the item, one word): called with 1 on the
/// item `GetInventoryItem` gave (its result is kept in a local the function
/// never reads again).
const ITEM_RELEASE: u32 = 0x0044_59e0;
/// `Actor::IsFleeing` (Xbox PDB): `__thiscall`, one word, `AL`.
const IS_FLEEING: u32 = 0x008a_6650;
/// `00403e20` (`__thiscall` on a setting): the address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `fCombatLowFleeingTargetHitPercent` and `fCombatLowTargetHitPercent`
/// (the names their static initializers give them).
const SETTING_FLEEING_TARGET_HIT_PERCENT: u32 = 0x011c_e230;
const SETTING_TARGET_HIT_PERCENT: u32 = 0x011c_fbb4;
/// `RandomFloat` (engine map name of `00476b70`): `__cdecl`, two floats,
/// `ST0`.
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `operator new` (`00401000`, `__cdecl`, one word).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `HitData::HitData` (Xbox PDB): `__thiscall`, returns `this`.
const HIT_DATA_CONSTRUCT: u32 = 0x009b_4d90;
/// `008c1c30` (`__thiscall` on a local `NiPointer<HitData>`, one word): the
/// pointer's constructor from a raw pointer.
const HIT_POINTER_CONSTRUCT: u32 = 0x008c_1c30;
/// `NiPointer<HitData>::~NiPointer<HitData>` (Xbox PDB).
const HIT_POINTER_DESTRUCT: u32 = 0x008c_1c60;
/// `00559450` (`__thiscall`): the word at `+0` (the smart pointer's object).
const POINTER_GET: u32 = 0x0055_9450;
/// `HitData::InitializeHitData_ov5` (Xbox PDB): `__cdecl`, four words
/// (hit data, attacker, target, damage `float`).
const INITIALIZE_HIT_DATA: u32 = 0x009b_5720;
/// `Actor::HitMe` (Xbox PDB): `__thiscall`, two words.
const HIT_ME: u32 = 0x0089_a760;
/// `TESObjectWEAP::GetCurrentAmmo` (Xbox PDB): `__thiscall`, one word.
const GET_CURRENT_AMMO: u32 = 0x0052_5980;
/// `Actor::ShouldUseAmmo` (Xbox PDB): `__thiscall`, one word, `AL`.
const SHOULD_USE_AMMO: u32 = 0x008a_8dd0;
/// `Actor::GetItemCountinContainer` (Xbox PDB): `__thiscall`, one word.
const GET_ITEM_COUNT: u32 = 0x0057_5610;
/// `CombatFormulas::GetWeaponRateOfFire` (Xbox PDB): `__cdecl`, three
/// words, `ST0`.
const GET_WEAPON_RATE_OF_FIRE: u32 = 0x0064_5d00;
/// `_ftol2_sse` (`00ec62c0`): the value in `ST0` (a leading `f64`) truncated
/// to an integer.
const FTOL: u32 = 0x00ec_62c0;
/// `00647b70` (`__cdecl`, two words): the first if it is greater (signed),
/// else the second.
const INT_MAX: u32 = 0x0064_7b70;
/// `004a8f20` (`__cdecl`, two words): the first if it is less (signed),
/// else the second.
const INT_MIN: u32 = 0x004a_8f20;
/// The actor's vtable slot the update calls with the ammunition and the
/// count to use up (ten words after `this`).
const SLOT_REMOVE_ITEM: u32 = 0x17c;
/// `00408da0` (engine map: `MapMarkerData::GetLocationName`, a folded
/// body): `__thiscall` on the weapon's `+0x30`, the name string.
const GET_NAME: u32 = 0x0040_8da0;
/// `BGSSaveGameBuffer` writers: `00865e50` (bytes: pointer, size, 0) and
/// `BGSSaveGameBuffer::SaveFormID_ov2` (form, 0).
const SAVE_BYTES: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSLoadGameBuffer` readers: `00864980` (bytes: pointer, size) and
/// `BGSLoadGameBuffer::LoadFormID` (no stack words).
const LOAD_BYTES: u32 = 0x0086_4980;
const LOAD_FORM_ID: u32 = 0x0086_48a0;
/// `004839c0` (`__cdecl`, one word): the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `__cdecl`, five words) from `TESForm`
/// (`01183028`) to `TESObjectWEAP` (`01183998`), the type descriptors.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_WEAP: u32 = 0x0118_3998;

/// `"Target is in different space"`.
const TEXT_DIFFERENT_SPACE: u32 = 0x0109_0b30;
/// `"INITIALIZING"`, `"ATTACKING"`, `"Weapon: %s  DPS: %.2f"`, `"Unarmed"`.
const TEXT_INITIALIZING: u32 = 0x0109_0b68;
const TEXT_ATTACKING: u32 = 0x0109_0a98;
const TEXT_WEAPON_DPS: u32 = 0x0109_0b50;
const TEXT_UNARMED: u32 = 0x0106_76a0;
/// The debug-text object's print slot (`__cdecl` through the vtable).
const SLOT_PRINT: u32 = 0x14;

/// The combat weapon type of the weapon (6 when there is none).
fn weapon_type(e: &mut Engine, weapon: Ptr) -> u32 {
    if weapon.is_null() {
        6
    } else {
        e.call(GET_COMBAT_WEAPON_TYPE, &args![weapon]).u32()
    }
}

// Translated from 009cbb70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009cbb70 - the compiler's FS:[0] exception frame (unwinding of the hit data allocation and its NiPointer) is not translated
/// `CombatProcedureAttackLow::Update` (Xbox PDB): fails when the target is
/// missing or in another space. On the first update it takes the combat
/// state's weapon, its damage per second and (for a weapon) the inventory
/// item's health; with no inventory item it sets the combat state's byte
/// `+0x77` and stops. Then, while the weapon is still the combat state's
/// weapon of its type, it hits the target when a random number in [0, 1)
/// is below the hit chance setting (the fleeing one for a fleeing target),
/// and uses up as many rounds as the rate of fire says (at least one, at
/// most the count carried); when that empties the ammunition, or the weapon
/// changed, the byte `+0x77` is set and the state goes back to 0.
pub fn update(e: &mut Engine, this: Ptr<CombatProcedureAttackLow>) {
    let t = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    let controller = e.get(this, CombatProcedureAttackLow::pCombatController);
    let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
    let same_space = target != 0 && {
        let target_space = e.call(GET_SPACE, &args![target]).u32();
        let attacker_space = e.call(GET_SPACE, &args![attacker]).u32();
        target_space == attacker_space
    };
    if !same_space {
        e.call(FAIL_PROCEDURE, &args![t, TEXT_DIFFERENT_SPACE]);
        return;
    }

    if e.get(this, CombatProcedureAttackLow::eState) == 0 {
        let weapon = e
            .call(COMBAT_STATE_WEAPON, &args![combat_state])
            .ptr::<()>();
        e.set(this, CombatProcedureAttackLow::pWeapon, weapon);
        let kind = weapon_type(e, weapon);
        let dps = e
            .call(COMBAT_STATE_WEAPON_DPS, &args![combat_state, kind])
            .f64();
        e.mem.set_u32(t + 0x14, x87_store32(dps));
        let weapon = e.get(this, CombatProcedureAttackLow::pWeapon);
        if !weapon.is_null() {
            let item = e
                .call(GET_INVENTORY_ITEM, &args![attacker, weapon, 0u32])
                .u32();
            if item == 0 {
                e.call(COMBAT_STATE_SET_BYTE_77, &args![combat_state, 1u32]);
                return;
            }
            let health = e.call(GET_ITEM_HEALTH, &args![item, 1u32]).f64();
            e.mem.set_u32(t + 0x18, x87_store32(health));
            if item != 0 {
                e.call(ITEM_RELEASE, &args![item, 1u32]);
            }
        }
        e.set(this, CombatProcedureAttackLow::eState, 1);
    }

    let weapon = e.get(this, CombatProcedureAttackLow::pWeapon);
    let kind = weapon_type(e, weapon);
    let weapon = e.get(this, CombatProcedureAttackLow::pWeapon);
    if !weapon.is_null() {
        let current = e
            .call(COMBAT_STATE_WEAPON_OF_TYPE, &args![combat_state, kind])
            .u32();
        if e.get(this, CombatProcedureAttackLow::pWeapon).addr() != current {
            e.call(COMBAT_STATE_SET_BYTE_77, &args![combat_state, 1u32]);
            e.set(this, CombatProcedureAttackLow::eState, 0);
            return;
        }
    }

    let setting = if e.call(IS_FLEEING, &args![target, 0u32]).bool() {
        SETTING_FLEEING_TARGET_HIT_PERCENT
    } else {
        SETTING_TARGET_HIT_PERCENT
    };
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    let chance_bits = x87_quiet_bits(e.mem.u32(value));
    let random = e
        .call(RANDOM_FLOAT, &args![0.0f32.to_bits(), 1.0f32.to_bits()])
        .f64();
    // FLD chance; FCOMPP (random); TEST AH,0x41; JNZ: hits when chance > random.
    if x87_load32(chance_bits) > random {
        let block = e.call(OPERATOR_NEW, &args![100u32]).u32();
        let hit = if block == 0 {
            0
        } else {
            e.call(HIT_DATA_CONSTRUCT, &args![block]).u32()
        };
        e.with_stack(4, |e, pointer| {
            let pointer = pointer.addr();
            e.call(HIT_POINTER_CONSTRUCT, &args![pointer, hit]);
            let dps = x87_quiet_bits(e.mem.u32(t + 0x14));
            let raw = e.call(POINTER_GET, &args![pointer]).u32();
            e.call(INITIALIZE_HIT_DATA, &args![raw, attacker, target, dps]);
            let raw = e.call(POINTER_GET, &args![pointer]).u32();
            e.call(HIT_ME, &args![target, raw, 0u32]);
            e.call(HIT_POINTER_DESTRUCT, &args![pointer]);
        });
    }

    let weapon = e.get(this, CombatProcedureAttackLow::pWeapon);
    if weapon.is_null() {
        return;
    }
    if e.call(GET_CURRENT_AMMO, &args![weapon, attacker]).u32() == 0 {
        return;
    }
    let weapon = e.get(this, CombatProcedureAttackLow::pWeapon);
    if !e.call(SHOULD_USE_AMMO, &args![attacker, weapon]).bool() {
        return;
    }
    let weapon = e.get(this, CombatProcedureAttackLow::pWeapon);
    let ammo = e.call(GET_CURRENT_AMMO, &args![weapon, attacker]).u32();
    let count = e.call(GET_ITEM_COUNT, &args![attacker, ammo]).i32();
    let owner = if attacker == 0 { 0 } else { attacker + 0xa4 };
    let weapon = e.get(this, CombatProcedureAttackLow::pWeapon);
    let rate = e
        .call(GET_WEAPON_RATE_OF_FIRE, &args![owner, weapon, 0u32])
        .f64();
    let rounds = e.call(FTOL, &args![rate]).i32();
    let rounds = e.call(INT_MAX, &args![rounds, 1i32]).i32();
    let rounds = e.call(INT_MIN, &args![rounds, count]).i32();
    e.vcall(
        attacker,
        SLOT_REMOVE_ITEM,
        &args![ammo, 0u32, rounds, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
    if rounds == count {
        e.call(COMBAT_STATE_SET_BYTE_77, &args![combat_state, 1u32]);
        e.set(this, CombatProcedureAttackLow::eState, 0);
    }
}

// Translated from 009cbf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackLow::DisplayDebugText` (Xbox PDB): prints the
/// state through the text object's print slot (`+0x14`, `__cdecl`, the
/// object and `first`/`second` first), and while attacking the weapon's
/// name (`"Unarmed"` without one) and its damage per second.
pub fn display_debug_text(
    e: &mut Engine,
    this: Ptr<CombatProcedureAttackLow>,
    text: u32,
    first: u32,
    second: u32,
) {
    match e.get(this, CombatProcedureAttackLow::eState) {
        0 => {
            e.vcall(text, SLOT_PRINT, &args![first, second, TEXT_INITIALIZING]);
        }
        1 => {
            e.vcall(text, SLOT_PRINT, &args![first, second, TEXT_ATTACKING]);
            let weapon = e.get(this, CombatProcedureAttackLow::pWeapon);
            let name = if weapon.is_null() {
                TEXT_UNARMED
            } else {
                e.call(GET_NAME, &args![weapon.addr() + 0x30]).u32()
            };
            let dps = x87_load32(e.mem.u32(this.addr() + 0x14));
            e.vcall(
                text,
                SLOT_PRINT,
                &args![first, second, TEXT_WEAPON_DPS, name, dps],
            );
        }
        _ => {}
    }
}

// Translated from 009cbfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackLow::SaveGame` (Xbox PDB): the base class's
/// fields, then the state, damage per second, condition and the weapon's
/// form id.
pub fn save_game(e: &mut Engine, this: Ptr<CombatProcedureAttackLow>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_SAVE_GAME, &args![t, buffer]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x10, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x14, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x18, 4u32, 0u32]);
    let weapon = e.get(this, CombatProcedureAttackLow::pWeapon);
    e.call(SAVE_FORM_ID, &args![buffer, weapon, 0u32]);
}

// Translated from 009cc040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackLow::LoadGame` (Xbox PDB): the mirror of
/// `SaveGame`; the weapon is the form with the loaded id cast to
/// `TESObjectWEAP`.
pub fn load_game(e: &mut Engine, this: Ptr<CombatProcedureAttackLow>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_LOAD_GAME, &args![t, buffer]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x10, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x14, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x18, 4u32]);
    let form_id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
    let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
    let weapon = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_WEAP, 0u32],
        )
        .u32();
    e.mem.set_u32(t + 0x1c, weapon);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009c_bb70, update(Ptr<CombatProcedureAttackLow>)),
        entry!(
            0x009c_bf10,
            display_debug_text(Ptr<CombatProcedureAttackLow>, u32, u32, u32)
        ),
        entry!(0x009c_bfd0, save_game(Ptr<CombatProcedureAttackLow>, u32)),
        entry!(0x009c_c040, load_game(Ptr<CombatProcedureAttackLow>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// Every callee outside the file, answering 0.
    const CALLEES: &[u32] = &[
        GET_ATTACKER,
        GET_TARGET,
        BASE_SAVE_GAME,
        BASE_LOAD_GAME,
        GET_COMBAT_STATE,
        GET_SPACE,
        FAIL_PROCEDURE,
        COMBAT_STATE_WEAPON,
        GET_COMBAT_WEAPON_TYPE,
        COMBAT_STATE_WEAPON_DPS,
        COMBAT_STATE_WEAPON_OF_TYPE,
        COMBAT_STATE_SET_BYTE_77,
        GET_INVENTORY_ITEM,
        GET_ITEM_HEALTH,
        ITEM_RELEASE,
        IS_FLEEING,
        SETTING_VALUE,
        RANDOM_FLOAT,
        OPERATOR_NEW,
        HIT_DATA_CONSTRUCT,
        HIT_POINTER_CONSTRUCT,
        HIT_POINTER_DESTRUCT,
        POINTER_GET,
        INITIALIZE_HIT_DATA,
        HIT_ME,
        GET_CURRENT_AMMO,
        SHOULD_USE_AMMO,
        GET_ITEM_COUNT,
        GET_WEAPON_RATE_OF_FIRE,
        FTOL,
        INT_MAX,
        INT_MIN,
        GET_NAME,
        SAVE_BYTES,
        SAVE_FORM_ID,
        LOAD_BYTES,
        LOAD_FORM_ID,
        LOOKUP_FORM,
        RT_DYNAMIC_CAST,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in CALLEES {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    fn answer(e: &mut Engine, addr: u32, value: Ret) {
        e.register_double(addr, move |_, _| value);
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> Log {
        e.call_log = Some(vec![]);
        e.call(addr, words);
        e.call_log.take().unwrap()
    }

    /// An attacker with a vtable whose slot `0x17c` is a logging double.
    fn attacker(e: &mut Engine) -> u32 {
        let actor = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(actor, vtable);
        e.mem.set_u32(vtable + SLOT_REMOVE_ITEM, 0x7000_0100);
        e.register(0x7000_0100, |_, _| Ret::default());
        actor
    }

    #[test]
    fn update_fails_without_a_target_or_in_another_space() {
        let mut e = engine();
        let this = e.new_object::<CombatProcedureAttackLow>();
        let log = run(&mut e, 0x009c_bb70, &args![this]);
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![this.addr(), TEXT_DIFFERENT_SPACE]]
        );
        assert!(calls_to(&log, GET_SPACE).is_empty());

        answer(&mut e, GET_TARGET, ret(0x500));
        let mut space = 0;
        e.register_double(GET_SPACE, move |_, _| {
            space += 1;
            ret(space)
        });
        let log = run(&mut e, 0x009c_bb70, &args![this]);
        assert_eq!(calls_to(&log, FAIL_PROCEDURE).len(), 1);
        assert!(calls_to(&log, COMBAT_STATE_WEAPON).is_empty());
    }

    #[test]
    fn update_initializes_then_hits_and_uses_up_the_ammunition() {
        let mut e = engine();
        let this = e.new_object::<CombatProcedureAttackLow>();
        let actor = attacker(&mut e);
        answer(&mut e, GET_ATTACKER, ret(actor));
        answer(&mut e, GET_TARGET, ret(0x500));
        answer(&mut e, GET_COMBAT_STATE, ret(0x600));
        answer(&mut e, COMBAT_STATE_WEAPON, ret(0x700));
        answer(&mut e, GET_COMBAT_WEAPON_TYPE, ret(2));
        answer(&mut e, COMBAT_STATE_WEAPON_DPS, float(12.5));
        answer(&mut e, COMBAT_STATE_WEAPON_OF_TYPE, ret(0x700));
        answer(&mut e, GET_INVENTORY_ITEM, ret(0x800));
        answer(&mut e, GET_ITEM_HEALTH, float(0.75));
        let chance = e.mem.alloc(4);
        e.mem.set_f32(chance, 0.5);
        answer(&mut e, SETTING_VALUE, ret(chance));
        answer(&mut e, RANDOM_FLOAT, float(0.25));
        answer(&mut e, OPERATOR_NEW, ret(0x900));
        answer(&mut e, HIT_DATA_CONSTRUCT, ret(0x900));
        answer(&mut e, POINTER_GET, ret(0x900));
        answer(&mut e, GET_CURRENT_AMMO, ret(0xa00));
        answer(&mut e, SHOULD_USE_AMMO, ret(1));
        answer(&mut e, GET_ITEM_COUNT, ret(3));
        answer(&mut e, GET_WEAPON_RATE_OF_FIRE, float(7.9));
        answer(&mut e, FTOL, ret(7));
        answer(&mut e, INT_MAX, ret(7));
        answer(&mut e, INT_MIN, ret(3));
        let log = run(&mut e, 0x009c_bb70, &args![this]);

        assert_eq!(e.mem.f32(this.addr() + 0x14), 12.5);
        assert_eq!(e.mem.f32(this.addr() + 0x18), 0.75);
        assert_eq!(e.get(this, CombatProcedureAttackLow::pWeapon).addr(), 0x700);
        assert_eq!(
            calls_to(&log, COMBAT_STATE_WEAPON_DPS),
            vec![vec![0x600, 2]]
        );
        assert_eq!(
            calls_to(&log, GET_INVENTORY_ITEM),
            vec![vec![actor, 0x700, 0]]
        );
        assert_eq!(calls_to(&log, ITEM_RELEASE), vec![vec![0x800, 1]]);
        assert_eq!(
            calls_to(&log, SETTING_VALUE),
            vec![vec![SETTING_TARGET_HIT_PERCENT]]
        );
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT),
            vec![vec![0, 1.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, INITIALIZE_HIT_DATA),
            vec![vec![0x900, actor, 0x500, 12.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, HIT_ME), vec![vec![0x500, 0x900, 0]]);
        assert_eq!(calls_to(&log, HIT_POINTER_DESTRUCT).len(), 1);
        assert_eq!(
            calls_to(&log, GET_WEAPON_RATE_OF_FIRE),
            vec![vec![actor + 0xa4, 0x700, 0]]
        );
        assert_eq!(calls_to(&log, FTOL), vec![args![7.9f64]]);
        assert_eq!(calls_to(&log, INT_MAX), vec![vec![7, 1]]);
        assert_eq!(calls_to(&log, INT_MIN), vec![vec![7, 3]]);
        assert_eq!(
            calls_to(&log, 0x7000_0100),
            vec![vec![actor, 0xa00, 0, 3, 0, 0, 0, 0, 0, 1, 0]]
        );
        // The rounds used equal the count carried: back to state 0.
        assert_eq!(
            calls_to(&log, COMBAT_STATE_SET_BYTE_77),
            vec![vec![0x600, 1]]
        );
        assert_eq!(e.get(this, CombatProcedureAttackLow::eState), 0);
    }

    #[test]
    fn update_stops_without_an_inventory_item_and_misses_by_chance() {
        let mut e = engine();
        let this = e.new_object::<CombatProcedureAttackLow>();
        answer(&mut e, GET_TARGET, ret(0x500));
        answer(&mut e, GET_COMBAT_STATE, ret(0x600));
        answer(&mut e, COMBAT_STATE_WEAPON, ret(0x700));
        let log = run(&mut e, 0x009c_bb70, &args![this]);
        assert_eq!(
            calls_to(&log, COMBAT_STATE_SET_BYTE_77),
            vec![vec![0x600, 1]]
        );
        assert_eq!(e.get(this, CombatProcedureAttackLow::eState), 0);
        assert!(calls_to(&log, IS_FLEEING).is_empty());

        // Unarmed (no weapon), a fleeing target, the chance equal to the
        // random number: no hit, and nothing more without a weapon.
        let mut e = engine();
        let this = e.new_object::<CombatProcedureAttackLow>();
        answer(&mut e, GET_TARGET, ret(0x500));
        answer(&mut e, IS_FLEEING, ret(1));
        let chance = e.mem.alloc(4);
        e.mem.set_f32(chance, 0.5);
        answer(&mut e, SETTING_VALUE, ret(chance));
        answer(&mut e, RANDOM_FLOAT, float(0.5));
        let log = run(&mut e, 0x009c_bb70, &args![this]);
        assert_eq!(e.get(this, CombatProcedureAttackLow::eState), 1);
        assert_eq!(calls_to(&log, COMBAT_STATE_WEAPON_DPS), vec![vec![0, 6]]);
        assert_eq!(
            calls_to(&log, SETTING_VALUE),
            vec![vec![SETTING_FLEEING_TARGET_HIT_PERCENT]]
        );
        assert_eq!(calls_to(&log, IS_FLEEING), vec![vec![0x500, 0]]);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, GET_CURRENT_AMMO).is_empty());
    }

    #[test]
    fn update_resets_when_the_weapon_changed() {
        let mut e = engine();
        let this = e.new_object::<CombatProcedureAttackLow>();
        e.set(this, CombatProcedureAttackLow::eState, 1);
        e.set(this, CombatProcedureAttackLow::pWeapon, Ptr::new(0x700));
        answer(&mut e, GET_TARGET, ret(0x500));
        answer(&mut e, GET_COMBAT_STATE, ret(0x600));
        answer(&mut e, GET_COMBAT_WEAPON_TYPE, ret(3));
        answer(&mut e, COMBAT_STATE_WEAPON_OF_TYPE, ret(0x780));
        let log = run(&mut e, 0x009c_bb70, &args![this]);
        assert_eq!(
            calls_to(&log, COMBAT_STATE_WEAPON_OF_TYPE),
            vec![vec![0x600, 3]]
        );
        assert_eq!(
            calls_to(&log, COMBAT_STATE_SET_BYTE_77),
            vec![vec![0x600, 1]]
        );
        assert_eq!(e.get(this, CombatProcedureAttackLow::eState), 0);
        assert!(calls_to(&log, IS_FLEEING).is_empty());
    }

    #[test]
    fn display_debug_text_prints_the_state_and_the_weapon() {
        let mut e = engine();
        let this = e.new_object::<CombatProcedureAttackLow>();
        let text = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(text, vtable);
        e.mem.set_u32(vtable + SLOT_PRINT, 0x7000_0200);
        e.register(0x7000_0200, |_, _| Ret::default());
        let log = run(&mut e, 0x009c_bf10, &args![this, text, 5u32, 6u32]);
        assert_eq!(
            calls_to(&log, 0x7000_0200),
            vec![vec![text, 5, 6, TEXT_INITIALIZING]]
        );

        e.set(this, CombatProcedureAttackLow::eState, 1);
        e.mem.set_f32(this.addr() + 0x14, 2.5);
        let log = run(&mut e, 0x009c_bf10, &args![this, text, 5u32, 6u32]);
        let mut weapon_line = vec![text, 5, 6, TEXT_WEAPON_DPS, TEXT_UNARMED];
        weapon_line.extend(args![2.5f64]);
        assert_eq!(
            calls_to(&log, 0x7000_0200),
            vec![vec![text, 5, 6, TEXT_ATTACKING], weapon_line]
        );

        e.set(this, CombatProcedureAttackLow::pWeapon, Ptr::new(0x700));
        answer(&mut e, GET_NAME, ret(0x1234));
        let log = run(&mut e, 0x009c_bf10, &args![this, text, 5u32, 6u32]);
        assert_eq!(calls_to(&log, GET_NAME), vec![vec![0x730]]);
        assert_eq!(calls_to(&log, 0x7000_0200)[1][4], 0x1234);

        e.set(this, CombatProcedureAttackLow::eState, 2);
        let log = run(&mut e, 0x009c_bf10, &args![this, text, 5u32, 6u32]);
        assert!(calls_to(&log, 0x7000_0200).is_empty());
    }

    #[test]
    fn save_and_load_game_mirror_each_other() {
        let mut e = engine();
        let this = e.new_object::<CombatProcedureAttackLow>();
        let t = this.addr();
        e.set(this, CombatProcedureAttackLow::pWeapon, Ptr::new(0x700));
        let log = run(&mut e, 0x009c_bfd0, &args![this, 0x40u32]);
        let expected: Log = vec![
            (0x009c_bfd0, vec![t, 0x40]),
            (BASE_SAVE_GAME, vec![t, 0x40]),
            (SAVE_BYTES, vec![0x40, t + 0x10, 4, 0]),
            (SAVE_BYTES, vec![0x40, t + 0x14, 4, 0]),
            (SAVE_BYTES, vec![0x40, t + 0x18, 4, 0]),
            (SAVE_FORM_ID, vec![0x40, 0x700, 0]),
        ];
        assert_eq!(log, expected);

        answer(&mut e, LOAD_FORM_ID, ret(0x1234));
        answer(&mut e, LOOKUP_FORM, ret(0x880));
        answer(&mut e, RT_DYNAMIC_CAST, ret(0x888));
        let log = run(&mut e, 0x009c_c040, &args![this, 0x40u32]);
        let expected: Log = vec![
            (0x009c_c040, vec![t, 0x40]),
            (BASE_LOAD_GAME, vec![t, 0x40]),
            (LOAD_BYTES, vec![0x40, t + 0x10, 4]),
            (LOAD_BYTES, vec![0x40, t + 0x14, 4]),
            (LOAD_BYTES, vec![0x40, t + 0x18, 4]),
            (LOAD_FORM_ID, vec![0x40]),
            (LOOKUP_FORM, vec![0x1234]),
            (
                RT_DYNAMIC_CAST,
                vec![0x880, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_WEAP, 0],
            ),
        ];
        assert_eq!(log, expected);
        assert_eq!(e.get(this, CombatProcedureAttackLow::pWeapon).addr(), 0x888);
    }
}
