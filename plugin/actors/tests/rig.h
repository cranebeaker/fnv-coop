// Test rig for plugin/actors: a fake engine per machine (implements IActorPort), a lossy in-memory net (implements IActorNet) and a
// multi-machine Rig that advances a virtual clock. Machine 0 is the host. No threads, no wall clock.
#pragma once
#include <cmath>
#include <functional>
#include <map>
#include <memory>
#include <set>
#include <sstream>
#include <string>
#include <vector>
#include "actors/actor_runtime.h"
#include "actors/actor_sync.h"
#include "fnvcoop/rng.h"

namespace nva_test {

using namespace nvcoop::actors;

struct FakeActor {
    bool real = false, standIn = false;
    live::Vec3 pos, rot, vel;
    CellId cell = 0;
    bool ai = true;                 // an engine actor runs its native AI until told otherwise
    bool prot = false, dead = false;
    FormRef base;
    std::map<uint8_t, int32_t> av;
    float vx = 0, vy = 0;           // what the native AI does while ai is on (a straight walk)
    int placeCalls = 0, drives = 0, standInSpawns = 0, standInRemoves = 0, aiOn = 0, aiOff = 0, kills = 0, revives = 0, baseSets = 0, protOn = 0, protOff = 0;
    live::ActorSnapshot lastPlaced;
    std::map<FormRef, int32_t> inv;   // inventory totals
    int invWrites = 0;
    std::set<FormRef> equip;          // what it holds and wears
    int equipWrites = 0;
};

class FakeEngine : public IActorPort {
public:
    std::map<NetId, FakeActor> a;
    std::set<CellId> cellsLoaded;
    std::vector<std::string> ops;   // "place:100", "ai1:100", "ai0:100", "spawn:100", "remove:100", "kill:100", "base:100", "prot1:100"...

    void addReal(NetId id, CellId cell, live::Vec3 pos, const FormRef& base, float vx = 0, float vy = 0) {
        FakeActor& f = a[id];
        f.real = true; f.cell = cell; f.pos = pos; f.base = base; f.vx = vx; f.vy = vy; f.ai = true;
        f.av[0] = 100;   // health
        cellsLoaded.insert(cell);
    }
    int count(const std::string& op, NetId id) const {
        int n = 0;
        const std::string want = op + ":" + std::to_string(id);
        for (auto& o : ops) if (o == want) ++n;
        return n;
    }
    std::vector<std::string> opsFor(NetId id) const {
        std::vector<std::string> v;
        const std::string suffix = ":" + std::to_string(id);
        for (auto& o : ops) if (o.size() > suffix.size() && o.compare(o.size() - suffix.size(), suffix.size(), suffix) == 0) v.push_back(o);
        return v;
    }
    void simulate(uint32_t dtMs) {
        for (auto& kv : a) {
            FakeActor& f = kv.second;
            if (!f.real || f.dead || !f.ai) continue;
            f.pos.x += f.vx * dtMs / 1000.0f; f.pos.y += f.vy * dtMs / 1000.0f;
            f.vel.x = f.vx; f.vel.y = f.vy;
        }
    }

