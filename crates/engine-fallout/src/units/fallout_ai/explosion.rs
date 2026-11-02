//! `fallout/ai/combat/explosion.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

// Continuation audit (2026-10-11): normal paths of 009ae280, 009b2940,
// 009acd60, 009ad670, 009ae6e0, 009b00a0 and 009b1810 have regression
// coverage. 009b00a0 has no SEH frame; the other six remain partial for
// compiler SEH cleanup. Seventeen functions retain explicit SEH partial
// markers; no normal-path body remains skipped.
// 009acd60 has a normal-path translation; it remains partial for compiler SEH
//   cleanup and calls the now-translated 009ad670 Init3D path.
// 009ad670 has its normal Init3D path translated, with a per-iteration 0xe0
//   Havok cinfo and stack-backed clone/transform temporaries. Compiler SEH
//   cleanup remains partial.
// 009ae6e0 has its normal effect/contact/decal/sound flow translated with
//   PC EBP slots and callback reloads. Compiler SEH cleanup remains partial;
//   its 009b00a0 target callee now has a normal-path translation.
// 009b1810 has a four-word stack ABI, a 0x320-byte collector, 0x60-byte hit
//   rows and two PC collector slots. See the private a04 round-9 handoff for
//   the ray-input audit; exception cleanup remains open.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `Explosion` (Xbox PDB), PC offsets confirmed by 009abf30; the TESForm
    /// base is 0x10 smaller than the Xbox build. Embedded fields are addressed
    /// through their shared helpers rather than redeclaring container layouts.
    pub struct Explosion: 0x104 {
        0x88 fAge: f32,
        0x8c fLifetime: f32,
        0x90 fBuildTime: f32,
        0x94 fRadius: f32,
        0x98 fImageSpaceRadius: f32,
        0x9c fWeaponCondition: f32,
        0xa0 spPhantom: Ptr,
        0xc4 spLight: Ptr,
        0xc8 pOwner: Ptr,
        0xcc pExplodedRef: Ptr,
        0xd0 spActorCause: Ptr,
        0xd4 bDecalsPlaced: bool,
        0xd5 bTargetsFound: bool,
        0xd6 bTargetsProcessed: bool,
        0xd7 bForcesApplied: bool,
        0xd8 bIgnoreImageSpaceSwap: bool,
        0xd9 bUnderWater: bool,
        0xdc pCaster: Ptr,
        0xe0 pWeaponSource: Ptr,
        0xe4 iFrameCount: i32,
        0x100 fCalculatedDamage: f32,
    }
    /// `bhkLiquidAction` (Xbox PDB), same size/offsets confirmed in 009ae140.
    pub struct BhkLiquidAction: 0x40 {
        0x10 pTargetObject: Ptr,
        0x14 fNeighborDistance: f32,
        0x18 fNeighborStrength: f32,
        0x1c fInitialStickForce: f32,
        0x20 fStickStrength: f32,
        0x3c bFirstUpdate: bool,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009ac200, explosion_ctor_ov2(Ptr, Ptr, Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x009b0e00, fn_009b0e00(Ptr, Ptr, u8, u8) -> Ptr),
        entry!(0x009b0f80, fn_009b0f80(Ptr) -> f32),
        entry!(0x009ac5a0, fn_009ac5a0(Ptr)),
        entry!(0x009ac9c0, fn_009ac9c0(Ptr, Ptr, Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x009afcc0, fn_009afcc0(Ptr)),
        entry!(0x009afef0, fn_009afef0(Ptr)),
        entry!(0x009b0920, fn_009b0920(Ptr, Ptr)),
        entry!(0x009b2040, fn_009b2040(Ptr, Ptr)),
        entry!(0x009b22c0, fn_009b22c0(Ptr, Ptr)),
        entry!(0x009b25b0, fn_009b25b0(Ptr, Ptr)),
        entry!(0x009b1510, fn_009b1510(Ptr, Ptr, Ptr, f32)),
        entry!(0x009b1630, fn_009b1630(Ptr, Ptr, i32, f32)),
        entry!(0x009b1720, fn_009b1720(Ptr, Ptr) -> Ptr),
        entry!(0x009b17a0, fn_009b17a0(Ptr, u32)),
        entry!(0x009b2770, fn_009b2770(Ptr, u32)),
        entry!(0x009b2790, fn_009b2790(Ptr, u32)),
        entry!(0x009b28a0, fn_009b28a0(Ptr, Ptr)),
        entry!(0x009b2940, fn_009b2940(Ptr) -> bool),
        entry!(0x009b2c90, fn_009b2c90(Ptr) -> Ptr),
        entry!(0x009b2cb0, fn_009b2cb0(Ptr, u32) -> Ptr),
        entry!(0x009b2d30, fn_009b2d30(Ptr)),
        entry!(0x009b2db0, fn_009b2db0(Ptr)),
        entry!(0x009b2dd0, fn_009b2dd0(Ptr, Ptr) -> Ptr),
        entry!(0x009b2e00, fn_009b2e00(Ptr, i32)),
        entry!(0x009b2e30, fn_009b2e30(Ptr, u32) -> Ptr),
        entry!(0x009b2e60, fn_009b2e60(u32, u32) -> Ptr),
        entry!(0x009b2e90, fn_009b2e90(Ptr, i32, u8)),
        entry!(0x009b2ed0, fn_009b2ed0(Ptr, i32, i32, u8)),
        entry!(0x009b3080, fn_009b3080(Ptr, Ptr, Ptr) -> bool),
        entry!(0x009abf30, explosion_ctor(Ptr) -> Ptr),
        entry!(0x009ad630, tes_image_space_modifier_get_explosion_in_face() -> Ptr),
        entry!(0x009ae140, bhk_liquid_action_ctor(Ptr, Ptr) -> Ptr),
        entry!(0x009ae4a0, fn_009ae4a0(Ptr, Ptr) -> Ptr),
        entry!(0x009ae4d0, fn_009ae4d0(Ptr, Ptr) -> Ptr),
        entry!(0x009afaf0, fn_009afaf0(Ptr, u32, u32) -> Ptr),
        entry!(0x009afbe0, fn_009afbe0(Ptr)),
        entry!(0x009afed0, fn_009afed0(Ptr) -> Ptr),
        entry!(0x009b0900, fn_009b0900(Ptr) -> Ptr),
        entry!(0x009b0bd0, fn_009b0bd0(Ptr) -> f32),
        entry!(0x009b0bf0, fn_009b0bf0(Ptr, Ptr)),
        entry!(0x009b0c50, fn_009b0c50(Ptr, Ptr)),
        entry!(0x009b0c90, fn_009b0c90(Ptr, Ptr)),
        entry!(0x009b0cf0, fn_009b0cf0(Ptr, Ptr)),
        entry!(0x009b0d30, fn_009b0d30(Ptr, Ptr)),
        entry!(0x009b0d70, fn_009b0d70(Ptr, Ptr, f32)),
        entry!(0x009b0f40, fn_009b0f40(Ptr) -> Ptr),
        entry!(0x009b11c0, explosion_stop_sounds(Ptr)),
        entry!(0x009b1260, explosion_attach_light(Ptr)),
        entry!(0x009b12d0, fn_009b12d0(Ptr)),
        entry!(0x009b1090, explosion_start_sounds(Ptr)),
        entry!(0x009b1320, fn_009b1320(Ptr)),
        entry!(0x009b13a0, fn_009b13a0(Ptr)),
        entry!(0x009b1450, fn_009b1450(Ptr) -> bool),
        entry!(0x00471840, fn_00471840(Ptr) -> u32),
        entry!(0x009ac170, fn_009ac170(Ptr, Ptr) -> u32),
        entry!(0x009ac190, fn_009ac190(Ptr) -> Ptr),
        entry!(0x009ac1d0, fn_009ac1d0(Ptr, u32) -> Ptr),
        entry!(0x009ac550, fn_009ac550(Ptr) -> f32),
        entry!(0x009ac910, fn_009ac910(Ptr, u32) -> Ptr),
        entry!(0x009ac940, fn_009ac940(Ptr)),
        entry!(0x009ac9a0, fn_009ac9a0(Ptr)),
        entry!(0x009acce0, fn_009acce0(Ptr) -> Ptr),
        entry!(0x009acd00, explosion_attach_distant(Ptr)),
        entry!(0x009acd60, explosion_initialize(Ptr)),
        entry!(0x009ad670, explosion_init_3d(Ptr)),
        entry!(0x009ad5a0, fn_009ad5a0(Ptr) -> f32),
        entry!(0x009ad5f0, fn_009ad5f0(Ptr) -> bool),
        entry!(0x009ad610, fn_009ad610(Ptr) -> Ptr),
        entry!(0x009ae0f0, fn_009ae0f0(Ptr)),
        entry!(0x009ae240, fn_009ae240(Ptr) -> Ptr),
        entry!(0x009ae250, fn_009ae250(Ptr, u32) -> Ptr),
        entry!(0x009ae280, fn_009ae280(Ptr)),
        entry!(0x009ae540, fn_009ae540(Ptr, Ptr) -> Ptr),
        entry!(0x009ae580, fn_009ae580(Ptr, f32)),
        entry!(0x009ae5d0, fn_009ae5d0(Ptr, u32)),
        entry!(0x009ae6e0, explosion_apply_effects(Ptr)),
        entry!(0x009b00a0, explosion_process_targets(Ptr)),
        entry!(0x009b1810, explosion_run_los_pick(Ptr, u32, u32, u32, Ptr) -> bool),
        entry!(0x009afbb0, fn_009afbb0(Ptr, u32) -> Ptr),
        entry!(0x009afc10, fn_009afc10(Ptr)),
        entry!(0x009afc30, fn_009afc30(Ptr) -> Ptr),
        entry!(0x009afc50, fn_009afc50(Ptr) -> u32),
        entry!(0x009afc70, fn_009afc70(Ptr, u32) -> u32),
        entry!(0x009afca0, fn_009afca0(Ptr, Ptr)),
    ]
}

// Translated from 00471840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLiquidAction::QCinfoSize` (Xbox PDB): returns the fixed size 0x18.
pub fn fn_00471840(_e: &mut Engine, _this: Ptr) -> u32 {
    0x18
}

// Translated from 009ac170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the actor-cause pointer held at `this+0xD0` through the engine's
/// reference-pointer helper, answering the helper's value (the exe ends on
/// that call and keeps its `EAX`; the virtual slot's `ActorCause*`).
pub fn fn_009ac170(e: &mut Engine, this: Ptr, cause: Ptr) -> u32 {
    e.call(0x0042_f780, &args![this.addr() + 0xd0, cause]).u32()
}

// Translated from 009ac190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the pointer at `this+0xDC`, advanced by 0x0C when non-null.
pub fn fn_009ac190(e: &mut Engine, this: Ptr) -> Ptr {
    let pointer = e.mem.u32(this.addr() + 0xdc);
    Ptr::new(if pointer == 0 { 0 } else { pointer + 0x0c })
}

// Translated from 009ac1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor wrapper: runs the destructor and frees `this`
/// when bit 0 of `flags` is set.
pub fn fn_009ac1d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009ac5a0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 009ac5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the `Explosion` destruction path: removes particle and physics
/// attachments, target records, sounds and owned references, then destroys
/// the embedded members and `MobileObject` base.
// open: compiler SEH cleanup for partially destroyed members is not modeled.
// Translation incomplete: 009ac5a0 - compiler SEH cleanup for partially destroyed members
pub fn fn_009ac5a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_ee04);
    e.mem.set_u32(this.addr() + 0x18, 0x0108_edf8);

    let base = e.call(0x0043_fcd0, &args![this]).u32();
    if base != 0 {
        let has_object = e.vcall(base, 0xc, &args![]).u32();
        if has_object != 0 {
            let base = e.call(0x0043_fcd0, &args![this]).u32();
            let object = e.vcall(base, 0xc, &args![]).u32();
            if e.call(0x0057_84b0, &args![object]).bool() {
                let base = e.call(0x0043_fcd0, &args![this]).u32();
                let object = e.vcall(base, 0xc, &args![]).u32();
                e.call(0x0057_8170, &args![object]);
            }
        }
    }

    let physics = e.call(0x008d_6f30, &args![this]).u32();
    if physics != 0 && e.call(0x0045_43c0, &args![physics]).bool() {
        let physics = e.call(0x008d_6f30, &args![this]).u32();
        if physics != 0 && e.call(0x0045_43c0, &args![physics]).bool() {
            let base = e.call(0x0043_fcd0, &args![this]).u32();
            e.call(0x00c6_9ee0, &args![base, 1u32, 0u32]);
        }
    }

    fn_009b12d0(e, this);
    let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
    if phantom != 0 {
        let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
        e.vcall(phantom, 0xa0, &args![]);
    }
    e.call(0x0066_b0d0, &args![this.addr() + 0xa0, 0u32]);

    let targets = this.addr() + 0xa4;
    while !e.call(0x0082_56d0, &args![targets]).bool() {
        let value = e.call(0x0068_15c0, &args![targets]).u32();
        let record = e.mem.u32(value);
        if record != 0 {
            fn_009ac910(e, Ptr::new(record), 1);
        }
        e.call(0x0063_f7b0, &args![targets]);
    }

    if !e
        .call(0x0042_26e0, &args![e.global::<u32>(0x011c_3f2c)])
        .bool()
    {
        e.call(0x0092_ed60, &args![this, 0u32]);
    }
    explosion_stop_sounds(e, this);
    e.call(0x0042_f780, &args![this.addr() + 0xd0, 0u32]);
    let actor_cause = e.mem.u32(this.addr() + 0xdc);
    if actor_cause != 0 {
        e.vcall(actor_cause, 0, &args![1u32]);
    }

    let process_list_enabled = e.call(0x0040_8d60, &args![0x011c_7ae8u32]).u32();
    if e.mem.u8(process_list_enabled) != 0 {
        let list = 0x011c_7b8c;
        let mut node = e.call(0x0055_9450, &args![list]).u32();
        let mut removed = 0;
        while node != 0 && removed == 0 {
            let next = e.call(0x007b_52d0, &args![list, node]).u32();
            let reference = e.call(0x0063_17a0, &args![list, node]).u32();
            if e.mem.u32(reference) == this.addr() {
                removed = node;
            }
            node = next;
        }
        if removed != 0 {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), removed);
                e.call(0x0049_f590, &args![list, slot]);
            });
        }
    }

    let base = e.call(0x0043_fcd0, &args![this]).u32();
    let physics = e.call(0x008d_6f30, &args![this]).u32();
    if physics != 0 {
        let process = e.call(0x009d_9f20, &args![physics]).u32();
        if process != 0 {
            e.call(0x00c5_b370, &args![process, base]);
        }
    }

    e.call(0x0042_f760, &args![this.addr() + 0xd0]);
    e.call(0x0045_cec0, &args![this.addr() + 0xc4]);
    e.call(0x0048_3710, &args![this.addr() + 0xb8]);
    e.call(0x0048_3710, &args![this.addr() + 0xac]);
    e.call(0x0046_ffb0, &args![this.addr() + 0xa4]);
    e.call(0x0045_cec0, &args![this.addr() + 0xa0]);
    e.call(0x0092_ec40, &args![this]);
}

// Translated from 009ac550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSExplosion::GetImageSpaceRadiusBSUnits` (Xbox PDB): returns the stored
/// radius when the unit flag is set; otherwise scales by the game setting.
pub fn fn_009ac550(e: &mut Engine, this: Ptr) -> f32 {
    if e.call(0x0047_7950, &args![this, 1u32]).bool() {
        return e.mem.f32(this.addr() + 0x8c);
    }
    let setting = e.call(0x0040_3e20, &args![0x011d_1218u32]).ptr::<u32>();
    ((e.mem.f32(this.addr() + 0x8c) as f64) * (e.mem.f32(setting.addr()) as f64)) as f32
}

// Translated from 009ac9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Optionally creates an explosion, applies the game entry point damage
/// multiplier, attaches actor cause and performs the post-creation callbacks.
// open: compiler SEH cleanup for allocated explosion and actor-cause temporaries is not modeled.
// Translation incomplete: 009ac9c0 - compiler SEH cleanup for allocated explosion and actor-cause temporaries
#[allow(clippy::too_many_arguments)]
pub fn fn_009ac9c0(
    e: &mut Engine,
    definition: Ptr,
    owner: Ptr,
    actor_cause: Ptr,
    cell: Ptr,
    x: u32,
    y: u32,
    z: u32,
    m00: u32,
    m01: u32,
    m02: u32,
    m10: u32,
    m11: u32,
    m12: u32,
    m20: u32,
    m21: u32,
    m22: u32,
) -> Ptr {
    let player = Ptr::new(e.global::<u32>(0x011d_ea3c));
    let mut damage_multiplier = 1.0f32;
    let mut explosion = Ptr::new(0);

    if !definition.is_null() {
        if !owner.is_null() {
            if e.vcall(owner.addr(), 0x224, &args![]).bool() {
                let owner_base = e.call(0x0087_4480, &args![owner]).ptr::<()>();
                if owner_base == player {
                    let entry_value = e.call(0x0047_6c90, &args![owner]).u32();
                    e.with_stack(4, |e, output| {
                        e.mem.set_f32(output.addr(), damage_multiplier);
                        e.call(0x005e_58f0, &args![0x48u32, player, entry_value, output]);
                        damage_multiplier = e.mem.f32(output.addr());
                    });
                }
            } else if owner == player {
                let process = e.call(0x008d_8520, &args![player]).ptr::<()>();
                let saved_object = if process.is_null() {
                    0
                } else {
                    e.vcall(process.addr(), 0x148, &args![]).u32()
                };
                let entry_value = if saved_object == 0 {
                    0
                } else {
                    e.call(0x0044_ddc0, &args![saved_object]).u32()
                };
                e.with_stack(4, |e, output| {
                    e.mem.set_f32(output.addr(), damage_multiplier);
                    e.call(0x005e_58f0, &args![0x48u32, player, entry_value, output]);
                    damage_multiplier = e.mem.f32(output.addr());
                });
            }
        } else if owner == player {
            let process = e.call(0x008d_8520, &args![player]).ptr::<()>();
            let saved_object = if process.is_null() {
                0
            } else {
                e.vcall(process.addr(), 0x148, &args![]).u32()
            };
            let entry_value = if saved_object == 0 {
                0
            } else {
                e.call(0x0044_ddc0, &args![saved_object]).u32()
            };
            e.with_stack(4, |e, output| {
                e.mem.set_f32(output.addr(), damage_multiplier);
                e.call(0x005e_58f0, &args![0x48u32, player, entry_value, output]);
                damage_multiplier = e.mem.f32(output.addr());
            });
        }

        let allocation = e.call(0x0040_1000, &args![0x104u32]).ptr::<()>();
        if !allocation.is_null() {
            explosion = explosion_ctor_ov2(
                e, allocation, definition, owner, cell, x, y, z, m00, m01, m02, m10, m11, m12, m20,
                m21, m22,
            );
        }
    }

    let base_damage = e.call(0x0064_4930, &args![explosion]).f32();
    let scaled_damage = quiet_f32(((base_damage as f64) * (damage_multiplier as f64)) as f32);
    e.call(0x0040_7880, &args![explosion, scaled_damage]);

    if !explosion.is_null() {
        let current_owner = fn_009acce0(e, explosion);
        if !current_owner.is_null() {
            let attached_actor_cause = if e.vcall(current_owner.addr(), 0x100, &args![]).bool() {
                let allocation = e.call(0x0040_1000, &args![0x1cu32]).ptr::<()>();
                if allocation.is_null() {
                    0
                } else {
                    let owner = fn_009acce0(e, explosion);
                    e.call(0x0066_e010, &args![allocation, owner]).u32()
                }
            } else {
                let current_owner = fn_009acce0(e, explosion);
                e.vcall(current_owner.addr(), 0x168, &args![]).u32()
            };
            e.vcall(explosion.addr(), 0x164, &args![attached_actor_cause]);
        }

        e.call(0x0056_c7d0, &args![explosion, actor_cause]);
        if !actor_cause.is_null() && e.call(0x0057_2c80, &args![actor_cause]).bool() {
            e.vcall(actor_cause.addr(), 0x48, &args![4u32]);
        }
        if !e.call(0x008c_7aa0, &args![]).bool() {
            e.vcall(explosion.addr(), 0x304, &args![]);
        } else {
            // 009accb2: `004537b0` takes no stack word (plain RET); the pushed explosion
            // is the word of `0087ae60` (RET 4).
            let process = e.call(0x0045_37b0, &args![]).u32();
            e.call(0x0087_ae60, &args![process, explosion]);
        }
    }
    explosion
}

// Translated from 009ac910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor wrapper for an explosion target record.
pub fn fn_009ac910(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x009a_c940, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 009ac940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears and destroys the embedded target list at `this+8`.
pub fn fn_009ac940(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 8;
    e.call(0x0047_0470, &args![list]);
    e.call(0x0046_ffb0, &args![list]);
}

// Translated from 009ac9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the object's virtual function at byte offset 0xC4 with argument 1.
pub fn fn_009ac9a0(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0xc4, &args![1u32]);
}

// Translated from 009acce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `this+0xC8`.
pub fn fn_009acce0(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0xc8))
}

// Translated from 009acd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attaches this explosion to distant terrain when its virtual target exists.
pub fn explosion_attach_distant(e: &mut Engine, this: Ptr) {
    let target = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    if target == 0 {
        return;
    }
    let tes = e.call(0x004f_d3e0, &args![0x011d_ea10u32]).u32();
    let world = e.call(0x0058_6170, &args![tes]).u32();
    let terrain = e.call(0x0055_85e0, &args![world]).u32();
    if terrain != 0 {
        e.vcall(terrain, 0xdc, &args![target, 1u32]);
    }
}

// Translated from 009acd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::Initialize` (Xbox PDB): creates the high process if absent,
/// computes explosion power and image-space effects, initializes water and
/// 3D, spawns its impact placed object, attaches the reference, starts sound,
/// and registers a newly created process. The PC disassembly fixes the call order and all pending stack
/// arguments. The debug-scope TLS pair has no gameplay state.
// open: compiler SEH cleanup for the high process and sound temporaries.
// Translation incomplete: 009acd60 - compiler SEH cleanup on an exception
pub fn explosion_initialize(e: &mut Engine, this: Ptr) {
    let scene_manager = e.global::<u32>(0x011d_df38);
    let scene_mode = e.call(0x0042_ce10, &args![scene_manager]).bool();
    let needs_process = e.mem.u32(this.addr() + 0x68) == 0;
    if needs_process {
        let allocation = e.call(0x0040_1000, &args![0x46cu32]).ptr::<()>();
        let process = if allocation.is_null() {
            Ptr::NULL
        } else {
            e.call(0x008d_7510, &args![allocation]).ptr::<()>()
        };
        e.mem.set_u32(this.addr() + 0x68, process.addr());
    }

    e.with_stack(0xb0, |e, frame| {
        let difference = Ptr::<()>::new(frame.addr() + 0x10);
        let world_position = e.vcall(this.addr(), 0x1f4, &args![]).ptr::<()>();
        let player = e.global::<u32>(0x011d_ea3c);
        let player_position = e.vcall(player, 0x1f4, &args![]).ptr::<()>();
        let distance_vector = e
            .call(
                0x0043_9ef0,
                &args![player_position, difference, world_position],
            )
            .ptr::<()>();
        let distance = e.call(0x0045_7990, &args![distance_vector]).f32();
        let radius = e.call(0x0064_4930, &args![this]).f32();
        let power = e.call(0x0064_7920, &args![radius, distance]).f32();
        let threshold = e.call(0x0040_3e20, &args![0x011d_0520u32]).ptr::<()>();
        let in_face = e.mem.f32(threshold.addr()) < power && e.mem.u8(this.addr() + 0xd8) == 0;
        if in_face {
            let modifier = tes_image_space_modifier_get_explosion_in_face(e);
            e.call(0x0052_99a0, &args![modifier, 1.0f32, 0u32]);
        }

        fn_009b2940(e, this);
        // Keep this through the registry so Initialize's existing regression
        // doubles can still isolate its 3D initialization branch.
        e.call(0x009a_d670, &args![this]);
        let distant = fn_009ad5f0(e, this);
        if distant {
            explosion_attach_distant(e, this);
        } else {
            if !scene_mode {
                let base = e.call(0x0041_81e0, &args![this]).u32();
                // Folded 004fd380 reads BGSExplosion::pImpactPlacedObject
                // (Xbox PDB), at PC +0x70.
                let impact_placed_object = e.call(0x004f_d380, &args![base]).u32();
                if impact_placed_object != 0 {
                    // Three zero pushes precede this no-argument getter and
                    // belong to 004698a0 (RET 0x20), not to the getter.
                    let worldspace = e.call(0x0057_5d70, &args![this]).u32();
                    let cell = e.call(0x008d_6f30, &args![this]).u32();
                    let rotation = e.call(0x0043_0830, &args![this]).u32();
                    let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                    let base = e.call(0x0041_81e0, &args![this]).u32();
                    let impact_placed_object = e.call(0x004f_d380, &args![base]).u32();
                    let data_handler = e.global::<u32>(0x011c_3f2c);
                    e.call(
                        0x0046_98a0,
                        &args![
                            data_handler,
                            impact_placed_object,
                            position,
                            rotation,
                            cell,
                            worldspace,
                            0u32,
                            0u32,
                            0u32
                        ],
                    );
                }

                let base = e.call(0x0041_81e0, &args![this]).u32();
                let radiation_radius = fn_009ad5a0(e, Ptr::new(base));
                let zero = e.global::<f64>(0x0101_2060);
                if radiation_radius as f64 > zero {
                    let base = e.call(0x0041_81e0, &args![this]).u32();
                    let radiation = e.call(0x008d_01e0, &args![base]).f32();
                    let zero = e.global::<f64>(0x0101_2060);
                    if radiation as f64 > zero {
                        fn_009b13a0(e, this);
                        let base = e.call(0x0041_81e0, &args![this]).u32();
                        let radius = fn_009ad5a0(e, Ptr::new(base));
                        let extra = e.call(0x005d_43c0, &args![this]).u32();
                        e.call(0x0042_2220, &args![extra, radius]);
                        e.vcall(this.addr(), 0x48, &args![0x4000_0000u32]);
                    }
                }
            }

            explosion_attach_light(e, this);
            e.vcall(this.addr(), 0x1c4, &args![]);
            let cell = e.call(0x008d_6f30, &args![this]).u32();
            if cell != 0 {
                if !scene_mode {
                    e.call(0x0054_ca90, &args![cell, this]);
                }
                e.call(0x0054_8230, &args![cell, this, 0u32]);
            }

            let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
            if node != 0 {
                let threshold = e.call(0x0040_3e20, &args![0x011d_0520u32]).u32();
                let close = e.mem.f32(threshold) < power && e.mem.u8(this.addr() + 0xd8) == 0;
                e.call(0x0045_0f90, &args![node, u32::from(close)]);
                if e.call(0x0054_97a0, &args![cell]).u32() != 0 {
                    let scene = e.call(0x0054_97a0, &args![cell]).u32();
                    e.vcall(scene, 0xdc, &args![node, 1u32]);
                }
            }
            let scene = if node == 0 {
                0
            } else {
                e.vcall(node, 0xc, &args![]).u32()
            };
            let object = if scene == 0 {
                0
            } else {
                e.vcall(scene, 0x10, &args![]).u32()
            };
            if object != 0 {
                e.call(0x0047_6ab0, &args![object]);
            }
            let age = e
                .call(0x004b_6220, &args![node, 0x0102_054cu32, 0x0103_275cu32])
                .f32();
            e.mem.set_f32(this.addr() + 0x90, quiet_f32(age));
        }

        if !scene_mode {
            let actor = e.vcall(this.addr(), 0x168, &args![]).u32();
            if actor != 0 {
                let process = e.call(0x0055_9450, &args![actor]).u32();
                if process != 0 {
                    let process = e.call(0x0055_9450, &args![actor]).u32();
                    if e.call(0x008d_8520, &args![process]).u32() != 0 {
                        let process = e.call(0x0055_9450, &args![actor]).u32();
                        let saved = e.call(0x008d_8520, &args![process]).u32();
                        let exploded = fn_009ad610(e, this);
                        let base = e.call(0x0041_81e0, &args![this]).u32();
                        // The exploded reference and 2 were pushed before
                        // this no-argument getter; both belong to slot FC.
                        let sound_level = e.call(0x004f_b810, &args![base]).u32();
                        let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                        let xyz = [
                            e.mem.u32(position),
                            e.mem.u32(position + 4),
                            e.mem.u32(position + 8),
                        ];
                        e.vcall(
                            saved,
                            0xfc,
                            &[
                                process,
                                xyz[0],
                                xyz[1],
                                xyz[2],
                                sound_level,
                                2,
                                exploded.addr(),
                            ],
                        );
                    }
                }
            }

            explosion_start_sounds(e, this);
            let first_valid = e.call(0x00ad_8ce0, &args![this.addr() + 0xac]).bool();
            if first_valid || e.call(0x00ad_8ce0, &args![this.addr() + 0xb8]).bool() {
                let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                let setting = e.call(0x0040_3e20, &args![0x011d_0280u32]).u32();
                let shake =
                    ((e.mem.f32(this.addr() + 0x94) as f64) * (e.mem.f32(setting) as f64)) as f32;
                e.call(0x008d_1960, &args![shake, position]);
            }

            let cell = e.call(0x008d_6f30, &args![this]).u32();
            if cell != 0 && e.call(0x004f_b970, &args![this]).bool() {
                let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                let mut xyz = [
                    e.mem.u32(position),
                    e.mem.u32(position + 4),
                    e.mem.u32(position + 8),
                ];
                let cell = e.call(0x008d_6f30, &args![this]).u32();
                xyz[2] = e.call(0x0054_71e0, &args![cell]).f32().to_bits();
                let radius = e.call(0x0064_4930, &args![this]).f32();
                let setting = e.call(0x0040_3e20, &args![0x011d_0da4u32]).u32();
                let effect_radius = ((radius as f64) / e.mem.f32(setting) as f64) as f32;
                let matrix = Ptr::<()>::new(frame.addr() + 0x54);
                e.call(0x0068_15c0, &args![matrix]);
                let heading = quiet_f32(e.call(0x004a_4240, &args![]).f32());
                e.call(0x004a_0c90, &args![matrix, heading]);
                // MOVSD.REP captures the by-value matrix before either
                // callback below can alter its stack storage.
                let matrix_words: Vec<_> = (0..9)
                    .map(|index| e.mem.u32(matrix.addr() + index * 4))
                    .collect();
                let particle = e.call(0x0040_3df0, &args![0x011d_0ce4u32]).u32();
                let player = e.global::<u32>(0x011d_ea3c);
                let world = e.call(0x008d_6f30, &args![player]).u32();
                let mut effect_args = vec![world, 1.0f32.to_bits(), particle];
                effect_args.extend(matrix_words);
                effect_args.extend(xyz);
                effect_args.extend([effect_radius.to_bits(), 7, 0]);
                e.call(0x0068_9210, &effect_args);

                let handle = Ptr::<()>::new(frame.addr() + 0x38);
                e.call(0x0041_a250, &args![handle]);
                let audio = e.call(0x0045_3a70, &args![]).u32();
                let temporary = Ptr::<()>::new(frame.addr() + 4);
                let sound = e
                    .call(
                        0x00ad_7550,
                        &args![audio, temporary, 0x0108_f10cu32, 0x102u32],
                    )
                    .u32();
                e.call(0x0041_8900, &args![handle, sound]);
                e.call(0x0048_3710, &args![temporary]);
                if e.call(0x00ad_8ce0, &args![handle]).bool() {
                    let sound_xyz = xyz.map(|bits| quiet_f32(f32::from_bits(bits)).to_bits());
                    e.call(
                        0x00ad_8b60,
                        &[handle.addr(), sound_xyz[0], sound_xyz[1], sound_xyz[2]],
                    );
                    e.call(0x00ad_8830, &args![handle, 0u32]);
                }
                e.call(0x0048_3710, &args![handle]);
            }
        }

        let debug_setting = e.call(0x0043_d4d0, &args![0x011d_0a78u32]).u32();
        if e.mem.i32(debug_setting) > 0 {
            let color = Ptr::<()>::new(frame.addr() + 0x74);
            let alpha = quiet_f32(e.global::<f32>(0x0102_3934));
            e.call(0x0041_4430, &args![color, 1.0f32, 1.0f32, 1.0f32, alpha]);
            let size = quiet_f32(e.global::<f32>(0x0101_84f4));
            let point = e.call(0x004b_29b0, &args![size, color, 1u32]).u32();
            e.call(0x00a5_a040, &args![point]);
            let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
            e.call(0x0044_0460, &args![point, position]);
            let duration = quiet_f32(e.global::<f32>(0x0101_7b78));
            let tes = e.global::<u32>(0x011d_ea10);
            e.call(0x0045_8e20, &args![tes, point, duration]);
        }

        if needs_process {
            e.call(
                0x0096_d450,
                &args![0x011e_0e80u32, this, 0u32, 0u32, 0u32, 0u32],
            );
        }
        let script_setting = e.call(0x0040_8d60, &args![0x011c_7ae8u32]).u32();
        if e.mem.u8(script_setting) != 0 {
            let value = Ptr::<()>::new(frame.addr());
            e.mem.set_u32(value.addr(), this.addr());
            e.call(0x0076_b660, &args![0x011c_7b8cu32, value]);
        }
    });
}

// Translated from 009ad670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::Init3D` (Xbox PDB): sets the loaded scene node and its scale,
/// builds debug blast-radius nodes and liquid actions when enabled, refreshes
/// the cell's Havok world, and starts the node's animation. The clone loop
/// rereads the debug setting each iteration, as the PC code does.
// open: compiler SEH cleanup for allocated shapes, actions, rigid-body cinfo,
// and cloning state on exceptions is not represented by the engine runtime.
// Translation incomplete: 009ad670 - compiler SEH cleanup for stack temporaries
pub fn explosion_init_3d(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x1c8, &args![0u32]);
    let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    if node == 0 {
        return;
    }

    let mut scale = 1.0f32;
    if fn_009ad5f0(e, this) {
        let scene = e.vcall(node, 0xc, &args![]).u32();
        let fade = if scene == 0 {
            0
        } else {
            e.vcall(scene, 0x10, &args![]).u32()
        };
        if fade != 0 {
            let range = quiet_f32(e.global::<f32>(0x0101_6970));
            e.call(0x00b4_dfd0, &args![fade, range, range]);
        }
    }

    let debug_count = e.call(0x0043_d4d0, &args![0x011d_0fe4u32]).u32();
    if e.mem.i32(debug_count) == 0 {
        let radius_node = e.call(0x004a_ae30, &args![node, 0x0108_f1a8u32]).u32();
        if radius_node != 0 {
            let radius = e.call(0x0064_4930, &args![this]).f64();
            let world_radius = e.call(0x0043_c490, &args![radius_node]).u32();
            let world_radius = e.call(0x0045_7990, &args![world_radius]).f64();
            scale = quiet_f32((radius / world_radius) as f32);
        } else {
            let base = e.call(0x0041_81e0, &args![this]).u32();
            let display = if e.call(0x0047_4cb0, &args![base]).u32() == 0 {
                let base = e.call(0x0041_81e0, &args![this]).u32();
                e.call(0x0040_8da0, &args![base + 0x30]).u32()
            } else {
                let base = e.call(0x0041_81e0, &args![this]).u32();
                e.vcall(base, 0x130, &args![]).u32()
            };
            let base = e.call(0x0041_81e0, &args![this]).u32();
            let id = e.call(0x0084_e3a0, &args![base]).u32();
            e.call(0x005b_5e40, &args![0x0108_f16cu32, id, display]);
        }
    }
    e.call(0x0044_0490, &args![node, quiet_f32(scale)]);
    let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
    e.call(0x0044_0460, &args![node, position]);

    e.with_stack(0x240, |e, frame| {
        let frame = frame.addr();
        let rotation = frame + 0x1d0;
        let angles = e.call(0x0043_0830, &args![this]).u32();
        let xyz = [
            quiet_f32(f32::from_bits(e.mem.u32(angles))).to_bits(),
            quiet_f32(f32::from_bits(e.mem.u32(angles + 4))).to_bits(),
            quiet_f32(f32::from_bits(e.mem.u32(angles + 8))).to_bits(),
        ];
        e.call(0x0068_15c0, &args![rotation]);
        e.call(0x00a5_9780, &args![rotation, xyz[2], xyz[1], xyz[0]]);
        e.call(0x0043_fa80, &args![node, rotation]);

        let debug_count = e.call(0x0043_d4d0, &args![0x011d_0fe4u32]).u32();
        if e.mem.i32(debug_count) > 1 {
            let large_scale = quiet_f32(e.global::<f32>(0x011a_5810));
            let base = e.call(0x0041_81e0, &args![this]).u32();
            e.call(0x006d_3b00, &args![base, large_scale]);
            let lifetime = quiet_f32(e.global::<f32>(0x011a_580c));
            e.mem.set_f32(this.addr() + 0x8c, lifetime);

            let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
            for word in 0..3 {
                let component = e.mem.u32(position + word * 4);
                e.mem.set_u32(frame + 0x1c4 + word * 4, component);
            }
            let name = e.call(0x0044_8a80, &args![]).u32();
            let extra = e.call(0x00a5_bdd0, &args![node, name]).u32();
            if extra != 0 {
                e.call(0x0051_9230, &args![extra, 0x10u32, 1u32]);
            }

            let mut index = 0i32;
            loop {
                let debug_count = e.call(0x0043_d4d0, &args![0x011d_0fe4u32]).u32();
                if index >= e.mem.i32(debug_count) {
                    break;
                }
                let cloning = frame + 0x19c;
                e.call(0x004a_d050, &args![cloning, 1.0f32]);
                let raw = e.call(0x00aa_13e0, &args![0xacu32]).u32();
                let collision = if raw == 0 {
                    0
                } else {
                    e.call(0x00a5_ecb0, &args![raw, 0u32]).u32()
                };

                let random_coordinate = |e: &mut Engine| {
                    let source = e.call(0x0047_6c00, &args![]).u32();
                    let random = e.call(0x004d_fec0, &args![source]).f64();
                    random * e.global::<f64>(0x0102_0758)
                };
                let x = quiet_f32((random_coordinate(e) + e.global::<f64>(0x0102_f070)) as f32);
                let y = quiet_f32(random_coordinate(e) as f32);
                let z = quiet_f32(random_coordinate(e) as f32);
                e.call(0x004b_c1f0, &args![collision, z, y, x]);

                let raw = e.call(0x00aa_13e0, &args![0x14u32]).u32();
                let collision_object = if raw == 0 {
                    0
                } else {
                    e.call(0x0056_d580, &args![raw, collision]).u32()
                };
                let raw = e.call(0x00aa_13e0, &args![0x14u32]).u32();
                let shape = if raw == 0 {
                    0
                } else {
                    let radius = quiet_f32(e.global::<f32>(0x0102_918c));
                    e.call(0x0056_e950, &args![raw, radius, 1u32]).u32()
                };

                let cinfo = frame + 0x80;
                e.call(0x00c8_f510, &args![cinfo]);
                let damping = quiet_f32(e.global::<f32>(0x0101_712c));
                e.call(0x0050_7610, &args![cinfo, damping]);
                e.call(0x006e_4340, &args![cinfo, 0.0f32]);
                e.call(0x0056_d360, &args![cinfo, 1u32]);
                e.call(0x0050_76c0, &args![cinfo, 1.0f32]);
                let seed = e.call(0x005d_be10, &args![]).u32();
                let string = e
                    .call(0x0062_2290, &args![cinfo - 4, 4u32, seed, 0u32])
                    .u32();
                let string_value = e.call(0x0055_9450, &args![string]).u32();
                e.call(0x0056_f0f0, &args![cinfo, string_value]);
                e.call(0x0056_f110, &args![cinfo, shape]);

                let raw = e.call(0x00aa_13e0, &args![0x1cu32]).u32();
                let body = if raw == 0 {
                    0
                } else {
                    e.call(0x0056_d380, &args![raw, cinfo]).u32()
                };
                e.call(0x00c6_b980, &args![collision_object, body]);
                e.vcall(node, 0xdc, &args![collision, 1u32]);

                let raw = e.call(0x00aa_13e0, &args![0xb8u32]).u32();
                let value_node = if raw == 0 {
                    0
                } else {
                    e.call(0x00c4_3660, &args![raw]).u32()
                };
                e.call(0x0054_68d0, &args![value_node, 0xeu32]);
                e.vcall(collision, 0xdc, &args![value_node, 1u32]);

                let action_data = frame + 0x44;
                fn_009ae0f0(e, Ptr::new(action_data));
                let target = e.call(0x004a_e750, &args![body]).u32();
                e.mem.set_u32(action_data + 4, target);
                for (offset, address) in [
                    (0x08, 0x011a_5808),
                    (0x0c, 0x011a_5804),
                    (0x10, 0x011a_5800),
                    (0x14, 0x011a_57fc),
                ] {
                    let value = e.global::<f32>(address);
                    e.mem.set_f32(action_data + offset, quiet_f32(value));
                }
                let raw = e.call(0x00aa_13e0, &args![0x40u32]).u32();
                let action = if raw == 0 {
                    0
                } else {
                    bhk_liquid_action_ctor(e, Ptr::new(raw), Ptr::new(action_data)).addr()
                };
                e.call(0x00c8_ef80, &args![body, action]);
                e.call(0x0056_d730, &args![cinfo]);
                e.call(0x004a_d270, &args![cloning]);
                index = index.wrapping_add(1);
            }
            refresh_explosion_havok(e, this, node, frame + 0x28);
        }

        refresh_explosion_havok(e, this, node, frame + 0x18);
        let scene = e.vcall(node, 0xc, &args![]).u32();
        if scene != 0 {
            let scene = e.vcall(node, 0xc, &args![]).u32();
            if e.call(0x0057_84b0, &args![scene]).bool() {
                let scene = e.vcall(node, 0xc, &args![]).u32();
                e.call(0x0057_8060, &args![scene]);
            }
        }

        let cell = e.call(0x008d_6f30, &args![this]).u32();
        if cell != 0 {
            let cell = e.call(0x008d_6f30, &args![this]).u32();
            let world = e.call(0x0045_43c0, &args![cell]).u32();
            if world != 0 {
                let cell = e.call(0x008d_6f30, &args![this]).u32();
                let world = e.call(0x0045_43c0, &args![cell]).u32();
                if e.call(0x004b_ae00, &args![world]).bool() {
                    let cell = e.call(0x008d_6f30, &args![this]).u32();
                    let world = e.call(0x0045_43c0, &args![cell]).u32();
                    e.vcall(world, 0xd8, &args![node, 1u32]);
                    e.call(0x004b_6f90, &args![node]);
                }
            }
        }

        let mut controller = e.call(0x0043_b230, &args![node]).u32();
        if controller == 0 {
            let scene = e.vcall(node, 0xc, &args![]).u32();
            if scene != 0 && e.call(0x0045_bc00, &args![scene, 0u32]).u32() != 0 {
                let scene = e.call(0x0043_b4a0, &args![scene, 0u32]).u32();
                controller = e.call(0x0043_b230, &args![scene]).u32();
            }
        }
        let mut manager = 0;
        while controller != 0 {
            if e.call(0x0043_b300, &args![0x011f_36acu32, controller])
                .bool()
            {
                manager = controller;
                break;
            }
            controller = e.call(0x004a_8a90, &args![controller]).u32();
        }
        if manager != 0 {
            let sequence_name = e.call(0x005c_1490, &args![]).u32();
            let sequence = e.call(0x0047_a520, &args![manager, sequence_name]).u32();
            if sequence != 0 {
                e.call(0x0048_fef0, &args![manager, 0.0f32]);
                e.call(
                    0x0047_aab0,
                    &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
                );
                e.call(0x0047_aa40, &args![manager, 1u32]);
                e.call(0x0049_54c0, &args![sequence]);
                let duration = e.call(0x0050_7e10, &args![this]).f64();
                let sequence_end = e.call(0x0050_8100, &args![sequence]).f64();
                if duration < sequence_end {
                    let sequence_end = e.call(0x0050_8100, &args![sequence]).f64();
                    e.call(0x0081_0550, &args![this, quiet_f32(sequence_end as f32)]);
                }
                return;
            }
        }
        e.call(0x00a6_d2d0, &args![node]);
    });
}

/// Repeats the game's cell/world callback sequence. Both sites construct a
/// zero point and pass it to the node helper between the world calls.
fn refresh_explosion_havok(e: &mut Engine, this: Ptr, node: u32, point: u32) {
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    if cell == 0 {
        return;
    }
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    if e.call(0x0045_43c0, &args![cell]).u32() == 0 {
        return;
    }
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    e.call(0x0045_43c0, &args![cell]);
    e.call(0x00c6_9f50, &args![node, 1u32, 1u32]);
    e.call(0x0043_d410, &args![point, 0.0f32, 0u32, 0u32]);
    e.call(0x00a5_9c60, &args![node, point]);
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let world = e.call(0x0045_43c0, &args![cell]).u32();
    e.vcall(world, 0xd0, &args![node, 1u32, 1u32, 0u32, 1u32]);
}

// Translated from 009ad5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSExplosion::GetRadiationRadiusBSUnits` (Xbox PDB), using the same
/// unit-flag branch as the image-space radius accessor.
pub fn fn_009ad5a0(e: &mut Engine, this: Ptr) -> f32 {
    if e.call(0x0047_7950, &args![this, 1u32]).bool() {
        return e.mem.f32(this.addr() + 0xa0);
    }
    let setting = e.call(0x0040_3e20, &args![0x011d_1218u32]).ptr::<u32>();
    ((e.mem.f32(this.addr() + 0xa0) as f64) * (e.mem.f32(setting.addr()) as f64)) as f32
}

// Translated from 009ad5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns true when the cell pointer returned by the callee is null.
pub fn fn_009ad5f0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x008d_6f30, &args![this]).u32() == 0
}

// Translated from 009ad610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the 32-bit word at `this+0xCC`.
pub fn fn_009ad610(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0xcc))
}

// Translated from 009ae0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a four-float point record from executable constants.
pub fn fn_009ae0f0(e: &mut Engine, this: Ptr) {
    e.call(0x0053_7bb0, &args![this]);
    for (offset, address) in [
        (8, 0x0103_0020),
        (12, 0x0101_e580),
        (16, 0x0101_e6ec),
        (20, 0x0101_e580),
    ] {
        let value: f32 = e.global(address);
        e.mem.set_f32(this.addr() + offset, value);
    }
}

// Translated from 009ae240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLiquidAction::GetRTTI` (Xbox PDB): returns its RTTI descriptor address.
pub fn fn_009ae240(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(0x0126_8420)
}

// Translated from 009ae250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLiquidAction` (Xbox PDB) scalar deleting destructor wrapper; the
/// allocation size passed to the engine's delete helper is 0x40.
pub fn fn_009ae250(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00cb_c920, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x40u32]);
    }
    this
}

// Translated from 009ae280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the reference's Havok phantom from a 0x60-byte creation record,
/// then attaches it to the reference's current Havok world. PC stack slots put
/// the CString object at `cinfo-0x14`, its pointer at cinfo+0, the resolved
/// shape at cinfo+4, and the matrix at cinfo+0x20. The C++ SEH cleanup path is
/// not translated.
// open: compiler SEH cleanup for the temporary CString and creation record.
// Translation incomplete: 009ae280 - compiler SEH cleanup for stack temporaries
pub fn fn_009ae280(e: &mut Engine, this: Ptr) {
    if !e.call(0x0082_2510, &args![this.addr() + 0xa0, 0u32]).bool() {
        return;
    }
    if e.call(0x008d_6f30, &args![this]).ptr::<()>().is_null() {
        return;
    }
    let parent = e.call(0x008d_6f30, &args![this]).ptr::<()>();
    if e.call(0x0045_43c0, &args![parent]).ptr::<()>().is_null() {
        return;
    }

    e.with_stack(0xb0, |e, frame| {
        let cinfo = Ptr::<()>::new(frame.addr() + 0x30);
        let text_object = Ptr::<()>::new(frame.addr() + 0x1c);
        e.call(0x0056_e090, &args![cinfo]);

        let sphere = e.call(0x00aa_13e0, &args![0x14u32]).ptr::<()>();
        let sphere_shape = if sphere.is_null() {
            Ptr::NULL
        } else {
            let radius = e.call(0x0064_4930, &args![this]).f32();
            e.call(0x0056_e950, &args![sphere, radius, 1u32])
                .ptr::<()>()
        };
        // Native calls operatorP even when the sphere allocation returned null.
        let shape = e.call(0x004a_e750, &args![sphere_shape]).ptr::<()>();
        e.mem.set_u32(cinfo.addr() + 4, shape.addr());

        let text_seed = e.call(0x005d_be10, &args![]).ptr::<()>();
        let text = e
            .call(0x0062_2290, &args![text_object, 0x29u32, text_seed, 0u32])
            .ptr::<()>();
        let text = e.call(0x0055_9450, &args![text]).ptr::<()>();
        e.mem.set_u32(cinfo.addr(), text.addr());

        if !e.vcall(this.addr(), 0x1d0, &args![]).ptr::<()>().is_null() {
            // The original reloads the virtual target and invokes it again.
            let node = e.vcall(this.addr(), 0x1d0, &args![]).ptr::<()>();
            let rotation = e.call(0x0046_1130, &args![node]).ptr::<()>();
            fn_009ae4a0(e, Ptr::<()>::new(cinfo.addr() + 0x20), rotation);
        } else {
            e.call(0x0056_e120, &args![cinfo.addr() + 0x20]);
        }

        let allocation = e.call(0x00aa_13e0, &args![0x18u32]).ptr::<()>();
        let phantom = if allocation.is_null() {
            Ptr::NULL
        } else {
            e.call(0x0056_e2d0, &args![allocation, cinfo]).ptr::<()>()
        };
        e.call(0x0066_b0d0, &args![this.addr() + 0xa0, phantom]);
        let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).ptr::<()>();
        let parent = e.call(0x008d_6f30, &args![this]).ptr::<()>();
        let world = e.call(0x0045_43c0, &args![parent]).ptr::<()>();
        e.vcall(phantom.addr(), 0x9c, &args![world]);

        e.call(0x0056_ea90, &args![cinfo]);
    });
}

// Translated from 009ae540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies three floats from `source` and writes zero to the fourth component.
pub fn fn_009ae540(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    for offset in [0, 4, 8] {
        let bits = e.mem.u32(source.addr() + offset);
        // FLD/FSTP quiets signaling NaNs while preserving sign and payload.
        let bits = if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
            bits | 0x0040_0000
        } else {
            bits
        };
        e.mem.set_u32(this.addr() + offset, bits);
    }
    e.mem.set_f32(this.addr() + 0x0c, 0.0);
    this
}

// Translated from 009ae580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a setting-derived step to `this+0x88` and forwards it through the
/// object's virtual update slot at byte offset 0x2F8.
pub fn fn_009ae580(e: &mut Engine, this: Ptr, _unused_1: f32) {
    let delta = e.call(0x0084_d030, &args![0x011f_6394u32]).f32();
    e.vcall(this.addr(), 0x2f8, &args![delta]);
    let old = e.mem.f32(this.addr() + 0x88);
    e.mem
        .set_f32(this.addr() + 0x88, (old as f64 + delta as f64) as f32);
}

// Translated from 009ae5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// On the first frame, reports the first target to the process-list callback.
/// Processes targets after the explosion's age advances, updates radiation
/// and scale while not underwater, and asks the base explosion to finish when
/// both expiration comparisons pass.
pub fn fn_009ae5d0(e: &mut Engine, this: Ptr, _unused_1: u32) {
    if e.mem.u32(this.addr() + 0xe4) == 0 {
        let mut target = 0;
        let targets = this.addr() + 0xa4;
        if !e.call(0x0082_56d0, &args![targets]).bool() {
            let value = e.call(0x0068_15c0, &args![targets]).u32();
            let record = e.mem.u32(value);
            target = e.mem.u32(record);
        }
        let owner = fn_009acce0(e, this).addr();
        e.call(0x008d_0600, &args![0x011e_0e80u32, owner, target]);
    }

    let age = e.mem.f32(this.addr() + 0x90);
    let now = e.call(0x0051_b8e0, &args![this]).f32();
    if now > age && e.mem.u8(this.addr() + 0xd5) == 0 {
        e.call(0x009a_fcc0, &args![this]);
    }

    let frame_count = e.mem.u32(this.addr() + 0xe4).wrapping_add(1);
    e.mem.set_u32(this.addr() + 0xe4, frame_count);
    if !fn_009ad5f0(e, this) {
        fn_009b13a0(e, this);
        fn_009b1320(e, this);
    }

    let now = e.call(0x0051_b8e0, &args![this]).f32();
    let expiration = e.call(0x0050_7e10, &args![this]).f32();
    if expiration < now {
        let now = e.call(0x0051_b8e0, &args![this]).f32();
        let base = e.call(0x0041_81e0, &args![this]).u32();
        let base_expiration = e.call(0x004e_3cc0, &args![base]).f32();
        if base_expiration < now {
            fn_009ac9a0(e, this);
        }
    }
}

// Translated from 009afbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor wrapper for the Havok contact collector.
pub fn fn_009afbb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x009a_fc10, &args![this]);
    if flags & 1 != 0 {
        e.call(0x009a_fbe0, &args![this]);
    }
    this
}

// Translated from 009afc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Invokes the collector base destructor.
pub fn fn_009afc10(e: &mut Engine, this: Ptr) {
    e.call(0x0062_1100, &args![this]);
}

// Translated from 009afc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the buffer pointer at `this+8` through its accessor. The caller
/// at 009ae865 adds a contact index times 0x70 to this return value.
pub fn fn_009afc30(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0045_8b50, &args![this.addr() + 8]).ptr()
}

// Translated from 009afc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the embedded count at `this+0x14` through its accessor.
pub fn fn_009afc50(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0045_8b50, &args![this.addr() + 0x14]).u32()
}

// Translated from 009afc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores whether the embedded signed count at `this+0x14` is positive into
/// the byte pointed to by `value`, then returns that pointer unchanged.
pub fn fn_009afc70(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let count = e.call(0x0045_8b50, &args![this.addr() + 0x14]).i32();
    e.call(0x0062_2570, &args![value, u32::from(count > 0)]);
    value
}

// Translated from 009afca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the function pointer stored at 0x01268430 with `(point, this)`.
pub fn fn_009afca0(e: &mut Engine, this: Ptr, point: Ptr) {
    let target = e.global::<u32>(0x0126_8430);
    e.call(target, &args![point, this]);
}

// Translated from 009abf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::Explosion` (Xbox PDB), PC size 0x104 versus Xbox 0x114.
/// Initializes the base, members and defaults.
// open: compiler SEH cleanup after constructor failure is not modeled.
// Translation incomplete: 009abf30 - compiler SEH cleanup after constructor failure
pub fn explosion_ctor(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0092_eb50, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_ee04);
    e.mem.set_u32(this.addr() + 0x18, 0x0108_edf8);
    e.call(0x0063_3c90, &args![this.addr() + 0xa0, 0u32]);
    e.call(0x0096_a2d0, &args![this.addr() + 0xa4]);
    e.call(0x0041_a250, &args![this.addr() + 0xac]);
    e.call(0x0041_a250, &args![this.addr() + 0xb8]);
    e.call(0x0063_3c90, &args![this.addr() + 0xc4, 0u32]);
    e.call(0x0042_f730, &args![this.addr() + 0xd0, 0u32]);
    for offset in [0xe8, 0xf4] {
        e.call(0x0068_15c0, &args![this.addr() + offset]);
    }
    for offset in [0x68, 0xe4, 0x88, 0x8c, 0x90, 0x94, 0x98] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    e.mem.set_f32(this.addr() + 0x9c, 1.0);
    for offset in [0xa0, 0xc4] {
        e.call(0x0066_b0d0, &args![this.addr() + offset, 0u32]);
    }
    e.call(0x0042_f780, &args![this.addr() + 0xd0, 0u32]);
    for offset in [0xc8, 0xcc, 0xe0] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    for offset in 0xd4..=0xd9 {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    e.mem.set_u32(this.addr() + 0xdc, 0);
    for offset in [0xe8, 0xf4] {
        for component in [0, 4, 8] {
            let bits = e.global::<u32>(0x011f_426c + component);
            e.mem.set_u32(this.addr() + offset + component, bits);
        }
    }
    let damage = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(this.addr() + 0x100, damage);
    this
}

// Translated from 009ad630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::GetExplosionInFace` (Xbox PDB): lazy lookup and RTTI cast.
pub fn tes_image_space_modifier_get_explosion_in_face(e: &mut Engine) -> Ptr {
    if e.global::<u32>(0x011c_9708) == 0 {
        let form = e.call(0x0048_39c0, &args![0x166u32]).u32();
        let modifier = e
            .call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_62b8u32, 0u32],
            )
            .u32();
        e.set_global(0x011c_9708, modifier);
    }
    Ptr::new(e.global(0x011c_9708))
}

// Translated from 009ae140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLiquidAction::bhkLiquidAction` (Xbox PDB): initializes the 0x40-byte
/// action and increments its live counter.
// open: compiler SEH cleanup after constructor failure is not modeled.
// Translation incomplete: 009ae140 - compiler SEH cleanup after constructor failure
pub fn bhk_liquid_action_ctor(e: &mut Engine, this: Ptr, info: Ptr) -> Ptr {
    e.call(0x004b_50c0, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_f1bc);
    for offset in [0x24, 0x30] {
        e.call(0x0068_15c0, &args![this.addr() + offset]);
    }
    for offset in [0x10, 0x14, 0x18, 0x1c, 0x20] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    for offset in [0x24, 0x30] {
        for component in [0, 4, 8] {
            let bits = e.global::<u32>(0x011f_426c + component);
            e.mem.set_u32(this.addr() + offset + component, bits);
        }
    }
    e.mem.set_u8(this.addr() + 0x3c, 1);
    e.call(0x00cb_d520, &args![this, info]);
    let count = e.global::<u32>(0x0126_841c).wrapping_add(1);
    e.set_global(0x0126_841c, count);
    this
}

// Translated from 009ae4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the rotation rows and converts the translation at source+0x24.
pub fn fn_009ae4a0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    fn_009ae4d0(e, this, source);
    e.call(
        0x004b_4e50,
        &args![this.addr() + 0x30, source.addr() + 0x24],
    );
    this
}

// Translated from 009ae4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Extracts three rotation rows into four-float vectors with zero fourth lanes.
pub fn fn_009ae4d0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.with_stack(12, |e, row| {
        e.call(0x0068_15c0, &args![row]);
        for index in 0..3u32 {
            e.call(0x0043_9f50, &args![source, index, row]);
            fn_009ae540(e, Ptr::new(this.addr() + index * 16), row.cast());
        }
    });
    this
}

// Translated from 009afaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the fixed-buffer contact collector.
// open: compiler SEH cleanup after constructor failure is not modeled.
// Translation incomplete: 009afaf0 - compiler SEH cleanup after constructor failure
pub fn fn_009afaf0(e: &mut Engine, this: Ptr, buffer: u32, capacity: u32) -> Ptr {
    e.call(0x0062_1060, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_f288);
    for offset in [8, 12, 16, 20] {
        e.call(0x0068_15c0, &args![this.addr() + offset]);
    }
    for (offset, value) in [(8, buffer), (12, buffer), (16, capacity), (20, 0)] {
        e.call(0x0053_7e90, &args![this.addr() + offset, value]);
    }
    this
}

// Translated from 009afbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a non-null collector through the thread allocator, with size 0x18.
/// This is a cdecl one-argument delete, not a thiscall member.
pub fn fn_009afbe0(e: &mut Engine, allocation: Ptr) {
    if !allocation.is_null() {
        let router = e.call(0x00c8_5750, &args![]).u32();
        let allocator = e.call(0x0044_edb0, &args![router]).u32();
        e.vcall(allocator, 8, &args![allocation, 0x18u32]);
    }
}

// Translated from 009afed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the underlying Havok object's embedded array at +0x160.
pub fn fn_009afed0(e: &mut Engine, this: Ptr) -> Ptr {
    let object = e.call(0x004a_e750, &args![this]).u32();
    e.call(0x0062_d2f0, &args![object]).ptr()
}

// Translated from 009b0900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns Explosion::pWeaponSource (Xbox PDB), PC +0xE0.
pub fn fn_009b0900(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0xe0))
}

// Translated from 009b0bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float at +0x74 from the caller's object.
pub fn fn_009b0bd0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x74)
}

// Translated from 009b0bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts the supplied vector into Havok units before applying linear impulse.
pub fn fn_009b0bf0(e: &mut Engine, this: Ptr, vector: Ptr) {
    e.with_stack(16, |e, converted| {
        e.call(0x0068_15c0, &args![converted]);
        let result = e.call(0x004a_3e00, &args![converted, vector]).ptr();
        fn_009b0c50(e, this, result);
    });
}

// Translated from 009b0c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies a linear impulse only when the wrapped body exists.
pub fn fn_009b0c50(e: &mut Engine, this: Ptr, impulse: Ptr) {
    let body = e.call(0x004a_e750, &args![this]).u32();
    if body != 0 {
        e.call(0x00a2_9680, &args![this]);
        e.call(0x0081_82d0, &args![body, impulse]);
        e.call(0x00a2_9680, &args![this]);
    }
}

// Translated from 009b0c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts the supplied vector before forwarding to the activated body's motion.
pub fn fn_009b0c90(e: &mut Engine, this: Ptr, vector: Ptr) {
    e.with_stack(16, |e, converted| {
        e.call(0x0068_15c0, &args![converted]);
        let result = e.call(0x004a_3e00, &args![converted, vector]).ptr();
        fn_009b0cf0(e, this, result);
    });
}

// Translated from 009b0cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to the body's motion only when the wrapped body exists.
pub fn fn_009b0cf0(e: &mut Engine, this: Ptr, impulse: Ptr) {
    let body = e.call(0x004a_e750, &args![this]).u32();
    if body != 0 {
        e.call(0x00a2_9680, &args![this]);
        fn_009b0d30(e, Ptr::new(body), impulse);
        e.call(0x00a2_9680, &args![this]);
    }
}

// Translated from 009b0d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Activates the entity and invokes its motion's virtual slot +0x54.
pub fn fn_009b0d30(e: &mut Engine, this: Ptr, impulse: Ptr) {
    e.call(0x00c9_c1d0, &args![this]);
    let motion = e.call(0x009d_9f40, &args![this]).u32();
    e.vcall(motion, 0x54, &args![impulse]);
}

// Translated from 009b0d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the explosion position to the target's process above the setting threshold.
pub fn fn_009b0d70(e: &mut Engine, this: Ptr, target: Ptr, magnitude: f32) {
    if target.is_null() || e.call(0x008d_8520, &args![target]).u32() == 0 {
        return;
    }
    let setting = e.call(0x0040_3e20, &args![0x011d_1188u32]).u32();
    if e.mem.f32(setting) < magnitude {
        let process = e.call(0x008d_8520, &args![target]).u32();
        // 009b0dc2: the explosion's +0x1f4 takes no stack word (plain RET in every
        // reference class); the 0.0 pushed before it is the last word of the process'
        // +0x418 (RET 0x14).
        let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let words = [
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        ];
        e.vcall(
            process,
            0x418,
            &args![target, words[0], words[1], words[2], 0.0f32],
        );
    }
}

// Translated from 009b0f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes an explosion target and its embedded list at +8.
pub fn fn_009b0f40(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0096_a2d0, &args![this.addr() + 8]);
    e.mem.set_u32(this.addr(), 0);
    for offset in 4..=6 {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    this
}

// Translated from 009b11c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::StopSounds` (Xbox PDB): stops both valid handles, then rechecks
/// validity before releasing each one.
pub fn explosion_stop_sounds(e: &mut Engine, this: Ptr) {
    for offset in [0xac, 0xb8] {
        let handle = this.addr() + offset;
        if e.call(0x00ad_8ce0, &args![handle]).bool() {
            e.call(0x00ad_88f0, &args![handle]);
            if e.call(0x00ad_8ce0, &args![handle]).bool() {
                e.call(0x00ad_8d10, &args![handle]);
            }
        }
    }
}

// Translated from 009b1260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::AttachLight` (Xbox PDB): creates a dynamic light when both 3D
/// and the base explosion's light form exist, then assigns spLight.
pub fn explosion_attach_light(e: &mut Engine, this: Ptr) {
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() == 0 {
        return;
    }
    let base = e.call(0x0041_81e0, &args![this]).u32();
    if e.call(0x004f_b070, &args![base]).u32() == 0 {
        return;
    }
    let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    let base = e.call(0x0041_81e0, &args![this]).u32();
    let form = e.call(0x004f_b070, &args![base]).u32();
    let light = e.call(0x0050_d810, &args![form, this, node, 0u32]).u32();
    e.call(0x0066_b0d0, &args![this.addr() + 0xc4, light]);
}

// Translated from 009b12d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes a non-null spLight from the shadow scene and clears the smart pointer.
pub fn fn_009b12d0(e: &mut Engine, this: Ptr) {
    let member = this.addr() + 0xc4;
    if e.call(0x0055_9450, &args![member]).u32() != 0 {
        let light = e.call(0x0055_9450, &args![member]).u32();
        let scene = e.call(0x0045_0b80, &args![0u32]).u32();
        e.call(0x00b5_eed0, &args![scene, light]);
        e.call(0x0066_b0d0, &args![member, 0u32]);
    }
}
// Translated from 009b1090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::StartSounds` (Xbox PDB): replaces the two sound handles from
/// their base-form records.
// open: compiler SEH cleanup after sound-handle construction failure is not modeled.
// Translation incomplete: 009b1090 - compiler SEH cleanup after sound-handle construction failure
pub fn explosion_start_sounds(e: &mut Engine, this: Ptr) {
    explosion_stop_sounds(e, this);
    for (getter, offset) in [(0x004f_d3c0, 0xac), (0x0091_85e0, 0xb8)] {
        let base = e.call(0x0041_81e0, &args![this]).u32();
        if e.call(getter, &args![base]).u32() != 0 {
            let base = e.call(0x0041_81e0, &args![this]).u32();
            let sound = e.call(getter, &args![base]).u32();
            let id = e.call(0x0084_e3a0, &args![sound]).u32();
            e.with_stack(12, |e, handle| {
                let result = e
                    .call(
                        0x0093_3150,
                        &args![this, handle, id, 0u32, 0x4000_0182u32, 1u32],
                    )
                    .u32();
                e.call(0x0041_8900, &args![this.addr() + offset, result]);
                e.call(0x0048_3710, &args![handle]);
            });
        }
    }
}

// Translated from 009b1320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the current light by the age/lifetime fade, clamped through the exe helpers.
pub fn fn_009b1320(e: &mut Engine, this: Ptr) {
    let member = this.addr() + 0xc4;
    if e.call(0x0055_9450, &args![member]).u32() != 0 {
        let age = e.call(0x0051_b8e0, &args![this]).f32() as f64;
        let scaled = age * e.global::<f64>(0x0101_1590);
        let lifetime = e.call(0x0050_7e10, &args![this]).f32() as f64;
        let factor = (1.0 - scaled / lifetime) as f32;
        let factor = e.call(0x0040_ebd0, &args![1.0f32, factor]).f32();
        let factor = e.call(0x0040_4010, &args![0.0f32, factor]).f32();
        let light = e.call(0x0055_9450, &args![member]).u32();
        e.call(0x0050_dd20, &args![light, factor]);
    }
}

// Translated from 009b13a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates ExtraRadiation from positive BGSExplosionData::fRadiationRads and
/// fRadiationDissipationTime (Xbox PDB), setting the reference change flag.
pub fn fn_009b13a0(e: &mut Engine, this: Ptr) {
    let base = e.call(0x0041_81e0, &args![this]).u32();
    let rads = e.call(0x008d_01e0, &args![base]).f32();
    let base = e.call(0x0041_81e0, &args![this]).u32();
    let duration = e.call(0x004e_3cc0, &args![base]).f32();
    let zero = e.global::<f64>(0x0101_2060);
    if rads as f64 > zero && duration as f64 > zero {
        let age = e.call(0x0051_b8e0, &args![this]).f32() as f64;
        let mut scaled = ((1.0 - age / duration as f64) * rads as f64) as f32;
        // TEST AH,0x41; JP skips the replacement for unordered or strictly greater.
        if scaled as f64 <= zero {
            scaled = e.global::<f32>(0x0101_3ea4);
        }
        let member = e.call(0x005d_43c0, &args![this]).u32();
        e.call(0x0042_2350, &args![member, scaled]);
        e.vcall(this.addr(), 0x48, &args![0x4000_0000u32]);
    }
}

// Translated from 009b1450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Retains the explosion at the current process level or when its cell is loaded;
/// otherwise invokes its scalar destructor and reports false.
pub fn fn_009b1450(e: &mut Engine, this: Ptr) -> bool {
    let process = e.mem.u32(this.addr() + 0x68);
    let level = e.call(0x0045_cd60, &args![process]).u32();
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    let visible = node != 0 || {
        let manager = e.global::<u32>(0x011c_3b3c);
        e.call(0x0044_5750, &args![manager, this]).bool()
    };
    let desired = if visible && {
        let tes = e.global::<u32>(0x011d_ea10);
        e.call(0x0045_11e0, &args![tes, cell, 1u32]).bool()
    } {
        0
    } else {
        3
    };
    if level != desired && desired != 0 {
        if !this.is_null() {
            e.vcall(this.addr(), 0x10, &args![1u32]);
        }
        false
    } else {
        true
    }
}

/// FLD/FSTP quiets a signaling single-precision NaN without changing its payload.
fn quiet_f32(value: f32) -> f32 {
    let bits = value.to_bits();
    f32::from_bits(
        if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
            bits | 0x0040_0000
        } else {
            bits
        },
    )
}

// Translated from 009ac200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::Explosion_ov2` (Xbox PDB). RET 0x3c consumes three pointers,
/// the three position words and nine orientation words. PC 009ac2fc takes
/// position at EBP+0x14; 009ac310 copies the matrix from EBP+0x20;
/// 009ac544 returns with RET 0x3c.
// open: compiler SEH cleanup after constructor failure is not modeled.
// Translation incomplete: 009ac200 - compiler SEH cleanup after constructor failure
#[allow(clippy::too_many_arguments)]
pub fn explosion_ctor_ov2(
    e: &mut Engine,
    this: Ptr,
    definition: Ptr,
    owner: Ptr,
    cell: Ptr,
    x: u32,
    y: u32,
    z: u32,
    m00: u32,
    m01: u32,
    m02: u32,
    m10: u32,
    m11: u32,
    m12: u32,
    m20: u32,
    m21: u32,
    m22: u32,
) -> Ptr {
    e.call(0x0092_eb50, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_ee04);
    e.mem.set_u32(this.addr() + 0x18, 0x0108_edf8);
    e.call(0x0063_3c90, &args![this.addr() + 0xa0, 0u32]);
    e.call(0x0096_a2d0, &args![this.addr() + 0xa4]);
    for offset in [0xac, 0xb8] {
        e.call(0x0041_a250, &args![this.addr() + offset]);
    }
    e.call(0x0063_3c90, &args![this.addr() + 0xc4, 0u32]);
    e.call(0x0042_f730, &args![this.addr() + 0xd0, 0u32]);
    for offset in [0xe8, 0xf4] {
        e.call(0x0068_15c0, &args![this.addr() + offset]);
    }
    e.mem.set_u32(this.addr() + 0x68, 0);
    e.mem.set_u32(this.addr() + 0xc8, owner.addr());
    e.call(0x0093_0700, &args![this, cell]);
    e.with_stack(12, |e, position| {
        for (index, bits) in [x, y, z].into_iter().enumerate() {
            e.mem.set_u32(position.addr() + index as u32 * 4, bits);
        }
        e.call(0x0057_5830, &args![this, position]);
    });
    e.call(
        0x0056_fa90,
        &args![this, m00, m01, m02, m10, m11, m12, m20, m21, m22],
    );
    e.mem.set_u8(this.addr() + 0xd8, 0);
    if !definition.is_null() {
        e.call(0x0057_5690, &args![this, definition]);
        let base = e.call(0x0041_81e0, &args![this]).ptr::<()>();
        let radius = e.call(0x0047_7900, &args![base]).f32();
        e.mem.set_f32(this.addr() + 0x94, radius);
        let base = e.call(0x0041_81e0, &args![this]).ptr::<()>();
        let radius = quiet_f32(fn_009ac550(e, base));
        e.mem.set_f32(this.addr() + 0x98, radius);
        let base = e.call(0x0041_81e0, &args![this]).u32();
        let flag = e.call(0x0047_7950, &args![base, 0x40u32]).u8();
        e.mem.set_u8(this.addr() + 0xd8, flag);
        let base = e.call(0x0041_81e0, &args![this]).u32();
        if e.call(0x0047_7950, &args![base, 2u32]).bool() {
            let mut words = vec![this.addr()];
            for index in 0..9 {
                words.push(e.global::<u32>(0x011a_9448 + index * 4));
            }
            e.call(0x0056_fa90, &words);
        }
    }
    e.mem.set_u32(this.addr() + 0xe4, 0);
    let lifetime = quiet_f32(e.global::<f32>(0x0101_7718));
    e.mem.set_f32(this.addr() + 0x8c, lifetime);
    e.mem.set_f32(this.addr() + 0x90, 0.0);
    e.mem.set_f32(this.addr() + 0x88, 0.0);
    e.mem.set_f32(this.addr() + 0x9c, 1.0);
    for offset in [0xa0, 0xc4] {
        e.call(0x0066_b0d0, &args![this.addr() + offset, 0u32]);
    }
    e.call(0x0042_f780, &args![this.addr() + 0xd0, 0u32]);
    e.mem.set_u32(this.addr() + 0xe0, 0);
    for offset in [0xd4, 0xd5, 0xd6, 0xd7, 0xd9] {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    let caster = if !owner.is_null() && e.vcall(owner.addr(), 0x100, &args![]).bool() {
        0
    } else {
        let allocation = e.call(0x0040_1000, &args![0x24u32]).u32();
        if allocation == 0 {
            0
        } else {
            e.call(0x0082_5ad0, &args![allocation, this]).u32()
        }
    };
    e.mem.set_u32(this.addr() + 0xdc, caster);
    for offset in [0xe8, 0xf4] {
        for component in [0, 4, 8] {
            let bits = e.global::<u32>(0x011f_426c + component);
            e.mem.set_u32(this.addr() + offset + component, bits);
        }
    }
    let damage = quiet_f32(e.global::<f32>(0x0101_2054));
    e.mem.set_f32(this.addr() + 0x100, damage);
    this
}

// Translated from 009b0e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a reference's target record, setting only currently zero flag bytes,
/// or appends a newly allocated record. The native null-allocation dereference
/// is retained.
// open: compiler SEH cleanup after record/list construction failure is not modeled.
// Translation incomplete: 009b0e00 - compiler SEH cleanup after record/list construction failure
pub fn fn_009b0e00(e: &mut Engine, this: Ptr, reference: Ptr, first: u8, second: u8) -> Ptr {
    let mut node = this.addr().wrapping_add(0xa4);
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(slot);
        if record != 0 && e.mem.u32(record) == reference.addr() {
            for (offset, value) in [(4, first), (5, second)] {
                if value != 0 && e.mem.u8(record + offset) == 0 {
                    e.mem.set_u8(record + offset, value);
                }
            }
            return Ptr::new(record);
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    let allocation = e.call(0x0040_1000, &args![0x10u32]).ptr::<()>();
    let record = if allocation.is_null() {
        allocation
    } else {
        fn_009b0f40(e, allocation)
    };
    e.mem.set_u32(record.addr(), reference.addr());
    e.mem.set_u8(record.addr() + 4, first);
    e.mem.set_u8(record.addr() + 5, second);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), record.addr());
        e.call(0x005a_e3d0, &args![this.addr() + 0xa4, slot]);
        Ptr::new(e.mem.u32(slot.addr()))
    })
}

// Translated from 009b0f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes and caches explosion damage. FCOMP's C0 test also recomputes NaN.
/// Weapon actor-value scaling rounds at both explicit single-precision stores.
pub fn fn_009b0f80(e: &mut Engine, this: Ptr) -> f32 {
    let cached = e.mem.f32(this.addr() + 0x100);
    let threshold = e.global::<f64>(0x0101_2060);
    if matches!(
        (cached as f64).partial_cmp(&threshold),
        None | Some(std::cmp::Ordering::Less)
    ) {
        let base = e.call(0x0041_81e0, &args![this]).u32();
        let damage = e.call(0x0064_79e0, &args![base]).f32();
        e.mem.set_f32(this.addr() + 0x100, damage);
        let owner = fn_009acce0(e, this);
        if !owner.is_null() {
            let actor_owner = fn_009acce0(e, this);
            if e.vcall(actor_owner.addr(), 0x100, &args![]).bool() {
                // Native code checks LastBoundWeapon, then reloads pOwner and
                // calls LastBoundWeapon again before applying actor-value scaling.
                if !fn_009b0900(e, this).is_null() {
                    let owner = fn_009acce0(e, this);
                    let weapon = fn_009b0900(e, this);
                    let value = e.call(0x008d_85e0, &args![weapon]).u32();
                    let skill = e.vcall(owner.addr() + 0xa4, 0xc, &args![value]).f32();
                    let additive = e.call(0x0040_3e20, &args![0x011c_ece8u32]).u32();
                    let factor = e.call(0x0040_3e20, &args![0x011c_f350u32]).u32();
                    let multiplier =
                        ((skill as f64) * e.global::<f64>(0x0101_6408) * (e.mem.f32(factor) as f64)
                            + (e.mem.f32(additive) as f64)) as f32;
                    let result =
                        ((e.mem.f32(this.addr() + 0x100) as f64) * (multiplier as f64)) as f32;
                    e.mem.set_f32(this.addr() + 0x100, result);
                }
            }
        }
    }
    e.mem.f32(this.addr() + 0x100)
}

// Translated from 009afcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Enumerates collidables in spPhantom, finds their references, and records
/// eligible actor targets with their closest body-part hit distance. The
/// target-found flag is set after the scan even when the phantom is empty.
pub fn fn_009afcc0(e: &mut Engine, this: Ptr) {
    let player = Ptr::new(e.global::<u32>(0x011d_ea3c));
    let player_target = fn_009b0e00(e, this, player, 0, 1);
    let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
    if phantom != 0 {
        let mut index = 0u32;
        loop {
            let array = fn_009afed0(e, Ptr::new(phantom));
            let count = e.call(0x0072_6070, &args![array]).u32();
            if index >= count {
                break;
            }

            let array = fn_009afed0(e, Ptr::new(phantom));
            let collidable_slot = e.call(0x0062_9ab0, &args![array, index]).u32();
            let collidable = e.mem.u32(collidable_slot);
            let av_object = e.call(0x004b_5820, &args![collidable]).u32();
            let reference = e.call(0x0056_f930, &args![av_object]).u32();
            if reference != 0 {
                let actor = e.vcall(reference, 0x100, &args![]).bool();
                if !(actor && e.call(0x008a_ce90, &args![reference]).bool()) {
                    let collidable_slot = e.call(0x0062_9ab0, &args![array, index]).u32();
                    let collidable = e.mem.u32(collidable_slot);
                    let owner = e.call(0x004b_59f0, &args![collidable]).u32();
                    let flags = if owner == 0 {
                        0
                    } else {
                        u32::from(e.call(0x0051_7670, &args![owner]).u32() < 4)
                    };

                    let record = if reference == player.addr() {
                        player_target
                    } else if reference == e.mem.u32(this.addr() + 0xc8) {
                        Ptr::new(0)
                    } else {
                        fn_009b0e00(e, this, Ptr::new(reference), 0, flags as u8)
                    };
                    if !record.is_null() && e.vcall(reference, 0x100, &args![]).bool() {
                        let damage = fn_009b0f80(e, this);
                        if damage as f64 > e.global::<f64>(0x0101_2060) {
                            let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                            let distance = e.with_stack(12, |e, point| {
                                let coordinate = e.call(0x0045_bb80, &args![av_object]).u32();
                                // 009afe84: the two pushed words belong to
                                // `00439ef0` (RET 8).
                                let coordinate = e
                                    .call(0x0043_9ef0, &args![coordinate, point, position])
                                    .u32();
                                e.call(0x0045_7990, &args![coordinate]).f32()
                            });
                            fn_009b1510(e, this, record, Ptr::new(av_object), distance);
                        }
                    }
                }
            }
            index = index.wrapping_add(1);
        }
    }
    e.mem.set_u8(this.addr() + 0xd5, 1);
}

// Translated from 009afef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes eligible rigid bodies from the phantom into the explosion's force
/// list. The two native collision-layer lookups are kept as address calls;
/// their internal Havok object representation is outside this unit.
pub fn fn_009afef0(e: &mut Engine, this: Ptr) {
    let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
    if phantom != 0 {
        // 009aff0f: the pushed 0x20 is the word of `00477950` (RET 4).
        let form_data = e.call(0x0041_81e0, &args![this]).u32();
        let always_process = e.call(0x0047_7950, &args![form_data, 0x20u32]).bool();
        let mut index = 0u32;
        loop {
            let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
            let array = fn_009afed0(e, Ptr::new(phantom));
            let count = e.call(0x0072_6070, &args![array]).u32();
            if index >= count {
                break;
            }

            let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
            let array = fn_009afed0(e, Ptr::new(phantom));
            let slot = e.call(0x0062_9ab0, &args![array, index]).u32();
            let collidable = e.mem.u32(slot);
            let owner = e.call(0x004b_59f0, &args![collidable]).u32();
            let collision_layer = e.call(0x004b_5a80, &args![owner]).u32();
            let layer = if collision_layer == 0 {
                0
            } else {
                e.with_stack(4, |e, out| {
                    let filter = e.call(0x0043_b4f0, &args![collision_layer, out]).u32();
                    e.call(0x0043_b4d0, &args![filter]).u32()
                })
            };

            let owner_layer = if collision_layer == 0 {
                None
            } else {
                Some(e.call(0x0051_7630, &args![collision_layer]).u32())
            };
            let eligible_layer = owner_layer.is_some_and(|value| value < 4) || layer == 0x14;
            if always_process || layer == 0x1d {
                let havok_body = e.call(0x004b_5a20, &args![owner]).u32();
                let entity = if havok_body == 0 {
                    0
                } else {
                    e.call(0x0044_ddc0, &args![havok_body]).u32()
                };
                let reference = e.call(0x0056_f930, &args![entity]).u32();
                let exploded_reference = e.mem.u32(this.addr() + 0xcc);
                if !always_process || reference == exploded_reference {
                    let is_actor = reference != 0 && e.vcall(reference, 0x100, &args![]).bool();
                    let skipped_idle =
                        is_actor && e.call(0x005a_29b0, &args![reference]).u32() == 1;
                    if !skipped_idle && eligible_layer {
                        e.call(0x009b_0920, &args![this, collision_layer]);
                    }
                }
            } else if eligible_layer {
                e.call(0x009b_0920, &args![this, collision_layer]);
            }
            index = index.wrapping_add(1);
        }
    }
    e.mem.set_u8(this.addr() + 0xd7, 1);
}

// Translated from 009b0920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies linear and angular impulses to a non-null rigid body when the
/// explosion's force magnitude is strictly above the global threshold.
/// Intermediate x87 values are rounded where the native code stores floats.
pub fn fn_009b0920(e: &mut Engine, this: Ptr, rigid_body: Ptr) {
    if rigid_body.is_null() {
        return;
    }
    let data = e.call(0x0041_81e0, &args![this]).u32();
    let mut magnitude = fn_009b0bd0(e, Ptr::new(data));
    if !matches!(
        (magnitude as f64).partial_cmp(&e.global::<f64>(0x0101_2060)),
        Some(std::cmp::Ordering::Greater)
    ) {
        return;
    }

    // The exe's vectors are separate 12-byte locals: the impulse at EBP-0x70, its scaled copy
    // at EBP-0x64, the torque at EBP-0x7c, the angular intermediate at EBP-0x88 and the scaled
    // torque at EBP-0x94 (Ghidra prints the last two as `EBP + 0xffffff78` and `0xffffff6c`).
    // The angular intermediate and the scaled torque get their own slots past the others here
    // instead of +0x78 and +0x6c, where they lay over the torque and the impulse.
    e.with_stack(0xbc, |e, frame| {
        let target_vector = Ptr::<()>::new(frame.addr());
        let direction = Ptr::<()>::new(frame.addr() + 0x20);
        let position = Ptr::<()>::new(frame.addr() + 0x38);
        let impulse = Ptr::<()>::new(frame.addr() + 0x70);
        let torque = Ptr::<()>::new(frame.addr() + 0x7c);
        let scaled_impulse = Ptr::<()>::new(frame.addr() + 0x64);
        let angular_intermediate = Ptr::<()>::new(frame.addr() + 0xa4);
        let scaled_torque = Ptr::<()>::new(frame.addr() + 0xb0);
        let output = Ptr::<()>::new(frame.addr() + 0x98);
        e.call(0x0068_15c0, &args![target_vector]);
        e.call(0x0068_15c0, &args![direction]);
        let target_vector_object = e
            .vcall(rigid_body.addr(), 0xf0, &args![target_vector])
            .u32();
        e.call(0x0045_8620, &args![direction, target_vector_object]);

        let position_vector = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let transformed = e
            .call(0x0043_9ef0, &args![direction, position, position_vector])
            .u32();
        e.call(0x004a_0c10, &args![transformed]);

        let filter = e.call(0x0043_b4f0, &args![rigid_body, output]).u32();
        if e.call(0x0062_4070, &args![filter]).bool() {
            let coordinate = e.call(0x004a_51b0, &args![position, 2u32]).u32();
            let adjustment = e.call(0x0040_3e20, &args![0x011d_0e70u32]).u32();
            let adjusted = e.mem.f32(coordinate) + e.mem.f32(adjustment);
            e.mem.set_f32(coordinate, adjusted);
        }

        let mut actor_cause = false;
        if !fn_009ad610(e, this).is_null() {
            let proxy = e.call(0x004a_e750, &args![rigid_body]).u32();
            let body = e.call(0x004b_5a20, &args![proxy]).u32();
            let node = if body == 0 {
                0
            } else {
                e.call(0x0044_ddc0, &args![body]).u32()
            };
            let reference = e.call(0x0056_f930, &args![node]).u32();
            if reference != 0 && reference == fn_009ad610(e, this).addr() {
                let filter = e.call(0x0043_b4f0, &args![rigid_body, output]).u32();
                if e.call(0x0043_b4d0, &args![filter]).u32() == 0x14 {
                    e.vcall(rigid_body.addr(), 0xe4, &args![1u32]);
                }
                let scale = e.call(0x0040_3e20, &args![0x011d_0dbc_u32]).u32();
                magnitude *= e.mem.f32(scale);
                actor_cause = true;
            }
        }

        magnitude = e
            .call(0x0062_b520, &args![magnitude, rigid_body, 1u32])
            .f32();
        if !actor_cause {
            let maximum = e.call(0x0040_3e20, &args![0x011d_0c9cu32]).u32();
            magnitude = e
                .call(0x0040_ebd0, &args![magnitude, e.mem.f32(maximum)])
                .f32();
        }

        let linear_scale = e.call(0x0040_3e20, &args![0x011c_ff64u32]).u32();
        let scaled_direction = e.call(0x0045_bb20, &args![position, impulse, magnitude]);
        let scaled_direction = e.call(
            0x0045_bb20,
            &args![
                scaled_direction.u32(),
                scaled_impulse,
                e.mem.f32(linear_scale)
            ],
        );
        fn_009b0bf0(e, rigid_body, Ptr::new(scaled_direction.u32()));

        let mut random = [0.0f32; 3];
        for component in &mut random {
            let lower = e.global::<f32>(0x0101_2054);
            *component = e.call(0x0047_6b70, &args![lower, 1.0f32]).f32();
        }
        e.call(0x0041_6870, &args![torque, random[0], random[1], random[2]]);
        let angular_scale = e.call(0x0040_3e20, &args![0x011d_02ecu32]).u32();
        let angular_magnitude =
            e.call(0x0045_bb20, &args![torque, angular_intermediate, magnitude]);
        let scaled_torque = e.call(
            0x0045_bb20,
            &args![
                angular_magnitude.u32(),
                scaled_torque,
                e.mem.f32(angular_scale)
            ],
        );
        fn_009b0c90(e, rigid_body, Ptr::new(scaled_torque.u32()));
    });
}

// Translated from 009b1510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a body-part sample for actor targets when fewer than two samples exist
/// or the distance is strictly less than their minimum. Unordered comparisons
/// do not replace the minimum or admit the sample.
pub fn fn_009b1510(e: &mut Engine, this: Ptr, target: Ptr, object: Ptr, distance: f32) {
    if target.is_null() {
        return;
    }
    let actor = e.mem.u32(target.addr());
    if actor == 0 || !e.vcall(actor, 0x100, &args![]).bool() {
        return;
    }
    let mut node = target.addr() + 8;
    let mut count = 0i32;
    let mut minimum = 0;
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        count = count.wrapping_add(1);
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(slot);
        node = e.call(0x0072_6070, &args![node]).u32();
        if minimum == 0 || e.mem.f32(record + 4) < e.mem.f32(minimum + 4) {
            minimum = record;
        }
    }
    if count < 2 || minimum == 0 || distance < e.mem.f32(minimum + 4) {
        let name = e.vcall(object.addr(), 0xc, &args![]).u32();
        let part = e.call(0x008b_3ef0, &args![actor, name]).i32();
        if part != -1 && part != 0xe {
            fn_009b1630(e, this, target, part, quiet_f32(distance));
        }
    }
}

// Translated from 009b1630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Selects the minimum-distance record while traversing, stopping when that
/// selected record matches the requested part. With fewer than two traversed
/// records it allocates, even when the first record already matched.
pub fn fn_009b1630(e: &mut Engine, _this: Ptr, target: Ptr, part: i32, distance: f32) {
    let mut node = target.addr().wrapping_add(8);
    let mut count = 0i32;
    let mut selected = 0;
    while node != 0
        && !e.call(0x0082_56d0, &args![node]).bool()
        && (selected == 0 || e.mem.i32(selected) != part)
    {
        count = count.wrapping_add(1);
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(slot);
        node = e.call(0x0072_6070, &args![node]).u32();
        if selected == 0
            || e.mem.f32(record + 4) < e.mem.f32(selected + 4)
            || e.mem.i32(selected) == part
        {
            selected = record;
        }
    }
    if selected == 0 || count < 2 {
        selected = e.call(0x0040_1000, &args![8u32]).u32();
        e.mem.set_i32(selected, part);
        e.mem.set_f32(selected + 4, quiet_f32(distance));
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), selected);
            e.call(0x005a_e3d0, &args![target.addr() + 8, slot]);
        });
    } else {
        e.mem.set_i32(selected, part);
        e.mem.set_f32(selected + 4, quiet_f32(distance));
    }
}

// Translated from 009b1720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a target and returns its embedded body-part list, or null.
pub fn fn_009b1720(e: &mut Engine, this: Ptr, reference: Ptr) -> Ptr {
    let mut node = this.addr().wrapping_add(0xa4);
    let mut found = 0;
    while found == 0 && node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(slot);
        if record != 0 && e.mem.u32(record) == reference.addr() {
            found = record + 8;
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    Ptr::new(found)
}

// Translated from 009b17a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sends a temporary update record to the 3D object only while the flag is set.
/// RET 4 preserves an unread caller argument; the pending record argument
/// belongs to 00a59c60, not the intervening virtual getter.
pub fn fn_009b17a0(e: &mut Engine, this: Ptr, _unused_1: u32) {
    if fn_009ad5f0(e, this) && e.vcall(this.addr(), 0x1d0, &args![]).u32() != 0 {
        e.with_stack(12, |e, record| {
            let time = quiet_f32(e.global::<f32>(0x011c_3c08));
            e.call(0x0043_d410, &args![record, time, 1u32, 0u32]);
            let object = e.vcall(this.addr(), 0x1d0, &args![]).u32();
            e.call(0x00a5_9c60, &args![object, record]);
        });
    }
}

// Translated from 009b2040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Serializes persistent explosion members, references, actor-cause state,
/// and its three form IDs through the supplied save buffer.
pub fn fn_009b2040(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(0x0093_2880, &args![this, buffer]);
    for (offset, size) in [
        (0x88, 4),
        (0x8c, 4),
        (0x94, 4),
        (0x98, 4),
        (0xd4, 1),
        (0xe4, 4),
        (0x90, 4),
        (0x9c, 4),
        (0xd8, 1),
        (0xd9, 1),
        (0xe8, 0xc),
        (0xf4, 0xc),
        (0x100, 4),
    ] {
        e.call(
            0x0086_5e50,
            &args![buffer, this.addr() + offset, size, 0u32],
        );
    }
    for offset in [0xc8, 0xcc, 0xe0] {
        let form = e.mem.u32(this.addr() + offset);
        e.call(0x0086_5df0, &args![buffer, form, 0u32]);
    }

    let actor_cause = e.call(0x0055_9450, &args![this.addr() + 0xd0]).u32();
    let actor_cause_id = if actor_cause == 0 {
        u32::MAX
    } else {
        e.call(0x0082_5c00, &args![actor_cause]).u32()
    };
    e.with_stack(4, |e, saved_id| {
        e.mem.set_u32(saved_id.addr(), actor_cause_id);
        e.call(0x0086_5e50, &args![buffer, saved_id, 4u32, 0u32]);
    });
    let has_actor_cause = u32::from(e.mem.u32(this.addr() + 0xdc) != 0);
    e.with_stack(1, |e, saved_flag| {
        e.mem.set_u8(saved_flag.addr(), has_actor_cause as u8);
        e.call(0x0086_5e50, &args![buffer, saved_flag, 1u32, 0u32]);
    });
    if has_actor_cause != 0 {
        let cause = e.mem.u32(this.addr() + 0xdc);
        let magic_item = e.mem.u32(cause + 0x18);
        let item_id = if magic_item == 0 {
            0
        } else {
            e.call(0x0040_a1e0, &args![magic_item]).u32()
        };
        e.call(0x0086_5db0, &args![buffer, item_id, 0u32]);

        let magic_target = e.mem.u32(cause + 0x1c);
        let target_id = if magic_target == 0 {
            0
        } else {
            e.call(0x0082_54c0, &args![magic_target]).u32()
        };
        e.call(0x0086_5db0, &args![buffer, target_id, 0u32]);
        e.call(0x0086_5df0, &args![buffer, e.mem.u32(cause + 0x20), 0u32]);
    }
}

// Translated from 009b22c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads version-gated explosion fields, resolves the weapon and actor-cause
/// forms, and restores the target-processing flags from the frame counter.
// open: compiler SEH cleanup for a partially initialized actor-cause allocation is not modeled.
// Translation incomplete: 009b22c0 - compiler SEH cleanup for partially initialized actor-cause allocation
pub fn fn_009b22c0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(0x0093_2a00, &args![this, buffer]);
    for (offset, size) in [
        (0x88, 4),
        (0x8c, 4),
        (0x94, 4),
        (0x98, 4),
        (0xd4, 1),
        (0xe4, 4),
        (0x90, 4),
    ] {
        e.call(0x0086_4980, &args![buffer, this.addr() + offset, size]);
    }
    let version = e.vcall(buffer.addr(), 0, &args![]).u8();
    if version > 13 {
        for (offset, size) in [(0x9c, 4), (0xd8, 1), (0xd9, 1), (0xe8, 0xc), (0xf4, 0xc)] {
            e.call(0x0086_4980, &args![buffer, this.addr() + offset, size]);
        }
    }
    if e.vcall(buffer.addr(), 0, &args![]).u8() > 19 {
        e.call(0x0086_4980, &args![buffer, this.addr() + 0x100, 4u32]);
    }

    for offset in [0xc8, 0xcc] {
        e.call(0x0086_48e0, &args![buffer, this.addr() + offset]);
    }
    // 009b244f: `008648a0` takes no stack word (plain RET); the three words pushed
    // before it are the last three of the cdecl `__RTDynamicCast` (`00ec43fb`).
    let form = e.call(0x0086_48a0, &args![buffer]).u32();
    let form = e.call(0x0048_39c0, &args![form]).u32();
    let weapon = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_3028u32, 0x0118_3998u32, 0u32],
        )
        .u32();
    e.mem.set_u32(this.addr() + 0xe0, weapon);

    e.with_stack(4, |e, actor_cause_id| {
        e.mem.set_u32(actor_cause_id.addr(), u32::MAX);
        e.call(0x0086_4980, &args![buffer, actor_cause_id, 4u32]);
        let actor_cause = e
            .call(0x0066_e330, &args![e.mem.u32(actor_cause_id.addr())])
            .u32();
        e.call(0x0042_f780, &args![this.addr() + 0xd0, actor_cause]);
    });

    e.with_stack(1, |e, has_actor_cause| {
        e.mem.set_u8(has_actor_cause.addr(), 1);
        if e.vcall(buffer.addr(), 0, &args![]).u8() > 6 {
            e.call(0x0086_4980, &args![buffer, has_actor_cause, 1u32]);
        }
        if e.mem.u8(has_actor_cause.addr()) == 0 {
            return;
        }

        let mut actor_cause = e.mem.u32(this.addr() + 0xdc);
        if actor_cause == 0 {
            let allocation = e.call(0x0040_1000, &args![0x24u32]).u32();
            actor_cause = if allocation == 0 {
                0
            } else {
                e.call(0x0082_5ad0, &args![allocation, this]).u32()
            };
            e.mem.set_u32(this.addr() + 0xdc, actor_cause);
        }
        for offset in [0x18, 0x1c, 0x20] {
            e.call(
                0x0086_48e0,
                &args![buffer, actor_cause.wrapping_add(offset)],
            );
        }
    });

    if e.mem.i32(this.addr() + 0xe4) > 0 {
        for offset in [0xd5, 0xd6, 0xd7] {
            e.mem.set_u8(this.addr() + offset, 1);
        }
    }
}

// Translated from 009b25b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves saved owner, exploded-reference and actor-cause numeric IDs, then
/// invokes the object's post-load virtual when the base post-load predicate
/// is true.
pub fn fn_009b25b0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(0x0093_2b70, &args![this, buffer]);
    for offset in [0xc8, 0xcc] {
        let unresolved = e.mem.u32(this.addr() + offset);
        let resolved = if unresolved == 0 {
            0
        } else {
            // 009b25e7 / 009b2634: the cdecl `004839c0` takes one word (ADD ESP,4);
            // the three words pushed before it are the last three of
            // `__RTDynamicCast` (`00ec43fb`, ADD ESP,0x14).
            let form = e.call(0x0048_39c0, &args![unresolved]).u32();
            e.call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_41ccu32, 0u32],
            )
            .u32()
        };
        e.mem.set_u32(this.addr() + offset, resolved);
    }

    let actor_cause = e.mem.u32(this.addr() + 0xdc);
    if actor_cause != 0 {
        let magic_item = e.mem.u32(actor_cause + 0x18);
        let magic_item = if magic_item == 0 {
            0
        } else {
            e.call(0x0040_a250, &args![magic_item]).u32()
        };
        e.mem.set_u32(actor_cause + 0x18, magic_item);

        let magic_target = e.mem.u32(actor_cause + 0x1c);
        let magic_target = if magic_target == 0 {
            0
        } else {
            e.call(0x0082_5550, &args![magic_target]).u32()
        };
        e.mem.set_u32(actor_cause + 0x1c, magic_target);

        let form = e.mem.u32(actor_cause + 0x20);
        let form = if form == 0 {
            0
        } else {
            // 009b2715: as above, the three words pushed before `004839c0` belong to
            // `__RTDynamicCast` (`00ec43fb`).
            let form = e.call(0x0048_39c0, &args![form]).u32();
            e.call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_41ccu32, 0u32],
            )
            .u32()
        };
        e.mem.set_u32(actor_cause + 0x20, form);
    }

    if fn_009ad5f0(e, this) {
        e.vcall(this.addr(), 0x304, &args![]);
    }
}

// Translated from 009b2770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the one-word load callback to MobileObject.
pub fn fn_009b2770(e: &mut Engine, this: Ptr, value: u32) {
    e.call(0x0093_2d60, &args![this, value]);
}

// Translated from 009b2790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets transient explosion flags, actor cause and vectors, then destroys
/// every non-null target and removes every list entry.
pub fn fn_009b2790(e: &mut Engine, this: Ptr, value: u32) {
    e.call(0x0093_2f60, &args![this, value]);
    for offset in 0xd4..=0xd7 {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    e.call(0x0042_f780, &args![this.addr() + 0xd0, 0u32]);
    e.mem.set_f32(this.addr() + 0x9c, 1.0);
    for offset in [0xd8, 0xd9] {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    for offset in [0xe8, 0xf4] {
        for component in [0, 4, 8] {
            let bits = e.global::<u32>(0x011f_426c + component);
            e.mem.set_u32(this.addr() + offset + component, bits);
        }
    }
    let list = this.addr() + 0xa4;
    while !e.call(0x0082_56d0, &args![list]).bool() {
        let slot = e.call(0x0068_15c0, &args![list]).u32();
        let target = e.mem.u32(slot);
        if target != 0 {
            fn_009ac910(e, Ptr::new(target), 1);
        }
        e.call(0x0063_f7b0, &args![list]);
    }
}

// Translated from 009b28a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears matching owner/exploded references and invalidates every matching
/// target record. It saves the next node before reading the current payload.
pub fn fn_009b28a0(e: &mut Engine, this: Ptr, reference: Ptr) {
    if fn_009acce0(e, this) == reference {
        e.call(0x0040_30d0, &args![this, 0u32]);
    }
    if fn_009ad610(e, this) == reference {
        e.call(0x0056_c7d0, &args![this, 0u32]);
    }
    let mut node = this.addr().wrapping_add(0xa4);
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let next = e.call(0x0072_6070, &args![node]).u32();
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let target = e.mem.u32(slot);
        if target != 0 && e.mem.u32(target) == reference.addr() {
            e.mem.set_u8(target + 6, 1);
            e.mem.set_u32(target, 0);
        }
        node = next;
    }
}

// Translated from 009b2940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the explosion overlaps a water surface or a water reference,
/// then ORs that result into `Explosion::bUnderWater` (Xbox PDB, PC `+0xD9`).
/// The embedded array descriptor at EBP-0x4B0, its 0x400-byte storage at
/// EBP-0x4A4 and the neighboring vector locals retain their native locations
/// in one raw stack region; only compiler SEH cleanup is omitted.
// open: compiler SEH cleanup for the stack contact collector.
// Translation incomplete: 009b2940 - compiler SEH cleanup for contact collector
pub fn fn_009b2940(e: &mut Engine, this: Ptr) -> bool {
    let mut underwater = false;
    let setting = e.call(0x0040_3e20, &args![0x011d_0e34u32]).ptr::<()>();
    let margin = e.mem.f32(setting.addr());
    let cell = e.call(0x008d_6f30, &args![this]).ptr::<()>();

    if !fn_009ad5f0(e, this) && !cell.is_null() {
        let bounds = e.vcall(this.addr(), 0x1f4, &args![]).ptr::<()>();
        let radius = e.call(0x0064_4930, &args![this]).f32();
        let top_z = radius as f64 * margin as f64 + e.mem.f32(bounds.addr() + 8) as f64;
        let water_height = e.call(0x0054_71e0, &args![cell]).f32();

        if water_height as f64 > top_z {
            underwater = true;
        } else {
            let world = e.call(0x0045_43c0, &args![cell]).ptr::<()>();
            let low_process = if world.is_null() {
                Ptr::NULL
            } else {
                e.call(0x0054_1c80, &args![world]).ptr::<()>()
            };
            let contact_collector = if low_process.is_null() {
                Ptr::NULL
            } else {
                e.call(0x0062_86d0, &args![low_process]).ptr::<()>()
            };

            if !contact_collector.is_null() {
                e.with_stack(0x4d0, |e, stack| {
                    // Native locals reach EBP-0x4d0 for the final double.
                    // Keep the collector's established frame base at EBP-4cc.
                    let frame = Ptr::<()>::new(stack.addr() + 4);
                    // EBP-0x4b0 is the 12-byte array object. Its backing
                    // storage starts at EBP-0x4a4 and spans 0x400 bytes.
                    let collector = Ptr::<()>::new(frame.addr() + 0x1c);
                    let first_scaled = Ptr::<()>::new(frame.addr() + 0x478);
                    let first_vector = Ptr::<()>::new(frame.addr() + 0x46c);
                    let second_scaled = Ptr::<()>::new(frame.addr() + 0x460);
                    let second_vector = Ptr::<()>::new(frame.addr() + 0x454);
                    let first_collector_vector = Ptr::<()>::new(frame.addr() + 0x42c);
                    let second_collector_vector = Ptr::<()>::new(frame.addr() + 0x43c);

                    let first = e.call(0x004a_3760, &args![first_scaled, 1.0f32, 0x011a_9490u32]);
                    let position = e.vcall(this.addr(), 0x1f4, &args![]).ptr::<()>();
                    e.call(
                        0x0043_9ef0,
                        &args![position, first_vector, first.ptr::<()>()],
                    );

                    let second = e.call(0x004a_3760, &args![second_scaled, 1.0f32, 0x011a_9490u32]);
                    let position = e.vcall(this.addr(), 0x1f4, &args![]).ptr::<()>();
                    e.call(
                        0x0043_9e90,
                        &args![position, second_vector, second.ptr::<()>()],
                    );
                    e.call(0x0043_7c10, &args![first_collector_vector]);
                    e.call(0x004a_3e00, &args![first_collector_vector, first_vector]);
                    e.call(0x004a_3e00, &args![second_collector_vector, second_vector]);

                    fn_009b2e30(e, collector, 0);
                    e.vcall(
                        contact_collector.addr(),
                        0x44,
                        &args![first_collector_vector, collector],
                    );

                    let mut index = 0u32;
                    while !underwater {
                        let count = e.call(0x0072_6070, &args![collector]).i32();
                        if (index as i32) >= count {
                            break;
                        }
                        let entry = e.call(0x0062_6660, &args![collector, index]).ptr::<()>();
                        let collidable_slot = Ptr::<()>::new(entry.addr() + 4);
                        let collidable_source = Ptr::<()>::new(e.mem.u32(collidable_slot.addr()));
                        let collidable = fn_009b2c90(e, collidable_source);
                        let av_object = e.call(0x004b_5820, &args![collidable]).ptr::<()>();
                        let reference = e.call(0x0056_f930, &args![av_object]).ptr::<()>();
                        if !reference.is_null() {
                            let form = e.call(0x007a_f430, &args![reference]).ptr::<()>();
                            if e.call(0x0040_1170, &args![form]).u32() == 0x23 {
                                let explosion_position =
                                    e.vcall(this.addr(), 0x1f4, &args![]).ptr::<()>();
                                let explosion_top = e.call(0x0064_4930, &args![this]).f32() as f64
                                    * margin as f64
                                    + e.mem.f32(explosion_position.addr() + 8) as f64;
                                e.mem.set_f64(frame.addr() - 4, explosion_top);
                                let water_position =
                                    e.vcall(reference.addr(), 0x1f4, &args![]).ptr::<()>();
                                if e.mem.f32(water_position.addr() + 8) as f64 > explosion_top {
                                    underwater = true;
                                }
                            }
                        }
                        index = index.wrapping_add(1);
                    }
                    fn_009b2db0(e, collector);
                });
            }
        }
    }

    let underwater_byte = u8::from(underwater);
    let stored = e.mem.u8(this.addr() + 0xd9) | underwater_byte;
    e.mem.set_u8(this.addr() + 0xd9, stored);
    underwater
}

// Translated from 009b2c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns this plus the signed byte at +5, with 32-bit wrapping.
pub fn fn_009b2c90(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(
        this.addr()
            .wrapping_add(e.mem.u8(this.addr() + 5) as i8 as i32 as u32),
    )
}

// Translated from 009b2cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the array, allocates aligned contact storage and marks its
/// capacity with the high bit.
// open: compiler SEH cleanup after aligned-storage allocation failure is not modeled.
// Translation incomplete: 009b2cb0 - compiler SEH cleanup after aligned-storage allocation failure
pub fn fn_009b2cb0(e: &mut Engine, this: Ptr, capacity: u32) -> Ptr {
    e.call(0x0062_99a0, &args![this]);
    let storage = fn_009b2e60(e, capacity, 0);
    e.mem.set_u32(this.addr(), storage.addr());
    e.mem.set_u32(this.addr() + 8, capacity | 0x8000_0000);
    e.mem.set_u32(this.addr() + 12, storage.addr());
    this
}

// Translated from 009b2d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Conditionally clears changed storage, releases the original allocation,
/// then destroys the array base.
// open: compiler SEH cleanup during array destruction is not modeled.
// Translation incomplete: 009b2d30 - compiler SEH cleanup during array destruction
pub fn fn_009b2d30(e: &mut Engine, this: Ptr) {
    e.with_stack(1, |e, result| {
        let flag = fn_009b2dd0(e, this, result);
        if !e.call(0x0056_09f0, &args![flag]).bool() {
            e.call(0x0062_6870, &args![this]);
        }
    });
    let allocation = e.mem.u32(this.addr() + 12);
    e.call(0x005e_0b60, &args![allocation]);
    e.call(0x0062_1b60, &args![this]);
}

// Translated from 009b2db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards destruction of the embedded array base.
pub fn fn_009b2db0(e: &mut Engine, this: Ptr) {
    e.call(0x0057_c530, &args![this]);
}

// Translated from 009b2dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the caller's boolean result from current versus original storage.
pub fn fn_009b2dd0(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let changed = e.mem.u32(this.addr() + 12) != e.mem.u32(this.addr());
    e.call(0x0062_2570, &args![result, changed]);
    result
}

// Translated from 009b2e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sorts contact records with a zero comparator-state byte; the decompiler's
/// standard-library name is an incorrect identical-body match.
pub fn fn_009b2e00(e: &mut Engine, records: Ptr, count: i32) {
    fn_009b2e90(e, records, count, 0);
}

// Translated from 009b2e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an embedded-capacity array over storage beginning at +12.
pub fn fn_009b2e30(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    e.call(0x0062_e230, &args![this, this.addr() + 12, count, 0x80u32]);
    this
}

// Translated from 009b2e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates count times 0x70 bytes, rounded up to a 0x80-byte boundary.
/// Cdecl has two words; the identity helper leaves both pending for 005e0ad0.
pub fn fn_009b2e60(e: &mut Engine, count: u32, flags: u32) -> Ptr {
    let allocator = e.call(0x00c8_5750, &args![]).u32();
    let bytes = count.wrapping_mul(0x70).wrapping_add(0x7f) & 0xffff_ff80;
    let allocator = e.call(0x0068_15c0, &args![allocator]).u32();
    e.call(0x005e_0ad0, &args![allocator, bytes, flags]).ptr()
}

// Translated from 009b2e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the recursive sort only for signed counts greater than one.
pub fn fn_009b2e90(e: &mut Engine, records: Ptr, count: i32, state: u8) {
    if count > 1 {
        fn_009b2ed0(e, records, 0, count - 1, state);
    }
}

// Translated from 009b2ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hkpRootCdPoint,...>` (Xbox PDB).
/// Copies the middle record as pivot, partitions through the game's comparator,
/// recursively sorts the left range and iterates the right range.
pub fn fn_009b2ed0(e: &mut Engine, records: Ptr, mut first: i32, last: i32, state: u8) {
    e.with_stack(0x104, |e, temporary| {
        let pivot = temporary;
        let swap = Ptr::<()>::new(temporary.addr() + 0x80);
        let comparator = Ptr::<()>::new(temporary.addr() + 0x100);
        e.mem.set_u8(comparator.addr(), state);
        e.call(0x0062_1c40, &args![pivot]);
        loop {
            let mut low = first;
            let mut high = last;
            let middle = first.wrapping_add(last) >> 1;
            let address = |index: i32| {
                Ptr::<()>::new(
                    records
                        .addr()
                        .wrapping_add((index as u32).wrapping_mul(0x70)),
                )
            };
            e.call(0x0062_12c0, &args![pivot, address(middle)]);
            loop {
                while fn_009b3080(e, comparator, address(low), pivot) {
                    low = low.wrapping_add(1);
                }
                while fn_009b3080(e, comparator, pivot, address(high)) {
                    high = high.wrapping_sub(1);
                }
                if high >= low {
                    if high != low {
                        e.call(0x0062_1c40, &args![swap]);
                        e.call(0x0062_12c0, &args![swap, address(high)]);
                        e.call(0x0062_12c0, &args![address(high), address(low)]);
                        e.call(0x0062_12c0, &args![address(low), swap]);
                    }
                    high = high.wrapping_sub(1);
                    low = low.wrapping_add(1);
                }
                if low > high {
                    break;
                }
            }
            if first < high {
                fn_009b2ed0(e, records, first, high, state);
            }
            if low >= last {
                break;
            }
            first = low;
        }
    });
}

// Translated from 009b3080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Contact comparator wrapper: constructs a temporary boolean and returns AL.
pub fn fn_009b3080(e: &mut Engine, _this: Ptr, left: Ptr, right: Ptr) -> bool {
    e.with_stack(1, |e, flag| {
        let result = e.call(0x009b_30b0, &args![left, flag, right]).u32();
        e.call(0x0056_09f0, &args![result]).bool()
    })
}

// The 009ae6e0 frame is 16-byte aligned and its live locals span
// EBP-0x928..EBP-0x14. Keep all collector and callback temporaries at
// their PC offsets: several callees retain pointers into this frame.
#[allow(dead_code)]
struct ExplosionEffectFrame {
    ebp: u32,
}

#[allow(dead_code)]
impl ExplosionEffectFrame {
    fn at(&self, offset: u32) -> u32 {
        self.ebp - offset
    }

    fn point(&self, offset: u32) -> Ptr {
        Ptr::new(self.at(offset))
    }
}

#[allow(dead_code)]
fn copy_explosion_point(e: &mut Engine, destination: u32, source: u32) {
    for offset in [0, 4, 8] {
        e.mem
            .set_u32(destination + offset, e.mem.u32(source + offset));
    }
}

#[allow(dead_code)]
fn explosion_debug_contact(e: &mut Engine, frame: &ExplosionEffectFrame) {
    let colour = quiet_f32(e.global::<f32>(0x0102_3934));
    e.call(
        0x0041_4430,
        &args![frame.point(0xe8), 1.0f32, 0.0f32, 0.0f32, colour],
    );
    let radius = quiet_f32(e.global::<f32>(0x0101_6088));
    let marker = e
        .call(0x004b_29b0, &args![radius, frame.point(0xe8), 1u32])
        .u32();
    e.call(0x00a5_a040, &args![marker]);
    e.call(0x0044_0460, &args![marker, frame.point(0xcc)]);
    let duration = quiet_f32(e.global::<f32>(0x0101_7b78));
    let world: u32 = e.global(0x011d_ea10);
    e.call(0x0045_8e20, &args![world, marker, duration]);

    let duration = quiet_f32(e.global::<f32>(0x0101_7b78));
    let scaled = e
        .call(
            0x0045_bb20,
            &args![frame.point(0xd8), frame.point(0xf8), duration],
        )
        .u32();
    let endpoint = e
        .call(
            0x0043_9e90,
            &args![frame.point(0xcc), frame.point(0x104), scaled],
        )
        .u32();
    let marker = e
        .call(
            0x004b_3890,
            &args![
                frame.point(0xcc),
                frame.point(0xe8),
                endpoint,
                frame.point(0xe8),
                1u32
            ],
        )
        .u32();
    let world: u32 = e.global(0x011d_ea10);
    let duration = quiet_f32(e.global::<f32>(0x0101_7b78));
    e.call(0x0045_8e20, &args![world, marker, duration]);
}

#[allow(dead_code)]
fn explosion_scan_contact(
    e: &mut Engine,
    this: Ptr,
    frame: &ExplosionEffectFrame,
    object_3d: u32,
    found: &mut bool,
) {
    e.call(0x0068_15c0, &args![frame.point(0x194)]);
    let source = e.call(0x0068_15c0, &args![frame.point(0x180)]).u32();
    e.call(0x0045_8620, &args![frame.point(0x194), source]);
    e.call(0x0068_15c0, &args![frame.point(0x1a0)]);
    let source = e.call(0x0046_0140, &args![frame.point(0x180)]).u32();
    e.call(0x004a_3970, &args![frame.point(0x1a0), source]);
    e.call(0x004a_3a70, &args![frame.point(0x4c0)]);
    e.call(0x004a_3c20, &args![frame.point(0x570)]);

    let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
    copy_explosion_point(e, frame.at(0x57c), position);
    let endpoint = e
        .call(
            0x0043_9ef0,
            &args![frame.point(0x194), frame.point(0x588), frame.point(0x57c)],
        )
        .u32();
    let two = quiet_f32(e.global::<f32>(0x0101_62c0));
    e.call(0x004a_3760, &args![frame.point(0x594), two, endpoint]);
    let length = e.call(0x0045_7990, &args![frame.point(0x594)]).f64();
    let one: f64 = e.global(0x0101_2070);
    if length < one {
        let scale = quiet_f32(e.global::<f32>(0x0101_e57c));
        let scaled = e
            .call(
                0x0045_bb20,
                &args![frame.point(0x1a0), frame.point(0x5a0), scale],
            )
            .u32();
        e.call(0x0045_78c0, &args![frame.point(0x57c), scaled]);
        let scale = quiet_f32(e.global::<f32>(0x0101_e340));
        let scaled = e
            .call(
                0x0045_bb20,
                &args![frame.point(0x1a0), frame.point(0x5ac), scale],
            )
            .u32();
        copy_explosion_point(e, frame.at(0x594), scaled);
    }
    e.call(0x004a_3da0, &args![frame.point(0x570), frame.point(0x57c)]);
    e.call(0x007f_68a0, &args![frame.point(0x570), frame.point(0x594)]);
    let phantom = e.call(0x0055_9450, &args![this.addr() + 0xa0]).u32();
    let data = e
        .call(0x0043_b4f0, &args![phantom, frame.point(0x5b0)])
        .u32();
    e.call(0x004a_3f70, &args![frame.point(0x570), e.mem.u32(data)]);
    e.call(0x004a_3fb0, &args![frame.point(0x570), frame.point(0x4c0)]);
    e.mem.set_u32(frame.at(0x5b4), 0x24);
    e.mem.set_u8(frame.at(0x5b5), 0);

    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let world = e.call(0x0045_43c0, &args![cell]).u32();
    if world != 0 {
        let cell = e.call(0x008d_6f30, &args![this]).u32();
        let world = e.call(0x0045_43c0, &args![cell]).u32();
        if e.vcall(world, 0xc8, &args![frame.point(0x570)]).bool() {
            let mut ray_index = 0i32;
            let mut stop = false;
            let mut ray_found = false;
            while !stop && !ray_found {
                let ray_hits = e.call(0x008c_dd90, &args![frame.point(0x570)]).u32();
                let hits = e.call(0x0046_0140, &args![ray_hits]).u32();
                let count = e.call(0x0072_6070, &args![hits]).i32();
                if ray_index >= count {
                    break;
                }
                let ray_hits = e.call(0x008c_dd90, &args![frame.point(0x570)]).u32();
                let hits = e.call(0x0046_0140, &args![ray_hits]).u32();
                let row = e.call(0x004a_46b0, &args![hits, ray_index]).u32();
                e.call(0x0069_6c30, &args![frame.point(0x630), row]);
                let collidable = e.mem.u32(frame.at(0x5e0));
                let hit_object = e.call(0x004b_5820, &args![collidable]).u32();
                let reference = e.call(0x0056_f930, &args![hit_object]).u32();
                if hit_object == object_3d {
                    ray_found = true;
                    let mut last_part = -1i32;
                    for part_index in 0..8 {
                        let part = e.mem.i32(frame.at(0x610) + part_index * 4);
                        if part == -1 {
                            break;
                        }
                        last_part = part;
                    }
                    let shape = e.call(0x0043_b540, &args![collidable]).u32();
                    let probe = e.call(0x008c_71b0, &args![frame.point(0x644), shape]).u32();
                    let shape_kind = e.call(0x004a_3a20, &args![probe]).i32();
                    if shape_kind == 1 {
                        let world: u32 = e.global(0x011d_ea10);
                        let part = e.call(0x0045_7880, &args![world, frame.point(0x194)]).u32();
                        e.mem.set_u32(frame.at(0x5b4), part);
                    } else {
                        let container = e.call(0x0055_9450, &args![collidable]).u32();
                        let shape = e.call(0x004a_3a40, &args![container]).u32();
                        if shape != 0 {
                            let part = e.call(0x00c8_4f10, &args![shape, last_part]).u32();
                            e.mem.set_u32(frame.at(0x5b4), part);
                        }
                    }
                } else if reference == 0 || !e.vcall(reference, 0x100, &args![]).bool() {
                    stop = true;
                }
                ray_index += 1;
            }
            if ray_found {
                explosion_hit_effect(e, this, frame, found);
            }
        }
    }
    e.call(0x004a_3bc0, &args![frame.point(0x4c0)]);
}

#[allow(dead_code)]
fn explosion_debug_hit(e: &mut Engine, this: Ptr, frame: &ExplosionEffectFrame) {
    let colour = quiet_f32(e.global::<f32>(0x0102_3934));
    e.call(
        0x0041_4430,
        &args![frame.point(0x65c), 0.0f32, 1.0f32, 0.0f32, colour],
    );
    let radius = quiet_f32(e.global::<f32>(0x0101_712c));
    let marker = e
        .call(0x004b_29b0, &args![radius, frame.point(0x65c), 1u32])
        .u32();
    e.call(0x00a5_a040, &args![marker]);
    e.call(0x0044_0460, &args![marker, this.addr() + 0xe8]);
    let duration = quiet_f32(e.global::<f32>(0x0101_7b78));
    let world: u32 = e.global(0x011d_ea10);
    e.call(0x0045_8e20, &args![world, marker, duration]);

    let scale = quiet_f32(e.global::<f32>(0x0101_7868));
    let scaled = e
        .call(
            0x0045_bb20,
            &args![this.addr() + 0xf4, frame.point(0x66c), scale],
        )
        .u32();
    let endpoint = e
        .call(
            0x0043_9e90,
            &args![this.addr() + 0xe8, frame.point(0x678), scaled],
        )
        .u32();
    let marker = e
        .call(
            0x004b_3890,
            &args![
                this.addr() + 0xe8,
                frame.point(0x65c),
                endpoint,
                frame.point(0x65c),
                1u32
            ],
        )
        .u32();
    let world: u32 = e.global(0x011d_ea10);
    let duration = quiet_f32(e.global::<f32>(0x0101_7b78));
    e.call(0x0045_8e20, &args![world, marker, duration]);
}

#[allow(dead_code)]
fn explosion_hit_effect(e: &mut Engine, this: Ptr, frame: &ExplosionEffectFrame, found: &mut bool) {
    copy_explosion_point(e, this.addr() + 0xe8, frame.at(0x194));
    if e.call(0x0043_90c0, &args![this.addr() + 0xf4, 0x011f_426cu32])
        .bool()
    {
        copy_explosion_point(e, this.addr() + 0xf4, frame.at(0x1a0));
    }
    let debug_level_slot = e.call(0x0043_d4d0, &args![0x011d_0a78u32]).u32();
    if e.mem.i32(debug_level_slot) > 1 {
        explosion_debug_hit(e, this, frame);
    }
    *found = true;

    let part = e.mem.u32(frame.at(0x5b4));
    let attack = e.call(0x0041_81e0, &args![this]).u32();
    let effect = e.call(0x004f_b7d0, &args![attack, part]).u32();
    if effect == 0 {
        return;
    }
    let mut allow_hit_sound = true;
    let lighting = e.call(0x0052_4c90, &args![]).u32();
    let vector = e.call(0x0045_bb80, &args![lighting]).u32();
    e.call(
        0x0043_9ef0,
        &args![frame.point(0x194), frame.point(0x68c), vector],
    );
    if !e.call(0x0052_5420, &args![]).bool() {
        let distance = e.call(0x0045_7990, &args![frame.point(0x68c)]).f64();
        let limit_pointer = e.call(0x0040_3e20, &args![0x011c_e26cu32]).u32();
        let limit = e.mem.f32(limit_pointer);
        if !matches!(
            (limit as f64).partial_cmp(&distance),
            Some(std::cmp::Ordering::Less)
        ) {
            let forty = quiet_f32(e.global::<f32>(0x0101_e340));
            let lighting = e.call(0x0052_4c90, &args![]).u32();
            let p = frame.at(0x194);
            let point = [e.mem.u32(p), e.mem.u32(p + 4), e.mem.u32(p + 8)];
            if !e
                .call(
                    0x004b_61d0,
                    &args![point[0], point[1], point[2], lighting, forty],
                )
                .bool()
            {
                allow_hit_sound = false;
            }
        } else {
            allow_hit_sound = false;
        }
    } else {
        allow_hit_sound = false;
    }
    let location = e.call(0x0067_33e0, &args![effect]).u32();
    if location != 0 {
        explosion_emit_decals(e, this, frame, effect, location);
    }
    if allow_hit_sound && e.mem.u8(this.addr() + 0xd9) == 0 {
        explosion_play_hit_sound(e, this, frame, effect);
    }
    explosion_play_effect_sounds(e, this, frame, effect);
}

#[allow(dead_code)]
fn explosion_emit_decals(
    e: &mut Engine,
    this: Ptr,
    frame: &ExplosionEffectFrame,
    effect: u32,
    location: u32,
) {
    let minimum = quiet_f32(e.call(0x004a_40c0, &args![effect]).f32());
    let maximum = quiet_f32(e.call(0x004a_40a0, &args![effect]).f32());
    let magnitude = quiet_f32(e.call(0x0047_6b70, &args![maximum, minimum]).f32());
    let caster = e.call(0x0062_2a70, &args![magnitude, 1u32]).u32();
    e.call(0x0062_33c0, &args![caster, 0u32]);
    e.call(0x004a_1a10, &args![caster, 8u32, 0u32]);
    e.call(0x004a_1a10, &args![caster, 0x1du32, 0u32]);
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let world = if cell == 0 {
        0
    } else {
        e.call(0x0045_43c0, &args![cell]).u32()
    };
    if world == 0 {
        return;
    }
    e.call(0x0062_31a0, &args![caster, world]);
    e.call(0x008a_ff30, &args![frame.point(0x7c8)]);
    e.call(
        0x0062_3090,
        &args![caster, frame.point(0x194), frame.point(0x7c8)],
    );
    let iterator = e.call(0x0041_3f40, &args![frame.point(0x7c8)]).u32();
    let count = e.call(0x0072_6070, &args![iterator]).i32();
    let heading = quiet_f32(e.call(0x004a_4240, &args![]).f32());
    let random_maximum = quiet_f32(e.global::<f32>(0x0101_e570));
    let random = quiet_f32(e.call(0x0047_6b70, &args![0.0f32, random_maximum]).f32());
    let random = e.call(0x0040_6cc0, &args![random]).f64();
    let random_mode = e.call(0x00ec_62c0, &args![random]).u32() as u8;
    for index in 0..count {
        explosion_decal_for_hit(
            e,
            this,
            frame,
            effect,
            location,
            magnitude,
            heading,
            random_mode,
            index,
        );
    }
    e.call(0x00cb_9900, &args![frame.point(0x7c8)]);
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
fn explosion_decal_for_hit(
    e: &mut Engine,
    this: Ptr,
    frame: &ExplosionEffectFrame,
    effect: u32,
    location: u32,
    magnitude: f32,
    heading: f32,
    random_mode: u8,
    index: i32,
) {
    let iterator = e.call(0x0041_3f40, &args![frame.point(0x7c8)]).u32();
    let hit = e.call(0x0062_e150, &args![iterator, index]).u32();
    let object = e.mem.u32(hit + 8);
    if object == 0 {
        return;
    }
    let mut part = e.mem.u32(frame.at(0x5b4));
    let iterator = e.call(0x0041_3f40, &args![frame.point(0x7c8)]).u32();
    let hit = e.call(0x0062_e150, &args![iterator, index]).u32();
    let material = e.mem.u32(hit + 12);
    if material == u32::MAX {
        let pointer = e.call(0x0055_9450, &args![object]).u32();
        let shape = e.call(0x004a_3a40, &args![pointer]).u32();
        if shape == 0 {
            return;
        }
        part = e.call(0x0062_05a0, &args![shape]).u32();
    } else {
        let pointer = e.call(0x0055_9450, &args![object]).u32();
        let shape = e.call(0x004a_3a40, &args![pointer]).u32();
        if shape == 0 {
            return;
        }
        let body = e.vcall(shape, 0xcc, &args![]).u32();
        if body != 0 {
            part = e.vcall(body, 0xe0, &args![material]).u32();
        }
    }
    if part != e.mem.u32(frame.at(0x5b4)) {
        return;
    }

    let shape = e.call(0x0043_b540, &args![object]).u32();
    let probe = e.call(0x008c_71b0, &args![frame.point(0x7f4), shape]).u32();
    let kind = e.call(0x004a_3a20, &args![probe]).i32();
    let dynamic_shape = kind == 1;
    let node = if dynamic_shape {
        let body = e.call(0x004b_59f0, &args![object]).u32();
        if body == 0 {
            0
        } else {
            let property = e
                .call(
                    0x00c8_5ef0,
                    &args![frame.point(0x808), body, 0x011f_4a20u32],
                )
                .u32();
            let node = e.call(0x0055_9450, &args![property]).u32();
            if node != 0 && e.call(0x0096_11e0, &args![node]).u32() != 0 {
                e.call(0x0096_11e0, &args![node]).u32()
            } else {
                node
            }
        }
    } else {
        e.call(0x00c7_fa90, &args![object]).u32()
    };
    let reference = e.call(0x0056_f930, &args![node]).u32();
    if !dynamic_shape && (reference == 0 || !e.call(0x004a_1060, &args![reference, 1u32]).bool()) {
        return;
    }

    let p = frame.at(0x194);
    let position = [e.mem.u32(p), e.mem.u32(p + 4), e.mem.u32(p + 8)];
    let world: u32 = e.global(0x011d_ea10);
    let marker = e
        .call(
            0x0045_19d0,
            &args![world, position[0], position[1], position[2]],
        )
        .u32();
    e.call(0x004a_37b0, &args![frame.point(0x890)]);
    copy_explosion_point(e, frame.at(0x890), this.addr() + 0xe8);
    copy_explosion_point(e, frame.at(0x884), this.addr() + 0xf4);
    e.mem.set_f32(frame.at(0x858), quiet_f32(magnitude));
    e.mem.set_f32(frame.at(0x854), quiet_f32(magnitude));
    let opacity = quiet_f32(e.call(0x008a_ff10, &args![effect]).f32());
    e.mem.set_f32(frame.at(0x850), opacity);
    e.mem.set_u32(frame.at(0x868), node);
    e.mem.set_u32(frame.at(0x860), location);
    e.mem.set_f32(frame.at(0x84c), quiet_f32(heading));
    e.mem.set_u8(frame.at(0x820), random_mode);
    let red = e.call(0x004a_4140, &args![effect]).u32() as u8;
    e.mem.set_u8(frame.at(0x81b), red);
    let green = e.call(0x004a_4180, &args![effect]).u32() as u8;
    e.mem.set_u8(frame.at(0x81c), green);
    let blue = e.call(0x004a_4100, &args![effect]).u32() as u8;
    e.mem.set_u8(frame.at(0x81d), blue);
    let scale = quiet_f32(e.call(0x004a_41c0, &args![effect]).f32());
    e.mem.set_f32(frame.at(0x844), scale);
    let flag = e.call(0x004a_41e0, &args![effect]).u32() as u8;
    e.mem.set_u8(frame.at(0x81a), flag);
    let offset = quiet_f32(e.call(0x004a_40e0, &args![effect]).f32());
    e.mem.set_f32(frame.at(0x83c), offset);
    let lifetime = quiet_f32(e.call(0x009a_9350, &args![effect]).f32());
    e.mem.set_f32(frame.at(0x838), lifetime);
    let packed = e.call(0x004a_4220, &args![effect]).u32();
    let divisor: f64 = e.global(0x0101_e568);
    let r = quiet_f32(((packed >> 16 & 0xff) as f64 / divisor) as f32);
    let packed = e.call(0x004a_4220, &args![effect]).u32();
    let divisor: f64 = e.global(0x0101_e568);
    let g = quiet_f32(((packed >> 8 & 0xff) as f64 / divisor) as f32);
    let packed = e.call(0x004a_4220, &args![effect]).u32();
    let divisor: f64 = e.global(0x0101_e568);
    let b = quiet_f32(((packed & 0xff) as f64 / divisor) as f32);
    let colour = e
        .call(0x0041_6870, &args![frame.point(0x8b4), b, g, r])
        .u32();
    copy_explosion_point(e, frame.at(0x830), colour);
    e.call(0x004a_3fe0, &args![marker, frame.point(0x890), 1u32, 0u32]);
}

#[allow(dead_code)]
fn explosion_play_hit_sound(e: &mut Engine, this: Ptr, frame: &ExplosionEffectFrame, effect: u32) {
    let filename = e.vcall(effect + 0x18, 0x14, &args![]).u32();
    if filename == 0 || e.mem.u8(filename) == 0 {
        return;
    }
    e.call(0x0068_15c0, &args![frame.point(0x8c4)]);
    let mode = e.call(0x005f_36f0, &args![effect]).u32();
    match mode {
        1 => {
            let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
            let point = e
                .call(
                    0x0043_9ef0,
                    &args![position, frame.point(0x8d4), frame.point(0x194)],
                )
                .u32();
            copy_explosion_point(e, frame.at(0x8c4), point);
            e.call(0x004a_0c10, &args![frame.point(0x8c4)]);
        }
        2 => {
            let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
            e.call(
                0x0043_9ef0,
                &args![position, frame.point(0x8e0), frame.point(0x194)],
            );
            e.call(0x004a_0c10, &args![frame.point(0x8e0)]);
            copy_explosion_point(e, frame.at(0x8ec), frame.at(0x1a0));
            e.call(0x004a_0c10, &args![frame.point(0x8ec)]);
            let dot = e
                .call(0x004b_6190, &args![frame.point(0x8ec), frame.point(0x8e0)])
                .f32();
            let two: f64 = e.global(0x0101_1590);
            let projection = quiet_f32((dot as f64 * two) as f32);
            let scaled = e
                .call(
                    0x004a_3760,
                    &args![frame.point(0x908), projection, frame.point(0x8ec)],
                )
                .u32();
            let reflected = e
                .call(
                    0x0043_9ef0,
                    &args![scaled, frame.point(0x8f8), frame.point(0x8e0)],
                )
                .u32();
            copy_explosion_point(e, frame.at(0x8c4), reflected);
        }
        _ => {
            copy_explosion_point(e, frame.at(0x8c4), frame.at(0x1a0));
            e.call(0x004a_0c10, &args![frame.point(0x8c4)]);
        }
    }
    let first = frame.at(0x8c4);
    let second = frame.at(0x194);
    let first = [e.mem.u32(first), e.mem.u32(first + 4), e.mem.u32(first + 8)];
    let second = [
        e.mem.u32(second),
        e.mem.u32(second + 4),
        e.mem.u32(second + 8),
    ];
    let volume = quiet_f32(e.call(0x0050_8100, &args![effect]).f32());
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    e.call(
        0x0068_90b0,
        &args![
            cell, volume, filename, first[0], first[1], first[2], second[0], second[1], second[2],
            1.0f32, 7u32, 0u32
        ],
    );
}

#[allow(dead_code)]
fn explosion_play_effect_sounds(
    e: &mut Engine,
    _this: Ptr,
    frame: &ExplosionEffectFrame,
    effect: u32,
) {
    for (getter, handle_offset) in [(0x0068_a810, 0x918), (0x0068_a830, 0x928)] {
        let filename = e.call(getter, &args![effect]).u32();
        if filename == 0 {
            continue;
        }
        let file = e.call(0x0051_1840, &args![filename]).u32();
        let audio = e.call(0x0045_3a70, &args![]).u32();
        e.call(
            0x00ad_7480,
            &args![audio, frame.point(handle_offset), file, 0x4102u32, filename],
        );
        let position = frame.at(0x194);
        let xyz = [0, 4, 8]
            .map(|offset| quiet_f32(f32::from_bits(e.mem.u32(position + offset))).to_bits());
        e.call(
            0x00ad_8b60,
            &args![frame.point(handle_offset), xyz[0], xyz[1], xyz[2]],
        );
        e.call(0x00ad_8830, &args![frame.point(handle_offset), 0u32]);
        e.call(0x0048_3710, &args![frame.point(handle_offset)]);
    }
}

// Translated from 009ae6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies contact, hit, decal and sound effects through the PC stack frame.
// open: compiler SEH cleanup after temporary construction/callback failure is not modeled.
// Translation incomplete: 009ae6e0 - compiler SEH cleanup after temporary construction/callback failure
pub fn explosion_apply_effects(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    if e.call(0x0055_9450, &args![object + 0xa0]).u32() == 0 {
        return;
    }
    if e.mem.i32(object + 0xe4) > 0 && e.mem.u8(object + 0xd6) != 0 && e.mem.u8(object + 0xd7) == 0
    {
        fn_009afef0(e, this);
    }
    if e.mem.u8(object + 0xd5) == 0 || e.mem.u8(object + 0xd4) != 0 {
        return;
    }
    e.mem.set_u8(object + 0xd4, 1);

    e.with_stack(0x948, |e, block| {
        let frame = ExplosionEffectFrame {
            ebp: (block.addr() + 0x938 + 15) & !15,
        };
        let collider_capacity_slot = e.call(0x0043_d4d0, &args![0x011d_10a4u32]).u32();
        let collider_capacity = e.mem.u32(collider_capacity_slot);
        fn_009b2cb0(e, frame.point(0x28), collider_capacity);
        let collider_capacity_slot = e.call(0x0043_d4d0, &args![0x011d_10a4u32]).u32();
        let collider_capacity = e.mem.u32(collider_capacity_slot);
        let collider_buffer = e.call(0x0055_9450, &args![frame.point(0x28)]).u32();
        fn_009afaf0(e, frame.point(0x40), collider_buffer, collider_capacity);
        e.call(0x00cb_d960, &args![]);
        let phantom = e.call(0x0055_9450, &args![object + 0xa0]).u32();
        e.call(0x00c9_f1e0, &args![phantom, frame.point(0x40)]);
        let has_contacts = fn_009afc70(e, frame.point(0x40), frame.at(0x41));
        if e.call(0x0056_09f0, &args![has_contacts]).bool() {
            let count = e.call(0x009a_fc50, &args![frame.point(0x40)]).i32();
            let records: Ptr = e.call(0x0055_9450, &args![frame.point(0x28)]).ptr();
            e.call(0x009b_2e00, &args![records, count]);
        }

        let debug_level_slot = e.call(0x0043_d4d0, &args![0x011d_0a78u32]).u32();
        if e.mem.i32(debug_level_slot) > 1 {
            let mut index = 0;
            while index < fn_009afc50(e, frame.point(0x40)) as i32 {
                let records = fn_009afc30(e, frame.point(0x40));
                let row = records
                    .addr()
                    .wrapping_add((index as u32).wrapping_mul(0x70));
                e.call(0x0063_2d40, &args![frame.point(0xc0), row]);
                e.call(0x0068_15c0, &args![frame.point(0xcc)]);
                let source = e.call(0x0068_15c0, &args![frame.point(0xc0)]).u32();
                e.call(0x0045_8620, &args![frame.point(0xcc), source]);
                e.call(0x0068_15c0, &args![frame.point(0xd8)]);
                let source = e.call(0x0046_0140, &args![frame.point(0xc0)]).u32();
                e.call(0x004a_3970, &args![frame.point(0xd8), source]);
                explosion_debug_contact(e, &frame);
                index += 1;
            }
        }

        let mut found = false;
        let mut index = 0;
        while !found && index < fn_009afc50(e, frame.point(0x40)) as i32 {
            let records = fn_009afc30(e, frame.point(0x40));
            let row = records
                .addr()
                .wrapping_add((index as u32).wrapping_mul(0x70));
            e.call(0x0063_2d40, &args![frame.point(0x180), row]);
            let collidable = e.mem.u32(frame.at(0x138));
            if collidable != 0 {
                let object_3d = e.call(0x004b_5820, &args![collidable]).u32();
                let reference = e.call(0x0056_f930, &args![object_3d]).u32();
                if reference == 0 || !e.vcall(reference, 0x100, &args![]).bool() {
                    explosion_scan_contact(e, this, &frame, object_3d, &mut found);
                }
            }
            index += 1;
        }
        if e.mem.u8(object + 0xd5) != 0 && e.mem.u8(object + 0xd6) == 0 {
            explosion_process_targets(e, this);
        }
        fn_009afc10(e, frame.point(0x40));
        fn_009b2d30(e, frame.point(0x28));
    });
}

// Translated from 009b00a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::ProcessTargets` (Xbox PDB): visits the target list, computes
/// explosion power and radiation falloff, then applies hits and impulses.
pub fn explosion_process_targets(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u8(object + 0xd6, 1);
    e.with_stack(0x110, |e, stack| {
        let ebp = stack.addr() + 0x110;
        let at = |offset: u32| ebp - offset;
        let mut cursor = object + 0xa4;
        while cursor != 0 && !e.call(0x0082_56d0, &args![cursor]).bool() {
            let entry = e.call(0x0068_15c0, &args![cursor]).u32();
            let target_record = e.mem.u32(entry);
            cursor = e.call(0x0072_6070, &args![cursor]).u32();
            if target_record == 0
                || e.mem.u32(target_record) == 0
                || e.mem.u8(target_record + 6) != 0
            {
                continue;
            }
            let target = e.mem.u32(target_record);
            if e.call(0x0047_7ba0, &args![target]).bool() {
                continue;
            }
            let target = e.mem.u32(target_record);
            if e.call(0x0044_0da0, &args![target]).bool() {
                continue;
            }

            let origin = e.vcall(object, 0x1f4, &args![]).u32();
            let target = e.mem.u32(target_record);
            // The two pushes before this getter remain on the stack for
            // 00439ef0; the virtual getter itself takes no arguments.
            let target_origin = e.vcall(target, 0x1f4, &args![]).u32();
            let separation = e
                .call(0x0043_9ef0, &args![target_origin, at(0xd0), origin])
                .u32();
            let distance = e.call(0x0045_7990, &args![separation]).f32();
            // 00644930 reads only this; the pending distance goes to 00647920.
            let radius = e.call(0x0064_4930, &args![this]).f32();
            let power = e.call(0x0064_7920, &args![radius, distance]).f32();
            let radiation = e.call(0x008d_01e0, &args![this]).f32();
            let falloff = if radiation as f64 > e.global::<f64>(0x0101_2060) {
                e.call(0x0064_7920, &args![radiation, distance]).f32()
            } else {
                1.0
            };
            if falloff as f64 > e.global::<f64>(0x0101_2060) {
                let target = e.mem.u32(target_record);
                if e.vcall(target, 0x100, &args![]).bool()
                    && e.mem.u32(target_record) == e.global::<u32>(0x011d_ea3c)
                {
                    let base = e.call(0x0041_81e0, &args![this]).u32();
                    let modifier = e.call(0x0072_6070, &args![base + 0x68]).u32();
                    if modifier != 0 {
                        e.call(0x0052_99a0, &args![modifier, falloff, 0u32]);
                    }
                }
            }
            if (power as f64).partial_cmp(&e.global::<f64>(0x0101_2060))
                != Some(std::cmp::Ordering::Greater)
            {
                continue;
            }
            if e.mem.u8(target_record + 5) != 0 {
                let source = e.vcall(object, 0x168, &args![]).u32();
                let target = e.mem.u32(target_record);
                e.vcall(target, 0x164, &args![source]);
            }
            let target = e.mem.u32(target_record);
            let actor = if e.vcall(target, 0x100, &args![]).bool() {
                e.mem.u32(target_record)
            } else {
                0
            };
            if actor != 0 {
                e.call(0x008a_5300, &args![actor]);
            }
            let owner = if e.vcall(object, 0x168, &args![]).u32() != 0 {
                let source = e.vcall(object, 0x168, &args![]).u32();
                e.call(0x0055_9450, &args![source]).u32()
            } else {
                0
            };
            let target = e.mem.u32(target_record);
            // The pushed flag belongs to SetActionFlag: 005d43c0 is a
            // no-argument getter for the embedded script object.
            let action = e.call(0x005d_43c0, &args![target]).u32();
            e.call(0x005a_c750, &args![owner, action, 0x80u32]);
            let has_weapon = !fn_009b0900(e, this).is_null();
            let target = e.mem.u32(target_record);
            let action = e.call(0x005d_43c0, &args![target]).u32();
            let weapon = if has_weapon {
                fn_009b0900(e, this).addr()
            } else {
                e.call(0x0041_81e0, &args![this]).u32()
            };
            e.call(0x005a_c750, &args![weapon, action, 0x100u32]);
            let damage = (fn_009b0f80(e, this) as f64 * power as f64) as f32;
            let target = e.mem.u32(target_record);
            if e.call(0x0047_7ba0, &args![target]).bool()
                && fn_009acce0(e, this).addr() == e.global::<u32>(0x011d_ea3c)
            {
                let player = e.global::<u32>(0x011d_ea3c);
                let amount = quiet_f32(e.global::<f32>(0x0102_918c));
                e.call(0x0096_3eb0, &args![player, 10u32, amount, 0u32]);
            }
            if actor == 0 {
                let target = e.mem.u32(target_record);
                e.vcall(target, 0x144, &args![damage, 0u32]);
                continue;
            }

            let origin = e.vcall(object, 0x1f4, &args![]).u32();
            let position = [
                e.mem.u32(origin),
                e.mem.u32(origin + 4),
                e.mem.u32(origin + 8),
            ];
            if !explosion_run_los_pick(
                e,
                this,
                position[0],
                position[1],
                position[2],
                Ptr::new(actor),
            ) {
                continue;
            }

            let hit = at(0x98);
            e.call(0x009b_4d90, &args![hit]);
            let cause = e.mem.u32(object + 0xc8);
            let cause = if cause != 0 && e.vcall(cause, 0x100, &args![]).bool() {
                e.mem.u32(object + 0xc8)
            } else {
                0
            };
            e.call(0x009b_5770, &args![hit, cause, actor, this, damage]);
            e.call(0x0089_a760, &args![actor, hit, 0u32]);
            if actor == e.global::<u32>(0x011d_ea3c) {
                explosion_player_hit_effect(e, at, power);
            }
            explosion_target_impulse(e, this, actor);
            if !e.vcall(actor, 0x22c, &args![0u32]).bool() {
                let base = e.call(0x0041_81e0, &args![this]).u32();
                let has_first = e.call(0x0047_7950, &args![base, 4u32]).bool();
                let apply = if has_first {
                    true
                } else {
                    let base = e.call(0x0041_81e0, &args![this]).u32();
                    if e.call(0x0047_7950, &args![base, 8u32]).bool() {
                        let component = quiet_f32(e.mem.f32(at(0x84)));
                        let component = e.call(0x00ec_62c0, &args![component as f64]).u32();
                        e.call(0x0064_6400, &args![actor + 0xa4, component]).bool()
                    } else {
                        false
                    }
                };
                if apply {
                    fn_009b0d70(e, this, Ptr::new(actor), power);
                }
            }
            if e.vcall(actor, 0x22c, &args![0u32]).bool() {
                let motion = if actor == e.global::<u32>(0x011d_ea3c) {
                    e.call(0x0095_0bb0, &args![actor, 0u32]).u32()
                } else {
                    e.vcall(actor, 0x1d0, &args![]).u32()
                };
                e.call(0x00c6_a350, &args![motion, 1u32, 1u32, 0u32, 1u32]);
                e.call(0x0043_d410, &args![at(0xc4), 0.0f32, 0u32, 0u32]);
                e.call(0x00a5_9c60, &args![motion, at(0xc4)]);
            }
        }
    });
}

fn explosion_player_hit_effect(e: &mut Engine, at: impl Fn(u32) -> u32, power: f32) {
    let threshold = e.call(0x0040_3e20, &args![0x011d_0030u32]).u32();
    if e.mem.f32(threshold).partial_cmp(&power) != Some(std::cmp::Ordering::Less) {
        return;
    }
    let mut points = [0u32; 3];
    for (index, (first, second)) in [
        (0x011c_fea0u32, 0x011d_0490u32),
        (0x011d_0d38, 0x011d_0edc),
        (0x011d_05e0, 0x011d_00d8),
    ]
    .into_iter()
    .enumerate()
    {
        let second = e.call(0x0040_3e20, &args![second]).u32();
        let second = quiet_f32(e.mem.f32(second));
        let first = e.call(0x0040_3e20, &args![first]).u32();
        let first = quiet_f32(e.mem.f32(first));
        points[index] = at(0xb4 - index as u32 * 8);
        e.call(0x0045_2dc0, &args![points[index], first, second]);
    }
    let scaled_second = e
        .call(0x0053_2fd0, &args![at(0xd8), power, points[1]])
        .u32();
    let scaled_first = e
        .call(0x0053_2fd0, &args![at(0xe0), power, points[0]])
        .u32();
    let interface = e.call(0x0070_5950, &args![]).u32();
    e.call(
        0x007f_78f0,
        &args![interface, scaled_first, scaled_second, points[2]],
    );
}

fn explosion_target_impulse(e: &mut Engine, this: Ptr, actor: u32) {
    let base = e.call(0x0041_81e0, &args![this]).u32();
    let source = e.call(0x0072_6070, &args![base + 0x54]).u32();
    if source == 0 {
        return;
    }
    let owner = fn_009acce0(e, this).addr();
    let owner_is_actor = owner != 0 && {
        let reloaded_owner = fn_009acce0(e, this).addr();
        e.vcall(reloaded_owner, 0x100, &args![]).bool()
    };
    if !owner_is_actor && e.mem.u32(this.addr() + 0xdc) == 0 {
        return;
    }
    let location = if source == 0 { 0 } else { source + 0x18 };
    let point = if actor == 0 { 0 } else { actor + 0x94 };
    let caster = e.mem.u32(this.addr() + 0xdc);
    if caster != 0 {
        e.vcall(
            caster + 0xc,
            0xc,
            &args![location, 0u32, point, 1.0f32, 0u32],
        );
    } else {
        let owner = fn_009acce0(e, this).addr();
        e.vcall(
            owner + 0x88,
            0xc,
            &args![location, 0u32, point, 1.0f32, 0u32],
        );
    }
}

// Translated from 009b1810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Explosion::RunLOSPick` (Xbox PDB): raycasts toward the target's world
/// bound, then tries six offset points when the target has a controller.
// open: compiler SEH cleanup while collectors and ray inputs are live is not modeled.
// Translation incomplete: 009b1810 - compiler SEH cleanup while collectors and ray inputs are live
pub fn explosion_run_los_pick(
    e: &mut Engine,
    this: Ptr,
    x: u32,
    y: u32,
    z: u32,
    target: Ptr,
) -> bool {
    let base = e.call(0x0041_81e0, &args![this]).u32();
    if e.call(0x0047_7950, &args![base, 0x10u32]).bool() {
        return true;
    }
    let node = if target.is_null() {
        0
    } else {
        e.vcall(target.addr(), 0x1d0, &args![]).u32()
    };
    let node = if target.addr() == e.global::<u32>(0x011d_ea3c) {
        e.call(0x0095_0bb0, &args![target, 0u32]).u32()
    } else {
        node
    };
    if node == 0 || e.call(0x004b_2960, &args![node]).u32() == 0 {
        return false;
    }
    let cell = e.call(0x008d_6f30, &args![target]).u32();
    let world = if cell == 0 {
        0
    } else {
        let cell = e.call(0x008d_6f30, &args![target]).u32();
        e.call(0x0045_43c0, &args![cell]).u32()
    };
    if world == 0 {
        return false;
    }

    e.with_stack(0x940, |e, stack| {
        let ebp = stack.addr() + 0x920;
        let at = |offset: u32| ebp - offset;
        let position = ebp + 0x10;
        for (index, word) in [x, y, z].into_iter().enumerate() {
            e.mem.set_u32(position + index as u32 * 4, word);
        }
        e.call(0x00c8_2870, &args![0x27u32, 0x26206u32, 0u32]);
        e.call(0x0040_1050, &args![at(0x88), 12u32, 7u32, 0x0068_15c0u32]);
        let bound = e.call(0x0043_d450, &args![node]).u32();
        let centre = e.call(0x0068_15c0, &args![bound]).u32();
        copy_explosion_point(e, at(0x88), centre);

        let has_probes = e.vcall(target.addr(), 0x100, &args![]).bool()
            && e.vcall(target.addr(), 0x214, &args![]).u32() != 0;
        if has_probes {
            // Each point starts with the first bound centre. The PC asks for
            // the world bound and radius again for every axis adjustment.
            for (index, axis, sign) in [
                (1u32, 2u32, 1.0f64),
                (2, 2, -1.0),
                (3, 1, 1.0),
                (4, 1, -1.0),
                (5, 0, 1.0),
                (6, 0, -1.0),
            ] {
                let point = at(0x88) + index * 12;
                copy_explosion_point(e, point, at(0x88));
                let bound = e.call(0x0043_d450, &args![node]).u32();
                let radius = e.call(0x0084_d030, &args![bound]).f64();
                let axis_address = point + axis * 4;
                let initial = e.mem.f32(axis_address);
                e.mem.set_f32(
                    axis_address,
                    (initial as f64 + sign * (radius + radius)) as f32,
                );
            }
        }

        let ray = at(0x140);
        let id = at(0x144);
        e.call(0x004a_3c20, &args![ray]);
        e.call(0x008c_71b0, &args![id, 0u32]);
        e.call(0x004a_39f0, &args![id, 0x27u32]);
        let selected = if e.vcall(target.addr(), 0x100, &args![]).bool() {
            target.addr()
        } else {
            0
        };
        let controller = if selected != 0 {
            e.call(0x0093_06d0, &args![selected]).u32()
        } else {
            0
        };
        let controller = if controller != 0 {
            let controller = e.call(0x0093_06d0, &args![selected]).u32();
            e.call(0x0081_9250, &args![controller]).u32()
        } else {
            e.call(0x005d_be10, &args![]).u32()
        };
        e.call(0x0059_ce80, &args![id, controller]);
        let scale_slot = e.call(0x0040_3e20, &args![0x011d_1344u32]).u32();
        let scale = quiet_f32(e.mem.f32(scale_slot));
        let scaled = e
            .call(0x0045_bb20, &args![this.addr() + 0xf4, at(0x154), scale])
            .u32();
        e.call(0x0063_c8a0, &args![position, scaled]);
        e.call(0x004a_3f70, &args![ray, e.mem.u32(id)]);
        e.call(0x004a_3da0, &args![ray, position]);
        e.call(0x004a_3eb0, &args![ray, at(0x88)]);

        e.call(0x004a_3a70, &args![at(0x480)]);
        e.call(0x004a_3fb0, &args![ray, at(0x480)]);
        e.mem.set_u8(at(0x481), 0);
        e.vcall(world, 0xc8, &args![ray]);
        explosion_los_scan(e, this, ebp, position, 0x4f0, 0x4fc, 0x508, true);
        explosion_los_debug(e, ebp, position);

        if has_probes && e.mem.u8(at(0x481)) != 0 {
            for probe in 1..7 {
                if e.mem.u8(at(0x481)) == 0 {
                    break;
                }
                e.mem.set_u8(at(0x481), 0);
                e.call(0x004a_3a70, &args![at(0x870)]);
                e.call(0x004a_3fb0, &args![ray, at(0x870)]);
                e.call(0x004a_3eb0, &args![ray, at(0x88) + probe * 12]);
                e.vcall(world, 0xc8, &args![ray]);
                explosion_los_scan(e, this, ebp, position, 0x8e0, 0x8ec, 0x8f8, false);
                e.call(0x004a_3bc0, &args![at(0x870)]);
            }
        }
        let visible = e.mem.u8(at(0x481)) == 0;
        e.call(0x004a_3bc0, &args![at(0x480)]);
        visible
    })
}

#[allow(clippy::too_many_arguments)]
fn explosion_los_scan(
    e: &mut Engine,
    this: Ptr,
    ebp: u32,
    position: u32,
    hit_offset: u32,
    delta_offset: u32,
    scaled_offset: u32,
    first: bool,
) {
    let at = |offset: u32| ebp - offset;
    let ray = at(0x140);
    let mut index = 0i32;
    while e.mem.u8(at(0x481)) == 0 {
        let hits = e.call(0x008c_dd90, &args![ray]).u32();
        let hits = e.call(0x0046_0140, &args![hits]).u32();
        let count = e.call(0x0072_6070, &args![hits]).i32();
        if index >= count {
            break;
        }
        let hits = e.call(0x008c_dd90, &args![ray]).u32();
        let hits = e.call(0x0046_0140, &args![hits]).u32();
        let row = e.call(0x004a_46b0, &args![hits, index]).u32();
        e.call(0x0069_6c30, &args![at(hit_offset), row]);
        // The second scan indexes probe points by this *hit* index in the PC
        // code, even though the outer loop has a separate probe index.
        let point = if first {
            at(0x88)
        } else {
            at(0x88) + (index as u32).wrapping_mul(12)
        };
        e.call(0x0043_9ef0, &args![point, at(delta_offset), position]);
        let fraction = quiet_f32(e.mem.f32(at(hit_offset - 0x10)));
        e.call(
            0x004a_3760,
            &args![at(scaled_offset), fraction, at(delta_offset)],
        );
        let component = e.call(0x004a_51b0, &args![this.addr() + 0xf4, 2u32]).u32();
        let normal = e.mem.f32(component);
        let threshold = e.global::<f64>(0x0108_f2a8);
        let mut blocked = (normal as f64).partial_cmp(&threshold) != Some(std::cmp::Ordering::Less);
        if !blocked {
            let distance = e.call(0x0045_7990, &args![at(scaled_offset)]).f64();
            e.mem.set_f64(at(scaled_offset + 8), distance);
            let setting = e.call(0x0040_3e20, &args![0x011d_150cu32]).u32();
            blocked = (e.mem.f32(setting) as f64).partial_cmp(&e.mem.f64(at(scaled_offset + 8)))
                != Some(std::cmp::Ordering::Greater);
        }
        if blocked && first {
            let owner = fn_009acce0(e, this).addr();
            if owner != 0 {
                let owner = fn_009acce0(e, this).addr();
                blocked = !e.call(0x0045_2370, &args![owner]).bool();
            }
        }
        if blocked {
            if first {
                let level = e.call(0x0043_d4d0, &args![0x011d_0a78u32]).u32();
                if e.mem.i32(level) > 0 {
                    e.call(0x005b_5e40, &args![0x0108_f294u32]);
                }
            }
            e.mem.set_u8(at(0x481), 1);
        }
        index += 1;
    }
}

fn explosion_los_debug(e: &mut Engine, ebp: u32, position: u32) {
    let at = |offset: u32| ebp - offset;
    let level = e.call(0x0043_d4d0, &args![0x011d_0a78u32]).u32();
    if e.mem.i32(level) <= 0 {
        return;
    }
    let colour = if e.mem.u8(at(0x481)) != 0 {
        e.call(
            0x0041_4430,
            &args![at(0x520), 1.0f32, 0.0f32, 0.0f32, 1.0f32],
        )
        .u32()
    } else {
        e.call(
            0x0041_4430,
            &args![at(0x534), 0.0f32, 1.0f32, 0.0f32, 1.0f32],
        )
        .u32()
    };
    for offset in [0, 4, 8, 12] {
        e.mem
            .set_u32(at(0x548) + offset, e.mem.u32(colour + offset));
    }
    let marker = e
        .call(
            0x004b_3890,
            &args![position, at(0x548), at(0x88), at(0x548), 1u32],
        )
        .u32();
    let duration = quiet_f32(e.global::<f32>(0x0101_81e8));
    let world = e.global::<u32>(0x011d_ea10);
    e.call(0x0045_8e20, &args![world, marker, duration]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn los_pick_handles_flag_null_target_and_empty_ray() {
        let mut e = Engine::new();
        for page in [
            0x1000,
            0x2000,
            0x3000,
            0x4000,
            0x5000,
            0x7000,
            0x8000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(0x0041_81e0, |_, _| 0x5000u32.into_ret());
        e.register(0x0047_7950, |_, _| true.into_ret());
        assert!(e
            .call(0x009b_1810, &args![0x1000u32, 0u32, 0u32, 0u32, 0u32])
            .bool());
        e.register(0x0047_7950, |_, _| false.into_ret());
        assert!(!e
            .call(0x009b_1810, &args![0x1000u32, 0u32, 0u32, 0u32, 0u32])
            .bool());

        let mut target_slots = vec![0u32; 0x1d0 / 4 + 1];
        target_slots[0x100 / 4] = 0x9004;
        target_slots[0x1d0 / 4] = 0x9008;
        e.put_vtable(0x3000, &target_slots);
        e.mem.set_u32(0x2000, 0x3000);
        let mut world_slots = vec![0u32; 0xc8 / 4 + 1];
        world_slots[0xc8 / 4] = 0x900c;
        e.put_vtable(0x4000, &world_slots);
        e.mem.set_u32(0x5000, 0x4000);
        e.register(0x9004, |_, _| false.into_ret());
        e.register(0x9008, |_, _| 0x7000u32.into_ret());
        e.register(0x900c, |_, _| Ret::default());
        e.register(0x004b_2960, |_, _| 1u32.into_ret());
        e.register(0x008d_6f30, |_, _| 0x6000u32.into_ret());
        e.register(0x0045_43c0, |_, _| 0x5000u32.into_ret());
        e.register(0x00c8_2870, |_, _| Ret::default());
        e.register(0x0040_1050, |_, _| Ret::default());
        e.register(0x0043_d450, |_, _| 0x7000u32.into_ret());
        e.register(0x0068_15c0, |_, _| 0x7100u32.into_ret());
        for address in [
            0x004a_3c20,
            0x008c_71b0,
            0x004a_39f0,
            0x0059_ce80,
            0x0063_c8a0,
            0x004a_3f70,
            0x004a_3da0,
            0x004a_3eb0,
            0x004a_3a70,
            0x004a_3fb0,
            0x004a_3bc0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x005d_be10, |_, _| 0u32.into_ret());
        e.register(0x0040_3e20, |_, _| 0x8000u32.into_ret());
        e.mem.set_f32(0x8000, 1.0);
        e.register(0x0045_bb20, |_, a| a[1].into_ret());
        e.register(0x008c_dd90, |_, _| 0x7100u32.into_ret());
        e.register(0x0046_0140, |_, _| 0x7100u32.into_ret());
        e.register(0x0072_6070, |_, _| 0u32.into_ret());
        e.register(0x0043_d4d0, |_, _| 0x8004u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x009b_1810,
                &args![0x1000u32, 1.0f32, 2.0f32, 3.0f32, 0x2000u32]
            )
            .bool());
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x900c)
                .count(),
            1
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x004a_3bc0)
                .count(),
            1
        );
    }

    #[test]
    fn los_hit_blocks_unless_first_ray_owner_allows_it() {
        let mut e = Engine::new();
        e.map(0x1000, 0x1000);
        e.map(0x2000, 0x1000);
        e.map(0x0108_f000, 0x1000);
        e.mem.set_f32(0x2000, -0.5);
        e.mem.set_u32(0x2004, 0);
        e.set_global(0x0108_f2a8, -0.98f64);
        e.register(0x008c_dd90, |_, _| 0x3000u32.into_ret());
        e.register(0x0046_0140, |_, _| 0x3000u32.into_ret());
        e.register(0x0072_6070, |_, _| 1i32.into_ret());
        e.register(0x004a_46b0, |_, _| 0x4000u32.into_ret());
        e.register(0x0069_6c30, |_, _| Ret::default());
        e.register(0x0043_9ef0, |_, _| Ret::default());
        e.register(0x004a_3760, |_, _| Ret::default());
        e.register(0x004a_51b0, |_, _| 0x2000u32.into_ret());
        e.register(0x0043_d4d0, |_, _| 0x2004u32.into_ret());
        e.register(0x0045_2370, |_, _| true.into_ret());
        e.with_stack(0x940, |e, block| {
            let ebp = block.addr() + 0x920;
            let blocked = ebp - 0x481;
            explosion_los_scan(
                e,
                Ptr::new(0x1000),
                ebp,
                ebp + 0x10,
                0x4f0,
                0x4fc,
                0x508,
                true,
            );
            assert_eq!(e.mem.u8(blocked), 1);
            e.mem.set_u8(blocked, 0);
            e.mem.set_u32(0x10c8, 0x5000);
            explosion_los_scan(
                e,
                Ptr::new(0x1000),
                ebp,
                ebp + 0x10,
                0x4f0,
                0x4fc,
                0x508,
                true,
            );
            assert_eq!(e.mem.u8(blocked), 0);
            explosion_los_scan(
                e,
                Ptr::new(0x1000),
                ebp,
                ebp + 0x10,
                0x8e0,
                0x8ec,
                0x8f8,
                false,
            );
            assert_eq!(e.mem.u8(blocked), 1);
        });
    }

    #[test]
    fn los_pick_retries_controller_probes_and_destroys_each_collector() {
        let mut e = Engine::new();
        for page in [
            0x1000,
            0x2000,
            0x3000,
            0x4000,
            0x5000,
            0x7000,
            0x8000,
            0x0108_f000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(0x0108_f2a8, -0.98f64);
        e.mem.set_f32(0x8000, 1.0);
        e.mem.set_f32(0x8008, -0.5);
        for offset in [0, 4, 8] {
            e.mem.set_f32(0x7100 + offset, 0.0);
        }
        let mut target_slots = vec![0u32; 0x214 / 4 + 1];
        target_slots[0x100 / 4] = 0x9004;
        target_slots[0x1d0 / 4] = 0x9008;
        target_slots[0x214 / 4] = 0x9010;
        e.put_vtable(0x3000, &target_slots);
        e.mem.set_u32(0x2000, 0x3000);
        let mut world_slots = vec![0u32; 0xc8 / 4 + 1];
        world_slots[0xc8 / 4] = 0x900c;
        e.put_vtable(0x4000, &world_slots);
        e.mem.set_u32(0x5000, 0x4000);
        e.register(0x9004, |_, _| true.into_ret());
        e.register(0x9008, |_, _| 0x7000u32.into_ret());
        e.register(0x9010, |_, _| 1u32.into_ret());
        let rays = Rc::new(Cell::new(0));
        let rays_for_callback = rays.clone();
        e.register_double(0x900c, move |e, _| {
            let number = rays_for_callback.get();
            rays_for_callback.set(number + 1);
            e.mem.set_f32(0x8008, if number == 0 { -0.5 } else { -1.0 });
            Ret::default()
        });
        e.register(0x0041_81e0, |_, _| 0x6000u32.into_ret());
        e.register(0x0047_7950, |_, _| false.into_ret());
        e.register(0x004b_2960, |_, _| 1u32.into_ret());
        e.register(0x008d_6f30, |_, _| 0x6100u32.into_ret());
        e.register(0x0045_43c0, |_, _| 0x5000u32.into_ret());
        e.register(0x0043_d450, |_, _| 0x7000u32.into_ret());
        e.register(0x0068_15c0, |_, _| 0x7100u32.into_ret());
        e.register(0x0084_d030, |_, _| 1.0f32.into_ret());
        e.register(0x0093_06d0, |_, _| 0u32.into_ret());
        e.register(0x005d_be10, |_, _| 0x8010u32.into_ret());
        e.register(0x0040_3e20, |_, _| 0x8000u32.into_ret());
        e.register(0x0045_bb20, |_, a| a[1].into_ret());
        e.register(0x008c_dd90, |_, _| 0x7200u32.into_ret());
        e.register(0x0046_0140, |_, _| 0x7200u32.into_ret());
        e.register(0x0072_6070, |_, _| 1i32.into_ret());
        e.register(0x004a_46b0, |_, _| 0x7300u32.into_ret());
        e.register(0x0069_6c30, |e, a| {
            e.mem.set_f32(a[0] + 0x10, 1.0);
            Ret::default()
        });
        e.register(0x004a_51b0, |_, _| 0x8008u32.into_ret());
        e.register(0x0045_7990, |_, _| 0.5f32.into_ret());
        e.register(0x0043_d4d0, |_, _| 0x8004u32.into_ret());
        for address in [
            0x00c8_2870,
            0x0040_1050,
            0x004a_3c20,
            0x008c_71b0,
            0x004a_39f0,
            0x0059_ce80,
            0x0063_c8a0,
            0x004a_3f70,
            0x004a_3da0,
            0x004a_3eb0,
            0x004a_3a70,
            0x004a_3fb0,
            0x004a_3bc0,
            0x0043_9ef0,
            0x004a_3760,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let visible = e
            .call(
                0x009b_1810,
                &args![0x1000u32, 0.0f32, 0.0f32, 0.0f32, 0x2000u32],
            )
            .bool();
        assert!(visible);
        assert_eq!(rays.get(), 2);
        let log = e.call_log.as_ref().unwrap();
        assert!(log.iter().any(|(address, _)| *address == 0x005d_be10));
        let constructors: Vec<_> = log
            .iter()
            .filter(|(address, _)| *address == 0x004a_3a70)
            .map(|(_, words)| words[0])
            .collect();
        let destructors: Vec<_> = log
            .iter()
            .filter(|(address, _)| *address == 0x004a_3bc0)
            .map(|(_, words)| words[0])
            .collect();
        assert_eq!(constructors.len(), 2);
        assert_eq!(destructors, vec![constructors[1], constructors[0]]);
    }

    #[test]
    fn process_targets_skips_empty_list_and_applies_non_actor_damage() {
        let mut e = Engine::new();
        for page in [
            0x1000,
            0x3000,
            0x4000,
            0x5000,
            0x6000,
            0x7000,
            0x8000,
            0x9000,
            0x0101_2000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(0x0101_2060, 0.0f64);
        e.mem.set_f32(0x1100, 10.0);
        e.mem.set_u32(0x5000, 0x6000);
        e.mem.set_u32(0x6000, 0x7000);
        e.mem.set_u32(0x8000, 1.0f32.to_bits());
        e.mem.set_u32(0x8004, 2.0f32.to_bits());
        e.mem.set_u32(0x8008, 3.0f32.to_bits());
        let mut explosion_slots = vec![0u32; 0x1f4 / 4 + 1];
        explosion_slots[0x168 / 4] = 0xa010;
        explosion_slots[0x1f4 / 4] = 0xa000;
        e.put_vtable(0x3000, &explosion_slots);
        e.mem.set_u32(0x1000, 0x3000);
        let mut target_slots = vec![0u32; 0x1f4 / 4 + 1];
        target_slots[0x100 / 4] = 0xa030;
        target_slots[0x144 / 4] = 0xa040;
        target_slots[0x1f4 / 4] = 0xa020;
        e.put_vtable(0x4000, &target_slots);
        e.mem.set_u32(0x7000, 0x4000);
        e.register(0xa000, |_, _| 0x8000u32.into_ret());
        e.register(0xa010, |_, _| 0u32.into_ret());
        e.register(0xa020, |_, _| 0x8000u32.into_ret());
        e.register(0xa030, |_, _| false.into_ret());
        e.register(0xa040, |_, _| Ret::default());
        e.register(0x0068_15c0, |_, _| 0x5000u32.into_ret());
        e.register(0x0072_6070, |_, _| 0u32.into_ret());
        e.register(0x0047_7ba0, |_, _| false.into_ret());
        e.register(0x0044_0da0, |_, _| false.into_ret());
        e.register(0x0043_9ef0, |_, a| a[1].into_ret());
        e.register(0x0045_7990, |_, _| 2.0f32.into_ret());
        e.register(0x0064_4930, |_, _| 4.0f32.into_ret());
        e.register(0x0064_7920, |_, _| 0.5f32.into_ret());
        e.register(0x008d_01e0, |_, _| 0.0f32.into_ret());
        let script_getters = Rc::new(Cell::new(0));
        let script_getters_for_callback = script_getters.clone();
        e.register_double(0x005d_43c0, move |e, a| {
            assert_eq!(a, &[0x7000]);
            let count = script_getters_for_callback.get();
            script_getters_for_callback.set(count + 1);
            if count == 1 {
                // The branch was chosen before this callback changes the
                // weapon. The base-form lookup still follows this getter.
                e.mem.set_u32(0x10e0, 0xa000);
            }
            0x9000u32.into_ret()
        });
        e.register(0x005a_c750, |_, _| Ret::default());
        e.register(0x0041_81e0, |_, _| 0x9000u32.into_ret());
        e.call_log = Some(vec![]);
        e.register(0x0082_56d0, |_, _| true.into_ret());
        e.call(0x009b_00a0, &args![0x1000u32]);
        assert_eq!(e.mem.u8(0x10d6), 1);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0xa040));

        e.register(0x0082_56d0, |_, _| false.into_ret());
        e.mem.set_u8(0x6006, 1);
        e.call(0x009b_00a0, &args![0x1000u32]);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0xa040));
        e.mem.set_u8(0x6006, 0);
        e.call(0x009b_00a0, &args![0x1000u32]);
        let hits: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0xa040)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x7000, 5.0f32.to_bits(), 0]);
        assert_eq!(script_getters.get(), 2);
        let calls = e.call_log.as_ref().unwrap();
        let getters: Vec<_> = calls
            .iter()
            .enumerate()
            .filter(|(_, call)| call.0 == 0x005d_43c0)
            .map(|(index, _)| index)
            .collect();
        let base = calls.iter().position(|call| call.0 == 0x0041_81e0).unwrap();
        assert!(getters[0] < getters[1] && getters[1] < base);
        let actions: Vec<_> = calls.iter().filter(|call| call.0 == 0x005a_c750).collect();
        assert_eq!(actions[1].1, vec![0x9000, 0x9000, 0x100]);
        e.set_global(0x0101_2060, f64::NAN);
        e.call(0x009b_00a0, &args![0x1000u32]);
        let hit_count = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0xa040)
            .count();
        assert_eq!(hit_count, 1, "unordered threshold must skip damage");
    }

    #[test]
    fn effect_flow_early_gates_and_empty_contact_cleanup() {
        let mut e = Engine::new();
        e.map(0x1000, 0x1000);
        e.map(0x7000, 0x1000);
        e.map(0x8000, 0x1000);
        e.call_log = Some(vec![]);
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        let this = Ptr::new(0x1000);
        explosion_apply_effects(&mut e, this);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.mem.set_u32(0x10a0, 0x6000);
        explosion_apply_effects(&mut e, this);
        assert_eq!(e.mem.u8(0x10d4), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);

        for address in [
            0x0062_99a0,
            0x00c8_5750,
            0x0068_15c0,
            0x0062_1060,
            0x00cb_d960,
            0x00c9_f1e0,
            0x0062_1100,
            0x0062_6870,
            0x005e_0b60,
            0x0062_1b60,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0043_d4d0, |_, a| {
            match a[0] {
                0x011d_10a4 => 0x8000u32,
                0x011d_0a78 => 0x8004u32,
                _ => panic!("setting address"),
            }
            .into_ret()
        });
        e.register(0x005e_0ad0, |_, _| 0x7000u32.into_ret());
        e.register(0x0053_7e90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x0045_8b50, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0062_2570, |e, a| {
            e.mem.set_u8(a[0], u8::from(a[1] != 0));
            a[0].into_ret()
        });
        e.register(0x0056_09f0, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(0x0082_56d0, |_, _| true.into_ret());
        e.mem.set_u8(0x10d5, 1);
        explosion_apply_effects(&mut e, this);
        assert_eq!(e.mem.u8(0x10d4), 1);
        let calls = e.call_log.as_ref().unwrap();
        let collector_cleanup = calls.iter().position(|call| call.0 == 0x0062_1100).unwrap();
        let storage_cleanup = calls.iter().position(|call| call.0 == 0x005e_0b60).unwrap();
        assert_eq!(e.mem.u8(0x10d6), 1);
        assert!(collector_cleanup < storage_cleanup);
        let calls_before = calls.len();
        explosion_apply_effects(&mut e, this);
        assert_eq!(e.call_log.as_ref().unwrap().len(), calls_before + 1);
    }

    #[test]
    fn effect_flow_sorts_returned_contact_storage() {
        let mut e = Engine::new();
        e.map(0x1000, 0x1000);
        e.map(0x7000, 0x1000);
        e.map(0x8000, 0x1000);
        e.mem.set_u32(0x10a0, 0x6000);
        e.mem.set_u8(0x10d5, 1);
        e.mem.set_u8(0x10d6, 1);
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        for address in [
            0x0062_99a0,
            0x00c8_5750,
            0x0068_15c0,
            0x0062_1060,
            0x00cb_d960,
            0x0063_2d40,
            0x0062_1100,
            0x0062_6870,
            0x005e_0b60,
            0x0062_1b60,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0043_d4d0, |_, a| {
            match a[0] {
                0x011d_10a4 => 0x8000u32,
                0x011d_0a78 => 0x8004u32,
                _ => panic!("setting address"),
            }
            .into_ret()
        });
        e.register(0x005e_0ad0, |_, _| 0x7000u32.into_ret());
        e.register(0x0053_7e90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x00c9_f1e0, |e, a| {
            e.mem.set_u32(a[1] + 0x14, 1);
            Ret::default()
        });
        e.register(0x0045_8b50, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0062_2570, |e, a| {
            e.mem.set_u8(a[0], u8::from(a[1] != 0));
            a[0].into_ret()
        });
        e.register(0x0056_09f0, |e, a| e.mem.u8(a[0]).into_ret());
        let count_calls = Rc::new(Cell::new(0));
        let count_calls_for_double = count_calls.clone();
        e.register_double(0x009a_fc50, move |_, _| {
            let calls = count_calls_for_double.get();
            count_calls_for_double.set(calls + 1);
            u32::from(calls == 0).into_ret()
        });
        e.register(0x009b_2e00, |_, a| {
            assert_eq!(a, &[0x7000, 1]);
            Ret::default()
        });
        explosion_apply_effects(&mut e, Ptr::new(0x1000));
        assert_eq!(count_calls.get(), 1);
        assert_eq!(e.mem.u8(0x10d4), 1);
    }

    #[test]
    fn effect_distance_gate_includes_equal_greater_and_unordered() {
        for (distance, expect_probe) in
            [(2.0f64, false), (1.0, true), (0.5, true), (f64::NAN, true)]
        {
            let mut e = Engine::new();
            e.map(0x1000, 0x1000);
            e.map(0x8000, 0x2000);
            e.map(0x0101_e000, 0x1000);
            e.mem.set_f32(0x9000, 1.0);
            e.mem.set_f32(0x0101_e340, 40.0);
            e.mem.set_u8(0x10d9, 1);
            e.call_log = Some(vec![]);
            e.register(0x0043_90c0, |_, _| false.into_ret());
            e.register(0x0043_d4d0, |_, _| 0x8000u32.into_ret());
            e.register(0x0041_81e0, |_, _| 0x6000u32.into_ret());
            e.register(0x004f_b7d0, |_, _| 0x6100u32.into_ret());
            e.register(0x0052_4c90, |_, _| 0x6200u32.into_ret());
            e.register(0x0045_bb80, |_, _| 0x6300u32.into_ret());
            e.register(0x0043_9ef0, |_, _| 0x6400u32.into_ret());
            e.register(0x0052_5420, |_, _| false.into_ret());
            e.register_double(0x0045_7990, move |_, _| Ret {
                st0: distance,
                ..Ret::default()
            });
            e.register(0x0040_3e20, |_, _| 0x9000u32.into_ret());
            e.register(0x004b_61d0, |_, _| true.into_ret());
            e.register(0x0067_33e0, |_, _| 0u32.into_ret());
            e.register(0x0068_a810, |_, _| 0u32.into_ret());
            e.register(0x0068_a830, |_, _| 0u32.into_ret());
            let mut found = false;
            e.with_stack(0x948, |e, block| {
                let frame = ExplosionEffectFrame {
                    ebp: (block.addr() + 0x938 + 15) & !15,
                };
                explosion_hit_effect(e, Ptr::new(0x1000), &frame, &mut found);
            });
            assert!(found);
            let probed = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .any(|call| call.0 == 0x004b_61d0);
            assert_eq!(probed, expect_probe, "distance {distance}");
        }
    }

    #[test]
    fn hit_sound_snapshots_both_vectors_before_callbacks() {
        let mut e = Engine::new();
        e.map(0x5000, 0x1000);
        e.map(0x6000, 0x1000);
        e.mem.set_u8(0x5000, b'h');
        let mut slots = vec![0; 0x14 / 4 + 1];
        slots[0x14 / 4] = 0x7000;
        e.put_vtable(0x3000, &slots);
        e.mem.set_u32(0x6118, 0x3000);
        e.register(0x7000, |_, _| 0x5000u32.into_ret());
        e.register(0x0068_15c0, |_, _| Ret::default());
        e.register(0x005f_36f0, |_, _| 0u32.into_ret());
        e.register(0x004a_0c10, |_, _| Ret::default());
        e.register(0x008d_6f30, |_, _| 0x7777u32.into_ret());
        e.register(0x0068_90b0, |_, a| {
            assert_eq!(
                a,
                &[
                    0x7777,
                    0.5f32.to_bits(),
                    0x5000,
                    4.0f32.to_bits(),
                    5.0f32.to_bits(),
                    6.0f32.to_bits(),
                    1.0f32.to_bits(),
                    2.0f32.to_bits(),
                    3.0f32.to_bits(),
                    1.0f32.to_bits(),
                    7,
                    0,
                ]
            );
            Ret::default()
        });
        e.with_stack(0x948, |e, block| {
            let frame = ExplosionEffectFrame {
                ebp: (block.addr() + 0x938 + 15) & !15,
            };
            for (offset, values) in [(0x194, [1.0f32, 2.0, 3.0]), (0x1a0, [4.0f32, 5.0, 6.0])] {
                for (index, value) in values.into_iter().enumerate() {
                    e.mem.set_f32(frame.at(offset) + index as u32 * 4, value);
                }
            }
            let first = frame.at(0x8c4);
            let second = frame.at(0x194);
            e.register_double(0x0050_8100, move |e, _| {
                e.mem.set_f32(first, 99.0);
                e.mem.set_f32(second, 98.0);
                Ret {
                    st0: 0.5,
                    ..Ret::default()
                }
            });
            explosion_play_hit_sound(e, Ptr::new(0x1000), &frame, 0x6100);
        });
    }

    #[test]
    fn decal_iterator_getter_leaves_index_for_entry_lookup() {
        let mut e = Engine::new();
        e.map(0x6000, 0x1000);
        e.call_log = Some(vec![]);
        e.register(0x0041_3f40, |_, _| 0x5000u32.into_ret());
        e.register(0x0062_e150, |_, _| 0x6000u32.into_ret());
        e.with_stack(0x948, |e, block| {
            let frame = ExplosionEffectFrame {
                ebp: (block.addr() + 0x938 + 15) & !15,
            };
            explosion_decal_for_hit(e, Ptr::new(0x1000), &frame, 0, 0, 0.0, 0.0, 0, 3);
            assert_eq!(
                e.call_log.as_ref().unwrap(),
                &vec![
                    (0x0041_3f40, vec![frame.at(0x7c8)]),
                    (0x0062_e150, vec![0x5000, 3]),
                ]
            );
        });
    }

    #[test]
    fn decal_record_stores_colour_between_callbacks_and_reloads_divisor() {
        let mut e = Engine::new();
        e.map(0x1000, 0x1000);
        e.map(0x5000, 0x4000);
        e.map(0x0101_e000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(0x6008, 0x7000);
        e.mem.set_u32(0x600c, u32::MAX);
        for (address, result) in [
            (0x0041_3f40, 0x5000u32),
            (0x0062_e150, 0x6000),
            (0x0055_9450, 0x7100),
            (0x004a_3a40, 0x7200),
            (0x0062_05a0, 0x44),
            (0x0043_b540, 0x7200),
            (0x008c_71b0, 0x7300),
            (0x004a_3a20, 0),
            (0x00c7_fa90, 0x7400),
            (0x0056_f930, 0x7500),
            (0x004a_1060, 1),
            (0x0045_19d0, 0x7600),
        ] {
            e.register_double(address, move |_, _| result.into_ret());
        }
        e.register(0x004a_37b0, |_, _| Ret::default());
        e.register(0x008a_ff10, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register(0x004a_4140, |_, _| 0x11u32.into_ret());
        for address in [0x004a_41c0, 0x004a_40e0, 0x009a_9350] {
            e.register(address, |_, _| Ret {
                st0: 0.25,
                ..Ret::default()
            });
        }
        e.register(0x004a_41e0, |_, _| 0u32.into_ret());
        e.with_stack(0x948, |e, block| {
            let frame = ExplosionEffectFrame {
                ebp: (block.addr() + 0x938 + 15) & !15,
            };
            e.mem.set_u32(frame.at(0x5b4), 0x44);
            let red = frame.at(0x81b);
            let green = frame.at(0x81c);
            e.register_double(0x004a_4180, move |e, _| {
                assert_eq!(e.mem.u8(red), 0x11);
                0x22u32.into_ret()
            });
            e.register_double(0x004a_4100, move |e, _| {
                assert_eq!(e.mem.u8(green), 0x22);
                0x33u32.into_ret()
            });
            let getter_count = Rc::new(Cell::new(0));
            let getter_count_for_double = getter_count.clone();
            e.register_double(0x004a_4220, move |e, _| {
                let count = getter_count_for_double.get();
                getter_count_for_double.set(count + 1);
                e.mem.set_f64(0x0101_e568, [255.0, 127.5, 85.0][count]);
                0x00ff_ffffu32.into_ret()
            });
            let output = frame.at(0x830);
            e.register_double(0x0041_6870, move |e, args| {
                assert_eq!(
                    &args[1..],
                    &[3.0f32.to_bits(), 2.0f32.to_bits(), 1.0f32.to_bits()]
                );
                for (index, value) in [3.0f32, 2.0, 1.0].into_iter().enumerate() {
                    e.mem.set_f32(0x7800 + index as u32 * 4, value);
                }
                0x7800u32.into_ret()
            });
            e.register(0x004a_3fe0, |_, args| {
                assert_eq!(args[0], 0x7600);
                assert_eq!(&args[2..], &[1, 0]);
                Ret::default()
            });
            explosion_decal_for_hit(e, Ptr::new(0x1000), &frame, 0x9000, 0x9100, 0.5, 0.0, 0, 0);
            assert_eq!(getter_count.get(), 3);
            assert_eq!(e.mem.f32(output), 3.0);
        });
    }

    fn engine_with(addrs: &[u32]) -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for page in [
            0x0101_2000,
            0x0101_7000,
            0x0101_8000,
            0x0102_3000,
            0x011c_3000,
            0x011c_7000,
            0x011c_9000,
            0x011d_d000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    fn mapped(e: &mut Engine, address: u32) -> Ptr {
        e.map(address & !0xfff, 0x1000);
        Ptr::new(address)
    }

    #[test]
    fn init_3d_stops_after_setting_a_missing_node() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x3000);
        let mut slots = vec![0; 0x1d0 / 4 + 1];
        slots[0x1c8 / 4] = 0x7000;
        slots[0x1d0 / 4] = 0x7004;
        e.put_vtable(0x3000, &slots);
        e.mem.set_u32(this.addr(), 0x3000);
        e.register(0x7000, |_, a| {
            assert_eq!(a, &[0x1000, 0]);
            Ret::default()
        });
        e.register(0x7004, |_, _| 0u32.into_ret());

        explosion_init_3d(&mut e, this);
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().map(|call| call.0).collect::<Vec<_>>(),
            [0x7000, 0x7004]
        );
    }

    #[test]
    fn init_3d_sets_node_transform_and_starts_animation_without_debug_clones() {
        let mut e = engine_with(&[
            0x008d_6f30,
            0x0043_d4d0,
            0x0044_0490,
            0x0044_0460,
            0x0043_0830,
            0x0068_15c0,
            0x00a5_9780,
            0x0043_fa80,
            0x0043_b230,
            0x00a6_d2d0,
        ]);
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x2000);
        mapped(&mut e, 0x3000);
        mapped(&mut e, 0x5000);
        mapped(&mut e, 0x6000);
        e.mem.set_i32(0x6000, 1);
        e.mem.set_u32(0x5000, 1.0f32.to_bits());
        e.mem.set_u32(0x5004, 2.0f32.to_bits());
        e.mem.set_u32(0x5008, 3.0f32.to_bits());
        let mut explosion_slots = vec![0; 0x1f4 / 4 + 1];
        explosion_slots[0x1c8 / 4] = 0x7000;
        explosion_slots[0x1d0 / 4] = 0x7004;
        explosion_slots[0x1f4 / 4] = 0x7008;
        e.put_vtable(0x3000, &explosion_slots);
        e.mem.set_u32(this.addr(), 0x3000);
        let mut node_slots = vec![0; 0x0c / 4 + 1];
        node_slots[0x0c / 4] = 0x7010;
        e.put_vtable(0x3100, &node_slots);
        e.mem.set_u32(0x2000, 0x3100);
        e.register(0x7000, |_, _| Ret::default());
        e.register(0x7004, |_, _| 0x2000u32.into_ret());
        e.register(0x7008, |_, _| 0x5000u32.into_ret());
        e.register(0x7010, |_, _| 0u32.into_ret());
        e.register(0x008d_6f30, |_, _| 0u32.into_ret());
        e.register(0x0043_d4d0, |_, _| 0x6000u32.into_ret());
        e.register(0x0043_0830, |_, _| 0x5000u32.into_ret());

        explosion_init_3d(&mut e, this);
        let log = e.call_log.unwrap();
        let addresses: Vec<_> = log.iter().map(|call| call.0).collect();
        assert_eq!(addresses[0], 0x7000);
        assert_eq!(addresses[1], 0x7004);
        assert!(addresses.contains(&0x0044_0490));
        assert!(addresses.contains(&0x0044_0460));
        assert_eq!(addresses.last(), Some(&0x00a6_d2d0));
        let rotate = log.iter().find(|call| call.0 == 0x00a5_9780).unwrap();
        assert_eq!(
            rotate.1[1..],
            [3.0f32.to_bits(), 2.0f32.to_bits(), 1.0f32.to_bits()]
        );
        assert!(!addresses.contains(&0x00aa_13e0));
    }

    #[test]
    fn init_3d_scales_the_named_radius_node_and_extends_distant_fade_range() {
        let mut e = engine_with(&[
            0x008d_6f30,
            0x0043_d4d0,
            0x00b4_dfd0,
            0x004a_ae30,
            0x0064_4930,
            0x0043_c490,
            0x0045_7990,
            0x0044_0490,
            0x0044_0460,
            0x0043_0830,
            0x0068_15c0,
            0x00a5_9780,
            0x0043_fa80,
            0x0057_84b0,
            0x0043_b230,
            0x00a6_d2d0,
        ]);
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x2000);
        mapped(&mut e, 0x3000);
        mapped(&mut e, 0x4000);
        mapped(&mut e, 0x5000);
        mapped(&mut e, 0x6000);
        e.map(0x0101_6000, 0x1000);
        e.set_global(0x0101_6970, 0x7f80_0001u32);
        e.mem.set_i32(0x6000, 0);
        e.mem.set_u32(0x5000, 0x7f80_0001);
        let mut explosion_slots = vec![0; 0x1f4 / 4 + 1];
        explosion_slots[0x1c8 / 4] = 0x7000;
        explosion_slots[0x1d0 / 4] = 0x7004;
        explosion_slots[0x1f4 / 4] = 0x7008;
        e.put_vtable(0x3000, &explosion_slots);
        e.mem.set_u32(this.addr(), 0x3000);
        let mut node_slots = vec![0; 0x0c / 4 + 1];
        node_slots[0x0c / 4] = 0x7010;
        e.put_vtable(0x3100, &node_slots);
        e.mem.set_u32(0x2000, 0x3100);
        let mut scene_slots = vec![0; 0x10 / 4 + 1];
        scene_slots[0x10 / 4] = 0x7014;
        e.put_vtable(0x3200, &scene_slots);
        e.mem.set_u32(0x4000, 0x3200);
        e.register(0x7000, |_, _| Ret::default());
        e.register(0x7004, |_, _| 0x2000u32.into_ret());
        e.register(0x7008, |_, _| 0x5000u32.into_ret());
        e.register(0x7010, |_, _| 0x4000u32.into_ret());
        e.register(0x7014, |_, _| 0x4500u32.into_ret());
        e.register(0x008d_6f30, |_, _| 0u32.into_ret());
        e.register(0x0043_d4d0, |_, _| 0x6000u32.into_ret());
        e.register(0x004a_ae30, |_, _| 0x4300u32.into_ret());
        e.register(0x0064_4930, |_, _| 10.0f64.into_ret());
        e.register(0x0043_c490, |_, _| 0x4400u32.into_ret());
        e.register(0x0045_7990, |_, _| 2.0f64.into_ret());
        e.register(0x0043_0830, |_, _| 0x5000u32.into_ret());

        explosion_init_3d(&mut e, this);
        let log = e.call_log.unwrap();
        let fade = log.iter().find(|call| call.0 == 0x00b4_dfd0).unwrap();
        assert_eq!(fade.1, [0x4500, 0x7fc0_0001, 0x7fc0_0001]);
        let scaled = log.iter().find(|call| call.0 == 0x0044_0490).unwrap();
        assert_eq!(scaled.1, [0x2000, 5.0f32.to_bits()]);
        let rotate = log.iter().find(|call| call.0 == 0x00a5_9780).unwrap();
        assert_eq!(rotate.1[3], 0x7fc0_0001);
        assert_eq!(log.iter().filter(|call| call.0 == 0x004a_ae30).count(), 1);
    }

    #[test]
    fn init_3d_debug_loop_builds_and_releases_each_rigid_body_cinfo() {
        let mut e = engine_with(&[
            0x008d_6f30,
            0x0043_d4d0,
            0x0044_0490,
            0x0044_0460,
            0x0043_0830,
            0x0068_15c0,
            0x00a5_9780,
            0x0043_fa80,
            0x0041_81e0,
            0x006d_3b00,
            0x0044_8a80,
            0x00a5_bdd0,
            0x004a_d050,
            0x0047_6c00,
            0x004d_fec0,
            0x004b_c1f0,
            0x00c8_f510,
            0x0050_7610,
            0x006e_4340,
            0x0056_d360,
            0x0050_76c0,
            0x005d_be10,
            0x0062_2290,
            0x0055_9450,
            0x0056_f0f0,
            0x0056_f110,
            0x00c6_b980,
            0x0054_68d0,
            0x0053_7bb0,
            0x004a_e750,
            0x004b_50c0,
            0x00cb_d520,
            0x00c8_ef80,
            0x0056_d730,
            0x004a_d270,
            0x0043_b230,
            0x00a6_d2d0,
        ]);
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x2000);
        mapped(&mut e, 0x3000);
        mapped(&mut e, 0x4000);
        mapped(&mut e, 0x5000);
        mapped(&mut e, 0x6000);
        e.map(0x8000, 0x10_000);
        for page in [
            0x0101_1000,
            0x0101_e000,
            0x0102_0000,
            0x0102_9000,
            0x0102_f000,
            0x0103_0000,
            0x011a_5000,
            0x011f_4000,
            0x0126_8000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_i32(0x6000, 2);
        e.mem.set_u32(0x5000, 1.0f32.to_bits());
        e.mem.set_u32(0x5004, 2.0f32.to_bits());
        e.mem.set_u32(0x5008, 3.0f32.to_bits());
        e.set_global(0x0102_0758, 10.0f64);
        e.set_global(0x0102_f070, 32.0f64);
        e.set_global(0x0102_918c, 8.0f32);
        e.set_global(0x011a_5808, 11.0f32);
        e.set_global(0x011a_5804, 12.0f32);
        e.set_global(0x011a_5800, 13.0f32);
        e.set_global(0x011a_57fc, 14.0f32);
        let mut explosion_slots = vec![0; 0x1f4 / 4 + 1];
        explosion_slots[0x1c8 / 4] = 0x7000;
        explosion_slots[0x1d0 / 4] = 0x7004;
        explosion_slots[0x1f4 / 4] = 0x7008;
        e.put_vtable(0x3000, &explosion_slots);
        e.mem.set_u32(this.addr(), 0x3000);
        let mut node_slots = vec![0; 0xdc / 4 + 1];
        node_slots[0x0c / 4] = 0x7010;
        node_slots[0xdc / 4] = 0x7014;
        e.put_vtable(0x4000, &node_slots);
        e.mem.set_u32(0x2000, 0x4000);
        e.put_vtable(0x4200, &node_slots);
        e.register(0x7000, |_, _| Ret::default());
        e.register(0x7004, |_, _| 0x2000u32.into_ret());
        e.register(0x7008, |_, _| 0x5000u32.into_ret());
        e.register(0x7010, |_, _| 0u32.into_ret());
        e.register(0x7014, |_, _| Ret::default());
        e.register(0x008d_6f30, |_, _| 0u32.into_ret());
        e.register(0x0043_d4d0, |_, _| 0x6000u32.into_ret());
        e.register(0x0043_0830, |_, _| 0x5000u32.into_ret());
        e.register(0x0041_81e0, |_, _| 0x5000u32.into_ret());
        e.register(0x004d_fec0, |_, _| 0.5f64.into_ret());
        e.register(0x0047_6c00, |_, _| 0x5000u32.into_ret());
        e.register(0x00a5_ecb0, |e, a| {
            let collision = a[0] + 0x100;
            e.mem.set_u32(collision, 0x4200);
            collision.into_ret()
        });
        for constructor in [0x0056_d580, 0x0056_e950, 0x00c4_3660] {
            e.register(constructor, |_, a| (a[0] + 0x100).into_ret());
        }
        e.register(0x00c8_f510, |e, a| {
            // The PC reserves one 0xe0-byte bhkRigidBodyCinfo at EBP-0x1c0.
            for offset in 0..0xe0 {
                e.mem.set_u8(a[0] + offset, 0xa5);
            }
            a[0].into_ret()
        });
        e.register(0x0056_d380, |e, a| {
            assert_eq!(e.mem.u8(a[1] + 0xdf), 0xa5);
            (a[0] + 0x100).into_ret()
        });
        e.register(0x005d_be10, |_, a| {
            assert!(a.is_empty());
            0x5600u32.into_ret()
        });
        e.register(0x0062_2290, |_, a| {
            assert_eq!(a.len(), 4);
            assert_eq!(&a[1..], &[4, 0x5600, 0]);
            a[0].into_ret()
        });
        e.register(0x0055_9450, |_, _| 0u32.into_ret());
        e.register(0x004a_e750, |_, _| 0x5000u32.into_ret());
        e.register(0x00cb_d520, |e, a| {
            let info = a[1];
            assert_eq!(e.mem.u32(info + 4), 0x5000);
            assert_eq!(e.mem.f32(info + 8), 11.0);
            assert_eq!(e.mem.f32(info + 0x0c), 12.0);
            assert_eq!(e.mem.f32(info + 0x10), 13.0);
            assert_eq!(e.mem.f32(info + 0x14), 14.0);
            Ret::default()
        });
        let allocations = Rc::new(Cell::new(0u32));
        let allocation_counter = allocations.clone();
        e.register_double(0x00aa_13e0, move |_, _| {
            let index = allocation_counter.get();
            allocation_counter.set(index + 1);
            (0x8000 + index * 0x200).into_ret()
        });

        explosion_init_3d(&mut e, this);
        assert_eq!(allocations.get(), 12);
        let log = e.call_log.unwrap();
        let count = |address| log.iter().filter(|call| call.0 == address).count();
        assert_eq!(count(0x00c8_f510), 2);
        assert_eq!(count(0x00c8_ef80), 2);
        assert_eq!(count(0x0056_d730), 2);
        assert_eq!(count(0x004a_d270), 2);
        assert_eq!(count(0x00a6_d2d0), 1);
        let cinfo = log.iter().find(|call| call.0 == 0x00c8_f510).unwrap().1[0];
        let cloning = log.iter().find(|call| call.0 == 0x004a_d050).unwrap().1[0];
        assert_eq!(cinfo + 0xe0 + 0x3c, cloning);
        assert_eq!(
            log.iter().find(|call| call.0 == 0x0062_2290).unwrap().1[0],
            cinfo - 4
        );
        for address in [
            0x0050_7610,
            0x006e_4340,
            0x0056_d360,
            0x0050_76c0,
            0x0056_f0f0,
            0x0056_f110,
            0x0056_d730,
        ] {
            assert!(log
                .iter()
                .filter(|call| call.0 == address)
                .all(|call| call.1[0] == cinfo));
        }
        assert_eq!(
            log.iter().find(|call| call.0 == 0x0056_d380).unwrap().1[1],
            cinfo
        );
        assert_eq!(
            log.iter().find(|call| call.0 == 0x00cb_d520).unwrap().1[1],
            cinfo - 0x3c
        );
        for window in log.windows(2) {
            if window[0].0 == 0x0056_d730 {
                assert_eq!(window[0].1, [cinfo]);
                assert_eq!(window[1].0, 0x004a_d270);
            }
        }
        for iteration in 0..2 {
            let raw = 0x8000 + iteration * 0xc00;
            let calls = |address| {
                log.iter()
                    .filter(|call| call.0 == address)
                    .nth(iteration as usize)
                    .unwrap()
            };
            assert_eq!(calls(0x00a5_ecb0).1, [raw, 0]);
            assert_eq!(calls(0x004b_c1f0).1[0], raw + 0x100);
            assert_eq!(calls(0x0056_d580).1, [raw + 0x200, raw + 0x100]);
            assert_eq!(calls(0x0056_e950).1[0], raw + 0x400);
            assert_eq!(calls(0x0056_f110).1[1], raw + 0x500);
            assert_eq!(calls(0x0056_d380).1, [raw + 0x600, cinfo]);
            assert_eq!(calls(0x00c6_b980).1, [raw + 0x300, raw + 0x700]);
            assert_eq!(calls(0x00c4_3660).1, [raw + 0x800]);
            assert_eq!(calls(0x0054_68d0).1, [raw + 0x900, 0xe]);
            assert_eq!(calls(0x00c8_ef80).1[0], raw + 0x700);
        }
        let location = log.iter().find(|call| call.0 == 0x004b_c1f0).unwrap();
        assert_eq!(
            location.1[1..],
            [5.0f32.to_bits(), 5.0f32.to_bits(), 37.0f32.to_bits()]
        );
    }

    #[test]
    fn initialize_process_and_face_effect_gates_precede_distant_attachment() {
        for (initially_has_process, power, allocation_succeeds) in [
            (false, 4.0f32, true),
            (false, 4.0, false),
            (true, 4.0, true),
            (false, 0.5, true),
            (true, 0.5, true),
        ] {
            let mut e = engine_with(&[
                0x0042_ce10,
                0x0040_1000,
                0x008d_7510,
                0x0043_9ef0,
                0x0045_7990,
                0x0064_4930,
                0x0064_7920,
                0x0040_3e20,
                0x0052_99a0,
                0x008d_6f30,
                0x009a_d670,
                0x0043_d4d0,
                0x0040_8d60,
                0x0096_d450,
            ]);
            let this = mapped(&mut e, 0x1000);
            mapped(&mut e, 0x2000);
            mapped(&mut e, 0x4000);
            mapped(&mut e, 0x5000);
            mapped(&mut e, 0x8000);
            let mut slots = vec![0; 0x1f4 / 4 + 1];
            slots[0x1d0 / 4] = 0x6000;
            slots[0x1f4 / 4] = 0x6100;
            e.put_vtable(0x3000, &slots);
            e.mem.set_u32(this.addr(), 0x3000);
            e.mem.set_u32(0x2000, 0x3000);
            e.mem.set_u32(
                this.addr() + 0x68,
                if initially_has_process { 0x9300 } else { 0 },
            );
            e.mem.set_f32(0x5000, 1.0);
            e.mem.set_i32(0x8000, 0);
            e.mem.set_u8(0x8100, 0);
            e.set_global(0x011d_df38, 0x7000u32);
            e.set_global(0x011d_ea3c, 0x2000u32);
            e.set_global(0x011c_9708, 0x9500u32);
            e.register(0x0042_ce10, |_, _| true.into_ret());
            e.register_double(0x0040_1000, move |_, _| {
                (if allocation_succeeds { 0x9000u32 } else { 0 }).into_ret()
            });
            e.register(0x008d_7510, |_, _| 0x9200u32.into_ret());
            e.register(0x0000_6000, |_, _| 0u32.into_ret());
            e.register(0x0000_6100, |_, _| 0x4000u32.into_ret());
            e.register(0x0043_9ef0, |_, a| a[1].into_ret());
            e.register(0x0045_7990, |_, _| 3.0f32.into_ret());
            e.register(0x0064_4930, |_, _| 2.0f32.into_ret());
            e.register_double(0x0064_7920, move |_, a| {
                assert_eq!(a, &[2.0f32.to_bits(), 3.0f32.to_bits()]);
                power.into_ret()
            });
            e.register(0x0040_3e20, |_, _| 0x5000u32.into_ret());
            e.register(0x0043_d4d0, |_, _| 0x8000u32.into_ret());
            e.register(0x0040_8d60, |_, _| 0x8100u32.into_ret());

            explosion_initialize(&mut e, this);
            assert_eq!(
                e.mem.u32(this.addr() + 0x68),
                if initially_has_process {
                    0x9300
                } else if allocation_succeeds {
                    0x9200
                } else {
                    0
                }
            );
            let log = e.call_log.unwrap();
            let calls: Vec<_> = log.iter().map(|call| call.0).collect();
            let init_3d = calls
                .iter()
                .position(|&address| address == 0x009a_d670)
                .unwrap();
            let effects: Vec<_> = log
                .iter()
                .enumerate()
                .filter(|(_, call)| call.0 == 0x0052_99a0)
                .collect();
            assert_eq!(effects.len(), usize::from(power > 1.0));
            if let Some((index, call)) = effects.first() {
                assert!(*index < init_3d);
                assert_eq!(call.1, [0x9500, 1.0f32.to_bits(), 0]);
            }
            assert_eq!(
                calls
                    .iter()
                    .filter(|&&address| address == 0x0040_1000)
                    .count(),
                usize::from(!initially_has_process)
            );
            assert_eq!(
                calls
                    .iter()
                    .filter(|&&address| address == 0x008d_7510)
                    .count(),
                usize::from(!initially_has_process && allocation_succeeds)
            );
            assert_eq!(
                calls
                    .iter()
                    .filter(|&&address| address == 0x0096_d450)
                    .count(),
                usize::from(!initially_has_process)
            );
        }
    }

    #[test]
    fn initialize_attached_cell_adds_reference_and_stores_animation_age() {
        let mut e = engine_with(&[
            0x0042_ce10,
            0x0043_9ef0,
            0x0045_7990,
            0x0064_4930,
            0x0064_7920,
            0x0040_3e20,
            0x008d_6f30,
            0x0054_71e0,
            0x009a_d670,
            0x0054_8230,
            0x004b_6220,
            0x0043_d4d0,
            0x0040_8d60,
        ]);
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x2000);
        mapped(&mut e, 0x4000);
        mapped(&mut e, 0x5000);
        mapped(&mut e, 0x8000);
        let mut slots = vec![0; 0x1f4 / 4 + 1];
        slots[0x1c4 / 4] = 0x6200;
        slots[0x1d0 / 4] = 0x6000;
        slots[0x1f4 / 4] = 0x6100;
        e.put_vtable(0x3000, &slots);
        e.mem.set_u32(this.addr(), 0x3000);
        e.mem.set_u32(0x2000, 0x3000);
        e.mem.set_u32(this.addr() + 0x68, 0x9300);
        e.mem.set_f32(0x4008, 0.0);
        e.mem.set_f32(0x5000, 10.0);
        e.mem.set_i32(0x8000, 0);
        e.mem.set_u8(0x8100, 0);
        e.set_global(0x011d_df38, 0x7000u32);
        e.set_global(0x011d_ea3c, 0x2000u32);
        e.register(0x0042_ce10, |_, _| true.into_ret());
        e.register(0x0000_6000, |_, _| 0u32.into_ret());
        e.register(0x0000_6100, |_, _| 0x4000u32.into_ret());
        e.register(0x0000_6200, |_, _| Ret::default());
        e.register(0x0043_9ef0, |_, a| a[1].into_ret());
        e.register(0x0045_7990, |_, _| 3.0f32.into_ret());
        e.register(0x0064_4930, |_, _| 2.0f32.into_ret());
        e.register(0x0064_7920, |_, _| 0.0f32.into_ret());
        e.register(0x0040_3e20, |_, _| 0x5000u32.into_ret());
        e.register(0x008d_6f30, |_, _| 0x7000u32.into_ret());
        e.register(0x0054_71e0, |_, _| 100.0f32.into_ret());
        e.register(0x004b_6220, |_, a| {
            assert_eq!(a, &[0, 0x0102_054c, 0x0103_275c]);
            6.5f32.into_ret()
        });
        e.register(0x0043_d4d0, |_, _| 0x8000u32.into_ret());
        e.register(0x0040_8d60, |_, _| 0x8100u32.into_ret());

        explosion_initialize(&mut e, this);
        assert_eq!(e.mem.f32(this.addr() + 0x90), 6.5);
        assert_eq!(e.mem.u8(this.addr() + 0xd9), 1);
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0x0054_8230 && call.1 == [0x7000, 0x1000, 0]));
        assert!(!log.iter().any(|call| call.0 == 0x0096_d450));
    }

    #[test]
    fn initialize_reloads_impact_object_and_radiation_inputs_after_callbacks() {
        let mut e = engine_with(&[
            0x0042_ce10,
            0x0043_9ef0,
            0x0045_7990,
            0x0064_4930,
            0x0064_7920,
            0x0040_3e20,
            0x008d_6f30,
            0x0054_71e0,
            0x009a_d670,
            0x0057_5d70,
            0x0043_0830,
            0x0046_98a0,
            0x0054_ca90,
            0x0054_8230,
            0x004b_6220,
            0x00ad_8ce0,
            0x004f_d3c0,
            0x0091_85e0,
            0x004f_b970,
            0x0043_d4d0,
            0x0040_8d60,
        ]);
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x2000);
        mapped(&mut e, 0x4000);
        mapped(&mut e, 0x5000);
        mapped(&mut e, 0x8000);
        mapped(&mut e, 0xb000);
        let mut slots = vec![0; 0x1f4 / 4 + 1];
        slots[0x168 / 4] = 0x6300;
        slots[0x1c4 / 4] = 0x6200;
        slots[0x1d0 / 4] = 0x6000;
        slots[0x1f4 / 4] = 0x6100;
        e.put_vtable(0x3000, &slots);
        e.mem.set_u32(this.addr(), 0x3000);
        e.mem.set_u32(0x2000, 0x3000);
        e.mem.set_u32(this.addr() + 0x68, 0x9300);
        e.mem.set_f32(0x4008, 0.0);
        e.mem.set_f32(0x5000, 1.0);
        e.mem.set_i32(0x8000, 0);
        e.mem.set_u8(0x8100, 0);
        e.mem.set_f32(0xb0a0, 2.0);
        e.mem.set_f32(0xb1a0, 2.0);
        e.set_global(0x011d_df38, 0x7000u32);
        e.set_global(0x011d_ea3c, 0x2000u32);
        e.set_global(0x011c_3f2c, 0xd001u32);
        e.set_global(0x0101_2060, 0.0f64);
        e.register(0x0042_ce10, |_, _| false.into_ret());
        e.register(0x0000_6000, |_, _| 0u32.into_ret());
        e.register(0x0000_6100, |_, _| 0x4000u32.into_ret());
        e.register(0x0000_6200, |_, _| Ret::default());
        e.register(0x0000_6300, |_, _| 0u32.into_ret());
        e.register(0x0043_9ef0, |_, a| a[1].into_ret());
        e.register(0x0045_7990, |_, _| 3.0f32.into_ret());
        e.register(0x0064_4930, |_, _| 2.0f32.into_ret());
        e.register(0x0064_7920, |_, _| 0.5f32.into_ret());
        e.register(0x0040_3e20, |_, _| 0x5000u32.into_ret());
        e.register(0x008d_6f30, |_, _| 0x7000u32.into_ret());
        e.register(0x0054_71e0, |_, _| 100.0f32.into_ret());
        let base_calls = Rc::new(Cell::new(0u32));
        let base_calls_for_double = Rc::clone(&base_calls);
        e.register_double(0x0041_81e0, move |_, _| {
            let count = base_calls_for_double.get();
            base_calls_for_double.set(count + 1);
            (if count == 0 { 0xb000u32 } else { 0xb100u32 }).into_ret()
        });
        let impact_object_calls = Rc::new(Cell::new(0u32));
        let impact_object_calls_for_double = Rc::clone(&impact_object_calls);
        e.register_double(0x004f_d380, move |e, a| {
            let count = impact_object_calls_for_double.get();
            impact_object_calls_for_double.set(count + 1);
            if count == 0 {
                assert_eq!(a, &[0xb000]);
                0xc000u32.into_ret()
            } else {
                assert_eq!(a, &[0xb100]);
                e.set_global(0x011c_3f2c, 0xd002u32);
                0xd000u32.into_ret()
            }
        });
        e.register(0x0057_5d70, |_, _| 0xa100u32.into_ret());
        e.register(0x0043_0830, |_, _| 0xa300u32.into_ret());
        e.register(0x0047_7950, |_, _| true.into_ret());
        e.register_double(0x008d_01e0, |e, _| {
            e.set_global(0x0101_2060, 100.0f64);
            1.0f32.into_ret()
        });
        e.register(0x0043_d4d0, |_, _| 0x8000u32.into_ret());
        e.register(0x0040_8d60, |_, _| 0x8100u32.into_ret());

        explosion_initialize(&mut e, this);
        assert_eq!(impact_object_calls.get(), 2);
        let log = e.call_log.unwrap();
        let probe = log.iter().position(|call| call.0 == 0x004f_d380).unwrap();
        let observed: Vec<_> = log[probe..probe + 9].iter().map(|call| call.0).collect();
        assert_eq!(
            observed,
            [
                0x004f_d380,
                0x0057_5d70,
                0x008d_6f30,
                0x0043_0830,
                0x0000_6100,
                0x0041_81e0,
                0x004f_d380,
                0x0046_98a0,
                0x0041_81e0,
            ]
        );
        assert!(log.iter().any(|call| call.0 == 0x0046_98a0
            && call.1 == [0xd002, 0xd000, 0x4000, 0xa300, 0x7000, 0xa100, 0, 0, 0]));
        assert!(!log.iter().any(|call| call.0 == 0x0042_2220));
    }

    #[test]
    fn initialize_sound_water_debug_and_script_keep_snapshot_arguments() {
        let mut e = engine_with(&[
            0x0042_ce10,
            0x0043_9ef0,
            0x0045_7990,
            0x0064_4930,
            0x0064_7920,
            0x0040_3e20,
            0x0052_99a0,
            0x0054_71e0,
            0x009a_d670,
            0x0041_81e0,
            0x004f_d380,
            0x0047_7950,
            0x004f_b070,
            0x0054_ca90,
            0x0054_8230,
            0x0045_0f90,
            0x0054_97a0,
            0x0047_6ab0,
            0x004b_6220,
            0x004f_d3c0,
            0x0091_85e0,
            0x0084_e3a0,
            0x0093_3150,
            0x0041_8900,
            0x0048_3710,
            0x008d_1960,
            0x004f_b970,
            0x0068_15c0,
            0x004a_4240,
            0x0068_9210,
            0x0041_a250,
            0x0045_3a70,
            0x00ad_7550,
            0x00ad_8b60,
            0x00ad_8830,
            0x0041_4430,
            0x004b_29b0,
            0x00a5_a040,
            0x0044_0460,
            0x0045_8e20,
            0x0076_b660,
        ]);
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x2000);
        mapped(&mut e, 0x4000);
        mapped(&mut e, 0x5000);
        mapped(&mut e, 0x6000);
        mapped(&mut e, 0x7000);
        mapped(&mut e, 0x8000);
        mapped(&mut e, 0xb000);
        let mut explosion_slots = vec![0; 0x1f4 / 4 + 1];
        explosion_slots[0x168 / 4] = 0x6300;
        explosion_slots[0x1c4 / 4] = 0x6200;
        explosion_slots[0x1d0 / 4] = 0x6000;
        explosion_slots[0x1f4 / 4] = 0x6100;
        e.put_vtable(0x3000, &explosion_slots);
        e.mem.set_u32(this.addr(), 0x3000);
        e.mem.set_u32(0x2000, 0x3000);
        e.mem.set_u32(this.addr() + 0x68, 0x9300);
        e.mem.set_f32(this.addr() + 0x94, 2.0);
        e.mem.set_f32(0x4000, 10.0);
        e.mem.set_f32(0x4004, 20.0);
        e.mem.set_f32(0x4008, 0.0);
        e.mem.set_f32(0x5000, 1.0);
        e.mem.set_f32(0x5004, 3.0);
        e.mem.set_f32(0x5008, 1.0);
        e.mem.set_f32(0x500c, 1.0);
        e.mem.set_i32(0x5500, 1);
        e.mem.set_u8(0x5600, 1);
        e.set_global(0x011d_df38, 0x7000u32);
        e.set_global(0x011d_ea3c, 0x2000u32);
        e.set_global(0x011d_ea10, 0xb000u32);
        e.set_global(0x011c_9708, 0xb100u32);
        e.set_global(0x0102_3934, 0.65f32);
        e.set_global(0x0101_84f4, 2.5f32);
        e.set_global(0x0101_7b78, 10.0f32);
        e.register(0x0042_ce10, |_, _| false.into_ret());
        e.register(0x0000_6000, |_, _| 0x6000u32.into_ret());
        e.register(0x0000_6100, |_, _| 0x4000u32.into_ret());
        e.register(0x0000_6200, |_, _| Ret::default());
        e.register(0x0000_6300, |_, _| 0u32.into_ret());
        let mut node_slots = vec![0; 0x10 / 4 + 1];
        node_slots[0xc / 4] = 0x6400;
        e.put_vtable(0xc000, &node_slots);
        e.mem.set_u32(0x6000, 0xc000);
        e.register(0x0000_6400, |_, _| 0x7100u32.into_ret());
        let mut scene_slots = vec![0; 0x10 / 4 + 1];
        scene_slots[0x10 / 4] = 0x6500;
        e.put_vtable(0xd000, &scene_slots);
        e.mem.set_u32(0x7100, 0xd000);
        e.register(0x0000_6500, |_, _| 0x8000u32.into_ret());
        let mut world_slots = vec![0; 0xdc / 4 + 1];
        world_slots[0xdc / 4] = 0x6600;
        e.put_vtable(0xe000, &world_slots);
        e.mem.set_u32(0x7000, 0xe000);
        e.register(0x0000_6600, |_, a| {
            assert_eq!(a, &[0x7000, 0x6000, 1]);
            Ret::default()
        });
        e.register(0x0043_9ef0, |_, a| a[1].into_ret());
        e.register(0x0045_7990, |_, _| 3.0f32.into_ret());
        e.register(0x0064_4930, |_, _| 2.0f32.into_ret());
        e.register(0x0064_7920, |_, _| 4.0f32.into_ret());
        e.register(0x0040_3e20, |_, a| {
            match a[0] {
                0x011d_0280 => 0x5004u32,
                0x011d_0da4 => 0x5008,
                0x011d_0e34 => 0x500c,
                _ => 0x5000,
            }
            .into_ret()
        });
        e.register(0x008d_6f30, |e, a| {
            if a[0] == e.global::<u32>(0x011d_ea3c) {
                0xa000u32.into_ret()
            } else {
                0x9000u32.into_ret()
            }
        });
        e.register(0x0054_71e0, |_, _| 100.0f32.into_ret());
        e.register(0x0041_81e0, |_, _| 0xb000u32.into_ret());
        e.register(0x004f_d380, |_, _| 0u32.into_ret());
        e.register(0x0047_7950, |_, _| true.into_ret());
        e.register(0x004f_b070, |_, _| 0u32.into_ret());
        e.register(0x0054_97a0, |_, _| 0x7000u32.into_ret());
        e.register(0x004b_6220, |_, _| 0.3f32.into_ret());
        let handle_calls = Rc::new(Cell::new(0u32));
        let handle_calls_for_double = Rc::clone(&handle_calls);
        e.register_double(0x00ad_8ce0, move |_, a| {
            if a[0] == 0x10ac {
                let count = handle_calls_for_double.get();
                handle_calls_for_double.set(count + 1);
                (count != 0).into_ret()
            } else {
                (a[0] != 0x10b8).into_ret()
            }
        });
        e.register(0x004f_d3c0, |_, _| 0x7300u32.into_ret());
        e.register(0x0091_85e0, |_, _| 0u32.into_ret());
        e.register(0x0084_e3a0, |_, _| 0x42u32.into_ret());
        e.register(0x0093_3150, |_, _| 0xc100u32.into_ret());
        e.register(0x004f_b970, |_, _| true.into_ret());
        e.register(0x004a_4240, |_, _| 0.0f32.into_ret());
        let matrix_address = Rc::new(Cell::new(0u32));
        let matrix_for_set = Rc::clone(&matrix_address);
        e.register_double(0x004a_0c90, move |e, a| {
            matrix_for_set.set(a[0]);
            for index in 0..9 {
                e.mem.set_u32(a[0] + index * 4, index + 1);
            }
            Ret::default()
        });
        let matrix_for_particle = Rc::clone(&matrix_address);
        e.register_double(0x0040_3df0, move |e, _| {
            let matrix = matrix_for_particle.get();
            for index in 0..9 {
                e.mem.set_u32(matrix + index * 4, 0xdead_beef);
            }
            e.set_global(0x011d_ea3c, 0x2004u32);
            0xc000u32.into_ret()
        });
        e.register(0x0068_9210, |_, a| {
            assert_eq!(a.len(), 18);
            assert_eq!(a[0..3], [0xa000, 1.0f32.to_bits(), 0xc000]);
            assert_eq!(a[3..12], [1, 2, 3, 4, 5, 6, 7, 8, 9]);
            assert_eq!(
                a[12..15],
                [10.0f32.to_bits(), 20.0f32.to_bits(), 100.0f32.to_bits()]
            );
            assert_eq!(a[15..18], [2.0f32.to_bits(), 7, 0]);
            Ret::default()
        });
        e.register(0x0045_3a70, |_, _| 0xb000u32.into_ret());
        e.register(0x00ad_7550, |_, _| 0xc000u32.into_ret());
        e.register(0x004b_29b0, |_, _| 0xd000u32.into_ret());
        e.register(0x0043_d4d0, |_, _| 0x5500u32.into_ret());
        e.register(0x0040_8d60, |_, _| 0x5600u32.into_ret());
        e.register(0x0076_b660, |e, a| {
            assert_eq!(a[0], 0x011c_7b8c);
            assert_eq!(e.mem.u32(a[1]), 0x1000);
            Ret::default()
        });

        explosion_initialize(&mut e, this);
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0x0045_0f90 && call.1 == [0x6000, 1]));
        assert!(log
            .iter()
            .any(|call| call.0 == 0x008d_1960 && call.1 == [6.0f32.to_bits(), 0x4000]));
        assert!(log.iter().any(|call| call.0 == 0x00ad_8b60));
        assert!(log.iter().any(|call| call.0 == 0x0045_8e20));
        assert!(log.iter().any(|call| call.0 == 0x0076_b660));
    }

    #[test]
    fn initialize_queries_cell_scene_even_when_cell_becomes_null() {
        let mut e = engine_with(&[
            0x0042_ce10,
            0x0043_9ef0,
            0x0045_7990,
            0x0064_4930,
            0x0064_7920,
            0x0040_3e20,
            0x0054_71e0,
            0x009a_d670,
            0x0041_81e0,
            0x004f_b070,
            0x0045_0f90,
            0x004b_6220,
            0x0043_d4d0,
            0x0040_8d60,
        ]);
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x2000);
        mapped(&mut e, 0x4000);
        mapped(&mut e, 0x5000);
        mapped(&mut e, 0x6000);
        mapped(&mut e, 0x8000);
        let mut explosion_slots = vec![0; 0x1f4 / 4 + 1];
        explosion_slots[0x1c4 / 4] = 0x6200;
        explosion_slots[0x1d0 / 4] = 0x6000;
        explosion_slots[0x1f4 / 4] = 0x6100;
        e.put_vtable(0x3000, &explosion_slots);
        e.mem.set_u32(this.addr(), 0x3000);
        e.mem.set_u32(0x2000, 0x3000);
        e.mem.set_u32(this.addr() + 0x68, 0x9300);
        e.mem.set_f32(0x4008, 0.0);
        e.mem.set_f32(0x5000, 1.0);
        e.mem.set_i32(0x8000, 0);
        e.mem.set_u8(0x8100, 0);
        e.set_global(0x011d_df38, 0x7000u32);
        e.set_global(0x011d_ea3c, 0x2000u32);
        e.register(0x0042_ce10, |_, _| true.into_ret());
        e.register(0x0000_6000, |_, _| 0x6000u32.into_ret());
        e.register(0x0000_6100, |_, _| 0x4000u32.into_ret());
        e.register(0x0000_6200, |_, _| Ret::default());
        let mut node_slots = vec![0; 0x10 / 4 + 1];
        node_slots[0xc / 4] = 0x6400;
        e.put_vtable(0xc000, &node_slots);
        e.mem.set_u32(0x6000, 0xc000);
        e.register(0x0000_6400, |_, _| 0u32.into_ret());
        e.register(0x0043_9ef0, |_, a| a[1].into_ret());
        e.register(0x0045_7990, |_, _| 3.0f32.into_ret());
        e.register(0x0064_4930, |_, _| 2.0f32.into_ret());
        e.register(0x0064_7920, |_, _| 0.0f32.into_ret());
        e.register(0x0040_3e20, |_, _| 0x5000u32.into_ret());
        let cells = Rc::new(Cell::new(0u32));
        let cells_for_double = Rc::clone(&cells);
        e.register_double(0x008d_6f30, move |_, _| {
            let count = cells_for_double.get();
            cells_for_double.set(count + 1);
            (if count < 3 { 0x7000u32 } else { 0u32 }).into_ret()
        });
        e.register(0x0054_71e0, |_, _| 100.0f32.into_ret());
        e.register(0x0041_81e0, |_, _| 0xb000u32.into_ret());
        e.register(0x004f_b070, |_, _| 0u32.into_ret());
        e.register(0x0054_97a0, |_, a| {
            assert_eq!(a, &[0]);
            0u32.into_ret()
        });
        e.register(0x0043_d4d0, |_, _| 0x8000u32.into_ret());
        e.register(0x0040_8d60, |_, _| 0x8100u32.into_ret());

        explosion_initialize(&mut e, this);
        let log = e.call_log.unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == 0x0054_97a0 && call.1 == [0]));
        assert!(!log.iter().any(|call| call.0 == 0x0054_8230));
    }

    #[test]
    fn qcinfo_size_is_fixed() {
        assert_eq!(fn_00471840(&mut Engine::new(), Ptr::new(0)), 0x18);
    }

    #[test]
    fn actor_cause_release_passes_adjusted_member_and_pointer() {
        let mut e = engine_with(&[0x0042_f780]);
        fn_009ac170(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));
        assert_eq!(e.call_log.unwrap()[0].1, vec![0x10d0, 0x2000]);
    }

    #[test]
    fn actor_cause_release_answers_the_helper() {
        let mut e = Engine::new();
        e.register(0x0042_f780, |_, a| a[0].into_ret());
        assert_eq!(
            e.call(0x009a_c170, &args![0x1000u32, 0x2000u32]).u32(),
            0x10d0
        );
    }

    #[test]
    fn caster_accessor_adjusts_non_null_pointer() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        e.mem.set_u32(this.addr() + 0xdc, 0x2000);
        assert_eq!(fn_009ac190(&mut e, this).addr(), 0x200c);
        e.mem.set_u32(this.addr() + 0xdc, 0);
        assert!(fn_009ac190(&mut e, this).is_null());
    }

    #[test]
    fn scalar_destructor_frees_only_when_flag_is_set() {
        for flags in [0u32, 1, 2] {
            let mut e = empty_explosion_destructor_engine();
            fn_009ac1d0(&mut e, Ptr::new(0x1000), flags);
            let calls = e.call_log.unwrap();
            assert_eq!(
                calls.iter().any(|call| call.0 == 0x0040_1030),
                flags & 1 != 0
            );
        }
    }

    #[test]
    fn image_space_radius_uses_stored_value_when_flag_is_set() {
        let mut e = Engine::new();
        e.register(0x0047_7950, |_, _| true.into_ret());
        let this = mapped(&mut e, 0x1000);
        e.mem.set_f32(this.addr() + 0x8c, 42.5);
        assert_eq!(fn_009ac550(&mut e, this), 42.5);
    }

    #[test]
    fn target_scalar_destructor_honors_flag() {
        let mut e = engine_with(&[0x009a_c940, 0x0040_1030]);
        fn_009ac910(&mut e, Ptr::new(0x1000), 1);
        assert_eq!(e.call_log.unwrap().len(), 2);
    }

    #[test]
    fn target_list_destruction_uses_adjusted_member() {
        let mut e = engine_with(&[0x0047_0470, 0x0046_ffb0]);
        fn_009ac940(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.unwrap()[0].1[0], 0x1008);
    }

    #[test]
    fn explosion_activation_uses_slot_c4_with_one() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        e.map(0x3000, 0x1000);
        e.mem.set_u32(this.addr(), 0x3000);
        e.mem.set_u32(0x30c4, 0x6000_00c4);
        e.register(0x6000_00c4, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        fn_009ac9a0(&mut e, this);
        assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![0x1000, 1]);
    }

    #[test]
    fn word_at_c8_is_returned() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        e.mem.set_u32(this.addr() + 0xc8, 0x2000);
        assert_eq!(fn_009acce0(&mut e, this).addr(), 0x2000);
    }

    #[test]
    fn attach_distant_stops_without_target() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        e.map(0x3000, 0x1000);
        e.mem.set_u32(this.addr(), 0x3000);
        e.mem.set_u32(0x31d0, 0x6000_01d0);
        e.register(0x6000_01d0, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        explosion_attach_distant(&mut e, this);
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn radiation_radius_uses_stored_value_when_flag_is_set() {
        let mut e = Engine::new();
        e.register(0x0047_7950, |_, _| true.into_ret());
        let this = mapped(&mut e, 0x1000);
        e.mem.set_f32(this.addr() + 0xa0, 20.0);
        assert_eq!(fn_009ad5a0(&mut e, this), 20.0);
    }

    #[test]
    fn null_cell_is_reported_as_true() {
        let mut e = Engine::new();
        e.register(0x008d_6f30, |_, _| 0u32.into_ret());
        assert!(fn_009ad5f0(&mut e, Ptr::new(0x1000)));
    }

    #[test]
    fn word_at_cc_is_returned() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        e.mem.set_u32(this.addr() + 0xcc, 1);
        assert_eq!(fn_009ad610(&mut e, this).addr(), 1);
    }

    #[test]
    fn point_record_constructor_copies_constants() {
        let mut e = engine_with(&[0x0053_7bb0]);
        let this = mapped(&mut e, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.map(0x0103_0000, 0x1000);
        e.set_global(0x0103_0020, 1.0f32);
        e.set_global(0x0101_e580, 2.0f32);
        e.set_global(0x0101_e6ec, 3.0f32);
        fn_009ae0f0(&mut e, this);
        assert_eq!(e.mem.f32(this.addr() + 0x10), 3.0);
        assert_eq!(e.mem.f32(this.addr() + 0x14), 2.0);
    }

    #[test]
    fn rtti_accessor_returns_constant_address() {
        assert_eq!(
            fn_009ae240(&mut Engine::new(), Ptr::new(0)).addr(),
            0x0126_8420
        );
    }

    #[test]
    fn collector_scalar_destructor_honors_flag() {
        let mut e = engine_with(&[0x00cb_c920, 0x00aa_1460]);
        fn_009ae250(&mut e, Ptr::new(0x1000), 1);
        assert_eq!(e.call_log.unwrap().len(), 2);
    }

    #[test]
    fn point_copy_writes_zero_w_component() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        let source = mapped(&mut e, 0x2000);
        e.mem.set_f32(source.addr(), 1.0);
        e.mem.set_f32(source.addr() + 4, -2.0);
        e.mem.set_f32(source.addr() + 8, 3.5);
        fn_009ae540(&mut e, this, source);
        assert_eq!(e.mem.f32(this.addr() + 8), 3.5);
        assert_eq!(e.mem.f32(this.addr() + 12), 0.0);
    }

    #[test]
    fn frame_step_calls_virtual_update_and_adds_delta() {
        let mut e = Engine::new();
        e.map(0x3000, 0x1000);
        let this = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(this.addr(), 0x3000);
        e.mem.set_u32(0x3000 + 0x2f8, 0x6000_02f8);
        e.register(0x0084_d030, |_, _| 2.5f32.into_ret());
        e.register(0x6000_02f8, |_, a| a[1].into_ret());
        e.call_log = Some(vec![]);
        fn_009ae580(&mut e, this, 9.0);
        assert_eq!(e.mem.f32(this.addr() + 0x88), 2.5);
        assert!(e
            .call_log
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0x6000_02f8));
    }

    #[test]
    fn collector_base_destructor_is_called() {
        let mut e = engine_with(&[0x0062_1100]);
        fn_009afc10(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn contact_collector_deleting_destructor_honors_flag() {
        let mut e = engine_with(&[0x009a_fc10, 0x009a_fbe0]);
        let this = Ptr::new(0x1000);
        assert_eq!(fn_009afbb0(&mut e, this, 0).addr(), this.addr());
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn buffer_accessor_adjusts_this_by_eight() {
        let mut e = engine_with(&[0x0045_8b50]);
        fn_009afc30(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.unwrap()[0].1[0], 0x1008);
    }

    #[test]
    fn proxy_count_adjusts_this_by_twenty() {
        let mut e = engine_with(&[0x0045_8b50]);
        fn_009afc50(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.unwrap()[0].1[0], 0x1014);
    }

    #[test]
    fn proxy_boolean_returns_input() {
        let mut e = engine_with(&[0x0045_8b50, 0x0062_2570]);
        let value = 0x1234;
        assert_eq!(fn_009afc70(&mut e, Ptr::new(0x1000), value), value);
        assert_eq!(e.call_log.unwrap().len(), 2);
    }

    #[test]
    fn contact_callback_uses_global_target_and_ordered_arguments() {
        let mut e = engine_with(&[0x7000]);
        e.map(0x0126_8000, 0x1000);
        e.set_global(0x0126_8430, 0x7000);
        fn_009afca0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));
        assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![0x2000, 0x1000]);
    }
    #[test]
    fn native_point_copy_vectors_preserve_abi_and_quiet_nan() {
        // Generated nv-call vectors, FalloutNV.exe 1.4.0.525, 2026-10-10.
        for (input, expected) in [
            (
                [0x3f80_0000, 0xc000_0000, 0x8000_0000],
                [0x3f80_0000, 0xc000_0000, 0x8000_0000],
            ),
            (
                [0x7f80_0001, 0xff80_0001, 0x7fc1_2345],
                [0x7fc0_0001, 0xffc0_0001, 0x7fc1_2345],
            ),
        ] {
            let mut e = Engine::new();
            let dst = mapped(&mut e, 0x1000);
            let src = mapped(&mut e, 0x2000);
            for (i, bits) in input.into_iter().enumerate() {
                e.mem.set_u32(src.addr() + i as u32 * 4, bits);
            }
            assert_eq!(e.call(0x009a_e540, &args![dst, src]).u32(), dst.addr());
            for (i, bits) in expected.into_iter().enumerate() {
                assert_eq!(e.mem.u32(dst.addr() + i as u32 * 4), bits);
            }
            assert_eq!(e.mem.u32(dst.addr() + 12), 0);
        }
    }

    #[test]
    fn collector_accessors_return_callee_word_through_registry() {
        for (address, offset) in [(0x009a_fc30, 8), (0x009a_fc50, 0x14)] {
            let mut e = engine_with(&[]);
            e.register(0x0045_8b50, |_, a| {
                assert!(a[0] == 0x1008 || a[0] == 0x1014);
                0xffff_fffeu32.into_ret()
            });
            assert_eq!(e.call(address, &args![0x1000u32]).u32(), 0xffff_fffe);
            assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![0x1000 + offset]);
        }
    }

    #[test]
    fn count_boolean_uses_signed_positive_test_and_output_address() {
        for count in [-1i32, 0, 1] {
            let mut e = engine_with(&[]);
            e.register_double(0x0045_8b50, move |_, a| {
                assert_eq!(a, &[0x1014]);
                count.into_ret()
            });
            e.register(0x0062_2570, |e, a| {
                e.mem.set_u8(a[0], a[1] as u8);
                a[0].into_ret()
            });
            mapped(&mut e, 0x2000);
            assert_eq!(
                e.call(0x009a_fc70, &args![0x1000u32, 0x2000u32]).u32(),
                0x2000
            );
            assert_eq!(e.mem.u8(0x2000), u8::from(count > 0));
        }
    }

    #[test]
    fn both_radius_accessors_scale_only_without_unit_flag() {
        for (address, offset) in [(0x009a_c550, 0x8c), (0x009a_d5a0, 0xa0)] {
            let mut e = engine_with(&[]);
            mapped(&mut e, 0x1000);
            mapped(&mut e, 0x2000);
            e.mem.set_f32(0x1000 + offset, 3.5);
            e.mem.set_f32(0x2000, 2.25);
            e.register(0x0047_7950, |_, a| {
                assert_eq!(a, &[0x1000, 1]);
                false.into_ret()
            });
            e.register(0x0040_3e20, |_, a| {
                assert_eq!(a, &[0x011d_1218]);
                0x2000u32.into_ret()
            });
            assert_eq!(e.call(address, &args![0x1000u32]).f32(), 7.875);
        }
    }

    #[test]
    fn distant_target_attaches_only_when_terrain_exists() {
        for terrain in [0u32, 0x4000] {
            let mut e = engine_with(&[0x6000_00dc]);
            for address in [0x1000, 0x3000, 0x4000, 0x5000] {
                mapped(&mut e, address);
            }
            e.mem.set_u32(0x1000, 0x3000);
            e.mem.set_u32(0x31d0, 0x6000_01d0);
            e.mem.set_u32(0x4000, 0x5000);
            e.mem.set_u32(0x50dc, 0x6000_00dc);
            e.register(0x6000_01d0, |_, _| 0x9000u32.into_ret());
            e.register(0x004f_d3e0, |_, a| {
                assert_eq!(a, &[0x011d_ea10]);
                0x7000u32.into_ret()
            });
            e.register(0x0058_6170, |_, a| {
                assert_eq!(a, &[0x7000]);
                0x8000u32.into_ret()
            });
            e.register_double(0x0055_85e0, move |_, a| {
                assert_eq!(a, &[0x8000]);
                terrain.into_ret()
            });
            explosion_attach_distant(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            assert_eq!(log.len(), if terrain == 0 { 4 } else { 5 });
            if terrain != 0 {
                assert_eq!(log.last().unwrap().1, vec![terrain, 0x9000, 1]);
            }
        }
    }

    #[test]
    fn deleting_wrappers_preserve_this_and_gate_free_on_low_bit() {
        for (address, destructor, delete, size) in [
            (0x009a_c910, 0x009a_c940, 0x0040_1030, None),
            (0x009a_e250, 0x00cb_c920, 0x00aa_1460, Some(0x40)),
            (0x009a_fbb0, 0x009a_fc10, 0x009a_fbe0, None),
        ] {
            for flags in [0u32, 1, 2] {
                let mut e = engine_with(&[destructor, delete]);
                assert_eq!(e.call(address, &args![0x1000u32, flags]).u32(), 0x1000);
                let log = e.call_log.unwrap();
                assert_eq!(log[1], (destructor, vec![0x1000]));
                assert_eq!(log.len(), if flags & 1 == 0 { 2 } else { 3 });
                if flags & 1 != 0 {
                    assert_eq!(
                        log[2],
                        (delete, size.map_or(vec![0x1000], |size| vec![0x1000, size]))
                    );
                }
            }
        }
    }
    fn constructor_engine() -> Engine {
        let mut e = engine_with(&[
            0x0092_eb50,
            0x0063_3c90,
            0x0096_a2d0,
            0x0041_a250,
            0x0042_f730,
            0x0068_15c0,
            0x0066_b0d0,
            0x0042_f780,
            0x004b_50c0,
            0x00cb_d520,
        ]);
        mapped(&mut e, 0x1000);
        mapped(&mut e, 0x011f_4000);
        mapped(&mut e, 0x0101_2000);
        mapped(&mut e, 0x0126_8000);
        for (offset, value) in [(0, 1u32), (4, 2), (8, 3)] {
            e.set_global(0x011f_426c + offset, value);
        }
        e.set_global(0x0101_2054, -1.0f32);
        e
    }

    #[test]
    fn explosion_default_constructor_initializes_pc_fields_and_order() {
        let mut e = constructor_engine();
        for offset in 0..0x104 {
            e.mem.set_u8(0x1000 + offset, 0xaa);
        }
        assert_eq!(e.call(0x009a_bf30, &args![0x1000u32]).u32(), 0x1000);
        assert_eq!(e.mem.u32(0x1000), 0x0108_ee04);
        assert_eq!(e.mem.u32(0x1018), 0x0108_edf8);
        for offset in [
            0x68, 0xe4, 0x88, 0x8c, 0x90, 0x94, 0x98, 0xc8, 0xcc, 0xe0, 0xdc,
        ] {
            assert_eq!(e.mem.u32(0x1000 + offset), 0);
        }
        for offset in 0xd4..=0xd9 {
            assert_eq!(e.mem.u8(0x1000 + offset), 0);
        }
        assert_eq!(e.mem.u8(0x10da), 0xaa);
        assert_eq!(e.mem.f32(0x109c), 1.0);
        assert_eq!(e.mem.f32(0x1100), -1.0);
        for offset in [0xe8, 0xf4] {
            for i in 0..3 {
                assert_eq!(e.mem.u32(0x1000 + offset + i * 4), i + 1);
            }
        }
        let log = e.call_log.unwrap();
        assert_eq!(log[1], (0x0092_eb50, vec![0x1000]));
        assert_eq!(log[2], (0x0063_3c90, vec![0x10a0, 0]));
        assert_eq!(log.last().unwrap(), &(0x0042_f780, vec![0x10d0, 0]));
    }

    #[test]
    fn modifier_lookup_caches_success_and_retries_null() {
        for cast_result in [0u32, 0x9000] {
            let mut e = engine_with(&[]);
            mapped(&mut e, 0x011c_9000);
            e.register(0x0048_39c0, |_, a| {
                assert_eq!(a, &[0x166]);
                0x8000u32.into_ret()
            });
            e.register_double(0x00ec_43fb, move |_, a| {
                assert_eq!(a, &[0x8000, 0, 0x0118_3028, 0x0118_62b8, 0]);
                cast_result.into_ret()
            });
            for _ in 0..2 {
                assert_eq!(
                    tes_image_space_modifier_get_explosion_in_face(&mut e).addr(),
                    cast_result
                );
            }
            assert_eq!(
                e.call_log.unwrap().len(),
                if cast_result == 0 { 4 } else { 2 }
            );
        }
    }

    #[test]
    fn liquid_constructor_sets_defaults_then_initializes_and_wraps_counter() {
        let mut e = constructor_engine();
        e.set_global(0x0126_841c, u32::MAX);
        assert_eq!(
            bhk_liquid_action_ctor(&mut e, Ptr::new(0x1000), Ptr::new(0x8000)).addr(),
            0x1000
        );
        assert_eq!(e.mem.u32(0x1000), 0x0108_f1bc);
        assert_eq!(e.mem.u8(0x103c), 1);
        assert_eq!(e.global::<u32>(0x0126_841c), 0);
        for offset in [0x24, 0x30] {
            for i in 0..3 {
                assert_eq!(e.mem.u32(0x1000 + offset + i * 4), i + 1);
            }
        }
        assert_eq!(
            e.call_log.unwrap().last().unwrap(),
            &(0x00cb_d520, vec![0x1000, 0x8000])
        );
    }

    fn matrix_engine() -> Engine {
        let mut e = engine_with(&[0x0068_15c0, 0x004b_4e50]);
        mapped(&mut e, 0x1000);
        e.register(0x0043_9f50, |e, a| {
            assert_eq!(a[0], 0x2000);
            for i in 0..3 {
                e.mem.set_f32(a[2] + i * 4, (a[1] * 3 + i) as f32);
            }
            Ret::default()
        });
        e
    }

    #[test]
    fn matrix_rows_convert_to_padded_vectors() {
        let mut e = matrix_engine();
        assert_eq!(
            fn_009ae4d0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000)).addr(),
            0x1000
        );
        for row in 0..3 {
            for column in 0..3 {
                assert_eq!(
                    e.mem.f32(0x1000 + row * 16 + column * 4),
                    (row * 3 + column) as f32
                );
            }
            assert_eq!(e.mem.u32(0x100c + row * 16), 0);
        }
    }

    #[test]
    fn transform_copy_forwards_translation_after_rows() {
        let mut e = matrix_engine();
        assert_eq!(
            fn_009ae4a0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000)).addr(),
            0x1000
        );
        assert_eq!(
            e.call_log.unwrap().last().unwrap(),
            &(0x004b_4e50, vec![0x1030, 0x2024])
        );
    }

    #[test]
    fn fixed_buffer_constructor_initializes_each_member() {
        let mut e = engine_with(&[0x0062_1060, 0x0068_15c0]);
        mapped(&mut e, 0x1000);
        e.register(0x0053_7e90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        assert_eq!(
            fn_009afaf0(&mut e, Ptr::new(0x1000), 0x8000, 12).addr(),
            0x1000
        );
        assert_eq!(e.mem.u32(0x1000), 0x0108_f288);
        for (offset, value) in [(8, 0x8000), (12, 0x8000), (16, 12), (20, 0)] {
            assert_eq!(e.mem.u32(0x1000 + offset), value);
        }
    }

    #[test]
    fn fixed_buffer_delete_handles_null_and_allocator_size() {
        let mut e = engine_with(&[0x6000_0008]);
        mapped(&mut e, 0x1000);
        mapped(&mut e, 0x2000);
        e.mem.set_u32(0x1000, 0x2000);
        e.mem.set_u32(0x2008, 0x6000_0008);
        e.register(0x00c8_5750, |_, a| {
            assert!(a.is_empty());
            0x7000u32.into_ret()
        });
        e.register(0x0044_edb0, |_, a| {
            assert_eq!(a, &[0x7000]);
            0x1000u32.into_ret()
        });
        fn_009afbe0(&mut e, Ptr::NULL);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        fn_009afbe0(&mut e, Ptr::new(0x8000));
        assert_eq!(
            e.call_log.unwrap().last().unwrap().1,
            vec![0x1000, 0x8000, 0x18]
        );
    }

    #[test]
    fn embedded_array_uses_resolved_havok_object() {
        let mut e = engine_with(&[0x0062_d2f0]);
        e.register(0x004a_e750, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x2000u32.into_ret()
        });
        e.register(0x0062_d2f0, |_, a| (a[0] + 0x160).into_ret());
        assert_eq!(e.call(0x009a_fed0, &args![0x1000u32]).u32(), 0x2160);
        assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![0x2000]);
    }

    #[test]
    fn phantom_initialization_keeps_distinct_stack_temporaries_and_order() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x5000);
        let mut reference_vtable = vec![0; 0x1d0 / 4 + 1];
        reference_vtable[0x1d0 / 4] = 0xa100;
        e.put_vtable(0x2000, &reference_vtable);
        e.mem.set_u32(this.addr(), 0x2000);
        let mut phantom_vtable = vec![0; 0x9c / 4 + 1];
        phantom_vtable[0x9c / 4] = 0xa200;
        e.put_vtable(0x2100, &phantom_vtable);

        let cinfo_address = Rc::new(Cell::new(0));
        let cinfo_for_ctor = Rc::clone(&cinfo_address);
        let cinfo_for_string = Rc::clone(&cinfo_address);
        e.register_double(0x0056_e090, move |e, a| {
            let cinfo = a[0];
            cinfo_for_ctor.set(cinfo);
            e.mem.set_u32(cinfo, 0);
            e.mem.set_u32(cinfo + 4, 0x44);
            for offset in (0x20..0x60).step_by(4) {
                e.mem.set_u32(cinfo + offset, 0xdead_beef);
            }
            cinfo.into_ret()
        });
        e.register_double(0x0062_2290, move |e, a| {
            assert_eq!(a, &[cinfo_for_string.get() - 0x14, 0x29, 0x8000, 0]);
            // PC places the CStringT object immediately below, but outside, cinfo.
            e.mem.set_u32(a[0], 0x9000);
            a[0].into_ret()
        });
        let cinfo_for_init = Rc::clone(&cinfo_address);
        e.register_double(0x0056_e2d0, move |e, a| {
            let cinfo = cinfo_for_init.get();
            assert_eq!(a[1], cinfo);
            assert!(a[0] == 0x5000 || a[0] == 0x5001);
            assert_eq!(e.mem.u32(cinfo), 0x9000);
            assert_eq!(
                e.mem.u32(cinfo + 4),
                if a[0] == 0x5000 { 0xa000 } else { 0 }
            );
            assert_eq!(e.mem.f32(cinfo + 0x20), 1.0);
            assert_eq!(e.mem.f32(cinfo + 0x34), 1.0);
            assert_eq!(e.mem.f32(cinfo + 0x48), 1.0);
            assert_eq!(e.mem.f32(cinfo + 0x5c), 0.0);
            e.mem.set_u32(a[0], 0x2100);
            a[0].into_ret()
        });
        e.register(0x0082_2510, |_, _| true.into_ret());
        e.register(0x008d_6f30, |_, _| 0x6000u32.into_ret());
        e.register(0x0045_43c0, |_, _| 0x7000u32.into_ret());
        mapped(&mut e, 0x5001);
        let mut allocations = [0x4000u32, 0x5000, 0, 0x5001].into_iter();
        e.register_double(0x00aa_13e0, move |_, a| {
            assert!(a[0] == 0x14 || a[0] == 0x18);
            allocations.next().unwrap().into_ret()
        });
        e.register(0x0064_4930, |_, a| {
            assert_eq!(a, &[0x1000]);
            2.5f32.into_ret()
        });
        e.register(0x0056_e950, |_, a| {
            assert_eq!(a, &[0x4000, 2.5f32.to_bits(), 1]);
            0xb000u32.into_ret()
        });
        let mut operator_calls = 0;
        e.register_double(0x004a_e750, move |_, a| {
            if operator_calls == 0 {
                assert_eq!(a, &[0xb000]);
                operator_calls += 1;
                0xa000u32.into_ret()
            } else {
                assert_eq!(a, &[0]);
                operator_calls += 1;
                0u32.into_ret()
            }
        });
        e.register(0x005d_be10, |_, a| {
            assert!(a.is_empty());
            0x8000u32.into_ret()
        });
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0000_a100, |_, _| 0u32.into_ret());
        e.register(0x004b_4fc0, |e, a| {
            for offset in (0..0x10).step_by(4) {
                e.mem.set_u32(a[0] + offset, 0);
            }
            Ret::default()
        });
        e.register(0x0066_b0d0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x0000_a200, |_, a| {
            assert!(a[0] == 0x5000 || a[0] == 0x5001);
            assert_eq!(a[1], 0x7000);
            Ret::default()
        });
        let cinfo_for_dtor = Rc::clone(&cinfo_address);
        e.register_double(0x0056_ea90, move |_, a| {
            assert_eq!(a[0], cinfo_for_dtor.get());
            Ret::default()
        });

        fn_009ae280(&mut e, this);
        assert_eq!(e.mem.u32(this.addr() + 0xa0), 0x5000);
        let log = e.call_log.as_ref().unwrap().clone();
        assert_eq!(
            log.iter()
                .filter(|(address, _)| *address == 0x00aa_13e0)
                .map(|(_, words)| words[0])
                .collect::<Vec<_>>(),
            vec![0x14, 0x18]
        );
        assert_eq!(
            log.iter()
                .filter(|(address, _)| *address == 0x0000_a100)
                .count(),
            1
        );
        let text_constructor = log
            .iter()
            .position(|(address, _)| *address == 0x0062_2290)
            .unwrap();
        let matrix_zeroes: Vec<_> = log
            .iter()
            .enumerate()
            .filter(|(_, (address, _))| *address == 0x004b_4fc0)
            .map(|(index, _)| index)
            .collect();
        let phantom_init = log
            .iter()
            .position(|(address, _)| *address == 0x0056_e2d0)
            .unwrap();
        assert_eq!(matrix_zeroes.len(), 4);
        assert!(text_constructor < matrix_zeroes[0]);
        assert!(matrix_zeroes[3] < phantom_init);

        // With a null sphere allocation, native still calls operatorP with
        // the null constructor result before writing the cinfo shape field.
        fn_009ae280(&mut e, this);
        assert_eq!(e.mem.u32(this.addr() + 0xa0), 0x5001);
        let complete_log = e.call_log.as_ref().unwrap();
        assert_eq!(
            complete_log
                .iter()
                .filter(|(address, _)| *address == 0x0056_e950)
                .count(),
            1
        );
        assert_eq!(
            complete_log
                .iter()
                .filter(|(address, _)| *address == 0x004a_e750)
                .map(|(_, args)| args[0])
                .collect::<Vec<_>>(),
            vec![0xb000, 0]
        );
    }

    #[test]
    fn phantom_initialization_returns_when_havok_is_disabled() {
        let mut e = engine_with(&[0x0082_2510]);
        fn_009ae280(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.unwrap(), vec![(0x0082_2510, vec![0x10a0, 0])]);
    }

    #[test]
    fn underwater_test_uses_cell_water_height_and_preserves_the_existing_bit() {
        let mut e = Engine::new();
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x3000);
        mapped(&mut e, 0x5000);
        e.mem.set_f32(0x3000, 0.5);
        e.mem.set_f32(0x5008, 3.0);
        let mut slots = vec![0; 0x1f4 / 4 + 1];
        slots[0x1f4 / 4] = 0xa100;
        e.put_vtable(0x2100, &slots);
        e.mem.set_u32(this.addr(), 0x2100);
        e.register(0x0040_3e20, |_, _| 0x3000u32.into_ret());
        e.register(0x008d_6f30, |_, _| 0x4000u32.into_ret());
        e.register(0x0000_a100, |_, _| 0x5000u32.into_ret());
        e.register(0x0064_4930, |_, _| 4.0f32.into_ret());
        e.register(0x0054_71e0, |_, _| 6.0f32.into_ret());

        e.mem.set_u8(this.addr() + 0xd9, 0x80);
        assert!(fn_009b2940(&mut e, this));
        assert_eq!(e.mem.u8(this.addr() + 0xd9), 0x81);

        e.register(0x008d_6f30, |_, _| 0u32.into_ret());
        e.mem.set_u8(this.addr() + 0xd9, 0x40);
        assert!(!fn_009b2940(&mut e, this));
        assert_eq!(e.mem.u8(this.addr() + 0xd9), 0x40);
    }

    #[test]
    fn underwater_test_skips_low_process_lookup_for_null_world() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x3000);
        mapped(&mut e, 0x5000);
        let mut slots = vec![0; 0x1f4 / 4 + 1];
        slots[0x1f4 / 4] = 0xa100;
        e.put_vtable(0x2100, &slots);
        e.mem.set_u32(this.addr(), 0x2100);
        e.mem.set_f32(0x3000, 1.0);
        e.mem.set_f32(0x5008, 3.0);
        e.register(0x0040_3e20, |_, _| 0x3000u32.into_ret());
        e.register(0x008d_6f30, |_, _| 0x4000u32.into_ret());
        e.register(0x009a_d5f0, |_, _| false.into_ret());
        e.register(0x0000_a100, |_, _| 0x5000u32.into_ret());
        e.register(0x0064_4930, |_, _| 1.0f32.into_ret());
        e.register(0x0054_71e0, |_, _| 0.0f32.into_ret());
        e.register(0x0045_43c0, |_, _| 0u32.into_ret());
        e.register(0x0054_1c80, |_, _| {
            panic!("null world must bypass low-process lookup")
        });

        assert!(!fn_009b2940(&mut e, this));
        let log = e.call_log.unwrap();
        assert!(!log.iter().any(|(address, _)| *address == 0x0054_1c80));
    }

    #[test]
    fn underwater_contact_path_uses_native_stack_slots_and_signed_count() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = mapped(&mut e, 0x1000);
        mapped(&mut e, 0x3000);
        mapped(&mut e, 0x4000);
        mapped(&mut e, 0x5000);
        mapped(&mut e, 0x6000);
        mapped(&mut e, 0x7000);
        mapped(&mut e, 0x8000);
        mapped(&mut e, 0x8100);
        mapped(&mut e, 0x9000);
        mapped(&mut e, 0x9100);
        mapped(&mut e, 0x9200);
        mapped(&mut e, 0x9300);
        mapped(&mut e, 0xa000);
        mapped(&mut e, 0xb000);
        mapped(&mut e, 0xc000);
        mapped(&mut e, 0xd000);
        mapped(&mut e, 0xe000);
        mapped(&mut e, 0xf000);

        let mut explosion_vtable = vec![0; 0x1f4 / 4 + 1];
        explosion_vtable[0x1f4 / 4] = 0xa100;
        e.put_vtable(0x2000, &explosion_vtable);
        e.mem.set_u32(this.addr(), 0x2000);
        let mut positions = [
            0x9000u32, 0x9100, 0x9200, // mismatched form type
            0x9000, 0x9100, 0x9200, // negative contact count
            0x9000, 0x9100, 0x9200, 0x9000, 0x9300, // matched form type
        ]
        .into_iter();
        e.register_double(0x0000_a100, move |_, _| {
            positions.next().unwrap().into_ret()
        });
        e.mem.set_f32(0x9008, 0.0);
        e.mem.set_f32(0x9108, 0.0);
        e.mem.set_f32(0x9208, 0.0);
        e.mem.set_f32(0x9308, 5.0);
        e.mem.set_f32(0x3000, 1.0);
        e.mem.set_f32(0x4008, 0.0);
        e.register(0x0040_3e20, |_, _| 0x3000u32.into_ret());
        e.register(0x008d_6f30, |_, _| 0x4000u32.into_ret());
        e.register(0x009a_d5f0, |_, _| false.into_ret());
        e.register(0x0064_4930, |_, _| 2.0f32.into_ret());
        e.register(0x0054_71e0, |_, _| 0.0f32.into_ret());
        e.register(0x0045_43c0, |_, _| 0x5000u32.into_ret());
        e.register(0x0054_1c80, |_, _| 0x6000u32.into_ret());
        e.register(0x0062_86d0, |_, _| 0x7000u32.into_ret());
        let mut contact_vtable = vec![0; 0x44 / 4 + 1];
        contact_vtable[0x44 / 4] = 0xa200;
        e.put_vtable(0x8100, &contact_vtable);
        e.mem.set_u32(0x7000, 0x8100);
        e.put_vtable(0x8200, &explosion_vtable);
        e.mem.set_u32(0xe000, 0x8200);
        let collector_address = Rc::new(Cell::new(0));
        let collector_address_for_contact = Rc::clone(&collector_address);
        let collector_address_for_entry = Rc::clone(&collector_address);
        let array_for_init = Rc::clone(&collector_address);
        e.register_double(0x0062_e230, move |e, a| {
            assert_eq!(a[1], a[0] + 12);
            assert_eq!(a[2], 0);
            assert_eq!(a[3], 0x80);
            array_for_init.set(a[0]);
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 8, 0x80);
            e.mem.set_u32(a[0] + 12, a[1]);
            Ret::default()
        });
        let first_vec_from_call = Rc::new(Cell::new(0));
        let vec_for_contact = Rc::clone(&first_vec_from_call);
        e.register_double(0x0000_a200, move |_, a| {
            assert_eq!(a[0], 0x7000);
            assert_eq!(a[2], collector_address_for_contact.get());
            vec_for_contact.set(a[1]);
            Ret::default()
        });
        e.register_double(0x004a_3760, |_, a| {
            assert_eq!(a[1], 1.0f32.to_bits());
            assert_eq!(a[2], 0x011a_9490);
            a[0].into_ret()
        });
        e.register(0x0043_9ef0, |_, a| {
            assert_eq!(a[0], 0x9100);
            Ret::default()
        });
        e.register(0x0043_9e90, |_, a| {
            assert_eq!(a[0], 0x9200);
            Ret::default()
        });
        e.register(0x0043_7c10, |_, _| Ret::default());
        e.register(0x004a_3e00, |_, _| Ret::default());
        e.register(0x0072_6070, |_, _| 1i32.into_ret());
        e.register_double(0x0062_6660, move |_, a| {
            assert_eq!(a, &[collector_address_for_entry.get(), 0]);
            0xb000u32.into_ret()
        });
        e.mem.set_u32(0xb004, 0xc000);
        e.mem.set_u8(0xc005, 2);
        e.register(0x004b_5820, |_, a| {
            assert_eq!(a, &[0xc002]);
            0xd000u32.into_ret()
        });
        e.register(0x0056_f930, |_, a| {
            assert_eq!(a, &[0xd000]);
            0xe000u32.into_ret()
        });
        e.register(0x007a_f430, |_, a| {
            assert_eq!(a, &[0xe000]);
            0xf000u32.into_ret()
        });
        e.register(0x0040_1170, |_, a| {
            assert_eq!(a, &[0xf000]);
            // The native compares the full return value, not its low byte.
            0x123u32.into_ret()
        });
        e.register(0x0057_c530, |_, _| Ret::default());

        assert!(!fn_009b2940(&mut e, this));
        let first_calls = e.call_log.as_ref().unwrap().clone();
        let collector = collector_address.get();
        assert_ne!(collector, 0);
        let contact_call = first_calls
            .iter()
            .find(|(address, _)| *address == 0x0000_a200)
            .unwrap();
        assert_eq!(contact_call.1[1], first_vec_from_call.get());
        assert_eq!(first_vec_from_call.get(), collector + 0x410);
        assert_eq!(e.mem.u8(this.addr() + 0xd9), 0);

        // A negative signed count exits before querying element zero.
        e.call_log = Some(vec![]);
        e.register(0x0072_6070, |_, _| (-1i32).into_ret());
        assert!(!fn_009b2940(&mut e, this));
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0x0062_6660));

        // A water form compares its own height with the explosion's top.
        e.call_log = Some(vec![]);
        e.register(0x0072_6070, |_, _| 1i32.into_ret());
        e.register(0x0040_1170, |_, _| 0x23u32.into_ret());
        assert!(fn_009b2940(&mut e, this));
        assert_eq!(e.mem.u8(this.addr() + 0xd9), 1);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(address, args)| *address == 0x0000_a100 && args.as_slice() == [0xe000]));
    }

    #[test]
    fn weapon_source_accessor_reads_pc_offset() {
        let mut e = Engine::new();
        mapped(&mut e, 0x1000);
        e.mem.set_u32(0x10e0, 0x9876);
        assert_eq!(e.call(0x009b_0900, &args![0x1000u32]).u32(), 0x9876);
    }

    #[test]
    fn float_accessor_preserves_negative_zero() {
        let mut e = Engine::new();
        mapped(&mut e, 0x1000);
        e.mem.set_u32(0x1074, 0x8000_0000);
        assert_eq!(
            e.call(0x009b_0bd0, &args![0x1000u32]).f32().to_bits(),
            0x8000_0000
        );
    }

    fn impulse_engine(body: u32) -> Engine {
        let mut e = engine_with(&[
            0x0068_15c0,
            0x00a2_9680,
            0x0081_82d0,
            0x00c9_c1d0,
            0x6000_0054,
        ]);
        mapped(&mut e, 0x3000);
        mapped(&mut e, 0x4000);
        e.mem.set_u32(0x3000, 0x4000);
        e.mem.set_u32(0x4054, 0x6000_0054);
        e.register_double(0x004a_e750, move |_, a| {
            assert_eq!(a, &[0x1000]);
            body.into_ret()
        });
        e.register(0x009d_9f40, |_, a| {
            assert_eq!(a, &[0x2000]);
            0x3000u32.into_ret()
        });
        e.register(0x004a_3e00, |e, a| {
            assert_eq!(a[1], 0x9000);
            e.mem.set_f32(a[0], 5.0);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn linear_impulse_branches_on_body_and_brackets_call() {
        for body in [0, 0x2000] {
            let mut e = impulse_engine(body);
            fn_009b0c50(&mut e, Ptr::new(0x1000), Ptr::new(0x9000));
            let log = e.call_log.unwrap();
            assert_eq!(log.len(), if body == 0 { 1 } else { 4 });
            if body != 0 {
                assert_eq!(log[2], (0x0081_82d0, vec![body, 0x9000]));
                assert_eq!(log[1], log[3]);
            }
        }
    }

    #[test]
    fn converted_linear_impulse_passes_stack_result() {
        let mut e = impulse_engine(0x2000);
        e.register(0x0081_82d0, |e, a| {
            assert_eq!(a[0], 0x2000);
            assert_eq!(e.mem.f32(a[1]), 5.0);
            Ret::default()
        });
        fn_009b0bf0(&mut e, Ptr::new(0x1000), Ptr::new(0x9000));
        assert_eq!(e.call_log.unwrap().len(), 6);
    }

    #[test]
    fn motion_impulse_branches_on_body_and_brackets_call() {
        for body in [0, 0x2000] {
            let mut e = impulse_engine(body);
            fn_009b0cf0(&mut e, Ptr::new(0x1000), Ptr::new(0x9000));
            let log = e.call_log.unwrap();
            assert_eq!(log.len(), if body == 0 { 1 } else { 6 });
            if body != 0 {
                assert_eq!(log[4], (0x6000_0054, vec![0x3000, 0x9000]));
                assert_eq!(log[1], log[5]);
            }
        }
    }

    #[test]
    fn converted_motion_impulse_passes_stack_result() {
        let mut e = impulse_engine(0x2000);
        e.register(0x6000_0054, |e, a| {
            assert_eq!(a[0], 0x3000);
            assert_eq!(e.mem.f32(a[1]), 5.0);
            Ret::default()
        });
        fn_009b0c90(&mut e, Ptr::new(0x1000), Ptr::new(0x9000));
        assert_eq!(e.call_log.unwrap().len(), 8);
    }

    #[test]
    fn motion_forwarding_activates_before_lookup() {
        let mut e = impulse_engine(0x2000);
        fn_009b0d30(&mut e, Ptr::new(0x2000), Ptr::new(0x9000));
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (0x00c9_c1d0, vec![0x2000]),
                (0x009d_9f40, vec![0x2000]),
                (0x6000_0054, vec![0x3000, 0x9000])
            ]
        );
    }

    #[test]
    fn target_constructor_leaves_padding_and_initializes_list() {
        let mut e = engine_with(&[0x0096_a2d0]);
        mapped(&mut e, 0x1000);
        e.mem.set_u32(0x1004, u32::MAX);
        assert_eq!(fn_009b0f40(&mut e, Ptr::new(0x1000)).addr(), 0x1000);
        assert_eq!(e.mem.u32(0x1000), 0);
        assert_eq!(e.mem.u32(0x1004), 0xff00_0000);
        assert_eq!(e.call_log.unwrap(), vec![(0x0096_a2d0, vec![0x1008])]);
    }

    #[test]
    fn sounds_recheck_validity_after_stop() {
        for validity in [
            vec![false, false],
            vec![true, false, false],
            vec![true, true, true, true],
        ] {
            let mut e = engine_with(&[0x00ad_88f0, 0x00ad_8d10]);
            let expected_releases = if validity.len() == 4 { 2 } else { 0 };
            let mut values = validity.into_iter();
            e.register_double(0x00ad_8ce0, move |_, _| values.next().unwrap().into_ret());
            explosion_stop_sounds(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            assert_eq!(
                log.iter()
                    .filter(|(address, _)| *address == 0x00ad_8d10)
                    .count(),
                expected_releases
            );
            assert!(log.iter().all(|(_, a)| a[0] == 0x10ac || a[0] == 0x10b8));
        }
    }
    #[test]
    fn target_process_impulse_obeys_null_threshold_and_by_value_position() {
        for (target, process, magnitude, forwarded) in [
            (0, 0x3000, 3.0, false),
            (0x2000, 0, 3.0, false),
            (0x2000, 0x3000, 2.0, false),
            (0x2000, 0x3000, f32::NAN, false),
            (0x2000, 0x3000, 3.0, true),
        ] {
            let mut e = engine_with(&[0x6000_0418]);
            for address in [0x1000, 0x3000, 0x4000, 0x5000, 0x6000, 0x7000] {
                mapped(&mut e, address);
            }
            e.mem.set_u32(0x1000, 0x4000);
            e.mem.set_u32(0x41f4, 0x6000_01f4);
            e.mem.set_u32(0x3000, 0x5000);
            e.mem.set_u32(0x5418, 0x6000_0418);
            e.mem.set_f32(0x6000, 2.0);
            for i in 0..3 {
                e.mem.set_u32(0x7000 + i * 4, i + 7);
            }
            e.register_double(0x008d_8520, move |_, a| {
                assert_eq!(a, &[target]);
                process.into_ret()
            });
            e.register(0x0040_3e20, |_, a| {
                assert_eq!(a, &[0x011d_1188]);
                0x6000u32.into_ret()
            });
            e.register(0x6000_01f4, |_, a| {
                assert_eq!(a, &[0x1000]);
                0x7000u32.into_ret()
            });
            fn_009b0d70(&mut e, Ptr::new(0x1000), Ptr::new(target), magnitude);
            let log = e.call_log.unwrap();
            assert_eq!(
                log.iter().any(|(address, _)| *address == 0x6000_0418),
                forwarded
            );
            if forwarded {
                assert_eq!(log.last().unwrap().1, vec![0x3000, 0x2000, 7, 8, 9, 0]);
            }
        }
    }

    #[test]
    fn sound_start_handles_each_optional_record_and_temporary_cleanup() {
        for present in [false, true] {
            let mut e = engine_with(&[0x0041_8900, 0x0048_3710]);
            e.register(0x00ad_8ce0, |_, _| false.into_ret());
            e.register(0x0041_81e0, |_, a| {
                assert_eq!(a, &[0x1000]);
                0x2000u32.into_ret()
            });
            for getter in [0x004f_d3c0, 0x0091_85e0] {
                e.register_double(getter, move |_, a| {
                    assert_eq!(a, &[0x2000]);
                    (if present { 0x3000u32 } else { 0 }).into_ret()
                });
            }
            e.register(0x0084_e3a0, |_, a| {
                assert_eq!(a, &[0x3000]);
                77u32.into_ret()
            });
            e.register(0x0093_3150, |_, a| {
                assert_eq!(a[0], 0x1000);
                assert_eq!(&a[2..], &[77, 0, 0x4000_0182, 1]);
                a[1].into_ret()
            });
            explosion_start_sounds(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            let assignments: Vec<_> = log
                .iter()
                .filter(|(address, _)| *address == 0x0041_8900)
                .collect();
            assert_eq!(assignments.len(), if present { 2 } else { 0 });
            if present {
                assert_eq!(assignments[0].1[0], 0x10ac);
                assert_eq!(assignments[1].1[0], 0x10b8);
            }
            assert_eq!(
                log.iter()
                    .filter(|(address, _)| *address == 0x0048_3710)
                    .count(),
                assignments.len()
            );
        }
    }

    #[test]
    fn light_attach_checks_node_and_form_before_assignment() {
        for (node, form) in [(0u32, 0x3000u32), (0x2000, 0), (0x2000, 0x3000)] {
            let mut e = engine_with(&[0x0066_b0d0]);
            mapped(&mut e, 0x1000);
            mapped(&mut e, 0x4000);
            e.mem.set_u32(0x1000, 0x4000);
            e.mem.set_u32(0x41d0, 0x6000_01d0);
            e.register_double(0x6000_01d0, move |_, a| {
                assert_eq!(a, &[0x1000]);
                node.into_ret()
            });
            e.register(0x0041_81e0, |_, _| 0x5000u32.into_ret());
            e.register_double(0x004f_b070, move |_, a| {
                assert_eq!(a, &[0x5000]);
                form.into_ret()
            });
            e.register(0x0050_d810, |_, a| {
                assert_eq!(a, &[0x3000, 0x1000, 0x2000, 0]);
                0x7000u32.into_ret()
            });
            explosion_attach_light(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            if node != 0 && form != 0 {
                assert_eq!(log.last().unwrap(), &(0x0066_b0d0, vec![0x10c4, 0x7000]));
            } else {
                assert!(!log.iter().any(|(address, _)| *address == 0x0066_b0d0));
            }
        }
    }

    #[test]
    fn light_remove_checks_pointer_and_preserves_pending_stack_argument() {
        for light in [0u32, 0x2000] {
            let mut e = engine_with(&[0x00b5_eed0, 0x0066_b0d0]);
            e.register_double(0x0055_9450, move |_, a| {
                assert_eq!(a, &[0x10c4]);
                light.into_ret()
            });
            e.register(0x0045_0b80, |_, a| {
                assert_eq!(a, &[0]);
                0x3000u32.into_ret()
            });
            fn_009b12d0(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            assert_eq!(log.len(), if light == 0 { 1 } else { 5 });
            if light != 0 {
                assert_eq!(log[3], (0x00b5_eed0, vec![0x3000, light]));
                assert_eq!(log[4], (0x0066_b0d0, vec![0x10c4, 0]));
            }
        }
    }

    #[test]
    fn light_fade_passes_computed_factor_through_both_clamps() {
        for light in [0u32, 0x2000] {
            let mut e = engine_with(&[0x0050_dd20]);
            mapped(&mut e, 0x0101_1000);
            e.set_global(0x0101_1590, 2.0f64);
            e.register_double(0x0055_9450, move |_, _| light.into_ret());
            e.register(0x0051_b8e0, |_, _| 1.0f32.into_ret());
            e.register(0x0050_7e10, |_, _| 8.0f32.into_ret());
            e.register(0x0040_ebd0, |_, a| {
                assert_eq!(a, &[1.0f32.to_bits(), 0.75f32.to_bits()]);
                0.75f32.into_ret()
            });
            e.register(0x0040_4010, |_, a| {
                assert_eq!(a, &[0, 0.75f32.to_bits()]);
                0.75f32.into_ret()
            });
            fn_009b1320(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            if light == 0 {
                assert_eq!(log.len(), 1);
            } else {
                assert_eq!(
                    log.last().unwrap(),
                    &(0x0050_dd20, vec![light, 0.75f32.to_bits()])
                );
            }
        }
    }

    #[test]
    fn radiation_update_requires_positive_inputs_and_floors_nonpositive_result() {
        for (radius, duration, age, expected) in [
            (0.0f32, 4.0f32, 1.0f32, None),
            (2.0, 0.0, 1.0, None),
            (f32::NAN, 4.0, 1.0, None),
            (2.0, 4.0, 1.0, Some(1.5)),
            (2.0, 4.0, 4.0, Some(0.01)),
            (2.0, 4.0, 5.0, Some(0.01)),
        ] {
            let mut e = engine_with(&[0x0042_2350, 0x6000_0048]);
            for address in [0x1000, 0x2000, 0x0101_2000, 0x0101_3000] {
                mapped(&mut e, address);
            }
            e.mem.set_u32(0x1000, 0x2000);
            e.mem.set_u32(0x2048, 0x6000_0048);
            e.set_global(0x0101_2060, 0.0f64);
            e.set_global(0x0101_3ea4, 0.01f32);
            e.register(0x0041_81e0, |_, _| 0x3000u32.into_ret());
            e.register_double(0x008d_01e0, move |_, _| radius.into_ret());
            e.register_double(0x004e_3cc0, move |_, _| duration.into_ret());
            e.register_double(0x0051_b8e0, move |_, _| age.into_ret());
            e.register(0x005d_43c0, |_, a| {
                assert_eq!(a, &[0x1000]);
                0x4000u32.into_ret()
            });
            fn_009b13a0(&mut e, Ptr::new(0x1000));
            let log = e.call_log.unwrap();
            let update = log.iter().find(|(address, _)| *address == 0x0042_2350);
            if let Some(expected) = expected {
                assert_eq!(update.unwrap().1, vec![0x4000, f32::to_bits(expected)]);
                assert_eq!(log.last().unwrap().1, vec![0x1000, 0x4000_0000]);
            } else {
                assert!(update.is_none());
            }
        }
    }

    #[test]
    fn process_level_retains_loaded_cell_or_matching_level() {
        for (node, fallback, loaded, level, retained) in [
            (0u32, false, false, 2u32, false),
            (1, false, true, 2, true),
            (0, true, true, 2, true),
            (1, false, false, 3, true),
            (1, false, false, 2, false),
        ] {
            let mut e = engine_with(&[0x6000_0010]);
            for address in [0x1000, 0x2000, 0x011c_3000, 0x011d_e000] {
                mapped(&mut e, address);
            }
            e.mem.set_u32(0x1000, 0x2000);
            e.mem.set_u32(0x21d0, 0x6000_01d0);
            e.mem.set_u32(0x2010, 0x6000_0010);
            e.mem.set_u32(0x1068, 0x3000);
            e.set_global(0x011c_3b3c, 0x4000u32);
            e.set_global(0x011d_ea10, 0x5000u32);
            e.register_double(0x0045_cd60, move |_, a| {
                assert_eq!(a, &[0x3000]);
                level.into_ret()
            });
            e.register(0x008d_6f30, |_, _| 0x6000u32.into_ret());
            e.register_double(0x6000_01d0, move |_, _| node.into_ret());
            e.register_double(0x0044_5750, move |_, a| {
                assert_eq!(a, &[0x4000, 0x1000]);
                fallback.into_ret()
            });
            e.register_double(0x0045_11e0, move |_, a| {
                assert_eq!(a, &[0x5000, 0x6000, 1]);
                loaded.into_ret()
            });
            assert_eq!(fn_009b1450(&mut e, Ptr::new(0x1000)), retained);
            assert_eq!(
                e.call_log
                    .unwrap()
                    .iter()
                    .any(|(address, _)| *address == 0x6000_0010),
                !retained
            );
        }
    }

    fn map_generated_constant_pages(e: &mut Engine) {
        for address in [
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x011a_9000,
            0x011c_3000,
            0x011f_4000,
        ] {
            e.map(address, 0x1000);
        }
    }

    fn list_doubles(e: &mut Engine) {
        // Generated two-word list nodes: the helpers expose payload and next.
        e.register(0x0082_56d0, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0096_a2d0, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(0x005a_e3d0, |e, a| {
            let record = e.mem.u32(a[1]);
            e.mem.set_u32(0x1f00, a[0]);
            e.mem.set_u32(0x1f04, record);
            Ret::default()
        });
        e.register(0x0040_1000, |_, a| {
            assert!([8, 16].contains(&a[0]));
            0x3000u32.into_ret()
        });
    }

    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x1000, 0x6000);
        map_generated_constant_pages(&mut e);
        e.call_log = Some(vec![]);
        list_doubles(&mut e);
        e
    }

    fn empty_explosion_destructor_engine() -> Engine {
        let mut e = list_engine();
        for address in [0x0043_fcd0, 0x008d_6f30, 0x0055_9450] {
            e.register(address, |_, _| 0u32.into_ret());
        }
        for address in [
            0x0042_f760,
            0x0045_cec0,
            0x0048_3710,
            0x0046_ffb0,
            0x0066_b0d0,
            0x0042_f780,
            0x0092_ec40,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0042_26e0, |_, _| true.into_ret());
        e.register(0x0040_8d60, |_, _| 0x2600u32.into_ret());
        e.register(0x0040_1030, |_, _| Ret::default());
        e.register(0x00ad_8ce0, |_, _| false.into_ret());
        e.mem.set_u8(0x2600, 0);
        e
    }

    #[test]
    fn parameterized_constructor_preserves_word_abi_and_owner_allocation_branches() {
        for (owner, actor, allocated) in [
            (0u32, false, 0u32),
            (0, false, 0x6000),
            (0x4000, false, 0x6000),
            (0x4000, true, 0),
        ] {
            for definition in [0u32, 0x3000] {
                for orient in [false, true] {
                    let mut e = engine_with(&[
                        0x0092_eb50,
                        0x0063_3c90,
                        0x0096_a2d0,
                        0x0041_a250,
                        0x0042_f730,
                        0x0068_15c0,
                        0x0093_0700,
                        0x0057_5690,
                        0x0066_b0d0,
                        0x0042_f780,
                        0x0056_fa90,
                    ]);
                    e.map(0x1000, 0x6000);
                    map_generated_constant_pages(&mut e);
                    for offset in (0..0x104).step_by(4) {
                        e.mem.set_u32(0x1000 + offset, 0xa5a5_a5a5);
                    }
                    e.mem.set_u32(0x4000, 0x5000);
                    e.mem.set_u32(0x5100, 0x6000_0100);
                    e.register_double(0x6000_0100, move |_, _| actor.into_ret());
                    e.register_double(0x0040_1000, move |_, a| {
                        assert_eq!(a, &[0x24]);
                        allocated.into_ret()
                    });
                    e.register(0x0082_5ad0, |_, a| {
                        assert_eq!(a, &[0x6000, 0x1000]);
                        0x6004u32.into_ret()
                    });
                    e.register(0x0057_5830, |e, a| {
                        assert_eq!(a[0], 0x1000);
                        assert_eq!(
                            [e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8)],
                            [11, 12, 13]
                        );
                        Ret::default()
                    });
                    e.register(0x0041_81e0, |_, a| {
                        assert_eq!(a, &[0x1000]);
                        0x3000u32.into_ret()
                    });
                    e.register(0x0047_7900, |_, a| {
                        assert_eq!(a, &[0x3000]);
                        8.0f32.into_ret()
                    });
                    e.register_double(0x0047_7950, move |_, a| match a[1] {
                        1 => true.into_ret(),
                        2 => orient.into_ret(),
                        0x40 => 0x82u32.into_ret(),
                        _ => panic!("unexpected mask"),
                    });
                    e.mem.set_f32(0x308c, 9.0);
                    e.set_global(0x0101_7718, 3.0f32);
                    e.set_global(0x0101_2054, -1.0f32);
                    for i in 0..3 {
                        e.set_global(0x011f_426c + i * 4, 90 + i);
                    }
                    for i in 0..9 {
                        e.set_global(0x011a_9448 + i * 4, 100 + i);
                    }
                    let result = e.call(
                        0x009a_c200,
                        &[
                            0x1000, definition, owner, 0x7000, 11, 12, 13, 21, 22, 23, 24, 25, 26,
                            27, 28, 29,
                        ],
                    );
                    assert_eq!(result.u32(), 0x1000);
                    assert_eq!(e.mem.u32(0x10c8), owner);
                    assert_eq!(e.mem.u32(0x10cc), 0xa5a5_a5a5); // This overload does not write +0xCC after the base constructor.
                    assert_eq!(
                        e.mem.u32(0x10dc),
                        if actor || allocated == 0 { 0 } else { 0x6004 }
                    );
                    assert_eq!(e.mem.f32(0x108c), 3.0);
                    assert_eq!(e.mem.f32(0x1100), -1.0);
                    assert_eq!(e.mem.u8(0x10d8), if definition == 0 { 0 } else { 0x82 });
                    assert_eq!(
                        e.mem.u32(0x1094),
                        if definition == 0 {
                            0xa5a5_a5a5
                        } else {
                            8.0f32.to_bits()
                        }
                    );
                    assert_eq!(
                        e.mem.u32(0x1098),
                        if definition == 0 {
                            0xa5a5_a5a5
                        } else {
                            9.0f32.to_bits()
                        }
                    );
                    for base in [0x10e8, 0x10f4] {
                        for i in 0..3 {
                            assert_eq!(e.mem.u32(base + i * 4), 90 + i);
                        }
                    }
                    let calls = e.call_log.unwrap();
                    assert_eq!(calls.iter().any(|c| c.0 == 0x0040_1000), !actor);
                    assert_eq!(
                        calls.iter().any(|c| c.0 == 0x0082_5ad0),
                        !actor && allocated != 0
                    );
                    let rotations: Vec<_> = calls.iter().filter(|c| c.0 == 0x0056_fa90).collect();
                    assert_eq!(
                        rotations[0].1,
                        vec![0x1000, 21, 22, 23, 24, 25, 26, 27, 28, 29]
                    );
                    assert_eq!(
                        rotations.len(),
                        if definition != 0 && orient { 2 } else { 1 }
                    );
                    if rotations.len() == 2 {
                        assert_eq!(
                            rotations[1].1,
                            vec![0x1000, 100, 101, 102, 103, 104, 105, 106, 107, 108]
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn target_lookup_updates_zero_bytes_only_and_skips_null_payloads() {
        let mut e = list_engine();
        e.mem.set_u32(0x10a8, 0x1200);
        e.mem.set_u32(0x1200, 0x2000);
        e.mem.set_u32(0x2000, 0x9000);
        e.mem.set_u8(0x2004, 0);
        e.mem.set_u8(0x2005, 0x7f);
        assert_eq!(
            fn_009b0e00(&mut e, Ptr::new(0x1000), Ptr::new(0x9000), 0x80, 0xff).addr(),
            0x2000
        );
        assert_eq!(e.mem.u8(0x2004), 0x80);
        assert_eq!(e.mem.u8(0x2005), 0x7f);
        fn_009b0e00(&mut e, Ptr::new(0x1000), Ptr::new(0x9000), 0, 0);
        assert_eq!(e.mem.u8(0x2004), 0x80);
        assert!(!e.call_log.unwrap().iter().any(|c| c.0 == 0x0040_1000));
    }

    #[test]
    fn target_lookup_allocates_after_nonmatch_and_initializes_embedded_list() {
        let mut e = list_engine();
        e.mem.set_u32(0x10a4, 0x2000);
        e.mem.set_u32(0x2000, 0x8888);
        assert_eq!(
            fn_009b0e00(&mut e, Ptr::new(0x1000), Ptr::new(0x9999), 2, 3).addr(),
            0x3000
        );
        assert_eq!(e.mem.u32(0x3000), 0x9999);
        assert_eq!(e.mem.u8(0x3004), 2);
        assert_eq!(e.mem.u8(0x3005), 3);
        assert_eq!(e.mem.u8(0x3006), 0);
        assert_eq!(e.mem.u32(0x3008), 0);
        assert_eq!(e.mem.u32(0x1f00), 0x10a4);
        assert_eq!(e.mem.u32(0x1f04), 0x3000);
    }

    #[test]
    fn cached_damage_handles_cache_nan_owner_and_weapon_gates() {
        for cached in [12.0f32, -1.0, f32::NAN] {
            for owner in [0u32, 0x2000] {
                for actor in [false, true] {
                    for weapon in [0u32, 0x3000] {
                        let mut e = list_engine();
                        e.set_global(0x0101_2060, 0.0f64);
                        e.set_global(0x0101_6408, 0.01f64);
                        e.mem.set_f32(0x1100, cached);
                        e.mem.set_u32(0x10c8, owner);
                        e.mem.set_u32(0x10e0, weapon);
                        e.mem.set_u32(0x2000, 0x4000);
                        e.mem.set_u32(0x4100, 0x6000_0100);
                        e.mem.set_u32(0x20a4, 0x5000);
                        e.mem.set_u32(0x500c, 0x6000_000c);
                        e.register_double(0x6000_0100, move |_, _| actor.into_ret());
                        e.register(0x6000_000c, |_, a| {
                            assert_eq!(a, &[0x20a4, 42]);
                            25.0f32.into_ret()
                        });
                        e.register(0x0041_81e0, |_, _| 0x7000u32.into_ret());
                        e.register(0x0064_79e0, |_, a| {
                            assert_eq!(a, &[0x7000]);
                            10.0f32.into_ret()
                        });
                        e.register(0x008d_85e0, |_, a| {
                            assert_eq!(a, &[0x3000]);
                            42u32.into_ret()
                        });
                        e.register(0x0040_3e20, |_, a| {
                            if a[0] == 0x011c_ece8 {
                                0x6000u32
                            } else {
                                assert_eq!(a[0], 0x011c_f350);
                                0x6004u32
                            }
                            .into_ret()
                        });
                        e.mem.set_f32(0x6000, 1.0);
                        e.mem.set_f32(0x6004, 2.0);
                        let expected = if cached == 12.0 {
                            12.0
                        } else if owner != 0 && actor && weapon != 0 {
                            15.0
                        } else {
                            10.0
                        };
                        assert_eq!(fn_009b0f80(&mut e, Ptr::new(0x1000)), expected);
                        assert_eq!(e.mem.f32(0x1100), expected);
                        let count = e.call_log.as_ref().unwrap().len();
                        assert_eq!(fn_009b0f80(&mut e, Ptr::new(0x1000)), expected);
                        assert_eq!(e.call_log.unwrap().len(), count);
                    }
                }
            }
        }
    }

    #[test]
    fn cached_damage_reloads_owner_and_weapon_after_is_actor_callback() {
        let mut e = list_engine();
        e.set_global(0x0101_2060, 0.0f64);
        e.set_global(0x0101_6408, 0.01f64);
        e.mem.set_f32(0x1100, -1.0);
        e.mem.set_u32(0x10c8, 0x2000);
        e.mem.set_u32(0x10e0, 0x3000);
        e.mem.set_u32(0x2000, 0x4100);
        e.mem.set_u32(0x30a4, 0x5100);
        e.mem.set_u32(0x510c, 0x6000_000c);
        e.register_double(0x6000_0100, |e, _| {
            e.mem.set_u32(0x10c8, 0x3000);
            e.mem.set_u32(0x10e0, 0x4000);
            true.into_ret()
        });
        // The first owner's actor predicate is at its original vtable slot.
        e.mem.set_u32(0x4100 + 0x100, 0x6000_0100);
        e.register(0x0041_81e0, |_, _| 0x7000u32.into_ret());
        e.register(0x0064_79e0, |_, _| 10.0f32.into_ret());
        e.register(0x008d_85e0, |_, a| {
            assert_eq!(a, &[0x4000]);
            42u32.into_ret()
        });
        e.register(0x6000_000c, |_, a| {
            assert_eq!(a, &[0x30a4, 42]);
            25.0f32.into_ret()
        });
        e.register(0x0040_3e20, |_, a| {
            match a[0] {
                0x011c_ece8 => 0x6000u32,
                0x011c_f350 => 0x6004u32,
                _ => panic!("unexpected game setting"),
            }
            .into_ret()
        });
        e.mem.set_f32(0x6000, 1.0);
        e.mem.set_f32(0x6004, 2.0);

        assert_eq!(fn_009b0f80(&mut e, Ptr::new(0x1000)), 15.0);
        assert!(e
            .call_log
            .unwrap()
            .iter()
            .any(|call| { call.0 == 0x6000_000c && call.1 == [0x30a4, 42] }));
    }

    #[test]
    fn target_scan_marks_complete_and_creates_player_record_without_phantom() {
        let mut e = list_engine();
        e.map(0x011d_e000, 0x1000);
        e.set_global(0x011d_ea3c, 0x9000u32);
        e.register(0x0055_9450, |_, a| {
            assert_eq!(a, &[0x10a0]);
            0u32.into_ret()
        });

        fn_009afcc0(&mut e, Ptr::new(0x1000));

        assert_eq!(e.mem.u8(0x10d5), 1);
        assert_eq!(e.mem.u32(0x3000), 0x9000);
        assert_eq!(e.mem.u8(0x3004), 0);
        assert_eq!(e.mem.u8(0x3005), 1);
        assert_eq!(e.mem.u32(0x1f00), 0x10a4);
        assert_eq!(e.mem.u32(0x1f04), 0x3000);
    }

    #[test]
    fn rigid_body_scan_marks_complete_when_phantom_is_null() {
        let mut e = list_engine();
        e.register(0x0055_9450, |_, a| {
            assert_eq!(a, &[0x10a0]);
            0u32.into_ret()
        });

        fn_009afef0(&mut e, Ptr::new(0x1000));

        assert_eq!(e.mem.u8(0x10d7), 1);
        let calls = e.call_log.unwrap();
        assert_eq!(calls.len(), 1);
        assert!(calls.iter().all(|call| call.0 == 0x0055_9450));
    }

    #[test]
    fn rigid_body_scan_pushes_low_layer_body() {
        let mut e = list_engine();
        let push_state = std::rc::Rc::new(std::cell::Cell::new(false));
        let callback_state = std::rc::Rc::clone(&push_state);
        e.register(0x0055_9450, |_, a| {
            assert_eq!(a, &[0x10a0]);
            0x2000u32.into_ret()
        });
        e.register(0x0041_81e0, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x7000u32.into_ret()
        });
        e.register(0x0047_7950, |_, a| {
            assert_eq!(a, &[0x7000, 0x20]);
            false.into_ret()
        });
        e.register(0x004a_e750, |_, a| {
            assert_eq!(a, &[0x2000]);
            0x2100u32.into_ret()
        });
        e.register(0x0062_d2f0, |_, a| {
            assert_eq!(a, &[0x2100]);
            0x3000u32.into_ret()
        });
        e.register(0x0072_6070, |_, a| {
            assert_eq!(a, &[0x3000]);
            1u32.into_ret()
        });
        e.register(0x0062_9ab0, |_, a| {
            assert_eq!(a, &[0x3000, 0]);
            0x3100u32.into_ret()
        });
        e.mem.set_u32(0x3100, 0x4000);
        e.register(0x004b_59f0, |_, a| {
            assert_eq!(a, &[0x4000]);
            0x5000u32.into_ret()
        });
        e.register(0x004b_5a80, |_, a| {
            assert_eq!(a, &[0x5000]);
            0x6000u32.into_ret()
        });
        e.register(0x0043_b4f0, |_, a| {
            assert_eq!(a[0], 0x6000);
            0x6100u32.into_ret()
        });
        e.register(0x0043_b4d0, |_, a| {
            assert_eq!(a, &[0x6100]);
            0u32.into_ret()
        });
        e.register(0x0051_7630, |_, a| {
            assert_eq!(a, &[0x6000]);
            1u32.into_ret()
        });
        e.register_double(0x009b_0920, move |_, a| {
            assert_eq!(a, &[0x1000, 0x6000]);
            callback_state.set(true);
            Ret::default()
        });

        fn_009afef0(&mut e, Ptr::new(0x1000));

        assert!(push_state.get());
        assert_eq!(e.mem.u8(0x10d7), 1);
    }

    #[test]
    fn rigid_body_impulse_rejects_magnitude_at_global_threshold() {
        let mut e = list_engine();
        e.set_global(0x0101_2060, 2.0f64);
        e.mem.set_f32(0x2074, 2.0);
        e.register(0x0041_81e0, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x2000u32.into_ret()
        });

        fn_009b0920(&mut e, Ptr::new(0x1000), Ptr::new(0x3000));

        let calls = e.call_log.unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, 0x0041_81e0);
    }

    #[test]
    fn rigid_body_impulse_scales_linear_and_random_angular_vectors() {
        let mut e = list_engine();
        e.set_global(0x0101_2060, 0.0f64);
        e.set_global(0x0101_2054, -1.0f32);
        e.mem.set_u32(0x1000, 0x4000);
        e.mem.set_u32(0x4000 + 0x1f4, 0x6000_01f4);
        e.mem.set_u32(0x3000, 0x4100);
        e.mem.set_u32(0x4100 + 0xf0, 0x6000_00f0);
        e.mem.set_u32(0x10cc, 0);
        e.mem.set_f32(0x2074, 8.0);
        e.mem.set_f32(0x5000, 16.0);
        e.mem.set_f32(0x5004, 0.5);
        e.mem.set_f32(0x5008, 0.25);
        e.mem.set_f32(0x5010, 0.75);
        e.register(0x0041_81e0, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x2000u32.into_ret()
        });
        e.register(0x6000_01f4, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x3400u32.into_ret()
        });
        e.register(0x6000_00f0, |e, a| {
            assert_eq!(a.len(), 2);
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[1] + 4, 2.0);
            e.mem.set_f32(a[1] + 8, 3.0);
            0x3300u32.into_ret()
        });
        e.register(0x0045_8620, |e, a| {
            assert_eq!(a[1], 0x3300);
            for offset in [0, 4, 8] {
                e.mem.set_f32(a[0] + offset, offset as f32 + 1.0);
            }
            Ret::default()
        });
        e.register(0x0043_9ef0, |e, a| {
            assert_eq!(a.len(), 3);
            assert_ne!(a[0], 0);
            assert_eq!(a[2], 0x3400);
            e.mem.set_f32(a[1], 4.0);
            e.mem.set_f32(a[1] + 4, 5.0);
            e.mem.set_f32(a[1] + 8, 6.0);
            a[1].into_ret()
        });
        e.register(0x004a_0c10, |_, a| {
            assert_eq!(a.len(), 1);
            Ret::default()
        });
        e.register(0x0043_b4f0, |_, a| {
            assert_eq!(a[0], 0x3000);
            0x3500u32.into_ret()
        });
        e.register(0x0062_4070, |_, a| {
            assert_eq!(a, &[0x3500]);
            false.into_ret()
        });
        e.register(0x0062_b520, |_, a| {
            assert_eq!(a, &[8.0f32.to_bits(), 0x3000, 1]);
            8.0f32.into_ret()
        });
        e.register(0x0040_3e20, |_, a| match a[0] {
            0x011d_0c9c => 0x5000u32.into_ret(),
            0x011c_ff64 => 0x5004u32.into_ret(),
            0x011d_02ec => 0x5008u32.into_ret(),
            other => panic!("unexpected setting {other:#x}"),
        });
        e.register(0x0040_ebd0, |_, a| {
            assert_eq!(a, &[8.0f32.to_bits(), 16.0f32.to_bits()]);
            8.0f32.into_ret()
        });
        let scaled = std::rc::Rc::new(std::cell::Cell::new(0));
        let scaled_calls = std::rc::Rc::clone(&scaled);
        e.register_double(0x0045_bb20, move |e, a| {
            assert_eq!(a.len(), 3);
            let scale = f32::from_bits(a[2]);
            for offset in [0, 4, 8] {
                let value = e.mem.f32(a[0] + offset) * scale;
                e.mem.set_f32(a[1] + offset, value);
            }
            scaled_calls.set(scaled_calls.get() + 1);
            a[1].into_ret()
        });
        e.register(0x0047_6b70, |_, a| {
            assert_eq!(a, &[(-1.0f32).to_bits(), 1.0f32.to_bits()]);
            0.5f32.into_ret()
        });
        e.register(0x0041_6870, |e, a| {
            assert_eq!(a.len(), 4);
            for (index, value) in a[1..].iter().copied().enumerate() {
                e.mem
                    .set_f32(a[0] + index as u32 * 4, f32::from_bits(value));
            }
            a[0].into_ret()
        });
        e.register(0x004a_3e00, |_, a| {
            assert_eq!(a.len(), 2);
            a[0].into_ret()
        });
        e.register(0x004a_e750, |_, a| {
            assert_eq!(a, &[0x3000]);
            0u32.into_ret()
        });

        fn_009b0920(&mut e, Ptr::new(0x1000), Ptr::new(0x3000));

        assert_eq!(scaled.get(), 4);
        let calls = e.call_log.unwrap();
        assert_eq!(calls.iter().filter(|call| call.0 == 0x0045_bb20).count(), 4);
        assert_eq!(calls.iter().filter(|call| call.0 == 0x0047_6b70).count(), 3);
    }

    #[test]
    fn explosion_save_serializes_base_fields_and_optional_caster_ids() {
        for has_caster in [false, true] {
            let mut e = list_engine();
            let this = Ptr::new(0x1000);
            let buffer = Ptr::new(0x2000);
            e.mem.set_u32(0x10c8, 0x3100);
            e.mem.set_u32(0x10cc, 0x3200);
            e.mem.set_u32(0x10e0, 0x3300);
            if has_caster {
                e.mem.set_u32(0x10dc, 0x3400);
                e.mem.set_u32(0x3418, 0x3500);
                e.mem.set_u32(0x341c, 0x3600);
                e.mem.set_u32(0x3420, 0x3700);
            }
            e.register(0x0093_2880, |_, a| {
                assert_eq!(a, &[0x1000, 0x2000]);
                Ret::default()
            });
            e.register(0x0086_5e50, |_e, a| {
                assert_eq!(a.len(), 4);
                assert_eq!(a[0], 0x2000);
                assert!(matches!(a[2], 1 | 4 | 12));
                assert_eq!(a[3], 0);
                Ret::default()
            });
            e.register(0x0086_5df0, |_, a| {
                assert_eq!(a.len(), 3);
                assert_eq!(a[0], 0x2000);
                assert_eq!(a[2], 0);
                Ret::default()
            });
            e.register(0x0086_5db0, |_, a| {
                assert_eq!(a.len(), 3);
                assert_eq!(a[0], 0x2000);
                assert_eq!(a[2], 0);
                Ret::default()
            });
            e.register(0x0055_9450, move |e, a| {
                assert_eq!(a, &[0x10d0]);
                e.mem.u32(0x10dc).into_ret()
            });
            e.register(0x0082_5c00, |_, a| {
                assert_eq!(a, &[0x3400]);
                0x44u32.into_ret()
            });
            e.register(0x0040_a1e0, |_, a| {
                assert_eq!(a, &[0x3500]);
                0x55u32.into_ret()
            });
            e.register(0x0082_54c0, |_, a| {
                assert_eq!(a, &[0x3600]);
                0x66u32.into_ret()
            });

            fn_009b2040(&mut e, this, buffer);

            let calls = e.call_log.unwrap();
            assert_eq!(
                calls.iter().filter(|call| call.0 == 0x0086_5e50).count(),
                15
            );
            assert_eq!(
                calls.iter().filter(|call| call.0 == 0x0086_5df0).count(),
                3 + if has_caster { 1 } else { 0 }
            );
            assert_eq!(
                calls.iter().filter(|call| call.0 == 0x0086_5db0).count(),
                2 * if has_caster { 1 } else { 0 }
            );
        }
    }

    #[test]
    fn explosion_load_obeys_version_gates_and_optional_caster_allocation() {
        for (version, has_caster) in [(13u8, false), (20, true)] {
            let mut e = list_engine();
            e.mem.set_u32(0x2000, 0x4000);
            e.mem.set_u32(0x4000, 0x6000_0000);
            e.mem.set_i32(0x10e4, 3);
            e.register(0x0093_2a00, |_, a| {
                assert_eq!(a, &[0x1000, 0x2000]);
                Ret::default()
            });
            e.register_double(0x6000_0000, move |_, a| {
                assert_eq!(a, &[0x2000]);
                version.into_ret()
            });
            e.register_double(0x0086_4980, move |e, a| {
                assert_eq!(a[0], 0x2000);
                let destination = a[1];
                if a[2] == 1 {
                    e.mem.set_u8(destination, u8::from(has_caster));
                } else if destination == 0x10e4 {
                    e.mem.set_i32(destination, 3);
                } else {
                    e.mem.set_u32(destination, 0x44);
                }
                Ret::default()
            });
            e.register(0x0086_48e0, |_, a| {
                assert_eq!(a[0], 0x2000);
                Ret::default()
            });
            e.register(0x0086_48a0, |_, a| {
                assert_eq!(a, &[0x2000]);
                0x100u32.into_ret()
            });
            e.register(0x0048_39c0, |_, a| {
                assert_eq!(a, &[0x100]);
                0x200u32.into_ret()
            });
            e.register(0x00ec_43fb, |_, a| {
                assert_eq!(a, &[0x200, 0, 0x0118_3028, 0x0118_3998, 0]);
                0x300u32.into_ret()
            });
            e.register(0x0066_e330, |_, a| {
                assert_eq!(a, &[0x44]);
                0u32.into_ret()
            });
            e.register(0x0042_f780, |e, a| {
                assert_eq!(a, &[0x10d0, 0]);
                e.mem.set_u32(a[0] + 0xc, a[1]);
                Ret::default()
            });
            e.register(0x0040_1000, |_, a| {
                assert_eq!(a, &[0x24]);
                0x5000u32.into_ret()
            });
            e.register(0x0082_5ad0, |_, a| {
                assert_eq!(a, &[0x5000, 0x1000]);
                0x6000u32.into_ret()
            });

            fn_009b22c0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));

            let calls = e.call_log.unwrap();
            let loaded = calls
                .iter()
                .filter(|call| call.0 == 0x0086_4980)
                .map(|call| call.1[1])
                .collect::<Vec<_>>();
            assert_eq!(loaded.contains(&0x109c), version > 13);
            assert_eq!(loaded.contains(&0x1100), version > 19);
            assert_eq!(e.mem.u32(0x10e0), 0x300);
            assert_eq!(e.mem.u32(0x10dc), if has_caster { 0x6000 } else { 0 });
            assert_eq!(e.mem.u8(0x10d5), 1);
            assert_eq!(e.mem.u8(0x10d6), 1);
            assert_eq!(e.mem.u8(0x10d7), 1);
            assert_eq!(
                calls.iter().filter(|call| call.0 == 0x0040_1000).count(),
                if has_caster { 1 } else { 0 }
            );
        }
    }

    #[test]
    fn explosion_init_load_resolves_reference_and_caster_members() {
        let mut e = list_engine();
        e.mem.set_u32(0x10c8, 0x111);
        e.mem.set_u32(0x10cc, 0x222);
        e.mem.set_u32(0x10dc, 0x3000);
        e.mem.set_u32(0x3018, 0x333);
        e.mem.set_u32(0x301c, 0x444);
        e.mem.set_u32(0x3020, 0x555);
        e.mem.set_u32(0x1000, 0x4000);
        e.mem.set_u32(0x4000 + 0x304, 0x6000_0304);
        e.register(0x0093_2b70, |_, a| {
            assert_eq!(a, &[0x1000, 0x2000]);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| {
            assert_eq!(a.len(), 1);
            (a[0] + 0x100).into_ret()
        });
        e.register(0x00ec_43fb, |_, a| {
            assert_eq!(&a[1..], &[0, 0x0118_3028, 0x0118_41cc, 0]);
            (a[0] + 0x1000).into_ret()
        });
        e.register(0x0040_a250, |_, a| {
            assert_eq!(a, &[0x333]);
            0x433u32.into_ret()
        });
        e.register(0x0082_5550, |_, a| {
            assert_eq!(a, &[0x444]);
            0x544u32.into_ret()
        });
        e.register(0x008d_6f30, |_, a| {
            assert_eq!(a, &[0x1000]);
            0u32.into_ret()
        });
        e.register(0x6000_0304, |_, a| {
            assert_eq!(a, &[0x1000]);
            Ret::default()
        });

        fn_009b25b0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));

        assert_eq!(e.mem.u32(0x10c8), 0x1211);
        assert_eq!(e.mem.u32(0x10cc), 0x1322);
        assert_eq!(e.mem.u32(0x3018), 0x433);
        assert_eq!(e.mem.u32(0x301c), 0x544);
        assert_eq!(e.mem.u32(0x3020), 0x1655);
    }

    #[test]
    fn explosion_update_handles_first_target_age_and_expiration_gates() {
        let mut e = list_engine();
        e.mem.set_u32(0x10a4, 0x2100);
        e.mem.set_u32(0x2100, 0x2200);
        e.mem.set_u32(0x2200, 0x2300);
        e.mem.set_u32(0x10c8, 0x2000);
        e.mem.set_u32(0x1000, 0x4000);
        e.mem.set_u32(0x40c4, 0x6000_00c4);
        e.mem.set_f32(0x1090, 1.0);
        e.register(0x0082_56d0, |_, a| {
            assert_eq!(a, &[0x10a4]);
            false.into_ret()
        });
        e.register(0x0068_15c0, |_, a| {
            assert_eq!(a, &[0x10a4]);
            0x2100u32.into_ret()
        });
        e.register(0x008d_0600, |_, a| {
            assert_eq!(a, &[0x011e_0e80, 0x2000, 0x2300]);
            Ret::default()
        });
        let mut times = [2.0f32, 5.0, 6.0].into_iter();
        e.register_double(0x0051_b8e0, move |_, a| {
            assert_eq!(a, &[0x1000]);
            times.next().unwrap().into_ret()
        });
        e.register(0x0050_7e10, |_, a| {
            assert_eq!(a, &[0x1000]);
            4.0f32.into_ret()
        });
        e.register(0x009a_fcc0, |_, a| {
            assert_eq!(a, &[0x1000]);
            Ret::default()
        });
        e.register(0x008d_6f30, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x3000u32.into_ret()
        });
        e.register(0x0041_81e0, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x3100u32.into_ret()
        });
        e.register(0x004e_3cc0, |_, a| {
            assert_eq!(a, &[0x3100]);
            5.0f32.into_ret()
        });
        e.register(0x008d_01e0, |_, a| {
            assert_eq!(a, &[0x3100]);
            0.0f32.into_ret()
        });
        e.register(0x0055_9450, |_, _| 0u32.into_ret());
        e.register(0x6000_00c4, |_, a| {
            assert_eq!(a, &[0x1000, 1]);
            Ret::default()
        });

        fn_009ae5d0(&mut e, Ptr::new(0x1000), 0);

        assert_eq!(e.mem.u32(0x10e4), 1);
        let calls = e.call_log.unwrap();
        assert!(calls.iter().any(|call| call.0 == 0x009a_fcc0));
        assert!(calls.iter().any(|call| call.0 == 0x008d_0600));
        assert!(calls.iter().any(|call| call.0 == 0x6000_00c4));
    }

    #[test]
    fn explosion_destructor_runs_conditional_cleanup_and_member_destruction() {
        let mut e = list_engine();
        let this = Ptr::new(0x1000);
        e.mem.set_u32(0x10dc, 0x2500);
        e.mem.set_u32(0x2500, 0x4500);
        e.mem.set_u32(0x4500, 0x6000_0003);
        e.mem.set_u32(0x2000, 0x4000);
        e.mem.set_u32(0x400c, 0x6000_0001);
        e.mem.set_u32(0x2400, 0x4400);
        e.mem.set_u32(0x44a0, 0x6000_0002);
        e.mem.set_u32(0x011c_3f2c, 0x2b00);
        e.mem.set_u32(0x2800, this.addr());
        e.mem.set_u8(0x2600, 1);
        let list_visits = std::cell::Cell::new(0);

        e.register(0x0043_fcd0, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x2000u32.into_ret()
        });
        e.register(0x6000_0001, |_, a| {
            assert_eq!(a, &[0x2000]);
            0x5000u32.into_ret()
        });
        e.register(0x0057_84b0, |_, a| {
            assert_eq!(a, &[0x5000]);
            true.into_ret()
        });
        e.register(0x0057_8170, |_, a| {
            assert_eq!(a, &[0x5000]);
            Ret::default()
        });
        e.register(0x008d_6f30, |_, a| {
            assert_eq!(a, &[0x1000]);
            0x2100u32.into_ret()
        });
        e.register(0x0045_43c0, |_, a| {
            assert_eq!(a, &[0x2100]);
            true.into_ret()
        });
        e.register(0x00c6_9ee0, |_, a| {
            assert_eq!(a, &[0x2000, 1, 0]);
            Ret::default()
        });
        e.register(0x0055_9450, |_, a| match a[0] {
            0x10a0 => 0x2400u32.into_ret(),
            0x10c4 => 0u32.into_ret(),
            0x011c_7b8c => 0x2700u32.into_ret(),
            _ => panic!("unexpected smart pointer or list"),
        });
        e.register(0x6000_0002, |_, a| {
            assert_eq!(a, &[0x2400]);
            Ret::default()
        });
        e.register(0x0066_b0d0, |_, a| {
            assert_eq!(a, &[0x10a0, 0]);
            Ret::default()
        });
        e.register_double(0x0082_56d0, move |_, a| {
            assert_eq!(a, &[0x10a4]);
            let empty = list_visits.get() != 0;
            list_visits.set(list_visits.get() + 1);
            empty.into_ret()
        });
        e.register(0x0068_15c0, |_, a| {
            assert_eq!(a, &[0x10a4]);
            0x2200u32.into_ret()
        });
        e.mem.set_u32(0x2200, 0x2300);
        e.register(0x0047_0470, |_, a| {
            assert_eq!(a, &[0x2308]);
            Ret::default()
        });
        e.register(0x0040_1030, |_, a| {
            assert_eq!(a, &[0x2300]);
            Ret::default()
        });
        e.register(0x0063_f7b0, |_, a| {
            assert_eq!(a, &[0x10a4]);
            Ret::default()
        });
        e.register(0x0042_26e0, |_, a| {
            assert_eq!(a, &[0x2b00]);
            true.into_ret()
        });
        e.register(0x00ad_8ce0, |_, a| {
            assert!(a == [0x10ac] || a == [0x10b8]);
            false.into_ret()
        });
        e.register(0x0042_f780, |_, a| {
            assert_eq!(a, &[0x10d0, 0]);
            Ret::default()
        });
        e.register(0x6000_0003, |_, a| {
            assert_eq!(a, &[0x2500, 1]);
            Ret::default()
        });
        e.register(0x0040_8d60, |_, a| {
            assert_eq!(a, &[0x011c_7ae8]);
            0x2600u32.into_ret()
        });
        e.register(0x007b_52d0, |_, a| {
            assert_eq!(a, &[0x011c_7b8c, 0x2700]);
            0u32.into_ret()
        });
        e.register(0x0063_17a0, |_, a| {
            assert_eq!(a, &[0x011c_7b8c, 0x2700]);
            0x2800u32.into_ret()
        });
        e.register(0x0049_f590, |e, a| {
            assert_eq!(a[0], 0x011c_7b8c);
            assert_eq!(e.mem.u32(a[1]), 0x2700);
            Ret::default()
        });
        e.register(0x009d_9f20, |_, a| {
            assert_eq!(a, &[0x2100]);
            0x2900u32.into_ret()
        });
        e.register(0x00c5_b370, |_, a| {
            assert_eq!(a, &[0x2900, 0x2000]);
            Ret::default()
        });
        for address in [0x0042_f760, 0x0045_cec0, 0x0048_3710, 0x0046_ffb0] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0092_ec40, |_, a| {
            assert_eq!(a, &[0x1000]);
            Ret::default()
        });

        fn_009ac5a0(&mut e, this);

        assert_eq!(e.mem.u32(0x1000), 0x0108_ee04);
        assert_eq!(e.mem.u32(0x1018), 0x0108_edf8);
        assert!(e.call_log.unwrap().iter().any(|call| call.0 == 0x0049_f590));
    }

    #[test]
    fn explosion_factory_skips_creation_for_null_definition() {
        let mut e = list_engine();
        e.map(0x011d_e000, 0x1000);
        e.register(0x0064_4930, |_, a| {
            assert_eq!(a, &[0]);
            8.0f32.into_ret()
        });
        e.register(0x0040_7880, |_, a| {
            assert_eq!(a, &[0, 8.0f32.to_bits()]);
            Ret::default()
        });

        let result = fn_009ac9c0(
            &mut e,
            Ptr::new(0),
            Ptr::new(0),
            Ptr::new(0),
            Ptr::new(0),
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
        );
        assert!(result.is_null());
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (0x0064_4930, vec![0]),
                (0x0040_7880, vec![0, 8.0f32.to_bits()]),
            ]
        );
    }

    #[test]
    fn explosion_factory_skips_perk_and_construction_after_null_allocation() {
        let mut e = list_engine();
        e.map(0x011d_e000, 0x1000);
        e.set_global(0x011d_ea3c, 0x3000u32);
        e.mem.set_u32(0x2000, 0x4000);
        e.mem.set_u32(0x4224, 0x6000_0224);
        e.register(0x6000_0224, |_, a| {
            assert_eq!(a, &[0x2000]);
            false.into_ret()
        });
        e.register(0x0040_1000, |_, a| {
            assert_eq!(a, &[0x104]);
            0u32.into_ret()
        });
        e.register(0x0064_4930, |_, a| {
            assert_eq!(a, &[0]);
            8.0f32.into_ret()
        });
        e.register(0x0040_7880, |_, a| {
            assert_eq!(a, &[0, 8.0f32.to_bits()]);
            Ret::default()
        });

        let result = fn_009ac9c0(
            &mut e,
            Ptr::new(0x5000),
            Ptr::new(0x2000),
            Ptr::new(0),
            Ptr::new(0x3000),
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
        );
        assert!(result.is_null());
        assert!(!e
            .call_log
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0x005e_58f0 || *address == 0x009a_c200));
    }

    #[test]
    fn body_part_insertion_allocates_for_empty_or_single_matching_sample() {
        for existing in [false, true] {
            let mut e = list_engine();
            if existing {
                e.mem.set_u32(0x2008, 0x2100);
                e.mem.set_i32(0x2100, 7);
                e.mem.set_f32(0x2104, 2.0);
            }
            fn_009b1630(
                &mut e,
                Ptr::new(0x1000),
                Ptr::new(0x2000),
                7,
                f32::from_bits(0x7f80_0001),
            );
            assert_eq!(e.mem.i32(0x3000), 7);
            assert_eq!(e.mem.u32(0x3004), 0x7fc0_0001);
            assert_eq!(e.mem.u32(0x1f00), 0x2008);
            if existing {
                assert_eq!(e.mem.f32(0x2104), 2.0);
            }
        }
    }

    #[test]
    fn body_part_replacement_uses_minimum_and_stops_on_selected_match() {
        for second_distance in [1.0f32, 8.0, f32::NAN] {
            let mut e = list_engine();
            e.mem.set_u32(0x2008, 0x2100);
            e.mem.set_u32(0x200c, 0x2200);
            e.mem.set_u32(0x2200, 0x2300);
            e.mem.set_i32(0x2100, 1);
            e.mem.set_f32(0x2104, 4.0);
            e.mem.set_i32(0x2300, 2);
            e.mem.set_f32(0x2304, second_distance);
            fn_009b1630(&mut e, Ptr::new(0x1000), Ptr::new(0x2000), 9, 0.5);
            let chosen = if second_distance == 1.0 {
                0x2300
            } else {
                0x2100
            };
            assert_eq!(e.mem.i32(chosen), 9);
            assert_eq!(e.mem.f32(chosen + 4), 0.5);
            assert_eq!(e.mem.u32(0x1f04), 0);
        }
    }

    #[test]
    fn body_part_sampling_checks_actor_minimum_and_excluded_parts() {
        for sample_count in [0, 1, 2] {
            for distance in [1.0f32, 3.0, f32::NAN] {
                for part in [-1, 14, 7] {
                    let mut e = list_engine();
                    e.mem.set_u32(0x2000, 0x4000);
                    e.mem.set_u32(0x4000, 0x5000);
                    e.mem.set_u32(0x5100, 0x6000_0100);
                    e.register(0x6000_0100, |_, _| true.into_ret());
                    e.mem.set_u32(0x6000, 0x5000);
                    e.mem.set_u32(0x500c, 0x6000_000c);
                    e.register(0x6000_000c, |_, a| {
                        assert_eq!(a, &[0x6000]);
                        42u32.into_ret()
                    });
                    e.register_double(0x008b_3ef0, move |_, a| {
                        assert_eq!(a, &[0x4000, 42]);
                        part.into_ret()
                    });
                    if sample_count > 0 {
                        e.mem.set_u32(0x2008, 0x2100);
                        e.mem.set_i32(0x2100, 1);
                        e.mem.set_f32(0x2104, 2.0);
                    }
                    if sample_count > 1 {
                        e.mem.set_u32(0x200c, 0x2200);
                        e.mem.set_u32(0x2200, 0x2300);
                        e.mem.set_i32(0x2300, 2);
                        e.mem.set_f32(0x2304, 4.0);
                    }
                    fn_009b1510(
                        &mut e,
                        Ptr::new(0x1000),
                        Ptr::new(0x2000),
                        Ptr::new(0x6000),
                        distance,
                    );
                    let allowed = sample_count < 2 || distance < 2.0;
                    assert_eq!(
                        e.call_log
                            .as_ref()
                            .unwrap()
                            .iter()
                            .any(|c| c.0 == 0x008b_3ef0),
                        allowed
                    );
                    if allowed && part == 7 {
                        assert_eq!(e.mem.i32(if sample_count < 2 { 0x3000 } else { 0x2100 }), 7);
                    } else {
                        assert_eq!(e.mem.u32(0x1f04), 0);
                        assert_eq!(e.mem.i32(0x2100), if sample_count > 0 { 1 } else { 0 });
                    }
                }
            }
        }
        let mut e = list_engine();
        fn_009b1510(&mut e, Ptr::new(0), Ptr::new(0), Ptr::new(0), 0.0);
        fn_009b1510(&mut e, Ptr::new(0), Ptr::new(0x2000), Ptr::new(0), 0.0);
        e.mem.set_u32(0x2000, 0x4000);
        e.mem.set_u32(0x4000, 0x5000);
        e.mem.set_u32(0x5100, 0x6000_0100);
        e.register(0x6000_0100, |_, _| false.into_ret());
        fn_009b1510(&mut e, Ptr::new(0), Ptr::new(0x2000), Ptr::new(0), 0.0);
    }

    #[test]
    fn target_body_list_lookup_finds_match_after_null_and_returns_null_for_missing() {
        let mut e = list_engine();
        e.mem.set_u32(0x10a8, 0x1200);
        e.mem.set_u32(0x1200, 0x2000);
        e.mem.set_u32(0x2000, 0x9000);
        assert_eq!(
            fn_009b1720(&mut e, Ptr::new(0x1000), Ptr::new(0x9000)).addr(),
            0x2008
        );
        assert!(fn_009b1720(&mut e, Ptr::new(0x1000), Ptr::new(0x9001)).is_null());
        assert!(fn_009b1720(&mut e, Ptr::new(0x1300), Ptr::new(0x9000)).is_null());
    }

    #[test]
    fn update_record_obeys_both_gates_and_passes_pending_argument_to_update() {
        for cell in [0u32, 0x7000] {
            for object in [0u32, 0x2000] {
                let mut e = list_engine();
                e.mem.set_u32(0x1000, 0x4000);
                e.mem.set_u32(0x41d0, 0x6000_01d0);
                e.register_double(0x008d_6f30, move |_, _| cell.into_ret());
                e.register_double(0x6000_01d0, move |_, a| {
                    assert_eq!(a, &[0x1000]);
                    object.into_ret()
                });
                e.set_global(0x011c_3c08, 1.25f32);
                e.register(0x0043_d410, |e, a| {
                    assert_eq!(&a[1..], &[1.25f32.to_bits(), 1, 0]);
                    e.mem.set_u32(a[0], 123);
                    Ret::default()
                });
                e.register(0x00a5_9c60, |e, a| {
                    assert_eq!(a[0], 0x2000);
                    assert_eq!(e.mem.u32(a[1]), 123);
                    Ret::default()
                });
                e.call(0x009b_17a0, &args![0x1000u32, 0xfeedu32]);
                assert_eq!(
                    e.call_log.unwrap().iter().any(|c| c.0 == 0x00a5_9c60),
                    cell == 0 && object != 0
                );
            }
        }
    }

    #[test]
    fn load_forwarder_preserves_this_and_value() {
        let mut e = engine_with(&[0x0093_2d60]);
        fn_009b2770(&mut e, Ptr::new(4), 7);
        assert_eq!(e.call_log.unwrap()[0], (0x0093_2d60, vec![4, 7]));
    }

    #[test]
    fn transient_reset_removes_null_and_nonnull_entries_and_resets_vectors() {
        let mut e = list_engine();
        e.register(0x0093_2f60, |_, a| {
            assert_eq!(a, &[0x1000, 7]);
            Ret::default()
        });
        e.register(0x0042_f780, |_, a| {
            assert_eq!(a, &[0x10d0, 0]);
            Ret::default()
        });
        e.register(0x009a_c940, |_, a| {
            assert_eq!(a, &[0x2000]);
            Ret::default()
        });
        e.register(0x0040_1030, |_, a| {
            assert_eq!(a, &[0x2000]);
            Ret::default()
        });
        let mut calls = 0;
        e.register_double(0x0082_56d0, move |_, a| {
            assert_eq!(a, &[0x10a4]);
            calls += 1;
            (calls > 2).into_ret()
        });
        let mut payloads = 0;
        e.register_double(0x0068_15c0, move |e, _| {
            payloads += 1;
            e.mem
                .set_u32(0x1200, if payloads == 1 { 0 } else { 0x2000 });
            0x1200u32.into_ret()
        });
        e.register(0x0063_f7b0, |_, a| {
            assert_eq!(a, &[0x10a4]);
            Ret::default()
        });
        for offset in 0xd4..=0xd9 {
            e.mem.set_u8(0x1000 + offset, 0xff);
        }
        for i in 0..3 {
            e.set_global(0x011f_426c + i * 4, 90 + i);
        }
        fn_009b2790(&mut e, Ptr::new(0x1000), 7);
        for offset in 0xd4..=0xd9 {
            assert_eq!(e.mem.u8(0x1000 + offset), 0);
        }
        assert_eq!(e.mem.f32(0x109c), 1.0);
        for base in [0x10e8, 0x10f4] {
            for i in 0..3 {
                assert_eq!(e.mem.u32(base + i * 4), 90 + i);
            }
        }
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .filter(|c| c.0 == 0x0063_f7b0)
                .count(),
            2
        );
    }

    #[test]
    fn reference_invalidation_clears_only_matching_fields_and_all_matching_targets() {
        for owner in [0x9000u32, 0x9001] {
            for exploded in [0x9000u32, 0x9001] {
                let mut e = list_engine();
                e.mem.set_u32(0x10c8, owner);
                e.mem.set_u32(0x10cc, exploded);
                e.register(0x0040_30d0, |_, a| {
                    assert_eq!(a, &[0x1000, 0]);
                    Ret::default()
                });
                e.register(0x0056_c7d0, |_, a| {
                    assert_eq!(a, &[0x1000, 0]);
                    Ret::default()
                });
                for (node, next, record, reference) in [
                    (0x10a4, 0x1200, 0x2000, 0x9000),
                    (0x1200, 0x1300, 0x2100, 0x9001),
                    (0x1300, 0, 0x2200, 0x9000),
                ] {
                    e.mem.set_u32(node, record);
                    e.mem.set_u32(node + 4, next);
                    e.mem.set_u32(record, reference);
                }
                fn_009b28a0(&mut e, Ptr::new(0x1000), Ptr::new(0x9000));
                for record in [0x2000, 0x2200] {
                    assert_eq!(e.mem.u32(record), 0);
                    assert_eq!(e.mem.u8(record + 6), 1);
                }
                assert_eq!(e.mem.u32(0x2100), 0x9001);
                assert_eq!(e.mem.u8(0x2106), 0);
                let calls = e.call_log.unwrap();
                assert_eq!(calls.iter().any(|c| c.0 == 0x0040_30d0), owner == 0x9000);
                assert_eq!(calls.iter().any(|c| c.0 == 0x0056_c7d0), exploded == 0x9000);
            }
        }
    }

    #[test]
    fn signed_byte_pointer_adjustment_covers_both_signs() {
        let mut e = list_engine();
        for value in [0u8, 1, 127, 128, 255] {
            e.mem.set_u8(0x1005, value);
            assert_eq!(
                fn_009b2c90(&mut e, Ptr::new(0x1000)).addr(),
                0x1000u32.wrapping_add(value as i8 as i32 as u32)
            );
        }
    }

    fn allocator_doubles(e: &mut Engine) {
        e.register(0x00c8_5750, |_, a| {
            assert!(a.is_empty());
            0x5000u32.into_ret()
        });
        e.register(0x0068_15c0, |_, a| {
            assert_eq!(a, &[0x5000]);
            0x5004u32.into_ret()
        });
        e.register(0x005e_0ad0, |_, a| {
            assert_eq!(a[0], 0x5004);
            0x6000u32.into_ret()
        });
    }

    #[test]
    fn contact_allocator_rounds_with_wrapping_and_preserves_flags() {
        let mut e = list_engine();
        allocator_doubles(&mut e);
        for count in [0u32, 1, 2, 8, u32::MAX] {
            assert_eq!(fn_009b2e60(&mut e, count, 0x80).addr(), 0x6000);
            assert_eq!(
                e.call_log.as_ref().unwrap().last().unwrap().1,
                vec![
                    0x5004,
                    count.wrapping_mul(0x70).wrapping_add(0x7f) & 0xffff_ff80,
                    0x80
                ]
            );
        }
    }

    #[test]
    fn contact_array_constructor_marks_capacity_and_preserves_base_count() {
        let mut e = list_engine();
        allocator_doubles(&mut e);
        e.register(0x0062_99a0, |e, a| {
            e.mem.set_u32(a[0] + 4, 17);
            Ret::default()
        });
        assert_eq!(fn_009b2cb0(&mut e, Ptr::new(0x1000), 3).addr(), 0x1000);
        assert_eq!(e.mem.u32(0x1000), 0x6000);
        assert_eq!(e.mem.u32(0x1004), 17);
        assert_eq!(e.mem.u32(0x1008), 0x8000_0003);
        assert_eq!(e.mem.u32(0x100c), 0x6000);
    }

    fn boolean_doubles(e: &mut Engine) {
        e.register(0x0062_2570, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            a[0].into_ret()
        });
        e.register(0x0056_09f0, |e, a| e.mem.u8(a[0]).into_ret());
    }

    #[test]
    fn storage_comparison_constructs_result_for_equal_and_changed_storage() {
        let mut e = list_engine();
        boolean_doubles(&mut e);
        for original in [0x2000u32, 0x3000] {
            e.mem.set_u32(0x1000, 0x2000);
            e.mem.set_u32(0x100c, original);
            assert_eq!(
                fn_009b2dd0(&mut e, Ptr::new(0x1000), Ptr::new(0x1200)).addr(),
                0x1200
            );
            assert_eq!(e.mem.u8(0x1200), u8::from(original != 0x2000));
        }
    }

    #[test]
    fn array_destructor_clears_equal_storage_and_always_releases_original_then_base() {
        for original in [0x2000u32, 0x3000] {
            let mut e = list_engine();
            boolean_doubles(&mut e);
            for address in [0x0062_6870, 0x005e_0b60, 0x0062_1b60] {
                e.register(address, |_, _| Ret::default());
            }
            e.mem.set_u32(0x1000, 0x2000);
            e.mem.set_u32(0x100c, original);
            fn_009b2d30(&mut e, Ptr::new(0x1000));
            let calls = e.call_log.unwrap();
            assert_eq!(calls.iter().any(|c| c.0 == 0x0062_6870), original == 0x2000);
            assert_eq!(
                &calls[calls.len() - 2..],
                &[(0x005e_0b60, vec![original]), (0x0062_1b60, vec![0x1000])]
            );
        }
    }

    #[test]
    fn array_base_destructor_forwarder_preserves_this() {
        let mut e = engine_with(&[0x0057_c530]);
        fn_009b2db0(&mut e, Ptr::new(0x1000));
        assert_eq!(e.call_log.unwrap()[0].1, vec![0x1000]);
    }

    #[test]
    fn embedded_array_initializer_passes_storage_count_and_capacity() {
        let mut e = engine_with(&[0x0062_e230]);
        assert_eq!(fn_009b2e30(&mut e, Ptr::new(0x1000), 19).addr(), 0x1000);
        assert_eq!(e.call_log.unwrap()[0].1, vec![0x1000, 0x100c, 19, 0x80]);
    }

    fn sorting_doubles(e: &mut Engine) {
        e.register(0x0062_1c40, |_, _| Ret::default());
        e.register(0x0062_12c0, |e, a| {
            for offset in (0..0x70).step_by(4) {
                let bits = e.mem.u32(a[1] + offset);
                e.mem.set_u32(a[0] + offset, bits);
            }
            a[0].into_ret()
        });
        e.register(0x009b_30b0, |e, a| {
            let less = e.mem.f32(a[0]) < e.mem.f32(a[2]);
            e.mem.set_u8(a[1], u8::from(less));
            a[1].into_ret()
        });
        e.register(0x0056_09f0, |e, a| e.mem.u8(a[0]).into_ret());
    }

    #[test]
    fn contact_comparator_wrapper_passes_left_result_right_and_uses_low_byte() {
        let mut e = list_engine();
        e.register(0x009b_30b0, |_, a| {
            assert_eq!(a[0], 0x2000);
            assert_eq!(a[2], 0x3000);
            a[1].into_ret()
        });
        for raw in [0x100u32, 0x101] {
            e.register_double(0x0056_09f0, move |_, _| raw.into_ret());
            assert_eq!(
                fn_009b3080(&mut e, Ptr::new(0), Ptr::new(0x2000), Ptr::new(0x3000)),
                raw == 0x101
            );
        }
    }

    #[test]
    fn contact_sort_preserves_full_records_for_reversed_duplicate_and_nan_inputs() {
        for input in [
            vec![5.0f32, 4.0, 3.0, 2.0, 1.0],
            vec![2.0, 1.0, 2.0, 1.0, 3.0],
            vec![f32::NAN, 3.0, 1.0],
            vec![1.0],
        ] {
            let mut e = list_engine();
            sorting_doubles(&mut e);
            for (index, value) in input.iter().enumerate() {
                let record = 0x2000 + index as u32 * 0x70;
                e.mem.set_f32(record, *value);
                for offset in (4..0x70).step_by(4) {
                    e.mem.set_u32(record + offset, index as u32);
                }
            }
            fn_009b2ed0(&mut e, Ptr::new(0x2000), 0, input.len() as i32 - 1, 0x8a);
            let mut identities = vec![];
            let mut output = vec![];
            for index in 0..input.len() {
                let record = 0x2000 + index as u32 * 0x70;
                let identity = e.mem.u32(record + 4);
                identities.push(identity);
                output.push(e.mem.f32(record));
                assert_eq!(e.mem.u32(record), input[identity as usize].to_bits());
                for offset in (4..0x70).step_by(4) {
                    assert_eq!(e.mem.u32(record + offset), identity);
                }
            }
            identities.sort();
            assert_eq!(identities, (0..input.len() as u32).collect::<Vec<_>>());
            if input.iter().all(|v| !v.is_nan()) {
                assert!(output.windows(2).all(|w| w[0] <= w[1]));
            }
        }
    }

    #[test]
    fn sort_entry_points_skip_signed_small_counts_and_sort_nonempty_ranges() {
        for count in [-1, 0, 1] {
            let mut e = Engine::new();
            e.call_log = Some(vec![]);
            fn_009b2e90(&mut e, Ptr::new(0), count, 9);
            fn_009b2e00(&mut e, Ptr::new(0), count);
            assert!(e.call_log.unwrap().is_empty());
        }
        for explicit_state in [false, true] {
            let mut e = list_engine();
            sorting_doubles(&mut e);
            e.mem.set_f32(0x2000, 3.0);
            e.mem.set_f32(0x2070, 1.0);
            if explicit_state {
                fn_009b2e90(&mut e, Ptr::new(0x2000), 2, 8);
            } else {
                fn_009b2e00(&mut e, Ptr::new(0x2000), 2);
            }
            assert_eq!(e.mem.f32(0x2000), 1.0);
            assert_eq!(e.mem.f32(0x2070), 3.0);
        }
    }

    #[test]
    fn contact_sort_matches_native_generated_record_permutations() {
        // nv-call, FalloutNV.exe 1.4.0.525, SHA256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f.
        // AMD Ryzen AI 9 HX 370; FCW 027F, MXCSR 1F80. Five fault-free
        // generated vectors; no executable bytes or raw recordings embedded.
        let cases: &[(&[f32], &[u32])] = &[
            (&[5.0, 4.0, 3.0, 2.0, 1.0], &[5, 4, 3, 2, 1]),
            (&[2.0, 1.0, 2.0, 1.0, 3.0], &[2, 4, 3, 1, 5]),
            (&[f32::NAN, 3.0, 1.0], &[3, 2, 1]),
            (
                &[0.0, -0.0, f32::INFINITY, f32::NEG_INFINITY],
                &[4, 2, 1, 3],
            ),
            (&[1.0], &[1]),
        ];
        for &(values, expected) in cases {
            let mut e = list_engine();
            sorting_doubles(&mut e);
            // 009b30b0 compares +0x1c via 009b3100/00560d30.
            e.register(0x009b_30b0, |e, a| {
                let less = e.mem.f32(a[0] + 0x1c) < e.mem.f32(a[2] + 0x1c);
                e.mem.set_u8(a[1], u8::from(less));
                a[1].into_ret()
            });
            for (index, value) in values.iter().enumerate() {
                let record = 0x2000 + index as u32 * 0x70;
                e.mem.set_f32(record + 0x1c, *value);
                e.mem.set_u32(record + 0x20, index as u32 + 1);
            }
            fn_009b2ed0(&mut e, Ptr::new(0x2000), 0, values.len() as i32 - 1, 0x8a);
            for (index, &identity) in expected.iter().enumerate() {
                let record = 0x2000 + index as u32 * 0x70;
                assert_eq!(e.mem.u32(record + 0x20), identity);
                assert_eq!(
                    e.mem.u32(record + 0x1c),
                    values[identity as usize - 1].to_bits()
                );
                for offset in (0..0x70).step_by(4) {
                    if offset != 0x1c && offset != 0x20 {
                        assert_eq!(e.mem.u32(record + offset), 0);
                    }
                }
            }
        }
    }

    #[test]
    fn target_allocation_failure_keeps_native_null_dereference() {
        let mut e = list_engine();
        e.register(0x0040_1000, |_, _| 0u32.into_ret());
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            fn_009b0e00(&mut e, Ptr::new(0x1000), Ptr::new(0x9000), 1, 1)
        }));
        assert!(result.is_err());
        assert!(!e.call_log.unwrap().iter().any(|c| c.0 == 0x005a_e3d0));
    }

    #[test]
    fn selected_second_body_part_match_stops_before_third_payload() {
        let mut e = list_engine();
        for (node, next, record, part, distance) in [
            (0x2008, 0x2200, 0x2100, 1, 4.0),
            (0x2200, 0x2400, 0x2300, 7, 2.0),
            (0x2400, 0, 0x2500, 8, 1.0),
        ] {
            e.mem.set_u32(node, record);
            e.mem.set_u32(node + 4, next);
            e.mem.set_i32(record, part);
            e.mem.set_f32(record + 4, distance);
        }
        fn_009b1630(&mut e, Ptr::new(0x1000), Ptr::new(0x2000), 7, 0.5);
        assert_eq!(e.mem.f32(0x2304), 0.5);
        assert_eq!(e.mem.f32(0x2504), 1.0);
        assert!(!e
            .call_log
            .unwrap()
            .iter()
            .any(|c| c.0 == 0x0068_15c0 && c.1 == [0x2400]));
    }

    #[test]
    fn parameterized_constructor_quiets_float_stores_but_preserves_copied_nan_words() {
        let mut e = engine_with(&[
            0x0092_eb50,
            0x0063_3c90,
            0x0096_a2d0,
            0x0041_a250,
            0x0042_f730,
            0x0068_15c0,
            0x0057_5830,
            0x0057_5690,
            0x0066_b0d0,
            0x0042_f780,
            0x0056_fa90,
        ]);
        e.map(0x1000, 0x5000);
        map_generated_constant_pages(&mut e);
        e.register(0x0093_0700, |_, a| {
            assert_eq!(a, &[0x1000, 0x7000]);
            Ret::default()
        });
        e.register(0x0041_81e0, |_, _| 0x3000u32.into_ret());
        e.register(0x0047_7900, |_, _| f32::from_bits(0x7f80_0001).into_ret());
        e.register(0x0047_7950, |_, a| {
            match a[1] {
                1 => 1u32,
                2 => 0x100,
                0x40 => 0x182,
                _ => panic!("mask"),
            }
            .into_ret()
        });
        e.register(0x0040_1000, |_, _| 0u32.into_ret());
        e.set_global(0x0101_7718, 0x7f80_0001u32);
        e.set_global(0x0101_2054, 0xff80_0001u32);
        e.mem.set_u32(0x308c, 0x7f80_0002);
        for i in 0..3 {
            e.set_global(0x011f_426c + i * 4, 0x7f80_0010u32 + i);
        }
        e.call(
            0x009a_c200,
            &[
                0x1000,
                0x3000,
                0,
                0x7000,
                0x7f80_0001,
                0,
                0,
                0x7f80_0002,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        );
        assert_eq!(e.mem.u32(0x108c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(0x1094), 0x7fc0_0001);
        assert_eq!(e.mem.u32(0x1098), 0x7fc0_0002);
        assert_eq!(e.mem.u32(0x1100), 0xffc0_0001);
        assert_eq!(e.mem.u8(0x10d8), 0x82);
        assert_eq!(e.mem.u32(0x1088), 0);
        assert_eq!(e.mem.u32(0x1090), 0);
        for base in [0x10e8, 0x10f4] {
            for i in 0..3 {
                assert_eq!(e.mem.u32(base + i * 4), 0x7f80_0010 + i);
            }
        }
        let rotations: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == 0x0056_fa90)
            .collect();
        assert_eq!(rotations.len(), 1);
        assert_eq!(rotations[0].1[1], 0x7f80_0002);
    }
}
