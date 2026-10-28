// IntentService on the REAL coop Runtime: a host and two guests over ENet loopback, the fake world of plugin/coop/tests, and a FakeEngine per machine.
// What this proves that the loopback tests cannot: the mux channel really carries the datagrams, Runtime::runAsGuest really forces the txn origin,
// the story-channel executor really goes through the intents host, rewards really reach the right player, and a leaver's locks really go.
#include <chrono>
#include <thread>

#include "fake_world.h"
#include "host_rig.h"
#include "intents/intent_service.h"
#include "minitest.h"

using namespace nvcoop::intents;
using namespace nvcoop::intents::testing;
using nvcoop::coop::Role;
using nvcoop::coop::Runtime;
using nvcoop::coop::test::FakeWorld;

namespace {

uint64_t wallMs() {
    return std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now().time_since_epoch()).count();
}

bool pumpUntil(const std::function<void()>& frame, const std::function<bool()>& done, int timeoutMs = 6000) {
    uint64_t end = wallMs() + timeoutMs;
    while (wallMs() < end) {
        frame();
        if (done()) return true;
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
    return done();
}

std::string S(const FormRef& r) { return nvhooks::refToStr(r); }

struct Node {
    nvhooks::CaptureHub hub;
    FakeWorld world;
    FakeEngine eng;
    std::unique_ptr<Runtime> rt;
    std::unique_ptr<IntentService> svc;
    bool menuOpen = false;
    nvcoop::coop::Config rtCfg;
    PlayerId slotOverride = kNoPlayer;
    std::vector<std::pair<int, bool>> acting;   // the acting hook as the engine layer would see it

    int stubRuns = 0;                                   // coop_module's stub runner (returns 0): must NOT be the one that runs
    std::vector<fnvcoop_dlg::Event> dlgEvents;

    Node(Role role, uint16_t port, uint64_t token, const nvcoop::intents::Config& icfg, bool dialogue = false) {
        world.seed(0);
        rtCfg.enable = true;
        rtCfg.role = role;
        rtCfg.port = port;
        rtCfg.token = token;
        rtCfg.transport = "enet";
        rt.reset(new Runtime(rtCfg, &hub, &world, wallMs));
        rt->setDumper(&world);
        // the v1 executor of coop_module is what IntentService takes over; this stand-in must NOT run when the service is up
        rt->setActionExecutor([this](PlayerId, fnvcoop::IntentKind, const FormRef&) { ++legacyV1Runs; });
        if (dialogue) {
            nvcoop::coop::DialogueHooks h;
            h.run = [this](const fnvcoop_dlg::ResultRequest&) -> uint64_t { ++stubRuns; return 0; };
            h.sink = [this](const fnvcoop_dlg::Event& e) { dlgEvents.push_back(e); };
            rt->enableDialogue(h);
        }
        rt->setActingHook([this](PlayerId s, bool on) { acting.emplace_back(int(s), on); });
        eng.hub = &hub;
        eng.onCall = [this](const FakeEngine::Call& c) {
            if (c.what == "activate") {
                world.hostMutate(hub, fnvcoop::EnableRef{c.target});
                hub.record(fnvcoop::AddItem{R(0xF), 5, fnvcoop::Recipient::origin()});
            } else if (c.what == "disable") {
                world.hostMutate(hub, fnvcoop::DisableRef{c.target});
            } else if (c.what == "infoResult") {
                world.hostMutate(hub, fnvcoop::SetGlobal{R(0x500 + c.arg), 1.0f});     // the INFO's result script sets a global ...
                hub.record(fnvcoop::AddItem{R(0xF), 7, fnvcoop::Recipient::origin()}); // ... and pays the player
            }
        };
        ServiceDeps d;
        d.eng = &eng;
        d.nowMs = wallMs;
        d.puppetOf = [this](PlayerId s) { return s == 0 ? RefId(0) : RefId(0x1000 + s); };
        d.containerMenuOpen = [this] { return menuOpen; };
        svc.reset(new IntentService(std::move(d), icfg));
        std::string err;
        if (!rt->start(&err)) std::printf("  start failed: %s\n", err.c_str());
    }
    void frame() {
        rt->frame();
        svc->frame(rt.get());
    }
    int legacyV1Runs = 0;
};

nvcoop::intents::Config icfg() {
    nvcoop::intents::Config c = testConfig();
    c.coalesceMs = 0;
    return c;
}

void populate(FakeEngine& e) {
    e.world[R(0xABC)] = mkTarget(TargetClass::Door, 0xABC, 100, 0, 0);
    TargetInfo item = mkTarget(TargetClass::Item, 0x700, 100, 0, 0);
    item.base = R(0x77);
    item.count = 3;
    e.world[R(0x700)] = item;
    e.world[R(0x501)] = mkTarget(TargetClass::Container, 0x501, 100, 0, 0);
    e.positions[0x1001] = {90, 0, 0};
    e.positions[0x1002] = {95, 0, 0};
}

struct Trio {
    Node host;
    Node a, b;
    std::function<void()> fr;
    explicit Trio(bool dialogue = false)
        : host(Role::Host, 0, 0, icfg(), dialogue), a(Role::Client, host.rt->boundPort(), 3001, icfg(), dialogue), b(Role::Client, host.rt->boundPort(), 3002, icfg(), dialogue) {
        populate(host.eng);
        populate(a.eng);
        populate(b.eng);
        fr = [this] { host.frame(); a.frame(); b.frame(); };
    }
    bool joined() { return pumpUntil(fr, [&] { return a.rt->client()->joined() && b.rt->client()->joined() && host.svc->host() && a.svc->client() && b.svc->client(); }); }
    bool synced() { return pumpUntil(fr, [&] { return host.rt->host()->allSynced(); }); }
};

}  // namespace

