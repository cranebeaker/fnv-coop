//! `fallout/ai/combat/combatprocedureattackgrenade.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009cafc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00996ea0` (Ghidra name
/// `CombatProcedure::ClearLookAtTarget`), which clears the look-at target of the controller
/// at `+4` through `00981720`. The Xbox PDB name `CombatProcedureAttackGrenade::Shutdown` is
/// the class's vslot +0x00c (pdbdump `type ... CombatProcedureAttackGrenade`; engine map row
/// `vt 827db8b8`), and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00996ea0` ends right after
/// calling `00981720` (`RET 4`, a byte store only), so no result is computed (Ghidra types
/// it void).
pub fn combat_procedure_attack_grenade_shutdown(e: &mut Engine, this: Ptr) {
    e.call(0x0099_6ea0, &args![this]);
}

// Translated from 009cafa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `010909bc` (RTTI `.?AVCombatProcedureAttackGrenade@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009cafa0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_09bc);
    e.call(0x009c_a6c0, &args![this]);
}

// ---- Batch n023 ----------------------------------------------------------------------------

/// The `CombatProcedureAttackGrenade` vtable (RTTI name).
const GRENADE_VTABLE: u32 = 0x0109_09bc;
/// The `CombatProcedure` base constructor (`009ca680`, `combatprocedureactivateobject.cpp`).
const PROCEDURE_CONSTRUCT: u32 = 0x009c_a680;
/// The procedure's accessors and look-at calls (`combatprocedure.cpp`): `GetAttacker`,
/// `GetTarget`, `SetLookAtTarget`, `ClearLookAtTarget`, `SaveGame`, `LoadGame`.
const GET_ATTACKER: u32 = 0x0099_6de0;
const GET_TARGET: u32 = 0x0099_6e00;
const SET_LOOK_AT_TARGET: u32 = 0x0099_6e60;
const CLEAR_LOOK_AT_TARGET: u32 = 0x0099_6ea0;
const PROCEDURE_SAVE_GAME: u32 = 0x0099_6ed0;
const PROCEDURE_LOAD_GAME: u32 = 0x0099_6ef0;
/// The procedure fails with a text (`009caac0`) or is done (`009caae0`)
/// (`combatprocedureactivateobject.cpp`).
const PROCEDURE_FAIL: u32 = 0x009c_aac0;
const PROCEDURE_DONE: u32 = 0x009c_aae0;
/// `00992720` (`combatmanager.cpp`): whether an explosion may go ahead, on the combat manager
/// at `011f1958`.
const EXPLOSION_ALLOWED: u32 = 0x0099_2720;
const COMBAT_MANAGER_POINTER: u32 = 0x011f_1958;
/// The zero point (three `float`s at `011f426c`).
const ZERO_POINT: u32 = 0x011f_426c;
/// `00403e20` (`thiscall` on a setting): the address of its value; the settings at `011cfb78`
/// and `011cf8ec`.
const SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_011CFB78: u32 = 0x011c_fb78;
const SETTING_011CF8EC: u32 = 0x011c_f8ec;
/// The `float` `1.0` push of `CalculateTargetedPoint` is `FLD1`; the argument the projectile
/// arc checks get is the `float` at `01012054`.
const ARC_ARGUMENT: u32 = 0x0101_2054;
/// `00586150` (`thiscall` on the controller): its combat state; `007031c0` (`thiscall` on the
/// controller): its weapon; `00525a90` (`thiscall` on the weapon, `RET 4`: the attacker);
/// `004fd3c0` (`thiscall`): the explosion of that.
const CONTROLLER_STATE: u32 = 0x0058_6150;
const CONTROLLER_WEAPON: u32 = 0x0070_31c0;
const WEAPON_FOR: u32 = 0x0052_5a90;
const EXPLOSION_OF: u32 = 0x004f_d3c0;
/// The timer at `+0x34`: `00906f60` constructs it, `0097fa50`, `008d7f80` test it, `008d7f40`
/// starts it with a `float`, `00910b10` resets it; `009a5eb0` / `009a5f20` save and load it.
const TIMER_CONSTRUCT: u32 = 0x0090_6f60;
const TIMER_RUNNING: u32 = 0x0097_fa50;
const TIMER_TEST: u32 = 0x008d_7f80;
const TIMER_SET: u32 = 0x008d_7f40;
const TIMER_RESET: u32 = 0x0091_0b10;
/// `CombatDialogueManager::StartDialogue` (engine-map name of `00983970`; `thiscall`, `RET
/// 0x10`) on the object at `011f1708`.
const START_DIALOGUE: u32 = 0x0098_3970;
const DIALOGUE_MANAGER_POINTER: u32 = 0x011f_1708;
/// `"Missing either weapon, animation, or weapon projectile"`, `"IDLE"`, `"ATTACKING"`,
/// `"Targeted Point: %.02f, %.02f, %.02f"`, `"HIGH ARC"`, `"Explosive Projectile Blocked
/// Timer: %.02f/%.02f"`.
const MISSING_TEXT: u32 = 0x0109_0a00;
const IDLE_TEXT: u32 = 0x0101_c42c;
const ATTACKING_TEXT: u32 = 0x0109_0a98;
const POINT_FORMAT: u32 = 0x0109_0a74;
const HIGH_ARC_TEXT: u32 = 0x0109_0a68;
const BLOCKED_TIMER_FORMAT: u32 = 0x0109_0a38;
/// `00865e50` (save buffer: data, size, 0) and `00864980` (load buffer: data, size).
const SAVE_DATA: u32 = 0x0086_5e50;
const LOAD_DATA: u32 = 0x0086_4980;

/// Copies the three words at `from` to `to`.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for k in 0..3u32 {
        let word = e.mem.u32(from.wrapping_add(4 * k));
        e.mem.set_u32(to.wrapping_add(4 * k), word);
    }
}

/// The value of the setting at `setting`, as `FLD`/`FSTP` pass it.
fn setting_float(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(value))
}

/// The common tail of a grenade procedure's constructors: the vtable, `+0x10` = 0, the
/// "thrown at a point" byte `+0x14`, the point `+0x18` from `point` (the zero point when
/// `None`), the targeted point `+0x24` from the zero point, the "high arc" byte `+0x30` and
/// the timer at `+0x34` (`00906f60`).
fn grenade_fields(e: &mut Engine, this: Ptr, at_point: bool, point: u32, high_arc: u8) {
    let me = this.addr();
    e.mem.set_u32(me, GRENADE_VTABLE);
    e.mem.set_u32(me.wrapping_add(0x10), 0);
    e.mem.set_u8(me.wrapping_add(0x14), u8::from(at_point));
    copy_point(e, me.wrapping_add(0x18), point);
    copy_point(e, me.wrapping_add(0x24), ZERO_POINT);
    e.mem.set_u8(me.wrapping_add(0x30), high_arc);
    e.call(TIMER_CONSTRUCT, &args![this.byte_add(0x34)]);
}

// Translated from 009cadf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009cadf0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `CombatProcedureAttackGrenade::CombatProcedureAttackGrenade` (Xbox PDB; `ECX` only): the
/// base constructor (`009ca680`), the vtable `010909bc`, `+0x10` = 0, the byte `+0x14` = 0,
/// both points (`+0x18`, `+0x24`) the zero point (`011f426c`), the byte `+0x30` = 0 and the
/// timer at `+0x34` (`00906f60`). Returns `this`.
pub fn combat_procedure_attack_grenade_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PROCEDURE_CONSTRUCT, &args![this]);
    grenade_fields(e, this, false, ZERO_POINT, 0);
    this
}

