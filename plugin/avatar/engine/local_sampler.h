// Reads the LOCAL player (pose, velocity, animation state, equipment) through IGameBackend and turns it into what goes on the wire:
// a live::ActorSnapshot at 20 Hz (built with avatar::MakeLocalSnapshot, live_bridge.h) and a reliable "my equipment is now ..." event.
// Game-free apart from the backend reads; unit tested against FakeGameBackend and judged by the same detectors as the receiver.
//
// Conventions (all UNMEASURED in the running game, docs/tools/avatar.md TODO-RE):
//   * rot = (pitch, yaw, roll) = (rotX, rotZ, rotY) of the player reference; yaw is the heading.
//   * locomotion: ActorMover flags bit 7 walk / bit 8 run / bit 9 sneak are used only when at least one of the walk / run bits is
//     set (a mover whose flags say nothing is not trusted); otherwise the speed decides (SpeedBands in loco.h).
//   * jump / reload come from the running animation groups (the xNVSE group table: 227..247 jump, 173..199 reload), because the
//     engine has no "is jumping" accessor we can verify; aim from the process IsAiming; weapon drawn from the IsWeaponOut evaluator;
//     sneak from the IsSneaking evaluator (mover bit 9 as fallback).
#pragma once
#include <cstdint>
#include <vector>

#include "../avatar_types.h"
#include "../live_bridge.h"
#include "game_backend.h"

namespace nvcoop::avatar::engine {

struct LocalSamplerConfig {
    uint32_t periodMs = 50;          // 20 Hz
    float teleportUnits = 100.0f;    // a position jump larger than this AND faster than maxSpeed is a teleport, not a speed (at 33 ms, 2200 u/s is 73 units)
    float velAlpha = 0.6f;           // smoothing of the finite-difference velocity (1 = no smoothing)
    uint32_t maxGapMs = 500;         // a longer pause restarts the velocity estimate
    bool speedTeleport = true;       // false: the old rule, a jump of more than 350 units in one period (ini [avatar] legacyRules=1)
    float maxSpeed = 2200.0f;        // clamp (units/s)
    // The player's one-shots (attack swing / shot / throw, power attack, block, stagger ...). Same contract as the NPC layer's sampler (plugin/actors/anim).
    uint32_t actionHoldMs = 600;     // a one-shot stays in the stream at least this long: the far rate is 5 Hz and a datagram can be lost, a 260 ms hold could be skipped
    uint32_t heldActionHoldMs = 150; // a held action (block, automatic fire) lingers this long after the engine stops playing it: releasing the block shows quickly
    int restartBackstepMs = 60;      // the same group's play time going back by more than this = the owner started it again (two punches in a row)
};

// One engine read of the player, already reduced to what the snapshot needs.
struct LocalFrame {
    bool valid = false;              // the player exists, is in a cell, has finite numbers
    CellId cell = 0;
    Vec3 pos;
    Vec3 rot;                        // (pitch, yaw, roll)
    Anim anim;                       // loco is filled by Sample (needs the velocity)
    bool haveMoveFlags = false, moverRun = false, moverWalk = false, moverSneak = false;
    bool swimming = false, dead = false, firstPerson = false;
    bool animReadable = false;
    int actionGroup = -1;            // base group id of the action group playing now (attack, block, stagger, reload ...), -1 none
    int actionPlayMs = -1;           // how long it has been playing, -1 unknown
    bool havePos3D = false;          // the player's 3D node was readable: pos3D is where the renderer has him (the reference position `pos` can lag it)
    Vec3 pos3D;
};

// Group classes for the local animation state (xNVSE TESAnimGroup table, tools/animcheck/animgroups_table.py).
bool IsReloadGroupId(int g);
bool IsJumpGroupId(int g);
bool IsAttackGroupId(int g);

// The player's one-shot as the snapshot carries it: the group playing now (held for a while after it ends), a counter that bumps when one STARTS (a new group, or
// the same group restarted: its play time going backwards) and the block bit. The counter is never reset (a copy compares it with the last value it saw).
class ActionTracker {
public:
    struct Out {
        uint8_t action = 0, actionSeq = 0;
        bool block = false;
    };
    explicit ActionTracker(const LocalSamplerConfig& cfg = {}) : cfg_(cfg) {}
    // `rawGroup`: the action group playing now (-1 none), `rawPlayMs` how long it has been playing (-1 unknown).
    Out Step(uint32_t nowMs, int rawGroup, int rawPlayMs);
    // Nothing is playing (dead, not readable, a load): no action, the counter is kept.
    void Clear();
    uint8_t seq() const { return seq_; }

private:
    LocalSamplerConfig cfg_;
    uint8_t seq_ = 0;
    int rawPrev_ = -1, playPrev_ = -1;
    int cur_ = 0;
    uint32_t heldUntil_ = 0;
    bool holding_ = false;
};

// Reads the player into a frame. Cheap: one reference read, one animation read, two evaluators.
LocalFrame ReadLocalFrame(IGameBackend& be);

class LocalSampler {
public:
    explicit LocalSampler(LocalSamplerConfig cfg = {}) : cfg_(cfg), action_(cfg) {}

    // Identity comes from the co-op layer (net id of the local player's avatar and the ownership epoch). The sequence restarts at 1
    // when either changes.
    void SetIdentity(live::NetId netId, live::OwnerEpoch epoch);
    bool Due(uint32_t nowMs) const;
    // Builds the next snapshot when one is due. False when not due or the world is not ready (nothing is published then).
    bool Sample(IGameBackend& be, uint32_t nowMs, live::ActorSnapshot& out);
    // The same from an already read frame (tests, loopback).
    bool Update(uint32_t nowMs, const LocalFrame& f, live::ActorSnapshot& out);
    void Reset();   // after a load / teleport: forget the previous position

    struct Counters {
        uint64_t published = 0, notReady = 0, teleports = 0, cellChanges = 0;
    };
    const Counters& counters() const { return c_; }
    const LocalFrame& lastFrame() const { return last_; }
    Vec3 lastVelocity() const { return vel_; }
    uint8_t lastLoco() const { return loco_; }

private:
    LocalSamplerConfig cfg_;
    ActionTracker action_;
    live::NetId netId_ = 0;
    live::OwnerEpoch epoch_ = 0;
    uint32_t seq_ = 0;
    bool haveAt_ = false;
    uint32_t nextAt_ = 0;
    bool havePrev_ = false;
    CellId prevCell_ = 0;
    Vec3 prevPos_;
    uint32_t prevAt_ = 0;
    Vec3 vel_;
    uint8_t loco_ = 0;
    LocalFrame last_;
    Counters c_;
};

// Watches the player's worn items. Poll() returns true once per stable change (and once at the start) with the full set, weapons
// flagged, sorted by form id. Not part of the 20 Hz snapshot: it goes on the reliable channel.
class EquipWatcher {
public:
    explicit EquipWatcher(uint32_t pollMs = 400, uint32_t stableMs = 250) : pollMs_(pollMs), stableMs_(stableMs) {}
    bool Poll(IGameBackend& be, uint32_t nowMs, std::vector<EquipEntry>& out);
    void Reset();
    const std::vector<EquipEntry>& published() const { return published_; }

private:
    uint32_t pollMs_, stableMs_;
    bool havePolled_ = false;
    uint32_t lastPoll_ = 0;
    bool havePublished_ = false;
    std::vector<EquipEntry> published_, pending_;
    bool havePending_ = false;
    uint32_t pendingSince_ = 0;
};

}  // namespace nvcoop::avatar::engine
