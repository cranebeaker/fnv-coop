// Test harness for plugin/combat: a cluster of machines, each with its own CombatSync, its own (possibly stale) idea of who owns what, a recording
// fake engine and an in-memory network (star topology: machine 0 is the host). The fake engine models just enough of the game: damage lowers
// health, health 0 kills, and a kill reaches the other machines the way ActorSync's death message would.
#pragma once
#include <algorithm>
#include <cstdio>
#include <deque>
#include <functional>
#include <map>
#include <memory>
#include <set>
#include "combat/combat_sync.h"
#include "minitest.h"

namespace ctest {
using namespace nvcoop::combat;

struct Rec {
    cc::Entity who;
    PlayerId owner = kNoPlayer;
    OwnerEpoch epoch = 0;
    bool pending = false, alive = true, named = false, essential = false, standIn = false;
    cc::CellId cell = 1;
    cc::Vec3 pos;
    float radius = 40;
    int32_t health = 100, maxHealth = 100;
    bool lifeLag = false;             // health <= 0 (alive == false in the view) but the engine's life state still reads alive: a lethal VATS hit that was never finished
    float sliver = 0;                 // the engine's float health beyond the integer the model shows (12.4 reads 12: sliver 0.4)
    uint8_t limbMask = 0;
    PlayerId recruiter = kNoPlayer;
    FormRef faction;
};

inline FormRef rifle() { return FormRef("falloutnv.esm", 0x1000); }
inline FormRef grenade() { return FormRef("falloutnv.esm", 0x1002); }

constexpr uint32_t kLocalPlayerRef = 0x14;
inline uint32_t refOfPlayer(PlayerId me, PlayerId slot) { return slot == me ? kLocalPlayerRef : 0x2000u + slot; }
inline uint32_t refOfActor(uint32_t id) { return 0x1000u + id; }

struct FakeWorld : IWorldView {
    PlayerId me = 0;
    std::map<cc::Entity, Rec> recs;
    std::vector<PlayerId> present;
    std::map<FormRef, cc::WeaponInfo> weapons;
    bool los = true;

