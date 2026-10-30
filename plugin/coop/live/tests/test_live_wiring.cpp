// Live-layer wiring tests (dll-wiring): ini switches, LiveHub lifecycle + channel routing, the avatar network service on a loopback of
// three machines, and the avatar wire codec. Game-free. Built once with FNVCOOP_RUNTIME_MUT; run once for real and once per mutation
// twin (FNVCOOP_MUT=<n>, tools/mut/run_twins.py) - every twin MUST make this binary fail.
//   1..4   switches        11..23 hub        31..33 wire        41..51 avatar net        61..64 [actors] ini
//   1170..1178 player entities ([entities] parts / stream, plugin/coop/live and plugin/avatar/look_codec.cpp)
#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <functional>
#include <new>
#include <stdexcept>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <tuple>
#include <vector>

#include "actors/actor_runtime.h"
#include "avatar_net.h"
#include "avatar_runtime_side.h"
#include "avatar_wire.h"
#include "live_hub.h"
#include "look_text.h"
#include "live_switches.h"
#ifdef NVLIVE_HAVE_ACTOR_API
#include "live_actors_cfg.h"
#endif
#include "testing/fake_backend.h"

using namespace nvcoop::livewire;
namespace live = fnvcoop::live;
namespace av = nvcoop::avatar;

static int g_fail = 0, g_checks = 0;
#define CHECK(c)                                                       \
    do {                                                               \
        ++g_checks;                                                    \
        if (!(c)) {                                                    \
            ++g_fail;                                                  \
            std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c); \
        }                                                              \
    } while (0)
#define TEST(name) static void name()
static void run(const char* n, void (*f)()) {
    int before = g_fail;
    f();
    std::printf("[%s] %s\n", g_fail == before ? " ok " : "FAIL", n);
}

// =============================================================================================================================
// switches
// =============================================================================================================================
static IniGetter fakeIni(std::map<std::string, std::string> kv) {
    return [kv](const char* section, const char* key, const char* def) {
        auto it = kv.find(std::string(section) + "." + key);
        return it == kv.end() ? std::string(def) : it->second;
    };
}

TEST(switches_default_off) {
    LiveSwitches s = readLiveSwitches(fakeIni({}));
    CHECK(!s.actors && !s.avatars && !s.combat && !s.spawns && !s.social);
    CHECK(!s.any());
    CHECK(s.warnings.empty());
    CHECK(describeSwitches(s) == "actors=0 avatars=0 combat=0 spawns=0");
}

TEST(switches_parse_values) {
    for (const char* on : {"1", "true", "TRUE", " on ", "Yes", "yes"}) CHECK(parseSwitchValue(on));
    for (const char* off : {"0", "", "false", "off", "no", "2", "enabled", "maybe", "  "}) CHECK(!parseSwitchValue(off));
    LiveSwitches s = readLiveSwitches(fakeIni({{"live.actors", "1"}, {"live.avatars", "on"}}));
    CHECK(s.actors && s.avatars && !s.combat && !s.spawns);
    CHECK(describeSwitches(s) == "actors=1 avatars=1 combat=0 spawns=0");
    // a typo must not switch anything on
    s = readLiveSwitches(fakeIni({{"live.actors", "ofF"}, {"live.avatars", "disabled"}, {"live.combat", "none"}}));
    CHECK(!s.any());
}

TEST(switches_dependencies) {
    LiveSwitches s = readLiveSwitches(fakeIni({{"live.combat", "1"}, {"live.spawns", "1"}}));
    CHECK(!s.combat && !s.spawns && !s.actors);
    CHECK(s.warnings.find("combat=1 ignored") != std::string::npos);
    CHECK(s.warnings.find("spawns=1 ignored") != std::string::npos);
    s = readLiveSwitches(fakeIni({{"live.actors", "1"}, {"live.combat", "1"}, {"live.spawns", "1"}, {"live.avatars", "1"}}));
    CHECK(s.actors && s.combat && s.spawns && s.avatars);
    CHECK(s.warnings.empty());
    s = readLiveSwitches(fakeIni({{"live.avatars", "1"}, {"live.combat", "1"}}));   // avatars need nobody
    CHECK(s.avatars && !s.combat);
    LiveSwitches r;
    r.combat = true;
    r = resolveSwitches(r);
    CHECK(!r.combat);
}

// [live] social (plugin/companions): off by default, needs actors, never switched on by another lane's switch. Twins 65 / 66 (and 4).
TEST(switches_social) {
    LiveSwitches s = readLiveSwitches(fakeIni({{"live.actors", "1"}, {"live.avatars", "1"}, {"live.combat", "1"}, {"live.spawns", "1"}}));
    CHECK(!s.social);                                                      // every other lane on: still off
    CHECK(describeSwitches(s) == "actors=1 avatars=1 combat=1 spawns=1");  // the line other tools match is unchanged while it is off
    s = readLiveSwitches(fakeIni({{"live.actors", "1"}, {"live.social", "1"}}));
    CHECK(s.social && s.actors && s.warnings.empty() && s.any());
    CHECK(describeSwitches(s) == "actors=1 avatars=0 combat=0 spawns=0 social=1");
    s = readLiveSwitches(fakeIni({{"live.social", "1"}}));                  // companions are owned actors: nobody owns them without actors=1
    CHECK(!s.social && !s.any());
    CHECK(s.warnings.find("social=1 ignored") != std::string::npos);
    s = readLiveSwitches(fakeIni({{"live.actors", "1"}, {"live.social", "typo"}}));
    CHECK(!s.social);
    LiveSwitches r;
    r.social = true;
    r = resolveSwitches(r);
    CHECK(!r.social);
}

// =============================================================================================================================
// hub
// =============================================================================================================================
struct Rec : ILiveService {
    std::string nm;
    std::vector<uint8_t> ch;
    std::vector<std::string>* shared = nullptr;     // optional common log across services
    std::vector<std::string> log;
    std::vector<std::pair<PlayerId, Bytes>> got;
    ILiveLink* link = nullptr;
    Rec(std::string n, std::vector<uint8_t> c, std::vector<std::string>* sh = nullptr) : nm(std::move(n)), ch(std::move(c)), shared(sh) {}
    const char* name() const override { return nm.c_str(); }
    std::vector<uint8_t> channels() const override { return ch; }
    void note(const std::string& s) { log.push_back(s); if (shared) shared->push_back(nm + ":" + s); }
    void sessionBegin(const SessionInfo& i, ILiveLink& l) override { link = &l; note(std::string("begin:") + (i.isHost ? "host" : "client") + ":" + std::to_string(i.self)); }
    void sessionEnd() override { note("end"); link = nullptr; }
    void playerJoined(PlayerId p) override { note("join:" + std::to_string(p)); }
    void playerLeft(PlayerId p) override { note("leave:" + std::to_string(p)); }
    void hostLinkUp() override { note("up"); }
    void hostLinkDown() override { note("down"); }
    void receive(PlayerId from, const uint8_t* d, size_t n) override { got.push_back({from, Bytes(d, d + n)}); note("rx:" + std::to_string(from)); }
    void tick(uint64_t) override { note("tick"); }
    void idleTick(uint64_t) override { note("idle"); }
    void loadBegin() override { note("loadBegin"); }
    void loadEnd() override { note("loadEnd"); }
    void saved() override { note("saved"); }
    void gameExit() override { note("exit"); }
    size_t count(const std::string& s) const { size_t n = 0; for (auto& l : log) n += l == s ? 1 : 0; return n; }
    bool has(const std::string& s) const { return count(s) != 0; }
};

struct Net3 {
    fnvcoop::InMemoryNetwork net{7};
    std::map<fnvcoop::PeerId, std::unique_ptr<fnvcoop::MuxTransport>> mux;
    uint64_t now = 0;
    fnvcoop::MuxTransport* m(fnvcoop::PeerId id) {
        auto& p = mux[id];
        if (!p) p.reset(new fnvcoop::MuxTransport(*net.endpoint(id)));
        return p.get();
    }
    void pump() {   // deliver everything due and let each mux dispatch its registered channels
        net.advance(now);
        for (auto& kv : mux) { fnvcoop::PeerId f; fnvcoop::Bytes d; while (kv.second->poll(f, d)) {} }
    }
};

static SessionView hostView(fnvcoop::MuxTransport* mux, std::vector<ClientLink> clients, uint64_t now) {
    SessionView v;
    v.active = true; v.isHost = true; v.self = kHostPlayer; v.mux = mux; v.clients = std::move(clients); v.nowMs = now;
    return v;
}
static SessionView clientView(fnvcoop::MuxTransport* mux, PlayerId self, bool up, uint64_t now) {
    SessionView v;
    v.active = true; v.isHost = false; v.self = self; v.hostLinkUp = up; v.mux = mux; v.nowMs = now;
    return v;
}
static ClientLink cl(PlayerId slot, fnvcoop::PeerId peer, bool joined = true) { ClientLink c; c.slot = slot; c.peer = peer; c.joined = joined; return c; }

TEST(hub_disabled_service_is_never_called) {
    Net3 n;
    LiveHub hub;
    Rec on("on", {0xA1}), off("off", {0xA2});
    CHECK(hub.add(&on, true));
    CHECK(hub.add(&off, false));
    CHECK(hub.enabledCount() == 1 && hub.serviceCount() == 2);
    hub.onLoadBegin();
    hub.onLoadEnd();
    hub.frame(hostView(n.m(0), {cl(1, 1)}, 10));
    hub.frame(hostView(n.m(0), {cl(1, 1)}, 20));
    hub.onSaved();
    hub.onGameExit();
    CHECK(off.log.empty());
    CHECK(on.has("begin:host:0") && on.has("loadBegin") && on.has("loadEnd") && on.has("saved") && on.has("exit"));
    // a datagram on the disabled service's channel is not delivered anywhere
    hub.frame(hostView(n.m(0), {cl(1, 1)}, 30));
    n.m(1)->sendEx(0, {0xA2, 1, 2, 3}, fnvcoop::ITransport::Channel::Reliable);
    n.pump();
    CHECK(off.got.empty() && on.got.empty());
}

TEST(hub_session_lifecycle_host) {
    Net3 n;
    LiveHub hub;
    std::vector<std::string> order;
    Rec a("a", {0xA1}, &order), b("b", {0xA2}, &order);
    hub.add(&a, true);
    hub.add(&b, true);
    CHECK(!hub.inSession());
    hub.frame(hostView(n.m(0), {}, 10));
    CHECK(hub.inSession());
    CHECK(a.count("begin:host:0") == 1 && b.count("begin:host:0") == 1);
    CHECK(a.count("tick") == 1);
    hub.frame(hostView(n.m(0), {cl(1, 1)}, 20));
    hub.frame(hostView(n.m(0), {cl(1, 1)}, 30));
    hub.frame(hostView(n.m(0), {cl(1, 1), cl(2, 2)}, 40));
    CHECK(a.count("join:1") == 1 && a.count("join:2") == 1 && b.count("join:1") == 1);   // each exactly once
    hub.frame(hostView(n.m(0), {cl(2, 2)}, 50));
    CHECK(a.count("leave:1") == 1 && b.count("leave:1") == 1 && a.count("leave:2") == 0);
    hub.frame(hostView(n.m(0), {cl(1, 1, false), cl(2, 2)}, 60));   // not joined = not a player
    CHECK(a.count("join:1") == 1);
    order.clear();
    SessionView off = hostView(n.m(0), {}, 70);
    off.active = false;
    hub.frame(off);
    CHECK(!hub.inSession());
    CHECK(a.count("end") == 1 && b.count("end") == 1);
    CHECK(order.size() >= 2 && order[0] == "b:end" && order[1] == "a:end");   // services close in reverse order
    hub.frame(off);
    CHECK(a.count("end") == 1);
    CHECK(hub.stats().sessionsBegun == 1 && hub.stats().sessionsEnded == 1);
}

TEST(hub_existing_clients_are_announced_after_begin) {
    Net3 n;
    LiveHub hub;
    std::vector<std::string> order;
    Rec a("a", {0xA1}, &order);
    hub.add(&a, true);
    hub.frame(hostView(n.m(0), {cl(1, 1), cl(2, 2)}, 10));
    CHECK(order.size() >= 3);
    CHECK(order[0] == "a:begin:host:0");
    CHECK(a.count("join:1") == 1 && a.count("join:2") == 1);
    // and a client that connects to a host link that is already up hears hostLinkUp after the begin
    LiveHub chub;
    std::vector<std::string> corder;
    Rec c("c", {0xA1}, &corder);
    chub.add(&c, true);
    chub.frame(clientView(n.m(1), 1, true, 10));
    CHECK(corder.size() >= 2 && corder[0] == "c:begin:client:1" && corder[1] == "c:up");
}

TEST(hub_client_link_events) {
    Net3 n;
    LiveHub hub;
    Rec c("c", {0xA1});
    hub.add(&c, true);
    hub.frame(clientView(n.m(1), kNoPlayer, false, 10));   // not joined, no link: no session
    CHECK(!hub.inSession() && c.log.empty() == false);     // (idle ticks only)
    CHECK(!c.has("begin:client:255"));
    hub.frame(clientView(n.m(1), 1, true, 20));
    CHECK(hub.inSession() && c.count("up") == 1);
    hub.frame(clientView(n.m(1), 1, false, 30));   // rebaseline: the host link drops, the session stays
    CHECK(hub.inSession() && c.count("down") == 1);
    hub.frame(clientView(n.m(1), 1, true, 40));
    CHECK(c.count("up") == 2 && c.count("end") == 0);
    CHECK(c.count("join:1") == 0);   // joins are host-only
}

TEST(hub_new_mux_is_a_new_session_and_old_handlers_are_inert) {
    Net3 n;
    LiveHub hub;
    Rec a("a", {0xA1});
    hub.add(&a, true);
    hub.frame(hostView(n.m(0), {cl(1, 1)}, 10));
    CHECK(a.count("begin:host:0") == 1);
    // the runtime was rebuilt: a new mux object (peer 5) replaces the old one (peer 0)
    hub.frame(hostView(n.m(5), {cl(1, 1)}, 20));
    CHECK(a.count("end") == 1 && a.count("begin:host:0") == 2);
    CHECK(a.count("join:1") == 2);   // the new session re-announces the roster
    CHECK(hub.stats().sessionsBegun == 2);
    // a datagram still arriving on the OLD mux must not reach the service
    n.m(1)->sendEx(0, {0xA1, 9, 9}, fnvcoop::ITransport::Channel::Reliable);
    n.pump();
    CHECK(a.got.empty());
    CHECK(hub.stats().droppedSessionOver == 1);
    // the new mux does deliver
    n.m(1)->sendEx(5, {0xA1, 7}, fnvcoop::ITransport::Channel::Reliable);
    n.pump();
    CHECK(a.got.size() == 1 && a.got[0].first == 1);
}

