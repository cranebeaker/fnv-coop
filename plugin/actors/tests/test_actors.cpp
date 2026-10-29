// plugin/actors tests (game-free). Every scenario is a function returning "" on success or a description of the first violation; the real
// tests CHECK it is empty, the `twin` tests switch a run-time fault in the OwnershipService / live layer on and CHECK it is NOT empty
// (the oracle sees the damage). The compile-time mutants (NVA_MUT=n) run this whole file against a deliberately broken ActorSync.
#include <algorithm>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <sstream>
#include "minitest.h"
#include "rig.h"
#include "actors/owner_rpc.h"
#include "actors/state_parts.h"
#include "actors/state_parts.h"
#include "common/minijson.h"
#include "fnvcoop/wire.h"

using namespace nva_test;

namespace {

std::string fail(const std::string& what, const Rig& r) { return what + " (t=" + std::to_string(r.now) + ")"; }

bool closeTo(float a, float b, float eps) { return std::fabs(a - b) <= eps; }
float dist2d(const live::Vec3& a, const live::Vec3& b) { return std::sqrt((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)); }

Bytes craftBatch(PlayerId peer, const live::ActorSnapshot& s) {
    live::SnapshotSender snd;
    snd.queue(peer, s, 1000);
    std::vector<Bytes> v = snd.flush(peer, 1000);
    return v.empty() ? Bytes() : wrapLive(SubBatch, v[0]);
}

live::ActorSnapshot craftSnap(NetId id, OwnerEpoch epoch, uint32_t seq, CellId cell, live::Vec3 pos) {
    live::ActorSnapshot s;
    s.netId = id; s.ownerEpoch = epoch; s.seq = seq; s.tick = 5000; s.cell = cell; s.pos = pos;
    return s;
}

void inject(Rig& r, int to, int from, const Bytes& data) { r.at(to).sync->receive(static_cast<PlayerId>(from), data.data(), data.size()); }

Bytes envelopeBytes(PlayerId src, PlayerId dest, Body b) {
    Envelope e;
    e.src = src; e.dest = dest; e.body = std::move(b);
    return encodeEnvelope(e);
}

// ---- scenario: one actor, two machines; ownership follows the players, the owner publishes, the others are driven ----------------
std::string scnOwnerNonOwner() {
    Rig r(2);
    r.addActor(100, 3, V(1000), false, ~0u, 40, 0);
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(3000));
    r.run(1500);
    OwnerInfo h = r.host().sync->ownerOf(100), c = r.at(1).sync->ownerOf(100);
    if (h.owner != 0 || c.owner != 0) return fail("host should own an actor both players are near", r);
    if (h.epoch == 0 || h.epoch != c.epoch) return fail("epoch not shared", r);
    if (!r.host().eng.a[100].ai) return fail("owner AI is off", r);
    if (r.at(1).eng.a[100].ai) return fail("non-owner AI is on", r);
    if (r.at(1).sync->stats().published != 0) return fail("non-owner published", r);
    if (r.host().sync->stats().published == 0) return fail("owner never published", r);
    const float ox = r.host().eng.a[100].pos.x, cx = r.at(1).eng.a[100].pos.x;
    if (ox < 1030) return fail("actor never moved", r);
    if (std::fabs(ox - cx) > 40) return fail("non-owner copy lags more than 40 units: " + std::to_string(ox - cx), r);
    if (r.at(1).eng.a[100].drives == 0) return fail("non-owner is not driven from snapshots", r);
    if (!r.errors.empty()) return r.errors[0];
    return "";
}

std::string scnClientOwns() {
    Rig r(2);
    r.addActor(101, 3, V(1000), false, 0b11, 30, 0);
    r.setPose(0, 8, V(0)); r.setPose(1, 3, V(1200));   // the host player is far away
    r.run(1500);
    const OwnerInfo h = r.host().sync->ownerOf(101), c = r.at(1).sync->ownerOf(101);
    if (h.owner != 1 || c.owner != 1) return fail("the client should own an actor only it is near", r);
    if (!r.at(1).eng.a[101].ai) return fail("client AI off for its own actor", r);
    if (r.host().eng.a[101].ai) return fail("host AI on for a client-owned actor", r);
    if (r.host().sync->stats().published != 0) return fail("host published an actor it does not own", r);
    if (dist2d(r.host().eng.a[101].pos, r.at(1).eng.a[101].pos) > 40) return fail("host copy does not follow the client-owned actor", r);
    if (!r.errors.empty()) return r.errors[0];
    return "";
}

// ---- scenario: the host walks away, ownership moves to the client with no respawn and no lost state ------------------------------
std::string scnHandoff(Rig& r) {
    r.addActor(100, 3, V(1000), false, ~0u, 40, 0);
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(3000));
    r.run(1500);
    const OwnerInfo before = r.host().sync->ownerOf(100);
    if (before.owner != 0) return fail("setup: host should own", r);
    const std::vector<std::string> hostOps0 = r.host().eng.opsFor(100);
    r.setPose(0, 9, V(0));   // host leaves: its cell is no longer next to the actor
    uint64_t pendingSince = 0;
    bool aiSwitched = false;
    live::Vec3 hostPosAtSwitch, clientPosAtSwitch;
    for (int i = 0; i < 300 && !aiSwitched; ++i) {
        r.step(10);
        if (!pendingSince && r.host().sync->service()->handoffPending(100)) pendingSince = r.now;
        if (r.at(1).eng.a[100].ai) {
            aiSwitched = true;
            hostPosAtSwitch = r.host().eng.a[100].pos; clientPosAtSwitch = r.at(1).eng.a[100].pos;
        }
    }
    if (!aiSwitched) return fail("the client never took over", r);
    if (pendingSince == 0) return fail("no handoff was ever pending", r);
    if (r.now - pendingSince > 500) return fail("handoff needed " + std::to_string(r.now - pendingSince) + " ms (a forced commit?)", r);
    if (r.host().eng.a[100].ai) return fail("old owner still runs AI", r);
    if (!closeTo(hostPosAtSwitch.x, clientPosAtSwitch.x, 0.45f))   /* the new owner has already walked one 10 ms step (0.4 units) when we look */ return fail("new owner did not continue from the old owner's final state: " + std::to_string(hostPosAtSwitch.x) + " vs " + std::to_string(clientPosAtSwitch.x), r);
    r.run(600);
    const float x1 = r.at(1).eng.a[100].pos.x;
    if (x1 < clientPosAtSwitch.x + 10) return fail("the new owner's AI does not keep walking", r);
    const OwnerInfo h = r.host().sync->ownerOf(100), c = r.at(1).sync->ownerOf(100);
    if (h.owner != 1 || c.owner != 1) return fail("owner after handoff is not the client", r);
    if (h.epoch != before.epoch + 1 || c.epoch != h.epoch) return fail("epoch did not rise by exactly one", r);
    // order of engine calls on the new owner: place, then AI (never the other way round)
    {
        const std::vector<std::string> ops = r.at(1).eng.opsFor(100);
        auto place = std::find(ops.begin(), ops.end(), "place:100"), ai1 = std::find(ops.begin(), ops.end(), "ai1:100");
        if (place == ops.end() || ai1 == ops.end() || place > ai1) return fail("new owner must place the authoritative state BEFORE restoring AI", r);
        if (r.at(1).eng.count("place", 100) != 1 || r.at(1).eng.count("ai1", 100) != 1) return fail("engine calls on the new owner are not exactly one place + one AI-on", r);
    }
    for (auto& mc : r.m) {
        if (mc->eng.count("spawn", 100) || mc->eng.count("remove", 100)) return fail("actor was recreated during the handoff (Respawn/Reset3D class)", r);
        if (mc->eng.a[100].standInSpawns) return fail("clone made for an actor that exists", r);
    }
    if (r.host().eng.opsFor(100).size() < hostOps0.size() + 1 || r.host().eng.count("ai0", 100) < 2) return fail("old owner never turned AI off", r);
    if (r.aiCount(100) != 1) return fail("AI count is not exactly one", r);
    if (!r.errors.empty()) return r.errors[0];
    return "";
}

std::string scnTransferContinuity() { Rig r(2); return scnHandoff(r); }

// ---- scenario: replayed ownership traffic and final states change nothing (idempotent handoff) ----------------------------------
std::string scnIdempotent() {
    Rig r(3);
    std::string e = scnHandoff(r);
    if (!e.empty()) return "setup: " + e;
    r.setPose(2, 9, V(0));
    r.run(300);
    std::vector<std::vector<std::string>> opsBefore;
    std::vector<uint64_t> takeBefore;
    std::vector<OwnerInfo> infoBefore;
    for (auto& mc : r.m) { opsBefore.push_back(mc->eng.ops); takeBefore.push_back(mc->sync->stats().takeovers); infoBefore.push_back(mc->sync->ownerOf(100)); }
    const std::vector<NetMsg> all = r.net.captured;
    for (int pass = 0; pass < 3; ++pass)
        for (size_t i = 0; i < all.size(); ++i) {
            const NetMsg& m = all[pass % 2 ? all.size() - 1 - i : i];   // forward, backward, forward
            r.net.q.push_back(m);
            r.net.q.back().at = r.now; r.net.q.back().order = r.net.order++;
        }
    r.run(200);
    for (int i = 0; i < r.n(); ++i) {
        Machine& mc = r.at(i);
        if (mc.eng.ops.size() != opsBefore[static_cast<size_t>(i)].size()) return fail("machine " + std::to_string(i) + ": a replayed message caused engine calls: " + mc.eng.ops.back(), r);
        if (mc.sync->stats().takeovers != takeBefore[static_cast<size_t>(i)]) return fail("machine " + std::to_string(i) + ": a replay caused a second takeover", r);
        const OwnerInfo o = mc.sync->ownerOf(100);
        if (o.owner != infoBefore[static_cast<size_t>(i)].owner || o.epoch != infoBefore[static_cast<size_t>(i)].epoch) return fail("machine " + std::to_string(i) + ": a replay changed ownership", r);
    }
    if (r.at(1).eng.count("ai1", 100) != 1) return fail("AI restored more than once", r);
    if (r.at(1).eng.a[100].placeCalls != 1) return fail("final state placed more than once", r);
    if (r.aiCount(100) != 1) return fail("AI count after replay is not one", r);
    if (!r.errors.empty()) return r.errors[0];
    return "";
}

// ---- scenario: STR #906. A past owner (or an impostor) talking after a handoff must not move the actor or the ownership ----------
std::string scnStaleEcho() {
    Rig r(3);
    r.addActor(100, 3, V(1000), false, ~0u, 40, 0);
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(2500)); r.setPose(2, 9, V(0));
    r.run(1500);
    const OwnerEpoch e0 = r.host().sync->ownerOf(100).epoch;
    if (r.host().sync->ownerOf(100).owner != 0) return fail("setup: host should own", r);
    r.setPose(0, 9, V(0));
    if (!r.runUntil([&] { return r.at(1).eng.a[100].ai; }, 2000)) return fail("setup: handoff did not happen", r);
    r.run(400);
    const OwnerInfo now1 = r.at(2).sync->ownerOf(100);
    if (now1.owner != 1 || now1.epoch != e0 + 1) return fail("setup: bad owner after handoff", r);
    live::ActorSnapshot cur;
    // (a) the old owner's last words arrive late as a reliable full state, epoch e0
    {
        live::ActorSnapshot stale = craftSnap(100, e0, 900000, 3, V(9999, 9999));
        MSnapFull m; m.s = stale; m.final = true;
        inject(r, 2, 0, envelopeBytes(0, kNoPlayer, m));
        if (!r.at(2).sync->latestState(100, cur) || closeTo(cur.pos.x, 9999, 1)) return fail("stale final state (old epoch) was applied on a bystander", r);
        inject(r, 1, 0, envelopeBytes(0, kNoPlayer, m));
        if (!r.at(1).sync->latestState(100, cur) || closeTo(cur.pos.x, 9999, 1)) return fail("stale final state (old epoch) was applied on the new owner", r);
        r.step(10);
        if (closeTo(r.at(2).eng.a[100].pos.x, 9999, 100)) return fail("the bystander's actor was thrown across the map by a stale echo", r);
    }
    // (b) an old Grant replayed at everybody
    {
        fnvcoop::OwnMsg g; g.kind = fnvcoop::OwnMsg::Grant; g.netId = 100; g.epoch = e0; g.owner = 0; g.prevOwner = kNoPlayer;
        inject(r, 2, 0, envelopeBytes(0, kNoPlayer, MOwn{g}));
        inject(r, 1, 0, envelopeBytes(0, kNoPlayer, MOwn{g}));
        if (r.at(2).sync->ownerOf(100).owner != 1 || r.at(2).sync->ownerOf(100).epoch != e0 + 1) return fail("a replayed old Grant rewound ownership on a bystander", r);
        if (r.at(1).sync->ownerOf(100).owner != 1) return fail("a replayed old Grant took ownership from the new owner", r);
        if (!r.at(1).eng.a[100].ai) return fail("a replayed old Grant turned the new owner's AI off", r);
    }
    // (c) a client that is not the owner publishes at the right epoch: the host refuses
    {
        live::ActorSnapshot imp = craftSnap(100, e0 + 1, 800000, 3, V(7777, 7777));
        inject(r, 0, 2, craftBatch(2, imp));
        if (!r.host().sync->latestState(100, cur) || closeTo(cur.pos.x, 7777, 1)) return fail("the host accepted a snapshot from a player who does not own the actor", r);
        MSnapFull m; m.s = imp; m.s.seq = 800001;
        inject(r, 0, 2, envelopeBytes(2, kNoPlayer, m));
        if (!r.host().sync->latestState(100, cur) || closeTo(cur.pos.x, 7777, 1)) return fail("the host accepted a full state from a non-owner", r);
    }
    // (d) an old-epoch snapshot as a datagram batch, from the old owner, at the new owner and a bystander
    {
        live::ActorSnapshot stale = craftSnap(100, e0, 700000, 3, V(6666, 6666));
        inject(r, 2, 0, craftBatch(0, stale));
        inject(r, 1, 0, craftBatch(0, stale));
        if (!r.at(2).sync->latestState(100, cur) || closeTo(cur.pos.x, 6666, 1)) return fail("a stale batch moved the bystander's copy", r);
        if (!r.at(1).sync->latestState(100, cur) || closeTo(cur.pos.x, 6666, 1)) return fail("a stale batch moved the new owner's state", r);
    }
    r.run(300);
    if (r.at(2).sync->stats().staleEpochDropped == 0) return fail("no stale-epoch drop was counted", r);
    if (r.host().sync->stats().notOwnerDropped == 0) return fail("no non-owner drop was counted at the host", r);
    if (r.aiCount(100) != 1 || !r.at(1).eng.a[100].ai) return fail("AI is not exactly on the owner after the echoes", r);
    if (!r.errors.empty()) return r.errors[0];
    return "";
}

// ---- scenario: follower exception ----------------------------------------------------------------------------------------------
std::string scnFollower() {
    Rig r(2);
    r.addActor(300, 3, V(1000), false, ~0u, 20, 0);
    r.setPose(0, 3, V(0)); r.setPose(1, 6, V(0));   // the client player is far from the actor
    r.run(1200);
    if (r.host().sync->ownerOf(300).owner != 0) return fail("setup: host should own", r);
    fnvcoop::CompanionOwner co;
    co.npc = refOf(300); co.owner = 1; co.active = true;
    r.host().sync->onEffect(fnvcoop::Effect(co));
    r.at(1).sync->onEffect(fnvcoop::Effect(co));
    if (!r.runUntil([&] { return r.at(1).eng.a[300].ai; }, 2000)) return fail("a recruited companion did not move to its recruiter's machine", r);
    if (r.host().eng.a[300].ai) return fail("old owner still runs the companion", r);
    const OwnerEpoch e1 = r.host().sync->ownerOf(300).epoch;
    // the recruiter wanders on; the host player stays right next to the companion: ownership must not come back
    r.setPose(1, 7, V(0));
    r.run(4000);
    if (r.host().sync->ownerOf(300).owner != 1 || r.host().sync->ownerOf(300).epoch != e1) return fail("the companion left its recruiter while the recruiter walked on", r);
    // dismissed: the normal rule returns it to the host (nearest eligible player, host preferred)
    co.active = false;
    r.host().sync->onEffect(fnvcoop::Effect(co));
    if (!r.runUntil([&] { return r.host().sync->ownerOf(300).owner == 0 && r.host().eng.a[300].ai; }, 3000)) return fail("a dismissed companion did not return to the nearest player", r);
    if (r.aiCount(300) != 1) return fail("AI count is not one after dismissal", r);
    if (!r.errors.empty()) return r.errors[0];
    return "";
}

