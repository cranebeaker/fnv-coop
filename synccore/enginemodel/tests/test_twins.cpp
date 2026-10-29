// Mutation twins: switch on one known bug class; the scenario that guards it must then report a violation.
#include "scenarios.h"
#include "minitest.h"

using namespace fnvcoop;
using namespace fnvcoop::engine;
using namespace fnvcoop::testing;

namespace {
LinkParams dupLink() {
    LinkParams l;
    l.minLatencyMs = 5; l.maxLatencyMs = 25; l.duplicate = 0.3; l.loss = 0.05;
    return l;
}
}  // namespace

#define OWN_TWIN(name, flag, scenario) \
    TEST(twin, name) { OwnershipFaults f; f.flag = true; { OwnershipFaultScope s(f); CHECK(!(scenario)); } }
#define ENGINE_TWIN(name, flag, scenario) \
    TEST(twin, name) { EngineFaults f; f.flag = true; { EngineFaultScope s(f); CHECK(!(scenario)); } }

OWN_TWIN(own_no_epoch, noEpochBump, churnHolds(1, 400))
OWN_TWIN(own_stale_epoch, acceptStaleEpoch, staleEchoRejected())
OWN_TWIN(own_adopt_echo, adoptEcho, staleEchoRejected())
OWN_TWIN(own_transfer_no_ack, transferNoAck, handoffKeepsLastState(1))
OWN_TWIN(own_no_hysteresis, noHysteresis, noFlap())
OWN_TWIN(own_follower_ignored, followerIgnored, followerStaysWithRecruiter())
OWN_TWIN(own_accept_non_owner, acceptNonOwner, churnHolds(1, 400))
OWN_TWIN(own_commit_to_gone_owner, commitToGoneOwner, commitNothingToHandOverHolds())
OWN_TWIN(own_pinned_can_move, pinnedCanMove, pinnedPlayersHold())
OWN_TWIN(own_no_epoch_floor, noEpochFloor, pinnedPlayersHold())
OWN_TWIN(own_leave_keeps_pinned, leaveKeepsPinned, pinnedPlayersHold())
// the same faults caught by the engine model's invariants in a whole session (leave, rejoin, host load)
OWN_TWIN(own_model_leave_keeps_pinned, leaveKeepsPinned, playerLeavesAndRejoins(5))
OWN_TWIN(own_model_no_epoch_floor, noEpochFloor, playerLeavesAndRejoins(5))

ENGINE_TWIN(no_dedupe, noDedupe, questStageSync(1, dupLink()))
ENGINE_TWIN(hit_no_validation, hitNoValidation, forgedHitsRejected(1))
ENGINE_TWIN(hit_by_non_owner, hitAppliedByNonOwner, hitToNonOwnerRejected(1))
ENGINE_TWIN(pickup_no_arbitration, pickupNoArbitration, lootRace(1))
ENGINE_TWIN(pickup_no_rollback, noPickupRollback, lootRace(1))
ENGINE_TWIN(spawn_no_arbitration, spawnNoArbitration, spawnOnce(1))
ENGINE_TWIN(loot_per_machine, lootPerMachine, fightScenario(1, {}))
ENGINE_TWIN(pick_per_machine, pickPerMachine, spawnOnce(1))
ENGINE_TWIN(container_no_lock, containerNoLock, containerMatrix(1))
ENGINE_TWIN(named_killable, namedActorKillable, namedProtected(1))
ENGINE_TWIN(no_session_epoch, noSessionEpoch, saveLoadRejoin(1))
ENGINE_TWIN(heal_disabled, healDisabled, healsCorruption(1))
ENGINE_TWIN(drop_no_idempotency, dropNoIdempotency, dropIdempotent(1))
ENGINE_TWIN(join_no_ownership, joinNoOwnership, lateJoinerGetsOwnership(1))
// The scenarios of the NPC-sync fixes report a failed SETUP (no suitable actor, cluster never quiet before the test begins) with the same
// "false" as a caught bug. A twin that only fails because its setup broke proves nothing, so these twins also require that the scenario
// failed on its real checks (the reason must not be a setup failure), on both seeds the real tests use.
#define STRICT_ENGINE_TWIN(name, flag, scenario)                                                                                        \
    TEST(twin, name) {                                                                                                                  \
        EngineFaults f; f.flag = true;                                                                                                  \
        EngineFaultScope s(f);                                                                                                          \
        for (uint64_t seed = 1; seed <= 2; ++seed) {                                                                                    \
            std::string why;                                                                                                            \
            const bool ok = scenario(seed, &why);                                                                                       \
            CHECK_MSG(!ok, std::string("twin not caught on seed ") + std::to_string(seed));                                             \
            CHECK_MSG(why.find("setup") == std::string::npos && why.find("no actor far") == std::string::npos,                          \
                      "twin 'caught' only because the scenario setup failed (seed " + std::to_string(seed) + "): " + why);              \
        }                                                                                                                               \
    }
