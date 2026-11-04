//! `fallout/ai/run_009ba550` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two groups of functions:
//!
//! - `hkConstraintCinfo` and `hkHingeConstraintCinfo` (0x14-byte Havok
//!   construction info structs): constructors and destructors. Identity: the
//!   vtables at `0108ffb4` and `0108ff98` carry the RTTI names
//!   `.?AUhkConstraintCinfo@@` and `.?AUhkHingeConstraintCinfo@@`, and
//!   `009ba5c0` / `009ba630` are their slot 0 (scalar deleting destructors).
//! - `MissileProjectile` methods (the vtable at `0108fa44` has the RTTI name
//!   `.?AVMissileProjectile@@`): the save/load/revert slots, a predicate
//!   slot, the impact handling (`009babe0`, `009ba660`) and the
//!   `MuzzleFlash` constructor `009bacb0` (the Xbox PDB `MuzzleFlash` is 0x20
//!   bytes with the fields at the offsets the constructor writes).
//!
//! `MissileProjectile` fields at PC offsets 0x150/0x154/0x158 correspond, by
//! position and type, to the Xbox PDB fields `eImpactResult`,
//! `bWaitingToInitialize3D` and `fStickTime` (Xbox offsets 0x180, 0x184,
//! 0x188); their meaning is not confirmed beyond that.
//!
//! Functions that open a C++ exception frame (`FS:[0]`) carry a
//! `Translation incomplete` marker: the unwinding states are not translated.
//! The x87 loads of float globals are `FLD`/`FSTP` copies, which quiet a
//! signalling NaN (`x87_quiet`).

#[allow(unused_imports)]
use crate::prelude::*;

/// Deallocator (cdecl `(block)`).
const FREE: u32 = 0x0040_1030;
/// Returns its `this` (an identity function the code uses to address a
/// member).
const SELF_GETTER: u32 = 0x0068_15c0;
/// `hkConstraintCinfo::SetData`-named function (map name; `this`, pointer):
/// the destructor calls it with null.
const CONSTRAINT_SET_DATA: u32 = 0x00c8_9ac0;
/// `NiPointer` constructor `(this, pointer)` and assignment `(this, pointer)`.
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_SET: u32 = 0x0066_b0d0;
/// Reads the word at +4 of a `BSSimpleList` node (`m_pkNext`).
const LIST_NEXT: u32 = 0x0072_6070;
/// `BSSimpleList::IsEmpty` (`m_item` and `m_pkNext` both null).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;

/// `hkConstraintCinfo` vtable (RTTI name confirmed).
const CONSTRAINT_CINFO_VTABLE: u32 = 0x0108_ffb4;
/// `hkHingeConstraintCinfo` vtable (RTTI name confirmed).
const HINGE_CONSTRAINT_CINFO_VTABLE: u32 = 0x0108_ff98;

/// Function-local static float of `009ba660` and its guard word (bit 0 set
/// once initialized).
const STICK_VALUE: u32 = 0x011f_2020;
const STICK_VALUE_GUARD: u32 = 0x011f_2024;
/// Global object whose `00403e20` accessor supplies the static's value.
const STICK_VALUE_SOURCE: u32 = 0x011c_fc8c;
/// Float global stored in `fStickTime`.
const STICK_TIME: u32 = 0x0101_7b78;
/// A global object whose method `007af430` answers with a reference and
/// whose `009c8c40(0)` the impact code calls when that reference is the
/// projectile itself.
const REFERENCE_HOLDER: u32 = 0x011f_2250;
/// A global (`0x011ddf38`) passed to `0042ce10`.
const IMPACT_QUERY_OBJECT: u32 = 0x011d_df38;
/// The process lists object.
const PROCESS_LISTS: u32 = 0x011e_0e80;

layout! {
    /// `hkConstraintCinfo` (Xbox PDB), 0x14 bytes. `hkHingeConstraintCinfo`
    /// derives from it with the same size and fields.
    pub struct HkConstraintCinfo: 0x14 {
        /// Virtual function table pointer.
        0x00 vfptr: u32,
        /// `pData` (Xbox PDB): `hkpConstraintData*`.
        0x04 pData: Ptr,
        /// `ePriority` (Xbox PDB).
        0x08 ePriority: u32,
        /// `pEntityA` (Xbox PDB): `hkpEntity*`.
        0x0c pEntityA: Ptr,
        /// `pEntityB` (Xbox PDB): `hkpEntity*`.
        0x10 pEntityB: Ptr,
    }

    /// `MissileProjectile` (Xbox PDB), PC offsets. Only the members the
    /// functions of this unit use; the size covers them.
    pub struct MissileProjectile: 0x15c {
        /// `ImpactList` (Xbox PDB `BSSimpleList<Projectile::ImpactData *>`,
        /// at +0x88 on the PC): `m_item`, the first `ImpactData*`.
        0x88 ImpactListItem: Ptr,
        /// The impact result (see the module note), 4 bytes at +0x150.
        0x150 eImpactResult: u32,
        /// `bWaitingToInitialize3D` (see the module note).
        0x154 bWaitingToInitialize3D: bool,
        /// `fStickTime` (see the module note).
        0x158 fStickTime: f32,
    }

    /// `MuzzleFlash` (Xbox PDB), 0x20 bytes; the PC constructor writes the
    /// same offsets.
    pub struct MuzzleFlash: 0x20 {
        /// `bEnabled` (Xbox PDB).
        0x00 bEnabled: bool,
        /// `bMPSEnabled` (Xbox PDB).
        0x01 bMPSEnabled: bool,
        /// `bUpdateLight` (Xbox PDB).
        0x02 bUpdateLight: bool,
        /// `fEnableTimer` (Xbox PDB).
        0x04 fEnableTimer: f32,
        /// `fDurationTimer` (Xbox PDB).
        0x08 fDurationTimer: f32,
        /// `spAVObject` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x0c spAVObject: Ptr,
        /// `spLight` (Xbox PDB): `NiPointer<NiLight>`.
        0x10 spLight: Ptr,
        /// `pProjectile` (Xbox PDB): `BGSProjectile*`.
        0x14 pProjectile: Ptr,
        /// `pWeaponSource` (Xbox PDB): `TESObjectWEAP*`.
        0x18 pWeaponSource: Ptr,
        /// `pParentReference` (Xbox PDB): `TESObjectREFR*`.
        0x1c pParentReference: Ptr,
    }
}

// Translated from 009ba550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkHingeConstraintCinfo` constructor: the base constructor, then the
/// hinge vtable. Returns `this`.
pub fn hk_hinge_constraint_cinfo_construct(
    e: &mut Engine,
    this: Ptr<HkConstraintCinfo>,
) -> Ptr<HkConstraintCinfo> {
    hk_constraint_cinfo_construct(e, this);
    e.set(
        this,
        HkConstraintCinfo::vfptr,
        HINGE_CONSTRAINT_CINFO_VTABLE,
    );
    this
}

// Translated from 009ba570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkConstraintCinfo` constructor: constructs the base part at +4
/// (`006815c0`), sets the vtable, `pData` null, `ePriority` 1 and both
/// entities null. Returns `this`.
pub fn hk_constraint_cinfo_construct(
    e: &mut Engine,
    this: Ptr<HkConstraintCinfo>,
) -> Ptr<HkConstraintCinfo> {
    e.call(SELF_GETTER, &args![this.addr() + 4]);
    e.set(this, HkConstraintCinfo::vfptr, CONSTRAINT_CINFO_VTABLE);
    e.set(this, HkConstraintCinfo::pData, Ptr::new(0));
    e.set(this, HkConstraintCinfo::ePriority, 1);
    e.set(this, HkConstraintCinfo::pEntityA, Ptr::new(0));
    e.set(this, HkConstraintCinfo::pEntityB, Ptr::new(0));
    this
}