TEST(hub_routing_and_channel_ownership) {
    Net3 n;
    LiveHub hub;
    Rec a("a", {0xA1}), b("b", {0xA2, 0xA3}), dup("dup", {0xA3});
    CHECK(hub.add(&a, true));
    CHECK(hub.add(&b, true));
    CHECK(!hub.add(&dup, true));   // 0xA3 is taken
    CHECK(!hub.add(&a, true));     // already registered
    CHECK(!hub.add(nullptr, true));
    hub.frame(hostView(n.m(0), {cl(1, 1), cl(2, 2)}, 10));
    n.m(1)->sendEx(0, {0xA1, 1}, fnvcoop::ITransport::Channel::Reliable);
    n.m(2)->sendEx(0, {0xA3, 2, 2}, fnvcoop::ITransport::Channel::Unreliable);
    n.m(2)->sendEx(0, {0xA2, 3}, fnvcoop::ITransport::Channel::Reliable);
    n.pump();
    CHECK(a.got.size() == 1 && a.got[0].first == 1 && a.got[0].second.size() == 2);
    CHECK(b.got.size() == 2 && b.got[0].first == 2);
    CHECK(dup.got.empty());
    // a peer that is not a joined client is ignored
    hub.frame(hostView(n.m(0), {cl(1, 1), cl(2, 2, false)}, 20));
    n.m(2)->sendEx(0, {0xA1, 4}, fnvcoop::ITransport::Channel::Reliable);
    n.m(7)->sendEx(0, {0xA1, 5}, fnvcoop::ITransport::Channel::Reliable);
    n.pump();
    CHECK(a.got.size() == 1);
    CHECK(hub.stats().droppedUnknownPeer == 2);
    // a client hears the host as slot 0
    LiveHub chub;
    Rec c("c", {0xA1});
    chub.add(&c, true);
    chub.frame(clientView(n.m(1), 1, true, 30));
    n.m(0)->sendEx(1, {0xA1, 6}, fnvcoop::ITransport::Channel::Reliable);
    n.pump();
    CHECK(c.got.size() == 1 && c.got[0].first == kHostPlayer);
}

TEST(hub_send_paths) {
    Net3 n;
    LiveHub hub, chub;
    Rec h("h", {0xA1}), c("c", {0xA1});
    hub.add(&h, true);
    chub.add(&c, true);
    hub.frame(hostView(n.m(0), {cl(1, 1), cl(2, 2, false)}, 10));
    chub.frame(clientView(n.m(1), 1, true, 10));
    CHECK(h.link && c.link);
    CHECK(h.link->isHost() && !c.link->isHost());
    CHECK(h.link->self() == kHostPlayer && c.link->self() == 1);
    CHECK(h.link->send(1, true, {0xA1, 1}));        // to a joined client
    CHECK(!h.link->send(2, true, {0xA1, 2}));       // slot 2 has not joined
    CHECK(!h.link->send(3, true, {0xA1, 3}));       // nobody
    CHECK(!h.link->send(1, true, {}));              // empty
    CHECK(c.link->send(kNoPlayer, false, {0xA1, 4}));   // a client always sends to the host
    n.pump();
    CHECK(c.got.size() == 1 && c.got[0].first == kHostPlayer);
    CHECK(h.got.size() == 1 && h.got[0].first == 1);
    CHECK(hub.stats().sendsOk == 1 && hub.stats().sendsFailed == 3);
    // after the session ended the link is dead
    ILiveLink* old = h.link;
    SessionView off = hostView(n.m(0), {}, 20);
    off.active = false;
    hub.frame(off);
    (void)old;   // (the service dropped it in sessionEnd; the hub's own link object is gone)
    CHECK(h.link == nullptr);
}

TEST(hub_loads_and_idle) {
    Net3 n;
    LiveHub hub;
    Rec a("a", {0xA1});
    hub.add(&a, true);
    // no session at all
    SessionView idle;
    idle.nowMs = 5;
    hub.onLoadBegin();
    hub.frame(idle);
    hub.frame(idle);
    hub.onLoadEnd();
    CHECK(a.count("loadBegin") == 1 && a.count("loadEnd") == 1);   // purges happen without a coop session too
    CHECK(a.count("idle") == 2 && a.count("tick") == 0);
    hub.frame(hostView(n.m(0), {}, 10));
    CHECK(a.count("tick") == 1 && a.count("idle") == 2);
    hub.onLoadBegin();
    hub.onLoadEnd();
    CHECK(a.count("loadBegin") == 2 && a.count("loadEnd") == 2);
    hub.onSaved();
    CHECK(a.count("saved") == 1);
    hub.onGameExit();
    CHECK(!hub.inSession() && a.count("end") == 1 && a.count("exit") == 1);
}

TEST(hub_client_reslot_is_a_new_session) {
    Net3 n;
    LiveHub hub;
    Rec c("c", {0xA1});
    hub.add(&c, true);
    hub.frame(clientView(n.m(1), 1, true, 10));
    hub.frame(clientView(n.m(1), 1, true, 20));
    CHECK(c.count("begin:client:1") == 1);
    hub.frame(clientView(n.m(1), 2, true, 30));   // the host gave this client another slot
    CHECK(c.count("end") == 1 && c.count("begin:client:2") == 1);
    CHECK(c.count("up") == 2);   // the new session hears the link state again
}

struct Boom : Rec {
    std::string when;
    Boom(std::string w) : Rec("boom", {0xA3}), when(std::move(w)) {}
    void sessionBegin(const SessionInfo& i, ILiveLink& l) override { Rec::sessionBegin(i, l); if (when == "begin") throw 5; }
    void tick(uint64_t n) override { Rec::tick(n); if (when == "tick") throw std::runtime_error("tick exploded"); }
    void receive(PlayerId f, const uint8_t* d, size_t n) override { Rec::receive(f, d, n); if (when == "receive") throw std::runtime_error("rx exploded"); }
    void playerJoined(PlayerId p) override { Rec::playerJoined(p); if (when == "join") throw std::bad_alloc(); }
    void loadEnd() override { Rec::loadEnd(); if (when == "load") throw std::runtime_error("load exploded"); }
};

TEST(hub_a_throwing_service_is_switched_off_and_the_others_go_on) {
    for (const char* when : {"begin", "tick", "receive", "join", "load"}) {
        Net3 n;
        LiveHub hub;
        Rec good("good", {0xA1});
        Boom bad(when);
        hub.add(&good, true);
        hub.add(&bad, true);
        bool escaped = false;
        auto guarded = [&](const std::function<void()>& f) {   // (under the twin the exception would otherwise end the test binary)
            try {
                f();
            } catch (...) {
                escaped = true;
            }
        };
        guarded([&] { hub.frame(hostView(n.m(0), {cl(1, 1)}, 10)); });
        n.m(1)->sendEx(0, {0xA3, 1}, fnvcoop::ITransport::Channel::Reliable);
        guarded([&] { n.pump(); });
        guarded([&] { hub.onLoadBegin(); });
        guarded([&] { hub.onLoadEnd(); });
        guarded([&] { hub.frame(hostView(n.m(0), {cl(1, 1)}, 20)); });
        guarded([&] { hub.frame(hostView(n.m(0), {cl(1, 1)}, 30)); });
        CHECK(!escaped);                                   // nothing unwinds into the game's frame
        CHECK(good.count("tick") == 3 && good.count("loadEnd") == 1);   // the other lane never noticed
        CHECK(hub.stats().serviceFaults == 1);
        CHECK(hub.enabledCount() == 1);                    // the faulty one is off now
        const size_t calls = bad.log.size();
        guarded([&] { hub.frame(hostView(n.m(0), {cl(1, 1)}, 40)); });
        guarded([&] { hub.onLoadEnd(); });
        CHECK(bad.log.size() == calls);                    // and is never called again
        std::string j = hub.statusJson();
        CHECK(j.find("\"fault\":\"boom: ") != std::string::npos && j.find("\"serviceFaults\":1") != std::string::npos);
        SessionView off = hostView(n.m(0), {}, 50);
        off.active = false;
        guarded([&] { hub.frame(off); });                  // the session ends cleanly
        CHECK(!hub.inSession());
    }
}

TEST(hub_status_json) {
    Net3 n;
    LiveHub hub;
    Rec a("alpha", {0xA1}), b("beta", {0xA2});
    hub.add(&a, true);
    hub.add(&b, false);
    hub.frame(hostView(n.m(0), {cl(1, 1)}, 10));
    std::string j = hub.statusJson();
    CHECK(j.find("\"inSession\":true") != std::string::npos);
    CHECK(j.find("\"role\":\"host\"") != std::string::npos);
    CHECK(j.find("{\"name\":\"alpha\",\"enabled\":true,\"sessionOpen\":true}") != std::string::npos);
    CHECK(j.find("{\"name\":\"beta\",\"enabled\":false,\"sessionOpen\":false}") != std::string::npos);
    CHECK(j.find("\"joins\":1") != std::string::npos);
}

#ifdef NVLIVE_HAVE_ACTOR_API
// =============================================================================================================================
// [actors] ini -> actors::engine::Config
// =============================================================================================================================
static IniGetter actorsIni(std::map<std::string, std::string> kv) {
    return [kv](const char* section, const char* key, const char* def) {
        if (std::string(section) != "actors") return std::string(def);
        auto it = kv.find(key);
        return it == kv.end() ? std::string(def) : it->second;
    };
}

TEST(actors_cfg_defaults_and_overrides) {
    using nvcoop::actors::engine::Config;
    Config d;
    std::string notes;
    Config c = readActorsConfig(actorsIni({}), d, &notes);
    CHECK(notes.empty());
    CHECK(c.enabled == d.enabled && c.scanMs == d.scanMs && c.maxScanActors == d.maxScanActors && c.maxLinesPerFrame == d.maxLinesPerFrame);
    CHECK(c.driveMinMs == d.driveMinMs && c.posEpsilonXY == d.posEpsilonXY && c.persistLedger == d.persistLedger && c.maxStandIns == d.maxStandIns);
    c = readActorsConfig(actorsIni({{"engine", "0"}, {"scanMs", "250"}, {"maxScanActors", "100"}, {"maxLinesPerFrame", "8"}, {"driveMinMs", "50"},
                                    {"posEpsilonXY", "0.5"}, {"posEpsilonZ", "1.5"}, {"yawEpsilonDeg", "2"}, {"pollDeathMs", "200"},
                                    {"restrainNonOwned", "0"}, {"stopCombatOnRestrain", "0"}, {"startCombatOnTakeover", "off"},
                                    {"persistLedger", "0"}, {"maxStandIns", "10"}, {"swapLeveled", "0"}, {"saveGraceMs", "500"}, {"deathGraceMs", "900"}}),
                         d, &notes);
    CHECK(!c.enabled && c.scanMs == 250 && c.maxScanActors == 100 && c.maxLinesPerFrame == 8 && c.driveMinMs == 50);
    CHECK(c.posEpsilonXY == 0.5f && c.posEpsilonZ == 1.5f && c.yawEpsilonDeg == 2.0f && c.pollDeathMs == 200);
    CHECK(!c.restrainNonOwned && !c.stopCombatOnRestrain && !c.startCombatOnTakeover && !c.persistLedger && !c.swapLeveled);
    CHECK(c.maxStandIns == 10 && c.saveGraceMs == 500 && c.deathGraceMs == 900);
    CHECK(notes.empty());
    c = readActorsConfig(actorsIni({{"restrain", "0"}}), d, nullptr);   // the older spelling of restrainNonOwned
    CHECK(!c.restrainNonOwned);
    // the base passed in is the starting point, not the compiled-in default
    Config base;
    base.scanMs = 777;
    base.persistLedger = false;
    c = readActorsConfig(actorsIni({{"maxStandIns", "5"}}), base, nullptr);
    CHECK(c.scanMs == 777 && !c.persistLedger && c.maxStandIns == 5);
}

TEST(actors_cfg_clamps_and_rejects_nonsense) {
    using nvcoop::actors::engine::Config;
    Config d;
    std::string notes;
    Config c = readActorsConfig(actorsIni({{"scanMs", "0"}, {"maxLinesPerFrame", "100000"}, {"maxScanActors", "-5"}, {"pollDeathMs", "1"},
                                           {"maxStandIns", "99999"}, {"posEpsilonXY", "-3"}, {"yawEpsilonDeg", "1000"}, {"driveMinMs", "-1"}}),
                                d, &notes);
    CHECK(c.scanMs == 50);
    CHECK(c.maxLinesPerFrame == 200);
    CHECK(c.maxScanActors == 16);
    CHECK(c.pollDeathMs == 16);
    CHECK(c.maxStandIns == 400);
    CHECK(c.posEpsilonXY == 0.0f);
    CHECK(c.yawEpsilonDeg == 90.0f);
    CHECK(c.driveMinMs == 0);
    CHECK(!notes.empty() && notes.find("scanMs") != std::string::npos);
    // text that is not a number keeps the default and says so
    notes.clear();
    c = readActorsConfig(actorsIni({{"scanMs", "fast"}, {"posEpsilonZ", "1.5x"}, {"saveGraceMs", ""}}), d, &notes);
    CHECK(c.scanMs == d.scanMs && c.posEpsilonZ == d.posEpsilonZ && c.saveGraceMs == d.saveGraceMs);
    CHECK(notes.find("scanMs=fast is not a number") != std::string::npos && notes.find("posEpsilonZ") != std::string::npos);
}
#endif

// =============================================================================================================================
// avatar wire
// =============================================================================================================================
TEST(wire_roster_roundtrip_and_truncation) {
    AvBytes b = encodeRoster({0, 1, 3});
    std::vector<uint8_t> s;
    CHECK(decodeRoster(b.data(), b.size(), s));
    CHECK(s == std::vector<uint8_t>({0, 1, 3}));
    for (size_t n = 0; n < b.size(); ++n) CHECK(!decodeRoster(b.data(), n, s));   // every truncation is rejected
    AvBytes extra = b;
    extra.push_back(0x55);
    CHECK(!decodeRoster(extra.data(), extra.size(), s));                          // trailing garbage is rejected
    AvBytes bad = encodeRoster({0, 9});
    CHECK(!decodeRoster(bad.data(), bad.size(), s));                              // slot outside the table
    AvBytes wrongCh = b;
    wrongCh[0] = 0xAC;
    CHECK(!decodeRoster(wrongCh.data(), wrongCh.size(), s));
}

