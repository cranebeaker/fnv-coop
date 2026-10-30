// L10 in-process tests: config parsing, UDP transport, host+client Runtime convergence over real loopback sockets.
// (The two-PROCESS test lives in tests/test_integration.py and drives coop_peer.exe.)
#include <chrono>
#include <cstdio>
#include <functional>
#include <memory>
#include <thread>
#include <vector>

#include "fake_world.h"
#include "guest_item.h"
#include "spectator_say.h"
#include "baseline_file.h"
#include "change_tracker.h"
#include "udp_transport.h"
#include "common/nvcoop_api.h"
#include "party_state.h"
#include "save_sidecars.h"
#include "objvar_sidecar.h"
#include "stash_order.h"

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
    struct Opt { const char* transport = "enet"; bool baseline = false; int seed = 0; bool dialogue = false; bool blockClientLoad = false; bool rules = false; bool handshake = false; bool partyWarp = false; bool noPlayerRef = false; int bindGhosts = 0; /*1: bindPlayerRefs with ghosts for slots 1,2; 2: ghost for slot 1 only*/ uint64_t buildHash = 0; unsigned maxPlayers = 0; std::vector<std::string> plugins; bool ui = false; const char* rulesJson = nullptr; std::function<void(Runtime&)> preStart; std::string host; /*client: the host address*/ };
    nvhooks::CaptureHub hub;
    FakeWorld world;
    std::unique_ptr<Runtime> rt;
    std::vector<fnvcoop_dlg::Event> dlgEvents;
    UiRig ui;
    int dlgRuns = 0;
    int dlgRunsInScope = 0, scopesOpened = 0;   // wave 2: runs that happened while the result scope handle was alive
    bool scopeOpen = false;
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
        if (!o.host.empty()) c.host = o.host;
        c.baseline = o.baseline;
        c.clientLoadBlock = o.blockClientLoad;
        c.handshake = o.handshake;
        if (o.maxPlayers) c.maxPlayers = o.maxPlayers;
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
            if (!rt->setRules(o.rulesJson ? o.rulesJson : kRules, &rerr)) std::printf("  rules failed: %s\n", rerr.c_str());
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
                if (scopeOpen) ++dlgRunsInScope;
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
            w.varName = [](const fnvcoop::FormRef&, uint32_t idx, std::string* name) { if (idx != 0) return false; *name = "Banned"; return true; };
            rt->enableUi(std::move(w));
        }
        rt->setWorldEffectApplier([this](const std::string& j) { return world.applyWorldJson(j); });
        if (o.preStart) o.preStart(*rt);
        std::string err;
        if (!rt->start(&err)) std::printf("  start failed: %s\n", err.c_str());
    }
    // Host: NPC state of a lane-local world effect; the host applies it to its own fake world, guests get it through the runtime.
    void pushWorld(const std::string& key, const std::string& json) { world.applyWorldJson(json); rt->pushWorldEffect(key, json); }
};

#include "load_guard.h"
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
    CHECK(e.clientLoadBlock);                                             // only the host loads saves unless the host allows it
    Config r;
    CHECK(ParseConfig("[coop]\nclientload=rejoin\n", &r, &err) && !r.clientLoadBlock);
    // the load guard: a joined guest under the block is refused, except the console's LoadGame (the mod's own start save / Continue)
    CHECK(!nvcoop::coop::guestLoadAllowed(true, 0x00851234u));
    CHECK(nvcoop::coop::guestLoadAllowed(true, nvcoop::coop::kConsoleLoadReturn));
    CHECK(nvcoop::coop::guestLoadAllowed(false, 0x00851234u));
    Config f;
    CHECK(!ParseConfig("[coop]\nrole=banana\n", &f, &err));
    CHECK(!ParseConfig("[coop]\nport=99999\n", &f, &err));
    Config g;
    CHECK(ParseConfig("[coop]\nenable=1\nrole=client\ntransport=enet\nhost=127.0.0.1\nport=40123\n", &g, &err));
    CHECK(g.transport == "enet" && g.host == "127.0.0.1" && g.port == 40123);   // a Steam session is this: ENet to the helper's local port
    CHECK(!ParseConfig("[coop]\ntransport=steam\n", &g, &err));                // the game itself no longer talks to Steam (docs/steam.md)
    CHECK(!ParseConfig("[coop]\ntransport=carrier-pigeon\n", &g, &err));
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
    // a plain box has no consent round; a consent box the host decides is raised to the party too (G08: it used to stay local)
    CHECK(boxFor(host, sb, endgameBox(93, 0x123456)) == nvui::Route::Suppressed);
    CHECK(pumpUntil(fr, [&] { return menuIdOf(b) != id2; }));
    b.rt->dialogue()->menuReply(menuIdOf(b), 1, (uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return host.ui.eng.resumed() == 3; }));
    CHECK(host.ui.eng.lastButton() == 1 && menuIdOf(a) == id2);
    CHECK(boxFor(host, nvui::kHost, endgameBox(94)) == nvui::Route::Suppressed);
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


// Reward-multiplication report (story run S06, docs/handover/DLG-requests.md): ONE committed AddItem(Ammo556mm x60) with a full mask paid 3 to 4 times on both copies. Found in the
// host's log: the INFO result runner committed the reward once (the guest's window, origin 1), and the engine ran the same Begin script twice more by itself while the host played
// the two response lines of the INFO with `<npc>.Say` (host-origin commits, full mask, the host's real AddItem each time). A reward is applied once per player on that player's own
// machine, however many lines the voice has, however the txn travelled (live, resent after a lost datagram) and however many slots the mask names. The engine's part is modelled
// like the hook does it: the gate (plugin/hooks/spectator_say.h) decides whether the engine's own run of an INFO result script goes ahead.
static void testGuestRewardLandsOnce(unsigned players, const char* transport) {
    std::printf("reward once: a guest's AddItem with a %u-slot mask lands once on each of %u machines, with a 2-line voice (%s)\n", players, players, transport);
    Node::Opt o;
    o.maxPlayers = players;
    o.transport = transport;
    Node host(Role::Host, 0, 0, o);
    const bool lossy = std::string(transport) == "udp";
    if (lossy) host.rt->transport()->setDropEveryNth(3);
    std::vector<std::unique_ptr<Node>> guests;
    for (unsigned i = 1; i < players; ++i) guests.emplace_back(new Node(Role::Client, host.rt->boundPort(), 7100 + i, o));
    auto fr = [&] { host.rt->frame(); for (auto& g : guests) g->rt->frame(); };
    CHECK(pumpUntil(fr, [&] {
        if (host.rt->host()->clients().size() != guests.size()) return false;
        for (auto& g : guests) if (!g->rt->client() || !g->rt->client()->joined()) return false;
        return true;
    }, 10000));
    const auto ammo = R("FalloutNV.esm", 0x4240);
    const auto info = R("FalloutNV.esm", 0x10A1EB);
    const uint32_t sunny = 0x104E85;
    const std::string key = nvhooks::refToStr(ammo);
    nvhooks::SpectatorSayGate gate;
    // The engine's own run of the INFO's Begin script (what h_TopicRun lets through): the real `Player.AddItem` on the host's player, recorded by the hook.
    host.rt->setHostEffectApplier([&host](const fnvcoop::Effect& e) { fnvcoop::IEffectSink::Context c; c.seq = 1; host.world.apply(e, c); });   // the module's applier, for the host's own share
    auto engineRuns = [&](bool insideRunner, uint64_t now) {
        if (gate.blocks(sunny, insideRunner, now)) return;
        nvhooks::ScopedCapture sc(host.hub, fnvcoop::Cause{fnvcoop::CauseKind::DialogueResult, info, 0});
        const fnvcoop::AddItem add{ammo, 60, fnvcoop::Recipient::dflt()};
        if (nvhooks::planPlayerAddItem(true, insideRunner, true, true, false).runEngine) host.world.hostMutate(host.hub, add);   // the engine's own AddItem on the host's player
        else host.hub.record(add);                                                                                          // a guest acts: recorded only, the session gives the host its share
    };
    // 1. the dialogue host's INFO result runner: the host runs the guest's script in that guest's window; the committed txn carries the full mask.
    const uint64_t head0 = host.rt->host()->log().head();
    host.rt->runAsGuest(1, [&] { engineRuns(true, 1000); });
    CHECK(host.rt->host()->log().head() == head0 + 1);
    if (const fnvcoop::EffectTxn* t = host.rt->host()->log().get(host.rt->host()->log().head())) {
        CHECK(t->origin == 1 && t->effects.size() == 1);
        const auto* ai = t->effects.empty() ? nullptr : std::get_if<fnvcoop::AddItem>(&t->effects[0]);
        CHECK(ai && ai->count == 60 && ai->to.mode == fnvcoop::Recipient::Mask && ai->to.mask == ((1u << players) - 1u));
    }
    // 2. the host plays the INFO's two response lines for the guest's conversation: `Say` on the NPC twice; the engine runs the INFO's scripts for each (outside every window)
    for (int line = 0; line < 2; ++line) {
        gate.arm(sunny, 1200 + 100 * line);
        engineRuns(false, 1300 + 100 * line);
    }
    CHECK(pumpUntil(fr, [&] {
        for (auto& g : guests) if (g->rt->client()->applied() < 1) return false;
        return host.rt->host()->allSynced();
    }, 15000));
    pumpUntil(fr, [] { return false; }, lossy ? 1500 : 400);   // keep the frames going: a retransmit, a repair or a replay would land a further copy now
    CHECK(host.rt->host()->log().head() == head0 + 1);          // one commit for the one reward
    CHECK(host.world.inventory[key] == 60);
    for (auto& g : guests) CHECK(g->world.inventory[key] == 60);
    const auto& led = host.rt->host()->state().ledger;
    for (unsigned s = 0; s < players; ++s) {
        auto it = led[s].items.find(ammo);
        CHECK(it != led[s].items.end() && it->second == 60);
    }
    for (auto& g : guests) {
        const auto& m = g->rt->client()->state().ledger[g->rt->client()->slot()].items;
        auto it = m.find(ammo);
        CHECK(it != m.end() && it->second == 60);
    }
    CHECK(host.world.effectsApplied == 1);   // the host's share of the guest's reward is applied once by the session (the hook did not give it)
    CHECK(gate.blockedCount() == 2);
}