// ---- scenario: no flapping at a border -----------------------------------------------------------------------------------------
std::string scnNoFlap() {
    Rig r(3);
    r.addActor(150, 3, V(0));
    r.setPose(0, 9, V(0));   // host out of range
    r.setPose(1, 3, V(1000)); r.setPose(2, 3, V(1100));
    r.run(1200);
    int changes = 0;
    OwnerEpoch last = r.host().sync->ownerOf(150).epoch;
    for (int i = 0; i < 40; ++i) {   // the two clients take turns being a little closer
        r.setPose(1, 3, V(i % 2 ? 1100.0f : 1000.0f)); r.setPose(2, 3, V(i % 2 ? 1000.0f : 1100.0f));
        r.run(300);
        const OwnerEpoch e = r.host().sync->ownerOf(150).epoch;
        if (e != last) { ++changes; last = e; }
    }
    if (changes > 1) return fail("ownership flapped " + std::to_string(changes) + " times between two equally near players", r);
    return "";
}

// ---- scenario: named NPCs die only through the host's KillActor ------------------------------------------------------------------
std::string scnNamed() {
    Rig r(3);
    r.addActor(200, 3, V(500), true, ~0u);      // named story NPC
    r.addActor(201, 3, V(600), false, ~0u);     // ordinary NPC
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(2000)); r.setPose(2, 3, V(2500));
    r.run(1500);
    for (auto& mc : r.m) if (!mc->eng.a[200].prot) return fail("a named NPC is not protected on machine " + std::to_string(mc->slot), r);
    if (r.host().eng.a[201].prot) return fail("an ordinary NPC was protected", r);
    // the engine on every machine reports a lethal hit: the answer is no (owner: named; others: not the owner)
    for (auto& mc : r.m)
        if (mc->sync->onLocalDeath(200)) return fail("machine " + std::to_string(mc->slot) + " let a named NPC die by damage", r);
    if (r.host().sync->stats().namedDeathsBlocked == 0) return fail("blocked named death not counted", r);
    // forged death declarations: from the owner (named) and from a non-owner
    {
        MDeath d; d.id = 200; d.epoch = r.host().sync->ownerOf(200).epoch; d.eventId = 77; d.cell = 3;
        inject(r, 1, 0, envelopeBytes(0, kNoPlayer, d));
        inject(r, 0, 1, envelopeBytes(1, kNoPlayer, d));
        inject(r, 2, 0, envelopeBytes(0, kNoPlayer, d));
        r.run(200);
        for (auto& mc : r.m) if (mc->sync->isDead(200) || mc->eng.a[200].dead) return fail("a forged death declaration killed the named NPC on machine " + std::to_string(mc->slot), r);
    }
    // the story authority kills it: everybody, exactly once, protection dropped first, ownership released
    {
        fnvcoop::KillActor ka; ka.actor = refOf(200);
        for (auto& mc : r.m) mc->sync->onEffect(fnvcoop::Effect(ka));
        for (auto& mc : r.m) mc->sync->onEffect(fnvcoop::Effect(ka));   // delivered twice: still once
        r.run(400);
        for (auto& mc : r.m) {
            if (!mc->sync->isDead(200)) return fail("KillActor did not kill the named NPC on machine " + std::to_string(mc->slot), r);
            if (mc->eng.a[200].kills != 1) return fail("kill applied " + std::to_string(mc->eng.a[200].kills) + " times on machine " + std::to_string(mc->slot) + " (must be once)", r);
            if (mc->eng.a[200].prot) return fail("protection still on after the authorised kill", r);
            if (mc->eng.a[200].ai) return fail("a dead named NPC still runs AI on machine " + std::to_string(mc->slot), r);
        }
    }
    // an ordinary NPC: the owner's death declaration reaches everybody once
    {
        if (!r.host().sync->onLocalDeath(201)) return fail("the owner may not let an ordinary NPC die", r);
        if (r.at(1).sync->onLocalDeath(201)) return fail("a non-owner declared a death", r);
        r.run(400);
        for (auto& mc : r.m) {
            if (!mc->sync->isDead(201)) return fail("ordinary NPC death did not reach machine " + std::to_string(mc->slot), r);
            if (mc->slot != 0 && mc->eng.a[201].kills != 1) return fail("ordinary NPC death applied " + std::to_string(mc->eng.a[201].kills) + " times on machine " + std::to_string(mc->slot), r);
        }
        // a delayed duplicate of the declaration changes nothing
        NetMsg dm;
        bool haveDm = false;
        for (auto& c : r.net.captured) { Envelope e; if (c.data.size() > 2 && c.data[1] == SubEnvelope && decodeEnvelope(c.data.data() + 2, c.data.size() - 2, e) && std::holds_alternative<MDeath>(e.body)) { dm = c; haveDm = true; break; } }
        if (!haveDm) return fail("no death message was sent", r);
        const int kills = r.at(1).eng.a[201].kills;
        inject(r, 1, dm.from, dm.data);
        if (r.at(1).eng.a[201].kills != kills) return fail("a duplicated death message killed twice", r);
    }
    if (!r.errors.empty()) return r.errors[0];
    return "";
}

// ---- scenario: leveled pick is shared ------------------------------------------------------------------------------------------------
std::string scnPick() {
    const FormRef A("falloutnv.esm", 0x601), B("falloutnv.esm", 0x602), C("falloutnv.esm", 0x603);
    Rig r(3);
    r.addActor(400, 3, V(500), false, 0b011, 0, 0, A);   // machines 0 and 1 have it; machine 2 gets it later
    r.at(1).eng.a[400].base = B; r.at(1).sync->onNativeBase(400, B);   // the client's own leveled roll differs
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(2000)); r.setPose(2, 3, V(2500));
    r.run(1500);
    if (r.host().sync->ownerOf(400).owner != 0) return fail("setup: host should own", r);
    for (int i = 0; i < 2; ++i) {
        if (r.at(i).sync->pickOf(400) != A) return fail("machine " + std::to_string(i) + " has no pick A", r);
        if (r.at(i).eng.a[400].base != A) return fail("machine " + std::to_string(i) + " engine base is not the owner's pick", r);
    }
    if (r.at(1).eng.a[400].baseSets != 1) return fail("client swapped its base " + std::to_string(r.at(1).eng.a[400].baseSets) + " times", r);
    // a non-owner cannot override the pick
    {
        MPickReport rep; rep.id = 400; rep.base = B;
        inject(r, 0, 1, envelopeBytes(1, kHostPlayer, rep));
        r.run(100);
        if (r.host().sync->pickOf(400) != A || r.at(1).sync->pickOf(400) != A) return fail("a non-owner's report changed the pick", r);
    }
    // ownership moves; the new owner's native roll does not replace the pick
    r.setPose(0, 9, V(0));
    if (!r.runUntil([&] { return r.at(1).eng.a[400].ai; }, 2500)) return fail("setup: handoff did not happen", r);
    r.run(300);
    if (r.host().sync->pickOf(400) != A || r.at(1).eng.a[400].base != A || r.host().eng.a[400].base != A) return fail("the pick changed with the owner", r);
    r.at(1).eng.a[400].base = B; r.at(1).sync->onNativeBase(400, B);   // the engine rolls again (e.g. after a reload)
    if (r.at(1).eng.a[400].base != A) return fail("a re-roll was not swapped back to the shared pick", r);
    // late load: the actor appears on machine 2 with yet another roll
    r.at(2).eng.addReal(400, 3, V(500), C);
    ActorMeta meta; meta.netId = 400; meta.ref = refOf(400); meta.base = C; meta.cell = 3; meta.pos = V(500);
    r.at(2).sync->track(meta);
    r.run(200);
    if (r.at(2).eng.a[400].base != A) return fail("a late-loaded actor kept its own roll", r);
    return r.errors.empty() ? "" : r.errors[0];
}

// ---- scenario: stand-in clones ----------------------------------------------------------------------------------------------------
std::string scnStandIn() {
    Rig r(2);
    r.addActor(500, 3, V(800), false, 0b01, 40, 0);   // only the host has it loaded
    r.at(1).eng.cellsLoaded.insert(3);                // the client has the cell, but no ref of the actor
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(400));
    r.run(1500);
    Machine& c = r.at(1);
    if (!c.sync->isStandIn(500) || c.eng.count("spawn", 500) != 1) return fail("no stand-in was made for an actor the client does not have", r);
    if (!c.eng.a[500].standIn || c.eng.a[500].real) return fail("stand-in state wrong", r);
    if (c.eng.a[500].ai) return fail("a stand-in runs AI", r);
    if (c.sync->stats().published != 0) return fail("a stand-in published", r);
    if (std::fabs(c.eng.a[500].pos.x - r.host().eng.a[500].pos.x) > 40) return fail("stand-in does not follow the owner", r);
    if (r.host().sync->isStandIn(500)) return fail("the owner made a clone of its own actor", r);
    // the real actor loads: the clone goes away, exactly once
    c.eng.addReal(500, 3, V(900), FormRef("falloutnv.esm", 0x500));
    c.sync->onActorLoaded(500);
    r.run(300);
    if (c.sync->isStandIn(500) || c.eng.count("remove", 500) != 1) return fail("the stand-in was not removed when the real actor loaded", r);
    if (c.eng.a[500].ai) return fail("the real actor runs AI on a non-owner after the stand-in was replaced", r);
    // a second one, then purge on load
    r.addActor(501, 3, V(900), false, 0b01);
    r.run(600);
    if (!c.sync->isStandIn(501)) return fail("second stand-in missing", r);
    c.sync->purgeStandIns();
    if (!c.sync->standIns().empty() || c.eng.a[501].standIn) return fail("stand-ins survived a purge (they must never be saved)", r);
    // an unloaded cell makes it go away too
    if (!r.runUntil([&] { return c.sync->isStandIn(501); }, 600)) return fail("the purged stand-in never came back while the actor is still around", r);
    c.eng.cellsLoaded.clear();
    r.run(200);
    if (c.sync->isStandIn(501)) return fail("a stand-in outlived its cell", r);
    return r.errors.empty() ? "" : r.errors[0];
}

// ---- scenario: the host loses the final state: the forced commit still hands over ------------------------------------------------
std::string scnLostFinal() {
    Rig r(2);
    r.net.dropIf = [](const NetMsg& m) {
        Envelope e;
        return m.data.size() > 2 && m.data[1] == SubEnvelope && decodeEnvelope(m.data.data() + 2, m.data.size() - 2, e) && std::holds_alternative<MSnapFull>(e.body) && std::get<MSnapFull>(e.body).final;
    };
    r.addActor(100, 3, V(1000), false, ~0u, 40, 0);
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(3000));
    r.run(1500);
    r.setPose(0, 9, V(0));
    if (!r.runUntil([&] { return r.at(1).eng.a[100].ai; }, 3000)) return fail("a lost final state must not block the handoff forever", r);
    if (r.host().sync->service()->stats().forcedCommits == 0) return fail("the commit was not forced", r);
    if (std::fabs(r.at(1).eng.a[100].pos.x - r.host().eng.a[100].pos.x) > 60) return fail("takeover without the final state used a stale position", r);
    {   // no final state arrived: the best-known state is placed at the takeover, still before the AI comes back
        const std::vector<std::string> ops = r.at(1).eng.opsFor(100);
        auto place = std::find(ops.begin(), ops.end(), "place:100"), ai1 = std::find(ops.begin(), ops.end(), "ai1:100");
        if (place == ops.end() || ai1 == ops.end() || place > ai1) return fail("a takeover without a final state must place the best-known state before restoring AI", r);
        if (r.at(1).eng.a[100].placeCalls != 1) return fail("expected exactly one place on the new owner", r);
    }
    return r.errors.empty() ? "" : r.errors[0];
}

// ---- scenario: the old owner's final state is overtaken by the unreliable copy of the same snapshot ---------------------------------
// The host relays every actor's newest snapshot to the clients unreliably, and that includes the final it has just sent reliably (same seq). On a link with jitter
// (lab run 20261028-032920: transatlantic all 7 actors, mobile_tether the NPC) the copy arrives first, the final is then "the same seq again", is dropped, and the new
// owner never acknowledges: the host forced the commit after handoffTimeoutMs (1000 ms) and ownership reached the client 1.1 s late.
std::string scnFinalOvertaken(uint32_t finalLagMs, bool hostIsNew) {
    Rig r(2);
    r.net.extraDelay = [finalLagMs](const NetMsg& m) -> uint32_t {
        Envelope e;
        return m.data.size() > 2 && m.data[1] == SubEnvelope && decodeEnvelope(m.data.data() + 2, m.data.size() - 2, e) && std::holds_alternative<MSnapFull>(e.body) &&
                       std::get<MSnapFull>(e.body).final ? finalLagMs : 0u;
    };
    r.addActor(100, 3, V(1000), false, ~0u, 40, 0);
    if (hostIsNew) { r.setPose(0, 8, V(0)); r.setPose(1, 3, V(1200)); }   // the client owns it first, the host walks back
    else { r.setPose(0, 3, V(0)); r.setPose(1, 3, V(3000)); }
    r.run(1500);
    const PlayerId first = hostIsNew ? 1 : 0, second = hostIsNew ? 0 : 1;
    if (r.host().sync->ownerOf(100).owner != first) return fail("setup: wrong first owner", r);
    const uint64_t forced0 = r.host().sync->service()->stats().forcedCommits;
    if (hostIsNew) r.setPose(0, 3, V(0)); else r.setPose(0, 9, V(0));
    const uint64_t t0 = r.now;
    if (!r.runUntil([&] { return r.at(second).eng.a[100].ai; }, 3000)) return fail("the new owner never took over", r);
    const uint64_t took = r.now - t0;
    if (r.host().sync->service()->stats().forcedCommits != forced0) return fail("the handoff needed a forced commit (the final state was dropped as stale): " + std::to_string(took) + " ms", r);
    if (took > 600) return fail("handoff took " + std::to_string(took) + " ms", r);
    if (r.at(second).eng.count("place", 100) != 1) return fail("the final state was not placed on the new owner", r);
    r.run(500);
    if (r.aiCount(100) != 1) return fail("AI count is not exactly one", r);
    if (!r.errors.empty()) return r.errors[0];
    return "";
}

// ---- scenario: the old owner has no state to hand over: nothing to wait for ----------------------------------------------------------
// An actor the host owns but never sampled (it is not loaded on the host) has no final state to send. The new owner then never gets one to acknowledge, and every such
// handoff waited the whole handoffTimeoutMs (lab run 20261028-032920: one actor at 1013 ms after every host move, in every pass, on every profile).
std::string scnNothingToHandOver() {
    Rig r(2);
    r.addActor(100, 3, V(1000), false, 0b10, 0, 0);    // only the client's engine has it; the host learns of it from the client's track message
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(1200));
    r.run(1500);
    if (r.host().sync->ownerOf(100).owner != 0) return fail("setup: the host should own it (eligible, preferred)", r);
    const uint64_t forced0 = r.host().sync->service()->stats().forcedCommits;
    r.setPose(0, 9, V(0));
    const uint64_t t0 = r.now;
    if (!r.runUntil([&] { return r.at(1).eng.a[100].ai; }, 3000)) return fail("the client never took over", r);
    const uint64_t took = r.now - t0;
    if (r.host().sync->service()->stats().forcedCommits != forced0) return fail("a handoff with nothing to hand over waited for the timeout: " + std::to_string(took) + " ms", r);
    if (took > 600) return fail("handoff took " + std::to_string(took) + " ms", r);
    if (r.host().sync->ownerOf(100).owner != 1 || r.at(1).sync->ownerOf(100).owner != 1) return fail("owner after the handoff is not the client", r);
    if (!r.errors.empty()) return r.errors[0];
    return "";
}

// ---- scenario: a copy's actor value drifted after the owner's last snapshot ----------------------------------------------------------
// The owner's value is written to a copy once, when it changes. An engine that moves the actor's damage when the copy finishes loading leaves the copy at another health
// with nothing new from the owner to correct it (lab vats room behind bad_wifi: the copy read -55 for the owner's 5, until the area digest healed it).
std::string scnAvDrift() {
    Rig r(2);
    r.addActor(100, 3, V(1000), false, ~0u, 0, 0);
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(1200));
    r.run(1500);
    if (r.host().sync->ownerOf(100).owner != 0) return fail("setup: the host should own it", r);
    r.host().eng.a[100].av[0] = 5;                         // the owner weakens it
    r.run(600);
    if (r.at(1).eng.a[100].av[0] != 5) return fail("setup: the copy did not take the owner's health: " + std::to_string(r.at(1).eng.a[100].av[0]), r);
    r.at(1).eng.a[100].av[0] = -55;                         // the engine moves the damage back when the copy loads
    r.run(900);                                             // nothing new from the owner: it is idle; the audit runs well inside the area digest's 1 s settle time
    if (r.at(1).eng.a[100].av[0] != 5) return fail("the copy kept the drifted health " + std::to_string(r.at(1).eng.a[100].av[0]) + " for the owner's 5", r);
    if (r.host().eng.a[100].av[0] != 5) return fail("the owner's health changed", r);
    if (!r.errors.empty()) return r.errors[0];
    return "";
}