static AvLook sampleLook(uint8_t slot) {
    AvLook l;
    l.slot = slot;
    l.appearance.race = 0x19;
    l.appearance.female = true;
    l.appearance.hair = 0x1234;
    l.appearance.eyes = 0x55;
    l.appearance.faceHash = 0xDEADBEEF;
    l.appearance.name = "Courier Six";
    l.equipment = {{0x1000, false}, {0x2000, true}, {0xFF000123, false}};
    return l;
}

TEST(wire_look_roundtrip_and_truncation) {
    AvLook l = sampleLook(2);
    AvBytes b = encodeLook(l);
    AvLook o;
    CHECK(decodeLook(b.data(), b.size(), o));
    CHECK(o == l);
    for (size_t n = 0; n < b.size(); ++n) CHECK(!decodeLook(b.data(), n, o));
    AvBytes extra = b;
    extra.push_back(0);
    CHECK(!decodeLook(extra.data(), extra.size(), o));
    AvLook bad = l;
    bad.slot = 8;   // outside the slot table
    AvBytes bb = encodeLook(bad);
    CHECK(!decodeLook(bb.data(), bb.size(), o));
    // an equipment count above the cap, with enough bytes behind it to be "valid" otherwise
    AvLook many = l;
    many.equipment.clear();
    AvBytes mb = encodeLook(many);
    mb[mb.size() - 1] = 200;   // count byte
    for (int i = 0; i < 200; ++i) { for (int k = 0; k < 4; ++k) mb.push_back(0); mb.push_back(0); }
    CHECK(!decodeLook(mb.data(), mb.size(), o));
    // empty look and a name at the limit round-trip
    AvLook e;
    e.slot = 0;
    e.appearance.name = std::string(48, 'x');
    AvBytes eb = encodeLook(e);
    CHECK(decodeLook(eb.data(), eb.size(), o) && o == e);
    // random garbage never crashes and is rejected unless it happens to be valid
    std::vector<uint8_t> sink;
    uint32_t x = 12345;
    for (int i = 0; i < 3000; ++i) {
        AvBytes g(1 + (x % 90));
        for (auto& c : g) { x = x * 1664525u + 1013904223u; c = static_cast<uint8_t>(x >> 24); }
        g[0] = kChannelAvatars;
        if (g.size() > 1) g[1] = (i & 1) ? kAvKindLook : kAvKindRoster;
        decodeLook(g.data(), g.size(), o);
        decodeRoster(g.data(), g.size(), sink);
    }
    CHECK(true);
}

TEST(wire_facts_roundtrip_and_truncation) {
    AvFacts f;
    f.slot = 2;
    f.facts.level = 31;
    f.facts.sitting = 2;
    f.facts.actions = (1u << 9) | 1u;
    f.facts.regions = {0x00ABCD, 0x04001234};
    AvBytes b = encodeFacts(f);
    AvFacts o;
    CHECK(decodeFacts(b.data(), b.size(), o) && o == f);
    for (size_t n = 0; n < b.size(); ++n) CHECK(!decodeFacts(b.data(), n, o));        // every truncation is rejected as a whole
    AvBytes more = b;
    more.push_back(0);
    CHECK(!decodeFacts(more.data(), more.size(), o));                                   // trailing bytes too
    AvFacts big = f;
    big.facts.regions.assign(av::kMaxFactRegions + 1, 7);
    AvBytes bb = encodeFacts(big);                                                      // the encoder cuts the list
    CHECK(decodeFacts(bb.data(), bb.size(), o) && o.facts.regions.size() == av::kMaxFactRegions);
    AvBytes forged = b;
    forged[2 + 1 + 1 + 2 + 1 + 4] = 17;                                                 // a region count above the cap
    forged.resize(12 + 4 * 17);
    CHECK(!decodeFacts(forged.data(), forged.size(), o));
    AvBytes badslot = b;
    badslot[3] = 9;                                                                     // not a slot
    CHECK(!decodeFacts(badslot.data(), badslot.size(), o));
}

TEST(wire_pack_roundtrip_and_truncation) {
    AvPack p;
    p.slot = 2;
    p.pack.watchId = 0xC0FFEE11u;
    p.pack.haveWeight = true;
    p.pack.weightMilli = 75960;
    p.pack.items = {{0x0093C4, 1}, {0x00A84A, 3}, {0x04F85D, 12}};
    AvBytes b = encodePack(p);
    AvPack o;
    CHECK(decodePack(b.data(), b.size(), o) && o == p);
    for (size_t n = 0; n < b.size(); ++n) CHECK(!decodePack(b.data(), n, o));        // every truncation is rejected as a whole
    AvBytes more = b;
    more.push_back(0);
    CHECK(!decodePack(more.data(), more.size(), o));                                   // trailing bytes too
    AvPack noWeight = p;
    noWeight.pack.haveWeight = false;
    noWeight.pack.weightMilli = 0;
    AvBytes nb = encodePack(noWeight);
    CHECK(decodePack(nb.data(), nb.size(), o) && o == noWeight && !o.pack.haveWeight && nb.size() + 4 == b.size());
    AvPack big;
    big.slot = 1;
    for (uint32_t i = 1; i <= av::kMaxPackItems + 5; ++i) big.pack.items.push_back({i, 1});
    AvBytes bb = encodePack(big);                                                      // the encoder cuts the list
    CHECK(decodePack(bb.data(), bb.size(), o) && o.pack.items.size() == av::kMaxPackItems);
    const size_t nAt = 3 + 1 + 1 + 4;                                                  // [ch][kind][ver][slot][flags][watchId] then (no weight) n
    AvBytes forged(bb.begin(), bb.begin() + nAt);                                      // a well-formed list of 256 entries: one above the cap
    forged.push_back(0x00);
    forged.push_back(0x01);
    for (uint32_t i = 1; i <= 256; ++i) { forged.push_back(uint8_t(i)); forged.push_back(uint8_t(i >> 8)); forged.push_back(0); forged.push_back(0); forged.push_back(1); forged.push_back(0); forged.push_back(0); forged.push_back(0); }
    CHECK(!decodePack(forged.data(), forged.size(), o));
    AvBytes badslot = b;
    badslot[3] = 9;                                                                    // not a slot
    CHECK(!decodePack(badslot.data(), badslot.size(), o));
    AvBytes badflags = b;
    badflags[4] = 0x80;                                                                // a reserved flag
    CHECK(!decodePack(badflags.data(), badflags.size(), o));
    AvPack zero = p;
    zero.pack.items[1].count = 0;                                                      // only counts above zero travel
    AvBytes zb = encodePack(zero);
    CHECK(!decodePack(zb.data(), zb.size(), o));
    AvPack unsorted = p;
    std::swap(unsorted.pack.items[0], unsorted.pack.items[2]);                         // sorted by form
    AvBytes ub = encodePack(unsorted);
    CHECK(!decodePack(ub.data(), ub.size(), o));
    AvPack twice = p;
    twice.pack.items[1].form = twice.pack.items[0].form;                               // no form twice
    AvBytes tb = encodePack(twice);
    CHECK(!decodePack(tb.data(), tb.size(), o));
}

TEST(wire_player_net_ids) {
    for (uint8_t s = 0; s < 8; ++s) {
        CHECK(isPlayerNetId(playerNetId(s)));
        CHECK(slotOfPlayerNetId(playerNetId(s)) == s);
    }
    CHECK(!isPlayerNetId(0x12345678));
    CHECK(!isPlayerNetId(0xFF000001));
}

// =============================================================================================================================
// avatar network service on a loopback of three machines
// =============================================================================================================================
namespace {
using nvcoop::avatar::engine::AvatarRuntime;
using nvcoop::avatar::engine::FakeGameBackend;
using nvcoop::avatar::engine::RuntimeConfig;
constexpr uint32_t kTemplate = 0x1002;
constexpr av::CellId kCell = FakeGameBackend::kExtA;

// The NPC half of the engine port: there is no NPC in these scenarios, so nothing is ever loaded (ActorSync keeps only the player records).
class NullNpcPort : public nvcoop::actors::IActorPort {
public:
    bool loaded(nvcoop::actors::NetId) override { return false; }
    bool cellLoaded(nvcoop::actors::CellId) override { return false; }
    bool sample(nvcoop::actors::NetId, live::ActorSnapshot&) override { return false; }
    bool pose(nvcoop::actors::NetId, live::ActualPose&) override { return false; }
    void drive(nvcoop::actors::NetId, const live::InterpOut&) override { ++calls; }
    void applyAv(nvcoop::actors::NetId, const std::vector<live::AvValue>&) override { ++calls; }
    void place(nvcoop::actors::NetId, const live::ActorSnapshot&) override { ++calls; }
    void setAi(nvcoop::actors::NetId, bool) override { ++calls; }
    void setProtected(nvcoop::actors::NetId, bool) override { ++calls; }
    bool spawnStandIn(const nvcoop::actors::ActorMeta&, const live::ActorSnapshot&) override { ++calls; return false; }
    void removeStandIn(nvcoop::actors::NetId) override { ++calls; }
    void setBase(nvcoop::actors::NetId, const fnvcoop::FormRef&) override { ++calls; }
    void kill(nvcoop::actors::NetId) override { ++calls; }
    void revive(nvcoop::actors::NetId) override { ++calls; }
    int calls = 0;   // a call for a player that reached the NPC port (must stay 0)
};

// The roster of parts=unified: the player records of this machine's ActorSync.
struct EntDir : IEntityDirectory {
    nvcoop::actors::ActorRuntime* rt = nullptr;
    std::vector<PlayerId> playerSlots() const override { return rt && rt->sync() ? rt->sync()->playerSlots() : std::vector<PlayerId>(); }
};

struct Node {
    fnvcoop::PeerId peer = 0;
    PlayerId slot = 0;
    bool host = false;
    bool present = true;      // the machine is connected to the session
    bool linkUp = true;
    fnvcoop::MuxTransport* mux = nullptr;
    FakeGameBackend be;
    std::unique_ptr<AvatarRuntime> rt;
    std::unique_ptr<RuntimeSide> side;
    std::unique_ptr<AvatarNet> an;
    LiveHub hub;
    uint32_t epoch = 1000;
    std::vector<std::tuple<PlayerId, bool, live::Vec3>> poses;
    float vx = 0;             // the local player walks along +x at this speed (units/s)
    // [entities] parts / stream = unified: the actor module's ActorSync runs next to AvatarNet and carries what the mode says (docs/entity-core-plan.md S5, S11)
    bool parts = false, stream = false;
    std::unique_ptr<NullNpcPort> npcPort;
    std::unique_ptr<nvcoop::actors::ActorRuntime> actors;
    EntDir dir;

    Node(fnvcoop::PeerId p, PlayerId s, bool h, fnvcoop::MuxTransport* m, bool partsUnified = false, bool streamUnified = false)
        : peer(p), slot(s), host(h), mux(m), parts(partsUnified), stream(streamUnified) {
        be.knownBases = {kTemplate};
        be.vanishAfterFrames = 2;
        RuntimeConfig rc;
        rc.puppet.defaultTemplate = kTemplate;
        // localSlot stays at the ini default 0, exactly as the shipped module has it: the session start (AvatarNet::sessionBegin) must tell the runtime its slot
        rt.reset(new AvatarRuntime(be, rc));
        side.reset(new RuntimeSide(*rt));
        AvatarNetOptions ao;
        ao.partsUnified = parts;
        ao.streamUnified = stream;
        an.reset(new AvatarNet(*side, ao, [this] { return epoch++; }));
        if (parts || stream) {
            npcPort.reset(new NullNpcPort);
            actors.reset(new nvcoop::actors::ActorRuntime(*npcPort));
            dir.rt = actors.get();
            an->setEntityDirectory(&dir);
        }
        an->setPoseSink([this](PlayerId sl, bool pr, live::CellId, const live::Vec3& pos) { poses.emplace_back(sl, pr, pos); });
        hub.add(an.get(), true);
        be.Player().pos = {1000.0f * (slot + 1), 0, 0};
        be.Player().cell = kCell;
    }
    // actors that are not the player and not deleted (the puppets in this machine's world)
    int puppets() {
        std::vector<nvcoop::avatar::engine::ActorEntry> v;
        be.ListActors(kCell, v);
        int n = 0;
        for (auto& e : v) {
            auto* f = be.Get(e.ref);
            if (e.ref != nvcoop::avatar::engine::kPlayerRef && f && f->exists && !f->deleted) ++n;
        }
        return n;
    }
};

struct Rig3 {
    Net3 n;
    bool parts, stream;
    std::vector<std::unique_ptr<Node>> nodes;   // [0] host (peer 0), then clients (peer = slot)
    explicit Rig3(bool partsUnified = false, bool streamUnified = false) : parts(partsUnified), stream(streamUnified) {
        fnvcoop::LinkParams lp;
        lp.minLatencyMs = 20;
        lp.maxLatencyMs = 50;   // a decent connection: 20 to 50 ms one way, with reordering
        n.net.setLink(lp);
        nodes.emplace_back(new Node(0, 0, true, n.m(0), parts, stream));
    }
    Node& host() { return *nodes[0]; }
    Node& addClient(PlayerId slot) {
        nodes.emplace_back(new Node(slot, slot, false, n.m(slot), parts, stream));
        return *nodes.back();
    }
    Node* bySlot(PlayerId s) { for (auto& p : nodes) if (p->slot == s) return p.get(); return nullptr; }
    void step(uint64_t dtMs = 33) {
        n.now += dtMs;
        n.pump();
        std::vector<ClientLink> links;
        for (auto& p : nodes)
            if (!p->host) links.push_back(cl(p->slot, p->peer, p->present));
        for (auto& p : nodes) {
            if (p->vx != 0) p->be.Player().pos.x += p->vx * float(dtMs) / 1000.0f;
            SessionView v;
            if (p->host) {
                v = hostView(p->mux, links, n.now);
            } else {
                v = clientView(p->mux, p->slot, p->linkUp, n.now);
                v.active = p->present;
            }
            if (p->actors) {   // the actor module's frame, as the live glue drives it (ActorsService), before the avatar service's
                nvcoop::actors::FrameInfo fi;
                fi.active = v.active; fi.isHost = v.isHost; fi.self = v.isHost ? kHostPlayer : v.self; fi.hostLinkUp = v.hostLinkUp;
                for (const ClientLink& c : v.clients) fi.clients.push_back({c.slot, c.peer, c.joined});
                fi.mux = v.mux; fi.nowMs = v.nowMs;
                p->actors->frame(fi);
                if (nvcoop::actors::ActorSync* sy = p->actors->sync()) sy->enablePlayerEntities(&p->an->playerPort(), p->parts, p->stream);
            }
            p->hub.frame(v);
            p->be.Frame();
        }
    }
    void run(uint64_t ms) { for (uint64_t t = 0; t < ms; t += 33) step(); }
};

float xOfPuppet(Node& viewer, PlayerId slot, bool* ok = nullptr) {
    av::RefId r = viewer.an->puppetRef(slot);
    const auto* f = r ? viewer.be.Get(r) : nullptr;
    if (ok) *ok = f != nullptr && f->exists;
    return f ? f->pos.x : -1e9f;
}
float playerX(Node& n) { return n.be.Player().pos.x; }
}  // namespace