TEST(intents_service, activate_runs_on_the_host_as_the_puppet_and_rewards_only_the_requester) {
    Trio t;
    REQUIRE(t.joined());
    const PlayerId sa = t.a.rt->client()->slot();
    t.host.acting.clear();
    CHECK(t.a.svc->onActivate(R(0xABC)) == Obs::Proceed);       // door: the local animation goes on
    CHECK(pumpUntil(t.fr, [&] { return t.a.rt->client()->applied() >= 1 && t.b.rt->client()->applied() >= 1 && t.host.rt->host()->allSynced(); }));
    REQUIRE(t.host.eng.calls.size() == 1);
    CHECK(t.host.eng.calls[0].what == "activate" && t.host.eng.calls[0].actor == RefId(0x1000 + sa) && t.host.eng.calls[0].contextActor == RefId(0x1000 + sa));
    CHECK(t.host.eng.outsideScope == 0);
    CHECK(t.host.legacyV1Runs == 0);
    const auto* txn = t.host.rt->host()->log().get(1);
    REQUIRE(txn != nullptr);
    CHECK(txn->origin == sa && txn->cause.kind == fnvcoop::CauseKind::Intent && txn->cause.source == R(0xABC));      // the guest is the origin
    CHECK(t.host.world.enabled.at(S(R(0xABC))) && t.a.world.enabled.at(S(R(0xABC))) && t.b.world.enabled.at(S(R(0xABC))));
    CHECK(t.a.world.inventory.at(S(R(0xF))) == 5);                                                                 // the requester only
    CHECK(t.b.world.inventory.count(S(R(0xF))) == 0 && t.host.world.inventory.count(S(R(0xF))) == 0);
    CHECK(t.host.acting.size() == 2 && t.host.acting[0] == std::make_pair(int(sa), true) && t.host.acting[1] == std::make_pair(int(sa), false));   // one window
    CHECK(pumpUntil(t.fr, [&] { return t.a.svc->client()->stats().verdicts == 1; }));
    CHECK(t.host.svc->host()->stats().executed == 1 && t.host.rt->actingSlot() == fnvcoop::kNoPlayer);
}