// ---- scenario: a client owner leaves ----------------------------------------------------------------------------------------------
std::string scnOwnerLeaves() {
    Rig r(2);
    r.addActor(101, 3, V(1000), false, ~0u, 30, 0);
    r.setPose(0, 5, V(0)); r.setPose(1, 3, V(1000));
    r.run(1500);
    if (r.host().sync->ownerOf(101).owner != 1) return fail("setup: client should own", r);
    r.net.down.insert(1);
    r.host().sync->playerLeft(1);
    r.setPose(1, 0, V(0), false);
    r.setPose(0, 3, V(0));
    r.run(1500);
    if (r.host().sync->ownerOf(101).owner != 0 || !r.host().eng.a[101].ai) return fail("the host did not take over an actor whose owner left", r);
    if (r.host().eng.count("spawn", 101) || r.host().eng.count("remove", 101)) return fail("recreated on owner loss", r);
    return r.errors.empty() ? "" : r.errors[0];
}

// ---- scenario: a client-owned NPC dies; a scripted resurrect continues the same ownership history ----------------------------------
std::string scnClientOwnerDeath() {
    Rig r(3);
    r.addActor(101, 3, V(1000), false, ~0u, 30, 0);
    r.setPose(0, 8, V(0)); r.setPose(1, 3, V(1200)); r.setPose(2, 3, V(2500));
    r.run(1500);
    if (r.host().sync->ownerOf(101).owner != 1) return fail("setup: client 1 should own", r);
    if (r.host().sync->onLocalDeath(101)) return fail("the host (not the owner) declared a death", r);
    if (!r.at(1).sync->onLocalDeath(101)) return fail("the owner may not let an ordinary NPC die", r);
    r.run(500);
    for (auto& mc : r.m) {
        if (!mc->sync->isDead(101)) return fail("death of a client-owned NPC did not reach machine " + std::to_string(mc->slot), r);
        if (mc->eng.a[101].ai) return fail("a dead NPC still runs AI on machine " + std::to_string(mc->slot), r);
        if (mc->slot != 1 && mc->eng.a[101].kills != 1) return fail("death applied " + std::to_string(mc->eng.a[101].kills) + " times on machine " + std::to_string(mc->slot), r);
        if (mc->slot == 1 && mc->eng.a[101].kills != 0) return fail("the owner's own engine was told to kill again", r);
    }
    return r.errors.empty() ? "" : r.errors[0];
}

std::string scnResurrect() {
    Rig r(2);
    r.addActor(210, 3, V(500), true, ~0u, 20, 0);
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(2000));
    r.run(1500);
    const OwnerInfo before = r.host().sync->ownerOf(210);
    if (before.owner != 0) return fail("setup: host should own", r);
    fnvcoop::KillActor ka; ka.actor = refOf(210);
    for (auto& mc : r.m) mc->sync->onEffect(fnvcoop::Effect(ka));
    r.run(600);
    for (auto& mc : r.m) if (!mc->sync->isDead(210) || mc->eng.a[210].ai) return fail("setup: kill did not take on machine " + std::to_string(mc->slot), r);
    const float xDead = r.host().eng.a[210].pos.x;
    fnvcoop::ResurrectActor rz; rz.actor = refOf(210);
    for (auto& mc : r.m) mc->sync->onEffect(fnvcoop::Effect(rz));
    r.run(1500);
    for (auto& mc : r.m) {
        if (mc->sync->isDead(210) || mc->eng.a[210].dead) return fail("resurrect did not revive on machine " + std::to_string(mc->slot), r);
        if (mc->eng.a[210].revives != 1) return fail("revive applied " + std::to_string(mc->eng.a[210].revives) + " times on machine " + std::to_string(mc->slot), r);
        if (!mc->eng.a[210].prot) return fail("a resurrected named NPC is not protected again on machine " + std::to_string(mc->slot), r);
    }
    const OwnerInfo after = r.host().sync->ownerOf(210), c = r.at(1).sync->ownerOf(210);
    if (after.owner != 0 || c.owner != 0) return fail("ownership after a resurrect is not the host's", r);
    if (after.epoch != before.epoch || c.epoch != before.epoch) return fail("the resurrect restarted the epoch history: " + std::to_string(before.epoch) + " -> " + std::to_string(after.epoch) + "/" + std::to_string(c.epoch), r);
    if (!r.host().eng.a[210].ai || r.at(1).eng.a[210].ai) return fail("AI is not on the owner only after the resurrect", r);
    if (r.host().eng.a[210].pos.x <= xDead + 5) return fail("the resurrected NPC does not move", r);
    if (std::fabs(r.at(1).eng.a[210].pos.x - r.host().eng.a[210].pos.x) > 40) return fail("the other copy does not follow the resurrected NPC", r);
    return r.errors.empty() ? "" : r.errors[0];
}

// ---- scenario: the host restarts from a save (a new session: ownership epochs start again) while a client keeps running ---------------
// The client had seen epoch 2 for the actor. A new host numbers its epochs from 1 again; a client that kept its old epoch / seq floors would
// drop every snapshot of the new session as "stale" and show a frozen actor forever.
std::string scnHostRestart() {
    Rig r(2);
    std::string e = scnHandoff(r);   // the client now owns actor 100 at epoch 2 and has published at it
    if (!e.empty()) return "setup: " + e;
    Machine& h = r.host();
    SyncConfig c;
    c.self = 0; c.isHost = true;
    c.adjacent = [](CellId a, CellId b) { return a == b || a + 1 == b || b + 1 == a; };
    h.sync = std::make_unique<ActorSync>(c, h.eng, *h.ep);   // the host process was restarted: no ownership history
    h.sync->playerJoined(1);
    ActorMeta meta;
    meta.netId = 100; meta.ref = refOf(100); meta.base = FormRef("falloutnv.esm", 0x500); meta.cell = 3; meta.pos = V(1000);
    h.sync->track(meta);
    r.at(1).sync->hostLost();
    r.at(1).sync->hostConnected();
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(3000));   // the host is next to the actor again (the client is in the cell too, but farther)
    const int drivesBefore = r.at(1).eng.a[100].drives;
    r.run(3000);
    const OwnerInfo hi = r.host().sync->ownerOf(100), ci = r.at(1).sync->ownerOf(100);
    if (hi.owner != 0 || ci.owner != 0) return fail("the new session did not give the actor to the host (host sees " + std::to_string(hi.owner) + ", client sees " + std::to_string(ci.owner) + ")", r);
    if (ci.epoch != hi.epoch) return fail("client and host disagree on the epoch after the restart", r);
    if (r.at(1).eng.a[100].drives < drivesBefore + 20) { const Stats& st = r.at(1).sync->stats(); const Stats& hs = r.host().sync->stats(); return fail("the client stopped following the actor after the host restarted (its old epoch floor rejects the new session) drives=" + std::to_string(r.at(1).eng.a[100].drives) + " before=" + std::to_string(drivesBefore) + " applied=" + std::to_string(st.snapshotsApplied) + " staleEp=" + std::to_string(st.staleEpochDropped) + " staleSeq=" + std::to_string(st.staleSeqDropped) + " notOwner=" + std::to_string(st.notOwnerDropped) + " future=" + std::to_string(st.futureEpoch) + " unknown=" + std::to_string(st.unknownActor) + " hostPub=" + std::to_string(hs.published) + " hostEpoch=" + std::to_string(hi.epoch), r); }
    if (std::fabs(r.at(1).eng.a[100].pos.x - r.host().eng.a[100].pos.x) > 60) return fail("the client's copy is frozen far from the host's actor after the host restarted", r);
    return "";
}

// ---- scenario: a client whose actor module comes up after the host's join burst ----------------------------------------------------
// ActorRuntime registers its mux channel only once the client has joined, and the host sends its records / picks / ownership table in the
// frame it sees the join: those datagrams can reach the client's mux before any handler exists and are dropped there. The same happens to a
// client that goes through a baseline (hostLost / hostConnected) while the host never saw it leave. The client has to ask for the table.
std::string scnJoinBurstLost() {
    Rig r(2);
    r.net.dropIf = [](const NetMsg& m) { return m.from == 0 && m.to == 1; };   // nothing the host sends reaches the client's module yet
    r.addActor(100, 3, V(1000), false, 0b01, 40, 0);                            // only the host has it loaded
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(3000));
    r.run(1200);
    if (r.at(1).sync->ownerOf(100).known) return fail("setup: the client learned of the actor although the host's datagrams were dropped", r);
    r.net.dropIf = nullptr;
    r.at(1).sync->hostConnected();   // the runtime has just come up
    r.run(1500);
    if (!r.at(1).sync->ownerOf(100).known) return fail("a client that missed the host's join burst never learned the actor", r);
    std::string ag = r.agreement(100);
    if (!ag.empty()) return fail("after the late start: " + ag, r);
    if (r.at(1).sync->ownerOf(100).owner != 0) return fail("the client does not know the host owns the actor", r);
    // a baseline: the client's view is wiped, the host does not notice anything
    r.at(1).sync->hostLost();
    if (r.at(1).sync->ownerOf(100).owner != kNoPlayer) return fail("setup: hostLost kept the ownership view", r);
    r.at(1).sync->hostConnected();
    r.run(1500);
    ag = r.agreement(100);
    if (!ag.empty()) return fail("after a baseline: " + ag, r);
    if (r.at(1).sync->ownerOf(100).owner != 0) return fail("the client's ownership view stayed empty after a baseline", r);
    return r.errors.empty() ? "" : r.errors[0];
}

// ---- scenario: a forced commit while the old owner has not heard of the Transfer ---------------------------------------------------
// The host's messages to guest 1 (the owner) take 2.6 s, longer than the handoff timeout (1 s): the host commits without guest 1's final state,
// while guest 1 keeps simulating until the Transfer reaches it. The new owner must not start its AI before guest 1's final state (or its
// departure, or the safety timeout) shows that guest 1 stopped: at no step may two machines run the AI of one actor.
std::string scnSlowOldOwner(bool newOwnerIsHost) {
    Rig r(newOwnerIsHost ? 2 : 3);
    r.addActor(100, 3, V(1000), false, ~0u, 40, 0);
    r.setPose(0, 9, V(0)); r.setPose(1, 3, V(3000));
    if (!newOwnerIsHost) r.setPose(2, 3, V(6000));
    r.run(1800);
    if (r.host().sync->ownerOf(100).owner != 1) return fail("setup: guest 1 should own the actor", r);
    r.net.extraDelay = [](const NetMsg& m) { return (m.from == 0 && m.to == 1) ? 2600u : 0u; };
    const int next = newOwnerIsHost ? 0 : 2;
    if (newOwnerIsHost) r.setPose(0, 3, V(1000)); else r.setPose(2, 3, V(1100));
    const uint64_t t0 = r.now;
    if (!r.runUntil([&] { return r.host().sync->ownerOf(100).owner == next; }, 4000)) return fail("setup: the actor never moved to machine " + std::to_string(next), r);
    if (r.host().sync->service()->stats().forcedCommits == 0) return fail("setup: the commit was not forced", r);
    r.run(1000);   // the old owner still has not heard (2.6 s)
    if (r.at(next).eng.a[100].ai) return fail("the new owner drives the actor while the old owner has not heard of the handoff", r);
    if (r.at(1).eng.a[100].ai == false && r.now < t0 + 2400) return fail("setup: guest 1 stopped before it could have heard", r);
    // a final state of a much older handoff says nothing about this one
    if (!newOwnerIsHost) {
        MSnapFull old; old.s = craftSnap(100, 0, 99, 3, V(1000)); old.final = true;
        inject(r, 2, 0, envelopeBytes(1, kNoPlayer, old));
        r.run(10);
        if (r.at(2).eng.a[100].ai) return fail("an old-epoch final state released the hold", r);
    }
    if (!r.runUntil([&] { return r.at(next).eng.a[100].ai; }, 4000 - 1000)) return fail("the new owner never started after the old owner stopped", r);
    if (r.now - t0 > 3600) return fail("the new owner waited for the safety timeout instead of the old owner's final state (" + std::to_string(r.now - t0) + " ms)", r);
    r.run(600);
    if (r.aiCount(100) != 1) return fail("AI count is not exactly one", r);
    if (!newOwnerIsHost && r.host().sync->stats().staleFinalsForwarded == 0) return fail("the host never passed the old owner's final state on", r);
    r.net.extraDelay = nullptr;
    r.run(1500);
    const std::string ag = r.agreement(100);
    if (!ag.empty()) return fail(ag, r);
    return r.errors.empty() ? "" : r.errors[0];
}

void checkEmpty(const std::string& e) { CHECK_MSG(e.empty(), e); }

}  // namespace

TEST(own, owner_and_non_owner) { checkEmpty(scnOwnerNonOwner()); }
TEST(own, client_owns_what_only_it_is_near) { checkEmpty(scnClientOwns()); }
TEST(own, transfer_continues_from_final_state) { checkEmpty(scnTransferContinuity()); }
TEST(own, transfer_is_idempotent) { checkEmpty(scnIdempotent()); }
TEST(own, stale_echo_str906) { checkEmpty(scnStaleEcho()); }
TEST(own, named_npc_protection) { checkEmpty(scnNamed()); }
TEST(own, follower_exception) { checkEmpty(scnFollower()); }
TEST(own, no_flapping_at_a_border) { checkEmpty(scnNoFlap()); }
TEST(own, leveled_pick_shared) { checkEmpty(scnPick()); }
TEST(own, stand_in_clones) { checkEmpty(scnStandIn()); }
TEST(own, lost_final_state_forced_commit) { checkEmpty(scnLostFinal()); }
TEST(own, host_restart_resets_epoch_floors) { checkEmpty(scnHostRestart()); }
TEST(own, join_burst_lost_is_requested_again) { checkEmpty(scnJoinBurstLost()); }
TEST(own, forced_commit_host_new_owner_waits_for_old_owner) { checkEmpty(scnSlowOldOwner(true)); }
TEST(own, forced_commit_guest_new_owner_waits_for_old_owner) { checkEmpty(scnSlowOldOwner(false)); }
TEST(own, owner_leaves) { checkEmpty(scnOwnerLeaves()); }
TEST(own, a_copys_drifted_health_is_brought_back_without_a_new_snapshot) { checkEmpty(scnAvDrift()); }
TEST(own, nothing_to_hand_over_commits_without_the_timeout) { checkEmpty(scnNothingToHandOver()); }
TEST(own, final_state_overtaken_by_its_relay_copy_still_hands_over_host_to_client) { checkEmpty(scnFinalOvertaken(30, false)); }
TEST(own, final_state_overtaken_by_its_relay_copy_still_hands_over_client_to_host) { checkEmpty(scnFinalOvertaken(30, true)); }
TEST(own, client_owner_death) { checkEmpty(scnClientOwnerDeath()); }
TEST(own, resurrect_continues_epoch_history) { checkEmpty(scnResurrect()); }

TEST(own, handoff_with_loss_and_jitter) {
    NetOpts o;
    o.lossUnreliable = 0.2; o.dupUnreliable = 0.1; o.jitterMs = 40;
    Rig r(2, 5, o);
    checkEmpty(scnHandoff(r));
}

// ---- run-time fault twins: the oracle must notice the damage -------------------------------------------------------------------------
TEST(twin, accept_stale_epoch) {
    fnvcoop::OwnershipFaults f; f.acceptStaleEpoch = true;
    fnvcoop::OwnershipFaultScope s(f);
    CHECK(!scnStaleEcho().empty());
}
TEST(twin, adopt_echo) {
    fnvcoop::OwnershipFaults f; f.adoptEcho = true;
    fnvcoop::OwnershipFaultScope s(f);
    CHECK(!scnStaleEcho().empty());
}
TEST(twin, accept_non_owner) {
    fnvcoop::OwnershipFaults f; f.acceptNonOwner = true;
    fnvcoop::OwnershipFaultScope s(f);
    CHECK(!scnStaleEcho().empty());
}
TEST(twin, follower_ignored) {
    fnvcoop::OwnershipFaults f; f.followerIgnored = true;
    fnvcoop::OwnershipFaultScope s(f);
    CHECK(!scnFollower().empty());
}
TEST(twin, transfer_no_ack) {
    fnvcoop::OwnershipFaults f; f.transferNoAck = true;
    fnvcoop::OwnershipFaultScope s(f);
    CHECK(!scnTransferContinuity().empty());
}
TEST(twin, no_hysteresis) {
    fnvcoop::OwnershipFaults f; f.noHysteresis = true;
    fnvcoop::OwnershipFaultScope s(f);
    CHECK(!scnNoFlap().empty());
}
TEST(twin, no_epoch_bump) {
    fnvcoop::OwnershipFaults f; f.noEpochBump = true;
    fnvcoop::OwnershipFaultScope s(f);
    CHECK(!scnTransferContinuity().empty());
}