// Translated from 009caee0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009caee0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// A `CombatProcedureAttackGrenade` constructor for a point (unnamed in the engine map;
/// `thiscall`, `RET 8`): as the plain one, but the byte `+0x14` = 1, the point `+0x18` copied
/// from `point` and the byte `+0x30` = `high_arc`. Returns `this`.
pub fn fn_009caee0(e: &mut Engine, this: Ptr, point: Ptr, high_arc: u8) -> Ptr {
    e.call(PROCEDURE_CONSTRUCT, &args![this]);
    grenade_fields(e, this, true, point.addr(), high_arc);
    this
}

// Translated from 009caeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatProcedureAttackGrenade` scalar deleting destructor (by its body; `thiscall`,
/// `RET 4`): the destructor `009cafa0`, then `00401030` frees `this` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_009caeb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009cafa0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 009cb440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the byte `value` at `+0x73` (of the
/// combat state the grenade procedure hands it).
pub fn fn_009cb440(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x73), value);
}

// Translated from 009cb380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackGrenade::CheckExplosion` (Xbox PDB; `thiscall`, `RET 0xc`): true when
/// the combat manager allows the explosion at `point` for the attacker (`00992720`, with the
/// controller's `0097ae90` and `time`). Otherwise the timer at `+0x34` starts with the setting
/// `011cf8ec` when it is not running (`0097fa50`), or, when it passes (`008d7f80`), the
/// controller's combat state gets `00442a80(explosion)`; a procedure thrown at a point
/// (`+0x14`) marks the state (`009cb440(1)`) and completes; false.
pub fn combat_procedure_attack_grenade_check_explosion(
    e: &mut Engine,
    this: Ptr,
    explosion: Ptr,
    point: Ptr,
    time: f32,
) -> bool {
    let me = this.addr();
    let controller = e.mem.u32(me.wrapping_add(4));
    let attacker = e.call(0x0097_ae90, &args![controller]).u32();
    let controller = e.mem.u32(me.wrapping_add(4));
    let state = e.call(CONTROLLER_STATE, &args![controller]).u32();
    let manager = e.mem.u32(COMBAT_MANAGER_POINTER);
    if e.call(
        EXPLOSION_ALLOWED,
        &args![
            manager,
            attacker,
            explosion,
            point,
            x87_quiet_bits(time.to_bits())
        ],
    )
    .bool()
    {
        return true;
    }
    let timer = me.wrapping_add(0x34);
    if !e.call(TIMER_RUNNING, &args![timer]).bool() {
        let delay = setting_float(e, SETTING_011CF8EC);
        e.call(TIMER_SET, &args![timer, delay]);
    } else if e.call(TIMER_TEST, &args![timer]).bool() {
        e.call(0x0044_2a80, &args![state, explosion]);
    }
    if e.mem.u8(me.wrapping_add(0x14)) != 0 {
        fn_009cb440(e, Ptr::new(state), 1);
        e.call(PROCEDURE_DONE, &args![this]);
    }
    false
}

// Translated from 009cb6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackGrenade::CalculateTargetedPoint` (Xbox PDB; `thiscall`, `RET 4`):
/// writes the point to aim at into `out` and returns `out`: the zero point when the procedure
/// is not thrown at a point (`+0x14`) and has no target; otherwise the projected point
/// (`009a8bf0` for the target, `009a8d80` for the point `+0x18`; both `cdecl`, with the
/// attacker, 0, the weapon's projectile (`00525a90`), its speed (`0069ef80`) times the setting
/// `011cfb78` for a high arc (`+0x30`) or 1, and the arc byte).
pub fn combat_procedure_attack_grenade_calculate_targeted_point(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
) -> Ptr {
    let me = this.addr();
    if e.mem.u8(me.wrapping_add(0x14)) == 0 && e.call(GET_TARGET, &args![this]).u32() == 0 {
        copy_point(e, out.addr(), ZERO_POINT);
        return out;
    }
    let controller = e.mem.u32(me.wrapping_add(4));
    let weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let projectile = e.call(WEAPON_FOR, &args![weapon, attacker]).u32();
    let factor = if e.mem.u8(me.wrapping_add(0x30)) != 0 {
        setting_float(e, SETTING_011CFB78)
    } else {
        x87_store32(1.0)
    };
    let speed = e.call(0x0069_ef80, &args![projectile]).f64();
    let speed = x87_store32(x87_mul(speed, x87_m32(factor)));
    // The frame: the second result +0, the first result +0xc, the point +0x18.
    e.with_stack(0x24, |e, frame| {
        let f = frame.addr();
        let point = f + 0x18;
        e.call(0x0068_15c0, &args![point]);
        let high_arc = u32::from(e.mem.u8(me.wrapping_add(0x30)));
        let result = if e.mem.u8(me.wrapping_add(0x14)) == 0 {
            let target = e.call(GET_TARGET, &args![this]).u32();
            let attacker = e.call(GET_ATTACKER, &args![this]).u32();
            e.call(
                0x009a_8bf0,
                &args![
                    f + 0xc,
                    attacker,
                    target,
                    0u32,
                    projectile,
                    x87_quiet_bits(speed),
                    high_arc
                ],
            )
            .u32()
        } else {
            let attacker = e.call(GET_ATTACKER, &args![this]).u32();
            e.call(
                0x009a_8d80,
                &args![
                    f,
                    attacker,
                    me.wrapping_add(0x18),
                    0u32,
                    projectile,
                    x87_quiet_bits(speed),
                    high_arc
                ],
            )
            .u32()
        };
        copy_point(e, point, result);
        copy_point(e, out.addr(), point);
    });
    out
}

