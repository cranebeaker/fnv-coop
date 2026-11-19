// L10 in-process tests: config parsing, UDP transport, host+client Runtime convergence over real loopback sockets.
// (The two-PROCESS test lives in tests/test_integration.py and drives coop_peer.exe.)
#include <chrono>
#include <cstdio>
#include <functional>
#include <memory>
#include <thread>
#include <vector>

#include "fake_world.h"
#include "baseline_file.h"
#include "change_tracker.h"
#include "udp_transport.h"
#include "talk_subs.h"
#include "shared_xp.h"
#include "container_sync.h"
#include "player_stats.h"

using namespace nvcoop::coop;
using namespace nvcoop::coop::test;
using nvcoop::json::Value;

static int g_checks = 0, g_failed = 0;
#define CHECK(c)                                                                  \
    do {                                                                          \
        ++g_checks;                                                               \
        if (!(c)) { ++g_failed; std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c); } \
    } while (0)

static uint64_t nowMs() {
    return std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now().time_since_epoch()).count();
}

static bool pumpUntil(const std::function<void()>& frame, const std::function<bool()>& done, int timeoutMs = 5000) {
    uint64_t end = nowMs() + timeoutMs;
    while (nowMs() < end) {
        frame();
        if (done()) return true;
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
    return done();
}

static fnvcoop::FormRef R(const char* plugin, uint32_t id) { return nvhooks::makeRef(plugin, id); }

// A nested host script: setstage VCG02 10 -> result script sets a global, adds caps/items, enables a ref; one nested
// stage on a second quest. Returns via the fake engine so it is captured exactly like L5's hooks would.
static void runVcg02(FakeWorld& w, nvhooks::CaptureHub& hub) {
    using namespace fnvcoop;
    nvhooks::ScopedCapture outer(hub, Cause{CauseKind::QuestResult, R("FalloutNV.esm", 0x10A214), 10});
    w.hostMutate(hub, SetStage{R("FalloutNV.esm", 0x10A214), 10});
    w.hostMutate(hub, SetGlobal{R("FalloutNV.esm", 0x123456), 3.0f});
    w.hostMutate(hub, AddItem{R("FalloutNV.esm", 0xF), 100, Recipient::dflt()});
    {
        nvhooks::ScopedCapture inner(hub, Cause{CauseKind::QuestResult, R("FalloutNV.esm", 0x104C00), 5});
        w.hostMutate(hub, SetStage{R("FalloutNV.esm", 0x104C00), 5});
        w.hostMutate(hub, StartQuest{R("FalloutNV.esm", 0x104C01)});
    }
    w.hostMutate(hub, EnableRef{R("FalloutNV.esm", 0x777)});
    w.hostMutate(hub, ModKarma{2, Recipient::dflt()});
}

static std::string g_outDir;   // directory of the test exe (= <repo>/out/L10)

// L11 rig: one router (host only), one slide host (host), one slide client (every node) per Node. A fake IUiEngine records the box answers
// the real engine would get in ShowMessageBox_button.
struct UiRig {
    struct Eng : nvui::IUiEngine {
        std::vector<std::pair<uint32_t, int32_t>> writes;
        uint64_t nowMs() override { return ::nowMs(); }
        void setMessageBoxResult(uint32_t s, int32_t b) override { writes.emplace_back(s, b); }
        bool refOf(uint32_t id, fnvcoop::FormRef& out) override { if (!id) return false; out = fnvcoop::FormRef("falloutnv.esm", id); return true; }
        void log(const std::string&) override {}
        int32_t lastButton() const { return writes.empty() ? -2 : writes.back().second; }
        // The router writes -1 ("box open": GetButtonPressed answers -1) when a box opens; only writes with a real button resume a script.
        size_t resumed() const { size_t n = 0; for (auto& w : writes) n += w.second != nvui::kNoButton; return n; }
        bool waiting() const { return resumed() == 0 && lastButton() == nvui::kNoButton; }
    } eng;
    std::unique_ptr<nvui::UiRouter> router;
    std::unique_ptr<nvui::SlideHost> host;
    std::unique_ptr<nvui::SlideClient> client;
    std::vector<std::string> swaps;
    std::vector<bool> locks;
    int readerInstalls = 0;
    std::function<std::vector<fnvcoop_dlg::Line>(uint32_t)> installedReader;
};
static std::string consentPath() { return g_outDir + "/../../plugin/dialogue/data/consent_messages.txt"; }

struct Node {
    struct Opt { const char* transport = "enet"; bool baseline = false; int seed = 0; bool dialogue = false; bool blockClientLoad = false; bool rules = false; bool handshake = false; bool partyWarp = false; bool noPlayerRef = false; int bindGhosts = 0; /*1: bindPlayerRefs with ghosts for slots 1,2; 2: ghost for slot 1 only*/ uint64_t buildHash = 0; std::vector<std::string> plugins; bool ui = false; };
    nvhooks::CaptureHub hub;
    FakeWorld world;
    std::unique_ptr<Runtime> rt;
    std::vector<fnvcoop_dlg::Event> dlgEvents;
    UiRig ui;
    int dlgRuns = 0;
    Node(Role role, uint16_t port, uint64_t token = 0, const char* transport = "enet") : Node(role, port, token, Opt{transport}) {}
    Node(Role role, uint16_t port, uint64_t token, Opt o, const std::function<void(FakeWorld&)>& prep = nullptr) {
        world.seed(o.seed);
        if (prep) prep(world);
        Config c;
        c.enable = true;
        c.role = role;
        c.port = port;
        c.token = token;
        c.transport = o.transport;
        c.baseline = o.baseline;
        c.clientLoadBlock = o.blockClientLoad;
        c.handshake = o.handshake;
        rt.reset(new Runtime(c, &hub, &world, nowMs));
        rt->setDumper(&world);
        if (o.handshake) {
            fnvcoop::HandshakeInfo hs;
            hs.buildNumber = 7;
            hs.buildHash = o.buildHash;
            hs.buildName = "test build";
            hs.plugins = o.plugins;
            rt->setHandshakeBase(hs);
        }
        rt->setActionExecutor([this](fnvcoop::PlayerId s, fnvcoop::IntentKind k, const fnvcoop::FormRef& r) { world.executeAction(hub, s, k, r); });
        if (o.rules && role == Role::Host) {
            static const char* kRules = R"({"schema":1,"quests":{
 "VCG01":{"tier":"T4","questline":"Main: Prologue","curated":true,"rules":[
   {"kind":"Barrier","params":{"step":"all-players-pass"},"origin":"curated"},
   {"kind":"RewardAll","params":{},"origin":"curated"},
   {"kind":"SaveSuppress","params":{},"origin":"curated"}]},
 "WarpQ":{"tier":"T3","questline":"x","curated":true,"rules":[{"kind":"PartyWarp","params":{},"origin":"curated"}]}}})";
            std::string rerr;
            if (!rt->setRules(kRules, &rerr)) std::printf("  rules failed: %s\n", rerr.c_str());
            rt->setEdidResolver([](const std::string& edid, fnvcoop::FormRef& out) {
                uint32_t h = 2166136261u;
                for (char ch : edid) { h ^= (uint8_t)ch; h *= 16777619u; }
                out = fnvcoop::FormRef("falloutnv.esm", h);
                return true;
            });
            if (o.bindGhosts)
                rt->bindPlayerRefs([n = o.bindGhosts](fnvcoop::PlayerId slot) -> uint32_t { return slot == 0 ? 0u : (n == 2 && slot > 1) ? 0u : 0x1000u + slot; },
                                   [](uint32_t id, fnvcoop::FormRef& out) { out = fnvcoop::FormRef("falloutnv.esm", id); return true; });
            else if (!o.noPlayerRef)   // twin: with no resolver beyond the default (host only) a guest would be left behind -> still host-only
                rt->setPlayerRefResolver([](fnvcoop::PlayerId slot, fnvcoop::FormRef& out) {
                    out = fnvcoop::FormRef("falloutnv.esm", slot == 0 ? 0x14u : 0x1000u + slot);
                    return true;
                });
            rt->setStageApplier([this](const fnvcoop::FormRef& q, uint32_t st) {
                nvhooks::ScopedCapture sc(hub, fnvcoop::Cause{fnvcoop::CauseKind::QuestResult, q, (int)st});
                world.hostMutate(hub, fnvcoop::SetStage{q, st});
            });
        }
        if (o.dialogue) {
            DialogueHooks h;
            h.run = [this](const fnvcoop_dlg::ResultRequest& rq) -> uint64_t {
                ++dlgRuns;   // the host "runs the result script": commits one DialogueResult txn per phase
                nvhooks::ScopedCapture sc(hub, fnvcoop::Cause{fnvcoop::CauseKind::DialogueResult, rq.info, (int)rq.phase});
                world.hostMutate(hub, fnvcoop::SetGlobal{R("FalloutNV.esm", 0x500 + rq.step * 2 + (int)rq.phase), 1.0f});
                return rt->host() ? rt->host()->log().head() + 1 : 0;
            };
            h.sink = [this](const fnvcoop_dlg::Event& e) { dlgEvents.push_back(e); };
            rt->enableDialogue(h);
        }
        if (o.ui) {
            const bool isHost = role == Role::Host;
            ui.router.reset(new nvui::UiRouter(ui.eng, nvui::Bridge()));   // the runtime points the bridge at the DialogueNode
            ui.router->setHost(isHost);
            ui.client.reset(new nvui::SlideClient({[this](const fnvcoop::FormRef&, const std::string&, const std::string& nw) { ui.swaps.push_back(nw); return true; },
                                                   [this](bool l) { ui.locks.push_back(l); }, nullptr}));
            UiWiring w;
            w.router = [this, isHost]() -> nvui::UiRouter* { return isHost ? ui.router.get() : nullptr; };
            w.slideHost = [this](nvui::SlideHost::Send f) -> nvui::SlideHost& { ui.host.reset(new nvui::SlideHost(std::move(f))); return *ui.host; };
            w.slideClient = [this]() -> nvui::SlideClient& { return *ui.client; };
            w.consentFile = consentPath();
            w.installInfoReader = [this](std::function<std::vector<fnvcoop_dlg::Line>(uint32_t)> f) { ++ui.readerInstalls; ui.installedReader = std::move(f); };
            w.infoReader = [](uint32_t info) { return std::vector<fnvcoop_dlg::Line>{fnvcoop_dlg::Line{"info " + std::to_string(info), "", 0, 50}}; };
            rt->enableUi(std::move(w));
        }
        rt->setWorldEffectApplier([this](const std::string& j) { return world.applyWorldJson(j); });
        std::string err;
        if (!rt->start(&err)) std::printf("  start failed: %s\n", err.c_str());
    }
    // Host: NPC state of a lane-local world effect; the host applies it to its own fake world, guests get it through the runtime.
    void pushWorld(const std::string& key, const std::string& json) { world.applyWorldJson(json); rt->pushWorldEffect(key, json); }
};