// ---- coop.owner ------------------------------------------------------------------------------------------------------------------------
TEST(rpc, coop_owner) {
    Rig r(3);
    r.addActor(100, 3, V(1000), false, ~0u, 20, 0);
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(2000)); r.setPose(2, 3, V(2500));
    r.run(1500);
    const OwnerInfo truth = r.host().sync->ownerOf(100);
    CHECK(truth.known && truth.owner == 0 && truth.epoch >= 1);
    CHECK(r.host().sync->service()->ownerOf(100) == truth.owner && r.host().sync->service()->epochOf(100) == truth.epoch);   // the service itself agrees
    for (int i = 0; i < 3; ++i) {
        std::string out;
        CHECK(nvcoop::actors::ownerRpc(r.at(i).sync.get(), "{\"ref\":\"FalloutNV.esm:000064\"}", out));
        nvcoop::json::Value v; std::string err;
        REQUIRE(nvcoop::json::parse(out, v, err));
        CHECK(v.get("known").asBool());
        CHECK_MSG(v.get("owner").asInt(-1) == truth.owner, out);
        CHECK_MSG(v.get("epoch").asInt(-1) == static_cast<int64_t>(truth.epoch), out);
        CHECK(v.get("self").asInt(-1) == i);
        CHECK(v.get("mine").asBool() == (i == 0));
        CHECK(v.get("ai").asBool() == (i == 0));
        CHECK(!v.find("unsupported"));
    }
    // after a handoff every machine reports the new pair
    r.setPose(0, 9, V(0));
    CHECK(r.runUntil([&] { return r.at(1).eng.a[100].ai || r.at(2).eng.a[100].ai; }, 3000));
    r.run(400);
    const OwnerInfo t2 = r.host().sync->ownerOf(100);
    CHECK(t2.owner != 0 && t2.epoch == truth.epoch + 1);
    for (int i = 0; i < 3; ++i) {
        std::string out;
        CHECK(nvcoop::actors::ownerRpc(r.at(i).sync.get(), "{\"ref\":\"falloutnv.esm:64\"}", out));
        nvcoop::json::Value v; std::string err;
        REQUIRE(nvcoop::json::parse(out, v, err));
        CHECK_MSG(v.get("owner").asInt(-1) == t2.owner && v.get("epoch").asInt(-1) == static_cast<int64_t>(t2.epoch), out);
    }
    std::string out;
    CHECK(nvcoop::actors::ownerRpc(r.host().sync.get(), "{\"ref\":\"falloutnv.esm:00FFFF\"}", out));
    CHECK(out.find("\"known\":false") != std::string::npos && out.find("\"owner\":255") != std::string::npos);
    CHECK(nvcoop::actors::ownerRpc(nullptr, "{}", out) && out.find("unsupported") != std::string::npos);
    CHECK(!nvcoop::actors::ownerRpc(r.host().sync.get(), "{\"ref\":\"nonsense\"}", out));
    CHECK(!nvcoop::actors::ownerRpc(r.host().sync.get(), "{}", out));
    CHECK(!nvcoop::actors::ownerRpc(r.host().sync.get(), "{not json", out));
    FormRef fr;
    CHECK(nvcoop::actors::parseRef("FalloutNV.esm:00ABCD", fr) && fr.plugin == "falloutnv.esm" && fr.localId == 0xABCD);
    CHECK(!nvcoop::actors::parseRef(":12", fr) && !nvcoop::actors::parseRef("a.esm:", fr) && !nvcoop::actors::parseRef("a.esm:zz", fr));
}

// ---- wire ----------------------------------------------------------------------------------------------------------------------------
namespace {
std::vector<Body> sampleBodies() {
    std::vector<Body> v;
    fnvcoop::OwnMsg g; g.kind = fnvcoop::OwnMsg::Transfer; g.netId = 5; g.epoch = 9; g.owner = 2; g.prevOwner = 1;
    v.push_back(MOwn{g});
    ActorMeta meta; meta.netId = 7; meta.ref = refOf(7); meta.base = refOf(8); meta.named = true; meta.created = true; meta.cell = 4; meta.pos = V(1, 2, 3);
    v.push_back(MTrack{meta});
    MSnapFull sf; sf.s = craftSnap(7, 3, 4, 5, V(1.5f, -2.5f, 3)); sf.s.rot = V(0.1f, 0.2f, 0.3f); sf.s.vel = V(4, 5, 6);
    sf.s.anim.locomotion = 3; sf.s.anim.sneak = true; sf.s.anim.aim = true; sf.s.ai.combatTarget = 11; sf.s.ai.package = 12; sf.s.ai.alert = 2;
    live::AvValue av; av.av = 1; av.value = -40; sf.s.avDeltas.push_back(av);
    sf.final = true;
    v.push_back(sf);
    MDeath d; d.id = 7; d.epoch = 3; d.eventId = 0x0100000000000009ull; d.cell = 4; d.pos = V(9, 8, 7);
    v.push_back(d);
    v.push_back(MPick{7, refOf(99)});
    v.push_back(MPickReport{7, refOf(98)});
    v.push_back(MResync{7});
    // enemy-anim: the schema v2 animation extension inside the full state, and the opaque owner event
    MSnapFull sf2; sf2.s = craftSnap(8, 3, 6, 5, V(1, 2, 3));
    sf2.s.anim.locomotion = 5; sf2.s.anim.weaponDrawn = true; sf2.s.anim.action = 223; sf2.s.anim.actionSeq = 200; sf2.s.anim.knocked = true; sf2.s.anim.ragdoll = true;
    v.push_back(sf2);
    // live schema v3: a seated actor's full state keeps its furniture block (a handoff / resync must not stand it up)
    MSnapFull sf3; sf3.s = craftSnap(9, 3, 7, 6, V(4, 5, 6));
    sf3.s.anim.sit = live::SitState::Using; sf3.s.anim.sleep = true; sf3.s.anim.furniture = 0x00104C2Au;
    v.push_back(sf3);
    MAnimEvent ae; ae.id = 7; ae.epoch = 3; ae.eventId = 0x0100000000000011ull; ae.payload = Bytes{1, 2, 3, 4, 5};
    v.push_back(ae);
    MPart iv; iv.id = 7; iv.epoch = 3; iv.eventId = 0x0100000000000012ull; iv.kind = PartKind::Inventory;
    iv.value = encodeInventory({{refOf(0x15169), 2}, {refOf(0x8ED03), 300}});
    v.push_back(iv);
    MPart eq; eq.id = 7; eq.epoch = 3; eq.eventId = 0x0100000000000013ull; eq.kind = PartKind::Equipment;
    eq.value = encodeEquipment({refOf(0x8ED04), refOf(0x1234)});
    v.push_back(eq);
    return v;
}

// ---- enemy-anim: MAnimEvent (reliable owner event with an opaque payload) ----------------------------------------------------------------
std::string scnAnimEvents() {
    Rig r(3);
    r.addActor(101, 3, V(1000), false, ~0u, 30, 0);
    r.setPose(0, 8, V(0)); r.setPose(1, 3, V(1200)); r.setPose(2, 3, V(2500));
    r.run(1500);
    if (r.host().sync->ownerOf(101).owner != 1) return fail("setup: client 1 should own", r);
    std::vector<std::vector<ActorSync::AnimEventIn>> got(3);
    for (int i = 0; i < 3; ++i) r.at(i).sync->setAnimEventHandler([&got, i](const ActorSync::AnimEventIn& e) { got[static_cast<size_t>(i)].push_back(e); });
    const Bytes payload{9, 8, 7};
    if (r.host().sync->sendAnimEvent(101, payload)) return fail("the host (not the owner) sent an animation event", r);
    if (r.at(2).sync->sendAnimEvent(101, payload)) return fail("a bystander sent an animation event", r);
    if (!r.at(1).sync->sendAnimEvent(101, payload)) return fail("the owner could not send an animation event", r);
    r.run(300);
    if (got[1].size() != 0) return fail("the sender got its own event back", r);
    for (int i : {0, 2}) {
        if (got[static_cast<size_t>(i)].size() != 1) return fail("machine " + std::to_string(i) + " got " + std::to_string(got[static_cast<size_t>(i)].size()) + " copies of the event", r);
        const auto& e = got[static_cast<size_t>(i)][0];
        if (e.id != 101 || e.payload != payload || e.from != 1) return fail("event content wrong on machine " + std::to_string(i), r);
    }
    // a duplicate delivery of the same datagram is applied once
    const size_t before = r.net.captured.size();
    for (size_t k = 0; k < before; ++k) {
        const NetMsg& m = r.net.captured[k];
        Envelope env;
        if (m.to == 2 && m.data.size() > 2 && m.data[1] == SubEnvelope && decodeEnvelope(m.data.data() + 2, m.data.size() - 2, env) && std::holds_alternative<MAnimEvent>(env.body))
            r.at(2).sync->receive(static_cast<PlayerId>(m.from), m.data.data(), m.data.size());
    }
    r.run(100);
    if (got[2].size() != 1) return fail("a replayed event was applied again (" + std::to_string(got[2].size()) + ")", r);
    // a forged event from a bystander (src 2, owner is 1) is judged and dropped by the host: nobody else sees it
    MAnimEvent forged; forged.id = 101; forged.epoch = r.host().sync->ownerOf(101).epoch; forged.eventId = (2ull << 48) | 77; forged.payload = Bytes{6};
    Envelope fe; fe.src = 2; fe.dest = kNoPlayer; fe.body = forged;
    r.net.send(2, 0, true, encodeEnvelope(fe));
    r.run(300);
    if (got[0].size() != 1 || got[1].size() != 0) return fail("a forged event from a non-owner was delivered", r);
    if (r.host().sync->stats().animEventsRejected == 0) return fail("the forged event was not counted as rejected", r);
    // an old epoch from the real owner is dropped too (stale)
    MAnimEvent stale; stale.id = 101; stale.epoch = 0; stale.eventId = (1ull << 48) | 999; stale.payload = Bytes{5};
    Envelope se; se.src = 1; se.dest = kNoPlayer; se.body = stale;
    r.net.send(1, 0, true, encodeEnvelope(se));
    r.run(300);
    if (got[0].size() != 1 || got[2].size() != 1) return fail("an event of an old epoch was delivered", r);
    return r.errors.empty() ? "" : r.errors[0];
}

const PartStats& pst(const Machine& m, PartKind k) { return m.sync->stats().parts[partIndex(k)]; }

// ---- inventory: the owner's AI uses an item, every other copy loses it too ---------------------------------------------------------------
// corpse_loot_race, game 2026-10-22: the ganger drank a Stimpak on the owner's machine; host 2 Stimpaks, client 3, before anyone looted.
std::string scnInventory() {
    Rig r(3);
    const FormRef stim = refOf(0x15169), ammo = refOf(0x8ED03);
    r.addActor(101, 3, V(1000), false, ~0u, 30, 0);
    r.setPose(0, 8, V(0)); r.setPose(1, 3, V(1200)); r.setPose(2, 3, V(2500));
    for (auto& mc : r.m) { mc->eng.a[101].inv[stim] = 3; mc->eng.a[101].inv[ammo] = 40; }
    r.run(1500);
    if (r.host().sync->ownerOf(101).owner != 1) return fail("setup: client 1 should own", r);
    for (auto& mc : r.m) if (mc->sync->stats().parts[partIndex(PartKind::Inventory)].published != 0 || mc->eng.a[101].invWrites != 0) return fail("equal worlds sent or wrote an inventory on machine " + std::to_string(mc->slot), r);
    // 1. the owner's AI drinks a Stimpak: not at once (it must stay the same for a moment), then on every other machine
    r.at(1).eng.a[101].inv[stim] = 2;
    r.run(400);
    for (int i : {0, 2}) if (r.at(i).eng.a[101].inv[stim] != 3) return fail("the list was published before it settled (machine " + std::to_string(i) + ")", r);
    r.run(2000);
    for (int i : {0, 2}) {
        if (r.at(i).eng.a[101].inv[stim] != 2) return fail("machine " + std::to_string(i) + " still has " + std::to_string(r.at(i).eng.a[101].inv[stim]) + " Stimpaks after the owner used one", r);
        if (r.at(i).eng.a[101].inv[ammo] != 40) return fail("an item the owner did not touch changed on machine " + std::to_string(i), r);
        if (r.at(i).eng.a[101].invWrites != 1) return fail("machine " + std::to_string(i) + " wrote the inventory " + std::to_string(r.at(i).eng.a[101].invWrites) + " times (once)", r);
    }
    if (r.at(1).eng.a[101].invWrites != 0 || r.at(1).eng.a[101].inv[stim] != 2) return fail("the owner's own inventory was written", r);
    if (r.at(1).sync->stats().parts[partIndex(PartKind::Inventory)].published != 1) return fail("the owner published " + std::to_string(r.at(1).sync->stats().parts[partIndex(PartKind::Inventory)].published) + " lists (one)", r);
    // 2. a replicated AddItem effect lands on the owner now and on the others 600 ms later: the list that follows must not add it a second time
    r.at(1).eng.a[101].inv[stim] += 2;
    r.run(600);
    for (int i : {0, 2}) r.at(i).eng.a[101].inv[stim] += 2;
    r.run(3000);
    for (auto& mc : r.m) if (mc->eng.a[101].inv[stim] != 4) return fail("machine " + std::to_string(mc->slot) + " has " + std::to_string(mc->eng.a[101].inv[stim]) + " Stimpaks after a replicated AddItem 2 (4)", r);
    // 3. a list from a bystander (it does not own the actor) is dropped by the host and reaches nobody
    MPart forged; forged.id = 101; forged.epoch = r.host().sync->ownerOf(101).epoch; forged.eventId = (2ull << 48) | 77; forged.kind = PartKind::Inventory;
    forged.value = encodeInventory({{stim, 99}});
    Envelope fe; fe.src = 2; fe.dest = kNoPlayer; fe.body = forged;
    r.net.send(2, 0, true, encodeEnvelope(fe));
    r.run(300);
    for (auto& mc : r.m) if (mc->eng.a[101].inv[stim] == 99) return fail("a forged inventory from a non-owner was applied on machine " + std::to_string(mc->slot), r);
    if (r.host().sync->stats().parts[partIndex(PartKind::Inventory)].rejected == 0) return fail("the forged inventory was not counted as rejected", r);
    // 4. a duplicate delivery is applied once
    const int writes = r.at(0).eng.a[101].invWrites;
    for (size_t k = 0, n = r.net.captured.size(); k < n; ++k) {
        const NetMsg& m = r.net.captured[k];
        Envelope env;
        if (m.to == 0 && m.data.size() > 2 && m.data[1] == SubEnvelope && decodeEnvelope(m.data.data() + 2, m.data.size() - 2, env) && std::holds_alternative<MPart>(env.body))
            r.at(0).sync->receive(static_cast<PlayerId>(m.from), m.data.data(), m.data.size());
    }
    r.run(100);
    if (r.at(0).eng.a[101].invWrites != writes) return fail("a replayed inventory was written again", r);
    // 5. a client that joins later hears what was used before it came
    r.at(2).eng.a[101].inv[stim] = 9;
    r.host().sync->playerJoined(2);
    r.run(500);
    if (r.at(2).eng.a[101].inv[stim] != 4) return fail("a joining client did not get the shared inventory", r);
    // 6. the last thing it used before it died counts, even inside the settle time
    r.at(1).eng.a[101].inv[stim] = 3;
    if (!r.at(1).sync->onLocalDeath(101)) return fail("the owner may not let an ordinary NPC die", r);
    r.run(300);
    for (int i : {0, 2}) if (r.at(i).eng.a[101].inv[stim] != 3) return fail("machine " + std::to_string(i) + " did not get the item used just before the death", r);
    return r.errors.empty() ? "" : r.errors[0];
}

// A list that arrives while the actor is not loaded here (or while the engine refuses the write) is kept and written when it can be.
std::string scnInventoryLate() {
    Rig r(2);
    const FormRef stim = refOf(0x15169);
    r.addActor(101, 3, V(1000), false, ~0u, 30, 0);
    r.setPose(0, 8, V(0)); r.setPose(1, 3, V(1200));
    for (auto& mc : r.m) mc->eng.a[101].inv[stim] = 3;
    r.run(1500);
    if (r.host().sync->ownerOf(101).owner != 1) return fail("setup: client 1 should own", r);
    r.host().eng.refuseInventory = true;
    r.at(1).eng.a[101].inv[stim] = 1;
    r.run(2500);
    if (r.host().eng.a[101].inv[stim] != 3) return fail("the host wrote while the engine refused", r);
    r.host().eng.refuseInventory = false;
    r.run(1500);
    if (r.host().eng.a[101].inv[stim] != 1) return fail("a refused write was not retried", r);
    return r.errors.empty() ? "" : r.errors[0];
}
// ---- equipment: the owner's AI draws a rifle, every copy holds the same one ------------------------------------------------------------
// Goodsprings settler, game 2026-10-29: he drew his rifle and shot at the host; on the guest's copy he cowered empty-handed.
std::string scnEquipment() {
    Rig r(3);
    const FormRef rifle = refOf(0x8ED04), pistol = refOf(0x1234), armor = refOf(0x500A);
    r.addActor(101, 3, V(1000), false, ~0u, 30, 0);
    r.setPose(0, 8, V(0)); r.setPose(1, 3, V(1200)); r.setPose(2, 3, V(2500));
    for (auto& mc : r.m) mc->eng.a[101].equip = {armor};
    r.run(1500);
    if (r.host().sync->ownerOf(101).owner != 1) return fail("setup: client 1 should own", r);
    for (auto& mc : r.m) if (pst(*mc, PartKind::Equipment).published != 0 || mc->eng.a[101].equipWrites != 0) return fail("equal worlds sent or wrote equipment on machine " + std::to_string(mc->slot), r);
    // 1. the owner's AI draws the rifle: not at once (it must stay the same for a moment), then on every other machine
    r.at(1).eng.a[101].equip.insert(rifle);
    r.run(200);
    for (int i : {0, 2}) if (r.at(i).eng.a[101].equip.count(rifle)) return fail("the equipment was published before it settled (machine " + std::to_string(i) + ")", r);
    r.run(1500);
    for (int i : {0, 2}) {
        const FakeActor& f = r.at(i).eng.a[101];
        if (!f.equip.count(rifle)) return fail("machine " + std::to_string(i) + " does not hold the rifle the owner drew", r);
        if (!f.equip.count(armor) || f.equip.size() != 2) return fail("machine " + std::to_string(i) + " wears " + std::to_string(f.equip.size()) + " items (the armour and the rifle)", r);
        if (f.equipWrites != 1) return fail("machine " + std::to_string(i) + " wrote the equipment " + std::to_string(f.equipWrites) + " times (once)", r);
    }
    if (r.at(1).eng.a[101].equipWrites != 0) return fail("the owner's own equipment was written", r);
    if (pst(r.at(1), PartKind::Equipment).published != 1) return fail("the owner published " + std::to_string(pst(r.at(1), PartKind::Equipment).published) + " equipment lists (one)", r);
    for (auto& mc : r.m) if (pst(*mc, PartKind::Inventory).published != 0 || mc->eng.a[101].invWrites != 0) return fail("a weapon draw moved the inventory part on machine " + std::to_string(mc->slot), r);
    {   // the counters are reported as JSON (coop.owner stats)
        std::string out; nvcoop::json::Value v; std::string err;
        if (!nvcoop::actors::ownerRpc(r.at(0).sync.get(), "{\"stats\":true}", out) || !nvcoop::json::parse(out, v, err)) return fail("stats json: " + out, r);
        if (v.get("parts").get("equipment").get("applied").asInt(-1) != 1 || v.get("parts").get("inventory").get("applied").asInt(-1) != 0) return fail("stats json has the wrong part counters: " + out, r);
    }
    // 2. an equipment list from a bystander (it does not own the actor) is dropped by the host and reaches nobody
    MPart forged; forged.id = 101; forged.epoch = r.host().sync->ownerOf(101).epoch; forged.eventId = (2ull << 48) | 77; forged.kind = PartKind::Equipment;
    forged.value = encodeEquipment({refOf(0xBAD)});
    Envelope fe; fe.src = 2; fe.dest = kNoPlayer; fe.body = forged;
    r.net.send(2, 0, true, encodeEnvelope(fe));
    r.run(400);
    for (auto& mc : r.m) if (mc->eng.a[101].equip.count(refOf(0xBAD))) return fail("a forged equipment list from a non-owner was applied on machine " + std::to_string(mc->slot), r);
    if (pst(r.host(), PartKind::Equipment).rejected == 0) return fail("the forged equipment list was not counted as rejected", r);
    // 3. a duplicate delivery is applied once
    const int writes = r.at(0).eng.a[101].equipWrites;
    for (size_t k = 0, n = r.net.captured.size(); k < n; ++k) {
        const NetMsg& m = r.net.captured[k];
        Envelope env;
        if (m.to == 0 && m.data.size() > 2 && m.data[1] == SubEnvelope && decodeEnvelope(m.data.data() + 2, m.data.size() - 2, env) && std::holds_alternative<MPart>(env.body))
            r.at(0).sync->receive(static_cast<PlayerId>(m.from), m.data.data(), m.data.size());
    }
    r.run(100);
    if (r.at(0).eng.a[101].equipWrites != writes) return fail("a replayed equipment list was written again", r);
    if (pst(r.at(0), PartKind::Equipment).duplicate == 0) return fail("the replayed equipment list was not counted as a duplicate", r);
    // 4. a client that joins later gets what the actor holds now
    r.at(2).eng.a[101].equip = {armor};
    r.host().sync->playerJoined(2);
    r.run(500);
    if (!r.at(2).eng.a[101].equip.count(rifle) || r.at(2).eng.a[101].equip.size() != 2) return fail("a joining client did not get the current equipment", r);
    // 5. the last thing it drew before it died counts, even inside the settle time
    r.at(1).eng.a[101].equip = {armor, pistol};
    if (!r.at(1).sync->onLocalDeath(101)) return fail("the owner may not let an ordinary NPC die", r);
    r.run(300);
    for (int i : {0, 2}) if (!r.at(i).eng.a[101].equip.count(pistol) || r.at(i).eng.a[101].equip.count(rifle)) return fail("machine " + std::to_string(i) + " did not get the weapon held just before the death", r);
    return r.errors.empty() ? "" : r.errors[0];
}

// What the old owner holds when it hands the actor over reaches the new owner before the final state, even inside the settle time.
std::string scnEquipmentHandoff() {
    Rig r(2);
    const FormRef rifle = refOf(0x8ED04), armor = refOf(0x500A);
    r.addActor(100, 3, V(1000), false, ~0u, 40, 0);
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(3000));
    for (auto& mc : r.m) mc->eng.a[100].equip = {armor};
    r.run(1500);
    if (r.host().sync->ownerOf(100).owner != 0) return fail("setup: host should own", r);
    r.host().eng.a[100].equip.insert(rifle);
    const uint64_t drawnAt = r.now;
    r.setPose(0, 9, V(0));   // host leaves
    bool pending = false;
    for (int i = 0; i < 300 && !pending; ++i) { r.step(10); pending = r.host().sync->service()->handoffPending(100); }
    if (!pending) return fail("no handoff was ever pending", r);
    r.run(80);
    if (r.now - drawnAt >= 300) return fail("setup: the handoff came after the equipment settled (" + std::to_string(r.now - drawnAt) + " ms)", r);
    if (!r.at(1).eng.a[100].equip.count(rifle)) return fail("the new owner did not get the weapon the old owner held at the handoff", r);
    r.run(600);
    if (r.at(1).sync->ownerOf(100).owner != 1) return fail("the client did not take over", r);
    if (!r.at(1).eng.a[100].equip.count(rifle) || pst(r.at(1), PartKind::Equipment).published != 0) return fail("the new owner changed or republished what the old owner held", r);
    return r.errors.empty() ? "" : r.errors[0];
}

