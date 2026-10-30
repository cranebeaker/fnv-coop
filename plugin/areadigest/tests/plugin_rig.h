// Test rig for plugin/areadigest (game-free): a fake actor facade, session shadows, and a host + client pair of AreaServices connected through
// the in-memory network and real MuxTransports (the channel registry the dll uses).
#pragma once
#include <cmath>
#include <map>
#include <set>
#include <memory>
#include <string>
#include <vector>
#include "areadigest/actor_source.h"
#include "areadigest/area_runtime.h"
#include "areadigest/area_service.h"
#include "fnvcoop/shadow.h"
#include "fnvcoop/transport.h"
#include "fnvcoop/transport_util.h"
#include "minitest.h"

namespace nvat {

using namespace nvcoop::areadigest;
namespace ad = fnvcoop::areadigest;
using fnvcoop::FormRef;
using fnvcoop::PlayerId;

inline Vec3 V(float x = 0, float y = 0, float z = 0) { Vec3 v; v.x = x; v.y = y; v.z = z; return v; }
constexpr float kU = 70.0f;

class FakeFacade : public IActorFacade {
public:
    struct A { ActorRead r; OwnerView own; };
    std::map<uint32_t, A> a;
    bool isReady = true;
    bool havePlayerCell = true;
    CellId playerCell_ = 0x100;
    bool resyncOk = true;
    bool failMoves = false;
    bool ignoreHealth = false;            // setHealth says yes and changes nothing (an engine that keeps its own value)
    std::vector<uint32_t> resyncs;
    std::vector<std::string> calls;       // "hp 5 40", "pos 5", "cell 5 256"
    std::set<uint32_t> held;              // copies the actor port holds in place while they get out of furniture (the pose lock)

    void add(uint32_t id, CellId cell, Vec3 pos, float hp = 100, bool alive = true, bool named = false, PlayerId owner = 0, uint32_t epoch = 1) {
        A x;
        x.r.netId = id; x.r.cell = cell; x.r.pos = pos; x.r.hp = hp; x.r.alive = alive; x.r.named = named;
        x.own.known = true; x.own.owner = owner; x.own.epoch = epoch;
        a[id] = x;
    }
    bool ready() override { return isReady; }
    void tracked(std::vector<ActorRead>& out, size_t max) override {
        for (auto& kv : a) {
            if (out.size() >= max) break;
            out.push_back(kv.second.r);
        }
    }
    bool read(uint32_t id, ActorRead& out) override {
        auto i = a.find(id);
        if (i == a.end()) return false;
        out = i->second.r;
        return true;
    }
    bool owner(uint32_t id, OwnerView& out) override {
        auto i = a.find(id);
        if (i == a.end()) return false;
        out = i->second.own;
        return true;
    }
    bool playerCell(CellId& out) override { out = playerCell_; return havePlayerCell; }
    bool setHealth(uint32_t id, float hp) override {
        auto i = a.find(id);
        if (i == a.end()) return false;
        if (!ignoreHealth) i->second.r.hp = hp;
        calls.push_back("hp " + std::to_string(id) + " " + std::to_string(static_cast<int>(hp)));
        return true;
    }
    bool setPos(uint32_t id, const Vec3& p) override {
        auto i = a.find(id);
        if (i == a.end() || failMoves) return false;
        i->second.r.pos = p;
        calls.push_back("pos " + std::to_string(id));
        return true;
    }
    bool moveToCell(uint32_t id, CellId cell, const Vec3& p) override {
        auto i = a.find(id);
        if (i == a.end() || failMoves) return false;
        i->second.r.cell = cell;
        i->second.r.pos = p;
        calls.push_back("cell " + std::to_string(id) + " " + std::to_string(cell));
        return true;
    }
    bool moveHeld(uint32_t id) override { return held.count(id) != 0; }
    bool requestOwnerResync(uint32_t id) override {
        resyncs.push_back(id);
        return resyncOk;
    }
};

// A session shadow with its seq and epoch, the way ShadowStorySource wants it.
struct FakeWorld {
    fnvcoop::ShadowState st;
    uint64_t seq = 0;
    uint32_t epoch = 1;
    uint64_t repairs = 0;
    std::vector<fnvcoop::PlayerId> repairAsked;
    void stage(uint32_t quest, uint32_t stage) {
        fnvcoop::EffectTxn t;
        t.seq = ++seq;
        t.effects.push_back(fnvcoop::SetStage{FormRef("falloutnv.esm", quest), stage});
        st.applyTxn(t);
    }
    void setLock(uint32_t ref, bool locked, int level) {
        fnvcoop::EffectTxn t;
        t.seq = ++seq;
        t.effects.push_back(fnvcoop::SetLock{FormRef("falloutnv.esm", ref), locked, level});
        st.applyTxn(t);
    }
    void addItem(uint32_t ref, uint32_t item, int count) {
        fnvcoop::EffectTxn t;
        t.seq = ++seq;
        fnvcoop::RefAddItem e;
        e.ref = FormRef("falloutnv.esm", ref);
        e.item = FormRef("falloutnv.esm", item);
        e.count = count;
        t.effects.push_back(e);
        st.applyTxn(t);
    }
    ServiceInputs inputs(IActorFacade* f, fnvcoop::flight::FlightRecorder* rec = nullptr) {
        ServiceInputs in;
        in.actors = f;
        in.world = [this](ShadowStorySource::World& w) { w.state = &st; w.seq = seq; w.epoch = epoch; return true; };
        in.repair = [this](fnvcoop::PlayerId p) { repairAsked.push_back(p); };
        in.repairCount = [this]() { return repairs; };
        in.flight = rec;
        return in;
    }
};

// Host (slot 0, peer id 0) and one client (slot 1, peer id 1) over the in-memory network.
struct Pair {
    fnvcoop::InMemoryNetwork net{7};
    fnvcoop::MuxTransport hostMux{*net.endpoint(0)};
    fnvcoop::MuxTransport clientMux{*net.endpoint(1)};
    FakeFacade hf, cf;
    FakeWorld hw, cw;
    std::unique_ptr<AreaService> host, client;
    uint64_t now = 20000;
    bool hostLinkUp = true;
    bool clientActive = true;
    bool clientListed = true;             // the host's client list contains slot 1
    fnvcoop::MuxTransport* clientMuxOverride = nullptr;

