//! `fallout/interface/menus/vatsmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `VATSMenu` is the menu of V.A.T.S.: the target parts with their chance to
//! hit, the queue of actions, the action point bar and the special attack
//! buttons. `VATSMenu::pVatsMenu` (Xbox PDB static) is the open menu at
//! `011db0d4` on PC (`VATSMenu::Create` stores the menu there).
//!
//! The unit has 66 functions; they are translated in address order over
//! several sessions. The first part of the file has the layouts, the exe
//! addresses of the callees outside this unit and small helpers; a later
//! session adds what it needs there. Functions of this unit that are not
//! translated yet are called by address like any other callee.
//!
//! Notes that apply to every function:
//! - x87: the game computes in extended precision and stores `float`
//!   results. The translations compute in `f64` and round to `f32` at each
//!   `float` store; a `FLD`/`FSTP` copy of a float makes a signalling NaN
//!   quiet (`quiet_f32`). Floats pushed as arguments are passed as their
//!   bit pattern.
//! - Comparisons follow the flag tests of the code (`FCOMP`/`FNSTSW`/`TEST
//!   AH`), NaN included; each site says which test it is.
//! - The compiler's exception-unwinding frames (`FS:[0]`) and the security
//!   cookie are not translated; the functions that have them carry a
//!   `Translation incomplete` marker.

// `!(a < b)` mirrors the exe's x87 flag tests, which are true for NaN.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_interface::startmenu::ListBox;

// ---------------------------------------------------------------------------
// Layouts

layout! {
    /// `VATSMenu` (Xbox PDB) on PC: a `Menu` (0x28 bytes) followed by the
    /// fields this unit uses, at the offsets the PC code uses (they match
    /// the PDB's so far).
    pub struct VatsMenu: 0x144 {
        /// `Menu::pMenu` (Xbox PDB): the menu tile.
        0x04 pMenu: u32,
        /// `pTiles` (Xbox PDB): `Tile *[32]` indexed by `VATS_TILES`, the
        /// first one.
        0x28 pTiles: u32,
        /// `pActive` (Xbox PDB).
        0xa8 pActive: u32,
        /// `bDebounceMenuButton` (Xbox PDB).
        0xac bDebounceMenuButton: bool,
        /// `lbActions` (Xbox PDB): a `ListBox<unsigned int>`.
        0xb0 lbActions: u32,
        /// `fActionPoints` (Xbox PDB).
        0xe0 fActionPoints: f32,
        /// `fActionPointsBase` (Xbox PDB).
        0xe4 fActionPointsBase: f32,
        /// `fClipAmmo` (Xbox PDB).
        0xe8 fClipAmmo: f32,
        /// `fTotalAmmo` (Xbox PDB).
        0xec fTotalAmmo: f32,
        /// `fQueuedClipAmmo` (Xbox PDB).
        0xf0 fQueuedClipAmmo: f32,
        /// `fQueuedTotalAmmo` (Xbox PDB).
        0xf4 fQueuedTotalAmmo: f32,
        /// `bHeadingReached` (Xbox PDB).
        0xf8 bHeadingReached: bool,
        /// `bMetersShown` (Xbox PDB).
        0xf9 bMetersShown: bool,
        /// `bTargetsLit` (Xbox PDB).
        0xfa bTargetsLit: bool,
        /// `bClosing` (Xbox PDB).
        0xfb bClosing: bool,
        /// `fProbableEnemyDamage` (Xbox PDB).
        0xfc fProbableEnemyDamage: f32,
        /// `pCurrentLimb` (Xbox PDB): a `VATSMenu::ChanceToHit *`.
        0x100 pCurrentLimb: u32,
        /// `bConcentratedFirePerk` (Xbox PDB).
        0x104 bConcentratedFirePerk: bool,
        /// `pLastLimb` (Xbox PDB): a `VATSMenu::ChanceToHit *`.
        0x108 pLastLimb: u32,
        /// `bStickReset` (Xbox PDB).
        0x10c bStickReset: bool,
        /// `bTargetStickReset` (Xbox PDB).
        0x10d bTargetStickReset: bool,
        /// `bMouseOverButton` (Xbox PDB).
        0x10e bMouseOverButton: bool,
        /// `bSelectLeftHeld` (Xbox PDB).
        0x10f bSelectLeftHeld: bool,
        /// `bSelectRightHeld` (Xbox PDB).
        0x110 bSelectRightHeld: bool,
        /// `uiSelectTimer` (Xbox PDB).
        0x114 uiSelectTimer: u32,
        /// `bDataInTemporaryVATSCommand` (Xbox PDB).
        0x118 bDataInTemporaryVATSCommand: bool,
        /// `pTemporaryVATSCommand` (Xbox PDB): a `VATS_COMMAND` (0x28 bytes).
        0x11c pTemporaryVATSCommand: u32,
    }

    /// `VATSMenu::ChanceToHit` (Xbox PDB), 0x3C bytes: one target part.
    pub struct ChanceToHit: 0x3c {
        /// `Location` (Xbox PDB): an `NiPoint2`, x.
        0x00 Location: f32,
        /// `PlaneLocation` (Xbox PDB): an `NiPoint3`, x.
        0x08 PlaneLocation: f32,
        /// `WorldCoord` (Xbox PDB): an `NiPoint3`, x.
        0x14 WorldCoord: f32,
        /// `eLimb` (Xbox PDB): `BGSBodyPart::LIMB_ENUM`.
        0x20 eLimb: i32,
        /// `fScreenPercent` (Xbox PDB).
        0x24 fScreenPercent: f32,
        /// `fChance` (Xbox PDB).
        0x28 fChance: f32,
        /// `bOnScreen` (Xbox PDB).
        0x2c bOnScreen: bool,
        /// `bChanceCalculated` (Xbox PDB).
        0x2d bChanceCalculated: bool,
        /// `bFirstTimeShown` (Xbox PDB).
        0x2e bFirstTimeShown: bool,
        /// `bNeedsRecalc` (Xbox PDB).
        0x2f bNeedsRecalc: bool,
        /// `pMeter` (Xbox PDB): the meter tile.
        0x30 pMeter: u32,
        /// `fAngleToStick` (Xbox PDB).
        0x34 fAngleToStick: f32,
        /// `bLeftOfCenter` (Xbox PDB).
        0x38 bLeftOfCenter: bool,
    }

    /// `VATSMenu::VatsTarget` (Xbox PDB), 0x24 bytes: one target.
    pub struct VatsTarget: 0x24 {
        /// `pTarget` (Xbox PDB): the reference.
        0x00 pTarget: u32,
        /// `eTargetType` (Xbox PDB): `VTT_NONE` 0, `VTT_NPC` 1,
        /// `VTT_DESTRUCTABLE_OBJECT` 2.
        0x04 eTargetType: u32,
        /// `xTargetParts` (Xbox PDB): `BSSimpleList<ChanceToHit *>`, the
        /// first node's item.
        0x08 xTargetParts: u32,
        /// `bChancesCalculated` (Xbox PDB).
        0x10 bChancesCalculated: bool,
        /// `fDesiredTarget` (Xbox PDB).
        0x14 fDesiredTarget: f32,
        /// `fAngleToPlayer` (Xbox PDB).
        0x18 fAngleToPlayer: f32,
        /// `fDistanceToPlayer` (Xbox PDB).
        0x1c fDistanceToPlayer: f32,
        /// `bHostile` (Xbox PDB).
        0x20 bHostile: bool,
    }
}

// ---------------------------------------------------------------------------
// Globals of the unit

/// `VATSMenu::pVatsMenu` (Xbox PDB static): the open menu.
const VATS_MENU: u32 = 0x011d_b0d4;
/// a byte: the first special attack button is offered.
const SPECIAL_ONE_SHOWN: u32 = 0x011d_b0cf;
/// a byte: the second special attack button is offered.
const SPECIAL_TWO_SHOWN: u32 = 0x011d_b0d8;
/// the float the perk entry point `0x28` adjusts for the first special
/// attack (its action point cost: `fUpperCutAPCost` in the unarmed case).
const SPECIAL_ONE_COST: u32 = 0x011d_b0b8;
/// the float after it (the weapon's `+0x378` float, or 1.0 unarmed).
const SPECIAL_ONE_FACTOR: u32 = 0x011d_b0bc;
/// the float the entry point `0x28` adjusts for the second special attack
/// (`fCrossAPCost`).
const SPECIAL_TWO_COST: u32 = 0x011d_b0c0;
/// the float after it (1.0).
const SPECIAL_TWO_FACTOR: u32 = 0x011d_b0c4;

// ---------------------------------------------------------------------------
// Exe data read by the translations

/// the player (`PlayerCharacter *`).
const PLAYER: u32 = 0x011d_ea3c;
/// a reference the unit keeps (`TESObjectREFR *`, an actor where its slot
/// `0x100` answers true).
const TARGET_REFERENCE: u32 = 0x011f_21cc;
/// the form used when the special-attack check gets none.
const DEFAULT_FORM: u32 = 0x011c_a278;
/// the double 0.0 at `01012060`.
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// the pointer to the table of the fast square root (256 words).
const SQRT_TABLE: u32 = 0x011f_4aa8;
/// `fUpperCutThreshold` (setting, initializer `00f6a2d0`).
const SETTING_UPPERCUT_THRESHOLD: u32 = 0x011d_1380;
/// `fUpperCutAPCost` (setting, initializer `00f6a300`).
const SETTING_UPPERCUT_AP_COST: u32 = 0x011d_14f4;
/// `fCrossThreshold` (setting, initializer `00f6a390`).
const SETTING_CROSS_THRESHOLD: u32 = 0x011d_1500;
/// `fCrossAPCost` (setting, initializer `00f6a420`).
const SETTING_CROSS_AP_COST: u32 = 0x011d_12e4;
/// `sVatsUnarmedAttackGround` (setting, initializer `00f75bd0`).
const SETTING_UNARMED_GROUND: u32 = 0x011d_2754;
/// `sVatsUnarmedAttack1` (setting, initializer `00f75b70`).
const SETTING_UNARMED_ONE: u32 = 0x011d_46c0;
/// `sVatsUnarmedAttack2` (setting, initializer `00f75ba0`).
const SETTING_UNARMED_TWO: u32 = 0x011d_46d8;
/// the text `child(lb_scrollbar)`.
const TEXT_CHILD_LB_SCROLLBAR: u32 = 0x0107_05f8;

// ---------------------------------------------------------------------------
// Callees outside this unit (exe addresses)

/// `BGSEntryPoint::HandleEntryPoint` (engine map), cdecl, variable
/// arguments (entry point, actor, ...).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// thiscall on a setting: the address of its value (`+4`, or a zeroed
/// word at `01202800` for none).
const SETTING_VALUE_ADDRESS: u32 = 0x0040_3e20;
/// thiscall on a setting: the word at `+4` (its string), 0 for none.
const SETTING_STRING: u32 = 0x0040_3df0;
/// thiscall on a form: the word at `+0x368` (through `00559450`).
const FORM_WORD_368: u32 = 0x0052_2be0;
/// thiscall on a tile: (trait id, string, 1): sets a string trait.
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// thiscall on a tile: (trait id, integer): sets a trait to the integer as
/// a float (through `00a012d0`).
const TILE_SET_INT: u32 = 0x0070_0320;
/// thiscall on a form: the word at `+0x15c` (the engine map's name,
/// `MiddleHighProcess::GetLastBoundWeapon`, is a folded body).
const WORD_AT_15C: u32 = 0x008d_85e0;
/// thiscall on a form: the float at `+0x378` in ST0 (folded body named
/// `HighProcess::GetPackageEndTimerValue` in the engine map).
const FLOAT_AT_378: u32 = 0x008d_9140;
/// thiscall on a reference: whether the word `004f8960` answers is 3.
const STATE_IS_THREE: u32 = 0x0043_7bd0;
/// cdecl (tile, path): the tile a path names from it.
const TILE_FROM_PATH: u32 = 0x00a0_8b20;
/// thiscall on the audio object: (sound type, flag) (`BSAudio`).
const AUDIO_PAUSE_TYPE: u32 = 0x00ad_8510;
/// thiscall on the audio object: (sound type, flag) (`BSAudio`).
const AUDIO_RESUME_TYPE: u32 = 0x00ad_8540;

/// thiscall on a menu: the word at `+0x24` (the folded body the engine map
/// names `D3DTexture_LockRect`).
const MENU_FADE_STATE: u32 = 0x0059_bb30;
/// `BSAudio::QInstance` (engine map).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `Tile::GetMenuByClass` (engine map), cdecl (class): the menu tile.
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `VATS::SetMode` (engine map): (mode, 0).
const VATS_SET_MODE: u32 = 0x009c_6c30;
/// `VATS::GetCount` (engine map).
const VATS_GET_COUNT: u32 = 0x005a_e380;
/// thiscall on the VATS object: (command): queues a `VATS_COMMAND`.
const VATS_QUEUE_COMMAND: u32 = 0x009c_71a0;
/// thiscall on a `VATS_COMMAND` block: constructs it (through `009ca500`),
/// returns it.
const VATS_COMMAND_CONSTRUCT: u32 = 0x009c_a4a0;
/// thiscall on a `BSSimpleList`: whether it is empty.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// thiscall on a `BSSimpleList`: removes every node.
const LIST_CLEAR: u32 = 0x0047_0470;
/// returns an object (`004b7210`).
const OBJECT_004B7210: u32 = 0x004b_7210;
/// thiscall on `004b7210`'s object.
const UPDATE_0070BC20: u32 = 0x0070_bc20;
/// `BSAudio::EndAllSoundsOfType` (engine map): (type).
const END_ALL_SOUNDS_OF_TYPE: u32 = 0x00ad_8780;
/// thiscall: whether bit 2 of the word at `+0x244` is set.
const FLAG_244_BIT_2: u32 = 0x0042_ce10;
/// `BSAudioManager::QInstance` (engine map).
const AUDIO_MANAGER_INSTANCE: u32 = 0x00ad_9060;
/// `BSAudioManager::GetSoundHandleByName` (engine map): (out, name, flags).
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ae_5680;
/// `BSAudio::GetSoundHandleByName` (engine map): (out, name, flags).
const GET_SOUND_HANDLE_BY_NAME_ON_AUDIO: u32 = 0x00ad_7550;
/// constructor of a `BSSoundHandle` (0xC bytes): id -1, flags cleared.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
/// `BSSoundHandle` assignment: (source).
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
/// an empty member function (the sound handle's destructor).
const EMPTY_MEMBER: u32 = 0x0048_3710;
/// `BSSoundHandle::Play` (engine map): (flag).
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// `Menu::StartFadeOut` (engine map).
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// `BGSDefaultObjectManager::GetDefaultObject` (engine map), cdecl (index).
const GET_DEFAULT_OBJECT: u32 = 0x0058_db10;
/// `Actor::IsInFaction` (engine map): (faction).
const ACTOR_IS_IN_FACTION: u32 = 0x008b_8e90;
/// `Actor::GetEssential` (engine map).
const ACTOR_GET_ESSENTIAL: u32 = 0x0087_f3d0;
/// `RandomFloat` (engine map), cdecl (low, high) as float bits; ST0.
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// the allocator, cdecl (size).
const ALLOCATE: u32 = 0x0040_1000;
/// returns an object (`00705910`, through `004b7210`) or 0.
const OBJECT_00705910: u32 = 0x0070_5910;
/// thiscall on `00705910`'s object: three words.
const SET_00800AC0: u32 = 0x0080_0ac0;
/// thiscall on `00705910`'s object: one word.
const SET_00800D10: u32 = 0x0080_0d10;
/// thiscall on `00705910`'s object.
const UPDATE_00800ED0: u32 = 0x0080_0ed0;

/// thiscall: the word at `+8`.
const WORD_AT_8: u32 = 0x0044_ddc0;
/// thiscall: the word at `+0x68` (folded body named
/// `MiddleHighProcess::GetSavedAcquireObject` in the engine map).
const WORD_AT_68: u32 = 0x008d_8520;
/// thiscall: returns `this` (a list node's accessor).
const IDENTITY: u32 = 0x0068_15c0;
/// thiscall on a list node: the word at `+4` (the next node).
const NEXT_NODE: u32 = 0x0072_6070;
/// thiscall on a form: a check (`006450c0`).
const CHECK_006450C0: u32 = 0x0064_50c0;
/// `Interface::IsTopMenuID` (engine map), cdecl (menu id).
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `Interface::IsConsoleVisible` (engine map).
const IS_CONSOLE_VISIBLE: u32 = 0x0070_3d50;
/// `Interface::CloseVATSMenu` (engine map).
const CLOSE_VATS_MENU: u32 = 0x0070_5780;
/// `Tile::RemoveFadeControl` (engine map): (flags).
const REMOVE_FADE_CONTROL: u32 = 0x00a0_7dc0;
/// `Tile::IsVisible` (engine map).
const TILE_IS_VISIBLE: u32 = 0x00a0_40a0;
/// `Tile::AddUserTrait` (engine map), cdecl (name, id): the trait id.
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// thiscall on a list head: the address of the first node's data (`*this
/// + 8`).
const FIRST_CHILD_DATA: u32 = 0x0073_ac90;
/// `VATSMenu::BuildMetersForTarget` (engine map; this unit, not translated
/// yet): (target).
const BUILD_METERS_FOR_TARGET: u32 = 0x007f_6e50;
/// thiscall on the menu: a check of this unit not translated yet
/// (`007f6af0`).
const VATS_MENU_007F6AF0: u32 = 0x007f_6af0;
/// thiscall on the holder at `011dea0c`: the controls object.
const CONTROLS: u32 = 0x0087_7720;
/// thiscall on the controls: (0, axis): an axis value.
const AXIS_VALUE: u32 = 0x00a2_3390;
/// thiscall on the controls: (control): a packed key word.
const CONTROL_KEY: u32 = 0x00a2_4f10;
/// thiscall on the controls: (control, state).
const CONTROL_STATE: u32 = 0x00a2_4660;
/// thiscall on the controls: (control, state).
const CONTROL_PRESSED: u32 = 0x00a2_4180;
/// thiscall on the controls: (button, state).
const BUTTON_STATE: u32 = 0x00a2_3a50;
/// `InterfaceManager::CheckMenuButton` (engine map), cdecl (button, 1).
const CHECK_MENU_BUTTON: u32 = 0x0071_7a40;
/// whether the gamepad interface is in use (`004b71d0`).
const GAMEPAD_IN_USE: u32 = 0x004b_71d0;
/// a check (`00709d40`).
const OBJECT_00709D40: u32 = 0x0070_9d40;
/// thiscall on the timer object: the word at `+0x14` (milliseconds).
const TIMER_MILLISECONDS: u32 = 0x0082_5c00;
/// thiscall on the timer object: the float at `+0xC` in ST0.
const TIMER_FLOAT_C: u32 = 0x0084_d030;
/// cdecl (value): the absolute value.
const ABS: u32 = 0x00ec_7d40;
/// cdecl (a, b): the larger integer (`b` unless `a` is above it).
const INT_MAX: u32 = 0x0064_7b70;
/// cdecl (a, b): `b` when `a <= b`, else `a`, as floats; ST0.
const FLOAT_MAX: u32 = 0x0040_4010;
/// `_ftol2`: truncates ST0 (passed as an `f64`).
const FTOL: u32 = 0x00ec_62c0;
/// `sprintf` (engine map), cdecl, variable arguments.
const SPRINTF: u32 = 0x00ec_623a;
/// `HUDMainMenu::UpdateMeter_ov2` (engine map), cdecl (meter, ghost,
/// ghost value, value, two flags).
const HUD_UPDATE_METER_GHOST: u32 = 0x0077_4b00;
/// `HUDMainMenu::UpdateMeter` (engine map), cdecl (meter, value, 0, 0, 0).
const HUD_UPDATE_METER: u32 = 0x0077_48b0;
/// `HUDMainMenu::UpdateCompass` (engine map), cdecl (four tiles).
const HUD_UPDATE_COMPASS: u32 = 0x0077_9070;
/// thiscall on the HUD menu: (text, 2, 0, 0, time, 0): shows a message.
const HUD_SHOW_MESSAGE: u32 = 0x0077_5380;
/// thiscall on a reference: its full name (through `00891170` and
/// `TESFullName::GetFullName`).
const REFERENCE_NAME: u32 = 0x0055_d520;
/// cdecl (player, reference): a float (printed after
/// `sInventoryDamageThreshold`).
const DAMAGE_THRESHOLD: u32 = 0x0077_00f0;
/// `PlayerCharacter::IsActorPerceived` (engine map): (actor): an entry
/// whose `+4` byte is tested.
const IS_ACTOR_PERCEIVED: u32 = 0x0096_7090;
/// thiscall on the player: (frame float, 0, 0, 0).
const PLAYER_UPDATE_945B0: u32 = 0x0094_45b0;
/// thiscall on the player: (frame float).
const CHARACTER_UPDATE_8D3550: u32 = 0x008d_3550;
/// `Actor::UpdateAnimationMovement` (engine map): (value, float).
const UPDATE_ANIMATION_MOVEMENT: u32 = 0x0088_85e0;
/// `PlayerCharacter::UpdateCamera` (engine map): (0, 0).
const UPDATE_CAMERA: u32 = 0x0094_ae40;
/// `PlayerCharacter::UpdateFirstPersonZoom` (engine map).
const UPDATE_FIRST_PERSON_ZOOM: u32 = 0x0095_0290;
/// `PlayerCharacter::UpdateAutoAimActor` (engine map).
const UPDATE_AUTO_AIM_ACTOR: u32 = 0x0096_4260;
/// `PlayerCharacter::IsGodMode` (engine map).
const IS_GOD_MODE: u32 = 0x0095_26b0;
/// thiscall on `004b7210`'s object (`00747d00`, result not used).
const OBJECT_00747D00: u32 = 0x0074_7d00;
/// thiscall on `004b7210`'s object (`0073b020`, result not used).
const OBJECT_0073B020: u32 = 0x0073_b020;
/// `InterfaceManager::GetScreenWidth` (engine map): ST0.
const SCREEN_WIDTH: u32 = 0x0071_5d40;
/// an `InterfaceManager` size in ST0 (`00715da0`).
const SIZE_00715DA0: u32 = 0x0071_5da0;
/// an `InterfaceManager` size in ST0 (`007177c0`).
const SIZE_007177C0: u32 = 0x0071_77c0;
/// an `InterfaceManager` size in ST0 (`00717820`).
const SIZE_00717820: u32 = 0x0071_7820;
/// `TESObjectWEAP::GetCurrentAmmo` (engine
/// map): (actor).
const GET_CURRENT_AMMO: u32 = 0x0052_5980;
/// `TESObjectWEAP::GetFormClipRounds` (engine map): (0).
const GET_FORM_CLIP_ROUNDS: u32 = 0x004f_e160;
/// thiscall on a weapon: the byte at `+0x102`.
const WEAPON_102_524B60: u32 = 0x0052_4b60;
/// thiscall on a weapon: the signed byte at `+0xF4` (its kind).
const WEAPON_KIND: u32 = 0x0044_6390;
/// `BGSAmmoForm::IsRockItLauncher` (engine map), on the weapon's `+0xA4`.
const IS_ROCK_IT_LAUNCHER: u32 = 0x0047_4a80;
/// `ActionPoints::GetAttackAction` (engine map), cdecl (weapon kind).
const GET_ATTACK_ACTION: u32 = 0x0066_dba0;
/// `ActionPoints::GetWeaponAttackActionPoints` (engine map), cdecl (weapon,
/// 1, 0): ST0.
const GET_WEAPON_ATTACK_ACTION_POINTS: u32 = 0x0066_dce0;
/// cdecl (action, 1, float, 0): an action point cost in ST0.
const ACTION_POINTS_COST: u32 = 0x0066_dca0;
/// `VATS::IsMeleeCommand` (engine map): (command).
const VATS_IS_MELEE_COMMAND: u32 = 0x009c_9f60;
/// thiscall on a reference: a form (through `BGSSaveFormBuffer::GetForm`,
/// engine map name of a folded body).
const BASE_FORM_OF: u32 = 0x0041_81e0;
/// `BGSBodyPartData::GetBodyPart_ov2` (engine map): (index).
const GET_BODY_PART: u32 = 0x005e_5130;
/// thiscall on body part data: its part array (15 words).
const BODY_PART_ARRAY: u32 = 0x006a_9540;
/// thiscall on a body part: the byte at `+0x63`.
const BODY_PART_VALUE: u32 = 0x005e_5190;
/// thiscall on an integer setting: the address of its value.
const SETTING_INT_ADDRESS: u32 = 0x0043_d4d0;
/// thiscall on a bool setting: the address of its value.
const SETTING_BOOL_ADDRESS: u32 = 0x0040_8d60;
/// cdecl (chance as float bits): the roll's result in `AL`.
const ROLL_CHANCE: u32 = 0x004d_ff00;

/// thiscall on `004b7210`'s object: the mouse wheel value.
const WHEEL_VALUE: u32 = 0x0070_ec70;
/// thiscall on the VATS object: a word (`005d8710`).
const VATS_005D8710: u32 = 0x005d_8710;
/// thiscall on the VATS object: (reference) (`009c9ae0`).
const VATS_009C9AE0: u32 = 0x009c_9ae0;
/// thiscall on the VATS object: its last command, or 0.
const VATS_LAST_COMMAND: u32 = 0x009c_71f0;
/// thiscall on the VATS object: (action out, flag out): drops the last
/// command.
const VATS_REMOVE_LAST_COMMAND: u32 = 0x009c_88a0;
/// `ListBox<unsigned_int>::GetLastItem` (engine map; this unit, not
/// translated yet).
const LIST_BOX_GET_LAST_ITEM: u32 = 0x007f_6fb0;
/// `ListBox<unsigned_int>::AddItem` (engine map; this unit, not translated
/// yet): (id, 0, 0, 0): the item's tile.
const LIST_BOX_ADD_ITEM: u32 = 0x007f_6fe0;
/// thiscall on the list box (this unit, not translated yet): (index, 0).
const LIST_BOX_007F7280: u32 = 0x007f_7280;
/// thiscall on a list: its count (`VATS::GetCount` on it).
const LIST_COUNT: u32 = 0x0072_6c40;
/// `Tile::GetString` (engine map): (trait id).
const TILE_GET_STRING: u32 = 0x00a0_11f0;
/// cdecl (action): the action's name.
const ACTION_NAME: u32 = 0x0066_ddc0;
/// `ActionPoints::IsAttackAction` (engine map), cdecl (action).
const IS_ATTACK_ACTION: u32 = 0x0066_dde0;
/// cdecl (a, b): `strcmp`.
const STRCMP: u32 = 0x0040_8b20;
/// cdecl (buffer, size, text): `strcpy_s`.
const STRCPY_S: u32 = 0x0040_6d30;
/// cdecl (buffer, size, text): `strcat_s`.
const STRCAT_S: u32 = 0x0040_6d50;
/// thiscall on a body part: its name, or 0.
const BODY_PART_NAME: u32 = 0x0068_38b0;
/// thiscall on an entry: an object (`00413f40`).
const ENTRY_OBJECT_413F40: u32 = 0x0041_3f40;
/// thiscall on that object: a check (`0043b1b0`).
const OBJECT_CHECK_43B1B0: u32 = 0x0043_b1b0;

/// `HUDMainMenu::HeadingBetweenLocations` (engine map), cdecl (from, to):
/// ST0.
const HEADING_BETWEEN_LOCATIONS: u32 = 0x0077_2840;
/// cdecl (value as float bits): its absolute value as a float (`00408860`,
/// `_fabs`); ST0.
const FLOAT_ABS: u32 = 0x0040_8840;
/// returns the object at `011deb7c` (through `00559450`).
const CAMERA_HOLDER: u32 = 0x0045_c670;
/// thiscall: the word at `+0` (the folded body the engine map names
/// `BSFaceGenNiNode::GetAnimationData`): the camera.
const CAMERA_OF: u32 = 0x0066_29f0;
/// `NiCamera::WorldPtToScreenPt` (engine map): (point, x out, y out,
/// epsilon).
const WORLD_PT_TO_SCREEN_PT: u32 = 0x00a6_fc50;

/// the table of setting pointers `007f0360` indexes.
const SETTING_TABLE: u32 = 0x0118_3aa0;
/// the double `57.2957763671875` (degrees per radian) at `0102f248`.
const DOUBLE_DEGREES: u32 = 0x0102_f248;
/// the double 180.0 at `0104ed58`.
const DOUBLE_180: u32 = 0x0104_ed58;
/// the double 360.0 at `0103c348`.
const DOUBLE_360: u32 = 0x0103_c348;
/// the double 1.0 at `01012070`.
const DOUBLE_ONE: u32 = 0x0101_2070;
/// the float epsilon passed to `WorldPtToScreenPt` (`010718c0`).
const SCREEN_EPSILON: u32 = 0x0107_18c0;

/// thiscall on a tile: (trait id): the trait's float in ST0.
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// thiscall on a tile: (trait id, float, 1): sets a float trait.
const SET_FLOAT_TRAIT: u32 = 0x00a0_12d0;
/// `Tile::AddFadeControl` (engine map): (trait id, from, to, time, flag).
const ADD_FADE_CONTROL: u32 = 0x00a0_7c60;
/// `Tile::GetFadeEndFor` (engine map): (trait id): ST0.
const GET_FADE_END_FOR: u32 = 0x00a0_7f30;
/// `Tile::GetFadeFinished` (engine map): (trait id).
const GET_FADE_FINISHED: u32 = 0x00a0_7fc0;
/// `Tile::GetFadeType` (engine map): (trait id).
const GET_FADE_TYPE: u32 = 0x00a0_8070;

/// the guard bits of `UpdateChanceToHit`'s function-local statics.
const STATIC_TRAIT_GUARD: u32 = 0x011d_b1ac;
/// `UpdateChanceToHit` static: the user trait `_MeterPercent`.
const STATIC_METER_PERCENT: u32 = 0x011d_b1a8;
/// `UpdateChanceToHit` static: the user trait `_LimbName`.
const STATIC_LIMB_NAME: u32 = 0x011d_b1a4;
/// `UpdateChanceToHit` static: the user trait `_LimbNameVisible`.
const STATIC_LIMB_NAME_VISIBLE: u32 = 0x011d_b1a0;
/// `UpdateChanceToHit` static: the user trait `_Brightness`.
const STATIC_BRIGHTNESS_TRAIT_2: u32 = 0x011d_b19c;
/// a byte: the chances are being recalculated (`011db0ce`).
const FLAG_011DB0CE: u32 = 0x011d_b0ce;
/// the timer's milliseconds when the recalculation mode started.
const RECALCULATION_TIME: u32 = 0x011d_b0dc;
/// the text `_MeterPercent`.
const TEXT_METER_PERCENT: u32 = 0x0107_7df4;
/// the text `_LimbName`.
const TEXT_LIMB_NAME: u32 = 0x0107_7de8;
/// the text `_LimbNameVisible`.
const TEXT_LIMB_NAME_VISIBLE: u32 = 0x0107_7dd4;
/// the float 255.0 at `01023cd8`.
const FLOAT_255: u32 = 0x0102_3cd8;
/// the float 0.2 at `0101df44`.
const FLOAT_POINT_TWO: u32 = 0x0101_df44;
/// the float 0.5 at `01016248`.
const FLOAT_HALF: u32 = 0x0101_6248;
/// the double 255.0 at `0101e568`.
const DOUBLE_255: u32 = 0x0101_e568;

/// cdecl (buffer, size, format, value): `sprintf_s` with one value.
const SPRINTF_S: u32 = 0x0040_6d00;
/// cdecl (a, b): the smaller integer (`a` when below `b`).
const INT_MIN: u32 = 0x004a_8f20;
/// `Actor::GetCurrentWeapon` (engine map).
const GET_CURRENT_WEAPON: u32 = 0x008a_1710;
/// the text `%i%%`.
const TEXT_PERCENT_FORMAT: u32 = 0x0107_7674;

/// `ItemChange::GetItemHealth` (engine map): (1): ST0.
const GET_ITEM_HEALTH: u32 = 0x004b_cdb0;
/// `BGSBodyPartData::GetBodyPart` (engine map): (limb).
const GET_BODY_PART_BY_LIMB: u32 = 0x005e_50f0;
/// `TESBoundObject::GetBoundSize` (engine map): ST0.
const GET_BOUND_SIZE: u32 = 0x0050_ebf0;
/// `TESObjectREFR::GetModelBoundSize` (engine map): ST0.
const GET_MODEL_BOUND_SIZE: u32 = 0x0057_1600;
/// thiscall on the player: (3): a float in ST0 (`008b0dd0`).
const PLAYER_FACTOR_8B0DD0: u32 = 0x008b_0dd0;
/// `CombatUtilities::GetDistanceBetweenFacingActors` (engine map), cdecl
/// (actor, actor, flag): ST0.
const GET_DISTANCE_BETWEEN_FACING_ACTORS: u32 = 0x009a_64d0;
/// `CombatUtilities::GetWeaponReach` (engine map), cdecl (actor, weapon):
/// ST0.
const GET_WEAPON_REACH: u32 = 0x009a_69c0;
/// `CombatFormulas::GetVATSChanceToHit` (engine map), cdecl, twelve words:
/// ST0.
const GET_VATS_CHANCE_TO_HIT: u32 = 0x0064_6f70;
/// `PathingLocation::PathingLocation_ov3` (engine map): (reference).
const PATHING_LOCATION_CONSTRUCT: u32 = 0x006d_cd70;
/// thiscall on a `PathingLocation`: its destructor (`004ff7e0`).
const PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
/// cdecl (from, to, 0, length out, 0): whether a path was found.
const PATH_LENGTH: u32 = 0x006d_5490;
/// thiscall on a point: (out, other): `this - other` into `out`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// thiscall on a point: its length in ST0.
const POINT_LENGTH: u32 = 0x0045_7990;
/// thiscall on a `VATS_COMMAND`: initializes it (`009ca500`).
const VATS_COMMAND_INITIALIZE: u32 = 0x009c_a500;
/// thiscall: (mask, set): sets or clears `mask` in the word at `+0x58`.
const FLAGS_SET_00407E00: u32 = 0x0040_7e00;
/// `HitData::InitializeHitData` (engine map), cdecl (hit data, command).
const INITIALIZE_HIT_DATA: u32 = 0x009b_5050;
/// the float 128.0 at `0101e704`.
const FLOAT_128: u32 = 0x0101_e704;
/// the float `FLT_MAX` at `01016970`.
const FLOAT_LARGEST: u32 = 0x0101_6970;
/// the double 2.0 at `01011590`.
const DOUBLE_TWO: u32 = 0x0101_1590;

/// cdecl (a, b): `b` when `b <= a` or unordered, else `a`, as floats; ST0.
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// thiscall on a body part: its first word (the node name).
const BODY_PART_NODE_NAME: u32 = 0x0043_b230;
/// cdecl (root, name): the node of that name (through `00c4b310`).
const FIND_NODE: u32 = 0x004a_ae30;
/// thiscall on a scene object: the address of its world position (`+0x8C`).
const OBJECT_WORLD_POSITION: u32 = 0x0045_bb80;
/// thiscall on a 3x3 matrix: (out, point): `matrix * point` into `out`.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// thiscall on a reference: its health in ST0 (`00568ad0`).
const REFERENCE_HEALTH: u32 = 0x0056_8ad0;
/// thiscall on a form's full name (`+0x30`): the text (folded body named
/// `MapMarkerData::GetLocationName` in the engine map).
const FORM_FULL_NAME: u32 = 0x0040_8da0;
/// `TESHealthForm::GetFormHealth` (engine map), cdecl (form).
const GET_FORM_HEALTH: u32 = 0x0048_73d0;
/// thiscall on an `NiPoint2`: (out, other): `this - other`.
const POINT2_SUBTRACT: u32 = 0x004e_8880;
/// thiscall on an `NiPoint2`: normalises it, its former length in ST0.
const POINT2_NORMALIZE: u32 = 0x004e_7560;
/// thiscall on an `NiPoint2`: its length in ST0.
const POINT2_LENGTH: u32 = 0x0058_9850;
/// thiscall on an `NiPoint2`: (out, factor): `this * factor`.
const POINT2_SCALE: u32 = 0x004a_4f40;
/// thiscall on an `NiPoint2`: (out, other): `this + other`.
const POINT2_ADD: u32 = 0x004d_fe30;
/// thiscall on an `NiPoint2`: (other): whether both components are equal.
const POINT2_EQUALS: u32 = 0x0045_2df0;

/// the guard bits of `007f1840`'s function-local statics.
const LAYOUT_GUARD: u32 = 0x011d_b1c8;
/// `007f1840` static: half the screen width.
const LAYOUT_HALF_WIDTH: u32 = 0x011d_b1c4;
/// `007f1840` static: half the screen height.
const LAYOUT_HALF_HEIGHT: u32 = 0x011d_b1c0;
/// `007f1840` static: the left bound of the meters.
const LAYOUT_LEFT: u32 = 0x011d_b1bc;
/// `007f1840` static: the top bound of the meters.
const LAYOUT_TOP: u32 = 0x011d_b1b8;
/// `007f1840` static: the right bound of the meters.
const LAYOUT_RIGHT: u32 = 0x011d_b1b4;
/// `007f1840` static: the bottom bound of the meters.
const LAYOUT_BOTTOM: u32 = 0x011d_b1b0;
/// the double 50.0 at `0101e2c0`.
const DOUBLE_FIFTY: u32 = 0x0101_e2c0;
/// the float 150.0 at `010231d0`.
const FLOAT_150: u32 = 0x0102_31d0;

/// `PlayerCharacter::GetNumRockItAmmo` (engine map).
const GET_NUM_ROCK_IT_AMMO: u32 = 0x0096_9040;
/// `InventoryChanges::GetInventoryChanges` (engine map), cdecl (actor).
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::GetObjectCount` (engine map): (form).
const GET_OBJECT_COUNT: u32 = 0x004c_8f30;
/// `ItemChange::HasModEffectActive_ov2` (engine map): (effect): `AL`.
const HAS_MOD_EFFECT: u32 = 0x004b_da70;
/// `TESObjectWEAP::GetAmmoRegenRate` (engine map): (flag): ST0.
const GET_AMMO_REGEN_RATE: u32 = 0x0070_9430;
/// the text `%i/%i`.
const TEXT_AMMO_FORMAT: u32 = 0x0107_14ec;

/// thiscall on the camera: the address of its rotation (`+0x68`).
const CAMERA_ROTATION: u32 = 0x0046_1130;
/// thiscall on an `NiPoint3`: (x, y, z) as float bits: sets it.
const POINT3_SET: u32 = 0x0041_6870;
/// thiscall on an `NiPoint3`: normalises it.
const POINT3_NORMALIZE: u32 = 0x004a_0c10;
/// `NiPoint3::Dot` (engine map): (other): ST0.
const POINT3_DOT: u32 = 0x004b_6190;
/// cdecl (float bits): a float in ST0 (`004f6e40`, which Ghidra's library
/// matching names `_logf`).
const FLOAT_FUNCTION_4F6E40: u32 = 0x004f_6e40;
/// `Tile::IsTrue` (engine map): (trait id).
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// the float 180.0 at `0106b158`.
const FLOAT_180: u32 = 0x0106_b158;
/// the double 45.0 at `010357e8`.
const DOUBLE_45: u32 = 0x0103_57e8;

/// thiscall on `00705910`'s object: (flag address).
const SCENE_0044AC20: u32 = 0x0044_ac20;
/// thiscall on `00705910`'s object: (reference).
const SET_00800D50: u32 = 0x0080_0d50;
/// thiscall on `00705910`'s object: (reference).
const SET_00800E50: u32 = 0x0080_0e50;
/// thiscall on `00705910`'s object.
const SET_00800700: u32 = 0x0080_0700;
/// thiscall on `00705910`'s object.
const SET_008009D0: u32 = 0x0080_09d0;
/// thiscall on `00705910`'s object.
const SET_00800A90: u32 = 0x0080_0a90;
/// thiscall on `00705910`'s object: (limb, float out): whether it answered.
const SCREEN_PERCENT_00800A00: u32 = 0x0080_0a00;
/// thiscall on an `NiPoint3`: (other): whether the points are equal.
const POINT3_EQUALS: u32 = 0x0043_90c0;
/// cdecl, variable arguments (text): prints a message (`005b5e40`).
const PRINT_005B5E40: u32 = 0x005b_5e40;
/// `Interface::EmergencyCloseAllMenusAndBreakStuff` (engine map).
const EMERGENCY_CLOSE_ALL_MENUS: u32 = 0x0070_37e0;
/// `Menu::RenderTemplate` (engine map): (parent tile, template name, 0).
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// the zero point at `011f426c`.
const POINT_ZERO: u32 = 0x011f_426c;
/// the double 300.0 at `01077e08`.
const DOUBLE_300: u32 = 0x0107_7e08;
/// the target item the limb was last picked for (`011db1cc`).
const LAST_SCANNED_TARGET: u32 = 0x011d_b1cc;
/// the text `MENUS: VATS Menu Creation Failed...`.
const TEXT_MENU_CREATION_FAILED: u32 = 0x0107_7e10;

/// cdecl (command): the command's damage in ST0 (`009b4fe0`).
const COMMAND_DAMAGE: u32 = 0x009b_4fe0;
/// `Actor::DifficultyLevelAdjustHealthModifier` (engine map): (value,
/// attacker): ST0.
const DIFFICULTY_ADJUST_HEALTH_MODIFIER: u32 = 0x0088_08a0;
/// thiscall on a `VATS_COMMAND`: its destructor (`009ca4c0`).
const VATS_COMMAND_DESTRUCT: u32 = 0x009c_a4c0;

/// thiscall on a weapon: whether bit 2 of its `+0x100` byte is set.
const WEAPON_FLAG_100_BIT_2: u32 = 0x0052_4b40;
/// thiscall on a weapon: a float in ST0 (`00821640`).
const WEAPON_VALUE_821640: u32 = 0x0082_1640;
/// `TESObjectWEAP::GetAttackSpeed` (engine map): (flag): ST0.
const GET_ATTACK_SPEED: u32 = 0x0064_6020;
/// thiscall on the camera: (out): its direction (`0045bba0`).
const CAMERA_DIRECTION: u32 = 0x0045_bba0;
/// thiscall on a 3x3 matrix: (row, three floats address): sets the row.
const MATRIX_SET_ROW: u32 = 0x005d_f510;
/// `fVATSShotLongBurstTime` (setting, initializer `00f60510`).
const SETTING_SHOT_LONG_BURST_TIME: u32 = 0x011c_f108;
/// `fVATSShotBurstTime` (setting, initializer `00f604e0`).
const SETTING_SHOT_BURST_TIME: u32 = 0x011c_f63c;

/// `NiCamera::WorldToCameraMatrix` (engine map).
const WORLD_TO_CAMERA_MATRIX: u32 = 0x00a7_0ba0;
/// thiscall on the player: the head node of a list of entries (a
/// reference first, a byte at `+4`) (`00595d10`).
const PLAYER_TARGET_LIST: u32 = 0x0059_5d10;
/// thiscall on an actor: its cell (`008d6f30`).
const PARENT_CELL_8D6F30: u32 = 0x008d_6f30;
/// thiscall on a cell: a flag (`00425fd0`).
const CELL_FLAG_425FD0: u32 = 0x0042_5fd0;
/// thiscall on a cell: the object whose slot `0xC8` casts (`004543c0`).
const CELL_WORLD_4543C0: u32 = 0x0045_43c0;
/// thiscall on a reference: its base form (the folded body the engine map
/// names `BGSSaveFormBuffer::GetForm`).
const FORM_OF_REFERENCE: u32 = 0x007a_f430;
/// `BGSDestructibleObjectForm::GetDestructionForm` (engine map), cdecl
/// (form).
const GET_DESTRUCTION_FORM: u32 = 0x0047_5400;
/// `BGSDestructibleObjectForm::IsDestructible` (engine map), cdecl (form).
const IS_DESTRUCTIBLE: u32 = 0x0047_53d0;
/// thiscall on a reference: a check (`00576070`).
const IS_DESTRUCTIBLE_576070: u32 = 0x0057_6070;
/// `MobileObject::GetCurrentProcessType` (engine map).
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `TESObjectREFR::GetDistanceFromReference` (engine map): (reference, 0,
/// 0): ST0.
const GET_DISTANCE_FROM_REFERENCE: u32 = 0x0057_23b0;
/// `Actor::LineOfSight` (engine map): (1, reference, 1, 0, 1).
const LINE_OF_SIGHT: u32 = 0x0088_b880;
/// thiscall on a weapon: a check (`0046e8c0`).
const WEAPON_CHECK_46E8C0: u32 = 0x0046_e8c0;
/// `TESObjectREFR::GetWorldSpace` (engine map).
const REFERENCE_GET_WORLDSPACE: u32 = 0x0057_5d70;
/// thiscall on a worldspace: (position, radius, position, radius,
/// callback, actor): calls the callback for the references around.
const WORLDSPACE_GATHER: u32 = 0x0058_85f0;
/// thiscall on a cell: the same as `WORLDSPACE_GATHER` (`0054da20`).
const CELL_GATHER: u32 = 0x0054_da20;
/// the callback passed to the gather calls (`007f4f30`, this unit).
const GATHER_CALLBACK: u32 = 0x007f_4f30;
/// thiscall on a reference: a check (`00477ba0`).
const CHECK_477BA0: u32 = 0x0047_7ba0;
/// thiscall on a reference: a check (`00440da0`).
const CHECK_440DA0: u32 = 0x0044_0da0;
/// thiscall on a reference: a check (`00440d80`).
const CHECK_440D80: u32 = 0x0044_0d80;
/// thiscall on a reference: an actor compared with the player
/// (`00874480`).
const OWNER_874480: u32 = 0x0087_4480;
/// thiscall on a list: (address of the pointer to add) (`005ae3d0`).
const LIST_APPEND: u32 = 0x005a_e3d0;
/// thiscall: constructs the query object of the casts (`004a3c20`).
const QUERY_CONSTRUCT_4A3C20: u32 = 0x004a_3c20;
/// the one-word store translated in `fallout_misc/bgschangeflags.rs`.
const STORE_8C71B0: u32 = 0x008c_71b0;
/// thiscall on the filter word: (`0x25`) (`004a39f0`).
const FILTER_4A39F0: u32 = 0x004a_39f0;
/// thiscall on the player: (out): an object (`00931ed0`).
const COLLISION_OBJECT_931ED0: u32 = 0x0093_1ed0;
/// thiscall on that object: a word (`004a3a20`).
const GROUP_4A3A20: u32 = 0x004a_3a20;
/// thiscall on the filter word: (word) (`0059ce80`).
const FILTER_59CE80: u32 = 0x0059_ce80;
/// thiscall on the query: (filter word) (`004a3f70`).
const QUERY_4A3F70: u32 = 0x004a_3f70;
/// thiscall on the query: (collector) (`004a3fb0`).
const QUERY_4A3FB0: u32 = 0x004a_3fb0;
/// thiscall on the query: (point) (`004a3da0`).
const QUERY_4A3DA0: u32 = 0x004a_3da0;
/// thiscall on the query: (point) (`004a3eb0`).
const QUERY_4A3EB0: u32 = 0x004a_3eb0;
/// thiscall on the query: its results (`008cdd90`), 0 for none.
const QUERY_8CDD90: u32 = 0x008c_dd90;
/// thiscall on the results: their array (`00460140`).
const ARRAY_460140: u32 = 0x0046_0140;
/// thiscall on the array: (index): the entry (`004a46b0`).
const ARRAY_AT_4A46B0: u32 = 0x004a_46b0;
/// thiscall on a local: (entry): copies the entry into it (`00696c30`).
const COPY_696C30: u32 = 0x0069_6c30;
/// `hkpAllRayHitCollector::hkpAllRayHitCollector` (engine map).
const RAY_COLLECTOR_CONSTRUCT: u32 = 0x004a_3a70;
/// `hkpAllRayHitCollector::~hkpAllRayHitCollector` (engine map).
const RAY_COLLECTOR_DESTRUCT: u32 = 0x004a_3bc0;
/// `GetAVObjectForCollidable` (engine map), cdecl (collidable).
const GET_AV_OBJECT_FOR_COLLIDABLE: u32 = 0x004b_5820;
/// `TESObjectREFR::FindReferenceFor3D` (engine map), cdecl (object).
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `NiAVObject::GetWorldBound` (engine map).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
/// thiscall on an `NiPoint3`: (out, factor): `this * factor`.
const POINT3_SCALE: u32 = 0x0045_bb20;
/// `fVATSMaxEngageDistance` (setting, initializer `00f75ab0`).
const SETTING_VATS_MAX_ENGAGE_DISTANCE: u32 = 0x011d_45b8;
/// `fSneakMaxDistance` (setting, initializer `00f59450`).
const SETTING_SNEAK_MAX_DISTANCE: u32 = 0x011c_d7d8;
/// `fSneakExteriorDistanceMult` (setting, initializer `00f59780`).
const SETTING_SNEAK_EXTERIOR_MULT: u32 = 0x011c_dcbc;
/// the double 128.0 at `0102e430`.
const DOUBLE_128: u32 = 0x0102_e430;

/// the deallocator, cdecl (pointer).
const FREE: u32 = 0x0040_1030;
/// thiscall on a reference: a check (`00452370`).
const REFERENCE_CHECK_452370: u32 = 0x0045_2370;
/// thiscall on a reference: an object whose kind (`+0xF4`) is tested
/// (`00476c90`).
const REFERENCE_OBJECT_476C90: u32 = 0x0047_6c90;
/// cdecl (out, point): converts the point into `out`; returns `out`
/// (`004a3e00`).
const CONVERT_4A3E00: u32 = 0x004a_3e00;
/// thiscall on the query: (converted point) (`004a3f90`).
const QUERY_4A3F90: u32 = 0x004a_3f90;
/// thiscall on a list: its constructor (`0096a2d0`).
const LIST_CONSTRUCT_96A2D0: u32 = 0x0096_a2d0;
/// thiscall on a list: its destructor (`0046ffb0`).
const LIST_DESTRUCT_46FFB0: u32 = 0x0046_ffb0;
/// thiscall on a list, run after its first item is freed (`0063f7b0`).
const LIST_0063F7B0: u32 = 0x0063_f7b0;
/// `fVATSMeleeMaxDistance` (setting, initializer `00f60780`).
const SETTING_VATS_MELEE_MAX_DISTANCE: u32 = 0x011c_f678;

/// `Tile::GetMenu` (engine map).
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::GetMaximumDepth` (engine map): ST0.
const GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// thiscall on `004b7210`'s object: an object for `Tile::ReadFile`
/// (`00586150`).
const MENU_PATH_586150: u32 = 0x0058_6150;
/// `Tile::ReadFile` (engine map): (path): the tile.
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::UpdateAll` (engine map), cdecl (0).
const TILE_UPDATE_ALL: u32 = 0x00a0_4200;
/// `Tile::GetChildByName` (engine map): (name).
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// `Tile::TextToTrait` (engine map), cdecl (name).
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Menu::SetMenuTile` (engine map): (tile, 0).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// `Menu::PrepForVisibility` (engine map): (0).
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// `__RTDynamicCast` (engine map), cdecl (object, 0, from, to, 0).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// the RTTI type descriptor of `Tile` (`.?AVTile@@`).
const TYPE_TILE: u32 = 0x0119_f4cc;
/// the RTTI type descriptor of `TileMenu` (`.?AVTileMenu@@`).
const TYPE_TILE_MENU: u32 = 0x0119_f49c;
/// the RTTI type descriptor of `Menu` (`.?AVMenu@@`).
const TYPE_MENU: u32 = 0x0119_f34c;
/// the RTTI type descriptor of `VATSMenu` (`.?AVVATSMenu@@`).
const TYPE_VATS_MENU: u32 = 0x0119_f3e8;
/// cdecl (form id): the form (`004839c0`).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// thiscall on a form: an object for `005f65d0` (`00500940`).
const FORM_LIST_500940: u32 = 0x0050_0940;
/// thiscall on that object: (address of a form): whether it holds it.
const LIST_CONTAINS_5F65D0: u32 = 0x005f_65d0;
/// thiscall on a reference: a level compared with 3 and 4 (`00472380`).
const REFERENCE_472380: u32 = 0x0047_2380;
/// thiscall on an actor: a check (`005a2030`).
const ACTOR_CHECK_5A2030: u32 = 0x005a_2030;
/// thiscall on a list (or its node): (item): removes it (`00905330`).
const LIST_REMOVE_905330: u32 = 0x0090_5330;
/// thiscall on a list: (compare function): sorts it (`0083fd60`).
const LIST_SORT: u32 = 0x0083_fd60;
/// `007f0380` of this unit (the facing comparator).
const FN_ORDER_BY_FACING: u32 = 0x007f_0380;
/// `007f0500` of this unit (the heading comparator).
const FN_ORDER_BY_HEADING: u32 = 0x007f_0500;
/// `PlayerCharacter::SetFirstPerson` (engine map): (flag).
const SET_FIRST_PERSON: u32 = 0x0095_0110;
/// `Actor::SetIronSights` (engine map): (0, 0, 0).
const SET_IRON_SIGHTS: u32 = 0x008b_b650;
/// `HighProcess::SkipFadeIn` (engine map): (actor).
const SKIP_FADE_IN: u32 = 0x008f_f030;
/// `Actor::SetAlphaMult` (engine map): (float).
const SET_ALPHA_MULT: u32 = 0x008c_4890;
/// thiscall on an actor: (1) (`008a6840`).
const ACTOR_8A6840: u32 = 0x008a_6840;
/// thiscall on a scene object: the address of a position (`0043c490`).
const NODE_POSITION_43C490: u32 = 0x0043_c490;
/// thiscall on a 3x3 matrix: (column, 1): an entry in ST0 (`004b55c0`).
const MATRIX_ENTRY_4B55C0: u32 = 0x004b_55c0;
/// thiscall on an `NiPoint3`: (factor): scales it (`00439180`).
const POINT3_SCALE_IN_PLACE: u32 = 0x0043_9180;
/// thiscall on an `NiPoint3`: (other): adds it (`004578c0`).
const POINT3_ADD_IN_PLACE: u32 = 0x0045_78c0;
/// `TES::GetWorldSpace` (engine map).
const TES_GET_WORLDSPACE: u32 = 0x004f_d3e0;
/// thiscall on the TES object: a cell (the folded body the engine map names
/// `ActorMover::GetPreferredMoveMode`).
const TES_CELL_5F36F0: u32 = 0x005f_36f0;
/// thiscall on `*011c3f2c`: (form, position, rotation, cell, worldspace,
/// 0, 0, 0): a new reference (`004698a0`).
const PLACE_REFERENCE_4698A0: u32 = 0x0046_98a0;
/// `HUDMainMenu::SetBrightness` (engine map), cdecl (tile, value).
const HUD_SET_BRIGHTNESS: u32 = 0x0077_4800;
/// thiscall: constructs an `NiPointer` (0) (`00633c90`).
const NI_POINTER_CONSTRUCT_633C90: u32 = 0x0063_3c90;
/// thiscall on an `NiPointer`: (object): assigns it (`0066b0d0`).
const NI_POINTER_ASSIGN_66B0D0: u32 = 0x0066_b0d0;
/// thiscall on an `NiPointer`: its destructor (`0045cec0`).
const NI_POINTER_DESTRUCT_45CEC0: u32 = 0x0045_cec0;
/// thiscall: the first word (`00559450`).
const POINTER_GET_559450: u32 = 0x0055_9450;
/// `NiFixedString` constructor from a C string (this, text); returns this.
const FIXED_STRING_FROM: u32 = 0x0043_8170;
/// `NiFixedString` destructor.
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// `NiSourceTexture::Create` (engine map), cdecl (name, prefs, 1, 0).
const SOURCE_TEXTURE_CREATE: u32 = 0x00a5_fd70;
/// thiscall on the compass window's node: (texture) (`007700b0`).
const COMPASS_SET_TEXTURE_7700B0: u32 = 0x0077_00b0;
/// thiscall on the compass window's node: (1) (`007700d0`).
const COMPASS_SET_7700D0: u32 = 0x0077_00d0;
/// `Error` (engine map), cdecl (text).
const ERROR: u32 = 0x0040_fbe0;
/// thiscall: constructs a string (`004037b0`).
const STRING_CONSTRUCT_4037B0: u32 = 0x0040_37b0;
/// thiscall: destroys that string (`004037d0`).
const STRING_DESTRUCT_4037D0: u32 = 0x0040_37d0;
/// `Calendar::GetHour` (engine map): ST0.
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// `InterfaceManager::TutorialManager::ShowMessage` (engine map): (message,
/// menu, flags).
const TUTORIAL_SHOW_MESSAGE: u32 = 0x0071_8630;

/// the calendar (`011de7b8`).
const CALENDAR: u32 = 0x011d_e7b8;
/// the TES object (`TES::GetWorldSpace` runs on it).
const TES: u32 = 0x011d_ea10;
/// the object `004698a0` runs on (`011c3f2c`).
const OBJECT_011C3F2C: u32 = 0x011c_3f2c;
/// the texture preferences passed to `NiSourceTexture::Create`.
const GLOBAL_011A9598: u32 = 0x011a_9598;
/// the cached default object 7 (the stranger).
const STRANGER_FORM: u32 = 0x011d_b0ac;
/// the cached default object `0x16` (the fortune companion).
const FORTUNE_FORM: u32 = 0x011d_b0b4;
/// a byte cleared when the menu opens (`011db0b1`).
const FLAG_011DB0B1: u32 = 0x011d_b0b1;
/// a byte cleared when the menu opens (`011db0cc`).
const FLAG_011DB0CC: u32 = 0x011d_b0cc;
/// a byte: back to first person when no target is left (`011f21d1`).
const FLAG_011F21D1: u32 = 0x011f_21d1;
/// the float distance of the companion from the player (`0104fca0`).
const FLOAT_COMPANION_DISTANCE: u32 = 0x0104_fca0;
/// the double 0.5 at `01011588`.
const DOUBLE_HALF: u32 = 0x0101_1588;
/// the small double at `0101e788` (the divisor for no distance).
const DOUBLE_SMALL: u32 = 0x0101_e788;
/// the double 40.0 at `01035810`.
const DOUBLE_40: u32 = 0x0103_5810;
/// the double 30.0 at `0101db88`.
const DOUBLE_30: u32 = 0x0101_db88;
/// the double 3.0 at `01021928`.
const DOUBLE_THREE: u32 = 0x0102_1928;
/// the double 5.0 at `01020998`.
const DOUBLE_FIVE: u32 = 0x0102_0998;
/// the double 6006.0 at `010704f0`.
const DOUBLE_6006: u32 = 0x0107_04f0;
/// the double 102.0 at `010704e8`.
const DOUBLE_102: u32 = 0x0107_04e8;
/// the double 60.0 at `01012638`.
const DOUBLE_60: u32 = 0x0101_2638;
/// the double 12.0 at `01035710`.
const DOUBLE_12: u32 = 0x0103_5710;
/// `iVatsTargetAngle` (setting, initializer `00f653e0`).
const SETTING_VATS_TARGET_ANGLE: u32 = 0x011c_fec4;
/// `iHUDMaxCompassNPCTicks` (setting, initializer `00f6ed60`).
const SETTING_HUD_MAX_COMPASS_NPC_TICKS: u32 = 0x011d_32d0;
/// `sActionPointsShort` (setting, initializer `00f6ea60`).
const SETTING_ACTION_POINTS_SHORT: u32 = 0x011d_3e8c;
/// `sVATSMessageLowAP` (setting, initializer `00f75ae0`).
const SETTING_MESSAGE_LOW_AP: u32 = 0x011d_1d90;
/// `sInventoryCondition` (setting, initializer `00f6f570`).
const SETTING_INVENTORY_CONDITION: u32 = 0x011d_4da4;
/// `sHitPointsShort` (setting, initializer `00f6ea30`).
const SETTING_HIT_POINTS_SHORT: u32 = 0x011d_300c;
/// `sLanguage:General` (setting, initializer `00f388a0`).
const SETTING_LANGUAGE: u32 = 0x011c_3cb4;
/// `sVatsSelect` (setting, initializer `00f759c0`).
const SETTING_VATS_SELECT: u32 = 0x011d_5080;
/// `sAccept` (setting, initializer `00f6cde0`).
const SETTING_ACCEPT: u32 = 0x011d_26f4;
/// `sReturn` (setting, initializer `00f6d050`).
const SETTING_RETURN: u32 = 0x011d_21c0;
/// `sVatsBodyPart` (setting, initializer `00f75960`).
const SETTING_VATS_BODY_PART: u32 = 0x011d_2b74;
/// `sVatsTarget` (setting, initializer `00f75990`).
const SETTING_VATS_TARGET: u32 = 0x011d_4d38;
/// the text `Data\Menus\VATS_menu.xml`.
const TEXT_VATS_MENU_XML: u32 = 0x0107_7d50;
/// the text `_OriginalX`.
const TEXT_ORIGINAL_X: u32 = 0x0107_3430;
/// the text `_TotalWidth`.
const TEXT_TOTAL_WIDTH: u32 = 0x0107_3424;
/// the text `_ImageWidth`.
const TEXT_IMAGE_WIDTH: u32 = 0x0107_3418;
/// the text `ActionPoints`.
const TEXT_ACTION_POINTS: u32 = 0x0106_7858;
/// the text `HitPoints`.
const TEXT_HIT_POINTS: u32 = 0x0107_338c;
/// the text `EnemyHealth`.
const TEXT_ENEMY_HEALTH: u32 = 0x0107_3260;
/// the text `template_right_bracket`.
const TEXT_TEMPLATE_RIGHT_BRACKET: u32 = 0x0107_3400;
/// the text `template_left_bracket`.
const TEXT_TEMPLATE_LEFT_BRACKET: u32 = 0x0107_3374;
/// the text `template_meter`.
const TEXT_TEMPLATE_METER: u32 = 0x0107_33f0;
/// the text `template_meter_background`.
const TEXT_TEMPLATE_METER_BACKGROUND: u32 = 0x0107_3398;
/// the text `template_justify_right_text`.
const TEXT_TEMPLATE_RIGHT_TEXT: u32 = 0x0107_33d0;
/// the text `template_justify_center_text`.
const TEXT_TEMPLATE_CENTER_TEXT: u32 = 0x0107_3220;
/// the text `template_compass_window`.
const TEXT_TEMPLATE_COMPASS_WINDOW: u32 = 0x0107_335c;
/// the text `template_compass_icon_group`.
const TEXT_TEMPLATE_COMPASS_ICON_GROUP: u32 = 0x0107_3340;
/// the text `compass_npc_icon`.
const TEXT_COMPASS_NPC_ICON: u32 = 0x0107_332c;
/// the text `template_compass_icon_quest`.
const TEXT_TEMPLATE_COMPASS_ICON_QUEST: u32 = 0x0107_3310;
/// the text `template_compass_icon_player`.
const TEXT_TEMPLATE_COMPASS_ICON_PLAYER: u32 = 0x0107_32f0;
/// the text `template_compass_icon_marker`.
const TEXT_TEMPLATE_COMPASS_ICON_MARKER: u32 = 0x0107_32d0;
/// the text `template_warning_background`.
const TEXT_TEMPLATE_WARNING_BACKGROUND: u32 = 0x0107_7d34;
/// the text `template_enemy_health_bracket`.
const TEXT_TEMPLATE_ENEMY_HEALTH_BRACKET: u32 = 0x0107_3240;
/// the text `Interface\VATS\vats_bar.dds`.
const TEXT_VATS_BAR: u32 = 0x0107_33b4;
/// the text `Textures\Interface\HUD\hud_compass_alphamap.dds`.
const TEXT_COMPASS_ALPHA_MAP: u32 = 0x0107_2e40;
/// the text `VATS Menu Creation Failed... This likely means t...`.
const TEXT_CREATION_FAILED_TILES: u32 = 0x0107_7cc0;
/// the text `queued_action`.
const TEXT_QUEUED_ACTION: u32 = 0x0107_7cb0;
/// the text `UIVATSEnter`.
const TEXT_UI_VATS_ENTER: u32 = 0x0107_7ca4;
/// the text `ENGLISH`.
const TEXT_ENGLISH: u32 = 0x0101_8348;
/// the text `Target_button`.
const TEXT_TARGET_BUTTON: u32 = 0x0107_7c94;
/// the text `BodyPart_button`.
const TEXT_BODY_PART_BUTTON: u32 = 0x0107_7c84;
/// the text `&xbuttonls;`.
const TEXT_XBUTTON_LS: u32 = 0x0107_7c78;
/// the text `&xbuttonrs;`.
const TEXT_XBUTTON_RS: u32 = 0x0107_7c6c;
/// the text `_xbox_button`.
const TEXT_XBOX_BUTTON: u32 = 0x0107_34d0;

/// the id of the next action list item.
const ACTION_ITEM_COUNTER: u32 = 0x011d_b198;
/// the text `UIMenuCancel`.
const TEXT_UI_MENU_CANCEL: u32 = 0x0106_f360;
/// the text `UIVATSMove`.
const TEXT_UI_VATS_MOVE: u32 = 0x0107_7dc8;
/// the text `UIVATSSelectTarget`.
const TEXT_UI_VATS_SELECT_TARGET: u32 = 0x0107_7dac;
/// the text `: `.
const TEXT_COLON: u32 = 0x0107_7dc4;
/// the text `---`.
const TEXT_DASHES: u32 = 0x0107_7dc0;

/// the holder of the controls object.
const CONTROLS_HOLDER: u32 = 0x011d_ea0c;
/// the timer object (`011f6394`).
const TIMER: u32 = 0x011f_6394;
/// the HUD menu (`HUDMainMenu *`).
const HUD_MAIN_MENU: u32 = 0x011d_96c0;
/// a byte: the target's limb has been shown (`DoIdle`).
const LIMB_SHOWN: u32 = 0x011a_0b68;
/// the last limb shown (`ShowTargetLimb`).
const LAST_LIMB: u32 = 0x011a_0b70;
/// the reference the last limb was shown on (`ShowTargetLimb`).
const LAST_REFERENCE: u32 = 0x011d_b0d0;
/// a word the menu clears (`011dea34`).
const GLOBAL_011DEA34: u32 = 0x011d_ea34;
/// the index passed to `GetBodyPart` before an attack (`011a59f0`).
const GLOBAL_011A59F0: u32 = 0x011a_59f0;
/// the special attack whose control is held (0 to 3).
const QUEUED_SPECIAL: u32 = 0x011d_b180;
/// a byte cleared every frame (`011db0b2`).
const FLAG_011DB0B2: u32 = 0x011d_b0b2;
/// a byte cleared every frame (`011db0b3`).
const FLAG_011DB0B3: u32 = 0x011d_b0b3;
/// a byte: paralyzing palm attacks are possible (`011db0b0`).
const FLAG_011DB0B0: u32 = 0x011d_b0b0;
/// the guard bits of `DoIdle`'s function-local statics.
const STATIC_GUARD: u32 = 0x011d_b194;
/// `DoIdle` static: the screen width, truncated.
const STATIC_SCREEN_WIDTH: u32 = 0x011d_b190;
/// `DoIdle` static: `00715da0`, truncated.
const STATIC_SIZE_00715DA0: u32 = 0x011d_b18c;
/// `DoIdle` static: `007177c0`, truncated.
const STATIC_SIZE_007177C0: u32 = 0x011d_b188;
/// `DoIdle` static: `00717820`, truncated.
const STATIC_SIZE_00717820: u32 = 0x011d_b184;
/// `DoIdle` static: the user trait `_Brightness`.
const STATIC_BRIGHTNESS_TRAIT: u32 = 0x011d_b17c;
/// the float 2.0 at `010162c0`.
const FLOAT_TWO: u32 = 0x0101_62c0;
/// the float 100.0 at `01016410`.
const FLOAT_HUNDRED: u32 = 0x0101_6410;
/// the double 100.0 at `01017a40`.
const DOUBLE_HUNDRED: u32 = 0x0101_7a40;
/// the text `%s\n %s:%d %s:%.2f`.
const TEXT_NAME_FORMAT: u32 = 0x0107_3460;
/// the text `_Brightness`.
const TEXT_BRIGHTNESS: u32 = 0x0107_34b8;
/// the text `UIVATSInsufficientAP`.
const TEXT_UI_VATS_INSUFFICIENT_AP: u32 = 0x0107_3a94;
/// `sInventoryDamageThreshold` (setting, initializer `00f6f540`).
const SETTING_DAMAGE_THRESHOLD_TEXT: u32 = 0x011d_396c;
/// `sStatsHP` (setting, initializer `00f75030`).
const SETTING_STATS_HP: u32 = 0x011d_50b0;
/// `sVATSMessageZeroChance` (setting, initializer `00f75b10`).
const SETTING_MESSAGE_ZERO_CHANCE: u32 = 0x011d_3894;
/// `sVATSMessageNoAmmo` (setting, initializer `00f75b40`).
const SETTING_MESSAGE_NO_AMMO: u32 = 0x011d_28c8;
/// `fVATSParalyzePalmChance` (setting, initializer `00f608d0`).
const SETTING_PARALYZE_PALM_CHANCE: u32 = 0x011c_e8dc;
/// `iVATSConcentratedFireBonus` (setting, initializer `00f759f0`).
const SETTING_CONCENTRATED_FIRE_BONUS: u32 = 0x011d_4210;
/// `bVatsAlwaysHit:Combat` (setting, initializer `00f94020`).
const SETTING_VATS_ALWAYS_HIT: u32 = 0x011d_b114;

/// the VATS object (`VATS`, its methods `VATS::SetMode`, `VATS::GetCount`).
const VATS: u32 = 0x011f_2250;
/// a byte the menu clears (also cleared with VATS mode 0).
const FLAG_011A3B31: u32 = 0x011a_3b31;
/// a byte of the unit, cleared when the menu closes.
const FLAG_011DB0CD: u32 = 0x011d_b0cd;
/// a `BSSimpleList` of the unit (cleared when the menu closes).
const LIST_011DB124: u32 = 0x011d_b124;
/// a `BSSimpleList` of the unit (its emptiness picks the closing sound).
const LIST_011DB150: u32 = 0x011d_b150;
/// the save/load game object (`011ddf38`).
const SAVE_LOAD_GAME: u32 = 0x011d_df38;
/// the actor of the first perk strike (stranger: `bStranger`).
const STRANGER_ACTOR: u32 = 0x011f_21e8;
/// the actor of the second perk strike (`bMissFortune`).
const FORTUNE_ACTOR: u32 = 0x011f_21ec;
/// `fVATSFortuneOdds` (setting, initializer `00f60840`).
const SETTING_FORTUNE_ODDS: u32 = 0x011c_e32c;
/// `fVATSStrangerOdds` (setting, initializer `00f60810`).
const SETTING_STRANGER_ODDS: u32 = 0x011c_e738;
/// the text `UIVATSEnterFail`.
const TEXT_UI_VATS_ENTER_FAIL: u32 = 0x0107_7d84;
/// the text `UIVATSExit`.
const TEXT_UI_VATS_EXIT: u32 = 0x0107_7d78;
/// the text `UIVATSReady`.
const TEXT_UI_VATS_READY: u32 = 0x0107_7d6c;
/// the text `UIVATSSelectTargetPart`.
const TEXT_UI_VATS_SELECT_TARGET_PART: u32 = 0x0107_7d94;

/// `bhkCollisionFilter::SetMultiLayerCollision` (engine map), cdecl
/// (layer, value, 0).
const SET_MULTI_LAYER_COLLISION: u32 = 0x00c8_2870;
/// thiscall on a camera: (first, second, origin, direction, 0), five words
/// (`RET 0x14`), the two integers from `00792760`/`009a9350` (`00a71080`,
/// unnamed in the engine map).
const CAMERA_RAY_A71080: u32 = 0x00a7_1080;
/// thiscall on `004b7210`'s object: the float at `+0x40` in ST0
/// (`00792760`).
const FLOAT_AT_40: u32 = 0x0079_2760;
/// thiscall on `004b7210`'s object: the float at `+0x38` in ST0
/// (`009a9350`).
const FLOAT_AT_38: u32 = 0x009a_9350;
/// the float the cast direction is scaled by (`011a0b74`).
const FLOAT_CAST_LENGTH: u32 = 0x011a_0b74;
/// `ListBox<unsigned_int>::NumUnfiltered` (engine map): (0).
const LIST_BOX_NUM_UNFILTERED: u32 = 0x0072_6b40;
/// `Tile::GetValue` (engine map): (trait).
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// `Tile::GetValueQ` (engine map): (trait).
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
/// `Tile::Value::ClearActions` (engine map).
const TILE_VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
/// `Tile::Value::AddAction_ov2` (engine map): (action, tile, trait).
const TILE_VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
/// `Tile::Value::AddAction` (engine map): (action, float).
const TILE_VALUE_ADD_ACTION_FLOAT: u32 = 0x00a0_9080;
/// `Tile::Value::CalculateValue` (engine map): (0).
const TILE_VALUE_CALCULATE: u32 = 0x00a0_9410;
/// thiscall: constructs a list box item (tile, address of the data);
/// returns it (`007a62e0`).
const LIST_ITEM_CONSTRUCT: u32 = 0x007a_62e0;
/// thiscall on the list box's list: (item, position): inserts the item
/// (`007a7eb0`, named `BSSimpleList<ActiveEffect_P>::Insert` by the engine
/// map on a folded body).
const LIST_INSERT: u32 = 0x007a_7eb0;
/// thiscall on the list box's list: (address of the item): adds it
/// (`00905820`).
const LIST_ADD_TAIL: u32 = 0x0090_5820;
/// thiscall on the list box: (0): a flag test (`007242c0`).
const LIST_BOX_FLAG_TEST: u32 = 0x0072_42c0;
/// `ListBox<unsigned_int>::Synchronize` (engine map).
const LIST_BOX_SYNCHRONIZE: u32 = 0x0072_6090;
/// `ListBox<StartMenu::Option_P>::InitTilePosition` (engine map name on a
/// shared body): (tile).
const LIST_BOX_INIT_TILE_POSITION: u32 = 0x0072_69d0;
/// `ListBox<unsigned_int>::SetTotalListHeight` (engine map).
const LIST_BOX_SET_TOTAL_LIST_HEIGHT: u32 = 0x0072_6530;
/// thiscall on the list box: (node, node before it): removes the item
/// (`007f7380`, not in this unit's queue); answers a bool.
const LIST_BOX_REMOVE_7F7380: u32 = 0x007f_7380;
/// the text `_number_of_visible_items`.
const TEXT_NUMBER_OF_VISIBLE_ITEMS: u32 = 0x0106_f860;
/// the text `body_part_percent`.
const TEXT_BODY_PART_PERCENT: u32 = 0x0107_7e98;
/// the text `GlowBranch`.
const TEXT_GLOW_BRANCH: u32 = 0x0107_7eac;

// ---------------------------------------------------------------------------
// Helpers

/// `value` as `FLD`/`FSTP` copies it: a signalling NaN becomes quiet.
fn quiet_f32(value: f32) -> f32 {
    let bits = value.to_bits();
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        f32::from_bits(bits | 0x0040_0000)
    } else {
        value
    }
}

/// The float at `address` as `FLD` loads it.
fn load_f32(e: &Engine, address: u32) -> f32 {
    quiet_f32(e.mem.f32(address))
}

/// `pTiles[index]` of the menu at `menu`.
fn tile(e: &Engine, menu: u32, index: u32) -> u32 {
    e.mem.u32(menu + 0x28 + 4 * index)
}

/// The open menu (`pVatsMenu`).
fn vats_menu(e: &Engine) -> u32 {
    e.global::<u32>(VATS_MENU)
}

/// The integer the player's sub-object at `+0xA4` answers in its slot
/// `0x08` for `index`.
fn player_value_08(e: &mut Engine, index: u32) -> i32 {
    let owner = e.global::<u32>(PLAYER) + 0xa4;
    e.vcall(owner, 0x08, &args![index]).i32()
}

// ---------------------------------------------------------------------------
// Functions

// Translated from 0075a0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned_int>::Initialize` (Xbox PDB): stores the container tile
/// and the template name and finds the scroll bar tile
/// (`child(lb_scrollbar)` of the container).
pub fn list_box_unsigned_int_initialize(
    e: &mut Engine,
    this: Ptr<ListBox>,
    container: u32,
    template_name: u32,
) {
    e.set(this, ListBox::pContainerRect, Ptr::new(container));
    e.set(this, ListBox::pTemplateName, Ptr::new(template_name));
    let container = e.get(this, ListBox::pContainerRect).addr();
    let scroll_bar = e
        .call(TILE_FROM_PATH, &args![container, TEXT_CHILD_LB_SCROLLBAR])
        .u32();
    e.set(this, ListBox::pScrollBar, Ptr::new(scroll_bar));
}

// Translated from 007eb7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (point): the length of a 3D vector through the fast square root:
/// the squared length (`z*z + y*y + x*x` stored as a float) is 0 when its
/// bits are 0, otherwise the exponent is halved and the mantissa's top
/// byte (with bit 23 for an odd exponent) indexes the table at
/// `*011f4aa8`.
pub fn fn_007eb7f0(e: &mut Engine, point: u32) -> f32 {
    let x = e.mem.f32(point) as f64;
    let y = e.mem.f32(point + 4) as f64;
    let z = e.mem.f32(point + 8) as f64;
    let squared = (x * x + y * y + z * z) as f32;
    let bits = squared.to_bits();
    if bits == 0 {
        return 0.0;
    }
    let mut exponent = (bits >> 23).wrapping_sub(0x7f) as u16;
    let mut mantissa = bits & 0x007f_ffff;
    if (exponent as i16 as i32) & 1 != 0 {
        mantissa |= 0x0080_0000;
    }
    exponent = ((exponent as i16) >> 1) as u16;
    let table = e.global::<u32>(SQRT_TABLE);
    let entry = e.mem.u32(table.wrapping_add((mantissa >> 16) * 4));
    let result = ((exponent as i16 as i32 + 0x7f) as u32) << 23 | entry;
    quiet_f32(f32::from_bits(result))
}

// Translated from 007eb8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the audio object (`BSAudio`): `00ad8510(0x3002, 0)`.
pub fn fn_007eb8a0(e: &mut Engine, this: u32) {
    e.call(AUDIO_PAUSE_TYPE, &args![this, 0x3002u32, 0u32]);
}

// Translated from 007eb8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class id of the VATS menu: `0x420`.
pub fn fn_007eb8c0(_e: &mut Engine) -> u32 {
    0x420
}

// Translated from 007eb8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: (index out): looks for a missing tile among
/// `pTiles[1..=31]`. Writes the index of the first null one and answers
/// false, or answers true when all are set (the index ends at 32).
pub fn fn_007eb8d0(e: &mut Engine, this: Ptr<VatsMenu>, index: u32) -> bool {
    e.mem.set_i32(index, 1);
    loop {
        let current = e.mem.i32(index);
        if current > 0x1f {
            return true;
        }
        let address = this
            .addr()
            .wrapping_add(0x28)
            .wrapping_add((current as u32).wrapping_mul(4));
        if e.mem.u32(address) == 0 {
            return false;
        }
        e.mem.set_i32(index, current.wrapping_add(1));
    }
}

/// Sets the special attack button `tile` to the text of `setting` and
/// shows it (`TILE_SET_STRING(0xfc4, text, 1)`, `TILE_SET_INT(0xfa3, 1)`).
fn show_special_button(e: &mut Engine, tile_offset: u32, setting: u32) {
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    let button = e.mem.u32(vats_menu(e) + tile_offset);
    e.call(TILE_SET_STRING, &args![button, 0xfc4u32, text, 1u32]);
    let button = e.mem.u32(vats_menu(e) + tile_offset);
    e.call(TILE_SET_INT, &args![button, 0xfa3u32, 1u32]);
}

/// `TILE_SET_INT(0xfa3, 0)` on the button at `tile_offset` of the menu.
fn hide_special_button(e: &mut Engine, tile_offset: u32) {
    let button = e.mem.u32(vats_menu(e) + tile_offset);
    e.call(TILE_SET_INT, &args![button, 0xfa3u32, 0u32]);
}

/// The value at the address `SETTING_VALUE_ADDRESS` answers for `setting`.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let address = e.call(SETTING_VALUE_ADDRESS, &args![setting]).u32();
    load_f32(e, address)
}

/// `HandleEntryPoint(0x28, player, form or 011ca278, value)`.
fn special_entry_point(e: &mut Engine, form: u32, value: u32) {
    let form = if form != 0 {
        form
    } else {
        e.global::<u32>(DEFAULT_FORM)
    };
    let player = e.global::<u32>(PLAYER);
    e.call(HANDLE_ENTRY_POINT, &args![0x28u32, player, form, value]);
}

// Translated from 007eb920 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (form): sets up the special attack buttons (`pTiles[30]` at
/// `+0xA0`, `pTiles[31]` at `+0xA4`) for the form (the weapon, by its use:
/// unconfirmed).
///
/// - A form whose `+0x37C` float is above 0.0 (`TEST AH,0x41`) and whose
///   player value (slot `0x08` of the player at `+0xA4`, for the form's
///   `+0x15C` word) is at least the `+0x374` float (`TEST AH,1`, NaN fails)
///   gets the first button with the text at its `+0x368`, flag
///   `011db0cf` set, the second button hidden, `011db0bc` = its `+0x378`
///   float, `011db0b8` = its `+0x37C` float and the entry point `0x28` on
///   `011db0b8`.
/// - Otherwise, with no form or a form whose `+0x15C` word is `0x2d`: when
///   `fUpperCutThreshold` is at most the player value `0x2d` (`TEST
///   AH,0x41; JP`: less or equal), the first button shows
///   `sVatsUnarmedAttackGround` when the reference at `011f21cc` answers its
///   slot `0x230`, or its slot `0x100` and `00437bd0`, and
///   `sVatsUnarmedAttack1` otherwise; `011db0cf` = 1, `011db0bc` = 1.0,
///   `011db0b8` = `fUpperCutAPCost`, entry point `0x28`. Without the ground
///   text, when `fCrossThreshold` is at most the same value, the second
///   button shows `sVatsUnarmedAttack2`, `011db0d8` = 1, `011db0c4` = 1.0,
///   `011db0c0` = `fCrossAPCost`, entry point `0x28` on `011db0c0`;
///   otherwise the second button is hidden and `011db0d8` cleared.
/// - In every other case both buttons are hidden and `011db0cf` cleared
///   (twice, as the code does).
pub fn fn_007eb920(e: &mut Engine, form: u32) {
    if form != 0 {
        let threshold = e.global::<f64>(DOUBLE_ZERO);
        // `FCOMP; TEST AH,0x41; JNZ`: continues when above.
        if fn_007ebd10(e, form) as f64 > threshold {
            let required = fn_007ebd30(e, form) as f64;
            let index = e.call(WORD_AT_15C, &args![form]).u32();
            let value = player_value_08(e, index);
            // `FILD; FCOMP; TEST AH,1; JNZ`: continues when not below.
            if value as f64 >= required {
                let text = e.call(FORM_WORD_368, &args![form]).u32();
                let button = tile(e, vats_menu(e), 30);
                e.call(TILE_SET_STRING, &args![button, 0xfc4u32, text, 1u32]);
                let button = tile(e, vats_menu(e), 30);
                e.call(TILE_SET_INT, &args![button, 0xfa3u32, 1u32]);
                e.set_global::<u8>(SPECIAL_ONE_SHOWN, 1);
                hide_special_button(e, 0xa4);
                e.set_global::<u8>(SPECIAL_TWO_SHOWN, 0);
                let factor = e.call(FLOAT_AT_378, &args![form]).f64() as f32;
                e.set_global::<f32>(SPECIAL_ONE_FACTOR, factor);
                let cost = fn_007ebd10(e, form);
                e.set_global::<f32>(SPECIAL_ONE_COST, cost);
                special_entry_point(e, form, SPECIAL_ONE_COST);
                return;
            }
        }
    }
    let unarmed = form == 0 || e.call(WORD_AT_15C, &args![form]).u32() == 0x2d;
    if unarmed {
        let value = player_value_08(e, 0x2d) as f64;
        let threshold = setting_float(e, SETTING_UPPERCUT_THRESHOLD) as f64;
        // `TEST AH,0x41; JP`: continues when less or equal.
        if threshold <= value {
            let mut ground = false;
            let reference = e.global::<u32>(TARGET_REFERENCE);
            if reference != 0
                && (e.vcall(reference, 0x230, &[]).bool()
                    || (e.vcall(reference, 0x100, &[]).bool()
                        && e.call(STATE_IS_THREE, &args![reference]).bool()))
            {
                ground = true;
                show_special_button(e, 0xa0, SETTING_UNARMED_GROUND);
            } else {
                show_special_button(e, 0xa0, SETTING_UNARMED_ONE);
            }
            e.set_global::<u8>(SPECIAL_ONE_SHOWN, 1);
            e.set_global::<f32>(SPECIAL_ONE_FACTOR, 1.0);
            let cost = setting_float(e, SETTING_UPPERCUT_AP_COST);
            e.set_global::<f32>(SPECIAL_ONE_COST, cost);
            special_entry_point(e, form, SPECIAL_ONE_COST);
            if !ground {
                let value = player_value_08(e, 0x2d) as f64;
                let threshold = setting_float(e, SETTING_CROSS_THRESHOLD) as f64;
                if threshold <= value {
                    show_special_button(e, 0xa4, SETTING_UNARMED_TWO);
                    e.set_global::<u8>(SPECIAL_TWO_SHOWN, 1);
                    e.set_global::<f32>(SPECIAL_TWO_FACTOR, 1.0);
                    let cost = setting_float(e, SETTING_CROSS_AP_COST);
                    e.set_global::<f32>(SPECIAL_TWO_COST, cost);
                    special_entry_point(e, form, SPECIAL_TWO_COST);
                    return;
                }
            }
            hide_special_button(e, 0xa4);
            e.set_global::<u8>(SPECIAL_TWO_SHOWN, 0);
            return;
        }
    }
    hide_special_button(e, 0xa0);
    e.set_global::<u8>(SPECIAL_ONE_SHOWN, 0);
    hide_special_button(e, 0xa4);
    e.set_global::<u8>(SPECIAL_ONE_SHOWN, 0);
}

// Translated from 007ebd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on a form: the float at `+0x37C`.
pub fn fn_007ebd10(e: &mut Engine, this: u32) -> f32 {
    load_f32(e, this + 0x37c)
}

// Translated from 007ebd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on a form: the float at `+0x374`.
pub fn fn_007ebd30(e: &mut Engine, this: u32) -> f32 {
    load_f32(e, this + 0x374)
}

/// Plays the sound `name` through a `BSSoundHandle` local (0xC bytes):
/// constructs it (`0041a250`), fetches the handle into a temporary with
/// `fetch` (the audio manager's or the audio object's lookup, flags
/// `0x121`), assigns it (`00418900`), destroys the temporary (`00483710`),
/// plays it (`BSSoundHandle::Play(0)`) and destroys the local.
/// `before_fetch` runs between the construction and the lookup (the code
/// that picks the name or fetches the audio object).
fn play_sound_local(
    e: &mut Engine,
    lookup: u32,
    before_fetch: impl FnOnce(&mut Engine) -> (u32, u32),
) {
    play_sound_then(e, lookup, before_fetch, |_| {});
}

/// [`play_sound_local`] with `after_play` running after `Play` and before
/// the local is destroyed.
fn play_sound_then(
    e: &mut Engine,
    lookup: u32,
    before_fetch: impl FnOnce(&mut Engine) -> (u32, u32),
    after_play: impl FnOnce(&mut Engine),
) {
    e.with_stack(0x18, |e, block| {
        let handle = block.addr();
        let temporary = handle + 0xc;
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
        let (object, name) = before_fetch(e);
        let fetched = e
            .call(lookup, &args![object, temporary, name, 0x121u32])
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, fetched]);
        e.call(EMPTY_MEMBER, &args![temporary]);
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        after_play(e);
        e.call(EMPTY_MEMBER, &args![handle]);
    });
}

/// `HUDMainMenu` message (`00775380` on `*011d96c0`): the text of
/// `setting`, 2, 0, 0, 2.0, 0.
fn show_hud_message(e: &mut Engine, setting: u32) {
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    let hud = e.global::<u32>(HUD_MAIN_MENU);
    let two = load_f32(e, FLOAT_TWO).to_bits();
    e.call(
        HUD_SHOW_MESSAGE,
        &args![hud, text, 2u32, 0u32, 0u32, two, 0u32],
    );
}

/// Fills a new `VATS_COMMAND` for a perk strike of `reference`
/// (`eAction` 3, `bSuccess`, `cFireShots` 5, `pTarget`, `eLimb` 0x19 and
/// the flag at `flag_offset`), queues it on the VATS object
/// (`009c71a0`) and has `actor` attack through its slot `0x424`
/// (`reference, 0, 1, 1, 0, 0, 1, 0`).
fn queue_perk_strike(e: &mut Engine, actor: u32, flag_offset: u32) {
    let block = e.call(ALLOCATE, &args![0x28u32]).u32();
    let command = if block != 0 {
        e.call(VATS_COMMAND_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(command + 0x10, 0x19);
    e.mem.set_u8(command + 0x8, 5);
    e.mem.set_u32(command, 3);
    e.mem.set_u8(command + flag_offset, 1);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    e.mem.set_u32(command + 0xc, reference);
    e.mem.set_u8(command + 0x4, 1);
    e.call(VATS_QUEUE_COMMAND, &args![VATS, command]);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let actor_now = e.global::<u32>(actor);
    e.vcall(
        actor_now,
        0x424,
        &args![reference, 0u32, 1u32, 1u32, 0u32, 0u32, 1u32, 0u32],
    );
}

/// Whether the perk strike of the actor at `actor` (global) applies: it
/// and the reference at `011f21cc` exist, the reference answers its slot
/// `0x100`, is not essential (`Actor::GetEssential`) and `RandomFloat(0,
/// 1)` is not above `odds` (`FCOMPP; TEST AH,1`: NaN fails).
fn perk_strike_applies(e: &mut Engine, actor: u32, odds: f32) -> bool {
    if e.global::<u32>(actor) == 0 {
        return false;
    }
    let reference = e.global::<u32>(TARGET_REFERENCE);
    if reference == 0 || !e.vcall(reference, 0x100, &[]).bool() {
        return false;
    }
    if e.call(ACTOR_GET_ESSENTIAL, &args![reference]).bool() {
        return false;
    }
    let random = e
        .call(RANDOM_FLOAT, &args![0.0f32.to_bits(), 1.0f32.to_bits()])
        .f64();
    odds as f64 >= random
}

/// Releases the object at the global `slot` through its slot `0x10` with 1
/// and clears the global.
fn release_global(e: &mut Engine, slot: u32) {
    let object = e.global::<u32>(slot);
    if object != 0 {
        e.vcall(object, 0x10, &args![1u32]);
        e.set_global::<u32>(slot, 0);
    }
}

// Translated from 007ebd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::Close` (Xbox PDB): closes the menu. Runs only with a menu
/// whose `+0x24` word (`0059bb30`) is 1. Unless already closing, resumes
/// the paused sounds (`007ec380` on `BSAudio::QInstance`); sets `bClosing`.
/// Without a tile of menu class `0x420`, the VATS object goes to mode 0
/// and `011a3b31` is cleared. Otherwise, with the menu open: the list at
/// `011db124` is cleared when not empty, `0070bc20` runs on `004b7210`'s
/// object, sounds of type `0x100` end, and unless the save/load object at
/// `011ddf38` has its `+0x244` bit 2 (`0042ce10`), `UIVATSExit` (or
/// `UIVATSEnterFail` when the list at `011db150` is empty) plays; trait
/// `0x1772` of the menu tile is set to 1 and the fade out starts. Then,
/// when the VATS object has no queued action (`VATS::GetCount`), mode 0,
/// `011a3b31` cleared and both perk actors released; otherwise
/// `UIVATSReady` plays, `fVATSFortuneOdds`/`fVATSStrangerOdds` go through
/// the entry point `0x43`, the perk strikes are queued when they apply
/// (the stranger also needs the reference outside the faction of default
/// object 8) and the VATS object goes to mode 4. Finally `00800ac0(0, -1,
/// 0)` and `00800ed0` on `00705910`'s object, and `011db0cd` is cleared.
// Translation incomplete: 007ebd50 - the C++ exception-unwinding frame (FS:[0]) and the security cookie are not translated
pub fn vats_menu_close(e: &mut Engine) {
    let menu = vats_menu(e);
    if menu == 0 || e.call(MENU_FADE_STATE, &args![menu]).u32() != 1 {
        return;
    }
    if e.mem.u8(vats_menu(e) + 0xfb) == 0 {
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        fn_007ec380(e, audio);
    }
    let menu = vats_menu(e);
    e.mem.set_u8(menu + 0xfb, 1);
    let class = fn_007eb8c0(e);
    let menu_tile = e.call(GET_MENU_BY_CLASS, &args![class]).u32();
    if menu_tile == 0 {
        e.call(VATS_SET_MODE, &args![VATS, 0u32, 0u32]);
        e.set_global::<u8>(FLAG_011A3B31, 0);
        return;
    }
    if vats_menu(e) != 0 {
        if !e.call(LIST_IS_EMPTY, &args![LIST_011DB124]).bool() {
            e.call(LIST_CLEAR, &args![LIST_011DB124]);
        }
        let object = e.call(OBJECT_004B7210, &[]).u32();
        e.call(UPDATE_0070BC20, &args![object]);
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        e.call(END_ALL_SOUNDS_OF_TYPE, &args![audio, 0x100u32]);
        let save_load = e.global::<u32>(SAVE_LOAD_GAME);
        if !e.call(FLAG_244_BIT_2, &args![save_load]).bool() {
            let manager = e.call(AUDIO_MANAGER_INSTANCE, &[]).u32();
            play_sound_local(e, GET_SOUND_HANDLE_BY_NAME, |e| {
                let name = if e.call(LIST_IS_EMPTY, &args![LIST_011DB150]).bool() {
                    TEXT_UI_VATS_ENTER_FAIL
                } else {
                    TEXT_UI_VATS_EXIT
                };
                (manager, name)
            });
        }
        e.call(TILE_SET_INT, &args![menu_tile, 0x1772u32, 1u32]);
        let menu = vats_menu(e);
        e.call(MENU_START_FADE_OUT, &args![menu]);
    }
    if e.call(VATS_GET_COUNT, &args![VATS]).u32() == 0 {
        e.call(VATS_SET_MODE, &args![VATS, 0u32, 0u32]);
        e.set_global::<u8>(FLAG_011A3B31, 0);
        release_global(e, STRANGER_ACTOR);
        release_global(e, FORTUNE_ACTOR);
    } else {
        let manager = e.call(AUDIO_MANAGER_INSTANCE, &[]).u32();
        play_sound_local(e, GET_SOUND_HANDLE_BY_NAME, |_| {
            (manager, TEXT_UI_VATS_READY)
        });
        e.with_stack(8, |e, odds| {
            let fortune = odds.addr();
            let stranger = fortune + 4;
            let value = setting_float(e, SETTING_FORTUNE_ODDS);
            e.mem.set_f32(fortune, value);
            let value = setting_float(e, SETTING_STRANGER_ODDS);
            e.mem.set_f32(stranger, value);
            let player = e.global::<u32>(PLAYER);
            e.call(HANDLE_ENTRY_POINT, &args![0x43u32, player, fortune]);
            let player = e.global::<u32>(PLAYER);
            e.call(HANDLE_ENTRY_POINT, &args![0x43u32, player, stranger]);
            let stranger_odds = load_f32(e, stranger);
            if perk_strike_applies(e, STRANGER_ACTOR, stranger_odds) {
                let faction = e.call(GET_DEFAULT_OBJECT, &args![8u32]).u32();
                if faction != 0 {
                    let reference = e.global::<u32>(TARGET_REFERENCE);
                    if !e
                        .call(ACTOR_IS_IN_FACTION, &args![reference, faction])
                        .bool()
                    {
                        queue_perk_strike(e, STRANGER_ACTOR, 0x6);
                    }
                }
            }
            let fortune_odds = load_f32(e, fortune);
            if perk_strike_applies(e, FORTUNE_ACTOR, fortune_odds) {
                queue_perk_strike(e, FORTUNE_ACTOR, 0x24);
            }
        });
        e.call(VATS_SET_MODE, &args![VATS, 4u32, 0u32]);
    }
    let object = e.call(OBJECT_00705910, &[]).u32();
    e.call(SET_00800AC0, &args![object, 0u32, 0xffff_ffffu32, 0u32]);
    let object = e.call(OBJECT_00705910, &[]).u32();
    e.call(UPDATE_00800ED0, &args![object]);
    e.set_global::<u8>(FLAG_011DB0CD, 0);
}

// Translated from 007ec380 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the audio object (`BSAudio`): `00ad8540(0x3002, 0)`.
pub fn fn_007ec380(e: &mut Engine, this: u32) {
    e.call(AUDIO_RESUME_TYPE, &args![this, 0x3002u32, 0u32]);
}

/// `eLimb` of the menu's `pCurrentLimb`.
fn current_limb(e: &Engine) -> u32 {
    let limb = e.mem.u32(vats_menu(e) + 0x100);
    e.mem.u32(limb + 0x20)
}

/// `00800ac0(first, second, flag)` on `00705910`'s object (the arguments
/// are computed before `00705910` runs, as the code pushes them).
fn set_00800ac0(e: &mut Engine, first: u32, second: u32, flag: u32) {
    let object = e.call(OBJECT_00705910, &[]).u32();
    e.call(SET_00800AC0, &args![object, first, second, flag]);
}

// Translated from 007ec3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::ShowTargetLimb` (Xbox PDB), cdecl (forced): shows the current
/// limb (`pCurrentLimb`) of the target reference (`011f21cc`).
///
/// Unless forced, nothing happens when the VATS object's `+8` word is not
/// 2, or when the limb's actor value matches the last one shown
/// (`011a0b70`) on the same reference (`011db0d0`); on the same reference,
/// `UIVATSSelectTargetPart` plays unless the player's `+0x68` object
/// answers its slot `0x1a8`. Then `fProbableEnemyDamage` = `007f48e0`.
/// With a target (`GetVatsTarget`) that is not a destructible object
/// (type 2) and the process check failing (slot `0x1a8` false, a slot
/// `0x148` object with a `+8` word that `006450c0` rejects), or without a
/// target, the limb is shown on the reference through `00800ac0` when its
/// target reference is an actor (slot `0x100`) with a `+0x68` object
/// (flag 1 when the VATS object's word is 3 and `bFirstTimeShown` is
/// clear), every other part of the target with the same actor value is
/// passed to `00800d10`, and the shown limb and reference are remembered.
/// Otherwise, when forced or (word 2 and another reference), the target's
/// reference gets `00800ac0` (limb, 1 for word 3; 0xF, 0 otherwise) and
/// becomes the remembered reference; the limb is always remembered.
// Translation incomplete: 007ec3a0 - the C++ exception-unwinding frame (FS:[0]) and the security cookie are not translated
pub fn vats_menu_show_target_limb(e: &mut Engine, forced: u8) {
    let menu = vats_menu(e);
    if menu == 0 {
        return;
    }
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let target = vats_menu_get_vats_target(e, Ptr::new(menu), reference);
    if forced == 0 {
        if e.call(WORD_AT_8, &args![VATS]).u32() != 2 {
            return;
        }
        let last = e.global::<u32>(LAST_LIMB);
        let menu = vats_menu(e);
        let shown = vats_menu_get_actor_value_from_body_part(e, Ptr::new(menu), last);
        let limb = current_limb(e);
        let menu = vats_menu(e);
        let current = vats_menu_get_actor_value_from_body_part(e, Ptr::new(menu), limb);
        if shown == current && e.global::<u32>(LAST_REFERENCE) == e.global::<u32>(TARGET_REFERENCE)
        {
            return;
        }
    }
    if forced == 0 && e.global::<u32>(LAST_REFERENCE) == e.global::<u32>(TARGET_REFERENCE) {
        let player = e.global::<u32>(PLAYER);
        let process = e.call(WORD_AT_68, &args![player]).u32();
        if !e.vcall(process, 0x1a8, &[]).bool() {
            play_sound_local(e, GET_SOUND_HANDLE_BY_NAME_ON_AUDIO, |e| {
                let audio = e.call(AUDIO_INSTANCE, &[]).u32();
                (audio, TEXT_UI_VATS_SELECT_TARGET_PART)
            });
        }
    }
    let menu = vats_menu(e);
    let damage = fn_007f48e0(e, Ptr::new(menu));
    let menu = vats_menu(e);
    e.mem.set_f32(menu + 0xfc, damage);
    let mut target_actor = 0;
    if target != 0 {
        let node = e.call(IDENTITY, &args![target]).u32();
        if e.mem.u32(node) != 0 {
            let node = e.call(IDENTITY, &args![target]).u32();
            let candidate = e.mem.u32(e.mem.u32(node));
            if e.vcall(candidate, 0x100, &[]).bool() {
                let node = e.call(IDENTITY, &args![target]).u32();
                target_actor = e.mem.u32(e.mem.u32(node));
            }
        }
    }
    let player = e.global::<u32>(PLAYER);
    let process = e.call(WORD_AT_68, &args![player]).u32();
    let object = e.vcall(process, 0x148, &[]).u32();
    let word = if object != 0 {
        e.call(WORD_AT_8, &args![object]).u32()
    } else {
        0
    };
    let mut on_target = target != 0;
    if on_target {
        let node = e.call(IDENTITY, &args![target]).u32();
        let item = e.mem.u32(node);
        if e.mem.u32(item + 4) != 2 {
            let player = e.global::<u32>(PLAYER);
            let process = e.call(WORD_AT_68, &args![player]).u32();
            if !e.vcall(process, 0x1a8, &[]).bool()
                && word != 0
                && !e.call(CHECK_006450C0, &args![word]).bool()
            {
                on_target = false;
            }
        }
    }
    if on_target {
        let show = forced != 0
            || (e.call(WORD_AT_8, &args![VATS]).u32() == 2
                && e.global::<u32>(LAST_REFERENCE) != e.global::<u32>(TARGET_REFERENCE));
        if show {
            if e.call(WORD_AT_8, &args![VATS]).u32() == 3 {
                let limb = current_limb(e);
                let node = e.call(IDENTITY, &args![target]).u32();
                let target_reference = e.mem.u32(e.mem.u32(node));
                set_00800ac0(e, target_reference, limb, 1);
            } else {
                let node = e.call(IDENTITY, &args![target]).u32();
                let target_reference = e.mem.u32(e.mem.u32(node));
                set_00800ac0(e, target_reference, 0xf, 0);
            }
            let node = e.call(IDENTITY, &args![target]).u32();
            let target_reference = e.mem.u32(e.mem.u32(node));
            e.set_global::<u32>(LAST_REFERENCE, target_reference);
        }
        let limb = current_limb(e);
        e.set_global::<u32>(LAST_LIMB, limb);
        return;
    }
    if target_actor == 0 || e.call(WORD_AT_68, &args![target_actor]).u32() == 0 {
        return;
    }
    let first_time = {
        let limb = e.mem.u32(vats_menu(e) + 0x100);
        e.mem.u8(limb + 0x2e)
    };
    let flag = u32::from(first_time == 0 && e.call(WORD_AT_8, &args![VATS]).u32() == 3);
    let limb = current_limb(e);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    set_00800ac0(e, reference, limb, flag);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let menu = vats_menu(e);
    let target = vats_menu_get_vats_target(e, Ptr::new(menu), reference);
    let node = e.call(IDENTITY, &args![target]).u32();
    let mut part_node = e.mem.u32(node).wrapping_add(8);
    while part_node != 0 && !e.call(LIST_IS_EMPTY, &args![part_node]).bool() {
        let node = e.call(IDENTITY, &args![part_node]).u32();
        let part_limb = e.mem.u32(e.mem.u32(node) + 0x20);
        let menu = vats_menu(e);
        let part_value = vats_menu_get_actor_value_from_body_part(e, Ptr::new(menu), part_limb);
        let limb = current_limb(e);
        let menu = vats_menu(e);
        let current = vats_menu_get_actor_value_from_body_part(e, Ptr::new(menu), limb);
        if part_value == current {
            let node = e.call(IDENTITY, &args![part_node]).u32();
            let part = e.mem.u32(node);
            if part != e.mem.u32(vats_menu(e) + 0x100) {
                let node = e.call(IDENTITY, &args![part_node]).u32();
                let part_limb = e.mem.u32(e.mem.u32(node) + 0x20);
                let object = e.call(OBJECT_00705910, &[]).u32();
                e.call(SET_00800D10, &args![object, part_limb]);
            }
        }
        part_node = e.call(NEXT_NODE, &args![part_node]).u32();
    }
    let limb = current_limb(e);
    e.set_global::<u32>(LAST_LIMB, limb);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    e.set_global::<u32>(LAST_REFERENCE, reference);
}

/// `FCOMP`/`FCOMPP` then `TEST AH,0x41`: `a` above `b` (false for NaN).
fn above(a: f64, b: f64) -> bool {
    a > b
}

/// `FCOMPP` then `TEST AH,0x41; JP` not taken: `a` less than or equal to
/// `b` (false for NaN).
fn at_most(a: f64, b: f64) -> bool {
    a <= b
}

/// The double at `01012060` (0.0) the code compares with.
fn double_zero(e: &Engine) -> f64 {
    e.global::<f64>(DOUBLE_ZERO)
}

/// `004b7210` (its object is not used) then `004b71d0`: whether the
/// gamepad interface is in use.
fn gamepad_in_use(e: &mut Engine) -> bool {
    e.call(OBJECT_004B7210, &[]);
    e.call(GAMEPAD_IN_USE, &[]).bool()
}

/// `TILE_SET_INT(trait, value)` on the tile at `offset` of `menu`.
fn set_tile_int(e: &mut Engine, menu: u32, offset: u32, trait_id: u32, value: u32) {
    let tile = e.mem.u32(menu + offset);
    e.call(TILE_SET_INT, &args![tile, trait_id, value]);
}

/// The truncated integer of `ftol(value)` (`_ftol2`, ST0 passed as `f64`).
fn ftol(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// The function-local static at `slot`, guarded by `bit` of the guard word
/// at `011db194`: the first call stores `ftol` of the float `getter`
/// answers.
fn init_static_size(e: &mut Engine, bit: u32, slot: u32, getter: u32) {
    let guard = e.global::<u32>(STATIC_GUARD);
    if guard & bit == 0 {
        e.set_global::<u32>(STATIC_GUARD, guard | bit);
        let value = e.call(getter, &[]).f64();
        let truncated = ftol(e, value);
        e.set_global::<i32>(slot, truncated);
    }
}

/// The chance of the current target part (`GetTargetPart`), when there is
/// one: `fChance * 100.0` truncated.
fn target_part_percent(e: &mut Engine, menu: Ptr<VatsMenu>) -> Option<i32> {
    if vats_menu_get_target_part(e, menu) == 0 {
        return None;
    }
    let node = vats_menu_get_target_part(e, menu);
    let node = e.call(IDENTITY, &args![node]).u32();
    let part = e.mem.u32(node);
    let chance = load_f32(e, part + 0x28) as f64;
    let hundred = e.global::<f64>(DOUBLE_HUNDRED);
    Some(ftol(e, chance * hundred))
}

/// `[+0xF4] + [+0xF0]` of the menu (queued total and clip ammo) as x87
/// adds them.
fn queued_ammo(e: &Engine, menu: u32) -> f64 {
    load_f32(e, menu + 0xf4) as f64 + load_f32(e, menu + 0xf0) as f64
}

/// The `BSSimpleList` node of the target parts of `reference`'s target
/// (`GetVatsTarget`, then the item's `+8`).
fn target_parts(e: &mut Engine, menu: Ptr<VatsMenu>, reference: u32) -> u32 {
    let target = vats_menu_get_vats_target(e, menu, reference);
    let node = e.call(IDENTITY, &args![target]).u32();
    e.mem.u32(node).wrapping_add(8)
}

/// `eLimb` of the item of the list node `node`.
fn node_limb(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(IDENTITY, &args![node]).u32();
    let part = e.mem.u32(node);
    e.mem.u32(part + 0x20)
}

// Translated from 007ec810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::DoIdle` (Xbox PDB): the menu's frame update. Every branch
/// of the code is kept in order; the comments name the code's blocks.
///
/// Leaves at once while closing or with a menu of class `0x423` open;
/// closes the menu (`VATSMenu::Close`) when the target reference is gone,
/// answers 0 in its slot `0x1d0`, or it or the player answers its slot
/// `0x22c(0)`. Without the menu on top (or with the VATS object's mode 0
/// or 4) the meters lose their fade controls. Otherwise it builds the
/// target's meters, shows the limb, updates the chance to hit, reads the
/// gamepad stick (switching targets), updates the target's and the
/// player's health meters, the compass, the VATS mode from the controls,
/// the queued actions and the special attack buttons, then handles the
/// attack control: it queues a `VATS_COMMAND` for the current part (the
/// action, shots, limb, paralyzing palm, success roll) when the action
/// points, ammo and chance allow it, or plays the failure sound and
/// message.
// Translation incomplete: 007ec810 - the C++ exception-unwinding frame (FS:[0]) and the security cookie (00ec408c) are not translated
pub fn vats_menu_do_idle(e: &mut Engine, this: Ptr<VatsMenu>) {
    let m = this.addr();
    if e.mem.u8(m + 0xfb) != 0 {
        return;
    }
    if e.call(GET_MENU_BY_CLASS, &args![0x423u32]).u32() != 0 {
        return;
    }
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let close = reference == 0
        || e.vcall(reference, 0x1d0, &[]).u32() == 0
        || e.vcall(reference, 0x22c, &args![0u32]).bool()
        || {
            let player = e.global::<u32>(PLAYER);
            e.vcall(player, 0x22c, &args![0u32]).bool()
        };
    if close {
        vats_menu_close(e);
        return;
    }
    let active = vats_menu(e) != 0
        && e.call(WORD_AT_8, &args![VATS]).u32() != 0
        && e.call(WORD_AT_8, &args![VATS]).u32() != 4
        && e.call(IS_TOP_MENU_ID, &args![0x420u32]).bool();
    if !active {
        if !e.call(IS_TOP_MENU_ID, &args![0x420u32]).bool() && e.global::<u8>(LIMB_SHOWN) != 0 {
            e.set_global::<u8>(LIMB_SHOWN, 0);
            set_00800ac0(e, 0, 0xffff_ffff, 0);
        }
        let ghost = e.mem.u32(m + 0x98);
        e.call(REMOVE_FADE_CONTROL, &args![ghost, 0x8000_0000u32]);
        let ghost = e.mem.u32(m + 0x98);
        let child = e.call(FIRST_CHILD_DATA, &args![ghost + 4]).u32();
        let child = e.mem.u32(child);
        e.call(REMOVE_FADE_CONTROL, &args![child, 0x8000_0000u32]);
        let bar = e.mem.u32(m + 0x50);
        e.call(REMOVE_FADE_CONTROL, &args![bar, 0x8000_0000u32]);
        return;
    }
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let target = vats_menu_get_vats_target(e, this, reference);
    let node = e.call(IDENTITY, &args![target]).u32();
    let parts = e.mem.u32(node) + 8;
    let parts = e.call(IDENTITY, &args![parts]).u32();
    if e.mem.u32(e.mem.u32(parts) + 0x30) == 0 {
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let target = vats_menu_get_vats_target(e, this, reference);
        e.call(BUILD_METERS_FOR_TARGET, &args![m, target]);
    }
    if e.global::<u8>(LIMB_SHOWN) == 0 {
        vats_menu_show_target_limb(e, 1);
    }
    e.set_global::<u8>(LIMB_SHOWN, 1);
    if e.call(IS_CONSOLE_VISIBLE, &[]).bool() {
        return;
    }
    vats_menu_update_chance_to_hit(e, this);
    let holder = e.global::<u32>(CONTROLS_HOLDER);
    let controls = e.call(CONTROLS, &args![holder]).u32();

    // 007eca2a: the gamepad stick.
    if gamepad_in_use(e) {
        // The value computed from the VATS mode (0x1ea9 or 0x21f1) and the
        // first two axis reads are never used.
        e.call(WORD_AT_8, &args![VATS]);
        e.call(AXIS_VALUE, &args![controls, 0u32, 7u32]);
        e.call(AXIS_VALUE, &args![controls, 0u32, 8u32]);
        let horizontal = e.call(AXIS_VALUE, &args![controls, 0u32, 9u32]).i32();
        let vertical = e
            .call(AXIS_VALUE, &args![controls, 0u32, 10u32])
            .i32()
            .wrapping_neg();
        if horizontal > 0x21f1 {
            if e.mem.u8(m + 0x110) == 0 {
                let time = e.call(TIMER_MILLISECONDS, &args![TIMER]).u32();
                e.mem.set_u32(m + 0x114, time);
                e.mem.set_u8(m + 0x110, 1);
            }
        } else if horizontal < -0x21f1 {
            if e.mem.u8(m + 0x10f) == 0 {
                let time = e.call(TIMER_MILLISECONDS, &args![TIMER]).u32();
                e.mem.set_u32(m + 0x114, time);
                e.mem.set_u8(m + 0x10f, 1);
            }
        } else if e.call(ABS, &args![horizontal]).i32() < 0x21f1
            && e.call(ABS, &args![vertical]).i32() < 0x21f1
        {
            if fn_007f4c00(e) {
                let menu = vats_menu(e);
                e.mem.set_u8(menu + 0x110, 0);
                e.vcall(m, 0x0c, &args![3u32, 0u32]);
            } else if e.mem.u8(m + 0x110) != 0 {
                e.vcall(m, 0x0c, &args![3u32, 0u32]);
            } else if fn_007f4bb0(e) {
                let menu = vats_menu(e);
                e.mem.set_u8(menu + 0x10f, 0);
                e.vcall(m, 0x0c, &args![2u32, 0u32]);
            } else if e.mem.u8(m + 0x10f) != 0 {
                e.vcall(m, 0x0c, &args![2u32, 0u32]);
            }
            e.mem.set_u8(m + 0x10d, 1);
        }
    }

    // 007ecc20: the target's health meter and name.
    let reference = e.global::<u32>(TARGET_REFERENCE);
    if reference != 0 && e.vcall(reference, 0x100, &[]).bool() {
        let owner = reference + 0xa4;
        let base = e.vcall(owner, 0x1c, &args![0x10u32]).i32() as f32;
        let owner = e.global::<u32>(TARGET_REFERENCE) + 0xa4;
        let health = e.vcall(owner, 0x08, &args![0x10u32]).i32() as f32;
        let mut ratio = 0.0f32;
        // `TEST AH,0x41; JNZ`: divides when above 0.0.
        if base as f64 > double_zero(e) {
            ratio = (health as f64 / base as f64) as f32;
        }
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let last = if e.vcall(reference, 0x22c, &args![0u32]).bool() {
            0
        } else {
            2
        };
        let ghost = (ratio as f64 - load_f32(e, m + 0xfc) as f64) as f32;
        let meter = e.mem.u32(m + 0x90);
        let meter_ghost = e.mem.u32(m + 0x98);
        e.call(
            HUD_UPDATE_METER_GHOST,
            &args![meter, meter_ghost, ghost, ratio, 2u32, last],
        );
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let name = e.call(REFERENCE_NAME, &args![reference]).u32();
        let name_tile = e.mem.u32(m + 0x9c);
        e.call(TILE_SET_STRING, &args![name_tile, 0xfc4u32, name, 1u32]);
        for offset in [0x8c, 0x90, 0x98, 0x94, 0x9c] {
            set_tile_int(e, m, offset, 0xfa3, 1);
        }
        let player = e.global::<u32>(PLAYER);
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let threshold = e.call(DAMAGE_THRESHOLD, &args![player, reference]).f64() as f32;
        e.with_stack(4, |e, value| {
            let value = value.addr();
            e.mem.set_f32(value, 0.0);
            let player = e.global::<u32>(PLAYER);
            e.call(HANDLE_ENTRY_POINT, &args![0x2fu32, player, value]);
            // `TEST AH,0x41; JNZ`: the detailed name when above 0.0.
            if load_f32(e, value) as f64 > double_zero(e) {
                e.with_stack(0x104, |e, buffer| {
                    let buffer = buffer.addr();
                    let threshold = threshold as f64;
                    let threshold_text = e
                        .call(SETTING_STRING, &args![SETTING_DAMAGE_THRESHOLD_TEXT])
                        .u32();
                    let hit_points = ftol(e, health as f64);
                    let hp_text = e.call(SETTING_STRING, &args![SETTING_STATS_HP]).u32();
                    let reference = e.global::<u32>(TARGET_REFERENCE);
                    let name = e.call(REFERENCE_NAME, &args![reference]).u32();
                    e.call(
                        SPRINTF,
                        &args![
                            buffer,
                            TEXT_NAME_FORMAT,
                            name,
                            hp_text,
                            hit_points,
                            threshold_text,
                            threshold
                        ],
                    );
                    let name_tile = e.mem.u32(m + 0x9c);
                    e.call(TILE_SET_STRING, &args![name_tile, 0xfc4u32, buffer, 1u32]);
                });
            } else {
                let reference = e.global::<u32>(TARGET_REFERENCE);
                let name = e.call(REFERENCE_NAME, &args![reference]).u32();
                let name_tile = e.mem.u32(m + 0x9c);
                e.call(TILE_SET_STRING, &args![name_tile, 0xfc4u32, name, 1u32]);
            }
        });
        let player = e.global::<u32>(PLAYER);
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let perceived = e.call(IS_ACTOR_PERCEIVED, &args![player, reference]).u32() != 0 && {
            let player = e.global::<u32>(PLAYER);
            let reference = e.global::<u32>(TARGET_REFERENCE);
            let entry = e.call(IS_ACTOR_PERCEIVED, &args![player, reference]).u32();
            e.mem.u8(entry + 4) != 0
        };
        let value = if perceived { 2 } else { 1 };
        for offset in [0x9c, 0x8c, 0x90, 0x98, 0x94] {
            set_tile_int(e, m, offset, 0xff4, value);
        }
    } else {
        for offset in [0x8c, 0x90, 0x98, 0x94, 0x9c] {
            set_tile_int(e, m, offset, 0xfa3, 0);
        }
    }

    // 007ed092: the player's health meter and the compass.
    let owner = e.global::<u32>(PLAYER) + 0xa4;
    let base = e.vcall(owner, 0x1c, &args![0x10u32]).u32();
    let mut ratio = e.call(INT_MAX, &args![0u32, base]).i32() as f32;
    // `TEST AH,0x44; JNP`: divides unless equal to 0.0.
    if ratio as f64 != double_zero(e) {
        let owner = e.global::<u32>(PLAYER) + 0xa4;
        let health = e.vcall(owner, 0x08, &args![0x10u32]).u32();
        let health = e.call(INT_MAX, &args![0u32, health]).i32();
        ratio = (health as f64 / ratio as f64) as f32;
    }
    let meter = e.mem.u32(m + 0x70);
    e.call(HUD_UPDATE_METER, &args![meter, ratio, 0u32, 0u32, 0u32]);
    let (compass, general, markers, npcs) = (
        e.mem.u32(m + 0x84),
        e.mem.u32(m + 0x78),
        e.mem.u32(m + 0x7c),
        e.mem.u32(m + 0x80),
    );
    e.call(HUD_UPDATE_COMPASS, &args![compass, general, markers, npcs]);

    // 007ed16b: the VATS mode from the controls.
    let key = e.call(CONTROL_KEY, &args![controls, 0x10u32]).i32();
    let key_high = key >> 16;
    let key_mid = (key & 0xff00) >> 8;
    let key_low = key & 0xff;
    if e.call(WORD_AT_8, &args![VATS]).u32() != 4 {
        if e.call(CONTROL_STATE, &args![controls, 0x10u32, 1u32]).u32() != 0
            && e.call(WORD_AT_8, &args![VATS]).u32() != 1
        {
            let excepted =
                e.call(GAMEPAD_IN_USE, &[]).bool() && matches!(key_low, 0xa | 0xb | 0x11);
            if !excepted {
                e.call(VATS_SET_MODE, &args![VATS, 1u32, 0u32]);
                e.mem.set_u8(m + 0xfa, 0);
            }
        } else if e.call(CONTROL_STATE, &args![controls, 0x10u32, 0u32]).u32() == 0
            && e.call(WORD_AT_8, &args![VATS]).u32() == 1
        {
            e.call(VATS_SET_MODE, &args![VATS, 2u32, 0u32]);
            e.mem.set_u8(m + 0xfa, 1);
        }
    }
    let frame = e.call(TIMER_FLOAT_C, &args![TIMER]).f64() as f32;
    let player = e.global::<u32>(PLAYER);
    e.call(PLAYER_UPDATE_945B0, &args![player, frame, 0u32, 0u32, 0u32]);
    let mut accept = e.call(OBJECT_00709D40, &[]).bool()
        && (e.call(CHECK_MENU_BUTTON, &args![0xau32, 1u32]).i32() == 1
            || e.call(CHECK_MENU_BUTTON, &args![6u32, 1u32]).i32() == 1);
    if !accept {
        accept = e.call(BUTTON_STATE, &args![controls, 1u32, 1u32]).u32() != 0 && key_mid != 1;
    }
    if accept {
        // 007ed2ad
        let proceed = if !gamepad_in_use(e) && e.mem.u8(m + 0x10e) == 0 {
            true
        } else {
            gamepad_in_use(e) && e.call(CHECK_MENU_BUTTON, &args![0xau32, 1u32]).i32() == 1
        };
        if proceed {
            if e.call(WORD_AT_8, &args![VATS]).u32() == 2 && e.mem.u8(m + 0xf8) != 0 {
                vats_menu_update_queued_action_list(e, this, 0, 0, 0);
                let menu = vats_menu(e);
                let damage = fn_007f48e0(e, Ptr::new(menu));
                let menu = vats_menu(e);
                e.mem.set_f32(menu + 0xfc, damage);
            }
            fn_007f0ea0(e, this);
        }
    }

    // 007ed343
    if !fn_007f3e00(e, this) {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    let process = e.call(WORD_AT_68, &args![player]).u32();
    let equipped = e.vcall(process, 0x148, &[]).u32();
    let weapon = if equipped != 0 {
        e.call(WORD_AT_8, &args![equipped]).u32()
    } else {
        0
    };
    let rock_it = weapon != 0 && e.call(IS_ROCK_IT_LAUNCHER, &args![weapon + 0xa4]).bool();
    fn_007f3070(e, this);
    if e.call(WORD_AT_8, &args![VATS]).u32() == 4 {
        return;
    }
    e.set_global::<u32>(GLOBAL_011DEA34, 0);
    let player = e.global::<u32>(PLAYER);
    e.call(UPDATE_AUTO_AIM_ACTOR, &args![player]);
    let frame = e.call(TIMER_FLOAT_C, &args![TIMER]).f64() as f32;
    let player = e.global::<u32>(PLAYER);
    e.call(CHARACTER_UPDATE_8D3550, &args![player, frame]);
    let player = e.global::<u32>(PLAYER);
    let movement = e.vcall(player, 0x1e4, &[]).u32();
    let player = e.global::<u32>(PLAYER);
    e.call(UPDATE_ANIMATION_MOVEMENT, &args![player, movement, 0.0f32]);
    let player = e.global::<u32>(PLAYER);
    e.call(UPDATE_CAMERA, &args![player, 0u32, 0u32]);
    let player = e.global::<u32>(PLAYER);
    e.call(UPDATE_FIRST_PERSON_ZOOM, &args![player]);
    vats_menu_show_target_limb(e, 0);
    let object = e.call(OBJECT_004B7210, &[]).u32();
    e.call(OBJECT_00747D00, &args![object]);
    e.call(OBJECT_0073B020, &args![object]);
    init_static_size(e, 1, STATIC_SCREEN_WIDTH, SCREEN_WIDTH);
    init_static_size(e, 2, STATIC_SIZE_00715DA0, SIZE_00715DA0);
    init_static_size(e, 4, STATIC_SIZE_007177C0, SIZE_007177C0);
    init_static_size(e, 8, STATIC_SIZE_00717820, SIZE_00717820);
    let close_vats = if e
        .call(CONTROL_PRESSED, &args![controls, 0x12u32, 0u32])
        .u32()
        != 0
        && key_high != 0x12
    {
        true
    } else {
        gamepad_in_use(e)
            && e.call(CHECK_MENU_BUTTON, &args![9u32, 1u32]).i32() == 2
            && {
                let vats = e.call(IDENTITY, &args![VATS]).u32();
                e.call(VATS_GET_COUNT, &args![vats]).u32() > 0
            }
            && e.call(WORD_AT_8, &args![VATS]).u32() == 2
    };
    if close_vats {
        e.call(CLOSE_VATS_MENU, &[]);
        return;
    }

    // 007ed5e2: the special attack control.
    let mut special = 0u32;
    if e.global::<u32>(TARGET_REFERENCE) != 0 {
        let mid = key_mid as u32;
        if is_desired_attack_control_pressed(e, 1, 0, mid) {
            special = 1;
            e.set_global::<u32>(QUEUED_SPECIAL, 1);
        } else if is_desired_attack_control_pressed(e, 2, 0, mid) {
            special = 2;
            e.set_global::<u32>(QUEUED_SPECIAL, 2);
        } else if is_desired_attack_control_pressed(e, 3, 0, mid) {
            special = 3;
            e.set_global::<u32>(QUEUED_SPECIAL, 3);
        } else {
            let queued = e.global::<u32>(QUEUED_SPECIAL);
            if queued != 0 && !is_desired_attack_control_pressed(e, queued, 1, mid) {
                e.set_global::<u32>(QUEUED_SPECIAL, 0);
            }
        }
    } else {
        e.set_global::<u32>(QUEUED_SPECIAL, 0);
    }
    e.set_global::<u8>(FLAG_011DB0B2, 0);
    e.set_global::<u8>(FLAG_011DB0B3, 0);
    for (offset, cost_slot, kind) in [
        (0xa0u32, SPECIAL_ONE_COST, 2u32),
        (0xa4, SPECIAL_TWO_COST, 3),
    ] {
        let button = e.mem.u32(m + offset);
        if e.call(TILE_IS_VISIBLE, &args![button]).bool() {
            let cost = load_f32(e, cost_slot) as f64;
            let points = load_f32(e, m + 0xe0) as f64;
            // `FCOMPP; TEST AH,5; JP`: 1 unless the points are below the cost.
            let value = if points < cost { 0 } else { 1 };
            set_tile_int(e, m, offset, 0xfaf, value);
        } else if special == kind {
            special = 0;
        }
    }
    let mut cost = e
        .call(GET_WEAPON_ATTACK_ACTION_POINTS, &args![weapon, 1u32, 0u32])
        .f64() as f32;
    let mut queued_cost = quiet_f32(cost);
    match special {
        2 => cost = load_f32(e, SPECIAL_ONE_COST),
        3 => cost = load_f32(e, SPECIAL_TWO_COST),
        _ => {}
    }
    match e.global::<u32>(QUEUED_SPECIAL) {
        2 => queued_cost = load_f32(e, SPECIAL_ONE_COST),
        3 => queued_cost = load_f32(e, SPECIAL_TWO_COST),
        _ => {}
    }
    if e.call(IS_GOD_MODE, &[]).bool() {
        cost = 0.0;
        queued_cost = 0.0;
    }
    vats_menu_update_ammo(e, this);
    let guard = e.global::<u32>(STATIC_GUARD);
    if guard & 0x10 == 0 {
        e.set_global::<u32>(STATIC_GUARD, guard | 0x10);
        let id = e
            .call(ADD_USER_TRAIT, &args![TEXT_BRIGHTNESS, 0xffff_ffffu32])
            .u32();
        e.set_global::<u32>(STATIC_BRIGHTNESS_TRAIT, id);
    }
    let points = load_f32(e, m + 0xe0) as f64;
    let base_points = load_f32(e, m + 0xe4) as f64;
    // `FCOMPP; TEST AH,5; JP`: the queued meter when the points are below.
    if e.global::<u32>(TARGET_REFERENCE) != 0
        && points < queued_cost as f64
        && e.global::<u32>(QUEUED_SPECIAL) != 0
    {
        let share = (queued_cost as f64 / base_points) as f32;
        let (bar, ghost) = (e.mem.u32(m + 0x4c), e.mem.u32(m + 0x50));
        e.call(
            HUD_UPDATE_METER_GHOST,
            &args![bar, ghost, share, 0.0f32, 1u32, 0u32],
        );
    } else {
        let full = (points / base_points) as f32;
        let left = ((points - queued_cost as f64) / base_points) as f32;
        let (bar, ghost) = (e.mem.u32(m + 0x4c), e.mem.u32(m + 0x50));
        e.call(
            HUD_UPDATE_METER_GHOST,
            &args![bar, ghost, left, full, 1u32, 0u32],
        );
        set_tile_int(e, m, 0x58, 0xfa3, 0);
    }

    // 007ed9c0
    if e.global::<u32>(TARGET_REFERENCE) == 0 || special == 0 {
        // 007eea88: the target switch keys.
        if !gamepad_in_use(e)
            && e.call(CONTROL_PRESSED, &args![controls, 0x1eu32, 1u32])
                .u32()
                != 0
            && e.call(CONTROL_PRESSED, &args![controls, 0x1eu32, 3u32])
                .u32()
                != 0
            && key_high != 0x1e
        {
            e.vcall(m, 0x0c, &args![2u32, 0u32]);
            return;
        }
        if !gamepad_in_use(e)
            && e.call(CONTROL_PRESSED, &args![controls, 0x20u32, 1u32])
                .u32()
                != 0
            && e.call(CONTROL_PRESSED, &args![controls, 0x20u32, 3u32])
                .u32()
                != 0
            && key_high != 0x20
        {
            e.vcall(m, 0x0c, &args![3u32, 0u32]);
        }
        return;
    }
    let blocked =
        !e.call(GAMEPAD_IN_USE, &[]).bool() && e.call(VATS_MENU_007F6AF0, &args![m]).bool();
    let limb = e.mem.u32(m + 0x100);
    // `TEST AH,0x41; JNZ`: no chance (not above 0.0) goes to 007ee961.
    if blocked || !above(load_f32(e, limb + 0x28) as f64, double_zero(e)) {
        // 007ee961
        if blocked {
            return;
        }
        let limb = e.mem.u32(m + 0x100);
        if e.mem.u8(limb + 0x2d) == 0
            || e.call(WORD_AT_8, &args![VATS]).u32() != 2
            || e.mem.u8(m + 0xf8) == 0
        {
            return;
        }
        let manager = e.call(AUDIO_MANAGER_INSTANCE, &[]).u32();
        play_sound_then(
            e,
            GET_SOUND_HANDLE_BY_NAME,
            |_| (manager, TEXT_UI_VATS_INSUFFICIENT_AP),
            |e| {
                let limb = e.mem.u32(m + 0x100);
                // `TEST AH,0x44; JP`: the message when equal to 0.0.
                if load_f32(e, limb + 0x28) as f64 == double_zero(e) {
                    show_hud_message(e, SETTING_MESSAGE_ZERO_CHANCE);
                }
            },
        );
        return;
    }
    let attack = (gamepad_in_use(e) && e.call(WORD_AT_8, &args![VATS]).u32() == 2) || {
        !gamepad_in_use(e)
            && e.call(WORD_AT_8, &args![VATS]).u32() == 2
            && (e.mem.u8(m + 0x10e) == 0 || special != 1)
    };
    if !attack {
        // 007ee920
        if !gamepad_in_use(e)
            && e.call(WORD_AT_8, &args![VATS]).u32() == 1
            && e.mem.u8(m + 0x10e) == 0
        {
            e.call(VATS_MENU_007F6AF0, &args![m]);
        }
        return;
    }

    // 007edaaf: the attack.
    let kind = if weapon != 0 {
        e.call(WEAPON_KIND, &args![weapon]).u32()
    } else {
        0
    };
    let action = e.call(GET_ATTACK_ACTION, &args![kind]).u32();
    let shots = vats_menu_get_shot_burst_count(e, action, 0);
    let queued_clip = load_f32(e, m + 0xf0) as f64;
    // `FCOMPP; TEST AH,0x41; JP`: the reload cost when the queued clip is
    // not above the shots.
    if queued_clip <= shots as f64
        && weapon != 0
        && {
            let player = e.global::<u32>(PLAYER);
            e.call(GET_CURRENT_AMMO, &args![weapon, player]).u32() != 0
        }
        && e.call(GET_FORM_CLIP_ROUNDS, &args![weapon, 0u32]).i32() > 1
    {
        let reload = e
            .call(ACTION_POINTS_COST, &args![9u32, 1u32, 0.0f32, 0u32])
            .f64();
        cost = (reload + cost as f64) as f32;
    }
    let player = e.global::<u32>(PLAYER);
    let process = e.call(WORD_AT_68, &args![player]).u32();
    let ammo_list = e.vcall(process, 0x14c, &[]).u32();
    let enough = if weapon == 0 {
        false
    } else if e.call(WEAPON_KIND, &args![weapon]).u32() == 0xa
        || e.call(WEAPON_KIND, &args![weapon]).u32() == 0xd
    {
        if ammo_list == 0 {
            false
        } else {
            let count = e.call(NEXT_NODE, &args![ammo_list]).u32();
            let vats = e.call(IDENTITY, &args![VATS]).u32();
            count > e.call(VATS_GET_COUNT, &args![vats]).u32()
        }
    } else {
        true
    };
    let thrown = weapon != 0
        && (e.call(WEAPON_KIND, &args![weapon]).u32() == 0xa
            || e.call(WEAPON_KIND, &args![weapon]).u32() == 0xd);
    let mut fail = thrown && !enough;
    // `TEST AH,0x41; JNZ`: fails when the queued ammo is not above 0.0.
    if !fail && rock_it && !above(queued_ammo(e, m), double_zero(e)) {
        fail = true;
    }
    if !fail && action == 0x16 {
        fail = true;
    }
    // `FCOMPP; TEST AH,0x41; JP`: fails when the cost is above the points
    // or unordered.
    if !fail && !at_most(cost as f64, load_f32(e, m + 0xe0) as f64) {
        fail = true;
    }
    if !fail
        && weapon != 0
        && {
            let player = e.global::<u32>(PLAYER);
            e.call(GET_CURRENT_AMMO, &args![weapon, player]).u32() != 0
        }
        && !above(queued_ammo(e, m), double_zero(e))
    {
        fail = true;
    }
    if !fail
        && weapon != 0
        && {
            let player = e.global::<u32>(PLAYER);
            e.call(GET_CURRENT_AMMO, &args![weapon, player]).u32() != 0
        }
        && (ammo_list == 0 || e.call(NEXT_NODE, &args![ammo_list]).i32() <= 0)
    {
        fail = true;
    }
    if !fail && matches!(target_part_percent(e, this), Some(percent) if percent <= 0) {
        fail = true;
    }
    if fail {
        // 007ee652
        let zero_chance = matches!(target_part_percent(e, this), Some(percent) if percent <= 0)
            && e.call(WORD_AT_8, &args![VATS]).u32() == 2
            && e.mem.u8(m + 0xf8) != 0;
        if zero_chance {
            play_sound_then(
                e,
                GET_SOUND_HANDLE_BY_NAME_ON_AUDIO,
                |e| {
                    let audio = e.call(AUDIO_INSTANCE, &[]).u32();
                    (audio, TEXT_UI_VATS_ENTER_FAIL)
                },
                |e| show_hud_message(e, SETTING_MESSAGE_ZERO_CHANCE),
            );
        } else if cost as f64 > load_f32(e, m + 0xe0) as f64 {
            // `FCOMPP; TEST AH,0x41; JNZ`: the cost is above the points.
            let manager = e.call(AUDIO_MANAGER_INSTANCE, &[]).u32();
            play_sound_then(
                e,
                GET_SOUND_HANDLE_BY_NAME,
                |_| (manager, TEXT_UI_VATS_INSUFFICIENT_AP),
                |e| set_tile_int(e, m, 0x58, 0xfa3, 1),
            );
        } else if queued_ammo(e, m) == double_zero(e) {
            // `TEST AH,0x44; JP`: no queued ammo left (equal to 0.0).
            show_hud_message(e, SETTING_MESSAGE_NO_AMMO);
            play_sound_local(e, GET_SOUND_HANDLE_BY_NAME_ON_AUDIO, |e| {
                let audio = e.call(AUDIO_INSTANCE, &[]).u32();
                (audio, TEXT_UI_VATS_ENTER_FAIL)
            });
        }
    } else {
        do_idle_queue_attack(e, this, weapon, action, special, cost);
    }
    // 007ee907
    let menu = vats_menu(e);
    let damage = fn_007f48e0(e, Ptr::new(menu));
    let menu = vats_menu(e);
    e.mem.set_f32(menu + 0xfc, damage);
}

/// The block of `VATSMenu::DoIdle` from `007edd5d`: builds and queues the
/// `VATS_COMMAND` of the attack (see `vats_menu_do_idle`).
fn do_idle_queue_attack(
    e: &mut Engine,
    this: Ptr<VatsMenu>,
    weapon: u32,
    action: u32,
    special: u32,
    cost: f32,
) {
    let m = this.addr();
    let block = e.call(ALLOCATE, &args![0x28u32]).u32();
    let command = if block != 0 {
        e.call(VATS_COMMAND_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    // The body part data of the reference and its weapon kind are fetched
    // and not used.
    let reference = e.global::<u32>(TARGET_REFERENCE);
    if e.vcall(reference, 0x100, &[]).bool() {
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let base = e.call(BASE_FORM_OF, &args![reference]).u32();
        let body_parts = e.vcall(base, 0x180, &[]).u32();
        let index = e.global::<u32>(GLOBAL_011A59F0);
        e.call(GET_BODY_PART, &args![body_parts, index]);
    }
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let target_equipped = if e.vcall(reference, 0x100, &[]).bool() {
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let process = e.call(WORD_AT_68, &args![reference]).u32();
        e.vcall(process, 0x148, &[]).u32()
    } else {
        0
    };
    if target_equipped != 0 {
        e.call(WORD_AT_8, &args![target_equipped]);
    }
    if vats_menu_get_target_part(e, this) != 0 {
        let node = vats_menu_get_target_part(e, this);
        let node = e.call(IDENTITY, &args![node]).u32();
        let part = e.mem.u32(node);
        e.mem.set_u32(m + 0x108, part);
    }
    let mut limb_set = false;
    if weapon == 0 || e.call(CHECK_006450C0, &args![weapon]).bool() {
        let hundred = load_f32(e, FLOAT_HUNDRED).to_bits();
        let roll = e
            .call(RANDOM_FLOAT, &args![0.0f32.to_bits(), hundred])
            .f64() as f32;
        let mut count = 0i32;
        let mut last = 0xffff_ffffu32;
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let mut node = target_parts(e, this, reference);
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let limb = node_limb(e, node);
            match limb {
                1 | 2 => {
                    last = node_limb(e, node);
                    count = count.wrapping_add(1);
                }
                0 | 3..=6 => count = count.wrapping_add(1),
                _ => {}
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
        if weapon == 0 && special == 2 && last != 0xffff_ffff {
            let value = vats_menu_get_actor_value_from_body_part(e, this, last);
            e.mem.set_u32(command + 0x10, value);
            limb_set = true;
        } else if count > 0 {
            let hundred = e.global::<f64>(DOUBLE_HUNDRED);
            let share = (hundred / count as f64) as f32;
            let one = 1.0f32.to_bits();
            let bounded = e
                .call(FLOAT_MAX, &args![quiet_f32(roll).to_bits(), one])
                .f64();
            let pick = ftol(e, bounded / share as f64);
            let mut index = 0i32;
            let reference = e.global::<u32>(TARGET_REFERENCE);
            let mut node = target_parts(e, this, reference);
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let limb = node_limb(e, node) as i32;
                if (0..=6).contains(&limb) {
                    if index == pick {
                        let limb = node_limb(e, node);
                        let value = vats_menu_get_actor_value_from_body_part(e, this, limb);
                        e.mem.set_u32(command + 0x10, value);
                        limb_set = true;
                    }
                    index = index.wrapping_add(1);
                }
                node = e.call(NEXT_NODE, &args![node]).u32();
            }
        }
    }
    if !limb_set {
        let limb = current_limb_of(e, m);
        let value = vats_menu_get_actor_value_from_body_part(e, this, limb);
        e.mem.set_u32(command + 0x10, value);
    }
    let shots = vats_menu_get_shot_burst_count(e, action, 0);
    e.mem.set_u8(command + 0x8, shots);
    let all_shots = vats_menu_get_shot_burst_count(e, action, 1);
    let difference = (all_shots as i32).wrapping_sub(e.mem.u8(command + 0x8) as i32);
    let fake = e.call(INT_MAX, &args![difference, 0u32]).u8();
    e.mem.set_u8(command + 0x9, fake);
    let command_action = if special == 2 {
        // `TEST AH,0x41; JNZ`: the weapon's own special when its `+0x37C`
        // float is above 0.0.
        if weapon != 0 && fn_007ebd10(e, weapon) as f64 > double_zero(e) {
            0x10
        } else {
            let reference = e.global::<u32>(TARGET_REFERENCE);
            if e.vcall(reference, 0x230, &[]).bool() {
                0x15
            } else {
                0x11
            }
        }
    } else if special == 3 {
        0x14
    } else {
        action
    };
    e.mem.set_u32(command, command_action);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    e.mem.set_u32(command + 0xc, reference);
    e.mem.set_f32(command + 0x20, quiet_f32(cost));
    let palm = (setting_float(e, SETTING_PARALYZE_PALM_CHANCE) as f64
        * e.global::<f64>(DOUBLE_HUNDRED)) as f32;
    let hundred = load_f32(e, FLOAT_HUNDRED).to_bits();
    let roll = e
        .call(RANDOM_FLOAT, &args![0.0f32.to_bits(), hundred])
        .f64() as f32;
    // `FCOMPP; TEST AH,1; JNZ`: the palm when its chance is not below the roll.
    if e.global::<u8>(FLAG_011DB0B0) != 0 && action == 0 && palm as f64 >= roll as f64 {
        e.mem.set_u8(command + 0x7, 1);
    }
    let reference = e.global::<u32>(TARGET_REFERENCE);
    if e.call(VATS_IS_MELEE_COMMAND, &args![VATS, command]).bool()
        && e.vcall(reference, 0x100, &[]).bool()
        && !limb_set
    {
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let base = e.call(BASE_FORM_OF, &args![reference]).u32();
        let body_parts = e.vcall(base, 0x180, &[]).u32();
        if e.call(GET_BODY_PART, &args![body_parts, 0x19u32]).u32() != 0 {
            e.mem.set_u32(command + 0x10, 0x19);
        } else if e.call(GET_BODY_PART, &args![body_parts, 0x1au32]).u32() != 0 {
            e.mem.set_u32(command + 0x10, 0x1a);
        } else {
            let parts = e.call(BODY_PART_ARRAY, &args![body_parts]).u32();
            let mut found = false;
            let mut index = 0u32;
            while index < 0xf && !found {
                let part = e.mem.u32(parts + 4 * index);
                if part != 0 {
                    let value = e.call(BODY_PART_VALUE, &args![part]).u8() as i8 as i32;
                    e.mem.set_i32(command + 0x10, value);
                    found = true;
                }
                index += 1;
            }
        }
    }
    e.call(VATS_QUEUE_COMMAND, &args![VATS, command]);
    let mut chance = match vats_menu_get_target_part(e, this) {
        0 => 0.0f32,
        _ => {
            let node = vats_menu_get_target_part(e, this);
            let node = e.call(IDENTITY, &args![node]).u32();
            let part = e.mem.u32(node);
            load_f32(e, part + 0x28)
        }
    };
    chance = quiet_f32(chance);
    if e.mem.u8(m + 0x104) != 0 {
        let hits = fn_007ef930(e, this);
        let address = e
            .call(SETTING_INT_ADDRESS, &args![SETTING_CONCENTRATED_FIRE_BONUS])
            .u32();
        let bonus = e.mem.i32(address).wrapping_mul(hits);
        chance = (bonus as f64 / e.global::<f64>(DOUBLE_HUNDRED) + chance as f64) as f32;
    }
    let always_hit = {
        let address = e
            .call(SETTING_BOOL_ADDRESS, &args![SETTING_VATS_ALWAYS_HIT])
            .u32();
        e.mem.u8(address) != 0
    } && vats_menu_get_target_part(e, this) != 0;
    let success = if always_hit {
        1
    } else {
        e.call(ROLL_CHANCE, &args![chance]).u8()
    };
    e.mem.set_u8(command + 0x4, success);
    fn_007f1840(e, this);
    let fire = vats_menu_update_ammo(e, this)
        && weapon != 0
        && e.call(WEAPON_KIND, &args![weapon]).u32() != 0xa
        && e.call(WEAPON_KIND, &args![weapon]).u32() != 0xd
        && {
            let rounds = e.call(GET_FORM_CLIP_ROUNDS, &args![weapon, 0u32]).u32();
            rounds != e.call(WEAPON_102_524B60, &args![weapon]).u8() as u32
        }
        && queued_ammo(e, m) > double_zero(e);
    let not_first = u8::from(special != 1);
    vats_menu_update_queued_action_list(e, this, command, u8::from(fire), not_first);
    // `FCOMPP; TEST AH,0x41; JNZ`: the warning when the cost is above the
    // points.
    if cost as f64 > load_f32(e, m + 0xe0) as f64 {
        set_tile_int(e, m, 0x44, 0xfaf, 0);
    }
    fn_007f0ea0(e, this);
}

/// `eLimb` of `pCurrentLimb` of the menu at `menu`.
fn current_limb_of(e: &Engine, menu: u32) -> u32 {
    let limb = e.mem.u32(menu + 0x100);
    e.mem.u32(limb + 0x20)
}

// Translated from 007eeb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IsDesiredAttackControlPressed` (Xbox PDB), cdecl (attack, held, key):
/// whether the control of attack 1 (the normal attack), 2 or 3 (the
/// special attacks) is pressed (`held` false) or held (`held` true; the
/// controls get state `held == 0`). Without the gamepad: attack 1 asks
/// `00a23a50(0, state)` and needs `key`; attacks 2 and 3 ask control
/// `0x13` or `0x21` or the flags `011db0b2`/`011db0b3`. With the gamepad:
/// menu button `0xE` (`0xC` for 2, `0xB` for 3) through
/// `InterfaceManager::CheckMenuButton`: 2 when pressed, 0 when held.
pub fn is_desired_attack_control_pressed(e: &mut Engine, attack: u32, held: u8, key: u32) -> bool {
    if !gamepad_in_use(e) {
        let holder = e.global::<u32>(CONTROLS_HOLDER);
        let controls = e.call(CONTROLS, &args![holder]).u32();
        let state = u32::from(held == 0);
        match attack {
            1 => e.call(BUTTON_STATE, &args![controls, 0u32, state]).u32() != 0 && key != 0,
            2 => {
                e.call(CONTROL_PRESSED, &args![controls, 0x13u32, state])
                    .u32()
                    != 0
                    || e.global::<u8>(FLAG_011DB0B2) != 0
            }
            3 => {
                e.call(CONTROL_PRESSED, &args![controls, 0x21u32, state])
                    .u32()
                    != 0
                    || e.global::<u8>(FLAG_011DB0B3) != 0
            }
            _ => false,
        }
    } else {
        let button = match attack {
            2 => 0xcu32,
            3 => 0xb,
            _ => 0xe,
        };
        let result = e.call(CHECK_MENU_BUTTON, &args![button, 1u32]).i32();
        if held != 0 {
            result == 0
        } else {
            result == 2
        }
    }
}

/// Fetches the sound `name` through the audio object (`BSAudio::QInstance`
/// then `00ad7550`, flags `0x121`) into a temporary, assigns it to the
/// sound handle `handle`, destroys the temporary and plays the handle.
fn play_into_handle(e: &mut Engine, handle: u32, name: u32) {
    e.with_stack(0xc, |e, temporary| {
        let temporary = temporary.addr();
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        let fetched = e
            .call(
                GET_SOUND_HANDLE_BY_NAME_ON_AUDIO,
                &args![audio, temporary, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, fetched]);
        e.call(EMPTY_MEMBER, &args![temporary]);
    });
    e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
}

/// `pCurrentLimb` of the open menu for the target at `node`: the part
/// `FindTargetPart(0x1a)` finds when `type_node`'s target is an NPC (type
/// 1), the first part of `parts` otherwise.
fn select_target_limb(e: &mut Engine, this: Ptr<VatsMenu>, type_node: u32, parts: u32) {
    let found = vats_menu_find_target_part(e, this, 0x1a);
    let node = e.call(IDENTITY, &args![type_node]).u32();
    let item = e.mem.u32(node);
    let source = if e.mem.u32(item + 4) == 1 && found != 0 {
        found
    } else {
        parts
    };
    let source = e.call(IDENTITY, &args![source]).u32();
    let part = e.mem.u32(source);
    let menu = vats_menu(e);
    e.mem.set_u32(menu + 0x100, part);
}

/// `pTarget` of the item of the target list node `node`.
fn node_reference(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(IDENTITY, &args![node]).u32();
    e.mem.u32(e.mem.u32(node))
}

/// The part list of the item of the target list node `node` (`item + 8`).
fn node_parts(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(IDENTITY, &args![node]).u32();
    e.mem.u32(node).wrapping_add(8)
}

/// The target switch of `DoClick` (ids 2 and 3): plays
/// `UIVATSSelectTarget`, removes the meters of every target's parts (fade
/// control and trait `0xfa9`), picks the previous (`forward` false) or the
/// next target of the list at `011db150` as the code walks it, runs
/// `007f3bd0(0)`, clears the parts' `bChanceCalculated`/`bFirstTimeShown`
/// when the target's chances are not calculated and builds its meters when
/// its first part has none. False when the list has at most one target
/// (`UIVATSEnterFail` is played instead).
fn do_click_switch_target(e: &mut Engine, this: Ptr<VatsMenu>, handle: u32, forward: bool) -> bool {
    let m = this.addr();
    if e.call(VATS_GET_COUNT, &args![LIST_011DB150]).u32() <= 1 {
        play_into_handle(e, handle, TEXT_UI_VATS_ENTER_FAIL);
        return false;
    }
    play_into_handle(e, handle, TEXT_UI_VATS_SELECT_TARGET);
    let mut node = LIST_011DB150;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let mut parts = node_parts(e, node);
        while parts != 0 && !e.call(LIST_IS_EMPTY, &args![parts]).bool() && {
            let item = e.call(IDENTITY, &args![parts]).u32();
            e.mem.u32(e.mem.u32(item) + 0x30) != 0
        } {
            let item = e.call(IDENTITY, &args![parts]).u32();
            let meter = e.mem.u32(e.mem.u32(item) + 0x30);
            e.call(REMOVE_FADE_CONTROL, &args![meter, 0xfa9u32]);
            let item = e.call(IDENTITY, &args![parts]).u32();
            let meter = e.mem.u32(e.mem.u32(item) + 0x30);
            e.call(TILE_SET_INT, &args![meter, 0xfa9u32, 0u32]);
            parts = e.call(NEXT_NODE, &args![parts]).u32();
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    let mut node = LIST_011DB150;
    while node != 0 {
        if forward {
            // 007ef352: the target after the current one, or the first.
            if e.global::<u32>(TARGET_REFERENCE) == node_reference(e, node) {
                let next = e.call(NEXT_NODE, &args![node]).u32();
                let usable = next != 0
                    && {
                        let next = e.call(NEXT_NODE, &args![node]).u32();
                        node_reference(e, next) != 0
                    }
                    && {
                        let next = e.call(NEXT_NODE, &args![node]).u32();
                        let reference = node_reference(e, next);
                        e.vcall(reference, 0x1d0, &[]).u32() != 0
                    };
                if usable {
                    let next = e.call(NEXT_NODE, &args![node]).u32();
                    let reference = node_reference(e, next);
                    e.set_global::<u32>(TARGET_REFERENCE, reference);
                    let next = e.call(NEXT_NODE, &args![node]).u32();
                    let parts = node_parts(e, next);
                    select_target_limb(e, this, node, parts);
                    break;
                }
                let reference = node_reference(e, LIST_011DB150);
                e.set_global::<u32>(TARGET_REFERENCE, reference);
                let found = vats_menu_find_target_part(e, this, 0x1a);
                let item_node = e.call(IDENTITY, &args![node]).u32();
                let item = e.mem.u32(item_node);
                let part = if e.mem.u32(item + 4) == 1 && found != 0 {
                    let found = e.call(IDENTITY, &args![found]).u32();
                    e.mem.u32(found)
                } else {
                    let parts = node_parts(e, LIST_011DB150);
                    let parts = e.call(IDENTITY, &args![parts]).u32();
                    e.mem.u32(parts)
                };
                let menu = vats_menu(e);
                e.mem.set_u32(menu + 0x100, part);
            }
        } else {
            // 007eef94: the target before the current one, or the last.
            let reference = node_reference(e, node);
            let usable = reference != 0
                && {
                    let reference = node_reference(e, node);
                    e.vcall(reference, 0x1d0, &[]).u32() != 0
                }
                && e.call(NEXT_NODE, &args![node]).u32() != 0
                && {
                    let next = e.call(NEXT_NODE, &args![node]).u32();
                    node_reference(e, next) == e.global::<u32>(TARGET_REFERENCE)
                };
            if usable {
                let reference = node_reference(e, node);
                e.set_global::<u32>(TARGET_REFERENCE, reference);
                let parts = node_parts(e, node);
                select_target_limb(e, this, node, parts);
                break;
            }
            if e.call(NEXT_NODE, &args![node]).u32() == 0 {
                let reference = node_reference(e, node);
                e.set_global::<u32>(TARGET_REFERENCE, reference);
                let found = vats_menu_find_target_part(e, this, 0x1a);
                let item_node = e.call(IDENTITY, &args![node]).u32();
                let item = e.mem.u32(item_node);
                let part = if e.mem.u32(item + 4) == 1 && found != 0 {
                    let found = e.call(IDENTITY, &args![found]).u32();
                    e.mem.u32(found)
                } else {
                    let parts = node_parts(e, node);
                    let parts = e.call(IDENTITY, &args![parts]).u32();
                    e.mem.u32(parts)
                };
                let menu = vats_menu(e);
                e.mem.set_u32(menu + 0x100, part);
            }
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    fn_007f3bd0(e, 0);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let target = vats_menu_get_vats_target(e, this, reference);
    let target_node = e.call(IDENTITY, &args![target]).u32();
    if e.mem.u8(e.mem.u32(target_node) + 0x10) == 0 {
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let mut parts = target_parts(e, this, reference);
        while parts != 0 && !e.call(LIST_IS_EMPTY, &args![parts]).bool() {
            let item = e.call(IDENTITY, &args![parts]).u32();
            let part = e.mem.u32(item);
            e.mem.set_u8(part + 0x2d, 0);
            let item = e.call(IDENTITY, &args![parts]).u32();
            let part = e.mem.u32(item);
            e.mem.set_u8(part + 0x2e, 0);
            parts = e.call(NEXT_NODE, &args![parts]).u32();
        }
    }
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let parts = target_parts(e, this, reference);
    let first = e.call(IDENTITY, &args![parts]).u32();
    if e.mem.u32(e.mem.u32(first) + 0x30) == 0 {
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let target = vats_menu_get_vats_target(e, this, reference);
        e.call(BUILD_METERS_FOR_TARGET, &args![m, target]);
    }
    true
}

// Translated from 007eecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::DoClick` (Xbox PDB): (tile id, tile). A held stick (`+0x10F`
/// or `+0x110`) whose time is up (`007f4bb0`/`007f4c00`) turns into
/// `007f4c50(1)`/`007f4c50(0)` and ends the click; the flag is cleared
/// either way. With the gamepad and no `bTargetStickReset`, nothing else
/// happens. Ids: 2 and 3 switch to the previous or next target, 4 closes
/// the menu, 5 clears the queue (`UpdateQueuedActionList(0, 0, 0)` in mode
/// 2 with `bHeadingReached`), `0x1e`/`0x1f` set the special attack flags
/// `011db0b2`/`011db0b3`; all of them only while not closing. Then
/// `011a59f0` = 0x19 for an actor target, the special buttons are set up
/// for the equipped weapon (`007eb920`), `007f3070` runs and
/// `009c9ae0(target)` when the VATS object answers `005d8710`.
// Translation incomplete: 007eecb0 - the C++ exception-unwinding frame (FS:[0]) and the security cookie are not translated
pub fn vats_menu_do_click(e: &mut Engine, this: Ptr<VatsMenu>, id: u32, _unused_2: u32) {
    let m = this.addr();
    let player = e.global::<u32>(PLAYER);
    let process = e.call(WORD_AT_68, &args![player]).u32();
    let equipped = e.vcall(process, 0x148, &[]).u32();
    let weapon = if equipped != 0 {
        e.call(WORD_AT_8, &args![equipped]).u32()
    } else {
        0
    };
    if e.mem.u8(m + 0x10f) != 0 {
        if fn_007f4bb0(e) {
            fn_007f4c50(e, this, 1);
            e.mem.set_u8(m + 0x10f, 0);
            return;
        }
        e.mem.set_u8(m + 0x10f, 0);
    }
    if e.mem.u8(m + 0x110) != 0 {
        if fn_007f4c00(e) {
            fn_007f4c50(e, this, 0);
            e.mem.set_u8(m + 0x110, 0);
            return;
        }
        e.mem.set_u8(m + 0x110, 0);
    }
    e.with_stack(0xc, |e, handle| {
        let handle = handle.addr();
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
        if gamepad_in_use(e) && e.mem.u8(m + 0x10d) == 0 {
            e.call(EMPTY_MEMBER, &args![handle]);
            return;
        }
        let closing = e.mem.u8(m + 0xfb) != 0;
        match id {
            2 | 3 => {
                if !do_click_switch_target(e, this, handle, id == 3) {
                    e.call(EMPTY_MEMBER, &args![handle]);
                    return;
                }
            }
            4 if !closing => {
                e.call(CLOSE_VATS_MENU, &[]);
            }
            5 if !closing
                && e.call(WORD_AT_8, &args![VATS]).u32() == 2
                && e.mem.u8(m + 0xf8) != 0 =>
            {
                vats_menu_update_queued_action_list(e, this, 0, 0, 0);
                let menu = vats_menu(e);
                let damage = fn_007f48e0(e, Ptr::new(menu));
                let menu = vats_menu(e);
                e.mem.set_f32(menu + 0xfc, damage);
            }
            0x1e if !closing => e.set_global::<u8>(FLAG_011DB0B2, 1),
            0x1f if !closing => e.set_global::<u8>(FLAG_011DB0B3, 1),
            _ => {}
        }
        let reference = e.global::<u32>(TARGET_REFERENCE);
        if reference != 0 && e.vcall(reference, 0x100, &[]).bool() {
            e.set_global::<u32>(GLOBAL_011A59F0, 0x19);
        }
        fn_007eb920(e, weapon);
        fn_007f3070(e, this);
        if e.call(VATS_005D8710, &args![VATS]).u32() != 0 {
            let reference = e.global::<u32>(TARGET_REFERENCE);
            if reference != 0 {
                e.call(VATS_009C9AE0, &args![VATS, reference]);
            }
        }
        e.call(EMPTY_MEMBER, &args![handle]);
    });
}

// Translated from 007ef710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::DoKey` (Xbox PDB): (key). Keys `0x42` and `0x62` clear the
/// queued actions (`UpdateQueuedActionList(0, 0, 0)`) in mode 2 with
/// `bHeadingReached` and answer true; other keys answer false.
pub fn vats_menu_do_key(e: &mut Engine, this: Ptr<VatsMenu>, key: u32) -> bool {
    if key != 0x42 && key != 0x62 {
        return false;
    }
    if e.call(WORD_AT_8, &args![VATS]).u32() == 2 && e.mem.u8(this.addr() + 0xf8) != 0 {
        vats_menu_update_queued_action_list(e, this, 0, 0, 0);
    }
    true
}

// Translated from 007ef760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::DoWheelMove` (Xbox PDB): (two words it does not read). The
/// wheel value of `0070ec70` (on `004b7210`'s object) divided by -120
/// (`IDIV`): 1 clicks id 2 (`DoClick`, slot `0x0C`), anything else id 3.
pub fn vats_menu_do_wheel_move(
    e: &mut Engine,
    this: Ptr<VatsMenu>,
    _unused_1: u32,
    _unused_2: u32,
) {
    let object = e.call(OBJECT_004B7210, &[]).u32();
    let wheel = e.call(WHEEL_VALUE, &args![object]).i32();
    let id = if wheel.wrapping_div(-120) == 1 {
        2u32
    } else {
        3
    };
    e.vcall(this.addr(), 0x0c, &args![id, 0u32]);
}

// Translated from 007ef7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: (tile id, tile): without the gamepad, ids 2 to 5,
/// `0x1e` and `0x1f` set `bMouseOverButton` (the code's jump table).
pub fn fn_007ef7b0(e: &mut Engine, this: Ptr<VatsMenu>, id: u32, _unused_2: u32) {
    if gamepad_in_use(e) {
        return;
    }
    if matches!(id, 2..=5 | 0x1e | 0x1f) {
        e.mem.set_u8(this.addr() + 0x10e, 1);
    }
}

// Translated from 007ef830 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: (tile id, tile): without the gamepad, ids 2 to 5,
/// `0x1e` and `0x1f` clear `bMouseOverButton` (the code's jump table).
pub fn fn_007ef830(e: &mut Engine, this: Ptr<VatsMenu>, id: u32, _unused_2: u32) {
    if gamepad_in_use(e) {
        return;
    }
    if matches!(id, 2..=5 | 0x1e | 0x1f) {
        e.mem.set_u8(this.addr() + 0x10e, 0);
    }
}

// Translated from 007ef8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::DoStillDown` (Xbox PDB): (tile id, tile): id 2 starts
/// `bSelectLeftHeld`, id 3 `bSelectRightHeld`, each with `uiSelectTimer` =
/// the timer's milliseconds, unless already held.
pub fn vats_menu_do_still_down(e: &mut Engine, this: Ptr<VatsMenu>, id: u32, _unused_2: u32) {
    let m = this.addr();
    let flag = match id {
        2 => 0x10f,
        3 => 0x110,
        _ => return,
    };
    if e.mem.u8(m + flag) == 0 {
        let time = e.call(TIMER_MILLISECONDS, &args![TIMER]).u32();
        e.mem.set_u32(m + 0x114, time);
        e.mem.set_u8(m + flag, 1);
    }
}

// Translated from 007ef930 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: how many queued commands in a row (the VATS
/// object's list, from its start) repeat the target (`+0xC`) and limb
/// (`+0x10`) of the one before them; the count restarts at 0 on a
/// different command and is answered at the last node.
pub fn fn_007ef930(e: &mut Engine, _this: Ptr<VatsMenu>) -> i32 {
    let mut count = 0i32;
    let mut node = e.call(IDENTITY, &args![VATS]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let has_next = e.call(NEXT_NODE, &args![node]).u32() != 0;
        let next_item = if has_next {
            let next = e.call(NEXT_NODE, &args![node]).u32();
            let next = e.call(IDENTITY, &args![next]).u32();
            e.mem.u32(next)
        } else {
            0
        };
        let item_node = e.call(IDENTITY, &args![node]).u32();
        let item = e.mem.u32(item_node);
        if !has_next {
            return count;
        }
        if e.mem.u32(item + 0xc) == e.mem.u32(next_item + 0xc)
            && e.mem.u32(item + 0x10) == e.mem.u32(next_item + 0x10)
        {
            count = count.wrapping_add(1);
        } else {
            count = 0;
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    count
}

/// `strcpy_s(buffer, 0x104, text)` (`00406d30`).
fn copy_text(e: &mut Engine, buffer: u32, text: u32) {
    e.call(STRCPY_S, &args![buffer, 0x104u32, text]);
}

/// `strcat_s(buffer, 0x104, text)` (`00406d50`).
fn append_text(e: &mut Engine, buffer: u32, text: u32) {
    e.call(STRCAT_S, &args![buffer, 0x104u32, text]);
}

/// Adds an item with id `011db198` to the action list box (`+0xB0`,
/// `ListBox<unsigned int>::AddItem(id, 0, 0, 0)`) and sets its text.
fn add_action_item(e: &mut Engine, this: u32, text: u32) {
    let id = e.global::<u32>(ACTION_ITEM_COUNTER);
    let item = e
        .call(LIST_BOX_ADD_ITEM, &args![this + 0xb0, id, 0u32, 0u32, 0u32])
        .u32();
    e.call(TILE_SET_STRING, &args![item, 0xfc4u32, text, 1u32]);
}

// Translated from 007efa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::UpdateQueuedActionList` (Xbox PDB): (command, with reload,
/// with special name).
///
/// Without a command it removes the last queued action: `pLastLimb`
/// cleared, the VATS object drops its last command (`009c88a0`, which
/// answers its action, preset `0x16`) whose action points (`+0x20` of
/// `009c71f0`'s command) go back to `fActionPoints` (unless god mode),
/// capped at `fActionPointsBase`; the menu closes when nothing was queued
/// in mode 2; the last list item goes (and the one before when the dropped
/// action was not a reload and the last item reads the reload text), and
/// `UIMenuCancel` plays.
///
/// With a command it adds it: `UIVATSMove` plays, its action points are
/// taken, and a list item reads the action's name (`0066ddc0`) or, for an
/// attack, the target's name with the part (body part data `006838b0`, or
/// `---`), the special attack's name, and a reload item before it.
/// Finally trait `0xfaf` of `pTiles[4]` (`+0x38`) says whether the list
/// has items, except when the attacked part is missing (the function ends
/// there).
// Translation incomplete: 007efa10 - the C++ exception-unwinding frame (FS:[0]) and the security cookie (00ec408c) are not translated
pub fn vats_menu_update_queued_action_list(
    e: &mut Engine,
    this: Ptr<VatsMenu>,
    command: u32,
    with_reload: u8,
    with_special: u8,
) {
    let m = this.addr();
    if command == 0 {
        e.mem.set_u32(m + 0x108, 0);
        let refund = if e.call(VATS_LAST_COMMAND, &args![VATS]).u32() != 0 {
            let last = e.call(VATS_LAST_COMMAND, &args![VATS]).u32();
            load_f32(e, last + 0x20)
        } else {
            0.0
        };

        let action = e.with_stack(8, |e, locals| {
            let action = locals.addr();
            let flag = action + 4;
            e.mem.set_u32(action, 0x16);
            e.mem.set_u8(flag, 0);
            e.call(VATS_REMOVE_LAST_COMMAND, &args![VATS, action, flag]);
            e.mem.u32(action)
        });
        fn_007f1840(e, this);
        if action == 0x16 && e.call(WORD_AT_8, &args![VATS]).u32() == 2 {
            e.call(CLOSE_VATS_MENU, &[]);
        }
        if !e.call(IS_GOD_MODE, &[]).bool() {
            let player = e.global::<u32>(PLAYER);
            let process = e.call(WORD_AT_68, &args![player]).u32();
            let equipped = e.vcall(process, 0x148, &[]).u32();
            if equipped != 0 {
                e.call(WORD_AT_8, &args![equipped]);
            }
            let points = (load_f32(e, m + 0xe0) as f64 + refund as f64) as f32;
            e.mem.set_f32(m + 0xe0, points);
        }
        // `FCOMPP; TEST AH,5; JP`: capped when the base is below.
        if (load_f32(e, m + 0xe4) as f64) < load_f32(e, m + 0xe0) as f64 {
            let base = load_f32(e, m + 0xe4);
            e.mem.set_f32(m + 0xe0, base);
        }
        set_tile_int(e, m, 0x44, 0xfaf, 1);
        if action != 9 {
            let last = e.call(LIST_BOX_GET_LAST_ITEM, &args![m + 0xb0]).u32();
            if last != 0 {
                let text = e.call(TILE_GET_STRING, &args![last, 0xfc4u32]).u32();
                let reload = e.call(ACTION_NAME, &args![9u32]).u32();
                if e.call(STRCMP, &args![reload, text]).i32() == 0 {
                    let count = e.call(LIST_COUNT, &args![m + 0xb0]).u32();
                    e.call(
                        LIST_BOX_007F7280,
                        &args![m + 0xb0, count.wrapping_sub(1), 0u32],
                    );
                }
            }
        }
        let count = e.call(LIST_COUNT, &args![m + 0xb0]).u32();
        e.call(
            LIST_BOX_007F7280,
            &args![m + 0xb0, count.wrapping_sub(1), 0u32],
        );
        play_sound_local(e, GET_SOUND_HANDLE_BY_NAME_ON_AUDIO, |e| {
            let audio = e.call(AUDIO_INSTANCE, &[]).u32();
            (audio, TEXT_UI_MENU_CANCEL)
        });
    } else {
        let manager = e.call(AUDIO_MANAGER_INSTANCE, &[]).u32();
        let complete = e.with_stack(0xc + 3 * 0x104, |e, block| {
            let handle = block.addr();
            let text = handle + 0xc;
            let reload_text = text + 0x104;
            let item_text = reload_text + 0x104;
            e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
            e.with_stack(0xc, |e, temporary| {
                let temporary = temporary.addr();
                let fetched = e
                    .call(
                        GET_SOUND_HANDLE_BY_NAME,
                        &args![manager, temporary, TEXT_UI_VATS_MOVE, 0x121u32],
                    )
                    .u32();
                e.call(SOUND_HANDLE_ASSIGN, &args![handle, fetched]);
                e.call(EMPTY_MEMBER, &args![temporary]);
            });
            e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
            let menu = vats_menu(e);
            let reference = e.global::<u32>(TARGET_REFERENCE);
            let target = vats_menu_get_vats_target(e, Ptr::new(menu), reference);
            let player = e.global::<u32>(PLAYER);
            let process = e.call(WORD_AT_68, &args![player]).u32();
            let equipped = e.vcall(process, 0x148, &[]).u32();
            let weapon = if equipped != 0 {
                e.call(WORD_AT_8, &args![equipped]).u32()
            } else {
                0
            };
            let points = (load_f32(e, m + 0xe0) as f64 - load_f32(e, command + 0x20) as f64) as f32;
            e.mem.set_f32(m + 0xe0, points);
            let action = e.mem.u32(command);
            if !e.call(IS_ATTACK_ACTION, &args![action]).bool() {
                let counter = e.global::<u32>(ACTION_ITEM_COUNTER);
                e.set_global::<u32>(ACTION_ITEM_COUNTER, counter.wrapping_add(1));
                let action = e.mem.u32(command);
                let name = e.call(ACTION_NAME, &args![action]).u32();
                copy_text(e, text, name);
            } else {
                let counter = e.global::<u32>(ACTION_ITEM_COUNTER);
                e.set_global::<u32>(ACTION_ITEM_COUNTER, counter.wrapping_add(1));
                let unsuitable = |e: &mut Engine| {
                    e.call(CHECK_006450C0, &args![weapon]).bool()
                        || e.call(WEAPON_KIND, &args![weapon]).u32() == 0xa
                };
                let by_body_part = e.global::<i32>(GLOBAL_011A59F0) != -1
                    && {
                        let reference = e.global::<u32>(TARGET_REFERENCE);
                        e.vcall(reference, 0x100, &[]).bool()
                    }
                    && weapon != 0
                    && !unsuitable(e);
                if by_body_part {
                    let reference = e.global::<u32>(TARGET_REFERENCE);
                    let base = e.call(BASE_FORM_OF, &args![reference]).u32();
                    let body_parts = e.vcall(base, 0x180, &[]).u32();
                    let limb = current_limb_of(e, m);
                    let value = vats_menu_get_actor_value_from_body_part(e, this, limb);
                    let part = e.call(GET_BODY_PART, &args![body_parts, value]).u32();
                    if part == 0 {
                        e.call(EMPTY_MEMBER, &args![handle]);
                        return false;
                    }
                    let target_reference = e.mem.u32(command + 0xc);
                    let name = e.call(REFERENCE_NAME, &args![target_reference]).u32();
                    copy_text(e, text, name);
                    append_text(e, text, TEXT_COLON);
                    if e.call(BODY_PART_NAME, &args![part]).u32() != 0 {
                        let part_name = e.call(BODY_PART_NAME, &args![part]).u32();
                        append_text(e, text, part_name);
                    } else {
                        append_text(e, text, TEXT_DASHES);
                    }
                } else {
                    let node = e.call(IDENTITY, &args![target]).u32();
                    let item = e.mem.u32(node);
                    if e.mem.u32(item + 4) == 2 {
                        let target_reference = e.mem.u32(command + 0xc);
                        let name = e.call(REFERENCE_NAME, &args![target_reference]).u32();
                        copy_text(e, text, name);
                    } else {
                        let reference = e.global::<u32>(TARGET_REFERENCE);
                        let process = e.call(WORD_AT_68, &args![reference]).u32();
                        let reference = e.global::<u32>(TARGET_REFERENCE);
                        let key = e.vcall(reference, 0x1e8, &[]).u32();
                        let entry = e.vcall(process, 0x190, &args![key]).u32();
                        if entry == 0 || weapon == 0 || unsuitable(e) {
                            let target_reference = e.mem.u32(command + 0xc);
                            let name = e.call(REFERENCE_NAME, &args![target_reference]).u32();
                            copy_text(e, text, name);
                        } else {
                            let target_reference = e.mem.u32(command + 0xc);
                            let name = e.call(REFERENCE_NAME, &args![target_reference]).u32();
                            copy_text(e, text, name);
                            append_text(e, text, TEXT_COLON);
                            let object = e.call(ENTRY_OBJECT_413F40, &args![entry]).u32();
                            if e.call(OBJECT_CHECK_43B1B0, &args![object]).u32() != 0 {
                                let part_name = fn_007f0360(e, 2);
                                append_text(e, text, part_name);
                            } else {
                                append_text(e, text, TEXT_DASHES);
                            }
                        }
                    }
                }
                if with_special != 0 {
                    if weapon != 0 && e.call(WORD_AT_15C, &args![weapon]).u32() == 0x26 {
                        append_text(e, text, TEXT_COLON);
                        let special = e.call(FORM_WORD_368, &args![weapon]).u32();
                        append_text(e, text, special);
                    } else {
                        let setting = match e.mem.u32(command) {
                            0x11 => Some(SETTING_UNARMED_ONE),
                            0x15 => Some(SETTING_UNARMED_GROUND),
                            0x14 => Some(SETTING_UNARMED_TWO),
                            _ => None,
                        };
                        if let Some(setting) = setting {
                            append_text(e, text, TEXT_COLON);
                            let special = e.call(SETTING_STRING, &args![setting]).u32();
                            append_text(e, text, special);
                        }
                    }
                }
                if with_reload != 0 {
                    copy_text(e, reload_text, text);
                    add_action_item(e, m, reload_text);
                    let counter = e.global::<u32>(ACTION_ITEM_COUNTER);
                    e.set_global::<u32>(ACTION_ITEM_COUNTER, counter.wrapping_add(1));
                    let reload = e.call(ACTION_NAME, &args![9u32]).u32();
                    copy_text(e, text, reload);
                }
            }
            copy_text(e, item_text, text);
            add_action_item(e, m, item_text);
            e.call(EMPTY_MEMBER, &args![handle]);
            true
        });
        if !complete {
            return;
        }
    }
    let value = u32::from(e.call(LIST_COUNT, &args![m + 0xb0]).u32() != 0);
    set_tile_int(e, m, 0x38, 0xfaf, value);
}

// Translated from 007f0360 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (index): the string of the setting `index` of the table at
/// `01183aa0` (`00403df0`).
pub fn fn_007f0360(e: &mut Engine, index: u32) -> u32 {
    let setting = e.mem.u32(SETTING_TABLE.wrapping_add(index.wrapping_mul(4)));
    e.call(SETTING_STRING, &args![setting]).u32()
}

/// The heading from the player's position (slot `0x1F4`, copied) to the
/// position of the reference at `*target` (`HeadingBetweenLocations`),
/// for each of `first` and `second`, as floats.
fn headings_from_player(e: &mut Engine, first: u32, second: u32) -> (f32, f32) {
    e.with_stack(0xc, |e, local| {
        let local = local.addr();
        let player = e.global::<u32>(PLAYER);
        let position = e.vcall(player, 0x1f4, &[]).u32();
        for word in 0..3 {
            let value = e.mem.u32(position + 4 * word);
            e.mem.set_u32(local + 4 * word, value);
        }
        let mut headings = [0.0f32; 2];
        for (index, target) in [first, second].into_iter().enumerate() {
            let reference = e.mem.u32(target);
            let other = e.vcall(reference, 0x1f4, &[]).u32();
            headings[index] = e
                .call(HEADING_BETWEEN_LOCATIONS, &args![local, other])
                .f64() as f32;
        }
        (headings[0], headings[1])
    })
}

// Translated from 007f0380 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (first, second): orders two targets by how far their heading from
/// the player is from the player's facing (slot `0x2BC(0)` times
/// `57.2957763671875`): the absolute difference (`00408840`) is folded
/// to `360 - d` above 180 (`TEST AH,0x41`) and made absolute
/// again. -1 when the second is farther, 1 when nearer, 0 otherwise
/// (equal or unordered).
///
/// The player's heading is read before the targets'; the three player calls
/// (`0x1F4`, `0x2BC`) come first as in the code.
pub fn fn_007f0380(e: &mut Engine, first: u32, second: u32) -> i32 {
    e.with_stack(0xc, |e, local| {
        let local = local.addr();
        let player = e.global::<u32>(PLAYER);
        let position = e.vcall(player, 0x1f4, &[]).u32();
        for word in 0..3 {
            let value = e.mem.u32(position + 4 * word);
            e.mem.set_u32(local + 4 * word, value);
        }
        let player = e.global::<u32>(PLAYER);
        let angle = e.vcall(player, 0x2bc, &args![0u32]).f64();
        let facing = (angle * e.global::<f64>(DOUBLE_DEGREES)) as f32;
        let mut headings = [0.0f32; 2];
        for (index, target) in [first, second].into_iter().enumerate() {
            let reference = e.mem.u32(target);
            let other = e.vcall(reference, 0x1f4, &[]).u32();
            headings[index] = e
                .call(HEADING_BETWEEN_LOCATIONS, &args![local, other])
                .f64() as f32;
        }
        let mut distances = [0.0f32; 2];
        for (index, heading) in headings.into_iter().enumerate() {
            let difference = (facing as f64 - heading as f64) as f32;
            let wrapped = e.call(FLOAT_ABS, &args![difference]).f64() as f32;
            let folded = if above(wrapped as f64, e.global::<f64>(DOUBLE_180)) {
                e.global::<f64>(DOUBLE_360) - wrapped as f64
            } else {
                wrapped as f64
            };
            distances[index] = e.call(FLOAT_ABS, &args![folded as f32]).f64() as f32;
        }
        let [first, second] = distances;
        if above(second as f64, first as f64) {
            -1
        } else if (second as f64) < first as f64 {
            1
        } else {
            0
        }
    })
}

// Translated from 007f0500 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (first, second): orders two targets by their heading from the
/// player (`HeadingBetweenLocations` from the player's position): -1 when
/// the second's is above the first's, 1 when below, 0 otherwise (`TEST
/// AH,0x41` both ways).
pub fn fn_007f0500(e: &mut Engine, first: u32, second: u32) -> i32 {
    let (first, second) = headings_from_player(e, first, second);
    if above(second as f64, first as f64) {
        -1
    } else if above(first as f64, second as f64) {
        1
    } else {
        0
    }
}

/// Projects the world point at `point` to the screen (`NiCamera::
/// WorldPtToScreenPt` on the camera of `0045c670`/`006629f0`, epsilon from
/// `010718c0`) and answers whether it is inside: x and y each above 0.0
/// (`TEST AH,0x41; JNP`: NaN passes) and below 1.0 (`TEST AH,1; JZ`: NaN
/// passes).
fn point_on_screen(e: &mut Engine, point: u32) -> bool {
    e.with_stack(0x14, |e, local| {
        let world = local.addr();
        let x = world + 0xc;
        let y = world + 0x10;
        for word in 0..3 {
            let value = e.mem.u32(point + 4 * word);
            e.mem.set_u32(world + 4 * word, value);
        }
        e.mem.set_f32(x, 0.0);
        e.mem.set_f32(y, 0.0);
        let epsilon = load_f32(e, SCREEN_EPSILON).to_bits();
        let holder = e.call(CAMERA_HOLDER, &[]).u32();
        let camera = e.call(CAMERA_OF, &args![holder]).u32();
        e.call(WORLD_PT_TO_SCREEN_PT, &args![camera, world, x, y, epsilon]);
        let zero = double_zero(e);
        let one = e.global::<f64>(DOUBLE_ONE);
        let x = load_f32(e, x) as f64;
        let y = load_f32(e, y) as f64;
        let positive = |value: f64| value > zero || value.is_nan();
        let below_one = |value: f64| value < one || value.is_nan();
        positive(x) && positive(y) && below_one(x) && below_one(y)
    })
}

// Translated from 007f05b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: whether a part of the current target that is not
/// marked on screen (`bOnScreen`) projects inside the screen.
pub fn fn_007f05b0(e: &mut Engine, this: Ptr<VatsMenu>) -> bool {
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let mut node = target_parts(e, this, reference);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item = e.call(IDENTITY, &args![node]).u32();
        if e.mem.u8(e.mem.u32(item) + 0x2c) == 0 {
            let item = e.call(IDENTITY, &args![node]).u32();
            let part = e.mem.u32(item);
            if point_on_screen(e, part + 0x14) {
                return true;
            }
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    false
}

// Translated from 007f06d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: (part): whether the part's `WorldCoord` projects
/// inside the screen; stores the answer in its `bOnScreen`.
pub fn fn_007f06d0(e: &mut Engine, _this: Ptr<VatsMenu>, part: u32) -> bool {
    let inside = point_on_screen(e, part + 0x14);
    e.mem.set_u8(part + 0x2c, u8::from(inside));
    inside
}

/// The function-local static at `slot` guarded by `bit` of the guard word
/// `guard`: the first call stores `Tile::AddUserTrait(name, -1)`.
fn init_static_trait(e: &mut Engine, guard: u32, bit: u32, slot: u32, name: u32) {
    let bits = e.global::<u32>(guard);
    if bits & bit == 0 {
        e.set_global::<u32>(guard, bits | bit);
        let id = e.call(ADD_USER_TRAIT, &args![name, 0xffff_ffffu32]).u32();
        e.set_global::<u32>(slot, id);
    }
}

/// The meter tile (`pMeter`) of the part of the list node `node`.
fn node_meter(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(IDENTITY, &args![node]).u32();
    e.mem.u32(e.mem.u32(node) + 0x30)
}

/// `Tile::AddFadeControl(0xfa9, from, to, time, flag)` on the meter of the
/// part of `node` (the floats are copied through the x87 stack).
fn add_meter_fade(e: &mut Engine, node: u32, from: f32, to: f32, time: f32, flag: u32) {
    let meter = node_meter(e, node);
    e.call(
        ADD_FADE_CONTROL,
        &args![
            meter,
            0xfa9u32,
            quiet_f32(from),
            quiet_f32(to),
            quiet_f32(time),
            flag
        ],
    );
}

/// The player's equipped weapon form (`+0x68` object, slot `0x148`, its
/// `+8` word), or 0.
fn player_weapon(e: &mut Engine) -> u32 {
    let player = e.global::<u32>(PLAYER);
    let process = e.call(WORD_AT_68, &args![player]).u32();
    let equipped = e.vcall(process, 0x148, &[]).u32();
    if equipped != 0 {
        e.call(WORD_AT_8, &args![equipped]).u32()
    } else {
        0
    }
}

/// Whether the player's `+0x68` object answers its slot `0x1a8`.
fn player_process_1a8(e: &mut Engine) -> bool {
    let player = e.global::<u32>(PLAYER);
    let process = e.call(WORD_AT_68, &args![player]).u32();
    e.vcall(process, 0x1a8, &[]).bool()
}

// Translated from 007f07a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::UpdateChanceToHit` (Xbox PDB): fades the target's part
/// meters. Registers the user traits `_MeterPercent`, `_LimbName`,
/// `_LimbNameVisible` and `_Brightness` once (statics guarded by
/// `011db1ac`).
///
/// In VATS modes 1 and 3, or 2 without `bHeadingReached`, every meter
/// with trait `0xfa9` above 0.0 loses its fade control and gets 0.0, and
/// `bMetersShown` is cleared. Otherwise the first call after that only
/// sets `bMetersShown` and runs `007f1840`. Then, unless the target is a
/// destructible object or the player's weapon check fails, a part that
/// projects on screen without being marked (`007f05b0`), or a marked,
/// recalculation-needing part without a chance, sets `011db0ce`, clears
/// the target's `bChancesCalculated`, switches to mode 3 (keeping the
/// time at `011db0dc`) and ends. Finally each meter fades: off-screen ones
/// out (to 0.0 over 0.5), on-screen ones in to 255.0 (the current or the
/// first-limb part over 1.0, a part with another actor value than the
/// current limb over 0.2), and `_LimbNameVisible` says whether the part has
/// the current limb's actor value.
// Translation incomplete: 007f07a0 - the C++ exception-unwinding frame (FS:[0]) of the static initialisation is not translated
pub fn vats_menu_update_chance_to_hit(e: &mut Engine, this: Ptr<VatsMenu>) {
    let m = this.addr();
    init_static_trait(
        e,
        STATIC_TRAIT_GUARD,
        1,
        STATIC_METER_PERCENT,
        TEXT_METER_PERCENT,
    );
    init_static_trait(e, STATIC_TRAIT_GUARD, 2, STATIC_LIMB_NAME, TEXT_LIMB_NAME);
    init_static_trait(
        e,
        STATIC_TRAIT_GUARD,
        4,
        STATIC_LIMB_NAME_VISIBLE,
        TEXT_LIMB_NAME_VISIBLE,
    );
    init_static_trait(
        e,
        STATIC_TRAIT_GUARD,
        8,
        STATIC_BRIGHTNESS_TRAIT_2,
        TEXT_BRIGHTNESS,
    );
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let target = vats_menu_get_vats_target(e, this, reference);
    let target_node = e.call(IDENTITY, &args![target]).u32();
    let mut node = e.mem.u32(target_node).wrapping_add(8);
    let hidden = e.call(WORD_AT_8, &args![VATS]).u32() == 1
        || e.call(WORD_AT_8, &args![VATS]).u32() == 3
        || (e.call(WORD_AT_8, &args![VATS]).u32() == 2 && e.mem.u8(m + 0xf8) == 0);
    if hidden {
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let meter = node_meter(e, node);
            let value = e.call(GET_FLOAT_TRAIT, &args![meter, 0xfa9u32]).f64();
            // `TEST AH,0x41; JNZ`: only meters above 0.0.
            if above(value, double_zero(e)) {
                let meter = node_meter(e, node);
                e.call(REMOVE_FADE_CONTROL, &args![meter, 0xfa9u32]);
                let meter = node_meter(e, node);
                e.call(SET_FLOAT_TRAIT, &args![meter, 0xfa9u32, 0.0f32, 1u32]);
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
        e.mem.set_u8(m + 0xf9, 0);
        return;
    }
    if e.mem.u8(m + 0xf9) != 1 {
        e.mem.set_u8(m + 0xf9, 1);
        fn_007f1840(e, this);
        return;
    }
    let weapon = player_weapon(e);
    let skip_scan = {
        let target_node = e.call(IDENTITY, &args![target]).u32();
        e.mem.u32(e.mem.u32(target_node) + 4) == 2
            || player_process_1a8(e)
            || weapon == 0
            || e.call(CHECK_006450C0, &args![weapon]).bool()
    };
    let mut first_limb = 0u32;
    while !skip_scan && node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let recalculate = fn_007f05b0(e, this) || {
            let part = node_part(e, node);
            e.mem.u8(part + 0x2d) == 0
                && {
                    let part = node_part(e, node);
                    e.mem.u8(part + 0x2c) != 0
                }
                && {
                    let part = node_part(e, node);
                    e.mem.u8(part + 0x2f) != 0
                }
        };
        if recalculate {
            e.set_global::<u8>(FLAG_011DB0CE, 1);
            let target_node = e.call(IDENTITY, &args![target]).u32();
            let item = e.mem.u32(target_node);
            e.mem.set_u8(item + 0x10, 0);
            if e.call(WORD_AT_8, &args![VATS]).u32() != 3 {
                e.call(VATS_SET_MODE, &args![VATS, 3u32, 0u32]);
                let time = e.call(TIMER_MILLISECONDS, &args![TIMER]).u32();
                e.set_global::<u32>(RECALCULATION_TIME, time);
            }
            return;
        }
        let part = node_part(e, node);
        if e.mem.u32(part + 0x20) == 1 {
            first_limb = node;
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    let target_node = e.call(IDENTITY, &args![target]).u32();
    let mut node = e.mem.u32(target_node).wrapping_add(8);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let meter = node_meter(e, node);
        let value = e.call(GET_FLOAT_TRAIT, &args![meter, 0xfa9u32]).f64() as f32;
        let part = node_part(e, node);
        if e.mem.u8(part + 0x2c) != 0 {
            let part = node_part(e, node);
            e.mem.set_u8(part + 0x2f, 0);
            let weapon = player_weapon(e);
            let tracking = !player_process_1a8(e)
                && weapon != 0
                && !e.call(CHECK_006450C0, &args![weapon]).bool();
            if tracking {
                let part = node_part(e, node);
                let other_value = part != e.mem.u32(m + 0x100) && {
                    let part = node_part(e, node);
                    let limb = e.mem.u32(part + 0x20);
                    let value = vats_menu_get_actor_value_from_body_part(e, this, limb);
                    let limb = current_limb_of(e, m);
                    value != vats_menu_get_actor_value_from_body_part(e, this, limb)
                };
                if other_value {
                    let meter = node_meter(e, node);
                    let end = e.call(GET_FADE_END_FOR, &args![meter, 0xfa9u32]).f64();
                    // `FUCOMPP; TEST AH,0x44; JP` and `TEST AH,0x44; JP`.
                    let fade = (value as f64 == end && value as f64 != e.global::<f64>(DOUBLE_255))
                        || {
                            let meter = node_meter(e, node);
                            e.call(GET_FADE_TYPE, &args![meter, 0xfa9u32]).i32() == 1
                        };
                    if fade {
                        let to = load_f32(e, FLOAT_255);
                        let time = load_f32(e, FLOAT_POINT_TWO);
                        add_meter_fade(e, node, value, to, time, 0);
                    }
                } else {
                    let meter = node_meter(e, node);
                    if e.call(GET_FADE_FINISHED, &args![meter, 0xfa9u32]).bool() {
                        let to = load_f32(e, FLOAT_255);
                        add_meter_fade(e, node, 0.0, to, 1.0, 1);
                    }
                }
            } else {
                let fade_in = (first_limb != 0 && node == first_limb)
                    || (first_limb == 0 && e.call(NEXT_NODE, &args![node]).u32() == 0);
                if fade_in {
                    let meter = node_meter(e, node);
                    if e.call(GET_FADE_FINISHED, &args![meter, 0xfa9u32]).bool() {
                        let to = load_f32(e, FLOAT_255);
                        add_meter_fade(e, node, 0.0, to, 1.0, 1);
                    }
                }
            }
        } else {
            let meter = node_meter(e, node);
            let end = e.call(GET_FADE_END_FOR, &args![meter, 0xfa9u32]).f64();
            // `TEST AH,0x41; JNZ`: fades out when the end is above 0.0.
            if above(end, double_zero(e)) {
                let time = load_f32(e, FLOAT_HALF);
                let meter = node_meter(e, node);
                let current = e.call(GET_FLOAT_TRAIT, &args![meter, 0xfa9u32]).f64() as f32;
                add_meter_fade(e, node, current, 0.0, time, 0);
            }
        }
        let menu = vats_menu(e);
        let limb = current_limb_of(e, menu);
        let current = vats_menu_get_actor_value_from_body_part(e, Ptr::new(menu), limb);
        let part = node_part(e, node);
        let limb = e.mem.u32(part + 0x20);
        let menu = vats_menu(e);
        let value = vats_menu_get_actor_value_from_body_part(e, Ptr::new(menu), limb);
        let visible = u32::from(current == value);
        let id = e.global::<u32>(STATIC_LIMB_NAME_VISIBLE);
        let meter = node_meter(e, node);
        e.call(TILE_SET_INT, &args![meter, id, visible]);
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
}

/// The part (`ChanceToHit *`) of the list node `node`.
fn node_part(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(IDENTITY, &args![node]).u32();
    e.mem.u32(node)
}

/// The float trait `id` of the meter of `part`.
fn meter_float(e: &mut Engine, part: u32, id: u32) -> f64 {
    let meter = e.mem.u32(part + 0x30);
    e.call(GET_FLOAT_TRAIT, &args![meter, id]).f64()
}

/// `GetActorValueFromBodyPart` of the limb of `part`.
fn part_value(e: &mut Engine, this: Ptr<VatsMenu>, part: u32) -> u32 {
    let limb = e.mem.u32(part + 0x20);
    vats_menu_get_actor_value_from_body_part(e, this, limb)
}

/// `sprintf_s(buffer, 8, "%i%%", value)` (`00406d00`).
fn format_percent(e: &mut Engine, buffer: u32, value: u32) {
    e.call(SPRINTF_S, &args![buffer, 8u32, TEXT_PERCENT_FORMAT, value]);
}

// Translated from 007f0ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: writes the chance to hit into the target's part
/// meters. For each part with a limb from 1 to 12, every later part with
/// the same actor value (`GetActorValueFromBodyPart`) gets its meter built
/// if missing, one of the two meters hidden (trait `0xfa3` = 0: the
/// part's own when the later one is limb 1, else the later one's), the
/// part's `_MeterPercent` lowered to the later one's when above it (`TEST
/// AH,0x41`) and the part's chance raised to the later one's when below
/// (`TEST AH,5; JP`). The text is `%i%%` of `fChance * 100`
/// (plus `iVATSConcentratedFireBonus` times the repeated shots at the same
/// limb with the concentrated fire perk), limited by `004a8f20(.., 0x5f)`
/// and `INT_MAX(0, ..)`, or 0 with `fChance` = 0 when the player has no
/// usable weapon and the target's base answers slot `0x14` of its `+0x30`.
// Translation incomplete: 007f0ea0 - the security cookie (00ec408c) is not translated
pub fn fn_007f0ea0(e: &mut Engine, this: Ptr<VatsMenu>) {
    let m = this.addr();
    e.with_stack(8, |e, buffer| {
        let buffer = buffer.addr();
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let target = vats_menu_get_vats_target(e, this, reference);
        let target_node = e.call(IDENTITY, &args![target]).u32();
        let mut node = e.mem.u32(target_node).wrapping_add(8);
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let part = node_part(e, node);
            let mut other_node = e.call(NEXT_NODE, &args![node]).u32();
            let limb = e.mem.i32(part + 0x20);
            if limb > 0 && limb <= 0xc {
                while other_node != 0 && !e.call(LIST_IS_EMPTY, &args![other_node]).bool() {
                    let other = node_part(e, other_node);
                    let value = part_value(e, this, part);
                    let other_value = part_value(e, this, other);
                    if value == other_value {
                        if e.mem.u32(other + 0x30) == 0 {
                            let reference = e.global::<u32>(TARGET_REFERENCE);
                            let target = vats_menu_get_vats_target(e, this, reference);
                            e.call(BUILD_METERS_FOR_TARGET, &args![m, target]);
                        }
                        if other != part {
                            let hidden = if e.mem.u32(other + 0x20) == 1 {
                                e.mem.u32(part + 0x30)
                            } else {
                                e.mem.u32(other + 0x30)
                            };
                            e.call(TILE_SET_INT, &args![hidden, 0xfa3u32, 0u32]);
                        }
                        let id = e
                            .call(ADD_USER_TRAIT, &args![TEXT_METER_PERCENT, 0xffff_ffffu32])
                            .u32();
                        let other_percent = meter_float(e, other, id);
                        let id = e
                            .call(ADD_USER_TRAIT, &args![TEXT_METER_PERCENT, 0xffff_ffffu32])
                            .u32();
                        let percent = meter_float(e, part, id);
                        if above(percent, other_percent) {
                            let id = e
                                .call(ADD_USER_TRAIT, &args![TEXT_METER_PERCENT, 0xffff_ffffu32])
                                .u32();
                            let lower = meter_float(e, other, id) as f32;
                            let id = e
                                .call(ADD_USER_TRAIT, &args![TEXT_METER_PERCENT, 0xffff_ffffu32])
                                .u32();
                            let meter = e.mem.u32(part + 0x30);
                            e.call(SET_FLOAT_TRAIT, &args![meter, id, lower, 1u32]);
                        }
                        // `FCOMPP; TEST AH,5; JP`: raised when below.
                        if (load_f32(e, part + 0x28) as f64) < load_f32(e, other + 0x28) as f64 {
                            let chance = load_f32(e, other + 0x28);
                            e.mem.set_f32(part + 0x28, chance);
                        }
                    }
                    other_node = e.call(NEXT_NODE, &args![other_node]).u32();
                }
            }
            let vats = e.call(IDENTITY, &args![VATS]).u32();
            let repeated = if !e.call(LIST_IS_EMPTY, &args![vats]).bool() && {
                let last = e.call(VATS_LAST_COMMAND, &args![VATS]).u32();
                let value = part_value(e, this, part);
                e.mem.u32(last + 0x10) == value
            } {
                fn_007ef930(e, this).wrapping_add(1)
            } else {
                0
            };
            let player = e.global::<u32>(PLAYER);
            let unusable = e.call(GET_CURRENT_WEAPON, &args![player]).u32() == 0 || {
                let player = e.global::<u32>(PLAYER);
                let weapon = e.call(GET_CURRENT_WEAPON, &args![player]).u32();
                e.call(CHECK_006450C0, &args![weapon]).bool()
            };
            let no_chance = unusable
                && {
                    let reference = e.global::<u32>(TARGET_REFERENCE);
                    e.vcall(reference, 0x100, &[]).bool()
                }
                && {
                    let reference = e.global::<u32>(TARGET_REFERENCE);
                    let base = e.call(BASE_FORM_OF, &args![reference]).u32();
                    e.vcall(base + 0x30, 0x14, &[]).bool()
                };
            if no_chance {
                format_percent(e, buffer, 0);
                e.mem.set_f32(part + 0x28, 0.0);
            } else {
                let hundred = e.global::<f64>(DOUBLE_HUNDRED);
                let percent = if repeated > 0 && e.mem.u8(m + 0x104) != 0 {
                    let scaled = load_f32(e, part + 0x28) as f64 * hundred;
                    let address = e
                        .call(SETTING_INT_ADDRESS, &args![SETTING_CONCENTRATED_FIRE_BONUS])
                        .u32();
                    let bonus = e.mem.i32(address).wrapping_mul(repeated);
                    ftol(e, bonus as f64 + scaled)
                } else {
                    let scaled = load_f32(e, part + 0x28) as f64 * hundred;
                    ftol(e, scaled)
                };
                let limited = e.call(INT_MIN, &args![percent, 0x5fu32]).u32();
                let value = e.call(INT_MAX, &args![0u32, limited]).u32();
                format_percent(e, buffer, value);
            }
            if e.mem.u32(part + 0x30) == 0 {
                let reference = e.global::<u32>(TARGET_REFERENCE);
                let target = vats_menu_get_vats_target(e, this, reference);
                e.call(BUILD_METERS_FOR_TARGET, &args![m, target]);
            }
            let meter = e.mem.u32(part + 0x30);
            e.call(TILE_SET_STRING, &args![meter, 0xfc4u32, buffer, 1u32]);
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
    });
}

// Translated from 007f1290 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: (part node): the chance to hit the part of the
/// list node (0.0 without one). Gathers the player's weapon, the target's
/// equipped item (its health from `ItemChange::GetItemHealth(1) / 100`,
/// 1.0 without), the body part of the part's limb (actors only), the
/// bound size (doubled; 128.0 below 1.0), the weapon factor `008b0dd0(3)`
/// and asks `CombatFormulas::GetVATSChanceToHit` with the distance
/// (`GetDistanceBetweenFacingActors`) and reach (`GetWeaponReach`); the
/// target's item and body part count only when the target is not a
/// destructible object. Without a usable weapon the chance is 0 when the
/// path to the target (`006d5490` between two `PathingLocation`s) is
/// longer than twice the straight distance. Fills the menu's temporary
/// `VATS_COMMAND` (`+0x11C`: hit data, limb, target), runs the entry
/// points 8 (the chance) and `0x21` (`bConcentratedFirePerk`), keeps the
/// larger chance and stores it in the part when its chance is 0.0.
// Translation incomplete: 007f1290 - the C++ exception-unwinding frame (FS:[0]) that destroys the PathingLocation locals is not translated
pub fn fn_007f1290(e: &mut Engine, this: Ptr<VatsMenu>, node: u32) -> f32 {
    let m = this.addr();
    if node == 0 {
        return 0.0;
    }
    let part = node_part(e, node);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let target = vats_menu_get_vats_target(e, this, reference);
    let target_node = e.call(IDENTITY, &args![target]).u32();
    let target_item = e.mem.u32(target_node);
    let weapon = player_weapon(e);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let target_equipped = if e.vcall(reference, 0x100, &[]).bool() {
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let process = e.call(WORD_AT_68, &args![reference]).u32();
        e.vcall(process, 0x148, &[]).u32()
    } else {
        0
    };
    let condition = if target_equipped != 0 {
        let health = e.call(GET_ITEM_HEALTH, &args![target_equipped, 1u32]).f64();
        (health / e.global::<f64>(DOUBLE_HUNDRED)) as f32
    } else {
        1.0
    };
    let target_weapon = if target_equipped != 0 {
        e.call(WORD_AT_8, &args![target_equipped]).u32()
    } else {
        0
    };
    let mut body_part = 0;
    let reference = e.global::<u32>(TARGET_REFERENCE);
    if e.vcall(reference, 0x100, &[]).bool() {
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let base = e.call(BASE_FORM_OF, &args![reference]).u32();
        let body_parts = e.vcall(base, 0x180, &[]).u32();
        let limb = e.mem.u32(part + 0x20);
        body_part = e
            .call(GET_BODY_PART_BY_LIMB, &args![body_parts, limb])
            .u32();
    }
    let mut size = if weapon != 0 && e.global::<u32>(TARGET_REFERENCE) == 0 {
        let bound = e.call(GET_BOUND_SIZE, &args![weapon]).f64();
        (bound + bound) as f32
    } else {
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let bound = e.call(GET_MODEL_BOUND_SIZE, &args![reference]).f64();
        (bound + bound) as f32
    };
    // `TEST AH,5; JP`: replaced when below 1.0.
    if (size as f64) < e.global::<f64>(DOUBLE_ONE) {
        size = load_f32(e, FLOAT_128);
    }
    let factor = if weapon != 0 {
        let player = e.global::<u32>(PLAYER);
        e.call(PLAYER_FACTOR_8B0DD0, &args![player, 3u32]).f64() as f32
    } else {
        1.0
    };
    let player = e.global::<u32>(PLAYER);
    let owner = if player != 0 { player + 0xa4 } else { 0 };
    let player = e.global::<u32>(PLAYER);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let distance = e
        .call(
            GET_DISTANCE_BETWEEN_FACING_ACTORS,
            &args![player, reference, 0u32],
        )
        .f64() as f32;
    let screen = load_f32(e, part + 0x24);
    let not_destructible = e.mem.u32(target_item + 4) != 2;
    let counted_weapon = if not_destructible { target_weapon } else { 0 };
    let counted_part = if not_destructible { body_part } else { 0 };
    let player = e.global::<u32>(PLAYER);
    let reach = e.call(GET_WEAPON_REACH, &args![player, weapon]).f64() as f32;
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let mut chance = e
        .call(
            GET_VATS_CHANCE_TO_HIT,
            &args![
                owner,
                reference,
                weapon,
                condition,
                reach,
                factor,
                counted_part,
                counted_weapon,
                screen,
                distance,
                size,
                0u32
            ],
        )
        .f64() as f32;
    let mut unreachable = false;
    if weapon == 0 || e.call(CHECK_006450C0, &args![weapon]).bool() {
        unreachable = e.with_stack(0x28 * 2 + 4 + 0xc, |e, block| {
            let from = block.addr();
            let to = from + 0x28;
            let length = to + 0x28;
            let offset = length + 4;
            let player = e.global::<u32>(PLAYER);
            e.call(PATHING_LOCATION_CONSTRUCT, &args![from, player]);
            let reference = e.global::<u32>(TARGET_REFERENCE);
            e.call(PATHING_LOCATION_CONSTRUCT, &args![to, reference]);
            let largest = load_f32(e, FLOAT_LARGEST);
            e.mem.set_f32(length, largest);
            if !e
                .call(PATH_LENGTH, &args![from, to, 0u32, length, 0u32])
                .bool()
            {
                let largest = load_f32(e, FLOAT_LARGEST);
                e.mem.set_f32(length, largest);
            }
            let reference = e.global::<u32>(TARGET_REFERENCE);
            let destination = e.vcall(reference, 0x1f4, &[]).u32();
            let player = e.global::<u32>(PLAYER);
            let origin = e.vcall(player, 0x1f4, &[]).u32();
            let difference = e
                .call(POINT_SUBTRACT, &args![origin, offset, destination])
                .u32();
            let straight = e.call(POINT_LENGTH, &args![difference]).f64() as f32;
            let doubled = straight as f64 * e.global::<f64>(DOUBLE_TWO);
            // `FCOMPP; TEST AH,0x41; JNZ`: unreachable when the path is longer.
            let far = above(load_f32(e, length) as f64, doubled);
            e.call(PATHING_LOCATION_DESTRUCT, &args![to]);
            e.call(PATHING_LOCATION_DESTRUCT, &args![from]);
            far
        });
    }
    let reported = if unreachable { 0.0 } else { quiet_f32(chance) };
    e.with_stack(8, |e, values| {
        let reported_slot = values.addr();
        let bonus = reported_slot + 4;
        e.mem.set_f32(reported_slot, reported);
        e.mem.set_f32(bonus, 0.0);
        e.mem.set_u8(m + 0x118, 1);
        e.call(VATS_COMMAND_INITIALIZE, &args![m + 0x11c]);
        let limb = if body_part != 0 {
            e.call(BODY_PART_VALUE, &args![body_part]).u8() as i8 as i32
        } else {
            -1
        };
        e.mem.set_i32(m + 0x12c, limb);
        let reference = e.global::<u32>(TARGET_REFERENCE);
        e.mem.set_u32(m + 0x128, reference);
        e.mem.set_u8(m + 0x120, 1);
        let hit_data = e.mem.u32(m + 0x130);
        e.call(FLAGS_SET_00407E00, &args![hit_data, 0x800u32, 1u32]);
        let hit_data = e.mem.u32(m + 0x130);
        e.call(INITIALIZE_HIT_DATA, &args![hit_data, m + 0x11c]);
        let form = if weapon != 0 {
            weapon
        } else {
            e.global::<u32>(DEFAULT_FORM)
        };
        let player = e.global::<u32>(PLAYER);
        let reference = e.global::<u32>(TARGET_REFERENCE);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![8u32, player, form, reference, reported_slot],
        );
        let player = e.global::<u32>(PLAYER);
        e.call(HANDLE_ENTRY_POINT, &args![0x21u32, player, bonus]);
        // `TEST AH,0x44; JNP`: set unless equal to 0.0.
        let perk = load_f32(e, bonus) as f64 != double_zero(e);
        e.mem.set_u8(m + 0x104, u8::from(perk));
        let reported = load_f32(e, reported_slot);
        // `FCOMPP; TEST AH,5; JP`: the larger when below.
        if (chance as f64) < reported as f64 {
            chance = reported;
        }
    });
    // `TEST AH,0x44; JP` (equal to 0.0) then `TEST AH,0x41` (above 0.0).
    if load_f32(e, part + 0x28) as f64 == double_zero(e) && above(chance as f64, double_zero(e)) {
        e.mem.set_f32(part + 0x28, quiet_f32(chance));
    }
    quiet_f32(chance)
}

/// `FLOAT_MIN(1.0, FLOAT_MAX(0.0, value))` through the two helpers (each
/// takes its floats as bit patterns): the meter fraction clamped to
/// `0..=1`.
fn clamp_unit(e: &mut Engine, value: f32) -> f32 {
    let low = e
        .call(
            FLOAT_MAX,
            &args![0.0f32.to_bits(), quiet_f32(value).to_bits()],
        )
        .f64() as f32;
    e.call(FLOAT_MIN, &args![1.0f32.to_bits(), low.to_bits()])
        .f64() as f32
}

/// Sets the `_LimbName` text of the meter of `part` (`Tile::AddUserTrait`
/// asked for the id each time, as the code does).
fn set_meter_name(e: &mut Engine, part: u32, name: u32) {
    let id = e
        .call(ADD_USER_TRAIT, &args![TEXT_LIMB_NAME, 0xffff_ffffu32])
        .u32();
    let meter = e.mem.u32(part + 0x30);
    e.call(TILE_SET_STRING, &args![meter, id, name, 1u32]);
}

/// Sets `_MeterPercent` of the meter of `part` to the clamped `fraction`.
fn set_meter_percent(e: &mut Engine, part: u32, fraction: f32) {
    let value = clamp_unit(e, fraction);
    let id = e
        .call(ADD_USER_TRAIT, &args![TEXT_METER_PERCENT, 0xffff_ffffu32])
        .u32();
    let meter = e.mem.u32(part + 0x30);
    e.call(SET_FLOAT_TRAIT, &args![meter, id, value, 1u32]);
}

/// `FLOAT_MAX(low, FLOAT_MIN(high, value))` with the integer bounds
/// converted to floats (`FILD` then a float push).
fn clamp_to_screen(e: &mut Engine, value: f32, low: u32, high: u32) -> f32 {
    let high = e.global::<i32>(high) as f32;
    let limited = e
        .call(
            FLOAT_MIN,
            &args![high.to_bits(), quiet_f32(value).to_bits()],
        )
        .f64() as f32;
    let low = e.global::<i32>(low) as f32;
    e.call(FLOAT_MAX, &args![low.to_bits(), limited.to_bits()])
        .f64() as f32
}

/// Moves the point `point` (two floats) away from `middle`: the direction
/// (`point - middle`, normalised by `004e7560`, whose length divides it
/// through `007f2890`) times half the length of `limits`, added to
/// `middle`. `scratch` holds the code's temporaries (0x28 bytes).
fn push_from_middle(e: &mut Engine, point: u32, middle: u32, limits: u32, scratch: u32) {
    let first = scratch;
    let divided = scratch + 8;
    let second = scratch + 0x10;
    let scaled = scratch + 0x18;
    let moved = scratch + 0x20;
    let difference = e.call(POINT2_SUBTRACT, &args![point, first, middle]).u32();
    let length = e.call(POINT2_NORMALIZE, &args![difference]).f64() as f32;
    let difference = e.call(POINT2_SUBTRACT, &args![point, second, middle]).u32();
    let direction = fn_007f2890(e, difference, divided, length);
    let (x, y) = (e.mem.u32(direction), e.mem.u32(direction + 4));
    e.mem.set_u32(point, x);
    e.mem.set_u32(point + 4, y);
    let half = (e.call(POINT2_LENGTH, &args![limits]).f64() / e.global::<f64>(DOUBLE_TWO)) as f32;
    let offset = e.call(POINT2_SCALE, &args![point, scaled, half]).u32();
    let sum = e.call(POINT2_ADD, &args![middle, moved, offset]).u32();
    let (x, y) = (e.mem.u32(sum), e.mem.u32(sum + 4));
    e.mem.set_u32(point, x);
    e.mem.set_u32(point + 4, y);
}

// Translated from 007f1840 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: lays out the meters of the target's parts.
///
/// Sets the screen bounds once (statics guarded by `011db1c8`: half the
/// width and height, the left and top margins `2 * 007177c0 + 100` and
/// `2 * 00717820 + 100`, the right edge `width - left - 50` and the bottom
/// edge `height - 2 * 00717820 - trait 0xfb0 of pTiles[17] + 50`). For each
/// part it finds the scene object (the target's slot `0x1D0` for a
/// destructible object, the `0x190` lookup for limb 14, the body part's
/// node `004aae30` otherwise; kept from the part before when there is
/// none), stores its world position (`+0x8C`) and screen position
/// (`WorldPtToScreenPt`, scaled to the screen with y flipped), the plane
/// position (`007f51b0`'s matrix times the world position), the meter's
/// `_LimbName` and `_MeterPercent` (the target's name and health, the body
/// part's name and actor value, or the item's name and condition), and
/// `bOnScreen` (off screen: the meter fades out over 0.5). Then each meter
/// on screen is placed beside its point (100 or 50 pixels off the screen
/// centre for most limbs), kept inside the bounds and told its side (trait
/// `0xfb7`), and overlapping meters (closer than 150 by 100) are pushed
/// apart from their middle, up to 31 passes.
// Translation incomplete: 007f1840 - the C++ exception-unwinding frame (FS:[0]) of the static initialisation is not translated
pub fn fn_007f1840(e: &mut Engine, this: Ptr<VatsMenu>) {
    let m = this.addr();
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let target = vats_menu_get_vats_target(e, this, reference);
    let target_node = e.call(IDENTITY, &args![target]).u32();
    let target_item = e.mem.u32(target_node);
    e.with_stack(0x100, |e, block| {
        let position = block.addr();
        let matrix = position + 0xc;
        let plane = matrix + 0x24;
        let points = plane + 0xc;
        let scratch = points + 0x20;
        let mut scene_object = 0u32;
        e.call(IDENTITY, &args![position]);
        let guard = e.global::<u32>(LAYOUT_GUARD);
        if guard & 1 == 0 {
            e.set_global::<u32>(LAYOUT_GUARD, guard | 1);
            let width = e.call(SCREEN_WIDTH, &[]).f64();
            let value = ftol(e, width / e.global::<f64>(DOUBLE_TWO));
            e.set_global::<i32>(LAYOUT_HALF_WIDTH, value);
        }
        let guard = e.global::<u32>(LAYOUT_GUARD);
        if guard & 2 == 0 {
            e.set_global::<u32>(LAYOUT_GUARD, guard | 2);
            let height = e.call(SIZE_00715DA0, &[]).f64();
            let value = ftol(e, height / e.global::<f64>(DOUBLE_TWO));
            e.set_global::<i32>(LAYOUT_HALF_HEIGHT, value);
        }
        let guard = e.global::<u32>(LAYOUT_GUARD);
        if guard & 4 == 0 {
            e.set_global::<u32>(LAYOUT_GUARD, guard | 4);
            let size = e.call(SIZE_007177C0, &[]).f64();
            let value = ftol(e, size + size + e.global::<f64>(DOUBLE_HUNDRED));
            e.set_global::<i32>(LAYOUT_LEFT, value);
        }
        let guard = e.global::<u32>(LAYOUT_GUARD);
        if guard & 8 == 0 {
            e.set_global::<u32>(LAYOUT_GUARD, guard | 8);
            let size = e.call(SIZE_00717820, &[]).f64();
            let value = ftol(e, size + size + e.global::<f64>(DOUBLE_HUNDRED));
            e.set_global::<i32>(LAYOUT_TOP, value);
        }
        let guard = e.global::<u32>(LAYOUT_GUARD);
        if guard & 0x10 == 0 {
            e.set_global::<u32>(LAYOUT_GUARD, guard | 0x10);
            let width = e.call(SCREEN_WIDTH, &[]).f64();
            let left = e.global::<i32>(LAYOUT_LEFT) as f64;
            let value = ftol(e, width - left - e.global::<f64>(DOUBLE_FIFTY));
            e.set_global::<i32>(LAYOUT_RIGHT, value);
        }
        let guard = e.global::<u32>(LAYOUT_GUARD);
        if guard & 0x20 == 0 {
            e.set_global::<u32>(LAYOUT_GUARD, guard | 0x20);
            let height = e.call(SIZE_00715DA0, &[]).f64();
            let size = e.call(SIZE_00717820, &[]).f64();
            let remaining = height - size * e.global::<f64>(DOUBLE_TWO);
            let menu = vats_menu(e);
            let bar = e.mem.u32(menu + 0x6c);
            let bar_height = e.call(GET_FLOAT_TRAIT, &args![bar, 0xfb0u32]).f64();
            let value = ftol(e, remaining - bar_height + e.global::<f64>(DOUBLE_FIFTY));
            e.set_global::<i32>(LAYOUT_BOTTOM, value);
        }
        let weapon = player_weapon(e);
        let mut node = target_item.wrapping_add(8);
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let part = node_part(e, node);
            let mut body_part = 0u32;
            if e.mem.u32(target_item + 4) == 2 {
                let object = e.mem.u32(target_item);
                scene_object = e.vcall(object, 0x1d0, &[]).u32();
            } else if e.mem.u32(part + 0x20) == 0xe {
                let reference = e.global::<u32>(TARGET_REFERENCE);
                scene_object = if e.vcall(reference, 0x100, &[]).bool() {
                    let reference = e.global::<u32>(TARGET_REFERENCE);
                    let process = e.call(WORD_AT_68, &args![reference]).u32();
                    let reference = e.global::<u32>(TARGET_REFERENCE);
                    let key = e.vcall(reference, 0x1e8, &[]).u32();
                    e.vcall(process, 0x190, &args![key]).u32()
                } else {
                    0
                };
            } else {
                let reference = e.global::<u32>(TARGET_REFERENCE);
                let base = e.call(BASE_FORM_OF, &args![reference]).u32();
                let body_parts = e.vcall(base, 0x180, &[]).u32();
                let limb = e.mem.u32(part + 0x20);
                body_part = e
                    .call(GET_BODY_PART_BY_LIMB, &args![body_parts, limb])
                    .u32();
                if body_part != 0 {
                    let name = e.call(BODY_PART_NODE_NAME, &args![body_part]).u32();
                    let reference = e.global::<u32>(TARGET_REFERENCE);
                    let root = e.vcall(reference, 0x1d0, &[]).u32();
                    scene_object = e.call(FIND_NODE, &args![root, name]).u32();
                }
            }
            if scene_object != 0 {
                let world = e.call(OBJECT_WORLD_POSITION, &args![scene_object]).u32();
                for word in 0..3 {
                    let value = e.mem.u32(world + 4 * word);
                    e.mem.set_u32(position + 4 * word, value);
                }
                for word in 0..3 {
                    let value = e.mem.u32(position + 4 * word);
                    e.mem.set_u32(part + 0x14 + 4 * word, value);
                }
                let epsilon = load_f32(e, SCREEN_EPSILON).to_bits();
                let holder = e.call(CAMERA_HOLDER, &[]).u32();
                let camera = e.call(CAMERA_OF, &args![holder]).u32();
                e.call(
                    WORLD_PT_TO_SCREEN_PT,
                    &args![camera, position, part, part + 4, epsilon],
                );
            }
            let width = e.call(SCREEN_WIDTH, &[]).f64();
            let x = (width * load_f32(e, part) as f64) as f32;
            e.mem.set_f32(part, x);
            let flipped = 1.0 - load_f32(e, part + 4) as f64;
            let height = e.call(SIZE_00715DA0, &[]).f64();
            e.mem.set_f32(part + 4, (height * flipped) as f32);
            e.call(IDENTITY, &args![matrix]);
            fn_007f51b0(e, this, matrix);
            let transformed = e
                .call(MATRIX_TIMES_POINT, &args![matrix, plane, part + 0x14])
                .u32();
            for word in 0..3 {
                let value = e.mem.u32(transformed + 4 * word);
                e.mem.set_u32(part + 8 + 4 * word, value);
            }
            let reference = e.global::<u32>(TARGET_REFERENCE);
            let item = if e.vcall(reference, 0x100, &[]).bool() {
                let reference = e.global::<u32>(TARGET_REFERENCE);
                let process = e.call(WORD_AT_68, &args![reference]).u32();
                e.vcall(process, 0x148, &[]).u32()
            } else {
                0
            };
            if item != 0 {
                e.call(GET_ITEM_HEALTH, &args![item, 1u32]);
            }
            let item_form = if item != 0 {
                e.call(WORD_AT_8, &args![item]).u32()
            } else {
                0
            };
            e.mem.set_u8(m + 0x118, 0);
            let whole_target = e.mem.u32(target_item + 4) == 2
                || player_process_1a8(e)
                || weapon == 0
                || e.call(CHECK_006450C0, &args![weapon]).bool();
            if whole_target {
                let object = e.mem.u32(target_item);
                let name = e.call(REFERENCE_NAME, &args![object]).u32();
                set_meter_name(e, part, name);
                let object = e.mem.u32(target_item);
                let health = e.call(REFERENCE_HEALTH, &args![object]).f64();
                let fraction = (health / e.global::<f64>(DOUBLE_HUNDRED)) as f32;
                set_meter_percent(e, part, fraction);
            } else if body_part != 0 {
                let name = e.call(BODY_PART_NAME, &args![body_part]).u32();
                set_meter_name(e, part, name);
                let limb = e.mem.u32(part + 0x20);
                let value = vats_menu_get_actor_value_from_body_part(e, this, limb);
                let owner = e.global::<u32>(TARGET_REFERENCE) + 0xa4;
                let condition = e.vcall(owner, 0x08, &args![value]).i32();
                let fraction = (condition as f64 / e.global::<f64>(DOUBLE_HUNDRED)) as f32;
                set_meter_percent(e, part, fraction);
            } else if item_form != 0 {
                let name = e.call(FORM_FULL_NAME, &args![item_form + 0x30]).u32();
                set_meter_name(e, part, name);
                let form = e.call(WORD_AT_8, &args![item]).u32();
                let maximum = if e.call(GET_FORM_HEALTH, &args![form]).u32() != 0 {
                    let form = e.call(WORD_AT_8, &args![item]).u32();
                    e.call(GET_FORM_HEALTH, &args![form]).u32()
                } else {
                    1
                };
                let health = e.call(GET_ITEM_HEALTH, &args![item, 0u32]).f64();
                let fraction = (health / maximum as f64) as f32;
                set_meter_percent(e, part, fraction);
            }
            // `TEST AH,5; JNP` (below 0.0) and `TEST AH,0x41; JZ` (above
            // the size): off screen.
            let x = load_f32(e, part) as f64;
            let off_screen = x < double_zero(e)
                || {
                    let width = e.call(SCREEN_WIDTH, &[]).f64();
                    x > width
                }
                || (load_f32(e, part + 4) as f64) < double_zero(e)
                || {
                    let height = e.call(SIZE_00715DA0, &[]).f64();
                    load_f32(e, part + 4) as f64 > height
                };
            if off_screen {
                e.mem.set_u8(part + 0x2c, 0);
                e.mem.set_u8(part + 0x2d, 0);
                e.mem.set_u8(part + 0x2e, 0);
                let meter = e.mem.u32(part + 0x30);
                e.call(REMOVE_FADE_CONTROL, &args![meter, 0xfa9u32]);
                let time = load_f32(e, FLOAT_HALF);
                let meter = e.mem.u32(part + 0x30);
                let current = e.call(GET_FLOAT_TRAIT, &args![meter, 0xfa9u32]).f64() as f32;
                let meter = e.mem.u32(part + 0x30);
                e.call(
                    ADD_FADE_CONTROL,
                    &args![meter, 0xfa9u32, current, 0.0f32, time, 0u32],
                );
            } else {
                e.mem.set_u8(part + 0x2c, 1);
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }

        // 007f2100: the meters beside their points.
        let target_node = e.call(IDENTITY, &args![target]).u32();
        let mut node = e.mem.u32(target_node).wrapping_add(8);
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let part = node_part(e, node);
            if e.mem.u8(part + 0x2c) != 0 {
                let limb = e.mem.i32(part + 0x20);
                let x = load_f32(e, part) as f64;
                let y = load_f32(e, part + 4) as f64;
                let half_width = e.global::<i32>(LAYOUT_HALF_WIDTH) as f64;
                let half_height = e.global::<i32>(LAYOUT_HALF_HEIGHT) as f64;
                let (column, row) = match limb.wrapping_sub(1) as u32 {
                    0 | 1 => {
                        let column = ftol(e, x);
                        let offset = if above(y, half_height) { 100 } else { -100 };
                        (column, ftol(e, offset as f64 + y))
                    }
                    2..=11 | 13 => {
                        let offset = if above(x, half_width) { 100 } else { -100 };
                        let column = ftol(e, offset as f64 + x);
                        let offset = if above(y, half_height) { 50 } else { -50 };
                        (column, ftol(e, offset as f64 + y))
                    }
                    _ => {
                        let column = ftol(e, x);
                        (column, ftol(e, y))
                    }
                };
                let right = e.global::<u32>(LAYOUT_RIGHT);
                let limited = e.call(INT_MIN, &args![right, column]).u32();
                let left = e.global::<u32>(LAYOUT_LEFT);
                let column = e.call(INT_MAX, &args![left, limited]).i32();
                let bottom = e.global::<u32>(LAYOUT_BOTTOM);
                let limited = e.call(INT_MIN, &args![bottom, row]).u32();
                let top = e.global::<u32>(LAYOUT_TOP);
                let row = e.call(INT_MAX, &args![top, limited]).i32();
                let meter = e.mem.u32(part + 0x30);
                e.call(TILE_SET_INT, &args![meter, 0xfa1u32, column]);
                let meter = e.mem.u32(part + 0x30);
                e.call(TILE_SET_INT, &args![meter, 0xfa2u32, row]);
                let limb = e.mem.i32(part + 0x20);
                if (3..=12).contains(&limb) || limb == 0xe {
                    let side = if column < e.global::<i32>(LAYOUT_HALF_WIDTH) {
                        4u32
                    } else {
                        1
                    };
                    let meter = e.mem.u32(part + 0x30);
                    e.call(TILE_SET_INT, &args![meter, 0xfb7u32, side]);
                }
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }

        // 007f237b: overlapping meters pushed apart.
        let first = points;
        let second = points + 8;
        let middle = points + 0x10;
        let limits = points + 0x18;
        e.call(IDENTITY, &args![first]);
        e.call(IDENTITY, &args![second]);
        e.call(IDENTITY, &args![middle]);
        e.call(IDENTITY, &args![limits]);
        let distance_x = load_f32(e, FLOAT_150);
        e.mem.set_f32(limits, distance_x);
        let distance_y = load_f32(e, FLOAT_HUNDRED);
        e.mem.set_f32(limits + 4, distance_y);
        let mut moved = true;
        let mut passes = 0i32;
        while moved {
            moved = false;
            passes = passes.wrapping_add(1);
            let target_node = e.call(IDENTITY, &args![target]).u32();
            let mut node = e.mem.u32(target_node).wrapping_add(8);
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let part = node_part(e, node);
                let mut other_node = e.call(NEXT_NODE, &args![node]).u32();
                while other_node != 0 && !e.call(LIST_IS_EMPTY, &args![other_node]).bool() {
                    let other = node_part(e, other_node);
                    for (point, owner) in [(first, part), (second, other)] {
                        let meter = e.mem.u32(owner + 0x30);
                        let x = e.call(GET_FLOAT_TRAIT, &args![meter, 0xfa1u32]).f64() as f32;
                        e.mem.set_f32(point, x);
                        let meter = e.mem.u32(owner + 0x30);
                        let y = e.call(GET_FLOAT_TRAIT, &args![meter, 0xfa2u32]).f64() as f32;
                        e.mem.set_f32(point + 4, y);
                    }
                    let gap_x = (load_f32(e, first) as f64 - load_f32(e, second) as f64) as f32;
                    let gap_x = e.call(FLOAT_ABS, &args![gap_x]).f64();
                    // `FCOMPP; TEST AH,0x41; JNZ`: overlapping when the
                    // limit is above the gap, both ways.
                    let overlapping = above(load_f32(e, limits) as f64, gap_x) && {
                        let gap_y =
                            (load_f32(e, first + 4) as f64 - load_f32(e, second + 4) as f64) as f32;
                        let gap_y = e.call(FLOAT_ABS, &args![gap_y]).f64();
                        above(load_f32(e, limits + 4) as f64, gap_y)
                    };
                    if overlapping {
                        if e.call(POINT2_EQUALS, &args![first, second]).bool() {
                            let x =
                                (load_f32(e, second) as f64 + e.global::<f64>(DOUBLE_ONE)) as f32;
                            e.mem.set_f32(second, x);
                        }
                        for axis in [0u32, 4] {
                            let a = load_f32(e, first + axis);
                            let b = load_f32(e, second + axis);
                            // `FCOMPP; TEST AH,5; JP`: the second when below.
                            let lower = if (b as f64) < a as f64 { b } else { a };
                            let gap = (a as f64 - b as f64) as f32;
                            let gap = e.call(FLOAT_ABS, &args![gap]).f64();
                            let centre = (gap / e.global::<f64>(DOUBLE_TWO) + lower as f64) as f32;
                            e.mem.set_f32(middle + axis, centre);
                        }
                        push_from_middle(e, first, middle, limits, scratch);
                        push_from_middle(e, second, middle, limits, scratch + 0x28);
                        for point in [first, second] {
                            let x = load_f32(e, point);
                            let x = clamp_to_screen(e, x, LAYOUT_LEFT, LAYOUT_RIGHT);
                            e.mem.set_f32(point, x);
                            let y = load_f32(e, point + 4);
                            let y = clamp_to_screen(e, y, LAYOUT_TOP, LAYOUT_BOTTOM);
                            e.mem.set_f32(point + 4, y);
                        }
                        for (point, owner) in [(first, part), (second, other)] {
                            let x = load_f32(e, point);
                            let meter = e.mem.u32(owner + 0x30);
                            e.call(SET_FLOAT_TRAIT, &args![meter, 0xfa1u32, x, 1u32]);
                            let y = load_f32(e, point + 4);
                            let meter = e.mem.u32(owner + 0x30);
                            e.call(SET_FLOAT_TRAIT, &args![meter, 0xfa2u32, y, 1u32]);
                        }
                        moved = true;
                    }
                    other_node = e.call(NEXT_NODE, &args![other_node]).u32();
                }
                node = e.call(NEXT_NODE, &args![node]).u32();
            }
            if passes > 0x1e {
                moved = false;
            }
        }
    });
}

// Translated from 007f2890 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on a point (`NiPoint2`): (out, divisor): `out = this /
/// divisor` (each component stored as a float); returns `out`.
pub fn fn_007f2890(e: &mut Engine, this: u32, out: u32, divisor: f32) -> u32 {
    let x = e.mem.f32(this);
    let y = e.mem.f32(this + 4);
    e.mem
        .set_f32(out, (quiet_f32(x) as f64 / divisor as f64) as f32);
    e.mem
        .set_f32(out + 4, (quiet_f32(y) as f64 / divisor as f64) as f32);
    out
}

/// The clip size used by `UpdateAmmo`: the Rock-It ammo count when
/// `rock_it`, else `GetFormClipRounds` with the mod effect 2 of the
/// equipped item.
fn clip_capacity(
    e: &mut Engine,
    rock_it: bool,
    rock_it_ammo: f32,
    equipped: u32,
    weapon: u32,
) -> f32 {
    if rock_it {
        quiet_f32(rock_it_ammo)
    } else {
        let modded = e.call(HAS_MOD_EFFECT, &args![equipped, 2u32]).u8();
        e.call(GET_FORM_CLIP_ROUNDS, &args![weapon, modded as u32])
            .i32() as f32
    }
}

/// `FLOAT_MAX(0.0, value)` (the float pushed as bits).
fn not_negative(e: &mut Engine, value: f32) -> f32 {
    e.call(
        FLOAT_MAX,
        &args![0.0f32.to_bits(), quiet_f32(value).to_bits()],
    )
    .f64() as f32
}

/// `FLOAT_MIN(a, b)` (the floats pushed as bits).
fn float_min(e: &mut Engine, a: f32, b: f32) -> f32 {
    e.call(
        FLOAT_MIN,
        &args![quiet_f32(a).to_bits(), quiet_f32(b).to_bits()],
    )
    .f64() as f32
}

// Translated from 007f28d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::UpdateAmmo` (Xbox PDB): counts the ammo of the equipped
/// weapon into `fClipAmmo`/`fTotalAmmo` (thrown and placed kinds 10, 11,
/// 13: the weapon's `+0x102` byte and the rest of the inventory count;
/// the Rock-It Launcher: its ammo count; others: the loaded ammo `+4` of
/// the player's ammo list, or the inventory count up to the clip size,
/// the rest in reserve, or a full clip in reserve with ammo
/// regeneration), copies them to the queued counts and replays the queued
/// commands on them (attacks 3 to 6 use their shots and reload when the
/// clip runs empty, 7 and 8 use one, 9 reloads). The ammo, condition and
/// count tiles (`pTiles[13..=16]`) show or hide with the weapon's
/// ammunition, the condition meter (`pTiles[15]`) gets the item's health
/// fraction, and `pTiles[13]` reads `%i/%i` of the queued clip and total.
/// Answers whether the last queued attack emptied the clip (and reloaded).
// Translation incomplete: 007f28d0 - the security cookie (00ec408c) is not translated
pub fn vats_menu_update_ammo(e: &mut Engine, this: Ptr<VatsMenu>) -> bool {
    let m = this.addr();
    let player = e.global::<u32>(PLAYER);
    let process = e.call(WORD_AT_68, &args![player]).u32();
    let equipped = e.vcall(process, 0x148, &[]).u32();
    let player = e.global::<u32>(PLAYER);
    let process = e.call(WORD_AT_68, &args![player]).u32();
    let ammo_list = e.vcall(process, 0x14c, &[]).u32();
    e.mem.set_f32(m + 0xec, 0.0);
    e.mem.set_f32(m + 0xe8, 0.0);
    let mut reloaded = false;
    if equipped == 0 {
        set_tile_int(e, m, 0x60, 0xfa3, 0);
        set_tile_int(e, m, 0x64, 0xfa3, 0);
        set_tile_int(e, m, 0x68, 0xfc4, 0);
        set_tile_int(e, m, 0x68, 0xfa3, 0);
        set_tile_int(e, m, 0x5c, 0xfa3, 0);
        e.mem.set_f32(m + 0xf0, 0.0);
        e.mem.set_f32(m + 0xf4, 0.0);
    } else {
        let weapon = e.call(WORD_AT_8, &args![equipped]).u32();
        let rock_it = e.call(IS_ROCK_IT_LAUNCHER, &args![weapon + 0xa4]).bool();
        let player = e.global::<u32>(PLAYER);
        let rock_it_ammo = e.call(GET_NUM_ROCK_IT_AMMO, &args![player]).i32() as f32;
        let clip;
        let mut total = 0.0f32;
        let mut no_regeneration = true;
        let kind = e.call(WEAPON_KIND, &args![weapon]).i32();
        if kind >= 0xa && (kind <= 0xb || kind == 0xd) {
            clip = e.call(WEAPON_102_524B60, &args![weapon]).u8() as f32;
            let player = e.global::<u32>(PLAYER);
            let inventory = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
            let count = e.call(GET_OBJECT_COUNT, &args![inventory, weapon]).i32();
            total = (count as f64 - clip as f64) as f32;
        } else if rock_it {
            total = 0.0;
            clip = quiet_f32(rock_it_ammo);
        } else {
            let player = e.global::<u32>(PLAYER);
            let inventory = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
            let player = e.global::<u32>(PLAYER);
            let ammo = e.call(GET_CURRENT_AMMO, &args![weapon, player]).u32();
            let count = e.call(GET_OBJECT_COUNT, &args![inventory, ammo]).i32();
            let modded = e.call(HAS_MOD_EFFECT, &args![equipped, 6u32]).u8();
            let regeneration = e
                .call(GET_AMMO_REGEN_RATE, &args![weapon, modded as u32])
                .f64();
            // `TEST AH,0x41; JNZ`: a full clip in reserve when above 0.0.
            if above(regeneration, double_zero(e)) {
                let modded = e.call(HAS_MOD_EFFECT, &args![equipped, 2u32]).u8();
                total = e
                    .call(GET_FORM_CLIP_ROUNDS, &args![weapon, modded as u32])
                    .i32() as f32;
                no_regeneration = false;
            }
            let loaded = if ammo_list != 0 {
                e.call(NEXT_NODE, &args![ammo_list]).i32()
            } else {
                let modded = e.call(HAS_MOD_EFFECT, &args![equipped, 2u32]).u8();
                let rounds = e
                    .call(GET_FORM_CLIP_ROUNDS, &args![weapon, modded as u32])
                    .i32();
                if count > rounds {
                    let modded = e.call(HAS_MOD_EFFECT, &args![equipped, 2u32]).u8();
                    e.call(GET_FORM_CLIP_ROUNDS, &args![weapon, modded as u32])
                        .i32()
                } else {
                    count
                }
            };
            clip = loaded as f32;
            if no_regeneration {
                total = (count as f64 - clip as f64) as f32;
            }
        }
        e.mem.set_f32(m + 0xe8, quiet_f32(clip));
        e.mem.set_f32(m + 0xf0, quiet_f32(clip));
        e.mem.set_f32(m + 0xec, quiet_f32(total));
        e.mem.set_f32(m + 0xf4, quiet_f32(total));
        let mut node = e.call(IDENTITY, &args![VATS]).u32();
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let command = node_part(e, node);
            match e.mem.u32(command) {
                3..=6 => {
                    let command = node_part(e, node);
                    let shots = e.mem.u8(command + 8) as f64;
                    let left = (load_f32(e, m + 0xf0) as f64 - shots) as f32;
                    let left = not_negative(e, left);
                    e.mem.set_f32(m + 0xf0, left);
                    // `TEST AH,0x44; JP`: reloads when the clip is empty.
                    if load_f32(e, m + 0xf0) as f64 == double_zero(e) {
                        let capacity = clip_capacity(e, rock_it, rock_it_ammo, equipped, weapon);
                        let reserve = load_f32(e, m + 0xf4);
                        let loaded = float_min(e, reserve, capacity);
                        e.mem.set_f32(m + 0xf0, loaded);
                        let capacity = clip_capacity(e, rock_it, rock_it_ammo, equipped, weapon);
                        let ammo = load_f32(e, m + 0xec);
                        let taken = float_min(e, ammo, capacity) as f64;
                        let reserve = (load_f32(e, m + 0xf4) as f64 - taken) as f32;
                        e.mem.set_f32(m + 0xf4, reserve);
                        let reserve = load_f32(e, m + 0xf4);
                        let reserve = not_negative(e, reserve);
                        e.mem.set_f32(m + 0xf4, reserve);
                        reloaded = true;
                    } else {
                        reloaded = false;
                    }
                }
                7 | 8 => {
                    let left = (load_f32(e, m + 0xf0) as f64 - e.global::<f64>(DOUBLE_ONE)) as f32;
                    let left = not_negative(e, left);
                    e.mem.set_f32(m + 0xf0, left);
                }
                9 => {
                    let capacity = clip_capacity(e, rock_it, rock_it_ammo, equipped, weapon);
                    let ammo = load_f32(e, m + 0xec);
                    let loaded = float_min(e, ammo, capacity);
                    e.mem.set_f32(m + 0xf0, loaded);
                    let capacity = clip_capacity(e, rock_it, rock_it_ammo, equipped, weapon);
                    let ammo = load_f32(e, m + 0xec);
                    let taken = float_min(e, ammo, capacity) as f64;
                    let reserve = (load_f32(e, m + 0xf4) as f64 - taken) as f32;
                    e.mem.set_f32(m + 0xf4, reserve);
                }
                _ => {}
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
        let player = e.global::<u32>(PLAYER);
        if e.call(GET_CURRENT_AMMO, &args![weapon, player]).u32() != 0 {
            set_tile_int(e, m, 0x60, 0xfa3, 1);
            set_tile_int(e, m, 0x64, 0xfc4, 1);
            set_tile_int(e, m, 0x68, 0xfc4, 1);
            set_tile_int(e, m, 0x68, 0xfa3, 1);
            let form = e.call(WORD_AT_8, &args![equipped]).u32();
            let maximum = if e.call(GET_FORM_HEALTH, &args![form]).u32() != 0 {
                let form = e.call(WORD_AT_8, &args![equipped]).u32();
                e.call(GET_FORM_HEALTH, &args![form]).u32()
            } else {
                1
            };
            let health = e.call(GET_ITEM_HEALTH, &args![equipped, 0u32]).f64();
            let fraction = (health / maximum as f64) as f32;
            let value = clamp_unit(e, fraction);
            let meter = e.mem.u32(m + 0x64);
            e.call(HUD_UPDATE_METER, &args![meter, value, 0u32, 0u32, 0u32]);
            set_tile_int(e, m, 0x5c, 0xfa3, 1);
        } else {
            set_tile_int(e, m, 0x60, 0xfa3, 0);
            set_tile_int(e, m, 0x64, 0xfa3, 0);
            set_tile_int(e, m, 0x68, 0xfc4, 0);
            set_tile_int(e, m, 0x68, 0xfa3, 0);
            set_tile_int(e, m, 0x5c, 0xfa3, 0);
            if e.call(WEAPON_KIND, &args![weapon]).u32() == 0xa
                || e.call(WEAPON_KIND, &args![weapon]).u32() == 0xb
                || e.call(WEAPON_KIND, &args![weapon]).u32() == 0xd
            {
                set_tile_int(e, m, 0x5c, 0xfa3, 1);
            }
        }
    }
    e.with_stack(0x10, |e, buffer| {
        let buffer = buffer.addr();
        let total = load_f32(e, m + 0xf4) as f64;
        let total = ftol(e, total);
        let clip = load_f32(e, m + 0xf0) as f64;
        let clip = ftol(e, clip);
        e.call(
            SPRINTF_S,
            &args![buffer, 0x10u32, TEXT_AMMO_FORMAT, clip, total],
        );
        let tile = e.mem.u32(m + 0x5c);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, buffer, 1u32]);
    });
    reloaded
}

/// The node of the parts list (`GetVatsTarget` of the target reference)
/// whose part has the same actor value as the part of `node`, the last one
/// when several do (0 when none).
fn same_value_node(e: &mut Engine, this: Ptr<VatsMenu>, node: u32) -> u32 {
    let mut found = 0;
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let mut other = target_parts(e, this, reference);
    while other != 0 && !e.call(LIST_IS_EMPTY, &args![other]).bool() {
        let other_part = node_part(e, other);
        let other_value = part_value(e, this, other_part);
        let part = node_part(e, node);
        if other_value == part_value(e, this, part) {
            found = other;
        }
        other = e.call(NEXT_NODE, &args![other]).u32();
    }
    found
}

/// Makes the part of `node` the current limb with the stick: its value
/// becomes the chosen one, `bStickReset` is cleared, the current limb's
/// meter fades from its value to 255.0 over 0.2, and `pCurrentLimb` = the
/// part.
fn stick_select(e: &mut Engine, this: Ptr<VatsMenu>, node: u32) -> u32 {
    let m = this.addr();
    let part = node_part(e, node);
    let chosen = part_value(e, this, part);
    e.mem.set_u8(m + 0x10c, 0);
    let meter = e.mem.u32(e.mem.u32(m + 0x100) + 0x30);
    e.call(REMOVE_FADE_CONTROL, &args![meter, 0xfa9u32]);
    let time = load_f32(e, FLOAT_POINT_TWO);
    let full = load_f32(e, FLOAT_255);
    let meter = e.mem.u32(e.mem.u32(m + 0x100) + 0x30);
    let current = e.call(GET_FLOAT_TRAIT, &args![meter, 0xfa9u32]).f64() as f32;
    let meter = e.mem.u32(e.mem.u32(m + 0x100) + 0x30);
    e.call(
        ADD_FADE_CONTROL,
        &args![meter, 0xfa9u32, current, full, time, 0u32],
    );
    let part = node_part(e, node);
    e.mem.set_u32(m + 0x100, part);
    chosen
}

/// Whether the parts of `first` and `second` (list nodes) are the same.
fn same_part(e: &mut Engine, first: u32, second: u32) -> bool {
    let first = e.call(IDENTITY, &args![first]).u32();
    let second = e.call(IDENTITY, &args![second]).u32();
    e.mem.u32(first) == e.mem.u32(second)
}

// Translated from 007f3070 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: picks the current limb from the controls.
///
/// Outside VATS mode 3, without a usable weapon or against a destructible
/// object only `011a59f0` is set (-1 for a destructible object or when
/// the target's body part data has no second part, else 0x19). With the
/// gamepad the left stick (axes 7 and 8, turned by the camera's rotation
/// at `+0x68`) picks the part whose plane position lies within 45 degrees
/// of the stick and nearest to the current one, or else the one with the
/// smallest angle (the parts' `fAngleToStick`), once the stick has been
/// centred (`bStickReset`, a quarter of `0x7fff`) and pushed beyond three
/// quarters. Otherwise control `0x11` or the wheel forward moves to the
/// next part and control `0x1f` or the wheel back to the previous one,
/// skipping parts whose meter is hidden (trait `0xfa3`), and the limb is
/// shown (`ShowTargetLimb(0)`). `011a59f0` keeps the chosen limb's actor
/// value.
pub fn fn_007f3070(e: &mut Engine, this: Ptr<VatsMenu>) {
    let m = this.addr();
    let holder = e.global::<u32>(CONTROLS_HOLDER);
    let controls = e.call(CONTROLS, &args![holder]).u32();
    let weapon = player_weapon(e);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let target = vats_menu_get_vats_target(e, this, reference);
    if e.call(WORD_AT_8, &args![VATS]).u32() != 3 {
        let whole_target = player_process_1a8(e)
            || {
                let node = e.call(IDENTITY, &args![target]).u32();
                e.mem.u32(e.mem.u32(node) + 4) == 2
            }
            || weapon == 0
            || e.call(CHECK_006450C0, &args![weapon]).bool();
        if whole_target {
            let node = e.call(IDENTITY, &args![target]).u32();
            let item = e.mem.u32(node);
            if e.mem.u32(item + 4) == 2 {
                e.set_global::<i32>(GLOBAL_011A59F0, -1);
                return;
            }
            let node = e.call(IDENTITY, &args![target]).u32();
            let object = e.mem.u32(e.mem.u32(node));
            let base = e.call(BASE_FORM_OF, &args![object]).u32();
            let body_parts = e.vcall(base, 0x180, &[]).u32();
            let parts = e.call(BODY_PART_ARRAY, &args![body_parts]).u32();
            let value = if e.mem.u32(parts + 4) != 0 { 0x19 } else { -1 };
            e.set_global::<i32>(GLOBAL_011A59F0, value);
            return;
        }
    }
    if gamepad_in_use(e) {
        let horizontal = e.call(AXIS_VALUE, &args![controls, 0u32, 7u32]).i32();
        let vertical = e.call(AXIS_VALUE, &args![controls, 0u32, 8u32]).i32();
        let range = 0x7fffi32;
        if e.call(ABS, &args![vertical]).i32() < range / 4
            && e.call(ABS, &args![horizontal]).i32() < range / 4
        {
            e.mem.set_u8(m + 0x10c, 1);
        }
        e.with_stack(0x24 + 0xc * 4, |e, block| {
            let rotation = block.addr();
            let stick = rotation + 0x24;
            let rotated = stick + 0xc;
            let direction = rotated + 0xc;
            let difference = direction + 0xc;
            let holder = e.call(CAMERA_HOLDER, &[]).u32();
            let camera = e.call(CAMERA_OF, &args![holder]).u32();
            let source = e.call(CAMERA_ROTATION, &args![camera]).u32();
            for word in 0..9 {
                let value = e.mem.u32(source + 4 * word);
                e.mem.set_u32(rotation + 4 * word, value);
            }
            let down = horizontal.wrapping_neg() as f32;
            let across = vertical.wrapping_neg() as f32;
            e.call(POINT3_SET, &args![stick, 0.0f32, across, down]);
            let product = e
                .call(MATRIX_TIMES_POINT, &args![rotation, rotated, stick])
                .u32();
            for word in 0..3 {
                let value = e.mem.u32(product + 4 * word);
                e.mem.set_u32(stick + 4 * word, value);
            }
            e.call(POINT3_NORMALIZE, &args![stick]);
            let pushed = e.mem.u8(m + 0x10c) != 0
                && (e.call(ABS, &args![vertical]).i32() > range * 3 / 4
                    || e.call(ABS, &args![horizontal]).i32() > range * 3 / 4);
            if !pushed {
                return;
            }
            // The code also keeps the best angle (from `0106b158`, 180.0) in this
            // loop without reading it; the fallback below starts it again.
            let mut best_distance = load_f32(e, FLOAT_LARGEST);
            let mut chosen = e.global::<u32>(GLOBAL_011A59F0);
            let reference = e.global::<u32>(TARGET_REFERENCE);
            let mut node = target_parts(e, this, reference);
            let current = e.mem.u32(m + 0x100);
            let mut found = false;
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                e.call(IDENTITY, &args![direction]);
                let part = node_part(e, node);
                let result = e
                    .call(POINT_SUBTRACT, &args![current + 8, difference, part + 8])
                    .u32();
                for word in 0..3 {
                    let value = e.mem.u32(result + 4 * word);
                    e.mem.set_u32(direction + 4 * word, value);
                }
                let distance = e.call(POINT_LENGTH, &args![direction]).f64() as f32;
                e.call(POINT3_NORMALIZE, &args![direction]);
                let dot = e.call(POINT3_DOT, &args![stick, direction]).f64() as f32;
                let angle = e.call(FLOAT_FUNCTION_4F6E40, &args![dot]).f64() as f32;
                let angle = (angle as f64 * e.global::<f64>(DOUBLE_DEGREES)) as f32;
                let same = same_value_node(e, this, node);
                let part = node_part(e, node);
                e.mem.set_f32(part + 0x34, quiet_f32(angle));
                // `TEST AH,5; JP` (below 45), `TEST AH,0x44; JNP` (not 0.0),
                // `FCOMPP; TEST AH,0x41; JNZ` (nearer than the best).
                if (angle as f64) < e.global::<f64>(DOUBLE_45)
                    && distance as f64 != double_zero(e)
                    && above(best_distance as f64, distance as f64)
                    && same != 0
                    && same_part(e, same, node)
                {
                    found = true;
                    best_distance = quiet_f32(distance);
                    chosen = stick_select(e, this, node);
                }
                node = e.call(NEXT_NODE, &args![node]).u32();
            }
            if !found {
                let mut best_angle = load_f32(e, FLOAT_180);
                let reference = e.global::<u32>(TARGET_REFERENCE);
                let mut node = target_parts(e, this, reference);
                while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                    let same = same_value_node(e, this, node);
                    let part = node_part(e, node);
                    let angle = load_f32(e, part + 0x34);
                    if above(best_angle as f64, angle as f64)
                        && current != node_part(e, node)
                        && same != 0
                        && same_part(e, same, node)
                    {
                        let part = node_part(e, node);
                        best_angle = load_f32(e, part + 0x34);
                        chosen = stick_select(e, this, node);
                    }
                    node = e.call(NEXT_NODE, &args![node]).u32();
                }
            }
            e.set_global::<u32>(GLOBAL_011A59F0, chosen);
        });
        return;
    }
    let forward = e
        .call(CONTROL_PRESSED, &args![controls, 0x11u32, 1u32])
        .u32()
        != 0
        || {
            let object = e.call(OBJECT_004B7210, &[]).u32();
            let wheel = e.call(WHEEL_VALUE, &args![object]).i32();
            // `TEST AH,0x41; JNZ`: forward when above 0.0.
            above(wheel as f64, double_zero(e))
        };
    if forward {
        loop {
            let reference = e.global::<u32>(TARGET_REFERENCE);
            let mut node = target_parts(e, this, reference);
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                if e.call(NEXT_NODE, &args![node]).u32() != 0
                    && node_part(e, node) == e.mem.u32(m + 0x100)
                {
                    let next = e.call(NEXT_NODE, &args![node]).u32();
                    let part = node_part(e, next);
                    e.mem.set_u32(m + 0x100, part);
                    break;
                }
                if e.call(NEXT_NODE, &args![node]).u32() == 0 {
                    let reference = e.global::<u32>(TARGET_REFERENCE);
                    let first = target_parts(e, this, reference);
                    let part = node_part(e, first);
                    e.mem.set_u32(m + 0x100, part);
                }
                node = e.call(NEXT_NODE, &args![node]).u32();
            }
            if select_shown_limb(e, this) {
                break;
            }
        }
        vats_menu_show_target_limb(e, 0);
        return;
    }
    let backward = e
        .call(CONTROL_PRESSED, &args![controls, 0x1fu32, 1u32])
        .u32()
        != 0
        || {
            let object = e.call(OBJECT_004B7210, &[]).u32();
            let wheel = e.call(WHEEL_VALUE, &args![object]).i32();
            // `TEST AH,5; JP`: back when below 0.0.
            (wheel as f64) < double_zero(e)
        };
    if !backward {
        return;
    }
    loop {
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let mut node = target_parts(e, this, reference);
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            if e.call(NEXT_NODE, &args![node]).u32() != 0 && {
                let next = e.call(NEXT_NODE, &args![node]).u32();
                node_part(e, next) == e.mem.u32(m + 0x100)
            } {
                let part = node_part(e, node);
                e.mem.set_u32(m + 0x100, part);
                break;
            }
            if e.call(NEXT_NODE, &args![node]).u32() == 0 {
                let part = node_part(e, node);
                e.mem.set_u32(m + 0x100, part);
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
        if select_shown_limb(e, this) {
            break;
        }
    }
    vats_menu_show_target_limb(e, 0);
}

/// `011a59f0` = the actor value of `pCurrentLimb`'s limb; whether its meter
/// is shown (`Tile::IsTrue(0xfa3)`).
fn select_shown_limb(e: &mut Engine, this: Ptr<VatsMenu>) -> bool {
    let m = this.addr();
    let limb = current_limb_of(e, m);
    let value = vats_menu_get_actor_value_from_body_part(e, this, limb);
    e.set_global::<u32>(GLOBAL_011A59F0, value);
    let meter = e.mem.u32(e.mem.u32(m + 0x100) + 0x30);
    e.call(TILE_IS_TRUE, &args![meter, 0xfa3u32]).bool()
}

// Translated from 007f3b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (limb): with the menu open, makes the first part of the current
/// target with that limb (`eLimb`) the menu's `pCurrentLimb`.
pub fn fn_007f3b40(e: &mut Engine, limb: u32) {
    let menu = vats_menu(e);
    if menu == 0 {
        return;
    }
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let mut node = target_parts(e, Ptr::new(menu), reference);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        if node_part(e, node) != 0 && {
            let part = node_part(e, node);
            e.mem.u32(part + 0x20) == limb
        } {
            let part = node_part(e, node);
            let menu = vats_menu(e);
            e.mem.set_u32(menu + 0x100, part);
            return;
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
}

// Translated from 007f3bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (flag): with the menu open, sets its `bHeadingReached`.
pub fn fn_007f3bd0(e: &mut Engine, flag: u8) {
    let menu = vats_menu(e);
    if menu != 0 {
        e.mem.set_u8(menu + 0xf8, flag);
    }
}

// Translated from 007f3bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::GetActorValueFromBodyPart` (Xbox PDB): (limb): the actor
/// value of the target's body part for the limb (`+0x63` byte, sign
/// extended), when the target reference answers slot `0x100` and has the
/// part; otherwise -1 for limb 14 and `0x1a` else (`0x1a` also without an
/// actor target).
pub fn vats_menu_get_actor_value_from_body_part(
    e: &mut Engine,
    _this: Ptr<VatsMenu>,
    limb: u32,
) -> u32 {
    let reference = e.global::<u32>(TARGET_REFERENCE);
    if reference == 0 || !e.vcall(reference, 0x100, &[]).bool() {
        return 0x1a;
    }
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let base = e.call(BASE_FORM_OF, &args![reference]).u32();
    let body_parts = e.vcall(base, 0x180, &[]).u32();
    if body_parts != 0 {
        let part = e
            .call(GET_BODY_PART_BY_LIMB, &args![body_parts, limb])
            .u32();
        if part != 0 {
            return e.call(BODY_PART_VALUE, &args![part]).u8() as i8 as i32 as u32;
        }
    }
    if limb == 0xe {
        0xffff_ffff
    } else {
        0x1a
    }
}

// Translated from 007f3c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::GetVatsTarget` (Xbox PDB): (reference): the node of the
/// target list at `011db150` whose target's `pTarget` is the reference, or
/// 0 (also for no reference).
pub fn vats_menu_get_vats_target(e: &mut Engine, _this: Ptr<VatsMenu>, reference: u32) -> u32 {
    if reference == 0 {
        return 0;
    }
    let mut node = LIST_011DB150;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        if node_reference(e, node) == reference {
            return node;
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    0
}

// Translated from 007f3cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::GetTargetPart` (Xbox PDB): the node of the current target's
/// parts holding `pCurrentLimb`, or 0.
pub fn vats_menu_get_target_part(e: &mut Engine, this: Ptr<VatsMenu>) -> u32 {
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let mut node = target_parts(e, this, reference);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        if node_part(e, node) == e.mem.u32(this.addr() + 0x100) {
            return node;
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    0
}

// Translated from 007f3d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::FindTargetPart` (Xbox PDB): (actor value): for an actor
/// target (slot `0x100`), the node of its parts whose limb has that actor
/// value, or 0.
pub fn vats_menu_find_target_part(e: &mut Engine, this: Ptr<VatsMenu>, value: u32) -> u32 {
    let reference = e.global::<u32>(TARGET_REFERENCE);
    if !e.vcall(reference, 0x100, &[]).bool() {
        return 0;
    }
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let target = vats_menu_get_vats_target(e, this, reference);
    let node = e.call(IDENTITY, &args![target]).u32();
    let item = e.mem.u32(node);
    if item == 0 {
        return 0;
    }
    let mut node = item.wrapping_add(8);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let part = node_part(e, node);
        if part_value(e, this, part) == value {
            return node;
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    0
}

/// Whether the limb is shown at once after the scan (`ShowTargetLimb(1)`):
/// a destructible target, the player's `+0x68` object answering its slot
/// `0x1a8`, no weapon, or a weapon `006450c0` accepts.
fn scan_shows_limb(e: &mut Engine, item: u32, weapon: u32) -> bool {
    e.mem.u32(item + 4) == 2
        || player_process_1a8(e)
        || weapon == 0
        || e.call(CHECK_006450C0, &args![weapon]).bool()
}

/// The end of the scan in `007f3e00`: the meters' text (`007f0ea0`),
/// `00800a90`, and the current limb: the part of actor value 0x19 when it
/// is within 300 of the player's scene object on a new target, else on the
/// same target as before (`011db1cc`) the part of the remembered value
/// `011a59f0` (or the first part), else a limb-0 part (value `0x1a`), else
/// the current limb's value is remembered. The target is remembered.
fn scan_pick_limb(e: &mut Engine, this: Ptr<VatsMenu>, item: u32, limb_zero: u32) {
    let m = this.addr();
    fn_007f0ea0(e, this);
    let object = e.call(OBJECT_00705910, &[]).u32();
    e.call(SET_00800A90, &args![object]);
    let found = vats_menu_find_target_part(e, this, 0x19);
    let near = found != 0
        && {
            let player = e.global::<u32>(PLAYER);
            let scene = e.vcall(player, 0x1d0, &[]).u32();
            let position = e.call(OBJECT_WORLD_POSITION, &args![scene]).u32();
            e.with_stack(0xc, |e, local| {
                let part = node_part(e, found);
                let difference = e
                    .call(POINT_SUBTRACT, &args![part + 0x14, local.addr(), position])
                    .u32();
                let distance = e.call(POINT_LENGTH, &args![difference]).f64();
                // `TEST AH,5; JP`: near when below 300.
                distance < e.global::<f64>(DOUBLE_300)
            })
        }
        && item != e.global::<u32>(LAST_SCANNED_TARGET);
    if near {
        let part = node_part(e, found);
        e.mem.set_u32(m + 0x100, part);
    } else if item == e.global::<u32>(LAST_SCANNED_TARGET) {
        let value = e.global::<u32>(GLOBAL_011A59F0);
        let remembered = vats_menu_find_target_part(e, this, value);
        let part = if remembered != 0 {
            node_part(e, remembered)
        } else {
            node_part(e, item + 8)
        };
        e.mem.set_u32(m + 0x100, part);
    } else if limb_zero != 0 {
        e.set_global::<u32>(GLOBAL_011A59F0, 0x1a);
        e.mem.set_u32(m + 0x100, limb_zero);
    } else {
        let limb = current_limb_of(e, m);
        let value = vats_menu_get_actor_value_from_body_part(e, this, limb);
        e.set_global::<u32>(GLOBAL_011A59F0, value);
    }
    e.set_global::<u32>(LAST_SCANNED_TARGET, item);
}

// Translated from 007f3e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: the scan of the target's parts before VATS
/// mode 2. Answers false without a target reference, while closing,
/// when leaving mode 1 (the scene goes back: `00800ac0(0, 0, 0)`,
/// `00800ed0`, `0044ac20(0)`, `ShowTargetLimb(1)`) or without a target.
///
/// In mode 1 the scene is prepared once (`0044ac20(1)`, `00800d50` for
/// each target, `bTargetsLit`) and `00800e50(target)` runs. A target whose
/// chances are calculated only leaves mode 3 for mode 2. Otherwise the
/// target is lit (`00800700`, and `00800ac0(target, 1, 1)`/`008009d0` for
/// a non-actor) unless `007f4810` or `011db0ce` say so, mode 3 starts (its
/// time at `011db0dc`; after 2000 ms the parts without a chance get the
/// average of the calculated ones), the ammo is updated, and the parts
/// are measured one by one (`00800a00` gives the screen percent, the meters
/// are laid out again when a part has no world position yet, `007f06d0`
/// says whether it is on screen, `007f1290` its chance). When every part
/// on screen is measured the limb is picked (`scan_pick_limb`), the
/// target's chances are marked calculated and mode 2 starts.
pub fn fn_007f3e00(e: &mut Engine, this: Ptr<VatsMenu>) -> bool {
    let m = this.addr();
    if e.global::<u32>(TARGET_REFERENCE) == 0 || e.mem.u8(m + 0xfb) != 0 {
        return false;
    }
    if e.call(WORD_AT_8, &args![VATS]).u32() == 1 {
        if e.mem.u8(m + 0xfa) == 0 {
            e.with_stack(4, |e, flag| {
                let flag = flag.addr();
                e.mem.set_u32(flag, 1);
                let object = e.call(OBJECT_00705910, &[]).u32();
                e.call(SCENE_0044AC20, &args![object, flag]);
            });
            let mut node = LIST_011DB150;
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let reference = node_reference(e, node);
                let object = e.call(OBJECT_00705910, &[]).u32();
                e.call(SET_00800D50, &args![object, reference]);
                node = e.call(NEXT_NODE, &args![node]).u32();
            }
            e.mem.set_u8(m + 0xfa, 1);
        }
        let reference = e.global::<u32>(TARGET_REFERENCE);
        let object = e.call(OBJECT_00705910, &[]).u32();
        e.call(SET_00800E50, &args![object, reference]);
        return true;
    }
    if e.mem.u8(m + 0xfa) != 0 {
        e.mem.set_u8(m + 0xfa, 0);
        set_00800ac0(e, 0, 0, 0);
        let object = e.call(OBJECT_00705910, &[]).u32();
        e.call(UPDATE_00800ED0, &args![object]);
        e.with_stack(4, |e, flag| {
            let flag = flag.addr();
            e.mem.set_u32(flag, 0);
            let object = e.call(OBJECT_00705910, &[]).u32();
            e.call(SCENE_0044AC20, &args![object, flag]);
        });
        vats_menu_show_target_limb(e, 1);
        return false;
    }
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let target = vats_menu_get_vats_target(e, this, reference);
    let node = e.call(IDENTITY, &args![target]).u32();
    let item = e.mem.u32(node);
    if item == 0 {
        return false;
    }
    let weapon = player_weapon(e);
    if e.mem.u8(item + 0x10) != 0 {
        if e.call(WORD_AT_8, &args![VATS]).u32() == 3 {
            e.call(VATS_SET_MODE, &args![VATS, 2u32, 0u32]);
        }
        return true;
    }
    let object = e.call(OBJECT_00705910, &[]).u32();
    if !fn_007f4810(e, object) && e.global::<u8>(FLAG_011DB0CE) == 0 {
        let object = e.call(OBJECT_00705910, &[]).u32();
        e.call(SET_00800700, &args![object]);
        let target_reference = e.mem.u32(item);
        if !e.vcall(target_reference, 0x100, &[]).bool() {
            let target_reference = e.mem.u32(item);
            set_00800ac0(e, target_reference, 1, 1);
            let object = e.call(OBJECT_00705910, &[]).u32();
            e.call(SET_008009D0, &args![object]);
        }
    }
    if e.call(WORD_AT_8, &args![VATS]).u32() != 3 {
        e.call(VATS_SET_MODE, &args![VATS, 3u32, 0u32]);
        let time = e.call(TIMER_MILLISECONDS, &args![TIMER]).u32();
        e.set_global::<u32>(RECALCULATION_TIME, time);
    } else {
        let time = e.call(TIMER_MILLISECONDS, &args![TIMER]).u32();
        if time.wrapping_sub(e.global::<u32>(RECALCULATION_TIME)) > 2000 {
            let mut count = 0i32;
            let mut sum = 0.0f32;
            let mut node = item.wrapping_add(8);
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let part = node_part(e, node);
                if e.mem.u8(part + 0x2d) != 0 {
                    count = count.wrapping_add(1);
                    let part = node_part(e, node);
                    sum = (sum as f64 + load_f32(e, part + 0x28) as f64) as f32;
                }
                node = e.call(NEXT_NODE, &args![node]).u32();
            }
            let object = e.call(OBJECT_00705910, &[]).u32();
            e.call(SET_008009D0, &args![object]);
            let mut node = item.wrapping_add(8);
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let part = node_part(e, node);
                if e.mem.u8(part + 0x2d) == 0 {
                    let part = node_part(e, node);
                    // `TEST AH,0x41; JP`: the average when not above 0.0.
                    if at_most(load_f32(e, part + 0x28) as f64, double_zero(e)) {
                        let average = sum as f64 / count as f64;
                        let part = node_part(e, node);
                        e.mem.set_f32(part + 0x28, average as f32);
                    }
                    let part = node_part(e, node);
                    e.mem.set_u8(part + 0x2d, 1);
                    let part = node_part(e, node);
                    e.mem.set_u8(part + 0x2e, 1);
                    let part = node_part(e, node);
                    e.mem.set_u8(part + 0x2f, 0);
                }
                node = e.call(NEXT_NODE, &args![node]).u32();
            }
            let time = e.call(TIMER_MILLISECONDS, &args![TIMER]).u32();
            e.set_global::<u32>(RECALCULATION_TIME, time);
        }
    }
    vats_menu_update_ammo(e, this);
    let mut limb_zero = 0u32;
    let mut node = item.wrapping_add(8);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let part = node_part(e, node);
        if e.mem.u32(part + 0x20) == 0 {
            limb_zero = node_part(e, node);
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    let mut pending = item.wrapping_add(8);
    while pending != 0 && !e.call(LIST_IS_EMPTY, &args![pending]).bool() {
        let part = node_part(e, pending);
        let skip = e.mem.u8(part + 0x2d) != 0
            || {
                let part = node_part(e, pending);
                e.mem.u8(part + 0x2e) != 0
            }
            || (e.mem.u8(m + 0xf8) != 0 && {
                let part = node_part(e, pending);
                e.mem.u8(part + 0x2c) == 0
            });
        if !skip {
            break;
        }
        pending = e.call(NEXT_NODE, &args![pending]).u32();
    }
    let mut all_measured = true;
    let mut node = item.wrapping_add(8);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let part = node_part(e, node);
        if e.mem.u8(part + 0x2d) == 0 && {
            let part = node_part(e, node);
            e.mem.u8(part + 0x2c) != 0
        } {
            all_measured = false;
            break;
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    if all_measured && (pending == 0 || e.call(LIST_IS_EMPTY, &args![pending]).bool()) {
        scan_pick_limb(e, this, item, limb_zero);
        e.mem.set_u8(item + 0x10, 1);
        e.set_global::<u8>(FLAG_011DB0CE, 0);
        e.call(VATS_SET_MODE, &args![VATS, 2u32, 0u32]);
        let object = e.call(OBJECT_00705910, &[]).u32();
        e.call(SET_008009D0, &args![object]);
        if scan_shows_limb(e, item, weapon) {
            vats_menu_show_target_limb(e, 1);
        }
        vats_menu_update_chance_to_hit(e, this);
        return true;
    }
    if pending != 0 && node_part(e, pending) != 0 {
        let part = node_part(e, pending);
        e.mem.set_u32(m + 0x100, part);
        let part = node_part(e, pending);
        if e.mem.u8(part + 0x2e) == 0 {
            vats_menu_show_target_limb(e, 1);
            let part = node_part(e, pending);
            e.mem.set_u8(part + 0x2e, 1);
        }
    }
    let mut ready = true;
    let mut node = item.wrapping_add(8);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let part = node_part(e, node);
        let measured = e.mem.u8(part + 0x2e) != 0
            && e.with_stack(4, |e, value| {
                let value = value.addr();
                e.mem.set_f32(value, 0.0);
                let limb = e.mem.u32(part + 0x20);
                let object = e.call(OBJECT_00705910, &[]).u32();
                let answered = e
                    .call(SCREEN_PERCENT_00800A00, &args![object, limb, value])
                    .bool();
                if answered {
                    let percent = load_f32(e, value);
                    e.mem.set_f32(part + 0x24, percent);
                }
                answered
            });
        if measured {
            if e.call(POINT3_EQUALS, &args![part + 0x14, POINT_ZERO])
                .bool()
            {
                fn_007f1840(e, this);
            }
            if !fn_007f06d0(e, this, part) {
                e.mem.set_u8(part + 0x2f, 1);
                e.mem.set_u8(part + 0x2e, 0);
            } else {
                e.mem.set_u8(part + 0x2d, 1);
            }
            if e.mem.u8(part + 0x2d) == 0 && e.mem.u8(part + 0x2c) != 0 {
                ready = false;
            }
            fn_007f1290(e, this, node);
            if ready && e.call(NEXT_NODE, &args![node]).u32() == 0 {
                scan_pick_limb(e, this, item, limb_zero);
                e.call(VATS_SET_MODE, &args![VATS, 2u32, 0u32]);
                let object = e.call(OBJECT_00705910, &[]).u32();
                e.call(SET_008009D0, &args![object]);
                vats_menu_update_chance_to_hit(e, this);
                if scan_shows_limb(e, item, weapon) {
                    vats_menu_show_target_limb(e, 1);
                }
                e.mem.set_u8(item + 0x10, 1);
                e.set_global::<u8>(FLAG_011DB0CE, 0);
                return true;
            }
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    let player = e.global::<u32>(PLAYER);
    e.call(UPDATE_CAMERA, &args![player, 0u32, 0u32]);
    let player = e.global::<u32>(PLAYER);
    e.call(UPDATE_FIRST_PERSON_ZOOM, &args![player]);
    true
}

// Translated from 007f4810 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on `00705910`'s object: its byte at `+0x2B4`.
pub fn fn_007f4810(e: &mut Engine, this: u32) -> bool {
    e.mem.u8(this + 0x2b4) != 0
}

/// The failure path of the template lookups: prints `MENUS: VATS Menu
/// Creation Failed...` (`005b5e40`), closes all menus
/// (`Interface::EmergencyCloseAllMenusAndBreakStuff`) and the VATS menu.
fn menu_creation_failed(e: &mut Engine) {
    e.call(PRINT_005B5E40, &args![TEXT_MENU_CREATION_FAILED]);
    e.call(EMERGENCY_CLOSE_ALL_MENUS, &[]);
    vats_menu_close(e);
}

// Translated from 007f4830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::VatsMenuRenderTemplate` (Xbox PDB), cdecl (parent, template
/// name): `Menu::RenderTemplate(parent, template, 0)` on the open menu; without a
/// result the menu creation failed (see `menu_creation_failed`) and 0 is
/// answered (also without a menu).
pub fn vats_menu_vats_menu_render_template(e: &mut Engine, parent: u32, template: u32) -> u32 {
    let menu = vats_menu(e);
    if menu == 0 {
        return 0;
    }
    let tile = e
        .call(MENU_RENDER_TEMPLATE, &args![menu, parent, template, 0u32])
        .u32();
    if tile != 0 {
        return tile;
    }
    menu_creation_failed(e);
    0
}

// Translated from 007f4890 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (tile, path): the tile the path names from `tile` (`00a08b20`);
/// without one the menu creation failed (see `menu_creation_failed`) and 0
/// is answered.
pub fn fn_007f4890(e: &mut Engine, tile: u32, path: u32) -> u32 {
    let found = e.call(TILE_FROM_PATH, &args![tile, path]).u32();
    if found != 0 {
        return found;
    }
    menu_creation_failed(e);
    0
}

/// `damage - Actor::DifficultyLevelAdjustHealthModifier(-damage of the
/// command (`009b4fe0`), player)` on the target reference, as a float.
fn subtract_adjusted_damage(e: &mut Engine, damage: f32, command: u32) -> f32 {
    let raw = e.call(COMMAND_DAMAGE, &args![command]).f64() as f32;
    let player = e.global::<u32>(PLAYER);
    let reference = e.global::<u32>(TARGET_REFERENCE);
    let adjusted = e
        .call(
            DIFFICULTY_ADJUST_HEALTH_MODIFIER,
            &args![reference, -raw, player],
        )
        .f64();
    (damage as f64 - adjusted) as f32
}

// Translated from 007f48e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: the probable damage to an actor target as a
/// fraction of its base health (0.0 for other targets): the damage of the
/// queued commands at the target (`009b4fe0`, adjusted by the difficulty,
/// negated) plus that of a command built for the current limb with the
/// weapon's attack action. With no health (actor value `0x10` is 0) the
/// answer is 0.0; above the health (`TEST AH,0x41`) it is the health
/// itself; otherwise the damage over slot 0's value of `0x10`.
// Translation incomplete: 007f48e0 - the C++ exception-unwinding frame (FS:[0]) that destroys the local VATS_COMMAND is not translated
pub fn fn_007f48e0(e: &mut Engine, this: Ptr<VatsMenu>) -> f32 {
    let m = this.addr();
    let reference = e.global::<u32>(TARGET_REFERENCE);
    if reference == 0 || !e.vcall(reference, 0x100, &[]).bool() {
        return 0.0;
    }
    let mut damage = 0.0f32;
    let mut node = e.call(IDENTITY, &args![VATS]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let command = node_part(e, node);
        if e.mem.u32(command + 0xc) == e.global::<u32>(TARGET_REFERENCE) {
            let command = node_part(e, node);
            damage = subtract_adjusted_damage(e, damage, command);
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    let weapon = player_weapon(e);
    let kind = if weapon != 0 {
        e.call(WEAPON_KIND, &args![weapon]).u32()
    } else {
        0
    };
    let action = e.call(GET_ATTACK_ACTION, &args![kind]).u32();
    e.with_stack(0x28, |e, command| {
        let command = command.addr();
        e.call(VATS_COMMAND_CONSTRUCT, &args![command]);
        e.mem.set_u32(command, action);
        e.mem.set_u8(command + 4, 1);
        let reference = e.global::<u32>(TARGET_REFERENCE);
        e.mem.set_u32(command + 0xc, reference);
        let limb = current_limb_of(e, m);
        let value = vats_menu_get_actor_value_from_body_part(e, this, limb);
        e.mem.set_u32(command + 0x10, value);
        let shots = vats_menu_get_shot_burst_count(e, action, 0);
        e.mem.set_u8(command + 8, shots);
        damage = subtract_adjusted_damage(e, damage, command);
        let owner = e.global::<u32>(TARGET_REFERENCE) + 0xa4;
        let result = if e.vcall(owner, 0x08, &args![0x10u32]).i32() == 0 {
            0.0
        } else {
            let owner = e.global::<u32>(TARGET_REFERENCE) + 0xa4;
            let health = e.vcall(owner, 0x08, &args![0x10u32]).i32();
            if above(damage as f64, health as f64) {
                let owner = e.global::<u32>(TARGET_REFERENCE) + 0xa4;
                e.vcall(owner, 0x08, &args![0x10u32]).i32() as f32
            } else {
                let owner = e.global::<u32>(TARGET_REFERENCE) + 0xa4;
                let base = e.vcall(owner, 0x00, &args![0x10u32]).i32();
                (damage as f64 / base as f64) as f32
            }
        };
        e.call(VATS_COMMAND_DESTRUCT, &args![command]);
        result
    })
}

/// Whether the stick has been held long enough: with the menu open and
/// its `flag` byte set, more than 500 milliseconds (unsigned) since
/// `uiSelectTimer`.
fn select_held_long(e: &mut Engine, flag: u32) -> bool {
    let menu = vats_menu(e);
    if menu == 0 || e.mem.u8(menu + flag) == 0 {
        return false;
    }
    let time = e.call(TIMER_MILLISECONDS, &args![TIMER]).u32();
    let menu = vats_menu(e);
    time.wrapping_sub(e.mem.u32(menu + 0x114)) > 500
}

// Translated from 007f4bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the left select (`bSelectLeftHeld`) has been held over 500 ms.
pub fn fn_007f4bb0(e: &mut Engine) -> bool {
    select_held_long(e, 0x10f)
}

// Translated from 007f4c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the right select (`bSelectRightHeld`) has been held over 500 ms.
pub fn fn_007f4c00(e: &mut Engine) -> bool {
    select_held_long(e, 0x110)
}

// Translated from 007f4c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: (left): turns to the target nearest the player's
/// facing on the right (left false: the smallest positive heading
/// difference below 360) or on the left (the largest negative one above
/// -360), the differences folded above 180 (`TEST AH,0x41`); each better
/// one becomes the target reference, gets the special buttons
/// (`007eb920`) and its first part (or its limb-1 part on an NPC) as the
/// current limb. `UIVATSSelectTarget` plays first.
// Translation incomplete: 007f4c50 - the C++ exception-unwinding frame (FS:[0]) that destroys the sound handle is not translated
pub fn fn_007f4c50(e: &mut Engine, _this: Ptr<VatsMenu>, left: u8) {
    play_sound_then(
        e,
        GET_SOUND_HANDLE_BY_NAME_ON_AUDIO,
        |e| {
            let audio = e.call(AUDIO_INSTANCE, &[]).u32();
            (audio, TEXT_UI_VATS_SELECT_TARGET)
        },
        |e| {
            e.with_stack(0xc, |e, position| {
                let position = position.addr();
                let player = e.global::<u32>(PLAYER);
                let angle = e.vcall(player, 0x2bc, &args![0u32]).f64();
                let facing = (angle * e.global::<f64>(DOUBLE_DEGREES)) as f32;
                let player = e.global::<u32>(PLAYER);
                let source = e.vcall(player, 0x1f4, &[]).u32();
                for word in 0..3 {
                    let value = e.mem.u32(source + 4 * word);
                    e.mem.set_u32(position + 4 * word, value);
                }
                let mut best = if left != 0 { -360.0f32 } else { 360.0 };
                let weapon = player_weapon(e);
                let mut node = LIST_011DB150;
                while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                    let reference = node_reference(e, node);
                    let other = e.vcall(reference, 0x1f4, &[]).u32();
                    let heading = e
                        .call(HEADING_BETWEEN_LOCATIONS, &args![position, other])
                        .f64() as f32;
                    let mut difference = (facing as f64 - heading as f64) as f32;
                    difference = if above(difference as f64, e.global::<f64>(DOUBLE_180)) {
                        (difference as f64 - e.global::<f64>(DOUBLE_360)) as f32
                    } else {
                        difference
                    };
                    let d = difference as f64;
                    let zero = double_zero(e);
                    // `TEST AH,0x41` (above) for the right, `TEST AH,5; JP`
                    // (below) for the left.
                    let better = (above(best as f64, d) && above(d, zero) && left == 0)
                        || ((best as f64) < d && d < zero && left != 0);
                    if better {
                        best = quiet_f32(difference);
                        let reference = node_reference(e, node);
                        e.set_global::<u32>(TARGET_REFERENCE, reference);
                        fn_007eb920(e, weapon);
                        let mut parts = node_parts(e, node);
                        let first = node_part(e, parts);
                        let menu = vats_menu(e);
                        e.mem.set_u32(menu + 0x100, first);
                        while parts != 0 && !e.call(LIST_IS_EMPTY, &args![parts]).bool() {
                            let item = node_part(e, node);
                            if e.mem.u32(item + 4) == 1 && {
                                let part = node_part(e, parts);
                                e.mem.u32(part + 0x20) == 1
                            } {
                                let part = node_part(e, parts);
                                let menu = vats_menu(e);
                                e.mem.set_u32(menu + 0x100, part);
                            }
                            parts = e.call(NEXT_NODE, &args![parts]).u32();
                        }
                    }
                    node = e.call(NEXT_NODE, &args![node]).u32();
                }
            });
        },
    );
}

/// `FISTP dword` with the rounding mode set to truncation (`FLDCW` with
/// `0xc00`): out of range and NaN give the integer indefinite.
fn fistp_truncate(value: f64) -> i32 {
    if value.is_nan() || !(-2_147_483_648.0..2_147_483_648.0).contains(&value.trunc()) {
        i32::MIN
    } else {
        value.trunc() as i32
    }
}

// Translated from 007f5050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::GetShotBurstCount` (Xbox PDB), cdecl (action, long burst):
/// the number of shots of one attack: the weapon's `+0x102` byte (at least
/// 1); for an attack action with a weapon whose `+0x100` bit 2 is set (or
/// no weapon), the truncated product of `GetAttackSpeed` (mod effect 8),
/// `00821640` and `fVATSShotBurstTime` (`fVATSShotLongBurstTime` with
/// `long burst` and a weapon whose `+0x12C` bit `0x800` is set), its low
/// byte, 1 when that is 0.
pub fn vats_menu_get_shot_burst_count(e: &mut Engine, action: u32, long_burst: u8) -> u8 {
    let player = e.global::<u32>(PLAYER);
    let process = e.call(WORD_AT_68, &args![player]).u32();
    let equipped = e.vcall(process, 0x148, &[]).u32();
    let weapon = if equipped != 0 {
        e.call(WORD_AT_8, &args![equipped]).u32()
    } else {
        0
    };
    let shots = if weapon != 0 {
        e.call(WEAPON_102_524B60, &args![weapon]).u8() as i32
    } else {
        0
    };
    let mut count = 1u8;
    if shots > 0 {
        count = shots as u8;
    }
    if action == 0 || (weapon != 0 && !e.call(WEAPON_FLAG_100_BIT_2, &args![weapon]).bool()) {
        return count;
    }
    let setting = if long_burst != 0 && weapon != 0 && fn_007f5190(e, weapon) {
        SETTING_SHOT_LONG_BURST_TIME
    } else {
        SETTING_SHOT_BURST_TIME
    };
    let time = setting_float(e, setting);
    let factor = e.call(WEAPON_VALUE_821640, &args![weapon]).f64() * time as f64;
    let modded = e.call(HAS_MOD_EFFECT, &args![equipped, 8u32]).u8();
    let speed = e
        .call(GET_ATTACK_SPEED, &args![weapon, modded as u32])
        .f64();
    let shots = fistp_truncate(speed * factor) as u8;
    if shots == 0 {
        1
    } else {
        shots
    }
}

// Translated from 007f5190 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on a weapon: whether bit `0x800` of its `+0x12C` word is set.
pub fn fn_007f5190(e: &mut Engine, this: u32) -> bool {
    e.mem.u32(this + 0x12c) & 0x800 != 0
}

// Translated from 007f51b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: (matrix out): the projection that removes the
/// camera's direction `v` (`0045bba0` of the camera): rows `(y*y + z*z,
/// -x*y, -x*z)`, `(-x*y, x*x + z*z, -y*z)`, `(-x*z, -y*z, x*x + y*y)`, each
/// stored through `005df510`.
pub fn fn_007f51b0(e: &mut Engine, _this: Ptr<VatsMenu>, matrix: u32) {
    e.with_stack(0x18, |e, block| {
        let direction = block.addr();
        let row = direction + 0xc;
        let holder = e.call(CAMERA_HOLDER, &[]).u32();
        let camera = e.call(CAMERA_OF, &args![holder]).u32();
        e.call(CAMERA_DIRECTION, &args![camera, direction]);
        e.call(IDENTITY, &args![row]);
        let x = load_f32(e, direction) as f64;
        let y = load_f32(e, direction + 4) as f64;
        let z = load_f32(e, direction + 8) as f64;
        let rows = [
            [y * y + z * z, -x * y, -x * z],
            [-x * y, x * x + z * z, -y * z],
            [-x * z, -y * z, x * x + y * y],
        ];
        for (index, values) in rows.iter().enumerate() {
            for (column, value) in values.iter().enumerate() {
                e.mem.set_f32(row + 4 * column as u32, *value as f32);
            }
            e.call(MATRIX_SET_ROW, &args![matrix, index as u32, row]);
        }
    });
}

// Translated from 007f5280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu's temporary `VATS_COMMAND` (`+0x11C`) while it holds data
/// (`bDataInTemporaryVATSCommand`), else 0 (also without a menu).
pub fn fn_007f5280(e: &mut Engine) -> u32 {
    let menu = vats_menu(e);
    if menu != 0 && e.mem.u8(menu + 0x118) != 0 {
        menu + 0x11c
    } else {
        0
    }
}

/// The size of `007f52c0`'s frame: its locals sit at their `EBP - offset`
/// addresses inside a block of this size (`frame_slot`).
const TARGETS_FRAME: u32 = 0xa60;

/// The address of the local at `EBP - offset` in the frame block `frame`.
fn frame_slot(frame: u32, offset: u32) -> u32 {
    frame + TARGETS_FRAME - offset
}

/// A new `VATSMenu::ChanceToHit` (0x3C bytes, the allocator then `007f6820`);
/// the pointer is kept in the frame slot `slot`. The callers set its
/// fields (`reset_part`).
fn new_part(e: &mut Engine, slot: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x3cu32]).u32();
    let part = if block != 0 { fn_007f6820(e, block) } else { 0 };
    e.mem.set_u32(slot, part);
    part
}

/// Sets the flags and `WorldCoord` of a new part (see `new_part`).
fn reset_part(e: &mut Engine, part: u32) {
    e.mem.set_u8(part + 0x2c, 0);
    e.mem.set_u8(part + 0x2d, 0);
    e.mem.set_u8(part + 0x2e, 0);
    e.mem.set_u8(part + 0x2f, 1);
    for word in 0..3 {
        let value = e.global::<u32>(POINT_ZERO + 4 * word);
        e.mem.set_u32(part + 0x14 + 4 * word, value);
    }
}

/// A new `VATSMenu::VatsTarget` (0x24 bytes, `007f6900`), kept in the
/// frame slot `slot`.
fn new_target(e: &mut Engine, slot: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x24u32]).u32();
    let target = if block != 0 { fn_007f6900(e, block) } else { 0 };
    e.mem.set_u32(slot, target);
    target
}

/// `005ae3d0` on the list at `list` with the frame slot `slot`, which holds
/// the pointer the code adds to the list.
fn append(e: &mut Engine, list: u32, slot: u32) {
    e.call(LIST_APPEND, &args![list, slot]);
}

/// The frame slots of a cast in `007f52c0`: the query object of
/// `004a3c20` (`input`), the `hkpAllRayHitCollector` (`collector`), the
/// camera position (`from`), a scratch point (`to`) and the words of the
/// filter calls (`filter`, `filter_out`). The field names follow how the
/// code uses them (not confirmed names).
struct Ray {
    input: u32,
    from: u32,
    to: u32,
    filter: u32,
    filter_out: u32,
    collector: u32,
}

/// Builds the ray up to its start point (see [`Ray`]).
fn ray_begin(e: &mut Engine, ray: &Ray) {
    e.call(QUERY_CONSTRUCT_4A3C20, &args![ray.input]);
    e.call(IDENTITY, &args![ray.from]);
    e.call(IDENTITY, &args![ray.to]);
    let holder = e.call(CAMERA_HOLDER, &[]).u32();
    let camera = e.call(CAMERA_OF, &args![holder]).u32();
    let position = e.call(OBJECT_WORLD_POSITION, &args![camera]).u32();
    for word in 0..3 {
        let value = e.mem.u32(position + 4 * word);
        e.mem.set_u32(ray.from + 4 * word, value);
    }
    e.call(STORE_8C71B0, &args![ray.filter, 0u32]);
    e.call(FILTER_4A39F0, &args![ray.filter, 0x25u32]);
    let player = e.global::<u32>(PLAYER);
    let object = e
        .call(COLLISION_OBJECT_931ED0, &args![player, ray.filter_out])
        .u32();
    let group = e.call(GROUP_4A3A20, &args![object]).u32();
    e.call(FILTER_59CE80, &args![ray.filter, group]);
    let filter = e.mem.u32(ray.filter);
    e.call(QUERY_4A3F70, &args![ray.input, filter]);
    e.call(RAY_COLLECTOR_CONSTRUCT, &args![ray.collector]);
    e.call(QUERY_4A3FB0, &args![ray.input, ray.collector]);
    e.call(QUERY_4A3DA0, &args![ray.input, ray.from]);
}

/// Casts the ray (the player's cell's world, slot `0xC8`); whether it hit.
fn ray_cast(e: &mut Engine, ray: &Ray) -> bool {
    let player = e.global::<u32>(PLAYER);
    let cell = e.call(PARENT_CELL_8D6F30, &args![player]).u32();
    let world = e.call(CELL_WORLD_4543C0, &args![cell]).u32();
    e.vcall(world, 0xc8, &args![ray.input]).bool()
}

/// The `index`th hit of the ray's collector, copied to `hit` (0x60 bytes):
/// its collidable's scene object (`GetAVObjectForCollidable` of the word
/// at `hit + 0x50`).
fn ray_hit_object(e: &mut Engine, ray: &Ray, index: u32, hit: u32) -> u32 {
    let hits = e.call(QUERY_8CDD90, &args![ray.input]).u32();
    let array = e.call(ARRAY_460140, &args![hits]).u32();
    let entry = e.call(ARRAY_AT_4A46B0, &args![array, index]).u32();
    e.call(COPY_696C30, &args![hit, entry]);
    let collidable = e.mem.u32(hit + 0x50);
    e.call(GET_AV_OBJECT_FOR_COLLIDABLE, &args![collidable])
        .u32()
}

/// The number of the ray's hits.
fn ray_hit_count(e: &mut Engine, ray: &Ray) -> i32 {
    let hits = e.call(QUERY_8CDD90, &args![ray.input]).u32();
    let array = e.call(ARRAY_460140, &args![hits]).u32();
    e.call(NEXT_NODE, &args![array]).i32()
}

/// The projection of `007f52c0`: the screen position of the world point at
/// `point` into `x`/`y` and whether it is inside, with NaN passing (see
/// `point_on_screen`).
fn project_inside(e: &mut Engine, point: u32, x: u32, y: u32) -> bool {
    e.mem.set_f32(x, 0.0);
    e.mem.set_f32(y, 0.0);
    let epsilon = load_f32(e, SCREEN_EPSILON).to_bits();
    let holder = e.call(CAMERA_HOLDER, &[]).u32();
    let camera = e.call(CAMERA_OF, &args![holder]).u32();
    e.call(WORLD_PT_TO_SCREEN_PT, &args![camera, point, x, y, epsilon]);
    let zero = double_zero(e);
    let one = e.global::<f64>(DOUBLE_ONE);
    let x = load_f32(e, x) as f64;
    let y = load_f32(e, y) as f64;
    let positive = |value: f64| value > zero || value.is_nan();
    let below_one = |value: f64| value < one || value.is_nan();
    positive(x) && positive(y) && below_one(x) && below_one(y)
}

/// The sneak distance limit of `007f52c0`: `fSneakMaxDistance` when the
/// flag of `00425fd0` is set, else `fSneakExteriorDistanceMult *
/// fSneakMaxDistance`; whether `distance` is beyond it (`TEST AH,5; JNP`
/// for the first, `TEST AH,0x41; JZ` / `JNZ` for the product).
fn beyond_sneak_distance(e: &mut Engine, interior: bool, distance: f32) -> bool {
    if interior {
        let limit = setting_float(e, SETTING_SNEAK_MAX_DISTANCE) as f64;
        limit < distance as f64
    } else {
        let multiplier = setting_float(e, SETTING_SNEAK_EXTERIOR_MULT) as f64;
        let limit = multiplier * setting_float(e, SETTING_SNEAK_MAX_DISTANCE) as f64;
        above(distance as f64, limit)
    }
}

// Translated from 007f52c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl: builds the target list (`011db150`) when it is empty; answers
/// true at once when it is not, else whether a target can be seen.
///
/// Actors: every entry of the player's list (`00595d10`) whose reference
/// is alive (slot `0x22c(0)` false), has 3D, a process type 0, is not
/// in slot `0x1a0(0)`, is not a destroyed destructible, lies between 2 and
/// `fVATSMaxEngageDistance` and is in the player's line of sight becomes
/// a target (type 1, `bHostile` from the entry's `+4` byte) with a part per
/// body part, a limb-14 part when its `0x190` lookup and weapon allow,
/// or type 2 with one part (limb -1) when it has no part; targets beyond
/// the sneak distance are taken only when seen on screen (themselves or a
/// body part node). The farthest kept distance (plus 128) bounds the
/// destructible objects found around the player (`005885f0` in a
/// worldspace, `0054da20` in an interior, with the callback `007f4f30`
/// filling `011db124`); each one not an actor, not slot `0x220`, within
/// the distances, not `00477ba0`/`00440da0`/`00440d80`, destructible
/// (`00576070`) and not the player's own (slot `0x224` and `00874480`)
/// becomes a type 2 target; while nothing is seen yet a ray from the
/// camera to it tells whether it is visible. Without a visible target a
/// ray along the camera's view looks for a destructible reference
/// (`BGSDestructibleObjectForm::IsDestructible` and slot `0x224`), and when
/// none is found `007f6920` runs.
// Translation incomplete: 007f52c0 - the C++ exception-unwinding frames (FS:[0]) of the allocations and the ray collectors, and the security cookie, are not translated
pub fn fn_007f52c0(e: &mut Engine) -> bool {
    if !e.call(LIST_IS_EMPTY, &args![LIST_011DB150]).bool() {
        return true;
    }
    e.with_stack(TARGETS_FRAME, |e, block| {
        let frame = block.addr();
        let mut seen = false;
        let holder = e.call(CAMERA_HOLDER, &[]).u32();
        let camera = e.call(CAMERA_OF, &args![holder]).u32();
        e.call(WORLD_TO_CAMERA_MATRIX, &args![camera]);
        let player = e.global::<u32>(PLAYER);
        let mut node = e.call(PLAYER_TARGET_LIST, &args![player]).u32();
        let mut farthest = 0.0f32;
        let player = e.global::<u32>(PLAYER);
        let cell = e.call(PARENT_CELL_8D6F30, &args![player]).u32();
        let interior = e.call(CELL_FLAG_425FD0, &args![cell]).bool();
        e.call(LIST_CLEAR, &args![LIST_011DB124]);

        // 007f5361: the actors.
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let entry = node_part(e, node);
            let mut destroyed = true;
            if entry != 0 {
                let reference = e.mem.u32(entry);
                let base = e.call(FORM_OF_REFERENCE, &args![reference]).u32();
                let destruction = e.call(GET_DESTRUCTION_FORM, &args![base]).u32();
                destroyed = e.call(NEXT_NODE, &args![destruction]).u32() != 0 && {
                    let reference = e.mem.u32(entry);
                    !e.call(IS_DESTRUCTIBLE_576070, &args![reference]).bool()
                };
            }
            let usable = entry != 0
                && {
                    let reference = e.mem.u32(entry);
                    !e.vcall(reference, 0x22c, &args![0u32]).bool()
                }
                && {
                    let reference = e.mem.u32(entry);
                    e.vcall(reference, 0x1d0, &[]).u32() != 0
                }
                && {
                    let reference = e.mem.u32(entry);
                    e.call(GET_CURRENT_PROCESS_TYPE, &args![reference]).u32() == 0
                }
                && {
                    let reference = e.mem.u32(entry);
                    !e.vcall(reference, 0x1a0, &args![0u32]).bool()
                }
                && !destroyed;
            if !usable {
                node = e.call(NEXT_NODE, &args![node]).u32();
                continue;
            }
            let player = e.global::<u32>(PLAYER);
            let reference = e.mem.u32(entry);
            let distance = e
                .call(
                    GET_DISTANCE_FROM_REFERENCE,
                    &args![reference, player, 0u32, 0u32],
                )
                .f64() as f32;
            // `TEST AH,5; JNP`: too near below 2.0, too far beyond the
            // setting.
            let in_range = !((distance as f64) < e.global::<f64>(DOUBLE_TWO))
                && !((setting_float(e, SETTING_VATS_MAX_ENGAGE_DISTANCE) as f64) < distance as f64)
                && {
                    let player = e.global::<u32>(PLAYER);
                    let reference = e.mem.u32(entry);
                    e.call(
                        LINE_OF_SIGHT,
                        &args![player, 1u32, reference, 1u32, 0u32, 1u32],
                    )
                    .bool()
                };
            if !in_range {
                node = e.call(NEXT_NODE, &args![node]).u32();
                continue;
            }
            // `TEST AH,5; JP`: the farthest so far.
            if (farthest as f64) < distance as f64 {
                farthest = quiet_f32(distance);
            }
            let reference = e.mem.u32(entry);
            let scene = e.vcall(reference, 0x1d0, &[]).u32();
            let position = e.call(OBJECT_WORLD_POSITION, &args![scene]).u32();
            let local = frame_slot(frame, 0x40);
            for word in 0..3 {
                let value = e.mem.u32(position + 4 * word);
                e.mem.set_u32(local + 4 * word, value);
            }
            let mut visible =
                project_inside(e, local, frame_slot(frame, 0x44), frame_slot(frame, 0x48));
            if !visible {
                let reference = e.mem.u32(entry);
                let base = e.call(BASE_FORM_OF, &args![reference]).u32();
                let body_parts = e.vcall(base, 0x180, &[]).u32();
                let parts = e.call(BODY_PART_ARRAY, &args![body_parts]).u32();
                for index in 0..0xfu32 {
                    if e.mem.u32(parts + 4 * index) == 0 {
                        continue;
                    }
                    let x = frame_slot(frame, 0x64);
                    let y = frame_slot(frame, 0x60);
                    e.call(IDENTITY, &args![x]);
                    let owner = node_reference(e, node);
                    let part = e
                        .call(GET_BODY_PART_BY_LIMB, &args![body_parts, index])
                        .u32();
                    let name = e.call(BODY_PART_NODE_NAME, &args![part]).u32();
                    let root = e.vcall(owner, 0x1d0, &[]).u32();
                    let found = e.call(FIND_NODE, &args![root, name]).u32();
                    if found == 0 {
                        continue;
                    }
                    let epsilon = load_f32(e, SCREEN_EPSILON).to_bits();
                    let point = e.call(OBJECT_WORLD_POSITION, &args![found]).u32();
                    let holder = e.call(CAMERA_HOLDER, &[]).u32();
                    let camera = e.call(CAMERA_OF, &args![holder]).u32();
                    e.call(WORLD_PT_TO_SCREEN_PT, &args![camera, point, x, y, epsilon]);
                    let zero = double_zero(e);
                    let one = e.global::<f64>(DOUBLE_ONE);
                    let (px, py) = (load_f32(e, x) as f64, load_f32(e, y) as f64);
                    // `TEST AH,0x41; JNZ` then `TEST AH,5; JP`: strictly
                    // inside, NaN fails.
                    if px > zero && py > zero && px < one && py < one {
                        visible = true;
                        seen = true;
                        break;
                    }
                }
            } else {
                seen = true;
            }
            if beyond_sneak_distance(e, interior, distance) && !visible {
                node = e.call(NEXT_NODE, &args![node]).u32();
                continue;
            }
            let reference = e.mem.u32(entry);
            if e.vcall(reference, 0x1d0, &[]).u32() != 0 {
                let target_slot = frame_slot(frame, 0x7c);
                let target = new_target(e, target_slot);
                e.mem.set_u32(target + 4, 1);
                let reference = e.mem.u32(entry);
                e.mem.set_u32(target, reference);
                e.mem.set_u8(target + 0x10, 0);
                let entry = node_part(e, node);
                let hostile = e.mem.u8(entry + 4);
                e.mem.set_u8(target + 0x20, hostile);
                let reference = e.mem.u32(target);
                let base = e.call(BASE_FORM_OF, &args![reference]).u32();
                let body_parts = e.vcall(base, 0x180, &[]).u32();
                let parts = e.call(BODY_PART_ARRAY, &args![body_parts]).u32();
                for index in 0..0xfu32 {
                    if e.mem.u32(parts + 4 * index) == 0 {
                        continue;
                    }
                    let slot = frame_slot(frame, 0x9c);
                    let part = new_part(e, slot);
                    reset_part(e, part);
                    e.mem.set_f32(part + 0x28, 0.0);
                    e.mem.set_u32(part + 0x20, index);
                    append(e, target + 8, slot);
                }
                let reference = e.mem.u32(target);
                let extra = if e.vcall(reference, 0x100, &[]).bool() {
                    let reference = e.mem.u32(target);
                    let process = e.call(WORD_AT_68, &args![reference]).u32();
                    let reference = e.mem.u32(target);
                    let key = e.vcall(reference, 0x1e8, &[]).u32();
                    e.vcall(process, 0x190, &args![key]).u32()
                } else {
                    0
                };
                let reference = e.mem.u32(target);
                let equipped = if e.vcall(reference, 0x100, &[]).bool() {
                    let reference = e.mem.u32(target);
                    let process = e.call(WORD_AT_68, &args![reference]).u32();
                    e.vcall(process, 0x148, &[]).u32()
                } else {
                    0
                };
                let weapon = if equipped != 0 {
                    e.call(WORD_AT_8, &args![equipped]).u32()
                } else {
                    0
                };
                if extra != 0 && weapon != 0 && !e.call(WEAPON_CHECK_46E8C0, &args![weapon]).bool()
                {
                    let slot = frame_slot(frame, 0xc8);
                    let part = new_part(e, slot);
                    reset_part(e, part);
                    e.mem.set_u32(part + 0x20, 0xe);
                    append(e, target + 8, slot);
                }
                if e.call(LIST_IS_EMPTY, &args![target + 8]).bool() {
                    e.mem.set_u32(target + 4, 2);
                    let slot = frame_slot(frame, 0xd8);
                    let part = new_part(e, slot);
                    e.mem.set_u32(part + 0x20, 0xffff_ffff);
                    reset_part(e, part);
                    append(e, target + 8, slot);
                }
                append(e, LIST_011DB150, target_slot);
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }

        // 007f5bee: the destructible objects around the player.
        // `TEST AH,0x44; JNP`: widened unless 0.0.
        if farthest as f64 != double_zero(e) {
            farthest = (farthest as f64 + e.global::<f64>(DOUBLE_128)) as f32;
        }
        let player = e.global::<u32>(PLAYER);
        let in_worldspace = e.call(REFERENCE_GET_WORLDSPACE, &args![player]).u32() != 0;
        let radius = |e: &mut Engine| {
            let multiplier = setting_float(e, SETTING_SNEAK_EXTERIOR_MULT) as f64;
            (multiplier * setting_float(e, SETTING_SNEAK_MAX_DISTANCE) as f64) as f32
        };
        let first_radius = radius(e);
        let player = e.global::<u32>(PLAYER);
        let first_position = e.vcall(player, 0x1f4, &[]).u32();
        let second_radius = radius(e);
        let player = e.global::<u32>(PLAYER);
        let second_position = e.vcall(player, 0x1f4, &[]).u32();
        let player = e.global::<u32>(PLAYER);
        let (space, gather) = if in_worldspace {
            (
                e.call(REFERENCE_GET_WORLDSPACE, &args![player]).u32(),
                WORLDSPACE_GATHER,
            )
        } else {
            (
                e.call(PARENT_CELL_8D6F30, &args![player]).u32(),
                CELL_GATHER,
            )
        };
        let player = e.global::<u32>(PLAYER);
        e.call(
            gather,
            &args![
                space,
                second_position,
                second_radius,
                first_position,
                first_radius,
                GATHER_CALLBACK,
                player
            ],
        );
        let mut node = LIST_011DB124;
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let player = e.global::<u32>(PLAYER);
            let reference = node_part(e, node);
            let distance = e
                .call(
                    GET_DISTANCE_FROM_REFERENCE,
                    &args![reference, player, 0u32, 0u32],
                )
                .f64() as f32;
            let take = !{
                let reference = node_part(e, node);
                e.vcall(reference, 0x100, &[]).bool()
            } && !{
                let reference = node_part(e, node);
                e.vcall(reference, 0x220, &[]).bool()
            } && (!above(farthest as f64, double_zero(e)) || {
                let player = e.global::<u32>(PLAYER);
                let reference = node_part(e, node);
                let again = e
                    .call(
                        GET_DISTANCE_FROM_REFERENCE,
                        &args![reference, player, 0u32, 0u32],
                    )
                    .f64();
                // `FCOMPP; TEST AH,5; JNP`: out when the bound is below.
                !((farthest as f64) < again)
            }) && !{
                let reference = node_part(e, node);
                e.call(CHECK_477BA0, &args![reference]).bool()
            } && !{
                let reference = node_part(e, node);
                e.call(CHECK_440DA0, &args![reference]).bool()
            } && !{
                let reference = node_part(e, node);
                e.call(CHECK_440D80, &args![reference]).bool()
            } && !beyond_sneak_distance(e, interior, distance)
                && {
                    let reference = node_part(e, node);
                    e.call(IS_DESTRUCTIBLE_576070, &args![reference]).bool()
                };
            if !take {
                node = e.call(NEXT_NODE, &args![node]).u32();
                continue;
            }
            let reference = node_part(e, node);
            if e.vcall(reference, 0x224, &[]).bool() && {
                let reference = node_part(e, node);
                e.call(OWNER_874480, &args![reference]).u32() == e.global::<u32>(PLAYER)
            } {
                node = e.call(NEXT_NODE, &args![node]).u32();
                continue;
            }
            let reference = node_part(e, node);
            if !e.call(CHECK_477BA0, &args![reference]).bool() {
                let target_slot = frame_slot(frame, 0x110);
                let target = new_target(e, target_slot);
                e.mem.set_u32(target + 4, 2);
                let reference = node_part(e, node);
                e.mem.set_u32(target, reference);
                e.mem.set_u8(target + 0x10, 0);
                e.mem.set_u8(target + 0x20, 0);
                let slot = frame_slot(frame, 0x120);
                let part = new_part(e, slot);
                e.mem.set_u32(part + 0x20, 0xffff_ffff);
                reset_part(e, part);
                append(e, target + 8, slot);
                if !seen {
                    let reference = node_part(e, node);
                    let scene = e.vcall(reference, 0x1d0, &[]).u32();
                    let position = e.call(OBJECT_WORLD_POSITION, &args![scene]).u32();
                    let local = frame_slot(frame, 0x130);
                    for word in 0..3 {
                        let value = e.mem.u32(position + 4 * word);
                        e.mem.set_u32(local + 4 * word, value);
                    }
                    let on_screen = project_inside(
                        e,
                        local,
                        frame_slot(frame, 0x134),
                        frame_slot(frame, 0x138),
                    );
                    if on_screen {
                        let ray = Ray {
                            input: frame_slot(frame, 0x1f0),
                            from: frame_slot(frame, 0x1fc),
                            to: frame_slot(frame, 0x208),
                            filter: frame_slot(frame, 0x20c),
                            filter_out: frame_slot(frame, 0x210),
                            collector: frame_slot(frame, 0x530),
                        };
                        ray_begin(e, &ray);
                        let reference = node_part(e, node);
                        let scene = e.vcall(reference, 0x1d0, &[]).u32();
                        let bound = e.call(GET_WORLD_BOUND, &args![scene]).u32();
                        let centre = e.call(IDENTITY, &args![bound]).u32();
                        e.call(QUERY_4A3EB0, &args![ray.input, centre]);
                        let reference = node_part(e, node);
                        let scene = e.vcall(reference, 0x1d0, &[]).u32();
                        let bound = e.call(GET_WORLD_BOUND, &args![scene]).u32();
                        let centre = e.call(IDENTITY, &args![bound]).u32();
                        let difference = e
                            .call(
                                POINT_SUBTRACT,
                                &args![centre, frame_slot(frame, 0x544), ray.from],
                            )
                            .u32();
                        fn_007f68a0(e, ray.input, difference);
                        if ray_cast(e, &ray) && e.call(QUERY_8CDD90, &args![ray.input]).u32() != 0 {
                            let mut index = 0i32;
                            while !seen {
                                if index >= ray_hit_count(e, &ray) {
                                    break;
                                }
                                let hit = frame_slot(frame, 0x5b0);
                                let object = ray_hit_object(e, &ray, index as u32, hit);
                                e.mem.set_u32(frame_slot(frame, 0x5b4), object);
                                if object != 0 {
                                    let reference = node_part(e, node);
                                    if object == e.vcall(reference, 0x1d0, &[]).u32() {
                                        seen = true;
                                    }
                                }
                                index = index.wrapping_add(1);
                            }
                        }
                        e.call(RAY_COLLECTOR_DESTRUCT, &args![ray.collector]);
                    }
                }
                append(e, LIST_011DB150, target_slot);
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }

        // 007f64f4: a ray along the view.
        if !seen {
            let ray = Ray {
                input: frame_slot(frame, 0x670),
                from: frame_slot(frame, 0x67c),
                to: frame_slot(frame, 0x688),
                filter: frame_slot(frame, 0x68c),
                filter_out: frame_slot(frame, 0x690),
                collector: frame_slot(frame, 0x9b0),
            };
            ray_begin(e, &ray);
            let direction_out = frame_slot(frame, 0x9bc);
            let holder = e.call(CAMERA_HOLDER, &[]).u32();
            let camera = e.call(CAMERA_OF, &args![holder]).u32();
            let direction = e
                .call(CAMERA_DIRECTION, &args![camera, direction_out])
                .u32();
            e.call(QUERY_4A3EB0, &args![ray.input, direction]);
            let length = if interior {
                setting_float(e, SETTING_SNEAK_MAX_DISTANCE) as f64
            } else {
                let multiplier = setting_float(e, SETTING_SNEAK_EXTERIOR_MULT) as f64;
                multiplier * setting_float(e, SETTING_SNEAK_MAX_DISTANCE) as f64
            } as f32;
            let holder = e.call(CAMERA_HOLDER, &[]).u32();
            let camera = e.call(CAMERA_OF, &args![holder]).u32();
            let direction = e
                .call(CAMERA_DIRECTION, &args![camera, frame_slot(frame, 0x9e4)])
                .u32();
            let scaled = e
                .call(
                    POINT3_SCALE,
                    &args![direction, frame_slot(frame, 0x9d8), length],
                )
                .u32();
            fn_007f68a0(e, ray.input, scaled);
            if ray_cast(e, &ray) && e.call(QUERY_8CDD90, &args![ray.input]).u32() != 0 {
                let mut index = 0i32;
                while !seen {
                    if index >= ray_hit_count(e, &ray) {
                        break;
                    }
                    let hit = frame_slot(frame, 0xa50);
                    let object = ray_hit_object(e, &ray, index as u32, hit);
                    if object != 0 {
                        let reference = e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
                        if reference != 0 {
                            let base = e.call(FORM_OF_REFERENCE, &args![reference]).u32();
                            if e.call(IS_DESTRUCTIBLE, &args![base]).bool()
                                && e.vcall(reference, 0x224, &[]).bool()
                            {
                                seen = true;
                            }
                        }
                    }
                    index = index.wrapping_add(1);
                }
            }
            e.call(RAY_COLLECTOR_DESTRUCT, &args![ray.collector]);
        }
        if !seen {
            fn_007f6920(e);
        }
        seen
    })
}

// Translated from 007f4f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (reference): the callback of the gather calls in `007f52c0`;
/// answers false. With a melee-like weapon (`006450c0` of the player's
/// current weapon) a reference farther than `fVATSMeleeMaxDistance` (`TEST
/// AH,5; JP`: strictly beyond) is left out. A reference that `00452370`
/// accepts, answering its slot `0x224`, whose `00476c90` object (if any)
/// is not of kind 11 or 12 and whose `00874480` actor is not the player is
/// added to the list at `011db124` (`005ae3d0` with the address of the
/// argument).
pub fn fn_007f4f30(e: &mut Engine, reference: u32) -> u8 {
    let distance = e.with_stack(0xc, |e, local| {
        let target_position = e.vcall(reference, 0x1f4, &[]).u32();
        let player = e.global::<u32>(PLAYER);
        let player_position = e.vcall(player, 0x1f4, &[]).u32();
        let difference = e
            .call(
                POINT_SUBTRACT,
                &args![player_position, local.addr(), target_position],
            )
            .u32();
        e.call(POINT_LENGTH, &args![difference]).f64() as f32
    });
    let player = e.global::<u32>(PLAYER);
    if e.call(GET_CURRENT_WEAPON, &args![player]).u32() != 0
        && {
            let player = e.global::<u32>(PLAYER);
            let weapon = e.call(GET_CURRENT_WEAPON, &args![player]).u32();
            e.call(CHECK_006450C0, &args![weapon]).bool()
        }
        && (setting_float(e, SETTING_VATS_MELEE_MAX_DISTANCE) as f64) < distance as f64
    {
        return 0;
    }
    if !e.call(REFERENCE_CHECK_452370, &args![reference]).bool() {
        return 0;
    }
    if !e.vcall(reference, 0x224, &[]).bool() {
        return 0;
    }
    if e.call(REFERENCE_OBJECT_476C90, &args![reference]).u32() != 0 {
        let object = e.call(REFERENCE_OBJECT_476C90, &args![reference]).u32();
        if e.call(WEAPON_KIND, &args![object]).u32() == 0xb {
            return 0;
        }
        let object = e.call(REFERENCE_OBJECT_476C90, &args![reference]).u32();
        if e.call(WEAPON_KIND, &args![object]).u32() == 0xc {
            return 0;
        }
    }
    if e.call(OWNER_874480, &args![reference]).u32() != e.global::<u32>(PLAYER) {
        e.with_stack(4, |e, argument| {
            e.mem.set_u32(argument.addr(), reference);
            e.call(LIST_APPEND, &args![LIST_011DB124, argument.addr()]);
        });
    }
    0
}

// Translated from 007f6820 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall: constructs a `VATSMenu::ChanceToHit` in place (its three
/// points through `006815c0`, `eLimb` -1, `fScreenPercent`, `fChance` and
/// `fAngleToStick` 0.0, `bOnScreen`/`bChanceCalculated` clear,
/// `bFirstTimeShown`/`bNeedsRecalc` set, no meter, `bLeftOfCenter` clear);
/// returns it.
pub fn fn_007f6820(e: &mut Engine, this: u32) -> u32 {
    e.call(IDENTITY, &args![this]);
    e.call(IDENTITY, &args![this + 8]);
    e.call(IDENTITY, &args![this + 0x14]);
    e.mem.set_i32(this + 0x20, -1);
    e.mem.set_f32(this + 0x24, 0.0);
    e.mem.set_f32(this + 0x28, 0.0);
    e.mem.set_u8(this + 0x2c, 0);
    e.mem.set_u8(this + 0x2d, 0);
    e.mem.set_u8(this + 0x2e, 1);
    e.mem.set_u8(this + 0x2f, 1);
    e.mem.set_u32(this + 0x30, 0);
    e.mem.set_f32(this + 0x34, 0.0);
    e.mem.set_u8(this + 0x38, 0);
    this
}

// Translated from 007f68a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the query of `007f52c0`: (point): converts the point
/// (`004a3e00` into a local of 0x1C bytes passed through `006815c0`) and
/// hands it to `004a3f90`.
// Translation incomplete: 007f68a0 - the security cookie (00ec408c) is not translated
pub fn fn_007f68a0(e: &mut Engine, this: u32, point: u32) {
    e.with_stack(0x20, |e, local| {
        let local = local.addr();
        e.call(IDENTITY, &args![local]);
        let converted = e.call(CONVERT_4A3E00, &args![local, point]).u32();
        e.call(QUERY_4A3F90, &args![this, converted]);
    });
}

// Translated from 007f6900 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall: constructs a `VATSMenu::VatsTarget` in place (its part list
/// at `+8` through `0096a2d0`); returns it.
pub fn fn_007f6900(e: &mut Engine, this: u32) -> u32 {
    e.call(LIST_CONSTRUCT_96A2D0, &args![this + 8]);
    this
}

// Translated from 007f6920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the target list at `011db150`: for each first target, frees its
/// parts (the deallocator, then `0063f7b0` on its part list), destroys and
/// frees it (`007f69d0(1)`) and runs `0063f7b0` on the list.
pub fn fn_007f6920(e: &mut Engine) {
    loop {
        let head = e.call(IDENTITY, &args![LIST_011DB150]).u32();
        if e.mem.u32(head) == 0 {
            return;
        }
        let head = e.call(IDENTITY, &args![LIST_011DB150]).u32();
        let target = e.mem.u32(head);
        loop {
            let parts = e.call(IDENTITY, &args![target + 8]).u32();
            if e.mem.u32(parts) == 0 {
                break;
            }
            let parts = e.call(IDENTITY, &args![target + 8]).u32();
            let part = e.mem.u32(parts);
            e.call(FREE, &args![part]);
            e.call(LIST_0063F7B0, &args![target + 8]);
        }
        if target != 0 {
            fn_007f69d0(e, target, 1);
        }
        e.call(LIST_0063F7B0, &args![LIST_011DB150]);
    }
}

// Translated from 007f69d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on a `VATSMenu::VatsTarget`: (flags): its deleting destructor
/// (`007f6a00`, then the deallocator when bit 1 is set); returns it.
pub fn fn_007f69d0(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_007f6a00(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007f6a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on a `VATSMenu::VatsTarget`: its destructor (`0046ffb0` on the
/// part list at `+8`).
pub fn fn_007f6a00(e: &mut Engine, this: u32) {
    e.call(LIST_DESTRUCT_46FFB0, &args![this + 8]);
}

// Translated from 007f6a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl (reference): removes every target of the list at `011db150`
/// whose `pTarget` is the reference (`00905330` on the list). When the
/// reference is the target reference and the menu is open, the menu's
/// slot `0x0C` runs with (2, 0) first; when the list is then empty and the
/// menu is open, `VATSMenu::Close` runs. Nothing when the list is empty.
pub fn fn_007f6a20(e: &mut Engine, reference: u32) {
    if e.call(LIST_IS_EMPTY, &args![LIST_011DB150]).bool() {
        return;
    }
    let mut node = LIST_011DB150;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        if node_reference(e, node) == reference {
            let menu = vats_menu(e);
            if e.global::<u32>(TARGET_REFERENCE) == reference && menu != 0 {
                e.vcall(menu, 0x0c, &args![2u32, 0u32]);
            }
            let item = e.call(IDENTITY, &args![node]).u32();
            e.call(LIST_REMOVE_905330, &args![LIST_011DB150, item]);
            if e.call(LIST_IS_EMPTY, &args![LIST_011DB150]).bool() && vats_menu(e) != 0 {
                vats_menu_close(e);
            }
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    // 007f6ada: the head's item is read into a local nobody uses.
    e.call(IDENTITY, &args![LIST_011DB150]);
}

/// The size of `007f6af0`'s frame: its locals sit at their `EBP - offset`
/// addresses inside a block of this size (`pick_slot`).
const PICK_FRAME: u32 = 0x4b0;

/// The address of `007f6af0`'s local at `EBP - offset` in `frame`.
fn pick_slot(frame: u32, offset: u32) -> u32 {
    frame + PICK_FRAME - offset
}

// Translated from 007f6af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on the menu: picks a target under the pointer. False when
/// `bMouseOverButton` is set. Otherwise a ray is built from the camera
/// (`00a71080` with the truncated floats of `009a9350` and `00792760` on
/// `004b7210`'s object, into an origin and a direction), with the query of
/// `004a3c20` filtered like `007f52c0`'s casts but with layer `0x27`
/// (`bhkCollisionFilter::SetMultiLayerCollision(0x27, 0x140, 0)` first),
/// its end the direction scaled by the float at `011a0b74` (`0045bb20`,
/// `007f68a0`), and cast into an `hkpAllRayHitCollector`. The hits are
/// walked in order: a hit whose reference (`FindReferenceFor3D` of its
/// scene object) has a target node (`GetVatsTarget`) ends the walk: false
/// when that target is the current target reference, else `007f3bd0(0)`,
/// `UpdateChanceToHit`, the target reference becomes that target's and
/// the answer is true. False when no hit has a target.
// Translation incomplete: 007f6af0 - the C++ exception-unwinding frame (FS:[0]) that destroys the ray collector, and the security cookie (00ec408c), are not translated
pub fn fn_007f6af0(e: &mut Engine, this: Ptr<VatsMenu>) -> bool {
    if e.get(this, VatsMenu::bMouseOverButton) {
        return false;
    }
    let length = load_f32(e, FLOAT_CAST_LENGTH);
    e.with_stack(PICK_FRAME, |e, block| {
        let frame = block.addr();
        let ray = Ray {
            input: pick_slot(frame, 0xf0),
            from: pick_slot(frame, 0x28),
            to: pick_slot(frame, 0x34),
            filter: pick_slot(frame, 0xfc),
            filter_out: pick_slot(frame, 0xf8),
            collector: pick_slot(frame, 0x430),
        };
        let hit = pick_slot(frame, 0x4a0);
        let scaled = pick_slot(frame, 0x108);
        e.call(IDENTITY, &args![ray.from]);
        e.call(IDENTITY, &args![ray.to]);
        let object = e.call(OBJECT_004B7210, &[]).u32();
        let value = e.call(FLOAT_AT_40, &args![object]).f64();
        let second = ftol(e, value);
        let object = e.call(OBJECT_004B7210, &[]).u32();
        let value = e.call(FLOAT_AT_38, &args![object]).f64();
        let first = ftol(e, value);
        let holder = e.call(CAMERA_HOLDER, &[]).u32();
        let camera = e.call(CAMERA_OF, &args![holder]).u32();
        e.call(
            CAMERA_RAY_A71080,
            &args![camera, first, second, ray.from, ray.to, 0u32],
        );
        e.call(QUERY_CONSTRUCT_4A3C20, &args![ray.input]);
        e.call(SET_MULTI_LAYER_COLLISION, &args![0x27u32, 0x140u32, 0u32]);
        let player = e.global::<u32>(PLAYER);
        e.call(COLLISION_OBJECT_931ED0, &args![player, ray.filter_out]);
        e.call(STORE_8C71B0, &args![ray.filter, 0u32]);
        e.call(FILTER_4A39F0, &args![ray.filter, 0x27u32]);
        let group = e.call(GROUP_4A3A20, &args![ray.filter_out]).u32();
        e.call(FILTER_59CE80, &args![ray.filter, group]);
        let filter = e.mem.u32(ray.filter);
        e.call(QUERY_4A3F70, &args![ray.input, filter]);
        e.call(QUERY_4A3DA0, &args![ray.input, ray.from]);
        let end = e
            .call(POINT3_SCALE, &args![ray.to, scaled, length.to_bits()])
            .u32();
        fn_007f68a0(e, ray.input, end);
        e.call(RAY_COLLECTOR_CONSTRUCT, &args![ray.collector]);
        e.call(QUERY_4A3FB0, &args![ray.input, ray.collector]);
        ray_cast(e, &ray);
        let mut index = 0i32;
        let picked = loop {
            if index >= ray_hit_count(e, &ray) {
                break false;
            }
            let object = ray_hit_object(e, &ray, index as u32, hit);
            let reference = e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
            if object != 0 {
                let target = vats_menu_get_vats_target(e, this, reference);
                if target != 0 && node_reference(e, target) == e.global::<u32>(TARGET_REFERENCE) {
                    break false;
                }
                if target != 0 {
                    fn_007f3bd0(e, 0);
                    vats_menu_update_chance_to_hit(e, this);
                    let reference = node_reference(e, target);
                    e.set_global::<u32>(TARGET_REFERENCE, reference);
                    break true;
                }
            }
            index += 1;
        };
        e.call(RAY_COLLECTOR_DESTRUCT, &args![ray.collector]);
        picked
    })
}

// Translated from 007f6e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::BuildMetersForTarget` (Xbox PDB): (target node): renders a
/// `body_part_percent` meter (`Menu::RenderTemplate` of the open menu
/// under the `GlowBranch` tile of menu class `0x420`) into `pMeter` of
/// each part of the node's target, and sets trait `0xfb7` of each meter
/// that was made by the part's `eLimb` (the jump table at `007f6f90`): 5
/// for -1, 1, 2 and 13; 6 for 0; 1 for 3, 4, 7, 8, 9 and 14; 4 for 5, 6,
/// 10, 11 and 12; nothing for other values. `this` is not read.
pub fn vats_menu_build_meters_for_target(e: &mut Engine, _this: Ptr<VatsMenu>, target: u32) {
    let menu_tile = e.call(GET_MENU_BY_CLASS, &args![0x420u32]).u32();
    let glow = e
        .call(TILE_FROM_PATH, &args![menu_tile, TEXT_GLOW_BRANCH])
        .u32();
    let mut node = node_parts(e, target);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let menu = vats_menu(e);
        let meter = e
            .call(
                MENU_RENDER_TEMPLATE,
                &args![menu, glow, TEXT_BODY_PART_PERCENT, 0u32],
            )
            .u32();
        let part = node_part(e, node);
        e.mem.set_u32(part + 0x30, meter);
        let part = node_part(e, node);
        if e.mem.u32(part + 0x30) != 0 {
            let part = node_part(e, node);
            let value = match e.mem.i32(part + 0x20) {
                -1 | 1 | 2 | 13 => Some(5u32),
                0 => Some(6),
                3 | 4 | 7 | 8 | 9 | 14 => Some(1),
                5 | 6 | 10..=12 => Some(4),
                _ => None,
            };
            if let Some(value) = value {
                let part = node_part(e, node);
                let meter = e.mem.u32(part + 0x30);
                e.call(TILE_SET_INT, &args![meter, 0xfb7u32, value]);
            }
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
}

// Translated from 007f6fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned_int>::GetLastItem` (Xbox PDB): the list box's slot
/// `0x14` for `(NumUnfiltered() - 1, 0)`; returns what the slot answers.
/// The 0 pushed at 007f6fb7 is not read by 00726b40 (plain `RET`); it is the
/// slot's second word (007a30a0, `RET 8`).
pub fn list_box_unsigned_int_get_last_item(e: &mut Engine, this: Ptr<ListBox>) -> u32 {
    let count = e.call(LIST_BOX_NUM_UNFILTERED, &args![this]).u32();
    e.vcall(this.addr(), 0x14, &args![count.wrapping_sub(1), 0u32])
        .u32()
}

// Translated from 007f6fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned_int>::AddItem` (Xbox PDB): (item, text, position,
/// template). Renders the template (`template`, else `pTemplateName`)
/// under `pContainerRect` (`Menu::RenderTemplate` of the container's
/// menu), gives the new tile trait `0xfaa` -1 when it has no such value,
/// its text (trait `0xfc4`) when `text` is set, and links a new list box
/// item (0x0C bytes, `007a62e0` with the address of `item`): after
/// `position` when it is set (`007a7eb0`, then `Synchronize` when the flag
/// test `007242c0(0)` answers true), else at the end (`00905820`, then
/// `InitTilePosition` and `SetTotalListHeight` when the flag test answers
/// true; trait `0xfac` gets `usNextIndex`, which goes up by one). When the
/// list then has one item and the container's `_number_of_visible_items`
/// is above 0.0, the container's value `0xfb0` gets the actions `0x7d0`
/// (the new tile, `0xfb0`) and `0x7d3` (that count) and is recalculated.
/// Returns the new tile; 0 when there is no container, or neither template.
// Translation incomplete: 007f6fe0 - the C++ exception-unwinding frame (FS:[0]) that frees the new list box item when its constructor throws is not translated
pub fn list_box_unsigned_int_add_item(
    e: &mut Engine,
    this: Ptr<ListBox>,
    item: u32,
    text: u32,
    position: u32,
    template: u32,
) -> u32 {
    let container = e.get(this, ListBox::pContainerRect).addr();
    if container == 0 {
        return 0;
    }
    let own_template = e.get(this, ListBox::pTemplateName).addr();
    if own_template == 0 && template == 0 {
        return 0;
    }
    let template = if template != 0 {
        template
    } else {
        own_template
    };
    let menu = e.call(TILE_GET_MENU, &args![container]).u32();
    let tile = e
        .call(
            MENU_RENDER_TEMPLATE,
            &args![menu, container, template, 0u32],
        )
        .u32();
    if e.call(TILE_GET_VALUE, &args![tile, 0xfaau32]).u32() == 0 {
        e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0xffff_ffffu32]);
    }
    if text != 0 {
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    }
    let new_item = |e: &mut Engine| -> u32 {
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        if block == 0 {
            return 0;
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            e.call(LIST_ITEM_CONSTRUCT, &args![block, tile, slot]).u32()
        })
    };
    let list = this.addr() + 4;
    if position != 0 {
        let list_item = new_item(e);
        e.call(LIST_INSERT, &args![list, list_item, position]);
        if e.call(LIST_BOX_FLAG_TEST, &args![this, 0u32]).bool() {
            e.call(LIST_BOX_SYNCHRONIZE, &args![this]);
        }
    } else {
        let list_item = new_item(e);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), list_item);
            e.call(LIST_ADD_TAIL, &args![list, slot]);
        });
        if e.call(LIST_BOX_FLAG_TEST, &args![this, 0u32]).bool() {
            e.call(LIST_BOX_INIT_TILE_POSITION, &args![this, tile]);
            e.call(LIST_BOX_SET_TOTAL_LIST_HEIGHT, &args![this]);
        }
        let index = e.get(this, ListBox::usNextIndex);
        e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
        e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
    }
    if e.call(LIST_COUNT, &args![this]).u32() == 1 {
        let id = e
            .call(
                ADD_USER_TRAIT,
                &args![TEXT_NUMBER_OF_VISIBLE_ITEMS, 0xffff_ffffu32],
            )
            .u32();
        let container = e.get(this, ListBox::pContainerRect).addr();
        let visible = e.call(GET_FLOAT_TRAIT, &args![container, id]).f64();
        // FCOMP double [01012060]; TEST AH,0x41; JNZ: on only when above.
        if above(visible, double_zero(e)) {
            let container = e.get(this, ListBox::pContainerRect).addr();
            let value = e.call(TILE_GET_VALUE_Q, &args![container, 0xfb0u32]).u32();
            e.call(TILE_VALUE_CLEAR_ACTIONS, &args![value]);
            e.call(
                TILE_VALUE_ADD_ACTION_TILE,
                &args![value, 0x7d0u32, tile, 0xfb0u32],
            );
            let id = e
                .call(
                    ADD_USER_TRAIT,
                    &args![TEXT_NUMBER_OF_VISIBLE_ITEMS, 0xffff_ffffu32],
                )
                .u32();
            let container = e.get(this, ListBox::pContainerRect).addr();
            let visible = e.call(GET_FLOAT_TRAIT, &args![container, id]).f64();
            // FSTP float [ESP]: the count rounded to a float.
            let visible = visible as f32;
            e.call(
                TILE_VALUE_ADD_ACTION_FLOAT,
                &args![value, 0x7d3u32, visible.to_bits()],
            );
            e.call(TILE_VALUE_CALCULATE, &args![value, 0u32]);
        }
    }
    tile
}

// Translated from 007f7280 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall on a `ListBox<unsigned int>`: (value, by_index). Finds a node
/// of the list at `+4`: with `by_index` set, the `value`th node (counting
/// from 0); else the first whose item tile's trait `0xfac` equals `value`
/// (`FILD` of the zero-extended value, `FUCOMPP`, `TEST AH,0x44; JNP`:
/// only an ordered equal stops). Then `007f7380(node, node before it)`
/// (the head is its own predecessor; the node is 0 when none matched) and
/// answers its bool. False when the list is empty.
pub fn fn_007f7280(e: &mut Engine, this: Ptr<ListBox>, value: u32, by_index: u8) -> bool {
    let list = this.addr().wrapping_add(4);
    if e.call(LIST_IS_EMPTY, &args![list]).bool() {
        return false;
    }
    let head = if this.addr() == 0 { 0 } else { list };
    let mut node = head;
    let mut previous = head;
    if by_index != 0 {
        let mut counter = 0u32;
        while node != 0 {
            let current = counter;
            counter = counter.wrapping_add(1);
            if current == value {
                break;
            }
            previous = node;
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
    } else {
        while node != 0 {
            let entry = e.call(IDENTITY, &args![node]).u32();
            let tile = e.mem.u32(e.mem.u32(entry));
            let id = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfacu32]).f64();
            if f64::from(value) == id {
                break;
            }
            previous = node;
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
    }
    e.call(LIST_BOX_REMOVE_7F7380, &args![this, node, previous])
        .bool()
}

/// `pTiles` slot at `offset` of the open menu (re-read each time, as the
/// code reads `pVatsMenu`).
fn menu_tile(e: &Engine, offset: u32) -> u32 {
    e.mem.u32(vats_menu(e) + offset)
}

/// `menu[offset] = VatsMenuRenderTemplate(parent, template)`; false when it
/// is 0 (the creation then answers false).
fn render_into(e: &mut Engine, offset: u32, parent: u32, template: u32) -> bool {
    let tile = vats_menu_vats_menu_render_template(e, parent, template);
    let menu = vats_menu(e);
    e.mem.set_u32(menu + offset, tile);
    menu_tile(e, offset) != 0
}

/// `TILE_SET_INT(trait, value)` on the menu tile at `offset`.
fn menu_set_int(e: &mut Engine, offset: u32, trait_id: u32, value: u32) {
    let tile = menu_tile(e, offset);
    e.call(TILE_SET_INT, &args![tile, trait_id, value]);
}

/// `HUDMainMenu::SetBrightness(tile, value)` on the menu tile at `offset`.
fn menu_brightness(e: &mut Engine, offset: u32, value: u32) {
    let tile = menu_tile(e, offset);
    e.call(HUD_SET_BRIGHTNESS, &args![tile, value]);
}

/// The text of `setting` into trait `0xfc4` of the menu tile at `offset`.
fn menu_set_text(e: &mut Engine, offset: u32, setting: u32) {
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    let tile = menu_tile(e, offset);
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
}

/// The float of trait `trait_id` of `tile`.
fn tile_float(e: &mut Engine, tile: u32, trait_id: u32) -> f64 {
    e.call(GET_FLOAT_TRAIT, &args![tile, trait_id]).f64()
}

/// `SET_FLOAT_TRAIT(trait, value, 1)` on `tile`.
fn tile_set_float(e: &mut Engine, tile: u32, trait_id: u32, value: f32) {
    e.call(SET_FLOAT_TRAIT, &args![tile, trait_id, value, 1u32]);
}

/// The perk companion of `VATSMenu::Create`: the default object `index`
/// cached in `cache` (when its slot `0xE4` answers true) is placed near
/// the player (its scene object's rotation `006a9540` and position
/// `0043c490`; the offset from the rotation's first column, `004b55c0(i,
/// 1)`, scaled by the float at `0104fca0` through `00439180` and added
/// through `004578c0`) by `004698a0` on `*011c3f2c` into `slot`, then
/// `008a6840(1)`, `HighProcess::SkipFadeIn` and an alpha of 0.0.
fn spawn_perk_companion(
    e: &mut Engine,
    frame: u32,
    offsets: (u32, u32, u32, u32),
    cache: u32,
    slot: u32,
) {
    let (scene_slot, position, rotation, origin) = offsets;
    let player = e.global::<u32>(PLAYER);
    let scene = e.vcall(player, 0x1d0, &[]).u32();
    e.mem.set_u32(frame_slot_create(frame, scene_slot), scene);
    let position = frame_slot_create(frame, position);
    let rotation = frame_slot_create(frame, rotation);
    let origin = frame_slot_create(frame, origin);
    for word in 0..3 {
        let value = e.global::<u32>(POINT_ZERO + 4 * word);
        e.mem.set_u32(position + 4 * word, value);
    }
    let source = e.call(BODY_PART_ARRAY, &args![scene]).u32();
    for word in 0..9 {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(rotation + 4 * word, value);
    }
    let source = e.call(NODE_POSITION_43C490, &args![scene]).u32();
    for word in 0..3 {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(origin + 4 * word, value);
    }
    for column in 0..3u32 {
        let value = e
            .call(MATRIX_ENTRY_4B55C0, &args![rotation, column, 1u32])
            .f64() as f32;
        e.mem.set_f32(position + 4 * column, value);
    }
    let distance = load_f32(e, FLOAT_COMPANION_DISTANCE).to_bits();
    e.call(POINT3_SCALE_IN_PLACE, &args![position, distance]);
    e.call(POINT3_ADD_IN_PLACE, &args![origin, position]);
    let tes = e.global::<u32>(TES);
    let worldspace = e.call(TES_GET_WORLDSPACE, &args![tes]).u32();
    let tes = e.global::<u32>(TES);
    let cell = e.call(TES_CELL_5F36F0, &args![tes]).u32();
    let handler = e.global::<u32>(OBJECT_011C3F2C);
    let form = e.global::<u32>(cache);
    let actor = e
        .call(
            PLACE_REFERENCE_4698A0,
            &args![handler, form, origin, POINT_ZERO, cell, worldspace, 0u32, 0u32, 0u32],
        )
        .u32();
    e.set_global::<u32>(slot, actor);
    let actor = e.global::<u32>(slot);
    e.call(ACTOR_8A6840, &args![actor, 1u32]);
    let actor = e.global::<u32>(slot);
    let process = e.call(WORD_AT_68, &args![actor]).u32();
    e.call(SKIP_FADE_IN, &args![process, actor]);
    let actor = e.global::<u32>(slot);
    e.call(SET_ALPHA_MULT, &args![actor, 0.0f32]);
}

/// The default object `index` cached at `cache` when it is unset and the
/// object answers its slot `0xE4`.
fn cache_default_object(e: &mut Engine, cache: u32, index: u32) {
    if e.global::<u32>(cache) != 0 {
        return;
    }
    let object = e.call(GET_DEFAULT_OBJECT, &args![index]).u32();
    if object != 0 && e.vcall(object, 0xe4, &[]).bool() {
        e.set_global::<u32>(cache, object);
    }
}

/// The size of `VATSMenu::Create`'s frame block (`frame_slot_create`).
const CREATE_FRAME: u32 = 0x300;

/// The address of `Create`'s local at `EBP - offset` in its frame block.
fn frame_slot_create(frame: u32, offset: u32) -> u32 {
    frame + CREATE_FRAME - offset
}

// Translated from 007e9200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::Create` (Xbox PDB): opens the VATS menu; answers whether it
/// opened.
///
/// Refuses when the menu is open or menu `0x3f4` is on top. Builds the
/// targets (`007f52c0`) and drops the actors answering slot `0x22c(0)`, the
/// bases `005f65d0` finds in the form `001770bc`, the actors `005a2030` refuses and the
/// references with slot `0x304` = 4 and `00472380` 3 or 4; with none left it gives up (`SetFirstPerson(1)` when
/// `011f21d1` asks). Refuses with a weapon in the form `00174256` (found the same way) or
/// when the player's process answers slot `0x1ac`. Reads
/// `Data\Menus\VATS_menu.xml`, makes its menu the open one and runs the
/// entry points `0x1c` and `0x29` (the stranger and fortune perks may
/// place a companion), `0x1d` (paralyzing palm). Sorts the targets by
/// desirability, picks the best (hostile ones within `iVatsTargetAngle`
/// first), sorts them by heading, starts VATS mode 1 and builds the
/// action point, hit point, compass and enemy health tiles from their
/// templates, the compass texture, the action list box, the `UIVATSEnter`
/// sound, the button texts and the special attack buttons.
// Translation incomplete: 007e9200 - the C++ exception-unwinding frame (FS:[0]) and the security cookie are not translated
pub fn vats_menu_create(e: &mut Engine) -> bool {
    e.with_stack(CREATE_FRAME, create_body)
}

/// The body of `VATSMenu::Create` with its frame block at `frame`.
fn create_body(e: &mut Engine, frame: Ptr) -> bool {
    let frame = frame.addr();
    e.set_global::<u8>(FLAG_011DB0B2, 0);
    e.set_global::<u8>(FLAG_011DB0B3, 0);
    e.set_global::<u8>(FLAG_011DB0B1, 0);
    let class = fn_007eb8c0(e);
    let class_tile = e.call(GET_MENU_BY_CLASS, &args![class]).u32();
    let menu = e.call(TILE_GET_MENU, &args![class_tile]).u32();
    e.set_global::<u32>(VATS_MENU, menu);
    if menu != 0 || e.call(IS_TOP_MENU_ID, &args![0x3f4u32]).bool() {
        return false;
    }
    e.call(LIST_CLEAR, &args![LIST_011DB150]);
    fn_007f52c0(e);
    let forbidden = e.call(LOOKUP_FORM, &args![0x0017_70bcu32]).u32();
    let mut node = LIST_011DB150;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let reference = node_reference(e, node);
        let base = if e.vcall(reference, 0x100, &[]).bool() {
            let reference = node_reference(e, node);
            e.call(FORM_OF_REFERENCE, &args![reference]).u32()
        } else {
            0
        };
        let reference = node_reference(e, node);
        let actor = if e.vcall(reference, 0x100, &[]).bool() {
            node_reference(e, node)
        } else {
            0
        };
        let reference = node_reference(e, node);
        let thing = if e.vcall(reference, 0x224, &[]).bool() {
            node_reference(e, node)
        } else {
            0
        };
        let mut level_three_or_four = false;
        if thing != 0
            && e.vcall(thing, 0x304, &[]).u32() == 4
            && (e.call(REFERENCE_472380, &args![thing]).u32() == 3
                || e.call(REFERENCE_472380, &args![thing]).u32() == 4)
        {
            level_three_or_four = true;
        }
        let reference = node_reference(e, node);
        let answers_22c = e.vcall(reference, 0x100, &[]).bool() && {
            let reference = node_reference(e, node);
            e.vcall(reference, 0x22c, &args![0u32]).bool()
        };
        let remove = answers_22c
            || (forbidden != 0 && base != 0 && {
                let local = frame_slot_create(frame, 0x198);
                e.mem.set_u32(local, base);
                let list = e.call(FORM_LIST_500940, &args![forbidden]).u32();
                e.call(LIST_CONTAINS_5F65D0, &args![list, local]).bool()
            })
            || (actor != 0 && e.call(ACTOR_CHECK_5A2030, &args![actor]).bool())
            || level_three_or_four;
        if remove {
            if e.global::<u32>(TARGET_REFERENCE) == node_reference(e, node) {
                e.set_global::<u32>(TARGET_REFERENCE, 0);
            }
            let item = e.call(IDENTITY, &args![node]).u32();
            e.call(LIST_REMOVE_905330, &args![node, item]);
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    if e.call(LIST_IS_EMPTY, &args![LIST_011DB150]).bool() {
        if e.global::<u8>(FLAG_011F21D1) != 0 {
            let player = e.global::<u32>(PLAYER);
            e.call(SET_FIRST_PERSON, &args![player, 1u32]);
        }
        return false;
    }
    let mut weapon = 0;
    let player = e.global::<u32>(PLAYER);
    if player != 0 && e.call(WORD_AT_68, &args![player]).u32() != 0 {
        let player = e.global::<u32>(PLAYER);
        let process = e.call(WORD_AT_68, &args![player]).u32();
        weapon = if e.vcall(process, 0x148, &[]).u32() != 0 {
            let player = e.global::<u32>(PLAYER);
            let process = e.call(WORD_AT_68, &args![player]).u32();
            let equipped = e.vcall(process, 0x148, &[]).u32();
            e.call(WORD_AT_8, &args![equipped]).u32()
        } else {
            0
        };
        let player = e.global::<u32>(PLAYER);
        let process = e.call(WORD_AT_68, &args![player]).u32();
        if e.vcall(process, 0x1ac, &[]).bool() {
            return false;
        }
    }
    let weapons = e.call(LOOKUP_FORM, &args![0x0017_4256u32]).u32();
    if weapon != 0 && weapons != 0 && {
        let local = frame_slot_create(frame, 0x19c);
        e.mem.set_u32(local, weapon);
        let list = e.call(FORM_LIST_500940, &args![weapons]).u32();
        e.call(LIST_CONTAINS_5F65D0, &args![list, local]).bool()
    } {
        return false;
    }
    if e.global::<u32>(TARGET_REFERENCE) == 0 {
        let reference = node_reference(e, LIST_011DB150);
        e.set_global::<u32>(TARGET_REFERENCE, reference);
    }
    let object = e.call(OBJECT_004B7210, &[]).u32();
    let depth = e.call(GET_MAXIMUM_DEPTH, &[]).f64() as f32;
    let path = e.call(MENU_PATH_586150, &args![object]).u32();
    let read = e
        .call(TILE_READ_FILE, &args![path, TEXT_VATS_MENU_XML])
        .u32();
    let menu_object = e.call(TILE_GET_MENU, &args![read]).u32();
    if menu_object == 0 || e.vcall(menu_object, 0x34, &[]).u32() != fn_007eb8c0(e) {
        // 007eb787
        if menu_object != 0 && e.call(NEXT_NODE, &args![menu_object]).u32() != 0 {
            e.vcall(menu_object, 0x00, &args![1u32]);
        }
        return false;
    }
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![read, 0u32, TYPE_TILE, TYPE_TILE_MENU, 0u32],
        )
        .u32();
    e.call(MENU_SET_MENU_TILE, &args![menu_object, cast, 0u32]);
    let menu = e
        .call(
            DYNAMIC_CAST,
            &args![menu_object, 0u32, TYPE_MENU, TYPE_VATS_MENU, 0u32],
        )
        .u32();
    e.set_global::<u32>(VATS_MENU, menu);

    // 007e96ed: the stranger and fortune perks.
    let stranger = frame_slot_create(frame, 0xd8);
    let fortune = frame_slot_create(frame, 0x54);
    e.mem.set_f32(stranger, 0.0);
    let player = e.global::<u32>(PLAYER);
    e.call(HANDLE_ENTRY_POINT, &args![0x1cu32, player, stranger]);
    e.mem.set_f32(fortune, 0.0);
    let player = e.global::<u32>(PLAYER);
    e.call(HANDLE_ENTRY_POINT, &args![0x29u32, player, fortune]);
    let roll = e
        .call(RANDOM_FLOAT, &args![0.0f32.to_bits(), 1.0f32.to_bits()])
        .f64() as f32;
    let threshold = (1.0 - load_f32(e, fortune) as f64 * e.global::<f64>(DOUBLE_HALF)) as f32;
    // `TEST AH,0x41; JNZ` then `FCOMPP; TEST AH,1; JNZ`.
    if above(load_f32(e, stranger) as f64, double_zero(e)) && threshold as f64 >= roll as f64 {
        cache_default_object(e, STRANGER_FORM, 7);
        // `TEST AH,0x44; JNP`: placed unless equal to 0.0.
        if load_f32(e, stranger) as f64 != double_zero(e) && e.global::<u32>(STRANGER_FORM) != 0 {
            spawn_perk_companion(
                e,
                frame,
                (0x10c, 0x118, 0x108, 0x124),
                STRANGER_FORM,
                STRANGER_ACTOR,
            );
        }
    } else if above(load_f32(e, fortune) as f64, double_zero(e)) {
        cache_default_object(e, FORTUNE_FORM, 0x16);
        if load_f32(e, fortune) as f64 != double_zero(e) && e.global::<u32>(FORTUNE_FORM) != 0 {
            spawn_perk_companion(
                e,
                frame,
                (0x150, 0x15c, 0x14c, 0x168),
                FORTUNE_FORM,
                FORTUNE_ACTOR,
            );
        }
    }
    let palm = frame_slot_create(frame, 0x8c);
    e.mem.set_f32(palm, 0.0);
    let player = e.global::<u32>(PLAYER);
    e.call(HANDLE_ENTRY_POINT, &args![0x1du32, player, palm]);
    let value = u8::from(above(load_f32(e, palm) as f64, double_zero(e)));
    e.set_global::<u8>(FLAG_011DB0B0, value);
    let menu = vats_menu(e);
    e.mem.set_u32(menu + 0x100, 0);
    let menu = vats_menu(e);
    e.call(NEXT_NODE, &args![menu]);
    e.call(TILE_UPDATE_ALL, &args![0u32]);
    e.call(LIST_SORT, &args![LIST_011DB150, FN_ORDER_BY_FACING]);

    // 007e9ba1: the targets' desirability.
    let mut node = LIST_011DB150;
    let player = e.global::<u32>(PLAYER);
    let angle = e.vcall(player, 0x2bc, &args![0u32]).f64();
    let facing = (angle * e.global::<f64>(DOUBLE_DEGREES)) as f32;
    let position = frame_slot_create(frame, 0x88);
    let player = e.global::<u32>(PLAYER);
    let source = e.vcall(player, 0x1f4, &[]).u32();
    for word in 0..3 {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(position + 4 * word, value);
    }
    if node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let reference = node_reference(e, node);
        e.set_global::<u32>(TARGET_REFERENCE, reference);
    }
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let gone = node_reference(e, node) == 0 || {
            let reference = node_reference(e, node);
            e.vcall(reference, 0x1d0, &[]).u32() == 0
        };
        if gone {
            let next = e.call(NEXT_NODE, &args![node]).u32();
            let item = e.call(IDENTITY, &args![node]).u32();
            e.call(LIST_REMOVE_905330, &args![LIST_011DB150, item]);
            node = next;
            continue;
        }
        let reference = node_reference(e, node);
        let other = e.vcall(reference, 0x1f4, &[]).u32();
        let heading = e
            .call(HEADING_BETWEEN_LOCATIONS, &args![position, other])
            .f64() as f32;
        let difference = (facing as f64 - heading as f64) as f32;
        let angle = e.call(FLOAT_ABS, &args![difference]).f64() as f32;
        let reference = node_reference(e, node);
        let other = e.vcall(reference, 0x1f4, &[]).u32();
        let offset = e
            .call(
                POINT_SUBTRACT,
                &args![position, frame_slot_create(frame, 0x1a8), other],
            )
            .u32();
        let distance = fn_007eb7f0(e, offset);
        let divisor = if above(distance as f64, double_zero(e)) {
            distance as f64
        } else {
            e.global::<f64>(DOUBLE_SMALL)
        };
        let half_turn = e.global::<f64>(DOUBLE_180);
        let folded = if above(angle as f64, half_turn) {
            e.global::<f64>(DOUBLE_360) - angle as f64
        } else {
            angle as f64
        };
        let desire =
            (e.global::<f64>(DOUBLE_40) / divisor + (half_turn - folded) / half_turn) as f32;
        let item = node_part(e, node);
        e.mem.set_f32(item + 0x14, desire);
        let folded = if above(angle as f64, half_turn) {
            e.global::<f64>(DOUBLE_360) - angle as f64
        } else {
            angle as f64
        } as f32;
        let to_player = e.call(FLOAT_ABS, &args![folded]).f64() as f32;
        let item = node_part(e, node);
        e.mem.set_f32(item + 0x18, to_player);
        let item = node_part(e, node);
        e.mem.set_f32(item + 0x1c, quiet_f32(distance));
        node = e.call(NEXT_NODE, &args![node]).u32();
    }

    // 007e9ebf: the best target.
    let mut node = LIST_011DB150;
    let mut best = 0.0f32;
    let mut best_node = 0u32;
    let mut hostile = false;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item = node_part(e, node);
        let desire = load_f32(e, item + 0x14) as f64;
        // `FCOMPP; TEST AH,5; JP`: more desirable when the best is below.
        let more_desirable = (best as f64) < desire;
        let take = if more_desirable && !hostile {
            true
        } else {
            let item = node_part(e, node);
            let address = e
                .call(SETTING_INT_ADDRESS, &args![SETTING_VATS_TARGET_ANGLE])
                .u32();
            let limit = e.mem.i32(address) as f64;
            (load_f32(e, item + 0x18) as f64) < limit
                && {
                    let item = node_part(e, node);
                    e.mem.u8(item + 0x20) != 0
                }
                && (best_node == 0 || {
                    let item = node_part(e, node);
                    let best_item = node_part(e, best_node);
                    above(
                        load_f32(e, best_item + 0x1c) as f64,
                        load_f32(e, item + 0x1c) as f64,
                    )
                })
        };
        if take {
            let item = node_part(e, node);
            let address = e
                .call(SETTING_INT_ADDRESS, &args![SETTING_VATS_TARGET_ANGLE])
                .u32();
            if (load_f32(e, item + 0x18) as f64) < e.mem.i32(address) as f64 {
                hostile = true;
            }
            let item = node_part(e, node);
            best = load_f32(e, item + 0x14);
            best_node = node;
            let reference = node_reference(e, node);
            e.set_global::<u32>(TARGET_REFERENCE, reference);
            let mut parts = node_parts(e, node);
            let first = node_part(e, parts);
            let menu = vats_menu(e);
            e.mem.set_u32(menu + 0x100, first);
            while parts != 0 && !e.call(LIST_IS_EMPTY, &args![parts]).bool() {
                let item = node_part(e, node);
                if e.mem.u32(item + 4) == 1 && {
                    let part = node_part(e, parts);
                    e.mem.u32(part + 0x20) == 1
                } {
                    let part = node_part(e, parts);
                    let menu = vats_menu(e);
                    e.mem.set_u32(menu + 0x100, part);
                }
                parts = e.call(NEXT_NODE, &args![parts]).u32();
            }
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    let menu = vats_menu(e);
    e.mem.set_u8(menu + 0x118, 0);
    e.call(LIST_SORT, &args![LIST_011DB150, FN_ORDER_BY_HEADING]);
    let menu = vats_menu(e);
    e.mem.set_u8(menu + 0xac, 1);
    e.call(VATS_SET_MODE, &args![VATS, 1u32, 0u32]);
    let width = e.call(SCREEN_WIDTH, &[]).f64();
    let width = ftol(e, width);
    let height = e.call(SIZE_00715DA0, &[]).f64();
    let height = ftol(e, height);
    let margin_x = e.call(SIZE_007177C0, &[]).f64();
    let margin_x = ftol(e, margin_x);
    let margin_y = e.call(SIZE_00717820, &[]).f64();
    let margin_y = ftol(e, margin_y);
    let original_x = e
        .call(ADD_USER_TRAIT, &args![TEXT_ORIGINAL_X, 0xffff_ffffu32])
        .u32();
    let total_width = e
        .call(ADD_USER_TRAIT, &args![TEXT_TOTAL_WIDTH, 0xffff_ffffu32])
        .u32();
    let image_width = e
        .call(ADD_USER_TRAIT, &args![TEXT_IMAGE_WIDTH, 0xffff_ffffu32])
        .u32();
    let audio = e.call(AUDIO_INSTANCE, &[]).u32();
    fn_007eb8a0(e, audio);

    // 007ea15c: the action points.
    let parent = e.call(GET_MENU_BY_CLASS, &args![0x420u32]).u32();
    let root = fn_007f4890(e, parent, TEXT_ACTION_POINTS);
    if root == 0 {
        return false;
    }
    let left = width.wrapping_sub(margin_x.wrapping_mul(2)) as f64;
    let value = (left - tile_float(e, root, 0xfb1) + e.global::<f64>(DOUBLE_30)) as f32;
    tile_set_float(e, root, 0xfa1, value);
    let top = height.wrapping_sub(margin_y.wrapping_mul(2)) as f64;
    let value = (top - tile_float(e, root, 0xfb0)) as f32;
    tile_set_float(e, root, 0xfa2, value);
    e.call(TILE_SET_INT, &args![root, 0xfa8u32, 1u32]);
    if !render_into(e, 0x48, root, TEXT_TEMPLATE_RIGHT_BRACKET) {
        return false;
    }
    menu_brightness(e, 0x48, 2);
    menu_set_int(e, 0x48, 0xfa1, 0xffff_ffdb);
    if !render_into(e, 0x4c, root, TEXT_TEMPLATE_METER) {
        return false;
    }
    menu_set_int(e, 0x4c, original_x, 0x146);
    menu_set_int(e, 0x4c, 0xfa2, 0x29);
    menu_set_int(e, 0x4c, 0xfad, 5);
    menu_brightness(e, 0x4c, 3);
    if !render_into(e, 0x50, root, TEXT_TEMPLATE_METER) {
        return false;
    }
    menu_set_int(e, 0x50, original_x, 0x146);
    menu_set_int(e, 0x50, 0xfa2, 0x29);
    menu_set_int(e, 0x50, 0xfa9, 0x80);
    menu_brightness(e, 0x50, 3);
    if !render_into(e, 0x54, root, TEXT_TEMPLATE_RIGHT_TEXT) {
        return false;
    }
    menu_set_int(e, 0x54, 0xfa1, 0x13b);
    menu_set_int(e, 0x54, 0xfa2, 8);
    menu_brightness(e, 0x54, 0);
    menu_set_text(e, 0x54, SETTING_ACTION_POINTS_SHORT);
    if !render_into(e, 0x58, root, TEXT_TEMPLATE_RIGHT_TEXT) {
        return false;
    }
    menu_set_int(e, 0x58, 0xfa1, 0x10e);
    menu_set_int(e, 0x58, 0xfa2, 8);
    menu_brightness(e, 0x58, 0);
    menu_set_text(e, 0x58, SETTING_MESSAGE_LOW_AP);
    menu_set_int(e, 0x58, 0xfa3, 0);
    if !render_into(e, 0x5c, root, TEXT_TEMPLATE_RIGHT_TEXT) {
        return false;
    }
    menu_set_int(e, 0x5c, 0xfa1, 0x13b);
    menu_set_int(e, 0x5c, 0xfa2, 0x50);
    menu_set_int(e, 0x5c, 0xfb9, 7);
    menu_brightness(e, 0x5c, 0);
    if !render_into(e, 0x60, root, TEXT_TEMPLATE_RIGHT_TEXT) {
        return false;
    }
    menu_set_int(e, 0x60, 0xfa1, 0x50);
    let tile = menu_tile(e, 0x5c);
    let value = tile_float(e, tile, 0xfa2) as f32;
    let tile = menu_tile(e, 0x60);
    tile_set_float(e, tile, 0xfa2, value);
    menu_set_int(e, 0x60, 0xfb9, 7);
    menu_set_text(e, 0x60, SETTING_INVENTORY_CONDITION);
    menu_brightness(e, 0x60, 0);
    let meter = e
        .call(
            MENU_RENDER_TEMPLATE,
            &args![menu_object, root, TEXT_TEMPLATE_METER, 0u32],
        )
        .u32();
    let menu = vats_menu(e);
    e.mem.set_u32(menu + 0x64, meter);
    menu_set_int(e, 0x64, original_x, 0x5a);
    let tile = menu_tile(e, 0x5c);
    let value = (tile_float(e, tile, 0xfa2) + e.global::<f64>(DOUBLE_THREE)) as f32;
    let tile = menu_tile(e, 0x64);
    tile_set_float(e, tile, 0xfa2, value);
    let tile = menu_tile(e, 0x64);
    e.call(TILE_SET_STRING, &args![tile, 0xfccu32, TEXT_VATS_BAR, 1u32]);
    menu_set_int(e, 0x64, total_width, 0x3c);
    menu_set_int(e, 0x64, image_width, 1);
    menu_set_int(e, 0x64, 0xfb8, 0xc8);
    menu_brightness(e, 0x64, 3);
    let background = e
        .call(
            MENU_RENDER_TEMPLATE,
            &args![menu_object, root, TEXT_TEMPLATE_METER_BACKGROUND, 0u32],
        )
        .u32();
    let menu = vats_menu(e);
    e.mem.set_u32(menu + 0x68, background);
    let tile = menu_tile(e, 0x5c);
    let value = (tile_float(e, tile, 0xfa2) + e.global::<f64>(DOUBLE_THREE)) as f32;
    let tile = menu_tile(e, 0x68);
    tile_set_float(e, tile, 0xfa2, value);
    menu_brightness(e, 0x68, 4);
    let owner = e.global::<u32>(PLAYER) + 0xa4;
    let points = e.vcall(owner, 0x0c, &args![0xcu32]).f64() as f32;
    let owner = e.global::<u32>(PLAYER) + 0xa4;
    let base = e.vcall(owner, 0x20, &args![0xcu32]).f64() as f32;
    let larger = e
        .call(FLOAT_MAX, &args![base.to_bits(), points.to_bits()])
        .f64() as f32;
    let base_points = not_negative(e, larger);
    let menu = vats_menu(e);
    e.mem.set_f32(menu + 0xe4, base_points);
    let owner = e.global::<u32>(PLAYER) + 0xa4;
    let points = e.vcall(owner, 0x0c, &args![0xcu32]).f64() as f32;
    let points = not_negative(e, points);
    let menu = vats_menu(e);
    e.mem.set_f32(menu + 0xe0, points);
    for offset in [0x4c, 0x50] {
        let menu = vats_menu(e);
        let ratio = (load_f32(e, menu + 0xe0) as f64 / load_f32(e, menu + 0xe4) as f64) as f32;
        let tile = menu_tile(e, offset);
        e.call(HUD_UPDATE_METER, &args![tile, ratio, 1u32, 0u32, 0u32]);
    }

    // 007ea86f: the hit points and the compass.
    let parent = e.call(GET_MENU_BY_CLASS, &args![0x420u32]).u32();
    let root = fn_007f4890(e, parent, TEXT_HIT_POINTS);
    if root == 0 {
        return false;
    }
    let left = margin_x.wrapping_mul(2).wrapping_add(0xa) as u32;
    e.call(TILE_SET_INT, &args![root, 0xfa1u32, left]);
    let top = height.wrapping_sub(margin_y.wrapping_mul(2)) as f64;
    let value = (top - tile_float(e, root, 0xfb0)) as f32;
    tile_set_float(e, root, 0xfa2, value);
    e.call(TILE_SET_INT, &args![root, 0xfa8u32, 1u32]);
    if !render_into(e, 0x6c, root, TEXT_TEMPLATE_LEFT_BRACKET) {
        return false;
    }
    menu_brightness(e, 0x6c, 2);
    if !render_into(e, 0x70, root, TEXT_TEMPLATE_METER) {
        return false;
    }
    menu_set_int(e, 0x70, original_x, 0x14);
    menu_set_int(e, 0x70, 0xfa2, 0x29);
    menu_brightness(e, 0x70, 3);
    if !render_into(e, 0x74, root, TEXT_TEMPLATE_RIGHT_TEXT) {
        return false;
    }
    menu_set_int(e, 0x74, 0xfa1, 0x35);
    menu_set_int(e, 0x74, 0xfa2, 8);
    menu_brightness(e, 0x74, 0);
    menu_set_text(e, 0x74, SETTING_HIT_POINTS_SHORT);
    if !render_into(e, 0x84, root, TEXT_TEMPLATE_COMPASS_WINDOW) {
        return false;
    }
    for (offset, row) in [(0x78u32, 0x1eu32), (0x7c, 0x1e), (0x80, 0xa)] {
        let window = menu_tile(e, 0x84);
        if !render_into(e, offset, window, TEXT_TEMPLATE_COMPASS_ICON_GROUP) {
            return false;
        }
        menu_set_int(e, offset, 0xfa2, row);
    }
    let mut index = 0i32;
    loop {
        let address = e
            .call(
                SETTING_INT_ADDRESS,
                &args![SETTING_HUD_MAX_COMPASS_NPC_TICKS],
            )
            .u32();
        if index >= e.mem.i32(address) {
            break;
        }
        let menu = vats_menu(e);
        let group = menu_tile(e, 0x80);
        e.call(
            MENU_RENDER_TEMPLATE,
            &args![menu, group, TEXT_COMPASS_NPC_ICON, 0u32],
        );
        index = index.wrapping_add(1);
    }
    for _ in 0..10 {
        let menu = vats_menu(e);
        let group = menu_tile(e, 0x7c);
        e.call(
            MENU_RENDER_TEMPLATE,
            &args![menu, group, TEXT_TEMPLATE_COMPASS_ICON_QUEST, 0u32],
        );
    }
    let menu = vats_menu(e);
    let group = menu_tile(e, 0x7c);
    e.call(
        MENU_RENDER_TEMPLATE,
        &args![menu, group, TEXT_TEMPLATE_COMPASS_ICON_PLAYER, 0u32],
    );
    for _ in 0..10 {
        let menu = vats_menu(e);
        let group = menu_tile(e, 0x78);
        e.call(
            MENU_RENDER_TEMPLATE,
            &args![menu, group, TEXT_TEMPLATE_COMPASS_ICON_MARKER, 0u32],
        );
    }

    // 007eac82: the enemy health.
    let parent = e.call(GET_MENU_BY_CLASS, &args![0x420u32]).u32();
    let root = fn_007f4890(e, parent, TEXT_ENEMY_HEALTH);
    if root == 0 {
        return false;
    }
    let half = width.wrapping_div(2) as f64;
    let value = (half - tile_float(e, root, 0xfb1) / e.global::<f64>(DOUBLE_TWO)) as f32;
    tile_set_float(e, root, 0xfa1, value);
    let row = height
        .wrapping_sub(margin_y.wrapping_mul(2))
        .wrapping_sub(0x58) as u32;
    e.call(TILE_SET_INT, &args![root, 0xfa2u32, row]);
    e.call(TILE_SET_INT, &args![root, 0xff4u32, 2u32]);
    if !render_into(e, 0x88, root, TEXT_TEMPLATE_WARNING_BACKGROUND) {
        return false;
    }
    let value = tile_float(e, root, 0xfb1) as f32;
    let tile = menu_tile(e, 0x88);
    tile_set_float(e, tile, 0xfb1, value);
    let value = (tile_float(e, root, 0xfb0) + e.global::<f64>(DOUBLE_FIVE)) as f32;
    let tile = menu_tile(e, 0x88);
    tile_set_float(e, tile, 0xfb0, value);
    let menu = vats_menu(e);
    e.call(NEXT_NODE, &args![menu]);
    e.call(TILE_UPDATE_ALL, &args![0u32]);
    if !render_into(e, 0x8c, root, TEXT_TEMPLATE_ENEMY_HEALTH_BRACKET) {
        return false;
    }
    menu_brightness(e, 0x8c, 1);
    menu_set_int(e, 0x8c, 0xfa3, 1);
    let meters = [
        (0x90u32, root, 0xabu32, 1u32),
        (0x98, root, 0xab, 0),
        (0x94, 0, 0xaf, 0),
    ];
    for (offset, parent, x, shown) in meters {
        let parent = if parent == 0 {
            menu_tile(e, 0x98)
        } else {
            parent
        };
        if !render_into(e, offset, parent, TEXT_TEMPLATE_METER) {
            return false;
        }
        menu_set_int(e, offset, original_x, x);
        menu_set_int(e, offset, total_width, 0xf0);
        menu_set_int(e, offset, 0xfa2, 3);
        menu_brightness(e, offset, 3);
        menu_set_int(e, offset, 0xfa3, shown);
    }
    if !render_into(e, 0x9c, root, TEXT_TEMPLATE_CENTER_TEXT) {
        return false;
    }
    menu_set_int(e, 0x9c, 0xfa1, 0xaa);
    menu_set_int(e, 0x9c, 0xfa2, 0x23);
    menu_set_int(e, 0x9c, 0xfba, 0x15e);
    menu_brightness(e, 0x9c, 0);
    create_finish(e, frame, read, menu_object, depth)
}

/// `VATSMenu::Create` from `007eb114`: the compass texture, the checks of
/// the tiles, the action list, the opening sound and the buttons.
fn create_finish(e: &mut Engine, frame: u32, read: u32, menu_object: u32, depth: f32) -> bool {
    let texture = frame_slot_create(frame, 0x98);
    e.call(NI_POINTER_CONSTRUCT_633C90, &args![texture, 0u32]);
    let name = frame_slot_create(frame, 0x1ac);
    let name = e
        .call(FIXED_STRING_FROM, &args![name, TEXT_COMPASS_ALPHA_MAP])
        .u32();
    let created = e
        .call(
            SOURCE_TEXTURE_CREATE,
            &args![name, GLOBAL_011A9598, 1u32, 0u32],
        )
        .u32();
    e.call(NI_POINTER_ASSIGN_66B0D0, &args![texture, created]);
    e.call(
        FIXED_STRING_DESTRUCT,
        &args![frame_slot_create(frame, 0x1ac)],
    );
    let image = e.call(POINTER_GET_559450, &args![texture]).u32();
    let window = menu_tile(e, 0x84);
    let node = e.vcall(window, 0x20, &[]).u32();
    e.call(COMPASS_SET_TEXTURE_7700B0, &args![node, image]);
    let window = menu_tile(e, 0x84);
    let node = e.vcall(window, 0x20, &[]).u32();
    e.call(COMPASS_SET_7700D0, &args![node, 1u32]);
    let missing = frame_slot_create(frame, 0x5c);
    let menu = vats_menu(e);
    if !fn_007eb8d0(e, Ptr::new(menu), missing) {
        e.call(ERROR, &args![TEXT_CREATION_FAILED_TILES]);
        e.call(EMERGENCY_CLOSE_ALL_MENUS, &[]);
        e.call(NI_POINTER_DESTRUCT_45CEC0, &args![texture]);
        return false;
    }
    // `TEST AH,0x44`: the depth goes into trait `0xfad` when trait `0xfa7`
    // equals either constant.
    let value = tile_float(e, read, 0xfa7);
    let matches = value == e.global::<f64>(DOUBLE_6006) || {
        let value = tile_float(e, read, 0xfa7);
        value == e.global::<f64>(DOUBLE_102)
    };
    if matches {
        tile_set_float(e, read, 0xfad, quiet_f32(depth));
    }
    let string = frame_slot_create(frame, 0x50);
    e.call(STRING_CONSTRUCT_4037B0, &args![string]);
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64() as f32;
    let hours = ftol(e, hour as f64) as i8;
    let fraction = (hour as f64 - hours as f64) * e.global::<f64>(DOUBLE_60);
    ftol(e, fraction);
    let twelve = e.global::<f64>(DOUBLE_12);
    let clock = if (hours as f64) < e.global::<f64>(DOUBLE_ONE) {
        twelve
    } else if above(hours as f64, twelve) {
        hours as f64 - twelve
    } else {
        hours as f64
    };
    ftol(e, clock);
    e.set_global::<i32>(LAST_LIMB, -1);
    e.set_global::<u32>(GLOBAL_011A59F0, 0x19);
    let player = e.global::<u32>(PLAYER);
    e.call(SET_IRON_SIGHTS, &args![player, 0u32, 0u32, 0u32]);
    e.set_global::<u8>(FLAG_011DB0CC, 0);
    let menu = vats_menu(e);
    let list_tile = e.mem.u32(menu + 0x2c);
    list_box_unsigned_int_initialize(e, Ptr::new(menu + 0xb0), list_tile, TEXT_QUEUED_ACTION);
    let manager = e.call(AUDIO_MANAGER_INSTANCE, &[]).u32();
    let handle = frame_slot_create(frame, 0xc0);
    e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
    let temporary = frame_slot_create(frame, 0x1bc);
    let fetched = e
        .call(
            GET_SOUND_HANDLE_BY_NAME,
            &args![manager, temporary, TEXT_UI_VATS_ENTER, 0x121u32],
        )
        .u32();
    e.call(SOUND_HANDLE_ASSIGN, &args![handle, fetched]);
    e.call(EMPTY_MEMBER, &args![temporary]);
    e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
    for offset in [0x10c, 0x10d, 0x10e, 0xf9, 0xfa, 0x10f, 0x110, 0xfb] {
        let menu = vats_menu(e);
        e.mem.set_u8(menu + offset, 0);
    }
    let time = e.call(TIMER_MILLISECONDS, &args![TIMER]).u32();
    let menu = vats_menu(e);
    e.mem.set_u32(menu + 0x114, time);
    let weapon = player_weapon(e);
    if weapon == 0
        || e.call(CHECK_006450C0, &args![weapon]).bool()
        || e.call(WEAPON_KIND, &args![weapon]).u32() == 0xa
    {
        menu_set_int(e, 0x40, 0xfa3, 0);
    }
    let class = fn_007eb8c0(e);
    let message = if e.call(OBJECT_00709D40, &[]).bool() {
        0x1b
    } else {
        0x15
    };
    let manager = e.call(OBJECT_004B7210, &[]).u32() + 0x4d4;
    e.call(
        TUTORIAL_SHOW_MESSAGE,
        &args![manager, message, class, 0x200u32],
    );
    let cost = e
        .call(GET_WEAPON_ATTACK_ACTION_POINTS, &args![weapon, 1u32, 0u32])
        .f64() as f32;
    let menu = vats_menu(e);
    // `FCOMPP; TEST AH,0x41; JNZ`: warned when the cost is above the points.
    if above(cost as f64, load_f32(e, menu + 0xe0) as f64) {
        menu_set_int(e, 0x44, 0xfaf, 0);
    }
    let language = e.call(SETTING_STRING, &args![SETTING_LANGUAGE]).u32();
    if e.call(STRCMP, &args![language, TEXT_ENGLISH]).i32() != 0 {
        menu_set_text(e, 0x44, SETTING_VATS_SELECT);
        menu_set_text(e, 0x38, SETTING_ACCEPT);
        menu_set_text(e, 0x3c, SETTING_RETURN);
        menu_set_text(e, 0x40, SETTING_VATS_BODY_PART);
        let text = e.call(SETTING_STRING, &args![SETTING_VATS_TARGET]).u32();
        let menu = vats_menu(e);
        let menu_root = e.call(NEXT_NODE, &args![menu]).u32();
        let button = e
            .call(
                TILE_GET_CHILD_BY_NAME,
                &args![menu_root, TEXT_TARGET_BUTTON],
            )
            .u32();
        e.call(TILE_SET_STRING, &args![button, 0xfc4u32, text, 1u32]);
    }
    for (name, label, value) in [
        (TEXT_BODY_PART_BUTTON, TEXT_XBUTTON_LS, 0x11u32),
        (TEXT_TARGET_BUTTON, TEXT_XBUTTON_RS, 0x12),
    ] {
        let menu = vats_menu(e);
        let menu_root = e.call(NEXT_NODE, &args![menu]).u32();
        let button = e
            .call(TILE_GET_CHILD_BY_NAME, &args![menu_root, name])
            .u32();
        if button != 0 {
            let id = e.call(TILE_TEXT_TO_TRAIT, &args![TEXT_XBOX_BUTTON]).u32();
            e.call(TILE_SET_STRING, &args![button, id, label, 1u32]);
            let id = e.call(TILE_TEXT_TO_TRAIT, &args![TEXT_XBOX_BUTTON]).u32();
            e.call(TILE_SET_INT, &args![button, id, value]);
        }
    }
    fn_007eb920(e, weapon);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu_object, 0u32]);
    e.call(EMPTY_MEMBER, &args![handle]);
    e.call(STRING_DESTRUCT_4037D0, &args![string]);
    e.call(NI_POINTER_DESTRUCT_45CEC0, &args![texture]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0075a0d0,
            list_box_unsigned_int_initialize(Ptr<ListBox>, u32, u32)
        ),
        entry!(0x007e9200, vats_menu_create() -> bool),
        entry!(0x007eb7f0, fn_007eb7f0(u32) -> f32),
        entry!(0x007eb8a0, fn_007eb8a0(u32)),
        entry!(0x007eb8c0, fn_007eb8c0() -> u32),
        entry!(0x007eb8d0, fn_007eb8d0(Ptr<VatsMenu>, u32) -> bool),
        entry!(0x007eb920, fn_007eb920(u32)),
        entry!(0x007ebd10, fn_007ebd10(u32) -> f32),
        entry!(0x007ebd30, fn_007ebd30(u32) -> f32),
        entry!(0x007ebd50, vats_menu_close()),
        entry!(0x007ec380, fn_007ec380(u32)),
        entry!(0x007ec3a0, vats_menu_show_target_limb(u8)),
        entry!(0x007ec810, vats_menu_do_idle(Ptr<VatsMenu>)),
        entry!(0x007eeb50, is_desired_attack_control_pressed(u32, u8, u32) -> bool),
        entry!(0x007eecb0, vats_menu_do_click(Ptr<VatsMenu>, u32, u32)),
        entry!(0x007ef710, vats_menu_do_key(Ptr<VatsMenu>, u32) -> bool),
        entry!(0x007ef760, vats_menu_do_wheel_move(Ptr<VatsMenu>, u32, u32)),
        entry!(0x007ef7b0, fn_007ef7b0(Ptr<VatsMenu>, u32, u32)),
        entry!(0x007ef830, fn_007ef830(Ptr<VatsMenu>, u32, u32)),
        entry!(0x007ef8b0, vats_menu_do_still_down(Ptr<VatsMenu>, u32, u32)),
        entry!(0x007ef930, fn_007ef930(Ptr<VatsMenu>) -> i32),
        entry!(
            0x007efa10,
            vats_menu_update_queued_action_list(Ptr<VatsMenu>, u32, u8, u8)
        ),
        entry!(0x007f0360, fn_007f0360(u32) -> u32),
        entry!(0x007f0380, fn_007f0380(u32, u32) -> i32),
        entry!(0x007f0500, fn_007f0500(u32, u32) -> i32),
        entry!(0x007f05b0, fn_007f05b0(Ptr<VatsMenu>) -> bool),
        entry!(0x007f06d0, fn_007f06d0(Ptr<VatsMenu>, u32) -> bool),
        entry!(0x007f07a0, vats_menu_update_chance_to_hit(Ptr<VatsMenu>)),
        entry!(0x007f0ea0, fn_007f0ea0(Ptr<VatsMenu>)),
        entry!(0x007f1290, fn_007f1290(Ptr<VatsMenu>, u32) -> f32),
        entry!(0x007f1840, fn_007f1840(Ptr<VatsMenu>)),
        entry!(0x007f2890, fn_007f2890(u32, u32, f32) -> u32),
        entry!(0x007f28d0, vats_menu_update_ammo(Ptr<VatsMenu>) -> bool),
        entry!(0x007f3070, fn_007f3070(Ptr<VatsMenu>)),
        entry!(0x007f3b40, fn_007f3b40(u32)),
        entry!(0x007f3bd0, fn_007f3bd0(u8)),
        entry!(
            0x007f3bf0,
            vats_menu_get_actor_value_from_body_part(Ptr<VatsMenu>, u32) -> u32
        ),
        entry!(
            0x007f3c90,
            vats_menu_get_vats_target(Ptr<VatsMenu>, u32) -> u32
        ),
        entry!(0x007f3cf0, vats_menu_get_target_part(Ptr<VatsMenu>) -> u32),
        entry!(
            0x007f3d60,
            vats_menu_find_target_part(Ptr<VatsMenu>, u32) -> u32
        ),
        entry!(0x007f3e00, fn_007f3e00(Ptr<VatsMenu>) -> bool),
        entry!(0x007f4810, fn_007f4810(u32) -> bool),
        entry!(0x007f4830, vats_menu_vats_menu_render_template(u32, u32) -> u32),
        entry!(0x007f4890, fn_007f4890(u32, u32) -> u32),
        entry!(0x007f48e0, fn_007f48e0(Ptr<VatsMenu>) -> f32),
        entry!(0x007f4bb0, fn_007f4bb0() -> bool),
        entry!(0x007f4c00, fn_007f4c00() -> bool),
        entry!(0x007f4c50, fn_007f4c50(Ptr<VatsMenu>, u8)),
        entry!(0x007f4f30, fn_007f4f30(u32) -> u8),
        entry!(0x007f5050, vats_menu_get_shot_burst_count(u32, u8) -> u8),
        entry!(0x007f5190, fn_007f5190(u32) -> bool),
        entry!(0x007f51b0, fn_007f51b0(Ptr<VatsMenu>, u32)),
        entry!(0x007f5280, fn_007f5280() -> u32),
        entry!(0x007f52c0, fn_007f52c0() -> bool),
        entry!(0x007f6820, fn_007f6820(u32) -> u32),
        entry!(0x007f68a0, fn_007f68a0(u32, u32)),
        entry!(0x007f6900, fn_007f6900(u32) -> u32),
        entry!(0x007f6920, fn_007f6920()),
        entry!(0x007f69d0, fn_007f69d0(u32, u32) -> u32),
        entry!(0x007f6a00, fn_007f6a00(u32)),
        entry!(0x007f6a20, fn_007f6a20(u32)),
        entry!(0x007f6af0, fn_007f6af0(Ptr<VatsMenu>) -> bool),
        entry!(
            0x007f6e50,
            vats_menu_build_meters_for_target(Ptr<VatsMenu>, u32)
        ),
        entry!(
            0x007f6fb0,
            list_box_unsigned_int_get_last_item(Ptr<ListBox>) -> u32
        ),
        entry!(
            0x007f6fe0,
            list_box_unsigned_int_add_item(Ptr<ListBox>, u32, u32, u32, u32) -> u32
        ),
        entry!(0x007f7280, fn_007f7280(Ptr<ListBox>, u32, u8) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every function outside the unit that the translations call; each
    /// gets a double answering 0 unless a test replaces it.
    const CALLEES: [u32; 312] = [
        ABS,
        ACTION_NAME,
        ACTION_POINTS_COST,
        ACTOR_8A6840,
        ACTOR_CHECK_5A2030,
        ACTOR_GET_ESSENTIAL,
        ACTOR_IS_IN_FACTION,
        ADD_FADE_CONTROL,
        ADD_USER_TRAIT,
        ALLOCATE,
        ARRAY_460140,
        ARRAY_AT_4A46B0,
        AUDIO_INSTANCE,
        AUDIO_MANAGER_INSTANCE,
        AUDIO_PAUSE_TYPE,
        AUDIO_RESUME_TYPE,
        AXIS_VALUE,
        BASE_FORM_OF,
        BODY_PART_ARRAY,
        BODY_PART_NAME,
        BODY_PART_NODE_NAME,
        BODY_PART_VALUE,
        BUILD_METERS_FOR_TARGET,
        BUTTON_STATE,
        CALENDAR_GET_HOUR,
        CAMERA_DIRECTION,
        CAMERA_HOLDER,
        CAMERA_OF,
        CAMERA_ROTATION,
        CELL_FLAG_425FD0,
        CELL_GATHER,
        CELL_WORLD_4543C0,
        CHARACTER_UPDATE_8D3550,
        CHECK_006450C0,
        CHECK_440D80,
        CHECK_440DA0,
        CHECK_477BA0,
        CHECK_MENU_BUTTON,
        CLOSE_VATS_MENU,
        COLLISION_OBJECT_931ED0,
        COMMAND_DAMAGE,
        COMPASS_SET_7700D0,
        COMPASS_SET_TEXTURE_7700B0,
        CONTROLS,
        CONTROL_KEY,
        CONTROL_PRESSED,
        CONTROL_STATE,
        CONVERT_4A3E00,
        COPY_696C30,
        DAMAGE_THRESHOLD,
        DIFFICULTY_ADJUST_HEALTH_MODIFIER,
        DYNAMIC_CAST,
        EMERGENCY_CLOSE_ALL_MENUS,
        EMPTY_MEMBER,
        END_ALL_SOUNDS_OF_TYPE,
        ENTRY_OBJECT_413F40,
        ERROR,
        FILTER_4A39F0,
        FILTER_59CE80,
        FIND_NODE,
        FIND_REFERENCE_FOR_3D,
        FIRST_CHILD_DATA,
        POINTER_GET_559450,
        FIXED_STRING_DESTRUCT,
        FIXED_STRING_FROM,
        FLAGS_SET_00407E00,
        FLAG_244_BIT_2,
        FLOAT_ABS,
        FLOAT_AT_378,
        FLOAT_FUNCTION_4F6E40,
        FLOAT_MAX,
        FLOAT_MIN,
        FORM_FULL_NAME,
        FORM_LIST_500940,
        FORM_OF_REFERENCE,
        FORM_WORD_368,
        FREE,
        FTOL,
        GAMEPAD_IN_USE,
        GET_AMMO_REGEN_RATE,
        GET_ATTACK_ACTION,
        GET_ATTACK_SPEED,
        GET_AV_OBJECT_FOR_COLLIDABLE,
        GET_BODY_PART,
        GET_BODY_PART_BY_LIMB,
        GET_BOUND_SIZE,
        GET_CURRENT_AMMO,
        GET_CURRENT_PROCESS_TYPE,
        GET_CURRENT_WEAPON,
        GET_DEFAULT_OBJECT,
        GET_DESTRUCTION_FORM,
        GET_DISTANCE_BETWEEN_FACING_ACTORS,
        GET_DISTANCE_FROM_REFERENCE,
        GET_FADE_END_FOR,
        GET_FADE_FINISHED,
        GET_FADE_TYPE,
        GET_FLOAT_TRAIT,
        GET_FORM_CLIP_ROUNDS,
        GET_FORM_HEALTH,
        GET_INVENTORY_CHANGES,
        GET_ITEM_HEALTH,
        GET_MAXIMUM_DEPTH,
        GET_MENU_BY_CLASS,
        GET_MODEL_BOUND_SIZE,
        GET_NUM_ROCK_IT_AMMO,
        GET_OBJECT_COUNT,
        GET_SOUND_HANDLE_BY_NAME,
        GET_SOUND_HANDLE_BY_NAME_ON_AUDIO,
        GET_VATS_CHANCE_TO_HIT,
        GET_WEAPON_ATTACK_ACTION_POINTS,
        GET_WEAPON_REACH,
        GET_WORLD_BOUND,
        HANDLE_ENTRY_POINT,
        HAS_MOD_EFFECT,
        HEADING_BETWEEN_LOCATIONS,
        HUD_SET_BRIGHTNESS,
        HUD_SHOW_MESSAGE,
        HUD_UPDATE_COMPASS,
        HUD_UPDATE_METER,
        HUD_UPDATE_METER_GHOST,
        IDENTITY,
        INITIALIZE_HIT_DATA,
        INT_MAX,
        INT_MIN,
        IS_ACTOR_PERCEIVED,
        IS_ATTACK_ACTION,
        IS_CONSOLE_VISIBLE,
        IS_DESTRUCTIBLE,
        IS_DESTRUCTIBLE_576070,
        IS_GOD_MODE,
        IS_ROCK_IT_LAUNCHER,
        IS_TOP_MENU_ID,
        LINE_OF_SIGHT,
        LIST_0063F7B0,
        LIST_APPEND,
        LIST_BOX_007F7280,
        LIST_BOX_ADD_ITEM,
        LIST_BOX_GET_LAST_ITEM,
        LIST_CLEAR,
        LIST_CONSTRUCT_96A2D0,
        LIST_CONTAINS_5F65D0,
        LIST_COUNT,
        LIST_DESTRUCT_46FFB0,
        LIST_IS_EMPTY,
        LIST_REMOVE_905330,
        LIST_SORT,
        LOOKUP_FORM,
        MATRIX_ENTRY_4B55C0,
        MATRIX_SET_ROW,
        MATRIX_TIMES_POINT,
        MENU_FADE_STATE,
        MENU_PATH_586150,
        MENU_PREP_FOR_VISIBILITY,
        MENU_RENDER_TEMPLATE,
        MENU_SET_MENU_TILE,
        MENU_START_FADE_OUT,
        NEXT_NODE,
        NI_POINTER_ASSIGN_66B0D0,
        NI_POINTER_CONSTRUCT_633C90,
        NI_POINTER_DESTRUCT_45CEC0,
        NODE_POSITION_43C490,
        OBJECT_004B7210,
        OBJECT_00705910,
        OBJECT_00709D40,
        OBJECT_0073B020,
        OBJECT_00747D00,
        OBJECT_CHECK_43B1B0,
        OBJECT_WORLD_POSITION,
        OWNER_874480,
        PARENT_CELL_8D6F30,
        PATHING_LOCATION_CONSTRUCT,
        PATHING_LOCATION_DESTRUCT,
        PATH_LENGTH,
        PLACE_REFERENCE_4698A0,
        PLAYER_FACTOR_8B0DD0,
        PLAYER_TARGET_LIST,
        PLAYER_UPDATE_945B0,
        POINT2_ADD,
        POINT2_EQUALS,
        POINT2_LENGTH,
        POINT2_NORMALIZE,
        POINT2_SCALE,
        POINT2_SUBTRACT,
        POINT3_ADD_IN_PLACE,
        POINT3_DOT,
        POINT3_EQUALS,
        POINT3_NORMALIZE,
        POINT3_SCALE,
        POINT3_SCALE_IN_PLACE,
        POINT3_SET,
        POINT_LENGTH,
        POINT_SUBTRACT,
        PRINT_005B5E40,
        QUERY_4A3DA0,
        QUERY_4A3EB0,
        QUERY_4A3F70,
        QUERY_4A3F90,
        QUERY_4A3FB0,
        QUERY_8CDD90,
        QUERY_CONSTRUCT_4A3C20,
        RANDOM_FLOAT,
        RAY_COLLECTOR_CONSTRUCT,
        RAY_COLLECTOR_DESTRUCT,
        REFERENCE_472380,
        REFERENCE_CHECK_452370,
        REFERENCE_GET_WORLDSPACE,
        REFERENCE_HEALTH,
        REFERENCE_NAME,
        REFERENCE_OBJECT_476C90,
        REMOVE_FADE_CONTROL,
        ROLL_CHANCE,
        SCENE_0044AC20,
        SCREEN_PERCENT_00800A00,
        SCREEN_WIDTH,
        SETTING_BOOL_ADDRESS,
        SETTING_INT_ADDRESS,
        SETTING_STRING,
        SETTING_VALUE_ADDRESS,
        SET_00800700,
        SET_008009D0,
        SET_00800A90,
        SET_00800AC0,
        SET_00800D10,
        SET_00800D50,
        SET_00800E50,
        SET_ALPHA_MULT,
        SET_FIRST_PERSON,
        SET_FLOAT_TRAIT,
        SET_IRON_SIGHTS,
        SIZE_00715DA0,
        SIZE_007177C0,
        SIZE_00717820,
        SKIP_FADE_IN,
        SOUND_HANDLE_ASSIGN,
        SOUND_HANDLE_CONSTRUCT,
        SOUND_HANDLE_PLAY,
        SOURCE_TEXTURE_CREATE,
        SPRINTF,
        SPRINTF_S,
        STATE_IS_THREE,
        STORE_8C71B0,
        STRCAT_S,
        STRCMP,
        STRCPY_S,
        STRING_CONSTRUCT_4037B0,
        STRING_DESTRUCT_4037D0,
        TES_CELL_5F36F0,
        TES_GET_WORLDSPACE,
        TILE_FROM_PATH,
        TILE_GET_CHILD_BY_NAME,
        TILE_GET_MENU,
        TILE_GET_STRING,
        TILE_IS_TRUE,
        TILE_IS_VISIBLE,
        TILE_READ_FILE,
        TILE_SET_INT,
        TILE_SET_STRING,
        TILE_TEXT_TO_TRAIT,
        TILE_UPDATE_ALL,
        TIMER_FLOAT_C,
        TIMER_MILLISECONDS,
        TUTORIAL_SHOW_MESSAGE,
        UPDATE_0070BC20,
        UPDATE_00800ED0,
        UPDATE_ANIMATION_MOVEMENT,
        UPDATE_AUTO_AIM_ACTOR,
        UPDATE_CAMERA,
        UPDATE_FIRST_PERSON_ZOOM,
        VATS_005D8710,
        VATS_009C9AE0,
        VATS_COMMAND_CONSTRUCT,
        VATS_COMMAND_DESTRUCT,
        VATS_COMMAND_INITIALIZE,
        VATS_GET_COUNT,
        VATS_IS_MELEE_COMMAND,
        VATS_LAST_COMMAND,
        VATS_MENU_007F6AF0,
        VATS_QUEUE_COMMAND,
        VATS_REMOVE_LAST_COMMAND,
        VATS_SET_MODE,
        WEAPON_102_524B60,
        WEAPON_CHECK_46E8C0,
        WEAPON_FLAG_100_BIT_2,
        WEAPON_VALUE_821640,
        WEAPON_KIND,
        WHEEL_VALUE,
        GROUP_4A3A20,
        WORD_AT_15C,
        WORD_AT_68,
        WORD_AT_8,
        WORLDSPACE_GATHER,
        WORLD_PT_TO_SCREEN_PT,
        WORLD_TO_CAMERA_MATRIX,
        SET_MULTI_LAYER_COLLISION,
        CAMERA_RAY_A71080,
        FLOAT_AT_40,
        FLOAT_AT_38,
        LIST_BOX_NUM_UNFILTERED,
        TILE_GET_VALUE,
        TILE_GET_VALUE_Q,
        TILE_VALUE_CLEAR_ACTIONS,
        TILE_VALUE_ADD_ACTION_TILE,
        TILE_VALUE_ADD_ACTION_FLOAT,
        TILE_VALUE_CALCULATE,
        LIST_ITEM_CONSTRUCT,
        LIST_INSERT,
        LIST_ADD_TAIL,
        LIST_BOX_FLAG_TEST,
        LIST_BOX_SYNCHRONIZE,
        LIST_BOX_INIT_TILE_POSITION,
        LIST_BOX_SET_TOTAL_LIST_HEIGHT,
        LIST_BOX_REMOVE_7F7380,
    ];

    type Log = Vec<Vec<u32>>;

    /// A virtual target that answers 0.
    const NOTHING: u32 = 0x0150_0000;
    /// A virtual target that answers 1.
    const TRUE_SLOT: u32 = 0x0150_0004;

    fn word(value: u32) -> Ret {
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

    fn arg_f32(word: u32) -> f64 {
        f32::from_bits(word) as f64
    }

    /// A fresh engine: the exe data pages mapped, every outside callee a
    /// do-nothing double, the accessors and float helpers as their bodies
    /// do, and the constants the code reads.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x21_0000);
        for address in CALLEES {
            e.register(address, |_, _| Ret::default());
        }
        e.register(NOTHING, |_, _| Ret::default());
        e.register(TRUE_SLOT, |_, _| word(1));
        e.register(IDENTITY, |_, a| word(a[0]));
        e.register(NEXT_NODE, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| {
            word(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e.register(WORD_AT_8, |e, a| word(e.mem.u32(a[0] + 8)));
        e.register(WORD_AT_68, |e, a| word(e.mem.u32(a[0] + 0x68)));
        e.register(SETTING_STRING, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(SETTING_VALUE_ADDRESS, |_, a| word(a[0] + 4));
        e.register(SETTING_INT_ADDRESS, |_, a| word(a[0] + 4));
        e.register(SETTING_BOOL_ADDRESS, |_, a| word(a[0] + 4));
        e.register(FLOAT_ABS, |_, a| float(arg_f32(a[0]).abs()));
        e.register(FLOAT_MAX, |_, a| {
            let (x, y) = (arg_f32(a[0]), arg_f32(a[1]));
            float(if x <= y { y } else { x })
        });
        e.register(FLOAT_MIN, |_, a| {
            let (x, y) = (arg_f32(a[0]), arg_f32(a[1]));
            float(if y <= x || x.is_nan() || y.is_nan() {
                y
            } else {
                x
            })
        });
        e.register(INT_MAX, |_, a| {
            word(if (a[1] as i32) < (a[0] as i32) {
                a[0]
            } else {
                a[1]
            })
        });
        e.register(INT_MIN, |_, a| {
            word(if (a[0] as i32) < (a[1] as i32) {
                a[0]
            } else {
                a[1]
            })
        });
        e.register(FTOL, |_, a| {
            word(f64::from_bits(a[0] as u64 | (a[1] as u64) << 32).trunc() as i32 as u32)
        });
        e.register(MENU_FADE_STATE, |e, a| word(e.mem.u32(a[0] + 0x24)));
        // Callees answering a pointer: their out argument, or the zeros at
        // `011f426c`.
        for address in [
            CAMERA_DIRECTION,
            MATRIX_TIMES_POINT,
            POINT_SUBTRACT,
            POINT2_SUBTRACT,
            POINT2_SCALE,
            POINT2_ADD,
            POINT3_SCALE,
        ] {
            e.register(address, |_, a| word(a[1]));
        }
        e.register(FIXED_STRING_FROM, |_, a| word(a[0]));
        for address in [
            OBJECT_WORLD_POSITION,
            CAMERA_ROTATION,
            BODY_PART_ARRAY,
            NODE_POSITION_43C490,
            FIRST_CHILD_DATA,
        ] {
            e.register(address, |_, _| word(POINT_ZERO));
        }
        let world = e.mem.alloc(0x400);
        let table = e.mem.alloc(0x400);
        for slot in (0..0x400).step_by(4) {
            e.mem.set_u32(table + slot, NOTHING);
        }
        e.mem.set_u32(world, table);
        e.register_double(CELL_WORLD_4543C0, move |_, _| word(world));
        for (address, value) in [
            (DOUBLE_ONE, 1.0),
            (DOUBLE_TWO, 2.0),
            (DOUBLE_HUNDRED, 100.0),
            (DOUBLE_180, 180.0),
            (DOUBLE_360, 360.0),
            (DOUBLE_DEGREES, 57.295_776_367_187_5),
            (DOUBLE_255, 255.0),
            (DOUBLE_45, 45.0),
            (DOUBLE_300, 300.0),
            (DOUBLE_128, 128.0),
            (DOUBLE_FIFTY, 50.0),
            (DOUBLE_HALF, 0.5),
            (DOUBLE_40, 40.0),
        ] {
            e.set_global::<f64>(address, value);
        }
        for (address, value) in [
            (FLOAT_TWO, 2.0f32),
            (FLOAT_HUNDRED, 100.0),
            (FLOAT_255, 255.0),
            (FLOAT_POINT_TWO, 0.2),
            (FLOAT_HALF, 0.5),
            (FLOAT_180, 180.0),
            (FLOAT_LARGEST, f32::MAX),
            (FLOAT_128, 128.0),
            (FLOAT_150, 150.0),
        ] {
            e.set_global::<f32>(address, value);
        }
        e.call_log = Some(vec![]);
        e
    }

    /// The argument words of every logged call to `address`.
    fn calls(e: &Engine, address: u32) -> Log {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A vtable whose slots answer 0, with `slots` (byte offset, target)
    /// replaced.
    fn vtable(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x400);
        for slot in (0..0x400).step_by(4) {
            e.mem.set_u32(table + slot, NOTHING);
        }
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        table
    }

    /// An object of 0x400 bytes with a vtable (see `vtable`) and a
    /// do-nothing sub-object vtable at `+0xA4`.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = vtable(e, slots);
        let object = e.mem.alloc(0x400);
        e.mem.set_u32(object, table);
        let owner = vtable(e, &[]);
        e.mem.set_u32(object + 0xa4, owner);
        object
    }

    /// The player: an object whose `+0x68` process is an object too.
    fn player(e: &mut Engine) -> u32 {
        let position = e.mem.alloc(12);
        e.register_double(0x0150_0060, move |_, _| word(position));
        let player = object(e, &[(0x1f4, 0x0150_0060)]);
        let process = object(e, &[]);
        e.mem.set_u32(player + 0x68, process);
        e.set_global::<u32>(PLAYER, player);
        player
    }

    /// The open menu: an object with 32 tiles.
    fn open_menu(e: &mut Engine) -> u32 {
        let menu = object(e, &[]);
        for index in 0..32 {
            let tile = e.mem.alloc(0x40);
            e.mem.set_u32(menu + 0x28 + 4 * index, tile);
        }
        e.set_global::<u32>(VATS_MENU, menu);
        menu
    }

    /// A list whose head node is at `head`: one node per item.
    fn list_at(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            let next = if index + 1 < items.len() {
                e.mem.alloc(8)
            } else {
                0
            };
            e.mem.set_u32(node + 4, next);
            node = next;
        }
    }

    /// A part with `limb` and a meter tile.
    fn part(e: &mut Engine, limb: u32) -> u32 {
        let part = e.mem.alloc(0x3c);
        e.mem.set_u32(part + 0x20, limb);
        let meter = e.mem.alloc(0x40);
        e.mem.set_u32(part + 0x30, meter);
        part
    }

    /// A target of `kind` for `reference` with `parts`.
    fn target(e: &mut Engine, reference: u32, kind: u32, parts: &[u32]) -> u32 {
        let target = e.mem.alloc(0x24);
        e.mem.set_u32(target, reference);
        e.mem.set_u32(target + 4, kind);
        list_at(e, target + 8, parts);
        target
    }

    /// A scene: the player, the open menu, an actor reference (slot `0x100`
    /// true) as the target reference with one target of type 1 and a part
    /// with limb 1 as the current limb.
    struct Scene {
        menu: u32,
        reference: u32,
        target: u32,
        part: u32,
    }

    fn scene(e: &mut Engine) -> Scene {
        player(e);
        let menu = open_menu(e);
        let reference = object(e, &[(0x100, TRUE_SLOT), (0x1d0, TRUE_SLOT)]);
        let process = object(e, &[]);
        e.mem.set_u32(reference + 0x68, process);
        let base = object(e, &[]);
        e.register_double(BASE_FORM_OF, move |_, _| word(base));
        let part = part(e, 1);
        let target = target(e, reference, 1, &[part]);
        list_at(e, LIST_011DB150, &[target]);
        e.set_global::<u32>(TARGET_REFERENCE, reference);
        e.mem.set_u32(menu + 0x100, part);
        Scene {
            menu,
            reference,
            target,
            part,
        }
    }

    #[test]
    fn list_box_initialize_finds_the_scroll_bar() {
        let mut e = engine();
        let list_box = e.mem.alloc(0x30);
        e.register(TILE_FROM_PATH, |_, a| word(a[0] + 0x10));
        list_box_unsigned_int_initialize(&mut e, Ptr::new(list_box), 0x5000, 0x6000);
        assert_eq!(e.mem.u32(list_box + 0xc), 0x5000);
        assert_eq!(e.mem.u32(list_box + 0x18), 0x6000);
        assert_eq!(e.mem.u32(list_box + 0x14), 0x5010);
        assert_eq!(
            calls(&e, TILE_FROM_PATH),
            vec![vec![0x5000, TEXT_CHILD_LB_SCROLLBAR]]
        );
    }

    #[test]
    fn fast_length_uses_the_table() {
        let mut e = engine();
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(table + 0x80 * 4, 0x0035_04f3);
        e.set_global::<u32>(SQRT_TABLE, table);
        let point = e.mem.alloc(12);
        assert_eq!(fn_007eb7f0(&mut e, point), 0.0);
        e.mem.set_f32(point, 2.0);
        assert_eq!(fn_007eb7f0(&mut e, point).to_bits(), 0x4000_0000);
        e.mem.set_f32(point, 1.0);
        e.mem.set_f32(point + 4, 1.0);
        assert_eq!(fn_007eb7f0(&mut e, point).to_bits(), 0x3fb5_04f3);
    }

    #[test]
    fn audio_type_pause_and_resume() {
        let mut e = engine();
        fn_007eb8a0(&mut e, 0x1234);
        fn_007ec380(&mut e, 0x1234);
        assert_eq!(calls(&e, AUDIO_PAUSE_TYPE), vec![vec![0x1234, 0x3002, 0]]);
        assert_eq!(calls(&e, AUDIO_RESUME_TYPE), vec![vec![0x1234, 0x3002, 0]]);
        assert_eq!(fn_007eb8c0(&mut e), 0x420);
    }

    #[test]
    fn missing_tile_is_found() {
        let mut e = engine();
        let menu = open_menu(&mut e);
        let index = e.mem.alloc(4);
        assert!(fn_007eb8d0(&mut e, Ptr::new(menu), index));
        assert_eq!(e.mem.i32(index), 32);
        e.mem.set_u32(menu + 0x28 + 5 * 4, 0);
        assert!(!fn_007eb8d0(&mut e, Ptr::new(menu), index));
        assert_eq!(e.mem.i32(index), 5);
    }

    #[test]
    fn special_attack_buttons() {
        let mut e = engine();
        let player = player(&mut e);
        let menu = open_menu(&mut e);
        e.register(0x0150_0010, |_, _| word(20));
        let owner = vtable(&mut e, &[(0x08, 0x0150_0010)]);
        e.mem.set_u32(player + 0xa4, owner);
        let form = e.mem.alloc(0x400);
        e.mem.set_f32(form + 0x37c, 5.0);
        e.mem.set_f32(form + 0x374, 10.0);
        e.register(FORM_WORD_368, |_, _| word(0x7777));
        fn_007eb920(&mut e, form);
        let button = e.mem.u32(menu + 0xa0);
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![button, 0xfc4, 0x7777, 1]]
        );
        assert_eq!(e.global::<u8>(SPECIAL_ONE_SHOWN), 1);
        assert_eq!(e.global::<f32>(SPECIAL_ONE_COST), 5.0);
        assert_eq!(
            calls(&e, HANDLE_ENTRY_POINT),
            vec![vec![0x28, player, form, SPECIAL_ONE_COST]]
        );
        // Unarmed: the first button above `fUpperCutThreshold`, not the second.
        e.call_log = Some(vec![]);
        e.mem.set_f32(SETTING_UPPERCUT_THRESHOLD + 4, 15.0);
        e.mem.set_f32(SETTING_UPPERCUT_AP_COST + 4, 20.0);
        e.mem.set_f32(SETTING_CROSS_THRESHOLD + 4, 90.0);
        e.mem.set_u32(SETTING_UNARMED_ONE + 4, 0x8888);
        fn_007eb920(&mut e, 0);
        assert_eq!(calls(&e, TILE_SET_STRING)[0][2], 0x8888);
        assert_eq!(e.global::<f32>(SPECIAL_ONE_COST), 20.0);
        assert_eq!(e.global::<u8>(SPECIAL_TWO_SHOWN), 0);
    }

    #[test]
    fn form_floats() {
        let mut e = engine();
        let form = e.mem.alloc(0x400);
        e.mem.set_f32(form + 0x37c, 1.5);
        e.mem.set_f32(form + 0x374, 2.5);
        assert_eq!(fn_007ebd10(&mut e, form), 1.5);
        assert_eq!(fn_007ebd30(&mut e, form), 2.5);
    }

    #[test]
    fn close_without_the_menu_tile_ends_vats() {
        let mut e = engine();
        player(&mut e);
        let menu = open_menu(&mut e);
        e.mem.set_u32(menu + 0x24, 1);
        e.set_global::<u8>(FLAG_011A3B31, 1);
        vats_menu_close(&mut e);
        assert_eq!(e.mem.u8(menu + 0xfb), 1);
        assert_eq!(calls(&e, VATS_SET_MODE), vec![vec![VATS, 0, 0]]);
        assert_eq!(e.global::<u8>(FLAG_011A3B31), 0);
        // With the menu tile and no queued action: the perk actors go.
        e.call_log = Some(vec![]);
        e.register(GET_MENU_BY_CLASS, |_, _| word(0x9000));
        e.register(FLAG_244_BIT_2, |_, _| word(1));
        e.register(0x0150_0020, |_, _| Ret::default());
        let stranger = object(&mut e, &[(0x10, 0x0150_0020)]);
        e.set_global::<u32>(STRANGER_ACTOR, stranger);
        vats_menu_close(&mut e);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x9000, 0x1772, 1]]);
        assert_eq!(calls(&e, 0x0150_0020), vec![vec![stranger, 1]]);
        assert_eq!(e.global::<u32>(STRANGER_ACTOR), 0);
        assert_eq!(calls(&e, SET_00800AC0), vec![vec![0, 0, 0xffff_ffff, 0]]);
    }

    #[test]
    fn show_target_limb_on_the_target() {
        let mut e = engine();
        let s = scene(&mut e);
        e.mem.set_u32(VATS + 8, 3);
        vats_menu_show_target_limb(&mut e, 1);
        assert_eq!(calls(&e, SET_00800AC0), vec![vec![0, s.reference, 1, 1]]);
        assert_eq!(e.global::<u32>(LAST_REFERENCE), s.reference);
        assert_eq!(e.global::<u32>(LAST_LIMB), 1);
        // Not forced and nothing new: nothing happens.
        e.call_log = Some(vec![]);
        e.mem.set_u32(VATS + 8, 1);
        vats_menu_show_target_limb(&mut e, 0);
        assert!(calls(&e, SET_00800AC0).is_empty());
    }

    #[test]
    fn idle_closes_or_drops_the_fades() {
        let mut e = engine();
        player(&mut e);
        let menu = open_menu(&mut e);
        e.mem.set_u8(menu + 0xfb, 1);
        vats_menu_do_idle(&mut e, Ptr::new(menu));
        assert_eq!(e.call_log.as_ref().unwrap().len(), 0);
        // No target: the menu closes.
        e.mem.set_u8(menu + 0xfb, 0);
        vats_menu_do_idle(&mut e, Ptr::new(menu));
        assert_eq!(calls(&e, MENU_FADE_STATE), vec![vec![menu]]);
        // A target outside mode 2: the meters lose their fade controls.
        e.call_log = Some(vec![]);
        let reference = object(&mut e, &[(0x1d0, TRUE_SLOT)]);
        e.set_global::<u32>(TARGET_REFERENCE, reference);
        vats_menu_do_idle(&mut e, Ptr::new(menu));
        assert_eq!(calls(&e, REMOVE_FADE_CONTROL).len(), 3);
        assert!(calls(&e, MENU_FADE_STATE).is_empty());
    }

    #[test]
    fn attack_controls() {
        let mut e = engine();
        e.register(CONTROL_PRESSED, |_, _| word(1));
        assert!(is_desired_attack_control_pressed(&mut e, 2, 0, 0));
        assert_eq!(calls(&e, CONTROL_PRESSED)[0][1..], [0x13, 1]);
        assert!(!is_desired_attack_control_pressed(&mut e, 4, 0, 0));
        e.register(GAMEPAD_IN_USE, |_, _| word(1));
        e.register(CHECK_MENU_BUTTON, |_, _| word(2));
        assert!(is_desired_attack_control_pressed(&mut e, 3, 0, 0));
        assert!(!is_desired_attack_control_pressed(&mut e, 3, 1, 0));
        assert_eq!(calls(&e, CHECK_MENU_BUTTON)[0], vec![0xb, 1]);
    }

    #[test]
    fn click_close_and_special_flags() {
        let mut e = engine();
        let s = scene(&mut e);
        e.mem.set_u32(VATS + 8, 3);
        vats_menu_do_click(&mut e, Ptr::new(s.menu), 4, 0);
        assert_eq!(calls(&e, CLOSE_VATS_MENU).len(), 1);
        assert_eq!(e.global::<u32>(GLOBAL_011A59F0), 0x19);
        vats_menu_do_click(&mut e, Ptr::new(s.menu), 0x1e, 0);
        assert_eq!(e.global::<u8>(FLAG_011DB0B2), 1);
        // A stick held long enough turns targets instead.
        e.call_log = Some(vec![]);
        e.mem.set_u8(s.menu + 0x10f, 1);
        e.register(TIMER_MILLISECONDS, |_, _| word(1000));
        vats_menu_do_click(&mut e, Ptr::new(s.menu), 0x1f, 0);
        assert_eq!(e.mem.u8(s.menu + 0x10f), 0);
        assert_eq!(e.global::<u8>(FLAG_011DB0B3), 0);
        assert_eq!(e.global::<u32>(TARGET_REFERENCE), s.reference);
        assert_eq!(e.mem.u32(s.menu + 0x100), s.part);
        assert_eq!(e.mem.u32(s.target), s.reference);
    }

    #[test]
    fn keys_wheel_and_buttons() {
        let mut e = engine();
        let menu = open_menu(&mut e);
        assert!(!vats_menu_do_key(&mut e, Ptr::new(menu), 0x41));
        assert!(vats_menu_do_key(&mut e, Ptr::new(menu), 0x62));
        e.register(0x0150_0030, |_, _| Ret::default());
        let table = vtable(&mut e, &[(0x0c, 0x0150_0030)]);
        e.mem.set_u32(menu, table);
        e.register(WHEEL_VALUE, |_, _| word(-120i32 as u32));
        vats_menu_do_wheel_move(&mut e, Ptr::new(menu), 0, 0);
        e.register(WHEEL_VALUE, |_, _| word(120));
        vats_menu_do_wheel_move(&mut e, Ptr::new(menu), 0, 0);
        assert_eq!(
            calls(&e, 0x0150_0030),
            vec![vec![menu, 2, 0], vec![menu, 3, 0]]
        );
        fn_007ef7b0(&mut e, Ptr::new(menu), 3, 0);
        assert_eq!(e.mem.u8(menu + 0x10e), 1);
        fn_007ef830(&mut e, Ptr::new(menu), 7, 0);
        assert_eq!(e.mem.u8(menu + 0x10e), 1);
        fn_007ef830(&mut e, Ptr::new(menu), 0x1f, 0);
        assert_eq!(e.mem.u8(menu + 0x10e), 0);
        e.register(TIMER_MILLISECONDS, |_, _| word(77));
        vats_menu_do_still_down(&mut e, Ptr::new(menu), 2, 0);
        assert_eq!((e.mem.u8(menu + 0x10f), e.mem.u32(menu + 0x114)), (1, 77));
        e.register(TIMER_MILLISECONDS, |_, _| word(99));
        vats_menu_do_still_down(&mut e, Ptr::new(menu), 2, 0);
        assert_eq!(e.mem.u32(menu + 0x114), 77);
    }

    #[test]
    fn repeated_commands_are_counted() {
        let mut e = engine();
        let command = |e: &mut Engine, target: u32, limb: u32| {
            let command = e.mem.alloc(0x28);
            e.mem.set_u32(command + 0xc, target);
            e.mem.set_u32(command + 0x10, limb);
            command
        };
        let a = command(&mut e, 1, 2);
        let b = command(&mut e, 1, 2);
        let c = command(&mut e, 1, 3);
        list_at(&mut e, VATS, &[a, b]);
        assert_eq!(fn_007ef930(&mut e, Ptr::new(0)), 1);
        list_at(&mut e, VATS, &[a, b, c]);
        assert_eq!(fn_007ef930(&mut e, Ptr::new(0)), 0);
    }

    #[test]
    fn queued_action_adds_a_line() {
        let mut e = engine();
        player(&mut e);
        let menu = open_menu(&mut e);
        e.mem.set_f32(menu + 0xe0, 10.0);
        let command = e.mem.alloc(0x28);
        e.mem.set_u32(command, 9);
        e.mem.set_f32(command + 0x20, 4.0);
        e.register(ACTION_NAME, |_, _| word(0x4444));
        e.register(LIST_BOX_ADD_ITEM, |_, _| word(0x5555));
        e.register(LIST_COUNT, |_, _| word(1));
        e.set_global::<u32>(ACTION_ITEM_COUNTER, 3);
        vats_menu_update_queued_action_list(&mut e, Ptr::new(menu), command, 0, 0);
        assert_eq!(e.mem.f32(menu + 0xe0), 6.0);
        assert_eq!(e.global::<u32>(ACTION_ITEM_COUNTER), 4);
        assert_eq!(calls(&e, STRCPY_S)[0][1..], [0x104, 0x4444]);
        assert_eq!(
            calls(&e, LIST_BOX_ADD_ITEM),
            vec![vec![menu + 0xb0, 4, 0, 0, 0]]
        );
        let tile = e.mem.u32(menu + 0x38);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![tile, 0xfaf, 1]]);
    }

    #[test]
    fn table_setting_text() {
        let mut e = engine();
        e.set_global::<u32>(SETTING_TABLE + 8, 0x0110_0000);
        e.set_global::<u32>(0x0110_0004, 0x1234);
        assert_eq!(fn_007f0360(&mut e, 2), 0x1234);
    }

    #[test]
    fn targets_by_facing_and_heading() {
        let mut e = engine();
        player(&mut e);
        let first = object(&mut e, &[]);
        let second = object(&mut e, &[]);
        let slot_a = e.mem.alloc(4);
        let slot_b = e.mem.alloc(4);
        e.mem.set_u32(slot_a, first);
        e.mem.set_u32(slot_b, second);
        let mut headings = vec![50.0, 10.0, 50.0, 10.0].into_iter();
        e.register_double(HEADING_BETWEEN_LOCATIONS, move |_, _| {
            float(headings.next().unwrap())
        });
        // Facing 0: the first argument is 50 away, the second 10: nearer.
        assert_eq!(fn_007f0380(&mut e, slot_b, slot_a), 1);
        // Headings 50 and 10: the second is below the first.
        assert_eq!(fn_007f0500(&mut e, slot_b, slot_a), 1);
        assert_eq!(calls(&e, HEADING_BETWEEN_LOCATIONS).len(), 4);
    }

    /// Projections answer `(x, y)` into their out words.
    fn project_to(e: &mut Engine, x: f32, y: f32) {
        e.register_double(WORLD_PT_TO_SCREEN_PT, move |e, a| {
            e.mem.set_f32(a[2], x);
            e.mem.set_f32(a[3], y);
            Ret::default()
        });
    }

    #[test]
    fn parts_on_screen() {
        let mut e = engine();
        let s = scene(&mut e);
        project_to(&mut e, 0.5, 0.5);
        e.mem.set_u8(s.part + 0x2c, 1);
        assert!(!fn_007f05b0(&mut e, Ptr::new(s.menu)));
        e.mem.set_u8(s.part + 0x2c, 0);
        assert!(fn_007f05b0(&mut e, Ptr::new(s.menu)));
        project_to(&mut e, 1.5, 0.5);
        assert!(!fn_007f06d0(&mut e, Ptr::new(s.menu), s.part));
        assert_eq!(e.mem.u8(s.part + 0x2c), 0);
        project_to(&mut e, f32::NAN, 0.5);
        assert!(fn_007f06d0(&mut e, Ptr::new(s.menu), s.part));
        assert_eq!(e.mem.u8(s.part + 0x2c), 1);
    }

    #[test]
    fn chance_meters_hide_outside_mode_two() {
        let mut e = engine();
        let s = scene(&mut e);
        e.mem.set_u32(VATS + 8, 1);
        e.register(GET_FLOAT_TRAIT, |_, _| float(1.0));
        vats_menu_update_chance_to_hit(&mut e, Ptr::new(s.menu));
        let meter = e.mem.u32(s.part + 0x30);
        assert_eq!(calls(&e, REMOVE_FADE_CONTROL), vec![vec![meter, 0xfa9]]);
        assert_eq!(calls(&e, SET_FLOAT_TRAIT), vec![vec![meter, 0xfa9, 0, 1]]);
        assert_eq!(calls(&e, ADD_USER_TRAIT).len(), 4);
        e.call_log = Some(vec![]);
        vats_menu_update_chance_to_hit(&mut e, Ptr::new(s.menu));
        assert!(calls(&e, ADD_USER_TRAIT).is_empty());
        assert_eq!(e.global::<u32>(STATIC_TRAIT_GUARD), 0xf);
    }

    #[test]
    fn chance_text() {
        let mut e = engine();
        let s = scene(&mut e);
        let other = object(&mut e, &[]);
        e.mem.set_u32(s.target, other);
        e.set_global::<u32>(TARGET_REFERENCE, other);
        e.mem.set_u32(s.part + 0x20, 3);
        e.mem.set_f32(s.part + 0x28, 0.25);
        fn_007f0ea0(&mut e, Ptr::new(s.menu));
        let text = calls(&e, SPRINTF_S);
        assert_eq!(text.len(), 1);
        assert_eq!(text[0][1..], [8, TEXT_PERCENT_FORMAT, 25]);
    }

    #[test]
    fn chance_to_hit_of_a_part() {
        let mut e = engine();
        let s = scene(&mut e);
        assert_eq!(fn_007f1290(&mut e, Ptr::new(s.menu), 0), 0.0);
        e.register(GET_VATS_CHANCE_TO_HIT, |_, _| float(0.6));
        e.register(POINT_LENGTH, |_, _| float(10.0));
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, s.part);
        let chance = fn_007f1290(&mut e, Ptr::new(s.menu), node);
        assert_eq!(chance, 0.6);
        assert_eq!(e.mem.f32(s.part + 0x28), 0.6);
        assert_eq!(e.mem.u8(s.menu + 0x118), 1);
        assert_eq!(e.mem.u32(s.menu + 0x128), s.reference);
        let entries = calls(&e, HANDLE_ENTRY_POINT);
        assert_eq!(entries[0][0], 8);
        assert_eq!(entries[1][0], 0x21);
    }

    #[test]
    fn meter_layout() {
        let mut e = engine();
        let s = scene(&mut e);
        e.mem.set_u32(s.part + 0x20, 3);
        e.register(SCREEN_WIDTH, |_, _| float(1000.0));
        e.register(SIZE_00715DA0, |_, _| float(500.0));
        fn_007f1840(&mut e, Ptr::new(s.menu));
        assert_eq!(e.global::<u32>(LAYOUT_GUARD), 0x3f);
        assert_eq!(e.global::<i32>(LAYOUT_HALF_WIDTH), 500);
        assert_eq!(e.mem.u8(s.part + 0x2c), 1);
        let meter = e.mem.u32(s.part + 0x30);
        let ints = calls(&e, TILE_SET_INT);
        assert!(ints.contains(&vec![meter, 0xfb7, 4]));
        assert_eq!(e.mem.f32(s.part + 4), 500.0);
    }

    #[test]
    fn point_division() {
        let mut e = engine();
        let point = e.mem.alloc(8);
        let out = e.mem.alloc(8);
        e.mem.set_f32(point, 4.0);
        e.mem.set_f32(point + 4, 6.0);
        assert_eq!(fn_007f2890(&mut e, point, out, 2.0), out);
        assert_eq!((e.mem.f32(out), e.mem.f32(out + 4)), (2.0, 3.0));
    }

    #[test]
    fn ammo_without_a_weapon() {
        let mut e = engine();
        player(&mut e);
        let menu = open_menu(&mut e);
        e.mem.set_f32(menu + 0xf0, 3.0);
        assert!(!vats_menu_update_ammo(&mut e, Ptr::new(menu)));
        assert_eq!(e.mem.f32(menu + 0xf0), 0.0);
        let text = calls(&e, SPRINTF_S);
        assert_eq!(text[0][1..], [0x10, TEXT_AMMO_FORMAT, 0, 0]);
        assert_eq!(calls(&e, TILE_SET_INT).len(), 5);
    }

    #[test]
    fn limb_pick_without_a_weapon() {
        let mut e = engine();
        let s = scene(&mut e);
        e.mem.set_u32(VATS + 8, 1);
        e.mem.set_u32(s.target + 4, 2);
        fn_007f3070(&mut e, Ptr::new(s.menu));
        assert_eq!(e.global::<i32>(GLOBAL_011A59F0), -1);
        e.mem.set_u32(s.target + 4, 1);
        fn_007f3070(&mut e, Ptr::new(s.menu));
        assert_eq!(e.global::<i32>(GLOBAL_011A59F0), -1);
    }

    #[test]
    fn limb_lookups() {
        let mut e = engine();
        let s = scene(&mut e);
        let second = part(&mut e, 5);
        list_at(&mut e, s.target + 8, &[s.part, second]);
        fn_007f3b40(&mut e, 5);
        assert_eq!(e.mem.u32(s.menu + 0x100), second);
        fn_007f3bd0(&mut e, 1);
        assert_eq!(e.mem.u8(s.menu + 0xf8), 1);
        assert_eq!(
            vats_menu_get_vats_target(&mut e, Ptr::new(s.menu), s.reference),
            LIST_011DB150
        );
        assert_eq!(vats_menu_get_vats_target(&mut e, Ptr::new(s.menu), 0), 0);
        let node = vats_menu_get_target_part(&mut e, Ptr::new(s.menu));
        assert_eq!(e.mem.u32(node), second);
        // No body part data: every limb is 0x1a, limb 14 -1.
        assert_eq!(
            vats_menu_get_actor_value_from_body_part(&mut e, Ptr::new(s.menu), 3),
            0x1a
        );
        assert_eq!(
            vats_menu_get_actor_value_from_body_part(&mut e, Ptr::new(s.menu), 0xe),
            0xffff_ffff
        );
        assert_eq!(
            vats_menu_find_target_part(&mut e, Ptr::new(s.menu), 0x1a),
            s.target + 8
        );
        assert_eq!(vats_menu_find_target_part(&mut e, Ptr::new(s.menu), 3), 0);
    }

    #[test]
    fn scan_lights_the_targets_in_mode_one() {
        let mut e = engine();
        let s = scene(&mut e);
        e.mem.set_u8(s.menu + 0xfb, 1);
        assert!(!fn_007f3e00(&mut e, Ptr::new(s.menu)));
        e.mem.set_u8(s.menu + 0xfb, 0);
        e.mem.set_u32(VATS + 8, 1);
        assert!(fn_007f3e00(&mut e, Ptr::new(s.menu)));
        assert_eq!(e.mem.u8(s.menu + 0xfa), 1);
        assert_eq!(calls(&e, SET_00800D50), vec![vec![0, s.reference]]);
        assert_eq!(calls(&e, SET_00800E50), vec![vec![0, s.reference]]);
        assert_eq!(calls(&e, SCENE_0044AC20).len(), 1);
    }

    #[test]
    fn templates_and_failures() {
        let mut e = engine();
        let object = e.mem.alloc(0x2c0);
        e.mem.set_u8(object + 0x2b4, 1);
        assert!(fn_007f4810(&mut e, object));
        assert_eq!(vats_menu_vats_menu_render_template(&mut e, 1, 2), 0);
        let menu = open_menu(&mut e);
        e.register(MENU_RENDER_TEMPLATE, |_, _| word(0x4321));
        assert_eq!(vats_menu_vats_menu_render_template(&mut e, 1, 2), 0x4321);
        assert_eq!(calls(&e, MENU_RENDER_TEMPLATE), vec![vec![menu, 1, 2, 0]]);
        assert_eq!(fn_007f4890(&mut e, 1, 2), 0);
        assert_eq!(
            calls(&e, PRINT_005B5E40),
            vec![vec![TEXT_MENU_CREATION_FAILED]]
        );
        assert_eq!(calls(&e, EMERGENCY_CLOSE_ALL_MENUS).len(), 1);
    }

    #[test]
    fn probable_damage() {
        let mut e = engine();
        let s = scene(&mut e);
        e.set_global::<u32>(TARGET_REFERENCE, 0);
        assert_eq!(fn_007f48e0(&mut e, Ptr::new(s.menu)), 0.0);
        e.set_global::<u32>(TARGET_REFERENCE, s.reference);
        e.register(0x0150_0040, |_, _| word(100));
        e.register(0x0150_0044, |_, _| word(200));
        let owner = vtable(&mut e, &[(0x08, 0x0150_0040), (0x00, 0x0150_0044)]);
        e.mem.set_u32(s.reference + 0xa4, owner);
        e.register(DIFFICULTY_ADJUST_HEALTH_MODIFIER, |_, _| float(-50.0));
        assert_eq!(fn_007f48e0(&mut e, Ptr::new(s.menu)), 0.25);
    }

    #[test]
    fn select_timers_and_turning() {
        let mut e = engine();
        let s = scene(&mut e);
        e.register(TIMER_MILLISECONDS, |_, _| word(1000));
        e.mem.set_u32(s.menu + 0x114, 400);
        assert!(!fn_007f4bb0(&mut e));
        e.mem.set_u8(s.menu + 0x10f, 1);
        assert!(fn_007f4bb0(&mut e));
        e.mem.set_u8(s.menu + 0x110, 1);
        e.mem.set_u32(s.menu + 0x114, 600);
        assert!(!fn_007f4c00(&mut e));
        // Turning right: the target 20 degrees off becomes the target.
        let other = object(&mut e, &[]);
        let other_part = part(&mut e, 4);
        let other_target = target(&mut e, other, 1, &[other_part]);
        list_at(&mut e, LIST_011DB150, &[s.target, other_target]);
        let mut headings = vec![-30.0, -20.0].into_iter();
        e.register_double(HEADING_BETWEEN_LOCATIONS, move |_, _| {
            float(headings.next().unwrap())
        });
        fn_007f4c50(&mut e, Ptr::new(s.menu), 0);
        assert_eq!(e.global::<u32>(TARGET_REFERENCE), other);
        assert_eq!(e.mem.u32(s.menu + 0x100), other_part);
    }

    #[test]
    fn gather_callback_adds_references() {
        let mut e = engine();
        player(&mut e);
        e.register(REFERENCE_CHECK_452370, |_, _| word(1));
        let reference = object(&mut e, &[(0x224, TRUE_SLOT)]);
        e.register_double(LIST_APPEND, |e, a| word(e.mem.u32(a[1])));
        assert_eq!(fn_007f4f30(&mut e, reference), 0);
        let appended = calls(&e, LIST_APPEND);
        assert_eq!(appended.len(), 1);
        assert_eq!(appended[0][0], LIST_011DB124);
    }

    #[test]
    fn shot_bursts() {
        let mut e = engine();
        let player = player(&mut e);
        assert_eq!(vats_menu_get_shot_burst_count(&mut e, 5, 0), 1);
        let weapon = e.mem.alloc(0x200);
        e.mem.set_u8(weapon + 0x102, 3);
        e.register(
            WEAPON_102_524B60,
            |e, a| word(e.mem.u8(a[0] + 0x102) as u32),
        );
        let equipped = e.mem.alloc(0x10);
        e.mem.set_u32(equipped + 8, weapon);
        e.register_double(0x0150_0050, move |_, _| word(equipped));
        let process = e.mem.u32(player + 0x68);
        let table = vtable(&mut e, &[(0x148, 0x0150_0050)]);
        e.mem.set_u32(process, table);
        assert_eq!(vats_menu_get_shot_burst_count(&mut e, 0, 0), 3);
        e.register(WEAPON_FLAG_100_BIT_2, |_, _| word(1));
        e.register(GET_ATTACK_SPEED, |_, _| float(2.0));
        e.register(WEAPON_VALUE_821640, |_, _| float(1.0));
        e.mem.set_f32(SETTING_SHOT_BURST_TIME + 4, 2.5);
        assert_eq!(vats_menu_get_shot_burst_count(&mut e, 5, 0), 5);
        e.mem.set_u32(weapon + 0x12c, 0x800);
        assert!(fn_007f5190(&mut e, weapon));
    }

    #[test]
    fn view_projection_rows() {
        let mut e = engine();
        e.register(CAMERA_DIRECTION, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[1] + 4, 2.0);
            e.mem.set_f32(a[1] + 8, 3.0);
            word(a[1])
        });
        e.register(MATRIX_SET_ROW, |e, a| {
            e.mem.set_f32(a[0] + 12 * a[1], e.mem.f32(a[2]));
            Ret::default()
        });
        let matrix = e.mem.alloc(0x24);
        fn_007f51b0(&mut e, Ptr::new(0), matrix);
        assert_eq!(e.mem.f32(matrix), 13.0);
        assert_eq!(e.mem.f32(matrix + 12), -2.0);
        assert_eq!(e.mem.f32(matrix + 24), -3.0);
    }

    #[test]
    fn temporary_command() {
        let mut e = engine();
        assert_eq!(fn_007f5280(&mut e), 0);
        let menu = open_menu(&mut e);
        assert_eq!(fn_007f5280(&mut e), 0);
        e.mem.set_u8(menu + 0x118, 1);
        assert_eq!(fn_007f5280(&mut e), menu + 0x11c);
    }

    #[test]
    fn target_building() {
        let mut e = engine();
        player(&mut e);
        e.mem.set_u32(LIST_011DB150, 0x1234);
        assert!(fn_007f52c0(&mut e));
        e.mem.set_u32(LIST_011DB150, 0);
        assert!(!fn_007f52c0(&mut e));
        assert_eq!(calls(&e, RAY_COLLECTOR_CONSTRUCT).len(), 1);
        assert_eq!(calls(&e, RAY_COLLECTOR_DESTRUCT).len(), 1);
        assert_eq!(calls(&e, CELL_GATHER)[0][5], GATHER_CALLBACK);
    }

    #[test]
    fn constructors_and_destructors() {
        let mut e = engine();
        let part = e.mem.alloc(0x3c);
        assert_eq!(fn_007f6820(&mut e, part), part);
        assert_eq!(e.mem.i32(part + 0x20), -1);
        assert_eq!((e.mem.u8(part + 0x2e), e.mem.u8(part + 0x2f)), (1, 1));
        let target = e.mem.alloc(0x24);
        assert_eq!(fn_007f6900(&mut e, target), target);
        assert_eq!(calls(&e, LIST_CONSTRUCT_96A2D0), vec![vec![target + 8]]);
        fn_007f69d0(&mut e, target, 0);
        assert!(calls(&e, FREE).is_empty());
        fn_007f69d0(&mut e, target, 1);
        assert_eq!(calls(&e, FREE), vec![vec![target]]);
        assert_eq!(calls(&e, LIST_DESTRUCT_46FFB0).len(), 2);
        e.register(CONVERT_4A3E00, |_, _| word(0x2468));
        fn_007f68a0(&mut e, 0x1357, 0x9999);
        assert_eq!(calls(&e, QUERY_4A3F90), vec![vec![0x1357, 0x2468]]);
    }

    #[test]
    fn target_list_is_emptied() {
        let mut e = engine();
        let part = e.mem.alloc(0x3c);
        let target = target(&mut e, 0x77, 1, &[part]);
        list_at(&mut e, LIST_011DB150, &[target]);
        e.register(LIST_0063F7B0, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        fn_007f6920(&mut e);
        assert_eq!(calls(&e, FREE), vec![vec![part], vec![target]]);
        assert_eq!(e.mem.u32(LIST_011DB150), 0);
    }

    #[test]
    fn create_refuses() {
        let mut e = engine();
        let player = player(&mut e);
        e.register(TILE_GET_MENU, |_, _| word(0x5000));
        assert!(!vats_menu_create(&mut e));
        assert!(calls(&e, LIST_CLEAR).is_empty());
        // No target left: back to first person.
        e.register(TILE_GET_MENU, |_, _| word(0));
        e.set_global::<u8>(FLAG_011F21D1, 1);
        e.register(LIST_CLEAR, |e, _| {
            e.mem.set_u32(LIST_011DB150, 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!vats_menu_create(&mut e));
        assert_eq!(calls(&e, SET_FIRST_PERSON), vec![vec![player, 1]]);
    }

    #[test]
    fn targets_of_a_reference_are_removed() {
        let mut e = engine();
        let s = scene(&mut e);
        let table = e.mem.u32(s.menu);
        e.mem.set_u32(table + 0x0c, 0x0150_0200);
        e.register(0x0150_0200, |_, _| Ret::default());
        e.register(LIST_REMOVE_905330, |e, _| {
            e.mem.set_u32(LIST_011DB150, 0);
            e.mem.set_u32(LIST_011DB150 + 4, 0);
            Ret::default()
        });
        // Another reference: nothing is removed.
        fn_007f6a20(&mut e, 0x4321);
        assert!(calls(&e, LIST_REMOVE_905330).is_empty());
        fn_007f6a20(&mut e, s.reference);
        assert_eq!(calls(&e, 0x0150_0200), vec![vec![s.menu, 2, 0]]);
        assert_eq!(
            calls(&e, LIST_REMOVE_905330),
            vec![vec![LIST_011DB150, LIST_011DB150]]
        );
        // The list is empty: the menu closes (Close reads its fade state).
        assert_eq!(calls(&e, MENU_FADE_STATE).len(), 1);
        // An empty list: nothing at all.
        e.call_log = Some(vec![]);
        fn_007f6a20(&mut e, s.reference);
        assert!(calls(&e, IDENTITY).is_empty());
    }

    #[test]
    fn pointer_picks_a_target() {
        let mut e = engine();
        let s = scene(&mut e);
        e.mem.set_u8(s.menu + 0x10e, 1);
        assert!(!fn_007f6af0(&mut e, Ptr::new(s.menu)));
        assert!(calls(&e, RAY_COLLECTOR_CONSTRUCT).is_empty());
        e.mem.set_u8(s.menu + 0x10e, 0);
        e.set_global::<f32>(FLOAT_CAST_LENGTH, 2560.0);
        e.register(FLOAT_AT_40, |_, _| float(3.7));
        e.register(FLOAT_AT_38, |_, _| float(5.2));
        let hits = e.mem.alloc(8);
        e.mem.set_u32(hits + 4, 1);
        e.register_double(ARRAY_460140, move |_, _| word(hits));
        e.register(GET_AV_OBJECT_FOR_COLLIDABLE, |_, _| word(0x7000));
        let reference = s.reference;
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, _| word(reference));
        // The hit is the current target: false.
        assert!(!fn_007f6af0(&mut e, Ptr::new(s.menu)));
        let camera_rays = calls(&e, CAMERA_RAY_A71080);
        assert_eq!((camera_rays[0][1], camera_rays[0][2]), (5, 3));
        assert_eq!(
            calls(&e, SET_MULTI_LAYER_COLLISION),
            vec![vec![0x27, 0x140, 0]]
        );
        let scaled = calls(&e, POINT3_SCALE);
        assert_eq!(scaled[0][0], camera_rays[0][4]);
        assert_eq!(scaled[0][2], 2560.0f32.to_bits());
        assert_eq!(calls(&e, RAY_COLLECTOR_DESTRUCT).len(), 1);
        // Another target is hit: it becomes the target reference (after
        // `UpdateChanceToHit`, which still sees the current one).
        let other = object(&mut e, &[]);
        let other_part = part(&mut e, 1);
        let other_target = target(&mut e, other, 1, &[other_part]);
        list_at(&mut e, LIST_011DB150, &[s.target, other_target]);
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, _| word(other));
        e.mem.set_u32(VATS + 8, 1);
        e.register(GET_FLOAT_TRAIT, |_, _| float(1.0));
        assert!(fn_007f6af0(&mut e, Ptr::new(s.menu)));
        assert_eq!(e.global::<u32>(TARGET_REFERENCE), other);
        // No hit has a target: false.
        e.register(FIND_REFERENCE_FOR_3D, |_, _| word(0x4321));
        assert!(!fn_007f6af0(&mut e, Ptr::new(s.menu)));
        assert_eq!(e.global::<u32>(TARGET_REFERENCE), other);
    }

    #[test]
    fn meters_for_a_target() {
        let mut e = engine();
        let menu = open_menu(&mut e);
        e.register(GET_MENU_BY_CLASS, |_, _| word(0x900));
        e.register(TILE_FROM_PATH, |_, _| word(0x910));
        e.register(MENU_RENDER_TEMPLATE, |_, _| word(0x920));
        let parts = [part(&mut e, 0), part(&mut e, 5), part(&mut e, 15)];
        let target = target(&mut e, 0x77, 1, &parts);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, target);
        vats_menu_build_meters_for_target(&mut e, Ptr::new(menu), node);
        assert_eq!(
            calls(&e, TILE_FROM_PATH),
            vec![vec![0x900, TEXT_GLOW_BRANCH]]
        );
        assert_eq!(
            calls(&e, MENU_RENDER_TEMPLATE)[0],
            vec![menu, 0x910, TEXT_BODY_PART_PERCENT, 0]
        );
        assert_eq!(e.mem.u32(parts[2] + 0x30), 0x920);
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![0x920, 0xfb7, 6], vec![0x920, 0xfb7, 4]]
        );
        // No meter rendered: no trait.
        e.register(MENU_RENDER_TEMPLATE, |_, _| word(0));
        e.call_log = Some(vec![]);
        vats_menu_build_meters_for_target(&mut e, Ptr::new(menu), node);
        assert!(calls(&e, TILE_SET_INT).is_empty());
        assert_eq!(e.mem.u32(parts[0] + 0x30), 0);
    }

    #[test]
    fn last_item_of_a_list_box() {
        let mut e = engine();
        let list_box = object(&mut e, &[(0x14, 0x0150_0300)]);
        e.register(0x0150_0300, |_, a| {
            assert_eq!(a[2], 0);
            word(a[1] * 10)
        });
        e.register(LIST_BOX_NUM_UNFILTERED, |_, _| word(3));
        let item = list_box_unsigned_int_get_last_item(&mut e, Ptr::new(list_box));
        assert_eq!(item, 20);
        assert_eq!(calls(&e, LIST_BOX_NUM_UNFILTERED), vec![vec![list_box]]);
    }

    /// The word the last `00905820` double was handed (by address).
    const ADDED_ITEM: u32 = 0x0120_0000;

    fn list_box_engine() -> (Engine, u32) {
        let mut e = engine();
        let list_box = e.mem.alloc(0x30);
        e.register(TILE_GET_MENU, |_, _| word(0x3100));
        e.register(MENU_RENDER_TEMPLATE, |_, _| word(0x3200));
        e.register(ALLOCATE, |e, a| word(e.mem.alloc(a[0])));
        e.register(LIST_ITEM_CONSTRUCT, |e, a| word(e.mem.u32(a[2]) + 1));
        e.register(LIST_ADD_TAIL, |e, a| {
            let added = e.mem.u32(a[1]);
            e.mem.set_u32(ADDED_ITEM, added);
            Ret::default()
        });
        e.register(LIST_BOX_FLAG_TEST, |_, _| word(1));
        e.register(TILE_GET_VALUE_Q, |_, _| word(0x3300));
        (e, list_box)
    }

    #[test]
    fn list_box_add_item() {
        let (mut e, list_box) = list_box_engine();
        let this = Ptr::new(list_box);
        assert_eq!(list_box_unsigned_int_add_item(&mut e, this, 1, 2, 0, 3), 0);
        e.mem.set_u32(list_box + 0x0c, 0x3000);
        assert_eq!(list_box_unsigned_int_add_item(&mut e, this, 1, 2, 0, 0), 0);
        assert!(calls(&e, MENU_RENDER_TEMPLATE).is_empty());
        // At the end, the first item: the visible count actions.
        e.mem.set_u32(list_box + 0x18, 0x6000);
        e.mem.set_u16(list_box + 0x1c, 7);
        e.register(LIST_COUNT, |_, _| word(1));
        e.register(GET_FLOAT_TRAIT, |_, _| float(2.5));
        let tile = list_box_unsigned_int_add_item(&mut e, this, 0x41, 0x4444, 0, 0);
        assert_eq!(tile, 0x3200);
        assert_eq!(
            calls(&e, MENU_RENDER_TEMPLATE),
            vec![vec![0x3100, 0x3000, 0x6000, 0]]
        );
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![0x3200, 0xfaa, 0xffff_ffff], vec![0x3200, 0xfac, 7]]
        );
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![0x3200, 0xfc4, 0x4444, 1]]
        );
        assert_eq!(e.mem.u32(ADDED_ITEM), 0x42);
        assert_eq!(e.mem.u16(list_box + 0x1c), 8);
        assert_eq!(
            calls(&e, LIST_BOX_INIT_TILE_POSITION),
            vec![vec![list_box, 0x3200]]
        );
        assert_eq!(calls(&e, LIST_BOX_SET_TOTAL_LIST_HEIGHT).len(), 1);
        assert_eq!(
            calls(&e, TILE_VALUE_ADD_ACTION_TILE),
            vec![vec![0x3300, 0x7d0, 0x3200, 0xfb0]]
        );
        assert_eq!(
            calls(&e, TILE_VALUE_ADD_ACTION_FLOAT),
            vec![vec![0x3300, 0x7d3, 2.5f32.to_bits()]]
        );
        assert_eq!(calls(&e, TILE_VALUE_CALCULATE), vec![vec![0x3300, 0]]);
        // After a position, with its own template and a value present; a NaN
        // count adds no actions.
        e.register(TILE_GET_VALUE, |_, _| word(0x10));
        e.register(GET_FLOAT_TRAIT, |_, _| float(f64::NAN));
        e.call_log = Some(vec![]);
        list_box_unsigned_int_add_item(&mut e, this, 0x41, 0, 0x55, 0x66);
        assert_eq!(calls(&e, MENU_RENDER_TEMPLATE)[0][2], 0x66);
        let inserted = calls(&e, LIST_INSERT);
        assert_eq!(inserted, vec![vec![list_box + 4, 0x42, 0x55]]);
        assert_eq!(calls(&e, LIST_BOX_SYNCHRONIZE), vec![vec![list_box]]);
        assert!(calls(&e, TILE_SET_INT).is_empty());
        assert!(calls(&e, TILE_SET_STRING).is_empty());
        assert!(calls(&e, TILE_VALUE_CLEAR_ACTIONS).is_empty());
        assert_eq!(e.mem.u16(list_box + 0x1c), 8);
    }

    #[test]
    fn list_box_remove_by_index_or_value() {
        let mut e = engine();
        let list_box = e.mem.alloc(0x30);
        let this = Ptr::new(list_box);
        e.register(LIST_BOX_REMOVE_7F7380, |_, _| word(1));
        assert!(!fn_007f7280(&mut e, this, 0, 1));
        assert!(calls(&e, LIST_BOX_REMOVE_7F7380).is_empty());
        let mut items = vec![];
        for value in [f32::NAN, 4.0, 9.0] {
            let tile = e.mem.alloc(4);
            e.mem.set_f32(tile, value);
            let item = e.mem.alloc(0x0c);
            e.mem.set_u32(item, tile);
            items.push(item);
        }
        list_at(&mut e, list_box + 4, &items);
        let second = e.mem.u32(list_box + 8);
        let third = e.mem.u32(second + 4);
        e.register(GET_FLOAT_TRAIT, |e, a| float(e.mem.f32(a[0]) as f64));
        assert!(fn_007f7280(&mut e, this, 1, 1));
        assert!(fn_007f7280(&mut e, this, 5, 1));
        assert!(fn_007f7280(&mut e, this, 9, 0));
        assert!(fn_007f7280(&mut e, this, 7, 0));
        assert_eq!(
            calls(&e, LIST_BOX_REMOVE_7F7380),
            vec![
                vec![list_box, second, list_box + 4],
                vec![list_box, 0, third],
                vec![list_box, third, second],
                vec![list_box, 0, third],
            ]
        );
    }
}