// Translated from 009cb460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackGrenade::GetTargetedPoint` (Xbox PDB; `thiscall`, `RET 8`): whether a
/// throw from `from` reaches; on success `out` gets the targeted point (`+0x24`). A procedure
/// thrown at a point (`+0x14`) checks the arc to that point (`009a7670`: attacker, target,
/// the projectile, `from`, the point `+0x18` and the zero point by value, the arc byte
/// `+0x30`, `+0x24` and the `float` at `01012054`); otherwise `009a7580` checks the arc to the
/// target. With an explosion (`004fd3c0` of the projectile) the throw must also pass
/// `CheckExplosion` at the landing point (the point `+0x18`, or `+0x24` with the height of the
/// target's position, slot `+0x1f4`) after the flight time (`009a7e00`). A failed throw at a
/// point marks the combat state (`009cb440(1)`) and completes the procedure.
pub fn combat_procedure_attack_grenade_get_targeted_point(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    from: Ptr,
) -> bool {
    let me = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let target = e.call(GET_TARGET, &args![this]).u32();
    let controller = e.mem.u32(me.wrapping_add(4));
    let weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
    let projectile = e.call(WEAPON_FOR, &args![weapon, attacker]).u32();
    let explosion = e.call(EXPLOSION_OF, &args![projectile]).u32();
    let arc_argument = x87_quiet_bits(e.mem.u32(ARC_ARGUMENT));
    let f = from.addr();
    let from_words = [e.mem.u32(f), e.mem.u32(f + 4), e.mem.u32(f + 8)];
    let mut ok = false;
    if e.mem.u8(me.wrapping_add(0x14)) != 0 {
        let p = me.wrapping_add(0x18);
        let point = [e.mem.u32(p), e.mem.u32(p + 4), e.mem.u32(p + 8)];
        let zero = [
            e.mem.u32(ZERO_POINT),
            e.mem.u32(ZERO_POINT + 4),
            e.mem.u32(ZERO_POINT + 8),
        ];
        let mut words = vec![attacker, target, projectile];
        words.extend(from_words);
        words.extend(point);
        words.extend(zero);
        words.extend([me.wrapping_add(0x30), me.wrapping_add(0x24), arc_argument]);
        if e.call(0x009a_7670, &words).bool() {
            copy_point(e, out.addr(), me.wrapping_add(0x24));
            ok = true;
        }
    } else {
        let projectile_now = e.call(WEAPON_FOR, &args![weapon, attacker]).u32();
        let target_now = e.call(GET_TARGET, &args![this]).u32();
        let attacker_now = e.call(GET_ATTACKER, &args![this]).u32();
        if e.call(
            0x009a_7580,
            &args![
                attacker_now,
                target_now,
                projectile_now,
                from,
                me.wrapping_add(0x30),
                me.wrapping_add(0x24),
                arc_argument
            ],
        )
        .bool()
        {
            copy_point(e, out.addr(), me.wrapping_add(0x24));
            ok = true;
        }
    }
    if ok && explosion != 0 {
        ok = e.with_stack(0xc, |e, landing| {
            let l = landing.addr();
            e.call(0x0068_15c0, &args![l]);
            if e.mem.u8(me.wrapping_add(0x14)) != 0 {
                copy_point(e, l, me.wrapping_add(0x18));
            } else {
                copy_point(e, l, me.wrapping_add(0x24));
                let position = e.vcall(target, 0x1f4, &[]).u32();
                let height = x87_quiet_bits(e.mem.u32(position.wrapping_add(8)));
                e.mem.set_u32(l + 8, height);
            }
            let high_arc = u32::from(e.mem.u8(me.wrapping_add(0x30)));
            let mut words = vec![projectile];
            words.extend(from_words);
            words.extend([e.mem.u32(l), e.mem.u32(l + 4), e.mem.u32(l + 8)]);
            words.extend([high_arc, x87_store32(0.0)]);
            let time = x87_store32(e.call(0x009a_7e00, &words).f64());
            combat_procedure_attack_grenade_check_explosion(
                e,
                this,
                Ptr::new(explosion),
                landing,
                f32::from_bits(x87_quiet_bits(time)),
            )
        });
    }
    if !ok && e.mem.u8(me.wrapping_add(0x14)) != 0 {
        let controller = e.mem.u32(me.wrapping_add(4));
        let state = e.call(CONTROLLER_STATE, &args![controller]).u32();
        fn_009cb440(e, Ptr::new(state), 1);
        e.call(PROCEDURE_DONE, &args![this]);
    }
    ok
}

// Translated from 009cafe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackGrenade::Update` (Xbox PDB; `ECX` only). Fails with `"Missing either
/// weapon, animation, or weapon projectile"` without the controller's weapon (`007031c0`), the
/// attacker's animation (slot `+0x1e4`) or the weapon's projectile (`00525a90`). The attack
/// word `+0x10` becomes 1 when `00894900` passes the attacker, or 0 when its
/// `Actor::GetAnimAction` (`008a7570`) is -1; with it 0 and no point (`+0x14`) the arc byte
/// `+0x30` is cleared. When the combat state (`00586150`) cannot act (`0097ee90` false or
/// `008b3c10` true) an attack in progress clears the look-at target and the word. Otherwise:
/// dialogue (`00983970(.., 2, 0, 0)`), the targeted point `+0x24` (`CalculateTargetedPoint`),
/// the weapon's ranges (`009a9180`); a target farther than the longest range squared
/// (`00595c80` of the point minus the attacker's position, slot `+0x1f4`) clears the look-at
/// target and the word; otherwise a non-zero point (`00439090`) becomes the look-at target
/// (priority 4). While attacking (word 1) it waits unless the anim action is 4 and the
/// animation's `0070f490(4)` is at most 2. Then, when the attacker's process (`008d8520`)
/// passes its slot `+0x3f8` and the controller is not switching weapons (`00981970`): an
/// explosive (`004fd3c0`) must pass `CheckExplosion` at the point (`+0x18`, or the target's
/// position) with time 0; a throw not at a point must pass the arc check (`009a7580`); then
/// the state forgets the explosion (`00442a80(0)`), the timer resets, the process' slot
/// `+0x43c(1)` runs, the attack is queued (`Actor::QueueAttack(0x72)`), the word becomes 1,
/// and a dangerous explosion (`009a9720`) starts dialogue `(2, 4, 1)`.
pub fn combat_procedure_attack_grenade_update(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let target = e.call(GET_TARGET, &args![this]).u32();
    let controller = e.mem.u32(me.wrapping_add(4));
    let state = e.call(CONTROLLER_STATE, &args![controller]).u32();
    let controller = e.mem.u32(me.wrapping_add(4));
    let weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
    let animation = e.vcall(attacker, 0x1e4, &[]).u32();
    if weapon == 0 || animation == 0 || e.call(WEAPON_FOR, &args![weapon, attacker]).u32() == 0 {
        e.call(PROCEDURE_FAIL, &args![this, MISSING_TEXT]);
        return;
    }
    let attacking = me.wrapping_add(0x10);
    if e.call(0x0089_4900, &args![attacker]).bool() {
        e.mem.set_u32(attacking, 1);
    } else if e.call(0x008a_7570, &args![attacker]).i32() == -1 {
        e.mem.set_u32(attacking, 0);
    }
    if e.mem.u8(me.wrapping_add(0x14)) == 0 && e.mem.u32(attacking) == 0 {
        e.mem.set_u8(me.wrapping_add(0x30), 0);
    }
    if !e.call(0x0097_ee90, &args![state]).bool() || e.call(0x008b_3c10, &args![state]).bool() {
        if e.mem.u32(attacking) != 0 {
            e.call(CLEAR_LOOK_AT_TARGET, &args![this]);
            e.mem.set_u32(attacking, 0);
        }
        return;
    }
    let dialogue = e.mem.u32(DIALOGUE_MANAGER_POINTER);
    let controller = e.mem.u32(me.wrapping_add(4));
    e.call(
        START_DIALOGUE,
        &args![dialogue, controller, 2u32, 0u32, 0u32],
    );
    // The frame: the calculated point +0, the ranges +0xc (three floats), the difference
    // +0x18, the explosion point +0x24.
    e.with_stack(0x30, |e, frame| {
        let f = frame.addr();
        let point = combat_procedure_attack_grenade_calculate_targeted_point(e, this, Ptr::new(f));
        copy_point(e, me.wrapping_add(0x24), point.addr());
        let ranges = f + 0xc;
        for k in 0..3 {
            e.mem.set_u32(ranges + 4 * k, x87_store32(0.0));
        }
        let controller = e.mem.u32(me.wrapping_add(4));
        e.call(
            0x009a_9180,
            &args![controller, weapon, ranges, ranges + 4, ranges + 8],
        );
        let position = e.vcall(attacker, 0x1f4, &[]).u32();
        e.call(
            0x0043_9ef0,
            &args![me.wrapping_add(0x24), f + 0x18, position],
        );
        let distance = e.call(0x0059_5c80, &args![f + 0x18]).f64();
        let longest = e.mem.u32(ranges + 8);
        if x87_mul(x87_load32(longest), x87_m32(longest)) < distance {
            e.call(CLEAR_LOOK_AT_TARGET, &args![this]);
            e.mem.set_u32(attacking, 0);
            return;
        }
        if e.call(0x0043_9090, &args![me.wrapping_add(0x24), ZERO_POINT])
            .bool()
        {
            e.call(
                SET_LOOK_AT_TARGET,
                &args![this, me.wrapping_add(0x24), 4u32],
            );
        }
        if e.mem.u32(attacking) == 1 {
            if e.call(0x008a_7570, &args![attacker]).u32() != 4 {
                return;
            }
            if e.call(0x0070_f490, &args![animation, 4u32]).i32() > 2 {
                return;
            }
        }
        let process = e.call(0x008d_8520, &args![attacker]).u32();
        if !e.vcall(process, 0x3f8, &[]).bool() {
            return;
        }
        let controller = e.mem.u32(me.wrapping_add(4));
        if e.call(0x0098_1970, &args![controller]).bool() {
            return;
        }
        let projectile = e.call(WEAPON_FOR, &args![weapon, attacker]).u32();
        let explosion = e.call(EXPLOSION_OF, &args![projectile]).u32();
        if explosion != 0 {
            let at = if e.mem.u8(me.wrapping_add(0x14)) != 0 {
                me.wrapping_add(0x18)
            } else {
                e.vcall(target, 0x1f4, &[]).u32()
            };
            copy_point(e, f + 0x24, at);
            if !combat_procedure_attack_grenade_check_explosion(
                e,
                this,
                Ptr::new(explosion),
                Ptr::new(f + 0x24),
                f32::from_bits(x87_store32(0.0)),
            ) {
                return;
            }
        }
        let projectile = e.call(WEAPON_FOR, &args![weapon, attacker]).u32();
        if e.mem.u8(me.wrapping_add(0x14)) == 0 {
            let argument = x87_quiet_bits(e.mem.u32(ARC_ARGUMENT));
            if !e
                .call(
                    0x009a_7580,
                    &args![
                        attacker,
                        target,
                        projectile,
                        0u32,
                        me.wrapping_add(0x30),
                        me.wrapping_add(0x24),
                        argument
                    ],
                )
                .bool()
            {
                return;
            }
        }
        if explosion != 0 {
            e.call(0x0044_2a80, &args![state, 0u32]);
            e.call(TIMER_RESET, &args![me.wrapping_add(0x34)]);
        }
        let process = e.call(0x008d_8520, &args![attacker]).u32();
        e.vcall(process, 0x43c, &args![1u32]);
        e.call(0x0089_35f0, &args![attacker, 0x72u32]);
        e.mem.set_u32(attacking, 1);
        if explosion != 0 && e.call(0x009a_9720, &args![explosion]).bool() {
            let dialogue = e.mem.u32(DIALOGUE_MANAGER_POINTER);
            let controller = e.mem.u32(me.wrapping_add(4));
            e.call(
                START_DIALOGUE,
                &args![dialogue, controller, 2u32, 4u32, 1u32],
            );
        }
    });
}