// A copy that unloads and loads again comes back with the engine's base outfit: the owner's equipment is written again.
std::string scnEquipmentReload() {
    Rig r(2);
    const FormRef rifle = refOf(0x8ED04), armor = refOf(0x500A);
    r.addActor(101, 3, V(1000), false, ~0u, 30, 0);
    r.setPose(0, 8, V(0)); r.setPose(1, 3, V(1200));
    for (auto& mc : r.m) mc->eng.a[101].equip = {armor};
    r.run(1500);
    if (r.host().sync->ownerOf(101).owner != 1) return fail("setup: client 1 should own", r);
    r.at(1).eng.a[101].equip.insert(rifle);
    r.run(1500);
    if (!r.host().eng.a[101].equip.count(rifle)) return fail("setup: the copy does not wear the rifle", r);
    r.host().sync->onActorUnloaded(101);
    r.host().eng.a[101].real = false;   // the copy's cell unloads
    r.run(200);
    r.host().eng.a[101].real = true;    // and loads again: the engine dresses the NPC from its base form
    r.host().eng.a[101].equip = {armor};
    r.host().sync->onActorLoaded(101);
    r.run(1200);
    if (!r.host().eng.a[101].equip.count(rifle)) return fail("a copy that loaded again was not given the owner's weapon back", r);
    if (r.host().eng.a[101].inv.size() != 0 || r.host().eng.a[101].invWrites != 0) return fail("the inventory part was written on a reload", r);
    return r.errors.empty() ? "" : r.errors[0];
}

// An equipment list that arrives while the engine refuses the write is kept and written when it can be.
std::string scnEquipmentLate() {
    Rig r(2);
    const FormRef rifle = refOf(0x8ED04);
    r.addActor(101, 3, V(1000), false, ~0u, 30, 0);
    r.setPose(0, 8, V(0)); r.setPose(1, 3, V(1200));
    r.run(1500);
    if (r.host().sync->ownerOf(101).owner != 1) return fail("setup: client 1 should own", r);
    r.host().eng.refuseEquipment = true;
    r.at(1).eng.a[101].equip.insert(rifle);
    r.run(2500);
    if (r.host().eng.a[101].equip.count(rifle)) return fail("the host wrote while the engine refused", r);
    if (pst(r.host(), PartKind::Equipment).applyFailed == 0) return fail("a refused write was not counted", r);
    r.host().eng.refuseEquipment = false;
    r.run(1500);
    if (!r.host().eng.a[101].equip.count(rifle)) return fail("a refused write was not retried", r);
    if (r.host().eng.a[101].equipWrites != 1) return fail("the retried write landed " + std::to_string(r.host().eng.a[101].equipWrites) + " times (once)", r);
    return r.errors.empty() ? "" : r.errors[0];
}

}  // namespace

TEST(inventory, an_owners_item_use_reaches_every_copy_once) { checkEmpty(scnInventory()); }
TEST(inventory, a_refused_write_is_retried) { checkEmpty(scnInventoryLate()); }
TEST(equipment, an_owners_weapon_draw_reaches_every_copy_once) { checkEmpty(scnEquipment()); }
TEST(equipment, what_the_old_owner_holds_reaches_the_new_owner) { checkEmpty(scnEquipmentHandoff()); }
TEST(equipment, a_refused_write_is_retried) { checkEmpty(scnEquipmentLate()); }
TEST(equipment, a_copy_that_loads_again_is_dressed_again) { checkEmpty(scnEquipmentReload()); }

TEST(parts, codecs_roundtrip_and_reject_garbage) {
    InvList inv{{refOf(0x20), 5}, {refOf(0x10), 1}}, back;
    const Bytes ib = encodeInventory(inv);
    CHECK(decodeInventory(ib, back) && back.size() == 2 && back[0].form == refOf(0x10) && back[1].count == 5);
    CHECK(encodeInventory({{refOf(0x10), 1}, {refOf(0x20), 5}}) == ib);   // sorted: the same state is the same bytes
    EquipList eq{refOf(0x20), refOf(0x10), refOf(0x20)}, eback;
    const Bytes eb = encodeEquipment(eq);
    CHECK(decodeEquipment(eb, eback) && eback.size() == 2 && eback[0] == refOf(0x10));   // sorted, no duplicates
    CHECK(encodeEquipment({refOf(0x10), refOf(0x20)}) == eb);
    CHECK(decodeEquipment(encodeEquipment({}), eback) && eback.empty());
    Bytes trailing = eb; trailing.push_back(0);
    CHECK(!decodeEquipment(trailing, eback) && !decodeInventory(trailing, back));
    CHECK(!decodeEquipment(Bytes(eb.begin(), eb.end() - 1), eback));
    CHECK(!decodeEquipment(Bytes{}, eback));
    CHECK(std::string(partName(PartKind::Inventory)) == "inventory" && std::string(partName(PartKind::Equipment)) == "equipment");
}

TEST(anim, owner_events_reach_everyone_once_and_forgeries_do_not) { checkEmpty(scnAnimEvents()); }

TEST(wire, roundtrip_every_message) {
    for (const Body& b : sampleBodies()) {
        Envelope e; e.src = 2; e.dest = kNoPlayer; e.body = b;
        const Bytes a = encodeEnvelope(e);
        REQUIRE(a.size() > 2 && a[0] == kChannelActors && a[1] == SubEnvelope);
        Envelope back;
        CHECK_MSG(decodeEnvelope(a.data() + 2, a.size() - 2, back), bodyName(b));
        CHECK(back.src == 2 && back.dest == kNoPlayer && back.body.index() == b.index());
        CHECK_MSG(encodeEnvelope(back) == a, bodyName(b));   // decode(encode(x)) re-encodes to the same bytes
    }
}

TEST(wire, malformed_input_never_crashes) {
    fnvcoop::Rng rng(11);
    std::vector<Bytes> seeds;
    for (const Body& b : sampleBodies()) { Envelope e; e.src = 1; e.body = b; seeds.push_back(encodeEnvelope(e)); }
    live::ActorSnapshot s = craftSnap(7, 3, 4, 5, V(1, 2, 3));
    seeds.push_back(craftBatch(1, s));
    Rig r(2);
    r.checkInvariants = false;   // forged host messages may legitimately confuse a client: only "no crash" is asserted here
    r.addActor(7, 3, V(0));
    int accepted = 0;
    for (int i = 0; i < 4000; ++i) {
        Bytes d = seeds[rng.range(0, static_cast<uint32_t>(seeds.size() - 1))];
        const int muts = static_cast<int>(rng.range(0, 4));
        for (int k = 0; k < muts && !d.empty(); ++k) d[rng.range(0, static_cast<uint32_t>(d.size() - 1))] ^= static_cast<uint8_t>(1u << rng.range(0, 7));
        if (rng.chance(0.3) && !d.empty()) d.resize(rng.range(0, static_cast<uint32_t>(d.size())));
        if (rng.chance(0.05)) { d.assign(rng.range(0, 40), 0); for (auto& b : d) b = static_cast<uint8_t>(rng.u32()); }
        Envelope e;
        if (d.size() > 2 && decodeEnvelope(d.data() + 2, d.size() - 2, e)) ++accepted;
        r.host().sync->receive(1, d.data(), d.size());
        r.at(1).sync->receive(0, d.data(), d.size());
        if (i % 50 == 0) r.step(10);
    }
    CHECK(accepted > 0);
    CHECK(r.host().sync->stats().undecodable > 0);
    r.run(1000);   // still alive
}

TEST(wire, spoofed_sources_are_ignored) {
    Rig r(3);
    r.addActor(100, 3, V(0));
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(100)); r.setPose(2, 3, V(200));
    r.run(1000);
    const OwnerInfo before = r.host().sync->ownerOf(100);
    fnvcoop::OwnMsg g; g.kind = fnvcoop::OwnMsg::Grant; g.netId = 100; g.epoch = before.epoch + 5; g.owner = 2;
    inject(r, 0, 1, envelopeBytes(1, kHostPlayer, MOwn{g}));            // a client may only Ack
    inject(r, 0, 1, envelopeBytes(2, kHostPlayer, MResync{100}));       // claims to be client 2
    inject(r, 1, 2, envelopeBytes(0, kNoPlayer, MOwn{g}));              // a client posing as the host
    CHECK(r.host().sync->ownerOf(100).owner == before.owner && r.host().sync->ownerOf(100).epoch == before.epoch);
    CHECK(r.at(1).sync->ownerOf(100).owner == before.owner);
    CHECK(r.host().sync->stats().spoofed >= 2 && r.at(1).sync->stats().spoofed >= 1);
}

TEST(wire, net_ids) {
    const NetId a = deriveNetId(FormRef("FalloutNV.esm", 0x104C1C)), b = deriveNetId(FormRef("falloutnv.esm", 0x104C1C));
    CHECK(a == b && a != 0);
    CHECK(deriveNetId(FormRef("falloutnv.esm", 1)) != deriveNetId(FormRef("deadmoney.esm", 1)));
    int bad = 0;
    for (uint32_t i = 1; i < 200000; ++i) { const NetId n = deriveNetId(FormRef("falloutnv.esm", i)); if (n == 0 || (n >> 24) == 0xFF || (n >> 24) == 0xFE) ++bad; }   // 0xFE.. players, 0xFF.. host-issued (twin 43)
    CHECK(bad == 0);
}

