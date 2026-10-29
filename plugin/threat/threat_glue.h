// What the rest of the plugin may call (and register) for plugin/threat. Everything is optional: with no threat module built or no session running these are
// harmless no-ops (threat_module.cpp is x86 only; without NVCOOP_WITH_THREAT the inline stubs below are used).
//
// Wiring (other lanes; docs/threat.md has the exact one-line changes):
//   plugin/combat   : call noteHit(victimRef, attackerRef, damage) for every hit that lands on an actor this machine owns (its accepted guest hits and the
//                     local engine's own), so the enemy's damage ledger is exact instead of inferred from its health.
//   rebalance mode  : setBalancedQuery([] { return <co-op balanced is on>; }) switches the cover / side-step / flee tactics.
//   death / downed  : setDownedQuery([](uint8_t slot) { return <that player is down>; }) so a downed player is never picked (dead or health <= 0 already counts).
#pragma once
#include <cstdint>
#include <functional>
#include <string>

namespace nvcoop::threat {

#ifdef NVCOOP_WITH_THREAT
void noteHit(uint32_t victimRef, uint32_t attackerRef, float damage);
void setBalancedQuery(std::function<bool()> balancedOn);
void setDownedQuery(std::function<bool(uint8_t slot)> downed);
bool running();
std::string statusJson();
#else
inline void noteHit(uint32_t, uint32_t, float) {}
inline void setBalancedQuery(std::function<bool()>) {}
inline void setDownedQuery(std::function<bool(uint8_t)>) {}
inline bool running() { return false; }
inline std::string statusJson() { return "{}"; }
#endif

}  // namespace nvcoop::threat