TEST(intents_service, guest_stage_request_goes_through_the_real_runtime_and_both_copies_agree) {
    // the lab's quest room on the real Runtime: host at stage 10, guest A's console setstage 20 was dropped by the suppression and forwarded as Kind::SetStage
    Trio t;
    REQUIRE(t.joined());
    const PlayerId sa = t.a.rt->client()->slot();
    const FormRef q = R(0x1000);
    t.host.eng.quests[q] = 10;
    CHECK(t.a.svc->onStageBlocked(q, 20));
    CHECK(pumpUntil(t.fr, [&] { return t.a.rt->client()->applied() >= 1 && t.b.rt->client()->applied() >= 1 && t.host.rt->host()->allSynced(); }));
    CHECK(t.host.eng.quests[q] == 20 && t.host.eng.count("setStage") == 1 && t.host.eng.calls.back().contextActor == RefId(0x1000 + sa));
    const auto* txn = t.host.rt->host()->log().get(1);
    REQUIRE(txn != nullptr);
    CHECK(txn->origin == sa && txn->cause.kind == fnvcoop::CauseKind::Intent);
    CHECK(t.a.world.stages.at(S(q)) == 20 && t.b.world.stages.at(S(q)) == 20);                // every copy ends at the same stage, the requester's too
    CHECK(pumpUntil(t.fr, [&] { return t.a.svc->client()->stats().verdicts == 1; }));
    CHECK(t.host.svc->host()->stats().executed == 1 && t.host.svc->host()->stats().stagesApplied == 1);
    // the same request again (the host's copy of the script is already there): refused, nothing runs, nothing new is announced
    t.host.eng.calls.clear();
    CHECK(t.b.svc->onStageBlocked(q, 20));
    CHECK(pumpUntil(t.fr, [&] { return t.b.svc->client()->stats().verdicts == 1; }));
    CHECK(t.host.eng.calls.empty() && t.host.svc->host()->stats().stagesNoAdvance == 1 && t.host.rt->host()->log().head() == 1);
}

TEST(intents_service, story_channel_intents_get_the_same_treatment) {
    Trio t;
    REQUIRE(t.joined());
    const PlayerId sa = t.a.rt->client()->slot();
    t.host.acting.clear();
    CHECK(t.a.rt->submitAction(fnvcoop::IntentKind::ActivateRef, R(0xABC)) != 0);        // what coop_module's forward_activate observer sends
    CHECK(pumpUntil(t.fr, [&] { return t.a.rt->client()->applied() >= 1 && t.host.rt->host()->allSynced(); }));
    REQUIRE(t.host.eng.calls.size() == 1);
    CHECK(t.host.eng.calls[0].actor == RefId(0x1000 + sa));                               // the puppet, not the v1 stand-in 0x14
    CHECK(t.host.legacyV1Runs == 0 && t.host.svc->host()->stats().legacy == 1);
    CHECK(t.host.acting.size() == 2);                                                      // the nested runAsGuest did not open a second window
    CHECK(t.a.world.inventory.at(S(R(0xF))) == 5 && t.b.world.inventory.count(S(R(0xF))) == 0);
}

TEST(intents_service, pickup_race_has_one_winner_everywhere) {
    Trio t;
    REQUIRE(t.joined());
    CHECK(t.a.svc->onActivate(R(0x700)) == Obs::Suppress && t.b.svc->onActivate(R(0x700)) == Obs::Suppress);
    CHECK(pumpUntil(t.fr, [&] { return t.a.svc->client()->stats().verdicts == 1 && t.b.svc->client()->stats().verdicts == 1; }));
    CHECK(t.synced());
    int grants = int(t.a.world.inventory.count(S(R(0x77)))) + int(t.b.world.inventory.count(S(R(0x77))));
    CHECK(grants == 1);                                                                  // exactly one of them got it
    Node& winner = t.a.world.inventory.count(S(R(0x77))) ? t.a : t.b;
    Node& loser = &winner == &t.a ? t.b : t.a;
    CHECK(winner.world.inventory.at(S(R(0x77))) == 3 && t.host.world.inventory.count(S(R(0x77))) == 0);   // 3 of them, not on the host
    CHECK(!t.host.world.enabled.at(S(R(0x700))) && !t.a.world.enabled.at(S(R(0x700))) && !t.b.world.enabled.at(S(R(0x700))));   // gone for everybody
    CHECK(loser.eng.uiHas("notify:Somebody else got there first."));
    CHECK(t.host.svc->host()->stats().claimsWon == 1 && t.host.svc->host()->stats().claimsLost == 1);
}