// ---- churn: many seeds, lossy net, players roaming; invariants every step, agreement at the end -----------------------------------
namespace {
std::string churn(uint64_t seed, int machines, int actors, uint32_t runMs, bool leaves = false) {
    NetOpts o;
    o.latencyMs = 15; o.jitterMs = 40; o.lossUnreliable = 0.10; o.dupUnreliable = 0.05;
    Rig r(machines, seed, o);
    fnvcoop::Rng rng(seed * 31 + 7);
    for (int i = 0; i < actors; ++i)
        r.addActor(1000 + i, static_cast<CellId>(2 + i % 6), V(500.0f + 300.0f * i), false, ~0u, rng.chance(0.5) ? 25.0f : -25.0f, 0);
    for (int p = 0; p < machines; ++p) r.setPose(p, static_cast<CellId>(2 + rng.range(0, 6)), V(100.0f * p));
    r.run(1200);
    uint64_t nextMove[8] = {}, nextToggle[8] = {};
    bool gone[8] = {};
    auto disconnect = [&](int p) {
        r.net.down.insert(p);
        r.host().sync->playerLeft(static_cast<PlayerId>(p));
        r.at(p).sync->hostLost();
        gone[p] = true;
    };
    auto reconnect = [&](int p) {
        r.net.down.erase(p);
        r.host().sync->playerJoined(static_cast<PlayerId>(p));
        r.at(p).sync->hostConnected();
        gone[p] = false;
    };
    for (uint32_t t = 0; t < runMs; t += 10) {
        if (leaves)
            for (int p = 1; p < machines; ++p)
                if (r.now >= nextToggle[p]) {
                    nextToggle[p] = r.now + 1500 + rng.range(0, 3500);
                    if (gone[p]) reconnect(p); else disconnect(p);
                }
        for (int p = 0; p < machines; ++p)
            if (r.now >= nextMove[p] && !gone[p]) {
                nextMove[p] = r.now + 400 + rng.range(0, 1800);
                r.setPose(p, static_cast<CellId>(2 + rng.range(0, 6)), V(static_cast<float>(rng.range(0, 3000))));
            }
        r.step(10);
        if (!r.errors.empty()) return "seed " + std::to_string(seed) + ": " + r.errors[0];
    }
    // quiet: everybody is back, no more movement, clean net
    for (int p = 1; p < machines; ++p) if (gone[p]) reconnect(p);
    r.net.o.lossUnreliable = 0; r.net.o.dupUnreliable = 0; r.net.o.jitterMs = 0;
    for (auto& mc : r.m) for (auto& kv : mc->eng.a) kv.second.vx = kv.second.vy = 0;
    r.run(4000);
    if (!r.errors.empty()) return "seed " + std::to_string(seed) + ": " + r.errors[0];
    for (int i = 0; i < actors; ++i) {
        const NetId id = 1000 + i;
        const std::string ag = r.agreement(id);
        if (!ag.empty()) return "seed " + std::to_string(seed) + ": actor " + std::to_string(id) + ": " + ag;
        const OwnerInfo h = r.host().sync->ownerOf(id);
        if (h.owner == kNoPlayer) { if (r.aiCount(id) != 0) return "seed " + std::to_string(seed) + ": unowned actor has AI running"; continue; }
        if (r.aiMachine(id) != h.owner || r.aiCount(id) != 1) return "seed " + std::to_string(seed) + ": actor " + std::to_string(id) + " AI is not exactly on owner " + std::to_string(h.owner) + " (on " + std::to_string(r.aiMachine(id)) + ", count " + std::to_string(r.aiCount(id)) + ")";
        const live::Vec3 truth = r.at(h.owner).eng.a[id].pos;
        for (int j = 0; j < machines; ++j) {
            if (dist2d(r.at(j).eng.a[id].pos, truth) > 60) return "seed " + std::to_string(seed) + ": actor " + std::to_string(id) + " on machine " + std::to_string(j) + " is " + std::to_string(dist2d(r.at(j).eng.a[id].pos, truth)) + " units from the owner";
            if (r.at(j).eng.a[id].standIn) return "seed " + std::to_string(seed) + ": unexpected stand-in";
        }
    }
    uint64_t dg = r.host().sync->ownershipDigest();
    for (int j = 1; j < machines; ++j) if (r.at(j).sync->ownershipDigest() != dg) return "seed " + std::to_string(seed) + ": ownership digests differ";
    return "";
}
}  // namespace

TEST(churn, players_leave_and_rejoin) {
    for (uint64_t seed = 101; seed <= 108; ++seed) {
        const std::string e = churn(seed, 3 + static_cast<int>(seed % 2), 4, 25000, true);
        CHECK_MSG(e.empty(), e);
    }
}

TEST(churn, roaming_players_lossy_net) {
    for (uint64_t seed = 1; seed <= 12; ++seed) {
        const std::string e = churn(seed, 2 + static_cast<int>(seed % 3), 5, 20000);
        CHECK_MSG(e.empty(), e);
    }
}

// ---- ActorRuntime over the real MuxTransport ------------------------------------------------------------------------------------------
TEST(runtime, mux_session_roundtrip) {
    fnvcoop::InMemoryNetwork net(3);
    FakeEngine hostEng, cliEng;
    fnvcoop::ITransport* hostEnd = net.endpoint(0);
    fnvcoop::ITransport* cliEnd = net.endpoint(1);
    fnvcoop::MuxTransport hostMux(*hostEnd), cliMux(*cliEnd);
    ActorRuntime hostRt(hostEng), cliRt(cliEng);
    hostRt.config().adjacent = cliRt.config().adjacent = [](CellId a, CellId b) { return a == b; };
    uint64_t now = 1000;
    bool joined = true;
    auto frame = [&](uint64_t dt) {
        now += dt;
        net.advance(now);
        hostEng.simulate(static_cast<uint32_t>(dt)); cliEng.simulate(static_cast<uint32_t>(dt));
        fnvcoop::PeerId from; Bytes d;
        while (hostMux.poll(from, d)) {}
        while (cliMux.poll(from, d)) {}
        FrameInfo hf;
        hf.active = true; hf.isHost = true; hf.self = 0; hf.mux = &hostMux; hf.nowMs = now;
        if (joined) { ClientLink l; l.slot = 1; l.peer = 1; l.joined = true; hf.clients.push_back(l); }
        hostRt.frame(hf);
        FrameInfo cf;
        cf.active = true; cf.isHost = false; cf.self = 1; cf.hostLinkUp = joined; cf.mux = &cliMux; cf.nowMs = now;
        cliRt.frame(cf);
    };
    frame(10);
    REQUIRE(hostRt.sync() && cliRt.sync());
    ActorSync* hs = hostRt.sync();
    ActorSync* cs = cliRt.sync();
    hostEng.addReal(100, 3, V(1000), refOf(0x500), 30, 0);
    cliEng.addReal(100, 3, V(1000), refOf(0x500), 30, 0);
    ActorMeta meta; meta.netId = 100; meta.ref = refOf(100); meta.base = refOf(0x500); meta.cell = 3; meta.pos = V(1000);
    hs->track(meta); cs->track(meta);
    PlayerPose p0, p1;
    p0.present = p1.present = true; p0.cell = p1.cell = 3; p0.pos = V(0); p1.pos = V(2000);
    for (int i = 0; i < 150; ++i) {
        for (ActorSync* s : {hs, cs}) { s->setPlayerPose(0, p0); s->setPlayerPose(1, p1); }
        frame(10);
    }
    CHECK(hs->ownerOf(100).owner == 0 && cs->ownerOf(100).owner == 0);
    CHECK(hs->ownerOf(100).epoch == cs->ownerOf(100).epoch && hs->ownerOf(100).epoch >= 1);
    CHECK(hostEng.a[100].ai && !cliEng.a[100].ai);
    CHECK_MSG(std::fabs(hostEng.a[100].pos.x - cliEng.a[100].pos.x) < 40 && hostEng.a[100].pos.x > 1010, "host x=" + std::to_string(hostEng.a[100].pos.x) + " client x=" + std::to_string(cliEng.a[100].pos.x) + " hostAi=" + std::to_string(hostEng.a[100].ai) + " owner=" + std::to_string((int)hs->ownerOf(100).owner));
    CHECK(cs->stats().published == 0 && hs->stats().published > 0);
    // a datagram from a peer that is not a joined client is dropped
    {
        fnvcoop::ITransport* stranger = net.endpoint(9);
        stranger->send(0, Bytes{kChannelActors, SubEnvelope, 1, 2, 3});
        frame(30);
        CHECK(hostRt.droppedUnknownPeer() >= 1);
    }
    // handoff over the real mux
    p0.cell = 9;
    for (int i = 0; i < 250; ++i) {
        for (ActorSync* s : {hs, cs}) { s->setPlayerPose(0, p0); s->setPlayerPose(1, p1); }
        frame(10);
    }
    CHECK(hs->ownerOf(100).owner == 1 && cs->ownerOf(100).owner == 1);
    CHECK(cliEng.a[100].ai && !hostEng.a[100].ai);
    CHECK(cliEng.count("place", 100) == 1 && cliEng.count("spawn", 100) == 0);
    // the link drops: the client forgets ownership; the host re-decides
    joined = false;
    for (int i = 0; i < 100; ++i) {
        hs->setPlayerPose(0, p0);
        frame(10);
    }
    CHECK(cliRt.sync() == nullptr || cliRt.sync()->ownerOf(100).owner == kNoPlayer);
    CHECK(hs->ownerOf(100).owner != 1);
    // the session ends
    {
        FrameInfo off;
        hostRt.frame(off);
        CHECK(hostRt.sync() == nullptr);
    }
}


// ---- player entities (docs/entity-core-plan.md S3): pinned records on every machine, no NPC work on them, MUntrack, early snapshots ----
// Twins NVA_MUT 44 (owned like an NPC), 45 (no MUntrack), 46 (NPC takeover on a player), 47 (early snapshot lost), 48 (NPC tick on a player).
TEST(players, every_machine_agrees_a_player_owns_its_entity_and_nothing_npc_runs_on_it) {
    Rig r(3);
    r.addActor(100, 3, V(0));
    for (int m = 0; m < 3; ++m)   // the engine has the local player and both puppets loaded: NPC code must still leave them alone
        for (PlayerId s = 0; s < 3; ++s) r.at(m).eng.addReal(live::playerNet(s), 3, V(50.0f * s), FormRef("falloutnv.esm", 0x7));
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(5)); r.setPose(2, 3, V(10));
    r.run(1500);
    for (int step = 0; step < 300; ++step) {   // players roam right past each other's entities
        for (int s = 0; s < 3; ++s) r.setPose(s, 3 + static_cast<CellId>((step + s) % 3), V(static_cast<float>((step * 37 + s * 11) % 400)));
        r.run(20);
    }
    for (PlayerId s = 0; s < 3; ++s) {
        const NetId id = live::playerNet(s);
        const OwnerInfo h = r.host().sync->ownerOf(id);
        CHECK_MSG(h.known && h.owner == s && !h.pending, "slot " + std::to_string(s));
        CHECK(r.host().sync->service()->pinned(id));
        for (int m = 1; m < 3; ++m) {
            const OwnerInfo o = r.at(m).sync->ownerOf(id);
            CHECK_MSG(o.known && o.owner == s && o.epoch == h.epoch, "machine " + std::to_string(m) + " slot " + std::to_string(s));
        }
        for (int m = 0; m < 3; ++m) CHECK_MSG(r.at(m).eng.opsFor(id).empty(), "the engine was told something about player " + std::to_string(s));
    }
    CHECK(r.host().sync->service()->stats().transfers == 0 || r.host().sync->ownerOf(100).known);   // NPC handoffs are fine; players never moved (above)
    CHECK(r.host().sync->stats().publishedNotOwner == 0);
}

TEST(players, a_leaving_player_is_untracked_everywhere_and_a_rejoin_starts_above_the_old_epoch) {
    Rig r(3);
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(5)); r.setPose(2, 3, V(10));
    r.run(1000);
    const NetId p1 = live::playerNet(1);
    const OwnerEpoch before = r.host().sync->ownerOf(p1).epoch;
    REQUIRE(r.at(2).sync->tracked(p1) && before >= 1);
    r.host().sync->playerLeft(1);
    r.setPose(1, 3, V(5), false);
    r.run(500);
    CHECK(!r.host().sync->tracked(p1));
    CHECK(!r.at(2).sync->tracked(p1));
    CHECK(r.at(2).sync->stats().playersUntracked >= 1);
    r.host().sync->playerJoined(1);
    r.at(1).sync->hostConnected();
    r.setPose(1, 3, V(5));
    r.run(1000);
    const OwnerInfo h = r.host().sync->ownerOf(p1);
    CHECK(h.known && h.owner == 1 && h.epoch > before);
    CHECK(r.at(2).sync->ownerOf(p1).epoch == h.epoch && r.at(1).sync->ownerOf(p1).epoch == h.epoch);
}

TEST(players, nobody_can_track_or_claim_a_player_entity) {
    Rig r(2);
    r.run(500);
    ActorMeta m;
    m.netId = live::playerNet(1); m.ref = live::playerFormRef(1);
    CHECK(!r.host().sync->track(m));                          // an engine feed never tracks a player
    ActorMeta forged;
    forged.netId = 0x00ABCDEF; forged.ref = live::playerFormRef(0);
    inject(r, 0, 1, envelopeBytes(1, kNoPlayer, MTrack{m}));      // a client claiming a player entity
    inject(r, 0, 1, envelopeBytes(1, kNoPlayer, MTrack{forged}));
    CHECK(r.host().sync->stats().playerRefused >= 3);
    CHECK(r.host().sync->ownerOf(live::playerNet(1)).owner == 1);
    CHECK(!r.host().sync->tracked(0x00ABCDEF));
}

TEST(players, a_snapshot_that_arrives_before_its_record_and_ownership_is_applied_once_they_arrive) {
    Rig r(2);
    r.run(500);
    const NetId id = 555;
    inject(r, 1, 0, craftBatch(1, craftSnap(id, 1, 7, 3, V(321))));   // the host's stream overtook its reliable messages
    CHECK(r.at(1).sync->stats().unknownActor >= 1);
    ActorMeta m;
    m.netId = id; m.ref = refOf(id); m.cell = 3;
    inject(r, 1, 0, envelopeBytes(0, kNoPlayer, MTrack{m}));
    fnvcoop::OwnMsg g; g.kind = fnvcoop::OwnMsg::Grant; g.netId = id; g.epoch = 1; g.owner = 0;
    inject(r, 1, 0, envelopeBytes(0, kNoPlayer, MOwn{g}));
    live::ActorSnapshot got;
    CHECK(r.at(1).sync->latestState(id, got) && closeTo(got.pos.x, 321, 1));
    CHECK(r.at(1).sync->stats().earlyApplied == 1);
}

// ---- the event bus (docs/entity-core-plan.md S6). Twins NVA_MUT 80 (anyone sends owner events), 81 (duplicates applied), 82 (no Nack),
// 83 (no re-route from a past owner), 84 (an event ahead of its record lost), 85 (the host believes a non-owner).
namespace {
struct BusLog { std::vector<ActorSync::EventIn> got; };
void listen(Rig& r, std::vector<BusLog>& logs, uint8_t kind) {
    logs.resize(static_cast<size_t>(r.n()));
    for (int i = 0; i < r.n(); ++i) {
        BusLog* l = &logs[static_cast<size_t>(i)];
        r.at(i).sync->setEventHandler(kind, [l](const ActorSync::EventIn& in) { l->got.push_back(in); });
    }
}
}  // namespace

TEST(bus, an_owner_event_reaches_everyone_once_and_nobody_else_may_send_one) {
    Rig r(3);
    r.addActor(100, 3, V(0));
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(3000)); r.setPose(2, 3, V(3000));
    r.run(1000);
    REQUIRE(r.host().sync->ownerOf(100).owner == 0);
    std::vector<BusLog> logs;
    listen(r, logs, 7);
    CHECK(r.host().sync->sendOwnerEvent(100, 7, Bytes{1, 2, 3}));
    CHECK(!r.at(1).sync->sendOwnerEvent(100, 7, Bytes{9}));   // not the owner
    r.run(300);
    CHECK(logs[1].got.size() == 1 && logs[2].got.size() == 1 && logs[1].got[0].payload == (Bytes{1, 2, 3}));
    // a client forging an owner event: the host refuses it, nobody else hears it
    MEvent forged;
    forged.id = 100; forged.epoch = r.host().sync->ownerOf(100).epoch; forged.eventId = (uint64_t(1) << 48) | 99; forged.kind = 7; forged.origin = 1; forged.payload = {6};
    inject(r, 0, 1, envelopeBytes(1, kNoPlayer, forged));
    r.run(300);
    CHECK(logs[0].got.empty() && logs[2].got.size() == 1);
    CHECK(r.host().sync->stats().eventsRejected >= 1);
    // the same owner event delivered twice: applied once
    MEvent dup;
    dup.id = 100; dup.epoch = r.host().sync->ownerOf(100).epoch; dup.eventId = 777; dup.kind = 7; dup.origin = 0; dup.payload = {5};
    inject(r, 1, 0, envelopeBytes(0, kNoPlayer, dup));
    inject(r, 1, 0, envelopeBytes(0, kNoPlayer, dup));
    CHECK(logs[1].got.size() == 2);
}

TEST(bus, a_request_reaches_the_current_owner_and_one_that_reached_a_past_owner_is_re_routed) {
    Rig r(3);
    r.addActor(101, 3, V(1000));
    r.setPose(0, 9, V(0)); r.setPose(1, 3, V(5000)); r.setPose(2, 3, V(1000));   // client 2 is next to it
    r.run(1500);
    REQUIRE(r.host().sync->ownerOf(101).owner == 2);
    std::vector<BusLog> logs;
    listen(r, logs, 8);
    CHECK(r.at(1).sync->sendToOwner(101, 8, Bytes{4}));
    r.run(300);
    REQUIRE(logs[2].got.size() == 1);
    CHECK(logs[2].got[0].origin == 1 && logs[2].got[0].toOwner && logs[0].got.empty() && logs[1].got.empty());
    // a request the host forwarded to client 1, who does not own it (its table moved on): client 1 hands it back, the owner gets it
    MEvent stale;
    stale.id = 101; stale.eventId = (uint64_t(0) << 48) | 4242; stale.kind = 8; stale.route = EventRoute::ToOwner; stale.origin = 0; stale.payload = {2};
    inject(r, 1, 0, envelopeBytes(0, 1, stale));
    r.run(300);
    CHECK(logs[2].got.size() == 2);
    CHECK(r.host().sync->stats().eventsRerouted >= 1);
}

