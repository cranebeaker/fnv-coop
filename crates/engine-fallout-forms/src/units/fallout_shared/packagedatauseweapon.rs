//! `fallout shared/packagedatauseweapon.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0067d9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01068a88` (RTTI `.?AVUseWeaponActorPackageData@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `0067d970`. One word, plain `RET`, returns nothing.
pub fn fn_0067d9c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_8a88);
    e.call(0x0067_d970, &args![this]);
}

layout! {
    /// `TESUseWeaponPackageData` (Xbox PDB), 0x24 bytes (the same on PC).
    pub struct TESUseWeaponPackageData: 0x24 {
        /// `pAttackTarget` (Xbox PDB): `PackageTarget*`.
        0x04 pAttackTarget: u32,
        /// `pTargetLocation` (Xbox PDB): `PackageLocation*`.
        0x08 pTargetLocation: u32,
        /// `bAlwaysHit` (Xbox PDB).
        0x0C bAlwaysHit: u8,
        /// `bDoNoDamage` (Xbox PDB).
        0x0D bDoNoDamage: u8,
        /// `bCrouch` (Xbox PDB).
        0x0E bCrouch: u8,
        /// `bHoldFire` (Xbox PDB).
        0x0F bHoldFire: u8,
        /// `bVolleyFire` (Xbox PDB).
        0x10 bVolleyFire: u8,
        /// `bRepeatFire` (Xbox PDB).
        0x11 bRepeatFire: u8,
        /// `sBurstCount` (Xbox PDB).
        0x12 sBurstCount: u16,
        /// `sVolleyShotsMin` (Xbox PDB).
        0x14 sVolleyShotsMin: u16,
        /// `sVolleyShotsMax` (Xbox PDB).
        0x16 sVolleyShotsMax: u16,
        /// `fVolleyWaitMin` (Xbox PDB), as stored bits.
        0x18 fVolleyWaitMin: u32,
        /// `fVolleyWaitMax` (Xbox PDB), as stored bits.
        0x1C fVolleyWaitMax: u32,
        /// `pWeapon` (Xbox PDB): `TESObjectWEAP*` (a form id until
        /// `InitItem`).
        0x20 pWeapon: u32,
    }
}

layout! {
    /// `UseWeaponActorPackageData` (Xbox PDB), 0x8 bytes.
    pub struct UseWeaponActorPackageData: 0x08 {
        /// `pAttackTarget` (Xbox PDB): `TESObjectREFR*`.
        0x04 pAttackTarget: u32,
    }
}

/// `TESUseWeaponPackageData`'s vtable (RTTI locator `01113fc8`).
const USE_WEAPON_DATA_VTABLE: u32 = 0x0106_8a64;
/// `UseWeaponActorPackageData`'s vtable (stored by `0067d9c0` too).
const USE_WEAPON_ACTOR_DATA_VTABLE: u32 = 0x0106_8a88;
/// The `ActorPackageData` base vtable (RTTI locator `01114094`).
const ACTOR_PACKAGE_DATA_VTABLE: u32 = 0x0106_8aa4;
/// The `TESPackageData` base constructor (`0067ad20`) and destructor
/// (`0067ad70`), plain `RET`.
const PACKAGE_DATA_CONSTRUCT: u32 = 0x0067_ad20;
const PACKAGE_DATA_DESTRUCT: u32 = 0x0067_ad70;
/// The `float` the default constructor puts in `fVolleyWaitMax` (`010162c0`).
const DEFAULT_VOLLEY_WAIT_MAX: u32 = 0x0101_62c0;
/// The `MemoryManager` allocation (`00401000`) and free (`00401030`)
/// wrappers, cdecl.
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Looks up a form by id (`004839c0`, cdecl, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// RTTI type descriptors `.?AVTESForm@@` and `.?AVTESObjectWEAP@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_WEAP: u32 = 0x0118_3998;
/// The two forms `InitItem` compares the location reference's form with
/// (pointers at `011ca244` and `011ca248`).
const SPECIAL_FORM_A: u32 = 0x011c_a244;
const SPECIAL_FORM_B: u32 = 0x011c_a248;

// Translated from 004a60f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UseWeaponActorPackageData::GetPackageType` (Xbox PDB): 0x10.
pub fn use_weapon_actor_package_data_get_package_type(
    _e: &mut Engine,
    _this: Ptr<UseWeaponActorPackageData>,
) -> u32 {
    0x10
}

// Translated from 0067caa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseWeaponPackageData::TESUseWeaponPackageData` (Xbox PDB): the
/// `TESPackageData` base (`0067ad20`), the vtable, no target, location or
/// weapon, every flag clear but `bCrouch` and `bHoldFire`, a burst count
/// and minimum volley of 1 shot, a maximum of 6, a minimum wait of 1.0 and
/// a maximum wait of the `float` at `010162c0`. Returns `this`.
pub fn tes_use_weapon_package_data_tes_use_weapon_package_data(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
) -> Ptr<TESUseWeaponPackageData> {
    e.call(PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), USE_WEAPON_DATA_VTABLE);
    e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
    e.set(this, TESUseWeaponPackageData::pTargetLocation, 0);
    e.set(this, TESUseWeaponPackageData::pWeapon, 0);
    e.set(this, TESUseWeaponPackageData::bAlwaysHit, 0);
    e.set(this, TESUseWeaponPackageData::bDoNoDamage, 0);
    e.set(this, TESUseWeaponPackageData::bCrouch, 1);
    e.set(this, TESUseWeaponPackageData::bHoldFire, 1);
    e.set(this, TESUseWeaponPackageData::bVolleyFire, 0);
    e.set(this, TESUseWeaponPackageData::bRepeatFire, 0);
    e.set(this, TESUseWeaponPackageData::sBurstCount, 1);
    e.set(this, TESUseWeaponPackageData::sVolleyShotsMin, 1);
    e.set(this, TESUseWeaponPackageData::sVolleyShotsMax, 6);
    // FLD1; FSTP.
    e.set(
        this,
        TESUseWeaponPackageData::fVolleyWaitMin,
        1.0f32.to_bits(),
    );
    let wait_max = x87_quiet_bits(e.global::<u32>(DEFAULT_VOLLEY_WAIT_MAX));
    e.set(this, TESUseWeaponPackageData::fVolleyWaitMax, wait_max);
    this
}