TEST(avatars_three_machines_see_each_other) {
    Rig3 r;
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    a.present = true;
    b.present = false;   // B joins later
    a.vx = 150;
    h.vx = 0;
    r.run(2500);
    CHECK(h.an->roster() == std::set<PlayerId>({1}));
    CHECK(a.an->roster() == std::set<PlayerId>({0}));   // A sees the host (the roster lists the host and A; A filters itself)
    bool ok = false;
    float ax = xOfPuppet(h, 1, &ok);
    CHECK(ok);
    CHECK(std::abs(ax - playerX(a)) < 150.0f);          // within the interpolation delay of a 150 u/s walker
    float hx = xOfPuppet(a, 0, &ok);
    CHECK(ok && std::abs(hx - playerX(h)) < 30.0f);
    // B joins: everybody gets a puppet of everybody else, B included, through the host's relay
    b.present = true;
    b.vx = -100;
    r.run(3500);
    CHECK(h.an->roster() == std::set<PlayerId>({1, 2}));
    CHECK(a.an->roster() == std::set<PlayerId>({0, 2}));
    CHECK(b.an->roster() == std::set<PlayerId>({0, 1}));
    CHECK(h.puppets() == 2 && a.puppets() == 2 && b.puppets() == 2);
    float bxAtA = xOfPuppet(a, 2, &ok);
    CHECK(ok && std::abs(bxAtA - playerX(b)) < 150.0f);   // A sees B only through the host (relay)
    float axAtB = xOfPuppet(b, 1, &ok);
    CHECK(ok && std::abs(axAtB - playerX(a)) < 180.0f);
    CHECK(h.an->stats().forged == 0 && a.an->stats().forged == 0);
    CHECK(a.an->puppetRef(a.slot) == 0 && b.an->puppetRef(b.slot) == 0);
}

TEST(avatars_never_a_puppet_of_oneself) {
    Rig3 r;
    r.addClient(1);
    r.run(2000);
    CHECK(r.bySlot(1)->an->roster().count(1) == 0);
    CHECK(r.bySlot(1)->puppets() == 1);   // just the host's puppet
    CHECK(r.host().puppets() == 1);
    CHECK(r.bySlot(1)->an->puppetRef(1) == 0);
}

TEST(avatars_leave_removes_puppets_everywhere) {
    Rig3 r;
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    a.vx = 50;
    b.vx = 50;
    r.run(3000);
    CHECK(r.host().puppets() == 2 && a.puppets() == 2);
    b.present = false;   // B quits
    r.run(1500);
    CHECK(r.host().an->roster() == std::set<PlayerId>({1}));
    CHECK(r.host().puppets() == 1);
    CHECK(a.an->roster() == std::set<PlayerId>({0}));
    CHECK(a.puppets() == 1);
    CHECK(b.puppets() == 0);   // B's own session ended: its puppets were taken down
    CHECK(b.an->roster().empty());
}

// A player's attacks are owner events: the host punches, a client throws, the other client swings. Every OTHER machine shows each one on that player's puppet exactly
// once (the host relays between the clients), over a link that loses a fifth of the datagrams and duplicates another fifth; the attacker's own machine plays nothing.
TEST(avatars_attacks_show_on_every_other_machine) {
    Rig3 r;
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    r.run(4500);
    CHECK(h.puppets() == 2 && a.puppets() == 2 && b.puppets() == 2);
    fnvcoop::LinkParams lp;
    lp.minLatencyMs = 20;
    lp.maxLatencyMs = 50;
    lp.loss = 0.2;
    lp.duplicate = 0.2;
    r.n.net.setLink(lp);
    auto swing = [&](Node& who, int group, uint32_t ms) {
        auto& p = who.be.Player();
        p.group = group;
        p.groupSlot = 1;
        p.groupState = 1;
        p.groupPlayMs = 0;
        for (uint32_t t = 0; t < ms; t += 33) {
            r.step();
            p.groupPlayMs += 33;
        }
        p.group = -1;
        p.groupPlayMs = -1;
        r.run(1500);
    };
    auto seen = [&](Node& viewer, PlayerId of, const std::string& line) {
        const av::RefId ref = viewer.an->puppetRef(of);
        int n = 0;
        for (const auto& l : viewer.be.lines)
            if (ref && l.ctx == ref && l.text == line) ++n;
        return n;
    };
    swing(h, 26, 400);    // the host punches
    CHECK(seen(a, 0, "PlayGroup AttackLeft 1") == 1);
    CHECK(seen(b, 0, "PlayGroup AttackLeft 1") == 1);
    swing(a, 114, 400);   // client 1 throws
    CHECK(seen(h, 1, "PlayGroup AttackThrow 1") == 1);
    CHECK(seen(b, 1, "PlayGroup AttackThrow 1") == 1);   // through the host's relay
    swing(b, 92, 600);    // client 2 makes a power attack
    CHECK(seen(h, 2, "PlayGroup AttackPower 1") == 1);
    CHECK(seen(a, 2, "PlayGroup AttackPower 1") == 1);
    swing(h, 26, 400);    // the host punches again: a second event, not a replay of the first
    CHECK(seen(a, 0, "PlayGroup AttackLeft 1") == 2);
    CHECK(seen(b, 0, "PlayGroup AttackLeft 1") == 2);
    // nothing plays on the attacker's own machine: it has no puppet of itself, and the real player is never driven
    CHECK(h.be.count("PlayGroup AttackLeft") == 0 && a.be.count("PlayGroup AttackThrow") == 0 && b.be.count("PlayGroup AttackPower") == 0);
    CHECK(h.be.badLines.empty() && a.be.badLines.empty() && b.be.badLines.empty());
}

TEST(avatars_rejoin_follows_again) {
    Rig3 r;
    Node& a = r.addClient(1);
    a.vx = 100;
    r.run(2500);
    CHECK(r.host().puppets() == 1);
    a.present = false;
    r.run(1000);
    CHECK(r.host().puppets() == 0);
    a.vx = 0;
    a.be.Player().pos.x = 50000;   // reconnects somewhere else (same cell here)
    a.present = true;
    r.run(3000);
    bool ok = false;
    float x = xOfPuppet(r.host(), 1, &ok);
    CHECK(ok && std::abs(x - playerX(a)) < 300.0f);
}

TEST(avatars_forged_stream_is_rejected_at_the_host) {
    Rig3 r;
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    b.vx = 30;
    r.run(2500);
    bool ok = false;
    float before = xOfPuppet(r.host(), 2, &ok);
    CHECK(ok);
    // client A publishes a snapshot that claims to be B's avatar, far away
    live::SnapshotSender evil;
    live::ActorSnapshot s;
    s.netId = playerNetId(2);
    s.ownerEpoch = 999999999u;   // an epoch that beats B's own
    s.seq = 1; s.tick = 5; s.cell = kCell;
    s.pos = {90000, 0, 0};
    evil.queue(0, s, 5);
    for (auto& d : evil.flush(0, 5)) {
        Bytes pk;
        pk.push_back(kChannelAvatars);
        pk.insert(pk.end(), d.begin(), d.end());
        a.mux->sendEx(0, pk, fnvcoop::ITransport::Channel::Unreliable);
    }
    r.run(900);
    CHECK(r.host().an->stats().forged >= 1);
    float after = xOfPuppet(r.host(), 2);
    CHECK(std::abs(after - playerX(b)) < 200.0f);     // still following the real B
    CHECK(std::abs(after - 90000.0f) > 1000.0f);
    CHECK(after >= before - 1.0f);
    // the same forgery with a Look message
    AvLook l = sampleLook(2);
    l.equipment = {{0x7777, false}};
    a.mux->sendEx(0, encodeLook(l), fnvcoop::ITransport::Channel::Reliable);
    r.run(300);
    CHECK(r.host().an->stats().looksRejected >= 1);
    const AvLook* real = r.host().an->lookOf(2);   // B's own (legitimate) look, if it has sent one: never the forged one
    bool forgedApplied = false;
    if (real)
        for (auto& e : real->equipment)
            if (e.form == 0x7777) forgedApplied = true;
    CHECK(!forgedApplied);
}

TEST(avatars_look_and_equipment_travel) {
    Rig3 r;
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    a.be.Player().worn.insert(0x2000);
    a.be.Player().inv[0x2000] = 1;
    a.an->setLocalAppearanceReader([](av::Appearance& ap) { ap.race = 1; ap.name = "Alpha"; return true; });
    b.an->setLocalAppearanceReader([](av::Appearance& ap) { ap.race = 1; ap.name = "Beta"; return true; });
    r.run(5000);
    CHECK(r.host().an->hasLook(1) && r.host().an->hasLook(2));
    CHECK(b.an->hasLook(1));   // relayed through the host
    CHECK(r.host().an->stats().looksApplied >= 1 && b.an->stats().looksApplied >= 1);
    av::RefId ref = r.host().an->puppetRef(1);
    const auto* f = ref ? r.host().be.Get(ref) : nullptr;
    CHECK(f && f->worn.count(0x2000) == 1);
    av::RefId refAtB = b.an->puppetRef(1);
    const auto* fb = refAtB ? b.be.Get(refAtB) : nullptr;
    CHECK(fb && fb->worn.count(0x2000) == 1);
    // a change travels too
    a.be.Player().worn.erase(0x2000);
    a.be.Player().worn.insert(0x2001);
    a.be.Player().inv[0x2001] = 1;
    r.run(5000);
    f = r.host().be.Get(r.host().an->puppetRef(1));
    CHECK(f && f->worn.count(0x2001) == 1 && f->worn.count(0x2000) == 0);
}

// W3a item 3: what a guest's snapshot does not carry reaches the host, on change, from the guest alone.
TEST(avatars_facts_travel_to_the_host_on_change_and_only_from_the_guest_itself) {
    Rig3 r;
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    a.be.Player().level = 12;
    a.be.Player().sitState = 2;
    a.be.Player().actions = {9};
    a.be.playerRegions = {0x1111, 0x2222};
    b.be.Player().level = 40;
    r.run(4000);
    const av::PlayerFacts* fa = r.host().an->factsOf(1);
    const av::PlayerFacts* fb = r.host().an->factsOf(2);
    CHECK(fa && fa->level == 12 && fa->sitting == 2 && fa->actions == (1u << 9) && fa->regions.size() == 2 && fa->regions[0] == 0x1111);
    CHECK(fb && fb->level == 40 && fb->sitting == 0 && fb->regions.empty());
    CHECK(r.host().an->factsOf(0) == nullptr && a.an->factsOf(2) == nullptr);   // the host's own player is read by the conditions; guests do not hear each other's
    const uint64_t sent = a.an->stats().factsSent;
    r.run(4000);
    CHECK(a.an->stats().factsSent == sent);                                      // unchanged facts are not re-sent
    a.be.Player().level = 13;
    a.be.Player().actions = {};
    a.be.playerRegions = {0x2222};
    r.run(2500);
    fa = r.host().an->factsOf(1);
    CHECK(fa && fa->level == 13 && fa->actions == 0 && fa->regions.size() == 1 && fa->regions[0] == 0x2222);
    CHECK(a.an->stats().factsSent == sent + 1);
    // client A claims to be B: the host drops it
    AvFacts evil;
    evil.slot = 2;
    evil.facts.level = 99;
    a.mux->sendEx(0, encodeFacts(evil), fnvcoop::ITransport::Channel::Reliable);
    r.run(300);
    CHECK(r.host().an->stats().factsRejected >= 1);
    CHECK(r.host().an->factsOf(2) && r.host().an->factsOf(2)->level == 40);
    // a player who leaves takes its facts with it
    a.present = false;
    r.run(1500);
    CHECK(r.host().an->factsOf(1) == nullptr);
}

