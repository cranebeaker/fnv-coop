//! `fallout/ai/combat/missileprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009ba4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraint` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00c89640` and frees 0x10 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bhk_constraint_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00c8_9640, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x10u32]);
    }
    this
}

// Translated from 009ba520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkHingeConstraint` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00cbd970` and frees 0x10 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bhk_hinge_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x00cb_d970, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x10u32]);
    }
    this
}

// Translated from 009ba4d0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 0126818c without reading the receiver. The map names this body `bhkConstraint::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_009ba4d0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_818c
}

// Translated from 009ba510 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01268438 without reading the receiver. The map names this body `bhkHingeConstraint::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_009ba510(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_8438
}

/// Projectile vtable slots (byte offsets): the 3D node (`0x1d0`, no stack
/// words) and the position (`0x1f4`, answers the address of three floats).
const GET_3D_SLOT: u32 = 0x1d0;
const POSITION_SLOT: u32 = 0x1f4;
/// The double constant the update compares with (`01012060`, 0.0 in the exe).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// Callees of `MissileProjectile::UpdateProjectile`, by address (`thiscall`
/// on the projectile unless said otherwise; stack words in brackets).
const UPDATE_DETACHED_3D: u32 = 0x009b_abe0;
const FIELD_110_GETTER: u32 = 0x0050_8070;
const FIELD_D4_GETTER: u32 = 0x0097_9260;
const END_FLIGHT: u32 = 0x009b_c8f0;
const FLAG_8_BIT_20: u32 = 0x0044_0d80;
const FLAG_8_BIT_800: u32 = 0x0044_0da0;
/// `00444d00` [mask]: whether the word at +0xc8 has any bit of the mask.
const FLAGS_C8_HAVE: u32 = 0x0044_4d00;
/// `008256d0` (on a list node): empty; `006815c0`: the node's item address.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
/// `00460140` (on a collidable's holder) and `GetAVObjectForCollidable`
/// (`004b5820`, `cdecl`).
const HOLDER_COLLIDABLE: u32 = 0x0046_0140;
const AV_OBJECT_FOR_COLLIDABLE: u32 = 0x004b_5820;
/// `00461130` (on an `NiAVObject`): its `+0x68`; `0062c250` [out, other]
/// on that: a 3x3 matrix product into `out`.
const WORLD_ROTATE_ADDRESS: u32 = 0x0046_1130;
const MATRIX_COMBINE: u32 = 0x0062_c250;
/// `0043fa80` [matrix] on a node: sets its rotation.
const NODE_SET_ROTATE: u32 = 0x0043_fa80;
/// `Projectile::SetProjectileOrientation` (`009c6330`) [matrix].
const SET_PROJECTILE_ORIENTATION: u32 = 0x009c_6330;
/// `00440460` [out] on a node: its translation.
const NODE_TRANSLATE: u32 = 0x0044_0460;
/// `TESObjectREFR::SetLocationOnReference` (`00575830`) [point].
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
/// `Projectile::Sync3DWithReference` (`009bf470`) [two flags].
const SYNC_3D_WITH_REFERENCE: u32 = 0x009b_f470;
/// `00508070`, `00979260` and `00885bd0`: the floats at +0x110, +0xd4 and
/// +0xd8 (x87 `ST0`).
const FIELD_D8_GETTER: u32 = 0x0088_5bd0;
/// `008d8b80` (no stack words): an object the projectile aims at, or null
/// (folded body; the engine map's name is another class's).
const AIM_TARGET: u32 = 0x008d_8b80;
/// `009c5f30` [out] on that object: a point into `out`, answers its address.
const AIM_TARGET_POINT: u32 = 0x009c_5f30;
/// `Projectile::AimAtPoint` (`009c6030`) [x, y, z].
const AIM_AT_POINT: u32 = 0x009c_6030;
/// `00439ef0` [out, other] on a point: `this - other` into `out`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00457990` on a point: its length (x87).
const POINT_LENGTH: u32 = 0x0045_7990;
/// The settings `00403e20` (a float's address) and `00408d60` (a byte's
/// address) read, the `Setting`s `fProjectileReorientTracerMin` (`011cff34`),
/// `bProjectileDebug:Combat` (`011f20ec`), `fProjectileDebugDuration:Combat`
/// (`011f210c`) and `fArrowAgeMax` (`011ce684`), by their initializers.
/// `009bc8f0` is the call that ends the projectile.s flight (not
/// checked further here).
const SETTING_FLOAT: u32 = 0x0040_3e20;
const SETTING_BOOL: u32 = 0x0040_8d60;
const REORIENT_TRACER_MIN_SETTING: u32 = 0x011c_ff34;
const PROJECTILE_DEBUG_SETTING: u32 = 0x011f_20ec;
const DEBUG_DURATION_SETTING: u32 = 0x011f_210c;
const ARROW_AGE_MAX_SETTING: u32 = 0x011c_e684;
/// `009bf300` [time]: the projectile's movement step.
const MOVE_STEP: u32 = 0x009b_f300;
/// `00414430` [r, g, b, a] on a colour; `004b3890` (`cdecl`: from, colour,
/// to, colour, 1): a debug line; `TES::AddTempDebugObject` (`00458e20`)
/// [object, time] on the `TES` at `011dea10`.
const COLOR_SET: u32 = 0x0041_4430;
const MAKE_DEBUG_LINE: u32 = 0x004b_3890;
const ADD_TEMP_DEBUG_OBJECT: u32 = 0x0045_8e20;
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `00a68bf0` on the 3D node.
const NODE_UPDATE: u32 = 0x00a6_8bf0;
/// `004181e0` (the projectile's base form) and `009b88e0` on it (whether it
/// spins); the spin rates `00812870`, `009b88a0`, `009b88c0` (x87) on it.
const BASE_FORM: u32 = 0x0041_81e0;
const BASE_SPINS: u32 = 0x009b_88e0;
const SPIN_RATE_X: u32 = 0x0081_2870;
const SPIN_RATE_Y: u32 = 0x009b_88a0;
const SPIN_RATE_Z: u32 = 0x009b_88c0;
/// `NiMatrix3::MakeXRotation` (`00524ac0`), `MakeYRotation` (`0043f850`) and
/// `004a0c90` [angle] on a matrix.
const MAKE_X_ROTATION: u32 = 0x0052_4ac0;
const MAKE_Y_ROTATION: u32 = 0x0043_f850;
const MAKE_Z_ROTATION: u32 = 0x004a_0c90;
/// `006a9540` on a node: the address of its local rotation (`+0x34`).
const NODE_ROTATE_ADDRESS: u32 = 0x006a_9540;
/// `NiMatrix3::operator*` (`0043f8d0`) [out, other].
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `0055d520`: the projectile's name; `005b5e40` (`cdecl`, variable) prints
/// "---> Get3D() has failed in MissileProjectile::UpdateProjectile for
/// projectile: %s!" (`0108fd78`) with level 0xb.
const NAME: u32 = 0x0055_d520;
const PRINT: u32 = 0x005b_5e40;
const GET_3D_FAILED_FORMAT: u32 = 0x0108_fd78;
/// `Projectile::CheckExplosion` (`009c3190`) [time].
const CHECK_EXPLOSION: u32 = 0x009c_3190;
/// `00450f90` [1] on the 3D node.
const NODE_DETACH: u32 = 0x0045_0f90;