// A guest's reward goes to the players the committed mask names, on their own machines, and not to anybody else: a quest reward (mask = everybody) reaches the host once, and a
// guest-only reward (a script run for the guest by an intent) never reaches the host's pack. Same for a hand-in: the host pays only when the mask names it.
static void testGuestRewardRouting(unsigned players) {
    std::printf("reward routing: a guest-only reward and a hand-in never touch the host's pack, a mask naming the host does, once (%u players)\n", players);
    Node::Opt o;
    o.maxPlayers = players;
    Node host(Role::Host, 0, 0, o);
    std::vector<std::unique_ptr<Node>> guests;
    for (unsigned i = 1; i < players; ++i) guests.emplace_back(new Node(Role::Client, host.rt->boundPort(), 7300 + i, o));
    auto fr = [&] { host.rt->frame(); for (auto& g : guests) g->rt->frame(); };
    CHECK(pumpUntil(fr, [&] {
        if (host.rt->host()->clients().size() != guests.size()) return false;
        for (auto& g : guests) if (!g->rt->client() || !g->rt->client()->joined()) return false;
        return true;
    }, 10000));
    host.rt->setHostEffectApplier([&host](const fnvcoop::Effect& e) { fnvcoop::IEffectSink::Context c; c.seq = 1; host.world.apply(e, c); });
    const auto caps = R("FalloutNV.esm", 0xF), junk = R("FalloutNV.esm", 0x777001), key1 = R("FalloutNV.esm", 0x777002);
    const std::string kc = nvhooks::refToStr(caps), kj = nvhooks::refToStr(junk), kq = nvhooks::refToStr(key1);
    host.world.inventory[kc] = 100; host.world.inventory[kq] = 1;
    for (auto& g : guests) { g->world.inventory[kc] = 100; g->world.inventory[kq] = 1; }
    // the engine's `Player.AddItem` / `Player.RemoveItem` inside guest 1's script, as the hook does it (guest_item.h decides whether the engine call goes ahead)
    auto runFor1 = [&](fnvcoop::CauseKind kind, const fnvcoop::Effect& e) {
        host.rt->runAsGuest(1, [&] {
            nvhooks::ScopedCapture sc(host.hub, fnvcoop::Cause{kind, R("FalloutNV.esm", 0x10A1EB), 0});
            const bool add = std::holds_alternative<fnvcoop::AddItem>(e);
            const bool item = add || std::holds_alternative<fnvcoop::RemoveItem>(e);
            const bool engine = !item ? nvhooks::planGuestReward(true, true, true).runEngine
                                      : add ? nvhooks::planPlayerAddItem(true, true, true, true, false).runEngine : nvhooks::planPlayerRemoveItem(true, true, true, true).runEngine;
            if (engine) host.world.hostMutate(host.hub, e); else host.hub.record(e);
        });
    };
    auto settle = [&](uint64_t applied) {
        CHECK(pumpUntil(fr, [&] {
            for (auto& g : guests) if (g->rt->client()->applied() < applied) return false;
            return host.rt->host()->allSynced();
        }, 15000));
        pumpUntil(fr, [] { return false; }, 300);
    };
    // 1. a reward of a non-quest cause (a script an intent ran for the guest): the guest's alone
    runFor1(fnvcoop::CauseKind::Intent, fnvcoop::AddItem{junk, 3, fnvcoop::Recipient::dflt()});
    settle(1);
    CHECK(host.world.inventory[kj] == 0);
    CHECK(guests[0]->world.inventory[kj] == 3);
    for (size_t i = 1; i < guests.size(); ++i) CHECK(guests[i]->world.inventory[kj] == 0);
    // 2. a dialogue hand-in of caps: the paying guest loses them, the host keeps its own
    runFor1(fnvcoop::CauseKind::DialogueResult, fnvcoop::RemoveItem{caps, 50, fnvcoop::Recipient::dflt()});
    settle(2);
    CHECK(host.world.inventory[kc] == 100);
    CHECK(guests[0]->world.inventory[kc] == 50);
    for (size_t i = 1; i < guests.size(); ++i) CHECK(guests[i]->world.inventory[kc] == 100);
    // 3. a quest item taken from everybody, a quest reward for everybody: the host's share once, every guest's on its own machine
    runFor1(fnvcoop::CauseKind::QuestResult, fnvcoop::RemoveItem{key1, 1, fnvcoop::Recipient::all()});
    runFor1(fnvcoop::CauseKind::QuestResult, fnvcoop::AddItem{junk, 5, fnvcoop::Recipient::all()});
    settle(4);
    CHECK(host.world.inventory[kq] == 0 && host.world.inventory[kj] == 5);
    for (auto& g : guests) CHECK(g->world.inventory[kq] == 0);
    CHECK(guests[0]->world.inventory[kj] == 8);
    for (size_t i = 1; i < guests.size(); ++i) CHECK(guests[i]->world.inventory[kj] == 5);
    CHECK(host.world.inventory[kc] == 100);
    // 4. XP, karma and reputation of a script run for the guest. XP of a non-quest cause is the guest's alone; karma and reputation are shared by the policy (everybody, once: the host's
    // share comes from the session, the engine did not move the host's player or standing); a quest reward is everybody's in full.
    const auto faction = R("FalloutNV.esm", 0x777003);
    const std::string kr = nvhooks::refToStr(faction) + "/fame";
    runFor1(fnvcoop::CauseKind::Intent, fnvcoop::ModActorValue{fnvcoop::kAvExperience, 40, fnvcoop::Recipient::dflt()});
    settle(5);
    CHECK(host.world.xp == 0);                                                                          // the host's player does not get a guest-only XP
    CHECK(guests[0]->world.xp == 40);
    for (size_t i = 1; i < guests.size(); ++i) CHECK(guests[i]->world.xp == 0);
    runFor1(fnvcoop::CauseKind::Intent, fnvcoop::ModKarma{7, fnvcoop::Recipient::dflt()});
    runFor1(fnvcoop::CauseKind::Intent, fnvcoop::ModReputation{faction, 2, 0, fnvcoop::Recipient::dflt()});
    settle(7);
    CHECK(host.world.karma == 7 && host.world.reputation[kr] == 2);                                     // shared: the host once (applied by the session, not twice)
    for (auto& g : guests) CHECK(g->world.karma == 7 && g->world.reputation[kr] == 2);
    runFor1(fnvcoop::CauseKind::QuestResult, fnvcoop::ModActorValue{fnvcoop::kAvExperience, 100, fnvcoop::Recipient::all()});
    settle(8);
    CHECK(host.world.xp == 100 && guests[0]->world.xp == 140);
    for (size_t i = 1; i < guests.size(); ++i) CHECK(guests[i]->world.xp == 100);
}

// ---- campaign wave 2 (P6 session wiring) --------------------------------------------------------------------------------------------------
// Each test below has a runtime twin (FNVCOOP_MUT=n, run with --wiring by run_tests.cmd): the test must FAIL with the twin on.

// P2/P3 handover R1: a guest's INFO result script runs inside the result scope (PlayerFactsScope in the module), once per phase.
static void testResultScope() {
    std::printf("wave 2: dialogue result scripts run inside the result scope (initiator facts)\n");
    Node::Opt o;
    o.dialogue = true;
    Node host(Role::Host, 0, 0, o);
    host.rt->setResultScope([&host](const fnvcoop_dlg::ResultRequest& rq) -> std::shared_ptr<void> {
        ++host.scopesOpened;
        host.scopeOpen = true;
        (void)rq;
        return std::shared_ptr<void>(new int(1), [&host](void* p) { delete static_cast<int*>(p); host.scopeOpen = false; });
    });
    Node a(Role::Client, host.rt->boundPort(), 4101, o);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && a.rt->dialogue() && host.rt->host()->clients().size() == 1; }));
    a.rt->dialogue()->requestStart(R("FalloutNV.esm", 0x104C0F), (uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return a.rt->dialogue()->client().role() == fnvcoop_dlg::ClientDialogue::Role::Initiator; }));
    a.rt->dialogue()->pick(R("FalloutNV.esm", 0x105560), R("FalloutNV.esm", 0x105583), 0, false, {fnvcoop_dlg::Line{"House.", "", 0, 50}}, {}, (uint32_t)nowMs());
    a.rt->dialogue()->lineDone((uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return host.dlgRuns == 2; }));
    CHECK(host.dlgRuns == 2);
    CHECK(host.scopesOpened == 2);          // one scope per run (begin, end)
    CHECK(host.dlgRunsInScope == 2);        // and the script really ran while it was alive
    CHECK(!host.scopeOpen);                 // closed again afterwards
}

// P5 handover 2: a one-shot presentation effect reaches the guests connected now and is never replayed to a late joiner.
static void testEphemeralWorldEffect() {
    std::printf("wave 2: ephemeral world effects (fade, sound) are not replayed to a late joiner\n");
    Node::Opt o;
    o.baseline = true;
    o.seed = 3;
    Node host(Role::Host, 0, 0, o);
    const char* fade = R"({"type":"SetEssential","actor":"falloutnv.esm:000AAB","essential":true,"refOnly":false})";
    const char* keep = R"({"type":"SetEssential","actor":"falloutnv.esm:000AAC","essential":true,"refOnly":false})";
    Node a(Role::Client, host.rt->boundPort(), 5401, o);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && !a.rt->syncing(); }));
    host.rt->pushWorldEffect("", fade, true);              // ephemeral: a is connected now
    host.rt->pushWorldEffect("ess:c:base", keep, false);   // state: remembered
    CHECK(pumpUntil(fr, [&] { return a.world.worldApplied == 2; }));
    Node b(Role::Client, host.rt->boundPort(), 5402, o);   // late joiner
    auto fr2 = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); };
    CHECK(pumpUntil(fr2, [&] { return b.rt->client()->joined() && !b.rt->syncing() && b.world.worldApplied >= 1; }));
    pumpUntil(fr2, [] { return false; }, 150);
    CHECK(b.world.worldApplied == 1);                       // only the state effect; the fade is gone
    CHECK(host.rt->worldStats().ephemeral == 1);
}

// G42: a kill meant for ONE guest reaches that guest only; a guest's request (G10 note) reaches the host with the sender's slot.
static void testPlayerOrders() {
    std::printf("wave 2: player orders (kill one guest, guest request to the host)\n");
    Node::Opt o;
    Node host(Role::Host, 0, 0, o);
    Node a(Role::Client, host.rt->boundPort(), 5501, o), b(Role::Client, host.rt->boundPort(), 5502, o);
    std::vector<std::string> gotA, gotB, gotHost;
    std::vector<int> fromSlot;
    a.rt->setOrderApplier([&](const std::string& j) { gotA.push_back(j); return std::string("Ok"); });
    b.rt->setOrderApplier([&](const std::string& j) { gotB.push_back(j); return std::string("Ok"); });
    host.rt->setOrderHandler([&](fnvcoop::PlayerId from, const std::string& j) { fromSlot.push_back((int)from); gotHost.push_back(j); return std::string("Ok"); });
    auto fr = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && b.rt->client()->joined() && host.rt->host()->clients().size() == 2; }));
    CHECK(!host.rt->killPlayer(0));                                   // the host's own player is killed by the caller, not by an order
    const fnvcoop::PlayerId slotA = a.rt->client()->slot();
    CHECK(host.rt->killPlayer(slotA));
    CHECK(pumpUntil(fr, [&] { return !gotA.empty(); }));
    pumpUntil(fr, [] { return false; }, 100);
    CHECK(gotA.size() == 1 && gotB.empty());                          // only the named slot dies
    CHECK(!gotA.empty() && gotA[0].find("\"kill\"") != std::string::npos);
    CHECK(!host.rt->sendOrder(7, "{\"op\":\"kill\"}"));              // nobody there
    CHECK(b.rt->sendOrderToHost(R"({"op":"note","note":"falloutnv.esm:00ABCD"})"));
    CHECK(pumpUntil(fr, [&] { return !gotHost.empty(); }));
    CHECK(gotHost.size() == 1 && !fromSlot.empty() && fromSlot[0] == (int)b.rt->client()->slot());
    CHECK(host.rt->orderStats().handled == 1 && a.rt->orderStats().applied == 1);
}


