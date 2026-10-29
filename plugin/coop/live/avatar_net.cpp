#include "avatar_net.h"

#include <algorithm>
#include <ctime>

#include "look_codec.h"
#include "fnvcoop_mut.h"   // FNV_MUT(n): twins 41..51 and 1170..1179 of the live wiring suite, 105..110 of the avatar suite (plugin/avatar/full)

namespace nvcoop::livewire {

namespace live = fnvcoop::live;

namespace {
uint32_t defaultEpoch() { return static_cast<uint32_t>(std::time(nullptr)); }
Bytes withChannel(const Bytes& liveDatagram) {
    Bytes b;
    b.reserve(liveDatagram.size() + 1);
    b.push_back(kChannelAvatars);
    b.insert(b.end(), liveDatagram.begin(), liveDatagram.end());
    return b;
}
live::ReceiverConfig receiverConfig() {
    live::ReceiverConfig c;
    c.checkSenderIsOwner = false;   // players have no ownership records; the netId-vs-sender check is done here
    c.maxActors = 64;
    return c;
}
nvcoop::avatar::StreamRateConfig rateConfig(nvcoop::avatar::StreamRateConfig c) {
    if (FNV_MUT(209)) c.jumpUnits = 0;   // MUTANT 209: a teleport is held for the tier's period like any other snapshot (the far tier's 200 ms, the out tier's 1 s)
    return c;
}
bool sameAppearance(const nvcoop::avatar::Appearance& a, const nvcoop::avatar::Appearance& b) {
    return a.race == b.race && a.female == b.female && a.hair == b.hair && a.eyes == b.eyes && a.faceHash == b.faceHash &&
           a.hairColor == b.hairColor && a.height == b.height && a.weight == b.weight && a.faceCoeffs == b.faceCoeffs;
}
}  // namespace

AvatarNet::AvatarNet(IAvatarSide& side, AvatarNetOptions opt, std::function<uint32_t()> epochSeed)
    : side_(side), opt_(std::move(opt)), epochSeed_(epochSeed ? std::move(epochSeed) : std::function<uint32_t()>(defaultEpoch)),
      rate_(rateConfig(opt_.rate)), sender_(opt_.sender), recv_(receiverConfig()) {}

AvatarNet::~AvatarNet() = default;

std::vector<PlayerId> AvatarNet::peers() const {
    std::vector<PlayerId> v;
    if (isHost_) v.assign(joined_.begin(), joined_.end());
    else if (hostUp_) v.push_back(kHostPlayer);
    return v;
}

void AvatarNet::sendTo(PlayerId peer, bool reliable, Bytes data) {
    if (link_) link_->send(peer, reliable, std::move(data));
}

void AvatarNet::broadcastReliable(const Bytes& data, PlayerId except) {
    if (!isHost_) return;
    for (PlayerId p : joined_)
        if (p != except) sendTo(p, true, data);
}

// ---------------------------------------------------------------------------------------------------------------------------------
// session
// ---------------------------------------------------------------------------------------------------------------------------------
void AvatarNet::sessionBegin(const SessionInfo& info, ILiveLink& link) {
    link_ = &link;
    inSession_ = true;
    isHost_ = info.isHost;
    self_ = info.self;
    hostUp_ = false;
    epoch_ = std::max<uint32_t>(1, epochSeed_());
    roster_.clear();
    joined_.clear();
    looks_.clear();
    lastSnap_.clear();
    reportedSnap_.clear();
    applied_.clear();
    rx_.clear();
    rate_.clear();
    haveLocalApp_ = false;
    lastAppRead_ = 0;
    sender_ = live::SnapshotSender(opt_.sender);
    recv_ = live::SnapshotReceiver(receiverConfig());
    localOut_.clear();
    queuedTicks_.clear();
    early_.clear();
    remoteEquip_.clear();
    samples_.clear();
    haveEquip_ = false;
    lookSentValid_ = false;
    packSentValid_ = false;
    watchLoaded_ = false;
    watchTriedAt_ = 0;
    lastAck_ = lastLookCheck_ = lastPoseReport_ = 0;
    haveLocal_ = reportedLocalValid_ = false;
    if (!FNV_MUT(110) && self_ != kNoPlayer) side_.setLocalSlot(static_cast<nvcoop::avatar::Slot>(self_));   // MUTANT 110: the avatar layer is never told its slot (a client keeps the ini default 0 and refuses the host's puppet)
    side_.setLocalIdentity(playerNetId(self_), epoch_);
    side_.setSnapshotSink([this](const live::ActorSnapshot& s) { localOut_.push_back(s); });
    side_.setEquipSink([this](const std::vector<nvcoop::avatar::EquipEntry>& e) { equip_ = e; haveEquip_ = true; });
}

void AvatarNet::sessionEnd() {
    side_.setSnapshotSink(nullptr);
    side_.setEquipSink(nullptr);
    for (PlayerId s : std::vector<PlayerId>(roster_.begin(), roster_.end())) {
        side_.removeRemote(s);
        if (poseSink_) poseSink_(s, false, 0, live::Vec3());
    }
    if (poseSink_ && reportedLocalValid_) poseSink_(self_, false, 0, live::Vec3());
    reportedLocalValid_ = haveLocal_ = false;
    if (opt_.tickSide) side_.tick(static_cast<uint32_t>(nowMs_));   // let the removals reach the engine
    roster_.clear();
    joined_.clear();
    looks_.clear();
    facts_.clear();
    factsSentValid_ = false;
    packs_.clear();
    packSentValid_ = false;
    lastSnap_.clear();
    reportedSnap_.clear();
    applied_.clear();
    rx_.clear();
    rate_.clear();
    localOut_.clear();
    queuedTicks_.clear();
    early_.clear();
    remoteEquip_.clear();
    samples_.clear();
    inSession_ = false;
    hostUp_ = false;
    link_ = nullptr;
}

void AvatarNet::addPuppet(PlayerId slot) {
    if (slot >= fnvcoop::kMaxSlots) return;
    if (slot == self_) {   // the local player never gets a puppet
        if (FNV_MUT(48)) roster_.insert(slot);   // MUTANT 48: the local player is listed in the roster like everybody else
        return;
    }
    roster_.insert(slot);
    auto it = looks_.find(slot);
    const nvcoop::avatar::Appearance app = it != looks_.end() ? it->second.appearance : nvcoop::avatar::Appearance{};
    side_.addRemote(slot, app);
    applied_[slot] = app;
    if (auto e = early_.find(slot); e != early_.end()) {
        side_.onRemoteSnapshot(slot, e->second, static_cast<uint32_t>(nowMs_));
        lastSnap_[slot] = e->second;
        rate_.setPlace(slot, e->second.cell, e->second.pos);
        early_.erase(e);
    }
    if (it != looks_.end()) side_.onRemoteEquipment(slot, it->second.equipment);
}

void AvatarNet::removePuppet(PlayerId slot) {
    if (!FNV_MUT(43)) side_.removeRemote(slot);   // MUTANT 43: the puppet of a player who left stays in the world
    roster_.erase(slot);
    looks_.erase(slot);
    facts_.erase(slot);
    packs_.erase(slot);
    lastSnap_.erase(slot);
    reportedSnap_.erase(slot);
    applied_.erase(slot);
    rx_.erase(slot);
    early_.erase(slot);
    remoteEquip_.erase(slot);
    rate_.removePlace(slot);
    queuedTicks_.erase(slot);
    recv_.forget(playerNetId(slot));
    sender_.forgetPeer(slot);
    if (poseSink_) poseSink_(slot, false, 0, live::Vec3());
}

void AvatarNet::sendRoster(PlayerId to) {
    if (!isHost_) return;
    if (opt_.partsUnified && !FNV_MUT(1170)) return;   // the roster is the set of player records (MUTANT 1170: the old message is sent too, two carriers for one fact)
    std::vector<uint8_t> slots{kHostPlayer};
    for (PlayerId p : joined_) slots.push_back(p);
    Bytes b = encodeRoster(slots);
    ++stats_.rostersSent;
    if (to == kNoPlayer) broadcastReliable(b, kNoPlayer);
    else sendTo(to, true, b);
}

void AvatarNet::sendLook(const AvLook& l, PlayerId to, PlayerId skip) {
    if (opt_.partsUnified && !FNV_MUT(1170)) return;   // look and equipment are parts of the player entity
    Bytes b = encodeLook(l);
    ++stats_.looksSent;
    if (to == kNoPlayer) broadcastReliable(b, skip);
    else sendTo(to, true, b);
}

void AvatarNet::playerJoined(PlayerId slot) {
    if (!isHost_ || slot == self_ || slot >= fnvcoop::kMaxSlots) return;
    joined_.insert(slot);
    sender_.forgetPeer(slot);
    recv_.forget(playerNetId(slot));
    addPuppet(slot);
    sendRoster(kNoPlayer);
    // the newcomer learns what everybody looks like (the host's own and the others' relayed ones)
    for (const auto& kv : looks_)
        if (kv.first != slot) sendLook(kv.second, slot, kNoPlayer);
}

void AvatarNet::playerLeft(PlayerId slot) {
    if (!isHost_) return;
    joined_.erase(slot);
    removePuppet(slot);
    sendRoster(kNoPlayer);
}

void AvatarNet::hostLinkUp() {
    if (isHost_) return;
    hostUp_ = true;
    sender_.forgetPeer(kHostPlayer);
    lookSentValid_ = false;   // the host (maybe a new one) learns our look again
    factsSentValid_ = false;  // ... and our facts
    packSentValid_ = false;   // ... and our pack
    if (!opt_.partsUnified || FNV_MUT(1170)) sendTo(kHostPlayer, true, Bytes{kChannelAvatars, kAvKindResync});   // and we learn the roster and every look again (parts=unified: ActorSync asks for its own join state)
}

void AvatarNet::hostLinkDown() {
    if (isHost_) return;
    hostUp_ = false;
    for (PlayerId s : std::vector<PlayerId>(roster_.begin(), roster_.end())) removePuppet(s);   // the roster comes back with the link
    recv_ = live::SnapshotReceiver(receiverConfig());
}

void AvatarNet::applyRoster(const std::vector<uint8_t>& slots) {
    if (FNV_MUT(44)) return;   // MUTANT 44: the roster is ignored (a client never learns who exists)
    std::set<PlayerId> want;
    for (uint8_t s : slots) want.insert(s);
    for (PlayerId s : std::vector<PlayerId>(roster_.begin(), roster_.end()))
        if (!want.count(s)) removePuppet(s);
    for (PlayerId s : want)
        if (!roster_.count(s)) addPuppet(s);
    ++stats_.rostersApplied;
}

// ---------------------------------------------------------------------------------------------------------------------------------
// receive
// ---------------------------------------------------------------------------------------------------------------------------------
void AvatarNet::receive(PlayerId from, const uint8_t* data, size_t size) {
    if (!inSession_ || size < 2 || data[0] != kChannelAvatars) { ++stats_.badDatagrams; return; }
    ++stats_.received;
    const uint8_t kind = data[1];
    if (opt_.partsUnified && FNV_MUT(1171) && (kind == kAvKindRoster || kind == kAvKindLook || kind == kAvKindLook2 || kind == kAvKindFacts || kind == kAvKindPack)) {
        ++stats_.badDatagrams;   // MUTANT 1171: the old messages are not understood under parts=unified (the decoders must accept both forms)
        return;
    }
    if (kind == static_cast<uint8_t>(live::MsgKind::SnapshotBatch)) {
        onSnapshotDatagram(from, data + 1, size - 1);
    } else if (kind == static_cast<uint8_t>(live::MsgKind::SnapshotAck)) {
        if (sender_.onAckPacket(from, data + 1, size - 1)) ++stats_.acksReceived;
        else ++stats_.badDatagrams;
    } else if (kind == kAvKindRoster) {
        std::vector<uint8_t> slots;
        if (isHost_ || from != kHostPlayer || !decodeRoster(data, size, slots)) { ++stats_.badDatagrams; return; }   // only the host sends rosters
        applyRoster(slots);
    } else if (kind == kAvKindResync) {
        if (!isHost_ || size != 2 || !joined_.count(from)) { ++stats_.badDatagrams; return; }
        if (FNV_MUT(51)) return;   // MUTANT 51: the host ignores a client's resync request (a returning link never gets the roster back)
        sendRoster(from);
        for (const auto& kv : looks_)
            if (kv.first != from) sendLook(kv.second, from, kNoPlayer);
        ++stats_.resyncsAnswered;
    } else if (kind == kAvKindLook || kind == kAvKindLook2) {
        onLook(from, data, size);
    } else if (kind == kAvKindFacts) {
        onFacts(from, data, size);
    } else if (kind == kAvKindPack) {
        onPack(from, data, size);
    } else {
        ++stats_.badDatagrams;
    }
}

void AvatarNet::onSnapshotDatagram(PlayerId from, const uint8_t* body, size_t size) {
    auto results = recv_.receive(from, body, size);
    for (const live::ReceivedSnapshot& r : results) {
        if (r.verdict != live::Verdict::Accepted) { ++stats_.rejected; continue; }
        const live::ActorSnapshot& s = r.snap;
        if (!isPlayerNetId(s.netId)) { ++stats_.rejected; continue; }
        const PlayerId slot = slotOfPlayerNetId(s.netId);
        if (isHost_ && slot != from && !FNV_MUT(42)) { ++stats_.forged; continue; }   // MUTANT 42: a client may publish another player's avatar
        if (slot == self_) { ++stats_.rejected; continue; }   // our own stream echoed back
        if (!isHost_ && from != kHostPlayer) { ++stats_.rejected; continue; }
        if (!roster_.count(slot)) {
            // The receiver acked this snapshot already (inside recv_.receive), so the sender takes it for delivered and, for a player who stands still, sends
            // nothing but a heartbeat a second from now on. Dropped here for good, the puppet that the roster creates a moment later would have no place until
            // that heartbeat (or, behind a lossy link, until the next one): lab spawn room, "the puppet arrives 650 to 2700 ms late, 773 units away". Kept
            // (the newest per player) and fed to the puppet the moment it exists.
            ++stats_.noRoster;
            if (!FNV_MUT(212)) early_[slot] = s;   // MUTANT 212: a snapshot ahead of the roster is dropped for good (the puppet waits for the next heartbeat)
            continue;
        }
        side_.onRemoteSnapshot(slot, s, static_cast<uint32_t>(nowMs_));
        side_.noteStage(nvcoop::avatar::NetStage::Rx, static_cast<double>(static_cast<int32_t>(clock() - s.tick)));
        lastSnap_[slot] = s;
        rate_.setPlace(slot, s.cell, s.pos);
        ++stats_.accepted;
        ++rx_[slot];
        if (isHost_ && !FNV_MUT(41)) {   // MUTANT 41: the host does not relay (a client never sees the other clients)
            live::ActorSnapshot relay = s;
            relay.avDeltas = r.avAll;
            if (FNV_MUT(213)) { relay.anim.action = 0; relay.anim.block = false; }   // twin 213: the relay drops the player's one-shot (a third machine never sees the attack)
            for (PlayerId c : joined_)
                if (c != slot) queueTo(c, slot, relay, nowMs_);
        }
    }
}

// Who our own stream is relayed to by the host: every other player we know of (the host itself included).
std::vector<PlayerId> AvatarNet::uplinkObservers() const {
    std::vector<PlayerId> v;
    for (PlayerId p : roster_)
        if (p != self_) v.push_back(p);
    if (!isHost_ && std::find(v.begin(), v.end(), kHostPlayer) == v.end()) v.push_back(kHostPlayer);
    return v;
}

// Queues one snapshot of `subject` for `peer` through the near / far rate. A snapshot that is not due yet is kept (latest wins) and
// released by releasePending() when its turn comes. `peer == kNoPlayer` is the uplink of the local stream (to the host).
void AvatarNet::queueTo(PlayerId peer, PlayerId subject, const live::ActorSnapshot& snap, uint64_t nowMs) {
    live::ActorSnapshot out;
    bool now = false;
    if (FNV_MUT(107)) {   // twin 107: the rate is ignored (everything goes at full rate to everybody)
        out = snap;
        now = true;
    } else if (peer == kNoPlayer) {
        const std::vector<PlayerId> obs = uplinkObservers();
        if (FNV_MUT(108)) {   // twin 108: the uplink follows the LOWEST tier of its observers (a near neighbour would see 5 or 1 Hz)
            nvcoop::avatar::StreamTier lo = nvcoop::avatar::StreamTier::Near;
            for (PlayerId o : obs) {
                nvcoop::avatar::StreamTier t = rate_.tierOf(o, subject);
                if (static_cast<uint8_t>(t) < static_cast<uint8_t>(lo)) lo = t;
            }
            now = rate_.offerAtTier(nvcoop::avatar::StreamRatePlanner::kUplink, subject, lo, snap, static_cast<uint32_t>(nowMs), out);
        } else {
            now = rate_.offerUplink(subject, obs, snap, static_cast<uint32_t>(nowMs), out);
        }
    } else {
        now = rate_.offer(peer, subject, snap, static_cast<uint32_t>(nowMs), out);
    }
    if (!now) {
        ++stats_.rateKept;
        return;
    }
    const PlayerId to = peer == kNoPlayer ? kHostPlayer : peer;
    if (sender_.queue(to, out, static_cast<uint32_t>(nowMs))) {
        if (subject != self_) ++stats_.relayed;
        noteQueued(to, subject, out.tick);
    }
}

void AvatarNet::releasePending(uint64_t nowMs) {
    if (FNV_MUT(109)) return;   // twin 109: a snapshot held back by the rate is never released (a stopped player stays where the last sent one was)
    std::vector<nvcoop::avatar::StreamRatePlanner::Release> rel;
    rate_.poll(static_cast<uint32_t>(nowMs), rel);
    for (auto& r : rel) {
        const PlayerId to = r.observer == nvcoop::avatar::StreamRatePlanner::kUplink ? kHostPlayer : r.observer;
        if (!isHost_ && (to != kHostPlayer || !hostUp_)) continue;   // a client only talks to the host, and only while the link is up
        if (isHost_ && to != kHostPlayer && !joined_.count(to)) continue;   // the observer left meanwhile
        if (sender_.queue(to, r.snap, static_cast<uint32_t>(nowMs))) {
            ++stats_.rateReleased;
            if (r.subject != self_) ++stats_.relayed;
            noteQueued(to, r.subject, r.snap.tick);
        }
    }
}

void AvatarNet::onLook(PlayerId from, const uint8_t* data, size_t size) {
    AvLook l;
    if (!decodeLook(data, size, l)) { ++stats_.looksRejected; return; }
    if (isHost_) {
        if (l.slot != from && !FNV_MUT(50)) { ++stats_.looksRejected; return; }   // MUTANT 50: a client may set another player's look
    } else {
        if (from != kHostPlayer || l.slot == self_) { ++stats_.looksRejected; return; }
    }
    auto old = looks_.find(l.slot);
    const bool newLook = old == looks_.end() || !(old->second == l);
    const bool oldHadLook = old != looks_.end();
    const nvcoop::avatar::Appearance oldApp = oldHadLook ? old->second.appearance : nvcoop::avatar::Appearance{};
    looks_[l.slot] = l;
    if (roster_.count(l.slot)) {
        applyRemoteAppearance(l.slot, l.appearance, oldHadLook, oldApp);
        if (!FNV_MUT(45)) side_.onRemoteEquipment(l.slot, l.equipment);   // MUTANT 45: the equipment of other players is never applied
        ++stats_.looksApplied;
    }
    if (isHost_ && newLook) sendLook(l, kNoPlayer, from);
}

// Compare with what the PUPPET was built from, not with the previous look: the first look of a player arrives after the roster created its puppet from the default
// appearance (a client learns who exists before it learns what they look like).
void AvatarNet::applyRemoteAppearance(PlayerId slot, const nvcoop::avatar::Appearance& app, bool oldHadLook, const nvcoop::avatar::Appearance& oldApp) {
    auto ap = applied_.find(slot);
    bool appearanceChanged = ap == applied_.end() || !sameAppearance(ap->second, app);
    if (FNV_MUT(106)) appearanceChanged = oldHadLook && !sameAppearance(oldApp, app);   // MUTANT 106: the FIRST look of a player never reaches its puppet
    if (appearanceChanged || FNV_MUT(105)) {
        const bool templateChanged = ap == applied_.end() || ap->second.race != app.race || ap->second.female != app.female;
        // MUTANT 105: every look change rebuilds the puppet (it flashes; the interpolation state and equipment start over)
        if (FNV_MUT(105)) side_.addRemote(slot, app);
        else side_.setRemoteAppearance(slot, app);
        applied_[slot] = app;
        ++stats_.appearanceUpdates;
        if (templateChanged) ++stats_.puppetRespawnsForLook;
    }
}

// Host: a client's facts. Only the host consumes them (host-run scripts ask the party conditions), so they are neither relayed nor accepted by a client.
void AvatarNet::onFacts(PlayerId from, const uint8_t* data, size_t size) {
    AvFacts f;
    if (!isHost_ || !joined_.count(from) || !decodeFacts(data, size, f)) { ++stats_.factsRejected; return; }
    if (f.slot != from && !FNV_MUT(68)) { ++stats_.factsRejected; return; }   // MUTANT 68: a client may set another player's facts
    facts_[from] = f.facts;
    ++stats_.factsApplied;
}

// A client: its level, sit state, active actions and regions go to the host when they change (read once per factsCheckMs). The host's own player is read by the conditions themselves.
void AvatarNet::checkLocalFacts(uint64_t nowMs) {
    if (opt_.partsUnified && !FNV_MUT(1170)) return;
    if (isHost_ || !hostUp_ || FNV_MUT(69)) return;   // MUTANT 69: a guest never tells the host its facts
    if (factsSentValid_ && nowMs - lastFactsCheck_ < opt_.factsCheckMs) return;
    lastFactsCheck_ = nowMs;
    nvcoop::avatar::PlayerFacts f;
    if (!side_.readLocalFacts(f)) return;
    if (factsSentValid_ && f == lastFactsSent_) return;
    AvFacts m;
    m.slot = self_;
    m.facts = f;
    sendTo(kHostPlayer, true, encodeFacts(m));
    ++stats_.factsSent;
    lastFactsSent_ = f;
    factsSentValid_ = true;
}

// ---- guest pack facts ------------------------------------------------------------------------------------------------------------
void AvatarNet::ensureWatch() const {
    if (watchLoaded_ || !packWatchFn_) return;
    if (watchTriedAt_ && nowMs_ - watchTriedAt_ < 5000) return;   // an empty answer (the game data is not up yet) is asked again in a few seconds
    watchTriedAt_ = nowMs_ ? nowMs_ : 1;
    std::vector<nvcoop::avatar::FormId> w = packWatchFn_();
    std::sort(w.begin(), w.end());
    w.erase(std::unique(w.begin(), w.end()), w.end());
    if (!w.empty() && w.front() == 0) w.erase(w.begin());
    watch_ = std::move(w);
    watchId_ = nvcoop::avatar::packWatchId(watch_);
    watchLoaded_ = !watch_.empty();
}

// Host: a client's pack. Only the host consumes it (host-run conditions and the pooled hand-ins ask for a guest), so it is neither relayed nor accepted by a client.
void AvatarNet::onPack(PlayerId from, const uint8_t* data, size_t size) {
    AvPack p;
    if (!isHost_ || !joined_.count(from) || !decodePack(data, size, p)) { ++stats_.packsRejected; return; }
    if (p.slot != from && !FNV_MUT(954)) { ++stats_.packsRejected; return; }   // twin 954: a client may set another player's pack
    packs_[from] = std::move(p.pack);
    ++stats_.packsApplied;
}

// A client: its weight and the counts of the watched item forms go to the host when they change (read once per packCheckMs, sent at most once per packMinGapMs). The host's own
// player is read by the conditions themselves.
void AvatarNet::checkLocalPack(uint64_t nowMs) {
    if (opt_.partsUnified && !FNV_MUT(1170)) return;
    if (isHost_ || !hostUp_ || FNV_MUT(952)) return;   // twin 952: a guest never tells the host its pack
    if (packSentValid_ && nowMs - lastPackCheck_ < opt_.packCheckMs) return;
    lastPackCheck_ = nowMs;
    ensureWatch();
    nvcoop::avatar::PlayerPack p;
    if (!side_.readLocalPack(watch_, p)) return;
    if (packSentValid_ && p == lastPackSent_) return;
    if (packSentValid_ && nowMs - lastPackSend_ < opt_.packMinGapMs && !FNV_MUT(953)) return;   // twin 953: no rate limit (every change goes out at once)
    AvPack m;
    m.slot = self_;
    m.pack = p;
    sendTo(kHostPlayer, true, encodePack(m));
    ++stats_.packsSent;
    lastPackSent_ = std::move(p);
    lastPackSend_ = nowMs;
    packSentValid_ = true;
}

bool AvatarNet::packCount(PlayerId slot, nvcoop::avatar::FormId item, int64_t* out) const {
    auto it = packs_.find(slot);
    if (it == packs_.end() || !item) return false;
    for (const auto& e : it->second.items)
        if (e.form == item) { if (out) *out = e.count; return true; }
    // not listed = zero, but only for a form both machines watch with the same list; anything else is unknown (the caller keeps its old answer)
    ensureWatch();
    if (FNV_MUT(955) || ((FNV_MUT(956) || it->second.watchId == watchId_) && std::binary_search(watch_.begin(), watch_.end(), item))) {   // twins 955: every unlisted form is a zero, 956: the watch list ids are not compared
        if (out) *out = 0;
        return true;
    }
    return false;
}

bool AvatarNet::packWeight(PlayerId slot, double* out) const {
    auto it = packs_.find(slot);
    if (it == packs_.end() || !it->second.haveWeight) return false;
    if (out) *out = it->second.weightMilli / 1000.0;
    return true;
}

// ---------------------------------------------------------------------------------------------------------------------------------
// per frame
// ---------------------------------------------------------------------------------------------------------------------------------
// The worn list: what the watcher last published (read, so a session that began after it still has it), else what the sink delivered.
bool AvatarNet::localEquipmentNow(std::vector<nvcoop::avatar::EquipEntry>& out) {
    if (side_.readLocalEquipment(out)) return true;
    if (!haveEquip_) return false;
    out = equip_;
    return true;
}

// The local look as the game has it: an injected reader is asked every time, the game itself at most once per appearancePollMs.
bool AvatarNet::localAppearanceNow(nvcoop::avatar::Appearance& out) {
    if (localAppearance_) return localAppearance_(out);
    if (haveLocalApp_ && nowMs_ - lastAppRead_ < opt_.appearancePollMs) {
        out = localApp_;
        return true;
    }
    nvcoop::avatar::Appearance a;
    lastAppRead_ = nowMs_;
    if (side_.readLocalAppearance(a)) {
        localApp_ = a;
        haveLocalApp_ = true;
    }
    if (!haveLocalApp_) return false;
    out = localApp_;
    return true;
}

void AvatarNet::checkLocalLook(uint64_t nowMs) {
    if (opt_.partsUnified && !FNV_MUT(1170)) return;   // the Look and Equipment parts carry it
    if (nowMs - lastLookCheck_ < opt_.lookCheckMs && lookSentValid_) return;
    lastLookCheck_ = nowMs;
    if (!(isHost_ || hostUp_)) return;
    AvLook l;
    l.slot = self_;
    const bool haveApp = localAppearanceNow(l.appearance);
    if (!haveEquip_ && !haveApp) return;
    l.equipment = equip_;
    if (lookSentValid_ && l == lastLookSent_) return;
    if (isHost_) {
        looks_[self_] = l;
        sendLook(l, kNoPlayer, kNoPlayer);
    } else {
        sendLook(l, kHostPlayer, kNoPlayer);
    }
    lastLookSent_ = l;
    lookSentValid_ = true;
}

void AvatarNet::flushStreams(uint64_t nowMs) {
    for (PlayerId p : peers()) {
        for (Bytes& d : sender_.flush(p, static_cast<uint32_t>(nowMs))) {
            ++stats_.sent;
            sendTo(p, false, withChannel(d));
        }
        // the Send stage: how old each local snapshot is at the moment its datagram has been handed to the transport
        auto qt = queuedTicks_.find(p);
        if (qt != queuedTicks_.end()) {
            const uint32_t now = clock();
            for (uint32_t t : qt->second) side_.noteStage(nvcoop::avatar::NetStage::Send, static_cast<double>(static_cast<int32_t>(now - t)));
            qt->second.clear();
        }
    }
}

void AvatarNet::sendAcks(uint64_t nowMs) {
    if (nowMs - lastAck_ < opt_.ackIntervalMs) return;
    lastAck_ = nowMs;
    for (PlayerId p : peers()) {
        Bytes a = recv_.takeAcks(p);
        if (a.empty()) continue;
        ++stats_.acksSent;
        sendTo(p, false, withChannel(a));
    }
}

void AvatarNet::reportPoses(uint64_t nowMs) {
    if (!poseSink_ || FNV_MUT(49)) return;   // MUTANT 49: the ownership service never learns where remote players are
    if (nowMs - lastPoseReport_ < opt_.poseReportMs) return;
    lastPoseReport_ = nowMs;
    for (const auto& kv : lastSnap_) {
        auto rep = reportedSnap_.find(kv.first);
        if (rep != reportedSnap_.end() && rep->second.cell == kv.second.cell && rep->second.pos == kv.second.pos) continue;
        reportedSnap_[kv.first] = kv.second;
        poseSink_(kv.first, true, kv.second.cell, kv.second.pos);
    }
    if (haveLocal_ && (!reportedLocalValid_ || reportedLocal_.cell != lastLocal_.cell || !(reportedLocal_.pos == lastLocal_.pos))) {
        reportedLocal_ = lastLocal_;
        reportedLocalValid_ = true;
        poseSink_(self_, true, lastLocal_.cell, lastLocal_.pos);
    }
}

void AvatarNet::tick(uint64_t nowMs) {
    nowMs_ = nowMs;
    if (!inSession_) return;
    if (opt_.tickSide) side_.tick(static_cast<uint32_t>(nowMs));   // publishes the local snapshot (sink), drives puppets
    side_.noteStage(nvcoop::avatar::NetStage::NetFrame, static_cast<double>(clock()));
    if (opt_.streamUnified) absorbLocalSamples();   // ActorSync takes the local player's samples through PlayerPort and sends them with its own planner and sender
    const uint32_t id = playerNetId(self_);
    for (const live::ActorSnapshot& s : localOut_) {
        ++stats_.localSnapshots;
        live::ActorSnapshot snap = s;
        snap.netId = id;
        snap.ownerEpoch = epoch_;
        lastLocal_ = snap;
        haveLocal_ = true;
        rate_.setPlace(self_, snap.cell, snap.pos);
        if (isHost_) {
            for (PlayerId p : joined_) queueTo(p, self_, snap, nowMs);
        } else if (hostUp_) {
            queueTo(kNoPlayer, self_, snap, nowMs);   // the uplink: the host relays it onward at the tier each other player has
        }
    }
    localOut_.clear();
    if (!opt_.streamUnified) releasePending(nowMs);
    checkLocalLook(nowMs);
    checkLocalFacts(nowMs);
    checkLocalPack(nowMs);
    if (!opt_.streamUnified) {
        flushStreams(nowMs);
        sendAcks(nowMs);
    }
    syncRosterFromEntities();
    reportPoses(nowMs);
}

// ---------------------------------------------------------------------------------------------------------------------------------
// the player entities (parts=unified, stream=unified): the roster, and PlayerPort's endpoint
// ---------------------------------------------------------------------------------------------------------------------------------
// stream=unified: the sampler's snapshots (the avatar module samples before the live layer runs) wait for ActorSync, which may run before this service's own tick in the
// same frame; PlayerPort::sample absorbs them itself, so a sample is never a frame late.
void AvatarNet::absorbLocalSamples() {
    const uint32_t id = playerNetId(self_);
    for (const live::ActorSnapshot& s : localOut_) {
        ++stats_.localSnapshots;
        live::ActorSnapshot snap = s;
        snap.netId = id;
        snap.ownerEpoch = epoch_;
        lastLocal_ = snap;
        haveLocal_ = true;
        rate_.setPlace(self_, snap.cell, snap.pos);
        samples_.push_back(snap);
        if (samples_.size() > 8) samples_.pop_front();
    }
    localOut_.clear();
}

// A client's roster is the set of player records ActorSync holds (the host's own is its joined clients, kept by playerJoined / playerLeft). While the host link is down
// the puppets are gone and stay gone; they come back with the link and the host's join state.
void AvatarNet::syncRosterFromEntities() {
    if (!opt_.partsUnified || !entities_ || isHost_ || !hostUp_ || FNV_MUT(1172)) return;   // MUTANT 1172: a client never learns who exists
    std::set<PlayerId> want;
    for (PlayerId s : entities_->playerSlots())
        if (s != self_ && s < fnvcoop::kMaxSlots) want.insert(s);
    for (PlayerId s : std::vector<PlayerId>(roster_.begin(), roster_.end()))
        if (!want.count(s)) removePuppet(s);
    for (PlayerId s : want)
        if (!roster_.count(s)) addPuppet(s);
}

fnvcoop::FormRef AvatarNet::toRef(nvcoop::avatar::FormId id) const {
    fnvcoop::FormRef r;
    if (formCodec_.toRef) r = formCodec_.toRef(id);
    if (r.plugin.empty()) r = fnvcoop::FormRef("@rt", id);   // not resolvable: the runtime id, which an equal load order makes the same everywhere
    return r;
}

nvcoop::avatar::FormId AvatarNet::fromRef(const fnvcoop::FormRef& r) const {
    if (r.plugin == "@rt") return r.localId;
    return formCodec_.fromRef ? formCodec_.fromRef(r) : 0;
}

bool AvatarNet::readLocalPart(nvcoop::actors::PartKind kind, Bytes& out) {
    using nvcoop::actors::PartKind;
    switch (kind) {
        case PartKind::Look: {
            nvcoop::avatar::Appearance a;
            if (!localAppearanceNow(a)) return false;
            out = nvcoop::avatar::encodeAppearance(a);
            lastLookSent_.slot = self_;
            lastLookSent_.appearance = a;
            lookSentValid_ = true;
            if (isHost_) looks_[self_] = lastLookSent_;
            return true;
        }
        case PartKind::Equipment: {
            std::vector<nvcoop::avatar::EquipEntry> worn;
            if (!localEquipmentNow(worn)) return false;
            nvcoop::actors::EquipList forms;
            for (const nvcoop::avatar::EquipEntry& e : worn) forms.push_back(toRef(e.form));
            out = nvcoop::actors::encodeEquipment(std::move(forms));
            lastLookSent_.equipment = worn;
            if (isHost_) looks_[self_].equipment = worn;
            return true;
        }
        case PartKind::WeaponState: {
            std::vector<nvcoop::avatar::EquipEntry> worn;
            if (!localEquipmentNow(worn)) return false;
            nvcoop::actors::WeaponState w;
            for (const nvcoop::avatar::EquipEntry& e : worn)
                if (e.weapon) {
                    w.weapon = toRef(e.form);
                    w.mods = static_cast<uint8_t>(e.mods & nvcoop::avatar::kWeaponModMask);
                    break;
                }
            if (FNV_MUT(1175)) w.weapon = fnvcoop::FormRef();   // MUTANT 1175: the held weapon is never named (a puppet draws nothing)
            out = nvcoop::actors::encodeWeaponState(w);
            return true;
        }
        case PartKind::Facts: {
            nvcoop::avatar::PlayerFacts f;
            if (!side_.readLocalFacts(f)) return false;
            out = nvcoop::avatar::encodeFacts(f);
            return true;
        }
        case PartKind::Pack: {
            ensureWatch();
            nvcoop::avatar::PlayerPack p;
            if (!side_.readLocalPack(watch_, p)) return false;
            out = nvcoop::avatar::encodePack(p);
            return true;
        }
        case PartKind::Inventory: break;   // a player's pack is its own; Pack is the story-visible slice
    }
    return false;
}

void AvatarNet::rebuildRemoteEquipment(PlayerId slot) {
    RemoteEquip& re = remoteEquip_[slot];
    if (!re.haveForms) return;   // the worn forms come first; a weapon state alone names no item
    std::vector<nvcoop::avatar::EquipEntry> list;
    for (const fnvcoop::FormRef& f : re.forms) {
        const nvcoop::avatar::FormId id = fromRef(f);
        if (!id) continue;
        nvcoop::avatar::EquipEntry e;
        e.form = id;
        if (re.haveWeapon && !re.weapon.weapon.plugin.empty() && f == re.weapon.weapon) {   // the held weapon, with the mods it wears
            e.weapon = true;
            e.mods = FNV_MUT(1173) ? 0 : static_cast<uint8_t>(re.weapon.mods & nvcoop::avatar::kWeaponModMask);   // MUTANT 1173: the weapon modifications never reach the puppet
        }
        list.push_back(e);
    }
    std::sort(list.begin(), list.end(), [](const nvcoop::avatar::EquipEntry& a, const nvcoop::avatar::EquipEntry& b) { return a.form < b.form; });
    AvLook& l = looks_[slot];
    l.slot = slot;
    l.equipment = list;
    if (roster_.count(slot) && !FNV_MUT(45)) {
        side_.onRemoteEquipment(slot, list);
        ++stats_.looksApplied;
    }
}

bool AvatarNet::writeRemotePart(PlayerId slot, nvcoop::actors::PartKind kind, const Bytes& value) {
    using nvcoop::actors::PartKind;
    if (!inSession_ || slot >= fnvcoop::kMaxSlots || slot == self_) return false;
    switch (kind) {
        case PartKind::Look: {
            nvcoop::avatar::Appearance a;
            if (!nvcoop::avatar::decodeAppearance(value, a)) return false;
            auto old = looks_.find(slot);
            const bool oldHad = old != looks_.end();
            const nvcoop::avatar::Appearance oldApp = oldHad ? old->second.appearance : nvcoop::avatar::Appearance{};
            AvLook& l = looks_[slot];
            l.slot = slot;
            l.appearance = a;
            if (roster_.count(slot) && !FNV_MUT(1174)) {   // MUTANT 1174: the look is stored and never reaches the puppet
                applyRemoteAppearance(slot, a, oldHad, oldApp);
                ++stats_.looksApplied;
            }
            return true;
        }
        case PartKind::Equipment: {
            nvcoop::actors::EquipList forms;
            if (!nvcoop::actors::decodeEquipment(value, forms)) return false;
            RemoteEquip& re = remoteEquip_[slot];
            re.forms = std::move(forms);
            re.haveForms = true;
            rebuildRemoteEquipment(slot);
            return true;
        }
        case PartKind::WeaponState: {
            nvcoop::actors::WeaponState w;
            if (!nvcoop::actors::decodeWeaponState(value, w)) return false;
            RemoteEquip& re = remoteEquip_[slot];
            re.weapon = w;
            re.haveWeapon = true;
            rebuildRemoteEquipment(slot);
            return true;
        }
        case PartKind::Facts: {
            nvcoop::avatar::PlayerFacts f;
            if (!isHost_ || !nvcoop::avatar::decodeFacts(value, f)) return false;
            facts_[slot] = std::move(f);
            ++stats_.factsApplied;
            return true;
        }
        case PartKind::Pack: {
            nvcoop::avatar::PlayerPack p;
            if (!isHost_ || !nvcoop::avatar::decodePack(value, p)) return false;
            packs_[slot] = std::move(p);
            ++stats_.packsApplied;
            return true;
        }
        case PartKind::Inventory: break;
    }
    return false;
}

bool AvatarNet::takeLocalSample(fnvcoop::live::ActorSnapshot& out) {
    if (!opt_.streamUnified || !inSession_) return false;
    absorbLocalSamples();
    if (samples_.empty()) return false;
    out = samples_.front();
    samples_.pop_front();
    side_.noteStage(nvcoop::avatar::NetStage::Send, static_cast<double>(static_cast<int32_t>(clock() - out.tick)));   // the Send stage ends where ActorSync takes it
    return true;
}

// A remote snapshot ActorSync has judged (pinned owner, epoch, order), for the avatar layer's playback buffer: the same steps as an accepted 0xAD snapshot, minus the relay
// (the host relays inside ActorSync).
void AvatarNet::remoteSnapshot(PlayerId slot, const fnvcoop::live::ActorSnapshot& s, uint32_t) {
    if (!opt_.streamUnified || slot >= fnvcoop::kMaxSlots || slot == self_) return;
    if (!roster_.count(slot)) {   // the record is known before the puppet: kept for it, newest wins
        ++stats_.noRoster;
        early_[slot] = s;
        return;
    }
    side_.onRemoteSnapshot(slot, s, static_cast<uint32_t>(nowMs_));
    side_.noteStage(nvcoop::avatar::NetStage::Rx, static_cast<double>(static_cast<int32_t>(clock() - s.tick)));
    lastSnap_[slot] = s;
    rate_.setPlace(slot, s.cell, s.pos);
    ++stats_.accepted;
    ++rx_[slot];
}

void AvatarNet::idleTick(uint64_t nowMs) {
    nowMs_ = nowMs;
    // no session: the avatar layer still has work after a load (orphan puppets from the save are purged by its next tick)
    if (opt_.tickSide && side_.purgePending()) {
        ++stats_.idleTicks;
        side_.tick(static_cast<uint32_t>(nowMs));
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------
// game events
// ---------------------------------------------------------------------------------------------------------------------------------
void AvatarNet::loadBegin() { side_.onEvent(IAvatarSide::Event::LoadBegin); }

void AvatarNet::loadEnd() {
    if (FNV_MUT(47) && !inSession_) return;   // MUTANT 47: the post-load purge only happens inside a coop session (a puppet that a save carried stays)
    side_.onEvent(IAvatarSide::Event::LoadEnd);
    watchLoaded_ = false;   // the load order is resolved again (forms of a new game / save)
    watchTriedAt_ = 0;
}

void AvatarNet::gameExit() { side_.onEvent(IAvatarSide::Event::ExitToMenu); }

// ---------------------------------------------------------------------------------------------------------------------------------
// queries
// ---------------------------------------------------------------------------------------------------------------------------------
namespace {
const char* tierName(nvcoop::avatar::StreamTier t) {
    return t == nvcoop::avatar::StreamTier::Near ? "near" : (t == nvcoop::avatar::StreamTier::Far ? "far" : "out");
}
std::string lookJson(const AvLook& l) {
    const nvcoop::avatar::Appearance& a = l.appearance;
    std::string s = "{\"race\":" + std::to_string(a.race) + ",\"female\":" + (a.female ? "true" : "false") + ",\"hair\":" + std::to_string(a.hair) +
                    ",\"eyes\":" + std::to_string(a.eyes) + ",\"hairColor\":" + std::to_string(a.hairColor) + ",\"height\":" + std::to_string(a.height) +
                    ",\"weight\":" + std::to_string(a.weight) + ",\"faceCoeffs\":" + std::to_string(a.faceCoeffs.size()) + ",\"equip\":[";
    for (size_t i = 0; i < l.equipment.size(); ++i) {
        if (i) s += ",";
        s += "{\"form\":" + std::to_string(l.equipment[i].form) + ",\"weapon\":" + (l.equipment[i].weapon ? "true" : "false") + ",\"mods\":" +
             std::to_string(static_cast<int>(l.equipment[i].mods)) + "}";
    }
    return s + "]}";
}
}  // namespace

std::string AvatarNet::statusJson() const {
    ensureWatch();   // the host resolves its list lazily; the status shows it
    std::string s = "{\"inSession\":";
    s += inSession_ ? "true" : "false";
    s += std::string(",\"entities\":{\"parts\":\"") + (opt_.partsUnified ? "unified" : "legacy") + "\",\"stream\":\"" + (opt_.streamUnified ? "unified" : "legacy") + "\"}";
    s += ",\"self\":" + std::to_string(static_cast<int>(self_)) + ",\"epoch\":" + std::to_string(epoch_) + ",\"roster\":[";
    bool first = true;
    for (PlayerId p : roster_) {
        if (!first) s += ",";
        first = false;
        auto rx = rx_.find(p);
        s += "{\"slot\":" + std::to_string(static_cast<int>(p)) + ",\"ref\":" + std::to_string(side_.refOf(p)) + ",\"look\":" + (looks_.count(p) ? "true" : "false") +
             ",\"tier\":\"" + tierName(rate_.tierOf(self_, p)) + "\",\"rx\":" + std::to_string(rx == rx_.end() ? 0ull : (unsigned long long)rx->second);
        auto lk = looks_.find(p);
        if (lk != looks_.end()) s += ",\"lookInfo\":" + lookJson(lk->second);
        auto fc = facts_.find(p);
        if (fc != facts_.end())
            s += ",\"facts\":{\"level\":" + std::to_string(fc->second.level) + ",\"sitting\":" + std::to_string(fc->second.sitting) + ",\"actions\":" + std::to_string(fc->second.actions) +
                 ",\"regions\":" + std::to_string(fc->second.regions.size()) + "}";
        auto pk = packs_.find(p);
        if (pk != packs_.end()) {
            s += ",\"pack\":{\"watchId\":" + std::to_string(pk->second.watchId) + ",\"weight\":" + (pk->second.haveWeight ? std::to_string(pk->second.weightMilli / 1000.0) : std::string("null")) + ",\"items\":[";
            for (size_t i = 0; i < pk->second.items.size(); ++i) s += (i ? "," : "") + std::string("[") + std::to_string(pk->second.items[i].form) + "," + std::to_string(pk->second.items[i].count) + "]";
            s += "]}";
        }
        s += "}";
    }
    s += "],\"selfLook\":" + (lookSentValid_ ? lookJson(lastLookSent_) : std::string("null"));
    s += ",\"sent\":" + std::to_string(stats_.sent) + ",\"received\":" + std::to_string(stats_.received) +
         ",\"accepted\":" + std::to_string(stats_.accepted) + ",\"rejected\":" + std::to_string(stats_.rejected) +
         ",\"forged\":" + std::to_string(stats_.forged) + ",\"noRoster\":" + std::to_string(stats_.noRoster) +
         ",\"relayed\":" + std::to_string(stats_.relayed) + ",\"looksApplied\":" + std::to_string(stats_.looksApplied) +
         ",\"rateKept\":" + std::to_string(stats_.rateKept) + ",\"rateReleased\":" + std::to_string(stats_.rateReleased) +
         ",\"appearanceUpdates\":" + std::to_string(stats_.appearanceUpdates) + ",\"puppetRespawnsForLook\":" + std::to_string(stats_.puppetRespawnsForLook) +
         ",\"localSnapshots\":" + std::to_string(stats_.localSnapshots) + ",\"factsSent\":" + std::to_string(stats_.factsSent) +
         ",\"factsApplied\":" + std::to_string(stats_.factsApplied) + ",\"factsRejected\":" + std::to_string(stats_.factsRejected) +
         ",\"packsSent\":" + std::to_string(stats_.packsSent) + ",\"packsApplied\":" + std::to_string(stats_.packsApplied) + ",\"packsRejected\":" + std::to_string(stats_.packsRejected) +
         ",\"watch\":" + std::to_string(watch_.size()) + ",\"watchId\":" + std::to_string(watchId_) +
         ",\"stream\":{\"senderQueued\":" + std::to_string(sender_.stats().queued) + ",\"senderSkippedIdle\":" + std::to_string(sender_.stats().skippedIdle) +
         ",\"senderPackets\":" + std::to_string(sender_.stats().packets) + ",\"senderAcked\":" + std::to_string(sender_.stats().acked) + ",\"senderResyncs\":" + std::to_string(sender_.stats().resyncs) +
         ",\"recvAccepted\":" + std::to_string(recv_.stats().accepted) + ",\"recvNeedBaseline\":" + std::to_string(recv_.stats().needBaseline) +
         ",\"recvStaleSeq\":" + std::to_string(recv_.stats().staleSeq) + ",\"plannerKept\":" + std::to_string(rate_.kept()) + ",\"plannerPending\":" + std::to_string(rate_.pending()) + "}}";
    return s;
}

}  // namespace nvcoop::livewire