// Guest pack facts: the weight and the counts of the watched item forms reach the host, on change, rate limited, from the guest alone.
namespace {
constexpr uint32_t kTotem = 0x00A84A, kRoot = 0x0093C4, kLunch = 0x00F85D, kJunk = 0x00BEEF, kUnwatched = 0x00C0DE;
void watchThese(Node& n, std::vector<uint32_t> forms) { n.an->setPackWatch([forms] { return forms; }); }
}  // namespace
TEST(avatars_pack_travels_to_the_host_on_change_rate_limited_and_only_from_the_guest_itself) {
    Rig3 r;
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    for (Node* n : {&r.host(), &a, &b}) watchThese(*n, {kTotem, kRoot, kLunch, kJunk});
    a.be.Player().inv[kTotem] = 3;
    a.be.Player().inv[kRoot] = 1;
    a.be.Player().inv[kUnwatched] = 9;
    a.be.Player().inventoryWeight = 75.96;
    b.be.Player().inv[kLunch] = 2;
    b.be.Player().inventoryWeight = 12.5;
    r.run(4000);
    int64_t n = -1;
    double w = -1;
    CHECK(r.host().an->packCount(1, kTotem, &n) && n == 3);
    CHECK(r.host().an->packCount(1, kRoot, &n) && n == 1);
    CHECK(r.host().an->packCount(1, kLunch, &n) && n == 0);                       // watched and not carried: a zero
    CHECK(!r.host().an->packCount(1, kUnwatched, &n));                            // not on the list: unknown, the caller keeps its old answer
    CHECK(r.host().an->packWeight(1, &w) && std::abs(w - 75.96) < 0.0006);       // the Jed gate's 76: a pack of 75.96 is below it
    CHECK(r.host().an->packCount(2, kLunch, &n) && n == 2 && r.host().an->packWeight(2, &w) && w == 12.5);
    CHECK(!r.host().an->packCount(0, kTotem, &n) && !r.host().an->packWeight(0, &w));   // the host's own pack is read by the conditions
    CHECK(a.an->packOf(2) == nullptr && b.an->packOf(1) == nullptr);              // guests do not hear each other's
    const uint64_t sent = a.an->stats().packsSent;
    CHECK(sent >= 1);
    r.run(4000);
    CHECK(a.an->stats().packsSent == sent);                                       // an unchanged pack is not re-sent
    // a burst of changes inside the minimum gap is one or two messages, not one per change
    a.be.Player().inv[kTotem] = 2;
    r.run(700);
    a.be.Player().inv[kTotem] = 1;
    r.run(100);
    a.be.Player().inv.erase(kTotem);
    a.be.Player().inventoryWeight = 70.0;
    r.run(100);
    a.be.Player().inv[kLunch] = 4;
    r.run(2500);
    CHECK(r.host().an->packCount(1, kTotem, &n) && n == 0 && r.host().an->packCount(1, kLunch, &n) && n == 4);
    CHECK(r.host().an->packWeight(1, &w) && w == 70.0);
    CHECK(a.an->stats().packsSent >= sent + 1 && a.an->stats().packsSent <= sent + 2);   // 4 changes in 0.9 s ride in one or two messages
    // client A claims to be B: the host drops it
    AvPack evil;
    evil.slot = 2;
    evil.pack.items = {{kLunch, 99}};
    a.mux->sendEx(0, encodePack(evil), fnvcoop::ITransport::Channel::Reliable);
    r.run(300);
    CHECK(r.host().an->stats().packsRejected >= 1);
    CHECK(r.host().an->packCount(2, kLunch, &n) && n == 2);
    // a pack sent to a client (a host never does) is ignored
    AvPack toClient;
    toClient.slot = 1;
    toClient.pack.items = {{kLunch, 5}};
    r.host().mux->sendEx(1, encodePack(toClient), fnvcoop::ITransport::Channel::Reliable);
    r.run(300);
    CHECK(a.an->stats().packsRejected >= 1 && a.an->packOf(1) == nullptr);
    // a player who leaves takes the pack with him
    a.present = false;
    r.run(1500);
    CHECK(r.host().an->packOf(1) == nullptr);
}

TEST(avatars_pack_zero_is_trusted_only_for_the_same_watch_list) {
    Rig3 r;
    Node& a = r.addClient(1);
    watchThese(a, {kTotem, kRoot});
    watchThese(r.host(), {kTotem, kRoot, kLunch});     // another release: the host watches one form more
    a.be.Player().inv[kTotem] = 1;
    r.run(3000);
    int64_t n = -1;
    CHECK(r.host().an->packCount(1, kTotem, &n) && n == 1);       // a listed count is a count
    CHECK(!r.host().an->packCount(1, kRoot, &n));                  // a zero from another list is not a zero
    CHECK(!r.host().an->packCount(1, kLunch, &n));
    watchThese(r.host(), {kTotem, kRoot});                         // the same list: the zero counts again
    CHECK(r.host().an->packCount(1, kRoot, &n) && n == 0);
    CHECK(!r.host().an->packCount(1, kLunch, &n));
}

TEST(avatars_pack_is_unknown_before_it_arrives_and_an_unreadable_player_sends_nothing) {
    Rig3 r;
    Node& a = r.addClient(1);
    watchThese(r.host(), {kTotem});
    watchThese(a, {kTotem});
    a.be.Player().exists = false;
    r.run(1500);
    int64_t n = -1;
    double w = -1;
    CHECK(!r.host().an->packCount(1, kTotem, &n) && !r.host().an->packWeight(1, &w) && a.an->stats().packsSent == 0);
    a.be.Player().exists = true;
    a.be.Player().inv[kTotem] = 1;
    r.run(2500);
    CHECK(r.host().an->packCount(1, kTotem, &n) && n == 1);
}

TEST(avatars_unreadable_local_player_sends_nothing) {
    Rig3 r;
    Node& a = r.addClient(1);
    a.be.Player().exists = false;   // the player cannot be read (main menu / loading)
    r.run(1500);
    CHECK(a.an->stats().localSnapshots == 0);
    CHECK(r.host().an->stats().accepted == 0);
    a.be.Player().exists = true;
    r.run(2000);
    CHECK(a.an->stats().localSnapshots > 0 && r.host().an->stats().accepted > 0);
}

TEST(avatars_pose_sink_reports_remote_players) {
    Rig3 r;
    Node& a = r.addClient(1);
    a.vx = 40;
    r.run(2500);
    bool saw = false;
    for (auto& p : r.host().poses)
        if (std::get<0>(p) == 1 && std::get<1>(p)) saw = true;
    CHECK(saw);
    bool sawSelf = false, sawClientSelf = false;   // the local player's own place is reported too (the spawn service wants every slot)
    for (auto& p : r.host().poses)
        if (std::get<0>(p) == 0 && std::get<1>(p)) sawSelf = true;
    for (auto& p : a.poses)
        if (std::get<0>(p) == 1 && std::get<1>(p)) sawClientSelf = true;
    CHECK(sawSelf && sawClientSelf);
    size_t n = r.host().poses.size();
    a.present = false;
    r.run(800);
    bool gone = false;
    for (size_t i = n; i < r.host().poses.size(); ++i)
        if (std::get<0>(r.host().poses[i]) == 1 && !std::get<1>(r.host().poses[i])) gone = true;
    CHECK(gone);   // the ownership service is told the player is no longer there
}

TEST(avatars_load_purges_orphans_without_a_session) {
    Net3 n;
    Node h(0, 0, true, n.m(0));
    h.be.AddActor(kTemplate, kCell, nvcoop::avatar::Vec3{});   // a save carried puppets of an old session
    h.be.AddActor(kTemplate, kCell, nvcoop::avatar::Vec3{});
    CHECK(h.puppets() == 2);
    SessionView idle;
    idle.nowMs = 10;
    h.hub.onLoadBegin();
    h.hub.onLoadEnd();
    for (int i = 0; i < 6; ++i) { idle.nowMs += 33; h.hub.frame(idle); h.be.Frame(); }
    CHECK(h.puppets() == 0);
    // a second load purges again
    h.be.AddActor(kTemplate, kCell, nvcoop::avatar::Vec3{});
    h.hub.onLoadBegin();
    h.hub.onLoadEnd();
    for (int i = 0; i < 6; ++i) { idle.nowMs += 33; h.hub.frame(idle); h.be.Frame(); }
    CHECK(h.puppets() == 0);
}

TEST(avatars_load_inside_a_session_drops_and_respawns) {
    Rig3 r;
    Node& a = r.addClient(1);
    a.vx = 20;
    r.run(2500);
    CHECK(r.host().puppets() == 1);
    // the host loads a save: the world is torn down (the engine forgets its refs), the save also carried the puppet
    r.host().hub.onLoadBegin();
    r.host().be.Load();
    r.host().hub.onLoadEnd();
    r.run(3000);
    CHECK(r.host().puppets() == 1);   // exactly one puppet again: the carried one was purged, the real one respawned
    bool ok = false;
    float x = xOfPuppet(r.host(), 1, &ok);
    CHECK(ok && std::abs(x - playerX(a)) < 200.0f);
}

TEST(avatars_session_end_takes_puppets_down) {
    Rig3 r;
    Node& a = r.addClient(1);
    a.vx = 10;
    r.run(2500);
    CHECK(a.puppets() == 1);
    a.linkUp = false;
    r.run(600);
    CHECK(a.puppets() == 0);   // the roster is gone with the host link
    a.linkUp = true;
    r.run(2500);
    CHECK(a.puppets() == 1);   // and comes back with it
}

TEST(avatars_ghost_ref_map) {
    Rig3 r;
    r.addClient(1);
    r.run(3000);
    av::RefId ref = r.host().an->puppetRef(1);
    CHECK(ref != 0);
    PlayerId slot = 99;
    CHECK(r.host().an->slotOfPuppetRef(ref, slot) && slot == 1);
    CHECK(!r.host().an->slotOfPuppetRef(0, slot));
    CHECK(!r.host().an->slotOfPuppetRef(ref + 1000, slot));
    CHECK(r.host().an->puppetRef(5) == 0);
    std::string j = r.host().an->statusJson();
    CHECK(j.find("\"slot\":1") != std::string::npos);
}

// A teleport (the lab's spawn room: "the client's puppet arrives 650 to 2700 ms late, catching up from 773 units away"). The players stand far apart, so each stream
// to the other runs at the far tier (5 Hz); then they jump next to each other. The first snapshot after the jump must leave at once (a jump is not held for the rate)
// and the puppet must snap to it, not glide.
static uint64_t arrivalAfterTeleport(Rig3& r, Node& viewer, PlayerId of, Node& target, float x, float tol, uint64_t limitMs) {
    const uint64_t t0 = r.n.now;
    while (r.n.now - t0 < limitMs) {
        r.step(16);
        bool ok = false;
        const float px = xOfPuppet(viewer, of, &ok);
        if (ok && std::abs(px - x) < tol) return r.n.now - t0;
    }
    (void)target;
    return limitMs;
}

static void quickLink(Rig3& r) {
    fnvcoop::LinkParams lp;
    lp.minLatencyMs = 3;
    lp.maxLatencyMs = 6;   // a LAN: the time to arrive is then the stream's own holds and the buffer, not the link
    r.n.net.setLink(lp);
}

TEST(avatars_teleport_arrives_at_once) {
    Rig3 r;
    quickLink(r);
    Node& h = r.host();
    Node& a = r.addClient(1);
    h.be.Player().pos = {0, 0, 0};
    a.be.Player().pos = {9000, 0, 0};
    r.run(5000);
    bool ok = false;
    CHECK(std::abs(xOfPuppet(a, 0, &ok) - 0.0f) < 100.0f && ok);       // steady state: each sees the other where he is
    h.be.Player().pos = {8800, 0, 0};                                   // both jump next to each other, at the same moment
    a.be.Player().pos = {9200, 0, 0};
    const uint64_t t0 = r.n.now;
    uint64_t hostSeesClient = 0, clientSeesHost = 0;
    while (r.n.now - t0 < 4000 && (!hostSeesClient || !clientSeesHost)) {
        r.step(16);
        const float cx = xOfPuppet(h, 1, &ok);
        if (!hostSeesClient && ok && std::abs(cx - 9200.0f) < 100.0f) hostSeesClient = r.n.now - t0;
        const float hx = xOfPuppet(a, 0, &ok);
        if (std::getenv("TP_TRACE")) {
            const auto* d = a.rt->InterpOf(0);
            const auto* d2 = h.rt->InterpOf(1);
            std::printf("      t=%llu hostPuppetAtClient=%.0f delay=%.0f want=%.0f | clientPuppetAtHost=%.0f delay=%.0f want=%.0f pushes=%llu\n", (unsigned long long)(r.n.now - t0), hx,
                        d ? d->delayInUse.last : -1.0, d ? d->wantDelayMs : 0.0, cx, d2 ? d2->delayInUse.last : -1.0, d2 ? d2->wantDelayMs : 0.0, d2 ? (unsigned long long)d2->pushes : 0ull);
        }
        if (!clientSeesHost && ok && std::abs(hx - 8800.0f) < 100.0f) clientSeesHost = r.n.now - t0;
    }
    std::printf("    teleport: host sees the client after %llu ms, the client sees the host after %llu ms\n", (unsigned long long)hostSeesClient, (unsigned long long)clientSeesHost);
    CHECK(hostSeesClient > 0 && hostSeesClient <= 400);
    CHECK(clientSeesHost > 0 && clientSeesHost <= 400);
}

TEST(avatars_teleport_of_one_player_next_to_a_player_he_has_not_heard_of_yet) {
    // only the host jumps (next to the client); the client has not moved, so for 1 s nobody has told the host that the client is near. The far tier sends a snapshot
    // every 200 ms, so a jump that waited for its turn would show up to 200 ms late: the jump is tried at eight phases of that period, the worst counts.
    uint64_t worst = 0;
    for (int phase = 0; phase < 8; ++phase) {
        Rig3 r;
        quickLink(r);
        Node& h = r.host();
        Node& a = r.addClient(1);
        h.be.Player().pos = {0, 0, 0};
        a.be.Player().pos = {9000, 0, 0};
        r.run(5000);
        for (int k = 0; k < phase * 2; ++k) r.step(16);
        h.be.Player().pos = {8700, 0, 0};
        worst = std::max(worst, arrivalAfterTeleport(r, a, 0, h, 8700.0f, 100.0f, 4000));
    }
    std::printf("    teleport: the client sees the host after at most %llu ms\n", (unsigned long long)worst);
    CHECK(worst <= 130);
}

// The rate planner sends a jump (another cell, or further than jumpUnits from the last snapshot sent on the link) at once, whatever the tier's period says, and a
// walk step still waits for its turn. The network service's planner is built with that rule on.
TEST(avatars_the_rate_planner_sends_a_jump_at_once) {
    av::StreamRatePlanner p(av::StreamRateConfig{});
    live::ActorSnapshot a, out;
    a.cell = kCell;
    a.pos = {0, 0, 0};
    p.setPlace(0, kCell, a.pos);
    p.setPlace(1, kCell, {9000, 0, 0});                                // far tier: one snapshot per 200 ms
    CHECK(p.offer(1, 0, a, 1000, out));                                // the first goes
    a.pos = {20, 0, 0};
    CHECK(!p.offer(1, 0, a, 1033, out));                               // a walk step 33 ms later waits for its turn
    a.pos = {8700, 0, 0};
    p.setPlace(0, kCell, a.pos);
    CHECK(p.offer(1, 0, a, 1066, out) && out.pos.x == 8700.0f);        // a jump does not
    a.cell = kCell + 1;
    CHECK(p.offer(1, 0, a, 1099, out) && out.cell == kCell + 1);       // nor does a new cell
    Node c(1, 1, false, nullptr);
    CHECK(c.an->ratePlanner().config().jumpUnits > 0.0f);              // and the service's planner has the rule on
}

