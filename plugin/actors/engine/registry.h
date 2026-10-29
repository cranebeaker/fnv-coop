// Internal to plugin/actors/engine: the process-wide registry behind actor_engine_api.h (listeners, providers, config, counters).
// The port, the feed and the module fill the providers; the free functions of the API read them. Game-free.
#pragma once
#include <functional>
#include <vector>

#include "actors/actor_sync.h"
#include "actors/engine/actor_engine_api.h"

namespace nvcoop::actors::engine {

class IActorEngine;   // engine_iface.h

class Registry {
public:
    // ---- providers (set by the module / tests) ----
    std::function<ActorSync*()> sync;                           // the running session's ActorSync, null = no session
    std::function<NetId(RefId)> netOfRef;                       // tracked actor ref -> NetId (real or stand-in or pick clone), 0 = none
    std::function<RefId(NetId)> refOfNet;                       // NetId -> live ref, 0 = none
    std::function<bool(RefId)> isStandInRef;
    std::function<bool(RefId)> localDeath;                      // the feed's decision for onLocalDeath(ref)
    RefFilter exclude;                                          // refs another layer owns (puppets, ghosts)
    CreatedIdResolver createdId;                                // spawn-arb: identity of a created ref
    InventoryReader invRead;                                    // plugin/loot: inventory totals of a tracked actor
    InventoryWriter invWrite;
    EquipmentReader equipRead;                                  // what a tracked actor holds and wears (engine_module.cpp wires equipment.h); null = equipment is not synced
    EquipmentWriter equipWrite;
    std::function<bool(NetId)> poseLocked;                     // plugin/actors/anim: the copy is getting out of furniture, do not move it yet; null = never locked
    std::function<void()> beforeSave;                           // the feed's clone clean-up
    std::function<PortStats()> stats;
    PlayerNetIdResolver playerNetByRef;
    PlayerRefResolver playerRefByNet;
    KillerResolver killerOf;                                    // G04: the puppet of the remote player whose shot is to be credited with a copy's death (plugin/combat)
    HitForwarder hitForwarder;
    AnimSampler animSampler;
    IActorEngine* engine = nullptr;                             // the process-wide engine the port runs on (wireApi); other lanes share it via actorEngine()
    Config cfg;
    uint32_t cfgVersion = 0;                                    // bumped by configure(): the feed and the port pick the new values up at the next tick

    // ---- listeners ----
    ListenerId addTracked(std::function<void(const TrackedEvent&)> f) { return add(tracked_, std::move(f)); }
    ListenerId addDied(std::function<void(const DiedEvent&)> f) { return add(died_, std::move(f)); }
    ListenerId addLost(std::function<void(const LostEvent&)> f) { return add(lost_, std::move(f)); }
    ListenerId addDrive(std::function<void(const DriveEvent&)> f) { return add(drive_, std::move(f)); }
    void remove(ListenerId id) {
        remove(tracked_, id); remove(died_, id); remove(lost_, id); remove(drive_, id);
    }
    void emit(const TrackedEvent& e) { fire(tracked_, e); }
    void emit(const DiedEvent& e) { fire(died_, e); }
    void emit(const LostEvent& e) { fire(lost_, e); }
    void emit(const DriveEvent& e) { fire(drive_, e); }
    size_t listenerCount() const { return tracked_.size() + died_.size() + lost_.size() + drive_.size(); }
    bool hasDriveListeners() const { return !drive_.empty(); }

    void reset() { *this = Registry(); }   // tests

private:
    template <class E> struct Slot { ListenerId id; std::function<void(const E&)> fn; };
    template <class E> ListenerId add(std::vector<Slot<E>>& v, std::function<void(const E&)> f) {
        if (!f) return 0;
        v.push_back({++next_, std::move(f)});
        return next_;
    }
    template <class E> static void remove(std::vector<Slot<E>>& v, ListenerId id) {
        for (size_t i = 0; i < v.size(); ++i)
            if (v[i].id == id) { v.erase(v.begin() + static_cast<long>(i)); return; }
    }
    // Calls the listeners that exist when the event starts; one removed (even by an earlier callback of this event) is not called.
    template <class E> static void fire(std::vector<Slot<E>>& v, const E& e) {
        std::vector<ListenerId> ids;
        ids.reserve(v.size());
        for (auto& s : v) ids.push_back(s.id);
        for (ListenerId id : ids) {
            std::function<void(const E&)> fn;
            for (auto& s : v) if (s.id == id) { fn = s.fn; break; }
            if (fn) fn(e);
        }
    }
    ListenerId next_ = 0;
    std::vector<Slot<TrackedEvent>> tracked_;
    std::vector<Slot<DiedEvent>> died_;
    std::vector<Slot<LostEvent>> lost_;
    std::vector<Slot<DriveEvent>> drive_;
};

Registry& registry();

// Points the registry's providers at one machine's port / feed / session (the module does this once; tests do it per machine).
class EnginePort;
class ActorFeed;
class IActorEngine;
void wireApi(EnginePort& port, ActorFeed& feed, IActorEngine& eng, std::function<ActorSync*()> sync);

}  // namespace nvcoop::actors::engine