// G23 / G10 call sites: the party the evaluators loop over, and what the session answers for a remote member (perk, note, companion owner).
static void testPartyState() {
    std::printf("wave 2: party conditions (present members, per-slot perk / note / companion answers, session reducer)\n");
    using nvhooks::CondFunc;
    Node::Opt o;
    Node host(Role::Host, 0, 0, o);
    Node a(Role::Client, host.rt->boundPort(), 5601, o), b(Role::Client, host.rt->boundPort(), 5602, o);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && b.rt->client()->joined() && host.rt->host()->clients().size() == 2; }));
    const fnvcoop::PlayerId sa = a.rt->client()->slot(), sb = b.rt->client()->slot();
    const fnvcoop::FormRef perk = R("FalloutNV.esm", 0x7777), note = R("FalloutNV.esm", 0x8888), npc = R("FalloutNV.esm", 0x9999);
    {   // the host commits: a perk for guest A only, a note for everyone, a companion that guest B recruited
        nvhooks::ScopedCapture sc(host.hub, fnvcoop::Cause{fnvcoop::CauseKind::Script, R("FalloutNV.esm", 0x1234), 0});
        host.hub.record(fnvcoop::SetPerk{perk, 1, fnvcoop::Recipient::slot(sa)});
        host.hub.record(fnvcoop::AddNote{note});
        host.hub.record(fnvcoop::CompanionOwner{npc, (uint8_t)sb, true, 0});
    }
    CHECK(pumpUntil(fr, [&] { return host.rt->host()->allSynced() && a.rt->client()->applied() >= 1; }));
    auto toRef = [](uint32_t id, fnvcoop::FormRef* out) { *out = R("FalloutNV.esm", id); return true; };
    nvhooks::StateProvider prov = makePartyStateProvider(sourceFromRuntime(*host.rt, toRef));
    auto ask = [&](CondFunc f, uint8_t slot, uint32_t thisId, uint32_t formId, double* v) {
        nvhooks::StateQuery q;
        q.f = f; q.slot = slot; q.thisRefID = thisId; q.formID = formId;
        return prov(q, v);
    };
    double v = -1;
    CHECK(ask(CondFunc::HasPerk, sa, 0x14, 0x7777, &v) && v == 1.0);          // A has it
    CHECK(ask(CondFunc::HasPerk, sb, 0x14, 0x7777, &v) && v == 0.0);          // B does not: the perk is not shared by accident
    CHECK(ask(CondFunc::GetHasNote, sb, 0x14, 0x8888, &v) && v == 1.0);       // the journal is shared
    CHECK(ask(CondFunc::GetHasNote, sb, 0x14, 0x8889, &v) && v == 0.0);
    CHECK(ask(CondFunc::GetPlayerTeammate, sb, 0x9999, 0, &v) && v == 1.0);   // B owns the companion
    {   // wave 1b: level, sex, seat, action: only what a source can answer; a member nobody can answer for is skipped (false)
        PartyStateSource src = sourceFromRuntime(*host.rt, toRef);
        src.level = [&](uint8_t s, double* out) { if (s != sa) return false; *out = 12; return true; };
        src.sex = [&](uint8_t s, int* out) { if (s != sa) return false; *out = 1; return true; };
        src.sitting = [&](uint8_t s, double* out) { *out = s == sa ? 1.0 : 0.0; return true; };
        src.actionActive = [&](uint8_t s, int32_t id, double* out) { *out = s == sa && id == 7 ? 1.0 : 0.0; return true; };
        nvhooks::StateProvider p2 = makePartyStateProvider(src);
        auto ask2 = [&](CondFunc f, uint8_t slot, int32_t arg1, double* out) { nvhooks::StateQuery q; q.f = f; q.slot = slot; q.arg1 = arg1; return p2(q, out); };
        CHECK(ask2(CondFunc::GetLevel, sa, 0, &v) && v == 12.0 && !ask2(CondFunc::GetLevel, sb, 0, &v));
        CHECK(ask2(CondFunc::GetIsSex, sa, 1, &v) && v == 1.0 && ask2(CondFunc::GetIsSex, sa, 0, &v) && v == 0.0);
        CHECK(ask2(CondFunc::GetPCIsSex, sa, 1, &v) && v == 1.0);
        CHECK(ask2(CondFunc::GetSitting, sa, 0, &v) && v == 1.0 && ask2(CondFunc::GetSitting, sb, 0, &v) && v == 0.0);
        CHECK(ask2(CondFunc::IsPlayerActionActive, sa, 7, &v) && v == 1.0 && ask2(CondFunc::IsPlayerActionActive, sa, 8, &v) && v == 0.0);
    }
    {   // W3a item 3: the avatar layer answers GetEquipped, IsSneaking, IsPlayerInRegion, GetLevel, sex, seat and action for a guest (twin 36: it is not connected)
        PartyStateSource src = sourceFromRuntime(*host.rt, toRef);
        bindSlotFacts(src, [&](uint8_t s, nvcoop::SlotFacts* out) {
            nvcoop::SlotFacts f;
            if (s == sa) {
                f.haveSex = true; f.sex = 1;
                f.haveEquip = true; f.equipped = {0x4001, 0x4002};
                f.haveSneak = true; f.sneaking = true;
                f.haveFacts = true; f.level = 22; f.sitting = 2; f.actions = 1u << 9; f.regions = {0x6001};
            } else if (s == sb) {
                f.haveSex = true; f.sex = 0;                                   // B has only delivered its Look so far
            } else {
                return false;
            }
            *out = f;
            return true;
        });
        nvhooks::StateProvider p3 = makePartyStateProvider(src);
        auto ask3 = [&](CondFunc f, uint8_t slot, uint32_t formId, int32_t arg1, double* out) { nvhooks::StateQuery q; q.f = f; q.slot = slot; q.formID = formId; q.arg1 = arg1; return p3(q, out); };
        CHECK(ask3(CondFunc::GetEquipped, sa, 0x4002, 0, &v) && v == 1.0 && ask3(CondFunc::GetEquipped, sa, 0x4003, 0, &v) && v == 0.0);
        CHECK(!ask3(CondFunc::GetEquipped, sb, 0x4002, 0, &v));               // B's worn items are not known yet: skipped, not "nothing worn"
        CHECK(ask3(CondFunc::IsSneaking, sa, 0, 0, &v) && v == 1.0 && !ask3(CondFunc::IsSneaking, sb, 0, 0, &v));
        CHECK(ask3(CondFunc::IsPlayerInRegion, sa, 0x6001, 0, &v) && v == 1.0 && ask3(CondFunc::IsPlayerInRegion, sa, 0x6002, 0, &v) && v == 0.0);
        CHECK(ask3(CondFunc::GetLevel, sa, 0, 0, &v) && v == 22.0 && !ask3(CondFunc::GetLevel, sb, 0, 0, &v));
        CHECK(ask3(CondFunc::GetPCIsSex, sa, 0, 1, &v) && v == 1.0 && ask3(CondFunc::GetIsSex, sb, 0, 0, &v) && v == 1.0);
        CHECK(ask3(CondFunc::GetSitting, sa, 0, 0, &v) && v == 2.0 && !ask3(CondFunc::GetSitting, sb, 0, 0, &v));
        CHECK(ask3(CondFunc::IsPlayerActionActive, sa, 0, 9, &v) && v == 1.0 && ask3(CondFunc::IsPlayerActionActive, sa, 0, 8, &v) && v == 0.0);
        CHECK(!ask3(CondFunc::IsPlayerActionActive, sa, 0, 40, &v));          // an id the guest does not sample
        CHECK(!ask3(CondFunc::GetLevel, 5, 0, 0, &v));                        // a slot nobody knows
    }
    {   // guest pack facts: GetItemCount of a watched form and InventoryWeight of a guest come from its Pack message (twins 960: not connected, 961: the weight answers every actor value)
        PartyStateSource src = sourceFromRuntime(*host.rt, toRef);
        bindSlotPack(src,
                     [&](uint8_t s, uint32_t item, int64_t* n) {
                         if (s != sa) return false;                       // B has not delivered a pack
                         if (item == 0x7001) { *n = 3; return true; }
                         if (item == 0x7002) { *n = 0; return true; }     // watched, not carried
                         return false;                                    // not on the list both machines share
                     },
                     [&](uint8_t s, double* w) { if (s != sa) return false; *w = 75.96; return true; });
        nvhooks::StateProvider p5 = makePartyStateProvider(src);
        auto ask5 = [&](CondFunc f, uint8_t slot, uint32_t formId, int32_t arg1, double* out) { nvhooks::StateQuery q; q.f = f; q.slot = slot; q.formID = formId; q.arg1 = arg1; return p5(q, out); };
        CHECK(ask5(CondFunc::GetItemCount, sa, 0x7001, 0, &v) && v == 3.0);
        CHECK(ask5(CondFunc::GetItemCount, sa, 0x7002, 0, &v) && v == 0.0);
        CHECK(!ask5(CondFunc::GetItemCount, sa, 0x7003, 0, &v));              // an item nobody watches: the puppet answers as before
        CHECK(!ask5(CondFunc::GetItemCount, sb, 0x7001, 0, &v));              // B has no pack yet
        CHECK(!ask5(CondFunc::GetItemCount, sa, 0, 0, &v));                   // no item at all
        CHECK(ask5(CondFunc::GetActorValue, sa, 0, 46, &v) && v == 75.96);    // InventoryWeight: the Jed gate reads 75.96 < 76
        CHECK(!ask5(CondFunc::GetActorValue, sa, 0, 5, &v));                  // any other actor value is the puppet's
        CHECK(!ask5(CondFunc::GetActorValue, sb, 0, 46, &v));
        PartyStateSource bare = sourceFromRuntime(*host.rt, toRef);          // nothing connected: unknown, never a guess
        nvhooks::StateProvider p6 = makePartyStateProvider(bare);
        nvhooks::StateQuery q6; q6.f = CondFunc::GetItemCount; q6.slot = sa; q6.formID = 0x7001;
        CHECK(!p6(q6, &v));
    }
    CHECK(ask(CondFunc::GetPlayerTeammate, sa, 0x9999, 0, &v) && v == 0.0);   // A does not
    CHECK(!ask(CondFunc::GetPlayerTeammate, sa, 0x9998, 0, &v));              // nobody's: the engine flag alone stands
    CHECK(!ask(CondFunc::IsSneaking, sa, 0x14, 0, &v));                       // no avatar state wired: unknown, member skipped
    // present members: the host + the guests whose puppet exists (A has one, B has not placed its puppet yet)
    auto ghost = [&](fnvcoop::PlayerId s) -> uint32_t { return s == sa ? 0x2000u + s : 0u; };
    auto members = presentMembers(*host.rt, ghost);
    CHECK(members.size() == 2 && members[0].slot == 0 && members[0].refID == 0x14 && members[1].slot == sa && members[1].refID == 0x2000u + sa);
    // the session reducer: no rules engine -> the initiator's own value
    auto red = makeSessionReducer([] { return static_cast<fnvcoop::RulesEngine*>(nullptr); });
    std::vector<nvhooks::CondValue> vals{{0, 5.0}, {(uint8_t)sa, 9.0}};
    CHECK(red(R("FalloutNV.esm", 1), "GetItemCount", vals, (uint8_t)sa, fnvcoop::CondPhase::Entry) == 9.0);
    // with a rules engine the quest's PartyAwareCond reduction applies (max-over-party)
    Config c;
    c.enable = true;
    c.role = Role::Host;
    c.port = 0;
    nvhooks::CaptureHub hub2;
    FakeWorld w2;
    Runtime rt2(c, &hub2, &w2, nowMs);
    std::string rerr;
    CHECK(rt2.setRules(R"({"schema":1,"quests":{"MaxQ":{"tier":"T3","questline":"x","curated":true,"rules":[{"kind":"PartyAwareCond","params":{"funcs":{"GetItemCount":"max-over-party"}},"origin":"curated"}]}}})", &rerr));
    rt2.setEdidResolver([](const std::string&, fnvcoop::FormRef& out) { out = fnvcoop::FormRef("falloutnv.esm", 0x4242); return true; });
    std::string serr;
    CHECK(rt2.start(&serr));
    auto red2 = makeSessionReducer([&rt2] { return rt2.rules(); });
    CHECK(red2(fnvcoop::FormRef("falloutnv.esm", 0x4242), "GetItemCount", vals, 0, fnvcoop::CondPhase::Entry) == 9.0);
    rt2.stop();
}


// P4 handover 8: the gear ledger travels with the save game that holds it.
static void testSaveSidecars() {
    std::printf("wave 2: per-save sidecars (the gear ledger follows the save, a save without one resets it)\n");
    std::map<std::string, std::string> fs;
    std::string ledger;   // the state under test
    SaveSidecars sc;
    sc.add({".fnvintents", [&] { return ledger; }, [&](const std::string& t) { ledger = t; }});
    SaveSidecars::Io io;
    io.write = [&](const std::string& p, const std::string& t) { fs[p] = t; return true; };
    io.read = [&](const std::string& p, std::string* t) { auto it = fs.find(p); if (it == fs.end()) return false; *t = it->second; return true; };
    ledger = "gear:A";
    sc.onSave("Saves\one.fos", io);
    ledger = "gear:A+B";
    sc.onSave("Saves\two.fos", io);
    CHECK(fs.size() == 2 && fs["Saves\one.fos.fnvintents"] == "gear:A");
    sc.onPreLoad("Saves\one.fos");
    sc.onPostLoad(io);
    CHECK(ledger == "gear:A");                    // the older save rolls the ledger back
    sc.onPreLoad("Saves\old.fos");
    sc.onPostLoad(io);
    CHECK(ledger.empty());                        // a save that has none resets it (it must not keep the previous save's gear)
    ledger.clear();
    sc.onSave("Saves\three.fos", io);
    CHECK(fs.size() == 2);                        // nothing to store: no file
}