/// The projectile's 3D node (vtable `+0x1d0`).
fn node_3d(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr(), GET_3D_SLOT, &args![]).u32()
}

/// `FCOMP` of the second value against the first and `TEST AH,5`/`JP`: true
/// when `second < first` (ordered).
fn below(first: f64, second: f64) -> bool {
    second < first
}

/// The check the update runs at the start and the end: the float at +0xd4
/// (`00979260`) below the one at +0x110 (`00508070`, stored as a double
/// first).
fn d4_below_110(e: &mut Engine, this: Ptr) -> bool {
    let limit = e.call(FIELD_110_GETTER, &args![this]).f64();
    let alive = e.call(FIELD_D4_GETTER, &args![this]).f64();
    below(limit, alive)
}

/// Whether the list at +0x88 is empty or the byte at +0x90 is set.
fn no_hits_or_flag_90(e: &mut Engine, this: Ptr) -> bool {
    e.call(NODE_IS_EMPTY, &args![this.addr().wrapping_add(0x88)])
        .bool()
        || e.mem.u8(this.addr().wrapping_add(0x90)) != 0
}

/// The first item of the list at +0x88.
fn first_hit(e: &mut Engine, this: Ptr) -> u32 {
    let item = e
        .call(NODE_ITEM_ADDRESS, &args![this.addr().wrapping_add(0x88)])
        .u32();
    e.mem.u32(item)
}