TEST(intents_service, container_in_use_lock_follows_the_menu_and_the_player) {
    Trio t;
    REQUIRE(t.joined());
    t.a.svc->onActivate(R(0x501));
    CHECK(pumpUntil(t.fr, [&] { return t.host.svc->host()->locks().size() == 1 && t.a.svc->client()->stats().verdicts == 1; }));
    t.b.svc->onActivate(R(0x501));
    CHECK(pumpUntil(t.fr, [&] { return t.b.eng.uiHas("closeMenus"); }));
    CHECK(t.b.eng.uiHas("notify:Player"));
    t.a.menuOpen = true;
    CHECK(pumpUntil(t.fr, [&] { return t.a.svc->client()->openContainer() != nullptr; }));
    t.a.menuOpen = false;                                                               // the player closed the loot menu
    CHECK(pumpUntil(t.fr, [&] { return t.host.svc->host()->locks().empty(); }));
    t.b.svc->onActivate(R(0x501));                                                      // now b gets it
    CHECK(pumpUntil(t.fr, [&] { return t.host.svc->host()->locks().size() == 1 && t.host.svc->host()->locks()[0].holder == t.b.rt->client()->slot(); }));
    // a leaver's lock goes with them
    t.b.rt->stop();
    CHECK(pumpUntil(t.fr, [&] { return t.host.svc->host()->locks().empty(); }));
}

TEST(intents_service, new_session_rebuilds_and_old_handlers_stay_inert) {
    Trio t;
    REQUIRE(t.joined());
    uint64_t before = t.host.svc->sessions();
    CHECK(before >= 1 && t.a.svc->sessions() >= 1);
    t.host.svc->frame(nullptr);                                                          // no runtime: the session part ends
    CHECK(!t.host.svc->sessionUp() && t.host.svc->host() == nullptr);
    t.host.svc->frame(t.host.rt.get());                                                  // and begins again on the same mux
    CHECK(t.host.svc->sessionUp() && t.host.svc->sessions() == before + 1);
    // the guest still reaches the NEW host object (the channel was re-registered)
    t.a.svc->onActivate(R(0xABC));
    CHECK(pumpUntil(t.fr, [&] { return t.host.svc->host() && t.host.svc->host()->stats().executed == 1; }));
}

TEST(intents_service, runtime_active_and_run_as_guest) {
    Runtime* alive = nullptr;
    {
        Trio t;
        REQUIRE(t.joined());
        CHECK(Runtime::active() != nullptr);
        alive = Runtime::active();
        int ran = 0;
        CHECK(t.a.rt->runAsGuest(1, [&] { ++ran; }) == false && ran == 0);              // a guest has no host session: the body never runs
        CHECK(t.host.rt->runAsGuest(fnvcoop::kNoPlayer, [&] { ++ran; }) == false && ran == 0);
        PlayerId inside = fnvcoop::kNoPlayer;
        CHECK(t.host.rt->runAsGuest(2, [&] {
            inside = t.host.rt->actingSlot();
            t.host.rt->runAsGuest(2, [&] { ++ran; });                                     // nested, same slot: no second window
        }) && inside == 2 && ran == 1);
        CHECK(t.host.rt->actingSlot() == fnvcoop::kNoPlayer);
        CHECK(t.host.rt->runAsGuest(3, [&] { try { throw 1; } catch (int) {} }));
        bool threw = false;
        try { t.host.rt->runAsGuest(2, [&] { throw 7; }); } catch (int) { threw = true; }
        CHECK(threw && t.host.rt->actingSlot() == fnvcoop::kNoPlayer);                    // the window is closed even then
        t.a.rt->stop();
        t.b.rt->stop();
        t.host.rt->stop();
        CHECK(Runtime::active() != alive);                                                // stop() clears it
    }
}

TEST(intents_service, save_suppress_is_enforced_from_the_session_flag) {
    Trio t;
    REQUIRE(t.joined());
    SaveGate ga(SaveGate::Deps{[&] { return saveSuppressedBy(t.a.rt.get()); }, nullptr, wallMs});
    SaveGate gh(SaveGate::Deps{[&] { return saveSuppressedBy(t.host.rt.get()); }, nullptr, wallMs});
    CHECK(ga.allow(true) && gh.allow(true));
    fnvcoop::Cause c;
    c.kind = fnvcoop::CauseKind::QuestResult;
    c.source = R(0x104C1C);
    t.host.rt->host()->commit(0, c, std::vector<fnvcoop::Effect>{fnvcoop::SaveSuppress{true}});
    CHECK(pumpUntil(t.fr, [&] { return t.a.rt->saveSuppressed() && t.b.rt->saveSuppressed(); }));
    CHECK(!ga.allow(true) && !gh.allow(true));                                          // guests AND the host are held back
    CHECK(!saveSuppressedBy(nullptr));
    t.host.rt->host()->commit(0, c, std::vector<fnvcoop::Effect>{fnvcoop::SaveSuppress{false}});
    CHECK(pumpUntil(t.fr, [&] { return !t.a.rt->saveSuppressed(); }));
    CHECK(ga.allow(true) && gh.allow(true));
}

