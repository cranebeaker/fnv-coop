// Engine facts the real puppet backend relies on. Everything here is either
//   * a function address from docs/engine-addresses.md / plugin/hooks/engine_addrs.h / plugin/bots/bots_addrs.h /
//     plugin/hooks/tools/re/command_table.json (the eval column), with the first 16 prologue bytes pinned, or
//   * a data address / struct offset taken from the same documents or from the xNVSE headers in third_party/NVSE.
// No address is invented here. Unknown facts are marked TODO-RE in plugin/avatar/engine/README section of docs/tools/avatar.md and
// the code falls back to a console line run on a calling reference (the way plugin/main.cpp RunLineSEH does).
//
// tools/avatar/verify_engine_pins.py re-reads <reference exe> (read only) and proves every row below byte for byte.
// At run time the Windows backend compares the live process memory with the same bytes before it calls anything; a mismatch
// disables that one function (the engine then reports "unknown" for what it served) and is shown by avatar.status.
#pragma once
#include <cstddef>
#include <cstdint>

namespace nvcoop::avatar::pins {

struct Pin {
    const char* name;
    uint32_t addr;
    uint8_t bytes[16];
    const char* source;   // where the address comes from
};

// cdecl TESForm* LookupFormByID(UInt32 refID). Same pin as plugin/hooks/engine_addrs.h LookupFormByID.
inline constexpr Pin LookupFormByID = {"LookupFormByID", 0x4839C0u,
    {0x55, 0x8b, 0xec, 0x51, 0xc7, 0x45, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x83, 0x3d, 0xc0, 0x54, 0x1c}, "engine-addresses.md section 2"};

// cdecl bool MenuMode(): the script condition `MenuMode` without an argument, 0x702360 = (InterfaceManager exists) && byte[mgr+0]
// && dword[mgr+0xC] != 1 (docs/re/dialogmenu-menus-vats-inventory.md "Menus"). True while any menu that pauses or overlays the
// world is up (Pip-Boy, dialogue, container, loading screen, ...). Bytes 4..7 are a call displacement (kept: it makes the pin unique).
inline constexpr Pin MenuModeAny = {"MenuModeAny", 0x702360u,
    {0x55, 0x8b, 0xec, 0xe8, 0xa8, 0x4e, 0xdb, 0xff, 0x85, 0xc0, 0x74, 0x21, 0xe8, 0x9f, 0x4e, 0xdb}, "docs/re/dialogmenu-menus-vats-inventory.md"};

// Condition evaluators: cdecl bool eval(TESObjectREFR* thisObj, void* p1, void* p2, double* result); the 4th column of the
// command table (plugin/hooks/tools/re/command_table.json). Same calling pattern as nvworld::queryCondition (plugin/hooks_world).
// Each one first stores 0.0 into *result and returns true; for a non-actor `thisObj` the result stays 0.
inline constexpr Pin EvalGetRestrained = {"GetRestrained", 0x59FF90u,
    {0x55, 0x8b, 0xec, 0x51, 0x8b, 0x45, 0x14, 0xd9, 0xee, 0xdd, 0x18, 0xc7, 0x45, 0xfc, 0x00, 0x00}, "command_table.json"};
inline constexpr Pin EvalIsWeaponOut = {"IsWeaponOut", 0x5A0550u,
    {0x55, 0x8b, 0xec, 0x51, 0x8b, 0x45, 0x14, 0xd9, 0xee, 0xdd, 0x18, 0xc7, 0x45, 0xfc, 0x00, 0x00}, "command_table.json"};
inline constexpr Pin EvalIsSneaking = {"IsSneaking", 0x5A2F60u,
    {0x55, 0x8b, 0xec, 0x51, 0x8b, 0x45, 0x14, 0xd9, 0xee, 0xdd, 0x18, 0xc7, 0x45, 0xfc, 0x00, 0x00}, "command_table.json"};
inline constexpr Pin EvalGetDead = {"GetDead", 0x59D840u,
    {0x55, 0x8b, 0xec, 0x51, 0x8b, 0x45, 0x14, 0xd9, 0xee, 0xdd, 0x18, 0xc7, 0x45, 0xfc, 0x00, 0x00}, "command_table.json"};
// p1 = TESForm* (a form id is resolved with LookupFormByID first). Proven in docs/re ("GetItemCount condition 0x59D8E0").
inline constexpr Pin EvalGetItemCount = {"GetItemCount", 0x59D8E0u,
    {0x55, 0x8b, 0xec, 0x83, 0xec, 0x28, 0x8b, 0x45, 0x14, 0xd9, 0xee, 0xdd, 0x18, 0x8b, 0x4d, 0x08}, "command_table.json + docs/re inventory"};
// p1 = TESForm*; result 1 when the actor wears the item (disassembled: vtbl+0x100 IsActor check, GetContainerData 0x4BF220, 0x4BFDA0 IsEquipped).
inline constexpr Pin EvalGetEquipped = {"GetEquipped", 0x59DA90u,
    {0x55, 0x8b, 0xec, 0x83, 0xec, 0x18, 0x8b, 0x45, 0x14, 0xd9, 0xee, 0xdd, 0x18, 0xc7, 0x45, 0xf8}, "command_table.json"};

// W3a item 3 (facts of the local player). Same bytes and addresses as plugin/hooks/engine_addrs.h Cond_GetLevel / Cond_GetSitting / Cond_IsPlayerActionActive
// (conf V, docs/engine-addresses.md section 12). GetLevel and GetSitting read thisObj; IsPlayerActionActive ignores it (the global player) and takes an integer in p1.
inline constexpr Pin EvalGetLevel = {"GetLevel", 0x5A00C0u,
    {0x55, 0x8b, 0xec, 0x83, 0xec, 0x08, 0x8b, 0x45, 0x14, 0xd9, 0xee, 0xdd, 0x18, 0xc7, 0x45, 0xfc}, "plugin/hooks/engine_addrs.h"};
inline constexpr Pin EvalGetSitting = {"GetSitting", 0x59DD90u,
    {0x55, 0x8b, 0xec, 0x83, 0xec, 0x08, 0x8b, 0x45, 0x14, 0xd9, 0xee, 0xdd, 0x18, 0xc7, 0x45, 0xfc}, "plugin/hooks/engine_addrs.h"};
inline constexpr Pin EvalIsPlayerActionActive = {"IsPlayerActionActive", 0x5A3B20u,
    {0x55, 0x8b, 0xec, 0x83, 0xec, 0x08, 0x8b, 0x45, 0x14, 0xd9, 0xee, 0xdd, 0x18, 0x8b, 0x4d, 0x0c}, "plugin/hooks/engine_addrs.h"};

// Guest pack facts: GetActorValue (InventoryWeight for the Jed gate). Same address and bytes as plugin/hooks/engine_addrs.h Cond_GetActorValue (conf V); p1 is the actor value code itself.
inline constexpr Pin EvalGetActorValue = {"GetActorValue", 0x59C4F0u,
    {0x55, 0x8b, 0xec, 0x83, 0xec, 0x0c, 0x8b, 0x45, 0x0c, 0x89, 0x45, 0xfc, 0xc7, 0x45, 0xf8, 0x00}, "plugin/hooks/engine_addrs.h"};

// thiscall bool TESObjectREFR::IsDisabled(): ExtraData 0x92 flags & 8 (plugin/hooks/engine_addrs.h RefIsDisabled, conf M: semantics inferred).
inline constexpr Pin RefIsDisabled = {"TESObjectREFR::IsDisabled", 0x565090u,
    {0x55, 0x8b, 0xec, 0x83, 0xec, 0x0c, 0x89, 0x4d, 0xf4, 0x68, 0x92, 0x00, 0x00, 0x00, 0x8b, 0x4d}, "plugin/hooks/engine_addrs.h"};

inline constexpr const Pin* kAllPins[] = {&LookupFormByID, &MenuModeAny, &EvalGetRestrained, &EvalIsWeaponOut,
                                          &EvalIsSneaking, &EvalGetDead, &EvalGetItemCount, &EvalGetEquipped, &RefIsDisabled,
                                          &EvalGetLevel, &EvalGetSitting, &EvalIsPlayerActionActive, &EvalGetActorValue};
inline constexpr size_t kPinCount = sizeof(kAllPins) / sizeof(kAllPins[0]);

// ---- data addresses (docs/engine-addresses.md section 3) ----
inline constexpr uint32_t kThePlayerPtr = 0x11DEA3Cu;   // PlayerCharacter** (refID 0x14)

// ---- object layouts (xNVSE GameObjects.h / GameForms.h, STATIC_ASSERTs there; animprobe verified the process/mover reads) ----
inline constexpr uint32_t kPlayerRefId = 0x14;
inline constexpr uint32_t kRefFlags = 0x08;             // TESForm::flags
inline constexpr uint32_t kFormFlagDeleted = 0x20;      // TESObjectREFR::kFlags_Deleted
inline constexpr uint32_t kRefRenderState = 0x64;       // TESObjectREFR::renderState; niNode at +0x14 (GetNiNode 0x43FCD0)
inline constexpr uint32_t kRenderStateNiNode = 0x14;
inline constexpr uint32_t kNiWorldTranslate = 0x8C;     // NiAVObject world translate (xNVSE struct is wrong here, see animprobe)
inline constexpr uint32_t kActorBaseProcess = 0x68;     // MobileObject::baseProcess
inline constexpr uint32_t kActorLifeState = 0x108;
inline constexpr uint32_t kActorSwimming = 0x14D;       // not proven (animprobe), runtime guarded
inline constexpr uint32_t kActorMover = 0x190;          // ActorMover*, vtable checked below
inline constexpr uint32_t kPlayerNotFirstPerson = 0x64A;
// PlayerCharacter::regionsList, a BSSimpleList<TESRegion> (xNVSE GameObjects.h: `TESRegionList regionsList; // 764`; the IsPlayerInRegion command 0x5CF490 reads it through
// 0x4F2770, docs/engine-addresses.md section 11). Node = {TESRegion* item; node* next}, the head node is inline. TESForm::refID at +0x0C.
inline constexpr uint32_t kPlayerRegionList = 0x764, kFormRefId = 0x0C;
inline constexpr uint32_t kVtblActorMover = 0x0109223Cu, kVtblPlayerMover = 0x0109297Cu;
// BaseProcess vtable slots (tools/animcheck/verify_engine.py)
inline constexpr uint32_t kProcGetAnimData = 0x6E, kProcGetAnimAction = 0xF9, kProcIsAiming = 0x101, kProcGetKnocked = 0x103,
                          kProcIsWeaponOut = 0x115;
// AnimData: animSequence[8] at +0xE0, animSeq100 at +0x100; BSAnimGroupSequence animGroup at +0x74, TESAnimGroup id (u8) at +0x10.
// animGroup MEASURED 2026-10-29 in two running copies: +0x68 holds 0x0000FFFF (not a pointer), +0x74 points at the TESAnimGroup (vtable 0x010484E8);
// the xNVSE header's "068" is wrong (the sequence is 0x78 bytes, tools/animcheck/verify_engine.py). With +0x68 every group read found nothing.
// The id byte MEASURED the same day: for "mtidle.kf" (group Idle = 0) the byte at +0x10 is 0 and the one at +0x08 is 0x0A (plugin/animprobe reads +0x10).
inline constexpr uint32_t kAnimDataSeqBase = 0xE0, kAnimDataSeq9 = 0x100, kSeqAnimGroup = 0x74, kAnimGroupId = 0x10, kSeqState = 0x44;
// NiControllerSequence: begin time @ 0x2C, last time @ 0x34 (floats, seconds); (last - begin) is how long the sequence has been playing. Same offsets as
// plugin/actors/anim/anim_pins.h (kSeqBegin / kSeqLast) and plugin/animprobe. Not proven against a running game.
inline constexpr uint32_t kSeqBegin = 0x2C, kSeqLast = 0x34;
inline constexpr uint32_t kVtblBSAnimGroupSequence = 0x0101DE3Cu, kVtblNiControllerSequence = 0x01096584u;
// ExtraContainerChanges: TESObjectREFR::extraDataList (+0x44) holds kExtraData_ContainerChanges (0x15); Data { objList @0 }, EntryData
// { extendData @0, countDelta @4, type @8 }; an entry whose ExtraDataList chain holds kExtraData_Worn (0x16) is worn.
inline constexpr uint8_t kExtraContainerChanges = 0x15, kExtraWorn = 0x16, kExtraWornLeft = 0x17;
// ExtraWeaponModFlags (GameExtraData.h kExtraData_WeaponModFlags = 0x8D): BSExtraData { vtbl, type(u8) @4, next @8 }, then `UInt8 flags` @ 0x0C.
inline constexpr uint8_t kExtraWeaponModFlags = 0x8D;
inline constexpr uint32_t kExtraWeaponModFlagsOffset = 0x0C;
inline constexpr uint32_t kRefExtraDataList = 0x44;

}  // namespace nvcoop::avatar::pins