/// A setting's float as the x87 loads it.
fn setting_float(e: &mut Engine, setting: u32) -> f64 {
    let address = e.call(SETTING_FLOAT, &args![setting]).u32();
    x87_m32(e.mem.u32(address))
}

// Translated from 009b8030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MissileProjectile::UpdateProjectile` (engine map name), `thiscall`,
/// `RET 4`: the time step. In the exe's order:
/// - with the byte at +0x154 and a 3D node, `009babe0`; when the float at
///   +0xd4 is below the one at +0x110 with no hits or the byte at +0x90,
///   `009bc8f0`;
/// - nothing more when bit 0x20 or 0x800 of the word at +8 is set;
/// - in state 4 (+0x150) the float at +0x158 drops by the step (x87
///   `FSUB`) and at or below 0.0 `009bc8f0` runs;
/// - flag 0x80 (+0xc8) in state 4 keeps it stuck to its first hit: with the
///   hit's `NiAVObject` (`004b5820` of `00460140` of the hit's +0x1c) and a
///   3D node, the hit's rotation times the matrix at +0x94 (`0062c250`)
///   orients the node and the projectile (`0043fa80`, `009c6330`) and the
///   node's translation (`00440460`) places it (`00575830`); then
///   `Sync3DWithReference(1, 0)`;
/// - otherwise, unless the byte at +0x90 is set, flag 1 is clear and the
///   state is not 3: the float at +0xd8 at or below 0.0 (or NaN) means
///   stopped. A
///   target (`008d8b80`) is aimed at (`009c6030` with `009c5f30`'s point).
///   Moving, the target's point minus the position (vtable `+0x1f4`) longer
///   than `fProjectileReorientTracerMin` sets the first sync flag; then the
///   step (`009bf300`), with a yellow debug line from the old to the new
///   position (`004b3890`, `00458e20` for `fProjectileDebugDuration`) when
///   `bProjectileDebug` is set. A first hit whose byte at +0x28 is
///   clear places the projectile at its point (+4, `00575830`) unless flag 1
///   is set and the +0xd4 check above fails. `Sync3DWithReference`
///   (that flag, moving), `00a68bf0` on the node, and for a spinning base
///   form (`009b88e0`) the angle at +0x15c grows by the first positive rate
///   (`00812870`, `009b88a0`, `009b88c0`) times the step, made into an X, Y
///   or third-axis rotation that multiplies the node's (`0043f8d0`,
///   `0043fa80`); without a node a warning names the projectile;
/// - always then `CheckExplosion` (`009c3190`); the float at +0xd8 above
///   `fArrowAgeMax` runs `009bc8f0` unless in state 4; and the +0xd4 check
///   with no hits or the byte at +0x90 gives the node `00450f90(1)` and runs
///   `009bc8f0`.
pub fn missile_projectile_update_projectile(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    let time_bits = time.to_bits();
    let zero = f64::from_bits(e.mem.u64(ZERO_DOUBLE));
    if e.mem.u8(base.wrapping_add(0x154)) != 0 && node_3d(e, this) != 0 {
        e.call(UPDATE_DETACHED_3D, &args![this]);
    }
    if d4_below_110(e, this) && no_hits_or_flag_90(e, this) {
        e.call(END_FLIGHT, &args![this]);
    }
    if e.call(FLAG_8_BIT_20, &args![this]).bool() || e.call(FLAG_8_BIT_800, &args![this]).bool() {
        return;
    }
    if e.mem.u32(base.wrapping_add(0x150)) == 4 {
        let left = x87_sub(
            x87_m32(e.mem.u32(base.wrapping_add(0x158))),
            x87_m32(time_bits),
        );
        e.mem.set_u32(base.wrapping_add(0x158), x87_store32(left));
        if x87_m32(e.mem.u32(base.wrapping_add(0x158))) <= zero {
            e.call(END_FLIGHT, &args![this]);
        }
    }
    let stuck = e.call(FLAGS_C8_HAVE, &args![this, 0x80u32]).bool()
        && e.mem.u32(base.wrapping_add(0x150)) == 4;
    if stuck {
        let hit = first_hit(e, this);
        if hit != 0 {
            let holder = e.mem.u32(hit.wrapping_add(0x1c));
            let object = if holder != 0 {
                let collidable = e.call(HOLDER_COLLIDABLE, &args![holder]).u32();
                e.call(AV_OBJECT_FOR_COLLIDABLE, &args![collidable]).u32()
            } else {
                0
            };
            let node = node_3d(e, this);
            if object != 0 && node != 0 {
                e.with_stack(0x24, |e, matrix| {
                    let rotate = e.call(WORLD_ROTATE_ADDRESS, &args![object]).u32();
                    e.call(
                        MATRIX_COMBINE,
                        &args![rotate, matrix, base.wrapping_add(0x94)],
                    );
                    e.call(NODE_SET_ROTATE, &args![node, matrix]);
                    e.call(SET_PROJECTILE_ORIENTATION, &args![this, matrix]);
                });
                e.with_stack(0xc, |e, point| {
                    e.call(NODE_TRANSLATE, &args![node, point]);
                    e.call(SET_LOCATION_ON_REFERENCE, &args![this, point]);
                });
            }
            e.call(SYNC_3D_WITH_REFERENCE, &args![this, 1u32, 0u32]);
        }
    } else if !(e.mem.u8(base.wrapping_add(0x90)) != 0
        && !e.call(FLAGS_C8_HAVE, &args![this, 1u32]).bool()
        && e.mem.u32(base.wrapping_add(0x150)) != 3)
    {
        update_moving(e, this, time_bits, zero);
    }
    e.call(CHECK_EXPLOSION, &args![this, time_bits]);
    let value = e.call(FIELD_D8_GETTER, &args![this]).f64();
    let limit = setting_float(e, ARROW_AGE_MAX_SETTING);
    if below(value, limit) && e.mem.u32(base.wrapping_add(0x150)) != 4 {
        e.call(END_FLIGHT, &args![this]);
    }
    if d4_below_110(e, this) && no_hits_or_flag_90(e, this) {
        if node_3d(e, this) != 0 {
            let node = node_3d(e, this);
            e.call(NODE_DETACH, &args![node, 1u32]);
        }
        e.call(END_FLIGHT, &args![this]);
    }
}

