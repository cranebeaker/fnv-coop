// Operation ids of the generic RefCommand effect (story-capture, docs/tools/story-capture.md).
//
// One RefCommand = one script command that acts on a reference (or on nothing) and has no Effect type of its own: AI orders, spells, equipment,
// ownership, chargen / progression calls, journal-ish calls. The op id is on the wire and in saved sidecars, so ids are append-only: never
// renumber, never reuse. The engine-facing table (command names, argument layout, who may apply it) lives in plugin/hooks/ref_ops.h; this
// header holds only what the shared state machine needs: whether an op is an EVENT (fire and forget) or STATE (the last value per key is
// kept in the ShadowState and sent to late joiners), and which slots form the key.
#pragma once
#include <cstdint>

#include "fnvcoop/types.h"

namespace fnvcoop {

enum RefOpKind : uint8_t {
    kRefOpEvent = 0,   // nothing is remembered; every machine's sink sees it once, in host order
    kRefOpState = 1,   // the last command per key is remembered (baseline, late join, rebase)
};
// Which slots of the command identify a state entry.
constexpr uint8_t kKeyRef = 1, kKeyA = 2, kKeyB = 4;
// Who the capture routes the command to when the script did not say: 0 host policy (Default), 1 the acting player only, 2 everyone.
constexpr uint8_t kRcpDefault = 0, kRcpOrigin = 1, kRcpAll = 2;

struct RefOpInfo {
    uint16_t id;
    const char* name;     // the engine command (long name), as shown by the console
    uint8_t kind;         // RefOpKind
    uint16_t group;       // state entries of ops with the same group replace each other (AddSpell / RemoveSpell); = id for a lone op
    uint8_t key;          // kKey* bits: the slots that identify the entry inside its group
    uint8_t recipient;    // kRcp*
};

// Ids are append-only. Order inside a family is only for reading.
enum RefOp : uint16_t {
    kOpNone = 0,
    // actor AI orders (events, applied by the machine that owns the actor)
    kOpEvaluatePackage = 1, kOpStartCombat = 2, kOpStopCombat = 3, kOpResetAI = 4, kOpStopLook = 5, kOpLook = 6, kOpSetAlert = 7,
    kOpStopCombatAlarmOnActor = 8, kOpForceFlee = 9, kOpPushActorAway = 10, kOpResetHealth = 11,
    // actor AI flags (state)
    kOpSetUnconscious = 12, kOpSetGhost = 13, kOpSetIgnoreFriendlyHits = 14, kOpSetCombatStyle = 15, kOpSetForceSneak = 16,
    kOpSetPackDuration = 17, kOpSetActorsAI = 18, kOpPickIdle = 19, kOpSetVATSTarget = 20, kOpFireWeapon = 21, kOpSetDisposition = 22,
    // spells
    kOpAddSpell = 23, kOpRemoveSpell = 24, kOpDispel = 25, kOpDispelAllSpells = 26, kOpCastImmediateOnSelf = 27,
    kOpPlayMagicShaderVisuals = 28, kOpStopMagicShaderVisuals = 29,
    // equipment and inventory
    kOpEquipItem = 30, kOpUnequipItem = 31, kOpResetInventory = 32, kOpDropMe = 33, kOpDrop = 34, kOpRemoveMe = 35,
    kOpAddItemToLeveledList = 36, kOpAddFormToFormList = 37,
    // world state (SetOwnership / ClearOwnership / SetCellOwnership / SetCellPublicFlag are the world-time lane's: plugin/hooks_world/world_owner_addrs.h)
    kOpSetActorAlpha = 38, kOpSetScale = 39,
    kOpSetItemValue = 40, kOpClearDestruction = 41, kOpRotate = 42,
    // quest meta
    kOpSetQuestDelay = 43, kOpResetQuest = 44, kOpSetQuestObject = 45, kOpAutoDisplayObjectives = 46,
    kOpActivate = 47,
    // character build and progression (the acting player)
    kOpSetPlayerTagSkill = 48, kOpAddSPECIALPoints = 49, kOpSetSPECIALPoints = 50, kOpSetTagSkills = 51, kOpAddTagSkills = 52, kOpSetLevel = 53,
    kOpResetXP = 54, kOpAgeRace = 55, kOpSexChange = 56, kOpSetPCCanUsePowerArmor = 57, kOpSetPCSleepHours = 58, kOpWakeUpPC = 59,
    kOpSetHardcore = 60, kOpSetInChargen = 61, kOpOpenTeammateContainer = 62,
    // challenges and achievements (everyone)
    kOpAddAchievement = 63, kOpIncrementScriptedChallenge = 64, kOpUnlockChallenge = 65, kOpRemoveRecurringFromChallenge = 66,
    kOpAddCardToPlayer = 67,
    // scene helpers
    kOpSetTalkingActivatorActor = 68, kOpAlwaysShowActorSubtitles = 69, kOpStartRadioConversation = 70, kOpSetCellImageSpace = 71,
    // inventory with a condition (the Effect types AddItem / RemoveItem have no health field)
    kOpAddItemHealthPercent = 72,
    // animation and faction deltas
    kOpPlayGroup = 73, kOpModFactionRank = 74, kOpPlayIdle = 75, kOpSetCriticalStage = 76, kOpSetActorFullName = 77,
    // the second shape: more parameters than a RefCommand's a / b / i..l slots (refOpIsWide)
    kOpUseWeapon = 78,
    // full-screen videos (Fallout 3's ending, TTW's train): every player watches
    kOpPlayBink = 79,
    kOpCount_
};

#define FNV_RO_E(id, name, rcp) {id, name, kRefOpEvent, id, 0, rcp}
#define FNV_RO_S(id, name, key, rcp) {id, name, kRefOpState, id, key, rcp}
inline constexpr RefOpInfo kRefOps[] = {
    {kOpNone, "None", kRefOpEvent, kOpNone, 0, kRcpDefault},
    FNV_RO_E(kOpEvaluatePackage, "EvaluatePackage", kRcpDefault),
    FNV_RO_E(kOpStartCombat, "StartCombat", kRcpDefault),
    FNV_RO_E(kOpStopCombat, "StopCombat", kRcpDefault),
    FNV_RO_E(kOpResetAI, "ResetAI", kRcpDefault),
    FNV_RO_E(kOpStopLook, "StopLook", kRcpDefault),
    FNV_RO_E(kOpLook, "Look", kRcpDefault),
    FNV_RO_E(kOpSetAlert, "SetAlert", kRcpDefault),
    FNV_RO_E(kOpStopCombatAlarmOnActor, "StopCombatAlarmOnActor", kRcpDefault),
    FNV_RO_E(kOpForceFlee, "ForceFlee", kRcpDefault),
    FNV_RO_E(kOpPushActorAway, "PushActorAway", kRcpDefault),
    FNV_RO_E(kOpResetHealth, "ResetHealth", kRcpDefault),
    FNV_RO_S(kOpSetUnconscious, "SetUnconscious", kKeyRef, kRcpDefault),
    FNV_RO_S(kOpSetGhost, "SetGhost", kKeyRef, kRcpDefault),
    FNV_RO_S(kOpSetIgnoreFriendlyHits, "SetIgnoreFriendlyHits", kKeyRef, kRcpDefault),
    FNV_RO_S(kOpSetCombatStyle, "SetCombatStyle", kKeyRef, kRcpDefault),
    FNV_RO_S(kOpSetForceSneak, "SetForceSneak", kKeyRef, kRcpDefault),
    FNV_RO_E(kOpSetPackDuration, "SetPackDuration", kRcpDefault),
    FNV_RO_S(kOpSetActorsAI, "SetActorsAI", kKeyRef, kRcpDefault),
    FNV_RO_E(kOpPickIdle, "PickIdle", kRcpDefault),
    FNV_RO_E(kOpSetVATSTarget, "SetVATSTarget", kRcpDefault),
    FNV_RO_E(kOpFireWeapon, "FireWeapon", kRcpDefault),
    FNV_RO_S(kOpSetDisposition, "SetDisposition", kKeyRef | kKeyA, kRcpDefault),
    FNV_RO_S(kOpAddSpell, "AddSpell", kKeyRef | kKeyA, kRcpDefault),
    {kOpRemoveSpell, "RemoveSpell", kRefOpState, kOpAddSpell, kKeyRef | kKeyA, kRcpDefault},   // same group: the later of add / remove per (ref, spell) wins
    FNV_RO_E(kOpDispel, "Dispel", kRcpDefault),
    FNV_RO_E(kOpDispelAllSpells, "DispelAllSpells", kRcpDefault),
    FNV_RO_E(kOpCastImmediateOnSelf, "CastImmediateOnSelf", kRcpDefault),
    FNV_RO_E(kOpPlayMagicShaderVisuals, "PlayMagicShaderVisuals", kRcpDefault),
    FNV_RO_E(kOpStopMagicShaderVisuals, "StopMagicShaderVisuals", kRcpDefault),
    FNV_RO_E(kOpEquipItem, "EquipItem", kRcpDefault),
    FNV_RO_E(kOpUnequipItem, "UnequipItem", kRcpDefault),
    FNV_RO_E(kOpResetInventory, "ResetInventory", kRcpDefault),
    FNV_RO_E(kOpDropMe, "DropMe", kRcpDefault),
    FNV_RO_E(kOpDrop, "Drop", kRcpDefault),
    FNV_RO_E(kOpRemoveMe, "RemoveMe", kRcpDefault),
    FNV_RO_S(kOpAddItemToLeveledList, "AddItemToLeveledList", kKeyA | kKeyB, kRcpDefault),
    FNV_RO_S(kOpAddFormToFormList, "AddFormToFormList", kKeyA | kKeyB, kRcpDefault),
    FNV_RO_S(kOpSetActorAlpha, "SetActorAlpha", kKeyRef, kRcpDefault),
    FNV_RO_S(kOpSetScale, "SetScale", kKeyRef, kRcpDefault),
    FNV_RO_S(kOpSetItemValue, "SetItemValue", kKeyRef, kRcpDefault),
    FNV_RO_E(kOpClearDestruction, "ClearDestruction", kRcpDefault),
    FNV_RO_S(kOpRotate, "Rotate", kKeyRef, kRcpDefault),
    FNV_RO_S(kOpSetQuestDelay, "SetQuestDelay", kKeyA, kRcpDefault),
    FNV_RO_E(kOpResetQuest, "ResetQuest", kRcpDefault),
    FNV_RO_S(kOpSetQuestObject, "SetQuestObject", kKeyA, kRcpDefault),
    FNV_RO_S(kOpAutoDisplayObjectives, "AutoDisplayObjectives", 0, kRcpDefault),
    FNV_RO_E(kOpActivate, "Activate", kRcpDefault),
    FNV_RO_E(kOpSetPlayerTagSkill, "SetPlayerTagSkill", kRcpOrigin),
    FNV_RO_E(kOpAddSPECIALPoints, "AddSPECIALPoints", kRcpOrigin),
    FNV_RO_E(kOpSetSPECIALPoints, "SetSPECIALPoints", kRcpOrigin),
    FNV_RO_E(kOpSetTagSkills, "SetTagSkills", kRcpOrigin),
    FNV_RO_E(kOpAddTagSkills, "AddTagSkills", kRcpOrigin),
    FNV_RO_E(kOpSetLevel, "SetLevel", kRcpOrigin),
    FNV_RO_E(kOpResetXP, "ResetXP", kRcpOrigin),
    FNV_RO_E(kOpAgeRace, "AgeRace", kRcpOrigin),
    FNV_RO_E(kOpSexChange, "SexChange", kRcpOrigin),
    FNV_RO_S(kOpSetPCCanUsePowerArmor, "SetPCCanUsePowerArmor", 0, kRcpAll),
    FNV_RO_E(kOpSetPCSleepHours, "SetPCSleepHours", kRcpOrigin),
    FNV_RO_E(kOpWakeUpPC, "WakeUpPC", kRcpOrigin),
    FNV_RO_E(kOpSetHardcore, "SetHardcore", kRcpOrigin),
    FNV_RO_E(kOpSetInChargen, "SetInChargen", kRcpOrigin),
    FNV_RO_E(kOpOpenTeammateContainer, "OpenTeammateContainer", kRcpOrigin),
    FNV_RO_E(kOpAddAchievement, "AddAchievement", kRcpAll),
    FNV_RO_E(kOpIncrementScriptedChallenge, "IncrementScriptedChallenge", kRcpAll),
    FNV_RO_E(kOpUnlockChallenge, "UnlockChallenge", kRcpAll),
    FNV_RO_E(kOpRemoveRecurringFromChallenge, "RemoveRecurringFromChallenge", kRcpAll),
    FNV_RO_E(kOpAddCardToPlayer, "AddCardToPlayer", kRcpOrigin),
    FNV_RO_E(kOpSetTalkingActivatorActor, "SetTalkingActivatorActor", kRcpDefault),
    FNV_RO_S(kOpAlwaysShowActorSubtitles, "AlwaysShowActorSubtitles", 0, kRcpDefault),
    FNV_RO_E(kOpStartRadioConversation, "StartRadioConversation", kRcpDefault),
    FNV_RO_S(kOpSetCellImageSpace, "SetCellImageSpace", kKeyA, kRcpDefault),
    FNV_RO_E(kOpAddItemHealthPercent, "AddItemHealthPercent", kRcpDefault),
    FNV_RO_E(kOpPlayGroup, "PlayGroup", kRcpDefault),
    FNV_RO_E(kOpModFactionRank, "ModFactionRank", kRcpDefault),
    FNV_RO_E(kOpPlayIdle, "PlayIdle", kRcpDefault),
    FNV_RO_E(kOpSetCriticalStage, "SetCriticalStage", kRcpDefault),
    FNV_RO_S(kOpSetActorFullName, "SetActorFullName", kKeyRef, kRcpDefault),
    FNV_RO_E(kOpUseWeapon, "UseWeapon", kRcpDefault),
    FNV_RO_E(kOpPlayBink, "PlayBink", kRcpAll),
};
#undef FNV_RO_E
#undef FNV_RO_S
static_assert(sizeof(kRefOps) / sizeof(kRefOps[0]) == kOpCount_, "kRefOps must list every RefOp, in id order");

// nullptr for an unknown id (a decoder rejects it).
inline const RefOpInfo* refOpInfo(uint16_t id) { return id < kOpCount_ && kRefOps[id].id == id ? &kRefOps[id] : nullptr; }
inline bool refOpKnown(uint16_t id) { return id != kOpNone && refOpInfo(id) != nullptr; }
// Wide ops carry the forms c / d and the integer m of RefCommand as well (UseWeapon: weapon, shooter, target, + optional fourth form; five integers).
inline bool refOpIsWide(uint16_t id) { return id == kOpUseWeapon; }
inline bool refOpIsState(uint16_t id) { const RefOpInfo* i = refOpInfo(id); return i && i->kind == kRefOpState; }

}  // namespace fnvcoop
