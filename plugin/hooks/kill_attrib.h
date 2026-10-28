// Kill attribution helpers (game-free; the engine glue is killWithAttribution / setKillerOf in engine_hooks.cpp).
//
// Engine facts (docs/engine-addresses.md section 11, read from the reference FalloutNV.exe 1.4.0.525):
//   * Actor::Kill @0x89D900 (thiscall(victim, killer ref, float), aligned frame, not hookable) stores the killer reference pointer at
//     victim+0xC0 (`mov [ecx+0xC0], edx`, 0x89DAAC) before it raises the death.
//   * The IsKiller condition evaluator @0x5A3F00 returns 1 when the calling reference is an actor, is dead (vtbl+0x22C) and param1 == [actor+0xC0].
//   * Cmd_KillActor @0x5BE2A0 passes its optional reference parameter as that killer, so the console line `KillActor <killerRef>` run on the victim
//     makes the ENGINE store the killer in the right order (before OnDeath). The console accepts hex form ids as arguments.
#pragma once
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <string>

namespace nvhooks {

constexpr size_t kActorKillerOffset = 0xC0;
constexpr uint8_t kFormTypeACHR = 0x3B, kFormTypeACRE = 0x3C;
inline bool isActorFormType(uint8_t typeID) { return typeID == kFormTypeACHR || typeID == kFormTypeACRE; }

// `actor` is the victim object, `typeID` its form type, `killer` the killer reference object (may be null to clear). False when it is not an actor.
inline bool storeKiller(void* actor, uint8_t typeID, void* killer) {
#ifdef NVHOOKS_MUTANT
    const size_t off = kActorKillerOffset + 4;   // mutant: the neighbouring field
#else
    const size_t off = kActorKillerOffset;
#endif
    if (!actor || !isActorFormType(typeID)) return false;
    std::memcpy(static_cast<char*>(actor) + off, &killer, sizeof killer);
    return true;
}
inline void* loadKiller(const void* actor, uint8_t typeID) {
    void* k = nullptr;
    if (actor && isActorFormType(typeID)) std::memcpy(&k, static_cast<const char*>(actor) + kActorKillerOffset, sizeof k);
    return k;
}

}  // namespace nvhooks
