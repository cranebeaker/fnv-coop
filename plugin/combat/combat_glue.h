// What plugin/coop/coop_module.cpp calls (only when built with NVCOOP_WITH_COMBAT). Everything behind it is tested game-free except the engine
// side (combat_module.cpp: the hit gate, the console effects, the ownership/identity view), which has never run in a game.
//
// Wiring (dll-wiring): in coop_module.cpp, next to the actors glue,
//     #ifdef NVCOOP_WITH_COMBAT
//     #include "combat/combat_glue.h"
//     #endif
//     ...once per frame, right after nvcoop::actors::glueFrame(g_rt.get()):
//     #ifdef NVCOOP_WITH_COMBAT
//         nvcoop::combat::glueFrame(g_rt.get());
//     #endif
// The module registers its own bridge method (combat.status) and reads nvcoop.ini [combat] itself (docs/tools/combat-hooks.md).
#pragma once
#include <cstdint>
#include <functional>
#include <string>
#include "combat/combat_sync.h"

namespace nvcoop::coop { class Runtime; }

namespace nvcoop::combat {

void glueFrame(nvcoop::coop::Runtime* rt);       // once per frame after Runtime::frame(); null = the session is gone
bool glueRunning();                              // a CombatSync exists (the coop session is up)
std::string glueStatusJson();                    // combat.status

// VATS interface (Track B). A VATS shot, real-time or not, must reach the engine's hit funnel through its normal callers: the gate then sees it and sets
// the `vats` flag from the engine's VATS mode. Code that resolves a VATS hit ITSELF (calling ApplyHitData 0x89A760 or writing health directly) bypasses the
// gate; it must ask first: fill an EngineHit (attackerRef = the shooter, victimRef, healthDmg, hitLocation, weapon, vats = true) and call glueGate(). The
// answer says Proceed (apply it as computed), Clamp (apply at most maxHealthDamage: the victim is a copy and the hit was forwarded to its owner) or
// Neutralize (apply nothing). With no running session the answer is always Proceed.
GateDecision glueGate(const EngineHit& h);

// Friendly fire (player-vs-player damage) from the campaign (plugin/rebalance; docs/rebalance.md): 1 on, 0 off, -1 back to nvcoop.ini [combat] friendlyfire. Takes effect
// at once in a running session and is kept for the next one. Every machine sets it to the campaign's value, so a hit between players is neutralised (or not) the same everywhere.
void glueSetFriendlyFire(int mode);
// plugin/rebalance (Co-op balanced): the factor for one hit's engine damage, from the victim and where it lands (combat's limb order: 0 torso, 1 head,
// 2..5 arms and legs). Asked once per hit by the hit gate of the machine whose engine computed it. Null = the game's own.
void glueSetHitScale(std::function<float(uint32_t victimRef, int limb)> scale);

// The avatar layer's puppet map (optional). Default: the NV:MP ghost map (nvmp::GhostRefOfPlayer). A puppet that is not mapped is not a
// "player" to combat: hits on it are not forwarded to its real player.
void setPlayerRefMap(std::function<bool(uint32_t ref, uint8_t& slot)> slotOfRef, std::function<uint32_t(uint8_t slot)> refOfSlot);

}  // namespace nvcoop::combat