    // ---- IActorPort ----
    bool loaded(NetId id) override { auto it = a.find(id); return it != a.end() && (it->second.real || it->second.standIn); }
    bool cellLoaded(CellId c) override { return cellsLoaded.count(c) != 0; }
    bool sample(NetId id, live::ActorSnapshot& out) override {
        auto it = a.find(id);
        if (it == a.end() || !(it->second.real || it->second.standIn)) return false;
        const FakeActor& f = it->second;
        out = live::ActorSnapshot{};
        out.netId = id; out.cell = f.cell; out.pos = f.pos; out.rot = f.rot; out.vel = f.vel;
        for (auto& kv : f.av) { live::AvValue v; v.av = kv.first; v.value = kv.second; out.avDeltas.push_back(v); }
        return true;
    }
    bool pose(NetId id, live::ActualPose& out) override {
        auto it = a.find(id);
        if (it == a.end() || !(it->second.real || it->second.standIn)) return false;
        out.cell = it->second.cell; out.pos = it->second.pos;
        return true;
    }
    void drive(NetId id, const live::InterpOut& o) override {
        FakeActor& f = a[id];
        f.pos = o.pos; f.rot = o.rot; f.vel = o.vel; f.cell = o.cell; ++f.drives;
    }
    void applyAv(NetId id, const std::vector<live::AvValue>& v) override { for (auto& x : v) a[id].av[x.av] = x.value; }
    void place(NetId id, const live::ActorSnapshot& s) override {
        FakeActor& f = a[id];
        f.pos = s.pos; f.rot = s.rot; f.vel = s.vel; f.cell = s.cell; f.lastPlaced = s;
        for (auto& v : s.avDeltas) f.av[v.av] = v.value;
        ++f.placeCalls;
        ops.push_back("place:" + std::to_string(id));
    }
    void setAi(NetId id, bool on) override {
        FakeActor& f = a[id];
        f.ai = on;
        if (on) ++f.aiOn; else ++f.aiOff;
        ops.push_back(std::string(on ? "ai1:" : "ai0:") + std::to_string(id));
    }
    void setProtected(NetId id, bool on) override {
        FakeActor& f = a[id];
        f.prot = on;
        if (on) ++f.protOn; else ++f.protOff;
        ops.push_back(std::string(on ? "prot1:" : "prot0:") + std::to_string(id));
    }
    bool spawnStandIn(const ActorMeta& m, const live::ActorSnapshot& at) override {
        FakeActor& f = a[m.netId];
        f.standIn = true; f.pos = at.pos; f.cell = at.cell; f.base = m.base; f.ai = false; ++f.standInSpawns;
        ops.push_back("spawn:" + std::to_string(m.netId));
        return true;
    }
    void removeStandIn(NetId id) override {
        FakeActor& f = a[id];
        f.standIn = false; ++f.standInRemoves;
        ops.push_back("remove:" + std::to_string(id));
    }
    void setBase(NetId id, const FormRef& pick) override {
        FakeActor& f = a[id];
        f.base = pick; ++f.baseSets;
        ops.push_back("base:" + std::to_string(id));
    }
    void kill(NetId id) override { FakeActor& f = a[id]; f.dead = true; ++f.kills; ops.push_back("kill:" + std::to_string(id)); }
    void revive(NetId id) override { FakeActor& f = a[id]; f.dead = false; ++f.revives; ops.push_back("revive:" + std::to_string(id)); }
    bool refuseInventory = false;
    bool sampleInventory(NetId id, InvList& out) override {
        auto it = a.find(id);
        if (it == a.end() || !(it->second.real || it->second.standIn)) return false;
        out.clear();
        for (const auto& kv : it->second.inv) if (kv.second > 0) out.push_back({kv.first, kv.second});
        return true;
    }
    bool applyInventory(NetId id, const InvList& want) override {
        auto it = a.find(id);
        if (it == a.end() || !(it->second.real || it->second.standIn) || refuseInventory) return false;
        it->second.inv.clear();
        for (const auto& w : want) it->second.inv[w.form] = w.count;
        ++it->second.invWrites;
        return true;
    }
    bool refuseEquipment = false;
    bool sampleEquipment(NetId id, EquipList& out) override {
        auto it = a.find(id);
        if (it == a.end() || !(it->second.real || it->second.standIn)) return false;
        out.assign(it->second.equip.begin(), it->second.equip.end());
        return true;
    }
    bool applyEquipment(NetId id, const EquipList& want) override {
        auto it = a.find(id);
        if (it == a.end() || !(it->second.real || it->second.standIn) || refuseEquipment) return false;
        it->second.equip = std::set<FormRef>(want.begin(), want.end());
        ++it->second.equipWrites;
        return true;
    }
};

struct NetOpts {
    uint32_t latencyMs = 20;
    uint32_t jitterMs = 0;            // unreliable only
    double lossUnreliable = 0, dupUnreliable = 0;
};

struct NetMsg { uint64_t at = 0, order = 0; int from = 0, to = 0; Bytes data; bool reliable = true; };

class SimNet {
public:
    NetOpts o;
    fnvcoop::Rng rng{7};
    uint64_t now = 0, order = 0;
    std::vector<NetMsg> q;
    std::map<std::pair<int, int>, uint64_t> lastReliableAt;
    std::set<int> down;                           // machines cut off from the net
    std::vector<NetMsg> captured;                 // every reliable datagram ever sent (for replay tests)
    std::function<bool(const NetMsg&)> dropIf;    // test hook: true = drop this datagram
    std::function<uint32_t(const NetMsg&)> extraDelay;   // test hook: extra latency (ms) for a reliable datagram; order per link is kept (a slow link)
    uint64_t sentReliable = 0, sentUnreliable = 0, lost = 0;

