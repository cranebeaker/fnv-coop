//! `fallout/ai/combat/combatstate.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is translated in address order over several sessions. The first
//! session took `006f2290` to `0099cb00` (60 functions); the next one starts
//! at `0099cb70`.
//!
//! Notes for the next session:
//! - `CombatState` (Xbox PDB, 0x22c bytes) is not a `TESForm`; its PC
//!   offsets are the PDB's, confirmed by the constructor `00997600` (every
//!   member it initialises sits where the PDB puts it). The layout below
//!   names the fields used so far.
//! - The build is unoptimised: every field is re-read from memory where the
//!   code re-reads it, and the translations do the same.
//! - Small callees confirmed from their bodies: `0097ae90` returns
//!   `CombatController` +0xbc (`pAttacker`), `004030b0` +0xc0 (`pTarget`),
//!   `004fb070` +0x80 (`pCombatGroup`); `00403e20` returns the address of a
//!   `Setting`'s value (+4, or a zeroed scratch float for a null setting);
//!   `00435dd0` loads the game time (`011f1bf0`); `00435de0` stores a float
//!   into a `CombatTimeStamp`; `00435e00` is the time since a stamp;
//!   `008d7f40` starts a `CombatTimer` (start = now, delay = argument),
//!   `008d7f80` says it has expired, `0097fa50` that its delay is not zero,
//!   `00910b10` clears the delay (the map names it
//!   `TESWeightForm::InitializeDataComponent`: folded code).
//! - Functions with a C++ exception-unwinding frame (`FS:[0]`) carry a
//!   `Translation incomplete` marker: the unwinding is not translated.
//! - Floats copied with `FLD`/`FSTP` go through [`x87_quiet`] /
//!   [`x87_quiet_bits`]; floats pushed as arguments are passed as bit
//!   patterns; `-FLT_MAX` is built as the exe does (`FLD` of `FLT_MAX`, then
//!   `FCHS`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiPoint3};
use crate::units::fallout_ai::highprocess::CombatTimer;

layout! {
    /// `CombatState` (Xbox PDB), 0x22c bytes; PC offsets as in the PDB.
    pub struct CombatState: 0x22c {
        /// `bBlocking` (Xbox PDB).
        0x000 bBlocking: bool,
        /// `iLocationUnreachable` (Xbox PDB).
        0x004 iLocationUnreachable: i32,
        /// `fThreatRatio` (Xbox PDB).
        0x008 fThreatRatio: f32,
        /// `pCombatWeapons` (Xbox PDB): six `TESObjectWEAP*`.
        0x00c pCombatWeapons: u32,
        /// `EmbeddedWeaponsArray` (Xbox PDB):
        /// `BSSimpleArray<TESObjectWEAP *, 1024>`.
        0x024 EmbeddedWeaponsArray: Inline<BSSimpleArray>,
        /// `pCurrentEmbeddedWeapon` (Xbox PDB).
        0x034 pCurrentEmbeddedWeapon: u32,
        /// `iHasCombatWeapon` (Xbox PDB): one bit per weapon type.
        0x038 iHasCombatWeapon: u32,
        /// `fCombatWeaponsDPS` (Xbox PDB): seven floats.
        0x03c fCombatWeaponsDPS: f32,
        /// The seventh entry of `fCombatWeaponsDPS` (+0x54).
        0x054 fUnarmedDPS: f32,
        /// `fMaxCombatWeaponDPS` (Xbox PDB).
        0x058 fMaxCombatWeaponDPS: f32,
        /// `fMaxRangedWeaponDPS` (Xbox PDB).
        0x05c fMaxRangedWeaponDPS: f32,
        /// `fMaxModifiedCombatWeaponDPS` (Xbox PDB).
        0x060 fMaxModifiedCombatWeaponDPS: f32,
        /// `fDisarmedDPS` (Xbox PDB).
        0x064 fDisarmedDPS: f32,
        /// `fBetterWeaponOutOfRangeDistance` (Xbox PDB).
        0x068 fBetterWeaponOutOfRangeDistance: f32,
        /// `iTargetDetectionLevel` (Xbox PDB).
        0x06c iTargetDetectionLevel: i32,
        /// `bTargetLineOfSight` (Xbox PDB).
        0x070 bTargetLineOfSight: u8,
        /// `bTarget360LineOfSight` (Xbox PDB).
        0x071 bTarget360LineOfSight: u8,
        /// `bTargetBlocked` (Xbox PDB).
        0x072 bTargetBlocked: u8,
        /// `cTargetFlushed` (Xbox PDB).
        0x073 cTargetFlushed: i8,
        /// `bTargetSpaceDifferent` (Xbox PDB).
        0x074 bTargetSpaceDifferent: u8,
        /// `bHasHandToHandEquip` (Xbox PDB).
        0x075 bHasHandToHandEquip: u8,
        /// `bGrenadeThrowHighArc` (Xbox PDB).
        0x076 bGrenadeThrowHighArc: u8,
        /// `bInventoryDirty` (Xbox PDB).
        0x077 bInventoryDirty: u8,
        /// `pExplosiveProjectileBlocked` (Xbox PDB): `BGSExplosion*`.
        0x078 pExplosiveProjectileBlocked: u32,
        /// `eTargetSegmentInView` (Xbox PDB).
        0x07c eTargetSegmentInView: u32,
        /// `fThreatScore` (Xbox PDB).
        0x080 fThreatScore: f32,
        /// `LastSeenTargetLocation` (Xbox PDB): an `NiPoint3`.
        0x084 LastSeenTargetLocation: Inline<NiPoint3>,
        /// `iCombatPermissions` (Xbox PDB).
        0x090 iCombatPermissions: u32,
        /// `fLastTargetDetectedTimeStamp` (Xbox PDB): a `CombatTimeStamp`.
        0x094 fLastTargetDetectedTimeStamp: u32,
        /// `fLastTarget360LineOfSightTimeStamp` (Xbox PDB).
        0x098 fLastTarget360LineOfSightTimeStamp: u32,
        /// `fLastAttackTimeStamp` (Xbox PDB).
        0x09c fLastAttackTimeStamp: u32,
        /// `fLastDamagedByAttackerTimeStamp` (Xbox PDB).
        0x0a0 fLastDamagedByAttackerTimeStamp: u32,
        /// `fFiringStartedTimeStamp` (Xbox PDB).
        0x0a4 fFiringStartedTimeStamp: u32,
        /// `AttackDialogueTimer` (Xbox PDB): a `CombatTimer`.
        0x0a8 AttackDialogueTimer: Inline<CombatTimer>,
        /// `fThreatsConsideredTimeStamp` (Xbox PDB).
        0x0c0 fThreatsConsideredTimeStamp: u32,
        /// `pLocationTarget` (Xbox PDB): `CombatLocationTarget*`.
        0x0c4 pLocationTarget: u32,
        /// `cStartingConfidence` (Xbox PDB).
        0x0c8 cStartingConfidence: u8,
        /// `cCurrentConfidence` (Xbox PDB).
        0x0c9 cCurrentConfidence: u8,
        /// `fConfidenceModifier` (Xbox PDB).
        0x0cc fConfidenceModifier: f32,
        /// `fCombatStrength` (Xbox PDB).
        0x0d0 fCombatStrength: f32,
        /// `fDamagePerSecond` (Xbox PDB).
        0x0d4 fDamagePerSecond: f32,
        /// `cFleeState` (Xbox PDB).
        0x0d8 cFleeState: u8,
        /// `iLastFleeDoor` (Xbox PDB).
        0x0dc iLastFleeDoor: u32,
        /// `fLastFleeDoorTimeStamp` (Xbox PDB).
        0x0e0 fLastFleeDoorTimeStamp: u32,
        /// `pCoverLocation` (Xbox PDB): `CombatCoverLocation*`.
        0x0e4 pCoverLocation: u32,
        /// `pSecondaryCoverLocation` (Xbox PDB).
        0x0e8 pSecondaryCoverLocation: u32,
        /// `CoverSearchInfo` (Xbox PDB): a `CombatCoverSearchInfo`.
        0x0ec CoverSearchInfo: Inline<CombatCoverSearchInfo>,
        /// `SecondaryCoverSearchInfo` (Xbox PDB).
        0x128 SecondaryCoverSearchInfo: Inline<CombatCoverSearchInfo>,
        /// `UnreachableCoverLocations` (Xbox PDB).
        0x164 UnreachableCoverLocations: Inline<BSSimpleArray>,
        /// `UnreachableLocations` (Xbox PDB).
        0x174 UnreachableLocations: Inline<BSSimpleArray>,
        /// `fLastDodgeSearch` (Xbox PDB).
        0x184 fLastDodgeSearch: u32,
        /// `cLastDodgeDirection` (Xbox PDB).
        0x188 cLastDodgeDirection: u8,
        /// `pCombatLocation` (Xbox PDB): `CombatLocation*`.
        0x18c pCombatLocation: u32,
        /// `pCombatItems` (Xbox PDB): five `CombatItem*`.
        0x190 pCombatItems: u32,
        /// `pUsableCombatItems` (Xbox PDB): two `TESBoundObject*`.
        0x1a4 pUsableCombatItems: u32,
        /// `UsableCombatItemTimers` (Xbox PDB): two `CombatTimer`s.
        0x1ac UsableCombatItemTimers: Inline<CombatTimer>,
        /// `iHasCombatItem` (Xbox PDB).
        0x1bc iHasCombatItem: u32,
        /// `pLastTarget` (Xbox PDB): `Actor*`.
        0x1c0 pLastTarget: u32,
        /// `pCombatController` (Xbox PDB).
        0x1c4 pCombatController: u32,
        /// `bInitializing` (Xbox PDB).
        0x1c8 bInitializing: u8,
        /// `TargetUnreachableTimer` (Xbox PDB).
        0x1cc TargetUnreachableTimer: Inline<CombatTimer>,
        /// `FindBetterWeaponTimer` (Xbox PDB).
        0x1d4 FindBetterWeaponTimer: Inline<CombatTimer>,
        /// `ExplosiveProjectileBlockedTimer` (Xbox PDB).
        0x1dc ExplosiveProjectileBlockedTimer: Inline<CombatTimer>,
        /// `CombatPermissionsTimer5sec` (Xbox PDB).
        0x1e4 CombatPermissionsTimer5sec: Inline<CombatTimer>,
        /// `CombatPermissionsTimer10sec` (Xbox PDB).
        0x1ec CombatPermissionsTimer10sec: Inline<CombatTimer>,
        /// `GuardSneakTimer` (Xbox PDB).
        0x1f4 GuardSneakTimer: Inline<CombatTimer>,
        /// `DynamicCoverLostTimer` (Xbox PDB).
        0x1fc DynamicCoverLostTimer: Inline<CombatTimer>,
        /// `CombatStrengthUpdateTimer` (Xbox PDB).
        0x204 CombatStrengthUpdateTimer: Inline<CombatTimer>,
        /// `ThreatRatioUpdateTimer` (Xbox PDB).
        0x20c ThreatRatioUpdateTimer: Inline<CombatTimer>,
        /// `EmbeddedWeaponUpdateTimer` (Xbox PDB).
        0x214 EmbeddedWeaponUpdateTimer: Inline<CombatTimer>,
        /// `UpdateInventoryTimer` (Xbox PDB).
        0x21c UpdateInventoryTimer: Inline<CombatTimer>,
        /// `spPathCheckQueue` (Xbox PDB): `NiPointer<ActorPathingMessageQueue>`.
        0x224 spPathCheckQueue: u32,
        /// `iQueuedPathCheck` (Xbox PDB).
        0x228 iQueuedPathCheck: i32,
    }

    /// `CombatCoverSearchInfo` (Xbox PDB), 0x3c bytes.
    pub struct CombatCoverSearchInfo: 0x3c {
        /// `TargetLocation` (Xbox PDB): an `NiPoint3`.
        0x00 TargetLocation: Inline<NiPoint3>,
        /// `Center` (Xbox PDB): an `NiPoint3`.
        0x0c Center: Inline<NiPoint3>,
        /// `fRadius` (Xbox PDB).
        0x18 fRadius: f32,
        /// `fTimeStamp` (Xbox PDB): a `CombatTimeStamp`.
        0x1c fTimeStamp: u32,
        /// `CoverLocations` (Xbox PDB):
        /// `BSSimpleArray<PathingCoverLocation, 1024>`.
        0x20 CoverLocations: Inline<BSSimpleArray>,
        /// `iPreviousCoverLocationKey` (Xbox PDB).
        0x30 iPreviousCoverLocationKey: u32,
        /// `ePreviousCoverState` (Xbox PDB).
        0x34 ePreviousCoverState: u32,
        /// `bUsedLastSeenLocation` (Xbox PDB).
        0x38 bUsedLastSeenLocation: u8,
    }

    /// `CombatLocation` (Xbox PDB), 0x20 bytes: a `BGSWorldLocation`
    /// (point and space) then the reference, type, radius and flags.
    pub struct CombatLocation: 0x20 {
        /// `WorldLoc` (Xbox PDB): a `BGSWorldLocation` (16 bytes).
        0x00 WorldLoc: u32,
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x10 pRef: u32,
        /// `iLocation` (Xbox PDB).
        0x14 iLocation: i32,
        /// `fRadius` (Xbox PDB).
        0x18 fRadius: f32,
        /// `iFlags` (Xbox PDB).
        0x1c iFlags: u32,
    }

    /// `CombatLocationTarget` (Xbox PDB), 0x20 bytes.
    pub struct CombatLocationTarget: 0x20 {
        /// `cType` (Xbox PDB).
        0x00 cType: u8,
        /// `bNearTarget` (Xbox PDB).
        0x01 bNearTarget: u8,
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x04 pRef: u32,
        /// `WorldLoc` (Xbox PDB): a `BGSWorldLocation` (16 bytes).
        0x08 WorldLoc: u32,
        /// `fMinDistance` (Xbox PDB).
        0x18 fMinDistance: f32,
        /// `fMaxDistance` (Xbox PDB).
        0x1c fMaxDistance: f32,
    }
}

/// `FLT_MAX` (`float`); `-FLT_MAX` is built from it with `FCHS`.
const MAX_FLOAT: u32 = 0x0108_df70;
/// `100.0` (`float`).
const HUNDRED: u32 = 0x0101_6410;
/// `-1.0` (`float`).
const MINUS_ONE: u32 = 0x0101_2054;
/// `2.0` (`float`).
const TWO: u32 = 0x0101_62c0;
/// `0.25` (`float`).
const QUARTER: u32 = 0x0101_622c;
/// `0.5` (`float`).
const HALF: u32 = 0x0101_6248;
/// `0.1` (`float`).
const TENTH: u32 = 0x0101_e2bc;
/// `5.0` (`float`).
const FIVE: u32 = 0x0101_712c;
/// `10.0` (`float`).
const TEN: u32 = 0x0101_7b78;
/// `3.0` (`float`).
const THREE: u32 = 0x0101_7718;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `256.0` (`double`).
const TWO_HUNDRED_FIFTY_SIX_DOUBLE: u32 = 0x0102_31d8;
/// `100.0` (`double`).
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
/// `120.0` (`double`).
const HUNDRED_TWENTY_DOUBLE: u32 = 0x0107_18b8;
/// `10.0` (`double`).
const TEN_DOUBLE: u32 = 0x0102_0758;
/// `65536.0` (`double`).
const SIXTY_FIVE_THOUSAND_DOUBLE: u32 = 0x0108_e058;
/// Three floats at `011f426c` (zero in the exe's data) that the code copies
/// as a default point.
const ZERO_POINT: u32 = 0x011f_426c;
/// The pointer at `011f1958` that `00992720` and `009a5670`'s getter are
/// called on (the combat manager in other units).
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// The pointer at `011f1708` that `CombatDialogueManager::StartDialogue`
/// (`00983970`) is called on.
const COMBAT_DIALOGUE_MANAGER: u32 = 0x011f_1708;
/// The pointer at `011dea3c` (the player character in other units).
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// A byte switch at `011f1ae4` that turns the location-target search of
/// `00997cf0` on.
const LOCATION_TARGET_SEARCH_011F1AE4: u32 = 0x011f_1ae4;
/// A byte switch at `011a4d34` (1 in the exe's data) choosing between the
/// two location-target searches of `00997cf0`.
const LOCATION_TARGET_MODE_011A4D34: u32 = 0x011a_4d34;
/// Three floats at `011a5308`, `011a530c`, `011a5310` (1024, 768, 384 in the
/// exe's data) that `00997cf0` passes to the location-target search.
const LOCATION_TARGET_MAX_DISTANCE: u32 = 0x011a_5308;
const LOCATION_TARGET_SECOND_DISTANCE: u32 = 0x011a_530c;
const LOCATION_TARGET_FIRST_DISTANCE: u32 = 0x011a_5310;
/// The 8-byte entries per combat location type that `0099ac20` reads: a byte
/// at +0, a byte at +1 and a float at +4.
const LOCATION_TYPE_TABLE: u32 = 0x011a_5650;

/// `Setting` value address (`00403e20`): +4 of the setting, or a zeroed
/// scratch float for a null setting.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The game time in ST0 (`00435dd0`).
const GAME_TIME: u32 = 0x0043_5dd0;
/// Stores its float argument into a `CombatTimeStamp` (`00435de0`).
const TIME_STAMP_SET: u32 = 0x0043_5de0;
/// The time since a `CombatTimeStamp`, in ST0 (`00435e00`).
const TIME_STAMP_ELAPSED: u32 = 0x0043_5e00;
/// `CombatTimer`: start with a delay (`008d7f40`).
const TIMER_START: u32 = 0x008d_7f40;
/// `CombatTimer`: the delay has passed (`008d7f80`).
const TIMER_EXPIRED: u32 = 0x008d_7f80;
/// `CombatTimer`: the delay is not zero (`0097fa50`).
const TIMER_RUNNING: u32 = 0x0097_fa50;
/// `CombatTimer`: clears the delay (`00910b10`).
const TIMER_STOP: u32 = 0x0091_0b10;
/// `CombatController` +0xbc (`0097ae90`).
const CONTROLLER_ATTACKER: u32 = 0x0097_ae90;
/// `CombatController` +0xc0 (`004030b0`).
const CONTROLLER_TARGET: u32 = 0x0040_30b0;
/// `CombatController` +0x80 (`004fb070`).
const CONTROLLER_GROUP: u32 = 0x004f_b070;
/// `RandomFloat` (Xbox PDB, `00476b70`): cdecl, two floats.
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `TESObjectREFR::GetWorldLocation` (Xbox PDB, `00575650`).
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
/// `TESObjectREFR::GetSpace` (Xbox PDB, `00575ca0`).
const GET_SPACE: u32 = 0x0057_5ca0;
/// `memset`-shaped cdecl helper (`00403d30`): address, byte, size.
const FILL_BYTES: u32 = 0x0040_3d30;
/// `BSSimpleArray` size getter (`0044ddc0`, +8).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `CombatState::ClearCoverLocation` (Xbox PDB, `009a03d0`).
const CLEAR_COVER_LOCATION: u32 = 0x009a_03d0;
/// `CombatState::ClearSecondaryCoverLocation` (Xbox PDB, `009a0470`).
const CLEAR_SECONDARY_COVER_LOCATION: u32 = 0x009a_0470;

/// `-FLT_MAX` as the exe builds it (`FLD FLT_MAX`, `FCHS`), as bits.
fn negative_max_bits(e: &Engine) -> u32 {
    x87_quiet_bits(e.global::<u32>(MAX_FLOAT)) ^ 0x8000_0000
}

/// A float constant pushed through `FLD`/`FSTP`, as bits.
fn constant_bits(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(addr))
}

/// The address of a `Setting`'s value (`00403e20`).
fn setting_value_address(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_VALUE, &args![setting]).u32()
}

/// A `Setting`'s value loaded with `FLD` and pushed with `FSTP`, as bits.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = setting_value_address(e, setting);
    x87_quiet_bits(e.mem.u32(value))
}

/// A `Setting`'s value as `FLD` loads it for a comparison.
fn setting_value(e: &mut Engine, setting: u32) -> f32 {
    let value = setting_value_address(e, setting);
    e.mem.f32(value)
}

/// The word a `CombatTimeStamp` local holds after `00435de0` stored `bits`
/// into it (the code then copies the local into the member).
fn time_stamp_word(e: &mut Engine, bits: u32) -> u32 {
    e.with_stack(4, |e, stamp| {
        e.call(TIME_STAMP_SET, &args![stamp, bits]);
        e.mem.u32(stamp.addr())
    })
}

/// The game time (`00435dd0`) as the `FSTP` to the stack rounds it, as bits.
fn game_time_bits(e: &mut Engine) -> u32 {
    e.call(GAME_TIME, &args![]).f32().to_bits()
}

fn state_controller(e: &Engine, this: Ptr<CombatState>) -> u32 {
    e.get(this, CombatState::pCombatController)
}

fn attacker(e: &mut Engine, this: Ptr<CombatState>) -> u32 {
    let controller = state_controller(e, this);
    e.call(CONTROLLER_ATTACKER, &args![controller]).u32()
}

fn target(e: &mut Engine, this: Ptr<CombatState>) -> u32 {
    let controller = state_controller(e, this);
    e.call(CONTROLLER_TARGET, &args![controller]).u32()
}

fn timer_start(e: &mut Engine, timer: u32, delay_bits: u32) {
    e.call(TIMER_START, &args![timer, delay_bits]);
}

fn timer_expired(e: &mut Engine, timer: u32) -> bool {
    e.call(TIMER_EXPIRED, &args![timer]).bool()
}

fn timer_running(e: &mut Engine, timer: u32) -> bool {
    e.call(TIMER_RUNNING, &args![timer]).bool()
}

fn timer_stop(e: &mut Engine, timer: u32) {
    e.call(TIMER_STOP, &args![timer]);
}

/// `RandomFloat(low, high)` (`00476b70`) as the `FSTP` to the stack rounds
/// it, as bits.
fn random_float_bits(e: &mut Engine, low_bits: u32, high_bits: u32) -> u32 {
    e.call(RANDOM_FLOAT, &args![low_bits, high_bits])
        .f32()
        .to_bits()
}

/// Copies `count` words from `from` to `to`.
fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// The address of member `off` of the state.
fn member(this: Ptr<CombatState>, off: u32) -> u32 {
    this.addr().wrapping_add(off)
}

// Translated from 006f2290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESObjectWEAP *, 1024>::SetSize` (Xbox PDB): resizes the
/// array to `size` elements. Size 0 hands over to `008454f0` (with
/// `shrink`); growing past the reserved size allocates through the array's
/// own vtable slot 4 (empty array) or reallocates (`0042fc00`), then
/// constructs the new elements (`00726bf0`); shrinking destroys the
/// elements past `size` (`0072ba80`) and, with `shrink` and at most a
/// quarter of the reservation in use, reallocates to `size`.
pub fn bs_simple_array_tes_object_weap_set_size(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    size: u32,
    shrink: u8,
) {
    if size == 0 {
        e.call(0x0084_54f0, &args![this, shrink as u32]);
    } else if size > e.get(this, BSSimpleArray::iReservedSize) {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let buffer = e.vcall(this.addr(), 0x4, &args![size]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, size);
        } else {
            let used = e.get(this, BSSimpleArray::iSize);
            e.call(0x0042_fc00, &args![this, size, used]);
            e.set(this, BSSimpleArray::iReservedSize, size);
        }
        let used = e.get(this, BSSimpleArray::iSize);
        let start = e
            .get(this, BSSimpleArray::pBuffer)
            .wrapping_add(used.wrapping_mul(4));
        e.call(0x0072_6bf0, &args![this, start, size.wrapping_sub(used)]);
        e.set(this, BSSimpleArray::iSize, size);
    } else if size < e.get(this, BSSimpleArray::iSize) {
        let used = e.get(this, BSSimpleArray::iSize);
        let start = e
            .get(this, BSSimpleArray::pBuffer)
            .wrapping_add(size.wrapping_mul(4));
        e.call(0x0072_ba80, &args![this, start, used.wrapping_sub(size)]);
        e.set(this, BSSimpleArray::iSize, size);
        if shrink != 0 && size <= e.get(this, BSSimpleArray::iReservedSize) >> 2 {
            e.call(0x0042_fc00, &args![this, size, size]);
            e.set(this, BSSimpleArray::iReservedSize, size);
        }
    } else {
        let used = e.get(this, BSSimpleArray::iSize);
        let start = e
            .get(this, BSSimpleArray::pBuffer)
            .wrapping_add(used.wrapping_mul(4));
        e.call(0x0072_6bf0, &args![this, start, size.wrapping_sub(used)]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 0097ee20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises a `CombatLocation`: the `BGSWorldLocation` part through
/// `0097ede0`, then no reference, location -1, radius 0 and no flags;
/// returns `this`. The map's name `BGSWorldLocation::SetSpace` is folded
/// code; the body matches `CombatLocation`'s layout (Xbox PDB).
pub fn fn_0097ee20(e: &mut Engine, this: Ptr<CombatLocation>) -> Ptr<CombatLocation> {
    e.call(0x0097_ede0, &args![this]);
    e.set(this, CombatLocation::pRef, 0);
    e.set(this, CombatLocation::iLocation, -1);
    e.set(this, CombatLocation::fRadius, 0.0);
    e.set(this, CombatLocation::iFlags, 0);
    this
}

// Translated from 009973a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the `CombatLocation` at `other` into `this` (the radius through
/// `FLD`/`FSTP`).
pub fn fn_009973a0(e: &mut Engine, this: Ptr<CombatLocation>, other: Ptr<CombatLocation>) {
    copy_words(e, this.addr(), other.addr(), 4);
    let reference = e.get(other, CombatLocation::pRef);
    e.set(this, CombatLocation::pRef, reference);
    let location = e.get(other, CombatLocation::iLocation);
    e.set(this, CombatLocation::iLocation, location);
    let radius = e.get(other, CombatLocation::fRadius);
    e.set(this, CombatLocation::fRadius, x87_quiet(radius));
    let flags = e.get(other, CombatLocation::iFlags);
    e.set(this, CombatLocation::iFlags, flags);
}

// Translated from 00997400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatLocation::SetRef` (Xbox PDB): stores the reference and copies its
/// world location (`TESObjectREFR::GetWorldLocation`) into `WorldLoc`.
pub fn combat_location_set_ref(e: &mut Engine, this: Ptr<CombatLocation>, reference: u32) {
    e.set(this, CombatLocation::pRef, reference);
    combat_location_copy_ref_location(e, this);
}

/// The world location of `pRef` (through a 16-byte local) copied into
/// `WorldLoc`, as `00997400` and `00997440` do.
fn combat_location_copy_ref_location(e: &mut Engine, this: Ptr<CombatLocation>) {
    e.with_stack(16, |e, location| {
        let reference = e.get(this, CombatLocation::pRef);
        let result = e
            .call(GET_WORLD_LOCATION, &args![reference, location])
            .u32();
        copy_words(e, this.addr(), result, 4);
    });
}

// Translated from 00997440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatLocation::UpdateRefLocation` (Xbox PDB): with a reference, copies
/// its current world location into `WorldLoc`.
pub fn combat_location_update_ref_location(e: &mut Engine, this: Ptr<CombatLocation>) {
    if e.get(this, CombatLocation::pRef) != 0 {
        combat_location_copy_ref_location(e, this);
    }
}

// Translated from 00997480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the `CombatCoverLocation` (Xbox PDB layout, 0x78 bytes) at `other`
/// into `this`: the three flag bytes, the location, the search point, the
/// `PathingCoverLocation` at +0x14 (through `006e4430` on `this`'s), the key,
/// the time stamp and the six-byte reservation information.
pub fn fn_00997480(e: &mut Engine, this: Ptr, other: Ptr) {
    let (to, from) = (this.addr(), other.addr());
    for off in 0..3 {
        let byte = e.mem.u8(from + off);
        e.mem.set_u8(to + off, byte);
    }
    copy_words(e, to + 0x4, from + 0x4, 1);
    copy_words(e, to + 0x8, from + 0x8, 3);
    e.call(0x006e_4430, &args![to + 0x14, from + 0x14]);
    copy_words(e, to + 0x68, from + 0x68, 1);
    copy_words(e, to + 0x6c, from + 0x6c, 1);
    copy_words(e, to + 0x70, from + 0x70, 1);
    let half = e.mem.u16(from + 0x74);
    e.mem.set_u16(to + 0x74, half);
}

// Translated from 00997520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the `CombatCoverSearchInfo` at `other` to `this`; a null `other`
/// resets `this` (`009975c0`). The cover location array is copied by
/// `009a3780`, which the code calls on `other`'s array with `this`'s as the
/// argument.
pub fn fn_00997520(
    e: &mut Engine,
    this: Ptr<CombatCoverSearchInfo>,
    other: Ptr<CombatCoverSearchInfo>,
) {
    if other.is_null() {
        fn_009975c0(e, this);
        return;
    }
    copy_words(e, this.addr(), other.addr(), 3);
    copy_words(e, this.addr() + 0xc, other.addr() + 0xc, 3);
    let radius = e.get(other, CombatCoverSearchInfo::fRadius);
    e.set(this, CombatCoverSearchInfo::fRadius, x87_quiet(radius));
    let stamp = e.get(other, CombatCoverSearchInfo::fTimeStamp);
    e.set(this, CombatCoverSearchInfo::fTimeStamp, stamp);
    e.call(0x009a_3780, &args![other.addr() + 0x20, this.addr() + 0x20]);
    let key = e.get(other, CombatCoverSearchInfo::iPreviousCoverLocationKey);
    e.set(this, CombatCoverSearchInfo::iPreviousCoverLocationKey, key);
    let state = e.get(other, CombatCoverSearchInfo::ePreviousCoverState);
    e.set(this, CombatCoverSearchInfo::ePreviousCoverState, state);
    let used = e.get(other, CombatCoverSearchInfo::bUsedLastSeenLocation);
    e.set(this, CombatCoverSearchInfo::bUsedLastSeenLocation, used);
}

// Translated from 009975c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets a `CombatCoverSearchInfo`: radius 0, time stamp `-FLT_MAX`, and the
/// cover location array emptied through `006dbde0(1)`.
pub fn fn_009975c0(e: &mut Engine, this: Ptr<CombatCoverSearchInfo>) {
    e.set(this, CombatCoverSearchInfo::fRadius, 0.0);
    let bits = negative_max_bits(e);
    let stamp = time_stamp_word(e, bits);
    e.set(this, CombatCoverSearchInfo::fTimeStamp, stamp);
    e.call(0x006d_bde0, &args![this.addr() + 0x20, 1u32]);
}

// Translated from 00997600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00997600 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `CombatState` constructor: initialises every member (time stamps to
/// `-FLT_MAX`, timers through `00906f60`, the arrays and cover search
/// information through their constructors), stores the controller, clears
/// the weapon, DPS, item and usable-item arrays, starts the threat-ratio
/// timer with a random delay below the setting at `011cf804`, and sets the
/// confidence modifier to a random value between the settings at `011cfdcc`
/// and `011cf7ec`. The second argument word is not read.
pub fn fn_00997600(
    e: &mut Engine,
    this: Ptr<CombatState>,
    combat_controller: u32,
    _unused_2: u32,
) -> Ptr<CombatState> {
    let at = |off: u32| member(this, off);
    e.set(this, CombatState::bBlocking, false);
    e.set(this, CombatState::iLocationUnreachable, 0);
    let ratio = constant_bits(e, HUNDRED);
    e.set(this, CombatState::fThreatRatio, f32::from_bits(ratio));
    e.call(0x009a_33e0, &args![at(0x24)]);
    e.set(this, CombatState::pCurrentEmbeddedWeapon, 0);
    e.set(this, CombatState::iHasCombatWeapon, 0);
    e.set(this, CombatState::fMaxCombatWeaponDPS, 0.0);
    e.set(this, CombatState::fMaxRangedWeaponDPS, 0.0);
    e.set(this, CombatState::fMaxModifiedCombatWeaponDPS, 0.0);
    e.set(this, CombatState::fDisarmedDPS, 0.0);
    e.set(this, CombatState::fBetterWeaponOutOfRangeDistance, 0.0);
    e.set(this, CombatState::iTargetDetectionLevel, -1);
    e.set(this, CombatState::bTargetLineOfSight, 0);
    e.set(this, CombatState::bTarget360LineOfSight, 0);
    e.set(this, CombatState::bTargetBlocked, 0);
    e.set(this, CombatState::cTargetFlushed, -1);
    e.set(this, CombatState::bTargetSpaceDifferent, 0);
    e.set(this, CombatState::bHasHandToHandEquip, 0);
    e.set(this, CombatState::bGrenadeThrowHighArc, 0);
    e.set(this, CombatState::bInventoryDirty, 0);
    e.set(this, CombatState::pExplosiveProjectileBlocked, 0);
    e.set(this, CombatState::eTargetSegmentInView, 3);
    e.set(this, CombatState::fThreatScore, 0.0);
    copy_words(e, at(0x84), ZERO_POINT, 3);
    e.set(this, CombatState::iCombatPermissions, 0);
    for off in [0x94, 0x98, 0x9c, 0xa0, 0xa4] {
        let bits = negative_max_bits(e);
        e.call(TIME_STAMP_SET, &args![at(off), bits]);
    }
    for off in [0xa8, 0xb0, 0xb8] {
        e.call(0x0090_6f60, &args![at(off)]);
    }
    e.call(0x0043_00a0, &args![at(0xc0)]);
    e.set(this, CombatState::pLocationTarget, 0);
    e.set(this, CombatState::cStartingConfidence, 0);
    e.set(this, CombatState::cCurrentConfidence, 0);
    let minus_one = f32::from_bits(constant_bits(e, MINUS_ONE));
    e.set(this, CombatState::fCombatStrength, minus_one);
    e.set(this, CombatState::fDamagePerSecond, minus_one);
    e.set(this, CombatState::cFleeState, 0);
    e.set(this, CombatState::iLastFleeDoor, 0);
    let bits = negative_max_bits(e);
    e.call(TIME_STAMP_SET, &args![at(0xe0), bits]);
    e.set(this, CombatState::pCoverLocation, 0);
    e.set(this, CombatState::pSecondaryCoverLocation, 0);
    e.call(0x006d_32e0, &args![at(0xec)]);
    e.call(0x006d_32e0, &args![at(0x128)]);
    e.call(0x009a_3430, &args![at(0x164)]);
    e.call(0x009a_35c0, &args![at(0x174)]);
    e.call(0x0043_00a0, &args![at(0x184)]);
    e.set(this, CombatState::cLastDodgeDirection, 0xff);
    e.set(this, CombatState::pCombatLocation, 0);
    // `vector constructor iterator` (stdcall): two 8-byte timers built by
    // `00906f60`.
    e.call(0x0040_1050, &args![at(0x1ac), 8u32, 2u32, 0x0090_6f60u32]);
    e.set(this, CombatState::iHasCombatItem, 0);
    e.set(this, CombatState::pLastTarget, 0);
    e.set(this, CombatState::pCombatController, combat_controller);
    e.set(this, CombatState::bInitializing, 1);
    for off in [
        0x1cc, 0x1d4, 0x1dc, 0x1e4, 0x1ec, 0x1f4, 0x1fc, 0x204, 0x20c, 0x214, 0x21c,
    ] {
        e.call(0x0090_6f60, &args![at(off)]);
    }
    e.call(0x006e_bf60, &args![at(0x224), 0u32]);
    e.set(this, CombatState::iQueuedPathCheck, -1);
    e.call(FILL_BYTES, &args![at(0xc), 0u32, 0x18u32]);
    e.call(FILL_BYTES, &args![at(0x3c), 0u32, 0x1cu32]);
    e.call(FILL_BYTES, &args![at(0x190), 0u32, 0x14u32]);
    e.call(FILL_BYTES, &args![at(0x1a4), 0u32, 0x8u32]);
    let high = setting_bits(e, 0x011c_f804);
    let delay = random_float_bits(e, 0.0f32.to_bits(), high);
    timer_start(e, at(0x20c), delay);
    let high = setting_bits(e, 0x011c_f7ec);
    let low = setting_bits(e, 0x011c_fdcc);
    let modifier = e.call(RANDOM_FLOAT, &args![low, high]).f32();
    e.set(this, CombatState::fConfidenceModifier, modifier);
    this
}

// Translated from 00997aa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00997aa0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `CombatState` destructor body: clears the weapon arrays
/// (`0099a2a0`), both cover locations, the combat location (`009a0680`) and
/// the location target (`009a0a40`), releases the five combat items
/// (`00981340` with indices 0 to 4), then destroys the path-check queue
/// pointer, the two unreachable arrays, both cover search informations and
/// the embedded weapon array.
pub fn fn_00997aa0(e: &mut Engine, this: Ptr<CombatState>) {
    fn_0099a2a0(e, this);
    e.call(CLEAR_COVER_LOCATION, &args![this, 0u32]);
    e.call(CLEAR_SECONDARY_COVER_LOCATION, &args![this, 0u32]);
    e.call(0x009a_0680, &args![this]);
    e.call(0x009a_0a40, &args![this]);
    for index in 0u32..5 {
        e.call(0x0098_1340, &args![this, index]);
    }
    e.call(0x006e_bf90, &args![member(this, 0x224)]);
    e.call(0x009a_35f0, &args![member(this, 0x174)]);
    e.call(0x009a_3460, &args![member(this, 0x164)]);
    e.call(0x006e_4d00, &args![member(this, 0x128)]);
    e.call(0x006e_4d00, &args![member(this, 0xec)]);
    e.call(0x009a_3410, &args![member(this, 0x24)]);
}

// Translated from 00997ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `first` is zero and `second` is not: clears both cover locations
/// and both cover search informations, and the combat location
/// (`009a0680`).
pub fn fn_00997ba0(e: &mut Engine, this: Ptr<CombatState>, first: u32, second: u32) {
    if first == 0 && second != 0 {
        e.call(CLEAR_COVER_LOCATION, &args![this, 0u32]);
        fn_00997bf0(e, this);
        e.call(CLEAR_SECONDARY_COVER_LOCATION, &args![this, 0u32]);
        fn_00997c10(e, this);
        e.call(0x009a_0680, &args![this]);
    }
}

// Translated from 00997bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets `CoverSearchInfo` (`009975c0`).
pub fn fn_00997bf0(e: &mut Engine, this: Ptr<CombatState>) {
    fn_009975c0(e, this.at(CombatState::CoverSearchInfo));
}

// Translated from 00997c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets `SecondaryCoverSearchInfo` (`009975c0`).
pub fn fn_00997c10(e: &mut Engine, this: Ptr<CombatState>) {
    fn_009975c0(e, this.at(CombatState::SecondaryCoverSearchInfo));
}

// Translated from 00997c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// First update of the state: records whether the attacker has hand-to-hand
/// equipment (`CombatController::CheckHasEquip(attacker, 0)`), updates the
/// weapons (`009993c0`) and the state (`00997cb0`), sets both confidences
/// from the attacker's +0xa4 object (virtual slot 8 with 1), and ends the
/// initialisation.
pub fn fn_00997c30(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    let equipped = e.call(0x0097_f310, &args![attacker, 0u32]).u8();
    e.set(this, CombatState::bHasHandToHandEquip, equipped);
    fn_009993c0(e, this);
    fn_00997cb0(e, this);
    let confidence = e.vcall(attacker.wrapping_add(0xa4), 0x8, &args![1u32]).u8();
    e.set(this, CombatState::cStartingConfidence, confidence);
    let starting = e.get(this, CombatState::cStartingConfidence);
    e.set(this, CombatState::cCurrentConfidence, starting);
    e.set(this, CombatState::bInitializing, 0);
}

// Translated from 00997cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the state: with the attacker's current process type 0
/// (`MobileObject::GetCurrentProcessType`), the full update (`00997cf0`,
/// then `00998a50`); otherwise the reduced one (`00999160`).
pub fn fn_00997cb0(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    if e.call(0x0093_1850, &args![attacker]).u32() == 0 {
        fn_00997cf0(e, this);
        fn_00998a50(e, this);
    } else {
        fn_00999160(e, this);
    }
}

// Translated from 00997cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The full per-frame update of the target information:
/// - whether the target is in another space (and dropping line of sight
///   when it leaves, `0097f200`);
/// - without a target: detection level -1, no 360-degree line of sight, the
///   detection and line-of-sight stamps `-FLT_MAX`, the flush state -1;
///   with one, a detection update through the group (`009887b0`) when the
///   controller asks for it or the target changed, the last-seen location
///   and stamp while the target is in 360-degree view, the flush state;
/// - the combat items (`0099a530`), the dynamic-cover timer, the location
///   target (searched through the group when the switch at `011f1ae4` is
///   on, then kept up to date: its reference's location, whether the
///   attacker is near it, and what that means for each target type), the
///   threat score (`009a5670`), the firing stamp, the out-of-range weapon
///   check, the inventory update (`009993c0`), the confidence (`009a0c50`)
///   and the flee door, and finally remembers the target.
pub fn fn_00997cf0(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    let target = target(e, this);
    let attacker_space = e.call(GET_SPACE, &args![attacker]).u32();

    if target != 0 && attacker_space != e.call(GET_SPACE, &args![target]).u32() {
        if e.get(this, CombatState::bTargetSpaceDifferent) == 0 {
            if e.get(this, CombatState::bTargetLineOfSight) != 0 {
                let controller = state_controller(e, this);
                e.call(0x0097_f200, &args![controller]);
            }
            e.set(this, CombatState::bTargetSpaceDifferent, 1);
        }
    } else {
        e.set(this, CombatState::bTargetSpaceDifferent, 0);
    }

    if target == 0 {
        e.set(this, CombatState::iTargetDetectionLevel, -1);
        e.set(this, CombatState::bTarget360LineOfSight, 0);
        let bits = negative_max_bits(e);
        let stamp = time_stamp_word(e, bits);
        e.set(this, CombatState::fLastTargetDetectedTimeStamp, stamp);
        let bits = negative_max_bits(e);
        let stamp = time_stamp_word(e, bits);
        e.set(this, CombatState::fLastTarget360LineOfSightTimeStamp, stamp);
        e.set(this, CombatState::cTargetFlushed, -1);
    } else {
        let controller = state_controller(e, this);
        let updated = fn_00998a10(e, controller) != 0;
        if updated || e.get(this, CombatState::pLastTarget) != target {
            e.with_stack(4, |e, level| {
                e.mem.set_u32(level.addr(), 0xffff_ff9c);
                let controller = state_controller(e, this);
                let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
                e.call(
                    0x0098_87b0,
                    &args![
                        group,
                        attacker,
                        target,
                        level,
                        member(this, 0x94),
                        member(this, 0x70),
                        member(this, 0x71)
                    ],
                );
                let detection = e.mem.i32(level.addr());
                e.set(this, CombatState::iTargetDetectionLevel, detection);
            });
            let controller = state_controller(e, this);
            e.call(0x008f_6630, &args![controller, 0u32]);
        }
        if e.get(this, CombatState::bTarget360LineOfSight) != 0 {
            let now = game_time_bits(e);
            let stamp = time_stamp_word(e, now);
            e.set(this, CombatState::fLastTarget360LineOfSightTimeStamp, stamp);
            let location = e.vcall(target, 0x1f4, &args![]).u32();
            copy_words(e, member(this, 0x84), location, 3);
            let controller = state_controller(e, this);
            let item = e.call(0x004f_9bf0, &args![controller]).u32();
            if item == 0 || e.call(0x0082_5c00, &args![item]).u32() != 0xd {
                e.set(this, CombatState::cTargetFlushed, -1);
                e.set(this, CombatState::bGrenadeThrowHighArc, 0);
            }
        }
        if e.get(this, CombatState::bTarget360LineOfSight) != 0
            && e.get(this, CombatState::bTargetBlocked) == 0
        {
            let controller = state_controller(e, this);
            if e.call(0x0098_1940, &args![controller, 0x18u32]).bool() {
                let controller = state_controller(e, this);
                e.call(0x0097_f200, &args![controller]);
            }
        }
        if !fn_00998980(e, this) && e.get(this, CombatState::cTargetFlushed) == -1 {
            e.set(this, CombatState::cTargetFlushed, 0);
        }
    }

    fn_0099a530(e, this);

    let dynamic_cover_timer = member(this, 0x1fc);
    if timer_running(e, dynamic_cover_timer) {
        if timer_expired(e, dynamic_cover_timer) {
            e.call(CLEAR_COVER_LOCATION, &args![this, 0u32]);
            timer_stop(e, dynamic_cover_timer);
        }
    } else {
        let cover = e.get(this, CombatState::pCoverLocation);
        if cover != 0 && e.mem.i32(cover + 0x6c) != -1 {
            let cover = e.get(this, CombatState::pCoverLocation);
            let key = e.mem.u32(cover + 0x6c);
            let cover = e.get(this, CombatState::pCoverLocation);
            if !e.call(0x006e_4650, &args![cover + 0x14, key]).bool() {
                let delay = constant_bits(e, TWO);
                timer_start(e, dynamic_cover_timer, delay);
            }
        }
    }

    if e.global::<u8>(LOCATION_TARGET_SEARCH_011F1AE4) != 0 {
        let location_target = e.get(this, CombatState::pLocationTarget);
        if location_target == 0 || e.mem.u8(location_target) == 2 {
            let mut first = x87_quiet(e.global::<f32>(LOCATION_TARGET_FIRST_DISTANCE));
            let mut second = x87_quiet(e.global::<f32>(LOCATION_TARGET_SECOND_DISTANCE));
            let mut maximum = x87_quiet(e.global::<f32>(LOCATION_TARGET_MAX_DISTANCE));
            if e.call(0x009a_9630, &args![attacker]).bool() {
                first = (first as f64 + first as f64) as f32;
                second = (second as f64 + second as f64) as f32;
            }
            let margin: f64 = e.global(TWO_HUNDRED_FIFTY_SIX_DOUBLE);
            if (maximum as f64) < second as f64 + margin {
                let margin: f64 = e.global(TWO_HUNDRED_FIFTY_SIX_DOUBLE);
                maximum = (second as f64 + margin) as f32;
            }
            let first_bits = x87_quiet_bits(first.to_bits());
            let second_bits = x87_quiet_bits(second.to_bits());
            if e.global::<u8>(LOCATION_TARGET_MODE_011A4D34) != 0 {
                let mut reference = 0;
                let location_target = e.get(this, CombatState::pLocationTarget);
                if location_target != 0 && e.mem.u32(location_target + 4) != 0 {
                    let location_target = e.get(this, CombatState::pLocationTarget);
                    let object = e.mem.u32(location_target + 4);
                    if e.vcall(object, 0x100, &args![]).bool() {
                        let location_target = e.get(this, CombatState::pLocationTarget);
                        reference = e.mem.u32(location_target + 4);
                    }
                }
                let controller = state_controller(e, this);
                let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
                let maximum_bits = x87_quiet_bits(maximum.to_bits());
                let found = e
                    .call(
                        0x0098_cd50,
                        &args![group, attacker, reference, maximum_bits],
                    )
                    .u32();
                if found != 0 {
                    e.call(
                        0x009a_0790,
                        &args![this, found, 2u32, first_bits, second_bits],
                    );
                } else {
                    e.call(0x009a_0a40, &args![this]);
                }
            } else {
                let controller = state_controller(e, this);
                let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
                let found = e.call(0x0098_de00, &args![group, attacker]).u32();
                if found != 0 {
                    e.call(
                        0x009a_0910,
                        &args![this, found, 2u32, first_bits, second_bits],
                    );
                } else {
                    e.call(0x009a_0a40, &args![this]);
                }
            }
        }
    } else {
        let location_target = e.get(this, CombatState::pLocationTarget);
        if location_target != 0 && e.mem.u8(location_target) == 2 {
            e.call(0x009a_0a40, &args![this]);
        }
    }

    if e.get(this, CombatState::pLocationTarget) != 0 {
        update_location_target(e, this, attacker, target, attacker_space);
    }

    let previous_score = x87_quiet(e.get(this, CombatState::fThreatScore));
    let controller = state_controller(e, this);
    let combat_plan = e.call(0x0098_0070, &args![controller]).u32();
    if !fn_009989d0(e, combat_plan) {
        let threshold = setting_value(e, 0x011c_fbcc);
        let score = e.get(this, CombatState::fThreatScore);
        if threshold < score || e.get(this, CombatState::iCombatPermissions) & 0x100 != 0 {
            let location = e.vcall(attacker, 0x1f4, &args![]).u32();
            let (x, y, z) = (
                e.mem.u32(location),
                e.mem.u32(location + 4),
                e.mem.u32(location + 8),
            );
            let manager: u32 = e.global(COMBAT_MANAGER);
            let threat_map = e.call(0x0082_5c00, &args![manager]).u32();
            let stamp = e.get(this, CombatState::fThreatsConsideredTimeStamp);
            let score = e
                .call(
                    0x009a_5670,
                    &args![
                        threat_map,
                        x,
                        y,
                        z,
                        attacker,
                        0.0f32.to_bits(),
                        0.0f32.to_bits(),
                        stamp,
                        0.0f32.to_bits()
                    ],
                )
                .f32();
            e.set(this, CombatState::fThreatScore, score);
            let threshold = setting_value(e, 0x011c_fbcc);
            let score = e.get(this, CombatState::fThreatScore);
            if threshold <= score {
                let threshold = setting_value(e, 0x011c_fbcc);
                if threshold > previous_score {
                    let controller = state_controller(e, this);
                    e.call(0x0097_f200, &args![controller]);
                }
            }
        } else {
            e.set(this, CombatState::fThreatScore, 0.0);
        }
    }

    let firing = e.call(0x006a_7f50, &args![member(this, 0xa4)]).f64();
    let lowest = f32::from_bits(negative_max_bits(e)) as f64;
    if lowest != firing && !e.call(0x0089_4900, &args![attacker]).bool() {
        let bits = negative_max_bits(e);
        let stamp = time_stamp_word(e, bits);
        e.set(this, CombatState::fFiringStartedTimeStamp, stamp);
    }

    if target != 0 {
        let zero: f64 = e.global(ZERO_DOUBLE);
        let range = e.get(this, CombatState::fBetterWeaponOutOfRangeDistance);
        if range as f64 > zero {
            e.with_stack(32, |e, locations| {
                let (mine, theirs) = (locations.addr(), locations.addr() + 16);
                e.call(GET_WORLD_LOCATION, &args![attacker, mine]);
                e.call(GET_WORLD_LOCATION, &args![target, theirs]);
                let distance = e.call(0x0052_7a40, &args![mine, theirs]).f64();
                let range = e.get(this, CombatState::fBetterWeaponOutOfRangeDistance) as f64;
                if range * range > distance {
                    e.call(0x0097_f220, &args![member(this, 0x21c)]);
                }
            });
        }
    }

    let inventory_timer = member(this, 0x21c);
    if e.get(this, CombatState::bInventoryDirty) != 0
        || (timer_running(e, inventory_timer) && timer_expired(e, inventory_timer))
    {
        fn_009993c0(e, this);
    }
    e.call(0x009a_0c50, &args![this, 0u32]);

    if e.get(this, CombatState::iLastFleeDoor) != 0 {
        let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0xe0)]).f64();
        let limit = setting_value(e, 0x011c_eed8);
        if (limit as f64) < elapsed {
            e.set(this, CombatState::iLastFleeDoor, 0);
        }
    }
    e.set(this, CombatState::pLastTarget, target);
}

/// `pLocationTarget`, re-read.
fn current_location_target(e: &Engine, this: Ptr<CombatState>) -> Ptr<CombatLocationTarget> {
    Ptr::new(e.get(this, CombatState::pLocationTarget))
}

/// The location-target part of `00997cf0` (`pLocationTarget` is set).
fn update_location_target(
    e: &mut Engine,
    this: Ptr<CombatState>,
    attacker: u32,
    target: u32,
    attacker_space: u32,
) {
    let lt = current_location_target(e, this);
    if e.get(lt, CombatLocationTarget::cType) == 3 {
        let lt = current_location_target(e, this);
        let space = e.call(0x0084_e3a0, &args![lt.addr() + 8]).u32();
        if space != attacker_space {
            e.call(0x009a_0c50, &args![this, 1u32]);
        }
    }
    let lt = current_location_target(e, this);
    if e.get(lt, CombatLocationTarget::pRef) != 0 {
        e.with_stack(16, |e, location| {
            let lt = current_location_target(e, this);
            let reference = e.get(lt, CombatLocationTarget::pRef);
            let result = e
                .call(GET_WORLD_LOCATION, &args![reference, location])
                .u32();
            let lt = current_location_target(e, this);
            copy_words(e, lt.addr() + 8, result, 4);
        });
    }

    let distance = e.with_stack(12 + 16, |e, locals| {
        let (scale, location) = (locals.addr(), locals.addr() + 12);
        let quarter = constant_bits(e, QUARTER);
        e.call(
            0x0041_6870,
            &args![scale, 1.0f32.to_bits(), 1.0f32.to_bits(), quarter],
        );
        let mine = e.call(GET_WORLD_LOCATION, &args![attacker, location]).u32();
        let lt = current_location_target(e, this);
        e.call(0x0052_7ac0, &args![lt.addr() + 8, mine, scale])
            .f32()
    });

    let mut near_override = false;
    let lt = current_location_target(e, this);
    let kind = e.get(lt, CombatLocationTarget::cType);
    let lt = current_location_target(e, this);
    if (kind == 0 || e.get(lt, CombatLocationTarget::cType) == 1) && target != 0 {
        let controller = state_controller(e, this);
        if e.call(0x0098_1300, &args![controller]).bool() {
            let controller = state_controller(e, this);
            if !e.call(0x0067_a700, &args![controller]).bool() {
                near_override = true;
            }
        }
    }

    if near_override {
        let lt = current_location_target(e, this);
        e.set(lt, CombatLocationTarget::bNearTarget, 1);
    } else {
        let lt = current_location_target(e, this);
        let maximum = e.get(lt, CombatLocationTarget::fMaxDistance);
        let minimum = e.get(lt, CombatLocationTarget::fMinDistance);
        if minimum < maximum && e.call(0x009a_0a80, &args![this]).bool() {
            let radius = e.call(0x009a_0b70, &args![this]).f32();
            let lt = current_location_target(e, this);
            let squared = radius as f64 * radius as f64;
            if e.get(lt, CombatLocationTarget::bNearTarget) != 0 {
                if distance as f64 > squared {
                    let lt = current_location_target(e, this);
                    e.set(lt, CombatLocationTarget::bNearTarget, 0);
                }
            } else if (distance as f64) < squared {
                let lt = current_location_target(e, this);
                e.set(lt, CombatLocationTarget::bNearTarget, 1);
            }
        } else {
            let lt = current_location_target(e, this);
            e.set(lt, CombatLocationTarget::bNearTarget, 0);
            let lt = current_location_target(e, this);
            let minimum = e.get(lt, CombatLocationTarget::fMinDistance) as f64;
            if (distance as f64) < minimum * minimum {
                let lt = current_location_target(e, this);
                e.set(lt, CombatLocationTarget::bNearTarget, 1);
            }
        }
    }

    let lt = current_location_target(e, this);
    match e.get(lt, CombatLocationTarget::cType) {
        0 => {
            let lt = current_location_target(e, this);
            if e.get(lt, CombatLocationTarget::bNearTarget) != 0 && !near_override {
                e.call(0x009a_0a40, &args![this]);
                let controller = state_controller(e, this);
                e.call(0x0098_1380, &args![controller]);
            }
        }
        1 => {
            let controller = state_controller(e, this);
            if fn_00998a30(e, controller) == 0 {
                return;
            }
            let lt = current_location_target(e, this);
            if e.get(lt, CombatLocationTarget::pRef) == 0 {
                return;
            }
            let lt = current_location_target(e, this);
            let object = e.get(lt, CombatLocationTarget::pRef);
            if !e.vcall(object, 0x100, &args![]).bool() {
                return;
            }
            let first = setting_value_address(e, 0x011c_f45c);
            let second = setting_value_address(e, 0x011c_f45c);
            let near = e.mem.f32(first) as f64 * e.mem.f32(second) as f64;
            if (distance as f64) < near {
                let lt = current_location_target(e, this);
                let reference = e.get(lt, CombatLocationTarget::pRef);
                let theirs = e.call(0x0049_97b0, &args![reference]).u8();
                let mine = e.call(0x0049_97b0, &args![attacker]).u8();
                let guard_sneak_timer = member(this, 0x1f4);
                if theirs == mine {
                    timer_stop(e, guard_sneak_timer);
                } else if !timer_running(e, guard_sneak_timer) {
                    let delay = if reference == e.global::<u32>(PLAYER_CHARACTER) {
                        let low = constant_bits(e, HALF);
                        random_float_bits(e, low, 1.0f32.to_bits())
                    } else {
                        let high = constant_bits(e, HALF);
                        let low = constant_bits(e, TENTH);
                        random_float_bits(e, low, high)
                    };
                    timer_start(e, guard_sneak_timer, delay);
                } else if timer_expired(e, guard_sneak_timer) {
                    let controller = state_controller(e, this);
                    e.call(0x0098_1520, &args![controller, theirs as u32]);
                    timer_stop(e, guard_sneak_timer);
                }
            } else {
                let controller = state_controller(e, this);
                if fn_009989f0(e, controller) != 0 {
                    let controller = state_controller(e, this);
                    e.call(0x0098_1520, &args![controller, 0u32]);
                }
            }
        }
        _ => {}
    }
}

// Translated from 00998980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the time since `fLastTarget360LineOfSightTimeStamp` is below the
/// setting at `011cea98` (`FCOMP` of the setting against the elapsed time,
/// true only when the setting is greater).
pub fn fn_00998980(e: &mut Engine, this: Ptr<CombatState>) -> bool {
    let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0x98)]).f64();
    let limit = setting_value(e, 0x011c_ea98);
    limit as f64 > elapsed
}

// Translated from 009989d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009928c0(this, 0x10)` on a `CombatPlan`-side object (the code passes
/// the result of `00980070` here); returns its `AL`.
pub fn fn_009989d0(e: &mut Engine, this: u32) -> bool {
    e.call(0x0099_28c0, &args![this, 0x10u32]).bool()
}

// Translated from 009989f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatController::bForceSneak` (+0xc7).
pub fn fn_009989f0(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this.wrapping_add(0xc7))
}

// Translated from 00998a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatController::bUpdatedDetection` (+0xc5).
pub fn fn_00998a10(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this.wrapping_add(0xc5))
}

// Translated from 00998a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatController::bAttackerCanSneak` (+0xd4).
pub fn fn_00998a30(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this.wrapping_add(0xd4))
}

// Translated from 00998a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00998a50 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The second half of the full update: picks another embedded weapon at
/// random when the embedded-weapon timer allows it (avoiding the current one
/// when the timer was cancelled), retries the blocked explosive projectile
/// through `00992720`, re-evaluates the better-weapon timer (`0099f6d0`),
/// the combat strength (`009a1010`) and threat ratio (`009a1060`), handles a
/// queued path check (`spPathCheckQueue`: its message, `009a1400`) or the
/// target-unreachable timer, updates the permissions (`00999260`), and
/// starts or ends the flee dialogue state.
pub fn fn_00998a50(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    let target = target(e, this);
    let embedded = this.at(CombatState::EmbeddedWeaponsArray).addr();
    let count = e.call(ARRAY_SIZE, &args![embedded]).i32();
    let embedded_timer = member(this, 0x214);
    if count > 1 {
        let controller = state_controller(e, this);
        let switching = e.call(0x0098_1970, &args![controller]).u8();
        if !timer_running(e, embedded_timer) && switching == 0 {
            let delay = setting_bits(e, 0x011c_eb50);
            timer_start(e, embedded_timer, delay);
        }
        if (timer_running(e, embedded_timer) && timer_expired(e, embedded_timer))
            || e.get(this, CombatState::bInitializing) != 0
        {
            let cancelled = fn_009990c0(e, Ptr::new(embedded_timer));
            let chance = setting_bits(e, 0x011c_ec7c);
            if e.call(0x004d_ff00, &args![chance]).bool()
                || e.get(this, CombatState::bInitializing) != 0
                || cancelled
            {
                let mut chosen = 0;
                if cancelled && e.get(this, CombatState::pCurrentEmbeddedWeapon) != 0 {
                    let mut skip = e.call(0x0094_4460, &args![0u32, (count - 1) as u32]).i32();
                    let mut index = 0u32;
                    while index < e.call(ARRAY_SIZE, &args![embedded]).u32() {
                        let slot = e.call(0x006a_7ad0, &args![embedded, index]).u32();
                        let weapon = e.mem.u32(slot);
                        if weapon != e.get(this, CombatState::pCurrentEmbeddedWeapon) {
                            if skip == 0 {
                                chosen = weapon;
                                break;
                            }
                            skip -= 1;
                        }
                        index += 1;
                    }
                } else {
                    let index = e.call(0x0094_4460, &args![0u32, count as u32]).u32();
                    let slot = e.call(0x006a_7ad0, &args![embedded, index]).u32();
                    chosen = e.mem.u32(slot);
                }
                if chosen != e.get(this, CombatState::pCurrentEmbeddedWeapon) {
                    e.set(this, CombatState::pCurrentEmbeddedWeapon, chosen);
                    fn_009993c0(e, this);
                    let controller = state_controller(e, this);
                    e.call(0x0097_f200, &args![controller]);
                }
            }
            timer_stop(e, embedded_timer);
        }
    }

    let blocked_timer = member(this, 0x1dc);
    if e.get(this, CombatState::pExplosiveProjectileBlocked) != 0 {
        if !timer_running(e, blocked_timer) {
            let delay = setting_bits(e, 0x011c_f15c);
            timer_start(e, blocked_timer, delay);
        }
        if timer_expired(e, blocked_timer) {
            let mut clear = true;
            if target != 0 {
                let radius = constant_bits(e, TWO);
                let location = e.vcall(target, 0x1f4, &args![]).u32();
                let blocked = e.get(this, CombatState::pExplosiveProjectileBlocked);
                let manager: u32 = e.global(COMBAT_MANAGER);
                clear = e
                    .call(
                        0x0099_2720,
                        &args![manager, attacker, blocked, location, radius],
                    )
                    .bool();
            }
            if clear {
                e.set(this, CombatState::pExplosiveProjectileBlocked, 0);
            } else {
                let delay = setting_bits(e, 0x011c_f15c);
                timer_start(e, blocked_timer, delay);
            }
        }
    } else {
        timer_stop(e, blocked_timer);
    }

    let better_weapon_timer = member(this, 0x1d4);
    if timer_expired(e, better_weapon_timer) {
        e.call(0x0099_f6d0, &args![this]);
        let disarmed = e.get(this, CombatState::fDisarmedDPS);
        let best = e.get(this, CombatState::fMaxCombatWeaponDPS);
        if best > disarmed {
            let delay = setting_bits(e, 0x011c_f9f8);
            timer_start(e, better_weapon_timer, delay);
            e.set(this, CombatState::fDisarmedDPS, 0.0);
        } else {
            let delay = setting_bits(e, 0x011c_e9d8);
            timer_start(e, better_weapon_timer, delay);
        }
    }
    update_strength_and_threat_ratio(e, this);

    let queue = member(this, 0x224);
    let unreachable_timer = member(this, 0x1cc);
    if e.call(0x0055_9450, &args![queue]).u32() != 0 {
        e.with_stack(8, |e, message| {
            e.call(0x006e_9bd0, &args![message]);
            let queue_object = e.call(0x0055_9450, &args![queue]).u32();
            if e.vcall(queue_object, 0x10, &args![message]).bool() {
                if e.mem.u32(message.addr()) == 0 {
                    fn_00999100(e, this);
                    e.set(this, CombatState::iQueuedPathCheck, -1);
                } else if e.get(this, CombatState::iQueuedPathCheck) == 1
                    && e.call(0x0097_fd40, &args![this]).bool()
                    && target != 0
                {
                    if !e.call(0x009a_1400, &args![this, 0u32]).bool() {
                        e.set(this, CombatState::iQueuedPathCheck, -1);
                    }
                } else {
                    e.set(this, CombatState::iQueuedPathCheck, -1);
                }
                if e.get(this, CombatState::iQueuedPathCheck) == -1 {
                    e.call(0x0090_57d0, &args![queue, 0u32]);
                }
            }
            e.call(0x0080_0190, &args![message]);
        });
    } else if e.call(0x0097_fd40, &args![this]).bool() && target != 0 {
        if !timer_running(e, unreachable_timer) {
            let delay = constant_bits(e, FIVE);
            timer_start(e, unreachable_timer, delay);
        }
        if timer_expired(e, unreachable_timer) {
            let mut queued = false;
            if e.call(0x0046_1580, &args![this, 2u32]).bool()
                && e.call(0x009a_1400, &args![this, 1u32]).bool()
            {
                queued = true;
            }
            if !queued {
                e.call(0x009a_1400, &args![this, 0u32]);
            }
            let delay = constant_bits(e, FIVE);
            timer_start(e, unreachable_timer, delay);
        }
    } else {
        timer_stop(e, unreachable_timer);
    }

    fn_00999260(e, this);
    let controller = state_controller(e, this);
    if e.call(0x0098_1990, &args![controller]).bool() {
        let controller = state_controller(e, this);
        if !fn_00999120(e, controller) && e.get(this, CombatState::cFleeState) == 0 {
            e.set(this, CombatState::cFleeState, 1);
            e.call(0x008a_6840, &args![attacker, 0u32]);
            let controller = state_controller(e, this);
            let dialogue: u32 = e.global(COMBAT_DIALOGUE_MANAGER);
            e.call(0x0098_3970, &args![dialogue, controller, 2u32, 3u32, 1u32]);
        }
    } else {
        let mut clear = true;
        let controller = state_controller(e, this);
        if e.call(0x004f_9bf0, &args![controller]).u32() != 0 {
            let controller = state_controller(e, this);
            let item = e.call(0x004f_9bf0, &args![controller]).u32();
            if e.call(0x0082_5c00, &args![item]).u32() == 6
                && e.call(0x009a_1380, &args![this]).bool()
            {
                clear = false;
            }
        }
        if clear {
            e.set(this, CombatState::cFleeState, 0);
        }
    }
}

/// The combat-strength (`009a1010`, setting `011cf198`) and threat-ratio
/// (`CombatState::UpdateThreatRatio`, setting `011cf804`) timers, as
/// `00998a50` and `00999160` run them.
fn update_strength_and_threat_ratio(e: &mut Engine, this: Ptr<CombatState>) {
    let strength_timer = member(this, 0x204);
    if timer_expired(e, strength_timer) {
        e.call(0x009a_1010, &args![this]);
        let delay = setting_bits(e, 0x011c_f198);
        timer_start(e, strength_timer, delay);
    }
    let ratio_timer = member(this, 0x20c);
    if timer_expired(e, ratio_timer) {
        e.call(0x009a_1060, &args![this]);
        let delay = setting_bits(e, 0x011c_f804);
        timer_start(e, ratio_timer, delay);
    }
}

// Translated from 009990c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a `CombatTimer`'s delay (+4) is below zero.
pub fn fn_009990c0(e: &mut Engine, this: Ptr<CombatTimer>) -> bool {
    let zero: f64 = e.global(ZERO_DOUBLE);
    (e.get(this, CombatTimer::fDelay) as f64) < zero
}

// Translated from 00999100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `006296b0(this, 3)`.
pub fn fn_00999100(e: &mut Engine, this: Ptr<CombatState>) {
    e.call(0x0062_96b0, &args![this, 3u32]);
}

// Translated from 00999120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `006b9130` on the controller's `PlanTimer` (+0xb4) gives exactly
/// zero (`FCOMP` with the `double` 0 at `01012060`, unordered is false).
pub fn fn_00999120(e: &mut Engine, this: u32) -> bool {
    let value = e.call(0x006b_9130, &args![this.wrapping_add(0xb4)]).f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    value == zero
}

// Translated from 00999160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reduced update (the attacker is not in the high process): the target
/// detection through the group (`009887b0`), the inventory when it is dirty
/// (`009993c0`), and the combat-strength and threat-ratio timers.
pub fn fn_00999160(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    let target = target(e, this);
    if target != 0 {
        let controller = state_controller(e, this);
        let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
        e.call(
            0x0098_87b0,
            &args![
                group,
                attacker,
                target,
                member(this, 0x6c),
                member(this, 0x94),
                member(this, 0x70),
                member(this, 0x71)
            ],
        );
    }
    if e.get(this, CombatState::bInventoryDirty) != 0 {
        fn_009993c0(e, this);
    }
    update_strength_and_threat_ratio(e, this);
}

// Translated from 00999260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates `iCombatPermissions` on its two timers: every 5 s the second byte
/// (bit 0x100 from a chance roll `004dff00` on the setting at `011ce954`),
/// every 10 s the low byte (bit 1 from a roll on the combat plan's value
/// `00495460` over 100, bit 2 when that value is above zero).
pub fn fn_00999260(e: &mut Engine, this: Ptr<CombatState>) {
    attacker(e, this);
    let short_timer = member(this, 0x1e4);
    if timer_expired(e, short_timer) {
        let permissions = e.get(this, CombatState::iCombatPermissions) & 0xffff_00ff;
        e.set(this, CombatState::iCombatPermissions, permissions);
        let chance = setting_bits(e, 0x011c_e954);
        if e.call(0x004d_ff00, &args![chance]).bool() {
            let permissions = e.get(this, CombatState::iCombatPermissions) | 0x100;
            e.set(this, CombatState::iCombatPermissions, permissions);
        }
        let delay = constant_bits(e, FIVE);
        timer_start(e, short_timer, delay);
    }
    let long_timer = member(this, 0x1ec);
    if timer_expired(e, long_timer) {
        let permissions = e.get(this, CombatState::iCombatPermissions) & 0xffff_ff00;
        e.set(this, CombatState::iCombatPermissions, permissions);
        let controller = state_controller(e, this);
        let plan = e.call(0x0098_0070, &args![controller]).u32();
        let value = e.call(0x0049_5460, &args![plan]).f32();
        let hundred: f64 = e.global(HUNDRED_DOUBLE);
        let chance = (value as f64 / hundred) as f32;
        if e.call(0x004d_ff00, &args![x87_quiet_bits(chance.to_bits())])
            .bool()
        {
            let permissions = e.get(this, CombatState::iCombatPermissions) | 1;
            e.set(this, CombatState::iCombatPermissions, permissions);
        }
        let zero: f64 = e.global(ZERO_DOUBLE);
        if value as f64 > zero {
            let permissions = e.get(this, CombatState::iCombatPermissions) | 2;
            e.set(this, CombatState::iCombatPermissions, permissions);
        }
        let delay = constant_bits(e, TEN);
        timer_start(e, long_timer, delay);
    }
}

/// Size of the stack frame `009993c0` keeps its locals in; a local at
/// `EBP - k` is at `frame + INVENTORY_FRAME - k`.
const INVENTORY_FRAME: u32 = 0x160;
/// `9999999.0` (`float`): the DPS `009993c0` gives a weapon the attacker
/// cannot unequip.
const FORCED_WEAPON_DPS: u32 = 0x0108_dfac;
/// `1000000.0` (`double`): the bonus `009993c0` gives the current embedded
/// weapon.
const CURRENT_WEAPON_BONUS_DOUBLE: u32 = 0x0106_b798;
/// `10000.0` (`double`): the divisor `009993c0` applies to out-of-range
/// weapons.
const OUT_OF_RANGE_DIVISOR_DOUBLE: u32 = 0x0102_f078;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout\AI\Combat\CombatState.cpp"`.
const SOURCE_FILE_NAME: u32 = 0x0108_dfb0;

// Translated from 009993c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009993c0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Re-evaluates the attacker's inventory for combat (inside a scope guard
/// `00404eb0`/`00404ee0`, category 0x2b, line 0x346 of the source file):
/// - clears the weapon arrays and DPS values;
/// - walks the inventory (`004d4830`): weapons the attacker can use
///   (`008bc9d0`) go into a local 0x14-byte-entry list with their health
///   over 100, form type 0x29 entries into a second list (`007cb2e0`), and
///   form type 0x2f items fill the two usable combat items (effect scores
///   for types 0x11 and 0x12);
/// - a weapon the attacker cannot unequip (`ExtraDataList::GetCanNotWear`
///   on the acquired object) becomes the only weapon, with DPS 9999999;
/// - otherwise every listed weapon gets its ammunition checked (queueing a
///   reload or an ammunition change where needed), its DPS (`00646060`;
///   weapons without DPS leave the list), its optimal range against the
///   target distance, then the best weapon per type, the maximum DPS
///   values, the embedded weapons (`0046e8c0`) and the hand-to-hand DPS;
/// - finally checks the projectile of the first weapon against the target
///   (`00992720`, else `00442a80`) and restarts the inventory timer.
pub fn fn_009993c0(e: &mut Engine, this: Ptr<CombatState>) {
    e.with_stack(INVENTORY_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + INVENTORY_FRAME - k;
        let scope = local(0x3c);
        e.call(
            0x0040_4eb0,
            &args![scope, 0x2bu32, 1u32, SOURCE_FILE_NAME, 0x346u32],
        );
        let attacker = attacker(e, this);
        let target = target(e, this);
        let controller = state_controller(e, this);
        let plan = e.call(0x0098_0070, &args![controller]).u32();
        let mut distance = 0.0f32;
        if target != 0 {
            let (mine, theirs) = (local(0x90), local(0x80));
            e.call(GET_WORLD_LOCATION, &args![attacker, mine]);
            e.call(GET_WORLD_LOCATION, &args![target, theirs]);
            distance = e.call(0x0052_7a40, &args![mine, theirs]).f32();
        }
        fn_0099a2a0(e, this);
        e.set(this, CombatState::bInventoryDirty, 0);
        e.set(this, CombatState::fMaxCombatWeaponDPS, 0.0);
        e.set(this, CombatState::fMaxRangedWeaponDPS, 0.0);
        e.set(this, CombatState::fMaxModifiedCombatWeaponDPS, 0.0);
        e.set(this, CombatState::fBetterWeaponOutOfRangeDistance, 0.0);
        e.set(this, CombatState::iHasCombatWeapon, 0);
        e.set(this, CombatState::iHasCombatItem, 0);
        e.call(FILL_BYTES, &args![member(this, 0x3c), 0u32, 0x1cu32]);
        e.call(FILL_BYTES, &args![member(this, 0x1a4), 0u32, 8u32]);
        let item_scores = local(0x34);
        e.call(FILL_BYTES, &args![item_scores, 0u32, 8u32]);
        let weapons = local(0x68);
        e.call(0x009a_37e0, &args![weapons, 0x20u32, 0u32]);
        let others = local(0x54);
        e.call(0x009a_3750, &args![others, 0x20u32, 0u32]);
        let item = local(0x44);
        e.mem.set_u32(item, 0);
        let iterator = local(0x24);
        e.call(0x0052_7270, &args![iterator]);
        let changes = e.call(0x004b_f220, &args![attacker]).u32();

        while e
            .call(0x004d_4830, &args![changes, iterator, item, 0u32])
            .bool()
        {
            let entry = e.mem.u32(item);
            let object = e.call(ARRAY_SIZE, &args![entry]).u32();
            match e.call(0x0040_1170, &args![object]).u32() {
                0x28 => {
                    if e.call(0x008b_c9d0, &args![attacker, object, 1u32]).bool() {
                        let index = e.call(0x0078_81d0, &args![weapons]).u32();
                        let slot = e.call(0x006b_3160, &args![weapons, index]).u32();
                        e.call(FILL_BYTES, &args![slot, 0u32, 0x14u32]);
                        let slot = e.call(0x006b_3160, &args![weapons, index]).u32();
                        e.mem.set_u32(slot, object);
                        let entry = e.mem.u32(item);
                        let health = e.call(0x004b_cdb0, &args![entry, 1u32]).f64();
                        let hundred: f64 = e.global(HUNDRED_DOUBLE);
                        let health = health / hundred;
                        let slot = e.call(0x006b_3160, &args![weapons, index]).u32();
                        e.mem.set_f32(slot + 4, health as f32);
                    }
                }
                0x29 => {
                    let value = local(0x128);
                    e.mem.set_u32(value, object);
                    e.call(0x007c_b2e0, &args![others, value]);
                }
                0x2f => {
                    for slot in 0u32..2 {
                        let mut score = 0.0f32;
                        let kind = slot + 0x11;
                        if kind == 0x11 {
                            let effects = object + 0x3c;
                            score = e
                                .call(0x0040_5c00, &args![effects, 0u32, 0u32, 0x10u32])
                                .f32();
                            let more = e
                                .call(0x0040_5c00, &args![effects, 0x22u32, 0u32, 0x10u32])
                                .f64();
                            score = (more + score as f64) as f32;
                        } else if kind == 0x12
                            && !e.call(0x0099_28c0, &args![plan, 0x100u32]).bool()
                        {
                            score = e
                                .call(0x0040_5c00, &args![object + 0x3c, 0xcu32, 0u32, 0x4du32])
                                .f32();
                        }
                        let best = e.mem.f32(item_scores + 4 * slot);
                        if best < score {
                            e.mem.set_u32(member(this, 0x1a4) + 4 * slot, object);
                            e.mem.set_f32(item_scores + 4 * slot, x87_quiet(score));
                            let items = e.get(this, CombatState::iHasCombatItem);
                            e.set(
                                this,
                                CombatState::iHasCombatItem,
                                1u32.wrapping_shl(kind) | items,
                            );
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        let mut forced = 0;
        let process = e.call(0x008d_8520, &args![attacker]).u32();
        let acquired = e.vcall(process, 0x148, &args![]).u32();
        if acquired != 0 && e.call(0x0055_9450, &args![acquired]).u32() != 0 {
            let extra = e.call(0x0055_9450, &args![acquired]).u32();
            let list = e.call(0x0068_15c0, &args![extra]).u32();
            if e.mem.u32(list) != 0 {
                let extra = e.call(0x0055_9450, &args![acquired]).u32();
                let list = e.call(0x0068_15c0, &args![extra]).u32();
                let first = e.mem.u32(list);
                if e.call(0x0041_8b10, &args![first]).bool() {
                    forced = e.call(ARRAY_SIZE, &args![acquired]).u32();
                }
            }
        }

        if forced == 0 {
            rate_weapons(
                e,
                this,
                frame,
                WeaponContext {
                    attacker,
                    target,
                    plan,
                    changes,
                    acquired,
                    distance,
                },
            );
        } else {
            let kind = e.call(0x0052_2c80, &args![forced]).u32();
            let weapon_slot = member(this, 0xc).wrapping_add(4u32.wrapping_mul(kind));
            e.mem.set_u32(weapon_slot, forced);
            let dps_slot = member(this, 0x3c).wrapping_add(4u32.wrapping_mul(kind));
            let dps = constant_bits(e, FORCED_WEAPON_DPS);
            e.mem.set_u32(dps_slot, dps);
            let types = e.get(this, CombatState::iHasCombatWeapon);
            e.set(
                this,
                CombatState::iHasCombatWeapon,
                1u32.wrapping_shl(kind) | types,
            );
            let dps = x87_quiet(
                e.mem
                    .f32(member(this, 0x3c).wrapping_add(4u32.wrapping_mul(kind))),
            );
            e.set(this, CombatState::fMaxCombatWeaponDPS, dps);
            let dps = x87_quiet(
                e.mem
                    .f32(member(this, 0x3c).wrapping_add(4u32.wrapping_mul(kind))),
            );
            e.set(this, CombatState::fMaxModifiedCombatWeaponDPS, dps);
            if kind == 1 || kind == 0 {
                let dps = x87_quiet(
                    e.mem
                        .f32(member(this, 0x3c).wrapping_add(4u32.wrapping_mul(kind))),
                );
                e.set(this, CombatState::fMaxRangedWeaponDPS, dps);
            }
        }

        if target != 0 && e.get(this, CombatState::pCombatWeapons) != 0 {
            let weapon = e.get(this, CombatState::pCombatWeapons);
            let ammo = e.call(0x0052_5a90, &args![weapon, attacker]).u32();
            let projectile = e.call(0x004f_d3c0, &args![ammo]).u32();
            let radius = constant_bits(e, TWO);
            let location = e.vcall(target, 0x1f4, &args![]).u32();
            let manager: u32 = e.global(COMBAT_MANAGER);
            if !e
                .call(
                    0x0099_2720,
                    &args![manager, attacker, projectile, location, radius],
                )
                .bool()
            {
                e.call(0x0044_2a80, &args![this, projectile]);
            }
        }
        let delay = setting_bits(e, 0x011c_f660);
        timer_start(e, member(this, 0x21c), delay);
        e.call(0x0052_72b0, &args![iterator]);
        e.call(0x0096_a5f0, &args![others]);
        e.call(0x009a_3810, &args![weapons]);
        e.call(0x0040_4ee0, &args![scope]);
    });
}

/// What `009993c0` hands to its weapon rating part.
struct WeaponContext {
    attacker: u32,
    target: u32,
    plan: u32,
    changes: u32,
    acquired: u32,
    distance: f32,
}

/// The weapon rating part of `009993c0` (no forced weapon): `frame` is its
/// stack frame (see [`INVENTORY_FRAME`]); the local weapon list is at
/// `EBP-0x68`, the ammunition list at `EBP-0x54`. Each weapon entry is
/// 0x14 bytes: weapon, health (+4), DPS (+8), optimal range (+0xc), in-range
/// flag (+0x10).
fn rate_weapons(e: &mut Engine, this: Ptr<CombatState>, frame: Ptr, context: WeaponContext) {
    let WeaponContext {
        attacker,
        target,
        plan,
        changes,
        acquired,
        distance,
    } = context;
    let local = |k: u32| frame.addr() + INVENTORY_FRAME - k;
    let weapons = local(0x68);
    let others = local(0x54);
    let mut best_out_of_range = 0.0f32;
    let mut best_out_of_range_weapon = 0u32;
    let mut best_in_range = 0.0f32;
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![weapons]).u32() {
        let entry = e.call(0x006d_ad90, &args![weapons, index]).u32();
        let weapon = e.mem.u32(entry);
        let owner = if acquired != 0 && weapon == e.call(ARRAY_SIZE, &args![acquired]).u32() {
            attacker
        } else {
            0
        };
        let ammo = local(0xc8);
        let current_ammo = e.call(0x0052_5980, &args![weapon, owner]).u32();
        e.mem.set_u32(ammo, current_ammo);
        let has_ammo = current_ammo == 0 || e.call(0x0099_62f0, &args![others, ammo]).bool();
        if has_ammo {
            if acquired != 0 && weapon == e.call(ARRAY_SIZE, &args![acquired]).u32() {
                let process = e.call(0x008d_8520, &args![attacker]).u32();
                let object = e.vcall(process, 0x14c, &args![]).u32();
                if object != 0 {
                    let equipped = e.vcall(attacker, 0x1e4, &args![]).u32();
                    let acquired_weapon = e.call(ARRAY_SIZE, &args![acquired]).u32();
                    if equipped != 0 && acquired_weapon != 0 {
                        let needs_reload = object != 0
                            && e.call(0x0072_6070, &args![object]).i32() <= 0
                            && acquired_weapon != 0
                            && e.call(0x0052_4b60, &args![acquired_weapon]).u8() != 0;
                        let busy = e.call(0x008a_8870, &args![attacker]).bool()
                            || (equipped != 0 && e.call(0x0088_46c0, &args![equipped]).bool());
                        if needs_reload && !busy {
                            e.call(0x008a_8840, &args![attacker]);
                        }
                    }
                }
            }
        } else {
            let found = local(0xd1);
            e.mem.set_u8(found, 0);
            let other_ammo = local(0xd8);
            let replacement = e.call(0x004c_7300, &args![changes, weapon, found]).u32();
            e.mem.set_u32(other_ammo, replacement);
            if replacement == 0 || !e.call(0x0099_62f0, &args![others, other_ammo]).bool() {
                e.call(0x0098_f1f0, &args![weapons, index, 1u32]);
                // The code steps the index back and the loop forward again.
                continue;
            }
            if acquired != 0 && weapon == e.call(ARRAY_SIZE, &args![acquired]).u32() {
                let equipped = e.vcall(attacker, 0x1e4, &args![]).u32();
                let busy = e.call(0x008a_8870, &args![attacker]).bool()
                    || (equipped != 0 && e.call(0x0088_46c0, &args![equipped]).bool());
                if !busy {
                    let process = e.call(0x008d_8520, &args![attacker]).u32();
                    let replacement = e.mem.u32(other_ammo);
                    let count = e.call(0x004c_8f30, &args![changes, replacement]).u32();
                    let block = e.call(0x0040_1000, &args![0xcu32]).u32();
                    let change = if block == 0 {
                        0
                    } else {
                        let replacement = e.mem.u32(other_ammo);
                        e.call(0x004b_c550, &args![block, replacement, count]).u32()
                    };
                    e.vcall(process, 0x168, &args![change]);
                    let object = e.vcall(process, 0x14c, &args![]).u32();
                    e.call(0x006e_cd40, &args![object, 0u32]);
                    e.call(0x008a_8840, &args![attacker]);
                }
            }
        }

        let health = x87_quiet_bits(e.mem.u32(entry + 4));
        let dps = e
            .call(
                0x0064_6060,
                &args![attacker, weapon, health, plan, 0u32, 0u32],
            )
            .f32();
        e.mem.set_f32(entry + 8, dps);
        let zero: f64 = e.global(ZERO_DOUBLE);
        let dps = e.mem.f32(entry + 8) as f64;
        if dps <= zero {
            e.call(0x0098_f1f0, &args![weapons, index, 1u32]);
            // The code steps the index back and the loop forward again.
            continue;
        }
        e.mem.set_u8(entry + 0x10, 1);
        e.mem.set_f32(entry + 0xc, 0.0);
        let kind = e.call(0x0052_2c80, &args![weapon]).u32();
        if target != 0 && (kind == 1 || kind == 0) {
            let (minimum, preferred, maximum) = (local(0x108), local(0x104), local(0x100));
            e.mem.set_f32(minimum, 0.0);
            e.mem.set_f32(preferred, 0.0);
            e.mem.set_f32(maximum, 0.0);
            let controller = state_controller(e, this);
            e.call(
                0x009a_9180,
                &args![controller, weapon, minimum, preferred, maximum],
            );
            let range = x87_quiet(e.mem.f32(maximum));
            e.mem.set_f32(entry + 0xc, range);
            let controller = state_controller(e, this);
            if e.call(0x0070_31c0, &args![controller]).u32() == weapon {
                let scale = setting_value_address(e, 0x011c_e4ac);
                let scaled = e.mem.f32(maximum) as f64 * e.mem.f32(scale) as f64;
                e.mem.set_f32(maximum, scaled as f32);
            }
            let range = e.mem.f32(maximum) as f64;
            let dps = e.mem.f32(entry + 8);
            if distance as f64 > range * range {
                e.mem.set_u8(entry + 0x10, 0);
                if best_out_of_range < dps {
                    best_out_of_range = x87_quiet(e.mem.f32(entry + 8));
                    best_out_of_range_weapon = weapon;
                }
            } else if best_in_range < dps {
                best_in_range = x87_quiet(e.mem.f32(entry + 8));
            }
        }
        index = index.wrapping_add(1);
    }

    let zero: f64 = e.global(ZERO_DOUBLE);
    if best_in_range as f64 > zero {
        best_out_of_range_weapon = 0;
    }

    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![weapons]).u32() {
        let entry = e.call(0x006d_ad90, &args![weapons, index]).u32();
        let weapon_local = local(0x114);
        let weapon = e.mem.u32(entry);
        e.mem.set_u32(weapon_local, weapon);
        let kind = e.call(0x0052_2c80, &args![weapon]).u32();
        let mut score = x87_quiet(e.mem.f32(entry + 8));
        if weapon == e.get(this, CombatState::pCurrentEmbeddedWeapon) {
            let bonus: f64 = e.global(CURRENT_WEAPON_BONUS_DOUBLE);
            score = (score as f64 + bonus) as f32;
        }
        if weapon == best_out_of_range_weapon {
            e.mem.set_u8(entry + 0x10, 1);
        }
        if e.mem.u8(entry + 0x10) == 0 {
            let divisor: f64 = e.global(OUT_OF_RANGE_DIVISOR_DOUBLE);
            score = (score as f64 / divisor) as f32;
            let range = e.mem.f32(entry + 0xc);
            if e.get(this, CombatState::fBetterWeaponOutOfRangeDistance) < range
                && best_in_range < e.mem.f32(entry + 8)
            {
                let range = x87_quiet(e.mem.f32(entry + 0xc));
                e.set(this, CombatState::fBetterWeaponOutOfRangeDistance, range);
            }
        }
        if e.call(0x0046_e8c0, &args![weapon]).bool() && e.mem.u8(entry + 0x10) != 0 {
            let embedded = this.at(CombatState::EmbeddedWeaponsArray).addr();
            e.call(0x007c_b2e0, &args![embedded, weapon_local]);
        }
        let dps = x87_quiet_bits(e.mem.u32(entry + 8));
        let best = x87_quiet_bits(e.get(this, CombatState::fMaxCombatWeaponDPS).to_bits());
        let best = e.call(0x0040_4010, &args![best, dps]).f32();
        e.set(this, CombatState::fMaxCombatWeaponDPS, best);
        if kind == 1 || kind == 0 {
            let dps = x87_quiet_bits(e.mem.u32(entry + 8));
            let best = x87_quiet_bits(e.get(this, CombatState::fMaxRangedWeaponDPS).to_bits());
            let best = e.call(0x0040_4010, &args![best, dps]).f32();
            e.set(this, CombatState::fMaxRangedWeaponDPS, best);
        }
        let modified = x87_quiet_bits(score.to_bits());
        let best = x87_quiet_bits(
            e.get(this, CombatState::fMaxModifiedCombatWeaponDPS)
                .to_bits(),
        );
        let best = e.call(0x0040_4010, &args![best, modified]).f32();
        e.set(this, CombatState::fMaxModifiedCombatWeaponDPS, best);
        let types = e.get(this, CombatState::iHasCombatWeapon);
        e.set(
            this,
            CombatState::iHasCombatWeapon,
            1u32.wrapping_shl(kind) | types,
        );
        let dps_slot = member(this, 0x3c).wrapping_add(4u32.wrapping_mul(kind));
        if e.mem.f32(dps_slot) < score {
            let weapon_slot = member(this, 0xc).wrapping_add(4u32.wrapping_mul(kind));
            e.mem.set_u32(weapon_slot, weapon);
            e.mem.set_f32(dps_slot, x87_quiet(score));
        }
        index = index.wrapping_add(1);
    }

    let unarmed = e
        .call(
            0x0064_6060,
            &args![attacker, 0u32, 1.0f32.to_bits(), plan, 0u32, 0u32],
        )
        .f32();
    e.set(this, CombatState::fUnarmedDPS, unarmed);
    let unarmed = x87_quiet_bits(e.get(this, CombatState::fUnarmedDPS).to_bits());
    let best = x87_quiet_bits(e.get(this, CombatState::fMaxCombatWeaponDPS).to_bits());
    let best = e.call(0x0040_4010, &args![best, unarmed]).f32();
    e.set(this, CombatState::fMaxCombatWeaponDPS, best);
    let unarmed = x87_quiet_bits(e.get(this, CombatState::fUnarmedDPS).to_bits());
    let best = x87_quiet_bits(
        e.get(this, CombatState::fMaxModifiedCombatWeaponDPS)
            .to_bits(),
    );
    let best = e.call(0x0040_4010, &args![best, unarmed]).f32();
    e.set(this, CombatState::fMaxModifiedCombatWeaponDPS, best);
    let types = e.get(this, CombatState::iHasCombatWeapon) | 0x40;
    e.set(this, CombatState::iHasCombatWeapon, types);
}

// Translated from 0099a2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the six combat weapons and empties the embedded weapon array
/// (`008454f0(0)`).
pub fn fn_0099a2a0(e: &mut Engine, this: Ptr<CombatState>) {
    e.call(FILL_BYTES, &args![member(this, 0xc), 0u32, 0x18u32]);
    let embedded = this.at(CombatState::EmbeddedWeaponsArray).addr();
    e.call(0x0084_54f0, &args![embedded, 0u32]);
}

// Translated from 0099a2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Of the seven entries `0097ac90(i)` rates on `this`, returns the
/// `0097c150(i)` of the first highest one above zero (0 for entry 6).
pub fn fn_0099a2d0(e: &mut Engine, this: u32) -> u32 {
    let mut best = 0.0f32;
    let mut result = 0;
    for index in 0u32..7 {
        let value = e.call(0x0097_ac90, &args![this, index]).f32();
        if best < value {
            best = x87_quiet(value);
            result = if index == 6 {
                0
            } else {
                e.call(0x0097_c150, &args![this, index]).u32()
            };
        }
    }
    result
}

// Translated from 0099a360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `reference` may be treated as reachable: always for another
/// reference than `pLastTarget`; for the last target, false only when the
/// detection level is above 0 and the world space (`TES::GetWorldSpace`
/// on the controller) has virtual slot 0x34 equal to 7 and `0099a3e0`
/// holds for it. The first argument word is not read.
pub fn fn_0099a360(e: &mut Engine, this: Ptr<CombatState>, _unused_1: u32, reference: u32) -> bool {
    if reference != e.get(this, CombatState::pLastTarget) {
        return true;
    }
    let mut result = true;
    if e.get(this, CombatState::iTargetDetectionLevel) > 0 {
        let controller = state_controller(e, this);
        let space = e.call(0x004f_d3e0, &args![controller]).u32();
        if space != 0 && e.vcall(space, 0x34, &args![]).u32() == 7 && fn_0099a3e0(e, space) {
            result = false;
        }
    }
    result
}

// Translated from 0099a3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at +0x10 of `this` is neither 3 nor 4.
pub fn fn_0099a3e0(e: &mut Engine, this: u32) -> bool {
    let value = e.mem.u32(this.wrapping_add(0x10));
    value != 3 && value != 4
}

/// The first parts of `0099a420` and `0099a530`: the combat items `0097cfc0`
/// returns for indices 0 to 4 that `keep` rejects are released
/// (`00981340`); then, unless `skip_pair` or the items 3 and 4 are still the
/// ones read at the start (`pCombatItems[3]`, `[4]`), both are released.
fn release_combat_items(
    e: &mut Engine,
    this: Ptr<CombatState>,
    skip_pair: bool,
    mut release: impl FnMut(&mut Engine, u32) -> bool,
) {
    let third = e.mem.u32(member(this, 0x19c));
    let fourth = e.mem.u32(member(this, 0x1a0));
    for index in 0u32..5 {
        let item = e.call(0x0097_cfc0, &args![this, index]).u32();
        if item != 0 && release(e, item) {
            e.call(0x0098_1340, &args![this, index]);
        }
    }
    if skip_pair {
        return;
    }
    if third == e.call(0x0097_cfc0, &args![this, 3u32]).u32()
        && fourth == e.call(0x0097_cfc0, &args![this, 4u32]).u32()
    {
        return;
    }
    e.call(0x0098_1340, &args![this, 3u32]);
    e.call(0x0098_1340, &args![this, 4u32]);
}

// Translated from 0099a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forgets `reference`: releases the combat items that hold it (and, unless
/// `keep_pair`, items 3 and 4 when they changed), clears the location
/// target, the combat location (when `BaseProcess::GetCurrentProcedureIndex`
/// on it gives the reference) and `pLastTarget` that refer to it.
pub fn fn_0099a420(e: &mut Engine, this: Ptr<CombatState>, reference: u32, keep_pair: u8) {
    release_combat_items(e, this, keep_pair != 0, |e, item| {
        e.mem.u32(item) == reference
    });
    let location_target = e.get(this, CombatState::pLocationTarget);
    if location_target != 0 {
        let location_target = e.get(this, CombatState::pLocationTarget);
        if e.mem.u32(location_target + 4) == reference {
            e.call(0x009a_0a40, &args![this]);
        }
    }
    if e.get(this, CombatState::pCombatLocation) != 0 {
        let location = e.get(this, CombatState::pCombatLocation);
        if e.call(0x0044_edb0, &args![location]).u32() == reference {
            e.call(0x009a_0680, &args![this]);
        }
    }
    if e.get(this, CombatState::pLastTarget) == reference {
        e.set(this, CombatState::pLastTarget, 0);
    }
}

// Translated from 0099a530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the combat items whose reference `00437b90` rejects (and items 3
/// and 4 when they changed meanwhile).
pub fn fn_0099a530(e: &mut Engine, this: Ptr<CombatState>) {
    release_combat_items(e, this, false, |e, item| {
        let reference = e.mem.u32(item);
        reference != 0 && e.call(0x0043_7b90, &args![reference]).bool()
    });
}

// Translated from 0099a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records an attack now: `fLastAttackTimeStamp`, and
/// `fFiringStartedTimeStamp` when it was still unset (`-FLT_MAX`, read
/// through `006a7f50`). The argument word is not read.
pub fn fn_0099a5e0(e: &mut Engine, this: Ptr<CombatState>, _unused_1: u32) {
    let now = game_time_bits(e);
    let stamp = time_stamp_word(e, now);
    e.set(this, CombatState::fLastAttackTimeStamp, stamp);
    let firing = e.call(0x006a_7f50, &args![member(this, 0xa4)]).f64();
    let lowest = f32::from_bits(negative_max_bits(e)) as f64;
    if lowest == firing {
        let now = game_time_bits(e);
        let stamp = time_stamp_word(e, now);
        e.set(this, CombatState::fFiringStartedTimeStamp, stamp);
    }
}

// Translated from 0099a650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unless `skip`, `0097f220` on the embedded-weapon timer. The first
/// argument word is not read.
pub fn fn_0099a650(e: &mut Engine, this: Ptr<CombatState>, _unused_1: u32, skip: u8) {
    if skip == 0 {
        e.call(0x0097_f220, &args![member(this, 0x214)]);
    }
}

// Translated from 0099a680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fLastAttackTimeStamp` to now. The argument word is not read.
pub fn fn_0099a680(e: &mut Engine, this: Ptr<CombatState>, _unused_1: u32) {
    let now = game_time_bits(e);
    let stamp = time_stamp_word(e, now);
    e.set(this, CombatState::fLastAttackTimeStamp, stamp);
}

// Translated from 0099a6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The attacker was damaged: more than 1 s after the previous hit,
/// `009a0c50(1)`; `fLastDamagedByAttackerTimeStamp` becomes now; in flee
/// state 2, `CombatState::BoostConfidence(1)`. The two argument words are
/// not read.
pub fn fn_0099a6b0(e: &mut Engine, this: Ptr<CombatState>, _unused_1: u32, _unused_2: u32) {
    let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0xa0)]).f64();
    let one: f64 = e.global(ONE_DOUBLE);
    if elapsed > one {
        e.call(0x009a_0c50, &args![this, 1u32]);
    }
    let now = game_time_bits(e);
    let stamp = time_stamp_word(e, now);
    e.set(this, CombatState::fLastDamagedByAttackerTimeStamp, stamp);
    if e.get(this, CombatState::cFleeState) == 2 {
        e.call(0x009a_1280, &args![this, 1u32]);
    }
}

// Translated from 0099a720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The attacker lost its weapon: with a better maximum DPS than the
/// disarmed one, remembers it as the disarmed DPS and starts the
/// better-weapon timer (setting `011ce058`); then re-evaluates the
/// inventory (`009993c0`).
pub fn fn_0099a720(e: &mut Engine, this: Ptr<CombatState>) {
    let disarmed = e.get(this, CombatState::fDisarmedDPS);
    let best = e.get(this, CombatState::fMaxCombatWeaponDPS);
    if best > disarmed {
        let best = x87_quiet(e.get(this, CombatState::fMaxCombatWeaponDPS));
        e.set(this, CombatState::fDisarmedDPS, best);
        let delay = setting_bits(e, 0x011c_e058);
        timer_start(e, member(this, 0x1d4), delay);
    }
    fn_009993c0(e, this);
}

// Translated from 0099a780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::PreActivateReference` (Xbox PDB): when combat item 2 holds
/// `reference`, the controller lets it go (`00981380`); then forgets the
/// reference (`0099a420`, keeping items 3 and 4).
pub fn combat_state_pre_activate_reference(e: &mut Engine, this: Ptr<CombatState>, reference: u32) {
    let item = e.call(0x0097_cfc0, &args![this, 2u32]).u32();
    if item != 0 && e.mem.u32(item) == reference {
        let controller = state_controller(e, this);
        e.call(0x0098_1380, &args![controller]);
    }
    fn_0099a420(e, this, reference, 1);
}

// Translated from 0099a7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// While fleeing (`CombatController::IsFleeing`), an activated object whose
/// form (`BGSSaveFormBuffer::GetForm`, then `00401170`) has type 0x1c
/// becomes the last flee door (`0084e3a0` of `activated`), stamped now. The
/// last three argument words are not read.
pub fn fn_0099a7d0(
    e: &mut Engine,
    this: Ptr<CombatState>,
    activated: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
    let controller = state_controller(e, this);
    if !e.call(0x0098_1990, &args![controller]).bool() {
        return;
    }
    let form = e.call(0x007a_f430, &args![activated]).u32();
    if e.call(0x0040_1170, &args![form]).u32() != 0x1c {
        return;
    }
    let door = e.call(0x0084_e3a0, &args![activated]).u32();
    e.set(this, CombatState::iLastFleeDoor, door);
    let now = game_time_bits(e);
    let stamp = time_stamp_word(e, now);
    e.set(this, CombatState::fLastFleeDoorTimeStamp, stamp);
}

// Translated from 0099a840 (decompiled, FalloutNV.exe 1.4.0.525)
/// A usable combat item was used: restarts its timer (in
/// `UsableCombatItemTimers`) with the delay of its kind (0x11: the setting
/// at `011cfdd8`, which the code then overwrites with the one at `011cf480`,
/// as for 0x12).
pub fn fn_0099a840(e: &mut Engine, this: Ptr<CombatState>, item: u32) {
    for index in 0u32..2 {
        if e.mem.u32(member(this, 0x1a4) + 4 * index) != item {
            continue;
        }
        let mut delay = 0.0f32.to_bits();
        let kind = index + 0x11;
        if kind == 0x11 {
            delay = setting_bits(e, 0x011c_fdd8);
        }
        if kind == 0x11 || kind == 0x12 {
            delay = setting_bits(e, 0x011c_f480);
        }
        timer_start(e, member(this, 0x1ac) + 8 * index, delay);
        return;
    }
}

// Translated from 0099a8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The attacker's cover was hit or the situation changed: with a cover
/// location and a target, unless the cover is still safe
/// (`CombatState::CheckCoverSafe`) and `0097fb20` holds, the cover is marked
/// (`005407b0(1)`); a combat location of type 0 or 1 is cleared
/// (`009a0680`); then `00999100` and the inventory update (`009993c0`).
pub fn fn_0099a8d0(e: &mut Engine, this: Ptr<CombatState>) {
    let target = target(e, this);
    if e.get(this, CombatState::pCoverLocation) != 0 && target != 0 {
        let safe = e.with_stack(16, |e, scratch| {
            let cover = e.get(this, CombatState::pCoverLocation);
            let point = e.call(0x005c_3420, &args![cover + 0x14, scratch]).u32();
            let cover = e.get(this, CombatState::pCoverLocation);
            e.call(0x0099_ccd0, &args![this, cover + 0x14, point, target, 0u32])
                .bool()
        });
        if !(safe && e.call(0x0097_fb20, &args![this]).bool()) {
            let cover = e.get(this, CombatState::pCoverLocation);
            e.call(0x0054_07b0, &args![cover, 1u32]);
        }
    }
    if e.get(this, CombatState::pCombatLocation) != 0 {
        let location = e.get(this, CombatState::pCombatLocation);
        let kind = e.mem.u32(location + 0x14);
        if kind == 0 || e.mem.u32(location + 0x14) == 1 {
            e.call(0x009a_0680, &args![this]);
        }
    }
    fn_00999100(e, this);
    fn_009993c0(e, this);
}

// Translated from 0099a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears each cover location for which `005dc960` fails.
pub fn fn_0099a9a0(e: &mut Engine, this: Ptr<CombatState>) {
    let cover = e.get(this, CombatState::pCoverLocation);
    if cover != 0 && !e.call(0x005d_c960, &args![cover]).bool() {
        e.call(CLEAR_COVER_LOCATION, &args![this, 0u32]);
    }
    let cover = e.get(this, CombatState::pSecondaryCoverLocation);
    if cover != 0 && !e.call(0x005d_c960, &args![cover]).bool() {
        e.call(CLEAR_SECONDARY_COVER_LOCATION, &args![this, 0u32]);
    }
}

// Translated from 0099aa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A procedure of `kind` ended for `reference`: kind 15 also tells the group
/// (`0098bd70`); a combat location of that kind is cleared (`009a0680`).
pub fn fn_0099aa10(e: &mut Engine, this: Ptr<CombatState>, reference: u32, kind: u32) {
    if kind == 0xf {
        let controller = state_controller(e, this);
        let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
        e.call(0x0098_bd70, &args![group, reference]);
    }
    if e.get(this, CombatState::pCombatLocation) != 0 {
        let location = e.get(this, CombatState::pCombatLocation);
        if e.call(0x0082_5c00, &args![location]).u32() == kind {
            e.call(0x009a_0680, &args![this]);
        }
    }
}

// Translated from 0099aa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingCoverLocation::PathingCoverLocation` (Xbox PDB), the copy
/// constructor: the `PathingLocation` part (`006dcc40`), the vtable
/// `0106cbc0` (RTTI `.?AVPathingCoverLocation@@`), the three edge vectors
/// and the seven bytes from +0x4c.
pub fn pathing_cover_location_pathing_cover_location(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(0x006d_cc40, &args![this, other]);
    let (to, from) = (this.addr(), other.addr());
    e.mem.set_u32(to, 0x0106_cbc0);
    copy_words(e, to + 0x28, from + 0x28, 3);
    copy_words(e, to + 0x34, from + 0x34, 3);
    copy_words(e, to + 0x40, from + 0x40, 3);
    for off in 0x4c..0x53 {
        let byte = e.mem.u8(from + off);
        e.mem.set_u8(to + off, byte);
    }
    this
}

// Translated from 0099ab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a target seen in 360-degree view within 120 s (not later; an
/// unordered time counts as within), copies `LastSeenTargetLocation` to
/// `out` and returns true.
pub fn fn_0099ab50(e: &mut Engine, this: Ptr<CombatState>, out: Ptr<NiPoint3>) -> bool {
    if target(e, this) == 0 {
        return false;
    }
    let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0x98)]).f64();
    let limit: f64 = e.global(HUNDRED_TWENTY_DOUBLE);
    if elapsed > limit {
        return false;
    }
    copy_words(e, out.addr(), member(this, 0x84), 3);
    true
}

// Translated from 0099abc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::GetLastSeenTargetFiringLocation` (Xbox PDB): the last seen
/// location (`0099ab50`) raised by `CombatUtilities::GetTargetFiringZOffset
/// (target, 2, 1)`.
pub fn combat_state_get_last_seen_target_firing_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    out: Ptr<NiPoint3>,
) -> bool {
    if !fn_0099ab50(e, this, out) {
        return false;
    }
    let target = target(e, this);
    let offset = e.call(0x009a_8460, &args![target, 2u32, 1u32]).f64();
    let z = e.get(out, NiPoint3::z);
    e.set(out, NiPoint3::z, (offset + z as f64) as f32);
    true
}

/// `128.0` (`float`): the dodge distances `0099ac20` passes.
const DODGE_DISTANCE: u32 = 0x0101_e704;
/// `"COMBAT: Invalid combat location type %i passed to
/// CombatState::FindCombatLocation()"`.
const INVALID_LOCATION_TYPE_MESSAGE: u32 = 0x0108_e000;
/// Stack frame of `0099ac20`; a local at `EBP - k` is at
/// `frame + FIND_LOCATION_FRAME - k`.
const FIND_LOCATION_FRAME: u32 = 0x80;

// Translated from 0099ac20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099ac20 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `CombatState::FindCombatLocation` (Xbox PDB): looks for a combat location
/// of `kind` for the planner (`005e3fc0` on the controller). A location the
/// planner already has (`00980700`) is reused (`00979f70`); an unreachable
/// one (`0097bd00` gives `FLT_MAX`) fails at once. Otherwise a new
/// `CombatLocation` (0x20 bytes, `0097ee20`) and `CombatCoverLocation` (0x78
/// bytes, `006d3150`) are made, the current combat location of the same kind
/// is reused, or the kind's search runs (target reference or group location,
/// cover search `0099b8e0`, dodge, location target, flee, grenade arc, line
/// of sight, the flag kinds 14 and 15, combat items 17-21). A found location
/// gets the attacker's space if it has none, is checked for movement
/// (`CombatState::CheckMovement`), costed (`CombatAction::GetMovementCost`
/// when the kind's table entry says so) and handed to the planner
/// (`00995980`, `009959c0`); otherwise both objects are freed and the planner
/// is told `FLT_MAX`.
pub fn combat_state_find_combat_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    kind: u32,
) -> bool {
    let attacker = attacker(e, this);
    let target = target(e, this);
    let controller = state_controller(e, this);
    let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
    let controller = state_controller(e, this);
    let planner = e.call(0x005e_3fc0, &args![controller]).u32();
    if e.call(0x0098_0700, &args![planner, kind]).u32() != 0 {
        e.call(0x0097_9f70, &args![planner, kind]);
        return true;
    }
    let cost = e.call(0x0097_bd00, &args![planner, kind]).f64();
    let unreachable = f32::from_bits(x87_quiet_bits(e.global::<u32>(MAX_FLOAT))) as f64;
    if unreachable == cost {
        return false;
    }
    e.with_stack(FIND_LOCATION_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + FIND_LOCATION_FRAME - k;
        let mut found = false;
        let block = e.call(0x0040_1000, &args![0x20u32]).u32();
        let location = if block == 0 {
            Ptr::<CombatLocation>::NULL
        } else {
            fn_0097ee20(e, Ptr::new(block))
        };
        let block = e.call(0x0040_1000, &args![0x78u32]).u32();
        let cover = if block == 0 {
            0
        } else {
            e.call(0x006d_3150, &args![block]).u32()
        };
        e.call(0x0060_0ab0, &args![location, kind]);
        let radius = x87_quiet_bits(e.mem.u32(LOCATION_TYPE_TABLE + 4 + 8 * kind));
        e.call(0x0081_9a50, &args![location, radius]);

        if e.get(this, CombatState::pCombatLocation) != 0 {
            let current = e.get(this, CombatState::pCombatLocation);
            if e.call(0x0082_5c00, &args![current]).u32() == kind {
                let current = e.get(this, CombatState::pCombatLocation);
                fn_009973a0(e, location, Ptr::new(current));
                let current_cover = e.get(this, CombatState::pCoverLocation);
                if current_cover != 0 {
                    let current_cover = e.get(this, CombatState::pCoverLocation);
                    if e.mem.u32(current_cover + 4) == kind {
                        let current_cover = e.get(this, CombatState::pCoverLocation);
                        fn_00997480(e, Ptr::new(cover), Ptr::new(current_cover));
                    }
                }
                if e.mem.u8(LOCATION_TYPE_TABLE + 8 * kind) == 0 {
                    combat_location_update_ref_location(e, location);
                    found = true;
                }
            }
        }
        let near_flag = kind == 10 || kind == 0xc || kind == 0xf;

        if !found {
            found = find_location_of_kind(
                e,
                this,
                FindContext {
                    kind,
                    location,
                    cover,
                    attacker,
                    target,
                    group,
                    point: local(0x48),
                    world_location: local(0x3c),
                },
            );
        }

        if found {
            if fn_0099b580(e, location) == 0 {
                let space = e.call(GET_SPACE, &args![attacker]).u32();
                fn_0099b520(e, location, space);
            }
            let world = local(0x64);
            let source = e.call(0x0068_15c0, &args![location]).u32();
            copy_words(e, world, source, 4);
            let point = e.call(0x0068_15c0, &args![world]).u32();
            let at_origin = e.call(0x0043_90c0, &args![point, ZERO_POINT]).bool();
            if at_origin
                || e.call(0x009a_02a0, &args![this, world, 1u32, near_flag as u32])
                    .bool()
            {
                let mut movement_cost = 0.0f32;
                if e.mem.u8(LOCATION_TYPE_TABLE + 1 + 8 * kind) != 0 {
                    let mine = local(0x78);
                    e.call(GET_WORLD_LOCATION, &args![attacker, mine]);
                    let controller = state_controller(e, this);
                    movement_cost = e.call(0x0097_acb0, &args![controller, mine, world]).f32();
                }
                let cost_bits = x87_quiet_bits(movement_cost.to_bits());
                e.call(0x0099_5980, &args![planner, kind, location, cost_bits]);
                if e.mem.u32(cover + 0x68) != 0 {
                    e.call(0x0099_59c0, &args![planner, kind, cover]);
                } else if cover != 0 {
                    e.call(0x006d_3370, &args![cover, 1u32]);
                }
                return true;
            }
        }
        e.call(0x0040_1030, &args![location]);
        if cover != 0 {
            e.call(0x006d_3370, &args![cover, 1u32]);
        }
        let unreachable = constant_bits(e, MAX_FLOAT);
        e.call(0x0099_5980, &args![planner, kind, 0u32, unreachable]);
        false
    })
}

/// What the kind switch of `0099ac20` works on: the new location and cover
/// location, the attacker, target and group, and two of its locals (a point
/// at `EBP-0x48`, a `BGSWorldLocation` at `EBP-0x3c`).
struct FindContext {
    kind: u32,
    location: Ptr<CombatLocation>,
    cover: u32,
    attacker: u32,
    target: u32,
    group: u32,
    point: u32,
    world_location: u32,
}

/// The kind switch of `0099ac20` (its jump table at `0099b4dc`, indices at
/// `0099b508`); returns whether a location was found.
fn find_location_of_kind(e: &mut Engine, this: Ptr<CombatState>, context: FindContext) -> bool {
    let FindContext {
        kind,
        location,
        cover,
        attacker,
        target,
        group,
        point,
        world_location,
    } = context;
    let search_info = this.at(CombatState::CoverSearchInfo).addr();
    match kind {
        0 | 1 => {
            if target == 0 {
                return false;
            }
            if e.call(0x0097_ee90, &args![this]).bool() {
                if !e.call(0x0097_fd40, &args![this]).bool() {
                    combat_location_set_ref(e, location, target);
                    return true;
                }
                false
            } else {
                e.call(0x0097_ede0, &args![world_location]);
                if e.call(0x0098_90e0, &args![group, target, world_location])
                    .bool()
                {
                    e.call(0x0097_ee60, &args![location, world_location]);
                    return true;
                }
                false
            }
        }
        2 | 4 | 5 => {
            let controller = state_controller(e, this);
            let weapon = e.call(0x0070_31c0, &args![controller]).u32();
            fn_0099b8e0(
                e,
                this,
                weapon,
                Ptr::new(search_info),
                cover,
                location.addr(),
                kind,
            )
        }
        3 => {
            let distance = constant_bits(e, DODGE_DISTANCE);
            let distance2 = constant_bits(e, DODGE_DISTANCE);
            e.call(
                0x0099_e190,
                &args![this, location, 3u32, 0xfu32, distance2, distance],
            )
            .bool()
        }
        6 => {
            let location_target = e.get(this, CombatState::pLocationTarget);
            if location_target != 0 && e.mem.u8(location_target) == 0 {
                let location_target = e.get(this, CombatState::pLocationTarget);
                let spot = e.call(0x0068_15c0, &args![location_target + 8]).u32();
                e.call(
                    0x0099_d030,
                    &args![
                        this,
                        search_info,
                        cover,
                        location,
                        kind,
                        spot,
                        0.0f32.to_bits(),
                        0u32
                    ],
                )
                .bool()
            } else if self::target(e, this) != 0 {
                let target = self::target(e, this);
                let source = e.vcall(target, 0x1f4, &args![]).u32();
                copy_words(e, point, source, 3);
                e.call(
                    0x0099_d030,
                    &args![
                        this,
                        search_info,
                        cover,
                        location,
                        kind,
                        point,
                        0.0f32.to_bits(),
                        0u32
                    ],
                )
                .bool()
            } else {
                false
            }
        }
        7..=9 | 0x10 => {
            if e.get(this, CombatState::pLocationTarget) == 0 {
                return false;
            }
            if kind == 8
                && e.call(0x0046_1580, &args![this, 1u32.wrapping_shl(kind)])
                    .bool()
            {
                return false;
            }
            let location_target = e.get(this, CombatState::pLocationTarget);
            if e.mem.u32(location_target + 4) != 0 {
                let location_target = e.get(this, CombatState::pLocationTarget);
                let reference = e.mem.u32(location_target + 4);
                combat_location_set_ref(e, location, reference);
                let location_target = e.get(this, CombatState::pLocationTarget);
                let object = e.mem.u32(location_target + 4);
                if e.vcall(object, 0x100, &args![]).bool() {
                    let location_target = e.get(this, CombatState::pLocationTarget);
                    let reference = e.mem.u32(location_target + 4);
                    let theirs = e.call(0x008b_e1b0, &args![reference]).f64();
                    let mine = e.call(0x008b_e1b0, &args![attacker]).f64();
                    let sum = (mine + theirs) as f32;
                    e.call(0x0081_9a50, &args![location, x87_quiet_bits(sum.to_bits())]);
                }
            } else {
                let location_target = e.get(this, CombatState::pLocationTarget);
                e.call(0x0097_ee60, &args![location, location_target + 8]);
            }
            let location_target = e.get(this, CombatState::pLocationTarget);
            let radius = x87_quiet_bits(e.mem.u32(location_target + 0x18));
            e.call(0x0081_9a50, &args![location, radius]);
            true
        }
        10 | 0xc => e
            .call(0x0099_ebe0, &args![this, cover, location, kind])
            .bool(),
        0xb => {
            let weapon = e.mem.u32(member(this, 0x18));
            fn_0099b5a0(e, this, location, member(this, 0x76), weapon)
        }
        0xd => e.call(0x0099_f480, &args![this, location, kind]).bool(),
        0xe => {
            fn_0099b540(e, location, 1, 1);
            true
        }
        0xf => {
            let spot = e.call(0x0068_15c0, &args![location]).u32();
            let controller = state_controller(e, this);
            let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
            let search = e
                .call(0x0098_bc00, &args![group, attacker, 0x32u32, spot])
                .u32();
            let mut found = false;
            if search != 0 {
                e.call(0x0097_ee60, &args![location, search]);
                found = true;
            } else {
                let own = e.call(0x0097_b940, &args![location]).u32();
                if e.call(0x0043_9090, &args![own, ZERO_POINT]).bool() {
                    found = true;
                }
            }
            if found {
                fn_0099b540(e, location, 2, 1);
            }
            found
        }
        0x11..=0x15 => {
            let item = e.call(0x0097_cfc0, &args![this, kind - 0x11]).u32();
            if item == 0 || e.mem.u32(item) == 0 {
                return false;
            }
            let reference = e.mem.u32(item);
            combat_location_set_ref(e, location, reference);
            let reference = e.mem.u32(item);
            let size = e.call(0x0057_1600, &args![reference]).f64();
            let reach = e.call(0x008b_e420, &args![attacker]).f64();
            let sum = reach + size;
            let extra = setting_value(e, 0x011c_f914);
            let radius = (extra as f64 + sum) as f32;
            e.call(
                0x0081_9a50,
                &args![location, x87_quiet_bits(radius.to_bits())],
            );
            fn_0099b540(e, location, 2, 1);
            true
        }
        _ => {
            e.call(0x005b_5e40, &args![INVALID_LOCATION_TYPE_MESSAGE, kind]);
            false
        }
    }
}

// Translated from 0099b520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the location's space through `0041fd00` (the map's name
/// `NonActorMagicCaster::SetCurrentSpell` is folded code).
pub fn fn_0099b520(e: &mut Engine, this: Ptr<CombatLocation>, space: u32) {
    e.call(0x0041_fd00, &args![this, space]);
}

// Translated from 0099b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears the `flags` bits of `CombatLocation::iFlags`.
pub fn fn_0099b540(e: &mut Engine, this: Ptr<CombatLocation>, flags: u32, set: u8) {
    let current = e.get(this, CombatLocation::iFlags);
    if set != 0 {
        e.set(this, CombatLocation::iFlags, current | flags);
    } else {
        e.set(this, CombatLocation::iFlags, !flags & current);
    }
}

// Translated from 0099b580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The location's space (`0084e3a0`: the word at +0xc).
pub fn fn_0099b580(e: &mut Engine, this: Ptr<CombatLocation>) -> u32 {
    e.call(0x0084_e3a0, &args![this]).u32()
}

/// Stack frame of `0099b5a0`; a local at `EBP - k` is at
/// `frame + GRENADE_FRAME - k`.
const GRENADE_FRAME: u32 = 0x68;

// Translated from 0099b5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks for a spot to throw `weapon`'s projectile at: with a projectile
/// (`00525a90` on the weapon) and a target, tries first the target's group
/// location in the attacker's space, then the last seen location (seen
/// within 10 s); a spot qualifies when it is within the projectile's range
/// (`009a7f00`) but not closer than 256 units (`65536` squared), the combat
/// manager accepts the projectile there (`00992720`), the arc reaches it
/// (`CombatUtilities::CheckProjectileArc`, arc flag in `high_arc`) and, for a
/// projectile with an explosion, the manager also accepts it at the flight
/// time (`CombatUtilities::ComputeProjectileTime`). The spot becomes `out`
/// (`0099b8b0`).
pub fn fn_0099b5a0(
    e: &mut Engine,
    this: Ptr<CombatState>,
    out: Ptr<CombatLocation>,
    high_arc: u32,
    weapon: u32,
) -> bool {
    if weapon == 0 {
        return false;
    }
    let attacker = attacker(e, this);
    if e.call(0x0052_5a90, &args![weapon, attacker]).u32() == 0 {
        return false;
    }
    let attacker = self::attacker(e, this);
    let target = target(e, this);
    if target == 0 {
        return false;
    }
    e.with_stack(GRENADE_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + GRENADE_FRAME - k;
        let projectile = e.call(0x0052_5a90, &args![weapon, attacker]).u32();
        let range = e
            .call(0x009a_7f00, &args![projectile, 0.0f32.to_bits()])
            .f32();
        let origin = local(0x18);
        e.call(0x009a_8050, &args![origin, attacker]);
        let spot = local(0x48);
        for attempt in 0u32..2 {
            e.call(0x0068_15c0, &args![spot]);
            if attempt == 0 {
                let world = local(0x5c);
                e.call(0x0097_ede0, &args![world]);
                let controller = state_controller(e, this);
                let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
                if !e.call(0x0098_90e0, &args![group, target, world]).bool() {
                    continue;
                }
                let space = e.call(0x0084_e3a0, &args![world]).u32();
                if space != e.call(GET_SPACE, &args![attacker]).u32() {
                    continue;
                }
                let source = e.call(0x0068_15c0, &args![world]).u32();
                copy_words(e, spot, source, 3);
            } else {
                let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0x98)]).f64();
                let limit: f64 = e.global(TEN_DOUBLE);
                if elapsed > limit {
                    continue;
                }
                copy_words(e, spot, member(this, 0x84), 3);
            }
            let offset = e.call(0x009a_8460, &args![target, 2u32, 0u32]).f64();
            let z = e.mem.f32(spot + 8);
            e.mem.set_f32(spot + 8, (offset + z as f64) as f32);
            let difference = local(0x2c);
            e.call(0x0043_9ef0, &args![spot, difference, origin]);
            let distance = e.call(0x0059_5c80, &args![difference]).f32();
            if distance as f64 > range as f64 * range as f64 {
                continue;
            }
            let closest: f64 = e.global(SIXTY_FIVE_THOUSAND_DOUBLE);
            if (distance as f64) < closest {
                continue;
            }
            let explosion = e.call(0x004f_d3c0, &args![projectile]).u32();
            let manager: u32 = e.global(COMBAT_MANAGER);
            if explosion != 0
                && !e
                    .call(
                        0x0099_2720,
                        &args![manager, attacker, explosion, spot, 0.0f32.to_bits()],
                    )
                    .bool()
            {
                continue;
            }
            let landing = local(0x3c);
            e.call(0x0068_15c0, &args![landing]);
            let minus_one = constant_bits(e, MINUS_ONE);
            let (ox, oy, oz) = (
                e.mem.u32(origin),
                e.mem.u32(origin + 4),
                e.mem.u32(origin + 8),
            );
            let (sx, sy, sz) = (e.mem.u32(spot), e.mem.u32(spot + 4), e.mem.u32(spot + 8));
            let (zx, zy, zz) = (
                e.global::<u32>(ZERO_POINT),
                e.global::<u32>(ZERO_POINT + 4),
                e.global::<u32>(ZERO_POINT + 8),
            );
            let reaches = e
                .call(
                    0x009a_7670,
                    &args![
                        attacker, target, projectile, ox, oy, oz, sx, sy, sz, zx, zy, zz, high_arc,
                        landing, minus_one
                    ],
                )
                .bool();
            if !reaches {
                continue;
            }
            if explosion != 0 {
                let arc = e.mem.u8(high_arc) as u32;
                let (ox, oy, oz) = (
                    e.mem.u32(origin),
                    e.mem.u32(origin + 4),
                    e.mem.u32(origin + 8),
                );
                let (sx, sy, sz) = (e.mem.u32(spot), e.mem.u32(spot + 4), e.mem.u32(spot + 8));
                let time = e
                    .call(
                        0x009a_7e00,
                        &args![projectile, ox, oy, oz, sx, sy, sz, arc, 0.0f32.to_bits()],
                    )
                    .f32();
                let manager: u32 = e.global(COMBAT_MANAGER);
                if !e
                    .call(
                        0x0099_2720,
                        &args![
                            manager,
                            attacker,
                            explosion,
                            spot,
                            x87_quiet_bits(time.to_bits())
                        ],
                    )
                    .bool()
                {
                    continue;
                }
            }
            fn_0099b8b0(e, out, spot, 0);
            return true;
        }
        false
    })
}

// Translated from 0099b8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the location's point and space (`BGSWorldLocation::SetLocation`,
/// `00989010`) and drops its reference.
pub fn fn_0099b8b0(e: &mut Engine, this: Ptr<CombatLocation>, point: u32, space: u32) {
    e.call(0x0098_9010, &args![this, point, space]);
    e.set(this, CombatLocation::pRef, 0);
}

/// `512.0` (`float`).
const FIVE_HUNDRED_TWELVE: u32 = 0x0102_31a0;
/// `2048.0` (`float`).
const TWO_THOUSAND_FORTY_EIGHT: u32 = 0x0101_8bfc;
/// `10000.0` (`float`).
const TEN_THOUSAND: u32 = 0x0102_2958;
/// `4096.0` (`float`).
const FOUR_THOUSAND_NINETY_SIX: u32 = 0x0101_7a3c;
/// `2.0` (`double`).
const TWO_DOUBLE: u32 = 0x0101_1590;
/// `1000.0` (`double`).
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
/// `3.0` (`double`).
const THREE_DOUBLE: u32 = 0x0102_1928;
/// `-1.0` (`double`).
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// `256.0` (`float`).
const TWO_HUNDRED_FIFTY_SIX: u32 = 0x0103_2adc;
/// The object `00483710` is called on around the cover search (a byte at
/// `011f1a61`).
const COVER_SEARCH_TOGGLE: u32 = 0x011f_1a61;
/// The object at `011f6394` whose +0x14 word (`00825c00`) the cover search
/// compares cover time stamps with, in milliseconds.
const TIMER_OBJECT_011F6394: u32 = 0x011f_6394;
/// Stack frame of `0099b8e0`; a local at `EBP - k` is at
/// `frame + COVER_SEARCH_FRAME - k`.
const COVER_SEARCH_FRAME: u32 = 0x1e0;

// Translated from 0099b8e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099b8e0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Searches cover for location `kind` (2, 4, 5, 10 or 12) against the
/// target (inside `00483710` on `011f1a61`): the weapon's optimal ranges
/// (`CombatUtilities::ComputeOptimalWeaponRanges`, scaled by the setting at
/// `011ce3f0` and limited by `0040ebd0`) and the kind's parameters give the
/// search; the current cover location is kept when it still passes
/// `0099c230`; otherwise a `PathingRequestCover` (built from the attacker's
/// or the location target's position, the search centre and the kind's
/// flags) fills `search_info`'s cover locations (`006d62e0`) and the first one
/// that passes `0099c230` (skipping ones used in the last 3 s) becomes the
/// cover location. Returns whether a location was found.
pub fn fn_0099b8e0(
    e: &mut Engine,
    this: Ptr<CombatState>,
    weapon: u32,
    search_info: Ptr<CombatCoverSearchInfo>,
    cover: u32,
    location: u32,
    kind: u32,
) -> bool {
    e.call(0x0048_3710, &args![COVER_SEARCH_TOGGLE]);
    let search_radius = x87_quiet(e.global::<f32>(FIVE_HUNDRED_TWELVE));
    let mut distance = x87_quiet(e.global::<f32>(TWO_THOUSAND_FORTY_EIGHT));
    let attacker = attacker(e, this);
    let target = target(e, this);
    if target == 0 {
        return false;
    }
    e.with_stack(COVER_SEARCH_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + COVER_SEARCH_FRAME - k;
        let (minimum, range, maximum) = (local(0x30), local(0x3c), local(0x70));
        e.mem.set_f32(minimum, 0.0);
        let mut half_base = x87_quiet(e.global::<f32>(FIVE_HUNDRED_TWELVE));
        let value = x87_quiet(e.global::<f32>(TWO_THOUSAND_FORTY_EIGHT));
        e.mem.set_f32(range, value);
        let value = x87_quiet(e.global::<f32>(TWO_THOUSAND_FORTY_EIGHT));
        e.mem.set_f32(maximum, value);
        let controller = state_controller(e, this);
        e.call(
            0x009a_9180,
            &args![controller, weapon, minimum, range, maximum],
        );
        let scale = setting_value_address(e, 0x011c_e3f0);
        let scaled = (e.mem.f32(range) as f64 * e.mem.f32(scale) as f64) as f32;
        e.mem.set_f32(range, scaled);
        let upper = x87_quiet_bits(e.mem.u32(maximum));
        let lower = x87_quiet_bits(e.mem.u32(range));
        let limited = e.call(0x0040_ebd0, &args![lower, upper]).f32();
        e.mem.set_f32(range, limited);
        let mut flag_006e4380 = 0u8;
        let mut flag_006e43a0 = 0u8;
        let mut flag_006e43c0 = 0u8;
        let mut flag_006e43e0 = 0u8;
        match kind {
            2 => {
                let controller = state_controller(e, this);
                let plan = e.call(0x0098_0070, &args![controller]).u32();
                distance = e.call(0x004a_7bd0, &args![plan]).f32();
            }
            4 => {
                flag_006e43e0 = 1;
                flag_006e4380 = 1;
            }
            5 => {
                distance = x87_quiet(e.global::<f32>(FOUR_THOUSAND_NINETY_SIX));
                let value = x87_quiet(e.global::<f32>(TEN_THOUSAND));
                e.mem.set_f32(range, value);
                flag_006e43e0 = 1;
                flag_006e43a0 = 1;
                flag_006e43c0 = 1;
            }
            10 => {
                let value = x87_quiet(e.global::<f32>(TEN_THOUSAND));
                e.mem.set_f32(range, value);
                flag_006e43e0 = 1;
            }
            12 => {
                half_base = x87_quiet(setting_value(e, 0x011c_ed34));
                distance = x87_quiet(setting_value(e, 0x011c_f8d4));
                let value = x87_quiet(e.global::<f32>(TEN_THOUSAND));
                e.mem.set_f32(range, value);
                flag_006e43a0 = 1;
                flag_006e43c0 = 1;
                flag_006e43e0 = 1;
            }
            _ => {}
        }

        let state = local(0x168);
        e.mem.set_u32(state, 0);
        let found_distance = local(0x38);
        e.mem.set_f32(found_distance, 0.0);
        if e.mem.u32(cover + 0x68) != 0 {
            let key = e.mem.u32(cover + 0x68);
            e.set(
                search_info,
                CombatCoverSearchInfo::iPreviousCoverLocationKey,
                key,
            );
            e.set(search_info, CombatCoverSearchInfo::ePreviousCoverState, 0);
            if kind == e.mem.u32(cover + 4) {
                let two: f64 = e.global(TWO_DOUBLE);
                let half = (half_base as f64 / two) as f32;
                let key = e.mem.u32(cover + 0x68);
                let keeps = fn_0099c230(
                    e,
                    this,
                    CoverCheck {
                        cover_location: cover + 0x14,
                        out: location,
                        reservation: cover + 0x70,
                        search_point: cover + 8,
                        key,
                        current_key: key,
                        kind,
                        attacker,
                        target,
                        minimum_distance: x87_quiet(half),
                        maximum_distance: x87_quiet(e.mem.f32(range)),
                        state,
                        distance: found_distance,
                        flag_006e43e0,
                    },
                );
                if keeps {
                    e.call(0x0054_07b0, &args![cover, 0u32]);
                    e.call(0x0048_3710, &args![COVER_SEARCH_TOGGLE]);
                    return true;
                }
                let previous = e.mem.u32(state);
                e.set(
                    search_info,
                    CombatCoverSearchInfo::ePreviousCoverState,
                    previous,
                );
            } else {
                e.set(search_info, CombatCoverSearchInfo::ePreviousCoverState, 0xf);
            }
        } else {
            e.set(
                search_info,
                CombatCoverSearchInfo::iPreviousCoverLocationKey,
                0,
            );
        }

        let origin = local(0x18);
        e.call(0x0068_15c0, &args![origin]);
        let location_target = e.get(this, CombatState::pLocationTarget);
        let near_location_target = location_target != 0 && {
            let target_kind = e.mem.u8(e.get(this, CombatState::pLocationTarget));
            target_kind == 1 || e.mem.u8(e.get(this, CombatState::pLocationTarget)) == 2
        };
        if near_location_target {
            let location_target = e.get(this, CombatState::pLocationTarget);
            let source = e.call(0x0068_15c0, &args![location_target + 8]).u32();
            copy_words(e, origin, source, 3);
            distance = e.call(0x009a_0b00, &args![this]).f32();
        } else {
            let source = e.vcall(attacker, 0x1f4, &args![]).u32();
            copy_words(e, origin, source, 3);
        }

        let request = local(0x164);
        e.call(0x006e_4080, &args![request]);
        let pathing_location = local(0x6c);
        e.call(0x006d_ce10, &args![pathing_location, origin, attacker]);
        e.call(0x006d_3ac0, &args![request, pathing_location]);
        let info = search_info.addr();
        if kind == 5 {
            let (group_point, member_point) = (local(0x198), local(0x17c));
            e.call(0x0068_15c0, &args![group_point]);
            e.call(0x0068_15c0, &args![member_point]);
            let controller = state_controller(e, this);
            let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
            e.call(0x0098_9eb0, &args![group, group_point]);
            let controller = state_controller(e, this);
            let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
            e.call(0x0098_9e20, &args![group, member_point]);
            copy_words(e, info, group_point, 3);
            let direction = local(0x18c);
            e.call(0x0043_9ef0, &args![member_point, direction, group_point]);
            e.call(0x006e_4400, &args![request, direction]);
        } else {
            let elapsed = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0x98)]).f64();
            let limit = setting_value(e, 0x011c_f6cc);
            let seen = limit as f64 > elapsed;
            e.set(
                search_info,
                CombatCoverSearchInfo::bUsedLastSeenLocation,
                seen as u8,
            );
            if e.get(search_info, CombatCoverSearchInfo::bUsedLastSeenLocation) != 0 {
                copy_words(e, info, member(this, 0x84), 3);
            } else {
                let source = e.vcall(target, 0x1f4, &args![]).u32();
                copy_words(e, info, source, 3);
            }
        }
        e.call(0x006d_ad40, &args![request, info]);
        e.call(
            0x006e_4340,
            &args![request, x87_quiet_bits(search_radius.to_bits())],
        );
        e.call(
            0x006e_4320,
            &args![request, x87_quiet_bits(distance.to_bits())],
        );
        e.call(0x0040_30d0, &args![request, 200u32]);
        e.call(0x006e_4380, &args![request, flag_006e4380 as u32]);
        e.call(0x006e_43a0, &args![request, flag_006e43a0 as u32]);
        e.call(0x006e_43c0, &args![request, flag_006e43c0 as u32]);
        let controller = state_controller(e, this);
        let height = e.call(0x0045_36e0, &args![controller]).f32();
        e.call(0x0051_b840, &args![request, height.to_bits()]);
        let controller = state_controller(e, this);
        let sneak_height = e.call(0x0045_3700, &args![controller]).f32();
        e.call(0x0050_76c0, &args![request, sneak_height.to_bits()]);
        let controller = state_controller(e, this);
        let can_sneak = fn_00998a30(e, controller);
        e.call(0x008e_eaa0, &args![request, can_sneak as u32]);
        e.call(0x006e_43e0, &args![request, flag_006e43e0 as u32]);
        e.mem.set_u32(cover + 4, kind);
        copy_words(e, cover + 8, info, 3);
        let now = game_time_bits(e);
        let stamp = time_stamp_word(e, now);
        e.set(search_info, CombatCoverSearchInfo::fTimeStamp, stamp);
        copy_words(e, info + 0xc, origin, 3);
        e.set(
            search_info,
            CombatCoverSearchInfo::fRadius,
            x87_quiet(distance),
        );
        let locations = info + 0x20;
        e.call(0x006d_bde0, &args![locations, 0u32]);
        if e.call(0x006d_62e0, &args![request, locations]).u32() != 0 {
            let manager: u32 = e.global(COMBAT_MANAGER);
            let cover_map = e.call(0x0096_11e0, &args![manager]).u32();
            fn_0099c210(e, cover_map);
            let mut index = 0u32;
            while index < e.call(ARRAY_SIZE, &args![locations]).u32() {
                let entry = e.call(0x0090_52c0, &args![locations, index]).u32();
                let entry_state = local(0x1a4);
                e.mem.set_u32(entry_state, 0);
                let entry_distance = local(0x1a8);
                e.mem.set_f32(entry_distance, 0.0);
                let key = e.call(0x006e_4570, &args![entry]).u32();
                if key == e.mem.u32(cover + 0x68) {
                    let previous = e.mem.u8(state);
                    e.mem.set_u8(entry + 0x52, previous);
                    index += 1;
                    continue;
                }
                let moved = e.call(0x006e_4740, &args![entry]).u32();
                if moved != 0 {
                    let now = e.call(0x0082_5c00, &args![TIMER_OBJECT_011F6394]).u32();
                    let seconds = now.wrapping_sub(moved) as f64 / e.global::<f64>(THOUSAND_DOUBLE);
                    if seconds < e.global::<f64>(THREE_DOUBLE) {
                        e.mem.set_u8(entry + 0x52, 1);
                        index += 1;
                        continue;
                    }
                }
                let reservation = local(0x1b4);
                e.call(0x006d_3280, &args![reservation]);
                let passes = fn_0099c230(
                    e,
                    this,
                    CoverCheck {
                        cover_location: entry,
                        out: location,
                        reservation,
                        search_point: cover + 8,
                        key,
                        current_key: 0,
                        kind,
                        attacker,
                        target,
                        minimum_distance: x87_quiet(half_base),
                        maximum_distance: x87_quiet(e.mem.f32(range)),
                        state: entry_state,
                        distance: entry_distance,
                        flag_006e43e0,
                    },
                );
                if passes {
                    e.mem.set_u8(entry + 0x52, 0x13);
                    e.call(0x006e_4430, &args![cover + 0x14, entry]);
                    e.mem.set_u32(cover + 0x68, key);
                    e.mem.set_u32(cover + 0x6c, moved);
                    copy_words(e, cover + 0x70, reservation, 1);
                    let half = e.mem.u16(reservation + 4);
                    e.mem.set_u16(cover + 0x74, half);
                    let height = fn_0099c1f0(e, cover + 0x14);
                    e.mem.set_u8(cover + 2, height);
                    e.call(0x0054_07b0, &args![cover, 0u32]);
                    timer_stop(e, member(this, 0x1fc));
                    e.call(0x0048_3710, &args![COVER_SEARCH_TOGGLE]);
                    e.call(0x004f_f7e0, &args![pathing_location]);
                    e.call(0x006d_ad70, &args![request]);
                    return true;
                }
                let failed = e.mem.u8(entry_state);
                e.mem.set_u8(entry + 0x52, failed);
                index += 1;
            }
            let manager: u32 = e.global(COMBAT_MANAGER);
            let cover_map = e.call(0x0096_11e0, &args![manager]).u32();
            e.call(0x0099_5890, &args![cover_map]);
        }
        e.call(0x0048_3710, &args![COVER_SEARCH_TOGGLE]);
        e.call(0x004f_f7e0, &args![pathing_location]);
        e.call(0x006d_ad70, &args![request]);
        false
    })
}

// Translated from 0099c1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingCoverLocation::cCoverHeight` (+0x4c).
pub fn fn_0099c1f0(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this.wrapping_add(0x4c))
}

// Translated from 0099c210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSReadWriteLock::LockForWrite` on the lock at +0x14.
pub fn fn_0099c210(e: &mut Engine, this: u32) {
    e.call(0x00aa_6060, &args![this.wrapping_add(0x14)]);
}

/// The fourteen argument words of `0099c230`, in order.
pub struct CoverCheck {
    /// The `PathingCoverLocation` to check.
    pub cover_location: u32,
    /// The `CombatLocation` set on success (`0099b8b0`).
    pub out: u32,
    /// The `CoverReservationInfo` the cover map fills (`009835d0`).
    pub reservation: u32,
    /// The point the cover is searched from.
    pub search_point: u32,
    /// The cover location's key.
    pub key: u32,
    /// The current cover location's key (equal to `key` when re-checking
    /// it, 0 otherwise).
    pub current_key: u32,
    /// The combat location kind.
    pub kind: u32,
    pub attacker: u32,
    pub target: u32,
    pub minimum_distance: f32,
    pub maximum_distance: f32,
    /// Out: the failure reason.
    pub state: u32,
    /// Out: the squared distance from the search point.
    pub distance: u32,
    /// The flag the search also passes to `006e43e0`.
    pub flag_006e43e0: u8,
}

/// Stack frame of `0099c230`; a local at `EBP - k` is at
/// `frame + COVER_CHECK_FRAME - k`.
const COVER_CHECK_FRAME: u32 = 0x110;

// Translated from 0099c230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099c230 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Checks one cover location (one or two passes, one per open edge, for an
/// edge cover outside a long-range search): failure reasons go to `state`
/// (2 closed edges, 13 blocked from the target's position, 3 a cover the
/// state rejects `009a0d70`, 9 reserved `009835d0`, 6 ledge angle, 4/5
/// invalid locations, 16 movement `009a0260`, 7 too far, 8 too close, 10 a
/// target near it, 11/12 threatened `009a5670`, 13 blocked `0099cc60` /
/// `0099ca00`, 14 no line of sight); the highest reason of the last two
/// passes is kept. On success the location is stored into `out`
/// (`0099b8b0`).
pub fn fn_0099c230(e: &mut Engine, this: Ptr<CombatState>, check: CoverCheck) -> bool {
    let CoverCheck {
        cover_location,
        out,
        reservation,
        search_point,
        key,
        current_key,
        kind,
        attacker: mover,
        target,
        minimum_distance,
        maximum_distance,
        state,
        distance,
        flag_006e43e0,
    } = check;
    let attacker = attacker(e, this);
    let near_radius = x87_quiet(e.global::<f32>(TWO_HUNDRED_FIFTY_SIX));
    e.with_stack(COVER_CHECK_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + COVER_CHECK_FRAME - k;
        if e.call(0x0098_2be0, &args![cover_location]).bool()
            && flag_006e43e0 == 0
            && e.mem.u8(cover_location + 0x4f) == 0
            && e.mem.u8(cover_location + 0x50) == 0
        {
            e.mem.set_u32(state, 2);
            return false;
        }
        if key == current_key {
            let target_location = e.vcall(target, 0x1f4, &args![]).u32();
            let point = e
                .call(0x005c_3420, &args![cover_location, local(0xd8)])
                .u32();
            if !e
                .call(
                    0x0069_6540,
                    &args![
                        point,
                        target_location,
                        cover_location + 0x28,
                        cover_location + 0x34
                    ],
                )
                .bool()
            {
                e.mem.set_u32(state, 0xd);
                return false;
            }
        }
        if e.call(0x009a_0d70, &args![this, key]).bool() {
            e.mem.set_u32(state, 3);
            return false;
        }
        let mut reason = 0u32;
        let mut previous = 0u32;
        e.mem.set_f32(distance, 0.0);
        let mut travel_time = x87_quiet(e.global::<f32>(MINUS_ONE));
        let radius = e.call(0x008b_e1b0, &args![attacker]).f32();
        let mut side = 1u32;
        if e.mem.u8(reservation + 1) != 0 {
            if e.mem.u8(reservation + 5) != 0 {
                side = 0;
            }
        } else {
            let mover_location = e.vcall(mover, 0x1f4, &args![]).u32();
            if e.call(0x006e_4840, &args![cover_location, mover_location])
                .bool()
            {
                side = 0;
            }
        }
        let both_edges = e.call(0x0098_2be0, &args![cover_location]).bool() && flag_006e43e0 == 0;
        let lean = (e.call(0x0098_2be0, &args![cover_location]).bool()
            || e.call(0x006e_4820, &args![cover_location]).bool())
            && flag_006e43e0 == 0;
        let passes = both_edges as u32 + 1;
        let (point, lean_point) = (local(0x64), local(0x48));
        let (location, lean_location) = (local(0x90), local(0xbc));
        for pass in 0..passes {
            if reason != 0 {
                previous = reason;
            }
            let mut edge = 0u8;
            if both_edges {
                if pass == side && e.mem.u8(cover_location + 0x4f) != 0 {
                    edge = 1;
                } else if pass != side && e.mem.u8(cover_location + 0x50) != 0 {
                    edge = 0;
                } else {
                    continue;
                }
            }
            let manager: u32 = e.global(COMBAT_MANAGER);
            let cover_map = e.call(0x0096_11e0, &args![manager]).u32();
            let controller = state_controller(e, this);
            if !e
                .call(
                    0x0098_35d0,
                    &args![
                        cover_map,
                        key,
                        cover_location,
                        controller,
                        x87_quiet_bits(radius.to_bits()),
                        reservation,
                        flag_006e43e0 as u32,
                        edge as u32
                    ],
                )
                .bool()
            {
                reason = 9;
                continue;
            }
            e.call(0x0068_15c0, &args![point]);
            e.call(0x0068_15c0, &args![lean_point]);
            let controller = state_controller(e, this);
            e.call(
                0x009a_93a0,
                &args![controller, cover_location, reservation, point, lean_point],
            );
            if e.call(0x006e_4820, &args![cover_location]).bool()
                && target != 0
                && !e
                    .call(
                        0x0099_cec0,
                        &args![this, cover_location, point, search_point, target],
                    )
                    .bool()
            {
                reason = 6;
                continue;
            }
            e.call(0x006d_d0d0, &args![location, point, cover_location]);
            let radius = e.call(0x008b_e1b0, &args![attacker]).f32();
            if !e
                .call(0x006d_ed70, &args![location, radius.to_bits()])
                .bool()
            {
                reason = 4;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            if lean {
                e.call(
                    0x006d_d0d0,
                    &args![lean_location, lean_point, cover_location],
                );
                let radius = e.call(0x008b_e1b0, &args![attacker]).f32();
                if !e
                    .call(0x006d_ed70, &args![lean_location, radius.to_bits()])
                    .bool()
                {
                    reason = 5;
                    e.call(0x004f_f7e0, &args![lean_location]);
                    e.call(0x004f_f7e0, &args![location]);
                    continue;
                }
                e.call(0x004f_f7e0, &args![lean_location]);
            }
            let space = e.call(GET_SPACE, &args![mover]).u32();
            if !e
                .call(0x009a_0260, &args![this, point, space, 1u32, 0u32])
                .bool()
            {
                reason = 0x10;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            let difference = e
                .call(0x0043_9ef0, &args![search_point, local(0xe4), point])
                .u32();
            let squared = e.call(0x004a_7290, &args![difference]).f32();
            e.mem.set_f32(distance, squared);
            let squared = e.mem.f32(distance) as f64;
            if squared > maximum_distance as f64 * maximum_distance as f64 {
                reason = 7;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            if squared < minimum_distance as f64 * minimum_distance as f64 {
                reason = 8;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            let world = local(0x58);
            e.call(0x0097_ede0, &args![world]);
            let space = e.call(GET_SPACE, &args![mover]).u32();
            e.call(0x0098_9010, &args![world, point, space]);
            let controller = state_controller(e, this);
            let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
            if e.call(
                0x0098_9700,
                &args![group, world, x87_quiet_bits(near_radius.to_bits()), 0u32],
            )
            .u32()
                != 0
            {
                reason = 0xa;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            if travel_time as f64 == e.global::<f64>(MINUS_ONE_DOUBLE) {
                let from = e.vcall(mover, 0x1f4, &args![]).u32();
                let offset = local(0xc8);
                e.call(0x0043_9ef0, &args![from, offset, point]);
                let length = e.call(0x0045_7990, &args![offset]).f64();
                let speed = e.call(0x008a_0b10, &args![mover]).f64();
                travel_time = (length / speed) as f32;
            }
            if threat_at(e, point, mover, travel_time) {
                reason = 0xb;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            if lean && threat_at(e, lean_point, mover, travel_time) {
                reason = 0xc;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            let mut blocked = e
                .call(0x0099_cc60, &args![location, search_point, 1u32])
                .bool();
            if !blocked {
                blocked = fn_0099ca00(e, this, location, target);
            }
            if blocked {
                reason = 0xd;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            let check_sight = !(kind == 5 || flag_006e43e0 != 0 || key == current_key);
            if check_sight
                && !e
                    .call(
                        0x009a_7940,
                        &args![
                            mover,
                            target,
                            lean_point,
                            search_point,
                            1u32,
                            0xffff_ffffu32
                        ],
                    )
                    .bool()
            {
                reason = 0xe;
                e.call(0x004f_f7e0, &args![location]);
                continue;
            }
            fn_0099b8b0(e, Ptr::new(out), point, 0);
            e.call(0x004f_f7e0, &args![location]);
            return true;
        }
        if (previous as i32) > (reason as i32) {
            e.mem.set_u32(state, previous);
        } else {
            e.mem.set_u32(state, reason);
        }
        false
    })
}

/// The threat test of `0099c230`: the threat map (`00825c00` on the combat
/// manager) value at `point` for `mover` arriving after `travel_time`
/// (`009a5670`, with a `CombatTimeStamp` of 0 built on the argument stack by
/// `00435de0`), true when it is above zero.
fn threat_at(e: &mut Engine, point: u32, mover: u32, travel_time: f32) -> bool {
    let stamp = time_stamp_word(e, 0.0f32.to_bits());
    let (x, y, z) = (e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8));
    let manager: u32 = e.global(COMBAT_MANAGER);
    let threat_map = e.call(0x0082_5c00, &args![manager]).u32();
    let threat = e
        .call(
            0x009a_5670,
            &args![
                threat_map,
                x,
                y,
                z,
                mover,
                x87_quiet_bits(travel_time.to_bits()),
                0.0f32.to_bits(),
                stamp,
                0.0f32.to_bits()
            ],
        )
        .f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    threat > zero
}

/// Stack frame of `0099ca00`; a local at `EBP - k` is at
/// `frame + BLOCKING_FRAME - k`.
const BLOCKING_FRAME: u32 = 0x30;

// Translated from 0099ca00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether another member of the attacker's group (`00968630` over
/// `005a4320` members, `ignored` excepted) blocks `location`: each member's
/// position (its group location from `00989140` within 3 units, or its
/// current one) is tested with `0099cc60`.
pub fn fn_0099ca00(e: &mut Engine, this: Ptr<CombatState>, location: u32, ignored: u32) -> bool {
    let controller = state_controller(e, this);
    let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
    let count = e.call(0x005a_4320, &args![group]).u32();
    e.with_stack(BLOCKING_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + BLOCKING_FRAME - k;
        let mut index = 0u32;
        while index < count {
            let ally = e.call(0x0096_8630, &args![group, index]).u32();
            if ally == ignored {
                index += 1;
                continue;
            }
            let world = local(0x1c);
            e.call(0x0097_ede0, &args![world]);
            let position = local(0x28);
            e.call(0x0068_15c0, &args![position]);
            let three = constant_bits(e, THREE);
            if e.call(0x0098_9140, &args![group, index, world, three])
                .bool()
            {
                let source = e.call(0x0068_15c0, &args![world]).u32();
                copy_words(e, position, source, 3);
            } else {
                let source = e.vcall(ally, 0x1f4, &args![]).u32();
                copy_words(e, position, source, 3);
            }
            if e.call(0x0099_cc60, &args![location, position, 0u32]).bool() {
                return true;
            }
            index += 1;
        }
        false
    })
}

// Translated from 0099cb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any point of the array `points` (`006a7af0`, three words each)
/// blocks `location` (`0099cc60`). cdecl.
pub fn fn_0099cb00(e: &mut Engine, location: u32, points: u32) -> bool {
    e.with_stack(12, |e, point| {
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![points]).u32() {
            let source = e.call(0x006a_7af0, &args![points, index]).u32();
            copy_words(e, point.addr(), source, 3);
            if e.call(0x0099_cc60, &args![location, point, 0u32]).bool() {
                return true;
            }
            index += 1;
        }
        false
    })
}

/// The uniform form of `0099c230` (fourteen argument words after `this`).
fn fn_0099c230_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let check = CoverCheck {
        cover_location: words[1],
        out: words[2],
        reservation: words[3],
        search_point: words[4],
        key: words[5],
        current_key: words[6],
        kind: words[7],
        attacker: words[8],
        target: words[9],
        minimum_distance: f32::from_bits(words[10]),
        maximum_distance: f32::from_bits(words[11]),
        state: words[12],
        distance: words[13],
        flag_006e43e0: words[14] as u8,
    };
    RetVal::into_ret(fn_0099c230(e, Ptr::new(words[0]), check))
}

/// `009052c0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const ELEMENT_TARGET_009052C0: u32 = 0x0090_52c0;

// Translated from 009a3280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (callers `SaveGame`, `LoadGame` of the combat state and
/// `CalculateLocationToExamine`): forwards `this` and an index to `009052c0` (in
/// `highprocess.cpp` per the engine map), which returns the word at `this+4` plus `index`
/// times 0x54: the address of a 0x54-byte element. The wrapper returns that address.
pub fn fn_009a3280(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    e.call(ELEMENT_TARGET_009052C0, &args![this, index]).ptr()
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVTESObjectWEAP@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108E068: u32 = 0x0108_e068;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VUnreachableCoverLocation@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108E07C: u32 = 0x0108_e07c;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VUnreachableLocation@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108E090: u32 = 0x0108_e090;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UWeaponData@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108E0A4: u32 = 0x0108_e0a4;

// Translated from 009a3410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVTESObjectWEAP@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009a3410(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E068);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 009a3460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VUnreachableCoverLocation@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009a3460(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E07C);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 009a35f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VUnreachableLocation@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009a35f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E090);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 009a3810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UWeaponData@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009a3810(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E0A4);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 009a3830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `cdecl`, two stack words: whether the 32-bit words the two pointers
/// point to are equal (`SETZ AL`, the result is `AL`).
pub fn fn_009a3830(e: &mut Engine, left: Ptr, right: Ptr) -> bool {
    e.mem.u32(left.addr()) == e.mem.u32(right.addr())
}

// Translated from 0099e040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float at +0xbc of `this` (`FLD`, so a signalling NaN comes
/// back quiet). No name in the engine map; the body reads +0xbc, which in
/// [`CombatState`] falls inside `AttackDialogueTimer`'s span, so the class
/// is unconfirmed.
pub fn fn_0099e040(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0xbc))
}

// Translated from 009a1260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float at +0xc8 of `this` (`FLD`, quieting a signalling NaN).
/// No name in the engine map; [`CombatState`] has bytes at +0xc8, so the
/// class is unconfirmed.
pub fn fn_009a1260(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0xc8))
}

// ---- Batch n025: `0099cb70` to `0099e060` -----------------------------------------------

/// `operator new` (`00401000`, `cdecl(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, `cdecl(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Stack frame of `0099cb70`; a local at `EBP - k` is at `frame + POINTS_FRAME - k`.
const POINTS_FRAME: u32 = 0x30;
/// `4194304.0` (`double`, 2048 squared).
const RANGE_SQUARED_DOUBLE: u32 = 0x0108_d658;
/// `16384.0` (`double`, 128 squared).
const COVER_MIN_DISTANCE_SQUARED_DOUBLE: u32 = 0x0106_cd60;
/// `0.7853981852531433` (`double`, the float nearest a quarter turn of pi).
const QUARTER_PI_DOUBLE: u32 = 0x0103_b100;
/// `1.5` (`double`).
const ONE_AND_A_HALF_DOUBLE: u32 = 0x0101_6ff0;
/// `1024.0` (`float`).
const THOUSAND_TWENTY_FOUR: u32 = 0x0102_36e0;
/// Stack frame of `0099ccd0` (below the exception frame).
const COVER_SAFE_FRAME: u32 = 0x68;
/// Stack frame of `0099cec0`.
const LEDGE_FRAME: u32 = 0x54;
/// Stack frame of `0099d030`.
const COVERED_MOVE_FRAME: u32 = 0x1a0;
/// The object at `011f1a63` that `00483710` is called on around the covered-move search of
/// `0099d030` (`00483710` only stores `ECX` in its frame).
const COVER_SEARCH_OBJECT_011F1A63: u32 = 0x011f_1a63;

// Translated from 0099cb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): reserves `points` for the member count
/// of the attacker's combat group (`005a4320`, then `006cee30`) and adds the position of
/// every member (`00968630`) to it (`0049f210`): the member's group location from
/// `00989140` (within 3 units) when there is one, else its current position (virtual slot
/// `+0x1f4`).
pub fn fn_0099cb70(e: &mut Engine, this: Ptr<CombatState>, points: u32) {
    let controller = state_controller(e, this);
    let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
    let count = e.call(0x005a_4320, &args![group]).u32();
    e.call(0x006c_ee30, &args![points, count]);
    e.with_stack(POINTS_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + POINTS_FRAME - k;
        let mut index = 0u32;
        while index < count {
            let member = e.call(0x0096_8630, &args![group, index]).u32();
            let world = local(0x1c);
            e.call(0x0097_ede0, &args![world]);
            let position = local(0x28);
            e.call(0x0068_15c0, &args![position]);
            let three = constant_bits(e, THREE);
            if e.call(0x0098_9140, &args![group, index, world, three])
                .bool()
            {
                let source = e.call(0x0068_15c0, &args![world]).u32();
                copy_words(e, position, source, 3);
            } else {
                let source = e.vcall(member, 0x1f4, &args![]).u32();
                copy_words(e, position, source, 3);
            }
            e.call(0x0049_f210, &args![points, position]);
            index += 1;
        }
    });
}

// Translated from 0099cc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words): whether `point` has line of sight to
/// `location` (`Pathing::CheckLineOfSight` `006d7490`, with 0, 0). Unless `skip_range` is
/// set, a point farther than 2048 units first fails: the squared length (`004a7290`) of
/// `point` minus the location's point (`005c3420`, `00439ef0`) above the `double`
/// 4194304 at `0108d658` (a NaN goes on to the line-of-sight test).
pub fn fn_0099cc60(e: &mut Engine, location: u32, point: u32, skip_range: u8) -> bool {
    if skip_range == 0 {
        let far = e.with_stack(0x18, |e, frame| {
            let own = frame.addr();
            let difference = frame.addr() + 0xc;
            let from = e.call(0x005c_3420, &args![location, own]).u32();
            e.call(0x0043_9ef0, &args![point, difference, from]);
            let squared = e.call(0x004a_7290, &args![difference]).f64();
            let limit: f64 = e.global(RANGE_SQUARED_DOUBLE);
            squared > limit
        });
        if far {
            return false;
        }
    }
    e.call(0x006d_7490, &args![location, point, 0u32, 0u32])
        .bool()
}

// Translated from 0099ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099ccd0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `CombatState::CheckCoverSafe` (Xbox PDB; `thiscall`, `RET 0x10`): whether `cover` (a
/// `PathingCoverLocation`) at `point` is safe from `actor`. The squared distance from the
/// actor's position (slot `+0x1f4`) to `point` (`00439ef0`, `004a7290`, rounded to a
/// float) below 128 squared (`double` at `0106cd60`) is unsafe. With `check_heading`, a
/// heading difference (`004b15e0` of the actor's heading, slot `+0x2bc(0)`, and the angle of
/// the offset, `004b13c0`) whose magnitude (`00408840`) is above the `double` at `0103b100`
/// is safe. A melee actor (`009a9630`) is safe at or beyond 1.5 times its weapon's reach
/// (`009a69c0` with `Actor::GetCurrentWeapon` `008a1710`), squared. Otherwise a pathing
/// location at `point` for the actor (`006dce10`) with line of sight to the actor's
/// position (`006d7490`) is unsafe, and a ledge cover (`006e4820`) must also pass
/// `0099cec0`; the location is destroyed (`004ff7e0`) on each of these exits.
pub fn combat_state_check_cover_safe(
    e: &mut Engine,
    this: Ptr<CombatState>,
    cover: u32,
    point: u32,
    actor: u32,
    check_heading: u8,
) -> bool {
    e.with_stack(COVER_SAFE_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + COVER_SAFE_FRAME - k;
        let position = local(0x28);
        let source = e.vcall(actor, 0x1f4, &args![]).u32();
        copy_words(e, position, source, 3);
        let difference = local(0x18);
        e.call(0x0043_9ef0, &args![point, difference, position]);
        let squared = x87_store32(e.call(0x004a_7290, &args![difference]).f64());
        e.mem.set_u32(local(0x1c), squared);
        let minimum: f64 = e.global(COVER_MIN_DISTANCE_SQUARED_DOUBLE);
        if x87_load32(squared) < minimum {
            return false;
        }
        if check_heading != 0 {
            let angle = x87_store32(e.call(0x004b_13c0, &args![difference]).f64());
            let heading = x87_store32(e.vcall(actor, 0x2bc, &args![0u32]).f64());
            let turn = e
                .call(
                    0x004b_15e0,
                    &args![heading, x87_quiet_bits(angle), local(0x30)],
                )
                .f64();
            let turn = x87_store32(turn);
            let magnitude = e.call(0x0040_8840, &args![x87_quiet_bits(turn)]).f64();
            let limit: f64 = e.global(QUARTER_PI_DOUBLE);
            if magnitude > limit {
                return true;
            }
        }
        if e.call(0x009a_9630, &args![actor]).bool() {
            let weapon = e.call(0x008a_1710, &args![actor]).u32();
            let reach = e.call(0x009a_69c0, &args![actor, weapon]).f64();
            let factor: f64 = e.global(ONE_AND_A_HALF_DOUBLE);
            let range = x87_store32(x87_mul(reach, factor));
            let range_squared = x87_mul(x87_load32(range), x87_m32(range));
            return !(x87_load32(squared) < range_squared);
        }
        let location = local(0x60);
        e.call(0x006d_ce10, &args![location, point, actor]);
        if e.call(0x006d_7490, &args![location, position, 0u32, 0u32])
            .bool()
        {
            e.call(0x004f_f7e0, &args![location]);
            return false;
        }
        if e.call(0x006e_4820, &args![cover]).bool() {
            let current = e.vcall(actor, 0x1f4, &args![]).u32();
            if !combat_state_check_ledge_cover_angle(e, this, cover, point, current, actor) {
                e.call(0x004f_f7e0, &args![location]);
                return false;
            }
        }
        e.call(0x004f_f7e0, &args![location]);
        true
    })
}

// Translated from 0099cec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::CheckLedgeCoverAngle` (Xbox PDB; `thiscall`, `RET 0x10`): whether the
/// ledge of `cover` (a `PathingCoverLocation`) hides `point` from `position`. The ledge
/// height `0040ebd0(cover+0x30, cover+0x3c)` must be above `position`'s height and above
/// that height raised by `actor`'s firing offset (`009a7fa0(actor, 1)`); the line from the
/// raised position to `point` must cross the ledge (`006ba6e0` with `cover+0x28`,
/// `cover+0x34`, writing the crossing point); then the drop from the crossing height times
/// the distance from the crossing to `point` over the distance to the raised position
/// (`00589850` of the `00439ef0` differences) must reach the controller's crouch or stand
/// height (`00453700` when `00998a30` says so, else `004536e0`).
pub fn combat_state_check_ledge_cover_angle(
    e: &mut Engine,
    this: Ptr<CombatState>,
    cover: u32,
    point: u32,
    position: u32,
    actor: u32,
) -> bool {
    let first = x87_quiet_bits(e.mem.u32(cover.wrapping_add(0x30)));
    let second = x87_quiet_bits(e.mem.u32(cover.wrapping_add(0x3c)));
    let height = x87_store32(e.call(0x0040_ebd0, &args![first, second]).f64());
    let ground = x87_load32(e.mem.u32(position.wrapping_add(8)));
    if x87_load32(height) <= ground {
        return false;
    }
    e.with_stack(LEDGE_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + LEDGE_FRAME - k;
        let raised_point = local(0x10);
        copy_words(e, raised_point, position, 3);
        let offset = e.call(0x009a_7fa0, &args![actor, 1u32]).f64();
        let raised = x87_store32(x87_add(offset, x87_m32(e.mem.u32(raised_point + 8))));
        e.mem.set_u32(raised_point + 8, raised);
        if x87_load32(height) <= x87_load32(raised) {
            return false;
        }
        let crossing = local(0x48);
        e.call(0x0068_15c0, &args![crossing]);
        let crossed = e
            .call(
                0x006b_a6e0,
                &args![
                    cover.wrapping_add(0x28),
                    cover.wrapping_add(0x34),
                    raised_point,
                    point,
                    crossing
                ],
            )
            .u8();
        e.mem.set_u8(local(0x25), crossed);
        if crossed == 0 {
            return false;
        }
        let drop = x87_store32(x87_sub(
            x87_load32(e.mem.u32(crossing + 8)),
            x87_m32(raised),
        ));
        let near = local(0x24);
        e.call(0x0043_9ef0, &args![crossing, near, raised_point]);
        let near_length = x87_store32(e.call(0x0058_9850, &args![near]).f64());
        let far = local(0x38);
        e.call(0x0043_9ef0, &args![crossing, far, point]);
        let far_length = x87_store32(e.call(0x0058_9850, &args![far]).f64());
        let projected = x87_store32(x87_div(
            x87_mul(x87_load32(drop), x87_m32(far_length)),
            x87_m32(near_length),
        ));
        let controller = state_controller(e, this);
        let height_getter = if fn_00998a30(e, controller) != 0 {
            0x0045_3700
        } else {
            0x0045_36e0
        };
        let controller = state_controller(e, this);
        let limit = x87_store32(e.call(height_getter, &args![controller]).f64());
        let limit = x87_quiet_bits(limit);
        !(x87_load32(projected) < x87_load32(limit))
    })
}

// Translated from 0099d030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099d030 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0x1c`): searches a covered move to cover of
/// `kind` near `point` and fills `location` with it; true when one is found.
/// - The search radius is `radius`, or 256 (`01032adc`) when it is zero; for kind 6 it is
///   `setting(011ce618) + (setting(011cf6e8) - setting(011ce618)) * (1 - v / 100)` with
///   `v` from `00495460` of the controller's `00980070`.
/// - `search_info`+0x30 is cleared. When `cover` has a key (+0x68) and is of this kind
///   (+0x4), the combat manager's cover map (`009611e0` of `011f1958`) is asked first
///   (`009835d0` with the key, `cover`+0x14, the controller, the attacker's radius
///   `008be1b0`, `cover`+0x70, 1 and the byte `cover`+0x75); a yes ends the search.
/// - Otherwise a `PathingRequestCoveredMove` (`006e4a50`) is built (`0099d310`, the
///   attacker's location `006dcd70` as start, `point` for the attacker `006dce10` as goal,
///   `search_info`, `cover`, `flags`, `kind`) and run through `0099d480`; its result point
///   (`006377e0`) goes into `location` (`0099b8b0`, space 0). `00483710` is called on
///   `011f1a63` before and after; the locals are destroyed (`004ff7e0` twice, `006e4bf0`).
/// - The frame also receives 1024 (`010236e0`), 2048 (`01018bfc`), 200 and the target
///   (`004030b0`), none of which is read again.
#[allow(clippy::too_many_arguments)]
pub fn fn_0099d030(
    e: &mut Engine,
    this: Ptr<CombatState>,
    search_info: u32,
    cover: u32,
    location: u32,
    kind: u32,
    point: u32,
    radius: f32,
    flags: u32,
) -> bool {
    e.with_stack(COVERED_MOVE_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + COVERED_MOVE_FRAME - k;
        let unused = constant_bits(e, THOUSAND_TWENTY_FOUR);
        e.mem.set_u32(local(0x18), unused);
        let unused = constant_bits(e, TWO_THOUSAND_FORTY_EIGHT);
        e.mem.set_u32(local(0x130), unused);
        e.mem.set_u32(local(0x160), 200);
        let attacker = attacker(e, this);
        e.mem.set_u32(local(0x14), attacker);
        let target = target(e, this);
        e.mem.set_u32(local(0x164), target);
        let zero: f64 = e.global(ZERO_DOUBLE);
        let mut search_radius = if x87_load32(radius.to_bits()) == zero {
            constant_bits(e, TWO_HUNDRED_FIFTY_SIX)
        } else {
            x87_quiet_bits(radius.to_bits())
        };
        if kind == 6 {
            let base = setting_value_address(e, 0x011c_e618);
            let controller = state_controller(e, this);
            let source = e.call(0x0098_0070, &args![controller]).u32();
            let value = e.call(0x0049_5460, &args![source]).f64();
            let hundred: f64 = e.global(HUNDRED_DOUBLE);
            let scale = x87_sub(1.0, x87_div(value, hundred));
            let high = setting_value_address(e, 0x011c_f6e8);
            let low = setting_value_address(e, 0x011c_e618);
            let span = x87_sub(x87_load32(e.mem.u32(high)), x87_m32(e.mem.u32(low)));
            search_radius = x87_store32(x87_add(x87_mul(span, scale), x87_m32(e.mem.u32(base))));
        }
        e.mem.set_u32(search_info.wrapping_add(0x30), 0);
        if e.mem.u32(cover.wrapping_add(0x68)) != 0 && kind == e.mem.u32(cover.wrapping_add(4)) {
            let flag = u32::from(e.mem.u8(cover.wrapping_add(0x75)));
            let size = x87_store32(e.call(0x008b_e1b0, &args![attacker]).f64());
            let controller = state_controller(e, this);
            let key = e.mem.u32(cover.wrapping_add(0x68));
            let manager: u32 = e.global(COMBAT_MANAGER);
            let cover_map = e.call(0x0096_11e0, &args![manager]).u32();
            if e.call(
                0x0098_35d0,
                &args![
                    cover_map,
                    key,
                    cover.wrapping_add(0x14),
                    controller,
                    size,
                    cover.wrapping_add(0x70),
                    1u32,
                    flag
                ],
            )
            .bool()
            {
                return true;
            }
        }
        e.call(0x0048_3710, &args![COVER_SEARCH_OBJECT_011F1A63]);
        let request = local(0x12c);
        e.call(0x006e_4a50, &args![request]);
        combat_state_build_covered_move_request(e, this, request);
        let start = local(0x15c);
        e.call(0x006d_cd70, &args![start, attacker]);
        e.call(0x006d_3ac0, &args![request, start]);
        let goal = local(0x18c);
        e.call(0x006d_ce10, &args![goal, point, attacker]);
        e.call(0x006d_1bc0, &args![request, goal]);
        e.call(0x006d_30e0, &args![request, search_info]);
        e.call(0x006d_3100, &args![request, cover]);
        e.call(0x006d_61e0, &args![request, flags]);
        e.call(0x0070_5b10, &args![request, kind]);
        let found = fn_0099d480(e, request, f32::from_bits(x87_quiet_bits(search_radius)), 0);
        e.mem.set_u8(local(0x131), u8::from(found));
        if found {
            let result = e.call(0x0063_77e0, &args![request]).u32();
            fn_0099b8b0(e, Ptr::new(location), result, 0);
        }
        e.call(0x006d_3120, &args![request]);
        e.call(0x0048_3710, &args![COVER_SEARCH_OBJECT_011F1A63]);
        e.call(0x004f_f7e0, &args![goal]);
        e.call(0x004f_f7e0, &args![start]);
        e.call(0x006e_4bf0, &args![request]);
        found
    })
}

// Translated from 0099d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::BuildCoveredMoveRequest` (Xbox PDB; `thiscall`, `RET 4`): fills the
/// `PathingRequestCoveredMove` at `request` from the attacker: its radius (`008be1b0` into
/// `00507610`), speed (`008a0b10` into `006e5ee0`), the controller's stand height
/// (`004536e0` into `006e4ef0`) and, when `00998a30` says so, crouch height (`00453700`
/// into `006e4320`), and the integer from its +0xa4 object's slot 8 (with 0x10) as a float
/// (`005076c0`). With a target: the target's position (slot `+0x1f4`, `006e4f10`) when
/// `0097ee90` holds and the target is in the same space, and the group positions
/// (`0099cb70` into `006e4fb0`'s array). Then the unreachable cover locations
/// (`009a0e10` into `006e4f90`'s array), plus the current cover location's key when it is
/// flagged (+1) (`007cb2e0`).
pub fn combat_state_build_covered_move_request(
    e: &mut Engine,
    this: Ptr<CombatState>,
    request: u32,
) {
    let attacker = attacker(e, this);
    let target = target(e, this);
    let radius = x87_store32(e.call(0x008b_e1b0, &args![attacker]).f64());
    e.call(0x0050_7610, &args![request, radius]);
    let speed = x87_store32(e.call(0x008a_0b10, &args![attacker]).f64());
    e.call(0x006e_5ee0, &args![request, speed]);
    let controller = state_controller(e, this);
    let stand = x87_store32(e.call(0x0045_36e0, &args![controller]).f64());
    e.call(0x006e_4ef0, &args![request, stand]);
    let controller = state_controller(e, this);
    if fn_00998a30(e, controller) != 0 {
        let controller = state_controller(e, this);
        let crouch = x87_store32(e.call(0x0045_3700, &args![controller]).f64());
        e.call(0x006e_4320, &args![request, crouch]);
    }
    let value = e
        .vcall(attacker.wrapping_add(0xa4), 0x8, &args![0x10u32])
        .u32();
    let value = x87_store32(f64::from(value as i32));
    e.call(0x0050_76c0, &args![request, value]);
    if target != 0 {
        if e.call(0x0097_ee90, &args![this]).bool()
            && e.get(this, CombatState::bTargetSpaceDifferent) == 0
        {
            let position = e.vcall(target, 0x1f4, &args![]).u32();
            e.call(0x006e_4f10, &args![request, position]);
        }
        let points = e.call(0x006e_4fb0, &args![request]).u32();
        fn_0099cb70(e, this, points);
    }
    let unreachable = e.call(0x006e_4f90, &args![request]).u32();
    combat_state_build_unreachable_cover_locations_array(e, this, unreachable);
    let cover = e.get(this, CombatState::pCoverLocation);
    if cover != 0 && e.mem.u8(cover.wrapping_add(1)) != 0 {
        let key = e.get(this, CombatState::pCoverLocation).wrapping_add(0x68);
        let array = e.call(0x006e_4f90, &args![request]).u32();
        e.call(0x007c_b2e0, &args![array, key]);
    }
}

// Translated from 0099e010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): copies the three words at `point` to
/// +0xe0 of `this` (`0099d480` calls it on its `PathingRequestCover`; the class is not
/// confirmed).
pub fn fn_0099e010(e: &mut Engine, this: Ptr, point: Ptr) {
    copy_words(e, this.addr().wrapping_add(0xe0), point.addr(), 3);
}

// Translated from 0099e060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099e060 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0x14`): finds a secondary cover location.
/// The old one is cleared (`009a0470(0)`), a new `CombatCoverLocation` (0x78 bytes,
/// `00401000`, `006d3150`) becomes `pSecondaryCoverLocation`, and a covered move is
/// searched for it (`0099d030` with `SecondaryCoverSearchInfo`, a local `CombatLocation`
/// from `0097ee20`, `kind`, `point`, `radius`, `flags`). On success its key (`006e4570`)
/// and reservation (`006e4740`) of the `PathingCoverLocation` at +0x14 are stored at +0x68
/// and +0x6c and that location is copied to `out` (`006e4430`); otherwise the secondary
/// cover location is cleared again.
pub fn fn_0099e060(
    e: &mut Engine,
    this: Ptr<CombatState>,
    out: u32,
    point: u32,
    kind: u32,
    radius: f32,
    flags: u32,
) -> bool {
    combat_state_clear_secondary_cover_location(e, this, 0);
    let block = e.call(OPERATOR_NEW, &args![0x78u32]).u32();
    let cover = if block != 0 {
        e.call(0x006d_3150, &args![block]).u32()
    } else {
        0
    };
    e.set(this, CombatState::pSecondaryCoverLocation, cover);
    e.with_stack(0x20, |e, location| {
        fn_0097ee20(e, location.cast());
        let cover = e.get(this, CombatState::pSecondaryCoverLocation);
        let found = fn_0099d030(
            e,
            this,
            member(this, 0x128),
            cover,
            location.addr(),
            kind,
            point,
            x87_quiet(radius),
            flags,
        );
        if !found {
            combat_state_clear_secondary_cover_location(e, this, 0);
            return false;
        }
        let cover = e.get(this, CombatState::pSecondaryCoverLocation);
        let key = e.call(0x006e_4570, &args![cover + 0x14]).u32();
        let cover = e.get(this, CombatState::pSecondaryCoverLocation);
        e.mem.set_u32(cover + 0x68, key);
        let cover = e.get(this, CombatState::pSecondaryCoverLocation);
        let reservation = e.call(0x006e_4740, &args![cover + 0x14]).u32();
        let cover = e.get(this, CombatState::pSecondaryCoverLocation);
        e.mem.set_u32(cover + 0x6c, reservation);
        let cover = e.get(this, CombatState::pSecondaryCoverLocation);
        e.call(0x006e_4430, &args![out, cover + 0x14]);
        true
    })
}
// ---- Batch n025: `009a02a0` to `009a1380` -----------------------------------------------

/// `0.75` (`double`).
const THREE_QUARTERS_DOUBLE: u32 = 0x0101_de30;
/// `0.8999999761581421` (`double`, the float nearest 0.9).
const NINE_TENTHS_DOUBLE: u32 = 0x0106_b9e8;
/// `4096.0` (`double`, 64 squared).
const FOUR_THOUSAND_NINETY_SIX_DOUBLE: u32 = 0x0101_7a10;
/// `16384.0` (`float`, 128 squared).
const SIXTEEN_THOUSAND_THREE_HUNDRED_EIGHTY_FOUR: u32 = 0x0106_6510;
/// The `Setting` whose value `009a0790` and `009a0910` pass as the second argument of
/// `00404010` with each location-target distance.
const LOCATION_TARGET_DISTANCE_SETTING: u32 = 0x011c_f0d8;
/// The `Setting` whose value `009a0b70` and `009a0be0` add to the location target's maximum
/// distance.
const LOCATION_TARGET_BONUS_SETTING: u32 = 0x011c_e7f0;

/// `CombatLocationTarget` at `pLocationTarget`.
fn location_target(e: &Engine, this: Ptr<CombatState>) -> Ptr<CombatLocationTarget> {
    Ptr::new(e.get(this, CombatState::pLocationTarget))
}

// Translated from 009a02a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::CheckMovement` (Xbox PDB; `thiscall`, `RET 0xc`): whether moving to
/// `location` (a `BGSWorldLocation`) is allowed. With a location target of type 1 (unless
/// `00461580(this, 0x100)` holds), type 2, or type 3 (unless `skip_type_three`), the
/// location's squared distance to the target's world location (`00527a40`) must not exceed
/// the square of `009a0be0`'s distance. With `check_unreachable`, a location unreachable
/// within the default radius (`009a0f40` with -1) is refused.
pub fn combat_state_check_movement(
    e: &mut Engine,
    this: Ptr<CombatState>,
    location: u32,
    check_unreachable: u8,
    skip_type_three: u8,
) -> bool {
    if e.get(this, CombatState::pLocationTarget) != 0 {
        let mut check = false;
        if e.get(location_target(e, this), CombatLocationTarget::cType) == 1
            && !e.call(0x0046_1580, &args![this, 0x100u32]).bool()
        {
            check = true;
        }
        if !check {
            let kind = e.get(location_target(e, this), CombatLocationTarget::cType);
            if kind == 2 || (kind == 3 && skip_type_three == 0) {
                check = true;
            }
        }
        if check {
            let reach = fn_009a0be0(e, this).to_bits();
            let target_location = location_target(e, this).addr() + 8;
            let squared = e.call(0x0052_7a40, &args![location, target_location]).f64();
            if x87_mul(x87_load32(reach), x87_m32(reach)) < squared {
                return false;
            }
        }
    }
    if check_unreachable != 0 {
        let radius = f32::from_bits(constant_bits(e, MINUS_ONE));
        if combat_state_is_location_unreachable(e, this, location, radius) {
            return false;
        }
    }
    true
}

// Translated from 009a0370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): clears the cover location (`009a03d0`
/// with 0) and stores `cover` as `pCoverLocation`.
pub fn fn_009a0370(e: &mut Engine, this: Ptr<CombatState>, cover: u32) {
    combat_state_clear_cover_location(e, this, 0);
    e.set(this, CombatState::pCoverLocation, cover);
}

// Translated from 009a03a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): clears the secondary cover location
/// (`009a0470` with 0) and stores `cover` as `pSecondaryCoverLocation`.
pub fn fn_009a03a0(e: &mut Engine, this: Ptr<CombatState>, cover: u32) {
    combat_state_clear_secondary_cover_location(e, this, 0);
    e.set(this, CombatState::pSecondaryCoverLocation, cover);
}

/// The body of `009a03d0` and `009a0470` on the cover location pointer at `field`.
fn clear_cover_location_at(
    e: &mut Engine,
    this: Ptr<CombatState>,
    field: Field<CombatState, u32>,
    mark_unreachable: u8,
) {
    if e.get(this, field) == 0 {
        return;
    }
    if mark_unreachable != 0 {
        let key = e.mem.u32(e.get(this, field) + 0x68);
        combat_state_add_unreachable_cover_location(e, this, key);
    }
    let cover = e.get(this, field);
    if e.call(0x005d_c960, &args![cover]).bool() {
        let cover = e.get(this, field);
        e.call(0x009a_05b0, &args![this, cover]);
    }
    let cover = e.get(this, field);
    if cover != 0 {
        e.call(0x006d_3370, &args![cover, 1u32]);
    }
    e.set(this, field, 0);
}

// Translated from 009a03d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::ClearCoverLocation` (Xbox PDB; `thiscall`, `RET 4`): with a cover
/// location, records its key (+0x68) as unreachable when `mark_unreachable` is set
/// (`009a0d00`), releases its reservation when `005dc960` says it is reserved
/// (`009a05b0`), deletes it (`006d3370` with 1) and clears `pCoverLocation`.
pub fn combat_state_clear_cover_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    mark_unreachable: u8,
) {
    clear_cover_location_at(e, this, CombatState::pCoverLocation, mark_unreachable);
}

// Translated from 009a0470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::ClearSecondaryCoverLocation` (Xbox PDB; `thiscall`, `RET 4`): as
/// `ClearCoverLocation` for `pSecondaryCoverLocation`.
pub fn combat_state_clear_secondary_cover_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    mark_unreachable: u8,
) {
    clear_cover_location_at(
        e,
        this,
        CombatState::pSecondaryCoverLocation,
        mark_unreachable,
    );
}

// Translated from 009a0600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::IsCoverLocationValid` (Xbox PDB; `thiscall`, `RET 4`): whether `cover` is
/// non-null and is the cover or the secondary cover location.
pub fn combat_state_is_cover_location_valid(
    e: &mut Engine,
    this: Ptr<CombatState>,
    cover: u32,
) -> bool {
    if cover == 0 {
        return false;
    }
    cover == e.get(this, CombatState::pCoverLocation)
        || cover == e.get(this, CombatState::pSecondaryCoverLocation)
}

// Translated from 009a0650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): frees the combat location
/// (`009a0680`) and stores `location` as `pCombatLocation`.
pub fn fn_009a0650(e: &mut Engine, this: Ptr<CombatState>, location: u32) {
    fn_009a0680(e, this);
    e.set(this, CombatState::pCombatLocation, location);
}

// Translated from 009a0680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): frees `pCombatLocation` (`operator delete`,
/// also when null) and clears it.
pub fn fn_009a0680(e: &mut Engine, this: Ptr<CombatState>) {
    let location = e.get(this, CombatState::pCombatLocation);
    e.call(OPERATOR_DELETE, &args![location]);
    e.set(this, CombatState::pCombatLocation, 0);
}

// Translated from 009a06c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a06c0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `CombatState::SetCombatItem` (Xbox PDB; `thiscall`, `RET 0xc`): releases combat item
/// `index` (`00981340`); with an `item`, a new 8-byte record (`00401000`, `0096a2d0`)
/// holding `item` and `value` becomes `pCombatItems[index]` and the item is marked
/// targeted (`00564db0` with 1).
pub fn combat_state_set_combat_item(
    e: &mut Engine,
    this: Ptr<CombatState>,
    index: u32,
    item: u32,
    value: u32,
) {
    e.call(0x0098_1340, &args![this, index]);
    if item == 0 {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let record = if block != 0 {
        e.call(0x0096_a2d0, &args![block]).u32()
    } else {
        0
    };
    let slot = member(this, 0x190).wrapping_add(index.wrapping_mul(4));
    e.mem.set_u32(slot, record);
    let record = e.mem.u32(slot);
    e.mem.set_u32(record, item);
    let record = e.mem.u32(slot);
    e.mem.set_u32(record + 4, value);
    if item != 0 {
        e.call(0x0056_4db0, &args![item, 1u32]);
    }
}

/// The two distances of a new location target: each `00404010(distance, setting)` with the
/// value of the setting at `011cf0d8`, rounded to a float.
fn set_location_target_distances(
    e: &mut Engine,
    this: Ptr<CombatState>,
    minimum: f32,
    maximum: f32,
) {
    let setting = setting_bits(e, LOCATION_TARGET_DISTANCE_SETTING);
    let value = e
        .call(
            0x0040_4010,
            &args![x87_quiet_bits(minimum.to_bits()), setting],
        )
        .f64();
    let target = location_target(e, this);
    e.set(
        target,
        CombatLocationTarget::fMinDistance,
        f32::from_bits(x87_store32(value)),
    );
    let setting = setting_bits(e, LOCATION_TARGET_DISTANCE_SETTING);
    let value = e
        .call(
            0x0040_4010,
            &args![x87_quiet_bits(maximum.to_bits()), setting],
        )
        .f64();
    let target = location_target(e, this);
    e.set(
        target,
        CombatLocationTarget::fMaxDistance,
        f32::from_bits(x87_store32(value)),
    );
}

/// A new `CombatLocationTarget` (0x20 bytes, `00401000`, `009a08d0`) as
/// `pLocationTarget`, of `kind` with `reference`.
fn new_location_target(e: &mut Engine, this: Ptr<CombatState>, kind: u8, reference: u32) {
    let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
    let target = if block != 0 {
        fn_009a08d0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.set(this, CombatState::pLocationTarget, target);
    e.set(location_target(e, this), CombatLocationTarget::cType, kind);
    e.set(
        location_target(e, this),
        CombatLocationTarget::pRef,
        reference,
    );
}

// Translated from 009a0790 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a0790 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `CombatState::SetLocationTargetRef` (Xbox PDB; `thiscall`, `RET 0x10`): frees the
/// location target (`009a0a40`); with a `reference`, a new one of `kind` for it, its world
/// location (`TESObjectREFR::GetWorldLocation`) and both distances (`00404010` with the
/// setting at `011cf0d8`).
pub fn combat_state_set_location_target_ref(
    e: &mut Engine,
    this: Ptr<CombatState>,
    reference: u32,
    kind: u8,
    minimum: f32,
    maximum: f32,
) {
    fn_009a0a40(e, this);
    if reference == 0 {
        return;
    }
    new_location_target(e, this, kind, reference);
    e.with_stack(0x10, |e, world| {
        let source = e.call(GET_WORLD_LOCATION, &args![reference, world]).u32();
        let to = location_target(e, this).addr() + 8;
        copy_words(e, to, source, 4);
    });
    set_location_target_distances(e, this, minimum, maximum);
}

// Translated from 009a08d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatLocationTarget` constructor (by its body; `thiscall`): type 0, not near the
/// target, no reference, the `BGSWorldLocation` through `0097ede0`, both distances 0.
/// Returns `this`.
pub fn fn_009a08d0(e: &mut Engine, this: Ptr<CombatLocationTarget>) -> Ptr<CombatLocationTarget> {
    e.set(this, CombatLocationTarget::cType, 0);
    e.set(this, CombatLocationTarget::bNearTarget, 0);
    e.set(this, CombatLocationTarget::pRef, 0);
    e.call(0x0097_ede0, &args![this.addr() + 8]);
    e.set(this, CombatLocationTarget::fMinDistance, 0.0);
    e.set(this, CombatLocationTarget::fMaxDistance, 0.0);
    this
}

// Translated from 009a0910 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a0910 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0x10`): as `SetLocationTargetRef`
/// (`009a0790`) for a world location instead of a reference: the new target of `kind` has
/// no reference and a copy of the `BGSWorldLocation` at `world_location`.
pub fn fn_009a0910(
    e: &mut Engine,
    this: Ptr<CombatState>,
    world_location: u32,
    kind: u8,
    minimum: f32,
    maximum: f32,
) {
    fn_009a0a40(e, this);
    new_location_target(e, this, kind, 0);
    let to = location_target(e, this).addr() + 8;
    copy_words(e, to, world_location, 4);
    set_location_target_distances(e, this, minimum, maximum);
}

// Translated from 009a0a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): frees `pLocationTarget` (`operator delete`, also
/// when null) and clears it.
pub fn fn_009a0a40(e: &mut Engine, this: Ptr<CombatState>) {
    let target = e.get(this, CombatState::pLocationTarget);
    e.call(OPERATOR_DELETE, &args![target]);
    e.set(this, CombatState::pLocationTarget, 0);
}

// Translated from 009a0a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): with a target, true when `0097ee90` holds or the
/// controller's `00981420` does; without one, true when the location target is of type 2.
pub fn fn_009a0a80(e: &mut Engine, this: Ptr<CombatState>) -> bool {
    let target = target(e, this);
    if target != 0 {
        if e.call(0x0097_ee90, &args![this]).bool() {
            return true;
        }
        let controller = state_controller(e, this);
        if e.call(0x0098_1420, &args![controller]).bool() {
            return true;
        }
    }
    target == 0
        && e.get(this, CombatState::pLocationTarget) != 0
        && e.get(location_target(e, this), CombatLocationTarget::cType) == 2
}

// Translated from 009a0b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::GetLocationTargetDistance` (Xbox PDB; `thiscall`): the location target's
/// maximum distance when `009a0a80` holds, else `009a0b30`.
pub fn combat_state_get_location_target_distance(e: &mut Engine, this: Ptr<CombatState>) -> f32 {
    if fn_009a0a80(e, this) {
        x87_quiet(e.get(location_target(e, this), CombatLocationTarget::fMaxDistance))
    } else {
        fn_009a0b30(e, this)
    }
}

// Translated from 009a0b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the location target's minimum distance times the
/// `double` at `0106b9e8` (0.9 as a float), 0 without a target; through a `double` local,
/// then rounded to a float.
pub fn fn_009a0b30(e: &mut Engine, this: Ptr<CombatState>) -> f32 {
    let value = if e.get(this, CombatState::pLocationTarget) != 0 {
        let minimum = e
            .get(location_target(e, this), CombatLocationTarget::fMinDistance)
            .to_bits();
        let factor: f64 = e.global(NINE_TENTHS_DOUBLE);
        x87_mul(x87_load32(minimum), factor)
    } else {
        0.0
    };
    f32::from_bits(x87_store32(value))
}

// Translated from 009a0b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): for a location target near its target (+1), the
/// maximum distance plus the setting at `011ce7f0`; otherwise three quarters of the way from
/// the minimum to the maximum distance (`double` 0.75 at `0101de30`). Rounded to a float.
pub fn fn_009a0b70(e: &mut Engine, this: Ptr<CombatState>) -> f32 {
    if e.get(location_target(e, this), CombatLocationTarget::bNearTarget) != 0 {
        let target = location_target(e, this);
        let bonus = setting_value_address(e, LOCATION_TARGET_BONUS_SETTING);
        let maximum = e.get(target, CombatLocationTarget::fMaxDistance).to_bits();
        let sum = x87_add(x87_load32(maximum), x87_m32(e.mem.u32(bonus)));
        return f32::from_bits(x87_store32(sum));
    }
    let target = location_target(e, this);
    let maximum = e.get(target, CombatLocationTarget::fMaxDistance).to_bits();
    let minimum = e.get(target, CombatLocationTarget::fMinDistance).to_bits();
    let factor: f64 = e.global(THREE_QUARTERS_DOUBLE);
    let span = x87_mul(x87_sub(x87_load32(maximum), x87_m32(minimum)), factor);
    let minimum = e
        .get(location_target(e, this), CombatLocationTarget::fMinDistance)
        .to_bits();
    f32::from_bits(x87_store32(x87_add(span, x87_m32(minimum))))
}

// Translated from 009a0be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the location target's maximum distance, plus the
/// setting at `011ce7f0` (rounded to a float) when the controller has a group
/// (`004f9bf0`) whose `00825c00` value is none of 0, 6 and 15.
pub fn fn_009a0be0(e: &mut Engine, this: Ptr<CombatState>) -> f32 {
    let controller = state_controller(e, this);
    let group = e.call(0x004f_9bf0, &args![controller]).u32();
    if group != 0 {
        let kind = e.call(0x0082_5c00, &args![group]).u32();
        if kind != 0xf && kind != 6 && kind != 0 {
            let target = location_target(e, this);
            let bonus = setting_value_address(e, LOCATION_TARGET_BONUS_SETTING);
            let maximum = e.get(target, CombatLocationTarget::fMaxDistance).to_bits();
            let sum = x87_add(x87_load32(maximum), x87_m32(e.mem.u32(bonus)));
            return f32::from_bits(x87_store32(sum));
        }
    }
    x87_quiet(e.get(location_target(e, this), CombatLocationTarget::fMaxDistance))
}

// Translated from 009a0c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): without a location target (or with a
/// type 3 one and `replace` set), takes the range `00507b00` of the controller's
/// `00980070` object; a range above 0 becomes a type 3 location target at the attacker's
/// world location (`009a0910`) from the range times the setting at `011cfb2c` to the
/// range.
pub fn fn_009a0c50(e: &mut Engine, this: Ptr<CombatState>, replace: u8) {
    if e.get(this, CombatState::pLocationTarget) != 0
        && !(e.get(location_target(e, this), CombatLocationTarget::cType) == 3 && replace != 0)
    {
        return;
    }
    let controller = state_controller(e, this);
    let source = e.call(0x0098_0070, &args![controller]).u32();
    let range = x87_store32(e.call(0x0050_7b00, &args![source]).f64());
    let zero: f64 = e.global(ZERO_DOUBLE);
    if !(x87_load32(range) > zero) {
        return;
    }
    let attacker = attacker(e, this);
    let factor = setting_value_address(e, 0x011c_fb2c);
    let minimum = x87_store32(x87_mul(x87_load32(range), x87_m32(e.mem.u32(factor))));
    e.with_stack(0x10, |e, world| {
        let location = e.call(GET_WORLD_LOCATION, &args![attacker, world]).u32();
        fn_009a0910(
            e,
            this,
            location,
            3,
            f32::from_bits(x87_quiet_bits(minimum)),
            f32::from_bits(x87_quiet_bits(range)),
        );
    });
}

// Translated from 009a0d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::AddUnreachableCoverLocation` (Xbox PDB; `thiscall`, `RET 4`): for a
/// non-zero `key`, appends an entry to `UnreachableCoverLocations` (`00845460`,
/// `006950d0`) holding the key and a time stamp of the game time (`00435dd0`,
/// `00435de0`).
pub fn combat_state_add_unreachable_cover_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    key: u32,
) {
    if key == 0 {
        return;
    }
    let array = member(this, 0x164);
    let index = e.call(0x0084_5460, &args![array]).u32();
    let entry = e.call(0x0069_50d0, &args![array, index]).u32();
    e.mem.set_u32(entry, key);
    let now = game_time_bits(e);
    let stamp = time_stamp_word(e, now);
    let entry = e.call(0x0069_50d0, &args![array, index]).u32();
    e.mem.set_u32(entry + 4, stamp);
}

// Translated from 009a0d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether `key` is in
/// `UnreachableCoverLocations`; entries older than 10 (`00435e00` above the `double` 10)
/// are removed on the way (`00983720` with 1).
pub fn fn_009a0d70(e: &mut Engine, this: Ptr<CombatState>, key: u32) -> bool {
    let array = member(this, 0x164);
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![array]).u32() {
        let entry = e.call(0x0084_4160, &args![array, index]).u32();
        let age = e.call(TIME_STAMP_ELAPSED, &args![entry + 4]).f64();
        let limit: f64 = e.global(TEN_DOUBLE);
        if age > limit {
            e.call(0x0098_3720, &args![array, index, 1u32]);
            index = index.wrapping_sub(1);
        } else if e.mem.u32(entry) == key {
            return true;
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 009a0e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::BuildUnreachableCoverLocationsArray` (Xbox PDB; `thiscall`, `RET 4`):
/// reserves `out` for the entries of `UnreachableCoverLocations` (`0084b5c0`) and adds each
/// entry (`007cb2e0`); entries older than 10 are removed instead (`00983720` with 1).
pub fn combat_state_build_unreachable_cover_locations_array(
    e: &mut Engine,
    this: Ptr<CombatState>,
    out: u32,
) {
    let array = member(this, 0x164);
    let count = e.call(ARRAY_SIZE, &args![array]).u32();
    e.call(0x0084_b5c0, &args![out, count]);
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![array]).u32() {
        let entry = e.call(0x0084_4160, &args![array, index]).u32();
        let age = e.call(TIME_STAMP_ELAPSED, &args![entry + 4]).f64();
        let limit: f64 = e.global(TEN_DOUBLE);
        if age > limit {
            e.call(0x0098_3720, &args![array, index, 1u32]);
            index = index.wrapping_sub(1);
        } else {
            e.call(0x007c_b2e0, &args![out, entry]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 009a0ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::AddUnreachableLocation` (Xbox PDB; `thiscall`, `RET 4`): appends an entry
/// to `UnreachableLocations` (`007881d0`, `006b3160`) holding a copy of the
/// `BGSWorldLocation` at `world_location` and a time stamp of the game time at +0x10.
pub fn combat_state_add_unreachable_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    world_location: u32,
) {
    let array = member(this, 0x174);
    let index = e.call(0x0078_81d0, &args![array]).u32();
    let entry = e.call(0x006b_3160, &args![array, index]).u32();
    copy_words(e, entry, world_location, 4);
    let now = game_time_bits(e);
    let stamp = time_stamp_word(e, now);
    let entry = e.call(0x006b_3160, &args![array, index]).u32();
    e.mem.set_u32(entry + 0x10, stamp);
}

// Translated from 009a0f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::IsLocationUnreachable` (Xbox PDB; `thiscall`, `RET 8`): whether an entry
/// of `UnreachableLocations` (`006dad90`) lies within `radius` of `location` (the squared
/// distance `00527a40` below `radius` squared; a radius of -1 means 128, the `float` 16384 at
/// `01066510`). Entries older than 10 are removed on the way (`0098f1f0` with 1).
pub fn combat_state_is_location_unreachable(
    e: &mut Engine,
    this: Ptr<CombatState>,
    location: u32,
    radius: f32,
) -> bool {
    let minus_one: f64 = e.global(MINUS_ONE_DOUBLE);
    let bits = radius.to_bits();
    let radius_squared = if x87_load32(bits) == minus_one {
        constant_bits(e, SIXTEEN_THOUSAND_THREE_HUNDRED_EIGHTY_FOUR)
    } else {
        x87_store32(x87_mul(x87_load32(bits), x87_m32(bits)))
    };
    let array = member(this, 0x174);
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![array]).u32() {
        let entry = e.call(0x006d_ad90, &args![array, index]).u32();
        let age = e.call(TIME_STAMP_ELAPSED, &args![entry + 0x10]).f64();
        let limit: f64 = e.global(TEN_DOUBLE);
        if age > limit {
            e.call(0x0098_f1f0, &args![array, index, 1u32]);
            index = index.wrapping_sub(1);
        } else {
            let squared = e.call(0x0052_7a40, &args![entry, location]).f64();
            if x87_load32(radius_squared) > squared {
                return true;
            }
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 009a1010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): updates `fDamagePerSecond` from the attacker's
/// weapon damage per second (`008be600`) and `fCombatStrength` from it
/// (`008acbe0`).
pub fn fn_009a1010(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    let damage = x87_store32(e.call(0x008b_e600, &args![attacker]).f64());
    e.set(this, CombatState::fDamagePerSecond, f32::from_bits(damage));
    let damage = x87_quiet_bits(e.get(this, CombatState::fDamagePerSecond).to_bits());
    let strength = x87_store32(e.call(0x008a_cbe0, &args![attacker, damage]).f64());
    e.set(this, CombatState::fCombatStrength, f32::from_bits(strength));
}

// Translated from 009a1060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::UpdateThreatRatio` (Xbox PDB; `thiscall`): unless fleeing (`cFleeState`
/// 1), resets `fThreatRatio` to 100; then, when the attacker was damaged less than 120 ago
/// (`fLastDamagedByAttackerTimeStamp`) and its +0xa4 object's slot 0xc (with 0) is not 3:
/// - the ratio becomes the group's threat to the attacker (`0098cb30`) over the group's
///   strength (`009a1260`) when the threat is at least 0 and the strength above 0;
/// - when that ratio is above 0 and `009a1240` holds for the controller's `00980070`
///   object, the attacker's combat strength from the group's other threat (`0098cc20`,
///   `008acbe0`) over the group's `004536e0` value replaces a larger ratio;
/// - while fleeing in state 2 with confidence below 4, a target near the attacker
///   (`00989700` with the setting at `011ce08c` and 1) boosts the confidence
///   (`009a1280` with 1).
pub fn combat_state_update_threat_ratio(e: &mut Engine, this: Ptr<CombatState>) {
    let attacker = attacker(e, this);
    if e.get(this, CombatState::cFleeState) == 1 {
        return;
    }
    let hundred = constant_bits(e, HUNDRED);
    e.set(this, CombatState::fThreatRatio, f32::from_bits(hundred));
    let since = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0xa0)]).f64();
    let limit: f64 = e.global(HUNDRED_TWENTY_DOUBLE);
    if !(since < limit) {
        return;
    }
    let level = e
        .vcall(attacker.wrapping_add(0xa4), 0xc, &args![0u32])
        .f64();
    let three: f64 = e.global(THREE_DOUBLE);
    if level == three {
        return;
    }
    let controller = state_controller(e, this);
    let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
    let threat = x87_store32(e.call(0x0098_cb30, &args![group, attacker]).f64());
    let strength = fn_009a1260(e, Ptr::new(group)).to_bits();
    let zero: f64 = e.global(ZERO_DOUBLE);
    if x87_load32(threat) >= zero && x87_load32(strength) > zero {
        let ratio = x87_store32(x87_div(x87_load32(threat), x87_m32(strength)));
        e.set(this, CombatState::fThreatRatio, f32::from_bits(ratio));
        let ratio = e.get(this, CombatState::fThreatRatio).to_bits();
        if x87_load32(ratio) > zero {
            let controller = state_controller(e, this);
            let source = e.call(0x0098_0070, &args![controller]).u32();
            if fn_009a1240(e, source) as u8 != 0 {
                let other = x87_store32(e.call(0x0098_cc20, &args![group, attacker]).f64());
                let divisor = x87_store32(e.call(0x0045_36e0, &args![group]).f64());
                if x87_load32(other) >= zero && x87_load32(divisor) > zero {
                    let scaled = e
                        .call(0x008a_cbe0, &args![attacker, x87_quiet_bits(other)])
                        .f64();
                    let scaled = x87_store32(scaled);
                    let candidate = x87_store32(x87_div(x87_load32(scaled), x87_m32(divisor)));
                    let current = e.get(this, CombatState::fThreatRatio).to_bits();
                    if x87_load32(current) > x87_load32(candidate) {
                        e.set(
                            this,
                            CombatState::fThreatRatio,
                            f32::from_bits(x87_quiet_bits(candidate)),
                        );
                    }
                }
            }
        }
    }
    if e.get(this, CombatState::cFleeState) == 2 && e.get(this, CombatState::cCurrentConfidence) < 4
    {
        let distance = setting_bits(e, 0x011c_e08c);
        let found = e.with_stack(0x10, |e, world| {
            let location = e.call(GET_WORLD_LOCATION, &args![attacker, world]).u32();
            e.call(0x0098_9700, &args![group, location, distance, 1u32])
                .u32()
        });
        if found != 0 {
            combat_state_boost_confidence(e, this, 1);
        }
    }
}

// Translated from 009a1240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): `009928c0(this, 4)`; `EAX` is left as that call
/// returns it (callers test `AL`).
pub fn fn_009a1240(e: &mut Engine, this: u32) -> u32 {
    e.call(0x0099_28c0, &args![this, 4u32]).u32()
}

// Translated from 009a1280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::BoostConfidence` (Xbox PDB; `thiscall`, `RET 4`): with confidence below 4,
/// raises it by one when `0047eeb0` of the confidence is below 100 and updates the
/// controller (`0097f200`). With `reset_flee`, the flee state is cleared, unless a type 1
/// location target lies within 64 units of the attacker (`00527a40` below the `double`
/// 4096). A type 12 cover location (+4) is cleared (`009a03d0` with 0).
pub fn combat_state_boost_confidence(e: &mut Engine, this: Ptr<CombatState>, reset_flee: u8) {
    if e.get(this, CombatState::cCurrentConfidence) >= 4 {
        return;
    }
    let confidence = u32::from(e.get(this, CombatState::cCurrentConfidence));
    let value = e.call(0x0047_eeb0, &args![confidence]).f64();
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    if value < hundred {
        let raised = e.get(this, CombatState::cCurrentConfidence).wrapping_add(1);
        e.set(this, CombatState::cCurrentConfidence, raised);
    }
    let controller = state_controller(e, this);
    e.call(0x0097_f200, &args![controller]);
    let mut reset = reset_flee;
    if reset != 0
        && e.get(this, CombatState::pLocationTarget) != 0
        && e.get(location_target(e, this), CombatLocationTarget::cType) == 1
    {
        let squared = e.with_stack(0x10, |e, world| {
            let controller = state_controller(e, this);
            // `CombatController::pAttacker` (Xbox PDB) +0xbc.
            let attacker = e.mem.u32(controller.wrapping_add(0xbc));
            let location = e.call(GET_WORLD_LOCATION, &args![attacker, world]).u32();
            let target_location = location_target(e, this).addr() + 8;
            e.call(0x0052_7a40, &args![target_location, location]).f64()
        });
        let limit: f64 = e.global(FOUR_THOUSAND_NINETY_SIX_DOUBLE);
        if squared < limit {
            reset = 0;
        }
    }
    if reset != 0 {
        e.set(this, CombatState::cFleeState, 0);
    }
    let cover = e.get(this, CombatState::pCoverLocation);
    if cover != 0 && e.mem.u32(cover + 4) == 0xc {
        combat_state_clear_cover_location(e, this, 0);
    }
}

// Translated from 009a1380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): with a target, whether the confidence value
/// (`0047eeb0` of `cCurrentConfidence`, less `fConfidenceModifier` when above 0) is above
/// `fThreatRatio`.
pub fn fn_009a1380(e: &mut Engine, this: Ptr<CombatState>) -> bool {
    if target(e, this) == 0 {
        return false;
    }
    let confidence = u32::from(e.get(this, CombatState::cCurrentConfidence));
    let mut value = x87_store32(e.call(0x0047_eeb0, &args![confidence]).f64());
    let zero: f64 = e.global(ZERO_DOUBLE);
    if x87_load32(value) > zero {
        let modifier = e.get(this, CombatState::fConfidenceModifier).to_bits();
        value = x87_store32(x87_sub(x87_load32(value), x87_m32(modifier)));
    }
    let ratio = e.get(this, CombatState::fThreatRatio).to_bits();
    x87_load32(value) > x87_load32(ratio)
}
// ---- Batch n025: `009a1400`, `009a2a30` to `009a3bc0` ----------------------------------

/// `TESForm`'s RTTI type descriptor (`.?AVTESForm@@`), the source type of the casts.
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// `TESObjectREFR`'s RTTI type descriptor (`.?AVTESObjectREFR@@`).
const TES_OBJECT_REFR_TYPE: u32 = 0x0118_41cc;
/// `Actor`'s RTTI type descriptor (`.?AVActor@@`).
const ACTOR_TYPE: u32 = 0x0118_46d4;
/// The vtable of the `BSSimpleArray` instance with RTTI name
/// `.?AV?$BSSimpleArray@PAVTESAmmo@@$0EAA@@` (read at the word before the vtable).
const ARRAY_VTABLE_0108B4C8: u32 = 0x0108_b4c8;
/// Save buffer: `00865e50(buffer, data, size, 0)`, `00865df0(buffer, form, 0)` (form id),
/// `00865f60(buffer, value)` (variable-sized value).
const SAVE_DATA: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const SAVE_VARIABLE_SIZE: u32 = 0x0086_5f60;
/// Load buffer: `00864980(buffer, data, size)`, `008648e0(buffer, form id pointer)`,
/// `00864a60(buffer)` (variable-sized value).
const LOAD_DATA: u32 = 0x0086_4980;
const LOAD_FORM_ID: u32 = 0x0086_48e0;
const LOAD_VARIABLE_SIZE: u32 = 0x0086_4a60;
/// `BGSWorldLocation::SaveGame` (`00527b70`) and `LoadGame` (`00527ba0`).
const WORLD_LOCATION_SAVE: u32 = 0x0052_7b70;
const WORLD_LOCATION_LOAD: u32 = 0x0052_7ba0;
/// `CombatTimeStamp::SaveGame` (`009a5f90`) and `LoadGame` (`009a5ff0`).
const TIME_STAMP_SAVE: u32 = 0x009a_5f90;
const TIME_STAMP_LOAD: u32 = 0x009a_5ff0;

/// The form a saved form pointer refers to after loading: `004839c0(word)`, then
/// `__RTDynamicCast(form, 0, TESForm, target_type, 0)` (`00ec43fb`).
fn resolve_loaded_form(e: &mut Engine, word: u32, target_type: u32) -> u32 {
    let form = e.call(0x0048_39c0, &args![word]).u32();
    e.call(
        0x00ec_43fb,
        &args![form, 0u32, TES_FORM_TYPE, target_type, 0u32],
    )
    .u32()
}

/// A loaded reference word at `slot` replaced by its `TESObjectREFR` (0 stays 0).
fn resolve_reference_at(e: &mut Engine, slot: u32) {
    let word = e.mem.u32(slot);
    let resolved = if word != 0 {
        resolve_loaded_form(e, word, TES_OBJECT_REFR_TYPE)
    } else {
        0
    };
    e.mem.set_u32(slot, resolved);
}

/// `PathManager::BuildPath` (`006eb9d0` on the manager from `0047d0b0`) for `request` with
/// the path-check queue: both `NiPointer` arguments are built in their argument slots as the
/// exe does (`006ebf30` copies `spPathCheckQueue`, `00633c90` takes `request`) and passed by
/// value, then 1.
fn build_path(e: &mut Engine, this: Ptr<CombatState>, request: u32) {
    e.with_stack(8, |e, slots| {
        let request_slot = slots.addr();
        let queue_slot = slots.addr() + 4;
        e.call(0x006e_bf30, &args![queue_slot, member(this, 0x224)]);
        e.call(0x0063_3c90, &args![request_slot, request]);
        let manager = e.call(0x0047_d0b0, &args![]).u32();
        let request_word = e.mem.u32(request_slot);
        let queue_word = e.mem.u32(queue_slot);
        e.call(0x006e_b9d0, &args![manager, request_word, queue_word, 1u32]);
    });
}

/// A new `PathingRequest` (0xb0 bytes, `00aa13e0`, `006e2420`), or 0.
fn new_pathing_request(e: &mut Engine) -> u32 {
    let block = e.call(0x00aa_13e0, &args![0xb0u32]).u32();
    if block != 0 {
        e.call(0x006e_2420, &args![block]).u32()
    } else {
        0
    }
}

/// Stack frame of the face-target case of `009a1400` (from `EBP-0x80`).
const PATH_CHECK_FRAME: u32 = 0x80;

// Translated from 009a1400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a1400 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 4`): queues a path check of `kind` to the
/// target and records it in `iQueuedPathCheck`; false without a target. The path-check
/// queue is created first when `spPathCheckQueue` is empty (`00aa13e0(0x1c)`, `008df900`,
/// `009057d0`).
/// - Kind 0: a new `PathingRequest` built for the target's world location (`008b38d0` with
///   its point `006815c0`, space `0084e3a0`, -1 and 0).
/// - Kind 1: a face-target location (`009a68b0` with the target's position, slot `+0x1f4`,
///   and -1) as a pathing location for the attacker (`006dce10`); when it is not valid for
///   the attacker's radius (`006ded70`) the check is refused (false, nothing recorded);
///   otherwise a new `PathingRequest` built for it (`008b3880` with its point `005c3420`,
///   `009611e0`, `00441110`, -1 and 0).
/// - Any other kind queues nothing.
///
/// Each request goes to `PathManager::BuildPath` with the queue (`006eb9d0`).
pub fn fn_009a1400(e: &mut Engine, this: Ptr<CombatState>, kind: u32) -> bool {
    let attacker = attacker(e, this);
    let target = target(e, this);
    if target == 0 {
        return false;
    }
    let queue = member(this, 0x224);
    if e.call(0x0055_9450, &args![queue]).u32() == 0 {
        let block = e.call(0x00aa_13e0, &args![0x1cu32]).u32();
        let created = if block != 0 {
            e.call(0x008d_f900, &args![block]).u32()
        } else {
            0
        };
        e.call(0x0090_57d0, &args![queue, created]);
    }
    match kind {
        0 => {
            let request = new_pathing_request(e);
            e.with_stack(0x10, |e, world| {
                e.call(GET_WORLD_LOCATION, &args![target, world]);
                let minus_one = constant_bits(e, MINUS_ONE);
                let space = e.call(0x0084_e3a0, &args![world]).u32();
                let point = e.call(0x0068_15c0, &args![world]).u32();
                e.call(
                    0x008b_38d0,
                    &args![attacker, request, point, space, minus_one, 0u32],
                );
            });
            build_path(e, this, request);
        }
        1 => {
            let valid = e.with_stack(PATH_CHECK_FRAME, |e, frame| {
                let local = |k: u32| frame.addr() + PATH_CHECK_FRAME - k;
                let face = local(0x48);
                let location = local(0x3c);
                let minus_one = constant_bits(e, MINUS_ONE);
                let position = e.vcall(target, 0x1f4, &args![]).u32();
                e.call(
                    0x009a_68b0,
                    &args![face, attacker, target, position, minus_one],
                );
                e.call(0x006d_ce10, &args![location, face, attacker]);
                let radius = x87_store32(e.call(0x008b_e1b0, &args![attacker]).f64());
                if !e.call(0x006d_ed70, &args![location, radius]).bool() {
                    e.call(0x004f_f7e0, &args![location]);
                    return false;
                }
                let request = new_pathing_request(e);
                let minus_one = constant_bits(e, MINUS_ONE);
                let space = e.call(0x0044_1110, &args![location]).u32();
                let cell = e.call(0x0096_11e0, &args![location]).u32();
                let point = e.call(0x005c_3420, &args![location, local(0x80)]).u32();
                e.call(
                    0x008b_3880,
                    &args![attacker, request, point, cell, space, minus_one, 0u32],
                );
                build_path(e, this, request);
                e.call(0x004f_f7e0, &args![location]);
                true
            });
            if !valid {
                return false;
            }
        }
        _ => {}
    }
    e.set(this, CombatState::iQueuedPathCheck, kind as i32);
    true
}

// Translated from 009a2a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::InitLoadGame` (Xbox PDB; `thiscall`, `RET 4`): resolves the loaded
/// pointers: the location target's reference (`009a3220`), each combat item's reference
/// (`TESObjectREFR`, then marked targeted with `00564db0(1)`), the combat location's
/// reference (`009a2cc0`) and `pLastTarget` (`Actor`); requeues a recorded path check
/// (`009a1400`), and makes `fMaxModifiedCombatWeaponDPS` the largest of itself and the
/// seven weapon DPS values (`00404010`, rounded to a float each time).
pub fn combat_state_init_load_game(e: &mut Engine, this: Ptr<CombatState>, buffer: u32) {
    let target = e.get(this, CombatState::pLocationTarget);
    if target != 0 {
        fn_009a3220(e, Ptr::new(target), buffer);
    }
    for index in 0u32..5 {
        let slot = member(this, 0x190 + 4 * index);
        if e.mem.u32(slot) == 0 {
            continue;
        }
        let item = e.mem.u32(slot);
        resolve_reference_at(e, item);
        let item = e.mem.u32(slot);
        let reference = e.mem.u32(item);
        if reference != 0 {
            e.call(0x0056_4db0, &args![reference, 1u32]);
        }
    }
    let location = e.get(this, CombatState::pCombatLocation);
    if location != 0 {
        fn_009a2cc0(e, Ptr::new(location), buffer);
    }
    let last = e.get(this, CombatState::pLastTarget);
    let resolved = if last != 0 {
        resolve_loaded_form(e, last, ACTOR_TYPE)
    } else {
        0
    };
    e.set(this, CombatState::pLastTarget, resolved);
    let queued = e.get(this, CombatState::iQueuedPathCheck);
    if queued != -1 {
        fn_009a1400(e, this, queued as u32);
    }
    for index in 0u32..7 {
        let dps = x87_quiet_bits(e.mem.u32(member(this, 0x3c + 4 * index)));
        let best = x87_quiet_bits(
            e.get(this, CombatState::fMaxModifiedCombatWeaponDPS)
                .to_bits(),
        );
        let value = e.call(0x0040_4010, &args![best, dps]).f64();
        e.set(
            this,
            CombatState::fMaxModifiedCombatWeaponDPS,
            f32::from_bits(x87_store32(value)),
        );
    }
}

// Translated from 009a2bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a `CombatLocation` (by its layout; `thiscall`, `RET 4`): the world location
/// (`00527b70`), the location, radius and flags (4 bytes each) and the reference's form id.
pub fn fn_009a2bf0(e: &mut Engine, this: Ptr<CombatLocation>, buffer: u32) {
    let me = this.addr();
    e.call(WORLD_LOCATION_SAVE, &args![me, buffer]);
    for off in [0x14u32, 0x18, 0x1c] {
        e.call(SAVE_DATA, &args![buffer, me + off, 4u32, 0u32]);
    }
    let reference = e.get(this, CombatLocation::pRef);
    e.call(SAVE_FORM_ID, &args![buffer, reference, 0u32]);
}

// Translated from 009a2c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a `CombatLocation` saved by `009a2bf0` (`thiscall`, `RET 4`); the reference stays a
/// form id until `009a2cc0`.
pub fn fn_009a2c60(e: &mut Engine, this: Ptr<CombatLocation>, buffer: u32) {
    let me = this.addr();
    e.call(WORLD_LOCATION_LOAD, &args![me, buffer]);
    for off in [0x14u32, 0x18, 0x1c] {
        e.call(LOAD_DATA, &args![buffer, me + off, 4u32]);
    }
    e.call(LOAD_FORM_ID, &args![buffer, me + 0x10]);
}

// Translated from 009a2cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves a loaded `CombatLocation`'s reference (`thiscall`, `RET 4`, the buffer word is
/// not read): the form id becomes its `TESObjectREFR` (0 stays 0).
pub fn fn_009a2cc0(e: &mut Engine, this: Ptr<CombatLocation>, _unused_1: u32) {
    resolve_reference_at(e, this.addr() + 0x10);
}

// Translated from 009a2d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves an unreachable cover location entry (`thiscall`, `RET 4`): the key (4 bytes) and
/// the time stamp at +4 (`009a5f90`).
pub fn fn_009a2d20(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(SAVE_DATA, &args![buffer, me, 4u32, 0u32]);
    e.call(TIME_STAMP_SAVE, &args![me + 4, buffer]);
}

// Translated from 009a2d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads an unreachable cover location entry saved by `009a2d20` (`thiscall`, `RET 4`).
pub fn fn_009a2d50(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(LOAD_DATA, &args![buffer, me, 4u32]);
    e.call(TIME_STAMP_LOAD, &args![me + 4, buffer]);
}

// Translated from 009a2d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves an unreachable location entry (`thiscall`, `RET 4`): the world location
/// (`00527b70`) and the time stamp at +0x10 (`009a5f90`).
pub fn fn_009a2d80(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(WORLD_LOCATION_SAVE, &args![me, buffer]);
    e.call(TIME_STAMP_SAVE, &args![me + 0x10, buffer]);
}

// Translated from 009a2db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads an unreachable location entry saved by `009a2d80` (`thiscall`, `RET 4`).
pub fn fn_009a2db0(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(WORLD_LOCATION_LOAD, &args![me, buffer]);
    e.call(TIME_STAMP_LOAD, &args![me + 0x10, buffer]);
}

// Translated from 009a2de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a `CombatCoverLocation` (`thiscall`, `RET 4`): the bytes +1, +0, +2, the word +4
/// and the point +8 (`00865e50`), the `PathingCoverLocation` at +0x14 (its virtual slot 0),
/// the key +0x68 and reservation +0x6c, and the reservation information at +0x70
/// (`00982c20`).
pub fn fn_009a2de0(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(SAVE_DATA, &args![buffer, me + 1, 1u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me, 1u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 2, 1u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 4, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 8, 0xcu32, 0u32]);
    e.vcall(me + 0x14, 0x0, &args![buffer]);
    e.call(SAVE_DATA, &args![buffer, me + 0x68, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 0x6c, 4u32, 0u32]);
    e.call(0x0098_2c20, &args![me + 0x70, buffer]);
}

// Translated from 009a2ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a `CombatCoverLocation` saved by `009a2de0` (`thiscall`, `RET 4`; the
/// `PathingCoverLocation` through its virtual slot 4, the reservation through `00982ca0`).
pub fn fn_009a2ea0(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(LOAD_DATA, &args![buffer, me + 1, 1u32]);
    e.call(LOAD_DATA, &args![buffer, me, 1u32]);
    e.call(LOAD_DATA, &args![buffer, me + 2, 1u32]);
    e.call(LOAD_DATA, &args![buffer, me + 4, 4u32]);
    e.call(LOAD_DATA, &args![buffer, me + 8, 0xcu32]);
    e.vcall(me + 0x14, 0x4, &args![buffer]);
    e.call(LOAD_DATA, &args![buffer, me + 0x68, 4u32]);
    e.call(LOAD_DATA, &args![buffer, me + 0x6c, 4u32]);
    e.call(0x0098_2ca0, &args![me + 0x70, buffer]);
}

// Translated from 009a2f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatCoverSearchInfo::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): both points (12
/// bytes each), the radius, the last-seen byte (+0x38), the key and state (`00865e50`), the
/// time stamp (`009a5f90`), the cover location count (0 when the array's address `this +
/// 0x20` is null) and each location through its virtual slot 0 (`009a3280`).
pub fn combat_cover_search_info_save_game(
    e: &mut Engine,
    this: Ptr<CombatCoverSearchInfo>,
    buffer: u32,
) {
    let me = this.addr();
    e.call(SAVE_DATA, &args![buffer, me, 0xcu32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 0xc, 0xcu32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 0x18, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 0x38, 1u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 0x30, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 0x34, 4u32, 0u32]);
    e.call(TIME_STAMP_SAVE, &args![me + 0x1c, buffer]);
    let array = me.wrapping_add(0x20);
    let count = if array == 0 {
        0
    } else {
        e.call(ARRAY_SIZE, &args![array]).u32()
    };
    e.call(SAVE_VARIABLE_SIZE, &args![buffer, count]);
    for index in 0..count {
        let location = fn_009a3280(e, Ptr::new(array), index).addr();
        e.vcall(location, 0x0, &args![buffer]);
    }
}

// Translated from 009a3050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatCoverSearchInfo::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): reads what
/// `009a2f50` saved; the array is sized to the loaded count (`009a32a0` with 1) and each
/// location loads through its virtual slot 4.
pub fn combat_cover_search_info_load_game(
    e: &mut Engine,
    this: Ptr<CombatCoverSearchInfo>,
    buffer: u32,
) {
    let me = this.addr();
    e.call(LOAD_DATA, &args![buffer, me, 0xcu32]);
    e.call(LOAD_DATA, &args![buffer, me + 0xc, 0xcu32]);
    e.call(LOAD_DATA, &args![buffer, me + 0x18, 4u32]);
    e.call(LOAD_DATA, &args![buffer, me + 0x38, 1u32]);
    e.call(LOAD_DATA, &args![buffer, me + 0x30, 4u32]);
    e.call(LOAD_DATA, &args![buffer, me + 0x34, 4u32]);
    e.call(TIME_STAMP_LOAD, &args![me + 0x1c, buffer]);
    let count = e.call(LOAD_VARIABLE_SIZE, &args![buffer]).u32();
    fn_009a32a0(e, Ptr::new(me + 0x20), count, 1);
    for index in 0..count {
        let location = fn_009a3280(e, Ptr::new(me + 0x20), index).addr();
        e.vcall(location, 0x4, &args![buffer]);
    }
}

// Translated from 009a3130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a `CombatLocationTarget` (`thiscall`, `RET 4`): type, near byte, both distances,
/// the reference's form id and the world location (`00527b70`).
pub fn fn_009a3130(e: &mut Engine, this: Ptr<CombatLocationTarget>, buffer: u32) {
    let me = this.addr();
    e.call(SAVE_DATA, &args![buffer, me, 1u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 1, 1u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 0x18, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, me + 0x1c, 4u32, 0u32]);
    let reference = e.get(this, CombatLocationTarget::pRef);
    e.call(SAVE_FORM_ID, &args![buffer, reference, 0u32]);
    e.call(WORLD_LOCATION_SAVE, &args![me + 8, buffer]);
}

// Translated from 009a31b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a `CombatLocationTarget` saved by `009a3130` (`thiscall`, `RET 4`).
pub fn fn_009a31b0(e: &mut Engine, this: Ptr<CombatLocationTarget>, buffer: u32) {
    let me = this.addr();
    e.call(LOAD_DATA, &args![buffer, me, 1u32]);
    e.call(LOAD_DATA, &args![buffer, me + 1, 1u32]);
    e.call(LOAD_DATA, &args![buffer, me + 0x18, 4u32]);
    e.call(LOAD_DATA, &args![buffer, me + 0x1c, 4u32]);
    e.call(LOAD_FORM_ID, &args![buffer, me + 4]);
    e.call(WORLD_LOCATION_LOAD, &args![me + 8, buffer]);
}

// Translated from 009a3220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves a loaded `CombatLocationTarget`'s reference (`thiscall`, `RET 4`, the buffer
/// word is not read): the form id becomes its `TESObjectREFR`.
pub fn fn_009a3220(e: &mut Engine, this: Ptr<CombatLocationTarget>, _unused_1: u32) {
    resolve_reference_at(e, this.addr() + 4);
}

/// The calls of one `BSSimpleArray<T, 1024>::SetSize` instance.
struct ArrayInstance {
    /// Empties the array (`thiscall(shrink)`).
    release: u32,
    /// Reallocates (`thiscall(size, kept)`).
    reallocate: u32,
    /// Constructs elements (`thiscall(first, count)`).
    construct: u32,
    /// Destroys elements (`thiscall(first, count)`).
    destroy: u32,
    /// The element size.
    stride: u32,
}

/// `BSSimpleArray<T, 1024>::SetSize` with `instance`'s calls (the shape of `006f2290`).
fn simple_array_set_size(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    size: u32,
    shrink: u8,
    instance: &ArrayInstance,
) {
    let element = |e: &Engine, index: u32| {
        e.get(this, BSSimpleArray::pBuffer)
            .wrapping_add(index.wrapping_mul(instance.stride))
    };
    if size == 0 {
        e.call(instance.release, &args![this, u32::from(shrink)]);
    } else if size > e.get(this, BSSimpleArray::iReservedSize) {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let buffer = e.vcall(this.addr(), 0x4, &args![size]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, size);
        } else {
            let used = e.get(this, BSSimpleArray::iSize);
            e.call(instance.reallocate, &args![this, size, used]);
            e.set(this, BSSimpleArray::iReservedSize, size);
        }
        let used = e.get(this, BSSimpleArray::iSize);
        let start = element(e, used);
        e.call(
            instance.construct,
            &args![this, start, size.wrapping_sub(used)],
        );
        e.set(this, BSSimpleArray::iSize, size);
    } else if size < e.get(this, BSSimpleArray::iSize) {
        let used = e.get(this, BSSimpleArray::iSize);
        let start = element(e, size);
        e.call(
            instance.destroy,
            &args![this, start, used.wrapping_sub(size)],
        );
        e.set(this, BSSimpleArray::iSize, size);
        if shrink != 0 && size <= e.get(this, BSSimpleArray::iReservedSize) >> 2 {
            e.call(instance.reallocate, &args![this, size, size]);
            e.set(this, BSSimpleArray::iReservedSize, size);
        }
    } else {
        let used = e.get(this, BSSimpleArray::iSize);
        let start = element(e, used);
        e.call(
            instance.construct,
            &args![this, start, size.wrapping_sub(used)],
        );
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 009a32a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingCoverLocation, 1024>::SetSize` (by its element size 0x54 and its
/// callers; `thiscall`, `RET 8`): size 0 empties the array (`006dbde0` with `shrink`);
/// growing past the reservation allocates through slot 4 (empty array) or reallocates
/// (`006dbfc0`), then constructs the new elements (`006dbe40`); shrinking destroys the
/// elements past `size` (`006dbee0`) and, with `shrink` and at most a quarter of the
/// reservation in use, reallocates to `size`.
pub fn fn_009a32a0(e: &mut Engine, this: Ptr<BSSimpleArray>, size: u32, shrink: u8) {
    let instance = ArrayInstance {
        release: 0x006d_bde0,
        reallocate: 0x006d_bfc0,
        construct: 0x006d_be40,
        destroy: 0x006d_bee0,
        stride: 0x54,
    };
    simple_array_set_size(e, this, size, shrink, &instance);
}

// Translated from 009a33e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray` instance `.?AV?$BSSimpleArray@PAVTESObjectWEAP@@$0EAA@@`
/// (`thiscall`): its vtable (`0108e068`), then `006b3eb0(this, 0, 0)`. Returns `this`.
pub fn fn_009a33e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E068);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 009a3430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray` instance
/// `.?AV?$BSSimpleArray@VUnreachableCoverLocation@@$0EAA@@` (`thiscall`): its vtable
/// (`0108e07c`), then `009a39f0(this, 0, 0)`. Returns `this`.
pub fn fn_009a3430(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E07C);
    fn_009a39f0(e, this.cast(), 0, 0);
    this
}

// Translated from 009a3480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<..., 1024>::SetSize` for 8-byte elements (the unreachable cover
/// locations; `thiscall`, `RET 8`): as `009a32a0` with `008454f0`, `006a2770`, `009a3950`
/// and `0072ba80`.
pub fn fn_009a3480(e: &mut Engine, this: Ptr<BSSimpleArray>, size: u32, shrink: u8) {
    let instance = ArrayInstance {
        release: 0x0084_54f0,
        reallocate: 0x006a_2770,
        construct: 0x009a_3950,
        destroy: 0x0072_ba80,
        stride: 8,
    };
    simple_array_set_size(e, this, size, shrink, &instance);
}

// Translated from 009a35c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray` instance
/// `.?AV?$BSSimpleArray@VUnreachableLocation@@$0EAA@@` (`thiscall`): its vtable
/// (`0108e090`), then `009a3bc0(this, 0, 0)`. Returns `this`.
pub fn fn_009a35c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E090);
    fn_009a3bc0(e, this.cast(), 0, 0);
    this
}

// Translated from 009a3610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<..., 1024>::SetSize` for 0x14-byte elements (the unreachable locations;
/// `thiscall`, `RET 8`): as `009a32a0` with `008454f0`, `009a3c50`, `009a3a80` and
/// `0072ba80`.
pub fn fn_009a3610(e: &mut Engine, this: Ptr<BSSimpleArray>, size: u32, shrink: u8) {
    let instance = ArrayInstance {
        release: 0x0084_54f0,
        reallocate: 0x009a_3c50,
        construct: 0x009a_3a80,
        destroy: 0x0072_ba80,
        stride: 0x14,
    };
    simple_array_set_size(e, this, size, shrink, &instance);
}

// Translated from 009a3750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray` instance `.?AV?$BSSimpleArray@PAVTESAmmo@@$0EAA@@`
/// (`thiscall`, `RET 8`): its vtable (`0108b4c8`), then `006b3eb0(this, reserve, size)`.
/// Returns `this`.
pub fn fn_009a3750(e: &mut Engine, this: Ptr, reserve: u32, size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108B4C8);
    e.call(0x006b_3eb0, &args![this, reserve, size]);
    this
}

// Translated from 009a3780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the `PathingCoverLocation` array `this` into `other` (`thiscall`, `RET 4`):
/// `other` is sized to `this`'s count (`009a32a0` with 1) and each element (0x54 bytes)
/// assigned to `other`'s (`009052c0`, `006e4430`).
pub fn fn_009a3780(e: &mut Engine, this: Ptr<BSSimpleArray>, other: Ptr<BSSimpleArray>) {
    let count = e.get(this, BSSimpleArray::iSize);
    fn_009a32a0(e, other, count, 1);
    let mut index = 0u32;
    while index < e.get(this, BSSimpleArray::iSize) {
        let source = index
            .wrapping_mul(0x54)
            .wrapping_add(e.get(this, BSSimpleArray::pBuffer));
        let element = e.call(ELEMENT_TARGET_009052C0, &args![other, index]).u32();
        e.call(0x006e_4430, &args![element, source]);
        index += 1;
    }
}

// Translated from 009a37e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray` instance `.?AV?$BSSimpleArray@UWeaponData@@$0EAA@@`
/// (`thiscall`, `RET 8`): its vtable (`0108e0a4`), then `009a3d00(this, reserve, size)`.
/// Returns `this`.
pub fn fn_009a37e0(e: &mut Engine, this: Ptr, reserve: u32, size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108E0A4);
    e.call(0x009a_3d00, &args![this, reserve, size]);
    this
}

/// A scalar deleting destructor: `destructor`, then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
fn scalar_deleting(e: &mut Engine, this: Ptr, flags: u32, destructor: fn(&mut Engine, Ptr)) -> Ptr {
    destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009a3850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESObjectWEAP *, 1024>::scalar deleting destructor` (Xbox PDB;
/// `thiscall`, `RET 4`): `009a3410`, then `operator delete` when bit 0 of `flags` is set.
pub fn bs_simple_array_tes_object_weap_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_009a3410)
}

// Translated from 009a3880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<UnreachableCoverLocation, 1024>::scalar deleting destructor` (Xbox PDB):
/// `009a3460`, then `operator delete` when bit 0 of `flags` is set.
pub fn bs_simple_array_unreachable_cover_location_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_009a3460)
}

// Translated from 009a38b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<UnreachableLocation, 1024>::scalar deleting destructor` (Xbox PDB):
/// `009a35f0`, then `operator delete` when bit 0 of `flags` is set.
pub fn bs_simple_array_unreachable_location_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_009a35f0)
}

// Translated from 009a38e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<WeaponData, 1024>::scalar deleting destructor` (Xbox PDB): `009a3810`,
/// then `operator delete` when bit 0 of `flags` is set.
pub fn bs_simple_array_weapon_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_009a3810)
}

/// The body of `009a39f0` and `009a3bc0`: empty, then a buffer of `reserve` (at least
/// `size`) elements from slot 4, and `size` elements constructed by `construct`.
fn simple_array_initialize(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    reserve: u32,
    size: u32,
    construct: u32,
) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let reserve = if size > reserve { size } else { reserve };
    if reserve != 0 {
        let buffer = e.vcall(this.addr(), 0x4, &args![reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(construct, &args![this, buffer, size]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 009a39f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the unreachable cover location array (`thiscall`, `RET 8`): no buffer, then
/// `reserve` (raised to `size`) elements from slot 4 and `size` of them constructed
/// (`009a3950`).
pub fn fn_009a39f0(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    simple_array_initialize(e, this, reserve, size, 0x009a_3950);
}

// Translated from 009a3bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the unreachable location array (`thiscall`, `RET 8`): as `009a39f0` with
/// `009a3a80`.
pub fn fn_009a3bc0(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    simple_array_initialize(e, this, reserve, size, 0x009a_3a80);
}

// Translated from 009d6c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::UnreserveCoverLocation` (Xbox PDB; `thiscall`): `009a05b0(this,
/// pSecondaryCoverLocation)`.
pub fn combat_state_unreserve_cover_location(e: &mut Engine, this: Ptr<CombatState>) {
    let cover = e.get(this, CombatState::pSecondaryCoverLocation);
    e.call(0x009a_05b0, &args![this, cover]);
}

// Translated from 009d7c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::ReserveCoverLocation` (Xbox PDB; `thiscall`): `009a0510(this,
/// pSecondaryCoverLocation)`.
pub fn combat_state_reserve_cover_location(e: &mut Engine, this: Ptr<CombatState>) {
    let cover = e.get(this, CombatState::pSecondaryCoverLocation);
    e.call(0x009a_0510, &args![this, cover]);
}

// ---- Batch n025: `0099d480` --------------------------------------------------------------

/// Stack frame of `0099d480` (from `EBP-0x300`); a local at `EBP - k` is at
/// `frame + COVER_SEARCH_FRAME_D480 - k`.
const COVER_SEARCH_FRAME_D480: u32 = 0x300;

/// What `0099d480` records in byte +0x52 of a rejected (or the chosen) cover location.
mod cover_verdict {
    /// Already in the request's unreachable cover locations (`00719b20`).
    pub const UNREACHABLE: u8 = 3;
    /// Not a valid location for the request's radius (`006ded70`).
    pub const INVALID: u8 = 4;
    /// Farther from the goal than the path budget.
    pub const TOO_FAR_FROM_GOAL: u8 = 7;
    /// Within the search radius of the start.
    pub const TOO_NEAR_START: u8 = 8;
    /// Refused by the combat manager's cover map (`009835d0`).
    pub const RESERVED: u8 = 9;
    /// Within 128 units of the request's preferred point.
    pub const TOO_NEAR_PREFERRED: u8 = 0xa;
    /// Threatened on the way (`009a5670` above 0).
    pub const THREATENED: u8 = 0xb;
    /// Exposed (`0099cb00` / `0099cc60`).
    pub const EXPOSED: u8 = 0xd;
    /// The path to the goal fails or is longer than the budget.
    pub const GOAL_PATH: u8 = 0x11;
    /// The path from the start fails or both paths together are too long.
    pub const START_PATH: u8 = 0x12;
    /// Chosen.
    pub const CHOSEN: u8 = 0x13;
}

// Translated from 0099d480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099d480 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map (`cdecl`, three words): the covered-move search of the
/// `PathingRequestCoveredMove` at `request`; true when a cover location was chosen.
/// - The request's start (`0048d150`) and goal (`006a9540`) locations must be at least
///   `radius` apart (squared, `004a7290` of the `00439ef0` difference of their points
///   `005c3420`), else false.
/// - The path length from start to goal (`006d5490` with `0069e390`) sets the budgets: the
///   goal distance `length - radius / 2` and the total `radius * f + length` (`f` 3 for
///   kind 6, `007058c0`, else 1); the search radius is 2048 capped by the length and by half
///   the start-goal distance (`00457910`), and the search centre lies at 0.9 times that
///   radius from the start towards the goal (`0045bb20`, `00439e90`).
/// - A `PathingRequestCover` (`006e4080`) is built from a pathing location at the centre
///   (`006dd0d0`), the start's point (`0099e010`), the goal point (or the request's
///   preferred point `00923000` when `008d8660` says there is one), 1024, the search radius,
///   200, the request's `0048ee50` and `0099e040` values (and whether the latter is
///   non-zero), with `006e43e0(1)` and, for a preferred point, `006e4360(1)`. The request's
///   cover location (`0068efb0`) gets the kind and goal point, its search information
///   (`004f8960`) the stamp of now, the centre, the radius and the goal point, and its cover
///   location array is emptied (`006dbde0(0)`) and filled (`006d62e0`).
/// - Unless `keep_reservations` is set, the cover map's reservations are cleared before
///   (`0099c210`) and, when nothing is chosen, refreshed after (`00995890`).
/// - Each candidate (`009052c0`) is rejected with its reason in byte +0x52
///   ([`cover_verdict`]) or chosen: its key (`006e4570`), the reservation (`006d3280`,
///   `009835d0` with the request's radius `0069ef80`), the adjusted point (`009a93e0`), a
///   pathing location for it (`006dd0d0`) that must be valid (`006ded70` with `006d2c20`),
///   the distances to goal and start, the preferred point, the threat on the way
///   (`009a5670` with the travel time `00457990 / 006d2c40` and `006d6d90`), exposure
///   (`0099cb00` with the request's group points `006e4fb0`, or `0099cc60` to the goal), and
///   both paths (`006d5490`) within the budgets.
/// - The chosen candidate fills the cover location: the `PathingCoverLocation` (+0x14,
///   `006e4430`), key, reservation, the byte +2 (`0099c1f0`), `005407b0(0)`, the ledge
///   flag +0x75 when the first path step (`006dad90(1)`, `006a6eb0`) passes `006e4840`, and
///   the request's result point (`006d30b0`).
pub fn fn_0099d480(e: &mut Engine, request: u32, radius: f32, keep_reservations: u8) -> bool {
    e.with_stack(COVER_SEARCH_FRAME_D480, |e, frame| {
        let local = |k: u32| frame.addr() + COVER_SEARCH_FRAME_D480 - k;
        let r = radius.to_bits();
        let cap_init = constant_bits(e, THOUSAND_TWENTY_FOUR);
        e.mem.set_u32(local(0x30), cap_init);
        let mut cap = constant_bits(e, TWO_THOUSAND_FORTY_EIGHT);
        e.mem.set_u32(local(0x4c), 200);
        let start = e.call(0x0048_d150, &args![request]).u32();
        let goal = e.call(0x006a_9540, &args![request]).u32();
        let pathing = e.call(0x0069_e390, &args![request]).u32();
        let cover = e.call(0x0068_efb0, &args![request]).u32();
        let search_info = e.call(0x004f_8960, &args![request]).u32();
        let difference = local(0x2c);
        let start_point = e.call(0x005c_3420, &args![start, local(0x28c)]).u32();
        let goal_point = e.call(0x005c_3420, &args![goal, local(0x280)]).u32();
        e.call(0x0043_9ef0, &args![goal_point, difference, start_point]);
        let squared = x87_store32(e.call(0x004a_7290, &args![difference]).f64());
        if x87_load32(squared) < x87_mul(x87_load32(r), x87_m32(r)) {
            return false;
        }
        let length_slot = local(0x5c);
        e.mem.set_u32(length_slot, 0);
        let path_ok = e
            .call(0x006d_5490, &args![start, goal, pathing, length_slot, 0u32])
            .u8();
        let mut factor = 1.0f32.to_bits();
        if e.call(0x0070_58c0, &args![request]).u32() == 6 {
            factor = constant_bits(e, THREE);
        }
        let two: f64 = e.global(TWO_DOUBLE);
        let length = e.mem.u32(length_slot);
        let goal_budget = x87_store32(x87_sub(x87_m32(length), x87_div(x87_load32(r), two)));
        let total_budget = x87_store32(x87_add(
            x87_mul(x87_load32(r), x87_m32(factor)),
            x87_m32(length),
        ));
        if x87_load32(cap) > x87_load32(length) {
            cap = x87_quiet_bits(length);
        }
        let distance = x87_store32(e.call(0x0045_7910, &args![difference]).f64());
        let half = x87_store32(x87_div(x87_load32(distance), two));
        if x87_load32(cap) > x87_load32(half) {
            cap = x87_quiet_bits(half);
        }
        let nine_tenths: f64 = e.global(NINE_TENTHS_DOUBLE);
        let reach = x87_store32(x87_mul(x87_load32(cap), nine_tenths));
        let scaled = e
            .call(
                0x0045_bb20,
                &args![difference, local(0x2a4), x87_quiet_bits(reach)],
            )
            .u32();
        let centre = local(0x18);
        let from = e.call(0x005c_3420, &args![start, local(0x298)]).u32();
        e.call(0x0043_9e90, &args![from, centre, scaled]);
        let centre_location = local(0x84);
        e.call(0x006d_d0d0, &args![centre_location, centre, start]);
        let goal_point = local(0x44);
        e.call(0x005c_3420, &args![goal, goal_point]);
        let mut preferred = 0u8;
        if e.call(0x008d_8660, &args![request]).bool() {
            let source = e.call(0x0092_3000, &args![request]).u32();
            copy_words(e, goal_point, source, 3);
            preferred = 1;
        }
        let cover_request = local(0x17c);
        e.call(0x006e_4080, &args![cover_request]);
        e.call(0x006d_3ac0, &args![cover_request, centre_location]);
        let start_point = e.call(0x005c_3420, &args![start, local(0x2b0)]).u32();
        fn_0099e010(e, Ptr::new(cover_request), Ptr::new(start_point));
        e.call(0x006d_ad40, &args![cover_request, goal_point]);
        let inner = x87_quiet_bits(e.mem.u32(local(0x30)));
        e.call(0x006e_4340, &args![cover_request, inner]);
        e.call(0x006e_4320, &args![cover_request, x87_quiet_bits(cap)]);
        e.call(0x0040_30d0, &args![cover_request, 0xc8u32]);
        let value = x87_store32(e.call(0x0048_ee50, &args![request]).f64());
        e.call(0x0051_b840, &args![cover_request, value]);
        let value = fn_0099e040(e, Ptr::new(request)).to_bits();
        e.call(0x0050_76c0, &args![cover_request, value]);
        let value = f64::from(fn_0099e040(e, Ptr::new(request)));
        let zero: f64 = e.global(ZERO_DOUBLE);
        let non_zero = u32::from(value != zero);
        e.call(0x008e_eaa0, &args![cover_request, non_zero]);
        e.call(0x006e_43e0, &args![cover_request, 1u32]);
        if preferred != 0 {
            e.call(0x006e_4360, &args![cover_request, 1u32]);
        }
        let kind = e.call(0x0070_58c0, &args![request]).u32();
        e.mem.set_u32(cover + 4, kind);
        copy_words(e, cover + 8, goal_point, 3);
        let now = game_time_bits(e);
        let stamp = local(0x2b4);
        e.call(TIME_STAMP_SET, &args![stamp, now]);
        let stamp = e.mem.u32(stamp);
        e.mem.set_u32(search_info + 0x1c, stamp);
        copy_words(e, search_info + 0xc, centre, 3);
        e.mem.set_u32(search_info + 0x18, x87_quiet_bits(cap));
        copy_words(e, search_info, cover + 8, 3);
        e.call(0x006d_bde0, &args![search_info + 0x20, 0u32]);
        let filled = e
            .call(0x006d_62e0, &args![cover_request, search_info + 0x20])
            .u32();
        if filled != 0 {
            if keep_reservations == 0 {
                let manager: u32 = e.global(COMBAT_MANAGER);
                let cover_map = e.call(0x0096_11e0, &args![manager]).u32();
                fn_0099c210(e, cover_map);
            }
            let check = CandidateCheck {
                request,
                start,
                goal,
                pathing,
                cover,
                radius: r,
                goal_budget,
                total_budget,
                path_ok,
                preferred,
            };
            let mut index = 0u32;
            while index < e.call(ARRAY_SIZE, &args![search_info + 0x20]).u32() {
                let candidate = e
                    .call(ELEMENT_TARGET_009052C0, &args![search_info + 0x20, index])
                    .u32();
                if check_cover_candidate(e, &check, frame, candidate) {
                    e.call(0x006d_ad70, &args![cover_request]);
                    e.call(0x004f_f7e0, &args![centre_location]);
                    return true;
                }
                index += 1;
            }
            if keep_reservations == 0 {
                let manager: u32 = e.global(COMBAT_MANAGER);
                let cover_map = e.call(0x0096_11e0, &args![manager]).u32();
                e.call(0x0099_5890, &args![cover_map]);
            }
        }
        e.call(0x006d_ad70, &args![cover_request]);
        e.call(0x004f_f7e0, &args![centre_location]);
        false
    })
}

/// What the candidate loop of `0099d480` reads.
struct CandidateCheck {
    request: u32,
    start: u32,
    goal: u32,
    pathing: u32,
    cover: u32,
    radius: u32,
    goal_budget: u32,
    total_budget: u32,
    path_ok: u8,
    preferred: u8,
}

/// One candidate of `0099d480` (the loop body): its verdict in byte +0x52; true when it is
/// chosen (the cover location is then filled).
fn check_cover_candidate(
    e: &mut Engine,
    check: &CandidateCheck,
    frame: Ptr,
    candidate: u32,
) -> bool {
    let local = |k: u32| frame.addr() + COVER_SEARCH_FRAME_D480 - k;
    let verdict = candidate + 0x52;
    let key_slot = local(0x1b4);
    let key = e.call(0x006e_4570, &args![candidate]).u32();
    e.mem.set_u32(key_slot, key);
    let unreachable = e.call(0x006e_4f90, &args![check.request]).u32();
    if e.call(
        0x0071_9b20,
        &args![unreachable, key_slot, 0u32, 0x009a_3830u32],
    )
    .i32()
        != -1
    {
        e.mem.set_u8(verdict, cover_verdict::UNREACHABLE);
        return false;
    }
    let reservation = local(0x1a0);
    e.call(0x006d_3280, &args![reservation]);
    let size = x87_store32(e.call(0x0069_ef80, &args![check.request]).f64());
    let manager: u32 = e.global(COMBAT_MANAGER);
    let cover_map = e.call(0x0096_11e0, &args![manager]).u32();
    if !e
        .call(
            0x0098_35d0,
            &args![
                cover_map,
                key,
                candidate,
                0u32,
                size,
                reservation,
                1u32,
                0u32
            ],
        )
        .bool()
    {
        e.mem.set_u8(verdict, cover_verdict::RESERVED);
        return false;
    }
    let point = local(0x1d0);
    e.call(0x005c_3420, &args![candidate, point]);
    let size = x87_store32(e.call(0x0069_ef80, &args![check.request]).f64());
    e.call(
        0x009a_93e0,
        &args![size, candidate, reservation, point, 0u32],
    );
    let location = local(0x20c);
    e.call(0x006d_d0d0, &args![location, point, candidate]);
    let step = x87_store32(e.call(0x006d_2c20, &args![check.request]).f64());
    if !e.call(0x006d_ed70, &args![location, step]).bool() {
        e.mem.set_u8(verdict, cover_verdict::INVALID);
        e.call(0x004f_f7e0, &args![location]);
        return false;
    }
    let goal_point = e.call(0x005c_3420, &args![check.goal, local(0x2c0)]).u32();
    let to_goal = local(0x1c4);
    e.call(0x0043_9ef0, &args![point, to_goal, goal_point]);
    let squared = e.call(0x004a_7290, &args![to_goal]).f64();
    let budget = check.goal_budget;
    if x87_mul(x87_load32(budget), x87_m32(budget)) < squared {
        e.mem.set_u8(verdict, cover_verdict::TOO_FAR_FROM_GOAL);
        e.call(0x004f_f7e0, &args![location]);
        return false;
    }
    let start_point = e.call(0x005c_3420, &args![check.start, local(0x2cc)]).u32();
    let from_start = local(0x1b0);
    e.call(0x0043_9ef0, &args![start_point, from_start, point]);
    let squared = e.call(0x004a_7290, &args![from_start]).f64();
    let r = check.radius;
    if x87_mul(x87_load32(r), x87_m32(r)) > squared {
        e.mem.set_u8(verdict, cover_verdict::TOO_NEAR_START);
        e.call(0x004f_f7e0, &args![location]);
        return false;
    }
    if check.preferred != 0 {
        let offset = local(0x220);
        e.call(0x0043_9ef0, &args![point, offset, check.cover + 8]);
        let squared = e.call(0x004a_7290, &args![offset]).f64();
        let limit: f64 = e.global(COVER_MIN_DISTANCE_SQUARED_DOUBLE);
        if squared < limit {
            e.mem.set_u8(verdict, cover_verdict::TOO_NEAR_PREFERRED);
            e.call(0x004f_f7e0, &args![location]);
            return false;
        }
    }
    let distance = e.call(0x0045_7990, &args![from_start]).f64();
    let speed = e.call(0x006d_2c40, &args![check.request]).f64();
    let travel = x87_store32(x87_div(distance, speed));
    let last = x87_store32(e.call(0x006d_6d90, &args![check.request]).f64());
    let stamp = time_stamp_word(e, 0.0f32.to_bits());
    let (x, y, z) = (e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8));
    let manager: u32 = e.global(COMBAT_MANAGER);
    let threat_map = e.call(0x0082_5c00, &args![manager]).u32();
    let threat = e
        .call(
            0x009a_5670,
            &args![
                threat_map,
                x,
                y,
                z,
                0u32,
                x87_quiet_bits(travel),
                0.0f32.to_bits(),
                stamp,
                last
            ],
        )
        .f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    if threat > zero {
        e.mem.set_u8(verdict, cover_verdict::THREATENED);
        e.call(0x004f_f7e0, &args![location]);
        return false;
    }
    let points = e.call(0x006e_4fb0, &args![check.request]).u32();
    let exposed = if e.call(ARRAY_SIZE, &args![points]).u32() != 0 {
        let points = e.call(0x006e_4fb0, &args![check.request]).u32();
        fn_0099cb00(e, location, points)
    } else {
        let goal_point = e.call(0x005c_3420, &args![check.goal, local(0x2dc)]).u32();
        fn_0099cc60(e, location, goal_point, 0)
    };
    if exposed {
        e.mem.set_u8(verdict, cover_verdict::EXPOSED);
        e.call(0x004f_f7e0, &args![location]);
        return false;
    }
    let to_goal_length = local(0x194);
    e.mem.set_u32(to_goal_length, 0);
    let steps = local(0x1e4);
    e.call(0x006a_bbb0, &args![steps]);
    let goal_ok = e
        .call(
            0x006d_5490,
            &args![location, check.goal, check.pathing, to_goal_length, steps],
        )
        .u8();
    let goal_length = e.mem.u32(to_goal_length);
    if (check.path_ok != 0 && goal_ok == 0)
        || x87_load32(check.goal_budget) < x87_load32(goal_length)
    {
        e.mem.set_u8(verdict, cover_verdict::GOAL_PATH);
        e.call(0x006a_bc50, &args![steps]);
        e.call(0x004f_f7e0, &args![location]);
        return false;
    }
    let from_start_length = local(0x210);
    e.mem.set_u32(from_start_length, 0);
    let start_ok = e
        .call(
            0x006d_5490,
            &args![
                check.start,
                location,
                check.pathing,
                from_start_length,
                0u32
            ],
        )
        .u8();
    let sum = x87_add(
        x87_load32(e.mem.u32(to_goal_length)),
        x87_m32(e.mem.u32(from_start_length)),
    );
    if (check.path_ok != 0 && start_ok == 0) || x87_load32(check.total_budget) < sum {
        e.mem.set_u8(verdict, cover_verdict::START_PATH);
        e.call(0x006a_bc50, &args![steps]);
        e.call(0x004f_f7e0, &args![location]);
        return false;
    }
    e.mem.set_u8(verdict, cover_verdict::CHOSEN);
    let cover = check.cover;
    e.call(0x006e_4430, &args![cover + 0x14, candidate]);
    e.mem.set_u32(cover + 0x68, e.mem.u32(key_slot));
    let reserved = e.call(0x006e_4740, &args![cover + 0x14]).u32();
    e.mem.set_u32(cover + 0x6c, reserved);
    let word = e.mem.u32(reservation);
    e.mem.set_u32(cover + 0x70, word);
    let half = e.mem.u16(reservation + 4);
    e.mem.set_u16(cover + 0x74, half);
    let byte = fn_0099c1f0(e, cover + 0x14);
    e.mem.set_u8(cover + 2, byte);
    e.call(0x0054_07b0, &args![cover, 0u32]);
    if check.preferred == 0 {
        if e.call(ARRAY_SIZE, &args![steps]).u32() > 1 {
            let next = local(0x22c);
            let step = e.call(0x006d_ad90, &args![steps, 1u32]).u32();
            e.call(0x006a_6eb0, &args![step, next]);
            if e.call(0x006e_4840, &args![cover + 0x14, next]).bool() {
                e.mem.set_u8(cover + 0x75, 1);
            }
        }
    } else {
        let aim = local(0x24c);
        copy_words(e, aim, local(0x44), 3);
        let preferred_steps = local(0x240);
        e.call(0x006a_bbb0, &args![preferred_steps]);
        let preferred_location = local(0x274);
        e.call(
            0x006d_d0d0,
            &args![preferred_location, local(0x44), check.start],
        );
        if e.call(
            0x006d_5490,
            &args![
                location,
                preferred_location,
                check.pathing,
                0u32,
                preferred_steps
            ],
        )
        .bool()
            && e.call(ARRAY_SIZE, &args![preferred_steps]).u32() > 1
        {
            let step = e.call(0x006d_ad90, &args![preferred_steps, 1u32]).u32();
            let source = e.call(0x006a_6eb0, &args![step, local(0x2e8)]).u32();
            copy_words(e, aim, source, 3);
        }
        if e.call(0x006e_4840, &args![cover + 0x14, aim]).bool() {
            e.mem.set_u8(cover + 0x75, 1);
        }
        e.call(0x004f_f7e0, &args![preferred_location]);
        e.call(0x006a_bc50, &args![preferred_steps]);
    }
    e.call(0x006d_30b0, &args![check.request, point]);
    e.call(0x006a_bc50, &args![steps]);
    e.call(0x004f_f7e0, &args![location]);
    true
}

// ---- Batch n025: `0099e190` --------------------------------------------------------------

/// Stack frame of `0099e190` (from `EBP-0x198`).
const DODGE_FRAME: u32 = 0x198;
/// `8.0` (`double`).
const EIGHT_DOUBLE: u32 = 0x0102_40b0;
/// The up vector (0, 0, 1) at `011a9484` (three floats in the exe's data).
const UP_VECTOR: u32 = 0x011a_9484;

// Translated from 0099e190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099e190 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `CombatState::FindBestDodgeLocation` (Xbox PDB; `thiscall`, `RET 0x14`): tries up to four
/// dodge directions from the attacker and puts one into `out` (`0099b8b0`, space 0); false
/// when none works.
/// - The directions come from the unit vector away from the target (`00439ef0`,
///   `004a0c10`) and its cross product with up (`004b3800` with `011a9484`): bit 0 the cross
///   product, bit 1 its negation (`004a0bd0`), bit 2 the negated away vector, bit 3 the
///   away vector (the exe's jump table at `0099e820`, here a `match`); only the bits set in
///   `directions` are tried.
/// - A direction's point is the attacker's position plus `distance` along it; with the
///   attacker's radius plus 8 further on as the probe. A point threatened by the combat
///   manager's threat map (`009a5670`) is skipped. When the probe is blocked
///   (`Pathing::CheckLineOfSight` `006d7490` false) the point is pulled back to the hit
///   (`005c3420`) less that margin, provided both `distance` and that pulled-back length
///   exceed `minimum`.
/// - Directions in `check_los` also need a line of fire from the point to the target
///   (`CombatUtilities::CalculateLOS` `009a7940` with 0, -1); every point must pass
///   `009a0260` (with the attacker's space, 1, 0).
/// - The previous direction (`cLastDodgeDirection`) is kept while it is less than 10 old
///   (`fLastDodgeSearch`) and still possible; otherwise a random one of the possible ones
///   (`005a00a0`) is chosen and the time stamped.
pub fn combat_state_find_best_dodge_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    out: Ptr<CombatLocation>,
    directions: u32,
    check_los: u32,
    distance: f32,
    minimum: f32,
) -> bool {
    e.with_stack(DODGE_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + DODGE_FRAME - k;
        let attacker = attacker(e, this);
        let target = target(e, this);
        let eight: f64 = e.global(EIGHT_DOUBLE);
        let margin = x87_store32(x87_add(e.call(0x008b_e1b0, &args![attacker]).f64(), eight));
        let world = local(0xb4);
        e.call(GET_WORLD_LOCATION, &args![attacker, world]);
        let position = local(0x98);
        let source = e.call(0x0068_15c0, &args![world]).u32();
        copy_words(e, position, source, 3);
        let target_position = local(0x50);
        let source = e.vcall(target, 0x1f4, &args![]).u32();
        copy_words(e, target_position, source, 3);
        let from = local(0x7c);
        e.call(0x006d_cd70, &args![from, attacker]);
        let away = local(0xa4);
        e.call(0x0043_9ef0, &args![position, away, target_position]);
        e.call(0x004a_0c10, &args![away]);
        let side = local(0x8c);
        e.call(0x004b_3800, &args![away, side, UP_VECTOR]);
        let points = local(0x44);
        // `vector constructor iterator` (stdcall): four 12-byte points built by `006815c0`.
        e.call(0x0040_1050, &args![points, 0xcu32, 4u32, 0x0068_15c0u32]);
        let slot_of = local(0xc0);
        let direction_of = local(0x54);
        let mut count = 0u32;
        for index in 0u32..4 {
            e.mem.set_u8(slot_of + index, 0xff);
            let bit = 1u32 << index;
            if directions & bit == 0 {
                continue;
            }
            let direction = local(0xd4);
            e.call(0x0068_15c0, &args![direction]);
            match bit {
                1 => copy_words(e, direction, side, 3),
                2 => {
                    let source = e.call(0x004a_0bd0, &args![side, local(0x130)]).u32();
                    copy_words(e, direction, source, 3);
                }
                4 => {
                    let source = e.call(0x004a_0bd0, &args![away, local(0x13c)]).u32();
                    copy_words(e, direction, source, 3);
                }
                8 => copy_words(e, direction, away, 3),
                _ => {}
            }
            let point = local(0x114);
            let step = e
                .call(
                    0x0045_bb20,
                    &args![direction, local(0x148), x87_quiet_bits(distance.to_bits())],
                )
                .u32();
            e.call(0x0043_9e90, &args![position, point, step]);
            let probe = local(0x108);
            let step = e
                .call(
                    0x0045_bb20,
                    &args![direction, local(0x154), x87_quiet_bits(margin)],
                )
                .u32();
            e.call(0x0043_9e90, &args![point, probe, step]);
            if threat_at(e, point, attacker, 0.0) {
                continue;
            }
            let hit = local(0xfc);
            e.call(0x006d_cbb0, &args![hit]);
            if !e.call(0x006d_7490, &args![from, probe, hit, 0u32]).bool() {
                let far_enough = x87_load32(distance.to_bits()) > x87_load32(minimum.to_bits());
                if !far_enough {
                    e.call(0x004f_f7e0, &args![hit]);
                    continue;
                }
                let hit_point = e.call(0x005c_3420, &args![hit, local(0x164)]).u32();
                let offset = local(0x120);
                e.call(0x0043_9ef0, &args![position, offset, hit_point]);
                let length = x87_store32(e.call(0x0058_9850, &args![offset]).f64());
                let room = x87_sub(x87_load32(length), x87_m32(margin));
                if x87_load32(minimum.to_bits()) > room {
                    e.call(0x004f_f7e0, &args![hit]);
                    continue;
                }
                let room = x87_store32(x87_sub(x87_load32(length), x87_m32(margin)));
                let step = e
                    .call(
                        0x0045_bb20,
                        &args![direction, local(0x170), x87_quiet_bits(room)],
                    )
                    .u32();
                let source = e
                    .call(0x0043_9e90, &args![position, local(0x17c), step])
                    .u32();
                copy_words(e, point, source, 3);
            }
            if check_los & bit != 0
                && !e
                    .call(
                        0x009a_7940,
                        &args![attacker, target, point, target_position, 0u32, u32::MAX],
                    )
                    .bool()
            {
                e.call(0x004f_f7e0, &args![hit]);
                continue;
            }
            let space = e.call(0x0084_e3a0, &args![world]).u32();
            if !e
                .call(0x009a_0260, &args![this, point, space, 1u32, 0u32])
                .bool()
            {
                e.call(0x004f_f7e0, &args![hit]);
                continue;
            }
            copy_words(e, points + count.wrapping_mul(0xc), point, 3);
            e.mem.set_u8(slot_of + index, count as u8);
            e.mem.set_u8(direction_of.wrapping_add(count), index as u8);
            count += 1;
            e.call(0x004f_f7e0, &args![hit]);
        }
        if count == 0 {
            e.call(0x004f_f7e0, &args![from]);
            return false;
        }
        let slot_point = |e: &Engine, direction: i8| {
            let slot = e.mem.u8(slot_of.wrapping_add(direction as i32 as u32)) as i8;
            points.wrapping_add((slot as i32 as u32).wrapping_mul(0xc))
        };
        let last = e.get(this, CombatState::cLastDodgeDirection) as i8;
        let keep = last != -1 && {
            let age = e
                .call(TIME_STAMP_ELAPSED, &args![member(this, 0x184)])
                .f64();
            let limit: f64 = e.global(TEN_DOUBLE);
            age < limit && {
                let last = e.get(this, CombatState::cLastDodgeDirection) as i8;
                e.mem.u8(slot_of.wrapping_add(last as i32 as u32)) as i8 != -1
            }
        };
        if !keep {
            let pick = e.call(0x005a_00a0, &args![count]).u32();
            let chosen = e.mem.u8(direction_of.wrapping_add(pick));
            e.set(this, CombatState::cLastDodgeDirection, chosen);
            let now = game_time_bits(e);
            let stamp = local(0x184);
            e.call(TIME_STAMP_SET, &args![stamp, now]);
            let stamp = e.mem.u32(stamp);
            e.set(this, CombatState::fLastDodgeSearch, stamp);
        }
        let last = e.get(this, CombatState::cLastDodgeDirection) as i8;
        let point = slot_point(e, last);
        fn_0099b8b0(e, out, point, 0);
        e.call(0x004f_f7e0, &args![from]);
        true
    })
}

// ---- Batch n025: `0099e840`, `0099ebe0` ------------------------------------------------

/// Stack frame of `0099e840` (from `EBP-0x118`).
const RANGE_LOCATION_FRAME: u32 = 0x118;
/// Stack frame of `0099ebe0` (from `EBP-0x138`).
const FLEE_FRAME: u32 = 0x138;
/// The pointer at `011c95c8` whose `00461110` gives the actor list `0099ebe0` walks.
const ACTOR_LIST_OWNER: u32 = 0x011c_95c8;
/// The object at `011f1840` whose `00408d60` gives the byte switch for the encounter-zone
/// check of `0099ebe0`.
const ENCOUNTER_ZONE_SWITCH_011F1840: u32 = 0x011f_1840;

// Translated from 0099e840 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099e840 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0x18`): finds a location between `near` and
/// `far` from the target and puts it into `out` (`0099b8b0`, space 0).
/// - The goal is the target's position less `(near + far) / 2` along the unit vector from
///   the attacker to the target (`00439ef0`, `00457910`, `0045bb20`); the tolerance is
///   `|(near + far) / 2 - distance|` (`00408840`).
/// - The closest navmesh point to the goal (`006d6f80` with the attacker location's
///   worldspace `00441110` and cell `006dd4f0`) must lie within the tolerance of it; then the
///   closest location reachable from the attacker within twice the tolerance
///   (`006d7a40`) is taken, and its squared distance to the target must lie between
///   `minimum` and `maximum` squared.
/// - With `check_los`, the location also needs a line of fire to the target
///   (`CombatUtilities::CalculateLOS` `009a7940` with 0, -1).
///
/// The pathing locations are destroyed (`004ff7e0`) on every exit.
#[allow(clippy::too_many_arguments)]
pub fn fn_0099e840(
    e: &mut Engine,
    this: Ptr<CombatState>,
    near: f32,
    far: f32,
    minimum: f32,
    maximum: f32,
    out: Ptr<CombatLocation>,
    check_los: u8,
) -> bool {
    e.with_stack(RANGE_LOCATION_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + RANGE_LOCATION_FRAME - k;
        let attacker = attacker(e, this);
        let target = target(e, this);
        let two: f64 = e.global(TWO_DOUBLE);
        let middle = x87_store32(x87_div(
            x87_add(x87_load32(near.to_bits()), x87_m32(far.to_bits())),
            two,
        ));
        let position = local(0x78);
        let source = e.vcall(attacker, 0x1f4, &args![]).u32();
        copy_words(e, position, source, 3);
        let target_position = local(0x44);
        let source = e.vcall(target, 0x1f4, &args![]).u32();
        copy_words(e, target_position, source, 3);
        let direction = local(0xb8);
        e.call(0x0043_9ef0, &args![target_position, direction, position]);
        let distance = x87_store32(e.call(0x0045_7910, &args![direction]).f64());
        let step = e
            .call(
                0x0045_bb20,
                &args![direction, local(0xfc), x87_quiet_bits(middle)],
            )
            .u32();
        let goal = local(0xac);
        e.call(0x0043_9ef0, &args![target_position, goal, step]);
        let offset = x87_store32(x87_sub(x87_load32(middle), x87_m32(distance)));
        let tolerance = x87_store32(e.call(0x0040_8840, &args![x87_quiet_bits(offset)]).f64());
        let from = local(0xa0);
        e.call(0x006d_cd70, &args![from, attacker]);
        let closest = local(0x2c);
        e.call(0x0068_15c0, &args![closest]);
        let cell = e.call(0x006d_d4f0, &args![from]).u32();
        let space = e.call(0x0044_1110, &args![from]).u32();
        if !e
            .call(0x006d_6f80, &args![space, cell, goal, closest])
            .bool()
        {
            e.call(0x004f_f7e0, &args![from]);
            return false;
        }
        let gap = local(0x20);
        e.call(0x0043_9ef0, &args![closest, gap, goal]);
        let squared = e.call(0x004a_7290, &args![gap]).f64();
        if x87_mul(x87_load32(tolerance), x87_m32(tolerance)) < squared {
            e.call(0x004f_f7e0, &args![from]);
            return false;
        }
        let to = local(0xf0);
        e.call(0x006d_ce10, &args![to, closest, attacker]);
        let reach = x87_store32(x87_add(x87_load32(tolerance), x87_load32(tolerance)));
        let reached = local(0x6c);
        e.call(
            0x006d_7a40,
            &args![reached, from, to, x87_quiet_bits(reach)],
        );
        let point = local(0xc8);
        e.call(0x005c_3420, &args![reached, point]);
        let source = e
            .call(0x0043_9ef0, &args![point, local(0x10c), target_position])
            .u32();
        copy_words(e, direction, source, 3);
        let squared = x87_store32(e.call(0x004a_7290, &args![direction]).f64());
        let low = minimum.to_bits();
        let high = maximum.to_bits();
        let accepted = !(x87_load32(squared) < x87_mul(x87_load32(low), x87_m32(low)))
            && !(x87_load32(squared) > x87_mul(x87_load32(high), x87_m32(high)))
            && (check_los == 0
                || e.call(
                    0x009a_7940,
                    &args![attacker, target, point, target_position, 0u32, u32::MAX],
                )
                .bool());
        if accepted {
            fn_0099b8b0(e, out, point, 0);
        }
        e.call(0x004f_f7e0, &args![reached]);
        e.call(0x004f_f7e0, &args![to]);
        e.call(0x004f_f7e0, &args![from]);
        accepted
    })
}

/// Fills `out`'s radius from a found reference (`00819a50`): `setting(011cf144)` plus the
/// attacker's and the reference's radii (`008be1b0`, summed as `double`s), rounded to a
/// float.
fn set_flee_radius(e: &mut Engine, out: Ptr<CombatLocation>, reference: u32, attacker: u32) {
    let theirs = e.call(0x008b_e1b0, &args![reference]).f64();
    let ours = e.call(0x008b_e1b0, &args![attacker]).f64();
    let sum = x87_add(ours, theirs);
    let margin = setting_value_address(e, 0x011c_f144);
    let radius = x87_store32(x87_add(x87_load32(e.mem.u32(margin)), sum));
    e.call(0x0081_9a50, &args![out, x87_quiet_bits(radius)]);
}

// Translated from 0099ebe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099ebe0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `CombatState::FindBestFleeLocation` (Xbox PDB; `thiscall`, `RET 0xc`): puts a flee
/// destination into `out`; always true in the end.
/// - With a target: the nearest ally to flee to (the list from `00461110` of `011c95c8`,
///   walked with `004b9ba0` and `006b7f20`): alive (`00437b90`), slot `+0x100` set, slot
///   `+0x22c(0)` clear, neither attacker nor target, between the setting `011ce040` and
///   `011cf020` (squared) from the attacker, not nearer the target than the attacker,
///   reachable (`009a02a0`), not fleeing (`Actor::IsFleeing`), hostile to the target
///   (`Actor::GetShouldAttackActor`) or wanting to help (`CombatManager::GetWanttoHelpGroup`
///   `00992530`) and not hostile to the attacker. The ally becomes the location's reference
///   (`00997400`) with the radius from `set_flee_radius`.
/// - Otherwise, when the attacker's `004181e0` +0x30 allows it (`0055e200`, and slot
///   `+0x21c` of the attacker or that object's slot `+0x28` clear) and `kind` is not 10: the
///   flee-door search runs once the stamp `fLastFleeDoorTimeStamp` (renewed while not
///   fleeing when `004dff00` of the setting `011cf294` says so) is older than the setting
///   `011cead4`. The nearest door (`006d9350` within the setting `011cf234` of the
///   attacker) that is not the last flee door, leads somewhere valid, passes both movement
///   checks (`009a02a0`, the second for its teleport location `0043a390`), shares the
///   encounter zone when the switch at `011f1840` asks (`00567d20`, `00546a90`) and has no
///   target near it (`00989700` with the setting `011ce2f0`) gives the location (its
///   destination package point `00717e50` and teleport worldspace or cell).
/// - Otherwise, with a cover location, a cover search (`0099b8e0`); then a type 1 location
///   target (its reference, `00997400`, with the radius when slot `+0x100` holds, or its
///   world location, `0097ee60`); else the location's flags 1 are set (`0099b540`).
pub fn combat_state_find_best_flee_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    cover_location: u32,
    out: Ptr<CombatLocation>,
    kind: u32,
) -> bool {
    e.with_stack(FLEE_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + FLEE_FRAME - k;
        let attacker = attacker(e, this);
        let target = target(e, this);
        let own = local(0x24);
        e.call(GET_WORLD_LOCATION, &args![attacker, own]);
        if target != 0 {
            let theirs = local(0x40);
            e.call(GET_WORLD_LOCATION, &args![target, theirs]);
            let mut best = 0u32;
            let first = setting_value_address(e, 0x011c_f020);
            let second = setting_value_address(e, 0x011c_f020);
            let mut limit = x87_store32(x87_mul(
                x87_load32(e.mem.u32(first)),
                x87_m32(e.mem.u32(second)),
            ));
            let owner: u32 = e.global(ACTOR_LIST_OWNER);
            let list = e.call(0x0046_1110, &args![owner]).u32();
            let position = local(0x2c);
            let first_position = e.call(0x004b_9ba0, &args![list]).u32();
            e.mem.set_u32(position, first_position);
            while e.mem.u32(position) != 0 {
                let key = local(0x70);
                let value = local(0x50);
                e.mem.set_u32(key, 0);
                e.mem.set_u32(value, 0);
                e.call(0x006b_7f20, &args![list, position, key, value]);
                let actor = e.mem.u32(value);
                if e.call(0x0043_7b90, &args![actor]).bool()
                    || !e.vcall(actor, 0x100, &args![]).bool()
                    || e.vcall(actor, 0x22c, &args![0u32]).bool()
                    || actor == attacker
                    || actor == target
                {
                    continue;
                }
                let theirs_now = local(0x68);
                e.call(GET_WORLD_LOCATION, &args![actor, theirs_now]);
                let to_ally = x87_store32(e.call(0x0052_7a40, &args![own, theirs_now]).f64());
                let first = setting_value_address(e, 0x011c_e040);
                let second = setting_value_address(e, 0x011c_e040);
                let nearest = x87_mul(x87_load32(e.mem.u32(first)), x87_m32(e.mem.u32(second)));
                if x87_load32(to_ally) < nearest || x87_load32(limit) < x87_load32(to_ally) {
                    continue;
                }
                let target_to_ally =
                    x87_store32(e.call(0x0052_7a40, &args![theirs, theirs_now]).f64());
                let target_to_us = x87_store32(e.call(0x0052_7a40, &args![theirs, own]).f64());
                if x87_load32(target_to_ally) < x87_load32(target_to_us) {
                    continue;
                }
                if !combat_state_check_movement(e, this, theirs_now, 1, 0) {
                    continue;
                }
                if e.call(0x008a_6650, &args![actor, 0u32]).bool() {
                    continue;
                }
                let scratch = local(0x6c);
                if !e
                    .call(0x008b_06d0, &args![actor, target, 0u32, scratch, 0u32])
                    .bool()
                {
                    let manager: u32 = e.global(COMBAT_MANAGER);
                    if !e.call(0x0099_2530, &args![manager, actor, attacker]).bool() {
                        continue;
                    }
                }
                if e.call(0x008b_06d0, &args![actor, attacker, 0u32, scratch, 0u32])
                    .bool()
                {
                    continue;
                }
                best = actor;
                limit = x87_quiet_bits(to_ally);
            }
            if best != 0 {
                combat_location_set_ref(e, out, best);
                set_flee_radius(e, out, best, attacker);
                return true;
            }
        }
        if flee_through_door(e, this, attacker, out, kind, frame) {
            return true;
        }
        if cover_location != 0
            && fn_0099b8e0(
                e,
                this,
                0,
                this.at(CombatState::CoverSearchInfo),
                cover_location,
                out.addr(),
                kind,
            )
        {
            return true;
        }
        let location_target = e.get(this, CombatState::pLocationTarget);
        if location_target != 0 && e.mem.u8(location_target) == 1 {
            let reference = e.mem.u32(e.get(this, CombatState::pLocationTarget) + 4);
            if reference != 0 {
                let reference = e.mem.u32(e.get(this, CombatState::pLocationTarget) + 4);
                combat_location_set_ref(e, out, reference);
                let reference = e.mem.u32(e.get(this, CombatState::pLocationTarget) + 4);
                if e.vcall(reference, 0x100, &args![]).bool() {
                    set_flee_radius(e, out, reference, attacker);
                }
            } else {
                let world = e.get(this, CombatState::pLocationTarget) + 8;
                e.call(0x0097_ee60, &args![out, world]);
            }
            return true;
        }
        fn_0099b540(e, out, 1, 1);
        true
    })
}

/// The flee-door part of `0099ebe0`; true when a door gave the location.
fn flee_through_door(
    e: &mut Engine,
    this: Ptr<CombatState>,
    attacker: u32,
    out: Ptr<CombatLocation>,
    kind: u32,
    frame: Ptr,
) -> bool {
    let local = |k: u32| frame.addr() + FLEE_FRAME - k;
    let extra = e.call(0x0041_81e0, &args![attacker]).u32();
    if !e.call(0x0055_e200, &args![extra + 0x30]).bool() {
        return false;
    }
    if e.vcall(attacker, 0x21c, &args![]).bool() && e.vcall(extra + 0x30, 0x28, &args![]).bool() {
        return false;
    }
    if kind == 10 {
        return false;
    }
    if e.get(this, CombatState::cFleeState) == 0 {
        let chance = setting_bits(e, 0x011c_f294);
        if e.call(0x004d_ff00, &args![chance]).bool() {
            let now = game_time_bits(e);
            let stamp = local(0xf0);
            e.call(TIME_STAMP_SET, &args![stamp, now]);
            let stamp = e.mem.u32(stamp);
            e.set(this, CombatState::fLastFleeDoorTimeStamp, stamp);
        }
    }
    let age = e.call(TIME_STAMP_ELAPSED, &args![member(this, 0xe0)]).f64();
    let delay = setting_value(e, 0x011c_ead4);
    if !(f64::from(x87_quiet(delay)) < age) {
        return false;
    }
    let mut door = 0u32;
    let position = local(0x84);
    let source = e.vcall(attacker, 0x1f4, &args![]).u32();
    copy_words(e, position, source, 3);
    let radius = setting_bits(e, 0x011c_f234);
    let mut nearest = x87_store32(x87_mul(x87_load32(radius), x87_m32(radius)));
    let doors = local(0x98);
    e.call(0x005e_04c0, &args![doors]);
    let space = e.call(GET_SPACE, &args![attacker]).u32();
    if e.call(0x006d_9350, &args![space, position, radius, doors])
        .u32()
        != 0
    {
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![doors]).u32() {
            let element = e.call(0x006a_7ad0, &args![doors, index]).u32();
            let candidate = e.mem.u32(element);
            index += 1;
            if e.call(0x0043_7b90, &args![candidate]).bool() {
                continue;
            }
            let last = e.get(this, CombatState::iLastFleeDoor);
            if e.call(0x0084_e3a0, &args![candidate]).u32() == last {
                continue;
            }
            let teleport = e.call(0x0056_8e50, &args![candidate]).u32();
            let linked = e.call(0x0055_9450, &args![teleport]).u32();
            let last = e.get(this, CombatState::iLastFleeDoor);
            if e.call(0x0084_e3a0, &args![linked]).u32() == last
                || e.call(0x0043_7b90, &args![linked]).bool()
            {
                continue;
            }
            let switch = e
                .call(0x0040_8d60, &args![ENCOUNTER_ZONE_SWITCH_011F1840])
                .u32();
            if e.mem.u8(switch) != 0 {
                let mut theirs = e.call(0x0056_7d20, &args![linked]).u32();
                let mut ours = e.call(0x0056_7d20, &args![attacker]).u32();
                if theirs == 0 {
                    theirs = e.call(0x0054_6a90, &args![]).u32();
                }
                if ours == 0 {
                    ours = e.call(0x0054_6a90, &args![]).u32();
                }
                if theirs != ours {
                    continue;
                }
            }
            let door_location = local(0xc8);
            e.call(GET_WORLD_LOCATION, &args![candidate, door_location]);
            if !combat_state_check_movement(e, this, door_location, 1, 0) {
                continue;
            }
            let destination = e.call(0x0043_a390, &args![teleport, local(0x100)]).u32();
            if !combat_state_check_movement(e, this, destination, 1, 1) {
                continue;
            }
            let near = setting_bits(e, 0x011c_e2f0);
            let controller = state_controller(e, this);
            let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
            if e.call(0x0098_9700, &args![group, door_location, near, 0u32])
                .u32()
                != 0
            {
                continue;
            }
            let door_position = local(0xd8);
            let source = e.vcall(candidate, 0x1f4, &args![]).u32();
            copy_words(e, door_position, source, 3);
            let offset = local(0xac);
            e.call(0x0043_9ef0, &args![door_position, offset, position]);
            let squared = x87_store32(e.call(0x004a_7290, &args![offset]).f64());
            if x87_load32(nearest) > x87_load32(squared) {
                nearest = x87_quiet_bits(squared);
                door = candidate;
            }
        }
    }
    if door != 0 {
        let teleport = e.call(0x0056_8e50, &args![door]).u32();
        let mut space = e.call(0x0043_a320, &args![teleport]).u32();
        if space == 0 {
            space = e.call(0x0043_a2b0, &args![teleport]).u32();
        }
        let point = e.call(0x0071_7e50, &args![teleport]).u32();
        fn_0099b8b0(e, out, point, space);
        e.call(0x005e_04f0, &args![doors]);
        return true;
    }
    e.call(0x005e_04f0, &args![doors]);
    false
}

// ---- Batch n025: `0099f480`, `0099f6d0` ------------------------------------------------

/// Stack frame of `0099f480` (from `EBP-0x68`).
const LOS_LOCATION_FRAME: u32 = 0x68;
/// `67108864.0` (`float`, 8192 squared).
const LOS_SEARCH_LIMIT: u32 = 0x0108_e060;
/// Stack frame of `0099f6d0` (from `EBP-0x1a0`).
const PICKUP_FRAME: u32 = 0x1a0;
/// `589824.0` (`double`, 768 squared).
const PICKUP_TARGET_NEAR_DOUBLE: u32 = 0x0108_cb08;
/// `64.0` (`float`).
const SIXTY_FOUR: u32 = 0x0101_e6ec;
/// The object at `011f1a60` that `00483710` is called on around the pickup search of
/// `0099f6d0`.
const PICKUP_SEARCH_OBJECT_011F1A60: u32 = 0x011f_1a60;
/// The pointer at `011c95c8` whose loaded-reference search (`005273c0`) `0099f6d0` runs.
const LOADED_REFERENCES: u32 = 0x011c_95c8;

/// Whether `member` (a group member with an active combat controller, slot `+0x428`) has a
/// combat state that `00998980` accepts, or line of sight in 360 degrees to the target
/// (`Actor::Has360LineOfSight`): the test `0099f480` makes of each candidate.
fn sees_target(e: &mut Engine, this: Ptr<CombatState>, member: u32) -> bool {
    let controller = e.vcall(member, 0x428, &args![]).u32();
    if controller != 0 {
        let state = e.call(0x0058_6150, &args![controller]).u32();
        if fn_00998980(e, Ptr::new(state)) {
            return true;
        }
    }
    let target = target(e, this);
    e.call(0x0088_c600, &args![member, target]).bool()
}

// Translated from 0099f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatState::FindLOSLocation` (Xbox PDB; `thiscall`, `RET 8`, the second word is not
/// read): a location from which the target can be seen. When `out`'s reference is a group
/// member (`CombatGroup::IsMember`) whose slot `+0x100` holds and who sees the target
/// (`sees_target`), `out` stays and the result is true. Otherwise the nearest other group
/// member (`00990890`, `00903600`) that sees the target, nearer than 8192 (`0108e060`,
/// squared) and reachable (`009a02a0`), becomes the reference (`00997400`) with the radius
/// from `set_flee_radius`; false without one.
pub fn combat_state_find_los_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    out: Ptr<CombatLocation>,
    _unused_2: u32,
) -> bool {
    let controller = state_controller(e, this);
    let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
    let current = e.get(out, CombatLocation::pRef);
    if current != 0 && e.vcall(current, 0x100, &args![]).bool() {
        let current = e.get(out, CombatLocation::pRef);
        if e.call(0x0098_6a80, &args![group, current]).bool() && sees_target(e, this, current) {
            return true;
        }
    }
    e.with_stack(LOS_LOCATION_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + LOS_LOCATION_FRAME - k;
        let attacker = attacker(e, this);
        let own = local(0x20);
        e.call(GET_WORLD_LOCATION, &args![attacker, own]);
        let mut best = 0u32;
        let mut nearest = constant_bits(e, LOS_SEARCH_LIMIT);
        let count = e.call(0x0099_0890, &args![group]).u32();
        let mut index = 0u32;
        while index < count {
            let member = e.call(0x0090_3600, &args![group, index]).u32();
            index += 1;
            if member == attacker || !sees_target(e, this, member) {
                continue;
            }
            let theirs = local(0x48);
            e.call(GET_WORLD_LOCATION, &args![member, theirs]);
            let squared = x87_store32(e.call(0x0052_7a40, &args![theirs, own]).f64());
            if x87_load32(nearest) <= x87_load32(squared) {
                continue;
            }
            if !combat_state_check_movement(e, this, theirs, 1, 0) {
                continue;
            }
            best = member;
            nearest = x87_quiet_bits(squared);
        }
        if best == 0 {
            return false;
        }
        combat_location_set_ref(e, out, best);
        set_flee_radius(e, out, best, attacker);
        true
    })
}

/// The pickup search state of `0099f6d0`.
struct PickupSearch {
    attacker: u32,
    own: u32,
    melee: u8,
    threshold: u32,
    near: u32,
    far: u32,
    best_value: u32,
    best_distance: u32,
    best: u32,
    best_form: u32,
    container: u32,
    container_ammo: u32,
}

// Translated from 0099f6d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0099f6d0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map (`thiscall`): searches the loaded references for a better weapon
/// to pick up and records it as combat items 4 (the weapon reference and form) and 3 (a
/// container with ammunition for it, and that ammunition); true when one was found.
/// - Nothing is searched unless the controller allows it (`009a0240`), there is no embedded
///   weapon (`pCurrentEmbeddedWeapon`, `EmbeddedWeaponsArray`), a commanded attacker (slot
///   `+0x21c`) has a commanding actor (`005f9b50` of `004181e0`), and the equipped entry's
///   extra data (slot `+0x148` of `008d8520`) does not forbid it (`00418b10`).
/// - The DPS threshold is `(setting(011cfb6c) + 1)` times the best ranged (melee controller,
///   `0097fd40`) or overall weapon DPS, raised to the disarmed DPS (`0040ebd0`) with the scale
///   `setting(011cfa10)` when that is higher; a melee controller uses the scale
///   `setting(011cf908)`. The near and far search distances come from settings by weapon
///   kind (`007031c0`, `004c0c30`), the far one widened by the ranged/overall DPS ratio when
///   the target is not within 768 and the ratio passes `setting(011ce260)`; both are scaled.
/// - Each candidate (`005273c0` over `011c95c8` with the iterator `005271f0(0x28,
///   attacker)`, kind 3) with non-zero health (`004bcdb0` or `004186f0`) that is not
///   excluded (`00477ba0`) gives a DPS (`00646060` with the health fraction over
///   `004873d0`, and `setting(011cf0c8)` for `006450c0` weapons) above the threshold; the
///   attacker must accept it (`008bc9d0`), it must lie within the distance the DPS allows
///   (interpolated between near and far through `setting(011ce7fc)`) and be reachable
///   (`009a02a0`); a container (form type 0x1b) must not be locked (`005021a0`) and be the
///   attacker's when owned (`005785e0`). A weapon with ammunition (`00525980`) needs it in
///   the candidate, the attacker or the nearest reachable container holding it (a second
///   search, `005272d0`).
/// - The best candidate has the highest `DPS * 10000 / max(sqrt(distance), 64)` (`004579e0`,
///   `00404010`), then the smallest distance.
pub fn fn_0099f6d0(e: &mut Engine, this: Ptr<CombatState>) -> bool {
    let attacker = attacker(e, this);
    let target = target(e, this);
    let controller = state_controller(e, this);
    if !e.call(0x009a_0240, &args![controller]).bool() {
        return false;
    }
    if e.get(this, CombatState::pCurrentEmbeddedWeapon) != 0
        || e.call(ARRAY_SIZE, &args![member(this, 0x24)]).u32() != 0
    {
        return false;
    }
    if e.vcall(attacker, 0x21c, &args![]).bool() {
        let extra = e.call(0x0041_81e0, &args![attacker]).u32();
        if e.call(0x005f_9b50, &args![extra]).u32() == 0 {
            return false;
        }
    }
    let process = e.call(0x008d_8520, &args![attacker]).u32();
    let entry = e.vcall(process, 0x148, &args![]).u32();
    if entry != 0 && e.call(0x0055_9450, &args![entry]).u32() != 0 {
        let list = e.call(0x0055_9450, &args![entry]).u32();
        let list = e.call(0x0068_15c0, &args![list]).u32();
        if e.mem.u32(list) != 0 {
            let list = e.call(0x0055_9450, &args![entry]).u32();
            let list = e.call(0x0068_15c0, &args![list]).u32();
            let extra = e.mem.u32(list);
            if e.call(0x0041_8b10, &args![extra]).bool() {
                return false;
            }
        }
    }
    e.with_stack(PICKUP_FRAME, |e, frame| {
        let local = |k: u32| frame.addr() + PICKUP_FRAME - k;
        e.call(0x0048_3710, &args![PICKUP_SEARCH_OBJECT_011F1A60]);
        let own = local(0x88);
        e.call(GET_WORLD_LOCATION, &args![attacker, own]);
        let controller = state_controller(e, this);
        let weapon = e.call(0x0070_31c0, &args![controller]).u32();
        let melee = e.call(0x0097_fd40, &args![this]).u8();
        let base = if melee != 0 {
            x87_quiet_bits(e.get(this, CombatState::fMaxRangedWeaponDPS).to_bits())
        } else {
            x87_quiet_bits(e.get(this, CombatState::fMaxCombatWeaponDPS).to_bits())
        };
        let factor = setting_value_address(e, 0x011c_fb6c);
        let mut threshold = x87_store32(x87_mul(
            x87_add(x87_load32(e.mem.u32(factor)), 1.0),
            x87_m32(base),
        ));
        let mut scale = 1.0f32.to_bits();
        let best_dps = e.get(this, CombatState::fMaxCombatWeaponDPS).to_bits();
        let disarmed = e.get(this, CombatState::fDisarmedDPS).to_bits();
        if x87_load32(best_dps) < x87_load32(disarmed) {
            scale = setting_bits(e, 0x011c_fa10);
            let disarmed = x87_quiet_bits(e.get(this, CombatState::fDisarmedDPS).to_bits());
            threshold = x87_store32(
                e.call(0x0040_ebd0, &args![x87_quiet_bits(threshold), disarmed])
                    .f64(),
            );
        }
        if melee != 0 {
            scale = setting_bits(e, 0x011c_f908);
        }
        let (mut near, mut far);
        if melee != 0 || (weapon != 0 && e.call(0x004c_0c30, &args![weapon]).bool()) {
            near = setting_bits(e, 0x011c_fb44);
            far = setting_bits(e, 0x011c_ef08);
        } else {
            if weapon == 0 {
                near = setting_bits(e, 0x011c_ee9c);
                far = setting_bits(e, 0x011c_ec4c);
            } else {
                near = setting_bits(e, 0x011c_f320);
                far = setting_bits(e, 0x011c_e460);
            }
            let mut target_near = false;
            if target != 0 {
                let theirs = local(0xb8);
                e.call(GET_WORLD_LOCATION, &args![target, theirs]);
                let squared = e.call(0x0052_7a40, &args![own, theirs]).f64();
                let limit: f64 = e.global(PICKUP_TARGET_NEAR_DOUBLE);
                if squared < limit {
                    target_near = true;
                }
            }
            let zero: f64 = e.global(ZERO_DOUBLE);
            let ranged = e.get(this, CombatState::fMaxRangedWeaponDPS).to_bits();
            let overall = e.get(this, CombatState::fMaxCombatWeaponDPS).to_bits();
            if !target_near && x87_load32(ranged) > zero && x87_load32(overall) > zero {
                let ranged = e.get(this, CombatState::fMaxRangedWeaponDPS).to_bits();
                let overall = e.get(this, CombatState::fMaxCombatWeaponDPS).to_bits();
                let ratio = x87_store32(x87_div(x87_load32(ranged), x87_m32(overall)));
                let minimum = setting_value(e, 0x011c_e260);
                if f64::from(x87_quiet(minimum)) < x87_load32(ratio) {
                    let widest = setting_value(e, 0x011c_ef08);
                    if x87_load32(far) < f64::from(x87_quiet(widest)) {
                        let widest = setting_value_address(e, 0x011c_ef08);
                        far = x87_store32(x87_add(
                            x87_mul(
                                x87_sub(x87_load32(e.mem.u32(widest)), x87_m32(far)),
                                x87_m32(ratio),
                            ),
                            x87_m32(far),
                        ));
                    }
                }
            }
        }
        near = x87_store32(x87_mul(x87_load32(near), x87_m32(scale)));
        far = x87_store32(x87_mul(x87_load32(far), x87_m32(scale)));
        let mut search = PickupSearch {
            attacker,
            own,
            melee,
            threshold,
            near,
            far,
            best_value: 0,
            best_distance: constant_bits(e, MAX_FLOAT),
            best: 0,
            best_form: 0,
            container: 0,
            container_ammo: 0,
        };
        let iterator = local(0x58);
        e.call(0x0052_71f0, &args![iterator, 0x28u32, attacker]);
        let reference_slot = local(0x98);
        let item_slot = local(0x74);
        e.mem.set_u32(reference_slot, 0);
        e.mem.set_u32(item_slot, 0);
        let references: u32 = e.global(LOADED_REFERENCES);
        while e
            .call(
                0x0052_73c0,
                &args![references, iterator, reference_slot, item_slot, 3u32],
            )
            .bool()
        {
            consider_pickup(e, this, &mut search, frame, reference_slot, item_slot);
        }
        e.call(0x0048_3710, &args![PICKUP_SEARCH_OBJECT_011F1A60]);
        let found = search.best != 0;
        if found {
            combat_state_set_combat_item(e, this, 3, search.container, search.container_ammo);
            combat_state_set_combat_item(e, this, 4, search.best, search.best_form);
        }
        e.call(0x0093_e510, &args![iterator]);
        found
    })
}

/// One candidate of `0099f6d0`'s reference search (the loop body).
fn consider_pickup(
    e: &mut Engine,
    this: Ptr<CombatState>,
    search: &mut PickupSearch,
    frame: Ptr,
    reference_slot: u32,
    item_slot: u32,
) {
    let local = |k: u32| frame.addr() + PICKUP_FRAME - k;
    let reference = e.mem.u32(reference_slot);
    let item = e.mem.u32(item_slot);
    let health = if item != 0 {
        e.call(0x004b_cdb0, &args![item, 0u32]).f64()
    } else {
        let extra = e.call(0x005d_43c0, &args![reference]).u32();
        e.call(0x0041_86f0, &args![extra]).f64()
    };
    let health = x87_quiet_bits(x87_store32(health));
    let reference = e.mem.u32(reference_slot);
    if e.call(0x0047_7ba0, &args![reference]).bool() {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    if x87_load32(health) == zero {
        return;
    }
    let item = e.mem.u32(item_slot);
    let form = if item != 0 {
        e.call(0x0044_ddc0, &args![item]).u32()
    } else {
        let reference = e.mem.u32(reference_slot);
        e.call(0x007a_f430, &args![reference]).u32()
    };
    let mut fraction = 1.0f32.to_bits();
    let minus_one: f64 = e.global(MINUS_ONE_DOUBLE);
    if x87_load32(health) != minus_one {
        let full = e.call(0x0048_73d0, &args![form]).u32();
        fraction = x87_store32(x87_div(x87_m32(health), f64::from(full)));
    }
    let controller = state_controller(e, this);
    let source = e.call(0x0098_0070, &args![controller]).u32();
    let mut dps = x87_store32(
        e.call(
            0x0064_6060,
            &args![
                search.attacker,
                form,
                x87_quiet_bits(fraction),
                source,
                0u32,
                0u32
            ],
        )
        .f64(),
    );
    if e.call(0x0064_50c0, &args![form]).bool() {
        let bonus = setting_value_address(e, 0x011c_f0c8);
        dps = x87_store32(x87_mul(x87_load32(dps), x87_m32(e.mem.u32(bonus))));
    }
    if x87_load32(search.threshold) >= x87_load32(dps) {
        return;
    }
    if search.melee != 0 && !e.call(0x004c_0c30, &args![form]).bool() {
        return;
    }
    if !e
        .call(0x008b_c9d0, &args![search.attacker, form, 0u32])
        .bool()
    {
        return;
    }
    let reach_squared;
    if x87_load32(search.threshold) <= zero {
        reach_squared = x87_store32(x87_mul(x87_load32(search.far), x87_m32(search.far)));
    } else {
        let ratio = x87_store32(x87_div(x87_load32(dps), x87_m32(search.threshold)));
        let cap = setting_bits(e, 0x011c_e7fc);
        let ratio = x87_store32(
            e.call(0x0040_ebd0, &args![x87_quiet_bits(ratio), cap])
                .f64(),
        );
        let one: f64 = e.global(ONE_DOUBLE);
        let numerator = x87_sub(x87_load32(ratio), one);
        let cap = setting_value_address(e, 0x011c_e7fc);
        let denominator = x87_sub(x87_load32(e.mem.u32(cap)), one);
        let t = x87_store32(x87_div(numerator, denominator));
        let reach = x87_store32(x87_add(
            x87_mul(
                x87_sub(x87_load32(search.far), x87_m32(search.near)),
                x87_m32(t),
            ),
            x87_m32(search.near),
        ));
        reach_squared = x87_store32(x87_mul(x87_load32(reach), x87_m32(reach)));
    }
    let reference = e.mem.u32(reference_slot);
    let theirs = local(0xe4);
    e.call(GET_WORLD_LOCATION, &args![reference, theirs]);
    let mut distance = x87_store32(e.call(0x0052_7a40, &args![theirs, search.own]).f64());
    if x87_load32(reach_squared) <= x87_load32(distance) {
        return;
    }
    if !combat_state_check_movement(e, this, theirs, 1, 0) {
        return;
    }
    let reference = e.mem.u32(reference_slot);
    if e.call(0x007a_f430, &args![reference]).u32() != 0 {
        let base = e.call(0x007a_f430, &args![reference]).u32();
        if e.call(0x0040_1170, &args![base]).u32() == 0x1b
            && e.call(0x007a_f430, &args![reference]).u32() != 0
        {
            if e.call(0x0056_9160, &args![reference]).u32() != 0 {
                let lock = e.call(0x0056_9160, &args![reference]).u32();
                if e.call(0x0050_21a0, &args![lock]).bool() {
                    return;
                }
            }
            if e.call(0x0056_7790, &args![reference]).u32() != 0
                && !e
                    .call(0x0057_85e0, &args![reference, search.attacker, 1u32])
                    .bool()
            {
                return;
            }
        }
    }
    let mut container = 0u32;
    let mut has_ammo = 1u8;
    let ammo = e.call(0x0052_5980, &args![form, search.attacker]).u32();
    if ammo != 0 {
        has_ammo = 0;
        if e.mem.u32(item_slot) == 0 {
            let reference = e.mem.u32(reference_slot);
            let extra = e.call(0x005d_43c0, &args![reference]).u32();
            if e.call(0x0042_eb40, &args![extra]).u32() != 0 {
                has_ammo = 1;
            }
        }
        if has_ammo == 0 && e.call(0x0057_5610, &args![search.attacker, ammo]).u32() != 0 {
            has_ammo = 1;
        }
        if has_ammo == 0 && e.mem.u32(item_slot) != 0 {
            let reference = e.mem.u32(reference_slot);
            if e.call(0x0057_5610, &args![reference, ammo]).u32() != 0 {
                has_ammo = 1;
                container = e.mem.u32(reference_slot);
            }
        }
        if has_ammo == 0 {
            let mut nearest = constant_bits(e, MAX_FLOAT);
            let reference = e.mem.u32(reference_slot);
            let source = e.vcall(reference, 0x1f4, &args![]).u32();
            copy_words(e, local(0x10c), source, 3);
            let iterator = local(0x150);
            e.call(0x0052_72d0, &args![iterator, ammo, search.attacker]);
            let other_slot = local(0x114);
            let other_item = local(0x118);
            e.mem.set_u32(other_slot, 0);
            e.mem.set_u32(other_item, 0);
            let references: u32 = e.global(LOADED_REFERENCES);
            while e
                .call(
                    0x0052_73c0,
                    &args![references, iterator, other_slot, other_item, 3u32],
                )
                .bool()
            {
                let other = e.mem.u32(other_slot);
                let place = local(0x164);
                e.call(GET_WORLD_LOCATION, &args![other, place]);
                let first = e.call(0x0052_7a40, &args![place, search.own]).f64();
                let second = e.call(0x0052_7a40, &args![place, theirs]).f64();
                let sum = x87_store32(x87_add(second, first));
                if x87_load32(reach_squared) < x87_load32(sum) {
                    continue;
                }
                if x87_load32(nearest) < x87_load32(sum) {
                    continue;
                }
                if !combat_state_check_movement(e, this, place, 1, 0) {
                    continue;
                }
                container = e.mem.u32(other_slot);
                nearest = x87_quiet_bits(sum);
                distance = x87_quiet_bits(sum);
                has_ammo = 1;
            }
            e.call(0x0093_e510, &args![iterator]);
        }
    }
    if has_ammo == 0 {
        return;
    }
    let root = e.call(0x0045_79e0, &args![x87_quiet_bits(distance)]).f64();
    let floor = constant_bits(e, SIXTY_FOUR);
    let range = x87_store32(e.call(0x0040_4010, &args![x87_store32(root), floor]).f64());
    let weight: f64 = e.global(OUT_OF_RANGE_DIVISOR_DOUBLE);
    let value = x87_store32(x87_div(x87_mul(x87_load32(dps), weight), x87_m32(range)));
    let better = if x87_load32(search.best_value) < x87_load32(value) {
        true
    } else {
        x87_load32(search.best_value) == x87_load32(value)
            && x87_load32(search.best_distance) > x87_load32(distance)
    };
    if better {
        search.best_value = x87_quiet_bits(value);
        search.best_distance = x87_quiet_bits(distance);
        search.best = e.mem.u32(reference_slot);
        search.best_form = if search.best != 0 { form } else { 0 };
        search.container = container;
        search.container_ammo = if container != 0 { ammo } else { 0 };
    }
}

// ---- Batch n025: `009a17a0`, `009a2040` ------------------------------------------------

/// `TESObjectWEAP`'s, `TESBoundObject`'s and `BGSExplosion`'s RTTI type descriptors.
const TES_OBJECT_WEAP_TYPE: u32 = 0x0118_3998;
const TES_BOUND_OBJECT_TYPE: u32 = 0x0118_3108;
const BGS_EXPLOSION_TYPE: u32 = 0x0118_620c;
/// `BGSSaveGameBuffer::SaveFormID` (Xbox PDB, `00865db0`, `thiscall(form, 0)`), used for
/// `iLastFleeDoor`.
const SAVE_FORM_ID_PLAIN: u32 = 0x0086_5db0;
/// `CombatTimer::SaveGame` / `LoadGame` (Xbox PDB, `009a5eb0` / `009a5f20`).
const TIMER_SAVE: u32 = 0x009a_5eb0;
const TIMER_LOAD: u32 = 0x009a_5f20;
/// The time stamp getter of a `CombatCoverSearchInfo` stamp (`006a7f50`, `ST0`).
const STAMP_VALUE: u32 = 0x006a_7f50;
/// The eleven timers saved after the path check, in the exe's order (`0x20c` before
/// `0x204`).
const TRAILING_TIMERS: [u32; 11] = [
    0x1cc, 0x1d4, 0x1dc, 0x1e4, 0x1ec, 0x1f4, 0x1fc, 0x20c, 0x204, 0x214, 0x21c,
];

/// Whether the cover search information's time stamp at `stamp` (`006a7f50`) differs from
/// `-FLT_MAX` (a NaN differs).
fn search_stamped(e: &mut Engine, stamp: u32) -> bool {
    let value = e.call(STAMP_VALUE, &args![stamp]).f64();
    let unset = f64::from(f32::from_bits(negative_max_bits(e)));
    value != unset
}

/// A loaded form id (`008648a0`) resolved and cast to `target_type`.
fn load_form(e: &mut Engine, buffer: u32, target_type: u32) -> u32 {
    let id = e.call(0x0086_48a0, &args![buffer]).u32();
    resolve_loaded_form(e, id, target_type)
}

/// The count saved for the array at `array` (0 when its address is null, as the exe tests).
fn saved_count(e: &mut Engine, array: u32) -> u32 {
    if array == 0 {
        0
    } else {
        e.call(ARRAY_SIZE, &args![array]).u32()
    }
}

// Translated from 009a17a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatState` save (by its body; `thiscall`, `RET 4`): the fields in the exe's order
/// through `00865e50` (data), `00865df0` (form ids), `00865f60` (counts), the time stamps
/// (`009a5f90`) and timers (`009a5eb0`); then a byte of which optional parts follow (1 cover
/// location, 2 cover search stamped, 4 secondary cover location, 8 secondary search
/// stamped, 0x10 combat location, 0x20 location target) and those parts (`009a2de0`,
/// `009a2bf0`, `009a3130`, `009a2f50`), both unreachable arrays (`009a2d20`, `009a2d80`), the
/// dodge stamp and direction, a byte of which combat items exist and their references and
/// values, the usable items with their timers, and the remaining fields and timers.
pub fn fn_009a17a0(e: &mut Engine, this: Ptr<CombatState>, buffer: u32) {
    let me = this.addr();
    let data = |e: &mut Engine, off: u32, size: u32| {
        e.call(SAVE_DATA, &args![buffer, me.wrapping_add(off), size, 0u32]);
    };
    let form = |e: &mut Engine, word: u32| {
        e.call(SAVE_FORM_ID, &args![buffer, word, 0u32]);
    };
    data(e, 0, 1);
    data(e, 4, 4);
    for index in 0u32..6 {
        let weapon = e.mem.u32(me + 0xc + 4 * index);
        form(e, weapon);
    }
    let weapons = me.wrapping_add(0x24);
    let count = saved_count(e, weapons);
    e.call(SAVE_VARIABLE_SIZE, &args![buffer, count]);
    for index in 0..count {
        let element = e.call(0x006a_7ad0, &args![weapons, index]).u32();
        let weapon = e.mem.u32(element);
        form(e, weapon);
    }
    let embedded = e.get(this, CombatState::pCurrentEmbeddedWeapon);
    form(e, embedded);
    data(e, 0x38, 4);
    for index in 0u32..7 {
        data(e, 0x3c + 4 * index, 4);
    }
    for off in [0x58u32, 0x5c, 0x64, 0x68, 0x6c] {
        data(e, off, 4);
    }
    for off in 0x70u32..0x78 {
        data(e, off, 1);
    }
    let explosion = e.get(this, CombatState::pExplosiveProjectileBlocked);
    form(e, explosion);
    data(e, 0x7c, 4);
    data(e, 0x80, 4);
    data(e, 0x84, 0xc);
    data(e, 0x90, 4);
    data(e, 0xc8, 1);
    data(e, 0xc9, 1);
    data(e, 0xcc, 4);
    data(e, 0xd0, 4);
    data(e, 0xd4, 4);
    data(e, 0x8, 4);
    data(e, 0xd8, 1);
    let door = e.get(this, CombatState::iLastFleeDoor);
    e.call(SAVE_FORM_ID_PLAIN, &args![buffer, door, 0u32]);
    for off in [0x94u32, 0x98, 0x9c, 0xa0, 0xa4] {
        e.call(TIME_STAMP_SAVE, &args![me + off, buffer]);
    }
    e.call(TIMER_SAVE, &args![me + 0xa8, buffer]);
    e.call(TIMER_SAVE, &args![me + 0xb0, buffer]);
    e.call(TIME_STAMP_SAVE, &args![me + 0xc0, buffer]);
    e.call(TIMER_SAVE, &args![me + 0xb8, buffer]);
    e.call(TIME_STAMP_SAVE, &args![me + 0xe0, buffer]);
    e.with_stack(4, |e, flags| {
        let mut parts = 0u8;
        if e.get(this, CombatState::pCoverLocation) != 0 {
            parts |= 1;
        }
        if search_stamped(e, me + 0x108) {
            parts |= 2;
        }
        if e.get(this, CombatState::pSecondaryCoverLocation) != 0 {
            parts |= 4;
        }
        if search_stamped(e, me + 0x144) {
            parts |= 8;
        }
        if e.get(this, CombatState::pCombatLocation) != 0 {
            parts |= 0x10;
        }
        if e.get(this, CombatState::pLocationTarget) != 0 {
            parts |= 0x20;
        }
        e.mem.set_u8(flags.addr(), parts);
        e.call(SAVE_DATA, &args![buffer, flags, 1u32, 0u32]);
    });
    let cover = e.get(this, CombatState::pCoverLocation);
    if cover != 0 {
        fn_009a2de0(e, Ptr::new(cover), buffer);
    }
    let cover = e.get(this, CombatState::pSecondaryCoverLocation);
    if cover != 0 {
        fn_009a2de0(e, Ptr::new(cover), buffer);
    }
    let location = e.get(this, CombatState::pCombatLocation);
    if location != 0 {
        fn_009a2bf0(e, Ptr::new(location), buffer);
    }
    let target = e.get(this, CombatState::pLocationTarget);
    if target != 0 {
        fn_009a3130(e, Ptr::new(target), buffer);
    }
    if search_stamped(e, me + 0x108) {
        combat_cover_search_info_save_game(e, this.at(CombatState::CoverSearchInfo), buffer);
    }
    if search_stamped(e, me + 0x144) {
        combat_cover_search_info_save_game(
            e,
            this.at(CombatState::SecondaryCoverSearchInfo),
            buffer,
        );
    }
    let array = me.wrapping_add(0x164);
    let count = saved_count(e, array);
    e.call(SAVE_VARIABLE_SIZE, &args![buffer, count]);
    for index in 0..count {
        let entry = e.call(0x0084_4160, &args![array, index]).u32();
        fn_009a2d20(e, Ptr::new(entry), buffer);
    }
    let array = me.wrapping_add(0x174);
    let count = saved_count(e, array);
    e.call(SAVE_VARIABLE_SIZE, &args![buffer, count]);
    for index in 0..count {
        let entry = e.call(0x006d_ad90, &args![array, index]).u32();
        fn_009a2d80(e, Ptr::new(entry), buffer);
    }
    e.call(TIME_STAMP_SAVE, &args![me + 0x184, buffer]);
    data(e, 0x188, 1);
    e.with_stack(4, |e, flags| {
        let mut present = 0u8;
        for index in 0u32..5 {
            if e.mem.u32(me + 0x190 + 4 * index) != 0 {
                present |= (1u32 << index) as u8;
            }
        }
        e.mem.set_u8(flags.addr(), present);
        e.call(SAVE_DATA, &args![buffer, flags, 1u32, 0u32]);
    });
    for index in 0u32..5 {
        let record = e.mem.u32(me + 0x190 + 4 * index);
        if record != 0 {
            let item = e.mem.u32(record);
            form(e, item);
            let record = e.mem.u32(me + 0x190 + 4 * index);
            let value = e.mem.u32(record + 4);
            form(e, value);
        }
    }
    for index in 0u32..2 {
        let item = e.mem.u32(me + 0x1a4 + 4 * index);
        form(e, item);
        e.call(TIMER_SAVE, &args![me + 0x1ac + 8 * index, buffer]);
    }
    data(e, 0x1bc, 4);
    let last = e.get(this, CombatState::pLastTarget);
    form(e, last);
    data(e, 0x1c8, 1);
    data(e, 0x228, 4);
    for off in TRAILING_TIMERS {
        e.call(TIMER_SAVE, &args![me + off, buffer]);
    }
}

/// A new `CombatCoverLocation` (0x78 bytes, `00401000`, `006d3150`) loaded into `field`
/// (`009a2ea0`); a reserved one (`005dc960`) is reset (`0067fdb0` with 0) and reserved
/// again (`CombatCoverMap::ReserveCoverLocation` `009a0510`).
fn load_cover_location(
    e: &mut Engine,
    this: Ptr<CombatState>,
    field: Field<CombatState, u32>,
    buffer: u32,
) {
    let block = e.call(OPERATOR_NEW, &args![0x78u32]).u32();
    let cover = if block != 0 {
        e.call(0x006d_3150, &args![block]).u32()
    } else {
        0
    };
    e.set(this, field, cover);
    let cover = e.get(this, field);
    fn_009a2ea0(e, Ptr::new(cover), buffer);
    let cover = e.get(this, field);
    if e.call(0x005d_c960, &args![cover]).bool() {
        let cover = e.get(this, field);
        e.call(0x0067_fdb0, &args![cover, 0u32]);
        let cover = e.get(this, field);
        e.call(0x009a_0510, &args![this, cover]);
    }
}

// Translated from 009a2040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a2040 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `CombatState` load (by its body; `thiscall`, `RET 4`): reads what `009a17a0` saved,
/// resolving the weapons (`TESObjectWEAP`), the blocked explosion (`BGSExplosion`) and the
/// combat item values and usable items (`TESBoundObject`) at once (`008648a0`, `004839c0`,
/// `00ec43fb`) and leaving the other references as form ids (`008648e0`) for
/// `InitLoadGame`. An `iLocationUnreachable` of -1 becomes 0; the arrays are sized first
/// (`006f2290`, `009a3480`, `009a3610`); the optional parts are created (`006d3150`,
/// `0097ee20`, `009a08d0`) and loaded. The weapon-blocked timer and the flee-door stamp
/// are read only from buffer versions 0x14 on, the queued path check from 0xf on (the
/// buffer's slot 0).
pub fn fn_009a2040(e: &mut Engine, this: Ptr<CombatState>, buffer: u32) {
    let me = this.addr();
    let data = |e: &mut Engine, off: u32, size: u32| {
        e.call(LOAD_DATA, &args![buffer, me.wrapping_add(off), size]);
    };
    data(e, 0, 1);
    data(e, 4, 4);
    if e.get(this, CombatState::iLocationUnreachable) == -1 {
        e.set(this, CombatState::iLocationUnreachable, 0);
    }
    for index in 0u32..6 {
        let weapon = load_form(e, buffer, TES_OBJECT_WEAP_TYPE);
        e.mem.set_u32(me + 0xc + 4 * index, weapon);
    }
    let count = e.call(LOAD_VARIABLE_SIZE, &args![buffer]).u32();
    bs_simple_array_tes_object_weap_set_size(
        e,
        this.at(CombatState::EmbeddedWeaponsArray),
        count,
        1,
    );
    for index in 0..count {
        let element = e.call(0x006a_7ad0, &args![me + 0x24, index]).u32();
        let weapon = load_form(e, buffer, TES_OBJECT_WEAP_TYPE);
        e.mem.set_u32(element, weapon);
    }
    let weapon = load_form(e, buffer, TES_OBJECT_WEAP_TYPE);
    e.set(this, CombatState::pCurrentEmbeddedWeapon, weapon);
    data(e, 0x38, 4);
    for index in 0u32..7 {
        data(e, 0x3c + 4 * index, 4);
    }
    for off in [0x58u32, 0x5c, 0x64, 0x68, 0x6c] {
        data(e, off, 4);
    }
    for off in 0x70u32..0x78 {
        data(e, off, 1);
    }
    let explosion = load_form(e, buffer, BGS_EXPLOSION_TYPE);
    e.set(this, CombatState::pExplosiveProjectileBlocked, explosion);
    data(e, 0x7c, 4);
    data(e, 0x80, 4);
    data(e, 0x84, 0xc);
    data(e, 0x90, 4);
    data(e, 0xc8, 1);
    data(e, 0xc9, 1);
    data(e, 0xcc, 4);
    data(e, 0xd0, 4);
    data(e, 0xd4, 4);
    data(e, 0x8, 4);
    data(e, 0xd8, 1);
    e.call(LOAD_FORM_ID, &args![buffer, me + 0xdc]);
    for off in [0x94u32, 0x98, 0x9c, 0xa0, 0xa4] {
        e.call(TIME_STAMP_LOAD, &args![me + off, buffer]);
    }
    e.call(TIMER_LOAD, &args![me + 0xa8, buffer]);
    e.call(TIMER_LOAD, &args![me + 0xb0, buffer]);
    e.call(TIME_STAMP_LOAD, &args![me + 0xc0, buffer]);
    if e.vcall(buffer, 0x0, &args![]).u8() >= 0x14 {
        e.call(TIMER_LOAD, &args![me + 0xb8, buffer]);
        e.call(TIME_STAMP_LOAD, &args![me + 0xe0, buffer]);
    }
    let parts = e.with_stack(4, |e, flags| {
        e.mem.set_u8(flags.addr(), 0);
        e.call(LOAD_DATA, &args![buffer, flags, 1u32]);
        e.mem.u8(flags.addr())
    });
    if parts & 1 != 0 {
        load_cover_location(e, this, CombatState::pCoverLocation, buffer);
    }
    if parts & 4 != 0 {
        load_cover_location(e, this, CombatState::pSecondaryCoverLocation, buffer);
    }
    if parts & 0x10 != 0 {
        let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
        let location = if block != 0 {
            fn_0097ee20(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.set(this, CombatState::pCombatLocation, location);
        let location = e.get(this, CombatState::pCombatLocation);
        fn_009a2c60(e, Ptr::new(location), buffer);
    }
    if parts & 0x20 != 0 {
        let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
        let target = if block != 0 {
            fn_009a08d0(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.set(this, CombatState::pLocationTarget, target);
        let target = e.get(this, CombatState::pLocationTarget);
        fn_009a31b0(e, Ptr::new(target), buffer);
    }
    if parts & 2 != 0 {
        combat_cover_search_info_load_game(e, this.at(CombatState::CoverSearchInfo), buffer);
    }
    if parts & 8 != 0 {
        combat_cover_search_info_load_game(
            e,
            this.at(CombatState::SecondaryCoverSearchInfo),
            buffer,
        );
    }
    let count = e.call(LOAD_VARIABLE_SIZE, &args![buffer]).u32();
    fn_009a3480(e, Ptr::new(me + 0x164), count, 1);
    for index in 0..count {
        let entry = e.call(0x0084_4160, &args![me + 0x164, index]).u32();
        fn_009a2d50(e, Ptr::new(entry), buffer);
    }
    let count = e.call(LOAD_VARIABLE_SIZE, &args![buffer]).u32();
    fn_009a3610(e, Ptr::new(me + 0x174), count, 1);
    for index in 0..count {
        let entry = e.call(0x006d_ad90, &args![me + 0x174, index]).u32();
        fn_009a2db0(e, Ptr::new(entry), buffer);
    }
    e.call(TIME_STAMP_LOAD, &args![me + 0x184, buffer]);
    data(e, 0x188, 1);
    let present = e.with_stack(4, |e, flags| {
        e.mem.set_u8(flags.addr(), 0);
        e.call(LOAD_DATA, &args![buffer, flags, 1u32]);
        e.mem.u8(flags.addr())
    });
    for index in 0u32..5 {
        if u32::from(present) & (1u32 << index) == 0 {
            continue;
        }
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let record = if block != 0 {
            e.call(0x0096_a2d0, &args![block]).u32()
        } else {
            0
        };
        let slot = me + 0x190 + 4 * index;
        e.mem.set_u32(slot, record);
        let record = e.mem.u32(slot);
        e.call(LOAD_FORM_ID, &args![buffer, record]);
        let value = load_form(e, buffer, TES_BOUND_OBJECT_TYPE);
        let record = e.mem.u32(slot);
        e.mem.set_u32(record + 4, value);
    }
    for index in 0u32..2 {
        let item = load_form(e, buffer, TES_BOUND_OBJECT_TYPE);
        e.mem.set_u32(me + 0x1a4 + 4 * index, item);
        e.call(TIMER_LOAD, &args![me + 0x1ac + 8 * index, buffer]);
    }
    data(e, 0x1bc, 4);
    e.call(LOAD_FORM_ID, &args![buffer, me + 0x1c0]);
    data(e, 0x1c8, 1);
    if e.vcall(buffer, 0x0, &args![]).u8() >= 0xf {
        data(e, 0x228, 4);
    }
    for off in TRAILING_TIMERS {
        e.call(TIMER_LOAD, &args![me + off, buffer]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_cb70, fn_0099cb70(Ptr<CombatState>, u32)),
        entry!(0x0099_cc60, fn_0099cc60(u32, u32, u8) -> bool),
        entry!(
            0x0099_ccd0,
            combat_state_check_cover_safe(Ptr<CombatState>, u32, u32, u32, u8) -> bool
        ),
        entry!(
            0x0099_cec0,
            combat_state_check_ledge_cover_angle(Ptr<CombatState>, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x0099_d030,
            fn_0099d030(Ptr<CombatState>, u32, u32, u32, u32, u32, f32, u32) -> bool
        ),
        entry!(
            0x0099_d310,
            combat_state_build_covered_move_request(Ptr<CombatState>, u32)
        ),
        entry!(0x0099_e010, fn_0099e010(Ptr, Ptr)),
        entry!(
            0x0099_e060,
            fn_0099e060(Ptr<CombatState>, u32, u32, u32, f32, u32) -> bool
        ),
        entry!(
            0x009a_02a0,
            combat_state_check_movement(Ptr<CombatState>, u32, u8, u8) -> bool
        ),
        entry!(0x009a_0370, fn_009a0370(Ptr<CombatState>, u32)),
        entry!(0x009a_03a0, fn_009a03a0(Ptr<CombatState>, u32)),
        entry!(
            0x009a_03d0,
            combat_state_clear_cover_location(Ptr<CombatState>, u8)
        ),
        entry!(
            0x009a_0470,
            combat_state_clear_secondary_cover_location(Ptr<CombatState>, u8)
        ),
        entry!(
            0x009a_0600,
            combat_state_is_cover_location_valid(Ptr<CombatState>, u32) -> bool
        ),
        entry!(0x009a_0650, fn_009a0650(Ptr<CombatState>, u32)),
        entry!(0x009a_0680, fn_009a0680(Ptr<CombatState>)),
        entry!(
            0x009a_06c0,
            combat_state_set_combat_item(Ptr<CombatState>, u32, u32, u32)
        ),
        entry!(
            0x009a_0790,
            combat_state_set_location_target_ref(Ptr<CombatState>, u32, u8, f32, f32)
        ),
        entry!(
            0x009a_08d0,
            fn_009a08d0(Ptr<CombatLocationTarget>) -> Ptr<CombatLocationTarget>
        ),
        entry!(
            0x009a_0910,
            fn_009a0910(Ptr<CombatState>, u32, u8, f32, f32)
        ),
        entry!(0x009a_0a40, fn_009a0a40(Ptr<CombatState>)),
        entry!(0x009a_0a80, fn_009a0a80(Ptr<CombatState>) -> bool),
        entry!(
            0x009a_0b00,
            combat_state_get_location_target_distance(Ptr<CombatState>) -> f32
        ),
        entry!(0x009a_0b30, fn_009a0b30(Ptr<CombatState>) -> f32),
        entry!(0x009a_0b70, fn_009a0b70(Ptr<CombatState>) -> f32),
        entry!(0x009a_0be0, fn_009a0be0(Ptr<CombatState>) -> f32),
        entry!(0x009a_0c50, fn_009a0c50(Ptr<CombatState>, u8)),
        entry!(
            0x009a_0d00,
            combat_state_add_unreachable_cover_location(Ptr<CombatState>, u32)
        ),
        entry!(0x009a_0d70, fn_009a0d70(Ptr<CombatState>, u32) -> bool),
        entry!(
            0x009a_0e10,
            combat_state_build_unreachable_cover_locations_array(Ptr<CombatState>, u32)
        ),
        entry!(
            0x009a_0ec0,
            combat_state_add_unreachable_location(Ptr<CombatState>, u32)
        ),
        entry!(
            0x009a_0f40,
            combat_state_is_location_unreachable(Ptr<CombatState>, u32, f32) -> bool
        ),
        entry!(0x009a_1010, fn_009a1010(Ptr<CombatState>)),
        entry!(
            0x009a_1060,
            combat_state_update_threat_ratio(Ptr<CombatState>)
        ),
        entry!(0x009a_1240, fn_009a1240(u32) -> u32),
        entry!(
            0x009a_1280,
            combat_state_boost_confidence(Ptr<CombatState>, u8)
        ),
        entry!(0x009a_1380, fn_009a1380(Ptr<CombatState>) -> bool),
        entry!(0x009a_1400, fn_009a1400(Ptr<CombatState>, u32) -> bool),
        entry!(
            0x009a_2a30,
            combat_state_init_load_game(Ptr<CombatState>, u32)
        ),
        entry!(0x009a_2bf0, fn_009a2bf0(Ptr<CombatLocation>, u32)),
        entry!(0x009a_2c60, fn_009a2c60(Ptr<CombatLocation>, u32)),
        entry!(0x009a_2cc0, fn_009a2cc0(Ptr<CombatLocation>, u32)),
        entry!(0x009a_2d20, fn_009a2d20(Ptr, u32)),
        entry!(0x009a_2d50, fn_009a2d50(Ptr, u32)),
        entry!(0x009a_2d80, fn_009a2d80(Ptr, u32)),
        entry!(0x009a_2db0, fn_009a2db0(Ptr, u32)),
        entry!(0x009a_2de0, fn_009a2de0(Ptr, u32)),
        entry!(0x009a_2ea0, fn_009a2ea0(Ptr, u32)),
        entry!(
            0x009a_2f50,
            combat_cover_search_info_save_game(Ptr<CombatCoverSearchInfo>, u32)
        ),
        entry!(
            0x009a_3050,
            combat_cover_search_info_load_game(Ptr<CombatCoverSearchInfo>, u32)
        ),
        entry!(0x009a_3130, fn_009a3130(Ptr<CombatLocationTarget>, u32)),
        entry!(0x009a_31b0, fn_009a31b0(Ptr<CombatLocationTarget>, u32)),
        entry!(0x009a_3220, fn_009a3220(Ptr<CombatLocationTarget>, u32)),
        entry!(0x009a_32a0, fn_009a32a0(Ptr<BSSimpleArray>, u32, u8)),
        entry!(0x009a_33e0, fn_009a33e0(Ptr) -> Ptr),
        entry!(0x009a_3430, fn_009a3430(Ptr) -> Ptr),
        entry!(0x009a_3480, fn_009a3480(Ptr<BSSimpleArray>, u32, u8)),
        entry!(0x009a_35c0, fn_009a35c0(Ptr) -> Ptr),
        entry!(0x009a_3610, fn_009a3610(Ptr<BSSimpleArray>, u32, u8)),
        entry!(0x009a_3750, fn_009a3750(Ptr, u32, u32) -> Ptr),
        entry!(
            0x009a_3780,
            fn_009a3780(Ptr<BSSimpleArray>, Ptr<BSSimpleArray>)
        ),
        entry!(0x009a_37e0, fn_009a37e0(Ptr, u32, u32) -> Ptr),
        entry!(
            0x009a_3850,
            bs_simple_array_tes_object_weap_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x009a_3880,
            bs_simple_array_unreachable_cover_location_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x009a_38b0,
            bs_simple_array_unreachable_location_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x009a_38e0,
            bs_simple_array_weapon_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x009a_39f0, fn_009a39f0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x009a_3bc0, fn_009a3bc0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(
            0x009d_6c80,
            combat_state_unreserve_cover_location(Ptr<CombatState>)
        ),
        entry!(
            0x009d_7c20,
            combat_state_reserve_cover_location(Ptr<CombatState>)
        ),
        entry!(0x0099_d480, fn_0099d480(u32, f32, u8) -> bool),
        entry!(
            0x0099_e190,
            combat_state_find_best_dodge_location(
                Ptr<CombatState>,
                Ptr<CombatLocation>,
                u32,
                u32,
                f32,
                f32,
            ) -> bool
        ),
        entry!(
            0x0099_e840,
            fn_0099e840(Ptr<CombatState>, f32, f32, f32, f32, Ptr<CombatLocation>, u8) -> bool
        ),
        entry!(
            0x0099_ebe0,
            combat_state_find_best_flee_location(
                Ptr<CombatState>,
                u32,
                Ptr<CombatLocation>,
                u32,
            ) -> bool
        ),
        entry!(
            0x0099_f480,
            combat_state_find_los_location(Ptr<CombatState>, Ptr<CombatLocation>, u32) -> bool
        ),
        entry!(0x0099_f6d0, fn_0099f6d0(Ptr<CombatState>) -> bool),
        entry!(0x009a_17a0, fn_009a17a0(Ptr<CombatState>, u32)),
        entry!(0x009a_2040, fn_009a2040(Ptr<CombatState>, u32)),
        entry!(0x009a_3830, fn_009a3830(Ptr, Ptr) -> bool),
        entry!(0x009a_3280, fn_009a3280(Ptr, u32) -> Ptr),
        entry!(
            0x006f2290,
            bs_simple_array_tes_object_weap_set_size(Ptr<BSSimpleArray>, u32, u8)
        ),
        entry!(
            0x0097ee20,
            fn_0097ee20(Ptr<CombatLocation>) -> Ptr<CombatLocation>
        ),
        entry!(
            0x009973a0,
            fn_009973a0(Ptr<CombatLocation>, Ptr<CombatLocation>)
        ),
        entry!(
            0x00997400,
            combat_location_set_ref(Ptr<CombatLocation>, u32)
        ),
        entry!(
            0x00997440,
            combat_location_update_ref_location(Ptr<CombatLocation>)
        ),
        entry!(0x00997480, fn_00997480(Ptr, Ptr)),
        entry!(
            0x00997520,
            fn_00997520(Ptr<CombatCoverSearchInfo>, Ptr<CombatCoverSearchInfo>)
        ),
        entry!(0x009975c0, fn_009975c0(Ptr<CombatCoverSearchInfo>)),
        entry!(
            0x00997600,
            fn_00997600(Ptr<CombatState>, u32, u32) -> Ptr<CombatState>
        ),
        entry!(0x00997aa0, fn_00997aa0(Ptr<CombatState>)),
        entry!(0x00997ba0, fn_00997ba0(Ptr<CombatState>, u32, u32)),
        entry!(0x00997bf0, fn_00997bf0(Ptr<CombatState>)),
        entry!(0x00997c10, fn_00997c10(Ptr<CombatState>)),
        entry!(0x00997c30, fn_00997c30(Ptr<CombatState>)),
        entry!(0x00997cb0, fn_00997cb0(Ptr<CombatState>)),
        entry!(0x00997cf0, fn_00997cf0(Ptr<CombatState>)),
        entry!(0x00998980, fn_00998980(Ptr<CombatState>) -> bool),
        entry!(0x009989d0, fn_009989d0(u32) -> bool),
        entry!(0x009989f0, fn_009989f0(u32) -> u8),
        entry!(0x00998a10, fn_00998a10(u32) -> u8),
        entry!(0x00998a30, fn_00998a30(u32) -> u8),
        entry!(0x00998a50, fn_00998a50(Ptr<CombatState>)),
        entry!(0x009990c0, fn_009990c0(Ptr<CombatTimer>) -> bool),
        entry!(0x00999100, fn_00999100(Ptr<CombatState>)),
        entry!(0x00999120, fn_00999120(u32) -> bool),
        entry!(0x00999160, fn_00999160(Ptr<CombatState>)),
        entry!(0x00999260, fn_00999260(Ptr<CombatState>)),
        entry!(0x009993c0, fn_009993c0(Ptr<CombatState>)),
        entry!(0x0099a2a0, fn_0099a2a0(Ptr<CombatState>)),
        entry!(0x0099a2d0, fn_0099a2d0(u32) -> u32),
        entry!(0x0099a360, fn_0099a360(Ptr<CombatState>, u32, u32) -> bool),
        entry!(0x0099a3e0, fn_0099a3e0(u32) -> bool),
        entry!(0x0099a420, fn_0099a420(Ptr<CombatState>, u32, u8)),
        entry!(0x0099a530, fn_0099a530(Ptr<CombatState>)),
        entry!(0x0099a5e0, fn_0099a5e0(Ptr<CombatState>, u32)),
        entry!(0x0099a650, fn_0099a650(Ptr<CombatState>, u32, u8)),
        entry!(0x0099a680, fn_0099a680(Ptr<CombatState>, u32)),
        entry!(0x0099a6b0, fn_0099a6b0(Ptr<CombatState>, u32, u32)),
        entry!(0x0099a720, fn_0099a720(Ptr<CombatState>)),
        entry!(
            0x0099a780,
            combat_state_pre_activate_reference(Ptr<CombatState>, u32)
        ),
        entry!(
            0x0099a7d0,
            fn_0099a7d0(Ptr<CombatState>, u32, u32, u32, u32)
        ),
        entry!(0x0099a840, fn_0099a840(Ptr<CombatState>, u32)),
        entry!(0x0099a8d0, fn_0099a8d0(Ptr<CombatState>)),
        entry!(0x0099a9a0, fn_0099a9a0(Ptr<CombatState>)),
        entry!(0x0099aa10, fn_0099aa10(Ptr<CombatState>, u32, u32)),
        entry!(0x0099aa80, pathing_cover_location_pathing_cover_location(Ptr, Ptr) -> Ptr),
        entry!(
            0x0099ab50,
            fn_0099ab50(Ptr<CombatState>, Ptr<NiPoint3>) -> bool
        ),
        entry!(
            0x0099abc0,
            combat_state_get_last_seen_target_firing_location(
                Ptr<CombatState>,
                Ptr<NiPoint3>,
            ) -> bool
        ),
        entry!(
            0x0099ac20,
            combat_state_find_combat_location(Ptr<CombatState>, u32) -> bool
        ),
        entry!(0x0099b520, fn_0099b520(Ptr<CombatLocation>, u32)),
        entry!(0x0099b540, fn_0099b540(Ptr<CombatLocation>, u32, u8)),
        entry!(0x0099b580, fn_0099b580(Ptr<CombatLocation>) -> u32),
        entry!(
            0x0099b5a0,
            fn_0099b5a0(Ptr<CombatState>, Ptr<CombatLocation>, u32, u32) -> bool
        ),
        entry!(0x0099b8b0, fn_0099b8b0(Ptr<CombatLocation>, u32, u32)),
        entry!(
            0x0099b8e0,
            fn_0099b8e0(Ptr<CombatState>, u32, Ptr<CombatCoverSearchInfo>, u32, u32, u32) -> bool
        ),
        entry!(0x0099c1f0, fn_0099c1f0(u32) -> u8),
        entry!(0x0099c210, fn_0099c210(u32)),
        (0x0099c230, fn_0099c230_entry as AbiFn),
        entry!(0x0099ca00, fn_0099ca00(Ptr<CombatState>, u32, u32) -> bool),
        entry!(0x0099cb00, fn_0099cb00(u32, u32) -> bool),
        entry!(0x009a_3410, fn_009a3410(Ptr)),
        entry!(0x009a_3460, fn_009a3460(Ptr)),
        entry!(0x009a_35f0, fn_009a35f0(Ptr)),
        entry!(0x009a_3810, fn_009a3810(Ptr)),
        entry!(0x0099_e040, fn_0099e040(Ptr) -> f32),
        entry!(0x009a_1260, fn_009a1260(Ptr) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Every callee outside this file that the translations call directly.
    const EXTERNAL: &[u32] = &[
        0x0040_1000,
        0x0040_1030,
        0x0040_1050,
        0x0040_1170,
        0x0040_30d0,
        0x0040_3d30,
        0x0040_4010,
        0x0040_4eb0,
        0x0040_4ee0,
        0x0040_5c00,
        0x0040_ebd0,
        0x0041_6870,
        0x0041_8b10,
        0x0041_fd00,
        0x0042_fc00,
        0x0043_00a0,
        0x0043_7b90,
        0x0043_9090,
        0x0043_90c0,
        0x0043_9ef0,
        0x0044_2a80,
        0x0044_ddc0,
        0x0044_edb0,
        0x0045_36e0,
        0x0045_3700,
        0x0045_7990,
        0x0046_1580,
        0x0046_e8c0,
        0x0047_6b70,
        0x0048_3710,
        0x0049_5460,
        0x0049_97b0,
        0x004a_7290,
        0x004a_7bd0,
        0x004b_c550,
        0x004b_cdb0,
        0x004b_f220,
        0x004c_7300,
        0x004c_8f30,
        0x004d_4830,
        0x004d_ff00,
        0x004f_9bf0,
        0x004f_d3c0,
        0x004f_d3e0,
        0x004f_f7e0,
        0x0050_76c0,
        0x0051_b840,
        0x0052_2c80,
        0x0052_4b60,
        0x0052_5980,
        0x0052_5a90,
        0x0052_7270,
        0x0052_72b0,
        0x0052_7a40,
        0x0052_7ac0,
        0x0054_07b0,
        0x0055_9450,
        0x0057_1600,
        0x0057_5650,
        0x0057_5ca0,
        0x0059_5c80,
        0x005a_4320,
        0x005b_5e40,
        0x005c_3420,
        0x005d_c960,
        0x005e_3fc0,
        0x0060_0ab0,
        0x0062_96b0,
        0x0064_6060,
        0x0067_a700,
        0x0068_15c0,
        0x0069_6540,
        0x006a_7ad0,
        0x006a_7af0,
        0x006a_7f50,
        0x006b_3160,
        0x006b_9130,
        0x006d_3150,
        0x006d_3280,
        0x006d_32e0,
        0x006d_3370,
        0x006d_3ac0,
        0x006d_62e0,
        0x006d_ad40,
        0x006d_ad70,
        0x006d_ad90,
        0x006d_bde0,
        0x006d_cc40,
        0x006d_ce10,
        0x006d_d0d0,
        0x006d_ed70,
        0x006e_4080,
        0x006e_4320,
        0x006e_4340,
        0x006e_4380,
        0x006e_43a0,
        0x006e_43c0,
        0x006e_43e0,
        0x006e_4400,
        0x006e_4430,
        0x006e_4570,
        0x006e_4650,
        0x006e_4740,
        0x006e_4820,
        0x006e_4840,
        0x006e_4d00,
        0x006e_9bd0,
        0x006e_bf60,
        0x006e_bf90,
        0x006e_cd40,
        0x0070_31c0,
        0x0072_6070,
        0x0072_6bf0,
        0x0072_ba80,
        0x0078_81d0,
        0x007a_f430,
        0x007c_b2e0,
        0x0080_0190,
        0x0081_9a50,
        0x0082_5c00,
        0x0084_54f0,
        0x0084_e3a0,
        0x0088_46c0,
        0x0089_4900,
        0x008a_0b10,
        0x008a_6840,
        0x008a_8840,
        0x008a_8870,
        0x008b_c9d0,
        0x008b_e1b0,
        0x008b_e420,
        0x008d_8520,
        0x008e_eaa0,
        0x008f_6630,
        0x0090_52c0,
        0x0090_57d0,
        0x0090_6f60,
        0x0093_1850,
        0x0094_4460,
        0x0096_11e0,
        0x0096_8630,
        0x0096_a5f0,
        0x0097_9f70,
        0x0097_ac90,
        0x0097_acb0,
        0x0097_b940,
        0x0097_bd00,
        0x0097_c150,
        0x0097_cfc0,
        0x0097_ede0,
        0x0097_ee60,
        0x0097_ee90,
        0x0097_f200,
        0x0097_f220,
        0x0097_f310,
        0x0097_fb20,
        0x0097_fd40,
        0x0098_0070,
        0x0098_0700,
        0x0098_1300,
        0x0098_1340,
        0x0098_1380,
        0x0098_1520,
        0x0098_1940,
        0x0098_1970,
        0x0098_1990,
        0x0098_2be0,
        0x0098_35d0,
        0x0098_3970,
        0x0098_87b0,
        0x0098_9010,
        0x0098_90e0,
        0x0098_9140,
        0x0098_9700,
        0x0098_9e20,
        0x0098_9eb0,
        0x0098_bc00,
        0x0098_bd70,
        0x0098_cd50,
        0x0098_de00,
        0x0098_f1f0,
        0x0099_2720,
        0x0099_28c0,
        0x0099_5890,
        0x0099_5980,
        0x0099_59c0,
        0x0099_62f0,
        0x0099_cc60,
        0x0099_ccd0,
        0x0099_cec0,
        0x0099_d030,
        0x0099_e190,
        0x0099_ebe0,
        0x0099_f480,
        0x0099_f6d0,
        0x009a_0260,
        0x009a_02a0,
        0x009a_03d0,
        0x009a_0470,
        0x009a_0680,
        0x009a_0790,
        0x009a_0910,
        0x009a_0a40,
        0x009a_0a80,
        0x009a_0b00,
        0x009a_0b70,
        0x009a_0c50,
        0x009a_0d70,
        0x009a_1010,
        0x009a_1060,
        0x009a_1280,
        0x009a_1380,
        0x009a_1400,
        0x009a_33e0,
        0x009a_3410,
        0x009a_3430,
        0x009a_3460,
        0x009a_35c0,
        0x009a_35f0,
        0x009a_3750,
        0x009a_3780,
        0x009a_37e0,
        0x009a_3810,
        0x009a_5670,
        0x009a_7670,
        0x009a_7940,
        0x009a_7e00,
        0x009a_7f00,
        0x009a_8050,
        0x009a_8460,
        0x009a_9180,
        0x009a_93a0,
        0x009a_9630,
        0x00aa_6060,
    ];

    /// A shared vtable whose slot `s` calls `SLOTS + s` (all doubled).
    const VTABLE: u32 = 0x0200_0000;
    const SLOTS: u32 = 0x0300_0000;
    /// The start-time word that makes the timer double report "expired".
    const EXPIRED: u32 = 1;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| ret(eax));
    }

    fn stub_st0(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| st0(value));
    }

    /// Returns `results` in turn (the last one again afterwards).
    fn sequence(e: &mut Engine, addr: u32, results: Vec<u32>) {
        let mut next = 0;
        e.register_double(addr, move |_, _| {
            let value = results[next.min(results.len() - 1)];
            next += 1;
            ret(value)
        });
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

    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size.max(0x10));
        e.mem.set_u32(object, VTABLE);
        object
    }

    fn put_f32(e: &mut Engine, addr: u32, value: f32) {
        e.mem.set_f32(addr, value);
    }

    fn put_f64(e: &mut Engine, addr: u32, value: f64) {
        e.mem.set_f64(addr, value);
    }

    /// Every outside callee doubled (returning 0), the constants the code
    /// reads, and working doubles for the small getters, time stamps and
    /// timers (game time 5).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in EXTERNAL {
            stub(&mut e, addr, 0);
        }
        let mut slots = vec![];
        for slot in (0..0x800).step_by(4) {
            slots.push(SLOTS + slot);
            stub(&mut e, SLOTS + slot, 0);
        }
        e.put_vtable(VTABLE, &slots);
        e.map(0x0101_0000, 0x0008_0000);
        e.map(0x0108_d000, 0x2000);
        e.map(0x011a_4000, 0x2000);
        e.map(0x011c_e000, 0x2000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x011f_1000, 0x6000);
        for (addr, value) in [
            (MAX_FLOAT, f32::MAX),
            (HUNDRED, 100.0),
            (MINUS_ONE, -1.0),
            (TWO, 2.0),
            (QUARTER, 0.25),
            (HALF, 0.5),
            (TENTH, 0.1),
            (FIVE, 5.0),
            (TEN, 10.0),
            (THREE, 3.0),
            (FORCED_WEAPON_DPS, 9_999_999.0),
            (DODGE_DISTANCE, 128.0),
            (FIVE_HUNDRED_TWELVE, 512.0),
            (TWO_THOUSAND_FORTY_EIGHT, 2048.0),
            (TEN_THOUSAND, 10000.0),
            (FOUR_THOUSAND_NINETY_SIX, 4096.0),
            (TWO_HUNDRED_FIFTY_SIX, 256.0),
            (LOCATION_TARGET_MAX_DISTANCE, 1024.0),
            (LOCATION_TARGET_SECOND_DISTANCE, 768.0),
            (LOCATION_TARGET_FIRST_DISTANCE, 384.0),
        ] {
            put_f32(&mut e, addr, value);
        }
        for (addr, value) in [
            (ZERO_DOUBLE, 0.0),
            (ONE_DOUBLE, 1.0),
            (TWO_HUNDRED_FIFTY_SIX_DOUBLE, 256.0),
            (HUNDRED_DOUBLE, 100.0),
            (HUNDRED_TWENTY_DOUBLE, 120.0),
            (TEN_DOUBLE, 10.0),
            (SIXTY_FIVE_THOUSAND_DOUBLE, 65536.0),
            (CURRENT_WEAPON_BONUS_DOUBLE, 1_000_000.0),
            (OUT_OF_RANGE_DIVISOR_DOUBLE, 10000.0),
            (TWO_DOUBLE, 2.0),
            (THOUSAND_DOUBLE, 1000.0),
            (THREE_DOUBLE, 3.0),
            (MINUS_ONE_DOUBLE, -1.0),
        ] {
            put_f64(&mut e, addr, value);
        }
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        e.register(CONTROLLER_ATTACKER, |e, a| ret(e.mem.u32(a[0] + 0xbc)));
        e.register(CONTROLLER_TARGET, |e, a| ret(e.mem.u32(a[0] + 0xc0)));
        e.register(CONTROLLER_GROUP, |e, a| ret(e.mem.u32(a[0] + 0x80)));
        e.register(GAME_TIME, |_, _| st0(5.0));
        e.register(TIME_STAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(TIME_STAMP_ELAPSED, |e, a| {
            st0((5.0 - e.mem.f32(a[0])) as f64)
        });
        e.register(TIMER_START, |e, a| {
            e.mem.set_f32(a[0], 5.0);
            e.mem.set_u32(a[0] + 4, a[1]);
            ret(0)
        });
        e.register(TIMER_EXPIRED, |e, a| {
            ret((e.mem.u32(a[0]) == EXPIRED) as u32)
        });
        e.register(TIMER_RUNNING, |e, a| {
            ret((e.mem.f32(a[0] + 4) != 0.0) as u32)
        });
        e.register(TIMER_STOP, |e, a| {
            e.mem.set_f32(a[0] + 4, 0.0);
            ret(0)
        });
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e
    }

    struct Fixture {
        state: Ptr<CombatState>,
        controller: u32,
        attacker: u32,
        target: u32,
    }

    /// A state with its controller, an attacker (with an object at +0xa4)
    /// and, if asked, a target; the attacker's process is an object too.
    fn fixture(e: &mut Engine, with_target: bool) -> Fixture {
        let state: Ptr<CombatState> = e.new_object();
        let controller = e.mem.alloc(0x188);
        let attacker = object(e, 0x200);
        e.mem.set_u32(attacker + 0xa4, VTABLE);
        let target = if with_target { object(e, 0x200) } else { 0 };
        let group = object(e, 0x40);
        e.mem.set_u32(controller + 0xbc, attacker);
        e.mem.set_u32(controller + 0xc0, target);
        e.mem.set_u32(controller + 0x80, group);
        e.set(state, CombatState::pCombatController, controller);
        let process = object(e, 0x40);
        stub(e, 0x008d_8520, process);
        Fixture {
            state,
            controller,
            attacker,
            target,
        }
    }

    fn timer(state: Ptr<CombatState>, off: u32) -> u32 {
        state.addr() + off
    }

    #[test]
    fn set_size_grows_an_empty_array_through_its_vtable() {
        let mut e = engine();
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr(), VTABLE);
        stub(&mut e, SLOTS + 4, 0x5000);
        let log = logged(&mut e, |e| {
            bs_simple_array_tes_object_weap_set_size(e, array, 3, 0)
        });
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0x5000);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 3);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        assert_eq!(
            calls(&log, 0x0072_6bf0),
            vec![vec![array.addr(), 0x5000, 3]]
        );

        // Shrinking to a quarter of the reservation with `shrink` reallocates.
        e.set(array, BSSimpleArray::iReservedSize, 8);
        e.set(array, BSSimpleArray::iSize, 8);
        let log = logged(&mut e, |e| {
            bs_simple_array_tes_object_weap_set_size(e, array, 2, 1)
        });
        assert_eq!(
            calls(&log, 0x0072_ba80),
            vec![vec![array.addr(), 0x5008, 6]]
        );
        assert_eq!(calls(&log, 0x0042_fc00), vec![vec![array.addr(), 2, 2]]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 2);

        // Size 0 hands over with the shrink flag.
        let log = logged(&mut e, |e| {
            bs_simple_array_tes_object_weap_set_size(e, array, 0, 1)
        });
        assert_eq!(calls(&log, 0x0084_54f0), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn combat_location_initialisation_sets_defaults() {
        let mut e = engine();
        let location: Ptr<CombatLocation> = e.new_object();
        e.set(location, CombatLocation::pRef, 7);
        assert_eq!(fn_0097ee20(&mut e, location), location);
        assert_eq!(e.get(location, CombatLocation::pRef), 0);
        assert_eq!(e.get(location, CombatLocation::iLocation), -1);
        assert_eq!(e.get(location, CombatLocation::iFlags), 0);
    }

    #[test]
    fn combat_location_copy_quiets_the_radius() {
        let mut e = engine();
        let (to, from): (Ptr<CombatLocation>, Ptr<CombatLocation>) =
            (e.new_object(), e.new_object());
        for i in 0..8 {
            e.mem.set_u32(from.addr() + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(from.addr() + 0x18, 0x7f80_0001);
        fn_009973a0(&mut e, to, from);
        assert_eq!(e.mem.u32(to.addr() + 0xc), 0x103);
        assert_eq!(e.mem.u32(to.addr() + 0x18), 0x7fc0_0001);
        assert_eq!(e.get(to, CombatLocation::iFlags), 0x107);
    }

    fn world_location_double(e: &mut Engine) {
        e.register(GET_WORLD_LOCATION, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[1] + 4 * i, a[0] + i);
            }
            ret(a[1])
        });
    }

    #[test]
    fn set_ref_copies_the_reference_location() {
        let mut e = engine();
        world_location_double(&mut e);
        let location: Ptr<CombatLocation> = e.new_object();
        combat_location_set_ref(&mut e, location, 0x40);
        assert_eq!(e.get(location, CombatLocation::pRef), 0x40);
        assert_eq!(e.mem.u32(location.addr() + 0xc), 0x43);
    }

    #[test]
    fn update_ref_location_needs_a_reference() {
        let mut e = engine();
        world_location_double(&mut e);
        let location: Ptr<CombatLocation> = e.new_object();
        let log = logged(&mut e, |e| combat_location_update_ref_location(e, location));
        assert!(calls(&log, GET_WORLD_LOCATION).is_empty());
        e.set(location, CombatLocation::pRef, 0x80);
        combat_location_update_ref_location(&mut e, location);
        assert_eq!(e.mem.u32(location.addr()), 0x80);
    }

    #[test]
    fn cover_location_copy_copies_fields_and_the_pathing_location() {
        let mut e = engine();
        let (to, from) = (Ptr::new(e.mem.alloc(0x78)), Ptr::new(e.mem.alloc(0x78)));
        e.mem.set_u8(from.addr() + 2, 9);
        e.mem.set_u32(from.addr() + 0x6c, 0x1234);
        e.mem.set_u16(from.addr() + 0x74, 0xbeef);
        let log = logged(&mut e, |e| fn_00997480(e, to, from));
        assert_eq!(e.mem.u8(to.addr() + 2), 9);
        assert_eq!(e.mem.u32(to.addr() + 0x6c), 0x1234);
        assert_eq!(e.mem.u16(to.addr() + 0x74), 0xbeef);
        assert_eq!(
            calls(&log, 0x006e_4430),
            vec![vec![to.addr() + 0x14, from.addr() + 0x14]]
        );
    }

    #[test]
    fn cover_search_info_assignment_and_reset() {
        let mut e = engine();
        let (to, from): (Ptr<CombatCoverSearchInfo>, Ptr<CombatCoverSearchInfo>) =
            (e.new_object(), e.new_object());
        e.set(from, CombatCoverSearchInfo::fRadius, 3.0);
        e.set(from, CombatCoverSearchInfo::bUsedLastSeenLocation, 1);
        let log = logged(&mut e, |e| fn_00997520(e, to, from));
        assert_eq!(e.get(to, CombatCoverSearchInfo::fRadius), 3.0);
        assert_eq!(e.get(to, CombatCoverSearchInfo::bUsedLastSeenLocation), 1);
        assert_eq!(
            calls(&log, 0x009a_3780),
            vec![vec![from.addr() + 0x20, to.addr() + 0x20]]
        );
        let log = logged(&mut e, |e| fn_00997520(e, to, Ptr::NULL));
        assert_eq!(e.get(to, CombatCoverSearchInfo::fRadius), 0.0);
        assert_eq!(
            e.get(to, CombatCoverSearchInfo::fTimeStamp),
            f32::MAX.to_bits() ^ 0x8000_0000
        );
        assert_eq!(calls(&log, 0x006d_bde0), vec![vec![to.addr() + 0x20, 1]]);
    }

    #[test]
    fn reset_cover_search_info_stamps_minus_max() {
        let mut e = engine();
        let info: Ptr<CombatCoverSearchInfo> = e.new_object();
        e.set(info, CombatCoverSearchInfo::fRadius, 9.0);
        fn_009975c0(&mut e, info);
        assert_eq!(e.get(info, CombatCoverSearchInfo::fRadius), 0.0);
        assert_eq!(
            f32::from_bits(e.get(info, CombatCoverSearchInfo::fTimeStamp)),
            -f32::MAX
        );
    }

    #[test]
    fn constructor_initialises_the_state() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        put_f32(&mut e, 0x011c_f804 + 4, 8.0);
        e.register(RANDOM_FLOAT, |_, a| {
            st0(f32::from_bits(a[0]) as f64 + f32::from_bits(a[1]) as f64)
        });
        put_f32(&mut e, 0x011c_fdcc + 4, 0.25);
        put_f32(&mut e, 0x011c_f7ec + 4, 0.5);
        let log = logged(&mut e, |e| {
            assert_eq!(fn_00997600(e, state, 0x4444, 0), state);
        });
        assert_eq!(e.get(state, CombatState::fThreatRatio), 100.0);
        assert_eq!(e.get(state, CombatState::cTargetFlushed), -1);
        assert_eq!(e.get(state, CombatState::eTargetSegmentInView), 3);
        assert_eq!(e.get(state, CombatState::fCombatStrength), -1.0);
        assert_eq!(e.get(state, CombatState::cLastDodgeDirection), 0xff);
        assert_eq!(e.get(state, CombatState::pCombatController), 0x4444);
        assert_eq!(e.get(state, CombatState::bInitializing), 1);
        assert_eq!(e.get(state, CombatState::iQueuedPathCheck), -1);
        assert_eq!(e.get(state, CombatState::fConfidenceModifier), 0.75);
        assert_eq!(calls(&log, TIME_STAMP_SET).len(), 6);
        assert_eq!(calls(&log, 0x0090_6f60).len(), 14);
        assert_eq!(
            calls(&log, 0x0040_1050),
            vec![vec![state.addr() + 0x1ac, 8, 2, 0x0090_6f60]]
        );
        assert_eq!(
            calls(&log, TIMER_START),
            vec![vec![state.addr() + 0x20c, 8.0f32.to_bits()]]
        );
    }

    #[test]
    fn destructor_releases_everything() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_00997aa0(e, state));
        assert_eq!(calls(&log, 0x0098_1340).len(), 5);
        assert_eq!(calls(&log, 0x009a_3410), vec![vec![state.addr() + 0x24]]);
        assert_eq!(calls(&log, 0x006e_4d00).len(), 2);
        assert_eq!(
            calls(&log, CLEAR_COVER_LOCATION),
            vec![vec![state.addr(), 0]]
        );
    }

    #[test]
    fn clearing_cover_needs_first_zero_and_second_set() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_00997ba0(e, state, 1, 1));
        assert!(calls(&log, CLEAR_COVER_LOCATION).is_empty());
        let log = logged(&mut e, |e| fn_00997ba0(e, state, 0, 1));
        assert_eq!(calls(&log, CLEAR_COVER_LOCATION).len(), 1);
        assert_eq!(calls(&log, 0x006d_bde0).len(), 2);
        assert_eq!(calls(&log, 0x009a_0680), vec![vec![state.addr()]]);
    }

    #[test]
    fn cover_search_resets_pick_their_member() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_00997bf0(e, state));
        assert_eq!(
            calls(&log, 0x006d_bde0),
            vec![vec![state.addr() + 0xec + 0x20, 1]]
        );
        let log = logged(&mut e, |e| fn_00997c10(e, state));
        assert_eq!(
            calls(&log, 0x006d_bde0),
            vec![vec![state.addr() + 0x148, 1]]
        );
    }

    #[test]
    fn first_update_sets_confidence_and_ends_initialisation() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub(&mut e, 0x0097_f310, 1);
        stub(&mut e, 0x0093_1850, 1);
        stub(&mut e, SLOTS + 8, 3);
        e.set(f.state, CombatState::bInitializing, 1);
        let log = logged(&mut e, |e| fn_00997c30(e, f.state));
        assert_eq!(e.get(f.state, CombatState::bHasHandToHandEquip), 1);
        assert_eq!(e.get(f.state, CombatState::cStartingConfidence), 3);
        assert_eq!(e.get(f.state, CombatState::cCurrentConfidence), 3);
        assert_eq!(e.get(f.state, CombatState::bInitializing), 0);
        assert_eq!(calls(&log, 0x0097_f310), vec![vec![f.attacker, 0]]);
        assert_eq!(calls(&log, SLOTS + 8), vec![vec![f.attacker + 0xa4, 1]]);
    }

    #[test]
    fn update_chooses_the_reduced_update_outside_the_high_process() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub(&mut e, 0x0093_1850, 2);
        let strength = timer(f.state, 0x204);
        e.mem.set_u32(strength, EXPIRED);
        let log = logged(&mut e, |e| fn_00997cb0(e, f.state));
        assert_eq!(calls(&log, 0x009a_1010), vec![vec![f.state.addr()]]);
        assert!(calls(&log, 0x009a_0c50).is_empty());
    }

    #[test]
    fn full_update_without_a_target_resets_the_target_information() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.set(f.state, CombatState::iTargetDetectionLevel, 3);
        e.set(f.state, CombatState::bTargetSpaceDifferent, 1);
        let log = logged(&mut e, |e| fn_00997cf0(e, f.state));
        let minus_max = f32::MAX.to_bits() ^ 0x8000_0000;
        assert_eq!(e.get(f.state, CombatState::iTargetDetectionLevel), -1);
        assert_eq!(e.get(f.state, CombatState::cTargetFlushed), -1);
        assert_eq!(e.get(f.state, CombatState::bTargetSpaceDifferent), 0);
        assert_eq!(
            e.get(f.state, CombatState::fLastTargetDetectedTimeStamp),
            minus_max
        );
        // `006a7f50` gives 0, not `-FLT_MAX`: the firing stamp is reset.
        assert_eq!(
            e.get(f.state, CombatState::fFiringStartedTimeStamp),
            minus_max
        );
        assert_eq!(calls(&log, 0x009a_0c50), vec![vec![f.state.addr(), 0]]);
        assert_eq!(e.get(f.state, CombatState::fThreatScore), 0.0);
    }

    #[test]
    fn full_update_with_a_target_in_view_records_where_it_was_seen() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let point = e.mem.alloc(12);
        e.mem.set_f32(point, 1.0);
        e.mem.set_f32(point + 8, 3.0);
        stub(&mut e, SLOTS + 0x1f4, point);
        e.set(f.state, CombatState::bTarget360LineOfSight, 1);
        e.set(f.state, CombatState::pLastTarget, f.target);
        let log = logged(&mut e, |e| fn_00997cf0(e, f.state));
        assert_eq!(
            e.get(f.state, CombatState::fLastTarget360LineOfSightTimeStamp),
            5.0f32.to_bits()
        );
        assert_eq!(e.mem.f32(f.state.addr() + 0x84), 1.0);
        assert_eq!(e.mem.f32(f.state.addr() + 0x8c), 3.0);
        // Flushed state -1, then 0 since the 360-degree stamp is fresh.
        assert_eq!(e.get(f.state, CombatState::cTargetFlushed), 0);
        // The target did not change and no detection update was asked for.
        assert!(calls(&log, 0x0098_87b0).is_empty());
        assert_eq!(e.get(f.state, CombatState::pLastTarget), f.target);
    }

    #[test]
    fn line_of_sight_stamp_check_compares_with_the_setting() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        put_f32(&mut e, 0x011c_ea98 + 4, 2.0);
        e.mem.set_f32(state.addr() + 0x98, 4.0);
        assert!(fn_00998980(&mut e, state));
        e.mem.set_f32(state.addr() + 0x98, 1.0);
        assert!(!fn_00998980(&mut e, state));
    }

    #[test]
    fn controller_flag_getters_read_their_bytes() {
        let mut e = engine();
        let controller = e.mem.alloc(0x188);
        e.mem.set_u8(controller + 0xc5, 1);
        e.mem.set_u8(controller + 0xc7, 2);
        e.mem.set_u8(controller + 0xd4, 3);
        assert_eq!(fn_00998a10(&mut e, controller), 1);
        assert_eq!(fn_009989f0(&mut e, controller), 2);
        assert_eq!(fn_00998a30(&mut e, controller), 3);
        stub(&mut e, 0x0099_28c0, 1);
        let log = logged(&mut e, |e| assert!(fn_009989d0(e, 0x50)));
        assert_eq!(calls(&log, 0x0099_28c0), vec![vec![0x50, 0x10]]);
    }

    #[test]
    fn second_update_restarts_the_better_weapon_timer_and_ends_fleeing() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        put_f32(&mut e, 0x011c_f9f8 + 4, 6.0);
        e.mem.set_u32(timer(f.state, 0x1d4), EXPIRED);
        e.set(f.state, CombatState::fMaxCombatWeaponDPS, 10.0);
        e.set(f.state, CombatState::fDisarmedDPS, 4.0);
        e.set(f.state, CombatState::cFleeState, 2);
        e.mem.set_f32(timer(f.state, 0x1dc) + 4, 1.0);
        let log = logged(&mut e, |e| fn_00998a50(e, f.state));
        assert_eq!(calls(&log, 0x0099_f6d0), vec![vec![f.state.addr()]]);
        assert_eq!(e.mem.f32(timer(f.state, 0x1d4) + 4), 6.0);
        assert_eq!(e.get(f.state, CombatState::fDisarmedDPS), 0.0);
        assert_eq!(e.get(f.state, CombatState::cFleeState), 0);
        // No blocked projectile: its timer is cleared.
        assert_eq!(e.mem.f32(timer(f.state, 0x1dc) + 4), 0.0);
    }

    #[test]
    fn negative_timer_delay_is_detected() {
        let mut e = engine();
        let timer: Ptr<CombatTimer> = e.new_object();
        e.set(timer, CombatTimer::fDelay, -1.0);
        assert!(fn_009990c0(&mut e, timer));
        e.set(timer, CombatTimer::fDelay, f32::NAN);
        assert!(!fn_009990c0(&mut e, timer));
    }

    #[test]
    fn small_helpers_pass_their_arguments() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_00999100(e, state));
        assert_eq!(calls(&log, 0x0062_96b0), vec![vec![state.addr(), 3]]);
        stub_st0(&mut e, 0x006b_9130, 0.0);
        assert!(fn_00999120(&mut e, 0x100));
        stub_st0(&mut e, 0x006b_9130, f64::NAN);
        assert!(!fn_00999120(&mut e, 0x100));
    }

    #[test]
    fn reduced_update_detects_the_target_and_runs_the_threat_timer() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        put_f32(&mut e, 0x011c_f804 + 4, 7.0);
        e.mem.set_u32(timer(f.state, 0x20c), EXPIRED);
        let log = logged(&mut e, |e| fn_00999160(e, f.state));
        let group = e.mem.u32(f.controller + 0x80);
        let s = f.state.addr();
        assert_eq!(
            calls(&log, 0x0098_87b0),
            vec![vec![
                group,
                f.attacker,
                f.target,
                s + 0x6c,
                s + 0x94,
                s + 0x70,
                s + 0x71
            ]]
        );
        assert_eq!(calls(&log, 0x009a_1060), vec![vec![s]]);
        assert_eq!(e.mem.f32(timer(f.state, 0x20c) + 4), 7.0);
    }

    #[test]
    fn permissions_follow_their_timers() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.set(f.state, CombatState::iCombatPermissions, 0xffff);
        e.mem.set_u32(timer(f.state, 0x1e4), EXPIRED);
        e.mem.set_u32(timer(f.state, 0x1ec), EXPIRED);
        stub(&mut e, 0x004d_ff00, 1);
        stub_st0(&mut e, 0x0049_5460, 50.0);
        let log = logged(&mut e, |e| fn_00999260(e, f.state));
        assert_eq!(e.get(f.state, CombatState::iCombatPermissions), 0x103);
        assert_eq!(calls(&log, 0x004d_ff00)[1], vec![0.5f32.to_bits()]);
        assert_eq!(e.mem.f32(timer(f.state, 0x1e4) + 4), 5.0);
        assert_eq!(e.mem.f32(timer(f.state, 0x1ec) + 4), 10.0);
    }

    #[test]
    fn inventory_update_without_items_keeps_the_unarmed_dps() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub_st0(&mut e, 0x0064_6060, 3.0);
        e.register(0x0040_4010, |_, a| {
            st0(f32::from_bits(a[0]).max(f32::from_bits(a[1])) as f64)
        });
        put_f32(&mut e, 0x011c_f660 + 4, 4.0);
        e.set(f.state, CombatState::bInventoryDirty, 1);
        let log = logged(&mut e, |e| fn_009993c0(e, f.state));
        assert_eq!(e.get(f.state, CombatState::fUnarmedDPS), 3.0);
        assert_eq!(e.get(f.state, CombatState::fMaxCombatWeaponDPS), 3.0);
        assert_eq!(
            e.get(f.state, CombatState::fMaxModifiedCombatWeaponDPS),
            3.0
        );
        assert_eq!(e.get(f.state, CombatState::iHasCombatWeapon), 0x40);
        assert_eq!(e.get(f.state, CombatState::bInventoryDirty), 0);
        assert_eq!(e.mem.f32(timer(f.state, 0x21c) + 4), 4.0);
        assert_eq!(
            calls(&log, 0x0040_4eb0)[0][1..],
            [0x2b, 1, SOURCE_FILE_NAME, 0x346]
        );
        assert_eq!(calls(&log, 0x0040_4ee0).len(), 1);
    }

    #[test]
    fn inventory_update_with_a_forced_weapon_uses_it_alone() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let acquired = object(&mut e, 0x40);
        stub(&mut e, SLOTS + 0x148, acquired);
        let extra = e.mem.alloc(4);
        e.mem.set_u32(extra, 0x77);
        stub(&mut e, 0x0055_9450, extra);
        stub(&mut e, 0x0041_8b10, 1);
        stub(&mut e, ARRAY_SIZE, 0x900);
        stub(&mut e, 0x0052_2c80, 1);
        fn_009993c0(&mut e, f.state);
        assert_eq!(e.mem.u32(f.state.addr() + 0x10), 0x900);
        assert_eq!(e.mem.f32(f.state.addr() + 0x40), 9_999_999.0);
        assert_eq!(
            e.get(f.state, CombatState::fMaxRangedWeaponDPS),
            9_999_999.0
        );
        assert_eq!(e.get(f.state, CombatState::iHasCombatWeapon), 2);
    }

    #[test]
    fn clearing_weapons_fills_and_empties() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_0099a2a0(e, state));
        assert_eq!(
            calls(&log, FILL_BYTES),
            vec![vec![state.addr() + 0xc, 0, 0x18]]
        );
        assert_eq!(calls(&log, 0x0084_54f0), vec![vec![state.addr() + 0x24, 0]]);
    }

    #[test]
    fn best_rated_entry_wins() {
        let mut e = engine();
        e.register(0x0097_ac90, |_, a| {
            st0([1.0, 4.0, 2.0, 4.0, 0.0, 0.0, 9.0][a[1] as usize])
        });
        e.register(0x0097_c150, |_, a| ret(0x100 + a[1]));
        assert_eq!(fn_0099a2d0(&mut e, 0x10), 0);
        e.register(0x0097_ac90, |_, a| {
            st0([1.0, 4.0, 2.0, 4.0, 0.0, 0.0, 0.0][a[1] as usize])
        });
        assert_eq!(fn_0099a2d0(&mut e, 0x10), 0x101);
    }

    #[test]
    fn reachability_depends_on_the_world_space() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        assert!(fn_0099a360(&mut e, f.state, 0, 5));
        e.set(f.state, CombatState::iTargetDetectionLevel, 1);
        let space = object(&mut e, 0x20);
        e.mem.set_u32(space + 0x10, 2);
        stub(&mut e, 0x004f_d3e0, space);
        stub(&mut e, SLOTS + 0x34, 7);
        assert!(!fn_0099a360(&mut e, f.state, 0, 0));
        e.mem.set_u32(space + 0x10, 3);
        assert!(fn_0099a360(&mut e, f.state, 0, 0));
        assert!(!fn_0099a3e0(&mut e, space));
    }

    #[test]
    fn forgetting_a_reference_releases_what_holds_it() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x55);
        e.register_double(0x0097_cfc0, move |_, a| {
            ret(if a[1] == 1 { item } else { 0 })
        });
        let target_location = e.mem.alloc(0x20);
        e.mem.set_u32(target_location + 4, 0x55);
        e.set(state, CombatState::pLocationTarget, target_location);
        e.set(state, CombatState::pLastTarget, 0x55);
        let log = logged(&mut e, |e| fn_0099a420(e, state, 0x55, 1));
        assert_eq!(calls(&log, 0x0098_1340), vec![vec![state.addr(), 1]]);
        assert_eq!(calls(&log, 0x009a_0a40).len(), 1);
        assert_eq!(e.get(state, CombatState::pLastTarget), 0);
        // Without `keep_pair`, items 3 and 4 changed (they were 7 and 0).
        e.mem.set_u32(state.addr() + 0x19c, 7);
        let log = logged(&mut e, |e| fn_0099a420(e, state, 0x66, 0));
        assert_eq!(
            calls(&log, 0x0098_1340),
            vec![vec![state.addr(), 3], vec![state.addr(), 4]]
        );
    }

    #[test]
    fn invalid_combat_items_are_released() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x55);
        e.register_double(0x0097_cfc0, move |_, a| {
            ret(if a[1] == 2 { item } else { 0 })
        });
        stub(&mut e, 0x0043_7b90, 1);
        let log = logged(&mut e, |e| fn_0099a530(e, state));
        assert_eq!(calls(&log, 0x0098_1340), vec![vec![state.addr(), 2]]);
        assert_eq!(calls(&log, 0x0043_7b90), vec![vec![0x55]]);
    }

    #[test]
    fn attack_stamps_set_the_firing_start_once() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        stub_st0(&mut e, 0x006a_7f50, -(f32::MAX as f64));
        fn_0099a5e0(&mut e, state, 0);
        assert_eq!(
            e.get(state, CombatState::fLastAttackTimeStamp),
            5.0f32.to_bits()
        );
        assert_eq!(
            e.get(state, CombatState::fFiringStartedTimeStamp),
            5.0f32.to_bits()
        );
        e.set(state, CombatState::fFiringStartedTimeStamp, 0);
        stub_st0(&mut e, 0x006a_7f50, 1.0);
        fn_0099a5e0(&mut e, state, 0);
        assert_eq!(e.get(state, CombatState::fFiringStartedTimeStamp), 0);
        e.set(state, CombatState::fLastAttackTimeStamp, 0);
        fn_0099a680(&mut e, state, 0);
        assert_eq!(
            e.get(state, CombatState::fLastAttackTimeStamp),
            5.0f32.to_bits()
        );
    }

    #[test]
    fn embedded_weapon_timer_poke_unless_skipped() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        let log = logged(&mut e, |e| fn_0099a650(e, state, 0, 1));
        assert!(calls(&log, 0x0097_f220).is_empty());
        let log = logged(&mut e, |e| fn_0099a650(e, state, 0, 0));
        assert_eq!(calls(&log, 0x0097_f220), vec![vec![state.addr() + 0x214]]);
    }

    #[test]
    fn damage_after_a_second_lowers_confidence_and_boosts_when_fleeing() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        e.mem.set_f32(state.addr() + 0xa0, 2.0);
        e.set(state, CombatState::cFleeState, 2);
        let log = logged(&mut e, |e| fn_0099a6b0(e, state, 0, 0));
        assert_eq!(calls(&log, 0x009a_0c50), vec![vec![state.addr(), 1]]);
        assert_eq!(calls(&log, 0x009a_1280), vec![vec![state.addr(), 1]]);
        assert_eq!(e.mem.f32(state.addr() + 0xa0), 5.0);
        let log = logged(&mut e, |e| fn_0099a6b0(e, state, 0, 0));
        assert!(calls(&log, 0x009a_0c50).is_empty());
    }

    #[test]
    fn disarm_remembers_the_best_dps() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        put_f32(&mut e, 0x011c_e058 + 4, 3.0);
        e.set(f.state, CombatState::fMaxCombatWeaponDPS, 8.0);
        fn_0099a720(&mut e, f.state);
        // The inventory update then clears the maximum again.
        assert_eq!(e.get(f.state, CombatState::fDisarmedDPS), 8.0);
        assert_eq!(e.mem.f32(timer(f.state, 0x1d4) + 4), 3.0);
    }

    #[test]
    fn pre_activation_lets_item_two_go() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x55);
        e.register_double(0x0097_cfc0, move |_, a| {
            ret(if a[1] == 2 { item } else { 0 })
        });
        let log = logged(&mut e, |e| {
            combat_state_pre_activate_reference(e, f.state, 0x55)
        });
        assert_eq!(calls(&log, 0x0098_1380), vec![vec![f.controller]]);
        assert_eq!(calls(&log, 0x0098_1340), vec![vec![f.state.addr(), 2]]);
    }

    #[test]
    fn flee_door_is_remembered_while_fleeing() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub(&mut e, 0x0098_1990, 1);
        stub(&mut e, 0x0040_1170, 0x1c);
        stub(&mut e, 0x0084_e3a0, 0x99);
        fn_0099a7d0(&mut e, f.state, 0x10, 0, 0, 0);
        assert_eq!(e.get(f.state, CombatState::iLastFleeDoor), 0x99);
        assert_eq!(
            e.get(f.state, CombatState::fLastFleeDoorTimeStamp),
            5.0f32.to_bits()
        );
        stub(&mut e, 0x0040_1170, 0x1d);
        e.set(f.state, CombatState::iLastFleeDoor, 0);
        fn_0099a7d0(&mut e, f.state, 0x10, 0, 0, 0);
        assert_eq!(e.get(f.state, CombatState::iLastFleeDoor), 0);
    }

    #[test]
    fn usable_item_timer_takes_the_second_setting() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        put_f32(&mut e, 0x011c_fdd8 + 4, 1.0);
        put_f32(&mut e, 0x011c_f480 + 4, 2.0);
        e.mem.set_u32(state.addr() + 0x1a4, 0x30);
        let log = logged(&mut e, |e| fn_0099a840(e, state, 0x30));
        assert_eq!(
            calls(&log, TIMER_START),
            vec![vec![state.addr() + 0x1ac, 2.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, SETTING_VALUE).len(), 2);
    }

    #[test]
    fn unsafe_cover_is_marked() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let cover = e.mem.alloc(0x78);
        e.set(f.state, CombatState::pCoverLocation, cover);
        let location = e.mem.alloc(0x20);
        e.mem.set_u32(location + 0x14, 1);
        e.set(f.state, CombatState::pCombatLocation, location);
        stub(&mut e, 0x005c_3420, 0x600);
        let log = logged(&mut e, |e| fn_0099a8d0(e, f.state));
        assert_eq!(
            calls(&log, 0x0099_ccd0),
            vec![vec![f.state.addr(), cover + 0x14, 0x600, f.target, 0]]
        );
        assert_eq!(calls(&log, 0x0054_07b0), vec![vec![cover, 1]]);
        assert_eq!(calls(&log, 0x009a_0680), vec![vec![f.state.addr()]]);
        assert_eq!(calls(&log, 0x0062_96b0), vec![vec![f.state.addr(), 3]]);
    }

    #[test]
    fn failing_cover_locations_are_cleared() {
        let mut e = engine();
        let state: Ptr<CombatState> = e.new_object();
        e.set(state, CombatState::pCoverLocation, 0x10);
        e.set(state, CombatState::pSecondaryCoverLocation, 0x20);
        sequence(&mut e, 0x005d_c960, vec![1, 0]);
        let log = logged(&mut e, |e| fn_0099a9a0(e, state));
        assert!(calls(&log, CLEAR_COVER_LOCATION).is_empty());
        assert_eq!(
            calls(&log, CLEAR_SECONDARY_COVER_LOCATION),
            vec![vec![state.addr(), 0]]
        );
    }

    #[test]
    fn ended_procedure_clears_its_location() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.set(f.state, CombatState::pCombatLocation, 0x40);
        stub(&mut e, 0x0082_5c00, 0xf);
        let log = logged(&mut e, |e| fn_0099aa10(e, f.state, 0x77, 0xf));
        let group = e.mem.u32(f.controller + 0x80);
        assert_eq!(calls(&log, 0x0098_bd70), vec![vec![group, 0x77]]);
        assert_eq!(calls(&log, 0x009a_0680), vec![vec![f.state.addr()]]);
    }

    #[test]
    fn pathing_cover_location_copy_sets_its_vtable() {
        let mut e = engine();
        let (to, from) = (Ptr::new(e.mem.alloc(0x54)), Ptr::new(e.mem.alloc(0x54)));
        e.mem.set_u32(from.addr() + 0x44, 0xabc);
        e.mem.set_u8(from.addr() + 0x52, 4);
        let log = logged(&mut e, |e| {
            assert_eq!(
                pathing_cover_location_pathing_cover_location(e, to, from),
                to
            );
        });
        assert_eq!(e.mem.u32(to.addr()), 0x0106_cbc0);
        assert_eq!(e.mem.u32(to.addr() + 0x44), 0xabc);
        assert_eq!(e.mem.u8(to.addr() + 0x52), 4);
        assert_eq!(calls(&log, 0x006d_cc40), vec![vec![to.addr(), from.addr()]]);
    }

    #[test]
    fn last_seen_location_is_valid_for_two_minutes() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let out: Ptr<NiPoint3> = e.new_object();
        e.mem.set_f32(f.state.addr() + 0x8c, 7.0);
        e.mem.set_f32(f.state.addr() + 0x98, 0.0);
        assert!(fn_0099ab50(&mut e, f.state, out));
        assert_eq!(e.get(out, NiPoint3::z), 7.0);
        stub_st0(&mut e, 0x009a_8460, 1.5);
        assert!(combat_state_get_last_seen_target_firing_location(
            &mut e, f.state, out
        ));
        assert_eq!(e.get(out, NiPoint3::z), 8.5);
        e.mem.set_f32(f.state.addr() + 0x98, -200.0);
        assert!(!fn_0099ab50(&mut e, f.state, out));
    }

    #[test]
    fn find_combat_location_reuses_a_planned_one_or_sets_flags() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub(&mut e, 0x005e_3fc0, 0x700);
        stub(&mut e, 0x0098_0700, 1);
        let log = logged(&mut e, |e| {
            assert!(combat_state_find_combat_location(e, f.state, 3))
        });
        assert_eq!(calls(&log, 0x0097_9f70), vec![vec![0x700, 3]]);

        stub(&mut e, 0x0098_0700, 0);
        e.register(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x006d_3150, |_, a| ret(a[0]));
        stub(&mut e, 0x0043_90c0, 1);
        let log = logged(&mut e, |e| {
            assert!(combat_state_find_combat_location(e, f.state, 0xe))
        });
        let planned = calls(&log, 0x0099_5980);
        assert_eq!(planned.len(), 1);
        let location = planned[0][2];
        assert_eq!(planned[0][1], 0xe);
        assert_eq!(e.mem.u32(location + 0x1c), 1);
        assert_eq!(calls(&log, 0x0041_fd00), vec![vec![location, 0]]);
        assert_eq!(calls(&log, 0x006d_3370).len(), 1);

        // `0097bd00` gives `FLT_MAX`: no search.
        stub_st0(&mut e, 0x0097_bd00, f32::MAX as f64);
        let log = logged(&mut e, |e| {
            assert!(!combat_state_find_combat_location(e, f.state, 0xe))
        });
        assert!(calls(&log, 0x0040_1000).is_empty());
    }

    #[test]
    fn location_flags_and_space() {
        let mut e = engine();
        let location: Ptr<CombatLocation> = e.new_object();
        fn_0099b540(&mut e, location, 6, 1);
        fn_0099b540(&mut e, location, 2, 0);
        assert_eq!(e.get(location, CombatLocation::iFlags), 4);
        stub(&mut e, 0x0084_e3a0, 0x33);
        assert_eq!(fn_0099b580(&mut e, location), 0x33);
        let log = logged(&mut e, |e| fn_0099b520(e, location, 0x44));
        assert_eq!(calls(&log, 0x0041_fd00), vec![vec![location.addr(), 0x44]]);
        e.set(location, CombatLocation::pRef, 9);
        let log = logged(&mut e, |e| fn_0099b8b0(e, location, 0x500, 0x44));
        assert_eq!(
            calls(&log, 0x0098_9010),
            vec![vec![location.addr(), 0x500, 0x44]]
        );
        assert_eq!(e.get(location, CombatLocation::pRef), 0);
    }

    #[test]
    fn grenade_spot_from_the_group_location() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let out: Ptr<CombatLocation> = e.new_object();
        assert!(!fn_0099b5a0(&mut e, f.state, out, 0, 0));
        stub(&mut e, 0x0052_5a90, 0x300);
        stub_st0(&mut e, 0x009a_7f00, 1000.0);
        stub(&mut e, 0x0098_90e0, 1);
        stub_st0(&mut e, 0x0059_5c80, 100_000.0);
        stub(&mut e, 0x009a_7670, 1);
        let arc = e.mem.alloc(4);
        let log = logged(&mut e, |e| {
            assert!(fn_0099b5a0(e, f.state, out, arc, 0x200))
        });
        let set = calls(&log, 0x0098_9010);
        assert_eq!(set.len(), 1);
        assert_eq!((set[0][0], set[0][2]), (out.addr(), 0));
        let arc_check = calls(&log, 0x009a_7670);
        assert_eq!(arc_check[0][..3], [f.attacker, f.target, 0x300]);
        assert_eq!(arc_check[0][12], arc);
        // Too close: no spot.
        stub_st0(&mut e, 0x0059_5c80, 100.0);
        assert!(!fn_0099b5a0(&mut e, f.state, out, arc, 0x200));
    }

    #[test]
    fn cover_search_builds_the_request_when_no_cover_is_kept() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let info: Ptr<CombatCoverSearchInfo> = e.new_object();
        let cover = e.mem.alloc(0x78);
        let point = e.mem.alloc(12);
        stub(&mut e, SLOTS + 0x1f4, point);
        stub_st0(&mut e, 0x0040_ebd0, 1500.0);
        stub_st0(&mut e, 0x004a_7bd0, 300.0);
        let log = logged(&mut e, |e| {
            assert!(!fn_0099b8e0(e, f.state, 0x400, info, cover, 0x900, 2))
        });
        assert_eq!(e.mem.u32(cover + 4), 2);
        assert_eq!(e.get(info, CombatCoverSearchInfo::fRadius), 300.0);
        assert_eq!(
            e.get(info, CombatCoverSearchInfo::iPreviousCoverLocationKey),
            0
        );
        assert_eq!(
            e.get(info, CombatCoverSearchInfo::fTimeStamp),
            5.0f32.to_bits()
        );
        let request = calls(&log, 0x006e_4080)[0][0];
        assert_eq!(
            calls(&log, 0x006e_4320),
            vec![vec![request, 300.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, 0x006e_4340),
            vec![vec![request, 512.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, 0x0040_30d0), vec![vec![request, 200]]);
        assert_eq!(calls(&log, 0x0048_3710).len(), 2);
        assert_eq!(calls(&log, 0x006d_ad70), vec![vec![request]]);
        // No target: nothing at all.
        e.mem.set_u32(f.controller + 0xc0, 0);
        assert!(!fn_0099b8e0(&mut e, f.state, 0x400, info, cover, 0x900, 2));
    }

    #[test]
    fn cover_height_and_lock() {
        let mut e = engine();
        let cover = e.mem.alloc(0x54);
        e.mem.set_u8(cover + 0x4c, 2);
        assert_eq!(fn_0099c1f0(&mut e, cover), 2);
        let log = logged(&mut e, |e| fn_0099c210(e, 0x100));
        assert_eq!(calls(&log, 0x00aa_6060), vec![vec![0x114]]);
    }

    #[test]
    fn cover_check_reports_its_reasons() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let state = e.mem.alloc(4);
        let distance = e.mem.alloc(4);
        let cover_location = e.mem.alloc(0x54);
        let reservation = e.mem.alloc(8);
        let check = |key| CoverCheck {
            cover_location,
            out: 0x900,
            reservation,
            search_point: 0x800,
            key,
            current_key: 0,
            kind: 2,
            attacker: f.attacker,
            target: f.target,
            minimum_distance: 0.0,
            maximum_distance: 100.0,
            state,
            distance,
            flag_006e43e0: 0,
        };
        stub(&mut e, 0x0098_2be0, 1);
        assert!(!fn_0099c230(&mut e, f.state, check(5)));
        assert_eq!(e.mem.u32(state), 2);
        stub(&mut e, 0x0098_2be0, 0);
        stub(&mut e, 0x009a_0d70, 1);
        assert!(!fn_0099c230(&mut e, f.state, check(5)));
        assert_eq!(e.mem.u32(state), 3);
        stub(&mut e, 0x009a_0d70, 0);
        let log = logged(&mut e, |e| assert!(!fn_0099c230(e, f.state, check(5))));
        assert_eq!(e.mem.u32(state), 9);
        let reserve = calls(&log, 0x0098_35d0);
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0][1..4], [5, cover_location, f.controller]);
    }

    #[test]
    fn group_member_blocks_the_location() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub(&mut e, 0x005a_4320, 2);
        let member = object(&mut e, 0x40);
        e.register_double(0x0096_8630, move |_, a| {
            ret(if a[1] == 0 { 0x66 } else { member })
        });
        let point = e.mem.alloc(12);
        stub(&mut e, SLOTS + 0x1f4, point);
        stub(&mut e, 0x0099_cc60, 1);
        let log = logged(&mut e, |e| assert!(fn_0099ca00(e, f.state, 0x700, 0x66)));
        let tests = calls(&log, 0x0099_cc60);
        assert_eq!(tests.len(), 1);
        assert_eq!((tests[0][0], tests[0][2]), (0x700, 0));
        stub(&mut e, 0x0099_cc60, 0);
        assert!(!fn_0099ca00(&mut e, f.state, 0x700, 0x66));
    }

    #[test]
    fn any_blocking_point_counts() {
        let mut e = engine();
        stub(&mut e, ARRAY_SIZE, 2);
        let point = e.mem.alloc(12);
        e.mem.set_u32(point, 0x42);
        stub(&mut e, 0x006a_7af0, point);
        sequence(&mut e, 0x0099_cc60, vec![0, 1]);
        let log = logged(&mut e, |e| assert!(fn_0099cb00(e, 0x700, 0x800)));
        assert_eq!(calls(&log, 0x0099_cc60).len(), 2);
        stub(&mut e, 0x0099_cc60, 0);
        assert!(!fn_0099cb00(&mut e, 0x700, 0x800));
    }

    #[test]
    fn forwarder_fn_009a3280_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(ELEMENT_TARGET_009052C0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x009a_3280, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == ELEMENT_TARGET_009052C0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_009a3410_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009a_3410, ARRAY_VTABLE_0108E068, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_009a3460_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009a_3460, ARRAY_VTABLE_0108E07C, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_009a35f0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009a_35f0, ARRAY_VTABLE_0108E090, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_009a3810_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009a_3810, ARRAY_VTABLE_0108E0A4, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_009a3830_compares_the_words_the_pointers_point_to() {
        let mut e = Engine::new();
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        e.mem.set_u32(a, 0x1234_5678);
        e.mem.set_u32(b, 0x1234_5678);
        assert!(e.call(0x009a_3830, &args![a, b]).bool());
        e.mem.set_u32(b, 0x1234_5679);
        assert!(!e.call(0x009a_3830, &args![a, b]).bool());
        // The same pointer twice is trivially equal.
        assert!(e.call(0x009a_3830, &args![a, a]).bool());
    }
}

#[cfg(test)]
mod float_reader_b039_tests {
    use super::*;

    #[test]
    fn fn_0099e040_reads_the_float_at_0xbc() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.mem.set_f32(this + 0xbc, 2.5);
        assert_eq!(e.call(0x0099_e040, &args![this]).f64(), 2.5);
        // A signalling NaN comes back quiet.
        e.mem.set_u32(this + 0xbc, 0x7f80_0001);
        let r = e.call(0x0099_e040, &args![this]).f64();
        assert_eq!((r as f32).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn fn_009a1260_reads_the_float_at_0xc8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.mem.set_f32(this + 0xc8, -7.25);
        assert_eq!(e.call(0x009a_1260, &args![this]).f64(), -7.25);
    }
}

#[cfg(test)]
mod n025_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Every callee outside this file that the batch's translations call.
    const EXTERNAL: &[u32] = &[
        0x0040_1000,
        0x0040_1030,
        0x0040_1050,
        0x0040_1170,
        0x0040_30b0,
        0x0040_30d0,
        0x0040_3e20,
        0x0040_4010,
        0x0040_8840,
        0x0040_8d60,
        0x0040_ebd0,
        0x0041_81e0,
        0x0041_86f0,
        0x0041_8b10,
        0x0042_eb40,
        0x0043_5dd0,
        0x0043_5de0,
        0x0043_5e00,
        0x0043_7b90,
        0x0043_9e90,
        0x0043_9ef0,
        0x0043_a2b0,
        0x0043_a320,
        0x0043_a390,
        0x0044_1110,
        0x0044_ddc0,
        0x0045_36e0,
        0x0045_3700,
        0x0045_7910,
        0x0045_7990,
        0x0045_79e0,
        0x0045_bb20,
        0x0046_1110,
        0x0046_1580,
        0x0047_7ba0,
        0x0047_d0b0,
        0x0047_eeb0,
        0x0048_3710,
        0x0048_39c0,
        0x0048_73d0,
        0x0048_d150,
        0x0048_ee50,
        0x0049_5460,
        0x0049_f210,
        0x004a_0bd0,
        0x004a_0c10,
        0x004a_7290,
        0x004b_13c0,
        0x004b_15e0,
        0x004b_3800,
        0x004b_9ba0,
        0x004b_cdb0,
        0x004c_0c30,
        0x004d_ff00,
        0x004f_8960,
        0x004f_9bf0,
        0x004f_b070,
        0x004f_f7e0,
        0x0050_21a0,
        0x0050_7610,
        0x0050_76c0,
        0x0050_7b00,
        0x0051_b840,
        0x0052_5980,
        0x0052_71f0,
        0x0052_72d0,
        0x0052_73c0,
        0x0052_7a40,
        0x0052_7b70,
        0x0052_7ba0,
        0x0054_07b0,
        0x0054_6a90,
        0x0055_9450,
        0x0055_e200,
        0x0056_4db0,
        0x0056_7790,
        0x0056_7d20,
        0x0056_8e50,
        0x0056_9160,
        0x0057_5610,
        0x0057_5650,
        0x0057_5ca0,
        0x0057_85e0,
        0x0058_6150,
        0x0058_9850,
        0x005a_00a0,
        0x005a_4320,
        0x005c_3420,
        0x005d_43c0,
        0x005d_c960,
        0x005e_04c0,
        0x005e_04f0,
        0x005f_9b50,
        0x0063_3c90,
        0x0063_77e0,
        0x0064_50c0,
        0x0064_6060,
        0x0067_fdb0,
        0x0068_15c0,
        0x0068_efb0,
        0x0069_50d0,
        0x0069_e390,
        0x0069_ef80,
        0x006a_2770,
        0x006a_6eb0,
        0x006a_7ad0,
        0x006a_7f50,
        0x006a_9540,
        0x006a_bbb0,
        0x006a_bc50,
        0x006b_3160,
        0x006b_3eb0,
        0x006b_7f20,
        0x006b_a6e0,
        0x006c_ee30,
        0x006d_1bc0,
        0x006d_2c20,
        0x006d_2c40,
        0x006d_30b0,
        0x006d_30e0,
        0x006d_3100,
        0x006d_3120,
        0x006d_3150,
        0x006d_3280,
        0x006d_3370,
        0x006d_3ac0,
        0x006d_5490,
        0x006d_61e0,
        0x006d_62e0,
        0x006d_6d90,
        0x006d_6f80,
        0x006d_7490,
        0x006d_7a40,
        0x006d_9350,
        0x006d_ad40,
        0x006d_ad70,
        0x006d_ad90,
        0x006d_bde0,
        0x006d_be40,
        0x006d_bee0,
        0x006d_bfc0,
        0x006d_cbb0,
        0x006d_cd70,
        0x006d_ce10,
        0x006d_d0d0,
        0x006d_d4f0,
        0x006d_ed70,
        0x006e_2420,
        0x006e_4080,
        0x006e_4320,
        0x006e_4340,
        0x006e_4360,
        0x006e_43e0,
        0x006e_4430,
        0x006e_4570,
        0x006e_4740,
        0x006e_4820,
        0x006e_4840,
        0x006e_4a50,
        0x006e_4bf0,
        0x006e_4ef0,
        0x006e_4f10,
        0x006e_4f90,
        0x006e_4fb0,
        0x006e_5ee0,
        0x006e_b9d0,
        0x006e_bf30,
        0x0070_31c0,
        0x0070_58c0,
        0x0070_5b10,
        0x0071_7e50,
        0x0071_9b20,
        0x0072_ba80,
        0x0078_81d0,
        0x007a_f430,
        0x007c_b2e0,
        0x0081_9a50,
        0x0082_5c00,
        0x0084_4160,
        0x0084_5460,
        0x0084_54f0,
        0x0084_b5c0,
        0x0084_e3a0,
        0x0086_48a0,
        0x0086_48e0,
        0x0086_4980,
        0x0086_4a60,
        0x0086_5db0,
        0x0086_5df0,
        0x0086_5e50,
        0x0086_5f60,
        0x0088_c600,
        0x008a_0b10,
        0x008a_1710,
        0x008a_6650,
        0x008a_cbe0,
        0x008b_06d0,
        0x008b_3880,
        0x008b_38d0,
        0x008b_c9d0,
        0x008b_e1b0,
        0x008b_e600,
        0x008d_8520,
        0x008d_8660,
        0x008d_f900,
        0x008e_eaa0,
        0x0090_3600,
        0x0090_52c0,
        0x0090_57d0,
        0x0092_3000,
        0x0093_e510,
        0x0096_11e0,
        0x0096_8630,
        0x0096_a2d0,
        0x0097_ae90,
        0x0097_ede0,
        0x0097_ee60,
        0x0097_ee90,
        0x0097_f200,
        0x0097_fd40,
        0x0098_0070,
        0x0098_1340,
        0x0098_1420,
        0x0098_2c20,
        0x0098_2ca0,
        0x0098_35d0,
        0x0098_3720,
        0x0098_6a80,
        0x0098_9140,
        0x0098_9700,
        0x0098_cb30,
        0x0098_cc20,
        0x0098_f1f0,
        0x0099_0890,
        0x0099_2530,
        0x0099_28c0,
        0x0099_5890,
        0x009a_0240,
        0x009a_0260,
        0x009a_0510,
        0x009a_05b0,
        0x009a_3950,
        0x009a_3a80,
        0x009a_3c50,
        0x009a_3d00,
        0x009a_5670,
        0x009a_5eb0,
        0x009a_5f20,
        0x009a_5f90,
        0x009a_5ff0,
        0x009a_68b0,
        0x009a_69c0,
        0x009a_7940,
        0x009a_7fa0,
        0x009a_93e0,
        0x009a_9630,
        0x00aa_13e0,
        0x00ec_43fb,
        // Callees of this file's older functions that the batch calls directly.
        0x00aa_6060,
        0x0098_9010,
        0x006a_7af0,
        0x0042_fc00,
        0x0072_6bf0,
    ];

    /// A shared vtable whose slot `s` calls `SLOTS + s` (all doubled).
    const VTABLE: u32 = 0x0200_0000;
    const SLOTS: u32 = 0x0300_0000;
    /// The start-time word that makes the timer double report "expired".
    const EXPIRED: u32 = 1;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| ret(eax));
    }

    fn stub_st0(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| st0(value));
    }

    /// Returns `results` in turn (the last one again afterwards).
    fn sequence(e: &mut Engine, addr: u32, results: Vec<u32>) {
        let mut next = 0;
        e.register_double(addr, move |_, _| {
            let value = results[next.min(results.len() - 1)];
            next += 1;
            ret(value)
        });
    }

    /// Returns `results` in turn in `ST0` (the last one again afterwards).
    fn sequence_st0(e: &mut Engine, addr: u32, results: Vec<f64>) {
        let mut next = 0;
        e.register_double(addr, move |_, _| {
            let value = results[next.min(results.len() - 1)];
            next += 1;
            st0(value)
        });
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

    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size.max(0x10));
        e.mem.set_u32(object, VTABLE);
        object
    }

    fn put_point(e: &mut Engine, addr: u32, x: f32, y: f32, z: f32) {
        e.mem.set_f32(addr, x);
        e.mem.set_f32(addr + 4, y);
        e.mem.set_f32(addr + 8, z);
    }

    /// The setting at `setting` holds `value` (its value word is at +4).
    fn put_setting(e: &mut Engine, setting: u32, value: f32) {
        e.mem.set_f32(setting + 4, value);
    }

    /// Every outside callee doubled (returning 0), the exe's constants, and working
    /// doubles for the small getters, time stamps and timers (game time 5).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in EXTERNAL {
            stub(&mut e, addr, 0);
        }
        let mut slots = vec![];
        for slot in (0..0x800).step_by(4) {
            slots.push(SLOTS + slot);
            stub(&mut e, SLOTS + slot, 0);
        }
        e.put_vtable(VTABLE, &slots);
        e.map(0x0101_0000, 0x0008_0000);
        e.map(0x0118_0000, 0x0001_0000);
        e.map(0x011a_0000, 0x0001_0000);
        e.map(0x011c_0000, 0x0002_0000);
        e.map(0x011f_0000, 0x0001_0000);
        for (addr, value) in [
            (MAX_FLOAT, f32::MAX),
            (HUNDRED, 100.0),
            (MINUS_ONE, -1.0),
            (TWO, 2.0),
            (THREE, 3.0),
            (THOUSAND_TWENTY_FOUR, 1024.0),
            (TWO_THOUSAND_FORTY_EIGHT, 2048.0),
            (TWO_HUNDRED_FIFTY_SIX, 256.0),
            (SIXTEEN_THOUSAND_THREE_HUNDRED_EIGHTY_FOUR, 16384.0),
            (LOS_SEARCH_LIMIT, 67_108_864.0),
            (SIXTY_FOUR, 64.0),
        ] {
            e.mem.set_f32(addr, value);
        }
        for (addr, value) in [
            (ZERO_DOUBLE, 0.0),
            (ONE_DOUBLE, 1.0),
            (TWO_DOUBLE, 2.0),
            (THREE_DOUBLE, 3.0),
            (MINUS_ONE_DOUBLE, -1.0),
            (HUNDRED_DOUBLE, 100.0),
            (HUNDRED_TWENTY_DOUBLE, 120.0),
            (TEN_DOUBLE, 10.0),
            (RANGE_SQUARED_DOUBLE, 4_194_304.0),
            (COVER_MIN_DISTANCE_SQUARED_DOUBLE, 16384.0),
            (QUARTER_PI_DOUBLE, f64::from(std::f32::consts::FRAC_PI_4)),
            (ONE_AND_A_HALF_DOUBLE, 1.5),
            (THREE_QUARTERS_DOUBLE, 0.75),
            (NINE_TENTHS_DOUBLE, f64::from(0.9f32)),
            (FOUR_THOUSAND_NINETY_SIX_DOUBLE, 4096.0),
            (EIGHT_DOUBLE, 8.0),
            (PICKUP_TARGET_NEAR_DOUBLE, 589_824.0),
            (OUT_OF_RANGE_DIVISOR_DOUBLE, 10000.0),
        ] {
            e.mem.set_f64(addr, value);
        }
        put_point(&mut e, UP_VECTOR, 0.0, 0.0, 1.0);
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        e.register(CONTROLLER_ATTACKER, |e, a| ret(e.mem.u32(a[0] + 0xbc)));
        e.register(CONTROLLER_TARGET, |e, a| ret(e.mem.u32(a[0] + 0xc0)));
        e.register(CONTROLLER_GROUP, |e, a| ret(e.mem.u32(a[0] + 0x80)));
        e.register(GAME_TIME, |_, _| st0(5.0));
        e.register(TIME_STAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(TIME_STAMP_ELAPSED, |e, a| {
            st0((5.0 - e.mem.f32(a[0])) as f64)
        });
        e.register(TIMER_START, |e, a| {
            e.mem.set_f32(a[0], 5.0);
            e.mem.set_u32(a[0] + 4, a[1]);
            ret(0)
        });
        e.register(TIMER_EXPIRED, |e, a| {
            ret((e.mem.u32(a[0]) == EXPIRED) as u32)
        });
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        // Getters that fill an out parameter return it.
        for addr in [
            0x005c_3420u32,
            0x0043_9ef0,
            0x0043_9e90,
            0x0045_bb20,
            0x004a_0bd0,
            0x006a_6eb0,
            0x0043_a390,
            GET_WORLD_LOCATION,
        ] {
            e.register(addr, |_, a| ret(a[1]));
        }
        e
    }

    struct Fixture {
        state: Ptr<CombatState>,
        controller: u32,
        attacker: u32,
        target: u32,
        group: u32,
    }

    /// A state with its controller, an attacker (with an object at +0xa4), a group and,
    /// if asked, a target.
    fn fixture(e: &mut Engine, with_target: bool) -> Fixture {
        let state: Ptr<CombatState> = e.new_object();
        let controller = e.mem.alloc(0x188);
        let attacker = object(e, 0x200);
        e.mem.set_u32(attacker + 0xa4, VTABLE);
        let target = if with_target { object(e, 0x200) } else { 0 };
        let group = object(e, 0x100);
        e.mem.set_u32(controller + 0xbc, attacker);
        e.mem.set_u32(controller + 0xc0, target);
        e.mem.set_u32(controller + 0x80, group);
        e.set(state, CombatState::pCombatController, controller);
        Fixture {
            state,
            controller,
            attacker,
            target,
            group,
        }
    }

    /// Slot `slot` of the shared vtable returns `eax`.
    fn slot(e: &mut Engine, slot: u32, eax: u32) {
        stub(e, SLOTS + slot, eax);
    }

    /// Slot `slot` of the shared vtable returns `value` in `ST0`.
    fn slot_st0(e: &mut Engine, slot: u32, value: f64) {
        stub_st0(e, SLOTS + slot, value);
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    type Points = Rc<RefCell<Vec<[f32; 3]>>>;

    /// A double for `addr` that records the point argument `index` points to.
    fn record_point(e: &mut Engine, addr: u32, index: usize, eax: u32) -> Points {
        let seen: Points = Rc::new(RefCell::new(vec![]));
        let keep = seen.clone();
        e.register_double(addr, move |e, a| {
            let p = a[index];
            keep.borrow_mut()
                .push([e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]);
            ret(eax)
        });
        seen
    }

    #[test]
    fn fn_0099cb70_adds_each_member_position() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let points = e.mem.alloc(0x10);
        stub(&mut e, 0x005a_4320, 2);
        let first = object(&mut e, 0x20);
        let second = object(&mut e, 0x20);
        sequence(&mut e, 0x0096_8630, vec![first, second]);
        // The first member has no group location, the second one has.
        e.register_double(0x0098_9140, |e, a| {
            if a[1] == 1 {
                put_point(e, a[2], 7.0, 8.0, 9.0);
                ret(1)
            } else {
                ret(0)
            }
        });
        let current = e.mem.alloc(12);
        put_point(&mut e, current, 1.0, 2.0, 3.0);
        slot(&mut e, 0x1f4, current);
        let added = record_point(&mut e, 0x0049_f210, 1, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0099_cb70, &args![f.state, points]);
        });
        assert_eq!(calls(&log, 0x006c_ee30), vec![vec![points, 2]]);
        assert_eq!(
            calls(&log, 0x0098_9140)
                .iter()
                .map(|w| w[3])
                .collect::<Vec<_>>(),
            vec![3.0f32.to_bits(); 2]
        );
        assert_eq!(*added.borrow(), vec![[1.0, 2.0, 3.0], [7.0, 8.0, 9.0]]);
    }

    #[test]
    fn fn_0099cc60_checks_range_unless_skipped() {
        let mut e = engine();
        stub(&mut e, 0x006d_7490, 1);
        stub_st0(&mut e, 0x004a_7290, 5_000_000.0);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0099_cc60, &args![0x100u32, 0x200u32, 0u32]).bool());
        });
        assert!(calls(&log, 0x006d_7490).is_empty());
        assert_eq!(calls(&log, 0x0043_9ef0)[0][0], 0x200);
        // Skipped: straight to the line of sight.
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0099_cc60, &args![0x100u32, 0x200u32, 1u32]).bool());
        });
        assert_eq!(calls(&log, 0x006d_7490), vec![vec![0x100, 0x200, 0, 0]]);
        // In range (and a NaN distance) go on to the line of sight.
        stub_st0(&mut e, 0x004a_7290, f64::NAN);
        stub(&mut e, 0x006d_7490, 0);
        assert!(!e.call(0x0099_cc60, &args![0x100u32, 0x200u32, 0u32]).bool());
    }

    #[test]
    fn check_cover_safe_branches() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let actor = object(&mut e, 0x100);
        let position = e.mem.alloc(12);
        slot(&mut e, 0x1f4, position);
        let cover = e.mem.alloc(0x54);
        let point = e.mem.alloc(12);
        let run = |e: &mut Engine, heading: u32| {
            e.call(0x0099_ccd0, &args![f.state, cover, point, actor, heading])
                .bool()
        };
        // Too near.
        stub_st0(&mut e, 0x004a_7290, 100.0);
        assert!(!run(&mut e, 1));
        // Far enough and turned away: safe without further checks.
        stub_st0(&mut e, 0x004a_7290, 20000.0);
        stub_st0(&mut e, 0x0040_8840, 1.0);
        let log = logged(&mut e, |e| assert!(run(e, 1)));
        assert_eq!(calls(&log, SLOTS + 0x2bc), vec![vec![actor, 0]]);
        assert!(calls(&log, 0x009a_9630).is_empty());
        // Melee: safe beyond 1.5 times the reach (150 squared is 22500).
        stub(&mut e, 0x009a_9630, 1);
        stub_st0(&mut e, 0x009a_69c0, 100.0);
        assert!(!run(&mut e, 0));
        stub_st0(&mut e, 0x004a_7290, 30000.0);
        assert!(run(&mut e, 0));
        // Ranged: in sight is unsafe; the location is destroyed.
        stub(&mut e, 0x009a_9630, 0);
        stub(&mut e, 0x006d_7490, 1);
        let log = logged(&mut e, |e| assert!(!run(e, 0)));
        assert_eq!(calls(&log, 0x004f_f7e0).len(), 1);
        assert_eq!(calls(&log, 0x006d_ce10)[0][1..], [point, actor]);
        // Out of sight and not a ledge: safe.
        stub(&mut e, 0x006d_7490, 0);
        assert!(run(&mut e, 0));
        // A ledge whose angle check fails (no height over the actor).
        stub(&mut e, 0x006e_4820, 1);
        stub_st0(&mut e, 0x0040_ebd0, -5.0);
        assert!(!run(&mut e, 0));
    }

    #[test]
    fn check_ledge_cover_angle_projects_the_drop() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let cover = e.mem.alloc(0x54);
        let point = e.mem.alloc(12);
        let position = e.mem.alloc(12);
        put_point(&mut e, position, 0.0, 0.0, 0.0);
        let run = |e: &mut Engine| {
            e.call(
                0x0099_cec0,
                &args![f.state, cover, point, position, 0x700u32],
            )
            .bool()
        };
        stub_st0(&mut e, 0x0040_ebd0, 10.0);
        stub_st0(&mut e, 0x009a_7fa0, 2.0);
        // No crossing: false.
        assert!(!run(&mut e));
        e.register(0x006b_a6e0, |e, a| {
            e.mem.set_f32(a[4] + 8, 8.0);
            ret(1)
        });
        sequence_st0(&mut e, 0x0058_9850, vec![3.0, 4.0]);
        // Stand height 5 (`00998a30` clear): 6 * 4 / 3 = 8 reaches it.
        stub_st0(&mut e, 0x0045_36e0, 5.0);
        let log = logged(&mut e, |e| assert!(run(e)));
        assert_eq!(calls(&log, 0x009a_7fa0), vec![vec![0x700, 1]]);
        // Crouching (`00998a30`, controller +0xd4) uses the crouch height.
        e.mem.set_u8(f.controller + 0xd4, 1);
        sequence_st0(&mut e, 0x0058_9850, vec![3.0, 4.0]);
        stub_st0(&mut e, 0x0045_3700, 9.0);
        assert!(!run(&mut e));
        // The raised height above the ledge fails early.
        stub_st0(&mut e, 0x009a_7fa0, 20.0);
        let log = logged(&mut e, |e| assert!(!run(e)));
        assert!(calls(&log, 0x006b_a6e0).is_empty());
    }

    #[test]
    fn fn_0099d030_asks_the_cover_map_then_searches() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let info = e.mem.alloc(0x3c);
        e.mem.set_u32(info + 0x30, 9);
        let cover = e.mem.alloc(0x78);
        let location = e.mem.alloc(0x20);
        let manager = e.mem.alloc(0x40);
        e.mem.set_u32(COMBAT_MANAGER, manager);
        stub(&mut e, 0x0096_11e0, 0x7700);
        e.mem.set_u32(cover + 0x68, 0x55);
        e.mem.set_u32(cover + 4, 3);
        e.mem.set_u8(cover + 0x75, 1);
        stub(&mut e, 0x0098_35d0, 1);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(
                    0x0099_d030,
                    &args![f.state, info, cover, location, 3u32, 0x900u32, 0.0f32, 4u32]
                )
                .bool());
        });
        assert_eq!(e.mem.u32(info + 0x30), 0);
        let asked = calls(&log, 0x0098_35d0);
        assert_eq!(
            asked[0],
            vec![
                0x7700,
                0x55,
                cover + 0x14,
                f.controller,
                0,
                cover + 0x70,
                1,
                1
            ]
        );
        assert!(calls(&log, 0x006e_4a50).is_empty());
        // Refused (and kind 6): the covered-move search runs with the kind 6 radius.
        stub(&mut e, 0x0098_35d0, 0);
        put_setting(&mut e, 0x011c_e618, 100.0);
        put_setting(&mut e, 0x011c_f6e8, 300.0);
        stub_st0(&mut e, 0x0049_5460, 50.0);
        e.mem.set_u32(cover + 4, 6);
        // The search (`0099d480`) stops at once: its start and goal are too near.
        stub_st0(&mut e, 0x004a_7290, 0.0);
        let log = logged(&mut e, |e| {
            assert!(!e
                .call(
                    0x0099_d030,
                    &args![f.state, info, cover, location, 6u32, 0x900u32, 0.0f32, 4u32]
                )
                .bool());
        });
        assert_eq!(calls(&log, 0x006e_4a50).len(), 1);
        assert_eq!(calls(&log, 0x0070_5b10)[0][1], 6);
        assert_eq!(calls(&log, 0x006d_61e0)[0][1], 4);
        assert_eq!(calls(&log, 0x006d_ce10)[0][1..], [0x900, f.attacker]);
        assert_eq!(calls(&log, 0x0048_3710).len(), 2);
        assert_eq!(calls(&log, 0x006e_4bf0).len(), 1);
        assert_eq!(calls(&log, 0x004f_f7e0).len(), 2);
        assert!(calls(&log, 0x0098_9010).is_empty());
    }

    #[test]
    fn build_covered_move_request_fills_the_request() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let request = e.mem.alloc(0x120);
        stub_st0(&mut e, 0x008b_e1b0, 20.0);
        stub_st0(&mut e, 0x008a_0b10, 150.0);
        stub_st0(&mut e, 0x0045_36e0, 90.0);
        slot(&mut e, 0x8, 3);
        stub(&mut e, 0x0097_ee90, 1);
        let position = e.mem.alloc(12);
        slot(&mut e, 0x1f4, position);
        stub(&mut e, 0x006e_4fb0, 0x6600);
        stub(&mut e, 0x006e_4f90, 0x6700);
        let cover = e.mem.alloc(0x78);
        e.mem.set_u8(cover + 1, 1);
        e.set(f.state, CombatState::pCoverLocation, cover);
        // `0099cb70` on the group points: an empty group.
        let log = logged(&mut e, |e| {
            e.call(0x0099_d310, &args![f.state, request]);
        });
        assert_eq!(
            calls(&log, 0x0050_7610),
            vec![vec![request, 20.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, 0x006e_5ee0),
            vec![vec![request, 150.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, 0x006e_4ef0),
            vec![vec![request, 90.0f32.to_bits()]]
        );
        assert!(calls(&log, 0x006e_4320).is_empty());
        assert_eq!(
            calls(&log, 0x0050_76c0),
            vec![vec![request, 3.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, 0x006e_4f10), vec![vec![request, position]]);
        assert_eq!(calls(&log, 0x006c_ee30), vec![vec![0x6600, 0]]);
        assert_eq!(calls(&log, 0x0084_b5c0), vec![vec![0x6700, 0]]);
        assert_eq!(calls(&log, 0x007c_b2e0), vec![vec![0x6700, cover + 0x68]]);
    }

    #[test]
    fn fn_0099e010_copies_the_point_to_0xe0() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let point = e.mem.alloc(12);
        put_point(&mut e, point, 1.0, 2.0, 3.0);
        e.call(0x0099_e010, &args![this, point]);
        assert_eq!(e.mem.f32(this + 0xe8), 3.0);
        assert_eq!(e.mem.f32(this + 0xe0), 1.0);
    }

    #[test]
    fn fn_0099e060_makes_a_secondary_cover_location() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let cover = e.mem.alloc(0x78);
        stub(&mut e, 0x006d_3150, cover);
        let manager = e.mem.alloc(0x40);
        e.mem.set_u32(COMBAT_MANAGER, manager);
        e.mem.set_u32(cover + 0x68, 0x55);
        e.mem.set_u32(cover + 4, 2);
        stub(&mut e, 0x0098_35d0, 1);
        stub(&mut e, 0x006e_4570, 0x66);
        stub(&mut e, 0x006e_4740, 0x77);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(
                    0x0099_e060,
                    &args![f.state, 0x4000u32, 0x900u32, 2u32, 1.0f32, 0u32]
                )
                .bool());
        });
        assert_eq!(e.get(f.state, CombatState::pSecondaryCoverLocation), cover);
        assert_eq!(e.mem.u32(cover + 0x68), 0x66);
        assert_eq!(e.mem.u32(cover + 0x6c), 0x77);
        assert_eq!(calls(&log, 0x006e_4430), vec![vec![0x4000, cover + 0x14]]);
        // The secondary search information's key is cleared by `0099d030`.
        assert_eq!(calls(&log, 0x0040_1000), vec![vec![0x78]]);
        // A failed search clears the secondary cover location again.
        stub(&mut e, 0x0098_35d0, 0);
        stub_st0(&mut e, 0x004a_7290, 0.0);
        let log = logged(&mut e, |e| {
            assert!(!e
                .call(
                    0x0099_e060,
                    &args![f.state, 0x4000u32, 0x900u32, 2u32, 1.0f32, 0u32]
                )
                .bool());
        });
        assert_eq!(calls(&log, 0x006d_3370).len(), 2);
        assert_eq!(e.get(f.state, CombatState::pSecondaryCoverLocation), 0);
    }

    fn new_location_target(e: &mut Engine, state: Ptr<CombatState>, kind: u8) -> u32 {
        let target = e.mem.alloc(0x20);
        e.mem.set_u8(target, kind);
        e.set(state, CombatState::pLocationTarget, target);
        target
    }

    #[test]
    fn check_movement_limits_by_location_target() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let target = new_location_target(&mut e, f.state, 2);
        e.mem.set_f32(target + 0x1c, 10.0);
        let run = |e: &mut Engine, unreachable: u32, skip: u32| {
            e.call(0x009a_02a0, &args![f.state, 0x500u32, unreachable, skip])
                .bool()
        };
        // Type 2 farther than the maximum (no group: the maximum itself).
        stub_st0(&mut e, 0x0052_7a40, 101.0);
        assert!(!run(&mut e, 0, 0));
        stub_st0(&mut e, 0x0052_7a40, 100.0);
        assert!(run(&mut e, 0, 0));
        // Type 3 skipped; type 1 skipped while `00461580(0x100)` holds.
        stub_st0(&mut e, 0x0052_7a40, 1000.0);
        e.mem.set_u8(target, 3);
        assert!(run(&mut e, 0, 1));
        assert!(!run(&mut e, 0, 0));
        e.mem.set_u8(target, 1);
        stub(&mut e, 0x0046_1580, 1);
        let log = logged(&mut e, |e| assert!(run(e, 0, 0)));
        assert_eq!(calls(&log, 0x0046_1580), vec![vec![f.state.addr(), 0x100]]);
        // An unreachable location (`009a0f40`: an entry within 128 of it).
        let entry = e.mem.alloc(0x14);
        stub(&mut e, ARRAY_SIZE, 1);
        stub(&mut e, 0x006d_ad90, entry);
        e.mem.set_f32(entry + 0x10, 5.0);
        stub_st0(&mut e, 0x0052_7a40, 1.0);
        assert!(!run(&mut e, 1, 0));
    }

    #[test]
    fn set_cover_locations_clear_the_old_ones() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let old = e.mem.alloc(0x78);
        e.set(f.state, CombatState::pCoverLocation, old);
        let log = logged(&mut e, |e| {
            e.call(0x009a_0370, &args![f.state, 0x1234u32]);
        });
        assert_eq!(calls(&log, 0x006d_3370), vec![vec![old, 1]]);
        assert_eq!(e.get(f.state, CombatState::pCoverLocation), 0x1234);
        e.set(f.state, CombatState::pSecondaryCoverLocation, old);
        let log = logged(&mut e, |e| {
            e.call(0x009a_03a0, &args![f.state, 0x5678u32]);
        });
        assert_eq!(calls(&log, 0x006d_3370), vec![vec![old, 1]]);
        assert_eq!(e.get(f.state, CombatState::pSecondaryCoverLocation), 0x5678);
    }

    #[test]
    fn clear_cover_location_marks_and_releases() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let cover = e.mem.alloc(0x78);
        e.mem.set_u32(cover + 0x68, 0x99);
        e.set(f.state, CombatState::pCoverLocation, cover);
        stub(&mut e, 0x005d_c960, 1);
        let entry = e.mem.alloc(8);
        stub(&mut e, 0x0069_50d0, entry);
        let log = logged(&mut e, |e| {
            e.call(0x009a_03d0, &args![f.state, 1u32]);
        });
        assert_eq!(e.mem.u32(entry), 0x99);
        assert_eq!(e.mem.f32(entry + 4), 5.0);
        assert_eq!(calls(&log, 0x009a_05b0), vec![vec![f.state.addr(), cover]]);
        assert_eq!(calls(&log, 0x006d_3370), vec![vec![cover, 1]]);
        assert_eq!(e.get(f.state, CombatState::pCoverLocation), 0);
        // Nothing to clear: nothing happens.
        let log = logged(&mut e, |e| {
            e.call(0x009a_03d0, &args![f.state, 1u32]);
        });
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn clear_secondary_cover_location_uses_its_own_field() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let cover = e.mem.alloc(0x78);
        e.set(f.state, CombatState::pSecondaryCoverLocation, cover);
        e.set(f.state, CombatState::pCoverLocation, 0x4444);
        let log = logged(&mut e, |e| {
            e.call(0x009a_0470, &args![f.state, 0u32]);
        });
        assert!(calls(&log, 0x0084_5460).is_empty());
        assert!(calls(&log, 0x009a_05b0).is_empty());
        assert_eq!(calls(&log, 0x006d_3370), vec![vec![cover, 1]]);
        assert_eq!(e.get(f.state, CombatState::pSecondaryCoverLocation), 0);
        assert_eq!(e.get(f.state, CombatState::pCoverLocation), 0x4444);
    }

    #[test]
    fn is_cover_location_valid_matches_either_location() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.set(f.state, CombatState::pCoverLocation, 0x10);
        e.set(f.state, CombatState::pSecondaryCoverLocation, 0x20);
        let run = |e: &mut Engine, cover: u32| e.call(0x009a_0600, &args![f.state, cover]).bool();
        assert!(run(&mut e, 0x10));
        assert!(run(&mut e, 0x20));
        assert!(!run(&mut e, 0x30));
        e.set(f.state, CombatState::pCoverLocation, 0);
        assert!(!run(&mut e, 0));
    }

    #[test]
    fn combat_location_is_freed_then_replaced() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.set(f.state, CombatState::pCombatLocation, 0x77);
        let log = logged(&mut e, |e| {
            e.call(0x009a_0650, &args![f.state, 0x88u32]);
        });
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![0x77]]);
        assert_eq!(e.get(f.state, CombatState::pCombatLocation), 0x88);
        let log = logged(&mut e, |e| {
            e.call(0x009a_0680, &args![f.state]);
        });
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![0x88]]);
        assert_eq!(e.get(f.state, CombatState::pCombatLocation), 0);
    }

    #[test]
    fn set_combat_item_stores_a_new_record() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.register(0x0096_a2d0, |_, a| ret(a[0]));
        let log = logged(&mut e, |e| {
            e.call(0x009a_06c0, &args![f.state, 2u32, 0x500u32, 0x600u32]);
        });
        assert_eq!(calls(&log, 0x0098_1340), vec![vec![f.state.addr(), 2]]);
        let record = e.mem.u32(f.state.addr() + 0x198);
        assert_eq!((e.mem.u32(record), e.mem.u32(record + 4)), (0x500, 0x600));
        assert_eq!(calls(&log, 0x0056_4db0), vec![vec![0x500, 1]]);
        // No item: only the release.
        let log = logged(&mut e, |e| {
            e.call(0x009a_06c0, &args![f.state, 1u32, 0u32, 0x600u32]);
        });
        assert!(calls(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls(&log, 0x0098_1340).len(), 1);
    }

    #[test]
    fn location_target_constructor_clears_it() {
        let mut e = engine();
        let target = e.mem.alloc(0x20);
        e.mem.write(target, &[0xcd; 0x20]);
        assert_eq!(e.call(0x009a_08d0, &args![target]).u32(), target);
        assert_eq!(e.mem.u16(target), 0);
        assert_eq!(e.mem.u32(target + 4), 0);
        assert_eq!(e.mem.u32(target + 0x18), 0);
        assert_eq!(e.mem.u32(target + 0x1c), 0);
    }

    #[test]
    fn set_location_target_ref_and_world_location() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        put_setting(&mut e, LOCATION_TARGET_DISTANCE_SETTING, 7.0);
        sequence_st0(&mut e, 0x0040_4010, vec![50.0, 60.0]);
        let world = e.mem.alloc(16);
        for k in 0..4u32 {
            e.mem.set_u32(world + 4 * k, 0x100 + k);
        }
        stub(&mut e, GET_WORLD_LOCATION, world);
        let log = logged(&mut e, |e| {
            e.call(0x009a_0790, &args![f.state, 0x900u32, 2u32, 1.0f32, 2.0f32]);
        });
        let target = e.get(f.state, CombatState::pLocationTarget);
        assert_eq!(e.mem.u8(target), 2);
        assert_eq!(e.mem.u32(target + 4), 0x900);
        assert_eq!(e.mem.u32(target + 0x14), 0x103);
        assert_eq!(e.mem.f32(target + 0x18), 50.0);
        assert_eq!(e.mem.f32(target + 0x1c), 60.0);
        assert_eq!(
            calls(&log, 0x0040_4010),
            vec![
                vec![1.0f32.to_bits(), 7.0f32.to_bits()],
                vec![2.0f32.to_bits(), 7.0f32.to_bits()]
            ]
        );
        // No reference: the old target is freed and none is made.
        let log = logged(&mut e, |e| {
            e.call(0x009a_0790, &args![f.state, 0u32, 2u32, 1.0f32, 2.0f32]);
        });
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![target]]);
        assert_eq!(e.get(f.state, CombatState::pLocationTarget), 0);
        // A world location instead.
        sequence_st0(&mut e, 0x0040_4010, vec![5.0, 6.0]);
        e.call(0x009a_0910, &args![f.state, world, 3u32, 1.0f32, 2.0f32]);
        let target = e.get(f.state, CombatState::pLocationTarget);
        assert_eq!((e.mem.u8(target), e.mem.u32(target + 4)), (3, 0));
        assert_eq!(e.mem.u32(target + 8), 0x100);
        assert_eq!(e.mem.f32(target + 0x1c), 6.0);
        e.call(0x009a_0a40, &args![f.state]);
        assert_eq!(e.get(f.state, CombatState::pLocationTarget), 0);
    }

    #[test]
    fn location_target_distances() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let target = new_location_target(&mut e, f.state, 2);
        e.mem.set_f32(target + 0x18, 100.0);
        e.mem.set_f32(target + 0x1c, 200.0);
        // With a target and no reason: 0.9 of the minimum.
        let value = e.call(0x009a_0b00, &args![f.state]).f64();
        assert_eq!(value as f32, (100.0 * f64::from(0.9f32)) as f32);
        // `0097ee90` holds: the maximum.
        stub(&mut e, 0x0097_ee90, 1);
        assert!(e.call(0x009a_0a80, &args![f.state]).bool());
        assert_eq!(e.call(0x009a_0b00, &args![f.state]).f64(), 200.0);
        // No target: type 2 counts.
        e.mem.set_u32(f.controller + 0xc0, 0);
        stub(&mut e, 0x0097_ee90, 0);
        assert!(e.call(0x009a_0a80, &args![f.state]).bool());
        e.mem.set_u8(target, 1);
        assert!(!e.call(0x009a_0a80, &args![f.state]).bool());
        e.set(f.state, CombatState::pLocationTarget, 0);
        assert_eq!(e.call(0x009a_0b30, &args![f.state]).f64(), 0.0);
        // Three quarters of the way, or the maximum plus the setting when near.
        e.set(f.state, CombatState::pLocationTarget, target);
        assert_eq!(e.call(0x009a_0b70, &args![f.state]).f64(), 175.0);
        put_setting(&mut e, LOCATION_TARGET_BONUS_SETTING, 32.0);
        e.mem.set_u8(target + 1, 1);
        assert_eq!(e.call(0x009a_0b70, &args![f.state]).f64(), 232.0);
        // `009a0be0`: the bonus only for a group with another kind.
        assert_eq!(e.call(0x009a_0be0, &args![f.state]).f64(), 200.0);
        stub(&mut e, 0x004f_9bf0, 0x40);
        stub(&mut e, 0x0082_5c00, 6);
        assert_eq!(e.call(0x009a_0be0, &args![f.state]).f64(), 200.0);
        stub(&mut e, 0x0082_5c00, 2);
        assert_eq!(e.call(0x009a_0be0, &args![f.state]).f64(), 232.0);
    }

    #[test]
    fn fn_009a0c50_makes_a_range_target() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        put_setting(&mut e, 0x011c_fb2c, 0.5);
        put_setting(&mut e, LOCATION_TARGET_DISTANCE_SETTING, 0.0);
        e.register(0x0040_4010, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        let world = e.mem.alloc(16);
        stub(&mut e, GET_WORLD_LOCATION, world);
        // No range: nothing.
        e.call(0x009a_0c50, &args![f.state, 0u32]);
        assert_eq!(e.get(f.state, CombatState::pLocationTarget), 0);
        stub_st0(&mut e, 0x0050_7b00, 400.0);
        e.call(0x009a_0c50, &args![f.state, 0u32]);
        let target = e.get(f.state, CombatState::pLocationTarget);
        assert_eq!(e.mem.u8(target), 3);
        assert_eq!(e.mem.f32(target + 0x18), 200.0);
        assert_eq!(e.mem.f32(target + 0x1c), 400.0);
        // A type 3 target is kept unless replacing.
        stub_st0(&mut e, 0x0050_7b00, 100.0);
        e.call(0x009a_0c50, &args![f.state, 0u32]);
        assert_eq!(e.mem.f32(target + 0x1c), 400.0);
        e.call(0x009a_0c50, &args![f.state, 1u32]);
        let target = e.get(f.state, CombatState::pLocationTarget);
        assert_eq!(e.mem.f32(target + 0x1c), 100.0);
    }

    #[test]
    fn unreachable_cover_locations_age_out() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let fresh = e.mem.alloc(8);
        e.mem.set_u32(fresh, 0x11);
        e.mem.set_f32(fresh + 4, 1.0);
        let stale = e.mem.alloc(8);
        e.mem.set_u32(stale, 0x22);
        e.mem.set_f32(stale + 4, -10.0);
        // Two entries, the first stale: it is removed and the size drops.
        let entries = Rc::new(RefCell::new(vec![stale, fresh]));
        let size = entries.clone();
        e.register_double(ARRAY_SIZE, move |_, _| ret(size.borrow().len() as u32));
        let get = entries.clone();
        e.register_double(0x0084_4160, move |_, a| ret(get.borrow()[a[1] as usize]));
        let remove = entries.clone();
        e.register_double(0x0098_3720, move |_, a| {
            remove.borrow_mut().remove(a[1] as usize);
            ret(0)
        });
        let log = logged(&mut e, |e| {
            assert!(e.call(0x009a_0d70, &args![f.state, 0x11u32]).bool());
        });
        assert_eq!(
            calls(&log, 0x0098_3720),
            vec![vec![f.state.addr() + 0x164, 0, 1]]
        );
        assert!(!e.call(0x009a_0d70, &args![f.state, 0x33u32]).bool());
        // Building the array adds the fresh entries.
        entries.borrow_mut().insert(0, stale);
        let log = logged(&mut e, |e| {
            e.call(0x009a_0e10, &args![f.state, 0x7000u32]);
        });
        assert_eq!(calls(&log, 0x0084_b5c0), vec![vec![0x7000, 2]]);
        assert_eq!(calls(&log, 0x007c_b2e0), vec![vec![0x7000, fresh]]);
        // Adding one: key and stamp.
        let entry = e.mem.alloc(8);
        stub(&mut e, 0x0069_50d0, entry);
        e.call(0x009a_0d00, &args![f.state, 0x44u32]);
        assert_eq!((e.mem.u32(entry), e.mem.f32(entry + 4)), (0x44, 5.0));
        let log = logged(&mut e, |e| {
            e.call(0x009a_0d00, &args![f.state, 0u32]);
        });
        assert!(calls(&log, 0x0084_5460).is_empty());
    }

    #[test]
    fn unreachable_locations_add_and_match() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let entry = e.mem.alloc(0x14);
        stub(&mut e, 0x006b_3160, entry);
        let world = e.mem.alloc(16);
        for k in 0..4u32 {
            e.mem.set_u32(world + 4 * k, 0x50 + k);
        }
        e.call(0x009a_0ec0, &args![f.state, world]);
        assert_eq!(e.mem.u32(entry + 0xc), 0x53);
        assert_eq!(e.mem.f32(entry + 0x10), 5.0);
        // `009a0f40`: within the radius squared, -1 meaning 16384.
        stub(&mut e, ARRAY_SIZE, 1);
        stub(&mut e, 0x006d_ad90, entry);
        stub_st0(&mut e, 0x0052_7a40, 99.0);
        let run = |e: &mut Engine, radius: f32| {
            e.call(0x009a_0f40, &args![f.state, 0x600u32, radius])
                .bool()
        };
        assert!(run(&mut e, 10.0));
        assert!(!run(&mut e, 9.0));
        stub_st0(&mut e, 0x0052_7a40, 16000.0);
        assert!(run(&mut e, -1.0));
        // A stale entry is removed instead.
        e.mem.set_f32(entry + 0x10, -10.0);
        sequence(&mut e, ARRAY_SIZE, vec![1, 0]);
        let log = logged(&mut e, |e| assert!(!run(e, -1.0)));
        assert_eq!(calls(&log, 0x0098_f1f0)[0][1..], [0, 1]);
    }

    #[test]
    fn fn_009a1010_updates_damage_and_strength() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        stub_st0(&mut e, 0x008b_e600, 12.5);
        stub_st0(&mut e, 0x008a_cbe0, 40.0);
        let log = logged(&mut e, |e| {
            e.call(0x009a_1010, &args![f.state]);
        });
        assert_eq!(e.get(f.state, CombatState::fDamagePerSecond), 12.5);
        assert_eq!(e.get(f.state, CombatState::fCombatStrength), 40.0);
        assert_eq!(
            calls(&log, 0x008a_cbe0),
            vec![vec![f.attacker, 12.5f32.to_bits()]]
        );
    }

    #[test]
    fn update_threat_ratio_branches() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.set(f.state, CombatState::fThreatRatio, 7.0);
        // Fleeing in state 1: untouched.
        e.set(f.state, CombatState::cFleeState, 1);
        e.call(0x009a_1060, &args![f.state]);
        assert_eq!(e.get(f.state, CombatState::fThreatRatio), 7.0);
        // Damaged long ago: reset to 100 only.
        e.set(f.state, CombatState::cFleeState, 0);
        e.mem.set_f32(f.state.addr() + 0xa0, -200.0);
        e.call(0x009a_1060, &args![f.state]);
        assert_eq!(e.get(f.state, CombatState::fThreatRatio), 100.0);
        // Recently damaged: threat over strength (group +0xc8).
        e.mem.set_f32(f.state.addr() + 0xa0, 4.0);
        stub_st0(&mut e, 0x0098_cb30, 30.0);
        e.mem.set_f32(f.group + 0xc8, 10.0);
        e.call(0x009a_1060, &args![f.state]);
        assert_eq!(e.get(f.state, CombatState::fThreatRatio), 3.0);
        // The other ratio replaces a larger one when `009a1240` holds.
        stub(&mut e, 0x0099_28c0, 1);
        stub_st0(&mut e, 0x0098_cc20, 5.0);
        stub_st0(&mut e, 0x0045_36e0, 4.0);
        stub_st0(&mut e, 0x008a_cbe0, 8.0);
        e.call(0x009a_1060, &args![f.state]);
        assert_eq!(e.get(f.state, CombatState::fThreatRatio), 2.0);
        // Slot 0xc of the +0xa4 object returning 3 stops after the reset.
        slot_st0(&mut e, 0xc, 3.0);
        e.call(0x009a_1060, &args![f.state]);
        assert_eq!(e.get(f.state, CombatState::fThreatRatio), 100.0);
        // Fleeing in state 2 with a target near: confidence boosted.
        slot_st0(&mut e, 0xc, 0.0);
        e.set(f.state, CombatState::cFleeState, 2);
        e.set(f.state, CombatState::cCurrentConfidence, 1);
        stub(&mut e, 0x0098_9700, 0x99);
        e.call(0x009a_1060, &args![f.state]);
        assert_eq!(e.get(f.state, CombatState::cCurrentConfidence), 2);
        assert_eq!(e.get(f.state, CombatState::cFleeState), 0);
    }

    #[test]
    fn fn_009a1240_returns_009928c0s_word() {
        let mut e = engine();
        stub(&mut e, 0x0099_28c0, 0x1234_5601);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009a_1240, &args![0x700u32]).u32(), 0x1234_5601);
        });
        assert_eq!(calls(&log, 0x0099_28c0), vec![vec![0x700, 4]]);
    }

    #[test]
    fn boost_confidence_branches() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.set(f.state, CombatState::cCurrentConfidence, 4);
        let log = logged(&mut e, |e| {
            e.call(0x009a_1280, &args![f.state, 1u32]);
        });
        assert_eq!(log.len(), 1);
        // Below 4: raised while `0047eeb0` is under 100; the flee state reset.
        e.set(f.state, CombatState::cCurrentConfidence, 2);
        e.set(f.state, CombatState::cFleeState, 2);
        e.call(0x009a_1280, &args![f.state, 1u32]);
        assert_eq!(e.get(f.state, CombatState::cCurrentConfidence), 3);
        assert_eq!(e.get(f.state, CombatState::cFleeState), 0);
        // A type 1 location target near the attacker keeps the flee state.
        stub_st0(&mut e, 0x0047_eeb0, 100.0);
        e.set(f.state, CombatState::cFleeState, 2);
        new_location_target(&mut e, f.state, 1);
        stub_st0(&mut e, 0x0052_7a40, 100.0);
        e.call(0x009a_1280, &args![f.state, 1u32]);
        assert_eq!(e.get(f.state, CombatState::cCurrentConfidence), 3);
        assert_eq!(e.get(f.state, CombatState::cFleeState), 2);
        // A type 12 cover location is cleared.
        let cover = e.mem.alloc(0x78);
        e.mem.set_u32(cover + 4, 0xc);
        e.set(f.state, CombatState::pCoverLocation, cover);
        e.call(0x009a_1280, &args![f.state, 0u32]);
        assert_eq!(e.get(f.state, CombatState::pCoverLocation), 0);
    }

    #[test]
    fn fn_009a1380_compares_confidence_and_ratio() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        e.set(f.state, CombatState::fThreatRatio, 2.0);
        e.set(f.state, CombatState::fConfidenceModifier, 1.0);
        stub_st0(&mut e, 0x0047_eeb0, 4.0);
        assert!(e.call(0x009a_1380, &args![f.state]).bool());
        stub_st0(&mut e, 0x0047_eeb0, 3.0);
        assert!(!e.call(0x009a_1380, &args![f.state]).bool());
        // Zero is not reduced.
        stub_st0(&mut e, 0x0047_eeb0, 0.0);
        e.set(f.state, CombatState::fThreatRatio, -1.0);
        assert!(e.call(0x009a_1380, &args![f.state]).bool());
        e.mem.set_u32(f.controller + 0xc0, 0);
        assert!(!e.call(0x009a_1380, &args![f.state]).bool());
    }

    #[test]
    fn fn_009a1400_queues_path_checks() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        // No target: nothing.
        let lonely = fixture(&mut e, false);
        assert!(!e.call(0x009a_1400, &args![lonely.state, 0u32]).bool());
        // Kind 0: a request for the target's world location, the queue created first.
        stub(&mut e, 0x00aa_13e0, 0x5000);
        stub(&mut e, 0x008d_f900, 0x5100);
        stub(&mut e, 0x006e_2420, 0x5200);
        stub(&mut e, 0x0084_e3a0, 0x33);
        stub(&mut e, 0x0047_d0b0, 0x9000);
        e.register(0x0063_3c90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(0x006e_bf30, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            ret(a[0])
        });
        e.set(f.state, CombatState::spPathCheckQueue, 0x7700);
        e.register(0x0055_9450, |_, _| ret(0));
        let log = logged(&mut e, |e| {
            assert!(e.call(0x009a_1400, &args![f.state, 0u32]).bool());
        });
        assert_eq!(
            calls(&log, 0x0090_57d0),
            vec![vec![f.state.addr() + 0x224, 0x5100]]
        );
        let built = calls(&log, 0x008b_38d0);
        assert_eq!(built[0][0..2], [f.attacker, 0x5200]);
        assert_eq!(built[0][3..], [0x33, (-1.0f32).to_bits(), 0]);
        assert_eq!(
            calls(&log, 0x006e_b9d0),
            vec![vec![0x9000, 0x5200, 0x7700, 1]]
        );
        assert_eq!(e.get(f.state, CombatState::iQueuedPathCheck), 0);
        // Kind 1: an invalid face location refuses the check.
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x009a_1400, &args![f.state, 1u32]).bool());
        });
        assert_eq!(calls(&log, 0x004f_f7e0).len(), 1);
        assert!(calls(&log, 0x006e_b9d0).is_empty());
        assert_eq!(e.get(f.state, CombatState::iQueuedPathCheck), 0);
        stub(&mut e, 0x006d_ed70, 1);
        stub(&mut e, 0x0044_1110, 0x44);
        stub(&mut e, 0x0096_11e0, 0x45);
        stub(&mut e, 0x005c_3420, 0x46);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x009a_1400, &args![f.state, 1u32]).bool());
        });
        assert_eq!(
            calls(&log, 0x008b_3880)[0][2..],
            [0x46, 0x45, 0x44, (-1.0f32).to_bits(), 0]
        );
        assert_eq!(calls(&log, 0x006e_b9d0).len(), 1);
        assert_eq!(e.get(f.state, CombatState::iQueuedPathCheck), 1);
        // Another kind only records it.
        let log = logged(&mut e, |e| {
            assert!(e.call(0x009a_1400, &args![f.state, 5u32]).bool());
        });
        assert!(calls(&log, 0x006e_b9d0).is_empty());
        assert_eq!(e.get(f.state, CombatState::iQueuedPathCheck), 5);
    }

    #[test]
    fn init_load_game_resolves_the_pointers() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.register(0x0048_39c0, |_, a| ret(a[0] + 0x1000));
        e.register(0x00ec_43fb, |_, a| ret(a[0] + a[3]));
        let target = new_location_target(&mut e, f.state, 1);
        e.mem.set_u32(target + 4, 0x20);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, 0x30);
        e.mem.set_u32(f.state.addr() + 0x194, record);
        let empty = e.mem.alloc(8);
        e.mem.set_u32(f.state.addr() + 0x198, empty);
        let location = e.mem.alloc(0x20);
        e.mem.set_u32(location + 0x10, 0x40);
        e.set(f.state, CombatState::pCombatLocation, location);
        e.set(f.state, CombatState::pLastTarget, 0x50);
        e.set(f.state, CombatState::iQueuedPathCheck, -1);
        for (index, dps) in [3.0f32, 9.0, 4.0, 0.0, 1.0, 2.0, 5.0].iter().enumerate() {
            e.mem
                .set_f32(f.state.addr() + 0x3c + 4 * index as u32, *dps);
        }
        e.register(0x0040_4010, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        let log = logged(&mut e, |e| {
            e.call(0x009a_2a30, &args![f.state, 0x8000u32]);
        });
        assert_eq!(e.mem.u32(target + 4), 0x20 + 0x1000 + TES_OBJECT_REFR_TYPE);
        assert_eq!(e.mem.u32(record), 0x30 + 0x1000 + TES_OBJECT_REFR_TYPE);
        assert_eq!(e.mem.u32(empty), 0);
        assert_eq!(
            e.mem.u32(location + 0x10),
            0x40 + 0x1000 + TES_OBJECT_REFR_TYPE
        );
        assert_eq!(
            e.get(f.state, CombatState::pLastTarget),
            0x50 + 0x1000 + ACTOR_TYPE
        );
        assert_eq!(calls(&log, 0x0056_4db0).len(), 1);
        assert_eq!(
            e.get(f.state, CombatState::fMaxModifiedCombatWeaponDPS),
            9.0
        );
        assert!(calls(&log, 0x00aa_13e0).is_empty());
    }

    #[test]
    fn combat_location_save_load_and_resolve() {
        let mut e = engine();
        let location = e.mem.alloc(0x20);
        e.mem.set_u32(location + 0x10, 0x77);
        let log = logged(&mut e, |e| {
            e.call(0x009a_2bf0, &args![location, 0x8000u32]);
            e.call(0x009a_2c60, &args![location, 0x8000u32]);
        });
        assert_eq!(
            calls(&log, WORLD_LOCATION_SAVE),
            vec![vec![location, 0x8000]]
        );
        assert_eq!(
            calls(&log, SAVE_DATA),
            vec![
                vec![0x8000, location + 0x14, 4, 0],
                vec![0x8000, location + 0x18, 4, 0],
                vec![0x8000, location + 0x1c, 4, 0]
            ]
        );
        assert_eq!(calls(&log, SAVE_FORM_ID), vec![vec![0x8000, 0x77, 0]]);
        assert_eq!(calls(&log, LOAD_DATA).len(), 3);
        assert_eq!(
            calls(&log, LOAD_FORM_ID),
            vec![vec![0x8000, location + 0x10]]
        );
        e.register(0x0048_39c0, |_, a| ret(a[0] + 1));
        e.register(0x00ec_43fb, |_, a| ret(a[0] + 1));
        e.call(0x009a_2cc0, &args![location, 0u32]);
        assert_eq!(e.mem.u32(location + 0x10), 0x79);
        e.mem.set_u32(location + 0x10, 0);
        e.call(0x009a_2cc0, &args![location, 0u32]);
        assert_eq!(e.mem.u32(location + 0x10), 0);
    }

    #[test]
    fn unreachable_entries_save_and_load() {
        let mut e = engine();
        let entry = e.mem.alloc(0x14);
        let log = logged(&mut e, |e| {
            e.call(0x009a_2d20, &args![entry, 0x8000u32]);
            e.call(0x009a_2d50, &args![entry, 0x8000u32]);
            e.call(0x009a_2d80, &args![entry, 0x8000u32]);
            e.call(0x009a_2db0, &args![entry, 0x8000u32]);
        });
        assert_eq!(calls(&log, SAVE_DATA), vec![vec![0x8000, entry, 4, 0]]);
        assert_eq!(calls(&log, LOAD_DATA), vec![vec![0x8000, entry, 4]]);
        assert_eq!(
            calls(&log, TIME_STAMP_SAVE),
            vec![vec![entry + 4, 0x8000], vec![entry + 0x10, 0x8000]]
        );
        assert_eq!(
            calls(&log, TIME_STAMP_LOAD),
            vec![vec![entry + 4, 0x8000], vec![entry + 0x10, 0x8000]]
        );
        assert_eq!(calls(&log, WORLD_LOCATION_SAVE), vec![vec![entry, 0x8000]]);
        assert_eq!(calls(&log, WORLD_LOCATION_LOAD), vec![vec![entry, 0x8000]]);
    }

    #[test]
    fn cover_location_save_and_load() {
        let mut e = engine();
        let cover = e.mem.alloc(0x78);
        e.mem.set_u32(cover + 0x14, VTABLE);
        let log = logged(&mut e, |e| {
            e.call(0x009a_2de0, &args![cover, 0x8000u32]);
            e.call(0x009a_2ea0, &args![cover, 0x8000u32]);
        });
        let saved: Vec<(u32, u32)> = calls(&log, SAVE_DATA)
            .iter()
            .map(|w| (w[1] - cover, w[2]))
            .collect();
        assert_eq!(
            saved,
            vec![
                (1, 1),
                (0, 1),
                (2, 1),
                (4, 4),
                (8, 0xc),
                (0x68, 4),
                (0x6c, 4)
            ]
        );
        assert_eq!(calls(&log, LOAD_DATA).len(), 7);
        assert_eq!(calls(&log, SLOTS), vec![vec![cover + 0x14, 0x8000]]);
        assert_eq!(calls(&log, SLOTS + 4), vec![vec![cover + 0x14, 0x8000]]);
        assert_eq!(calls(&log, 0x0098_2c20), vec![vec![cover + 0x70, 0x8000]]);
        assert_eq!(calls(&log, 0x0098_2ca0), vec![vec![cover + 0x70, 0x8000]]);
    }

    #[test]
    fn cover_search_info_save_and_load() {
        let mut e = engine();
        let info: Ptr<CombatCoverSearchInfo> = e.new_object();
        let me = info.addr();
        let element = e.mem.alloc(0x54);
        e.mem.set_u32(element, VTABLE);
        stub(&mut e, ARRAY_SIZE, 2);
        stub(&mut e, ELEMENT_TARGET_009052C0, element);
        let log = logged(&mut e, |e| {
            e.call(0x009a_2f50, &args![info, 0x8000u32]);
        });
        let saved: Vec<(u32, u32)> = calls(&log, SAVE_DATA)
            .iter()
            .map(|w| (w[1] - me, w[2]))
            .collect();
        assert_eq!(
            saved,
            vec![
                (0, 0xc),
                (0xc, 0xc),
                (0x18, 4),
                (0x38, 1),
                (0x30, 4),
                (0x34, 4)
            ]
        );
        assert_eq!(calls(&log, SAVE_VARIABLE_SIZE), vec![vec![0x8000, 2]]);
        assert_eq!(calls(&log, SLOTS).len(), 2);
        // Load: the array sized to the loaded count first.
        stub(&mut e, LOAD_VARIABLE_SIZE, 1);
        e.mem.set_u32(me + 0x20, VTABLE);
        stub(&mut e, SLOTS + 4, 0x6000);
        let log = logged(&mut e, |e| {
            e.call(0x009a_3050, &args![info, 0x8000u32]);
        });
        assert_eq!(calls(&log, LOAD_DATA).len(), 6);
        assert_eq!(calls(&log, TIME_STAMP_LOAD), vec![vec![me + 0x1c, 0x8000]]);
        assert_eq!(e.mem.u32(me + 0x28), 1);
        assert_eq!(calls(&log, 0x006d_be40), vec![vec![me + 0x20, 0x6000, 1]]);
        assert_eq!(calls(&log, SLOTS + 4).len(), 2);
    }

    #[test]
    fn location_target_save_load_and_resolve() {
        let mut e = engine();
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(target + 4, 0x66);
        let log = logged(&mut e, |e| {
            e.call(0x009a_3130, &args![target, 0x8000u32]);
            e.call(0x009a_31b0, &args![target, 0x8000u32]);
        });
        let saved: Vec<(u32, u32)> = calls(&log, SAVE_DATA)
            .iter()
            .map(|w| (w[1] - target, w[2]))
            .collect();
        assert_eq!(saved, vec![(0, 1), (1, 1), (0x18, 4), (0x1c, 4)]);
        assert_eq!(calls(&log, SAVE_FORM_ID), vec![vec![0x8000, 0x66, 0]]);
        assert_eq!(
            calls(&log, WORLD_LOCATION_SAVE),
            vec![vec![target + 8, 0x8000]]
        );
        assert_eq!(calls(&log, LOAD_FORM_ID), vec![vec![0x8000, target + 4]]);
        assert_eq!(
            calls(&log, WORLD_LOCATION_LOAD),
            vec![vec![target + 8, 0x8000]]
        );
        e.register(0x0048_39c0, |_, a| ret(a[0] * 2));
        e.register(0x00ec_43fb, |_, a| ret(a[0] + 1));
        e.call(0x009a_3220, &args![target, 0u32]);
        assert_eq!(e.mem.u32(target + 4), 0xcd);
    }

    /// The `SetSize` instances: grow from empty through slot 4, grow with a reallocation,
    /// shrink with and without giving the memory back, and size 0.
    fn check_set_size(addr: u32, instance: &ArrayInstance) {
        let mut e = engine();
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr(), VTABLE);
        stub(&mut e, SLOTS + 4, 0x5000);
        let log = logged(&mut e, |e| {
            e.call(addr, &args![array, 3u32, 1u32]);
        });
        assert_eq!(
            calls(&log, instance.construct),
            vec![vec![array.addr(), 0x5000, 3]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 3);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        let log = logged(&mut e, |e| {
            e.call(addr, &args![array, 5u32, 1u32]);
        });
        assert_eq!(
            calls(&log, instance.reallocate),
            vec![vec![array.addr(), 5, 3]]
        );
        assert_eq!(
            calls(&log, instance.construct),
            vec![vec![array.addr(), 0x5000 + 3 * instance.stride, 2]]
        );
        e.set(array, BSSimpleArray::iReservedSize, 8);
        let log = logged(&mut e, |e| {
            e.call(addr, &args![array, 2u32, 1u32]);
        });
        assert_eq!(
            calls(&log, instance.destroy),
            vec![vec![array.addr(), 0x5000 + 2 * instance.stride, 3]]
        );
        assert_eq!(
            calls(&log, instance.reallocate),
            vec![vec![array.addr(), 2, 2]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 2);
        let log = logged(&mut e, |e| {
            e.call(addr, &args![array, 1u32, 0u32]);
        });
        assert!(calls(&log, instance.reallocate).is_empty());
        assert_eq!(e.get(array, BSSimpleArray::iSize), 1);
        let log = logged(&mut e, |e| {
            e.call(addr, &args![array, 0u32, 1u32]);
        });
        assert_eq!(calls(&log, instance.release), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn fn_009a32a0_sets_the_cover_location_array_size() {
        check_set_size(
            0x009a_32a0,
            &ArrayInstance {
                release: 0x006d_bde0,
                reallocate: 0x006d_bfc0,
                construct: 0x006d_be40,
                destroy: 0x006d_bee0,
                stride: 0x54,
            },
        );
    }

    #[test]
    fn fn_009a3480_sets_the_unreachable_cover_array_size() {
        check_set_size(
            0x009a_3480,
            &ArrayInstance {
                release: 0x0084_54f0,
                reallocate: 0x006a_2770,
                construct: 0x009a_3950,
                destroy: 0x0072_ba80,
                stride: 8,
            },
        );
    }

    #[test]
    fn fn_009a3610_sets_the_unreachable_location_array_size() {
        check_set_size(
            0x009a_3610,
            &ArrayInstance {
                release: 0x0084_54f0,
                reallocate: 0x009a_3c50,
                construct: 0x009a_3a80,
                destroy: 0x0072_ba80,
                stride: 0x14,
            },
        );
    }

    #[test]
    fn array_constructors_set_their_vtables() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009a_33e0, &args![array]).u32(), array);
        });
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_0108E068);
        assert_eq!(calls(&log, 0x006b_3eb0), vec![vec![array, 0, 0]]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009a_3750, &args![array, 4u32, 2u32]).u32(), array);
        });
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_0108B4C8);
        assert_eq!(calls(&log, 0x006b_3eb0), vec![vec![array, 4, 2]]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009a_37e0, &args![array, 4u32, 2u32]).u32(), array);
        });
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_0108E0A4);
        assert_eq!(calls(&log, 0x009a_3d00), vec![vec![array, 4, 2]]);
    }

    #[test]
    fn unreachable_array_constructors_initialise_empty() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        e.mem.write(array, &[0xcd; 0x10]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009a_3430, &args![array]).u32(), array);
        });
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_0108E07C);
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0, 0, 0)
        );
        assert!(calls(&log, 0x009a_3950).is_empty());
        e.call(0x009a_35c0, &args![array]);
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_0108E090);
        // With sizes: the reservation is at least the size.
        e.mem.set_u32(array, VTABLE);
        stub(&mut e, SLOTS + 4, 0x7000);
        let log = logged(&mut e, |e| {
            e.call(0x009a_39f0, &args![array, 1u32, 3u32]);
        });
        assert_eq!(calls(&log, SLOTS + 4), vec![vec![array, 3]]);
        assert_eq!(calls(&log, 0x009a_3950), vec![vec![array, 0x7000, 3]]);
        assert_eq!((e.mem.u32(array + 8), e.mem.u32(array + 0xc)), (3, 3));
        let log = logged(&mut e, |e| {
            e.call(0x009a_3bc0, &args![array, 5u32, 0u32]);
        });
        assert!(calls(&log, 0x009a_3a80).is_empty());
        assert_eq!((e.mem.u32(array + 8), e.mem.u32(array + 0xc)), (0, 5));
    }

    #[test]
    fn fn_009a3780_copies_each_element() {
        let mut e = engine();
        let from: Ptr<BSSimpleArray> = e.new_object();
        let to: Ptr<BSSimpleArray> = e.new_object();
        e.set(from, BSSimpleArray::pBuffer, 0x4000);
        e.set(from, BSSimpleArray::iSize, 2);
        e.set(to, BSSimpleArray::iSize, 2);
        e.set(to, BSSimpleArray::iReservedSize, 2);
        e.register(ELEMENT_TARGET_009052C0, |_, a| ret(0x9000 + a[1]));
        let log = logged(&mut e, |e| {
            e.call(0x009a_3780, &args![from, to]);
        });
        assert_eq!(
            calls(&log, 0x006e_4430),
            vec![vec![0x9000, 0x4000], vec![0x9001, 0x4054]]
        );
    }

    #[test]
    fn array_deleting_destructors_free_on_bit_0() {
        let mut e = engine();
        for (addr, vtable) in [
            (0x009a_3850u32, ARRAY_VTABLE_0108E068),
            (0x009a_3880, ARRAY_VTABLE_0108E07C),
            (0x009a_38b0, ARRAY_VTABLE_0108E090),
            (0x009a_38e0, ARRAY_VTABLE_0108E0A4),
        ] {
            let array = e.mem.alloc(0x10);
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(addr, &args![array, 1u32]).u32(), array);
            });
            assert_eq!(e.mem.u32(array), vtable);
            assert_eq!(calls(&log, ARRAY_RELEASE_008454F0), vec![vec![array, 1]]);
            assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![array]]);
            let log = logged(&mut e, |e| {
                e.call(addr, &args![array, 0u32]);
            });
            assert!(calls(&log, OPERATOR_DELETE).is_empty());
        }
    }

    #[test]
    fn cover_reservations_use_the_secondary_location() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        e.set(f.state, CombatState::pSecondaryCoverLocation, 0x88);
        let log = logged(&mut e, |e| {
            e.call(0x009d_6c80, &args![f.state]);
            e.call(0x009d_7c20, &args![f.state]);
        });
        assert_eq!(calls(&log, 0x009a_05b0), vec![vec![f.state.addr(), 0x88]]);
        assert_eq!(calls(&log, 0x009a_0510), vec![vec![f.state.addr(), 0x88]]);
    }

    /// The objects a `PathingRequestCoveredMove` hands `0099d480`.
    struct CoveredMove {
        request: u32,
        start: u32,
        goal: u32,
        cover: u32,
        info: u32,
        candidate: u32,
    }

    /// A request whose start and goal are 1000 apart (squared `1e6`), whose paths
    /// (`006d5490`) succeed with length 100 from the start and 30 otherwise, and whose
    /// search fills one candidate.
    fn covered_move(e: &mut Engine) -> CoveredMove {
        let request = e.mem.alloc(0x120);
        let start = e.mem.alloc(0x30);
        let goal = e.mem.alloc(0x30);
        let cover = e.mem.alloc(0x78);
        let info = e.mem.alloc(0x3c);
        let candidate = e.mem.alloc(0x54);
        stub(e, 0x0048_d150, start);
        stub(e, 0x006a_9540, goal);
        stub(e, 0x0069_e390, 0x77);
        stub(e, 0x0068_efb0, cover);
        stub(e, 0x004f_8960, info);
        stub(e, 0x006d_62e0, 1);
        stub(e, ELEMENT_TARGET_009052C0, candidate);
        stub(e, 0x006e_4570, 0x42);
        stub(e, 0x0071_9b20, u32::MAX);
        stub(e, 0x0098_35d0, 1);
        stub(e, 0x006d_ed70, 1);
        stub_st0(e, 0x0045_7910, 1000.0);
        stub_st0(e, 0x0045_7990, 70.0);
        stub_st0(e, 0x006d_2c40, 2.0);
        let list = info + 0x20;
        e.register_double(ARRAY_SIZE, move |_, a| ret(u32::from(a[0] == list)));
        e.register_double(0x006d_5490, move |e, a| {
            if a[3] != 0 {
                let length = if a[0] == start { 100.0 } else { 30.0 };
                e.mem.set_f32(a[3], length);
            }
            ret(1)
        });
        sequence_st0(e, 0x004a_7290, vec![1_000_000.0, 100.0, 5000.0]);
        CoveredMove {
            request,
            start,
            goal,
            cover,
            info,
            candidate,
        }
    }

    #[test]
    fn fn_0099d480_chooses_a_candidate() {
        let mut e = engine();
        let m = covered_move(&mut e);
        e.mem.set_u32(COMBAT_MANAGER, 0x6000);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0099_d480, &args![m.request, 64.0f32, 0u32]).bool());
        });
        assert_eq!(e.mem.u8(m.candidate + 0x52), 0x13);
        assert_eq!(e.mem.u32(m.cover + 0x68), 0x42);
        assert_eq!(e.mem.f32(m.info + 0x18), 100.0);
        assert_eq!(e.mem.f32(m.info + 0x1c), 5.0);
        assert_eq!(
            calls(&log, 0x006e_4430),
            vec![vec![m.cover + 0x14, m.candidate]]
        );
        assert_eq!(calls(&log, 0x006d_30b0).len(), 1);
        assert_eq!(calls(&log, 0x0054_07b0), vec![vec![m.cover, 0]]);
        // The reservations were cleared first, and not refreshed.
        assert_eq!(calls(&log, 0x00aa_6060).len(), 1);
        assert!(calls(&log, 0x0099_5890).is_empty());
        // The paths: start to goal, the candidate to the goal, the start to it.
        let paths = calls(&log, 0x006d_5490);
        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0][..3], [m.start, m.goal, 0x77]);
        assert_eq!(paths[2][0], m.start);
        // Destroyed: the steps, both locations and the cover request.
        assert_eq!(calls(&log, 0x006a_bc50).len(), 1);
        assert_eq!(calls(&log, 0x004f_f7e0).len(), 2);
        assert_eq!(calls(&log, 0x006d_ad70).len(), 1);
    }

    #[test]
    fn fn_0099d480_rejects_and_refreshes() {
        let mut e = engine();
        let m = covered_move(&mut e);
        // Already unreachable.
        stub(&mut e, 0x0071_9b20, 0);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0099_d480, &args![m.request, 64.0f32, 0u32]).bool());
        });
        assert_eq!(e.mem.u8(m.candidate + 0x52), 3);
        assert_eq!(calls(&log, 0x0099_5890).len(), 1);
        // Threatened (with reservations kept: no lock, no refresh).
        stub(&mut e, 0x0071_9b20, u32::MAX);
        sequence_st0(&mut e, 0x004a_7290, vec![1_000_000.0, 100.0, 5000.0]);
        stub_st0(&mut e, 0x009a_5670, 1.0);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0099_d480, &args![m.request, 64.0f32, 1u32]).bool());
        });
        assert_eq!(e.mem.u8(m.candidate + 0x52), 0xb);
        assert!(calls(&log, 0x00aa_6060).is_empty());
        assert!(calls(&log, 0x0099_5890).is_empty());
        // Start and goal too near: nothing searched.
        sequence_st0(&mut e, 0x004a_7290, vec![100.0]);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0099_d480, &args![m.request, 64.0f32, 0u32]).bool());
        });
        assert!(calls(&log, 0x006d_5490).is_empty());
    }

    #[test]
    fn find_best_dodge_location_picks_a_direction() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let out = e.mem.alloc(0x20);
        let target_position = e.mem.alloc(12);
        put_point(&mut e, target_position, 100.0, 0.0, 0.0);
        slot(&mut e, 0x1f4, target_position);
        // Every direction is blocked unless `009a0260` allows it.
        stub(&mut e, 0x006d_7490, 1);
        let run = |e: &mut Engine, directions: u32| {
            e.call(
                0x0099_e190,
                &args![f.state, out, directions, 0u32, 50.0f32, 10.0f32],
            )
            .bool()
        };
        let log = logged(&mut e, |e| assert!(!run(e, 0xf)));
        assert_eq!(calls(&log, 0x009a_0260).len(), 4);
        assert_eq!(calls(&log, 0x004f_f7e0).len(), 5);
        // Directions 1 and 3 allowed; the random pick is the second (index 1 of the
        // possible ones, direction 3).
        e.register(0x009a_0260, |_, _| ret(1));
        stub(&mut e, 0x005a_00a0, 1);
        let log = logged(&mut e, |e| assert!(run(e, 0xa)));
        assert_eq!(e.get(f.state, CombatState::cLastDodgeDirection), 3);
        assert_eq!(e.mem.f32(f.state.addr() + 0x184), 5.0);
        assert_eq!(calls(&log, 0x005a_00a0), vec![vec![2]]);
        assert_eq!(calls(&log, 0x0098_9010).len(), 1);
        // The previous direction is kept while it is recent and possible.
        let log = logged(&mut e, |e| assert!(run(e, 0xa)));
        assert!(calls(&log, 0x005a_00a0).is_empty());
        // A threatened point is skipped.
        stub_st0(&mut e, 0x009a_5670, 1.0);
        assert!(!run(&mut e, 0xf));
    }

    #[test]
    fn fn_0099e840_finds_a_location_in_range() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let position = e.mem.alloc(12);
        slot(&mut e, 0x1f4, position);
        let out = e.mem.alloc(0x20);
        let run = |e: &mut Engine, check: u32| {
            e.call(
                0x0099_e840,
                &args![f.state, 100.0f32, 200.0f32, 10.0f32, 300.0f32, out, check],
            )
            .bool()
        };
        // No navmesh point: false.
        let log = logged(&mut e, |e| assert!(!run(e, 0)));
        assert_eq!(calls(&log, 0x004f_f7e0).len(), 1);
        stub(&mut e, 0x006d_6f80, 1);
        // The tolerance: |150 - 120| = 30 (`00408840` returns its argument here).
        stub_st0(&mut e, 0x0045_7910, 120.0);
        e.register(0x0040_8840, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).abs()))
        });
        // Navmesh point 31 away (961 > 900): false.
        sequence_st0(&mut e, 0x004a_7290, vec![961.0]);
        assert!(!run(&mut e, 0));
        // Within the tolerance and the range: the location is set.
        sequence_st0(&mut e, 0x004a_7290, vec![900.0, 400.0]);
        let log = logged(&mut e, |e| assert!(run(e, 0)));
        assert_eq!(calls(&log, 0x006d_7a40)[0][3], 60.0f32.to_bits());
        assert_eq!(calls(&log, 0x0098_9010).len(), 1);
        assert_eq!(calls(&log, 0x004f_f7e0).len(), 3);
        // Too near the target (below 10 squared), or out of sight.
        sequence_st0(&mut e, 0x004a_7290, vec![900.0, 99.0]);
        assert!(!run(&mut e, 0));
        sequence_st0(&mut e, 0x004a_7290, vec![900.0, 400.0]);
        assert!(!run(&mut e, 1));
        stub(&mut e, 0x009a_7940, 1);
        sequence_st0(&mut e, 0x004a_7290, vec![900.0, 400.0]);
        assert!(run(&mut e, 1));
    }

    #[test]
    fn find_best_flee_location_branches() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let out: Ptr<CombatLocation> = e.new_object();
        let list = e.mem.alloc(0x20);
        stub(&mut e, 0x0046_1110, list);
        put_setting(&mut e, 0x011c_f020, 100.0);
        put_setting(&mut e, 0x011c_e040, 1.0);
        put_setting(&mut e, 0x011c_f144, 4.0);
        let ally = object(&mut e, 0x200);
        // One ally in the list.
        stub(&mut e, 0x004b_9ba0, 1);
        e.register_double(0x006b_7f20, move |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[3], ally);
            ret(0)
        });
        slot(&mut e, 0x100, 1);
        // Squared distances: attacker to ally 50, target to ally 400, target to attacker 300.
        sequence_st0(&mut e, 0x0052_7a40, vec![50.0, 400.0, 300.0]);
        stub(&mut e, 0x008b_06d0, 0);
        stub(&mut e, 0x0099_2530, 1);
        stub_st0(&mut e, 0x008b_e1b0, 16.0);
        e.mem.set_u32(COMBAT_MANAGER, 0x6000);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0099_ebe0, &args![f.state, 0u32, out, 0u32]).bool());
        });
        assert_eq!(e.get(out, CombatLocation::pRef), ally);
        assert_eq!(
            calls(&log, 0x0081_9a50),
            vec![vec![out.addr(), 36.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, 0x0099_2530),
            vec![vec![0x6000, ally, f.attacker]]
        );
        // Hostile to the attacker: no ally; no door search (kind 10); a type 1 location
        // target with its world location.
        sequence_st0(&mut e, 0x0052_7a40, vec![50.0, 400.0, 300.0]);
        e.register(0x008b_06d0, |_, _| ret(1));
        let target = new_location_target(&mut e, f.state, 1);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(0x0099_ebe0, &args![f.state, 0u32, out, 10u32])
                .bool());
        });
        assert_eq!(calls(&log, 0x0097_ee60), vec![vec![out.addr(), target + 8]]);
        // No location target: the flags are set.
        e.set(f.state, CombatState::pLocationTarget, 0);
        e.set(out, CombatLocation::iFlags, 0);
        stub(&mut e, 0x004b_9ba0, 0);
        e.call(0x0099_ebe0, &args![f.state, 0u32, out, 10u32]);
        assert_eq!(e.get(out, CombatLocation::iFlags), 1);
    }

    #[test]
    fn find_best_flee_location_through_a_door() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let out: Ptr<CombatLocation> = e.new_object();
        let extra = e.mem.alloc(0x40);
        stub(&mut e, 0x0041_81e0, extra);
        stub(&mut e, 0x0055_e200, 1);
        put_setting(&mut e, 0x011c_ead4, 2.0);
        put_setting(&mut e, 0x011c_f234, 100.0);
        // The flee-door stamp is old enough (elapsed 10 > 2).
        e.mem.set_f32(f.state.addr() + 0xe0, -5.0);
        stub(&mut e, 0x006d_9350, 1);
        let door = object(&mut e, 0x100);
        let doors = Rc::new(RefCell::new(vec![door]));
        let size = doors.clone();
        let me = f.state.addr();
        e.register_double(ARRAY_SIZE, move |_, a| {
            let own = a[0] == me + 0x164 || a[0] == me + 0x174;
            ret(if own { 0 } else { size.borrow().len() as u32 })
        });
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, door);
        stub(&mut e, 0x006a_7ad0, cell);
        stub(&mut e, 0x0084_e3a0, 0x1234);
        stub(&mut e, 0x0056_8e50, 0x5000);
        stub(&mut e, 0x0040_8d60, 0x011f_1000);
        stub(&mut e, 0x0071_7e50, 0x5100);
        stub(&mut e, 0x0043_a320, 0x5200);
        stub_st0(&mut e, 0x004a_7290, 9.0);
        let position = e.mem.alloc(12);
        slot(&mut e, 0x1f4, position);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0099_ebe0, &args![f.state, 0u32, out, 0u32]).bool());
        });
        assert_eq!(
            calls(&log, 0x0098_9010),
            vec![vec![out.addr(), 0x5100, 0x5200]]
        );
        assert_eq!(calls(&log, 0x005e_04f0).len(), 1);
        // While not fleeing, `004dff00` renews the stamp first (then it is too new).
        stub(&mut e, 0x004d_ff00, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0099_ebe0, &args![f.state, 0u32, out, 0u32]);
        });
        assert_eq!(e.mem.f32(f.state.addr() + 0xe0), 5.0);
        assert!(calls(&log, 0x006d_9350).is_empty());
    }

    #[test]
    fn find_los_location_branches() {
        let mut e = engine();
        let f = fixture(&mut e, true);
        let out: Ptr<CombatLocation> = e.new_object();
        let member = object(&mut e, 0x200);
        e.set(out, CombatLocation::pRef, member);
        slot(&mut e, 0x100, 1);
        stub(&mut e, 0x0098_6a80, 1);
        stub(&mut e, 0x0088_c600, 1);
        // The current reference is a member who sees the target: kept.
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0099_f480, &args![f.state, out, 0u32]).bool());
        });
        assert_eq!(calls(&log, 0x0088_c600), vec![vec![member, f.target]]);
        assert!(calls(&log, 0x0099_0890).is_empty());
        // Otherwise the nearest member that sees the target.
        stub(&mut e, 0x0098_6a80, 0);
        stub(&mut e, 0x0099_0890, 2);
        let near = object(&mut e, 0x200);
        sequence(&mut e, 0x0090_3600, vec![f.attacker, near]);
        stub_st0(&mut e, 0x0052_7a40, 10.0);
        put_setting(&mut e, 0x011c_f144, 0.0);
        stub_st0(&mut e, 0x008b_e1b0, 1.0);
        assert!(e.call(0x0099_f480, &args![f.state, out, 0u32]).bool());
        assert_eq!(e.get(out, CombatLocation::pRef), near);
        // Nobody sees the target: false.
        stub(&mut e, 0x0088_c600, 0);
        sequence(&mut e, 0x0090_3600, vec![near]);
        assert!(!e.call(0x0099_f480, &args![f.state, out, 0u32]).bool());
    }

    #[test]
    fn fn_0099f6d0_records_a_better_weapon() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let process = object(&mut e, 0x40);
        stub(&mut e, 0x008d_8520, process);
        // Not allowed: nothing.
        assert!(!e.call(0x0099_f6d0, &args![f.state]).bool());
        stub(&mut e, 0x009a_0240, 1);
        e.set(f.state, CombatState::fMaxCombatWeaponDPS, 10.0);
        put_setting(&mut e, 0x011c_fb6c, 0.5);
        put_setting(&mut e, 0x011c_ee9c, 100.0);
        put_setting(&mut e, 0x011c_ec4c, 1000.0);
        put_setting(&mut e, 0x011c_e7fc, 2.0);
        e.mem.set_u32(LOADED_REFERENCES, 0x7000);
        // One loaded reference with a weapon of DPS 30 (threshold 15), no ammunition.
        let reference = object(&mut e, 0x100);
        let mut left = 1;
        e.register_double(0x0052_73c0, move |e, a| {
            if left == 0 {
                return ret(0);
            }
            left -= 1;
            e.mem.set_u32(a[2], reference);
            e.mem.set_u32(a[3], 0);
            ret(1)
        });
        stub_st0(&mut e, 0x0041_86f0, -1.0);
        stub(&mut e, 0x007a_f430, 0x5555);
        stub_st0(&mut e, 0x0064_6060, 30.0);
        stub(&mut e, 0x008b_c9d0, 1);
        stub_st0(&mut e, 0x0052_7a40, 400.0);
        e.register(0x0045_79e0, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).sqrt()))
        });
        e.register(0x0040_4010, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.register(0x0096_a2d0, |_, a| ret(a[0]));
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0099_f6d0, &args![f.state]).bool());
        });
        assert_eq!(
            calls(&log, 0x0098_1340),
            vec![vec![f.state.addr(), 3], vec![f.state.addr(), 4]]
        );
        let weapon = e.mem.u32(f.state.addr() + 0x190 + 16);
        assert_eq!(
            (e.mem.u32(weapon), e.mem.u32(weapon + 4)),
            (reference, 0x5555)
        );
        assert_eq!(e.mem.u32(f.state.addr() + 0x190 + 12), 0);
        assert_eq!(calls(&log, 0x0048_3710).len(), 2);
        assert_eq!(calls(&log, 0x0093_e510).len(), 1);
        // A weak weapon (below the threshold): nothing.
        let mut left = 1;
        e.register_double(0x0052_73c0, move |e, a| {
            if left == 0 {
                return ret(0);
            }
            left -= 1;
            e.mem.set_u32(a[2], reference);
            ret(1)
        });
        stub_st0(&mut e, 0x0064_6060, 15.0);
        assert!(!e.call(0x0099_f6d0, &args![f.state]).bool());
        // An embedded weapon: not searched.
        e.set(f.state, CombatState::pCurrentEmbeddedWeapon, 1);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0099_f6d0, &args![f.state]).bool());
        });
        assert!(calls(&log, 0x0052_71f0).is_empty());
    }

    #[test]
    fn fn_009a17a0_saves_the_state() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let me = f.state.addr();
        e.mem.set_f32(me + 0x108, -f32::MAX);
        e.mem.set_f32(me + 0x144, 3.0);
        e.register(0x006a_7f50, |e, a| st0(f64::from(e.mem.f32(a[0]))));
        let cover = e.mem.alloc(0x78);
        e.mem.set_u32(cover + 0x14, VTABLE);
        e.set(f.state, CombatState::pCoverLocation, cover);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, 0x11);
        e.mem.set_u32(record + 4, 0x22);
        e.mem.set_u32(me + 0x190 + 8, record);
        let flags = Rc::new(RefCell::new(vec![]));
        let seen = flags.clone();
        e.register_double(SAVE_DATA, move |e, a| {
            let own = (me..me + 0x22c).contains(&a[1]) || (cover..cover + 0x78).contains(&a[1]);
            if a[2] == 1 && !own {
                seen.borrow_mut().push(e.mem.u8(a[1]));
            }
            ret(0)
        });
        let log = logged(&mut e, |e| {
            e.call(0x009a_17a0, &args![f.state, 0x8000u32]);
        });
        // The optional parts: a cover location and a stamped secondary search; then
        // which combat items exist (item 2).
        assert_eq!(*flags.borrow(), vec![1 | 8, 4]);
        assert_eq!(calls(&log, SLOTS), vec![vec![cover + 0x14, 0x8000]]);
        let forms = calls(&log, SAVE_FORM_ID);
        assert!(forms.contains(&vec![0x8000, 0x11, 0]));
        assert!(forms.contains(&vec![0x8000, 0x22, 0]));
        assert_eq!(calls(&log, TIME_STAMP_SAVE).len(), 9);
        let timers: Vec<u32> = calls(&log, TIMER_SAVE).iter().map(|w| w[0] - me).collect();
        assert_eq!(
            timers,
            vec![
                0xa8, 0xb0, 0xb8, 0x1ac, 0x1b4, 0x1cc, 0x1d4, 0x1dc, 0x1e4, 0x1ec, 0x1f4, 0x1fc,
                0x20c, 0x204, 0x214, 0x21c
            ]
        );
        assert_eq!(calls(&log, SAVE_FORM_ID_PLAIN).len(), 1);
    }

    #[test]
    fn fn_009a2040_loads_the_state() {
        let mut e = engine();
        let f = fixture(&mut e, false);
        let me = f.state.addr();
        let buffer = object(&mut e, 0x40);
        slot(&mut e, 0x0, 0x14);
        e.register(0x0048_39c0, |_, a| ret(a[0]));
        e.register(0x00ec_43fb, |_, a| ret(a[3]));
        // The optional parts byte loads as 0x10 (a combat location), the items byte as 1.
        let mut bytes = 0u32;
        e.register_double(LOAD_DATA, move |e, a| {
            if a[2] == 1 && !(me..me + 0x22c).contains(&a[1]) {
                let value = if bytes % 2 == 0 { 0x10 } else { 1 };
                bytes += 1;
                e.mem.set_u8(a[1], value);
            } else if a[1] == me + 4 {
                e.mem.set_u32(a[1], u32::MAX);
            }
            ret(0)
        });
        let array = me + 0x24;
        e.mem.set_u32(array, VTABLE);
        stub(&mut e, SLOTS + 4, 0x4400);
        stub(&mut e, LOAD_VARIABLE_SIZE, 0);
        e.register(0x0096_a2d0, |_, a| ret(a[0]));
        let log = logged(&mut e, |e| {
            e.call(0x009a_2040, &args![f.state, buffer]);
        });
        assert_eq!(e.get(f.state, CombatState::iLocationUnreachable), 0);
        assert_eq!(e.mem.u32(me + 0xc), TES_OBJECT_WEAP_TYPE);
        assert_eq!(
            e.get(f.state, CombatState::pExplosiveProjectileBlocked),
            BGS_EXPLOSION_TYPE
        );
        let location = e.get(f.state, CombatState::pCombatLocation);
        assert_ne!(location, 0);
        assert_eq!(
            calls(&log, LOAD_FORM_ID)
                .iter()
                .filter(|w| w[1] == location + 0x10)
                .count(),
            1
        );
        let record = e.mem.u32(me + 0x190);
        assert_eq!(e.mem.u32(record + 4), TES_BOUND_OBJECT_TYPE);
        assert_eq!(e.mem.u32(me + 0x1a4), TES_BOUND_OBJECT_TYPE);
        // Version 0x14: the blocked timer and flee stamp are read, and the path check.
        assert_eq!(calls(&log, TIMER_LOAD).len(), 16);
        assert!(calls(&log, LOAD_DATA).iter().any(|w| w[1] == me + 0x228));
        assert_eq!(calls(&log, TIME_STAMP_LOAD).len(), 8);
        // An old buffer (version 0xe) skips them.
        slot(&mut e, 0x0, 0xe);
        let log = logged(&mut e, |e| {
            e.call(0x009a_2040, &args![f.state, buffer]);
        });
        assert_eq!(calls(&log, TIMER_LOAD).len(), 15);
        assert!(!calls(&log, LOAD_DATA).iter().any(|w| w[1] == me + 0x228));
        assert_eq!(calls(&log, TIME_STAMP_LOAD).len(), 7);
    }
}
