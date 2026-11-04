//! `fallout/ai/playercharacter.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PC `PlayerCharacter` is 0xE50 bytes. Like every class derived from
//! `TESForm` its fields sit 0x10 lower than in the Xbox PDB (`TESForm` is
//! 0x18 on PC), which holds for all of `PlayerCharacter`'s own fields up to
//! the end of the class; the PC build adds one word at +0xE4C. The layout
//! below lists the fields the translations use, with the PDB names at the PC
//! offsets.
//!
//! Several functions of this unit are the `MagicCaster` / `MagicTarget`
//! overrides of the player: the game calls them with the address of that base
//! subobject (`PlayerCharacter + 0x88` / `+ 0x94`) as `this`, so their
//! offsets are relative to it ([`MagicCasterBase`], [`MagicTargetBase`]).
//!
//! Not translated: the compiler's exception-unwinding state writes of the
//! constructor and destructor (SEH frames).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, BSSimpleList, NiPoint3};
use crate::units::fallout_ai::actor::Actor;

/// `operator new` (cdecl, size).
pub(crate) const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (cdecl, block).
pub(crate) const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 to an integer in
/// EAX; the uniform form passes it as a leading `f64` argument.
pub(crate) const FTOL: u32 = 0x00ec_62c0;
/// `_eh_vector_constructor_iterator_` (stdcall: array, element size, count,
/// constructor, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// `_eh_vector_destructor_iterator_` (stdcall: array, element size, count,
/// destructor).
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// `memset` (cdecl: destination, byte, count).
const MEMSET: u32 = 0x0040_3d30;

/// `ProcessLists` instance (the `this` of `0096d470`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `CombatManager` singleton pointer (`CombatManager::CreateCombatGroup`'s `this`).
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// A global pointer whose target is the `this` of `0042ce10`.
const GLOBAL_OBJECT_0042CE10: u32 = 0x011d_df38;
/// `0.0` as a `double` in the exe's constants.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// A `float` constant the constructor stores into the steal-warning timers
/// and other timers.
const TIMER_RESET: u32 = 0x0101_2054;
/// A `float` constant stored into `fBlockActivateTimer`.
const BLOCK_ACTIVATE_TIMER_RESET: u32 = 0x0101_712c;
/// A `float` constant stored into `fEyeHeight`.
const EYE_HEIGHT_DEFAULT: u32 = 0x0108_a8dc;
/// The three words (`NiPoint3`, zero in the exe's data) copied into every
/// vector member the constructor initializes.
const ZERO_VECTOR: u32 = 0x011f_426c;
/// Table the achievement entries read a flag byte from (stride 0x28).
const ACHIEVEMENT_TABLE: u32 = 0x011a_3ad4;

/// The getter (`00403e20`, returns a pointer to its `float` value) of the
/// game settings the constructor reads.
const SETTING_FLOAT_GETTER: u32 = 0x0040_3e20;
/// The setting getter that returns a pointer to a byte value.
const SETTING_BYTE_GETTER: u32 = 0x0040_8d60;
/// The setting getter that returns a pointer to a word value.
const SETTING_INT_GETTER: u32 = 0x0043_d4d0;

/// `BSSoundHandle` constructor (returns the handle), assignment from another
/// handle, destructor, validity test and stop.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
/// `BSSimpleList<T>` constructor, destructor, "clear", and the deleting
/// destructor of a heap list (`this`, flags).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_DELETING_DESTRUCT: u32 = 0x0047_02f0;
/// `BSSimpleList` node accessors: the address of a node's item word; the
/// next node; remove the head node; whether the list is empty.
const LIST_ITEM_ADDRESS: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `NiPointer<T>` constructor (with the pointer), destructor, and assignment.
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;

layout! {
    /// `NiTMapItem<unsigned int, unsigned char>` (Xbox PDB), 0xC bytes.
    pub struct NiTMapItem: 0x0C {
        /// `m_pkNext` (Xbox PDB).
        0x00 m_pkNext: Ptr,
        /// `m_key` (Xbox PDB).
        0x04 m_key: u32,
        /// `m_val` (Xbox PDB).
        0x08 m_val: u8,
    }

    /// `PlayerCharacter` (Xbox PDB), 0xE50 bytes on PC. Only the fields the
    /// translations use are listed; each is at the PDB offset minus 0x10.
    /// Members that are objects embedded by value are `Inline<..>`: their
    /// address is what the code uses.
    pub struct PlayerCharacter: 0xE50 {
        /// `PlayerAchievements` (Xbox PDB): three `AchievementInstance` of
        /// 0xC bytes (a flag byte, a word, a flag byte).
        0x1C8 PlayerAchievements: Inline<()>,
        /// `pQueuedTargetLoc` (Xbox PDB).
        0x1EC pQueuedTargetLoc: Ptr,
        /// `pQueuedWeaponAttach` (Xbox PDB).
        0x1F0 pQueuedWeaponAttach: Ptr,
        /// `fTimeSinceLastAmmoRegenTick` (Xbox PDB).
        0x1F4 fTimeSinceLastAmmoRegenTick: f32,
        /// `cShotsSinceLastAmmoRegen` (Xbox PDB).
        0x1F8 cShotsSinceLastAmmoRegen: u8,
        /// `iSandmanDetection` (Xbox PDB).
        0x1FC iSandmanDetection: u32,
        /// `iCombatPersue` (Xbox PDB).
        0x200 iCombatPersue: u32,
        /// `bTravelUseDoor` (Xbox PDB).
        0x204 bTravelUseDoor: bool,
        /// `bOnElevator` (Xbox PDB).
        0x205 bOnElevator: bool,
        /// `pClosestConversation` (Xbox PDB).
        0x208 pClosestConversation: Ptr,
        /// `btransporting` (Xbox PDB).
        0x20C btransporting: bool,
        /// `pActiveEffectList` (Xbox PDB): `BSSimpleList<ActiveEffect *> *`.
        0x210 pActiveEffectList: Ptr,
        /// `pCurrentSpell` (Xbox PDB).
        0x214 pCurrentSpell: Ptr,
        /// `pDesiredTarget` (Xbox PDB).
        0x218 pDesiredTarget: Ptr,
        /// `pCameraCaster` (Xbox PDB): `CameraCaster *`.
        0x21C pCameraCaster: Ptr,
        /// `pAIConversationRunning` (Xbox PDB).
        0x224 pAIConversationRunning: Ptr,
        /// `iNumberofStealWarnings` (Xbox PDB).
        0x228 iNumberofStealWarnings: u32,
        /// `fStealWarningTimer` (Xbox PDB).
        0x22C fStealWarningTimer: f32,
        /// `iNumberofPickpocketWarnings` (Xbox PDB).
        0x230 iNumberofPickpocketWarnings: u32,
        /// `fPickPocketWarningTimer` (Xbox PDB).
        0x234 fPickPocketWarningTimer: f32,
        /// `EatDrinkItems` (Xbox PDB): `BSSimpleList<MagicItem *> *`.
        0x238 EatDrinkItems: Ptr,
        /// `QueuedWornEnchantments` (Xbox PDB): `BSSimpleList<TESBoundObject *> *`.
        0x23C QueuedWornEnchantments: Ptr,
        /// `bShowQuestItemsInInventory` (Xbox PDB).
        0x240 bShowQuestItemsInInventory: bool,
        /// `TemporaryActorValueModifiers` (Xbox PDB): 77 floats.
        0x244 TemporaryActorValueModifiers: Inline<()>,
        /// `ScriptActorValueModifiers` (Xbox PDB): 77 floats.
        0x378 ScriptActorValueModifiers: Inline<()>,
        /// `fHealthModifier` (Xbox PDB).
        0x4AC fHealthModifier: f32,
        /// `DamageActorValueModifiers` (Xbox PDB): 77 floats.
        0x4B0 DamageActorValueModifiers: Inline<()>,
        /// `listNotes` (Xbox PDB): `BSSimpleList<BGSNote *>`.
        0x5E4 listNotes: Inline<BSSimpleList>,
        /// `pIronsightsDOFInstance` (Xbox PDB).
        0x5EC pIronsightsDOFInstance: Ptr,
        /// `pVatsDOFInstance` (Xbox PDB).
        0x5F0 pVatsDOFInstance: Ptr,
        /// `pVatsDRBInstance` (Xbox PDB).
        0x5F4 pVatsDRBInstance: Ptr,
        /// `bHostileDetection` (Xbox PDB).
        0x5F8 bHostileDetection: bool,
        /// `bIsAcousticSpaceTransition` (Xbox PDB).
        0x5F9 bIsAcousticSpaceTransition: bool,
        /// `pListOfTeammates` (Xbox PDB): `BSSimpleList<Actor *>`.
        0x5FC pListOfTeammates: Inline<BSSimpleList>,
        /// `pLastExtDoorActivated` (Xbox PDB).
        0x604 pLastExtDoorActivated: Ptr,
        /// `bSpeaking` (Xbox PDB).
        0x608 bSpeaking: bool,
        /// `pListofActions` (Xbox PDB): `BSSimpleList<PlayerActionObject *> *`.
        0x60C pListofActions: Ptr,
        /// `pListofCasinoData` (Xbox PDB): `BSSimpleList<CasinoData *> *`.
        0x610 pListofCasinoData: Ptr,
        /// `pInactiveListofCaravanCards` (Xbox PDB).
        0x614 pInactiveListofCaravanCards: Ptr,
        /// `pActiveListofCaravanCards` (Xbox PDB).
        0x618 pActiveListofCaravanCards: Ptr,
        /// `iCaravanCapWinnings` (Xbox PDB).
        0x61C iCaravanCapWinnings: u32,
        /// `iCaravanCapLosses` (Xbox PDB).
        0x620 iCaravanCapLosses: u32,
        /// `iCaravanWinnings` (Xbox PDB).
        0x624 iCaravanWinnings: u32,
        /// `iCaravanLosses` (Xbox PDB).
        0x628 iCaravanLosses: u32,
        /// `iCaravanLargestWinning` (Xbox PDB).
        0x62C iCaravanLargestWinning: u32,
        /// `iCasinoCheatLevel` (Xbox PDB).
        0x630 iCasinoCheatLevel: u32,
        /// `spGrabSpring` (Xbox PDB): an embedded `NiPointer`.
        0x634 spGrabSpring: Inline<()>,
        /// `pGrabbedObject` (Xbox PDB).
        0x638 pGrabbedObject: Ptr,
        /// `eGrabType` (Xbox PDB).
        0x63C eGrabType: u32,
        /// `fGrabObjectWeight` (Xbox PDB).
        0x640 fGrabObjectWeight: f32,
        /// `fGrabDistance` (Xbox PDB).
        0x644 fGrabDistance: f32,
        /// `bsave3rdPerson` (Xbox PDB).
        0x648 bsave3rdPerson: bool,
        /// `b3rdPersonSaved` (Xbox PDB).
        0x649 b3rdPersonSaved: bool,
        /// `b3rdPerson` (Xbox PDB).
        0x64A b3rdPerson: bool,
        /// `bActually3rdPerson` (Xbox PDB).
        0x64B bActually3rdPerson: bool,
        /// `bWant3rdPerson` (Xbox PDB).
        0x64C bWant3rdPerson: bool,
        /// `bTemp3rdPerson` (Xbox PDB).
        0x64D bTemp3rdPerson: bool,
        /// `bTemp3rdPersonSwitchBack` (Xbox PDB).
        0x64E bTemp3rdPersonSwitchBack: bool,
        /// `bTemp1stPerson` (Xbox PDB).
        0x64F bTemp1stPerson: bool,
        /// `bTemp1stPersonSwitchBack` (Xbox PDB).
        0x650 bTemp1stPersonSwitchBack: bool,
        /// `bAlwaysRun` (Xbox PDB).
        0x651 bAlwaysRun: bool,
        /// `bAutoMove` (Xbox PDB).
        0x652 bAutoMove: bool,
        /// `iSleepTime` (Xbox PDB).
        0x654 iSleepTime: u32,
        /// `bIsSleeping` (Xbox PDB).
        0x658 bIsSleeping: bool,
        /// `fsecondRunning` (Xbox PDB).
        0x660 fsecondRunning: f32,
        /// `fsecondSwimming` (Xbox PDB).
        0x664 fsecondSwimming: f32,
        /// `fsecondSneaking` (Xbox PDB).
        0x668 fsecondSneaking: f32,
        /// `bActorinSneakRange` (Xbox PDB).
        0x66C bActorinSneakRange: bool,
        /// `bCanFastTravel` (Xbox PDB).
        0x66D bCanFastTravel: bool,
        /// `bCanWait` (Xbox PDB).
        0x66E bCanWait: bool,
        /// `fWorldFOV` (Xbox PDB).
        0x670 fWorldFOV: f32,
        /// `f1stPersonFOV` (Xbox PDB).
        0x674 f1stPersonFOV: f32,
        /// `f3rdPersonFOV` (Xbox PDB).
        0x678 f3rdPersonFOV: f32,
        /// `iNumberTraining` (Xbox PDB).
        0x67C iNumberTraining: u32,
        /// `ucControlsDisabled` (Xbox PDB).
        0x680 ucControlsDisabled: u8,
        /// `bBlockActivate` (Xbox PDB).
        0x681 bBlockActivate: bool,
        /// `fBlockActivateTimer` (Xbox PDB).
        0x684 fBlockActivateTimer: f32,
        /// `p1stPersonBipedAnim` (Xbox PDB).
        0x68C p1stPersonBipedAnim: Ptr,
        /// `p1stPersonAnimation` (Xbox PDB).
        0x690 p1stPersonAnimation: Ptr,
        /// `sp1stPerson3D` (Xbox PDB): an embedded `NiPointer`.
        0x694 sp1stPerson3D: Inline<()>,
        /// `fEyeHeight` (Xbox PDB).
        0x698 fEyeHeight: f32,
        /// `spInventoryPC` (Xbox PDB): an embedded `NiPointer`.
        0x69C spInventoryPC: Inline<()>,
        /// `pInventoryAnimation` (Xbox PDB).
        0x6A0 pInventoryAnimation: Ptr,
        /// `pInventoryWeaponEffect` (Xbox PDB).
        0x6A4 pInventoryWeaponEffect: Ptr,
        /// `listTopics` (Xbox PDB): `BSSimpleList<TESTopic *>`.
        0x6A8 listTopics: Inline<BSSimpleList>,
        /// `listQuestLog` (Xbox PDB).
        0x6B0 listQuestLog: Inline<BSSimpleList>,
        /// `pActiveQuest` (Xbox PDB).
        0x6B8 pActiveQuest: Ptr,
        /// `listObjectives` (Xbox PDB).
        0x6BC listObjectives: Inline<BSSimpleList>,
        /// `listQuestTargets` (Xbox PDB).
        0x6C4 listQuestTargets: Inline<BSSimpleList>,
        /// `bGreetingPlayer` (Xbox PDB).
        0x6CC bGreetingPlayer: bool,
        /// `ihourstosleep` (Xbox PDB).
        0x6D4 ihourstosleep: u32,
        /// `cMurder` (Xbox PDB).
        0x6D8 cMurder: u8,
        /// `iAmountStolenSold` (Xbox PDB).
        0x6DC iAmountStolenSold: u32,
        /// `fSortActorDistanceTimer` (Xbox PDB).
        0x6E0 fSortActorDistanceTimer: f32,
        /// `fSitHeadingDelta` (Xbox PDB).
        0x6E4 fSitHeadingDelta: f32,
        /// `bBeenAttacked` (Xbox PDB).
        0x6E8 bBeenAttacked: bool,
        /// `pSelectedSpell` (Xbox PDB).
        0x6EC pSelectedSpell: Ptr,
        /// `pSelectedScroll` (Xbox PDB).
        0x6F0 pSelectedScroll: Ptr,
        /// `pPlayerMapMarker` (Xbox PDB).
        0x6F4 pPlayerMapMarker: Ptr,
        /// `PlayerMarkerPath` (Xbox PDB): an embedded `TeleportPath`.
        0x6F8 PlayerMarkerPath: Inline<()>,
        /// `fProjectileReleaseTimer` (Xbox PDB).
        0x730 fProjectileReleaseTimer: f32,
        /// `iNumAdvance` (Xbox PDB).
        0x734 iNumAdvance: u32,
        /// `eskilladvance` (Xbox PDB).
        0x738 eskilladvance: u32,
        /// `pDefaultClass` (Xbox PDB).
        0x73C pDefaultClass: Ptr,
        /// `pClassBasedOn` (Xbox PDB).
        0x740 pClassBasedOn: Ptr,
        /// `pCrimeCounts` (Xbox PDB): five words.
        0x744 pCrimeCounts: Inline<()>,
        /// `pPendingPoison` (Xbox PDB).
        0x758 pPendingPoison: Ptr,
        /// `bChargen` (Xbox PDB).
        0x75C bChargen: bool,
        /// `bAllowEGMCacheClear` (Xbox PDB).
        0x75D bAllowEGMCacheClear: bool,
        /// `bTelekinesisSelected` (Xbox PDB).
        0x75E bTelekinesisSelected: bool,
        /// `pOccupiedRegion` (Xbox PDB).
        0x760 pOccupiedRegion: Ptr,
        /// `m_AllOccupiedRegions` (Xbox PDB): an embedded `TESRegionList`.
        0x764 m_AllOccupiedRegions: Inline<()>,
        /// `CurrentRegionSoundList` (Xbox PDB).
        0x774 CurrentRegionSoundList: Inline<BSSimpleList>,
        /// `StatusSoundHandle` (Xbox PDB): an embedded `BSSoundHandle`.
        0x77C StatusSoundHandle: Inline<()>,
        /// `pInitialStateBuffer` (Xbox PDB).
        0x788 pInitialStateBuffer: Ptr,
        /// `iTotalPlayingTime` (Xbox PDB).
        0x790 iTotalPlayingTime: u32,
        /// `iCharacterSeed` (Xbox PDB).
        0x794 iCharacterSeed: u32,
        /// `bAiControlledToPos` (Xbox PDB).
        0x798 bAiControlledToPos: bool,
        /// `bAiControlledFromPos` (Xbox PDB).
        0x799 bAiControlledFromPos: bool,
        /// `bAiControlledActivate` (Xbox PDB).
        0x79A bAiControlledActivate: bool,
        /// `bAiControlledPackage` (Xbox PDB).
        0x79B bAiControlledPackage: bool,
        /// `bInBorderContainedCell` (Xbox PDB).
        0x79C bInBorderContainedCell: bool,
        /// `bReturnToLastKnownGoodPosition` (Xbox PDB).
        0x79D bReturnToLastKnownGoodPosition: bool,
        /// `LastKnownGoodPosition` (Xbox PDB).
        0x7A0 LastKnownGoodPosition: Inline<NiPoint3>,
        /// `pLastKnownGoodLocation` (Xbox PDB).
        0x7AC pLastKnownGoodLocation: Ptr,
        /// `pBorderRegions` (Xbox PDB).
        0x7B0 pBorderRegions: Ptr,
        /// `pLastKnownMusicType` (Xbox PDB).
        0x7B4 pLastKnownMusicType: Ptr,
        /// `eDifficultyLevel` (Xbox PDB).
        0x7B8 eDifficultyLevel: u32,
        /// `eKillCameraSetting` (Xbox PDB).
        0x7C0 eKillCameraSetting: u32,
        /// `bBeingChased` (Xbox PDB).
        0x7C4 bBeingChased: bool,
        /// `bIsYoung` (Xbox PDB).
        0x7C5 bIsYoung: bool,
        /// `bIsToddler` (Xbox PDB).
        0x7C6 bIsToddler: bool,
        /// `bCanUsePowerArmor` (Xbox PDB).
        0x7C7 bCanUsePowerArmor: bool,
        /// `MapMarkerList` (Xbox PDB).
        0x7C8 MapMarkerList: Inline<BSSimpleList>,
        /// `pMapWorld` (Xbox PDB).
        0x7D0 pMapWorld: Ptr,
        /// `AudioMarkerList` (Xbox PDB).
        0x7D4 AudioMarkerList: Inline<BSSimpleList>,
        /// `pClosestAudioMarkerInfo` (Xbox PDB).
        0x7DC pClosestAudioMarkerInfo: Ptr,
        /// `fUFOCameraHeading` (Xbox PDB).
        0x7E0 fUFOCameraHeading: f32,
        /// `fUFOCameraPitch` (Xbox PDB).
        0x7E4 fUFOCameraPitch: f32,
        /// `UFOCameraPos` (Xbox PDB).
        0x7E8 UFOCameraPos: Inline<NiPoint3>,
        /// `iSelectedSpellCastSoundID` (Xbox PDB).
        0x7F4 iSelectedSpellCastSoundID: u32,
        /// `SelectedSpellCastSound` (Xbox PDB): an embedded `BSSoundHandle`.
        0x7F8 SelectedSpellCastSound: Inline<()>,
        /// `MagicFailureSounds` (Xbox PDB): six `BSSoundHandle` of 0xC bytes.
        0x804 MagicFailureSounds: Inline<()>,
        /// `DroppedRefList` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x84C DroppedRefList: Inline<BSSimpleList>,
        /// `RandomDoorSpaceMap` (Xbox PDB): an embedded `NiTMap`.
        0x854 RandomDoorSpaceMap: Inline<()>,
        /// `sp1stPersonLight` (Xbox PDB): an embedded `NiPointer`.
        0x864 sp1stPersonLight: Inline<()>,
        /// `sp3rdPersonLight` (Xbox PDB): an embedded `NiPointer`.
        0x868 sp3rdPersonLight: Inline<()>,
        /// `fDropAngleMod` (Xbox PDB).
        0x870 fDropAngleMod: f32,
        /// `fLastDropAngleMod` (Xbox PDB).
        0x874 fLastDropAngleMod: f32,
        /// `CharacterProgressionInfo` (Xbox PDB): an embedded `CharacterProgression`.
        0x878 CharacterProgressionInfo: Inline<()>,
        /// `Perks` (Xbox PDB): `BSSimpleList<PerkRankData *>`.
        0x87C Perks: Inline<BSSimpleList>,
        /// `PerkEntryLists` (Xbox PDB): 74 lists of 8 bytes.
        0x884 PerkEntryLists: Inline<BSSimpleList>,
        /// `CompanionPerks` (Xbox PDB).
        0xAD4 CompanionPerks: Inline<BSSimpleList>,
        /// `CompanionPerkEntryLists` (Xbox PDB): 74 lists of 8 bytes.
        0xADC CompanionPerkEntryLists: Inline<BSSimpleList>,
        /// `pAutoAimActor` (Xbox PDB).
        0xD2C pAutoAimActor: Ptr,
        /// `BulletAutoAim` (Xbox PDB).
        0xD30 BulletAutoAim: Inline<NiPoint3>,
        /// `spTargeted3D` (Xbox PDB): an embedded `NiPointer`.
        0xD3C spTargeted3D: Inline<()>,
        /// `bTarget3DDistant` (Xbox PDB).
        0xD40 bTarget3DDistant: bool,
        /// `pPlayersTargetActor` (Xbox PDB).
        0xD44 pPlayersTargetActor: Ptr,
        /// `pListofPercievedActors` (Xbox PDB).
        0xD48 pListofPercievedActors: Ptr,
        /// `fMenuModeButtonTimer` (Xbox PDB).
        0xD4C fMenuModeButtonTimer: f32,
        /// `fAmmoSwapButtonTimer` (Xbox PDB).
        0xD50 fAmmoSwapButtonTimer: f32,
        /// `bMenuModeButtonClicked` (Xbox PDB).
        0xD54 bMenuModeButtonClicked: bool,
        /// `kCamera3rdPersonShoulderOffset` (Xbox PDB).
        0xD58 kCamera3rdPersonShoulderOffset: Inline<NiPoint3>,
        /// `pCombatGroup` (Xbox PDB): `CombatGroup *`.
        0xD64 pCombatGroup: Ptr,
        /// `iTeammateCount` (Xbox PDB).
        0xD68 iTeammateCount: u32,
        /// `fCombatTimer` (Xbox PDB).
        0xD6C fCombatTimer: f32,
        /// `fYieldTimer` (Xbox PDB).
        0xD70 fYieldTimer: f32,
        /// `pWobbleNodes` (Xbox PDB): 0x60 bytes of node pointers.
        0xD74 pWobbleNodes: Inline<()>,
        /// `Cached1stPersonCameraPos` (Xbox PDB).
        0xDD4 Cached1stPersonCameraPos: Inline<NiPoint3>,
        /// `CachedWorldCameraPos` (Xbox PDB).
        0xDE0 CachedWorldCameraPos: Inline<NiPoint3>,
        /// `spCameraRigidBody` (Xbox PDB): an embedded `NiPointer`.
        0xDEC spCameraRigidBody: Inline<()>,
        /// `bPlayerInCombat` (Xbox PDB).
        0xDF0 bPlayerInCombat: bool,
        /// `bAllCombatTargetsSearching` (Xbox PDB).
        0xDF1 bAllCombatTargetsSearching: bool,
        /// `RockItLauncherAmmoList` (Xbox PDB): `BSSimpleArray<ItemChange *, 1024>`.
        0xDF4 RockItLauncherAmmoList: Inline<BSSimpleArray>,
        /// `bNightVisionOn` (Xbox PDB).
        0xE08 bNightVisionOn: bool,
        /// `pReputationUpdate` (Xbox PDB).
        0xE0C pReputationUpdate: Ptr,
        /// `fTimeInSlowMoCam` (Xbox PDB).
        0xE18 fTimeInSlowMoCam: f32,
        /// `fKillCamCooldown` (Xbox PDB).
        0xE1C fKillCamCooldown: f32,
        /// `bIgnoresGTM` (Xbox PDB).
        0xE20 bIgnoresGTM: bool,
        /// `bTurboISM` (Xbox PDB).
        0xE21 bTurboISM: bool,
        /// `fLastHelloTime` (Xbox PDB).
        0xE24 fLastHelloTime: f32,
        /// `fCounterAttackTimer` (Xbox PDB).
        0xE28 fCounterAttackTimer: f32,
        /// `bCounterAttackCamera` (Xbox PDB).
        0xE2C bCounterAttackCamera: bool,
        /// `bHasCateyeActive` (Xbox PDB).
        0xE2D bHasCateyeActive: bool,
        /// `bHasSpotterActive` (Xbox PDB).
        0xE2E bHasSpotterActive: bool,
        /// `fCheckForItems` (Xbox PDB).
        0xE30 fCheckForItems: f32,
        /// `bAlwaysHardcore` (Xbox PDB).
        0xE38 bAlwaysHardcore: bool,
        /// `HotKeyLastAmmo` (Xbox PDB): `BSSimpleArray<TESAmmo *, 1024>`.
        0xE3C HotKeyLastAmmo: Inline<BSSimpleArray>,
        /// PC only (not in the Xbox PDB): set to 0 by the constructor.
        0xE4C pcOnlyWord: u32,
    }

    /// `PlayerCharacter` seen through its `MagicCaster` base subobject
    /// (`PlayerCharacter + 0x88`): the offsets are relative to that
    /// subobject.
    pub struct MagicCasterBase: 0x194 {
        /// `PlayerCharacter::pCurrentSpell` (Xbox PDB), at base-relative +0x18C.
        0x18C pCurrentSpell: u32,
        /// `PlayerCharacter::pDesiredTarget` (Xbox PDB), at base-relative +0x190.
        0x190 pDesiredTarget: u32,
    }

    /// `PlayerCharacter` seen through its `MagicTarget` base subobject
    /// (`PlayerCharacter + 0x94`).
    pub struct MagicTargetBase: 0x180 {
        /// `PlayerCharacter::pActiveEffectList` (Xbox PDB), at base-relative +0x17C.
        0x17C pActiveEffectList: Ptr,
    }
}

/// Offset of the word `00939860` stores into its object (the constructor
/// stores 0x3E8 into the object `009306d0` returns).
const WORD_STORED_BY_00939860: u32 = 0x604;
/// Offset of the float `00939880` clears in its 0x1A0-byte object.
const FLOAT_CLEARED_BY_00939880: u32 = 0x198;
/// Offset in the `Actor` base of `pCurrentProcess` (`BaseProcess *`).
const ACTOR_CURRENT_PROCESS: u32 = 0x68;
/// Offset in the `Actor` base of `pContinuousBeamPersistant`.
const ACTOR_CONTINUOUS_BEAM_PERSISTANT: u32 = 0x1A0;
/// Offset of the `MagicCaster` base within `PlayerCharacter`.
const MAGIC_CASTER_BASE: u32 = 0x88;
/// Offset of the `ActorValueOwner` base within `PlayerCharacter`.
const ACTOR_VALUE_OWNER_BASE: u32 = 0xA4;

/// The address of a member of the player.
fn member<T>(this: Ptr<PlayerCharacter>, field: Field<PlayerCharacter, T>) -> u32 {
    this.addr().wrapping_add(field.off)
}

/// Copies the three words of the zero vector into the vector at `dst`.
fn set_zero_vector(e: &mut Engine, dst: u32) {
    for i in 0..3 {
        let word = e.mem.u32(ZERO_VECTOR + 4 * i);
        e.mem.set_u32(dst + 4 * i, word);
    }
}

// Translated from 004a9950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int, unsigned char>>, unsigned int, unsigned char>::SetValue`
/// (Xbox PDB): stores `key` and `value` into the map item. The map itself
/// (`this`) is not read.
pub fn ni_tmap_base_set_value(
    e: &mut Engine,
    _this: Ptr,
    item: Ptr<NiTMapItem>,
    key: u32,
    value: u8,
) {
    e.set(item, NiTMapItem::m_key, key);
    e.set(item, NiTMapItem::m_val, value);
}

// Translated from 005f73f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names it `PlayerCharacter::GetPermanentActorValue` (Xbox
/// PDB), but the body is shared by seven vtables (linker folding): it calls
/// the object's virtual at `+0x20` with `actor_value`, passes the float result
/// through `00406ce0` and truncates the result to an integer.
pub fn fn_005f73f0(e: &mut Engine, this: Ptr, actor_value: u32) -> i32 {
    let value = e.vcall(this.addr(), 0x20, &args![actor_value]).f32();
    let converted = e.call(0x0040_6ce0, &args![value]).f64();
    e.call(FTOL, &args![converted]).i32()
}

// Translated from 006bf8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ItemChange *, 1024>::Remove` (Xbox PDB): removes the
/// pointer at `index`, shifting the later ones down. With `shrink` set and
/// `006f3170` agreeing, the array is first moved to a buffer of the size
/// `00869600` asks for (the shifted tail copy keeps the size minus one as its
/// count, as the code does).
pub fn bs_simple_array_item_change_remove(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    index: u32,
    shrink: bool,
) {
    if shrink && e.call(0x006f_3170, &args![this]).bool() {
        let new_reserved = e.call(0x0086_9600, &args![this]).u32();
        let new_buffer = e.vcall(this.addr(), 4, &args![new_reserved]).u32();
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(0x0042_fb60, &args![this, new_buffer, buffer, index]);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            0x0072_ba80,
            &args![this, buffer.wrapping_add(index.wrapping_mul(4)), 1u32],
        );
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            0x0042_fb60,
            &args![
                this,
                new_buffer.wrapping_add(index.wrapping_mul(4)),
                buffer.wrapping_add(4).wrapping_add(index.wrapping_mul(4)),
                size.wrapping_sub(1)
            ],
        );
        e.call(0x006a_8500, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, new_buffer);
        e.set(this, BSSimpleArray::iReservedSize, new_reserved);
    } else {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            0x0072_ba80,
            &args![this, buffer.wrapping_add(index.wrapping_mul(4)), 1u32],
        );
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            0x0042_fb60,
            &args![
                this,
                buffer.wrapping_add(index.wrapping_mul(4)),
                buffer.wrapping_add(4).wrapping_add(index.wrapping_mul(4)),
                size.wrapping_sub(index).wrapping_sub(1)
            ],
        );
    }
    let size = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, size.wrapping_sub(1));
}

// Translated from 00815340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names it `PlayerCharacter::StartAim` (Xbox PDB); the body
/// only calls [`fn_00815360`].
pub fn fn_00815340(e: &mut Engine, this: Ptr) {
    fn_00815360(e, this);
}

// Translated from 00815360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the object's virtual at `+0x20` (no arguments).
pub fn fn_00815360(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x20, &args![]);
}

// Translated from 00939680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster` base override: calls the object's virtual at `+0x20` (no
/// arguments).
pub fn fn_00939680(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x20, &args![]);
}

// Translated from 009396a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter` virtual `+0x42C`: returns `pPlayersTargetActor`.
pub fn fn_009396a0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> Ptr {
    e.get(this, PlayerCharacter::pPlayersTargetActor)
}

// Translated from 009396c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter` virtual `+0x4E4`: takes seven words and returns false.
#[allow(clippy::too_many_arguments)]
pub fn fn_009396c0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
) -> bool {
    false
}

// Translated from 009396d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter` virtual `+0x3F8`: returns `pCombatGroup`.
pub fn fn_009396d0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> Ptr {
    e.get(this, PlayerCharacter::pCombatGroup)
}

// Translated from 009396f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter` virtual `+0x3FC`: stores `pCombatGroup`.
pub fn fn_009396f0(e: &mut Engine, this: Ptr<PlayerCharacter>, group: Ptr) {
    e.set(this, PlayerCharacter::pCombatGroup, group);
}

// Translated from 00939710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicTarget` base override (`this` is the player plus 0x94): returns the
/// player's `pActiveEffectList`.
pub fn fn_00939710(e: &mut Engine, this: Ptr<MagicTargetBase>) -> Ptr {
    e.get(this, MagicTargetBase::pActiveEffectList)
}

// Translated from 00939730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter` virtual `+0x328`: stores `iNumAdvance`.
pub fn fn_00939730(e: &mut Engine, this: Ptr<PlayerCharacter>, value: u32) {
    e.set(this, PlayerCharacter::iNumAdvance, value);
}

// Translated from 00939750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter` virtual `+0x32C`: returns `iNumAdvance`.
pub fn fn_00939750(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    e.get(this, PlayerCharacter::iNumAdvance)
}

// Translated from 00939770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter` virtual `+0x330`: stores `eskilladvance`.
pub fn fn_00939770(e: &mut Engine, this: Ptr<PlayerCharacter>, value: u32) {
    e.set(this, PlayerCharacter::eskilladvance, value);
}

// Translated from 00939790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter` virtual `+0x334`: returns `eskilladvance`.
pub fn fn_00939790(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    e.get(this, PlayerCharacter::eskilladvance)
}

// Translated from 009397b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter` virtual `+0x4E8`: the address of the companion perk list
/// (`CompanionPerks`) when `companion` is non-zero, else of the player's own
/// (`Perks`).
pub fn fn_009397b0(e: &mut Engine, this: Ptr<PlayerCharacter>, companion: u8) -> Ptr {
    let _ = e;
    if companion == 0 {
        Ptr::new(member(this, PlayerCharacter::Perks))
    } else {
        Ptr::new(member(this, PlayerCharacter::CompanionPerks))
    }
}

// Translated from 009397e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter` virtual `+0x4B4`: calls `005a03f0` on the player with 1 and answers its
/// `AL` (the exe ends on that call). The `Actor` slot is `Actor::IsImmobile` (`bool`; Xbox PDB
/// slot `+0x4B0`, aligned by the named slots around it).
pub fn fn_009397e0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x005a_03f0, &args![this, 1u32]).bool()
}

// Translated from 00939800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster` base override (`this` is the player plus 0x88): stores the
/// player's `pCurrentSpell`.
pub fn fn_00939800(e: &mut Engine, this: Ptr<MagicCasterBase>, spell: u32) {
    e.set(this, MagicCasterBase::pCurrentSpell, spell);
}

// Translated from 00939820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster` base override: returns the player's `pDesiredTarget`.
pub fn fn_00939820(e: &mut Engine, this: Ptr<MagicCasterBase>) -> u32 {
    e.get(this, MagicCasterBase::pDesiredTarget)
}

// Translated from 00939840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicCaster` base override: stores the player's `pDesiredTarget`.
pub fn fn_00939840(e: &mut Engine, this: Ptr<MagicCasterBase>, target: u32) {
    e.set(this, MagicCasterBase::pDesiredTarget, target);
}

// Translated from 00939860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` into the word at `+0x604` of its object (the constructor
/// calls it with 0x3E8 on the object `009306d0` returns).
pub fn fn_00939860(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + WORD_STORED_BY_00939860, value);
}

// Translated from 00939880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializer of a 0x1A0-byte object (the constructor allocates two): clears
/// the float at `+0x198` and returns the object.
pub fn fn_00939880(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_f32(this.addr() + FLOAT_CLEARED_BY_00939880, 0.0);
    this
}

// Translated from 009398a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter` vector deleting destructor (`__vecDelDtor`, vtable slot
/// `0x10`): runs the destructor body [`fn_009398d0`], then frees the object
/// when bit 0 of `flags` is set.
pub fn player_character_vector_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    flags: u32,
) -> Ptr<PlayerCharacter> {
    fn_009398d0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0093a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CameraCaster` vector deleting destructor: runs `CameraCaster::~CameraCaster`
/// (Xbox PDB, `006208d0`), then frees the object when bit 0 of `flags` is set.
pub fn camera_caster_vector_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0062_08d0, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0093a5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `bAiControlledToPos`.
pub fn fn_0093a5d0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    e.get(this, PlayerCharacter::bAiControlledToPos)
}

// Translated from 0093a5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `bAiControlledToPos`, then calls the virtual at `+0x258` with 0 when
/// any of the four AI-control flags is set ([`fn_0093a740`]) and with 1 when
/// none is.
pub fn fn_0093a5f0(e: &mut Engine, this: Ptr<PlayerCharacter>, value: bool) {
    e.set(this, PlayerCharacter::bAiControlledToPos, value);
    let controlled = fn_0093a740(e, this);
    e.vcall(this.addr(), 0x258, &args![!controlled as u32]);
}

// Translated from 0093a640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `bAiControlledFromPos`.
pub fn fn_0093a640(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    e.get(this, PlayerCharacter::bAiControlledFromPos)
}

// Translated from 0093a660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::RemoveActorFromPlayercombatList` (Xbox PDB): when the
/// player has a combat group, removes `actor` from it (`00986500`).
pub fn player_character_remove_actor_from_playercombat_list(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor: Ptr,
) {
    let group = e.get(this, PlayerCharacter::pCombatGroup);
    if !group.is_null() {
        e.call(0x0098_6500, &args![group, actor]);
    }
}

// Translated from 0093a690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::AddActorToPlayerCombatList` (Xbox PDB): creates the
/// player's combat group on first use (`CombatManager::CreateCombatGroup`,
/// `00991e80`, and adds the player to it with `009867d0`), then adds `actor` as
/// a target (`00986410`).
pub fn player_character_add_actor_to_player_combat_list(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor: Ptr,
) {
    if e.get(this, PlayerCharacter::pCombatGroup).is_null() {
        let manager = e.global::<u32>(COMBAT_MANAGER);
        let group: Ptr = e.call(0x0099_1e80, &args![manager]).ptr();
        e.set(this, PlayerCharacter::pCombatGroup, group);
        let group = e.get(this, PlayerCharacter::pCombatGroup);
        e.call(0x0098_67d0, &args![group, this]);
    }
    let group = e.get(this, PlayerCharacter::pCombatGroup);
    e.call(0x0098_6410, &args![group, actor]);
}

// Translated from 0093a6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `bAiControlledFromPos`, then calls the virtual at `+0x258` as
/// [`fn_0093a5f0`] does.
pub fn fn_0093a6f0(e: &mut Engine, this: Ptr<PlayerCharacter>, value: bool) {
    e.set(this, PlayerCharacter::bAiControlledFromPos, value);
    let controlled = fn_0093a740(e, this);
    e.vcall(this.addr(), 0x258, &args![!controlled as u32]);
}

// Translated from 0093a740 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when any of `bAiControlledToPos`, `bAiControlledFromPos`,
/// `bAiControlledActivate` and `bAiControlledPackage` is set.
pub fn fn_0093a740(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    e.get(this, PlayerCharacter::bAiControlledFromPos)
        || e.get(this, PlayerCharacter::bAiControlledToPos)
        || e.get(this, PlayerCharacter::bAiControlledActivate)
        || e.get(this, PlayerCharacter::bAiControlledPackage)
}

// Translated from 0093a7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `pOccupiedRegion`.
pub fn fn_0093a7a0(e: &mut Engine, this: Ptr<PlayerCharacter>, region: Ptr) {
    e.set(this, PlayerCharacter::pOccupiedRegion, region);
}

/// The common tail of the actor-value setters: tells the interface the value
/// changed (`Interface::UpdateActorValue`, `00704e10`, cdecl) and calls
/// `008808f0` on the player.
fn notify_actor_value_changed(e: &mut Engine, this: Ptr<PlayerCharacter>, actor_value: u32) {
    e.call(0x0070_4e10, &args![actor_value]);
    e.call(0x0088_08f0, &args![this, actor_value, 1u32]);
}

/// Asks the player's virtual at `+0x48C` whether `actor_value` has an
/// override (the byte it writes through its second argument).
fn actor_value_has_override(e: &mut Engine, this: Ptr<PlayerCharacter>, actor_value: u32) -> bool {
    e.with_stack(4, |e, flag| {
        e.vcall(this.addr(), 0x48c, &args![actor_value, flag]);
        e.mem.u8(flag.addr()) != 0
    })
}

// Translated from 0093a7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetActorBaseValue` (Xbox PDB): sets the base value of
/// `actor_value` to the integer `value`. When the virtual at `+0x48C` reports
/// an override, the override (virtual `+0x490`, as a float) is set; otherwise
/// the object `004181e0` returns gets its virtual `+0x194` called. The
/// interface is then notified.
pub fn player_character_set_actor_base_value(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: u32,
    value: i32,
) {
    if !actor_value_has_override(e, this, actor_value) {
        let target: Ptr = e.call(0x0041_81e0, &args![this]).ptr();
        e.vcall(target.addr(), 0x194, &args![actor_value, value]);
    } else {
        e.vcall(
            this.addr(),
            0x490,
            &args![actor_value, (value as f64) as f32],
        );
    }
    notify_actor_value_changed(e, this, actor_value);
}

// Translated from 0093a850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`player_character_set_actor_base_value`] with a float value: the
/// object `004181e0` returns gets its virtual `+0x190` called when there is no
/// override.
pub fn fn_0093a850(e: &mut Engine, this: Ptr<PlayerCharacter>, actor_value: u32, value: f32) {
    if !actor_value_has_override(e, this, actor_value) {
        let target: Ptr = e.call(0x0041_81e0, &args![this]).ptr();
        e.vcall(target.addr(), 0x190, &args![actor_value, value]);
    } else {
        e.vcall(this.addr(), 0x490, &args![actor_value, value]);
    }
    notify_actor_value_changed(e, this, actor_value);
}

/// The value of `actor_value` the `ActorValueOwner` base reports (its virtual
/// `+0xC`), or 0 when `0066ee10` says the value does not apply.
fn owner_actor_value_or_zero(e: &mut Engine, this: Ptr<PlayerCharacter>, actor_value: u32) -> f32 {
    if e.call(0x0066_ee10, &args![actor_value]).bool() {
        e.vcall(
            this.addr() + ACTOR_VALUE_OWNER_BASE,
            0xc,
            &args![actor_value],
        )
        .f32()
    } else {
        0.0
    }
}

/// Reports a change of `actor_value` from `old` by `delta` (`0066ee50`,
/// cdecl) to the `ActorValueOwner` base.
fn report_actor_value_change(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: u32,
    old: f32,
    delta: f32,
) {
    let owner = if this.is_null() {
        0
    } else {
        this.addr() + ACTOR_VALUE_OWNER_BASE
    };
    e.call(0x0066_ee50, &args![owner, actor_value, old, delta, 0u32]);
}

// Translated from 0093a8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the integer `delta` to the base value of `actor_value`: the sum with
/// the current value goes to the override (virtual `+0x490`) when the virtual
/// `+0x48C` reports one, otherwise `delta` goes to virtual `+0x19C` of the
/// object `004181e0` returns. The interface and the `ActorValueOwner` are told.
pub fn fn_0093a8f0(e: &mut Engine, this: Ptr<PlayerCharacter>, actor_value: u32, delta: i32) {
    let old = owner_actor_value_or_zero(e, this, actor_value);
    if !actor_value_has_override(e, this, actor_value) {
        let target: Ptr = e.call(0x0041_81e0, &args![this]).ptr();
        e.vcall(target.addr(), 0x19c, &args![actor_value, delta]);
    } else {
        let sum = (delta as f64 + old as f64) as f32;
        e.vcall(this.addr(), 0x490, &args![actor_value, sum]);
    }
    notify_actor_value_changed(e, this, actor_value);
    report_actor_value_change(e, this, actor_value, old, (delta as f64) as f32);
}

// Translated from 0093aa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_0093a8f0`] with a float delta (virtual `+0x198` of the object
/// `004181e0` returns when there is no override).
pub fn fn_0093aa10(e: &mut Engine, this: Ptr<PlayerCharacter>, actor_value: u32, delta: f32) {
    let old = owner_actor_value_or_zero(e, this, actor_value);
    if !actor_value_has_override(e, this, actor_value) {
        let target: Ptr = e.call(0x0041_81e0, &args![this]).ptr();
        e.vcall(target.addr(), 0x198, &args![actor_value, delta]);
    } else {
        let sum = (old as f64 + delta as f64) as f32;
        e.vcall(this.addr(), 0x490, &args![actor_value, sum]);
    }
    notify_actor_value_changed(e, this, actor_value);
    report_actor_value_change(e, this, actor_value, old, delta);
}

// Translated from 0093ab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes the action-point cost of `actor_value` (`0066dca0`, with the flag
/// `00525430` reads from the object at `011f2250`) and passes it to the
/// virtual at `+0x33C` (which spends the action points).
pub fn fn_0093ab30(e: &mut Engine, this: Ptr<PlayerCharacter>, actor_value: u32) {
    // 0093ab39 pushes 0 and 0.0 before the thiscall 00525430 (plain RET); they
    // are the last two of the four words the cdecl 0066dca0 is given (ADD
    // ESP,0x10 after it).
    let flag = e.call(0x0052_5430, &args![0x011f_2250u32]).bool();
    let cost = e
        .call(0x0066_dca0, &args![actor_value, flag, 0.0f32, 0u32])
        .f32();
    e.vcall(this.addr(), 0x33c, &args![cost]);
}

// Translated from 0093ab80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Spends `amount` action points: unless the player is in god mode
/// (`009526b0`) or `0042ce10` (on the object at global `011ddf38`) says
/// otherwise, and `amount` is positive, calls virtual `+0x3AC` with
/// (12, `-amount`, 0).
pub fn fn_0093ab80(e: &mut Engine, this: Ptr<PlayerCharacter>, amount: f32) {
    if e.call(0x0095_26b0, &args![this]).bool() {
        return;
    }
    let object = e.global::<u32>(GLOBAL_OBJECT_0042CE10);
    if e.call(0x0042_ce10, &args![object]).bool() {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    if amount as f64 > zero {
        e.vcall(this.addr(), 0x3ac, &args![0xcu32, -amount, 0u32]);
    }
}

/// `PlayerCharacter` global singleton pointer (`011dea3c`).
const PLAYER_SINGLETON: u32 = 0x011d_ea3c;
/// `TES` global singleton pointer (`011dea10`): the `this` of the cell-grid
/// functions.
const TES_SINGLETON: u32 = 0x011d_ea10;
/// The limit (a `double`) above which the water-cell search radius is
/// clamped, and the radius it is clamped to (a `float`).
const WATER_RADIUS_LIMIT: u32 = 0x0101_6968;
const WATER_RADIUS_CLAMPED: u32 = 0x0101_8bfc;
/// The cell tests: `00425fd0` is bit 0 of the cell flag byte at +0x24 (what
/// `TESObjectREFR::GetInterior` returns for the parent cell, so the
/// interior-cell test), `004518e0` bit 1 (value 2) of the same byte.
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
const CELL_FLAG_2_TEST: u32 = 0x0045_18e0;
/// The width of a grid cell as a `double` (4096.0).
const CELL_SIZE: u32 = 0x0101_7a10;
/// The setting objects (read through `00403e20`) the heartbeat sound of the
/// health meter compares the health fraction with.
const HEARTBEAT_SETTING_LOW: u32 = 0x011d_00b4;
const HEARTBEAT_SETTING_HIGH: u32 = 0x011d_04a8;
/// `BSAudio::QInstance` (`00453a70`) returns the audio singleton; its
/// `GetSoundHandleByName` (`00ad7550`) fills a handle from a sound name.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_FADE_OUT_AND_RELEASE: u32 = 0x00ad_8da0;
/// The sound names the health heartbeat switches between.
const HEARTBEAT_SOUND_ALP: u32 = 0x0108_af40;
const HEARTBEAT_SOUND_BLP: u32 = 0x0108_af28;

// Translated from 0093abe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes two floats through the pointers: `out_second` gets the result of
/// `008a0c20` on the player and `out_first` gets the `ActorValueOwner` base's
/// virtual `+0x8` for actor value `0x2E`, converted from an integer.
pub fn fn_0093abe0(e: &mut Engine, this: Ptr<PlayerCharacter>, out_first: Ptr, out_second: Ptr) {
    let second = e.call(0x008a_0c20, &args![this]).f32();
    e.mem.set_f32(out_second.addr(), second);
    let first = e
        .vcall(this.addr() + ACTOR_VALUE_OWNER_BASE, 8, &args![0x2eu32])
        .i32();
    e.mem.set_f32(out_first.addr(), first as f32);
}

// Translated from 0093ac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValueOwner` base override (`this` is the player + `0xA4`): when
/// `004181e0` finds the object for the player, returns the base's virtual
/// `+0x0` for `actor_value` plus the three values `0094c3d0` (kinds 0, 1 and
/// 2) gives for it, each truncated to an integer; otherwise 0.
pub fn fn_0093ac20(e: &mut Engine, this: Ptr, actor_value: u32) -> i32 {
    let player = this.addr().wrapping_sub(ACTOR_VALUE_OWNER_BASE);
    if e.call(0x0041_81e0, &args![player]).u32() == 0 {
        return 0;
    }
    let mut sum = e.vcall(this.addr(), 0, &args![actor_value]).i32();
    for kind in 0..3u32 {
        let part = e.call(0x0094_c3d0, &args![player, kind, actor_value]).f64();
        sum = sum.wrapping_add(e.call(FTOL, &args![part]).i32());
    }
    sum
}

// Translated from 0093acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Float version of [`fn_0093ac20`]: the base's virtual `+0x4` plus the three
/// values `0094c3d0` gives, summed in that order.
pub fn fn_0093acb0(e: &mut Engine, this: Ptr, actor_value: u32) -> f32 {
    let player = this.addr().wrapping_sub(ACTOR_VALUE_OWNER_BASE);
    let mut sum = e.vcall(this.addr(), 4, &args![actor_value]).f64();
    for kind in 0..3u32 {
        let part = e.call(0x0094_c3d0, &args![player, kind, actor_value]).f64();
        sum += part;
    }
    sum as f32
}

// Translated from 0093ad30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the object's virtual at `+0x20` with `actor_value`, passes the float
/// through `00406ce0` and truncates the result to an integer (the same body as
/// [`fn_005f73f0`]).
pub fn fn_0093ad30(e: &mut Engine, this: Ptr, actor_value: u32) -> i32 {
    let value = e.vcall(this.addr(), 0x20, &args![actor_value]).f32();
    let converted = e.call(0x0040_6ce0, &args![value]).f64();
    e.call(FTOL, &args![converted]).i32()
}

// Translated from 0093ad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetPermanentActorFloatValue` (Xbox PDB): `this` is the
/// `ActorValueOwner` base (player + `0xA4`). The permanent value of
/// `actor_value`, clamped by `ActorValue::ClampActorValue` (`0066f190`),
/// whose result is returned. For a skill (`0047f060`) it is the base's
/// virtual `+0x18` plus the player's virtual `+0x48C` value plus the derived
/// skill of the player singleton's `ActorValueOwner` (`AiFormulas::AVDeriveSkill`);
/// otherwise the base's virtual `+0x18` plus its virtual `+0x4`.
pub fn player_character_get_permanent_actor_float_value(
    e: &mut Engine,
    this: Ptr,
    actor_value: u32,
) -> f32 {
    let value = if e.call(0x0047_f060, &args![actor_value]).bool() {
        let singleton = e.global::<u32>(PLAYER_SINGLETON);
        let derive_this = if singleton == 0 {
            0
        } else {
            singleton.wrapping_add(ACTOR_VALUE_OWNER_BASE)
        };
        let skill = e
            .call(0x0064_3c90, &args![derive_this, actor_value, 1u32])
            .f64();
        let player = this.addr().wrapping_sub(ACTOR_VALUE_OWNER_BASE);
        let modifier = e.with_stack(4, |e, flag| {
            e.vcall(player, 0x48c, &args![actor_value, flag]).f64()
        });
        let sum = modifier + skill;
        let base = e.vcall(this.addr(), 0x18, &args![actor_value]).f64();
        (base + sum) as f32
    } else {
        let first = e.vcall(this.addr(), 4, &args![actor_value]).f64();
        let base = e.vcall(this.addr(), 0x18, &args![actor_value]).f64();
        (base + first) as f32
    };
    e.call(0x0066_f190, &args![actor_value, value]).f32()
}

// Translated from 0093ae40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes `actor_value` and `value` to `00880700`, then does what the other
/// actor-value setters do afterwards (interface update, `008808f0`).
pub fn fn_0093ae40(e: &mut Engine, this: Ptr<PlayerCharacter>, actor_value: u32, value: f32) {
    e.call(0x0088_0700, &args![this, actor_value, value]);
    notify_actor_value_changed(e, this, actor_value);
}

/// Reports a change of `actor_value` from `old` by `delta` to the
/// `ActorValueOwner` bases of the player and of `source` (`0066ee50`).
fn report_change_from(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: u32,
    old: f32,
    delta: f32,
    source: Ptr,
) {
    let source_owner = if source.is_null() {
        0
    } else {
        source.addr().wrapping_add(ACTOR_VALUE_OWNER_BASE)
    };
    let owner = if this.is_null() {
        0
    } else {
        this.addr().wrapping_add(ACTOR_VALUE_OWNER_BASE)
    };
    e.call(
        0x0066_ee50,
        &args![owner, actor_value, old, delta, source_owner],
    );
}

/// The common tail of the four damage-style setters `0093ae80`, `0093afb0`,
/// `0093b0f0` and `0093b240`: reads the old value, records the change in
/// `0094c4f0` (kind `kind`, the last argument 2), updates the interface, for
/// health lost calls the player's virtual `+0x4B8` with `source`, and
/// reports the change.
fn apply_actor_value_change(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: u32,
    delta: f32,
    lost: bool,
    kind: u32,
    source: Ptr,
) {
    let old = owner_actor_value_or_zero(e, this, actor_value);
    e.call(0x0094_c4f0, &args![this, kind, actor_value, delta, 2u32]);
    e.call(0x0070_4e10, &args![actor_value]);
    if actor_value == 0x10 && lost {
        e.vcall(this.addr(), 0x4b8, &args![source, delta]);
    }
    e.call(0x0088_08f0, &args![this, actor_value, 0u32]);
    report_change_from(e, this, actor_value, old, delta, source);
}

// Translated from 0093ae80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unless `00406d70` says `actor_value` has flag `0x100`, applies a change of
/// the actor value: `00880850` gives the integer delta (see
/// [`apply_actor_value_change`], kind 0).
pub fn fn_0093ae80(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: u32,
    amount: i32,
    source: Ptr,
) {
    if e.call(0x0040_6d70, &args![actor_value, 0x100u32]).bool() {
        return;
    }
    let delta = e
        .call(0x0088_0850, &args![this, actor_value, amount, source])
        .i32();
    apply_actor_value_change(e, this, actor_value, delta as f32, delta < 0, 0, source);
}

// Translated from 0093afb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Float version of [`fn_0093ae80`] (`00880890` gives the delta).
pub fn fn_0093afb0(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: u32,
    amount: f32,
    source: Ptr,
) {
    if e.call(0x0040_6d70, &args![actor_value, 0x100u32]).bool() {
        return;
    }
    let delta = e
        .call(0x0088_0890, &args![this, actor_value, amount, source])
        .f32();
    let zero: f64 = e.global(ZERO_DOUBLE);
    apply_actor_value_change(
        e,
        this,
        actor_value,
        delta,
        (delta as f64) < zero,
        0,
        source,
    );
}

// Translated from 0093b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_0093ae80`] preceded by the check `00952730` (called with the amount
/// as a float) and with kind 1.
pub fn fn_0093b0f0(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: u32,
    amount: i32,
    source: Ptr,
) {
    if !e
        .call(0x0095_2730, &args![this, actor_value, amount as f32])
        .bool()
    {
        return;
    }
    if e.call(0x0040_6d70, &args![actor_value, 0x100u32]).bool() {
        return;
    }
    let delta = e
        .call(0x0088_0850, &args![this, actor_value, amount, source])
        .i32();
    apply_actor_value_change(e, this, actor_value, delta as f32, delta < 0, 1, source);
}

// Translated from 0093b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Float version of [`fn_0093b0f0`].
pub fn fn_0093b240(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: u32,
    amount: f32,
    source: Ptr,
) {
    if !e
        .call(0x0095_2730, &args![this, actor_value, amount])
        .bool()
    {
        return;
    }
    if e.call(0x0040_6d70, &args![actor_value, 0x100u32]).bool() {
        return;
    }
    let delta = e
        .call(0x0088_0890, &args![this, actor_value, amount, source])
        .f32();
    let zero: f64 = e.global(ZERO_DOUBLE);
    apply_actor_value_change(
        e,
        this,
        actor_value,
        delta,
        (delta as f64) < zero,
        1,
        source,
    );
}

/// The health of the player as a fraction of its maximum: the
/// `ActorValueOwner` base's virtual `+0xC` over its virtual `+0x20`, both for
/// actor value `0x10`, divided in double precision.
fn health_ratio(e: &mut Engine, this: Ptr<PlayerCharacter>) -> f64 {
    let owner = this.addr().wrapping_add(ACTOR_VALUE_OWNER_BASE);
    let current = e.vcall(owner, 0xc, &args![0x10u32]).f32() as f64;
    let maximum = e.vcall(owner, 0x20, &args![0x10u32]).f32() as f64;
    current / maximum
}

/// The float the settings getter returns for the setting object `setting`.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_FLOAT_GETTER, &args![setting]).u32();
    e.mem.f32(value)
}

/// Whether the x87 comparison of `a` with `b` is unordered (a NaN).
fn unordered(a: f32, b: f32) -> bool {
    a.is_nan() || b.is_nan()
}

/// `!(a < b)`: false only when `a` is below `b` (true for NaNs).
fn not_below(a: f32, b: f32) -> bool {
    a.partial_cmp(&b) != Some(std::cmp::Ordering::Less)
}

/// Stops the status sound and starts the sound called `name` in its place.
fn play_status_sound(e: &mut Engine, this: Ptr<PlayerCharacter>, name: u32) {
    let handle = member(this, PlayerCharacter::StatusSoundHandle);
    e.call(SOUND_HANDLE_STOP, &args![handle]);
    e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    e.with_stack(12, |e, found| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let result = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_NAME,
                &args![audio, found, name, 0x31u32],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, result]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![found]);
    });
    e.call(SOUND_HANDLE_PLAY, &args![handle, 1u32]);
}

/// The heartbeat sound of the health meter: given the health fraction before
/// (`before`) and after (`after`) a change, fades the status sound out, or
/// switches it to the "ALP" or "BLP" heartbeat according to the two health
/// thresholds of the settings at `011d00b4` and `011d04a8`. The branches
/// follow the x87 comparisons of the code, NaN included (one test pair can
/// never fall through for ordered values; it is still written out).
fn update_health_heartbeat(e: &mut Engine, this: Ptr<PlayerCharacter>, before: f32, after: f32) {
    let zero: f64 = e.global(ZERO_DOUBLE);
    let handle = member(this, PlayerCharacter::StatusSoundHandle);
    if (after as f64) <= zero || setting_float(e, HEARTBEAT_SETTING_LOW) < after {
        e.call(SOUND_HANDLE_FADE_OUT_AND_RELEASE, &args![handle, 1000u32]);
        return;
    }
    let low = setting_float(e, HEARTBEAT_SETTING_LOW);
    let mut check_blp = low < after || unordered(low, after);
    if !check_blp {
        let high = setting_float(e, HEARTBEAT_SETTING_HIGH);
        if not_below(high, after) {
            check_blp = true;
        } else {
            let high = setting_float(e, HEARTBEAT_SETTING_HIGH);
            let alp = if not_below(high, before) && !unordered(high, before) {
                true
            } else {
                let low = setting_float(e, HEARTBEAT_SETTING_LOW);
                if low < before {
                    true
                } else {
                    check_blp = true;
                    false
                }
            };
            if alp {
                play_status_sound(e, this, HEARTBEAT_SOUND_ALP);
                return;
            }
        }
    }
    if check_blp {
        let high = setting_float(e, HEARTBEAT_SETTING_HIGH);
        if high < before {
            let high = setting_float(e, HEARTBEAT_SETTING_HIGH);
            if !(high < after || unordered(high, after)) {
                play_status_sound(e, this, HEARTBEAT_SOUND_BLP);
            }
        }
    }
}

/// The part shared by `0093b3a0` and `0093b7a0`: `before` is the health
/// fraction before the change (1.0 for other actor values).
fn apply_health_change(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: u32,
    before: f32,
    delta: f32,
    lost: bool,
    source: Ptr,
) {
    let old = owner_actor_value_or_zero(e, this, actor_value);
    let flagged = e.call(0x0040_6d70, &args![actor_value, 0x200u32]).bool() as u32;
    e.call(0x0094_c4f0, &args![this, 2u32, actor_value, delta, flagged]);
    e.call(0x0070_4e10, &args![actor_value]);
    if actor_value == 0x10 {
        let after = health_ratio(e, this) as f32;
        if lost {
            e.vcall(this.addr(), 0x4b8, &args![source, delta]);
        }
        update_health_heartbeat(e, this, before, after);
    }
    e.call(0x0088_08f0, &args![this, actor_value, 0u32]);
    report_change_from(e, this, actor_value, old, delta, source);
}

// Translated from 0093b3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_0093b0f0`] with kind 2: when the actor value is health (`0x10`)
/// the heartbeat sound follows the change ([`update_health_heartbeat`]), and
/// the change is recorded with the flag `00406d70` reports for `0x200`. The
/// SEH frame (for the sound-handle temporaries) is not translated.
pub fn fn_0093b3a0(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: u32,
    amount: i32,
    source: Ptr,
) {
    if !e
        .call(0x0095_2730, &args![this, actor_value, amount as f32])
        .bool()
    {
        return;
    }
    if e.call(0x0040_6d70, &args![actor_value, 0x100u32]).bool() {
        return;
    }
    let before = if actor_value == 0x10 {
        health_ratio(e, this) as f32
    } else {
        1.0
    };
    let delta = e
        .call(0x0088_0850, &args![this, actor_value, amount, source])
        .i32();
    apply_health_change(
        e,
        this,
        actor_value,
        before,
        delta as f32,
        delta < 0,
        source,
    );
}

// Translated from 0093b7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Float version of [`fn_0093b3a0`] (`00880890` gives the delta).
pub fn fn_0093b7a0(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: u32,
    amount: f32,
    source: Ptr,
) {
    if !e
        .call(0x0095_2730, &args![this, actor_value, amount])
        .bool()
    {
        return;
    }
    if e.call(0x0040_6d70, &args![actor_value, 0x100u32]).bool() {
        return;
    }
    let before = if actor_value == 0x10 {
        health_ratio(e, this) as f32
    } else {
        1.0
    };
    let delta = e
        .call(0x0088_0890, &args![this, actor_value, amount, source])
        .f32();
    let zero: f64 = e.global(ZERO_DOUBLE);
    apply_health_change(
        e,
        this,
        actor_value,
        before,
        delta,
        (delta as f64) < zero,
        source,
    );
}

/// `TESDataHandler` singleton pointer (`011c3f2c`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `ExteriorCellLoader` singleton pointer (`011c9618`).
const EXTERIOR_CELL_LOADER: u32 = 0x011c_9618;
/// The `Main` singleton pointer (`011dea0c`).
const MAIN_SINGLETON: u32 = 0x011d_ea0c;
/// The `MagicTarget` base of the player: its offset in `PlayerCharacter`.
const MAGIC_TARGET_BASE: u32 = 0x94;
/// A pointer-sized global that `0093ccd0` returns.
const GLOBAL_RETURNED_BY_0093CCD0: u32 = 0x011c_358c;
/// The 3x3 identity matrix (nine words) the player's matrix starts from.
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// The constant vector (three words) passed as the second vector to the
/// listener orientation.
const LISTENER_UP_VECTOR: u32 = 0x011a_9484;
/// pi, 2 pi and -pi as `double`s.
const PI_DOUBLE: u32 = 0x0101_ff40;
const TWO_PI_DOUBLE: u32 = 0x0101_ff48;
const NEGATIVE_PI_DOUBLE: u32 = 0x0101_ff58;
/// 32.0 as a `double`: the height above the land the player is lifted to.
const LAND_CLEARANCE: u32 = 0x0102_f070;
/// 0.1 as a `double`: the vertical movement below which the settling loop stops.
const SETTLE_LIMIT: u32 = 0x0101_ffa0;
/// A `float` (0.16): the time step of the settling moves.
const SETTLE_TIME_STEP: u32 = 0x0108_aff0;
/// A `float` global of the process lists' fader (`01202d98` is the IO
/// manager, `011d8804` the fader manager).
const IO_MANAGER: u32 = 0x0120_2d98;
const FADER_MANAGER: u32 = 0x011d_8804;
/// The `MoviePlayer` singleton pointer.
const MOVIE_PLAYER: u32 = 0x0126_fac4;
/// The `HUDMainMenu` loading-screen flag byte.
const LOADING_SCREEN_FLAG: u32 = 0x011d_8907;
/// The `BSTimer` global (`011f6394`); `0084d030` also reads a float from it.
const GAME_TIMER: u32 = 0x011f_6394;
/// Object whose `00403df0` result is the movie to play.
const MOVIE_OBJECT: u32 = 0x011d_e71c;
/// The `Calendar` global (`011de7b8`): `00867e30` gives its state, `00867950` its time scale.
const CALENDAR_OBJECT: u32 = 0x011d_e7b8;

/// Water-search tile step in cells: `-1`, `0` or `1`, from the position of
/// the player relative to the edges of the cell's `[low, high]` range
/// widened by `radius` (the x87 comparisons: a NaN selects `0`).
fn tile_step(low: f32, high: f32, position: f32, radius: f32) -> i32 {
    if (low as f64) >= position as f64 - radius as f64 {
        -1
    } else if (high as f64) <= position as f64 + radius as f64 {
        1
    } else {
        0
    }
}

// Translated from 0093bba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetWaterCell` (Xbox PDB): finds the cell to look for water
/// in within `radius` of the player (clamped to a limit). The search ends with
/// the player's parent cell (`008d6f30`) when it is an interior cell
/// ([`CELL_IS_INTERIOR`]), passes [`CELL_FLAG_2_TEST`], or no world space is
/// current (`TES::GetWorldSpace`); otherwise the grid cells next to the
/// player's grid cell are tried, toward the edges the radius reaches (x, then
/// y, then the diagonal one; a clamped search finally falls back to
/// `TES::FindFirstGridCellWithWater`), stopping at the first that passes
/// [`CELL_FLAG_2_TEST`]. Returns the cell if it passes that test, else 0.
pub fn player_character_get_water_cell(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    radius: f32,
) -> u32 {
    let mut radius = radius;
    let mut clamped = false;
    let limit: f64 = e.global(WATER_RADIUS_LIMIT);
    if (radius as f64) > limit {
        radius = e.global(WATER_RADIUS_CLAMPED);
        clamped = true;
    }
    let mut cell = e.call(0x008d_6f30, &args![this]).u32();
    'done: {
        if cell != 0 {
            if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
                break 'done;
            }
            if e.call(CELL_FLAG_2_TEST, &args![cell]).bool() {
                break 'done;
            }
            let tes = e.global::<u32>(TES_SINGLETON);
            if e.call(0x004f_d3e0, &args![tes]).u32() == 0 {
                break 'done;
            }
        }
        let position = e.call(0x0089_1170, &args![this]).u32().wrapping_add(0x10);
        let x = e.mem.f32(position);
        let y = e.mem.f32(position.wrapping_add(4));
        let cell_x = e.call(0x0040_6d90, &args![x]).i32() >> 12;
        let cell_y = e.call(0x0040_6d90, &args![y]).i32() >> 12;
        let (low_x, low_y, high_x, high_y) = e.with_stack(16, |e, bounds| {
            let low = bounds.addr();
            let high = low + 8;
            e.call(
                0x0045_2dc0,
                &args![
                    low,
                    cell_x.wrapping_shl(12) as f32,
                    cell_y.wrapping_shl(12) as f32
                ],
            );
            let size: f64 = e.global(CELL_SIZE);
            e.call(
                0x0045_2dc0,
                &args![
                    high,
                    (cell_x.wrapping_shl(12) as f64 + size) as f32,
                    (cell_y.wrapping_shl(12) as f64 + size) as f32
                ],
            );
            (
                e.mem.f32(low),
                e.mem.f32(low + 4),
                e.mem.f32(high),
                e.mem.f32(high + 4),
            )
        });
        let tes = e.global::<u32>(TES_SINGLETON);
        let step_x = tile_step(low_x, high_x, x, radius);
        if step_x != 0 {
            cell = e
                .call(
                    0x0045_1900,
                    &args![tes, cell_x.wrapping_add(step_x), cell_y],
                )
                .u32();
        }
        if cell != 0 && e.call(CELL_FLAG_2_TEST, &args![cell]).bool() {
            break 'done;
        }
        let step_y = tile_step(low_y, high_y, y, radius);
        if step_y != 0 {
            cell = e
                .call(
                    0x0045_1900,
                    &args![tes, cell_x, cell_y.wrapping_add(step_y)],
                )
                .u32();
        }
        if cell != 0 && e.call(CELL_FLAG_2_TEST, &args![cell]).bool() {
            break 'done;
        }
        if step_x != 0 && step_y != 0 {
            cell = e
                .call(
                    0x0045_1900,
                    &args![
                        tes,
                        cell_x.wrapping_add(step_x),
                        cell_y.wrapping_add(step_y)
                    ],
                )
                .u32();
            if cell != 0 && e.call(CELL_FLAG_2_TEST, &args![cell]).bool() {
                break 'done;
            }
            if clamped {
                let found = e.call(0x0045_16d0, &args![tes]).u32();
                if found != 0 {
                    cell = e.mem.u32(found);
                }
            }
        }
    }
    if cell != 0 && !e.call(CELL_FLAG_2_TEST, &args![cell]).bool() {
        cell = 0;
    }
    cell
}

layout! {
    /// A queued `PositionPlayer` request (the object `pQueuedTargetLoc`
    /// points to; the Xbox PDB has no type for it), 0x34 bytes: the fields
    /// are named after how `HandlePositionPlayerRequest` uses them.
    pub struct PositionPlayerRequest: 0x34 {
        /// The world space of an exterior target (0 when the target is a cell).
        0x00 worldspace: Ptr,
        /// The target cell (used when `worldspace` is 0).
        0x04 cell: Ptr,
        /// The target position (three floats).
        0x08 position: Inline<NiPoint3>,
        /// The target rotation (three floats).
        0x14 rotation: Inline<NiPoint3>,
        /// The last argument of `PositionPlayer` (resets the weather).
        0x20 reset_weather: u8,
        /// A callback (an exe address) called with `callback_argument` after
        /// the player is placed; 0 for none.
        0x24 callback: u32,
        /// The argument of the callback.
        0x28 callback_argument: u32,
        /// A furniture reference the player is put into afterwards.
        0x2C furniture: Ptr,
        /// A reference to fast travel to (when set, nothing else is used).
        0x30 fast_travel_target: Ptr,
    }
}

// Translated from 0093be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::RequestPositionPlayer` (Xbox PDB): queues `request` in
/// `pQueuedTargetLoc`, freeing (and, if the new request is not null,
/// reporting that it bashes) an earlier one; when `00451530` says the cell
/// grid can take it, the request is handled at once
/// ([`player_character_handle_position_player_request`]).
pub fn player_character_request_position_player(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    request: Ptr<PositionPlayerRequest>,
) {
    let earlier = e.get(this, PlayerCharacter::pQueuedTargetLoc);
    if !earlier.is_null() {
        if !request.is_null() {
            e.call(0x005b_5e40, &args![0x0108_af58u32]);
        }
        let earlier = e.get(this, PlayerCharacter::pQueuedTargetLoc);
        e.call(OPERATOR_DELETE, &args![earlier]);
    }
    e.set(this, PlayerCharacter::pQueuedTargetLoc, request.cast());
    let tes = e.global::<u32>(TES_SINGLETON);
    if e.call(0x0045_1530, &args![tes]).bool() {
        player_character_handle_position_player_request(e, this);
    }
}

/// The queued request, read afresh (the callees may replace it).
fn queued_request(e: &Engine, this: Ptr<PlayerCharacter>) -> Ptr<PositionPlayerRequest> {
    e.get(this, PlayerCharacter::pQueuedTargetLoc).cast()
}

// Translated from 0093bea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::HandlePositionPlayerRequest` (Xbox PDB): carries out the
/// queued request, if any, and returns whether the player was moved.
/// A request with a fast-travel target travels there ([`player_character_fast_travel`],
/// the `00569b80` result taking precedence over the target); one with a world
/// space calls [`player_character_position_player_exterior`], one with only a
/// cell [`player_character_position_player`], and neither is reported as
/// invalid; after a move the character controller gets the target's height
/// (`00573f20`), the callback runs, and a furniture reference puts the player
/// in a chair or bed (`Actor::PutActorInChairBedQuick`). The request is freed
/// afterwards, the garbage collector cleared, and, unless `0042ce10` or the
/// fast-travel flag (`0093c1e0`) forbids it, `005d14d0` is called on the
/// player.
pub fn player_character_handle_position_player_request(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
) -> bool {
    let mut moved = false;
    if !e.get(this, PlayerCharacter::pQueuedTargetLoc).is_null() {
        let tes = e.global::<u32>(TES_SINGLETON);
        e.call(0x0045_2eb0, &args![tes]);
        e.call(
            NI_POINTER_ASSIGN,
            &args![member(this, PlayerCharacter::spTargeted3D), 0u32],
        );
        let request = queued_request(e, this);
        let travel = e.get(request, PositionPlayerRequest::fast_travel_target);
        if !travel.is_null() {
            let mut target = e.call(0x0056_9b80, &args![travel]).u32();
            if target == 0 {
                let request = queued_request(e, this);
                target = e
                    .get(request, PositionPlayerRequest::fast_travel_target)
                    .addr();
            }
            let player = e.global::<u32>(PLAYER_SINGLETON);
            player_character_fast_travel(e, Ptr::new(player), Ptr::new(target));
            moved = true;
        } else {
            let worldspace = e.get(request, PositionPlayerRequest::worldspace);
            let cell = e.get(request, PositionPlayerRequest::cell);
            if worldspace.is_null() && cell.is_null() {
                e.call(0x005b_5e40, &args![0x0108_afa0u32]);
            } else {
                let position = read_floats(e, request.addr() + 0x08);
                let rotation = read_floats(e, request.addr() + 0x14);
                let flag = e.get(request, PositionPlayerRequest::reset_weather) != 0;
                if !worldspace.is_null() {
                    player_character_position_player_exterior(
                        e,
                        this,
                        position[0],
                        position[1],
                        position[2],
                        rotation[0],
                        rotation[1],
                        rotation[2],
                        worldspace,
                        flag,
                    );
                } else {
                    player_character_position_player(
                        e,
                        this,
                        position[0],
                        position[1],
                        position[2],
                        rotation[0],
                        rotation[1],
                        rotation[2],
                        cell,
                        flag,
                    );
                }
                let controller = e.call(0x0093_06d0, &args![this]).u32();
                let request = queued_request(e, this);
                let height = e.mem.f32(request.addr() + 0x10);
                e.call(0x0057_3f20, &args![controller, height]);
                let request = queued_request(e, this);
                let callback = e.get(request, PositionPlayerRequest::callback);
                if callback != 0 {
                    let request = queued_request(e, this);
                    let argument = e.get(request, PositionPlayerRequest::callback_argument);
                    e.call(callback, &args![argument]);
                }
                moved = true;
            }
            let request = queued_request(e, this);
            let furniture = e.get(request, PositionPlayerRequest::furniture);
            if !furniture.is_null() && e.call(0x0056_8680, &args![furniture]).bool() {
                let process = e.call(0x008d_8520, &args![this]).u32();
                let marker_owner = e.vcall(process, 0x4d4, &args![]).u32();
                let request = queued_request(e, this);
                let furniture = e.get(request, PositionPlayerRequest::furniture);
                let form = e.call(0x007a_f430, &args![furniture]).u32();
                let can_sleep = e.call(0x0050_9420, &args![form]).bool();
                let request = queued_request(e, this);
                let furniture = e.get(request, PositionPlayerRequest::furniture);
                let index = e.call(0x0056_82c0, &args![furniture, 1u32]).i32();
                if index != -1 {
                    let request = queued_request(e, this);
                    let furniture = e.get(request, PositionPlayerRequest::furniture);
                    if e.call(0x0056_8500, &args![furniture, index, marker_owner])
                        .bool()
                    {
                        let request = queued_request(e, this);
                        let furniture = e.get(request, PositionPlayerRequest::furniture);
                        e.call(
                            0x0088_d2f0,
                            &args![this, furniture, marker_owner, index, can_sleep],
                        );
                    }
                }
            }
        }
        e.call(0x0086_8d70, &args![1u32]);
        let request = e.get(this, PlayerCharacter::pQueuedTargetLoc);
        e.call(OPERATOR_DELETE, &args![request]);
        e.set(this, PlayerCharacter::pQueuedTargetLoc, Ptr::NULL);
    }
    if moved {
        let object = e.global::<u32>(GLOBAL_OBJECT_0042CE10);
        if !e.call(0x0042_ce10, &args![object]).bool() {
            let player = e.global::<u32>(PLAYER_SINGLETON);
            if !fn_0093c1e0(e, Ptr::new(player)) {
                e.call(0x005d_14d0, &args![player, 1u32]);
            }
        }
    }
    moved
}

/// The three floats at `addr`.
fn read_floats(e: &Engine, addr: u32) -> [f32; 3] {
    [
        e.mem.f32(addr),
        e.mem.f32(addr.wrapping_add(4)),
        e.mem.f32(addr.wrapping_add(8)),
    ]
}

/// The three words at `addr`.
fn read_words(e: &Engine, addr: u32) -> [u32; 3] {
    [
        e.mem.u32(addr),
        e.mem.u32(addr.wrapping_add(4)),
        e.mem.u32(addr.wrapping_add(8)),
    ]
}

// Translated from 0093c1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit `0x2` of the byte at `+0x66D` (the byte `bCanFastTravel` sits in).
pub fn fn_0093c1e0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    e.mem.u8(this.addr().wrapping_add(0x66d)) & 2 != 0
}

// Translated from 0093ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global word at `011c358c`.
pub fn fn_0093ccd0(e: &mut Engine) -> u32 {
    e.global(GLOBAL_RETURNED_BY_0093CCD0)
}

/// Copies nine words (a 3x3 matrix) from `source` to `destination`.
fn copy_matrix(e: &mut Engine, destination: u32, source: u32) {
    for i in 0..9 {
        let word = e.mem.u32(source.wrapping_add(4 * i));
        e.mem.set_u32(destination.wrapping_add(4 * i), word);
    }
}

/// The spell target the player's `MagicTarget` base is asked about: the
/// object `0093ccd0` returns, at +0x18 (0 when there is none).
fn magic_source(e: &mut Engine) -> u32 {
    let object = fn_0093ccd0(e);
    if object == 0 {
        0
    } else {
        object.wrapping_add(0x18)
    }
}

// Translated from 0093c200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::PositionPlayer` (Xbox PDB): moves the player to the
/// position and rotation (three floats each; the third rotation float is the
/// heading, the first two pitch and roll) in `cell`. In order: the menu
/// background is rendered when the cell grid cannot take requests
/// (`00451530`), the process lists are prepared (`009785d0`) and a loading
/// menu's tile menu deleted; the encounter zones of the old and new cell are
/// switched (`00526070`/`005260a0`); the player leaves its parent cell, takes
/// the position (virtual `+0x2A8`) and rotation (`SetAngleOnReference`) and
/// enters `cell`; an interior cell ([`CELL_IS_INTERIOR`]) is entered through
/// `00453dc0` and clears the canopy shadows, any other keeps them (when the
/// world space has them) or clears the mask and sets the world space; a player
/// in move mode (`005f36f0`) has a spell target dispelled and the pipboy
/// light shown; sounds of kind `0x1000` stop and the weather resets when
/// `reset_weather`; the fader starts for a player without 3D; the player's
/// matrix is rebuilt from heading, pitch and roll and given to the 3D root and
/// the player's node; the audio listener is placed and oriented; for a player
/// with first-person 3D the position is lifted above the land and settled by
/// moving it down (at most 100 `Move`s) and the animations are refreshed;
/// the camera rigid body, terrain and world-space text are updated. The SEH
/// frame (stack-cookie check) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn player_character_position_player(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
    cell: Ptr,
    reset_weather: bool,
) {
    // The arguments live on the game's stack: position at +0x0, rotation at
    // +0xC, with the locals behind them.
    e.with_stack(0x180, |e, frame| {
        let f = frame.addr();
        e.mem.set_f32(f, position_x);
        e.mem.set_f32(f + 4, position_y);
        e.mem.set_f32(f + 8, position_z);
        e.mem.set_f32(f + 0xc, rotation_x);
        e.mem.set_f32(f + 0x10, rotation_y);
        e.mem.set_f32(f + 0x14, rotation_z);
        position_player_in_frame(e, this, f, cell, reset_weather);
    });
}

/// Frame offsets used by [`position_player_in_frame`].
const FRAME_POSITION: u32 = 0x00;
const FRAME_ROTATION: u32 = 0x0c;
const FRAME_PLAYER_MATRIX: u32 = 0x18;
const FRAME_TURN_MATRIX: u32 = 0x3c;
const FRAME_PRODUCT_A: u32 = 0x60;
const FRAME_PRODUCT_B: u32 = 0x84;
const FRAME_UPDATE_DATA: u32 = 0xa8;
const FRAME_FORWARD: u32 = 0xb8;
const FRAME_START_POSITION: u32 = 0xc4;
const FRAME_LAND_HEIGHT: u32 = 0xd0;
const FRAME_MOVE_VECTOR: u32 = 0xd4;
const FRAME_CAMERA_POSITION: u32 = 0xe0;
const FRAME_CAMERA_MATRIX: u32 = 0xf0;

fn position_player_in_frame(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    f: u32,
    cell: Ptr,
    reset_weather: bool,
) {
    let position = f + FRAME_POSITION;
    let rotation = f + FRAME_ROTATION;
    let tes = e.global::<u32>(TES_SINGLETON);
    if !e.call(0x0045_1530, &args![tes]).bool() {
        let main = e.global::<u32>(MAIN_SINGLETON);
        e.call(0x0087_1dc0, &args![main]);
    }
    e.call(0x0097_85d0, &args![PROCESS_LISTS]);
    if e.call(0x0070_edf0, &args![]).bool() {
        e.call(0x0070_5e30, &args![]);
        let tile = e.call(0x0070_ede0, &args![]).u32();
        let menu_class = e.call(0x00a0_9030, &args![tile]).u32();
        let menu = e.call(0x00a0_3c90, &args![menu_class]).u32();
        if menu != 0 {
            e.vcall(menu, 0, &args![1u32]);
        }
    }
    let uses_move_mode = e.call(0x005f_36f0, &args![tes]).u32() != 0;
    let world = e.call(0x004f_d3e0, &args![tes]).u32();
    e.call(0x0054_ddd0, &args![cell]);
    let new_zone = e.call(0x0054_6c20, &args![cell]).u32();
    let mut old_zone = 0;
    if e.call(0x008d_6f30, &args![this]).u32() != 0 {
        let parent = e.call(0x008d_6f30, &args![this]).u32();
        old_zone = e.call(0x0054_6c20, &args![parent]).u32();
    } else if world != 0 {
        old_zone = e.call(0x0045_8400, &args![world]).u32();
    }
    if old_zone != new_zone {
        let state = e.call(0x0086_7e30, &args![CALENDAR_OBJECT]).u32();
        if old_zone != 0 {
            e.call(0x0052_6070, &args![old_zone, state]);
        }
        if new_zone != 0 {
            e.call(0x0052_60a0, &args![new_zone, state]);
        }
    }
    let movie = e.global::<u32>(MOVIE_PLAYER);
    let playing_sequence = e.call(0x00ec_17c0, &args![movie]).bool();
    if e.call(0x008d_6f30, &args![this]).u32() != 0 {
        let parent = e.call(0x008d_6f30, &args![this]).u32();
        e.call(0x0054_ca90, &args![parent, this]);
    }
    e.vcall(this.addr(), 0x2a8, &args![position]);
    e.call(0x0086_d490, &args![this, rotation]);
    e.call(0x0057_5bb0, &args![this, cell]);
    'audio: {
        if !cell.is_null() && e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
            let music = e.call(0x0054_74b0, &args![cell, 0u32]).u32();
            e.set(this, PlayerCharacter::pLastKnownMusicType, Ptr::new(music));
            e.call(0x0045_3dc0, &args![tes, cell, position]);
            e.call(0x0066_5860, &args![]);
            break 'audio;
        }
        if !cell.is_null() {
            let music = e.call(0x0054_74b0, &args![cell, 0u32]).u32();
            e.set(this, PlayerCharacter::pLastKnownMusicType, Ptr::new(music));
            let mut clear_canopy = true;
            if e.call(0x0045_2480, &args![]).bool() {
                let world = e.call(0x0054_ddd0, &args![cell]).u32();
                let flag = e.call(0x0054_8210, &args![world]).u32();
                if e.mem.i8(flag) != 0 {
                    e.call(0x0066_5610, &args![]);
                    clear_canopy = false;
                }
            }
            if clear_canopy {
                e.call(0x0066_5860, &args![]);
                let grid = e.call(0x0044_ddc0, &args![tes]).u32();
                e.call(0x004b_aec0, &args![grid]);
            }
            let world = e.call(0x0054_ddd0, &args![cell]).u32();
            e.call(0x0045_8200, &args![tes, world]);
        }
        if uses_move_mode {
            let source = magic_source(e);
            if e.call(
                0x0082_2b90,
                &args![this.addr() + MAGIC_TARGET_BASE, source, 1u32],
            )
            .bool()
            {
                let pipboy = e.call(0x0070_5990, &args![]).u32();
                e.call(0x007f_a310, &args![pipboy, 1u32, 0u32, 1u32]);
                let pipboy = e.call(0x0070_5990, &args![]).u32();
                e.call(0x007f_a310, &args![pipboy, 0u32, 0u32, 1u32]);
                let source = magic_source(e);
                e.call(
                    0x0082_4400,
                    &args![this.addr() + MAGIC_TARGET_BASE, source, 0u32, 0u32],
                );
            }
        }
        e.call(0x0045_4450, &args![tes, position]);
    }
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(SOUND_HANDLE_DESTRUCT, &args![audio]);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(0x00ad_8780, &args![audio, 0x1000u32]);
    if reset_weather {
        let sky = e.call(0x0046_dd00, &args![]).u32();
        e.call(0x0063_d060, &args![sky]);
    }
    e.call(0x0057_5bb0, &args![this, 0u32]);
    e.call(0x0054_8230, &args![cell, this, 0u32]);
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() == 0 {
        let object = e.global::<u32>(GLOBAL_OBJECT_0042CE10);
        if !e.call(0x0042_ce10, &args![object]).bool() {
            let io = e.global::<u32>(IO_MANAGER);
            e.call(0x0045_6520, &args![io]);
            let fader = e.global::<u32>(FADER_MANAGER);
            e.call(0x0070_0960, &args![fader, 1u32, 0.0f32, 0u32]);
        }
    }

    // The player's 3D: heading, pitch and roll rotations.
    let player_matrix = f + FRAME_PLAYER_MATRIX;
    let turn_matrix = f + FRAME_TURN_MATRIX;
    copy_matrix(e, player_matrix, IDENTITY_MATRIX);
    e.call(LIST_ITEM_ADDRESS, &args![turn_matrix]);
    let pi: f64 = e.global(PI_DOUBLE);
    let two_pi: f64 = e.global(TWO_PI_DOUBLE);
    let negative_pi: f64 = e.global(NEGATIVE_PI_DOUBLE);
    let mut heading = e.mem.f32(rotation + 8);
    while heading as f64 > pi {
        heading = (heading as f64 - two_pi) as f32;
        e.mem.set_f32(rotation + 8, heading);
    }
    while (heading as f64) < negative_pi {
        heading = (heading as f64 + two_pi) as f32;
        e.mem.set_f32(rotation + 8, heading);
    }
    e.call(0x004a_0c90, &args![turn_matrix, heading]);
    let product = e
        .call(
            0x0043_f8d0,
            &args![player_matrix, f + FRAME_PRODUCT_A, turn_matrix],
        )
        .u32();
    copy_matrix(e, player_matrix, product);
    let rotation_x = e.mem.f32(rotation);
    let rotation_y = e.mem.f32(rotation + 4);
    let tilt = if heading >= 0.0 {
        (rotation_x as f64 - rotation_y as f64) as f32
    } else {
        (rotation_x as f64 + rotation_y as f64) as f32
    };
    e.call(0x0052_4ac0, &args![turn_matrix, tilt]);
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() != 0 {
        let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
        e.call(0x0043_fa80, &args![node, turn_matrix]);
    }
    let product = e
        .call(
            0x0043_f8d0,
            &args![turn_matrix, f + FRAME_PRODUCT_B, player_matrix],
        )
        .u32();
    copy_matrix(e, player_matrix, product);
    let root = e.call(0x0045_c670, &args![]).u32();
    let node = e.call(0x0055_8310, &args![root]).u32();
    e.call(0x0043_fa80, &args![node, player_matrix]);
    let root = e.call(0x0045_c670, &args![]).u32();
    let node = e.call(0x0055_8310, &args![root]).u32();
    e.call(0x0044_0460, &args![node, position]);
    let update_data = f + FRAME_UPDATE_DATA;
    e.call(0x0043_d410, &args![update_data, 0.0f32, 0u32, 0u32]);
    let root = e.call(0x0045_c670, &args![]).u32();
    let node = e.call(0x0055_8310, &args![root]).u32();
    e.call(0x00a5_9c60, &args![node, update_data]);
    let scene = e.call(0x0084_e3a0, &args![tes]).u32();
    e.call(0x00a5_a040, &args![scene]);

    // The audio listener.
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let [px, py, pz] = read_words(e, position);
    e.call(0x00ad_78b0, &args![audio, px, py, pz]);
    let up = read_words(e, LISTENER_UP_VECTOR);
    let angle = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
    let y = e.call(0x005b_9e80, &args![angle]).f32();
    let angle = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
    let x = e.call(0x005c_53d0, &args![angle]).f32();
    let forward = e
        .call(0x0041_6870, &args![f + FRAME_FORWARD, x, y, 0.0f32])
        .u32();
    let [fx, fy, fz] = read_words(e, forward);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(0x00ad_7940, &args![audio, fx, fy, fz, up[0], up[1], up[2]]);
    e.call(0x0095_2c30, &args![this, this]);
    let accumulator = e.call(0x00b4_f5c0, &args![]).u32();
    if accumulator != 0 {
        e.call(0x00b6_55b0, &args![accumulator]);
    }

    // The first-person 3D: settle the position and refresh the animations.
    if e.call(
        0x0055_9450,
        &args![member(this, PlayerCharacter::sp1stPerson3D)],
    )
    .u32()
        != 0
    {
        let pipboy = e.call(0x0070_5990, &args![]).u32();
        e.call(0x007f_a200, &args![pipboy]);
        let tes_object = e.call(0x0045_0b80, &args![0u32]).u32();
        e.call(0x00b5_fd60, &args![tes_object]);
        let object = e.global::<u32>(GLOBAL_OBJECT_0042CE10);
        if !e.call(0x0042_ce10, &args![object]).bool() {
            settle_player_position(e, this, f, cell);
        }
        let toggled = !e.get(this, PlayerCharacter::b3rdPerson);
        e.set(this, PlayerCharacter::b3rdPerson, toggled);
        refresh_animation(e, this);
        let toggled = !e.get(this, PlayerCharacter::b3rdPerson);
        e.set(this, PlayerCharacter::b3rdPerson, toggled);
        refresh_animation(e, this);
        e.call(0x0094_ae40, &args![this, 0u32, 0u32]);
    }
    if e.call(
        0x0055_9450,
        &args![member(this, PlayerCharacter::spCameraRigidBody)],
    )
    .u32()
        != 0
    {
        let camera_matrix = f + FRAME_CAMERA_MATRIX;
        let camera_position = f + FRAME_CAMERA_POSITION;
        e.call(LIST_ITEM_ADDRESS, &args![camera_matrix]);
        let current = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let words = read_words(e, current);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(camera_position + 4 * i as u32, *word);
        }
        e.call(0x004a_3e00, &args![camera_matrix, camera_position]);
        let body = e
            .call(
                0x0055_9450,
                &args![member(this, PlayerCharacter::spCameraRigidBody)],
            )
            .u32();
        e.call(0x0056_10f0, &args![body, camera_position]);
    }
    let weapon = e.call(0x0052_4c90, &args![]).u32();
    e.call(0x0066_52e0, &args![weapon, 0u32]);
    let fade = e.call(0x0084_d030, &args![GAME_TIMER]).f32();
    e.call(0x0045_3550, &args![tes, fade]);
    let object = e.global::<u32>(GLOBAL_OBJECT_0042CE10);
    if !e.call(0x0042_ce10, &args![object]).bool() {
        e.call(0x0078_cfc0, &args![]);
    }
    if playing_sequence {
        let movie = e.global::<u32>(MOVIE_PLAYER);
        e.call(0x00ec_1800, &args![movie]);
    }
    e.call(0x0096_1f90, &args![this]);
    e.call(0x0045_a750, &args![tes]);
    if e.call(0x0057_5d70, &args![this]).u32() != 0 {
        let current = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let world = e.call(0x0057_5d70, &args![this]).u32();
        let manager = e.call(0x0058_6170, &args![world]).u32();
        e.call(0x006f_ca90, &args![manager, current, 0x0fu32]);
        let current = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let world = e.call(0x0057_5d70, &args![this]).u32();
        let manager = e.call(0x0058_6170, &args![world]).u32();
        e.call(0x006f_cdb0, &args![manager, current]);
    }
    let main = e.global::<u32>(MAIN_SINGLETON);
    e.call(SOUND_HANDLE_DESTRUCT, &args![main]);
    if e.global::<u8>(LOADING_SCREEN_FLAG) != 0 {
        e.call(0x0045_7d70, &args![tes, 0u32, 0u32, 0u32]);
        e.call(0x0077_1700, &args![0x18u32]);
        e.call(0x0083_04a0, &args![]);
        let movie_name = e.call(0x0040_3df0, &args![MOVIE_OBJECT]).u32();
        let movie = e.global::<u32>(MOVIE_PLAYER);
        e.call(
            0x00ec_2320,
            &args![movie, movie_name, 1u32, 1u32, 0u32, 1u32, 1u32, 0u32, 1u32, 1u32],
        );
        e.call(0x0077_1700, &args![0x0cu32]);
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    let name = if e.call(0x0057_5d70, &args![player]).u32() != 0 {
        let world = e.call(0x0057_5d70, &args![player]).u32();
        e.call(0x0040_8da0, &args![world.wrapping_add(0x18)]).u32()
    } else {
        0
    };
    e.call(0x0077_2c30, &args![name]);
    e.call(0x0097_3de0, &args![PROCESS_LISTS]);
}

/// The player is lifted onto the land when it is below it (for a cell that
/// is not an interior), then moved down in 100 steps at most until it no longer falls;
/// when the loop does not settle, it is put back at its start position.
fn settle_player_position(e: &mut Engine, this: Ptr<PlayerCharacter>, f: u32, cell: Ptr) {
    let start = f + FRAME_START_POSITION;
    let current = e.vcall(this.addr(), 0x1f4, &args![]).u32();
    let words = read_words(e, current);
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(start + 4 * i as u32, *word);
    }
    if !cell.is_null() && !e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
        let height = f + FRAME_LAND_HEIGHT;
        e.mem.set_f32(height, 0.0);
        if e.call(0x0055_47c0, &args![cell, start, height]).bool() {
            let clearance: f64 = e.global(LAND_CLEARANCE);
            let land = e.mem.f32(height);
            let z = e.mem.f32(start + 8);
            if (z as f64) < land as f64 + clearance {
                e.mem.set_f32(start + 8, (land as f64 + clearance) as f32);
                e.vcall(this.addr(), 0x2a8, &args![start]);
            }
        }
    }
    let limit: f64 = e.global(SETTLE_LIMIT);
    let mut step = 0;
    while step < 100 {
        let before = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let before_z = e.mem.f32(before + 8);
        let vector = f + FRAME_MOVE_VECTOR;
        let zero_vector = read_words(e, ZERO_VECTOR);
        for (i, word) in zero_vector.iter().enumerate() {
            e.mem.set_u32(vector + 4 * i as u32, *word);
        }
        let time_step = e.global::<f32>(SETTLE_TIME_STEP);
        e.call(0x0092_f260, &args![this, time_step, vector, 0u32]);
        let after = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let after_z = e.mem.f32(after + 8);
        let moved = (after_z as f64 - before_z as f64) as f32;
        let magnitude = e.call(0x0040_8840, &args![moved]).f64();
        if magnitude < limit {
            break;
        }
        step += 1;
    }
    if step == 100 {
        e.vcall(this.addr(), 0x2a8, &args![start]);
    }
}

/// Refreshes the animation after the third-person flag changed: `008d3550`
/// with 0.0, then `Actor::UpdateAnimationMovement` with the animation
/// `GetAnimation` gives for the opposite of the flag.
fn refresh_animation(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    e.call(0x008d_3550, &args![this, 0.0f32]);
    let first_person = !e.get(this, PlayerCharacter::b3rdPerson);
    let animation = e.call(0x0095_0a60, &args![this, first_person]).u32();
    e.call(0x0088_85e0, &args![this, animation, 0.0f32]);
}

// Translated from 0093cce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::PositionPlayerExterior` (Xbox PDB): with a world space,
/// finds (or loads, or creates) the cell at the grid cell the position is in
/// (cancelling pending exterior loads first and clearing the cells that are
/// not needed, [`player_character_clear_cells_if_target_not_loaded`]) and
/// moves the player singleton there ([`player_character_position_player`]);
/// without a world space nothing happens.
#[allow(clippy::too_many_arguments)]
pub fn player_character_position_player_exterior(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
    world_space: Ptr,
    reset_weather: bool,
) {
    let mut cell = 0u32;
    if !world_space.is_null() {
        let loader = e.global::<u32>(EXTERIOR_CELL_LOADER);
        if loader != 0 {
            e.call(0x0052_8540, &args![loader]);
        }
        let cell_x = e.call(0x0040_6d90, &args![position_x]).i32() >> 12;
        let cell_y = e.call(0x0040_6d90, &args![position_y]).i32() >> 12;
        cell = e
            .call(0x0058_75a0, &args![world_space, cell_x, cell_y])
            .u32();
        player_character_clear_cells_if_target_not_loaded(e, this, Ptr::new(cell), world_space);
        if cell == 0 {
            cell = e
                .call(0x0058_5b30, &args![world_space, cell_x, cell_y])
                .u32();
        }
        if cell == 0 {
            let handler = e.global::<u32>(DATA_HANDLER);
            cell = e
                .call(
                    0x0046_1330,
                    &args![handler, 0u32, cell_x, cell_y, world_space],
                )
                .u32();
        }
    }
    if cell != 0 {
        let player = e.global::<u32>(PLAYER_SINGLETON);
        player_character_position_player(
            e,
            Ptr::new(player),
            position_x,
            position_y,
            position_z,
            rotation_x,
            rotation_y,
            rotation_z,
            Ptr::new(cell),
            reset_weather,
        );
    }
}

/// The two statics of `FastTravel`: the guard bits (bit 0 for the timestamp
/// below) and the game time of the last `00850a40` call.
const FAST_TRAVEL_STATIC_FLAGS: u32 = 0x011e_0ba4;
const FAST_TRAVEL_LAST_TIME: u32 = 0x011e_0ba0;
/// A `byte` setting object (`00408d60`'s `this`): whether to ask `00850a40`
/// to react to a long pause between fast travels.
const FAST_TRAVEL_SETTING: u32 = 0x011d_8980;
/// A pointer global whose object `FastTravel` passes to `00850a40`.
const FAST_TRAVEL_REACTOR: u32 = 0x011d_e134;
/// The fast-travel flag byte (`011d8906`) other code reads.
const FAST_TRAVELLING: u32 = 0x011d_8906;
/// 3600.0 as a `double` and as a `float` (seconds per hour).
const SECONDS_PER_HOUR_DOUBLE: u32 = 0x0101_2640;
const SECONDS_PER_HOUR_FLOAT: u32 = 0x0108_4838;
/// The `float` maximum (a `double`) `Pathing::ComputeTeleportDoorPathLength`
/// returns when there is no path.
const NO_PATH_LENGTH: u32 = 0x0102_31b0;
/// 2.0 as a `float`: the duration of the messages and health restored per
/// fast-travel hour.
const TWO_FLOAT: u32 = 0x0101_62c0;
/// The VATS object (`011f2250`).
const VATS_OBJECT: u32 = 0x011f_2250;
/// The object whose `0040fbf0`/`0040fba0` bracket each simulated hour.
const HOUR_BRACKET_OBJECT: u32 = 0x011f_11a0;

// Translated from 0093cdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::FastTravel` (Xbox PDB): travels to `destination`, a
/// reference. Does nothing when the entry point `0x33` (`005e58f0`) leaves the
/// delay at 0 and the player's virtual `+0x358` agrees. Otherwise the cell grid
/// is flushed, the player leaves any furniture, the path length to the
/// destination is computed (`Pathing::ComputeTeleportDoorPathLength`; with no
/// path a message naming the destination's cell is printed and the length is
/// 0), the travel time is that length over the run speed (in hours, scaled by
/// the multi-bound radius `00526ac0` gives for the setting `0x3a`), the player's
/// sleep time (`iSleepTime`) is set to its whole hours, and time is advanced:
/// hour by hour with the actors updated and the player healed when there are
/// whole hours, otherwise by the fraction at once. Then the player is placed at
/// the destination (`PositionPlayerExterior` with the destination's world
/// space, position and rotation) and the beds, chairs and actors are set up
/// for the new place. The SEH frame is not translated.
pub fn player_character_fast_travel(e: &mut Engine, this: Ptr<PlayerCharacter>, destination: Ptr) {
    let zero: f64 = e.global(ZERO_DOUBLE);
    let delay = e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), 0.0);
        e.call(0x005e_58f0, &args![0x33u32, this, slot]);
        e.mem.f32(slot.addr())
    });
    if e.vcall(this.addr(), 0x358, &args![]).bool() && delay as f64 == zero {
        return;
    }
    let flags: u32 = e.global(FAST_TRAVEL_STATIC_FLAGS);
    if flags & 1 == 0 {
        e.set_global(FAST_TRAVEL_STATIC_FLAGS, flags | 1);
        let now = e.call(0x0082_5c00, &args![GAME_TIMER]).u32();
        e.set_global(FAST_TRAVEL_LAST_TIME, now);
    }
    let now = e.call(0x0082_5c00, &args![GAME_TIMER]).u32();
    let elapsed = now.wrapping_sub(e.global::<u32>(FAST_TRAVEL_LAST_TIME));
    let setting = e.call(0x0040_8d60, &args![FAST_TRAVEL_SETTING]).u32();
    if e.mem.u8(setting) != 0 && (elapsed == 0 || elapsed > 20000) {
        let reactor = e.global::<u32>(FAST_TRAVEL_REACTOR);
        e.call(0x0085_0a40, &args![reactor]);
        let now = e.call(0x0082_5c00, &args![GAME_TIMER]).u32();
        e.set_global(FAST_TRAVEL_LAST_TIME, now);
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    e.mem.set_u8(player + 0x20c, 1);
    let tes = e.global::<u32>(TES_SINGLETON);
    e.with_stack(16, |e, state| {
        e.call(0x0087_8160, &args![state, 1u32, 1u32, 1u32]);
        e.call(0x0045_39a0, &args![tes, 1u32, 0u32]);
        let byte = e.mem.u8(state.addr() + 5);
        e.call(0x0087_8250, &args![byte as u32]);
        e.call(0x0087_8200, &args![state]);
    });
    let mover = e
        .get(
            this.cast::<crate::units::fallout_ai::actor::Actor>(),
            crate::units::fallout_ai::actor::Actor::pActorMover,
        )
        .addr();
    e.call(0x009e_a3b0, &args![mover, 0x800u32]);
    let controller = e.call(0x0093_06d0, &args![this]).u32();
    e.call(0x0088_b0a0, &args![controller, 0u32]);
    e.call(0x0088_d640, &args![this]);

    // Frame: the path length (+0), the lock data (+0x10), two pathing
    // locations (+0x20, +0x50), a string (+0x80) and the message (+0x90).
    e.with_stack(0x490, |e, frame| {
        let f = frame.addr();
        let length_slot = f;
        let lock_data = f + 0x10;
        let destination_location = f + 0x20;
        let player_location = f + 0x50;
        let name = f + 0x80;
        let message = f + 0x90;
        e.mem.set_f32(length_slot, 0.0);
        e.call(LIST_ITEM_ADDRESS, &args![f + 0x88]);
        let mut position = [0u32; 3];
        for (i, word) in position.iter_mut().enumerate() {
            let current = e.vcall(player, 0x1f4, &args![]).u32();
            *word = e.mem.u32(current + 4 * i as u32);
        }
        e.call(
            0x008d_0500,
            &args![
                this,
                destination,
                length_slot,
                position[0],
                position[1],
                position[2]
            ],
        );
        e.call(0x0050_2670, &args![lock_data, player]);
        let destination_loc = e
            .call(0x006d_cd70, &args![destination_location, destination])
            .u32();
        let player_loc = e.call(0x006d_cd70, &args![player_location, player]).u32();
        let length = e
            .call(
                0x006d_4eb0,
                &args![player_loc, destination_loc, 0.0f32, lock_data, 0u32],
            )
            .f32();
        e.mem.set_f32(length_slot, length);
        e.call(0x004f_f7e0, &args![player_location]);
        e.call(0x004f_f7e0, &args![destination_location]);
        let mut length = length;
        let no_path: f64 = e.global(NO_PATH_LENGTH);
        if length as f64 == no_path {
            e.call(0x0040_6d00, &args![message, 0x400u32, 0x0108_aff4u32]);
            if e.call(0x008d_6f30, &args![destination]).u32() != 0 {
                e.call(0x0040_37b0, &args![name]);
                let cell = e.call(0x008d_6f30, &args![destination]).u32();
                e.vcall(cell, 0x90, &args![name]);
                let text = e.call(0x0055_9450, &args![name]).u32();
                e.call(0x0040_6d50, &args![message, 0x400u32, text]);
                e.call(0x0040_37d0, &args![name]);
            }
            length = 0.0;
            e.call(0x005b_5e40, &args![message]);
        }
        let setting = e.call(0x0048_39c0, &args![0x3au32]).u32();
        let speed = e.call(0x0088_4eb0, &args![player]).f32();
        let per_second = (length as f64 / speed as f64) as f32;
        let per_hour: f64 = e.global(SECONDS_PER_HOUR_DOUBLE);
        let mut hours = (per_second as f64 / per_hour) as f32;
        let radius = e.call(0x0052_6ac0, &args![setting]).f32();
        hours = (radius as f64 * hours as f64) as f32;
        let whole_hours = e.call(FTOL, &args![hours as f64]).u32();
        e.set(this, PlayerCharacter::iSleepTime, whole_hours);
        e.set(this, PlayerCharacter::bIsSleeping, false);
        e.call(0x0045_7d70, &args![tes, 1u32, destination, 0u32]);
        if e.get(this, PlayerCharacter::iSleepTime) != 0 {
            loop {
                let remaining = e.get(this, PlayerCharacter::iSleepTime) as i32;
                if remaining <= 0 {
                    break;
                }
                let scale = e.call(0x0086_7950, &args![CALENDAR_OBJECT]).f32();
                let step = (per_hour / scale as f64) as f32;
                let clock = e.call(0x0096_d490, &args![PROCESS_LISTS]).f32();
                let advanced = (clock as f64 + step as f64) as f32;
                e.call(0x0096_d4b0, &args![PROCESS_LISTS, advanced]);
                e.call(0x0086_7a40, &args![CALENDAR_OBJECT, step]);
                e.call(0x0040_fbf0, &args![HOUR_BRACKET_OBJECT, 0u32]);
                let hour: f32 = e.global(SECONDS_PER_HOUR_FLOAT);
                e.call(0x0096_bcd0, &args![PROCESS_LISTS, hour, 1u32]);
                e.call(0x0096_db30, &args![PROCESS_LISTS, hour, 1u32, 0u32]);
                e.call(0x0096_b810, &args![PROCESS_LISTS, hour, 1u32]);
                e.call(0x0096_b470, &args![PROCESS_LISTS, hour, 1u32]);
                e.call(0x0096_b050, &args![PROCESS_LISTS, hour, 1u32]);
                e.call(0x0096_eb40, &args![PROCESS_LISTS]);
                e.call(0x0040_fba0, &args![HOUR_BRACKET_OBJECT]);
                let remaining = e.get(this, PlayerCharacter::iSleepTime);
                e.set(this, PlayerCharacter::iSleepTime, remaining.wrapping_sub(1));
                let restore: f32 = e.global(TWO_FLOAT);
                e.call(0x0088_b510, &args![player, restore]);
            }
        } else {
            let scale_hours = hours as f64 * per_hour;
            let scale = e.call(0x0086_7950, &args![CALENDAR_OBJECT]).f32();
            let step = (scale_hours / scale as f64) as f32;
            let clock = e.call(0x0096_d490, &args![PROCESS_LISTS]).f32();
            let advanced = (clock as f64 + step as f64) as f32;
            e.call(0x0096_d4b0, &args![PROCESS_LISTS, advanced]);
            e.call(0x0086_7a40, &args![CALENDAR_OBJECT, step]);
        }
    });
    e.call(0x0097_3ee0, &args![PROCESS_LISTS, destination]);
    if fn_0093d4f0(e) {
        e.call(0x0045_aee0, &args![tes, 0u32]);
    }
    if e.call(0x0065_2110, &args![]).u32() != 0 {
        let cache = e.call(0x0065_2110, &args![]).u32();
        e.call(0x0065_0a30, &args![cache, 0u32]);
    }
    e.call(0x00aa_7030, &args![]);
    let sky = e.call(0x0046_dd00, &args![]).u32();
    e.call(0x0063_e8f0, &args![sky, 1u32]);
    e.set_global::<u8>(FAST_TRAVELLING, 1);
    let world_space = e.call(0x0057_5d70, &args![destination]).u32();
    let rotation = e.call(0x0043_0830, &args![destination]).u32();
    let rotation = read_floats(e, rotation);
    let current = e.vcall(destination.addr(), 0x1f4, &args![]).u32();
    let position = read_floats(e, current);
    player_character_position_player_exterior(
        e,
        Ptr::new(player),
        position[0],
        position[1],
        position[2],
        rotation[0],
        rotation[1],
        rotation[2],
        Ptr::new(world_space),
        true,
    );
    e.set_global::<u8>(FAST_TRAVELLING, 0);
    e.set(this, PlayerCharacter::iSleepTime, 0);
    let sky = e.call(0x0046_dd00, &args![]).u32();
    e.call(0x0063_e8f0, &args![sky, 0u32]);
    if fn_0093d4f0(e) {
        e.call(0x0045_aee0, &args![tes, 1u32]);
    }
    e.call(0x0048_3710, &args![PROCESS_LISTS]);
    e.call(0x0045_9870, &args![tes]);
    e.call(0x0097_2d30, &args![PROCESS_LISTS]);
    e.call(0x0096_df40, &args![PROCESS_LISTS]);
    let player = e.global::<u32>(PLAYER_SINGLETON);
    e.mem.set_u8(player + 0x20c, 0);
    e.call(0x0097_5d10, &args![PROCESS_LISTS]);
    e.vcall(this.addr(), 0x3f4, &args![]);
    e.call(0x0096_1f90, &args![this]);
    let player = e.global::<u32>(PLAYER_SINGLETON);
    e.mem.set_u8(player + 0x204, 0);
}

// Translated from 0093d4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00454af0` on the object at `011c3c10` and returns its byte result.
pub fn fn_0093d4f0(e: &mut Engine) -> bool {
    e.call(0x0045_4af0, &args![0x011c_3c10u32]).bool()
}

// Translated from 0093d500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ClearCellsIfTargetNotLoaded` (Xbox PDB): unless `cell` is
/// already loaded (`TES::IsCellLoaded`), flushes the cell grid for the new
/// target (`cell`, or `world_space` when there is none): the player leaves its
/// parent cell, the object `00950bb0` finds is told (virtual `+0xE8` of the
/// object `009611e0` finds), the process lists are updated and the freed
/// objects released, and the face-gen model cache and `00aa7030` are cleaned
/// up.
pub fn player_character_clear_cells_if_target_not_loaded(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    cell: Ptr,
    world_space: Ptr,
) {
    let tes = e.global::<u32>(TES_SINGLETON);
    if !cell.is_null() && e.call(0x0045_11e0, &args![tes, cell, 0u32]).bool() {
        return;
    }
    let target = if cell.is_null() { world_space } else { cell };
    e.call(0x0045_7d70, &args![tes, 1u32, target, 0u32]);
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if e.call(0x008d_6f30, &args![player]).u32() != 0 {
        let player = e.global::<u32>(PLAYER_SINGLETON);
        let parent = e.call(0x008d_6f30, &args![player]).u32();
        e.call(0x0054_ca90, &args![parent, player]);
    }
    let found = e.call(0x0095_0bb0, &args![this, 1u32]).u32();
    let inner = if found != 0 {
        e.vcall(found, 0xc, &args![]).u32()
    } else {
        0
    };
    if inner != 0 && e.call(0x0096_11e0, &args![inner]).u32() != 0 {
        let handler = e.call(0x0096_11e0, &args![inner]).u32();
        e.vcall(handler, 0xe8, &args![found]);
    }
    e.with_stack(16, |e, state| {
        e.call(0x0087_8160, &args![state, 0u32, 1u32, 0u32]);
        e.call(0x0045_39a0, &args![tes, 0u32, 1u32]);
        e.set_global::<u8>(PROCESS_LISTS_BUSY, 1);
        e.call(0x0096_d810, &args![PROCESS_LISTS]);
        e.call(0x0096_eb40, &args![PROCESS_LISTS]);
        e.set_global::<u8>(PROCESS_LISTS_BUSY, 0);
        let byte = e.mem.u8(state.addr() + 5);
        e.call(0x0087_8250, &args![byte as u32]);
        e.call(0x0087_8200, &args![state]);
    });
    if e.call(0x0065_2110, &args![]).u32() != 0 {
        let cache = e.call(0x0065_2110, &args![]).u32();
        e.call(0x0065_0a30, &args![cache, 0u32]);
    }
    e.call(0x00aa_7030, &args![]);
}

// Translated from 0093d660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the player may fast travel now, printing a message with the sad
/// vault-boy icon and returning false at the first condition that fails: the
/// player is somewhere fast travel is not allowed (`009764a0` with the
/// interior flag `00575d10` gives), a certain list (`00971c30` kind `0x15`)
/// is not empty (it is also cleared), an active effect of the `MagicTarget`
/// base of the player has the property virtual `+0x3C` reports, the travel
/// flag (`0093db40`) is off, the entry point `0x33` leaves a delay below 1.0
/// (message without icon), health or the other actor value `0x16` are below 1,
/// `008849c0` says so, or the parent cell/world space refuses
/// (`00544520`/`00586210`).
pub fn fn_0093d660(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    let player = e.global::<u32>(PLAYER_SINGLETON);
    let interior = e.call(0x0057_5d10, &args![player]).bool();
    if e.call(0x0097_64a0, &args![PROCESS_LISTS, interior as u32])
        .bool()
    {
        show_message(e, 0x011d_3dd8, FAST_TRAVEL_ICON);
        return false;
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    let list = e
        .call(0x0097_1c30, &args![PROCESS_LISTS, player, 0x15u32, 0u32])
        .u32();
    if list != 0 && !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        show_message(e, 0x011d_4114, FAST_TRAVEL_ICON);
        e.call(LIST_CLEAR, &args![list]);
        if list != 0 {
            e.call(LIST_DELETING_DESTRUCT, &args![list, 1u32]);
        }
        return false;
    }
    let mut effect = e.vcall(this.addr() + MAGIC_TARGET_BASE, 8, &args![]).u32();
    if e.call(0x008d_8520, &args![this]).u32() != 0 {
        while effect != 0 && !e.call(LIST_IS_EMPTY, &args![effect]).bool() {
            let next = e.call(LIST_NEXT, &args![effect]).u32();
            let item = e.call(LIST_ITEM_ADDRESS, &args![effect]).u32();
            let item = e.mem.u32(item);
            if item != 0 && e.vcall(item, 0x3c, &args![]).bool() {
                show_message(e, 0x011d_255c, FAST_TRAVEL_ICON);
                return false;
            }
            effect = next;
        }
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if !fn_0093db40(e, Ptr::new(player)) {
        show_message(e, 0x011d_3630, FAST_TRAVEL_ICON);
        return false;
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if e.vcall(player, 0x358, &args![]).bool() {
        let delay = e.with_stack(4, |e, slot| {
            e.mem.set_f32(slot.addr(), 0.0);
            e.call(0x005e_58f0, &args![0x33u32, this, slot]);
            e.mem.f32(slot.addr())
        });
        let one: f64 = e.global(0x0101_2070);
        if (delay as f64) < one {
            show_message(e, 0x011d_50c8, 0);
            return false;
        }
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    let owner = player + ACTOR_VALUE_OWNER_BASE;
    if e.vcall(owner, 8, &args![0x16u32]).i32() < 1 {
        show_message(e, 0x011d_255c, FAST_TRAVEL_ICON);
        return false;
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if e.vcall(player + ACTOR_VALUE_OWNER_BASE, 8, &args![0x10u32])
        .i32()
        < 1
    {
        show_message(e, 0x011d_255c, FAST_TRAVEL_ICON);
        return false;
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if e.call(0x0088_49c0, &args![player]).bool() {
        show_message(e, 0x011d_4120, FAST_TRAVEL_ICON);
        return false;
    }
    let mut allowed = false;
    if e.call(0x008d_6f30, &args![this]).u32() != 0 {
        let parent = e.call(0x008d_6f30, &args![this]).u32();
        allowed = !e.call(0x0054_4520, &args![parent]).bool();
    } else {
        let world_space = e.call(0x0057_5d70, &args![this]).u32();
        if world_space != 0 {
            allowed = !e.call(0x0058_6210, &args![world_space]).bool();
        }
    }
    if !allowed {
        show_message(e, 0x011d_4738, FAST_TRAVEL_ICON);
    }
    allowed
}

/// The sad vault-boy icon path (a string in the exe).
const FAST_TRAVEL_ICON: u32 = 0x0102_08a0;
/// Byte global set while the process lists are being updated.
const PROCESS_LISTS_BUSY: u32 = 0x011e_0e34;

/// Prints the text of the setting object `setting` as a message with `icon`
/// (`007052f0`: text, sound 0, icon, 0, duration 2.0, 0).
fn show_message(e: &mut Engine, setting: u32, icon: u32) {
    e.with_stack(0x1f4, |e, text| {
        let format = e.call(0x0040_3df0, &args![setting]).u32();
        e.call(0x0040_6d00, &args![text, 0x1f4u32, format]);
        let duration: f32 = e.global(TWO_FLOAT);
        e.call(0x0070_52f0, &args![text, 0u32, icon, 0u32, duration, 0u32]);
    });
}

// Translated from 0093db40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit `0x1` of the byte at `+0x66D`.
pub fn fn_0093db40(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    e.mem.u8(this.addr().wrapping_add(0x66d)) & 1 != 0
}

// Translated from 0093db60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::CenterOnCell` (Xbox PDB): places the player at the
/// centre of a cell: `cell`, or the one named `editor_id` (looked up in the
/// loaded cells, or else among the exterior cells in the data files, which are
/// loaded after clearing the grid). The position is the cell's
/// `COC` placement (`0054cfd0`); [`player_character_position_player`] puts the
/// player there and, in an exterior, the height is then set to the land height
/// at the position (0 if negative).
pub fn player_character_center_on_cell(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    editor_id: u32,
    cell: Ptr,
) {
    e.call(0x0040_fbe0, &args![this]);
    let handler = e.global::<u32>(DATA_HANDLER);
    let mut cell = cell.addr();
    e.with_stack(0x30, |e, frame| {
        let position = frame.addr();
        let rotation = position + 0x0c;
        let coordinates = position + 0x18;
        e.call(LIST_ITEM_ADDRESS, &args![position]);
        if cell == 0 {
            cell = e.call(0x0046_1ae0, &args![handler, editor_id]).u32();
        }
        let loader = e.global::<u32>(EXTERIOR_CELL_LOADER);
        if loader != 0 {
            e.call(0x0052_8540, &args![loader]);
        }
        if cell == 0 {
            e.mem.set_u32(coordinates, 0);
            e.mem.set_u32(coordinates + 4, 0);
            let world_space = e
                .call(
                    0x0046_1cf0,
                    &args![handler, editor_id, coordinates, coordinates + 4],
                )
                .u32();
            if world_space != 0 {
                player_character_clear_cells_if_target_not_loaded(
                    e,
                    this,
                    Ptr::NULL,
                    Ptr::new(world_space),
                );
                let x = e.mem.u32(coordinates);
                let y = e.mem.u32(coordinates + 4);
                cell = e.call(0x0058_5b30, &args![world_space, x, y]).u32();
            }
        } else if !e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
            let world_space = e.call(0x0054_ddd0, &args![cell]).u32();
            if world_space != 0 {
                player_character_clear_cells_if_target_not_loaded(
                    e,
                    this,
                    Ptr::new(cell),
                    Ptr::new(world_space),
                );
                let y = e.call(0x0054_4c60, &args![cell]).u32();
                let x = e.call(0x0054_4c30, &args![cell]).u32();
                e.call(0x0058_5b30, &args![world_space, x, y]);
            }
        }
        if cell != 0 {
            let up = read_words(e, ZERO_VECTOR);
            for (i, word) in up.iter().enumerate() {
                e.mem.set_u32(rotation + 4 * i as u32, *word);
            }
            e.call(0x0054_cfd0, &args![cell, position, rotation]);
            let p = read_floats(e, position);
            let r = read_floats(e, rotation);
            player_character_position_player(
                e,
                this,
                p[0],
                p[1],
                p[2],
                r[0],
                r[1],
                r[2],
                Ptr::new(cell),
                true,
            );
            if !e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
                let height = e.with_stack(4, |e, slot| {
                    let tes = e.global::<u32>(TES_SINGLETON);
                    e.call(0x0045_72e0, &args![tes, position, slot]);
                    e.mem.f32(slot.addr())
                });
                let z = if height >= 0.0 { height } else { 0.0 };
                e.mem.set_f32(position + 8, z);
                e.vcall(this.addr(), 0x2a8, &args![position]);
            }
        }
    });
}

// Translated from 0093dd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the virtual at `+0xC` of the player's `ActorMover` (`pActorMover`)
/// with `(008846e0(this) & 0xCC00) | 0x400`.
pub fn fn_0093dd20(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let flags = e.call(0x0088_46e0, &args![this]).u16();
    let value = (flags & 0xcc00) | 0x400;
    let mover = e
        .get(
            this.cast::<crate::units::fallout_ai::actor::Actor>(),
            crate::units::fallout_ai::actor::Actor::pActorMover,
        )
        .addr();
    e.vcall(mover, 0xc, &args![value as u32]);
}

// Translated from 0093dd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// For every perceived actor (`pListofPercievedActors`, entries of an actor and
/// a byte) whose byte is set, makes a temp effect object (`0081f580`, 0x6C
/// bytes, from the actor, the object `007b8f10` finds in the data handler and
/// -1.0) and adds it to the process lists when its virtual `+0xC4` accepts
/// it, deleting it otherwise. Nothing happens without the handler's object.
/// The SEH frame is not translated.
pub fn fn_0093dd80(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let mut node = e.get(this, PlayerCharacter::pListofPercievedActors).addr();
    let handler = e.global::<u32>(DATA_HANDLER);
    let target = e.call(0x007b_8f10, &args![handler]).u32();
    if target == 0 {
        return;
    }
    while node != 0 {
        let item = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
        let entry = e.mem.u32(item);
        if e.mem.u8(entry + 4) != 0 {
            add_temp_effect(e, e.mem.u32(entry), target);
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
}

/// Creates a temp effect (`0081f580`) for `actor` against `target` with the
/// -1.0 strength, adds it to the process lists when its virtual `+0xC4`
/// accepts it and deletes it otherwise.
fn add_temp_effect(e: &mut Engine, actor: u32, target: u32) {
    let block = e.call(0x00aa_13e0, &args![0x6cu32]).u32();
    let effect = if block != 0 {
        let strength: f32 = e.global(TIMER_RESET);
        e.call(0x0081_f580, &args![block, actor, target, strength])
            .u32()
    } else {
        0
    };
    if e.vcall(effect, 0xc4, &args![]).bool() {
        e.call(0x0097_3fd0, &args![PROCESS_LISTS, effect]);
    } else if effect != 0 {
        e.vcall(effect, 0, &args![1u32]);
    }
}

// Translated from 0093dee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::IsTargetPerceivedAndHostile` (Xbox PDB): looks `actor` up
/// in the perceived actors (`pListofPercievedActors`) and returns the byte of
/// its entry (false when absent).
pub fn player_character_is_target_perceived_and_hostile(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor: u32,
) -> bool {
    let mut node = e.get(this, PlayerCharacter::pListofPercievedActors).addr();
    while node != 0 {
        let item = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
        let entry = e.mem.u32(item);
        if e.mem.u32(entry) == actor {
            return e.mem.u8(entry + 4) != 0;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 0093df50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Spawns temp effects (see [`add_temp_effect`]) for the actors four
/// iterators yield (`005271f0` with the kinds `0x28`, `0x29`, `0x2F` and `0x2E`
/// for `this`, walked with `005273c0`), against the object
/// `0093e4f0` finds in the data handler, with the strength of the setting
/// `011d0658`. Nothing is spawned without that object. The SEH frame is not
/// translated.
pub fn fn_0093df50(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    e.with_stack(0x120, |e, frame| {
        let kinds = [0x28u32, 0x29, 0x2f, 0x2e];
        let iterators: Vec<u32> = (0..4).map(|i| frame.addr() + 0x40 * i).collect();
        for (iterator, kind) in iterators.iter().zip(kinds) {
            e.call(0x0052_71f0, &args![*iterator, kind, this]);
        }
        let strength = e.call(SETTING_FLOAT_GETTER, &args![0x011d_0658u32]).u32();
        let strength = e.mem.f32(strength);
        let handler = e.global::<u32>(DATA_HANDLER);
        let target = fn_0093e4f0(e, Ptr::new(handler));
        let cursor = frame.addr() + 0x100;
        let state = frame.addr() + 0x108;
        e.mem.set_u32(cursor, 0);
        e.mem.set_u32(state, 0);
        if target == 0 {
            for iterator in iterators.iter().rev() {
                fn_0093e510(e, Ptr::new(*iterator));
            }
            return;
        }
        let list = e.global::<u32>(0x011c_95c8);
        for iterator in &iterators {
            while e
                .call(0x0052_73c0, &args![list, *iterator, cursor, state, 3u32])
                .bool()
            {
                let actor = e.mem.u32(cursor);
                let block = e.call(0x00aa_13e0, &args![0x6cu32]).u32();
                let effect = if block != 0 {
                    e.call(0x0081_f580, &args![block, actor, target, strength])
                        .u32()
                } else {
                    0
                };
                if e.vcall(effect, 0xc4, &args![]).bool() {
                    e.call(0x0097_3fd0, &args![PROCESS_LISTS, effect]);
                } else if effect != 0 {
                    e.vcall(effect, 0, &args![1u32]);
                }
            }
        }
        for iterator in iterators.iter().rev() {
            fn_0093e510(e, Ptr::new(*iterator));
        }
    });
}

// Translated from 0093e4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0x634` of the object (for the player this is
/// `spGrabSpring`'s pointer; the data handler singleton is also passed).
pub fn fn_0093e4f0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x634))
}

// Translated from 0093e510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `005272b0` on the member at `+0x1C` of the object.
pub fn fn_0093e510(e: &mut Engine, this: Ptr) {
    e.call(0x0052_72b0, &args![this.addr().wrapping_add(0x1c)]);
}

// Translated from 0093e530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetSlowMoCamera` (Xbox PDB): with a kill camera setting
/// (`eKillCameraSetting`), plays the "UIPopUpQuestNew" sound and, unless the
/// player's parent cell is the form `0x161E98` or `mode` is 0 or the VATS
/// object `0044ddc0` says so, starts the slow motion: `time` goes to
/// `fTimeInSlowMoCam` (only if the cool-down `fKillCamCooldown` is not
/// positive when `queued` is set, which also sets the cool-down from the
/// setting `011d0a54`; with `queued` set and a positive cool-down nothing
/// happens). For setting 2 the third-person flag is recorded in `011f21d1`
/// and the VATS object is told (`009ca2c0`) and the gun scope hidden when the
/// player is aiming; for other settings the time is scaled by 0.4. Unless the
/// player has the `ActorValueOwner` value `0x33`, `0093e750` runs. The SEH
/// frame is not translated.
pub fn player_character_set_slow_mo_camera(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    mode: u32,
    time: f32,
    queued: bool,
    extra: u32,
) {
    if e.get(this, PlayerCharacter::eKillCameraSetting) == 0 {
        return;
    }
    if e.call(0x008d_6f30, &args![this]).u32() != 0 {
        let parent = e.call(0x008d_6f30, &args![this]).u32();
        if e.call(0x0084_e3a0, &args![parent]).u32() == 0x0016_1e98 {
            return;
        }
    }
    e.with_stack(24, |e, frame| {
        let handle = frame.addr();
        let temporary = handle + 12;
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let found = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_NAME,
                &args![audio, temporary, 0x0106_f370u32, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, found]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![temporary]);
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        if mode == 0 || e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() != 0 {
            e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
            return;
        }
        let zero: f64 = e.global(ZERO_DOUBLE);
        let cooldown = e.get(this, PlayerCharacter::fKillCamCooldown);
        if queued && (cooldown as f64) <= zero {
            let setting = e.call(SETTING_FLOAT_GETTER, &args![0x011d_0a54u32]).u32();
            let value = e.mem.f32(setting);
            e.set(this, PlayerCharacter::fKillCamCooldown, value);
        } else if queued {
            e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
            return;
        }
        e.set(this, PlayerCharacter::fTimeInSlowMoCam, time);
        if e.get(this, PlayerCharacter::eKillCameraSetting) == 2 {
            let flag = if e.get(this, PlayerCharacter::b3rdPersonSaved) {
                !e.get(this, PlayerCharacter::bsave3rdPerson)
            } else {
                let player = e.global::<u32>(PLAYER_SINGLETON);
                !e.call(0x004e_af60, &args![player]).bool()
            };
            e.set_global::<u8>(0x011f_21d1, flag as u8);
            e.call(0x009c_a2c0, &args![VATS_OBJECT, mode, extra]);
            if e.call(0x008b_bc10, &args![this]).bool() {
                e.call(0x0070_9c40, &args![0u32]);
            }
        } else {
            let scale: f64 = e.global(0x0102_90c0);
            let current = e.get(this, PlayerCharacter::fTimeInSlowMoCam);
            e.set(
                this,
                PlayerCharacter::fTimeInSlowMoCam,
                (current as f64 * scale) as f32,
            );
        }
        let player = e.global::<u32>(PLAYER_SINGLETON);
        if e.vcall(player + ACTOR_VALUE_OWNER_BASE, 8, &args![0x33u32])
            .i32()
            == 0
        {
            fn_0093e750(e);
        }
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

// Translated from 0093e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the float at `011ac3a4` to `011ac3a8`.
pub fn fn_0093e750(e: &mut Engine) {
    let value: f32 = e.global(0x011a_c3a4);
    e.set_global(0x011a_c3a8, value);
}

// Translated from 0093e770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ends the slow-motion camera: restores the global time multiplier
/// ([`fn_0093e840`] on the game timer), and resets the first-/third-person
/// animation (`GetAnimation` for the opposite of `004eaf60`, whose animation
/// is told `1.0` by `0088b030`). For `mode` 2 (unless `flag` is clear and
/// `004a4040` holds) the VATS playback is quit and its data cleared, the VATS
/// mode set, and `011f21d0` set. `fTimeInSlowMoCam` is reset to 0.
pub fn fn_0093e770(e: &mut Engine, this: Ptr<PlayerCharacter>, mode: i32, flag: bool) {
    fn_0093e840(e, Ptr::new(GAME_TIMER));
    let third_person = e.call(0x004e_af60, &args![this]).bool();
    if e.call(0x0095_0a60, &args![this, !third_person]).u32() != 0 {
        let third_person = e.call(0x004e_af60, &args![this]).bool();
        let animation = e.call(0x0095_0a60, &args![this, !third_person]).u32();
        e.call(0x0088_b030, &args![animation, 1.0f32]);
    }
    if mode == 2 {
        if !flag && e.call(0x004a_4040, &args![]).bool() {
            e.set(this, PlayerCharacter::fTimeInSlowMoCam, 0.0);
            return;
        }
        let player = e.global::<u32>(PLAYER_SINGLETON);
        e.call(0x0095_f530, &args![player, 0u32, 3u32]);
        e.call(0x009c_8950, &args![VATS_OBJECT, 0u32, 1u32]);
        e.call(0x009c_6ba0, &args![VATS_OBJECT]);
        e.call(0x009c_6c30, &args![VATS_OBJECT, 0u32, 1u32]);
        e.set_global::<u8>(0x011f_21d0, 1);
    }
    e.set(this, PlayerCharacter::fTimeInSlowMoCam, 0.0);
}

// Translated from 0093e840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00aa4db0` on the object (the game timer) with the float at
/// `011ac3a8` and 0 (`BSTimer::SetGlobalTimeMultiplier`).
pub fn fn_0093e840(e: &mut Engine, this: Ptr) {
    let multiplier: f32 = e.global(0x011a_c3a8);
    e.call(0x00aa_4db0, &args![this, multiplier, 0u32]);
}

/// Size of the frame block `Update` keeps its address-taken locals in, and
/// the distance from its start to the game's frame base (locals are addressed
/// with negative offsets from that base).
const UPDATE_FRAME_SIZE: u32 = 0x600;
const UPDATE_FRAME_LOCALS: u32 = 0x560;

/// The address of the local at the (negative) frame offset `offset`.
fn local(frame: u32, offset: i32) -> u32 {
    frame
        .wrapping_add(UPDATE_FRAME_LOCALS)
        .wrapping_add(offset as u32)
}

const COUNTDOWN_TIMER: u32 = 0x011e_07e8;
const VATS_ENDED_FLAG: u32 = 0x011f_21d0;
const VATS_ENDED_THIRD_PERSON: u32 = 0x011f_21d1;
const UPDATE_GUARD_FILE: u32 = 0x0108_a8e0;
const FORM_ID_8CD8C: u32 = 0x0008_cd8c;
const EFFECT_PARAMETER: u32 = 0x0101_7868;
const STATE_FLAG_011E07AA: u32 = 0x011e_07aa;
const STATE_FLAG_011E07B8: u32 = 0x011e_07b8;
const STATE_FLAG_011E07B9: u32 = 0x011e_07b9;
const STATE_FLAG_011E07C3: u32 = 0x011e_07c3;
const MOVE_SPEED_011A3B3C: u32 = 0x011a_3b3c;
const LOOK_SPEED_011A3B40: u32 = 0x011a_3b40;
const CAMERA_VALUE_011E07BC: u32 = 0x011e_07bc;
const CAMERA_VALUE_011E07C4: u32 = 0x011e_07c4;
const CAMERA_VALUE_011E0B5C: u32 = 0x011e_0b5c;
const HELD_TIMER_011E07E0: u32 = 0x011e_07e0;
const HELD_FLAG_011E0BC1: u32 = 0x011e_0bc1;
const HOLD_VALUE_011E07AC: u32 = 0x011e_07ac;
const HOLD_VALUE_011E07B0: u32 = 0x011e_07b0;
const ORBIT_SCALE: u32 = 0x0102_3128;
const ORBIT_VALUE_011E08FC: u32 = 0x011e_08fc;
const ORBIT_VALUE_011E08F4: u32 = 0x011e_08f4;
const ORBIT_ANGLE_011E0BB8: u32 = 0x011e_0bb8;
const IDLE_OBJECT_011E09E0: u32 = 0x011e_09e0;
const MARKER_OBJECT_011E07EC: u32 = 0x011e_07ec;
const MARKER_COUNT_011E0BB4: u32 = 0x011e_0bb4;
const MARKER_OFFSET_XY: u32 = 0x0102_40c0;
const MARKER_OFFSET_Z: u32 = 0x0102_e430;

const ACTIVATE_INTERVAL_SETTING: u32 = 0x011c_d5c8;
const ACTIVATE_TIMER_011E0BC4: u32 = 0x011e_0bc4;
const ALERT_TIME_011E0BA8: u32 = 0x011e_0ba8;
const AMMO_SWAP_SETTING: u32 = 0x011c_d158;
const AUDIO_LEVEL_LIMIT: u32 = 0x0106_d508;
const AXIS_DIVISOR: u32 = 0x0101_7a40;
const CAMERA_FLAG_011A3B31: u32 = 0x011a_3b31;
const CAMERA_SETTING_011CD62C: u32 = 0x011c_d62c;
const CAMERA_SETTING_011CDE8C: u32 = 0x011c_de8c;
const CAMERA_TIMER_011E07C8: u32 = 0x011e_07c8;
const CAMERA_VALUE_011E0B58: u32 = 0x011e_0b58;
const CAMERA_VALUE_011E0B60: u32 = 0x011e_0b60;
const CAMERA_VALUE_011E0BBC: u32 = 0x011e_0bbc;
const CELL_CHANGE_STRING: u32 = 0x0108_b028;
const CELL_MANAGER_011C3B3C: u32 = 0x011c_3b3c;
const CHASE_SETTING: u32 = 0x011d_0b08;
const CLEARED_EACH_FRAME_011E077C: u32 = 0x011e_077c;
const COMBAT_SETTING_011CE3A8: u32 = 0x011c_e3a8;
const CONTROL_COUNTER_011E0798: u32 = 0x011e_0798;
const CONTROL_COUNTER_LIMIT: u32 = 0x0101_5a38;
const CROUCH_DOWN_SOUND_NAME: u32 = 0x0108_b050;
const CROUCH_UP_SOUND_NAME: u32 = 0x0108_b064;
const FACE_FACTOR_011E0BAC: u32 = 0x011e_0bac;
const FACE_RELAX_01016248: u32 = 0x0101_6248;
const FLAG_SETTING_011E0B64: u32 = 0x011e_0b64;
const HELD_SETTING_011CDFCC: u32 = 0x011c_dfcc;
const HELLO_SCALE: u32 = 0x0101_7b70;
const HELLO_SETTING_011D03A0: u32 = 0x011d_03a0;
const IDLE_SETTING_011CCF50: u32 = 0x011c_cf50;
const INTERACT_DISTANCE_01084D20: u32 = 0x0108_4d20;
const INT_SETTING_011CDDE0: u32 = 0x011c_dde0;
const ITEM_CHECK_INTERVAL_SETTING: u32 = 0x011d_0658;
const KEY_AXIS_LIMIT: u32 = 0x0106_6808;
const LOOK_SCALE: u32 = 0x0101_6ff0;
const MESSAGE_SETTING_011D20AC: u32 = 0x011d_20ac;
const MESSAGE_SETTING_011D28B0: u32 = 0x011d_28b0;
const MESSAGE_SETTING_011D2E50: u32 = 0x011d_2e50;
const MESSAGE_SETTING_011D3E20: u32 = 0x011d_3e20;
const MESSAGE_SETTING_011D400C: u32 = 0x011d_400c;
const MESSAGE_SETTING_011D4474: u32 = 0x011d_4474;
const MESSAGE_SETTING_011D4528: u32 = 0x011d_4528;
const MESSAGE_SETTING_011D47D4: u32 = 0x011d_47d4;
const MESSAGE_SETTING_011D4948: u32 = 0x011d_4948;
const MESSAGE_SETTING_011D4B10: u32 = 0x011d_4b10;
const MESSAGE_SETTING_011D4B40: u32 = 0x011d_4b40;
const MESSAGE_SHOWN_011E07C0: u32 = 0x011e_07c0;
const ONE_DOUBLE: u32 = 0x0101_2070;
const ORBIT_SETTING_011CD198: u32 = 0x011c_d198;
const ORBIT_SETTING_011CD4CC: u32 = 0x011c_d4cc;
const ORBIT_SETTING_011CD984: u32 = 0x011c_d984;
const PENDING_OBJECT_011E0788: u32 = 0x011e_0788;
const QUICK_MENU_SOUND_NAME: u32 = 0x0107_7d84;
const QUICK_STATE_011E0780: u32 = 0x011e_0780;
const RUN_WEIGHT_SETTING: u32 = 0x011d_097c;
const SAVED_VIEW_011E0BC0: u32 = 0x011e_0bc0;
const SLOW_MOTION_ANIMATION_SPEED: u32 = 0x0108_b084;
const SLOW_MOTION_TIME_SCALE: u32 = 0x0101_7870;
const SORT_ACTOR_DISTANCE_SETTING: u32 = 0x011c_d8f0;
const STATE_FLAG_011E07C1: u32 = 0x011e_07c1;
const STATIC_FLAGS_011E0BC8: u32 = 0x011e_0bc8;
const STICK_FACTOR: u32 = 0x0108_b044;
const STICK_REMAP_A: u32 = 0x0101_8238;
const STICK_REMAP_C: u32 = 0x0103_0ff0;
const STICK_REMAP_D: u32 = 0x0101_e2bc;
const STICK_REMAP_E: u32 = 0x0105_3154;
const STICK_SCALE_SETTING: u32 = 0x011e_0b94;
const STICK_THRESHOLD_HIGH: u32 = 0x0107_d1b8;
const STICK_THRESHOLD_LOW: u32 = 0x0102_17d8;
const TIMER_011A3B34: u32 = 0x011a_3b34;
const TRACKED_CELL_011E0BB0: u32 = 0x011e_0bb0;
const TURBO_MODIFIER_NAME: u32 = 0x0108_b078;
const TURBO_TIME_SCALE_SETTING: u32 = 0x011d_0fa8;
const VATS_AUDIO_SETTING: u32 = 0x011c_a7c0;
const VATS_SOUND_HANDLE: u32 = 0x011f_6df0;
const VATS_SOUND_NAME: u32 = 0x0108_b088;
const WALK_SPEED_SETTING: u32 = 0x011e_0b78;
const YIELD_SETTING_011CEDB8: u32 = 0x011c_edb8;
/// `Actor::pCurrentProcess` of the object.
fn process_of(e: &Engine, this: Ptr<PlayerCharacter>) -> u32 {
    e.get(this.cast::<Actor>(), Actor::pCurrentProcess).addr()
}

/// `pCurrentProcess` of the player singleton.
fn singleton_process(e: &Engine) -> u32 {
    let player = e.global::<u32>(PLAYER_SINGLETON);
    e.get(Ptr::<Actor>::new(player), Actor::pCurrentProcess)
        .addr()
}

/// Sets bits in the 16-bit word at `slot`.
fn or_word(e: &mut Engine, slot: u32, bits: u16) {
    let value = e.mem.u16(slot);
    e.mem.set_u16(slot, value | bits);
}

/// Keeps only the bits of `mask` in the 16-bit word at `slot`.
fn and_word(e: &mut Engine, slot: u32, mask: u16) {
    let value = e.mem.u16(slot);
    e.mem.set_u16(slot, value & mask);
}

/// Sets the animation speed `strength` on the animation selected by the
/// player's third-person flag (or, with `opposite`, by its negation).
fn refresh_animation_speed(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    strength: f32,
    opposite: bool,
) {
    let flag = e.call(0x004e_af60, &args![this]).bool();
    let animation = e
        .call(
            0x0095_0a60,
            &args![this, if opposite { !flag } else { flag }],
        )
        .u32();
    e.call(0x0088_b030, &args![animation, strength]);
}

/// Prints the text of the setting object `setting` as a message with `icon`.
fn show_text(e: &mut Engine, setting: u32, icon: u32) {
    let text = e.call(0x0040_3df0, &args![setting]).u32();
    let duration: f32 = 2.0;
    e.call(0x0070_52f0, &args![text, 0u32, icon, 0u32, duration, 0u32]);
}

// Translated from 0093e860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::Update` (Xbox PDB): the per-frame update of the player
/// with the frame time `delta`. In order: the VATS and slow-motion
/// bookkeeping and sounds, cool-down timers, the queued weapon attach, the
/// warning timers, a forced activation (which ends the update), the process
/// flags, the view switch (which also ends the update), the controlled player
/// (which ends the update after the activation checks), the input handling
/// (crouch toggle, movement from keys or stick, activation, menus), the
/// camera orbit and idle timers, then the combat, ammo, item-alert and hello
/// timers. The scope guard and the sound handle the function keeps are
/// released at every exit. The SEH frame is not translated; the block at
/// `009411e9..0094134c`, which only runs without a gamepad while the axis
/// values are still 0, is left out.
pub fn player_character_update(e: &mut Engine, this: Ptr<PlayerCharacter>, delta: f32) {
    e.with_stack(UPDATE_FRAME_SIZE, |e, frame| {
        update_in_frame(e, this, delta, frame.addr());
    });
}

#[allow(clippy::cognitive_complexity, clippy::if_same_then_else)]
fn update_in_frame(e: &mut Engine, this: Ptr<PlayerCharacter>, delta: f32, frame: u32) {
    let guard = local(frame, -0x50);
    let handle = local(frame, -0x78);
    'update: {
        // ---- 0093e860..0093ef51: VATS/slow-motion bookkeeping, timers ----
        let main = e.global::<u32>(MAIN_SINGLETON);
        let main_state = e.call(0x0087_7720, &args![main]).u32(); // -0x1c
        let zero: f64 = e.global(ZERO_DOUBLE);
        let countdown: f32 = e.global(COUNTDOWN_TIMER);
        e.set_global(COUNTDOWN_TIMER, (countdown as f64 - delta as f64) as f32);
        if e.global::<u8>(VATS_ENDED_FLAG) != 0 {
            let player = e.global::<u32>(PLAYER_SINGLETON);
            let flag = e.global::<u8>(VATS_ENDED_THIRD_PERSON) != 0;
            e.call(0x0095_0110, &args![player, flag as u32]);
            e.set_global::<u8>(VATS_ENDED_FLAG, 0);
        }
        let in_vats = e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() != 0; // -0x20
        let level_limit: f64 = e.global(AUDIO_LEVEL_LIMIT);
        if !in_vats {
            e.call(AUDIO_INSTANCE, &args![]);
            if e.call(SOUND_HANDLE_IS_VALID, &args![VATS_SOUND_HANDLE])
                .bool()
            {
                e.call(AUDIO_INSTANCE, &args![]);
                e.call(
                    SOUND_HANDLE_FADE_OUT_AND_RELEASE,
                    &args![VATS_SOUND_HANDLE, 0x5dcu32],
                );
            }
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let level = e.call(0x00ad_7c80, &args![audio]).f64();
            if level < level_limit {
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                e.call(0x00ad_7ec0, &args![audio]);
            }
        } else {
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let level = e.call(0x00ad_7c80, &args![audio]).f64();
            if level > level_limit && (setting_float(e, VATS_AUDIO_SETTING) as f64) < level_limit {
                let volume = setting_float(e, VATS_AUDIO_SETTING);
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                e.call(0x00ad_7da0, &args![audio, volume]);
                if !e
                    .call(SOUND_HANDLE_IS_VALID, &args![VATS_SOUND_HANDLE])
                    .bool()
                {
                    e.with_stack(12, |e, found| {
                        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                        let result = e
                            .call(
                                AUDIO_GET_SOUND_HANDLE_BY_NAME,
                                &args![audio, found, VATS_SOUND_NAME, 0x31u32],
                            )
                            .u32();
                        e.call(SOUND_HANDLE_ASSIGN, &args![VATS_SOUND_HANDLE, result]);
                        e.call(SOUND_HANDLE_DESTRUCT, &args![found]);
                    });
                }
                e.call(SOUND_HANDLE_PLAY, &args![VATS_SOUND_HANDLE, 0u32]);
            }
        }
        let cooldown = e.get(this, PlayerCharacter::fKillCamCooldown);
        if cooldown as f64 > zero {
            e.set(
                this,
                PlayerCharacter::fKillCamCooldown,
                (cooldown as f64 - delta as f64) as f32,
            );
        }
        let mut slow_mo_running = false; // -0x35
        let slow_mo_time = e.get(this, PlayerCharacter::fTimeInSlowMoCam);
        if slow_mo_time as f64 > zero {
            if e.call(0x00a2_4660, &args![main_state, 5u32, 1u32]).u32() != 0 {
                e.set(this, PlayerCharacter::fTimeInSlowMoCam, 0.0);
            }
            let remaining = e.get(this, PlayerCharacter::fTimeInSlowMoCam);
            e.set(
                this,
                PlayerCharacter::fTimeInSlowMoCam,
                (remaining as f64 - delta as f64) as f32,
            );
            let slow_scale: f32 = e.global(SLOW_MOTION_TIME_SCALE);
            e.call(0x00aa_4db0, &args![GAME_TIMER, slow_scale, 0u32]);
            if e.get(this, PlayerCharacter::bIgnoresGTM) {
                let strength: f32 = e.global(SLOW_MOTION_ANIMATION_SPEED);
                refresh_animation_speed(e, this, strength, true);
                let strength: f32 = e.global(SLOW_MOTION_ANIMATION_SPEED);
                refresh_animation_speed(e, this, strength, false);
            }
            let remaining = e.get(this, PlayerCharacter::fTimeInSlowMoCam);
            if remaining as f64 > zero || remaining.is_nan() {
                slow_mo_running = true;
            } else {
                let setting = e.get(this, PlayerCharacter::eKillCameraSetting) as i32;
                fn_0093e770(e, this, setting, false);
            }
        } else {
            refresh_animation_speed(e, this, 1.0, true);
            refresh_animation_speed(e, this, 1.0, false);
        }
        let counter_timer = e.call(0x0094_43a0, &args![this]).f64();
        if counter_timer > zero {
            let counter_timer = e.call(0x0094_43a0, &args![this]).f64();
            e.call(
                0x0094_4380,
                &args![this, (counter_timer - delta as f64) as f32],
            );
        }
        let has_perk = e
            .vcall(this.addr() + ACTOR_VALUE_OWNER_BASE, 8, &args![0x33u32])
            .i32();
        if has_perk > 0 && !slow_mo_running {
            if !e.get(this, PlayerCharacter::bTurboISM) {
                fn_0093e750(e);
                let modifier = e.call(0x0048_3a00, &args![TURBO_MODIFIER_NAME]).u32();
                e.call(0x0052_99a0, &args![modifier, 1.0f32, 0u32]);
                e.set(this, PlayerCharacter::bTurboISM, true);
            }
            let scale = setting_float(e, TURBO_TIME_SCALE_SETTING);
            e.call(0x00aa_4db0, &args![GAME_TIMER, scale, 0u32]);
            let ratio = e.call(0x0071_6440, &args![]).f64();
            let speed = (1.0 / ratio) as f32;
            refresh_animation_speed(e, this, speed, true);
            refresh_animation_speed(e, this, speed, false);
        } else if !slow_mo_running {
            let modifier = e.call(0x0048_3a00, &args![TURBO_MODIFIER_NAME]).u32();
            e.call(0x0052_9c90, &args![modifier]);
            if e.get(this, PlayerCharacter::bTurboISM) {
                fn_0093e840(e, Ptr::new(GAME_TIMER));
            }
            e.set(this, PlayerCharacter::bTurboISM, false);
        }
        let attach = e.get(this, PlayerCharacter::pQueuedWeaponAttach);
        if !attach.is_null() {
            let process = e.get(this.cast::<Actor>(), Actor::pCurrentProcess).addr();
            if !e.vcall(process, 0x474, &args![1u32]).bool() {
                let attach = e.get(this, PlayerCharacter::pQueuedWeaponAttach);
                e.call(0x0057_1760, &args![this, attach]);
                e.set(this, PlayerCharacter::pQueuedWeaponAttach, Ptr::NULL);
            }
        }
        if e.get(this, PlayerCharacter::bBlockActivate) && !e.call(0x0094_43c0, &args![main]).bool()
        {
            let timer = e.get(this, PlayerCharacter::fBlockActivateTimer);
            let timer = (timer as f64 - delta as f64) as f32;
            e.set(this, PlayerCharacter::fBlockActivateTimer, timer);
            if (timer as f64) < zero {
                e.set(this, PlayerCharacter::fBlockActivateTimer, 0.0);
                e.set(this, PlayerCharacter::bBlockActivate, false);
            }
        }
        e.set_global(CLEARED_EACH_FRAME_011E077C, 0u32);
        let targeted = e
            .call(
                0x0055_9450,
                &args![member(this, PlayerCharacter::spTargeted3D)],
            )
            .u32();
        let target = e.call(0x0056_f930, &args![targeted]).u32(); // -0x4c
        let distant = e.get(this, PlayerCharacter::bTarget3DDistant);
        e.call(0x0077_2d10, &args![target, distant as u32]);
        e.call(
            NI_POINTER_ASSIGN,
            &args![member(this, PlayerCharacter::spTargeted3D), 0u32],
        );
        e.call(0x008b_bdb0, &args![this]);
        e.call(
            0x0040_4eb0,
            &args![guard, 0x34u32, 1u32, UPDATE_GUARD_FILE, 0xc41u32],
        );
        let root = e.call(0x0045_c670, &args![]).u32(); // -0x54
        let process = e.get(this.cast::<Actor>(), Actor::pCurrentProcess).addr();
        let weapon = e.vcall(process, 0x148, &args![]).u32();
        let weapon_form = if weapon != 0 {
            e.call(0x0044_ddc0, &args![weapon]).u32()
        } else {
            0
        }; // -0x5c
        let animation_third_person = e.call(0x0095_0a60, &args![this, 0u32]).u32(); // -0x60
        let animation_first_person = e.call(0x0095_0a60, &args![this, 1u32]).u32(); // -0x64
        let mut moved_flag = false; // -0x65
        e.mem.set_u16(local(frame, -0x6c), 0);
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
        e.set(this, PlayerCharacter::fDropAngleMod, 0.0);
        e.set(this, PlayerCharacter::fLastDropAngleMod, 0.0);
        let actor_timer = e.get(this.cast::<Actor>(), Actor::fTimeronAction);
        if actor_timer as f64 > zero {
            let elapsed = e.call(0x0084_d030, &args![GAME_TIMER]).f64();
            let actor_timer = e.get(this.cast::<Actor>(), Actor::fTimeronAction);
            e.set(
                this.cast::<Actor>(),
                Actor::fTimeronAction,
                (actor_timer as f64 - elapsed) as f32,
            );
        } else {
            e.set(this.cast::<Actor>(), Actor::iActionValue, 0);
        }
        e.call(0x0096_2d00, &args![this]);
        let process = e.get(this.cast::<Actor>(), Actor::pCurrentProcess).addr();
        e.vcall(process, 0x464, &args![this]);
        if e.call(0x0096_7950, &args![this]).u32() != 0 {
            let item = e.call(0x0096_7950, &args![this]).u32();
            let duration: f32 = e.global(TWO_FLOAT);
            e.call(0x0096_3eb0, &args![this, 7u32, duration, item]);
        }
        if e.call(0x0096_7a00, &args![this]).u32() != 0 {
            let actor_timer = e.get(this.cast::<Actor>(), Actor::fTimeronAction);
            if actor_timer as f64 <= zero {
                let setting = e.call(0x0043_d4d0, &args![INT_SETTING_011CDDE0]).u32();
                let value = e.mem.u32(setting);
                e.call(0x008b_c240, &args![this, value]);
            }
        }
        let cached = e.call(0x008b_bc10, &args![this]).bool(); // -0x79
        let process = e.get(this.cast::<Actor>(), Actor::pCurrentProcess).addr();
        let combat_process_flag = cached && e.vcall(process, 0x454, &args![]).bool();
        if combat_process_flag {
            let tes = e.global::<u32>(TES_SINGLETON);
            let mut dark = false; // -0x7a
            let parent = e.call(0x008d_6f30, &args![this]).u32();
            if e.call(CELL_IS_INTERIOR, &args![parent]).bool() {
                dark = true;
            } else {
                let sky = e.call(0x008d_8520, &args![tes]).u32(); // -0x80
                let first = e.call(0x0096_6a20, &args![sky]).f32();
                let second = e.call(0x0059_5f50, &args![sky]).f32();
                let third = e.call(0x0059_5fc0, &args![sky]).f32();
                // `third <= first`, else `second >= first` (x87 compares).
                if (third as f64) <= first as f64 || (second as f64) >= first as f64 {
                    dark = true;
                }
            }
            if e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() != 0
                || e.get(this, PlayerCharacter::fTimeInSlowMoCam) as f64 > zero
            {
                dark = false;
            }
            let process = e.get(this.cast::<Actor>(), Actor::pCurrentProcess).addr();
            let combat_object = e.vcall(process, 0x148, &args![]).u32(); // -0x90
            let combat_state = if combat_object != 0 {
                e.call(0x0044_ddc0, &args![combat_object]).u32()
            } else {
                0
            }; // -0x98
            let mut applied = false;
            if combat_state != 0
                && e.call(0x0094_42c0, &args![combat_state]).bool()
                && !e.get(this, PlayerCharacter::bNightVisionOn)
                && dark
            {
                let ready = if e.call(0x004a_d030, &args![combat_state]).bool() {
                    e.call(0x004b_da70, &args![combat_object, 0x0eu32]).bool()
                } else {
                    true
                };
                if ready {
                    e.set(this, PlayerCharacter::bNightVisionOn, true);
                    let modifier = e.call(0x0048_39c0, &args![FORM_ID_8CD8C]).u32();
                    e.call(0x0052_99a0, &args![modifier, 1.0f32, 0u32]);
                    applied = true;
                }
            }
            if !applied && !dark {
                let modifier = e.call(0x0048_39c0, &args![FORM_ID_8CD8C]).u32();
                e.call(0x0052_9c90, &args![modifier]);
                e.call(0x0096_3e00, &args![this, 9u32, 0u32]);
                e.set(this, PlayerCharacter::bNightVisionOn, false);
            }
            if e.call(0x0096_4100, &args![this, 9u32]).u32() == 0 {
                let strength: f32 = e.global(EFFECT_PARAMETER);
                e.call(0x0096_3eb0, &args![this, 9u32, strength, 0u32]);
            }
        } else {
            if e.get(this, PlayerCharacter::bNightVisionOn) {
                let modifier = e.call(0x0048_39c0, &args![FORM_ID_8CD8C]).u32();
                e.call(0x0052_9c90, &args![modifier]);
                e.set(this, PlayerCharacter::bNightVisionOn, false);
            }
            e.call(0x0096_3e00, &args![this, 9u32, 0u32]);
        }
        if cached
            && !e.get(this, PlayerCharacter::bHasSpotterActive)
            && e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() != 4
        {
            let delay = e.with_stack(4, |e, slot| {
                e.mem.set_f32(slot.addr(), 0.0);
                let player = e.global::<u32>(PLAYER_SINGLETON);
                e.call(0x005e_58f0, &args![0x46u32, player, slot]);
                e.mem.f32(slot.addr())
            });
            if delay as f64 > zero {
                e.set(this, PlayerCharacter::bHasSpotterActive, true);
                fn_0093dd80(e, this);
            }
        } else if e.get(this, PlayerCharacter::bHasSpotterActive) && !cached {
            let handler = e.global::<u32>(DATA_HANDLER);
            let object = e.call(0x007b_8f10, &args![handler]).u32();
            e.call(0x0097_4af0, &args![PROCESS_LISTS, object]);
            e.set(this, PlayerCharacter::bHasSpotterActive, false);
        }
        let owner = this.addr() + ACTOR_VALUE_OWNER_BASE;
        if e.vcall(owner, 8, &args![0x32u32]).i32() > 0 {
            e.set(this, PlayerCharacter::bHasCateyeActive, true);
        } else if e.get(this, PlayerCharacter::bHasCateyeActive)
            && e.vcall(owner, 8, &args![0x32u32]).i32() <= 0
        {
            let handler = e.global::<u32>(DATA_HANDLER);
            let object = e.call(0x0089_f4e0, &args![handler]).u32();
            e.call(0x0097_4af0, &args![PROCESS_LISTS, object]);
            e.set(this, PlayerCharacter::bHasCateyeActive, false);
        }
        let player = e.global::<u32>(PLAYER_SINGLETON);
        let item_delay = e.with_stack(4, |e, slot| {
            e.mem.set_f32(slot.addr(), 0.0);
            e.call(0x005e_58f0, &args![0x47u32, player, slot]);
            e.mem.f32(slot.addr())
        });
        let check = e.get(this, PlayerCharacter::fCheckForItems);
        let check = (check as f64 - delta as f64) as f32;
        e.set(this, PlayerCharacter::fCheckForItems, check);
        if item_delay as f64 > zero && check as f64 <= zero && cached {
            let interval = setting_float(e, ITEM_CHECK_INTERVAL_SETTING);
            e.set(this, PlayerCharacter::fCheckForItems, interval);
            fn_0093df50(e, this);
        }
        e.call(0x0096_3bf0, &args![this]);
        if e.call(0x004d_1360, &args![this]).bool() {
            e.call(0x0096_9c30, &args![this]);
        }
        let sky_process = e.call(0x008d_8520, &args![this]).u32();
        e.vcall(sky_process, 0x500, &args![this, 0u32]);
        if e.call(0x0096_78a0, &args![this]).bool()
            && e.vcall(this.addr(), 0x214, &args![]).u32() == 0
        {
            if e.call(0x0096_4100, &args![this, 8u32]).u32() == 0 {
                let strength: f32 = e.global(EFFECT_PARAMETER);
                e.call(0x0096_3eb0, &args![this, 8u32, strength, 0u32]);
            }
        } else {
            e.call(0x0096_3e00, &args![this, 8u32, 0u32]);
        }
        let controller = e.call(0x0093_06d0, &args![this]).u32();
        if controller != 0 && e.call(0x0094_4430, &args![controller]).bool() {
            if !e.get(this, PlayerCharacter::bOnElevator) {
                e.call(0x0097_3de0, &args![PROCESS_LISTS]);
                e.set(this, PlayerCharacter::bOnElevator, true);
            }
        } else {
            e.set(this, PlayerCharacter::bOnElevator, false);
        }
        let mover_process = e.call(0x008d_8520, &args![this]).u32();
        let speed_factor = e.vcall(mover_process, 0x65c, &args![]).f64();
        if speed_factor > zero {
            let mover_process = e.call(0x008d_8520, &args![this]).u32();
            e.vcall(mover_process, 0x658, &args![]);
        } else {
            let mover_process = e.call(0x008d_8520, &args![this]).u32();
            e.vcall(mover_process, 0x4a4, &args![0.0f32]);
        }
        if e.get(this, PlayerCharacter::bBeingChased) {
            let value = e.vcall(this.addr(), 0x2b4, &args![]).f64();
            if value <= zero {
                e.call(0x0096_2190, &args![this]);
                let tes_object = e.call(0x008d_8520, &args![this]).u32();
                let amount = setting_float(e, CHASE_SETTING);
                e.vcall(tes_object, 0x2c8, &args![amount]);
            } else {
                e.vcall(this.addr(), 0x2b8, &args![]);
            }
        }
        // The warning timers count down; once they run out they are set to -1.
        let reset: f32 = e.global(TIMER_RESET);
        let steal = e.get(this, PlayerCharacter::fStealWarningTimer);
        if steal as f64 > zero {
            e.set(
                this,
                PlayerCharacter::fStealWarningTimer,
                (steal as f64 - delta as f64) as f32,
            );
        } else {
            e.set(this, PlayerCharacter::fStealWarningTimer, reset);
        }
        let pickpocket = e.get(this, PlayerCharacter::fPickPocketWarningTimer);
        if pickpocket as f64 > zero {
            e.set(
                this,
                PlayerCharacter::fPickPocketWarningTimer,
                (pickpocket as f64 - delta as f64) as f32,
            );
        } else {
            e.set(this, PlayerCharacter::fPickPocketWarningTimer, reset);
        }
        let sort_timer = e.get(this, PlayerCharacter::fSortActorDistanceTimer);
        if sort_timer as f64 > zero || sort_timer.is_nan() {
            e.set(
                this,
                PlayerCharacter::fSortActorDistanceTimer,
                (sort_timer as f64 - delta as f64) as f32,
            );
        } else {
            e.call(0x0096_e570, &args![PROCESS_LISTS]);
            let interval = setting_float(e, SORT_ACTOR_DISTANCE_SETTING);
            e.set(this, PlayerCharacter::fSortActorDistanceTimer, interval);
        }
        if e.get(this, PlayerCharacter::bReturnToLastKnownGoodPosition) {
            e.set(this, PlayerCharacter::bReturnToLastKnownGoodPosition, false);
            e.call(0x0094_dbe0, &args![this, 1u32]);
        }
        let force_activate = e.call(0x0094_4320, &args![this]).u32();
        if force_activate != 0 {
            let force_activate = e.call(0x0094_4320, &args![this]).u32();
            e.call(0x0057_3170, &args![force_activate, this, 0u32, 0u32, 1u32]);
            e.call(0x0051_9020, &args![this, 0u32]);
            break 'update;
        }
        let tes_object = e.call(0x008d_8520, &args![this]).u32();
        e.vcall(tes_object, 0xd4, &args![this]);
        let process = process_of(e, this);
        let process_flags = e.vcall(process, 0x618, &args![]).u32(); // -0xcc
        let process = process_of(e, this);
        e.vcall(process, 0x61c, &args![]);
        if process_flags & 4 != 0 {
            let process = process_of(e, this);
            if !e.vcall(process, 0x2b0, &args![this]).bool() {
                let process = process_of(e, this);
                e.vcall(process, 0x708, &args![0u32]);
            }
        }
        if process_flags & 8 != 0 {
            let process = process_of(e, this);
            if !e.vcall(process, 0x2b8, &args![this]).bool() {
                let process = process_of(e, this);
                e.vcall(process, 0x708, &args![0u32]);
            }
        }
        let state_flags = e.call(0x0088_46e0, &args![this]).u16(); // -0xd0
        let one: f64 = e.global(ONE_DOUBLE);
        if e.call(0x0049_97b0, &args![this]).bool() && e.call(0x0049_38e0, &args![this]).bool() {
            if e.call(0x0070_38a0, &args![]).i32() < 3
                && e.get(this, PlayerCharacter::bActorinSneakRange)
            {
                let sneaking = e.get(this, PlayerCharacter::fsecondSneaking);
                if sneaking as f64 > one {
                    e.vcall(this.addr(), 0x47c, &args![0x2au32, 0u32]);
                    e.set(this, PlayerCharacter::fsecondSneaking, 0.0);
                } else {
                    e.set(
                        this,
                        PlayerCharacter::fsecondSneaking,
                        (sneaking as f64 + delta as f64) as f32,
                    );
                }
            }
        } else if e.call(0x005a_2030, &args![this]).bool()
            && e.call(0x0049_38e0, &args![this]).bool()
        {
            let swimming = e.get(this, PlayerCharacter::fsecondSwimming);
            if swimming as f64 > one {
                e.set(this, PlayerCharacter::fsecondSwimming, 0.0);
            } else {
                e.set(
                    this,
                    PlayerCharacter::fsecondSwimming,
                    (swimming as f64 + delta as f64) as f32,
                );
            }
        } else if state_flags & 0x200 != 0 && e.call(0x0049_38e0, &args![this]).bool() {
            let running = e.get(this, PlayerCharacter::fsecondRunning);
            if running as f64 > one {
                e.set(this, PlayerCharacter::fsecondRunning, 0.0);
            } else {
                e.set(
                    this,
                    PlayerCharacter::fsecondRunning,
                    (running as f64 + delta as f64) as f32,
                );
            }
        }
        let word_slot = local(frame, -0x6c);
        e.mem.set_u16(word_slot, state_flags & 0xcc00);
        let penetration = e
            .get(this.cast::<Actor>(), Actor::pPenetrationDetection)
            .addr();
        e.call(0x00ca_1410, &args![penetration]);
        if e.call(0x0094_45b0, &args![this, delta, 0u32, word_slot, 0u32])
            .bool()
        {
            moved_flag = true;
        }
        let blocked_a =
            e.global::<u8>(STATE_FLAG_011E07B8) != 0 || e.global::<u8>(STATE_FLAG_011E07C1) != 0;
        let view_switch_allowed = if blocked_a {
            let process = process_of(e, this);
            e.vcall(process, 0x610, &args![]).u32() != 0
        } else {
            true
        };
        if view_switch_allowed && {
            let process = process_of(e, this);
            e.call(0x008f_ec10, &args![process, this]).bool()
        } {
            // The player's view is switched by the process; refresh both views.
            if e.call(
                0x0055_9450,
                &args![member(this, PlayerCharacter::spGrabSpring)],
            )
            .u32()
                != 0
            {
                e.call(0x0095_f6a0, &args![this]);
            }
            e.call(0x0094_81d0, &args![this]);
            for _ in 0..2 {
                let toggled = !e.get(this, PlayerCharacter::b3rdPerson);
                e.set(this, PlayerCharacter::b3rdPerson, toggled);
                e.call(0x008d_3550, &args![this, delta]);
                let first_person = !e.get(this, PlayerCharacter::b3rdPerson);
                let animation = e.call(0x0095_0a60, &args![this, first_person]).u32();
                e.call(0x0088_85e0, &args![this, animation, delta]);
            }
            e.call(0x0094_ae40, &args![this, 0u32, 0u32]);
            break 'update;
        }
        let tes_object = e.call(0x008d_8520, &args![this]).u32();
        let mut attack_ok = false; // -0xdd; only written
        if e.vcall(tes_object, 0x6dc, &args![]).bool()
            && !e.call(0x0052_5430, &args![VATS_OBJECT]).bool()
        {
            let process = process_of(e, this);
            if e.vcall(process, 0x148, &args![]).u32() != 0 {
                let kind = e.call(0x008a_7570, &args![this]).u32();
                if (2..=6).contains(&kind) {
                    let process = process_of(e, this);
                    let target = e.vcall(process, 0x148, &args![]).u32();
                    let target_state = e.call(0x0044_ddc0, &args![target]).u32();
                    if target_state != 0 {
                        attack_ok = true;
                        let player = e.global::<u32>(PLAYER_SINGLETON);
                        if e.call(0x0052_5980, &args![target_state, player]).u32() != 0 {
                            let process = process_of(e, this);
                            let list = e.vcall(process, 0x14c, &args![]).u32();
                            if list == 0 {
                                attack_ok = false;
                            } else {
                                let count = e.call(0x0072_6070, &args![list]).i32();
                                let limit = e.call(0x0052_4b60, &args![target_state]).u8() as i32;
                                if count < limit {
                                    attack_ok = false;
                                }
                            }
                        }
                    }
                }
            }
        }
        let _ = attack_ok;
        let process = process_of(e, this);
        if e.vcall(process, 0x40c, &args![]).u32() != 0
            || e.vcall(this.addr(), 0x234, &args![]).bool()
        {
            e.call(0x009c_8950, &args![VATS_OBJECT, 0u32, 0u32]);
            e.set(this, PlayerCharacter::bAiControlledActivate, false);
            e.set_global(PENDING_OBJECT_011E0788, 0u32);
            if !e.get(this, PlayerCharacter::b3rdPerson) {
                e.call(0x0095_0340, &args![this, 1u32]);
            }
        }
        let controlled = fn_0093a740(e, this) || e.vcall(this.addr(), 0x22c, &args![0u32]).bool();
        if controlled {
            if e.get(this, PlayerCharacter::bActually3rdPerson)
                && !e.get(this, PlayerCharacter::b3rdPerson)
                && e.call(0x005a_03f0, &args![this, 0x10u32]).bool()
            {
                e.call(0x0095_1a10, &args![this, 1u32]);
            }
            let mover = e.get(this.cast::<Actor>(), Actor::pActorMover).addr();
            let word_value = e.mem.u16(word_slot) as u32;
            e.vcall(mover, 0xc, &args![word_value]);
            let counter: f32 = e.global(CONTROL_COUNTER_011E0798);
            let counter_limit: f64 = e.global(CONTROL_COUNTER_LIMIT);
            let in_state = e.vcall(this.addr(), 0x214, &args![]).u32();
            if counter as f64 > counter_limit {
                let in_state_again = if in_state != 0 {
                    e.vcall(this.addr(), 0x214, &args![]).u32()
                } else {
                    0
                };
                if in_state == 0 || in_state_again == 4 {
                    if fn_0093a640(e, this) && e.vcall(this.addr(), 0x214, &args![]).u32() == 4 {
                        fn_0093a6f0(e, this, false);
                        let process = process_of(e, this);
                        e.vcall(process, 0x214, &args![]);
                    } else if e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
                        || e.vcall(this.addr(), 0x214, &args![]).u32() == 0
                    {
                        let process = process_of(e, this);
                        e.vcall(process, 0x214, &args![]);
                        fn_0093a5f0(e, this, false);
                        e.set_global(CONTROL_COUNTER_011E0798, 0.0f32);
                    }
                    e.call(0x0088_d640, &args![this]);
                    e.set_global(CONTROL_COUNTER_011E0798, 0.0f32);
                    break 'update;
                }
            }
            if fn_0093a5d0(e, this) {
                let counter: f32 = e.global(CONTROL_COUNTER_011E0798);
                e.set_global(
                    CONTROL_COUNTER_011E0798,
                    (counter as f64 + delta as f64) as f32,
                );
                let process = process_of(e, this);
                let mut reset = e.vcall(process, 0x228, &args![this]).bool();
                if !reset {
                    reset = e.vcall(this.addr(), 0x214, &args![]).u32() == 4;
                }
                if !reset {
                    let process = process_of(e, this);
                    reset = e.vcall(process, 0x4bc, &args![]).u32() == 9;
                }
                if !reset {
                    let process = process_of(e, this);
                    reset = e.vcall(process, 0x27c, &args![]).u32() == 0;
                }
                if reset {
                    fn_0093a5f0(e, this, false);
                    let process = process_of(e, this);
                    e.vcall(process, 0x214, &args![]);
                    e.set_global(CONTROL_COUNTER_011E0798, 0.0f32);
                }
            } else if fn_0093a640(e, this) {
                let counter: f32 = e.global(CONTROL_COUNTER_011E0798);
                e.set_global(
                    CONTROL_COUNTER_011E0798,
                    (counter as f64 + delta as f64) as f32,
                );
                if e.vcall(this.addr(), 0x214, &args![]).u32() == 0 {
                    fn_0093a6f0(e, this, false);
                    let process = process_of(e, this);
                    e.vcall(process, 0x214, &args![]);
                    e.set_global(CONTROL_COUNTER_011E0798, 0.0f32);
                }
            }
            if e.mem.u8(this.addr() + 0x64c) == e.mem.u8(this.addr() + 0x64a) {
                e.call(0x0088_6360, &args![this, delta]);
            }
            let mover = e.get(this.cast::<Actor>(), Actor::pActorMover).addr();
            e.call(0x009e_a570, &args![mover, ZERO_VECTOR]);
            let mover = e.get(this.cast::<Actor>(), Actor::pActorMover).addr();
            e.vcall(mover, 0x14, &args![delta]);
            let chase_scale = setting_float(e, CAMERA_SETTING_011CD62C);
            if chase_scale as f64 > zero {
                let mode = e.call(0x004f_8960, &args![this]).u32();
                if mode == 2 || e.call(0x004f_8960, &args![this]).u32() == 1 {
                    let value: f32 = e.global(TIMER_011A3B34);
                    if (value as f64) < zero {
                        e.call(0x0070_9c40, &args![0u32]);
                        let setting = setting_float(e, CAMERA_SETTING_011CD62C);
                        e.set_global(TIMER_011A3B34, setting);
                    }
                    let value: f32 = e.global(TIMER_011A3B34);
                    let value = (value as f64 - delta as f64) as f32;
                    e.set_global(TIMER_011A3B34, value);
                    if (value as f64) < zero {
                        e.set_global(TIMER_011A3B34, e.global::<f32>(TIMER_RESET));
                        let reactor = e.global::<u32>(FAST_TRAVEL_REACTOR);
                        if !e.call(0x0085_12f0, &args![reactor]).bool() {
                            e.call(0x007d_0a70, &args![]);
                        }
                    }
                } else {
                    e.set_global(TIMER_011A3B34, e.global::<f32>(TIMER_RESET));
                }
            } else if e.call(0x004f_8960, &args![this]).u32() == 2
                && e.global::<u8>(MESSAGE_SHOWN_011E07C0) == 0
            {
                e.vcall(this.addr(), 0x2a0, &args![]);
                e.call(0x0070_9c40, &args![0u32]);
                let reactor = e.global::<u32>(FAST_TRAVEL_REACTOR);
                if e.call(0x0085_1230, &args![reactor]).bool() {
                    let text_a = e.call(0x0040_3df0, &args![MESSAGE_SETTING_011D4528]).u32();
                    let text_b = e.call(0x0040_3df0, &args![MESSAGE_SETTING_011D400C]).u32();
                    let text_c = e.call(0x0040_3df0, &args![MESSAGE_SETTING_011D4B40]).u32();
                    e.call(
                        0x0070_3e80,
                        &args![
                            text_c,
                            0u32,
                            0u32,
                            0x0096_1d50u32,
                            1u32,
                            0x17u32,
                            0.0f32,
                            0.0f32,
                            text_b,
                            text_a,
                            0u32
                        ],
                    );
                } else {
                    e.call(0x007d_0a70, &args![]);
                }
                e.set_global::<u8>(MESSAGE_SHOWN_011E07C0, 1);
            }
            e.vcall(this.addr(), 0x1e0, &args![]);
            let ahead: f32 = e.global(MOVE_SPEED_011A3B3C);
            let behind: f32 = e.global(LOOK_SPEED_011A3B40);
            if e.get(this, PlayerCharacter::b3rdPerson) {
                e.call(0x0089_5110, &args![this, ahead, behind]);
            }
            let toggled = !e.get(this, PlayerCharacter::b3rdPerson);
            e.set(this, PlayerCharacter::b3rdPerson, toggled);
            if e.get(this, PlayerCharacter::b3rdPerson) {
                e.call(0x0089_5110, &args![this, ahead, behind]);
            }
            e.call(0x008d_3550, &args![this, delta]);
            let first_person = !e.get(this, PlayerCharacter::b3rdPerson);
            let animation = e.call(0x0095_0a60, &args![this, first_person]).u32();
            e.call(0x0088_85e0, &args![this, animation, delta]);
            let toggled = !e.get(this, PlayerCharacter::b3rdPerson);
            e.set(this, PlayerCharacter::b3rdPerson, toggled);
            e.call(0x008d_3550, &args![this, delta]);
            let first_person = !e.get(this, PlayerCharacter::b3rdPerson);
            let animation = e.call(0x0095_0a60, &args![this, first_person]).u32();
            e.call(0x0088_85e0, &args![this, animation, delta]);
            e.call(0x0094_ae40, &args![this, 0u32, 0u32]);
            let parent = e.call(0x008d_6f30, &args![this]).u32();
            e.call(0x0054_a070, &args![parent, this, 1u32, 0u32]);
            let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
            let tes_object = e.call(0x0045_0b80, &args![0u32]).u32();
            e.call(0x00b5_d9f0, &args![tes_object, node, 1u32]);
            let parent = e.call(0x008d_6f30, &args![this]).u32();
            let music = e.call(0x0054_74b0, &args![parent, 0u32]).u32();
            e.set(this, PlayerCharacter::pLastKnownMusicType, Ptr::new(music));
            if e.get(this, PlayerCharacter::bAiControlledActivate) {
                if e.global::<u32>(PENDING_OBJECT_011E0788) != 0 {
                    if e.global::<u32>(STATIC_FLAGS_011E0BC8) & 1 == 0 {
                        let flags = e.global::<u32>(STATIC_FLAGS_011E0BC8);
                        e.set_global(STATIC_FLAGS_011E0BC8, flags | 1);
                        let interval = setting_float(e, ACTIVATE_INTERVAL_SETTING);
                        e.set_global(ACTIVATE_TIMER_011E0BC4, interval);
                    }
                    let wait_a = e.call(0x00a2_4660, &args![main_state, 5u32, 1u32]).u32() != 0
                        || e.call(0x00a2_4660, &args![main_state, 5u32, 0u32]).u32() != 0;
                    if wait_a && !e.call(0x005a_03f0, &args![this, 1u32]).bool() {
                        let node = e.vcall(this.addr(), 0x1e4, &args![]).u32();
                        e.call(0x0092_2650, &args![node, 7u32, 1u32]);
                        let timer: f32 = e.global(ACTIVATE_TIMER_011E0BC4);
                        let mut timer = (timer as f64 - delta as f64) as f32;
                        e.set_global(ACTIVATE_TIMER_011E0BC4, timer);
                        while (timer as f64) < zero {
                            let interval = setting_float(e, ACTIVATE_INTERVAL_SETTING);
                            timer = (timer as f64 + interval as f64) as f32;
                            e.set_global(ACTIVATE_TIMER_011E0BC4, timer);
                            let pending = e.global::<u32>(PENDING_OBJECT_011E0788);
                            if pending != 0 {
                                e.vcall(pending, 0x124, &args![0u32, this, 0u32, 0u32, 0u32]);
                            }
                            timer = e.global(ACTIVATE_TIMER_011E0BC4);
                        }
                    } else {
                        e.with_stack(12, |e, handle| {
                            e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
                            let player = e.global::<u32>(PLAYER_SINGLETON);
                            let found = e.call(0x005d_43c0, &args![player]).u32();
                            e.call(0x0041_8a40, &args![found, handle]);
                            e.call(SOUND_HANDLE_STOP, &args![handle]);
                            let node = e.vcall(this.addr(), 0x1e4, &args![]).u32();
                            e.call(0x0092_2650, &args![node, 7u32, 0u32]);
                            let interval = setting_float(e, ACTIVATE_INTERVAL_SETTING);
                            e.set_global(ACTIVATE_TIMER_011E0BC4, interval);
                            e.set_global(PENDING_OBJECT_011E0788, 0u32);
                            e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
                        });
                    }
                }
                let node = e.vcall(this.addr(), 0x1e4, &args![]).u32();
                if e.call(0x0049_85f0, &args![node]).bool() {
                    let process = process_of(e, this);
                    if e.vcall(process, 0x3e4, &args![]).i32() == -1 {
                        let process = process_of(e, this);
                        if e.vcall(process, 0x40c, &args![]).u32() == 0 {
                            e.set(this, PlayerCharacter::bAiControlledActivate, false);
                            e.set_global(PENDING_OBJECT_011E0788, 0u32);
                        }
                    }
                }
            }
            if e.call(0x00a2_4660, &args![main_state, 0x1au32, 1u32]).u32() != 0 {
                let reactor = e.global::<u32>(FAST_TRAVEL_REACTOR);
                if e.call(0x0085_09f0, &args![reactor]).bool() {
                    e.set_global(TIMER_011A3B34, e.global::<f32>(TIMER_RESET));
                }
            }
            'activation: {
                if e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
                    || e.vcall(this.addr(), 0x2e8, &args![]).bool()
                {
                    break 'activation;
                }
                e.call(0x008c_3c40, &args![this, 0u32, 0u32]);
                if e.call(0x00a2_4660, &args![main_state, 5u32, 1u32]).u32() == 0
                    || e.call(0x0096_7ae0, &args![this]).bool()
                    || e.call(0x0070_2360, &args![]).bool()
                    || e.vcall(this.addr(), 0x234, &args![]).bool()
                    || e.call(0x0057_21e0, &args![this]).bool()
                {
                    break 'activation;
                }
                let reference = e.call(0x0070_3350, &args![]).u32(); // -0x1f4
                let mut allow = true; // -0xf5
                let mut blocked = false; // -0xf6
                if reference != 0 && e.call(0x0056_8680, &args![reference]).bool() && cached {
                    blocked = true;
                }
                if e.call(0x005a_03f0, &args![this, 1u32]).bool() {
                    blocked = true;
                }
                if reference != 0 && e.vcall(reference, 0x224, &args![]).bool() {
                    let cast = e
                        .call(
                            0x00ec_43fb,
                            &args![reference, 0u32, 0x0118_41ccu32, 0x011a_28e0u32, 0u32],
                        )
                        .u32();
                    if cast != 0 && e.call(0x008c_e390, &args![cast]).u32() == 0 {
                        blocked = true;
                    }
                }
                if reference != 0 && e.vcall(reference, 0x100, &args![]).bool() {
                    let ref_process = e.call(0x008d_8520, &args![reference]).u32();
                    let kind = e.vcall(ref_process, 0x610, &args![]).u32();
                    if kind == 5 || e.vcall(ref_process, 0x610, &args![]).u32() == 6 {
                        blocked = true;
                    }
                }
                let state = e.vcall(this.addr(), 0x214, &args![]).u32();
                let index = state.wrapping_sub(1);
                if index <= 9 && index != 3 && index != 8 {
                    blocked = true;
                }
                let class = e.call(0x008a_7570, &args![this]).u32();
                if class != u32::MAX {
                    blocked = true;
                }
                if e.mem.u8(this.addr() + 0x64c) != e.mem.u8(this.addr() + 0x64a) {
                    blocked = true;
                }
                if e.call(0x0094_4340, &args![this]).bool() {
                    blocked = true;
                }
                let mut activated = false; // -0x10d
                if !blocked {
                    e.call(0x0070_62e0, &args![0u32, 0u32, 0u32]);
                    if reference != 0 {
                        e.with_stack(8, |e, list| {
                            e.call(LIST_CONSTRUCT, &args![list]);
                            e.call(
                                0x005e_58f0,
                                &args![0x1bu32, this, reference, list, reference],
                            );
                            if !e.call(LIST_IS_EMPTY, &args![list]).bool() {
                                if !e.call(0x007a_a720, &args![list, reference]).bool() {
                                    let activator = e.call(0x0077_8930, &args![reference]).u32();
                                    if e.call(
                                        0x0057_3170,
                                        &args![reference, this, 0u32, 0u32, activator],
                                    )
                                    .bool()
                                    {
                                        allow = false;
                                    }
                                }
                            } else {
                                let activator = e.call(0x0077_8930, &args![reference]).u32();
                                if e.call(
                                    0x0057_3170,
                                    &args![reference, this, 0u32, 0u32, activator],
                                )
                                .bool()
                                {
                                    allow = false;
                                }
                            }
                            e.call(LIST_DESTRUCT, &args![list]);
                        });
                    } else {
                        activated = true;
                    }
                    if allow {
                        let process = process_of(e, this);
                        if e.vcall(process, 0x4c8, &args![]).u32() != 0 {
                            let process = process_of(e, this);
                            let object = e.vcall(process, 0x4c8, &args![]).u32();
                            activated = !e
                                .call(0x0057_3170, &args![object, this, 0u32, 0u32, 1u32])
                                .bool();
                        } else {
                            activated = true;
                        }
                    }
                }
                if activated || blocked {
                    e.with_stack(24, |e, scratch| {
                        let found = e.call(0x005d_43c0, &args![this]).u32();
                        e.call(0x0041_8940, &args![found, handle]);
                        let sound_form = e.call(0x0082_ec10, &args![]).u32(); // -0x124
                        if !e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() && sound_form != 0
                        {
                            let name = e.call(0x0051_1840, &args![sound_form]).u32();
                            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                            let result = e
                                .call(
                                    0x00ad_7480,
                                    &args![audio, scratch, name, 0x121u32, sound_form],
                                )
                                .u32();
                            e.call(SOUND_HANDLE_ASSIGN, &args![handle, result]);
                            e.call(SOUND_HANDLE_DESTRUCT, &args![scratch]);
                        }
                        if !e.call(0x00ad_8930, &args![handle]).bool() {
                            e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
                        }
                        let found = e.call(0x005d_43c0, &args![this]).u32();
                        e.call(0x0041_a090, &args![found, handle]);
                    });
                }
            }
            break 'update;
        }
        // ---- 009408e4: the player is not under control ----
        let player_process = singleton_process(e);
        if e.vcall(player_process, 0x22c, &args![]).u32() != 0 {
            let player_process = singleton_process(e);
            e.vcall(player_process, 0x234, &args![]);
        }
        let player_process = singleton_process(e);
        if e.vcall(player_process, 0x20c, &args![]).u32() != 0 {
            let player_process = singleton_process(e);
            e.vcall(player_process, 0x214, &args![]);
        }
        e.call(0x0095_3d40, &args![this]);
        e.call(0x0095_3d00, &args![this]);
        let mut looked_flag = false; // -0x139
        let ammo_timer = e.get(this, PlayerCharacter::fAmmoSwapButtonTimer);
        e.set(
            this,
            PlayerCharacter::fAmmoSwapButtonTimer,
            (ammo_timer as f64 + delta as f64) as f32,
        );
        if e.call(0x00a2_4660, &args![main_state, 0x12u32, 1u32]).u32() != 0 {
            let process = process_of(e, this);
            if e.vcall(process, 0x148, &args![]).u32() != 0
                && e.call(0x008a_16d0, &args![this]).bool()
            {
                let first = e.call(0x0071_7a40, &args![1u32, 0u32]).i32();
                let second = e.call(0x0071_7a40, &args![4u32, 0u32]).i32();
                let third = e.call(0x0071_7a40, &args![3u32, 0u32]).i32();
                let first_up = first == 2 || first == 0;
                let second_up = second == 2 || second == 0;
                let third_up = third == 2 || third == 0;
                if !first_up || (!second_up && !third_up) {
                    let interval = setting_float(e, AMMO_SWAP_SETTING);
                    let timer = e.get(this, PlayerCharacter::fAmmoSwapButtonTimer);
                    let long_enough = (interval as f64) < timer as f64;
                    e.call(0x0094_62c0, &args![this, long_enough, 0u32]);
                    if long_enough {
                        e.set(this, PlayerCharacter::fAmmoSwapButtonTimer, 0.0);
                    }
                }
            }
        }
        if e.call(0x00a2_4660, &args![main_state, 7u32, 1u32]).u32() != 0
            || e.call(0x00a2_4660, &args![main_state, 7u32, 0u32]).u32() != 0
        {
            if !e.call(0x005a_03f0, &args![this, 8u32]).bool() {
                let held: f32 = e.global(HELD_TIMER_011E07E0);
                e.set_global(HELD_TIMER_011E07E0, (held as f64 + delta as f64) as f32);
                if e.call(0x0088_43a0, &args![this]).bool()
                    && e.call(0x008a_7570, &args![this]).u32() == u32::MAX
                {
                    let mut handled = false;
                    if e.global::<u8>(HELD_FLAG_011E0BC1) == 0 {
                        let limit = setting_float(e, HELD_SETTING_011CDFCC);
                        let held: f32 = e.global(HELD_TIMER_011E07E0);
                        let mut run = (limit as f64) < held as f64;
                        if !run {
                            run = if weapon_form == 0 {
                                true
                            } else {
                                let player = e.global::<u32>(PLAYER_SINGLETON);
                                if e.call(0x0052_5980, &args![weapon_form, player]).u32() != 0 {
                                    false
                                } else {
                                    !e.call(0x0047_4a80, &args![weapon_form + 0xa4]).bool()
                                }
                            };
                        }
                        if run && e.call(0x008a_6970, &args![this]).bool() {
                            looked_flag = true;
                            e.call(0x008a_6840, &args![this, 0u32]);
                            e.set_global::<u8>(HELD_FLAG_011E0BC1, 1);
                            handled = true;
                        }
                    }
                    if !handled
                        && e.global::<u8>(HELD_FLAG_011E0BC1) == 0
                        && !e.call(0x008a_6970, &args![this]).bool()
                    {
                        if e.call(0x0088_46e0, &args![this]).u32() & 0x800 == 0 {
                            e.call(0x008a_6840, &args![this, 1u32]);
                        }
                        e.set_global::<u8>(HELD_FLAG_011E0BC1, 1);
                    }
                }
            }
        } else if e.call(0x00a2_4660, &args![main_state, 7u32, 2u32]).u32() == 0 {
            e.set_global(HELD_TIMER_011E07E0, 0.0f32);
            e.set_global::<u8>(HELD_FLAG_011E0BC1, 0);
        }
        e.call(0x0096_73d0, &args![this, delta]);
        if e.call(0x00a2_4660, &args![main_state, 0x0au32, 1u32]).u32() != 0 {
            let toggled = !e.get(this, PlayerCharacter::bAlwaysRun);
            e.set(this, PlayerCharacter::bAlwaysRun, toggled);
        }
        if e.call(0x00a2_4660, &args![main_state, 0x0bu32, 1u32]).u32() != 0 {
            let toggled = !e.get(this, PlayerCharacter::bAutoMove);
            e.set(this, PlayerCharacter::bAutoMove, toggled);
        }
        if e.get(this, PlayerCharacter::bAutoMove)
            && !e.call(0x0070_9bc0, &args![]).bool()
            && !e.call(0x0070_2360, &args![]).bool()
        {
            if e.call(0x00a2_4660, &args![main_state, 0u32, 0u32]).u32() != 0
                || e.call(0x00a2_4660, &args![main_state, 1u32, 0u32]).u32() != 0
                || e.call(0x00a2_4660, &args![main_state, 3u32, 0u32]).u32() != 0
                || e.call(0x00a2_4660, &args![main_state, 2u32, 0u32]).u32() != 0
            {
                e.set(this, PlayerCharacter::bAutoMove, false);
            } else {
                e.call(0x00a2_4280, &args![main_state, 0u32]);
            }
        }
        let mut crouch_handled = false; // -0x160
        if e.call(0x00a2_4660, &args![main_state, 8u32, 1u32]).u32() != 0 {
            let busy = e.vcall(this.addr(), 0x234, &args![]).bool()
                || e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
                || e.vcall(this.addr(), 0x230, &args![]).bool()
                || e.call(0x0043_7bf0, &args![this]).bool()
                || e.call(0x0043_7bd0, &args![this]).bool()
                || e.call(0x005a_03f0, &args![this, 0x40u32]).bool()
                || e.vcall(this.addr(), 0x214, &args![]).u32() != 0;
            if !busy {
                // The classes (8a7570 + 1) for which the crouch sounds play.
                const PLAYS_CROUCH_SOUND: [bool; 19] = [
                    true, false, false, true, true, true, true, true, true, false, true, false,
                    false, false, false, false, false, false, true,
                ];
                let class = e.call(0x008a_7570, &args![this]).u32().wrapping_add(1);
                if class <= 0x12 && PLAYS_CROUCH_SOUND[class as usize] {
                    let state = e.mem.u16(word_slot);
                    let (name, argument) = if state & 0x400 != 0 {
                        e.mem.set_u16(word_slot, state & 0xfbff);
                        (CROUCH_UP_SOUND_NAME, 0x0bu32)
                    } else {
                        e.mem.set_u16(word_slot, state | 0x400);
                        (CROUCH_DOWN_SOUND_NAME, 0x0au32)
                    };
                    e.with_stack(12, |e, found| {
                        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                        let result = e
                            .call(
                                AUDIO_GET_SOUND_HANDLE_BY_NAME,
                                &args![audio, found, name, 0x0004_0102u32],
                            )
                            .u32();
                        e.call(SOUND_HANDLE_ASSIGN, &args![handle, result]);
                        e.call(SOUND_HANDLE_DESTRUCT, &args![found]);
                    });
                    let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                    e.call(0x0068_a7d0, &args![handle, position]);
                    e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
                    e.vcall(this.addr(), 0x340, &args![argument]);
                    if e.mem.u8(this.addr() + 0x64f) == 0 {
                        e.call(0x008b_b650, &args![this, 0u32, 0u32, 0u32]);
                        e.call(0x0089_4cc0, &args![this, 0u32]);
                    }
                }
            }
            crouch_handled = true;
        }
        // ---- 00940fb5: movement input ----
        let root_node = e.call(0x0055_8310, &args![root]).u32();
        let root_matrix = e.call(0x006a_9540, &args![root_node]).u32();
        let matrix = local(frame, -0x1b0);
        copy_matrix(e, matrix, root_matrix);
        let current = e.call(0x0043_6aa0, &args![this]).u32();
        let position_copy = local(frame, -0x1bc);
        let second_copy = local(frame, -0x1c8);
        let words = read_words(e, current);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(position_copy + 4 * i as u32, *word);
            e.mem.set_u32(second_copy + 4 * i as u32, *word);
        }
        let right = local(frame, -0x1d4);
        let forward = local(frame, -0x1e0);
        e.call(LIST_ITEM_ADDRESS, &args![right]);
        e.call(LIST_ITEM_ADDRESS, &args![forward]);
        let unit = e
            .call(
                0x0041_6870,
                &args![local(frame, -0x1ec), 1.0f32, 0.0f32, 0.0f32],
            )
            .u32();
        let words = read_words(e, unit);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(forward + 4 * i as u32, *word);
        }
        e.call(0x0043_9f50, &args![matrix, 1u32, right]);
        if !e.call(0x0050_d4a0, &args![this]).bool() {
            e.mem.set_f32(right + 8, 0.0);
            e.call(0x004a_0c10, &args![right]);
        } else {
            e.call(0x0043_9f50, &args![matrix, 0u32, forward]);
        }
        let mut grab_flag = false; // -0x1ed
        if e.get(this, PlayerCharacter::eGrabType) == 2 {
            grab_flag = e.call(0x0096_13c0, &args![this, delta]).bool();
        }
        e.call(0x0096_2350, &args![this]);
        let mut axis_a = 0i32; // -0x1f4
        let mut axis_b = 0i32; // -0x1f8
        let mut axis_c = 0i32; // -0x1fc
        let gamepad = e.call(0x004b_71d0, &args![]).bool();
        if gamepad {
            axis_a = e.call(0x00a2_3390, &args![main_state, 0u32, 7u32]).i32();
            axis_c = e.call(0x00a2_3390, &args![main_state, 0u32, 9u32]).i32();
            axis_b = e.call(0x00a2_3390, &args![main_state, 0u32, 8u32]).i32();
            if e.call(0x00ec_7d40, &args![axis_a]).i32() < 0x1ea9 {
                axis_a = 0;
            }
            if e.call(0x00ec_7d40, &args![axis_b]).i32() < 0x1ea9 {
                axis_b = 0;
            }
            if e.call(0x00ec_7d40, &args![axis_c]).i32() < 0x21f1 {
                axis_c = 0;
            }
            axis_a = axis_a.wrapping_mul(100) / 0x7fff;
            axis_b = axis_b.wrapping_mul(100) / 0x7fff;
            axis_c /= 0x7fff;
        }
        // (The block at 009411e9..0094134c runs only without a gamepad, where the
        // three axis values are still 0, so it has no effect and is left out.)
        let process = process_of(e, this);
        let aim_object = e.vcall(process, 0x148, &args![]).u32();
        let _aim_state = if aim_object != 0 {
            e.call(0x0044_ddc0, &args![aim_object]).u32()
        } else {
            0
        };
        let one: f64 = e.global(ONE_DOUBLE);
        let move_speed: f32 = e.global(MOVE_SPEED_011A3B3C);
        if move_speed as f64 > one {
            e.set_global(MOVE_SPEED_011A3B3C, 1.0f32);
        }
        if e.call(0x004b_71d0, &args![]).bool() {
            let divisor: f64 = e.global(AXIS_DIVISOR);
            let mut stick_x = (axis_a as f64 / divisor) as f32; // -0x224
            let mut stick_y = (axis_b as f64 / divisor) as f32; // -0x228
            let mut stick_z = (axis_c as f64 / divisor) as f32; // -0x22c
            if e.call(0x008a_7570, &args![this]).u32() == 0xb {
                stick_x = 0.0;
                stick_y = 0.0;
                stick_z = 0.0;
            }
            let stick = local(frame, -0x238);
            e.call(LIST_ITEM_ADDRESS, &args![stick]);
            let abs_x = e.call(0x0040_8840, &args![stick_x]).f64();
            let scale = setting_float(e, STICK_SCALE_SETTING);
            let weighted = (scale as f64 * stick_y as f64) as f32;
            let abs_weighted = e.call(0x0040_8840, &args![weighted]).f64();
            let y_dominant = abs_weighted > abs_x;
            if !y_dominant {
                if (stick_x as f64) < zero {
                    or_word(e, word_slot, 0x4);
                } else if stick_x as f64 > zero {
                    or_word(e, word_slot, 0x8);
                }
            }
            if y_dominant {
                if (stick_y as f64) < zero {
                    or_word(e, word_slot, 0x2);
                } else if stick_y as f64 > zero {
                    or_word(e, word_slot, 0x1);
                }
            }
            e.mem.set_f32(stick, stick_x);
            e.mem.set_f32(stick + 4, stick_y);
            e.mem.set_f32(stick + 8, 0.0);
            e.set_global(MOVE_SPEED_011A3B3C, 0.0f32);
            let mut full_speed = false; // -0x24a
            let lengthed = local(frame, -0x258);
            e.call(LIST_ITEM_ADDRESS, &args![lengthed]);
            e.mem.set_f32(lengthed, stick_x);
            e.mem.set_f32(lengthed + 4, stick_y);
            e.mem.set_f32(lengthed + 8, 0.0);
            let length = e.call(0x0045_7990, &args![lengthed]).f64();
            let clamped = if length > one {
                1.0f32
            } else {
                e.call(0x0045_7990, &args![lengthed]).f32()
            };
            let speed: f32 = e.global(MOVE_SPEED_011A3B3C);
            if (speed as f64) < one {
                let high: f64 = e.global(STICK_THRESHOLD_HIGH);
                let low: f64 = e.global(STICK_THRESHOLD_LOW);
                if clamped as f64 > high {
                    e.set_global(MOVE_SPEED_011A3B3C, 1.0f32);
                    full_speed = true;
                } else if clamped as f64 > low {
                    let a: f32 = e.global(STICK_REMAP_A);
                    let c: f32 = e.global(STICK_REMAP_C);
                    let mapped = e
                        .call(0x004b_3ab0, &args![a, 1.0f32, c, 1.0f32, clamped])
                        .f32();
                    e.set_global(MOVE_SPEED_011A3B3C, mapped);
                    full_speed = true;
                } else {
                    let a: f32 = e.global(STICK_REMAP_D);
                    let b: f32 = e.global(STICK_REMAP_E);
                    let c: f32 = e.global(STICK_REMAP_C);
                    let mapped = e.call(0x004b_3ab0, &args![a, b, 0.0f32, c, clamped]).f32();
                    e.set_global(MOVE_SPEED_011A3B3C, mapped);
                }
            }
            let stick_length = e.call(0x0045_7990, &args![stick]).f64();
            if stick_length > zero {
                if e.call(0x00a2_4660, &args![main_state, 9u32, 1u32]).u32() != 0
                    || e.call(0x00a2_4660, &args![main_state, 9u32, 0u32]).u32() != 0
                    || full_speed
                {
                    let process = process_of(e, this);
                    let mut run = e.vcall(process, 0x3e4, &args![]).u32() != 7
                        && !e.vcall(this.addr(), 0x358, &args![]).bool()
                        && !e.call(0x008b_bc10, &args![this]).bool();
                    if run {
                        let limit = setting_float(e, RUN_WEIGHT_SETTING);
                        let weight = e.get(this, PlayerCharacter::fGrabObjectWeight);
                        run = (limit as f64) >= weight as f64;
                    }
                    if run {
                        or_word(e, word_slot, 0x200);
                    } else {
                        let walk = setting_float(e, WALK_SPEED_SETTING);
                        e.set_global(MOVE_SPEED_011A3B3C, walk);
                        or_word(e, word_slot, 0x100);
                    }
                } else {
                    or_word(e, word_slot, 0x100);
                }
            }
            let speed: f32 = e.global(MOVE_SPEED_011A3B3C);
            if speed as f64 > one && e.vcall(this.addr(), 0x358, &args![]).bool() {
                e.set_global(MOVE_SPEED_011A3B3C, 1.0f32);
            }
            let speed: f32 = e.global(MOVE_SPEED_011A3B3C);
            if speed as f64 > one && e.call(0x008b_bc10, &args![this]).bool() {
                e.set_global(MOVE_SPEED_011A3B3C, 1.0f32);
            }
            let look_scale: f64 = e.global(LOOK_SCALE);
            let abs_z = e.call(0x0040_8840, &args![stick_z]).f64();
            let abs_x2 = e.call(0x0040_8840, &args![stick_x]).f64();
            let look_axis = if abs_x2 < abs_z {
                let abs_z2 = e.call(0x0040_8840, &args![stick_z]).f64();
                let abs_y = e.call(0x0040_8840, &args![stick_y]).f64();
                if abs_y < abs_z2 {
                    e.call(0x0040_8840, &args![stick_z]).f64()
                } else {
                    e.call(0x0040_8840, &args![stick_x]).f64()
                }
            } else {
                e.call(0x0040_8840, &args![stick_x]).f64()
            };
            e.set_global(LOOK_SPEED_011A3B40, (look_axis * look_scale) as f32);
            let speed = e.call(0x008a_0b10, &args![this]).f64();
            let move_speed: f32 = e.global(MOVE_SPEED_011A3B3C);
            let step = (speed * move_speed as f64) as f32;
            e.call(0x0043_9180, &args![stick, step]);
            if e.call(0x0050_d4a0, &args![this]).bool() {
                let turn = local(frame, -0x298);
                e.call(LIST_ITEM_ADDRESS, &args![turn]);
                let heading = e.call(0x0093_1d70, &args![this]).f32();
                e.call(0x0052_4ac0, &args![turn, -heading]);
                let turned = e
                    .call(0x004b_3ae0, &args![local(frame, -0x2a4), stick, turn])
                    .u32();
                let words = read_words(e, turned);
                for (i, word) in words.iter().enumerate() {
                    e.mem.set_u32(stick + 4 * i as u32, *word);
                }
            }
            let factor: f32 = e.global(STICK_FACTOR);
            e.call(0x0043_9180, &args![stick, factor]);
            e.call(0x0063_c8a0, &args![position_copy, stick]);
            let words = read_words(e, stick);
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(forward + 4 * i as u32, *word);
            }
        } else {
            e.set_global(MOVE_SPEED_011A3B3C, 1.0f32);
            or_word(e, word_slot, 0x40);
            let keys = local(frame, -0x2ac);
            e.call(LIST_ITEM_ADDRESS, &args![keys]);
            e.mem.set_f32(keys, axis_a as f32);
            e.mem.set_f32(keys + 4, axis_b as f32);
            let key_length = e.call(0x0058_9850, &args![keys]).f64();
            let key_limit: f64 = e.global(KEY_AXIS_LIMIT);
            if key_length > key_limit {
                e.call(0x00a2_4280, &args![main_state, 9u32]);
                let process = process_of(e, this);
                if e.vcall(process, 0x3e4, &args![]).u32() != 7
                    && !e.vcall(this.addr(), 0x358, &args![]).bool()
                    && !e.call(0x008b_bc10, &args![this]).bool()
                {
                    or_word(e, word_slot, 0x200);
                }
            }
            let run_held = |e: &mut Engine| {
                e.call(0x00a2_4660, &args![main_state, 9u32, 1u32]).u32() != 0
                    || e.call(0x00a2_4660, &args![main_state, 9u32, 0u32]).u32() != 0
            };
            if !e.call(0x004b_71d0, &args![]).bool() && e.get(this, PlayerCharacter::bAlwaysRun) {
                if !run_held(e) {
                    let process = process_of(e, this);
                    if e.vcall(process, 0x3e4, &args![]).u32() != 7
                        && !e.vcall(this.addr(), 0x358, &args![]).bool()
                        && !e.call(0x008b_bc10, &args![this]).bool()
                    {
                        e.call(0x00a2_4280, &args![main_state, 9u32]);
                        or_word(e, word_slot, 0x200);
                    }
                }
            } else if run_held(e) {
                let process = process_of(e, this);
                if e.vcall(process, 0x3e4, &args![]).u32() != 7
                    && !e.vcall(this.addr(), 0x358, &args![]).bool()
                    && !e.call(0x008b_bc10, &args![this]).bool()
                {
                    or_word(e, word_slot, 0x200);
                }
            }
            let limit = setting_float(e, RUN_WEIGHT_SETTING);
            let weight = e.get(this, PlayerCharacter::fGrabObjectWeight);
            if (limit as f64) < weight as f64 {
                and_word(e, word_slot, 0xfdff);
            }
            let speed = e.call(0x008a_0b10, &args![this]).f32();
            let move_speed: f32 = e.global(MOVE_SPEED_011A3B3C);
            let step = (speed as f64 * move_speed as f64) as f32;
            e.call(0x0043_9180, &args![forward, step]);
            let step = (speed as f64 * move_speed as f64) as f32;
            e.call(0x0043_9180, &args![right, step]);
            let key_pressed = |e: &mut Engine, key: u32| {
                e.call(0x00a2_4660, &args![main_state, key, 1u32]).u32() != 0
                    || e.call(0x00a2_4660, &args![main_state, key, 0u32]).u32() != 0
            };
            if key_pressed(e, 0) {
                e.call(0x0063_c8a0, &args![position_copy, right]);
                if e.mem.u16(word_slot) & 0x200 == 0 {
                    or_word(e, word_slot, 0x100);
                }
                or_word(e, word_slot, 0x1);
            }
            if key_pressed(e, 1) {
                e.call(0x0045_78c0, &args![position_copy, right]);
                if e.mem.u16(word_slot) & 0x200 == 0 {
                    or_word(e, word_slot, 0x100);
                }
                or_word(e, word_slot, 0x2);
            }
            if key_pressed(e, 2) {
                e.call(0x0045_78c0, &args![position_copy, forward]);
                if e.mem.u16(word_slot) & 0x200 == 0 {
                    or_word(e, word_slot, 0x100);
                }
                or_word(e, word_slot, 0x4);
            }
            if key_pressed(e, 3) {
                e.call(0x0063_c8a0, &args![position_copy, forward]);
                if e.mem.u16(word_slot) & 0x200 == 0 {
                    or_word(e, word_slot, 0x100);
                }
                or_word(e, word_slot, 0x8);
            }
        }
        let _ = grab_flag;
        // ---- 00941d13: apply the input ----
        if e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() == 4 {
            and_word(e, word_slot, 0xffc0);
        }
        let mover = e.get(this.cast::<Actor>(), Actor::pActorMover).addr();
        let word_value = e.mem.u16(word_slot) as u32;
        e.vcall(mover, 0xc, &args![word_value]);
        if crouch_handled {
            e.call(0x0088_4f80, &args![this]);
        }
        let mover = e.get(this.cast::<Actor>(), Actor::pActorMover).addr();
        e.vcall(mover, 0x18, &args![delta]);
        e.call(0x0095_03d0, &args![this]);
        e.call(0x0095_0530, &args![this]);
        let third_person = e.mem.u8(this.addr() + 0x64a);
        e.mem.set_u8(this.addr() + 0x648, third_person);
        e.set(this, PlayerCharacter::b3rdPersonSaved, true);
        e.mem.set_u8(this.addr() + 0x64a, 1);
        let process = process_of(e, this);
        let skip_cancel = e.call(0x00a2_4660, &args![main_state, 4u32, 0u32]).u32() != 0
            && e.global::<u8>(STATE_FLAG_011E07AA) != 0
            && e.vcall(process, 0x14c, &args![]).u32() != 0
            && e.call(0x008a_6970, &args![this]).bool();
        if !skip_cancel {
            let process = process_of(e, this);
            let weight = e.vcall(process, 0x444, &args![]).f64();
            if weight != zero || weight.is_nan() {
                let in_vats_playback = e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() == 4
                    && e.call(0x009c_71c0, &args![VATS_OBJECT]).u32() != 0
                    && {
                        let record = e.call(0x009c_71c0, &args![VATS_OBJECT]).u32();
                        e.mem.u8(record + 8) != 0
                    };
                if !in_vats_playback {
                    let process = process_of(e, this);
                    e.vcall(process, 0x434, &args![0u32]);
                }
            }
        }
        let mut busy = false; // -0x2b9
        {
            let process = process_of(e, this);
            if e.vcall(this.addr(), 0x230, &args![]).bool()
                || e.vcall(this.addr(), 0x234, &args![]).bool()
                || e.call(0x004f_8960, &args![this]).u32() != 0
                || e.call(0x008a_7570, &args![this]).u32() == 0x0a
                || e.vcall(process, 0x4bc, &args![]).u32() != 0
            {
                busy = true;
            }
        }
        if busy || e.call(0x005a_03f0, &args![this, 1u32]).bool() {
            let mover = e.get(this.cast::<Actor>(), Actor::pActorMover).addr();
            e.call(0x009e_a360, &args![mover, 0x33fu32, 0u32]);
        }
        if !busy && !e.call(0x005a_03f0, &args![this, 8u32]).bool() {
            let grab_type = e.get(this, PlayerCharacter::eGrabType);
            if grab_type == 2 || grab_type == 3 {
                e.set_global(HOLD_VALUE_011E07AC, 0u32);
                e.set_global(HOLD_VALUE_011E07B0, 0.0f32);
            } else {
                if e.call(0x00a2_4660, &args![main_state, 6u32, 1u32]).u32() != 0
                    || e.call(0x00a2_4660, &args![main_state, 6u32, 0u32]).u32() != 0
                {
                    let process = process_of(e, this);
                    if e.vcall(process, 0x3e4, &args![]).u32() != 7
                        && !e.call(0x008b_bc10, &args![this]).bool()
                        && e.mem.u8(this.addr() + 0x64c) == e.mem.u8(this.addr() + 0x64b)
                        && !e.call(0x0096_7ae0, &args![this]).bool()
                        && {
                            let process = process_of(e, this);
                            e.vcall(process, 0x3f8, &args![]).bool()
                        }
                    {
                        // 00941fea: a weapon raises the iron sights unless it
                        // is a melee one (`006450c0`) that is drawn, which
                        // blocks; without a weapon, a drawn actor blocks.
                        let block = if weapon_form != 0 {
                            e.call(0x0064_50c0, &args![weapon_form]).bool()
                                && e.call(0x008a_16d0, &args![this]).bool()
                        } else {
                            e.call(0x008a_16d0, &args![this]).bool()
                        };
                        if block {
                            e.call(0x0089_4cc0, &args![this, 1u32]);
                        } else {
                            e.call(0x008b_b650, &args![this, 1u32, 0u32, 0u32]);
                        }
                        if e.global::<u8>(STATE_FLAG_011E07B8) == 0
                            || e.global::<u8>(STATE_FLAG_011E07B9) != 0
                        {
                            moved_flag = true;
                        }
                    }
                } else if e.call(0x00a2_4660, &args![main_state, 6u32, 2u32]).u32() != 0
                    || e.call(0x00a2_4660, &args![main_state, 6u32, 0u32]).u32() == 0
                    || e.call(0x0096_7ae0, &args![this]).bool()
                {
                    let mut release = e.call(0x008b_bc10, &args![this]).bool();
                    if !release {
                        let process = process_of(e, this);
                        release = e.vcall(process, 0x3e4, &args![]).u32() == 7;
                    }
                    if release {
                        e.call(0x008b_b650, &args![this, 0u32, 0u32, 0u32]);
                        e.call(0x0089_4cc0, &args![this, 0u32]);
                        moved_flag = true;
                    }
                }
                let process = process_of(e, this);
                let word_value = e.mem.u16(word_slot);
                if e.vcall(process, 0x3f8, &args![]).bool() && word_value & 0x800 == 0 {
                    if e.call(0x0094_8310, &args![this]).bool() {
                        moved_flag = true;
                    }
                } else {
                    e.set_global(HOLD_VALUE_011E07AC, 0u32);
                    e.set_global(HOLD_VALUE_011E07B0, 0.0f32);
                }
            }
        }
        if !busy
            && !e.call(0x005a_03f0, &args![this, 1u32]).bool()
            && e.call(0x00a2_4660, &args![main_state, 0x0cu32, 1u32]).u32() != 0
            && !e.vcall(this.addr(), 0x358, &args![]).bool()
        {
            let node = e.vcall(this.addr(), 0x1e4, &args![]).u32();
            let kind = e.call(0x0043_01b0, &args![node, 4u32]).u16();
            if !e.call(0x005f_2670, &args![kind as u32]).bool()
                && e.call(0x008a_7570, &args![this]).u32() != 0x0b
            {
                let node = e.vcall(this.addr(), 0x1e4, &args![]).u32();
                if e.call(0x008a_7570, &args![this]).u32() == 0x0c
                    && e.call(0x0049_1040, &args![node, 1u32]).u32() != 0
                {
                    let held = e.call(0x0049_1040, &args![node, 1u32]).u32();
                    let owner = e.call(0x0048_f7f0, &args![held]).u32();
                    if e.call(0x005f_4db0, &args![owner]).bool() {
                        e.call(0x008a_73e0, &args![this, u32::MAX, 0u32]);
                    }
                }
                if e.call(0x008a_7570, &args![this]).u32() != 0x0c {
                    let free_hands = !(e.call(0x0088_49c0, &args![this]).bool()
                        || e.call(0x005a_2030, &args![this]).bool());
                    let held_6 = e.call(0x00a2_4660, &args![main_state, 6u32, 0u32]).u32() != 0;
                    let blocking = e.call(0x0089_4d60, &args![this]).bool();
                    if held_6 {
                        if e.call(0x0070_3350, &args![]).u32() == 0 {
                            let controller = e.call(0x0093_06d0, &args![this]).u32();
                            if e.call(0x005c_0880, &args![controller]).u32() == 0 {
                                let low = e.mem.u8(word_slot) as u32;
                                e.call(0x0089_4f90, &args![this, low]);
                            }
                        } else if !blocking {
                            let controller = e.call(0x0093_06d0, &args![this]).u32();
                            if e.call(0x0094_4400, &args![controller]).bool() {
                                e.vcall(this.addr(), 0x254, &args![]);
                            }
                        }
                    } else {
                        let controller = e.call(0x0093_06d0, &args![this]).u32();
                        if e.call(0x0094_4400, &args![controller]).bool() {
                            e.vcall(this.addr(), 0x254, &args![]);
                        }
                    }
                    let _ = free_hands;
                    e.call(0x0095_f6a0, &args![this]);
                    moved_flag = true;
                }
            }
        }
        if !e.vcall(this.addr(), 0x448, &args![]).bool() {
            let holder = e.call(0x005d_43c0, &args![this]).u32();
            if e.call(0x0041_cd70, &args![holder]).u32() != 0 {
                let holder = e.call(0x005d_43c0, &args![this]).u32();
                e.call(0x0041_cda0, &args![holder]);
            }
        }
        let mut wait_menu = false; // -0x2c8
        if e.call(0x00a2_4660, &args![main_state, 0x0fu32, 1u32]).u32() != 0
            && !e.call(0x005a_03d0, &args![this]).bool()
        {
            let _ = &mut wait_menu;
            let icon = FAST_TRAVEL_ICON;
            if e.vcall(this.addr(), 0x448, &args![]).bool() {
                show_text(e, MESSAGE_SETTING_011D4474, icon);
            } else if e.call(0x0095_3c80, &args![this]).bool() {
                show_text(e, MESSAGE_SETTING_011D28B0, icon);
            } else {
                let distance: f32 = e.global(INTERACT_DISTANCE_01084D20);
                let parent = e.call(0x008d_6f30, &args![this]).u32();
                let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                if e.call(0x0088_5520, &args![this, position, parent, distance])
                    .bool()
                {
                    show_text(e, MESSAGE_SETTING_011D4948, icon);
                } else {
                    let player = e.global::<u32>(PLAYER_SINGLETON);
                    let interior = e.call(0x0057_5d10, &args![player]).bool();
                    if e.call(0x0097_64a0, &args![PROCESS_LISTS, interior as u32])
                        .bool()
                    {
                        show_text(e, MESSAGE_SETTING_011D2E50, icon);
                    } else {
                        let player = e.global::<u32>(PLAYER_SINGLETON);
                        let controller = e.call(0x0093_06d0, &args![player]).u32();
                        let first = e.call(0x005c_0880, &args![controller]).u32();
                        let second = if first == 1 {
                            1
                        } else {
                            let player = e.global::<u32>(PLAYER_SINGLETON);
                            let controller = e.call(0x0093_06d0, &args![player]).u32();
                            e.call(0x005c_0880, &args![controller]).u32()
                        };
                        if first == 1 || second == 2 {
                            show_text(e, MESSAGE_SETTING_011D3E20, icon);
                        } else {
                            let player = e.global::<u32>(PLAYER_SINGLETON);
                            let parent = e.call(0x008d_6f30, &args![player]).u32();
                            let cannot_wait = e.call(0x0054_44c0, &args![parent]).bool() || {
                                let player = e.global::<u32>(PLAYER_SINGLETON);
                                !e.call(0x0094_4360, &args![player]).bool()
                            };
                            if cannot_wait {
                                show_text(e, MESSAGE_SETTING_011D20AC, icon);
                            } else if e.call(0x0050_98e0, &args![PROCESS_LISTS]).bool() {
                                show_text(e, MESSAGE_SETTING_011D4B10, icon);
                            } else {
                                let waiting = e.call(0x008d_8520, &args![this]).u32() != 0 && {
                                    let process_like = e.call(0x008d_8520, &args![this]).u32();
                                    e.vcall(process_like, 0x764, &args![]).f64() != zero
                                };
                                if waiting {
                                    show_text(e, MESSAGE_SETTING_011D4B10, icon);
                                } else {
                                    let player = e.global::<u32>(PLAYER_SINGLETON);
                                    if e.call(0x0082_2e00, &args![player + MAGIC_TARGET_BASE])
                                        .bool()
                                    {
                                        show_text(e, MESSAGE_SETTING_011D47D4, icon);
                                    } else {
                                        e.call(0x0070_54f0, &args![0u32]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        e.mem.set_u8(this.addr() + 0x64a, 1);
        e.vcall(this.addr(), 0x1e0, &args![]);
        let ahead: f32 = e.global(MOVE_SPEED_011A3B3C);
        let behind: f32 = e.global(LOOK_SPEED_011A3B40);
        e.call(0x0089_5110, &args![this, ahead, behind]);
        let process = process_of(e, this);
        e.vcall(process, 0x41c, &args![this]);
        let saved_third_person = e.mem.u8(this.addr() + 0x648);
        e.mem.set_u8(this.addr() + 0x64a, saved_third_person);
        e.set(this, PlayerCharacter::b3rdPersonSaved, false);
        if e.call(0x005a_1e50, &args![this]).bool() {
            e.call(0x0094_df80, &args![this]);
        }
        if e.call(0x0045_6c70, &args![]).bool() || {
            let setting = e.call(0x0040_8d60, &args![FLAG_SETTING_011E0B64]).u32();
            e.mem.u8(setting) != 0
        } {
            if !e.call(0x0050_d4a0, &args![this]).bool() {
                e.call(LIST_ITEM_ADDRESS, &args![local(frame, -0x2e0)]);
                let controller = e.call(0x0093_06d0, &args![this]).u32();
                if controller != 0 {
                    e.call(0x0056_20e0, &args![controller, position_copy]);
                    let value = e.call(0x0045_8b20, &args![]).u32();
                    e.call(0x0080_fb00, &args![controller, value]);
                }
                e.call(0x0069_3ef0, &args![this, 1u32]);
            }
            let moved_by = local(frame, -0x2f0);
            e.call(0x0043_9ef0, &args![position_copy, moved_by, second_copy]);
            e.call(0x0043_9180, &args![moved_by, delta]);
            let lift = e.mem.f32(moved_by + 8);
            let pushed = e
                .call(
                    0x0041_6870,
                    &args![local(frame, -0x2fc), 0.0f32, 0.0f32, lift],
                )
                .u32();
            let sum = e
                .call(
                    0x0043_9e90,
                    &args![second_copy, local(frame, -0x308), pushed],
                )
                .u32();
            e.vcall(this.addr(), 0x2a8, &args![sum]);
        } else {
            e.call(0x0069_3ef0, &args![this, 0u32]);
        }
        let mover = e.get(this.cast::<Actor>(), Actor::pActorMover).addr();
        e.call(0x009e_a570, &args![mover, forward]);
        let mover = e.get(this.cast::<Actor>(), Actor::pActorMover).addr();
        e.vcall(mover, 0x14, &args![delta]);
        e.call(0x0094_7b10, &args![this]);
        if e.mem.u16(word_slot) & 0xf != 0 {
            moved_flag = true;
        }
        if e.call(0x00a2_4660, &args![main_state, 0x19u32, 1u32]).u32() != 0 {
            let reactor = e.global::<u32>(FAST_TRAVEL_REACTOR);
            e.call(0x0085_09a0, &args![reactor]);
        }
        if e.call(0x00a2_4660, &args![main_state, 0x1au32, 1u32]).u32() != 0 {
            let reactor = e.global::<u32>(FAST_TRAVEL_REACTOR);
            e.call(0x0085_09f0, &args![reactor]);
        }
        let quick_slot = e.call(0x00a2_4660, &args![main_state, 0x10u32, 1u32]).i32() > 0; // -0x309
        let mut run_body = e.global::<u8>(QUICK_STATE_011E0780) != 0;
        if !run_body && quick_slot {
            run_body = !(e.call(0x005a_03f0, &args![this, 8u32]).bool()
                || e.vcall(this.addr(), 0x214, &args![]).u32() != 0
                || e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() == 4
                || e.call(0x0096_7ae0, &args![this]).bool()
                || e.call(0x0070_5a00, &args![]).bool()
                || e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() != 0
                || e.call(0x0088_46e0, &args![this]).u32() & 0x800 != 0);
        }
        if run_body {
            if e.call(0x007f_52c0, &args![]).bool() {
                if quick_slot {
                    let player = e.global::<u32>(PLAYER_SINGLETON);
                    let flag = !e.call(0x004e_af60, &args![player]).bool();
                    e.set_global::<u8>(VATS_ENDED_THIRD_PERSON, flag as u8);
                }
                let mut pipboy_ok = false; // -0x30a
                let class = e.call(0x008a_7570, &args![this]).u32();
                if class == u32::MAX || class == 7 {
                    let a = e
                        .call(0x0049_1040, &args![animation_third_person, 4u32])
                        .u32();
                    let b = e
                        .call(0x0049_1040, &args![animation_third_person, 4u32])
                        .u32();
                    let x = if a != 0 {
                        e.call(0x0048_f7f0, &args![a]).u32()
                    } else {
                        0
                    };
                    let id = if x != 0 {
                        e.call(0x005f_2420, &args![x]).i32()
                    } else {
                        0xff
                    };
                    if !(0xc4..0xc7).contains(&id) {
                        pipboy_ok = true;
                    } else if a != 0 {
                        e.call(0x0049_74a0, &args![animation_third_person]);
                        e.call(0x0049_94f0, &args![animation_third_person, 4u32, 0u32]);
                        if b != 0 {
                            e.call(0x0049_74a0, &args![animation_first_person]);
                            e.call(0x0049_94f0, &args![animation_first_person, 4u32, 0u32]);
                        }
                    }
                }
                let mut opened = false;
                if pipboy_ok && !e.get(this, PlayerCharacter::bTemp1stPerson) {
                    let first = e.call(0x0071_6440, &args![]).f64();
                    let second = e.call(0x0094_43f0, &args![]).f64();
                    if first == second
                        && !e.get(this, PlayerCharacter::bTemp3rdPerson)
                        && e.call(0x008a_16d0, &args![this]).bool()
                        && !e.call(0x0095_0090, &args![this]).bool()
                        && !e.get(this, PlayerCharacter::b3rdPerson)
                        && e.mem.u8(this.addr() + 0x64c) == e.mem.u8(this.addr() + 0x64a)
                    {
                        opened = true;
                    }
                }
                if opened {
                    let form = e.call(0x005d_2860, &args![]).u32();
                    e.call(0x0052_9c90, &args![form]);
                    e.call(0x0086_fd90, &args![main, 1u32]);
                    e.call(0x0070_5640, &args![]);
                    e.set_global::<u8>(QUICK_STATE_011E0780, 0);
                } else {
                    e.set_global::<u8>(QUICK_STATE_011E0780, 1);
                    e.call(0x008a_6840, &args![this, 1u32]);
                    if e.call(0x0095_0090, &args![this]).bool()
                        || e.get(this, PlayerCharacter::b3rdPerson)
                    {
                        e.call(0x0095_00a0, &args![this]);
                        e.call(0x0095_1a10, &args![this, 1u32]);
                        e.set(this, PlayerCharacter::bWant3rdPerson, false);
                        e.set(this, PlayerCharacter::b3rdPerson, false);
                    }
                }
            } else {
                e.with_stack(12, |e, found| {
                    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                    let result = e
                        .call(
                            AUDIO_GET_SOUND_HANDLE_BY_NAME,
                            &args![audio, found, QUICK_MENU_SOUND_NAME, 0x121u32],
                        )
                        .u32();
                    e.call(SOUND_HANDLE_PLAY, &args![result, 0u32]);
                    e.call(SOUND_HANDLE_DESTRUCT, &args![found]);
                });
                e.set_global::<u8>(QUICK_STATE_011E0780, 0);
            }
            moved_flag = true;
        }
        if !e.call(0x0096_7ae0, &args![this]).bool()
            && e.global::<u32>(PENDING_OBJECT_011E0788) == 0
            && !e.get(this, PlayerCharacter::bTemp1stPerson)
            && (!e.get(this, PlayerCharacter::bTemp3rdPerson)
                || e.global::<u8>(STATE_FLAG_011E07B8) != 0)
            && e.global::<u8>(QUICK_STATE_011E0780) == 0
        {
            if (e.call(0x00a2_4660, &args![main_state, 0x0du32, 1u32]).u32() != 0
                || e.call(0x00a2_4660, &args![main_state, 0x0du32, 0u32]).u32() != 0)
                && !e.call(0x005a_03f0, &args![this, 0x10u32]).bool()
            {
                {
                    if !e.call(0x0070_2360, &args![]).bool()
                        && e.global::<u8>(CAMERA_FLAG_011A3B31) != 0
                    {
                        moved_flag = true;
                        e.set_global::<u8>(STATE_FLAG_011E07C1, 1);
                        if e.global::<u8>(STATE_FLAG_011E07B8) == 0 {
                            let actually = e.mem.u8(this.addr() + 0x64b);
                            e.set_global::<u8>(SAVED_VIEW_011E0BC0, actually);
                            e.call(0x0095_0110, &args![this, 0u32]);
                            e.set_global(CAMERA_VALUE_011E07C4, 0.0f32);
                            e.set_global::<u8>(STATE_FLAG_011E07B9, 0);
                            e.set_global::<u8>(STATE_FLAG_011E07B8, 1);
                            e.call(0x0077_1700, &args![0x14u32]);
                            e.set_global(CAMERA_VALUE_011E0B60, 0.0f32);
                            let value: f32 = e.global(CAMERA_VALUE_011E0B5C);
                            e.set_global(CAMERA_VALUE_011E0BBC, value);
                        }
                        let timer: f32 = e.global(CAMERA_TIMER_011E07C8);
                        e.set_global(CAMERA_TIMER_011E07C8, (timer as f64 + delta as f64) as f32);
                    }
                }
            } else if e.call(0x00a2_4660, &args![main_state, 0x0du32, 2u32]).u32() != 0
                || (e.global::<u8>(STATE_FLAG_011E07B8) != 0
                    && e.global::<u8>(STATE_FLAG_011E07B9) == 0)
            {
                e.set_global(CAMERA_VALUE_011E07C4, 0.0f32);
                e.set_global::<u8>(STATE_FLAG_011E07B9, 0);
                e.set_global::<u8>(STATE_FLAG_011E07B8, 0);
                e.call(0x0077_1700, &args![1u32]);
                let limit = setting_float(e, CAMERA_SETTING_011CDE8C);
                let timer: f32 = e.global(CAMERA_TIMER_011E07C8);
                if limit as f64 > timer as f64 {
                    let same = if e.call(0x004b_71d0, &args![]).bool() {
                        true
                    } else {
                        let a: f32 = e.global(CAMERA_VALUE_011E0B5C);
                        let b: f32 = e.global(CAMERA_VALUE_011E0BBC);
                        a == b
                    };
                    if same && !e.call(0x005a_03f0, &args![this, 0x10u32]).bool() {
                        if e.global::<u8>(SAVED_VIEW_011E0BC0) != 0 {
                            let flag = e.mem.u8(this.addr() + 0x64e) == 0;
                            e.call(0x0095_0110, &args![this, flag as u32]);
                        }
                        e.set(this, PlayerCharacter::bTemp3rdPerson, false);
                        e.set(this, PlayerCharacter::bTemp3rdPersonSwitchBack, false);
                    }
                }
                e.set_global(CAMERA_TIMER_011E07C8, 0.0f32);
                moved_flag = true;
                e.set_global::<u8>(STATE_FLAG_011E07C1, 0);
                e.set_global(CAMERA_VALUE_011E0B58, 0.0f32);
                e.set_global(CAMERA_VALUE_011E0B60, 0.0f32);
            }
        }
        let mut weapon_ready = false; // -0x345
        let process = process_of(e, this);
        if e.vcall(process, 0x4c8, &args![]).u32() != 0 && e.mem.u16(word_slot) & 0xf != 0 {
            weapon_ready = true;
        }
        if e.call(0x00a2_4660, &args![main_state, 5u32, 1u32]).u32() != 0 || weapon_ready {
            'second_activation: {
                if e.global::<u8>(QUICK_STATE_011E0780) != 0
                    || e.call(0x0096_7ae0, &args![this]).bool()
                    || e.call(0x0070_2360, &args![]).bool()
                    || e.vcall(this.addr(), 0x234, &args![]).bool()
                    || grab_flag
                    || e.call(0x0057_21e0, &args![this]).bool()
                {
                    break 'second_activation;
                }
                let reference = e.call(0x0070_3350, &args![]).u32(); // -0x34c
                let mut allow = true; // -0x34d
                let mut blocked = false; // -0x34e
                if reference != 0 && e.call(0x0056_8680, &args![reference]).bool() && cached {
                    blocked = true;
                }
                if e.call(0x005a_03f0, &args![this, 1u32]).bool() {
                    blocked = true;
                }
                if reference != 0 && e.vcall(reference, 0x224, &args![]).bool() {
                    let cast = e
                        .call(
                            0x00ec_43fb,
                            &args![reference, 0u32, 0x0118_41ccu32, 0x011a_28e0u32, 0u32],
                        )
                        .u32();
                    if cast != 0 && e.call(0x008c_e390, &args![cast]).u32() == 0 {
                        blocked = true;
                    }
                }
                if reference != 0 && e.vcall(reference, 0x100, &args![]).bool() {
                    let ref_process = e.call(0x008d_8520, &args![reference]).u32();
                    let distance: f32 = e.global(INTERACT_DISTANCE_01084D20);
                    if !e.vcall(reference, 0x22c, &args![0u32]).bool() {
                        let position = e.vcall(reference, 0x1f4, &args![]).u32();
                        let cell = e.call(0x008d_6f30, &args![reference]).u32();
                        if e.call(0x0088_5520, &args![reference, position, cell, distance])
                            .bool()
                        {
                            blocked = true;
                        } else {
                            let cell = e.call(0x008d_6f30, &args![this]).u32();
                            let player = e.global::<u32>(PLAYER_SINGLETON);
                            let position = e.vcall(player, 0x1f4, &args![]).u32();
                            if e.call(0x0088_5520, &args![player, position, cell, distance])
                                .bool()
                            {
                                blocked = true;
                            } else if ref_process != 0
                                && e.vcall(ref_process, 0x610, &args![]).u32() == 5
                            {
                                blocked = true;
                            } else if ref_process != 0
                                && e.vcall(ref_process, 0x610, &args![]).u32() == 6
                            {
                                blocked = true;
                            }
                        }
                    }
                }
                let state = e.vcall(this.addr(), 0x214, &args![]).u32();
                let index = state.wrapping_sub(1);
                if index <= 9 && index != 3 && index != 8 {
                    blocked = true;
                }
                if e.call(0x008a_7570, &args![this]).u32() != u32::MAX {
                    blocked = true;
                }
                if e.mem.u8(this.addr() + 0x64c) != e.mem.u8(this.addr() + 0x64a) {
                    blocked = true;
                }
                if e.call(0x0094_4340, &args![this]).bool() {
                    blocked = true;
                }
                let mut activated = false; // -0x365
                if !blocked {
                    if reference == 0 || !e.vcall(reference, 0x100, &args![]).bool() {
                        e.call(0x0070_62e0, &args![0u32, 0u32, 0u32]);
                    }
                    if reference != 0 && !weapon_ready {
                        e.with_stack(8, |e, list| {
                            e.call(LIST_CONSTRUCT, &args![list]);
                            e.call(
                                0x005e_58f0,
                                &args![0x1bu32, this, reference, list, reference],
                            );
                            if !e.call(LIST_IS_EMPTY, &args![list]).bool() {
                                if !e.call(0x007a_a720, &args![list, reference]).bool() {
                                    let activator = e.call(0x0077_8930, &args![reference]).u32();
                                    if e.call(
                                        0x0057_3170,
                                        &args![reference, this, 0u32, 0u32, activator],
                                    )
                                    .bool()
                                    {
                                        allow = false;
                                    }
                                }
                            } else {
                                let activator = e.call(0x0077_8930, &args![reference]).u32();
                                if e.call(
                                    0x0057_3170,
                                    &args![reference, this, 0u32, 0u32, activator],
                                )
                                .bool()
                                {
                                    allow = false;
                                } else {
                                    let owner = e.call(0x0056_9160, &args![reference]).u32();
                                    if owner != 0 {
                                        let owner = e.call(0x0056_9160, &args![reference]).u32();
                                        let level =
                                            e.call(0x0043_09e0, &args![owner, reference]).u32();
                                        if level != 5
                                            && !e.call(0x0057_b460, &args![reference]).bool()
                                        {
                                            allow = false;
                                        }
                                    }
                                }
                            }
                            e.call(LIST_DESTRUCT, &args![list]);
                        });
                    } else {
                        activated = true;
                    }
                    if allow {
                        let process = process_of(e, this);
                        if e.vcall(process, 0x4c8, &args![]).u32() != 0 {
                            let process = process_of(e, this);
                            let object = e.vcall(process, 0x4c8, &args![]).u32();
                            activated = !e
                                .call(0x0057_3170, &args![object, this, 0u32, 0u32, 1u32])
                                .bool();
                        } else {
                            activated = true;
                        }
                    }
                }
                if activated || blocked {
                    e.with_stack(0x30, |e, scratch| {
                        let sound = scratch.addr();
                        let temporary = scratch.addr() + 0x14;
                        e.call(SOUND_HANDLE_CONSTRUCT, &args![sound]);
                        let found = e.call(0x005d_43c0, &args![this]).u32();
                        e.call(0x0041_8940, &args![found, sound]);
                        let sound_form = e.call(0x0082_ec10, &args![]).u32();
                        if !e.call(SOUND_HANDLE_IS_VALID, &args![sound]).bool() && sound_form != 0 {
                            let name = e.call(0x0051_1840, &args![sound_form]).u32();
                            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                            let result = e
                                .call(
                                    0x00ad_7480,
                                    &args![audio, temporary, name, 0x121u32, sound_form],
                                )
                                .u32();
                            e.call(SOUND_HANDLE_ASSIGN, &args![sound, result]);
                            e.call(SOUND_HANDLE_DESTRUCT, &args![temporary]);
                        }
                        if !e.call(0x00ad_8930, &args![sound]).bool() {
                            e.call(SOUND_HANDLE_PLAY, &args![sound, 0u32]);
                        }
                        let found = e.call(0x005d_43c0, &args![this]).u32();
                        e.call(0x0041_a090, &args![found, sound]);
                        e.call(SOUND_HANDLE_DESTRUCT, &args![sound]);
                    });
                }
                moved_flag = true;
            }
        }
        // ---- 00943487: camera orbit, effects and animation refresh ----
        if e.global::<u8>(STATE_FLAG_011E07B9) != 0 {
            if moved_flag || e.call(0x005a_03f0, &args![this, 0x10u32]).bool() {
                e.call(0x0095_00a0, &args![this]);
            } else {
                let orbit_scale: f64 = e.global(ORBIT_SCALE);
                let speed_a = setting_float(e, ORBIT_SETTING_011CD4CC);
                let value: f32 = e.global(ORBIT_VALUE_011E08FC);
                let step = speed_a as f64 * orbit_scale * delta as f64;
                e.set_global(ORBIT_VALUE_011E08FC, (value as f64 - step) as f32);
                let speed_b = setting_float(e, ORBIT_SETTING_011CD198);
                let angle: f32 = e.global(ORBIT_ANGLE_011E0BB8);
                let step = speed_b as f64 * orbit_scale * delta as f64;
                let angle = (angle as f64 + step) as f32;
                e.set_global(ORBIT_ANGLE_011E0BB8, angle);
                let wave = e.call(0x004e_44b0, &args![angle]).f64();
                let amplitude = setting_float(e, ORBIT_SETTING_011CD984);
                e.set_global(
                    ORBIT_VALUE_011E08F4,
                    ((amplitude as f64 * wave) * orbit_scale) as f32,
                );
                let angle: f32 = e.global(ORBIT_ANGLE_011E0BB8);
                let two_pi: f64 = e.global(TWO_PI_DOUBLE);
                if angle as f64 > two_pi {
                    e.set_global(ORBIT_ANGLE_011E0BB8, (angle as f64 - two_pi) as f32);
                }
            }
        } else if e.global::<u8>(STATE_FLAG_011E07B8) == 0 {
            if moved_flag {
                e.set_global(CAMERA_VALUE_011E07C4, 0.0f32);
            } else {
                let idle: f32 = e.global(CAMERA_VALUE_011E07C4);
                e.set_global(CAMERA_VALUE_011E07C4, (idle as f64 + delta as f64) as f32);
            }
            if e.call(0x004b_9930, &args![IDLE_OBJECT_011E09E0]).bool() {
                let limit = setting_float(e, IDLE_SETTING_011CCF50);
                let idle: f32 = e.global(CAMERA_VALUE_011E07C4);
                if (limit as f64) < idle as f64
                    && !e.call(0x005a_03f0, &args![this, 0x10u32]).bool()
                {
                    let value: f32 = e.global(CAMERA_VALUE_011E0B5C);
                    e.set_global(CAMERA_VALUE_011E07BC, value);
                    e.set_global(ORBIT_VALUE_011E08FC, 0.0f32);
                    if e.get(this, PlayerCharacter::b3rdPerson) {
                        let heading = e.call(0x0093_1d70, &args![this]).f32();
                        e.set_global(ORBIT_VALUE_011E08F4, heading);
                    } else {
                        e.set_global(ORBIT_VALUE_011E08F4, 0.0f32);
                    }
                    e.call(0x0095_0340, &args![this, 1u32]);
                    e.set_global::<u8>(STATE_FLAG_011E07B9, 1);
                    e.set_global::<u8>(STATE_FLAG_011E07B8, 1);
                    e.call(0x0077_1700, &args![0x14u32]);
                    e.set_global::<u8>(STATE_FLAG_011E07C3, 1);
                }
            }
        }
        e.call(0x0095_f6c0, &args![this, delta]);
        let marker = e.global::<u32>(MARKER_OBJECT_011E07EC);
        if marker != 0 {
            let count = e.global::<u32>(MARKER_COUNT_011E0BB4);
            e.set_global(MARKER_COUNT_011E0BB4, count.wrapping_add(1));
            if count < 0x14 {
                let mut at = read_floats(e, position_copy);
                let z: f64 = e.global(MARKER_OFFSET_Z);
                let xy: f64 = e.global(MARKER_OFFSET_XY);
                at[2] = (at[2] as f64 + z) as f32;
                at[1] = (at[1] as f64 + xy) as f32;
                at[0] = (at[0] as f64 + xy) as f32;
                let marker_position = local(frame, -0x3b4);
                for (i, value) in at.iter().enumerate() {
                    e.mem.set_f32(marker_position + 4 * i as u32, *value);
                }
                e.call(0x0056_10f0, &args![marker, marker_position]);
                e.call(0x0056_15d0, &args![marker, ZERO_VECTOR]);
            }
        }
        if !e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
            && !e.vcall(this.addr(), 0x2e8, &args![]).bool()
        {
            e.call(0x008c_3c40, &args![this, 0u32, 0u32]);
        }
        let value = e.call(0x0094_42a0, &args![animation_third_person]).f32();
        e.call(0x0089_50f0, &args![animation_first_person, value]);
        let value = e.call(0x0050_8070, &args![animation_third_person]).f32();
        e.call(0x004c_0c90, &args![animation_first_person, value]);
        if !e.call(0x0070_50d0, &args![]).bool() {
            e.call(0x0095_de30, &args![this, delta]);
        }
        for _ in 0..2 {
            let toggled = !e.get(this, PlayerCharacter::b3rdPerson);
            e.set(this, PlayerCharacter::b3rdPerson, toggled);
            e.call(0x008d_3550, &args![this, delta]);
            let first_person = !e.get(this, PlayerCharacter::b3rdPerson);
            let animation = e.call(0x0095_0a60, &args![this, first_person]).u32();
            e.call(0x0088_85e0, &args![this, animation, delta]);
        }
        e.call(0x008b_a600, &args![this]);
        if e.global::<u8>(VATS_ENDED_FLAG) == 0 {
            e.call(0x0094_ae40, &args![this, 0u32, 0u32]);
        }
        let parent = e.call(0x008d_6f30, &args![this]).u32();
        e.call(0x0054_a070, &args![parent, this, 1u32, 0u32]);
        let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
        let tes_object = e.call(0x0045_0b80, &args![0u32]).u32();
        e.call(0x00b5_d9f0, &args![tes_object, node, 1u32]);
        let process = process_of(e, this);
        let tracked = e.vcall(process, 0x6b8, &args![]).u32();
        if tracked != 0 {
            let process = process_of(e, this);
            let tracked = e.vcall(process, 0x6b8, &args![]).u32();
            e.call(0x009b_b080, &args![tracked, delta, this]);
        }
        if e.vcall(this.addr(), 0x22c, &args![0u32]).bool() {
            let record_is_end = e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() == 4 && {
                let record = e.call(0x009c_71c0, &args![VATS_OBJECT]).u32();
                record != 0 && {
                    let record = e.call(0x009c_71c0, &args![VATS_OBJECT]).u32();
                    e.mem.u32(record) == 0x0f
                }
            };
            if !record_is_end {
                e.call(0x009c_a1f0, &args![VATS_OBJECT]);
            }
        }
        e.call(0x0055_5c20, &args![]);
        // ---- 009438fb: tracked cell, level up, combat updates ----
        let tracked_cell = e.global::<u32>(TRACKED_CELL_011E0BB0);
        if tracked_cell == 0 || tracked_cell != e.call(0x008d_6f30, &args![this]).u32() {
            let cell = e.call(0x008d_6f30, &args![this]).u32();
            e.set_global(TRACKED_CELL_011E0BB0, cell);
            let manager = e.global::<u32>(CELL_MANAGER_011C3B3C);
            if e.call(0x0044_7950, &args![manager, cell]).bool() {
                let tes = e.global::<u32>(TES_SINGLETON);
                e.call(0x0045_81e0, &args![tes, CELL_CHANGE_STRING]);
                let cell = e.global::<u32>(TRACKED_CELL_011E0BB0);
                let manager = e.global::<u32>(CELL_MANAGER_011C3B3C);
                e.call(0x0044_6b50, &args![manager, cell]);
                let io = e.global::<u32>(IO_MANAGER);
                e.call(0x00c3_dfa0, &args![io, 0u32]);
            }
        }
        let status_slot = local(frame, -0x2c8);
        e.mem.set_u8(status_slot, 0);
        let progression = e.call(0x0046_4e10, &args![this]).u32();
        if e.call(0x008d_51f0, &args![progression]).bool()
            && !e.call(0x0095_3c50, &args![this, status_slot]).bool()
            && !e.call(0x0052_5430, &args![VATS_OBJECT]).bool()
            && e.call(0x0094_44a0, &args![]).bool()
            && e.call(0x0070_9be0, &args![]).bool()
            && e.call(0x0094_4270, &args![]).u8() == 0
            && !e.call(0x005c_7870, &args![this]).bool()
        {
            let progression = e.call(0x0046_4e10, &args![this]).u32();
            e.call(0x008d_5210, &args![progression]);
        }
        let reputation = e.get(this, PlayerCharacter::pReputationUpdate);
        if !reputation.is_null()
            && !e.call(0x0095_3c50, &args![this, status_slot]).bool()
            && !e.call(0x0052_5430, &args![VATS_OBJECT]).bool()
            && !e.call(0x0070_50d0, &args![]).bool()
            && !e.call(0x004a_4040, &args![]).bool()
        {
            let reputation = e.get(this, PlayerCharacter::pReputationUpdate);
            e.call(0x0061_55f0, &args![reputation]);
            e.set(this, PlayerCharacter::pReputationUpdate, Ptr::NULL);
        }
        if e.get(this.cast::<Actor>(), Actor::pCurrentProcess).addr() != 0 {
            let process = process_of(e, this);
            e.vcall(process, 0x104, &args![]);
        }
        e.call(0x0094_44d0, &args![this, delta]);
        let player = e.global::<u32>(PLAYER_SINGLETON);
        if e.call(0x0089_4900, &args![player]).bool() {
            let factor: f32 = e.global(FACE_FACTOR_011E0BAC);
            if (factor as f64) < one {
                e.set_global(FACE_FACTOR_011E0BAC, 1.0f32);
                let face = e.call(0x008a_dcb0, &args![this]).u32();
                if face != 0 {
                    e.vcall(face, 0x114, &args![0x0eu32, 1.0f32]);
                }
            }
        } else {
            let factor: f32 = e.global(FACE_FACTOR_011E0BAC);
            if factor as f64 > zero {
                e.set_global(FACE_FACTOR_011E0BAC, 0.0f32);
                let face = e.call(0x008a_dcb0, &args![this]).u32();
                if face != 0 {
                    let relax: f32 = e.global(FACE_RELAX_01016248);
                    e.vcall(face, 0xb4, &args![relax, 1u32, 0u32, 0u32, 0u32, 0u32]);
                }
            }
        }
        if looked_flag {
            let combat_limit = setting_float(e, COMBAT_SETTING_011CE3A8);
            let combat_timer = e.get(this, PlayerCharacter::fCombatTimer);
            let yield_timer = e.get(this, PlayerCharacter::fYieldTimer);
            if combat_limit as f64 > combat_timer as f64
                && yield_timer as f64 <= zero
                && e.call(0x0096_7da0, &args![this]).u32() != 0
            {
                let interval = setting_float(e, YIELD_SETTING_011CEDB8);
                e.set(this, PlayerCharacter::fYieldTimer, interval);
            }
        }
        let yield_timer = e.get(this, PlayerCharacter::fYieldTimer);
        if yield_timer as f64 > zero {
            let reduced = (yield_timer as f64 - delta as f64) as f32;
            e.set(this, PlayerCharacter::fYieldTimer, reduced);
            let floor = e.call(0x0040_4010, &args![reduced, 0.0f32]).f32();
            e.set(this, PlayerCharacter::fYieldTimer, floor);
        }
        let process = process_of(e, this);
        let target = e.vcall(process, 0x148, &args![]).u32();
        if target != 0 {
            let process = process_of(e, this);
            let target = e.vcall(process, 0x148, &args![]).u32();
            let target_state = e.call(0x0044_ddc0, &args![target]).u32(); // -0x3c0
            let process = process_of(e, this);
            let weapon = e.vcall(process, 0x148, &args![]).u32();
            let flag = e.call(0x004b_da70, &args![weapon, 6u32]).bool();
            let rate = e.call(0x0070_9430, &args![target_state, flag as u32]).f64();
            if rate > zero {
                let regen = e.call(0x0094_4300, &args![this]).f64();
                let elapsed = e.call(0x0084_d030, &args![GAME_TIMER]).f64();
                e.call(0x0094_42e0, &args![this, (regen - elapsed) as f32]);
                let process = process_of(e, this);
                let list = e.vcall(process, 0x14c, &args![]).u32();
                if list != 0 {
                    let regen = e.call(0x0094_4300, &args![this]).f64();
                    if regen <= zero {
                        let process = process_of(e, this);
                        let weapon = e.vcall(process, 0x148, &args![]).u32();
                        let flag = e.call(0x004b_da70, &args![weapon, 6u32]).bool();
                        let rate = e.call(0x0070_9430, &args![target_state, flag as u32]).f32();
                        e.call(0x0094_42e0, &args![this, rate]);
                        let process = process_of(e, this);
                        let list = e.vcall(process, 0x14c, &args![]).u32();
                        let count = e.call(0x0072_6070, &args![list]).i32();
                        let process = process_of(e, this);
                        let weapon = e.vcall(process, 0x148, &args![]).u32();
                        let flag = e.call(0x004b_da70, &args![weapon, 2u32]).bool();
                        let limit = e.call(0x004f_e160, &args![target_state, flag as u32]).i32();
                        if count < limit {
                            let next = count + 1;
                            let process = process_of(e, this);
                            let list = e.vcall(process, 0x14c, &args![]).u32();
                            e.call(0x006e_cd40, &args![list, next]);
                        }
                    }
                } else {
                    let regen = e.call(0x0094_4300, &args![this]).f64();
                    if regen <= zero {
                        e.with_stack(24, |e, object| {
                            e.call(0x0048_1610, &args![object]);
                            let player = e.global::<u32>(PLAYER_SINGLETON);
                            let first = e.call(0x0052_5980, &args![weapon_form, player]).u32();
                            e.call(0x0048_18e0, &args![object, first, 1u32, 0u32]);
                            let player = e.global::<u32>(PLAYER_SINGLETON);
                            e.call(0x0048_21a0, &args![object, player, 1u32]);
                            let player = e.global::<u32>(PLAYER_SINGLETON);
                            let second = e.call(0x0052_5980, &args![weapon_form, player]).u32();
                            let player = e.global::<u32>(PLAYER_SINGLETON);
                            e.call(
                                0x0088_c830,
                                &args![player, second, 1u32, 0u32, 1u32, 0u32, 0u32],
                            );
                            e.call(0x0048_1680, &args![object]);
                        });
                    }
                }
            }
        }
        if e.call(0x0049_38e0, &args![this]).bool() {
            let cell = e.call(0x008d_6f30, &args![this]).u32();
            if cell != 0 {
                let mut list = e.call(0x0096_04f0, &args![cell]).u32();
                let now = e.call(0x0045_7fe0, &args![]).u32();
                if e.global::<u32>(ALERT_TIME_011E0BA8) < now {
                    while list != 0 {
                        let item = e.call(LIST_ITEM_ADDRESS, &args![list]).u32();
                        if e.mem.u32(item) == 0 {
                            break;
                        }
                        let item = e.call(LIST_ITEM_ADDRESS, &args![list]).u32();
                        let reference = e.mem.u32(item);
                        if e.call(0x0040_1170, &args![reference]).u32() == 0x3a {
                            let item = e.call(LIST_ITEM_ADDRESS, &args![list]).u32();
                            let base = e.mem.u32(item);
                            let form = e.call(0x007a_f430, &args![base]).u32();
                            let cast = e
                                .call(
                                    0x00ec_43fb,
                                    &args![form, 0u32, 0x0118_3108u32, 0x0118_6568u32, 0u32],
                                )
                                .u32();
                            if cast != 0 && e.mem.i8(cast + 0x50) != -1 {
                                let item = e.call(LIST_ITEM_ADDRESS, &args![list]).u32();
                                let first = e.mem.u32(item);
                                if e.vcall(first, 0x1d0, &args![]).u32() != 0 {
                                    let item = e.call(LIST_ITEM_ADDRESS, &args![list]).u32();
                                    let second = e.mem.u32(item);
                                    let node = e.vcall(second, 0x1d0, &args![]).u32();
                                    let radius = e.call(0x0087_ce50, &args![node]).f32();
                                    let radius_squared = (radius as f64 * radius as f64) as f32;
                                    let item = e.call(LIST_ITEM_ADDRESS, &args![list]).u32();
                                    let third = e.mem.u32(item);
                                    let own = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                                    let other = e.vcall(third, 0x1f4, &args![]).u32();
                                    let offset = local(frame, -0x40c);
                                    let difference =
                                        e.call(0x0043_9ef0, &args![other, offset, own]).u32();
                                    let distance = e.call(0x004a_7290, &args![difference]).f64();
                                    if distance < radius_squared as f64 {
                                        let kind = e.mem.i8(cast + 0x50) as i32;
                                        let sound_form = e.call(0x005e_2a70, &args![kind]).u32();
                                        if sound_form != 0 {
                                            e.with_stack(12, |e, found| {
                                                let id =
                                                    e.call(0x0084_e3a0, &args![sound_form]).u32();
                                                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                                                let result = e
                                                    .call(
                                                        0x00ad_73b0,
                                                        &args![audio, found, id, 0x101u32],
                                                    )
                                                    .u32();
                                                e.call(SOUND_HANDLE_PLAY, &args![result, 0u32]);
                                                e.call(SOUND_HANDLE_DESTRUCT, &args![found]);
                                            });
                                            let now = e.call(0x0045_7fe0, &args![]).u32();
                                            let wait = e
                                                .call(0x0094_4460, &args![0x5dcu32, 0x7d0u32])
                                                .u32();
                                            e.set_global(
                                                ALERT_TIME_011E0BA8,
                                                now.wrapping_add(wait),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        list = e.call(LIST_NEXT, &args![list]).u32();
                    }
                }
            }
        }
        e.call(0x0096_4260, &args![this]);
        let beam = e
            .get(this.cast::<Actor>(), Actor::pContinuousBeamPersistant)
            .addr();
        if beam != 0 {
            let mut keep_beam = true; // -0x425
            let process = process_of(e, this);
            let list = e.vcall(process, 0x14c, &args![]).u32();
            if list == 0 {
                keep_beam = false;
            } else {
                let count = e.call(0x0072_6070, &args![list]).i32();
                let limit = e.call(0x0052_4b60, &args![weapon_form]).u8() as i32;
                if count < limit {
                    keep_beam = false;
                }
            }
            if !keep_beam
                || e.call(0x00a2_4660, &args![main_state, 4u32, 2u32]).u32() != 0
                || e.call(0x00a2_4660, &args![main_state, 4u32, 0u32]).u32() == 0
            {
                e.call(0x009a_b9a0, &args![beam, this]);
            }
            let beam = e
                .get(this.cast::<Actor>(), Actor::pContinuousBeamPersistant)
                .addr();
            e.vcall(beam, 0x310, &args![0.0f32]);
        }
        let last_hello = e.get(this, PlayerCharacter::fLastHelloTime);
        if last_hello as f64 > zero {
            let now = e.call(0x0045_7fe0, &args![]).u32();
            let elapsed = now as f64 - last_hello as f64;
            let limit = setting_float(e, HELLO_SETTING_011D03A0);
            let scale: f64 = e.global(HELLO_SCALE);
            if (limit as f64 * scale) < elapsed {
                let reset: f32 = e.global(TIMER_RESET);
                e.set(this, PlayerCharacter::fLastHelloTime, reset);
            }
        }
    }
    // Every exit of the function (the early ones have already done the same) ends
    // the sound handle and the scope guard.
    e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    e.call(0x0040_4ee0, &args![guard]);
}
// Translated from 00944270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `+0x4CD` of the object the global `011d8a80` points to
/// (0 when it is null).
pub fn fn_00944270(e: &mut Engine) -> u8 {
    let object = e.global::<u32>(0x011d_8a80);
    if object == 0 {
        0
    } else {
        e.mem.u8(object + 0x4cd)
    }
}

// Translated from 009442a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float at `+0x10C` of the object.
pub fn fn_009442a0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr().wrapping_add(0x10c))
}

// Translated from 009442c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x1000` of the word at `+0x12C` of the object is set.
pub fn fn_009442c0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(0x12c)) & 0x1000 != 0
}

// Translated from 009442e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float at `+0x1F4` (for the player `fTimeSinceLastAmmoRegenTick`).
pub fn fn_009442e0(e: &mut Engine, this: Ptr<PlayerCharacter>, value: f32) {
    e.set(
        this,
        PlayerCharacter::fTimeSinceLastAmmoRegenTick,
        x87_quiet(value),
    );
}

/// Allocates 8 bytes and builds a `BSSimpleList` in them with `0096a2d0`,
/// the way the constructor creates each heap list (null when the allocation
/// fails). Returns what the list constructor returns.
fn new_list(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    if block != 0 {
        e.call(LIST_CONSTRUCT, &args![block]).u32()
    } else {
        0
    }
}

// Translated from 00938180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::PlayerCharacter` (Xbox PDB): constructs the player in
/// the 0xE50 bytes at `this` and returns it. Runs the `Actor` base
/// constructor (`008d1d30`), installs the six vtables, constructs every
/// embedded member (lists, smart pointers, sound handles, the per-perk lists)
/// and gives every field its initial value; it also creates the player's
/// process (a 0x46C-byte object), the `CameraCaster`, the active-effect and
/// other heap lists, and the two 0x1A0-byte objects the globals
/// `011e0774` and `011e0760` point to.
pub fn player_character_construct(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
) -> Ptr<PlayerCharacter> {
    let t = this.addr();
    e.call(0x008d_1d30, &args![this]);
    e.mem.set_u32(t, 0x0108_aa3c);
    e.mem.set_u32(t + 0x18, 0x0108_aa30);
    e.mem.set_u32(t + 0x88, 0x0108_a9dc);
    e.mem.set_u32(t + 0x94, 0x0108_a9a4);
    e.mem.set_u32(t + 0xa4, 0x0108_a974);
    e.mem.set_u32(t + 0xa8, 0x0108_a92c);

    // Embedded members, in declaration order.
    e.call(
        LIST_CONSTRUCT,
        &args![member(this, PlayerCharacter::listNotes)],
    );
    e.call(
        LIST_CONSTRUCT,
        &args![member(this, PlayerCharacter::pListOfTeammates)],
    );
    for field in [
        PlayerCharacter::spGrabSpring,
        PlayerCharacter::sp1stPerson3D,
        PlayerCharacter::spInventoryPC,
    ] {
        e.call(NI_POINTER_CONSTRUCT, &args![member(this, field), 0u32]);
    }
    for field in [
        PlayerCharacter::listTopics,
        PlayerCharacter::listQuestLog,
        PlayerCharacter::listObjectives,
        PlayerCharacter::listQuestTargets,
    ] {
        e.call(LIST_CONSTRUCT, &args![member(this, field)]);
    }
    e.call(
        0x006f_48b0,
        &args![member(this, PlayerCharacter::PlayerMarkerPath)],
    );
    e.call(
        0x004f_6320,
        &args![member(this, PlayerCharacter::m_AllOccupiedRegions), 1u32],
    );
    e.call(
        LIST_CONSTRUCT,
        &args![member(this, PlayerCharacter::CurrentRegionSoundList)],
    );
    e.call(
        SOUND_HANDLE_CONSTRUCT,
        &args![member(this, PlayerCharacter::StatusSoundHandle)],
    );
    e.call(
        LIST_ITEM_ADDRESS,
        &args![member(this, PlayerCharacter::LastKnownGoodPosition)],
    );
    e.call(
        LIST_CONSTRUCT,
        &args![member(this, PlayerCharacter::MapMarkerList)],
    );
    e.call(
        LIST_CONSTRUCT,
        &args![member(this, PlayerCharacter::AudioMarkerList)],
    );
    e.call(
        LIST_ITEM_ADDRESS,
        &args![member(this, PlayerCharacter::UFOCameraPos)],
    );
    e.call(
        SOUND_HANDLE_CONSTRUCT,
        &args![member(this, PlayerCharacter::SelectedSpellCastSound)],
    );
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            member(this, PlayerCharacter::MagicFailureSounds),
            0xcu32,
            6u32,
            SOUND_HANDLE_CONSTRUCT,
            SOUND_HANDLE_DESTRUCT
        ],
    );
    e.call(
        LIST_CONSTRUCT,
        &args![member(this, PlayerCharacter::DroppedRefList)],
    );
    e.call(
        0x0096_a280,
        &args![member(this, PlayerCharacter::RandomDoorSpaceMap), 0x25u32],
    );
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![member(this, PlayerCharacter::sp1stPersonLight), 0u32],
    );
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![member(this, PlayerCharacter::sp3rdPersonLight), 0u32],
    );
    e.call(
        0x008d_4360,
        &args![member(this, PlayerCharacter::CharacterProgressionInfo)],
    );
    e.call(LIST_CONSTRUCT, &args![member(this, PlayerCharacter::Perks)]);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            member(this, PlayerCharacter::PerkEntryLists),
            8u32,
            0x4au32,
            LIST_CONSTRUCT,
            LIST_DESTRUCT
        ],
    );
    e.call(
        LIST_CONSTRUCT,
        &args![member(this, PlayerCharacter::CompanionPerks)],
    );
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            member(this, PlayerCharacter::CompanionPerkEntryLists),
            8u32,
            0x4au32,
            LIST_CONSTRUCT,
            LIST_DESTRUCT
        ],
    );
    e.call(
        LIST_ITEM_ADDRESS,
        &args![member(this, PlayerCharacter::BulletAutoAim)],
    );
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![member(this, PlayerCharacter::spTargeted3D), 0u32],
    );
    for field in [
        PlayerCharacter::kCamera3rdPersonShoulderOffset,
        PlayerCharacter::Cached1stPersonCameraPos,
        PlayerCharacter::CachedWorldCameraPos,
    ] {
        e.call(LIST_ITEM_ADDRESS, &args![member(this, field)]);
    }
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![member(this, PlayerCharacter::spCameraRigidBody), 0u32],
    );
    e.call(
        0x0096_a570,
        &args![member(this, PlayerCharacter::RockItLauncherAmmoList)],
    );
    e.call(
        0x0096_a5c0,
        &args![member(this, PlayerCharacter::HotKeyLastAmmo)],
    );

    // A scope guard the game keeps on its stack for the rest of the body
    // (constructed with `00404eb0`, destroyed with `00404ee0`).
    e.with_stack(4, |e, guard| {
        e.call(
            0x0040_4eb0,
            &args![guard, 0x34u32, 1u32, 0x0108_a8e0u32, 0x2c7u32],
        );
        e.call(0x0048_4ab0, &args![this, 0u32]);
        set_zero_vector(e, member(this, PlayerCharacter::Cached1stPersonCameraPos));
        set_zero_vector(e, member(this, PlayerCharacter::CachedWorldCameraPos));
        e.call(0x008d_66b0, &args![]);
        e.call(0x008d_1820, &args![]);
        let setting: u32 = e.call(SETTING_FLOAT_GETTER, &args![0x011c_cf2cu32]).u32();
        let value = e.mem.f32(setting);
        e.set_global(0x011e_0b5c, value);
        e.set(this, PlayerCharacter::bAiControlledActivate, false);

        for i in 0..0x4d {
            e.mem.set_f32(
                member(this, PlayerCharacter::TemporaryActorValueModifiers) + 4 * i,
                0.0,
            );
            e.mem.set_f32(
                member(this, PlayerCharacter::ScriptActorValueModifiers) + 4 * i,
                0.0,
            );
            e.mem.set_f32(
                member(this, PlayerCharacter::DamageActorValueModifiers) + 4 * i,
                0.0,
            );
        }
        e.set(this, PlayerCharacter::fTimeSinceLastAmmoRegenTick, 0.0);
        e.set(this, PlayerCharacter::cShotsSinceLastAmmoRegen, 0);
        e.set(this, PlayerCharacter::fHealthModifier, 0.0);
        e.set(this, PlayerCharacter::iSleepTime, 0);
        e.set(this, PlayerCharacter::fsecondRunning, 0.0);
        e.set(this, PlayerCharacter::fsecondSwimming, 0.0);
        e.set(this, PlayerCharacter::fsecondSneaking, 0.0);
        e.set(this, PlayerCharacter::iAmountStolenSold, 0);
        e.set(this, PlayerCharacter::iNumAdvance, 0);
        e.set(this, PlayerCharacter::btransporting, false);
        e.set(this, PlayerCharacter::bCanFastTravel, false);
        e.call(0x005d_14d0, &args![this, 1u32]);
        e.set(this, PlayerCharacter::bCanWait, true);
        e.set(this, PlayerCharacter::bChargen, false);
        e.set(this, PlayerCharacter::bAllowEGMCacheClear, true);
        e.set(this, PlayerCharacter::eskilladvance, 0x4d);
        e.set(this, PlayerCharacter::pDefaultClass, Ptr::NULL);
        e.set(this, PlayerCharacter::pListofActions, Ptr::NULL);
        e.set(this, PlayerCharacter::pAutoAimActor, Ptr::NULL);
        set_zero_vector(e, member(this, PlayerCharacter::BulletAutoAim));
        e.set(this, PlayerCharacter::pPlayersTargetActor, Ptr::NULL);
        e.set(this, PlayerCharacter::bAiControlledPackage, false);
        e.set(this, PlayerCharacter::pClassBasedOn, Ptr::NULL);
        e.set(this, PlayerCharacter::bTravelUseDoor, false);
        e.call(
            NI_POINTER_ASSIGN,
            &args![member(this, PlayerCharacter::sp1stPersonLight), 0u32],
        );
        e.call(
            NI_POINTER_ASSIGN,
            &args![member(this, PlayerCharacter::sp3rdPersonLight), 0u32],
        );
        e.set(this, PlayerCharacter::bOnElevator, true);

        // The player's process: a 0x46C-byte object built here, told about the
        // process it replaces (virtual +4), registered with the process lists
        // before and after the swap, while the old one is destroyed.
        let block = e.call(OPERATOR_NEW, &args![0x46cu32]).u32();
        let process = if block != 0 {
            e.call(0x008d_7510, &args![block]).u32()
        } else {
            0
        };
        let old_process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        e.vcall(process, 4, &args![old_process]);
        let key = e.call(0x0093_1850, &args![this]).u32();
        e.call(0x0096_d470, &args![PROCESS_LISTS, this, key]);
        let old_process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        if old_process != 0 {
            e.vcall(old_process, 0, &args![1u32]);
        }
        e.mem.set_u32(at(this, ACTOR_CURRENT_PROCESS), process);
        let key = e.call(0x0093_1850, &args![this]).u32();
        e.call(0x0096_d470, &args![PROCESS_LISTS, this, key]);

        let caster_base: Ptr<MagicCasterBase> = Ptr::new(t + MAGIC_CASTER_BASE);
        fn_00939800(e, caster_base, 0);
        fn_00939840(e, caster_base, 0);

        let effects = new_list(e);
        e.set(this, PlayerCharacter::pActiveEffectList, Ptr::new(effects));

        e.call(0x0093_0c70, &args![this]);
        let camera_owner: Ptr = e.call(0x0093_06d0, &args![this]).ptr();
        if !camera_owner.is_null() {
            fn_00939860(e, camera_owner, 0x3e8);
            e.call(0x0045_34f0, &args![this, 1u32]);
            let index = 9u32;
            e.call(0x00c6_d770, &args![camera_owner, index]);
            e.with_stack(4, |e, out| {
                e.call(0x0070_c440, &args![camera_owner, out]);
            });
            let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
            let caster = if block != 0 {
                let setting = e.call(SETTING_FLOAT_GETTER, &args![0x011e_0934u32]).u32();
                let value = e.mem.f32(setting);
                e.call(0x0062_0850, &args![block, value, index]).u32()
            } else {
                0
            };
            e.set(this, PlayerCharacter::pCameraCaster, Ptr::new(caster));
        } else {
            e.set(this, PlayerCharacter::pCameraCaster, Ptr::NULL);
        }
        e.set(this, PlayerCharacter::b3rdPersonSaved, false);
        e.set(this, PlayerCharacter::bsave3rdPerson, false);
        e.set(this, PlayerCharacter::b3rdPerson, false);
        e.set(this, PlayerCharacter::bActually3rdPerson, false);
        e.set(this, PlayerCharacter::bWant3rdPerson, false);
        e.set(this, PlayerCharacter::bTemp3rdPerson, false);
        e.set(this, PlayerCharacter::bTemp3rdPersonSwitchBack, false);
        e.set(this, PlayerCharacter::bTemp1stPerson, false);
        e.set(this, PlayerCharacter::bTemp1stPersonSwitchBack, false);
        let setting = e.call(SETTING_BYTE_GETTER, &args![0x011e_0b4cu32]).u32();
        let always_run = e.mem.u8(setting);
        e.set(this, PlayerCharacter::bAlwaysRun, always_run != 0);
        e.set(this, PlayerCharacter::bAutoMove, false);
        let setting = e.call(SETTING_FLOAT_GETTER, &args![0x0120_3150u32]).u32();
        let value = e.mem.f32(setting);
        e.call(0x0095_0610, &args![this, value]);
        e.set(this, PlayerCharacter::p1stPersonBipedAnim, Ptr::NULL);
        e.set(this, PlayerCharacter::p1stPersonAnimation, Ptr::NULL);
        e.call(
            NI_POINTER_ASSIGN,
            &args![member(this, PlayerCharacter::sp1stPerson3D), 0u32],
        );
        let eye_height = e.global::<f32>(EYE_HEIGHT_DEFAULT);
        e.set(this, PlayerCharacter::fEyeHeight, eye_height);
        e.call(
            NI_POINTER_ASSIGN,
            &args![member(this, PlayerCharacter::spInventoryPC), 0u32],
        );
        e.set(this, PlayerCharacter::pInventoryAnimation, Ptr::NULL);
        e.set(this, PlayerCharacter::pInventoryAnimation, Ptr::NULL);
        set_zero_vector_global(e);
        e.set_global(0x011e_0768, 0.0f32);
        e.set_global(0x011e_07c3, 0u8);
        e.set(this, PlayerCharacter::bGreetingPlayer, false);
        e.set(this, PlayerCharacter::ucControlsDisabled, 0);
        e.set(this, PlayerCharacter::pClosestConversation, Ptr::NULL);
        e.set(this, PlayerCharacter::bBlockActivate, false);
        let timer = e.global::<f32>(BLOCK_ACTIVATE_TIMER_RESET);
        e.set(this, PlayerCharacter::fBlockActivateTimer, timer);
        e.set(this, PlayerCharacter::ihourstosleep, 0);
        e.set(this, PlayerCharacter::fSitHeadingDelta, 0.0);
        e.set(this, PlayerCharacter::cMurder, 0);
        e.set(this, PlayerCharacter::bBeenAttacked, false);
        e.set(this, PlayerCharacter::pSelectedSpell, Ptr::NULL);
        e.set(this, PlayerCharacter::pSelectedScroll, Ptr::NULL);
        e.set(this, PlayerCharacter::pPlayerMapMarker, Ptr::NULL);
        e.set(this, PlayerCharacter::pActiveQuest, Ptr::NULL);

        let eat_drink = new_list(e);
        e.set(this, PlayerCharacter::EatDrinkItems, Ptr::new(eat_drink));
        e.set(this, PlayerCharacter::QueuedWornEnchantments, Ptr::NULL);
        e.set(this, PlayerCharacter::iNumberTraining, 0);
        e.call(0x0051_9020, &args![this, 0u32]);
        e.set(this, PlayerCharacter::bIsSleeping, false);
        e.set(this, PlayerCharacter::pPendingPoison, Ptr::NULL);
        e.set(this, PlayerCharacter::bTelekinesisSelected, false);
        e.set(this, PlayerCharacter::pOccupiedRegion, Ptr::NULL);
        e.call(
            NI_POINTER_ASSIGN,
            &args![member(this, PlayerCharacter::spGrabSpring), 0u32],
        );
        e.set(this, PlayerCharacter::pGrabbedObject, Ptr::NULL);
        e.set(this, PlayerCharacter::eGrabType, 0);
        e.set(this, PlayerCharacter::fGrabObjectWeight, 0.0);
        e.set(this, PlayerCharacter::fGrabDistance, 0.0);
        e.set(this, PlayerCharacter::pInitialStateBuffer, Ptr::NULL);
        e.set(this, PlayerCharacter::iTotalPlayingTime, 0);
        e.call(0x0085_1d10, &args![this]);
        for i in 0..5 {
            e.mem
                .set_u32(member(this, PlayerCharacter::pCrimeCounts) + 4 * i, 0);
        }
        let seed = e.call(0x0061_b9d0, &args![0u32]).u32();
        e.set(this, PlayerCharacter::iCharacterSeed, seed);
        fn_0093a5f0(e, this, false);
        fn_0093a6f0(e, this, false);
        e.set(this, PlayerCharacter::bInBorderContainedCell, false);
        e.set(this, PlayerCharacter::bReturnToLastKnownGoodPosition, false);
        set_zero_vector(e, member(this, PlayerCharacter::LastKnownGoodPosition));
        e.set(this, PlayerCharacter::pLastKnownGoodLocation, Ptr::NULL);
        e.set(this, PlayerCharacter::pLastKnownMusicType, Ptr::NULL);
        e.set(this, PlayerCharacter::pBorderRegions, Ptr::NULL);

        let casino = new_list(e);
        e.set(this, PlayerCharacter::pListofCasinoData, Ptr::new(casino));
        let inactive = new_list(e);
        e.set(
            this,
            PlayerCharacter::pInactiveListofCaravanCards,
            Ptr::new(inactive),
        );
        let active = new_list(e);
        e.set(
            this,
            PlayerCharacter::pActiveListofCaravanCards,
            Ptr::new(active),
        );
        e.set(this, PlayerCharacter::iCaravanCapWinnings, 0);
        e.set(this, PlayerCharacter::iCaravanCapLosses, 0);
        e.set(this, PlayerCharacter::iCaravanWinnings, 0);
        e.set(this, PlayerCharacter::iCaravanLosses, 0);
        e.set(this, PlayerCharacter::iCaravanLargestWinning, 0);
        e.set(this, PlayerCharacter::iCasinoCheatLevel, 0);
        e.call(0x0096_9e90, &args![this, 0u32, 0u32]);
        e.set(this, PlayerCharacter::bAlwaysHardcore, true);
        let setting = e.call(SETTING_INT_GETTER, &args![0x011e_0940u32]).u32();
        let difficulty = e.mem.u32(setting);
        e.set(this, PlayerCharacter::eDifficultyLevel, difficulty);
        let setting = e.call(SETTING_INT_GETTER, &args![0x011e_087cu32]).u32();
        let kill_camera = e.mem.u32(setting);
        e.set(this, PlayerCharacter::eKillCameraSetting, kill_camera);
        e.set(this, PlayerCharacter::pInventoryWeaponEffect, Ptr::NULL);
        e.set(this, PlayerCharacter::bBeingChased, false);
        e.set(this, PlayerCharacter::bIsYoung, false);
        e.set(this, PlayerCharacter::bIsToddler, false);
        e.set(this, PlayerCharacter::bCanUsePowerArmor, false);
        e.set(this, PlayerCharacter::iCombatPersue, 0);
        e.set(this, PlayerCharacter::pMapWorld, Ptr::NULL);
        e.set(this, PlayerCharacter::fUFOCameraHeading, 0.0);
        e.set(this, PlayerCharacter::fUFOCameraPitch, 0.0);
        set_zero_vector(e, member(this, PlayerCharacter::UFOCameraPos));
        e.set(this, PlayerCharacter::bShowQuestItemsInInventory, true);
        e.set(this, PlayerCharacter::iSelectedSpellCastSoundID, 0);

        // The selected-spell sound and the six failure sounds are replaced by
        // freshly constructed (empty) handles.
        e.with_stack(0xc, |e, handle| {
            let fresh = e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]).u32();
            e.call(
                SOUND_HANDLE_ASSIGN,
                &args![member(this, PlayerCharacter::SelectedSpellCastSound), fresh],
            );
            e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
        });
        for i in 0..6u32 {
            e.with_stack(0xc, |e, handle| {
                let fresh = e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]).u32();
                e.call(
                    SOUND_HANDLE_ASSIGN,
                    &args![
                        member(this, PlayerCharacter::MagicFailureSounds) + i * 0xc,
                        fresh
                    ],
                );
                e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
            });
        }
        e.set(this, PlayerCharacter::pLastExtDoorActivated, Ptr::NULL);
        e.call(
            LIST_CLEAR,
            &args![member(this, PlayerCharacter::DroppedRefList)],
        );
        e.call(0x0096_27f0, &args![this]);
        e.call(0x004e_d780, &args![0x011c_d8f0u32, 0.0f32]);
        e.set(this, PlayerCharacter::fDropAngleMod, 0.0);
        e.set(this, PlayerCharacter::fLastDropAngleMod, 0.0);
        let flags = e.call(0x0044_ddc0, &args![this]).u32() | 0x400;
        e.call(0x0040_3550, &args![this, flags]);
        let setting = e.call(SETTING_FLOAT_GETTER, &args![0x0120_315cu32]).u32();
        let value = e.mem.f32(setting);
        e.set(this, PlayerCharacter::fWorldFOV, value);
        let setting = e.call(SETTING_FLOAT_GETTER, &args![0x011c_da58u32]).u32();
        let value = e.mem.f32(setting);
        e.set(this, PlayerCharacter::f3rdPersonFOV, value);
        let setting = e.call(SETTING_FLOAT_GETTER, &args![0x0120_3168u32]).u32();
        let value = e.mem.f32(setting);
        e.set(this, PlayerCharacter::f1stPersonFOV, value);
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        e.vcall(process, 0x6e0, &args![0u32]);
        e.call(0x0096_4190, &args![this]);
        e.set(this, PlayerCharacter::fProjectileReleaseTimer, 0.0);
        e.set(this, PlayerCharacter::iNumberofStealWarnings, 0);
        let timer = e.global::<f32>(TIMER_RESET);
        e.set(this, PlayerCharacter::fStealWarningTimer, timer);
        e.set(this, PlayerCharacter::iNumberofPickpocketWarnings, 0);
        e.set(this, PlayerCharacter::fPickPocketWarningTimer, timer);
        e.set(this, PlayerCharacter::pListofPercievedActors, Ptr::NULL);
        e.set(this, PlayerCharacter::bMenuModeButtonClicked, false);
        e.set(this, PlayerCharacter::fMenuModeButtonTimer, 0.0);
        e.set(this, PlayerCharacter::fAmmoSwapButtonTimer, 0.0);
        e.set(this, PlayerCharacter::pIronsightsDOFInstance, Ptr::NULL);
        e.set(this, PlayerCharacter::pVatsDOFInstance, Ptr::NULL);
        e.set(this, PlayerCharacter::pVatsDRBInstance, Ptr::NULL);
        set_zero_vector(
            e,
            member(this, PlayerCharacter::kCamera3rdPersonShoulderOffset),
        );
        e.set(this, PlayerCharacter::bSpeaking, false);
        e.set(this, PlayerCharacter::pQueuedTargetLoc, Ptr::NULL);
        e.set(this, PlayerCharacter::pQueuedWeaponAttach, Ptr::NULL);
        e.set(this, PlayerCharacter::pCombatGroup, Ptr::NULL);
        e.set(this, PlayerCharacter::iTeammateCount, 0);
        e.set(this, PlayerCharacter::bPlayerInCombat, false);
        e.set(this, PlayerCharacter::bAllCombatTargetsSearching, false);
        e.set(this, PlayerCharacter::fCombatTimer, 0.0);
        e.set(this, PlayerCharacter::fYieldTimer, 0.0);
        e.call(
            NI_POINTER_ASSIGN,
            &args![member(this, PlayerCharacter::spCameraRigidBody), 0u32],
        );
        let value = e.call(0x0054_6a90, &args![]).u32();
        e.call(0x0056_7dd0, &args![this, value]);
        e.call(0x005d_add0, &args![this, 0u32]);

        // Two 0x1A0-byte objects, held by globals.
        for global in [0x011e_0774u32, 0x011e_0760] {
            let block = e.call(OPERATOR_NEW, &args![0x1a0u32]).u32();
            let object = if block != 0 {
                fn_00939880(e, Ptr::new(block)).addr()
            } else {
                0
            };
            e.set_global(global, object);
        }

        e.call(
            MEMSET,
            &args![member(this, PlayerCharacter::pWobbleNodes), 0u32, 0x60u32],
        );
        e.call(0x0096_9110, &args![this, 0u32]);
        e.set(this, PlayerCharacter::pAIConversationRunning, Ptr::NULL);
        e.set(this, PlayerCharacter::bHostileDetection, false);
        e.set(this, PlayerCharacter::iSandmanDetection, 0);
        e.call(
            NI_POINTER_ASSIGN,
            &args![member(this, PlayerCharacter::spTargeted3D), 0u32],
        );
        e.set(this, PlayerCharacter::bTarget3DDistant, false);
        e.set(this, PlayerCharacter::bNightVisionOn, false);
        e.set(this, PlayerCharacter::bTurboISM, false);
        e.set(this, PlayerCharacter::pReputationUpdate, Ptr::NULL);
        e.set(this, PlayerCharacter::fTimeInSlowMoCam, 0.0);
        e.set(this, PlayerCharacter::fKillCamCooldown, 0.0);
        e.set(this, PlayerCharacter::bIgnoresGTM, false);
        e.mem.set_u32(at(this, ACTOR_CONTINUOUS_BEAM_PERSISTANT), 0);
        e.set(this, PlayerCharacter::bIsAcousticSpaceTransition, false);
        let hello = e.global::<f32>(TIMER_RESET);
        e.set(this, PlayerCharacter::fLastHelloTime, hello);
        for i in 0..3u32 {
            let entry = member(this, PlayerCharacter::PlayerAchievements) + i * 0xc;
            e.mem.set_u8(entry, 0);
            e.mem.set_u32(entry + 4, 0);
            let flags = e.mem.i8(ACHIEVEMENT_TABLE + i * 0x28) as i32;
            e.mem.set_u8(entry + 8, (!(flags & 2) != 0) as u8);
        }
        e.set(this, PlayerCharacter::pClosestAudioMarkerInfo, Ptr::NULL);
        e.set(this, PlayerCharacter::fCounterAttackTimer, 0.0);
        e.set(this, PlayerCharacter::bCounterAttackCamera, false);
        e.set(this, PlayerCharacter::bHasCateyeActive, false);
        e.set(this, PlayerCharacter::bHasSpotterActive, false);
        e.set(this, PlayerCharacter::fCheckForItems, 0.0);
        e.call(0x008b_bbf0, &args![this, 0u32]);
        for _ in 0..8 {
            e.with_stack(4, |e, null| {
                e.call(
                    0x007c_b2e0,
                    &args![member(this, PlayerCharacter::HotKeyLastAmmo), null],
                );
            });
        }
        e.set(this, PlayerCharacter::pcOnlyWord, 0);
        e.call(0x0040_4ee0, &args![guard]);
    });
    this
}

/// How the destructor releases each item of one of the player's heap lists.
#[derive(Clone, Copy)]
enum ItemRelease {
    /// `operator delete` on the item (the loop stops at a null item).
    Delete,
    /// The item's scalar deleting destructor (virtual `+0`, argument 1),
    /// skipped for a null item.
    VirtualDestructor,
    /// The item's virtual `+0x10` with argument 1; the loop stops at a null
    /// item.
    VirtualSlot10,
}

/// Releases the items of one of the player's heap lists (see
/// [`ItemRelease`]): takes the head node's item, releases it and removes the
/// head node (`0063f7b0`) until the list is gone or finished, then deletes the
/// list object (`004702f0`). `check_empty_first` skips the whole step when
/// `008256d0` says the list is empty; `check_empty_each_pass` ends the loop
/// when it says so.
fn destroy_heap_list(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    slot: Field<PlayerCharacter, Ptr>,
    release: ItemRelease,
    check_empty_each_pass: bool,
    check_empty_first: bool,
) {
    if e.get(this, slot).is_null() {
        return;
    }
    if check_empty_first {
        let list = e.get(this, slot);
        if e.call(LIST_IS_EMPTY, &args![list]).bool() {
            return;
        }
    }
    loop {
        let list = e.get(this, slot);
        if list.is_null() {
            break;
        }
        if check_empty_each_pass && e.call(LIST_IS_EMPTY, &args![list]).bool() {
            break;
        }
        let node = e.call(LIST_ITEM_ADDRESS, &args![list]).u32();
        let item = e.mem.u32(node);
        match release {
            ItemRelease::Delete => {
                if item == 0 {
                    break;
                }
                let list = e.get(this, slot);
                let node = e.call(LIST_ITEM_ADDRESS, &args![list]).u32();
                let item = e.mem.u32(node);
                e.call(OPERATOR_DELETE, &args![item]);
            }
            ItemRelease::VirtualDestructor => {
                if item != 0 {
                    e.vcall(item, 0, &args![1u32]);
                }
            }
            ItemRelease::VirtualSlot10 => {
                if item == 0 {
                    break;
                }
                let list = e.get(this, slot);
                let node = e.call(LIST_ITEM_ADDRESS, &args![list]).u32();
                let item = e.mem.u32(node);
                if item != 0 {
                    e.vcall(item, 0x10, &args![1u32]);
                }
            }
        }
        let list = e.get(this, slot);
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
    let list = e.get(this, slot);
    if !list.is_null() {
        e.call(LIST_DELETING_DESTRUCT, &args![list, 1u32]);
    }
}

/// Frees the item of every node of the embedded list at `list` with
/// `operator delete`, walking with `00726070`.
fn free_embedded_list_items(e: &mut Engine, list: u32) {
    let mut node = list;
    while node != 0 {
        let item_address = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(item_address);
        e.call(OPERATOR_DELETE, &args![item]);
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
}

// Translated from 009398d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body of `PlayerCharacter::~PlayerCharacter` (Xbox PDB name of the
/// function the engine map does not name): resets the six vtables to this
/// class's, releases the player's process, the camera caster, every heap list
/// and its items, the sounds, the per-perk lists and the two 0x1A0-byte
/// objects the constructor created, destroys the embedded members in reverse
/// order and runs the `Actor` base destructor (`008d2060`). The compiler's
/// exception-unwinding state writes are not translated.
pub fn fn_009398d0(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let t = this.addr();
    e.mem.set_u32(t, 0x0108_aa3c);
    e.mem.set_u32(t + 0x18, 0x0108_aa30);
    e.mem.set_u32(t + 0x88, 0x0108_a9dc);
    e.mem.set_u32(t + 0x94, 0x0108_a9a4);
    e.mem.set_u32(t + 0xa4, 0x0108_a974);
    e.mem.set_u32(t + 0xa8, 0x0108_a92c);

    e.call(0x0094_7d10, &args![this]);
    e.call(0x0094_8050, &args![this]);
    e.call(LIST_CLEAR, &args![t + 0x768]);
    e.call(0x008d_68d0, &args![]);
    e.call(0x008d_1920, &args![]);
    e.call(0x0095_0c20, &args![this, 0u32]);

    // The process.
    let process = e.mem.u32(t + ACTOR_CURRENT_PROCESS);
    if process != 0 {
        e.vcall(process, 0, &args![1u32]);
    }
    e.mem.set_u32(t + ACTOR_CURRENT_PROCESS, 0);

    // Lists of the eat/drink items and queued worn enchantments.
    for slot in [
        PlayerCharacter::EatDrinkItems,
        PlayerCharacter::QueuedWornEnchantments,
    ] {
        let list = e.get(this, slot);
        if !list.is_null() {
            e.call(LIST_CLEAR, &args![list]);
            let list = e.get(this, slot);
            if !list.is_null() {
                e.call(LIST_DELETING_DESTRUCT, &args![list, 1u32]);
            }
        }
    }

    // The camera caster.
    let caster = e.get(this, PlayerCharacter::pCameraCaster);
    if !caster.is_null() {
        camera_caster_vector_deleting_destructor(e, caster, 1);
    }

    e.call(
        NI_POINTER_ASSIGN,
        &args![member(this, PlayerCharacter::spGrabSpring), 0u32],
    );
    let selected_spell = e.get(this, PlayerCharacter::pSelectedSpell);
    if !selected_spell.is_null() {
        e.call(0x0040_b800, &args![selected_spell, 1u32]);
    }

    destroy_heap_list(
        e,
        this,
        PlayerCharacter::pActiveEffectList,
        ItemRelease::VirtualDestructor,
        true,
        false,
    );
    destroy_heap_list(
        e,
        this,
        PlayerCharacter::pListofActions,
        ItemRelease::Delete,
        false,
        false,
    );
    destroy_heap_list(
        e,
        this,
        PlayerCharacter::pListofCasinoData,
        ItemRelease::Delete,
        false,
        true,
    );
    destroy_heap_list(
        e,
        this,
        PlayerCharacter::pInactiveListofCaravanCards,
        ItemRelease::VirtualSlot10,
        false,
        true,
    );
    destroy_heap_list(
        e,
        this,
        PlayerCharacter::pActiveListofCaravanCards,
        ItemRelease::VirtualSlot10,
        false,
        true,
    );

    e.call(0x008b_3180, &args![this]);
    for field in [
        PlayerCharacter::listTopics,
        PlayerCharacter::listQuestLog,
        PlayerCharacter::listQuestTargets,
        PlayerCharacter::listObjectives,
    ] {
        e.call(LIST_CLEAR, &args![member(this, field)]);
    }
    e.call(0x0096_7290, &args![this]);
    e.call(0x0096_90a0, &args![this]);

    let weapon_effect = e.get(this, PlayerCharacter::pInventoryWeaponEffect);
    if !weapon_effect.is_null() {
        e.vcall(weapon_effect.addr(), 0, &args![1u32]);
    }

    e.call(0x0094_eb40, &args![this, 0u32, 1u32]);
    let initial_state = e.get(this, PlayerCharacter::pInitialStateBuffer);
    if !initial_state.is_null() {
        let object = e.global::<u32>(0x011d_e45c);
        e.call(0x0085_8700, &args![object, initial_state]);
    }
    let border_regions = e.get(this, PlayerCharacter::pBorderRegions);
    if !border_regions.is_null() {
        e.call(0x005e_03d0, &args![border_regions]);
        let border_regions = e.get(this, PlayerCharacter::pBorderRegions);
        if !border_regions.is_null() {
            e.vcall(border_regions.addr(), 0, &args![1u32]);
        }
        e.set(this, PlayerCharacter::pBorderRegions, Ptr::NULL);
    }
    e.call(0x0096_1f90, &args![this]);

    // Stop and clear the six failure sounds.
    for i in 0..6u32 {
        let handle = member(this, PlayerCharacter::MagicFailureSounds) + i * 0xc;
        if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
            e.call(SOUND_HANDLE_STOP, &args![handle]);
            e.with_stack(0xc, |e, empty| {
                let fresh = e.call(SOUND_HANDLE_CONSTRUCT, &args![empty]).u32();
                e.call(SOUND_HANDLE_ASSIGN, &args![handle, fresh]);
                e.call(SOUND_HANDLE_DESTRUCT, &args![empty]);
            });
        }
    }

    e.call(
        LIST_CLEAR,
        &args![member(this, PlayerCharacter::DroppedRefList)],
    );
    e.call(0x0096_2490, &args![this, 0u32, 1u32]);
    e.call(0x0096_2490, &args![this, 0u32, 0u32]);
    e.call(0x0070_5fc0, &args![0x011e_0aa8u32, 0u32]);
    e.call(
        0x00e9_8e20,
        &args![0x011e_0aa8u32, ZERO_VECTOR, ZERO_VECTOR, 0u32],
    );

    // The perk lists.
    free_embedded_list_items(e, member(this, PlayerCharacter::Perks));
    e.call(LIST_CLEAR, &args![member(this, PlayerCharacter::Perks)]);
    for i in 0..0x4au32 {
        e.call(
            LIST_CLEAR,
            &args![member(this, PlayerCharacter::PerkEntryLists) + i * 8],
        );
    }
    free_embedded_list_items(e, member(this, PlayerCharacter::CompanionPerks));
    e.call(
        LIST_CLEAR,
        &args![member(this, PlayerCharacter::CompanionPerks)],
    );
    for i in 0..0x4au32 {
        e.call(
            LIST_CLEAR,
            &args![member(this, PlayerCharacter::CompanionPerkEntryLists) + i * 8],
        );
    }

    e.call(LIST_CLEAR, &args![member(this, PlayerCharacter::listNotes)]);
    e.call(0x0096_55a0, &args![this]);
    e.call(0x0096_55e0, &args![this]);
    e.call(0x0095_2f90, &args![this]);
    let object = e.global::<u32>(0x011e_0774);
    e.call(OPERATOR_DELETE, &args![object]);
    let object = e.global::<u32>(0x011e_0760);
    e.call(OPERATOR_DELETE, &args![object]);

    // Members, in reverse order of construction.
    e.call(
        0x0096_a5f0,
        &args![member(this, PlayerCharacter::HotKeyLastAmmo)],
    );
    e.call(
        0x0096_a5a0,
        &args![member(this, PlayerCharacter::RockItLauncherAmmoList)],
    );
    e.call(
        NI_POINTER_DESTRUCT,
        &args![member(this, PlayerCharacter::spCameraRigidBody)],
    );
    e.call(
        NI_POINTER_DESTRUCT,
        &args![member(this, PlayerCharacter::spTargeted3D)],
    );
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![
            member(this, PlayerCharacter::CompanionPerkEntryLists),
            8u32,
            0x4au32,
            LIST_DESTRUCT
        ],
    );
    e.call(
        LIST_DESTRUCT,
        &args![member(this, PlayerCharacter::CompanionPerks)],
    );
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![
            member(this, PlayerCharacter::PerkEntryLists),
            8u32,
            0x4au32,
            LIST_DESTRUCT
        ],
    );
    e.call(LIST_DESTRUCT, &args![member(this, PlayerCharacter::Perks)]);
    e.call(
        0x008d_4380,
        &args![member(this, PlayerCharacter::CharacterProgressionInfo)],
    );
    e.call(
        NI_POINTER_DESTRUCT,
        &args![member(this, PlayerCharacter::sp3rdPersonLight)],
    );
    e.call(
        NI_POINTER_DESTRUCT,
        &args![member(this, PlayerCharacter::sp1stPersonLight)],
    );
    e.call(
        0x0096_a4b0,
        &args![member(this, PlayerCharacter::RandomDoorSpaceMap)],
    );
    e.call(
        LIST_DESTRUCT,
        &args![member(this, PlayerCharacter::DroppedRefList)],
    );
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![
            member(this, PlayerCharacter::MagicFailureSounds),
            0xcu32,
            6u32,
            SOUND_HANDLE_DESTRUCT
        ],
    );
    e.call(
        SOUND_HANDLE_DESTRUCT,
        &args![member(this, PlayerCharacter::SelectedSpellCastSound)],
    );
    e.call(
        LIST_DESTRUCT,
        &args![member(this, PlayerCharacter::AudioMarkerList)],
    );
    e.call(
        LIST_DESTRUCT,
        &args![member(this, PlayerCharacter::MapMarkerList)],
    );
    e.call(
        SOUND_HANDLE_DESTRUCT,
        &args![member(this, PlayerCharacter::StatusSoundHandle)],
    );
    e.call(
        LIST_DESTRUCT,
        &args![member(this, PlayerCharacter::CurrentRegionSoundList)],
    );
    e.call(
        0x004f_64b0,
        &args![member(this, PlayerCharacter::m_AllOccupiedRegions)],
    );
    e.call(
        0x006f_4930,
        &args![member(this, PlayerCharacter::PlayerMarkerPath)],
    );
    for field in [
        PlayerCharacter::listQuestTargets,
        PlayerCharacter::listObjectives,
        PlayerCharacter::listQuestLog,
        PlayerCharacter::listTopics,
    ] {
        e.call(LIST_DESTRUCT, &args![member(this, field)]);
    }
    for field in [
        PlayerCharacter::spInventoryPC,
        PlayerCharacter::sp1stPerson3D,
        PlayerCharacter::spGrabSpring,
    ] {
        e.call(NI_POINTER_DESTRUCT, &args![member(this, field)]);
    }
    e.call(
        LIST_DESTRUCT,
        &args![member(this, PlayerCharacter::pListOfTeammates)],
    );
    e.call(
        LIST_DESTRUCT,
        &args![member(this, PlayerCharacter::listNotes)],
    );

    e.call(0x008d_2060, &args![this]);
}

/// The address `offset` bytes into the player.
fn at(this: Ptr<PlayerCharacter>, offset: u32) -> u32 {
    this.addr().wrapping_add(offset)
}

/// Copies the zero vector into the three words at `011e0808`.
fn set_zero_vector_global(e: &mut Engine) {
    set_zero_vector(e, 0x011e_0808);
}

// Translated from 00944300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `fTimeSinceLastAmmoRegenTick` (Xbox PDB).
pub fn fn_00944300(e: &mut Engine, this: Ptr<PlayerCharacter>) -> f32 {
    e.get(this, PlayerCharacter::fTimeSinceLastAmmoRegenTick)
}

// Translated from 00944320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0x688` of the player (between `fBlockActivateTimer`
/// and `p1stPersonBipedAnim`; no field of the Xbox PDB layout is known there).
pub fn fn_00944320(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    e.mem.u32(at(this, 0x688))
}

// Translated from 00944340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `bBlockActivate` (Xbox PDB).
pub fn fn_00944340(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    e.get(this, PlayerCharacter::bBlockActivate)
}

// Translated from 00944360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `bCanWait` (Xbox PDB).
pub fn fn_00944360(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    e.get(this, PlayerCharacter::bCanWait)
}

// Translated from 00944380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fCounterAttackTimer` (Xbox PDB).
pub fn fn_00944380(e: &mut Engine, this: Ptr<PlayerCharacter>, value: f32) {
    e.set(this, PlayerCharacter::fCounterAttackTimer, x87_quiet(value));
}

// Translated from 009443a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `fCounterAttackTimer` (Xbox PDB).
pub fn fn_009443a0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> f32 {
    e.get(this, PlayerCharacter::fCounterAttackTimer)
}

// Translated from 009443c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether either of the two global flag bytes `011dea2b` and `011dea29` is
/// set. (The method's `this` is saved but never read.)
pub fn fn_009443c0(e: &mut Engine, _this: Ptr) -> bool {
    e.mem.u8(0x011d_ea2b) != 0 || e.mem.u8(0x011d_ea29) != 0
}

// Translated from 009443f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float global `011ac3a4`.
pub fn fn_009443f0(e: &mut Engine) -> f32 {
    e.global::<f32>(0x011a_c3a4)
}

// Translated from 00944400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the flag test `00621270` (the word at `+4` of its object ANDed
/// with the argument) is nonzero for the mask `0x400`, on the member at
/// `+0x410`.
pub fn fn_00944400(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0062_1270, &args![at_ptr(this, 0x410), 0x400u32])
        .u32()
        != 0
}

// Translated from 00944430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Same as [`fn_00944400`] for the mask `0x20000`.
pub fn fn_00944430(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0062_1270, &args![at_ptr(this, 0x410), 0x20000u32])
        .u32()
        != 0
}

// Translated from 00944460 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random number between two bounds from the global `BSRandom`: gets the
/// generator with `00476c00` (which takes no argument) and calls
/// [`fn_00944480`] on it with the two words, which the compiled code leaves
/// on the stack for it.
pub fn fn_00944460(e: &mut Engine, low: i32, high: i32) -> i32 {
    let random = e.call(0x0047_6c00, &args![]).u32();
    fn_00944480(e, Ptr::new(random), low, high)
}

// Translated from 00944480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `low` plus `BSRandom::UnsignedInt` (Xbox PDB, `00aa5230`) of
/// `high - low`, drawn from the generator `this`.
pub fn fn_00944480(e: &mut Engine, this: Ptr, low: i32, high: i32) -> i32 {
    let span = high.wrapping_sub(low);
    let drawn = e.call(0x00aa_5230, &args![this, span]).i32();
    drawn.wrapping_add(low)
}

// Translated from 009444a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `+0x204` of the object the global `011d96c0` points
/// to (0 when it is null).
pub fn fn_009444a0(e: &mut Engine) -> u8 {
    let object = e.global::<u32>(0x011d_96c0);
    if object == 0 {
        0
    } else {
        e.mem.u8(object + 0x204)
    }
}

// Translated from 009444d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdatePlayerCombat` (Xbox PDB): drops the combat group
/// when it is no longer live (`005a4320`, then `009869a0` with the player),
/// advances or clears `fCombatTimer`, and sets `bPlayerInCombat` and
/// `bAllCombatTargetsSearching` from the combat manager's combatant count
/// (`009931c0`).
pub fn player_character_update_player_combat(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    time_step: f32,
) {
    let group = e.get(this, PlayerCharacter::pCombatGroup);
    if group != Ptr::NULL {
        let alive = e.call(0x005a_4320, &args![group]).u32();
        if alive == 0 {
            e.call(0x0098_69a0, &args![group, this]);
            e.set(this, PlayerCharacter::pCombatGroup, Ptr::NULL);
        }
    }
    if e.get(this, PlayerCharacter::pCombatGroup) == Ptr::NULL {
        e.set(this, PlayerCharacter::fCombatTimer, 0.0);
    } else {
        let timer = e.get(this, PlayerCharacter::fCombatTimer);
        e.set(
            this,
            PlayerCharacter::fCombatTimer,
            (timer as f64 + time_step as f64) as f32,
        );
    }
    let manager = e.global::<u32>(COMBAT_MANAGER);
    let target = e.global::<u32>(0x011d_ea3c);
    let (count, searching) = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let count = e.call(0x0099_31c0, &args![manager, target, slot]).u32();
        (count, e.mem.u32(slot.addr()))
    });
    if count > 0 {
        e.set(this, PlayerCharacter::bPlayerInCombat, true);
        e.set(
            this,
            PlayerCharacter::bAllCombatTargetsSearching,
            count == searching,
        );
    } else {
        e.set(this, PlayerCharacter::bPlayerInCombat, false);
        e.set(this, PlayerCharacter::bAllCombatTargetsSearching, false);
    }
}

layout! {
    /// A 0x1A0-byte object the player's update code drives (the constructor
    /// allocates two; `009445b0` and `0095de30` use them): a table of 100
    /// floats, an end value, the two endpoints of a remapping, the time left
    /// and the duration. Its class name is not known; the field names
    /// describe how the code uses them.
    pub struct TimedCurve: 0x1A0 {
        /// The table of 100 eased fractions that `00946020` remaps from
        /// `[0, 1]` to `from..to` (400 bytes).
        0x000 samples: Inline<()>,
        /// The value `00946190` returns once the curve is over (what stores
        /// it is not confirmed).
        0x18C end_value: f32,
        /// First endpoint given to `00946020`.
        0x190 from: f32,
        /// Second endpoint given to `00946020`.
        0x194 to: f32,
        /// Time left (`00939880` clears it).
        0x198 remaining: f32,
        /// Total duration given to `00946020`.
        0x19C duration: f32,
    }
}

/// The address of sample `index` of a [`TimedCurve`].
fn curve_sample(this: Ptr<TimedCurve>, index: u32) -> u32 {
    this.addr().wrapping_add(index.wrapping_mul(4))
}

// Translated from 00946020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts a [`TimedCurve`]: fills the 100-entry table with an eased ramp
/// (two mirrored halves of 50: `ramp / 99` and `(99 - ramp) / 99`, the ramp
/// advancing by a step that itself grows each time, constants `0102d934` and
/// `01018144`), remaps every entry through `004b3ab0` from `[0, 1]` to
/// `from..to`, and stores the endpoints and the duration (the time left
/// starts at the duration). Always returns true: a test the compiler folded
/// makes the failure path unreachable.
pub fn timed_curve_start(
    e: &mut Engine,
    this: Ptr<TimedCurve>,
    from: f32,
    to: f32,
    duration: f32,
) -> bool {
    let mut ramp = 0.0f32;
    let mut step = e.global::<f32>(0x0102_d934);
    let growth = e.global::<f32>(0x0101_8144);
    let scale = e.global::<f64>(0x0107_5d30);
    for i in 0..0x32u32 {
        e.mem
            .set_f32(curve_sample(this, i), (ramp as f64 / scale) as f32);
        e.mem.set_f32(
            curve_sample(this, 99 - i),
            ((scale - ramp as f64) / scale) as f32,
        );
        ramp = (ramp as f64 + step as f64) as f32;
        step = (step as f64 + growth as f64) as f32;
    }
    for i in 0..100u32 {
        let sample = e.mem.f32(curve_sample(this, i));
        let mapped = e
            .call(0x004b_3ab0, &args![from, to, 0.0f32, 1.0f32, sample])
            .f32();
        e.mem.set_f32(curve_sample(this, i), mapped);
    }
    e.set(this, TimedCurve::from, from);
    e.set(this, TimedCurve::to, to);
    e.set(this, TimedCurve::duration, duration);
    e.set(this, TimedCurve::remaining, duration);
    true
}

// Translated from 00946140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances a [`TimedCurve`]: subtracts `time_step` from the time left and
/// returns whether it is now at or below zero.
pub fn timed_curve_advance(e: &mut Engine, this: Ptr<TimedCurve>, time_step: f32) -> bool {
    let remaining = e.get(this, TimedCurve::remaining);
    let remaining = (remaining as f64 - time_step as f64) as f32;
    e.set(this, TimedCurve::remaining, remaining);
    remaining <= 0.0
}

// Translated from 00946190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Current value of a [`TimedCurve`]. The table index is
/// `ftol(00406cc0((duration - max(remaining, 0)) / duration * 99))`; at index
/// 99 or beyond, or when the time is used up, the result is `end_value`,
/// otherwise the remap (`004b3ab0`) of the two neighbouring table entries
/// over `0..duration` at `duration - remaining`.
pub fn timed_curve_value(e: &mut Engine, this: Ptr<TimedCurve>) -> f32 {
    let remaining = e.get(this, TimedCurve::remaining);
    let left = e.call(0x0040_4010, &args![remaining, 0.0f32]).f32();
    let duration = e.get(this, TimedCurve::duration);
    let scale = e.global::<f64>(0x0107_5d30);
    let position = (((duration as f64 - left as f64) / duration as f64) * scale) as f32;
    let rounded = e.call(0x0040_6cc0, &args![position]).f64();
    let index = e.call(FTOL, &args![rounded]).i32();
    if index >= 99 {
        return e.get(this, TimedCurve::end_value);
    }
    let remaining = e.get(this, TimedCurve::remaining);
    if remaining <= 0.0 {
        return e.get(this, TimedCurve::end_value);
    }
    let duration = e.get(this, TimedCurve::duration);
    let elapsed = (duration as f64 - remaining as f64) as f32;
    let low = e.mem.f32(curve_sample(this, index as u32));
    let high = e
        .mem
        .f32(curve_sample(this, (index as u32).wrapping_add(1)));
    e.call(0x004b_3ab0, &args![low, high, 0.0f32, duration, elapsed])
        .f32()
}

// Translated from 00946280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the time left of a [`TimedCurve`] is at or below zero.
pub fn timed_curve_is_over(e: &mut Engine, this: Ptr<TimedCurve>) -> bool {
    e.get(this, TimedCurve::remaining) <= 0.0
}

/// The address `offset` bytes into an untyped object.
fn at_ptr(this: Ptr, offset: u32) -> u32 {
    this.addr().wrapping_add(offset)
}

// Translated from 009462c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::AmmoSwapHelper` (Xbox PDB): cycles the equipped weapon's
/// ammunition to the next type the player carries. `pCurrentProcess`
/// (`+0x68`) slot `0x148` gives the equipped item change, `0044ddc0` turns
/// it into the weapon; the weapon's ammo list (`+0xA4`) is walked from
/// `00500940(00474a40(..))`, wrapping to the start, past the current ammo
/// (`00525980`) until an entry the player has a stack of is found (the
/// player's `00576260` and the stack count `00726070` > 0). With
/// `keep_current` set and the current ammo still in stock nothing changes.
/// The found ammo replaces the equipped item (slot `0x168`; its hotkey,
/// `004bd7a0`, is carried over with `004bfb70`), and the player's slot
/// `0x3EC` is called with the weapon, the draw state (2 when `draw` is set and
/// the weapon is drawn, `008a16d0`) and whether `004bda70` (the mod effect
/// check) holds.
///
/// The compiler's exception-unwinding state writes are not translated.
pub fn player_character_ammo_swap_helper(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    draw: bool,
    keep_current: bool,
) {
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    if process == 0 {
        return;
    }
    if e.vcall(process, 0x148, &args![]).u32() == 0 {
        return;
    }
    let equipped = e.vcall(process, 0x148, &args![]).u32();
    let weapon = if equipped != 0 {
        e.call(0x0044_ddc0, &args![equipped]).u32()
    } else {
        0
    };
    if weapon == 0 {
        return;
    }
    let ammo_list = weapon.wrapping_add(0xa4);
    if e.call(0x0047_4a40, &args![ammo_list]).u32() == 0 {
        return;
    }
    let first = e.call(0x0047_4a40, &args![ammo_list]).u32();
    let mut node = e.call(0x0050_0940, &args![first]).u32();
    let player_actor = e.global::<u32>(0x011d_ea3c);
    let mut current = e.call(0x0052_5980, &args![weapon, player_actor]).u32();
    let mut candidate = 0u32;
    let mut found = false;
    let mut seen_current = false;
    if current != 0 && node != 0 {
        let in_list = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), current);
            e.call(0x005f_65d0, &args![node, slot]).bool()
        });
        if !in_list {
            let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
            if process != 0 {
                e.vcall(process, 0x168, &args![0u32]);
            }
            current = 0;
        }
    }
    let mut item = e.call(0x0057_6260, &args![this, current, 0u32]).u32();
    if keep_current && current != 0 && item != 0 && e.call(0x0072_6070, &args![item]).i32() > 0 {
        e.call(0x0044_59e0, &args![item, 1u32]);
        return;
    }
    if item != 0 {
        e.call(0x0044_59e0, &args![item, 1u32]);
    }
    item = 0;
    loop {
        if node == 0 {
            break;
        }
        let slot = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
        candidate = e.mem.u32(slot);
        node = e.call(LIST_NEXT, &args![node]).u32();
        if current == candidate {
            if seen_current {
                break;
            }
            seen_current = true;
        } else if seen_current || current == 0 {
            item = e.call(0x0057_6260, &args![this, candidate, 0u32]).u32();
            if item != 0 && e.call(0x0072_6070, &args![item]).i32() > 0 {
                found = true;
                break;
            }
        }
        let at_end = if node != 0 {
            let slot = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
            e.mem.u32(slot) == 0
        } else {
            true
        };
        if at_end {
            let first = e.call(0x0047_4a40, &args![ammo_list]).u32();
            node = e.call(0x0050_0940, &args![first]).u32();
        }
    }
    if found {
        let hotkey = e.call(0x004b_d7a0, &args![equipped]).i32();
        if hotkey >= 0 {
            let hotkey = e.call(0x004b_d7a0, &args![equipped]).u32();
            e.call(0x004b_fb70, &args![this, hotkey, candidate]);
        }
        let changes = e.call(0x004b_f220, &args![this]).u32();
        let count = e.call(0x004c_8f30, &args![changes, candidate]).u32();
        let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        let new_item = if block != 0 {
            e.call(0x004b_c550, &args![block, candidate, count]).u32()
        } else {
            0
        };
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        e.vcall(process, 0x168, &args![new_item]);
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        let state = e.vcall(process, 0x14c, &args![]).u32();
        e.call(0x006e_cd40, &args![state, 0u32]);
        let draw_state = if draw && e.call(0x008a_16d0, &args![this]).bool() {
            2u32
        } else {
            0
        };
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        let equipped_now = e.vcall(process, 0x148, &args![]).u32();
        let mod_active = e.call(0x004b_da70, &args![equipped_now, 2u32]).u8();
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        let equipped_now = e.vcall(process, 0x148, &args![]).u32();
        let object = e.call(0x0044_ddc0, &args![equipped_now]).u32();
        e.vcall(
            this.addr(),
            0x3ec,
            &args![object, draw_state, mod_active as u32, !draw as u32],
        );
    }
    if item != 0 {
        e.call(0x0044_59e0, &args![item, 1u32]);
    }
}

// Translated from 00947b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::CheckBorderRegion` (Xbox PDB): when the game setting
/// byte at `011e08e0` is on and `00451530` on the global `011dea10` is
/// false, decides whether the player stands inside the playable area:
/// `bInBorderContainedCell` counts as inside; with border regions (`+0x7B0`)
/// the player's position (`00436aa0`, turned into a 2D point by `004f7030`)
/// is tested against every region's entries with `004f8360`; without them the
/// player is inside unless the world space (`00575d70`) has a border region
/// (`00586260`). Inside records the position (`SetLastKnownGoodPosition`),
/// outside returns the player to the last good one (`0094dbe0`).
pub fn player_character_check_border_region(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let setting = e.call(SETTING_BYTE_GETTER, &args![0x011e_08e0u32]).u32();
    if e.mem.u8(setting) == 0 {
        return;
    }
    let object = e.global::<u32>(0x011d_ea10);
    if e.call(0x0045_1530, &args![object]).u8() != 0 {
        return;
    }
    let mut inside = false;
    if e.get(this, PlayerCharacter::bInBorderContainedCell) {
        inside = true;
    } else if e.get(this, PlayerCharacter::pBorderRegions) != Ptr::NULL {
        let position = e.call(0x0043_6aa0, &args![this]).u32();
        e.with_stack(8, |e, point| {
            e.call(0x004f_7030, &args![point, position]);
            let regions = e.get(this, PlayerCharacter::pBorderRegions);
            let count = e.call(0x0065_8930, &args![regions]).u32();
            let mut index = 0u32;
            while index < count {
                let regions = e.get(this, PlayerCharacter::pBorderRegions);
                let entry = e.call(0x0087_7a30, &args![regions, index]).u32();
                let region = e.mem.u32(entry);
                if e.call(0x0044_0d80, &args![region]).u8() == 0 {
                    let mut list = e.call(0x0044_1110, &args![region]).u32();
                    while !inside && list != 0 && e.call(0x0082_56d0, &args![list]).u8() == 0 {
                        let slot = e.call(LIST_ITEM_ADDRESS, &args![list]).u32();
                        let border = e.mem.u32(slot);
                        if e.call(0x004f_8360, &args![border, point]).u8() != 0 {
                            inside = true;
                        }
                        list = e.call(LIST_NEXT, &args![list]).u32();
                    }
                }
                if inside {
                    break;
                }
                index += 1;
            }
        });
    } else {
        let world_space = e.call(0x0057_5d70, &args![this]).u32();
        if world_space == 0 || e.call(0x0058_6260, &args![world_space]).u8() == 0 {
            inside = true;
        }
    }
    if inside {
        player_character_set_last_known_good_position(e, this);
    } else {
        e.call(0x0094_dbe0, &args![this, 0u32]);
    }
}

// Translated from 00947c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetLastKnownGoodPosition` (Xbox PDB): copies the
/// player's position (`00436aa0`) into `LastKnownGoodPosition` and stores in
/// `pLastKnownGoodLocation` the world space of the parent cell
/// (`008d6f30`, then `0054ddd0`) or, when there is none, the parent cell
/// itself.
pub fn player_character_set_last_known_good_position(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let position = e.call(0x0043_6aa0, &args![this]).u32();
    let target = member(this, PlayerCharacter::LastKnownGoodPosition);
    for i in 0..3 {
        let word = e.mem.u32(position + 4 * i);
        e.mem.set_u32(target + 4 * i, word);
    }
    let location = if e.call(0x008d_6f30, &args![this]).u32() != 0 {
        let cell = e.call(0x008d_6f30, &args![this]).u32();
        if e.call(0x0054_ddd0, &args![cell]).u32() != 0 {
            let cell = e.call(0x008d_6f30, &args![this]).u32();
            e.call(0x0054_ddd0, &args![cell]).u32()
        } else {
            e.call(0x008d_6f30, &args![this]).u32()
        }
    } else {
        e.call(0x008d_6f30, &args![this]).u32()
    };
    e.set(
        this,
        PlayerCharacter::pLastKnownGoodLocation,
        Ptr::new(location),
    );
}

/// Deletes every item of the `BSSimpleList` at `list` with `operator delete`,
/// then clears the list (`00470470`): the shared body of the two "clear a
/// list of heap structs" functions.
fn delete_list_items(e: &mut Engine, list: u32) {
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(slot);
        e.call(OPERATOR_DELETE, &args![item]);
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    e.call(LIST_CLEAR, &args![list]);
}

/// Clears and, when it is not null, deletes the temporary list `list` the
/// way the compiler does for a local `BSSimpleList *`.
fn release_temporary_list(e: &mut Engine, list: u32) {
    e.call(LIST_CLEAR, &args![list]);
    if list != 0 {
        e.call(LIST_DELETING_DESTRUCT, &args![list, 1u32]);
    }
}

// Translated from 00947d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every `AudioMarkerList` entry (`operator delete`), clears the list
/// and resets `pClosestAudioMarkerInfo`.
pub fn fn_00947d10(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    delete_list_items(e, member(this, PlayerCharacter::AudioMarkerList));
    e.set(this, PlayerCharacter::pClosestAudioMarkerInfo, Ptr::NULL);
}

// Translated from 00947d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds `AudioMarkerList`: clears it, takes the marker references of the
/// player's world space (`005883c0(worldSpace, 1)`) or, outside any, of the
/// parent cell (`008d6f30` fills a fresh list with `0054b8c0`), wraps every
/// non-null reference in an 8-byte entry (`00948000`) and appends the entry
/// with `005ae3d0` when its second word is set, else frees it.
///
/// The compiler's exception-unwinding state writes are not translated.
pub fn fn_00947d80(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let world_space = e.call(0x0057_5d70, &args![this]).u32();
    fn_00947d10(e, this);
    let list = if world_space != 0 {
        e.call(0x0058_83c0, &args![world_space, 1u32]).u32()
    } else {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(LIST_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        let cell = e.call(0x008d_6f30, &args![this]).u32();
        if cell != 0 {
            e.call(0x0054_b8c0, &args![cell, list]);
        }
        list
    };
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
        let reference = e.mem.u32(slot);
        if reference != 0 {
            let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
            let entry = if block != 0 {
                e.call(0x0094_8000, &args![block, reference]).u32()
            } else {
                0
            };
            if e.mem.u32(entry.wrapping_add(4)) != 0 {
                e.with_stack(4, |e, holder| {
                    e.mem.set_u32(holder.addr(), entry);
                    e.call(
                        0x005a_e3d0,
                        &args![member(this, PlayerCharacter::AudioMarkerList), holder],
                    );
                });
            } else {
                e.call(OPERATOR_DELETE, &args![entry]);
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    release_temporary_list(e, list);
}

// Translated from 00948000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 8-byte audio marker entry: stores `reference` and, when
/// it is not null, the value `00569080` returns for it. Returns the entry.
pub fn fn_00948000(e: &mut Engine, this: Ptr, reference: u32) -> Ptr {
    e.mem.set_u32(this.addr(), reference);
    if reference != 0 {
        let value = e.call(0x0056_9080, &args![reference]).u32();
        e.mem.set_u32(this.addr() + 4, value);
    }
    this
}

// Translated from 00948030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the address of the player's `MapMarkerList` (`+0x7C8`).
pub fn fn_00948030(_e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    member(this, PlayerCharacter::MapMarkerList)
}

// Translated from 00948050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ClearMapMarkerStructList` (Xbox PDB): frees every
/// `MapMarkerList` entry, clears the list and resets `pMapWorld`.
pub fn player_character_clear_map_marker_struct_list(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    delete_list_items(e, member(this, PlayerCharacter::MapMarkerList));
    e.set(this, PlayerCharacter::pMapWorld, Ptr::NULL);
}

// Translated from 009480c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ResetMapMarkerStructList` (Xbox PDB): when the player's
/// world space differs from `pMapWorld`, clears the list and (for a world
/// space) refills it from the world space's map marker references
/// (`005882a0(worldSpace, 1)`): each non-null reference gets an 8-byte entry
/// {`00569060(reference)`, reference}, appended (`005ae3d0`) when its first
/// word is set, else freed. `pMapWorld` becomes the world space.
pub fn player_character_reset_map_marker_struct_list(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let world_space = e.call(0x0057_5d70, &args![this]).u32();
    if e.get(this, PlayerCharacter::pMapWorld).addr() == world_space {
        return;
    }
    player_character_clear_map_marker_struct_list(e, this);
    if world_space != 0 {
        let list = e.call(0x0058_82a0, &args![world_space, 1u32]).u32();
        let mut node = list;
        while node != 0 {
            let slot = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
            let reference = e.mem.u32(slot);
            if reference != 0 {
                let entry = e.call(OPERATOR_NEW, &args![8u32]).u32();
                let data = e.call(0x0056_9060, &args![reference]).u32();
                e.mem.set_u32(entry, data);
                e.mem.set_u32(entry + 4, reference);
                if e.mem.u32(entry) != 0 {
                    e.with_stack(4, |e, holder| {
                        e.mem.set_u32(holder.addr(), entry);
                        e.call(
                            0x005a_e3d0,
                            &args![member(this, PlayerCharacter::MapMarkerList), holder],
                        );
                    });
                } else {
                    e.call(OPERATOR_DELETE, &args![entry]);
                }
            }
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
        release_temporary_list(e, list);
    }
    e.set(this, PlayerCharacter::pMapWorld, Ptr::new(world_space));
}

// Translated from 009481d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-synchronises the two player animations (`00950a60` for index 0 and 1)
/// while the process says (slots `0x1A8` and `0x454` of `pCurrentProcess`)
/// and the anim action (`008a7570`) is 5: when `0070f490(first, 4)` is 1,
/// both animations skip an update (`008eeaa0(4)`), then each animation with
/// a `00491040(4)` object gets that object's time
/// (`005f3780(0048f7f0(object, 1))` minus `00639aa0(object)`) passed to
/// `00966a00`.
pub fn fn_009481d0(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let first = e.call(0x0095_0a60, &args![this, 0u32]).u32();
    let second = e.call(0x0095_0a60, &args![this, 1u32]).u32();
    if first == 0 || second == 0 {
        return;
    }
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    if e.vcall(process, 0x1a8, &args![]).u8() == 0 {
        return;
    }
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    if e.vcall(process, 0x454, &args![]).u8() == 0 {
        return;
    }
    if e.call(0x008a_7570, &args![this]).u32() != 5 {
        return;
    }
    if e.call(0x0070_f490, &args![first, 4u32]).u32() != 1 {
        return;
    }
    e.call(0x008e_eaa0, &args![first, 4u32]);
    e.call(0x008e_eaa0, &args![second, 4u32]);
    for animation in [first, second] {
        let object = e.call(0x0049_1040, &args![animation, 4u32]).u32();
        if object != 0 {
            // 00948296: the pushed 1 is the word of `005f3780` (RET 4).
            let holder = e.call(0x0048_f7f0, &args![object]).u32();
            let time = e.call(0x005f_3780, &args![holder, 1u32]).f64();
            let base = e.call(0x0063_9aa0, &args![object]).f64();
            let delta = (time - base) as f32;
            e.call(0x0096_6a00, &args![animation, delta]);
        }
    }
}

// Translated from 0094a070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `(float at +0x30 - float at +0x2C) / float at +0x28` of its object.
pub fn fn_0094a070(e: &mut Engine, this: Ptr) -> f32 {
    let high = e.mem.f32(at_ptr(this, 0x30)) as f64;
    let low = e.mem.f32(at_ptr(this, 0x2c)) as f64;
    let span = e.mem.f32(at_ptr(this, 0x28)) as f64;
    ((high - low) / span) as f32
}

// Translated from 0094a0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `+0x198` of its object.
pub fn fn_0094a0a0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(at_ptr(this, 0x198))
}

// Translated from 0094c380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00621370(this, 0)`.
pub fn fn_0094c380(e: &mut Engine, this: Ptr) {
    e.call(0x0062_1370, &args![this, 0u32]);
}

// Translated from 0094c3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object the global `011d96c0` points to exists, stores
/// `scale * (float game setting at 011d9748)` into its float at `+0x274`.
pub fn fn_0094c3a0(e: &mut Engine, scale: f32) {
    let object = e.global::<u32>(0x011d_96c0);
    if object != 0 {
        let setting = e.call(SETTING_FLOAT_GETTER, &args![0x011d_9748u32]).u32();
        let value = e.mem.f32(setting);
        e.mem
            .set_f32(object + 0x274, (scale as f64 * value as f64) as f32);
    }
}

// Translated from 0094c3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads an actor value modifier of the player: `kind` 0 selects
/// `TemporaryActorValueModifiers`, 1 `ScriptActorValueModifiers`, 2 the
/// damage modifiers (`fHealthModifier` for actor value 0x10,
/// `DamageActorValueModifiers` otherwise); any other `kind` gives 0.
pub fn fn_0094c3d0(e: &mut Engine, this: Ptr<PlayerCharacter>, kind: u32, actor_value: u32) -> f32 {
    match kind {
        0 => e.mem.f32(at(this, 0x244 + actor_value.wrapping_mul(4))),
        1 => e.mem.f32(at(this, 0x378 + actor_value.wrapping_mul(4))),
        2 => {
            if actor_value == 0x10 {
                e.get(this, PlayerCharacter::fHealthModifier)
            } else {
                e.mem.f32(at(this, 0x4b0 + actor_value.wrapping_mul(4)))
            }
        }
        _ => 0.0,
    }
}

// Translated from 0094c460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValueOwner` base override: the temporary modifier (kind 0) of the
/// player for `actor_value`; `this` is the `ActorValueOwner` subobject
/// (`PlayerCharacter + 0xA4`). The float result stays in `ST0` from
/// [`fn_0094c3d0`].
pub fn fn_0094c460(e: &mut Engine, this: Ptr, actor_value: u32) -> f32 {
    let player = Ptr::new(this.addr().wrapping_sub(ACTOR_VALUE_OWNER_BASE));
    fn_0094c3d0(e, player, 0, actor_value)
}

/// Global pointer to the actor the look code of modes 1 to 3 turns the player
/// towards, and (`011a59f0`) the body part index it is aimed at.
const LOOK_TARGET_ACTOR: u32 = 0x011f_21cc;
const LOOK_TARGET_PART: u32 = 0x011a_59f0;
/// The actor the look state (below) was last built for.
const LAST_LOOK_TARGET: u32 = 0x011e_0be0;
/// Bit 0: `LOOK_CENTER` initialised; bit 1: `LOOK_LAST_HEIGHT` initialised.
const LOOK_FLAGS: u32 = 0x011e_0bdc;
/// The three floats of the aim point of the target (centre of its body parts).
const LOOK_CENTER: u32 = 0x011e_0bd0;
/// The last height (z) the camera was aimed at.
const LOOK_LAST_HEIGHT: u32 = 0x011e_0bcc;
/// Float added to the height of the aim point.
const LOOK_HEIGHT_OFFSET: u32 = 0x011e_075c;
/// Globals holding pointers to the two [`TimedCurve`] objects (heading, then
/// pitch) that smooth the look.
const HEADING_CURVE: u32 = 0x011e_0774;
const PITCH_CURVE: u32 = 0x011e_0760;
/// Byte global: set while the smoothed look is used.
const SMOOTH_LOOK_ENABLED: u32 = 0x011a_3b31;
/// Byte globals tested by the look input code (`011e07b8`: the player is in a
/// mode with a free camera, `011e07c1`: the analog input is a second stick).
const FREE_CAMERA_FLAG: u32 = 0x011e_07b8;
const SECOND_STICK_FLAG: u32 = 0x011e_07c1;
/// Floats the look input code keeps: the zoom distance (`011e0b5c`) and the
/// two accumulated angles (`011e0b60`, `011e0b58`); and the heading and pitch
/// of the player when the input was last handled (`011e076c`, `011e0764`).
const ZOOM_DISTANCE: u32 = 0x011e_0b5c;
const ACCUMULATED_YAW: u32 = 0x011e_0b60;
const ACCUMULATED_PITCH: u32 = 0x011e_0b58;
const LAST_HEADING: u32 = 0x011e_076c;
const LAST_PITCH: u32 = 0x011e_0764;
/// Float global set to a turn speed estimate while the player turns.
const TURN_SPEED: u32 = 0x011a_3b40;
/// `NiCamera::WorldPtToScreenPt`'s tolerance (a float in the exe).
const SCREEN_TOLERANCE: u32 = 0x0107_18c0;

/// The three words copied from the `NiPoint3` the node getter `0045bb80`
/// returns for `node`.
fn node_position(e: &mut Engine, node: u32) -> [u32; 3] {
    let position = e.call(0x0045_bb80, &args![node]).u32();
    read_words(e, position)
}

/// `NiAVObject::GetObjectByName(root, name)` of the part `part` (a body part
/// data entry): the node named by `0043b230(part)` under the 3D root of
/// `actor` (slot `0x1D0`).
fn body_part_node(e: &mut Engine, actor: u32, part: u32) -> u32 {
    let name = e.call(0x0043_b230, &args![part]).u32();
    let root = e.vcall(actor, 0x1d0, &args![]).u32();
    e.call(0x004a_ae30, &args![root, name]).u32()
}

/// Projects `position` (three words in the frame) to the screen with
/// `NiCamera::WorldPtToScreenPt` (`00a6fc50`) on the camera chain
/// `006629f0(0045c670())`; the two outputs are the screen x and y.
fn project_to_screen(e: &mut Engine, position: u32, screen: u32) {
    let camera = e.call(0x0045_c670, &args![]).u32();
    let space = e.call(0x0066_29f0, &args![camera]).u32();
    let tolerance = e.global::<f32>(SCREEN_TOLERANCE);
    e.call(
        0x00a6_fc50,
        &args![space, position, screen, screen + 4, tolerance],
    );
}

/// `GetAngleToProjectedPoint` (`009a8af0`) from the camera position to
/// `target`: returns the angles (x: pitch, z: heading) in the order the
/// callee writes them.
fn angles_from_camera(e: &mut Engine, target: [u32; 3]) -> [f32; 3] {
    let camera = e.call(0x0045_c670, &args![]).u32();
    let space = e.call(0x0066_29f0, &args![camera]).u32();
    let position = e.call(0x0045_bb80, &args![space]).u32();
    let origin = read_words(e, position);
    e.with_stack(12, |e, out| {
        e.call(
            0x009a_8af0,
            &args![out, origin[0], origin[1], origin[2], target[0], target[1], target[2]],
        );
        read_floats(e, out.addr())
    })
}

/// Brings `angle` into `[-pi, pi]` by adding or subtracting `2 pi` (the
/// double constants at `0101ff40`, `0101ff48` and `0101ff58`), rounding to a
/// float at each step.
fn wrap_angle(e: &mut Engine, angle: f32) -> f32 {
    let pi = e.global::<f64>(0x0101_ff40);
    let two_pi = e.global::<f64>(0x0101_ff48);
    let minus_pi = e.global::<f64>(0x0101_ff58);
    let mut angle = angle;
    while angle as f64 > pi {
        angle = (angle as f64 - two_pi) as f32;
    }
    while (angle as f64) < minus_pi {
        angle = (angle as f64 + two_pi) as f32;
    }
    angle
}

/// Truncates like `_ftol2_sse` (`00ec62c0`).
fn truncate(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// A game setting that holds an integer (`0043d4d0`).
fn setting_int(e: &mut Engine, setting: u32) -> i32 {
    let value = e.call(SETTING_INT_GETTER, &args![setting]).u32();
    e.mem.i32(value)
}

// Translated from 009445b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player's look handling, called with the frame time, whether the aim
/// button is down and a pointer to the 16-bit input flag word to update. The
/// mode of the `VATS` object (`0044ddc0`) selects what happens, and the
/// result is whether look input was used:
///
/// - mode 0, the free look: reads the analog look input, turns the player
///   (`0x931d30`/`0x931e50`) and handles the zoom ([`apply_look_input`]);
/// - modes 1 to 3, the VATS camera: [`follow_look_target`];
/// - mode 4, the VATS camera on an action's target:
///   [`look_at_vats_action_target`];
/// - any other mode does nothing and returns 0.
pub fn fn_009445b0(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    time_step: f32,
    aim_pressed: bool,
    flags: Ptr,
    _unused_4: u32,
) -> u8 {
    match e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() {
        0 => apply_look_input(e, this, time_step, aim_pressed, flags),
        1..=3 => {
            follow_look_target(e, this, time_step);
            0
        }
        4 => look_at_vats_action_target(e, this, flags),
        _ => 0,
    }
}

/// Mode 4 of [`fn_009445b0`]: the current VATS action (`009c71c0`) names a
/// target actor (`+0xC`) and a body part (`+0x10`, -1 for none). The aim
/// point is the part's node when the actor is an actor (slot `0x100`) with
/// body part data, else the centre of its 3D world bound, else its position
/// (slot `0x1F4`); an actor with process slot `0x1A8` set is aimed higher by
/// `572380` times a setting. `009a88e0` turns that point into angles that are
/// stored with slot `0x2C4` (heading) and `SetLooking` (pitch). Finally the
/// VATS object is updated (`009c7240`) and the input flags get the player's
/// `008846e0` bits masked with `0xCC00`. The result is always 1.
///
/// The temporary `NiPoint3` constructor calls (`006815c0`, which return
/// their argument) are omitted.
fn look_at_vats_action_target(e: &mut Engine, this: Ptr<PlayerCharacter>, flags: Ptr) -> u8 {
    let mut target = 0u32;
    let mut body_part = 0xffff_ffffu32;
    if e.call(0x009c_71c0, &args![VATS_OBJECT]).u32() != 0 {
        let action = e.call(0x009c_71c0, &args![VATS_OBJECT]).u32();
        target = e.mem.u32(action + 0xc);
        let action = e.call(0x009c_71c0, &args![VATS_OBJECT]).u32();
        body_part = e.mem.u32(action + 0x10);
    }
    if target != 0 {
        let root = e.vcall(target, 0x1d0, &args![]).u32();
        let mut aim = [0u32; 3];
        let mut found = false;
        if body_part != 0xffff_ffff && e.vcall(target, 0x100, &args![]).u8() != 0 {
            let owner = e.call(0x0041_81e0, &args![target]).u32();
            let data = e.vcall(owner, 0x180, &args![]).u32();
            if data != 0 {
                let part = e.call(0x005e_5130, &args![data, body_part]).u32();
                if part != 0 {
                    let node = body_part_node(e, target, part);
                    if node != 0 {
                        aim = node_position(e, node);
                        found = true;
                    }
                }
            }
        }
        if !found {
            if root != 0 {
                let bound = e.call(0x0043_d450, &args![root]).u32();
                let centre = e.call(LIST_ITEM_ADDRESS, &args![bound]).u32();
                aim = read_words(e, centre);
            } else {
                let position = e.vcall(target, 0x1f4, &args![]).u32();
                aim = read_words(e, position);
            }
        }
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        if e.vcall(process, 0x1a8, &args![]).u8() != 0 {
            let height = e.with_stack(12, |e, point| {
                for (i, word) in aim.iter().enumerate() {
                    e.mem.set_u32(point.addr() + 4 * i as u32, *word);
                }
                e.call(0x0057_2380, &args![this, point]).f64()
            });
            let scale = setting_float(e, 0x011c_e768);
            let raised = scale as f64 * height + f32::from_bits(aim[2]) as f64;
            aim[2] = (raised as f32).to_bits();
        }
        let angles = e.with_stack(12, |e, out| {
            e.call(0x009a_88e0, &args![out, this, aim[0], aim[1], aim[2]]);
            read_floats(e, out.addr())
        });
        e.vcall(this.addr(), 0x2c4, &args![angles[2]]);
        e.call(0x0093_1d90, &args![this, angles[0]]);
    }
    e.call(0x009c_7240, &args![VATS_OBJECT]);
    let bits = e.call(0x0088_46e0, &args![this]).u32();
    e.mem.set_u16(flags.addr(), (bits & 0xcc00) as u16);
    1
}

/// Modes 1 to 3 of [`fn_009445b0`]: smoothly turns the player to look at the
/// VATS target actor (`011f21cc`) with two [`TimedCurve`] objects (heading
/// `011e0774`, pitch `011e0760`).
///
/// First `0095de30(time_step)` runs. Without a target nothing else happens.
/// The aim point is the target's world position (3D root slot `0x1D0` through
/// `0045bb80`, or slot `0x1F4` without a 3D root). For an actor (slot
/// `0x100`) it is replaced by the average position of the target's body part
/// nodes ([`LOOK_CENTER`], rebuilt when the target changed: just the torso,
/// part 1, when it is closer than the global `011a3b60`, otherwise all 15
/// parts), and moved up by [`LOOK_HEIGHT_OFFSET`], which is set from the
/// screen positions of the parts in mode 2 with the VATS menu open, or from
/// the aimed body part or the acquire object. The angles from the camera to
/// the final point (`GetAngleToProjectedPoint_ov2`) are wrapped to within pi
/// of the player's heading and pitch; when the point changed the two curves
/// are restarted from the current heading and pitch with a duration from a
/// setting (`0x11d48dc` for mode 1; mode 2 `0x11d4588`, or `0x11d4a2c` once
/// the target was already looked at; the float `01016264` otherwise); the
/// curves are advanced by `0084d030(0x11f6394)`, and the player's heading
/// (slot `0x2C4`) and pitch (`SetLooking`) are set from them. Finally
/// `007f3bd0` is told whether both curves are over with the smooth look
/// enabled.
///
/// The temporary `NiPoint3` constructor calls (`006815c0`, which return
/// their argument) are omitted.
fn follow_look_target(e: &mut Engine, this: Ptr<PlayerCharacter>, time_step: f32) {
    let actor = e.global::<u32>(LOOK_TARGET_ACTOR);
    let part_index = e.global::<u32>(LOOK_TARGET_PART);
    e.call(0x0095_de30, &args![this, time_step]);
    if actor == 0 {
        return;
    }
    let heading_curve = Ptr::<TimedCurve>::new(e.global::<u32>(HEADING_CURVE));
    let pitch_curve = Ptr::<TimedCurve>::new(e.global::<u32>(PITCH_CURVE));
    let root = e.vcall(actor, 0x1d0, &args![]).u32();
    let mut changed = false;
    let mut saved_offset = 0.0f32;
    let mut same_target = true;
    if e.global::<u32>(LOOK_TARGET_ACTOR) != e.global::<u32>(LAST_LOOK_TARGET) {
        let current = e.global::<u32>(LOOK_TARGET_ACTOR);
        e.set_global(LAST_LOOK_TARGET, current);
        changed = true;
        same_target = false;
        if e.vcall(actor, 0x100, &args![]).u8() != 0 {
            e.set_global(LOOK_HEIGHT_OFFSET, 0.0f32);
            saved_offset = e.global::<f32>(LOOK_HEIGHT_OFFSET);
        } else {
            e.set_global(LOOK_HEIGHT_OFFSET, 0.0f32);
        }
    }
    let mut aim;
    if root != 0 {
        let node = e.vcall(actor, 0x1d0, &args![]).u32();
        aim = node_position(e, node);
        if e.vcall(actor, 0x100, &args![]).u8() != 0 {
            aim = aim_at_actor(e, this, actor, part_index, changed, saved_offset);
            let height = f32::from_bits(aim[2]);
            let offset = e.global::<f32>(LOOK_HEIGHT_OFFSET);
            aim[2] = ((height as f64 + offset as f64) as f32).to_bits();
        }
    } else {
        let position = e.vcall(actor, 0x1f4, &args![]).u32();
        aim = read_words(e, position);
    }
    let height = f32::from_bits(aim[2]);
    if e.global::<u32>(LOOK_FLAGS) & 2 == 0 {
        let flags = e.global::<u32>(LOOK_FLAGS);
        e.set_global(LOOK_FLAGS, flags | 2);
        e.set_global(LOOK_LAST_HEIGHT, height);
    }
    if e.global::<f32>(LOOK_LAST_HEIGHT) != height
        && timed_curve_is_over(e, heading_curve)
        && timed_curve_is_over(e, pitch_curve)
    {
        e.set_global(LOOK_LAST_HEIGHT, height);
        changed = true;
    }
    let mut angles = angles_from_camera(e, aim);
    let heading = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
    let mut heading_error = (angles[2] as f64 - heading as f64) as f32;
    let pitch = e.call(0x0093_1d70, &args![this]).f32();
    let mut pitch_error = (angles[0] as f64 - pitch as f64) as f32;
    heading_error = wrap_angle(e, heading_error);
    pitch_error = wrap_angle(e, pitch_error);
    let heading = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
    angles[2] = (heading as f64 + heading_error as f64) as f32;
    let pitch = e.call(0x0093_1d70, &args![this]).f32();
    angles[0] = (pitch as f64 + pitch_error as f64) as f32;
    if changed {
        let mode = e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32();
        let duration = if mode == 1 {
            setting_float(e, 0x011d_48dc)
        } else if mode == 2 {
            if same_target {
                setting_float(e, 0x011d_4588)
            } else {
                setting_float(e, 0x011d_4a2c)
            }
        } else {
            e.global::<f32>(0x0101_6264)
        };
        let heading = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
        timed_curve_start(e, heading_curve, heading, angles[2], duration);
        let pitch = e.call(0x0093_1d70, &args![this]).f32();
        timed_curve_start(e, pitch_curve, pitch, angles[0], duration);
    }
    let step = e.call(0x0084_d030, &args![0x011f_6394u32]).f32();
    timed_curve_advance(e, heading_curve, step);
    let step = e.call(0x0084_d030, &args![0x011f_6394u32]).f32();
    timed_curve_advance(e, pitch_curve, step);
    let new_heading = timed_curve_value(e, heading_curve);
    let new_pitch = timed_curve_value(e, pitch_curve);
    e.vcall(this.addr(), 0x2c4, &args![new_heading]);
    e.call(0x0093_1d90, &args![this, new_pitch]);
    let running = timed_curve_is_over(e, heading_curve)
        && timed_curve_is_over(e, pitch_curve)
        && e.mem.u8(SMOOTH_LOOK_ENABLED) != 0;
    e.call(0x007f_3bd0, &args![running as u32]);
}

/// The part of [`follow_look_target`] for an actor target (slot `0x100`):
/// rebuilds the aim point [`LOOK_CENTER`] when `changed`, then returns the
/// aim point (the centre, moved by the screen-space adjustments of
/// [`LOOK_HEIGHT_OFFSET`]).
fn aim_at_actor(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor: u32,
    part_index: u32,
    changed: bool,
    saved_offset: f32,
) -> [u32; 3] {
    if e.global::<u32>(LOOK_FLAGS) & 1 == 0 {
        let flags = e.global::<u32>(LOOK_FLAGS);
        e.set_global(LOOK_FLAGS, flags | 1);
        for i in 0..3 {
            let word = e.mem.u32(ZERO_VECTOR + 4 * i);
            e.mem.set_u32(LOOK_CENTER + 4 * i, word);
        }
    }
    if changed {
        rebuild_look_center(e, actor);
    }
    let aim = read_words(e, LOOK_CENTER);
    let mut covered = false;
    let input = e.call(0x004b_7210, &args![]).u32();
    let curves_idle = |e: &mut Engine| {
        timed_curve_is_over(e, Ptr::new(e.global::<u32>(HEADING_CURVE)))
            && timed_curve_is_over(e, Ptr::new(e.global::<u32>(PITCH_CURVE)))
    };
    if e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() == 2
        && e.call(0x0075_5680, &args![input]).u8() != 0
        && curves_idle(e)
    {
        e.call(0x0074_7d00, &args![input]);
        let selected = e.call(0x0073_b020, &args![input]).i32();
        if selected >= 0 {
            let menu_value = e.call(0x0071_5da0, &args![]).f64();
            let selected_value = selected as f64;
            if menu_value >= selected_value {
                let threshold = setting_float(e, 0x011c_e8b8);
                let divisor = e.call(0x0071_5da0, &args![]).f64();
                let ratio = (selected_value / divisor) as f32;
                covered = true;
                let mut highest = 0.0f32;
                let mut lowest = 0.0f32;
                let owner = e.call(0x0041_81e0, &args![actor]).u32();
                let data = e.vcall(owner, 0x180, &args![]).u32();
                let parts = e.call(0x006a_9540, &args![data]).u32();
                for index in 0..15u32 {
                    if e.mem.u32(parts + 4 * index) == 0 {
                        continue;
                    }
                    let part = e.call(0x005e_50f0, &args![data, index]).u32();
                    let node = body_part_node(e, actor, part);
                    if node == 0 {
                        continue;
                    }
                    e.with_stack(12, |e, screen| {
                        let position = e.call(0x0045_bb80, &args![node]).u32();
                        project_to_screen(e, position, screen.addr());
                    });
                    let position = e.call(0x0045_bb80, &args![node]).u32();
                    let part_height = e.mem.f32(position + 8);
                    if highest == 0.0 || highest < part_height {
                        highest = part_height;
                    }
                    if lowest == 0.0 || part_height < lowest {
                        lowest = part_height;
                    }
                }
                let angles = angles_from_camera(e, aim);
                let height = f32::from_bits(aim[2]);
                if threshold > ratio {
                    e.set_global(LOOK_HEIGHT_OFFSET, (highest as f64 - height as f64) as f32);
                } else if ratio as f64 > 1.0 - threshold as f64 {
                    e.set_global(LOOK_HEIGHT_OFFSET, (lowest as f64 - height as f64) as f32);
                } else {
                    let looking = e.call(0x0093_1d70, &args![this]).f32();
                    let mut keep = angles[0] < looking && (ratio as f64) < threshold as f64 * 2.0;
                    if !keep {
                        let looking = e.call(0x0093_1d70, &args![this]).f32();
                        keep = angles[0] > looking && (ratio as f64) > 1.0 - threshold as f64 * 2.0;
                    }
                    if keep {
                        e.set_global(LOOK_HEIGHT_OFFSET, saved_offset);
                    } else {
                        covered = false;
                    }
                }
            }
        }
    }
    if !covered
        && e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() != 3
        && e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() != 1
        && e.mem.u8(SMOOTH_LOOK_ENABLED) != 0
        && curves_idle(e)
    {
        let owner = e.call(0x0041_81e0, &args![actor]).u32();
        let data = e.vcall(owner, 0x180, &args![]).u32();
        if data != 0 {
            let part = e.call(0x005e_5130, &args![data, part_index]).u32();
            let mut acquired = 0u32;
            let current = e.global::<u32>(LOOK_TARGET_ACTOR);
            if e.vcall(current, 0x100, &args![]).u8() != 0 {
                let process = e.call(0x008d_8520, &args![current]).u32();
                let current = e.global::<u32>(LOOK_TARGET_ACTOR);
                let object = e.vcall(current, 0x1e8, &args![]).u32();
                acquired = e.vcall(process, 0x190, &args![object]).u32();
            }
            let node = if part != 0 {
                body_part_node(e, actor, part)
            } else {
                acquired
            };
            if node != 0 {
                let target = node_position(e, node);
                let outputs = e.with_stack(20, |e, frame| {
                    for (i, word) in target.iter().enumerate() {
                        e.mem.set_u32(frame.addr() + 8 + 4 * i as u32, *word);
                    }
                    project_to_screen(e, frame.addr() + 8, frame.addr());
                    e.mem.f32(frame.addr() + 4)
                });
                let threshold = setting_float(e, 0x011c_ef2c);
                let doubled = threshold as f64 + threshold as f64;
                if (outputs as f64) < doubled || (outputs as f64) > 1.0 - doubled {
                    let offset = f32::from_bits(target[2]) as f64 - f32::from_bits(aim[2]) as f64;
                    e.set_global(LOOK_HEIGHT_OFFSET, offset as f32);
                }
            }
        }
    }
    aim
}

/// Rebuilds [`LOOK_CENTER`] for `actor`: the sum of the positions of its
/// body part nodes divided by their count. When the torso node (part 1) is
/// within the distance `011a3b60` of the player it alone counts, otherwise
/// every one of the 15 parts that has a node.
fn rebuild_look_center(e: &mut Engine, actor: u32) {
    let owner = e.call(0x0041_81e0, &args![actor]).u32();
    let data = e.vcall(owner, 0x180, &args![]).u32();
    let parts = e.call(0x006a_9540, &args![data]).u32();
    e.with_stack(0x30, |e, frame| {
        let sum = frame.addr();
        let point = sum + 12;
        let quotient = sum + 24;
        let difference = sum + 36;
        for i in 0..3 {
            let word = e.mem.u32(ZERO_VECTOR + 4 * i);
            e.mem.set_u32(sum + 4 * i, word);
        }
        let mut count = 0u32;
        let mut torso = 0u32;
        if e.mem.u32(parts + 4) != 0 {
            let part = e.call(0x005e_50f0, &args![data, 1u32]).u32();
            torso = body_part_node(e, actor, part);
        }
        let mut only_torso = false;
        if torso != 0 {
            let player = e.global::<u32>(PLAYER_SINGLETON);
            let root = e.vcall(player, 0x1d0, &args![]).u32();
            let player_position = e.call(0x0045_bb80, &args![root]).u32();
            let torso_position = e.call(0x0045_bb80, &args![torso]).u32();
            let offset = e
                .call(
                    0x0043_9ef0,
                    &args![torso_position, difference, player_position],
                )
                .u32();
            let distance = e.call(0x0045_7990, &args![offset]).f64();
            let limit = e.mem.i32(0x011a_3b60) as f64;
            if distance < limit {
                only_torso = true;
                count += 1;
                let words = node_position(e, torso);
                for (i, word) in words.iter().enumerate() {
                    e.mem.set_u32(point + 4 * i as u32, *word);
                }
                e.call(0x0063_c8a0, &args![sum, point]);
            }
        }
        if !only_torso {
            for index in 0..15u32 {
                if e.mem.u32(parts + 4 * index) == 0 {
                    continue;
                }
                let part = e.call(0x005e_50f0, &args![data, index]).u32();
                let node = body_part_node(e, actor, part);
                if node != 0 {
                    count += 1;
                    let words = node_position(e, node);
                    for (i, word) in words.iter().enumerate() {
                        e.mem.set_u32(point + 4 * i as u32, *word);
                    }
                    e.call(0x0063_c8a0, &args![sum, point]);
                }
            }
        }
        if count != 0 {
            let average = e
                .call(0x0053_d280, &args![sum, quotient, count as f32])
                .u32();
            let words = read_words(e, average);
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(LOOK_CENTER + 4 * i as u32, *word);
            }
        }
    });
}

/// Mode 0 of [`fn_009445b0`]: reads the look input and applies it.
///
/// Returns 0 at once while the player is AI controlled (`0093a740`).
/// Otherwise it clears [`LAST_LOOK_TARGET`] and [`LOOK_HEIGHT_OFFSET`], and
/// scales the frame time by the player animation's `004e3d00` value (animation
/// `00950a60` of `004eaf60 == 0`). The input state (`00877720` of the global
/// at `011dea0c`) gives the yaw, pitch and zoom steps: with the analog
/// code (`004b71d0`) from the sticks (`00a23390` for the two axes, dead zone
/// 0x21F1, with a normalised response curve shaped by the settings
/// `011e0a90`, `011e08a0`, `011e0818`, `011e0928`, `011e08b8` when the
/// second stick flag is off), otherwise from `00a239e0` for 1, 2 and 3. The
/// pitch is inverted by the byte setting `011e0a5c`, and everything is
/// cleared while `005a03f0(this, 2)` holds.
///
/// A zoom step (when the Pipboy is not active) changes the zoom distance
/// [`ZOOM_DISTANCE`], switches to first or third person, and clamps it. Then
/// the player's heading and pitch are remembered, and the yaw and pitch steps
/// are applied: in the free camera modes by accumulating angles, otherwise
/// by turning the player (`00931d30`, `00931e50`, or the sitting heading
/// delta in mode 4 of slot `0x214`) and setting bits `0x10` / `0x20` of the
/// input flag word. The result is 1 when any step was non-zero (and in the
/// non-free-camera zoom case).
///
/// The compiler's temporary `NiPoint3` constructor calls (`006815c0`) are
/// omitted.
fn apply_look_input(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    time_step: f32,
    aim_pressed: bool,
    flags: Ptr,
) -> u8 {
    if fn_0093a740(e, this) {
        return 0;
    }
    let mut result = 0u8;
    let holder = e.global::<u32>(0x011d_ea0c);
    let input = e.call(0x0087_7720, &args![holder]).u32();
    e.set_global(LAST_LOOK_TARGET, 0u32);
    e.set_global(LOOK_HEIGHT_OFFSET, 0.0f32);
    let mut yaw: i32;
    let mut pitch = 0i32;
    let mut zoom = 0i32;
    let unflipped = e.call(0x004e_af60, &args![this]).u8() == 0;
    let animation = e.call(0x0095_0a60, &args![this, unflipped as u32]).u32();
    let animation_scale = e.call(0x004e_3d00, &args![animation]).f64();
    let time_step = (animation_scale * time_step as f64) as f32;
    e.call(0x004b_7210, &args![]);
    if e.call(0x004b_71d0, &args![]).u8() != 0 {
        if e.mem.u8(SECOND_STICK_FLAG) != 0 {
            let mut right_x = e.call(0x00a2_3390, &args![input, 0u32, 0xau32]).i32();
            if e.call(0x00ec_7d40, &args![right_x]).i32() < 0x21f1 {
                right_x = 0;
            }
            let response = e.global::<f64>(0x0101_ffa0);
            zoom = truncate(e, right_x as f64 * response * time_step as f64);
            yaw = e.call(0x00a2_3390, &args![input, 0u32, 9u32]).i32();
            if e.call(0x00ec_7d40, &args![yaw]).i32() < 0x21f1 {
                yaw = 0;
            }
            yaw /= 5;
            if yaw != 0 || zoom != 0 {
                let zoom_size = e.call(0x00ec_7d40, &args![right_x]).i32();
                let yaw_size = e.call(0x00ec_7d40, &args![yaw]).i32();
                if zoom_size > yaw_size {
                    yaw = 0;
                } else {
                    zoom = 0;
                }
            }
        } else {
            yaw = e.call(0x00a2_3390, &args![input, 0u32, 9u32]).i32();
            pitch = e.call(0x00a2_3390, &args![input, 0u32, 0xau32]).i32();
            if e.call(0x00ec_7d40, &args![yaw]).i32() < 0x21f1 {
                yaw = 0;
            }
            if e.call(0x00ec_7d40, &args![pitch]).i32() < 0x21f1 {
                pitch = 0;
            }
        }
        if yaw != 0 || pitch != 0 {
            let mut yaw_negative = false;
            if yaw < 0 {
                yaw = yaw.wrapping_neg();
                yaw_negative = true;
            }
            let mut pitch_up = true;
            if pitch < 0 {
                pitch = pitch.wrapping_neg();
                pitch_up = false;
            }
            let range = e.global::<f64>(0x0102_9790);
            let (stick_x, stick_y, mut length) = e.with_stack(12, |e, vector| {
                let x = (yaw as f64 / range) as f32;
                let y = (pitch as f64 / range) as f32;
                e.mem.set_f32(vector.addr(), x);
                e.mem.set_f32(vector.addr() + 4, y);
                e.mem.set_f32(vector.addr() + 8, 0.0);
                (x, y, e.call(0x0045_7990, &args![vector]).f32())
            });
            if length as f64 > e.global::<f64>(0x0101_2070) {
                length = 1.0;
            }
            let unit_x = (stick_x as f64 / length as f64) as f32;
            let unit_y = (stick_y as f64 / length as f64) as f32;
            let mut yaw_curve = 1.0f32;
            let mut pitch_curve = 1.0f32;
            let mut step = 0;
            while step < setting_int(e, 0x011e_0a90) {
                yaw_curve = (yaw_curve as f64 * length as f64) as f32;
                step += 1;
            }
            let mut step = 0;
            while step < setting_int(e, 0x011e_08a0) {
                pitch_curve = (pitch_curve as f64 * length as f64) as f32;
                step += 1;
            }
            let floor = setting_float(e, 0x011e_0818);
            let yaw_speed = setting_float(e, 0x011e_0928);
            let pitch_speed = setting_float(e, 0x011e_08b8);
            if yaw_curve < floor {
                yaw_curve = floor;
            }
            if pitch_curve < floor {
                pitch_curve = floor;
            }
            yaw = truncate(
                e,
                yaw_curve as f64 * unit_x as f64 * yaw_speed as f64 * time_step as f64,
            );
            pitch = truncate(
                e,
                pitch_curve as f64 * unit_y as f64 * pitch_speed as f64 * time_step as f64,
            );
            if yaw_negative {
                yaw = yaw.wrapping_neg();
            }
            if pitch_up {
                pitch = pitch.wrapping_neg();
            }
        }
    } else {
        yaw = e.call(0x00a2_39e0, &args![input, 1u32]).i32();
        pitch = e.call(0x00a2_39e0, &args![input, 2u32]).i32();
        zoom = e.call(0x00a2_39e0, &args![input, 3u32]).i32();
    }
    let invert = e.call(SETTING_BYTE_GETTER, &args![0x011e_0a5cu32]).u32();
    if e.mem.u8(invert) != 0 {
        pitch = pitch.wrapping_neg();
    }
    if e.call(0x005a_03f0, &args![this, 2u32]).u8() != 0 {
        yaw = 0;
        pitch = 0;
        zoom = 0;
    }
    if zoom != 0 && e.call(0x0096_7ae0, &args![this]).u8() == 0 {
        if e.mem.u8(FREE_CAMERA_FLAG) == 0 {
            result = 1;
        }
        if e.mem.u8(FREE_CAMERA_FLAG) != 0 || e.get(this, PlayerCharacter::b3rdPerson) {
            let distance = e.global::<f32>(ZOOM_DISTANCE) as f64;
            let scaled = zoom as f64 / e.global::<f64>(0x0107_18b8) * distance;
            let setting = if zoom > 0 { 0x011c_dc98 } else { 0x011c_d2ac };
            let speed = setting_float(e, setting);
            e.set_global(ZOOM_DISTANCE, (distance - speed as f64 * scaled) as f32);
            let in_first_person = e.vcall(this.addr(), 0x22c, &args![0u32]).u8() != 0;
            if in_first_person {
                let limit = setting_float(e, 0x011c_d614);
                if e.global::<f32>(ZOOM_DISTANCE) < limit {
                    let limit = setting_float(e, 0x011c_d614);
                    e.set_global(ZOOM_DISTANCE, limit);
                }
            } else {
                let limit = setting_float(e, 0x011c_da94);
                if e.global::<f32>(ZOOM_DISTANCE) < limit {
                    if !fn_0093a740(e, this)
                        && e.mem.u8(FREE_CAMERA_FLAG) == 0
                        && !e.get(this, PlayerCharacter::bTemp3rdPerson)
                        && e.call(0x005a_03f0, &args![this, 0x10u32]).u8() == 0
                    {
                        e.call(0x0095_0110, &args![this, 1u32]);
                    }
                    let limit = setting_float(e, 0x011c_da94);
                    e.set_global(ZOOM_DISTANCE, limit);
                }
            }
            let mut ceiling = setting_float(e, 0x011c_d568);
            if e.mem.u8(FREE_CAMERA_FLAG) != 0 {
                ceiling = setting_float(e, 0x011c_de14);
            }
            if ceiling < e.global::<f32>(ZOOM_DISTANCE) {
                e.set_global(ZOOM_DISTANCE, ceiling);
            }
        }
        if !fn_0093a740(e, this)
            && !e.get(this, PlayerCharacter::bTemp1stPerson)
            && !e.get(this, PlayerCharacter::b3rdPerson)
            && zoom < 0
            && e.call(0x005a_03f0, &args![this, 0x10u32]).u8() == 0
        {
            e.call(0x0095_0110, &args![this, 0u32]);
        }
    }
    let view_limit = setting_float(e, 0x0120_315c);
    let camera = e.call(0x0045_c670, &args![]).u32();
    let view_distance = e.call(0x0099_e040, &args![camera]).f32();
    let mut view_scale = 1.0f32;
    if view_limit > view_distance {
        view_scale = (view_distance as f64 / view_limit as f64) as f32;
    }
    let heading = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
    e.set_global(LAST_HEADING, heading);
    let pitch_now = e.call(0x0093_1d70, &args![this]).f32();
    e.set_global(LAST_PITCH, pitch_now);
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    if (e.mem.u8(FREE_CAMERA_FLAG) != 0 || e.mem.u8(SECOND_STICK_FLAG) != 0)
        && e.vcall(process, 0x610, &args![]).u32() == 0
    {
        let step = yaw as f64 * e.global::<f64>(0x0102_3128);
        let speed = setting_float(e, 0x011c_dd90);
        let turned =
            speed as f64 * step * time_step as f64 + e.global::<f32>(ACCUMULATED_YAW) as f64;
        e.set_global(ACCUMULATED_YAW, turned as f32);
        if e.call(0x004b_71d0, &args![]).u8() == 0 {
            let step = pitch as f64 * e.global::<f64>(0x0102_3128);
            let speed = setting_float(e, 0x011c_d444);
            let turned =
                speed as f64 * step * time_step as f64 + e.global::<f32>(ACCUMULATED_PITCH) as f64;
            e.set_global(ACCUMULATED_PITCH, turned as f32);
        }
    } else {
        if yaw != 0 || aim_pressed {
            let sensitivity = setting_float(e, 0x011e_0a6c);
            let turn = (sensitivity as f64 * yaw as f64) as f32;
            if flags != Ptr::NULL {
                if yaw != 0 && turn < 0.0 {
                    or_word(e, flags.addr(), 0x10);
                } else if yaw != 0 {
                    or_word(e, flags.addr(), 0x20);
                }
            }
            if e.call(0x004f_8960, &args![this]).u32() == 0
                && e.call(0x008a_7570, &args![this]).u32() != 0xa
                && !fn_0093a740(e, this)
            {
                let turn_mode = e.vcall(this.addr(), 0x214, &args![]).u32();
                if turn_mode == 4 {
                    let delta = e.get(this, PlayerCharacter::fSitHeadingDelta);
                    let delta = (delta as f64 + turn as f64) as f32;
                    e.set(this, PlayerCharacter::fSitHeadingDelta, delta);
                    if (delta as f64) < e.global::<f64>(0x0108_b098) {
                        let low = e.global::<f32>(0x0101_6b78);
                        e.set(this, PlayerCharacter::fSitHeadingDelta, low);
                    } else {
                        let delta = e.get(this, PlayerCharacter::fSitHeadingDelta);
                        if delta as f64 > e.global::<f64>(0x0103_0f38) {
                            let high = e.global::<f32>(0x0101_ff38);
                            e.set(this, PlayerCharacter::fSitHeadingDelta, high);
                        }
                    }
                    if aim_pressed {
                        let speed = e.global::<f32>(0x0101_6088);
                        e.set_global(TURN_SPEED, speed);
                    }
                } else if turn_mode == 0 {
                    let magnitude = e.call(0x0040_8860, &args![turn]).f64();
                    let sensitivity = setting_float(e, 0x011e_0a6c);
                    let divisor = sensitivity as f64 * e.global::<f64>(0x0103_56d8);
                    e.set_global(TURN_SPEED, (magnitude / divisor) as f32);
                    if e.global::<f32>(TURN_SPEED) as f64 > e.global::<f64>(0x0101_6ff0) {
                        let speed = e.global::<f32>(0x0101_6088);
                        e.set_global(TURN_SPEED, speed);
                    }
                    let scaled = (turn as f64 * view_scale as f64) as f32;
                    e.call(0x0093_1d30, &args![this, scaled]);
                }
            }
        }
        if pitch != 0 {
            let sensitivity = setting_float(e, 0x011e_0a6c);
            let turn = (sensitivity as f64 * pitch as f64 * view_scale as f64) as f32;
            e.call(0x0093_1e50, &args![this, turn]);
        }
    }
    if yaw != 0 || pitch != 0 || zoom != 0 {
        result = 1;
    }
    result
}

/// Size of the frame of [`fn_009466d0`]: the game keeps these locals on its
/// stack and passes their addresses.
const INPUT_FRAME_SIZE: u32 = 0x100;
/// Offsets in that frame: the profiling scope object; the 16-bit input flag
/// word; the 3x3 matrix copied from the camera; the two direction vectors; the
/// scratch point; the sound handle; the query point.
const INPUT_GUARD: u32 = 0x00;
const INPUT_FLAGS: u32 = 0x20;
const INPUT_MATRIX: u32 = 0x28;
const INPUT_AXIS_FORWARD: u32 = 0x50;
const INPUT_AXIS_SIDE: u32 = 0x5c;
const INPUT_SCRATCH_POINT: u32 = 0x68;
const INPUT_SOUND_HANDLE: u32 = 0x74;
const INPUT_QUERY_POINT: u32 = 0x80;

/// A global flag byte (`011e0bec`) set once the weapon-draw input was
/// handled, and the float (`011e07e0`) that times the draw button hold.
const DRAW_INPUT_HANDLED: u32 = 0x011e_0bec;
const DRAW_BUTTON_TIMER: u32 = 0x011e_07e0;
/// The float global (`011a3b3c`) the movement stick sets (up to 1).
const MOVE_SPEED_FACTOR: u32 = 0x011a_3b3c;
/// The object (`011de134`) the save and load hotkeys call.
const SAVE_LOAD_MANAGER: u32 = 0x011d_e134;

/// Whether the input state `input` reports the button `button` in `state`
/// (`00a24660`).
fn button_state(e: &mut Engine, input: u32, button: u32, state: u32) -> bool {
    e.call(0x00a2_4660, &args![input, button, state]).u32() != 0
}

/// Plays the sound called `name` once: `QInstance` (`00453a70`),
/// `GetSoundHandleByName` into the handle at `handle`, `BSSoundHandle::Play`,
/// then destroys the handle.
fn play_sound_once(e: &mut Engine, handle: u32, name: u32) {
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let found = e
        .call(
            AUDIO_GET_SOUND_HANDLE_BY_NAME,
            &args![audio, handle, name, 0x0004_0102u32],
        )
        .u32();
    e.call(SOUND_HANDLE_PLAY, &args![found, 0u32]);
    e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
}

/// The weapon draw button is held and nothing else is happening to the actor
/// (the part of the input code after its timer was advanced): once per press
/// it either sheathes the weapon (`008a6840(this, 0)`) when the button was
/// held longer than the setting `011cdfcc`, or there is no weapon or the
/// weapon has no ammunition and `008a6970` says it is out, or draws it
/// (`008a6840(this, 1)`) unless the actor's `008846e0` bits have `0x800`.
fn draw_button_held(e: &mut Engine, this: Ptr<PlayerCharacter>, weapon_kind: u32) {
    if e.mem.u8(DRAW_INPUT_HANDLED) != 0 {
        return;
    }
    let threshold = setting_float(e, 0x011c_dfcc);
    let timer = e.global::<f32>(DRAW_BUTTON_TIMER);
    let mut sheathe = (threshold as f64) < timer as f64;
    if !sheathe {
        if weapon_kind == 0 {
            sheathe = true;
        } else {
            let player = e.global::<u32>(PLAYER_SINGLETON);
            sheathe = e.call(0x0052_5980, &args![weapon_kind, player]).u32() == 0;
        }
    }
    if sheathe && e.call(0x008a_6970, &args![this]).u8() != 0 {
        e.call(0x008a_6840, &args![this, 0u32]);
        e.mem.set_u8(DRAW_INPUT_HANDLED, 1);
        return;
    }
    if e.call(0x008a_6970, &args![this]).u8() != 0 {
        return;
    }
    let bits = e.call(0x0088_46e0, &args![this]).u32();
    if bits & 0x800 == 0 {
        e.call(0x008a_6840, &args![this, 1u32]);
    }
    e.mem.set_u8(DRAW_INPUT_HANDLED, 1);
}

/// The sneak button was pressed (state 1 of button 8): when nothing blocks
/// it (slots `0x234`, `0x22c(0)`, `0x230`, `00437bf0`, `00437bd0` all false
/// and `0x214` zero) toggles bit `0x400` of the flag word at `flags`, plays
/// "NPCHumanCrouchUp" (bit was set) or "NPCHumanCrouchDown" and calls slot
/// `0x340` with 0xB or 0xA.
fn sneak_toggle(e: &mut Engine, this: Ptr<PlayerCharacter>, flags: u32, handle: u32) {
    if e.vcall(this.addr(), 0x234, &args![]).u8() == 0
        && e.vcall(this.addr(), 0x22c, &args![0u32]).u8() == 0
        && e.vcall(this.addr(), 0x230, &args![]).u8() == 0
        && e.call(0x0043_7bf0, &args![this]).u8() == 0
        && e.call(0x0043_7bd0, &args![this]).u8() == 0
        && e.vcall(this.addr(), 0x214, &args![]).u32() == 0
    {
        let mut state = 0xbu32;
        let word = e.mem.u16(flags);
        if word & 0x400 != 0 {
            e.mem.set_u16(flags, word & 0xfbff);
            play_sound_once(e, handle, 0x0108_b064);
        } else {
            e.mem.set_u16(flags, word | 0x400);
            state = 0xa;
            play_sound_once(e, handle, 0x0108_b050);
        }
        e.vcall(this.addr(), 0x340, &args![state]);
    }
}

/// The aim (button 6) and block handling of [`fn_009466d0`], with the actor
/// not blocked and not grabbing: slot `0x3F8` of the process decides whether
/// `00948310` runs, else `011e07ac`/`011e07b0` are cleared. With the aim
/// button down (states 1 or 0) and no iron sights, not in the process's
/// `0x3E4` state 7, and the weapon drawn: while the Pipboy is not active and
/// slot `0x3F8` holds it raises the iron sights (`Actor::SetIronSights`)
/// when the weapon is drawn, the equipped kind is set and `006450c0` of it is
/// false, otherwise blocks (`Actor::SetBlock(1)`). With the button released
/// (state 2, or not down, or the Pipboy active) it lowers them.
fn aim_and_block_input(e: &mut Engine, this: Ptr<PlayerCharacter>, input: u32, weapon_kind: u32) {
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    if e.vcall(process, 0x3f8, &args![]).u8() != 0 {
        fn_00948310(e, this);
    } else {
        e.set_global(0x011e_07ac, 0u32);
        e.set_global(0x011e_07b0, 0.0f32);
    }
    let mut aiming = false;
    if button_state(e, input, 6, 1) || button_state(e, input, 6, 0) {
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        let state = e.vcall(process, 0x3e4, &args![]).u32();
        if state != 7
            && e.call(0x008b_bc10, &args![this]).u8() == 0
            && e.call(0x008a_16d0, &args![this]).u8() != 0
        {
            aiming = true;
        }
    }
    if aiming {
        if e.call(0x0096_7ae0, &args![this]).u8() == 0 {
            let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
            if e.vcall(process, 0x3f8, &args![]).u8() != 0 {
                if e.call(0x008a_16d0, &args![this]).u8() != 0
                    && weapon_kind != 0
                    && e.call(0x0064_50c0, &args![weapon_kind]).u8() == 0
                {
                    e.call(0x008b_b650, &args![this, 1u32, 0u32, 0u32]);
                } else {
                    e.call(0x0089_4cc0, &args![this, 1u32]);
                }
            }
        }
    } else if button_state(e, input, 6, 2)
        || !button_state(e, input, 6, 0)
        || e.call(0x0096_7ae0, &args![this]).u8() != 0
    {
        let mut lower = e.call(0x008b_bc10, &args![this]).u8() != 0;
        if !lower {
            let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
            lower = e.vcall(process, 0x3e4, &args![]).u32() == 7;
        }
        if lower {
            e.call(0x008b_b650, &args![this, 0u32, 0u32, 0u32]);
            e.call(0x0089_4cc0, &args![this, 0u32]);
        }
    }
}

/// The attack button (12, state 1) handling of [`fn_009466d0`]: unless the
/// actor is in the animation action 0xB or the attack group
/// (`vcall 0x1E4(4)` -> `004301b0` -> `TESAnimGroup::IsPowerAttackAction`) is
/// a power attack, works out whether the attack can start (`008849c0` and
/// `005a2030` false) and whether the character controller (`009306d0`) is
/// moving (through `00944400` and slot `0x254`, or `00894f90` of the flag
/// byte while the aim button is down and nothing is selected in the menu),
/// runs `0095f6a0` and, when both hold, `00944400` again.
fn attack_input(e: &mut Engine, this: Ptr<PlayerCharacter>, input: u32, flags: u32) {
    // 00947321: slot 0x1e4 takes no stack word; the pushed 4 is the word of
    // `004301b0` (RET 4).
    let group = e.vcall(this.addr(), 0x1e4, &args![]).u32();
    let action = e.call(0x0043_01b0, &args![group, 4u32]).u16();
    if e.call(0x005f_2670, &args![action as u32]).u8() != 0 {
        return;
    }
    if e.call(0x008a_7570, &args![this]).u32() == 0xb {
        return;
    }
    let can_start = !(e.call(0x0088_49c0, &args![this]).u8() != 0
        || e.call(0x005a_2030, &args![this]).u8() != 0);
    let mut moving = false;
    let aim_down = button_state(e, input, 6, 0);
    let blocked = e.call(0x0089_4d60, &args![this]).u8() != 0;
    if aim_down {
        let menu_ref = e.call(0x0070_3350, &args![]).u32();
        let mask = e.mem.u16(flags) | 0xf;
        if menu_ref == 0 && mask != 0 {
            let controller = e.call(0x0093_06d0, &args![this]).u32();
            if e.call(0x005c_0880, &args![controller]).u32() == 0 {
                let flag_byte = e.mem.u8(flags) as u32;
                let value = e.call(0x0089_4f90, &args![this, flag_byte]).u32();
                moving = value != 0xff;
            }
        } else if !blocked {
            let controller = e.call(0x0093_06d0, &args![this]).u32();
            if fn_00944400(e, Ptr::new(controller)) {
                moving = e.vcall(this.addr(), 0x254, &args![]).u32() != 0;
            }
        }
    } else {
        let controller = e.call(0x0093_06d0, &args![this]).u32();
        if fn_00944400(e, Ptr::new(controller)) {
            moving = e.vcall(this.addr(), 0x254, &args![]).u32() != 0;
        }
    }
    e.call(0x0095_f6a0, &args![this]);
    if can_start && moving {
        let controller = e.call(0x0093_06d0, &args![this]).u32();
        fn_00944400(e, Ptr::new(controller));
    }
}

/// The activate button (5, state 1) handling of [`fn_009466d0`], when no
/// menu is open, the actor is not blocked by `0x234` and the drop spring
/// (`0095 13c0`) is not holding something: finds the reference under the
/// cursor (`00703350`) and, unless it is furniture used with iron sights, a
/// dynamic-cast object (`00ec43fb`) that `008ce390` rejects, or an actor
/// whose acquire object (`008d8520`) is in state 5 or 6 (slot `0x610`), clears
/// the info (`Interface::SetInfoForRef`) and activates it
/// (`TESObjectREFR::Activate`), falling back to the process's slot `0x4C8`
/// object when that did not activate anything.
fn activate_input(e: &mut Engine, this: Ptr<PlayerCharacter>, input: u32, drop_held: bool) {
    if !(button_state(e, input, 5, 1)
        && e.call(0x0070_2360, &args![]).u8() == 0
        && e.call(0x005a_03f0, &args![this, 1u32]).u8() == 0
        && e.vcall(this.addr(), 0x234, &args![]).u8() == 0
        && !drop_held)
    {
        return;
    }
    let reference = e.call(0x0070_3350, &args![]).u32();
    let mut activate_fallback = true;
    let mut skip = false;
    if reference != 0
        && e.call(0x0056_8680, &args![reference]).u8() != 0
        && e.call(0x008b_bc10, &args![this]).u8() != 0
    {
        skip = true;
    }
    if reference != 0 && e.vcall(reference, 0x224, &args![]).u8() != 0 {
        let cast = e
            .call(
                0x00ec_43fb,
                &args![reference, 0u32, 0x0118_41ccu32, 0x011a_28e0u32, 0u32],
            )
            .u32();
        if cast != 0 && e.call(0x008c_e390, &args![cast]).u32() == 0 {
            skip = true;
        }
    }
    if reference != 0 && e.vcall(reference, 0x100, &args![]).u8() != 0 {
        let acquire = e.call(0x008d_8520, &args![reference]).u32();
        if e.vcall(acquire, 0x610, &args![]).u32() == 5
            || e.vcall(acquire, 0x610, &args![]).u32() == 6
        {
            skip = true;
        }
    }
    if skip {
        return;
    }
    e.call(0x0070_62e0, &args![0u32, 0u32, 0u32]);
    if reference != 0
        && e.call(0x0057_3170, &args![reference, this, 0u32, 0u32, 1u32])
            .u8()
            != 0
    {
        activate_fallback = false;
    }
    if activate_fallback {
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        if e.vcall(process, 0x4c8, &args![]).u32() != 0 {
            let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
            let target = e.vcall(process, 0x4c8, &args![]).u32();
            e.call(0x0057_3170, &args![target, this, 0u32, 0u32, 1u32]);
        }
    }
}

// Translated from 009466d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player's per-frame input handling, called with the frame time and
/// whether to skip the player controls (which leaves only the save and load
/// hotkeys). In a profiling scope (`00404eb0`, file `PlayerCharacter.cpp`
/// line 0x17A3) it:
///
/// - clears the drop angle modifiers and `bOnElevator`, runs
///   [`player_character_update_ufo_camera`] and reads the two analog axes
///   (`00a23390` with 7 and 8) when `004b71d0` holds;
/// - with the controls active: tells the acquire object of the process about
///   the player (slot `0xD4`) when `b3rdPerson`, builds the movement flag word
///   from the actor's `008846e0` bits masked with `0xCC00`, runs the look
///   handling ([`fn_009445b0`]) unless the VATS mode is 0, remembers the
///   heading and pitch, handles the weapon draw button
///   ([`draw_button_held`]), the auto-move toggle, the sneak toggle
///   ([`sneak_toggle`]), builds the movement axes from the camera matrix,
///   applies the analog axes to `011a3b3c`/`011a3b40`, passes the flag word
///   to the mover (`pMover` at `+0x190`, slots `0xC` and `0x14`), handles the
///   aim, block ([`aim_and_block_input`]) and attack ([`attack_input`])
///   buttons, picks the animations, activates ([`activate_input`]), updates
///   the animation movement and the muzzle flash;
/// - in all cases handles the save (`0x19`) and load (`0x1A`) hotkeys with the
///   guard bytes `011e0be5` and `011e0be4`.
///
/// The byte local the original sets in several places (at `EBP-0xD`) is never
/// read and is not kept; the compiler's exception-unwinding state writes and
/// the NiPoint3 temporary constructor calls (`006815c0`, which return their
/// argument) are not translated.
pub fn fn_009466d0(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    time_step: f32,
    skip_controls: bool,
) {
    e.with_stack(INPUT_FRAME_SIZE, |e, frame| {
        input_in_frame(e, this, time_step, skip_controls, frame.addr());
    });
}

#[allow(clippy::cognitive_complexity)]
fn input_in_frame(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    time_step: f32,
    skip_controls: bool,
    frame: u32,
) {
    let guard = frame + INPUT_GUARD;
    let flags = frame + INPUT_FLAGS;
    e.call(
        0x0040_4eb0,
        &args![guard, 0x34u32, 1u32, UPDATE_GUARD_FILE, 0x17a3u32],
    );
    let main = e.global::<u32>(MAIN_SINGLETON);
    let input = e.call(0x0087_7720, &args![main]).u32();
    let camera = e.call(0x0045_c670, &args![]).u32();
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    let weapon_kind = if e.vcall(process, 0x148, &args![]).u32() != 0 {
        let item = e.vcall(process, 0x148, &args![]).u32();
        e.call(0x0044_ddc0, &args![item]).u32()
    } else {
        0
    };
    e.mem.set_u16(flags, 0);
    e.set(this, PlayerCharacter::fDropAngleMod, 0.0);
    e.set(this, PlayerCharacter::fLastDropAngleMod, 0.0);
    e.set(this, PlayerCharacter::bOnElevator, false);
    player_character_update_ufo_camera(e, this);
    let mut stick_x = 0i32;
    let mut stick_y = 0i32;
    e.call(0x004b_7210, &args![]);
    if e.call(0x004b_71d0, &args![]).u8() != 0 {
        stick_x = e.call(0x00a2_3390, &args![input, 0u32, 7u32]).i32();
        stick_y = e.call(0x00a2_3390, &args![input, 0u32, 8u32]).i32();
    }
    if !skip_controls {
        if e.get(this, PlayerCharacter::b3rdPerson) {
            let acquire = e.call(0x008d_8520, &args![this]).u32();
            e.vcall(acquire, 0xd4, &args![this]);
        }
        let bits = e.call(0x0088_46e0, &args![this]).u32() as u16;
        e.mem.set_u16(flags, bits & 0xcc00);
        if e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() != 0 {
            fn_009445b0(e, this, time_step, false, Ptr::new(flags), 0);
        }
        let heading = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
        e.set_global(LAST_HEADING, heading);
        let pitch = e.call(0x0093_1d70, &args![this]).f32();
        e.set_global(LAST_PITCH, pitch);
        let player = e.global::<u32>(PLAYER_SINGLETON);
        let player_process = e.mem.u32(player + ACTOR_CURRENT_PROCESS);
        if e.vcall(player_process, 0x22c, &args![]).u32() != 0 {
            let player_process = e.mem.u32(player + ACTOR_CURRENT_PROCESS);
            e.vcall(player_process, 0x234, &args![]);
        }
        let player_process = e.mem.u32(player + ACTOR_CURRENT_PROCESS);
        if e.vcall(player_process, 0x20c, &args![]).u32() != 0 {
            let player_process = e.mem.u32(player + ACTOR_CURRENT_PROCESS);
            e.vcall(player_process, 0x214, &args![]);
        }
        // The weapon draw button.
        if button_state(e, input, 7, 1) || button_state(e, input, 7, 0) {
            if e.call(0x005a_03f0, &args![this, 8u32]).u8() == 0 {
                let timer = e.global::<f32>(DRAW_BUTTON_TIMER);
                e.set_global(DRAW_BUTTON_TIMER, (timer as f64 + time_step as f64) as f32);
                if e.call(0x0088_43a0, &args![this]).u8() != 0
                    && e.call(0x008a_7570, &args![this]).u32() == 0xffff_ffff
                {
                    draw_button_held(e, this, weapon_kind);
                }
            }
        } else if !button_state(e, input, 7, 2) {
            e.set_global(DRAW_BUTTON_TIMER, 0.0f32);
            e.mem.set_u8(DRAW_INPUT_HANDLED, 0);
        }
        // The auto-move toggle.
        if button_state(e, input, 0xb, 1) {
            let toggled = !e.get(this, PlayerCharacter::bAutoMove);
            e.set(this, PlayerCharacter::bAutoMove, toggled);
        }
        if e.get(this, PlayerCharacter::bAutoMove) {
            if button_state(e, input, 0, 0)
                || button_state(e, input, 1, 0)
                || button_state(e, input, 3, 0)
                || button_state(e, input, 2, 0)
            {
                e.set(this, PlayerCharacter::bAutoMove, false);
            } else {
                e.call(0x00a2_4280, &args![input, 0u32]);
            }
        }
        // The sneak toggle.
        let mut sneak_pressed = false;
        if button_state(e, input, 8, 1) {
            sneak_toggle(e, this, flags, frame + INPUT_SOUND_HANDLE);
            sneak_pressed = true;
        }
        // The movement axes from the camera matrix.
        let cell = e.call(0x0055_8310, &args![camera]).u32();
        let matrix = e.call(0x006a_9540, &args![cell]).u32();
        for i in 0..9 {
            let word = e.mem.u32(matrix + 4 * i);
            e.mem.set_u32(frame + INPUT_MATRIX + 4 * i, word);
        }
        let position = e.call(0x0043_6aa0, &args![this]).u32();
        let position = read_words(e, position);
        let forward = frame + INPUT_AXIS_FORWARD;
        let side = frame + INPUT_AXIS_SIDE;
        let unit_x = e
            .call(
                0x0041_6870,
                &args![frame + INPUT_SCRATCH_POINT, 1.0f32, 0.0f32, 0.0f32],
            )
            .u32();
        let unit_x = read_words(e, unit_x);
        for (i, word) in unit_x.iter().enumerate() {
            e.mem.set_u32(side + 4 * i as u32, *word);
        }
        e.call(0x0043_9f50, &args![frame + INPUT_MATRIX, 1u32, forward]);
        if e.call(0x0050_d4a0, &args![this]).u8() == 0 {
            e.mem.set_f32(forward + 8, 0.0);
            e.call(0x004a_0c10, &args![forward]);
        } else {
            e.call(0x0043_9f50, &args![frame + INPUT_MATRIX, 0u32, side]);
        }
        let mut drop_held = false;
        if e.get(this, PlayerCharacter::eGrabType) == 2 {
            drop_held = e.call(0x0096_13c0, &args![this, time_step]).u8() != 0;
        }
        // The analog axes 7 and 8 (the camera stick and the move stick).
        if stick_x != 0 {
            if stick_x > 0 {
                e.call(0x00a2_4280, &args![input, 3u32]);
            } else if stick_x < 0 {
                e.call(0x00a2_4280, &args![input, 2u32]);
            }
            let size = e.call(0x00ec_7d40, &args![stick_x]).i32();
            let scale = setting_float(e, 0x011e_0964);
            e.set_global(TURN_SPEED, (scale as f64 * size as f64) as f32);
            if e.global::<f32>(TURN_SPEED) as f64 > e.global::<f64>(0x0101_6ff0) {
                let limit = e.global::<f32>(0x0101_6088);
                e.set_global(TURN_SPEED, limit);
            }
        }
        let size_y = e.call(0x00ec_7d40, &args![stick_y]).i32();
        let size_x = e.call(0x00ec_7d40, &args![stick_x]).i32();
        if size_y < size_x {
            let size_x = e.call(0x00ec_7d40, &args![stick_x]).i32();
            let factor = size_x as f64 * e.global::<f64>(0x0108_b048);
            e.set_global(MOVE_SPEED_FACTOR, factor as f32);
            if stick_x < 0 {
                or_word(e, flags, 4);
            } else {
                or_word(e, flags, 8);
            }
        } else if stick_y != 0 {
            if stick_y < 0 {
                e.call(0x00a2_4280, &args![input, 0u32]);
            } else if stick_y > 0 {
                e.call(0x00a2_4280, &args![input, 1u32]);
            }
            let size_y = e.call(0x00ec_7d40, &args![stick_y]).i32();
            let factor = size_y as f64 * e.global::<f64>(0x0108_b048);
            e.set_global(MOVE_SPEED_FACTOR, factor as f32);
        }
        if e.global::<f32>(MOVE_SPEED_FACTOR) as f64 > e.global::<f64>(0x0101_2070) {
            e.set_global(MOVE_SPEED_FACTOR, 1.0f32);
        }
        let speed = e.call(0x008a_0b10, &args![this]).f32();
        let factor = e.global::<f32>(MOVE_SPEED_FACTOR);
        let scaled = (speed as f64 * factor as f64) as f32;
        e.call(0x0043_9180, &args![side, scaled]);
        let factor = e.global::<f32>(MOVE_SPEED_FACTOR);
        let scaled = (speed as f64 * factor as f64) as f32;
        e.call(0x0043_9180, &args![forward, scaled]);
        let mover = e.mem.u32(at(this, 0x190));
        let word = e.mem.u16(flags) as u32;
        e.vcall(mover, 0xc, &args![word]);
        if sneak_pressed {
            e.call(0x0088_4f80, &args![this]);
        }
        e.set(this, PlayerCharacter::b3rdPerson, true);
        let blocked = e.vcall(this.addr(), 0x230, &args![]).u8() != 0
            || e.vcall(this.addr(), 0x234, &args![]).u8() != 0
            || e.call(0x004f_8960, &args![this]).u32() != 0
            || e.call(0x008a_7570, &args![this]).u32() == 0xa
            || e.vcall(this.addr(), 0x214, &args![]).u32() != 0;
        if blocked || e.call(0x005a_03f0, &args![this, 1u32]).u8() != 0 {
            let mover = e.mem.u32(at(this, 0x190));
            e.call(0x009e_a360, &args![mover, 0x33fu32, 0u32]);
        }
        if !blocked && e.call(0x005a_03f0, &args![this, 8u32]).u8() == 0 {
            if e.get(this, PlayerCharacter::eGrabType) == 2
                || e.get(this, PlayerCharacter::eGrabType) == 3
            {
                e.set_global(0x011e_07ac, 0u32);
                e.set_global(0x011e_07b0, 0.0f32);
            } else {
                aim_and_block_input(e, this, input, weapon_kind);
            }
            if button_state(e, input, 0xc, 1) && e.vcall(this.addr(), 0x358, &args![]).u8() == 0 {
                attack_input(e, this, input, flags);
            }
        }
        e.set(this, PlayerCharacter::b3rdPerson, true);
        e.vcall(this.addr(), 0x1e0, &args![]);
        let turn_speed = e.global::<f32>(TURN_SPEED);
        let move_speed = e.global::<f32>(MOVE_SPEED_FACTOR);
        e.call(0x0089_5110, &args![this, move_speed, turn_speed]);
        let actually = e.get(this, PlayerCharacter::bActually3rdPerson);
        e.set(this, PlayerCharacter::b3rdPerson, actually);
        if e.call(0x005a_1e50, &args![this]).u8() != 0 {
            e.call(0x0094_df80, &args![this]);
        }
        e.call(0x0069_3ef0, &args![this, 0u32]);
        let mover = e.mem.u32(at(this, 0x190));
        e.call(0x009e_a570, &args![mover, side]);
        let mover = e.mem.u32(at(this, 0x190));
        e.vcall(mover, 0x14, &args![time_step]);
        activate_input(e, this, input, drop_held);
        if e.global::<u32>(0x011e_07ec) != 0 {
            let count = e.global::<u32>(0x011e_0be8);
            e.set_global(0x011e_0be8, count.wrapping_add(1));
            if count < 0x14 {
                let query = frame + INPUT_QUERY_POINT;
                for (i, word) in position.iter().enumerate() {
                    e.mem.set_u32(query + 4 * i as u32, *word);
                }
                let up = e.global::<f64>(0x0102_e430);
                let across = e.global::<f64>(0x0102_40c0);
                let z = e.mem.f32(query + 8) as f64 + up;
                e.mem.set_f32(query + 8, z as f32);
                let y = e.mem.f32(query + 4) as f64 + across;
                e.mem.set_f32(query + 4, y as f32);
                let x = e.mem.f32(query) as f64 + across;
                e.mem.set_f32(query, x as f32);
                let object = e.global::<u32>(0x011e_07ec);
                e.call(0x0056_10f0, &args![object, query]);
                let object = e.global::<u32>(0x011e_07ec);
                e.call(0x0056_15d0, &args![object, 0x011f_426cu32]);
            }
        }
        if e.vcall(this.addr(), 0x22c, &args![0u32]).u8() == 0
            && e.vcall(this.addr(), 0x2e8, &args![]).u8() == 0
        {
            e.call(0x008c_3c40, &args![this, 0u32, 0u32]);
        }
        let third_person = e.call(0x0095_0a60, &args![this, 0u32]).u32();
        let first_person = e.call(0x0095_0a60, &args![this, 1u32]).u32();
        let regen = fn_009442a0(e, Ptr::new(third_person));
        e.call(0x0089_50f0, &args![first_person, regen]);
        let weight = e.call(0x0050_8070, &args![third_person]).f32();
        e.call(0x004c_0c90, &args![first_person, weight]);
        for _ in 0..2 {
            let flipped = !e.get(this, PlayerCharacter::b3rdPerson);
            e.set(this, PlayerCharacter::b3rdPerson, flipped);
            e.call(0x008d_3550, &args![this, time_step]);
            let using_first = !e.get(this, PlayerCharacter::b3rdPerson);
            let animation = e.call(0x0095_0a60, &args![this, using_first as u32]).u32();
            e.call(0x0088_85e0, &args![this, animation, time_step]);
        }
        e.call(0x008b_a600, &args![this]);
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        if e.vcall(process, 0x6b8, &args![]).u32() != 0 {
            let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
            let flash = e.vcall(process, 0x6b8, &args![]).u32();
            e.call(0x009b_b080, &args![flash, time_step, this]);
        }
    }
    if button_state(e, input, 0x19, 1) && e.mem.u8(0x011e_0be5) == 0 {
        e.mem.set_u8(0x011e_0be5, 1);
        let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
        e.call(0x0085_09a0, &args![manager]);
        e.mem.set_u8(0x011e_0be5, 0);
    }
    if button_state(e, input, 0x1a, 1) && e.mem.u8(0x011e_0be4) == 0 {
        e.mem.set_u8(0x011e_0be4, 1);
        let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
        e.call(0x0085_09f0, &args![manager]);
        e.mem.set_u8(0x011e_0be4, 0);
    }
    e.call(0x0040_4ee0, &args![guard]);
}
/// Size and offsets of the frame of [`player_character_update_ufo_camera`]:
/// the two rotation matrices and their product (36 bytes each), the move
/// vector, and two scratch points.
const UFO_FRAME_SIZE: u32 = 0xc0;
const UFO_MATRIX_Z: u32 = 0x00;
const UFO_MATRIX_X: u32 = 0x24;
const UFO_MATRIX_PRODUCT: u32 = 0x48;
const UFO_MOVE: u32 = 0x6c;
const UFO_SCRATCH: u32 = 0x78;
const UFO_ORIGIN: u32 = 0x84;

/// Zeroes `value` when `|value|` (`00408820`, taking the integer as a float)
/// is not above the dead zone 7849.0 (the double at `010718f0`).
fn ufo_dead_zone(e: &mut Engine, value: i32) -> i32 {
    let size = e.call(0x0040_8820, &args![value as f32]).f64();
    if size <= e.global::<f64>(0x0107_18f0) {
        0
    } else {
        value
    }
}

// Translated from 0094a8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdateUFOCamera` (Xbox PDB): the free ("UFO") camera.
/// Reads the input (`00877720`): the look axes 9 and 10 (dead zone 7849) turn
/// `fUFOCameraHeading` and `fUFOCameraPitch` (scaled by the setting at
/// `011e0a6c`, and divided by 1500 / -1500 with the analog code
/// `004b71d0`), the pitch inverted by the byte setting `011e0a5c`. A heading
/// and a pitch rotation matrix are multiplied (`NiMatrix3::operator*`); the
/// move vector comes from the axes 7 and 8 (dead zone, over 32767, times 15)
/// or the direction keys (+-10 on y for buttons 0 and 1, on x for 3 and 2;
/// button 9 replaces it with `0045bb20(vector, 2.0)`); it is scaled by the
/// elapsed time since the last call (`00457fe0` milliseconds, at most one
/// second, times 10), rotated by the matrix (`004b4500`), scaled by the
/// setting `011e090c` and added to `UFOCameraPos`. The camera object
/// (`0045c670`) then gets the position, rotation and an update, and the
/// world space's terrain manager (`00586170`) is updated with
/// `UFOCameraPos` and 0xF, and its morph parameters.
///
/// The NiPoint3 and NiMatrix3 temporary constructor calls (`006815c0`, which
/// return their argument) are not translated.
pub fn player_character_update_ufo_camera(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    e.with_stack(UFO_FRAME_SIZE, |e, frame| {
        ufo_camera_in_frame(e, this, frame.addr());
    });
}

fn ufo_camera_in_frame(e: &mut Engine, this: Ptr<PlayerCharacter>, frame: u32) {
    let main = e.global::<u32>(MAIN_SINGLETON);
    let input = e.call(0x0087_7720, &args![main]).u32();
    let mut look_x;
    let mut look_y;
    e.call(0x004b_7210, &args![]);
    if e.call(0x004b_71d0, &args![]).u8() != 0 {
        look_x = e.call(0x00a2_3390, &args![input, 0u32, 9u32]).i32();
        look_y = e.call(0x00a2_3390, &args![input, 0u32, 0xau32]).i32();
        look_x = ufo_dead_zone(e, look_x);
        look_y = ufo_dead_zone(e, look_y);
    } else {
        look_x = e.call(0x00a2_39e0, &args![input, 1u32]).i32();
        look_y = e.call(0x00a2_39e0, &args![input, 2u32]).i32();
    }
    let invert = e.call(SETTING_BYTE_GETTER, &args![0x011e_0a5cu32]).u32();
    if e.mem.u8(invert) != 0 {
        look_y = look_y.wrapping_neg();
    }
    e.call(0x004b_7210, &args![]);
    if e.call(0x004b_71d0, &args![]).u8() != 0 {
        let heading_step =
            setting_float(e, 0x011e_0a6c) as f64 * look_x as f64 / e.global::<f64>(0x0107_37c8);
        let heading = e.get(this, PlayerCharacter::fUFOCameraHeading);
        e.set(
            this,
            PlayerCharacter::fUFOCameraHeading,
            (heading_step + heading as f64) as f32,
        );
        let pitch_step =
            setting_float(e, 0x011e_0a6c) as f64 * look_y as f64 / e.global::<f64>(0x0108_b0a8);
        let pitch = e.get(this, PlayerCharacter::fUFOCameraPitch);
        e.set(
            this,
            PlayerCharacter::fUFOCameraPitch,
            (pitch_step + pitch as f64) as f32,
        );
    } else {
        let heading_step = setting_float(e, 0x011e_0a6c) as f64 * look_x as f64;
        let heading = e.get(this, PlayerCharacter::fUFOCameraHeading);
        e.set(
            this,
            PlayerCharacter::fUFOCameraHeading,
            (heading_step + heading as f64) as f32,
        );
        let pitch_step = setting_float(e, 0x011e_0a6c) as f64 * look_y as f64;
        let pitch = e.get(this, PlayerCharacter::fUFOCameraPitch);
        e.set(
            this,
            PlayerCharacter::fUFOCameraPitch,
            (pitch_step + pitch as f64) as f32,
        );
    }
    let matrix_z = frame + UFO_MATRIX_Z;
    let matrix_x = frame + UFO_MATRIX_X;
    let product = frame + UFO_MATRIX_PRODUCT;
    let heading = e.get(this, PlayerCharacter::fUFOCameraHeading);
    e.call(0x004a_0c90, &args![matrix_z, heading]);
    let pitch = e.get(this, PlayerCharacter::fUFOCameraPitch);
    e.call(0x0052_4ac0, &args![matrix_x, pitch]);
    e.call(0x0043_f8d0, &args![matrix_z, product, matrix_x]);
    let movement = frame + UFO_MOVE;
    for i in 0..3 {
        let word = e.mem.u32(ZERO_VECTOR + 4 * i);
        e.mem.set_u32(movement + 4 * i, word);
    }
    e.call(0x004b_7210, &args![]);
    if e.call(0x004b_71d0, &args![]).u8() != 0 {
        let mut stick_x = e.call(0x00a2_3390, &args![input, 0u32, 7u32]).i32();
        let mut stick_y = e.call(0x00a2_3390, &args![input, 0u32, 8u32]).i32();
        stick_x = ufo_dead_zone(e, stick_x);
        let range = e.global::<f64>(0x0102_9790);
        let move_x = (stick_x as f64 / range) as f32;
        stick_y = ufo_dead_zone(e, stick_y);
        let move_y = (stick_y as f64 / range) as f32;
        let speed = e.global::<f64>(0x0101_5a38);
        let x = (move_x as f64 * speed + e.mem.f32(movement) as f64) as f32;
        e.mem.set_f32(movement, x);
        let y = (move_y as f64 * speed + e.mem.f32(movement + 4) as f64) as f32;
        e.mem.set_f32(movement + 4, y);
    } else {
        let step = e.global::<f64>(0x0102_0758);
        if button_state(e, input, 0, 0) {
            let y = (e.mem.f32(movement + 4) as f64 + step) as f32;
            e.mem.set_f32(movement + 4, y);
        }
        if button_state(e, input, 1, 0) {
            let y = (e.mem.f32(movement + 4) as f64 - step) as f32;
            e.mem.set_f32(movement + 4, y);
        }
        if button_state(e, input, 3, 0) {
            let x = (e.mem.f32(movement) as f64 + step) as f32;
            e.mem.set_f32(movement, x);
        }
        if button_state(e, input, 2, 0) {
            let x = (e.mem.f32(movement) as f64 - step) as f32;
            e.mem.set_f32(movement, x);
        }
        if button_state(e, input, 9, 0) {
            let factor = e.global::<f32>(0x0101_62c0);
            let scaled = e
                .call(0x0045_bb20, &args![movement, frame + UFO_ORIGIN, factor])
                .u32();
            let words = read_words(e, scaled);
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(movement + 4 * i as u32, *word);
            }
        }
    }
    let now = e.call(0x0045_7fe0, &args![]).u32();
    let mut elapsed = 0.0f32;
    let last = e.global::<u32>(0x011e_0bf8);
    if last != 0 {
        elapsed = (now.wrapping_sub(last) as f64 / e.global::<f64>(0x0101_7b70)) as f32;
        elapsed = e.call(0x0040_ebd0, &args![elapsed, 1.0f32]).f32();
    }
    e.set_global(0x011e_0bf8, now);
    let scale = (elapsed as f64 * e.global::<f64>(0x0102_0758)) as f32;
    e.call(0x0043_9180, &args![movement, scale]);
    let rotated = e
        .call(0x004b_4500, &args![product, frame + UFO_ORIGIN, movement])
        .u32();
    let words = read_words(e, rotated);
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(movement + 4 * i as u32, *word);
    }
    let speed = setting_float(e, 0x011e_090c);
    e.call(0x0043_9180, &args![movement, speed]);
    let position = member(this, PlayerCharacter::UFOCameraPos);
    e.call(0x0063_c8a0, &args![position, movement]);
    let camera = e.call(0x0045_c670, &args![]).u32();
    let node = e.call(0x0055_8310, &args![camera]).u32();
    e.call(0x0044_0460, &args![node, position]);
    let node = e.call(0x0055_8310, &args![camera]).u32();
    e.call(0x0043_fa80, &args![node, product]);
    let scratch = frame + UFO_SCRATCH;
    e.call(0x0043_d410, &args![scratch, 0.0f32, 0u32, 0u32]);
    let node = e.call(0x0055_8310, &args![camera]).u32();
    e.call(0x00a5_9c60, &args![node, scratch]);
    if e.call(0x0057_5d70, &args![this]).u32() != 0 {
        let world_space = e.call(0x0057_5d70, &args![this]).u32();
        if e.call(0x0058_6170, &args![world_space]).u32() != 0 {
            let world_space = e.call(0x0057_5d70, &args![this]).u32();
            let terrain = e.call(0x0058_6170, &args![world_space]).u32();
            e.call(0x006f_ca90, &args![terrain, position, 0xfu32]);
            let world_space = e.call(0x0057_5d70, &args![this]).u32();
            let terrain = e.call(0x0058_6170, &args![world_space]).u32();
            e.call(0x006f_cdb0, &args![terrain, position]);
        }
    }
}

/// Globals of the attack input code: the queued attack (`011e07ac`: 0 none,
/// 1 normal, 2 power), the hold timer of a pending attack (`011e07b0`, float),
/// the latch bytes `011e07a9` (button consumed) and `011e07aa` (repeat), the
/// byte `011e0783`, the VATS attack delay counter (`011e0bf4`) and the word
/// `011e0bf0`; and the `CombatDialogueManager` (`011f1708`).
const ATTACK_QUEUED: u32 = 0x011e_07ac;
const ATTACK_HOLD_TIMER: u32 = 0x011e_07b0;
const ATTACK_BUTTON_LATCH: u32 = 0x011e_07a9;
const ATTACK_REPEAT_LATCH: u32 = 0x011e_07aa;
const ATTACK_RELEASE_BYTE: u32 = 0x011e_0783;
const VATS_ATTACK_DELAY: u32 = 0x011e_0bf4;
const ATTACK_COUNTER: u32 = 0x011e_0bf0;
const COMBAT_DIALOGUE_MANAGER: u32 = 0x011f_1708;

/// Whether the `slot` of the player's process (`pCurrentProcess`) reports
/// true (its low byte).
fn process_flag(e: &mut Engine, this: Ptr<PlayerCharacter>, slot: u32) -> bool {
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    e.vcall(process, slot, &args![]).u8() != 0
}

/// The animation group word of animation `animation` in slot `slot`
/// (`004301b0`).
fn animation_group(e: &mut Engine, animation: u32, slot: u32) -> u32 {
    e.call(0x0043_01b0, &args![animation, slot]).u16() as u32
}

/// The attack animation group `0051f5f0` gives for the weapon, or `None`
/// without a weapon or when it gives 0xFF.
fn weapon_attack_group(e: &mut Engine, weapon: u32) -> Option<u32> {
    if weapon != 0 && e.call(0x0051_f5f0, &args![weapon]).u32() != 0xff {
        Some(e.call(0x0051_f5f0, &args![weapon]).u32())
    } else {
        None
    }
}

/// The attack group for the animation type `00495e40` reports for
/// animation `animation`: the types 0x33 to 0x38 give 0x26, 0x2C, 0x32, 0x38,
/// 0x3E and 0x44, type 0x6C gives 0x1A, and any other the weapon's own group
/// or 0x20.
fn typed_attack_group(e: &mut Engine, animation: u32, weapon: u32) -> u32 {
    let kind = e.call(0x0049_5e40, &args![animation, 0u32]).u8() as i8 as i32;
    match kind {
        0x33 => 0x26,
        0x34 => 0x2c,
        0x35 => 0x32,
        0x36 => 0x38,
        0x37 => 0x3e,
        0x38 => 0x44,
        0x6c => 0x1a,
        _ => weapon_attack_group(e, weapon).unwrap_or(0x20),
    }
}

/// An entry point value of the player (`BGSEntryPoint::HandleEntryPoint`
/// with the entry point `entry` and the player, starting from 0.0).
fn entry_point_value(e: &mut Engine, entry: u32) -> f32 {
    let player = e.global::<u32>(PLAYER_SINGLETON);
    e.with_stack(4, |e, value| {
        e.mem.set_f32(value.addr(), 0.0);
        e.call(0x005e_58f0, &args![entry, player, value]);
        e.mem.f32(value.addr())
    })
}

/// The VATS part of [`fn_00948310`] (mode 4): clears the queued attack,
/// gets the current action (`009c71c0`) and, when no delay is counting down
/// (`011e0bf4`, which it decrements), the action is an attack
/// (`ActionPoints::IsAttackAction`, byte `+8` set, `+6` and `+0x24` clear) and
/// the animation and weapon conditions hold, sets the delay to 3, presses
/// button 4 (`00a24280`) and decrements the action's counter byte (`+8`) for
/// the action kinds 0, 1, 2, 0x10, 0x11, 0x14 and 0x15. Returns the action.
fn vats_attack_step(e: &mut Engine, anim0: u32, input: u32, weapon: u32) -> u32 {
    e.set_global(ATTACK_QUEUED, 0u32);
    let action = e.call(0x009c_71c0, &args![VATS_OBJECT]).u32();
    let delay = e.global::<u32>(VATS_ATTACK_DELAY);
    if delay != 0 {
        e.set_global(VATS_ATTACK_DELAY, delay.wrapping_sub(1));
        return action;
    }
    if action == 0 {
        return action;
    }
    let kind = e.mem.u32(action);
    if e.call(0x0066_dde0, &args![kind]).u8() == 0
        || e.mem.u8(action + 8) == 0
        || e.mem.u8(action + 6) != 0
        || e.mem.u8(action + 0x24) != 0
    {
        return action;
    }
    let melee = weapon != 0 && e.call(0x0052_4b40, &args![weapon]).u8() != 0;
    if !melee {
        let group = animation_group(e, anim0, 4);
        if e.call(0x005f_2440, &args![group]).i32() != 0x11 {
            let group = animation_group(e, anim0, 4);
            if e.call(0x005f_2440, &args![group]).i32() != 0x14 {
                return action;
            }
        }
        let current = e.call(0x0049_1040, &args![anim0, 4u32]).u32();
        if e.call(0x0080_41a0, &args![current]).u32() != 1 {
            return action;
        }
        for slot in [5u32, 6u32] {
            if e.call(0x0049_1040, &args![anim0, slot]).u32() != 0 {
                let other = e.call(0x0049_1040, &args![anim0, slot]).u32();
                if e.call(0x0080_41a0, &args![other]).u32() != 1 {
                    return action;
                }
            }
        }
    }
    let mut allowed = false;
    let mut check_idle = false;
    if e.call(0x0059_bb30, &args![VATS_OBJECT]).u32() != 0 {
        check_idle = true;
    } else if (weapon != 0 && e.call(0x0064_50c0, &args![weapon]).u8() == 0)
        || e.call(0x0044_edb0, &args![VATS_OBJECT]).u32() == 0
    {
        allowed = true;
    } else {
        let procedure = e.call(0x0044_edb0, &args![VATS_OBJECT]).u32();
        if e.call(0x0058_d630, &args![procedure]).u8() == 0 {
            allowed = true;
        } else {
            let distance = e.call(0x004a_7bd0, &args![VATS_OBJECT]).f64();
            if distance <= 0.0 {
                check_idle = true;
            } else {
                allowed = true;
            }
        }
    }
    if check_idle && e.call(0x0049_81f0, &args![anim0]).u8() != 0 {
        let part = e.call(0x0059_bb30, &args![VATS_OBJECT]).u32();
        if e.call(0x0049_8d30, &args![anim0, part]).u8() == 0 {
            allowed = true;
        }
    }
    if !allowed {
        return action;
    }
    e.call(0x00a2_4280, &args![input, 4u32]);
    if weapon != 0 && e.call(0x0052_4b40, &args![weapon]).u8() != 0 {
        return action;
    }
    e.set_global(VATS_ATTACK_DELAY, 3u32);
    match e.mem.u32(action) {
        0 | 1 | 2 | 0x10 | 0x11 | 0x14 | 0x15 => {
            let count = e.mem.u8(action + 8);
            e.mem.set_u8(action + 8, count.wrapping_sub(1));
        }
        _ => {}
    }
    action
}

/// The "queued attack" branch of [`fn_00948310`] (`011e07ac` nonzero): picks
/// the attack group for the queued attack from the current animation
/// (attack, power attack), the process flags (slots `0x1B0`, `0x1A8`,
/// `0x1B4`, `0x1AC`) and the weapon; returns the new group or `group`
/// unchanged.
fn queued_attack_group(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    anim0: u32,
    weapon: u32,
    group: u32,
) -> u32 {
    let mut group = group;
    let queued = e.global::<u32>(ATTACK_QUEUED);
    let current = animation_group(e, anim0, 4);
    if e.call(0x005f_2540, &args![current]).u8() != 0 {
        let current = animation_group(e, anim0, 4);
        if e.call(0x005f_2670, &args![current]).u8() != 0 {
            return group;
        }
        if process_flag(e, this, 0x1b0) {
            let action = e.call(0x008a_7570, &args![this]).u32();
            let held = action == 5 || e.call(0x008a_7570, &args![this]).u32() == 6;
            if held && e.call(0x0070_f490, &args![anim0, 4u32]).i32() <= 2 {
                return group;
            }
            if let Some(found) = weapon_attack_group(e, weapon) {
                group = found;
            } else if process_flag(e, this, 0x1a8) || process_flag(e, this, 0x1b4) {
                group = 0x72;
            } else if process_flag(e, this, 0x1ac) {
                group = 0x66;
            }
            e.set(this, PlayerCharacter::fProjectileReleaseTimer, 0.0);
        } else if e.call(0x0070_f490, &args![anim0, 4u32]).i32() == 3 {
            if queued == 1 {
                group = typed_attack_group(e, anim0, weapon);
            } else if queued == 2 {
                group = 0x5c;
            }
        }
    } else if process_flag(e, this, 0x1b0) {
        let action = e.call(0x008a_7570, &args![this]).u32();
        if action != 4 || e.call(0x0070_f490, &args![anim0, 4u32]).i32() > 2 {
            if let Some(found) = weapon_attack_group(e, weapon) {
                group = found;
            } else if process_flag(e, this, 0x1a8) || process_flag(e, this, 0x1b4) {
                group = 0x72;
            } else {
                group = 0x66;
            }
            e.set(this, PlayerCharacter::fProjectileReleaseTimer, 0.0);
        }
    } else if queued == 1 {
        group = weapon_attack_group(e, weapon).unwrap_or(0x20);
    } else if queued == 2 {
        group = 0x5c;
    }
    group
}

/// A new press of the attack button while the actor is reloading (animation
/// action 0x11): returns false (the whole function returns 0) when the
/// magazine item (process slot `0x14C`) is empty. Otherwise it works out the
/// stance and direction from the movement bits (`008846e0`), queues the
/// equip of the next round (`Actor::QueueEquipObject`) while the clip
/// (`GetFormClipRounds`) is not full and the player carries enough
/// (`InventoryChanges::GetObjectCount`), then picks the reload animation
/// (`TESAnimGroup::AnimGroup`, `Animation::PickBestAnimation`) and plays it
/// (`Animation::PlayGroup`), sets the anim action (`Actor::SetAnimAction`)
/// and calls slot `0x4B0` of the player.
fn reload_attack(e: &mut Engine, this: Ptr<PlayerCharacter>, anim0: u32, weapon: u32) -> bool {
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    let magazine = e.vcall(process, 0x14c, &args![]).u32();
    if magazine != 0 && e.call(0x0072_6070, &args![magazine]).u32() == 0 {
        return false;
    }
    let bits = e.call(0x0088_46e0, &args![this]).u32() as u16;
    let mut stance = 0u32;
    if bits & 0x800 != 0 {
        stance = 2;
    } else if bits & 0x2000 != 0 {
        stance = 3;
    } else if bits & 0x400 != 0 {
        stance = 1;
    }
    let mut direction = 0xffu32;
    if bits & 0x200 != 0 {
        if bits & 1 != 0 {
            direction = 7;
        } else if bits & 2 != 0 {
            direction = 8;
        } else if bits & 4 != 0 {
            direction = 9;
        } else if bits & 8 != 0 {
            direction = 10;
        }
    } else if bits & 1 != 0 {
        direction = 3;
    } else if bits & 2 != 0 {
        direction = 4;
    } else if bits & 4 != 0 {
        direction = 5;
    } else if bits & 8 != 0 {
        direction = 6;
    }
    let mut next = 0i32;
    let mut capacity = 0i32;
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    let magazine = e.vcall(process, 0x14c, &args![]).u32();
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    let equipped = e.vcall(process, 0x148, &args![]).u32();
    let changes = e.call(0x004b_f220, &args![this]).u32();
    if equipped != 0 {
        let modified = e.call(0x004b_da70, &args![equipped, 2u32]).u8();
        capacity = e.call(0x004f_e160, &args![weapon, modified as u32]).i32();
    }
    let mut carried = 0i32;
    if changes != 0 {
        let item = e.call(0x0044_ddc0, &args![magazine]).u32();
        carried = e.call(0x004c_8f30, &args![changes, item]).i32();
    }
    if magazine != 0 {
        next = e.call(0x0072_6070, &args![magazine]).i32().wrapping_add(1);
    }
    if magazine != 0 && next <= carried && next < capacity {
        let item = e.call(0x0044_ddc0, &args![magazine]).u32();
        e.call(
            0x0088_c650,
            &args![this, item, next, 0u32, 1u32, 0u32, 0u32],
        );
    }
    let rapid =
        e.call(0x008b_a3e0, &args![this]).u8() != 0 || e.call(0x008b_a410, &args![this]).u8() != 0;
    let group = e
        .call(0x005f_2370, &args![stance, 0u32, direction, rapid as u32])
        .u16();
    let pick = e.call(0x0049_5740, &args![anim0, group as u32, 0u32]).u16();
    if pick != 0xff {
        let player_animation = e.call(0x0095_0a60, &args![this, 0u32]).u32();
        e.call(
            0x0049_4740,
            &args![player_animation, pick as u32, 1u32, 1u32, 4u32],
        );
        let player_animation = e.call(0x0095_0a60, &args![this, 0u32]).u32();
        let object = e.call(0x0049_1040, &args![player_animation, 4u32]).u32();
        e.call(0x008a_73e0, &args![this, 0xffff_ffffu32, object]);
        e.vcall(this.addr(), 0x4b0, &args![pick as u32, 1u32]);
    }
    true
}

/// A new press of the attack button for a non-melee weapon with an attack
/// animation underway that is not a power attack: queues the next attack
/// (`011e07ac` = 1) unless the weapon has no ammunition (`GetCurrentAmmo` 0
/// and `00474a80` of its ammo list false) or, with ammunition, unless the
/// animation (`00598040`, `0094a070`, `00495e40`) still has time to run.
fn queue_next_attack(e: &mut Engine, anim0: u32, weapon: u32) {
    if weapon == 0 {
        e.set_global(ATTACK_QUEUED, 1u32);
        return;
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    let ammo = e.call(0x0052_5980, &args![weapon, player]).u32();
    let has_ammo = if ammo != 0 {
        true
    } else {
        e.call(0x0047_4a80, &args![weapon + 0xa4]).u8() != 0
    };
    if !has_ammo {
        e.set_global(ATTACK_QUEUED, 1u32);
        return;
    }
    let current = e.call(0x0049_1040, &args![anim0, 4u32]).u32();
    let start = e.call(0x0059_8040, &args![current]).f32();
    let current = e.call(0x0049_1040, &args![anim0, 4u32]).u32();
    let end = fn_0094a070(e, Ptr::new(current));
    let now = e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), 0.0);
        e.call(0x0049_5e40, &args![anim0, slot]);
        e.mem.f32(slot.addr())
    });
    if e.call(0x0052_4b40, &args![weapon]).u8() == 0
        && (now <= start || ((end as f64 - start as f64) < 0.5))
    {
        e.set_global(ATTACK_QUEUED, 1u32);
    }
}

/// The reaction of the attack button for a non-queued attack (the
/// `GetFormClipRounds`-free path): the process flag `0x1A8` gives group
/// 0x72 and state 2, `0x1AC` group 0x66 and state 4, `0x1B4` group 0x72 and
/// state 3, each replaced by the weapon's own group and each telling
/// `00963eb0(state, 3.0, 0)` and setting the release timer to 10 in VATS
/// (mode 4) or 0; none of them gives group 0x20 (or the weapon's).
fn ranged_attack_reaction(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    weapon: u32,
    vats: bool,
    group: &mut u32,
) {
    let states: [(u32, u32, u32); 3] = [(0x1a8, 2, 0x72), (0x1ac, 4, 0x66), (0x1b4, 3, 0x72)];
    for (slot, state, default) in states {
        if process_flag(e, this, slot) {
            *group = weapon_attack_group(e, weapon).unwrap_or(default);
            let speed = e.global::<f32>(0x0101_7718);
            e.call(0x0096_3eb0, &args![this, state, speed, 0u32]);
            let timer = if vats {
                e.global::<f32>(0x0101_7b78)
            } else {
                0.0
            };
            e.set(this, PlayerCharacter::fProjectileReleaseTimer, timer);
            return;
        }
    }
    *group = weapon_attack_group(e, weapon).unwrap_or(0x20);
}

/// The attack button held (state 0) with the weapon out (slot `0x454`,
/// `008a6970`) and the latch clear: advances the projectile release timer
/// while charging a thrown or mine attack (process slots `0x1A8` / `0x1B4`
/// with action 5 or 6 early in its animation), clears the VATS action's
/// counter when the weapon has no rounds left, and for the "main" weapon
/// cases tells `00963eb0` and sets the group; then, unless the player is
/// blocked (slot `0x358`) and the hold timer passed the setting `011cdda8`,
/// queues the next attack (`011e07ac` 1 or 2) or the unarmed power attack
/// (group 0x5C).
#[allow(clippy::too_many_arguments)]
fn held_attack(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    anim0: u32,
    weapon: u32,
    action: u32,
    vats: bool,
    time_step: f32,
    ready: &mut bool,
    group: &mut u32,
) {
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if process_flag(e, this, 0x1a8) || process_flag(e, this, 0x1b4) {
        let state = e.call(0x008a_7570, &args![this]).u32();
        let throwing = state == 5 || e.call(0x008a_7570, &args![this]).u32() == 6;
        if throwing && e.call(0x0070_f490, &args![anim0, 4u32]).i32() <= 2 {
            let timer = e.get(this, PlayerCharacter::fProjectileReleaseTimer);
            e.set(
                this,
                PlayerCharacter::fProjectileReleaseTimer,
                (timer as f64 + time_step as f64) as f32,
            );
        }
    }
    if action != 0 && weapon != 0 && e.call(0x0052_5980, &args![weapon, player]).u32() != 0 {
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        let magazine = e.vcall(process, 0x14c, &args![]).u32();
        if e.call(0x0072_6070, &args![magazine]).u32() == 0 {
            e.mem.set_u8(action + 8, 0);
        }
    }
    let mut main = false;
    if weapon != 0 {
        let ammo = e.call(0x0052_5980, &args![weapon, player]).u32();
        let mut recheck = true;
        if ammo == 0 && e.call(0x0052_4b40, &args![weapon]).u8() != 0 {
            main = true;
            recheck = false;
        }
        if recheck && e.call(0x0052_5980, &args![weapon, player]).u32() != 0 {
            let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
            main = e.vcall(process, 0x14c, &args![]).u32() != 0;
        }
    }
    if main {
        let two = e.global::<f32>(0x0101_62c0);
        e.call(0x0096_3eb0, &args![this, 3u32, two, 0u32]);
        if e.call(0x0052_4b40, &args![weapon]).u8() != 0 {
            let mut ready_for_group = true;
            let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
            let hand = e.vcall(process, 0x438, &args![this]).u8();
            let current = animation_group(e, anim0, 4);
            if e.call(0x005f_25d0, &args![current]).u8() == 0 {
                *ready = true;
            } else {
                let step = e.call(0x0070_f490, &args![anim0, 4u32]).i32();
                if step == 1 {
                    *ready = true;
                } else {
                    ready_for_group = false;
                }
            }
            if hand != 0 && ready_for_group {
                let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
                e.vcall(process, 0x43c, &args![hand as u32]);
                *group = typed_attack_group(e, anim0, weapon);
            }
        }
    } else {
        let speed = e.global::<f32>(0x0101_7718);
        if process_flag(e, this, 0x1a8) {
            e.call(0x0096_3eb0, &args![this, 2u32, speed, 0u32]);
        } else if process_flag(e, this, 0x1ac) {
            e.call(0x0096_3eb0, &args![this, 4u32, speed, 0u32]);
        } else if process_flag(e, this, 0x1b4) {
            e.call(0x0096_3eb0, &args![this, 3u32, speed, 0u32]);
        } else {
            let one_and_a_half = e.global::<f32>(0x0101_6088);
            e.call(0x0096_3eb0, &args![this, 1u32, one_and_a_half, 0u32]);
        }
        let current = animation_group(e, anim0, 4);
        if e.call(0x005f_2670, &args![current]).u8() == 0
            && e.global::<u32>(ATTACK_QUEUED) != 2
            && !process_flag(e, this, 0x1b0)
        {
            let timer = e.global::<f32>(ATTACK_HOLD_TIMER);
            e.set_global(ATTACK_HOLD_TIMER, (timer as f64 + time_step as f64) as f32);
        }
    }
    let _ = vats;
    let blocked = e.vcall(this.addr(), 0x358, &args![]).u8() != 0;
    if !blocked {
        let limit = setting_float(e, 0x011c_dda8);
        let held = e.global::<f32>(ATTACK_HOLD_TIMER);
        if (limit as f64) < held as f64 {
            if e.call(0x005a_2030, &args![this]).u8() != 0 {
                e.set_global(ATTACK_QUEUED, 1u32);
            } else {
                let current = animation_group(e, anim0, 4);
                let mut power = false;
                if e.call(0x005f_2540, &args![current]).u8() != 0 {
                    let object = e.call(0x0049_1040, &args![anim0, 4u32]).u32();
                    let looping = e.call(0x0049_3800, &args![anim0, object]).f64();
                    if looping > 0.0 {
                        e.set_global(ATTACK_QUEUED, 2u32);
                        power = true;
                    }
                }
                if !power {
                    *group = 0x5c;
                }
            }
            e.set_global(ATTACK_HOLD_TIMER, 0.0f32);
        }
    }
}

/// Nothing pressed or held for the attack button (the last branch of the
/// button handling). The draw button (button 7) released (state 2) while the
/// weapon is out, with the draw timer (`011e07e0`) running and below the
/// setting `011cdfcc` and an item equipped (slot `0x148`), while the action
/// is none or the animation step is 3: for the RockIt launcher queues a menu
/// (`Interface::QueueMenuCreate`), otherwise re-equips the weapon (slot
/// `0x3EC`) when its ammunition regeneration rate (`00709430`) is not
/// positive; slot `0x6E0` is told 0. Otherwise a released attack animation
/// ends the attack (slot `0x6E0`) and sets the byte `011e0783` from the
/// animation step.
fn released_attack(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    input: u32,
    anim0: u32,
    weapon: u32,
    ready: &mut bool,
) {
    let draw_timer = e.global::<f32>(DRAW_BUTTON_TIMER);
    if button_state(e, input, 7, 2)
        && process_flag(e, this, 0x454)
        && draw_timer != 0.0
        && {
            let limit = setting_float(e, 0x011c_dfcc);
            (limit as f64) > e.global::<f32>(DRAW_BUTTON_TIMER) as f64
        }
        && {
            let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
            e.vcall(process, 0x148, &args![]).u32() != 0
        }
    {
        let action = e.call(0x008a_7570, &args![this]).u32();
        if action == 0xffff_ffff || e.call(0x0070_f490, &args![anim0, 4u32]).i32() == 3 {
            if e.call(0x0047_4a80, &args![weapon.wrapping_add(0xa4)]).u8() != 0 {
                e.call(0x0070_9470, &args![1u32, 0u32, 0u32, 0u32, 4u32, 0u32]);
            } else {
                let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
                let item = e.vcall(process, 0x148, &args![]).u32();
                let modified = e.call(0x004b_da70, &args![item, 6u32]).u8();
                let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
                let item = e.vcall(process, 0x148, &args![]).u32();
                let object = e.call(0x0044_ddc0, &args![item]).u32();
                let rate = e.call(0x0070_9430, &args![object, modified as u32]).f64();
                if rate <= 0.0 {
                    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
                    let item = e.vcall(process, 0x148, &args![]).u32();
                    let modified = e.call(0x004b_da70, &args![item, 2u32]).u8();
                    e.vcall(
                        this.addr(),
                        0x3ec,
                        &args![weapon, 2u32, modified as u32, 0u32],
                    );
                }
            }
            let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
            e.vcall(process, 0x6e0, &args![0u32]);
            *ready = false;
        }
        return;
    }
    let group = animation_group(e, anim0, 4);
    if e.call(0x005f_2540, &args![group]).u8() != 0
        && (button_state(e, input, 4, 2) || !button_state(e, input, 4, 0))
    {
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        e.vcall(process, 0x6e0, &args![0u32]);
        *ready = false;
        let group = animation_group(e, anim0, 4);
        if e.call(0x005f_25d0, &args![group]).u8() != 0 {
            let step = e.call(0x0070_f490, &args![anim0, 4u32]).i32();
            e.mem.set_u8(ATTACK_RELEASE_BYTE, (step < 1) as u8);
        }
    }
}

/// The power attack group of [`fn_00948310`] (the group was 0x5C): with
/// `004997b0` false and one of the actor values 0x1D and 0x1E positive it
/// becomes the directional group 0x5D to 0x60 from the movement bits
/// (`008846e0`; 0x5D is replaced by the weapon's `0094a0a0` unless that is
/// 0xFF), and with `bound` (no weapon, or the bound weapon `0x2D`) the perk
/// groups 0x61, 0x62, 0x64 and 0x65 when the entry points 0x3D, 0x3E, 0x41
/// and 0x42 are positive. Otherwise, with `bound`, entry point 0x3F positive
/// gives 0x63. Returns the group.
fn power_attack_group(e: &mut Engine, this: Ptr<PlayerCharacter>, weapon: u32, bound: bool) -> u32 {
    let mut group = 0x5cu32;
    let mut directional = false;
    if e.call(0x0049_97b0, &args![this]).u8() == 0 {
        let owner = this.addr().wrapping_add(ACTOR_VALUE_OWNER_BASE);
        if e.vcall(owner, 0xc, &args![0x1du32]).f64() > 0.0
            || e.vcall(owner, 0xc, &args![0x1eu32]).f64() > 0.0
        {
            directional = true;
        }
    }
    if directional {
        let bits = e.call(0x0088_46e0, &args![this]).u32() as u16;
        if bits & 1 != 0 {
            group = 0x5d;
        } else if bits & 2 != 0 {
            group = 0x5e;
        } else if bits & 4 != 0 {
            group = 0x5f;
        } else if bits & 8 != 0 {
            group = 0x60;
        }
        if group == 0x5d && weapon != 0 && e.call(0x0094_a0a0, &args![weapon]).u32() != 0xff {
            group = e.call(0x0094_a0a0, &args![weapon]).u32();
        }
        if bound {
            let forward = entry_point_value(e, 0x3d);
            let backward = entry_point_value(e, 0x3e);
            let left = entry_point_value(e, 0x41);
            let right = entry_point_value(e, 0x42);
            if group == 0x5d && forward > 0.0 {
                group = 0x61;
            } else if group == 0x5e && backward > 0.0 {
                group = 0x62;
            } else if group == 0x5f && left > 0.0 {
                group = 0x64;
            } else if group == 0x60 && right > 0.0 {
                group = 0x65;
            }
        }
    } else if bound {
        let value = entry_point_value(e, 0x3f);
        if value > 0.0 {
            group = 0x63;
        }
    }
    group
}

/// The end of [`fn_00948310`] while the attack button is down and the weapon
/// is out: unless the weapon is a ranged one without ammunition or the
/// animation is not an attack (or is a power attack), an attack is queued or
/// the VATS mode is on, it looks at the animation's loop value
/// (`TESAnimGroup::IsJumpingLoopAnim`): with process slot `0x1A8` it
/// re-synchronises ([`fn_009481d0`]) when the repeat latch is set;
/// otherwise, when the loop value is not positive or
/// `LowProcess::GetGenericLocation` is not 1, and the player is not blocked
/// (slot `0x358`), both animations skip an update (`Animation::SkipUpdate`).
fn sync_attack_animation(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    anim0: u32,
    anim1: u32,
    weapon: u32,
    vats: bool,
) {
    if weapon != 0 && e.call(0x0064_50c0, &args![weapon]).u8() == 0 && !process_flag(e, this, 0x1a8)
    {
        return;
    }
    if weapon != 0 {
        let player = e.global::<u32>(PLAYER_SINGLETON);
        if e.call(0x0052_5980, &args![weapon, player]).u32() == 0
            && e.call(0x0052_4b40, &args![weapon]).u8() != 0
        {
            return;
        }
    }
    let group = animation_group(e, anim0, 4);
    if e.call(0x005f_2540, &args![group]).u8() == 0 {
        return;
    }
    let group = animation_group(e, anim0, 4);
    if e.call(0x005f_2670, &args![group]).u8() != 0 {
        return;
    }
    if e.global::<u32>(ATTACK_QUEUED) != 0 || vats {
        return;
    }
    let object = e.call(0x0049_1040, &args![anim0, 4u32]).u32();
    let looping = e.call(0x0049_3800, &args![anim0, object]).f32();
    if process_flag(e, this, 0x1a8) {
        if e.mem.u8(ATTACK_REPEAT_LATCH) != 0 {
            fn_009481d0(e, this);
        }
        return;
    }
    let skip = if looping <= 0.0 {
        true
    } else {
        e.call(0x0080_41a0, &args![object]).u32() != 1
    };
    if skip && e.vcall(this.addr(), 0x358, &args![]).u8() == 0 {
        e.call(0x008e_eaa0, &args![anim0, 4u32]);
        e.call(0x008e_eaa0, &args![anim1, 4u32]);
    }
}

// Translated from 00948310 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player's attack button handling (called from [`fn_009466d0`]): works
/// out which attack animation group to start (`0xFF` for none) from the
/// attack button (button 4: state 1 pressed, 0 held, 2 released) and the
/// weapon, then starts it with `Actor::StartAttack` (`00893a40`). Returns
/// whether an attack input was handled.
///
/// - In VATS (mode 4 of the `VATS` object) [`vats_attack_step`] runs, and
///   the group is overridden by the action kind (0x10: 0x5D or the weapon's
///   `0094a0a0` group if it is a power attack, else 0x5C; 0x11: 0x38; 0x14:
///   0x3E; 0x15: 0xA9).
/// - A queued attack (`011e07ac`) selects its group in
///   [`queued_attack_group`].
/// - A new press ([`reload_attack`] while reloading, else the next queued
///   attack or the weapon reaction), a held button ([`held_attack`]) or a
///   release ([`released_attack`]) set the group and the flags.
/// - A group of 0x5C is replaced by [`power_attack_group`]; with the bound
///   weapon (none, or `008d85e0` = 0x2D) a positive `fCounterAttackTimer`
///   (`009443a0`) and entry point 0x40 give 0xA8.
/// - Finally [`sync_attack_animation`] keeps the animation in step while the
///   button is down.
///
/// The byte local the original keeps at `EBP-5` is never read and is not
/// kept.
pub fn fn_00948310(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    let mut group = 0xffu32;
    let anim0 = e.call(0x0095_0a60, &args![this, 0u32]).u32();
    let anim1 = e.call(0x0095_0a60, &args![this, 1u32]).u32();
    let main = e.global::<u32>(MAIN_SINGLETON);
    let input = e.call(0x0087_7720, &args![main]).u32();
    let mut handled = false;
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    let weapon = if e.vcall(process, 0x148, &args![]).u32() != 0 {
        let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
        let item = e.vcall(process, 0x148, &args![]).u32();
        e.call(0x0044_ddc0, &args![item]).u32()
    } else {
        0
    };
    let vats = e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() == 4;
    let mut action = 0u32;
    let step = e.call(0x0084_d030, &args![0x011f_6394u32]).f64();
    let multiplier = e.call(0x009c_8cc0, &args![VATS_OBJECT]).f64();
    let time_step = (multiplier * step) as f32;
    if vats {
        action = vats_attack_step(e, anim0, input, weapon);
    } else {
        e.set_global(VATS_ATTACK_DELAY, 0u32);
    }
    if process_flag(e, this, 0x3f0) {
        e.set_global(ATTACK_QUEUED, 0u32);
        e.call(0x00a2_4280, &args![input, 4u32]);
    }
    let mut ready = false;
    'select: {
        if e.global::<u32>(ATTACK_QUEUED) != 0 {
            group = queued_attack_group(e, this, anim0, weapon, group);
            break 'select;
        }
        if button_state(e, input, 4, 1) && e.mem.u8(ATTACK_BUTTON_LATCH) == 0 {
            if !process_flag(e, this, 0x454) {
                let bits = e.call(0x0088_46e0, &args![this]).u32();
                if bits & 0x800 == 0 {
                    e.call(0x008a_6840, &args![this, 1u32]);
                }
            } else if e.call(0x008a_7570, &args![this]).u32() == 0x11 {
                if !reload_attack(e, this, anim0, weapon) {
                    return false;
                }
            } else {
                let current = animation_group(e, anim0, 4);
                let attacking = e.call(0x005f_2540, &args![current]).u8() != 0;
                if attacking && (weapon == 0 || e.call(0x0052_4b40, &args![weapon]).u8() == 0) {
                    if !process_flag(e, this, 0x1b0) {
                        let mut skip = false;
                        if e.call(0x0049_97b0, &args![this]).u8() != 0 {
                            let current = animation_group(e, anim0, 4);
                            skip = e.call(0x005f_2670, &args![current]).u8() != 0;
                        }
                        if !skip {
                            queue_next_attack(e, anim0, weapon);
                        }
                    }
                } else {
                    e.mem.set_u8(ATTACK_RELEASE_BYTE, 0);
                    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
                    e.vcall(process, 0x434, &args![1u32]);
                    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
                    e.vcall(process, 0x43c, &args![1u32]);
                    if weapon != 0 {
                        ready = true;
                    }
                    if weapon != 0 && e.call(0x0052_4b40, &args![weapon]).u8() != 0 {
                        let kind = e.call(0x0051_f5f0, &args![weapon]).u32();
                        if e.call(0x005f_25d0, &args![kind]).u8() == 0 {
                            let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
                            e.vcall(process, 0x3f4, &args![1u32]);
                        }
                    }
                    let mut thrown_kind = false;
                    if weapon != 0 {
                        let kind = e.call(0x0051_f5f0, &args![weapon]).u32();
                        thrown_kind = e.call(0x005f_25d0, &args![kind]).u8() != 0;
                    }
                    if !thrown_kind {
                        ranged_attack_reaction(e, this, weapon, vats, &mut group);
                    }
                }
            }
            e.set_global(ATTACK_HOLD_TIMER, 0.0f32);
            handled = true;
            break 'select;
        }
        if button_state(e, input, 4, 0)
            && process_flag(e, this, 0x454)
            && e.call(0x008a_6970, &args![this]).u8() != 0
            && e.mem.u8(ATTACK_BUTTON_LATCH) == 0
        {
            held_attack(
                e, this, anim0, weapon, action, vats, time_step, &mut ready, &mut group,
            );
            handled = true;
            break 'select;
        }
        released_attack(e, this, input, anim0, weapon, &mut ready);
    }
    // ---- 009498cf: start the chosen attack ----
    if group != 0xff {
        if (weapon == 0 || e.call(0x0064_50c0, &args![weapon]).u8() != 0)
            && e.call(0x008a_7570, &args![this]).u32() == 7
        {
            e.call(0x0089_4cc0, &args![this, 0u32]);
        }
        if e.call(0x0049_97b0, &args![this]).u8() != 0
            && (weapon == 0
                || (e.call(0x0064_50c0, &args![weapon]).u8() != 0
                    && e.call(0x0052_4b40, &args![weapon]).u8() == 0))
            && e.call(0x005a_2030, &args![this]).u8() == 0
        {
            group = 0x5c;
        }
        let mut dialogue = false;
        let bound = weapon == 0 || e.call(0x008d_85e0, &args![weapon]).u32() == 0x2d;
        if bound {
            let timer = fn_009443a0(e, this);
            if timer as f64 > 0.0 && entry_point_value(e, 0x40) > 0.0 {
                group = 0xa8;
            }
        }
        if group == 0x5c {
            group = power_attack_group(e, this, weapon, bound);
            let setting = setting_float(e, 0x011c_e3d8);
            if e.call(0x004d_ff00, &args![setting]).u8() != 0 {
                dialogue = true;
            }
        }
        if vats && action != 0 {
            let kind = e.mem.u32(action);
            if kind == 0x10 {
                group = 0x5d;
                if weapon != 0 && e.call(0x0094_a0a0, &args![weapon]).u32() != 0 {
                    group = e.call(0x0094_a0a0, &args![weapon]).u32();
                }
                if e.call(0x005f_2670, &args![group & 0xffff]).u8() == 0 {
                    group = 0x5c;
                }
            } else if kind == 0x11 && e.call(0x0049_97b0, &args![this]).u8() == 0 {
                group = 0x38;
            } else if kind == 0x14 {
                group = 0x3e;
            } else if kind == 0x15 {
                group = 0xa9;
            }
        }
        let started = e.call(0x0089_3a40, &args![this, group]).u8() != 0;
        if started
            && weapon != 0
            && (e.call(0x0052_4b40, &args![weapon]).u8() != 0 || e.mem.u32(at(this, 0x1a0)) != 0)
            && ready
        {
            let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
            e.vcall(process, 0x6e0, &args![1u32]);
        }
        if started && dialogue {
            let manager = e.global::<u32>(COMBAT_DIALOGUE_MANAGER);
            let player = e.global::<u32>(PLAYER_SINGLETON);
            e.call(
                0x0098_39b0,
                &args![manager, player, 0u32, 2u32, 1u32, 0u32, 0u32],
            );
        }
        e.set_global(ATTACK_QUEUED, 0u32);
        e.mem.set_u8(ATTACK_REPEAT_LATCH, 1);
        e.set_global(ATTACK_COUNTER, 0u32);
    }
    // ---- 00949d93: keep the attack animation in step with the button ----
    if (button_state(e, input, 4, 0) || button_state(e, input, 4, 1))
        && process_flag(e, this, 0x454)
        && e.mem.u8(ATTACK_BUTTON_LATCH) == 0
    {
        sync_attack_animation(e, this, anim0, anim1, weapon, vats);
    } else {
        e.mem.set_u8(ATTACK_REPEAT_LATCH, 0);
        if e.call(0x008a_7570, &args![this]).u32() == 0xffff_ffff {
            e.mem.set_u8(ATTACK_BUTTON_LATCH, 0);
        }
    }
    handled
}
/// Size and offsets of the frame of [`fn_0094a0c0`]: the working copy of the
/// camera position, three scratch points for the smoothing, the unit
/// vector from the anchor to the camera, the two copies for the camera
/// caster, its result structure, and the vectors of the distance update.
const CAMERA_FRAME_SIZE: u32 = 0x120;
const CAMERA_CURRENT: u32 = 0x00;
const CAMERA_SMOOTH_A: u32 = 0x0c;
const CAMERA_SMOOTH_B: u32 = 0x18;
const CAMERA_SMOOTH_C: u32 = 0x24;
const CAMERA_UNIT: u32 = 0x30;
const CAMERA_PICK_DIRECTION: u32 = 0x3c;
const CAMERA_PICK_ORIGIN: u32 = 0x48;
const CAMERA_PICK_RESULT: u32 = 0x54;
const CAMERA_SCALED_A: u32 = 0xd4;
const CAMERA_TARGET: u32 = 0xe0;
const CAMERA_SCALED_B: u32 = 0xec;
const CAMERA_FINAL: u32 = 0xf8;
const CAMERA_TRAVEL: u32 = 0x104;
const CAMERA_TRAVEL_ANCHOR: u32 = 0x110;

/// The smoothed camera position (three floats, `011e0808`).
const CAMERA_SMOOTHED: u32 = 0x011e_0808;
/// The camera distance (float, `011e0768`) that
/// [`fn_0094a0c0`] moves towards its target value.
const CAMERA_DISTANCE: u32 = 0x011e_0768;
/// A byte global (`011e07c3`): the distance snaps to the target next time.
const CAMERA_SNAP: u32 = 0x011e_07c3;
/// A byte global (`011e07c2`) set when the camera caster hit something far
/// closer than the wanted distance.
const CAMERA_BLOCKED: u32 = 0x011e_07c2;
/// The float (`011a3b38`) that fades the player in and out when the camera
/// is close, and a byte (`011a3b32`) that stops the fade-out.
const PLAYER_FADE: u32 = 0x011a_3b38;
const PLAYER_FADE_LATCH: u32 = 0x011a_3b32;

/// Copies the three words at `source` to `destination`.
fn copy_point(e: &mut Engine, destination: u32, source: u32) {
    for i in 0..3 {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(destination + 4 * i, word);
    }
}

// Translated from 0094a0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the third-person camera along the line from `anchor` (the point
/// the camera looks at) to `position` (in and out: the camera position):
/// the distance [`CAMERA_DISTANCE`] is brought towards the distance of the
/// current position, limited by the zoom settings, the camera caster's hit
/// (`CameraCaster::Pick`, which also sets [`CAMERA_BLOCKED`]), the player's
/// view toggling (`bWant3rdPerson` against `b3rdPerson`) and the VATS mode,
/// and the player is faded out when the camera is too close to him
/// (`Actor::UpdateAlpha`). `snap` (or [`CAMERA_SNAP`]) puts the distance
/// there at once. `position` receives the new camera position
/// (`anchor + unit * distance`).
///
/// The compiler's stack cookie check is not translated.
pub fn fn_0094a0c0(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    position: Ptr,
    anchor: Ptr,
    snap: bool,
) {
    e.with_stack(CAMERA_FRAME_SIZE, |e, frame| {
        camera_distance_in_frame(e, this, position.addr(), anchor.addr(), snap, frame.addr());
    });
}

#[allow(clippy::cognitive_complexity)]
fn camera_distance_in_frame(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    position: u32,
    anchor: u32,
    snap: bool,
    frame: u32,
) {
    let current = frame + CAMERA_CURRENT;
    // The mode: wanted minus current third-person flag, as a signed byte.
    let mut mode = (e.mem.u8(at(this, 0x64c)) as i32 - e.mem.u8(at(this, 0x64a)) as i32) as i8;
    let mut blend = 1.0f32;
    if mode != 0 {
        if snap || e.mem.u8(CAMERA_SNAP) != 0 {
            mode = 0;
        } else {
            let base = e.call(SETTING_FLOAT_GETTER, &args![0x011c_cfd8u32]).u32();
            let near = setting_float(e, 0x011c_da94);
            let from_near = e.global::<f32>(ZOOM_DISTANCE) as f64 - near as f64;
            let far = setting_float(e, 0x011c_de14);
            let near = setting_float(e, 0x011c_da94);
            let ratio = from_near / (far as f64 - near as f64);
            let scale = setting_float(e, 0x011c_dd60);
            blend = (scale as f64 * ratio + e.mem.f32(base) as f64) as f32;
        }
    }
    copy_point(e, current, position);
    if snap || e.mem.u8(CAMERA_SNAP) != 0 {
        copy_point(e, CAMERA_SMOOTHED, current);
    } else if e.mem.u8(0x011e_07b9) != 0 {
        let step = e.call(0x0084_d030, &args![0x011f_6394u32]).f32();
        let rate = setting_float(e, 0x011c_d3d8);
        let moved = e
            .call(
                0x0043_9ef0,
                &args![current, frame + CAMERA_SMOOTH_A, CAMERA_SMOOTHED],
            )
            .u32();
        let scaled = e
            .call(0x0045_bb20, &args![moved, frame + CAMERA_SMOOTH_B, rate])
            .u32();
        let scaled = e
            .call(0x0045_bb20, &args![scaled, frame + CAMERA_SMOOTH_C, step])
            .u32();
        e.call(0x0063_c8a0, &args![CAMERA_SMOOTHED, scaled]);
        copy_point(e, current, CAMERA_SMOOTHED);
    } else {
        copy_point(e, CAMERA_SMOOTHED, current);
    }
    let unit = frame + CAMERA_UNIT;
    e.call(0x0043_9ef0, &args![current, unit, anchor]);
    let mut length = e.call(0x0045_7910, &args![unit]).f32();
    e.mem.set_u8(CAMERA_BLOCKED, 0);
    let caster = e.mem.u32(at(this, 0x21c));
    if caster != 0 {
        let pick_direction = frame + CAMERA_PICK_DIRECTION;
        let pick_origin = frame + CAMERA_PICK_ORIGIN;
        let result = frame + CAMERA_PICK_RESULT;
        copy_point(e, pick_direction, anchor);
        copy_point(e, pick_origin, current);
        e.call(0x0062_1c40, &args![result]);
        if e.call(0x0062_1440, &args![caster]).u32() == 0 {
            let controller = e.call(0x0093_06d0, &args![this]).u32();
            let shape = if controller != 0 {
                let controller = e.call(0x0093_06d0, &args![this]).u32();
                e.call(0x0062_1ad0, &args![controller]).u32()
            } else {
                0
            };
            e.call(0x0062_1370, &args![caster, shape]);
        }
        if e.call(
            0x0062_0bc0,
            &args![caster, pick_direction, pick_origin, result],
        )
        .u8()
            != 0
        {
            e.call(0x0045_78c0, &args![pick_origin, pick_direction]);
            let hit = e.call(0x0045_7990, &args![pick_origin]).f64();
            if length as f64 - hit > e.global::<f64>(0x0101_1590) {
                e.mem.set_u8(CAMERA_BLOCKED, 1);
            }
            let hit = e.call(0x0045_7990, &args![pick_origin]).f64();
            let margin = setting_float(e, 0x011e_0934);
            length = (hit - margin as f64 * e.global::<f64>(0x0101_1588)) as f32;
        }
    }
    let rate = setting_float(e, 0x011c_d288);
    let elapsed = e.call(0x0084_d030, &args![0x011f_6394u32]).f64();
    let step = ((elapsed * rate as f64) * blend as f64) as f32;
    let mut ceiling = setting_float(e, 0x011c_d568);
    if e.mem.u8(FREE_CAMERA_FLAG) != 0 {
        ceiling = setting_float(e, 0x011c_de14);
    }
    if mode != 0 {
        let direction = mode as i32 as f64;
        let mut stepped = false;
        if mode == 1 {
            let wanted = direction * step as f64 + e.global::<f32>(CAMERA_DISTANCE) as f64;
            if (length as f64) < wanted {
                e.set_global(CAMERA_DISTANCE, length);
                e.set_global(0x011e_07dc, 0.0f32);
                mode = 0;
                stepped = true;
            }
        }
        if !stepped {
            let moved = mode as i32 as f64 * step as f64 + e.global::<f32>(CAMERA_DISTANCE) as f64;
            e.set_global(CAMERA_DISTANCE, moved as f32);
        }
        let distance = e.global::<f32>(CAMERA_DISTANCE);
        if ceiling < distance {
            e.set_global(CAMERA_DISTANCE, ceiling);
            e.set_global(0x011e_07dc, 0.0f32);
            mode = 0;
        } else if e.global::<f32>(CAMERA_DISTANCE) < 0.0 {
            e.set_global(CAMERA_DISTANCE, 0.0f32);
            e.set_global(0x011e_07dc, 0.0f32);
            mode = 0;
        }
    } else if snap || e.mem.u8(CAMERA_SNAP) != 0 {
        e.set_global(CAMERA_DISTANCE, length);
        e.mem.set_u8(CAMERA_SNAP, 0);
    } else {
        let distance = e.global::<f32>(CAMERA_DISTANCE);
        if distance >= length || (length as f64) < distance as f64 + step as f64 {
            e.set_global(CAMERA_DISTANCE, length);
        } else {
            e.set_global(CAMERA_DISTANCE, (distance as f64 + step as f64) as f32);
        }
        if e.mem.u8(CAMERA_BLOCKED) == 0 {
            let near = setting_float(e, 0x011c_da94);
            if (near as f64) > e.global::<f32>(CAMERA_DISTANCE) as f64 {
                let near = setting_float(e, 0x011c_da94);
                e.set_global(CAMERA_DISTANCE, near);
            }
        }
        if ceiling < e.global::<f32>(CAMERA_DISTANCE) {
            e.set_global(CAMERA_DISTANCE, ceiling);
        }
    }
    let distance = e.global::<f32>(CAMERA_DISTANCE);
    let scaled = e
        .call(0x0045_bb20, &args![unit, frame + CAMERA_SCALED_A, distance])
        .u32();
    let target = frame + CAMERA_TARGET;
    e.call(0x0043_9e90, &args![anchor, target, scaled]);
    let mut obstructed = false;
    if mode != 0 || e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() == 4 {
        if mode < 0 {
            let limit = setting_float(e, 0x011e_0a84);
            if (limit as f64) > e.global::<f32>(CAMERA_DISTANCE) as f64 {
                e.set_global(CAMERA_DISTANCE, 0.0f32);
                e.set_global(0x011e_07dc, 0.0f32);
                mode = 0;
            }
        } else {
            let limit = setting_float(e, 0x011e_0a84);
            if (limit as f64) > e.global::<f32>(CAMERA_DISTANCE) as f64 {
                let limit = setting_float(e, 0x011e_0a84);
                e.set_global(CAMERA_DISTANCE, limit);
            }
        }
    } else if e.call(0x0070_2680, &args![0x40cu32, 0u32]).u8() == 0
        && e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() != 4
    {
        let controller = e.call(0x0093_06d0, &args![this]).u32();
        let height = e.call(0x0088_53a0, &args![this]).f64();
        let above = (height - e.global::<f64>(0x0108_b0a0)) as f32;
        let margin = setting_float(e, 0x011e_09c8);
        if e.call(
            0x0062_14d0,
            &args![caster, controller, target, margin, above],
        )
        .u8()
            != 0
        {
            obstructed = true;
        }
    }
    if obstructed && !e.get(this, PlayerCharacter::bTemp3rdPerson) {
        let fade = e.global::<f32>(PLAYER_FADE);
        if fade as f64 > 0.0 && e.mem.u8(PLAYER_FADE_LATCH) == 0 {
            let elapsed = e.call(0x0084_d030, &args![0x011f_6394u32]).f64();
            let faded = fade as f64 - elapsed * e.global::<f64>(0x0101_db80);
            e.set_global(PLAYER_FADE, faded as f32);
            if e.global::<f32>(PLAYER_FADE) < 0.0 {
                e.set_global(PLAYER_FADE, 0.0f32);
            }
            e.call(0x008c_4640, &args![this]);
        }
    } else {
        let fade = e.global::<f32>(PLAYER_FADE);
        if (fade as f64) < 1.0 {
            let elapsed = e.call(0x0084_d030, &args![0x011f_6394u32]).f64();
            let faded =
                elapsed * e.global::<f64>(0x0101_db80) + e.global::<f32>(PLAYER_FADE) as f64;
            e.set_global(PLAYER_FADE, faded as f32);
            if e.global::<f32>(PLAYER_FADE) as f64 > 1.0
                || mode != 0
                || e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() == 4
            {
                e.set_global(PLAYER_FADE, 1.0f32);
            }
            e.call(0x008c_4640, &args![this]);
        }
        e.mem.set_u8(PLAYER_FADE_LATCH, 0);
    }
    let distance = e.global::<f32>(CAMERA_DISTANCE);
    let scaled = e
        .call(0x0045_bb20, &args![unit, frame + CAMERA_SCALED_B, distance])
        .u32();
    let placed = frame + CAMERA_FINAL;
    e.call(0x0043_9e90, &args![anchor, placed, scaled]);
    if mode != 0 {
        let travel = frame + CAMERA_TRAVEL;
        e.call(0x0043_9ef0, &args![position, travel, placed]);
        let moved = e.call(0x0045_7990, &args![travel]).f64();
        let elapsed = e.call(0x0084_d030, &args![0x011f_6394u32]).f64();
        e.set_global(0x011e_07dc, (moved / elapsed) as f32);
        if mode == -1 {
            let world = e.global::<u32>(0x011e_07d0);
            let point = e.call(0x0045_bb80, &args![world]).u32();
            let anchor_travel = frame + CAMERA_TRAVEL_ANCHOR;
            e.call(0x0043_9ef0, &args![position, anchor_travel, point]);
            let travelled = e.call(0x0045_7990, &args![travel]).f64();
            let to_anchor = e.call(0x0045_7990, &args![anchor_travel]).f64();
            if to_anchor <= travelled {
                e.set_global(0x011e_07dc, 0.0f32);
                e.set_global(CAMERA_DISTANCE, 0.0f32);
            }
        }
    }
    copy_point(e, position, placed);
}
/// The camera node object (`011e0c20`, built by `00a712f0` on first use) the
/// player's camera update positions and orients.
const CAMERA_NODE: u32 = 0x011e_0c20;
/// The "up" vector of `NiCamera::LookAtWorldPoint`.
const CAMERA_UP_VECTOR: u32 = 0x011a_9484;
/// Static-initialisation flags of [`player_character_update_camera`]: bit 0:
/// the camera node was built; bits 1, 2 and 3: the three cached points below
/// were set to zero.
const CAMERA_STATIC_FLAGS: u32 = 0x011e_0d34;
/// The point the terrain / listener code last saw (`011e0c14`), its
/// direction (`011e0c08`) and the point of the last terrain update
/// (`011e0bfc`).
const LISTENER_POINT: u32 = 0x011e_0c14;
const LISTENER_DIRECTION: u32 = 0x011e_0c08;
const TERRAIN_POINT: u32 = 0x011e_0bfc;

/// Layout of the frame of [`player_character_update_camera`]: persistent
/// matrices and points first, then a bump area for the temporaries the
/// compiled code keeps on its stack.
const CAMERA_UPDATE_FRAME_SIZE: u32 = 0xa00;
const CU_ROTATION: u32 = 0x000;
const CU_PITCH_ROTATION: u32 = 0x024;
const CU_HEADING_ROTATION: u32 = 0x048;
const CU_POSITION: u32 = 0x06c;
const CU_EYE: u32 = 0x078;
const CU_CAMERA_POINT: u32 = 0x084;
const CU_ANCHOR: u32 = 0x090;
const CU_VECTOR_A: u32 = 0x09c;
const CU_VECTOR_B: u32 = 0x0a8;
const CU_LOOK_AT: u32 = 0x0b4;
const CU_EULER: u32 = 0x0c0;
const CU_QUATERNION: u32 = 0x0cc;
const CU_SCRATCH_START: u32 = 0x100;

/// The frame of [`player_character_update_camera`] with its bump allocator
/// for the temporaries.
struct CameraFrame {
    base: u32,
    next: u32,
}

impl CameraFrame {
    fn at(&self, offset: u32) -> u32 {
        self.base + offset
    }

    /// A fresh 12-byte point.
    fn point(&mut self) -> u32 {
        let address = self.base + self.next;
        self.next += 12;
        address
    }

    /// A fresh 36-byte matrix.
    fn matrix(&mut self) -> u32 {
        let address = self.base + self.next;
        self.next += 36;
        address
    }
}

/// `NiPoint3` subtraction (`00439ef0`): `out = a - b`; returns `out`.
fn point_sub(e: &mut Engine, a: u32, out: u32, b: u32) -> u32 {
    e.call(0x0043_9ef0, &args![a, out, b]).u32()
}

/// `NiPoint3` addition (`00439e90`): `out = a + b`; returns `out`.
fn point_add(e: &mut Engine, a: u32, out: u32, b: u32) -> u32 {
    e.call(0x0043_9e90, &args![a, out, b]).u32()
}

/// `NiPoint3` scaling (`0045bb20`): `out = a * factor`; returns `out`.
fn point_scale(e: &mut Engine, a: u32, out: u32, factor: f32) -> u32 {
    e.call(0x0045_bb20, &args![a, out, factor]).u32()
}

/// `NiMatrix3 * NiPoint3` (`004b4500`): `out = matrix * point`.
fn matrix_point(e: &mut Engine, matrix: u32, out: u32, point: u32) -> u32 {
    e.call(0x004b_4500, &args![matrix, out, point]).u32()
}

/// `NiPoint3::NiPoint3(x, y, z)` (`00416870`) into `at`; returns `at`.
fn point_new(e: &mut Engine, at: u32, x: f32, y: f32, z: f32) -> u32 {
    e.call(0x0041_6870, &args![at, x, y, z]).u32()
}

/// Copies `count` words from `source` to `destination`.
fn copy_words(e: &mut Engine, destination: u32, source: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(destination + 4 * i, word);
    }
}

/// `matrix = matrix * other` through `NiMatrix3::operator*` (`0043f8d0`)
/// into a temporary, copied back.
fn matrix_multiply_into(e: &mut Engine, frame: &mut CameraFrame, matrix: u32, other: u32) {
    let out = frame.matrix();
    let result = e.call(0x0043_f8d0, &args![matrix, out, other]).u32();
    copy_words(e, matrix, result, 9);
}

/// Rebuilds the rotation from the camera node's matrix: copies the node's
/// matrix (`006a9540` of [`CAMERA_NODE`]) into the pitch matrix and, for each
/// of its three vectors (`00476930`), stores them with the components
/// rotated (`007133b0(index, z, x, y)`) into the rotation matrix.
fn rotation_from_camera_node(e: &mut Engine, frame: &mut CameraFrame) {
    let matrix = e.call(0x006a_9540, &args![CAMERA_NODE]).u32();
    copy_words(e, frame.at(CU_PITCH_ROTATION), matrix, 9);
    extract_axes(e, frame);
}

/// The vector-by-vector copy of `rotation_from_camera_node`.
fn extract_axes(e: &mut Engine, frame: &mut CameraFrame) {
    let euler = frame.at(CU_EULER);
    for index in 0..3u32 {
        e.call(
            0x0047_6930,
            &args![frame.at(CU_PITCH_ROTATION), index, euler],
        );
        let x = e.mem.u32(euler);
        let y = e.mem.u32(euler + 4);
        let z = e.mem.u32(euler + 8);
        e.call(0x0071_33b0, &args![frame.at(CU_ROTATION), index, z, x, y]);
    }
}

/// Moves the camera node to `position` with the rotation `rotation`, then
/// updates it: `440460` (translate), `43fa80` (rotate), `00a59c60` with a zero
/// point (`0043d410`). `node` is the object `558310` returns for the camera,
/// or [`CAMERA_NODE`].
fn place_camera_node(
    e: &mut Engine,
    frame: &mut CameraFrame,
    node: u32,
    position: u32,
    rotation: Option<u32>,
) {
    e.call(0x0044_0460, &args![node, position]);
    if let Some(rotation) = rotation {
        e.call(0x0043_fa80, &args![node, rotation]);
    }
    let zero = frame.point();
    e.call(0x0043_d410, &args![zero, 0.0f32, 0u32, 0u32]);
    e.call(0x00a5_9c60, &args![node, zero]);
}

// Translated from 0094ae40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdateCamera` (Xbox PDB): places the camera for the
/// frame. `snap` makes the third-person distance jump to its target
/// ([`fn_0094a0c0`]) and forces the listener and terrain updates;
/// `skip_terrain` skips the terrain manager update.
///
/// It records the player's heading and pitch (`011e076c`, `011e0764`), sets
/// the camera field of view from `fWorldFOV`, and then builds the camera
/// rotation (`rotation`) and position (`position`) in one of three ways:
///
/// - VATS mode 4 with `bWant3rdPerson` set: the camera node is placed at the
///   smoothed position (`011e0808`), looks at the VATS target point
///   (`009c8ac0`), and the rotation is taken from the node;
/// - first person (`b3rdPerson` clear, no free camera, no change of view
///   wanted): the heading and pitch rotations are multiplied; in dialogue the
///   dialogue rotation of the global at `011e07d0` is applied, the camera
///   follows the head node (`011e07d4`) by the weight of the animation
///   (`496550`), the first person node (`011e07d8`) is placed, and the camera
///   caster is told (`0094c380`);
/// - otherwise the third-person camera: the eye point from the biped, offset
///   by the zoom and shoulder vectors rotated by the camera rotation and
///   passed through [`fn_0094a0c0`] for the distance and collision, the
///   shoulder offset stored in `kCamera3rdPersonShoulderOffset` and the
///   camera node looking at the eye (`NiCamera::LookAtWorldPoint`).
///
/// Then (unless the VATS camera shot is not initialised) the camera node
/// gets the position and rotation, scaled by the animation's shake
/// (`008d1b30`, applied through Euler angles); the audio listener is moved
/// and oriented, its underwater flag set from the submerge level; the
/// listener and terrain reference points are refreshed when the camera
/// moved more than 64 / 512 units or turned (the terrain manager is updated
/// and its morph parameters); `Cached1stPersonCameraPos`,
/// `CachedWorldCameraPos` and the camera rigid body (`ApplyHardKeyframe`)
/// are updated, and finally `UpdateFirstPersonZoom` (`00950290`) runs.
///
/// The compiler's exception-unwinding state writes, the static
/// initialisation of the camera node (its constructor `00a712f0` and
/// `atexit` registration `00fd9b90` are called the first time) and the NiPoint3
/// / NiMatrix3 temporary constructor calls (`006815c0`, which return their
/// argument) are not translated.
pub fn player_character_update_camera(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    snap: bool,
    skip_terrain: bool,
) {
    e.with_stack(CAMERA_UPDATE_FRAME_SIZE, |e, base| {
        let mut frame = CameraFrame {
            base: base.addr(),
            next: CU_SCRATCH_START,
        };
        update_camera_in_frame(e, this, snap, skip_terrain, &mut frame);
    });
}

fn update_camera_in_frame(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    snap: bool,
    skip_terrain: bool,
    frame: &mut CameraFrame,
) {
    let heading = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
    e.set_global(LAST_HEADING, heading);
    let pitch = e.call(0x0093_1d70, &args![this]).f32();
    e.set_global(LAST_PITCH, pitch);
    let camera = e.call(0x0045_c670, &args![]).u32();
    e.set_global(0x011a_3b64, 1.0f32);
    let flags = e.global::<u32>(CAMERA_STATIC_FLAGS);
    if flags & 1 == 0 {
        e.set_global(CAMERA_STATIC_FLAGS, flags | 1);
        e.call(0x00a7_12f0, &args![CAMERA_NODE]);
        e.call(0x00ec_658f, &args![0x00fd_9b90u32]);
    }
    let fov = e.get(this, PlayerCharacter::fWorldFOV);
    let scene = e.call(0x0045_c670, &args![]).u32();
    e.call(0x00c5_2020, &args![scene, fov, 0u32, 0u32, 0u32]);
    let fov = e.get(this, PlayerCharacter::fWorldFOV);
    e.call(0x00b5_4000, &args![fov]);
    if e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() == 4
        && e.get(this, PlayerCharacter::bWant3rdPerson)
    {
        vats_camera(e, frame);
    } else if !e.get(this, PlayerCharacter::b3rdPerson)
        && e.mem.u8(FREE_CAMERA_FLAG) == 0
        && e.get(this, PlayerCharacter::bWant3rdPerson) == e.get(this, PlayerCharacter::b3rdPerson)
    {
        first_person_camera(e, this, frame);
    } else {
        third_person_camera(e, this, snap, frame);
    }
    camera_tail(e, this, camera, snap, skip_terrain, frame);
}

/// The VATS camera (mode 4 with the third person wanted) of
/// [`player_character_update_camera`].
fn vats_camera(e: &mut Engine, frame: &mut CameraFrame) {
    let position = frame.at(CU_POSITION);
    copy_words(e, position, 0x011e_0808, 3);
    place_camera_node(e, frame, CAMERA_NODE, position, None);
    let out = frame.point();
    let target = e.call(0x009c_8ac0, &args![VATS_OBJECT, out]).u32();
    e.call(0x00a7_01b0, &args![CAMERA_NODE, target, CAMERA_UP_VECTOR]);
    rotation_from_camera_node(e, frame);
    let offset = frame.matrix();
    let x = e.global::<f32>(0x011c_a738);
    let y = e.global::<f32>(0x011c_a73c);
    let z = e.global::<f32>(0x011c_a740);
    e.call(0x00a5_9540, &args![offset, x, y, z]);
    let rotation = frame.at(CU_ROTATION);
    matrix_multiply_into(e, frame, rotation, offset);
}

/// The first-person camera of [`player_character_update_camera`].
fn first_person_camera(e: &mut Engine, this: Ptr<PlayerCharacter>, frame: &mut CameraFrame) {
    let rotation = frame.at(CU_ROTATION);
    let pitch_rotation = frame.at(CU_PITCH_ROTATION);
    let heading = e.global::<f32>(LAST_HEADING);
    e.call(0x004a_0c90, &args![rotation, heading]);
    let pitch = e.global::<f32>(LAST_PITCH);
    e.call(0x0052_4ac0, &args![pitch_rotation, pitch]);
    matrix_multiply_into(e, frame, rotation, pitch_rotation);
    if e.call(0x0093_3840, &args![this]).u8() == 0 {
        let tilt = frame.matrix();
        let angle = -e.global::<f32>(0x011e_0770);
        e.call(0x0052_4ac0, &args![tilt, angle]);
        let dialogue = e.global::<u32>(0x011e_07d0);
        let matrix = e.call(0x0046_1130, &args![dialogue]).u32();
        copy_words(e, pitch_rotation, matrix, 9);
        let out = frame.matrix();
        let product = e.call(0x0043_f8d0, &args![pitch_rotation, out, tilt]).u32();
        copy_words(e, rotation, product, 9);
    }
    let position = frame.at(CU_POSITION);
    let target = e.global::<u32>(0x011e_07d0);
    if target != 0 {
        let point = e.call(0x0045_bb80, &args![target]).u32();
        copy_words(e, position, point, 3);
    }
    let process = e.mem.u32(at(this, ACTOR_CURRENT_PROCESS));
    let biped = e.call(0x0095_0b00, &args![this, 0u32]).u32();
    let animation = e.call(0x004a_b230, &args![biped, 0u32]).u32();
    let item = e.vcall(process, 0x1b8, &args![]).u32();
    let weight = e.call(0x0049_6550, &args![item, animation]).f32();
    if weight < 1.0 {
        let head = e.global::<u32>(0x011e_07d4);
        let head_point = e.call(0x0045_bb80, &args![head]).u32();
        let offset = frame.point();
        point_sub(e, head_point, offset, position);
        let setting = setting_float(e, 0x011c_da28);
        let scale = (setting as f64 * weight as f64) as f32;
        e.call(0x0043_9180, &args![offset, scale]);
        e.call(0x0063_c8a0, &args![position, offset]);
        let first_person = e.global::<u32>(0x011e_07d8);
        if first_person != 0 {
            let node = e.call(0x0055_9450, &args![at(this, 0x694)]).u32();
            let matrix = e.call(0x006a_9540, &args![node]).u32();
            let rotated = frame.point();
            let local = e.call(0x004b_3ae0, &args![rotated, offset, matrix]).u32();
            let base = e.call(0x0043_c490, &args![first_person]).u32();
            let placed = frame.point();
            let moved = point_add(e, base, placed, local);
            let _ = moved;
            place_camera_node(e, frame, first_person, moved, None);
        }
    }
    let caster = e.mem.u32(at(this, 0x21c));
    if caster != 0
        && e.call(0x0062_1440, &args![caster]).u32() != 0
        && e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() != 4
    {
        fn_0094c380(e, Ptr::new(caster));
    }
}

/// The third-person camera of [`player_character_update_camera`].
fn third_person_camera(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    snap: bool,
    frame: &mut CameraFrame,
) {
    let rotation = frame.at(CU_ROTATION);
    let pitch_rotation = frame.at(CU_PITCH_ROTATION);
    let heading_rotation = frame.at(CU_HEADING_ROTATION);
    let eye = frame.at(CU_EYE);
    let camera_point = frame.at(CU_CAMERA_POINT);
    let anchor = frame.at(CU_ANCHOR);
    let vector_a = frame.at(CU_VECTOR_A);
    let vector_b = frame.at(CU_VECTOR_B);
    let look_at = frame.at(CU_LOOK_AT);
    let position = frame.at(CU_POSITION);
    let biped = e.call(0x0095_0bb0, &args![this, 0u32]).u32();
    let head = e.call(0x0045_bb80, &args![biped]).u32();
    copy_words(e, eye, head, 3);
    let far = e.global::<f32>(0x0101_3974);
    if e.mem.u8(0x011e_07b9) != 0 {
        let angle = e.global::<f32>(0x011e_08fc);
        let two_pi = e.global::<f64>(0x0101_ff48);
        if angle < 0.0 {
            e.set_global(0x011e_08fc, (angle as f64 + two_pi) as f32);
        } else if angle as f64 > two_pi {
            e.set_global(0x011e_08fc, (angle as f64 - two_pi) as f32);
        }
        let heading = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
        let turned = (heading as f64 + e.global::<f32>(0x011e_08fc) as f64) as f32;
        e.call(0x004a_0c90, &args![rotation, turned]);
        let pitch = e.global::<f32>(0x011e_08f4);
        e.call(0x0052_4ac0, &args![pitch_rotation, pitch]);
        let scale = e.call(0x0056_7400, &args![this]).f64();
        let height = scale * e.global::<f64>(0x0101_7a40) + e.mem.f32(eye + 8) as f64;
        e.mem.set_f32(eye + 8, height as f32);
    } else {
        let fov = e.get(this, PlayerCharacter::f3rdPersonFOV);
        let scene = e.call(0x0045_c670, &args![]).u32();
        e.call(0x00c5_2020, &args![scene, fov, 0u32, 0u32, 0u32]);
        let fov = e.get(this, PlayerCharacter::f3rdPersonFOV);
        e.call(0x00b5_4000, &args![fov]);
        let turned =
            (e.global::<f32>(LAST_HEADING) as f64 + e.global::<f32>(ACCUMULATED_YAW) as f64) as f32;
        e.call(0x004a_0c90, &args![rotation, turned]);
        let tilted =
            (e.global::<f32>(LAST_PITCH) as f64 + e.global::<f32>(ACCUMULATED_PITCH) as f64) as f32;
        e.call(0x0052_4ac0, &args![pitch_rotation, tilted]);
        let eye_height = e.get(this, PlayerCharacter::fEyeHeight);
        let faded = eye_height as f64 * e.global::<f32>(0x011a_3b64) as f64;
        let scale = e.call(0x0056_7400, &args![this]).f64();
        let raised = scale * faded + e.mem.f32(eye + 8) as f64;
        e.mem.set_f32(eye + 8, raised as f32);
        let player_point = e.global::<u32>(0x011e_07d0);
        let first = e.call(0x0045_bb80, &args![player_point]).u32();
        let head_point = e.global::<u32>(0x011e_07d4);
        let second = e.call(0x0045_bb80, &args![head_point]).u32();
        let offset = (e.mem.f32(first + 8) as f64 - e.mem.f32(second + 8) as f64) as f32;
        let raised = e.mem.f32(eye + 8) as f64 + offset as f64;
        e.mem.set_f32(eye + 8, raised as f32);
    }
    // The zoom-in blend factor.
    let mut blend = 1.0f32;
    if setting_float(e, 0x011c_d844) as f64 > 0.0 {
        let limit = setting_float(e, 0x011c_d844);
        let zoom = e.global::<f32>(ZOOM_DISTANCE);
        blend = (1.0 - zoom as f64 / limit as f64) as f32;
        if blend < 0.0 {
            blend = 0.0;
        }
    }
    // Vector A: the shoulder offset scaled by the blend; vector B: the
    // look-at offset.
    let across = setting_float(e, 0x011c_dc44);
    let side = setting_float(e, 0x011c_dc5c);
    let temporary = frame.point();
    let built = point_new(e, temporary, side, 0.0, across);
    point_scale(e, built, vector_a, blend);
    let across = setting_float(e, 0x011c_dc44);
    let zoom = e.global::<f32>(ZOOM_DISTANCE);
    let depth = (-(zoom as f64) + far as f64) as f32;
    let side = setting_float(e, 0x011c_dc5c);
    point_new(e, vector_b, side, depth, across);
    let direction = (e.mem.u8(at(this, 0x64c)) as i32 - e.mem.u8(at(this, 0x64a)) as i32) as i8;
    matrix_multiply_into(e, frame, rotation, pitch_rotation);
    let zoom = e.global::<f32>(ZOOM_DISTANCE);
    let back = frame.point();
    let behind = point_new(e, back, 0.0, -zoom, 0.0);
    let out = frame.point();
    let behind = point_add(e, behind, out, vector_a);
    let out = frame.point();
    let rotated = matrix_point(e, rotation, out, behind);
    let out = frame.point();
    let wanted = point_add(e, eye, out, rotated);
    copy_words(e, camera_point, wanted, 3);
    let out = frame.point();
    let rotated = matrix_point(e, rotation, out, vector_b);
    point_add(e, eye, look_at, rotated);
    e.call(0x0095_edf0, &args![this]);
    let out = frame.point();
    let rotated = matrix_point(e, rotation, out, vector_a);
    e.call(0x0063_c8a0, &args![eye, rotated]);
    let height = e.mem.f32(eye + 8);
    let player_x = e.mem.f32(at(this, 0x30));
    let player_y = e.mem.f32(at(this, 0x34));
    point_new(e, anchor, player_x, player_y, height);
    fn_0094a0c0(e, this, Ptr::new(camera_point), Ptr::new(anchor), snap);
    let out = frame.point();
    let rotated = matrix_point(e, rotation, out, vector_a);
    e.call(0x0045_78c0, &args![eye, rotated]);
    copy_words(e, position, camera_point, 3);
    let out = frame.point();
    let target = point_add(e, eye, out, vector_b);
    copy_words(e, vector_b, target, 3);
    let out = frame.point();
    let offset = matrix_point(e, rotation, out, vector_a);
    let shoulder = member(this, PlayerCharacter::kCamera3rdPersonShoulderOffset);
    copy_words(e, shoulder, offset, 3);
    if blend as f64 > 0.0 {
        e.call(
            0x004a_0c90,
            &args![heading_rotation, e.global::<f32>(LAST_HEADING)],
        );
        let queued = e.global::<f32>(0x011e_07c8);
        if queued as f64 > 0.0 {
            let limit = setting_float(e, 0x011c_de8c);
            let mut ratio = (queued as f64 / limit as f64) as f32;
            if ratio as f64 > 1.0 {
                ratio = 1.0;
            }
            let z = setting_float(e, 0x011c_d31c);
            let y = setting_float(e, 0x011c_dfb4);
            let x = setting_float(e, 0x011c_da34);
            let temporary = frame.point();
            let built = point_new(e, temporary, x, y, z);
            let out = frame.point();
            let scaled = point_scale(e, built, out, ratio);
            copy_words(e, vector_a, scaled, 3);
            let rest = (1.0 - ratio as f64) as f32;
            let z = setting_float(e, 0x011c_d174);
            let y = setting_float(e, 0x011c_df50);
            let x = setting_float(e, 0x011c_d780);
            let temporary = frame.point();
            let built = point_new(e, temporary, x, y, z);
            let out = frame.point();
            let scaled = point_scale(e, built, out, rest);
            e.call(0x0063_c8a0, &args![vector_a, scaled]);
        } else if direction != 0
            || (e.mem.u8(CAMERA_BLOCKED) != 0 && e.mem.u8(FREE_CAMERA_FLAG) == 0)
        {
            let z = setting_float(e, 0x011c_d174);
            let y = setting_float(e, 0x011c_df50);
            let x = setting_float(e, 0x011c_d780);
            let temporary = frame.point();
            let built = point_new(e, temporary, x, y, z);
            copy_words(e, vector_a, built, 3);
        } else {
            let z = setting_float(e, 0x011c_d31c);
            let y = setting_float(e, 0x011c_dfb4);
            let x = setting_float(e, 0x011c_da34);
            let temporary = frame.point();
            let built = point_new(e, temporary, x, y, z);
            copy_words(e, vector_a, built, 3);
        }
        let pitch = e.global::<f32>(LAST_PITCH);
        e.call(0x0052_4ac0, &args![pitch_rotation, pitch]);
        let out = frame.point();
        let moved = matrix_point(e, pitch_rotation, out, vector_a);
        copy_words(e, vector_a, moved, 3);
        let out = frame.point();
        let moved = matrix_point(e, heading_rotation, out, vector_a);
        copy_words(e, vector_a, moved, 3);
        let reduction = setting_float(e, 0x011c_ddfc);
        let rest = (1.0 - reduction as f64) as f32;
        let out = frame.point();
        let scaled = point_scale(e, vector_a, out, rest);
        e.call(0x0063_c8a0, &args![eye, scaled]);
    }
    place_camera_node(e, frame, CAMERA_NODE, position, None);
    if e.mem.u8(0x011e_07b9) != 0 {
        e.call(0x00a7_01b0, &args![CAMERA_NODE, eye, CAMERA_UP_VECTOR]);
    } else {
        e.call(0x00a7_01b0, &args![CAMERA_NODE, look_at, CAMERA_UP_VECTOR]);
    }
    rotation_from_camera_node(e, frame);
}

/// The common end of [`player_character_update_camera`]: places the camera
/// node, updates the audio listener, the listener / terrain reference
/// points and the terrain, the cached camera positions and the camera rigid
/// body.
fn camera_tail(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    camera: u32,
    snap: bool,
    skip_terrain: bool,
    frame: &mut CameraFrame,
) {
    let position = frame.at(CU_POSITION);
    let rotation = frame.at(CU_ROTATION);
    let mut place = true;
    if e.call(0x0044_ddc0, &args![VATS_OBJECT]).u32() == 4 {
        let procedure = e.call(0x0044_edb0, &args![VATS_OBJECT]).u32();
        if procedure == 0 {
            place = false;
        } else {
            let procedure = e.call(0x0044_edb0, &args![VATS_OBJECT]).u32();
            if e.call(0x0058_d630, &args![procedure]).u8() == 0 {
                place = false;
            }
        }
    }
    if place {
        let matrix = frame.matrix();
        let owner = e.call(0x008b_70d0, &args![this]).u32();
        let animation_scale = e.call(0x0045_3700, &args![owner]).f32();
        let shake = e.call(0x008d_1b30, &args![animation_scale, matrix]).f32();
        if shake as f64 > 0.0 {
            let angles = frame.point();
            e.call(0x00a5_92c0, &args![matrix, angles, angles + 4, angles + 8]);
            let x = (e.mem.f32(angles) as f64 * shake as f64) as f32;
            let y = (e.mem.f32(angles + 4) as f64 * shake as f64) as f32;
            let z = (e.mem.f32(angles + 8) as f64 * shake as f64) as f32;
            e.call(0x00a5_9540, &args![matrix, x, y, z]);
            matrix_multiply_into(e, frame, rotation, matrix);
            fn_0094c3a0(e, shake);
        }
        let node = e.call(0x0055_8310, &args![camera]).u32();
        e.call(0x0044_0460, &args![node, position]);
        let node = e.call(0x0055_8310, &args![camera]).u32();
        e.call(0x0043_fa80, &args![node, rotation]);
        let zero = frame.point();
        e.call(0x0043_d410, &args![zero, 0.0f32, 0u32, 0u32]);
        let node = e.call(0x0055_8310, &args![camera]).u32();
        e.call(0x00a5_9c60, &args![node, zero]);
    }
    // ---- the audio listener ----
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let [px, py, pz] = read_words(e, position);
    e.call(0x00ad_78b0, &args![audio, px, py, pz]);
    let space = e.call(0x0066_29f0, &args![camera]).u32();
    let scratch = frame.point();
    let forward = e.call(0x004e_9c10, &args![space, scratch]).u32();
    let forward = read_words(e, forward);
    let space = e.call(0x0066_29f0, &args![camera]).u32();
    let scratch = frame.point();
    let up = e.call(0x0045_bba0, &args![space, scratch]).u32();
    let up = read_words(e, up);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(
        0x00ad_7940,
        &args![audio, up[0], up[1], up[2], forward[0], forward[1], forward[2]],
    );
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    if cell != 0 {
        let level = e.call(0x0088_5560, &args![this, pz, cell]).f64();
        let underwater = level >= e.global::<f64>(0x0108_b0b0);
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        e.call(0x00ad_7990, &args![audio, underwater as u32]);
    }
    for (bit, destination) in [
        (2u32, LISTENER_POINT),
        (4, LISTENER_DIRECTION),
        (8, TERRAIN_POINT),
    ] {
        let flags = e.global::<u32>(CAMERA_STATIC_FLAGS);
        if flags & bit == 0 {
            e.set_global(CAMERA_STATIC_FLAGS, flags | bit);
            copy_words(e, destination, ZERO_VECTOR, 3);
        }
    }
    // ---- how far the camera moved since the listener / terrain points ----
    let (x, y) = (f32::from_bits(px), f32::from_bits(py));
    let flat = frame.point();
    let built = point_new(e, flat, x, y, 0.0);
    let out = frame.point();
    let difference = point_sub(e, built, out, LISTENER_POINT);
    let length = e.call(0x0045_7990, &args![difference]).f32();
    let moved = e.call(0x0040_8860, &args![length]).f32();
    let flat = frame.point();
    let built = point_new(e, flat, x, y, 0.0);
    let out = frame.point();
    let difference = point_sub(e, built, out, TERRAIN_POINT);
    let length = e.call(0x0045_7990, &args![difference]).f32();
    let terrain_moved = e.call(0x0040_8860, &args![length]).f32();
    let facing = frame.point();
    copy_words(e, facing, ZERO_VECTOR, 3);
    let space = e.call(0x0066_29f0, &args![camera]).u32();
    let bounds = e.call(0x0045_bbe0, &args![space]).u32();
    let log_input = e.mem.f32(bounds + 4);
    let logarithm = e.call(0x004b_1460, &args![log_input]).f64();
    let size = e.call(0x0050_8070, &args![space]).f64();
    let ratio = (logarithm / size) as f32;
    let threshold = (1.0 - ratio as f64 / e.global::<f64>(0x0102_40b0)) as f32;
    let scratch = frame.point();
    let direction = e.call(0x0045_bba0, &args![space, scratch]).u32();
    copy_words(e, facing, direction, 3);
    e.call(0x004a_0c10, &args![facing]);
    let alignment = e
        .call(0x004b_6190, &args![LISTENER_DIRECTION, facing])
        .f32();
    if moved as f64 > e.global::<f64>(0x0102_40c0) || (threshold as f64) > alignment as f64 || snap
    {
        let adjusted = e.call(0x005d_c270, &args![threshold]).f32();
        let [dx, dy, dz] = read_words(e, facing);
        e.call(0x0057_d0a0, &args![px, py, pz, dx, dy, dz, adjusted]);
        let flat = frame.point();
        let built = point_new(e, flat, x, y, 0.0);
        copy_words(e, LISTENER_POINT, built, 3);
        copy_words(e, LISTENER_DIRECTION, facing, 3);
    }
    // ---- the terrain manager ----
    if !skip_terrain && e.call(0x0057_5d70, &args![this]).u32() != 0 {
        let world_space = e.call(0x0057_5d70, &args![this]).u32();
        let terrain = e.call(0x0058_6170, &args![world_space]).u32();
        if e.call(0x0097_4d90, &args![terrain]).u8() != 0 {
            let place = e.vcall(this.addr(), 0x1f4, &args![]).u32();
            let world_space = e.call(0x0057_5d70, &args![this]).u32();
            let terrain = e.call(0x0058_6170, &args![world_space]).u32();
            e.call(0x006f_ca90, &args![terrain, place, 0xfu32]);
        } else if terrain_moved as f64 > e.global::<f64>(0x0102_8338) || snap {
            let place = e.vcall(this.addr(), 0x1f4, &args![]).u32();
            let world_space = e.call(0x0057_5d70, &args![this]).u32();
            let terrain = e.call(0x0058_6170, &args![world_space]).u32();
            e.call(0x006f_ca90, &args![terrain, place, 0xfu32]);
            let flat = frame.point();
            let built = point_new(e, flat, x, y, 0.0);
            copy_words(e, TERRAIN_POINT, built, 3);
        }
        let place = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let world_space = e.call(0x0057_5d70, &args![this]).u32();
        let terrain = e.call(0x0058_6170, &args![world_space]).u32();
        e.call(0x006f_cdb0, &args![terrain, place]);
    }
    // ---- cached positions and the camera rigid body ----
    let player_point = e.global::<u32>(0x011e_07d0);
    if player_point != 0 {
        let point = e.call(0x0045_bb80, &args![player_point]).u32();
        let cached = member(this, PlayerCharacter::Cached1stPersonCameraPos);
        copy_words(e, cached, point, 3);
    }
    let quaternion = frame.at(CU_QUATERNION);
    copy_words(e, quaternion, 0x011a_9ea4, 4);
    let space = e.call(0x0066_29f0, &args![camera]).u32();
    let point = e.call(0x0045_bb80, &args![space]).u32();
    let cached = member(this, PlayerCharacter::CachedWorldCameraPos);
    copy_words(e, cached, point, 3);
    let rigid_body = e
        .call(
            0x0055_9450,
            &args![member(this, PlayerCharacter::spCameraRigidBody)],
        )
        .u32();
    e.call(0x00c8_e160, &args![rigid_body, cached, quaternion, 0u32]);
    e.call(0x0095_0290, &args![this]);
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004a9950,
            ni_tmap_base_set_value(Ptr, Ptr<NiTMapItem>, u32, u8)
        ),
        entry!(0x005f73f0, fn_005f73f0(Ptr, u32) -> i32),
        entry!(
            0x006bf8f0,
            bs_simple_array_item_change_remove(Ptr<BSSimpleArray>, u32, bool)
        ),
        entry!(0x00815340, fn_00815340(Ptr)),
        entry!(0x00815360, fn_00815360(Ptr)),
        entry!(
            0x00938180,
            player_character_construct(Ptr<PlayerCharacter>) -> Ptr<PlayerCharacter>
        ),
        entry!(0x00939680, fn_00939680(Ptr)),
        entry!(0x009396a0, fn_009396a0(Ptr<PlayerCharacter>) -> Ptr),
        entry!(0x009396c0, fn_009396c0(Ptr, u32, u32, u32, u32, u32, u32, u32) -> bool),
        entry!(0x009396d0, fn_009396d0(Ptr<PlayerCharacter>) -> Ptr),
        entry!(0x009396f0, fn_009396f0(Ptr<PlayerCharacter>, Ptr)),
        entry!(0x00939710, fn_00939710(Ptr<MagicTargetBase>) -> Ptr),
        entry!(0x00939730, fn_00939730(Ptr<PlayerCharacter>, u32)),
        entry!(0x00939750, fn_00939750(Ptr<PlayerCharacter>) -> u32),
        entry!(0x00939770, fn_00939770(Ptr<PlayerCharacter>, u32)),
        entry!(0x00939790, fn_00939790(Ptr<PlayerCharacter>) -> u32),
        entry!(0x009397b0, fn_009397b0(Ptr<PlayerCharacter>, u8) -> Ptr),
        entry!(0x009397e0, fn_009397e0(Ptr) -> bool),
        entry!(0x00939800, fn_00939800(Ptr<MagicCasterBase>, u32)),
        entry!(0x00939820, fn_00939820(Ptr<MagicCasterBase>) -> u32),
        entry!(0x00939840, fn_00939840(Ptr<MagicCasterBase>, u32)),
        entry!(0x00939860, fn_00939860(Ptr, u32)),
        entry!(0x00939880, fn_00939880(Ptr) -> Ptr),
        entry!(
            0x009398a0,
            player_character_vector_deleting_destructor(
                Ptr<PlayerCharacter>,
                u32,
            ) -> Ptr<PlayerCharacter>
        ),
        entry!(0x009398d0, fn_009398d0(Ptr<PlayerCharacter>)),
        entry!(0x0093a5a0, camera_caster_vector_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0093a5d0, fn_0093a5d0(Ptr<PlayerCharacter>) -> bool),
        entry!(0x0093a5f0, fn_0093a5f0(Ptr<PlayerCharacter>, bool)),
        entry!(0x0093a640, fn_0093a640(Ptr<PlayerCharacter>) -> bool),
        entry!(
            0x0093a660,
            player_character_remove_actor_from_playercombat_list(Ptr<PlayerCharacter>, Ptr)
        ),
        entry!(
            0x0093a690,
            player_character_add_actor_to_player_combat_list(Ptr<PlayerCharacter>, Ptr)
        ),
        entry!(0x0093a6f0, fn_0093a6f0(Ptr<PlayerCharacter>, bool)),
        entry!(0x0093a740, fn_0093a740(Ptr<PlayerCharacter>) -> bool),
        entry!(0x0093a7a0, fn_0093a7a0(Ptr<PlayerCharacter>, Ptr)),
        entry!(
            0x0093a7c0,
            player_character_set_actor_base_value(Ptr<PlayerCharacter>, u32, i32)
        ),
        entry!(0x0093a850, fn_0093a850(Ptr<PlayerCharacter>, u32, f32)),
        entry!(0x0093a8f0, fn_0093a8f0(Ptr<PlayerCharacter>, u32, i32)),
        entry!(0x0093aa10, fn_0093aa10(Ptr<PlayerCharacter>, u32, f32)),
        entry!(0x0093ab30, fn_0093ab30(Ptr<PlayerCharacter>, u32)),
        entry!(0x0093ab80, fn_0093ab80(Ptr<PlayerCharacter>, f32)),
        entry!(0x0093abe0, fn_0093abe0(Ptr<PlayerCharacter>, Ptr, Ptr)),
        entry!(0x0093ac20, fn_0093ac20(Ptr, u32) -> i32),
        entry!(0x0093acb0, fn_0093acb0(Ptr, u32) -> f32),
        entry!(0x0093ad30, fn_0093ad30(Ptr, u32) -> i32),
        entry!(
            0x0093ad60,
            player_character_get_permanent_actor_float_value(Ptr, u32) -> f32
        ),
        entry!(0x0093ae40, fn_0093ae40(Ptr<PlayerCharacter>, u32, f32)),
        entry!(0x0093ae80, fn_0093ae80(Ptr<PlayerCharacter>, u32, i32, Ptr)),
        entry!(0x0093afb0, fn_0093afb0(Ptr<PlayerCharacter>, u32, f32, Ptr)),
        entry!(0x0093b0f0, fn_0093b0f0(Ptr<PlayerCharacter>, u32, i32, Ptr)),
        entry!(0x0093b240, fn_0093b240(Ptr<PlayerCharacter>, u32, f32, Ptr)),
        entry!(0x0093b3a0, fn_0093b3a0(Ptr<PlayerCharacter>, u32, i32, Ptr)),
        entry!(0x0093b7a0, fn_0093b7a0(Ptr<PlayerCharacter>, u32, f32, Ptr)),
        entry!(
            0x0093bba0,
            player_character_get_water_cell(Ptr<PlayerCharacter>, f32) -> u32
        ),
        entry!(
            0x0093be30,
            player_character_request_position_player(
                Ptr<PlayerCharacter>,
                Ptr<PositionPlayerRequest>,
            )
        ),
        entry!(
            0x0093bea0,
            player_character_handle_position_player_request(Ptr<PlayerCharacter>) -> bool
        ),
        entry!(0x0093c1e0, fn_0093c1e0(Ptr<PlayerCharacter>) -> bool),
        entry!(
            0x0093c200,
            player_character_position_player(
                Ptr<PlayerCharacter>,
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                Ptr,
                bool,
            )
        ),
        entry!(0x0093ccd0, fn_0093ccd0() -> u32),
        entry!(
            0x0093cce0,
            player_character_position_player_exterior(
                Ptr<PlayerCharacter>,
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                Ptr,
                bool,
            )
        ),
        entry!(
            0x0093cdf0,
            player_character_fast_travel(Ptr<PlayerCharacter>, Ptr)
        ),
        entry!(0x0093d4f0, fn_0093d4f0() -> bool),
        entry!(
            0x0093d500,
            player_character_clear_cells_if_target_not_loaded(Ptr<PlayerCharacter>, Ptr, Ptr)
        ),
        entry!(0x0093d660, fn_0093d660(Ptr<PlayerCharacter>) -> bool),
        entry!(0x0093db40, fn_0093db40(Ptr<PlayerCharacter>) -> bool),
        entry!(
            0x0093db60,
            player_character_center_on_cell(Ptr<PlayerCharacter>, u32, Ptr)
        ),
        entry!(0x0093dd20, fn_0093dd20(Ptr<PlayerCharacter>)),
        entry!(0x0093dd80, fn_0093dd80(Ptr<PlayerCharacter>)),
        entry!(
            0x0093dee0,
            player_character_is_target_perceived_and_hostile(Ptr<PlayerCharacter>, u32) -> bool
        ),
        entry!(0x0093df50, fn_0093df50(Ptr<PlayerCharacter>)),
        entry!(0x0093e4f0, fn_0093e4f0(Ptr) -> u32),
        entry!(0x0093e510, fn_0093e510(Ptr)),
        entry!(
            0x0093e530,
            player_character_set_slow_mo_camera(Ptr<PlayerCharacter>, u32, f32, bool, u32)
        ),
        entry!(0x0093e750, fn_0093e750()),
        entry!(0x0093e770, fn_0093e770(Ptr<PlayerCharacter>, i32, bool)),
        entry!(0x0093e840, fn_0093e840(Ptr)),
        entry!(
            0x0093e860,
            player_character_update(Ptr<PlayerCharacter>, f32)
        ),
        entry!(0x00944270, fn_00944270() -> u8),
        entry!(0x009442a0, fn_009442a0(Ptr) -> f32),
        entry!(0x009442c0, fn_009442c0(Ptr) -> bool),
        entry!(0x009442e0, fn_009442e0(Ptr<PlayerCharacter>, f32)),
        entry!(0x00944300, fn_00944300(Ptr<PlayerCharacter>) -> f32),
        entry!(0x00944320, fn_00944320(Ptr<PlayerCharacter>) -> u32),
        entry!(0x00944340, fn_00944340(Ptr<PlayerCharacter>) -> bool),
        entry!(0x00944360, fn_00944360(Ptr<PlayerCharacter>) -> bool),
        entry!(0x00944380, fn_00944380(Ptr<PlayerCharacter>, f32)),
        entry!(0x009443a0, fn_009443a0(Ptr<PlayerCharacter>) -> f32),
        entry!(0x009443c0, fn_009443c0(Ptr) -> bool),
        entry!(0x009443f0, fn_009443f0() -> f32),
        entry!(0x00944400, fn_00944400(Ptr) -> bool),
        entry!(0x00944430, fn_00944430(Ptr) -> bool),
        entry!(0x00944460, fn_00944460(i32, i32) -> i32),
        entry!(0x00944480, fn_00944480(Ptr, i32, i32) -> i32),
        entry!(0x009444a0, fn_009444a0() -> u8),
        entry!(
            0x009444d0,
            player_character_update_player_combat(Ptr<PlayerCharacter>, f32)
        ),
        entry!(
            0x00946020,
            timed_curve_start(Ptr<TimedCurve>, f32, f32, f32) -> bool
        ),
        entry!(
            0x00946140,
            timed_curve_advance(Ptr<TimedCurve>, f32) -> bool
        ),
        entry!(0x00946190, timed_curve_value(Ptr<TimedCurve>) -> f32),
        entry!(0x00946280, timed_curve_is_over(Ptr<TimedCurve>) -> bool),
        entry!(
            0x009462c0,
            player_character_ammo_swap_helper(Ptr<PlayerCharacter>, bool, bool)
        ),
        entry!(
            0x00947b10,
            player_character_check_border_region(Ptr<PlayerCharacter>)
        ),
        entry!(
            0x00947c90,
            player_character_set_last_known_good_position(Ptr<PlayerCharacter>)
        ),
        entry!(0x00947d10, fn_00947d10(Ptr<PlayerCharacter>)),
        entry!(0x00947d80, fn_00947d80(Ptr<PlayerCharacter>)),
        entry!(0x00948000, fn_00948000(Ptr, u32) -> Ptr),
        entry!(0x00948030, fn_00948030(Ptr<PlayerCharacter>) -> u32),
        entry!(
            0x00948050,
            player_character_clear_map_marker_struct_list(Ptr<PlayerCharacter>)
        ),
        entry!(
            0x009480c0,
            player_character_reset_map_marker_struct_list(Ptr<PlayerCharacter>)
        ),
        entry!(0x009481d0, fn_009481d0(Ptr<PlayerCharacter>)),
        entry!(0x0094a070, fn_0094a070(Ptr) -> f32),
        entry!(0x0094a0a0, fn_0094a0a0(Ptr) -> u8),
        entry!(0x0094c380, fn_0094c380(Ptr)),
        entry!(0x0094c3a0, fn_0094c3a0(f32)),
        entry!(
            0x0094c3d0,
            fn_0094c3d0(Ptr<PlayerCharacter>, u32, u32) -> f32
        ),
        entry!(0x0094c460, fn_0094c460(Ptr, u32) -> f32),
        entry!(
            0x009445b0,
            fn_009445b0(Ptr<PlayerCharacter>, f32, bool, Ptr, u32) -> u8
        ),
        entry!(0x009466d0, fn_009466d0(Ptr<PlayerCharacter>, f32, bool)),
        entry!(
            0x0094a8c0,
            player_character_update_ufo_camera(Ptr<PlayerCharacter>)
        ),
        entry!(0x00948310, fn_00948310(Ptr<PlayerCharacter>) -> bool),
        entry!(
            0x0094a0c0,
            fn_0094a0c0(Ptr<PlayerCharacter>, Ptr, Ptr, bool)
        ),
        entry!(
            0x0094ae40,
            player_character_update_camera(Ptr<PlayerCharacter>, bool, bool)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    fn int(value: u32) -> Ret {
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

    fn f(words: &[u32], i: usize) -> f32 {
        f32::from_bits(words[i])
    }

    /// An engine that records every call.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e
    }

    fn player(e: &mut Engine) -> Ptr<PlayerCharacter> {
        e.new_object()
    }

    /// The argument lists of the calls made to `addr`.
    fn called(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn call_addresses(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    /// Writes a vtable at `base` whose slots (byte offsets) point at the given
    /// targets, and makes `object` use it.
    fn put_vtable(e: &mut Engine, object: u32, base: u32, slots: &[(u32, u32)]) {
        vtable_only(e, base, slots);
        e.mem.set_u32(object, base);
    }

    /// Writes a vtable at `base` whose slots (byte offsets) point at the given
    /// targets.
    fn vtable_only(e: &mut Engine, base: u32, slots: &[(u32, u32)]) {
        e.map(base, 0x800);
        for (slot, target) in slots {
            e.mem.set_u32(base + slot, *target);
        }
    }

    /// A zeroed 0x40-byte object whose vtable has the given slots.
    fn object(e: &mut Engine, base: u32, slots: &[(u32, u32)]) -> Ptr {
        let object = Ptr::new(e.mem.alloc(0x40));
        put_vtable(e, object.addr(), base, slots);
        object
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// Doubles the destructor's and constructor's callees that return their
    /// first argument (constructors) or nothing.
    fn stub_returning_this(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, a| int(a[0]));
        }
    }

    #[test]
    fn ni_tmap_base_set_value_stores_the_key_and_value_in_the_item() {
        let mut e = engine();
        let item = e.new_object::<NiTMapItem>();
        e.call(0x004a_9950, &args![0u32, item, 0x1234u32, 0x56u8]);
        assert_eq!(e.get(item, NiTMapItem::m_key), 0x1234);
        assert_eq!(e.get(item, NiTMapItem::m_val), 0x56);
        assert_eq!(e.get(item, NiTMapItem::m_pkNext), Ptr::NULL);
    }

    fn actor_value_owner(e: &mut Engine, value: f32) -> Ptr {
        let target = 0x00b0_0020;
        e.register_double(target, move |_, _| float(value as f64));
        object(e, 0x0200_0000, &[(0x20, target)])
    }

    #[test]
    fn fn_005f73f0_converts_the_virtual_result_and_truncates_it() {
        for (value, expected) in [(6.75f32, 6), (-6.75, -6)] {
            let mut e = engine();
            let owner = actor_value_owner(&mut e, value);
            e.register(0x0040_6ce0, |_, a| float(f32::from_bits(a[0]) as f64));
            e.register(FTOL, |_, a| {
                let bits = a[0] as u64 | (a[1] as u64) << 32;
                int(f64::from_bits(bits) as i32 as u32)
            });
            let result = e.call(0x005f_73f0, &args![owner, 0x2au32]).i32();
            assert_eq!(result, expected);
            assert_eq!(called(&e, 0x00b0_0020), vec![vec![owner.addr(), 0x2a]]);
            assert_eq!(called(&e, 0x0040_6ce0), vec![vec![value.to_bits()]]);
        }
    }

    /// A `BSSimpleArray` of `size` pointers at a fresh buffer, with a vtable
    /// whose slot 4 hands out `new_buffer`.
    fn array(e: &mut Engine, size: u32, new_buffer: u32) -> (Ptr<BSSimpleArray>, u32) {
        let array = e.new_object::<BSSimpleArray>();
        let buffer = e.mem.alloc(64);
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, size);
        e.set(array, BSSimpleArray::iReservedSize, 16);
        e.register_double(0x00b0_0004, move |_, _| int(new_buffer));
        put_vtable(e, array.addr(), 0x0200_0000, &[(4, 0x00b0_0004)]);
        (array, buffer)
    }

    #[test]
    fn bs_simple_array_remove_without_shrinking_shifts_the_tail_down() {
        let mut e = engine();
        let (array, buffer) = array(&mut e, 5, 0);
        stub(&mut e, &[0x0072_ba80, 0x0042_fb60]);
        e.call(0x006b_f8f0, &args![array, 1u32, false]);
        assert_eq!(
            called(&e, 0x0072_ba80),
            vec![vec![array.addr(), buffer + 4, 1]]
        );
        assert_eq!(
            called(&e, 0x0042_fb60),
            vec![vec![array.addr(), buffer + 4, buffer + 8, 3]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 4);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), buffer);
    }

    #[test]
    fn bs_simple_array_remove_with_shrinking_moves_to_the_new_buffer() {
        let mut e = engine();
        let (array, buffer) = array(&mut e, 5, 0x0300_0000);
        stub(&mut e, &[0x0072_ba80, 0x0042_fb60, 0x006a_8500]);
        e.register(0x006f_3170, |_, _| int(1));
        e.register(0x0086_9600, |_, _| int(8));
        e.call(0x006b_f8f0, &args![array, 2u32, true]);
        assert_eq!(called(&e, 0x00b0_0004), vec![vec![array.addr(), 8]]);
        assert_eq!(
            called(&e, 0x0042_fb60),
            vec![
                vec![array.addr(), 0x0300_0000, buffer, 2],
                // The count of the second copy is the old size minus one, as
                // the code has it.
                vec![array.addr(), 0x0300_0008, buffer + 12, 4],
            ]
        );
        assert_eq!(
            called(&e, 0x0072_ba80),
            vec![vec![array.addr(), buffer + 8, 1]]
        );
        assert_eq!(called(&e, 0x006a_8500), vec![vec![array.addr()]]);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0x0300_0000);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 8);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 4);
    }

    #[test]
    fn bs_simple_array_remove_without_the_shrink_test_passing_does_not_shrink() {
        let mut e = engine();
        let (array, buffer) = array(&mut e, 3, 0x0300_0000);
        stub(&mut e, &[0x0072_ba80, 0x0042_fb60]);
        e.register(0x006f_3170, |_, _| int(0));
        e.call(0x006b_f8f0, &args![array, 0u32, true]);
        assert!(called(&e, 0x0086_9600).is_empty());
        assert_eq!(
            called(&e, 0x0042_fb60),
            vec![vec![array.addr(), buffer, buffer + 4, 2]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 2);
    }

    #[test]
    fn fn_00815360_calls_the_virtual_at_0x20() {
        let mut e = engine();
        let target = 0x00b0_0020;
        stub(&mut e, &[target]);
        let this = object(&mut e, 0x0200_0000, &[(0x20, target)]);
        e.call(0x0081_5360, &args![this]);
        assert_eq!(called(&e, target), vec![vec![this.addr()]]);
    }

    #[test]
    fn fn_00815340_calls_fn_00815360_which_calls_the_virtual_at_0x20() {
        let mut e = engine();
        let target = 0x00b0_0020;
        stub(&mut e, &[target]);
        let this = object(&mut e, 0x0200_0000, &[(0x20, target)]);
        e.call(0x0081_5340, &args![this]);
        assert_eq!(called(&e, target), vec![vec![this.addr()]]);
    }

    #[test]
    fn fn_00939680_calls_the_virtual_at_0x20() {
        let mut e = engine();
        let target = 0x00b0_0020;
        stub(&mut e, &[target]);
        let this = object(&mut e, 0x0200_0000, &[(0x20, target)]);
        e.call(0x0093_9680, &args![this]);
        assert_eq!(called(&e, target), vec![vec![this.addr()]]);
    }

    #[test]
    fn fn_009396a0_returns_the_players_target_actor() {
        let mut e = engine();
        let this = player(&mut e);
        e.set(this, PlayerCharacter::pPlayersTargetActor, Ptr::new(0x1234));
        assert_eq!(e.call(0x0093_96a0, &args![this]).u32(), 0x1234);
    }

    #[test]
    fn fn_009396c0_returns_false_whatever_the_seven_words() {
        let mut e = engine();
        let ret = e.call(
            0x0093_96c0,
            &args![0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32],
        );
        assert!(!ret.bool());
    }

    #[test]
    fn fn_009396d0_returns_the_combat_group() {
        let mut e = engine();
        let this = player(&mut e);
        e.set(this, PlayerCharacter::pCombatGroup, Ptr::new(0x4321));
        assert_eq!(e.call(0x0093_96d0, &args![this]).u32(), 0x4321);
    }

    #[test]
    fn fn_009396f0_stores_the_combat_group() {
        let mut e = engine();
        let this = player(&mut e);
        e.call(0x0093_96f0, &args![this, 0x4321u32]);
        assert_eq!(e.mem.u32(this.addr() + 0xd64), 0x4321);
    }

    #[test]
    fn fn_00939710_returns_the_word_at_0x17c_of_the_magic_target_base() {
        let mut e = engine();
        let this = player(&mut e);
        e.mem.set_u32(this.addr() + 0x94 + 0x17c, 0x7777);
        assert_eq!(
            e.get(this, PlayerCharacter::pActiveEffectList).addr(),
            0x7777
        );
        assert_eq!(
            e.call(0x0093_9710, &args![this.addr() + 0x94]).u32(),
            0x7777
        );
    }

    #[test]
    fn fn_00939730_stores_the_advance_number() {
        let mut e = engine();
        let this = player(&mut e);
        e.call(0x0093_9730, &args![this, 3u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x734), 3);
    }

    #[test]
    fn fn_00939750_returns_the_advance_number() {
        let mut e = engine();
        let this = player(&mut e);
        e.mem.set_u32(this.addr() + 0x734, 9);
        assert_eq!(e.call(0x0093_9750, &args![this]).u32(), 9);
    }

    #[test]
    fn fn_00939770_stores_the_skill_advance() {
        let mut e = engine();
        let this = player(&mut e);
        e.call(0x0093_9770, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x738), 5);
    }

    #[test]
    fn fn_00939790_returns_the_skill_advance() {
        let mut e = engine();
        let this = player(&mut e);
        e.mem.set_u32(this.addr() + 0x738, 6);
        assert_eq!(e.call(0x0093_9790, &args![this]).u32(), 6);
    }

    #[test]
    fn fn_009397b0_picks_the_perk_list_by_the_flag() {
        let mut e = engine();
        let this = player(&mut e);
        assert_eq!(
            e.call(0x0093_97b0, &args![this, 0u8]).u32(),
            this.addr() + 0x87c
        );
        assert_eq!(
            e.call(0x0093_97b0, &args![this, 1u8]).u32(),
            this.addr() + 0xad4
        );
        assert_eq!(
            e.call(0x0093_97b0, &args![this, 7u8]).u32(),
            this.addr() + 0xad4
        );
    }

    #[test]
    fn fn_009397e0_calls_005a03f0_with_one() {
        let mut e = engine();
        stub(&mut e, &[0x005a_03f0]);
        e.call(0x0093_97e0, &args![0x5000u32]);
        assert_eq!(called(&e, 0x005a_03f0), vec![vec![0x5000, 1]]);
    }

    #[test]
    fn fn_009397e0_answers_the_005a03f0_al() {
        let mut e = engine();
        e.register(0x005a_03f0, |_, _| 0x0000_0101u32.into_ret());
        assert!(e.call(0x0093_97e0, &args![0x5000u32]).bool());
        e.register(0x005a_03f0, |_, _| 0x0000_0100u32.into_ret());
        assert!(!e.call(0x0093_97e0, &args![0x5000u32]).bool());
    }

    #[test]
    fn fn_00939800_stores_the_current_spell_through_the_magic_caster_base() {
        let mut e = engine();
        let this = player(&mut e);
        e.call(0x0093_9800, &args![this.addr() + 0x88, 0x99u32]);
        assert_eq!(e.get(this, PlayerCharacter::pCurrentSpell).addr(), 0x99);
    }

    #[test]
    fn fn_00939820_returns_the_desired_target() {
        let mut e = engine();
        let this = player(&mut e);
        e.set(this, PlayerCharacter::pDesiredTarget, Ptr::new(0x55));
        assert_eq!(e.call(0x0093_9820, &args![this.addr() + 0x88]).u32(), 0x55);
    }

    #[test]
    fn fn_00939840_stores_the_desired_target() {
        let mut e = engine();
        let this = player(&mut e);
        e.call(0x0093_9840, &args![this.addr() + 0x88, 0x66u32]);
        assert_eq!(e.get(this, PlayerCharacter::pDesiredTarget).addr(), 0x66);
    }

    #[test]
    fn fn_00939860_stores_the_word_at_0x604() {
        let mut e = engine();
        let block: Ptr = Ptr::new(e.mem.alloc(0x610));
        e.call(0x0093_9860, &args![block, 0x3e8u32]);
        assert_eq!(e.mem.u32(block.addr() + 0x604), 0x3e8);
    }

    #[test]
    fn fn_00939880_clears_the_float_and_returns_the_object() {
        let mut e = engine();
        let block: Ptr = Ptr::new(e.mem.alloc(0x1a0));
        e.mem.set_f32(block.addr() + 0x198, 2.5);
        assert_eq!(e.call(0x0093_9880, &args![block]).u32(), block.addr());
        assert_eq!(e.mem.f32(block.addr() + 0x198), 0.0);
    }

    #[test]
    fn fn_0093a5d0_returns_the_to_pos_flag() {
        let mut e = engine();
        let this = player(&mut e);
        assert!(!e.call(0x0093_a5d0, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x798, 1);
        assert!(e.call(0x0093_a5d0, &args![this]).bool());
    }

    #[test]
    fn fn_0093a640_returns_the_from_pos_flag() {
        let mut e = engine();
        let this = player(&mut e);
        assert!(!e.call(0x0093_a640, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x799, 1);
        assert!(e.call(0x0093_a640, &args![this]).bool());
    }

    #[test]
    fn fn_0093a740_is_true_when_any_ai_control_flag_is_set() {
        for (offset, expected) in [
            (0x798, true),
            (0x799, true),
            (0x79a, true),
            (0x79b, true),
            (0x79c, false),
        ] {
            let mut e = engine();
            let this = player(&mut e);
            e.mem.set_u8(this.addr() + offset, 1);
            assert_eq!(
                e.call(0x0093_a740, &args![this]).bool(),
                expected,
                "{offset:x}"
            );
        }
        let mut e = engine();
        let this = player(&mut e);
        assert!(!e.call(0x0093_a740, &args![this]).bool());
    }

    /// A player whose virtual at `+0x258` is recorded.
    fn player_with_process_virtual(e: &mut Engine) -> Ptr<PlayerCharacter> {
        let this = player(e);
        stub(e, &[0x00b0_0258]);
        put_vtable(e, this.addr(), 0x0200_0000, &[(0x258, 0x00b0_0258)]);
        this
    }

    #[test]
    fn fn_0093a5f0_stores_the_flag_and_passes_the_inverse_of_any_to_the_virtual() {
        let mut e = engine();
        let this = player_with_process_virtual(&mut e);
        e.call(0x0093_a5f0, &args![this, true]);
        assert_eq!(e.mem.u8(this.addr() + 0x798), 1);
        assert_eq!(called(&e, 0x00b0_0258), vec![vec![this.addr(), 0]]);
        e.call(0x0093_a5f0, &args![this, false]);
        assert_eq!(e.mem.u8(this.addr() + 0x798), 0);
        assert_eq!(called(&e, 0x00b0_0258)[1], vec![this.addr(), 1]);
        // Another flag keeps the result at 0.
        e.mem.set_u8(this.addr() + 0x79b, 1);
        e.call(0x0093_a5f0, &args![this, false]);
        assert_eq!(called(&e, 0x00b0_0258)[2], vec![this.addr(), 0]);
    }

    #[test]
    fn fn_0093a6f0_stores_the_flag_and_passes_the_inverse_of_any_to_the_virtual() {
        let mut e = engine();
        let this = player_with_process_virtual(&mut e);
        e.call(0x0093_a6f0, &args![this, true]);
        assert_eq!(e.mem.u8(this.addr() + 0x799), 1);
        assert_eq!(called(&e, 0x00b0_0258), vec![vec![this.addr(), 0]]);
        e.call(0x0093_a6f0, &args![this, false]);
        assert_eq!(e.mem.u8(this.addr() + 0x799), 0);
        assert_eq!(called(&e, 0x00b0_0258)[1], vec![this.addr(), 1]);
    }

    #[test]
    fn camera_caster_destructor_runs_the_body_and_frees_only_with_the_flag() {
        let mut e = engine();
        stub(&mut e, &[0x0062_08d0]);
        let caster: Ptr = Ptr::new(e.mem.alloc(8));
        assert_eq!(
            e.call(0x0093_a5a0, &args![caster, 0u32]).u32(),
            caster.addr()
        );
        assert_eq!(called(&e, 0x0062_08d0), vec![vec![caster.addr()]]);
        assert!(e.mem.block_size(caster.addr()).is_some());
        e.call(0x0093_a5a0, &args![caster, 1u32]);
        assert!(e.mem.block_size(caster.addr()).is_none());
    }

    #[test]
    fn remove_actor_from_playercombat_list_needs_a_group() {
        let mut e = engine();
        stub(&mut e, &[0x0098_6500]);
        let this = player(&mut e);
        e.call(0x0093_a660, &args![this, 0xaau32]);
        assert!(called(&e, 0x0098_6500).is_empty());
        e.set(this, PlayerCharacter::pCombatGroup, Ptr::new(0x6000));
        e.call(0x0093_a660, &args![this, 0xaau32]);
        assert_eq!(called(&e, 0x0098_6500), vec![vec![0x6000, 0xaa]]);
    }

    #[test]
    fn add_actor_to_player_combat_list_creates_the_group_once() {
        let mut e = engine();
        stub(&mut e, &[0x0098_67d0, 0x0098_6410]);
        e.map(0x011f_1000, 0x1000);
        e.set_global(COMBAT_MANAGER, 0x7100u32);
        e.register(0x0099_1e80, |_, _| int(0x6000));
        let this = player(&mut e);
        e.call(0x0093_a690, &args![this, 0xaau32]);
        assert_eq!(called(&e, 0x0099_1e80), vec![vec![0x7100]]);
        assert_eq!(called(&e, 0x0098_67d0), vec![vec![0x6000, this.addr()]]);
        assert_eq!(called(&e, 0x0098_6410), vec![vec![0x6000, 0xaa]]);
        assert_eq!(e.mem.u32(this.addr() + 0xd64), 0x6000);
        // With a group in place only the target is added.
        e.call(0x0093_a690, &args![this, 0xbbu32]);
        assert_eq!(called(&e, 0x0099_1e80).len(), 1);
        assert_eq!(called(&e, 0x0098_6410)[1], vec![0x6000, 0xbb]);
    }

    #[test]
    fn fn_0093a7a0_stores_the_occupied_region() {
        let mut e = engine();
        let this = player(&mut e);
        e.call(0x0093_a7a0, &args![this, 0x8800u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x760), 0x8800);
    }

    /// A player whose virtuals `+0x48C` (override query: writes `has_override`
    /// to its second argument), `+0x490`, and the settings object `004181e0`
    /// returns (virtuals `+0x190`, `+0x194`, `+0x198`, `+0x19C`) are recorded.
    fn setter_world(e: &mut Engine, has_override: bool) -> (Ptr<PlayerCharacter>, Ptr) {
        let this = player(e);
        e.register_double(0x00b0_048c, move |e, a| {
            e.mem.set_u8(a[2], has_override as u8);
            float(1.0)
        });
        stub(e, &[0x00b0_0490]);
        put_vtable(
            e,
            this.addr(),
            0x0200_0000,
            &[(0x48c, 0x00b0_048c), (0x490, 0x00b0_0490)],
        );
        let base = object(
            e,
            0x0201_0000,
            &[
                (0x190, 0x00b0_0190),
                (0x194, 0x00b0_0194),
                (0x198, 0x00b0_0198),
                (0x19c, 0x00b0_019c),
            ],
        );
        stub(e, &[0x00b0_0190, 0x00b0_0194, 0x00b0_0198, 0x00b0_019c]);
        e.register_double(0x0041_81e0, move |_, _| int(base.addr()));
        stub(e, &[0x0070_4e10, 0x0088_08f0]);
        (this, base)
    }

    #[test]
    fn set_actor_base_value_without_an_override_uses_the_base_object() {
        let mut e = engine();
        let (this, base) = setter_world(&mut e, false);
        e.call(0x0093_a7c0, &args![this, 7u32, -5i32]);
        assert_eq!(
            called(&e, 0x00b0_0194),
            vec![vec![base.addr(), 7, -5i32 as u32]]
        );
        assert!(called(&e, 0x00b0_0490).is_empty());
        assert_eq!(called(&e, 0x0070_4e10), vec![vec![7]]);
        assert_eq!(called(&e, 0x0088_08f0), vec![vec![this.addr(), 7, 1]]);
    }

    #[test]
    fn set_actor_base_value_with_an_override_sets_it_as_a_float() {
        let mut e = engine();
        let (this, _) = setter_world(&mut e, true);
        e.call(0x0093_a7c0, &args![this, 7u32, -5i32]);
        assert_eq!(called(&e, 0x00b0_0490).len(), 1);
        let call = &called(&e, 0x00b0_0490)[0];
        assert_eq!((call[0], call[1], f(call, 2)), (this.addr(), 7, -5.0));
        assert!(called(&e, 0x00b0_0194).is_empty());
        assert_eq!(called(&e, 0x0070_4e10), vec![vec![7]]);
    }

    #[test]
    fn fn_0093a850_sets_a_float_base_value() {
        let mut e = engine();
        let (this, base) = setter_world(&mut e, false);
        e.call(0x0093_a850, &args![this, 7u32, 2.5f32]);
        let call = &called(&e, 0x00b0_0190)[0];
        assert_eq!((call[0], call[1], f(call, 2)), (base.addr(), 7, 2.5));
        assert_eq!(called(&e, 0x0088_08f0), vec![vec![this.addr(), 7, 1]]);
    }

    #[test]
    fn fn_0093a850_with_an_override_sets_the_override() {
        let mut e = engine();
        let (this, _) = setter_world(&mut e, true);
        e.call(0x0093_a850, &args![this, 7u32, 2.5f32]);
        let call = &called(&e, 0x00b0_0490)[0];
        assert_eq!((call[0], call[1], f(call, 2)), (this.addr(), 7, 2.5));
        assert!(called(&e, 0x00b0_0190).is_empty());
    }

    /// The additions' extra doubles: `0066ee10` (does the value apply) and
    /// `0066ee50`, and the owner's virtual `+0xC` returning `current`.
    fn delta_world(
        e: &mut Engine,
        has_override: bool,
        applies: bool,
        current: f32,
    ) -> (Ptr<PlayerCharacter>, Ptr) {
        let (this, base) = setter_world(e, has_override);
        e.register_double(0x0066_ee10, move |_, _| int(applies as u32));
        stub(e, &[0x0066_ee50]);
        e.register_double(0x00b0_000c, move |_, _| float(current as f64));
        e.mem.set_u32(this.addr() + 0xa4, 0x0203_0000);
        e.map(0x0203_0000, 0x100);
        e.mem.set_u32(0x0203_000c, 0x00b0_000c);
        (this, base)
    }

    #[test]
    fn fn_0093a8f0_without_an_override_adds_through_the_base_object() {
        let mut e = engine();
        let (this, base) = delta_world(&mut e, false, true, 10.0);
        e.call(0x0093_a8f0, &args![this, 7u32, 3i32]);
        assert_eq!(called(&e, 0x00b0_019c), vec![vec![base.addr(), 7, 3]]);
        assert_eq!(called(&e, 0x00b0_000c), vec![vec![this.addr() + 0xa4, 7]]);
        let report = &called(&e, 0x0066_ee50)[0];
        assert_eq!(
            (report[0], report[1], f(report, 2), f(report, 3), report[4]),
            (this.addr() + 0xa4, 7, 10.0, 3.0, 0)
        );
    }

    #[test]
    fn fn_0093a8f0_with_an_override_sets_the_sum() {
        let mut e = engine();
        let (this, _) = delta_world(&mut e, true, true, 10.5);
        e.call(0x0093_a8f0, &args![this, 7u32, 3i32]);
        let call = &called(&e, 0x00b0_0490)[0];
        assert_eq!((call[0], call[1], f(call, 2)), (this.addr(), 7, 13.5));
        assert!(called(&e, 0x00b0_019c).is_empty());
    }

    #[test]
    fn fn_0093a8f0_treats_a_value_that_does_not_apply_as_zero() {
        let mut e = engine();
        let (this, _) = delta_world(&mut e, true, false, 10.5);
        e.call(0x0093_a8f0, &args![this, 7u32, 3i32]);
        assert!(called(&e, 0x00b0_000c).is_empty());
        let call = &called(&e, 0x00b0_0490)[0];
        assert_eq!(f(call, 2), 3.0);
        let report = &called(&e, 0x0066_ee50)[0];
        assert_eq!((f(report, 2), f(report, 3)), (0.0, 3.0));
    }

    #[test]
    fn fn_0093aa10_without_an_override_adds_a_float_through_the_base_object() {
        let mut e = engine();
        let (this, base) = delta_world(&mut e, false, true, 10.0);
        e.call(0x0093_aa10, &args![this, 7u32, 0.25f32]);
        let call = &called(&e, 0x00b0_0198)[0];
        assert_eq!((call[0], call[1], f(call, 2)), (base.addr(), 7, 0.25));
        let report = &called(&e, 0x0066_ee50)[0];
        assert_eq!((f(report, 2), f(report, 3)), (10.0, 0.25));
    }

    #[test]
    fn fn_0093aa10_with_an_override_sets_the_sum() {
        let mut e = engine();
        let (this, _) = delta_world(&mut e, true, true, 10.0);
        e.call(0x0093_aa10, &args![this, 7u32, 0.25f32]);
        let call = &called(&e, 0x00b0_0490)[0];
        assert_eq!((call[0], call[1], f(call, 2)), (this.addr(), 7, 10.25));
        assert_eq!(called(&e, 0x0088_08f0), vec![vec![this.addr(), 7, 1]]);
    }

    #[test]
    fn fn_0093ab30_passes_the_action_point_cost_to_the_virtual() {
        let mut e = engine();
        let this = player(&mut e);
        stub(&mut e, &[0x00b0_033c]);
        put_vtable(&mut e, this.addr(), 0x0200_0000, &[(0x33c, 0x00b0_033c)]);
        e.register(0x0052_5430, |_, _| int(1));
        e.register(0x0066_dca0, |_, a| float(a[0] as f64 + a[1] as f64 * 0.5));
        e.call(0x0093_ab30, &args![this, 10u32]);
        assert_eq!(called(&e, 0x0052_5430), vec![vec![0x011f_2250]]);
        assert_eq!(called(&e, 0x0066_dca0), vec![vec![10, 1, 0, 0]]);
        let call = &called(&e, 0x00b0_033c)[0];
        assert_eq!((call[0], f(call, 1)), (this.addr(), 10.5));
    }

    /// The action-point spender's world: god mode (`009526b0`), the other
    /// test (`0042ce10`) on the global object, and the virtual `+0x3AC`.
    fn spender(e: &mut Engine, god: bool, blocked: bool) -> Ptr<PlayerCharacter> {
        let this = player(e);
        stub(e, &[0x00b0_03ac]);
        put_vtable(e, this.addr(), 0x0200_0000, &[(0x3ac, 0x00b0_03ac)]);
        e.register_double(0x0095_26b0, move |_, _| int(god as u32));
        e.register_double(0x0042_ce10, move |_, _| int(blocked as u32));
        e.map(0x011d_d000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(GLOBAL_OBJECT_0042CE10, 0x9900u32);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        this
    }

    #[test]
    fn fn_0093ab80_spends_positive_amounts_as_a_negative_change() {
        let mut e = engine();
        let this = spender(&mut e, false, false);
        e.call(0x0093_ab80, &args![this, 4.5f32]);
        let call = &called(&e, 0x00b0_03ac)[0];
        assert_eq!(
            (call[0], call[1], f(call, 2), call[3]),
            (this.addr(), 0xc, -4.5, 0)
        );
        assert_eq!(called(&e, 0x0042_ce10), vec![vec![0x9900]]);
    }

    #[test]
    fn fn_0093ab80_does_nothing_for_zero_or_negative_amounts() {
        for amount in [0.0f32, -1.0] {
            let mut e = engine();
            let this = spender(&mut e, false, false);
            e.call(0x0093_ab80, &args![this, amount]);
            assert!(called(&e, 0x00b0_03ac).is_empty());
        }
    }

    #[test]
    fn fn_0093ab80_does_nothing_in_god_mode_or_when_the_other_test_says_so() {
        let mut e = engine();
        let this = spender(&mut e, true, false);
        e.call(0x0093_ab80, &args![this, 4.5f32]);
        assert!(called(&e, 0x00b0_03ac).is_empty());
        assert!(called(&e, 0x0042_ce10).is_empty());

        let mut e = engine();
        let this = spender(&mut e, false, true);
        e.call(0x0093_ab80, &args![this, 4.5f32]);
        assert!(called(&e, 0x00b0_03ac).is_empty());
    }

    // ---- the constructor and the destructor ----

    const PROCESS_VTABLE: u32 = 0x0204_0000;

    /// Every address the constructor calls other than `operator new`.
    const CONSTRUCTOR_CALLEES: &[u32] = &[
        0x008d_1d30,
        0x0063_3c90,
        0x006f_48b0,
        0x004f_6320,
        0x006a_8500,
        0x0096_a280,
        0x008d_4360,
        0x0096_a570,
        0x0096_a5c0,
        0x0040_4eb0,
        0x0048_4ab0,
        0x008d_66b0,
        0x008d_1820,
        0x005d_14d0,
        0x0066_b0d0,
        0x0096_d470,
        0x0093_0c70,
        0x0045_34f0,
        0x00c6_d770,
        0x0070_c440,
        0x0095_0610,
        0x0051_9020,
        0x0085_1d10,
        0x0096_9e90,
        0x0047_0470,
        0x0096_27f0,
        0x004e_d780,
        0x0040_3550,
        0x0096_4190,
        0x0056_7dd0,
        0x005d_add0,
        0x0040_3d30,
        0x0096_9110,
        0x008b_bbf0,
        0x007c_b2e0,
        0x0040_4ee0,
        0x0048_3710,
        0x0041_8900,
        0x00ec_782f,
    ];

    /// Doubles for everything the constructor calls. `camera_owner` is what
    /// `009306d0` returns.
    fn constructor_world(e: &mut Engine, camera_owner: u32) -> (Ptr<PlayerCharacter>, u32) {
        stub(e, CONSTRUCTOR_CALLEES);
        // Constructors return their object.
        stub_returning_this(
            e,
            &[
                0x0096_a2d0, // list
                0x0041_a250, // sound handle
                0x0068_15c0, // NiPoint3 default
                0x0062_0850, // CameraCaster
                0x0044_ddc0, // flags getter
                0x0056_7dd0,
            ],
        );
        e.register(0x008d_7510, |e, a| {
            e.mem.set_u32(a[0], PROCESS_VTABLE);
            int(a[0])
        });
        for slot in [0, 4, 0x6e0] {
            e.register(0x00b1_0000 + slot, |_, _| Ret::default());
        }
        vtable_only(
            e,
            PROCESS_VTABLE,
            &[(0, 0x00b1_0000), (4, 0x00b1_0004), (0x6e0, 0x00b1_06e0)],
        );
        e.register(0x0093_1850, |_, a| int(a[0] ^ 0x1000));
        e.register_double(0x0093_06d0, move |_, _| int(camera_owner));
        e.register(0x0054_6a90, |_, _| int(0x77));
        e.register(0x0061_b9d0, |_, _| int(0xfeed));
        // The settings getters return a pointer to a cell holding a value
        // that depends on the setting's address.
        for getter in [
            SETTING_FLOAT_GETTER,
            SETTING_BYTE_GETTER,
            SETTING_INT_GETTER,
        ] {
            e.register(getter, |e, a| {
                let cell = e.mem.alloc(8);
                e.mem.set_u32(cell, a[0] & 0xffff);
                if [
                    0x011c_cf2c,
                    0x011e_0934,
                    0x0120_315c,
                    0x011c_da58,
                    0x0120_3168,
                    0x0120_3150,
                ]
                .contains(&a[0])
                {
                    e.mem.set_f32(cell, (a[0] & 0xffff) as f32);
                }
                int(cell)
            });
        }
        for (addr, len) in [
            (0x011e_0000, 0x1000),
            (0x011f_4000, 0x1000),
            (0x0101_2000, 0x1000),
            (0x0101_7000, 0x1000),
            (0x0108_a000, 0x1000),
            (0x011a_3000, 0x1000),
            (0x011d_e000, 0x1000),
        ] {
            e.map(addr, len);
        }
        // The virtual at +0x258 that the AI-control setters call through the
        // player's own vtable.
        e.mem.set_u32(0x0108_aa3c + 0x258, 0x00b3_0258);
        e.register(0x00b3_0258, |_, _| Ret::default());
        e.set_global(TIMER_RESET, 5.5f32);
        e.set_global(BLOCK_ACTIVATE_TIMER_RESET, 1.25f32);
        e.set_global(EYE_HEIGHT_DEFAULT, 64.0f32);
        let this = Ptr::<PlayerCharacter>::new(e.mem.alloc(0xe50));
        // The old process the constructor replaces.
        let old: Ptr = Ptr::new(e.mem.alloc(0x40));
        put_vtable(e, old.addr(), 0x0205_0000, &[(0, 0x00b2_0000)]);
        e.register(0x00b2_0000, |_, _| Ret::default());
        e.mem.set_u32(this.addr() + 0x68, old.addr());
        (this, old.addr())
    }

    #[test]
    fn construct_initializes_the_player_and_replaces_the_process() {
        let mut e = engine();
        let (this, old_process) = constructor_world(&mut e, 0x5000);
        e.map(0x0000_5000, 0x1000);
        let result = e.call(0x0093_8180, &args![this]);
        assert_eq!(result.u32(), this.addr());
        // Calls begin with the base constructor and end with the guard.
        let order = call_addresses(&e);
        assert_eq!(order[0], 0x0093_8180);
        assert_eq!(order[1], 0x008d_1d30);
        assert_eq!(*order.last().unwrap(), 0x0040_4ee0);
        // The six vtables.
        for (offset, table) in [
            (0, 0x0108_aa3c),
            (0x18, 0x0108_aa30),
            (0x88, 0x0108_a9dc),
            (0x94, 0x0108_a9a4),
            (0xa4, 0x0108_a974),
            (0xa8, 0x0108_a92c),
        ] {
            assert_eq!(e.mem.u32(this.addr() + offset), table);
        }
        // The process: a fresh 0x46C-byte object, told about the old one, and
        // the old one destroyed.
        let process = e.mem.u32(this.addr() + 0x68);
        assert_ne!(process, old_process);
        assert_eq!(e.mem.block_size(process), Some(0x470));
        assert_eq!(called(&e, 0x00b1_0004), vec![vec![process, old_process]]);
        assert_eq!(called(&e, 0x00b2_0000), vec![vec![old_process, 1]]);
        assert_eq!(called(&e, 0x00b1_06e0), vec![vec![process, 0]]);
        assert_eq!(
            called(&e, 0x0096_d470),
            vec![
                vec![0x011e_0e80, this.addr(), this.addr() ^ 0x1000],
                vec![0x011e_0e80, this.addr(), this.addr() ^ 0x1000]
            ]
        );
        // Embedded members, in order.
        assert_eq!(
            called(&e, 0x0096_a2d0).first(),
            Some(&vec![this.addr() + 0x5e4])
        );
        assert_eq!(
            called(&e, 0x00ec_782f),
            vec![
                vec![this.addr() + 0x804, 0xc, 6, 0x0041_a250, 0x0048_3710],
                vec![this.addr() + 0x884, 8, 0x4a, 0x0096_a2d0, 0x0046_ffb0],
                vec![this.addr() + 0xadc, 8, 0x4a, 0x0096_a2d0, 0x0046_ffb0],
            ]
        );
        // Field values.
        assert!(e.get(this, PlayerCharacter::bCanWait));
        assert!(!e.get(this, PlayerCharacter::bCanFastTravel));
        assert!(e.get(this, PlayerCharacter::bOnElevator));
        assert!(e.get(this, PlayerCharacter::bAllowEGMCacheClear));
        assert!(e.get(this, PlayerCharacter::bAlwaysHardcore));
        assert!(e.get(this, PlayerCharacter::bShowQuestItemsInInventory));
        assert_eq!(e.get(this, PlayerCharacter::eskilladvance), 0x4d);
        assert_eq!(e.get(this, PlayerCharacter::iCharacterSeed), 0xfeed);
        assert_eq!(e.get(this, PlayerCharacter::fStealWarningTimer), 5.5);
        assert_eq!(e.get(this, PlayerCharacter::fPickPocketWarningTimer), 5.5);
        assert_eq!(e.get(this, PlayerCharacter::fLastHelloTime), 5.5);
        assert_eq!(e.get(this, PlayerCharacter::fBlockActivateTimer), 1.25);
        assert_eq!(e.get(this, PlayerCharacter::fEyeHeight), 64.0);
        assert_eq!(e.get(this, PlayerCharacter::fWorldFOV), 0x315c as f32);
        assert_eq!(e.get(this, PlayerCharacter::eDifficultyLevel), 0x0940);
        assert_eq!(e.get(this, PlayerCharacter::eKillCameraSetting), 0x087c);
        assert!(e.get(this, PlayerCharacter::bAlwaysRun));
        // The camera caster was built with the setting and the index 9.
        let caster = e.get(this, PlayerCharacter::pCameraCaster).addr();
        assert_eq!(
            called(&e, 0x0062_0850),
            vec![vec![caster, ((0x0934u32) as f32).to_bits(), 9]]
        );
        assert_eq!(called(&e, 0x00c6_d770), vec![vec![0x5000, 9]]);
        assert_eq!(called(&e, 0x0070_c440).len(), 1);
        assert_eq!(e.mem.u32(0x5000 + 0x604), 0x3e8);
        assert_eq!(called(&e, 0x0045_34f0), vec![vec![this.addr(), 1]]);
        // The heap lists exist.
        for offset in [0x210, 0x238, 0x610, 0x614, 0x618] {
            assert_ne!(e.mem.u32(this.addr() + offset), 0, "{offset:x}");
        }
        assert_eq!(e.mem.u32(this.addr() + 0x23c), 0);
        // The two 0x1A0-byte objects.
        for global in [0x011e_0774, 0x011e_0760] {
            let object = e.mem.u32(global);
            assert_eq!(e.mem.block_size(object), Some(0x1a0));
            assert_eq!(e.mem.f32(object + 0x198), 0.0);
        }
        // The achievement entries: the flag quirk makes the last byte 1.
        for i in 0..3 {
            assert_eq!(e.mem.u8(this.addr() + 0x1c8 + i * 0xc + 8), 1);
        }
        // Eight nulls pushed to the hot-key array; memset of the wobble nodes.
        assert_eq!(called(&e, 0x007c_b2e0).len(), 8);
        assert_eq!(
            called(&e, 0x0040_3d30),
            vec![vec![this.addr() + 0xd74, 0, 0x60]]
        );
        // The failure-sound handles were re-assigned: seven times.
        assert_eq!(called(&e, 0x0041_8900).len(), 7);
        // The flags getter result got the 0x400 bit.
        assert_eq!(
            called(&e, 0x0040_3550),
            vec![vec![this.addr(), this.addr() | 0x400]]
        );
        // The hostile-AI flags were set through the virtual-calling setters
        // (no process virtual here: the world has none at +0x258).
    }

    #[test]
    fn construct_without_a_camera_owner_leaves_the_camera_caster_null() {
        let mut e = engine();
        let (this, _) = constructor_world(&mut e, 0);
        e.call(0x0093_8180, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x21c), 0);
        assert!(called(&e, 0x0062_0850).is_empty());
        assert!(called(&e, 0x0045_34f0).is_empty());
    }

    /// Every address the destructor calls apart from the list helpers it
    /// loops on.
    const DESTRUCTOR_CALLEES: &[u32] = &[
        0x0094_7d10,
        0x0094_8050,
        0x0047_0470,
        0x008d_68d0,
        0x008d_1920,
        0x0095_0c20,
        0x0047_02f0,
        0x0062_08d0,
        0x0066_b0d0,
        0x0040_b800,
        0x008b_3180,
        0x0096_7290,
        0x0096_90a0,
        0x0094_eb40,
        0x0085_8700,
        0x005e_03d0,
        0x0096_1f90,
        0x00ad_88f0,
        0x0048_3710,
        0x0041_8900,
        0x0096_2490,
        0x0070_5fc0,
        0x00e9_8e20,
        0x0096_55a0,
        0x0096_55e0,
        0x0095_2f90,
        0x0096_a5f0,
        0x0096_a5a0,
        0x0045_cec0,
        0x00ec_5fce,
        0x0046_ffb0,
        0x008d_4380,
        0x0096_a4b0,
        0x004f_64b0,
        0x006f_4930,
        0x008d_2060,
    ];

    /// The shared state of the list doubles: the items still in each list.
    type Lists = Rc<RefCell<HashMap<u32, Vec<u32>>>>;

    /// The destructor's world: every callee doubled, `operator delete`
    /// recording its blocks, and `BSSimpleList` helpers that work on cells
    /// (a list is the address of a word holding its head item). Returns the
    /// player, the lists' items, and the recorded deletes.
    fn destructor_world(e: &mut Engine) -> (Ptr<PlayerCharacter>, Lists, Rc<RefCell<Vec<u32>>>) {
        stub(e, DESTRUCTOR_CALLEES);
        stub_returning_this(e, &[0x0041_a250]);
        let lists: Lists = Rc::default();
        let deleted: Rc<RefCell<Vec<u32>>> = Rc::default();
        // Item address of a list's head node: the cell itself.
        e.register(LIST_ITEM_ADDRESS, |_, a| int(a[0]));
        e.register(LIST_NEXT, |_, _| int(0));
        e.register(SOUND_HANDLE_IS_VALID, |_, _| int(0));
        let state = lists.clone();
        e.register_double(LIST_REMOVE_HEAD, move |e, a| {
            let mut state = state.borrow_mut();
            let items = state.entry(a[0]).or_default();
            if !items.is_empty() {
                items.remove(0);
            }
            e.mem.set_u32(a[0], items.first().copied().unwrap_or(0));
            Ret::default()
        });
        let state = lists.clone();
        e.register_double(LIST_IS_EMPTY, move |_, a| {
            int(state.borrow().get(&a[0]).is_none_or(|v| v.is_empty()) as u32)
        });
        let record = deleted.clone();
        e.register_double(OPERATOR_DELETE, move |_, a| {
            record.borrow_mut().push(a[0]);
            Ret::default()
        });
        for (addr, len) in [(0x011d_e000, 0x1000), (0x011e_0000, 0x1000)] {
            e.map(addr, len);
        }
        let this = Ptr::<PlayerCharacter>::new(e.mem.alloc(0xe50));
        (this, lists, deleted)
    }

    /// A heap list cell holding `items` (head item in the cell).
    fn list_of(e: &mut Engine, lists: &Lists, items: &[u32]) -> u32 {
        let cell = e.mem.alloc(8);
        e.mem.set_u32(cell, items.first().copied().unwrap_or(0));
        lists.borrow_mut().insert(cell, items.to_vec());
        cell
    }

    #[test]
    fn destruct_of_an_empty_player_destroys_the_members_and_the_base() {
        let mut e = engine();
        let (this, _, deleted) = destructor_world(&mut e);
        e.mem.set_u32(0x011e_0774, 0x1111);
        e.mem.set_u32(0x011e_0760, 0x2222);
        e.call(0x0093_98d0, &args![this]);
        for (offset, table) in [
            (0, 0x0108_aa3c),
            (0x18, 0x0108_aa30),
            (0x88, 0x0108_a9dc),
            (0x94, 0x0108_a9a4),
            (0xa4, 0x0108_a974),
            (0xa8, 0x0108_a92c),
        ] {
            assert_eq!(e.mem.u32(this.addr() + offset), table);
        }
        let order = call_addresses(&e);
        assert_eq!(order[0], 0x0093_98d0);
        assert_eq!(order[1], 0x0094_7d10);
        assert_eq!(order[2], 0x0094_8050);
        assert_eq!(*order.last().unwrap(), 0x008d_2060);
        // Only the two global objects are deleted when no list has items.
        assert_eq!(*deleted.borrow(), vec![0, 0, 0x1111, 0x2222]);
        // The members are destroyed in reverse order of construction: the
        // last-declared (hot-key array) first, the notes list last.
        assert_eq!(called(&e, 0x0096_a5f0), vec![vec![this.addr() + 0xe3c]]);
        let destroyed: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| [0x0045_cec0, 0x0046_ffb0].contains(a))
            .map(|(_, w)| w[0] - this.addr())
            .collect();
        assert_eq!(
            destroyed,
            vec![
                0xdec, 0xd3c, 0xad4, 0x87c, 0x868, 0x864, 0x84c, 0x7d4, 0x7c8, 0x774, 0x6c4, 0x6bc,
                0x6b0, 0x6a8, 0x69c, 0x694, 0x634, 0x5fc, 0x5e4
            ]
        );
        assert_eq!(
            called(&e, 0x00ec_5fce),
            vec![
                vec![this.addr() + 0xadc, 8, 0x4a, 0x0046_ffb0],
                vec![this.addr() + 0x884, 8, 0x4a, 0x0046_ffb0],
                vec![this.addr() + 0x804, 0xc, 6, 0x0048_3710],
            ]
        );
        // Nothing to release: no list deleting destructor runs.
        assert!(called(&e, 0x0047_02f0).is_empty());
        assert!(called(&e, 0x0040_b800).is_empty());
        assert!(called(&e, 0x0085_8700).is_empty());
        assert!(called(&e, 0x005e_03d0).is_empty());
        // Every perk list is cleared.
        let cleared: Vec<u32> = called(&e, 0x0047_0470)
            .iter()
            .map(|w| w[0] - this.addr())
            .collect();
        assert_eq!(cleared.len(), 1 + 4 + 1 + 1 + 74 + 1 + 74 + 1);
        assert_eq!(cleared[..6], [0x768, 0x6a8, 0x6b0, 0x6c4, 0x6bc, 0x84c]);
    }

    #[test]
    fn destruct_releases_the_processes_lists_and_items() {
        let mut e = engine();
        let (this, lists, deleted) = destructor_world(&mut e);
        let t = this.addr();
        for target in [0x00b4_0000u32, 0x00b4_0010] {
            e.register_double(target, |_, _| Ret::default());
        }
        // The process and the first-person effect have destructors at virtual +0.
        let process = object(&mut e, 0x0200_0000, &[(0, 0x00b4_0000)]);
        e.mem.set_u32(t + 0x68, process.addr());
        let weapon_effect = object(&mut e, 0x0201_0000, &[(0, 0x00b4_0000)]);
        e.mem.set_u32(t + 0x6a4, weapon_effect.addr());
        let border_regions = object(&mut e, 0x0202_0000, &[(0, 0x00b4_0000)]);
        e.mem.set_u32(t + 0x7b0, border_regions.addr());
        // Eat/drink list present, worn enchantments absent.
        let eat_drink = list_of(&mut e, &lists, &[]);
        e.mem.set_u32(t + 0x238, eat_drink);
        let caster = e.mem.alloc(8);
        e.mem.set_u32(t + 0x21c, caster);
        e.mem.set_u32(t + 0x6ec, 0x1234);
        e.mem.set_u32(t + 0x788, 0x2345);
        e.mem.set_u32(0x011d_e45c, 0x8800);
        // Active effects: two items with virtual destructors.
        let effect_1 = object(&mut e, 0x0203_0000, &[(0, 0x00b4_0000)]);
        let effect_2 = object(&mut e, 0x0204_0000, &[(0, 0x00b4_0000)]);
        let effects = list_of(&mut e, &lists, &[effect_1.addr(), effect_2.addr()]);
        e.mem.set_u32(t + 0x210, effects);
        // Actions: plain blocks deleted until the first null item.
        let actions = list_of(&mut e, &lists, &[0x111, 0x222]);
        e.mem.set_u32(t + 0x60c, actions);
        let casino = list_of(&mut e, &lists, &[0x333]);
        e.mem.set_u32(t + 0x610, casino);
        // Caravan cards use the item's virtual +0x10; the active list is
        // empty and is left alone.
        let card = object(&mut e, 0x0205_0000, &[(0x10, 0x00b4_0010)]);
        let inactive = list_of(&mut e, &lists, &[card.addr()]);
        e.mem.set_u32(t + 0x614, inactive);
        let active = list_of(&mut e, &lists, &[]);
        e.mem.set_u32(t + 0x618, active);
        // Perks: an embedded list of two nodes; companion perks of one.
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0x555);
        e.mem.set_u32(t + 0x87c, 0x444);
        e.mem.set_u32(t + 0xad4, 0x666);
        let chain: HashMap<u32, u32> =
            HashMap::from([(t + 0x87c, node), (node, 0), (t + 0xad4, 0)]);
        e.register_double(LIST_NEXT, move |_, a| int(chain[&a[0]]));
        // Sound 2 is valid.
        let valid_sound = t + 0x804 + 2 * 0xc;
        e.register_double(SOUND_HANDLE_IS_VALID, move |_, a| {
            int((a[0] == valid_sound) as u32)
        });
        e.mem.set_u32(0x011e_0774, 0x7771);
        e.mem.set_u32(0x011e_0760, 0x7772);
        e.call(0x0093_98d0, &args![this]);

        // The process is destroyed and its pointer cleared.
        assert_eq!(called(&e, 0x00b4_0000)[0], vec![process.addr(), 1]);
        assert_eq!(e.mem.u32(t + 0x68), 0);
        // The weapon effect and the border regions have virtual destructors.
        assert!(called(&e, 0x00b4_0000).contains(&vec![weapon_effect.addr(), 1]));
        assert!(called(&e, 0x00b4_0000).contains(&vec![border_regions.addr(), 1]));
        assert_eq!(called(&e, 0x005e_03d0), vec![vec![border_regions.addr()]]);
        assert_eq!(e.mem.u32(t + 0x7b0), 0);
        // The effects list: both items destroyed, then the list itself.
        assert!(called(&e, 0x00b4_0000).contains(&vec![effect_1.addr(), 1]));
        assert!(called(&e, 0x00b4_0000).contains(&vec![effect_2.addr(), 1]));
        // Deleted blocks: the camera caster, actions (all), casino data, perks,
        // companion perks, then the two global objects.
        assert_eq!(
            *deleted.borrow(),
            vec![caster, 0x111, 0x222, 0x333, 0x444, 0x555, 0x666, 0x7771, 0x7772]
        );
        // The caravan card's virtual +0x10 runs with 1.
        assert_eq!(called(&e, 0x00b4_0010), vec![vec![card.addr(), 1]]);
        // List objects are deleted with their deleting destructors, in order:
        // eat/drink, effects, actions, casino, inactive caravan cards. The
        // active (empty) list is skipped.
        let deleted_lists: Vec<u32> = called(&e, 0x0047_02f0).iter().map(|w| w[0]).collect();
        assert_eq!(
            deleted_lists,
            vec![eat_drink, effects, actions, casino, inactive]
        );
        assert!(called(&e, 0x0047_02f0).iter().all(|w| w[1] == 1));
        // The camera caster, spell, state buffer and the single valid sound.
        assert_eq!(called(&e, 0x0062_08d0), vec![vec![caster]]);
        assert_eq!(called(&e, 0x0040_b800), vec![vec![0x1234, 1]]);
        assert_eq!(called(&e, 0x0085_8700), vec![vec![0x8800, 0x2345]]);
        assert_eq!(called(&e, 0x00ad_88f0).len(), 1);
        assert_eq!(called(&e, 0x0041_8900).len(), 1);
        assert_eq!(called(&e, 0x0048_3710).len(), 3);
    }

    #[test]
    fn player_vector_deleting_destructor_frees_only_with_the_flag() {
        let mut e = engine();
        let (this, _, deleted) = destructor_world(&mut e);
        let result = e.call(0x0093_98a0, &args![this, 0u32]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(*deleted.borrow(), vec![0; 4]);
        e.call(0x0093_98a0, &args![this, 1u32]);
        assert_eq!(*deleted.borrow(), [vec![0; 8], vec![this.addr()]].concat());
        assert_eq!(called(&e, 0x008d_2060).len(), 2);
    }

    /// Gives the player an `ActorValueOwner` base (at +0xA4) whose vtable at
    /// `base` has the given slots.
    fn give_owner(e: &mut Engine, this: Ptr<PlayerCharacter>, base: u32, slots: &[(u32, u32)]) {
        put_vtable(e, this.addr() + 0xa4, base, slots);
    }

    /// A double for `_ftol2`: truncates the leading `f64` argument.
    fn register_ftol(e: &mut Engine) {
        e.register(FTOL, |_, a| {
            int(f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32)
        });
    }

    #[test]
    fn fn_0093abe0_writes_both_floats_through_the_pointers() {
        let mut e = engine();
        let this = player(&mut e);
        e.register(0x008a_0c20, |_, _| float(2.5));
        e.register_double(0x00b0_0008, |_, _| int(7));
        give_owner(&mut e, this, 0x0200_0000, &[(8, 0x00b0_0008)]);
        let out = e.mem.alloc(8);
        e.call(0x0093_abe0, &args![this, out, out + 4]);
        assert_eq!(e.mem.f32(out), 7.0);
        assert_eq!(e.mem.f32(out + 4), 2.5);
        assert_eq!(called(&e, 0x008a_0c20), vec![vec![this.addr()]]);
        assert_eq!(
            called(&e, 0x00b0_0008),
            vec![vec![this.addr() + 0xa4, 0x2e]]
        );
    }

    /// An `ActorValueOwner` (the player + 0xA4) with the virtuals at +0x0,
    /// +0x4 and +0x18 returning the given values.
    fn base_owner(
        e: &mut Engine,
        slot0: u32,
        slot4: f32,
        slot18: f32,
    ) -> (Ptr<PlayerCharacter>, Ptr) {
        let this = player(e);
        e.register_double(0x00b0_0000, move |_, _| int(slot0));
        e.register_double(0x00b0_0004, move |_, _| float(slot4 as f64));
        e.register_double(0x00b0_0018, move |_, _| float(slot18 as f64));
        give_owner(
            e,
            this,
            0x0200_0000,
            &[(0, 0x00b0_0000), (4, 0x00b0_0004), (0x18, 0x00b0_0018)],
        );
        (this, Ptr::new(this.addr() + 0xa4))
    }

    #[test]
    fn fn_0093ac20_sums_the_base_value_and_the_three_truncated_parts() {
        let mut e = engine();
        let (this, owner) = base_owner(&mut e, 10, 0.0, 0.0);
        e.register(0x0041_81e0, |_, _| int(1));
        e.register(0x0094_c3d0, |_, a| float([1.9, 2.5, -0.5][a[1] as usize]));
        register_ftol(&mut e);
        let result = e.call(0x0093_ac20, &args![owner, 5u32]).i32();
        assert_eq!(result, 10 + 1 + 2);
        assert_eq!(called(&e, 0x004_181e0), vec![vec![this.addr()]]);
        assert_eq!(
            called(&e, 0x0094_c3d0),
            vec![
                vec![this.addr(), 0, 5],
                vec![this.addr(), 1, 5],
                vec![this.addr(), 2, 5]
            ]
        );
    }

    #[test]
    fn fn_0093ac20_is_zero_without_the_object() {
        let mut e = engine();
        let (_, owner) = base_owner(&mut e, 10, 0.0, 0.0);
        e.register(0x0041_81e0, |_, _| int(0));
        assert_eq!(e.call(0x0093_ac20, &args![owner, 5u32]).i32(), 0);
        assert!(called(&e, 0x00b0_0000).is_empty());
    }

    #[test]
    fn fn_0093acb0_sums_the_float_base_value_and_the_three_parts() {
        let mut e = engine();
        let (this, owner) = base_owner(&mut e, 0, 1.5, 0.0);
        e.register(0x0094_c3d0, |_, a| float([0.25, 0.5, 0.125][a[1] as usize]));
        let result = e.call(0x0093_acb0, &args![owner, 5u32]).f32();
        assert_eq!(result, 2.375);
        assert_eq!(called(&e, 0x00b0_0004), vec![vec![owner.addr(), 5]]);
        assert_eq!(called(&e, 0x0094_c3d0)[2], vec![this.addr(), 2, 5]);
    }

    #[test]
    fn fn_0093ad30_converts_the_virtual_result_and_truncates_it() {
        let mut e = engine();
        let this = player(&mut e);
        e.register_double(0x00b0_0020, |_, _| float(4.75));
        put_vtable(&mut e, this.addr(), 0x0200_0000, &[(0x20, 0x00b0_0020)]);
        e.register(0x0040_6ce0, |_, a| float(f32::from_bits(a[0]) as f64 * 2.0));
        register_ftol(&mut e);
        let result = e.call(0x0093_ad30, &args![this, 3u32]).i32();
        assert_eq!(result, 9);
        assert_eq!(called(&e, 0x00b0_0020), vec![vec![this.addr(), 3]]);
    }

    /// The world of `GetPermanentActorFloatValue`: the owner's virtuals +0x4
    /// and +0x18, the player's virtual +0x48C, the skill derivation and the
    /// clamp (which doubles its argument so the test sees it is returned).
    fn permanent_world(e: &mut Engine, skill: bool) -> Ptr {
        let (this, owner) = base_owner(e, 0, 3.0, 10.0);
        put_vtable(e, this.addr(), 0x0210_0000, &[(0x48c, 0x00b0_048c)]);
        e.register_double(0x00b0_048c, |_, _| float(1.5));
        e.register_double(0x0047_f060, move |_, _| int(skill as u32));
        e.register(0x0064_3c90, |_, _| float(2.0));
        e.register(0x0066_f190, |_, a| float(f32::from_bits(a[1]) as f64 * 2.0));
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLAYER_SINGLETON, 0u32);
        owner
    }

    #[test]
    fn permanent_float_value_of_a_skill_adds_the_modifier_and_the_derived_skill() {
        let mut e = engine();
        let owner = permanent_world(&mut e, true);
        let result = e.call(0x0093_ad60, &args![owner, 5u32]).f32();
        // (10 + (1.5 + 2.0)) clamped (doubled by the double).
        assert_eq!(result, 27.0);
        assert_eq!(called(&e, 0x0064_3c90), vec![vec![0, 5, 1]]);
        let clamp = &called(&e, 0x0066_f190)[0];
        assert_eq!((clamp[0], f(clamp, 1)), (5, 13.5));
        let modifier = &called(&e, 0x00b0_048c)[0];
        assert_eq!((modifier[0], modifier[1]), (owner.addr() - 0xa4, 5));
    }

    #[test]
    fn permanent_float_value_derives_from_the_player_singleton_owner() {
        let mut e = engine();
        let owner = permanent_world(&mut e, true);
        e.set_global(PLAYER_SINGLETON, 0x7000u32);
        e.call(0x0093_ad60, &args![owner, 5u32]);
        assert_eq!(called(&e, 0x0064_3c90), vec![vec![0x7000 + 0xa4, 5, 1]]);
    }

    #[test]
    fn permanent_float_value_of_other_values_adds_the_two_virtuals() {
        let mut e = engine();
        let owner = permanent_world(&mut e, false);
        let result = e.call(0x0093_ad60, &args![owner, 5u32]).f32();
        assert_eq!(result, 26.0);
        assert!(called(&e, 0x0064_3c90).is_empty());
        assert!(called(&e, 0x00b0_048c).is_empty());
    }

    #[test]
    fn fn_0093ae40_sets_the_value_and_notifies() {
        let mut e = engine();
        let this = player(&mut e);
        stub(&mut e, &[0x0088_0700, 0x0070_4e10, 0x0088_08f0]);
        e.call(0x0093_ae40, &args![this, 7u32, 1.5f32]);
        let call = &called(&e, 0x0088_0700)[0];
        assert_eq!((call[0], call[1], f(call, 2)), (this.addr(), 7, 1.5));
        assert_eq!(called(&e, 0x0070_4e10), vec![vec![7]]);
        assert_eq!(called(&e, 0x0088_08f0), vec![vec![this.addr(), 7, 1]]);
    }

    /// The world of the damage-style setters: `0066ee10` says the value does
    /// not apply (the old value is 0), the flag test `00406d70` answers per
    /// flag, `00880850`/`00880890` give the delta, `00952730` the permission.
    fn damage_world(
        e: &mut Engine,
        flag_100: bool,
        flag_200: bool,
        allowed: bool,
    ) -> Ptr<PlayerCharacter> {
        let this = player(e);
        e.register_double(0x0040_6d70, move |_, a| {
            int(if a[1] == 0x100 { flag_100 } else { flag_200 } as u32)
        });
        e.register_double(0x0095_2730, move |_, _| int(allowed as u32));
        e.register(0x0066_ee10, |_, _| int(0));
        e.register(0x0088_0850, |_, _| int(-4i32 as u32));
        e.register(0x0088_0890, |_, _| float(-2.5));
        stub(
            e,
            &[
                0x0094_c4f0,
                0x0070_4e10,
                0x0088_08f0,
                0x0066_ee50,
                0x00b0_04b8,
            ],
        );
        put_vtable(e, this.addr(), 0x0200_0000, &[(0x4b8, 0x00b0_04b8)]);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        this
    }

    #[test]
    fn fn_0093ae80_applies_the_integer_change_and_reports_it() {
        let mut e = engine();
        let this = damage_world(&mut e, false, false, true);
        e.call(0x0093_ae80, &args![this, 7u32, 5i32, 0x6000u32]);
        assert_eq!(called(&e, 0x0040_6d70), vec![vec![7, 0x100]]);
        assert_eq!(
            called(&e, 0x0088_0850),
            vec![vec![this.addr(), 7, 5, 0x6000]]
        );
        let record = &called(&e, 0x0094_c4f0)[0];
        assert_eq!(
            (record[0], record[1], record[2], f(record, 3), record[4]),
            (this.addr(), 0, 7, -4.0, 2)
        );
        assert_eq!(called(&e, 0x0070_4e10), vec![vec![7]]);
        assert_eq!(called(&e, 0x0088_08f0), vec![vec![this.addr(), 7, 0]]);
        let report = &called(&e, 0x0066_ee50)[0];
        assert_eq!(
            (report[0], report[1], f(report, 2), f(report, 3), report[4]),
            (this.addr() + 0xa4, 7, 0.0, -4.0, 0x6000 + 0xa4)
        );
        assert!(called(&e, 0x00b0_04b8).is_empty());
    }

    #[test]
    fn fn_0093ae80_tells_the_player_when_health_is_lost() {
        let mut e = engine();
        let this = damage_world(&mut e, false, false, true);
        e.call(0x0093_ae80, &args![this, 0x10u32, 5i32, 0x6000u32]);
        let call = &called(&e, 0x00b0_04b8)[0];
        assert_eq!((call[0], call[1], f(call, 2)), (this.addr(), 0x6000, -4.0));
    }

    #[test]
    fn fn_0093ae80_does_nothing_for_a_flagged_value() {
        let mut e = engine();
        let this = damage_world(&mut e, true, false, true);
        e.call(0x0093_ae80, &args![this, 7u32, 5i32, 0u32]);
        assert_eq!(call_addresses(&e), vec![0x0093_ae80, 0x0040_6d70]);
    }

    #[test]
    fn fn_0093ae80_reports_a_null_source_as_null() {
        let mut e = engine();
        let this = damage_world(&mut e, false, false, true);
        e.call(0x0093_ae80, &args![this, 7u32, 5i32, 0u32]);
        assert_eq!(called(&e, 0x0066_ee50)[0][4], 0);
    }

    #[test]
    fn fn_0093afb0_applies_the_float_change() {
        let mut e = engine();
        let this = damage_world(&mut e, false, false, true);
        e.call(0x0093_afb0, &args![this, 0x10u32, 1.25f32, 0x6000u32]);
        let delta = &called(&e, 0x0088_0890)[0];
        assert_eq!(
            (delta[0], delta[1], f(delta, 2), delta[3]),
            (this.addr(), 0x10, 1.25, 0x6000)
        );
        let record = &called(&e, 0x0094_c4f0)[0];
        assert_eq!((record[1], f(record, 3), record[4]), (0, -2.5, 2));
        assert_eq!(called(&e, 0x00b0_04b8).len(), 1);
    }

    #[test]
    fn fn_0093afb0_keeps_positive_changes_from_the_health_virtual() {
        let mut e = engine();
        let this = damage_world(&mut e, false, false, true);
        e.register(0x0088_0890, |_, _| float(2.5));
        e.call(0x0093_afb0, &args![this, 0x10u32, 1.25f32, 0u32]);
        assert!(called(&e, 0x00b0_04b8).is_empty());
        let report = &called(&e, 0x0066_ee50)[0];
        assert_eq!(f(report, 3), 2.5);
    }

    #[test]
    fn fn_0093afb0_does_nothing_for_a_flagged_value() {
        let mut e = engine();
        let this = damage_world(&mut e, true, false, true);
        e.call(0x0093_afb0, &args![this, 7u32, 1.0f32, 0u32]);
        assert_eq!(call_addresses(&e), vec![0x0093_afb0, 0x0040_6d70]);
    }

    #[test]
    fn fn_0093b0f0_checks_permission_and_uses_kind_one() {
        let mut e = engine();
        let this = damage_world(&mut e, false, false, true);
        e.call(0x0093_b0f0, &args![this, 7u32, 3i32, 0x6000u32]);
        let check = &called(&e, 0x0095_2730)[0];
        assert_eq!((check[0], check[1], f(check, 2)), (this.addr(), 7, 3.0));
        let record = &called(&e, 0x0094_c4f0)[0];
        assert_eq!((record[1], f(record, 3)), (1, -4.0));
    }

    #[test]
    fn fn_0093b0f0_stops_when_not_permitted_or_flagged() {
        let mut e = engine();
        let this = damage_world(&mut e, false, false, false);
        e.call(0x0093_b0f0, &args![this, 7u32, 3i32, 0u32]);
        assert_eq!(call_addresses(&e), vec![0x0093_b0f0, 0x0095_2730]);
        let mut e = engine();
        let this = damage_world(&mut e, true, false, true);
        e.call(0x0093_b0f0, &args![this, 7u32, 3i32, 0u32]);
        assert!(called(&e, 0x0088_0850).is_empty());
    }

    #[test]
    fn fn_0093b240_applies_the_float_change_with_kind_one() {
        let mut e = engine();
        let this = damage_world(&mut e, false, false, true);
        e.call(0x0093_b240, &args![this, 7u32, 0.5f32, 0x6000u32]);
        let check = &called(&e, 0x0095_2730)[0];
        assert_eq!(f(check, 2), 0.5);
        let record = &called(&e, 0x0094_c4f0)[0];
        assert_eq!((record[1], f(record, 3)), (1, -2.5));
    }

    #[test]
    fn fn_0093b240_stops_when_not_permitted_or_flagged() {
        let mut e = engine();
        let this = damage_world(&mut e, false, false, false);
        e.call(0x0093_b240, &args![this, 7u32, 0.5f32, 0u32]);
        assert_eq!(call_addresses(&e), vec![0x0093_b240, 0x0095_2730]);
        let mut e = engine();
        let this = damage_world(&mut e, true, false, true);
        e.call(0x0093_b240, &args![this, 7u32, 0.5f32, 0u32]);
        assert!(called(&e, 0x0088_0890).is_empty());
    }

    /// The world of the health setters `0093b3a0` and `0093b7a0`: the owner's
    /// virtual +0xC gives the health `before` (first call) and `after`
    /// (second call) as fractions of a maximum of 100, the two heartbeat
    /// settings are `low` and `high`, and the sound functions are recorded.
    fn health_world(
        e: &mut Engine,
        before: f32,
        after: f32,
        low: f32,
        high: f32,
    ) -> Ptr<PlayerCharacter> {
        let this = damage_world(e, false, false, true);
        let calls = Rc::new(RefCell::new(0u32));
        e.register_double(0x00b0_000c, move |_, _| {
            let mut n = calls.borrow_mut();
            *n += 1;
            float(if *n == 1 { before } else { after } as f64 * 100.0)
        });
        e.register_double(0x00b0_0020, |_, _| float(100.0));
        give_owner(
            e,
            this,
            0x0201_0000,
            &[(0xc, 0x00b0_000c), (0x20, 0x00b0_0020)],
        );
        e.map(0x0300_0000, 0x100);
        e.mem.set_f32(0x0300_0000, low);
        e.mem.set_f32(0x0300_0004, high);
        e.register(0x0040_3e20, |_, a| {
            int(if a[0] == HEARTBEAT_SETTING_LOW {
                0x0300_0000
            } else {
                0x0300_0004
            })
        });
        e.register(0x0045_3a70, |_, _| int(0x0400_0000));
        e.register(0x00ad_7550, |_, a| int(a[1]));
        stub(
            e,
            &[
                0x00ad_88f0,
                0x00ad_8d10,
                0x0041_8900,
                0x0048_3710,
                0x00ad_8830,
                0x00ad_8da0,
            ],
        );
        this
    }

    /// The sound the heartbeat switched to (the name passed to `00ad7550`).
    fn heartbeat_names(e: &Engine) -> Vec<u32> {
        called(e, 0x00ad_7550).iter().map(|w| w[2]).collect()
    }

    #[test]
    fn health_loss_to_nothing_fades_the_status_sound_out() {
        let mut e = engine();
        let this = health_world(&mut e, 0.5, 0.0, 0.8, 0.4);
        e.call(0x0093_b3a0, &args![this, 0x10u32, 5i32, 0x6000u32]);
        assert_eq!(
            called(&e, 0x00ad_8da0),
            vec![vec![this.addr() + 0x77c, 1000]]
        );
        assert!(heartbeat_names(&e).is_empty());
        // The loss itself: the interface, the player's virtual and the report.
        assert_eq!(called(&e, 0x00b0_04b8).len(), 1);
        assert_eq!(called(&e, 0x0088_08f0), vec![vec![this.addr(), 0x10, 0]]);
        let report = &called(&e, 0x0066_ee50)[0];
        assert_eq!((f(report, 2), f(report, 3)), (0.0, -4.0));
    }

    #[test]
    fn health_above_the_upper_threshold_fades_the_status_sound_out() {
        let mut e = engine();
        let this = health_world(&mut e, 0.5, 0.9, 0.8, 0.4);
        e.call(0x0093_b3a0, &args![this, 0x10u32, 5i32, 0u32]);
        assert_eq!(called(&e, 0x00ad_8da0).len(), 1);
        assert!(heartbeat_names(&e).is_empty());
    }

    #[test]
    fn falling_below_the_lower_threshold_starts_the_blp_heartbeat() {
        let mut e = engine();
        let this = health_world(&mut e, 0.9, 0.3, 0.8, 0.4);
        e.call(0x0093_b3a0, &args![this, 0x10u32, 5i32, 0u32]);
        assert_eq!(heartbeat_names(&e), vec![HEARTBEAT_SOUND_BLP]);
        let lookup = &called(&e, 0x00ad_7550)[0];
        assert_eq!((lookup[0], lookup[3]), (0x0400_0000, 0x31));
        let handle = this.addr() + 0x77c;
        assert_eq!(called(&e, 0x00ad_88f0), vec![vec![handle]]);
        assert_eq!(called(&e, 0x00ad_8d10), vec![vec![handle]]);
        assert_eq!(called(&e, 0x0041_8900), vec![vec![handle, lookup[1]]]);
        assert_eq!(called(&e, 0x0048_3710), vec![vec![lookup[1]]]);
        assert_eq!(called(&e, 0x00ad_8830), vec![vec![handle, 1]]);
        assert!(called(&e, 0x00ad_8da0).is_empty());
    }

    #[test]
    fn rising_above_the_lower_threshold_starts_the_alp_heartbeat() {
        let mut e = engine();
        let this = health_world(&mut e, 0.3, 0.6, 0.8, 0.4);
        e.call(0x0093_b3a0, &args![this, 0x10u32, 5i32, 0u32]);
        assert_eq!(heartbeat_names(&e), vec![HEARTBEAT_SOUND_ALP]);
    }

    #[test]
    fn rising_from_between_the_thresholds_changes_no_sound() {
        let mut e = engine();
        let this = health_world(&mut e, 0.5, 0.6, 0.8, 0.4);
        e.call(0x0093_b3a0, &args![this, 0x10u32, 5i32, 0u32]);
        assert!(heartbeat_names(&e).is_empty());
        assert!(called(&e, 0x00ad_8da0).is_empty());
    }

    #[test]
    fn rising_from_above_the_upper_threshold_while_inside_starts_the_alp_heartbeat() {
        let mut e = engine();
        let this = health_world(&mut e, 0.9, 0.6, 0.8, 0.4);
        e.call(0x0093_b3a0, &args![this, 0x10u32, 5i32, 0u32]);
        assert_eq!(heartbeat_names(&e), vec![HEARTBEAT_SOUND_ALP]);
    }

    #[test]
    fn other_values_leave_the_heartbeat_alone() {
        let mut e = engine();
        let this = health_world(&mut e, 0.5, 0.0, 0.8, 0.4);
        e.call(0x0093_b3a0, &args![this, 7u32, 5i32, 0x6000u32]);
        assert!(called(&e, 0x00ad_8da0).is_empty());
        assert!(called(&e, 0x00b0_04b8).is_empty());
        assert!(called(&e, 0x00b0_000c).is_empty());
        assert_eq!(called(&e, 0x0070_4e10), vec![vec![7]]);
    }

    #[test]
    fn fn_0093b3a0_records_the_flag_of_the_value() {
        let mut e = engine();
        let this = damage_world(&mut e, false, true, true);
        e.call(0x0093_b3a0, &args![this, 7u32, 5i32, 0u32]);
        let record = &called(&e, 0x0094_c4f0)[0];
        assert_eq!((record[1], record[2], record[4]), (2, 7, 1));
    }

    #[test]
    fn fn_0093b3a0_stops_when_not_permitted_or_flagged() {
        let mut e = engine();
        let this = damage_world(&mut e, false, false, false);
        e.call(0x0093_b3a0, &args![this, 0x10u32, 5i32, 0u32]);
        assert_eq!(call_addresses(&e), vec![0x0093_b3a0, 0x0095_2730]);
        let mut e = engine();
        let this = damage_world(&mut e, true, false, true);
        e.call(0x0093_b3a0, &args![this, 0x10u32, 5i32, 0u32]);
        assert!(called(&e, 0x0088_0850).is_empty());
    }

    #[test]
    fn fn_0093b7a0_follows_the_health_of_a_float_change() {
        let mut e = engine();
        let this = health_world(&mut e, 0.9, 0.3, 0.8, 0.4);
        e.call(0x0093_b7a0, &args![this, 0x10u32, 4.0f32, 0x6000u32]);
        let delta = &called(&e, 0x0088_0890)[0];
        assert_eq!((delta[1], f(delta, 2), delta[3]), (0x10, 4.0, 0x6000));
        let call = &called(&e, 0x00b0_04b8)[0];
        assert_eq!((call[1], f(call, 2)), (0x6000, -2.5));
        assert_eq!(heartbeat_names(&e), vec![HEARTBEAT_SOUND_BLP]);
        let record = &called(&e, 0x0094_c4f0)[0];
        assert_eq!((record[1], f(record, 3)), (2, -2.5));
    }

    #[test]
    fn fn_0093b7a0_stops_when_not_permitted_or_flagged() {
        let mut e = engine();
        let this = damage_world(&mut e, false, false, false);
        e.call(0x0093_b7a0, &args![this, 0x10u32, 4.0f32, 0u32]);
        assert_eq!(call_addresses(&e), vec![0x0093_b7a0, 0x0095_2730]);
        let mut e = engine();
        let this = damage_world(&mut e, true, false, true);
        e.call(0x0093_b7a0, &args![this, 0x10u32, 4.0f32, 0u32]);
        assert!(called(&e, 0x0088_0890).is_empty());
    }

    /// The globals of the cell-grid functions: the `TES` singleton pointer
    /// (pointing at 0x5000), the water search constants and the cell size.
    fn grid_globals(e: &mut Engine) {
        e.map(0x011d_e000, 0x1000);
        e.set_global(TES_SINGLETON, 0x5000u32);
        e.map(0x0101_6000, 0x3000);
        e.set_global(WATER_RADIUS_LIMIT, 2048.0f64);
        e.set_global(WATER_RADIUS_CLAMPED, 2048.0f32);
        e.set_global(CELL_SIZE, 4096.0f64);
    }

    /// The water-cell search's world: the player has no parent cell (or
    /// `parent`), stands at (`x`, `y`); `451900` returns a cell number from
    /// the grid coordinates (`1000 + 10x + y`) and `004518e0` accepts only
    /// `accepted`.
    fn water_world(
        e: &mut Engine,
        parent: u32,
        x: f32,
        y: f32,
        accepted: u32,
    ) -> Ptr<PlayerCharacter> {
        let this = player(e);
        grid_globals(e);
        e.register_double(0x008d_6f30, move |_, _| int(parent));
        e.register(0x0042_5fd0, |_, _| int(0));
        e.register_double(0x0045_18e0, move |_, a| int((a[0] == accepted) as u32));
        e.register(0x004f_d3e0, |_, _| int(1));
        let block = e.mem.alloc(0x40);
        e.mem.set_f32(block + 0x10, x);
        e.mem.set_f32(block + 0x14, y);
        e.register_double(0x0089_1170, move |_, _| int(block));
        e.register(0x0040_6d90, |_, a| int(f32::from_bits(a[0]) as i32 as u32));
        e.register(0x0045_2dc0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            int(a[0])
        });
        e.register(0x0045_1900, |_, a| int(1000 + 10 * a[1] + a[2]));
        e.register(0x0045_16d0, |_, _| int(0));
        this
    }

    #[test]
    fn water_cell_of_the_current_cell() {
        let mut e = engine();
        let this = water_world(&mut e, 0x3000, 4200.0, 4200.0, 0x3000);
        let cell = e.call(0x0093_bba0, &args![this, 500.0f32]).u32();
        assert_eq!(cell, 0x3000);
        assert!(called(&e, 0x0045_1900).is_empty());
    }

    #[test]
    fn water_cell_of_an_unloaded_current_cell_is_none() {
        let mut e = engine();
        let this = water_world(&mut e, 0x3000, 4200.0, 4200.0, 0);
        e.register(0x004f_d3e0, |_, _| int(0));
        let cell = e.call(0x0093_bba0, &args![this, 500.0f32]).u32();
        assert_eq!(cell, 0);
    }

    #[test]
    fn water_cell_searches_the_neighbours_toward_the_edges() {
        let mut e = engine();
        // Grid cell (1, 1); 4200 is within 500 of the cell's low edges.
        let this = water_world(&mut e, 0, 4200.0, 4200.0, 1000);
        let cell = e.call(0x0093_bba0, &args![this, 500.0f32]).u32();
        assert_eq!(cell, 1000);
        assert_eq!(
            called(&e, 0x0045_1900),
            vec![vec![0x5000, 0, 1], vec![0x5000, 1, 0], vec![0x5000, 0, 0]]
        );
        // The bounds of grid cell (1, 1) were built with the cell size.
        let bounds = called(&e, 0x0045_2dc0);
        assert_eq!(bounds[0][1..], [4096.0f32.to_bits(), 4096.0f32.to_bits()]);
        assert_eq!(bounds[1][1..], [8192.0f32.to_bits(), 8192.0f32.to_bits()]);
    }

    #[test]
    fn water_cell_stays_in_the_middle_of_a_cell() {
        let mut e = engine();
        let this = water_world(&mut e, 0, 6000.0, 6000.0, 1011);
        let cell = e.call(0x0093_bba0, &args![this, 500.0f32]).u32();
        // No neighbour is needed: nothing is looked up and nothing is accepted.
        assert_eq!(cell, 0);
        assert!(called(&e, 0x0045_1900).is_empty());
    }

    #[test]
    fn water_cell_toward_the_high_edge_steps_one_cell_up() {
        let mut e = engine();
        // 8000 is within 500 of the high edge (8192) of grid cell (1, 1).
        let this = water_world(&mut e, 0, 6000.0, 8000.0, 1012);
        let cell = e.call(0x0093_bba0, &args![this, 500.0f32]).u32();
        assert_eq!(cell, 1012);
        assert_eq!(called(&e, 0x0045_1900), vec![vec![0x5000, 1, 2]]);
    }

    #[test]
    fn water_cell_clamps_a_large_radius_and_falls_back_to_the_first_water_cell() {
        let mut e = engine();
        let this = water_world(&mut e, 0, 4200.0, 4200.0, 0x7777);
        let found = e.mem.alloc(4);
        e.mem.set_u32(found, 0x7777);
        e.register_double(0x0045_16d0, move |_, _| int(found));
        let cell = e.call(0x0093_bba0, &args![this, 3000.0f32]).u32();
        assert_eq!(cell, 0x7777);
        assert_eq!(called(&e, 0x0045_16d0), vec![vec![0x5000]]);
        // An unclamped search does not use the fallback.
        let mut e = engine();
        let this = water_world(&mut e, 0, 4200.0, 4200.0, 0x7777);
        e.register(0x0045_16d0, |_, _| int(0x1234));
        e.call(0x0093_bba0, &args![this, 500.0f32]);
        assert!(called(&e, 0x0045_16d0).is_empty());
    }

    #[test]
    fn water_cell_is_none_when_the_final_cell_is_not_accepted() {
        let mut e = engine();
        let this = water_world(&mut e, 0, 6000.0, 6000.0, 1);
        e.register(0x0045_1900, |_, _| int(0x4444));
        let cell = e.call(0x0093_bba0, &args![this, 500.0f32]).u32();
        assert_eq!(cell, 0);
    }

    /// Every function `PositionPlayer` calls that is outside this file.
    const POSITION_PLAYER_CALLEES: &[u32] = &[
        0x0040_3df0,
        0x0040_8840,
        0x0040_8da0,
        0x0041_6870,
        0x0042_5fd0,
        0x0042_ce10,
        0x0043_d410,
        0x0043_f8d0,
        0x0043_fa80,
        0x0044_0460,
        0x0044_ddc0,
        0x0045_0b80,
        0x0045_1530,
        0x0045_2480,
        0x0045_3550,
        0x0045_3a70,
        0x0045_3dc0,
        0x0045_4450,
        0x0045_6520,
        0x0045_7d70,
        0x0045_8200,
        0x0045_8400,
        0x0045_a750,
        0x0045_c670,
        0x0046_dd00,
        0x0048_3710,
        0x004a_0c90,
        0x004a_3e00,
        0x004b_aec0,
        0x004f_d3e0,
        0x0052_4ac0,
        0x0052_4c90,
        0x0052_6070,
        0x0052_60a0,
        0x0054_6c20,
        0x0054_74b0,
        0x0054_8210,
        0x0054_8230,
        0x0054_ca90,
        0x0054_ddd0,
        0x0055_47c0,
        0x0055_8310,
        0x0055_9450,
        0x0056_10f0,
        0x0057_5bb0,
        0x0057_5d70,
        0x0058_6170,
        0x005b_9e80,
        0x005c_53d0,
        0x005f_36f0,
        0x0063_d060,
        0x0066_52e0,
        0x0066_5610,
        0x0066_5860,
        0x0068_15c0,
        0x006f_ca90,
        0x006f_cdb0,
        0x0070_0960,
        0x0070_5990,
        0x0070_5e30,
        0x0070_ede0,
        0x0070_edf0,
        0x0077_1700,
        0x0077_2c30,
        0x0078_cfc0,
        0x007f_a200,
        0x007f_a310,
        0x0082_2b90,
        0x0082_4400,
        0x0083_04a0,
        0x0084_d030,
        0x0084_e3a0,
        0x0086_7e30,
        0x0086_d490,
        0x0087_1dc0,
        0x0088_85e0,
        0x008d_3550,
        0x008d_6f30,
        0x0092_f260,
        0x0094_ae40,
        0x0095_0a60,
        0x0095_2c30,
        0x0096_1f90,
        0x0097_3de0,
        0x0097_85d0,
        0x00a0_3c90,
        0x00a0_9030,
        0x00a5_9c60,
        0x00a5_a040,
        0x00ad_78b0,
        0x00ad_7940,
        0x00ad_8780,
        0x00b4_f5c0,
        0x00b5_fd60,
        0x00b6_55b0,
        0x00ec_17c0,
        0x00ec_1800,
        0x00ec_2320,
        0x00ec_408c,
    ];

    /// Maps the exe regions the position code reads (constants and globals)
    /// and stores the constants it compares with.
    fn map_position_globals(e: &mut Engine) {
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x0108_0000, 0x1_0000);
        e.map(0x011a_0000, 0x6_0000);
        e.map(0x0120_0000, 0x1_0000);
        e.map(0x0126_f000, 0x1000);
        e.set_global(PI_DOUBLE, std::f64::consts::PI);
        e.set_global(TWO_PI_DOUBLE, 2.0 * std::f64::consts::PI);
        e.set_global(NEGATIVE_PI_DOUBLE, -std::f64::consts::PI);
        e.set_global(LAND_CLEARANCE, 32.0f64);
        e.set_global(SETTLE_LIMIT, 0.1f64);
        e.set_global(SETTLE_TIME_STEP, 0.16f32);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(IDENTITY_MATRIX, 0x3f80_0000u32);
        e.set_global(TES_SINGLETON, 0x5000u32);
        e.set_global(GLOBAL_OBJECT_0042CE10, 0x9900u32);
    }

    /// Registers a double over `addr` that records, at each call, the `count`
    /// floats the pointer argument number `index` points to (the frame
    /// blocks are gone after the call).
    fn watch_floats(
        e: &mut Engine,
        addr: u32,
        index: usize,
        count: u32,
    ) -> Rc<RefCell<Vec<Vec<f32>>>> {
        let seen = Rc::new(RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(addr, move |e, a| {
            let floats = (0..count).map(|i| e.mem.f32(a[index] + 4 * i)).collect();
            log.borrow_mut().push(floats);
            Ret::default()
        });
        seen
    }

    /// The player and the position block its virtual `+0x1F4` returns.
    struct PositionWorld {
        this: Ptr<PlayerCharacter>,
        /// The position the player reports.
        position: u32,
    }

    /// The world of `PositionPlayer`: every callee is a stub except those
    /// that must produce something usable (`NiMatrix3` products and the
    /// vector constructor return their output block).
    fn position_world(e: &mut Engine) -> PositionWorld {
        map_position_globals(e);
        let this = player(e);
        stub(e, POSITION_PLAYER_CALLEES);
        e.register(0x0043_f8d0, |_, a| int(a[1]));
        e.register(0x0041_6870, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            int(a[0])
        });
        e.set_global(PLAYER_SINGLETON, this.addr());
        let position = e.mem.alloc(16);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 100.0);
        for slot in [0x1d0u32, 0x2a8, 0x3f4] {
            stub(e, &[0x00b0_0000 + slot]);
        }
        e.register_double(0x00b0_01f4, move |_, _| int(position));
        e.register(0x00b0_02bc, |_, _| float(0.5));
        put_vtable(
            e,
            this.addr(),
            0x0200_0000,
            &[
                (0x1d0, 0x00b0_01d0),
                (0x1f4, 0x00b0_01f4),
                (0x2a8, 0x00b0_02a8),
                (0x2bc, 0x00b0_02bc),
                (0x3f4, 0x00b0_03f4),
            ],
        );
        PositionWorld { this, position }
    }

    /// Calls `PositionPlayer` with position (10, 20, 30), rotation
    /// (`x`, `y`, `z`) and the given cell.
    fn position_player(
        e: &mut Engine,
        world: &PositionWorld,
        rotation: [f32; 3],
        cell: u32,
        weather: bool,
    ) {
        e.call(
            0x0093_c200,
            &args![
                world.this,
                10.0f32,
                20.0f32,
                30.0f32,
                rotation[0],
                rotation[1],
                rotation[2],
                cell,
                weather
            ],
        );
    }

    #[test]
    fn position_player_into_an_interior_cell() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0042_5fd0, |_, _| int(1));
        e.register(0x0054_74b0, |_, _| int(0x7700));
        let grid_position = watch_floats(&mut e, 0x0045_3dc0, 2, 3);
        let placed_at = watch_floats(&mut e, 0x00b0_02a8, 1, 3);
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        let this = world.this;
        assert_eq!(
            e.get(this, PlayerCharacter::pLastKnownMusicType).addr(),
            0x7700
        );
        // Placed into the cell with the position and rotation in the frame.
        let placed = called(&e, 0x0057_5bb0);
        assert_eq!(placed[0][1], 0x6000);
        assert_eq!(placed[1], vec![this.addr(), 0]);
        let grid = &called(&e, 0x0045_3dc0)[0];
        assert_eq!((grid[0], grid[1]), (0x5000, 0x6000));
        assert_eq!(grid_position.borrow()[0], vec![10.0, 20.0, 30.0]);
        // The player was moved to the position first.
        assert_eq!(placed_at.borrow()[0], vec![10.0, 20.0, 30.0]);
        assert_eq!(called(&e, 0x0054_8230), vec![vec![0x6000, this.addr(), 0]]);
        assert_eq!(called(&e, 0x00ad_8780).len(), 1);
        assert_eq!(called(&e, 0x00ad_8780)[0][1], 0x1000);
        // The part for cells that are not interior is skipped.
        assert!(called(&e, 0x0045_4450).is_empty());
        assert!(called(&e, 0x0045_8200).is_empty());
        assert!(called(&e, 0x0046_dd00).is_empty());
    }

    #[test]
    fn position_player_into_an_exterior_cell_clears_the_canopy_shadows() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0054_ddd0, |_, _| int(0x8800));
        e.register(0x0044_ddc0, |_, _| int(0x9100));
        let tail_position = watch_floats(&mut e, 0x0045_4450, 1, 3);
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert_eq!(called(&e, 0x0066_5860).len(), 1);
        assert_eq!(called(&e, 0x004b_aec0), vec![vec![0x9100]]);
        assert_eq!(called(&e, 0x0045_8200), vec![vec![0x5000, 0x8800]]);
        assert_eq!(called(&e, 0x0045_4450)[0][0], 0x5000);
        assert_eq!(tail_position.borrow()[0], vec![10.0, 20.0, 30.0]);
        assert!(called(&e, 0x0045_3dc0).is_empty());
    }

    #[test]
    fn position_player_keeps_the_canopy_shadows_of_a_world_with_them() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0045_2480, |_, _| int(1));
        e.register(0x0054_ddd0, |_, _| int(0x8800));
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        e.register_double(0x0054_8210, move |_, _| int(flag));
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert_eq!(called(&e, 0x0054_8210), vec![vec![0x8800]]);
        assert_eq!(called(&e, 0x0066_5610).len(), 1);
        assert!(called(&e, 0x0066_5860).is_empty());
        assert_eq!(called(&e, 0x0045_8200), vec![vec![0x5000, 0x8800]]);
    }

    #[test]
    fn position_player_without_a_cell_only_places_the_player() {
        let mut e = engine();
        let world = position_world(&mut e);
        position_player(&mut e, &world, [0.0; 3], 0, false);
        assert!(called(&e, 0x0054_74b0).is_empty());
        assert!(called(&e, 0x0045_8200).is_empty());
        assert_eq!(called(&e, 0x0045_4450).len(), 1);
        assert_eq!(called(&e, 0x0057_5bb0)[0], vec![world.this.addr(), 0]);
    }

    #[test]
    fn position_player_resets_the_weather_when_asked() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0046_dd00, |_, _| int(0xaa00));
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert!(called(&e, 0x0063_d060).is_empty());
        position_player(&mut e, &world, [0.0; 3], 0x6000, true);
        assert_eq!(called(&e, 0x0063_d060), vec![vec![0xaa00]]);
    }

    #[test]
    fn position_player_in_move_mode_dispels_a_spell_target() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x005f_36f0, |_, _| int(1));
        e.register(0x0082_2b90, |_, _| int(1));
        e.register(0x0070_5990, |_, _| int(0xbb00));
        e.mem.set_u32(GLOBAL_RETURNED_BY_0093CCD0, 0x4000);
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        let magic = world.this.addr() + 0x94;
        assert_eq!(called(&e, 0x0082_2b90), vec![vec![magic, 0x4018, 1]]);
        assert_eq!(
            called(&e, 0x007f_a310),
            vec![vec![0xbb00, 1, 0, 1], vec![0xbb00, 0, 0, 1]]
        );
        assert_eq!(called(&e, 0x0082_4400), vec![vec![magic, 0x4018, 0, 0]]);
    }

    #[test]
    fn position_player_in_move_mode_leaves_other_targets_alone() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x005f_36f0, |_, _| int(1));
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        let magic = world.this.addr() + 0x94;
        // No object from `0093ccd0`: the null target is passed.
        assert_eq!(called(&e, 0x0082_2b90), vec![vec![magic, 0, 1]]);
        assert!(called(&e, 0x0082_4400).is_empty());
    }

    #[test]
    fn position_player_switches_the_encounter_zones() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0054_6c20, |_, a| int(a[0] + 1));
        e.register(0x008d_6f30, |_, _| int(0x3300));
        e.register(0x0086_7e30, |_, _| int(0x1111));
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert_eq!(called(&e, 0x0052_6070), vec![vec![0x3301, 0x1111]]);
        assert_eq!(called(&e, 0x0052_60a0), vec![vec![0x6001, 0x1111]]);
        assert_eq!(called(&e, 0x0086_7e30), vec![vec![CALENDAR_OBJECT]]);
        // The player leaves its old parent cell.
        assert_eq!(
            called(&e, 0x0054_ca90),
            vec![vec![0x3300, world.this.addr()]]
        );
    }

    #[test]
    fn position_player_keeps_equal_encounter_zones() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0054_6c20, |_, _| int(5));
        e.register(0x008d_6f30, |_, _| int(0x3300));
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert!(called(&e, 0x0086_7e30).is_empty());
        assert!(called(&e, 0x0052_6070).is_empty());
    }

    #[test]
    fn position_player_normalizes_the_heading() {
        let tau = 2.0 * std::f64::consts::PI;
        for (heading, expected) in [
            (7.0f32, (7.0f32 as f64 - tau) as f32),
            (-7.0, (-7.0f32 as f64 + tau) as f32),
            (1.0, 1.0),
        ] {
            let mut e = engine();
            let world = position_world(&mut e);
            position_player(&mut e, &world, [0.0, 0.0, heading], 0x6000, false);
            let call = &called(&e, 0x004a_0c90)[0];
            assert_eq!(f(call, 1), expected);
        }
    }

    #[test]
    fn position_player_tilts_by_pitch_and_roll() {
        for (heading, expected) in [(1.0f32, 0.375f32), (-1.0, 0.625)] {
            let mut e = engine();
            let world = position_world(&mut e);
            position_player(&mut e, &world, [0.5, 0.125, heading], 0x6000, false);
            let call = &called(&e, 0x0052_4ac0)[0];
            assert_eq!(f(call, 1), expected);
        }
    }

    #[test]
    fn position_player_builds_the_root_transform_and_listener() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0045_3a70, |_, _| int(0xaa00));
        e.register(0x005b_9e80, |_, a| {
            float(f32::from_bits(a[0]) as f64 + 10.0)
        });
        e.register(0x005c_53d0, |_, a| {
            float(f32::from_bits(a[0]) as f64 + 20.0)
        });
        e.register(0x0045_c670, |_, _| int(0x1230));
        e.register(0x0055_8310, |_, a| int(a[0] + 1));
        let translated = watch_floats(&mut e, 0x0044_0460, 1, 3);
        e.set_global(LISTENER_UP_VECTOR, 1.0f32);
        e.set_global(LISTENER_UP_VECTOR + 4, 2.0f32);
        e.set_global(LISTENER_UP_VECTOR + 8, 3.0f32);
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        // The matrix products: player matrix times the turn, then the turn
        // times the result.
        assert_eq!(called(&e, 0x0043_f8d0).len(), 2);
        // The root node gets the matrix and the position, then an update.
        let rotate = &called(&e, 0x0043_fa80);
        assert_eq!(rotate.last().unwrap()[0], 0x1231);
        assert_eq!(called(&e, 0x0044_0460)[0][0], 0x1231);
        assert_eq!(translated.borrow()[0], vec![10.0, 20.0, 30.0]);
        let update = &called(&e, 0x0043_d410)[0];
        assert_eq!((f(update, 1), update[2], update[3]), (0.0, 0, 0));
        assert_eq!(called(&e, 0x00a5_9c60)[0][0], 0x1231);
        // The listener: the position, then the forward vector (sin and cos
        // of the heading from the player's virtual) and the up constant.
        assert_eq!(
            called(&e, 0x00ad_78b0),
            vec![vec![
                0xaa00,
                10.0f32.to_bits(),
                20.0f32.to_bits(),
                30.0f32.to_bits()
            ]]
        );
        let orient = &called(&e, 0x00ad_7940)[0];
        assert_eq!(orient[0], 0xaa00);
        assert_eq!(f(orient, 1), 20.5);
        assert_eq!(f(orient, 2), 10.5);
        assert_eq!(f(orient, 3), 0.0);
        assert_eq!((f(orient, 4), f(orient, 5), f(orient, 6)), (1.0, 2.0, 3.0));
    }

    #[test]
    fn position_player_updates_the_3d_node_of_the_player() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register_double(0x00b0_01d0, |_, _| int(0x4400));
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        // The player's own node gets the turn matrix, once, and the
        // fader is not started.
        assert_eq!(called(&e, 0x0043_fa80)[0][0], 0x4400);
        assert!(called(&e, 0x0070_0960).is_empty());
    }

    #[test]
    fn position_player_starts_the_fader_for_a_player_without_3d() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0045_6520, |_, _| int(0));
        e.set_global(FADER_MANAGER, 0x6600u32);
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        let fader = &called(&e, 0x0070_0960)[0];
        assert_eq!(
            (fader[0], fader[1], f(fader, 2), fader[3]),
            (0x6600, 1, 0.0, 0)
        );
        // Not when the other test says no.
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0042_ce10, |_, _| int(1));
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert!(called(&e, 0x0070_0960).is_empty());
    }

    #[test]
    fn position_player_closes_the_loading_menu_and_deletes_the_menu() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0070_edf0, |_, _| int(1));
        e.register(0x0070_ede0, |_, _| int(0x1500));
        e.register(0x00a0_9030, |_, a| int(a[0] + 1));
        e.register_double(0x00b0_0000, |_, _| int(0));
        let menu = e.mem.alloc(16);
        put_vtable(&mut e, menu, 0x0220_0000, &[(0, 0x00b0_0000)]);
        e.register_double(0x00a0_3c90, move |_, _| int(menu));
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert_eq!(called(&e, 0x0070_5e30).len(), 1);
        assert_eq!(called(&e, 0x00a0_9030), vec![vec![0x1500]]);
        assert_eq!(called(&e, 0x00a0_3c90), vec![vec![0x1501]]);
        assert_eq!(called(&e, 0x00b0_0000), vec![vec![menu, 1]]);
    }

    #[test]
    fn position_player_updates_terrain_and_the_hud_with_the_world_space() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0057_5d70, |_, _| int(0x3100));
        e.register(0x0058_6170, |_, a| int(a[0] + 1));
        e.register(0x0040_8da0, |_, a| int(a[0] + 2));
        let update_position = watch_floats(&mut e, 0x006f_ca90, 1, 3);
        let morph_position = watch_floats(&mut e, 0x006f_cdb0, 1, 3);
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        let update = &called(&e, 0x006f_ca90)[0];
        assert_eq!((update[0], update[2]), (0x3101, 0x0f));
        assert_eq!(update_position.borrow()[0], vec![1.0, 2.0, 100.0]);
        assert_eq!(called(&e, 0x006f_cdb0)[0][0], 0x3101);
        assert_eq!(morph_position.borrow()[0], vec![1.0, 2.0, 100.0]);
        // The HUD text is built from the singleton's world space + 0x18.
        assert_eq!(called(&e, 0x0040_8da0), vec![vec![0x3118]]);
        assert_eq!(called(&e, 0x0077_2c30), vec![vec![0x311a]]);
        assert_eq!(called(&e, 0x0097_3de0), vec![vec![PROCESS_LISTS]]);
    }

    #[test]
    fn position_player_without_a_world_space_has_no_terrain_or_text() {
        let mut e = engine();
        let world = position_world(&mut e);
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert!(called(&e, 0x006f_ca90).is_empty());
        assert!(called(&e, 0x0040_8da0).is_empty());
        assert_eq!(called(&e, 0x0077_2c30), vec![vec![0]]);
    }

    #[test]
    fn position_player_plays_the_movie_when_the_loading_screen_flag_is_set() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.mem.set_u8(LOADING_SCREEN_FLAG, 1);
        e.register(0x0040_3df0, |_, _| int(0x7e00));
        e.set_global(MOVIE_PLAYER, 0x6300u32);
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert_eq!(called(&e, 0x0045_7d70), vec![vec![0x5000, 0, 0, 0]]);
        assert_eq!(called(&e, 0x0077_1700), vec![vec![0x18], vec![0x0c]]);
        assert_eq!(called(&e, 0x0083_04a0).len(), 1);
        assert_eq!(called(&e, 0x0040_3df0), vec![vec![MOVIE_OBJECT]]);
        assert_eq!(
            called(&e, 0x00ec_2320),
            vec![vec![0x6300, 0x7e00, 1, 1, 0, 1, 1, 0, 1, 1]]
        );
    }

    #[test]
    fn position_player_resumes_the_controller_after_a_sequence() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x00ec_17c0, |_, _| int(1));
        e.set_global(MOVIE_PLAYER, 0x6300u32);
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert_eq!(called(&e, 0x00ec_1800), vec![vec![0x6300]]);
        let mut e = engine();
        let world = position_world(&mut e);
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert!(called(&e, 0x00ec_1800).is_empty());
    }

    /// The first-person settling world: the player has a first-person 3D
    /// (the `NiPointer` content getter answers for the member), the other
    /// test (`0042ce10`) is false and `Move` changes the height reported by
    /// the player by `fall` each call.
    fn settling_world(e: &mut Engine, fall: f32) -> PositionWorld {
        let world = position_world(e);
        let member_3d = world.this.addr() + 0x694;
        e.register_double(0x0055_9450, move |_, a| int((a[0] == member_3d) as u32));
        e.register(0x0040_8840, |_, a| float(f32::from_bits(a[0]).abs() as f64));
        let position = world.position;
        e.register_double(0x0092_f260, move |e, _| {
            let z = e.mem.f32(position + 8);
            e.mem.set_f32(position + 8, z + fall);
            Ret::default()
        });
        world
    }

    #[test]
    fn position_player_settles_a_player_that_stops_falling() {
        let mut e = engine();
        let world = settling_world(&mut e, -0.01);
        let vectors = Rc::new(RefCell::new(vec![]));
        let log = vectors.clone();
        let position = world.position;
        e.register_double(0x0092_f260, move |e, a| {
            log.borrow_mut().push(vec![
                e.mem.f32(a[2]),
                e.mem.f32(a[2] + 4),
                e.mem.f32(a[2] + 8),
            ]);
            let z = e.mem.f32(position + 8);
            e.mem.set_f32(position + 8, z - 0.01);
            Ret::default()
        });
        let placed_at = watch_floats(&mut e, 0x00b0_02a8, 1, 3);
        position_player(&mut e, &world, [0.0; 3], 0, false);
        // One Move: the first fall is already below 0.1.
        let moves = called(&e, 0x0092_f260);
        assert_eq!(moves.len(), 1);
        assert_eq!((moves[0][0], f(&moves[0], 1)), (world.this.addr(), 0.16));
        assert_eq!(moves[0][3], 0);
        // The move vector is the zero vector.
        assert_eq!(vectors.borrow()[0], vec![0.0, 0.0, 0.0]);
        // Settled: the player is not put back (only the first placement).
        assert_eq!(placed_at.borrow().len(), 1);
    }

    #[test]
    fn position_player_puts_a_player_that_keeps_falling_back_after_100_moves() {
        let mut e = engine();
        let world = settling_world(&mut e, -1.0);
        let placed_at = watch_floats(&mut e, 0x00b0_02a8, 1, 3);
        position_player(&mut e, &world, [0.0; 3], 0, false);
        assert_eq!(called(&e, 0x0092_f260).len(), 100);
        let sets = placed_at.borrow();
        assert_eq!(sets.len(), 2);
        // The start position (z 100 before the moves) is restored.
        assert_eq!(sets[1], vec![1.0, 2.0, 100.0]);
    }

    #[test]
    fn position_player_lifts_a_player_below_the_land() {
        let mut e = engine();
        let world = settling_world(&mut e, 0.0);
        e.register(0x0055_47c0, |e, a| {
            e.mem.set_f32(a[2], 90.0);
            int(1)
        });
        let placed_at = watch_floats(&mut e, 0x00b0_02a8, 1, 3);
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        // The land is at 90 and the clearance 32: the player (at 100) is
        // below 122 and is lifted to it.
        let sets = placed_at.borrow();
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[1], vec![1.0, 2.0, 122.0]);
        let land = &called(&e, 0x0055_47c0)[0];
        assert_eq!(land[0], 0x6000);
    }

    #[test]
    fn position_player_leaves_a_player_above_the_land() {
        let mut e = engine();
        let world = settling_world(&mut e, 0.0);
        e.register(0x0055_47c0, |e, a| {
            e.mem.set_f32(a[2], 10.0);
            int(1)
        });
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert_eq!(called(&e, 0x00b0_02a8).len(), 1);
    }

    #[test]
    fn position_player_does_not_look_for_land_in_an_interior_cell() {
        let mut e = engine();
        let world = settling_world(&mut e, 0.0);
        e.register(0x0042_5fd0, |_, _| int(1));
        position_player(&mut e, &world, [0.0; 3], 0x6000, false);
        assert!(called(&e, 0x0055_47c0).is_empty());
    }

    #[test]
    fn position_player_refreshes_the_animations_and_the_camera() {
        let mut e = engine();
        let world = settling_world(&mut e, 0.0);
        e.register(0x0095_0a60, |_, a| int(0x1000 + a[1]));
        e.register(0x0042_ce10, |_, _| int(1));
        e.mem.set_u8(world.this.addr() + 0x64a, 0);
        position_player(&mut e, &world, [0.0; 3], 0, false);
        // The third-person flag is toggled around each refresh and ends as
        // it started.
        assert_eq!(e.mem.u8(world.this.addr() + 0x64a), 0);
        assert_eq!(
            called(&e, 0x0095_0a60),
            vec![vec![world.this.addr(), 0], vec![world.this.addr(), 1]]
        );
        let updates = called(&e, 0x0088_85e0);
        assert_eq!(updates[0][..2], [world.this.addr(), 0x1000]);
        assert_eq!(updates[1][..2], [world.this.addr(), 0x1001]);
        assert_eq!(called(&e, 0x008d_3550).len(), 2);
        assert_eq!(called(&e, 0x0094_ae40), vec![vec![world.this.addr(), 0, 0]]);
        // No settling when the other test says so.
        assert!(called(&e, 0x0092_f260).is_empty());
    }

    #[test]
    fn position_player_updates_the_camera_rigid_body() {
        let mut e = engine();
        let world = position_world(&mut e);
        let member = world.this.addr() + 0xdec;
        e.register_double(0x0055_9450, move |_, a| {
            int(if a[0] == member { 0x7a00 } else { 0 })
        });
        let camera_position = watch_floats(&mut e, 0x004a_3e00, 1, 3);
        position_player(&mut e, &world, [0.0; 3], 0, false);
        let matrix = &called(&e, 0x004a_3e00)[0];
        let body = &called(&e, 0x0056_10f0)[0];
        assert_eq!(body[0], 0x7a00);
        assert_eq!(body[1], matrix[1]);
        assert_eq!(camera_position.borrow()[0], vec![1.0, 2.0, 100.0]);
    }

    #[test]
    fn position_player_updates_the_transfer_state() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0052_4c90, |_, _| int(0x5100));
        e.register(0x0084_d030, |_, _| float(2.5));
        e.register(0x00b4_f5c0, |_, _| int(0x6200));
        position_player(&mut e, &world, [0.0; 3], 0, false);
        assert_eq!(called(&e, 0x0066_52e0), vec![vec![0x5100, 0]]);
        assert_eq!(called(&e, 0x0084_d030), vec![vec![GAME_TIMER]]);
        let fade = &called(&e, 0x0045_3550)[0];
        assert_eq!((fade[0], f(fade, 1)), (0x5000, 2.5));
        assert_eq!(called(&e, 0x00b6_55b0), vec![vec![0x6200]]);
        assert_eq!(called(&e, 0x0078_cfc0).len(), 1);
        assert_eq!(
            called(&e, 0x0095_2c30),
            vec![vec![world.this.addr(), world.this.addr()]]
        );
        assert_eq!(called(&e, 0x0096_1f90), vec![vec![world.this.addr()]]);
        assert_eq!(called(&e, 0x0045_a750), vec![vec![0x5000]]);
    }

    #[test]
    fn position_player_does_not_suspend_loading_when_the_other_test_says_so() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0042_ce10, |_, _| int(1));
        position_player(&mut e, &world, [0.0; 3], 0, false);
        assert!(called(&e, 0x0078_cfc0).is_empty());
    }

    #[test]
    fn position_player_renders_the_menu_background_when_the_grid_cannot_take_it() {
        let mut e = engine();
        let world = position_world(&mut e);
        e.set_global(MAIN_SINGLETON, 0x8100u32);
        position_player(&mut e, &world, [0.0; 3], 0, false);
        assert_eq!(called(&e, 0x0087_1dc0), vec![vec![0x8100]]);
        assert_eq!(called(&e, 0x0097_85d0), vec![vec![PROCESS_LISTS]]);
        let mut e = engine();
        let world = position_world(&mut e);
        e.register(0x0045_1530, |_, _| int(1));
        position_player(&mut e, &world, [0.0; 3], 0, false);
        assert!(called(&e, 0x0087_1dc0).is_empty());
    }

    /// Every function `FastTravel` calls that is outside this file.
    const FAST_TRAVEL_CALLEES: &[u32] = &[
        0x0040_37b0,
        0x0040_37d0,
        0x0040_6d00,
        0x0040_6d50,
        0x0040_8d60,
        0x0040_fba0,
        0x0040_fbf0,
        0x0043_0830,
        0x0045_39a0,
        0x0045_7d70,
        0x0045_9870,
        0x0045_aee0,
        0x0046_dd00,
        0x0048_3710,
        0x0048_39c0,
        0x004f_f7e0,
        0x0050_2670,
        0x0052_6ac0,
        0x0055_9450,
        0x0057_5d70,
        0x005b_5e40,
        0x005e_58f0,
        0x0063_e8f0,
        0x0065_0a30,
        0x0065_2110,
        0x0068_15c0,
        0x006d_4eb0,
        0x006d_cd70,
        0x0082_5c00,
        0x0085_0a40,
        0x0086_7950,
        0x0086_7a40,
        0x0087_8160,
        0x0087_8200,
        0x0087_8250,
        0x0088_4eb0,
        0x0088_b0a0,
        0x0088_b510,
        0x0088_d640,
        0x008d_0500,
        0x008d_6f30,
        0x0093_06d0,
        0x0096_1f90,
        0x0096_b050,
        0x0096_b470,
        0x0096_b810,
        0x0096_bcd0,
        0x0096_d490,
        0x0096_d4b0,
        0x0096_db30,
        0x0096_df40,
        0x0096_eb40,
        0x0097_2d30,
        0x0097_3ee0,
        0x0097_5d10,
        0x009e_a3b0,
        0x00aa_7030,
    ];

    /// The functions `PositionPlayerExterior` and `ClearCellsIfTargetNotLoaded`
    /// call outside this file (besides those in the lists above).
    const EXTERIOR_PLACEMENT_CALLEES: &[u32] = &[
        0x0045_11e0,
        0x0045_4af0,
        0x0046_1330,
        0x0052_8540,
        0x0054_ca90,
        0x0058_5b30,
        0x0058_75a0,
        0x0095_0bb0,
        0x0096_11e0,
        0x0096_d810,
    ];

    /// The world of `FastTravel`: the position world (the final placement
    /// runs `PositionPlayer`), the callees above as stubs, the player's
    /// virtual `+0x358` false and the destination object reporting the
    /// position (7, 8, 9).
    fn fast_travel_world(e: &mut Engine) -> (PositionWorld, u32) {
        let world = exterior_world(e);
        e.set_global(SECONDS_PER_HOUR_DOUBLE, 3600.0f64);
        e.set_global(SECONDS_PER_HOUR_FLOAT, 3600.0f32);
        e.set_global(TWO_FLOAT, 2.0f32);
        e.set_global(NO_PATH_LENGTH, f32::MAX as f64);
        register_ftol(e);
        e.register_double(0x00b0_0358, |_, _| int(0));
        e.mem.set_u32(0x0200_0000 + 0x358, 0x00b0_0358);
        let setting = e.mem.alloc(8);
        e.register_double(0x0040_8d60, move |_, _| int(setting));
        // The destination: an object with a virtual +0x1F4 giving its position.
        let destination = e.mem.alloc(16);
        let spot = e.mem.alloc(16);
        for (i, v) in [7.0f32, 8.0, 9.0].into_iter().enumerate() {
            e.mem.set_f32(spot + 4 * i as u32, v);
        }
        e.register_double(0x00b4_01f4, move |_, _| int(spot));
        put_vtable(e, destination, 0x0230_0000, &[(0x1f4, 0x00b4_01f4)]);
        // The destination's rotation (4, 5, 6).
        let rotation = e.mem.alloc(16);
        for (i, v) in [4.0f32, 5.0, 6.0].into_iter().enumerate() {
            e.mem.set_f32(rotation + 4 * i as u32, v);
        }
        e.register_double(0x0043_0830, move |_, _| int(rotation));
        (world, destination)
    }

    #[test]
    fn fast_travel_does_nothing_when_the_entry_point_leaves_no_delay() {
        let mut e = engine();
        let this = player(&mut e);
        e.map(0x0101_0000, 0x2_0000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        stub(&mut e, &[0x005e_58f0]);
        e.register_double(0x00b0_0358, |_, _| int(1));
        put_vtable(&mut e, this.addr(), 0x0200_0000, &[(0x358, 0x00b0_0358)]);
        e.call(0x0093_cdf0, &args![this, 0x7000u32]);
        assert_eq!(
            call_addresses(&e),
            vec![0x0093_cdf0, 0x005e_58f0, 0x00b0_0358]
        );
        let entry = &called(&e, 0x005e_58f0)[0];
        assert_eq!((entry[0], entry[1]), (0x33, this.addr()));
    }

    /// Runs `FastTravel` in the fast-travel world with the given path
    /// length and run speed; the multi-bound radius is 2.0, the calendar
    /// scale 20 and the clock 1000.
    fn run_fast_travel(e: &mut Engine, length: f32, speed: f32) -> (PositionWorld, u32) {
        run_fast_travel_with(e, length, speed, |_| {})
    }

    /// [`run_fast_travel`] with `setup` run after the world is built (so it
    /// can replace the stubs).
    fn run_fast_travel_with(
        e: &mut Engine,
        length: f32,
        speed: f32,
        setup: impl FnOnce(&mut Engine),
    ) -> (PositionWorld, u32) {
        let (world, destination) = fast_travel_world(e);
        e.register_double(0x006d_4eb0, move |_, _| float(length as f64));
        e.register_double(0x0088_4eb0, move |_, _| float(speed as f64));
        e.register(0x0052_6ac0, |_, _| float(2.0));
        e.register(0x0086_7950, |_, _| float(20.0));
        e.register(0x0096_d490, |_, _| float(1000.0));
        e.register(0x0057_5d70, |_, _| int(0x7300));
        e.register(0x0058_75a0, |_, _| int(0x7400));
        e.register(0x0045_11e0, |_, _| int(1));
        e.set_global(PLAYER_SINGLETON, world.this.addr());
        setup(e);
        e.call(0x0093_cdf0, &args![world.this, destination]);
        (world, destination)
    }

    #[test]
    fn fast_travel_advances_the_clock_hour_by_hour() {
        let mut e = engine();
        // 36000 units at speed 10 is 3600 seconds, an hour, times the
        // radius 2.0: two hours.
        let (world, destination) = run_fast_travel(&mut e, 36000.0, 10.0);
        let this = world.this;
        // Two hours of 3600 / 20 = 180 time units.
        let clock = called(&e, 0x0096_d4b0);
        assert_eq!(clock.len(), 2);
        assert_eq!((clock[0][0], f(&clock[0], 1)), (PROCESS_LISTS, 1180.0));
        let calendar = called(&e, 0x0086_7a40);
        assert_eq!(calendar.len(), 2);
        assert_eq!(
            (calendar[0][0], f(&calendar[0], 1)),
            (CALENDAR_OBJECT, 180.0)
        );
        assert_eq!(called(&e, 0x0040_fbf0), vec![vec![0x011f_11a0, 0]; 2]);
        assert_eq!(called(&e, 0x0040_fba0), vec![vec![0x011f_11a0]; 2]);
        for address in [0x0096_bcd0, 0x0096_b810, 0x0096_b470, 0x0096_b050] {
            let calls = called(&e, address);
            assert_eq!(calls.len(), 2);
            assert_eq!(
                (calls[0][0], f(&calls[0], 1), calls[0][2]),
                (PROCESS_LISTS, 3600.0, 1)
            );
        }
        let simulate = &called(&e, 0x0096_db30)[0];
        assert_eq!((f(simulate, 1), simulate[2], simulate[3]), (3600.0, 1, 0));
        assert_eq!(called(&e, 0x0096_eb40).len(), 2);
        // The player is healed each hour (with 2.0).
        let heal = called(&e, 0x0088_b510);
        assert_eq!(heal.len(), 2);
        assert_eq!((heal[0][0], f(&heal[0], 1)), (this.addr(), 2.0));
        assert_eq!(e.get(this, PlayerCharacter::iSleepTime), 0);
        assert!(!e.get(this, PlayerCharacter::bIsSleeping));
        let _ = destination;
    }

    #[test]
    fn fast_travel_advances_a_fractional_trip_at_once() {
        let mut e = engine();
        // 9000 units at speed 10 is 900 seconds: a quarter hour times the
        // radius 2.0 is half an hour: no whole hours.
        let (world, _) = run_fast_travel(&mut e, 9000.0, 10.0);
        assert!(called(&e, 0x0040_fbf0).is_empty());
        assert!(called(&e, 0x0088_b510).is_empty());
        let calendar = called(&e, 0x0086_7a40);
        assert_eq!(calendar.len(), 1);
        // 0.5 hours of 3600 seconds over the scale 20: 90 time units.
        assert_eq!(f(&calendar[0], 1), 90.0);
        assert_eq!(f(&called(&e, 0x0096_d4b0)[0], 1), 1090.0);
        assert_eq!(e.get(world.this, PlayerCharacter::iSleepTime), 0);
    }

    #[test]
    fn fast_travel_reports_a_missing_path_and_takes_no_time() {
        let mut e = engine();
        e.register(0x008d_6f30, |_, _| int(0));
        let (_world, _) = run_fast_travel(&mut e, f32::MAX, 10.0);
        // The message is built with the format (no cell name without a
        // parent cell) and logged; the trip takes no time.
        let format = &called(&e, 0x0040_6d00)[0];
        assert_eq!((format[1], format[2]), (0x400, 0x0108_aff4));
        assert_eq!(called(&e, 0x005b_5e40), vec![vec![format[0]]]);
        assert!(called(&e, 0x0040_37b0).is_empty());
        assert!(called(&e, 0x0040_fbf0).is_empty());
    }

    #[test]
    fn fast_travel_names_the_destination_cell_in_the_missing_path_message() {
        let mut e = engine();
        let cell = e.mem.alloc(16);
        e.register_double(0x00b4_0090, |_, _| Ret::default());
        put_vtable(&mut e, cell, 0x0231_0000, &[(0x90, 0x00b4_0090)]);
        let (_world, destination) = run_fast_travel_with(&mut e, f32::MAX, 10.0, |e| {
            e.register_double(0x008d_6f30, move |_, a| {
                int(if a[0] != 0 { cell } else { 0 })
            });
            e.register(0x0055_9450, |_, _| int(0x7e00));
        });
        let name = called(&e, 0x0040_37b0)[0][0];
        assert_eq!(called(&e, 0x00b4_0090), vec![vec![cell, name]]);
        let format = &called(&e, 0x0040_6d50)[0];
        assert_eq!((format[1], format[2]), (0x400, 0x7e00));
        assert_eq!(called(&e, 0x0040_37d0), vec![vec![name]]);
        let _ = destination;
    }

    #[test]
    fn fast_travel_places_the_player_at_the_destination() {
        let mut e = engine();
        let (world, destination) = run_fast_travel(&mut e, 36000.0, 10.0);
        let this = world.this;
        // PositionPlayerExterior found the cell of the destination's world
        // space and put the player there.
        assert_eq!(called(&e, 0x0058_75a0)[0][0], 0x7300);
        assert_eq!(called(&e, 0x0057_5bb0)[0], vec![this.addr(), 0x7400]);
        let listener = &called(&e, 0x00ad_78b0)[0];
        assert_eq!(
            listener[1..],
            [7.0f32.to_bits(), 8.0f32.to_bits(), 9.0f32.to_bits()]
        );
        // Around it: the travel flag, the sky and the beds/chairs set up.
        assert_eq!(e.mem.u8(FAST_TRAVELLING), 0);
        assert_eq!(called(&e, 0x0045_9870), vec![vec![0x5000]]);
        assert_eq!(called(&e, 0x0097_2d30), vec![vec![PROCESS_LISTS]]);
        assert_eq!(called(&e, 0x0096_df40), vec![vec![PROCESS_LISTS]]);
        assert_eq!(called(&e, 0x0097_5d10), vec![vec![PROCESS_LISTS]]);
        assert_eq!(
            called(&e, 0x0097_3ee0),
            vec![vec![PROCESS_LISTS, destination]]
        );
        assert_eq!(called(&e, 0x0096_1f90)[0], vec![this.addr()]);
        assert_eq!(e.mem.u8(this.addr() + 0x20c), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x204), 0);
        assert_eq!(called(&e, 0x0045_7d70)[0], vec![0x5000, 1, destination, 0]);
    }

    #[test]
    fn fast_travel_sets_the_sky_and_the_travel_flag_around_the_placement() {
        // The sky is told "fast travelling" (1) before the placement and
        // "ended" (0) after it.
        let mut e = engine();
        let flags = Rc::new(RefCell::new(vec![]));
        let sky_calls = Rc::new(RefCell::new(vec![]));
        let (world, destination) = fast_travel_world(&mut e);
        e.register(0x0046_dd00, |_, _| int(0xaa00));
        let log = sky_calls.clone();
        e.register_double(0x0063_e8f0, move |e, a| {
            log.borrow_mut()
                .push((a[0], a[1], e.mem.u8(FAST_TRAVELLING)));
            Ret::default()
        });
        let log = flags.clone();
        e.register_double(0x0057_5bb0, move |e, a| {
            log.borrow_mut().push((a[1], e.mem.u8(FAST_TRAVELLING)));
            Ret::default()
        });
        e.register(0x0057_5d70, |_, _| int(0x7300));
        e.register(0x0058_75a0, |_, _| int(0x7400));
        e.register(0x0045_11e0, |_, _| int(1));
        e.register(0x006d_4eb0, |_, _| float(0.0));
        e.register(0x0088_4eb0, |_, _| float(1.0));
        e.register(0x0086_7950, |_, _| float(1.0));
        let rotation = e.mem.alloc(16);
        e.register_double(0x0043_0830, move |_, _| int(rotation));
        e.set_global(PLAYER_SINGLETON, world.this.addr());
        e.call(0x0093_cdf0, &args![world.this, destination]);
        assert_eq!(*sky_calls.borrow(), vec![(0xaa00, 1, 0), (0xaa00, 0, 0)]);
        // The travel flag is set while the player is placed (twice: into the
        // cell, then out of it) and cleared afterwards.
        assert_eq!(*flags.borrow(), vec![(0x7400, 1), (0, 1)]);
    }

    #[test]
    fn fast_travel_flushes_the_cells_when_the_pause_is_long() {
        let mut e = engine();
        let (world, destination) = fast_travel_world(&mut e);
        let setting = e.mem.alloc(8);
        e.mem.set_u8(setting, 1);
        e.register_double(0x0040_8d60, move |_, _| int(setting));
        e.set_global(FAST_TRAVEL_REACTOR, 0x6a00u32);
        let times = Rc::new(RefCell::new(vec![5000u32, 5000, 30000, 31000]));
        let queue = times.clone();
        e.register_double(0x0082_5c00, move |_, _| int(queue.borrow_mut().remove(0)));
        e.register(0x006d_4eb0, |_, _| float(0.0));
        e.register(0x0088_4eb0, |_, _| float(1.0));
        e.register(0x0086_7950, |_, _| float(1.0));
        e.register(0x0058_75a0, |_, _| int(0));
        e.register(0x0058_5b30, |_, _| int(0));
        e.register(0x0046_1330, |_, _| int(0));
        e.set_global(PLAYER_SINGLETON, world.this.addr());
        // Run it twice; the static timestamp is initialised the first time.
        e.call(0x0093_cdf0, &args![world.this, destination]);
        // First call: init stamp 5000, now 5000: elapsed 0, so it reacts
        // and re-stamps with 30000.
        assert_eq!(called(&e, 0x0085_0a40), vec![vec![0x6a00]]);
        assert_eq!(e.global::<u32>(FAST_TRAVEL_LAST_TIME), 30000);
    }

    #[test]
    fn fast_travel_leaves_the_pause_alone_when_the_setting_is_off() {
        let mut e = engine();
        let (world, destination) = fast_travel_world(&mut e);
        e.register(0x006d_4eb0, |_, _| float(0.0));
        e.register(0x0088_4eb0, |_, _| float(1.0));
        e.register(0x0086_7950, |_, _| float(1.0));
        e.register(0x0058_75a0, |_, _| int(0));
        e.register(0x0058_5b30, |_, _| int(0));
        e.register(0x0046_1330, |_, _| int(0));
        e.set_global(PLAYER_SINGLETON, world.this.addr());
        e.call(0x0093_cdf0, &args![world.this, destination]);
        assert!(called(&e, 0x0085_0a40).is_empty());
        assert_eq!(called(&e, 0x0082_5c00).len(), 2);
    }

    #[test]
    fn fast_travel_cache_cleanup_runs_when_the_face_cache_exists() {
        let mut e = engine();
        let (world, destination) = fast_travel_world(&mut e);
        e.register(0x006d_4eb0, |_, _| float(0.0));
        e.register(0x0088_4eb0, |_, _| float(1.0));
        e.register(0x0086_7950, |_, _| float(1.0));
        e.register(0x0058_75a0, |_, _| int(0));
        e.register(0x0058_5b30, |_, _| int(0));
        e.register(0x0046_1330, |_, _| int(0));
        e.register(0x0065_2110, |_, _| int(0x6b00));
        e.register(0x0045_4af0, |_, _| int(1));
        e.set_global(PLAYER_SINGLETON, world.this.addr());
        e.call(0x0093_cdf0, &args![world.this, destination]);
        assert_eq!(called(&e, 0x0065_0a30)[0], vec![0x6b00, 0]);
        // The object at 011c3c10 is asked twice (before and after) and the
        // grid told (0, then 1).
        assert_eq!(
            called(&e, 0x0045_aee0),
            vec![vec![0x5000, 0], vec![0x5000, 1]]
        );
    }

    #[test]
    fn the_cell_grid_object_query_returns_the_callee_result() {
        let mut e = engine();
        e.register(0x0045_4af0, |_, _| int(1));
        assert!(e.call(0x0093_d4f0, &args![]).bool());
        assert_eq!(called(&e, 0x0045_4af0), vec![vec![0x011c_3c10]]);
        e.register(0x0045_4af0, |_, _| int(0));
        assert!(!e.call(0x0093_d4f0, &args![]).bool());
    }

    /// The world of `ClearCellsIfTargetNotLoaded`: every callee a stub, the
    /// TES singleton pointing at 0x5000 and the player singleton at the
    /// returned player.
    fn clear_cells_world(e: &mut Engine) -> Ptr<PlayerCharacter> {
        let this = player(e);
        e.map(0x011a_0000, 0x6_0000);
        e.set_global(TES_SINGLETON, 0x5000u32);
        e.set_global(PLAYER_SINGLETON, this.addr());
        stub(
            e,
            &[
                0x0045_11e0,
                0x0045_39a0,
                0x0045_7d70,
                0x0054_ca90,
                0x0065_0a30,
                0x0065_2110,
                0x0087_8160,
                0x0087_8200,
                0x0087_8250,
                0x008d_6f30,
                0x0095_0bb0,
                0x0096_11e0,
                0x0096_d810,
                0x0096_eb40,
                0x00aa_7030,
            ],
        );
        this
    }

    #[test]
    fn clear_cells_does_nothing_for_a_loaded_cell() {
        let mut e = engine();
        let this = clear_cells_world(&mut e);
        e.register(0x0045_11e0, |_, _| int(1));
        e.call(0x0093_d500, &args![this, 0x6000u32, 0x7000u32]);
        assert_eq!(call_addresses(&e), vec![0x0093_d500, 0x0045_11e0]);
        assert_eq!(called(&e, 0x0045_11e0), vec![vec![0x5000, 0x6000, 0]]);
    }

    #[test]
    fn clear_cells_flushes_the_grid_for_the_cell() {
        let mut e = engine();
        let this = clear_cells_world(&mut e);
        e.register(0x008d_6f30, |_, _| int(0x3300));
        e.register(0x0087_8160, |e, a| {
            e.mem.set_u8(a[0] + 5, 0x55);
            Ret::default()
        });
        e.register(0x0065_2110, |_, _| int(0x6b00));
        let busy = Rc::new(RefCell::new(vec![]));
        let seen = busy.clone();
        e.register_double(0x0096_d810, move |e, _| {
            seen.borrow_mut().push(e.mem.u8(PROCESS_LISTS_BUSY));
            Ret::default()
        });
        // The found object has a virtual +0xC returning a handler holder.
        let found = e.mem.alloc(16);
        let holder = e.mem.alloc(16);
        let handler = e.mem.alloc(16);
        e.register_double(0x00b0_000c, move |_, _| int(holder));
        e.register_double(0x00b0_00e8, |_, _| Ret::default());
        put_vtable(&mut e, found, 0x0240_0000, &[(0xc, 0x00b0_000c)]);
        put_vtable(&mut e, handler, 0x0241_0000, &[(0xe8, 0x00b0_00e8)]);
        e.register_double(0x0095_0bb0, move |_, _| int(found));
        e.register_double(0x0096_11e0, move |_, _| int(handler));
        e.call(0x0093_d500, &args![this, 0x6000u32, 0x7000u32]);
        assert_eq!(called(&e, 0x0045_7d70), vec![vec![0x5000, 1, 0x6000, 0]]);
        // The player leaves its parent cell.
        assert_eq!(called(&e, 0x0054_ca90), vec![vec![0x3300, this.addr()]]);
        assert_eq!(called(&e, 0x0095_0bb0), vec![vec![this.addr(), 1]]);
        assert_eq!(called(&e, 0x0096_11e0)[0], vec![holder]);
        assert_eq!(called(&e, 0x00b0_00e8), vec![vec![handler, found]]);
        assert_eq!(called(&e, 0x0087_8160)[0][1..], [0, 1, 0]);
        assert_eq!(called(&e, 0x0045_39a0), vec![vec![0x5000, 0, 1]]);
        assert_eq!(*busy.borrow(), vec![1]);
        assert_eq!(e.mem.u8(PROCESS_LISTS_BUSY), 0);
        assert_eq!(called(&e, 0x0096_eb40), vec![vec![PROCESS_LISTS]]);
        assert_eq!(called(&e, 0x0087_8250), vec![vec![0x55]]);
        assert_eq!(called(&e, 0x0087_8200).len(), 1);
        assert_eq!(called(&e, 0x0065_0a30), vec![vec![0x6b00, 0]]);
        assert_eq!(called(&e, 0x00aa_7030).len(), 1);
    }

    #[test]
    fn clear_cells_uses_the_world_space_without_a_cell() {
        let mut e = engine();
        let this = clear_cells_world(&mut e);
        e.call(0x0093_d500, &args![this, 0u32, 0x7000u32]);
        assert!(called(&e, 0x0045_11e0).is_empty());
        assert_eq!(called(&e, 0x0045_7d70), vec![vec![0x5000, 1, 0x7000, 0]]);
        // Nothing found, no parent cell, no face cache.
        assert!(called(&e, 0x0054_ca90).is_empty());
        assert!(called(&e, 0x0096_11e0).is_empty());
        assert!(called(&e, 0x0065_0a30).is_empty());
    }

    /// The world of the queued-request handler: the position world with
    /// the handler's own callees stubbed, the player's virtual `+0x358`
    /// true (so a fast travel returns at once) and the deletions recorded.
    fn handler_world(e: &mut Engine) -> (PositionWorld, Rc<RefCell<Vec<u32>>>) {
        let world = exterior_world(e);
        stub(
            e,
            &[
                0x0045_2eb0,
                NI_POINTER_ASSIGN,
                0x0056_9b80,
                0x0057_3f20,
                0x0086_8d70,
                0x005d_14d0,
                0x0056_8680,
                0x008d_8520,
                0x007a_f430,
                0x0050_9420,
                0x0056_82c0,
                0x0056_8500,
                0x0088_d2f0,
                0x0093_06d0,
            ],
        );
        e.register_double(0x00b0_0358, |_, _| int(1));
        e.mem.set_u32(0x0200_0000 + 0x358, 0x00b0_0358);
        let deleted = Rc::new(RefCell::new(vec![]));
        let log = deleted.clone();
        e.register_double(OPERATOR_DELETE, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        (world, deleted)
    }

    /// Queues a request with the given fields on the player.
    fn queue_request(
        e: &mut Engine,
        this: Ptr<PlayerCharacter>,
        fill: impl FnOnce(&mut Engine, Ptr<PositionPlayerRequest>),
    ) -> Ptr<PositionPlayerRequest> {
        let request = e.new_object::<PositionPlayerRequest>();
        fill(e, request);
        e.set(this, PlayerCharacter::pQueuedTargetLoc, request.cast());
        request
    }

    #[test]
    fn handling_an_empty_queue_does_nothing() {
        let mut e = engine();
        let (world, deleted) = handler_world(&mut e);
        assert!(!e.call(0x0093_bea0, &args![world.this]).bool());
        assert_eq!(call_addresses(&e), vec![0x0093_bea0]);
        assert!(deleted.borrow().is_empty());
    }

    #[test]
    fn handling_an_invalid_request_reports_it_and_frees_it() {
        let mut e = engine();
        let (world, deleted) = handler_world(&mut e);
        let this = world.this;
        let request = queue_request(&mut e, this, |_, _| {});
        let moved = e.call(0x0093_bea0, &args![this]).bool();
        assert!(!moved);
        assert_eq!(called(&e, 0x005b_5e40), vec![vec![0x0108_afa0]]);
        assert_eq!(called(&e, 0x0045_2eb0), vec![vec![0x5000]]);
        assert_eq!(
            called(&e, NI_POINTER_ASSIGN),
            vec![vec![this.addr() + 0xd3c, 0]]
        );
        assert_eq!(called(&e, 0x0086_8d70), vec![vec![1]]);
        assert_eq!(*deleted.borrow(), vec![request.addr()]);
        assert!(e.get(this, PlayerCharacter::pQueuedTargetLoc).is_null());
        // Nothing moved: no follow-up on the player.
        assert!(called(&e, 0x005d_14d0).is_empty());
        assert!(called(&e, 0x0057_3f20).is_empty());
    }

    #[test]
    fn handling_a_fast_travel_request_travels_and_follows_up() {
        let mut e = engine();
        let (world, deleted) = handler_world(&mut e);
        let this = world.this;
        e.register(0x0056_9b80, |_, _| int(0));
        let request = queue_request(&mut e, this, |e, r| {
            e.set(
                r,
                PositionPlayerRequest::fast_travel_target,
                Ptr::new(0x7000),
            );
            // Everything else is ignored by a fast travel.
            e.set(r, PositionPlayerRequest::furniture, Ptr::new(0x8800));
        });
        assert!(e.call(0x0093_bea0, &args![this]).bool());
        assert_eq!(called(&e, 0x0056_9b80), vec![vec![0x7000]]);
        // The travel ran (and stopped at once): the entry point was asked.
        let entry = &called(&e, 0x005e_58f0)[0];
        assert_eq!((entry[0], entry[1]), (0x33, this.addr()));
        assert!(called(&e, 0x0056_8680).is_empty());
        assert_eq!(*deleted.borrow(), vec![request.addr()]);
        // The follow-up on the player.
        assert_eq!(called(&e, 0x005d_14d0), vec![vec![this.addr(), 1]]);
    }

    #[test]
    fn the_follow_up_is_skipped_when_forbidden() {
        let mut e = engine();
        let (world, _) = handler_world(&mut e);
        let this = world.this;
        e.register(0x0042_ce10, |_, _| int(1));
        queue_request(&mut e, this, |e, r| {
            e.set(
                r,
                PositionPlayerRequest::fast_travel_target,
                Ptr::new(0x7000),
            );
        });
        assert!(e.call(0x0093_bea0, &args![this]).bool());
        assert!(called(&e, 0x005d_14d0).is_empty());
        // Or by the fast-travel flag.
        let mut e = engine();
        let (world, _) = handler_world(&mut e);
        let this = world.this;
        e.mem.set_u8(this.addr() + 0x66d, 2);
        queue_request(&mut e, this, |e, r| {
            e.set(
                r,
                PositionPlayerRequest::fast_travel_target,
                Ptr::new(0x7000),
            );
        });
        assert!(e.call(0x0093_bea0, &args![this]).bool());
        assert!(called(&e, 0x005d_14d0).is_empty());
    }

    #[test]
    fn handling_a_cell_request_positions_the_player_and_runs_the_callback() {
        let mut e = engine();
        let (world, _) = handler_world(&mut e);
        let this = world.this;
        e.register(0x0093_06d0, |_, _| int(0x4400));
        e.register_double(0x00b0_0cb0, |_, _| Ret::default());
        queue_request(&mut e, this, |e, r| {
            e.set(r, PositionPlayerRequest::cell, Ptr::new(0x6100));
            for (i, v) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].into_iter().enumerate() {
                e.mem.set_f32(r.addr() + 8 + 4 * i as u32, v);
            }
            e.set(r, PositionPlayerRequest::callback, 0x00b0_0cb0);
            e.set(r, PositionPlayerRequest::callback_argument, 0x1234);
        });
        let placed_at = watch_floats(&mut e, 0x00b0_02a8, 1, 3);
        let turned = watch_floats(&mut e, 0x0086_d490, 1, 3);
        assert!(e.call(0x0093_bea0, &args![this]).bool());
        // PositionPlayer ran with the request's position and rotation.
        assert_eq!(called(&e, 0x0057_5bb0)[0], vec![this.addr(), 0x6100]);
        assert_eq!(placed_at.borrow()[0], vec![1.0, 2.0, 3.0]);
        assert_eq!(turned.borrow()[0], vec![4.0, 5.0, 6.0]);
        // The controller gets the request's height (z), then the callback.
        let height = &called(&e, 0x0057_3f20)[0];
        assert_eq!((height[0], f(height, 1)), (0x4400, 3.0));
        assert_eq!(called(&e, 0x00b0_0cb0), vec![vec![0x1234]]);
    }

    #[test]
    fn handling_a_world_space_request_positions_the_player_in_that_world() {
        let mut e = engine();
        let (world, _) = handler_world(&mut e);
        let this = world.this;
        e.register(0x0058_75a0, |_, _| int(0x6200));
        e.register(0x0045_11e0, |_, _| int(1));
        queue_request(&mut e, this, |e, r| {
            e.set(r, PositionPlayerRequest::worldspace, Ptr::new(0x7200));
            // A cell is ignored when there is a world space.
            e.set(r, PositionPlayerRequest::cell, Ptr::new(0x6100));
            e.mem.set_f32(r.addr() + 8, 5000.0);
            e.mem.set_f32(r.addr() + 12, 9000.0);
            e.set(r, PositionPlayerRequest::reset_weather, 1);
        });
        e.register(0x0046_dd00, |_, _| int(0xaa00));
        assert!(e.call(0x0093_bea0, &args![this]).bool());
        assert_eq!(called(&e, 0x0058_75a0), vec![vec![0x7200, 1, 2]]);
        assert_eq!(called(&e, 0x0057_5bb0)[0], vec![this.addr(), 0x6200]);
        // The flag byte reset the weather.
        assert_eq!(called(&e, 0x0063_d060), vec![vec![0xaa00]]);
    }

    #[test]
    fn handling_a_request_with_furniture_puts_the_player_in_it() {
        let mut e = engine();
        let (world, _) = handler_world(&mut e);
        let this = world.this;
        e.register(0x0056_8680, |_, _| int(1));
        e.register(0x008d_8520, |_, _| int(0x4500));
        let process = e.mem.alloc(16);
        e.register_double(0x00b0_04d4, |_, _| int(0x77));
        put_vtable(&mut e, process, 0x0250_0000, &[(0x4d4, 0x00b0_04d4)]);
        e.register_double(0x008d_8520, move |_, _| int(process));
        e.register(0x007a_f430, |_, _| int(0x4600));
        e.register(0x0050_9420, |_, _| int(1));
        e.register(0x0056_82c0, |_, _| int(3));
        e.register(0x0056_8500, |_, _| int(1));
        queue_request(&mut e, this, |e, r| {
            e.set(r, PositionPlayerRequest::furniture, Ptr::new(0x8800));
        });
        e.call(0x0093_bea0, &args![this]);
        assert_eq!(called(&e, 0x0056_8680), vec![vec![0x8800]]);
        assert_eq!(called(&e, 0x0056_82c0), vec![vec![0x8800, 1]]);
        assert_eq!(called(&e, 0x0056_8500), vec![vec![0x8800, 3, 0x77]]);
        assert_eq!(
            called(&e, 0x0088_d2f0),
            vec![vec![this.addr(), 0x8800, 0x77, 3, 1]]
        );
    }

    #[test]
    fn furniture_without_a_free_marker_is_left_alone() {
        let mut e = engine();
        let (world, _) = handler_world(&mut e);
        let this = world.this;
        e.register(0x0056_8680, |_, _| int(1));
        let process = e.mem.alloc(16);
        e.register_double(0x00b0_04d4, |_, _| int(0x77));
        put_vtable(&mut e, process, 0x0250_0000, &[(0x4d4, 0x00b0_04d4)]);
        e.register_double(0x008d_8520, move |_, _| int(process));
        e.register(0x0056_82c0, |_, _| int(-1i32 as u32));
        queue_request(&mut e, this, |e, r| {
            e.set(r, PositionPlayerRequest::furniture, Ptr::new(0x8800));
        });
        e.call(0x0093_bea0, &args![this]);
        assert!(called(&e, 0x0056_8500).is_empty());
        assert!(called(&e, 0x0088_d2f0).is_empty());
        // A marker the furniture does not accept is not used either.
        let mut e = engine();
        let (world, _) = handler_world(&mut e);
        let this = world.this;
        e.register(0x0056_8680, |_, _| int(1));
        let process = e.mem.alloc(16);
        e.register_double(0x00b0_04d4, |_, _| int(0x77));
        put_vtable(&mut e, process, 0x0250_0000, &[(0x4d4, 0x00b0_04d4)]);
        e.register_double(0x008d_8520, move |_, _| int(process));
        e.register(0x0056_82c0, |_, _| int(2));
        e.register(0x0056_8500, |_, _| int(0));
        queue_request(&mut e, this, |e, r| {
            e.set(r, PositionPlayerRequest::furniture, Ptr::new(0x8800));
        });
        e.call(0x0093_bea0, &args![this]);
        assert_eq!(called(&e, 0x0056_8500).len(), 1);
        assert!(called(&e, 0x0088_d2f0).is_empty());
    }

    #[test]
    fn a_furniture_reference_that_is_not_furniture_is_ignored() {
        let mut e = engine();
        let (world, _) = handler_world(&mut e);
        let this = world.this;
        queue_request(&mut e, this, |e, r| {
            e.set(r, PositionPlayerRequest::furniture, Ptr::new(0x8800));
        });
        e.call(0x0093_bea0, &args![this]);
        assert_eq!(called(&e, 0x0056_8680), vec![vec![0x8800]]);
        assert!(called(&e, 0x008d_8520).is_empty());
    }

    /// The world of `RequestPositionPlayer`: the grid can take requests
    /// when `ready`; deletions are recorded.
    fn request_world(e: &mut Engine, ready: bool) -> (PositionWorld, Rc<RefCell<Vec<u32>>>) {
        let (world, deleted) = handler_world(e);
        e.register_double(0x0045_1530, move |_, _| int(ready as u32));
        (world, deleted)
    }

    #[test]
    fn requesting_a_position_stores_the_request() {
        let mut e = engine();
        let (world, deleted) = request_world(&mut e, false);
        let this = world.this;
        let request = e.new_object::<PositionPlayerRequest>();
        e.call(0x0093_be30, &args![this, request]);
        assert_eq!(
            e.get(this, PlayerCharacter::pQueuedTargetLoc).addr(),
            request.addr()
        );
        assert!(deleted.borrow().is_empty());
        assert!(called(&e, 0x005b_5e40).is_empty());
        assert_eq!(called(&e, 0x0045_1530), vec![vec![0x5000]]);
        assert!(called(&e, 0x0045_2eb0).is_empty());
    }

    #[test]
    fn requesting_a_position_replaces_an_earlier_request_with_a_warning() {
        let mut e = engine();
        let (world, deleted) = request_world(&mut e, false);
        let this = world.this;
        let earlier = queue_request(&mut e, this, |_, _| {});
        let request = e.new_object::<PositionPlayerRequest>();
        e.call(0x0093_be30, &args![this, request]);
        assert_eq!(called(&e, 0x005b_5e40), vec![vec![0x0108_af58]]);
        assert_eq!(*deleted.borrow(), vec![earlier.addr()]);
        assert_eq!(
            e.get(this, PlayerCharacter::pQueuedTargetLoc).addr(),
            request.addr()
        );
    }

    #[test]
    fn cancelling_a_request_frees_it_silently() {
        let mut e = engine();
        let (world, deleted) = request_world(&mut e, false);
        let this = world.this;
        let earlier = queue_request(&mut e, this, |_, _| {});
        e.call(0x0093_be30, &args![this, 0u32]);
        assert!(called(&e, 0x005b_5e40).is_empty());
        assert_eq!(*deleted.borrow(), vec![earlier.addr()]);
        assert!(e.get(this, PlayerCharacter::pQueuedTargetLoc).is_null());
    }

    #[test]
    fn a_ready_grid_handles_the_request_at_once() {
        let mut e = engine();
        let (world, deleted) = request_world(&mut e, true);
        let this = world.this;
        let request = e.new_object::<PositionPlayerRequest>();
        e.call(0x0093_be30, &args![this, request]);
        // The (invalid) request was handled and freed.
        assert_eq!(called(&e, 0x0045_2eb0).len(), 1);
        assert_eq!(*deleted.borrow(), vec![request.addr()]);
        assert!(e.get(this, PlayerCharacter::pQueuedTargetLoc).is_null());
    }

    /// The exterior-placement world: the position world, with the callees
    /// of the fast travel and of the cell clearing stubbed.
    fn exterior_world(e: &mut Engine) -> PositionWorld {
        let world = position_world(e);
        stub(e, FAST_TRAVEL_CALLEES);
        stub(e, EXTERIOR_PLACEMENT_CALLEES);
        e.register(0x0040_6d90, |_, a| int(f32::from_bits(a[0]) as i32 as u32));
        world
    }

    #[test]
    fn exterior_placement_without_a_world_space_does_nothing() {
        let mut e = engine();
        let world = exterior_world(&mut e);
        e.call(
            0x0093_cce0,
            &args![world.this, 1.0f32, 2.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, 0u32, true],
        );
        assert_eq!(call_addresses(&e), vec![0x0093_cce0]);
    }

    #[test]
    fn exterior_placement_cancels_loads_and_uses_the_cell_found() {
        let mut e = engine();
        let world = exterior_world(&mut e);
        e.set_global(EXTERIOR_CELL_LOADER, 0x5900u32);
        e.register(0x0058_75a0, |_, _| int(0x6200));
        e.register(0x0045_11e0, |_, _| int(1));
        e.call(
            0x0093_cce0,
            &args![
                world.this, 5000.0f32, 9000.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, 0x7200u32, false
            ],
        );
        assert_eq!(called(&e, 0x0052_8540), vec![vec![0x5900]]);
        // 5000 >> 12 = 1 and 9000 >> 12 = 2.
        assert_eq!(called(&e, 0x0058_75a0), vec![vec![0x7200, 1, 2]]);
        // The cell is already loaded: nothing is cleared, nothing created.
        assert_eq!(called(&e, 0x0045_11e0), vec![vec![0x5000, 0x6200, 0]]);
        assert!(called(&e, 0x0058_5b30).is_empty());
        assert!(called(&e, 0x0046_1330).is_empty());
        // The player singleton is placed in it.
        assert_eq!(called(&e, 0x0057_5bb0)[0], vec![world.this.addr(), 0x6200]);
    }

    #[test]
    fn exterior_placement_loads_and_then_creates_the_cell() {
        let mut e = engine();
        let world = exterior_world(&mut e);
        e.set_global(DATA_HANDLER, 0x5800u32);
        e.register(0x0058_75a0, |_, _| int(0));
        e.register(0x0058_5b30, |_, _| int(0));
        e.register(0x0046_1330, |_, _| int(0x6300));
        e.call(
            0x0093_cce0,
            &args![
                world.this, 5000.0f32, 9000.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, 0x7200u32, false
            ],
        );
        // Nothing found: the cells are cleared for the world space, the cell
        // is loaded, then created.
        assert_eq!(called(&e, 0x0045_7d70)[0], vec![0x5000, 1, 0x7200, 0]);
        assert_eq!(called(&e, 0x0058_5b30), vec![vec![0x7200, 1, 2]]);
        assert_eq!(called(&e, 0x0046_1330), vec![vec![0x5800, 0, 1, 2, 0x7200]]);
        assert_eq!(called(&e, 0x0057_5bb0)[0], vec![world.this.addr(), 0x6300]);
    }

    #[test]
    fn exterior_placement_gives_up_when_no_cell_can_be_made() {
        let mut e = engine();
        let world = exterior_world(&mut e);
        e.call(
            0x0093_cce0,
            &args![
                world.this, 5000.0f32, 9000.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, 0x7200u32, false
            ],
        );
        assert!(called(&e, 0x0057_5bb0).is_empty());
    }

    #[test]
    fn small_flag_getters() {
        let mut e = engine();
        let this = player(&mut e);
        for (byte, can_travel, travel_flag) in [
            (0u8, false, false),
            (1, false, true),
            (2, true, false),
            (3, true, true),
        ] {
            e.mem.set_u8(this.addr() + 0x66d, byte);
            assert_eq!(e.call(0x0093_c1e0, &args![this]).bool(), can_travel);
            assert_eq!(e.call(0x0093_db40, &args![this]).bool(), travel_flag);
        }
    }

    #[test]
    fn fn_0093ccd0_reads_its_global() {
        let mut e = engine();
        e.map(0x011c_3000, 0x1000);
        e.set_global(GLOBAL_RETURNED_BY_0093CCD0, 0x1234u32);
        assert_eq!(e.call(0x0093_ccd0, &args![]).u32(), 0x1234);
    }

    #[test]
    fn small_field_accessors() {
        let mut e = engine();
        let this = player(&mut e);
        e.call(0x0094_42e0, &args![this, 2.5f32]);
        assert_eq!(
            e.get(this, PlayerCharacter::fTimeSinceLastAmmoRegenTick),
            2.5
        );
        // The generic getters read their offsets.
        e.mem.set_f32(this.addr() + 0x10c, 7.5);
        assert_eq!(e.call(0x0094_42a0, &args![this]).f32(), 7.5);
        e.mem.set_u32(this.addr() + 0x12c, 0x1000);
        assert!(e.call(0x0094_42c0, &args![this]).bool());
        e.mem.set_u32(this.addr() + 0x12c, 0xefff);
        assert!(!e.call(0x0094_42c0, &args![this]).bool());
        e.mem.set_u32(this.addr() + 0x634, 0x9900);
        assert_eq!(e.call(0x0093_e4f0, &args![this]).u32(), 0x9900);
    }

    #[test]
    fn fn_00944270_reads_a_byte_of_the_global_object() {
        let mut e = engine();
        e.map(0x011d_8000, 0x1000);
        assert_eq!(e.call(0x0094_4270, &args![]).u8(), 0);
        let object = e.mem.alloc(0x500);
        e.mem.set_u8(object + 0x4cd, 9);
        e.set_global(0x011d_8a80, object);
        assert_eq!(e.call(0x0094_4270, &args![]).u8(), 9);
    }

    #[test]
    fn fn_0093e510_calls_through_the_member_at_0x1c() {
        let mut e = engine();
        stub(&mut e, &[0x0052_72b0]);
        e.call(0x0093_e510, &args![0x3000u32]);
        assert_eq!(called(&e, 0x0052_72b0), vec![vec![0x301c]]);
    }

    #[test]
    fn fn_0093dd20_passes_the_masked_flags_to_the_actor_mover() {
        let mut e = engine();
        let this = player(&mut e);
        let mover = e.mem.alloc(16);
        e.register_double(0x00b0_000c, |_, _| Ret::default());
        put_vtable(&mut e, mover, 0x0260_0000, &[(0xc, 0x00b0_000c)]);
        e.mem.set_u32(this.addr() + 0x190, mover);
        e.register(0x0088_46e0, |_, _| int(0xffff_ffff));
        e.call(0x0093_dd20, &args![this]);
        assert_eq!(called(&e, 0x00b0_000c), vec![vec![mover, 0xcc00 | 0x400]]);
        // Only the low word of the result counts, and the mask applies.
        e.register(0x0088_46e0, |_, _| int(0x1_0011));
        e.call(0x0093_dd20, &args![this]);
        assert_eq!(called(&e, 0x00b0_000c)[1], vec![mover, 0x400]);
    }

    /// The perceived-actors list: nodes of {item, next}, items {actor,
    /// hostile byte}. `LIST_ITEM_ADDRESS` returns the node itself.
    fn perceived_world(e: &mut Engine, entries: &[(u32, u8)]) -> Ptr<PlayerCharacter> {
        let this = player(e);
        let mut next = 0u32;
        for (actor, hostile) in entries.iter().rev() {
            let entry = e.mem.alloc(8);
            e.mem.set_u32(entry, *actor);
            e.mem.set_u8(entry + 4, *hostile);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, entry);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        e.set(
            this,
            PlayerCharacter::pListofPercievedActors,
            Ptr::new(next),
        );
        e.register(LIST_ITEM_ADDRESS, |_, a| int(a[0]));
        e.register(LIST_NEXT, |e, a| int(e.mem.u32(a[0] + 4)));
        this
    }

    #[test]
    fn the_hostile_flag_of_a_perceived_actor_is_found_by_actor() {
        let mut e = engine();
        let this = perceived_world(&mut e, &[(0x11, 0), (0x22, 1), (0x33, 0)]);
        assert!(!e.call(0x0093_dee0, &args![this, 0x11u32]).bool());
        assert!(e.call(0x0093_dee0, &args![this, 0x22u32]).bool());
        assert!(!e.call(0x0093_dee0, &args![this, 0x33u32]).bool());
        assert!(!e.call(0x0093_dee0, &args![this, 0x44u32]).bool());
    }

    #[test]
    fn nobody_is_hostile_when_nobody_is_perceived() {
        let mut e = engine();
        let this = perceived_world(&mut e, &[]);
        assert!(!e.call(0x0093_dee0, &args![this, 0x11u32]).bool());
    }

    #[test]
    fn hostile_perceived_actors_get_temp_effects() {
        let mut e = engine();
        let this = perceived_world(&mut e, &[(0x11, 0), (0x22, 1), (0x33, 1)]);
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x011a_0000, 0x6_0000);
        e.set_global(DATA_HANDLER, 0x5800u32);
        e.set_global(TIMER_RESET, -1.0f32);
        e.register(0x007b_8f10, |_, _| int(0x9a00));
        let blocks = Rc::new(RefCell::new(vec![0x6001u32, 0x6002]));
        let queue = blocks.clone();
        e.register_double(0x00aa_13e0, move |_, _| int(queue.borrow_mut().remove(0)));
        // The first effect is accepted, the second refused (and deleted).
        let accepted = e.mem.alloc(16);
        let refused = e.mem.alloc(16);
        e.register_double(0x00b0_00c4, move |_, a| int((a[0] == accepted) as u32));
        e.register_double(0x00b0_0000, |_, _| Ret::default());
        put_vtable(&mut e, accepted, 0x0270_0000, &[(0xc4, 0x00b0_00c4)]);
        put_vtable(
            &mut e,
            refused,
            0x0271_0000,
            &[(0xc4, 0x00b0_00c4), (0, 0x00b0_0000)],
        );
        let effects = Rc::new(RefCell::new(vec![accepted, refused]));
        let queue = effects.clone();
        e.register_double(0x0081_f580, move |_, _| int(queue.borrow_mut().remove(0)));
        stub(&mut e, &[0x0097_3fd0]);
        e.call(0x0093_dd80, &args![this]);
        assert_eq!(called(&e, 0x007b_8f10), vec![vec![0x5800]]);
        let creations = called(&e, 0x0081_f580);
        assert_eq!(creations.len(), 2);
        assert_eq!(creations[0][..3], [0x6001, 0x22, 0x9a00]);
        assert_eq!(f(&creations[0], 3), -1.0);
        assert_eq!(creations[1][1], 0x33);
        assert_eq!(called(&e, 0x0097_3fd0), vec![vec![PROCESS_LISTS, accepted]]);
        assert_eq!(called(&e, 0x00b0_0000), vec![vec![refused, 1]]);
    }

    #[test]
    fn temp_effects_need_the_data_handler_object() {
        let mut e = engine();
        let this = perceived_world(&mut e, &[(0x22, 1)]);
        e.map(0x011a_0000, 0x6_0000);
        e.register(0x007b_8f10, |_, _| int(0));
        e.call(0x0093_dd80, &args![this]);
        assert!(called(&e, 0x00aa_13e0).is_empty());
    }

    /// The messages the fast-travel check showed: (setting object, icon).
    fn shown_messages(e: &Engine) -> Vec<(u32, u32)> {
        let settings: Vec<u32> = called(e, 0x0040_3df0).iter().map(|w| w[0]).collect();
        let icons: Vec<u32> = called(e, 0x0070_52f0).iter().map(|w| w[2]).collect();
        settings.into_iter().zip(icons).collect()
    }

    /// The world of the fast-travel check where everything is allowed:
    /// the player singleton is `this` (travel flag set, health and the
    /// other value 5, no lists, no active effects), the cell test allows.
    fn travel_check_world(e: &mut Engine) -> Ptr<PlayerCharacter> {
        let this = player(e);
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x011a_0000, 0x6_0000);
        e.set_global(PLAYER_SINGLETON, this.addr());
        e.set_global(0x0101_2070, 1.0f64);
        e.set_global(TWO_FLOAT, 2.0f32);
        e.mem.set_u8(this.addr() + 0x66d, 1);
        stub(
            e,
            &[
                0x0057_5d10,
                0x0097_64a0,
                0x0097_1c30,
                0x0082_56d0,
                0x0040_3df0,
                0x0040_6d00,
                0x0070_52f0,
                0x008d_8520,
                0x005e_58f0,
                0x0088_49c0,
                0x008d_6f30,
                0x0054_4520,
                0x0058_6210,
                LIST_CLEAR,
                LIST_DELETING_DESTRUCT,
            ],
        );
        e.register(0x0057_5d70, |_, _| int(0x7300));
        // The player: no entry-point check (virtual +0x358 false); the
        // owner's virtual +0x8 gives 5; the magic target's +0x8 gives no effects.
        e.register_double(0x00b0_0358, |_, _| int(0));
        e.register_double(0x00b0_0008, |_, _| int(5));
        e.register_double(0x00b0_1008, |_, _| int(0));
        put_vtable(e, this.addr(), 0x0200_0000, &[(0x358, 0x00b0_0358)]);
        put_vtable(e, this.addr() + 0xa4, 0x0201_0000, &[(8, 0x00b0_0008)]);
        put_vtable(e, this.addr() + 0x94, 0x0202_0000, &[(8, 0x00b0_1008)]);
        this
    }

    #[test]
    fn fast_travel_is_allowed_when_nothing_forbids_it() {
        let mut e = engine();
        let this = travel_check_world(&mut e);
        assert!(e.call(0x0093_d660, &args![this]).bool());
        assert!(shown_messages(&e).is_empty());
    }

    #[test]
    fn fast_travel_is_refused_where_the_process_lists_say_so() {
        let mut e = engine();
        let this = travel_check_world(&mut e);
        e.register(0x0057_5d10, |_, _| int(1));
        e.register(0x0097_64a0, |_, a| int(a[1]));
        assert!(!e.call(0x0093_d660, &args![this]).bool());
        assert_eq!(shown_messages(&e), vec![(0x011d_3dd8, FAST_TRAVEL_ICON)]);
        // The interior flag was passed on and the text was formatted.
        assert_eq!(called(&e, 0x0097_64a0), vec![vec![PROCESS_LISTS, 1]]);
        let text = &called(&e, 0x0040_6d00)[0];
        assert_eq!(text[1], 0x1f4);
        let message = &called(&e, 0x0070_52f0)[0];
        assert_eq!((message[0], message[1], message[3]), (text[0], 0, 0));
        assert_eq!(f(message, 4), 2.0);
        assert_eq!(message[5], 0);
        assert!(called(&e, 0x0097_1c30).is_empty());
    }

    #[test]
    fn fast_travel_is_refused_while_a_list_is_pending_and_clears_it() {
        let mut e = engine();
        let this = travel_check_world(&mut e);
        e.register(0x0097_1c30, |_, _| int(0x5200));
        assert!(!e.call(0x0093_d660, &args![this]).bool());
        assert_eq!(shown_messages(&e), vec![(0x011d_4114, FAST_TRAVEL_ICON)]);
        assert_eq!(
            called(&e, 0x0097_1c30),
            vec![vec![PROCESS_LISTS, this.addr(), 0x15, 0]]
        );
        assert_eq!(called(&e, LIST_CLEAR), vec![vec![0x5200]]);
        assert_eq!(called(&e, LIST_DELETING_DESTRUCT), vec![vec![0x5200, 1]]);
    }

    #[test]
    fn an_empty_pending_list_does_not_forbid_fast_travel() {
        let mut e = engine();
        let this = travel_check_world(&mut e);
        e.register(0x0097_1c30, |_, _| int(0x5200));
        e.register(0x0082_56d0, |_, _| int(1));
        // With the list at its end the check goes on.
        assert!(e.call(0x0093_d660, &args![this]).bool());
        assert!(shown_messages(&e).is_empty());
    }

    /// A magic target whose effect list is a single node `0x5100` whose
    /// item is an object whose virtual +0x3C returns `forbids`.
    fn effect_world(e: &mut Engine, forbids: bool) -> Ptr<PlayerCharacter> {
        let this = travel_check_world(e);
        let node = e.mem.alloc(8);
        let effect = e.mem.alloc(8);
        e.mem.set_u32(node, effect);
        e.register_double(0x00b0_1008, move |_, _| int(node));
        e.register(0x008d_8520, |_, _| int(1));
        e.register_double(LIST_ITEM_ADDRESS, move |_, a| int(a[0]));
        e.register(LIST_NEXT, |_, _| int(0));
        e.register_double(0x00b0_003c, move |_, _| int(forbids as u32));
        put_vtable(e, effect, 0x0203_0000, &[(0x3c, 0x00b0_003c)]);
        this
    }

    #[test]
    fn fast_travel_is_refused_when_an_active_effect_forbids_it() {
        let mut e = engine();
        let this = effect_world(&mut e, true);
        assert!(!e.call(0x0093_d660, &args![this]).bool());
        assert_eq!(shown_messages(&e), vec![(0x011d_255c, FAST_TRAVEL_ICON)]);
    }

    #[test]
    fn fast_travel_ignores_effects_that_do_not_forbid_it() {
        let mut e = engine();
        let this = effect_world(&mut e, false);
        assert!(e.call(0x0093_d660, &args![this]).bool());
        assert_eq!(called(&e, 0x00b0_003c).len(), 1);
        assert!(shown_messages(&e).is_empty());
    }

    #[test]
    fn fast_travel_effects_are_only_checked_with_the_acquire_object() {
        let mut e = engine();
        let this = effect_world(&mut e, true);
        e.register(0x008d_8520, |_, _| int(0));
        assert!(e.call(0x0093_d660, &args![this]).bool());
        assert!(called(&e, 0x00b0_003c).is_empty());
    }

    #[test]
    fn fast_travel_needs_the_travel_flag() {
        let mut e = engine();
        let this = travel_check_world(&mut e);
        e.mem.set_u8(this.addr() + 0x66d, 0);
        assert!(!e.call(0x0093_d660, &args![this]).bool());
        assert_eq!(shown_messages(&e), vec![(0x011d_3630, FAST_TRAVEL_ICON)]);
    }

    #[test]
    fn fast_travel_waits_for_the_entry_point_delay() {
        let mut e = engine();
        let this = travel_check_world(&mut e);
        e.register_double(0x00b0_0358, |_, _| int(1));
        e.register(0x005e_58f0, |e, a| {
            e.mem.set_f32(a[2], 0.5);
            Ret::default()
        });
        assert!(!e.call(0x0093_d660, &args![this]).bool());
        // The message has no icon.
        assert_eq!(shown_messages(&e), vec![(0x011d_50c8, 0)]);
        let entry = &called(&e, 0x005e_58f0)[0];
        assert_eq!((entry[0], entry[1]), (0x33, this.addr()));
        // A delay of at least one allows it.
        let mut e = engine();
        let this = travel_check_world(&mut e);
        e.register_double(0x00b0_0358, |_, _| int(1));
        e.register(0x005e_58f0, |e, a| {
            e.mem.set_f32(a[2], 1.0);
            Ret::default()
        });
        assert!(e.call(0x0093_d660, &args![this]).bool());
    }

    #[test]
    fn fast_travel_needs_both_actor_values() {
        for (first, second) in [(0, 5), (5, 0)] {
            let mut e = engine();
            let this = travel_check_world(&mut e);
            e.register_double(0x00b0_0008, move |_, a| {
                int(if a[1] == 0x16 { first } else { second })
            });
            assert!(!e.call(0x0093_d660, &args![this]).bool());
            assert_eq!(shown_messages(&e), vec![(0x011d_255c, FAST_TRAVEL_ICON)]);
        }
    }

    #[test]
    fn fast_travel_is_refused_when_the_player_is_busy() {
        let mut e = engine();
        let this = travel_check_world(&mut e);
        e.register(0x0088_49c0, |_, _| int(1));
        assert!(!e.call(0x0093_d660, &args![this]).bool());
        assert_eq!(shown_messages(&e), vec![(0x011d_4120, FAST_TRAVEL_ICON)]);
    }

    #[test]
    fn fast_travel_follows_the_parent_cell_test() {
        // With a parent cell: allowed unless `00544520` says no.
        for (refuses, allowed) in [(0, true), (1, false)] {
            let mut e = engine();
            let this = travel_check_world(&mut e);
            e.register(0x008d_6f30, |_, _| int(0x3300));
            e.register_double(0x0054_4520, move |_, _| int(refuses));
            assert_eq!(e.call(0x0093_d660, &args![this]).bool(), allowed);
            assert_eq!(called(&e, 0x0054_4520), vec![vec![0x3300]]);
            assert_eq!(shown_messages(&e).len(), !allowed as usize);
            if !allowed {
                assert_eq!(shown_messages(&e), vec![(0x011d_4738, FAST_TRAVEL_ICON)]);
            }
        }
    }

    #[test]
    fn fast_travel_follows_the_world_space_test_without_a_parent_cell() {
        for (refuses, allowed) in [(0, true), (1, false)] {
            let mut e = engine();
            let this = travel_check_world(&mut e);
            e.register_double(0x0058_6210, move |_, _| int(refuses));
            assert_eq!(e.call(0x0093_d660, &args![this]).bool(), allowed);
            assert_eq!(called(&e, 0x0058_6210), vec![vec![0x7300]]);
        }
        // Without a world space either, it is refused.
        let mut e = engine();
        let this = travel_check_world(&mut e);
        e.register(0x0057_5d70, |_, _| int(0));
        assert!(!e.call(0x0093_d660, &args![this]).bool());
        assert_eq!(shown_messages(&e), vec![(0x011d_4738, FAST_TRAVEL_ICON)]);
    }

    /// The world of `CenterOnCell`: the exterior world (PositionPlayer and
    /// the cell clearing run for real) with the lookups stubbed.
    fn center_world(e: &mut Engine) -> PositionWorld {
        let world = exterior_world(e);
        stub(
            e,
            &[
                0x0040_fbe0,
                0x0046_1ae0,
                0x0046_1cf0,
                0x0054_4c30,
                0x0054_4c60,
                0x0054_cfd0,
                0x0045_72e0,
            ],
        );
        world
    }

    /// `0054cfd0` fills the position (1, 2, 3) and the rotation (4, 5, 6).
    fn register_placement(e: &mut Engine) {
        e.register(0x0054_cfd0, |e, a| {
            for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, v);
            }
            for (i, v) in [4.0f32, 5.0, 6.0].into_iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * i as u32, v);
            }
            Ret::default()
        });
    }

    #[test]
    fn centering_on_an_interior_cell_places_the_player() {
        let mut e = engine();
        let world = center_world(&mut e);
        register_placement(&mut e);
        e.register(0x0042_5fd0, |_, _| int(1));
        let placed_at = watch_floats(&mut e, 0x00b0_02a8, 1, 3);
        let turned = watch_floats(&mut e, 0x0086_d490, 1, 3);
        e.call(0x0093_db60, &args![world.this, 0u32, 0x6100u32]);
        assert_eq!(called(&e, 0x0040_fbe0), vec![vec![world.this.addr()]]);
        assert!(called(&e, 0x0046_1ae0).is_empty());
        let placement = &called(&e, 0x0054_cfd0)[0];
        assert_eq!(placement[0], 0x6100);
        assert_eq!(called(&e, 0x0057_5bb0)[0], vec![world.this.addr(), 0x6100]);
        assert_eq!(placed_at.borrow()[0], vec![1.0, 2.0, 3.0]);
        assert_eq!(turned.borrow()[0], vec![4.0, 5.0, 6.0]);
        // The weather is reset by the placement.
        assert_eq!(called(&e, 0x0046_dd00).len(), 1);
        // An interior cell has no land height to adjust to.
        assert!(called(&e, 0x0045_72e0).is_empty());
    }

    #[test]
    fn centering_on_a_cell_by_editor_id_looks_it_up() {
        let mut e = engine();
        let world = center_world(&mut e);
        register_placement(&mut e);
        e.set_global(DATA_HANDLER, 0x5800u32);
        e.register(0x0042_5fd0, |_, _| int(1));
        e.register(0x0046_1ae0, |_, _| int(0x6100));
        e.call(0x0093_db60, &args![world.this, 0x9990u32, 0u32]);
        assert_eq!(called(&e, 0x0046_1ae0), vec![vec![0x5800, 0x9990]]);
        assert_eq!(called(&e, 0x0057_5bb0)[0], vec![world.this.addr(), 0x6100]);
    }

    #[test]
    fn centering_on_an_exterior_cell_loads_it_and_sets_the_land_height() {
        for (height, expected) in [(12.5f32, 12.5f32), (-3.0, 0.0)] {
            let mut e = engine();
            let world = center_world(&mut e);
            register_placement(&mut e);
            e.set_global(EXTERIOR_CELL_LOADER, 0x5900u32);
            e.register(0x0054_ddd0, |_, _| int(0x7200));
            e.register(0x0054_4c30, |_, _| int(7));
            e.register(0x0054_4c60, |_, _| int(9));
            e.register(0x0045_11e0, |_, _| int(1));
            e.register_double(0x0045_72e0, move |e, a| {
                e.mem.set_f32(a[2], height);
                Ret::default()
            });
            let placed_at = watch_floats(&mut e, 0x00b0_02a8, 1, 3);
            e.call(0x0093_db60, &args![world.this, 0u32, 0x6100u32]);
            assert_eq!(called(&e, 0x0052_8540), vec![vec![0x5900]]);
            // The cell's world space is loaded at the cell's coordinates.
            assert_eq!(called(&e, 0x0058_5b30), vec![vec![0x7200, 7, 9]]);
            assert_eq!(called(&e, 0x0045_72e0)[0][0], 0x5000);
            // The last placement has the land height as z (the first is the
            // placement in PositionPlayer).
            let sets = placed_at.borrow();
            assert_eq!(sets.last().unwrap(), &vec![1.0, 2.0, expected]);
        }
    }

    #[test]
    fn centering_on_an_unknown_editor_id_loads_the_cell_from_the_data_files() {
        let mut e = engine();
        let world = center_world(&mut e);
        register_placement(&mut e);
        e.set_global(DATA_HANDLER, 0x5800u32);
        e.register(0x0046_1ae0, |_, _| int(0));
        e.register(0x0046_1cf0, |e, a| {
            e.mem.set_u32(a[2], 3);
            e.mem.set_u32(a[3], 4);
            int(0x7200)
        });
        e.register(0x0058_5b30, |_, _| int(0x6400));
        e.register(0x0045_11e0, |_, _| int(0));
        e.call(0x0093_db60, &args![world.this, 0x9990u32, 0u32]);
        assert_eq!(called(&e, 0x0046_1cf0)[0][..2], [0x5800, 0x9990]);
        // The grid is cleared for the world space, then the cell loaded at
        // the coordinates the data files gave.
        assert_eq!(called(&e, 0x0045_7d70)[0], vec![0x5000, 1, 0x7200, 0]);
        assert_eq!(called(&e, 0x0058_5b30), vec![vec![0x7200, 3, 4]]);
        assert_eq!(called(&e, 0x0057_5bb0)[0], vec![world.this.addr(), 0x6400]);
    }

    #[test]
    fn centering_on_nothing_does_nothing_more() {
        let mut e = engine();
        let world = center_world(&mut e);
        register_placement(&mut e);
        e.set_global(DATA_HANDLER, 0x5800u32);
        e.call(0x0093_db60, &args![world.this, 0x9990u32, 0u32]);
        assert!(called(&e, 0x0054_cfd0).is_empty());
        assert!(called(&e, 0x0057_5bb0).is_empty());
    }

    #[test]
    fn spawning_effects_for_the_four_iterators() {
        let mut e = engine();
        let this = player(&mut e);
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x011a_0000, 0x6_0000);
        e.map(0x5000, 0x2000);
        e.map(0x0300_0000, 0x100);
        e.set_global(DATA_HANDLER, 0x5800u32);
        e.set_global(0x011c_95c8, 0x6600u32);
        e.mem.set_f32(0x0300_0000, 0.5);
        e.register(SETTING_FLOAT_GETTER, |_, _| int(0x0300_0000));
        e.mem.set_u32(0x5800 + 0x634, 0x9a00);
        stub(&mut e, &[0x0052_71f0, 0x0052_72b0, 0x0097_3fd0]);
        // The walker yields two actors in all, to the first iterator.
        let remaining = Rc::new(RefCell::new(vec![0x31u32, 0x32]));
        let queue = remaining.clone();
        e.register_double(0x0052_73c0, move |e, a| {
            if queue.borrow().is_empty() {
                int(0)
            } else {
                let actor = queue.borrow_mut().remove(0);
                e.mem.set_u32(a[2], actor);
                int(1)
            }
        });
        let blocks = Rc::new(RefCell::new(vec![0x6001u32, 0x6002]));
        let queue = blocks.clone();
        e.register_double(0x00aa_13e0, move |_, _| int(queue.borrow_mut().remove(0)));
        let accepted = e.mem.alloc(16);
        let refused = e.mem.alloc(16);
        let effects = Rc::new(RefCell::new(vec![accepted, refused]));
        let queue = effects.clone();
        e.register_double(0x0081_f580, move |_, _| int(queue.borrow_mut().remove(0)));
        e.register_double(0x00b0_00c4, move |_, a| int((a[0] == accepted) as u32));
        e.register_double(0x00b0_0000, |_, _| Ret::default());
        put_vtable(&mut e, accepted, 0x0270_0000, &[(0xc4, 0x00b0_00c4)]);
        put_vtable(
            &mut e,
            refused,
            0x0271_0000,
            &[(0xc4, 0x00b0_00c4), (0, 0x00b0_0000)],
        );
        e.call(0x0093_df50, &args![this]);
        // Four iterators were built, in the order of the kinds.
        let kinds: Vec<u32> = called(&e, 0x0052_71f0).iter().map(|w| w[1]).collect();
        assert_eq!(kinds, vec![0x28, 0x29, 0x2f, 0x2e]);
        assert!(called(&e, 0x0052_71f0).iter().all(|w| w[2] == this.addr()));
        // Each iterator is walked until it is done: two yields and four ends.
        let walks = called(&e, 0x0052_73c0);
        assert_eq!(walks.len(), 6);
        assert_eq!((walks[0][0], walks[0][4]), (0x6600, 3));
        let created = called(&e, 0x0081_f580);
        assert_eq!(created.len(), 2);
        assert_eq!((created[0][1], created[0][2]), (0x31, 0x9a00));
        assert_eq!(f(&created[0], 3), 0.5);
        assert_eq!(called(&e, 0x0097_3fd0), vec![vec![PROCESS_LISTS, accepted]]);
        assert_eq!(called(&e, 0x00b0_0000), vec![vec![refused, 1]]);
        // The iterators are released in reverse.
        let released: Vec<u32> = called(&e, 0x0052_72b0)
            .iter()
            .map(|w| w[0] - 0x1c)
            .collect();
        let built: Vec<u32> = called(&e, 0x0052_71f0).iter().map(|w| w[0]).collect();
        assert_eq!(released, built.iter().rev().copied().collect::<Vec<_>>());
    }

    #[test]
    fn no_effects_are_spawned_without_the_object() {
        let mut e = engine();
        let this = player(&mut e);
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x011a_0000, 0x6_0000);
        e.map(0x5000, 0x2000);
        e.set_global(DATA_HANDLER, 0x5800u32);
        e.map(0x0300_0000, 0x100);
        e.register(SETTING_FLOAT_GETTER, |_, _| int(0x0300_0000));
        stub(&mut e, &[0x0052_71f0, 0x0052_72b0, 0x0052_73c0]);
        e.call(0x0093_df50, &args![this]);
        assert!(called(&e, 0x0052_73c0).is_empty());
        assert_eq!(called(&e, 0x0052_72b0).len(), 4);
    }

    /// The world of `SetSlowMoCamera`: a kill camera setting, sound
    /// functions recorded, the VATS object quiet.
    fn slow_mo_world(e: &mut Engine, setting: u32) -> Ptr<PlayerCharacter> {
        let this = player(e);
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x011a_0000, 0x6_0000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(PLAYER_SINGLETON, this.addr());
        e.set(this, PlayerCharacter::eKillCameraSetting, setting);
        stub(
            e,
            &[
                SOUND_HANDLE_CONSTRUCT,
                SOUND_HANDLE_ASSIGN,
                SOUND_HANDLE_DESTRUCT,
                SOUND_HANDLE_PLAY,
                0x008d_6f30,
                0x0084_e3a0,
                0x0044_ddc0,
                0x009c_a2c0,
                0x008b_bc10,
                0x0070_9c40,
                0x004e_af60,
            ],
        );
        e.register(0x0045_3a70, |_, _| int(0x0400_0000));
        e.register(0x00ad_7550, |_, a| int(a[1]));
        e.map(0x0300_0000, 0x100);
        e.mem.set_f32(0x0300_0000, 3.0);
        e.register(SETTING_FLOAT_GETTER, |_, _| int(0x0300_0000));
        e.register_double(0x00b0_0008, |_, _| int(1));
        put_vtable(e, this.addr() + 0xa4, 0x0201_0000, &[(8, 0x00b0_0008)]);
        this
    }

    #[test]
    fn slow_motion_does_nothing_without_a_kill_camera_setting() {
        let mut e = engine();
        let this = slow_mo_world(&mut e, 0);
        e.call(0x0093_e530, &args![this, 1u32, 2.0f32, false, 0u32]);
        assert_eq!(call_addresses(&e), vec![0x0093_e530]);
    }

    #[test]
    fn slow_motion_stops_in_the_special_cell() {
        let mut e = engine();
        let this = slow_mo_world(&mut e, 1);
        e.register(0x008d_6f30, |_, _| int(0x3300));
        e.register(0x0084_e3a0, |_, _| int(0x0016_1e98));
        e.call(0x0093_e530, &args![this, 1u32, 2.0f32, false, 0u32]);
        assert_eq!(called(&e, 0x0084_e3a0), vec![vec![0x3300]]);
        assert!(called(&e, SOUND_HANDLE_CONSTRUCT).is_empty());
    }

    #[test]
    fn slow_motion_plays_a_sound_and_scales_the_time_for_other_settings() {
        let mut e = engine();
        let this = slow_mo_world(&mut e, 1);
        e.set_global(0x0102_90c0, 0.4f64);
        e.call(0x0093_e530, &args![this, 1u32, 5.0f32, false, 0u32]);
        let lookup = &called(&e, 0x00ad_7550)[0];
        assert_eq!(
            (lookup[0], lookup[2], lookup[3]),
            (0x0400_0000, 0x0106_f370, 0x121)
        );
        let handle = called(&e, SOUND_HANDLE_CONSTRUCT)[0][0];
        assert_eq!(
            called(&e, SOUND_HANDLE_ASSIGN),
            vec![vec![handle, lookup[1]]]
        );
        assert_eq!(called(&e, SOUND_HANDLE_PLAY), vec![vec![handle, 0]]);
        assert_eq!(called(&e, SOUND_HANDLE_DESTRUCT).len(), 2);
        assert_eq!(
            e.get(this, PlayerCharacter::fTimeInSlowMoCam),
            (5.0f32 as f64 * 0.4f64) as f32
        );
        assert!(called(&e, 0x009c_a2c0).is_empty());
    }

    #[test]
    fn slow_motion_does_nothing_more_without_a_mode_or_during_vats() {
        let mut e = engine();
        let this = slow_mo_world(&mut e, 2);
        e.call(0x0093_e530, &args![this, 0u32, 5.0f32, false, 0u32]);
        assert_eq!(e.get(this, PlayerCharacter::fTimeInSlowMoCam), 0.0);
        assert_eq!(called(&e, SOUND_HANDLE_DESTRUCT).len(), 2);
        let mut e = engine();
        let this = slow_mo_world(&mut e, 2);
        e.register(0x0044_ddc0, |_, _| int(1));
        e.call(0x0093_e530, &args![this, 1u32, 5.0f32, false, 0u32]);
        assert_eq!(e.get(this, PlayerCharacter::fTimeInSlowMoCam), 0.0);
    }

    #[test]
    fn slow_motion_for_setting_two_starts_the_vats_camera() {
        let mut e = engine();
        let this = slow_mo_world(&mut e, 2);
        e.register(0x004e_af60, |_, _| int(0));
        e.register(0x008b_bc10, |_, _| int(1));
        e.set_global(0x011f_21d1, 0u8);
        e.call(0x0093_e530, &args![this, 3u32, 5.0f32, false, 0x77u32]);
        assert_eq!(e.get(this, PlayerCharacter::fTimeInSlowMoCam), 5.0);
        assert_eq!(called(&e, 0x009c_a2c0), vec![vec![VATS_OBJECT, 3, 0x77]]);
        // The third-person flag comes from the player when none is saved.
        assert_eq!(e.global::<u8>(0x011f_21d1), 1);
        assert_eq!(called(&e, 0x0070_9c40), vec![vec![0]]);
    }

    #[test]
    fn slow_motion_uses_the_saved_third_person_flag() {
        let mut e = engine();
        let this = slow_mo_world(&mut e, 2);
        e.set(this, PlayerCharacter::b3rdPersonSaved, true);
        e.set(this, PlayerCharacter::bsave3rdPerson, true);
        e.call(0x0093_e530, &args![this, 3u32, 5.0f32, false, 0u32]);
        assert_eq!(e.global::<u8>(0x011f_21d1), 0);
        assert!(called(&e, 0x004e_af60).is_empty());
        assert!(called(&e, 0x0070_9c40).is_empty());
    }

    #[test]
    fn slow_motion_cooldown_decides_whether_the_queued_camera_starts() {
        // Queued with no cool-down: the cool-down is set from the setting.
        let mut e = engine();
        let this = slow_mo_world(&mut e, 1);
        e.set_global(0x0102_90c0, 0.4f64);
        e.call(0x0093_e530, &args![this, 1u32, 5.0f32, true, 0u32]);
        assert_eq!(e.get(this, PlayerCharacter::fKillCamCooldown), 3.0);
        assert!(e.get(this, PlayerCharacter::fTimeInSlowMoCam) > 0.0);
        // Queued during a cool-down: nothing starts.
        let mut e = engine();
        let this = slow_mo_world(&mut e, 1);
        e.set(this, PlayerCharacter::fKillCamCooldown, 1.0);
        e.call(0x0093_e530, &args![this, 1u32, 5.0f32, true, 0u32]);
        assert_eq!(e.get(this, PlayerCharacter::fTimeInSlowMoCam), 0.0);
        assert_eq!(e.get(this, PlayerCharacter::fKillCamCooldown), 1.0);
    }

    #[test]
    fn slow_motion_restores_the_time_multiplier_unless_the_player_has_the_perk() {
        let mut e = engine();
        let this = slow_mo_world(&mut e, 1);
        e.set_global(0x0102_90c0, 0.4f64);
        e.register_double(0x00b0_0008, |_, _| int(0));
        e.set_global(0x011a_c3a4, 1.0f32);
        e.set_global(0x011a_c3a8, 0.0f32);
        e.call(0x0093_e530, &args![this, 1u32, 5.0f32, false, 0u32]);
        assert_eq!(
            called(&e, 0x00b0_0008),
            vec![vec![this.addr() + 0xa4, 0x33]]
        );
        assert_eq!(e.global::<f32>(0x011a_c3a8), 1.0);
    }

    #[test]
    fn the_time_multiplier_is_saved_and_applied() {
        let mut e = engine();
        e.map(0x011a_0000, 0x6_0000);
        e.set_global(0x011a_c3a4, 0.25f32);
        e.call(0x0093_e750, &args![]);
        assert_eq!(e.global::<f32>(0x011a_c3a8), 0.25);
        stub(&mut e, &[0x00aa_4db0]);
        e.call(0x0093_e840, &args![GAME_TIMER]);
        let call = &called(&e, 0x00aa_4db0)[0];
        assert_eq!((call[0], f(call, 1), call[2]), (GAME_TIMER, 0.25, 0));
    }

    /// The world of `0093e770`: the animation getters and the VATS object.
    fn slow_mo_end_world(e: &mut Engine) -> Ptr<PlayerCharacter> {
        let this = player(e);
        e.map(0x011a_0000, 0x6_0000);
        e.set_global(0x011a_c3a8, 0.5f32);
        e.set_global(PLAYER_SINGLETON, this.addr());
        stub(
            e,
            &[
                0x00aa_4db0,
                0x004e_af60,
                0x0095_0a60,
                0x0088_b030,
                0x004a_4040,
                0x0095_f530,
                0x009c_8950,
                0x009c_6ba0,
                0x009c_6c30,
            ],
        );
        e.set(this, PlayerCharacter::fTimeInSlowMoCam, 4.0);
        this
    }

    #[test]
    fn ending_the_slow_motion_restores_time_and_resets_the_timer() {
        let mut e = engine();
        let this = slow_mo_end_world(&mut e);
        e.call(0x0093_e770, &args![this, 0i32, false]);
        let multiplier = &called(&e, 0x00aa_4db0)[0];
        assert_eq!((multiplier[0], f(multiplier, 1)), (GAME_TIMER, 0.5));
        assert_eq!(e.get(this, PlayerCharacter::fTimeInSlowMoCam), 0.0);
        // No animation: nothing told 1.0.
        assert_eq!(called(&e, 0x0095_0a60), vec![vec![this.addr(), 1]]);
        assert!(called(&e, 0x0088_b030).is_empty());
        assert!(called(&e, 0x009c_8950).is_empty());
    }

    #[test]
    fn ending_the_slow_motion_resets_the_animation_when_there_is_one() {
        let mut e = engine();
        let this = slow_mo_end_world(&mut e);
        e.register(0x004e_af60, |_, _| int(1));
        e.register(0x0095_0a60, |_, a| int(0x4a00 + a[1]));
        e.call(0x0093_e770, &args![this, 0i32, false]);
        assert_eq!(
            called(&e, 0x0095_0a60),
            vec![vec![this.addr(), 0], vec![this.addr(), 0]]
        );
        let reset = &called(&e, 0x0088_b030)[0];
        assert_eq!((reset[0], f(reset, 1)), (0x4a00, 1.0));
    }

    #[test]
    fn ending_the_vats_slow_motion_quits_the_playback() {
        let mut e = engine();
        let this = slow_mo_end_world(&mut e);
        e.call(0x0093_e770, &args![this, 2i32, true]);
        assert_eq!(called(&e, 0x0095_f530), vec![vec![this.addr(), 0, 3]]);
        assert_eq!(called(&e, 0x009c_8950), vec![vec![VATS_OBJECT, 0, 1]]);
        assert_eq!(called(&e, 0x009c_6ba0), vec![vec![VATS_OBJECT]]);
        assert_eq!(called(&e, 0x009c_6c30), vec![vec![VATS_OBJECT, 0, 1]]);
        assert_eq!(e.global::<u8>(0x011f_21d0), 1);
        assert!(called(&e, 0x004a_4040).is_empty());
    }

    #[test]
    fn ending_the_vats_slow_motion_without_the_flag_asks_first() {
        let mut e = engine();
        let this = slow_mo_end_world(&mut e);
        e.register(0x004a_4040, |_, _| int(1));
        e.call(0x0093_e770, &args![this, 2i32, false]);
        assert!(called(&e, 0x009c_8950).is_empty());
        assert_eq!(e.get(this, PlayerCharacter::fTimeInSlowMoCam), 0.0);
        e.register(0x004a_4040, |_, _| int(0));
        e.call(0x0093_e770, &args![this, 2i32, false]);
        assert_eq!(called(&e, 0x009c_8950).len(), 1);
    }
    /// Every function `Update` calls by address (outside the vtable calls).
    const UPDATE_CALLEES: &[u32] = &[
        0x0040_1170,
        0x0040_3df0,
        0x0040_4010,
        0x0040_4eb0,
        0x0040_4ee0,
        0x0040_8840,
        0x0040_8d60,
        0x0041_6870,
        0x0041_8940,
        0x0041_8a40,
        0x0041_a090,
        0x0041_cd70,
        0x0041_cda0,
        0x0043_01b0,
        0x0043_09e0,
        0x0043_6aa0,
        0x0043_7bd0,
        0x0043_7bf0,
        0x0043_9180,
        0x0043_9e90,
        0x0043_9ef0,
        0x0043_9f50,
        0x0043_d4d0,
        0x0044_6b50,
        0x0044_7950,
        0x0044_ddc0,
        0x0045_0b80,
        0x0045_6c70,
        0x0045_78c0,
        0x0045_7990,
        0x0045_7fe0,
        0x0045_81e0,
        0x0045_8b20,
        0x0045_c670,
        0x0046_4e10,
        0x0047_4a80,
        0x0048_1610,
        0x0048_1680,
        0x0048_18e0,
        0x0048_21a0,
        0x0048_39c0,
        0x0048_3a00,
        0x0048_f7f0,
        0x0049_1040,
        0x0049_38e0,
        0x0049_74a0,
        0x0049_85f0,
        0x0049_94f0,
        0x0049_97b0,
        0x004a_0c10,
        0x004a_4040,
        0x004a_7290,
        0x004a_d030,
        0x004b_3ab0,
        0x004b_3ae0,
        0x004b_71d0,
        0x004b_9930,
        0x004b_da70,
        0x004c_0c90,
        0x004d_1360,
        0x004e_44b0,
        0x004e_af60,
        0x004f_8960,
        0x004f_e160,
        0x0050_8070,
        0x0050_98e0,
        0x0050_d4a0,
        0x0051_1840,
        0x0051_9020,
        0x0052_4ac0,
        0x0052_4b60,
        0x0052_5430,
        0x0052_5980,
        0x0052_99a0,
        0x0052_9c90,
        0x0054_44c0,
        0x0054_74b0,
        0x0054_a070,
        0x0055_5c20,
        0x0055_8310,
        0x0055_9450,
        0x0056_10f0,
        0x0056_15d0,
        0x0056_20e0,
        0x0056_8680,
        0x0056_9160,
        0x0056_f930,
        0x0057_1760,
        0x0057_21e0,
        0x0057_3170,
        0x0057_5d10,
        0x0057_b460,
        0x0058_9850,
        0x0059_5f50,
        0x0059_5fc0,
        0x005a_03d0,
        0x005a_03f0,
        0x005a_1e50,
        0x005a_2030,
        0x005c_0880,
        0x005c_7870,
        0x005d_2860,
        0x005d_43c0,
        0x005e_2a70,
        0x005e_58f0,
        0x005f_2420,
        0x005f_2670,
        0x005f_4db0,
        0x0061_55f0,
        0x0063_c8a0,
        0x0064_50c0,
        0x0068_a7d0,
        0x0069_3ef0,
        0x006a_9540,
        0x006e_cd40,
        0x0070_2360,
        0x0070_3350,
        0x0070_38a0,
        0x0070_3e80,
        0x0070_50d0,
        0x0070_52f0,
        0x0070_54f0,
        0x0070_5640,
        0x0070_5a00,
        0x0070_62e0,
        0x0070_9430,
        0x0070_9bc0,
        0x0070_9be0,
        0x0070_9c40,
        0x0071_6440,
        0x0071_7a40,
        0x0072_6070,
        0x0077_1700,
        0x0077_2d10,
        0x0077_8930,
        0x007a_a720,
        0x007a_f430,
        0x007b_8f10,
        0x007d_0a70,
        0x007f_52c0,
        0x0080_fb00,
        0x0082_2e00,
        0x0082_ec10,
        0x0084_d030,
        0x0084_e3a0,
        0x0085_09a0,
        0x0085_09f0,
        0x0085_1230,
        0x0085_12f0,
        0x0086_fd90,
        0x0087_7720,
        0x0087_ce50,
        0x0088_43a0,
        0x0088_46e0,
        0x0088_49c0,
        0x0088_4f80,
        0x0088_5520,
        0x0088_6360,
        0x0088_85e0,
        0x0088_b030,
        0x0088_c830,
        0x0088_d640,
        0x0089_4900,
        0x0089_4cc0,
        0x0089_4d60,
        0x0089_4f90,
        0x0089_50f0,
        0x0089_5110,
        0x0089_f4e0,
        0x008a_0b10,
        0x008a_16d0,
        0x008a_6840,
        0x008a_6970,
        0x008a_73e0,
        0x008a_7570,
        0x008a_dcb0,
        0x008b_a600,
        0x008b_b650,
        0x008b_bc10,
        0x008b_bdb0,
        0x008b_c240,
        0x008c_3c40,
        0x008c_e390,
        0x008d_3550,
        0x008d_51f0,
        0x008d_5210,
        0x008d_6f30,
        0x008d_8520,
        0x008f_ec10,
        0x0092_2650,
        0x0093_06d0,
        0x0093_1d70,
        0x0094_4270,
        0x0094_42a0,
        0x0094_42c0,
        0x0094_42e0,
        0x0094_4300,
        0x0094_4320,
        0x0094_4340,
        0x0094_4360,
        0x0094_4380,
        0x0094_43a0,
        0x0094_43c0,
        0x0094_43f0,
        0x0094_4400,
        0x0094_4430,
        0x0094_4460,
        0x0094_44a0,
        0x0094_44d0,
        0x0094_45b0,
        0x0094_62c0,
        0x0094_7b10,
        0x0094_81d0,
        0x0094_8310,
        0x0094_ae40,
        0x0094_dbe0,
        0x0094_df80,
        0x0095_0090,
        0x0095_00a0,
        0x0095_0110,
        0x0095_0340,
        0x0095_03d0,
        0x0095_0530,
        0x0095_0a60,
        0x0095_1a10,
        0x0095_3c50,
        0x0095_3c80,
        0x0095_3d00,
        0x0095_3d40,
        0x0095_de30,
        0x0095_f6a0,
        0x0095_f6c0,
        0x0096_04f0,
        0x0096_13c0,
        0x0096_2190,
        0x0096_2350,
        0x0096_2d00,
        0x0096_3bf0,
        0x0096_3e00,
        0x0096_3eb0,
        0x0096_4100,
        0x0096_4260,
        0x0096_6a20,
        0x0096_73d0,
        0x0096_78a0,
        0x0096_7950,
        0x0096_7a00,
        0x0096_7ae0,
        0x0096_7da0,
        0x0096_9c30,
        0x0096_e570,
        0x0097_3de0,
        0x0097_4af0,
        0x0097_64a0,
        0x009a_b9a0,
        0x009b_b080,
        0x009c_71c0,
        0x009c_8950,
        0x009c_a1f0,
        0x009e_a360,
        0x009e_a570,
        0x00a2_3390,
        0x00a2_4280,
        0x00a2_4660,
        0x00aa_4db0,
        0x00ad_73b0,
        0x00ad_7480,
        0x00ad_7c80,
        0x00ad_7da0,
        0x00ad_7ec0,
        0x00ad_8930,
        0x00b5_d9f0,
        0x00c3_dfa0,
        0x00ca_1410,
        0x00ec_43fb,
        0x00ec_7d40,
        SOUND_HANDLE_CONSTRUCT,
        SOUND_HANDLE_ASSIGN,
        SOUND_HANDLE_DESTRUCT,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_STOP,
        SOUND_HANDLE_IS_VALID,
        SOUND_HANDLE_FADE_OUT_AND_RELEASE,
        AUDIO_INSTANCE,
        AUDIO_GET_SOUND_HANDLE_BY_NAME,
        CELL_IS_INTERIOR,
        LIST_CONSTRUCT,
        LIST_DESTRUCT,
        LIST_IS_EMPTY,
        LIST_ITEM_ADDRESS,
        LIST_NEXT,
        NI_POINTER_ASSIGN,
    ];

    /// A world in which `Update` runs from start to end with everything it
    /// calls doing nothing: all the globals are 0, every setting is 0.0, and
    /// every virtual function of the player and of its process returns 0.
    fn update_world(e: &mut Engine) -> Ptr<PlayerCharacter> {
        let this = player(e);
        e.map(0x0100_0000, 0x0130_0000);
        e.map(0, 0x1000);
        e.register(0, |_, _| Ret::default());
        e.map(0x0300_0000, 0x1000);
        for addr in UPDATE_CALLEES {
            e.register(*addr, |_, _| Ret::default());
        }
        for getter in [
            SETTING_FLOAT_GETTER,
            SETTING_BYTE_GETTER,
            SETTING_INT_GETTER,
        ] {
            e.register(getter, |_, _| int(0x0300_0000));
        }
        e.register(0x00b0_0000, |_, _| Ret::default());
        let slots: Vec<(u32, u32)> = (0..0x200).map(|i| (i * 4, 0x00b0_0000)).collect();
        put_vtable(e, this.addr(), 0x0201_0000, &slots);
        let process = Ptr::<PlayerCharacter>::new(e.mem.alloc(0x40));
        put_vtable(e, process.addr(), 0x0202_0000, &slots);
        put_vtable(e, this.addr() + ACTOR_VALUE_OWNER_BASE, 0x0203_0000, &slots);
        e.mem.set_u32(this.addr() + 0x68, process.addr());
        e.set_global(PLAYER_SINGLETON, this.addr());
        this
    }

    #[test]
    fn update_in_a_quiet_world_runs_to_the_end_and_releases_its_locals() {
        let mut e = engine();
        let this = update_world(&mut e);
        player_character_update(&mut e, this, 0.5);
        let log = call_addresses(&e);
        let n = log.len();
        assert_eq!(&log[n - 2..], &[SOUND_HANDLE_DESTRUCT, 0x0040_4ee0]);
        assert_eq!(called(&e, 0x0040_4eb0).len(), 1);
        assert_eq!(
            called(&e, 0x0040_4eb0)[0][1..],
            [0x34, 1, UPDATE_GUARD_FILE, 0xc41]
        );
        assert_eq!(called(&e, 0x0040_4ee0).len(), 1);
    }

    #[test]
    fn update_with_a_forced_activation_activates_and_ends_early() {
        let mut e = engine();
        let this = update_world(&mut e);
        e.register(0x0094_4320, |_, _| int(0x4455));
        player_character_update(&mut e, this, 0.5);
        assert_eq!(
            called(&e, 0x0057_3170),
            vec![vec![0x4455, this.addr(), 0, 0, 1]]
        );
        assert_eq!(called(&e, 0x0051_9020), vec![vec![this.addr(), 0]]);
        let log = call_addresses(&e);
        let n = log.len();
        assert_eq!(&log[n - 2..], &[SOUND_HANDLE_DESTRUCT, 0x0040_4ee0]);
        assert!(called(&e, 0x0095_f6c0).is_empty());
    }

    #[test]
    fn update_counts_the_warning_timers_down_and_resets_them_when_out() {
        let mut e = engine();
        let this = update_world(&mut e);
        e.set_global(TIMER_RESET, -1.0f32);
        e.set(this, PlayerCharacter::fStealWarningTimer, 2.0);
        e.set(this, PlayerCharacter::fPickPocketWarningTimer, 0.0);
        player_character_update(&mut e, this, 0.5);
        assert_eq!(e.get(this, PlayerCharacter::fStealWarningTimer), 1.5);
        assert_eq!(e.get(this, PlayerCharacter::fPickPocketWarningTimer), -1.0);
    }

    #[test]
    fn update_runs_the_end_of_the_frame_work_once() {
        let mut e = engine();
        let this = update_world(&mut e);
        player_character_update(&mut e, this, 0.25);
        assert_eq!(
            called(&e, 0x0095_f6c0),
            vec![vec![this.addr(), 0.25f32.to_bits()]]
        );
        assert_eq!(called(&e, 0x0055_5c20).len(), 1);
    }

    #[test]
    fn update_aims_a_drawn_gun_and_blocks_with_a_drawn_melee_weapon() {
        // 00941fea: the aim button raises the iron sights of a gun, blocks
        // with a melee weapon (`006450c0`) and, without a weapon, blocks.
        for (weapon, melee, iron_sights) in
            [(0x5100u32, 0u32, true), (0x5100, 1, false), (0, 0, false)]
        {
            let mut e = engine();
            let this = update_world(&mut e);
            let process = e.mem.u32(this.addr() + 0x68);
            let vtable = e.mem.u32(process);
            e.mem.set_u32(vtable + 0x148, 0x00b1_0000);
            e.register_double(0x00b1_0000, move |_, _| int(weapon));
            e.mem.set_u32(vtable + 0x3f8, 0x00b1_0010);
            e.register(0x00b1_0010, |_, _| int(1));
            e.register(0x0044_ddc0, |_, a| {
                int(if a[0] == 0x5100 { 0x6000 } else { 0 })
            });
            e.register_double(0x0064_50c0, move |_, _| int(melee));
            e.register(0x008a_16d0, |_, _| int(1));
            e.register(0x00a2_4660, |_, a| int(u32::from(a[1] == 6 && a[2] == 1)));
            player_character_update(&mut e, this, 0.5);
            let raised = called(&e, 0x008b_b650).contains(&vec![this.addr(), 1, 0, 0]);
            let blocked = called(&e, 0x0089_4cc0).contains(&vec![this.addr(), 1]);
            assert_eq!(
                (raised, blocked),
                (iron_sights, !iron_sights),
                "weapon {weapon:x} melee {melee}"
            );
        }
    }

    // ---- tests of 00944300 .. 0094c460 (the third batch of this file) ----

    /// Maps the parts of the exe's data the translations of this batch read
    /// (settings objects and constants), zeroed.
    fn map_exe_data(e: &mut Engine) {
        e.map(0x0101_0000, 0x3_0000);
        e.map(0x0107_0000, 0x2_0000);
        e.map(0x011a_0000, 0x6_0000);
        e.map(0x0120_0000, 0x1_0000);
    }

    /// Writes the double and float constants of the exe that this batch's
    /// functions read.
    fn exe_constants(e: &mut Engine) {
        map_exe_data(e);
        let pi = std::f32::consts::PI as f64;
        for (addr, value) in [
            (0x0101_2070u32, 1.0f64),
            (0x0101_ff40, pi),
            (0x0101_ff48, 2.0 * pi),
            (0x0101_ff58, -pi),
            (0x0101_ffa0, 0.1f32 as f64),
            (0x0102_9790, 32767.0),
            (0x0107_18b8, 120.0),
            (0x0102_3128, 0.017453292f32 as f64),
            (0x0108_b098, -(std::f32::consts::FRAC_PI_2 as f64)),
            (0x0103_0f38, std::f32::consts::FRAC_PI_2 as f64),
            (0x0103_56d8, 24.0),
            (0x0101_6ff0, 1.5),
            (0x0107_5d30, 99.0),
            (0x0107_18f0, 7849.0),
            (0x0107_37c8, 1500.0),
            (0x0108_b0a8, -1500.0),
            (0x0101_5a38, 15.0),
            (0x0102_0758, 10.0),
            (0x0101_7b70, 1000.0),
            (0x0101_1590, 2.0),
            (0x0101_1588, 0.5),
            (0x0108_b0a0, 124.0),
            (0x0101_db80, 4.0),
            (0x0108_b048, 0.02f32 as f64),
            (0x0101_7a40, 100.0),
            (0x0102_e430, 128.0),
            (0x0102_40c0, 64.0),
            (0x0102_40b0, 8.0),
            (0x0102_8338, 512.0),
            (0x0108_b0b0, 0.04f32 as f64),
        ] {
            e.set_global(addr, value);
        }
        for (addr, value) in [
            (0x0101_6264u32, 0.75f32),
            (0x0107_18c0, 1.0e-5),
            (0x0101_6b78, -std::f32::consts::FRAC_PI_2),
            (0x0101_ff38, std::f32::consts::FRAC_PI_2),
            (0x0101_6088, 1.5),
            (0x0101_7718, 3.0),
            (0x0101_7b78, 10.0),
            (0x0101_62c0, 2.0),
            (0x0102_d934, 0.02),
            (0x0101_8144, 0.04),
            (0x0101_3974, 1000.0),
        ] {
            e.set_global(addr, value);
        }
        e.set_global(0x011a_3b60, 300i32);
    }

    /// Doubles that make the setting getters return their own argument: the
    /// test writes the setting value at the setting's address.
    fn settings_by_address(e: &mut Engine) {
        for addr in [
            SETTING_FLOAT_GETTER,
            SETTING_BYTE_GETTER,
            SETTING_INT_GETTER,
        ] {
            e.register(addr, |_, a| int(a[0]));
        }
    }

    /// The identity double of the list-item getter and the list-next
    /// walker over `{item, next}` nodes.
    fn list_walkers(e: &mut Engine) {
        e.register(LIST_ITEM_ADDRESS, |_, a| int(a[0]));
        e.register(LIST_NEXT, |e, a| int(e.mem.u32(a[0] + 4)));
    }

    /// Builds a chain of `{item, next}` nodes; returns the first node.
    fn node_chain(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0u32;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn fn_00944300_returns_the_ammo_regen_tick_time() {
        let mut e = engine();
        let this = player(&mut e);
        e.set(this, PlayerCharacter::fTimeSinceLastAmmoRegenTick, 1.5);
        assert_eq!(e.call(0x0094_4300, &args![this]).f32(), 1.5);
    }

    #[test]
    fn fn_00944320_returns_the_word_at_0x688() {
        let mut e = engine();
        let this = player(&mut e);
        e.mem.set_u32(this.addr() + 0x688, 0x1234);
        assert_eq!(e.call(0x0094_4320, &args![this]).u32(), 0x1234);
    }

    #[test]
    fn fn_00944340_and_00944360_return_the_block_activate_and_can_wait_flags() {
        let mut e = engine();
        let this = player(&mut e);
        assert!(!e.call(0x0094_4340, &args![this]).bool());
        assert!(!e.call(0x0094_4360, &args![this]).bool());
        e.set(this, PlayerCharacter::bBlockActivate, true);
        e.set(this, PlayerCharacter::bCanWait, true);
        assert!(e.call(0x0094_4340, &args![this]).bool());
        assert!(e.call(0x0094_4360, &args![this]).bool());
    }

    #[test]
    fn fn_00944380_and_009443a0_store_and_return_the_counter_attack_timer() {
        let mut e = engine();
        let this = player(&mut e);
        e.call(0x0094_4380, &args![this, 2.5f32]);
        assert_eq!(e.get(this, PlayerCharacter::fCounterAttackTimer), 2.5);
        assert_eq!(e.call(0x0094_43a0, &args![this]).f32(), 2.5);
    }

    #[test]
    fn fn_009443c0_is_true_when_either_global_flag_is_set() {
        let mut e = engine();
        map_exe_data(&mut e);
        assert!(!e.call(0x0094_43c0, &args![0u32]).bool());
        e.mem.set_u8(0x011d_ea2b, 1);
        assert!(e.call(0x0094_43c0, &args![0u32]).bool());
        e.mem.set_u8(0x011d_ea2b, 0);
        e.mem.set_u8(0x011d_ea29, 1);
        assert!(e.call(0x0094_43c0, &args![0u32]).bool());
    }

    #[test]
    fn fn_009443f0_returns_the_float_global() {
        let mut e = engine();
        map_exe_data(&mut e);
        e.set_global(0x011a_c3a4, 3.5f32);
        assert_eq!(e.call(0x0094_43f0, &args![]).f32(), 3.5);
    }

    #[test]
    fn fn_00944400_and_00944430_test_the_flags_of_the_member_with_their_masks() {
        let mut e = engine();
        e.register(0x0062_1270, |_, a| int(a[1] & 0x20000));
        let this = Ptr::<()>::new(0x5000);
        assert!(!e.call(0x0094_4400, &args![this]).bool());
        assert!(e.call(0x0094_4430, &args![this]).bool());
        assert_eq!(
            called(&e, 0x0062_1270),
            vec![vec![0x5410, 0x400], vec![0x5410, 0x20000]]
        );
    }

    #[test]
    fn fn_00944460_draws_between_its_bounds_with_the_global_generator() {
        let mut e = engine();
        e.register(0x0047_6c00, |_, _| int(0x2000));
        e.register(0x00aa_5230, |_, a| int(a[1] / 2));
        // low + draw(high - low), the generator `this` first.
        assert_eq!(
            e.call(0x0094_4460, &args![0x5dci32, 0x7d0i32]).i32(),
            0x5dc + 0xfa
        );
        assert_eq!(called(&e, 0x00aa_5230), vec![vec![0x2000, 0x1f4]]);
        assert_eq!(
            e.call(0x0094_4480, &args![0x3000u32, 10i32, 30i32]).i32(),
            20
        );
    }

    #[test]
    fn fn_009444a0_reads_a_byte_of_the_global_object_or_zero() {
        let mut e = engine();
        map_exe_data(&mut e);
        assert_eq!(e.call(0x0094_44a0, &args![]).u8(), 0);
        let object = e.mem.alloc(0x300);
        e.mem.set_u8(object + 0x204, 7);
        e.set_global(0x011d_96c0, object);
        assert_eq!(e.call(0x0094_44a0, &args![]).u8(), 7);
    }

    /// The combat manager and the group of the update-player-combat test.
    fn combat_world(e: &mut Engine, alive: u32, count: u32, searching: u32) {
        map_exe_data(e);
        e.set_global(0x011f_1958, 0x7700u32);
        e.set_global(0x011d_ea3c, 0x7800u32);
        e.register_double(0x005a_4320, move |_, _| int(alive));
        stub(e, &[0x0098_69a0]);
        e.register_double(0x0099_31c0, move |e, a| {
            e.mem.set_u32(a[2], searching);
            int(count)
        });
    }

    #[test]
    fn update_player_combat_drops_a_dead_group_and_resets_the_timer() {
        let mut e = engine();
        combat_world(&mut e, 0, 0, 0);
        let this = player(&mut e);
        e.set(this, PlayerCharacter::pCombatGroup, Ptr::new(0x9000));
        e.set(this, PlayerCharacter::fCombatTimer, 3.0);
        e.set(this, PlayerCharacter::bPlayerInCombat, true);
        e.call(0x0094_44d0, &args![this, 0.5f32]);
        assert_eq!(called(&e, 0x0098_69a0), vec![vec![0x9000, this.addr()]]);
        assert_eq!(e.get(this, PlayerCharacter::pCombatGroup), Ptr::NULL);
        assert_eq!(e.get(this, PlayerCharacter::fCombatTimer), 0.0);
        assert!(!e.get(this, PlayerCharacter::bPlayerInCombat));
        assert!(!e.get(this, PlayerCharacter::bAllCombatTargetsSearching));
        assert_eq!(called(&e, 0x0099_31c0)[0][..2], [0x7700, 0x7800]);
    }

    #[test]
    fn update_player_combat_counts_up_while_the_group_lives_and_flags_combat() {
        let mut e = engine();
        combat_world(&mut e, 1, 3, 3);
        let this = player(&mut e);
        e.set(this, PlayerCharacter::pCombatGroup, Ptr::new(0x9000));
        e.set(this, PlayerCharacter::fCombatTimer, 1.0);
        e.call(0x0094_44d0, &args![this, 0.5f32]);
        assert_eq!(e.get(this, PlayerCharacter::fCombatTimer), 1.5);
        assert!(called(&e, 0x0098_69a0).is_empty());
        assert!(e.get(this, PlayerCharacter::bPlayerInCombat));
        assert!(e.get(this, PlayerCharacter::bAllCombatTargetsSearching));
        // Not every combatant is searching.
        let mut e = engine();
        combat_world(&mut e, 1, 3, 2);
        let this = player(&mut e);
        e.call(0x0094_44d0, &args![this, 0.5f32]);
        assert!(e.get(this, PlayerCharacter::bPlayerInCombat));
        assert!(!e.get(this, PlayerCharacter::bAllCombatTargetsSearching));
    }

    /// An engine with the exe constants and doubles for the curve helpers:
    /// `004b3ab0` maps by adding 100 to its last argument, `00404010` is the
    /// larger of its two floats, `00406cc0` returns its argument, `ftol`
    /// truncates.
    fn curve_world() -> Engine {
        let mut e = engine();
        exe_constants(&mut e);
        e.register(0x004b_3ab0, |_, a| {
            float(f32::from_bits(a[4]) as f64 + 100.0)
        });
        e.register(0x0040_4010, |_, a| {
            float(f32::from_bits(a[0]).max(f32::from_bits(a[1])) as f64)
        });
        e.register(0x0040_6cc0, |_, a| float(f32::from_bits(a[0]) as f64));
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            int(value as i32 as u32)
        });
        e
    }

    #[test]
    fn timed_curve_start_fills_the_eased_table_and_remaps_it() {
        let mut e = curve_world();
        let curve = e.new_object::<TimedCurve>();
        assert!(e
            .call(0x0094_6020, &args![curve, 5.0f32, 9.0f32, 2.0f32])
            .bool());
        let sample = |e: &Engine, i: u32| e.mem.f32(curve.addr() + 4 * i);
        // sample[0] = 0 / 99, sample[99] = (99 - 0) / 99 = 1, then +100 from the double.
        assert_eq!(sample(&e, 0), 100.0);
        assert_eq!(sample(&e, 99), 101.0);
        // The ramp grows: sample[1] = 0.02 / 99, mirrored at 98.
        assert!((sample(&e, 1) - (100.0 + 0.02 / 99.0)).abs() < 1e-4);
        assert!(sample(&e, 49) > sample(&e, 1));
        assert_eq!(called(&e, 0x004b_3ab0).len(), 100);
        assert_eq!(
            called(&e, 0x004b_3ab0)[0][..4],
            [5.0f32.to_bits(), 9.0f32.to_bits(), 0, 1.0f32.to_bits()]
        );
        assert_eq!(e.get(curve, TimedCurve::from), 5.0);
        assert_eq!(e.get(curve, TimedCurve::to), 9.0);
        assert_eq!(e.get(curve, TimedCurve::duration), 2.0);
        assert_eq!(e.get(curve, TimedCurve::remaining), 2.0);
    }

    #[test]
    fn timed_curve_advance_reports_when_the_time_is_used_up() {
        let mut e = engine();
        let curve = e.new_object::<TimedCurve>();
        e.set(curve, TimedCurve::remaining, 1.0);
        assert!(!e.call(0x0094_6140, &args![curve, 0.25f32]).bool());
        assert_eq!(e.get(curve, TimedCurve::remaining), 0.75);
        assert!(e.call(0x0094_6140, &args![curve, 1.0f32]).bool());
        assert_eq!(e.get(curve, TimedCurve::remaining), -0.25);
    }

    #[test]
    fn timed_curve_value_interpolates_the_table_or_returns_the_end_value() {
        let mut e = curve_world();
        let curve = e.new_object::<TimedCurve>();
        e.set(curve, TimedCurve::end_value, 42.0);
        e.set(curve, TimedCurve::duration, 10.0);
        e.set(curve, TimedCurve::remaining, 5.0);
        // Halfway: position (10 - 5) / 10 * 99 = 49.5 -> index 49.
        e.mem.set_f32(curve.addr() + 4 * 49, 7.0);
        e.mem.set_f32(curve.addr() + 4 * 50, 8.0);
        assert_eq!(e.call(0x0094_6190, &args![curve]).f32(), 105.0);
        // (a, b, 0, duration, duration - remaining)
        assert_eq!(
            called(&e, 0x004b_3ab0)[0],
            vec![
                7.0f32.to_bits(),
                8.0f32.to_bits(),
                0,
                10.0f32.to_bits(),
                5.0f32.to_bits()
            ]
        );
        // Used up: the end value, without the remap.
        e.set(curve, TimedCurve::remaining, -1.0);
        assert_eq!(e.call(0x0094_6190, &args![curve]).f32(), 42.0);
        // Index 99 or more: the end value too.
        e.set(curve, TimedCurve::remaining, 0.5);
        e.set(curve, TimedCurve::duration, 0.5);
        e.set(curve, TimedCurve::remaining, 1.0e-9);
        assert_eq!(e.call(0x0094_6190, &args![curve]).f32(), 42.0);
        assert_eq!(called(&e, 0x004b_3ab0).len(), 1);
    }

    #[test]
    fn timed_curve_is_over_tests_the_time_left() {
        let mut e = engine();
        let curve = e.new_object::<TimedCurve>();
        e.set(curve, TimedCurve::remaining, 0.5);
        assert!(!e.call(0x0094_6280, &args![curve]).bool());
        e.set(curve, TimedCurve::remaining, 0.0);
        assert!(e.call(0x0094_6280, &args![curve]).bool());
    }

    #[test]
    fn fn_0094a070_divides_the_span_by_the_length() {
        let mut e = engine();
        let object = Ptr::<()>::new(e.mem.alloc(0x40));
        e.mem.set_f32(object.addr() + 0x30, 7.0);
        e.mem.set_f32(object.addr() + 0x2c, 1.0);
        e.mem.set_f32(object.addr() + 0x28, 4.0);
        assert_eq!(e.call(0x0094_a070, &args![object]).f32(), 1.5);
    }

    #[test]
    fn fn_0094a0a0_returns_the_byte_at_0x198() {
        let mut e = engine();
        let object = Ptr::<()>::new(e.mem.alloc(0x1a0));
        e.mem.set_u8(object.addr() + 0x198, 0x5c);
        assert_eq!(e.call(0x0094_a0a0, &args![object]).u8(), 0x5c);
    }

    #[test]
    fn fn_0094c380_calls_the_flag_function_with_zero() {
        let mut e = engine();
        stub(&mut e, &[0x0062_1370]);
        e.call(0x0094_c380, &args![0x4400u32]);
        assert_eq!(called(&e, 0x0062_1370), vec![vec![0x4400, 0]]);
    }

    #[test]
    fn fn_0094c3a0_scales_the_setting_into_the_global_object() {
        let mut e = engine();
        map_exe_data(&mut e);
        settings_by_address(&mut e);
        // Without the object nothing happens.
        e.call(0x0094_c3a0, &args![2.0f32]);
        let object = e.mem.alloc(0x300);
        e.set_global(0x011d_96c0, object);
        e.set_global(0x011d_9748, 1.5f32);
        e.call(0x0094_c3a0, &args![2.0f32]);
        assert_eq!(e.mem.f32(object + 0x274), 3.0);
    }

    #[test]
    fn fn_0094c3d0_and_0094c460_select_the_actor_value_modifier() {
        let mut e = engine();
        let this = player(&mut e);
        e.mem.set_f32(this.addr() + 0x244 + 4 * 5, 1.0);
        e.mem.set_f32(this.addr() + 0x378 + 4 * 5, 2.0);
        e.mem.set_f32(this.addr() + 0x4b0 + 4 * 5, 3.0);
        e.set(this, PlayerCharacter::fHealthModifier, 4.0);
        assert_eq!(e.call(0x0094_c3d0, &args![this, 0u32, 5u32]).f32(), 1.0);
        assert_eq!(e.call(0x0094_c3d0, &args![this, 1u32, 5u32]).f32(), 2.0);
        assert_eq!(e.call(0x0094_c3d0, &args![this, 2u32, 5u32]).f32(), 3.0);
        assert_eq!(e.call(0x0094_c3d0, &args![this, 2u32, 0x10u32]).f32(), 4.0);
        assert_eq!(e.call(0x0094_c3d0, &args![this, 3u32, 5u32]).f32(), 0.0);
        // The ActorValueOwner override reads kind 0 through the subobject.
        let owner = Ptr::<()>::new(this.addr() + 0xa4);
        assert_eq!(e.call(0x0094_c460, &args![owner, 5u32]).f32(), 1.0);
    }

    /// A process-like object whose vtable slots (byte offset, answer) return
    /// the given words; the doubles live at `0x00b0_0000 + base_offset +
    /// slot`.
    fn answering_object(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        let mut table = Vec::new();
        for (slot, answer) in slots {
            let target = 0x00b0_0000 + (vtable & 0xf_ffff) / 0x10 + slot;
            let answer = *answer;
            e.register_double(target, move |_, _| int(answer));
            table.push((*slot, target));
        }
        object(e, vtable, &table).addr()
    }

    /// The calls (arguments after `this`) a vtable slot of an object built by
    /// [`answering_object`] received.
    fn slot_calls(e: &Engine, vtable: u32, slot: u32) -> Vec<Vec<u32>> {
        called(e, 0x00b0_0000 + (vtable & 0xf_ffff) / 0x10 + slot)
    }

    /// Doubles every callee of `border_region_world` and gives the player a
    /// position pointer.
    fn border_world(e: &mut Engine, setting_on: bool) -> Ptr<PlayerCharacter> {
        map_exe_data(e);
        settings_by_address(e);
        list_walkers(e);
        stub(
            e,
            &[
                0x0044_0d80,
                0x0044_1110,
                0x0045_1530,
                0x004f_7030,
                0x004f_8360,
                0x0057_5d70,
                0x0058_6260,
                0x0065_8930,
                0x0072_6070,
                0x0082_56d0,
                0x0087_7a30,
                0x0094_dbe0,
                0x008d_6f30,
                0x0054_ddd0,
            ],
        );
        e.mem.set_u8(0x011e_08e0, setting_on as u8);
        let this = player(e);
        let position = e.mem.alloc(12);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        e.register_double(0x0043_6aa0, move |_, _| int(position));
        this
    }

    #[test]
    fn check_border_region_does_nothing_with_the_setting_off() {
        let mut e = engine();
        let this = border_world(&mut e, false);
        e.call(0x0094_7b10, &args![this]);
        assert!(called(&e, 0x0094_dbe0).is_empty());
        assert!(called(&e, 0x0043_6aa0).is_empty());
    }

    #[test]
    fn check_border_region_inside_a_contained_cell_records_the_position() {
        let mut e = engine();
        let this = border_world(&mut e, true);
        e.set(this, PlayerCharacter::bInBorderContainedCell, true);
        e.register(0x008d_6f30, |_, _| int(0));
        e.call(0x0094_7b10, &args![this]);
        assert_eq!(
            read_floats(&e, member(this, PlayerCharacter::LastKnownGoodPosition)),
            [1.0, 2.0, 3.0]
        );
        assert!(called(&e, 0x0094_dbe0).is_empty());
    }

    #[test]
    fn check_border_region_without_regions_is_inside_unless_the_world_space_has_borders() {
        let mut e = engine();
        let this = border_world(&mut e, true);
        // No world space: inside.
        e.call(0x0094_7b10, &args![this]);
        assert!(called(&e, 0x0094_dbe0).is_empty());
        // A world space with a border region: outside, back to the last good position.
        e.register(0x0057_5d70, |_, _| int(0x7000));
        e.register(0x0058_6260, |_, _| int(1));
        e.call(0x0094_7b10, &args![this]);
        assert_eq!(called(&e, 0x0094_dbe0), vec![vec![this.addr(), 0]]);
    }

    #[test]
    fn check_border_region_tests_the_point_against_each_region_entry() {
        let mut e = engine();
        let this = border_world(&mut e, true);
        let region = e.mem.alloc(8);
        let region_slot = e.mem.alloc(4);
        e.mem.set_u32(region_slot, region);
        let entry = e.mem.alloc(8);
        let entries = node_chain(&mut e, &[entry]);
        e.set(this, PlayerCharacter::pBorderRegions, Ptr::new(0x6000));
        e.register(0x0065_8930, |_, _| int(1));
        e.register_double(0x0087_7a30, move |_, _| int(region_slot));
        e.register_double(0x0044_1110, move |_, _| int(entries));
        // Not inside any entry: outside.
        e.call(0x0094_7b10, &args![this]);
        assert_eq!(called(&e, 0x0094_dbe0).len(), 1);
        assert_eq!(called(&e, 0x004f_8360)[0][0], entry);
        // Inside the entry: the position is recorded and nothing moves.
        e.register(0x004f_8360, |_, _| int(1));
        e.call(0x0094_7b10, &args![this]);
        assert_eq!(called(&e, 0x0094_dbe0).len(), 1);
        assert_eq!(
            read_floats(&e, member(this, PlayerCharacter::LastKnownGoodPosition)),
            [1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn set_last_known_good_position_prefers_the_world_space_of_the_parent_cell() {
        let mut e = engine();
        let this = border_world(&mut e, true);
        e.register(0x008d_6f30, |_, _| int(0x1111));
        e.register(0x0054_ddd0, |_, _| int(0x2222));
        e.call(0x0094_7c90, &args![this]);
        assert_eq!(
            e.get(this, PlayerCharacter::pLastKnownGoodLocation),
            Ptr::new(0x2222)
        );
        // Without a world space the cell itself is stored.
        e.register(0x0054_ddd0, |_, _| int(0));
        e.call(0x0094_7c90, &args![this]);
        assert_eq!(
            e.get(this, PlayerCharacter::pLastKnownGoodLocation),
            Ptr::new(0x1111)
        );
        // Without a cell: whatever `008d6f30` returns (null).
        e.register(0x008d_6f30, |_, _| int(0));
        e.call(0x0094_7c90, &args![this]);
        assert_eq!(
            e.get(this, PlayerCharacter::pLastKnownGoodLocation),
            Ptr::NULL
        );
    }

    /// Doubles for the list-clearing helpers: `operator delete` records its
    /// argument.
    fn list_world(e: &mut Engine) {
        list_walkers(e);
        stub(e, &[LIST_CLEAR, LIST_DELETING_DESTRUCT, LIST_CONSTRUCT]);
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| int(e.mem.alloc(a[0])));
    }

    /// Writes `items` into the embedded list at `head` (the head node holds
    /// the first item, the others are heap nodes).
    fn embedded_list(e: &mut Engine, head: u32, items: &[u32]) {
        e.mem.set_u32(head, items[0]);
        let rest = node_chain(e, &items[1..]);
        e.mem.set_u32(head + 4, rest);
    }

    #[test]
    fn the_audio_marker_list_is_freed_and_cleared() {
        let mut e = engine();
        list_world(&mut e);
        let this = player(&mut e);
        let head = member(this, PlayerCharacter::AudioMarkerList);
        embedded_list(&mut e, head, &[0xa1, 0xa2]);
        e.set(this, PlayerCharacter::pClosestAudioMarkerInfo, Ptr::new(5));
        e.call(0x0094_7d10, &args![this]);
        assert_eq!(called(&e, OPERATOR_DELETE), vec![vec![0xa1], vec![0xa2]]);
        assert_eq!(called(&e, LIST_CLEAR), vec![vec![head]]);
        assert_eq!(
            e.get(this, PlayerCharacter::pClosestAudioMarkerInfo),
            Ptr::NULL
        );
    }

    #[test]
    fn the_map_marker_list_is_freed_and_cleared() {
        let mut e = engine();
        list_world(&mut e);
        let this = player(&mut e);
        let head = member(this, PlayerCharacter::MapMarkerList);
        embedded_list(&mut e, head, &[0xb1, 0xb2, 0xb3]);
        e.set(this, PlayerCharacter::pMapWorld, Ptr::new(5));
        e.call(0x0094_8050, &args![this]);
        assert_eq!(
            called(&e, OPERATOR_DELETE),
            vec![vec![0xb1], vec![0xb2], vec![0xb3]]
        );
        assert_eq!(e.get(this, PlayerCharacter::pMapWorld), Ptr::NULL);
        assert_eq!(e.call(0x0094_8030, &args![this]).u32(), head);
    }

    #[test]
    fn the_audio_marker_entry_stores_the_reference_and_its_value() {
        let mut e = engine();
        e.register(0x0056_9080, |_, a| int(a[0] + 1));
        let entry = Ptr::<()>::new(e.mem.alloc(8));
        assert_eq!(
            e.call(0x0094_8000, &args![entry, 0x500u32]).ptr::<()>(),
            entry
        );
        assert_eq!(e.mem.u32(entry.addr()), 0x500);
        assert_eq!(e.mem.u32(entry.addr() + 4), 0x501);
        // A null reference stores nothing else.
        let other = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(other.addr() + 4, 0x77);
        e.call(0x0094_8000, &args![other, 0u32]);
        assert_eq!(e.mem.u32(other.addr()), 0);
        assert_eq!(e.mem.u32(other.addr() + 4), 0x77);
    }

    #[test]
    fn rebuilding_the_audio_markers_keeps_the_entries_with_a_second_word() {
        let mut e = engine();
        list_world(&mut e);
        stub(
            &mut e,
            &[
                0x0054_b8c0,
                0x0057_5d70,
                0x0058_83c0,
                0x005a_e3d0,
                0x008d_6f30,
            ],
        );
        let this = player(&mut e);
        let references = node_chain(&mut e, &[0x100, 0, 0x200]);
        e.register(0x0057_5d70, |_, _| int(0x7000));
        e.register_double(0x0058_83c0, move |_, _| int(references));
        // 0x100 has a value, 0x200 has none.
        e.register(0x0056_9080, |_, a| int(if a[0] == 0x100 { 9 } else { 0 }));
        e.call(0x0094_7d80, &args![this]);
        // One entry was added to the list, one freed; the temporary list is
        // cleared and deleted.
        let appended = called(&e, 0x005a_e3d0);
        assert_eq!(appended.len(), 1);
        assert_eq!(
            appended[0][0],
            member(this, PlayerCharacter::AudioMarkerList)
        );
        let deletes = called(&e, OPERATOR_DELETE);
        assert_eq!(deletes.len(), 2); // the old head item and the entry of 0x200
        assert_eq!(
            called(&e, LIST_DELETING_DESTRUCT),
            vec![vec![references, 1]]
        );
        // Without a world space the cell supplies the references.
        e.call_log = Some(vec![]);
        e.register(0x0057_5d70, |_, _| int(0));
        e.register(0x008d_6f30, |_, _| int(0x4000));
        e.call(0x0094_7d80, &args![this]);
        assert_eq!(called(&e, 0x0054_b8c0).len(), 1);
        assert_eq!(called(&e, 0x0054_b8c0)[0][0], 0x4000);
        assert!(called(&e, 0x0058_83c0).is_empty());
    }

    #[test]
    fn resetting_the_map_markers_rebuilds_them_when_the_world_space_changed() {
        let mut e = engine();
        list_world(&mut e);
        stub(&mut e, &[0x0057_5d70, 0x0058_82a0, 0x005a_e3d0]);
        let this = player(&mut e);
        e.register(0x0057_5d70, |_, _| int(0x7000));
        let markers = node_chain(&mut e, &[0x100, 0x200, 0]);
        e.register_double(0x0058_82a0, move |_, _| int(markers));
        e.register(0x0056_9060, |_, a| int(if a[0] == 0x100 { 5 } else { 0 }));
        e.call(0x0094_80c0, &args![this]);
        assert_eq!(e.get(this, PlayerCharacter::pMapWorld), Ptr::new(0x7000));
        // The entry of 0x100 was appended, the one of 0x200 freed.
        assert_eq!(called(&e, 0x005a_e3d0).len(), 1);
        assert_eq!(called(&e, LIST_DELETING_DESTRUCT), vec![vec![markers, 1]]);
        // Unchanged world space: nothing more happens.
        e.call_log = Some(vec![]);
        e.call(0x0094_80c0, &args![this]);
        assert_eq!(call_addresses(&e), vec![0x0094_80c0, 0x0057_5d70]);
    }

    #[test]
    fn fn_009481d0_resynchronises_the_two_animations() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                0x0048_f7f0,
                0x0049_1040,
                0x005f_3780,
                0x0063_9aa0,
                0x0070_f490,
                0x008a_7570,
            ],
        );
        stub(&mut e, &[0x008e_eaa0, 0x0096_6a00]);
        let this = player(&mut e);
        let process = answering_object(&mut e, 0x0262_0000, &[(0x1a8, 1), (0x454, 1)]);
        e.mem.set_u32(this.addr() + 0x68, process);
        e.register_double(0x0095_0a60, |_, a| int(0xa00 + a[1]));
        e.register(0x008a_7570, |_, _| int(5));
        e.register(0x0070_f490, |_, _| int(1));
        e.register(0x0049_1040, |_, a| int(a[0] + 0x10));
        e.register(0x0048_f7f0, |_, a| int(a[0] + 0x20));
        e.register(0x005f_3780, |_, _| float(3.0));
        e.register(0x0063_9aa0, |_, _| float(1.0));
        e.call(0x0094_81d0, &args![this]);
        assert_eq!(
            called(&e, 0x008e_eaa0),
            vec![vec![0xa00, 4], vec![0xa01, 4]]
        );
        assert_eq!(
            called(&e, 0x0096_6a00),
            vec![vec![0xa00, 2.0f32.to_bits()], vec![0xa01, 2.0f32.to_bits()]]
        );
        // Another animation action: nothing.
        e.call_log = Some(vec![]);
        e.register(0x008a_7570, |_, _| int(4));
        e.call(0x0094_81d0, &args![this]);
        assert!(called(&e, 0x0096_6a00).is_empty());
    }

    /// Every callee of `00948310` outside this file.
    const ATTACK_CALLEES: &[u32] = &[
        0x0040_3e20,
        0x0043_01b0,
        0x0044_ddc0,
        0x0044_edb0,
        0x0047_4a80,
        0x0049_1040,
        0x0049_3800,
        0x0049_4740,
        0x0049_5740,
        0x0049_5e40,
        0x0049_81f0,
        0x0049_8d30,
        0x0049_97b0,
        0x004a_7bd0,
        0x004b_da70,
        0x004b_f220,
        0x004c_8f30,
        0x004d_ff00,
        0x004f_e160,
        0x0051_f5f0,
        0x0052_4b40,
        0x0052_5980,
        0x0058_d630,
        0x0059_8040,
        0x0059_bb30,
        0x005a_2030,
        0x005e_58f0,
        0x005f_2370,
        0x005f_2440,
        0x005f_2540,
        0x005f_25d0,
        0x005f_2670,
        0x0064_50c0,
        0x0066_dde0,
        0x0070_9430,
        0x0070_9470,
        0x0070_f490,
        0x0072_6070,
        0x0080_41a0,
        0x0084_d030,
        0x0087_7720,
        0x0088_46e0,
        0x0088_c650,
        0x0089_3a40,
        0x0089_4cc0,
        0x008a_6840,
        0x008a_6970,
        0x008a_73e0,
        0x008a_7570,
        0x008b_a3e0,
        0x008b_a410,
        0x008d_85e0,
        0x008e_eaa0,
        0x0095_0a60,
        0x0096_3eb0,
        0x0098_39b0,
        0x009c_71c0,
        0x009c_8cc0,
        0x00a2_4280,
        0x00a2_4660,
    ];

    /// A world for `00948310`: every outside callee answers 0, the input
    /// object is `0x1000`, the animations are `0xa00 + index`, one frame is
    /// 0.5 s with the VATS multiplier 2, and the player's process is an
    /// object whose slots answer 0 unless `slots` says otherwise.
    fn attack_world(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> Ptr<PlayerCharacter> {
        exe_constants(e);
        stub(e, ATTACK_CALLEES);
        settings_by_address(e);
        e.register(0x0087_7720, |_, _| int(0x1000));
        e.register(0x0095_0a60, |_, a| int(0xa00 + a[1]));
        e.register(0x0084_d030, |_, _| float(0.5));
        e.register(0x009c_8cc0, |_, _| float(2.0));
        e.set_global(0x011d_ea3c, 0x7a00u32);
        let this = player(e);
        let mut all: Vec<(u32, u32)> = [
            0x148u32, 0x14c, 0x1a8, 0x1ac, 0x1b0, 0x1b4, 0x3f0, 0x3f4, 0x434, 0x438, 0x43c, 0x454,
            0x6e0,
        ]
        .iter()
        .map(|slot| (*slot, 0))
        .collect();
        all.extend_from_slice(slots);
        let process = answering_object(e, vtable, &all);
        e.mem.set_u32(this.addr() + 0x68, process);
        this
    }

    /// Makes the button state query answer true for exactly the given
    /// (button, state) pair.
    fn only_button(e: &mut Engine, button: u32, state: u32) {
        e.register_double(0x00a2_4660, move |_, a| {
            int((a[1] == button && a[2] == state) as u32)
        });
    }

    #[test]
    fn a_new_attack_press_without_the_weapon_out_draws_it() {
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[]);
        only_button(&mut e, 4, 1);
        assert!(e.call(0x0094_8310, &args![this]).bool());
        assert_eq!(called(&e, 0x008a_6840), vec![vec![this.addr(), 1]]);
        assert!(called(&e, 0x0089_3a40).is_empty());
        assert_eq!(e.global::<f32>(ATTACK_HOLD_TIMER), 0.0);
    }

    #[test]
    fn a_new_press_while_reloading_with_an_empty_magazine_stops_early() {
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[(0x454, 1), (0x14c, 0x5200)]);
        only_button(&mut e, 4, 1);
        e.register(0x008a_7570, |_, _| int(0x11));
        assert!(!e.call(0x0094_8310, &args![this]).bool());
        assert_eq!(called(&e, 0x0072_6070), vec![vec![0x5200]]);
        assert!(called(&e, 0x0089_3a40).is_empty());
    }

    #[test]
    fn a_new_press_while_reloading_queues_the_next_round_and_plays_the_reload() {
        let mut e = engine();
        let this = attack_world(
            &mut e,
            0x0265_0000,
            &[(0x454, 1), (0x14c, 0x5200), (0x148, 0x5300)],
        );
        player_slots(&mut e, this, 0.0);
        only_button(&mut e, 4, 1);
        e.register(0x008a_7570, |_, _| int(0x11));
        e.register(0x0072_6070, |_, _| int(2)); // two rounds in the magazine
        e.register(0x0088_46e0, |_, _| int(0x401)); // crouching, moving: bits 0x400 | 1
        e.register(0x0044_ddc0, |_, a| {
            int(if a[0] == 0x5200 { 0x5210 } else { 0x5310 })
        });
        e.register(0x004f_e160, |_, _| int(10)); // clip size
        e.register(0x004c_8f30, |_, _| int(8)); // rounds carried
        e.register(0x004b_f220, |_, _| int(0x5400));
        e.register(0x005f_2370, |_, _| int(0x77));
        e.register(0x0049_5740, |_, _| int(0x33));
        e.register(0x0049_1040, |_, _| int(0x5500));
        assert!(e.call(0x0094_8310, &args![this]).bool());
        // The next round is queued: (item, count 3, 0, 1, 0, 0).
        assert_eq!(
            called(&e, 0x0088_c650),
            vec![vec![this.addr(), 0x5210, 3, 0, 1, 0, 0]]
        );
        // The animation group is built from the stance (crouch 1) and direction (3).
        assert_eq!(called(&e, 0x005f_2370), vec![vec![1, 0, 3, 0]]);
        assert_eq!(called(&e, 0x0049_5740), vec![vec![0xa00, 0x77, 0]]);
        assert_eq!(called(&e, 0x0049_4740), vec![vec![0xa00, 0x33, 1, 1, 4]]);
        assert_eq!(
            called(&e, 0x008a_73e0),
            vec![vec![this.addr(), 0xffff_ffff, 0x5500]]
        );
    }

    /// The vtable of the player for the slot `0x3EC` and `0x4B0` calls, and
    /// the `ActorValueOwner` subobject slot `0xC` answering `value`.
    fn player_slots(e: &mut Engine, this: Ptr<PlayerCharacter>, value: f64) {
        e.register_double(0x00b0_03ec, |_, _| Ret::default());
        e.register_double(0x00b0_04b0, |_, _| Ret::default());
        put_vtable(
            e,
            this.addr(),
            0x0266_0000,
            &[(0x3ec, 0x00b0_03ec), (0x4b0, 0x00b0_04b0)],
        );
        e.register_double(0x00b0_000c, move |_, _| float(value));
        put_vtable(e, this.addr() + 0xa4, 0x0267_0000, &[(0xc, 0x00b0_000c)]);
    }

    #[test]
    fn a_queued_power_attack_becomes_the_directional_perk_attack_and_starts() {
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[]);
        player_slots(&mut e, this, 5.0);
        e.set_global(ATTACK_QUEUED, 2u32);
        e.register(0x0088_46e0, |_, _| int(1)); // moving forward: group 0x5d
        e.register(0x005e_58f0, |e, a| {
            e.mem.set_f32(a[2], 1.0);
            Ret::default()
        });
        e.register(0x0089_3a40, |_, _| int(1));
        e.register(0x004d_ff00, |_, _| int(1)); // the dialogue check
        assert!(!e.call(0x0094_8310, &args![this]).bool());
        // 0x5d with a positive entry point 0x3d: 0x61.
        assert_eq!(called(&e, 0x0089_3a40), vec![vec![this.addr(), 0x61]]);
        assert_eq!(
            called(&e, 0x0098_39b0),
            vec![vec![0, 0x7a00, 0, 2, 1, 0, 0]]
        );
        assert_eq!(e.global::<u32>(ATTACK_QUEUED), 0);
        // The repeat latch was set by the start and cleared again because no
        // attack button is down.
        assert_eq!(e.mem.u8(ATTACK_REPEAT_LATCH), 0);
        assert_eq!(e.global::<u32>(ATTACK_COUNTER), 0);
    }

    #[test]
    fn the_vats_attack_step_decrements_the_action_counter_and_picks_the_action_group() {
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[(0x454, 1)]);
        player_slots(&mut e, this, 0.0);
        only_button(&mut e, 4, 1);
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(action, 0x10);
        e.mem.set_u8(action + 8, 3);
        e.register(0x0044_ddc0, |_, a| int((a[0] == VATS_OBJECT) as u32 * 4));
        e.register_double(0x009c_71c0, move |_, _| int(action));
        e.register(0x0066_dde0, |_, _| int(1));
        e.register(0x005f_2440, |_, _| int(0x11));
        e.register(0x0049_1040, |_, a| int(if a[1] == 4 { 0x5500 } else { 0 }));
        e.register(0x0080_41a0, |_, _| int(1));
        e.register(0x0089_3a40, |_, _| int(1));
        e.register(0x005f_2670, |_, _| int(1));
        e.call(0x0094_8310, &args![this]);
        assert_eq!(e.mem.u8(action + 8), 2);
        assert_eq!(e.global::<u32>(VATS_ATTACK_DELAY), 3);
        assert_eq!(called(&e, 0x00a2_4280), vec![vec![0x1000, 4]]);
        // Kind 0x10 without a weapon: group 0x5d (a power attack), started.
        assert_eq!(called(&e, 0x0089_3a40), vec![vec![this.addr(), 0x5d]]);
        // The next frame only counts the delay down.
        e.call_log = Some(vec![]);
        e.call(0x0094_8310, &args![this]);
        assert_eq!(e.global::<u32>(VATS_ATTACK_DELAY), 2);
        assert_eq!(e.mem.u8(action + 8), 2);
    }

    /// The ammo swap world: an ammo list [A, B] on the weapon, A current.
    fn ammo_world(e: &mut Engine) -> (Ptr<PlayerCharacter>, [u32; 2]) {
        exe_constants(e);
        list_walkers(e);
        stub(
            e,
            &[
                0x0044_59e0,
                0x0047_4a40,
                0x004b_c550,
                0x004b_d7a0,
                0x004b_da70,
                0x004b_f220,
                0x004b_fb70,
                0x004c_8f30,
                0x0050_0940,
                0x0052_5980,
                0x0057_6260,
                0x005f_65d0,
                0x006e_cd40,
                0x008a_16d0,
            ],
        );
        e.register(0x0040_1000, |e, a| int(e.mem.alloc(a[0])));
        e.set_global(0x011d_ea3c, 0x7a00u32);
        let this = player(e);
        let process = answering_object(
            e,
            0x0268_0000,
            &[(0x148, 0x5100), (0x168, 0), (0x14c, 0x5200)],
        );
        e.mem.set_u32(this.addr() + 0x68, process);
        e.register_double(0x00b0_03ec, |_, _| Ret::default());
        put_vtable(e, this.addr(), 0x0269_0000, &[(0x3ec, 0x00b0_03ec)]);
        let ammo = node_chain(e, &[0xa1, 0xb2]);
        e.register(0x0044_ddc0, |_, a| {
            int(if a[0] == 0x5100 { 0x6000 } else { 0x6100 })
        });
        e.register(0x0047_4a40, |_, _| int(0x6200));
        e.register_double(0x0050_0940, move |_, _| int(ammo));
        e.register(0x0052_5980, |_, _| int(0xa1));
        e.register(0x005f_65d0, |_, _| int(1));
        // Item changes: one per ammo form, both with stacks.
        // The item changes are real objects: the count is at +4 (`00726070`,
        // which is also the list walker).
        let item_a = e.mem.alloc(8);
        let item_b = e.mem.alloc(8);
        e.mem.set_u32(item_a + 4, 3);
        e.mem.set_u32(item_b + 4, 3);
        e.register_double(0x0057_6260, move |_, a| {
            int(if a[1] == 0xa1 { item_a } else { item_b })
        });
        e.register(0x004b_d7a0, |_, _| int(2));
        e.register(0x004b_f220, |_, _| int(0x7300));
        e.register(0x004c_8f30, |_, _| int(9));
        e.register(0x004b_c550, |_, _| int(0x7400));
        e.register(0x008a_16d0, |_, _| int(1));
        (this, [item_a, item_b])
    }

    #[test]
    fn ammo_swap_equips_the_next_ammo_the_player_has() {
        let mut e = engine();
        let (this, [item_a, item_b]) = ammo_world(&mut e);
        e.call(0x0094_62c0, &args![this, true, false]);
        // The unused item change of the current ammo and the found one are freed.
        assert_eq!(
            called(&e, 0x0044_59e0),
            vec![vec![item_a, 1], vec![item_b, 1]]
        );
        // The hotkey moves to the new ammo, the new item change replaces the old.
        assert_eq!(called(&e, 0x004b_fb70), vec![vec![this.addr(), 2, 0xb2]]);
        assert_eq!(called(&e, 0x004b_c550)[0][1..], [0xb2, 9]);
        let process = e.mem.u32(this.addr() + 0x68);
        assert_eq!(
            slot_calls(&e, 0x0268_0000, 0x168),
            vec![vec![process, 0x7400]]
        );
        // The player is told: (weapon object, drawn state 2, mod flag, not-draw 0).
        let slot = called(&e, 0x00b0_03ec);
        assert_eq!(slot, vec![vec![this.addr(), 0x6000, 2, 0, 0]]);
    }

    #[test]
    fn ammo_swap_keeps_the_current_ammo_when_asked_and_still_in_stock() {
        let mut e = engine();
        let (this, [item_a, _]) = ammo_world(&mut e);
        e.call(0x0094_62c0, &args![this, false, true]);
        assert_eq!(called(&e, 0x0044_59e0), vec![vec![item_a, 1]]);
        assert!(called(&e, 0x004b_fb70).is_empty());
        assert!(called(&e, 0x00b0_03ec).is_empty());
        assert!(slot_calls(&e, 0x0268_0000, 0x168).is_empty());
    }

    #[test]
    fn ammo_swap_needs_a_process_with_an_equipped_weapon() {
        let mut e = engine();
        let (this, _) = ammo_world(&mut e);
        e.mem.set_u32(this.addr() + 0x68, 0);
        e.call(0x0094_62c0, &args![this, true, false]);
        assert_eq!(call_addresses(&e), vec![0x0094_62c0]);
    }

    /// Every callee of `009445b0` outside this file, and of the curve
    /// helpers it uses.
    const LOOK_CALLEES: &[u32] = &[
        0x0040_3e20,
        0x0040_4010,
        0x0040_6cc0,
        0x0040_8860,
        0x0040_8d60,
        0x0041_81e0,
        0x0043_9ef0,
        0x0043_b230,
        0x0043_d450,
        0x0043_d4d0,
        0x0044_ddc0,
        0x0045_7990,
        0x0045_bb80,
        0x0045_c670,
        0x004a_ae30,
        0x004b_3ab0,
        0x004b_71d0,
        0x004b_7210,
        0x004e_3d00,
        0x004e_af60,
        0x004f_8960,
        0x0053_d280,
        0x0057_2380,
        0x005a_03f0,
        0x005e_50f0,
        0x005e_5130,
        0x0063_c8a0,
        0x0066_29f0,
        0x0068_15c0,
        0x006a_9540,
        0x0071_5da0,
        0x0073_b020,
        0x0074_7d00,
        0x0075_5680,
        0x007f_3bd0,
        0x0084_d030,
        0x0087_7720,
        0x0088_46e0,
        0x008a_7570,
        0x008d_8520,
        0x0093_1d30,
        0x0093_1d70,
        0x0093_1d90,
        0x0093_1e50,
        0x0095_0110,
        0x0095_0a60,
        0x0095_de30,
        0x0096_7ae0,
        0x0099_e040,
        0x009a_88e0,
        0x009a_8af0,
        0x009c_71c0,
        0x009c_7240,
        0x00a2_3390,
        0x00a2_39e0,
        0x00a6_fc50,
        0x00ec_62c0,
        0x00ec_7d40,
    ];

    /// A real `004b3ab0`: remaps `t` from `[t0, t1]` to `[a, b]`.
    fn remap_double(e: &mut Engine) {
        e.register(0x004b_3ab0, |_, a| {
            let f = |i: usize| f32::from_bits(a[i]) as f64;
            float(f(0) + (f(1) - f(0)) * (f(4) - f(2)) / (f(3) - f(2)))
        });
    }

    /// A world for `009445b0`: the VATS mode is `mode`, the exe data is
    /// mapped, every outside callee answers 0 and the setting getters return
    /// their own address. The player's vtable answers slots `0x2bc` (heading
    /// 0), `0x214` (0) and records `0x2c4`.
    fn look_world(mode: u32) -> (Engine, Ptr<PlayerCharacter>) {
        let mut e = engine();
        exe_constants(&mut e);
        stub(&mut e, LOOK_CALLEES);
        settings_by_address(&mut e);
        remap_double(&mut e);
        e.register_double(0x0044_ddc0, move |_, _| int(mode));
        e.register(FTOL, |_, a| {
            int(f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32)
        });
        e.register(0x0040_4010, |_, a| {
            float(f32::from_bits(a[0]).max(f32::from_bits(a[1])) as f64)
        });
        e.register(0x0040_6cc0, |_, a| float(f32::from_bits(a[0]) as f64));
        let this = player(&mut e);
        e.register_double(0x00b0_02bc, |_, _| float(0.0));
        e.register_double(0x00b0_0214, |_, _| int(0));
        e.register_double(0x00b0_02c4, |_, _| Ret::default());
        put_vtable(
            &mut e,
            this.addr(),
            0x026a_0000,
            &[
                (0x2bc, 0x00b0_02bc),
                (0x214, 0x00b0_0214),
                (0x2c4, 0x00b0_02c4),
            ],
        );
        let process = answering_object(&mut e, 0x026b_0000, &[(0x1a8, 0)]);
        e.mem.set_u32(this.addr() + 0x68, process);
        (e, this)
    }

    #[test]
    fn the_look_handler_ignores_unknown_modes() {
        let (mut e, this) = look_world(9);
        let flags = Ptr::<()>::new(e.mem.alloc(4));
        assert_eq!(
            e.call(0x0094_45b0, &args![this, 0.1f32, false, flags, 0u32])
                .u8(),
            0
        );
        assert_eq!(call_addresses(&e), vec![0x0094_45b0, 0x0044_ddc0]);
    }

    #[test]
    fn the_look_handler_does_nothing_while_the_ai_controls_the_player() {
        let (mut e, this) = look_world(0);
        e.set(this, PlayerCharacter::bAiControlledToPos, true);
        let flags = Ptr::<()>::new(e.mem.alloc(4));
        assert_eq!(
            e.call(0x0094_45b0, &args![this, 0.1f32, false, flags, 0u32])
                .u8(),
            0
        );
        assert!(called(&e, 0x0087_7720).is_empty());
    }

    #[test]
    fn the_free_look_turns_the_player_by_the_input_and_sets_the_turn_flag() {
        let (mut e, this) = look_world(0);
        // Digital input: yaw 100, no pitch, no zoom.
        e.register(0x00a2_39e0, |_, a| int(if a[1] == 1 { 100 } else { 0 }));
        e.register(0x00ec_7d40, |_, a| int((a[0] as i32).unsigned_abs()));
        e.register(0x0040_8860, |_, a| float(f32::from_bits(a[0]).abs() as f64));
        e.register(0x0099_e040, |_, _| float(1.0));
        e.register(0x004e_3d00, |_, _| float(1.0));
        e.register(0x0045_c670, |_, _| int(0x6000));
        e.set_global(0x011e_0a6c, 2.0f32); // look sensitivity
        e.set_global(0x0120_315c, 2.0f32); // the view limit: half the limit is available
        let flags = Ptr::<()>::new(e.mem.alloc(4));
        e.mem.set_u16(flags.addr(), 0x0001);
        let result = e
            .call(0x0094_45b0, &args![this, 0.5f32, false, flags, 0u32])
            .u8();
        assert_eq!(result, 1);
        // turn = 2.0 * 100; the flag for a right turn is added.
        assert_eq!(e.mem.u16(flags.addr()), 0x0021);
        // The player turns by turn * (view / limit = 0.5).
        assert_eq!(
            called(&e, 0x0093_1d30),
            vec![vec![this.addr(), 100.0f32.to_bits()]]
        );
        // The turn speed estimate is clamped to the constant 1.5.
        assert_eq!(e.global::<f32>(TURN_SPEED), 1.5);
        assert_eq!(e.global::<f32>(LAST_HEADING), 0.0);
    }

    #[test]
    fn the_free_look_inverts_the_pitch_and_zooms_in_the_third_person() {
        let (mut e, this) = look_world(0);
        // Digital input: no yaw, pitch 10, zoom 120.
        e.register(0x00a2_39e0, |_, a| {
            int(match a[1] {
                2 => 10,
                3 => 120,
                _ => 0,
            })
        });
        e.register(0x00ec_7d40, |_, a| int((a[0] as i32).unsigned_abs()));
        e.register(0x0099_e040, |_, _| float(1.0));
        e.register(0x004e_3d00, |_, _| float(1.0));
        e.register(0x0045_c670, |_, _| int(0x6000));
        e.set_global(0x011e_0a6c, 3.0f32);
        e.set_global(0x0120_315c, 1.0f32);
        e.mem.set_u8(0x011e_0a5c, 1); // the invert-pitch setting
        e.set(this, PlayerCharacter::b3rdPerson, true);
        e.set_global(ZOOM_DISTANCE, 100.0f32);
        e.set_global(0x011c_dc98, 0.5f32); // zoom-in speed
        e.set_global(0x011c_d568, 500.0f32); // the largest distance
        e.set_global(0x011c_d614, 1.0f32);
        e.set_global(0x011c_da94, 10.0f32);
        e.register_double(0x00b0_022c, |_, _| int(1)); // slot 0x22c: first person?
        put_vtable(
            &mut e,
            this.addr(),
            0x026a_0000,
            &[
                (0x2bc, 0x00b0_02bc),
                (0x214, 0x00b0_0214),
                (0x2c4, 0x00b0_02c4),
                (0x22c, 0x00b0_022c),
            ],
        );
        let flags = Ptr::<()>::new(e.mem.alloc(4));
        let result = e
            .call(0x0094_45b0, &args![this, 0.5f32, false, flags, 0u32])
            .u8();
        // Zoom 120 / 120 = 1 * distance 100 * speed 0.5 = 50 less.
        assert_eq!(e.global::<f32>(ZOOM_DISTANCE), 50.0);
        // The pitch step (10 * 3.0 * view scale 1.0) is applied inverted.
        assert_eq!(
            called(&e, 0x0093_1e50),
            vec![vec![this.addr(), (-30.0f32).to_bits()]]
        );
        assert_eq!(result, 1);
    }

    #[test]
    fn the_vats_target_camera_aims_at_the_target_and_masks_the_input_flags() {
        let (mut e, this) = look_world(4);
        let target_pos = e.mem.alloc(12);
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(target_pos + 4 * i as u32, *v);
        }
        let target = answering_object(
            &mut e,
            0x026c_0000,
            &[(0x1d0, 0), (0x1f4, target_pos), (0x100, 0)],
        );
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(action + 0xc, target);
        e.mem.set_u32(action + 0x10, 0xffff_ffff);
        e.register_double(0x009c_71c0, move |_, _| int(action));
        e.register_double(0x009a_88e0, |e, a| {
            for (i, v) in [0.25f32, 0.0, 0.75].iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        e.register(0x0088_46e0, |_, _| int(0xffff));
        let flags = Ptr::<()>::new(e.mem.alloc(4));
        let result = e
            .call(0x0094_45b0, &args![this, 0.1f32, true, flags, 0u32])
            .u8();
        assert_eq!(result, 1);
        // The angles come from the target position: (out, player, x, y, z).
        let angles = &called(&e, 0x009a_88e0)[0];
        assert_eq!(angles[1], this.addr());
        assert_eq!(
            angles[2..],
            [10.0f32.to_bits(), 20.0f32.to_bits(), 30.0f32.to_bits()]
        );
        // Heading through slot 0x2C4, pitch through SetLooking.
        assert_eq!(
            called(&e, 0x00b0_02c4),
            vec![vec![this.addr(), 0.75f32.to_bits()]]
        );
        assert_eq!(
            called(&e, 0x0093_1d90),
            vec![vec![this.addr(), 0.25f32.to_bits()]]
        );
        assert_eq!(called(&e, 0x009c_7240), vec![vec![VATS_OBJECT]]);
        assert_eq!(e.mem.u16(flags.addr()), 0xcc00);
    }

    #[test]
    fn the_follow_camera_without_a_target_only_runs_its_pre_update() {
        let (mut e, this) = look_world(2);
        let flags = Ptr::<()>::new(e.mem.alloc(4));
        assert_eq!(
            e.call(0x0094_45b0, &args![this, 0.1f32, false, flags, 0u32])
                .u8(),
            0
        );
        assert_eq!(
            called(&e, 0x0095_de30),
            vec![vec![this.addr(), 0.1f32.to_bits()]]
        );
        assert!(called(&e, 0x009a_8af0).is_empty());
    }

    #[test]
    fn the_follow_camera_turns_the_player_towards_a_target_with_two_curves() {
        let (mut e, this) = look_world(1);
        let heading_curve = e.new_object::<TimedCurve>();
        let pitch_curve = e.new_object::<TimedCurve>();
        e.set_global(HEADING_CURVE, heading_curve.addr());
        e.set_global(PITCH_CURVE, pitch_curve.addr());
        // A target without a 3D root and not an actor, at a position.
        let target_pos = e.mem.alloc(12);
        let target = answering_object(
            &mut e,
            0x026c_0000,
            &[(0x1d0, 0), (0x1f4, target_pos), (0x100, 0)],
        );
        e.set_global(LOOK_TARGET_ACTOR, target);
        e.register_double(0x009a_8af0, |e, a| {
            // angles: x (pitch) 0.5, z (heading) 1.0.
            for (i, v) in [0.5f32, 0.0, 1.0].iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        e.register(0x0084_d030, |_, _| float(0.25));
        e.register(0x0045_bb80, |_, _| int(0x011a_0000));
        e.set_global(0x011d_48dc, 2.0f32); // the duration for mode 1
        e.set_global(SMOOTH_LOOK_ENABLED, 1u8);
        let flags = Ptr::<()>::new(e.mem.alloc(4));
        let result = e
            .call(0x0094_45b0, &args![this, 0.1f32, false, flags, 0u32])
            .u8();
        assert_eq!(result, 0);
        // Both curves were restarted over the setting's duration and advanced one step.
        assert_eq!(e.get(heading_curve, TimedCurve::duration), 2.0);
        assert_eq!(e.get(heading_curve, TimedCurve::to), 1.0);
        assert_eq!(e.get(pitch_curve, TimedCurve::to), 0.5);
        assert_eq!(e.get(heading_curve, TimedCurve::remaining), 1.75);
        // The player's heading and pitch are set from the curves, between the
        // start (0) and the target.
        let heading = f32::from_bits(called(&e, 0x00b0_02c4)[0][1]);
        let pitch = f32::from_bits(called(&e, 0x0093_1d90)[0][1]);
        assert!(heading > 0.0 && heading < 1.0, "heading {heading}");
        assert!(pitch > 0.0 && pitch < 0.5, "pitch {pitch}");
        // The VATS menu is told 0: the curves are not over yet.
        assert_eq!(called(&e, 0x007f_3bd0), vec![vec![0]]);
        assert_eq!(e.global::<u32>(LAST_LOOK_TARGET), target);
    }

    /// Every callee of `009466d0` outside this file.
    const INPUT_CALLEES: &[u32] = &[
        0x0040_3e20,
        0x0040_4eb0,
        0x0040_4ee0,
        0x0041_6870,
        0x0043_01b0,
        0x0043_6aa0,
        0x0043_7bd0,
        0x0043_7bf0,
        0x0043_9180,
        0x0043_9f50,
        0x0044_ddc0,
        0x0045_3a70,
        0x0045_c670,
        0x0048_3710,
        0x004a_0c10,
        0x004b_71d0,
        0x004b_7210,
        0x004c_0c90,
        0x004f_8960,
        0x0050_8070,
        0x0050_d4a0,
        0x0052_5980,
        0x0055_8310,
        0x0056_10f0,
        0x0056_15d0,
        0x0056_8680,
        0x0057_3170,
        0x005a_03f0,
        0x005a_1e50,
        0x005a_2030,
        0x005c_0880,
        0x005f_2670,
        0x0064_50c0,
        0x0068_15c0,
        0x0069_3ef0,
        0x006a_9540,
        0x0070_2360,
        0x0070_3350,
        0x0070_62e0,
        0x0085_09a0,
        0x0085_09f0,
        0x0087_7720,
        0x0088_43a0,
        0x0088_46e0,
        0x0088_49c0,
        0x0088_4f80,
        0x0088_85e0,
        0x0089_4cc0,
        0x0089_4d60,
        0x0089_4f90,
        0x0089_50f0,
        0x0089_5110,
        0x008a_0b10,
        0x008a_16d0,
        0x008a_6840,
        0x008a_6970,
        0x008a_7570,
        0x008b_a600,
        0x008b_b650,
        0x008b_bc10,
        0x008c_3c40,
        0x008c_e390,
        0x008d_3550,
        0x008d_8520,
        0x0093_06d0,
        0x0093_1d70,
        0x0094_df80,
        0x0095_0a60,
        0x0095_f6a0,
        0x0096_13c0,
        0x0096_7ae0,
        0x009b_b080,
        0x009e_a360,
        0x009e_a570,
        0x00a2_3390,
        0x00a2_4280,
        0x00a2_4660,
        0x00ad_7550,
        0x00ad_8830,
        0x00ec_43fb,
        0x00ec_7d40,
    ];

    /// Every callee of `0094a8c0` and of `009481d0` outside this file.
    const UFO_CALLEES: &[u32] = &[
        0x0040_3e20,
        0x0040_8820,
        0x0040_8d60,
        0x0040_ebd0,
        0x0043_9180,
        0x0043_d410,
        0x0043_f8d0,
        0x0043_fa80,
        0x0044_0460,
        0x0045_7fe0,
        0x0045_bb20,
        0x0045_c670,
        0x004a_0c90,
        0x004b_4500,
        0x004b_71d0,
        0x004b_7210,
        0x0052_4ac0,
        0x0055_8310,
        0x0057_5d70,
        0x0058_6170,
        0x0063_c8a0,
        0x0068_15c0,
        0x006f_ca90,
        0x006f_cdb0,
        0x0087_7720,
        0x00a2_3390,
        0x00a2_39e0,
        0x00a2_4660,
        0x00a5_9c60,
        0x0048_f7f0,
        0x0049_1040,
        0x005f_3780,
        0x0063_9aa0,
        0x0070_f490,
        0x008a_7570,
        0x008e_eaa0,
        0x0095_0a60,
        0x0096_6a00,
    ];

    /// The address of the double behind slot `slot` of the table at `base`
    /// (see [`zero_table`]).
    fn slot_target(base: u32, slot: u32) -> u32 {
        0x00b0_0000 + (base & 0xf_ffff) / 0x10 + slot
    }

    /// A vtable at `base` whose every slot (0 to 0x7FC) answers 0, except the
    /// given ones; the doubles are found with [`slot_target`].
    fn zero_table(e: &mut Engine, base: u32, answers: &[(u32, u32)]) {
        let mut slots = Vec::new();
        for slot in (0..0x800u32).step_by(4) {
            let target = slot_target(base, slot);
            e.register(target, |_, _| Ret::default());
            slots.push((slot, target));
        }
        for (slot, answer) in answers {
            let answer = *answer;
            e.register_double(slot_target(base, *slot), move |_, _| int(answer));
        }
        vtable_only(e, base, &slots);
    }

    /// An object with a [`zero_table`] vtable.
    fn zero_object(e: &mut Engine, base: u32, answers: &[(u32, u32)]) -> u32 {
        zero_table(e, base, answers);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, base);
        object
    }

    /// The world of `009466d0`: every outside callee answers 0, the settings
    /// are by address, the player has a zero vtable, a process and a mover
    /// that answer 0, and is the player singleton.
    fn input_world() -> (Engine, Ptr<PlayerCharacter>) {
        let mut e = engine();
        exe_constants(&mut e);
        for table in [INPUT_CALLEES, ATTACK_CALLEES, LOOK_CALLEES, UFO_CALLEES] {
            stub(&mut e, table);
        }
        settings_by_address(&mut e);
        e.register(0x0087_7720, |_, _| int(0x1000));
        let animations = [e.mem.alloc(0x400), e.mem.alloc(0x400)];
        e.register_double(0x0095_0a60, move |_, a| {
            int(animations[(a[1] != 0) as usize])
        });
        e.register(0x0084_d030, |_, _| float(0.5));
        e.register(0x00ec_7d40, |_, a| int((a[0] as i32).unsigned_abs()));
        e.register(0x0045_3a70, |_, _| int(0x4600));
        e.register(0x00ad_7550, |_, a| int(a[1]));
        // The point arithmetic returns its output buffer.
        for addr in [0x0045_bb20, 0x0043_9e90, 0x0043_9ef0, 0x004b_4500] {
            e.register(addr, |_, a| int(a[1]));
        }
        e.register(0x0041_6870, |e, a| {
            for i in 1..4 {
                e.mem.set_u32(a[0] + 4 * (i - 1), a[i as usize]);
            }
            int(a[0])
        });
        e.register(0x0055_8310, |_, _| int(0x011a_0000));
        e.register(0x006a_9540, |_, a| int(a[0]));
        e.register(0x0043_6aa0, |_, _| int(0x011a_0100));
        e.register(0x0045_bb80, |_, _| int(0x011a_0200));
        let this = player(&mut e);
        zero_table(&mut e, 0x0270_0000, &[]);
        e.mem.set_u32(this.addr(), 0x0270_0000);
        let process = zero_object(&mut e, 0x0271_0000, &[]);
        e.mem.set_u32(this.addr() + 0x68, process);
        let mover = zero_object(&mut e, 0x0272_0000, &[]);
        e.mem.set_u32(this.addr() + 0x190, mover);
        e.set_global(0x011d_ea3c, this.addr());
        e.set_global(0x011d_e134, 0x4500u32);
        e.call_log = Some(vec![]);
        (e, this)
    }

    /// Makes the button query true for the given (button, state) pairs.
    fn buttons(e: &mut Engine, pressed: &'static [(u32, u32)]) {
        e.register_double(0x00a2_4660, move |_, a| {
            int(pressed.iter().any(|(b, s)| a[1] == *b && a[2] == *s) as u32)
        });
    }

    #[test]
    fn the_input_handler_with_the_controls_skipped_only_runs_the_hotkeys() {
        let (mut e, this) = input_world();
        buttons(&mut e, &[(0x19, 1), (0x1a, 1)]);
        e.call(0x0094_66d0, &args![this, 0.5f32, true]);
        assert_eq!(called(&e, 0x0085_09a0), vec![vec![0x4500]]);
        assert_eq!(called(&e, 0x0085_09f0), vec![vec![0x4500]]);
        assert_eq!(e.mem.u8(0x011e_0be5), 0);
        assert_eq!(e.mem.u8(0x011e_0be4), 0);
        // The profiling scope opens and closes around it.
        let open = called(&e, 0x0040_4eb0);
        assert_eq!(open.len(), 1);
        assert_eq!(open[0][1..], [0x34, 1, UPDATE_GUARD_FILE, 0x17a3]);
        assert_eq!(called(&e, 0x0040_4ee0), vec![vec![open[0][0]]]);
        // No player control ran.
        assert!(called(&e, 0x0088_46e0).is_empty());
        // The UFO camera still ran first (it reads the look axes).
        assert!(!called(&e, 0x0087_7720).is_empty());
    }

    #[test]
    fn the_hotkeys_do_not_run_again_while_their_guard_is_set() {
        let (mut e, this) = input_world();
        buttons(&mut e, &[(0x19, 1), (0x1a, 1)]);
        e.mem.set_u8(0x011e_0be5, 1);
        e.mem.set_u8(0x011e_0be4, 1);
        e.call(0x0094_66d0, &args![this, 0.5f32, true]);
        assert!(called(&e, 0x0085_09a0).is_empty());
        assert!(called(&e, 0x0085_09f0).is_empty());
    }

    #[test]
    fn the_sneak_button_toggles_the_crouch_flag_and_plays_a_sound() {
        let (mut e, this) = input_world();
        buttons(&mut e, &[(8, 1)]);
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        // Crouching down: "NPCHumanCrouchDown", state 0xA, flag 0x400 to the mover.
        assert_eq!(called(&e, 0x00ad_7550)[0][2..], [0x0108_b050, 0x0004_0102]);
        assert_eq!(
            called(&e, slot_target(0x0270_0000, 0x340)),
            vec![vec![this.addr(), 0xa]]
        );
        assert_eq!(called(&e, slot_target(0x0272_0000, 0xc))[0][1], 0x400);
        assert_eq!(called(&e, 0x0088_4f80), vec![vec![this.addr()]]);
        assert_eq!(called(&e, 0x0048_3710).len(), 1);
        // Standing up again.
        let (mut e, this) = input_world();
        buttons(&mut e, &[(8, 1)]);
        e.register(0x0088_46e0, |_, _| int(0x400));
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        assert_eq!(called(&e, 0x00ad_7550)[0][2..], [0x0108_b064, 0x0004_0102]);
        assert_eq!(
            called(&e, slot_target(0x0270_0000, 0x340)),
            vec![vec![this.addr(), 0xb]]
        );
        assert_eq!(called(&e, slot_target(0x0272_0000, 0xc))[0][1], 0);
    }

    #[test]
    fn the_sneak_button_does_nothing_while_something_blocks_it() {
        let (mut e, this) = input_world();
        buttons(&mut e, &[(8, 1)]);
        zero_table(&mut e, 0x0270_0000, &[(0x234, 1)]);
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        assert!(called(&e, 0x00ad_7550).is_empty());
        assert!(called(&e, slot_target(0x0270_0000, 0x340)).is_empty());
        // The button still counts as handled.
        assert_eq!(called(&e, 0x0088_4f80).len(), 1);
    }

    #[test]
    fn the_auto_move_button_toggles_auto_move_and_any_direction_cancels_it() {
        let (mut e, this) = input_world();
        buttons(&mut e, &[(0xb, 1)]);
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        assert!(e.get(this, PlayerCharacter::bAutoMove));
        // Auto-move keeps pressing forward (button 0).
        assert!(called(&e, 0x00a2_4280).contains(&vec![0x1000, 0]));
        // A direction key cancels it.
        buttons(&mut e, &[(1, 0)]);
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        assert!(!e.get(this, PlayerCharacter::bAutoMove));
    }

    #[test]
    fn the_analog_move_stick_sets_the_move_speed_factor_and_the_direction_bits() {
        let (mut e, this) = input_world();
        e.register(0x004b_71d0, |_, _| int(1));
        e.register(0x00a2_3390, |_, a| {
            int(if a[2] == 7 { (-12000i32) as u32 } else { 0 })
        });
        e.register(0x008a_0b10, |_, _| float(3.0));
        e.set_global(0x011e_0964, 1.0e-4f32);
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        // Stick x is negative: button 2 pressed for the input, bit 4 of the flags.
        assert!(called(&e, 0x00a2_4280).contains(&vec![0x1000, 2]));
        assert!((e.global::<f32>(TURN_SPEED) - 1.2).abs() < 1e-5);
        // 12000 * 0.02 is clamped to 1.
        assert_eq!(e.global::<f32>(MOVE_SPEED_FACTOR), 1.0);
        assert_eq!(called(&e, slot_target(0x0272_0000, 0xc))[0][1], 4);
        // Both movement vectors are scaled by the speed.
        let scaled = called(&e, 0x0043_9180);
        let last = scaled.len();
        assert_eq!(scaled[last - 2][1], 3.0f32.to_bits());
        assert_eq!(scaled[last - 1][1], 3.0f32.to_bits());
    }

    #[test]
    fn the_aim_button_raises_the_iron_sights_of_a_drawn_weapon() {
        let (mut e, this) = input_world();
        buttons(&mut e, &[(6, 1)]);
        let process = zero_object(&mut e, 0x0273_0000, &[(0x148, 0x5100), (0x3f8, 1)]);
        e.mem.set_u32(this.addr() + 0x68, process);
        e.register(0x0044_ddc0, |_, a| {
            int(if a[0] == 0x5100 { 0x6000 } else { 0 })
        });
        e.register(0x008a_16d0, |_, _| int(1));
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        assert_eq!(called(&e, 0x008b_b650), vec![vec![this.addr(), 1, 0, 0]]);
        assert!(called(&e, 0x0089_4cc0).is_empty());
        // Releasing it lowers both again.
        let (mut e, this) = input_world();
        buttons(&mut e, &[(6, 2)]);
        e.register(0x008b_bc10, |_, _| int(1));
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        assert_eq!(called(&e, 0x008b_b650), vec![vec![this.addr(), 0, 0, 0]]);
        assert_eq!(called(&e, 0x0089_4cc0), vec![vec![this.addr(), 0]]);
    }

    #[test]
    fn the_activate_button_activates_the_reference_under_the_cursor() {
        let (mut e, this) = input_world();
        buttons(&mut e, &[(5, 1)]);
        let reference = zero_object(&mut e, 0x0274_0000, &[]);
        e.register_double(0x0070_3350, move |_, _| int(reference));
        e.register(0x0057_3170, |_, _| int(1));
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        assert_eq!(called(&e, 0x0070_62e0), vec![vec![0, 0, 0]]);
        assert_eq!(
            called(&e, 0x0057_3170),
            vec![vec![reference, this.addr(), 0, 0, 1]]
        );
    }

    #[test]
    fn the_activate_button_falls_back_to_the_process_object() {
        let (mut e, this) = input_world();
        buttons(&mut e, &[(5, 1)]);
        let process = zero_object(&mut e, 0x0273_0000, &[(0x4c8, 0x5900)]);
        e.mem.set_u32(this.addr() + 0x68, process);
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        // No reference: activate what the process offers.
        assert_eq!(
            called(&e, 0x0057_3170),
            vec![vec![0x5900, this.addr(), 0, 0, 1]]
        );
    }

    #[test]
    fn the_activate_button_does_nothing_while_a_menu_is_open() {
        let (mut e, this) = input_world();
        buttons(&mut e, &[(5, 1)]);
        e.register(0x0070_2360, |_, _| int(1));
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        assert!(called(&e, 0x0070_62e0).is_empty());
        assert!(called(&e, 0x0057_3170).is_empty());
    }

    #[test]
    fn the_attack_button_checks_the_controller_when_moving() {
        let (mut e, this) = input_world();
        buttons(&mut e, &[(0xc, 1)]);
        e.register(0x0093_06d0, |_, _| int(0x5a00));
        e.register(0x0062_1270, |_, _| int(1));
        zero_table(&mut e, 0x0270_0000, &[(0x254, 1)]);
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        assert_eq!(called(&e, 0x0095_f6a0), vec![vec![this.addr()]]);
        // Once to decide it moves and once more because it can start.
        assert_eq!(called(&e, 0x0062_1270).len(), 2);
    }

    #[test]
    fn holding_the_draw_button_long_enough_sheathes_the_weapon() {
        let (mut e, this) = input_world();
        buttons(&mut e, &[(7, 0)]);
        e.register(0x0088_43a0, |_, _| int(1));
        e.register(0x008a_7570, |_, _| int(0xffff_ffff));
        e.register(0x008a_6970, |_, _| int(1));
        e.set_global(0x011c_dfcc, 0.5f32); // the hold time
        e.call(0x0094_66d0, &args![this, 0.6f32, false]);
        assert_eq!(e.global::<f32>(DRAW_BUTTON_TIMER), 0.6);
        assert_eq!(called(&e, 0x008a_6840), vec![vec![this.addr(), 0]]);
        assert_eq!(e.mem.u8(DRAW_INPUT_HANDLED), 1);
    }

    #[test]
    fn the_look_handler_runs_unless_the_vats_mode_is_zero() {
        let (mut e, this) = input_world();
        e.register(0x0044_ddc0, |_, _| int(9));
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        // Mode 9: the mode query, then the look handler asks again.
        assert_eq!(called(&e, 0x0044_ddc0), vec![vec![VATS_OBJECT]; 2]);
        let (mut e, this) = input_world();
        e.call(0x0094_66d0, &args![this, 0.5f32, false]);
        assert_eq!(called(&e, 0x0044_ddc0), vec![vec![VATS_OBJECT]]);
    }

    /// Every callee of `0094a0c0` outside this file.
    const CAMERA_CALLEES: &[u32] = &[
        0x0040_3e20,
        0x0043_9e90,
        0x0043_9ef0,
        0x0044_ddc0,
        0x0045_78c0,
        0x0045_7910,
        0x0045_7990,
        0x0045_bb20,
        0x0045_bb80,
        0x0062_0bc0,
        0x0062_1370,
        0x0062_1440,
        0x0062_14d0,
        0x0062_1ad0,
        0x0062_1c40,
        0x0063_c8a0,
        0x0068_15c0,
        0x0070_2680,
        0x0084_d030,
        0x0088_53a0,
        0x008c_4640,
        0x0093_06d0,
        0x00ec_408c,
    ];

    /// Every callee of `0094ae40` outside this file.
    const UPDATE_CAMERA_CALLEES: &[u32] = &[
        0x0040_3e20,
        0x0040_8860,
        0x0041_6870,
        0x0043_9180,
        0x0043_9e90,
        0x0043_9ef0,
        0x0043_c490,
        0x0043_d410,
        0x0043_f8d0,
        0x0043_fa80,
        0x0044_0460,
        0x0044_ddc0,
        0x0044_edb0,
        0x0045_3700,
        0x0045_3a70,
        0x0045_78c0,
        0x0045_7990,
        0x0045_bb20,
        0x0045_bb80,
        0x0045_bba0,
        0x0045_bbe0,
        0x0045_c670,
        0x0046_1130,
        0x0047_6930,
        0x0049_6550,
        0x004a_0c10,
        0x004a_0c90,
        0x004a_b230,
        0x004b_1460,
        0x004b_3ae0,
        0x004b_4500,
        0x004b_6190,
        0x004e_9c10,
        0x0050_8070,
        0x0052_4ac0,
        0x0055_8310,
        0x0055_9450,
        0x0056_7400,
        0x0057_5d70,
        0x0057_d0a0,
        0x0058_6170,
        0x0058_d630,
        0x005d_c270,
        0x0062_1440,
        0x0063_c8a0,
        0x0066_29f0,
        0x0068_15c0,
        0x006a_9540,
        0x006f_ca90,
        0x006f_cdb0,
        0x0071_33b0,
        0x0088_5560,
        0x008b_70d0,
        0x008d_1b30,
        0x008d_6f30,
        0x0093_1d70,
        0x0093_3840,
        0x0095_0290,
        0x0095_0b00,
        0x0095_0bb0,
        0x0095_edf0,
        0x0097_4d90,
        0x009c_8ac0,
        0x00a5_92c0,
        0x00a5_9540,
        0x00a5_9c60,
        0x00a7_01b0,
        0x00a7_12f0,
        0x00ad_78b0,
        0x00ad_7940,
        0x00ad_7990,
        0x00b5_4000,
        0x00c5_2020,
        0x00c8_e160,
        0x00ec_658f,
    ];

    /// Real `NiPoint3` arithmetic on guest memory for the camera tests:
    /// subtraction, addition, scaling (output buffer first), the in-place
    /// forms, the length, the unitise-and-length, and the constructor.
    fn vector_math(e: &mut Engine) {
        fn load(e: &Engine, p: u32) -> [f64; 3] {
            [
                e.mem.f32(p) as f64,
                e.mem.f32(p + 4) as f64,
                e.mem.f32(p + 8) as f64,
            ]
        }
        fn store(e: &mut Engine, p: u32, v: [f64; 3]) {
            for (i, x) in v.iter().enumerate() {
                e.mem.set_f32(p + 4 * i as u32, *x as f32);
            }
        }
        e.register(0x0043_9ef0, |e, a| {
            let (x, y) = (load(e, a[0]), load(e, a[2]));
            store(e, a[1], [x[0] - y[0], x[1] - y[1], x[2] - y[2]]);
            int(a[1])
        });
        e.register(0x0043_9e90, |e, a| {
            let (x, y) = (load(e, a[0]), load(e, a[2]));
            store(e, a[1], [x[0] + y[0], x[1] + y[1], x[2] + y[2]]);
            int(a[1])
        });
        e.register(0x0045_bb20, |e, a| {
            let (x, f) = (load(e, a[0]), f32::from_bits(a[2]) as f64);
            store(e, a[1], [x[0] * f, x[1] * f, x[2] * f]);
            int(a[1])
        });
        e.register(0x0043_9180, |e, a| {
            let (x, f) = (load(e, a[0]), f32::from_bits(a[1]) as f64);
            store(e, a[0], [x[0] * f, x[1] * f, x[2] * f]);
            int(a[0])
        });
        e.register(0x0063_c8a0, |e, a| {
            let (x, y) = (load(e, a[0]), load(e, a[1]));
            store(e, a[0], [x[0] + y[0], x[1] + y[1], x[2] + y[2]]);
            int(a[0])
        });
        e.register(0x0045_78c0, |e, a| {
            let (x, y) = (load(e, a[0]), load(e, a[1]));
            store(e, a[0], [x[0] - y[0], x[1] - y[1], x[2] - y[2]]);
            int(a[0])
        });
        e.register(0x0045_7990, |e, a| {
            let x = load(e, a[0]);
            float((x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt())
        });
        e.register(0x0045_7910, |e, a| {
            let x = load(e, a[0]);
            let length = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
            if length > 0.0 {
                store(e, a[0], [x[0] / length, x[1] / length, x[2] / length]);
            }
            float(length)
        });
        e.register(0x0041_6870, |e, a| {
            for i in 1..4 {
                e.mem.set_u32(a[0] + 4 * (i - 1), a[i as usize]);
            }
            int(a[0])
        });
        // The matrix-vector product with the identity matrix.
        e.register(0x004b_4500, |e, a| {
            let v = load(e, a[2]);
            store(e, a[1], v);
            int(a[1])
        });
    }

    /// Stubs that return a pointer into zeroed memory (a point, a matrix, a
    /// node) for the camera tests.
    fn pointer_stubs(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| int(0x011a_0300));
        }
    }

    fn camera_distance_world() -> (Engine, Ptr<PlayerCharacter>) {
        let mut e = engine();
        exe_constants(&mut e);
        stub(&mut e, CAMERA_CALLEES);
        settings_by_address(&mut e);
        vector_math(&mut e);
        e.register(0x0084_d030, |_, _| float(0.1));
        let this = player(&mut e);
        e.call_log = Some(vec![]);
        (e, this)
    }

    /// Calls `0094a0c0` with the camera at `camera` and the anchor at `anchor`;
    /// returns the camera position after the call.
    fn move_camera(
        e: &mut Engine,
        this: Ptr<PlayerCharacter>,
        camera: [f32; 3],
        anchor: [f32; 3],
        snap: bool,
    ) -> [f32; 3] {
        let position = e.mem.alloc(12);
        let anchor_at = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_f32(position + 4 * i as u32, camera[i]);
            e.mem.set_f32(anchor_at + 4 * i as u32, anchor[i]);
        }
        e.call(
            0x0094_a0c0,
            &args![
                this,
                Ptr::<()>::new(position),
                Ptr::<()>::new(anchor_at),
                snap
            ],
        );
        read_floats(e, position)
    }

    #[test]
    fn the_camera_distance_moves_towards_the_wanted_distance_by_one_step() {
        let (mut e, this) = camera_distance_world();
        e.set_global(CAMERA_DISTANCE, 5.0f32);
        e.set_global(0x011c_d288, 10.0f32); // rate: step = 10 * 0.1
        e.set_global(0x011c_d568, 100.0f32); // the largest distance
        e.set_global(0x011c_da94, 2.0f32); // the smallest
        e.set_global(PLAYER_FADE, 1.0f32);
        let position = move_camera(&mut e, this, [8.0, 0.0, 0.0], [0.0, 0.0, 0.0], false);
        // The camera is further than the distance (5 + 1): it moves 1 closer.
        assert_eq!(e.global::<f32>(CAMERA_DISTANCE), 6.0);
        assert_eq!(position, [6.0, 0.0, 0.0]);
        assert_eq!(read_floats(&e, CAMERA_SMOOTHED), [8.0, 0.0, 0.0]);
        // Nothing hides the player: the fade stays full.
        assert!(called(&e, 0x008c_4640).is_empty());
    }

    #[test]
    fn the_camera_distance_snaps_to_the_current_distance_when_asked() {
        let (mut e, this) = camera_distance_world();
        e.set_global(CAMERA_DISTANCE, 5.0f32);
        e.set_global(0x011c_d568, 100.0f32);
        e.set_global(PLAYER_FADE, 1.0f32);
        let position = move_camera(&mut e, this, [0.0, 8.0, 0.0], [0.0, 0.0, 0.0], true);
        // The camera is at distance 8; the distance snaps there; the wanted
        // distance is not limited by the step.
        assert_eq!(e.global::<f32>(CAMERA_DISTANCE), 8.0);
        assert_eq!(position, [0.0, 8.0, 0.0]);
    }

    #[test]
    fn a_camera_caster_hit_shortens_the_distance_and_marks_the_view_blocked() {
        let (mut e, this) = camera_distance_world();
        e.set_global(CAMERA_DISTANCE, 5.0f32);
        e.set_global(0x011c_d288, 1000.0f32);
        e.set_global(0x011c_d568, 100.0f32);
        e.set_global(0x011c_da94, 1.0f32);
        e.set_global(0x011e_0934, 1.0f32); // the margin: half of it is kept free
        e.set_global(PLAYER_FADE, 1.0f32);
        let caster = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0x21c, caster);
        e.register(0x0062_0bc0, |_, _| int(1));
        // The hit point is 3 units away; the camera at 8 units.
        e.register(0x0045_7990, |_, _| float(3.0));
        let position = move_camera(&mut e, this, [8.0, 0.0, 0.0], [0.0, 0.0, 0.0], false);
        // length 8 - 3 > 2: blocked; the distance becomes 3 - 0.5 = 2.5 and the
        // camera is placed there.
        assert_eq!(e.mem.u8(CAMERA_BLOCKED), 1);
        assert_eq!(e.global::<f32>(CAMERA_DISTANCE), 2.5);
        assert_eq!(position, [2.5, 0.0, 0.0]);
        assert_eq!(called(&e, 0x0062_0bc0).len(), 1);
        // Without a world the caster is given the controller's shape (null here).
        assert_eq!(called(&e, 0x0062_1370), vec![vec![caster, 0]]);
    }

    #[test]
    fn the_camera_distance_zooms_out_with_the_third_person_toggle() {
        let (mut e, this) = camera_distance_world();
        // The player wants the third person but is in the first: mode +1.
        e.set(this, PlayerCharacter::bWant3rdPerson, true);
        e.set_global(CAMERA_DISTANCE, 1.0f32);
        e.set_global(ZOOM_DISTANCE, 5.0f32);
        e.set_global(0x011c_da94, 1.0f32); // near
        e.set_global(0x011c_de14, 11.0f32); // far
        e.set_global(0x011c_dd60, 4.0f32); // blend scale
        e.set_global(0x011c_cfd8, 0.0f32); // blend base
        e.set_global(0x011c_d288, 1.0f32);
        e.set_global(0x011c_d568, 100.0f32);
        e.set_global(PLAYER_FADE, 1.0f32);
        let position = move_camera(&mut e, this, [20.0, 0.0, 0.0], [0.0, 0.0, 0.0], false);
        // blend = 4 * ((5 - 1) / (11 - 1)) = 1.6; step = 0.1 * 1 * 1.6 = 0.16.
        let distance = e.global::<f32>(CAMERA_DISTANCE);
        assert!((distance - 1.16).abs() < 1e-5, "distance {distance}");
        assert!((position[0] - 1.16).abs() < 1e-5);
        // Travelling: the speed estimate is the distance moved over the frame time.
        assert!(e.global::<f32>(0x011e_07dc) > 0.0);
    }

    #[test]
    fn the_player_fades_out_when_the_camera_is_obstructed() {
        let (mut e, this) = camera_distance_world();
        e.set_global(CAMERA_DISTANCE, 5.0f32);
        e.set_global(0x011c_d568, 100.0f32);
        e.set_global(PLAYER_FADE, 1.0f32);
        let caster = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0x21c, caster);
        e.register(0x0062_14d0, |_, _| int(1));
        move_camera(&mut e, this, [8.0, 0.0, 0.0], [0.0, 0.0, 0.0], false);
        // 1.0 - 0.1 * 4.
        assert!((e.global::<f32>(PLAYER_FADE) - 0.6).abs() < 1e-6);
        assert_eq!(called(&e, 0x008c_4640), vec![vec![this.addr()]]);
    }

    fn ufo_world() -> (Engine, Ptr<PlayerCharacter>) {
        let mut e = engine();
        exe_constants(&mut e);
        stub(&mut e, UFO_CALLEES);
        settings_by_address(&mut e);
        vector_math(&mut e);
        e.register(0x0087_7720, |_, _| int(0x1000));
        e.register(0x0045_c670, |_, _| int(0x4800));
        e.register(0x0055_8310, |_, a| int(a[0] + 1));
        let this = player(&mut e);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn the_ufo_camera_turns_with_the_look_axes_and_moves_with_the_keys() {
        let (mut e, this) = ufo_world();
        e.register(0x00a2_39e0, |_, a| int(if a[1] == 1 { 50 } else { 20 }));
        e.register(0x00a2_4660, |_, a| int((a[1] == 0 && a[2] == 0) as u32));
        e.set_global(0x011e_0a6c, 0.1f32); // look speed
        e.set_global(0x011e_090c, 2.0f32); // move speed
        e.register(0x0045_7fe0, |_, _| int(5000));
        e.set_global(0x011e_0bf8, 4000u32);
        e.register(0x0040_ebd0, |_, a| {
            float(f32::from_bits(a[0]).min(f32::from_bits(a[1])) as f64)
        });
        e.register(0x0057_5d70, |_, _| int(0x7000));
        e.register(0x0058_6170, |_, _| int(0x7100));
        e.call(0x0094_a8c0, &args![this]);
        // heading += 0.1 * 50; pitch += 0.1 * 20.
        assert!((e.get(this, PlayerCharacter::fUFOCameraHeading) - 5.0).abs() < 1e-6);
        assert!((e.get(this, PlayerCharacter::fUFOCameraPitch) - 2.0).abs() < 1e-6);
        // Key 0 moves along y by 10, times the elapsed second (<= 1) times 10,
        // times the move speed 2.
        assert_eq!(
            read_floats(&e, member(this, PlayerCharacter::UFOCameraPos)),
            [0.0, 200.0, 0.0]
        );
        assert_eq!(e.global::<u32>(0x011e_0bf8), 5000);
        // The camera node follows and the terrain manager is updated.
        assert_eq!(called(&e, 0x0044_0460).len(), 1);
        assert_eq!(
            called(&e, 0x006f_ca90),
            vec![vec![
                0x7100,
                member(this, PlayerCharacter::UFOCameraPos),
                0xf
            ]]
        );
        assert_eq!(called(&e, 0x006f_cdb0).len(), 1);
    }

    /// The world of `0094ae40`: all outside callees answer 0 or point into
    /// zeroed memory, point arithmetic is real, the player's vtable gives a
    /// heading of 1.5, a pitch of 0.25 and the position pointer
    /// `011a0300`, and the world space has no cell.
    fn update_camera_world() -> (Engine, Ptr<PlayerCharacter>) {
        let mut e = engine();
        exe_constants(&mut e);
        stub(&mut e, UPDATE_CAMERA_CALLEES);
        stub(&mut e, CAMERA_CALLEES);
        settings_by_address(&mut e);
        vector_math(&mut e);
        pointer_stubs(
            &mut e,
            &[
                0x0045_bb80,
                0x0045_bba0,
                0x0045_bbe0,
                0x004e_9c10,
                0x004b_3ae0,
                0x0043_c490,
                0x0046_1130,
                0x0055_8310,
                0x0055_9450,
                0x0066_29f0,
            ],
        );
        e.register(0x006a_9540, |_, a| int(a[0]));
        e.register(0x0043_f8d0, |_, a| int(a[1]));
        e.register(0x0045_c670, |_, _| int(0x011a_4000));
        e.register(0x0045_3a70, |_, _| int(0x4600));
        e.register(0x0084_d030, |_, _| float(0.1));
        e.register(0x0093_1d70, |_, _| float(0.25));
        let this = player(&mut e);
        e.set(this, PlayerCharacter::fWorldFOV, 75.0);
        e.set(this, PlayerCharacter::f3rdPersonFOV, 60.0);
        zero_table(&mut e, 0x0275_0000, &[(0x1f4, 0x011a_0300)]);
        e.register_double(slot_target(0x0275_0000, 0x2bc), |_, _| float(1.5));
        e.mem.set_u32(this.addr(), 0x0275_0000);
        let process = zero_object(&mut e, 0x0276_0000, &[(0x1b8, 0x011a_0300)]);
        e.mem.set_u32(this.addr() + 0x68, process);
        e.call_log = Some(vec![]);
        (e, this)
    }

    /// Makes `addr`'s double copy the three words at its second argument to
    /// the scratch point `011a5000 + 12 * slot`.
    fn record_point(e: &mut Engine, addr: u32, slot: u32) {
        e.register_double(addr, move |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(0x011a_5000 + 12 * slot + 4 * i, word);
            }
            Ret::default()
        });
    }

    #[test]
    fn the_camera_update_in_first_person_follows_the_player_rotation() {
        let (mut e, this) = update_camera_world();
        e.call(0x0094_ae40, &args![this, false, false]);
        // The heading and pitch are remembered and make the two rotations.
        assert_eq!(e.global::<f32>(LAST_HEADING), 1.5);
        assert_eq!(e.global::<f32>(LAST_PITCH), 0.25);
        assert_eq!(called(&e, 0x004a_0c90)[0][1], 1.5f32.to_bits());
        assert_eq!(called(&e, 0x0052_4ac0)[0][1], 0.25f32.to_bits());
        // The field of view comes from fWorldFOV; the static camera node is built once.
        assert_eq!(
            called(&e, 0x00c5_2020),
            vec![vec![0x011a_4000, 75.0f32.to_bits(), 0, 0, 0]]
        );
        assert_eq!(called(&e, 0x00b5_4000), vec![vec![75.0f32.to_bits()]]);
        assert_eq!(called(&e, 0x00a7_12f0), vec![vec![CAMERA_NODE]]);
        assert_eq!(called(&e, 0x00ec_658f), vec![vec![0x00fd_9b90]]);
        // The fade factor and the caster hook.
        assert_eq!(e.global::<f32>(0x011a_3b64), 1.0);
        // The end of the frame: the first person zoom update.
        assert_eq!(called(&e, 0x0095_0290), vec![vec![this.addr()]]);
        // The rigid body is moved to the cached world camera position.
        let cached = member(this, PlayerCharacter::CachedWorldCameraPos);
        let body = called(&e, 0x00c8_e160);
        assert_eq!(body.len(), 1);
        assert_eq!(body[0][1], cached);
        assert_eq!(body[0][3], 0);
        // A second call does not rebuild the camera node.
        e.call(0x0094_ae40, &args![this, false, false]);
        assert_eq!(called(&e, 0x00a7_12f0).len(), 1);
    }

    #[test]
    fn the_camera_update_tells_the_caster_in_first_person_unless_in_vats() {
        let (mut e, this) = update_camera_world();
        let caster = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0x21c, caster);
        e.register(0x0062_1440, |_, _| int(1));
        e.call(0x0094_ae40, &args![this, false, false]);
        assert_eq!(called(&e, 0x0062_1370), vec![vec![caster, 0]]);
    }

    #[test]
    fn the_camera_update_in_the_vats_camera_looks_at_the_vats_point() {
        let (mut e, this) = update_camera_world();
        e.register(0x0044_ddc0, |_, _| int(4));
        e.set(this, PlayerCharacter::bWant3rdPerson, true);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(0x011e_0808 + 4 * i as u32, *v);
        }
        record_point(&mut e, 0x0044_0460, 0);
        e.register(0x009c_8ac0, |_, _| int(0x5555));
        e.call(0x0094_ae40, &args![this, false, false]);
        // The node is placed at the smoothed camera position and looks at the VATS point.
        assert_eq!(called(&e, 0x0044_0460)[0][0], CAMERA_NODE);
        assert_eq!(read_floats(&e, 0x011a_5000), [1.0, 2.0, 3.0]);
        assert_eq!(
            called(&e, 0x00a7_01b0),
            vec![vec![CAMERA_NODE, 0x5555, CAMERA_UP_VECTOR]]
        );
        // The rotation is read back from the node three times.
        assert_eq!(called(&e, 0x0047_6930).len(), 3);
        assert_eq!(called(&e, 0x0071_33b0).len(), 3);
    }

    #[test]
    fn the_camera_update_in_third_person_offsets_the_camera_and_stores_the_shoulder() {
        let (mut e, this) = update_camera_world();
        e.set(this, PlayerCharacter::b3rdPerson, true);
        e.set(this, PlayerCharacter::bWant3rdPerson, true);
        e.set_global(0x011c_dc44, 1.0f32); // the shoulder offset: up
        e.set_global(0x011c_dc5c, 2.0f32); // the shoulder offset: sideways
        e.set_global(CAMERA_DISTANCE, 1.0f32);
        e.set_global(0x011c_d568, 100.0f32);
        e.call(0x0094_ae40, &args![this, true, false]);
        // The field of view is set twice: the world one, then the third person one.
        let fov: Vec<u32> = called(&e, 0x00c5_2020).iter().map(|c| c[1]).collect();
        assert_eq!(fov, vec![75.0f32.to_bits(), 60.0f32.to_bits()]);
        // The shoulder offset (identity rotation): (side, 0, up) scaled by 1.
        assert_eq!(
            read_floats(
                &e,
                member(this, PlayerCharacter::kCamera3rdPersonShoulderOffset)
            ),
            [2.0, 0.0, 1.0]
        );
        // The distance code ran and the node looks at the look-at point.
        assert_eq!(called(&e, 0x0045_7910).len(), 1);
        let look = &called(&e, 0x00a7_01b0)[0];
        assert_eq!(look[0], CAMERA_NODE);
        assert_eq!(look[2], CAMERA_UP_VECTOR);
        // The listener and the terrain are refreshed with `snap`.
        assert_eq!(called(&e, 0x0057_d0a0).len(), 1);
    }

    #[test]
    fn the_camera_update_sets_the_listener_underwater_flag_from_the_submerge_level() {
        let (mut e, this) = update_camera_world();
        e.register(0x008d_6f30, |_, _| int(0x4000));
        e.register(0x0088_5560, |_, _| float(0.5));
        e.call(0x0094_ae40, &args![this, false, false]);
        assert_eq!(called(&e, 0x00ad_7990), vec![vec![0x4600, 1]]);
        e.register(0x0088_5560, |_, _| float(0.01));
        e.call(0x0094_ae40, &args![this, false, false]);
        assert_eq!(called(&e, 0x00ad_7990)[1], vec![0x4600, 0]);
        // The listener position and orientation are always set.
        assert_eq!(called(&e, 0x00ad_78b0).len(), 2);
        assert_eq!(called(&e, 0x00ad_7940).len(), 2);
    }

    #[test]
    fn the_camera_update_refreshes_the_terrain_unless_told_not_to() {
        let (mut e, this) = update_camera_world();
        e.register(0x0057_5d70, |_, _| int(0x7000));
        e.register(0x0058_6170, |_, _| int(0x7100));
        e.register(0x0097_4d90, |_, _| int(1));
        e.call(0x0094_ae40, &args![this, false, false]);
        assert_eq!(
            called(&e, 0x006f_ca90),
            vec![vec![0x7100, 0x011a_0300, 0xf]]
        );
        assert_eq!(called(&e, 0x006f_cdb0), vec![vec![0x7100, 0x011a_0300]]);
        // skip_terrain: no update at all.
        e.call_log = Some(vec![]);
        e.call(0x0094_ae40, &args![this, false, true]);
        assert!(called(&e, 0x006f_ca90).is_empty());
        assert!(called(&e, 0x006f_cdb0).is_empty());
    }

    #[test]
    fn the_camera_update_applies_the_camera_shake_through_euler_angles() {
        let (mut e, this) = update_camera_world();
        e.register(0x008d_1b30, |_, _| float(0.5));
        e.register(0x00a5_92c0, |e, a| {
            for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
                e.mem.set_f32(a[1 + i], *v);
            }
            Ret::default()
        });
        e.call(0x0094_ae40, &args![this, false, false]);
        // x, y and z angles halved.
        assert_eq!(
            called(&e, 0x00a5_9540)[0][1..],
            [0.5f32.to_bits(), 1.0f32.to_bits(), 1.5f32.to_bits()]
        );
    }

    #[test]
    fn the_typed_attack_group_maps_animation_types_and_falls_back_to_the_weapon() {
        let mut e = engine();
        e.register(0x0049_5e40, |_, _| int(0x35));
        assert_eq!(typed_attack_group(&mut e, 0xa00, 0), 0x32);
        for (kind, group) in [
            (0x33u32, 0x26u32),
            (0x34, 0x2c),
            (0x36, 0x38),
            (0x37, 0x3e),
            (0x38, 0x44),
            (0x6c, 0x1a),
        ] {
            e.register_double(0x0049_5e40, move |_, _| int(kind));
            assert_eq!(typed_attack_group(&mut e, 0xa00, 0), group);
        }
        // Another type: the weapon's own group when it has one, else 0x20.
        e.register(0x0049_5e40, |_, _| int(0x10));
        assert_eq!(typed_attack_group(&mut e, 0xa00, 0), 0x20);
        e.register(0x0051_f5f0, |_, _| int(0x41));
        assert_eq!(typed_attack_group(&mut e, 0xa00, 0x6000), 0x41);
        e.register(0x0051_f5f0, |_, _| int(0xff));
        assert_eq!(typed_attack_group(&mut e, 0xa00, 0x6000), 0x20);
    }

    #[test]
    fn a_queued_attack_picks_its_group_from_the_weapon_or_the_power_attack() {
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[]);
        // No attack animation: queued 1 gives the weapon group (0x20 without one),
        // queued 2 the unarmed power attack.
        e.set_global(ATTACK_QUEUED, 1u32);
        assert_eq!(queued_attack_group(&mut e, this, 0xa00, 0, 0xff), 0x20);
        e.register(0x0051_f5f0, |_, _| int(0x30));
        assert_eq!(queued_attack_group(&mut e, this, 0xa00, 0x6000, 0xff), 0x30);
        e.set_global(ATTACK_QUEUED, 2u32);
        assert_eq!(queued_attack_group(&mut e, this, 0xa00, 0x6000, 0xff), 0x5c);
        // A power attack underway leaves the group alone.
        e.register(0x005f_2540, |_, _| int(1));
        e.register(0x005f_2670, |_, _| int(1));
        assert_eq!(queued_attack_group(&mut e, this, 0xa00, 0x6000, 0xff), 0xff);
    }

    #[test]
    fn a_queued_attack_during_a_melee_attack_waits_for_the_swing_to_progress() {
        let mut e = engine();
        // Slot 0x1b0 (melee) and slot 0x1ac true.
        let this = attack_world(&mut e, 0x0265_0000, &[(0x1b0, 1), (0x1ac, 1)]);
        e.set_global(ATTACK_QUEUED, 1u32);
        e.register(0x005f_2540, |_, _| int(1));
        e.register(0x008a_7570, |_, _| int(5));
        e.register(0x0070_f490, |_, _| int(2));
        e.set(this, PlayerCharacter::fProjectileReleaseTimer, 4.0);
        // Early in the swing (step <= 2): nothing changes.
        assert_eq!(queued_attack_group(&mut e, this, 0xa00, 0, 0xff), 0xff);
        assert_eq!(e.get(this, PlayerCharacter::fProjectileReleaseTimer), 4.0);
        // Later in the swing the group is chosen and the release timer cleared.
        e.register(0x0070_f490, |_, _| int(3));
        assert_eq!(queued_attack_group(&mut e, this, 0xa00, 0, 0xff), 0x66);
        assert_eq!(e.get(this, PlayerCharacter::fProjectileReleaseTimer), 0.0);
    }

    #[test]
    fn the_next_attack_is_queued_unless_the_swing_still_has_time() {
        let mut e = engine();
        attack_world(&mut e, 0x0265_0000, &[]);
        // No weapon: queued at once.
        queue_next_attack(&mut e, 0xa00, 0);
        assert_eq!(e.global::<u32>(ATTACK_QUEUED), 1);
        // A weapon with ammunition and a swing with 3 s left of 4: not queued.
        e.set_global(ATTACK_QUEUED, 0u32);
        e.register(0x0052_5980, |_, _| int(0xa1));
        e.register(0x0059_8040, |_, _| float(1.0));
        e.register(0x0049_5e40, |e, a| {
            e.mem.set_f32(a[1], 4.0);
            Ret::default()
        });
        e.register(0x0049_1040, |_, _| int(0x0120_0000));
        e.mem.set_f32(0x0120_0000 + 0x30, 5.0);
        e.mem.set_f32(0x0120_0000 + 0x2c, 0.0);
        e.mem.set_f32(0x0120_0000 + 0x28, 1.0);
        // now (4.0) > start (1.0) and end - start = 5 - 1 >= 0.5: not queued.
        queue_next_attack(&mut e, 0xa00, 0x6000);
        assert_eq!(e.global::<u32>(ATTACK_QUEUED), 0);
        // Nearly at the start of the swing (now <= start): queued.
        e.register(0x0059_8040, |_, _| float(9.0));
        queue_next_attack(&mut e, 0xa00, 0x6000);
        assert_eq!(e.global::<u32>(ATTACK_QUEUED), 1);
    }

    #[test]
    fn the_ranged_reaction_picks_the_group_by_the_process_flag() {
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[(0x1ac, 1)]);
        let mut group = 0xffu32;
        ranged_attack_reaction(&mut e, this, 0, true, &mut group);
        // Slot 0x1ac: group 0x66 and the state 4, release timer 10 in VATS.
        assert_eq!(group, 0x66);
        assert_eq!(
            called(&e, 0x0096_3eb0),
            vec![vec![this.addr(), 4, 3.0f32.to_bits(), 0]]
        );
        assert_eq!(e.get(this, PlayerCharacter::fProjectileReleaseTimer), 10.0);
        // No flag: the default group 0x20 and no reaction call.
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[]);
        let mut group = 0xffu32;
        ranged_attack_reaction(&mut e, this, 0, false, &mut group);
        assert_eq!(group, 0x20);
        assert!(called(&e, 0x0096_3eb0).is_empty());
    }

    #[test]
    fn holding_attack_with_a_charged_timer_starts_the_unarmed_power_attack() {
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[]);
        zero_table(&mut e, 0x0270_0000, &[]);
        e.mem.set_u32(this.addr(), 0x0270_0000);
        e.set_global(0x011c_dda8, 0.5f32); // the charge time
        let mut ready = false;
        let mut group = 0xffu32;
        held_attack(
            &mut e, this, 0xa00, 0, 0, false, 1.0, &mut ready, &mut group,
        );
        // Without a weapon the unarmed state (1.5) is told and the timer runs.
        assert_eq!(
            called(&e, 0x0096_3eb0),
            vec![vec![this.addr(), 1, 1.5f32.to_bits(), 0]]
        );
        // The timer passed the charge time: the unarmed power attack, timer cleared.
        assert_eq!(group, 0x5c);
        assert_eq!(e.global::<f32>(ATTACK_HOLD_TIMER), 0.0);
        // A shorter hold only advances the timer.
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[]);
        zero_table(&mut e, 0x0270_0000, &[]);
        e.mem.set_u32(this.addr(), 0x0270_0000);
        e.set_global(0x011c_dda8, 5.0f32);
        let mut group = 0xffu32;
        held_attack(
            &mut e, this, 0xa00, 0, 0, false, 1.0, &mut ready, &mut group,
        );
        assert_eq!(group, 0xff);
        assert_eq!(e.global::<f32>(ATTACK_HOLD_TIMER), 1.0);
    }

    #[test]
    fn releasing_the_draw_button_for_the_rockit_launcher_queues_a_menu() {
        let mut e = engine();
        let this = attack_world(
            &mut e,
            0x0265_0000,
            &[(0x454, 1), (0x148, 0x5100), (0x6e0, 0)],
        );
        e.register_double(0x00a2_4660, |_, a| int((a[1] == 7 && a[2] == 2) as u32));
        e.set_global(DRAW_BUTTON_TIMER, 0.3f32);
        e.set_global(0x011c_dfcc, 0.5f32);
        e.register(0x008a_7570, |_, _| int(0xffff_ffff));
        e.register(0x0047_4a80, |_, _| int(1));
        let mut ready = true;
        released_attack(&mut e, this, 0x1000, 0xa00, 0x6000, &mut ready);
        assert_eq!(called(&e, 0x0047_4a80), vec![vec![0x6000 + 0xa4]]);
        assert_eq!(called(&e, 0x0070_9470), vec![vec![1, 0, 0, 0, 4, 0]]);
        assert!(!ready);
    }

    #[test]
    fn releasing_the_attack_button_ends_the_attack_animation_and_sets_the_release_byte() {
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[(0x6e0, 0)]);
        e.register(0x005f_2540, |_, _| int(1));
        e.register(0x005f_25d0, |_, _| int(1));
        e.register(0x0070_f490, |_, _| int(0));
        e.register_double(0x00a2_4660, |_, a| int((a[1] == 4 && a[2] == 2) as u32));
        let mut ready = true;
        released_attack(&mut e, this, 0x1000, 0xa00, 0, &mut ready);
        assert!(!ready);
        assert_eq!(e.mem.u8(ATTACK_RELEASE_BYTE), 1);
    }

    #[test]
    fn the_power_attack_group_follows_the_movement_direction_and_the_perks() {
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[]);
        player_slots(&mut e, this, 5.0);
        e.register(0x0088_46e0, |_, _| int(2)); // moving back: group 0x5e
        assert_eq!(power_attack_group(&mut e, this, 0, false), 0x5e);
        // Bound weapon with a positive entry point: the perk variant.
        e.register(0x005e_58f0, |e, a| {
            e.mem.set_f32(a[2], 1.0);
            Ret::default()
        });
        assert_eq!(power_attack_group(&mut e, this, 0, true), 0x62);
        // Without the actor values the entry point 0x3F can still replace 0x5C.
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[]);
        player_slots(&mut e, this, 0.0);
        assert_eq!(power_attack_group(&mut e, this, 0, true), 0x5c);
        e.register(0x005e_58f0, |e, a| {
            e.mem.set_f32(a[2], 1.0);
            Ret::default()
        });
        assert_eq!(power_attack_group(&mut e, this, 0, true), 0x63);
        assert_eq!(power_attack_group(&mut e, this, 0, false), 0x5c);
    }

    #[test]
    fn the_attack_animation_sync_skips_updates_unless_the_loop_is_running() {
        let mut e = engine();
        let this = attack_world(&mut e, 0x0265_0000, &[]);
        zero_table(&mut e, 0x0270_0000, &[]);
        e.mem.set_u32(this.addr(), 0x0270_0000);
        e.register(0x005f_2540, |_, _| int(1));
        e.register(0x0049_1040, |_, _| int(0x5500));
        e.register(0x0049_3800, |_, _| float(0.0));
        sync_attack_animation(&mut e, this, 0xa00, 0xa01, 0, false);
        assert_eq!(
            called(&e, 0x008e_eaa0),
            vec![vec![0xa00, 4], vec![0xa01, 4]]
        );
        // A running loop with the generic location 1 does not skip.
        e.call_log = Some(vec![]);
        e.register(0x0049_3800, |_, _| float(1.0));
        e.register(0x0080_41a0, |_, _| int(1));
        sync_attack_animation(&mut e, this, 0xa00, 0xa01, 0, false);
        assert!(called(&e, 0x008e_eaa0).is_empty());
        // With VATS on nothing happens.
        e.register(0x0049_3800, |_, _| float(0.0));
        sync_attack_animation(&mut e, this, 0xa00, 0xa01, 0, true);
        assert!(called(&e, 0x008e_eaa0).is_empty());
    }

    #[test]
    fn the_look_center_of_a_target_averages_its_body_part_nodes() {
        let mut e = engine();
        exe_constants(&mut e);
        vector_math(&mut e);
        stub(
            &mut e,
            &[
                0x0041_81e0,
                0x0043_b230,
                0x0044_ddc0,
                0x0045_c670,
                0x004a_ae30,
                0x005e_50f0,
            ],
        );
        e.register(0x0063_c8a0, |e, a| {
            for i in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, sum);
            }
            int(a[0])
        });
        e.register(0x0053_d280, |e, a| {
            let count = f32::from_bits(a[2]);
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) / count;
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            int(a[1])
        });
        // The actor's body part data: slot 0x180 of the owner object; the part
        // table has parts 1 and 2 only. Each node sits at (index, 0, 0).
        let owner = zero_object(&mut e, 0x0277_0000, &[(0x180, 0x6100)]);
        e.register_double(0x0041_81e0, move |_, _| int(owner));
        let table = e.mem.alloc(64);
        e.mem.set_u32(table + 4, 1);
        e.mem.set_u32(table + 8, 1);
        e.register_double(0x006a_9540, move |_, _| int(table));
        let nodes = [e.mem.alloc(0xa0), e.mem.alloc(0xa0), e.mem.alloc(0xa0)];
        e.mem.set_f32(nodes[1] + 0x8c, 1.0);
        e.mem.set_f32(nodes[2] + 0x8c, 3.0);
        e.register(0x005e_50f0, |_, a| int(a[1]));
        e.register(0x0043_b230, |_, a| int(a[0]));
        e.register_double(0x004a_ae30, move |_, a| int(nodes[a[1] as usize]));
        e.register(0x0045_bb80, |_, a| int(a[0] + 0x8c));
        // The torso (part 1) is far away: all parts are averaged.
        let player_object = zero_object(&mut e, 0x0278_0000, &[(0x1d0, 0x0120_1000)]);
        e.set_global(PLAYER_SINGLETON, player_object);
        e.register(0x0045_7990, |_, _| float(1000.0));
        let actor = zero_object(&mut e, 0x0279_0000, &[(0x1d0, 0x7000)]);
        rebuild_look_center(&mut e, actor);
        // Parts 1 and 2: (1 + 3) / 2 = 2 on x.
        assert_eq!(read_floats(&e, LOOK_CENTER), [2.0, 0.0, 0.0]);
    }

    /// A double that answers 1, put in a vtable slot by the order test.
    const ANSWERS_ONE: u32 = 0x00b1_0000;

    /// `Update` in [`update_world`] with the inputs of
    /// `world::frame::player::PlayerState` set through its globals, fields
    /// and doubles: the calls it makes, in order.
    fn update_calls(s: &world::frame::player::PlayerState) -> Vec<u32> {
        let mut e = engine();
        let this = update_world(&mut e);
        e.register(ANSWERS_ONE, |_, _| int(1));
        let answer = |e: &mut Engine, table: u32, slot: u32| {
            e.mem.set_u32(table + slot, ANSWERS_ONE);
        };
        e.set_global::<u8>(VATS_ENDED_FLAG, s.vats_ended.into());
        if s.forced_activation {
            e.register(0x0094_4320, |_, _| int(0x4455));
        }
        e.set_global::<u8>(STATE_FLAG_011E07B8, s.fade_test_skipped.into());
        if s.fading {
            e.register(0x008f_ec10, |_, _| int(1));
        }
        if s.knocked_or_paralysed {
            answer(&mut e, 0x0201_0000, 0x234);
        }
        e.set(this, PlayerCharacter::b3rdPerson, s.third_person);
        e.set(this, PlayerCharacter::bAiControlledFromPos, s.ai_controlled);
        if s.dead {
            answer(&mut e, 0x0201_0000, 0x22c);
        }
        if s.control_time_out {
            e.set_global(CONTROL_COUNTER_011E0798, 1.0f32);
        }
        if s.in_dialogue {
            e.register(0x0070_50d0, |_, _| int(1));
        }
        if s.vats_ending {
            // The flag set again just before the camera's test (`0094381c`).
            e.register(0x008b_a600, |e, _| {
                e.set_global::<u8>(VATS_ENDED_FLAG, 1);
                Ret::default()
            });
        }
        if s.muzzle_flash {
            answer(&mut e, 0x0202_0000, 0x6b8);
        }
        e.call_log = Some(vec![]);
        player_character_update(&mut e, this, 0.5);
        call_addresses(&e)
    }

    /// The calls `Update` makes follow `world::frame::player::UPDATE` for
    /// each of its gates: every step the gates let through is called, in
    /// order, except where its own tests (not modelled) keep it from
    /// calling, and nothing else of the list is (docs/FRAME_SKELETON.md,
    /// PR 4).
    #[test]
    fn update_follows_the_frame_model() {
        use world::frame::player::{steps_run, PlayerState, UPDATE};
        let game = PlayerState::default();
        let ai = PlayerState {
            ai_controlled: true,
            ..game
        };
        let knocked = PlayerState {
            knocked_or_paralysed: true,
            ..game
        };
        let states = [
            game,
            PlayerState {
                vats_ended: true,
                ..game
            },
            PlayerState {
                forced_activation: true,
                ..game
            },
            PlayerState {
                fading: true,
                ..game
            },
            PlayerState {
                fading: true,
                fade_test_skipped: true,
                ..game
            },
            knocked,
            PlayerState {
                third_person: true,
                ..knocked
            },
            ai,
            PlayerState {
                control_time_out: true,
                ..ai
            },
            PlayerState { dead: true, ..game },
            PlayerState {
                in_dialogue: true,
                ..game
            },
            PlayerState {
                vats_ending: true,
                ..game
            },
            PlayerState {
                muzzle_flash: true,
                ..game
            },
        ];
        let listed: Vec<u32> = UPDATE.iter().map(|s| s.address).collect();
        for s in states {
            let calls: Vec<u32> = update_calls(&s)
                .into_iter()
                .filter(|a| listed.contains(a))
                .collect();
            let mut next = calls.iter().peekable();
            for i in steps_run(&s) {
                let step = &UPDATE[i];
                if next.peek() == Some(&&step.address) {
                    next.next();
                } else {
                    assert!(
                        !step.own_tests.is_empty(),
                        "{s:?}: step {i} ({:08x} at {:08x}) not called; then {:08x?}",
                        step.address,
                        step.sites[0],
                        next.clone().collect::<Vec<_>>()
                    );
                }
            }
            assert_eq!(next.next(), None, "{s:?}: called beyond the model");
        }
    }

    /// Steps with own tests do call once those pass: the hardcore update
    /// (`004d1360`, `0093f36f`) and the return to the last good position
    /// (`0093f619`).
    #[test]
    fn update_steps_with_own_tests_call_when_they_pass() {
        let mut e = engine();
        let this = update_world(&mut e);
        e.register(0x004d_1360, |_, _| int(1));
        e.set(this, PlayerCharacter::bReturnToLastKnownGoodPosition, true);
        player_character_update(&mut e, this, 0.5);
        assert_eq!(called(&e, 0x0096_9c30).len(), 1);
        assert_eq!(called(&e, 0x0094_dbe0), vec![vec![this.addr(), 1]]);
    }
}