// A snapshot of the host that reaches a joining client before the roster that names the host: the client's receiver acks it (so the host's sender stops: a standing
// player sends only a heartbeat a second), and the roster creates the puppet a moment later. The puppet must get that snapshot, not wait for the heartbeat.
struct NullLink : ILiveLink {
    bool send(PlayerId, bool, Bytes) override { return true; }
    bool isHost() const override { return false; }
    PlayerId self() const override { return 1; }
};

TEST(avatars_a_snapshot_ahead_of_the_roster_reaches_the_puppet) {
    Node c(1, 1, false, nullptr);
    NullLink link;
    SessionInfo info;
    info.isHost = false;
    info.self = 1;
    c.an->sessionBegin(info, link);
    c.an->hostLinkUp();
    live::SnapshotSender hostSender;
    live::ActorSnapshot s;
    s.netId = playerNetId(0);
    s.ownerEpoch = 1;
    s.seq = 1;
    s.tick = 5000;
    s.cell = kCell;
    s.pos = {2932.0f, -6048.0f, 7040.0f};
    hostSender.queue(1, s, 5000);
    Bytes dg;
    for (Bytes& d : hostSender.flush(1, 5000)) {
        dg.push_back(kChannelAvatars);
        dg.insert(dg.end(), d.begin(), d.end());
    }
    c.an->tick(5010);
    c.an->receive(kHostPlayer, dg.data(), dg.size());                  // before the roster
    CHECK(c.an->stats().noRoster == 1 && c.rt->remoteSnapshots() == 0);
    Bytes roster = encodeRoster({kHostPlayer, 1});
    c.an->receive(kHostPlayer, roster.data(), roster.size());          // the roster names the host: the puppet is made
    CHECK(c.an->roster().count(kHostPlayer) == 1);
    CHECK(c.rt->remoteSnapshots() == 1);                               // and it already has the place the host's snapshot gave
    c.an->sessionEnd();
}

// =============================================================================================================================
// [entities] (docs/entity-core-plan.md S5, S11): the switches, the Look codec's golden bytes, and the player entities on the loopback
// Twins 1170 (parts=unified still sends the old messages) 1171 (the old messages are not understood) 1172 (a client never learns the roster) 1173 (weapon mods lost)
// 1174 (the look never reaches the puppet) 1175 (the held weapon is not named) 1176 (parts default legacy) 1177 (unified without actors) 1178 (Look v2 bytes changed)
// =============================================================================================================================
TEST(switches_entities) {
    LiveSwitches s = readLiveSwitches(fakeIni({}));
    CHECK(s.entities.parts == EntityMode::Unified);               // the new path is the default; legacy is only a fallback switch
    CHECK(s.entities.stream == kDefaultStreamMode);
    CHECK(!s.partsUnified() && !s.streamUnified());               // nothing runs without actors and avatars
    CHECK(s.warnings.empty() && !s.any());
    s = readLiveSwitches(fakeIni({{"live.actors", "1"}, {"live.avatars", "1"}}));
    CHECK(s.partsUnified());
    CHECK(describeEntities(s) == std::string("parts=unified stream=") + toString(kDefaultStreamMode));
    s = readLiveSwitches(fakeIni({{"live.actors", "1"}, {"live.avatars", "1"}, {"entities.parts", " Legacy "}, {"entities.stream", "UNIFIED"}}));
    CHECK(!s.partsUnified() && s.streamUnified());
    CHECK(describeEntities(s) == "parts=legacy stream=unified");
    s = readLiveSwitches(fakeIni({{"live.actors", "1"}, {"live.avatars", "1"}, {"entities.parts", "bus"}, {"entities.stream", "1"}}));   // a typo keeps the default
    CHECK(s.partsUnified() && s.entities.stream == kDefaultStreamMode);
    s = readLiveSwitches(fakeIni({{"live.avatars", "1"}, {"entities.stream", "unified"}}));   // no actors: ActorSync is not there to carry anything
    CHECK(!s.partsUnified() && !s.streamUnified());
    s = readLiveSwitches(fakeIni({{"live.actors", "1"}, {"entities.stream", "unified"}}));    // no avatars: no player
    CHECK(!s.partsUnified() && !s.streamUnified());
    CHECK(describeSwitches(readLiveSwitches(fakeIni({{"live.actors", "1"}, {"entities.parts", "legacy"}}))) == "actors=1 avatars=0 combat=0 spawns=0");   // the four-lane line is untouched
    CHECK(parseEntityMode("unified", EntityMode::Legacy) == EntityMode::Unified && parseEntityMode("", EntityMode::Legacy) == EntityMode::Legacy);
    CHECK(parseEntityMode("LEGACY ", EntityMode::Unified) == EntityMode::Legacy && parseEntityMode("uni", EntityMode::Unified) == EntityMode::Unified);
}

// The Look v2 message exactly as the codec in avatar_wire.cpp wrote it before its field codec moved to plugin/avatar/look_codec.cpp: kept here, verbatim, as the oracle.
namespace {
void oput8(AvBytes& b, uint8_t v) { b.push_back(v); }
void oput16(AvBytes& b, uint16_t v) { b.push_back(static_cast<uint8_t>(v)); b.push_back(static_cast<uint8_t>(v >> 8)); }
void oput32(AvBytes& b, uint32_t v) { for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(v >> (8 * i))); }
AvBytes oldEncodeLook(const AvLook& l) {
    constexpr uint8_t kLkFemale = 0x01, kLkHairColor = 0x02, kLkBody = 0x04, kLkFace = 0x08, kEqWeapon = 0x01, kEqMods = 0x02;
    AvBytes b;
    oput8(b, kChannelAvatars);
    oput8(b, kAvKindLook2);
    oput8(b, kAvatarWireVersion);
    oput8(b, l.slot);
    const av::Appearance& a = l.appearance;
    const size_t nFace = a.faceCoeffs.size() > av::kMaxFaceCoeffs ? av::kMaxFaceCoeffs : a.faceCoeffs.size();
    uint8_t flags = a.female ? kLkFemale : 0;
    if (a.hairColor) flags |= kLkHairColor;
    if (a.height || a.weight) flags |= kLkBody;
    if (nFace) flags |= kLkFace;
    oput8(b, flags);
    oput32(b, a.race);
    oput32(b, a.hair);
    oput32(b, a.eyes);
    oput32(b, a.faceHash);
    if (flags & kLkHairColor) oput32(b, a.hairColor);
    if (flags & kLkBody) {
        oput16(b, a.height);
        oput16(b, a.weight);
    }
    if (flags & kLkFace) {
        oput8(b, static_cast<uint8_t>(nFace));
        for (size_t i = 0; i < nFace; ++i) oput8(b, static_cast<uint8_t>(a.faceCoeffs[i]));
    }
    std::string name = a.name.substr(0, kMaxLookName);
    oput8(b, static_cast<uint8_t>(name.size()));
    b.insert(b.end(), name.begin(), name.end());
    size_t n = l.equipment.size() > kMaxLookEquip ? kMaxLookEquip : l.equipment.size();
    oput8(b, static_cast<uint8_t>(n));
    for (size_t i = 0; i < n; ++i) {
        const av::EquipEntry& e = l.equipment[i];
        const uint8_t mods = static_cast<uint8_t>(e.mods & av::kWeaponModMask);
        oput32(b, e.form);
        oput8(b, static_cast<uint8_t>((e.weapon ? kEqWeapon : 0) | (mods ? kEqMods : 0)));
        if (mods) oput8(b, mods);
    }
    return b;
}
std::string hex(const AvBytes& b) {
    static const char* d = "0123456789abcdef";
    std::string s;
    for (uint8_t c : b) { s += d[c >> 4]; s += d[c & 15]; }
    return s;
}
}  // namespace

TEST(wire_look_v2_bytes_are_the_old_codecs_bytes) {
    std::vector<AvLook> corpus;
    corpus.push_back(AvLook{});
    corpus.push_back(sampleLook(1));
    uint32_t x = 20261029;
    auto rnd = [&]() { x = x * 1664525u + 1013904223u; return x >> 8; };
    for (int i = 0; i < 400; ++i) {
        AvLook l;
        l.slot = static_cast<uint8_t>(rnd() % 8);
        av::Appearance& a = l.appearance;
        a.race = rnd(); a.female = rnd() & 1; a.hair = rnd(); a.eyes = rnd(); a.faceHash = rnd();
        if (rnd() & 1) a.hairColor = rnd();
        if (rnd() & 1) { a.height = static_cast<uint16_t>(rnd()); a.weight = static_cast<uint16_t>(rnd()); }
        if (rnd() & 1) { a.faceCoeffs.resize(1 + rnd() % (av::kMaxFaceCoeffs + 4)); for (auto& c : a.faceCoeffs) c = static_cast<int8_t>(rnd()); }
        a.name = std::string(rnd() % 60, static_cast<char>('a' + rnd() % 26));
        const size_t ne = rnd() % 70;
        for (size_t k = 0; k < ne; ++k) { av::EquipEntry e; e.form = rnd(); e.weapon = rnd() & 1; e.mods = e.weapon ? static_cast<uint8_t>(rnd() & 0xFF) : 0; l.equipment.push_back(e); }
        corpus.push_back(l);
    }
    for (const AvLook& l : corpus) {
        CHECK(encodeLook(l) == oldEncodeLook(l));
        // the same bytes assembled from the shared pieces (what the Look part and the Equipment list are made of)
        AvBytes viaPieces{kChannelAvatars, kAvKindLook2, kAvatarWireVersion, l.slot};
        av::encodeAppearanceFields(viaPieces, l.appearance);
        av::encodeEquipEntries(viaPieces, l.equipment);
        CHECK(viaPieces == oldEncodeLook(l));
    }
    // the Look part's value is the appearance fields of that message and round trips
    av::Appearance a = sampleLook(1).appearance;
    a.faceCoeffs = {3, -4, 5};
    a.hairColor = 0xABCDEF;
    a.height = 1010; a.weight = 990;
    const av::LookBytes v = av::encodeAppearance(a);
    av::Appearance back;
    CHECK(av::decodeAppearance(v, back) && back.race == a.race && back.name == a.name && back.faceCoeffs == a.faceCoeffs && back.hairColor == a.hairColor && back.height == 1010);
    av::LookBytes more = v;
    more.push_back(0);
    CHECK(!av::decodeAppearance(more, back));                                  // the whole buffer or nothing
    for (size_t n = 0; n < v.size(); ++n) CHECK(!av::decodeAppearance(av::LookBytes(v.begin(), v.begin() + static_cast<long>(n)), back));
    // one frozen message: the bytes of the codec of 2026-10-29, by hand
    AvLook g;
    g.slot = 2;
    g.appearance.race = 0x00001234; g.appearance.female = true; g.appearance.hair = 0x55; g.appearance.eyes = 0x66; g.appearance.faceHash = 0x77;
    g.appearance.hairColor = 0x00A0B0C0; g.appearance.height = 1000; g.appearance.weight = 900; g.appearance.faceCoeffs = {1, -1};
    g.appearance.name = "Ann";
    g.equipment = {{0x2000, false, 0}, {0x3000, true, 3}};
    if (std::getenv("LOOK_PRINT")) std::printf("LOOK-GOLDEN %s\n", hex(oldEncodeLook(g)).c_str());
    CHECK(hex(encodeLook(g)) == "ad4302020f34120000550000006600000077000000c0b0a000e80384030201ff03416e6e020020000000003000000303");
    CHECK(hex(oldEncodeLook(g)) == "ad4302020f34120000550000006600000077000000c0b0a000e80384030201ff03416e6e020020000000003000000303");
}

namespace {
struct Mode { const char* name; bool parts, stream; };
const Mode kModes[] = {{"legacy", false, false}, {"parts unified", true, false}, {"parts and stream unified", true, true}};
constexpr uint32_t kArmor = 0x2000, kGun = 0x3000;

const av::PlayerFacts* factsAt(Node& n, PlayerId of) { return n.an->factsOf(of); }

void dress(Node& n, uint8_t gunMods) {
    n.be.weaponItems = {kGun};
    n.be.Player().worn = {kArmor, kGun};
    n.be.Player().inv[kArmor] = 1;
    n.be.Player().inv[kGun] = 1;
    n.be.Player().mods[kGun] = gunMods;
}
bool wears(Node& viewer, PlayerId of, uint32_t form) {
    const av::RefId ref = viewer.an->puppetRef(of);
    const auto* f = ref ? viewer.be.Get(ref) : nullptr;
    return f && f->worn.count(form) == 1;
}
int modsOn(Node& viewer, PlayerId of, uint32_t form) {
    const av::RefId ref = viewer.an->puppetRef(of);
    const auto* f = ref ? viewer.be.Get(ref) : nullptr;
    if (!f) return -1;
    auto it = f->mods.find(form);
    return it == f->mods.end() ? 0 : it->second;
}
}  // namespace

