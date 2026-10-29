// Machine: ownership glue, story effects, loot / containers, spawn-site arbitration, digests and repair, full state.
#include <algorithm>
#include "../../live/entity.h"
#include <cmath>
#include <set>
#include "fnvcoop/engine/machine.h"
#include "fnvcoop/wire.h"

namespace fnvcoop::engine {

// ---- ownership glue --------------------------------------------------------------------------

std::vector<Presence> Machine::presenceFor(const ActorState& a) const {
    std::vector<Presence> who;
    for (uint8_t s = 0; s < kMaxSlots; ++s) {
        const PlayerState& p = world_.player(s);
        if (!p.present) continue;
        Presence pr;
        pr.player = s; pr.connected = true;
        pr.inCell = p.cell == a.cell;
        pr.adjacent = world_.adjacent(p.cell, a.cell);
        pr.distance = distance(p.pos, a.pos);
        who.push_back(pr);
    }
    return who;
}

void Machine::emitOwn(const std::vector<OwnMsg>& out) {
    for (const OwnMsg& m : out) {
        const ApplyResult r = view_.apply(m);
        if (r == ApplyResult::Applied) trackOwnerChange(m);
        if (obs_) obs_->onOwnership(cfg_.self, now_, m, r);
        sendBody(MOwn{m}, kNoPlayer);   // before the hooks: a Transfer must reach the new owner ahead of the old owner's final state
        onOwnApplied(m, r);
    }
}

// Hysteresis oracle (host): while an actor's owner is still eligible, it is kept for minHoldMs after the last change. Only the follower exception
// (a recruited companion goes to its recruiter at once) may hand it over sooner.
void Machine::trackOwnerChange(const OwnMsg& m) {
    if (m.kind == OwnMsg::Transfer) {
        const ActorState* a = world_.findActor(m.netId);
        auto at = ownChangedAt_.find(m.netId);
        if (!a || at == ownChangedAt_.end() || m.prevOwner == kNoPlayer || a->recruiter != kNoPlayer || now_ >= at->second + cfg_.own.minHoldMs) return;
        for (const Presence& p : presenceFor(*a))
            if (p.player == m.prevOwner && p.eligible())
                breach("owner_flapped", "actor " + std::to_string(m.netId) + " was taken from p" + std::to_string(m.prevOwner) + " " + std::to_string(now_ - at->second) + " ms after its last change while p" +
                       std::to_string(m.prevOwner) + " was still eligible (hold " + std::to_string(cfg_.own.minHoldMs) + " ms)");
    } else if (m.kind == OwnMsg::Grant || m.kind == OwnMsg::Commit || m.kind == OwnMsg::Release) {
        ownChangedAt_[m.netId] = now_;
    }
}

void Machine::onOwnApplied(const OwnMsg& m, ApplyResult r) {
    if (r != ApplyResult::Applied) return;
    ActorState* a = world_.findActor(m.netId);
    if (m.kind == OwnMsg::Transfer) {
        if (m.prevOwner == cfg_.self && a) {   // we are giving this actor away: stop its AI, send our last state
            handingOff_.insert(m.netId);
            a->aiOn = false;
            publishActor(*a, false, true);
        }
        return;
    }
    const bool sawFinal = acked_.count(m.netId) != 0;   // the old owner's last state reached us before the commit
    acked_.erase(m.netId);
    // Any ownership change ends an earlier wait for a previous owner's final state, with one exception: a Grant that gives the actor back to
    // us (it cancels our own outgoing handoff, or restates our ownership) changes nothing about who may still be driving it. The old owner has
    // not necessarily heard of the first handoff yet (slow link), so we keep waiting for its final state until the safety timeout.
    std::pair<PlayerId, uint64_t> keptWait{kNoPlayer, 0};
    bool hadWait = false;
    if (m.kind == OwnMsg::Grant && m.owner == cfg_.self && !engineFaults().grantEndsOwnerWait) {
        auto w = awaitOld_.find(m.netId);
        if (w != awaitOld_.end()) { keptWait = w->second; hadWait = true; }
    }
    awaitOld_.erase(m.netId);
    if (hadWait) awaitOld_[m.netId] = keptWait;
    if (m.kind == OwnMsg::Ack) return;
    // Sequence numbers must order across owners: a new owner that never saw the old owner's last snapshot (forced commit, lost
    // final) would otherwise reuse a sequence number and have its state ignored by the old owner.
    // (Epochs restart on a host load, so a sequence derived from the epoch cannot be used.)
    if (m.owner == cfg_.self && a) {
        a->snapSeq += 4096;
        lastMoveMs_[m.netId] = now_; settledAt_[m.netId] = 0;   // and re-announce our copy reliably once it settles: guests may hold an older one
    }
    // A forced commit (the host did not wait for the old owner) must not let two machines drive the actor: until the old owner's final state shows
    // that it has heard of the handoff and stopped, the new owner keeps its AI off.
    if (m.kind == OwnMsg::Commit && m.owner == cfg_.self && a && !sawFinal && !engineFaults().newOwnerNoWait &&
        m.prevOwner != kNoPlayer && m.prevOwner != cfg_.self && m.prevOwner < kMaxSlots && world_.player(m.prevOwner).present)
        awaitOld_[m.netId] = {m.prevOwner, now_};
    handingOff_.erase(m.netId);   // a Grant that cancels our own outgoing handoff, a plain take-over, or the handoff finishing
    refreshAi(m.netId);           // AI follows the view at once, not at the next tick: no window where two machines both drive it
}

void Machine::onOwn(const Envelope& e, const MOwn& m) {
    if (isHost()) {
        if (m.m.kind != OwnMsg::Ack) return;   // clients may only acknowledge
        std::vector<OwnMsg> out;
        if (svc_->onAck(m.m, e.src, static_cast<uint32_t>(now_), out)) ++stats_.acks;
        emitOwn(out);
        return;
    }
    const ApplyResult r = view_.apply(m.m);
    if (obs_) obs_->onOwnership(cfg_.self, now_, m.m, r);
    onOwnApplied(m.m, r);
}

void Machine::maybeAck(NetId id) {
    OwnerEpoch pe = 0;
    if (!view_.handoffIncoming(id, pe) || acked_.count(id)) return;
    acked_.insert(id);
    OwnMsg ack;
    ack.kind = OwnMsg::Ack; ack.netId = id; ack.epoch = pe; ack.owner = cfg_.self;
    if (isHost()) {
        std::vector<OwnMsg> out;
        if (svc_->onAck(ack, cfg_.self, static_cast<uint32_t>(now_), out)) ++stats_.acks;
        emitOwn(out);
    } else {
        sendBody(MOwn{ack}, kHostPlayer);
    }
}

void Machine::evalOwnership() {
    std::vector<OwnMsg> out;
    for (auto& kv : world_.actors()) {
        const ActorState& a = kv.second;
        if (a.dead || !a.enabled) continue;
        svc_->track(a.id, presenceFor(a), static_cast<uint32_t>(now_), out);
    }
    emitOwn(out);
}

std::vector<NetId> Machine::ownedActors() const {
    std::vector<NetId> r;
    view_.forEach([&](ActorNetId id, PlayerId owner, OwnerEpoch) { if (owner == cfg_.self) r.push_back(id); });
    return r;
}

// ---- story effects (existing Effect path) ----------------------------------------------------

bool Machine::commitEffects(const std::vector<Effect>& fx) {
    if (!isHost() || !synced_ || fx.empty()) return false;
    const uint64_t seq = ++fxSeq_;
    std::vector<OwnMsg> out;
    for (const Effect& e : fx) {
        world_.applyEffect(e);
        ++stats_.fxApplied;
        if (obs_) obs_->onEffectApplied(cfg_.self, now_, seq, e);
        if (auto* c = std::get_if<CompanionOwner>(&e)) {
            const NetId id = world_.actorByRef(c->npc);
            if (const ActorState* a = world_.findActor(id))
                svc_->setRecruiter(id, c->active ? c->owner : kNoPlayer, presenceFor(*a), static_cast<uint32_t>(now_), out);
        } else if (auto* k = std::get_if<KillActor>(&e)) {
            const NetId id = world_.actorByRef(k->actor);
            recordStoryDeath(id, true);
            if (id) onActorDiedHost(id);
        } else if (auto* r = std::get_if<ResurrectActor>(&e)) {
            recordStoryDeath(world_.actorByRef(r->actor), false);
        }
    }
    emitOwn(out);
    MEffects m;
    m.seq = seq; m.fx = fx;
    sendBody(m, kNoPlayer);
    return true;
}

void Machine::onEffects(const MEffects& m) {
    if (m.seq <= lastFxSeq_) {
        ++stats_.fxDuplicate;
        if (!engineFaults().noDedupe) return;
        ++stats_.doubleApplied;
    }
    for (const Effect& e : m.fx) {
        world_.applyEffect(e);
        ++stats_.fxApplied;
        if (obs_) obs_->onEffectApplied(cfg_.self, now_, m.seq, e);
        if (auto* k = std::get_if<KillActor>(&e)) recordStoryDeath(world_.actorByRef(k->actor), true);
        else if (auto* r = std::get_if<ResurrectActor>(&e)) recordStoryDeath(world_.actorByRef(r->actor), false);
    }
    lastFxSeq_ = std::max(lastFxSeq_, m.seq);
}

void Machine::onActorDiedHost(NetId id) {
    for (auto& kv : world_.sites()) {
        SpawnSiteState& s = kv.second;
        if (s.spawned != id) continue;
        s.spawned = 0; s.cleared = true; s.clearedAtMs = now_;
        MSiteState m;
        m.site = s.id; m.spawned = 0; m.cleared = true; m.clearedAtMs = s.clearedAtMs; m.spawnCount = s.spawnCount;
        sendBody(m, kNoPlayer);
    }
    std::vector<OwnMsg> out;
    svc_->untrack(id, out);
    emitOwn(out);
    handingOff_.erase(id);
}

// ---- loot, drops, pickups --------------------------------------------------------------------

void Machine::onDropReq(const Envelope& e, const MDropReq& m) {
    if (!isHost()) return;
    const uint64_t origin = (static_cast<uint64_t>(e.src) << 32) | m.reqId;
    if (!dropOrigins_.insert(origin).second) {
        if (!engineFaults().dropNoIdempotency && !engineFaults().noDedupe) { ++stats_.dropsDeduped; return; }
        ++stats_.doubleApplied;
    }
    ItemState it;
    it.id = world_.allocCreatedId(); it.base = m.base; it.cell = m.cell; it.pos = m.pos; it.origin = origin;
    world_.items()[it.id] = it;
    MItemCreated c;
    c.id = it.id; c.base = it.base; c.cell = it.cell; c.pos = it.pos; c.origin = origin;
    sendBody(c, kNoPlayer);
}

void Machine::onItemCreated(const MItemCreated& m) {
    ItemState it;
    it.id = m.id; it.base = m.base; it.cell = m.cell; it.pos = m.pos; it.origin = m.origin;
    world_.items()[it.id] = it;
    world_.noteCreatedId(it.id);
}

void Machine::onPickup(const Envelope& e, const MPickup& m) {
    if (!isHost()) return;
    auto it = world_.items().find(m.item);
    if (it == world_.items().end()) return;
    ItemState& item = it->second;
    if (!item.taken || engineFaults().pickupNoArbitration) {
        if (item.taken) ++stats_.doubleApplied;   // only the fault gets here: one world item granted to a second claimant (the oracle reports it like any id applied twice)
        item.taken = true; item.takenBy = e.src;
        ++stats_.pickupsWon;
        // (the fault: the grant goes to the claimant only, so the earlier winner is never told it lost anything)
        sendBody(MItemTaken{item.id, e.src, m.claimId}, engineFaults().pickupNoArbitration ? e.src : kNoPlayer);
    } else {
        ++stats_.pickupsLost;
        sendBody(MItemTaken{item.id, item.takenBy, m.claimId}, e.src);   // "already taken"
    }
}

void Machine::onItemTaken(const MItemTaken& m) {
    auto it = world_.items().find(m.item);
    if (it != world_.items().end()) { it->second.taken = true; it->second.takenBy = m.by; }
    if (m.by == cfg_.self) { pendingClaims_.erase(m.claimId); return; }
    for (auto p = pendingClaims_.begin(); p != pendingClaims_.end();) {
        if (p->second != m.item) { ++p; continue; }
        if (it != world_.items().end()) {
            if (engineFaults().noPickupRollback) {
                breach("pickup_loser_kept_item", "item " + std::to_string(m.item) + " went to p" + std::to_string(m.by) + " but this machine kept its optimistic copy");
            } else {
                auto& inv = world_.player(cfg_.self).inventory;
                auto q = inv.find(it->second.base);
                if (q != inv.end() && --q->second <= 0) inv.erase(q);
            }
        }
        ++stats_.rollbacks;
        p = pendingClaims_.erase(p);
    }
}

// ---- containers ------------------------------------------------------------------------------

void Machine::onContainerReq(const Envelope& e, const MContainerReq& m) {
    if (!isHost()) return;
    MContainerRes res;
    res.opId = m.opId; res.cont = m.cont; res.op = m.op; res.base = m.base; res.count = m.count;
    auto it = world_.containers().find(m.cont);
    auto reply = [&](const MContainerRes& r) { if (e.src == cfg_.self) onContainerRes(r); else sendBody(r, e.src); };
    if (it == world_.containers().end()) { reply(res); return; }
    ContainerState& c = it->second;
    auto dup = std::find(c.appliedOps.begin(), c.appliedOps.end(), m.opId);
    if (dup != c.appliedOps.end()) {
        if (!engineFaults().noDedupe) return;
        ++stats_.doubleApplied;
    } else {
        c.appliedOps.push_back(m.opId);
        if (c.appliedOps.size() > 128) c.appliedOps.erase(c.appliedOps.begin());
    }
    if (engineFaults().containerNoLock) {   // the fault: the lock is not checked. Say so when serving without it changes the answer.
        const bool heldByOther = c.inUseBy != kNoPlayer && c.inUseBy != e.src;
        const bool lockMissing = (m.op == ContainerOpKind::Open && heldByOther) || ((m.op == ContainerOpKind::Take || m.op == ContainerOpKind::Put) && c.inUseBy != e.src);
        if (lockMissing) breach("container_lock_ignored", "container " + std::to_string(c.id) + " is held by " + (c.inUseBy == kNoPlayer ? std::string("nobody") : "p" + std::to_string(c.inUseBy)) + " but p" + std::to_string(e.src) + " was served");
    }
    const bool lockOk = c.inUseBy == e.src || engineFaults().containerNoLock;
    bool changed = false;
    switch (m.op) {
        case ContainerOpKind::Open:
            res.ok = c.inUseBy == kNoPlayer || c.inUseBy == e.src || engineFaults().containerNoLock;
            if (res.ok) c.inUseBy = e.src;
            break;
        case ContainerOpKind::Close:
            res.ok = c.inUseBy == e.src;
            if (res.ok) {
                c.inUseBy = kNoPlayer;
                if (m.closeHash && m.closeHash != world_.keyHash(Category::Containers, c.id)) {   // anti-dupe: contents must agree on close
                    ++stats_.closeDigestMismatch;
                    pushRepair(e.src, Category::Containers, {c.id}, {});
                }
            }
            break;
        case ContainerOpKind::Take: {
            auto q = c.items.find(m.base);
            res.ok = lockOk && m.count > 0 && q != c.items.end() && q->second >= m.count;
            if (res.ok) { q->second -= m.count; changed = true; }
            break;
        }
        case ContainerOpKind::Put:
            res.ok = lockOk && m.count > 0;
            if (res.ok) { c.items[m.base] += m.count; changed = true; }
            break;
    }
    if (changed) {
        int32_t now = c.items[m.base];
        if (now <= 0) c.items.erase(m.base);
        sendBody(MContainerSync{c.id, m.base, std::max(now, 0)}, kNoPlayer);
    }
    if (res.ok) ++stats_.containerOps; else ++stats_.containerRefused;
    reply(res);
}

void Machine::onContainerSync(const MContainerSync& m) {
    auto it = world_.containers().find(m.cont);
    if (it == world_.containers().end()) return;
    if (m.newCount <= 0) it->second.items.erase(m.base);
    else it->second.items[m.base] = m.newCount;
}

void Machine::onContainerRes(const MContainerRes& m) {
    auto it = world_.containers().find(m.cont);
    auto& inv = world_.player(cfg_.self).inventory;
    switch (m.op) {
        case ContainerOpKind::Open: if (m.ok && it != world_.containers().end()) it->second.inUseBy = cfg_.self; break;
        case ContainerOpKind::Close: if (it != world_.containers().end() && it->second.inUseBy == cfg_.self) it->second.inUseBy = kNoPlayer; break;
        case ContainerOpKind::Take: if (m.ok) inv[m.base] += m.count; break;
        case ContainerOpKind::Put:
            if (pendingPuts_.erase(m.opId) && !m.ok) inv[m.base] += m.count;   // refused: give the item back
            break;
    }
}

// ---- spawns ----------------------------------------------------------------------------------

static bool siteFree(const SpawnSiteState& s, uint64_t now) {
    if (s.spawned != 0) return false;
    if (!s.cleared) return true;
    return !s.permanent && now >= s.clearedAtMs + s.respawnMs;
}

NetId Machine::hostSpawnAt(SpawnSiteState& site, PlayerId roller) {
    ActorState a;
    a.id = world_.allocCreatedId(); a.ref = createdRef(a.id); a.created = true; a.cell = site.cell;
    a.pos = site.pos; a.pos.x += static_cast<float>(rng_.range(0, 80)); a.hostile = true; a.site = site.id;
    a.pick = rng_.chance(0.5) ? site.variantA : site.variantB;   // leveled pick: made once, here
    a.base = a.pick;
    world_.putActor(a);
    site.spawned = a.id; site.cleared = false; site.rolledBy = roller; ++site.spawnCount;
    ++stats_.spawns;
    MSpawnActor m;
    m.id = a.id; m.site = site.id; m.base = a.base; m.pick = a.pick; m.cell = a.cell; m.pos = a.pos;
    sendBody(m, kNoPlayer);
    MSiteState s;
    s.site = site.id; s.spawned = a.id; s.cleared = false; s.clearedAtMs = site.clearedAtMs; s.spawnCount = site.spawnCount;
    sendBody(s, kNoPlayer);
    std::vector<OwnMsg> out;
    svc_->track(a.id, presenceFor(a), static_cast<uint32_t>(now_), out);
    emitOwn(out);
    return a.id;
}

void Machine::onSpawnRoll(const Envelope& e, const MSpawnRoll& m) {
    if (!isHost()) return;
    auto it = world_.sites().find(m.site);
    if (it == world_.sites().end()) return;
    if (siteFree(it->second, now_) || engineFaults().spawnNoArbitration) hostSpawnAt(it->second, e.src);
    else ++stats_.spawnRollsIgnored;   // someone else already owns this site's roll: this one is switched off
}

void Machine::evalSites() {
    for (auto& kv : world_.sites()) {
        SpawnSiteState& s = kv.second;
        if (!siteFree(s, now_)) continue;
        bool near = false;
        for (uint8_t p = 0; p < kMaxSlots && !near; ++p) near = world_.player(p).present && world_.adjacent(world_.player(p).cell, s.cell);
        if (near) hostSpawnAt(s, kHostPlayer);
    }
}

void Machine::onSpawnActor(const MSpawnActor& m) {
    ActorState a;
    a.id = m.id; a.ref = createdRef(m.id); a.created = true; a.cell = m.cell; a.pos = m.pos; a.hostile = true; a.site = m.site;
    a.base = m.base; a.pick = m.pick;
    if (engineFaults().pickPerMachine) { a.pick = rng_.chance(0.5) ? FormRef("falloutnv.esm", 0x100011u) : FormRef("falloutnv.esm", 0x100012u); a.base = a.pick; }
    world_.putActor(a);
}

void Machine::onSiteState(const MSiteState& m) {
    auto it = world_.sites().find(m.site);
    if (it == world_.sites().end()) return;
    SpawnSiteState& s = it->second;
    s.spawned = m.spawned; s.cleared = m.cleared; s.clearedAtMs = m.clearedAtMs; s.spawnCount = m.spawnCount;
}

// ---- objects ---------------------------------------------------------------------------------

void Machine::onActivate(const Envelope&, const MActivate& m) {
    if (!isHost()) return;
    auto it = world_.objects().find(m.obj);
    if (it == world_.objects().end()) return;
    const WorldObject& o = it->second;
    if (o.locked || o.destroyed || !o.enabled) return;
    ObjectState fx;
    fx.ref = placedRef(o.id); fx.mask = kObjOpen | kObjActivated; fx.open = !o.open; fx.activatedOnce = true;
    commitEffects({Effect{fx}});
}

// ---- digests and repair ----------------------------------------------------------------------

void Machine::onDigest(const Envelope& e, const MDigest& m) {
    if (!isHost() || m.cat >= static_cast<uint8_t>(Category::Count_)) return;
    const Category cat = static_cast<Category>(m.cat);
    // Digest reports travel unreliably: the link may duplicate or reorder them. A copy of one report is not a second, later observation, so
    // "the same mismatch twice" (the stability rule below) only counts reports with a higher number than the last one judged.
    uint32_t& seen = digestSeen_[std::make_pair(e.src, m.cat)];
    if (!engineFaults().digestNoSeq) {
        if (m.seq <= seen) { ++stats_.digestsIgnored; return; }
        seen = m.seq;
    }
    const auto hostKeys = world_.keyHashes(cat);
    uint64_t total = 1469598103934665603ull;
    for (auto& kv : hostKeys) { total = hash64(&kv.first, sizeof kv.first, total); total = hash64(&kv.second, sizeof kv.second, total); }
    const auto slotKey = std::make_pair(e.src, m.cat);
    if (m.total == total) { mismatch_.erase(slotKey); return; }
    // Per key, not per category: a key is repaired once it differs in the same way (same hash on both sides) in two separate reports, i.e. both
    // copies have stopped changing. One actor that keeps walking must not keep a different, static key from ever being healed.
    Mismatch& mm = mismatch_[slotKey];
    std::map<uint32_t, uint64_t> theirs(m.keys.begin(), m.keys.end());
    std::map<uint32_t, std::pair<uint64_t, uint64_t>> now;   // key -> {reporter's hash (0 = absent), host's hash (0 = absent)}
    std::vector<uint32_t> states, removes;
    for (auto& kv : hostKeys) {
        auto t = theirs.find(kv.first);
        if (t != theirs.end() && t->second == kv.second) continue;
        // keys the reporter owns are authoritative on the reporter: never overwrite them from the host's older copy
        if (cat == Category::Actors && ((!isPlayerKey(kv.first) && view_.ownerOf(kv.first) == e.src) || kv.first == playerKey(e.src))) { ++stats_.ownerDiffsSkipped; continue; }
        const std::pair<uint64_t, uint64_t> seenNow{t == theirs.end() ? 0 : t->second, kv.second};
        auto before = mm.keys.find(kv.first);
        if (before != mm.keys.end() && before->second == seenNow && !engineFaults().healDisabled) states.push_back(kv.first);
        else now[kv.first] = seenNow;
    }
    for (auto& kv : theirs) {
        if (hostKeys.count(kv.first) || (cat == Category::Actors && isPlayerKey(kv.first))) continue;
        const std::pair<uint64_t, uint64_t> seenNow{kv.second, 0};
        auto before = mm.keys.find(kv.first);
        if (before != mm.keys.end() && before->second == seenNow && !engineFaults().healDisabled) removes.push_back(kv.first);
        else now[kv.first] = seenNow;
    }
    mm.keys.swap(now);
    if (mm.keys.empty()) mismatch_.erase(slotKey);
    if (!states.empty() || !removes.empty()) pushRepair(e.src, cat, states, removes);
}

void Machine::pushRepair(PlayerId target, Category cat, const std::vector<uint32_t>& keys, const std::vector<uint32_t>& removes) {
    MRepair r;
    r.cat = static_cast<uint8_t>(cat);
    std::string log = "t=" + std::to_string(now_) + " repair p" + std::to_string(target) + " " + toString(cat) + " keys:";
    for (uint32_t k : keys) {
        Archive a = Archive::writer();
        if (!world_.exportKey(cat, k, a)) continue;
        r.states.push_back({k, a.finish()});
        log += " " + std::to_string(k);
        if (cat == Category::Containers && world_.containers().count(k)) { log += "["; for (auto& it : world_.containers().at(k).items) log += std::to_string(it.first.localId) + "x" + std::to_string(it.second) + ","; log += "]"; }
    }
    r.removes = removes;
    for (uint32_t k : removes) log += " -" + std::to_string(k);
    ++stats_.repairsSent;
    stats_.repairKeys += r.states.size() + r.removes.size();
    if (repairLog_.size() < 256) repairLog_.push_back(log);
    if (obs_) obs_->onRepair(cfg_.self, now_, target, cat, r.states.size() + r.removes.size());
    sendBody(std::move(r), target);
}

void Machine::onRepair(const MRepair& m) {
    if (m.cat >= static_cast<uint8_t>(Category::Count_)) return;
    const Category cat = static_cast<Category>(m.cat);
    for (auto& s : m.states) {
        Archive a = Archive::reader(s.second.data(), s.second.size());
        world_.importKey(cat, s.first, a);
    }
    for (uint32_t k : m.removes) world_.removeKey(cat, k);
    ++stats_.repairsApplied;
}

// ---- full state, save / load -----------------------------------------------------------------

MFullState Machine::buildFullState() const {
    MFullState f;
    f.sess = sess_; f.hostNowMs = now_; f.fxSeq = fxSeq_;
    f.world = world_.save();
    if (!engineFaults().joinNoOwnership) {
        svc_->snapshot(f.own);
        // The service forgets an actor once it is untracked (it died), but the host's view keeps the epoch of its last Release. The joiner gets that memory too: without it a
        // late copy of an older assignment (a reordered Grant from before the death) looks like news to a table that has never heard of the actor, and revives an owner for a corpse.
        std::set<NetId> have;
        for (const OwnMsg& o : f.own) have.insert(o.netId);
        if (!engineFaults().joinNoTombstones) view_.forEach([&](ActorNetId id, PlayerId owner, OwnerEpoch epoch) {
            if (owner != kNoPlayer || epoch == 0 || have.count(id)) return;
            OwnMsg r;
            r.kind = OwnMsg::Release; r.netId = id; r.owner = kNoPlayer; r.epoch = epoch;
            f.own.push_back(r);
        });
    }
    return f;
}

void Machine::onFullState(const MFullState& m) {
    const PlayerState before = world_.player(cfg_.self);
    const bool reload = synced_;   // a host load while we were connected: we are the authority on our own vitals, the saved copy is old
    if (!world_.load(m.world)) return;
    resetCombat();
    sess_ = m.sess; lastFxSeq_ = m.fxSeq;
    PlayerState& me = world_.player(cfg_.self);
    me.slot = cfg_.self; me.present = true; me.inventory = before.inventory;
    if (reload) { me.health = before.health; me.maxHealth = before.maxHealth; me.dead = before.dead; me.vitalsSeq = before.vitalsSeq; }
    view_.setSelf(cfg_.self);
    view_.clear();
    for (const OwnMsg& o : m.own) view_.apply(o);
    refreshAiFlags();
    handingOff_.clear(); acked_.clear(); awaitOld_.clear(); pendingClaims_.clear(); pendingPuts_.clear(); rolledSites_.clear();
    wander_.clear(); lastOwnedEpoch_.clear();
    synced_ = true;
    poseDirty_ = true;
    loaded_.clear();
    loadCellsFor(me.cell);
    publishPose(true);
    if (reload) {   // the host's table was rebuilt from a save: tell it our real vitals
        MVitals v;
        v.slot = cfg_.self; v.health = me.health; v.dead = me.dead; v.seq = seqBase() + ++vitalsCounter_;
        me.vitalsSeq = v.seq;
        sendBody(v, kNoPlayer);
    }
}

Bytes Machine::hostSave() const { return world_.save(); }

bool Machine::hostLoad(const Bytes& save) {
    if (!isHost()) return false;
    PlayerState keep[kMaxSlots];
    for (uint8_t s = 0; s < kMaxSlots; ++s) keep[s] = world_.player(s);
    if (!world_.load(save)) return false;
    resetCombat();
    for (uint8_t s = 0; s < kMaxSlots; ++s) world_.player(s) = keep[s];   // players are live, not part of the saved world
    ++sess_;
    ++stats_.hostLoads;
    svc_ = std::make_unique<OwnershipService>(cfg_.own);
    svc_->playerJoined(kHostPlayer);
    for (auto& kv : links_) svc_->playerJoined(static_cast<PlayerId>(kv.first));
    view_.clear();
    handingOff_.clear(); acked_.clear(); awaitOld_.clear(); pendingClaims_.clear(); pendingPuts_.clear(); rolledSites_.clear(); mismatch_.clear();
    dropOrigins_.clear(); wander_.clear(); lastOwnedEpoch_.clear(); ownChangedAt_.clear();
    std::vector<OwnMsg> out;
    for (auto& kv : world_.actors()) {
        if (kv.second.dead || !kv.second.enabled) continue;
        svc_->track(kv.first, presenceFor(kv.second), static_cast<uint32_t>(now_), out);
    }
    svc_->pin(fnvcoop::live::playerNet(kHostPlayer), kHostPlayer, static_cast<uint32_t>(now_), out);   // a new session: every player entity again
    for (auto& kv : links_) svc_->pin(fnvcoop::live::playerNet(static_cast<PlayerId>(kv.first)), static_cast<PlayerId>(kv.first), static_cast<uint32_t>(now_), out);
    for (const OwnMsg& m : out) view_.apply(m);   // silent: the FullState below carries the table
    refreshAiFlags();
    for (auto& kv : links_) sendBody(buildFullState(), static_cast<PlayerId>(kv.first));
    loaded_.clear();
    loadCellsFor(world_.player(kHostPlayer).cell);
    return true;
}

// ---- queries ---------------------------------------------------------------------------------

int64_t Machine::inventoryCount(const FormRef& base) const {
    auto& inv = world_.player(cfg_.self).inventory;
    auto it = inv.find(base);
    return it == inv.end() ? 0 : it->second;
}

std::map<FormRef, int64_t> Machine::inventoryTotals() const {
    std::map<FormRef, int64_t> r;
    for (auto& kv : world_.player(cfg_.self).inventory) r[kv.first] = kv.second;
    return r;
}

}  // namespace fnvcoop::engine