// Translated from 0067cb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseWeaponPackageData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor (`0067ccd0`), then frees `this` (`00401030`) when bit 0 of
/// `flags` is set. Returns `this`.
pub fn tes_use_weapon_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    flags: u32,
) -> Ptr<TESUseWeaponPackageData> {
    tes_use_weapon_package_data_destructor(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0067cb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `TESUseWeaponPackageData` constructor from the record's data
/// (`PACK_USE_WEAPON_DATA_PKW3`, Xbox PDB, 0x18 bytes, passed by value as
/// six words; `RET 0x18`): the `TESPackageData` base (`0067ad20`), the
/// vtable, no target or location, then the flag bytes, the three counts,
/// the two waits (`FLD`/`FSTP`) and the weapon word copied as they are.
/// Returns `this`.
pub fn fn_0067cb70(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    flags_low: u32,
    flags_high_and_burst: u32,
    volley_shots: u32,
    wait_min: u32,
    wait_max: u32,
    weapon: u32,
) -> Ptr<TESUseWeaponPackageData> {
    e.call(PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), USE_WEAPON_DATA_VTABLE);
    e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
    e.set(this, TESUseWeaponPackageData::pTargetLocation, 0);
    e.set(this, TESUseWeaponPackageData::pWeapon, weapon);
    let [always_hit, no_damage, crouch, hold_fire] = flags_low.to_le_bytes();
    e.set(this, TESUseWeaponPackageData::bAlwaysHit, always_hit);
    e.set(this, TESUseWeaponPackageData::bDoNoDamage, no_damage);
    e.set(this, TESUseWeaponPackageData::bCrouch, crouch);
    e.set(this, TESUseWeaponPackageData::bHoldFire, hold_fire);
    let [volley_fire, repeat_fire, _, _] = flags_high_and_burst.to_le_bytes();
    e.set(this, TESUseWeaponPackageData::bVolleyFire, volley_fire);
    e.set(this, TESUseWeaponPackageData::bRepeatFire, repeat_fire);
    let burst = (flags_high_and_burst >> 16) as u16;
    e.set(this, TESUseWeaponPackageData::sBurstCount, burst);
    e.set(
        this,
        TESUseWeaponPackageData::sVolleyShotsMin,
        volley_shots as u16,
    );
    e.set(
        this,
        TESUseWeaponPackageData::sVolleyShotsMax,
        (volley_shots >> 16) as u16,
    );
    e.set(
        this,
        TESUseWeaponPackageData::fVolleyWaitMin,
        x87_quiet_bits(wait_min),
    );
    e.set(
        this,
        TESUseWeaponPackageData::fVolleyWaitMax,
        x87_quiet_bits(wait_max),
    );
    this
}

// Translated from 0067ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067ccd0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESUseWeaponPackageData::~TESUseWeaponPackageData` (Xbox PDB): stores
/// the vtable, deletes the attack target (`007b3fa0` with 1, the engine map
/// placing that folded body in `racesexmenu.cpp`) and the target location
/// (`00670b30` with 1, named `PackageLocation::~PackageLocation` by the
/// map) when there is one, clearing each pointer, then runs the
/// `TESPackageData` destructor (`0067ad70`).
pub fn tes_use_weapon_package_data_destructor(e: &mut Engine, this: Ptr<TESUseWeaponPackageData>) {
    e.mem.set_u32(this.addr(), USE_WEAPON_DATA_VTABLE);
    let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
    if target != 0 {
        e.call(0x007b_3fa0, &args![target, 1u32]);
    }
    e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
    let location = e.get(this, TESUseWeaponPackageData::pTargetLocation);
    if location != 0 {
        e.call(0x0067_0b30, &args![location, 1u32]);
    }
    e.set(this, TESUseWeaponPackageData::pTargetLocation, 0);
    e.call(PACKAGE_DATA_DESTRUCT, &args![this]);
}

// Translated from 0067d1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067d1c0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESUseWeaponPackageData::InitItem` (Xbox PDB), `package` its package
/// (`RET 4`). Without an attack target, the package's location
/// (`0055b980` on `package`) is asked for its type
/// (`PackageLocation::GetLocType`, `00678ca0`) and, for type 0, its
/// reference (`PackageLocation::GetLocReference`, `0067f390`). When that
/// reference's form (`007af430`) is one of the two forms `011ca244` and
/// `011ca248` point at, its extra data (`005d43c0`) is asked for an
/// `ExtraTarget` reference (`ExtraDataList::GetXTarget`, `00421b40`); with
/// one, a new 0x10-byte `PackageTarget` (`00401000`, constructed by
/// `0067ff70` when the allocation succeeds, else null) becomes the attack
/// target, of type 0 (`006800b0`) and with that reference (`00680110`).
/// Otherwise an attack target that exists runs its own `InitItem`
/// (`006803b0` with `package`). Finally the weapon id is looked up and cast
/// to `TESObjectWEAP` (`004839c0`, `__RTDynamicCast`) in place.
pub fn tes_use_weapon_package_data_init_item(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    package: u32,
) {
    let location = e.call(0x0055_b980, &args![package]).u32();
    let mut init_target = true;
    if e.get(this, TESUseWeaponPackageData::pAttackTarget) == 0
        && location != 0
        && e.call(0x0067_8ca0, &args![location]).u32() == 0
    {
        let reference = e.call(0x0067_f390, &args![location]).u32();
        if reference != 0 {
            let form = e.call(0x007a_f430, &args![reference]).u32();
            if form == e.global::<u32>(SPECIAL_FORM_A) || form == e.global::<u32>(SPECIAL_FORM_B) {
                let extra = e.call(0x005d_43c0, &args![reference]).u32();
                if extra != 0 {
                    let target_ref = e.call(0x0042_1b40, &args![extra]).u32();
                    if target_ref != 0 {
                        let block = e.call(ALLOCATE, &args![0x10u32]).u32();
                        let target = if block != 0 {
                            e.call(0x0067_ff70, &args![block]).u32()
                        } else {
                            0
                        };
                        e.set(this, TESUseWeaponPackageData::pAttackTarget, target);
                        let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
                        e.call(0x0068_00b0, &args![target, 0u32]);
                        let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
                        e.call(0x0068_0110, &args![target, target_ref]);
                        init_target = false;
                    }
                }
            }
        }
    }
    let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
    if target != 0 && init_target {
        e.call(0x0068_03b0, &args![target, package]);
    }
    let weapon_id = e.get(this, TESUseWeaponPackageData::pWeapon);
    let form = e.call(LOOKUP_FORM, &args![weapon_id]).u32();
    let weapon = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_WEAP, 0u32],
        )
        .u32();
    e.set(this, TESUseWeaponPackageData::pWeapon, weapon);
}

// Translated from 0067d8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UseWeaponActorPackageData`'s constructor: the `ActorPackageData` base
/// (`0067d920`), the vtable `01068a88`, no attack target. Returns `this`.
pub fn fn_0067d8f0(
    e: &mut Engine,
    this: Ptr<UseWeaponActorPackageData>,
) -> Ptr<UseWeaponActorPackageData> {
    fn_0067d920(e, this.cast());
    e.mem.set_u32(this.addr(), USE_WEAPON_ACTOR_DATA_VTABLE);
    e.set(this, UseWeaponActorPackageData::pAttackTarget, 0);
    this
}

// Translated from 0067d920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ActorPackageData` constructor: stores the base vtable `01068aa4`.
/// Returns `this`.
pub fn fn_0067d920(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ACTOR_PACKAGE_DATA_VTABLE);
    this
}

// Translated from 0067d970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ActorPackageData` destructor body: stores the base vtable
/// `01068aa4`.
pub fn fn_0067d970(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ACTOR_PACKAGE_DATA_VTABLE);
}