/// The moving part of [`missile_projectile_update_projectile`] (from
/// `009b8298` to `009b87a1`).
fn update_moving(e: &mut Engine, this: Ptr, time_bits: u32, zero: f64) {
    let base = this.addr();
    let movement = e.call(FIELD_D8_GETTER, &args![this]).f64();
    let stopped = !(movement > zero);
    let mut far_from_target = false;
    if e.call(AIM_TARGET, &args![this]).u32() != 0 {
        e.with_stack(0xc, |e, point| {
            let target = e.call(AIM_TARGET, &args![this]).u32();
            let at = e.call(AIM_TARGET_POINT, &args![target, point]).u32();
            let (x, y, z) = (e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8));
            e.call(AIM_AT_POINT, &args![this, x, y, z]);
        });
    }
    if !stopped {
        if e.call(AIM_TARGET, &args![this]).u32() != 0 {
            let position = e.vcall(base, POSITION_SLOT, &args![]).u32();
            let distance = e.with_stack(0x18, |e, locals| {
                let difference = locals.addr();
                let point = difference + 0xc;
                let target = e.call(AIM_TARGET, &args![this]).u32();
                let at = e.call(AIM_TARGET_POINT, &args![target, point]).u32();
                e.call(POINT_SUBTRACT, &args![at, difference, position]);
                e.call(POINT_LENGTH, &args![difference]).f64()
            });
            if below(distance, setting_float(e, REORIENT_TRACER_MIN_SETTING)) {
                far_from_target = true;
            }
        }
        let flag = e.call(SETTING_BOOL, &args![PROJECTILE_DEBUG_SETTING]).u32();
        if e.mem.u8(flag) != 0 {
            e.with_stack(0x28, |e, locals| {
                let from = locals.addr();
                let to = from + 0xc;
                let color = from + 0x18;
                let position = e.vcall(base, POSITION_SLOT, &args![]).u32();
                let old = e.mem.bytes(position, 0xc);
                e.mem.write(from, &old);
                e.call(MOVE_STEP, &args![this, time_bits]);
                let position = e.vcall(base, POSITION_SLOT, &args![]).u32();
                let new = e.mem.bytes(position, 0xc);
                e.mem.write(to, &new);
                let one = x87_store32(1.0);
                let none = x87_store32(0.0);
                e.call(COLOR_SET, &args![color, one, one, none, one]);
                let line = e
                    .call(MAKE_DEBUG_LINE, &args![from, color, to, color, 1u32])
                    .u32();
                let lifetime = e.call(SETTING_FLOAT, &args![DEBUG_DURATION_SETTING]).u32();
                let lifetime = x87_quiet_bits(e.mem.u32(lifetime));
                let tes = e.global::<u32>(TES_SINGLETON);
                e.call(ADD_TEMP_DEBUG_OBJECT, &args![tes, line, lifetime]);
            });
        } else {
            e.call(MOVE_STEP, &args![this, time_bits]);
        }
    }
    let held = e
        .call(NODE_IS_EMPTY, &args![base.wrapping_add(0x88)])
        .bool()
        || {
            let hit = first_hit(e, this);
            e.mem.u8(hit.wrapping_add(0x28)) != 0
        };
    if !held {
        let skip = e.call(FLAGS_C8_HAVE, &args![this, 1u32]).bool() && !d4_below_110(e, this);
        if !skip {
            let hit = first_hit(e, this);
            e.call(SET_LOCATION_ON_REFERENCE, &args![this, hit.wrapping_add(4)]);
        }
    }
    e.call(
        SYNC_3D_WITH_REFERENCE,
        &args![this, u32::from(far_from_target), u32::from(!stopped)],
    );
    if node_3d(e, this) != 0 {
        let node = node_3d(e, this);
        e.call(NODE_UPDATE, &args![node]);
    }
    let form = e.call(BASE_FORM, &args![this]).u32();
    if !e.call(BASE_SPINS, &args![form]).bool() {
        return;
    }
    e.with_stack(0x6c, |e, locals| {
        let spin = locals.addr();
        let rotate = spin + 0x24;
        let product = spin + 0x48;
        e.call(NODE_ITEM_ADDRESS, &args![spin]);
        for (rate, make) in [
            (SPIN_RATE_X, MAKE_X_ROTATION),
            (SPIN_RATE_Y, MAKE_Y_ROTATION),
            (SPIN_RATE_Z, MAKE_Z_ROTATION),
        ] {
            let form = e.call(BASE_FORM, &args![this]).u32();
            let value = e.call(rate, &args![form]).f64();
            if value > zero {
                let form = e.call(BASE_FORM, &args![this]).u32();
                let value = e.call(rate, &args![form]).f64();
                let angle = x87_add(
                    x87_mul(value, x87_m32(time_bits)),
                    x87_m32(e.mem.u32(base.wrapping_add(0x15c))),
                );
                e.mem.set_u32(base.wrapping_add(0x15c), x87_store32(angle));
                let angle = x87_quiet_bits(e.mem.u32(base.wrapping_add(0x15c)));
                e.call(make, &args![spin, angle]);
                break;
            }
        }
        if node_3d(e, this) != 0 {
            let node = node_3d(e, this);
            let current = e.call(NODE_ROTATE_ADDRESS, &args![node]).u32();
            let words = e.mem.bytes(current, 0x24);
            e.mem.write(rotate, &words);
            let result = e.call(MATRIX_MULTIPLY, &args![rotate, product, spin]).u32();
            let words = e.mem.bytes(result, 0x24);
            e.mem.write(rotate, &words);
            let node = node_3d(e, this);
            e.call(NODE_SET_ROTATE, &args![node, rotate]);
        } else {
            let name = e.call(NAME, &args![this]).u32();
            e.call(PRINT, &args![0xbu32, GET_3D_FAILED_FORMAT, name]);
        }
    });
}