    explicit Pair(AreaSettings s = AreaSettings(), fnvcoop::flight::FlightRecorder* hostRec = nullptr, fnvcoop::flight::FlightRecorder* clientRec = nullptr) {
        s.enabled = true;
        host.reset(new AreaService(s, hw.inputs(&hf, hostRec)));
        client.reset(new AreaService(s, cw.inputs(&cf, clientRec)));
    }
    void step(uint64_t ms) {
        now += ms;
        net.advance(now);
        fnvcoop::PeerId f;
        fnvcoop::Bytes d;
        while (hostMux.poll(f, d)) {}
        while ((clientMuxOverride ? *clientMuxOverride : clientMux).poll(f, d)) {}
        FrameInfo h;
        h.active = true; h.isHost = true; h.self = fnvcoop::kHostPlayer; h.mux = &hostMux; h.nowMs = now;
        ClientLink l;
        l.slot = 1; l.peer = 1; l.joined = true;
        if (clientListed) h.clients.push_back(l);
        host->frame(h);
        FrameInfo c;
        c.active = clientActive; c.isHost = false; c.self = 1; c.hostLinkUp = hostLinkUp; c.mux = clientMuxOverride ? clientMuxOverride : &clientMux; c.nowMs = now;
        client->frame(c);
    }
    void run(uint64_t ms, uint64_t stepMs = 50) {
        for (uint64_t t = 0; t < ms; t += stepMs) step(stepMs);
    }
    // the same actor on both machines; owner slot as each machine believes it
    void actor(uint32_t id, CellId cell, Vec3 pos, float hp = 100, PlayerId owner = 0, bool named = false) {
        hf.add(id, cell, pos, hp, true, named, owner);
        cf.add(id, cell, pos, hp, true, named, owner);
    }
};

}  // namespace nvat