// Translated from 0067d990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UseWeaponActorPackageData`'s scalar deleting destructor: the
/// destructor body (`0067d9c0`), then frees `this` (`00401030`) when bit 0
/// of `flags` is set. Returns `this`.
pub fn fn_0067d990(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0067d9c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0067d9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UseWeaponActorPackageData::HandleDeletedReference` (Xbox PDB): clears
/// the attack target when it is `reference`; `RET 4`.
pub fn use_weapon_actor_package_data_handle_deleted_reference(
    e: &mut Engine,
    this: Ptr<UseWeaponActorPackageData>,
    reference: u32,
) {
    let target = e.get(this, UseWeaponActorPackageData::pAttackTarget);
    let kept = if target == reference { 0 } else { target };
    e.set(this, UseWeaponActorPackageData::pAttackTarget, kept);
}

/// The RTTI type descriptors of `TESPackageData` and
/// `TESUseWeaponPackageData` (read from the exe).
const TYPE_TES_PACKAGE_DATA: u32 = 0x0119_bdec;
const TYPE_TES_USE_WEAPON_PACKAGE_DATA: u32 = 0x0119_bf00;
/// `PackageTarget::PackageTarget` (engine map, `0067ff70`), `::Copy`
/// (`006801a0`, `RET 4`), `::Compare` (`00680220`, `RET 4`), `::Save`
/// (`006802f0`, `RET 4`: the chunk type), `::SaveGame` (`00680720`),
/// `::LoadGame` (`006807b0`) and `::InitLoadGame` (`00680830`, each `RET
/// 4`: the buffer), all `thiscall`; `007b3fa0` (`thiscall`, `RET 4`) deletes
/// a target when given 1.
const TARGET_CONSTRUCT: u32 = 0x0067_ff70;
const TARGET_COPY: u32 = 0x0068_01a0;
const TARGET_COMPARE: u32 = 0x0068_0220;
const TARGET_SAVE: u32 = 0x0068_02f0;
const TARGET_SAVE_GAME: u32 = 0x0068_0720;
const TARGET_LOAD_GAME: u32 = 0x0068_07b0;
const TARGET_INIT_LOAD_GAME: u32 = 0x0068_0830;
const TARGET_DELETE: u32 = 0x007b_3fa0;
/// `PackageLocation::PackageLocation` (engine map, `0067f030`), `::Copy`
/// (`0067f4d0`, `RET 8`: the other, a word), `::Compare` (`0067f600`),
/// `::Save` (`0067f6c0`), `::SaveGame` (`0067fb60`), `::LoadGame`
/// (`0067fbe0`) and `::InitLoadGame` (`0067fc50`, each `RET 4`), all
/// `thiscall`; `00670b30` (`RET 4`) deletes a location when given 1.
const LOCATION_CONSTRUCT: u32 = 0x0067_f030;
const LOCATION_COPY: u32 = 0x0067_f4d0;
const LOCATION_COMPARE: u32 = 0x0067_f600;
const LOCATION_SAVE: u32 = 0x0067_f6c0;
const LOCATION_SAVE_GAME: u32 = 0x0067_fb60;
const LOCATION_LOAD_GAME: u32 = 0x0067_fbe0;
const LOCATION_INIT_LOAD_GAME: u32 = 0x0067_fc50;
const LOCATION_DELETE: u32 = 0x0067_0b30;
/// `00408860` (`cdecl`, a `float`; answer in `ST0`): its absolute value.
const FLOAT_ABS: u32 = 0x0040_8860;
/// `0.01` (`double`): the volley wait tolerance of `Compare`.
const WAIT_TOLERANCE: u32 = 0x0103_1148;
/// `0084e3a0` (`thiscall`): a form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `00403d30` (`cdecl`: destination, value, count): fills bytes.
const FILL_BYTES: u32 = 0x0040_3d30;
/// `00401500` (no parameters): whether saved data is byte-swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1500;
/// `0067ca10` (`thiscall`, no stack word): byte-swaps the `PKW3` record.
const RECORD_SWAP: u32 = 0x0067_ca10;
/// `TESForm::__AddChunkData` (engine map, `00485990`, `cdecl`: type, data,
/// size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// The save buffer's write (`00865e50`, `thiscall`: data, size, flag),
/// `BGSSaveGameBuffer::SaveFormID_ov2` (engine map, `00865df0`, `RET 8`:
/// form, flag), the load buffer's read (`00864980`: destination, size) and
/// `BGSLoadGameBuffer::LoadFormID` (engine map, `008648a0`, no stack word).
const SAVE_WRITE: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const LOAD_READ: u32 = 0x0086_4980;
const LOAD_FORM_ID: u32 = 0x0086_48a0;

/// `dynamic_cast` of `other` to `TESUseWeaponPackageData`.
fn cast_use_weapon(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_TES_PACKAGE_DATA,
            TYPE_TES_USE_WEAPON_PACKAGE_DATA,
            0u32
        ],
    )
    .u32()
}

/// A new object of `size` bytes from `00401000` constructed by
/// `construct`; 0 when the allocation fails.
fn new_object(e: &mut Engine, size: u32, construct: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![size]).u32();
    if block != 0 {
        e.call(construct, &args![block]).u32()
    } else {
        0
    }
}

/// The flag bytes (+0xc to +0x11) and 16-bit counts (+0x12 to +0x16).
const FLAG_BYTES: [u32; 6] = [0xc, 0xd, 0xe, 0xf, 0x10, 0x11];
const COUNTS: [u32; 3] = [0x12, 0x14, 0x16];

// Translated from 0067cc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseWeaponPackageData`'s constructor from values (`thiscall`, `RET
/// 0x14`; the engine map gives it no name): the `TESPackageData` base
/// (`0067ad20`), the vtable `01068a64`, no target, location or weapon; the
/// four bytes of `flags` into +0xc to +0xf; the low byte of `fire` into
/// `bVolleyFire` and its high 16 bits into `sBurstCount`, `bRepeatFire`
/// being whether those are 0; the two halves of `shots` into
/// `sVolleyShotsMin` and `sVolleyShotsMax`; the two waits (`float`s,
/// loaded and stored through the x87). Returns `this`.
pub fn fn_0067cc20(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    flags: u32,
    fire: u32,
    shots: u32,
    wait_min: u32,
    wait_max: u32,
) -> Ptr<TESUseWeaponPackageData> {
    e.call(PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), USE_WEAPON_DATA_VTABLE);
    e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
    e.set(this, TESUseWeaponPackageData::pTargetLocation, 0);
    e.set(this, TESUseWeaponPackageData::pWeapon, 0);
    for (i, offset) in FLAG_BYTES[..4].iter().enumerate() {
        e.mem.set_u8(this.addr() + offset, (flags >> (8 * i)) as u8);
    }
    e.set(this, TESUseWeaponPackageData::bVolleyFire, fire as u8);
    let burst = (fire >> 16) as u16;
    e.set(
        this,
        TESUseWeaponPackageData::bRepeatFire,
        u8::from(burst == 0),
    );
    e.set(this, TESUseWeaponPackageData::sBurstCount, burst);
    e.set(this, TESUseWeaponPackageData::sVolleyShotsMin, shots as u16);
    e.set(
        this,
        TESUseWeaponPackageData::sVolleyShotsMax,
        (shots >> 16) as u16,
    );
    e.set(
        this,
        TESUseWeaponPackageData::fVolleyWaitMin,
        x87_quiet_bits(wait_min),
    );
    e.set(
        this,
        TESUseWeaponPackageData::fVolleyWaitMax,
        x87_quiet_bits(wait_max),
    );
    this
}

// Translated from 0067cd90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067cd90 - the exception-unwinding frame (FS:[0]) around the target's and the location's allocations is not translated.
/// `TESUseWeaponPackageData::Copy` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): nothing unless `other` is a
/// `TESUseWeaponPackageData`. The target is deleted (`007b3fa0` with 1) and
/// cleared, and, when the other has one, a new 0x10-byte target
/// (`0067ff70`) takes a copy of it (`006801a0`); the weapon is copied; the
/// location likewise (`00670b30`; a new 0xc-byte one, `0067f030`,
/// `0067f4d0` with 0); then the flags, counts and waits (the waits through
/// the x87).
pub fn tes_use_weapon_package_data_copy(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    other: Ptr,
) {
    let cast = cast_use_weapon(e, other);
    if cast == 0 {
        return;
    }
    let other = Ptr::<TESUseWeaponPackageData>::new(cast);
    let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
    if target != 0 {
        e.call(TARGET_DELETE, &args![target, 1u32]);
    }
    e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
    if e.get(other, TESUseWeaponPackageData::pAttackTarget) != 0 {
        let target = new_object(e, 0x10, TARGET_CONSTRUCT);
        e.set(this, TESUseWeaponPackageData::pAttackTarget, target);
        let theirs = e.get(other, TESUseWeaponPackageData::pAttackTarget);
        let mine = e.get(this, TESUseWeaponPackageData::pAttackTarget);
        e.call(TARGET_COPY, &args![mine, theirs]);
    }
    let weapon = e.get(other, TESUseWeaponPackageData::pWeapon);
    e.set(this, TESUseWeaponPackageData::pWeapon, weapon);
    let location = e.get(this, TESUseWeaponPackageData::pTargetLocation);
    if location != 0 {
        e.call(LOCATION_DELETE, &args![location, 1u32]);
    }
    e.set(this, TESUseWeaponPackageData::pTargetLocation, 0);
    if e.get(other, TESUseWeaponPackageData::pTargetLocation) != 0 {
        let location = new_object(e, 0xc, LOCATION_CONSTRUCT);
        e.set(this, TESUseWeaponPackageData::pTargetLocation, location);
        let theirs = e.get(other, TESUseWeaponPackageData::pTargetLocation);
        let mine = e.get(this, TESUseWeaponPackageData::pTargetLocation);
        e.call(LOCATION_COPY, &args![mine, theirs, 0u32]);
    }
    for offset in FLAG_BYTES {
        let value = e.mem.u8(cast + offset);
        e.mem.set_u8(this.addr() + offset, value);
    }
    for offset in COUNTS {
        let value = e.mem.u16(cast + offset);
        e.mem.set_u16(this.addr() + offset, value);
    }
    for offset in [0x18u32, 0x1c] {
        let value = x87_quiet_bits(e.mem.u32(cast + offset));
        e.mem.set_u32(this.addr() + offset, value);
    }
}