// Translated from 00402b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 0x14 without reading the receiver (`thiscall`, plain `RET`). The
/// engine map names this body `bhkHingeConstraint::QCinfoSize`; identical-code
/// folding can share it.
pub fn fn_00402b00(_e: &mut Engine, _this: Ptr) -> u32 {
    0x14
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009b_a4d0, fn_009ba4d0(Ptr) -> u32),
        entry!(0x009b_a510, fn_009ba510(Ptr) -> u32),
        entry!(0x009b_a4e0, bhk_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x009b_a520, bhk_hinge_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x009b_8030, missile_projectile_update_projectile(Ptr, f32)),
        entry!(0x0040_2b00, fn_00402b00(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bhk_constraint_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x009b_a4e0, 0x00c8_9640, 0x10);
    }

    #[test]
    fn bhk_hinge_constraint_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x009b_a520, 0x00cb_d970, 0x10);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_009ba4d0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009b_a4d0, &args![0x1000u32]).u32(), 0x0126_818c);
        assert_eq!(e.call(0x009b_a4d0, &args![0u32]).u32(), 0x0126_818c);
    }

    #[test]
    fn fn_009ba510_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009b_a510, &args![0x1000u32]).u32(), 0x0126_8438);
        assert_eq!(e.call(0x009b_a510, &args![0u32]).u32(), 0x0126_8438);
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    const VTABLE: u32 = 0x00ab_0000;
    const GET_3D_FN: u32 = 0x00ab_1000;
    const POSITION_FN: u32 = 0x00ab_1004;
    /// Knobs the doubles read: the f64s for +0x110 (+0), +0xd4 (+8),
    /// +0xd8 (+0x10), spin rates x/y/z (+0x18, +0x20, +0x28); the 3D node
    /// (+0x30), the aim target (+0x34), the base spins (+0x38), the debug
    /// flag byte (+0x3c), the float settings (+0x40 aim distance, +0x44 debug
    /// time, +0x48 limit), the position (+0x50).
    const KNOBS: u32 = 0x00ad_0000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn st0(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x100);
        e.map(KNOBS, 0x100);
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(TES_SINGLETON, 0x7777);
        let mut slots = vec![0u32; 0x80];
        slots[0x1d0 / 4] = GET_3D_FN;
        slots[0x1f4 / 4] = POSITION_FN;
        e.put_vtable(VTABLE, &slots);
        e.register(GET_3D_FN, |e, _| e.mem.u32(KNOBS + 0x30).into_ret());
        e.register(POSITION_FN, |_, _| (KNOBS + 0x50).into_ret());
        e.register(FIELD_110_GETTER, |e, _| st0(e.mem.f64(KNOBS)));
        e.register(FIELD_D4_GETTER, |e, _| st0(e.mem.f64(KNOBS + 8)));
        e.register(FIELD_D8_GETTER, |e, _| st0(e.mem.f64(KNOBS + 0x10)));
        e.register(SPIN_RATE_X, |e, _| st0(e.mem.f64(KNOBS + 0x18)));
        e.register(SPIN_RATE_Y, |e, _| st0(e.mem.f64(KNOBS + 0x20)));
        e.register(SPIN_RATE_Z, |e, _| st0(e.mem.f64(KNOBS + 0x28)));
        e.register(AIM_TARGET, |e, _| e.mem.u32(KNOBS + 0x34).into_ret());
        e.register(BASE_SPINS, |e, _| e.mem.u32(KNOBS + 0x38).into_ret());
        e.register(SETTING_BOOL, |_, _| (KNOBS + 0x3c).into_ret());
        e.register(SETTING_FLOAT, |_, a| {
            match a[0] {
                REORIENT_TRACER_MIN_SETTING => KNOBS + 0x40,
                DEBUG_DURATION_SETTING => KNOBS + 0x44,
                _ => KNOBS + 0x48,
            }
            .into_ret()
        });
        e.register(FLAG_8_BIT_20, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & 0x20 != 0).into_ret()
        });
        e.register(FLAG_8_BIT_800, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & 0x800 != 0).into_ret()
        });
        e.register(FLAGS_C8_HAVE, |e, a| {
            u32::from(e.mem.u32(a[0] + 0xc8) & a[1] != 0).into_ret()
        });
        e.register(NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(AIM_TARGET_POINT, |e, a| {
            e.mem.set_f32(a[1], 5.0);
            e.mem.set_f32(a[1] + 4, 6.0);
            e.mem.set_f32(a[1] + 8, 7.0);
            a[1].into_ret()
        });
        e.register(POINT_LENGTH, |_, _| st0(100.0));
        e.register(MAKE_DEBUG_LINE, |_, _| 0x4444u32.into_ret());
        e.register(HOLDER_COLLIDABLE, |_, a| (a[0] + 1).into_ret());
        e.register(AV_OBJECT_FOR_COLLIDABLE, |_, a| (a[0] + 1).into_ret());
        e.register(WORLD_ROTATE_ADDRESS, |_, a| (a[0] + 0x68).into_ret());
        e.register(BASE_FORM, |_, _| 0x5555u32.into_ret());
        e.register(NODE_ROTATE_ADDRESS, |_, a| (a[0] + 0x34).into_ret());
        e.register(MATRIX_MULTIPLY, |_, a| a[1].into_ret());
        e.register(NAME, |_, _| 0x6666u32.into_ret());
        for addr in [
            UPDATE_DETACHED_3D,
            END_FLIGHT,
            MATRIX_COMBINE,
            NODE_SET_ROTATE,
            SET_PROJECTILE_ORIENTATION,
            NODE_TRANSLATE,
            SET_LOCATION_ON_REFERENCE,
            SYNC_3D_WITH_REFERENCE,
            AIM_AT_POINT,
            POINT_SUBTRACT,
            MOVE_STEP,
            COLOR_SET,
            ADD_TEMP_DEBUG_OBJECT,
            NODE_UPDATE,
            MAKE_X_ROTATION,
            MAKE_Y_ROTATION,
            MAKE_Z_ROTATION,
            PRINT,
            CHECK_EXPLOSION,
            NODE_DETACH,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this, VTABLE);
        // The float at +0xd4 (20) is not below the one at +0x110 (10); the
        // one at +0xd8 (2) is above 0.
        e.mem.set_f64(KNOBS, 10.0);
        e.mem.set_f64(KNOBS + 8, 20.0);
        e.mem.set_f64(KNOBS + 0x10, 2.0);
        e.mem.set_f32(KNOBS + 0x40, 50.0);
        e.mem.set_f32(KNOBS + 0x44, 3.0);
        e.mem.set_f32(KNOBS + 0x48, 1000.0);
        (e, this)
    }

    #[test]
    fn update_stops_early_on_the_flags_at_8() {
        let (mut e, this) = engine();
        e.mem.set_u32(this + 8, 0x800);
        e.call_log = Some(vec![]);
        e.call(0x009b_8030, &args![this, 0.5f32]);
        assert!(calls_to(&e, CHECK_EXPLOSION).is_empty());
        assert_eq!(calls_to(&e, FIELD_D4_GETTER).len(), 1);
    }

    #[test]
    fn update_stuck_in_state_4_follows_its_hit() {
        let (mut e, this) = engine();
        e.mem.set_u32(this + 0x150, 4);
        e.mem.set_u32(this + 0xc8, 0x80);
        e.mem.set_f32(this + 0x158, 0.25);
        e.mem.set_u32(KNOBS + 0x30, 0x9000);
        let hit = e.mem.alloc(0x30);
        e.mem.set_u32(hit + 0x1c, 0x100);
        e.mem.set_u32(this + 0x88, hit);
        e.call_log = Some(vec![]);
        e.call(0x009b_8030, &args![this, 0.5f32]);
        // The timer ran out: marked.
        assert_eq!(e.mem.f32(this + 0x158), -0.25);
        assert_eq!(calls_to(&e, END_FLIGHT), vec![vec![this]]);
        let combine = calls_to(&e, MATRIX_COMBINE);
        assert_eq!((combine[0][0], combine[0][2]), (0x102 + 0x68, this + 0x94));
        assert_eq!(
            calls_to(&e, NODE_SET_ROTATE),
            vec![vec![0x9000, combine[0][1]]]
        );
        assert_eq!(
            calls_to(&e, SET_PROJECTILE_ORIENTATION),
            vec![vec![this, combine[0][1]]]
        );
        assert_eq!(calls_to(&e, SYNC_3D_WITH_REFERENCE), vec![vec![this, 1, 0]]);
        assert!(calls_to(&e, MOVE_STEP).is_empty());
        assert_eq!(
            calls_to(&e, CHECK_EXPLOSION),
            vec![vec![this, 0.5f32.to_bits()]]
        );
    }

    #[test]
    fn update_moves_aims_places_and_spins() {
        let (mut e, this) = engine();
        let node = e.mem.alloc(0x60);
        e.mem.set_u32(KNOBS + 0x30, node);
        e.mem.set_u32(KNOBS + 0x34, 0x8000);
        e.mem.set_u32(KNOBS + 0x38, 1);
        // Only the Y rate is positive.
        e.mem.set_f64(KNOBS + 0x20, 4.0);
        e.mem.set_f32(this + 0x15c, 1.0);
        let hit = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x88, hit);
        e.call_log = Some(vec![]);
        e.call(0x009b_8030, &args![this, 0.5f32]);
        assert_eq!(
            calls_to(&e, AIM_AT_POINT),
            vec![vec![
                this,
                5.0f32.to_bits(),
                6.0f32.to_bits(),
                7.0f32.to_bits()
            ]]
        );
        assert_eq!(calls_to(&e, MOVE_STEP), vec![vec![this, 0.5f32.to_bits()]]);
        assert_eq!(
            calls_to(&e, SET_LOCATION_ON_REFERENCE),
            vec![vec![this, hit + 4]]
        );
        // Distance 100 above the setting 50: the first sync flag; moving.
        assert_eq!(calls_to(&e, SYNC_3D_WITH_REFERENCE), vec![vec![this, 1, 1]]);
        assert_eq!(calls_to(&e, NODE_UPDATE), vec![vec![node]]);
        assert_eq!(e.mem.f32(this + 0x15c), 3.0);
        assert!(calls_to(&e, MAKE_X_ROTATION).is_empty());
        let y = calls_to(&e, MAKE_Y_ROTATION);
        assert_eq!(y[0][1], 3.0f32.to_bits());
        let multiply = calls_to(&e, MATRIX_MULTIPLY);
        assert_eq!(multiply[0][2], y[0][0]);
        assert_eq!(
            calls_to(&e, NODE_SET_ROTATE),
            vec![vec![node, multiply[0][0]]]
        );
        assert!(calls_to(&e, END_FLIGHT).is_empty());
    }

    #[test]
    fn update_draws_debug_lines_warns_without_3d_and_dies_at_the_end() {
        let (mut e, this) = engine();
        e.mem.set_u8(KNOBS + 0x3c, 1);
        e.mem.set_u32(KNOBS + 0x38, 1);
        e.mem.set_f64(KNOBS + 0x28, 2.0);
        // +0xd4 (5) below +0x110 (10), no hits; +0xd8 above fArrowAgeMax.
        e.mem.set_f64(KNOBS + 8, 5.0);
        e.mem.set_f64(KNOBS + 0x10, 2000.0);
        e.call_log = Some(vec![]);
        e.call(0x009b_8030, &args![this, 0.5f32]);
        let color = calls_to(&e, COLOR_SET);
        assert_eq!(
            &color[0][1..],
            [1.0f32.to_bits(), 1.0f32.to_bits(), 0, 1.0f32.to_bits()]
        );
        let line = calls_to(&e, MAKE_DEBUG_LINE);
        assert_eq!(
            (line[0][1], line[0][3], line[0][4]),
            (color[0][0], color[0][0], 1)
        );
        assert_eq!(
            calls_to(&e, ADD_TEMP_DEBUG_OBJECT),
            vec![vec![0x7777, 0x4444, 3.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, MAKE_Z_ROTATION).len(), 1);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![0xb, GET_3D_FAILED_FORMAT, 0x6666]]
        );
        // 009bc8f0 at the start, for fArrowAgeMax and at the end.
        assert_eq!(calls_to(&e, END_FLIGHT).len(), 3);
        assert!(calls_to(&e, NODE_DETACH).is_empty());
        // Stopped (movement 0) with the byte at +0x90 and flag 1 clear in
        // state 3: no step, a non-moving sync.
        let (mut e, this) = engine();
        e.mem.set_f64(KNOBS + 0x10, 0.0);
        e.mem.set_u8(this + 0x90, 1);
        e.mem.set_u32(this + 0x150, 3);
        e.call_log = Some(vec![]);
        e.call(0x009b_8030, &args![this, 0.5f32]);
        assert!(calls_to(&e, MOVE_STEP).is_empty());
        assert_eq!(calls_to(&e, SYNC_3D_WITH_REFERENCE), vec![vec![this, 0, 0]]);
        // The same in state 0: the moving part is skipped.
        e.mem.set_u32(this + 0x150, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_8030, &args![this, 0.5f32]);
        assert!(calls_to(&e, SYNC_3D_WITH_REFERENCE).is_empty());
        assert_eq!(calls_to(&e, CHECK_EXPLOSION).len(), 1);
    }

    #[test]
    fn fn_00402b00_answers_0x14() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0040_2b00, &args![0x1234u32]).u32(), 0x14);
    }
}