    PlayerId self() const override { return me; }
    uint32_t refOf(const cc::Entity& e) const {
        auto it = recs.find(e);
        if (it == recs.end()) return 0;
        return e.isPlayer() ? refOfPlayer(me, static_cast<PlayerId>(e.id)) : refOfActor(e.id);
    }
    bool entityOfRef(uint32_t ref, cc::Entity& out) const override {
        if (ref == kLocalPlayerRef) { out = cc::Entity::player(me); return recs.count(out) != 0; }
        if (ref >= 0x2000 && ref < 0x2100) { out = cc::Entity::player(static_cast<PlayerId>(ref - 0x2000)); return recs.count(out) != 0; }
        if (ref >= 0x1000 && ref < 0x2000) { out = cc::Entity::actor(ref - 0x1000); return recs.count(out) != 0; }
        return false;
    }
    bool entity(const cc::Entity& e, EntityView& out) const override {
        auto it = recs.find(e);
        if (it == recs.end()) { out = EntityView{}; return false; }
        const Rec& r = it->second;
        out = EntityView{};
        out.known = true; out.who = e; out.ref = refOf(e); out.owner = e.isPlayer() ? static_cast<PlayerId>(e.id) : r.owner; out.epoch = r.epoch;
        out.simulatedHere = e.isPlayer() ? e.id == me : (r.owner == me && !r.pending && !r.standIn);
        out.pending = r.pending; out.standIn = r.standIn; out.alive = r.alive; out.named = r.named; out.essential = r.essential;
        out.cell = r.cell; out.pos = r.pos; out.radius = r.radius; out.health = r.health; out.maxHealth = r.maxHealth; out.limbMask = r.limbMask;
        out.recruiter = r.recruiter; out.faction = r.faction;
        return true;
    }
    std::vector<cc::Entity> entitiesNear(cc::CellId cell, const cc::Vec3& pos, float radius) const override {
        std::vector<cc::Entity> v;
        for (auto& kv : recs)
            if (kv.second.cell == cell && cc::distance(kv.second.pos, pos) <= radius + kv.second.radius) v.push_back(kv.first);
        return v;
    }
    std::vector<PlayerId> presentPlayers() const override { return present; }
    const cc::WeaponInfo* weapon(const FormRef& id) const override { auto it = weapons.find(id); return it == weapons.end() ? nullptr : &it->second; }
    bool lineOfFire(const cc::Vec3&, const cc::Vec3&) const override { return los; }
};

struct DamageCall { cc::Entity victim; uint32_t ref = 0; int32_t amount = 0; uint32_t attackerRef = 0; uint64_t hitId = 0; bool vats = false; cc::Limb limb = cc::Limb::Torso; cc::Entity attacker; bool owned = false; };

struct FakeEngine : ICombatEngine {
    FakeWorld* w = nullptr;
    std::vector<DamageCall> damage;
    std::vector<std::pair<cc::Entity, cc::Limb>> cripples;
    std::vector<std::pair<uint32_t, int32_t>> mirrors;
    std::vector<std::pair<uint32_t, uint32_t>> provokes;
    std::vector<std::pair<uint32_t, uint8_t>> corpses;
    std::vector<int32_t> xp;
    std::vector<cc::ProjectileSpawn> projectiles;
    std::vector<cc::ExplosionEvent> explosions;
    std::vector<cc::HostilityEffect> hostility;
    std::deque<uint32_t> deaths;          // refs that died here by damage (the actor port would report them)
    int32_t xpValue = 50;
    bool vats = false;
    bool refuseDamage = false;
    bool refuseLethal = false;            // the engine takes at most the integer health off: a sliver of float health survives every blow
    bool refuseKill = false;
    std::vector<uint32_t> kills;          // refs the owner killed explicitly (killActor)
    std::vector<std::pair<uint32_t, uint32_t>> killsBy;     // (victim ref, killer puppet ref): the kill credited to a remote player (G04)
    std::vector<std::pair<uint32_t, uint32_t>> killerSet;   // (corpse ref, killer puppet ref): the killer written onto a corpse that was already here

