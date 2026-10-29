// Owner side of the NPC animation sync (enemy-anim): turns what the engine reports about an actor this machine OWNS into the
// live::AnimState of its snapshot (locomotion code, weapon / sneak / jump / reload / aim, and the schema v2 extension: the one-shot action
// group + counter, dead, knocked, swim, talk, block, ragdoll).
//
// Rules (each has a test and a mutation twin in tests/test_anim.cpp):
//   * booleans are debounced: the engine's weapon / aim / knocked flags flicker for a frame, so a change is published only after
//     `flickerSamples` equal readings in a row (the first reading of an actor is taken at once);
//   * a one-shot (attack swing / shot, reload, stagger, recoil, block hit, dodge ...) is reported as `action` = its base group id and
//     `actionSeq` is bumped when it STARTS (a new group, or the same group restarted: play time going backwards), exactly once;
//   * the action is held in the stream for at least `actionHoldMs` so a lost datagram or a slow frame cannot skip a short one;
//   * a dead actor reports dead and nothing else (no locomotion, no action): death is a pose, not a movement;
//   * furniture: the sit state (entering / using / leaving, bed or chair) is read from the process, debounced like the flags above, and
//     travels with the furniture reference. The reference is remembered while the actor stays in the furniture (the engine read can come
//     back empty for a moment), and forgotten with the pose.
#pragma once
#include <map>

#include "npc_anim_config.h"
#include "npc_anim_engine.h"

namespace nvcoop::actors::anim {

struct SamplerStats {
    uint64_t samples = 0, unreadable = 0, actionsStarted = 0, actionsRestarted = 0, flickersAbsorbed = 0;
};

class NpcAnimSampler {
public:
    NpcAnimSampler(INpcAnimEngine& engine, const SamplerConfig& cfg = {}) : eng_(engine), cfg_(cfg) {}
    // `snap` already holds pos / rot (pitch, yaw, roll radians) / vel / cell from the port. Fills snap.anim. False = the actor could not be
    // read; snap.anim is then left as it was (the port's own fields stay valid).
    bool sample(NetId id, live::ActorSnapshot& snap, uint32_t nowMs);
    void forget(NetId id) { st_.erase(id); }
    // This machine takes over an actor another machine ran: continue its one-shot counter, so the others (who compare against the last value they
    // saw) see the next event as newer instead of as a stale value from a restarted counter.
    void seed(NetId id, uint8_t lastSeq) { st_[id].actionSeq = lastSeq; }
    void clear() { st_.clear(); }
    const SamplerStats& stats() const { return stats_; }
    // For tests and the dashboard: the last published state of an actor.
    bool last(NetId id, live::AnimState& out) const;

private:
    struct Flag {
        bool value = false, init = false;
        bool pending = false;
        uint32_t same = 0;
    };
    // A small enumerated reading (the sit state) with the same debounce as the boolean flags.
    struct Level {
        uint8_t value = 0, pending = 0;
        bool init = false;
        uint32_t same = 0;
    };
    struct State {
        Flag weaponOut, aiming, knocked, swimming, talking;
        Level sit;                    // anim_sit.h state | bed << 2
        uint32_t furniture = 0;       // the furniture the actor is in, kept while it stays in it
        bool haveLoco = false, runNow = false;
        int dirPrev = -1;
        uint8_t actionSeq = 0;
        int rawAction = -1;           // last action group seen on the engine slot
        int rawPlayMs = -1;
        int curAction = 0;            // published action group
        uint32_t heldUntil = 0;
        bool holding = false;
        live::AnimState last;
        bool haveLast = false;
    };
    bool debounce(Flag& f, bool raw);
    uint8_t debounceLevel(Level& l, uint8_t raw);
    INpcAnimEngine& eng_;
    SamplerConfig cfg_;
    std::map<NetId, State> st_;
    SamplerStats stats_;
};

}  // namespace nvcoop::actors::anim
