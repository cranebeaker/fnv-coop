#include "sim.h"
#include <algorithm>
#include <memory>
#include <set>
#include <tuple>
#include "fnvcoop/authority.h"
#include "fnvcoop/replicator.h"

namespace fnvcoop::sim {
namespace {

// What a client's game world looks like (shared story state + this player's own ledger).
struct WorldView {
    std::map<FormRef, uint32_t> stages;
    std::map<std::pair<FormRef, uint32_t>, uint8_t> objectives;
    std::map<FormRef, float> globals;
    std::map<FormRef, bool> enabled;
    std::set<FormRef> kills;
    std::map<FormRef, bool> running;
    std::map<FormRef, Placement> placed;
    std::map<FormRef, int64_t> items;
    std::map<uint32_t, int64_t> av;
    int64_t karma = 0;
    std::map<FormRef, std::pair<int64_t, int64_t>> rep;
    // round 2
    std::map<std::pair<FormRef, uint16_t>, double> scriptVars;
    std::map<FormRef, bool> tethers;
    bool slideshow = false, saveSuppress = false;
    std::map<FormRef, uint8_t> companion;
    std::set<FormRef> released, sayOnce, completed, allObj, failed;
    std::map<FormRef, std::pair<bool, int32_t>> locks;
    std::map<std::pair<FormRef, FormRef>, int32_t> factionRank;
    std::map<std::pair<FormRef, uint8_t>, FormRef> textures;
    std::set<FormRef> fastTravel;   // locked scopes
    std::map<FormRef, int32_t> perks;
    uint32_t controlsOff = 0;
    // round 3
    std::map<FormRef, SpawnInfo> spawned;
    std::set<FormRef> despawned;
    uint32_t createdCounter = 0;
    std::map<FormRef, ObjState> objects;
    std::map<std::pair<FormRef, uint16_t>, double> refVars;
    std::map<FormRef, RefInv> refInv;
    std::set<FormRef> topics, notes;
    std::map<FormRef, uint8_t> markers, compKind;
    FormRef weather; bool weatherSet = false, weatherForced = false;
    float timeScale = 0;
    std::map<std::pair<FormRef, uint8_t>, float> angles, angleOrig;
    OriginMap origins;
    FormRef active; bool hasActive = false;   // per player

    bool operator==(const WorldView& o) const { return diff(o).empty(); }
    std::string diff(const WorldView& o) const {
        if (stages != o.stages) return "stages";
        if (objectives != o.objectives) return "objectives";
        if (globals != o.globals) return "globals";
        if (enabled != o.enabled) return "enabled";
        if (kills != o.kills) {
            std::string r = "kills mine-only:";
            for (auto& k : kills) if (!o.kills.count(k)) r += " " + std::to_string(k.localId);
            r += " oracle-only:";
            for (auto& k : o.kills) if (!kills.count(k)) r += " " + std::to_string(k.localId);
            return r;
        }
        if (running != o.running) return "questRunning";
        if (placed != o.placed) return "placed";
        if (items != o.items) {
            std::string r = "items";
            for (auto& [k, v] : items) { auto it = o.items.find(k); if (it == o.items.end() || it->second != v) r += " " + std::to_string(k.localId) + ":" + std::to_string(v) + "/" + std::to_string(it == o.items.end() ? 0 : it->second); }
            for (auto& [k, v] : o.items) if (!items.count(k)) r += " " + std::to_string(k.localId) + ":0/" + std::to_string(v);
            return r;
        }
        if (av != o.av) return "av";
        if (karma != o.karma) return "karma " + std::to_string(karma) + "/" + std::to_string(o.karma);
        if (rep != o.rep) return "rep";
        if (scriptVars != o.scriptVars) return "scriptVars";
        if (tethers != o.tethers) return "tethers";
        if (slideshow != o.slideshow) return "slideshow";
        if (saveSuppress != o.saveSuppress) return "saveSuppress";
        if (companion != o.companion) return "companion";
        if (released != o.released) return "barrierReleased";
        if (sayOnce != o.sayOnce) return "sayOnce";
        if (completed != o.completed) return "completed";
        if (failed != o.failed) return "failed";
        if (allObj != o.allObj) return "allObjectives";
        if (locks != o.locks) {
            std::string r = "locks mine:";
            for (auto& [k, v] : locks) { auto it = o.locks.find(k); if (it == o.locks.end() || it->second != v) r += " " + std::to_string(k.localId) + "=" + (v.first ? "L" : "u") + std::to_string(v.second); }
            r += " oracle:";
            for (auto& [k, v] : o.locks) { auto it = locks.find(k); if (it == locks.end() || it->second != v) r += " " + std::to_string(k.localId) + "=" + (v.first ? "L" : "u") + std::to_string(v.second); }
            return r;
        }
        if (factionRank != o.factionRank) return "factionRank";
        if (textures != o.textures) return "textures";
        if (fastTravel != o.fastTravel) return "fastTravel";
        if (perks != o.perks) return "perks";
        if (controlsOff != o.controlsOff) return "controls";
        if (spawned != o.spawned || despawned != o.despawned) return "spawned";   // createdCounter is the host's allocation bookkeeping: a client that undid a spawn keeps the higher one
        if (objects != o.objects) return "objects";
        if (refVars != o.refVars) return "refVars";
        if (refInv != o.refInv) {
            auto show = [](const RefInv& v) { std::string t = v.cleared ? "[cleared" : "["; for (auto& [i, d] : v.delta) t += " " + std::to_string(i.localId) + "=" + std::to_string(d); return t + "]"; };
            std::string r = "refInv";
            for (auto& [k, v] : refInv) { auto it = o.refInv.find(k); if (it == o.refInv.end() || !(it->second == v)) r += " " + std::to_string(k.localId) + " mine" + show(v) + " oracle" + (it == o.refInv.end() ? "[none]" : show(it->second)); }
            for (auto& [k, v] : o.refInv) if (!refInv.count(k)) r += " " + std::to_string(k.localId) + " mine[none] oracle" + show(v);
            return r;
        }
        if (topics != o.topics) {
            std::string r = "topics mine-only:";
            for (auto& k : topics) if (!o.topics.count(k)) r += " " + std::to_string(k.localId);
            r += " oracle-only:"; for (auto& k : o.topics) if (!topics.count(k)) r += " " + std::to_string(k.localId);
            return r;
        }
        if (notes != o.notes) {
            std::string r = "notes mine:";
            for (auto& k : notes) if (!o.notes.count(k)) r += " " + std::to_string(k.localId);
            r += " oracle:"; for (auto& k : o.notes) if (!notes.count(k)) r += " " + std::to_string(k.localId);
            return r;
        }
        if (markers != o.markers) return "markers";
        if (compKind != o.compKind) {
            std::string r = "companionKind mine:";
            for (auto& [k, v] : compKind) { auto it = o.compKind.find(k); if (it == o.compKind.end() || it->second != v) r += " " + std::to_string(k.localId) + "=" + std::to_string(v); }
            r += " oracle:";
            for (auto& [k, v] : o.compKind) { auto it = compKind.find(k); if (it == compKind.end() || it->second != v) r += " " + std::to_string(k.localId) + "=" + std::to_string(v); }
            r += " companion mine:"; for (auto& [k, v] : companion) r += " " + std::to_string(k.localId);
            r += " oracle:"; for (auto& [k, v] : o.companion) r += " " + std::to_string(k.localId);
            return r;
        }
        if (weatherSet != o.weatherSet || (weatherSet && (!(weather == o.weather) || weatherForced != o.weatherForced))) return "weather";
        if (timeScale != o.timeScale) return "timeScale";
        if (angles != o.angles || angleOrig != o.angleOrig) return "angles";
        if (hasActive != o.hasActive || (hasActive && !(active == o.active))) return "activeQuest mine " + std::to_string(hasActive) + ":" + std::to_string(active.localId) + " oracle " + std::to_string(o.hasActive) + ":" + std::to_string(o.active.localId);
        return "";
    }
};

template <class K> void bumpZ(std::map<K, int64_t>& m, const K& k, int64_t d) {
    auto& v = m[k];
    v += d;
    if (v == 0) m.erase(k);
}

// Round-3 state transitions, written once for the sink view and once more inside the oracle's feed (same simple semantics).
void objState(WorldView& w, const ObjectState& e) {
    ObjState o;
    if (auto it = w.objects.find(e.ref); it != w.objects.end()) o = it->second;
    if (e.mask & 1) o.open = e.open;
    if ((e.mask & 4) && e.destroyed) o.destroyed = true;
    if ((e.mask & 8) && e.activatedOnce) o.activated = true;
    if (o.isDefault()) w.objects.erase(e.ref); else w.objects[e.ref] = o;
    if (e.mask & 2) foldLock(w.locks, w.origins, e.ref, e.locked, e.level, e.hasBefore, e.beforeLocked, e.beforeLevel);
}
void invAdd(WorldView& w, const FormRef& ref, const FormRef& item, int64_t d) {
    RefInv& inv = w.refInv[ref];
    auto& v = inv.delta[item];
    v += d;
    if (v == 0) inv.delta.erase(item);
    if (inv.delta.empty() && !inv.cleared) w.refInv.erase(ref);
}
void spawn(WorldView& w, const SpawnRef& e) {
    if (w.despawned.count(e.ref())) return;
    w.createdCounter = std::max(w.createdCounter, e.id & 0x00FFFFFFu);
    w.spawned[e.ref()] = SpawnInfo{e.base, e.cell, e.x, e.y, e.z, e.leveled};
}

// Blind application of already-merged effects, plus the "engine hook" that records every mutation.
class SimWorld : public IEffectSink {
public:
    WorldView v;
    EffectCapture* cap = nullptr;
    std::set<std::pair<uint64_t, size_t>> seen;
    size_t duplicateApplications = 0;
    size_t applies = 0;

