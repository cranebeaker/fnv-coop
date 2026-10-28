#include "fnvcoop/session.h"
#include <algorithm>
#include <sstream>

namespace fnvcoop {

// ================================ HostSession ==================================================

namespace {
// Identity of a shared shadow for a resume cursor (Hello::cursorCheck): its shard hashes and seq. Never 0 (0 = no check).
uint64_t cursorHashOf(const ShadowState& s) {
    uint64_t h = 1469598103934665603ull;
    auto mixIn = [&](uint64_t v) { for (int i = 0; i < 8; ++i) { h ^= (v >> (8 * i)) & 0xFF; h *= 1099511628211ull; } };
    mixIn(s.seq);
    for (uint32_t x : shardHashesOf(s)) mixIn(x);
    return h ? h : 1;
}
struct RepairItem {
    HostSession::DesyncEntry d;
    bool clientOnly = false, race = false, revertToo = false;   // revertToo: additive entry, the client takes its own amount back before the host's is added
    bool questRunning = false;   // client-only StartQuest / StopQuest entry: settled by the host's own running state (issue #25), fix = the client's entry
    bool stageZero = false;      // client-only SetStage q = 0 entry (quest not started): the host commits the same stage 0, fix = the client's entry
    uint8_t questState = 0;      // client-only real save state: 1 SetStage q = n (n > 0), 2 CompleteQuest, 3 FailQuest; adopted only when the host engine confirms it
    bool objectiveZero = false;  // client-only SetObjective flags 0 entry (objective not displayed, not completed): the host commits the same flags 0, fix = the client's entry
    uint8_t objectiveState = 0;  // client-only real objective state: 1 SetObjective with non-zero flags, 2 CompleteAllObjectives; adopted only when the host engine confirms it
    bool scriptVar = false;      // client-only SetScriptVar entry (a quest / ref script var both copies loaded from the same save, never recorded by the baseline-less host): fix = the client's entry
    bool global = false;         // client-only SetGlobal entry (same story as scriptVar): fix = the client's entry
    bool acked = false;          // already acknowledged as a known divergence (matching values): listed, never repaired
    bool dropAck = false;        // marker, not an entry: an acknowledgement whose two sides no longer differ in the way it was made for
    bool hostHas = false, clientHas = false;
    uint32_t hostVal = 0, clientVal = 0;   // value hashes of the two sides' entries for this key
    uint8_t cls = 0;
    bool noInverse = false;      // client-only entry of a kind the client can never revert (revertEffectFor has no case for it): a revert key would be skipped by the client and resent every round
    Effect fix;
    uint64_t key = 0;
};
}  // namespace

struct HostSession::Sync {
    uint32_t id = 0;
    uint64_t seq = 0, at = 0;
    std::shared_ptr<const ShadowState> state;   // host state as of seq
    std::vector<size_t> shards;
    size_t badTotal = 0;   // mismatching shards found, of which `shards` were asked
};

struct HostSession::Job {
    bool computing = false;
    uint32_t sentUpTo = 0, backoff = 0;
    uint32_t id = 0;
    std::vector<RewardGrant> grantsAtH;   // the joiner slot's rewards as of snapAtH
    ShadowState snapAtH;   // frozen at Welcome time (baseline + session txns): the transfer is relative to THIS point
    std::map<uint32_t, BaselineSummaryChunk> got;
    uint32_t summaryTotal = 0;
    bool built = false, done = false;
    std::vector<BaselineDiffChunk> chunks;
    uint32_t acked = 0;
    uint64_t lastSend = 0;
};

void HostSession::setWorldBaseline(const ShadowState& baseline, uint64_t contentHash) {
    ShadowState b = baseline;
    for (auto& k : baseKarma_) k = b.ledger[0].karma;
    for (auto& r : baseRep_) r = b.ledger[0].rep;
    b.ledger[0] = Ledger();
    contentHash_ = contentHash;
    haveBaseline_ = log_.setInitialState(b);  // must be called before the first commit
}

std::string checkHandshake(const HandshakeInfo& host, const HandshakeInfo& client, uint8_t& reason) {
    auto num = [](uint64_t v) { return std::to_string(v); };
    auto name = [](const HandshakeInfo& i) { return i.buildName.empty() ? "build " + std::to_string(i.buildNumber) : i.buildName; };
    if (client.protocol != host.protocol) {
        reason = kRejectProtocol;
        return client.protocol < host.protocol
                   ? "your fnv-coop is older than the host's (protocol " + num(client.protocol) + " < " + num(host.protocol) + "): update fnv-coop"
                   : "the host has an older fnv-coop (protocol " + num(host.protocol) + ", you have " + num(client.protocol) + "): ask the host to update";
    }
    if ((host.buildNumber || host.buildHash) && (client.buildNumber != host.buildNumber || client.buildHash != host.buildHash)) {
        reason = kRejectBuild;
        if (client.buildNumber < host.buildNumber) return "the host has a newer fnv-coop build (" + name(host) + ") than yours (" + name(client) + "): update fnv-coop";
        if (client.buildNumber > host.buildNumber) return "the host has an older fnv-coop build (" + name(host) + ") than yours (" + name(client) + "): ask the host to update";
        return "different fnv-coop binaries with the same version (" + name(host) + "): reinstall the same package as the host";
    }
    if (host.rulesHash && client.rulesHash != host.rulesHash) {
        reason = kRejectRules;
        return "your quest rules (rules.json) differ from the host's: reinstall the same fnv-coop package";
    }
    if (host.volatileHash && client.volatileHash != host.volatileHash) {
        reason = kRejectVolatile;
        return "your volatile-keys list differs from the host's: reinstall the same fnv-coop package";
    }
    if (!host.plugins.empty() && client.plugins != host.plugins) {
        reason = kRejectModList;
        std::set<std::string> h(host.plugins.begin(), host.plugins.end()), c(client.plugins.begin(), client.plugins.end());
        std::string missing, extra;
        for (auto& p : host.plugins) if (!c.count(p) && missing.size() < 120) missing += (missing.empty() ? "" : ", ") + p;
        for (auto& p : client.plugins) if (!h.count(p) && extra.size() < 120) extra += (extra.empty() ? "" : ", ") + p;
        std::string msg = "different mod list";
        if (!missing.empty()) msg += ": you are missing " + missing;
        if (!extra.empty()) msg += (missing.empty() ? ": " : "; ") + std::string("you have extra ") + extra;
        if (missing.empty() && extra.empty()) msg += ": same mods but a different load order";
        return msg;
    }
    return std::string();
}

HostSession::HostSession(ITransport& t, SharePolicy policy, Options opt) : t_(t), policy_(policy), opt_(opt) {
    if (policy_.maxPlayers < 1) policy_.maxPlayers = 1;
    if (policy_.maxPlayers > kMaxSlots) policy_.maxPlayers = kMaxSlots;
}

void HostSession::sendTo(PeerId to, const Message& m) {
    Bytes b = encodeMessage(m);
    if (!b.empty()) t_.send(to, std::move(b));
}

HostSession::Client* HostSession::byPeer(PeerId p) {
    for (auto& [tok, c] : clients_) if (c.peer == p && c.slot != kNoPlayer) return &c;
    return nullptr;
}

uint64_t HostSession::commit(PlayerId origin, const Cause& cause, std::vector<Effect> effects, uint8_t flags,
                             uint32_t originEpoch, uint32_t originIntent) {
    EffectTxn t;
    t.origin = origin;
    t.cause = cause;
    t.effects = std::move(effects);
    t.flags = flags;
    t.originEpoch = originEpoch;
    t.originIntent = originIntent;
    if (txnFilter) txnFilter(t);
    stats_.followerCapDrops += enforceFollowerCaps(t);
    size_t dropped = normalizeTxn(t, policy_);
    if (dropped == kNormalizeInvalid) return 0;
    stats_.droppedByPolicy += dropped;
    if (t.effects.empty()) return 0;
    uint64_t seq = log_.append(std::move(t));
    if (opt_.autoCompactKeep && log_.size() > opt_.autoCompactKeep * 2) log_.compact(opt_.autoCompactKeep);
    const EffectTxn& stored = *log_.get(seq);
    forEachReward(stored, epoch_, [&](PlayerId slot, uint64_t id, const Effect& e) { if (slot != kHostPlayer) grants_[slot].push_back({id, e}); });
    ++stats_.commits;
    if (onCommit) onCommit(stored);
    for (auto& [tok, c] : clients_) {
        if (!c.joined) continue;
        TxnBatch b;
        b.session = c.session;
        b.worldEpoch = epoch_;
        b.txns.push_back(stored);
        // A client that is far behind would only buffer (and later drop) live txns: it is caught up by the retransmit timer.
        // Live sends must NOT touch the retransmit timer, or a steady stream of commits starves the retransmits forever.
        if (seq - std::min<uint64_t>(seq, c.acked) <= 32) sendTo(c.peer, b);
    }
    return seq;
}

uint32_t HostSession::allocCreatedId() {
    uint32_t cur = std::max(createdNext_, log_.headState().createdCounter);
    createdNext_ = cur + 1;
    return kCreatedRefPrefix | (createdNext_ & 0x00FFFFFFu);
}

// Companion recruits beyond the per-owner (per kind) or party-wide follower caps are dropped; dismisses always pass.
size_t HostSession::enforceFollowerCaps(EffectTxn& t) {
    if (companionVet && !mutations().companionVetIgnored) stats_.companionVetDrops += companionVet(t);   // W3a item 6: swaps and slot overruns never reach the wire
    if (mutations().ignoreFollowerCaps) return 0;
    bool any = false;
    for (auto& e : t.effects) if (std::holds_alternative<CompanionOwner>(e)) { any = true; break; }
    if (!any) return 0;
    const ShadowState& st = log_.headState();
    std::map<FormRef, uint8_t> owner = st.companionOwner, kind = st.companionKind;
    size_t dropped = 0;
    std::vector<Effect> kept;
    kept.reserve(t.effects.size());
    for (auto& e : t.effects) {
        auto* c = std::get_if<CompanionOwner>(&e);
        if (!c) { kept.push_back(std::move(e)); continue; }
        if (!c->active) { owner.erase(c->npc); kind.erase(c->npc); kept.push_back(std::move(e)); continue; }
        if (isPartyCompanion && isPartyCompanion(c->npc)) { owner[c->npc] = c->owner; kind[c->npc] = c->kind; kept.push_back(std::move(e)); continue; }   // the party's own: no cap
        size_t mine = 0, total = 0;
        for (auto& [npc, o] : owner) {
            if (npc == c->npc) continue;
            if (isPartyCompanion && isPartyCompanion(npc)) continue;
            ++total;
            auto kt = kind.find(npc);
            if (o == c->owner && (kt == kind.end() ? 0 : kt->second) == c->kind) ++mine;
        }
        const size_t perOwnerCap = c->kind == 0 ? opt_.followers.humanoidPerOwner : opt_.followers.critterPerOwner;
        if (mine >= perOwnerCap || total >= opt_.followers.partyTotal) { ++dropped; continue; }
        owner[c->npc] = c->owner; kind[c->npc] = c->kind;
        kept.push_back(std::move(e));
    }
    t.effects = std::move(kept);
    return dropped;
}

bool HostSession::bulkOk(Client& c) {
    const double rate = double(opt_.bulkBytesPerSec);
    if (c.bulkTokens < 0 && c.bulkAt == 0) { c.bulkTokens = rate; c.bulkAt = now_ ? now_ : 1; }
    c.bulkTokens = std::min(rate, c.bulkTokens + rate * double(now_ - c.bulkAt) / 1000.0);
    c.bulkAt = now_;
    return c.bulkTokens >= 0 && t_.backlogBytes(c.peer) <= opt_.maxBacklogBytes;
}

void HostSession::sendBulk(Client& c, const Message& m) {
    Bytes b = encodeMessage(m);
    if (b.empty()) return;
    c.bulkTokens -= double(b.size());   // may go negative: the debt throttles what follows
    t_.send(c.peer, std::move(b));
}

void HostSession::sendWelcome(Client& c, uint64_t lastSeq, bool fresh) {
    if (!fresh && !mutations().chattyBaseline) {
        // A repeated Hello means the client has not seen our Welcome yet. Answer idempotently (same job, same snapshot),
        // but not on every retry: Welcomes carry the whole snapshot and would pile up behind each other.
        uint64_t gap = uint64_t(opt_.welcomeResendMs) << std::min<uint32_t>(c.welcomeResends, 4);
        if (now_ < c.lastWelcomeAt + gap || !bulkOk(c)) { ++stats_.welcomesThrottled; return; }
        ++c.welcomeResends;
    }
    if (fresh) c.welcomeResends = 0;
    c.lastWelcomeAt = now_ ? now_ : 1;
    c.welcomedEpoch = epoch_;
    Welcome w;
    w.session = c.session;
    w.slot = c.slot;
    w.headSeq = log_.head();
    w.policy = policy_;
    w.worldEpoch = epoch_;
    w.intentEpoch = c.intentEpoch;
    w.lastIntent = c.lastIntent;
    bool replay = lastSeq <= log_.head() && log_.canReplayFrom(lastSeq) && !c.job;
    uint64_t base = lastSeq;
    if (c.job) {  // baseline join: always the snapshot frozen when the job started
        w.hasSnapshot = true;
        w.snapshot = c.job->snapAtH;
        w.baselineId = c.job->id;
        w.baselineAbs = static_cast<uint8_t>((policy_.karma ? 1 : 0) | (policy_.reputation ? 2 : 0));
        w.rewards = c.job->grantsAtH;
        base = c.job->snapAtH.seq;
        ++stats_.snapshots;
    } else if (!replay) {
        w.hasSnapshot = true;
        w.snapshot = log_.headState();
        w.rewards = grants_[c.slot];
        base = log_.head();
        ++stats_.snapshots;
    } else {
        ++stats_.replays;
    }
    // A repeated Welcome for the same session must not raise `acked`: only the client's Acks may do that.
    if (fresh) c.acked = base;
    c.lastSend = now_;
    c.lastRetx = now_;
    c.retxBackoff = 0;
    c.sendNow = replay;  // first batch goes out on the next tick
    ++stats_.welcomes;
    sendBulk(c, w);
}

void HostSession::sendSnapshotWelcome(Client& c) {
    if (c.job && !c.job->done) { sendWelcome(c, c.acked, false); return; }  // keep the frozen baseline reference
    Welcome w;
    w.session = c.session;
    w.slot = c.slot;
    w.headSeq = log_.head();
    w.policy = policy_;
    w.worldEpoch = epoch_;
    w.intentEpoch = c.intentEpoch;
    w.lastIntent = c.lastIntent;
    w.hasSnapshot = true;
    w.snapshot = log_.headState();
    w.rewards = grants_[c.slot];
    ++stats_.snapshots;
    c.lastSend = now_;
    sendTo(c.peer, w);
}

void HostSession::handle(PeerId from, const Message& m) {
    if (auto* h = std::get_if<Hello>(&m)) {
        const bool wantBase = (h->flags & kHelloWantBaseline) != 0;
        if (!mutations().skipHandshake) {
            uint8_t why = 0;
            std::string text = checkHandshake(hs_, h->hs, why);
            if (!text.empty()) { ++stats_.handshakeRejects; Reject r; r.session = h->session; r.reason = why; r.text = text; sendTo(from, r); return; }
        }
        if (wantBase && !haveBaseline_) { ++stats_.rejected; { Reject r; r.session = h->session; r.reason = kRejectNoBaseline; r.text = "the host did not prepare a world baseline for this session"; sendTo(from, r); } return; }
        if (wantBase && h->contentHash != contentHash_) { ++stats_.rejected; { Reject r; r.session = h->session; r.reason = kRejectContent; r.text = "your mod list differs from the host's: the world cannot be synchronized"; sendTo(from, r); } return; }
        if (haveBaseline_ && !wantBase && h->lastEpoch != epoch_) {  // state from another world epoch: it cannot resume
            ++stats_.rebasesSent;
            sendTo(from, Rebase{h->session, epoch_});
            return;
        }
        auto it = clients_.find(h->token);
        if (it == clients_.end()) {
            std::vector<bool> used(policy_.maxPlayers, false);
            used[0] = true;  // host
            for (auto& [tok, c] : clients_) if (c.slot < used.size()) used[c.slot] = true;
            auto free = std::find(used.begin(), used.end(), false);
            if (free == used.end()) {
                ++stats_.rejected;
                { Reject r; r.session = h->session; r.reason = kRejectFull; r.text = "the session is full"; sendTo(from, r); }
                return;
            }
            Client c;
            c.slot = static_cast<PlayerId>(free - used.begin());
            it = clients_.emplace(h->token, c).first;
        }
        Client& c = it->second;
        if (c.session != 0 && h->session < c.session) return;  // stale hello from an earlier connect
        for (auto& [tok, o] : clients_) if (&o != &c && o.peer == from) o.peer = kNoPeer;
        c.peer = from;
        bool fresh = h->session != c.session;
        if (!fresh && !c.joined) return;  // stale Hello of a session that was invalidated by a world epoch change
        // A resume cursor (lastEpoch, lastSeq) from the client's own save sidecar is only meaningful against the timeline it was saved in.
        // Epoch numbers are not unique across host runs: a fresh host that loads its prepared save reaches the same epoch number the
        // client's (newer) sidecar was saved in during an earlier run, with a different history. Replaying from that cursor applies the
        // wrong txns, and a snapshot diff from it rewinds per-player state (rewards the client's save holds are taken away); a rebase does not.
        // The client sends a hash of its shadow at the cursor (Hello::cursorCheck) and the host compares it with its own state at that seq.
        // A cursor past our head is foreign for certain; one the log no longer holds is ours only if we welcomed this token in this epoch.
        bool foreign = false;
        if (fresh && haveBaseline_ && !wantBase && !mutations().trustForeignCursor) {
            foreign = h->lastSeq > log_.head();
            if (!foreign && h->cursorCheck != 0) {
                ShadowState at;
                if (log_.stateAt(h->lastSeq, at)) foreign = cursorHashOf(at) != h->cursorCheck;
                else foreign = c.welcomedEpoch != epoch_;
            }
        }
        if (foreign) {
            ++stats_.resumesRefused;
            ++stats_.rebasesSent;
            sendTo(from, Rebase{h->session, epoch_});
            return;
        }
        if (fresh && c.session != 0) releaseControlsOf(c.slot);   // a returning player never comes back frozen (their client released locally too)
        if (fresh || (mutations().chattyBaseline && wantBase)) {
            c.session = h->session;
            c.joined = true;
            c.rebasePending = false;
            c.job.reset();
            if (wantBase) {
                c.job = std::make_shared<Job>();
                c.job->id = static_cast<uint32_t>(++baselineCounter_);
                c.job->snapAtH = log_.headState();
                c.job->grantsAtH = grants_[c.slot];
                ++stats_.baselineJoins;
            }
        }
        sendWelcome(c, h->lastSeq, fresh);
        return;
    }
    Client* c = byPeer(from);
    if (!c || !c->joined) return;
    if (auto* a = std::get_if<Ack>(&m)) {
        if (a->session != c->session) return;
        if (a->worldEpoch != 0 && a->worldEpoch != epoch_) { ++stats_.staleEpochAcks; if (!mutations().noEpochBump) return; }
        uint64_t before = c->acked;
        c->acked = std::max(c->acked, std::min(a->upTo, log_.head()));
        if (c->acked > before) {   // ack-clocked catch-up: progress means the path works, send the next batch right away
            c->retxBackoff = 0;
            if (c->acked < log_.head()) c->sendNow = true;
        }
    } else if (auto* dg = std::get_if<SyncDigest>(&m)) {
        if (dg->session != c->session || (dg->epoch != 0 && dg->epoch != epoch_) || dg->shards.size() != kSyncShards) return;
        ++stats_.digestsReceived;
        if (!c->digestRequested && c->lastDigestAt != 0 && now_ < c->lastDigestAt + dopt_.minDigestIntervalMs) { ++stats_.digestsRateLimited; return; }
        c->digestRequested = false;
        c->lastDigestAt = now_ ? now_ : 1;
        auto st = std::make_shared<ShadowState>();
        if (!log_.stateAt(dg->seq, *st)) { ++stats_.digestsStale; return; }
        // RefScriptVar (object / NPC script vars, polled by the host) is not digest state: the client's world dump (quests, globals, watched
        // refs) has no way to produce it, so each such key was "missing" on the client and resent every round without ever converging
        // (two-copy combat_proof 2026-10-22: 99 digests, 0 ok, window-script timers 171CDB / 1771EA / 1771EB repaired every 10 s). The values
        // still reach the clients through the live stream and the join baseline; only the compare leaves them out.
        if (!mutations().digestKeepsRefVars) st->refVars.clear();
        // Weather and time scale are the same kind of key: the host shadow holds the last SetWeather / SetTimeScale, the client's world dump cannot read
        // either one, so the client reported them "missing" and the host resent them every round (two-copy npc_sync / combat_proof / enemy_fight
        // 2026-10-22: SetWeather 1237D7 and SetTimeScale 30, 'client (missing)', a new mismatch in every step). The live stream carries both.
        if (!mutations().digestKeepsWorldEnv) { st->weatherSet = false; st->weatherForced = false; st->weather = FormRef(); st->timeScale = 0; }
        // Map markers likewise: the host shadow holds every committed SetMapMarker, the client's world dump has no marker reader, so the client reported
        // each one "missing" and the host resent it every round (Goodsprings story run 2026-10-28: SetMapMarker 157F7D / 157F7E / 159AC5 applied on the
        // client again and again, the only digest divergence left). The reveal itself reaches the clients through the live stream and the join baseline.
        if (!mutations().digestKeepsMapMarkers) st->mapMarkers.clear();
        if (!mutations().digestKeepsNotes) st->notes.clear();   // the client dump has no note reader either (VMQ01CourierNote from Doc, story run 2026-10-28)
        Client* cp = c;
        const uint32_t sess = c->session, dgId = dg->digestId, ep = epoch_;
        const uint64_t dseq = dg->seq;
        // Acknowledged divergences count as agreeing: the host sums the client's term for them instead of its own.
        std::vector<ShardAdjust> adj;
        if (!mutations().ackIgnored) for (auto& [k, a] : c->acks) adj.push_back({a.shard, a.clientTerm, a.hostTerm});
        const size_t cursor = c->askCursor;
        const size_t askMax = c->askShards ? std::min(c->askShards, dopt_.maxShardsPerRound) : dopt_.maxShardsPerRound;
        runAsync<std::pair<std::vector<size_t>, size_t>>(worker_, mq_,
            [st, theirs = dg->shards, cls = dopt_.compareClasses, maxShards = askMax, adj = std::move(adj), cursor]() {
                std::vector<uint32_t> hh = shardHashesOf(*st, adj.empty() ? nullptr : &adj);
                std::vector<size_t> bad;
                for (size_t i = 0; i < kSyncShards; ++i)
                    if ((cls >> (i / 16) & 1u) && hh[i] != theirs[i]) bad.push_back(i);
                const size_t badTotal = bad.size();
                // Round-robin: start at the first bad shard at or after the cursor, so shards that never settle cannot starve the others
                // (two-PC benchmark 2026-10-27: the same quest shards were asked first every round while 25 other shards waited).
                if (!mutations().askLowestFirst)
                    std::rotate(bad.begin(), std::lower_bound(bad.begin(), bad.end(), cursor), bad.end());
                if (bad.size() > maxShards) bad.resize(maxShards);
                // Ask for no more shards than the client's reply can hold (kMaxShardEntries, judged by our own counts): a truncated reply
                // drops entries in no shard order, so the real drift could sit in the part cut off and never be repaired, while every
                // key cut off looks missing and is resent each round (two-copy fight test 2026-10-21: 24 digests, 1244 effects each).
                // The shards left out are still bad at the next digest and are asked for then. At least one shard is always asked for.
                if (!mutations().overfullShardRequest && bad.size() > 1) {
                    std::vector<size_t> perShard(kSyncShards, 0);
                    for (auto& e : baselineEntries(*st)) ++perShard[shardOf(e)];
                    size_t sum = 0, keep = 0;
                    while (keep < bad.size() && (keep == 0 || sum + perShard[bad[keep]] <= kMaxShardEntries)) sum += perShard[bad[keep++]];
                    bad.resize(keep);
                }
                return std::make_pair(std::move(bad), badTotal);
            },
            [this, cp, st, sess, dgId, ep, dseq](std::pair<std::vector<size_t>, size_t> res) {
                std::vector<size_t> bad = std::move(res.first);
                if (!cp->joined || cp->session != sess || epoch_ != ep) return;
                if (bad.empty()) { ++stats_.digestsOk; return; }
                ++stats_.digestsMismatch;
                cp->askCursor = (bad.back() + 1) % kSyncShards;
                cp->sync = std::make_shared<Sync>();
                cp->sync->id = dgId; cp->sync->seq = dseq; cp->sync->at = now_; cp->sync->state = st; cp->sync->shards = bad;
                cp->sync->badTotal = res.second;
                ShardRequest rq;
                rq.session = sess; rq.digestId = dgId;
                for (size_t s : bad) (s < 64 ? rq.maskLo : rq.maskHi) |= 1ull << (s % 64);
                sendTo(cp->peer, rq);
            });
    } else if (auto* se = std::get_if<ShardEntries>(&m)) {
        if (!c->sync || se->session != c->session || se->digestId != c->sync->id) return;
        std::shared_ptr<Sync> sy = std::move(c->sync);
        c->sync.reset();
        // The host cannot know how many entries a client holds per shard (its own world is nearly empty). A reply cut off at kMaxShardEntries
        // holds only the first entries in class order, usually ones already settled, so the rest is never seen: ask for half as many shards
        // next time, and again for more once a reply fits.
        if (!mutations().askNeverShrinks) {
            if (se->truncated) c->askShards = std::max<size_t>(1, sy->shards.size() / 2);
            else if (c->askShards && c->askShards < dopt_.maxShardsPerRound) c->askShards = std::min(dopt_.maxShardsPerRound, c->askShards * 2);
        }
        // keys changed by txns after the digest are in flight, not drifted
        std::set<uint64_t> touched;
        for (uint64_t s = sy->seq + 1; s <= log_.head(); ++s)
            if (const EffectTxn* t = log_.get(s))
                for (auto& e : t->effects) { BaselineEntry be; if (entryOf(e, be)) touched.insert(be.key); }
        Client* cp = c;
        const uint32_t sess = c->session;
        const bool truncated = se->truncated;
        runAsync<std::vector<RepairItem>>(worker_, mq_,
            [sy, clientEffects = se->entries, touched = std::move(touched), truncated = se->truncated, known = c->acks]() {
                std::set<size_t> shardSet(sy->shards.begin(), sy->shards.end());
                std::map<uint64_t, BaselineEntry> hostE, clientE;
                for (auto& e : baselineEntries(*sy->state)) if (shardSet.count(shardOf(e))) hostE.emplace(e.key, e);
                for (auto& e : clientEffects) { BaselineEntry be; if (entryOf(e, be) && shardSet.count(shardOf(be))) clientE.emplace(be.key, std::move(be)); }
                std::vector<RepairItem> items;
                // An acknowledgement still holds while both sides show the values it was made for.
                auto isAcked = [&](const RepairItem& r) {
                    auto k = known.find(r.key);
                    return k != known.end() && k->second.hostHas == r.hostHas && (!r.hostHas || k->second.hostVal == r.hostVal) &&
                           r.clientHas && k->second.clientVal == r.clientVal;
                };
                for (auto& [key, he] : hostE) {
                    auto it = clientE.find(key);
                    if (it != clientE.end() && it->second.value == he.value) continue;
                    if (it == clientE.end() && truncated && !mutations().overfullShardRequest) continue;   // cut off, not missing
                    RepairItem r;
                    r.key = key; r.fix = he.effect;
                    r.revertToo = it != clientE.end() && (std::holds_alternative<RefAddItem>(he.effect) || std::holds_alternative<RefRemoveItem>(he.effect));
                    r.d.key = describeEntry(he.effect); r.d.keyHash = key; r.d.host = r.d.key;
                    r.d.client = it == clientE.end() ? "(missing)" : describeEntry(it->second.effect);
                    r.race = touched.count(key) > 0;
                    r.hostHas = true; r.hostVal = he.value; r.cls = he.cls;
                    r.clientHas = it != clientE.end(); r.clientVal = r.clientHas ? it->second.value : 0;
                    r.acked = isAcked(r);
                    items.push_back(std::move(r));
                }
                for (auto& [key, ce] : clientE) {
                    if (hostE.count(key)) continue;
                    RepairItem r;
                    r.key = key; r.clientOnly = true;
                    r.clientHas = true; r.clientVal = ce.value; r.cls = ce.cls;
                    r.questRunning = std::holds_alternative<StartQuest>(ce.effect) || std::holds_alternative<StopQuest>(ce.effect);
                    if (r.questRunning) r.fix = ce.effect;
                    if (auto* ss = std::get_if<SetStage>(&ce.effect); ss && ss->stage == 0) { r.stageZero = true; r.fix = ce.effect; }
                    else if (auto* ss2 = std::get_if<SetStage>(&ce.effect); ss2 && ss2->stage > 0) { r.questState = 1; r.fix = ce.effect; }
                    else if (std::holds_alternative<CompleteQuest>(ce.effect)) { r.questState = 2; r.fix = ce.effect; }
                    else if (std::holds_alternative<FailQuest>(ce.effect)) { r.questState = 3; r.fix = ce.effect; }
                    else if (auto* so = std::get_if<SetObjective>(&ce.effect)) {
                        if (so->flags == 0) r.objectiveZero = true; else r.objectiveState = 1;
                        r.fix = ce.effect;
                    }
                    else if (std::holds_alternative<CompleteAllObjectives>(ce.effect)) { r.objectiveState = 2; r.fix = ce.effect; }
                    else if (std::holds_alternative<SetScriptVar>(ce.effect)) { r.scriptVar = true; r.fix = ce.effect; }
                    else if (std::holds_alternative<SetGlobal>(ce.effect)) { r.global = true; r.fix = ce.effect; }
                    r.noInverse = !mayHaveRevert(ce.effect);
                    r.d.key = describeEntry(ce.effect); r.d.keyHash = key; r.d.client = r.d.key; r.d.host = "(absent)"; r.d.clientOnly = true;
                    r.race = touched.count(key) > 0;
                    r.acked = isAcked(r);
                    items.push_back(std::move(r));
                }
                // Acknowledgements of the asked shards that no longer match a difference (the two sides agree now, or one value moved on) are withdrawn.
                for (auto& [key, k] : known) {
                    if (!shardSet.count(k.shard)) continue;
                    bool live = false;
                    for (auto& r : items) if (r.key == key && r.acked) { live = true; break; }
                    if (live) continue;
                    RepairItem r;
                    r.key = key; r.dropAck = true;
                    items.push_back(std::move(r));
                }
                return items;
            },
            [this, cp, sy, sess, truncated](std::vector<RepairItem> items) {
                if (!cp->joined || cp->session != sess) return;
                DesyncReport rep;
                rep.id = ++reportCounter_; rep.atMs = now_; rep.atSeq = sy->seq; rep.epoch = epoch_; rep.slot = cp->slot;
                for (size_t s : sy->shards) rep.shards.push_back(static_cast<uint32_t>(s));
                rep.incomplete = truncated;
                const Mutations& mu = mutations();
                // Acknowledgements whose difference is gone (the sides agree now, or a value moved on) are withdrawn; the rest of the bookkeeping
                // (repair attempts, unread rounds) of the asked shards forgets keys that no longer differ.
                std::set<uint64_t> present;
                {
                    std::vector<RepairItem> real;
                    for (auto& it : items) {
                        if (it.dropAck) { if (cp->acks.erase(it.key)) ++stats_.ackDropped; continue; }
                        present.insert(it.key);
                        real.push_back(std::move(it));
                    }
                    items = std::move(real);
                    std::set<size_t> asked(sy->shards.begin(), sy->shards.end());
                    for (auto it = cp->attempts.begin(); it != cp->attempts.end();)
                        it = (asked.count(it->second.shard) && !present.count(it->first)) ? cp->attempts.erase(it) : std::next(it);
                    for (auto it = cp->unread.begin(); it != cp->unread.end();)
                        it = (asked.count(it->second.shard) && !present.count(it->first)) ? cp->unread.erase(it) : std::next(it);
                }
                // newest txns touching each key: ONE pass over the newest part of the retained log (not one pass per entry)
                std::map<uint64_t, std::vector<uint64_t>> recent;
                {
                    uint64_t lo = log_.head() > 1024 ? log_.head() - 1024 : 1;
                    if (lo < log_.firstSeq()) lo = log_.firstSeq();
                    for (uint64_t s = log_.head(); s >= lo && s > 0; --s) {
                        const EffectTxn* t = log_.get(s);
                        if (!t) break;
                        for (auto& e : t->effects) {
                            BaselineEntry be;
                            if (!entryOf(e, be)) continue;
                            auto& v = recent[be.key];
                            if (v.size() < 3 && (v.empty() || v.back() != s)) v.push_back(s);
                        }
                    }
                }
                auto lastSeqs = [&](uint64_t key) { auto it = recent.find(key); return it == recent.end() ? std::vector<uint64_t>() : it->second; };
                std::vector<Effect> fix;
                std::vector<uint64_t> revert;
                std::vector<size_t> fixIdx, revertIdx;   // the item behind each planned fix / revert (repair attempts are counted per key)
                std::vector<size_t> adoptRun;      // client-only quest running keys (issue #25): items
                std::vector<FormRef> adoptStage;   // client-only "SetStage q = 0" keys: not started on the client, simply absent on the host
                struct StateItem { uint8_t kind; Effect fx; FormRef quest; size_t idx; };
                std::vector<StateItem> stateItems;   // client-only real quest state, adopted only when the host engine confirms it
                std::vector<Effect> adoptObjZero;   // client-only SetObjective flags 0 entries: not displayed / completed on the client, simply absent on the host
                struct ObjItem { uint8_t kind; Effect fx; FormRef quest; size_t idx; };
                std::vector<ObjItem> objItems;   // client-only real objective state, adopted only when the host engine confirms it
                struct VarItem { SetScriptVar fx; size_t idx; };
                std::vector<VarItem> varItems;   // client-only script vars: adopted outright at 0 (the untouched default), else only when the host engine confirms the value
                struct GlobItem { SetGlobal fx; size_t idx; };
                std::vector<GlobItem> globItems; // client-only globals: adopted only when the host engine holds the same value
                // Old behaviour twins and the no-inverse guard share one switch: a revert key for an entry the client cannot undo is skipped by the client
                // and would be sent again every round. Production never sends one.
                const bool anyRevertTwin = mu.questRunningReverted || mu.stageZeroReverted || mu.questStateReverted || mu.objectiveZeroReverted ||
                                           mu.objectiveStateReverted || mu.scriptVarReverted || mu.noInverseResent || mu.globalReverted;
                std::set<size_t> newlyAcked;
                // A key the host can neither adopt nor repair becomes a known divergence: reported once, then it no longer breaks the digest compare
                // (the host sums the client's term for it, ShardAdjust) and is never sent again. Either side's value moving on withdraws it.
                auto ackItem = [&](size_t i) {
                    if (mu.ackIgnored || mu.healDisabled || cp->acks.size() >= kMaxAcks) return false;
                    const RepairItem& it = items[i];
                    Client::Known a;
                    a.shard = static_cast<size_t>(it.cls) * 16 + static_cast<size_t>(it.key % 16);
                    a.hostHas = it.hostHas; a.clientHas = it.clientHas; a.hostVal = it.hostVal; a.clientVal = it.clientVal;
                    a.hostTerm = it.hostHas ? shardTerm(it.key, it.hostVal) : 0;
                    a.clientTerm = it.clientHas ? shardTerm(it.key, it.clientVal) : 0;
                    if (cp->acks.emplace(it.key, a).second) ++stats_.divergenceAcked;
                    cp->attempts.erase(it.key); cp->unread.erase(it.key);
                    newlyAcked.insert(i);
                    return true;
                };
                // The host engine could not read the key this round: left alone, acknowledged when it stays unreadable.
                auto noteUnread = [&](size_t i) {
                    auto& u = cp->unread[items[i].key];
                    u.shard = static_cast<size_t>(items[i].cls) * 16 + static_cast<size_t>(items[i].key % 16);
                    if (++u.rounds >= kUnreadRounds) ackItem(i);
                };
                // No way to settle the key at all (no reader, a failing reader, or a kind with no inverse): counted, never sent, acknowledged.
                auto unsettleable = [&](size_t i) {
                    if (anyRevertTwin) { revert.push_back(items[i].key); return; }
                    ++stats_.revertsSkippedNoInverse;
                    ackItem(i);
                };
                size_t n = 0;
                for (size_t i = 0; i < items.size(); ++i) {
                    RepairItem& it = items[i];
                    if (n++ < 64) it.d.lastSeqs = lastSeqs(it.key);
                    if (it.race) it.d.skippedRace = true;
                    else if (it.acked && !mu.ackIgnored) it.d.acknowledged = true;   // a known divergence: listed, nothing sent
                    else if (it.clientOnly && it.questRunning && !mu.questRunningReverted) adoptRun.push_back(i);
                    else if (it.clientOnly && it.stageZero && !mu.stageZeroReverted)
                        adoptStage.push_back(std::get<SetStage>(it.fix).quest);
                    else if (it.clientOnly && it.questState && hostQuestState && !mu.questStateReverted) {
                        FormRef q;
                        if (auto* s1 = std::get_if<SetStage>(&it.fix)) q = s1->quest;
                        else if (auto* s2 = std::get_if<CompleteQuest>(&it.fix)) q = s2->quest;
                        else q = std::get<FailQuest>(it.fix).quest;
                        stateItems.push_back({it.questState, it.fix, q, i});
                    }
                    else if (it.clientOnly && it.objectiveZero && !mu.objectiveZeroReverted)
                        adoptObjZero.push_back(it.fix);
                    else if (it.clientOnly && it.objectiveState && hostObjectives && !mu.objectiveStateReverted) {
                        FormRef q = it.objectiveState == 1 ? std::get<SetObjective>(it.fix).quest : std::get<CompleteAllObjectives>(it.fix).quest;
                        objItems.push_back({it.objectiveState, it.fix, q, i});
                    }
                    else if (it.clientOnly && it.scriptVar && !mu.scriptVarReverted)
                        varItems.push_back({std::get<SetScriptVar>(it.fix), i});
                    else if (it.clientOnly && it.global && !mu.globalReverted)
                        globItems.push_back({std::get<SetGlobal>(it.fix), i});
                    else if (it.clientOnly && it.noInverse && !anyRevertTwin) unsettleable(i);   // reported, never sent: the client has no inverse
                    else if (it.clientOnly) { revert.push_back(it.key); revertIdx.push_back(i); }
                    else { fix.push_back(it.fix); fixIdx.push_back(i); if (it.revertToo) { revert.push_back(it.key); revertIdx.push_back(i); } }
                    rep.entries.push_back(std::move(it.d));
                }
                rep.totalEntries = rep.entries.size();
                // Issue #25: a quest's running flag has no inverse, so a revert key for it was skipped by the client and resent every round
                // (the digest never converged). The host's shared world simply has no entry for these quests: settle them host-authoritatively
                // by committing the host's OWN running state (the engine via hostQuestRunning). A normal txn: every client gets it, late joiners too,
                // and the host's key space is complete afterwards. A quest the engine cannot read is left alone (acknowledged when it stays unreadable).
                // Only with no reader at all the completeness default (a quest the host does not list is not running) is committed; the game plugin
                // always has one (2026-10-27 benchmark: the baseline-less host had none, so it committed "not running" for ~190 quests the client was
                // running and the client stopped them).
                size_t adopted = 0;
                if (!adoptRun.empty() && !mu.healDisabled) {
                    std::vector<FormRef> qs;
                    for (size_t i : adoptRun)
                        qs.push_back(std::holds_alternative<StartQuest>(items[i].fix) ? std::get<StartQuest>(items[i].fix).quest : std::get<StopQuest>(items[i].fix).quest);
                    std::map<FormRef, bool> running;
                    const bool engineRead = hostQuestRunning && !mu.runningBlindStop && hostQuestRunning(qs, running);
                    std::vector<Effect> fx;
                    for (size_t k = 0; k < adoptRun.size(); ++k) {
                        auto r = running.find(qs[k]);
                        if (r == running.end() && engineRead) { ++stats_.runningUnread; noteUnread(adoptRun[k]); continue; }   // the engine could not read this one
                        if (r != running.end() && r->second) fx.push_back(StartQuest{qs[k]}); else fx.push_back(StopQuest{qs[k]});
                    }
                    const size_t nFx = fx.size();
                    Cause cause; cause.kind = CauseKind::Script;
                    if (nFx && commit(kHostPlayer, cause, std::move(fx))) { adopted = nFx; stats_.questRunningAdopted += nFx; }
                }
                // A stage-0 key only the client has: SetStage merges by max, so a revert key can never remove it and every digest round resent
                // the same ~200 repair effects (the host never reached ok, and the repair traffic starved its frame budget). Commit the same
                // stage 0 on the host: its shared world then has the key too, the digests agree, and every client and late joiner gets it.
                if (!adoptStage.empty() && !mu.healDisabled) {
                    std::vector<Effect> fx;
                    for (auto& q : adoptStage) fx.push_back(SetStage{q, 0});
                    const size_t nFx = fx.size();
                    Cause cause; cause.kind = CauseKind::Script;
                    if (nFx && commit(kHostPlayer, cause, std::move(fx))) { adopted += nFx; stats_.stageZeroAdopted += nFx; }
                }
                // An objective that is not displayed and not completed (flags 0) only the client has: the objective analogue of stage 0. SetObjective
                // from a guest only adds bits and there is no inverse, so a revert key was skipped by the client and resent every round. Both copies
                // loaded the same save, the host's shared world just never recorded the objective: commit the same flags 0 on the host.
                if (!adoptObjZero.empty() && !mu.healDisabled) {
                    const size_t nFx = adoptObjZero.size();
                    Cause cause; cause.kind = CauseKind::Script;
                    if (commit(kHostPlayer, cause, std::move(adoptObjZero))) { adopted += nFx; stats_.objectiveZeroAdopted += nFx; }
                }
                // Client-only objective with non-zero flags / all objectives completed: same rule as the quest state below, the host's OWN engine
                // must show the same flags (every objective of the quest completed, for CompleteAllObjectives). A reader that fails adopts nothing;
                // a quest it cannot read is left alone; a disagreement is a real divergence, acknowledged (reported once, never resent).
                if (!objItems.empty() && !mu.healDisabled) {
                    std::vector<FormRef> qs;
                    for (auto& oi : objItems) qs.push_back(oi.quest);
                    std::sort(qs.begin(), qs.end());
                    qs.erase(std::unique(qs.begin(), qs.end()), qs.end());
                    std::map<FormRef, std::map<uint32_t, uint8_t>> st;
                    if (!hostObjectives(qs, st)) {
                        for (auto& oi : objItems) noteUnread(oi.idx);
                    } else {
                        std::vector<Effect> fx;
                        for (auto& oi : objItems) {
                            auto e = st.find(oi.quest);
                            if (e == st.end()) { noteUnread(oi.idx); continue; }   // the engine could not read this quest: left alone this round
                            bool agree = false;
                            if (oi.kind == 1) {
                                auto& so = std::get<SetObjective>(oi.fx);
                                auto o = e->second.find(so.objective);
                                if (o == e->second.end()) { noteUnread(oi.idx); continue; }   // the host engine has no such objective: unknown, left alone
                                agree = (o->second & 3) == (so.flags & 3);
                            } else {
                                if (e->second.empty()) { noteUnread(oi.idx); continue; }
                                agree = true;
                                for (auto& [id, fl] : e->second) if (!(fl & 2)) agree = false;
                            }
                            if (agree) fx.push_back(oi.fx); else { ++stats_.objectiveStateDisagreed; ackItem(oi.idx); }
                        }
                        const size_t nFx = fx.size();
                        Cause cause; cause.kind = CauseKind::Script;
                        if (nFx && commit(kHostPlayer, cause, std::move(fx))) { adopted += nFx; stats_.objectiveStateAdopted += nFx; }
                    }
                }
                // Client-only real quest state (the host runs without a world baseline, so its shared world never recorded what both copies
                // loaded from the same save): settle it host-authoritatively, but only where the host's OWN engine confirms the value. A key
                // the engine disagrees with is a real divergence: it is not adopted, it is acknowledged (listed once, no longer a digest mismatch).
                // A reader that fails adopts nothing; quests it cannot read are left alone this round.
                if (!stateItems.empty() && !mu.healDisabled) {
                    std::vector<FormRef> qs;
                    for (auto& si : stateItems) qs.push_back(si.quest);
                    std::sort(qs.begin(), qs.end());
                    qs.erase(std::unique(qs.begin(), qs.end()), qs.end());
                    std::map<FormRef, HostQuestState> st;
                    if (!hostQuestState(qs, st)) {
                        for (auto& si : stateItems) noteUnread(si.idx);
                    } else {
                        std::vector<Effect> fx;
                        for (auto& si : stateItems) {
                            auto e = st.find(si.quest);
                            if (e == st.end() || (si.kind == 1 && e->second.stage < 0)) { noteUnread(si.idx); continue; }
                            bool agree = false;
                            if (si.kind == 1) agree = e->second.stage >= static_cast<int32_t>(std::get<SetStage>(si.fx).stage);
                            else if (si.kind == 2) agree = e->second.completed && !e->second.failed;
                            else agree = e->second.failed;
                            if (agree) fx.push_back(si.fx); else { ++stats_.questStateDisagreed; ackItem(si.idx); }
                        }
                        const size_t nFx = fx.size();
                        Cause cause; cause.kind = CauseKind::Script;
                        if (nFx && commit(kHostPlayer, cause, std::move(fx))) { adopted += nFx; stats_.questStateAdopted += nFx; }
                    }
                }
                // Client-only script vars (quest vars, and the vars of watched refs): the baseline-less host primes its poll with the engine's present
                // values and drops them, so its shared world only has vars that CHANGED since the session started, while the client's dump (the digest)
                // lists every var of every quest. SetScriptVar has no inverse, so the old revert keys were skipped by the client and resent every round
                // (256 per round: 1180 repair effects per minute and a digest mismatch in every phase, even idle, of the 2026-10-26 benchmark).
                // An untouched default (value 0) is the absent key's own meaning (completeness rule): the host commits the same 0, like stage 0.
                // A non-zero value is real save state: adopted only where the host's OWN engine reads the same value (HostSession::hostScriptVars).
                // A different engine value is a real divergence: not adopted, not resent, acknowledged (scriptVarDisagreed).
                if (!varItems.empty() && !mu.healDisabled) {
                    std::vector<std::pair<FormRef, uint16_t>> want;
                    for (auto& vi : varItems) want.emplace_back(vi.fx.owner, vi.fx.index);
                    std::sort(want.begin(), want.end());
                    want.erase(std::unique(want.begin(), want.end()), want.end());
                    std::map<std::pair<FormRef, uint16_t>, double> engine;
                    const bool engineRead = hostScriptVars && hostScriptVars(want, engine);
                    std::vector<Effect> fx;
                    for (auto& vi : varItems) {
                        auto e = engine.find({vi.fx.owner, vi.fx.index});
                        const bool zero = vi.fx.value == 0.0;
                        if (e != engine.end()) {
                            if (sameScriptVarValue(vi.fx.owner, vi.fx.index, e->second, vi.fx.value)) fx.push_back(vi.fx);
                            else { ++stats_.scriptVarDisagreed; ackItem(vi.idx); }
                        } else if (zero) {
                            fx.push_back(vi.fx);   // the engine shows nothing for it: the default
                        } else if (!engineRead) {
                            unsettleable(vi.idx);  // no reader: nothing non-zero is adopted blindly
                        } else {
                            noteUnread(vi.idx);    // reader ran but could not read this owner: left alone this round
                        }
                    }
                    const size_t nFx = fx.size();
                    Cause cause; cause.kind = CauseKind::Script;
                    if (nFx && commit(kHostPlayer, cause, std::move(fx))) { adopted += nFx; stats_.scriptVarAdopted += nFx; }
                } else if (!varItems.empty()) {
                    for (auto& vi : varItems) unsettleable(vi.idx);
                }
                // Client-only globals: the same story as script vars. The baseline-less host only records globals that changed since the session started.
                // Adopted only where the host's own engine holds the same value; a different value is a real divergence (acknowledged).
                if (!globItems.empty() && !mu.healDisabled) {
                    std::vector<FormRef> want;
                    for (auto& gi : globItems) want.push_back(gi.fx.global);
                    std::sort(want.begin(), want.end());
                    want.erase(std::unique(want.begin(), want.end()), want.end());
                    std::map<FormRef, double> engine;
                    const bool engineRead = hostGlobals && hostGlobals(want, engine);
                    std::vector<Effect> fx;
                    for (auto& gi : globItems) {
                        auto e = engine.find(gi.fx.global);
                        if (e != engine.end()) {
                            if (sameGlobalValue(gi.fx.global, e->second, double(gi.fx.value))) fx.push_back(gi.fx);
                            else { ++stats_.globalDisagreed; ackItem(gi.idx); }
                        } else if (!engineRead) {
                            unsettleable(gi.idx);
                        } else {
                            noteUnread(gi.idx);
                        }
                    }
                    const size_t nFx = fx.size();
                    Cause cause; cause.kind = CauseKind::Script;
                    if (nFx && commit(kHostPlayer, cause, std::move(fx))) { adopted += nFx; stats_.globalAdopted += nFx; }
                } else if (!globItems.empty()) {
                    for (auto& gi : globItems) unsettleable(gi.idx);
                }
                // bandwidth budget
                if (repairTokens_ < 0) { repairTokens_ = double(dopt_.maxRepairEffectsPerMin); repairRefillAt_ = now_; }
                repairTokens_ = std::min(double(dopt_.maxRepairEffectsPerMin), repairTokens_ + double(dopt_.maxRepairEffectsPerMin) * double(now_ - repairRefillAt_) / 60000.0);
                repairRefillAt_ = now_;
                size_t allowed = std::min<size_t>({fix.size() + revert.size(), dopt_.maxRepairEffects, size_t(repairTokens_)});
                if (allowed < fix.size() + revert.size()) { ++stats_.repairsDeferredBudget; rep.incomplete = true; }
                if (fix.size() > allowed) { fix.resize(allowed); fixIdx.resize(allowed); }
                size_t revBudget = allowed - fix.size();
                if (revert.size() > revBudget) {
                    revert.resize(revBudget);
                    if (revertIdx.size() > revBudget) revertIdx.resize(revBudget);
                }
                // A key repaired kMaxRepairAttempts times with the same two values and still different is not going to converge by being sent again
                // (a per-machine value the live stream rewrites, a state the client engine refuses): acknowledged instead of resent forever.
                if (!mu.repairForever && !mu.healDisabled) {
                    std::set<size_t> gaveUp, counted;
                    for (size_t pass = 0; pass < 2; ++pass) {
                        for (size_t i : (pass == 0 ? fixIdx : revertIdx)) {
                            if (counted.count(i)) continue;
                            counted.insert(i);
                            const RepairItem& it = items[i];
                            auto& a = cp->attempts[it.key];
                            const bool same = a.sent && a.hostHas == it.hostHas && a.clientHas == it.clientHas && a.hostVal == it.hostVal && a.clientVal == it.clientVal;
                            if (same && a.sent >= kMaxRepairAttempts) { gaveUp.insert(i); continue; }
                            if (!same) { a = Client::Attempt(); a.hostHas = it.hostHas; a.clientHas = it.clientHas; a.hostVal = it.hostVal; a.clientVal = it.clientVal; }
                            a.shard = static_cast<size_t>(it.cls) * 16 + static_cast<size_t>(it.key % 16);
                            ++a.sent;
                        }
                    }
                    std::set<uint64_t> dropKeys;
                    for (size_t i : gaveUp) if (ackItem(i)) { ++stats_.repairGaveUp; dropKeys.insert(items[i].key); }
                    if (!dropKeys.empty()) {
                        std::vector<Effect> f2;
                        std::vector<uint64_t> r2;
                        for (size_t k = 0; k < fix.size(); ++k) if (!dropKeys.count(items[fixIdx[k]].key)) f2.push_back(std::move(fix[k]));
                        for (size_t k = 0; k < revert.size(); ++k) if (!dropKeys.count(revert[k])) r2.push_back(revert[k]);
                        fix = std::move(f2);
                        revert = std::move(r2);
                    }
                }
                if (!mu.healDisabled && (!fix.empty() || !revert.empty())) {
                    repairTokens_ -= double(fix.size() + revert.size());
                    SyncRepair rp;
                    rp.session = sess; rp.epoch = epoch_; rp.digestId = sy->id; rp.effects = std::move(fix); rp.revertKeys = std::move(revert);
                    rep.effectsSent = rp.effects.size() + rp.revertKeys.size();
                    rep.repaired = true;
                    ++stats_.desyncRepairs;
                    stats_.desyncEffectsRepaired += rep.effectsSent;
                    sendTo(cp->peer, rp);
                }
                if (adopted) {
                    // Settling is the host recording, once, what both copies loaded from the same save (see the Adopted counters): bookkeeping, not repair.
                    ++stats_.settleRounds;
                    stats_.effectsSettled += adopted;
                    if (mu.settleCountsAsRepair) stats_.desyncEffectsRepaired += adopted;
                    rep.repaired = true;
                    rep.effectsSent += adopted;
                }
                for (size_t i : newlyAcked) if (i < rep.entries.size()) rep.entries[i].acknowledged = true;
                if (rep.entries.size() > 64) rep.entries.resize(64);
                reports_.push_back(std::move(rep));
                while (reports_.size() > dopt_.reportLimit) reports_.pop_front();
                // The settle committed keys; the shards still unasked are probably the same story: ask for the next digest now instead of in 10 s.
                if (adopted && (sy->badTotal > sy->shards.size() || truncated) && !mu.noSettleFollowUp && !mu.healDisabled) { ++stats_.settleFollowUps; requestDigest(cp->slot); }
            });
    } else if (auto* sc = std::get_if<BaselineSummaryChunk>(&m)) {
        if (!c->job || sc->session != c->session || sc->id != c->job->id || sc->total > 4096) return;
        Job& j = *c->job;
        if (!j.built) {
            if (j.summaryTotal && sc->total != j.summaryTotal) return;
            j.summaryTotal = sc->total;
            j.got.emplace(sc->index, *sc);
        }
        uint32_t next = 0;
        while (j.got.count(next)) ++next;
        sendTo(c->peer, BaselineAck{c->session, j.id, 0, j.built ? j.summaryTotal : next});
        if (!j.built && !j.computing && j.summaryTotal && next == j.summaryTotal) buildJobDiff(*c);   // async; sends the first window when ready
    } else if (auto* ba = std::get_if<BaselineAck>(&m)) {
        if (!c->job || ba->session != c->session || ba->id != c->job->id || ba->phase != 1) return;
        Job& j = *c->job;
        if (!j.built) return;
        uint32_t before = j.acked;
        j.acked = std::max(j.acked, std::min<uint32_t>(ba->next, static_cast<uint32_t>(j.chunks.size())));
        if (j.acked > before) j.backoff = 0;
        if (j.acked >= j.chunks.size()) j.done = true; else sendJobWindow(*c, false);
    } else if (auto* ap = std::get_if<AppData>(&m)) {
        if (ap->session == c->session && onApp) onApp(c->slot, ap->channel, ap->payload);
    } else if (auto* in = std::get_if<Intent>(&m)) {
        if (in->session != c->session || in->epoch < c->intentEpoch) return;
        if (in->epoch > c->intentEpoch) { c->intentEpoch = in->epoch; c->lastIntent = 0; }
        bool dup = in->id <= c->lastIntent;
        if (dup && !mutations().noIdempotency) {
            ++stats_.intentsDuplicate;
        } else if (in->id > c->lastIntent + 1) {
            return;  // gap: client sends strictly in order, wait for the retransmit
        } else if (in->kind != IntentKind::AppliedEffects) {
            c->lastIntent = std::max(c->lastIntent, in->id);
            ++stats_.actionsExecuted;
            if (onAction) onAction(c->slot, in->kind, in->ref, in->epoch, in->id);
        } else {
            Cause cause = in->cause;
            if (isQuestRewardCause(cause.kind)) cause.kind = CauseKind::Intent;  // clients cannot claim host-run scripts
            commit(c->slot, cause, in->effects, kTxnOriginApplied, in->epoch, in->id);
            c->lastIntent = std::max(c->lastIntent, in->id);
            ++stats_.intentsCommitted;
        }
        sendTo(c->peer, IntentAck{c->session, in->epoch, in->id});
    }
}

void HostSession::sendApp(PlayerId toSlot, uint16_t channel, const Bytes& payload) {
    for (auto& [tok, c] : clients_)
        if (c.joined && (toSlot == kNoPlayer || c.slot == toSlot)) sendTo(c.peer, AppData{c.session, channel, kHostPlayer, payload});
}

void HostSession::relayApp(PlayerId fromSlot, uint16_t channel, const Bytes& payload) {
    for (auto& [tok, c] : clients_)
        if (c.joined && c.slot != fromSlot) sendTo(c.peer, AppData{c.session, channel, fromSlot, payload});
}

void HostSession::requestDigest(PlayerId slot) {
    for (auto& [tok, c] : clients_)
        if (c.joined && (slot == kNoPlayer || c.slot == slot)) { c.digestRequested = true; sendTo(c.peer, DigestRequest{c.session, epoch_}); }
}

std::string HostSession::desyncsJson() const {
    auto esc = [](const std::string& s) {
        std::string o;
        for (char ch : s) {
            if (ch == '"' || ch == '\\') { o.push_back('\\'); o.push_back(ch); }
            else if (static_cast<unsigned char>(ch) < 0x20) o += ' ';
            else o.push_back(ch);
        }
        return o;
    };
    std::ostringstream o;
    o << "{\"digests\":{\"received\":" << stats_.digestsReceived << ",\"ok\":" << stats_.digestsOk << ",\"mismatch\":" << stats_.digestsMismatch
      << ",\"rateLimited\":" << stats_.digestsRateLimited << ",\"stale\":" << stats_.digestsStale << ",\"repairs\":" << stats_.desyncRepairs
      << ",\"effectsRepaired\":" << stats_.desyncEffectsRepaired << ",\"deferredByBudget\":" << stats_.repairsDeferredBudget
      << ",\"questRunningAdopted\":" << stats_.questRunningAdopted << ",\"stageZeroAdopted\":" << stats_.stageZeroAdopted
      << ",\"objectiveZeroAdopted\":" << stats_.objectiveZeroAdopted
      << ",\"questStateAdopted\":" << stats_.questStateAdopted << ",\"questStateDisagreed\":" << stats_.questStateDisagreed
      << ",\"objectiveStateAdopted\":" << stats_.objectiveStateAdopted << ",\"objectiveStateDisagreed\":" << stats_.objectiveStateDisagreed
      << ",\"scriptVarAdopted\":" << stats_.scriptVarAdopted << ",\"scriptVarDisagreed\":" << stats_.scriptVarDisagreed
      << ",\"revertsSkippedNoInverse\":" << stats_.revertsSkippedNoInverse
      << ",\"effectsSettled\":" << stats_.effectsSettled << ",\"settleRounds\":" << stats_.settleRounds << ",\"settleFollowUps\":" << stats_.settleFollowUps
      << ",\"globalAdopted\":" << stats_.globalAdopted << ",\"globalDisagreed\":" << stats_.globalDisagreed << ",\"runningUnread\":" << stats_.runningUnread
      << ",\"divergenceAcked\":" << stats_.divergenceAcked << ",\"ackDropped\":" << stats_.ackDropped << ",\"repairGaveUp\":" << stats_.repairGaveUp
      << "},\"desyncs\":[";
    bool firstR = true;
    for (auto& r : reports_) {
        if (!firstR) o << ',';
        firstR = false;
        o << "{\"id\":" << r.id << ",\"atMs\":" << r.atMs << ",\"atSeq\":" << r.atSeq << ",\"epoch\":" << r.epoch << ",\"slot\":" << int(r.slot)
          << ",\"repaired\":" << (r.repaired ? "true" : "false") << ",\"incomplete\":" << (r.incomplete ? "true" : "false")
          << ",\"effectsSent\":" << r.effectsSent << ",\"total\":" << r.totalEntries << ",\"entries\":[";
        bool firstE = true;
        for (auto& e : r.entries) {
            if (!firstE) o << ',';
            firstE = false;
            o << "{\"key\":\"" << esc(e.key) << "\",\"host\":\"" << esc(e.host) << "\",\"client\":\"" << esc(e.client) << "\",\"clientOnly\":"
              << (e.clientOnly ? "true" : "false") << ",\"race\":" << (e.skippedRace ? "true" : "false") << ",\"acked\":" << (e.acknowledged ? "true" : "false") << ",\"lastSeqs\":[";
            for (size_t i = 0; i < e.lastSeqs.size(); ++i) o << (i ? "," : "") << e.lastSeqs[i];
            o << "]}";
        }
        o << "]}";
    }
    o << "]}";
    return o.str();
}

std::string HostSession::explainJoinEntry(PlayerId slot, const Effect& probe) const {
    for (auto& [tok, c] : clients_)
        if (c.slot == slot && c.job) return "host snapshot H (job " + std::to_string(c.job->id) + "): " + explainEntry(c.job->snapAtH, probe);
    return "host head (no open job): " + explainEntry(log_.headState(), probe);
}

void HostSession::peerDisconnected(PeerId peer) {
    std::vector<PlayerId> left;
    for (auto& [tok, c] : clients_)
        if (c.peer == peer) { c.joined = false; c.job.reset(); c.peer = kNoPeer; if (c.slot < kMaxSlots) left.push_back(c.slot); }
    if (mutations().disconnectKeepsLock) return;
    // A cinematic / scripted lock must never outlive the connection of the locked player: release their control bits so the rejoin
    // (or the party) is not left with a frozen character.
    for (PlayerId s : left) releaseControlsOf(s);
}

void HostSession::releaseControlsOf(PlayerId s) {
    if (mutations().disconnectKeepsLock || s >= kMaxSlots) return;
    const uint32_t off = log_.headState().ledger[s].controlsOff;
    if (!off) return;
    Cause c; c.kind = CauseKind::Script;
    ++stats_.controlReleases;
    commit(kHostPlayer, c, {SetPlayerControls{off, true, Recipient::slot(s)}});
}

void HostSession::bumpEpochAndRebase() {
    if (mutations().noEpochBump) return;   // TEST ONLY: stale traffic is no longer fenced off
    ++epoch_;
    for (auto& [tok, c] : clients_) {
        c.joined = false;
        c.job.reset();
        c.acked = 0;
        c.rebasePending = c.peer != kNoPeer && c.session != 0;
        c.lastRebaseSend = now_;
        if (c.rebasePending) { ++stats_.rebasesSent; sendTo(c.peer, Rebase{c.session, epoch_}); }
    }
}

bool HostSession::reloadWorld(const ShadowState& baseline, uint64_t contentHash) {
    ShadowState b = baseline;
    for (auto& k : baseKarma_) k = b.ledger[0].karma;
    for (auto& r : baseRep_) r = b.ledger[0].rep;
    b.ledger[0] = Ledger();
    b.seq = 0;
    for (auto& g : grants_) g.clear();
    log_ = StoryLog();
    log_.setInitialState(b);
    contentHash_ = contentHash;
    haveBaseline_ = true;
    ++stats_.reloads;
    bumpEpochAndRebase();
    return true;
}

namespace {
constexpr uint32_t kSidecarMagic = 0x43534346;  // 'FCSC'
constexpr uint8_t kSidecarVersion = 4;
}

Bytes HostSession::saveSidecar() const {
    Archive a = Archive::writer();
    uint32_t magic = kSidecarMagic; a.u32(magic);
    uint8_t ver = kSidecarVersion; a.u8(ver);
    uint64_t sid = sessionId_; a.u64(sid);
    uint32_t ep = epoch_; a.varu32(ep);
    SharePolicy p = policy_; io(a, p);
    uint64_t ch = contentHash_; a.u64(ch);
    bool hb = haveBaseline_; a.boolean(hb);
    for (unsigned s = 0; s < kMaxSlots; ++s) {
        int64_t k = baseKarma_[s]; a.vari(k);
        size_t n = baseRep_[s].size();
        a.count(n, 1u << 20);
        for (auto& [f, v] : baseRep_[s]) { FormRef ff = f; int64_t x = v.first, y = v.second; io(a, ff); a.vari(x); a.vari(y); }
    }
    size_t nc = clients_.size();
    a.count(nc, kMaxSlots);
    for (auto& [tok, c] : clients_) { uint64_t t = tok; uint8_t sl = c.slot; a.u64(t); a.u8(sl); }
    ShadowState head = log_.headState();
    io(a, head);
    return a.finish();
}

bool HostSession::loadSidecar(const Bytes& bytes) {
    Archive a = Archive::reader(bytes.data(), bytes.size());
    uint32_t magic = 0; a.u32(magic);
    uint8_t ver = 0; a.u8(ver);
    if (!a.ok() || magic != kSidecarMagic || ver != kSidecarVersion) return false;
    uint64_t sid = 0; a.u64(sid);
    uint32_t ep = 0; a.varu32(ep);
    SharePolicy p; io(a, p);
    uint64_t ch = 0; a.u64(ch);
    bool hb = false; a.boolean(hb);
    int64_t bk[kMaxSlots] = {};
    std::map<FormRef, std::pair<int64_t, int64_t>> br[kMaxSlots];
    for (unsigned s = 0; s < kMaxSlots && a.ok(); ++s) {
        a.vari(bk[s]);
        size_t n = 0;
        if (!a.count(n, 1u << 20)) return false;
        for (size_t i = 0; i < n && a.ok(); ++i) { FormRef f; int64_t x = 0, y = 0; io(a, f); a.vari(x); a.vari(y); if (a.ok()) br[s][f] = {x, y}; }
    }
    size_t nc = 0;
    if (!a.count(nc, kMaxSlots)) return false;
    std::vector<std::pair<uint64_t, uint8_t>> toks;
    for (size_t i = 0; i < nc && a.ok(); ++i) { uint64_t t = 0; uint8_t sl = 0; a.u64(t); a.u8(sl); if (sl >= kMaxSlots) return false; toks.emplace_back(t, sl); }
    ShadowState head;
    io(a, head);
    if (!a.ok() || !a.atEnd() || !hb) return false;

    // commit: fold each slot's session deltas into its absolute karma/rep base, then restart the history in a new epoch
    policy_ = p;
    sessionId_ = sid;
    contentHash_ = ch;
    for (unsigned s = 0; s < kMaxSlots; ++s) {
        baseKarma_[s] = bk[s] + head.ledger[s].karma;
        baseRep_[s] = br[s];
        for (auto& [f, v] : head.ledger[s].rep) { baseRep_[s][f].first += v.first; baseRep_[s][f].second += v.second; }
        head.ledger[s] = Ledger();
    }
    head.seq = 0;
    for (auto& g : grants_) g.clear();
    log_ = StoryLog();
    log_.setInitialState(head);
    haveBaseline_ = true;
    epoch_ = std::max(epoch_, ep);
    for (auto& [tok, slot] : toks) {
        if (clients_.count(tok)) continue;
        bool taken = false;
        for (auto& [t2, c2] : clients_) taken |= c2.slot == slot;
        if (taken || slot == kHostPlayer) continue;
        Client c;
        c.slot = slot;
        clients_.emplace(tok, c);
    }
    ++stats_.reloads;
    bumpEpochAndRebase();
    return true;
}

void HostSession::buildJobDiff(Client& c) {
    std::shared_ptr<Job> job = c.job;
    Job& j = *job;
    j.computing = true;
    BaselineSummary s;
    s.contentHash = contentHash_;
    for (uint32_t i = 0; i < j.summaryTotal; ++i) {
        const auto& ch = j.got.at(i);
        s.entries.insert(s.entries.end(), ch.entries.begin(), ch.entries.end());
        if (ch.hasAbs) { s.karma = ch.karma; s.rep = ch.rep; }
    }
    // Absolute karma/reputation for THIS joiner = the session-start value + the deltas its slot received up to H.
    const Ledger& led = j.snapAtH.ledger[c.slot];
    int64_t hostKarma = baseKarma_[c.slot] + led.karma;
    std::map<FormRef, std::pair<int64_t, int64_t>> rep = baseRep_[c.slot];
    for (auto& [f, v] : led.rep) { rep[f].first += v.first; rep[f].second += v.second; }
    std::vector<RepAbs> hostRep;
    for (auto& [f, v] : rep) hostRep.push_back({f, v.first, v.second});
    struct Plan { std::vector<BaselineDiffChunk> chunks; size_t effects = 0, reverts = 0, kept = 0; };
    Client* cp = &c;
    runAsync<Plan>(worker_, mq_,
        [job, s = std::move(s), pol = policy_, slot = c.slot, session = c.session, hostKarma, hostRep = std::move(hostRep)]() mutable {
            BaselineDiff d = diffBaseline(job->snapAtH, s, pol, slot, pol.karma, hostKarma, pol.reputation, hostRep);
            size_t nE = (d.effects.size() + kBaselineEffectsPerChunk - 1) / kBaselineEffectsPerChunk;
            size_t nR = (d.revertKeys.size() + kBaselineRevertsPerChunk - 1) / kBaselineRevertsPerChunk;
            size_t total = std::max<size_t>({1, nE, nR});
            Plan p;
            for (size_t i = 0; i < total; ++i) {
                BaselineDiffChunk ch;
                ch.session = session; ch.id = job->id; ch.index = static_cast<uint32_t>(i); ch.total = static_cast<uint32_t>(total);
                ch.kept = static_cast<uint32_t>(std::min<size_t>(d.keptByClient, 0xFFFFFFFFu));
                for (size_t k = i * kBaselineEffectsPerChunk; k < std::min(d.effects.size(), (i + 1) * kBaselineEffectsPerChunk); ++k)
                    ch.effects.push_back(d.effects[k]);
                for (size_t k = i * kBaselineRevertsPerChunk; k < std::min(d.revertKeys.size(), (i + 1) * kBaselineRevertsPerChunk); ++k)
                    ch.revertKeys.push_back(d.revertKeys[k]);
                p.chunks.push_back(std::move(ch));
            }
            p.effects = d.effects.size(); p.reverts = d.revertKeys.size(); p.kept = d.keptByClient;
            return p;
        },
        [this, cp, job](Plan p) {
            job->computing = false;
            if (cp->job != job) return;   // the client rejoined meanwhile: a newer job owns the transfer
            job->chunks = std::move(p.chunks);
            job->built = true;
            job->acked = 0; job->sentUpTo = 0; job->backoff = 0;
            job->got.clear();
            stats_.baselineEffectsSent += p.effects;
            stats_.baselineEntriesKept += p.kept;
            stats_.baselineReverts += p.reverts;
            if (cp->joined) sendJobWindow(*cp, false);
        });
}

void HostSession::sendJobWindow(Client& c, bool retransmit) {
    Job& j = *c.job;
    const bool chatty = mutations().chattyBaseline;
    if (retransmit) { j.sentUpTo = j.acked; if (j.backoff < 3) ++j.backoff; }
    // Only chunks not yet in flight (an ack must not re-send the whole window), and only while the pipe has room.
    size_t start = chatty ? j.acked : std::max<size_t>(j.acked, j.sentUpTo);
    size_t end = std::min<size_t>(j.chunks.size(), size_t(j.acked) + 8);
    for (size_t i = start; i < end; ++i) {
        if (!chatty && !bulkOk(c)) { ++stats_.bulkDeferred; break; }
        sendBulk(c, j.chunks[i]);
        j.sentUpTo = static_cast<uint32_t>(i + 1);
        ++stats_.baselineChunksSent;
        if (retransmit) ++stats_.baselineRetransmits;
    }
    j.lastSend = now_;
}

void HostSession::tick(uint64_t nowMs) {
    now_ = nowMs;
    mq_->drain();
    PeerId from;
    Bytes data;
    while (t_.poll(from, data)) {
        Message m;
        if (decodeMessage(data, m) != DecodeStatus::Ok) { ++stats_.badMessages; continue; }
        handle(from, m);
    }
    for (auto& [tok, c] : clients_)
        if (c.sync && now_ > c.sync->at + 5000) c.sync.reset();
    for (auto& [tok, c] : clients_) {
        if (c.rebasePending && !c.joined && c.peer != kNoPeer && now_ >= c.lastRebaseSend + opt_.rtoMs) {
            c.lastRebaseSend = now_;
            ++stats_.rebasesSent;
            sendTo(c.peer, Rebase{c.session, epoch_});
        }
    }
    for (auto& [tok, c] : clients_) {
        if (c.joined && c.job && c.job->built && !c.job->done && now_ >= c.job->lastSend + (uint64_t(opt_.rtoMs) << c.job->backoff)) sendJobWindow(c, true);
    }
    for (auto& [tok, c] : clients_) {
        if (!c.joined || c.acked >= log_.head()) { c.retxBackoff = 0; continue; }
        if (!c.sendNow && now_ < c.lastRetx + (uint64_t(opt_.rtoMs) << c.retxBackoff)) continue;
        if (!log_.canReplayFrom(c.acked)) {
            sendSnapshotWelcome(c);  // the txns this client still needs were compacted away
            continue;
        }
        TxnBatch b;
        b.session = c.session;
        b.worldEpoch = epoch_;
        b.txns = log_.range(c.acked, opt_.batch);
        c.lastSend = now_;
        c.lastRetx = now_;
        if (c.acked == c.retxAcked && !c.sendNow) { if (c.retxBackoff < 3) ++c.retxBackoff; } else c.retxBackoff = 0;   // no progress since the last batch: back off
        c.retxAcked = c.acked;
        c.sendNow = false;
        ++stats_.retransmitBatches;
        sendTo(c.peer, b);
    }
}

std::vector<HostSession::ClientView> HostSession::clients() const {
    std::vector<ClientView> v;
    for (auto& [tok, c] : clients_) v.push_back({tok, c.slot, c.peer, c.joined, c.acked, c.job && !c.job->done});
    return v;
}

bool HostSession::allSynced() const {
    for (auto& [tok, c] : clients_) if (c.joined && c.acked < log_.head()) return false;
    return true;
}

namespace {
bool isAdditive(const Effect& e) {
    return std::holds_alternative<AddItem>(e) || std::holds_alternative<RemoveItem>(e) ||
           std::holds_alternative<ModActorValue>(e) || std::holds_alternative<ModReputation>(e) ||
           std::holds_alternative<ModKarma>(e) || std::holds_alternative<RefAddItem>(e) || std::holds_alternative<RefRemoveItem>(e);
}

// A local state-like effect may have been overtaken by later host txns this client never saw (it is joining
// from a snapshot); put the key back to whatever the snapshot says.
void reassertState(const Effect& e, const ShadowState& snap, std::vector<Effect>& out) {
    std::visit([&](const auto& x) {
        using T = std::decay_t<decltype(x)>;
        if constexpr (std::is_same_v<T, SetStage>) {
            auto it = snap.stages.find(x.quest); if (it != snap.stages.end()) out.push_back(SetStage{x.quest, it->second});
        } else if constexpr (std::is_same_v<T, SetObjective>) {
            auto it = snap.objectives.find({x.quest, x.objective});
            if (it != snap.objectives.end()) out.push_back(SetObjective{x.quest, x.objective, it->second});
        } else if constexpr (std::is_same_v<T, SetGlobal>) {
            auto it = snap.globals.find(x.global); if (it != snap.globals.end()) out.push_back(SetGlobal{x.global, it->second});
        } else if constexpr (std::is_same_v<T, EnableRef> || std::is_same_v<T, DisableRef>) {
            auto it = snap.enabled.find(x.ref);
            if (it != snap.enabled.end()) { if (it->second) out.push_back(EnableRef{x.ref}); else out.push_back(DisableRef{x.ref}); }
        } else if constexpr (std::is_same_v<T, StartQuest> || std::is_same_v<T, StopQuest>) {
            auto it = snap.questRunning.find(x.quest);
            if (it != snap.questRunning.end()) { if (it->second) out.push_back(StartQuest{x.quest}); else out.push_back(StopQuest{x.quest}); }
        } else if constexpr (std::is_same_v<T, MoveRef>) {
            auto it = snap.placed.find(x.ref);
            if (it != snap.placed.end()) out.push_back(MoveRef{x.ref, it->second.cell, it->second.x, it->second.y, it->second.z});
        } else if constexpr (std::is_same_v<T, KillActor> || std::is_same_v<T, ResurrectActor>) {
            if (snap.kills.count(x.actor)) out.push_back(KillActor{x.actor}); else out.push_back(ResurrectActor{x.actor});
        }
    }, e);
}
}  // namespace

// ================================ ClientSession ================================================

ClientSession::ClientSession(ITransport& t, PeerId host, uint64_t token, IEffectSink& sink, EffectCapture* capture,
                             Options opt)
    : t_(t), host_(host), token_(token), sink_(sink), capture_(capture), opt_(opt) {
    if (capture_) capture_->setGuest(true);
}

void ClientSession::send(const Message& m) {
    Bytes b = encodeMessage(m);
    if (!b.empty()) t_.send(host_, std::move(b));
}

void ClientSession::sendApp(uint16_t channel, const Bytes& payload) {
    if (joined_) send(AppData{session_, channel, slot_, payload});
}

void ClientSession::sendAck() {
    uint64_t up = shadow_.seq;
    if (baselineActive_) while (pending_.count(up + 1)) ++up;  // received (buffered) counts as acked
    send(Ack{session_, up, worldEpoch_});
}

void ClientSession::sendSummaryWindow(bool retransmit) {
    lastSummarySend_ = now_;
    const bool chatty = mutations().chattyBaseline;
    if (retransmit) { summarySent_ = summaryNext_; if (summaryBackoff_ < 3) ++summaryBackoff_; }
    size_t start = chatty ? summaryNext_ : std::max<size_t>(summaryNext_, summarySent_);
    size_t end = std::min<size_t>(summaryChunks_.size(), size_t(summaryNext_) + 8);
    for (size_t i = start; i < end; ++i) {
        if (!chatty && t_.backlogBytes(host_) > (1u << 20)) break;   // do not queue behind what is already waiting
        send(summaryChunks_[i]);
        summarySent_ = static_cast<uint32_t>(i + 1);
    }
}

void ClientSession::applyRepairEffect(const Effect& e) {
    BaselineEntry be;
    if (entryOf(e, be) && touchedSince_.count(be.key)) { ++syncStats_.repairEffectsSkipped; return; }   // a newer txn / local action owns this key now
    EchoGuard g(capture_);
    IEffectSink::Context ctx;
    ctx.baseline = true;
    ctx.repair = true;
    ctx.fromSnapshot = true;
    ctx.catchUp = true;
    sink_.apply(e, ctx);
    ++syncStats_.effectsRepaired;
}

void ClientSession::sendDigest() {
    lastDigest_ = now_ ? now_ : 1;
    digestWanted_ = false;
    ShadowState w;
    uint64_t h = 0;
    if (!provider_ || !provider_(w, h)) return;
    if (!mutations().digestKeepsRefVars) w.refVars.clear();   // not digest state, see the host side of the compare (SyncDigest handler)
    if (!mutations().digestKeepsWorldEnv) { w.weatherSet = false; w.weatherForced = false; w.weather = FormRef(); w.timeScale = 0; }   // same
    if (!mutations().digestKeepsMapMarkers) w.mapMarkers.clear();   // same
    if (!mutations().digestKeepsNotes) w.notes.clear();             // same
    auto world = std::make_shared<const ShadowState>(std::move(w));
    digestWorld_ = world;
    digestSeq_ = shadow_.seq;
    ++digestId_;
    touchedSince_.clear();
    digestOutstanding_ = true;
    digestInFlight_ = true;
    const uint64_t gen = connGen_, seq = digestSeq_;
    const uint32_t id = digestId_;
    runAsync<std::vector<uint32_t>>(worker_, mq_,
        [world]() { return shardHashesOf(*world); },
        [this, gen, seq, id](std::vector<uint32_t> shards) {
            digestInFlight_ = false;
            if (gen != connGen_ || id != digestId_ || !digestOutstanding_) return;
            SyncDigest d;
            d.session = session_; d.epoch = worldEpoch_; d.seq = seq; d.digestId = id; d.shards = std::move(shards);
            ++syncStats_.digestsSent;
            send(d);
        });
}

void ClientSession::applyBaselineEffect(const Effect& e) {
    if (mutations().baselineSkipped) return;
    EchoGuard g(capture_);
    IEffectSink::Context ctx;
    ctx.baseline = true;
    ctx.fromSnapshot = true;
    ctx.catchUp = true;
    sink_.apply(e, ctx);
    ++baselineResult_.effectsApplied;
}

void ClientSession::applyReadyDiffChunks() {
    for (;;) {
        auto it = diffGot_.find(diffNext_);
        if (it == diffGot_.end()) break;
        // Collected until the last chunk: reverts must all run before the host's effects (a revert is the inverse of what the client's own
        // world holds; a host effect such as RemoveAllItems resets state a later revert would then subtract from).
        for (auto& e : it->second.effects) diffEffects_.push_back(e);
        for (uint64_t key : it->second.revertKeys) diffReverts_.push_back(key);
        diffGot_.erase(it);
        ++diffNext_;
        ++baselineResult_.chunks;
    }
    send(BaselineAck{session_, baselineId_, 1, diffNext_});
    if (diffTotal_ != 0 && diffNext_ >= diffTotal_) {
        for (uint64_t key : diffReverts_) {
            Effect rev;
            if (clientWorld_ && revertEffectFor(*clientWorld_, key, rev)) { applyBaselineEffect(rev); ++baselineResult_.reverted; }
            else ++baselineResult_.unrevertable;
        }
        for (auto& e : diffEffects_) applyBaselineEffect(e);
        diffReverts_.clear(); diffEffects_.clear();
        finishBaseline();
    }
}

void ClientSession::finishBaseline() {
    // Now the session's own deltas (reward items etc.). Karma/reputation the baseline already synced absolutely are skipped.
    for (auto& e : heldDiff_) {
        if ((baselineAbs_ & 1) && std::holds_alternative<ModKarma>(e)) continue;
        if ((baselineAbs_ & 2) && std::holds_alternative<ModReputation>(e)) continue;
        if (mutations().baselineSkipped && !isAdditive(e)) continue;
        EchoGuard g(capture_);
        IEffectSink::Context ctx;
        ctx.fromSnapshot = true;
        ctx.catchUp = true;
        sink_.apply(e, ctx);
    }
    heldDiff_.clear();
    for (uint64_t id : heldRewardIds_) delivered_.insert(id);
    heldRewardIds_.clear();
    baselineActive_ = false;
    baselineDone_ = true;
    clientWorld_.reset();
    summaryChunks_.clear();
    drain();
    sendAck();
    sink_.onBaselineComplete(baselineResult_);
}

void ClientSession::connect(uint64_t nowMs, uint32_t session) {
    session_ = session;
    active_ = true;
    joined_ = false;
    rejected_ = false;
    pending_.clear();
    // A (re)started baseline attempt must describe the world as it is NOW: an earlier, interrupted attempt may already have
    // changed it, and a stale summary would hide those changes from the host.
    if (wantBaseline_ && !baselineDone_ && provider_) {
        ShadowState w;
        uint64_t h = 0;
        if (provider_(w, h)) setBaselineWorld(w, h);
    }
    if (epoch_ == 0) epoch_ = session;
    now_ = nowMs;
    lastHello_ = nowMs;
    helloRetries_ = 0;
    baselineActive_ = false;
    ++connGen_;
    digestOutstanding_ = false; digestWanted_ = false; touchedSince_.clear();
    lastDigest_ = nowMs;
    diffGot_.clear(); diffEffects_.clear(); diffReverts_.clear(); diffNext_ = diffTotal_ = 0; summaryNext_ = 0;
    helloCursor_ = (!(wantBaseline_ && !baselineDone_) && worldEpoch_ != 0) ? cursorHashOf(shadow_) : 0;
    send(makeHello());
}

Hello ClientSession::makeHello() const {
    Hello h{session_, token_, shadow_.seq};
    h.lastEpoch = worldEpoch_;
    h.hs = hs_;
    if (wantBaseline_ && !baselineDone_) { h.flags = kHelloWantBaseline; h.contentHash = contentHash_; }
    else h.cursorCheck = helloCursor_;
    return h;
}

void ClientSession::doRebase() {
    ++stats_.rebases;
    resetSession();   // forgets shadow, queued intents, baseline progress (the world itself is whatever the engine has)
    ShadowState w;
    uint64_t h = 0;
    if (provider_ && provider_(w, h)) {
        setBaselineWorld(w, h);
        connect(now_, session_ + 1);
    } else {
        rebaseRequired_ = true;
    }
}

namespace {
constexpr uint32_t kClientStateMagic = 0x53434346;  // 'FCCS'
}

Bytes ClientSession::saveState() const {
    Archive a = Archive::writer();
    uint32_t magic = kClientStateMagic; a.u32(magic);
    uint8_t ver = 3; a.u8(ver);
    bool clean = joined_ && intents_.empty() && !baselineActive_ && pending_.empty();
    a.boolean(clean);
    uint8_t sl = slot_; a.u8(sl);
    uint32_t we = worldEpoch_; a.varu32(we);
    // Which rewards this save's world already holds. Valid even when the rest of the sidecar is not (unclean / other epoch):
    // the game save contains them regardless, so a baseline rejoin must not deliver them again.
    size_t nd = delivered_.size(); a.count(nd, kMaxRewardsPerWelcome * 4);
    for (uint64_t id : delivered_) { uint64_t x = id; a.varu(x); }
    ShadowState s = shadow_;
    io(a, s);
    return a.finish();
}

bool ClientSession::loadStateBytes(const Bytes& b) {
    Archive a = Archive::reader(b.data(), b.size());
    uint32_t magic = 0; a.u32(magic);
    uint8_t ver = 0; a.u8(ver);
    bool clean = false; a.boolean(clean);
    uint8_t sl = 0; a.u8(sl);
    uint32_t we = 0; a.varu32(we);
    std::set<uint64_t> ids;
    size_t nd = 0;
    if (a.count(nd, kMaxRewardsPerWelcome * 4))
        for (size_t i = 0; i < nd && a.ok(); ++i) { uint64_t id = 0; a.varu(id); if (a.ok()) ids.insert(id); }
    ShadowState s;
    io(a, s);
    if (!a.ok() || !a.atEnd() || magic != kClientStateMagic || ver != 3) return false;
    delivered_ = std::move(ids);   // the save's reward record counts even if the shadow below is not resumable
    if (!clean || sl >= kMaxSlots || we == 0) return false;
    if (mutations().sidecarLosesLedger) { for (auto& l : s.ledger) l = Ledger(); delivered_.clear(); }
    shadow_ = s;
    slot_ = sl;
    worldEpoch_ = we;
    return true;
}

bool ClientSession::onLocalSaveLoaded(uint64_t nowMs, const Bytes* sidecar) {
    if (!clientLoadAllowed()) return false;
    now_ = nowMs;
    resetSession();
    delivered_.clear();   // the loaded save's record replaces the running one (no sidecar: nothing is known to be delivered)
    bool restored = sidecar && loadStateBytes(*sidecar);
    if (!restored) {
        ShadowState w;
        uint64_t h = 0;
        if (provider_ && provider_(w, h)) setBaselineWorld(w, h);
        else { rebaseRequired_ = true; return true; }
    }
    connect(nowMs, session_ + 1);
    return true;
}

void ClientSession::setBaselineWorld(const ShadowState& world, uint64_t contentHash) {
    auto w = std::make_shared<const ShadowState>(world);
    clientWorld_ = w;
    contentHash_ = contentHash;
    wantBaseline_ = true;
    baselineDone_ = false;
    baselineActive_ = false;
    summaryChunks_.clear();
    summaryReady_ = false;
    const uint64_t gen = ++sumGen_;
    runAsync<std::vector<BaselineSummaryChunk>>(worker_, mq_,
        [w, contentHash]() {
            BaselineSummary s = summarizeWorld(*w, contentHash);
            std::vector<BaselineSummaryChunk> chunks;
            size_t total = std::max<size_t>(1, (s.entries.size() + kBaselineSummaryPerChunk - 1) / kBaselineSummaryPerChunk);
            for (size_t i = 0; i < total; ++i) {
                BaselineSummaryChunk ch;
                ch.index = static_cast<uint32_t>(i); ch.total = static_cast<uint32_t>(total);
                for (size_t k = i * kBaselineSummaryPerChunk; k < std::min(s.entries.size(), (i + 1) * kBaselineSummaryPerChunk); ++k)
                    ch.entries.push_back(s.entries[k]);
                if (i == 0) { ch.hasAbs = true; ch.karma = s.karma; ch.rep = s.rep; }
                chunks.push_back(std::move(ch));
            }
            return chunks;
        },
        [this, gen](std::vector<BaselineSummaryChunk> chunks) {
            if (gen != sumGen_) return;   // a newer dump replaced this one (or the session was reset)
            summaryChunks_ = std::move(chunks);
            summaryReady_ = true;
            if (baselineActive_) {        // the Welcome arrived before the summary was ready
                stampSummary();
                summarySent_ = 0; summaryBackoff_ = 0;
                sendSummaryWindow(false);
            }
        });
}

std::string ClientSession::explainSummaryEntry(const Effect& probe) const {
    if (!clientWorld_) return "client: no world dump set";
    return "client summary world: " + explainEntry(*clientWorld_, probe);
}

void ClientSession::stampSummary() {
    for (auto& ch : summaryChunks_) { ch.session = session_; ch.id = baselineId_; }
}

void ClientSession::disconnect() {
    // The link is gone: a scripted control lock must not keep this player frozen (the host released it for us as well).
    if (joined_ && slot_ < kMaxSlots && !mutations().disconnectKeepsLock) {
        const uint32_t off = shadow_.ledger[slot_].controlsOff;
        if (off) {
            shadow_.ledger[slot_].controlsOff = 0;
            EchoGuard g(capture_);
            IEffectSink::Context ctx;
            ctx.epoch = worldEpoch_;
            sink_.apply(SetPlayerControls{off, true, Recipient::slot(slot_)}, ctx);
        }
    }
    active_ = false;
    joined_ = false;
    ++connGen_;
    digestInFlight_ = false;
}

void ClientSession::resetState() {
    resetSession();
    delivered_.clear();
}

void ClientSession::resetSession() {
    disconnect();
    shadow_ = ShadowState();
    pending_.clear();
    wantBaseline_ = baselineDone_ = baselineActive_ = false;
    rebaseRequired_ = false;
    worldEpoch_ = 0;
    clientWorld_.reset(); ++sumGen_; summaryReady_ = false; ++connGen_; digestInFlight_ = false;
    summaryChunks_.clear(); diffGot_.clear(); diffEffects_.clear(); diffReverts_.clear(); heldDiff_.clear(); heldRewardIds_.clear();
    intents_.clear();
    nextIntent_ = 1;
    seenIntent_ = 0;
    joinHead_ = 0;
    epoch_ = 0;
    slot_ = kNoPlayer;
}

uint32_t ClientSession::submitAction(IntentKind kind, const FormRef& ref) {
    if (epoch_ == 0 || kind == IntentKind::AppliedEffects) return 0;
    uint32_t id = nextIntent_++;
    PendingIntent pi{id, Cause(), {}, false, kind, ref};
    intents_.push_back(std::move(pi));
    if (joined_ && std::count_if(intents_.begin(), intents_.end(), [](const PendingIntent& p) { return !p.acked; }) == 1)
        sendHeadIntent();
    return id;
}

uint32_t ClientSession::submitIntent(const Cause& cause, std::vector<Effect> effects) {
    if (epoch_ == 0 || effects.empty()) return 0;
    uint32_t id = nextIntent_++;
    if (digestOutstanding_)
        for (auto& e : effects) { BaselineEntry be; if (entryOf(e, be)) touchedSince_.insert(be.key); }
    intents_.push_back({id, cause, std::move(effects)});
    if (joined_ && std::count_if(intents_.begin(), intents_.end(), [](const PendingIntent& p) { return !p.acked; }) == 1)
        sendHeadIntent();
    return id;
}

void ClientSession::tick(uint64_t nowMs) {
    now_ = nowMs;
    mq_->drain();
    PeerId from;
    Bytes data;
    while (t_.poll(from, data)) {
        if (!active_ || from != host_) continue;
        Message m;
        if (decodeMessage(data, m) != DecodeStatus::Ok) continue;
        handle(m);
    }
    if (!active_ || rejected_) return;
    if (!joined_) {
        // Hello retries back off (250, 500, 1000, 2000, 4000 ms): the host answers each with a Welcome that carries a snapshot.
        uint64_t gap = std::min<uint64_t>(uint64_t(opt_.helloIntervalMs) << std::min<uint32_t>(helloRetries_, 4), 4000);
        if (mutations().chattyBaseline) gap = opt_.helloIntervalMs;
        if (now_ >= lastHello_ + gap) {
            lastHello_ = now_;
            ++helloRetries_;
            send(makeHello());
        }
    } else if (!baselineActive_ && provider_ && intents_.empty() &&
               !digestInFlight_ && (digestWanted_ || (opt_.digestIntervalMs && now_ >= lastDigest_ + opt_.digestIntervalMs))) {
        sendDigest();
    } else if (baselineActive_) {
        if (summaryNext_ < summaryChunks_.size() && now_ >= lastSummarySend_ + (uint64_t(opt_.rtoMs) << summaryBackoff_)) sendSummaryWindow(true);
    } else if (now_ >= lastIntentSend_ + opt_.rtoMs) {
        sendHeadIntent();
    }
}

void ClientSession::handle(const Message& m) {
    if (auto* w = std::get_if<Welcome>(&m)) {
        if (w->session != session_) return;
        if (!joined_ && wantBaseline_ && !baselineDone_ && w->baselineId == 0) return;  // must join via the baseline transfer
        if (joined_ && baselineActive_) return;  // mid-transfer: the frozen snapshot is the reference, ignore newer ones
        if (joined_ && !(w->hasSnapshot && w->snapshot.seq > shadow_.seq)) { sendAck(); return; }  // already there: just confirm
        slot_ = w->slot;
        policy_ = w->policy;
        if (w->worldEpoch != 0) worldEpoch_ = w->worldEpoch;
        everJoined_ = true;
        joinHead_ = std::max(joinHead_, w->headSeq);
        if (w->hasSnapshot && w->baselineId != 0 && wantBaseline_ && !baselineDone_) {
            // World-baseline join: our world came from our own save. Hold the snapshot's session deltas (items etc.) until
            // the baseline has made the shared world match; live txns buffer meanwhile (deliver()).
            heldDiff_.clear();
            ShadowState::diffEffects(ShadowState(), w->snapshot, slot_, heldDiff_);
            // The baseline makes the shared world match the snapshot already; keep only what it does not carry (ledger deltas, barriers).
            if (!mutations().snapshotFloodAfterBaseline)
                heldDiff_.erase(std::remove_if(heldDiff_.begin(), heldDiff_.end(), [](const Effect& e) { return isBaselineCovered(e); }), heldDiff_.end());
            // Rewards: the snapshot's net ledger would hand over everything this slot ever received. Replace it by the rewards the save's
            // world does not hold yet (ids persisted in the sidecar).
            heldRewardIds_.clear();
            if (!mutations().noRewardIds) {
                heldDiff_.erase(std::remove_if(heldDiff_.begin(), heldDiff_.end(), [](const Effect& e) { return isRewardEffect(e); }), heldDiff_.end());
                for (auto& g : w->rewards) {
                    heldRewardIds_.push_back(g.id);
                    if (!delivered_.count(g.id)) heldDiff_.push_back(g.effect);
                }
            }
            shadow_ = w->snapshot;
            baselineActive_ = true;
            baselineId_ = w->baselineId;
            baselineAbs_ = w->baselineAbs;
            baselineResult_ = BaselineResult();
            diffGot_.clear(); diffEffects_.clear(); diffReverts_.clear(); diffNext_ = diffTotal_ = 0; summaryNext_ = 0;
            ++stats_.snapshots;
            summarySent_ = 0; summaryBackoff_ = 0;
            if (summaryReady_) { stampSummary(); sendSummaryWindow(false); }
        } else if (w->hasSnapshot) {
            // Intents this client applied locally that the host committed while we were away are already in the
            // world but not in our shadow: fold them in first so the diff does not apply them a second time.
            ShadowState base = shadow_;
            std::vector<Effect> reassert;  // state-like local effects to re-check against the snapshot
            if (w->intentEpoch == epoch_) {
                for (auto& pi : intents_) {
                    if (pi.id <= seenIntent_ || pi.id > w->lastIntent) continue;
                    EffectTxn pre;
                    pre.origin = slot_;
                    pre.cause = pi.cause;
                    // Only additive effects need accounting: state-like ones are idempotent and the diff re-asserts them.
                    for (auto& e : pi.effects) {
                        if (isAdditive(e)) pre.effects.push_back(e);
                        else reassert.push_back(e);
                    }
                    if (normalizeTxn(pre, policy_) == kNormalizeInvalid) continue;
                    base.applyTxn(pre);
                }
                seenIntent_ = std::max(seenIntent_, w->lastIntent);
                dropCommittedIntents();
            }
            std::vector<Effect> diff;
            ShadowState::diffEffects(base, w->snapshot, slot_, diff);
            // Rewards: after a restart whose sidecar was not resumable the shadow is empty, so the diff would hand over the whole
            // ledger. Deliver only the rewards the save's world does not hold yet (ids persisted in the sidecar), like the baseline path.
            // With an intact shadow the diff's net ledger delta is already right (and coalesced), so it is kept.
            if (!mutations().noRewardIds && shadow_.seq == 0 && !delivered_.empty()) {
                diff.erase(std::remove_if(diff.begin(), diff.end(), [](const Effect& e) { return isRewardEffect(e); }), diff.end());
                for (auto& g : w->rewards)
                    if (!delivered_.count(g.id)) diff.push_back(g.effect);
            }
            for (auto& e : reassert) reassertState(e, w->snapshot, diff);
            {
                EchoGuard g(capture_);
                IEffectSink::Context ctx;
                ctx.fromSnapshot = true;
                ctx.catchUp = true;
                for (auto& e : diff) sink_.apply(e, ctx);
            }
            shadow_ = w->snapshot;
            for (auto& g : w->rewards) delivered_.insert(g.id);   // the diff above made the world hold every reward up to the snapshot
            ++stats_.snapshots;
        } else {
            ++stats_.replays;
        }
        bool first = !joined_;
        joined_ = true;
        if (first) sink_.onJoined(slot_, policy_);
        drain();
        sendAck();
        if (first) sendHeadIntent();
    } else if (auto* ap = std::get_if<AppData>(&m)) {
        if (ap->session == session_ && joined_ && onApp) onApp(ap->from, ap->channel, ap->payload);
    } else if (auto* r = std::get_if<Reject>(&m)) {
        if (r->session == session_) { rejected_ = true; rejectReason_ = r->reason; rejectText_ = r->text; }
    } else if (auto* ba = std::get_if<BaselineAck>(&m)) {
        if (!baselineActive_ || ba->session != session_ || ba->id != baselineId_ || ba->phase != 0) return;
        uint32_t before = summaryNext_;
        summaryNext_ = std::max(summaryNext_, std::min<uint32_t>(ba->next, static_cast<uint32_t>(summaryChunks_.size())));
        if (summaryNext_ > before) summaryBackoff_ = 0;
        if (summaryNext_ < summaryChunks_.size()) sendSummaryWindow(false);
    } else if (auto* dc = std::get_if<BaselineDiffChunk>(&m)) {
        if (dc->session != session_ || dc->id != baselineId_) return;
        if (!baselineActive_) {  // already finished: the host missed our final ack, repeat it
            if (baselineDone_) send(BaselineAck{session_, baselineId_, 1, dc->total});
            return;
        }
        diffTotal_ = dc->total;
        baselineResult_.entriesKept = dc->kept;
        summaryNext_ = static_cast<uint32_t>(summaryChunks_.size());  // the host has the whole summary
        if (dc->index >= diffNext_) diffGot_.emplace(dc->index, *dc);
        applyReadyDiffChunks();
    } else if (auto* rb = std::get_if<Rebase>(&m)) {
        // A Rebase to our own epoch answers this session's Hello: the host refused the resume cursor of our save (another timeline).
        if (rb->session != session_ || (rb->epoch == worldEpoch_ && joined_)) return;
        doRebase();
    } else if (auto* dr = std::get_if<DigestRequest>(&m)) {
        if (dr->session == session_ && joined_) digestWanted_ = true;
    } else if (auto* rq = std::get_if<ShardRequest>(&m)) {
        if (rq->session != session_ || !digestOutstanding_ || rq->digestId != digestId_) return;
        ShardEntries se;
        se.session = session_; se.digestId = digestId_;
        if (!digestWorld_) return;
        for (auto& e : baselineEntries(*digestWorld_)) {
            size_t sh = shardOf(e);
            if (((sh < 64 ? rq->maskLo : rq->maskHi) >> (sh % 64)) & 1ull) {
                if (se.entries.size() >= kMaxShardEntries) { se.truncated = true; break; }
                se.entries.push_back(e.effect);
            }
        }
        send(se);
    } else if (auto* rp = std::get_if<SyncRepair>(&m)) {
        if (rp->session != session_ || !digestOutstanding_ || rp->digestId != digestId_ || !joined_) return;
        ++syncStats_.repairsApplied;
        for (uint64_t key : rp->revertKeys) {   // reverts first: they undo what the client holds, the host's effects then build on that
            if (touchedSince_.count(key)) { ++syncStats_.repairEffectsSkipped; continue; }
            Effect rev;
            if (digestWorld_ && revertEffectFor(*digestWorld_, key, rev)) applyRepairEffect(rev);
            else ++syncStats_.repairRevertsUnrevertable;   // no inverse for this kind: the key stays (was silent before issue #25)
        }
        for (auto& e : rp->effects) applyRepairEffect(e);
        digestOutstanding_ = false;
        touchedSince_.clear();
    } else if (auto* b = std::get_if<TxnBatch>(&m)) {
        if (b->session != session_ || !joined_) return;
        if (b->worldEpoch != 0 && worldEpoch_ != 0 && b->worldEpoch != worldEpoch_ && !mutations().noEpochBump) {
            ++stats_.staleEpochDropped;   // another world epoch: stale (older) is dropped, newer means we must rebase
            if (b->worldEpoch > worldEpoch_) doRebase();
            return;
        }
        for (auto& t : b->txns) deliver(t);
        sendAck();
    } else if (auto* ia = std::get_if<IntentAck>(&m)) {
        if (ia->session != session_ || ia->epoch != epoch_) return;
        // Committed at the host, but our shadow may not have seen the txn yet: keep it (acked) until it does,
        // so a snapshot join can still account for the already-applied local effects.
        for (auto& pi : intents_) if (pi.id == ia->id) pi.acked = true;
        // Requests have no local effects to account for: done once the host acknowledged them.
        intents_.erase(std::remove_if(intents_.begin(), intents_.end(),
                                      [&](const PendingIntent& p) { return p.id == ia->id && p.kind != IntentKind::AppliedEffects; }),
                       intents_.end());
        sendHeadIntent();
    }
}

void ClientSession::deliver(const EffectTxn& txn) {
    const Mutations& mut = mutations();
    if (mut.applyOutOfOrder) { applyOne(txn); return; }
    if (txn.seq <= shadow_.seq) {
        ++stats_.duplicatesDropped;
        if (!mut.noIdempotency) return;
        applyOne(txn);
        return;
    }
    if (baselineActive_) {  // hold live txns until the world baseline is in
        if (txn.seq - shadow_.seq <= opt_.window && pending_.emplace(txn.seq, txn).second) ++stats_.buffered;
        return;
    }
    if (txn.seq == shadow_.seq + 1) {
        applyOne(txn);
        drain();
    } else if (txn.seq - shadow_.seq <= opt_.window) {
        if (pending_.emplace(txn.seq, txn).second) ++stats_.buffered;
    }
}

void ClientSession::dropCommittedIntents() {
    size_t before = intents_.size();
    intents_.erase(std::remove_if(intents_.begin(), intents_.end(), [&](const PendingIntent& p) { return p.id <= seenIntent_; }),
                   intents_.end());
    if (intents_.size() != before && joined_) sendHeadIntent();
}

void ClientSession::sendHeadIntent() {
    for (auto& pi : intents_) {
        if (pi.acked) continue;
        send(Intent{session_, epoch_, pi.id, pi.cause, pi.effects, pi.kind, pi.ref});
        lastIntentSend_ = now_;
        return;
    }
}

void ClientSession::drain() {
    if (baselineActive_) return;
    for (;;) {
        while (!pending_.empty() && pending_.begin()->first <= shadow_.seq) pending_.erase(pending_.begin());
        auto it = pending_.find(shadow_.seq + 1);
        if (it == pending_.end()) return;
        EffectTxn t = std::move(it->second);
        pending_.erase(it);
        applyOne(t);
    }
}

// Rewards addressed to this player in a txn the shadow just folded in are in the world (applied now, or by the player's own action).
void ClientSession::noteDelivered(const EffectTxn& txn) {
    if (slot_ >= kMaxSlots) return;
    forEachReward(txn, worldEpoch_, [&](PlayerId slot, uint64_t id, const Effect&) { if (slot == slot_) delivered_.insert(id); });
}

void ClientSession::applyOne(const EffectTxn& txn) {
    uint64_t keep = shadow_.seq;
    std::vector<Effect> out;
    // A replay after a restart (plain rejoin, empty shadow) re-sends txns whose rewards the loaded save already holds (ids persisted
    // in the sidecar): the shadow folds them in, but the engine must not get them again.
    auto held = [&](size_t i) {
        return isRewardEffect(txn.effects[i]) && delivered_.count(rewardId(worldEpoch_, txn.seq, uint32_t(i))) != 0;
    };
    bool anyHeld = false;
    // (a dedupe as well: the noIdempotency twin turns it off too, so that twin still sees the double apply it injects)
    if (!mutations().noRewardIds && !mutations().noIdempotency && slot_ < kMaxSlots && !delivered_.empty())
        for (size_t i = 0; i < txn.effects.size() && !anyHeld; ++i) anyHeld = held(i);
    if (!anyHeld) {
        shadow_.applyTxn(txn, slot_, &out);
    } else {
        EffectTxn one;
        one.origin = txn.origin;
        one.seq = txn.seq;
        for (size_t i = 0; i < txn.effects.size(); ++i) {
            one.effects.assign(1, txn.effects[i]);
            shadow_.applyTxn(one, slot_, held(i) ? nullptr : &out);
        }
    }
    shadow_.seq = std::max(keep, txn.seq);
    ++stats_.txnsApplied;
    noteDelivered(txn);
    if (digestOutstanding_)
        for (auto& e : txn.effects) { BaselineEntry be; if (entryOf(e, be)) touchedSince_.insert(be.key); }
    if ((txn.flags & kTxnOriginApplied) && txn.origin == slot_ && txn.originEpoch == epoch_ && txn.originIntent > seenIntent_) {
        seenIntent_ = txn.originIntent;
        dropCommittedIntents();
    }
    // The origin already applied its own action locally, so additive effects (items, AV, karma, rep) must not
    // be applied again. State-like effects still flow: they are post-merge, idempotent, and correct the origin
    // when a conflicting host txn (e.g. a resurrect) was ordered between its action and the commit.
    const bool ownEcho = (txn.flags & kTxnOriginApplied) && txn.origin == slot_ && txn.originEpoch == epoch_;
    EchoGuard g(capture_);
    IEffectSink::Context ctx;
    ctx.seq = txn.seq;
    ctx.origin = txn.origin;
    ctx.cause = &txn.cause;
    ctx.catchUp = txn.seq <= joinHead_;
    ctx.epoch = worldEpoch_;
    for (auto& e : out) {
        if (ownEcho && isAdditive(e)) continue;
        sink_.apply(e, ctx);
        ++ctx.index;
    }
}

}  // namespace fnvcoop
