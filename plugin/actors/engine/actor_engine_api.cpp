#include "actors/engine/actor_engine_api.h"

#include "actors/engine/actor_feed.h"
#include "actors/engine/engine_port.h"
#include "actors/engine/registry.h"
#include "fnvcoop_mut.h"

namespace nvcoop::actors::engine {

Registry& registry() {
    static Registry r;
    return r;
}

namespace {

OwnerRef toOwnerRef(const OwnerInfo& o) {
    OwnerRef r;
    r.known = o.known;
    r.netId = o.netId;
    r.ownerSlot = o.owner;
    r.epoch = o.epoch;
    r.mine = o.known && o.owner == o.self && o.owner != kNoPlayer;
    r.pending = o.pending;
    r.standIn = o.standIn;
    r.dead = o.dead;
    return r;
}

}  // namespace

OwnerRef ownerOfNet(NetId id) {
    Registry& reg = registry();
    ActorSync* s = reg.sync ? reg.sync() : nullptr;
    if (!s || !id) return OwnerRef{};
    return toOwnerRef(s->ownerOf(id));
}

NetId netIdOf(RefId ref) {
    Registry& reg = registry();
    return reg.netOfRef ? reg.netOfRef(ref) : 0;
}

RefId refOf(NetId id) {
    Registry& reg = registry();
    return reg.refOfNet ? reg.refOfNet(id) : 0;
}

bool isStandIn(RefId ref) {
    Registry& reg = registry();
    return reg.isStandInRef && reg.isStandInRef(ref);
}

OwnerRef ownerOf(RefId ref) {
    const NetId id = netIdOf(ref);
    if (!id) return OwnerRef{};
    OwnerRef o = ownerOfNet(id);
    if (o.known && isStandIn(ref)) o.standIn = true;
    return o;
}

bool isRemoteDriven(RefId ref) {
    const OwnerRef o = ownerOf(ref);
    if (!o.known) return false;
    if (o.standIn) return true;
    if (FNV_MUT(61)) return !o.mine;   // BAD: a handoff in flight and an actor nobody owns yet count as "remote" / "not remote" wrongly
    if (o.pending) return true;
    return o.ownerSlot != kNoPlayer && !o.mine;
}

HitRouting routeLocalHit(const LocalHitInfo& hit) {
    HitRouting r;
    r.victim = ownerOf(hit.victim);
    r.attackerNet = netIdOf(hit.attacker);
    Registry& reg = registry();
    if (!r.attackerNet && hit.attacker && reg.playerNetByRef) r.attackerNet = reg.playerNetByRef(hit.attacker);
    if (!r.victim.known) { r.route = HitRoute::Untracked; return r; }
    if (r.victim.standIn || (r.victim.ownerSlot != kNoPlayer && !r.victim.mine && !r.victim.pending)) r.route = HitRoute::ForwardToOwner;
    else if (r.victim.mine && !r.victim.pending) r.route = HitRoute::LocalOwner;
    else r.route = HitRoute::NoOwner;
    return r;
}

void setHitForwarder(HitForwarder f) { registry().hitForwarder = std::move(f); }

bool onLocalHit(const LocalHitInfo& hit) {
    const HitRouting r = routeLocalHit(hit);
    if (r.route == HitRoute::Untracked) return false;
    if (!FNV_MUT(62) && r.route != HitRoute::ForwardToOwner) return false;   // twin 62: every tracked hit is forwarded, even the ones this machine resolves
    Registry& reg = registry();
    if (!reg.hitForwarder) return false;   // nobody to hand it to: the caller keeps the local damage
    reg.hitForwarder(hit, r);
    return true;
}

bool onLocalDeath(RefId ref) {
    Registry& reg = registry();
    if (FNV_MUT(63)) return true;   // BAD: never asks the ownership rules
    if (!reg.localDeath) return true;
    return reg.localDeath(ref);
}

void removeListener(ListenerId id) { registry().remove(id); }
ListenerId addActorTrackedListener(std::function<void(const TrackedEvent&)> fn) { return registry().addTracked(std::move(fn)); }
ListenerId addActorDiedListener(std::function<void(const DiedEvent&)> fn) { return registry().addDied(std::move(fn)); }
ListenerId addActorLostListener(std::function<void(const LostEvent&)> fn) { return registry().addLost(std::move(fn)); }
ListenerId addDriveListener(std::function<void(const DriveEvent&)> fn) { return registry().addDrive(std::move(fn)); }
void setAnimSampler(AnimSampler f) { registry().animSampler = std::move(f); }

void setPlayerNetIdResolver(PlayerNetIdResolver byRef, PlayerRefResolver byNet) {
    registry().playerNetByRef = std::move(byRef);
    registry().playerRefByNet = std::move(byNet);
}

void setKillerResolver(KillerResolver f) { registry().killerOf = std::move(f); }

void setPlayerPose(PlayerId slot, const PlayerPose& pose) {
    Registry& reg = registry();
    ActorSync* s = reg.sync ? reg.sync() : nullptr;
    if (s) s->setPlayerPose(slot, pose);
}

void setRefExclusion(RefFilter f) { registry().exclude = std::move(f); }
void setCreatedIdResolver(CreatedIdResolver f) { registry().createdId = std::move(f); }
void setInventoryAccess(InventoryReader read, InventoryWriter write) { registry().invRead = std::move(read); registry().invWrite = std::move(write); }

void onEffect(const fnvcoop::Effect& e) {
    Registry& reg = registry();
    ActorSync* s = reg.sync ? reg.sync() : nullptr;
    if (s) s->onEffect(e);
}

void beforeSave() {
    Registry& reg = registry();
    if (reg.beforeSave) reg.beforeSave();
}

void configure(const Config& c) { registry().cfg = c; ++registry().cfgVersion; }
const Config& config() { return registry().cfg; }

PortStats portStats() {
    Registry& reg = registry();
    return reg.stats ? reg.stats() : PortStats{};
}

IActorEngine* actorEngine() { return registry().engine; }

void wireApi(EnginePort& port, ActorFeed& feed, IActorEngine& eng, std::function<ActorSync*()> sync) {
    Registry& reg = registry();
    reg.sync = std::move(sync);
    reg.engine = &eng;
    reg.netOfRef = [&port, &feed](RefId ref) { NetId n = port.netOfRef(ref); return n ? n : feed.netOfRef(ref); };
    reg.refOfNet = [&port](NetId id) { return port.activeRef(id); };
    reg.isStandInRef = [&port](RefId ref) { return port.isStandInRef(ref); };
    reg.localDeath = [&feed](RefId ref) { return feed.onLocalDeathRef(ref); };
    reg.beforeSave = [&feed]() { feed.beforeSave(); };
    reg.stats = [&port, &feed, &eng]() {
        PortStats s = port.stats();
        const FeedStats& f = feed.stats();
        s.scans = f.scans; s.trackedNew = f.trackedNew; s.trackedGone = f.trackedGone;
        s.deathsPolled = f.deathsPolled; s.deathsReportedToSync = f.deathsReported; s.deathsRefused = f.deathsRefused;
        s.resyncs = f.sessions + f.worldChanges;
        s.consoleLines = eng.linesRun(); s.consoleFailed = eng.linesFailed();
        return s;
    };
}

}  // namespace nvcoop::actors::engine
