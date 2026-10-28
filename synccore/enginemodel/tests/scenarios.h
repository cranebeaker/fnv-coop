// Scenarios shared by the real tests and their mutation twins. Each returns true iff every invariant held; the
// twin turns one fault switch on and expects false. `why` (optional) receives the first few problems.
#pragma once
#include <string>
#include "fnvcoop/engine/cluster.h"
#include "fnvcoop/ownership.h"

namespace fnvcoop::testing {

// Debug aid: called with the cluster when a scenario ends with violations (set by a test to print diagnostics).
extern void (*dumpHook)(engine::Cluster&);
extern engine::IEngineObserver* debugObserver;   // optional: attached to the cluster of randomSession

// ---- ownership (game-free, no network) ----
bool churnHolds(uint64_t seed, int steps, std::string* why = nullptr);   // random presence churn, lossy ack path, reordered duplicated OwnMsgs
bool staleEchoRejected(std::string* why = nullptr);                      // Skyrim Together PR #906: an old assignment replayed must not win
bool rejoinEpochsMonotonic(std::string* why = nullptr);
bool commitNothingToHandOverHolds(std::string* why = nullptr);                // a handoff the old owner has nothing to hand over commits at once, only to a connected new owner, only once
bool noFlap(std::string* why = nullptr);                                 // two players equally near: ownership must not flap (engine level)
bool followerStaysWithRecruiter(std::string* why = nullptr);

// ---- engine scenarios (Cluster) ----
engine::ClusterConfig baseConfig(uint64_t seed, int players, const LinkParams& link = {});
LinkParams lossyLink();

bool joinAndSync(uint64_t seed, int players, const LinkParams& link, std::string* why = nullptr);
bool fightScenario(uint64_t seed, const LinkParams& link, std::string* why = nullptr);
bool handoffMidFight(uint64_t seed, std::string* why = nullptr);
bool handoffKeepsLastState(uint64_t seed, std::string* why = nullptr);   // guest to guest: the old owner's last state reaches the new owner before it takes over
bool lootRace(uint64_t seed, std::string* why = nullptr);
bool spawnOnce(uint64_t seed, std::string* why = nullptr);
bool containerMatrix(uint64_t seed, std::string* why = nullptr);
bool doorsAndLocks(uint64_t seed, std::string* why = nullptr);
bool namedProtected(uint64_t seed, std::string* why = nullptr);
bool questStageSync(uint64_t seed, const LinkParams& link, std::string* why = nullptr);
bool recruitCompanion(uint64_t seed, std::string* why = nullptr);
bool saveLoadRejoin(uint64_t seed, std::string* why = nullptr);
bool healsCorruption(uint64_t seed, std::string* why = nullptr);
bool dropIdempotent(uint64_t seed, std::string* why = nullptr);
bool forgedHitsRejected(uint64_t seed, std::string* why = nullptr);
bool hitToNonOwnerRejected(uint64_t seed, std::string* why = nullptr);
bool lateJoinerGetsOwnership(uint64_t seed, std::string* why = nullptr);
bool formerOwnerSilenced(uint64_t seed, std::string* why = nullptr);
bool createdActorRepairKeepsPlayerState(uint64_t seed, std::string* why = nullptr);   // npc-bugs fix 1: digest keys of created actors are not player slots
bool forcedCommitNeverDrivesTwice(uint64_t seed, std::string* why = nullptr);        // npc-bugs fix 2: a forced commit waits for the old owner's final state before the new owner drives
bool forcedCommitGuestNewOwnerStartsPromptly(uint64_t seed, std::string* why = nullptr);   // npc-bugs fix 2b: the host passes the old owner's final state on, so a guest new owner starts without waiting for the safety timeout
bool duplicatedDigestIsNotAStableMismatch(uint64_t seed, std::string* why = nullptr);   // npc-bugs fix 3: digest reports are numbered; a duplicate is not a second look
bool joinerKeepsReleasedEpochs(uint64_t seed, std::string* why = nullptr);              // a joiner remembers the epoch of an actor that died before it came, so a late old assignment cannot revive an owner for the corpse
bool smallDriftIsHealed(uint64_t seed, std::string* why = nullptr);                      // npc-bugs fix 4: digests compare positions and health exactly
bool randomSession(uint64_t seed, int players, uint32_t ms, const LinkParams& link, std::string* why = nullptr);

// ---- combat-core (scenarios_combat.cpp); a failed SETUP puts "setup" in `why` ----
bool limbsSync(uint64_t seed, std::string* why = nullptr);               // limbs cripple by accumulated damage, the counters survive a handoff, every machine agrees
bool deathHappensOnce(uint64_t seed, std::string* why = nullptr);        // two lethal hits at once: one death, one corpse
bool storyKillRace(uint64_t seed, std::string* why = nullptr);           // story KillActor racing a combat kill: one death
bool projectileVisualOnly(uint64_t seed, std::string* why = nullptr);    // a projectile hurts only through its shooter's machine; copies elsewhere are visual
bool blastViaOwners(uint64_t seed, std::string* why = nullptr);          // an explosion hurts each actor in range once, through its owner
bool killXpOnce(uint64_t seed, std::string* why = nullptr);              // killing blow / party share, paid once
bool forgedCombatRejected(uint64_t seed, std::string* why = nullptr);    // stale, future, spoofed, forged-npc and far hits are rejected; a real one lands

}  // namespace fnvcoop::testing