STRICT_ENGINE_TWIN(digest_buckets_positions, digestBucketsPositions, smallDriftIsHealed)
// combat-core: the scenarios say "setup" in `why` when they could not start, which the strict twin refuses to count as a catch.
// The reason each twin was caught is printed (it documents what the oracle saw).
#define COMBAT_TWIN(name, flag, scenario)                                                                                               \
    TEST(twin, name) {                                                                                                                  \
        EngineFaults f; f.flag = true;                                                                                                  \
        EngineFaultScope s(f);                                                                                                          \
        for (uint64_t seed = 1; seed <= 2; ++seed) {                                                                                    \
            std::string why;                                                                                                            \
            const bool ok = scenario(seed, &why);                                                                                       \
            CHECK_MSG(!ok, std::string("twin not caught on seed ") + std::to_string(seed));                                             \
            CHECK_MSG(why.find("setup") == std::string::npos, "twin 'caught' only because the scenario setup failed (seed " + std::to_string(seed) + "): " + why); \
            std::printf("    twin %s seed %d caught by: %s\n", #name, static_cast<int>(seed), why.substr(0, 160).c_str());                 \
        }                                                                                                                               \
    }
COMBAT_TWIN(death_not_idempotent, deathNotIdempotent, deathHappensOnce)
COMBAT_TWIN(limb_not_synced, limbNotSynced, limbsSync)
COMBAT_TWIN(visual_shot_damages, visualShotDamages, projectileVisualOnly)
COMBAT_TWIN(blast_runs_everywhere, blastRunsEverywhere, blastViaOwners)
COMBAT_TWIN(hit_no_validation_combat, hitNoValidation, forgedCombatRejected)
STRICT_ENGINE_TWIN(stale_final_not_forwarded, staleFinalNotForwarded, forcedCommitGuestNewOwnerStartsPromptly)
STRICT_ENGINE_TWIN(digest_no_seq, digestNoSeq, duplicatedDigestIsNotAStableMismatch)
STRICT_ENGINE_TWIN(new_owner_no_wait, newOwnerNoWait, forcedCommitNeverDrivesTwice)
STRICT_ENGINE_TWIN(created_key_is_player, createdKeyIsPlayer, createdActorRepairKeepsPlayerState)
STRICT_ENGINE_TWIN(join_no_tombstones, joinNoTombstones, joinerKeepsReleasedEpochs)

// A former owner that never noticed it lost the actor keeps publishing; the receivers must reject it. The bug only
// bites when a receiver also applies old-epoch updates, so the twin combines the two switches.
TEST(twin, owner_keeps_publishing) {
    EngineFaults e; e.ownerKeepsPublishing = true;
    OwnershipFaults o; o.acceptStaleEpoch = true;
    EngineFaultScope es(e);
    OwnershipFaultScope os(o);
    CHECK(!formerOwnerSilenced(1));
}