/// `Compare`'s wait test: whether `|mine - theirs|` (x87 `FSUB` stored as a
/// `float`, then `00408860`) is above the `0.01` at `01031148` (`FCOMP`,
/// `TEST AH,0x41`: a NaN is not above).
fn wait_differs(e: &mut Engine, mine: u32, theirs: u32) -> bool {
    let difference = x87_store32(x87_sub(x87_load32(mine), x87_m32(theirs)));
    let size = e.call(FLOAT_ABS, &args![difference]).f64();
    let tolerance: f64 = e.global(WAIT_TOLERANCE);
    size > tolerance
}

// Translated from 0067cfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseWeaponPackageData::Compare` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): true (they differ) when `other` is not a
/// `TESUseWeaponPackageData`, the weapons differ, the targets differ
/// (`00680220`, or only the other has one), the locations differ
/// (`0067f600`, likewise), any flag byte or count differs, or either wait
/// differs by more than 0.01.
pub fn tes_use_weapon_package_data_compare(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    other: Ptr,
) -> bool {
    let cast = cast_use_weapon(e, other);
    if cast == 0 {
        return true;
    }
    let other = Ptr::<TESUseWeaponPackageData>::new(cast);
    if e.get(this, TESUseWeaponPackageData::pWeapon)
        != e.get(other, TESUseWeaponPackageData::pWeapon)
    {
        return true;
    }
    let mine = e.get(this, TESUseWeaponPackageData::pAttackTarget);
    let theirs = e.get(other, TESUseWeaponPackageData::pAttackTarget);
    if mine != 0 {
        if e.call(TARGET_COMPARE, &args![mine, theirs]).bool() {
            return true;
        }
    } else if theirs != 0 {
        return true;
    }
    let mine = e.get(this, TESUseWeaponPackageData::pTargetLocation);
    let theirs = e.get(other, TESUseWeaponPackageData::pTargetLocation);
    if mine != 0 {
        if e.call(LOCATION_COMPARE, &args![mine, theirs]).bool() {
            return true;
        }
    } else if theirs != 0 {
        return true;
    }
    for offset in FLAG_BYTES {
        if e.mem.u8(this.addr() + offset) != e.mem.u8(cast + offset) {
            return true;
        }
    }
    for offset in COUNTS {
        if e.mem.u16(this.addr() + offset) != e.mem.u16(cast + offset) {
            return true;
        }
    }
    for offset in [0x18u32, 0x1c] {
        let mine = e.mem.u32(this.addr() + offset);
        let theirs = e.mem.u32(cast + offset);
        if wait_differs(e, mine, theirs) {
            return true;
        }
    }
    false
}

// Translated from 0067d350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseWeaponPackageData::Save` (Xbox PDB name in the engine map; no
/// stack word): a 0x18-byte `PKW3` record (`00485990`; a stack block
/// cleared by `00403d30`) of the six flag bytes, the three counts, the two
/// waits (through the x87) and the weapon's id (`0084e3a0`, 0 without
/// one), byte-swapped before and after (`0067ca10`) when `00401500` says
/// so; then the target as `PTD2` (`006802f0`) and the location as `PLD2`
/// (`0067f6c0`) when there are.
pub fn tes_use_weapon_package_data_save(e: &mut Engine, this: Ptr<TESUseWeaponPackageData>) {
    e.with_stack(0x18, |e, record| {
        let record = record.addr();
        e.call(FILL_BYTES, &args![record, 0u32, 0x18u32]);
        for (i, offset) in FLAG_BYTES.iter().enumerate() {
            let value = e.mem.u8(this.addr() + offset);
            e.mem.set_u8(record + i as u32, value);
        }
        for (i, offset) in COUNTS.iter().enumerate() {
            let value = e.mem.u16(this.addr() + offset);
            e.mem.set_u16(record + 6 + 2 * i as u32, value);
        }
        let wait_min = x87_quiet_bits(e.get(this, TESUseWeaponPackageData::fVolleyWaitMin));
        e.mem.set_u32(record + 0xc, wait_min);
        let wait_max = x87_quiet_bits(e.get(this, TESUseWeaponPackageData::fVolleyWaitMax));
        e.mem.set_u32(record + 0x10, wait_max);
        let weapon = e.get(this, TESUseWeaponPackageData::pWeapon);
        let id = if weapon != 0 {
            e.call(FORM_ID, &args![weapon]).u32()
        } else {
            0
        };
        e.mem.set_u32(record + 0x14, id);
        if e.call(NEEDS_SWAP, &args![]).bool() {
            e.call(RECORD_SWAP, &args![record]);
        }
        e.call(ADD_CHUNK_DATA, &args![0x3357_4b50u32, record, 0x18u32]);
        if e.call(NEEDS_SWAP, &args![]).bool() {
            e.call(RECORD_SWAP, &args![record]);
        }
    });
    let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
    if target != 0 {
        e.call(TARGET_SAVE, &args![target, 0x3244_5450u32]);
    }
    let location = e.get(this, TESUseWeaponPackageData::pTargetLocation);
    if location != 0 {
        e.call(LOCATION_SAVE, &args![location, 0x3244_4c50u32]);
    }
}

// Translated from 0067d470 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067d470 - the exception-unwinding frame (FS:[0]) around the target's allocation is not translated.
/// `TESUseWeaponPackageData::SetAttackTarget` (Xbox PDB name in the engine
/// map; `thiscall`, `RET 4`): a null `target` deletes the target
/// (`007b3fa0` with 1) and clears it; otherwise the target, made when
/// missing (0x10 bytes, `0067ff70`), takes a copy of `target`
/// (`006801a0`).
pub fn tes_use_weapon_package_data_set_attack_target(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    target: u32,
) {
    if target == 0 {
        let mine = e.get(this, TESUseWeaponPackageData::pAttackTarget);
        if mine != 0 {
            e.call(TARGET_DELETE, &args![mine, 1u32]);
        }
        e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
        return;
    }
    if e.get(this, TESUseWeaponPackageData::pAttackTarget) == 0 {
        let made = new_object(e, 0x10, TARGET_CONSTRUCT);
        e.set(this, TESUseWeaponPackageData::pAttackTarget, made);
    }
    let mine = e.get(this, TESUseWeaponPackageData::pAttackTarget);
    e.call(TARGET_COPY, &args![mine, target]);
}

/// The sizes of the saved fields from +0xc on, in order.
const SAVED_FIELDS: [(u32, u32); 11] = [
    (0xc, 1),
    (0xd, 1),
    (0xe, 1),
    (0xf, 1),
    (0x10, 1),
    (0x11, 1),
    (0x12, 2),
    (0x14, 2),
    (0x16, 2),
    (0x18, 4),
    (0x1c, 4),
];