TEST(intents_service, dialogue_result_scripts_of_a_guest_conversation_run_on_the_host_as_that_guest) {
    Trio t(true);
    REQUIRE(t.joined());
    REQUIRE(pumpUntil(t.fr, [&] { return t.a.rt->dialogue() && t.b.rt->dialogue() && t.host.rt->host()->clients().size() == 2; }));
    const PlayerId sa = t.a.rt->client()->slot();
    t.a.rt->dialogue()->requestStart(R(0x104C0F), (uint32_t)wallMs());
    CHECK(pumpUntil(t.fr, [&] { return !t.a.dlgEvents.empty(); }));
    t.a.rt->dialogue()->pick(R(0x105560), R(0x105583), 0, false, {fnvcoop_dlg::Line{"House.", "", 0, 50}}, {}, (uint32_t)wallMs());
    t.a.rt->dialogue()->lineDone((uint32_t)wallMs());
    CHECK(pumpUntil(t.fr, [&] { return t.host.svc->dialogueStats().ran == 2 && t.a.rt->client()->applied() >= 2 && t.host.rt->host()->allSynced(); }));
    CHECK(t.host.stubRuns == 0);                                           // coop_module's stub was replaced: Begin and End ran for real
    int infoCalls = 0;
    for (const auto& c : t.host.eng.calls)
        if (c.what == "infoResult") {
            ++infoCalls;
            CHECK(c.actor == RefId(0x1000 + sa) && c.contextActor == RefId(0x1000 + sa) && c.contextSlot == sa && c.target == R(0x104C0F));   // the guest's puppet is `player`
        }
    CHECK(infoCalls == 2 && t.host.eng.outsideScope == 0);
    const auto* first = t.host.rt->host()->log().get(1);
    REQUIRE(first != nullptr);
    CHECK(first->origin == sa && first->cause.kind == fnvcoop::CauseKind::DialogueResult && first->cause.source == R(0x105583));   // the initiator pays and is paid
    CHECK(t.a.world.inventory.at(S(R(0xF))) == 14 && t.b.world.inventory.count(S(R(0xF))) == 0 && t.host.world.inventory.count(S(R(0xF))) == 0);
    CHECK(t.host.world.globals.at(S(R(0x500))) == 1.0f && t.a.world.globals.at(S(R(0x500))) == 1.0f && t.b.world.globals.at(S(R(0x500))) == 1.0f);   // Begin
    CHECK(t.host.world.globals.at(S(R(0x501))) == 1.0f && t.b.world.globals.at(S(R(0x501))) == 1.0f);                                                   // End
    CHECK(t.host.svc->dialogueStats().committed == 2 && t.host.svc->dialogueStats().refused == 0);
    // no hook installed in the engine (supported = false): nothing runs, the story log does not move, the conversation is not stranded
    t.host.eng.infoSupported = false;
    uint64_t head = t.host.rt->host()->log().head();
    t.a.rt->dialogue()->pick(R(0x105560), R(0x105584), 0, true, {fnvcoop_dlg::Line{"Bye.", "", 0, 50}}, {}, (uint32_t)wallMs());
    t.a.rt->dialogue()->lineDone((uint32_t)wallMs());
    CHECK(pumpUntil(t.fr, [&] { return t.host.svc->dialogueStats().ran == 4; }));
    CHECK(t.host.svc->dialogueStats().unsupported == 2 && t.host.rt->host()->log().head() == head);
}

