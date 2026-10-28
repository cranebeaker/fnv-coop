// Player facts for a guest's dialogue result script (P3 handover R1, G05). The result script of the INFO a guest picked runs on the HOST; its
// `player.GetItemCount` / `GetAV` / `HasPerk` / `GetEquipped` would answer for the host's player. The initiator's machine therefore sends the facts
// the INFO reads (plugin/dialogue Pick.facts) and the host answers those four reads from them, for the player only, while the scope below is alive.
// Game-free; the detours (script commands + the condition evaluators of the same names) are in engine_hooks.cpp.
#pragma once
#include <cstdint>
#include <vector>

#include "fnvcoop/types.h"

namespace nvhooks {

// Same meaning as fnvcoop_dlg::Fact (plugin/dialogue): kind 1 actor value (what.localId = the AV code), 2 item count, 3 has perk (value = rank, 0 = no),
// 4 equipped (0 / 1). The dialogue module's Fact converts field by field; this header does not depend on it.
enum : uint8_t { kFactActorValue = 1, kFactItemCount = 2, kFactHasPerk = 3, kFactEquipped = 4 };
struct PlayerFact {
    uint8_t kind = 0;
    fnvcoop::FormRef what;
    int32_t value = 0;
};

// The value the initiator reported for (kind, what); false = no such fact (the engine answers). An actor value is matched by its code alone (its
// FormRef has no plugin), every other kind by the whole reference.
inline bool lookupFact(const std::vector<PlayerFact>& facts, uint8_t kind, const fnvcoop::FormRef& what, int32_t* value) {
    for (const PlayerFact& f : facts) {
        if (f.kind != kind) continue;
#ifdef NVHOOKS_MUTANT
        const bool same = f.what.localId == what.localId;   // mutant: the plugin is ignored, so two plugins' forms with the same local id alias
#else
        const bool same = kind == kFactActorValue ? f.what.localId == what.localId : f.what == what;
#endif
        if (same) { *value = f.value; return true; }
    }
    return false;
}

}  // namespace nvhooks