// G33 / P4 note 2: the follower caps from nvcoop.ini reach every dialogue node; puppets are registered as acting players again after a load.
static void testCompanionCapsAndSlots() {
    std::printf("wave 2: companion caps reach the dialogue node, puppets register as acting players\n");
    Node::Opt o;
    o.dialogue = true;
    Node host(Role::Host, 0, 0, o);
    fnvcoop_dlg::CompanionRules rules;
    rules.humanoidCap = 2;
    rules.critterCap = 0;
    rules.partyTotal = 3;
    // Node enabled dialogue without caps; a second runtime with caps proves the plumbing
    Config c;
    c.enable = true;
    c.role = Role::Host;
    c.port = 0;
    nvhooks::CaptureHub hub2;
    FakeWorld w2;
    Runtime rt2(c, &hub2, &w2, nowMs);
    DialogueHooks h;
    h.run = [](const fnvcoop_dlg::ResultRequest&) -> uint64_t { return 0; };
    h.sink = [](const fnvcoop_dlg::Event&) {};
    h.companionRules = rules;
    rt2.enableDialogue(h);
    std::string err;
    CHECK(rt2.start(&err));
    for (int i = 0; i < 5; ++i) rt2.frame();
    CHECK(rt2.dialogue() != nullptr);
    if (rt2.dialogue()) {
        const auto& r = rt2.dialogue()->companions().rules();
        CHECK(r.humanoidCap == 2 && r.critterCap == 0 && r.partyTotal == 3);
    }
    rt2.stop();
    // acting-player registration
    Node a(Role::Client, host.rt->boundPort(), 5701, o);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1; }));
    const fnvcoop::PlayerId sa = a.rt->client()->slot();
    SlotRegistrar reg;
    std::vector<std::pair<uint32_t, int>> regs;
    auto ghost = [&](fnvcoop::PlayerId s) -> uint32_t { return s == sa ? 0x3000u : 0u; };
    auto doReg = [&](uint32_t id, uint8_t s) { regs.emplace_back(id, (int)s); };
    reg.sync(*host.rt, ghost, doReg);
    reg.sync(*host.rt, ghost, doReg);
    CHECK(regs.size() == 1 && regs[0].first == 0x3000u && regs[0].second == (int)sa);   // once, not every frame
    reg.reset();                                                                       // a save was loaded (clearPlayerSlots)
    reg.sync(*host.rt, ghost, doReg);
    CHECK(regs.size() == 2);                                                           // registered again
}


// ---- campaign wave 2, the rules engine in the session (P1 handover to P6) -----------------------------------------------------------------------------------
static fnvcoop::FormRef QF(const char* edid) {   // the test resolver (Node): an EDID is its hash
    uint32_t h = 2166136261u;
    for (const char* c = edid; *c; ++c) { h ^= (uint8_t)*c; h *= 16777619u; }
    return fnvcoop::FormRef("falloutnv.esm", h);
}

static const char* kW2Rules = R"({"schema":1,
 "policy":{"death":{"mode":"downed","revive_s":60,"all_down":"reload-host-save"},"late_join":{"during_scene":"wait","after_scene":"warp-to-zone"}},
 "quests":{
 "Ending":{"tier":"T4","questline":"x","curated":true,"rules":[
   {"kind":"SetPlayerControls","params":{"scope":"all","release":"host-effect","timeout_s":600,"masks":["1 1 1 1 1 1 1"]},"origin":"curated"},
   {"kind":"SaveSuppress","params":{},"origin":"curated"}]},
 "Casino":{"tier":"T3","questline":"x","curated":false,"rules":[{"kind":"PerPlayerVars","params":{"vars":["Banned","Frisked"]},"origin":"derived"}]},
 "Zone":{"tier":"T4","questline":"x","curated":false,"rules":[
   {"kind":"PartyTether","params":{"cells":["HooverDam"],"gather":"auto-warp","timeout_s":1,"stages":[10]},"origin":"questsweep"},
   {"kind":"PartyTether","params":{"cells":["HooverDam"],"gather":"wait","stages":[20]},"origin":"questsweep"}]},
 "Orbit":{"tier":"T4","questline":"x","curated":false,"rules":[
   {"kind":"PartyTether","params":{"anchor":"Satellite","radius":100,"gather":"wait","stages":[5]},"origin":"questsweep"}]},
 "Chargen":{"tier":"T4","questline":"x","curated":true,"rules":[
   {"kind":"Barrier","params":{"step":"all-players-pass","stages":[200]},"origin":"curated"}]},
 "House":{"tier":"T4","questline":"x","curated":true,"rules":[
   {"kind":"ReadyCheck","params":{"infos":["FalloutNV.esm:15EB69"],"timeout_s":120,"on_timeout":"cancel","confirm":"all-players"},"origin":"questsweep"}]},
 "Fork":{"tier":"T4","questline":"x","curated":true,"rules":[
   {"kind":"ChoiceConsent","params":{"boxes":["FalloutNV.esm:0AAA01"],"mode":"unanimous","timeout_s":120},"origin":"questsweep"}]},
 "Guard":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"FriendlyFireGuard","params":{"actors":["Joshua"],"deliberate_hits":3,"window_s":10},"origin":"questsweep"}]}
}})";

static std::string noticeText(const std::vector<std::string>& orders) {
    std::string all;
    for (auto& o : orders) all += o + "\n";
    return all;
}

// G02: a quest's own script commits with the SCRIPT form as its cause; the cause map points the rules engine at the quest, so the lock and the save block apply.
static void testCauseQuestsInSession() {
    std::printf("wave 2: cause quests (a quest script's own txn finds its rules), host-side effects of the rules engine\n");
    Node::Opt o;
    o.rules = true;
    o.rulesJson = kW2Rules;
    o.preStart = [](Runtime& rt) {
        rt.setCauseQuests(R"({"schema":1,"scripts":{"FalloutNV.esm:005C01":"Ending"}})");
    };
    Node host(Role::Host, 0, 0, o);
    std::vector<fnvcoop::Effect> hostApplied;
    host.rt->setHostEffectApplier([&](const fnvcoop::Effect& e) { hostApplied.push_back(e); });
    Node::Opt oc;
    Node a(Role::Client, host.rt->boundPort(), 6001, oc);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1; }));
    const fnvcoop::PlayerId sa = a.rt->client()->slot();
    {
        nvhooks::ScopedCapture sc(host.hub, fnvcoop::Cause{fnvcoop::CauseKind::Script, fnvcoop::FormRef("falloutnv.esm", 0x5C01), 0});
        host.hub.record(fnvcoop::SetPlayerControls{1, false, fnvcoop::Recipient::dflt()});
        host.hub.record(fnvcoop::SaveSuppress{true});
    }
    CHECK(pumpUntil(fr, [&] { return a.rt->saveSuppressed(); }));          // saving is blocked on the guest
    CHECK(a.rt->client()->state().ledger[sa].controlsOff == 1);             // and the lock reached it
    CHECK(host.rt->rules() && host.rt->rules()->causeQuestCount() == 1 && host.rt->rules()->stats().causesResolved >= 1);
    // an effect the rules engine commits by itself (a lock timeout, a gather) reaches the host's own player through the host applier, exactly once
    hostApplied.clear();
    host.rt->host()->commit(0, fnvcoop::Cause{fnvcoop::CauseKind::Script, QF("Ending"), 0}, {fnvcoop::SetPlayerControls{4, true, fnvcoop::Recipient::all()}});
    CHECK(hostApplied.size() == 1 && std::holds_alternative<fnvcoop::SetPlayerControls>(hostApplied[0]));
    host.rt->host()->commit(0, fnvcoop::Cause{fnvcoop::CauseKind::Script, QF("Ending"), 0},
                            {fnvcoop::MoveRef{fnvcoop::FormRef("falloutnv.esm", 0x1000u + sa), QF("HooverDam"), 0, 0, 0, true}});
    CHECK(hostApplied.size() == 1);                                         // a guest's move is not the host's
    host.rt->host()->commit(0, fnvcoop::Cause{fnvcoop::CauseKind::Script, QF("Ending"), 0},
                            {fnvcoop::MoveRef{fnvcoop::FormRef("falloutnv.esm", 0x14), QF("HooverDam"), 0, 0, 0, true}});
    CHECK(hostApplied.size() == 2);                                         // the host's own is
}

// G24: a guest's run sees and changes only that guest's copy of the per-player quest variables; the host plays on with its own.
static void testActingCopies() {
    std::printf("wave 2: acting-player copies of per-player quest variables around a guest's run\n");
    std::map<std::string, double> vars{{"Banned", 0}, {"Frisked", 0}};
    Node::Opt o;
    o.rules = true;
    o.rulesJson = kW2Rules;
    o.preStart = [&](Runtime& rt) {
        Runtime::PlayerVarIO io;
        io.read = [&](const fnvcoop::FormRef&, const std::vector<std::string>& names, std::vector<fnvcoop::RulesEngine::VarValue>& out) {
            for (auto& n : names) out.push_back({n, vars[n]});
            return true;
        };
        io.write = [&](const fnvcoop::FormRef&, const std::vector<fnvcoop::RulesEngine::VarValue>& vals) { for (auto& v : vals) vars[v.var] = v.value; };
        rt.setPlayerVarIO(io);
    };
    Node host(Role::Host, 0, 0, o);
    CHECK(host.rt->actingQuestCount() == 1);
    double seenB = -1, seenA = -1;
    host.rt->runAsGuest(1, [&] { vars["Banned"] = 1; });                      // guest 1 gets banned from the casino
    CHECK(vars["Banned"] == 0);                                               // the host's own copy is back in the variable
    host.rt->runAsGuest(2, [&] { seenB = vars["Banned"]; });
    CHECK(seenB == 0);                                                        // guest 2 is not banned
    host.rt->runAsGuest(1, [&] { seenA = vars["Banned"]; });
    CHECK(seenA == 1);                                                        // guest 1 still is
}

// G25: the host watches every player against the anchor of an anchor tether and feeds the rules engine; the stage waits for the whole party.
static void testAnchorTether() {
    std::printf("wave 2: anchor tether (near / far watch of every joined player)\n");
    float posAx = 0;
    int stageRuns = 0;
    Node::Opt o;
    o.rules = true;
    o.rulesJson = kW2Rules;
    o.preStart = [&](Runtime& rt) {
        Runtime::AnchorProbe p;
        p.anchorPos = [](const fnvcoop::FormRef&, float& x, float& y, float& z) { x = 1000; y = 0; z = 0; return true; };
        p.slotPos = [&](fnvcoop::PlayerId s, float& x, float& y, float& z) { x = s == 0 ? 1000.f : posAx; y = 0; z = 0; return true; };
        rt.setAnchorProbe(p);
        rt.setStageApplier([&](const fnvcoop::FormRef&, uint32_t) { ++stageRuns; });
    };
    Node host(Role::Host, 0, 0, o);
    Node::Opt oc;
    Node a(Role::Client, host.rt->boundPort(), 6101, oc);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1; }));
    CHECK(!host.rt->stageRequested(QF("Orbit"), 5, 0));                       // the guest is far: the stage is held
    pumpUntil(fr, [] { return false; }, 1300);
    CHECK(stageRuns == 0);
    posAx = 1050;                                                             // the guest walks up to the satellite
    CHECK(pumpUntil(fr, [&] { return stageRuns == 1; }, 4000));
    CHECK(host.rt->rulesStats().nearReports >= 2);
}

// G26: the host's own chargen end and a guest's stage intent are each player's pass; the barrier opens when everybody passed.
// ---- the guest's character creation order (chargen_flow.h) ------------------------------------------------------------------------------------------
#include <map>
#include <set>
#include <string>
#include "chargen_flow.h"
namespace {
using nvcoop::coop::ChargenFlow;
using nvcoop::coop::ChargenFlowConfig;
using nvcoop::coop::ChargenStepRule;
// A scripted game: `opens[k]` says whether the k-th line of a step makes its menu appear (menu id 2000 + step); an open menu closes after `openFor` frames.
struct ChargenGame {
    ChargenFlow flow;
    uint64_t now = 0;
    std::set<int> open;
    std::map<int, int> willOpenOnAttempt;   // step -> the attempt (1-based) whose line works; absent = every attempt works
    std::map<int, int> neverOpens;          // step -> 1: no attempt ever works
    int openFor = 10;
    int pauseMenuUntilMs = 0;               // the pause menu (1013) is open until this time
    std::vector<std::pair<int, int>> issued;   // (step, attempt)
    std::vector<std::string> log;
    int closeIn = 0, curMenu = 0;
    bool finished = false;
    uint64_t pauseSeenAtIssue = 0;
    std::vector<bool> pauseOpenAtIssue;
    ChargenGame(ChargenFlowConfig cfg = {}) : flow({{"name", false}, {"face", false}, {"SPECIAL", true}, {"tag skills", true}, {"traits", false}}, cfg) { flow.start(0); }
    void Frame() {
        now += 16;
        std::set<int> view = open;
        if (now < static_cast<uint64_t>(pauseMenuUntilMs)) view.insert(1013);
        const ChargenFlow::Out o = flow.frame(now, view);
        for (const std::string& l : flow.takeLog()) log.push_back(l);
        if (o.what == ChargenFlow::Do::Finish) finished = true;
        if (o.what == ChargenFlow::Do::Issue) {
            issued.push_back({o.step, o.attempt});
            pauseOpenAtIssue.push_back(view.count(1013) != 0);
            flow.issued(now, view);
            const bool works = !neverOpens.count(o.step) && (!willOpenOnAttempt.count(o.step) || willOpenOnAttempt[o.step] == o.attempt);
            if (works) {
                open.insert(2000 + o.step);
                closeIn = openFor;
                curMenu = 2000 + o.step;
            }
        }
        if (curMenu && --closeIn <= 0) {
            open.erase(curMenu);
            curMenu = 0;
        }
    }
    void Run(uint64_t ms) {
        const uint64_t end = now + ms;
        while (now < end && !finished) Frame();
    }
};
}  // namespace