// Roster, look, equipment with a held weapon and its mods, facts and pack: the same assertions in every mode.
static void looksFactsPacksAndRoster(const Mode& m) {
    std::printf("    mode: %s\n", m.name);
    Rig3 r(m.parts, m.stream);
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    b.present = false;   // joins later
    for (Node* n : {&h, &a, &b}) { watchThese(*n, {kTotem, kRoot}); n->be.weaponItems = {kGun}; }   // every backend knows which item is a weapon
    dress(a, 3);
    dress(b, 1);
    h.an->setLocalAppearanceReader([](av::Appearance& ap) { ap.race = 1; ap.name = "Host"; return true; });
    a.an->setLocalAppearanceReader([](av::Appearance& ap) { ap.race = 1; ap.name = "Alpha"; ap.height = 1010; return true; });
    b.an->setLocalAppearanceReader([](av::Appearance& ap) { ap.race = 1; ap.name = "Beta"; return true; });
    a.be.Player().level = 12;
    a.be.Player().sitState = 2;
    a.be.Player().actions = {9};
    a.be.playerRegions = {0x1111, 0x2222};
    a.be.Player().inv[kTotem] = 3;
    a.be.Player().inventoryWeight = 75.96;
    r.run(5500);
    CHECK(h.an->roster() == std::set<PlayerId>({1}));
    CHECK(a.an->roster() == std::set<PlayerId>({0}));
    CHECK(h.an->hasLook(1) && h.an->lookOf(1) && h.an->lookOf(1)->appearance.name == "Alpha" && h.an->lookOf(1)->appearance.height == 1010);
    CHECK(a.an->hasLook(0) && a.an->lookOf(0)->appearance.name == "Host");
    CHECK(h.an->stats().appearanceUpdates >= 1 && a.an->stats().appearanceUpdates >= 1);   // the look reached the puppet, not only the table
    CHECK(wears(h, 1, kArmor) && wears(h, 1, kGun) && modsOn(h, 1, kGun) == 3);             // the held weapon, with the two mods it wears
    const av::PlayerFacts* fa = factsAt(h, 1);
    CHECK(fa && fa->level == 12 && fa->sitting == 2 && fa->actions == (1u << 9) && fa->regions.size() == 2 && fa->regions[0] == 0x1111);
    int64_t n = -1;
    double w = -1;
    CHECK(h.an->packCount(1, kTotem, &n) && n == 3 && h.an->packWeight(1, &w) && std::abs(w - 75.96) < 0.0006);
    CHECK(h.an->packCount(1, kRoot, &n) && n == 0);                                         // watched and not carried: a zero
    // B joins: everybody gets a puppet of everybody else, wearing what they wear, through the host's relay
    b.present = true;
    r.run(6000);
    CHECK(h.an->roster() == std::set<PlayerId>({1, 2}));
    CHECK(a.an->roster() == std::set<PlayerId>({0, 2}));
    CHECK(b.an->roster() == std::set<PlayerId>({0, 1}));
    CHECK(h.puppets() == 2 && a.puppets() == 2 && b.puppets() == 2);
    CHECK(b.an->hasLook(1) && b.an->lookOf(1)->appearance.name == "Alpha" && a.an->hasLook(2) && a.an->lookOf(2)->appearance.name == "Beta");
    CHECK(wears(b, 1, kGun) && modsOn(b, 1, kGun) == 3 && wears(a, 2, kGun) && modsOn(a, 2, kGun) == 1 && wears(h, 2, kArmor));
    CHECK(factsAt(h, 2) != nullptr);                       // B's facts reach the host, whatever its level is
    CHECK(factsAt(a, 2) == nullptr && factsAt(b, 1) == nullptr && a.an->packOf(2) == nullptr && b.an->packOf(1) == nullptr);   // facts and pack are the host's alone
    // a change travels: A swaps its armour and its gun loses a mod
    a.be.Player().worn = {0x2001, kGun};
    a.be.Player().inv[0x2001] = 1;
    a.be.Player().mods[kGun] = 1;
    a.be.Player().level = 13;
    r.run(5000);
    CHECK(wears(h, 1, 0x2001) && !wears(h, 1, kArmor) && wears(b, 1, 0x2001) && !wears(b, 1, kArmor));
    CHECK(factsAt(h, 1) && factsAt(h, 1)->level == 13);
    // B leaves: its puppets and its facts go with it
    b.present = false;
    r.run(2500);
    CHECK(h.an->roster() == std::set<PlayerId>({1}) && a.an->roster() == std::set<PlayerId>({0}));
    CHECK(h.puppets() == 1 && a.puppets() == 1 && factsAt(h, 2) == nullptr);
    // what travels: the old messages in legacy; in the unified modes none of them, and no player call ever reached the NPC port
    if (m.parts) {
        for (Node* nd : {&h, &a, &b}) CHECK(nd->an->stats().rostersSent == 0 && nd->an->stats().looksSent == 0 && nd->an->stats().factsSent == 0 && nd->an->stats().packsSent == 0);
        for (Node* nd : {&h, &a, &b}) CHECK(nd->npcPort->calls == 0);
        CHECK(h.actors->sync()->playerSlots().size() == 2);   // the host and A
    } else {
        CHECK(h.an->stats().rostersSent > 0 && a.an->stats().looksSent > 0 && a.an->stats().factsSent > 0 && a.an->stats().packsSent > 0);
    }
    CHECK(h.be.badLines.empty() && a.be.badLines.empty() && b.be.badLines.empty());
}

TEST(entities_roster_look_equipment_weapon_facts_and_pack_in_every_mode) {
    for (const Mode& m : kModes) looksFactsPacksAndRoster(m);
}

// A character creation edits the base form live: every state of the race menu would make the other machines build the puppet again. A look goes out once it has
// stayed the same for a moment (the first look and a finished step, announced with noteLookEdited(), at once), and the finished face is the one that arrives.
TEST(entities_a_look_settles_before_it_goes_out_and_a_finished_face_goes_at_once) {
    Rig3 r(true, false);
    Node& h = r.host();
    Node& a = r.addClient(1);
    auto cur = std::make_shared<av::Appearance>();
    cur->race = 1;
    cur->name = "Alpha";
    cur->hair = 0x10;
    a.an->setLocalAppearanceReader([cur](av::Appearance& ap) { ap = *cur; return true; });
    h.an->setLocalAppearanceReader([](av::Appearance& ap) { ap.race = 1; ap.name = "Host"; return true; });
    r.run(4000);
    CHECK(h.an->lookOf(1) && h.an->lookOf(1)->appearance.hair == 0x10);
    const uint64_t before = h.an->stats().appearanceUpdates;
    for (int i = 0; i < 10; ++i) {   // the race menu: a new state every 300 ms for three seconds
        cur->hair = static_cast<uint32_t>(0x20 + i);
        cur->faceCoeffs.assign(130, static_cast<int8_t>(i + 1));
        r.run(300);
    }
    CHECK(h.an->lookOf(1) && h.an->lookOf(1)->appearance.hair == 0x10);   // none of it went out while it kept changing
    CHECK(h.an->stats().appearanceUpdates == before);
    r.run(2500);                                                          // it stopped changing: the last state goes out, and only that one
    CHECK(h.an->lookOf(1) && h.an->lookOf(1)->appearance.hair == 0x29 && h.an->lookOf(1)->appearance.faceCoeffs.size() == 130 && h.an->lookOf(1)->appearance.faceCoeffs[0] == 10);
    CHECK(h.an->stats().appearanceUpdates == before + 1);                 // one new puppet, not ten
    cur->headParts = {0x70, 0x71};
    r.run(3000);
    CHECK(h.an->lookOf(1) && h.an->lookOf(1)->appearance.headParts == (std::vector<uint32_t>{0x70, 0x71}));
    // the face menu closed: the look is read and goes out without waiting for the poll and the settle time
    cur->hair = 0x30;
    cur->headParts = {0x72};
    a.an->noteLookEdited();
    r.run(900);
    CHECK(h.an->lookOf(1) && h.an->lookOf(1)->appearance.hair == 0x30 && h.an->lookOf(1)->appearance.headParts == (std::vector<uint32_t>{0x72}));
    CHECK(h.puppets() == 1 && a.puppets() == 1);
}

// Every part of a look that the race / face menus change reaches the other machine's puppet on its own: the skin tone block alone, the face shape, the hair colour, the
// height and weight, the eyes, the head parts, the sex, the race. Each is ONE new puppet, the line in nvcoop.log names what changed, and a look that did not change
// builds nothing (the settle time and the equality cover every field, not only the ones the first version compared).
TEST(entities_each_part_of_a_look_reaches_the_puppet_by_itself) {
    Rig3 r(true, false);
    Node& h = r.host();
    Node& a = r.addClient(1);
    auto cur = std::make_shared<av::Appearance>();
    cur->race = 25;
    cur->name = "Alpha";
    cur->hair = 0x10;
    cur->eyes = 0x20;
    cur->hairColor = 0x00112233;
    cur->height = 1000;
    cur->weight = 1000;
    cur->faceCoeffs.assign(130, 2);
    cur->headParts = {0x70};
    a.an->setLocalAppearanceReader([cur](av::Appearance& ap) { ap = *cur; return true; });
    h.an->setLocalAppearanceReader([](av::Appearance& ap) { ap.race = 25; ap.name = "Host"; return true; });
    std::vector<std::string> log;
    h.an->setLog([&](const std::string& l) { log.push_back(l); });
    r.run(4000);
    CHECK(h.an->lookOf(1) && h.an->lookOf(1)->appearance.hair == 0x10);
    struct Step { const char* what; std::function<void()> edit; };
    const Step steps[] = {
        {"skin tone", [&] { for (int i = 80; i < 130; ++i) cur->faceCoeffs[i] = 9; }},
        {"face shape", [&] { cur->faceCoeffs[0] = -7; }},
        {"face asymmetry", [&] { cur->faceCoeffs[60] = 5; }},
        {"hair colour", [&] { cur->hairColor = 0x00445566; }},
        {"height", [&] { cur->height = 1060; }},
        {"weight", [&] { cur->weight = 940; }},
        {"eyes", [&] { cur->eyes = 0x21; }},
        {"head parts", [&] { cur->headParts = {0x70, 0x71}; }},
        {"hair", [&] { cur->hair = 0x11; }},
        {"sex", [&] { cur->female = true; }},
        {"race", [&] { cur->race = 16970; }},
    };
    for (const Step& st : steps) {
        const uint64_t before = h.an->stats().appearanceUpdates;
        log.clear();
        st.edit();
        r.run(3500);                                                   // the settle time passes
        const av::Appearance& got = h.an->lookOf(1)->appearance;
        CHECK(h.an->stats().appearanceUpdates == before + 1);          // one new puppet for this change
        CHECK(!av::lookDiffers(got, *cur));                            // and it is built from the whole look as it is now
        bool named = false;
        for (const std::string& l : log)
            if (l.find("look of slot 1 arrived") != std::string::npos && l.find(std::string("changed: ") + st.what) != std::string::npos) named = true;
        CHECK(named);
        if (!named)
            for (const std::string& l : log) std::printf("      log: %s\n", l.c_str());
    }
    const uint64_t quiet = h.an->stats().appearanceUpdates;
    r.run(6000);                                                       // nothing changes: nothing is built
    CHECK(h.an->stats().appearanceUpdates == quiet);
    CHECK(h.puppets() == 1 && a.puppets() == 1);
}

// A player sits down on a couch (the host) and a client lies down on a bed: every OTHER machine seats that player's puppet in the same furniture, once, over a link that
// loses a fifth of the datagrams; nothing moves or animates a seated puppet; when the player gets up the puppet does too. (Nothing is told to the player's own machine.)
TEST(avatars_a_seated_player_is_seated_on_every_other_machine) {
    Rig3 r;
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    constexpr uint32_t kCouch = 0x6000, kBed = 0x6001;
    for (Node* n : {&h, &a, &b}) {
        n->be.AddFurniture(FakeGameBackend::kExtA, kCouch, av::Vec3{n->be.Player().pos.x + 30, n->be.Player().pos.y, 0});
        n->be.AddFurniture(FakeGameBackend::kExtA, kBed, av::Vec3{n->be.Player().pos.x + 90, n->be.Player().pos.y, 0});
    }
    r.run(4500);
    CHECK(h.puppets() == 2 && a.puppets() == 2 && b.puppets() == 2);
    fnvcoop::LinkParams lp;
    lp.minLatencyMs = 20;
    lp.maxLatencyMs = 50;
    lp.loss = 0.2;
    r.n.net.setLink(lp);
    auto activates = [&](Node& viewer, PlayerId of, uint32_t furniture) {
        const av::RefId ref = viewer.an->puppetRef(of);
        char want[48];
        std::snprintf(want, sizeof want, "Activate %08X 1", static_cast<unsigned>(ref));
        int n = 0;
        for (const auto& l : viewer.be.lines)
            if (ref && l.ctx == furniture && l.text == want) ++n;
        return n;
    };
    auto seatedState = [&](Node& viewer, PlayerId of) {
        auto* f = viewer.be.Get(viewer.an->puppetRef(of));
        return f ? f->sitSleep : -1;
    };
    h.be.Player().sitSleep = 4;     // the host sits on the couch
    a.be.Player().sitSleep = 9;     // client 1 lies down (a bed state; the nearest furniture to it is a couch here)
    r.run(4000);
    CHECK(activates(a, 0, kCouch) == 1 && activates(b, 0, kCouch) == 1);
    CHECK(seatedState(a, 0) == 4 && seatedState(b, 0) == 4);
    CHECK(activates(h, 1, kCouch) == 1 && activates(b, 1, kCouch) == 1);   // the nearest furniture to client 1 is the couch too (every rig places both next to its own player)
    CHECK(seatedState(h, 1) == 4 && seatedState(b, 1) == 4);
    r.run(4000);
    CHECK(activates(a, 0, kCouch) == 1 && activates(b, 0, kCouch) == 1);  // never repeated while he sits
    for (Node* v : {&a, &b}) CHECK(v->an->puppetRef(0) != 0);
    h.be.Player().sitSleep = 0;     // the host gets up
    r.run(4000);
    CHECK(seatedState(a, 0) == 0 && seatedState(b, 0) == 0);
    CHECK(seatedState(h, 1) == 4);  // client 1 is still down
    CHECK(h.be.badLines.empty() && a.be.badLines.empty() && b.be.badLines.empty());
}

// A session where the roster changes and the link to the host drops: the puppets come back, dressed, in every mode.
static void churn(const Mode& m) {
    std::printf("    mode: %s\n", m.name);
    Rig3 r(m.parts, m.stream);
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    for (Node* n : {&h, &a, &b}) n->be.weaponItems = {kGun};
    dress(a, 3);
    dress(b, 1);
    h.an->setLocalAppearanceReader([](av::Appearance& ap) { ap.race = 1; ap.name = "Host"; return true; });
    a.an->setLocalAppearanceReader([](av::Appearance& ap) { ap.race = 1; ap.name = "Alpha"; return true; });
    b.an->setLocalAppearanceReader([](av::Appearance& ap) { ap.race = 1; ap.name = "Beta"; return true; });
    a.vx = 60;
    r.run(5000);
    CHECK(h.puppets() == 2 && a.puppets() == 2 && b.puppets() == 2);
    // the link of A to the host drops and comes back (a baseline): its puppets go and return, dressed again
    a.linkUp = false;
    r.run(1500);
    CHECK(a.puppets() == 0);
    a.linkUp = true;
    r.run(6000);
    CHECK(a.puppets() == 2);
    CHECK(wears(a, 2, kGun) && modsOn(a, 2, kGun) == 1 && a.an->lookOf(2) && a.an->lookOf(2)->appearance.name == "Beta");
    CHECK(a.an->lookOf(0) && a.an->lookOf(0)->appearance.name == "Host");
    CHECK(wears(h, 1, kGun) && modsOn(h, 1, kGun) == 3);                 // nothing changed for the host's puppet of A
    // B leaves and comes back
    b.present = false;
    r.run(2500);
    CHECK(h.puppets() == 1 && a.puppets() == 1);
    b.present = true;
    r.run(6000);
    CHECK(h.puppets() == 2 && a.puppets() == 2);
    CHECK(wears(h, 2, kGun));
    CHECK(modsOn(h, 2, kGun) == 1);
    CHECK(wears(a, 2, kArmor));
    bool ok = false;
    const float x = xOfPuppet(h, 1, &ok);
    CHECK(ok && std::abs(x - playerX(a)) < 300.0f);
    if (m.parts) for (Node* nd : {&h, &a, &b}) CHECK(nd->npcPort->calls == 0);
}

