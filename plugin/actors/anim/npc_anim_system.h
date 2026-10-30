// NpcAnimSystem: the one object the glue talks to (enemy-anim). It owns the owner-side sampler, the non-owner driver and the corpse
// bookkeeping, and turns the actor module's callbacks into calls on them:
//
//   actors::engine::setAnimSampler(...)          -> sampleOwned()   (owner: fill snapshot.anim from the engine)
//   actors::engine::addDriveListener(...)        -> drive()         (non-owner: animate the copy that was just positioned)
//   actors::engine::addActorDiedListener(...)    -> onDied()
//   actors::engine::addActorLostListener(...)    -> onLost()
//   ActorSync::setAnimEventHandler(...)          -> onNetEvent()    (the owner's corpse resting place)
//   frame hook                                   -> tick()
//
// Game-free (INpcAnimEngine + IAnimNet are the only outside world); anim_glue.cpp does the wiring inside the game.
#pragma once
#include <functional>
#include <string>
#include <vector>

#include "corpse.h"
#include "npc_anim_config.h"
#include "npc_anim_driver.h"
#include "npc_anim_engine.h"
#include "npc_anim_sampler.h"

namespace nvcoop::actors::anim {

// Reliable owner-to-everyone events (ActorSync::sendAnimEvent). False = refused (this machine is not the owner right now).
class IAnimNet {
public:
    virtual ~IAnimNet() = default;
    virtual bool sendEvent(NetId id, const std::vector<uint8_t>& payload) = 0;
};

struct SystemConfig {
    SamplerConfig sampler;
    DriverConfig driver;
    CorpseWatchConfig corpse;
    uint32_t releaseAfterIdleMs = 800;   // a copy that has not been driven this long and is now OURS is released (its own AI animates it)
    uint32_t corpseResendMs = 5000;      // the corpse event is retried this long while the actor module refuses it (ownership in flux)
};

struct SystemStats {
    uint64_t sampledOwned = 0, drives = 0, died = 0, diedMine = 0, corpseSent = 0, corpseSendRefused = 0, corpseUnreadable = 0, corpseDropped = 0;
    uint64_t netEvents = 0, netEventsBad = 0;
};

class NpcAnimSystem {
public:
    NpcAnimSystem(INpcAnimEngine& engine, IAnimNet& net, const SystemConfig& cfg = {});

    // ---- owner side ----
    bool sampleOwned(NetId id, live::ActorSnapshot& snap, uint32_t nowMs);
    // ---- non-owner side ----
    void drive(NetId id, const live::InterpOut& out, uint32_t nowMs);
    // ---- events ----
    void onDied(NetId id, bool mine, bool fromNetwork, uint32_t nowMs);
    void onLost(NetId id);
    void onRevived(NetId id);
    void onNetEvent(NetId id, const std::vector<uint8_t>& payload, uint32_t nowMs);
    // Once per frame. `isMine` answers the ownership view (null: nothing is ever released).
    void tick(uint32_t nowMs, const std::function<bool(NetId)>& isMine = {});

    NpcAnimSampler& sampler() { return sampler_; }
    NpcAnimDriver& driver() { return driver_; }
    CorpseWatcher& watcher() { return watcher_; }
    const SystemStats& stats() const { return stats_; }
    std::string statsJson() const;   // flat JSON object of every counter (the bridge method answers with it)

private:
    struct PendingCorpse { NetId id; CorpsePose pose; uint32_t since; };
    void trySendCorpse(PendingCorpse& pc, uint32_t now, bool& done);
    void summary(uint32_t now);
    uint32_t nextSummaryMs_ = 0;
    std::string lastSummary_;
    INpcAnimEngine& eng_;
    IAnimNet& net_;
    SystemConfig cfg_;
    NpcAnimSampler sampler_;
    NpcAnimDriver driver_;
    CorpseWatcher watcher_;
    std::vector<PendingCorpse> outbox_;
    std::map<NetId, uint32_t> unreadableSince_;
    SystemStats stats_;
};

}  // namespace nvcoop::actors::anim