// Translated from 0067d550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseWeaponPackageData::SaveGame` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`, the save buffer): the fields from +0xc to +0x1f
/// (`00865e50`, each its own size, flag 0), the weapon (`SaveFormID`, flag
/// 0), then a byte saying whether there is a target, the target's save
/// (`00680720`) when there is, and the same for the location
/// (`0067fb60`).
pub fn tes_use_weapon_package_data_save_game(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    buffer: u32,
) {
    for (offset, size) in SAVED_FIELDS {
        e.call(SAVE_WRITE, &args![buffer, this.addr() + offset, size, 0u32]);
    }
    let weapon = e.get(this, TESUseWeaponPackageData::pWeapon);
    e.call(SAVE_FORM_ID, &args![buffer, weapon, 0u32]);
    e.with_stack(4, |e, flags| {
        let has_target = flags.addr();
        let has_location = flags.addr() + 1;
        let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
        e.mem.set_u8(has_target, u8::from(target != 0));
        e.call(SAVE_WRITE, &args![buffer, has_target, 1u32, 0u32]);
        if e.mem.u8(has_target) != 0 {
            let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
            e.call(TARGET_SAVE_GAME, &args![target, buffer]);
        }
        let location = e.get(this, TESUseWeaponPackageData::pTargetLocation);
        e.mem.set_u8(has_location, u8::from(location != 0));
        e.call(SAVE_WRITE, &args![buffer, has_location, 1u32, 0u32]);
        if e.mem.u8(has_location) != 0 {
            let location = e.get(this, TESUseWeaponPackageData::pTargetLocation);
            e.call(LOCATION_SAVE_GAME, &args![location, buffer]);
        }
    });
}

// Translated from 0067d6b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067d6b0 - the exception-unwinding frame (FS:[0]) around the target's and the location's allocations is not translated.
/// `TESUseWeaponPackageData::LoadGame` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`, the load buffer): the fields from +0xc to +0x1f
/// (`00864980`, each its own size), the weapon (`LoadFormID`, the form
/// looked up, `004839c0`, and cast to `TESObjectWEAP`), then a byte (from
/// 0) saying whether a target follows: a new 0x10-byte target (`0067ff70`)
/// loads it (`006807b0`); and the same for the location (0xc bytes,
/// `0067f030`, `0067fbe0`).
pub fn tes_use_weapon_package_data_load_game(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    buffer: u32,
) {
    for (offset, size) in SAVED_FIELDS {
        e.call(LOAD_READ, &args![buffer, this.addr() + offset, size]);
    }
    let id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let weapon = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_WEAP, 0u32],
        )
        .u32();
    e.set(this, TESUseWeaponPackageData::pWeapon, weapon);
    e.with_stack(4, |e, flags| {
        let has_target = flags.addr();
        let has_location = flags.addr() + 1;
        e.mem.set_u8(has_target, 0);
        e.call(LOAD_READ, &args![buffer, has_target, 1u32]);
        if e.mem.u8(has_target) != 0 {
            let target = new_object(e, 0x10, TARGET_CONSTRUCT);
            e.set(this, TESUseWeaponPackageData::pAttackTarget, target);
            let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
            e.call(TARGET_LOAD_GAME, &args![target, buffer]);
        }
        e.mem.set_u8(has_location, 0);
        e.call(LOAD_READ, &args![buffer, has_location, 1u32]);
        if e.mem.u8(has_location) != 0 {
            let location = new_object(e, 0xc, LOCATION_CONSTRUCT);
            e.set(this, TESUseWeaponPackageData::pTargetLocation, location);
            let location = e.get(this, TESUseWeaponPackageData::pTargetLocation);
            e.call(LOCATION_LOAD_GAME, &args![location, buffer]);
        }
    });
}

// Translated from 0067d8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseWeaponPackageData::InitLoadGame` (Xbox PDB name in the engine
/// map; `thiscall`, `RET 4`, the buffer): the target's (`00680830`) and the
/// location's (`0067fc50`) when there are.
pub fn tes_use_weapon_package_data_init_load_game(
    e: &mut Engine,
    this: Ptr<TESUseWeaponPackageData>,
    buffer: u32,
) {
    let target = e.get(this, TESUseWeaponPackageData::pAttackTarget);
    if target != 0 {
        e.call(TARGET_INIT_LOAD_GAME, &args![target, buffer]);
    }
    let location = e.get(this, TESUseWeaponPackageData::pTargetLocation);
    if location != 0 {
        e.call(LOCATION_INIT_LOAD_GAME, &args![location, buffer]);
    }
}