    Rec* find(uint32_t ref) {
        cc::Entity e;
        if (!w->entityOfRef(ref, e)) return nullptr;
        auto it = w->recs.find(e);
        return it == w->recs.end() ? nullptr : &it->second;
    }
    bool damageHealth(const cc::Entity& victim, uint32_t ref, int32_t amount, uint32_t attackerRef, const cc::HitEvent& h) override {
        if (refuseDamage) return false;
        Rec* r = find(ref);
        if (!r) return false;
        // "owned": the machine that applies a hit simulates its victim right now (an actor it owns, not mid-handoff, or its own player)
        const bool owned = victim.isPlayer() ? victim.id == w->me : (r->owner == w->me && !r->pending && !r->standIn);
        damage.push_back({victim, ref, amount, attackerRef, h.hitId, h.vats, h.limb, h.attacker, owned});
        const float real = static_cast<float>(r->health) + r->sliver - static_cast<float>(refuseLethal ? std::min(amount, r->health) : amount);
        if (real <= 0 && r->alive) { r->health = 0; r->sliver = 0; r->alive = false; deaths.push_back(ref); }
        else if (r->alive) { r->health = static_cast<int32_t>(real); r->sliver = real - static_cast<float>(r->health); }
        return true;
    }
    bool killActor(uint32_t ref) override {
        Rec* r = find(ref);
        if (!r || refuseKill) return false;
        kills.push_back(ref);
        if (r->alive || r->lifeLag) { r->alive = false; r->lifeLag = false; r->health = 0; r->sliver = 0; deaths.push_back(ref); }
        return true;
    }
    bool killActorBy(uint32_t ref, uint32_t killer) override {
        Rec* r = find(ref);
        if (!r || refuseKill) return false;
        killsBy.push_back({ref, killer});
        if (r->alive || r->lifeLag) { r->alive = false; r->lifeLag = false; r->health = 0; r->sliver = 0; deaths.push_back(ref); }
        return true;
    }
    bool setKiller(uint32_t ref, uint32_t killer) override { killerSet.push_back({ref, killer}); return true; }
    bool lifeDead(uint32_t ref) override { Rec* r = find(ref); return r && !r->alive && !r->lifeLag; }
    bool setLimbCrippled(const cc::Entity& v, uint32_t ref, cc::Limb l) override {
        Rec* r = find(ref);
        if (!r) return false;
        r->limbMask = static_cast<uint8_t>(r->limbMask | cc::LimbState::bit(l));
        cripples.push_back({v, l});
        return true;
    }
    bool mirrorHealth(uint32_t ref, int32_t health) override {
        Rec* r = find(ref);
        if (!r || !r->alive) return false;
        r->health = health;
        mirrors.push_back({ref, health});
        return true;
    }
    bool provoke(uint32_t v, uint32_t a) override { provokes.push_back({v, a}); return true; }
    bool decorateCorpse(uint32_t ref, uint8_t mask) override { corpses.push_back({ref, mask}); return true; }
    bool isDead(uint32_t ref) override { Rec* r = find(ref); return r && !r->alive; }
    int32_t killXpOf(uint32_t) override { return xpValue; }
    bool awardXp(int32_t x) override { xp.push_back(x); return true; }
    bool spawnVisualProjectile(const cc::ProjectileSpawn& p) override { projectiles.push_back(p); return true; }
    bool spawnVisualExplosion(const cc::ExplosionEvent& e) override { explosions.push_back(e); return true; }
    bool applyHostility(const cc::HostilityEffect& e) override { hostility.push_back(e); return true; }
    bool vatsActive() override { return vats; }
};

struct Cluster;

struct Machine : ICombatNet {
    Cluster* cl = nullptr;
    PlayerId slot = 0;
    FakeWorld world;
    FakeEngine eng;
    std::unique_ptr<CombatSync> sync;
    void send(PlayerId peer, Bytes data) override;
};

struct Datagram { PlayerId from, to; Bytes data; uint64_t at; };

struct Cluster {
    std::vector<std::unique_ptr<Machine>> m;
    std::deque<Datagram> q;
    uint64_t now = 1000;
    uint64_t latency = 0;
    std::function<int(const Datagram&)> copies = [](const Datagram&) { return 1; };   // 0 drop, 2 duplicate
    uint64_t delivered = 0, sentTotal = 0;
    bool propagateDeaths = true;
    uint64_t deathLatency = 20;
    std::vector<std::pair<uint64_t, std::pair<PlayerId, uint32_t>>> pendingKills;   // when, (machine, actor id): ActorSync's death message arriving

    explicit Cluster(int n, const std::function<void(Config&)>& tweak = nullptr) {
        for (int i = 0; i < n; ++i) {
            auto mc = std::make_unique<Machine>();
            mc->cl = this; mc->slot = static_cast<PlayerId>(i);
            mc->world.me = mc->slot;
            mc->eng.w = &mc->world;
            cc::WeaponInfo r; r.range = 3000; r.maxDamage = 400; r.minIntervalMs = 0;
            mc->world.weapons[rifle()] = r;
            Config c; c.self = mc->slot; c.isHost = i == 0;
            if (tweak) tweak(c);
            mc->sync.reset(new CombatSync(c, mc->world, mc->eng, *mc));
            m.push_back(std::move(mc));
        }
        for (int i = 1; i < n; ++i) m[0]->sync->playerJoined(static_cast<PlayerId>(i));
        for (int i = 0; i < n; ++i) {   // everybody knows every player; positions spread along x
            for (int j = 0; j < n; ++j) {
                Rec r; r.who = cc::Entity::player(static_cast<PlayerId>(j)); r.owner = static_cast<PlayerId>(j);
                r.pos = {static_cast<float>(j) * 100, 0, 0}; r.health = r.maxHealth = 200;
                m[i]->world.recs[r.who] = r;
            }
            for (int j = 0; j < n; ++j) m[i]->world.present.push_back(static_cast<PlayerId>(j));
        }
        tick(0);
    }
    Machine& at(int i) { return *m[i]; }
    CombatSync& cs(int i) { return *m[i]->sync; }