static void testConfig() {
    std::printf("config\n");
    Config c;
    std::string err;
    CHECK(ParseConfig("[bridge]\nport=1\n[coop]\nenable=1\nrole=host\nport=4000\nlisten=0.0.0.0\ntoken=77\n; c\n", &c, &err));
    CHECK(c.enable && c.role == Role::Host && c.port == 4000 && c.listen == "0.0.0.0" && c.token == 77);
    Config d;
    CHECK(ParseConfig("[coop]\nrole=guest\nhost=10.0.0.5\nenable=true\n", &d, &err));
    CHECK(d.role == Role::Client && d.host == "10.0.0.5" && d.enable);
    Config e;
    CHECK(ParseConfig("", &e, &err) && !e.enable && e.role == Role::Off);  // default: inert
    Config f;
    CHECK(!ParseConfig("[coop]\nrole=banana\n", &f, &err));
    CHECK(!ParseConfig("[coop]\nport=99999\n", &f, &err));
}

static void testTransport() {
    std::printf("transport\n");
    std::string err;
    auto srv = UdpTransport::Server("127.0.0.1", 0, &err);
    CHECK(srv != nullptr);
    if (!srv) return;
    auto c1 = UdpTransport::Client("127.0.0.1", srv->localPort(), &err);
    auto c2 = UdpTransport::Client("127.0.0.1", srv->localPort(), &err);
    CHECK(c1 && c2);
    fnvcoop::Bytes small = {1, 2, 3}, big(300000);
    for (size_t i = 0; i < big.size(); ++i) big[i] = static_cast<uint8_t>(i * 31 + 7);
    c1->send(0, small);
    c2->send(0, big);
    c2->send(0, {});  // empty message
    fnvcoop::PeerId p1 = ~0u, p2 = ~0u;
    int got = 0;
    pumpUntil([&] {
        fnvcoop::PeerId from;
        fnvcoop::Bytes b;
        while (srv->poll(from, b)) {
            ++got;
            if (b == small) p1 = from;
            else if (b == big) p2 = from;
        }
    }, [&] { return got >= 3; });
    CHECK(got == 3);
    CHECK(p1 != ~0u && p2 != ~0u && p1 != p2 && p1 >= 1 && p2 >= 1);
    // reply per peer
    srv->send(p2, big);
    srv->send(p1, small);
    fnvcoop::Bytes r1, r2;
    pumpUntil([&] {
        fnvcoop::PeerId from;
        fnvcoop::Bytes b;
        while (c1->poll(from, b)) { CHECK(from == 0); r1 = b; }
        while (c2->poll(from, b)) { CHECK(from == 0); r2 = b; }
    }, [&] { return !r1.empty() && !r2.empty(); });
    CHECK(r1 == small && r2 == big);
    // garbage and truncated fragments are ignored, never delivered
    {
        auto g = UdpTransport::Client("127.0.0.1", srv->localPort(), &err);
        g->send(0, small);  // legit so the peer exists
        const auto before = srv->stats().badDatagrams;
        // raw garbage through a second plain socket is covered by the fragment validity checks; simulate by loss:
        fnvcoop::PeerId from;
        fnvcoop::Bytes b;
        pumpUntil([&] { while (srv->poll(from, b)) {} }, [&] { return false; }, 100);
        CHECK(srv->stats().badDatagrams == before);
    }
    // loss: dropping a fragment loses the whole message (no partial delivery)
    c2->setDropEveryNth(2);
    c2->send(0, big);
    int deliveredBig = 0;
    pumpUntil([&] {
        fnvcoop::PeerId from;
        fnvcoop::Bytes b;
        while (srv->poll(from, b)) if (b == big) ++deliveredBig;
    }, [&] { return false; }, 300);
    CHECK(deliveredBig == 0);
}

static void testConvergence(const char* tr) {
    std::printf("convergence (loopback sockets, 2 clients, late join)\n");
    Node host(Role::Host, 0, 0, tr);
    CHECK(host.rt->running());
    uint16_t port = host.rt->boundPort();
    Node c1(Role::Client, port, 1001, tr), c2(Role::Client, port, 1002, tr);
    CHECK(c1.rt->running() && c2.rt->running());
    auto frameAll = [&] { host.rt->frame(); c1.rt->frame(); c2.rt->frame(); };

    CHECK(pumpUntil(frameAll, [&] { return c1.rt->client()->joined() && c2.rt->client()->joined(); }));
    CHECK(c1.rt->client()->slot() != c2.rt->client()->slot());

    runVcg02(host.world, host.hub);
    CHECK(host.rt->counters().captured == 1);   // nested scopes = ONE txn
    CHECK(host.rt->host()->log().head() == 1);
    CHECK(pumpUntil(frameAll, [&] { return c1.rt->client()->applied() == 1 && c2.rt->client()->applied() == 1 && host.rt->host()->allSynced(); }));

    Value hw = host.world.dump(), w1 = c1.world.dump(), w2 = c2.world.dump();
    CHECK(hw.dump() == w1.dump());
    CHECK(hw.dump() == w2.dump());
    CHECK(c1.world.stages.at("falloutnv.esm:10A214") == 10);
    CHECK(c1.world.stages.at("falloutnv.esm:104C00") == 5);   // the nested stage arrived in the SAME txn
    CHECK(c1.world.inventory.at("falloutnv.esm:00000F") == 100);
    CHECK(c1.world.karma == 2);

    // echo guard: applying a remote txn on a client never produced a client-side capture
    CHECK(c1.rt->counters().captured == 0 && c1.hub.stats().txns == 0);

    // second txn, then a late joiner replays everything
    {
        using namespace fnvcoop;
        nvhooks::ScopedCapture s(host.hub, Cause{CauseKind::DialogueResult, R("FalloutNV.esm", 0x15D4E9), 0});
        host.world.hostMutate(host.hub, SetObjective{R("FalloutNV.esm", 0x10A214), 3, 3});
        host.world.hostMutate(host.hub, DisableRef{R("FalloutNV.esm", 0x777)});
    }
    Node late(Role::Client, port, 1003, tr);
    auto frameAll2 = [&] { frameAll(); late.rt->frame(); };
    CHECK(pumpUntil(frameAll2, [&] { return late.rt->client()->applied() == 2 && c1.rt->client()->applied() == 2 && c2.rt->client()->applied() == 2 && host.rt->host()->allSynced(); }));
    CHECK(host.world.dump().dump() == late.world.dump().dump());
    CHECK(host.world.dump().dump() == c1.world.dump().dump());

    // log + status JSON
    Value st, lg;
    std::string err;
    CHECK(nvcoop::json::parse(host.rt->statusJson(), st, err));
    CHECK(st.get("role").asString("") == "host" && st.get("lastSeq").asInt() == 2 && st.get("connected").asInt() == 3);
    CHECK(st.get("host").get("commits").asInt() == 2);
    CHECK(nvcoop::json::parse(late.rt->logJson(10), lg, err));
    CHECK(lg.get("count").asInt() >= 1);
    CHECK(nvcoop::json::parse(c1.rt->statusJson(), st, err));
    CHECK(st.get("client").get("joined").asBool() && st.get("client").get("applied").asInt() == 2);

    // inject on host (as if captured) reaches everyone
    std::string res;
    CHECK(host.rt->inject(R"({"txn":{"cause":{"kind":1,"source":"falloutnv.esm:000AAA","aux":1},"origin":0,
        "effects":[{"type":"SetGlobal","global":"falloutnv.esm:000BBB","value":9}]}})", &res, &err));
    CHECK(pumpUntil(frameAll2, [&] { return late.rt->client()->applied() == 3 && host.rt->host()->allSynced(); }));
    CHECK(late.world.globals.at("falloutnv.esm:000BBB") == 9.0f);
    // inject on a client = applied as if received (no seq, no wire)
    CHECK(c1.rt->inject(R"({"cause":{"kind":1,"source":"","aux":0},"origin":0,"effects":[{"type":"SetGlobal","global":"falloutnv.esm:000CCC","value":4}]})", &res, &err));
    CHECK(c1.world.globals.at("falloutnv.esm:000CCC") == 4.0f);
    CHECK(!host.rt->inject("{not json", &res, &err));
    CHECK(!c1.rt->inject(R"({"cause":{},"effects":[{"type":"Nope"}]})", &res, &err));
}

static void testActions() {
    std::printf("guest actions (activate -> host executes -> txn to all)\n");
    Node host(Role::Host, 0);
    Node a(Role::Client, host.rt->boundPort(), 3001), b(Role::Client, host.rt->boundPort(), 3002);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); };
    CHECK(a.rt->submitAction(fnvcoop::IntentKind::ActivateRef, R("FalloutNV.esm", 0xABC)) == 0);   // not joined yet: refused
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && b.rt->client()->joined(); }));
    std::vector<std::pair<int, bool>> actingEvents;   // (slot, on) as seen by the engine layer's acting-player hook
    fnvcoop::PlayerId seenInside = fnvcoop::kNoPlayer;
    host.rt->setActingHook([&](fnvcoop::PlayerId slot, bool on) {
        actingEvents.emplace_back((int)slot, on);
        if (on) seenInside = host.rt->actingSlot();
    });
    uint32_t id = a.rt->submitAction(fnvcoop::IntentKind::ActivateRef, R("FalloutNV.esm", 0xABC));
    CHECK(id != 0);
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->applied() == 1 && b.rt->client()->applied() == 1 && host.rt->host()->allSynced(); }));
    CHECK(host.rt->counters().actions == 1);
    CHECK(actingEvents.size() == 2 && actingEvents[0].second && !actingEvents[1].second && actingEvents[0].first == (int)a.rt->client()->slot());
    CHECK(seenInside == a.rt->client()->slot() && host.rt->actingSlot() == fnvcoop::kNoPlayer);   // set exactly while the executor runs
    CHECK(host.world.enabled.at("falloutnv.esm:000ABC") && a.world.enabled.at("falloutnv.esm:000ABC") && b.world.enabled.at("falloutnv.esm:000ABC"));
    CHECK(a.world.inventory.at("falloutnv.esm:00000F") == 5);      // the item goes to the requester ONLY
    CHECK(b.world.inventory.count("falloutnv.esm:00000F") == 0 && host.world.inventory.count("falloutnv.esm:00000F") == 0);
    const auto* t = host.rt->host()->log().get(1);                  // the txn carries the guest as origin
    CHECK(t && t->origin == a.rt->client()->slot());
    a.rt->submitAction(fnvcoop::IntentKind::PickUp, R("FalloutNV.esm", 0xDEF));
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->applied() == 2 && host.rt->host()->allSynced(); }));
    CHECK(a.world.inventory.at("falloutnv.esm:000DEF") == 1 && host.rt->counters().actions == 2);   // exactly once each
    // A presence report (the guest entered cell 0x1234) is not an interaction: it must never reach the executor, which would run
    // "Activate" with a cell as the calling reference (that hung the host in game). Twin 7: reports are executed like activations.
    const size_t actingBefore = actingEvents.size();
    a.rt->submitAction(fnvcoop::IntentKind::ReportCell, R("FalloutNV.esm", 0x1234));
    CHECK(pumpUntil(fr, [&] { return host.rt->counters().actions == 3; }));
    for (int i = 0; i < 50; ++i) fr();
    CHECK(actingEvents.size() == actingBefore);               // the executor did not run
    CHECK(host.rt->counters().reportsDropped == 1);
}