// Translated from 009cb830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackGrenade::FiredProjectile` (Xbox PDB; `thiscall`, `RET 4`): a high-arc
/// throw (`+0x30`) gives the projectile `00966a00` with the setting `011cfb78`; the arc byte is
/// cleared; a throw at a point (`+0x14`) marks the combat state (`009cb440(1)`) and completes
/// the procedure.
pub fn combat_procedure_attack_grenade_fired_projectile(
    e: &mut Engine,
    this: Ptr,
    projectile: Ptr,
) {
    let me = this.addr();
    if e.mem.u8(me.wrapping_add(0x30)) != 0 {
        let value = setting_float(e, SETTING_011CFB78);
        e.call(0x0096_6a00, &args![projectile, value]);
    }
    e.mem.set_u8(me.wrapping_add(0x30), 0);
    if e.mem.u8(me.wrapping_add(0x14)) != 0 {
        let controller = e.mem.u32(me.wrapping_add(4));
        let state = e.call(CONTROLLER_STATE, &args![controller]).u32();
        fn_009cb440(e, Ptr::new(state), 1);
        e.call(PROCEDURE_DONE, &args![this]);
    }
}

// Translated from 009cb890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackGrenade::DisplayDebugText` (Xbox PDB; `thiscall`, `RET 0xc`): prints
/// through the printer's slot `+0x14` (`cdecl`: printer, the two words, text, ...) `"IDLE"` or
/// `"ATTACKING"` for the attack word 0 or 1, the targeted point (`+0x24`, as doubles),
/// `"HIGH ARC"` for a high arc, and while the timer at `+0x34` runs (`0097fa50`) its elapsed
/// (`00435e00`) and total (`006b9130`, read first) times.
pub fn combat_procedure_attack_grenade_display_debug_text(
    e: &mut Engine,
    this: Ptr,
    printer: Ptr,
    first: u32,
    second: u32,
) {
    let me = this.addr();
    let p = printer.addr();
    match e.mem.u32(me.wrapping_add(0x10)) {
        0 => {
            e.vcall(p, 0x14, &args![first, second, IDLE_TEXT]);
        }
        1 => {
            e.vcall(p, 0x14, &args![first, second, ATTACKING_TEXT]);
        }
        _ => {}
    }
    let x = x87_load32(e.mem.u32(me.wrapping_add(0x24)));
    let y = x87_load32(e.mem.u32(me.wrapping_add(0x28)));
    let z = x87_load32(e.mem.u32(me.wrapping_add(0x2c)));
    e.vcall(p, 0x14, &args![first, second, POINT_FORMAT, x, y, z]);
    if e.mem.u8(me.wrapping_add(0x30)) != 0 {
        e.vcall(p, 0x14, &args![first, second, HIGH_ARC_TEXT]);
    }
    let timer = me.wrapping_add(0x34);
    if e.call(TIMER_RUNNING, &args![timer]).bool() {
        let total = e.call(0x006b_9130, &args![timer]).f64();
        let elapsed = e.call(0x0043_5e00, &args![timer]).f64();
        e.vcall(
            p,
            0x14,
            &args![first, second, BLOCKED_TIMER_FORMAT, elapsed, total],
        );
    }
}

// Translated from 009cb9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackGrenade::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): the procedure's
/// own save (`00996ed0`), then the attack word (4 bytes), the point byte (1), both points
/// (12 each), the arc byte (1) (`00865e50`) and the timer (`009a5eb0`).
pub fn combat_procedure_attack_grenade_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let me = this.addr();
    e.call(PROCEDURE_SAVE_GAME, &args![this, buffer]);
    for (offset, size) in [
        (0x10u32, 4u32),
        (0x14, 1),
        (0x18, 0xc),
        (0x24, 0xc),
        (0x30, 1),
    ] {
        e.call(
            SAVE_DATA,
            &args![buffer, me.wrapping_add(offset), size, 0u32],
        );
    }
    e.call(0x009a_5eb0, &args![me.wrapping_add(0x34), buffer]);
}