TEST(bus, a_request_for_an_entity_nobody_owns_is_answered_with_a_nack) {
    Rig r(2);
    r.addActor(102, 50, V(0));   // far from everyone: nobody owns it
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(10));
    r.run(1000);
    REQUIRE(r.host().sync->ownerOf(102).owner == kNoPlayer);
    std::vector<BusLog> logs;
    listen(r, logs, 9);
    CHECK(r.at(1).sync->sendToOwner(102, 9, Bytes{1}));
    r.run(300);
    CHECK(logs[1].got.size() == 1 && logs[1].got[0].nack);
}

TEST(bus, an_event_ahead_of_its_record_is_held_and_delivered) {
    Rig r(2);
    r.run(500);
    std::vector<BusLog> logs;
    listen(r, logs, 10);
    MEvent m;
    m.id = 556; m.epoch = 1; m.eventId = 31337; m.kind = 10; m.origin = 0; m.payload = {8};
    inject(r, 1, 0, envelopeBytes(0, kNoPlayer, m));
    CHECK(logs[1].got.empty());
    ActorMeta meta;
    meta.netId = 556; meta.ref = refOf(556); meta.cell = 3;
    inject(r, 1, 0, envelopeBytes(0, kNoPlayer, MTrack{meta}));
    fnvcoop::OwnMsg g; g.kind = fnvcoop::OwnMsg::Grant; g.netId = 556; g.epoch = 1; g.owner = 0;
    inject(r, 1, 0, envelopeBytes(0, kNoPlayer, MOwn{g}));
    r.at(1).sync->tick(r.now + 10);
    CHECK(logs[1].got.size() == 1);
}

// ---- NPC byte identity (docs/entity-core-plan.md S5, S10) ----------------------------------------------------------------------------------
// The player parts and the player stream profile are additions: not one byte of what the NPC path sends may change. A fixed NPC scenario (walkers, a handoff,
// an inventory and an equipment change, a death, on a lossy and duplicating link) is replayed and every datagram of every machine is folded into one hash;
// the constant was taken from ActorSync as it was before S5. NVA_GOLDEN_PRINT=1 prints the value of the current code instead of checking it.
namespace {
uint64_t npcTrafficHash(uint64_t* datagrams) {
    Rig r(3, 11, NetOpts{20, 8, 0.03, 0.03});
    uint64_t h = 1469598103934665603ull, n = 0;
    auto mix = [&](const void* p, size_t len) {
        const uint8_t* b = static_cast<const uint8_t*>(p);
        for (size_t i = 0; i < len; ++i) { h ^= b[i]; h *= 1099511628211ull; }
    };
    r.net.dropIf = [&](const NetMsg& m) {
        ++n;
        const uint64_t head[4] = {r.net.now, static_cast<uint64_t>(m.from), static_cast<uint64_t>(m.to), static_cast<uint64_t>(m.reliable)};
        mix(head, sizeof head);
        mix(m.data.data(), m.data.size());
        return false;
    };
    const FormRef stim = refOf(0x15169), rifle = refOf(0x1B2C4), armor = refOf(0x2C3D5);
    r.addActor(100, 3, V(1000), false, ~0u, 40, 0);
    r.addActor(101, 3, V(1500), false, ~0u, 0, 30);
    r.addActor(102, 4, V(6500), true, ~0u, -20, 10);
    r.setPose(0, 3, V(0)); r.setPose(1, 3, V(3000)); r.setPose(2, 4, V(6000));
    for (auto& mc : r.m) { mc->eng.a[101].inv[stim] = 3; mc->eng.a[102].equip.insert(armor); }
    r.run(1500);
    const int o101 = r.aiMachine(101);
    if (o101 >= 0) { r.at(o101).eng.a[101].inv[stim] = 2; r.at(o101).eng.a[101].equip.insert(rifle); }
    r.run(1500);
    r.setPose(1, 9, V(100));   // client 1 leaves the area: ownership moves
    r.run(1500);
    r.setPose(1, 3, V(1200));
    r.run(1200);
    const int o100 = r.aiMachine(100);
    if (o100 >= 0) r.at(o100).sync->onLocalDeath(100);
    r.run(1200);
    if (datagrams) *datagrams = n;
    return h;
}
}  // namespace

TEST(golden, npc_traffic_is_byte_identical_to_the_version_before_the_player_parts_and_the_player_stream) {
    uint64_t n = 0;
    const uint64_t h = npcTrafficHash(&n);
    if (const char* p = std::getenv("NVA_GOLDEN_PRINT")) { (void)p; std::printf("NPC-TRAFFIC datagrams=%llu hash=%llu\n", static_cast<unsigned long long>(n), static_cast<unsigned long long>(h)); }
    CHECK(n > 200);
    CHECK_MSG(h == 9335289451351416134ull, "datagrams=" + std::to_string(n) + " hash=" + std::to_string(h));
}

// ---- player parts (docs/entity-core-plan.md S5): Look, Equipment, WeaponState everywhere, Facts and Pack to the host only ----
// Twins NVA_MUT 49 (player calls reach the NPC port), 50 (silent baseline), 51 (host-only parts relayed), 52 (join state sends them), 53 (a non-owner's player
// part believed), 54 (respawned puppet not dressed again), 55 (join state without player parts), 56 (an NPC record takes a player part), 57 (no republish).
namespace {
Bytes pv(const std::string& s) { return Bytes(s.begin(), s.end()); }
const PartKind kAllPlayerKinds[] = {PartKind::Look, PartKind::Equipment, PartKind::WeaponState, PartKind::Facts, PartKind::Pack};
const PartKind kSharedKinds[] = {PartKind::Look, PartKind::Equipment, PartKind::WeaponState};
const PartKind kHostOnlyKinds[] = {PartKind::Facts, PartKind::Pack};

std::string valueOf(PlayerId slot, PartKind k) { return std::string(partName(k)) + "-" + std::to_string(slot); }

// every machine's avatar layer is up and has a puppet of every other player
void playersUp(Rig& r) {
    for (int m = 0; m < r.n(); ++m) {
        FakePlayerPort& pp = r.at(m).pp;
        pp.up = true;
        for (int s = 0; s < r.n(); ++s) if (s != m) pp.puppets.insert(static_cast<PlayerId>(s));
    }
}
void setLocalParts(Rig& r, int m) {
    for (PartKind k : kAllPlayerKinds) r.at(m).pp.local[k] = pv(valueOf(static_cast<PlayerId>(m), k));
}
}  // namespace

TEST(playerparts, the_first_read_is_published_and_every_puppet_gets_look_equipment_and_weapon) {
    Rig r(3);
    playersUp(r);
    for (int m = 0; m < 3; ++m) setLocalParts(r, m);
    r.run(2500);
    for (int viewer = 0; viewer < 3; ++viewer)
        for (int owner = 0; owner < 3; ++owner) {
            if (viewer == owner) continue;
            for (PartKind k : kSharedKinds)
                CHECK_MSG(r.at(viewer).pp.lastWritten(static_cast<PlayerId>(owner), k) == valueOf(static_cast<PlayerId>(owner), k),
                          std::string("machine ") + std::to_string(viewer) + " never dressed player " + std::to_string(owner) + " with its " + partName(k));
        }
    for (int m = 0; m < 3; ++m) {
        CHECK(r.at(m).pp.wrong == 0);
        for (PartKind k : kSharedKinds) CHECK(r.at(m).sync->stats().parts[partIndex(k)].published == 1);
        // the local player is never written onto
        CHECK(r.at(m).pp.written.count(static_cast<PlayerId>(m)) == 0);
        // and no NPC call was made for a player
        for (PlayerId s = 0; s < 3; ++s) CHECK(r.at(m).eng.opsFor(live::playerNet(s)).empty());
    }
    // a change is published once it stayed the same for the settle time, and reaches both others
    r.at(1).pp.local[PartKind::Equipment] = pv("equipment-1-b");
    r.run(100);
    CHECK(r.at(0).pp.lastWritten(1, PartKind::Equipment) == "equipment-1");   // not yet: 0.2 s settle
    r.run(1500);
    for (int viewer : {0, 2}) CHECK(r.at(viewer).pp.lastWritten(1, PartKind::Equipment) == "equipment-1-b");
    CHECK(r.at(0).pp.writeCount(1, PartKind::Equipment) == 2 && r.at(2).pp.writeCount(1, PartKind::Equipment) == 2);
    CHECK(r.at(1).sync->stats().parts[partIndex(PartKind::Equipment)].published == 2);
}

TEST(playerparts, facts_and_pack_are_stored_on_the_host_and_never_reach_another_guest) {
    Rig r(3);
    playersUp(r);
    for (int m = 0; m < 3; ++m) setLocalParts(r, m);
    r.run(2500);
    for (PartKind k : kHostOnlyKinds) {
        for (PlayerId s : std::initializer_list<PlayerId>{1, 2}) CHECK_MSG(r.at(0).pp.lastWritten(s, k) == valueOf(s, k), std::string("the host never got ") + partName(k) + " of " + std::to_string(s));
        Bytes stored;
        CHECK(r.host().sync->partValue(live::playerNet(1), k, stored) && stored == pv(valueOf(1, k)));
        for (int g : {1, 2}) {
            for (PlayerId s = 0; s < 3; ++s) CHECK(r.at(g).pp.writeCount(s, k) == 0);   // no guest hears anything of another's facts or pack
            CHECK(r.at(g).sync->stats().parts[partIndex(k)].applied == 0);
        }
        CHECK(r.at(0).pp.writeCount(0, k) == 0);   // the host's own are not written onto itself
    }
    // on the wire: no host-only part ever travels to a guest
    for (const NetMsg& m : r.net.captured) {
        Envelope env;
        if (m.data.size() > 2 && m.data[1] == SubEnvelope && decodeEnvelope(m.data.data() + 2, m.data.size() - 2, env))
            if (const MPart* p = std::get_if<MPart>(&env.body))
                if (p->kind == PartKind::Facts || p->kind == PartKind::Pack) CHECK_MSG(m.to == 0, "a host-only part was sent to machine " + std::to_string(m.to));
    }
    // a changed fact reaches the host once
    r.at(2).pp.local[PartKind::Facts] = pv("facts-2-b");
    r.run(1500);
    CHECK(r.at(0).pp.lastWritten(2, PartKind::Facts) == "facts-2-b" && r.at(0).pp.writeCount(2, PartKind::Facts) == 2);
    CHECK(r.at(1).pp.writeCount(2, PartKind::Facts) == 0);
}

TEST(playerparts, a_guest_cannot_publish_another_players_part) {
    Rig r(3);
    playersUp(r);
    for (int m = 0; m < 3; ++m) setLocalParts(r, m);
    r.run(2000);
    const OwnerEpoch ep = r.host().sync->ownerOf(live::playerNet(1)).epoch;
    for (PartKind k : {PartKind::Look, PartKind::Facts}) {
        MPart forged; forged.id = live::playerNet(1); forged.epoch = ep; forged.eventId = (2ull << 48) | (900 + partIndex(k)); forged.kind = k; forged.value = pv("forged");
        Envelope fe; fe.src = 2; fe.dest = kNoPlayer; fe.body = forged;
        r.net.send(2, 0, true, encodeEnvelope(fe));
    }
    r.run(500);
    for (int m = 0; m < 3; ++m)
        for (PartKind k : kAllPlayerKinds) CHECK_MSG(r.at(m).pp.lastWritten(1, k) != "forged", "a forged part was applied on machine " + std::to_string(m));
    CHECK(r.host().sync->stats().parts[partIndex(PartKind::Look)].rejected >= 1);
    CHECK(r.host().sync->stats().parts[partIndex(PartKind::Facts)].rejected >= 1);
    // and a part kind that does not belong to a player record is refused
    MPart inv; inv.id = live::playerNet(2); inv.epoch = r.host().sync->ownerOf(live::playerNet(2)).epoch; inv.eventId = (2ull << 48) | 950; inv.kind = PartKind::Inventory; inv.value = pv("x");
    Envelope ie; ie.src = 2; ie.dest = kNoPlayer; ie.body = inv;
    r.net.send(2, 0, true, encodeEnvelope(ie));
    r.run(300);
    CHECK(r.host().sync->stats().parts[partIndex(PartKind::Inventory)].rejected >= 1);
}

TEST(playerparts, a_late_joiner_hears_every_part_of_every_player_and_a_player_that_comes_back_tells_the_host_again) {
    Rig r(3);
    r.net.down.insert(2);   // machine 2 joins late: the join burst never reaches it
    playersUp(r);
    for (int m = 0; m < 3; ++m) setLocalParts(r, m);
    r.run(2000);
    CHECK(r.at(2).pp.written.empty());
    r.net.down.erase(2);
    r.at(2).sync->hostConnected();
    r.run(2500);
    for (PlayerId owner : std::initializer_list<PlayerId>{0, 1})
        for (PartKind k : kSharedKinds) CHECK_MSG(r.at(2).pp.lastWritten(owner, k) == valueOf(owner, k), std::string("the late joiner never got ") + partName(k) + " of " + std::to_string(owner));
    for (PlayerId owner : std::initializer_list<PlayerId>{0, 1})
        for (PartKind k : kHostOnlyKinds) CHECK(r.at(2).pp.writeCount(owner, k) == 0);   // host-only parts are not part of a join burst
    // what the late joiner wears reaches the others, too (its first parts went nowhere while it was cut off)
    for (int viewer : {0, 1})
        for (PartKind k : kSharedKinds) CHECK_MSG(r.at(viewer).pp.lastWritten(2, k) == valueOf(2, k), std::string("machine ") + std::to_string(viewer) + " never got " + partName(k) + " of the late joiner");
    Bytes pack;
    CHECK(r.host().sync->partValue(live::playerNet(2), PartKind::Pack, pack) && pack == pv(valueOf(2, PartKind::Pack)));
    // a guest whose link to the host was lost and came back tells the host everything again (a restarted host has none of it)
    const uint64_t before = r.at(1).sync->stats().parts[partIndex(PartKind::Look)].published;
    r.at(1).sync->hostLost();
    r.run(200);
    r.at(1).sync->hostConnected();
    r.run(2500);
    CHECK_MSG(r.at(1).sync->stats().parts[partIndex(PartKind::Look)].published == before + 1, "the guest did not publish its look again after the host link came back");
    CHECK(r.at(1).sync->stats().playerPartsRepublished >= 3);
}

TEST(playerparts, a_puppet_that_was_made_again_is_dressed_again) {
    Rig r(3);
    playersUp(r);
    for (int m = 0; m < 3; ++m) setLocalParts(r, m);
    r.run(2500);
    CHECK(r.at(0).pp.writeCount(1, PartKind::Look) == 1 && r.at(0).pp.writeCount(1, PartKind::Equipment) == 1);
    r.at(0).sync->onPlayerBound(live::playerNet(1));   // the puppet of player 1 on the host was deleted and spawned again (a save, a cell park, another race)
    r.run(1200);
    for (PartKind k : kSharedKinds) CHECK_MSG(r.at(0).pp.writeCount(1, k) == 2, std::string("the new puppet was not given its ") + partName(k) + " again");
    CHECK(r.at(0).pp.lastWritten(1, PartKind::Look) == valueOf(1, PartKind::Look));
    for (PartKind k : kHostOnlyKinds) CHECK(r.at(0).pp.writeCount(1, k) == 1);   // facts and pack are no puppet's business
    CHECK(r.at(2).pp.writeCount(1, PartKind::Look) == 1);   // the others' puppets are untouched
}

TEST(playerparts, an_npc_record_never_takes_a_player_part_and_the_switch_off_keeps_the_avatar_channel_in_charge) {
    {
        Rig r(2);
        playersUp(r);
        r.addActor(100, 3, V(1000));
        r.run(500);
        MPart m; m.id = 100; m.epoch = r.host().sync->ownerOf(100).epoch; m.eventId = 7000; m.kind = PartKind::Look; m.value = pv("look");
        inject(r, 1, 0, envelopeBytes(0, kNoPlayer, m));
        CHECK(r.at(1).sync->stats().parts[partIndex(PartKind::Look)].rejected >= 1);
        CHECK(r.at(1).pp.writeCount(0, PartKind::Look) == 0);
    }
    {
        Rig r(2);   // [entities] parts=legacy: ActorSync leaves the player parts to the avatar channel
        for (int i = 0; i < 2; ++i) r.at(i).sync->enablePlayerEntities(&r.at(i).pp, false, false);
        playersUp(r);
        for (int m = 0; m < 2; ++m) setLocalParts(r, m);
        r.run(2500);
        for (int m = 0; m < 2; ++m) {
            CHECK(r.at(m).pp.readCalls == 0 && r.at(m).pp.written.empty());
            for (PartKind k : kAllPlayerKinds) CHECK(r.at(m).sync->stats().parts[partIndex(k)].published == 0);
        }
    }
}