    std::vector<FormRef> ghostKills;  // kills applied from a remote txn: the engine would fire OnDeath scripts for them
    bool ghostThisTick = false;
    size_t eventsSeen = 0;

    void reset() { v = WorldView(); seen.clear(); ghostKills.clear(); }
    // What a guest would compute if it ran the quest script itself (locally derived value -> diverges from the host).
    void runQuestScriptLocally(uint64_t now) { v.scriptVars[{FormRef("falloutnv.esm", 0x100), 0}] = double(now % 1000); }

    void apply(const Effect& e, const Context& ctx) override {
        ++applies;
        if (ctx.seq != 0 && !seen.insert({ctx.seq | (uint64_t(ctx.epoch) << 40), ctx.index}).second) ++duplicateApplications;
        if (!ctx.fromSnapshot && !ctx.catchUp)
            if (auto* k = std::get_if<KillActor>(&e)) ghostKills.push_back(k->actor);
        if (ctx.baseline) {
            if (auto* o = std::get_if<SetObjective>(&e)) { v.objectives[{o->quest, o->objective}] = o->flags; return; }  // force-set
        }
        mutate(e);
    }
    // Local (engine-originated) mutation: applies and fires the capture hook.
    void local(const Effect& e) { mutate(e); }
    // A script running on this machine wants to mutate shared state. The hook asks the capture first: if the
    // capture blocks it (guest / remote event), the engine mutation does not proceed.
    void scriptEffect(const Effect& e) {
        if (cap && !cap->record(e)) return;
        std::visit([&](const auto& x) { do_(x); }, e);
    }

private:
    void mutate(const Effect& e) {
        std::visit([&](const auto& x) { do_(x); }, e);
        if (cap) cap->record(e);
    }
    void do_(const SetStage& e) { v.stages[e.quest] = e.stage; }
    void do_(const SetObjective& e) { v.objectives[{e.quest, e.objective}] = e.flags; }   // exact flag state (host-authoritative writer)
    void do_(const SetGlobal& e) { v.globals[e.global] = e.value; }
    void do_(const EnableRef& e) { v.enabled[e.ref] = true; }
    void do_(const DisableRef& e) { v.enabled[e.ref] = false; }
    void do_(const AddItem& e) { bumpZ(v.items, e.item, e.count); }
    void do_(const RemoveItem& e) { bumpZ(v.items, e.item, -static_cast<int64_t>(e.count)); }
    void do_(const ModActorValue& e) { bumpZ(v.av, e.av, e.delta); }
    void do_(const ModReputation& e) {
        auto& r = v.rep[e.faction];
        r.first += e.fame; r.second += e.infamy;
        if (r.first == 0 && r.second == 0) v.rep.erase(e.faction);
    }
    void do_(const ModKarma& e) { v.karma += e.delta; }
    void do_(const StartQuest& e) { v.running[e.quest] = true; }
    void do_(const StopQuest& e) { v.running[e.quest] = false; }
    void do_(const MoveRef& e) { v.placed[e.ref] = Placement{e.cell, e.x, e.y, e.z}; }
    void do_(const KillActor& e) { v.kills.insert(e.actor); }
    void do_(const ResurrectActor& e) { v.kills.erase(e.actor); }
    void do_(const SetScriptVar& e) { v.scriptVars[{e.owner, e.index}] = e.value; }
    void do_(const SetPlayerControls& e) { v.controlsOff = e.enable ? (v.controlsOff & ~e.controls) : (v.controlsOff | e.controls); }
    void do_(const PartyTether& e) { v.tethers[e.cell] = e.on; }
    void do_(const SlideshowEnd&) { v.slideshow = true; }
    void do_(const SaveSuppress& e) { v.saveSuppress = e.on; }
    void do_(const CompanionOwner& e) {
        if (e.active) { v.companion[e.npc] = e.owner; v.compKind[e.npc] = e.kind; } else { v.companion.erase(e.npc); v.compKind.erase(e.npc); }
    }
    void do_(const SpawnRef& e) { spawn(v, e); }
    void do_(const DespawnRef& e) { v.spawned.erase(e.ref()); if (e.forget) { v.despawned.erase(e.ref()); return; } v.despawned.insert(e.ref()); v.createdCounter = std::max(v.createdCounter, e.id & 0x00FFFFFFu); }
    void do_(const ObjectState& e) { objState(v, e); }
    void do_(const RefScriptVar& e) { foldRefVar(v.refVars, v.origins, e); }
    void do_(const RefAddItem& e) { if (e.count != 0) invAdd(v, e.ref, e.item, e.count); }
    void do_(const RefRemoveItem& e) { if (e.count != 0) invAdd(v, e.ref, e.item, -static_cast<int64_t>(e.count)); }
    void do_(const RemoveAllItems& e) { foldRefClear(v.refInv, e); }
    void do_(const AddTopic& e) { v.topics.insert(e.topic); }
    void do_(const AddNote& e) { v.notes.insert(e.note); }
    void do_(const RemoveNote& e) { v.notes.erase(e.note); }
    void do_(const SetMapMarker& e) { foldMarker(v.markers, v.origins, e); }
    void do_(const ForceActiveQuest& e) { v.active = e.quest; v.hasActive = true; }
    void do_(const RequestSave&) { ++eventsSeen; }
    void do_(const ContainerTransfer&) { ++eventsSeen; }   // per-machine stash move: no shared state in the model
    void do_(const SetWeather& e) { foldWeather(v.weather, v.weatherSet, v.weatherForced, v.origins, e); }
    void do_(const SetTimeScale& e) { foldTimeScale(v.timeScale, v.origins, e); }
    void do_(const RefAngle& e) { foldAngle(v.angles, v.angleOrig, e); }
    void do_(const AdvanceTime&) { ++eventsSeen; }
    void do_(const RefCommand&) { ++eventsSeen; }   // the generator never emits it: synccore/tests/test_refcmd.cpp owns the RefCommand state machine
    void do_(const ForcedDialogue&) { ++eventsSeen; }
    void do_(const BarrierArrive&) {}
    void do_(const BarrierRelease& e) { v.released.insert(e.barrier); }
    void do_(const SayOnceFlag& e) { v.sayOnce.insert(e.info); }
    void do_(const CompleteQuest& e) { v.completed.insert(e.quest); }
    void do_(const FailQuest& e) { v.completed.insert(e.quest); v.failed.insert(e.quest); }
    void do_(const CompleteAllObjectives& e) { v.allObj.insert(e.quest); }
    void do_(const SetLock& e) { foldLock(v.locks, v.origins, e.ref, e.locked, e.level, e.hasBefore, e.beforeLocked, e.beforeLevel); }
    void do_(const SetFactionRank& e) { v.factionRank[{e.actor, e.faction}] = e.rank < 0 ? -1 : e.rank; }
    void do_(const SetPerk& e) { if (e.rank <= 0) v.perks.erase(e.perk); else v.perks[e.perk] = e.rank; }
    void do_(const SwapTexture& e) { v.textures[{e.ref, e.slot}] = e.texture; }
    void do_(const SetFastTravelLock& e) { if (e.on) v.fastTravel.insert(e.scope); else v.fastTravel.erase(e.scope); }
};

// Independent reference implementation of the semantics, fed from the host's committed txns.
// Deliberately written without sharing code with ShadowState.
class Oracle {
public:
    void feed(const EffectTxn& t) {
        for (const auto& e : t.effects) {
            if (auto* x = std::get_if<SetStage>(&e)) {
                auto it = shared.stages.find(x->quest);
                if (it == shared.stages.end()) shared.stages[x->quest] = x->stage;
                else it->second = std::max(it->second, x->stage);
            } else if (auto* x = std::get_if<SetObjective>(&e)) {
                auto& f = shared.objectives[{x->quest, x->objective}];   // host writes are the exact state, guest writes only add bits
                if (t.origin == kHostPlayer) f = x->flags; else f |= x->flags;
            } else if (auto* x = std::get_if<SetGlobal>(&e)) {
                shared.globals[x->global] = x->value;
            } else if (auto* x = std::get_if<EnableRef>(&e)) {
                shared.enabled[x->ref] = true;
            } else if (auto* x = std::get_if<DisableRef>(&e)) {
                shared.enabled[x->ref] = false;
            } else if (auto* x = std::get_if<StartQuest>(&e)) {
                shared.running[x->quest] = true;
            } else if (auto* x = std::get_if<StopQuest>(&e)) {
                shared.running[x->quest] = false;
            } else if (auto* x = std::get_if<MoveRef>(&e)) {
                shared.placed[x->ref] = Placement{x->cell, x->x, x->y, x->z};
            } else if (auto* x = std::get_if<KillActor>(&e)) {
                shared.kills.insert(x->actor);
            } else if (auto* x = std::get_if<ResurrectActor>(&e)) {
                shared.kills.erase(x->actor);
            } else if (auto* x = std::get_if<SetScriptVar>(&e)) {
                shared.scriptVars[{x->owner, x->index}] = x->value;
            } else if (auto* x = std::get_if<SetPlayerControls>(&e)) {
                for (int s = 0; s < kMaxSlots; ++s) if (x->to.includes(PlayerId(s)))
                    perCtl[s] = x->enable ? (perCtl[s] & ~x->controls) : (perCtl[s] | x->controls);
            } else if (auto* x = std::get_if<PartyTether>(&e)) {
                shared.tethers[x->cell] = x->on;
            } else if (std::get_if<SlideshowEnd>(&e)) {
                shared.slideshow = true;
            } else if (auto* x = std::get_if<SaveSuppress>(&e)) {
                shared.saveSuppress = x->on;
            } else if (auto* x = std::get_if<CompanionOwner>(&e)) {
                if (x->active) { shared.companion[x->npc] = x->owner; shared.compKind[x->npc] = x->kind; }
                else { shared.companion.erase(x->npc); shared.compKind.erase(x->npc); }
            } else if (auto* x = std::get_if<SpawnRef>(&e)) {
                spawn(shared, *x);
            } else if (auto* x = std::get_if<DespawnRef>(&e)) {
                shared.spawned.erase(x->ref());
                if (x->forget) { shared.despawned.erase(x->ref()); continue; }
                shared.despawned.insert(x->ref());
                shared.createdCounter = std::max(shared.createdCounter, x->id & 0x00FFFFFFu);
            } else if (auto* x = std::get_if<ObjectState>(&e)) {
                objState(shared, *x);
            } else if (auto* x = std::get_if<RefScriptVar>(&e)) {
                foldRefVar(shared.refVars, shared.origins, *x);
            } else if (auto* x = std::get_if<RefAddItem>(&e)) {
                if (x->count != 0) invAdd(shared, x->ref, x->item, x->count);
            } else if (auto* x = std::get_if<RefRemoveItem>(&e)) {
                if (x->count != 0) invAdd(shared, x->ref, x->item, -static_cast<int64_t>(x->count));
            } else if (auto* x = std::get_if<RemoveAllItems>(&e)) {
                foldRefClear(shared.refInv, *x);
            } else if (auto* x = std::get_if<AddTopic>(&e)) {
                shared.topics.insert(x->topic);
            } else if (auto* x = std::get_if<AddNote>(&e)) {
                shared.notes.insert(x->note);
            } else if (auto* x = std::get_if<RemoveNote>(&e)) {
                shared.notes.erase(x->note);
            } else if (auto* x = std::get_if<SetMapMarker>(&e)) {
                foldMarker(shared.markers, shared.origins, *x);
            } else if (auto* x = std::get_if<ForceActiveQuest>(&e)) {
                for (int s = 0; s < kMaxSlots; ++s) if (x->to.includes(PlayerId(s))) { per[s].active = x->quest; per[s].hasActive = true; }
            } else if (auto* x = std::get_if<SetWeather>(&e)) {
                foldWeather(shared.weather, shared.weatherSet, shared.weatherForced, shared.origins, *x);
            } else if (auto* x = std::get_if<SetTimeScale>(&e)) {
                foldTimeScale(shared.timeScale, shared.origins, *x);
            } else if (auto* x = std::get_if<RefAngle>(&e)) {
                foldAngle(shared.angles, shared.angleOrig, *x);
            } else if (auto* x = std::get_if<BarrierArrive>(&e)) {
                auto& b = bar[x->barrier];
                b.required |= x->required;
                b.arrived |= uint8_t(1u << t.origin);
                if (b.required && (b.arrived & b.required) == b.required) shared.released.insert(x->barrier);
            } else if (auto* x = std::get_if<BarrierRelease>(&e)) {
                shared.released.insert(x->barrier);
            } else if (auto* x = std::get_if<SayOnceFlag>(&e)) {
                shared.sayOnce.insert(x->info);
            } else if (auto* x = std::get_if<CompleteQuest>(&e)) {
                shared.completed.insert(x->quest);
            } else if (auto* x = std::get_if<FailQuest>(&e)) {
                shared.completed.insert(x->quest); shared.failed.insert(x->quest);
            } else if (auto* x = std::get_if<CompleteAllObjectives>(&e)) {
                shared.allObj.insert(x->quest);
            } else if (auto* x = std::get_if<SetLock>(&e)) {
                foldLock(shared.locks, shared.origins, x->ref, x->locked, x->level, x->hasBefore, x->beforeLocked, x->beforeLevel);
            } else if (auto* x = std::get_if<SetFactionRank>(&e)) {
                shared.factionRank[{x->actor, x->faction}] = x->rank < 0 ? -1 : x->rank;
            } else if (auto* x = std::get_if<SwapTexture>(&e)) {
                shared.textures[{x->ref, x->slot}] = x->texture;
            } else if (auto* x = std::get_if<SetFastTravelLock>(&e)) {
                if (x->on) shared.fastTravel.insert(x->scope); else shared.fastTravel.erase(x->scope);
            } else if (auto* x = std::get_if<SetPerk>(&e)) {
                for (int s = 0; s < kMaxSlots; ++s) if (x->to.includes(PlayerId(s))) {
                    if (x->rank <= 0) per[s].perks.erase(x->perk); else per[s].perks[x->perk] = x->rank;
                }
            } else if (auto* x = std::get_if<AddItem>(&e)) {
                for (int s = 0; s < kMaxSlots; ++s) if (x->to.includes(PlayerId(s))) bumpZ(per[s].items, x->item, x->count);
            } else if (auto* x = std::get_if<RemoveItem>(&e)) {
                for (int s = 0; s < kMaxSlots; ++s) if (x->to.includes(PlayerId(s))) bumpZ(per[s].items, x->item, -int64_t(x->count));
            } else if (auto* x = std::get_if<ModActorValue>(&e)) {
                for (int s = 0; s < kMaxSlots; ++s) if (x->to.includes(PlayerId(s))) bumpZ(per[s].av, x->av, x->delta);
            } else if (auto* x = std::get_if<ModKarma>(&e)) {
                for (int s = 0; s < kMaxSlots; ++s) if (x->to.includes(PlayerId(s))) per[s].karma += x->delta;
            } else if (auto* x = std::get_if<ModReputation>(&e)) {
                for (int s = 0; s < kMaxSlots; ++s) if (x->to.includes(PlayerId(s))) {
                    auto& r = per[s].rep[x->faction];
                    r.first += x->fame; r.second += x->infamy;
                    if (r.first == 0 && r.second == 0) per[s].rep.erase(x->faction);
                }
            }
        }
    }
    // The host loaded an older save: the SHARED world rewinds, per-player state does not (clients keep their inventories).
    // Per-player values are taken from the live client worlds at this instant; shared karma/reputation rewind with the world.
    void rewind(const Oracle& saved, const std::vector<std::pair<int, WorldView>>& clients, const SharePolicy& p) {
        WorldView keepReleased;
        shared = saved.shared;
        bar = saved.bar;
        for (int s = 0; s < kMaxSlots; ++s) { per[s] = saved.per[s]; perCtl[s] = saved.perCtl[s]; }
        for (auto& [s, v] : clients) {
            per[s].items = v.items; per[s].av = v.av; per[s].perks = v.perks; perCtl[s] = v.controlsOff;
            per[s].active = v.active; per[s].hasActive = v.hasActive;
            if (!p.karma) per[s].karma = v.karma;
            if (!p.reputation) per[s].rep = v.rep;
        }
    }
    void seed(const WorldView& base) {
        shared = base;
        for (int s = 0; s < kMaxSlots; ++s) { per[s].karma = base.karma; per[s].rep = base.rep; }
        shared.karma = 0; shared.rep.clear();
    }
    WorldView view(int slot) const {
        WorldView w = shared;
        w.items = per[slot].items; w.av = per[slot].av; w.karma = per[slot].karma; w.rep = per[slot].rep;
        w.perks = per[slot].perks; w.controlsOff = perCtl[slot];
        w.active = per[slot].active; w.hasActive = per[slot].hasActive;
        return w;
    }
private:
    WorldView shared;
    WorldView per[kMaxSlots];
    uint32_t perCtl[kMaxSlots] = {};
    struct Bar { uint8_t arrived = 0, required = 0; };
    std::map<FormRef, Bar> bar;
};

WorldView viewOf(const ShadowState& s, int slot) {
    WorldView w;
    w.stages = s.stages; w.objectives = s.objectives; w.globals = s.globals; w.enabled = s.enabled;
    w.kills = s.kills; w.running = s.questRunning; w.placed = s.placed;
    w.items = s.ledger[slot].items; w.av = s.ledger[slot].av; w.karma = s.ledger[slot].karma; w.rep = s.ledger[slot].rep;
    w.perks = s.ledger[slot].perks; w.controlsOff = s.ledger[slot].controlsOff;
    w.scriptVars = s.scriptVars; w.tethers = s.tethers; w.slideshow = s.slideshowEnded; w.saveSuppress = s.saveSuppress;
    w.companion = s.companionOwner; w.sayOnce = s.sayOnce; w.completed = s.questCompleted; w.failed = s.questFailed; w.allObj = s.allObjectivesDone;
    w.locks = s.locks; w.factionRank = s.factionRank; w.textures = s.textures;
    w.spawned = s.spawned; w.despawned = s.despawned; w.createdCounter = s.createdCounter; w.objects = s.objects; w.refVars = s.refVars;
    w.refInv = s.refInv; w.topics = s.topics; w.notes = s.notes; w.markers = s.mapMarkers; w.compKind = s.companionKind;
    w.weather = s.weather; w.weatherSet = s.weatherSet; w.weatherForced = s.weatherForced; w.timeScale = s.timeScale; w.angles = s.angles; w.angleOrig = s.angleOrig; w.origins = s.origins;
    w.active = s.ledger[slot].activeQuest; w.hasActive = s.ledger[slot].hasActive;
    for (auto& [k, on] : s.fastTravelLock) if (on) w.fastTravel.insert(k);
    for (auto& [k, b] : s.barriers) if (b.released) w.released.insert(k);
    return w;
}

// ---- random content ---------------------------------------------------------------------------

FormRef pick(Rng& r, const char* const* plugins, size_t np, uint32_t base, uint32_t n) {
    return FormRef(plugins[r.range(0, uint32_t(np - 1))], base + r.range(0, n - 1));
}

Recipient pickRecipient(Rng& r, int maxPlayers) {
    uint32_t k = r.range(0, 99);
    if (k < 60) return Recipient::dflt();
    if (k < 75) return Recipient::origin();
    if (k < 90) return Recipient::all();
    return Recipient::slots(uint8_t(r.range(1, (1u << maxPlayers) - 1)));
}

// Pristine values, a pure function of the key, as the engine would report them before any script touched it.
void pristineLock(const FormRef& ref, bool& locked, int32_t& level) { locked = ref.localId % 3 == 0; level = locked ? int32_t((ref.localId * 11) % 100) : 0; }
double pristineVar(const FormRef& ref, uint16_t idx) { return double((ref.localId * 5 + idx * 3) % 7) * 0.5; }

Effect genEffect(Rng& r, int maxPlayers) {
    static const char* const plugins[] = {"falloutnv.esm", "deadmoney.esm"};
    auto form = [&](uint32_t base, uint32_t n) { return pick(r, plugins, 2, base, n); };
    auto created = [&] { return kCreatedRefPrefix | r.range(1, 8); };
    switch (r.range(0, 57)) {
        case 0: case 1: case 2: return SetStage{form(0x100, 6), r.range(0, 100)};
        case 3: case 4: return SetObjective{form(0x100, 6), r.range(0, 3), uint8_t(r.range(0, 3))};
        case 5: return SetGlobal{form(0x200, 4), float(r.range(0, 20)) * 0.5f};
        case 6: return EnableRef{form(0x300, 6)};
        case 7: return DisableRef{form(0x300, 6)};
        case 8: case 9: return AddItem{form(0x400, 5), int32_t(r.range(1, 5)), pickRecipient(r, maxPlayers)};
        case 10: return RemoveItem{form(0x400, 5), int32_t(r.range(1, 3)), pickRecipient(r, maxPlayers)};
        case 11: return ModActorValue{r.range(0, 4), int32_t(r.range(0, 25)) - 5, pickRecipient(r, maxPlayers)};
        case 12: return ModReputation{form(0x500, 3), int32_t(r.range(0, 6)) - 2, int32_t(r.range(0, 4)) - 1, pickRecipient(r, maxPlayers)};
        case 13: return ModKarma{int32_t(r.range(0, 20)) - 10, pickRecipient(r, maxPlayers)};
        case 14: return StartQuest{form(0x100, 6)};
        case 15: return StopQuest{form(0x100, 6)};
        case 16: return MoveRef{form(0x300, 6), form(0x600, 2), float(r.range(0, 100)), float(r.range(0, 100)), 0.f, r.chance(0.2)};
        case 17: case 18: return KillActor{form(0x700, 6)};
        case 19: return ResurrectActor{form(0x700, 6)};
        case 20: case 21: return SetScriptVar{form(0x100, 6), uint16_t(r.range(0, 5)), double(r.range(0, 40)) * 0.25};
        case 22: return SetPlayerControls{1u << r.range(0, 5), r.chance(0.5), pickRecipient(r, maxPlayers)};
        case 23: return PartyTether{form(0x600, 2), r.chance(0.5)};
        case 24: return SlideshowEnd{};
        case 25: return SaveSuppress{r.chance(0.5)};
        case 26: return CompanionOwner{form(0x700, 6), uint8_t(r.range(0, maxPlayers - 1)), r.chance(0.8), uint8_t(r.range(0, 1))};
        case 27: return ForcedDialogue{form(0x700, 6), form(0x800, 4), uint8_t(r.range(0, maxPlayers - 1))};
        case 28: case 29: return BarrierArrive{form(0x900, 2), uint8_t(r.range(1, (1u << maxPlayers) - 1))};
        case 30: return BarrierRelease{form(0x900, 2)};
        case 31: return SayOnceFlag{form(0xA00, 8)};
        case 32: return CompleteQuest{form(0x100, 6)};
        case 33: return CompleteAllObjectives{form(0x100, 6)};
        case 34: { SetLock l{form(0x300, 6), r.chance(0.5), int32_t(r.range(0, 100))}; pristineLock(l.ref, l.beforeLocked, l.beforeLevel); l.hasBefore = true; return l; }
        case 35: return SetFactionRank{form(0x700, 6), form(0x500, 3), int32_t(r.range(0, 6)) - 1};
        case 36: return SetPerk{form(0xB00, 4), int32_t(r.range(0, 3)), pickRecipient(r, maxPlayers)};
        case 37: case 38: return SwapTexture{form(0x300, 6), form(0xC00, 3), uint8_t(r.range(0, 2))};
        case 40: return SetFastTravelLock{form(0x100, 6), r.chance(0.6)};
        case 41: return SpawnRef{created(), form(0x1000, 4), form(0x600, 2), float(r.range(0, 50)), float(r.range(0, 50)), 0.f, r.chance(0.3)};
        case 42: return DespawnRef{created()};
        case 43: { ObjectState o; o.ref = form(0x300, 6); o.mask = uint8_t(r.range(1, 15)); o.open = r.chance(0.5); o.locked = r.chance(0.5);
                   o.level = int32_t(r.range(0, 100)); o.destroyed = r.chance(0.2); o.activatedOnce = r.chance(0.3);
                   pristineLock(o.ref, o.beforeLocked, o.beforeLevel); o.hasBefore = true; return o; }
        case 44: { RefScriptVar x{form(0x700, 6), uint16_t(r.range(0, 3)), double(r.range(0, 20)) * 0.5}; x.hasBefore = true; x.before = pristineVar(x.ref, x.index); return x; }
        case 45: return RefAddItem{form(0x1100, 4), form(0x400, 5), int32_t(r.range(1, 4))};
        case 46: return RefRemoveItem{form(0x1100, 4), form(0x400, 5), int32_t(r.range(1, 3))};
        case 47: {
            if (!r.chance(0.3)) return RefAddItem{form(0x1100, 4), form(0x400, 5), 1};
            RemoveAllItems c; c.ref = form(0x1100, 4); c.hasBefore = true;   // the contents are a pure function of the ref, as the engine would report them
            c.items = {{FormRef("falloutnv.esm", 0x400 + c.ref.localId % 5), int32_t(1 + c.ref.localId % 3)}, {FormRef("falloutnv.esm", 0x405), 2}};
            return c;
        }
        case 48: return AddTopic{form(0x1200, 6)};
        case 49: return AddNote{form(0x1300, 6)};
        case 50: return RemoveNote{form(0x1300, 6)};
        case 51: { SetMapMarker m{form(0x1400, 4), uint8_t(r.range(0, 3))}; m.hasBefore = true; m.before = uint8_t(m.marker.localId % 4); return m; }
        case 52: return ForceActiveQuest{form(0x100, 6), pickRecipient(r, maxPlayers)};
        case 53: return RequestSave{uint8_t(r.range(0, 2))};
        case 54: { SetWeather w{form(0x1500, 3), r.chance(0.4)}; w.hasBefore = true; w.before = FormRef("falloutnv.esm", 0x1500); w.beforeForce = false; return w; }
        case 55: { SetTimeScale t{float(r.range(1, 30))}; t.hasBefore = true; t.before = 20.f; return t; }
        case 56: {   // the pre-image is the ref axis's pristine angle, a pure function of (ref, axis), as the engine would report it
            FormRef ref = form(0x300, 6);
            uint8_t axis = uint8_t(r.range(0, 2));
            return RefAngle{ref, axis, float(r.range(0, 36)) * 10.f, true, float((ref.localId * 7 + axis * 13) % 37) * 10.f};
        }
        default: return AdvanceTime{r.range(1, 600)};
    }
}

Cause genCause(Rng& r) {
    uint32_t k = r.range(0, 99);
    Cause c;
    c.kind = k < 50 ? CauseKind::QuestResult : k < 65 ? CauseKind::DialogueResult : k < 75 ? CauseKind::Console
           : k < 85 ? CauseKind::Loot : CauseKind::Script;
    c.source = FormRef("falloutnv.esm", 0x100 + r.range(0, 5));
    c.aux = int32_t(r.range(0, 100));
    return c;
}


// Effects kept out of host-load scenarios (a load rewinds the host to an older save; clients must come back to that state), and why.
// Everything NOT listed here is in the scenario and reverts through a pre-image or an exact diff:
//   SetWeather, SetTimeScale, RefScriptVar, SetMapMarker, SetLock and the lock bit of ObjectState (Origin pre-images in ShadowState::origins),
//   the open bit of ObjectState (a plain two-way bit), RefAddItem / RefRemoveItem (signed deltas), RemoveAllItems (RefInv::pre, undone by an
//   undo RemoveAllItems), SpawnRef / DespawnRef (a forget DespawnRef drops a spawn or a tombstone the loaded save never had), RefAngle.
//  - BarrierArrive / BarrierRelease: transient rendezvous state (who has arrived at a rendezvous right now). It has no baseline entry and is not
//    in the digest, and a rendezvous belongs to the live session rather than to a save game, so a load has nothing to revert.
//  - SayOnceFlag: a SayOnce topic flag is one-way in the engine (no command clears "said once"), so a client that set it cannot be reverted.
//  - CompleteQuest / FailQuest / CompleteAllObjectives: one-way in the engine (a completed or failed quest cannot be un-completed by script).
//  - SlideshowEnd: one-way in the engine (a played slideshow cannot be un-played).
//  - AddTopic: one-way in the engine (there is no RemoveTopic).
//  - RefCommand: never generated here (synccore/tests/test_refcmd.cpp covers it). Its state ops carry no pre-image, so a host load cannot take one
//    back from a client (the same limit as the one-way effects above); events have no state.
//  - ObjectState destroyed / activated-once bits: sticky in the engine (the generator strips them below, the open and lock bits are kept).
bool hostLoadExcluded(Effect& e) {
    if (std::holds_alternative<BarrierArrive>(e) || std::holds_alternative<BarrierRelease>(e) || std::holds_alternative<SayOnceFlag>(e) ||
        std::holds_alternative<CompleteQuest>(e) || std::holds_alternative<FailQuest>(e) || std::holds_alternative<CompleteAllObjectives>(e) ||
        std::holds_alternative<SlideshowEnd>(e) || std::holds_alternative<AddTopic>(e))
        return true;
    if (auto* o = std::get_if<ObjectState>(&e)) {
        o->mask &= uint8_t(~(kObjDestroyed | kObjActivated));
        o->destroyed = o->activatedOnce = false;
        if (o->mask == 0) return true;
    }
    const size_t i = e.index();
    (void)i;
    return false;
}
bool hasPreimage(const Effect& e) {
    return std::holds_alternative<SetWeather>(e) || std::holds_alternative<SetTimeScale>(e) || std::holds_alternative<RefScriptVar>(e) ||
           std::holds_alternative<SetMapMarker>(e) || std::holds_alternative<SetLock>(e) || std::holds_alternative<ObjectState>(e) ||
           std::holds_alternative<RefAddItem>(e) || std::holds_alternative<RefRemoveItem>(e) || std::holds_alternative<RemoveAllItems>(e) ||
           std::holds_alternative<SpawnRef>(e) || std::holds_alternative<DespawnRef>(e);
}

bool baselineKind(const Effect& e) {
    static const bool ok[] = {1,1,1,1,1,0,0,0,0,0,1,1,1,1,0,1,0,1,1,1,1,0,0,0,1,1,1,1,1,0,1, 0,0,0,0,1,1,0,0,0,1,1,0,1,0,0,1,1,0};
    return e.index() < sizeof ok && ok[e.index()];
}

ShadowState genBaseline(Rng& r, int maxPlayers) {
    ShadowState s;
    EffectTxn t;
    t.origin = 0;
    for (int i = 0; i < 400; ++i) { Effect e = genEffect(r, maxPlayers); if (baselineKind(e)) t.effects.push_back(e); }
    t.seq = 1;
    s.applyTxn(t);
    s.seq = 0;
    // Complete key space, like a real engine dump: every quest/var/global/ref exists with its default value even if never touched.
    // (Absent-vs-default is not representable in a baseline, and a rewound host world must still know every key.)
    for (const char* pl : {"falloutnv.esm", "deadmoney.esm"}) {
        for (uint32_t q = 0x100; q < 0x106; ++q) {
            FormRef f(pl, q);
            s.stages.emplace(f, 0u);
            s.questRunning.emplace(f, false);
            for (uint32_t o = 0; o < 4; ++o) s.objectives.emplace(std::make_pair(f, o), uint8_t(0));
            for (uint16_t i = 0; i < 6; ++i) s.scriptVars.emplace(std::make_pair(f, i), 0.0);
        }
        for (uint32_t g = 0x200; g < 0x204; ++g) s.globals.emplace(FormRef(pl, g), 0.f);
        for (uint32_t r2 = 0x300; r2 < 0x306; ++r2) {
            FormRef f(pl, r2);
            s.enabled.emplace(f, true);
            s.placed.emplace(f, Placement{FormRef("falloutnv.esm", 0x600), 0.f, 0.f, 0.f});
            s.locks.emplace(f, std::make_pair(false, 0));
            for (uint8_t sl = 0; sl < 3; ++sl) s.textures.emplace(std::make_pair(f, sl), FormRef("falloutnv.esm", 0xC00));
        }
        for (uint32_t c = 0x600; c < 0x602; ++c) s.tethers.emplace(FormRef(pl, c), false);
        for (uint32_t a = 0x700; a < 0x706; ++a) {
            s.companionOwner.emplace(FormRef(pl, a), uint8_t(0));
            s.companionKind.emplace(FormRef(pl, a), uint8_t(0));
            for (const char* pl2 : {"falloutnv.esm", "deadmoney.esm"})
                for (uint32_t fa = 0x500; fa < 0x503; ++fa) s.factionRank.emplace(std::make_pair(FormRef(pl, a), FormRef(pl2, fa)), -1);
        }
    }
    for (uint32_t q : {0xD00u, 0xD01u, 0xD02u, 0x180u, 0x181u, 0x182u, 0x183u}) s.stages.emplace(FormRef("falloutnv.esm", q), 0u);
    s.ledger[0].karma = int64_t(r.range(0, 200)) - 100;
    for (uint32_t f = 0; f < 3; ++f) s.ledger[0].rep[FormRef("falloutnv.esm", 0x500 + f)] = {int64_t(r.range(1, 50)), int64_t(r.range(1, 50))};
    return s;
}

template <class M, class F> void wobble(Rng& r, M& m, F change) {
    for (auto it = m.begin(); it != m.end();) {
        if (r.chance(0.1)) { it = m.erase(it); continue; }
        if (r.chance(0.3)) change(it->second);
        ++it;
    }
}

// A client's own save: same key space as the host, different values, a few keys missing, extra kills.
ShadowState diverge(const ShadowState& h, Rng& r) {
    ShadowState c = h;
    wobble(r, c.stages, [&](uint32_t& v) { v = r.range(0, 100); });
    wobble(r, c.objectives, [&](uint8_t& v) { v = uint8_t(r.range(0, 3)); });
    wobble(r, c.globals, [&](float& v) { v = float(r.range(0, 20)) * 0.5f; });
    wobble(r, c.enabled, [&](bool& v) { v = !v; });
    wobble(r, c.scriptVars, [&](double& v) { v = double(r.range(0, 40)) * 0.25; });
    wobble(r, c.questRunning, [&](bool& v) { v = !v; });
    wobble(r, c.placed, [&](Placement& p) { p.x += 5; p.cell = FormRef("falloutnv.esm", 0x600 + r.range(0, 1)); });
    wobble(r, c.locks, [&](std::pair<bool, int32_t>& v) { v = {!v.first, v.first ? 0 : 33}; });
    wobble(r, c.textures, [&](FormRef& f) { f = FormRef("falloutnv.esm", 0xC00 + r.range(0, 2)); });
    wobble(r, c.factionRank, [&](int32_t& v) { v = int32_t(r.range(0, 5)) - 1; });
    wobble(r, c.companionOwner, [&](uint8_t& v) { v = uint8_t(r.range(0, 3)); });
    for (auto it = c.companionKind.begin(); it != c.companionKind.end();) { if (!c.companionOwner.count(it->first)) it = c.companionKind.erase(it); else ++it; }
    for (auto& kv : c.companionOwner) c.companionKind.emplace(kv.first, uint8_t(0));
    wobble(r, c.tethers, [&](bool& v) { v = !v; });
    for (auto* set : {&c.kills, &c.sayOnce, &c.questCompleted, &c.allObjectivesDone})
        for (auto it = set->begin(); it != set->end();) { if (r.chance(0.3)) it = set->erase(it); else ++it; }
    for (uint32_t i = 0; i < 3; ++i) if (r.chance(0.5)) c.kills.insert(FormRef("falloutnv.esm", 0x7A0 + i));  // client-only dead NPCs
    c.slideshowEnded = h.slideshowEnded && r.chance(0.7);
    c.saveSuppress = r.chance(0.5);
    c.ledger[0] = Ledger();
    c.ledger[0].karma = int64_t(r.range(0, 200)) - 100;
    for (uint32_t f = 0; f < 4; ++f) if (r.chance(0.6)) c.ledger[0].rep[FormRef("falloutnv.esm", 0x500 + f)] = {int64_t(r.range(1, 50)), int64_t(r.range(1, 50))};
    return c;
}

ShadowState dumpOf(const WorldView& v) {
    ShadowState s;
    s.stages = v.stages; s.objectives = v.objectives; s.globals = v.globals; s.enabled = v.enabled; s.kills = v.kills;
    s.questRunning = v.running; s.placed = v.placed; s.scriptVars = v.scriptVars; s.tethers = v.tethers;
    s.slideshowEnded = v.slideshow; s.saveSuppress = v.saveSuppress; s.companionOwner = v.companion; s.sayOnce = v.sayOnce;
    s.questCompleted = v.completed; s.questFailed = v.failed; s.allObjectivesDone = v.allObj; s.locks = v.locks; s.factionRank = v.factionRank;
    s.textures = v.textures;
    s.companionKind = v.compKind; s.refVars = v.refVars; s.topics = v.topics; s.notes = v.notes; s.mapMarkers = v.markers;
    for (auto& [r, o] : v.objects) s.objects[r] = o;
    s.weather = v.weather; s.weatherSet = v.weatherSet; s.weatherForced = v.weatherForced; s.timeScale = v.timeScale; s.angles = v.angles; s.angleOrig = v.angleOrig; s.origins = v.origins; s.refInv = v.refInv; s.spawned = v.spawned; s.despawned = v.despawned; s.createdCounter = v.createdCounter;
    for (auto& k : v.fastTravel) s.fastTravelLock[k] = true;
    s.ledger[0].karma = v.karma;
    for (auto& [f, r] : v.rep) s.ledger[0].rep[f] = r;
    return s;
}

WorldView baselineView(const ShadowState& b, const SharePolicy& p) {
    WorldView w = viewOf(b, 0);
    w.items.clear(); w.av.clear(); w.perks.clear(); w.controlsOff = 0; w.karma = 0; w.rep.clear();
    if (p.karma) w.karma = b.ledger[0].karma;
    if (p.reputation) w.rep = b.ledger[0].rep;
    w.released.clear(); w.hasActive = false; w.active = FormRef();
    w.spawned.clear(); w.despawned.clear(); w.createdCounter = 0; w.refInv.clear();   // not baseline state: they reach a joiner by snapshot
    return w;
}

// Silent client-side corruption (a missed engine apply, a stray local script write): changes shared state without any txn.
bool corruptWorld(WorldView& v, Rng& r) {
    auto pick = [&](auto& m) { return std::next(m.begin(), r.range(0, uint32_t(m.size() - 1))); };
    switch (r.range(0, 6)) {
        case 0: if (v.stages.empty()) return false; pick(v.stages)->second += 1 + r.range(0, 5); return true;
        case 1: if (v.globals.empty()) return false; pick(v.globals)->second += 1.f; return true;
        case 2: if (v.scriptVars.empty()) return false; pick(v.scriptVars)->second += 2.0; return true;
        case 3: if (v.enabled.empty()) return false; { auto it = pick(v.enabled); it->second = !it->second; } return true;
        case 4:
            if (r.chance(0.5) || v.kills.empty()) { v.kills.insert(FormRef("falloutnv.esm", 0x7C0 + r.range(0, 3))); return true; }
            v.kills.erase(pick(v.kills));
            return true;
        case 5: if (v.objectives.empty()) return false; pick(v.objectives)->second ^= 1; return true;
        default: if (v.placed.empty()) return false; pick(v.placed)->second.x += 1.f; return true;
    }
}

struct SimClient {
    PeerId peer = 0;
    uint64_t token = 0;
    SimWorld world;
    EffectCapture cap;
    std::unique_ptr<ClientSession> session;
    uint32_t sessionCounter = 0;
    bool wantConnected = false;
    uint64_t reconnectAt = 0;
};

}  // namespace

Effect randomEffect(Rng& r, int maxPlayers) { return genEffect(r, maxPlayers); }
Cause randomCause(Rng& r) { return genCause(r); }

SimConfig configForSeed(uint64_t seed) {
    Rng r(seed * 7919 + 13);
    SimConfig c;
    c.seed = seed;
    c.clients = int(r.range(1, 5));
    c.durationMs = r.range(2000, 8000);
    uint32_t mn = r.range(0, 40);
    c.link.minLatencyMs = mn;
    c.link.maxLatencyMs = mn + r.range(0, 200);
    c.link.loss = r.range(0, 30) / 100.0;
    c.link.duplicate = r.range(0, 20) / 100.0;
    c.link.corrupt = r.range(0, 5) / 100.0;
    c.txnPerTick = 0.01 + r.range(0, 8) / 100.0;
    c.actionPerTick = r.range(0, 3) / 100.0;
    c.disconnectPerTick = r.range(0, 4) / 1000.0;
    c.freshRestartProb = r.range(0, 50) / 100.0;
    c.compactPerTick = r.range(0, 3) / 1000.0;
    c.varWritesPerTick = (seed % 3 == 0) ? 10 : 0;
    c.baselineJoin = (seed % 4 == 1);
    c.hostLoads = c.baselineJoin && seed % 8 == 1;
    c.desync = (seed % 5 == 2) && !(c.baselineJoin && (seed % 8 == 1 || seed % 8 == 5));
    c.clientLoads = c.baselineJoin && seed % 8 == 5;
    // A rewind discards the txn history, so rewards a restarted client would have replayed from it are gone (documented);
    // keep restarts out of the host-load scenario so the per-player oracle stays exact.
    if (c.hostLoads) c.freshRestartProb = 0;
    if (c.clientLoads && seed % 3 == 0) c.policy.clientLoad = SharePolicy::ClientLoad::Block;
    c.actionRequestPerTick = r.range(0, 3) / 100.0;
    c.policy.karma = r.chance(0.8);
    c.policy.reputation = r.chance(0.8);
    c.policy.questRewardsToAll = r.chance(0.8);
    c.policy.maxPlayers = uint8_t(c.clients + 1 + r.range(0, 2));
    if (c.policy.maxPlayers > kMaxSlots) c.policy.maxPlayers = kMaxSlots;
    return c;
}

SimResult runSim(const SimConfig& cfg) {
    SimResult res;
    auto violate = [&](const std::string& s) {
        res.ok = false;
        ++res.violationCount;
        if (res.violations.size() < 8) res.violations.push_back(s);
    };

    Rng rng(cfg.seed ^ 0xA5A5A5A5ull);
    InMemoryNetwork net(cfg.seed + 101);
    net.setLink(cfg.link);
    SharePolicy policy = cfg.policy;
    const int maxPlayers = policy.maxPlayers;
    HostSession host(*net.endpoint(0), policy);
    Oracle oracle;
    // Host engine: a guest-owned NPC died (reported as an intent). The HOST runs the OnDeath script and is the only
    // machine that commits its result.
    StateReplicator rep(host);
    std::set<std::tuple<int, uint32_t, uint32_t>> executedActions;
    host.onAction = [&](PlayerId slot, IntentKind, const FormRef& ref, uint32_t epoch, uint32_t id) {
        if (!executedActions.insert({int(slot), epoch, id}).second)
            violate("action executed twice: slot " + std::to_string(slot) + " epoch " + std::to_string(epoch) + " id " + std::to_string(id));
        // The host runs the real activation with its capture open and commits what it did.
        Cause ac; ac.kind = CauseKind::Script; ac.source = ref;
        host.commit(slot, ac, {SetStage{FormRef("falloutnv.esm", 0xD00 + ref.localId % 3), 10 + id % 5},
                               AddItem{FormRef("falloutnv.esm", 0x490), 1, Recipient::dflt()}});
    };
    std::vector<FormRef> hostDeaths;
    uint64_t deathScriptsCommitted = 0;
    host.onCommit = [&](const EffectTxn& t) {
        oracle.feed(t);
        if (t.flags & kTxnOriginApplied)
            for (auto& e : t.effects) if (auto* k = std::get_if<KillActor>(&e)) hostDeaths.push_back(k->actor);
    };
    auto deathEffects = [](const FormRef& actor) {
        return std::vector<Effect>{SetStage{FormRef("falloutnv.esm", 0x180 + actor.localId % 4), 50},
                                   AddItem{FormRef("falloutnv.esm", 0x480), 1, Recipient::dflt()}};
    };

    std::vector<std::unique_ptr<SimClient>> cl;
    for (int i = 0; i < cfg.clients; ++i) {
        auto c = std::make_unique<SimClient>();
        c->peer = PeerId(i + 1);
        c->token = 0xC0DE0000ull + uint64_t(i) * 17 + cfg.seed * 1000003ull;
        c->world.cap = &c->cap;
        c->session = std::make_unique<ClientSession>(*net.endpoint(c->peer), 0, c->token, c->world, &c->cap);
        cl.push_back(std::move(c));
    }

    uint64_t now = 0;
    auto connect = [&](SimClient& c) {
        net.setConnected(c.peer, true);
        c.session->connect(now, c.session->lastSession() + 1);
        c.wantConnected = true;
    };
    auto drop = [&](SimClient& c) {
        net.setConnected(c.peer, false);
        c.session->disconnect();
        c.wantConnected = false;
    };
    auto step = [&]() {
        for (auto& actor : hostDeaths) {
            Cause dc; dc.kind = CauseKind::QuestResult; dc.source = FormRef("falloutnv.esm", 0x180); dc.aux = 50;
            host.commit(kHostPlayer, dc, deathEffects(actor));
            ++deathScriptsCommitted;
        }
        hostDeaths.clear();
        now += cfg.tickMs;
        rep.flush(now);
        net.advance(now);
        host.tick(now);
        for (auto& c : cl) {
            if (c->wantConnected) c->session->tick(now);
            // Ghost death: the guest engine fires OnDeath scripts for NPCs killed by another machine. Those
            // script effects must be blocked on guests; only the host's txn may change the story.
            if (questScriptsMayRun(Role::Guest)) c->world.runQuestScriptLocally(now);
            c->world.ghostThisTick = !c->world.ghostKills.empty();
            for (auto& actor : c->world.ghostKills)
                for (auto& e : deathEffects(actor)) c->world.scriptEffect(e);
            c->world.ghostKills.clear();
            // Echo monitor: after a tick no unsolicited local mutation may be pending.
            auto e = c->cap.take();
            if (!e.empty()) {
                violate((c->world.ghostThisTick ? "ghost: guest produced death-script effect " : "echo: remote apply re-emitted ") +
                        std::string(effectName(e[0])) + " on client " + std::to_string(c->peer));
                // Forward a bounded number of echoes so the damage (duplicate commits) is visible, without an endless storm.
                if (res.violationCount < 20) {
                    Cause cz; cz.kind = CauseKind::Intent;
                    c->session->submitIntent(cz, std::move(e));
                }
            }
        }
        net.advance(now);
    };

    // ---- optional: join-in-progress. Everyone starts from their own divergent save. ----
    ShadowState hostBase;
    std::vector<ShadowState> clientBase(cl.size());
    const uint64_t kContent = 0xC0FFEE;
    if (cfg.baselineJoin) {
        hostBase = genBaseline(rng, maxPlayers);
        host.setWorldBaseline(hostBase, kContent);
        oracle.seed(baselineView(hostBase, policy));
        for (size_t i = 0; i < cl.size(); ++i) {
            clientBase[i] = diverge(hostBase, rng);
            cl[i]->world.v = baselineView(clientBase[i], policy);
            cl[i]->session->setBaselineWorld(clientBase[i], kContent);
            cl[i]->wantConnected = false;
            cl[i]->reconnectAt = rng.range(0, cfg.durationMs / 2);   // late joiners
        }
    } else {
        for (auto& c : cl) connect(*c);
    }

    if (cfg.baselineJoin) {
        for (auto& c : cl) {
            SimClient* cp = c.get();
            // a rebase / unusable-sidecar load re-dumps the client's CURRENT world
            c->session->setBaselineProvider([cp, kContent](ShadowState& w, uint64_t& h) { w = dumpOf(cp->world.v); h = kContent; return true; });
        }
    }
    if (cfg.desync) {
        HostSession::DesyncOptions dop;
        dop.minDigestIntervalMs = 300;
        host.setDesyncOptions(dop);
        for (auto& c : cl) {
            SimClient* cp = c.get();
            c->session->setBaselineProvider([cp, kContent](ShadowState& w, uint64_t& h) { w = dumpOf(cp->world.v); h = kContent; return true; });
            c->session->setDigestInterval(cfg.digestMs);
        }
    }
    struct HostSave { Bytes side; Oracle oracle; };
    std::vector<HostSave> hostSaves;
    struct ClientSave { Bytes side; WorldView world; };
    std::vector<std::vector<ClientSave>> clientSaves(cl.size());
    uint64_t lastSaveAt = 0;

    // ---- chaos phase ----
    for (uint64_t t = 0; t < cfg.durationMs; t += cfg.tickMs) {
        if (rng.chance(cfg.txnPerTick)) {
            std::vector<Effect> eff;
            for (uint32_t i = 0, n = rng.range(1, 6); i < n; ++i) {
                Effect e = genEffect(rng, maxPlayers);
                if (cfg.hostLoads && hostLoadExcluded(e)) continue;
                if (cfg.hostLoads && hostSaves.empty() && hasPreimage(e)) continue;   // state that first appears after a save is the case a host load must undo
                eff.push_back(std::move(e));
            }
            if (eff.empty()) eff.push_back(SetStage{FormRef("falloutnv.esm", 0x100), 1});
            host.commit(PlayerId(rng.range(0, maxPlayers - 1)), genCause(rng), std::move(eff));
        }
        for (int w = 0; w < cfg.varWritesPerTick; ++w) {  // per-frame quest-script churn on the host
            ++res.varWrites;
            if (rng.chance(0.2)) rep.setGlobal(FormRef("falloutnv.esm", 0x200 + rng.range(0, 3)), float(rng.range(0, 2000)) * 0.25f);
            else rep.setScriptVar(FormRef("falloutnv.esm", 0x100 + rng.range(0, 3)), uint16_t(rng.range(0, 3)), double(rng.range(0, 2000)) * 0.25);
        }
        if (cfg.baselineJoin && now >= lastSaveAt + 400) {   // periodic saves (host and clients) with their sidecars
            lastSaveAt = now;
            if (host.hasWorldBaseline() && cfg.hostLoads) {
                hostSaves.push_back({host.saveSidecar(), oracle});
                if (hostSaves.size() > 6) hostSaves.erase(hostSaves.begin());
            }
            if (cfg.clientLoads)
                for (size_t i = 0; i < cl.size(); ++i)
                    if (cl[i]->wantConnected && cl[i]->session->canSaveState()) {
                        clientSaves[i].push_back({cl[i]->session->saveState(), cl[i]->world.v});
                        if (clientSaves[i].size() > 4) clientSaves[i].erase(clientSaves[i].begin());
                    }
        }
        if (cfg.hostLoads && !hostSaves.empty() && rng.chance(cfg.hostLoadPerTick)) {
            const HostSave& hs = hostSaves[rng.range(0, uint32_t(hostSaves.size() - 1))];
            std::vector<std::pair<int, WorldView>> views;
            for (auto& c : cl) if (c->session->slot() != kNoPlayer) views.emplace_back(int(c->session->slot()), c->world.v);
            if (host.loadSidecar(hs.side)) {
                oracle.rewind(hs.oracle, views, policy);
                rep.clear();
                hostDeaths.clear();
                ++res.hostLoads;
            } else {
                violate("host sidecar failed to load");
            }
        }
        if (cfg.clientLoads)
            for (size_t i = 0; i < cl.size(); ++i) {
                auto& c = cl[i];
                if (clientSaves[i].empty() || !c->wantConnected || !rng.chance(cfg.clientLoadPerTick)) continue;
                const ClientSave& cs = clientSaves[i][rng.range(0, uint32_t(clientSaves[i].size() - 1))];
                if (!c->session->clientLoadAllowed()) { ++res.clientLoadsBlocked; continue; }   // the engine refuses the load
                c->world.v = cs.world;
                c->world.seen.clear();
                c->world.ghostKills.clear();
                c->session->onLocalSaveLoaded(now, &cs.side);
                ++res.clientLoads;
            }
        if (rng.chance(cfg.compactPerTick)) host.mutableLog().compact(rng.range(0, 30));
        for (auto& c : cl) {
            if (c->wantConnected && rng.chance(cfg.disconnectPerTick)) {
                drop(*c);
                c->reconnectAt = now + rng.range(100, 2000);
                ++res.disconnects;
            } else if (!c->wantConnected && now >= c->reconnectAt) {
                if (rng.chance(cfg.freshRestartProb)) {
                    c->session->resetState();
                    c->world.reset();
                    if (cfg.baselineJoin) {  // a restart reloads the player's own save, then joins by baseline again
                        size_t ci = size_t(c->peer) - 1;
                        c->world.v = baselineView(clientBase[ci], policy);
                        c->session->setBaselineWorld(clientBase[ci], kContent);
                    }
                    ++res.freshRestarts;
                }
                connect(*c);
            }
            if (cfg.desync && c->session->joined() && !c->session->baselineActive() && !c->session->intentsPending() &&
                rng.chance(cfg.corruptPerTick) && corruptWorld(c->world.v, rng))
                ++res.injected;
            if (c->session->joined() && !c->session->baselineActive() && rng.chance(cfg.actionRequestPerTick))
                c->session->submitAction(IntentKind(rng.range(1, 3)), FormRef("falloutnv.esm", 0xE00 + rng.range(0, 5)));
            if (c->session->joined() && !c->session->baselineActive() && rng.chance(cfg.actionPerTick)) {
                // A local engine action: mutates the local world, hook captures it, session reports it.
                uint32_t k = rng.range(0, 2);
                Effect e = k == 0 ? Effect(AddItem{FormRef("falloutnv.esm", 0x400 + rng.range(0, 4)), int32_t(rng.range(1, 3)), Recipient::dflt()})
                         : k == 1 ? Effect(ModActorValue{rng.range(0, 4), int32_t(rng.range(1, 9)), Recipient::dflt()})
                                  : Effect(KillActor{FormRef("falloutnv.esm", 0x700 + rng.range(0, 5))});
                {
                    LocalAction la(&c->cap);  // a genuine local player action on a guest
                    c->world.local(e);
                }
                Cause cz; cz.kind = CauseKind::Loot;
                c->session->submitIntent(cz, c->cap.take());
                ++res.intents;
            }
        }
        step();
    }

    // ---- heal and settle ----
    LinkParams calm;
    calm.minLatencyMs = 1; calm.maxLatencyMs = 5;
    net.setLink(calm);
    for (auto& c : cl) if (!c->wantConnected) connect(*c);
    auto clientWorldDiff = [&](SimClient& c) -> std::string {
        int slot = c.session->slot();
        if (slot == kNoPlayer) return "no slot";
        return c.world.v.diff(oracle.view(slot));
    };
    auto quiescent = [&]() {
        if (cfg.desync)
            for (auto& c : cl) if (!clientWorldDiff(*c).empty()) return false;
        for (auto& c : cl)
            if (!c->session->joined() || c->session->applied() != host.log().head() || c->session->intentsPending() ||
                c->session->baselineActive() || c->session->rebaseRequired() || (cfg.baselineJoin && c->session->worldEpoch() == 0))
                return false;
        for (auto& v : host.clients()) if (v.joined && v.baselineJobOpen) return false;   // the host still waits for a final baseline ack
        return host.allSynced() && (cfg.desync || net.idle()) && rep.pending() == 0;   // digests keep the link busy
    };
    bool settled = false;
    for (uint64_t t = 0; t < 120000 && !settled; t += cfg.tickMs) {
        step();
        settled = quiescent();
    }
    for (int i = 0; i < 20; ++i) step();  // let any stray echo/dup surface
    if (!settled || !quiescent()) {
        std::string why;
        for (auto& c : cl)
            why += " [c" + std::to_string(c->peer) + " joined=" + std::to_string(c->session->joined()) + " applied=" +
                   std::to_string(c->session->applied()) + " intents=" + std::to_string(c->session->intentsPending()) + " bActive=" + std::to_string(c->session->baselineActive()) + " bDone=" + std::to_string(c->session->baselineDone()) + " rej=" + std::to_string(c->session->rejected()) + "]";
        for (auto& v : host.clients()) why += " {tok=" + std::to_string(v.token & 0xFFFF) + " slot=" + std::to_string(v.slot) + " peer=" + std::to_string(v.peer) + " joined=" + std::to_string(v.joined) + " acked=" + std::to_string(v.acked) + " job=" + std::to_string(v.baselineJobOpen) + "}";
        violate("no quiescence within 120s of healed link; head=" + std::to_string(host.log().head()) +
                " synced=" + std::to_string(host.allSynced()) + " netIdle=" + std::to_string(net.idle()) + " inFlight=" + std::to_string(net.inFlight()) + " baselineRetx=" + std::to_string(host.stats().baselineRetransmits) + " bulkDeferred=" + std::to_string(host.stats().bulkDeferred) + " welcomes=" + std::to_string(host.stats().welcomes) + " throttled=" + std::to_string(host.stats().welcomesThrottled) + " retxBatches=" + std::to_string(host.stats().retransmitBatches) + why);
    }

    // ---- invariants ----
    for (int s = 0; s < maxPlayers; ++s) {
        WorldView want = oracle.view(s);
        WorldView got = viewOf(host.state(), s);
        if (cfg.baselineJoin) { got.karma = want.karma; got.rep = want.rep; }  // absolutes are checked on the clients
        if (cfg.hostLoads) { got.items = want.items; got.av = want.av; got.perks = want.perks; got.controlsOff = want.controlsOff; got.active = want.active; got.hasActive = want.hasActive; }  // per-player state is not rewound: the host's ledger restarted
        std::string d = got.diff(want);
        if (!d.empty()) violate("host shadow != oracle for slot " + std::to_string(s) + " (" + d + ")");
    }
    for (auto& c : cl) {
        if (c->session->rejected() || c->session->slot() == kNoPlayer) { violate("client never got a slot"); continue; }
        int slot = c->session->slot();
        if (c->session->worldEpoch() != host.epoch() && cfg.baselineJoin)
            violate("client " + std::to_string(c->peer) + " on world epoch " + std::to_string(c->session->worldEpoch()) + " but host is on " + std::to_string(host.epoch()));
        res.rebases += c->session->stats().rebases;
        res.staleEpochDropped += c->session->stats().staleEpochDropped;
        if (c->session->state() != host.state())
            violate("client " + std::to_string(c->peer) + " shadow != host (" + c->session->state().describeDifference(host.state()) + ")");
        WorldView ov = oracle.view(slot), cw = c->world.v;
        if (cfg.hostLoads) {   // per-player state is never rewound or re-sent; old-epoch txns still in flight at a load make it inexact
            cw.items = ov.items; cw.av = ov.av; cw.perks = ov.perks; cw.controlsOff = ov.controlsOff; cw.active = ov.active; cw.hasActive = ov.hasActive;
            if (!policy.karma) cw.karma = ov.karma;
            if (!policy.reputation) cw.rep = ov.rep;
        }
        std::string d = cw.diff(ov);
        if (!d.empty()) violate("client " + std::to_string(c->peer) + " world != oracle (" + d + ")");
        if (c->world.duplicateApplications)
            violate("duplicate application on client " + std::to_string(c->peer) + " x" + std::to_string(c->world.duplicateApplications));
        res.duplicatesDropped += c->session->stats().duplicatesDropped;
        res.snapshots += c->session->stats().snapshots;
        res.replays += c->session->stats().replays;
    }
    res.digestsMismatch = host.stats().digestsMismatch;
    res.desyncRepairs = host.stats().desyncRepairs;
    res.desyncReports = host.desyncs().size();
    res.baselineJoins = host.stats().baselineJoins;
    res.baselineEffects = host.stats().baselineEffectsSent;
    res.baselineReverts = host.stats().baselineReverts;
    res.baselineChunks = host.stats().baselineChunksSent;
    res.deathScripts = deathScriptsCommitted;
    res.varEffects = rep.stats().effectsCommitted;
    res.actionsExecuted = host.stats().actionsExecuted;
    res.commits = host.stats().commits;
    res.retransmitBatches = host.stats().retransmitBatches;
    res.msgsSent = net.stats().sent; res.msgsLost = net.stats().lost;
    res.msgsDup = net.stats().duplicated; res.msgsCorrupt = net.stats().corrupted;
    res.virtualMs = now;
    return res;
}

}  // namespace fnvcoop::sim