static void testGuestChargenFlow() {
    std::printf("guest chargen flow\n");
    {   // all five menus open at once: five steps, in order, once each
        ChargenGame g;
        g.Run(60000);
        CHECK(g.finished);
        CHECK(g.issued.size() == 5);
        for (size_t i = 0; i < g.issued.size(); ++i) CHECK(g.issued[i].first == static_cast<int>(i) && g.issued[i].second == 1);
    }
    {   // the SPECIAL line does nothing the first time (the game was not ready for it): it is tried again, the step is not lost
        ChargenGame g;
        g.willOpenOnAttempt[2] = 2;
        g.Run(120000);
        CHECK(g.finished);
        int specialLines = 0;
        for (auto& i : g.issued) specialLines += i.first == 2;
        CHECK(specialLines == 2);
        bool tryAgain = false, skipped = false;
        for (auto& l : g.log) {
            tryAgain = tryAgain || l.find("SPECIAL: no menu after attempt 1") != std::string::npos;
            skipped = skipped || l.find("SPECIAL never opened") != std::string::npos;
        }
        CHECK(tryAgain && !skipped);
        CHECK(g.issued.size() == 6);   // five steps and the one repeated line
    }
    {   // a line that never works is tried maxAttempts times and then skipped; the steps after it still run
        ChargenGame g;
        g.neverOpens[3] = 1;
        g.Run(200000);
        CHECK(g.finished);
        int lines = 0;
        for (auto& i : g.issued) lines += i.first == 3;
        CHECK(lines == 3);
        bool skipped = false;
        for (auto& l : g.log) skipped = skipped || l.find("tag skills never opened after 3 attempts") != std::string::npos;
        CHECK(skipped);
        bool traitsRan = false;
        for (auto& i : g.issued) traitsRan = traitsRan || i.first == 4;
        CHECK(traitsRan);
    }
    {   // the old behaviour (one line per step, then skipped) loses the step: this is what the owner's two-copy run showed
        ChargenFlowConfig cfg;
        cfg.maxAttempts = 1;
        ChargenGame g(cfg);
        g.willOpenOnAttempt[2] = 2;
        g.Run(120000);
        int specialLines = 0;
        for (auto& i : g.issued) specialLines += i.first == 2;
        CHECK(specialLines == 1);
        bool skipped = false;
        for (auto& l : g.log) skipped = skipped || l.find("SPECIAL never opened") != std::string::npos;
        CHECK(skipped);
    }
    {   // the pause menu (the game window lost focus): SPECIAL and tag skills wait until it is gone; name, face and traits do not
        ChargenGame g;
        g.pauseMenuUntilMs = 15000;
        g.Run(120000);
        CHECK(g.finished);
        for (size_t i = 0; i < g.issued.size(); ++i) {
            const int step = g.issued[i].first;
            const bool quietStep = step == 2 || step == 3;
            if (quietStep) CHECK(!g.pauseOpenAtIssue[i]);
        }
        bool waited = false;
        for (auto& l : g.log) waited = waited || l.find("waits for the pause menu") != std::string::npos;
        CHECK(waited);
    }
    {   // a pause menu that never goes away does not hold the guest back for ever
        ChargenFlowConfig cfg;
        cfg.escapeWaitMs = 5000;
        ChargenGame g(cfg);
        g.pauseMenuUntilMs = 1 << 30;
        g.Run(200000);
        CHECK(g.finished);
        bool gaveUp = false;
        for (auto& l : g.log) gaveUp = gaveUp || l.find("going on anyway") != std::string::npos;
        CHECK(gaveUp);
    }
    {   // a message box in the way holds the next step back until it is closed
        ChargenGame g;
        g.open.insert(1001);
        g.Run(3000);
        CHECK(g.issued.empty());
        g.open.erase(1001);
        g.Run(60000);
        CHECK(g.finished && g.issued.size() == 5);
    }
}

static void testChargenPasses() {
    std::printf("wave 2: chargen passes (host stage, guest stage intent)\n");
    int stageRuns = 0;
    Node::Opt o;
    o.rules = true;
    o.rulesJson = kW2Rules;
    o.preStart = [&](Runtime& rt) { rt.setStageApplier([&](const fnvcoop::FormRef&, uint32_t) { ++stageRuns; }); };
    Node host(Role::Host, 0, 0, o);
    Node::Opt oc;
    Node a(Role::Client, host.rt->boundPort(), 6201, oc);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1; }));
    CHECK(!host.rt->stageRequested(QF("Chargen"), 200, 0));                   // the host's chargen end: held, and it counts as the host's pass
    CHECK(stageRuns == 0);
    CHECK(host.rt->stageHeldForPass(a.rt->client()->slot(), QF("Chargen"), 200));   // the guest's intent is its pass (held too, then everybody has passed)
    pumpUntil(fr, [&] { return stageRuns == 1; }, 2000);
    CHECK(stageRuns == 1);                                                    // released once
    CHECK(!host.rt->stageHeldForPass(a.rt->client()->slot(), QF("House"), 5));  // not a pass stage: the intent is handled as usual
}

// G27: an INFO that is a ReadyCheck holds its end script until everybody confirmed.
static void testReadyCheckInfo() {
    std::printf("wave 2: ready check on an INFO end script (held until the party confirmed)\n");
    Node::Opt o;
    o.rules = true;
    o.dialogue = true;
    o.rulesJson = kW2Rules;
    Node host(Role::Host, 0, 0, o);
    Node::Opt oc;
    oc.dialogue = true;
    Node a(Role::Client, host.rt->boundPort(), 6301, oc);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && a.rt->dialogue() && host.rt->host()->clients().size() == 1; }));
    a.rt->dialogue()->requestStart(R("FalloutNV.esm", 0x104C0F), (uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return a.rt->dialogue()->client().role() == fnvcoop_dlg::ClientDialogue::Role::Initiator; }));
    a.rt->dialogue()->pick(R("FalloutNV.esm", 0x105560), R("FalloutNV.esm", 0x15EB69), 0, false, {fnvcoop_dlg::Line{"I am ready.", "", 0, 50}}, {}, (uint32_t)nowMs());
    a.rt->dialogue()->lineDone((uint32_t)nowMs());
    CHECK(pumpUntil(fr, [&] { return host.dlgRuns >= 1; }));                  // the begin script
    pumpUntil(fr, [] { return false; }, 400);
    CHECK(host.dlgRuns == 1);                                                 // the end script waits for the host to confirm
    host.rt->scenePass(R("FalloutNV.esm", 0x15EB69));                          // the host says it is ready too
    CHECK(pumpUntil(fr, [&] { return host.dlgRuns == 2; }));
    CHECK(host.rt->rulesStats().rulesLinks == 1);
}

// G40: the combat gate. An accidental hit on a protected story actor is blocked with a warning to the attacker; a deliberate one goes through.
static void testHitGate() {
    std::printf("wave 2: friendly fire guard (combat gate)\n");
    std::vector<std::string> noticesA;
    Node::Opt o;
    o.rules = true;
    o.rulesJson = kW2Rules;
    Node host(Role::Host, 0, 0, o);
    Node::Opt oc;
    Node a(Role::Client, host.rt->boundPort(), 6401, oc);
    a.rt->setNotice([&](const std::string& t) { noticesA.push_back(t); });
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1; }));
    const fnvcoop::PlayerId sa = a.rt->client()->slot();
    using HV = Runtime::HitVerdict;
    CHECK(host.rt->protectedHit(QF("Nobody"), sa, false) == HV::Allow);       // no rule protects it
    CHECK(host.rt->protectedHit(QF("Joshua"), sa, true) == HV::BlockWarn);    // splash: blocked, the attacker is told once
    CHECK(pumpUntil(fr, [&] { return !noticesA.empty(); }));
    CHECK(host.rt->protectedHit(QF("Joshua"), sa, false) == HV::Block);       // one direct hit: still an accident
    CHECK(host.rt->protectedHit(QF("Joshua"), sa, false) == HV::Block);
    CHECK(host.rt->protectedHit(QF("Joshua"), sa, false) == HV::Allow);       // three direct hits: meant it
}

// G41: a death is a downed state for that player; the last one down reloads the host save; a late joiner waits for a running scene.
static void testDeathAndLateJoin() {
    std::printf("wave 2: death policy (downed, all down) and the late joiner held during a scene\n");
    bool reloaded = false;
    std::vector<std::string> hostOrders, ordersA;
    Node::Opt o;
    o.rules = true;
    o.rulesJson = kW2Rules;
    o.preStart = [&](Runtime& rt) {
        rt.setReloadHostSave([&] { reloaded = true; });
        rt.setOrderApplier([&](const std::string& j) { hostOrders.push_back(j); return std::string("Ok"); });
    };
    Node host(Role::Host, 0, 0, o);
    Node::Opt oc;
    Node a(Role::Client, host.rt->boundPort(), 6501, oc);
    a.rt->setOrderApplier([&](const std::string& j) { ordersA.push_back(j); return std::string("Ok"); });
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1; }));
    pumpUntil(fr, [] { return false; }, 400);
    a.rt->localPlayerDied();                                                  // the guest's player died: the host decides
    CHECK(pumpUntil(fr, [&] { return noticeText(ordersA).find("\"downed\"") != std::string::npos; }));
    CHECK(!reloaded);                                                         // the host is still up
    host.rt->localPlayerDied();                                               // now everybody is down
    CHECK(noticeText(hostOrders).find("\"downed\"") != std::string::npos || reloaded);
    CHECK(reloaded);
    // a late joiner during a scene (the chargen barrier is pending): held with its controls locked, released when the scene ends
    int stageRuns = 0;
    host.rt->setStageApplier([&](const fnvcoop::FormRef&, uint32_t) { ++stageRuns; });
    CHECK(!host.rt->stageRequested(QF("Chargen"), 200, 0));
    Node::Opt ob;
    Node b(Role::Client, host.rt->boundPort(), 6502, ob);
    auto fr2 = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); };
    CHECK(pumpUntil(fr2, [&] { return b.rt->client()->joined() && host.rt->host()->clients().size() == 2; }));
    CHECK(pumpUntil(fr2, [&] { return b.rt->client()->state().ledger[b.rt->client()->slot()].controlsOff != 0; }));   // held
    CHECK(host.rt->stageHeldForPass(a.rt->client()->slot(), QF("Chargen"), 200));
    CHECK(host.rt->stageHeldForPass(b.rt->client()->slot(), QF("Chargen"), 200));
    CHECK(pumpUntil(fr2, [&] { return stageRuns == 1; }));
    CHECK(pumpUntil(fr2, [&] { return b.rt->client()->state().ledger[b.rt->client()->slot()].controlsOff == 0; }));   // released with the scene
}

// Owner switch: nvcoop.ini [rules] tether=warn never holds a stage and names the far player; the exterior worldspace satisfies a tether.
static void testTetherPolicyAndWorldspace() {
    std::printf("wave 2: tether policy warn (ini [rules]), worldspace reports\n");
    Node::Opt o;
    o.rules = true;
    o.rulesJson = kW2Rules;
    o.preStart = [](Runtime& rt) { rt.setPolicy({{"tether.mode", "warn"}}); };
    Node host(Role::Host, 0, 0, o);
    std::vector<std::string> noticesA;
    Node::Opt oc;
    Node a(Role::Client, host.rt->boundPort(), 6601, oc);
    a.rt->setNotice([&](const std::string& t) { noticesA.push_back(t); });
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1; }));
    host.rt->reportCell(QF("Grid(1,2)"), QF("HooverDam"));
    a.rt->reportCell(QF("Somewhere"), fnvcoop::FormRef());                     // an interior: the cell itself
    pumpUntil(fr, [] { return false; }, 400);
    CHECK(host.rt->stageRequested(QF("Zone"), 20, 0));                         // warn: the stage is NOT held
    CHECK(pumpUntil(fr, [&] { return !noticesA.empty(); }));                   // the far guest is told
    CHECK(host.rt->rules()->stats().tetherWarnings == 1);
}