TEST(entities_link_loss_leave_and_rejoin_in_every_mode) {
    for (const Mode& m : kModes) churn(m);
}

// The decoders accept both forms in every mode: a unified host still applies a legacy Look, Facts and Pack, and a unified client a legacy Roster.
TEST(entities_the_old_messages_are_still_understood_under_parts_unified) {
    Rig3 r(true, false);
    Node& a = r.addClient(1);
    r.run(2500);
    AvLook l = sampleLook(1);
    l.appearance.name = "Legacy";
    l.equipment = {{0x4444, false, 0}};
    a.mux->sendEx(0, encodeLook(l), fnvcoop::ITransport::Channel::Reliable);
    AvFacts f;
    f.slot = 1;
    f.facts.level = 77;
    a.mux->sendEx(0, encodeFacts(f), fnvcoop::ITransport::Channel::Reliable);
    AvPack pk;
    pk.slot = 1;
    pk.pack.haveWeight = true;
    pk.pack.weightMilli = 5000;
    a.mux->sendEx(0, encodePack(pk), fnvcoop::ITransport::Channel::Reliable);
    r.run(500);
    CHECK(r.host().an->lookOf(1) && r.host().an->lookOf(1)->appearance.name == "Legacy");
    CHECK(r.host().an->factsOf(1) && r.host().an->factsOf(1)->level == 77);
    double w = 0;
    CHECK(r.host().an->packWeight(1, &w) && w == 5.0);
    Bytes roster = encodeRoster({kHostPlayer, 1});   // an old host's roster at a unified client: applied (and the same set the records give)
    r.host().mux->sendEx(1, roster, fnvcoop::ITransport::Channel::Reliable);
    r.run(500);
    CHECK(a.an->roster() == std::set<PlayerId>({0}));
}

// stream=unified: the player's snapshots go through ActorSync (own sender and receiver pair, pinned owner and epoch), the puppets follow as before.
TEST(entities_the_stream_through_actor_sync_follows_like_the_avatar_channel) {
    Rig3 r(true, true);
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    a.present = true;
    b.present = false;
    a.vx = 150;
    r.run(2500);
    CHECK(h.an->roster() == std::set<PlayerId>({1}));
    bool ok = false;
    float ax = xOfPuppet(h, 1, &ok);
    CHECK(ok && std::abs(ax - playerX(a)) < 150.0f);
    float hx = xOfPuppet(a, 0, &ok);
    CHECK(ok && std::abs(hx - playerX(h)) < 30.0f);
    b.present = true;
    b.vx = -100;
    r.run(3500);
    CHECK(h.puppets() == 2 && a.puppets() == 2 && b.puppets() == 2);
    float bxAtA = xOfPuppet(a, 2, &ok);
    CHECK(ok && std::abs(bxAtA - playerX(b)) < 150.0f);   // A sees B only through the host's relay
    float axAtB = xOfPuppet(b, 1, &ok);
    CHECK(ok && std::abs(axAtB - playerX(a)) < 180.0f);
    // the traffic went through ActorSync, not the avatar channel
    for (Node* nd : {&h, &a, &b}) {
        CHECK(nd->an->stats().sent == 0 && nd->an->stats().accepted > 0);
        const nvcoop::actors::Stats& st = nd->actors->sync()->stats();
        CHECK(st.playerSampled > 0 && st.playerSent > 0 && st.playerForged == 0);
        CHECK(nd->npcPort->calls == 0);
    }
    CHECK(h.actors->sync()->stats().playerRelayed > 0 && h.actors->sync()->stats().playerAccepted > 0);
    CHECK(a.actors->sync()->playerSenderStats().packets > 0 && a.actors->sync()->playerReceiverStats().accepted > 0);
    // A claims to be B: the pinned owner is B, the host drops it
    live::SnapshotSender evil;
    live::ActorSnapshot s;
    s.netId = playerNetId(2);
    s.ownerEpoch = h.actors->sync()->ownerOf(playerNetId(2)).epoch;
    s.seq = 900000; s.tick = 5; s.cell = kCell;
    s.pos = {90000, 0, 0};
    evil.queue(0, s, 5);
    for (auto& d : evil.flush(0, 5)) a.mux->sendEx(0, nvcoop::actors::wrapLive(nvcoop::actors::SubBatchPlayer, d), fnvcoop::ITransport::Channel::Unreliable);
    r.run(900);
    CHECK(h.actors->sync()->stats().playerForged >= 1);
    const float after = xOfPuppet(h, 2);
    CHECK(std::abs(after - playerX(b)) < 200.0f && std::abs(after - 90000.0f) > 1000.0f);
    // a client that leaves and comes back (a higher epoch) is followed again
    a.vx = 0;
    a.present = false;
    r.run(1500);
    CHECK(h.puppets() == 1);
    a.be.Player().pos.x = 50000;
    a.present = true;
    r.run(3500);
    float x = xOfPuppet(h, 1, &ok);
    CHECK(ok && std::abs(x - playerX(a)) < 300.0f);
    CHECK(h.actors->sync()->ownerOf(playerNetId(1)).epoch >= 2);
}

// The lab's spawn-room case: a teleport arrives at once through ActorSync's profile, too (the same limits as the avatar channel's).
TEST(entities_teleport_through_actor_sync_arrives_at_once) {
    Rig3 r(true, true);
    quickLink(r);
    Node& h = r.host();
    Node& a = r.addClient(1);
    h.be.Player().pos = {0, 0, 0};
    a.be.Player().pos = {9000, 0, 0};
    r.run(5000);
    bool ok = false;
    CHECK(std::abs(xOfPuppet(a, 0, &ok) - 0.0f) < 100.0f && ok);
    h.be.Player().pos = {8800, 0, 0};
    a.be.Player().pos = {9200, 0, 0};
    const uint64_t t0 = r.n.now;
    uint64_t hostSeesClient = 0, clientSeesHost = 0;
    while (r.n.now - t0 < 4000 && (!hostSeesClient || !clientSeesHost)) {
        r.step(16);
        const float cx = xOfPuppet(h, 1, &ok);
        if (!hostSeesClient && ok && std::abs(cx - 9200.0f) < 100.0f) hostSeesClient = r.n.now - t0;
        const float hx = xOfPuppet(a, 0, &ok);
        if (!clientSeesHost && ok && std::abs(hx - 8800.0f) < 100.0f) clientSeesHost = r.n.now - t0;
    }
    std::printf("    teleport through ActorSync: host sees the client after %llu ms, the client sees the host after %llu ms\n", (unsigned long long)hostSeesClient, (unsigned long long)clientSeesHost);
    CHECK(hostSeesClient > 0 && hostSeesClient <= 400);
    CHECK(clientSeesHost > 0 && clientSeesHost <= 400);
    uint64_t worst = 0;   // one player jumps next to one he has not heard of: no wait for the far tier's turn
    for (int phase = 0; phase < 8; ++phase) {
        Rig3 r2(true, true);
        quickLink(r2);
        Node& h2 = r2.host();
        Node& a2 = r2.addClient(1);
        h2.be.Player().pos = {0, 0, 0};
        a2.be.Player().pos = {9000, 0, 0};
        r2.run(5000);
        for (int k = 0; k < phase * 2; ++k) r2.step(16);
        h2.be.Player().pos = {8700, 0, 0};
        worst = std::max(worst, arrivalAfterTeleport(r2, a2, 0, h2, 8700.0f, 100.0f, 4000));
    }
    std::printf("    teleport through ActorSync: the client sees the host after at most %llu ms\n", (unsigned long long)worst);
    CHECK(worst <= 130);
}

int main() {
    std::setvbuf(stdout, nullptr, _IONBF, 0);
    run("switches_default_off", switches_default_off);
    run("switches_parse_values", switches_parse_values);
    run("switches_dependencies", switches_dependencies);
    run("switches_social", switches_social);
    run("switches_entities", switches_entities);
    run("hub_disabled_service_is_never_called", hub_disabled_service_is_never_called);
    run("hub_session_lifecycle_host", hub_session_lifecycle_host);
    run("hub_existing_clients_are_announced_after_begin", hub_existing_clients_are_announced_after_begin);
    run("hub_client_link_events", hub_client_link_events);
    run("hub_new_mux_is_a_new_session_and_old_handlers_are_inert", hub_new_mux_is_a_new_session_and_old_handlers_are_inert);
    run("hub_routing_and_channel_ownership", hub_routing_and_channel_ownership);
    run("hub_send_paths", hub_send_paths);
    run("hub_loads_and_idle", hub_loads_and_idle);
    run("hub_client_reslot_is_a_new_session", hub_client_reslot_is_a_new_session);
    run("hub_a_throwing_service_is_switched_off_and_the_others_go_on", hub_a_throwing_service_is_switched_off_and_the_others_go_on);
    run("hub_status_json", hub_status_json);
#ifdef NVLIVE_HAVE_ACTOR_API
    run("actors_cfg_defaults_and_overrides", actors_cfg_defaults_and_overrides);
    run("actors_cfg_clamps_and_rejects_nonsense", actors_cfg_clamps_and_rejects_nonsense);
#endif
    run("wire_roster_roundtrip_and_truncation", wire_roster_roundtrip_and_truncation);
    run("wire_look_roundtrip_and_truncation", wire_look_roundtrip_and_truncation);
    run("wire_facts_roundtrip_and_truncation", wire_facts_roundtrip_and_truncation);
    run("wire_pack_roundtrip_and_truncation", wire_pack_roundtrip_and_truncation);
    run("wire_player_net_ids", wire_player_net_ids);
    run("wire_look_v2_bytes_are_the_old_codecs_bytes", wire_look_v2_bytes_are_the_old_codecs_bytes);
    run("avatars_three_machines_see_each_other", avatars_three_machines_see_each_other);
    run("avatars_attacks_show_on_every_other_machine", avatars_attacks_show_on_every_other_machine);
    run("avatars_a_seated_player_is_seated_on_every_other_machine", avatars_a_seated_player_is_seated_on_every_other_machine);
    run("avatars_never_a_puppet_of_oneself", avatars_never_a_puppet_of_oneself);
    run("avatars_leave_removes_puppets_everywhere", avatars_leave_removes_puppets_everywhere);
    run("avatars_rejoin_follows_again", avatars_rejoin_follows_again);
    run("avatars_forged_stream_is_rejected_at_the_host", avatars_forged_stream_is_rejected_at_the_host);
    run("avatars_look_and_equipment_travel", avatars_look_and_equipment_travel);
    run("avatars_facts_travel_to_the_host_on_change_and_only_from_the_guest_itself", avatars_facts_travel_to_the_host_on_change_and_only_from_the_guest_itself);
    run("avatars_pack_travels_to_the_host_on_change_rate_limited_and_only_from_the_guest_itself", avatars_pack_travels_to_the_host_on_change_rate_limited_and_only_from_the_guest_itself);
    run("avatars_pack_zero_is_trusted_only_for_the_same_watch_list", avatars_pack_zero_is_trusted_only_for_the_same_watch_list);
    run("avatars_pack_is_unknown_before_it_arrives_and_an_unreadable_player_sends_nothing", avatars_pack_is_unknown_before_it_arrives_and_an_unreadable_player_sends_nothing);
    run("avatars_unreadable_local_player_sends_nothing", avatars_unreadable_local_player_sends_nothing);
    run("avatars_pose_sink_reports_remote_players", avatars_pose_sink_reports_remote_players);
    run("avatars_load_purges_orphans_without_a_session", avatars_load_purges_orphans_without_a_session);
    run("avatars_load_inside_a_session_drops_and_respawns", avatars_load_inside_a_session_drops_and_respawns);
    run("avatars_session_end_takes_puppets_down", avatars_session_end_takes_puppets_down);
    run("avatars_ghost_ref_map", avatars_ghost_ref_map);
    run("avatars_the_rate_planner_sends_a_jump_at_once", avatars_the_rate_planner_sends_a_jump_at_once);
    run("avatars_a_snapshot_ahead_of_the_roster_reaches_the_puppet", avatars_a_snapshot_ahead_of_the_roster_reaches_the_puppet);
    run("avatars_teleport_arrives_at_once", avatars_teleport_arrives_at_once);
    run("avatars_teleport_of_one_player_next_to_a_player_he_has_not_heard_of_yet", avatars_teleport_of_one_player_next_to_a_player_he_has_not_heard_of_yet);
    run("entities_roster_look_equipment_weapon_facts_and_pack_in_every_mode", entities_roster_look_equipment_weapon_facts_and_pack_in_every_mode);
    run("entities_a_look_settles_before_it_goes_out_and_a_finished_face_goes_at_once", entities_a_look_settles_before_it_goes_out_and_a_finished_face_goes_at_once);
    run("entities_each_part_of_a_look_reaches_the_puppet_by_itself", entities_each_part_of_a_look_reaches_the_puppet_by_itself);
    run("entities_link_loss_leave_and_rejoin_in_every_mode", entities_link_loss_leave_and_rejoin_in_every_mode);
    run("entities_the_old_messages_are_still_understood_under_parts_unified", entities_the_old_messages_are_still_understood_under_parts_unified);
    run("entities_the_stream_through_actor_sync_follows_like_the_avatar_channel", entities_the_stream_through_actor_sync_follows_like_the_avatar_channel);
    run("entities_teleport_through_actor_sync_arrives_at_once", entities_teleport_through_actor_sync_arrives_at_once);
    std::printf("%d checks, %d failed\n", g_checks, g_fail);
    return g_fail ? 1 : 0;
}