// Story run S06: a guest's pick ("Sure, I'll come with you", 60 rounds of ammo) ran its result script four times on the host. One pick pays once: the
// runner is keyed by (session, conversation, step, phase), the dialogue host answers a resent Pick without a run, and the same INFO again with no list
// offered in between is a repeat, not a second pick.
TEST(intents_service, a_guests_pick_pays_once_through_retries_resends_and_a_repeat) {
    Trio t(true);
    REQUIRE(t.joined());
    REQUIRE(pumpUntil(t.fr, [&] { return t.a.rt->dialogue() && t.b.rt->dialogue() && t.host.rt->host()->clients().size() == 2; }));
    const PlayerId sa = t.a.rt->client()->slot();
    t.a.rt->dialogue()->requestStart(R(0x104C0F), (uint32_t)wallMs());
    CHECK(pumpUntil(t.fr, [&] { return !t.a.dlgEvents.empty(); }));
    t.a.rt->dialogue()->pick(R(0x105560), R(0x105583), 0, false, {fnvcoop_dlg::Line{"House.", "", 0, 50}}, {}, (uint32_t)wallMs());
    const uint32_t conv = t.a.rt->dialogue()->client().conv(), step = t.a.rt->dialogue()->client().step();
    // the same Pick on the wire three more times (a retry, a resend after a reconnect)
    fnvcoop_dlg::Pick again;
    again.conv = conv; again.step = step; again.topic = R(0x105560); again.info = R(0x105583);
    for (int i = 0; i < 3; ++i) t.a.rt->mux()->send(0, fnvcoop_dlg::encode(fnvcoop_dlg::Msg(again)));
    t.a.rt->dialogue()->lineDone((uint32_t)wallMs());
    for (int i = 0; i < 3; ++i) t.a.rt->mux()->send(0, fnvcoop_dlg::encode(fnvcoop_dlg::Msg(fnvcoop_dlg::LineDone{conv, step})));
    CHECK(pumpUntil(t.fr, [&] { return t.host.svc->dialogueStats().ran == 2 && t.a.rt->client()->applied() >= 2 && t.host.rt->host()->allSynced(); }));
    for (int i = 0; i < 200; ++i) { t.fr(); std::this_thread::sleep_for(std::chrono::milliseconds(1)); }
    CHECK(t.host.svc->dialogueStats().ran == 2 && t.host.svc->dialogueStats().committed == 2);
    CHECK(t.a.world.inventory.at(S(R(0xF))) == 14);                         // 7 for Begin + 7 for End, once
    // the runner itself, called again for the same (conversation, step, phase): answered from the first run, nothing runs
    fnvcoop_dlg::ResultRequest rq;
    rq.conv = conv; rq.step = step; rq.phase = fnvcoop_dlg::Phase::Begin; rq.initiator = sa; rq.npc = R(0x104C0F); rq.info = R(0x105583);
    const uint64_t head = t.host.rt->host()->log().head();
    for (int i = 0; i < 3; ++i) CHECK(t.host.svc->runDialogueResult(rq) != 0 || head == 0);
    rq.phase = fnvcoop_dlg::Phase::End;
    for (int i = 0; i < 3; ++i) CHECK(t.host.svc->runDialogueResult(rq) != 0 || head == 0);
    CHECK(t.host.svc->dialogueStats().ran == 2 && t.host.svc->dialogueStats().deduped == 6);
    CHECK(t.host.rt->host()->log().head() == head);
    int infoCalls = 0;
    for (const auto& c : t.host.eng.calls) infoCalls += c.what == "infoResult";
    CHECK(infoCalls == 2);
    // the engine shows the same INFO as picked again, no topic list in between: the host answers the step and runs nothing
    t.a.rt->dialogue()->pick(R(0x105560), R(0x105583), 0, false, {fnvcoop_dlg::Line{"House.", "", 0, 50}}, {}, (uint32_t)wallMs());
    t.a.rt->dialogue()->lineDone((uint32_t)wallMs());
    CHECK(pumpUntil(t.fr, [&] { return t.host.rt->host() && t.a.rt->dialogue()->client().topicsReady(); }));
    for (int i = 0; i < 200; ++i) { t.fr(); std::this_thread::sleep_for(std::chrono::milliseconds(1)); }
    CHECK(t.host.svc->dialogueStats().ran == 2);
    CHECK(t.a.world.inventory.at(S(R(0xF))) == 14 && t.b.world.inventory.count(S(R(0xF))) == 0 && t.host.world.inventory.count(S(R(0xF))) == 0);
}