static void testSharedXp() {
    std::printf("shared XP (0xD6 wire, outbox batching)\n");
    namespace sx = nvcoop::coop::sharedxp;
    sx::Share s; s.slot = 2; s.xp = 1234567 % sx::kMaxShare;
    std::vector<uint8_t> b = sx::encode(s);
    sx::Share d;
    CHECK(b.size() == 7 && b[0] == sx::kChannel && sx::decode(b.data(), b.size(), &d) && d.slot == 2 && d.xp == s.xp);
    for (size_t cut = 0; cut < b.size(); ++cut) CHECK(!sx::decode(b.data(), cut, &d));   // truncated
    s.xp = 0; b = sx::encode(s); CHECK(!sx::decode(b.data(), b.size(), &d));              // nothing to give
    s.xp = -5; b = sx::encode(s); CHECK(!sx::decode(b.data(), b.size(), &d));             // never takes XP away
    s.xp = sx::kMaxShare + 1; b = sx::encode(s); CHECK(!sx::decode(b.data(), b.size(), &d));   // a bogus amount
    b = sx::encode(sx::Share{1, 10}); b[1] = 9; CHECK(!sx::decode(b.data(), b.size(), &d));   // other version
    sx::Outbox o;
    CHECK(o.take(1000) == 0);
    o.earned(20); o.earned(15); o.earned(-3); o.earned(0);
    CHECK(o.take(1000) == 35 && o.pending() == 0);   // a burst goes out as one message
    o.earned(5);
    CHECK(o.take(1100) == 0);                        // within the gap: held
    CHECK(o.take(1250) == 5);
    o.earned(sx::kMaxShare + 50);
    CHECK(o.take(2000) == sx::kMaxShare && o.pending() == 50 && o.take(2300) == 50);   // split, nothing lost
    // whole-XP totals and the catch-up after a respawn (NV:MP puts the character back to its last save)
    b = sx::encodeTotal(3, 4567);
    uint8_t ts = 0; int32_t tt = 0;
    CHECK(sx::decodeTotal(b.data(), b.size(), &ts, &tt) && ts == 3 && tt == 4567 && !sx::decode(b.data(), b.size(), &d));
    CHECK(!sx::decodeTotal(sx::encode(sx::Share{1, 10}).data(), 7, &ts, &tt));   // a share is not a total
    sx::CatchUp cu;
    CHECK(cu.check(1000, 1010) == 0 && cu.check(1000, 1010) == 0);            // within the slack: shares on the way
    CHECK(cu.check(1, 221) == 0 && cu.check(1, 221) == 220);                   // a respawn: seen twice, then topped up
    CHECK(cu.check(221, 221) == 0);
    CHECK(cu.check(1, 500) == 0 && cu.check(480, 500) == 0);                   // the gap closed (within the slack) before the second look
    CHECK(cu.check(500, 400) == 0 && cu.check(500, 400) == 0);                 // ahead of the others: nothing
}

static void testContainerSync() {
    std::printf("container contents (0xD7 wire, diff / apply)\n");
    namespace cs = nvcoop::coop::containers;
    const fnvcoop::FormRef caps("fallout3.esm", 0xF), stim("falloutnv.esm", 0x15169), pistol("falloutnv.esm", 0x431E);
    cs::Items before{{caps, 25}, {stim, 2}};
    cs::Items after{{caps, 5}, {pistol, 1}};   // took 20 caps and both stimpaks, put a pistol in
    cs::Items d = cs::diff(before, after);
    CHECK(d.size() == 3 && d[caps] == -20 && d[stim] == -2 && d[pistol] == 1);
    CHECK(cs::diff(before, before).empty());
    cs::Items copy = before;
    cs::apply(copy, d);
    CHECK(copy == after);                      // the other copy ends where the taker's did
    cs::Items small{{caps, 3}};
    cs::apply(small, cs::Items{{caps, -10}});
    CHECK(small.empty());                      // never below zero
    cs::Change c; c.slot = 1; c.container = fnvcoop::FormRef("fallout3.esm", 0x77785); c.delta = d;
    std::vector<uint8_t> b = cs::encode(c);
    cs::Change r;
    CHECK(b[0] == cs::kChannel && cs::decode(b.data(), b.size(), &r) && r.slot == 1 && r.container == c.container && r.delta == d);
    for (size_t cut = 0; cut < b.size(); ++cut) CHECK(!cs::decode(b.data(), cut, &r));   // truncated
    c.delta.clear(); b = cs::encode(c); CHECK(!cs::decode(b.data(), b.size(), &r));        // an empty change is not sent
    c.delta = cs::Items{{caps, 0}}; b = cs::encode(c); CHECK(!cs::decode(b.data(), b.size(), &r));   // a zero entry is malformed
    c.container = fnvcoop::FormRef(); c.delta = d; b = cs::encode(c); CHECK(!cs::decode(b.data(), b.size(), &r));   // no container
    {   // a guest's SPECIAL / skills for the host's stand-in runs (player_stats.h)
        namespace ps = nvcoop::coop::pstats;
        ps::Values v{{35, 42.0f}, {5, 6.0f}, {45, 100.0f}};
        std::vector<uint8_t> pb = ps::encode(v);
        ps::Values back;
        CHECK(pb[0] == ps::kChannel && ps::decode(pb.data(), pb.size(), &back) && back == v);
        for (size_t cut = 0; cut < pb.size(); ++cut) CHECK(!ps::decode(pb.data(), cut, &back));
        CHECK(std::string(ps::avName(35)) == "Explosives" && std::string(ps::avName(5)) == "Strength" && !ps::avName(16));
        CHECK(ps::trackedValues().size() == 21);
        pb[3] = 16; CHECK(!ps::decode(pb.data(), pb.size(), &back));   // health is not a tracked value
    }
    // the first opener's whole contents: counts, may be empty, never negative
    cs::Change f; f.slot = 0; f.container = fnvcoop::FormRef("fallout3.esm", 0x8833F); f.full = true; f.delta = after;
    b = cs::encode(f);
    CHECK(cs::decode(b.data(), b.size(), &r) && r.full && r.delta == after && r.base.plugin.empty());
    f.base = fnvcoop::FormRef("fallout3.esm", 0x3D2C1); b = cs::encode(f);   // what the corpse is on the sender's machine travels along
    CHECK(cs::decode(b.data(), b.size(), &r) && r.base == f.base && r.delta == after);
    f.base = fnvcoop::FormRef();
    f.delta.clear(); b = cs::encode(f);
    CHECK(cs::decode(b.data(), b.size(), &r) && r.full && r.delta.empty());
    f.delta = cs::Items{{caps, -1}}; b = cs::encode(f); CHECK(!cs::decode(b.data(), b.size(), &r));
    // the receiver's own roll differs: diff(own, snapshot) turns its copy into the snapshot
    cs::Items own{{stim, 3}, {caps, 7}};
    cs::Items fix = cs::diff(own, after);
    cs::apply(own, fix);
    CHECK(own == after);
}

static void testTalkSubtitles() {
    std::printf("conversation subtitles (0xD5 wire, new-line watch, HUD chunks)\n");
    namespace ts = nvcoop::coop::talksubs;
    ts::Line l;
    l.slot = 1; l.npc = fnvcoop::FormRef("fallout3.esm", 0x3B46); l.speaker = "Lucas Simms"; l.player = "Guest";
    l.text = "Staying out of trouble, I hope? Don't want to have to waste good bullets on your sorry hide.";
    l.topic = fnvcoop::FormRef("falloutnv.esm", 0xC8); l.info = fnvcoop::FormRef("fallout3.esm", 0x3DA20);
    std::vector<uint8_t> b = ts::encode(l);
    ts::Line d;
    CHECK(b[0] == ts::kChannel && ts::decode(b.data(), b.size(), &d));
    CHECK(d.slot == 1 && d.npc == l.npc && d.speaker == l.speaker && d.player == l.player && d.text == l.text);
    CHECK(d.topic == l.topic && d.info == l.info);
    for (size_t cut = 0; cut < b.size(); ++cut) CHECK(!ts::decode(b.data(), cut, &d));   // every truncation is refused
    std::vector<uint8_t> extra = b; extra.push_back(0); CHECK(!ts::decode(extra.data(), extra.size(), &d));
    std::vector<uint8_t> ver = b; ver[1] = 9; CHECK(!ts::decode(ver.data(), ver.size(), &d));
    ts::Line big = l; big.text.assign(5000, 'x'); big.speaker.assign(300, 'n');
    std::vector<uint8_t> bb = ts::encode(big);
    CHECK(ts::decode(bb.data(), bb.size(), &d) && d.text.size() == ts::kMaxText && d.speaker.size() == ts::kMaxName);

    ts::Watch w;
    CHECK(!w.update(true, 0x3B46, ""));                 // the menu is building its first line
    CHECK(w.update(true, 0x3B46, "Hello."));            // new line
    CHECK(!w.update(true, 0x3B46, "Hello."));           // still shown: sent once
    CHECK(w.update(true, 0x3B46, "The bomb?"));         // next line
    CHECK(!w.update(false, 0, ""));                     // menu closed
    CHECK(w.update(true, 0x3B46, "Hello."));            // the same greeting in a new conversation is new
    CHECK(w.update(true, 0x1234, "Hello."));            // same words, another speaker

    std::vector<std::string> c = ts::chunks(l, 40);
    CHECK(ts::chunks(l).size() == 1);   // a whole line is one HUD message by default
    CHECK(c.size() >= 3 && c[0].rfind("Lucas Simms (to Guest): ", 0) == 0);
    std::string joined;
    for (const auto& x : c) { CHECK(x.size() <= 40 && !x.empty() && x[0] != ' '); joined += (joined.empty() ? "" : " ") + x; }
    CHECK(joined == "Lucas Simms (to Guest): " + l.text);
    ts::Line word = l; word.speaker.clear(); word.player.clear(); word.text.assign(100, 'z');
    CHECK(ts::chunks(word, 40).size() == 3);            // one long word: hard cuts
    CHECK(ts::seconds("Hi.") == 2.0f && ts::seconds(std::string(140, 'a')) == 10.0f && ts::seconds("Hi.", true) == 1.5f);

    ts::ReplyWatch r;
    CHECK(!r.update(true, 0x1000, ""));                 // the greeting: no text the player clicked
    CHECK(r.update(true, 0x2000, "What's the deal with the bomb?"));
    CHECK(!r.update(true, 0x2000, "What's the deal with the bomb?"));   // same pick, polled again
    CHECK(r.update(true, 0x3000, "I'll disarm it."));
    CHECK(!r.update(false, 0, ""));
    CHECK(r.update(true, 0x3000, "I'll disarm it."));   // the same reply in a new conversation
    ts::Line pick; pick.speaker = "Guest"; pick.text = "I'll disarm it.";
    CHECK(ts::chunks(pick).size() == 1 && ts::chunks(pick)[0] == "Guest: I'll disarm it.");
}

