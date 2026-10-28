#include "companions/social_sync.h"

#include <algorithm>
#include <cstdio>

#include "../../tools/mut/fnvcoop_mut.h"

namespace nvcoop::social {

namespace {

uint8_t bit(PlayerId p) { return p < kMaxSlots ? static_cast<uint8_t>(1u << p) : 0; }
constexpr size_t kMirrorRemember = 8192;
constexpr size_t kAlarmRemember = 4096;

}  // namespace

SocialSync::SocialSync(const Config& cfg, ISocialWorld& world, ISocialEngine& engine, ISocialNet& net, ICommitter& committer)
    : cfg_(cfg), world_(world), engine_(engine), net_(net), commit_(committer), book_(cfg.settings.caps),
      escort_(cfg.escortRules), det_(cfg.crimeRules.witnessWindowMs), crime_(cfg.crimeRules, cfg.self), settings_(cfg.settings) {
    for (const FormRef& p : cfg_.sets.partyNpcs()) book_.addFixedParty(p);
    if (cfg_.isHost && cfg_.isPartyCompanion && !FNV_MUT(936)) book_.setPartyOracle(cfg_.isPartyCompanion);   // BAD (936): the rules engine's owner=party is never asked, so ED-E uses a follower slot
    if (cfg_.isHost) {
        connected_.insert(kHostPlayer);
        synced_ = true;
        if (settings_.rev == 0) settings_.rev = 1;
    }
}

// ---- sending -------------------------------------------------------------------------------------------------------------------------
void SocialSync::send(PlayerId dest, const sc::Body& b, PlayerId src) {
    sc::Envelope e;
    e.src = src == kNoPlayer ? cfg_.self : src;
    e.dest = dest;
    e.body = b;
    Bytes w = sc::encodeEnvelope(e);
    if (w.empty()) return;
    net_.send(cfg_.isHost ? dest : kHostPlayer, std::move(w));
    ++stats_.sent;
}

void SocialSync::sendAll(const sc::Body& b, PlayerId except, PlayerId src) {
    if (!cfg_.isHost) return;
    for (PlayerId p : std::vector<PlayerId>(connected_.begin(), connected_.end())) {
        if (p == kHostPlayer || p == except) continue;
        send(p, b, src);
    }
}

uint8_t SocialSync::connectedMask() const {
    uint8_t m = 0;
    for (PlayerId p : connected_) m = static_cast<uint8_t>(m | bit(p));
    return m;
}

void SocialSync::sendJoinState(PlayerId p) {
    if (FNV_MUT(922)) return;   // BAD (922): a joiner never gets the host's picture: no roster, no wanted scores, no settings
    send(p, sc::MSettings{settings_});
    sc::MRoster r;
    r.all = book_.roster();
    send(p, r);
    ++stats_.rosterSent;
    sc::MBounty b;
    b.u = crime_.bounty().roster();
    b.full = true;
    send(p, b);
    for (const sc::Companion& c : book_.roster()) {
        sc::InvFull f = inv_.fullOf(c.npc);
        if (!f.items.empty()) send(p, sc::MInvFull{f});
    }
}

// ---- lifecycle -------------------------------------------------------------------------------------------------------------------------
void SocialSync::playerJoined(PlayerId p) {
    if (!cfg_.isHost || p >= kMaxSlots || p == kHostPlayer) return;
    connected_.insert(p);
    sendJoinState(p);
}

void SocialSync::playerLeft(PlayerId p) {
    if (!cfg_.isHost || p == kHostPlayer) return;
    connected_.erase(p);
    if (cfg_.companions && !FNV_MUT(910)) {   // BAD (910): the companions of a player who left stay flagged as theirs
        for (const FormRef& npc : book_.companionsOf(p)) {
            if (book_.isParty(npc) && !FNV_MUT(937)) continue;   // BAD (937): ED-E goes with its recruiter (the Lonesome Road plot loses its companion)
            releaseCompanion(npc, "owner-left");
        }
        handPartyCompanions(p);
    }
}

// The party companions of a player who left stay in the session: the host becomes their recruiter (the simulating machine), committed so the story state,
// the dialogue registry and a joiner's baseline all name the same owner.
void SocialSync::handPartyCompanions(PlayerId leaver) {
    std::vector<sc::Companion> before;
    for (const sc::Companion& c : book_.roster()) if (c.party && c.owner == leaver) before.push_back(c);
    const std::vector<FormRef> handed = book_.handParty(leaver, kHostPlayer);
    if (handed.empty()) return;
    std::vector<Effect> fx;
    for (const sc::Companion& c : before) {
        fnvcoop::CompanionOwner x;
        x.npc = c.npc; x.owner = kHostPlayer; x.active = true; x.kind = c.kind == sc::CompanionKind::Critter ? 1 : 0;
        fx.push_back(x);
        if (cfg_.feedActors) cfg_.feedActors(x);
    }
    stats_.partyHanded += handed.size();
    if (!FNV_MUT(938) && commit_.commit(fx, "party-companion-handed") == 0) ++stats_.commitFailed;   // BAD (938): the hand-over is only in the book, so the session still names the leaver
    broadcastRoster();
    dirtyReconcile_ = true;
}

void SocialSync::hostConnected() {
    if (cfg_.isHost) return;
    hostUp_ = true;
    synced_ = false;
    askResync();
}

void SocialSync::hostLost() {
    if (cfg_.isHost) return;
    hostUp_ = false;
    synced_ = false;
}

// ---- story effects (host) ----------------------------------------------------------------------------------------------------------------
void SocialSync::onEffect(const Effect& e) {
    if (!cfg_.isHost || !cfg_.companions) return;
    const auto* c = std::get_if<fnvcoop::CompanionOwner>(&e);
    if (!c) return;
    if (c->active) {
        // The session drops a recruit beyond the caps at commit time, but the capture sink sees it first: do not learn what was dropped.
        const sc::CompanionKind kind = c->kind == 1 ? sc::CompanionKind::Critter : sc::CompanionKind::Humanoid;
        const sc::RecruitVerdict v = book_.check(c->npc, c->owner, kind);
        if (!v.ok && (v.reason == sc::RecruitReason::OwnerCap || v.reason == sc::RecruitReason::PartyCap || v.reason == sc::RecruitReason::BadOwner) && !FNV_MUT(909)) {   // BAD (909): a dropped recruit is learnt anyway
            ++stats_.recruitsDropped;
            dirtyReconcile_ = true;   // the engine on the recruiting machine flagged it: the reconcile takes the flag off
            return;
        }
    }
    const sc::CompanionBook::Change ch = book_.applyEffect(e);
    if (!ch.changed) return;
    if (!c->active) { inv_.forget(c->npc); appliedState_.erase(c->npc); ++stats_.released; }
    if (cfg_.feedActors && !FNV_MUT(923)) cfg_.feedActors(e);   // BAD (923): the actor layer never hears of the recruiter (the companion is owned by the nearest player)
    broadcastRoster();
    dirtyReconcile_ = true;
}

void SocialSync::onTxn(const std::vector<Effect>& effects) {
    if (!cfg_.isHost || !cfg_.companions) return;
    if (!cfg_.swapGuard || FNV_MUT(934)) {   // BAD (934): the whole-transaction judgement is skipped, so one player's hire can fire another player's companion
        for (const Effect& e : effects) onEffect(e);
        return;
    }
    const sc::TxnVet vet = sc::vetCompanionTxn(book_, cfg_.sets, effects);
    for (const Effect& e : vet.learn) onEffect(e);
    stats_.swapsReverted += vet.swapsReverted;
    stats_.slotDenied += vet.slotDenied;
    if (!vet.repairs.empty()) {
        repairs_.insert(repairs_.end(), vet.repairs.begin(), vet.repairs.end());
        repairAtMs_ = nowMs_ + cfg_.repairDelayMs;
        dirtyReconcile_ = true;   // the engine flags the casualty / the stray recruit changed: the next reconcile sets them back
    }
}

size_t SocialSync::vetOutgoing(std::vector<Effect>& effects) {
    if (!cfg_.isHost || !cfg_.companions || !cfg_.swapGuard || FNV_MUT(944)) return 0;   // BAD (944): the commit is never vetted (the 250 ms revert is the only net)
    const sc::TxnVet vet = sc::vetCompanionTxn(book_, cfg_.sets, effects);
    if (vet.swapsReverted == 0 && vet.slotDenied == 0) return 0;
    std::vector<Effect> out;
    size_t companionIn = 0;
    bool placed = false;
    for (Effect& e : effects) {
        if (!std::holds_alternative<fnvcoop::CompanionOwner>(e)) { out.push_back(std::move(e)); continue; }
        ++companionIn;
        if (!placed) { out.insert(out.end(), vet.learn.begin(), vet.learn.end()); placed = true; }   // the vet kept these (in their order), the rest are casualties
    }
    const size_t removed = companionIn > vet.learn.size() ? companionIn - vet.learn.size() : 0;
    effects = std::move(out);
    return removed;
}

// Repairs are committed after the transaction that needed them was seen by the session (the capture sink and the session's own commit may run in either order).
void SocialSync::flushRepairs() {
    if (repairs_.empty() || nowMs_ < repairAtMs_) return;
    std::vector<Effect> fx;
    fx.swap(repairs_);
    if (!FNV_MUT(935)) {   // BAD (935): the repair stays in the book only, so the session, the dialogue registry and a joiner still see the fired companion
        if (commit_.commit(fx, "companion-repair") == 0) ++stats_.commitFailed;
        else stats_.repairsCommitted += fx.size();
    }
    for (const Effect& e : fx)
        if (cfg_.feedActors) cfg_.feedActors(e);
}

// The rules engine learns that a recruit is a party companion when the transaction passes its filter, which may be after our sink saw it: re-ask every second.
void SocialSync::refreshParty() {
    if (!cfg_.isHost) return;
    const std::vector<FormRef> changed = book_.refreshParty();
    if (changed.empty()) return;
    stats_.partyFlagged += changed.size();
    broadcastRoster();
    dirtyReconcile_ = true;
}

void SocialSync::releaseCompanion(const FormRef& npc, const char* why) {
    const sc::Companion* c = book_.find(npc);
    if (!c) return;
    fnvcoop::CompanionOwner fx;
    fx.npc = npc; fx.owner = c->owner; fx.active = false; fx.kind = c->kind == sc::CompanionKind::Critter ? 1 : 0;
    book_.applyEffect(fx);
    inv_.forget(npc);
    appliedState_.erase(npc);
    ++stats_.released;
    if (cfg_.feedActors) cfg_.feedActors(fx);
    if (commit_.commit({fx}, why) == 0) ++stats_.commitFailed;
    broadcastRoster();
    dirtyReconcile_ = true;
}

void SocialSync::broadcastRoster() {
    sc::MRoster r;
    r.all = book_.roster();
    sendAll(r);
    ++stats_.rosterSent;
    if (cfg_.onRoster && !FNV_MUT(943)) cfg_.onRoster(r.all);   // BAD (943): the dialogue registry never hears of a party companion or a handed-over owner
}

// ---- receiving -------------------------------------------------------------------------------------------------------------------------
void SocialSync::receive(PlayerId from, const uint8_t* data, size_t size) {
    sc::Envelope env;
    if (!sc::decodeEnvelope(data, size, env)) { ++stats_.dropped; return; }
    if (cfg_.isHost) {
        if (from >= kMaxSlots || from == kHostPlayer || connected_.count(from) == 0) { ++stats_.dropped; return; }
        if (env.src != from) ++stats_.spoofed;
        if (!FNV_MUT(902)) env.src = from;   // BAD (902): a client's claim about who authored a message is believed
    } else {
        if (from != kHostPlayer && !FNV_MUT(901)) { ++stats_.notFromHost; return; }   // BAD (901): a client believes a datagram from another client
    }
    ++stats_.received;
    onMessage(env);
}

void SocialSync::onMessage(const sc::Envelope& env) {
    const PlayerId src = env.src;
    if (cfg_.isHost) {
        if (auto* m = std::get_if<sc::MCrime>(&env.body)) hCrime(src, *m);
        else if (auto* m = std::get_if<sc::MDetect>(&env.body)) hDetect(src, *m);
        else if (auto* m = std::get_if<sc::MKill>(&env.body)) hKill(src, *m);
        else if (auto* m = std::get_if<sc::MCmd>(&env.body)) hCmd(src, *m);
        else if (auto* m = std::get_if<sc::MInv>(&env.body)) hInv(src, *m);
        else if (auto* m = std::get_if<sc::MInvFull>(&env.body)) hInvFull(src, *m);
        else if (auto* m = std::get_if<sc::MInvAsk>(&env.body)) hInvAsk(src, *m);
        else if (std::holds_alternative<sc::MResync>(env.body)) { ++stats_.resyncsAnswered; sendJoinState(src); }
        // everything else is the host's to send: a client's copy of it is ignored
        return;
    }
    if (auto* m = std::get_if<sc::MBounty>(&env.body)) cBounty(*m);
    else if (auto* m = std::get_if<sc::MAlarm>(&env.body)) cAlarm(*m);
    else if (auto* m = std::get_if<sc::MHostility>(&env.body)) cHostility(*m);
    else if (auto* m = std::get_if<sc::MRoster>(&env.body)) cRoster(*m);
    else if (auto* m = std::get_if<sc::MState>(&env.body)) cState(*m);
    else if (auto* m = std::get_if<sc::MInv>(&env.body)) cInv(*m);
    else if (auto* m = std::get_if<sc::MInvFull>(&env.body)) cInvFull(*m);
    else if (auto* m = std::get_if<sc::MSettings>(&env.body)) cSettings(*m);
    else if (auto* m = std::get_if<sc::MDigest>(&env.body)) cDigest(*m);
    else if (auto* m = std::get_if<sc::MCmdNack>(&env.body)) cCmdNack(*m);
}

// ---- host handlers ---------------------------------------------------------------------------------------------------------------------
void SocialSync::hCrime(PlayerId src, const sc::MCrime& m) {
    if (!cfg_.crime || !settings_.crime) return;
    judgeCrime(m.e, src);
}

void SocialSync::hDetect(PlayerId src, const sc::MDetect& m) {
    if (!cfg_.crime || !settings_.crime) return;
    // A machine may only say what ITS OWN NPCs see: a client cannot invent a witness (or hide one) for an NPC somebody else simulates.
    ActorView v;
    const bool known = world_.actorByNet(m.d.observer.id, v) && v.known;
    if ((!known || v.owner != src) && !FNV_MUT(903)) {   // BAD (903): any machine can report what any NPC sees
        ++stats_.detectRejected;
        return;
    }
    det_.report(m.d, nowMs_);
}

void SocialSync::hKill(PlayerId src, const sc::MKill& m) {
    if (!cfg_.rewards) return;
    sc::KillReport r = m.r;
    r.reporter = src;
    if (!FNV_MUT(931)) r.nativeMask = static_cast<uint8_t>(r.nativeMask & bit(src));           // BAD (931): a client can claim that ANOTHER player's engine paid already
    ++stats_.killsReported;
    settleKill(r);
}

void SocialSync::hCmd(PlayerId src, const sc::MCmd& m) {
    if (!cfg_.companions) return;
    sc::CmdVerdict v = hostJudge_.judge(m.c, src, nowMs_, book_);
    if (FNV_MUT(916) && v.reason == sc::CmdReason::NotOwner) {   // BAD (916): the host obeys a command from a player who does not own the companion
        if (const sc::Companion* comp = book_.find(m.c.npc)) {
            v.apply = true; v.stateChanged = true; v.newState = comp->state; v.newState.rev = comp->state.rev + 1;
            v.newState.mode = m.c.kind == sc::CmdKind::Wait ? sc::FollowMode::Wait : sc::FollowMode::Follow;
        }
    }
    if (!v.apply) {
        ++stats_.cmdNacked;
        if (!FNV_MUT(917)) {   // BAD (917): the issuer is never told, so its optimistic state stays
            send(src, sc::MCmdNack{m.c.id, v.reason});
        }
        return;
    }
    ++stats_.cmdAccepted;
    if (v.stateChanged) {
        book_.setState(m.c.npc, v.newState);
        sendAll(sc::MState{m.c.npc, v.newState});
        // A party companion the HOST simulates, ordered by a guest: the guest's machine cannot touch it, so the host applies the state to its own engine.
        if (const sc::Companion* comp = book_.find(m.c.npc)) if (comp->party && comp->owner == cfg_.self && !FNV_MUT(942)) applyOwnState(*comp);   // BAD (942): a guest's order for the host's party companion changes the book and nothing else
    }
    if (v.dismiss) releaseCompanion(m.c.npc, "dismissed");
}

void SocialSync::hInv(PlayerId src, const sc::MInv& m) {
    if (!cfg_.companions) return;
    const sc::InvVerdict v = inv_.judge(m.d, src, book_);
    if (!v.apply) {
        ++stats_.invRejected;
        if (v.reason == sc::InvReason::Duplicate) ++stats_.invDuplicates;
        if (v.resync) send(src, sc::MInvFull{inv_.fullOf(m.d.npc)});
        return;
    }
    inv_.apply(m.d);
    if (book_.ownerOf(m.d.npc) != cfg_.self && engine_.applyInventoryDelta(m.d.npc, m.d.item, m.d.delta)) ++stats_.invApplied;
    sendAll(sc::MInv{m.d}, src, src);
}

void SocialSync::hInvFull(PlayerId src, const sc::MInvFull& m) {
    if (!cfg_.companions) return;
    if (book_.ownerOf(m.f.npc) != src) { ++stats_.invRejected; return; }
    for (const sc::InvDelta& d : inv_.replace(m.f, src, invCounter_))
        if (book_.ownerOf(d.npc) != cfg_.self && engine_.applyInventoryDelta(d.npc, d.item, d.delta)) ++stats_.invApplied;
    ++stats_.invFulls;
    sendAll(m, src, src);
}

void SocialSync::hInvAsk(PlayerId src, const sc::MInvAsk& m) {
    if (!cfg_.companions) return;
    send(src, sc::MInvFull{inv_.fullOf(m.npc)});
}

// ---- client handlers -------------------------------------------------------------------------------------------------------------------
void SocialSync::cBounty(const sc::MBounty& m) {
    if (m.full) {
        const std::vector<sc::BountyUpdate> before = crime_.bounty().roster();
        crime_.bounty().replace(m.u);
        if (FNV_MUT(925)) return;   // BAD (925): the full table fills the book but never reaches the engine, so a late joiner's guards do not know the party is wanted
        for (const sc::BountyUpdate& u : m.u) {
            bool same = false;
            for (const sc::BountyUpdate& b : before) if (b == u) { same = true; break; }
            if (!same) applyStandingLocal(u);   // (what this machine's engine already has is not written again)
        }
        return;
    }
    for (const sc::BountyUpdate& u : m.u)
        if (crime_.bounty().apply(u)) applyStandingLocal(u);
}

void SocialSync::cAlarm(const sc::MAlarm& m) { raiseAlarmLocal(m.o); }

void SocialSync::cHostility(const sc::MHostility& m) {
    if (!crime_.hostility().accept(m.e, kHostPlayer)) return;
    if (engine_.applyHostility(m.e)) ++stats_.hostilityApplied;
}

void SocialSync::cRoster(const sc::MRoster& m) {
    const std::vector<sc::Companion> prev = book_.roster();
    book_.replaceRoster(m.all);
    synced_ = true;
    ++stats_.rosterApplied;
    if (cfg_.onRoster && !FNV_MUT(943)) cfg_.onRoster(m.all);
    for (const sc::Companion& p : prev)
        if (!book_.isCompanion(p.npc)) { inv_.forget(p.npc); appliedState_.erase(p.npc); }
    for (const sc::Companion& c : book_.roster())
        if (c.owner == cfg_.self) applyOwnState(c);
    dirtyReconcile_ = true;
}

void SocialSync::applyOwnState(const sc::Companion& c) {
    if (c.state.rev == 0) return;   // never commanded: the engine keeps its own behaviour
    auto it = appliedState_.find(c.npc);
    if (it != appliedState_.end() && it->second == c.state) return;
    if (engine_.applyCompanionState(c.npc, c.state)) { appliedState_[c.npc] = c.state; ++stats_.stateApplied; }
    else ++stats_.stateUnmapped;
}

void SocialSync::cState(const sc::MState& m) {
    if (!book_.setState(m.npc, m.st)) return;
    const sc::Companion* c = book_.find(m.npc);
    if (c && c->owner == cfg_.self) applyOwnState(*c);
}

bool SocialSync::applyInvMirror(const sc::InvDelta& d) {
    if (mirrorSeen_.count(d.txn) && !FNV_MUT(915)) return false;   // BAD (915): a repeated delta is applied again
    mirrorSeen_.insert(d.txn);
    mirrorOrder_.push_back(d.txn);
    while (mirrorOrder_.size() > kMirrorRemember) { mirrorSeen_.erase(mirrorOrder_.front()); mirrorOrder_.erase(mirrorOrder_.begin()); }
    inv_.apply(d);
    if (engine_.applyInventoryDelta(d.npc, d.item, d.delta)) ++stats_.invApplied;
    return true;
}

void SocialSync::cInv(const sc::MInv& m) {
    if (!cfg_.companions) return;
    if (book_.ownerOf(m.d.npc) == cfg_.self && !FNV_MUT(914)) return;   // BAD (914): the owner applies the host's echo of its own change on top of it
    applyInvMirror(m.d);
}

void SocialSync::cInvFull(const sc::MInvFull& m) {
    if (!cfg_.companions) return;
    const bool mine = book_.ownerOf(m.f.npc) == cfg_.self;
    for (const sc::InvDelta& d : inv_.replace(m.f, kHostPlayer, invCounter_))
        if (!mine && engine_.applyInventoryDelta(d.npc, d.item, d.delta)) ++stats_.invApplied;
    ++stats_.invFulls;
}

void SocialSync::cSettings(const sc::MSettings& m) {
    if (m.s.rev <= settings_.rev) return;
    settings_ = m.s;
    book_.setCaps(m.s.caps);
    ++stats_.settingsApplied;
}

void SocialSync::cDigest(const sc::MDigest& m) {
    const bool bad = m.companions != book_.digest() || m.bounty != crime_.bounty().digest() || m.settingsRev != settings_.rev;
    if (!bad) return;
    ++stats_.digestMismatch;
    if (!FNV_MUT(921)) askResync();   // BAD (921): a difference is counted but never repaired
}

void SocialSync::cCmdNack(const sc::MCmdNack&) {
    ++stats_.cmdNacked;
    askResync();   // the roster that comes back carries the host's state for my companions: the optimistic one is replaced
}

void SocialSync::askResync() {
    if (cfg_.isHost) return;
    if (lastResyncMs_ != 0 && nowMs_ >= lastResyncMs_ && nowMs_ - lastResyncMs_ < cfg_.resyncMinMs) return;
    lastResyncMs_ = nowMs_ == 0 ? 1 : nowMs_;
    ++stats_.resyncsAsked;
    send(kHostPlayer, sc::MResync{0});
}

// ---- crime -----------------------------------------------------------------------------------------------------------------------------
void SocialSync::reportCrime(sc::CrimeEvent e) {
    if (!cfg_.crime || !settings_.crime) return;
    if (e.id == 0) e.id = nextId();
    e.reporter = cfg_.self;
    if (e.criminal == kNoPlayer) e.criminal = cfg_.self;
    ++stats_.crimesReported;
    if (cfg_.isHost) judgeCrime(e, cfg_.self);
    else send(kHostPlayer, sc::MCrime{e});
}

void SocialSync::judgeCrime(const sc::CrimeEvent& e, PlayerId sender) {
    if (!cfg_.isHost) return;
    const sc::CrimeDecision d = crime_.judge(e, sender, nowMs_, det_);
    if (!d.counted()) { ++stats_.crimesRefused; return; }
    ++stats_.crimesCounted;
    applyDecision(d);
}

void SocialSync::applyStandingLocal(const sc::BountyUpdate& u) {
    if (engine_.applyStanding(u.faction, u.st)) ++stats_.bountyApplied;
}

void SocialSync::applyDecision(const sc::CrimeDecision& d) {
    for (const sc::BountyUpdate& u : d.bounty) applyStandingLocal(u);
    if (!d.bounty.empty()) {
        sc::MBounty b;
        b.u = d.bounty;
        sendAll(b);
    }
    for (const cc::HostilityEffect& fx : d.hostility) {
        if (engine_.applyHostility(fx)) ++stats_.hostilityApplied;
        sendAll(sc::MHostility{fx});
    }
    for (const sc::AlarmOrder& o : d.alarms) {
        raiseAlarmLocal(o);
        sendAll(sc::MAlarm{o});
    }
    if (!d.reputation.empty()) {
        const std::vector<Effect> fx = sc::reputationEffects(d.reputation, d.blamed, cfg_.shareReputation, connectedMask(), d.nativeMask, bit(kHostPlayer));
        if (!fx.empty()) {
            if (commit_.commit(fx, "crime-reputation") == 0) ++stats_.commitFailed;
            else stats_.repCommitted += fx.size();
        }
        // the host's own part, applied directly (the committed effects skip its slot)
        uint8_t mask = cfg_.shareReputation ? connectedMask() : bit(d.blamed);
        mask = static_cast<uint8_t>(mask & ~d.nativeMask);
        if (mask & bit(kHostPlayer))
            for (const sc::RepChange& r : d.reputation)
                if (engine_.modReputation(r.faction, r.fame, r.infamy)) ++stats_.repLocal;
    }
}

void SocialSync::raiseAlarmLocal(const sc::AlarmOrder& o) {
    if (!alarmsSeen_.insert(o.id).second && !FNV_MUT(907)) { ++stats_.alarmsDuplicate; return; }   // BAD (907): the same alarm alerts the NPCs again
    if (alarmsSeen_.size() > kAlarmRemember) { alarmsSeen_.clear(); alarmsSeen_.insert(o.id); }
    ++stats_.alarmsRaised;
    const uint32_t criminalRef = world_.refOfPlayer(o.criminal);
    if (criminalRef == 0) return;   // the criminal is not in the world here: there is nobody to hunt
    for (const FormRef& npc : world_.factionActorsHere(o.faction, o.cell, o.where, cfg_.crimeRules.alarmRadius))
        if (engine_.alertActor(npc, criminalRef)) ++stats_.actorsAlerted;
}

void SocialSync::forgive(const FormRef& faction) {
    if (!cfg_.isHost) return;
    applyDecision(crime_.forgive(faction, cfg_.self));
}

void SocialSync::onGuestInteraction(PlayerId slot, bool pickup, const FormRef& target) {
    if (!cfg_.isHost || !cfg_.crime || !settings_.crime || slot >= kMaxSlots || slot == kHostPlayer) return;
    ++stats_.guestInteractions;
    sc::InteractionFacts f;
    uint32_t cell = 0;
    cc::Vec3 where;
    if (!engine_.readInteraction(target, pickup, f, cell, where)) return;
    sc::CrimeEvent ev;
    const PlayerId criminal = FNV_MUT(926) ? kHostPlayer : slot;   // BAD (926): the guest's theft is charged to the host
    if (!sc::classifyInteraction(f, criminal, kHostPlayer, nextId(), cell, where, {sc::playerFaction()}, ev)) return;
    ++stats_.crimesReported;
    judgeCrime(ev, kHostPlayer);   // the host reports on behalf of the guest
}

// ---- rewards ---------------------------------------------------------------------------------------------------------------------------
void SocialSync::reportKill(sc::KillReport r) {
    if (!cfg_.rewards) return;
    r.reporter = cfg_.self;
    ++stats_.killsReported;
    if (cfg_.isHost) settleKill(r);
    else send(kHostPlayer, sc::MKill{r});
}

void SocialSync::settleKill(const sc::KillReport& r) {
    if (!cfg_.isHost || !cfg_.rewards) return;
    std::vector<sc::PlayerPlace> places;
    std::set<PlayerId> placed;
    for (const sc::PlayerPlace& p : world_.players())
        if (connected_.count(p.slot) && placed.insert(p.slot).second) places.push_back(p);
    for (PlayerId p : connected_) {   // a player whose place this machine does not know (no puppet, far away) still counts for "everybody connected"
        if (placed.count(p)) continue;
        sc::PlayerPlace u;
        u.slot = p; u.connected = true; u.cell = 0;
        places.push_back(u);
    }
    const std::vector<PlayerId> present = sc::presentPlayers(settings_.scope, places, r.cell, r.where, settings_.nearRadius);
    sc::KillRewards rw;
    if (FNV_MUT(904)) { sc::RewardLedger fresh; rw = fresh.settle(r, settings_, present, connectedMask(), cfg_.combatRules); }   // BAD (904): every report settles on its own, so a kill two machines report pays twice
    else rw = rewards_.settle(r, settings_, present, connectedMask(), cfg_.combatRules);
    if (!rw.settled) return;
    ++stats_.killsSettled;
    const std::vector<Effect> fx = sc::rewardEffects(rw, FNV_MUT(905) ? uint8_t(0) : bit(kHostPlayer));   // BAD (905): the host's own part is also committed
    if (!fx.empty() && !FNV_MUT(906)) {   // BAD (906): the other players' parts are never committed
        if (commit_.commit(fx, "kill") == 0) ++stats_.commitFailed;
        else for (const Effect& e : fx) { if (std::holds_alternative<fnvcoop::ModActorValue>(e)) ++stats_.xpCommitted; else ++stats_.karmaCommitted; }
    }
    for (const sc::XpGrant& g : rw.xp)
        if (g.slot == kHostPlayer && engine_.awardXp(g.xp)) ++stats_.xpLocal;
    if (rw.karma != 0 && (rw.karmaTo & bit(kHostPlayer)) && engine_.rewardKarma(rw.karma)) ++stats_.karmaLocal;
}

void SocialSync::setSettings(const sc::SocialSettings& s) {
    if (!cfg_.isHost) return;
    const uint32_t rev = settings_.rev;
    settings_ = s;
    settings_.rev = rev + 1;
    book_.setCaps(settings_.caps);
    if (!FNV_MUT(924)) sendAll(sc::MSettings{settings_});   // BAD (924): the other machines keep the old settings
}

// ---- companion commands ----------------------------------------------------------------------------------------------------------------
bool SocialSync::issueCommand(sc::CmdKind kind, const FormRef& npc, uint8_t arg) {
    if (!cfg_.companions) return false;
    sc::CompanionCmd cmd;
    cmd.id = nextId(); cmd.from = cfg_.self; cmd.npc = npc; cmd.kind = kind; cmd.arg = arg;
    ++stats_.cmdIssued;
    const sc::CmdVerdict v = localJudge_.judge(cmd, cfg_.self, nowMs_, book_);   // owner-only, bad arguments, floods: refused before anything happens
    if (!v.apply) { ++stats_.cmdRefused; return false; }
    if (v.stateChanged) {
        book_.setState(npc, v.newState);
        // Only the machine that simulates the companion (its recruiter) writes the engine state; a stranger ordering a party companion sends the order to the host and
        // the owner applies the MState that comes back.
        if (book_.ownerOf(npc) == cfg_.self || FNV_MUT(941)) {   // BAD (941): the commander applies a party order to its own copy of the companion
            if (engine_.applyCompanionState(npc, v.newState)) { appliedState_[npc] = v.newState; ++stats_.stateApplied; }
            else ++stats_.stateUnmapped;
        }
    }
    if (kind == sc::CmdKind::Summon && book_.ownerOf(npc) == cfg_.self) {   // (the judge already refused a non-owner; this is the second lock)
        if (engine_.summonCompanion(npc, cfg_.self)) ++stats_.summons;
    }
    if (cfg_.isHost) {
        if (v.stateChanged) sendAll(sc::MState{npc, v.newState});
        if (v.dismiss) releaseCompanion(npc, "dismissed");
    } else {
        send(kHostPlayer, sc::MCmd{cmd});
    }
    return true;
}

bool SocialSync::allowActivation(PlayerId who, const FormRef& npc) {
    if (!cfg_.companions) return true;
    const sc::ActivationVerdict v = sc::gateActivation(book_, who, npc, FNV_MUT(927) ? sc::ForeignPolicy::Allow : settings_.foreignCompanions);   // BAD (927): the setting is ignored
    if (v.suppress()) { ++stats_.activationsDenied; return false; }
    return true;
}

static sc::Combatant combatantOfEntity(const cc::Entity& e, ISocialWorld& world, const sc::CompanionBook& book) {
    if (e.isPlayer() && e.id < kMaxSlots) return sc::Combatant::player(static_cast<PlayerId>(e.id));
    if (e.isActor()) {
        ActorView v;
        if (world.actorByNet(e.id, v) && v.known) {
            const PlayerId o = book.ownerOf(v.npc);
            if (o != kNoPlayer && !FNV_MUT(928)) return sc::Combatant::companion(o);   // BAD (928): somebody's companion is "anybody" to the party rules
        }
    }
    return sc::Combatant::other();
}

sc::HitPolicy SocialSync::hitPolicy(const cc::Entity& attacker, const cc::Entity& victim) {
    if (!cfg_.companions) return sc::HitPolicy::Allow;
    const sc::HitPolicy p = sc::companionHitPolicy(combatantOfEntity(attacker, world_, book_), combatantOfEntity(victim, world_, book_), settings_.friendlyFire);
    if (p == sc::HitPolicy::Neutralize) ++stats_.hitsNeutralized;
    return p;
}

bool SocialSync::vatsTargetAllowed(const cc::Entity& target) {
    if (!cfg_.companions) return true;
    return sc::vatsTargetAllowed(combatantOfEntity(target, world_, book_));
}

PlayerId SocialSync::responsiblePlayer(const cc::Entity& e) { return combatantOfEntity(e, world_, book_).responsible(); }

// ---- periodic ----------------------------------------------------------------------------------------------------------------------------
void SocialSync::tick(uint64_t nowMs) {
    nowMs_ = nowMs;
    if (cfg_.isHost) {
        if (cfg_.crime && settings_.crime && (lastCrimeTickMs_ == 0 || nowMs - lastCrimeTickMs_ >= 1000)) {
            lastCrimeTickMs_ = nowMs ? nowMs : 1;
            det_.expire(nowMs);
            for (const sc::BountyUpdate& u : crime_.tick(nowMs)) {
                applyStandingLocal(u);
                sc::MBounty b;
                b.u = {u};
                sendAll(b);
            }
        }
        if (connected_.size() > 1 && (lastDigestMs_ == 0 || nowMs - lastDigestMs_ >= cfg_.digestMs)) {
            lastDigestMs_ = nowMs ? nowMs : 1;
            sendAll(sc::MDigest{book_.digest(), crime_.bounty().digest(), settings_.rev});
        }
    }
    if (cfg_.companions) {
        if (cfg_.isHost && !seeded_) seedHostCompanions();
        if (cfg_.isHost) { flushRepairs(); refreshParty(); }
        pollEscort();
        if (dirtyReconcile_ || lastReconcileMs_ == 0 || nowMs - lastReconcileMs_ >= cfg_.reconcileMs) reconcileTeammates();
        if (lastInvMs_ == 0 || nowMs - lastInvMs_ >= cfg_.inventoryMs) sampleInventories();
    }
    if (cfg_.crime && settings_.crime) {
        pollDetections();
        pollTrespass();
    }
}

// The host's own followers from before the session began (a save loaded with Boone already in the party): nobody committed a CompanionOwner for them
// in THIS session, so they are adopted once, as the host's. Anything flagged later that the book does not know is a stray and comes off.
void SocialSync::seedHostCompanions() {
    seeded_ = true;
    std::vector<Effect> adopted;
    for (const FormRef& npc : world_.teammatesHere()) {
        if (book_.isCompanion(npc)) continue;
        ActorView v;
        const bool critter = world_.actor(npc, v) && v.known && v.critter;
        const fnvcoop::CompanionOwner x{npc, kHostPlayer, true, static_cast<uint8_t>(critter ? 1 : 0)};
        onEffect(x);
        if (book_.isCompanion(npc)) adopted.push_back(x);
    }
    // They are committed too: the story session (follower caps for the other recruiters), the dialogue registry (Boone's "hired" flag for the host's own view) and a
    // late joiner's baseline only know a companion that went through the session.
    if (!adopted.empty() && !FNV_MUT(933)) {   // BAD (933): the adopted companions stay in the book only, so the host sees Boone's recruit topic again
        if (commit_.commit(adopted, "seed-companions") == 0) ++stats_.commitFailed;
        else stats_.seedCommitted += adopted.size();
    }
}

// Escort: the OWNER's machine brings its own simulated companions to its player when a scripted move left them behind (Fort entry and exit, the Dam start, PartyWarp).
void SocialSync::pollEscort() {
    if (!cfg_.escort || FNV_MUT(939)) return;   // BAD (939): the escort never runs, so a warped recruiter's companions stay at the old place
    if (lastEscortMs_ != 0 && nowMs_ - lastEscortMs_ < cfg_.escortMs) return;
    lastEscortMs_ = nowMs_ ? nowMs_ : 1;
    sc::PlayerPlace me;
    bool haveMe = false;
    for (const sc::PlayerPlace& p : world_.players()) if (p.slot == cfg_.self) { me = p; haveMe = true; break; }
    if (!haveMe || !synced_) return;
    std::vector<sc::EscortSubject> mine;
    for (const sc::Companion& c : book_.roster()) {
        if (c.owner != cfg_.self && !FNV_MUT(940)) continue;   // BAD (940): every machine brings every companion, so a recruiter's companion is moved by a stranger
        ActorView v;
        if (!world_.actor(c.npc, v) || !v.known || (!v.simulatedHere && !FNV_MUT(940))) continue;   // (BAD 940 also brings what another machine simulates)
        sc::EscortSubject s;
        s.npc = c.npc; s.loaded = v.ref != 0 && !v.standIn; s.alive = v.alive;
        s.waiting = v.waiting || c.state.mode == sc::FollowMode::Wait;
        s.cell = v.cell; s.pos = v.pos;
        mine.push_back(s);
    }
    for (const FormRef& npc : escort_.poll(nowMs_, me, mine)) {
        if (engine_.summonCompanion(npc, cfg_.self)) ++stats_.escorted; else ++stats_.escortFailed;
    }
}

void SocialSync::reconcileTeammates() {
    lastReconcileMs_ = nowMs_ ? nowMs_ : 1;
    dirtyReconcile_ = false;
    if (!hostUp_) return;
    if (cfg_.isHost && !FNV_MUT(932)) {   // BAD (932): a companion that died keeps its recruiter's follower slot for good
        for (const sc::Companion& c : book_.roster()) {
            ActorView v;
            if (world_.actor(c.npc, v) && v.known && !v.alive && !v.standIn) releaseCompanion(c.npc, "companion-died");
        }
    }
    if (!synced_ && !FNV_MUT(911)) return;   // BAD (911): a client that has not heard the host yet takes its own companion off before it knows it is its own
    std::set<FormRef> flagged;
    for (const FormRef& f : world_.teammatesHere()) flagged.insert(f);
    std::set<FormRef> loaded;
    for (const sc::Companion& c : book_.roster()) {
        ActorView v;
        if (world_.actor(c.npc, v) && v.known && v.ref != 0 && !v.standIn) loaded.insert(c.npc);
    }
    const sc::TeammateFix fix = sc::reconcileTeammates(book_, cfg_.self, flagged, loaded);
    auto cooled = [&](const FormRef& n) {
        auto it = lastFixMs_.find(n);
        if (it != lastFixMs_.end() && nowMs_ >= it->second && nowMs_ - it->second < cfg_.fixCooldownMs && !FNV_MUT(912)) return false;   // BAD (912): the same NPC is flipped every pass
        lastFixMs_[n] = nowMs_ ? nowMs_ : 1;
        return true;
    };
    for (const FormRef& n : fix.unset) {
        if (!cooled(n)) continue;
        if (engine_.setTeammate(n, false)) ++stats_.teammateUnset; else ++stats_.teammateFailed;
    }
    for (const FormRef& n : fix.set) {
        if (!cooled(n)) continue;
        if (engine_.setTeammate(n, true)) ++stats_.teammateSet; else ++stats_.teammateFailed;
    }
}

void SocialSync::sampleInventories() {
    lastInvMs_ = nowMs_ ? nowMs_ : 1;
    if (FNV_MUT(913)) return;   // BAD (913): the owner never reports its companions' inventory changes
    for (const sc::Companion& c : book_.roster()) {
        if (c.owner != cfg_.self) continue;
        ActorView v;
        if (!world_.actor(c.npc, v) || !v.known || v.ref == 0 || !v.simulatedHere) continue;
        sc::ItemCounts now;
        if (!engine_.readInventory(c.npc, now)) continue;
        for (const sc::InvDelta& d : inv_.sample(c.npc, now, cfg_.self, invCounter_)) {
            ++stats_.invSent;
            inv_.apply(d);   // my own picture follows what I send
            if (cfg_.isHost) sendAll(sc::MInv{d}, kNoPlayer, cfg_.self);
            else send(kHostPlayer, sc::MInv{d});
        }
    }
}

void SocialSync::pollDetections() {
    if (lastDetectMs_ != 0 && nowMs_ - lastDetectMs_ < cfg_.detectMs) return;
    lastDetectMs_ = nowMs_ ? nowMs_ : 1;
    std::vector<sc::Detection> cur;
    engine_.observeDetections(cur);
    std::set<std::pair<uint32_t, PlayerId>> keys;
    auto publish = [&](const sc::Detection& d) {
        ++stats_.detectSent;
        if (cfg_.isHost) det_.report(d, nowMs_);
        else send(kHostPlayer, sc::MDetect{d});
    };
    for (sc::Detection d : cur) {
        if (!d.observer.isActor() || d.target >= kMaxSlots) continue;
        d.seen = true;
        const auto key = std::make_pair(d.observer.id, d.target);
        keys.insert(key);
        auto it = detectSent_.find(key);
        const bool stale = it == detectSent_.end() || (nowMs_ - it->second >= cfg_.detectRefreshMs && !FNV_MUT(920));   // BAD (920): a sighting is announced once and then goes stale at the host
        if (stale) { detectSent_[key] = nowMs_; lastSeen_[key] = d; publish(d); }
    }
    for (auto it = detectSent_.begin(); it != detectSent_.end();) {
        if (keys.count(it->first)) { ++it; continue; }
        sc::Detection lost = lastSeen_[it->first];
        lost.seen = false;
        publish(lost);
        lastSeen_.erase(it->first);
        it = detectSent_.erase(it);
    }
}

void SocialSync::pollTrespass() {
    if (lastTrespassMs_ != 0 && nowMs_ - lastTrespassMs_ < cfg_.trespassMs) return;
    lastTrespassMs_ = nowMs_ ? nowMs_ : 1;
    sc::InteractionFacts f;
    uint32_t cell = 0;
    cc::Vec3 where;
    if (!engine_.localTrespass(f, cell, where)) return;
    auto it = trespassAt_.find(cell);
    if (it != trespassAt_.end() && nowMs_ - it->second < cfg_.trespassCooldownMs && !FNV_MUT(919)) return;   // BAD (919): every poll while inside is another report
    sc::CrimeEvent ev;
    if (!sc::classifyInteraction(f, cfg_.self, cfg_.self, nextId(), cell, where, {sc::playerFaction()}, ev)) return;
    trespassAt_[cell] = nowMs_;
    ++stats_.trespassReports;
    reportCrime(ev);
}

// ---- status ------------------------------------------------------------------------------------------------------------------------------
std::string SocialSync::statusJson() const {
    char b[3072];
    std::snprintf(b, sizeof b,
                  "{\"host\":%s,\"self\":%u,\"synced\":%s,\"companions\":%zu,\"companionDigest\":\"%016llx\",\"bounties\":%zu,\"bountyDigest\":\"%016llx\",\"settingsRev\":%u,"
                  "\"xpMode\":%u,\"stats\":{\"sent\":%llu,\"received\":%llu,\"dropped\":%llu,\"spoofed\":%llu,"
                  "\"rosterApplied\":%llu,\"recruitsDropped\":%llu,\"released\":%llu,\"teammateUnset\":%llu,\"teammateSet\":%llu,"
                  "\"cmdIssued\":%llu,\"cmdAccepted\":%llu,\"cmdNacked\":%llu,\"stateApplied\":%llu,\"stateUnmapped\":%llu,"
                  "\"invSent\":%llu,\"invApplied\":%llu,\"invRejected\":%llu,\"activationsDenied\":%llu,\"hitsNeutralized\":%llu,"
                  "\"crimesReported\":%llu,\"crimesCounted\":%llu,\"crimesRefused\":%llu,\"detectSent\":%llu,\"bountyApplied\":%llu,\"hostilityApplied\":%llu,"
                  "\"alarmsRaised\":%llu,\"actorsAlerted\":%llu,\"killsReported\":%llu,\"killsSettled\":%llu,\"xpLocal\":%llu,\"xpCommitted\":%llu,"
                  "\"karmaLocal\":%llu,\"karmaCommitted\":%llu,\"repLocal\":%llu,\"repCommitted\":%llu,\"resyncsAsked\":%llu,\"digestMismatch\":%llu,"
                  "\"partyFlagged\":%llu,\"partyHanded\":%llu,\"swapsReverted\":%llu,\"slotDenied\":%llu,\"repairsCommitted\":%llu,\"seedCommitted\":%llu,\"escorted\":%llu,\"escortFailed\":%llu}}",
                  cfg_.isHost ? "true" : "false", static_cast<unsigned>(cfg_.self), synced_ ? "true" : "false", book_.total(),
                  static_cast<unsigned long long>(book_.digest()), crime_.bounty().size(), static_cast<unsigned long long>(crime_.bounty().digest()), settings_.rev,
                  static_cast<unsigned>(settings_.xpMode),
                  (unsigned long long)stats_.sent, (unsigned long long)stats_.received, (unsigned long long)stats_.dropped, (unsigned long long)stats_.spoofed,
                  (unsigned long long)stats_.rosterApplied, (unsigned long long)stats_.recruitsDropped, (unsigned long long)stats_.released,
                  (unsigned long long)stats_.teammateUnset, (unsigned long long)stats_.teammateSet,
                  (unsigned long long)stats_.cmdIssued, (unsigned long long)stats_.cmdAccepted, (unsigned long long)stats_.cmdNacked, (unsigned long long)stats_.stateApplied,
                  (unsigned long long)stats_.stateUnmapped, (unsigned long long)stats_.invSent, (unsigned long long)stats_.invApplied, (unsigned long long)stats_.invRejected,
                  (unsigned long long)stats_.activationsDenied, (unsigned long long)stats_.hitsNeutralized,
                  (unsigned long long)stats_.crimesReported, (unsigned long long)stats_.crimesCounted, (unsigned long long)stats_.crimesRefused, (unsigned long long)stats_.detectSent,
                  (unsigned long long)stats_.bountyApplied, (unsigned long long)stats_.hostilityApplied, (unsigned long long)stats_.alarmsRaised, (unsigned long long)stats_.actorsAlerted,
                  (unsigned long long)stats_.killsReported, (unsigned long long)stats_.killsSettled, (unsigned long long)stats_.xpLocal, (unsigned long long)stats_.xpCommitted,
                  (unsigned long long)stats_.karmaLocal, (unsigned long long)stats_.karmaCommitted, (unsigned long long)stats_.repLocal, (unsigned long long)stats_.repCommitted,
                  (unsigned long long)stats_.resyncsAsked, (unsigned long long)stats_.digestMismatch,
                  (unsigned long long)stats_.partyFlagged, (unsigned long long)stats_.partyHanded, (unsigned long long)stats_.swapsReverted, (unsigned long long)stats_.slotDenied,
                  (unsigned long long)stats_.repairsCommitted, (unsigned long long)stats_.seedCommitted, (unsigned long long)stats_.escorted, (unsigned long long)stats_.escortFailed);
    return b;
}

}  // namespace nvcoop::social