TEST(playerparts, a_player_that_leaves_and_comes_back_in_a_new_session_is_dressed_again_on_the_host) {
    Rig r(3);
    playersUp(r);
    for (int m = 0; m < 3; ++m) setLocalParts(r, m);
    r.run(2500);
    CHECK(r.host().pp.lastWritten(2, PartKind::Look) == valueOf(2, PartKind::Look) && r.host().pp.writeCount(2, PartKind::Look) == 1);
    // player 2 quits (the host untracks it) and starts a fresh session: a new ActorSync whose event counter starts again from 1
    r.host().sync->playerLeft(2);
    r.run(300);
    CHECK(!r.host().sync->tracked(live::playerNet(2)));
    Machine& m2 = r.at(2);
    SyncConfig c;
    c.self = 2; c.isHost = false;
    c.adjacent = [](CellId a, CellId b) { return a == b || a + 1 == b || b + 1 == a; };
    m2.sync = std::make_unique<ActorSync>(c, m2.eng, *m2.ep);
    m2.sync->enablePlayerEntities(&m2.pp, true, false);
    m2.pp.local[PartKind::Look] = pv("look-2-again");
    r.host().sync->playerJoined(2);
    m2.sync->hostConnected();
    r.run(2500);
    CHECK(r.host().sync->ownerOf(live::playerNet(2)).epoch >= 2);
    CHECK_MSG(r.host().pp.lastWritten(2, PartKind::Look) == "look-2-again", "the host never heard the look of the player who came back");
    CHECK(r.at(1).pp.lastWritten(2, PartKind::Look) == "look-2-again");
    CHECK(r.host().pp.lastWritten(2, PartKind::Equipment) == valueOf(2, PartKind::Equipment));
}

// ---- the player stream profile (docs/entity-core-plan.md S10): 30 Hz, own sender and receiver, relay on arrival, near / far / out tiers, a jump at once ----
// Twins NVA_MUT 58 (a player shares the NPC sender), 59 (the rate is ignored), 60 (the uplink follows the lowest tier), 61 (held snapshots never released),
// 62 (no jump pass), 64 (a client may publish another player's stream), 67 (the sampler's tick replaced), 69 (the relay goes back to the owner), 72 (places never learned).
namespace {
struct Walkers {
    Rig& r;
    std::vector<uint64_t> next;
    std::vector<float> x, vx;
    std::vector<CellId> cell;
    std::vector<bool> on;
    std::vector<float> lastX;   // where the last sample was
    explicit Walkers(Rig& rig) : r(rig), next(8, 0), x(8, 0.0f), vx(8, 0.0f), cell(8, 3), on(8, true), lastX(8, 0.0f) {}
    void feed() {   // the sampler's output, 33 ms apart
        for (int m = 0; m < r.n(); ++m) {
            if (!on[static_cast<size_t>(m)] || r.now < next[static_cast<size_t>(m)]) continue;
            const size_t i = static_cast<size_t>(m);
            live::ActorSnapshot s;
            s.cell = cell[i]; s.pos = V(x[i]); s.vel = V(vx[i]); s.tick = static_cast<uint32_t>(r.now) + 9000u * static_cast<uint32_t>(m + 1);   // each machine's own clock
            r.at(m).pp.samples.push_back(s);
            lastX[i] = x[i];
            next[i] = (next[i] ? next[i] : r.now) + 33;
            x[i] += vx[i] * 0.033f;
        }
    }
    void run(uint32_t ms) { for (uint32_t t = 0; t < ms; t += 10) { feed(); r.step(10); } }
};

void streamsOn(Rig& r) {
    playersUp(r);
    for (int m = 0; m < r.n(); ++m) r.at(m).sync->enablePlayerEntities(&r.at(m).pp, true, true);
}

int rxCount(Rig& r, int viewer, int owner, uint64_t from = 0, uint64_t to = ~0ull) {
    int n = 0;
    for (const auto& x : r.at(viewer).pp.rx) if (x.id == live::playerNet(static_cast<PlayerId>(owner)) && x.at >= from && x.at < to) ++n;
    return n;
}
const FakePlayerPort::Rx* lastRx(Rig& r, int viewer, int owner) {
    const FakePlayerPort::Rx* last = nullptr;
    for (const auto& x : r.at(viewer).pp.rx) if (x.id == live::playerNet(static_cast<PlayerId>(owner))) last = &x;
    return last;
}
}  // namespace

TEST(playerstream, the_profiles_say_what_differs_between_a_player_and_an_npc) {
    Rig r(2);
    const StreamProfile p = r.host().sync->profileFor(live::playerNet(1)), n = r.host().sync->profileFor(100);
    CHECK(p.interp.lowLatency && !n.interp.lowLatency && n.interp.delayMs == 100);
    CHECK(p.sampleMs == 33 && p.nearMs == 33 && p.farMs == 200 && p.outMs == 1000 && p.jumpUnits == 350.0f);
    CHECK(p.relayOnArrival && p.uplinkFromObservers && p.interpInPort && p.tickFromPort);
    CHECK(p.sub == SubBatchPlayer && p.ackSub == SubAckPlayer && n.sub == SubBatch && n.ackSub == SubAck);
    CHECK(!n.relayOnArrival && !n.uplinkFromObservers && !n.interpInPort && n.outMs == 0 && n.jumpUnits == 0.0f && n.nearMs == 50 && n.farMs == 200);
}

TEST(playerstream, every_puppet_follows_its_owner_at_30_hz_through_the_pinned_epoch_and_the_players_own_packets) {
    Rig r(3, 5, NetOpts{20, 0, 0, 0});
    streamsOn(r);
    int bySub[8] = {0, 0, 0, 0, 0, 0, 0, 0};
    r.net.dropIf = [&](const NetMsg& m) { if (m.data.size() > 1 && m.data[1] < 8) ++bySub[m.data[1]]; return false; };
    Walkers w(r);
    w.vx[0] = 100; w.vx[1] = 200; w.vx[2] = 300;
    w.x[1] = 50; w.x[2] = 90;
    w.run(3500);
    for (int viewer = 0; viewer < 3; ++viewer)
        for (int owner = 0; owner < 3; ++owner) {
            if (viewer == owner) { CHECK(rxCount(r, viewer, owner) == 0); continue; }   // nobody is played its own stream back
            const int n = rxCount(r, viewer, owner, 1500, 3500);
            CHECK_MSG(n >= 50 && n <= 70, "machine " + std::to_string(viewer) + " got " + std::to_string(n) + " snapshots of player " + std::to_string(owner) + " in two seconds (about 60)");
            const FakePlayerPort::Rx* l = lastRx(r, viewer, owner);
            CHECK(l && l->s.ownerEpoch == r.host().sync->ownerOf(live::playerNet(static_cast<PlayerId>(owner))).epoch && l->s.netId == live::playerNet(static_cast<PlayerId>(owner)));
            CHECK(l && std::fabs(l->s.pos.x - w.x[static_cast<size_t>(owner)]) < 3.0f * static_cast<float>(w.vx[static_cast<size_t>(owner)]) / 10.0f + 30.0f);   // within about a tenth of a second of the walker
            CHECK(l && l->s.tick > 9000u * static_cast<uint32_t>(owner + 1));   // the sampler's own tick, not ActorSync's clock
            // seq strictly rises per owner
            uint32_t prev = 0;
            bool ordered = true;
            for (const auto& x : r.at(viewer).pp.rx) if (x.id == live::playerNet(static_cast<PlayerId>(owner))) { if (x.s.seq <= prev) ordered = false; prev = x.s.seq; }
            CHECK(ordered);
        }
    CHECK(bySub[SubBatchPlayer] > 100 && bySub[SubAckPlayer] > 10 && bySub[SubBatch] == 0);   // the player stream rides its own packets and acknowledgements
    for (int m = 0; m < 3; ++m) {
        CHECK(r.at(m).pp.wrong == 0 && r.at(m).eng.ops.empty());
        CHECK(r.at(m).sync->stats().playerForged == 0 && r.at(m).sync->stats().playerSampled > 80);
        CHECK(r.at(m).sync->stats().playerEchoed == 0);   // the host never plays a player its own stream back
    }
    CHECK(r.host().sync->stats().playerRelayed > 100);   // the host relays at once
    CHECK(r.host().sync->playerSenderStats().packets > 100 && r.host().sync->senderStats().packets == 0);
}

TEST(playerstream, a_far_player_is_trickled_once_a_second_the_last_place_is_never_lost_and_a_jump_passes_at_once) {
    Rig r(3, 5, NetOpts{20, 0, 0, 0});
    streamsOn(r);
    Walkers w(r);
    w.cell[2] = 9;   // player 2 is in another cell, further than adjacent: the out tier for everybody in cell 3
    w.vx[1] = 120;
    w.vx[2] = 120;
    w.run(4000);
    CHECK_MSG(rxCount(r, 0, 1, 2000, 4000) >= 50, "the host (same cell as player 1) must see it at the near rate");
    const int out = rxCount(r, 2, 1, 2000, 4000);
    CHECK_MSG(out >= 1 && out <= 4, "player 2 (another cell) got " + std::to_string(out) + " snapshots of player 1 in two seconds (the out tier: about 2)");
    const int up = rxCount(r, 0, 2, 2000, 4000);
    CHECK_MSG(up >= 1 && up <= 4, "the host got " + std::to_string(up) + " snapshots from player 2 in two seconds (every observer is out: about 2)");
    // the uplink follows the HIGHEST tier any observer has: player 1 (cell 3) stays near for the host and is sent at 30 Hz although player 2 is out
    CHECK(rxCount(r, 0, 1, 2000, 4000) >= 50);
    // player 1 stops sampling at six different moments of the out tier's period: the last place it was at always reaches the observer (the planner keeps the newest
    // snapshot that was not due and releases it), though nothing newer follows
    for (int k = 0; k < 6; ++k) {
        w.on[1] = true;
        w.next[1] = r.now;   // (no backlog of samples from the time it was off)
        w.vx[1] = 200;
        w.run(1000 + static_cast<uint32_t>(k) * 170);
        w.on[1] = false;
        const float stopX = w.lastX[1];
        w.run(1300);
        const FakePlayerPort::Rx* l = lastRx(r, 2, 1);
        CHECK_MSG(l && std::fabs(l->s.pos.x - stopX) < 0.5f, "round " + std::to_string(k) + ": the last place of player 1 never reached the out-tier observer");
    }
    // a jump goes through the rate gate at once, at every tier and at any phase of the period
    w.on[1] = true;
    w.next[1] = r.now;
    w.vx[1] = 0;
    for (int k = 0; k < 6; ++k) {
        w.run(137 + static_cast<uint32_t>(k) * 61);
        const uint64_t t0 = r.now;
        w.x[1] += (k & 1) ? -900.0f : 900.0f;
        w.run(200);
        bool arrived = false;
        uint64_t at = 0;
        for (const auto& x : r.at(2).pp.rx) if (x.id == live::playerNet(1) && x.at >= t0 && std::fabs(x.s.pos.x - w.x[1]) < 1.0f) { arrived = true; at = x.at; break; }
        CHECK_MSG(arrived, "jump " + std::to_string(k) + " of player 1 did not reach the out-tier observer within 200 ms");
        CHECK_MSG(arrived && at - t0 <= 140, "jump " + std::to_string(k) + " arrived after " + std::to_string(at - t0) + " ms");
    }
}

TEST(playerstream, a_player_does_not_wait_behind_an_npc_burst) {
    Rig r(3, 5, NetOpts{20, 0, 0, 0});
    streamsOn(r);
    for (uint32_t i = 0; i < 60; ++i) r.addActor(300 + i, 3, V(100.0f * static_cast<float>(i)), false, ~0u, 40, 10);   // sixty walking NPCs fill the NPC packets
    for (int i = 0; i < 3; ++i) r.setPose(i, 3, V(500.0f * static_cast<float>(i)));
    int bySub[8] = {0, 0, 0, 0, 0, 0, 0, 0};
    r.net.dropIf = [&](const NetMsg& m) { if (m.data.size() > 1 && m.data[1] < 8) ++bySub[m.data[1]]; return false; };
    Walkers w(r);
    w.vx[1] = 150; w.vx[2] = 150; w.vx[0] = 150;
    w.run(3000);
    CHECK(bySub[SubBatch] > 50 && bySub[SubBatchPlayer] > 50);   // both streams flow, in packets of their own
    CHECK(rxCount(r, 0, 1, 1000, 3000) >= 50 && rxCount(r, 2, 0, 1000, 3000) >= 50);
    CHECK(r.host().sync->senderStats().packets > 0 && r.host().sync->playerSenderStats().packets > 0);
    CHECK(r.at(2).sync->stats().snapshotsApplied > 100);   // and the NPCs went on being played, too
}

TEST(playerstream, a_client_cannot_publish_another_players_stream_and_an_old_session_is_not_believed_after_a_rejoin) {
    Rig r(3, 5, NetOpts{20, 0, 0, 0});
    streamsOn(r);
    Walkers w(r);
    w.vx[1] = 100; w.vx[2] = 100;
    w.run(2000);
    // client 2 sends a snapshot that claims to be player 1's, at the right epoch, far away
    live::SnapshotSender evil;
    live::ActorSnapshot s = craftSnap(live::playerNet(1), r.host().sync->ownerOf(live::playerNet(1)).epoch, 900000, 3, V(70000));
    evil.queue(0, s, 5);
    for (Bytes& d : evil.flush(0, 5)) inject(r, 0, 2, wrapLive(SubBatchPlayer, d));
    r.run(200);
    CHECK(r.host().sync->stats().playerForged >= 1);
    for (int m = 0; m < 3; ++m) for (const auto& x : r.at(m).pp.rx) CHECK(x.s.pos.x < 60000.0f);
    // player 1 leaves and comes back in a new session (epoch above the old one): a datagram of the old session is not believed
    live::SnapshotSender old;
    live::ActorSnapshot os = craftSnap(live::playerNet(1), r.host().sync->ownerOf(live::playerNet(1)).epoch, 800000, 3, V(40000));
    const OwnerEpoch oldEpoch = os.ownerEpoch;
    old.queue(0, os, 5);
    std::vector<Bytes> oldDatagrams = old.flush(0, 5);
    r.host().sync->playerLeft(1);
    r.run(300);
    r.host().sync->playerJoined(1);
    r.at(1).sync->hostConnected();
    w.run(800);
    CHECK(r.host().sync->ownerOf(live::playerNet(1)).epoch > oldEpoch);
    const size_t before = r.host().pp.rx.size();
    for (Bytes& d : oldDatagrams) inject(r, 0, 1, wrapLive(SubBatchPlayer, d));
    r.run(200);
    for (size_t i = before; i < r.host().pp.rx.size(); ++i) CHECK(r.host().pp.rx[i].s.pos.x < 30000.0f);
}

TEST(playerstream, a_snapshot_ahead_of_its_record_is_kept_and_played_when_the_record_arrives) {
    Rig r(3, 5, NetOpts{20, 0, 0, 0});
    r.net.down.insert(2);   // machine 2 has heard nothing yet
    streamsOn(r);
    r.run(300);
    live::SnapshotSender hostSender;
    live::ActorSnapshot s = craftSnap(live::playerNet(1), 1, 5, 3, V(1234));
    hostSender.queue(2, s, 5);
    for (Bytes& d : hostSender.flush(2, 5)) inject(r, 2, 0, wrapLive(SubBatchPlayer, d));   // the host's stream overtook its reliable messages
    CHECK(r.at(2).sync->stats().playerUnknown >= 1 && r.at(2).pp.rx.empty());
    r.net.down.erase(2);
    r.at(2).sync->hostConnected();
    r.run(1500);
    CHECK(r.at(2).sync->stats().earlyApplied >= 1);
    bool played = false;
    for (const auto& x : r.at(2).pp.rx) if (x.id == live::playerNet(1) && std::fabs(x.s.pos.x - 1234.0f) < 0.5f) played = true;
    CHECK(played);
}

TEST(playerstream, with_the_stream_switched_off_the_avatar_channel_keeps_it) {
    Rig r(2, 5, NetOpts{20, 0, 0, 0});
    playersUp(r);   // parts on, stream off (the default of a session that runs stream=legacy)
    Walkers w(r);
    w.vx[0] = 100; w.vx[1] = 100;
    w.run(1500);
    for (int m = 0; m < 2; ++m) {
        CHECK(r.at(m).pp.samples.size() > 20 && r.at(m).pp.rx.empty());   // nothing was sampled, sent or played
        CHECK(r.at(m).sync->stats().playerSampled == 0 && r.at(m).sync->playerSenderStats().packets == 0);
    }
}