// Translated from 009ba5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkConstraintCinfo` scalar deleting destructor (vtable slot 0): runs the
/// destructor and frees the block when bit 0 of `flags` is set. Returns
/// `this`.
pub fn hk_constraint_cinfo_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HkConstraintCinfo>,
    flags: u32,
) -> Ptr<HkConstraintCinfo> {
    hk_constraint_cinfo_destruct(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 009ba5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkConstraintCinfo` destructor body: restores the base vtable and clears
/// the constraint data (`00c89ac0(this, 0)`).
pub fn hk_constraint_cinfo_destruct(e: &mut Engine, this: Ptr<HkConstraintCinfo>) {
    e.set(this, HkConstraintCinfo::vfptr, CONSTRAINT_CINFO_VTABLE);
    e.call(CONSTRAINT_SET_DATA, &args![this, 0u32]);
}

// Translated from 009ba610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkHingeConstraintCinfo` destructor: runs the base destructor body.
pub fn hk_hinge_constraint_cinfo_destruct(e: &mut Engine, this: Ptr<HkConstraintCinfo>) {
    hk_constraint_cinfo_destruct(e, this);
}

// Translated from 009ba630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkHingeConstraintCinfo` scalar deleting destructor (vtable slot 0): runs
/// the destructor and frees the block when bit 0 of `flags` is set. Returns
/// `this`.
pub fn hk_hinge_constraint_cinfo_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HkConstraintCinfo>,
    flags: u32,
) -> Ptr<HkConstraintCinfo> {
    hk_hinge_constraint_cinfo_destruct(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 009ba660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The impact handling of a missile that has hit something. Does nothing
/// when virtual slot +0x1D0 answers null, when the list at +0x88 is empty
/// (`008256d0`), or when the first impact has no world object (+0x1C).
/// Otherwise it first sets `fStickTime` from a float global (and releases
/// `011f2250`'s reference when it is this projectile), then, depending on the
/// impact's target, either calls `009b8c90` (target missing, type 0x13, or a
/// collidee for which virtual slot +0x100 answers true), or places the
/// projectile on the impact: marks flags (`005de0a0(4, 1)`), calls virtual
/// slot +0x2A4, clears a flag word on the colliding object, repositions the
/// reference (unless `0042ce10` is true), syncs the 3D (`009bf470(0, 0)`),
/// updates the 3D object's world position and sets its motion (5 when
/// `00517630` of the target is 5, otherwise 4, with the follow-offset
/// transform at +0x94 recomputed), and finally sets the motion of the
/// object returned by virtual slot +0x9C.
// Translation incomplete: 009ba660 - the C++ exception unwinding frame (FS:[0]) is not translated.
pub fn fn_009ba660(e: &mut Engine, this: Ptr<MissileProjectile>) {
    let this_addr = this.addr();
    if e.global::<u32>(STICK_VALUE_GUARD) & 1 == 0 {
        let guard = e.global::<u32>(STICK_VALUE_GUARD);
        e.set_global(STICK_VALUE_GUARD, guard | 1);
        let source = e.call(0x0040_3e20, &args![STICK_VALUE_SOURCE]).u32();
        let value = x87_quiet(e.mem.f32(source));
        e.set_global(STICK_VALUE, value);
    }
    let query_object = e.global::<u32>(IMPACT_QUERY_OBJECT);
    let skip_reposition = e.call(0x0042_ce10, &args![query_object]).bool();
    if e.vcall(this_addr, 0x1d0, &args![]).u32() == 0 {
        return;
    }
    let list = this_addr + 0x88;
    if e.call(LIST_IS_EMPTY, &args![list]).bool() {
        return;
    }
    if e.call(0x007a_f430, &args![REFERENCE_HOLDER]).u32() == this_addr {
        e.call(0x009c_8c40, &args![REFERENCE_HOLDER, 0u32]);
    }
    let stick_time = x87_quiet(e.global::<f32>(STICK_TIME));
    e.set(this, MissileProjectile::fStickTime, stick_time);

    let list_head = e.call(SELF_GETTER, &args![list]).u32();
    let impact = e.mem.u32(list_head);
    if impact == 0 || e.mem.u32(impact + 0x1c) == 0 {
        return;
    }
    let object_3d = e.vcall(this_addr, 0x1d0, &args![]).u32();
    let world_object = e.mem.u32(impact + 0x1c);
    let collidable = e.call(0x0046_0140, &args![world_object]).u32();
    let target_node = e.call(0x004b_5820, &args![collidable]).u32();
    let target = e.call(0x004b_5a80, &args![world_object]).u32();
    let target_type = if target == 0 {
        0
    } else {
        e.with_stack(4, |e, out| {
            let info = e.call(0x0043_b4f0, &args![target, out]).u32();
            e.call(0x0043_b4d0, &args![info]).u32()
        })
    };
    let collidee = e.mem.u32(impact);
    if target == 0
        || target_type == 0x13
        || (collidee != 0 && e.vcall(collidee, 0x100, &args![]).bool())
    {
        e.call(0x009b_8c90, &args![this]);
        return;
    }

    e.call(0x005d_e0a0, &args![this, 4u32, 1u32]);
    e.vcall(this_addr, 0x2a4, &args![]);
    let collision_object = e.call(0x004b_5260, &args![object_3d]).u32();
    let body = if collision_object == 0 {
        0
    } else {
        e.call(0x006f_a820, &args![collision_object]).u32()
    };
    if body != 0 {
        e.with_stack(4, |e, filter| {
            e.call(0x0043_b4f0, &args![body, filter]);
            e.call(0x004a_39f0, &args![filter, 0xfu32]);
            let value = e.mem.u32(filter.addr());
            e.call(0x0062_6cb0, &args![body, value]);
        });
    }
    if !skip_reposition {
        let static_value = x87_quiet(e.global::<f32>(STICK_VALUE));
        e.with_stack(12, |e, first| {
            e.with_stack(12, |e, second| {
                e.with_stack(12, |e, offset| {
                    let hit = e.call(0x009b_7010, &args![this, first]).u32();
                    let direction = e.call(0x0045_bb20, &args![hit, second, static_value]);
                    let location = e.call(0x0043_9e90, &args![impact + 4, offset, direction.u32()]);
                    e.call(0x0057_5830, &args![this, location.u32()]);
                });
            });
        });
    }
    e.call(0x009b_f470, &args![this, 0u32, 0u32]);
    e.call(0x00c6_9f50, &args![object_3d, 1u32, 1u32]);
    e.with_stack(12, |e, update| {
        e.call(0x0043_d410, &args![update, 0.0f32, 0u32, 0u32]);
        e.call(0x00a5_9c60, &args![object_3d, update]);
    });
    if e.call(0x0051_7630, &args![target]).u32() == 5 {
        e.call(0x00c6_a350, &args![object_3d, 5u32, 1u32, 0u32, 1u32]);
    } else {
        e.call(0x00c6_a350, &args![object_3d, 4u32, 1u32, 0u32, 1u32]);
        if !e.call(0x0044_4d00, &args![this, 0x80u32]).bool() {
            e.call(0x005d_e0a0, &args![this, 0x80u32, 1u32]);
            let holder = e.call(0x0046_10f0, &args![PROCESS_LISTS]).u32();
            if holder != 0 {
                e.with_stack(4, |e, reference| {
                    e.mem.set_u32(reference.addr(), this_addr);
                    e.call(0x005a_e3d0, &args![holder, reference]);
                });
            }
        }
        e.call(0x0062_8f30, &args![target, 0x100u32, 1u32]);
        if !skip_reposition && target_node != 0 {
            follow_offset_transform(e, this_addr, target_node, object_3d);
        }
    }
    let moved = if object_3d == 0 {
        0
    } else {
        let argument = e.call(0x009b_7cb0, &args![]).u32();
        e.vcall(object_3d, 0x9c, &args![argument]).u32()
    };
    if moved != 0 {
        e.call(0x00c6_a350, &args![moved, 1u32, 1u32, 1u32, 1u32]);
    }
}

/// `009ba660`'s follow-offset step: the inverse of the target node's
/// transform (`00461130`, `004b4880`) multiplied with the 3D object's
/// transform (`0062c250`), copied (13 words) to `kFollowOffset` at +0x94.
fn follow_offset_transform(e: &mut Engine, this: u32, target_node: u32, object_3d: u32) {
    e.with_stack(0x34, |e, inverse| {
        e.call(0x0047_6a80, &args![inverse]);
        let node_transform = e.call(0x0046_1130, &args![target_node]).u32();
        e.call(0x004b_4880, &args![node_transform, inverse]);
        let object_transform = e.call(0x0046_1130, &args![object_3d]).u32();
        e.with_stack(0x34, |e, product| {
            let result = e.call(0x0062_c250, &args![inverse, product, object_transform]);
            let result = result.u32();
            for word in 0..13 {
                let value = e.mem.u32(result + 4 * word);
                e.mem.set_u32(this + 0x94 + 4 * word, value);
            }
        });
    });
}

// Translated from 009baaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MissileProjectile::SaveGame` (Xbox PDB name at vtable slot +0x54): the
/// base save (`009c4ff0`), then the 4-byte word at +0x150 written to the
/// buffer (`00865e50(buffer, this + 0x150, 4, 0)`).
pub fn missile_projectile_save_game(e: &mut Engine, this: Ptr<MissileProjectile>, buffer: Ptr) {
    e.call(0x009c_4ff0, &args![this, buffer]);
    e.call(0x0086_5e50, &args![buffer, this.addr() + 0x150, 4u32, 0u32]);
}

// Translated from 009baad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MissileProjectile::LoadGame` (Xbox PDB name at vtable slot +0x5C): the
/// base load (`009c52f0`), then 4 bytes read from the buffer into +0x150
/// (`00864980(buffer, this + 0x150, 4)`).
pub fn missile_projectile_load_game(e: &mut Engine, this: Ptr<MissileProjectile>, buffer: Ptr) {
    e.call(0x009c_52f0, &args![this, buffer]);
    e.call(0x0086_4980, &args![buffer, this.addr() + 0x150, 4u32]);
}

// Translated from 009bab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MissileProjectile::InitLoadGame` (Xbox PDB name at vtable slot +0x64):
/// forwards to the base implementation `009c5740`.
pub fn missile_projectile_init_load_game(
    e: &mut Engine,
    this: Ptr<MissileProjectile>,
    argument: u32,
) {
    e.call(0x009c_5740, &args![this, argument]);
}

// Translated from 009bab20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MissileProjectile::FinishLoadGame` (Xbox PDB name at vtable slot +0x84):
/// the base implementation (`009c5870`), then, when virtual slot +0x1D0
/// answers nonzero, the impact handling `009babe0`; otherwise
/// `bWaitingToInitialize3D` is set.
pub fn missile_projectile_finish_load_game(
    e: &mut Engine,
    this: Ptr<MissileProjectile>,
    argument: u32,
) {
    e.call(0x009c_5870, &args![this, argument]);
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() == 0 {
        e.set(this, MissileProjectile::bWaitingToInitialize3D, true);
    } else {
        fn_009babe0(e, this);
    }
}

// Translated from 009bab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MissileProjectile::Revert` (Xbox PDB name at vtable slot +0x70): the base
/// implementation (`009c58e0`), then the word at +0x150 is cleared.
pub fn missile_projectile_revert(e: &mut Engine, this: Ptr<MissileProjectile>, argument: u32) {
    e.call(0x009c_58e0, &args![this, argument]);
    e.set(this, MissileProjectile::eImpactResult, 0);
}

// Translated from 009baba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the word at +0x150 is 4 or `00978f10` answers true (vtable slot
/// +0x324; the Xbox PDB name at that slot is not confirmed on the PC).
pub fn fn_009baba0(e: &mut Engine, this: Ptr<MissileProjectile>) -> bool {
    e.get(this, MissileProjectile::eImpactResult) == 4 || e.call(0x0097_8f10, &args![this]).bool()
}

// Translated from 009babe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the word at +0x150 is 3 or 4: finds the impact item (the first item
/// of the list at +0x88, or for 4 the item of its second node, null when
/// there is none) and, when its collidee answers true to virtual slot
/// +0x100, calls `009b90b0`. Then, when the word is 4, runs the impact
/// handling `009ba660`.
pub fn fn_009babe0(e: &mut Engine, this: Ptr<MissileProjectile>) {
    let state = e.get(this, MissileProjectile::eImpactResult);
    if state == 4 || state == 3 {
        let list = this.addr() + 0x88;
        let list_head = e.call(SELF_GETTER, &args![list]).u32();
        let mut impact = e.mem.u32(list_head);
        if e.get(this, MissileProjectile::eImpactResult) == 4 {
            impact = if e.call(LIST_NEXT, &args![list]).u32() != 0 {
                let next = e.call(LIST_NEXT, &args![list]).u32();
                let node = e.call(SELF_GETTER, &args![next]).u32();
                e.mem.u32(node)
            } else {
                0
            };
        }
        if impact != 0 {
            let collidee = e.mem.u32(impact);
            if collidee != 0 && e.vcall(collidee, 0x100, &args![]).bool() {
                e.call(0x009b_90b0, &args![this]);
            }
        }
    }
    if e.get(this, MissileProjectile::eImpactResult) == 4 {
        fn_009ba660(e, this);
    }
}

// Translated from 009bacb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MuzzleFlash` constructor: both `NiPointer`s (+0xC, +0x10) constructed and
/// cleared, `pProjectile` and `pWeaponSource` read from `source` (`004181e0`,
/// `00476c90`) when it is nonzero, `pParentReference`, the three flag bytes
/// and `fEnableTimer` cleared, `fDurationTimer` taken from `00507e10` of
/// `pProjectile`, then `009baf50(parent_reference)`. Returns `this`.
// Translation incomplete: 009bacb0 - the C++ exception unwinding frame (FS:[0]) is not translated.
pub fn fn_009bacb0(
    e: &mut Engine,
    this: Ptr<MuzzleFlash>,
    source: u32,
    parent_reference: u32,
) -> Ptr<MuzzleFlash> {
    e.call(NI_POINTER_CONSTRUCT, &args![this.addr() + 0xc, 0u32]);
    e.call(NI_POINTER_CONSTRUCT, &args![this.addr() + 0x10, 0u32]);
    e.call(NI_POINTER_SET, &args![this.addr() + 0xc, 0u32]);
    e.call(NI_POINTER_SET, &args![this.addr() + 0x10, 0u32]);
    let projectile = if source == 0 {
        0
    } else {
        e.call(0x0041_81e0, &args![source]).u32()
    };
    e.set(this, MuzzleFlash::pProjectile, Ptr::new(projectile));
    let weapon = if source == 0 {
        0
    } else {
        e.call(0x0047_6c90, &args![source]).u32()
    };
    e.set(this, MuzzleFlash::pWeaponSource, Ptr::new(weapon));
    e.set(
        this,
        MuzzleFlash::pParentReference,
        Ptr::new(parent_reference),
    );
    e.set(this, MuzzleFlash::bEnabled, false);
    e.set(this, MuzzleFlash::bMPSEnabled, false);
    e.set(this, MuzzleFlash::bUpdateLight, false);
    e.set(this, MuzzleFlash::fEnableTimer, 0.0);
    let projectile = e.get(this, MuzzleFlash::pProjectile);
    let duration = e.call(0x0050_7e10, &args![projectile]).f32();
    e.set(this, MuzzleFlash::fDurationTimer, duration);
    e.call(0x009b_af50, &args![this, parent_reference]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009ba550,
            hk_hinge_constraint_cinfo_construct(Ptr<HkConstraintCinfo>) -> Ptr<HkConstraintCinfo>
        ),
        entry!(
            0x009ba570,
            hk_constraint_cinfo_construct(Ptr<HkConstraintCinfo>) -> Ptr<HkConstraintCinfo>
        ),
        entry!(
            0x009ba5c0,
            hk_constraint_cinfo_scalar_deleting_destructor(
                Ptr<HkConstraintCinfo>,
                u32,
            ) -> Ptr<HkConstraintCinfo>
        ),
        entry!(
            0x009ba5f0,
            hk_constraint_cinfo_destruct(Ptr<HkConstraintCinfo>)
        ),
        entry!(
            0x009ba610,
            hk_hinge_constraint_cinfo_destruct(Ptr<HkConstraintCinfo>)
        ),
        entry!(
            0x009ba630,
            hk_hinge_constraint_cinfo_scalar_deleting_destructor(
                Ptr<HkConstraintCinfo>,
                u32,
            )
                -> Ptr<HkConstraintCinfo>
        ),
        entry!(0x009ba660, fn_009ba660(Ptr<MissileProjectile>)),
        entry!(
            0x009baaa0,
            missile_projectile_save_game(Ptr<MissileProjectile>, Ptr)
        ),
        entry!(
            0x009baad0,
            missile_projectile_load_game(Ptr<MissileProjectile>, Ptr)
        ),
        entry!(
            0x009bab00,
            missile_projectile_init_load_game(Ptr<MissileProjectile>, u32)
        ),
        entry!(
            0x009bab20,
            missile_projectile_finish_load_game(Ptr<MissileProjectile>, u32)
        ),
        entry!(
            0x009bab70,
            missile_projectile_revert(Ptr<MissileProjectile>, u32)
        ),
        entry!(0x009baba0, fn_009baba0(Ptr<MissileProjectile>) -> bool),
        entry!(0x009babe0, fn_009babe0(Ptr<MissileProjectile>)),
        entry!(
            0x009bacb0,
            fn_009bacb0(Ptr<MuzzleFlash>, u32, u32) -> Ptr<MuzzleFlash>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    /// Every function outside this file that the translations call, except
    /// those the tests give a behavior.
    const EXTERNAL: &[u32] = &[
        CONSTRAINT_SET_DATA,
        NI_POINTER_CONSTRUCT,
        0x009c_4ff0,
        0x0086_5e50,
        0x009c_52f0,
        0x0086_4980,
        0x009c_5740,
        0x009c_5870,
        0x009c_58e0,
        0x0097_8f10,
        0x009b_90b0,
        0x0042_ce10,
        0x007a_f430,
        0x009c_8c40,
        0x0046_0140,
        0x004b_5820,
        0x004b_5a80,
        0x0043_b4f0,
        0x0043_b4d0,
        0x005d_e0a0,
        0x004b_5260,
        0x006f_a820,
        0x004a_39f0,
        0x0062_6cb0,
        0x009b_7010,
        0x0045_bb20,
        0x0043_9e90,
        0x0057_5830,
        0x009b_f470,
        0x00c6_9f50,
        0x0043_d410,
        0x00a5_9c60,
        0x0051_7630,
        0x00c6_a350,
        0x0044_4d00,
        0x0046_10f0,
        0x005a_e3d0,
        0x0062_8f30,
        0x0047_6a80,
        0x0046_1130,
        0x004b_4880,
        0x0062_c250,
        0x009b_7cb0,
        0x009b_8c90,
        0x0041_81e0,
        0x0047_6c90,
        0x0050_7e10,
        0x009b_af50,
    ];

    /// Virtual slot targets, outside the heap.
    const OBJECT_GETTER: u32 = 0x0700_0000;
    const MOVE_SLOT: u32 = 0x0700_0010;
    const MOTION_SLOT: u32 = 0x0700_0020;
    const IS_ACTOR_SLOT: u32 = 0x0700_0030;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn zero(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn identity(_: &mut Engine, a: &[u32]) -> Ret {
        ret(a[0])
    }

    fn release(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.free(a[0]);
        Ret::default()
    }

    fn pointer_set(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(a[0], a[1]);
        Ret::default()
    }

    fn list_next(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0] + 4))
    }

    fn list_is_empty(e: &mut Engine, a: &[u32]) -> Ret {
        ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &address in EXTERNAL {
            e.register(address, zero);
        }
        e.register(FREE, release);
        e.register(SELF_GETTER, identity);
        e.register(NI_POINTER_SET, pointer_set);
        e.register(LIST_NEXT, list_next);
        e.register(LIST_IS_EMPTY, list_is_empty);
        for address in [STICK_VALUE, STICK_TIME, IMPACT_QUERY_OBJECT] {
            e.map(address, 8);
        }
        // The static float accessor answers a cell holding 1.0.
        let cell = e.mem.alloc(4);
        e.mem.set_f32(cell, 1.0);
        e.register_double(0x0040_3e20, move |_, _| ret(cell));
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    /// The addresses of the logged calls that are in `wanted`, in order.
    fn order(e: &Engine, wanted: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|call| call.0)
            .filter(|address| wanted.contains(address))
            .collect()
    }

    /// An object whose vtable holds `slots` (byte offset, target).
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x500);
        e.mem.set_u32(object, vtable);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        object
    }

    fn cinfo(e: &mut Engine) -> Ptr<HkConstraintCinfo> {
        let cinfo = e.new_object::<HkConstraintCinfo>();
        e.set(cinfo, HkConstraintCinfo::vfptr, 0xdead_0001);
        e.set(cinfo, HkConstraintCinfo::pData, Ptr::new(0xdead_0002));
        e.set(cinfo, HkConstraintCinfo::ePriority, 0xdead_0003);
        e.set(cinfo, HkConstraintCinfo::pEntityA, Ptr::new(0xdead_0004));
        e.set(cinfo, HkConstraintCinfo::pEntityB, Ptr::new(0xdead_0005));
        cinfo
    }

    #[test]
    fn the_base_constructor_resets_the_fields() {
        let mut e = engine();
        let c = cinfo(&mut e);
        assert_eq!(e.call(0x009b_a570, &args![c]).u32(), c.addr());
        assert_eq!(calls_to(&e, SELF_GETTER), vec![vec![c.addr() + 4]]);
        assert_eq!(e.get(c, HkConstraintCinfo::vfptr), 0x0108_ffb4);
        assert_eq!(e.get(c, HkConstraintCinfo::pData).addr(), 0);
        assert_eq!(e.get(c, HkConstraintCinfo::ePriority), 1);
        assert_eq!(e.get(c, HkConstraintCinfo::pEntityA).addr(), 0);
        assert_eq!(e.get(c, HkConstraintCinfo::pEntityB).addr(), 0);
    }

    #[test]
    fn the_hinge_constructor_ends_with_the_hinge_vtable() {
        let mut e = engine();
        let c = cinfo(&mut e);
        assert_eq!(e.call(0x009b_a550, &args![c]).u32(), c.addr());
        assert_eq!(e.get(c, HkConstraintCinfo::vfptr), 0x0108_ff98);
        assert_eq!(e.get(c, HkConstraintCinfo::ePriority), 1);
        assert_eq!(e.get(c, HkConstraintCinfo::pEntityB).addr(), 0);
    }

    #[test]
    fn the_destructor_restores_the_base_vtable_and_clears_the_data() {
        let mut e = engine();
        let c = cinfo(&mut e);
        e.call(0x009b_a5f0, &args![c]);
        assert_eq!(e.get(c, HkConstraintCinfo::vfptr), 0x0108_ffb4);
        assert_eq!(calls_to(&e, CONSTRAINT_SET_DATA), vec![vec![c.addr(), 0]]);
    }

    #[test]
    fn the_hinge_destructor_runs_the_base_destructor_body() {
        let mut e = engine();
        let c = cinfo(&mut e);
        e.call(0x009b_a610, &args![c]);
        assert_eq!(e.get(c, HkConstraintCinfo::vfptr), 0x0108_ffb4);
        assert_eq!(calls_to(&e, CONSTRAINT_SET_DATA), vec![vec![c.addr(), 0]]);
    }

    #[test]
    fn the_scalar_deleting_destructors_free_on_flag_one_only() {
        for address in [0x009b_a5c0, 0x009b_a630] {
            let mut e = engine();
            let c = cinfo(&mut e);
            assert_eq!(e.call(address, &args![c, 0u32]).u32(), c.addr());
            assert!(calls_to(&e, FREE).is_empty());
            assert_eq!(calls_to(&e, CONSTRAINT_SET_DATA).len(), 1);
            assert_eq!(e.call(address, &args![c, 1u32]).u32(), c.addr());
            assert_eq!(calls_to(&e, FREE), vec![vec![c.addr()]]);
            assert_eq!(calls_to(&e, CONSTRAINT_SET_DATA).len(), 2);
        }
    }

    #[test]
    fn saving_writes_the_word_at_0x150_after_the_base_save() {
        let mut e = engine();
        let this = e.new_object::<MissileProjectile>();
        e.call(0x009b_aaa0, &args![this, 0x4000u32]);
        assert_eq!(
            order(&e, &[0x009c_4ff0, 0x0086_5e50]),
            vec![0x009c_4ff0, 0x0086_5e50]
        );
        assert_eq!(calls_to(&e, 0x009c_4ff0), vec![vec![this.addr(), 0x4000]]);
        assert_eq!(
            calls_to(&e, 0x0086_5e50),
            vec![vec![0x4000, this.addr() + 0x150, 4, 0]]
        );
    }

    #[test]
    fn loading_reads_the_word_at_0x150_after_the_base_load() {
        let mut e = engine();
        let this = e.new_object::<MissileProjectile>();
        e.call(0x009b_aad0, &args![this, 0x4000u32]);
        assert_eq!(
            order(&e, &[0x009c_52f0, 0x0086_4980]),
            vec![0x009c_52f0, 0x0086_4980]
        );
        assert_eq!(calls_to(&e, 0x009c_52f0), vec![vec![this.addr(), 0x4000]]);
        assert_eq!(
            calls_to(&e, 0x0086_4980),
            vec![vec![0x4000, this.addr() + 0x150, 4]]
        );
    }

    #[test]
    fn init_load_game_forwards_to_the_base() {
        let mut e = engine();
        let this = e.new_object::<MissileProjectile>();
        e.call(0x009b_ab00, &args![this, 0x77u32]);
        assert_eq!(calls_to(&e, 0x009c_5740), vec![vec![this.addr(), 0x77]]);
    }

    /// A projectile whose slot +0x1D0 is `OBJECT_GETTER` (answering `object`)
    /// and whose slot +0x2A4 is `MOVE_SLOT`.
    fn projectile(e: &mut Engine, object_3d: u32) -> Ptr<MissileProjectile> {
        e.register_double(OBJECT_GETTER, move |_, _| ret(object_3d));
        e.register(MOVE_SLOT, zero);
        let vtable = e.mem.alloc(0x500);
        e.mem.set_u32(vtable + 0x1d0, OBJECT_GETTER);
        e.mem.set_u32(vtable + 0x2a4, MOVE_SLOT);
        let this = e.new_object::<MissileProjectile>();
        e.mem.set_u32(this.addr(), vtable);
        this
    }

    #[test]
    fn finishing_the_load_without_a_3d_object_waits_for_it() {
        let mut e = engine();
        let this = projectile(&mut e, 0);
        e.call(0x009b_ab20, &args![this, 0x55u32]);
        assert_eq!(calls_to(&e, 0x009c_5870), vec![vec![this.addr(), 0x55]]);
        assert!(e.get(this, MissileProjectile::bWaitingToInitialize3D));
        assert!(calls_to(&e, 0x0040_3e20).is_empty());
    }

    #[test]
    fn finishing_the_load_with_a_3d_object_runs_the_impact_check() {
        let mut e = engine();
        let this = projectile(&mut e, 0x1234);
        e.set(this, MissileProjectile::eImpactResult, 4);
        e.call(0x009b_ab20, &args![this, 0x55u32]);
        assert!(!e.get(this, MissileProjectile::bWaitingToInitialize3D));
        // State 4 reaches the impact handling (its static is initialized).
        assert_eq!(calls_to(&e, 0x0040_3e20).len(), 1);
    }

    #[test]
    fn reverting_clears_the_word_after_the_base_revert() {
        let mut e = engine();
        let this = e.new_object::<MissileProjectile>();
        e.set(this, MissileProjectile::eImpactResult, 4);
        e.call(0x009b_ab70, &args![this, 0x66u32]);
        assert_eq!(calls_to(&e, 0x009c_58e0), vec![vec![this.addr(), 0x66]]);
        assert_eq!(e.get(this, MissileProjectile::eImpactResult), 0);
    }

    #[test]
    fn the_predicate_is_true_for_state_4_without_asking() {
        let mut e = engine();
        let this = e.new_object::<MissileProjectile>();
        e.set(this, MissileProjectile::eImpactResult, 4);
        assert!(e.call(0x009b_aba0, &args![this]).bool());
        assert!(calls_to(&e, 0x0097_8f10).is_empty());
    }

    #[test]
    fn the_predicate_asks_the_callee_in_other_states() {
        let mut e = engine();
        let this = e.new_object::<MissileProjectile>();
        e.set(this, MissileProjectile::eImpactResult, 3);
        assert!(!e.call(0x009b_aba0, &args![this]).bool());
        e.register(0x0097_8f10, |_, _| ret(1));
        assert!(e.call(0x009b_aba0, &args![this]).bool());
        assert_eq!(calls_to(&e, 0x0097_8f10).len(), 2);
    }

    /// A projectile (no 3D object) whose impact list has a first node and,
    /// when `second` is given, a second node; each node holds an impact
    /// whose collidee answers `collidee_true` to slot +0x100.
    fn list_scene(e: &mut Engine, first: u32, second: Option<u32>) -> Ptr<MissileProjectile> {
        let this = projectile(e, 0);
        e.set(this, MissileProjectile::ImpactListItem, Ptr::new(first));
        if let Some(second_item) = second {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, second_item);
            e.mem.set_u32(this.addr() + 0x8c, node);
        }
        this
    }

    fn impact_with_collidee(e: &mut Engine, is_actor: bool) -> u32 {
        e.register(IS_ACTOR_SLOT, |_, _| ret(1));
        e.register(IS_ACTOR_SLOT + 0x10, zero);
        let slot = if is_actor {
            IS_ACTOR_SLOT
        } else {
            IS_ACTOR_SLOT + 0x10
        };
        let collidee = object_with_slots(e, &[(0x100, slot)]);
        let impact = e.mem.alloc(0x30);
        e.mem.set_u32(impact, collidee);
        impact
    }

    #[test]
    fn state_1_does_nothing() {
        let mut e = engine();
        let impact = impact_with_collidee(&mut e, true);
        let this = list_scene(&mut e, impact, None);
        e.set(this, MissileProjectile::eImpactResult, 1);
        e.call(0x009b_abe0, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn state_3_tests_the_first_items_collidee() {
        let mut e = engine();
        let impact = impact_with_collidee(&mut e, true);
        let this = list_scene(&mut e, impact, None);
        e.set(this, MissileProjectile::eImpactResult, 3);
        e.call(0x009b_abe0, &args![this]);
        assert_eq!(calls_to(&e, 0x009b_90b0), vec![vec![this.addr()]]);
        // State 3 does not run the impact handling.
        assert!(calls_to(&e, 0x0040_3e20).is_empty());
    }

    #[test]
    fn state_3_with_a_collidee_that_is_not_an_actor_does_nothing() {
        let mut e = engine();
        let impact = impact_with_collidee(&mut e, false);
        let this = list_scene(&mut e, impact, None);
        e.set(this, MissileProjectile::eImpactResult, 3);
        e.call(0x009b_abe0, &args![this]);
        assert!(calls_to(&e, 0x009b_90b0).is_empty());
    }

    #[test]
    fn state_3_with_an_empty_list_does_nothing() {
        let mut e = engine();
        let this = list_scene(&mut e, 0, None);
        e.set(this, MissileProjectile::eImpactResult, 3);
        e.call(0x009b_abe0, &args![this]);
        assert!(calls_to(&e, 0x009b_90b0).is_empty());
    }

    #[test]
    fn state_3_with_a_null_collidee_does_nothing() {
        let mut e = engine();
        let impact = e.mem.alloc(0x30);
        let this = list_scene(&mut e, impact, None);
        e.set(this, MissileProjectile::eImpactResult, 3);
        e.call(0x009b_abe0, &args![this]);
        assert!(calls_to(&e, 0x009b_90b0).is_empty());
    }

    #[test]
    fn state_4_uses_the_second_node_and_then_handles_the_impact() {
        let mut e = engine();
        let first = impact_with_collidee(&mut e, false);
        let second = impact_with_collidee(&mut e, true);
        let this = list_scene(&mut e, first, Some(second));
        e.set(this, MissileProjectile::eImpactResult, 4);
        e.call(0x009b_abe0, &args![this]);
        assert_eq!(calls_to(&e, 0x009b_90b0), vec![vec![this.addr()]]);
        assert_eq!(
            order(&e, &[0x009b_90b0, 0x0040_3e20]),
            vec![0x009b_90b0, 0x0040_3e20]
        );
    }

    #[test]
    fn state_4_without_a_second_node_has_no_item() {
        let mut e = engine();
        let first = impact_with_collidee(&mut e, true);
        let this = list_scene(&mut e, first, None);
        e.set(this, MissileProjectile::eImpactResult, 4);
        e.call(0x009b_abe0, &args![this]);
        assert!(calls_to(&e, 0x009b_90b0).is_empty());
        assert_eq!(calls_to(&e, 0x0040_3e20).len(), 1);
    }

    /// The impact scene of `009ba660`.
    struct Impact {
        this: Ptr<MissileProjectile>,
        object_3d: u32,
        impact: u32,
        world_object: u32,
    }

    /// A projectile with a 3D object, one impact whose world object is
    /// 0x5000 and a collidee that is not an actor; the callees answer plain
    /// values (see the constants in the body).
    fn impact_scene(e: &mut Engine) -> Impact {
        let object_3d = object_with_slots(e, &[(0x9c, MOTION_SLOT)]);
        e.register(MOTION_SLOT, |_, _| ret(0x9999));
        let this = projectile(e, object_3d);
        let impact = impact_with_collidee(e, false);
        let world_object = 0x5000;
        e.mem.set_u32(impact + 0x1c, world_object);
        e.set(this, MissileProjectile::ImpactListItem, Ptr::new(impact));
        let cell = e.mem.alloc(4);
        e.mem.set_f32(cell, 3.5);
        e.register_double(0x0040_3e20, move |_, _| ret(cell));
        e.set_global(STICK_TIME, 10.0f32);
        e.register(0x0046_0140, |_, a| ret(a[0] + 0x10));
        e.register(0x004b_5820, |_, _| ret(0xa0));
        e.register(0x004b_5a80, |_, _| ret(0xb0));
        e.register(0x0043_b4d0, |_, _| ret(1));
        e.register(0x004b_5260, |_, _| ret(0xd0));
        e.register(0x006f_a820, |_, _| ret(0xe0));
        e.register(0x009b_7010, |_, _| ret(0x11));
        e.register(0x0045_bb20, |_, _| ret(0x22));
        e.register(0x0043_9e90, |_, _| ret(0x33));
        e.register(0x0051_7630, |_, _| ret(5));
        e.register(0x0046_1130, |_, a| ret(0x1000 + a[0]));
        e.register(0x009b_7cb0, |_, _| ret(0x66));
        let transform = e.mem.alloc(0x34);
        e.register_double(0x0062_c250, move |_, _| ret(transform));
        Impact {
            this,
            object_3d,
            impact,
            world_object,
        }
    }

    #[test]
    fn the_static_value_is_initialized_once_from_the_accessor() {
        let mut e = engine();
        let this = projectile(&mut e, 0);
        let cell = e.mem.alloc(4);
        e.mem.set_f32(cell, 3.5);
        e.register_double(0x0040_3e20, move |_, _| ret(cell));
        e.call(0x009b_a660, &args![this]);
        assert_eq!(calls_to(&e, 0x0040_3e20), vec![vec![STICK_VALUE_SOURCE]]);
        assert_eq!(e.global::<u32>(STICK_VALUE_GUARD) & 1, 1);
        assert_eq!(e.global::<f32>(STICK_VALUE), 3.5);
        // The guard is set: a second run keeps the value.
        e.mem.set_f32(cell, 9.0);
        e.call(0x009b_a660, &args![this]);
        assert_eq!(calls_to(&e, 0x0040_3e20).len(), 1);
        assert_eq!(e.global::<f32>(STICK_VALUE), 3.5);
    }

    #[test]
    fn the_static_value_copy_quiets_a_signalling_nan() {
        let mut e = engine();
        let this = projectile(&mut e, 0);
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, 0x7fa0_0000);
        e.register_double(0x0040_3e20, move |_, _| ret(cell));
        e.call(0x009b_a660, &args![this]);
        assert_eq!(e.global::<u32>(STICK_VALUE), 0x7fe0_0000);
    }

    #[test]
    fn nothing_happens_without_a_3d_object() {
        let mut e = engine();
        let this = projectile(&mut e, 0);
        e.call(0x009b_a660, &args![this]);
        assert!(calls_to(&e, LIST_IS_EMPTY).is_empty());
        assert_eq!(calls_to(&e, 0x0042_ce10), vec![vec![0]]);
    }

    #[test]
    fn nothing_happens_with_an_empty_impact_list() {
        let mut e = engine();
        let this = projectile(&mut e, 0x1234);
        e.call(0x009b_a660, &args![this]);
        assert_eq!(calls_to(&e, LIST_IS_EMPTY), vec![vec![this.addr() + 0x88]]);
        assert!(calls_to(&e, 0x007a_f430).is_empty());
        assert_eq!(e.get(this, MissileProjectile::fStickTime), 0.0);
    }

    #[test]
    fn the_stick_time_is_set_and_the_holder_released_when_it_holds_this() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        // A null world object ends the handling after the stick time.
        e.mem.set_u32(s.impact + 0x1c, 0);
        e.register_double(0x007a_f430, {
            let this = s.this.addr();
            move |_, _| ret(this)
        });
        e.call(0x009b_a660, &args![s.this]);
        assert_eq!(e.get(s.this, MissileProjectile::fStickTime), 10.0);
        assert_eq!(calls_to(&e, 0x007a_f430), vec![vec![REFERENCE_HOLDER]]);
        assert_eq!(calls_to(&e, 0x009c_8c40), vec![vec![REFERENCE_HOLDER, 0]]);
        assert!(calls_to(&e, 0x0046_0140).is_empty());
    }

    #[test]
    fn another_holder_is_left_alone() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.mem.set_u32(s.impact + 0x1c, 0);
        e.register(0x007a_f430, |_, _| ret(0x4321));
        e.call(0x009b_a660, &args![s.this]);
        assert!(calls_to(&e, 0x009c_8c40).is_empty());
    }

    #[test]
    fn an_empty_impact_item_ends_the_handling_after_the_stick_time() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.set(s.this, MissileProjectile::ImpactListItem, Ptr::new(0));
        e.mem.set_u32(s.this.addr() + 0x8c, 0x100);
        e.call(0x009b_a660, &args![s.this]);
        assert_eq!(e.get(s.this, MissileProjectile::fStickTime), 10.0);
        assert!(calls_to(&e, 0x0046_0140).is_empty());
    }

    #[test]
    fn the_missing_target_takes_the_alternative_path() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(0x004b_5a80, zero);
        e.call(0x009b_a660, &args![s.this]);
        assert_eq!(calls_to(&e, 0x009b_8c90), vec![vec![s.this.addr()]]);
        assert!(calls_to(&e, 0x0043_b4f0).is_empty());
        assert!(calls_to(&e, 0x005d_e0a0).is_empty());
    }

    #[test]
    fn type_0x13_takes_the_alternative_path() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(0x0043_b4d0, |_, _| ret(0x13));
        e.call(0x009b_a660, &args![s.this]);
        assert_eq!(calls_to(&e, 0x009b_8c90), vec![vec![s.this.addr()]]);
        assert!(calls_to(&e, 0x005d_e0a0).is_empty());
    }

    #[test]
    fn an_actor_collidee_takes_the_alternative_path() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        let other = impact_with_collidee(&mut e, true);
        let actor = e.mem.u32(other);
        e.mem.set_u32(s.impact, actor);
        e.call(0x009b_a660, &args![s.this]);
        assert_eq!(calls_to(&e, 0x009b_8c90), vec![vec![s.this.addr()]]);
        assert!(calls_to(&e, 0x005d_e0a0).is_empty());
    }

    #[test]
    fn a_null_collidee_does_not_divert() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.mem.set_u32(s.impact, 0);
        e.call(0x009b_a660, &args![s.this]);
        assert!(calls_to(&e, 0x009b_8c90).is_empty());
        assert_eq!(calls_to(&e, 0x005d_e0a0).len(), 1);
    }

    #[test]
    fn the_target_lookup_chain_uses_the_world_object() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.call(0x009b_a660, &args![s.this]);
        assert_eq!(calls_to(&e, 0x0046_0140), vec![vec![s.world_object]]);
        assert_eq!(calls_to(&e, 0x004b_5820), vec![vec![s.world_object + 0x10]]);
        assert_eq!(calls_to(&e, 0x004b_5a80), vec![vec![s.world_object]]);
        let info = calls_to(&e, 0x0043_b4f0)[0].clone();
        assert_eq!(info[0], 0xb0);
        assert_eq!(calls_to(&e, 0x0043_b4d0)[0], vec![0]);
    }

    #[test]
    fn a_type_5_target_gets_motion_5_and_nothing_else_of_the_other_branch() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.call(0x009b_a660, &args![s.this]);
        let this = s.this.addr();
        assert_eq!(calls_to(&e, 0x005d_e0a0), vec![vec![this, 4, 1]]);
        assert_eq!(calls_to(&e, MOVE_SLOT), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x004b_5260), vec![vec![s.object_3d]]);
        assert_eq!(calls_to(&e, 0x006f_a820), vec![vec![0xd0]]);
        assert_eq!(calls_to(&e, 0x009b_f470), vec![vec![this, 0, 0]]);
        assert_eq!(calls_to(&e, 0x00c6_9f50), vec![vec![s.object_3d, 1, 1]]);
        let update = calls_to(&e, 0x0043_d410);
        assert_eq!(update[0][1..], [0, 0, 0]);
        assert_eq!(
            calls_to(&e, 0x00a5_9c60),
            vec![vec![s.object_3d, update[0][0]]]
        );
        assert_eq!(calls_to(&e, 0x0051_7630), vec![vec![0xb0]]);
        assert_eq!(
            calls_to(&e, 0x00c6_a350),
            vec![vec![s.object_3d, 5, 1, 0, 1], vec![0x9999, 1, 1, 1, 1]]
        );
        assert!(calls_to(&e, 0x0044_4d00).is_empty());
        assert!(calls_to(&e, 0x0062_8f30).is_empty());
        assert!(calls_to(&e, 0x0047_6a80).is_empty());
        assert_eq!(calls_to(&e, 0x009b_7cb0).len(), 1);
        assert_eq!(calls_to(&e, MOTION_SLOT), vec![vec![s.object_3d, 0x66]]);
    }

    #[test]
    fn the_collision_filter_is_rewritten_through_the_body() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(0x0043_b4f0, |e, a| {
            if a[0] == 0xe0 {
                e.mem.set_u32(a[1], 0x0000_00f0);
            }
            ret(0)
        });
        e.register(0x004a_39f0, |e, a| {
            let value = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], value | a[1]);
            Ret::default()
        });
        e.call(0x009b_a660, &args![s.this]);
        // Body 0xE0: filter 0xF0 | 0xF = 0xFF written back.
        assert_eq!(calls_to(&e, 0x0062_6cb0), vec![vec![0xe0, 0xff]]);
        assert_eq!(calls_to(&e, 0x004a_39f0)[0][1], 0xf);
    }

    #[test]
    fn without_a_collision_object_there_is_no_body_work() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(0x004b_5260, zero);
        e.call(0x009b_a660, &args![s.this]);
        assert!(calls_to(&e, 0x006f_a820).is_empty());
        assert!(calls_to(&e, 0x0062_6cb0).is_empty());
        assert!(calls_to(&e, 0x004a_39f0).is_empty());
        assert_eq!(calls_to(&e, 0x009b_f470).len(), 1);
    }

    #[test]
    fn the_projectile_is_repositioned_on_the_impact_location() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.call(0x009b_a660, &args![s.this]);
        let this = s.this.addr();
        let first = calls_to(&e, 0x009b_7010)[0].clone();
        assert_eq!(first[0], this);
        let second = calls_to(&e, 0x0045_bb20)[0].clone();
        assert_eq!(second[0], 0x11);
        assert_eq!(second[2], 3.5f32.to_bits());
        let third = calls_to(&e, 0x0043_9e90)[0].clone();
        assert_eq!(third[0], s.impact + 4);
        assert_eq!(third[2], 0x22);
        assert_eq!(calls_to(&e, 0x0057_5830), vec![vec![this, 0x33]]);
        let steps = [
            0x009b_7010,
            0x0045_bb20,
            0x0043_9e90,
            0x0057_5830,
            0x009b_f470,
        ];
        assert_eq!(order(&e, &steps), steps);
    }

    #[test]
    fn a_nonzero_query_skips_the_repositioning_and_the_follow_offset() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(0x0042_ce10, |_, _| ret(1));
        e.register(0x0051_7630, |_, _| ret(2));
        e.call(0x009b_a660, &args![s.this]);
        assert!(calls_to(&e, 0x009b_7010).is_empty());
        assert!(calls_to(&e, 0x0057_5830).is_empty());
        assert!(calls_to(&e, 0x0047_6a80).is_empty());
        assert_eq!(calls_to(&e, 0x009b_f470).len(), 1);
    }

    #[test]
    fn other_targets_get_motion_4_flags_and_the_follow_offset() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(0x0051_7630, |_, _| ret(2));
        e.register(0x0046_10f0, |_, _| ret(0x44));
        let result = e.mem.alloc(0x34);
        for word in 0..13 {
            e.mem.set_u32(result + 4 * word, 100 + word);
        }
        e.register_double(0x0062_c250, move |_, _| ret(result));
        e.call(0x009b_a660, &args![s.this]);
        let this = s.this.addr();
        assert_eq!(calls_to(&e, 0x00c6_a350)[0], vec![s.object_3d, 4, 1, 0, 1]);
        assert_eq!(calls_to(&e, 0x0044_4d00), vec![vec![this, 0x80]]);
        assert_eq!(
            calls_to(&e, 0x005d_e0a0),
            vec![vec![this, 4, 1], vec![this, 0x80, 1]]
        );
        // The reference cell handed to 005ae3d0 holds the projectile.
        let register = calls_to(&e, 0x005a_e3d0);
        assert_eq!(register.len(), 1);
        assert_eq!(register[0][0], 0x44);
        assert_eq!(calls_to(&e, 0x0046_10f0), vec![vec![PROCESS_LISTS]]);
        assert_eq!(calls_to(&e, 0x0062_8f30), vec![vec![0xb0, 0x100, 1]]);
        // Follow offset: inverse of the node's transform times the 3D
        // object's, copied to +0x94.
        let inverse = calls_to(&e, 0x0047_6a80)[0][0];
        assert_eq!(calls_to(&e, 0x004b_4880), vec![vec![0x10a0, inverse]]);
        let product = calls_to(&e, 0x0062_c250)[0].clone();
        assert_eq!(product[0], inverse);
        assert_eq!(product[2], 0x1000 + s.object_3d);
        assert_eq!(
            calls_to(&e, 0x0046_1130),
            vec![vec![0xa0], vec![s.object_3d]]
        );
        for word in 0..13 {
            assert_eq!(e.mem.u32(this + 0x94 + 4 * word), 100 + word);
        }
        assert_eq!(e.mem.u32(this + 0x94 + 4 * 13), 0);
    }

    #[test]
    fn the_registration_cell_holds_the_projectile() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(0x0051_7630, |_, _| ret(2));
        e.register(0x0046_10f0, |_, _| ret(0x44));
        let seen = std::rc::Rc::new(std::cell::Cell::new(0));
        let sink = std::rc::Rc::clone(&seen);
        e.register_double(0x005a_e3d0, move |e, a| {
            sink.set(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call(0x009b_a660, &args![s.this]);
        assert_eq!(seen.get(), s.this.addr());
    }

    #[test]
    fn flag_0x80_already_set_skips_the_registration() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(0x0051_7630, |_, _| ret(2));
        e.register(0x0044_4d00, |_, _| ret(1));
        e.call(0x009b_a660, &args![s.this]);
        assert_eq!(calls_to(&e, 0x005d_e0a0).len(), 1);
        assert!(calls_to(&e, 0x0046_10f0).is_empty());
        assert!(calls_to(&e, 0x005a_e3d0).is_empty());
        assert_eq!(calls_to(&e, 0x0062_8f30).len(), 1);
    }

    #[test]
    fn a_missing_process_holder_skips_the_registration_call() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(0x0051_7630, |_, _| ret(2));
        e.call(0x009b_a660, &args![s.this]);
        assert_eq!(calls_to(&e, 0x0046_10f0).len(), 1);
        assert!(calls_to(&e, 0x005a_e3d0).is_empty());
    }

    #[test]
    fn without_a_target_node_there_is_no_follow_offset() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(0x0051_7630, |_, _| ret(2));
        e.register(0x004b_5820, zero);
        e.call(0x009b_a660, &args![s.this]);
        assert!(calls_to(&e, 0x0047_6a80).is_empty());
        assert_eq!(e.mem.u32(s.this.addr() + 0x94), 0);
    }

    #[test]
    fn a_nonzero_query_keeps_the_follow_offset_untouched() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(0x0051_7630, |_, _| ret(2));
        e.register(0x0042_ce10, |_, _| ret(1));
        e.call(0x009b_a660, &args![s.this]);
        assert!(calls_to(&e, 0x0047_6a80).is_empty());
    }

    #[test]
    fn no_motion_object_means_no_final_motion_call() {
        let mut e = engine();
        let s = impact_scene(&mut e);
        e.register(MOTION_SLOT, zero);
        e.call(0x009b_a660, &args![s.this]);
        assert_eq!(calls_to(&e, 0x00c6_a350).len(), 1);
    }

    #[test]
    fn the_muzzle_flash_constructor_reads_the_source_and_clears_the_rest() {
        let mut e = engine();
        e.register(0x0041_81e0, |_, a| ret(a[0] + 1));
        e.register(0x0047_6c90, |_, a| ret(a[0] + 2));
        e.register(0x0050_7e10, |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        let flash = e.new_object::<MuzzleFlash>();
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(flash.addr() + offset, 0xdead_beef);
        }
        let result = e.call(0x009b_acb0, &args![flash, 0x500u32, 0x600u32]).u32();
        assert_eq!(result, flash.addr());
        let this = flash.addr();
        assert_eq!(
            calls_to(&e, NI_POINTER_CONSTRUCT),
            vec![vec![this + 0xc, 0], vec![this + 0x10, 0]]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_SET),
            vec![vec![this + 0xc, 0], vec![this + 0x10, 0]]
        );
        assert_eq!(e.get(flash, MuzzleFlash::pProjectile).addr(), 0x501);
        assert_eq!(e.get(flash, MuzzleFlash::pWeaponSource).addr(), 0x502);
        assert_eq!(e.get(flash, MuzzleFlash::pParentReference).addr(), 0x600);
        assert!(!e.get(flash, MuzzleFlash::bEnabled));
        assert!(!e.get(flash, MuzzleFlash::bMPSEnabled));
        assert!(!e.get(flash, MuzzleFlash::bUpdateLight));
        assert_eq!(e.get(flash, MuzzleFlash::fEnableTimer), 0.0);
        assert_eq!(calls_to(&e, 0x0050_7e10), vec![vec![0x501]]);
        assert_eq!(e.get(flash, MuzzleFlash::fDurationTimer), 0.25);
        assert_eq!(calls_to(&e, 0x009b_af50), vec![vec![this, 0x600]]);
    }

    #[test]
    fn the_muzzle_flash_without_a_source_has_null_members() {
        let mut e = engine();
        let flash = e.new_object::<MuzzleFlash>();
        e.call(0x009b_acb0, &args![flash, 0u32, 0x600u32]);
        assert!(calls_to(&e, 0x0041_81e0).is_empty());
        assert!(calls_to(&e, 0x0047_6c90).is_empty());
        assert_eq!(e.get(flash, MuzzleFlash::pProjectile).addr(), 0);
        assert_eq!(e.get(flash, MuzzleFlash::pWeaponSource).addr(), 0);
        // The duration getter still runs, on the null projectile.
        assert_eq!(calls_to(&e, 0x0050_7e10), vec![vec![0]]);
    }
}
