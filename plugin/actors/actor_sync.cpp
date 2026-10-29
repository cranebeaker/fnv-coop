#include "actors/actor_sync.h"
#include <algorithm>
#include <cctype>
#include <cmath>
#include "fnvcoop/wire.h"

// Mutation twins: NVA_MUT=<n> compiles one deliberate bug into this file; the test suite must fail for every n (see tests/ and CMakeLists).
#ifndef NVA_MUT
#define NVA_MUT 0
#endif
#define MUT(n) (NVA_MUT == (n))

namespace nvcoop::actors {

using fnvcoop::ApplyResult;
using fnvcoop::OwnMsg;
using fnvcoop::Presence;
using fnvcoop::UpdateVerdict;

NetId deriveNetId(const FormRef& ref) {
    std::string name = ref.plugin;
    for (auto& c : name) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
    uint64_t h = fnvcoop::hash64(name.data(), name.size());
    h = fnvcoop::hash64(&ref.localId, sizeof ref.localId, h);
    NetId id = static_cast<NetId>(h ^ (h >> 32));
    if (MUT(43)) return id ? id : 1u;   // MUTANT 43: a placed ref may get a player's (0xFE..) or a created (0xFF..) id
    return live::maskPlacedNetId(id);         // 0xFE.. players, 0xFF.. host-issued (synccore/live/entity.h)
}

namespace {
bool sameState(const live::ActorSnapshot& a, const live::ActorSnapshot& b) {
    return a.cell == b.cell && a.pos == b.pos && a.rot == b.rot && a.vel == b.vel && a.anim == b.anim && a.ai == b.ai && a.avDeltas == b.avDeltas;
}

// The rate planner of the player stream: the profile's tiers and jump rule (the avatar layer's StreamRateConfig, now in synccore/live).
live::StreamRateConfig plannerConfig(const SyncConfig& cfg) {
    const StreamProfile& p = cfg.playerProfile;
    live::StreamRateConfig rc;
    rc.nearPeriodMs = p.nearMs; rc.farPeriodMs = p.farMs; rc.outPeriodMs = p.outMs;
    rc.jumpUnits = MUT(62) ? 0.0f : p.jumpUnits;   // MUT 62 BAD: a teleport waits for its tier's period like any other snapshot
    rc.nearRadius = cfg.interest.nearRadius; rc.hysteresis = cfg.interest.hysteresis;
    rc.enabled = !MUT(59);                          // MUT 59 BAD: the rate is ignored (everything goes at full rate to everybody)
    return rc;
}

// A player's snapshots are checked by ActorSync against the pinned owner and epoch; the receiver only decodes, orders and acknowledges them.
live::ReceiverConfig playerReceiverConfig() {
    live::ReceiverConfig rc;
    rc.checkSenderIsOwner = false;
    rc.maxActors = 64;
    return rc;
}
}  // namespace

ActorSync::ActorSync(const SyncConfig& cfg, IActorPort& port, IActorNet& net)
    : cfg_(cfg), port_(port), net_(net), view_(cfg.self), sender_(cfg.sender),
      receiver_([&cfg] { live::ReceiverConfig rc; rc.checkSenderIsOwner = cfg.isHost; return rc; }()),   // host: sender must be the owner; clients get host-relayed packets
      senderP_(cfg.sender), receiverP_(playerReceiverConfig()), planner_(plannerConfig(cfg)),
      interest_(cfg.interest) {
    if (cfg_.isHost) {
        svc_.reset(new fnvcoop::OwnershipService(cfg_.own));
        svc_->playerJoined(kHostPlayer);
    }
}

ActorSync::~ActorSync() = default;

// ---- plumbing --------------------------------------------------------------------------------------------------------------

bool ActorSync::cellsAdjacent(CellId a, CellId b) const { return a == b || (cfg_.adjacent && cfg_.adjacent(a, b)); }

std::vector<PlayerId> ActorSync::peers() const {
    std::vector<PlayerId> v;
    if (cfg_.isHost) v.assign(joined_.begin(), joined_.end());
    else if (hostUp_) v.push_back(kHostPlayer);
    return v;
}

void ActorSync::sendBody(Body b, PlayerId dest) {
    Envelope e;
    e.src = cfg_.self; e.dest = dest; e.body = std::move(b);
    const Bytes data = encodeEnvelope(e);
    if (!cfg_.isHost) {
        if (hostUp_) net_.send(kHostPlayer, true, data);
        return;
    }
    if (dest != kNoPlayer) {
        if (dest != cfg_.self && joined_.count(dest)) net_.send(dest, true, data);
        return;
    }
    for (PlayerId p : joined_) if (p != cfg_.self) net_.send(p, true, data);
}

void ActorSync::relay(const Envelope& e, PlayerId except) {
    const Bytes data = encodeEnvelope(e);
    for (PlayerId p : joined_) if (p != except && p != cfg_.self) net_.send(p, true, data);
}

ActorSync::Rec* ActorSync::ensureRec(const ActorMeta& m, bool local) {
    auto it = recs_.find(m.netId);
    if (it == recs_.end()) {
        it = recs_.emplace(std::piecewise_construct, std::forward_as_tuple(m.netId), std::forward_as_tuple(cfg_.interp)).first;
        it->second.meta = m;
        it->second.player = live::isPlayerNet(m.netId);
        refToNet_[m.ref] = m.netId;
        auto pk = pendingPick_.find(m.netId);
        if (pk != pendingPick_.end()) { it->second.havePick = true; it->second.pick = pk->second; pendingPick_.erase(pk); }
    }
    if (local) {
        Rec& r = it->second;
        if (!r.local) { r.local = true; r.engineAuthoritative = !r.haveLatest; }
        r.meta.base = m.base;
        r.meta.named = r.meta.named || m.named;
    }
    return &it->second;
}

bool ActorSync::ownedByMe(const Rec& r) const { return view_.ownerOf(r.meta.netId) == cfg_.self && !r.handingOff; }

live::ActorSnapshot ActorSync::fullState(const Rec& r) const {
    live::ActorSnapshot s = r.latest;
    s.avDeltas.clear();
    for (auto& kv : r.av) { live::AvValue v; v.av = kv.first; v.value = kv.second; s.avDeltas.push_back(v); }
    return s;
}

// ---- session ---------------------------------------------------------------------------------------------------------------

void ActorSync::setPlayerPose(PlayerId slot, const PlayerPose& p) { if (slot < fnvcoop::kMaxSlots) players_[slot] = p; }

void ActorSync::playerJoined(PlayerId slot) {
    if (!cfg_.isHost || slot == cfg_.self || slot >= fnvcoop::kMaxSlots) return;
    joined_.insert(slot);
    svc_->playerJoined(slot);
    trackPlayer(cfg_.self);   // (once: the host's own entity)
    trackPlayer(slot);
    sendJoinState(slot);
    nextEvalMs_ = 0;
}

bool ActorSync::isPlayerRecord(NetId id) const { const Rec* r = find(id); return r && r->player; }

void ActorSync::enablePlayerEntities(IActorPort* port, bool parts, bool stream) {
    playerPort_ = port;
    cfg_.playerParts = parts && port != nullptr;
    cfg_.playerStream = stream && port != nullptr;
}

std::vector<PlayerId> ActorSync::playerSlots() const {
    std::vector<PlayerId> v;
    for (const auto& kv : recs_) if (kv.second.player) v.push_back(live::slotOfPlayerNet(kv.first));
    return v;
}

StreamProfile ActorSync::profileFor(NetId id) const {
    if (live::isPlayerNet(id)) return cfg_.playerProfile;
    StreamProfile p;
    p.interp = cfg_.interp;
    p.nearMs = cfg_.interest.nearPeriodMs; p.farMs = cfg_.interest.farPeriodMs; p.outMs = 0;
    return p;
}

bool ActorSync::partValue(NetId id, PartKind kind, Bytes& out) const {
    const Rec* r = find(id);
    if (!r || !partApplies(*r, partIndex(kind))) return false;
    const Rec::Part& p = r->parts[partIndex(kind)];
    if (!p.publishedAny) return false;
    out = p.shared;
    return true;
}

void ActorSync::trackPlayer(PlayerId slot) {
    if (!cfg_.isHost || !svc_ || slot >= fnvcoop::kMaxSlots) return;
    const NetId id = live::playerNet(slot);
    if (find(id) && svc_->pinned(id)) return;
    ActorMeta m;
    m.netId = id; m.ref = live::playerFormRef(slot);
    const bool fresh = !find(id);
    Rec* r = ensureRec(m, slot == cfg_.self);
    r->player = true;
    if (fresh) { ++stats_.playersTracked; sendBody(MTrack{r->meta}, kNoPlayer); }
    std::vector<OwnMsg> out;
    if (!MUT(44)) svc_->pin(id, slot, static_cast<uint32_t>(now_), out);   // MUT 44 BAD: a player entity is owned like an NPC (the nearest player)
    else svc_->track(id, presenceFor(*r), static_cast<uint32_t>(now_), out);
    emitOwn(out);
}

void ActorSync::untrackPlayer(PlayerId slot) {
    if (!cfg_.isHost || slot >= fnvcoop::kMaxSlots) return;
    const NetId id = live::playerNet(slot);
    if (!find(id)) return;
    if (!MUT(45)) sendBody(MUntrack{id}, kNoPlayer);   // MUT 45 BAD: the clients keep a record of a player that left
    ++stats_.playersUntracked;
    dropRecord(id);
}

void ActorSync::onUntrack(const MUntrack& m) {
    if (cfg_.isHost || !live::isPlayerNet(m.id)) return;   // only player records are untracked by message (NPCs leave through ownership)
    if (!find(m.id)) return;
    ++stats_.playersUntracked;
    dropRecord(m.id);
}

void ActorSync::dropRecord(NetId id) {
    Rec* r = find(id);
    if (!r) return;
    interest_.removeActor(id);
    receiver_.forget(id);
    receiverP_.forget(id);
    if (live::isPlayerNet(id)) planner_.removePlace(live::slotOfPlayerNet(id));
    if (!cfg_.isHost) view_.forget(id);
    refToNet_.erase(r->meta.ref);
    acked_.erase(id);
    early_.erase(id);
    recs_.erase(id);
}

// Everything a client needs to know the host's actors: every record, every pick, the whole ownership table. Sent when a client joins and
// again whenever it asks (MResync with id 0): the client's ActorRuntime only exists, and only listens, from the moment it has joined, so
// the first burst can reach the mux before the channel has a handler (dropped there), and a client that went through a baseline
// (hostLost / hostConnected) has an empty view while the host never saw it leave. Every part is idempotent on the receiving side.
void ActorSync::sendJoinState(PlayerId slot) {
    for (auto& kv : recs_) {
        sendBody(MTrack{kv.second.meta}, slot);
        if (kv.second.havePick) sendBody(MPick{kv.first, kv.second.pick}, slot);
        for (size_t k = 0; k < kPartCount; ++k) {
            const Rec::Part& p = kv.second.parts[k];
            if (!p.publishedAny || kv.second.dead || MUT(37)) continue;   // MUT 37 BAD: a joining client never hears about earlier item use / weapon draws
            if (!partApplies(kv.second, k)) continue;
            if (kv.second.player) {
                if (MUT(55)) continue;   // MUT 55 BAD: a late joiner hears nothing of the players' looks and equipment
                if (kv.first == live::playerNet(slot)) continue;   // its own parts are its own
                if (specFor(kv.second, k).audience == PartAudience::HostOnly && !MUT(52)) continue;   // MUT 52 BAD: host-only parts (facts, pack) are sent to a joining client
            }
            MPart m;
            m.id = kv.first; m.epoch = view_.epochOf(kv.first); m.eventId = (static_cast<uint64_t>(cfg_.self) << 48) | ++partEventCounter_;
            m.kind = static_cast<PartKind>(k); m.value = p.shared;
            sendBody(m, slot);
        }
    }
    std::vector<OwnMsg> out;
    svc_->snapshot(out);
    for (auto& m : out) sendBody(MOwn{m}, slot);
}

void ActorSync::playerLeft(PlayerId slot) {
    if (!cfg_.isHost || slot == cfg_.self) return;
    joined_.erase(slot);
    players_[slot].present = false;
    interest_.removeObserver(slot);
    sender_.forgetPeer(slot);
    senderP_.forgetPeer(slot);
    std::vector<OwnMsg> out;
    const std::vector<NetId> orphans = svc_->playerLeft(slot, static_cast<uint32_t>(now_), out);
    emitOwn(out);
    untrackPlayer(slot);
    std::vector<OwnMsg> out2;
    for (NetId id : orphans) {
        Rec* r = find(id);
        if (r && !r->dead) svc_->evaluate(id, presenceFor(*r), static_cast<uint32_t>(now_), out2);
    }
    emitOwn(out2);
}

void ActorSync::hostConnected() {
    if (cfg_.isHost) return;
    hostUp_ = true;
    for (auto& kv : recs_) {
        kv.second.reported = false;
        if (kv.second.local && !kv.second.player) { sendBody(MTrack{kv.second.meta}, kHostPlayer); kv.second.reported = true; }
    }
    senderP_.forgetPeer(kHostPlayer);
    if (!MUT(57)) resetOwnParts();   // MUT 57 BAD: the host (maybe a new one) never hears the local player's look and equipment again
    if (!MUT(23)) { ++stats_.resyncsSent; sendBody(MResync{0}, kHostPlayer); }   // "send me the whole table": the host's join burst may have been lost, or never sent
}

void ActorSync::hostLost() {
    if (cfg_.isHost) return;
    hostUp_ = false;
    view_.clear();
    // The next host may be a different session (restarted from a save): its epochs start from scratch, so no epoch or seq floor from the old
    // one may survive. The cleared view is what keeps stale datagrams out until the new host's ownership table has arrived.
    for (auto& kv : recs_) { if (!MUT(22)) receiver_.forget(kv.first); else receiver_.clearAuthority(kv.first); }
    sender_.forgetPeer(kHostPlayer);
    senderP_.forgetPeer(kHostPlayer);
    receiverP_ = live::SnapshotReceiver(playerReceiverConfig());
    acked_.clear();
    for (auto& kv : recs_) {
        Rec& r = kv.second;
        if (!MUT(22)) { r.lastEpoch = 0; r.lastSeq = 0; r.placedAny = false; }   // (placed marks the exact state already in the engine: a new session may reuse its epoch and seq)
        r.active = false; r.handingOff = false; r.finalApplied = false; r.awaitPrev = kNoPlayer;
        r.interp.reset();
        refreshAi(r);
    }
}

// ---- ownership glue --------------------------------------------------------------------------------------------------------

std::vector<Presence> ActorSync::presenceFor(const Rec& r) const {
    const CellId cell = r.haveLatest ? r.latest.cell : r.meta.cell;
    const live::Vec3 pos = r.haveLatest ? r.latest.pos : r.meta.pos;
    std::vector<Presence> who;
    for (auto& kv : players_) {
        const PlayerPose& p = kv.second;
        if (!p.present) continue;
        if (kv.first != cfg_.self && !joined_.count(kv.first)) continue;   // a pose of someone who left
        Presence pr;
        pr.player = kv.first; pr.connected = true;
        pr.inCell = p.cell == cell;
        pr.adjacent = cellsAdjacent(p.cell, cell) || std::find(p.adjacent.begin(), p.adjacent.end(), cell) != p.adjacent.end();
        pr.distance = live::distance(p.pos, pos);
        who.push_back(pr);
    }
    return who;
}

void ActorSync::emitOwn(const std::vector<OwnMsg>& out) {
    for (const OwnMsg& m : out) {
        const ApplyResult r = view_.apply(m);
        sendBody(MOwn{m}, kNoPlayer);   // before the hooks: a Transfer must reach the new owner ahead of the old owner's final state
        onOwnApplied(m, r);
    }
}

void ActorSync::onOwnApplied(const OwnMsg& m, ApplyResult res) {
    if (res == ApplyResult::Applied) ++stats_.ownershipApplied;
    else if (res == ApplyResult::Duplicate) ++stats_.ownershipDuplicates;
    else ++stats_.ownershipStale;
    if (res != ApplyResult::Applied && !MUT(8)) return;   // idempotent: a replayed message changes nothing
    if (m.kind == OwnMsg::Ack) return;
    if (m.kind == OwnMsg::Transfer) { /* the committed owner does not change until the Commit */ }
    else {
        live::SnapshotReceiver& rx = live::isPlayerNet(m.netId) ? receiverP_ : receiver_;   // a player's stream has its own receiver (StreamProfile)
        if (m.kind == OwnMsg::Release) rx.clearAuthority(m.netId);
        else rx.setAuthority(m.netId, m.epoch, m.owner);   // epoch floor and (host) sender check for the snapshot stream
    }
    Rec* r = find(m.netId);
    if (r && m.kind != OwnMsg::Transfer && m.kind != OwnMsg::Release) feedEarly(m.netId, static_cast<uint32_t>(now_));
    if (r && r->player && !MUT(46)) { r->handingOff = false; r->finalApplied = false; r->awaitPrev = kNoPlayer; return; }   // MUT 46 BAD: the NPC takeover runs on a player
    if (m.kind == OwnMsg::Transfer) {
        if (!r) return;
        if (m.prevOwner == cfg_.self) {   // we give it away: AI off at once, then our last state, reliably
            ++stats_.handoffsOut;
            r->handingOff = true;
            if (!MUT(5)) { r->active = false; refreshAi(*r); }
            flushParts(*r);   // before the final state: the new owner starts from what we last used and held
            const bool sentFinal = sendFull(*r, true, false);
            // Nothing to hand over (an actor this host never sampled): the new owner would wait for a final state that does not exist and the commit came
            // only after handoffTimeoutMs (lab run 20261028-032920: one actor at 1013 ms after every host move, on every profile). A host that sent
            // a final waits for the acknowledgement, as ever. MUT 41 BAD: the commit waits for an acknowledgement that cannot come.
            if (!sentFinal && cfg_.commitWithoutFinal && !MUT(41) && cfg_.isHost && svc_) {
                std::vector<OwnMsg> now2;
                if (svc_->commitNothingToHandOver(m.netId, static_cast<uint32_t>(now_), now2)) emitOwn(now2);
            }
        } else if (m.owner == cfg_.self) {   // we receive it: AI stays off until the Commit
            if (MUT(3)) { r->active = true; refreshAi(*r); }
        }
        return;
    }
    acked_.erase(m.netId);
    if (!r) return;
    const bool sawFinal = r->finalApplied;   // the old owner's last state reached us before the Commit
    r->handingOff = false;
    r->finalApplied = false;
    r->awaitPrev = kNoPlayer;                // any ownership change ends an earlier wait
    // A forced commit (the host did not wait for the old owner) must not let two machines drive the actor: the old owner may not have heard of the
    // Transfer yet (a slow link) and keeps simulating until it does. Until its final state shows that it stopped (or it left, or holdAiMaxMs
    // passed) the new owner keeps its AI off. A previous owner that is the host needs no wait: it stopped when it emitted the Transfer.
    if (m.kind == OwnMsg::Commit && m.owner == cfg_.self && !sawFinal && !MUT(25) && m.prevOwner != kNoPlayer && m.prevOwner != cfg_.self &&
        m.prevOwner != kHostPlayer && playerHere(m.prevOwner)) {
        r->awaitPrev = m.prevOwner;
        r->awaitSince = now_;
        ++stats_.aiHolds;
    }
    if (m.kind != OwnMsg::Release && m.owner == cfg_.self) takeOver(*r, m.epoch);
    else if (r->active) loseOwnership(*r);   // we were running it; observers keep their interpolation buffer across an owner change
    else refreshAi(*r);
}

void ActorSync::placeBest(Rec& r) {
    if (!r.haveLatest) return;
    if (r.placedAny && r.placedEpoch == r.latest.ownerEpoch && r.placedSeq == r.latest.seq) return;   // that exact state is already in the engine
    port_.place(r.meta.netId, fullState(r));
    r.placedAny = true; r.placedEpoch = r.latest.ownerEpoch; r.placedSeq = r.latest.seq;
    r.engineAuthoritative = true;
}

void ActorSync::takeOver(Rec& r, OwnerEpoch epoch) {
    const NetId id = r.meta.netId;
    if (r.dead) return;
    if (!MUT(8) && r.active && r.activeEpoch == epoch) return;   // already running at this epoch (replay / duplicate)
    if (!port_.loaded(id)) return;   // retried by tick() once the engine has the actor
    if (MUT(6)) {   // BAD: recreate the actor instead of continuing it
        port_.removeStandIn(id);
        port_.spawnStandIn(r.meta, fullState(r));
    }
    if (!MUT(4) && !r.engineAuthoritative) placeBest(r);   // the authoritative state goes in BEFORE the AI comes back
    r.active = true; r.activeEpoch = epoch; r.engineAuthoritative = true;
    r.interp.reset();
    r.published = false; r.settled = false; r.lastChangeMs = now_;
    for (Rec::Part& p : r.parts) { p.base = false; p.pendingValid = false; p.nextMs = 0; }   // our copy is the truth from here: read it afresh
    ++stats_.takeovers;
    refreshAi(r);
    considerPick(r);
}

bool ActorSync::playerHere(PlayerId p) const {
    if (p >= fnvcoop::kMaxSlots) return false;
    if (cfg_.isHost && !joined_.count(p)) return false;
    auto it = players_.find(p);
    return it != players_.end() && it->second.present;
}

void ActorSync::releaseAiHold(Rec& r) {
    if (r.awaitPrev == kNoPlayer) return;
    r.awaitPrev = kNoPlayer;
    ++stats_.aiHoldsReleased;
    r.published = false; r.settled = false; r.lastChangeMs = now_;   // our copy may be newer than what the others hold: announce it
    refreshAi(r);
}

// The previous owner's last word after a forced commit: its epoch is older than ours, so it is not applied, but it proves that the old owner
// stopped. The new owner releases its AI hold; when the new owner is another guest only the host saw the message (a client's broadcast
// is relayed by the host, but a stale one is not), so the host passes it on to the new owner alone.
void ActorSync::onStaleFinal(Rec& r, const Envelope& e, const live::ActorSnapshot& s) {
    const NetId id = r.meta.netId;
    if (r.awaitPrev != kNoPlayer && r.awaitPrev == e.src) {
        if (static_cast<OwnerEpoch>(s.ownerEpoch + 1) >= view_.epochOf(id) || MUT(27)) releaseAiHold(r);   // (a much older final says nothing about the handoff we wait for)
        return;
    }
    if (!cfg_.isHost || MUT(26)) return;
    const PlayerId owner = view_.ownerOf(id);
    if (owner == cfg_.self || owner == e.src || owner == kNoPlayer || owner >= fnvcoop::kMaxSlots || !joined_.count(owner)) return;
    Envelope fwd = e;
    fwd.dest = owner;
    ++stats_.staleFinalsForwarded;
    net_.send(owner, true, encodeEnvelope(fwd));
}

void ActorSync::loseOwnership(Rec& r) {
    r.active = false;
    r.interp.reset();
    refreshAi(r);
}

void ActorSync::refreshAi(Rec& r) {
    const NetId id = r.meta.netId;
    if (!port_.loaded(id)) return;
    const bool want = r.active && !r.dead && !r.handingOff && r.awaitPrev == kNoPlayer;
    if (MUT(5) && r.handingOff) return;   // BAD: the old owner keeps simulating while the handoff is pending
    if (!want && MUT(1)) return;   // BAD: nobody ever turns the AI of a non-owned actor off
    if (r.aiKnown && r.aiApplied == want) return;
    port_.setAi(id, want);
    r.aiKnown = true; r.aiApplied = want;
}

void ActorSync::ensureProtected(Rec& r) {
    if (r.meta.named && !r.killAuthorized && !r.protectedOn && !r.dead && port_.loaded(r.meta.netId)) {
        port_.setProtected(r.meta.netId, true);
        r.protectedOn = true;
    }
}

void ActorSync::syncToView(Rec& r) {
    if (r.player) return;
    if (r.dead) {
        if (r.killPending && port_.loaded(r.meta.netId)) { r.killPending = false; port_.kill(r.meta.netId); }
        return;
    }
    r.aiKnown = false;
    refreshAi(r);
    ensureProtected(r);
    if (view_.ownerOf(r.meta.netId) == cfg_.self && !r.handingOff && !r.dead) takeOver(r, view_.epochOf(r.meta.netId));
    considerPick(r);
}

void ActorSync::maybeAck(Rec& r) {
    OwnerEpoch pe = 0;
    const NetId id = r.meta.netId;
    if (!view_.handoffIncoming(id, pe) || acked_.count(id)) return;
    if (MUT(17)) return;   // BAD: never acknowledge; the host can only force the commit after its timeout
    acked_.insert(id);
    OwnMsg ack;
    ack.kind = OwnMsg::Ack; ack.netId = id; ack.epoch = pe; ack.owner = cfg_.self;
    ++stats_.acksSent;
    if (cfg_.isHost) {
        std::vector<OwnMsg> out;
        svc_->onAck(ack, cfg_.self, static_cast<uint32_t>(now_), out);
        emitOwn(out);
    } else {
        sendBody(MOwn{ack}, kHostPlayer);
    }
}

void ActorSync::hostRegister(Rec& r) {
    if (!svc_) return;
    std::vector<OwnMsg> out;
    svc_->track(r.meta.netId, presenceFor(r), static_cast<uint32_t>(now_), out);
    auto pr = pendingRecruiter_.find(r.meta.ref);
    if (pr != pendingRecruiter_.end()) {
        svc_->setRecruiter(r.meta.netId, pr->second, presenceFor(r), static_cast<uint32_t>(now_), out);
        pendingRecruiter_.erase(pr);
    }
    emitOwn(out);
}

void ActorSync::evalOwnership() {
    std::vector<OwnMsg> out;
    for (auto& kv : recs_) {
        const Rec& r = kv.second;
        if (r.dead || r.player) continue;
        svc_->track(r.meta.netId, presenceFor(r), static_cast<uint32_t>(now_), out);
    }
    emitOwn(out);
}

// ---- engine events ---------------------------------------------------------------------------------------------------------

bool ActorSync::track(const ActorMeta& meta) {
    if (meta.netId == 0) return false;
    if (live::isPlayerNet(meta.netId) || live::isPlayerFormRef(meta.ref)) { ++stats_.playerRefused; return false; }   // the host's trackPlayer makes those
    auto byRef = refToNet_.find(meta.ref);
    auto byId = recs_.find(meta.netId);
    if ((byRef != refToNet_.end() && byRef->second != meta.netId) || (byId != recs_.end() && !(byId->second.meta.ref == meta.ref))) {
        ++stats_.idCollisions;
        return false;
    }
    const bool fresh = byId == recs_.end();
    Rec* r = ensureRec(meta, true);
    if (fresh) { r->latest.cell = meta.cell; r->latest.pos = meta.pos; }
    if (cfg_.isHost) {
        if (fresh) { sendBody(MTrack{r->meta}, kNoPlayer); hostRegister(*r); }
    } else if (hostUp_ && !r->reported) {
        r->reported = true;
        sendBody(MTrack{r->meta}, kHostPlayer);
    }
    syncToView(*r);
    return true;
}

void ActorSync::untrack(NetId id) {
    Rec* r = find(id);
    if (!r || r->player) return;
    dropStandIn(*r);
    if (svc_) {
        std::vector<OwnMsg> out;
        svc_->untrack(id, out);
        emitOwn(out);
    }
    interest_.removeActor(id);
    receiver_.forget(id);
    refToNet_.erase(r->meta.ref);
    acked_.erase(id);
    recs_.erase(id);
}

void ActorSync::onPlayerBound(NetId id) {
    Rec* r = find(id);
    if (!r || !r->player) return;
    rewantParts(*r);   // a new puppet (a respawn after a save, a cell park, a template change) is dressed again from the last values known
}

void ActorSync::onActorLoaded(NetId id) {
    Rec* r = find(id);
    if (!r) return;
    if (r->player) { rewantParts(*r); return; }
    dropStandIn(*r);   // the real ref replaces the clone
    r->local = true;
    rewantParts(*r);
    if (!cfg_.isHost && hostUp_ && !r->reported) { r->reported = true; sendBody(MTrack{r->meta}, kHostPlayer); }
    syncToView(*r);
}

void ActorSync::onActorUnloaded(NetId id) {
    Rec* r = find(id);
    if (!r) return;
    r->local = false;
    r->aiKnown = false;
    r->protectedOn = false;
    r->active = false;
}

void ActorSync::onNativeBase(NetId id, const FormRef& base) {
    Rec* r = find(id);
    if (!r) return;
    r->meta.base = base;
    considerPick(*r);
}

// A load brings every actor back as the save has it, without a ResurrectActor. A death kept from the old world made the owner skip the next
// death ("the network already declared it") and every other machine ignore the death message (in game 2026-10-21: a gecko killed in one run
// stayed alive on the guest in the next). A corpse the loaded world still has is polled and declared again by its owner.
void ActorSync::onWorldReloaded() {
    for (auto& kv : recs_) {
        Rec& r = kv.second;
        r.dead = false;
        r.killAuthorized = false;
        r.killPending = false;
    }
}

void ActorSync::purgeStandIns() {
    if (MUT(14)) return;   // BAD: clones survive a load
    for (auto& kv : recs_) dropStandIn(kv.second);
}

// ---- stand-in clones -------------------------------------------------------------------------------------------------------

void ActorSync::ensureStandIn(Rec& r) {
    if (r.standIn || r.dead || !r.haveLatest) return;
    if (!port_.cellLoaded(r.latest.cell)) return;
    ActorMeta m = r.meta;
    if (r.havePick) m.base = r.pick;
    if (port_.spawnStandIn(m, fullState(r))) {
        r.standIn = true;
        r.aiKnown = false;
        ++stats_.standInsMade;
        refreshAi(r);
        rewantParts(r);
    }
}

void ActorSync::dropStandIn(Rec& r) {
    if (!r.standIn) return;
    port_.removeStandIn(r.meta.netId);
    r.standIn = false;
    r.aiKnown = false;
    ++stats_.standInsRemoved;
}

// ---- owner side: sampling and publishing -----------------------------------------------------------------------------------

uint32_t ActorSync::ownerPeriod(const Rec& r) const {
    const CellId cell = r.haveLatest ? r.latest.cell : r.meta.cell;
    bool anyKnown = false, nearBy = false;
    for (auto& kv : players_) {
        if (kv.first == cfg_.self || !kv.second.present) continue;
        anyKnown = true;
        if (kv.second.cell == cell) nearBy = true;
    }
    return (nearBy || !anyKnown) ? cfg_.interest.nearPeriodMs : cfg_.interest.farPeriodMs;
}

void ActorSync::publishOwned(Rec& r, uint32_t now) {
    const NetId id = r.meta.netId;
    if (r.handingOff || r.dead) return;
    if (r.published && now_ < r.lastPublishMs + ownerPeriod(r)) return;
    OwnerEpoch ep = 0;
    if (!view_.mayPublish(id, ep)) {
        if (!MUT(2)) return;
        ep = view_.epochOf(id);   // BAD: publish without being the owner
        ++stats_.publishedNotOwner;
    }
    live::ActorSnapshot s;
    if (!port_.sample(id, s)) return;
    s.netId = id; s.ownerEpoch = ep; s.seq = seqs_.next(id, ep); s.tick = now;
    r.lastPublishMs = now_;
    const bool changed = !r.published || !sameState(s, r.lastPub);
    r.published = true;
    r.lastPub = s;
    ++stats_.published;
    if (changed) { r.lastChangeMs = now_; r.settled = false; }
    for (auto& v : s.avDeltas) r.av[v.av] = v.value;
    r.latest = s;
    r.haveLatest = true;
    r.lastEpoch = ep; r.lastSeq = s.seq;
    r.engineAuthoritative = true;
    if (!cfg_.isHost && hostUp_) sender_.queue(kHostPlayer, s, now);   // the host relays; its own samples are pulled by relayToClients
    if (!changed && !r.settled && now_ >= r.lastChangeMs + cfg_.settleMs) {
        r.settled = true;
        sendFull(r, false, true);   // one reliable copy of a settled actor: a lost datagram cannot leave others behind
    }
}

bool ActorSync::sendFull(Rec& r, bool final, bool settle) {
    const NetId id = r.meta.netId;
    OwnerEpoch ep = 0;
    if (!view_.mayPublish(id, ep)) return false;   // still the owner at the committed epoch while a handoff is pending
    live::ActorSnapshot s;
    if (!port_.sample(id, s)) { if (!r.haveLatest) return false; s = r.latest; }
    s.netId = id; s.ownerEpoch = ep; s.seq = seqs_.next(id, ep); s.tick = static_cast<uint32_t>(now_);
    for (auto& v : s.avDeltas) r.av[v.av] = v.value;
    r.latest = s; r.latest.avDeltas = fullState(r).avDeltas;
    r.haveLatest = true; r.lastEpoch = ep; r.lastSeq = s.seq;
    if (final) ++stats_.finalsSent;
    MSnapFull m;
    m.s = r.latest; m.final = final; m.settle = settle;
    sendBody(m, kNoPlayer);
    return true;
}

// ---- receiving snapshots ---------------------------------------------------------------------------------------------------

bool ActorSync::acceptSnapshot(Rec& r, const live::ActorSnapshot& s, uint32_t now) {
    const NetId id = r.meta.netId;
    if (live::serialLess(s.ownerEpoch, r.lastEpoch)) { ++stats_.staleEpochDropped; return false; }
    if (s.ownerEpoch == r.lastEpoch && r.lastSeq != 0 && live::serialLE(s.seq, r.lastSeq)) { ++stats_.staleSeqDropped; return false; }
    r.lastEpoch = s.ownerEpoch; r.lastSeq = s.seq;
    std::vector<live::AvValue> changedAv;
    for (auto& v : s.avDeltas) {
        auto it = r.av.find(v.av);
        if (it == r.av.end() || it->second != v.value) changedAv.push_back(v);
        r.av[v.av] = v.value;
    }
    r.latest = s;
    r.latest.avDeltas = fullState(r).avDeltas;
    r.haveLatest = true;
    r.interp.push(s, now);
    ++stats_.snapshotsApplied;
    if (!changedAv.empty() && port_.loaded(id) && !(view_.ownerOf(id) == cfg_.self && r.active)) port_.applyAv(id, changedAv);
    return true;
}

void ActorSync::requestResync(Rec& r) {
    if (cfg_.isHost || !hostUp_) return;
    if (r.resyncedOnce && now_ < r.lastResyncMs + cfg_.resyncMinMs) return;
    r.resyncedOnce = true; r.lastResyncMs = now_;
    ++stats_.resyncsSent;
    sendBody(MResync{r.meta.netId}, kHostPlayer);
}

void ActorSync::onLiveBatch(PlayerId from, const uint8_t* data, size_t size, uint32_t now) {
    for (const live::ReceivedSnapshot& res : receiver_.receive(from, data, size)) {
        if (res.verdict == live::Verdict::Malformed) { ++stats_.undecodable; continue; }
        if (res.verdict == live::Verdict::StaleSeq) { ++stats_.staleSeqDropped; continue; }
        if (res.verdict == live::Verdict::StaleEpoch) { ++stats_.staleEpochDropped; continue; }
        if (res.verdict == live::Verdict::WrongOwner) { ++stats_.notOwnerDropped; continue; }
        if (res.verdict != live::Verdict::Accepted) { ++stats_.snapshotsDropped; continue; }
        const live::ActorSnapshot& s = res.snap;
        Rec* r = find(s.netId);
        if (!r) { ++stats_.unknownActor; holdEarly(from, s); continue; }
        if (r->player) { ++stats_.snapshotsDropped; continue; }   // a player's stream still rides the avatar layer (S11 moves it here)
        if (!view_.known(s.netId) && !MUT(47)) { holdEarly(from, s); continue; }   // MUT 47 BAD: a snapshot ahead of its ownership is lost
        UpdateVerdict v;
        if (MUT(7)) v = UpdateVerdict::Accept;   // BAD: no epoch / owner check (STR #906)
        else if (cfg_.isHost) v = view_.check(s.netId, from, s.ownerEpoch);
        else v = view_.check(s.netId, view_.ownerOf(s.netId), s.ownerEpoch);   // relayed by the host: the epoch must be the one we know
        switch (v) {
            case UpdateVerdict::Accept: break;
            case UpdateVerdict::StaleEpoch: ++stats_.staleEpochDropped; continue;
            case UpdateVerdict::FutureEpoch: ++stats_.futureEpoch; requestResync(*r); continue;
            case UpdateVerdict::NotOwner: case UpdateVerdict::NotYetActive: ++stats_.notOwnerDropped; continue;
            case UpdateVerdict::EchoOfSelf: continue;
            case UpdateVerdict::UnknownActor: ++stats_.unknownActor; continue;
        }
        acceptSnapshot(*r, s, now);
    }
}

bool ActorSync::onSnapFull(const Envelope& e, const MSnapFull& m, uint32_t now) {
    Rec* r = find(m.s.netId);
    if (!r) { ++stats_.unknownActor; return false; }
    if (r->player) { ++stats_.snapshotsDropped; return false; }
    UpdateVerdict v = MUT(7) ? UpdateVerdict::Accept : view_.check(m.s.netId, e.src, m.s.ownerEpoch);
    switch (v) {
        case UpdateVerdict::Accept: break;
        case UpdateVerdict::StaleEpoch: ++stats_.staleEpochDropped; if (m.final) onStaleFinal(*r, e, m.s); return false;
        case UpdateVerdict::FutureEpoch: ++stats_.futureEpoch; requestResync(*r); return false;
        case UpdateVerdict::NotOwner: case UpdateVerdict::NotYetActive: ++stats_.notOwnerDropped; return false;
        case UpdateVerdict::EchoOfSelf: return false;
        case UpdateVerdict::UnknownActor: ++stats_.unknownActor; return false;
    }
    // The host also relays every actor's newest snapshot UNRELIABLY, and that includes the final it has just sent reliably (same seq). On a link with jitter the
    // copy arrives first, and the final then reads "the same seq again" and was dropped: the new owner never placed the state and never acknowledged, so the host forced
    // the commit after handoffTimeoutMs (lab run 20261028-032920: ownership reached the client 1.1 s late on mobile_tether and transatlantic). A final that only
    // repeats what this epoch already delivered still completes the handoff (the state it carries is the one already held).
    bool repeated = false;
    if (!acceptSnapshot(*r, m.s, now)) {
        repeated = m.final && cfg_.acceptRepeatedFinal && !MUT(40) && m.s.ownerEpoch == r->lastEpoch && r->lastSeq != 0 && live::serialLE(m.s.seq, r->lastSeq);
        if (!repeated) return false;
        ++stats_.finalsRepeated;
        --stats_.staleSeqDropped;   // not a drop: it is counted above as one by acceptSnapshot
    }
    if (cfg_.isHost && e.src != cfg_.self) relay(e, e.src);   // onward first: the new owner's Ack (and the Commit) must follow the relay
    if (m.final) {
        ++stats_.finalsApplied;
        OwnerEpoch pe = 0;
        if (view_.handoffIncoming(r->meta.netId, pe)) {   // we are the new owner: the old owner's last state goes into the engine now
            placeBest(*r);
            r->finalApplied = true;
            maybeAck(*r);
        }
    }
    return true;
}

// ---- non-owner side --------------------------------------------------------------------------------------------------------

void ActorSync::driveNonOwner(Rec& r, uint32_t now) {
    const NetId id = r.meta.netId;
    if (r.dead || !r.haveLatest) return;
    if (r.standIn && !port_.cellLoaded(r.latest.cell)) dropStandIn(r);   // it walked out of everything we have loaded
    const bool needStandIn = MUT(15) ? !r.standIn : !port_.loaded(id);   // MUT 15 BAD: a clone even though the real actor is there
    if (needStandIn) {
        ensureStandIn(r);
        if (!r.standIn) return;
    }
    refreshAi(r);
    OwnerEpoch pe = 0;
    if (r.finalApplied && view_.handoffIncoming(id, pe)) return;   // new owner, waiting for the Commit: the placed final state stands
    live::ActualPose act;
    const bool have = port_.pose(id, act);
    const live::InterpOut out = r.interp.sample(now, have ? &act : nullptr);
    if (out.state == live::InterpState::Empty) return;
    port_.drive(id, out);
    r.engineAuthoritative = false;
}

// A copy's actor values are written when the owner's value CHANGES. An engine that keeps an actor's damage elsewhere while it is not fully loaded gives the copy
// another health once it loads, with nothing new from the owner to correct it: the copy read -55 for the owner's 5 until the area digest healed it a few seconds
// later (vats room behind bad_wifi). The copy is checked against the owner's merged values on a slow timer and brought back.
void ActorSync::auditAvs(Rec& r) {
    if (cfg_.avAuditMs == 0 || MUT(42)) return;   // MUT 42 BAD: a copy is never checked against the owner's values
    if (r.dead || !r.haveLatest || r.av.empty() || now_ < r.avAuditAt || avAuditBudget_ == 0) return;
    const NetId id = r.meta.netId;
    if (!port_.loaded(id) || r.killPending) return;
    --avAuditBudget_;
    r.avAuditAt = now_ + cfg_.avAuditMs;
    std::vector<live::AvValue> want;
    for (auto& kv : r.av) { live::AvValue v; v.av = kv.first; v.value = kv.second; want.push_back(v); }
    ++stats_.avAudits;
    stats_.avDriftFixed += port_.auditAv(id, want);
}

void ActorSync::relayToClients(uint32_t now) {
    if (!cfg_.isHost) return;
    for (auto& kv : players_) {
        const PlayerId p = kv.first;
        if (!kv.second.present) continue;
        live::Observer o;
        o.player = p; o.cell = kv.second.cell; o.pos = kv.second.pos; o.adjacent = kv.second.adjacent;
        interest_.setObserver(o);
    }
    for (auto& kv : recs_) {
        const Rec& r = kv.second;
        if (r.dead || r.player) continue;
        live::InterestActor a;
        a.id = kv.first;
        a.cell = r.haveLatest ? r.latest.cell : r.meta.cell;
        a.pos = r.haveLatest ? r.latest.pos : r.meta.pos;
        a.owner = view_.ownerOf(kv.first);
        interest_.setActor(a);
    }
    for (PlayerId slot : joined_) {
        auto pose = players_.find(slot);
        const bool known = pose != players_.end() && pose->second.present;
        std::vector<NetId> due;
        if (known) {
            due = interest_.collect(slot, now);
        } else {   // no pose for this client yet: send everything at the near rate
            for (auto& kv : recs_) {
                uint32_t& last = relayAt_[{slot, kv.first}];
                if (last == 0 || now >= last + cfg_.interest.nearPeriodMs) { last = now ? now : 1; due.push_back(kv.first); }
            }
        }
        for (NetId id : due) {
            Rec* r = find(id);
            if (!r || r->dead || r->player || !r->haveLatest || r->latest.seq == 0) continue;
            if (view_.ownerOf(id) == slot) continue;   // never back to its owner
            sender_.queue(slot, r->latest, now);
        }
    }
}

void ActorSync::flushPeers(uint32_t now) {
    for (PlayerId p : peers()) {
        for (Bytes& d : sender_.flush(p, now)) net_.send(p, false, wrapLive(SubBatch, d));
        Bytes ack = receiver_.takeAcks(p);
        if (!ack.empty()) net_.send(p, false, wrapLive(SubAck, ack));
    }
}

// ---- the player stream (StreamProfile of a player, docs/entity-core-plan.md S10) -----------------------------------------------------------
// The same snapshot kind, codec, ids and ownership checks as an NPC's, with the avatar layer's policy: the owner's port samples at 30 Hz, a client sends its stream
// to the host at the highest tier any other player has for it, the host judges it (pinned owner, epoch), hands it to the port's playback buffer and relays it to
// every other client the moment it arrives, through the rate planner (near / far / out tiers, a teleport passes at once, the last place is never lost behind the
// rate). Its own sender and receiver pair keeps a player's snapshot out of the packets NPC bursts fill.

// Who a client's own stream is relayed to by the host: every other player it knows of, the host included.
std::vector<PlayerId> ActorSync::uplinkObservers() const {
    std::vector<PlayerId> v;
    for (const auto& kv : recs_) {
        if (!kv.second.player) continue;
        const PlayerId p = live::slotOfPlayerNet(kv.first);
        if (p != cfg_.self) v.push_back(p);
    }
    if (std::find(v.begin(), v.end(), kHostPlayer) == v.end()) v.push_back(kHostPlayer);
    return v;
}

// One snapshot of `subject` for `peer` through the planner. A snapshot that is not due yet is kept (latest wins) and released by releasePlayerPending when its turn
// comes. `peer == kNoPlayer` is the uplink of the local stream, to the host.
void ActorSync::queuePlayer(PlayerId peer, PlayerId subject, const live::ActorSnapshot& snap, uint32_t now) {
    live::ActorSnapshot out;
    bool go = false;
    if (peer == kNoPlayer) {
        const std::vector<PlayerId> obs = uplinkObservers();
        if (MUT(60)) {   // MUT 60 BAD: the uplink follows the LOWEST tier of its observers (a near neighbour would see 5 or 1 Hz)
            live::StreamTier lo = live::StreamTier::Near;
            for (PlayerId o : obs) {
                const live::StreamTier t = planner_.tierOf(o, subject);
                if (static_cast<uint8_t>(t) < static_cast<uint8_t>(lo)) lo = t;
            }
            go = planner_.offerAtTier(live::StreamRatePlanner::kUplink, subject, lo, snap, now, out);
        } else {
            go = planner_.offerUplink(subject, obs, snap, now, out);
        }
    } else {
        go = planner_.offer(peer, subject, snap, now, out);
    }
    if (!go) { ++stats_.playerRateKept; return; }
    const PlayerId to = peer == kNoPlayer ? kHostPlayer : peer;
    live::SnapshotSender& snd = MUT(58) ? sender_ : senderP_;   // MUT 58 BAD: a player's snapshot shares the NPC sender (and its packets)
    if (snd.queue(to, out, now)) {
        ++stats_.playerSent;
        if (subject != cfg_.self) ++stats_.playerRelayed;
    }
}

void ActorSync::samplePlayer(uint32_t now) {
    const NetId id = live::playerNet(cfg_.self);
    Rec* r = find(id);
    if (!r || !r->player) return;
    OwnerEpoch ep = 0;
    if (!view_.mayPublish(id, ep)) return;   // the host has not granted the entity yet (or it is not ours)
    IActorPort& pt = portFor(id);
    if (!pt.loaded(id)) return;
    const StreamProfile& prof = cfg_.playerProfile;
    for (int i = 0; i < 4; ++i) {   // a slow frame may have produced more than one sample
        live::ActorSnapshot s;
        if (!pt.sample(id, s)) break;
        s.netId = id; s.ownerEpoch = ep; s.seq = seqs_.next(id, ep);
        if (!prof.tickFromPort || MUT(67)) s.tick = now;   // MUT 67 BAD: the sampler's tick is replaced by ActorSync's clock (the playback buffer's offset filter sees another clock)
        ++stats_.playerSampled;
        for (auto& v : s.avDeltas) r->av[v.av] = v.value;
        r->latest = s; r->haveLatest = true; r->published = true;
        r->lastEpoch = ep; r->lastSeq = s.seq;
        planner_.setPlace(cfg_.self, s.cell, s.pos);
        if (cfg_.isHost) {
            for (PlayerId c : joined_) queuePlayer(c, cfg_.self, s, now);
        } else if (hostUp_) {
            queuePlayer(kNoPlayer, cfg_.self, s, now);   // the uplink: the host relays it onward at the tier each other player has
        }
    }
}

void ActorSync::releasePlayerPending(uint32_t now) {
    if (MUT(61)) return;   // MUT 61 BAD: a snapshot held back by the rate is never released (a stopped player stays where the last sent one was)
    std::vector<live::StreamRatePlanner::Release> rel;
    planner_.poll(now, rel);
    for (auto& r : rel) {
        const PlayerId to = r.observer == live::StreamRatePlanner::kUplink ? kHostPlayer : r.observer;
        if (!cfg_.isHost && (to != kHostPlayer || !hostUp_)) continue;   // a client only talks to the host, and only while the link is up
        if (cfg_.isHost && !joined_.count(to)) continue;                  // the observer left meanwhile
        if (senderP_.queue(to, r.snap, now)) {
            ++stats_.playerSent; ++stats_.playerRateReleased;
            if (r.subject != cfg_.self) ++stats_.playerRelayed;
        }
    }
}

void ActorSync::flushPlayerPeers(uint32_t now) {
    for (PlayerId p : peers())
        for (Bytes& d : senderP_.flush(p, now)) net_.send(p, false, wrapLive(SubBatchPlayer, d));
    const uint32_t ackMs = cfg_.playerProfile.ackMs;
    if (ackMs && now_ - lastAckP_ < ackMs) return;
    lastAckP_ = now_;
    for (PlayerId p : peers()) {
        Bytes ack = receiverP_.takeAcks(p);
        if (ack.empty()) continue;
        ++stats_.playerAcksSent;
        net_.send(p, false, wrapLive(SubAckPlayer, ack));
    }
}

void ActorSync::onPlayerBatch(PlayerId from, const uint8_t* data, size_t size, uint32_t now) {
    if (!cfg_.playerStream || !playerPort_) { ++stats_.playerRejected; return; }   // the avatar channel carries the stream in this session
    for (const live::ReceivedSnapshot& res : receiverP_.receive(from, data, size)) {
        if (res.verdict != live::Verdict::Accepted) { ++stats_.playerRejected; continue; }
        if (!live::isPlayerNet(res.snap.netId)) { ++stats_.playerRejected; continue; }
        Rec* r = find(res.snap.netId);
        if (!r || !r->player || !view_.known(res.snap.netId)) {
            // The receiver acknowledged this snapshot already, so a player who stands still would send nothing but a heartbeat for a second from now on:
            // keep the newest per player and feed it the moment the record and its ownership are known (the avatar layer's early buffer, for every entity).
            ++stats_.playerUnknown;
            live::ActorSnapshot held = res.snap;
            held.avDeltas = res.avAll;
            holdEarly(from, held);
            continue;
        }
        if (live::slotOfPlayerNet(res.snap.netId) == cfg_.self) { ++stats_.playerEchoed; continue; }   // our own stream echoed back
        UpdateVerdict v = cfg_.isHost ? view_.check(res.snap.netId, from, res.snap.ownerEpoch) : view_.check(res.snap.netId, view_.ownerOf(res.snap.netId), res.snap.ownerEpoch);
        if (cfg_.isHost && MUT(64)) v = UpdateVerdict::Accept;                         // MUT 64 BAD: a client may publish another player's stream
        switch (v) {
            case UpdateVerdict::Accept: break;
            case UpdateVerdict::StaleEpoch: ++stats_.playerStale; continue;
            case UpdateVerdict::FutureEpoch: ++stats_.futureEpoch; requestResync(*r); continue;
            case UpdateVerdict::NotOwner: case UpdateVerdict::NotYetActive: ++stats_.playerForged; continue;
            case UpdateVerdict::EchoOfSelf: ++stats_.playerEchoed; continue;
            case UpdateVerdict::UnknownActor: ++stats_.playerUnknown; continue;
        }
        acceptPlayerSnapshot(*r, res, from, now);
    }
}

void ActorSync::acceptPlayerSnapshot(Rec& r, const live::ReceivedSnapshot& res, PlayerId from, uint32_t now) {
    (void)from;
    const live::ActorSnapshot& s = res.snap;
    if (live::serialLess(s.ownerEpoch, r.lastEpoch)) { ++stats_.playerStale; return; }
    if (s.ownerEpoch == r.lastEpoch && r.lastSeq != 0 && live::serialLE(s.seq, r.lastSeq)) { ++stats_.playerStale; return; }
    r.lastEpoch = s.ownerEpoch; r.lastSeq = s.seq;
    for (const live::AvValue& v : res.avAll) r.av[v.av] = v.value;
    r.latest = s; r.latest.avDeltas = res.avAll; r.haveLatest = true;
    const PlayerId slot = live::slotOfPlayerNet(r.meta.netId);
    if (!MUT(72)) planner_.setPlace(slot, s.cell, s.pos);   // MUT 72 BAD: the places the tiers are decided by are never learned
    ++stats_.playerAccepted;
    portFor(r.meta.netId).onSnapshot(r.meta.netId, s, now);   // the playback buffer is the port's (StreamProfile::interpInPort)
    if (cfg_.isHost && cfg_.playerProfile.relayOnArrival) {   // the host relays at once, to every other client, at the tier it has for them
        live::ActorSnapshot relayed = s;
        relayed.avDeltas = res.avAll;
        for (PlayerId c : joined_)
            if (c != slot || MUT(69)) queuePlayer(c, slot, relayed, now);   // MUT 69 BAD: the relay goes back to the owner too
    }
}

void ActorSync::applyEarlyPlayer(Rec& r, const Early& e, uint32_t now) {
    const UpdateVerdict v = cfg_.isHost ? view_.check(r.meta.netId, e.from, e.s.ownerEpoch) : view_.check(r.meta.netId, view_.ownerOf(r.meta.netId), e.s.ownerEpoch);
    if (v != UpdateVerdict::Accept || live::slotOfPlayerNet(r.meta.netId) == cfg_.self) { ++stats_.earlyDropped; return; }
    live::ReceivedSnapshot res;
    res.verdict = live::Verdict::Accepted;
    res.snap = e.s;
    res.avAll = e.s.avDeltas;   // held with every value known (onPlayerBatch)
    acceptPlayerSnapshot(r, res, e.from, now);
    ++stats_.earlyApplied;
}

// ---- receive / dispatch ----------------------------------------------------------------------------------------------------

void ActorSync::receive(PlayerId from, const uint8_t* data, size_t size) {
    if (!data || size < 2 || data[0] != kChannelActors) { ++stats_.undecodable; return; }
    const uint32_t now = static_cast<uint32_t>(now_);
    const uint8_t sub = data[1];
    if (sub == SubBatch) { onLiveBatch(from, data + 2, size - 2, now); return; }
    if (sub == SubAck) { if (!sender_.onAckPacket(from, data + 2, size - 2)) ++stats_.undecodable; return; }
    if (sub == SubBatchPlayer) { onPlayerBatch(from, data + 2, size - 2, now); return; }
    if (sub == SubAckPlayer) { if (!senderP_.onAckPacket(from, data + 2, size - 2)) ++stats_.undecodable; return; }
    if (sub != SubEnvelope) { ++stats_.undecodable; return; }
    Envelope e;
    if (!decodeEnvelope(data + 2, size - 2, e)) { ++stats_.undecodable; return; }
    if (cfg_.isHost ? (e.src != from || from >= fnvcoop::kMaxSlots) : (from != kHostPlayer)) { ++stats_.spoofed; return; }
    if (cfg_.isHost && !joined_.count(from)) { ++stats_.spoofed; return; }
    dispatch(from, e, now);
}

void ActorSync::dispatch(PlayerId from, const Envelope& e, uint32_t now) {
    if (e.dest != kNoPlayer && e.dest != cfg_.self) return;   // not for us (clients never address each other)
    if (auto* m = std::get_if<MOwn>(&e.body)) {
        if (cfg_.isHost) {
            if (m->m.kind != OwnMsg::Ack) { ++stats_.spoofed; return; }   // clients may only acknowledge
            std::vector<OwnMsg> out;
            svc_->onAck(m->m, from, static_cast<uint32_t>(now_), out);
            emitOwn(out);
        } else {
            const ApplyResult r = view_.apply(m->m);
            onOwnApplied(m->m, r);
        }
    } else if (auto* m2 = std::get_if<MTrack>(&e.body)) {
        onTrack(from, *m2);
    } else if (auto* m3 = std::get_if<MSnapFull>(&e.body)) {
        onSnapFull(e, *m3, now);
    } else if (auto* m4 = std::get_if<MDeath>(&e.body)) {
        onDeath(e, *m4);
    } else if (auto* m5 = std::get_if<MPick>(&e.body)) {
        if (!cfg_.isHost) onPick(*m5);
    } else if (auto* m6 = std::get_if<MPickReport>(&e.body)) {
        if (cfg_.isHost) onPickReport(e, *m6);
    } else if (auto* m7 = std::get_if<MResync>(&e.body)) {
        if (cfg_.isHost) onResync(e, *m7);
    } else if (auto* m8 = std::get_if<MAnimEvent>(&e.body)) {
        onAnimEvent(e, *m8);
    } else if (auto* m9 = std::get_if<MPart>(&e.body)) {
        onPart(e, *m9);
    } else if (auto* m10 = std::get_if<MUntrack>(&e.body)) {
        onUntrack(*m10);
    } else if (auto* m11 = std::get_if<MEvent>(&e.body)) {
        onEvent(e, *m11);
    }
}

// ---- the event bus (S6) ---------------------------------------------------------------------------------------------------------

bool ActorSync::sendOwnerEvent(NetId id, uint8_t kind, const Bytes& payload) {
    Rec* r = find(id);
    if (!r || payload.size() > 4096) return false;
    if (!MUT(80) && (view_.ownerOf(id) != cfg_.self || r->handingOff)) return false;   // MUT 80 BAD: anybody may send an owner event
    MEvent m;
    m.id = id; m.epoch = view_.epochOf(id); m.eventId = (static_cast<uint64_t>(cfg_.self) << 48) | ++busCounter_;
    m.kind = kind; m.route = EventRoute::Owner; m.origin = cfg_.self; m.payload = payload;
    busSeen_.insert(m.eventId);
    ++stats_.eventsSent;
    sendBody(m, kNoPlayer);
    return true;
}

bool ActorSync::sendToOwner(NetId id, uint8_t kind, const Bytes& payload) {
    if (!find(id) || payload.size() > 4096) return false;
    MEvent m;
    m.id = id; m.epoch = view_.epochOf(id); m.eventId = (static_cast<uint64_t>(cfg_.self) << 48) | ++busCounter_;
    m.kind = kind; m.route = EventRoute::ToOwner; m.origin = cfg_.self; m.payload = payload;
    ++stats_.eventsSent;
    if (view_.ownerOf(id) == cfg_.self) { deliverEvent(m, cfg_.self); return true; }
    if (cfg_.isHost) routeToOwner(m, cfg_.self);
    else sendBody(m, kHostPlayer);
    return true;
}

void ActorSync::deliverEvent(const MEvent& m, PlayerId from) {
    if (!MUT(81) && !busSeen_.insert(m.eventId).second && m.route != EventRoute::Nack) { ++stats_.eventsDuplicate; return; }   // MUT 81 BAD: a duplicate is applied twice
    while (busSeen_.size() > 8192) busSeen_.erase(busSeen_.begin());
    ++stats_.eventsDelivered;
    if (m.route == EventRoute::Nack) ++stats_.eventsNacked;
    auto h = eventHandlers_.find(m.kind);
    if (h == eventHandlers_.end() || !h->second) return;
    EventIn in;
    in.id = m.id; in.kind = m.kind; in.from = from; in.origin = m.origin; in.epoch = m.epoch; in.eventId = m.eventId;
    in.toOwner = m.route == EventRoute::ToOwner; in.nack = m.route == EventRoute::Nack; in.payload = m.payload;
    h->second(in);
}

// Host: hand a request to whoever owns the entity now. The sender's table may be behind (it asked an owner that gave the entity away), so
// the host decides from its own service, re-routes a request that comes back, and answers the origin with a Nack when nobody owns it.
void ActorSync::routeToOwner(MEvent m, PlayerId from) {
    const PlayerId owner = svc_ ? svc_->ownerOf(m.id) : view_.ownerOf(m.id);
    if (owner == cfg_.self) { deliverEvent(m, from); return; }
    if (owner == kNoPlayer || owner >= fnvcoop::kMaxSlots || !joined_.count(owner)) {
        if (MUT(82)) return;   // MUT 82 BAD: a request for an entity nobody owns vanishes without an answer
        MEvent n = m;
        n.route = EventRoute::Nack;
        if (m.origin == cfg_.self) deliverEvent(n, cfg_.self);
        else if (m.origin < fnvcoop::kMaxSlots && joined_.count(m.origin)) sendBody(n, m.origin);
        return;
    }
    if (m.hops >= 3) { ++stats_.eventsHopLimit; return; }
    if (from != cfg_.self && from != m.origin) { ++m.hops; ++stats_.eventsRerouted; }   // it came back from an old owner
    sendBody(m, owner);
}

void ActorSync::onEvent(const Envelope& e, const MEvent& m) {
    if (!find(m.id) && m.route != EventRoute::Nack) {   // an event can overtake the record it is about (late join): hold it briefly
        if (MUT(84)) { ++stats_.eventsRejected; return; }   // MUT 84 BAD: an event ahead of its record is lost
        if (heldEvents_.size() < 256) { heldEvents_.push_back(HeldEvent{e, now_}); ++stats_.eventsHeld; }
        else ++stats_.eventsRejected;
        return;
    }
    switch (m.route) {
        case EventRoute::Owner:
            if (cfg_.isHost) {
                const UpdateVerdict v = view_.check(m.id, e.src, m.epoch);
                if (v != UpdateVerdict::Accept && !MUT(85)) { ++stats_.eventsRejected; return; }   // MUT 85 BAD: a non-owner or a stale owner is believed
                relay(e, e.src);
            }
            deliverEvent(m, e.src);
            return;
        case EventRoute::ToOwner:
            if (m.origin >= fnvcoop::kMaxSlots) { ++stats_.spoofed; return; }   // (an old owner may hand back someone else's request: origin != src)
            if (cfg_.isHost) { routeToOwner(m, e.src); return; }
            if (view_.ownerOf(m.id) == cfg_.self) { deliverEvent(m, e.src); return; }
            if (MUT(83)) { ++stats_.eventsRejected; return; }   // MUT 83 BAD: a request that reached a past owner is dropped instead of re-routed
            sendBody(m, kHostPlayer);   // we gave it away meanwhile: back to the host, which knows the new owner
            return;
        case EventRoute::Nack:
            if (m.origin == cfg_.self) deliverEvent(m, e.src);
            return;
    }
}

void ActorSync::retryHeldEvents() {
    if (heldEvents_.empty()) return;
    std::vector<HeldEvent> keep;
    for (HeldEvent& h : heldEvents_) {
        const MEvent* m = std::get_if<MEvent>(&h.e.body);
        if (!m) continue;
        if (find(m->id) && view_.known(m->id)) { onEvent(h.e, *m); continue; }
        if (now_ > h.at + 2000) { ++stats_.eventsExpired; continue; }
        keep.push_back(std::move(h));
    }
    heldEvents_.swap(keep);
}

// ---- enemy-anim events (opaque payload, owner to everyone, reliable) -------------------------------------------------------------

bool ActorSync::sendAnimEvent(NetId id, const Bytes& payload) {
    Rec* r = find(id);
    if (!r || payload.empty() || payload.size() > 256) return false;
    if (!MUT(30) && view_.ownerOf(id) != cfg_.self) return false;   // MUT 30 BAD: anybody may send (the host would still judge it, a client's own view would not)
    MAnimEvent m;
    m.id = id; m.epoch = view_.epochOf(id); m.eventId = (static_cast<uint64_t>(cfg_.self) << 48) | ++animEventCounter_;
    m.payload = payload;
    animEvents_.insert(m.eventId);
    ++stats_.animEventsSent;
    sendBody(m, kNoPlayer);
    return true;
}

void ActorSync::onAnimEvent(const Envelope& e, const MAnimEvent& m) {
    if (!find(m.id) || find(m.id)->player) { ++stats_.animEventsRejected; return; }
    if (cfg_.isHost) {
        const UpdateVerdict v = view_.check(m.id, e.src, m.epoch);
        if (v != UpdateVerdict::Accept && !MUT(31)) { ++stats_.animEventsRejected; return; }   // MUT 31 BAD: a non-owner or stale-epoch event is relayed
        relay(e, e.src);
    }
    if (!MUT(32) && !animEvents_.insert(m.eventId).second) { ++stats_.animEventsDuplicate; return; }   // MUT 32 BAD: a duplicate delivery is applied twice
    while (animEvents_.size() > 4096) animEvents_.erase(animEvents_.begin());
    ++stats_.animEventsApplied;
    if (animHandler_) {
        AnimEventIn in;
        in.id = m.id; in.from = e.src; in.epoch = m.epoch; in.eventId = m.eventId; in.payload = m.payload;
        animHandler_(in);
    }
}

// ---- inventory (owner to everyone, reliable) --------------------------------------------------------------------------------------
// ---- state parts (state_parts.h): one replicator for everything the owner's engine changes by itself ---------------------------------
// The AI of an owned actor uses its items (a ganger drinks a Stimpak mid-fight) and draws the weapon it carries. A replicated AddItem /
// RemoveItem effect lands on every machine by itself, but the AI's own use happens on the owner only, so the copies kept the item
// (corpse_loot_race, game 2026-10-22: host 2 Stimpaks, client 3, before anyone looted), and a copy that was told "weapon drawn" without the
// weapon drew nothing and cowered (Goodsprings settler, game 2026-10-29). The owner reads each part of its actors, and a value that changed and
// then stayed the same for the part's settle time (for inventory long enough for a replicated effect to have landed everywhere, so it is not
// applied twice) is sent as the new truth. The first read of an actor is a silent baseline: both worlds start from the same save.

bool ActorSync::readPart(Rec& r, PartKind k, Bytes& out) {
    return portFor(r.meta.netId).readPart(r.meta.netId, k, out) && out.size() <= specFor(r, partIndex(k)).maxBytes;
}

// A player entity's calls go to the player port, everything else to the engine port. Without a player port a player record has no engine at all.
IActorPort& ActorSync::portFor(NetId id) {
    if (playerPort_ && live::isPlayerNet(id) && !MUT(49)) return *playerPort_;   // MUT 49 BAD: a call for a player id reaches the NPC port
    return port_;
}

bool ActorSync::publishPart(Rec& r, PartKind k, const Bytes& cur) {
    const NetId id = r.meta.netId;
    OwnerEpoch ep = 0;
    if (!view_.mayPublish(id, ep)) return false;
    MPart m;
    m.id = id; m.epoch = ep; m.eventId = (static_cast<uint64_t>(cfg_.self) << 48) | ++partEventCounter_;
    if (r.player && !MUT(73)) m.eventId |= static_cast<uint64_t>(ep & 0xFFFFu) << 32;   // a player who comes back runs a new ActorSync whose counter starts again: the epoch (it rises with every join) keeps its ids apart from the old session's, which the host still remembers (MUT 73 BAD: they collide and the rejoined player's parts are dropped as duplicates)
    m.kind = k; m.value = cur;
    partEvents_.insert(m.eventId);
    Rec::Part& p = r.parts[partIndex(k)];
    p.shared = cur; p.publishedAny = true; p.pendingValid = false;
    ++stats_.parts[partIndex(k)].published;
    if (specFor(r, partIndex(k)).audience == PartAudience::HostOnly && !MUT(51)) {
        if (!cfg_.isHost) sendBody(m, kHostPlayer);   // a guest's facts and pack go to the host only; the host keeps its own to itself
        return true;
    }
    if (!MUT(33)) sendBody(m, kNoPlayer);   // MUT 33 BAD: the owner keeps its changes to itself
    return true;
}

void ActorSync::pollParts(Rec& r) {
    const NetId id = r.meta.netId;
    if (r.dead || !(r.active || r.player) || r.handingOff || !portFor(id).loaded(id)) return;
    uint32_t& budget = r.player ? playerPartBudget_ : partBudget_;
    for (size_t k = 0; k < kPartCount; ++k) {
        if (!partApplies(r, k)) continue;
        Rec::Part& p = r.parts[k];
        const PartSpec& spec = specFor(r, k);
        if (now_ < p.nextMs || budget == 0) continue;
        --budget;
        p.nextMs = now_ + spec.pollMs;
        Bytes cur;
        if (!readPart(r, static_cast<PartKind>(k), cur)) continue;
        const bool silent = spec.silentBaseline || (MUT(50) && r.player);   // MUT 50 BAD: a player's first read is a silent baseline (every puppet stays a naked template)
        if (!p.base) {
            p.base = true; p.pendingValid = false;
            if (silent) { p.shared = cur; continue; }
            p.shared.clear();   // not silent: the first read is a change from "nothing"
        }
        const bool first = !silent && !p.publishedAny;
        if (cur == p.shared && !first) { p.pendingValid = false; continue; }
        if (!p.pendingValid || cur != p.pending) {
            p.pending = cur; p.pendingValid = true; p.pendingAt = now_;
            if (!MUT(35) && spec.settleMs > 0) continue;   // MUT 35 BAD: no settle time, the first sight of a change is published
        }
        if (MUT(35) || now_ >= p.pendingAt + spec.settleMs) publishPart(r, static_cast<PartKind>(k), cur);
    }
}

void ActorSync::flushParts(Rec& r) {
    const NetId id = r.meta.netId;
    if (!port_.loaded(id) || MUT(38)) return;   // MUT 38 BAD: a value still inside its settle time is lost at a handoff or a death
    for (size_t k = 0; k < kPartCount; ++k) {
        Rec::Part& p = r.parts[k];
        if (!p.base || !partApplies(r, k)) continue;
        Bytes cur;
        if (!readPart(r, static_cast<PartKind>(k), cur)) continue;
        if (cur != p.shared) publishPart(r, static_cast<PartKind>(k), cur);
    }
}

// A clone is a fresh engine object, and a real ref that comes back after its cell unloaded was rebuilt from its base form: either is dressed as the base
// form says, not as the owner's AI left it. The parts that say so (PartSpec::reapplyOnLoad) are written again from the last value known.
void ActorSync::rewantParts(Rec& r) {
    if (MUT(39)) return;   // MUT 39 BAD: a clone or a rebuilt NPC keeps the engine's base outfit (it draws nothing)
    if (MUT(54) && r.player) return;   // MUT 54 BAD: a puppet that was made again keeps the "already applied" memory (it stays a naked template)
    for (size_t k = 0; k < kPartCount; ++k) {
        Rec::Part& p = r.parts[k];
        if (!partApplies(r, k)) continue;
        const PartSpec& spec = specFor(r, k);
        if (spec.reapplyOnLoad && spec.audience == PartAudience::All && p.publishedAny && !p.wantValid) { p.want = p.shared; p.wantValid = true; p.tries = 0; p.nextMs = 0; }
    }
}

void ActorSync::applyWantedParts(Rec& r) {
    const NetId id = r.meta.netId;
    if (r.dead) return;
    IActorPort& pt = portFor(id);
    if (!pt.loaded(id)) return;
    for (size_t k = 0; k < kPartCount; ++k) {
        Rec::Part& p = r.parts[k];
        if (!p.wantValid || now_ < p.nextMs) continue;
        p.nextMs = now_ + specFor(r, k).pollMs;
        if (pt.writePart(id, static_cast<PartKind>(k), p.want)) { p.wantValid = false; p.tries = 0; ++stats_.parts[k].applied; continue; }
        ++stats_.parts[k].applyFailed;
        if (++p.tries >= 5) { p.wantValid = false; p.tries = 0; }   // never hammer an actor the engine refuses
    }
}

// The local player's own parts are read and published again: a new host (or a host that never heard of this client) has none of them.
void ActorSync::resetOwnParts() {
    Rec* r = find(live::playerNet(cfg_.self));
    if (!r || !r->player) return;
    for (size_t k = 0; k < kPartCount; ++k) {
        Rec::Part& p = r->parts[k];
        if (!p.base && !p.publishedAny) continue;
        p.base = false; p.publishedAny = false; p.pendingValid = false; p.nextMs = 0;
        ++stats_.playerPartsRepublished;
    }
}

void ActorSync::onPart(const Envelope& e, const MPart& m) {
    const size_t k = partIndex(m.kind);
    Rec* r = find(m.id);
    if (!r) { ++stats_.parts[k].rejected; return; }
    if (!partApplies(*r, k) && !(MUT(56) && !r->player)) { ++stats_.parts[k].rejected; return; }   // MUT 56 BAD: an NPC record takes a player part (a look, a pack)
    if (r->player && !cfg_.playerParts) { ++stats_.parts[k].rejected; return; }                  // the avatar channel carries these in this session
    const PartSpec& spec = specFor(*r, k);
    if (cfg_.isHost) {
        const UpdateVerdict v = view_.check(m.id, e.src, m.epoch);
        if (v != UpdateVerdict::Accept && !MUT(36) && !(MUT(53) && r->player)) { ++stats_.parts[k].rejected; return; }   // MUT 36 BAD: a non-owner's value is believed and relayed; 53 BAD: ... for a player's parts
        if (spec.audience == PartAudience::All || MUT(51)) relay(e, e.src);   // MUT 51 BAD: the host relays a guest's facts and pack to the other guests
    } else if (spec.audience == PartAudience::HostOnly) {
        ++stats_.parts[k].rejected;   // only the host hears these
        return;
    }
    if (!partEvents_.insert(m.eventId).second) { ++stats_.parts[k].duplicate; return; }
    while (partEvents_.size() > 4096) partEvents_.erase(partEvents_.begin());
    if (ownedByMe(*r) && !MUT(36)) { ++stats_.parts[k].rejected; return; }   // we are the truth for this actor
    Rec::Part& p = r->parts[k];
    p.shared = m.value; p.publishedAny = true; p.base = false;
    if (r->dead) return;   // a corpse's contents belong to the loot service
    if (MUT(34)) return;   // MUT 34 BAD: the value is received and never written
    p.want = m.value; p.wantValid = true; p.tries = 0; p.nextMs = 0;
    applyWantedParts(*r);
}

void ActorSync::onTrack(PlayerId from, const MTrack& m) {
    if (m.meta.netId == 0) return;
    const bool playerMeta = live::isPlayerNet(m.meta.netId) || live::isPlayerFormRef(m.meta.ref);
    if (playerMeta && (cfg_.isHost || !live::isPlayerNet(m.meta.netId) || !(m.meta.ref == live::playerFormRef(live::slotOfPlayerNet(m.meta.netId))))) {
        ++stats_.playerRefused;   // a client claiming a player entity, or a player meta that does not name itself
        return;
    }
    auto byRef = refToNet_.find(m.meta.ref);
    auto byId = recs_.find(m.meta.netId);
    if ((byRef != refToNet_.end() && byRef->second != m.meta.netId) || (byId != recs_.end() && !(byId->second.meta.ref == m.meta.ref))) {
        ++stats_.idCollisions;
        return;
    }
    const bool fresh = byId == recs_.end();
    Rec* r = ensureRec(m.meta, playerMeta && live::slotOfPlayerNet(m.meta.netId) == cfg_.self);
    if (fresh) { r->latest.cell = m.meta.cell; r->latest.pos = m.meta.pos; }
    if (fresh && playerMeta) ++stats_.playersTracked;
    if (fresh) feedEarly(m.meta.netId, static_cast<uint32_t>(now_));
    if (cfg_.isHost && fresh) {
        Envelope e;
        e.src = from; e.dest = kNoPlayer; e.body = m;
        relay(e, from);   // every other client learns the actor too (it may need a stand-in)
        hostRegister(*r);
    }
}

void ActorSync::onResync(const Envelope& e, const MResync& m) {
    if (m.id == 0) {   // the whole table (NetId 0 is never a tracked actor)
        if (svc_ && joined_.count(e.src) && !MUT(24)) { ++stats_.resyncsAnswered; sendJoinState(e.src); }
        return;
    }
    OwnMsg cur;
    if (svc_ && svc_->current(m.id, cur)) { ++stats_.resyncsAnswered; sendBody(MOwn{cur}, e.src); }
}

// ---- death -----------------------------------------------------------------------------------------------------------------

void ActorSync::applyDeath(Rec& r, bool engineKills) {
    if (r.dead) return;
    const NetId id = r.meta.netId;
    r.dead = true;
    ++stats_.deathsApplied;
    r.active = false; r.handingOff = false; r.awaitPrev = kNoPlayer;
    refreshAi(r);
    dropStandIn(r);
    if (engineKills) {
        if (port_.loaded(id)) port_.kill(id);
        else r.killPending = true;   // applied when the actor loads (syncToView)
    }
    // The ownership record is KEPT (not untracked): a scripted ResurrectActor must continue the same epoch history, otherwise the re-created
    // record would restart at epoch 1 and every view would call its Grants stale. Dead actors are skipped by evalOwnership().
    if (MUT(21) && svc_) {   // BAD: forget the ownership record (the epoch history restarts when the actor comes back)
        std::vector<OwnMsg> out;
        svc_->untrack(id, out);
        emitOwn(out);
    }
    interest_.removeActor(id);
}

bool ActorSync::onLocalDeath(NetId id) {
    Rec* r = find(id);
    if (!r || r->dead || r->player) return true;
    if (r->meta.named && !r->killAuthorized && !MUT(10)) { ++stats_.namedDeathsBlocked; return false; }   // story NPCs die only by KillActor
    if (!r->killAuthorized && !ownedByMe(*r) && !MUT(10)) return false;   // only the owner declares death
    MDeath m;
    m.id = id; m.epoch = view_.epochOf(id); m.eventId = (static_cast<uint64_t>(cfg_.self) << 48) | ++eventCounter_;
    m.cell = r->haveLatest ? r->latest.cell : r->meta.cell; m.pos = r->haveLatest ? r->latest.pos : r->meta.pos;
    deathEvents_.insert(m.eventId);
    if (ownedByMe(*r)) flushParts(*r);   // what it used up to its last breath; after this the corpse is the loot service's
    sendBody(m, kNoPlayer);
    applyDeath(*r, false);   // the engine is already killing it
    return true;
}

void ActorSync::onDeath(const Envelope& e, const MDeath& m) {
    Rec* r = find(m.id);
    if (!r || r->player) return;
    if (r->meta.named && !r->killAuthorized && !MUT(9)) { ++stats_.namedDeathsBlocked; return; }   // a named NPC's death is not an owner's call
    if (cfg_.isHost) {
        const UpdateVerdict v = view_.check(m.id, e.src, m.epoch);
        if (v != UpdateVerdict::Accept) {
            if (v == UpdateVerdict::StaleEpoch) ++stats_.staleEpochDropped; else ++stats_.notOwnerDropped;
            return;
        }
        relay(e, e.src);
    }
    if (!deathEvents_.insert(m.eventId).second) return;   // duplicate delivery
    applyDeath(*r, true);
}

// ---- story effects ---------------------------------------------------------------------------------------------------------

void ActorSync::onEffect(const fnvcoop::Effect& e) {
    if (auto* k = std::get_if<fnvcoop::KillActor>(&e)) {
        NetId id = 0;
        if (!netIdOf(k->actor, id)) return;
        if (MUT(20) && !cfg_.isHost) return;   // BAD: only the host kills
        Rec* r = find(id);
        if (!r || r->dead) return;
        r->killAuthorized = true;
        ++stats_.killsAuthorised;
        if (r->protectedOn && port_.loaded(id)) { port_.setProtected(id, false); r->protectedOn = false; }
        applyDeath(*r, true);
    } else if (auto* rz = std::get_if<fnvcoop::ResurrectActor>(&e)) {
        NetId id = 0;
        if (!netIdOf(rz->actor, id)) return;
        Rec* r = find(id);
        if (!r || !r->dead) return;
        r->dead = false; r->killAuthorized = false;
        if (port_.loaded(id)) port_.revive(id);
        r->aiKnown = false;
        if (MUT(21) && cfg_.isHost) hostRegister(*r);
        if (cfg_.isHost) nextEvalMs_ = 0;   // re-decide at the next tick (the record and its epoch history were kept)
        syncToView(*r);
    } else if (auto* c = std::get_if<fnvcoop::CompanionOwner>(&e)) {
        if (MUT(11) || !cfg_.isHost) return;   // BAD (11): the follower exception is never applied
        const PlayerId recruiter = c->active ? c->owner : kNoPlayer;
        NetId id = 0;
        if (!netIdOf(c->npc, id)) { pendingRecruiter_[c->npc] = recruiter; return; }
        if (isPlayerRecord(id)) return;
        std::vector<OwnMsg> out;
        svc_->setRecruiter(id, recruiter, presenceFor(*find(id)), static_cast<uint32_t>(now_), out);
        ++stats_.recruiterChanges;
        emitOwn(out);
    }
}

// ---- leveled pick ----------------------------------------------------------------------------------------------------------

void ActorSync::applyPick(Rec& r, const FormRef& pick) {
    if (MUT(12)) return;   // BAD: the pick is never applied
    if (r.meta.base == pick) return;
    if (!port_.loaded(r.meta.netId)) return;   // applied when the actor loads (onActorLoaded -> considerPick)
    port_.setBase(r.meta.netId, pick);
    r.meta.base = pick;
    ++stats_.picksApplied;
}

void ActorSync::hostSetPick(Rec& r, const FormRef& pick) {
    if (r.havePick && !MUT(13)) { ++stats_.picksRejected; return; }   // first pick wins, forever
    r.havePick = true; r.pick = pick;
    sendBody(MPick{r.meta.netId, pick}, kNoPlayer);
    applyPick(r, pick);
}

void ActorSync::considerPick(Rec& r) {
    if (r.havePick) { applyPick(r, r.pick); return; }
    if (!r.local || !ownedByMe(r) || r.meta.base.plugin.empty()) return;   // only an owner's native roll can become the pick
    if (cfg_.isHost) hostSetPick(r, r.meta.base);
    else if (hostUp_ && !r.pickReported) { r.pickReported = true; sendBody(MPickReport{r.meta.netId, r.meta.base}, kHostPlayer); }
}

void ActorSync::onPick(const MPick& m) {
    Rec* r = find(m.id);
    if (!r) { pendingPick_[m.id] = m.pick; return; }
    r->havePick = true; r->pick = m.pick;
    applyPick(*r, m.pick);
}

void ActorSync::onPickReport(const Envelope& e, const MPickReport& m) {
    Rec* r = find(m.id);
    if (!r) return;
    if (view_.ownerOf(m.id) != e.src && !MUT(13)) { ++stats_.picksRejected; return; }   // only the current owner's roll counts
    hostSetPick(*r, m.base);
}

// ---- tick ------------------------------------------------------------------------------------------------------------------

void ActorSync::tick(uint64_t nowMs) {
    now_ = nowMs;
    const uint32_t now = static_cast<uint32_t>(nowMs);
    if (svc_) {
        std::vector<OwnMsg> out;
        svc_->tick(now, out);
        emitOwn(out);
        if (now_ >= nextEvalMs_) { nextEvalMs_ = now_ + cfg_.ownEvalMs; evalOwnership(); }
        trackPlayer(cfg_.self);   // a solo host too (cheap when it is pinned already)
    }
    partBudget_ = 4 * static_cast<uint32_t>(kPartCount);
    playerPartBudget_ = 2 * static_cast<uint32_t>(kPartCount);
    avAuditBudget_ = 4;
    for (auto& kv : recs_) {
        Rec& r = kv.second;
        if (r.dead) continue;
        if (r.player && !MUT(48)) { tickPlayerRec(r); continue; }   // MUT 48 BAD: the NPC tick (AI, takeover, stand-ins, parts) runs on a player record
        const NetId id = kv.first;
        const bool owner = ownedByMe(r);
        if (r.awaitPrev != kNoPlayer && (now_ >= r.awaitSince + cfg_.holdAiMaxMs || !playerHere(r.awaitPrev))) releaseAiHold(r);   // safety net / the old owner is gone
        if (owner && !r.active && !r.handingOff && port_.loaded(id)) takeOver(r, view_.epochOf(id));   // the engine got the actor after the Grant
        if (MUT(2) ? (r.local && port_.loaded(id)) : (owner && r.active)) publishOwned(r, now);
        else if (!owner) { driveNonOwner(r, now); auditAvs(r); }
        if (owner) pollParts(r);
        else applyWantedParts(r);
    }
    retryHeldEvents();
    for (auto it = early_.begin(); it != early_.end();) {   // nothing claimed it: an actor that never reached this machine
        if (now_ > it->second.at + 5000) { ++stats_.earlyDropped; it = early_.erase(it); } else ++it;
    }
    relayToClients(now);
    flushPeers(now);
    if (cfg_.playerStream && playerPort_) {   // the player stream: its own sender, receiver and planner
        samplePlayer(now);
        releasePlayerPending(now);
        flushPlayerPeers(now);
    }
}

// A player record has no AI, no takeover, no stand-in: its whole tick is its parts. The owner (the pinned machine) reads and publishes them; the others write
// what they received onto the puppet (or, for the host-only parts, keep it for the party conditions).
void ActorSync::tickPlayerRec(Rec& r) {
    if (!cfg_.playerParts || !playerPort_) return;
    if (ownedByMe(r)) pollParts(r);
    else applyWantedParts(r);
}

void ActorSync::holdEarly(PlayerId from, const live::ActorSnapshot& s) {
    if (MUT(47)) return;
    auto it = early_.find(s.netId);
    if (it != early_.end() && !live::serialLess(it->second.s.ownerEpoch, s.ownerEpoch) &&
        !(it->second.s.ownerEpoch == s.ownerEpoch && live::serialLess(it->second.s.seq, s.seq))) return;   // keep the newest
    if (it == early_.end() && early_.size() >= 512) { ++stats_.earlyDropped; return; }
    if (it == early_.end()) ++stats_.earlyHeld;
    early_[s.netId] = Early{from, s, now_};
}

void ActorSync::feedEarly(NetId id, uint32_t now) {
    auto it = early_.find(id);
    if (it == early_.end()) return;
    Rec* r = find(id);
    if (!r || !view_.known(id)) return;   // still waiting for the record or its ownership
    const Early e = it->second;
    early_.erase(it);
    if (r->player) { applyEarlyPlayer(*r, e, now); return; }
    const UpdateVerdict v = cfg_.isHost ? view_.check(id, e.from, e.s.ownerEpoch) : view_.check(id, view_.ownerOf(id), e.s.ownerEpoch);
    if (v != UpdateVerdict::Accept) { ++stats_.earlyDropped; return; }
    if (acceptSnapshot(*r, e.s, now)) ++stats_.earlyApplied;
}

// ---- queries ---------------------------------------------------------------------------------------------------------------

OwnerInfo ActorSync::ownerOf(NetId id) const {
    OwnerInfo o;
    o.self = cfg_.self;
    const Rec* r = find(id);
    if (!r) return o;
    o.known = true; o.netId = id;
    o.owner = view_.ownerOf(id);
    o.epoch = view_.epochOf(id);
    if (MUT(16)) ++o.epoch;   // BAD: wrong epoch reported
    o.mine = o.owner == cfg_.self;
    o.dead = r->dead;
    o.aiOn = r->aiKnown && r->aiApplied;
    o.standIn = r->standIn;
    OwnerEpoch pe = 0;
    if (view_.handoffIncoming(id, pe)) { o.pending = true; o.pendingTo = cfg_.self; }
    else if (cfg_.isHost && svc_ && svc_->handoffPending(id)) o.pending = true;
    else if (r->handingOff) o.pending = true;
    return o;
}

bool ActorSync::netIdOf(const FormRef& ref, NetId& out) const {
    auto it = refToNet_.find(ref);
    if (it == refToNet_.end()) return false;
    out = it->second;
    return true;
}

bool ActorSync::isStandIn(NetId id) const { const Rec* r = find(id); return r && r->standIn; }
bool ActorSync::isDead(NetId id) const { const Rec* r = find(id); return r && r->dead; }

std::vector<NetId> ActorSync::standIns() const {
    std::vector<NetId> v;
    for (auto& kv : recs_) if (kv.second.standIn) v.push_back(kv.first);
    return v;
}

std::vector<NetId> ActorSync::ownedActors() const {
    std::vector<NetId> v;
    view_.forEach([&](fnvcoop::ActorNetId id, PlayerId owner, OwnerEpoch) { if (owner == cfg_.self) v.push_back(id); });
    return v;
}

FormRef ActorSync::pickOf(NetId id) const { const Rec* r = find(id); return r && r->havePick ? r->pick : FormRef(); }
FormRef ActorSync::baseOf(NetId id) const { const Rec* r = find(id); return r ? r->meta.base : FormRef(); }

bool ActorSync::latestState(NetId id, live::ActorSnapshot& out) const {
    const Rec* r = find(id);
    if (!r || !r->haveLatest) return false;
    out = fullState(*r);
    return true;
}

}  // namespace nvcoop::actors