// Translated from 0067d940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackageData`'s scalar deleting destructor (`thiscall`, `RET 4`;
/// the engine map gives it no name): the destructor body (`0067d970`), then
/// frees `this` (`00401030`) when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0067d940(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0067d970(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0067_cc20,
            fn_0067cc20(
                Ptr<TESUseWeaponPackageData>,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<TESUseWeaponPackageData>
        ),
        entry!(
            0x0067_cd90,
            tes_use_weapon_package_data_copy(Ptr<TESUseWeaponPackageData>, Ptr)
        ),
        entry!(
            0x0067_cfc0,
            tes_use_weapon_package_data_compare(Ptr<TESUseWeaponPackageData>, Ptr) -> bool
        ),
        entry!(
            0x0067_d350,
            tes_use_weapon_package_data_save(Ptr<TESUseWeaponPackageData>)
        ),
        entry!(
            0x0067_d470,
            tes_use_weapon_package_data_set_attack_target(Ptr<TESUseWeaponPackageData>, u32)
        ),
        entry!(
            0x0067_d550,
            tes_use_weapon_package_data_save_game(Ptr<TESUseWeaponPackageData>, u32)
        ),
        entry!(
            0x0067_d6b0,
            tes_use_weapon_package_data_load_game(Ptr<TESUseWeaponPackageData>, u32)
        ),
        entry!(
            0x0067_d8b0,
            tes_use_weapon_package_data_init_load_game(Ptr<TESUseWeaponPackageData>, u32)
        ),
        entry!(0x0067_d940, fn_0067d940(Ptr, u32) -> Ptr),
        entry!(0x0067_d9c0, fn_0067d9c0(Ptr)),
        entry!(
            0x004a_60f0,
            use_weapon_actor_package_data_get_package_type(Ptr<UseWeaponActorPackageData>) -> u32
        ),
        entry!(
            0x0067_caa0,
            tes_use_weapon_package_data_tes_use_weapon_package_data(
                Ptr<TESUseWeaponPackageData>,
            )
                -> Ptr<TESUseWeaponPackageData>
        ),
        entry!(
            0x0067_cb40,
            tes_use_weapon_package_data_scalar_deleting_destructor(
                Ptr<TESUseWeaponPackageData>,
                u32,
            )
                -> Ptr<TESUseWeaponPackageData>
        ),
        entry!(
            0x0067_cb70,
            fn_0067cb70(
                Ptr<TESUseWeaponPackageData>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<TESUseWeaponPackageData>
        ),
        entry!(
            0x0067_ccd0,
            tes_use_weapon_package_data_destructor(Ptr<TESUseWeaponPackageData>)
        ),
        entry!(
            0x0067_d1c0,
            tes_use_weapon_package_data_init_item(Ptr<TESUseWeaponPackageData>, u32)
        ),
        entry!(
            0x0067_d8f0,
            fn_0067d8f0(Ptr<UseWeaponActorPackageData>) -> Ptr<UseWeaponActorPackageData>
        ),
        entry!(0x0067_d920, fn_0067d920(Ptr) -> Ptr),
        entry!(0x0067_d970, fn_0067d970(Ptr)),
        entry!(0x0067_d990, fn_0067d990(Ptr, u32) -> Ptr),
        entry!(
            0x0067_d9e0,
            use_weapon_actor_package_data_handle_deleted_reference(
                Ptr<UseWeaponActorPackageData>,
                u32,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_0067d9c0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0067_d970, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0067_d9c0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0067_d970)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_8a88);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_8a88,
            "the vtable was stored before the call"
        );
    }

    mod b023 {
        use super::super::*;

        fn ret(eax: u32) -> Ret {
            Ret {
                eax,
                ..Ret::default()
            }
        }

        fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(a, _)| *a == addr)
                .map(|(_, w)| w.clone())
                .collect()
        }

        fn quiet(e: &mut Engine, addrs: &[u32]) {
            for a in addrs {
                e.register(*a, |_, _| Ret::default());
            }
        }

        #[test]
        fn package_type_is_0x10() {
            let mut e = Engine::new();
            assert_eq!(e.call(0x004a_60f0, &[0x1234]).u32(), 0x10);
        }

        #[test]
        fn default_constructor_values() {
            let mut e = Engine::new();
            quiet(&mut e, &[PACKAGE_DATA_CONSTRUCT]);
            e.map(DEFAULT_VOLLEY_WAIT_MAX & !0xfff, 0x1000);
            // A signalling NaN comes out quiet (FLD/FSTP).
            e.set_global(DEFAULT_VOLLEY_WAIT_MAX, 0x7f80_0001u32);
            let this: Ptr<TESUseWeaponPackageData> = e.new_object();
            e.mem.write(this.addr(), &[0xcd; 0x24]);
            e.call_log = Some(vec![]);
            let r = e.call(0x0067_caa0, &args![this]).u32();
            assert_eq!(r, this.addr());
            assert_eq!(
                calls_to(e.call_log.as_ref().unwrap(), PACKAGE_DATA_CONSTRUCT),
                vec![vec![this.addr()]]
            );
            let t = this.addr();
            assert_eq!(e.mem.u32(t), USE_WEAPON_DATA_VTABLE);
            assert_eq!(e.mem.u32(t + 4), 0);
            assert_eq!(e.mem.u32(t + 8), 0);
            assert_eq!(e.mem.bytes(t + 0xc, 6), vec![0, 0, 1, 1, 0, 0]);
            assert_eq!(e.mem.u16(t + 0x12), 1);
            assert_eq!(e.mem.u16(t + 0x14), 1);
            assert_eq!(e.mem.u16(t + 0x16), 6);
            assert_eq!(e.mem.f32(t + 0x18), 1.0);
            assert_eq!(e.mem.u32(t + 0x1c), 0x7fc0_0001);
            assert_eq!(e.mem.u32(t + 0x20), 0);
        }

        #[test]
        fn data_constructor_copies_the_record() {
            let mut e = Engine::new();
            quiet(&mut e, &[PACKAGE_DATA_CONSTRUCT]);
            let this: Ptr<TESUseWeaponPackageData> = e.new_object();
            e.mem.write(this.addr(), &[0xcd; 0x24]);
            let words = [
                this.addr(),
                0x0403_0201,
                0x0007_0605,
                0x0009_0008,
                0x7f80_0002,
                2.5f32.to_bits(),
                0x0001_5cde,
            ];
            assert_eq!(e.call(0x0067_cb70, &words).u32(), this.addr());
            let t = this.addr();
            assert_eq!(e.mem.u32(t), USE_WEAPON_DATA_VTABLE);
            assert_eq!(e.mem.u32(t + 4), 0);
            assert_eq!(e.mem.u32(t + 8), 0);
            assert_eq!(e.mem.bytes(t + 0xc, 6), vec![1, 2, 3, 4, 5, 6]);
            assert_eq!(e.mem.u16(t + 0x12), 7);
            assert_eq!(e.mem.u16(t + 0x14), 8);
            assert_eq!(e.mem.u16(t + 0x16), 9);
            assert_eq!(e.mem.u32(t + 0x18), 0x7fc0_0002);
            assert_eq!(e.mem.f32(t + 0x1c), 2.5);
            assert_eq!(e.mem.u32(t + 0x20), 0x0001_5cde);
        }

        #[test]
        fn destructor_deletes_target_and_location() {
            let mut e = Engine::new();
            quiet(
                &mut e,
                &[0x007b_3fa0, 0x0067_0b30, PACKAGE_DATA_DESTRUCT, FREE],
            );
            let this: Ptr<TESUseWeaponPackageData> = e.new_object();
            let t = this.addr();
            for (target, location) in [(0x111u32, 0x222u32), (0, 0)] {
                e.set(this, TESUseWeaponPackageData::pAttackTarget, target);
                e.set(this, TESUseWeaponPackageData::pTargetLocation, location);
                e.call_log = Some(vec![]);
                e.call(0x0067_ccd0, &args![this]);
                let log = e.call_log.take().unwrap();
                let expect = |p: u32| if p != 0 { vec![vec![p, 1]] } else { vec![] };
                assert_eq!(calls_to(&log, 0x007b_3fa0), expect(target));
                assert_eq!(calls_to(&log, 0x0067_0b30), expect(location));
                assert_eq!(calls_to(&log, PACKAGE_DATA_DESTRUCT), vec![vec![t]]);
                assert_eq!(e.mem.u32(t), USE_WEAPON_DATA_VTABLE);
                assert_eq!(e.mem.u32(t + 4), 0);
                assert_eq!(e.mem.u32(t + 8), 0);
            }
            // The deleting destructor frees on bit 0 only.
            for (flags, frees) in [(0u32, false), (2, false), (3, true)] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(0x0067_cb40, &[t, flags]).u32(), t);
                let log = e.call_log.take().unwrap();
                assert_eq!(calls_to(&log, PACKAGE_DATA_DESTRUCT).len(), 1);
                assert_eq!(calls_to(&log, FREE).len(), usize::from(frees));
            }
        }

        /// Doubles for `InitItem`'s path to a new target: location 0x500
        /// of type 0, reference 0x600 whose form is `011ca248`'s, extra
        /// 0x700 with target reference 0x800, allocation 0x900.
        fn init_doubles(e: &mut Engine) {
            e.map(SPECIAL_FORM_A & !0xfff, 0x1000);
            e.set_global(SPECIAL_FORM_A, 0xa0u32);
            e.set_global(SPECIAL_FORM_B, 0xb0u32);
            e.register(0x0055_b980, |_, _| ret(0x500));
            e.register(0x0067_8ca0, |_, _| ret(0));
            e.register(0x0067_f390, |_, _| ret(0x600));
            e.register(0x007a_f430, |_, _| ret(0xb0));
            e.register(0x005d_43c0, |_, _| ret(0x700));
            e.register(0x0042_1b40, |_, _| ret(0x800));
            e.register(ALLOCATE, |_, _| ret(0x900));
            e.register(0x0067_ff70, |_, a| ret(a[0] + 1));
            quiet(e, &[0x0068_00b0, 0x0068_0110, 0x0068_03b0]);
            e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x1000));
            e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0] + 0x2000));
        }

        #[test]
        fn init_item_makes_a_target_from_the_location_reference() {
            let mut e = Engine::new();
            init_doubles(&mut e);
            let this: Ptr<TESUseWeaponPackageData> = e.new_object();
            e.set(this, TESUseWeaponPackageData::pWeapon, 0x42);
            e.call_log = Some(vec![]);
            e.call(0x0067_d1c0, &args![this, 0x0abcu32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, 0x0055_b980), vec![vec![0x0abc]]);
            assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x10]]);
            assert_eq!(e.get(this, TESUseWeaponPackageData::pAttackTarget), 0x901);
            assert_eq!(calls_to(&log, 0x0068_00b0), vec![vec![0x901, 0]]);
            assert_eq!(calls_to(&log, 0x0068_0110), vec![vec![0x901, 0x800]]);
            assert!(calls_to(&log, 0x0068_03b0).is_empty());
            assert_eq!(
                calls_to(&log, RT_DYNAMIC_CAST),
                vec![vec![0x1042, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_WEAP, 0]]
            );
            assert_eq!(e.get(this, TESUseWeaponPackageData::pWeapon), 0x3042);

            // A failed allocation leaves a null target, still set up.
            e.register(ALLOCATE, |_, _| ret(0));
            e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
            e.call_log = Some(vec![]);
            e.call(0x0067_d1c0, &args![this, 0x0abcu32]);
            let log = e.call_log.take().unwrap();
            assert!(calls_to(&log, 0x0067_ff70).is_empty());
            assert_eq!(calls_to(&log, 0x0068_00b0), vec![vec![0, 0]]);
        }

        #[test]
        fn init_item_otherwise_inits_an_existing_target() {
            let mut e = Engine::new();
            init_doubles(&mut e);
            let this: Ptr<TESUseWeaponPackageData> = e.new_object();
            // An existing target: no new one, its InitItem runs.
            e.set(this, TESUseWeaponPackageData::pAttackTarget, 0x333);
            e.call_log = Some(vec![]);
            e.call(0x0067_d1c0, &args![this, 0x0abcu32]);
            let log = e.call_log.take().unwrap();
            assert!(calls_to(&log, 0x0067_8ca0).is_empty());
            assert_eq!(calls_to(&log, 0x0068_03b0), vec![vec![0x333, 0x0abc]]);
            // Each failed test on the way leaves no target and no InitItem.
            let fails: [(u32, u32); 6] = [
                (0x0055_b980, 0),
                (0x0067_8ca0, 1),
                (0x0067_f390, 0),
                (0x007a_f430, 0xc0),
                (0x005d_43c0, 0),
                (0x0042_1b40, 0),
            ];
            for (addr, value) in fails {
                init_doubles(&mut e);
                e.register_double(addr, move |_, _| ret(value));
                e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
                e.call_log = Some(vec![]);
                e.call(0x0067_d1c0, &args![this, 0x0abcu32]);
                let log = e.call_log.take().unwrap();
                assert!(calls_to(&log, ALLOCATE).is_empty(), "{addr:08x}");
                assert!(calls_to(&log, 0x0068_03b0).is_empty());
                assert_eq!(e.get(this, TESUseWeaponPackageData::pAttackTarget), 0);
            }
            // The first special form also qualifies.
            init_doubles(&mut e);
            e.register(0x007a_f430, |_, _| ret(0xa0));
            e.set(this, TESUseWeaponPackageData::pAttackTarget, 0);
            e.call(0x0067_d1c0, &args![this, 0x0abcu32]);
            assert_eq!(e.get(this, TESUseWeaponPackageData::pAttackTarget), 0x901);
        }

        #[test]
        fn actor_data_constructors_and_destructors() {
            let mut e = Engine::new();
            quiet(&mut e, &[FREE]);
            let this: Ptr<UseWeaponActorPackageData> = e.new_object();
            let t = this.addr();
            e.mem.set_u32(t + 4, 0xcdcd_cdcd);
            assert_eq!(e.call(0x0067_d8f0, &[t]).u32(), t);
            assert_eq!(e.mem.u32(t), USE_WEAPON_ACTOR_DATA_VTABLE);
            assert_eq!(e.mem.u32(t + 4), 0);
            assert_eq!(e.call(0x0067_d920, &[t]).u32(), t);
            assert_eq!(e.mem.u32(t), ACTOR_PACKAGE_DATA_VTABLE);
            e.mem.set_u32(t, 0);
            e.call(0x0067_d970, &[t]);
            assert_eq!(e.mem.u32(t), ACTOR_PACKAGE_DATA_VTABLE);
            for (flags, frees) in [(0u32, false), (0xfffe, false), (1, true)] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(0x0067_d990, &[t, flags]).u32(), t);
                let log = e.call_log.take().unwrap();
                // The body stores the derived vtable, then the base one.
                assert_eq!(e.mem.u32(t), ACTOR_PACKAGE_DATA_VTABLE);
                assert_eq!(calls_to(&log, FREE).len(), usize::from(frees));
            }
        }

        #[test]
        fn handle_deleted_reference_clears_only_the_target() {
            let mut e = Engine::new();
            let this: Ptr<UseWeaponActorPackageData> = e.new_object();
            e.set(this, UseWeaponActorPackageData::pAttackTarget, 0x55);
            e.call(0x0067_d9e0, &args![this, 0x56u32]);
            assert_eq!(e.get(this, UseWeaponActorPackageData::pAttackTarget), 0x55);
            e.call(0x0067_d9e0, &args![this, 0x55u32]);
            assert_eq!(e.get(this, UseWeaponActorPackageData::pAttackTarget), 0);
        }
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    const SCRATCH: u32 = 0x0f00_0000;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Doubles: the cast answers `SCRATCH`, the target and location
    /// compares `SCRATCH + 4`, the swap test `SCRATCH + 8`; the
    /// allocations answer 0x7000 and the constructors their block + 1; the
    /// absolute value is exact.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(SCRATCH, 0x10);
        e.mem.map(0x0103_1000, 0x1000);
        e.set_global(WAIT_TOLERANCE, 0.01f64);
        e.register(RT_DYNAMIC_CAST, |e, a| {
            if a[3] == TYPE_TES_OBJECT_WEAP {
                word(a[0] + 2)
            } else {
                word(e.mem.u32(SCRATCH))
            }
        });
        e.register(TARGET_COMPARE, |e, _| word(e.mem.u32(SCRATCH + 4)));
        e.register(LOCATION_COMPARE, |e, _| word(e.mem.u32(SCRATCH + 4)));
        e.register(NEEDS_SWAP, |e, _| word(e.mem.u32(SCRATCH + 8)));
        e.register(ALLOCATE, |_, _| word(0x7000));
        e.register(TARGET_CONSTRUCT, |_, a| word(a[0] + 1));
        e.register(LOCATION_CONSTRUCT, |_, a| word(a[0] + 2));
        e.register(FLOAT_ABS, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).abs()),
            ..Ret::default()
        });
        e.register(FORM_ID, |_, a| word(a[0] + 1));
        e.register(LOAD_FORM_ID, |_, _| word(0x44));
        e.register(LOOKUP_FORM, |_, a| word(a[0] + 0x1000));
        // A read hands out 1 for every byte asked.
        e.register(LOAD_READ, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, 1);
            }
            Ret::default()
        });
        for addr in [
            PACKAGE_DATA_CONSTRUCT,
            TARGET_COPY,
            TARGET_SAVE,
            TARGET_SAVE_GAME,
            TARGET_LOAD_GAME,
            TARGET_INIT_LOAD_GAME,
            TARGET_DELETE,
            LOCATION_COPY,
            LOCATION_SAVE,
            LOCATION_SAVE_GAME,
            LOCATION_LOAD_GAME,
            LOCATION_INIT_LOAD_GAME,
            LOCATION_DELETE,
            FILL_BYTES,
            RECORD_SWAP,
            ADD_CHUNK_DATA,
            SAVE_WRITE,
            SAVE_FORM_ID,
            FREE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    #[test]
    fn constructor_spreads_the_values() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        e.mem.write(this, &[0xff; 0x24]);
        let words = [
            this,
            0x0403_0201,
            0x0007_ff05,
            0x0009_0008,
            1.5f32.to_bits(),
            0x7f80_0001,
        ];
        let (result, log) = run(&mut e, 0x0067_cc20, &words);
        assert_eq!(result.u32(), this);
        assert_eq!(log, vec![(PACKAGE_DATA_CONSTRUCT, vec![this])]);
        assert_eq!(e.mem.u32(this), USE_WEAPON_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.bytes(this + 0xc, 6), vec![1, 2, 3, 4, 5, 0]);
        assert_eq!(e.mem.u16(this + 0x12), 7);
        assert_eq!(e.mem.u16(this + 0x14), 8);
        assert_eq!(e.mem.u16(this + 0x16), 9);
        assert_eq!(e.mem.u32(this + 0x18), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x1c), 0x7fc0_0001);
        // A burst count of 0 sets bRepeatFire.
        let words = [this, 0, 0x0000_0005, 0, 0, 0];
        e.call(0x0067_cc20, &words);
        assert_eq!(e.mem.u8(this + 0x11), 1);
    }

    #[test]
    fn copy_rebuilds_target_and_location() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        let other = e.mem.alloc(0x24);
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(this + 8, 0x5100);
        let (_, log) = run(&mut e, 0x0067_cd90, &[this, other]);
        assert_eq!(log.len(), 1);
        e.mem.set_u32(SCRATCH, other);
        for i in 0..0x18 {
            e.mem.set_u8(other + 0xc + i, i as u8 + 1);
        }
        e.mem.set_u32(other + 0x18, 0x7f80_0002);
        e.mem.set_u32(other + 0x20, 0x9000);
        // The other has neither: both deleted.
        let (_, log) = run(&mut e, 0x0067_cd90, &[this, other]);
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (TARGET_DELETE, vec![0x5000, 1]),
                (LOCATION_DELETE, vec![0x5100, 1])
            ]
        );
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 8)), (0, 0));
        assert_eq!(e.mem.u32(this + 0x20), 0x9000);
        assert_eq!(e.mem.bytes(this + 0xc, 0xc), e.mem.bytes(other + 0xc, 0xc));
        assert_eq!(e.mem.u32(this + 0x18), 0x7fc0_0002);
        assert_eq!(e.mem.u32(this + 0x1c), e.mem.u32(other + 0x1c));
        // The other has both: copies.
        e.mem.set_u32(other + 4, 0x6000);
        e.mem.set_u32(other + 8, 0x6100);
        let (_, log) = run(&mut e, 0x0067_cd90, &[this, other]);
        assert!(log.contains(&(TARGET_COPY, vec![0x7001, 0x6000])));
        assert!(log.contains(&(LOCATION_COPY, vec![0x7002, 0x6100, 0])));
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 8)), (0x7001, 0x7002));
    }

    #[test]
    fn compare_checks_every_field() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        let other = e.mem.alloc(0x24);
        let compare = |e: &mut Engine| e.call(0x0067_cfc0, &args![this, other]).bool();
        assert!(compare(&mut e));
        e.mem.set_u32(SCRATCH, other);
        assert!(!compare(&mut e));
        // Each field in turn.
        for (offset, value) in [(0x20u32, 1u32), (0xc, 1), (0x11, 1), (0x12, 1), (0x16, 1)] {
            e.mem.set_u8(other + offset, value as u8);
            assert!(compare(&mut e), "field {offset:#x}");
            e.mem.set_u8(other + offset, 0);
        }
        // Targets: only the other has one; both, the compare decides.
        e.mem.set_u32(other + 4, 0x6000);
        assert!(compare(&mut e));
        e.mem.set_u32(this + 4, 0x5000);
        assert!(!compare(&mut e));
        e.mem.set_u32(SCRATCH + 4, 1);
        assert!(compare(&mut e));
        e.mem.set_u32(SCRATCH + 4, 0);
        e.mem.set_u32(other + 8, 0x6100);
        assert!(compare(&mut e));
        e.mem.set_u32(this + 8, 0x5100);
        assert!(!compare(&mut e));
        // Waits within 0.01 match; beyond differ; a NaN matches.
        e.mem.set_u32(this + 0x18, 1.0f32.to_bits());
        e.mem.set_u32(other + 0x18, 1.005f32.to_bits());
        assert!(!compare(&mut e));
        e.mem.set_u32(other + 0x1c, 0.5f32.to_bits());
        assert!(compare(&mut e));
        e.mem.set_u32(other + 0x1c, 0x7fc0_0000);
        assert!(!compare(&mut e));
    }

    #[test]
    fn save_writes_the_record_target_and_location() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        for i in 0..0x14 {
            e.mem.set_u8(this + 0xc + i, i as u8 + 1);
        }
        e.mem.set_u32(this + 0x20, 0x9000);
        // The record's bytes when the chunk is added.
        e.register(ADD_CHUNK_DATA, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x18);
            let mut expected: Vec<u8> = (1..=0x14).collect();
            expected.extend_from_slice(&0x9001u32.to_le_bytes());
            assert_eq!(bytes, expected);
            Ret::default()
        });
        let (_, log) = run(&mut e, 0x0067_d350, &[this]);
        let record = log[0].1[0];
        assert_eq!(log[0], (FILL_BYTES, vec![record, 0, 0x18]));
        assert!(log.contains(&(ADD_CHUNK_DATA, vec![0x3357_4b50, record, 0x18])));
        assert!(!log.iter().any(|c| c.0 == RECORD_SWAP || c.0 == TARGET_SAVE));
        e.mem.set_u32(SCRATCH + 8, 1);
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(this + 8, 0x5100);
        let (_, log) = run(&mut e, 0x0067_d350, &[this]);
        assert_eq!(log.iter().filter(|c| c.0 == RECORD_SWAP).count(), 2);
        let tail = log[log.len() - 2..].to_vec();
        assert_eq!(
            tail,
            vec![
                (TARGET_SAVE, vec![0x5000, 0x3244_5450]),
                (LOCATION_SAVE, vec![0x5100, 0x3244_4c50]),
            ]
        );
    }

    #[test]
    fn set_attack_target_makes_copies_or_deletes() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        let (_, log) = run(&mut e, 0x0067_d470, &[this, 0x6000]);
        assert_eq!(
            log,
            vec![
                (ALLOCATE, vec![0x10]),
                (TARGET_CONSTRUCT, vec![0x7000]),
                (TARGET_COPY, vec![0x7001, 0x6000]),
            ]
        );
        let (_, log) = run(&mut e, 0x0067_d470, &[this, 0x6000]);
        assert_eq!(log, vec![(TARGET_COPY, vec![0x7001, 0x6000])]);
        let (_, log) = run(&mut e, 0x0067_d470, &[this, 0]);
        assert_eq!(log, vec![(TARGET_DELETE, vec![0x7001, 1])]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn save_game_and_load_game() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x20, 0x9000);
        let (_, log) = run(&mut e, 0x0067_d550, &[this, 0x4000]);
        let fields: Vec<_> = log
            .iter()
            .take(11)
            .map(|c| (c.1[1] - this, c.1[2]))
            .collect();
        assert_eq!(fields, SAVED_FIELDS.to_vec());
        assert_eq!(log[11], (SAVE_FORM_ID, vec![0x4000, 0x9000, 0]));
        assert_eq!(log.len(), 14);
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(this + 8, 0x5100);
        let (_, log) = run(&mut e, 0x0067_d550, &[this, 0x4000]);
        assert_eq!(log[13], (TARGET_SAVE_GAME, vec![0x5000, 0x4000]));
        assert_eq!(log[15], (LOCATION_SAVE_GAME, vec![0x5100, 0x4000]));
        // Loading: every flag byte reads 1, so both follow.
        let fresh = e.mem.alloc(0x24);
        let (_, log) = run(&mut e, 0x0067_d6b0, &[fresh, 0x4000]);
        assert_eq!(e.mem.u32(fresh + 0x20), 0x1046);
        assert!(log.contains(&(
            RT_DYNAMIC_CAST,
            vec![0x1044, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_WEAP, 0]
        )));
        assert_eq!(e.mem.u32(fresh + 4), 0x7001);
        assert_eq!(e.mem.u32(fresh + 8), 0x7002);
        assert!(log.contains(&(TARGET_LOAD_GAME, vec![0x7001, 0x4000])));
        assert!(log.contains(&(LOCATION_LOAD_GAME, vec![0x7002, 0x4000])));
        assert_eq!(e.mem.u16(fresh + 0x12), 0x0101);
    }

    #[test]
    fn init_load_game_and_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        let (_, log) = run(&mut e, 0x0067_d8b0, &[this, 0x4000]);
        assert!(log.is_empty());
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(this + 8, 0x5100);
        let (_, log) = run(&mut e, 0x0067_d8b0, &[this, 0x4000]);
        assert_eq!(
            log,
            vec![
                (TARGET_INIT_LOAD_GAME, vec![0x5000, 0x4000]),
                (LOCATION_INIT_LOAD_GAME, vec![0x5100, 0x4000]),
            ]
        );
        for (flags, frees) in [(0u32, false), (1, true)] {
            let (result, log) = run(&mut e, 0x0067_d940, &[this, flags]);
            assert_eq!(result.u32(), this);
            assert_eq!(e.mem.u32(this), ACTOR_PACKAGE_DATA_VTABLE);
            assert_eq!(log.len(), usize::from(frees));
        }
    }
}