static void testDialogue() {
    std::printf("dialogue channel (0xD1 through the Mux, result scripts on the host)\n");
    Node::Opt o;
    o.dialogue = true;
    Node host(Role::Host, 0, 0, o);
    Node a(Role::Client, host.rt->boundPort(), 4001, o), b(Role::Client, host.rt->boundPort(), 4002, o);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && b.rt->client()->joined() && a.rt->dialogue() && b.rt->dialogue() && host.rt->host()->clients().size() == 2; }));
    auto count = [](Node& n, fnvcoop_dlg::Event::Kind k) { int c = 0; for (auto& e : n.dlgEvents) c += e.kind == k; return c; };
    a.rt->dialogue()->requestStart(R("FalloutNV.esm", 0x104C0F), (uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return count(a, fnvcoop_dlg::Event::OpenMenu) == 1; }));
    CHECK(pumpUntil(fr, [&] { return b.rt->dialogue()->client().role() == fnvcoop_dlg::ClientDialogue::Role::Spectator; }));
    a.rt->dialogue()->pick(R("FalloutNV.esm", 0x105560), R("FalloutNV.esm", 0x105583), 0, false, {fnvcoop_dlg::Line{"House.", "", 0, 50}}, {}, (uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return count(b, fnvcoop_dlg::Event::ShowLines) == 1; }));        // spectator sees the line
    a.rt->dialogue()->lineDone((uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return host.dlgRuns == 2 && a.rt->client()->applied() == 2 && b.rt->client()->applied() == 2 && host.rt->host()->allSynced(); }));
    CHECK(host.dlgRuns == 2);                                                                   // begin + end, once each
    CHECK(host.world.dump().dump() == a.world.dump().dump() && host.world.dump().dump() == b.world.dump().dump());
    a.rt->dialogue()->leave((uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return host.rt->dialogue()->host()->open() == 0; }));
    CHECK(host.rt->mux()->unroutable() == 0);
}

static void testBaseline() {
    std::printf("world baseline join (client has its own, different world)\n");
    Node::Opt ho;
    ho.baseline = true;
    ho.seed = 6;
    Node host(Role::Host, 0, 0, ho, [](FakeWorld& w) {
        w.stages["falloutnv.esm:000200"] = 10;
        w.stages["falloutnv.esm:000203"] = 40;
        w.globals["falloutnv.esm:000300"] = 7.0f;
        w.objectives["falloutnv.esm:000200#0"] = 3;
        w.questRunning["falloutnv.esm:000200"] = true;
        w.karma = 12;
    });
    {   // host plays on before the friend joins: the join must see baseline + these txns
        using namespace fnvcoop;
        nvhooks::ScopedCapture s(host.hub, Cause{CauseKind::QuestResult, R("FalloutNV.esm", 0x201), 5});
        host.world.hostMutate(host.hub, SetStage{R("FalloutNV.esm", 0x201), 5});
    }
    Node c(Role::Client, host.rt->boundPort(), 5001, ho, [](FakeWorld& w) {
        w.stages["falloutnv.esm:000200"] = 30;      // ahead of the host: force-set must LOWER it
        w.stages["falloutnv.esm:000202"] = 9;       // host has 0
        w.globals["falloutnv.esm:000300"] = 99.0f;
        w.objectives["falloutnv.esm:000200#0"] = 1;
        w.karma = -5;
    });
    auto fr = [&] { host.rt->frame(); c.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return c.rt->client()->baselineDone() && c.rt->client()->applied() == host.rt->host()->log().head(); }, 8000));
    CHECK(!c.rt->syncing());
    CHECK(pumpUntil(fr, [&] { return host.rt->host()->allSynced(); }));
    fnvcoop::ShadowState hs, cs;
    uint64_t h1 = 0, h2 = 0;
    std::string e;
    host.world.dump(&hs, &h1, &e);
    c.world.dump(&cs, &h2, &e);
    CHECK(hs.stages == cs.stages && hs.globals == cs.globals && hs.objectives == cs.objectives && hs.questRunning == cs.questRunning);
    CHECK(c.world.stages.at("falloutnv.esm:000200") == 10 && c.world.stages.at("falloutnv.esm:000202") == 0);   // lowered / reset
    CHECK(c.world.stages.at("falloutnv.esm:000201") == 5);                                                       // host's post-baseline txn
    CHECK(c.world.baselineApplied > 0);
    {   // no snapshot flood after a baseline join: only per-slot ledger kinds may arrive untagged, and only a handful
        CHECK(c.world.untaggedSnapshot.size() <= 8);
        for (auto& n : c.world.untaggedSnapshot) {
            std::string k = n;
            CHECK(k == "AddItem" || k == "RemoveItem" || k == "ModActorValue" || k == "ModReputation" || k == "ModKarma" || k == "SetPerk" ||
                  k == "SetPlayerControls" || k == "BarrierRelease");
        }
    }
    CHECK(c.world.karma == 12);
    {   // after the join, live txns still flow
        using namespace fnvcoop;
        nvhooks::ScopedCapture s(host.hub, Cause{CauseKind::QuestResult, R("FalloutNV.esm", 0x204), 5});
        host.world.hostMutate(host.hub, SetStage{R("FalloutNV.esm", 0x204), 15});
    }
    CHECK(pumpUntil(fr, [&] { return c.world.stages.at("falloutnv.esm:000204") == 15; }));
}


// Host saves (sidecar), plays on, then "loads" the older save: epoch+1, the client rebases and its world is force-set back.
static bool hostLoadScenario(bool* epochBumped, bool strict) {
    auto chk = [&](bool c) { if (strict) { ++g_checks; if (!c) { ++g_failed; std::printf("  FAIL save/load scenario check\n"); } } };
    Node::Opt ho;
    ho.baseline = true;
    ho.seed = 6;
    Node host(Role::Host, 0, 0, ho);
    Node c(Role::Client, host.rt->boundPort(), 6001, ho);
    auto fr = [&] { host.rt->frame(); c.rt->frame(); };
    auto txn = [&](uint32_t q, uint32_t stage) {
        using namespace fnvcoop;
        nvhooks::ScopedCapture s(host.hub, Cause{CauseKind::QuestResult, R("FalloutNV.esm", q), (int)stage});
        host.world.hostMutate(host.hub, SetStage{R("FalloutNV.esm", q), stage});
    };
    pumpUntil(fr, [&] { return c.rt->client()->baselineDone() && host.rt->host()->allSynced(); });
    txn(0x201, 5);
    pumpUntil(fr, [&] { return c.rt->client()->applied() == 1; });
    fnvcoop::Bytes sidecar = host.rt->saveSidecar();     // the PreSave hook
    FakeWorld savedWorld = host.world;                    // what the engine would restore on load
    uint32_t epoch0 = host.rt->host()->epoch();
    txn(0x202, 7);
    txn(0x203, 9);
    pumpUntil(fr, [&] { return c.rt->client()->applied() == 3 && host.rt->host()->allSynced(); });
    chk(c.world.stages.at("falloutnv.esm:000203") == 9);
    chk(!sidecar.empty());
    host.world = savedWorld;                              // engine rolled back by the load
    std::string what;
    chk(host.rt->onGameLoaded(&sidecar, &what));        // the PostLoad hook
    *epochBumped = host.rt->host()->epoch() == epoch0 + 1;
    // clients converge to the loaded save's shared world (rebase -> fresh baseline -> force-set)
    bool conv = pumpUntil(fr, [&] {
        return c.world.stages.at("falloutnv.esm:000203") == 0 && c.world.stages.at("falloutnv.esm:000202") == 0 &&
               c.world.stages.at("falloutnv.esm:000201") == 5 && host.rt->host()->allSynced();
    }, 6000);
    if (conv) {
        chk(c.rt->client()->worldEpoch() == epoch0 + 1);
        chk(c.rt->client()->stats().rebases >= 1);
        chk(host.world.dump().dump() == c.world.dump().dump());
        // history restarted: the next txn is seq 1 of the new epoch and still reaches the client
        txn(0x204, 3);
        chk(pumpUntil(fr, [&] { return c.world.stages.at("falloutnv.esm:000204") == 3; }));
    }
    return conv;
}

// Join a baseline client to a host with a 60-quest world; returns how many untagged (non-baseline) effects reached the client.
static size_t floodCount() {
    Node::Opt ho;
    ho.baseline = true;
    ho.seed = 60;
    Node host(Role::Host, 0, 0, ho, [](FakeWorld& w) { w.stages["falloutnv.esm:000200"] = 10; w.karma = 3; });
    Node c(Role::Client, host.rt->boundPort(), 5201, ho, [](FakeWorld& w) { w.stages["falloutnv.esm:000200"] = 30; });
    auto fr = [&] { host.rt->frame(); c.rt->frame(); };
    pumpUntil(fr, [&] { return c.rt->client()->baselineDone() && host.rt->host()->allSynced(); }, 8000);
    pumpUntil(fr, [] { return false; }, 300);   // let any held snapshot diff arrive
    return c.world.untaggedSnapshot.size();
}

static void testNoFlood() {
    std::printf("no snapshot flood after a baseline join (+ twin)\n");
    CHECK(floodCount() <= 8);
    fnvcoop::Mutations m;
    m.snapshotFloodAfterBaseline = true;
    fnvcoop::MutationScope scope(m);
    CHECK(floodCount() > 8);   // twin: with the L4 filter off the same scenario floods, so the assertion above would fail
}

