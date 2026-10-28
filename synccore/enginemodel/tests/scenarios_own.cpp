// Ownership scenarios without a network: the service and views driven by hand, with reordered, duplicated messages.
#include <algorithm>
#include <map>
#include <set>
#include <vector>
#include "scenarios.h"
#include "fnvcoop/rng.h"

namespace fnvcoop::testing {

namespace {
void note(std::string* why, const std::string& s) { if (why && why->size() < 600) *why += s + "; "; }
Presence P(PlayerId p, bool inCell, bool adj, float d) {
    Presence x; x.player = p; x.inCell = inCell; x.adjacent = adj; x.distance = d; return x;
}
}  // namespace

bool churnHolds(uint64_t seed, int steps, std::string* why) {
    Rng r(seed * 977 + 13);
    constexpr int kPlayers = 3, kActors = 6;
    OwnershipService svc;
    OwnershipView views[kPlayers] = {OwnershipView(0), OwnershipView(1), OwnershipView(2)};
    bool connected[kPlayers] = {true, true, true};
    for (int p = 0; p < kPlayers; ++p) svc.playerJoined(static_cast<PlayerId>(p));
    struct Pending { int to; OwnMsg m; int at; };
    std::vector<Pending> q;
    struct Ack { OwnMsg m; PlayerId from; int at; };
    std::vector<Ack> acks;
    std::map<std::pair<ActorNetId, OwnerEpoch>, PlayerId> history;
    std::map<ActorNetId, OwnerEpoch> lastEpoch;
    bool ok = true;
    int t = 0;

    auto emit = [&](const OwnershipService::Out& out) {
        for (const OwnMsg& m : out) {
            views[0].apply(m);   // the host's own view is applied at once
            if (m.kind == OwnMsg::Grant || m.kind == OwnMsg::Commit || m.kind == OwnMsg::Release) {
                auto ins = history.emplace(std::make_pair(m.netId, m.epoch), m.kind == OwnMsg::Release ? kNoPlayer : m.owner);
                if (!ins.second && ins.first->second != (m.kind == OwnMsg::Release ? kNoPlayer : m.owner)) {
                    ok = false; note(why, "two owners at one epoch for actor " + std::to_string(m.netId));
                }
            }
            for (int p = 1; p < kPlayers; ++p) {
                q.push_back({p, m, t + static_cast<int>(r.range(0, 4))});
                if (r.chance(0.2)) q.push_back({p, m, t + static_cast<int>(r.range(0, 9))});   // duplicate
            }
            if (m.kind == OwnMsg::Transfer && m.owner == 0) acks.push_back({m, 0, t});        // the host acks itself at once
        }
    };

    for (ActorNetId id = 1; id <= kActors; ++id) {
        OwnershipService::Out out;
        svc.track(id, {P(0, r.chance(0.5), true, 100.0f + 50 * id), P(1, true, true, 90.0f + 10 * id), P(2, false, true, 500)}, 0, out);
        emit(out);
    }
    for (; t < steps; ++t) {
        const int now = t * 100;
        // players come and go (never the host)
        if (r.chance(0.04)) {
            const int p = static_cast<int>(r.range(1, 2));
            OwnershipService::Out out;
            if (connected[p]) {
                connected[p] = false;
                auto orphans = svc.playerLeft(static_cast<PlayerId>(p), now, out);
                emit(out);
                (void)orphans;
            } else {
                connected[p] = true;
                views[p].clear();
                svc.playerJoined(static_cast<PlayerId>(p));
                OwnershipService::Out snap;
                svc.snapshot(snap);
                for (auto& m : snap) q.push_back({p, m, t + 1});
            }
        }
        OwnershipService::Out out;
        svc.tick(now, out);
        for (ActorNetId id = 1; id <= kActors; ++id) {
            std::vector<Presence> who;
            for (int p = 0; p < kPlayers; ++p) {
                if (!connected[p]) continue;
                Presence x = P(static_cast<PlayerId>(p), r.chance(0.4), r.chance(0.7), static_cast<float>(r.range(10, 3000)));
                who.push_back(x);
            }
            if (r.chance(0.02)) svc.setRecruiter(id, static_cast<PlayerId>(r.range(0, 2)), who, now, out);
            else svc.evaluate(id, who, now, out);
        }
        emit(out);
        // deliver due messages in random order
        std::vector<Pending> due, keep;
        for (auto& p : q) (p.at <= t ? due : keep).push_back(p);
        q = keep;
        for (size_t i = due.size(); i > 1; --i) std::swap(due[i - 1], due[r.range(0, static_cast<uint32_t>(i - 1))]);
        for (auto& d : due) {
            if (!connected[d.to]) continue;
            views[d.to].apply(d.m);
            if (d.m.kind == OwnMsg::Transfer && d.m.owner == d.to && !r.chance(0.15))   // 15% of acks are lost: the timeout path
                acks.push_back({d.m, static_cast<PlayerId>(d.to), t + static_cast<int>(r.range(0, 3))});
        }
        std::vector<Ack> dueAcks, keepAcks;
        for (auto& a : acks) (a.at <= t ? dueAcks : keepAcks).push_back(a);
        acks = keepAcks;
        OwnershipService::Out aout;
        for (auto& a : dueAcks) {
            OwnMsg ack = a.m; ack.kind = OwnMsg::Ack; ack.owner = a.from;
            svc.onAck(ack, a.from, now, aout);
        }
        emit(aout);

        // invariants on the host
        for (ActorNetId id = 1; id <= kActors; ++id) {
            if (svc.epochOf(id) < lastEpoch[id]) { ok = false; note(why, "epoch went backwards for actor " + std::to_string(id)); }
            lastEpoch[id] = svc.epochOf(id);
            if (views[0].ownerOf(id) != svc.ownerOf(id) || views[0].epochOf(id) != svc.epochOf(id)) {
                ok = false; note(why, "host view differs from service for actor " + std::to_string(id));
            }
        }
        // random actor updates arrive at the host: only the current owner at the current epoch may be accepted
        for (int k = 0; k < 4; ++k) {
            const ActorNetId id = static_cast<ActorNetId>(r.range(1, kActors));
            const PlayerId from = static_cast<PlayerId>(r.range(0, 2));
            OwnerEpoch e = svc.epochOf(id);
            if (r.chance(0.5)) e = static_cast<OwnerEpoch>(r.range(0, e + 1));
            const UpdateVerdict v = views[0].check(id, from, e);
            if (v == UpdateVerdict::Accept && (from != svc.ownerOf(id) || e != svc.epochOf(id))) {
                ok = false; note(why, "accepted a stale or non-owner update for actor " + std::to_string(id));
            }
        }
        if (!ok) return false;
    }
    // drain: everything still queued arrives, then every connected view must equal the host's
    for (auto& d : q) if (connected[d.to]) views[d.to].apply(d.m);
    for (int p = 1; p < kPlayers; ++p) {
        if (!connected[p]) continue;
        // views of players that joined late got a snapshot, others got every message: all must agree with the host
        for (ActorNetId id = 1; id <= kActors; ++id) {
            // a handoff still pending at the end legitimately shows the old owner in a view that has not seen the commit
            if (svc.handoffPending(id)) continue;
            if (views[p].ownerOf(id) != svc.ownerOf(id) || views[p].epochOf(id) != svc.epochOf(id)) {
                ok = false; note(why, "view of p" + std::to_string(p) + " differs from host for actor " + std::to_string(id));
            }
        }
    }
    return ok;
}

bool staleEchoRejected(std::string* why) {
    bool ok = true;
    OwnershipView host(0), a(1), b(2);
    OwnMsg g; g.kind = OwnMsg::Grant; g.netId = 9; g.epoch = 1; g.owner = 1;
    OwnMsg t; t.kind = OwnMsg::Transfer; t.netId = 9; t.epoch = 2; t.owner = 2; t.prevOwner = 1;
    OwnMsg c; c.kind = OwnMsg::Commit; c.netId = 9; c.epoch = 2; c.owner = 2; c.prevOwner = 1;
    for (OwnershipView* v : {&host, &a, &b}) { v->apply(g); v->apply(t); v->apply(c); }
    // the old assignment is echoed back (a stale client re-sending what it believed)
    for (OwnershipView* v : {&host, &a, &b}) {
        v->apply(g);
        if (v->ownerOf(9) != 2 || v->epochOf(9) != 2) { ok = false; note(why, "an echoed old Grant overwrote the new owner"); }
    }
    // the previous owner keeps publishing at its old epoch
    if (host.check(9, 1, 1) == UpdateVerdict::Accept) { ok = false; note(why, "update from the previous owner at the old epoch was accepted"); }
    if (host.check(9, 1, 2) == UpdateVerdict::Accept) { ok = false; note(why, "update from the previous owner at the new epoch was accepted"); }
    if (host.check(9, 2, 2) != UpdateVerdict::Accept) { ok = false; note(why, "the real owner was rejected"); }
    // the new owner sees its own publish reflected back
    if (b.check(9, 2, 2) == UpdateVerdict::Accept) { ok = false; note(why, "own echo accepted"); }
    return ok;
}

bool rejoinEpochsMonotonic(std::string* why) {
    bool ok = true;
    OwnershipService s; s.playerJoined(0); s.playerJoined(1);
    OwnerEpoch last = 0;
    std::vector<OwnMsg> seen;
    for (int round = 0; round < 4; ++round) {
        OwnershipService::Out out;
        s.playerJoined(1);
        s.track(5, {P(1, true, true, 10)}, round * 1000u, out);
        s.evaluate(5, {P(1, true, true, 10)}, round * 1000u + 1, out);
        s.playerLeft(1, round * 1000u + 500, out);
        for (auto& m : out) {
            if (m.epoch <= last) { ok = false; note(why, "epoch did not rise across leave/rejoin"); }
            last = m.epoch;
            seen.push_back(m);
        }
    }
    OwnershipView v(0);
    for (auto& m : seen) v.apply(m);
    for (auto& m : seen) if (v.apply(m) == ApplyResult::Applied) { ok = false; note(why, "a replayed message from an earlier incarnation applied"); }
    return ok;
}

bool commitNothingToHandOverHolds(std::string* why) {
    bool ok = true;
    auto bad = [&](const std::string& m) { ok = false; note(why, m); };
    {   // a live new owner: one Commit at the pending epoch, not counted as forced, and the next call finds nothing pending
        OwnershipService s; s.playerJoined(0); s.playerJoined(1);
        OwnershipService::Out out;
        s.track(7, {P(0, false, false, 9000), P(1, true, true, 50)}, 0, out);
        out.clear();
        s.evaluate(7, {P(0, true, true, 10), P(1, true, true, 50)}, 2000, out);
        if (out.size() != 1 || out[0].kind != OwnMsg::Transfer) bad("setup: no Transfer");
        out.clear();
        if (!s.commitNothingToHandOver(7, 2001, out) || out.size() != 1 || out[0].kind != OwnMsg::Commit || out[0].owner != 0 || out[0].epoch != 2) bad("no Commit at the pending epoch");
        if (s.ownerOf(7) != 0 || s.epochOf(7) != 2 || s.handoffPending(7)) bad("the handoff did not complete");
        if (s.stats().forcedCommits != 0 || s.stats().immediateCommits != 1) bad("counted as forced");
        out.clear();
        if (s.commitNothingToHandOver(7, 2002, out) || !out.empty()) bad("a second call changed something");
        if (s.commitNothingToHandOver(99, 2002, out) || !out.empty()) bad("an unknown actor changed something");
    }
    {   // a new owner that left: nothing is committed to it
        OwnershipService s; s.playerJoined(0); s.playerJoined(1);
        OwnershipService::Out out;
        s.track(7, {P(1, true, true, 50)}, 0, out);
        out.clear();
        s.evaluate(7, {P(0, true, true, 10), P(1, true, true, 50)}, 2000, out);
        out.clear();
        s.playerLeft(0, 2001, out);   // cancels the handoff by itself
        out.clear();
        if (s.commitNothingToHandOver(7, 2002, out) || !out.empty() || s.ownerOf(7) != 1) bad("committed to a new owner that is gone");
    }
    {   // a Transfer to a player the service does not know as connected (the timeout path would abort it): nothing is committed to it
        OwnershipService s; s.playerJoined(0);
        OwnershipService::Out out;
        s.track(7, {P(0, true, true, 10)}, 0, out);                                    // the host owns it
        out.clear();
        s.evaluate(7, {P(0, false, false, 9000), P(1, true, true, 50)}, 2000, out);   // host left: transfer to 1, who never joined
        if (out.empty() || out[0].kind != OwnMsg::Transfer) bad("setup: no Transfer to the unknown client");
        out.clear();
        if (s.commitNothingToHandOver(7, 2002, out) || !out.empty() || s.ownerOf(7) != 0) bad("committed to a player that is not connected");
    }
    return ok;
}

}  // namespace fnvcoop::testing
