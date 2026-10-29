// Test rig for plugin/markers: 1..4 machines (machine 0 is the host) with a fake game each, joined by an in-memory order channel with latency,
// and a fake rules engine on the host (downed list, revive window, respawn at the end of it).
#pragma once
#include <algorithm>
#include <cmath>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <vector>

#include "mk_service.h"

namespace mk_test {

using namespace nvmk;

constexpr uint32_t kWorld = 0x3C;      // the wasteland
constexpr uint32_t kOtherWorld = 0x77;

inline Place outdoors(float x, float y, float z = 0, uint32_t world = kWorld) {
    Place p;
    p.valid = true;
    p.world = world;
    p.x = x; p.y = y; p.z = z;
    p.name = world == kWorld ? "Mojave Wasteland" : "Zion";
    return p;
}
inline Place indoors(uint32_t cell, float x = 0, float y = 0, const char* name = "Vault 21") {
    Place p;
    p.valid = true;
    p.world = 0;
    p.cell = cell;
    p.x = x; p.y = y;
    p.name = name;
    return p;
}

struct Rig;

struct Machine : IMarkerPort, IMarkerHud {
    Rig* rig = nullptr;
    int idx = 0;
    LocalState local;
    std::unique_ptr<MarkerService> svc;
    HudFrame frame;
    std::vector<std::string> notes;       // this machine's HUD lines
    int frames = 0;
    bool autoPuppets = true;              // a friend within 7000 units in the same space has a puppet here

    Slot localSlot() override { return static_cast<Slot>(idx); }
    bool isHost() override { return idx == 0; }
    bool readLocal(LocalState& out) override { out = local; return local.valid; }
    bool puppetPlace(Slot s, Place& out) override;
    std::vector<Slot> guests() override;
    bool sendToHost(const std::string& json) override;
    bool sendToGuest(Slot s, const std::string& json) override;
    void notify(const std::string& t) override { notes.push_back(t); }
    void notifyGuest(Slot s, const std::string& t) override;
    bool ruleDowned(Slot s) override;
    bool ruleCheckpointOnly(Slot s) override;
    uint32_t reviveWindowMs() override;
    void reviveInPlace(Slot s, float frac, const std::string& by) override;
    void show(const HudFrame& f) override { frame = f; ++frames; }
};

struct Rig {
    uint64_t now = 100000;
    uint32_t latencyMs = 60;
    std::vector<std::unique_ptr<Machine>> m;
    struct Msg { uint64_t due; int from; int to; std::string json; bool note; };
    std::vector<Msg> q;
    // the fake rules engine
    std::map<Slot, uint64_t> downedSince;
    std::set<Slot> ckpt;
    uint32_t windowMs = 60000;
    bool modeRespawn = false;
    struct Rev { Slot slot; float frac; std::string by; };
    std::vector<Rev> revivedLog;
    std::vector<Slot> respawnedLog;
    std::set<int> left;                   // machines that left the session
    std::set<int> frozen;                 // machines that stopped running (still joined, but silent)
    uint64_t sent = 0, dropped = 0;
    uint32_t dropEvery = 0;               // drop every n-th message (0 = none)

    explicit Rig(int n, ServiceConfig cfg = {}) {
        for (int i = 0; i < n; ++i) {
            auto mc = std::make_unique<Machine>();
            mc->rig = this;
            mc->idx = i;
            mc->local.valid = true;
            mc->local.name = i == 0 ? "Host" : (i == 1 ? "Alex" : (i == 2 ? "Bo" : "Cy"));
            mc->local.at = outdoors(1000.0f * i, 0);
            mc->local.alive = true;
            mc->svc = std::make_unique<MarkerService>(*mc, mc.get(), cfg);
            m.push_back(std::move(mc));
        }
    }

    void send(int from, int to, const std::string& json, bool note = false) {
        ++sent;
        if (dropEvery && sent % dropEvery == 0) { ++dropped; return; }
        q.push_back({now + latencyMs, from, to, json, note});
    }

    void deliver() {
        std::vector<Msg> due, rest;
        for (auto& x : q) (x.due <= now ? due : rest).push_back(x);
        q = rest;
        std::stable_sort(due.begin(), due.end(), [](const Msg& a, const Msg& b) { return a.due < b.due; });
        for (auto& x : due) {
            if (left.count(x.to)) continue;
            Machine& t = *m[x.to];
            if (x.note) { t.notes.push_back(x.json); continue; }
            if (x.to == 0) t.svc->onHostOrder(static_cast<Slot>(x.from), x.json, now);
            else t.svc->onGuestOrder(x.json, now);
        }
    }

    void rulesTick() {
        for (auto it = downedSince.begin(); it != downedSince.end();) {
            if (!ckpt.count(it->first) && now >= it->second + (modeRespawn ? 0 : windowMs)) {
                const Slot s = it->first;
                it = downedSince.erase(it);
                respawnedLog.push_back(s);
                m[s]->local.alive = true;
                m[s]->svc->onLocalUp("");
            } else {
                ++it;
            }
        }
    }

    void step(uint32_t ms) {
        for (uint32_t t = 0; t < ms; t += 50) {
            now += 50;
            rulesTick();
            deliver();
            for (size_t i = 0; i < m.size(); ++i)
                if (!left.count((int)i) && !frozen.count((int)i)) m[i]->svc->tick(now);
        }
    }

    // a player at 0 health: the rules engine downs him and his own machine is told (as the "downed" order does)
    void kill(int i, bool checkpointOnly = false) {
        downedSince[static_cast<Slot>(i)] = now;
        if (checkpointOnly) ckpt.insert(static_cast<Slot>(i));
        m[i]->local.alive = false;
        m[i]->svc->onLocalDowned(checkpointOnly || modeRespawn ? 0 : windowMs, now);
    }
};

inline bool Machine::puppetPlace(Slot s, Place& out) {
    if (!autoPuppets || s == idx || s >= rig->m.size() || rig->left.count(s)) return false;
    const Place& there = rig->m[s]->local.at;
    if (!sameSpace(local.at, there) || distance3(local.at, there) > 7000.0f) return false;
    out = there;
    return true;
}
inline std::vector<Slot> Machine::guests() {
    std::vector<Slot> g;
    for (size_t i = 1; i < rig->m.size(); ++i)
        if (!rig->left.count((int)i)) g.push_back(static_cast<Slot>(i));
    return g;
}
inline bool Machine::sendToHost(const std::string& json) { rig->send(idx, 0, json); return true; }
inline bool Machine::sendToGuest(Slot s, const std::string& json) { rig->send(0, s, json); return true; }
inline void Machine::notifyGuest(Slot s, const std::string& t) { rig->send(0, s, t, true); }
inline bool Machine::ruleDowned(Slot s) { return rig->downedSince.count(s) > 0; }
inline bool Machine::ruleCheckpointOnly(Slot s) { return rig->ckpt.count(s) > 0; }
inline uint32_t Machine::reviveWindowMs() { return rig->modeRespawn ? 0u : rig->windowMs; }
inline void Machine::reviveInPlace(Slot s, float frac, const std::string& by) {
    rig->downedSince.erase(s);
    rig->revivedLog.push_back({s, frac, by});
    rig->m[s]->local.alive = true;
    rig->m[s]->svc->onLocalUp(by);   // the "respawn" order reaches the machine (the rig is instant here; the order's own latency is not the point)
}

}  // namespace mk_test