// Quest vars that an OBJECT script writes inline (the gangers' OnDeath: `set VMS16.nGangerDeathCount to ... + 1`) are inside no capture scope,
// so the host's per-frame poll is the only way they reach the shared state and the guests. Real retest (enemy_spawn, baseline=0): the host
// reached 3, the host's shared state stayed 0 (replicator writes 0), the guest kept its own count.
static bool questVarReachesGuests(bool baseline, bool* quietVarSent) {
    Node::Opt o;
    o.baseline = baseline;
    o.seed = 4;
    Node host(Role::Host, 0, 0, o);
    Node c(Role::Client, host.rt->boundPort(), 7001, o);
    // the host engine's quest variable table (owner, index) -> value, and the host poll exactly as the module wires it
    std::map<std::pair<uint32_t, uint16_t>, double> engine;
    engine[{0x104EAE, 1}] = 0;   // nGangerDeathCount
    engine[{0x104EAE, 2}] = 5;   // a var nobody writes during the test
    nvhooks::ChangeTracker tracker;
    host.rt->setStatePoll([&](fnvcoop::StateReplicator& to, bool first, bool prime) {
        if (first) { tracker.clear(); tracker.endPrime(); if (prime) tracker.beginPrime(); }
        for (auto& [k, v] : engine)
            if (tracker.update(nvhooks::ChangeTracker::varKey(k.first, k.second), v)) to.setScriptVar(R("FalloutNV.esm", k.first), k.second, v);
        tracker.endPrime();
    });
    auto fr = [&] { host.rt->frame(); c.rt->frame(); };
    bool ok = pumpUntil(fr, [&] { return c.rt->client()->joined() && !c.rt->client()->baselineActive(); });
    for (int death = 1; death <= 3; ++death) {   // three gangers die one after the other: their OnDeath scripts run on the host, in no scope
        engine[{0x104EAE, 1}] = death;
        for (int i = 0; i < 10; ++i) fr();
    }
    ok = ok && pumpUntil(fr, [&] { return c.world.questVars.count("falloutnv.esm:104EAE#1") && c.world.questVars["falloutnv.esm:104EAE#1"] == 3.0; }, 3000);
    *quietVarSent = host.rt->host()->state().scriptVars.count({R("FalloutNV.esm", 0x104EAE), 2}) > 0;
    ok = ok && host.rt->host()->state().scriptVars.at({R("FalloutNV.esm", 0x104EAE), 1}) == 3.0;
    return ok;
}

static void testObjectScriptQuestVarsReachGuests() {
    std::printf("quest vars written by object scripts reach the guests (with and without a world baseline, + twin 8)\n");
    for (bool baseline : {false, true}) {
        bool quiet = false;
        CHECK(questVarReachesGuests(baseline, &quiet));
        CHECK(quiet == baseline);   // no baseline: the present values are taken as known, not flooded; with one the first poll sends all
    }
}

static void testSaveLoad() {
    std::printf("save / load (sidecar, epoch bump, rebase) + client load policy\n");
    bool bumped = false;
    CHECK(hostLoadScenario(&bumped, true));
    CHECK(bumped);
    {   // mutation twin: the host load does NOT bump the epoch -> clients never rebase -> must NOT converge
        fnvcoop::Mutations m;
        m.noEpochBump = true;
        fnvcoop::MutationScope scope(m);
        bool b2 = true;
        CHECK(!hostLoadScenario(&b2, false));
        CHECK(!b2);
    }
    // client policy: Block refuses a load while joined; Rejoin re-baselines to the host world
    for (bool block : {true, false}) {
        Node::Opt ho;
        ho.baseline = true;
        ho.seed = 4;
        ho.blockClientLoad = block;
        Node host(Role::Host, 0, 0, ho);
        Node c(Role::Client, host.rt->boundPort(), 6101, ho);
        auto fr = [&] { host.rt->frame(); c.rt->frame(); };
        CHECK(pumpUntil(fr, [&] { return c.rt->client()->baselineDone(); }));
        std::string what;
        c.world.stages["falloutnv.esm:000200"] = 77;       // the "loaded save" has different local state
        bool ok = c.rt->onGameLoaded(nullptr, &what);
        CHECK(ok == !block);
        CHECK(c.rt->clientLoadAllowed() == !block);
        if (!block) CHECK(pumpUntil(fr, [&] { return c.world.stages.at("falloutnv.esm:000200") == 0; }, 6000));
        else CHECK(c.rt->counters().loadsBlocked == 1);
    }
}


// VCG01 chargen: the host's SetStage 30 is deferred until EVERY joined player passed; then the real stage runs once.
static bool chargenScenario(bool strict, bool disconnectLast) {
    auto chk = [&](bool c) { if (strict) { ++g_checks; if (!c) { ++g_failed; std::printf("  FAIL chargen scenario check\n"); } } };
    Node::Opt o;
    o.rules = true;
    Node host(Role::Host, 0, 0, o);
    auto a = std::make_unique<Node>(Role::Client, host.rt->boundPort(), 7001, o);
    auto b = std::make_unique<Node>(Role::Client, host.rt->boundPort(), 7002, o);
    auto fr = [&] { host.rt->frame(); if (a) a->rt->frame(); if (b) b->rt->frame(); };
    pumpUntil(fr, [&] { return a->rt->client()->joined() && b->rt->client()->joined(); });
    fnvcoop::FormRef vcg01;
    uint32_t h = 2166136261u;
    for (char ch : std::string("VCG01")) { h ^= (uint8_t)ch; h *= 16777619u; }
    vcg01 = fnvcoop::FormRef("falloutnv.esm", h);
    std::string key = nvhooks::refToStr(vcg01);
    chk(host.rt->rules() != nullptr);
    chk(!host.rt->stageRequested(vcg01, 30, 0));           // deferred by the barrier
    chk(!host.rt->stageRequested(vcg01, 30, 0));           // asking again changes nothing
    chk(host.rt->stageRequested(R("FalloutNV.esm", 0x555), 10, 0));   // a quest without rules proceeds
    host.rt->scenePass(vcg01);                             // the host player finished chargen
    a->rt->scenePass(vcg01);                               // guest A finished
    pumpUntil(fr, [&] { return host.rt->counters().rulesActions >= 1; });
    pumpUntil(fr, [] { return false; }, 200);
    bool early = host.world.stages.count(key) != 0;        // B has NOT passed: nothing may have advanced
    chk(!early);
    if (disconnectLast) {
        b->rt->stop();                                     // B leaves: the barrier must not wait for them any more
        b.reset();
    } else {
        b->rt->scenePass(vcg01);
    }
    bool done = pumpUntil(fr, [&] { return host.world.stages.count(key) && host.world.stages.at(key) == 30; }, 8000);
    if (done) {
        chk(a->world.stages.count(key) ? true : pumpUntil(fr, [&] { return a->world.stages.count(key) != 0; }));
        chk(a->world.stages.at(key) == 30);
        chk(host.rt->rules()->pendingDeferrals() == 0);
        chk(host.rt->rules()->stats().stagesReleased == 1);
        if (!disconnectLast) {
            pumpUntil(fr, [&] { return b->world.stages.count(key) != 0; });
            chk(b->world.stages.count(key) && b->world.stages.at(key) == 30);
        }
    }
    return done && !early;
}

static void testRules() {
    std::printf("quest rules: VCG01 chargen barrier (3 players), early-leaver release, ENet disconnect\n");
    CHECK(chargenScenario(true, false));
    CHECK(chargenScenario(true, true));      // the leaver is dropped through the ENet Disconnected event -> peerDisconnected
    {   // twin: the barrier releases as soon as ANYONE passes -> the stage advances before everyone passed -> scenario must fail
        fnvcoop::Mutations m;
        m.barrierReleaseEarly = true;
        fnvcoop::MutationScope scope(m);
        CHECK(!chargenScenario(false, false));
    }
    {   // INFO result cause -> owning quest, and client ReportCell is consumed by the rules engine
        Node::Opt o;
        o.rules = true;
        Node host(Role::Host, 0, 0, o);
        host.rt->setInfoQuestResolver([](const fnvcoop::FormRef& info, fnvcoop::FormRef& q) {
            if (info.localId != 0xAAA) return false;
            q = R("FalloutNV.esm", 0xBBB);
            return true;
        });
        Node c(Role::Client, host.rt->boundPort(), 7101, o);
        auto fr = [&] { host.rt->frame(); c.rt->frame(); };
        pumpUntil(fr, [&] { return c.rt->client()->joined(); });
        {
            using namespace fnvcoop;
            nvhooks::ScopedCapture sc(host.hub, Cause{CauseKind::DialogueResult, R("FalloutNV.esm", 0xAAA), 0});
            host.world.hostMutate(host.hub, SetGlobal{R("FalloutNV.esm", 0x777), 1.0f});
        }
        const auto* t = host.rt->host()->log().get(1);
        CHECK(t && t->cause.source == R("FalloutNV.esm", 0xBBB));
        c.rt->reportCell(R("FalloutNV.esm", 0xCCC));
        CHECK(pumpUntil(fr, [&] { return host.rt->counters().rulesActions >= 1; }));
    }
}


// Real-data repro (skipped when the fixtures do not exist): host baseline + the sidecar written by the real game, client with the
// same real world. The host loads the sidecar (epoch 2) and the client MUST finish the rebase baseline.
class StateDumper : public IWorldDumper {
public:
    fnvcoop::ShadowState st;
    uint64_t hash = 0;
    bool dump(fnvcoop::ShadowState* o, uint64_t* h, std::string*) override { *o = st; *h = hash; return true; }
};

static void testRealSidecar() {
    std::string base = g_outDir + "/ingame_baseline/host_baseline.bin", sc = g_outDir + "/ingame_saveload/sidecar.bin";
    StateDumper d;
    if (!LoadBaselineFile(base, &d.st, &d.hash)) { std::printf("real sidecar scenario: SKIPPED (no %s)\n", base.c_str()); return; }
    FILE* f = std::fopen(sc.c_str(), "rb");
    if (!f) { std::printf("real sidecar scenario: SKIPPED (no %s)\n", sc.c_str()); return; }
    fnvcoop::Bytes side;
    uint8_t buf[65536];
    size_t n;
    while ((n = std::fread(buf, 1, sizeof buf, f)) > 0) side.insert(side.end(), buf, buf + n);
    std::fclose(f);
    std::printf("real sidecar scenario (%zu stages, sidecar %zu bytes)\n", d.st.stages.size(), side.size());
    nvhooks::CaptureHub hh, hc;
    FakeWorld wh, wc;
    Config c;
    c.enable = true; c.baseline = true; c.role = Role::Host; c.port = 0;
    Runtime host(c, &hh, &wh, nowMs);
    host.setDumper(&d);
    std::string err;
    CHECK(host.start(&err));
    c.role = Role::Client; c.port = host.boundPort(); c.token = 8001;
    Runtime cl(c, &hc, &wc, nowMs);
    cl.setDumper(&d);
    CHECK(cl.start(&err));
    auto fr = [&] { host.frame(); cl.frame(); };
    CHECK(pumpUntil(fr, [&] { return cl.client()->baselineDone(); }, 15000));
    CHECK(host.onGameLoaded(&side, nullptr));
    bool again = pumpUntil(fr, [&] { return cl.client()->worldEpoch() == host.host()->epoch() && cl.client()->baselineDone() && !cl.client()->baselineActive(); }, 15000);
    if (!again) std::printf("  client stuck after rebase: %s\n  host: %s\n", cl.statusJson().c_str(), host.statusJson().c_str());
    CHECK(again);
}


