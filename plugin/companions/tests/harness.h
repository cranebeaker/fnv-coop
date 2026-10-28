// Test harness for plugin/companions: a cluster of machines (0 = host), each with its own SocialSync, a scripted world, a recording fake engine and an
// in-memory star network. The host's committer plays the story session: committed XP / karma / reputation effects are delivered to the machines their
// recipient mask names (exactly what the real session would do), so a test can assert "every player got it ONCE".
#pragma once
#include <algorithm>
#include <cstdio>
#include <deque>
#include <functional>
#include <map>
#include <memory>
#include <set>
#include <tuple>
#include "companions/social_sync.h"
#include "minitest.h"

namespace stest {
using namespace nvcoop::social;

inline FormRef npcForm(uint32_t n) { return FormRef("falloutnv.esm", 0x100000u + n); }
inline FormRef itemForm(uint32_t n) { return FormRef("falloutnv.esm", 0x200000u + n); }
inline FormRef factionForm(uint32_t n) { return FormRef("falloutnv.esm", 0x300000u + n); }
inline fnvcoop::Effect recruitFx(const FormRef& n, PlayerId owner, uint8_t kind = 0) { fnvcoop::CompanionOwner c; c.npc = n; c.owner = owner; c.active = true; c.kind = kind; return c; }
inline fnvcoop::Effect dismissFx(const FormRef& n, PlayerId owner) { fnvcoop::CompanionOwner c; c.npc = n; c.owner = owner; c.active = false; return c; }

struct NpcRec {
    uint32_t netId = 0;
    PlayerId owner = kNoPlayer;
    bool standIn = false, alive = true, loaded = true, waiting = false;
    uint32_t cell = 1;
    cc::Vec3 pos;
    FormRef faction;
};

struct Machine;

struct FakeEngine : ISocialEngine {
    Machine* mc = nullptr;
    // companions
    std::set<FormRef> teammates;
    std::vector<std::pair<FormRef, bool>> teammateCalls;
    bool teammateWorks = true;
    std::vector<std::pair<FormRef, sc::CompanionState>> stateCalls;
    bool stateMappable = true;
    std::vector<FormRef> summons;
    std::map<FormRef, sc::ItemCounts> inventory;     // this machine's copy of each NPC's inventory
    std::vector<std::pair<FormRef, std::pair<FormRef, int32_t>>> invCalls;
    // crime
    std::map<FormRef, sc::FactionStanding> standing;
    std::vector<std::pair<FormRef, sc::FactionStanding>> standingCalls;
    std::vector<cc::HostilityEffect> hostility;
    std::vector<std::pair<FormRef, uint32_t>> alerts;
    std::vector<sc::Detection> detections;           // what observeDetections reports right now
    bool trespassing = false;
    sc::InteractionFacts trespassFacts;
    uint32_t trespassCell = 1;
    cc::Vec3 trespassWhere;
    std::map<FormRef, sc::InteractionFacts> interactions;   // readInteraction by target
    // rewards
    int32_t xp = 0, karma = 0;
    int xpCalls = 0, karmaCalls = 0;
    std::vector<std::tuple<FormRef, int, int>> reps;

    bool setTeammate(const FormRef& npc, bool on) override {
        teammateCalls.push_back({npc, on});
        if (!teammateWorks) return false;
        if (on) teammates.insert(npc); else teammates.erase(npc);
        return true;
    }
    bool applyCompanionState(const FormRef& npc, const sc::CompanionState& s) override {
        stateCalls.push_back({npc, s});
        return stateMappable;
    }
    bool summonCompanion(const FormRef& npc, PlayerId) override { summons.push_back(npc); return true; }
    bool readInventory(const FormRef& npc, sc::ItemCounts& out) override {
        auto it = inventory.find(npc);
        out = it == inventory.end() ? sc::ItemCounts() : it->second;
        return true;
    }
    bool applyInventoryDelta(const FormRef& npc, const FormRef& item, int32_t delta) override {
        invCalls.push_back({npc, {item, delta}});
        int32_t& n = inventory[npc][item];
        n += delta;
        if (n == 0) inventory[npc].erase(item);
        return true;
    }
    bool applyStanding(const FormRef& f, const sc::FactionStanding& st) override { standingCalls.push_back({f, st}); standing[f] = st; return true; }
    bool applyHostility(const cc::HostilityEffect& e) override { hostility.push_back(e); return true; }
    bool alertActor(const FormRef& npc, uint32_t criminalRef) override { alerts.push_back({npc, criminalRef}); return true; }
    void observeDetections(std::vector<sc::Detection>& out) override { out = detections; }
    bool localTrespass(sc::InteractionFacts& f, uint32_t& cell, cc::Vec3& where) override {
        if (!trespassing) return false;
        f = trespassFacts; cell = trespassCell; where = trespassWhere;
        return true;
    }
    bool readInteraction(const FormRef& target, bool, sc::InteractionFacts& f, uint32_t& cell, cc::Vec3& where) override {
        auto it = interactions.find(target);
        if (it == interactions.end()) return false;
        f = it->second; cell = 1; where = {10, 20, 30};
        return true;
    }
    bool awardXp(int32_t x) override { xp += x; ++xpCalls; return true; }
    bool rewardKarma(int32_t d) override { karma += d; ++karmaCalls; return true; }
    bool modReputation(const FormRef& f, int fame, int infamy) override { reps.push_back({f, fame, infamy}); return true; }
};

struct FakeWorld : ISocialWorld {
    Machine* mc = nullptr;
    PlayerId me = 0;
    std::map<FormRef, NpcRec> npcs;
    std::vector<sc::PlayerPlace> places;