    // an actor everybody knows, owned by `owner` at `epoch`
    Rec& addActor(uint32_t id, PlayerId owner, OwnerEpoch epoch, cc::Vec3 pos, int32_t hp = 100) {
        Rec r; r.who = cc::Entity::actor(id); r.owner = owner; r.epoch = epoch; r.pos = pos; r.health = r.maxHealth = hp;
        for (auto& mc : m) mc->world.recs[r.who] = r;
        return m[0]->world.recs[r.who];
    }
    void setOwner(uint32_t id, PlayerId owner, OwnerEpoch epoch, int only = -1) {
        for (size_t i = 0; i < m.size(); ++i) {
            if (only >= 0 && static_cast<int>(i) != only) continue;
            auto& r = m[i]->world.recs[cc::Entity::actor(id)];
            r.owner = owner; r.epoch = epoch; r.pending = false;
        }
    }
    void forAll(uint32_t id, const std::function<void(Rec&)>& f) { for (auto& mc : m) f(mc->world.recs[cc::Entity::actor(id)]); }

    EngineHit shot(int attackerMachine, uint32_t victimRef, float dmg, int loc = 0) {
        (void)attackerMachine;
        EngineHit h;
        h.attackerRef = kLocalPlayerRef; h.victimRef = victimRef; h.healthDmg = dmg; h.hitLocation = loc; h.weapon = rifle(); h.weaponRange = 3000;
        return h;
    }

    void deliverAll(int maxRounds = 200) {
        for (int round = 0; round < maxRounds && !q.empty(); ++round) {
            std::deque<Datagram> cur;
            cur.swap(q);
            for (auto& d : cur) {
                if (d.at > now) { q.push_back(d); continue; }
                const int n = copies(d);
                for (int i = 0; i < n; ++i) { ++delivered; m[d.to]->sync->receive(d.from, d.data.data(), d.data.size()); }
            }
            if (q.empty()) break;
            bool anyDue = false;
            for (auto& d : q) if (d.at <= now) anyDue = true;
            if (!anyDue) break;
        }
    }
    void pumpEngine() {   // the actor port: a death that happened on an owner reaches onActorDied and (as ActorSync would) the other machines
        for (auto& mc : m) {
            while (!mc->eng.deaths.empty()) {
                const uint32_t ref = mc->eng.deaths.front();
                mc->eng.deaths.pop_front();
                mc->sync->onActorDied(ref, true, false, true, 0);
                if (propagateDeaths) pendingKills.push_back({now + deathLatency, {mc->slot, ref - 0x1000}});
            }
        }
        for (size_t i = 0; i < pendingKills.size();) {
            if (pendingKills[i].first > now) { ++i; continue; }
            const PlayerId from = pendingKills[i].second.first; const uint32_t id = pendingKills[i].second.second;
            pendingKills.erase(pendingKills.begin() + static_cast<long>(i));
            for (auto& mc : m) {
                if (mc->slot == from) continue;
                auto it = mc->world.recs.find(cc::Entity::actor(id));
                if (it == mc->world.recs.end() || !it->second.alive) continue;
                it->second.alive = false; it->second.health = 0;
                mc->sync->onActorDied(refOfActor(id), true, true, false, 0);
            }
        }
    }
    void tick(uint64_t adv = 10) {
        now += adv;
        for (auto& mc : m) mc->sync->tick(now);
        pumpEngine();
        deliverAll();
        pumpEngine();
        deliverAll();
    }
    void run(uint64_t ms, uint64_t step = 10) { for (uint64_t t = 0; t < ms; t += step) tick(step); }
};

inline void Machine::send(PlayerId peer, Bytes data) {
    ++cl->sentTotal;
    cl->q.push_back({slot, peer, std::move(data), cl->now + cl->latency});
}

inline int totalDamageCalls(Cluster& c) { int n = 0; for (auto& mc : c.m) n += static_cast<int>(mc->eng.damage.size()); return n; }

}  // namespace ctest