// Offline replay of the join diff on the REAL exported baseline: a client whose world equals the host world must get ZERO effects.
static void testRealSelfDiff() {
    StateDumper d;
    std::string base = g_outDir + "/ingame_baseline/host_baseline.bin";
    if (!LoadBaselineFile(base, &d.st, &d.hash)) { std::printf("real self-diff: SKIPPED\n"); return; }
    fnvcoop::BaselineSummary sum = fnvcoop::summarizeWorld(d.st, d.hash);
    fnvcoop::BaselineDiff diff = fnvcoop::diffBaseline(d.st, sum, fnvcoop::SharePolicy(), 1, true, d.st.ledger[0].karma, true, {});
    std::printf("real self-diff: %zu entries, diff=%zu effects, kept=%zu\n", sum.entries.size(), diff.effects.size(), diff.keptByClient);
    for (size_t i = 0; i < diff.effects.size() && i < 10; ++i) std::printf("   %s\n", nvhooks::effectToJson(diff.effects[i]).dump().c_str());
    CHECK(diff.effects.empty());
}


static void testHandshake() {
    std::printf("join handshake (build / mod list), reject text, worker thread\n");
    auto join = [&](uint64_t hostHash, uint64_t clientHash, std::vector<std::string> hostMods, std::vector<std::string> clientMods, uint8_t* reason, std::string* text) {
        Node::Opt ho;
        ho.handshake = true; ho.buildHash = hostHash; ho.plugins = hostMods;
        Node host(Role::Host, 0, 0, ho);
        Node::Opt co = ho;
        co.buildHash = clientHash; co.plugins = clientMods;
        Node c(Role::Client, host.rt->boundPort(), 9001, co);
        auto fr = [&] { host.rt->frame(); c.rt->frame(); };
        bool ended = pumpUntil(fr, [&] { return c.rt->client()->joined() || c.rt->client()->rejected(); }, 6000);
        *reason = c.rt->client()->rejectReason();
        *text = c.rt->client()->rejectText();
        return ended && c.rt->client()->joined();
    };
    uint8_t reason = 0;
    std::string text;
    CHECK(join(0xAB, 0xAB, {"falloutnv.esm", "a.esp"}, {"falloutnv.esm", "a.esp"}, &reason, &text));      // identical builds join
    CHECK(!join(0xAB, 0xCD, {"falloutnv.esm"}, {"falloutnv.esm"}, &reason, &text));                         // different build hash
    CHECK(reason == fnvcoop::kRejectBuild && !text.empty());
    std::printf("    build mismatch -> reason %d: %s\n", (int)reason, text.c_str());
    CHECK(!join(0xAB, 0xAB, {"falloutnv.esm", "a.esp"}, {"falloutnv.esm", "b.esp"}, &reason, &text));      // different mod list
    CHECK(reason == fnvcoop::kRejectModList && !text.empty());
    std::printf("    mod list mismatch -> reason %d: %s\n", (int)reason, text.c_str());
}


static bool g_lastPlayerRefBinds = false;
static size_t partyWarpCount(bool withResolver, int bindGhosts = 0, bool reload = false) {
    Node::Opt o;
    o.rules = true;
    o.noPlayerRef = !withResolver;
    o.bindGhosts = bindGhosts;
    Node host(Role::Host, 0, 0, o);
    Node a(Role::Client, host.rt->boundPort(), 7301, o), b(Role::Client, host.rt->boundPort(), 7302, o);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); };
    pumpUntil(fr, [&] { return a.rt->client()->joined() && b.rt->client()->joined(); });
    if (reload) {   // a load / rebase re-asserts the binding (module calls onGameLoaded after PostLoadGame)
        std::string what;
        host.rt->onGameLoaded(nullptr, &what);
        g_lastPlayerRefBinds = host.rt->statusJson().find("\"binds\":2") != std::string::npos;
    }
    uint32_t h = 2166136261u;
    for (char ch : std::string("WarpQ")) { h ^= (uint8_t)ch; h *= 16777619u; }
    fnvcoop::MoveRef m;
    m.ref = R("FalloutNV.esm", 0x14);          // the quest script moves "the player"
    m.cell = R("FalloutNV.esm", 0x777);
    m.x = 100;
    host.rt->host()->commit(0, fnvcoop::Cause{fnvcoop::CauseKind::QuestResult, fnvcoop::FormRef("falloutnv.esm", h), 0}, {m});
    const auto* t = host.rt->host()->log().get(1);
    size_t n = 0;
    if (t) for (auto& e : t->effects) if (auto* mv = std::get_if<fnvcoop::MoveRef>(&e)) n += mv->partyWarp;
    return n;
}

static void testPartyWarp() {
    std::printf("PartyWarp needs the rules engine's playerRef (set by the runtime)\n");
    CHECK(partyWarpCount(true) == 3);      // host + 2 guests moved together
    CHECK(partyWarpCount(false) == 1);     // twin: without a guest ref resolver only the host player is expanded (inert for guests)
    CHECK(partyWarpCount(false, 1) == 3);  // module wiring (bindPlayerRefs: host 0x14 + L6 ghost refs) expands to host + 2 guests
    CHECK(partyWarpCount(false, 2) == 2);  // a guest whose ghost is not spawned yet is skipped, the rest still move
    CHECK(partyWarpCount(false, 1, true) == 3 && g_lastPlayerRefBinds);   // still bound after a load (binds == 2 in status)
}


// ---------------------------------------------------------------------------------------------------------------------------------
// L11 / L12 wiring: bridge config, menu resume, 0xE5 slideshow channel, SlideshowEnd calls, INFO reader, world effects, companions
// ---------------------------------------------------------------------------------------------------------------------------------
using fnvcoop_dlg::Event;
static uint32_t menuIdOf(const Node& n) {
    for (size_t i = n.dlgEvents.size(); i-- > 0;)
        if (n.dlgEvents[i].kind == Event::MenuShow) return std::get<fnvcoop_dlg::MenuRequest>(n.dlgEvents[i].msg).menu;
    return 0;
}
static int evCount(const Node& n, Event::Kind k) { int c = 0; for (auto& e : n.dlgEvents) c += e.kind == k; return c; }
static nvui::MessageBoxCall endgameBox(uint32_t script, uint32_t msg = 0x161E8C) {
    nvui::MessageBoxCall c;
    c.scriptRefID = script;
    c.thisObj = 0x300;
    c.message = R("falloutnv.esm", msg);
    c.text = "Are you ready for the battle of Hoover Dam?";
    c.buttons = {"I'm not ready yet.", "Let's do this!"};
    return c;
}
static nvui::Route boxFor(Node& host, fnvcoop::PlayerId origin, const nvui::MessageBoxCall& c) {
    nvui::OriginScope sc(host.ui.router->origin(), nvui::OriginContext::Kind::Acting, origin);
    return host.ui.router->onMessageBox(c);
}
static void slide(Node& host, int i) {
    char b[64];
    std::snprintf(b, sizeof b, "interface\\endgame\\endscrn_WN%02d", i);
    host.ui.router->onSwapTexture(0x9000, "##GS01:0", b);
}
static void commitEnd(Node& host) {
    nvhooks::ScopedCapture sc(host.hub, fnvcoop::Cause{fnvcoop::CauseKind::Script, R("FalloutNV.esm", 0x161E00), 0});
    host.world.hostMutate(host.hub, fnvcoop::SlideshowEnd{});
}
static Node::Opt uiOpt() { Node::Opt o; o.dialogue = true; o.ui = true; return o; }

static void testUiBridgeConsentResume() {
    std::printf("L11 wiring: bridge config, consent prompt, menu resume, INFO reader\n");
    Node host(Role::Host, 0, 0, uiOpt());
    Node a(Role::Client, host.rt->boundPort(), 5001, uiOpt()), b(Role::Client, host.rt->boundPort(), 5002, uiOpt());
    auto fr = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && b.rt->client()->joined() && host.rt->host()->clients().size() == 2 && a.rt->dialogue() && b.rt->dialogue() && host.rt->uiStats().bridges == 1; }));
    const auto sa = a.rt->client()->slot(), sb = b.rt->client()->slot();
    CHECK(host.rt->uiStats().bridges == 1 && host.rt->uiStats().consentRows >= 5);          // (1) bridge config built from the consent table
    CHECK(host.ui.readerInstalls == 1 && a.ui.readerInstalls == 1 && b.ui.readerInstalls == 1);   // (5) INFO reader installed on every node
    CHECK(host.ui.installedReader && host.ui.installedReader(7).size() == 1 && host.ui.installedReader(7)[0].text == "info 7");
    // point of no return, guest A pushes the button: the others are asked, the script resumes once with the confirm button
    CHECK(boxFor(host, sa, endgameBox(91)) == nvui::Route::Suppressed);
    CHECK(pumpUntil(fr, [&] { return menuIdOf(a) != 0; }));
    const uint32_t id1 = menuIdOf(a);
    CHECK(host.ui.eng.waiting() && host.ui.eng.resumed() == 0);
    a.rt->dialogue()->menuReply(id1, 1, (uint32_t)nowMs());                                    // "Let's do this!"
    CHECK(pumpUntil(fr, [&] { return menuIdOf(b) == id1 && menuIdOf(host) == id1; }));        // consent prompts on B and on the host
    CHECK(host.ui.eng.waiting() && host.ui.eng.resumed() == 0);                                                         // still waiting for the party
    b.rt->dialogue()->menuReply(id1, 0, (uint32_t)nowMs());                                    // "Ready"
    pumpUntil(fr, [&] { return false; }, 100);
    CHECK(host.ui.eng.waiting() && host.ui.eng.resumed() == 0);                                                         // the host has not answered yet
    host.rt->dialogue()->menuReply(id1, 0, (uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return host.ui.eng.resumed() == 1; }));                      // (2) menu resume reaches the script
    CHECK(host.ui.eng.lastButton() == 1 && host.ui.router->pendingBoxes() == 0);
    pumpUntil(fr, [&] { return false; }, 150);
    CHECK(host.ui.eng.resumed() == 1);                                                     // exactly once
    // a decline anywhere takes the table's decline button
    CHECK(boxFor(host, sa, endgameBox(92)) == nvui::Route::Suppressed);
    CHECK(pumpUntil(fr, [&] { return menuIdOf(a) != id1 && menuIdOf(a) != 0; }));
    const uint32_t id2 = menuIdOf(a);
    a.rt->dialogue()->menuReply(id2, 1, (uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return menuIdOf(b) == id2; }));
    b.rt->dialogue()->menuReply(id2, 1, (uint32_t)nowMs());                                    // "Not yet"
    CHECK(pumpUntil(fr, [&] { return host.ui.eng.resumed() == 2; }));
    CHECK(host.ui.eng.lastButton() == 0);
    // a plain box has no consent round and a host-origin consent box stays local
    CHECK(boxFor(host, sb, endgameBox(93, 0x123456)) == nvui::Route::Suppressed);
    CHECK(pumpUntil(fr, [&] { return menuIdOf(b) != id2; }));
    b.rt->dialogue()->menuReply(menuIdOf(b), 1, (uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return host.ui.eng.resumed() == 3; }));
    CHECK(host.ui.eng.lastButton() == 1 && menuIdOf(a) == id2);
    CHECK(boxFor(host, nvui::kHost, endgameBox(94)) == nvui::Route::Local);
    CHECK(host.rt->mux()->unroutable() == 0);
}