TEST(intents_service, dialogue_runner_needs_a_puppet_or_a_stand_in_policy) {
    nvcoop::intents::Config c = icfg();
    c.requirePuppet = true;
    Node host(Role::Host, 0, 0, c, true);
    Node a(Role::Client, host.rt->boundPort(), 3101, c, true);
    populate(host.eng);
    host.svc.reset();   // rebuilt below with no puppet at all
    ServiceDeps d;
    d.eng = &host.eng;
    d.nowMs = wallMs;
    d.puppetOf = [](PlayerId) { return RefId(0); };
    host.svc.reset(new IntentService(std::move(d), c));
    auto fr = [&] { host.frame(); a.frame(); };
    REQUIRE(pumpUntil(fr, [&] { return a.rt->client()->joined() && a.rt->dialogue() && host.svc->host() && host.rt->host()->clients().size() == 1; }));
    a.rt->dialogue()->requestStart(R(0x104C0F), (uint32_t)wallMs());
    pumpUntil(fr, [&] { return !a.dlgEvents.empty(); });
    a.rt->dialogue()->pick(R(0x105560), R(0x105583), 0, false, {fnvcoop_dlg::Line{"House.", "", 0, 50}}, {}, (uint32_t)wallMs());
    CHECK(pumpUntil(fr, [&] { return host.svc->dialogueStats().refused >= 1; }));
    CHECK(host.eng.count("infoResult") == 0 && host.stubRuns == 0);                  // no puppet and no stand-in allowed: the script does not run for a stranger
}

TEST(intents_service, a_guest_conversations_result_scripts_see_that_guests_stash) {
    // The Fort's "give me my gear back" lines are INFO result scripts: they run on the host as the guest and must read that guest's stash of the shared
    // storage container, not the host's gear and not the other guest's.
    const FormRef store = R(0x139B6B);
    Trio t(true);
    StorageBook b;
    std::string err;
    REQUIRE(b.load(R"({"stashes":[{"target":"falloutnv.esm:139B6B"}]})", &err));
    ServiceDeps d;
    d.eng = &t.host.eng;
    d.nowMs = wallMs;
    d.puppetOf = [](PlayerId s) { return s == 0 ? RefId(0) : RefId(0x1000 + s); };
    t.host.svc.reset(new IntentService(std::move(d), icfg(), ContainerRedirect(), std::move(b)));   // rebuilt before anybody joined
    REQUIRE(t.joined());
    REQUIRE(pumpUntil(t.fr, [&] { return t.a.rt->dialogue() && t.b.rt->dialogue() && t.host.rt->host()->clients().size() == 2; }));
    const PlayerId sa = t.a.rt->client()->slot(), sb = t.b.rt->client()->slot();
    t.host.svc->stash().put(store, sa, {{R(0xB01), 2}});
    t.host.svc->stash().put(store, sb, {{R(0xB02), 9}});
    t.host.eng.containers[store] = {{R(0xAAA), 1}};
    std::vector<std::vector<StoredItem>> seen;
    auto prev = t.host.eng.onCall;
    t.host.eng.onCall = [&, prev](const FakeEngine::Call& c) {
        if (c.what == "infoResult") seen.push_back(t.host.eng.containers[store]);
        prev(c);
    };
    t.a.rt->dialogue()->requestStart(R(0x104C0F), (uint32_t)wallMs());
    CHECK(pumpUntil(t.fr, [&] { return !t.a.dlgEvents.empty(); }));
    t.a.rt->dialogue()->pick(R(0x105560), R(0x105583), 0, false, {fnvcoop_dlg::Line{"House.", "", 0, 50}}, {}, (uint32_t)wallMs());
    t.a.rt->dialogue()->lineDone((uint32_t)wallMs());
    CHECK(pumpUntil(t.fr, [&] { return t.host.svc->dialogueStats().ran == 2; }));
    REQUIRE(seen.size() == 2);
    for (const auto& s : seen) CHECK(s == std::vector<StoredItem>({{R(0xB01), 2}}));   // Begin and End read guest A's gear: not the host's, not guest B's
    CHECK(t.host.eng.containers[store] == std::vector<StoredItem>({{R(0xAAA), 1}}));  // the host's container is back as it was
    REQUIRE(t.host.svc->stash().peek(store, sb) != nullptr);
    CHECK(*t.host.svc->stash().peek(store, sb) == std::vector<StoredItem>({{R(0xB02), 9}}));
}