// Tether (default policy on): the exterior worldspace satisfies it (a guest reports the worldspace), and after timeout_s the far guest is warped to the zone.
static void testGatherAndWorldReport() {
    std::printf("wave 2: tether gather (auto-warp of the far guest) and worldspace reports\n");
    Node::Opt o;
    o.rules = true;
    o.rulesJson = kW2Rules;
    Node host(Role::Host, 0, 0, o);
    Node::Opt oc;
    Node a(Role::Client, host.rt->boundPort(), 6701, oc);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1; }));
    const fnvcoop::PlayerId sa = a.rt->client()->slot();
    host.rt->reportCell(QF("Grid(1,2)"), QF("HooverDam"));
    a.rt->reportCell(QF("Grid(3,4)"), QF("HooverDam"));                        // an exterior: the worldspace is the cell it reports
    pumpUntil(fr, [] { return false; }, 400);
    CHECK(host.rt->stageRequested(QF("Zone"), 20, 0));                         // everybody is in the zone: nothing held
    a.rt->reportCell(QF("Elsewhere"), fnvcoop::FormRef());                     // the guest goes into another interior
    pumpUntil(fr, [] { return false; }, 400);
    CHECK(!host.rt->stageRequested(QF("Zone"), 10, 0));                        // gather=auto-warp, timeout_s=1: held
    const fnvcoop::FormRef gp("falloutnv.esm", 0x1000u + sa);
    CHECK(pumpUntil(fr, [&] { return host.rt->host()->state().placed.count(gp) == 1; }, 4000));   // the far guest was warped to the zone
    CHECK(host.rt->rulesStats().casesGathered == 1);
}


// 2026-10-30 ("on the CLIENT I was constantly running and RUBBER BANDING back"): the session never holds or moves a player silently. A tether hold tells the players it
// waits for (and the others why the story waits), the warp that follows is announced to the player and is a log line (what, why, from where to where).
static void testTetherHoldAndWarpAreAnnounced() {
    std::printf("2026-10-30: a tether hold and its warp are on-screen messages and log lines\n");
    Node::Opt o;
    o.rules = true;
    o.rulesJson = kW2Rules;
    Node host(Role::Host, 0, 0, o);
    std::vector<std::string> trace, noticesA, noticesHost;
    host.rt->setTrace([&](const std::string& l) { trace.push_back(l); });
    host.rt->setNotice([&](const std::string& t) { noticesHost.push_back(t); });
    Node::Opt oc;
    Node a(Role::Client, host.rt->boundPort(), 6711, oc);
    a.rt->setNotice([&](const std::string& t) { noticesA.push_back(t); });
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1; }));
    const fnvcoop::PlayerId sa = a.rt->client()->slot();
    host.rt->reportCell(QF("Grid(1,2)"), QF("HooverDam"));
    a.rt->reportCell(QF("Elsewhere"), fnvcoop::FormRef());
    pumpUntil(fr, [] { return false; }, 400);
    CHECK(!host.rt->stageRequested(QF("Zone"), 10, 0));                       // gather=auto-warp, timeout_s=1: held
    CHECK(pumpUntil(fr, [&] { return noticeText(noticesA).find("Go to your party; in 1 s you are brought there") != std::string::npos; }));   // the far guest is told, with the time, why the story waits
    CHECK(noticeText(noticesA).find("needs the whole party") != std::string::npos);
    CHECK(noticeText(noticesHost).find("Waiting for the rest of the party") != std::string::npos);     // and the one who is there hears why the story waits
    CHECK(noticeText(trace).find("HOLDS story: tether") != std::string::npos);
    CHECK(pumpUntil(fr, [&] { return noticeText(trace).find("MOVES player: tether gather") != std::string::npos; }, 4000));   // the warp is a log line: what, why, who ...
    CHECK(noticeText(trace).find("warping slot " + std::to_string((int)sa)) != std::string::npos);
    CHECK(noticeText(trace).find("from (position unknown) to zone") != std::string::npos || noticeText(trace).find(" to zone ") != std::string::npos);   // ... from where to where
    CHECK(pumpUntil(fr, [&] { return noticeText(noticesA).find("being brought to your party now") != std::string::npos; }));   // ... and the player heard it before it happened
}

// A late joiner held during a scene and warped afterwards: both are log lines, and the warp is announced.
static void testLateJoinHoldAndWarpAreLogged() {
    std::printf("2026-10-30: a late joiner's hold and warp are log lines and messages\n");
    Node::Opt o;
    o.rules = true;
    o.rulesJson = kW2Rules;
    Node host(Role::Host, 0, 0, o);
    std::vector<std::string> trace, noticesB;
    host.rt->setTrace([&](const std::string& l) { trace.push_back(l); });
    host.rt->setStageApplier([](const fnvcoop::FormRef&, uint32_t) {});
    Node::Opt oa;
    Node a(Role::Client, host.rt->boundPort(), 6721, oa);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1; }));
    pumpUntil(fr, [] { return false; }, 400);                                  // A's own late join is over (no scene, no zone yet)
    CHECK(!host.rt->stageRequested(QF("Chargen"), 200, 0));                   // a barrier scene runs ...
    Node::Opt ob;
    Node b(Role::Client, host.rt->boundPort(), 6722, ob);
    b.rt->setNotice([&](const std::string& t) { noticesB.push_back(t); });
    auto fr2 = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); };
    CHECK(pumpUntil(fr2, [&] { return b.rt->client()->joined() && host.rt->host()->clients().size() == 2; }));
    CHECK(pumpUntil(fr2, [&] { return noticeText(trace).find("HOLDS player: late join of slot") != std::string::npos; }));   // ... B is held: a log line ...
    CHECK(noticeText(noticesB).find("A scene is running") != std::string::npos);                                            // ... and a message
    host.rt->rules()->setZone(QF("HooverDam"));                               // the party has a checkpoint
    CHECK(host.rt->stageHeldForPass(a.rt->client()->slot(), QF("Chargen"), 200));   // everybody passes: the scene (and B's hold) ends
    CHECK(host.rt->stageHeldForPass(b.rt->client()->slot(), QF("Chargen"), 200));
    CHECK(pumpUntil(fr2, [&] { return noticeText(trace).find("MOVES player: late join of slot") != std::string::npos; }, 4000));   // the warp: what, why, from where to where
    CHECK(noticeText(trace).find("to the party's zone") != std::string::npos);
    CHECK(pumpUntil(fr2, [&] { return noticeText(noticesB).find("You join your party.") != std::string::npos; }));   // (the hold said "...when it ends.")
}

// W3a item 5 (P6 request 1): a guest-only scene lock does not lock the host. controlsGuestOnly is what the SetPlayerControls hook asks inside the capture scope.
static void testControlsGuestOnly() {
    std::printf("W3a: a lock for the initiator alone skips the host's own engine call\n");
    Node::Opt o;
    o.rules = true;
    o.rulesJson = R"({"schema":1,"quests":{
      "Scene":{"tier":"T3","questline":"x","curated":false,"rules":[{"kind":"SetPlayerControls","params":{"scope":"initiator","release":"host-effect","timeout_s":60,"masks":["1 1 1 1 1 1 1"]},"origin":"curated"}]},
      "Ending":{"tier":"T4","questline":"x","curated":true,"rules":[{"kind":"SetPlayerControls","params":{"scope":"all","release":"host-effect","timeout_s":60,"masks":["1 1 1 1 1 1 1"]},"origin":"curated"}]}}})";
    o.preStart = [](Runtime& rt) { rt.setCauseQuests(R"({"schema":1,"scripts":{"FalloutNV.esm:005C02":"Scene","FalloutNV.esm:005C01":"Ending"}})"); };
    Node host(Role::Host, 0, 0, o);
    Node::Opt oc;
    Node a(Role::Client, host.rt->boundPort(), 6851, oc);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->host()->clients().size() == 1; }));
    const fnvcoop::PlayerId sa = a.rt->client()->slot();
    CHECK(!host.rt->controlsGuestOnly(sa, true));                                       // no scope open: the engine call runs
    {
        nvhooks::ScopedCapture sc(host.hub, fnvcoop::Cause{fnvcoop::CauseKind::Script, fnvcoop::FormRef("falloutnv.esm", 0x5C02), 0});
        CHECK(host.rt->controlsGuestOnly(sa, true));                                    // scope=initiator, a guest acts: the host is not locked
        CHECK(host.rt->controlsGuestOnly(sa, false));                                   // ... nor released
        CHECK(!host.rt->controlsGuestOnly(0, true));                                    // the host acts: its own lock
    }
    {
        nvhooks::ScopedCapture sc(host.hub, fnvcoop::Cause{fnvcoop::CauseKind::Script, fnvcoop::FormRef("falloutnv.esm", 0x5C01), 0});
        CHECK(!host.rt->controlsGuestOnly(sa, true));                                   // scope=all: everybody locks, the host too
    }
    {
        nvhooks::ScopedCapture sc(host.hub, fnvcoop::Cause{fnvcoop::CauseKind::Script, fnvcoop::FormRef("falloutnv.esm", 0x5C77), 0});
        CHECK(!host.rt->controlsGuestOnly(sa, true));                                   // a script no quest rule knows
    }
    CHECK(host.rt->rulesStats().controlsHostSkipped == 2);
}

// W3a item 4: around a script that runs for a guest the shared companion variables hold THAT guest's view; a view-party script (a mover) and the rest of the session see the aggregate.
static void testCompanionViewsAroundScripts() {
    std::printf("W3a: the acting guest's hired flags and Dead Money rCurrentFollower around its scripts\n");
    const fnvcoop::FormRef quest = R("FalloutNV.esm", 0xB16D0), dmQuest = R("DeadMoney.esm", 0x12FD), npcA = R("FalloutNV.esm", 0x79A12), npcB = R("FalloutNV.esm", 0x79A13);
    std::map<std::string, double> vars{{"bAHired", 1}, {"bBHired", 1}, {"rCurrentFollower", 777}};
    std::vector<std::string> writes;
    Node::Opt o;
    o.dialogue = true;
    Node host(Role::Host, 0, 0, o);
    Node::Opt oc;
    oc.dialogue = true;
    Node a(Role::Client, host.rt->boundPort(), 6861, oc);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->dialogue() && host.rt->host()->clients().size() == 1; }));
    const fnvcoop::PlayerId sa = a.rt->client()->slot();
    auto& reg = host.rt->dialogue()->companions();
    reg.define(npcA, fnvcoop_dlg::CompanionKind::Humanoid);
    reg.define(npcB, fnvcoop_dlg::CompanionKind::Humanoid);
    reg.addVarRule(quest, 6, fnvcoop_dlg::VarKind::Hired, npcA);
    reg.addVarRule(quest, 7, fnvcoop_dlg::VarKind::Hired, npcB);
    reg.addVarRule(quest, 8, fnvcoop_dlg::VarKind::Available, npcB);      // not per-player state: never rewritten
    reg.setOwner(npcA, sa);                                               // the guest has A, the host has B
    reg.setOwner(npcB, 0);
    Runtime::PlayerVarIO io;
    io.read = [&](const fnvcoop::FormRef&, const std::vector<std::string>& names, std::vector<fnvcoop::RulesEngine::VarValue>& out) {
        for (auto& n : names) if (vars.count(n)) out.push_back({n, vars[n]});
        return !out.empty();
    };
    io.write = [&](const fnvcoop::FormRef&, const std::vector<fnvcoop::RulesEngine::VarValue>& v) { for (auto& x : v) { vars[x.var] = x.value; writes.push_back(x.var); } };
    io.name = [&](const fnvcoop::FormRef&, uint32_t idx, std::string* name) { if (idx == 6) *name = "bAHired"; else if (idx == 7) *name = "bBHired"; else return false; return true; };
    host.rt->setPlayerVarIO(io);
    host.rt->setViewPolicy([](uint32_t script, uint8_t acting) -> uint8_t { return script == 0xAAA ? uint8_t(0xFE) : acting; },
                           [&](fnvcoop::PlayerId slot, Runtime::FollowerVar* out) { out->quest = dmQuest; out->var = "rCurrentFollower"; out->formId = slot == sa ? 0x4242 : 0; return true; });
    host.rt->onScriptRun(0x111, true);                                    // the host's own run (no guest window): nothing is rewritten
    host.rt->onScriptRun(0x111, false);
    CHECK(vars["bAHired"] == 1 && vars["bBHired"] == 1 && writes.empty());
    host.rt->runAsGuest(sa, [&] {
        host.rt->onScriptRun(0x111, true);                                // a hire / fire script run for the guest: its own flags
        CHECK(vars["bAHired"] == 1 && vars["bBHired"] == 0);              // B is the host's companion
        CHECK(vars["rCurrentFollower"] == 0x4242);                        // its own Dead Money companion
        host.rt->onScriptRun(0xAAA, true);                                // a mover nested in it reads the party
        CHECK(vars["bAHired"] == 1 && vars["bBHired"] == 1);
        host.rt->onScriptRun(0xAAA, false);
        CHECK(vars["bBHired"] == 0 && vars["rCurrentFollower"] == 0x4242);   // back in the hire script
        host.rt->onScriptRun(0x111, false);
        CHECK(vars["bBHired"] == 1 && vars["rCurrentFollower"] == 777);   // and the shared variables are the aggregate again
        host.rt->onScriptRun(0x111, true);                                // the window closing restores them even when a script never reported its end
        CHECK(vars["bBHired"] == 0);
    });
    CHECK(vars["bAHired"] == 1 && vars["bBHired"] == 1 && vars["rCurrentFollower"] == 777);
    CHECK(vars.count("bCAvailable") == 0 && host.rt->rulesStats().companionViews >= 4);
}