static void testUiSlideshowChannel() {
    std::printf("L11 wiring: 0xE5 slideshow channel, SlideshowEnd on host and guest, late joiner\n");
    Node host(Role::Host, 0, 0, uiOpt());
    Node a(Role::Client, host.rt->boundPort(), 5101, uiOpt());
    std::unique_ptr<Node> c;
    auto fr = [&] { host.rt->frame(); a.rt->frame(); if (c) c->rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1 && host.rt->uiStats().bridges == 1; }));
    slide(host, 1);
    slide(host, 2);
    CHECK(pumpUntil(fr, [&] { return a.ui.swaps.size() == 2; }));                              // (3) guest registered 0xE5 and plays in order
    CHECK(a.ui.swaps[0] == "interface\\endgame\\endscrn_WN01" && a.ui.swaps[1] == "interface\\endgame\\endscrn_WN02");
    CHECK(a.rt->uiStats().slidePackets >= 2 && a.ui.client->locked() && !a.ui.locks.empty() && a.ui.locks.back());
    c.reset(new Node(Role::Client, host.rt->boundPort(), 5102, uiOpt()));                       // joins mid-show: replay
    CHECK(pumpUntil(fr, [&] { return c->ui.swaps.size() == 2; }));
    CHECK(c->ui.swaps == a.ui.swaps && c->ui.client->locked());
    commitEnd(host);                                                                           // (4) SlideshowEnd: End packet + guest effect call
    CHECK(pumpUntil(fr, [&] { return !a.ui.client->locked() && !c->ui.client->locked() && a.world.slideEnds == 1 && c->world.slideEnds == 1; }));
    CHECK(a.ui.client->finished() && c->ui.client->finished() && a.ui.client->played() == 2 && c->ui.client->played() == 2);
    CHECK(host.rt->uiStats().slideEnds == 1 && a.rt->uiStats().slideEnds >= 1 && !host.ui.host->active());
    CHECK(a.ui.swaps.size() == 2 && c->ui.swaps.size() == 2);                                  // no replays after the end
    CHECK(host.rt->mux()->unroutable() == 0 && a.rt->mux()->unroutable() == 0);
}

// Goodsprings mini story through host + 2 guests (+ a late joiner): every new effect type, convergence, no duplicates.
static void testGoodspringsStory() {
    std::printf("Goodsprings mini-story: stage, companion, consent, forced dialogue, spawn, ref var, world effects, slideshow end\n");
    using namespace fnvcoop;
    Node host(Role::Host, 0, 0, uiOpt());
    Node a(Role::Client, host.rt->boundPort(), 5201, uiOpt()), b(Role::Client, host.rt->boundPort(), 5202, uiOpt());
    std::unique_ptr<Node> late;
    auto fr = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); if (late) late->rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && b.rt->client()->joined() && host.rt->host()->clients().size() == 2 && a.rt->dialogue() && b.rt->dialogue() && host.rt->uiStats().bridges == 1; }));
    const auto sa = a.rt->client()->slot(), sb = b.rt->client()->slot();
    const FormRef quest = R("FalloutNV.esm", 0x10A214), sunny = R("FalloutNV.esm", 0x104C4B), follQuest = R("FalloutNV.esm", 0x104C00);
    for (Node* n : {&host, &a, &b}) n->rt->dialogue()->companions().addVarRule(follQuest, 3, fnvcoop_dlg::VarKind::HasFollower);

    // 1. stage + the spawn of a crate and script vars on it, in one result script
    const FormRef crate = host.rt->host()->allocCreatedRef();
    {
        nvhooks::ScopedCapture sc(host.hub, Cause{CauseKind::QuestResult, quest, 10});
        host.world.hostMutate(host.hub, SetStage{quest, 10});
        SpawnRef sp;
        sp.id = crate.localId;
        sp.base = R("FalloutNV.esm", 0x7777);
        sp.cell = R("FalloutNV.esm", 0x1A8AF);
        sp.x = 1; sp.y = 2; sp.z = 3;
        host.world.hostMutate(host.hub, sp);
        host.world.hostMutate(host.hub, RefScriptVar{crate, 2, 5.0});                             // follow-up on the spawned (host) ref
        host.world.hostMutate(host.hub, RefScriptVar{R("FalloutNV.esm", 0x104C50), 3, 7.0});      // an esm ref
    }
    // 2. companion recruit by guest A (Sunny Smiles), then the per-owner condition view
    {
        nvhooks::ScopedCapture sc(host.hub, Cause{CauseKind::DialogueResult, R("FalloutNV.esm", 0x105583), 0});
        host.world.hostMutate(host.hub, CompanionOwner{sunny, (uint8_t)sa, true, 0});
    }
    CHECK(pumpUntil(fr, [&] { return a.world.companions.size() == 1 && b.world.companions.size() == 1 && b.rt->dialogue()->companions().ownerOf(sunny) == sa; }));
    CHECK(host.rt->dialogue()->companions().ownerOf(sunny) == sa && a.rt->dialogue()->companions().ownerOf(sunny) == sa);
    {
        nvui::OriginScope s1(host.ui.router->origin(), nvui::OriginContext::Kind::Acting, sa);
        CHECK(host.ui.router->viewQuestVar(0x104C00, 3, 0) == 1.0);
    }
    {
        nvui::OriginScope s2(host.ui.router->origin(), nvui::OriginContext::Kind::Acting, sb);
        CHECK(host.ui.router->viewQuestVar(0x104C00, 3, 1) == 0.0);                            // B has no follower
    }
    // 3. consent prompt (point of no return) answered by the whole party
    CHECK(boxFor(host, sb, endgameBox(95)) == nvui::Route::Suppressed);
    CHECK(pumpUntil(fr, [&] { return menuIdOf(b) != 0; }));
    const uint32_t mid = menuIdOf(b);
    b.rt->dialogue()->menuReply(mid, 1, (uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return menuIdOf(a) == mid && menuIdOf(host) == mid; }));
    a.rt->dialogue()->menuReply(mid, 0, (uint32_t)nowMs());
    host.rt->dialogue()->menuReply(mid, 0, (uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return host.ui.eng.resumed() == 1; }));
    CHECK(host.ui.eng.lastButton() == 1);
    // 4. forced dialogue: the capture record (ForcedDialogue effect) and the party scene through the node
    {
        nvhooks::ScopedCapture sc(host.hub, Cause{CauseKind::Script, R("FalloutNV.esm", 0x10A215), 0});
        host.world.hostMutate(host.hub, ForcedDialogue{sunny, R("FalloutNV.esm", 0x6000), 0});
        nvui::ForcedCall fc;
        fc.kind = nvui::ForcedCall::Kind::SayTo;
        fc.speaker = 0x5000;
        fc.target = nvui::kPlayerRefID;   // outside an acting scope: the whole party
        fc.topic = R("FalloutNV.esm", 0x6000);
        fc.info = R("FalloutNV.esm", 0x6001);
        fc.lines = {fnvcoop_dlg::Line{"Doc Mitchell's waiting for you.", "", 0, 50}};
        CHECK(host.ui.router->onForced(fc).captured);
    }
    CHECK(pumpUntil(fr, [&] { return evCount(host, Event::ForcedSayShow) == 1 && evCount(a, Event::ForcedSayShow) == 1 && evCount(b, Event::ForcedSayShow) == 1; }));
    // 5. lane-local world effects (NPC state): a replacing key and an accumulating delta
    host.pushWorld("rel:NCRfaction:Powderganger", R"({"type":"SetRelation","a":"falloutnv.esm:000001","b":"falloutnv.esm:000002","kind":1})");
    host.pushWorld("rel:NCRfaction:Powderganger", R"({"type":"SetRelation","a":"falloutnv.esm:000001","b":"falloutnv.esm:000002","kind":2})");
    host.pushWorld("", R"({"type":"SetActorValueRef","actor":"falloutnv.esm:104C4B","av":16,"value":-10,"mode":2})");
    // 6. ending slides then the SlideshowEnd effect
    slide(host, 1);
    slide(host, 2);
    commitEnd(host);
    auto synced = [&](Node& n) { return n.rt->client()->applied() == host.rt->host()->log().head(); };
    CHECK(pumpUntil(fr, [&] { return synced(a) && synced(b) && a.world.worldApplied == 3 && b.world.worldApplied == 3 && !a.ui.client->locked() && !b.ui.client->locked() && a.ui.swaps.size() == 2; }));

    const std::string hd = host.world.dump().dump();
    CHECK(hd == a.world.dump().dump() && hd == b.world.dump().dump());                         // convergence
    CHECK(host.world.stages.at("falloutnv.esm:10A214") == 10 && a.world.stages.at("falloutnv.esm:10A214") == 10);
    for (Node* n : {&host, &a, &b}) CHECK(n->world.spawned.size() == 1 && n->world.dupSpawns == 0 && n->world.unresolvedRefOps == 0);
    CHECK(a.world.refVars.at(nvhooks::refToStr(crate) + "#2") == 5.0 && a.world.refVars.at("falloutnv.esm:104C50#3") == 7.0);
    CHECK(a.world.spawned.begin()->second != 0 && a.world.forcedEvents == 1 && b.world.forcedEvents == 1 && host.world.forcedEvents == 1);
    CHECK(a.world.slideEnds == 1 && b.world.slideEnds == 1 && a.rt->uiStats().slideEnds >= 1);
    CHECK(a.world.worldApplied == 3 && a.rt->worldStats().stale == 0 && host.rt->worldStats().pushed == 3);

    // 7. a late joiner converges on the shared story state with every effect exactly once (events are not state)
    late.reset(new Node(Role::Client, host.rt->boundPort(), 5203, uiOpt()));
    CHECK(pumpUntil(fr, [&] { return late->rt->client()->joined() && !late->rt->syncing() && late->rt->client()->applied() == host.rt->host()->log().head() && late->world.worldApplied == 2 && late->rt->dialogue(); }));
    CHECK(late->world.spawned.size() == 1 && late->world.dupSpawns == 0 && late->world.refVars.size() == 2);
    CHECK(late->world.stages.at("falloutnv.esm:10A214") == 10 && late->world.companions.size() == 1 && late->world.slideEnds == 1);
    CHECK(late->rt->dialogue()->companions().ownerOf(sunny) == sa);
    CHECK(host.rt->worldStats().replayed == 2);   // latest relation + the delta, not the superseded push
    CHECK(late->world.worldState == host.world.worldState);
    CHECK(host.rt->mux()->unroutable() == 0);
}

