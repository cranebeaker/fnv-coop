// Tunables of the NPC animation layer (enemy-anim). Every number is a first guess in game units (70 per metre) or milliseconds; none has
// been measured in the running game. dll-wiring may map nvcoop.ini [anim] keys onto them (docs/tools/enemy-anim.md lists the names).
#pragma once
#include <cstdint>

namespace nvcoop::actors::anim {

// Milliseconds from `since` to `now`, wrap safe.
inline uint32_t elapsed(uint32_t now, uint32_t since) { return static_cast<uint32_t>(now - since); }

struct SamplerConfig {
    // ---- locomotion code (owner side). Same bands as the avatar layer so both agree on walk / run ----
    float standMax = 20.0f;         // below this horizontal speed (units/s) the actor stands
    float runEnter = 230.0f;
    float runExit = 190.0f;
    float fallVz = 350.0f;          // |vertical speed| above this counts as airborne when the engine read cannot say (stairs and ramps stay below)
    // ---- debouncing (VaultMP lesson: the engine's weapon / aim flags flicker for a frame) ----
    uint32_t flickerSamples = 2;    // a boolean must read the same this many samples in a row before it is published (first read is taken at once)
    // ---- one-shots ----
    uint32_t actionHoldMs = 260;    // a one-shot stays in the snapshot stream at least this long (>= 5 snapshots at 20 Hz): a lost datagram or a slow frame cannot skip it
    int restartBackstepMs = 60;     // the same group's play time going back by more than this = the owner started it again
};

struct DriverConfig {
    // ---- call budget (console lines are not free; the port has its own budget for SetPos) ----
    uint32_t budgetPerTick = 12;    // animation commands per tick() interval
    uint32_t hardBudget = 36;       // one-shots, death and weapon changes may exceed the budget up to this
    // ---- locomotion ----
    float standEnter = 20.0f;       // displayed speed above this = moving
    float standExit = 12.0f;        // below this = standing again
    float runEnter = 230.0f;
    float runExit = 190.0f;
    uint32_t speedDebounceMs = 150; // the owner's walk / run code and the displayed speed must disagree this long before the speed wins
    uint32_t speedWindowMs = 120;   // smoothing window of the displayed speed
    float walkNominal = 140.0f;     // nominal ground speed of the walk / run animations, for the playback rate (TODO-RE: per race / creature)
    float runNominal = 330.0f;
    float rateMin = 0.6f, rateMax = 1.6f;
    // ---- turn in place ----
    float turnEnterRadPerSec = 1.2f;
    float turnExitRadPerSec = 0.5f;
    float turnMinTotalRad = 0.35f;  // about 20 degrees of accumulated turn before a turn animation is worth playing
    uint32_t turnMinMs = 300;
    // ---- transitions ----
    uint32_t equipAnimMs = 450;     // draw / holster transition length when standing still
    uint32_t jumpStartMs = 200;
    uint32_t landMs = 350;
    float aimPitchDeadRad = 0.26f;  // about 15 degrees
    // ---- verification (readback) ----
    uint32_t reassertMs = 500;      // how often a driven actor's current group is read back
    uint32_t blindReassertMs = 2000; // when the backend cannot read the group at all, re-issue the loop this often
    uint32_t verifyGraceMs = 250;   // a group just issued is not judged before this
    uint32_t retryBackoffMs = 400;  // base for the exponential backoff after a refused command (x2 per failure, capped at 8 s)
    uint32_t weaponVerifyMs = 700;  // the drawn / holstered state must read back within this
    uint32_t weaponRetryMs = 3000;  // after a failed ladder, try again this much later
    // ---- furniture (sit / sleep) ----
    uint32_t standWaitMs = 1500;    // after a stand-up command the copy must read "out of the furniture" within this, else the next escalation
    uint32_t standPollMs = 100;     // while a stand-up is under way the copy's sit state is read this often (otherwise every reassertMs)
    uint32_t standSettleMs = 900;   // a backend that cannot read the sit state: the stand-up is assumed finished after this
    uint32_t standMaxTries = 3;     // escalations (gentle, then harder) before the layer gives up for sitRetryMs
    uint32_t sitRetryMs = 3000;     // after giving up a stand-up, look again this much later
    // ---- death / corpse ----
    uint32_t deathVerifyMs = 600;   // the actor must read dead this long after we were told it died; else Kill again
    uint32_t deathMaxRetries = 4;
    uint32_t corpseApplyDelayMs = 1500;   // earliest corpse correction after the local death (the local ragdoll gets time to land)
    uint32_t corpseMaxWaitMs = 9000;      // correct anyway after this
    uint32_t corpsePoseTimeoutMs = 15000; // no final pose from the owner this long after the death = counted as missing
    float corpseSnapUnits = 40.0f;        // horizontal distance from the owner's resting place that is worth a correction (about 0.6 m)
    float corpseSnapZ = 30.0f;
    float corpseRestUnitsPerSec = 25.0f;  // the local body counts as at rest below this speed ...
    uint32_t corpseRestMs = 400;          // ... for this long
    uint32_t corpseMaxCorrections = 4;
    // ---- feature switches (an in-game surprise can be switched off without a rebuild) ----
    bool playAttacks = true;        // replay the owner's attack / reload / reaction groups
    bool playTurns = true;
    bool playTalking = true;
    bool playSit = true;            // sit / stand like the owner's actor (furniture)
    bool useRate = true;            // SetGroupRate when the backend has it
    bool playBlock = true;
    bool knockdown = true;
};

struct CorpseWatchConfig {
    uint32_t minRagdollMs = 800;    // never report before this long after the death
    uint32_t restMs = 500;          // the body must not have moved more than restUnits for this long
    float restUnits = 3.0f;
    uint32_t maxWaitMs = 8000;      // report anyway
    uint32_t pollMs = 100;
};

}  // namespace nvcoop::actors::anim