// P7 handover item 2 and 4: the roster listener feeds the dialogue registry; the host commits the registry's reconcile fixes.
static void testRosterAndReconcile() {
    std::printf("wave 2: companion roster feeds the registry, reconcile fixes are committed\n");
    const fnvcoop::FormRef quest = R("FalloutNV.esm", 0xB16D0), npc = R("FalloutNV.esm", 0x79A12);
    std::vector<fnvcoop::Effect> hostApplied;
    Node::Opt o;
    o.dialogue = true;
    o.preStart = [&](Runtime& rt) {
        rt.setScriptVarReader([&](const fnvcoop::FormRef&, std::map<uint16_t, double>& vars) { vars[18] = 0; return true; });   // the engine says nobody has a follower
        rt.setHostEffectApplier([&](const fnvcoop::Effect& e) { hostApplied.push_back(e); });
    };
    Node host(Role::Host, 0, 0, o);
    Node::Opt oc;
    oc.dialogue = true;
    Node a(Role::Client, host.rt->boundPort(), 6801, oc);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && host.rt->dialogue() && host.rt->host()->clients().size() == 1; }));
    host.rt->dialogue()->companions().define(npc, fnvcoop_dlg::CompanionKind::Humanoid);
    host.rt->dialogue()->companions().addVarRule(quest, 18, fnvcoop_dlg::VarKind::HasFollower);   // VNPCFollowers.bPlayerHasFollower
    host.rt->onRosterChanged({{npc, 1, false}});
    CHECK(host.rt->dialogue()->companions().ownerOf(npc) == 1);                  // the registry learned who owns it
    const fnvcoop::ShadowState& st = host.rt->host()->state();
    auto it = st.scriptVars.find({quest, 18});
    CHECK(it != st.scriptVars.end() && it->second == 1.0);                       // the shared flag was put right (it read 0)
    CHECK(hostApplied.size() == 1);                                                // and so was the host's own engine variable
    CHECK(!host.rt->dialogue()->companions().isPartyOwned(npc));
    host.rt->onRosterChanged({{npc, 1, true}});                                    // ED-E: owned by the whole party
    CHECK(host.rt->dialogue()->companions().isPartyOwned(npc));
}

// Module frame order (avatar lag): the late co-op module runs after the capture modules, whatever order they registered in.
static void testFrameOrder() {
    std::printf("wave 2: module frame order (the co-op session runs after the avatar capture)\n");
    static std::vector<std::string> ran;
    ran.clear();
    std::vector<nvcoop::Module> mods;
    mods.push_back(nvcoop::Module{"coop", nullptr, nullptr, [] { ran.push_back("coop"); }, 100});       // registered FIRST (link order is not a promise)
    mods.push_back(nvcoop::Module{"avatar", nullptr, nullptr, [] { ran.push_back("avatar"); }});
    mods.push_back(nvcoop::Module{"hooks", nullptr, nullptr, [] { ran.push_back("hooks"); }});
    nvcoop::RunFrameCallbacks(mods);
    CHECK(ran.size() == 3 && ran[0] == "avatar" && ran[1] == "hooks" && ran[2] == "coop");
    // the plain registration-order loop (the old dispatch) would have sent last frame's avatar snapshot: this is what the test exists to catch
    std::vector<std::string> naive;
    ran.clear();
    for (auto& m : mods) if (m.onFrame) m.onFrame();
    naive = ran;
    CHECK(naive.size() == 3 && naive[0] == "coop");
}


// Wave 1b: the rules engine's per-player variables and ChoiceConsent boxes reach the UI bridge; the dialogue host is linked to the rules engine.
static void testRulesViewsAndLink() {
    std::printf("wave 2: UI bridge views of the rules engine (per-player vars, rules-owned boxes), dialogue host linked\n");
    Node::Opt o;
    o.rules = true;
    o.ui = true;
    o.dialogue = true;
    o.rulesJson = kW2Rules;
    Node host(Role::Host, 0, 0, o);
    auto fr = [&] { host.rt->frame(); };
    pumpUntil(fr, [&] { return host.rt->dialogue() && host.ui.router && host.ui.router->bridge().hasPlayerVars != nullptr; }, 3000);
    if (!(host.ui.router && host.ui.router->bridge().hasPlayerVars != nullptr)) { CHECK(false); return; }
    CHECK(host.ui.router->bridge().hasPlayerVars(QF("Casino")));              // PerPlayerVars: the condition views are on for this quest
    CHECK(!host.ui.router->bridge().hasPlayerVars(QF("Orbit")));
    CHECK(host.ui.router->bridge().consentFor(fnvcoop::FormRef("falloutnv.esm", 0x0AAA01)).has_value());   // a ChoiceConsent box: the party is asked
    CHECK(!host.ui.router->bridge().consentFor(fnvcoop::FormRef("falloutnv.esm", 0x0BBB01)).has_value());
    CHECK(host.rt->rulesStats().rulesLinks == 1);                             // the dialogue host asks the rules engine (INFO end scripts, forks)
}

// ---- W3d (items 1 and 4): the options the rules engine asks the session, the scripts the rules run once per slot, the typed gear strip -------------------------------------
static const char* kW3dRules = R"({"schema":1,
 "quests":{
 "Collar":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"PerPlayerState","params":{"scope":"collar","vars":["iNumRadii"],"scripts":["SpeakerScript"],"objvars":["SpeakerScript.bInRadius","OtherScript.bX"]},"origin":"questsweep"}]},
 "Hand":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"PooledItems","params":{"items":["Totem"]},"origin":"questsweep"},
   {"kind":"QuestItemDup","params":{"recipient":"all"},"origin":"derived"}]},
 "Tele":{"tier":"T2","questline":"x","curated":false,"rules":[
   {"kind":"PartyWarp","params":{"markers":["ArrivalMarker"],"mode":"slot-offset","absent":"hold","range":500},"origin":"questsweep"}]}
}})";

static std::string hashHex(const char* edid) {
    char b[48];
    std::snprintf(b, sizeof b, "FalloutNV.esm:%06X", QF(edid).localId);
    return b;
}

static void testW3dOptionsAndPerSlotScripts() {
    std::printf("W3d: item counts and distances for the rules engine, scripts that run once per slot\n");
    std::map<std::string, double> qvars{{"iNumRadii", 0}};
    int64_t held[3] = {1, 1, 0};
    float posX[3] = {0, 200, 900};
    Node::Opt o;
    o.rules = true;
    o.rulesJson = kW3dRules;
    o.bindGhosts = 1;
    o.preStart = [&](Runtime& rt) {
        rt.setCauseQuests(std::string("{\"schema\":1,\"scripts\":{\"") + hashHex("SpeakerScript") + "\":\"Collar\"}}");
        Runtime::PlayerVarIO io;
        io.read = [&](const fnvcoop::FormRef&, const std::vector<std::string>& names, std::vector<fnvcoop::RulesEngine::VarValue>& out) {
            for (auto& n : names) out.push_back({n, qvars[n]});
            return true;
        };
        io.write = [&](const fnvcoop::FormRef&, const std::vector<fnvcoop::RulesEngine::VarValue>& vals) { for (auto& v : vals) qvars[v.var] = v.value; };
        rt.setPlayerVarIO(io);
        rt.setItemCountProbe([&](fnvcoop::PlayerId s, const fnvcoop::FormRef&, int64_t& n) { if (s > 2) return false; n = held[s]; return true; });
        Runtime::AnchorProbe p;
        p.anchorPos = [](const fnvcoop::FormRef&, float& x, float& y, float& z) { x = y = z = 0; return true; };
        p.slotPos = [&](fnvcoop::PlayerId s, float& x, float& y, float& z) { if (s > 2) return false; x = posX[s]; y = z = 0; return true; };
        rt.setAnchorProbe(p);
    };
    Node host(Role::Host, 0, 0, o);
    Node::Opt oc;
    Node a(Role::Client, host.rt->boundPort(), 6901, oc), b(Role::Client, host.rt->boundPort(), 6902, oc);
    auto fr = [&] { host.rt->frame(); a.rt->frame(); b.rt->frame(); };
    CHECK(pumpUntil(fr, [&] { return a.rt->client()->joined() && b.rt->client()->joined() && host.rt->host()->clients().size() == 2; }));
    const fnvcoop::PlayerId sa = a.rt->client()->slot(), sb = b.rt->client()->slot();
    CHECK(sa == 1 && sb == 2);

    // the scripts the rules run per slot, resolved, with the object variables of THAT script (the other script's entry is not ours)
    const auto info = host.rt->perSlotScripts();
    CHECK(info.size() == 1 && info[0].edid == "SpeakerScript" && info[0].script == QF("SpeakerScript") && info[0].vars.size() == 1 && info[0].vars[0] == "binradius");

    // the script runs once for each joined guest, in that guest's window, with that guest's copy of the quest variable; the host's copy comes back
    std::vector<fnvcoop::PlayerId> seen, acting;
    std::vector<double> inVar;
    auto body = [&](fnvcoop::PlayerId s) {
        seen.push_back(s);
        acting.push_back(host.rt->actingSlot());
        inVar.push_back(qvars["iNumRadii"]);
        qvars["iNumRadii"] = 10 + s;
    };
    CHECK(host.rt->runScriptPerSlot(QF("SpeakerScript").localId, body) == 2);
    CHECK(seen == std::vector<fnvcoop::PlayerId>({1, 2}) && acting == seen);                 // once per joined guest, the commit origin is that guest
    CHECK(inVar == std::vector<double>({0, 0}));                                             // both start from the shared initial state, not from each other
    CHECK(qvars["iNumRadii"] == 0);                                                          // the host's own copy is back for the engine
    CHECK(host.rt->actingSlot() == fnvcoop::kNoPlayer);
    inVar.clear();
    CHECK(host.rt->runScriptPerSlot(QF("SpeakerScript").localId, body) == 2);
    CHECK(inVar == std::vector<double>({11, 12}));                                           // the second run: each guest's own copy
    CHECK(host.rt->runScriptPerSlot(QF("NoSuchScript").localId, body) == 0 && seen.size() == 4);   // a script no rule runs per slot: nothing
    CHECK(host.rt->rulesStats().perSlotRuns == 4);

    // pooled hand-in: slot 1 hands in two totems, it holds one, the host holds the other
    const auto totem = QF("Totem");
    host.rt->host()->commit(sa, fnvcoop::Cause{fnvcoop::CauseKind::DialogueResult, QF("Hand"), 0}, {fnvcoop::RemoveItem{totem, 2, fnvcoop::Recipient::dflt()}});
    auto itemOf = [&](unsigned s) {
        const auto& m = host.rt->host()->state().ledger[s].items;
        auto it = m.find(totem);
        return it == m.end() ? 0ll : static_cast<long long>(it->second);
    };
    CHECK(itemOf(sa) == -1 && itemOf(0) == -1 && itemOf(sb) == 0);                            // charged to the holders, not twice to the initiator
    CHECK(host.rt->rulesStats().itemCountAsks >= 2 && host.rt->rules()->stats().pooledDebits == 1);

    // guest pack facts: the same hand-in run as a guest's script. `player.RemoveItem` did not touch the host's pack (nvhooks::GuestItemScope), so what the rules charge to the HOST
    // is taken from the host's pack here, once; the guest's part is an order for the guest's machine (the txn recipient), never the host's inventory
    std::vector<fnvcoop::Effect> hostApplied;
    host.rt->setHostEffectApplier([&](const fnvcoop::Effect& e) { hostApplied.push_back(e); });
    auto handIn = [&](bool asGuest, int n) {
        auto body = [&] {
            nvhooks::ScopedCapture sc(host.hub, fnvcoop::Cause{fnvcoop::CauseKind::DialogueResult, QF("Hand"), 0});
            host.hub.record(fnvcoop::RemoveItem{totem, n, fnvcoop::Recipient::dflt()});
        };
        if (asGuest) host.rt->runAsGuest(sa, body); else body();
    };
    auto removals = [&](unsigned slot) {
        std::vector<int32_t> r;
        for (const auto& e : hostApplied)
            if (const auto* ri = std::get_if<fnvcoop::RemoveItem>(&e)) if (ri->to.includes(slot)) r.push_back(ri->count);
        return r;
    };
    held[0] = 1; held[1] = 1; held[2] = 0;
    handIn(true, 2);                                                                          // the guest hands in two; it holds one, the host the other
    CHECK(removals(0) == std::vector<int32_t>({1}) && hostApplied.size() == 1);               // the host's share is taken from the host, exactly once
    hostApplied.clear();
    held[0] = 1; held[1] = 1;
    handIn(false, 2);                                                                         // the host's own script: its RemoveItem already ran in the engine
    CHECK(hostApplied.empty());
    held[0] = 0; held[1] = 2;
    handIn(true, 2);                                                                          // the guest carries all of it: nothing is taken from the host
    CHECK(hostApplied.empty());

    // the Teleporter: slot 1 is 200 units from the firer (in range), slot 2 is 900 away (out)
    host.rt->host()->commit(0, fnvcoop::Cause{fnvcoop::CauseKind::Script, QF("Tele"), 0}, {fnvcoop::MoveRef{R("FalloutNV.esm", 0x14), QF("ArrivalMarker"), 10, 0, 0, false}});
    std::set<uint32_t> moved;
    if (const auto* t = host.rt->host()->log().get(host.rt->host()->log().head()))
        for (auto& e : t->effects) if (auto* mv = std::get_if<fnvcoop::MoveRef>(&e)) moved.insert(mv->ref.localId);
    CHECK(moved.count(0x14) == 1 && moved.count(0x1000u + sa) == 1 && moved.count(0x1000u + sb) == 0);
    CHECK(host.rt->rulesStats().distanceAsks >= 2 && host.rt->rules()->stats().rangeWarpsHeld == 1);
}