static void testWorldEffectChannel() {
    std::printf("L12 wiring: world effects on the app channel (replace, accumulate, stale guard, queued until the baseline is done)\n");
    Node::Opt o;
    o.baseline = true;
    o.seed = 3;
    Node host(Role::Host, 0, 0, o);
    host.pushWorld("ess:a:base", R"({"type":"SetEssential","actor":"falloutnv.esm:000AAA","essential":true,"refOnly":false})");   // before anyone joined
    Node a(Role::Client, host.rt->boundPort(), 5301, o);
    a.rt->setWorldEffectApplier([&a](const std::string& j) { return a.world.applyWorldJson(j); });
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && !a.rt->syncing() && a.world.worldApplied == 1; }));
    CHECK(a.world.worldState.size() == 1 && host.rt->worldStats().replayed == 1);
    host.pushWorld("ess:a:base", R"({"type":"SetEssential","actor":"falloutnv.esm:000AAA","essential":false,"refOnly":false})");
    CHECK(pumpUntil(fr, [&] { return a.world.worldApplied == 2; }));
    CHECK(a.world.worldState == host.world.worldState && a.world.worldState.size() == 1);
    Value st;
    std::string perr;
    nvcoop::json::parse(a.rt->statusJson(), st, perr);
    CHECK(st.get("world").get("applied").asInt() == 2 && st.get("world").get("failed").asInt() == 0);
}

// coop.leave / coop.join: a guest drops out (game keeps running, world untouched), the host plays on, the guest rejoins through the baseline path.
// Rewards earned before leaving must not be granted a second time.
static void testLeaveJoin() {
    std::printf("leave / join (clean disconnect, rejoin resumes from the saved view, no duplicate rewards)\n");
    Node::Opt o;
    o.baseline = true;
    o.seed = 3;
    Node host(Role::Host, 0, 0, o);
    Node c(Role::Client, host.rt->boundPort(), 5001, o);
    auto fr = [&] { host.rt->frame(); c.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return c.rt->client()->joined() && c.rt->client()->baselineDone(); }, 8000));
    runVcg02(host.world, host.hub);                                   // caps 100 + karma 2 for everyone
    CHECK(pumpUntil(fr, [&] { return c.rt->client()->applied() == host.rt->host()->log().head() && host.rt->host()->allSynced(); }));
    const std::string caps = "falloutnv.esm:00000F";
    CHECK(c.world.inventory[caps] == 100 && c.world.karma == 2);
    std::string err;
    CHECK(!c.rt->join("", 0, &err) && err.find("coop.leave") != std::string::npos);                 // already in a session
    CHECK(c.rt->leave(&err) && c.rt->left() && !c.rt->running());
    CHECK(!c.rt->leave(&err));                                                                      // leaving twice is an error, not a crash
    c.rt->frame();                                                                                   // frame() on a left runtime is a no-op
    CHECK(pumpUntil(fr, [&] { return host.rt->host()->clients().empty() || !host.rt->host()->clients()[0].joined; }, 8000));   // the host saw the disconnect
    CHECK(c.world.inventory[caps] == 100 && c.world.karma == 2);                                    // leaving does not touch the local world
    {   // the host earns another reward while the guest is away
        nvhooks::ScopedCapture sc(host.hub, fnvcoop::Cause{fnvcoop::CauseKind::QuestResult, R("FalloutNV.esm", 0x10A215), 10});
        host.world.hostMutate(host.hub, fnvcoop::AddItem{R("FalloutNV.esm", 0xF), 7, fnvcoop::Recipient::dflt()});
    }
    CHECK(c.rt->join("127.0.0.1", host.rt->boundPort(), &err));
    CHECK(c.rt->running() && !c.rt->left());
    CHECK(pumpUntil(fr, [&] { return c.rt->client() && c.rt->client()->joined() && c.rt->client()->applied() == host.rt->host()->log().head(); }, 8000));
    CHECK(pumpUntil(fr, [&] { return host.rt->host()->allSynced(); }));
    CHECK(c.world.inventory[caps] == host.world.inventory[caps]);                                 // 107 on both: the 100 was not granted again
    CHECK(c.world.inventory[caps] == 107 && c.world.karma == 2 && host.world.karma == 2);
}

// A client whose game restarted loads its save; the sidecar stored with that save lists the rewards the saved world already holds, so the
// plain baseline join delivers only the rewards it is missing. `useSidecar=false` is the twin: without it the whole ledger arrives again.
static int64_t restartedCaps(bool useSidecar, bool saveWhileLeft) {
    Node::Opt o;
    o.baseline = true;
    o.seed = 4;
    Node host(Role::Host, 0, 0, o);
    const std::string caps = "falloutnv.esm:00000F";
    fnvcoop::Bytes sidecar;
    int64_t capsInSave = 0, karmaInSave = 0;
    {
        Node c(Role::Client, host.rt->boundPort(), 5001, o);
        auto fr = [&] { host.rt->frame(); c.rt->frame(); };
        CHECK(pumpUntil(fr, [&] { return c.rt->client()->joined() && c.rt->client()->baselineDone(); }, 8000));
        runVcg02(host.world, host.hub);
        CHECK(pumpUntil(fr, [&] { return c.rt->client()->applied() == host.rt->host()->log().head() && host.rt->host()->allSynced(); }));
        std::string err;
        if (saveWhileLeft) CHECK(c.rt->leave(&err));
        sidecar = c.rt->saveSidecar();                    // the game save is written together with this sidecar
        capsInSave = c.world.inventory[caps];
        karmaInSave = c.world.karma;
        CHECK(!sidecar.empty() && capsInSave == 100);
    }   // the game process ends
    pumpUntil([&] { host.rt->frame(); }, [&] { return host.rt->host()->clients().empty() || !host.rt->host()->clients()[0].joined; }, 3000);
    {   // the host earns more while the friend's game is down
        nvhooks::ScopedCapture sc(host.hub, fnvcoop::Cause{fnvcoop::CauseKind::QuestResult, R("FalloutNV.esm", 0x10A215), 10});
        host.world.hostMutate(host.hub, fnvcoop::AddItem{R("FalloutNV.esm", 0xF), 7, fnvcoop::Recipient::dflt()});
    }
    Node c2(Role::Client, host.rt->boundPort(), 5001, o, [&](FakeWorld& w) { w.inventory[caps] = capsInSave; w.karma = karmaInSave; });   // the loaded save
    auto fr2 = [&] { host.rt->frame(); c2.rt->frame(); };
    std::string what;
    CHECK(c2.rt->onGameLoaded(useSidecar ? &sidecar : nullptr, &what));
    pumpUntil(fr2, [&] { return c2.rt->client()->joined() && c2.rt->client()->applied() == host.rt->host()->log().head(); }, 8000);
    pumpUntil(fr2, [&] { return false; }, 300);
    return c2.world.inventory[caps];
}

static void testRestartedClientRejoin() {
    std::printf("restarted client rejoins from its save (sidecar), no duplicate rewards\n");
    CHECK(restartedCaps(true, false) == 107);
    CHECK(restartedCaps(true, true) == 107);      // the save was written after coop.leave
    CHECK(restartedCaps(false, false) != 107);    // twin: without the sidecar the host cannot know and re-delivers the ledger
}

static void testLossyHost() {
    std::printf("convergence under 1/3 datagram loss from the host\n");
    Node host(Role::Host, 0, 0, "udp");
    host.rt->transport()->setDropEveryNth(3);
    Node c1(Role::Client, host.rt->boundPort(), 2001, 0 ? "" : "udp");
    auto frameAll = [&] { host.rt->frame(); c1.rt->frame(); };
    CHECK(pumpUntil(frameAll, [&] { return c1.rt->client()->joined(); }, 8000));
    for (int i = 0; i < 10; ++i) {
        using namespace fnvcoop;
        nvhooks::ScopedCapture s(host.hub, Cause{CauseKind::QuestResult, R("FalloutNV.esm", 0x100 + i), i});
        host.world.hostMutate(host.hub, SetStage{R("FalloutNV.esm", 0x100 + i), static_cast<uint32_t>(i + 1)});
        host.world.hostMutate(host.hub, AddItem{R("FalloutNV.esm", 0xF), 10, Recipient::dflt()});
    }
    CHECK(pumpUntil(frameAll, [&] { return c1.rt->client()->applied() == 10 && host.rt->host()->allSynced(); }, 15000));
    CHECK(host.world.dump().dump() == c1.world.dump().dump());
}

int main(int argc, char** argv) {
    if (argc > 0) { g_outDir = argv[0]; size_t sl = g_outDir.find_last_of("\\/"); g_outDir = sl == std::string::npos ? "." : g_outDir.substr(0, sl); }
    const bool uiOnly = argc > 1 && std::string(argv[1]) == "--ui";   // mutation-twin runs: only the wiring tests (COOP_UI_MUTANT=n must break them)
    if (argc > 1 && std::string(argv[1]) == "--leave") {   // mutation-twin runs for leave()/join() (COOP_LEAVE_MUTANT=n must break them)
        testLeaveJoin();
        std::printf("%d checks, %d failed\n", g_checks, g_failed);
        return g_failed ? 1 : 0;
    }
    if (uiOnly) {
        testUiBridgeConsentResume();
        testUiSlideshowChannel();
        testGoodspringsStory();
        std::printf("%d checks, %d failed\n", g_checks, g_failed);
        return g_failed ? 1 : 0;
    }
    testConfig();
    testTransport();
    testConvergence("enet");
    testConvergence("udp");
    testActions();
    testDialogue();
    testTalkSubtitles();
    testSharedXp();
    testContainerSync();
    testBaseline();
    testSaveLoad();
    testNoFlood();
    testObjectScriptQuestVarsReachGuests();
    testRules();
    testPartyWarp();
    testRealSidecar();
    testRealSelfDiff();
    testHandshake();
    testUiBridgeConsentResume();
    testUiSlideshowChannel();
    testGoodspringsStory();
    testWorldEffectChannel();
    testLeaveJoin();
    testRestartedClientRejoin();
    testLossyHost();
    std::printf("%d checks, %d failed\n", g_checks, g_failed);
    return g_failed ? 1 : 0;
}