    static uint32_t refOf(const NpcRec& r) { return r.loaded ? 0x5000u + r.netId : 0; }
    void view(const FormRef& n, const NpcRec& r, ActorView& v) const {
        v = ActorView{};
        v.known = true; v.npc = n; v.netId = r.netId; v.ref = refOf(r); v.owner = r.owner; v.standIn = r.standIn; v.alive = r.alive; v.waiting = r.waiting;
        v.simulatedHere = r.owner == me && !r.standIn && r.loaded;
        v.cell = r.cell; v.pos = r.pos; v.faction = r.faction;
    }
    bool actor(const FormRef& n, ActorView& v) const override {
        auto it = npcs.find(n);
        if (it == npcs.end()) return false;
        view(n, it->second, v);
        return true;
    }
    bool actorByNet(uint32_t id, ActorView& v) const override {
        for (auto& kv : npcs) if (kv.second.netId == id) { view(kv.first, kv.second, v); return true; }
        return false;
    }
    std::vector<sc::PlayerPlace> players() const override { return places; }
    uint32_t refOfPlayer(PlayerId slot) const override { return slot == me ? 0x14u : 0x2000u + slot; }
    std::vector<FormRef> teammatesHere() const override;
    std::vector<FormRef> factionActorsHere(const FormRef& f, uint32_t cell, const cc::Vec3& where, float radius) const override {
        std::vector<FormRef> v;
        for (auto& kv : npcs) {
            const NpcRec& r = kv.second;
            if (r.owner != me || r.standIn || !r.loaded || !r.alive || r.faction != f || r.cell != cell) continue;
            if (cc::distance(r.pos, where) <= radius) v.push_back(kv.first);
        }
        return v;
    }
};

struct Cluster;

struct Machine : ISocialNet, ICommitter {
    Cluster* cl = nullptr;
    PlayerId slot = 0;
    FakeWorld world;
    FakeEngine eng;
    std::unique_ptr<SocialSync> sync;
    std::vector<fnvcoop::Effect> fed;     // CompanionOwner effects fed to the actor layer
    void send(PlayerId peer, Bytes data) override;
    uint64_t commit(const std::vector<Effect>& fx, const char* what) override;
};

inline std::vector<FormRef> FakeWorld::teammatesHere() const { return std::vector<FormRef>(mc->eng.teammates.begin(), mc->eng.teammates.end()); }

struct Datagram { PlayerId from, to; Bytes data; };
struct Batch { std::string what; std::vector<fnvcoop::Effect> fx; };

struct Cluster {
    std::vector<std::unique_ptr<Machine>> m;
    std::deque<Datagram> q;
    std::vector<Batch> commits;
    uint64_t now = 1000;
    uint64_t delivered = 0;
    bool commitWorks = true;

    explicit Cluster(int n, const std::function<void(Config&)>& tweak = nullptr, bool join = true, bool firstTick = true) {
        for (int i = 0; i < n; ++i) {
            auto mc = std::make_unique<Machine>();
            mc->cl = this; mc->slot = static_cast<PlayerId>(i);
            mc->world.mc = mc.get(); mc->world.me = mc->slot; mc->eng.mc = mc.get();
            Config c;
            c.self = mc->slot; c.isHost = i == 0;
            Machine* raw = mc.get();
            c.feedActors = [raw](const Effect& e) { raw->fed.push_back(e); };
            if (tweak) tweak(c);
            mc->sync.reset(new SocialSync(c, mc->world, mc->eng, *mc, *mc));
            m.push_back(std::move(mc));
        }
        // everybody knows every player; positions spread along x, all in cell 1
        for (int i = 0; i < n; ++i)
            for (int j = 0; j < n; ++j) {
                sc::PlayerPlace p;
                p.slot = static_cast<PlayerId>(j); p.connected = true; p.cell = 1; p.pos = {static_cast<float>(j) * 100, 0, 0};
                m[i]->world.places.push_back(p);
            }
        if (join) {
            for (int i = 1; i < n; ++i) m[0]->sync->playerJoined(static_cast<PlayerId>(i));
            for (int i = 1; i < n; ++i) m[i]->sync->hostConnected();
        }
        if (firstTick) tick(0);
    }
    Machine& at(int i) { return *m[i]; }
    SocialSync& s(int i) { return *m[i]->sync; }
    FakeEngine& e(int i) { return m[i]->eng; }