// Follow-ups 1, item 3: the per-slot object variable copies of the scripts the rules run once per player are written next to the host's save and come back with it.
static void testObjVarSidecar() {
    std::printf("follow-ups 1: object variable copies travel with the host save\n");
    nvhooks::ObjVarCopies live;
    const std::vector<std::string> names = {"binradius", "fcollartimer"};
    const uint64_t A = nvhooks::objVarInstance(0x1000, 0x2000), B = nvhooks::objVarInstance(0x1000, 0x2001);   // two speakers running the same script
    double bin = 0, timer = 30;                                                                                 // pristine: what every slot starts from
    live.seed(A, names, {&bin, &timer});
    live.seed(B, names, {&bin, &timer});
    nvhooks::ObjVarCopies::Bracket br;
    double hostBin = 5, hostTimer = 99;
    live.enter(A, 1, names, {&hostBin, &hostTimer}, br);
    hostBin = 1; hostTimer = 12;                                                                               // slot 1 is in A's radius, its collar timer ran down to 12
    live.leave(A, 1, names, {&hostBin, &hostTimer}, br);
    live.enter(B, 2, names, {&hostBin, &hostTimer}, br);
    hostBin = 1; hostTimer = 3;
    live.leave(B, 2, names, {&hostBin, &hostTimer}, br);
    CHECK(hostBin == 5 && hostTimer == 99);                                                                     // the host's own values are back
    std::map<std::string, std::string> fs;
    SaveSidecars sc;
    sc.add({".fnvobjvars", [&] { return nvcoop::coop::objVarSnapshot(live); }, [&](const std::string& t) { nvcoop::coop::objVarRestore(live, t); }});
    SaveSidecars::Io io;
    io.write = [&](const std::string& p, const std::string& t) { fs[p] = t; return true; };
    io.read = [&](const std::string& p, std::string* t) { auto it = fs.find(p); if (it == fs.end()) return false; *t = it->second; return true; };
    sc.onSave("Saves/one.fos", io);
    CHECK(fs.size() == 1 && fs.count("Saves/one.fos.fnvobjvars") == 1);
    live.clear();                                                                                               // the host loads: the process memory of the old world is gone
    sc.onPreLoad("Saves/one.fos");
    sc.onPostLoad(io);
    double v = -1;
    CHECK(live.copyOf(A, "fcollartimer", 1, &v) && v == 12 && live.copyOf(A, "binradius", 1, &v) && v == 1);   // slot 1's timer did not restart
    CHECK(live.copyOf(B, "fcollartimer", 2, &v) && v == 3);
    CHECK(!live.copyOf(A, "fcollartimer", 2, nullptr) && !live.copyOf(B, "fcollartimer", 1, nullptr));       // slots and instances stay apart
    double loadedBin = 4, loadedTimer = 77;                                                                     // the host's variables as the load left them
    live.enter(A, 2, names, {&loadedBin, &loadedTimer}, br);                                                    // a slot that never ran on A starts from the seed, not from 77
    CHECK(loadedTimer == 30 && loadedBin == 0);
    live.leave(A, 2, names, {&loadedBin, &loadedTimer}, br);
    CHECK(loadedTimer == 77 && loadedBin == 4);
    sc.onPreLoad("Saves/old.fos");                                                                              // a save made before the feature has no file: the copies are reset
    sc.onPostLoad(io);
    CHECK(live.copies() == 0 && !live.copyOf(A, "fcollartimer", 1, nullptr));
    nvcoop::coop::objVarRestore(live, "junk without a header\nS 0000000000001000 x 1\n");                       // an unreadable file resets too
    CHECK(live.copies() == 0 && live.exportSeeds().empty());
    sc.onSave("Saves/three.fos", io);
    CHECK(fs.size() == 1);                                                                                      // nothing to store: no file
}

static void testStashOrderCarriesTheType() {
    std::printf("W3d: the gear strip order keeps its type filter (the Dead Money failure ending strips weapons only)\n");
    nvcoop::coop::StashOrder plain;
    plain.container = "deadmoney.esm:00efcc";
    plain.retain = true;
    nvcoop::coop::StashOrder typed = plain;
    typed.type = 40;
    typed.noMsg = true;
    typed.holdout = "deadmoney.esm:001234";
    auto parse = [](const std::string& js) {
        nvcoop::json::Value v;
        std::string err;
        nvcoop::json::parse(js, v, err);
        return nvcoop::coop::stashOrderFrom(v);
    };
    const auto p = parse(nvcoop::coop::stashOrderJson(plain));
    CHECK(p.type < 0 && p.container == plain.container && p.retain && !p.into);
    const auto t = parse(nvcoop::coop::stashOrderJson(typed));
    CHECK(t.type == 40 && t.noMsg && t.holdout == typed.holdout && t.retain);                  // survives the wire
    const auto outOfPlayer = nvcoop::coop::stashLine(t, 0xEFCC, 0x1234);
    CHECK(outOfPlayer.text == "RemoveAllTypedItems 0000EFCC 1 1 40 00001234" && outOfPlayer.onRef == 0x14);   // player.RemoveAllTypedItems container 1 1 40 holdout
    typed.into = true;
    typed.holdout.clear();
    const auto intoPlayer = nvcoop::coop::stashLine(parse(nvcoop::coop::stashOrderJson(typed)), 0xEFCC, 0);
    CHECK(intoPlayer.text == "RemoveAllTypedItems 00000014 1 1 40" && intoPlayer.onRef == 0xEFCC);          // container.RemoveAllTypedItems player 1 1 40
    const auto all = nvcoop::coop::stashLine(p, 0xEFCC, 0);
    CHECK(all.text == "RemoveAllItems 0000EFCC 1" && all.onRef == 0x14);                       // the untyped order is the old one
    // no container (the items are destroyed): ordered to the guest's machine, the typed strip keeps its type, noMsg and holdout
    nvcoop::coop::StashOrder none;
    none.type = 40;
    none.noMsg = true;
    none.holdout = "deadmoney.esm:001234";
    const auto noBox = nvcoop::coop::stashLine(parse(nvcoop::coop::stashOrderJson(none)), 0, 0x1234);
    CHECK(noBox.text == "RemoveAllTypedItems 0 0 1 40 00001234" && noBox.onRef == 0x14);
    none.holdout.clear();
    CHECK(nvcoop::coop::stashLine(parse(nvcoop::coop::stashOrderJson(none)), 0, 0).text == "RemoveAllTypedItems 0 0 1 40");
    nvcoop::coop::StashOrder noBoxAll;
    CHECK(nvcoop::coop::stashLine(noBoxAll, 0, 0).text == "RemoveAllItems");
}

int main(int argc, char** argv) {
    if (argc > 0) { g_outDir = argv[0]; size_t sl = g_outDir.find_last_of("\\/"); g_outDir = sl == std::string::npos ? "." : g_outDir.substr(0, sl); }
    const bool uiOnly = argc > 1 && std::string(argv[1]) == "--ui";   // mutation-twin runs: only the wiring tests (COOP_UI_MUTANT=n must break them)
    if (argc > 1 && std::string(argv[1]) == "--leave") {   // mutation-twin runs for leave()/join() (COOP_LEAVE_MUTANT=n must break them)
        testLeaveJoin();
        std::printf("%d checks, %d failed\n", g_checks, g_failed);
        return g_failed ? 1 : 0;
    }
    if (argc > 1 && std::string(argv[1]) == "--wiring") {   // wave 2 twins (FNVCOOP_MUT=9..): only the session-wiring tests
        testResultScope();
        testEphemeralWorldEffect();
        testPlayerOrders();
        testPartyState();
        testSaveSidecars();
        testCompanionCapsAndSlots();
        testCauseQuestsInSession();
        testActingCopies();
        testAnchorTether();
        testChargenPasses();
        testReadyCheckInfo();
        testHitGate();
        testDeathAndLateJoin();
        testTetherPolicyAndWorldspace();
        testGatherAndWorldReport();
        testTetherHoldAndWarpAreAnnounced();
        testLateJoinHoldAndWarpAreLogged();
        testRosterAndReconcile();
        testControlsGuestOnly();
        testCompanionViewsAroundScripts();
        testW3dOptionsAndPerSlotScripts();
        testStashOrderCarriesTheType();
        testObjVarSidecar();
        testFrameOrder();
        testRulesViewsAndLink();
        testGuestRewardLandsOnce(2, "enet");
        testGuestRewardLandsOnce(4, "enet");
        testGuestRewardRouting(2);
        testGuestRewardRouting(4);
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
    testGuestChargenFlow();
    testTransport();
    testConvergence("enet");
    testConvergence("udp");
    testActions();
    testDialogue();
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
    testResultScope();
    testEphemeralWorldEffect();
    testPlayerOrders();
    testPartyState();
    testSaveSidecars();
    testCompanionCapsAndSlots();
    testCauseQuestsInSession();
    testActingCopies();
    testAnchorTether();
    testChargenPasses();
    testReadyCheckInfo();
    testHitGate();
    testDeathAndLateJoin();
    testTetherPolicyAndWorldspace();
    testGatherAndWorldReport();
    testTetherHoldAndWarpAreAnnounced();
    testLateJoinHoldAndWarpAreLogged();
    testRosterAndReconcile();
    testControlsGuestOnly();
    testCompanionViewsAroundScripts();
    testW3dOptionsAndPerSlotScripts();
    testStashOrderCarriesTheType();
    testObjVarSidecar();
    testFrameOrder();
    testRulesViewsAndLink();
    testLeaveJoin();
    testRestartedClientRejoin();
    testLossyHost();
    testGuestRewardLandsOnce(2, "enet");
    testGuestRewardLandsOnce(4, "enet");
    testGuestRewardLandsOnce(4, "udp");
    testGuestRewardRouting(2);
    testGuestRewardRouting(4);
    std::printf("%d checks, %d failed\n", g_checks, g_failed);
    return g_failed ? 1 : 0;
}