    void send(int from, int to, bool reliable, const Bytes& data) {
        NetMsg m;
        m.from = from; m.to = to; m.data = data; m.reliable = reliable; m.order = order++;
        if (down.count(from) || down.count(to)) { ++lost; return; }
        if (dropIf && dropIf(m)) { ++lost; return; }
        if (reliable) {
            ++sentReliable;
            uint64_t at = now + o.latencyMs + (extraDelay ? extraDelay(m) : 0u);
            uint64_t& last = lastReliableAt[{from, to}];
            if (at < last) at = last;
            last = at;
            m.at = at;
            captured.push_back(m);
            q.push_back(m);
            return;
        }
        ++sentUnreliable;
        if (o.lossUnreliable > 0 && rng.chance(o.lossUnreliable)) { ++lost; return; }
        const int copies = (o.dupUnreliable > 0 && rng.chance(o.dupUnreliable)) ? 2 : 1;
        for (int i = 0; i < copies; ++i) {
            NetMsg c = m;
            c.at = now + o.latencyMs + (o.jitterMs ? rng.range(0, o.jitterMs) : 0);
            c.order = order++;
            q.push_back(c);
        }
    }
    // Delivers everything due at `t`, in (time, order) order.
    template <class F> void deliver(uint64_t t, F&& f) {
        now = t;
        for (;;) {
            size_t best = q.size();
            for (size_t i = 0; i < q.size(); ++i)
                if (q[i].at <= t && (best == q.size() || q[i].at < q[best].at || (q[i].at == q[best].at && q[i].order < q[best].order))) best = i;
            if (best == q.size()) break;
            NetMsg m = std::move(q[best]);
            q.erase(q.begin() + static_cast<long>(best));
            if (down.count(m.from) || down.count(m.to)) { ++lost; continue; }
            f(m);
        }
    }
};

class Endpoint : public IActorNet {
public:
    Endpoint(SimNet& n, int self) : net_(n), self_(self) {}
    void send(PlayerId peer, bool reliable, Bytes data) override { net_.send(self_, peer, reliable, data); }
private:
    SimNet& net_;
    int self_;
};

struct Machine {
    int slot = 0;
    FakeEngine eng;
    std::unique_ptr<Endpoint> ep;
    std::unique_ptr<ActorSync> sync;
};

inline FormRef refOf(uint32_t localId) { return FormRef("falloutnv.esm", localId); }
inline live::Vec3 V(float x, float y = 0, float z = 0) { live::Vec3 v; v.x = x; v.y = y; v.z = z; return v; }

class Rig {
public:
    SimNet net;
    std::vector<std::unique_ptr<Machine>> m;
    uint64_t now = 1000;
    PlayerPose pose[fnvcoop::kMaxSlots];
    std::vector<std::string> errors;   // invariant violations seen by step()
    bool checkInvariants = true;

    explicit Rig(int n, uint64_t seed = 1, NetOpts opts = {}, SyncConfig base = {}) {
        net.o = opts; net.rng.reseed(seed); net.now = now;
        base.adjacent = [](CellId a, CellId b) { return a == b || a + 1 == b || b + 1 == a; };   // cells are a row; neighbours touch
        for (int i = 0; i < n; ++i) {
            auto mc = std::make_unique<Machine>();
            mc->slot = i;
            mc->ep = std::make_unique<Endpoint>(net, i);
            SyncConfig c = base;
            c.self = static_cast<PlayerId>(i); c.isHost = i == 0;
            mc->sync = std::make_unique<ActorSync>(c, mc->eng, *mc->ep);
            m.push_back(std::move(mc));
        }
        for (int i = 1; i < n; ++i) { m[0]->sync->playerJoined(static_cast<PlayerId>(i)); m[static_cast<size_t>(i)]->sync->hostConnected(); }
    }
    Machine& host() { return *m[0]; }
    Machine& at(int i) { return *m[static_cast<size_t>(i)]; }
    int n() const { return static_cast<int>(m.size()); }