    // an NPC every machine knows, simulated by `owner`
    void addNpc(const FormRef& n, uint32_t netId, PlayerId owner, const FormRef& faction = FormRef(), cc::Vec3 pos = {}, uint32_t cell = 1) {
        for (auto& mc : m) {
            NpcRec r;
            r.netId = netId; r.owner = owner; r.faction = faction; r.pos = pos; r.cell = cell;
            mc->world.npcs[n] = r;
        }
    }
    void setOwner(const FormRef& n, PlayerId owner) { for (auto& mc : m) mc->world.npcs[n].owner = owner; }

    void deliverAll(int maxRounds = 100) {
        for (int round = 0; round < maxRounds && !q.empty(); ++round) {
            std::deque<Datagram> cur;
            cur.swap(q);
            for (auto& d : cur) { ++delivered; m[d.to]->sync->receive(d.from, d.data.data(), d.data.size()); }
        }
    }
    void tick(uint64_t adv = 10) {
        now += adv;
        for (auto& mc : m) mc->sync->tick(now);
        deliverAll();
    }
    void run(uint64_t ms, uint64_t step = 100) { for (uint64_t t = 0; t < ms; t += step) tick(step); }
    // a hand-made datagram (forgery, duplicate, wrong sender): `from` is what the transport says
    void inject(PlayerId from, PlayerId to, const sc::Body& body, PlayerId claimedSrc = kNoPlayer) {
        sc::Envelope env;
        env.src = claimedSrc == kNoPlayer ? from : claimedSrc;
        env.dest = to;
        env.body = body;
        Bytes w = sc::encodeEnvelope(env);
        ++delivered;
        m[to]->sync->receive(from, w.data(), w.size());
        deliverAll();
    }
    size_t committedCount(const char* what) const { size_t n = 0; for (auto& b : commits) if (b.what == what) ++n; return n; }
};

inline void Machine::send(PlayerId peer, Bytes data) { cl->q.push_back({slot, peer, std::move(data)}); }

inline uint8_t maskOf(const fnvcoop::Recipient& r, uint8_t all) {
    if (r.mode == fnvcoop::Recipient::Mask) return r.mask;
    if (r.mode == fnvcoop::Recipient::All) return all;
    return 0;
}

inline uint64_t Machine::commit(const std::vector<Effect>& fx, const char* what) {
    if (!cl->commitWorks) return 0;
    cl->commits.push_back({what, fx});
    const uint8_t all = static_cast<uint8_t>((1u << cl->m.size()) - 1u);
    // the story session delivers each effect to the machines its recipient mask names (the host's own engine is the source of its effects)
    for (const Effect& e : fx) {
        if (const auto* x = std::get_if<fnvcoop::ModActorValue>(&e)) {
            const uint8_t mk = maskOf(x->to, all);
            for (size_t i = 0; i < cl->m.size(); ++i) if (mk & (1u << i)) { cl->m[i]->eng.xp += x->delta; ++cl->m[i]->eng.xpCalls; }
        } else if (const auto* k = std::get_if<fnvcoop::ModKarma>(&e)) {
            const uint8_t mk = maskOf(k->to, all);
            for (size_t i = 0; i < cl->m.size(); ++i) if (mk & (1u << i)) { cl->m[i]->eng.karma += k->delta; ++cl->m[i]->eng.karmaCalls; }
        } else if (const auto* r = std::get_if<fnvcoop::ModReputation>(&e)) {
            const uint8_t mk = maskOf(r->to, all);
            for (size_t i = 0; i < cl->m.size(); ++i) if (mk & (1u << i)) cl->m[i]->eng.reps.push_back({r->faction, r->fame, r->infamy});
        }
    }
    return cl->commits.size();
}

inline sc::Detection sees(uint32_t observerNet, const FormRef& faction, PlayerId target, uint32_t cell = 1) {
    sc::Detection d;
    d.observer = cc::Entity::actor(observerNet); d.faction = faction; d.target = target; d.cell = cell; d.seen = true;
    return d;
}

}  // namespace stest
