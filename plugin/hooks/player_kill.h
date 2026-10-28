// `player.KillActor` and a guest's ghost as the target (G42, wave 1b). Game-free decision; the detour is h_Kill in engine_hooks.cpp.
//
// The rules engine (RulesEngine, PerPlayerState rule: the bomb collar, the cloud, the gas trap) turns a KillActor whose actor is a PLAYER REF into a Health drain
// addressed to the slot that ref belongs to (RulesEngine::Options::playerRef), so a script's `player.kill` kills THAT player and nobody else. For that the capture
// must record KillActor{<the player's own ref>}: the player ref for the host's own player, the ghost / puppet ref the session registered for a guest's slot.
// The engine kill itself runs only on a machine whose own player it concerns (the host's player: slot 0); a kill meant for a guest is never run on the host.
#pragma once
#include <cstdint>
#include <functional>

#include "fnvcoop/types.h"
#include "suppress_policy.h"

namespace nvhooks {

// slot -> the FormRef the rules engine knows that player by (the same lambda the session passes as RulesEngine::Options::playerRef).
using PlayerRefOf = std::function<bool(uint8_t slot, fnvcoop::FormRef* out)>;

struct PlayerKillPlan {
    bool report = false;     // tell the PlayerKillObserver (slot, onHost)
    bool runEngine = true;   // run the original KillActor on this machine
    bool record = false;     // capture KillActor{ref}
    bool legacy = false;     // neither an observer nor a player ref is wired for this slot: the old path (record the operand's own ref, run the engine kill)
    fnvcoop::FormRef ref;
};

inline PlayerKillPlan planPlayerKill(const ActingContext::PlayerTarget& pt, bool hasObserver, const PlayerRefOf& refOf, bool capturing) {
    PlayerKillPlan p;
    if (!pt.player) { p.legacy = true; return p; }
    p.report = hasObserver;
    p.runEngine = pt.onHost;
#ifdef NVHOOKS_MUTANT
    const uint8_t refSlot = 0;   // mutant: every player kill is recorded against the host's own player ref
#else
    const uint8_t refSlot = pt.slot;
#endif
    fnvcoop::FormRef r;
    if (capturing && refOf && refOf(refSlot, &r)) { p.record = true; p.ref = r; }
    p.legacy = !p.report && !p.record;
    if (p.legacy) p.runEngine = true;
    return p;
}

}  // namespace nvhooks
