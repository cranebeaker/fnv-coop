// Engine facts the NPC animation game backend (anim_game.cpp) relies on. Either a verified address from docs/engine-addresses.md /
// plugin/avatar/engine/engine_pins.h / the command table, or a vtable slot named in the xNVSE header (third_party/NVSE/nvse/nvse/GameProcess.h,
// class BaseProcess) that verify_slots.py proves against the reference FalloutNV.exe 1.4.0.525: the python check reads this file and compares
// every constant below with the exe, so a typo here fails run_tests.cmd. No address is invented.
//
// A direct call is made only after a RUN-TIME check that the function the actor's process vtable holds in that slot IS the pinned function
// (a process of another class, or another exe build, makes the call Unsupported and the backend falls back to a console line).
#pragma once
#include <cstdint>

namespace nvcoop::actors::anim::pins {

// HighProcess vtable (RTTI walk in tools/animcheck/verify_engine.py). Every loaded actor near the player has a HighProcess or a subclass.
inline constexpr uint32_t kHighProcessVtable = 0x01087864u;

// Slot numbers = the position of the virtual function in BaseProcess (GameProcess.h). Function addresses = what the HighProcess vtable holds there.
inline constexpr uint32_t kSlotSetIsAiming = 0x100, kSlotGetKnockedState = 0x103, kSlotSetKnockedState = 0x104, kSlotPushActorAway = 0x106,
                          kSlotIsWeaponOut = 0x115, kSlotSetWeaponOut = 0x116, kSlotGetSitSleepState = 0x12F;
inline constexpr uint32_t kFnSetIsAiming = 0x008D8820u;      // void __thiscall(BaseProcess*, bool): writes byte +0x228
inline constexpr uint32_t kFnSetKnockedState = 0x008D8560u;  // void __thiscall(BaseProcess*, char): writes byte +0x13C
inline constexpr uint32_t kFnPushActorAway = 0x0091FEE0u;    // void __thiscall(BaseProcess*, Actor* pushed, float x, y, z, float force)
inline constexpr uint32_t kFnIsWeaponOut = 0x00915D40u;      // bool __thiscall(BaseProcess*): reads byte +0x135
inline constexpr uint32_t kFnSetWeaponOut = 0x00915D60u;     // void __thiscall(BaseProcess*, Actor*, bool): writes byte +0x135 and notifies the actor
inline constexpr uint32_t kFnGetSitSleepState = 0x008D8B20u; // int __thiscall(BaseProcess*): the byte +0x13D (xNVSE kSitSleepState_*); 4 and 9 read as 5 and 10 while a virtual of the process (+0x618) reports bit 3

// cdecl bool MenuMode(): the condition `MenuMode` without an argument (docs/re/dialogmenu-menus-vats-inventory.md, plugin/avatar/engine/engine_pins.h).
inline constexpr uint32_t kMenuModeAny = 0x00702360u;
inline constexpr uint8_t kMenuModeBytes[8] = {0x55, 0x8b, 0xec, 0xe8, 0xa8, 0x4e, 0xdb, 0xff};

// Object layouts (xNVSE GameObjects.h; animprobe verified the process / mover reads, plugin/avatar/engine/engine_pins.h repeats them)
inline constexpr uint32_t kActorBaseProcess = 0x68, kActorLifeState = 0x108, kActorRagdoll = 0xAC, kActorSwimming = 0x14D, kActorMover = 0x190;
inline constexpr uint32_t kRefRenderState = 0x64, kRenderStateNiNode = 0x14;
inline constexpr uint32_t kAnimDataSeqBase = 0xE0, kAnimDataSeq100 = 0x100;     // AnimData::animSequence[8] and animSeq100 (not proven: guarded by the sequence vtable)
// MEASURED 2026-10-29 (read-only memory reads of a running FalloutNV.exe 1.4.0.525): in a live BSAnimGroupSequence (vtable 0x0101DE3C) the dword at +0x68 is
// 0x0000FFFF, not a pointer, and +0x74 points at the TESAnimGroup (vtable 0x010484E8); in it the byte at +0x10 is the group id (Idle = 0 for
// "Characters\_Male\Locomotion\mtidle.kf"; +0x08 held 0x0A). The xNVSE header comment (animGroup "068", id at 08) is wrong; the sequence is 0x78 bytes.
inline constexpr uint32_t kSeqAnimGroup = 0x74, kAnimGroupId = 0x10, kSeqState = 0x44, kSeqLast = 0x34, kSeqBegin = 0x2C, kSeqFreq = 0x28;
inline constexpr uint32_t kVtblBSAnimGroupSequence = 0x0101DE3Cu, kVtblNiControllerSequence = 0x01096584u, kVtblActorMover = 0x0109223Cu, kVtblPlayerMover = 0x0109297Cu;
inline constexpr uint32_t kProcGetAnimData = 0x6E;   // BaseProcess slot, verified by verify_engine.py
// Furniture lookup (anim_game.cpp): a placed furniture reference within this distance of a seated actor is taken for the one it uses. The engine
// has no pinned read for "the furniture this actor uses" (xNVSE has no layout for it), so this is a heuristic, UNMEASURED in a game.
inline constexpr float kFurnitureReachXY = 140.0f, kFurnitureReachZ = 120.0f;
inline constexpr uint32_t kFormFlagDeleted = 0x20;    // TESObjectREFR::kFlags_Deleted (plugin/avatar/engine/engine_pins.h)

}  // namespace nvcoop::actors::anim::pins