    // The actor exists in the plugin data on every machine in `mask` (bit i = machine i); cells it lives in count as loaded there.
    void addActor(uint32_t id, CellId cell, live::Vec3 pos, bool named = false, uint32_t mask = ~0u, float vx = 0, float vy = 0,
                  const FormRef& base = FormRef("falloutnv.esm", 0x500)) {
        for (int i = 0; i < n(); ++i) {
            if (!((mask >> i) & 1u)) continue;
            Machine& mc = at(i);
            mc.eng.addReal(id, cell, pos, base, vx, vy);
            ActorMeta meta;
            meta.netId = id; meta.ref = refOf(id); meta.base = base; meta.named = named; meta.cell = cell; meta.pos = pos;
            mc.sync->track(meta);
        }
    }
    void setPose(int slot, CellId cell, live::Vec3 pos, bool present = true) {
        PlayerPose p;
        p.present = present; p.cell = cell; p.pos = pos;
        if (present) { p.adjacent.push_back(cell - 1); p.adjacent.push_back(cell + 1); }
        pose[slot] = p;
    }
    void feedPoses() {
        for (auto& mc : m)
            for (int s = 0; s < n(); ++s) mc->sync->setPlayerPose(static_cast<PlayerId>(s), pose[s]);
    }
    void step(uint32_t dt = 10) {
        now += dt;
        feedPoses();
        net.deliver(now, [&](const NetMsg& msg) {
            Machine& dst = at(msg.to);
            dst.sync->receive(static_cast<PlayerId>(msg.from), msg.data.data(), msg.data.size());
        });
        for (auto& mc : m) { mc->eng.simulate(dt); mc->sync->tick(now); }
        if (checkInvariants) checkStep();
    }
    void run(uint32_t ms, uint32_t dt = 10) { for (uint32_t t = 0; t < ms; t += dt) step(dt); }
    bool runUntil(const std::function<bool()>& pred, uint32_t maxMs, uint32_t dt = 10) {
        for (uint32_t t = 0; t < maxMs; t += dt) { if (pred()) return true; step(dt); }
        return pred();
    }

    int aiCount(NetId id) {
        int c = 0;
        for (auto& mc : m) { if (net.down.count(mc->slot)) continue; auto it = mc->eng.a.find(id); if (it != mc->eng.a.end() && it->second.real && it->second.ai && !it->second.dead) ++c; }
        return c;
    }
    // Machine whose committed view says it owns `id` and that runs its AI (-1 = none).
    int aiMachine(NetId id) {
        for (int i = 0; i < n(); ++i) { if (net.down.count(i)) continue; auto it = at(i).eng.a.find(id); if (it != at(i).eng.a.end() && it->second.real && it->second.ai && !it->second.dead) return i; }
        return -1;
    }
    // After the system has been quiet: every machine agrees on (owner, epoch) and the AI runs exactly on the owner.
    std::string agreement(NetId id) {
        std::ostringstream err;
        const OwnerInfo h = host().sync->ownerOf(id);
        for (int i = 0; i < n(); ++i) {
            if (net.down.count(i)) continue;
            const OwnerInfo o = at(i).sync->ownerOf(id);
            if (!o.known) { err << "machine " << i << " does not know actor " << id << "; "; continue; }
            if (o.owner != h.owner || o.epoch != h.epoch) err << "machine " << i << " sees owner " << (int)o.owner << " epoch " << o.epoch << " but host has " << (int)h.owner << "/" << h.epoch << "; ";
        }
        return err.str();
    }

    void checkStep() {
        for (int i = 0; i < n(); ++i) {
            if (net.down.count(i)) continue;
            if (at(i).sync->stats().publishedNotOwner) errors.push_back("machine " + std::to_string(i) + " published without owning");
            for (auto& kv : at(i).eng.a) {
                const NetId id = kv.first;
                if (aiCount(id) > 1) errors.push_back("actor " + std::to_string(id) + " has AI running on " + std::to_string(aiCount(id)) + " machines at t=" + std::to_string(now));
                const OwnerInfo o = at(i).sync->ownerOf(id);
                if (!o.known) continue;
                if (o.pending && (o.mine || o.pendingTo == o.self) && kv.second.real && kv.second.ai && !kv.second.dead)
                    errors.push_back("machine " + std::to_string(i) + " runs AI for " + std::to_string(id) + " during a handoff at t=" + std::to_string(now));
            }
        }
    }
};

}  // namespace nva_test