// Translated from 009cba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackGrenade::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): the procedure's
/// own load (`00996ef0`), then the same fields as `SaveGame` (`00864980`) and the timer
/// (`009a5f20`).
pub fn combat_procedure_attack_grenade_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let me = this.addr();
    e.call(PROCEDURE_LOAD_GAME, &args![this, buffer]);
    for (offset, size) in [
        (0x10u32, 4u32),
        (0x14, 1),
        (0x18, 0xc),
        (0x24, 0xc),
        (0x30, 1),
    ] {
        e.call(LOAD_DATA, &args![buffer, me.wrapping_add(offset), size]);
    }
    e.call(0x009a_5f20, &args![me.wrapping_add(0x34), buffer]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009c_afa0, fn_009cafa0(Ptr)),
        entry!(0x009c_afc0, combat_procedure_attack_grenade_shutdown(Ptr)),
        entry!(
            0x009c_adf0,
            combat_procedure_attack_grenade_construct(Ptr) -> Ptr
        ),
        entry!(0x009c_aee0, fn_009caee0(Ptr, Ptr, u8) -> Ptr),
        entry!(0x009c_aeb0, fn_009caeb0(Ptr, u32) -> Ptr),
        entry!(0x009c_b440, fn_009cb440(Ptr, u8)),
        entry!(
            0x009c_b380,
            combat_procedure_attack_grenade_check_explosion(Ptr, Ptr, Ptr, f32) -> bool
        ),
        entry!(
            0x009c_b6e0,
            combat_procedure_attack_grenade_calculate_targeted_point(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x009c_b460,
            combat_procedure_attack_grenade_get_targeted_point(Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x009c_afe0, combat_procedure_attack_grenade_update(Ptr)),
        entry!(
            0x009c_b830,
            combat_procedure_attack_grenade_fired_projectile(Ptr, Ptr)
        ),
        entry!(
            0x009c_b890,
            combat_procedure_attack_grenade_display_debug_text(Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x009c_b9c0,
            combat_procedure_attack_grenade_save_game(Ptr, Ptr)
        ),
        entry!(
            0x009c_ba50,
            combat_procedure_attack_grenade_load_game(Ptr, Ptr)
        ),
    ]
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
    fn fwd_a03_009cafc0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0099_6ea0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009c_afc0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0099_6ea0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009cafa0_stores_the_vtable_then_forwards_this() {
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
        e.call(0x009c_afa0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_09bc);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_09bc,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n023_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    const VTABLE: u32 = 0x0e00_0000;

    const fn slot(offset: u32) -> u32 {
        0x0f00_0000 + offset
    }

    fn object(e: &mut Engine, words: &[u32]) -> u32 {
        if !e.mem.is_mapped(VTABLE) {
            let slots: Vec<u32> = (0..0x140).map(|i| slot(4 * i)).collect();
            e.put_vtable(VTABLE, &slots);
        }
        let o = e.mem.alloc(4 + 4 * words.len() as u32);
        e.mem.set_u32(o, VTABLE);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(o + 4 + 4 * i as u32, *w);
        }
        o
    }

    fn point(e: &mut Engine, p: [f32; 3]) -> u32 {
        let a = e.mem.alloc(12);
        for (i, v) in p.iter().enumerate() {
            e.mem.set_f32(a + 4 * i as u32, *v);
        }
        a
    }

    fn floats(e: &Engine, a: u32) -> [f32; 3] {
        [e.mem.f32(a), e.mem.f32(a + 4), e.mem.f32(a + 8)]
    }

    fn zero_point(e: &mut Engine) {
        e.map(ZERO_POINT & !0xfff, 0x1000);
        e.mem.set_f32(ZERO_POINT, 0.25);
        e.mem.set_f32(ZERO_POINT + 4, 0.5);
        e.mem.set_f32(ZERO_POINT + 8, 0.75);
    }

    #[test]
    fn the_constructor_starts_from_the_zero_point() {
        let mut e = Engine::new();
        zero_point(&mut e);
        noop(&mut e, &[TIMER_CONSTRUCT]);
        let p = e.mem.alloc(0x3c);
        e.mem.write(p, &[0xcd; 0x3c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009c_adf0, &args![p]).u32(), p);
        assert_eq!(calls(&e, PROCEDURE_CONSTRUCT), vec![vec![p]]);
        assert_eq!(e.mem.u32(p), GRENADE_VTABLE);
        assert_eq!(e.mem.u32(p + 4), 0);
        assert_eq!((e.mem.u32(p + 0x10), e.mem.u8(p + 0x14)), (0, 0));
        assert_eq!(floats(&e, p + 0x18), [0.25, 0.5, 0.75]);
        assert_eq!(floats(&e, p + 0x24), [0.25, 0.5, 0.75]);
        assert_eq!(e.mem.u8(p + 0x30), 0);
        assert_eq!(calls(&e, TIMER_CONSTRUCT), vec![vec![p + 0x34]]);
    }

    #[test]
    fn the_point_constructor_keeps_the_point_and_the_arc() {
        let mut e = Engine::new();
        zero_point(&mut e);
        noop(&mut e, &[TIMER_CONSTRUCT]);
        let p = e.mem.alloc(0x3c);
        let at = point(&mut e, [1.0, 2.0, 3.0]);
        assert_eq!(e.call(0x009c_aee0, &args![p, at, 0x101u32]).u32(), p);
        assert_eq!(e.mem.u8(p + 0x14), 1);
        assert_eq!(floats(&e, p + 0x18), [1.0, 2.0, 3.0]);
        assert_eq!(floats(&e, p + 0x24), [0.25, 0.5, 0.75]);
        assert_eq!(e.mem.u8(p + 0x30), 1);
    }

    #[test]
    fn the_deleting_destructor_frees_on_bit_0() {
        let mut e = Engine::new();
        noop(&mut e, &[0x0040_1030]);
        let p = e.mem.alloc(0x3c);
        for (flags, frees) in [(0u32, 0usize), (1, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009c_aeb0, &args![p, flags]).u32(), p);
            assert_eq!(calls(&e, 0x009c_a6c0), vec![vec![p]]);
            assert_eq!(calls(&e, 0x0040_1030).len(), frees);
        }
    }

    #[test]
    fn the_state_byte_at_0x73() {
        let mut e = Engine::new();
        let s = e.mem.alloc(0x74);
        e.call(0x009c_b440, &args![s, 0x1ffu32]);
        assert_eq!(e.mem.u8(s + 0x73), 0xff);
    }

    /// A throw: the procedure (controller +4 = 0x6100), the attacker (animation word +4,
    /// position word +8), the target (position word +8), the combat state, the weapon and its
    /// projectile (explosion word +0).
    struct Throw {
        procedure: u32,
        attacker: u32,
        target: u32,
        state: u32,
        weapon: u32,
        projectile: u32,
    }

    fn throw(e: &mut Engine, at_point: bool, explosion: u32) -> Throw {
        zero_point(e);
        let animation = e.mem.alloc(4);
        let here = point(e, [0.0, 0.0, 0.0]);
        let attacker = object(e, &[animation, here]);
        let there = point(e, [3.0, 4.0, 5.0]);
        let target = object(e, &[0, there]);
        let state = e.mem.alloc(0x80);
        let weapon = e.mem.alloc(4);
        let projectile = e.mem.alloc(4);
        e.mem.set_u32(projectile, explosion);
        let procedure = e.mem.alloc(0x40);
        e.mem.set_u32(procedure + 4, 0x6100);
        e.mem.set_u8(procedure + 0x14, u8::from(at_point));
        for k in 0..3 {
            e.mem.set_f32(procedure + 0x18 + 4 * k, 7.0 + k as f32);
            e.mem.set_f32(procedure + 0x24 + 4 * k, 0.0);
        }
        e.register_double(GET_ATTACKER, move |_, _| attacker.into_ret());
        e.register_double(GET_TARGET, move |_, _| target.into_ret());
        e.register_double(CONTROLLER_STATE, move |_, _| state.into_ret());
        e.register_double(CONTROLLER_WEAPON, move |_, _| weapon.into_ret());
        e.register_double(WEAPON_FOR, move |_, a| {
            assert_eq!(a[1], attacker);
            projectile.into_ret()
        });
        e.register(EXPLOSION_OF, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(slot(0x1e4), |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(slot(0x1f4), |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        for setting in [SETTING_011CFB78, SETTING_011CF8EC] {
            e.map(setting & !0xfff, 0x1000);
        }
        e.mem.set_f32(SETTING_011CFB78 + 4, 2.0);
        e.mem.set_f32(SETTING_011CF8EC + 4, 9.0);
        e.map(ARC_ARGUMENT & !0xfff, 0x1000);
        e.mem.set_f32(ARC_ARGUMENT, 0.125);
        e.map(COMBAT_MANAGER_POINTER & !0xfff, 0x1000);
        e.set_global(COMBAT_MANAGER_POINTER, 0x7100u32);
        e.map(DIALOGUE_MANAGER_POINTER & !0xfff, 0x1000);
        e.set_global(DIALOGUE_MANAGER_POINTER, 0x7200u32);
        Throw {
            procedure,
            attacker,
            target,
            state,
            weapon,
            projectile,
        }
    }

    #[test]
    fn check_explosion_asks_the_manager_then_waits_on_the_timer() {
        let mut e = Engine::new();
        let t = throw(&mut e, false, 0x5500);
        e.register(0x0097_ae90, |_, _| 0x8800u32.into_ret());
        e.register(EXPLOSION_ALLOWED, |_, _| 1u32.into_ret());
        noop(&mut e, &[TIMER_SET, 0x0044_2a80]);
        let at = point(&mut e, [1.0, 1.0, 1.0]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x009c_b380,
                &args![t.procedure, 0x5500u32, at, 0x7f80_0001u32]
            )
            .bool());
        assert_eq!(
            calls(&e, EXPLOSION_ALLOWED),
            vec![vec![0x7100, 0x8800, 0x5500, at, 0x7fc0_0001]]
        );
        // Refused, the timer not running: it starts with the setting.
        e.register(EXPLOSION_ALLOWED, |_, _| Ret::default());
        e.register(TIMER_RUNNING, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x009c_b380, &args![t.procedure, 0x5500u32, at, 0u32])
            .bool());
        assert_eq!(
            calls(&e, TIMER_SET),
            vec![vec![t.procedure + 0x34, 9.0f32.to_bits()]]
        );
        // Running and passed: the state hears of the explosion.
        e.register(TIMER_RUNNING, |_, _| 1u32.into_ret());
        e.register(TIMER_TEST, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009c_b380, &args![t.procedure, 0x5500u32, at, 0u32]);
        assert_eq!(calls(&e, 0x0044_2a80), vec![vec![t.state, 0x5500]]);
        assert_eq!(e.mem.u32(t.procedure + 8), 0);
        // Thrown at a point: refused means done.
        let t = throw(&mut e, true, 0x5500);
        e.call(0x009c_b380, &args![t.procedure, 0x5500u32, at, 0u32]);
        assert_eq!(e.mem.u8(t.state + 0x73), 1);
        assert_eq!(e.mem.u32(t.procedure + 8), 2);
    }

    #[test]
    fn the_targeted_point_is_projected_for_the_target_or_the_point() {
        let mut e = Engine::new();
        let t = throw(&mut e, false, 0);
        e.register(0x0069_ef80, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        let projected = point(&mut e, [1.5, 2.5, 3.5]);
        e.register_double(0x009a_8bf0, move |_, _| projected.into_ret());
        e.register_double(0x009a_8d80, move |_, _| projected.into_ret());
        noop(&mut e, &[NODE_ITEM_CONSTRUCT]);
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009c_b6e0, &args![t.procedure, out]).u32(), out);
        assert_eq!(floats(&e, out), [1.5, 2.5, 3.5]);
        let projection = calls(&e, 0x009a_8bf0);
        assert_eq!(
            projection[0][1..],
            [t.attacker, t.target, 0, t.projectile, 10.0f32.to_bits(), 0]
        );
        // A high arc scales the speed by the setting and passes the byte.
        e.mem.set_u8(t.procedure + 0x30, 1);
        e.call_log = Some(vec![]);
        e.call(0x009c_b6e0, &args![t.procedure, out]);
        assert_eq!(calls(&e, 0x009a_8bf0)[0][5..], [20.0f32.to_bits(), 1]);
        // At a point: the point's own projection.
        e.mem.set_u8(t.procedure + 0x14, 1);
        e.call_log = Some(vec![]);
        e.call(0x009c_b6e0, &args![t.procedure, out]);
        assert_eq!(
            calls(&e, 0x009a_8d80)[0][1..],
            [
                t.attacker,
                t.procedure + 0x18,
                0,
                t.projectile,
                20.0f32.to_bits(),
                1
            ]
        );
        // No point and no target: the zero point.
        e.mem.set_u8(t.procedure + 0x14, 0);
        e.register(GET_TARGET, |_, _| Ret::default());
        e.call(0x009c_b6e0, &args![t.procedure, out]);
        assert_eq!(floats(&e, out), [0.25, 0.5, 0.75]);
    }

    /// `006815c0`: the point constructor the exe runs on a stack point (here a no-op).
    const NODE_ITEM_CONSTRUCT: u32 = 0x0068_15c0;

    #[test]
    fn a_targeted_point_needs_the_arc_and_a_safe_explosion() {
        let mut e = Engine::new();
        let t = throw(&mut e, true, 0x5500);
        noop(&mut e, &[NODE_ITEM_CONSTRUCT]);
        e.register(0x009a_7670, |e, a| {
            for k in 0..3 {
                e.mem.set_f32(a[13] + 4 * k, 4.0);
            }
            1u32.into_ret()
        });
        e.register(0x009a_7e00, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.register(0x0097_ae90, |_, _| 0x8800u32.into_ret());
        e.register(EXPLOSION_ALLOWED, |_, _| 1u32.into_ret());
        let from = point(&mut e, [1.0, 2.0, 3.0]);
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert!(e.call(0x009c_b460, &args![t.procedure, out, from]).bool());
        assert_eq!(floats(&e, out), [4.0, 4.0, 4.0]);
        let arc = calls(&e, 0x009a_7670);
        let f = |v: f32| v.to_bits();
        assert_eq!(
            arc[0],
            vec![
                t.attacker,
                t.target,
                t.projectile,
                f(1.0),
                f(2.0),
                f(3.0),
                f(7.0),
                f(8.0),
                f(9.0),
                f(0.25),
                f(0.5),
                f(0.75),
                t.procedure + 0x30,
                t.procedure + 0x24,
                f(0.125)
            ]
        );
        assert_eq!(
            calls(&e, 0x009a_7e00),
            vec![vec![
                t.projectile,
                f(1.0),
                f(2.0),
                f(3.0),
                f(7.0),
                f(8.0),
                f(9.0),
                0,
                0
            ]]
        );
        let allowed = calls(&e, EXPLOSION_ALLOWED);
        assert_eq!(allowed[0][2], 0x5500);
        assert_eq!(allowed[0][4], f(1.5));
        // The explosion refused: the throw fails and, at a point, completes.
        e.register(EXPLOSION_ALLOWED, |_, _| Ret::default());
        noop(&mut e, &[TIMER_RUNNING, TIMER_SET]);
        assert!(!e.call(0x009c_b460, &args![t.procedure, out, from]).bool());
        assert_eq!(e.mem.u8(t.state + 0x73), 1);
        assert_eq!(e.mem.u32(t.procedure + 8), 2);
        // Not at a point: the plain arc check; its failure does not complete.
        let t = throw(&mut e, false, 0);
        e.register(0x009a_7580, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x009c_b460, &args![t.procedure, out, from]).bool());
        assert_eq!(
            calls(&e, 0x009a_7580),
            vec![vec![
                t.attacker,
                t.target,
                t.projectile,
                from,
                t.procedure + 0x30,
                t.procedure + 0x24,
                f(0.125)
            ]]
        );
        assert_eq!(e.mem.u32(t.procedure + 8), 0);
        e.register(0x009a_7580, |_, _| 1u32.into_ret());
        assert!(e.call(0x009c_b460, &args![t.procedure, out, from]).bool());
    }

    /// The doubles of a full `Update`: the attack checks pass, the ranges are (1, 2, 10).
    fn attack_doubles(e: &mut Engine) {
        noop(
            e,
            &[
                START_DIALOGUE,
                SET_LOOK_AT_TARGET,
                CLEAR_LOOK_AT_TARGET,
                NODE_ITEM_CONSTRUCT,
                0x0044_2a80,
                TIMER_RESET,
                slot(0x43c),
                0x0089_35f0,
            ],
        );
        e.register(0x0089_4900, |_, _| Ret::default());
        e.register(0x008a_7570, |_, _| 0u32.into_ret());
        e.register(0x0097_ee90, |_, _| 1u32.into_ret());
        e.register(0x008b_3c10, |_, _| Ret::default());
        e.register(0x0069_ef80, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        e.register(0x009a_8bf0, |e, a| {
            for k in 0..3 {
                e.mem.set_f32(a[0] + 4 * k, 3.0 + k as f32);
            }
            a[0].into_ret()
        });
        e.register(0x009a_9180, |e, a| {
            e.mem.set_f32(a[2], 1.0);
            e.mem.set_f32(a[3], 2.0);
            e.mem.set_f32(a[4], 10.0);
            Ret::default()
        });
        e.register(0x0043_9ef0, |e, a| {
            for k in 0..3 {
                let d = e.mem.f32(a[0] + 4 * k) - e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, d);
            }
            Ret::default()
        });
        e.register(0x0059_5c80, |e, a| {
            let s: f32 = (0..3).map(|k| e.mem.f32(a[0] + 4 * k).powi(2)).sum();
            Ret {
                st0: f64::from(s),
                ..Ret::default()
            }
        });
        e.register(0x0043_9090, |_, _| 1u32.into_ret());
        e.register(0x008d_8520, |_, _| 0x9100u32.into_ret());
        e.mem.map(0x9000, 0x1000);
        e.mem.set_u32(0x9100, VTABLE);
        e.register(slot(0x3f8), |_, _| 1u32.into_ret());
        e.register(0x0098_1970, |_, _| Ret::default());
        e.register(0x009a_7580, |_, _| 1u32.into_ret());
        e.register(0x009a_9720, |_, _| 1u32.into_ret());
    }

    #[test]
    fn update_fails_without_weapon_animation_or_projectile() {
        let mut e = Engine::new();
        let t = throw(&mut e, false, 0);
        e.register(CONTROLLER_WEAPON, |_, _| Ret::default());
        e.call(0x009c_afe0, &args![t.procedure]);
        assert_eq!(
            (e.mem.u32(t.procedure + 8), e.mem.u32(t.procedure + 0xc)),
            (1, MISSING_TEXT)
        );
        let t = throw(&mut e, false, 0);
        e.mem.set_u32(t.attacker + 4, 0);
        e.call(0x009c_afe0, &args![t.procedure]);
        assert_eq!(e.mem.u32(t.procedure + 8), 1);
        let t = throw(&mut e, false, 0);
        e.register(WEAPON_FOR, |_, _| Ret::default());
        e.call(0x009c_afe0, &args![t.procedure]);
        assert_eq!(e.mem.u32(t.procedure + 8), 1);
    }

    #[test]
    fn update_stops_attacking_when_the_state_cannot_act_or_the_target_is_far() {
        let mut e = Engine::new();
        let t = throw(&mut e, false, 0);
        attack_doubles(&mut e);
        // Attacking (00894900) but the state cannot act: the look-at goes.
        e.register(0x0089_4900, |_, _| 1u32.into_ret());
        e.register(0x0097_ee90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009c_afe0, &args![t.procedure]);
        assert_eq!(calls(&e, CLEAR_LOOK_AT_TARGET), vec![vec![t.procedure]]);
        assert_eq!(e.mem.u32(t.procedure + 0x10), 0);
        assert!(calls(&e, START_DIALOGUE).is_empty());
        // Idle (GetAnimAction -1) and not at a point: the arc byte clears.
        e.mem.set_u8(t.procedure + 0x30, 1);
        e.register(0x0089_4900, |_, _| Ret::default());
        e.register(0x008a_7570, |_, _| 0xffff_ffffu32.into_ret());
        e.call(0x009c_afe0, &args![t.procedure]);
        assert_eq!(e.mem.u8(t.procedure + 0x30), 0);
        // The target beyond the longest range (10): the look-at goes too.
        e.register(0x0097_ee90, |_, _| 1u32.into_ret());
        e.register(0x0059_5c80, |_, _| Ret {
            st0: 100.5,
            ..Ret::default()
        });
        e.mem.set_u32(t.procedure + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x009c_afe0, &args![t.procedure]);
        assert_eq!(
            calls(&e, START_DIALOGUE),
            vec![vec![0x7200, 0x6100, 2, 0, 0]]
        );
        assert_eq!(floats(&e, t.procedure + 0x24), [3.0, 4.0, 5.0]);
        assert_eq!(calls(&e, CLEAR_LOOK_AT_TARGET).len(), 1);
        assert_eq!(e.mem.u32(t.procedure + 0x10), 0);
        assert!(calls(&e, SET_LOOK_AT_TARGET).is_empty());
    }

    #[test]
    fn update_throws_at_the_targeted_point() {
        let mut e = Engine::new();
        let t = throw(&mut e, false, 0x5500);
        attack_doubles(&mut e);
        e.register(0x0097_ae90, |_, _| 0x8800u32.into_ret());
        e.register(EXPLOSION_ALLOWED, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009c_afe0, &args![t.procedure]);
        assert_eq!(
            calls(&e, SET_LOOK_AT_TARGET),
            vec![vec![t.procedure, t.procedure + 0x24, 4]]
        );
        assert_eq!(calls(&e, 0x009a_9180)[0][..2], [0x6100, t.weapon]);
        // The explosion is checked at the target's position with time 0.
        let allowed = calls(&e, EXPLOSION_ALLOWED);
        assert_eq!(allowed[0][2], 0x5500);
        assert_eq!(floats(&e, allowed[0][3]), [3.0, 4.0, 5.0]);
        assert_eq!(allowed[0][4], 0);
        assert_eq!(
            calls(&e, 0x009a_7580)[0],
            vec![
                t.attacker,
                t.target,
                t.projectile,
                0,
                t.procedure + 0x30,
                t.procedure + 0x24,
                0.125f32.to_bits()
            ]
        );
        assert_eq!(calls(&e, 0x0044_2a80), vec![vec![t.state, 0]]);
        assert_eq!(calls(&e, TIMER_RESET), vec![vec![t.procedure + 0x34]]);
        assert_eq!(calls(&e, slot(0x43c)), vec![vec![0x9100, 1]]);
        assert_eq!(calls(&e, 0x0089_35f0), vec![vec![t.attacker, 0x72]]);
        assert_eq!(e.mem.u32(t.procedure + 0x10), 1);
        assert_eq!(
            calls(&e, START_DIALOGUE),
            vec![vec![0x7200, 0x6100, 2, 0, 0], vec![0x7200, 0x6100, 2, 4, 1]]
        );
        // Attacking already: wait unless the anim action is 4 with a low animation state.
        e.register(0x0089_4900, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009c_afe0, &args![t.procedure]);
        assert!(calls(&e, 0x0089_35f0).is_empty());
        e.register(0x008a_7570, |_, _| 4u32.into_ret());
        e.register(0x0070_f490, |_, a| (a[1] - 2).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009c_afe0, &args![t.procedure]);
        let animation = e.mem.u32(t.attacker + 4);
        assert_eq!(calls(&e, 0x0070_f490), vec![vec![animation, 4]]);
        assert_eq!(calls(&e, 0x0089_35f0).len(), 1);
        // A failed arc stops the throw.
        e.register(0x009a_7580, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009c_afe0, &args![t.procedure]);
        assert!(calls(&e, 0x0089_35f0).is_empty());
        // Switching weapons stops it before anything is checked.
        e.register(0x0098_1970, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009c_afe0, &args![t.procedure]);
        assert!(calls(&e, EXPLOSION_ALLOWED).is_empty());
    }

    #[test]
    fn a_fired_projectile_gets_the_arc_setting() {
        let mut e = Engine::new();
        let t = throw(&mut e, false, 0);
        noop(&mut e, &[0x0096_6a00]);
        e.mem.set_u8(t.procedure + 0x30, 1);
        e.call_log = Some(vec![]);
        e.call(0x009c_b830, &args![t.procedure, 0x4400u32]);
        assert_eq!(calls(&e, 0x0096_6a00), vec![vec![0x4400, 2.0f32.to_bits()]]);
        assert_eq!(e.mem.u8(t.procedure + 0x30), 0);
        assert_eq!(e.mem.u32(t.procedure + 8), 0);
        let t = throw(&mut e, true, 0);
        e.call_log = Some(vec![]);
        e.call(0x009c_b830, &args![t.procedure, 0x4400u32]);
        assert!(calls(&e, 0x0096_6a00).is_empty());
        assert_eq!(e.mem.u8(t.state + 0x73), 1);
        assert_eq!(e.mem.u32(t.procedure + 8), 2);
    }

    #[test]
    fn debug_text_shows_state_point_arc_and_timer() {
        let mut e = Engine::new();
        let printed = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let p = printed.clone();
        e.register_double(slot(0x14), move |_, a| {
            p.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.register(TIMER_RUNNING, |_, _| 1u32.into_ret());
        e.register(0x006b_9130, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        e.register(0x0043_5e00, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        let printer = object(&mut e, &[]);
        let g = e.mem.alloc(0x40);
        for k in 0..3 {
            e.mem.set_f32(g + 0x24 + 4 * k, 1.0 + k as f32);
        }
        e.mem.set_u8(g + 0x30, 1);
        e.mem.set_u32(g + 0x10, 1);
        e.call(0x009c_b890, &args![g, printer, 7u32, 8u32]);
        let d = |v: f64| [v.to_bits() as u32, (v.to_bits() >> 32) as u32];
        let mut point_line = vec![printer, 7, 8, POINT_FORMAT];
        for v in [1.0, 2.0, 3.0] {
            point_line.extend(d(v));
        }
        let mut timer_line = vec![printer, 7, 8, BLOCKED_TIMER_FORMAT];
        timer_line.extend(d(1.0));
        timer_line.extend(d(4.0));
        assert_eq!(
            *printed.borrow(),
            vec![
                vec![printer, 7, 8, ATTACKING_TEXT],
                point_line.clone(),
                vec![printer, 7, 8, HIGH_ARC_TEXT],
                timer_line
            ]
        );
        printed.borrow_mut().clear();
        e.mem.set_u32(g + 0x10, 0);
        e.mem.set_u8(g + 0x30, 0);
        e.register(TIMER_RUNNING, |_, _| Ret::default());
        e.call(0x009c_b890, &args![g, printer, 7u32, 8u32]);
        assert_eq!(
            *printed.borrow(),
            vec![vec![printer, 7, 8, IDLE_TEXT], point_line.clone()]
        );
        printed.borrow_mut().clear();
        e.mem.set_u32(g + 0x10, 2);
        e.call(0x009c_b890, &args![g, printer, 7u32, 8u32]);
        assert_eq!(*printed.borrow(), vec![point_line]);
    }

    #[test]
    fn save_game_writes_every_field_then_the_timer() {
        let mut e = Engine::new();
        noop(&mut e, &[PROCEDURE_SAVE_GAME, SAVE_DATA, 0x009a_5eb0]);
        let g = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x009c_b9c0, &args![g, 0x9000u32]);
        assert_eq!(calls(&e, PROCEDURE_SAVE_GAME), vec![vec![g, 0x9000]]);
        assert_eq!(
            calls(&e, SAVE_DATA),
            vec![
                vec![0x9000, g + 0x10, 4, 0],
                vec![0x9000, g + 0x14, 1, 0],
                vec![0x9000, g + 0x18, 0xc, 0],
                vec![0x9000, g + 0x24, 0xc, 0],
                vec![0x9000, g + 0x30, 1, 0]
            ]
        );
        assert_eq!(calls(&e, 0x009a_5eb0), vec![vec![g + 0x34, 0x9000]]);
    }

    #[test]
    fn load_game_reads_every_field_then_the_timer() {
        let mut e = Engine::new();
        noop(&mut e, &[PROCEDURE_LOAD_GAME, LOAD_DATA, 0x009a_5f20]);
        let g = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x009c_ba50, &args![g, 0x9000u32]);
        assert_eq!(calls(&e, PROCEDURE_LOAD_GAME), vec![vec![g, 0x9000]]);
        assert_eq!(
            calls(&e, LOAD_DATA),
            vec![
                vec![0x9000, g + 0x10, 4],
                vec![0x9000, g + 0x14, 1],
                vec![0x9000, g + 0x18, 0xc],
                vec![0x9000, g + 0x24, 0xc],
                vec![0x9000, g + 0x30, 1]
            ]
        );
        assert_eq!(calls(&e, 0x009a_5f20), vec![vec![g + 0x34, 0x9000]]);
    }
}
