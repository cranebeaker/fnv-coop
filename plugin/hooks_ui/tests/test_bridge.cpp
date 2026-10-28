// Game-free tests for the production wiring: UiRouter + makeDialogueBridge + L8 DialogueNode over L4's InMemoryNetwork, and the ending
// slideshow channel (SlideHost / SlideClient). Built once (FNVCOOP_RUNTIME_MUT); run once for real and once per twin (FNVCOOP_MUT=10..20, all must fail):
//   10 client never unlocks at End   11 ending slide does not lock controls   12 slide swap not played   13 duplicate slides are replayed
//   14 late joiner gets no replay    15 consent table ignored (plain box)     16 slide host not connected to the router
//   17 narration stamp ignored       18 sequencer replays a released item      19 a lost predecessor blocks the show forever
//   20 scene narration not stamped by the host
//   26 a rules-owned message box is raised plain  27 the rules engine's per-player quest variables are not connected
#include <cstdio>
#include <memory>
#include <set>
#include <sstream>
#include <string>
#include <vector>

#include "../showorder.h"
#include "../slideshow.h"
#include "../ui_bridge.h"
#include "../voice_token.h"
#include "fnvcoop/transport.h"

using namespace nvui;
using fnvcoop_dlg::DialogueNode;
using fnvcoop_dlg::Event;

static int g_fail = 0, g_checks = 0;
#define CHECK(c)                                                       \
    do {                                                               \
        ++g_checks;                                                    \
        if (!(c)) {                                                    \
            ++g_fail;                                                  \
            std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c); \
        }                                                              \
    } while (0)
#define REQUIRE(c)        \
    do {                  \
        const bool ok_ = bool(c); \
        CHECK(ok_);       \
        if (!ok_) return; \
    } while (0)
#define TEST(name) static void name()
static void run(const char* n, void (*f)()) {
    int before = g_fail;
    f();
    std::printf("[%s] %s\n", g_fail == before ? " ok " : "FAIL", n);
}

static FormRef F(uint32_t id) { return FormRef("falloutnv.esm", id); }

struct FakeEngine : IUiEngine {
    uint64_t now = 1000;
    std::vector<std::pair<uint32_t, int32_t>> writes;
    uint64_t nowMs() override { return now; }
    void setMessageBoxResult(uint32_t s, int32_t b) override { writes.emplace_back(s, b); }
    bool refOf(uint32_t id, FormRef& out) override {
        if (!id) return false;
        out = F(id);
        return true;
    }
    void log(const std::string&) override {}
    int32_t lastButton() const { return writes.empty() ? -2 : writes.back().second; }
};

static const char* kConsentFile =
    "# test table\n"
    "FalloutNV.esm:161E8C VEndGameWarningMsg confirm=1 decline=0 buttons=I'm not ready yet.|Let's do this!\n"
    "FalloutNV.esm:002000 INFO_002000 confirm=0 decline=-1 mode=vote kind=info buttons=\n";   // a fork INFO: held until the party voted
static const uint32_t kEndGameMsg = 0x161E8C;

// Host (slot 0) + clients 1 and 2 over the in-memory network. Slideshow packets ride the same endpoints (first byte 0xE5).
struct World {
    fnvcoop::InMemoryNetwork net;
    FakeEngine eng;
    std::vector<std::unique_ptr<DialogueNode>> n;
    std::vector<std::vector<Event>> ev;
    std::unique_ptr<UiRouter> router;
    std::unique_ptr<SlideHost> slideHost;
    struct ClientRig {
        ShowSequencer seq;                      // shared by the slide client and the narration gate, like in ui_hooks.cpp
        std::unique_ptr<SlideClient> sc;
        std::unique_ptr<NarrationGate> gate;
        std::vector<std::string> swaps;
        std::vector<std::string> shown;         // "S:<texture>" / "N:<text>" in the order the player sees them
        std::vector<bool> lockLog;
        size_t evDone = 0;                      // dialogue events already fed to the gate
    };
    ClientRig cr[3];
    uint64_t now = 0;
    uint32_t seqProvided = 0;
    int scriptRuns = 0;                         // INFO result scripts the host ran
    // wave 1b: a stand-in rules engine (the real one is covered in plugin/dialogue rules_link_test.cpp)
    std::vector<std::tuple<PlayerId, FormRef, bool>> votes;
    std::vector<std::pair<FormRef, PlayerId>> boxAsked;

    explicit World(uint32_t latency = 20, double dup = 0, uint32_t jitter = 0, uint64_t seed = 5) : net(seed) {
        fnvcoop::LinkParams lp;
        lp.minLatencyMs = latency;
        lp.maxLatencyMs = latency + jitter;
        lp.duplicate = dup;
        net.setLink(lp);
        ev.resize(3);
        for (PlayerId i = 0; i < 3; ++i) {
            DialogueNode::Config c;
            c.self = i;
            c.isHost = (i == 0);
            c.hostPeer = 0;
            c.peerOfSlot = [](PlayerId p) { return fnvcoop::PeerId(p); };
            c.slotOfPeer = [](fnvcoop::PeerId p) { return PlayerId(p); };
            fnvcoop_dlg::ResultRunner runr;
            if (i == 0) runr = [this](const fnvcoop_dlg::ResultRequest&) { ++scriptRuns; return uint64_t(1); };
            n.emplace_back(new DialogueNode(net.endpoint(i), c, runr, [this, i](const Event& e) { ev[i].push_back(e); }));
            cr[i].sc.reset(new SlideClient({[this, i](const FormRef&, const std::string&, const std::string& nw) {
                                                cr[i].swaps.push_back(nw);
                                                cr[i].shown.push_back("S:" + nw);
                                                return true;
                                            },
                                            [this, i](bool l) { cr[i].lockLog.push_back(l); }, nullptr},
                                           &cr[i].seq));
            NarrationGate::Sink sink;
            sink.subtitle = [this, i](const FormRef&, const std::string& text, float) { cr[i].shown.push_back("N:" + text); };
            sink.voice = [this, i](const FormRef&, const std::string& tok) { cr[i].shown.push_back("V:" + tok); };
            cr[i].gate.reset(new NarrationGate(cr[i].seq, sink));
        }
        slideHost.reset(new SlideHost([this](PlayerId to, std::vector<uint8_t> b) { net.endpoint(0)->send(fnvcoop::PeerId(to), std::move(b)); }));

        DialogueBridgeConfig bc;
        bc.node = n[0].get();
        bc.nowMs = [this] { return uint32_t(now); };
        std::istringstream in(kConsentFile);
        bc.consent = fnvcoop_dlg::loadConsentMessages(in);
        bc.slotOfActor = [](uint32_t ref) -> PlayerId { return ref == 0x14 ? kHost : ref == 0x2001 ? PlayerId(1) : ref == 0x2002 ? PlayerId(2) : kNoSlot; };
        bc.lastCommittedSeq = [this] { return uint64_t(++seqProvided); };
        bc.slides = slideHost.get();
        bc.isRulesChoice = [](const FormRef& m) { return m.localId == 0x4D0001; };                       // ChoiceConsent box (a Temple confirm box)
        bc.varName = [](const FormRef& q, uint32_t idx, std::string* name) { if (q.localId != 0x7700 || idx != 4) return false; *name = "fCollarTimer"; return true; };
        bc.viewPlayerVar = [](const FormRef&, const std::string& var, PlayerId slot, double real) { return var == "fCollarTimer" ? 10.5 + slot : real; };
        bc.hasPlayerVars = [](const FormRef& q) { return q.localId == 0x7700; };
        router.reset(new UiRouter(eng, makeDialogueBridge(bc)));
        router->setHost(true);
        attachMenuResume(*n[0], *router);
        n[0]->setRulesGates(nullptr, [this](const FormRef& box, PlayerId origin) { boxAsked.emplace_back(box, origin); return fnvcoop_dlg::HostDialogue::Gate::Hold; });
        n[0]->setVoteSink([this](PlayerId slot, const FormRef& ref, bool yes) { votes.emplace_back(slot, ref, yes); });

        n[0]->playerJoined(1);
        n[0]->playerJoined(2);
        slideHost->playerJoined(1, 0);
        slideHost->playerJoined(2, 0);
    }
    void step(uint64_t ms = 0) {
        for (uint64_t t = 0; t <= ms; t += 5) {
            now += (t ? 5 : 0);
            eng.now = 1000 + now;
            net.advance(now);
            for (PlayerId i = 0; i < 3; ++i) {
                fnvcoop::PeerId from;
                fnvcoop::Bytes b;
                while (net.endpoint(i)->poll(from, b)) {
                    if (cr[i].sc->onPacket(b, now)) continue;
                    n[i]->onPacket(from, b, uint32_t(now));
                }
                n[i]->tick(uint32_t(now));
                if (i != 0) feedNarration(i);  // the host's own engine speaks natively
                cr[i].sc->tick(now);
                cr[i].gate->tick(now);
            }
            slideHost->tick(now);
        }
    }
    // What L8's EngineGlue::onEvent does for ForcedSayShow: subtitle, then voice (when the line has one), for every line the player hears.
    void feedNarration(PlayerId p) {
        for (; cr[p].evDone < ev[p].size(); ++cr[p].evDone) {
            const Event& e = ev[p][cr[p].evDone];
            if (e.kind != Event::ForcedSayShow) continue;
            const auto& f = std::get<fnvcoop_dlg::ForcedLines>(e.msg);
            if (!(f.target == p || f.scene)) continue;
            for (const auto& ln : f.lines) {
                cr[p].gate->onSubtitle(f.npc, ln.text, 2.0f);
                if (!ln.voice.empty()) cr[p].gate->onVoice(f.npc, ln.voice, now);
            }
        }
    }
    // The narrator says one scene line on the host.
    void narrate(const std::string& text, uint32_t topic = 0x6000) {
        ForcedCall c;
        c.kind = ForcedCall::Kind::SayTo;
        c.speaker = 0x5000;
        c.target = kPlayerRefID;  // outside any scope: the party
        c.topic = F(topic);
        c.info = F(topic + 1);
        c.lines = {fnvcoop_dlg::Line{text, "", 0, 50}};
        router->onForced(c);
    }
    int count(PlayerId p, Event::Kind k) const {
        int c = 0;
        for (auto& e : ev[p]) c += e.kind == k;
        return c;
    }
    // distinct menus shown to p (the host re-sends an unanswered request, the client glue dedupes by id)
    size_t shown(PlayerId p) const {
        std::set<uint32_t> ids;
        for (auto& e : ev[p])
            if (e.kind == Event::MenuShow) ids.insert(std::get<fnvcoop_dlg::MenuRequest>(e.msg).menu);
        return ids.size();
    }
    uint32_t lastMenuId(PlayerId p) const {
        for (size_t i = ev[p].size(); i-- > 0;)
            if (ev[p][i].kind == Event::MenuShow) return std::get<fnvcoop_dlg::MenuRequest>(ev[p][i].msg).menu;
        return 0;
    }
};

static MessageBoxCall endgameBox(uint32_t msg, uint32_t script = 91) {
    MessageBoxCall c;
    c.scriptRefID = script;
    c.thisObj = 0x300;
    c.message = F(msg);
    c.text = "Are you ready for the battle of Hoover Dam?";
    c.buttons = {"I'm not ready yet.", "Let's do this!"};
    return c;
}

// ---- consent over the real node ------------------------------------------------------------------------------------------------
TEST(consent_waits_for_the_party_then_resumes_with_confirm) {
    World w;
    OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 2);
    REQUIRE(w.router->onMessageBox(endgameBox(kEndGameMsg)) == Route::Suppressed);
    w.step(100);
    REQUIRE(w.shown(2) == 1);
    const uint32_t id = w.lastMenuId(2);
    CHECK(w.eng.lastButton() == kNoButton);
    w.n[2]->menuReply(id, 1, uint32_t(w.now));  // "Let's do this!" is button 1 = the point of no return
    w.step(100);
    CHECK(w.shown(0) == 1 && w.shown(1) == 1);  // consent prompts on the other two
    CHECK(w.eng.lastButton() == kNoButton);                                       // script still waiting
    w.n[0]->menuReply(id, 0, uint32_t(w.now));
    w.step(100);
    CHECK(w.eng.lastButton() == kNoButton);  // one consent missing
    w.n[1]->menuReply(id, 0, uint32_t(w.now));
    w.step(100);
    CHECK(w.eng.lastButton() == 1);
    CHECK(w.router->pendingBoxes() == 0);
}
TEST(consent_declined_gives_the_not_ready_button) {
    World w;
    OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 1);
    REQUIRE(w.router->onMessageBox(endgameBox(kEndGameMsg)) == Route::Suppressed);
    w.step(100);
    const uint32_t id = w.lastMenuId(1);
    w.n[1]->menuReply(id, 1, uint32_t(w.now));
    w.step(100);
    w.n[2]->menuReply(id, 1, uint32_t(w.now));  // "Not yet"
    w.step(100);
    CHECK(w.eng.lastButton() == 0);  // declineChoice from the table
}
TEST(plain_box_has_no_consent_round) {
    World w;
    OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 2);
    REQUIRE(w.router->onMessageBox(endgameBox(0x123456)) == Route::Suppressed);
    w.step(100);
    w.n[2]->menuReply(w.lastMenuId(2), 1, uint32_t(w.now));
    w.step(100);
    CHECK(w.shown(0) == 0 && w.shown(1) == 0);
    CHECK(w.eng.lastButton() == 1);
}
// The host player decides the Hoover Dam start: the guests are still asked (the box used to stay a native host box, with no consent round at all).
TEST(consent_box_decided_by_the_host_asks_the_guests) {
    World w;
    const Route routed = w.router->onMessageBox(endgameBox(kEndGameMsg));
    REQUIRE(routed == Route::Suppressed);
    w.step(100);
    REQUIRE(w.shown(0) == 1);
    const uint32_t id = w.lastMenuId(0);
    CHECK(w.shown(1) == 0 && w.shown(2) == 0 && w.eng.lastButton() == kNoButton);
    w.n[0]->menuReply(id, 1, uint32_t(w.now));                                    // the host presses "Let's do this!"
    w.step(100);
    CHECK(w.shown(1) == 1 && w.shown(2) == 1 && w.eng.lastButton() == kNoButton);   // the guests are asked, the script waits
    w.n[1]->menuReply(id, 0, uint32_t(w.now));
    w.n[2]->menuReply(id, 1, uint32_t(w.now));                                    // one "Not yet"
    w.step(100);
    CHECK(w.eng.lastButton() == 0);                                               // ... cancels it: the decline button
}
// Joshua or Daniel, Elijah's offer: a fork INFO from the consent table is held at the host until the party agreed
TEST(fork_info_from_the_consent_table_waits_for_the_party) {
    World w;
    w.n[2]->requestStart(F(0x7000), uint32_t(w.now));
    w.step(100);
    w.n[2]->pick(F(0x1000), F(0x2000), 0, false, {fnvcoop_dlg::Line{"Lead the way.", "", 0, 50}}, {}, uint32_t(w.now));
    w.step(100);
    CHECK(w.scriptRuns == 0);                                                      // no result script before the vote
    REQUIRE(w.shown(0) == 1 && w.shown(1) == 1 && w.shown(2) == 0);
    w.n[1]->menuReply(w.lastMenuId(1), 0, uint32_t(w.now));                      // agree: initiator + player 1 = a majority of three
    w.step(100);
    CHECK(w.scriptRuns >= 1);
}
TEST(consent_origin_disconnect_takes_the_decline_default) {
    World w;
    OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 2);
    w.router->onMessageBox(endgameBox(kEndGameMsg));
    w.step(100);
    w.n[0]->playerLeft(2, uint32_t(w.now));
    w.step(50);
    CHECK(w.eng.lastButton() != kNoButton);  // the script is never stranded
}

// ---- wave 1b (G08): a ChoiceConsent box, the vote is the rules engine's ----------------------------------------------------------------
static MessageBoxCall templeBox(uint32_t script = 93, std::vector<std::string> buttons = {"Yes", "No"}) {
    MessageBoxCall c;
    c.scriptRefID = script;
    c.thisObj = 0x300;
    c.message = F(0x4D0001);
    c.text = "Launch the missiles at the Legion?";
    c.buttons = std::move(buttons);
    return c;
}
TEST(rules_owned_box_asks_the_party_and_waits_for_the_engines_verdict) {
    World w;
    OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 2);
    REQUIRE(w.router->onMessageBox(templeBox()) == Route::Suppressed);
    w.step(100);
    REQUIRE(w.shown(2) == 1);
    const uint32_t id = w.lastMenuId(2);
    w.n[2]->menuReply(id, 0, uint32_t(w.now));                                      // the origin picks "Yes", the point of no return
    w.step(100);
    CHECK(w.boxAsked.size() == 1 && w.boxAsked[0].first == F(0x4D0001) && w.boxAsked[0].second == 2);   // the engine is asked once
    CHECK(w.shown(0) == 1 && w.shown(1) == 1 && w.eng.lastButton() == kNoButton);   // the others are asked, the script waits
    w.n[0]->menuReply(id, 0, uint32_t(w.now));                                      // Agree
    w.n[1]->menuReply(id, 1, uint32_t(w.now));                                      // Not yet
    w.step(100);
    CHECK(w.votes.size() == 2 && std::get<0>(w.votes[0]) == 0 && std::get<2>(w.votes[0]) && std::get<0>(w.votes[1]) == 1 && !std::get<2>(w.votes[1]));
    CHECK(w.eng.lastButton() == kNoButton);                                         // the host does not count the votes: only the engine's verdict resumes the script
    w.n[0]->rulesResolved(F(0x4D0001), true, uint32_t(w.now));
    w.step(100);
    CHECK(w.eng.lastButton() == 0 && w.router->pendingBoxes() == 0);                // accepted: the origin's own button
}
TEST(rules_owned_box_declined_gives_the_last_button) {
    World w;
    OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 1);
    REQUIRE(w.router->onMessageBox(templeBox()) == Route::Suppressed);
    w.step(100);
    w.n[1]->menuReply(w.lastMenuId(1), 0, uint32_t(w.now));
    w.step(100);
    w.n[0]->rulesResolved(F(0x4D0001), false, uint32_t(w.now));
    w.step(100);
    CHECK(w.eng.lastButton() == 1);                                                 // "No": the last button, nothing irreversible happened
}
// The game data (LonesomeRoad.esm, the three NVDLC04Temple*ConfirmMSG boxes) lists the buttons "No." then "Yes.": the point of no return is the LAST one.
TEST(rules_owned_box_reads_yes_and_no_from_the_button_texts) {
    int32_t yes = 0, no = 1;
    rulesBoxButtons({"No.", "Yes."}, &yes, &no);
    CHECK(yes == 1 && no == 0);
    yes = 0; no = 1;
    rulesBoxButtons({"Yes", "No"}, &yes, &no);
    CHECK(yes == 0 && no == 1);
    yes = 0; no = 2;
    rulesBoxButtons({"Launch", "Wait", "Cancel"}, &yes, &no);   // no yes / no text: the conventions stay
    CHECK(yes == 0 && no == 2);
    World w;
    OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 1);
    REQUIRE(w.router->onMessageBox(templeBox(93, {"No.", "Yes."})) == Route::Suppressed);
    w.step(100);
    REQUIRE(w.shown(1) == 1);
    w.n[1]->menuReply(w.lastMenuId(1), 1, uint32_t(w.now));                          // "Yes." is button 1 here: the party is asked
    w.step(100);
    CHECK(w.boxAsked.size() == 1);
    w.n[0]->rulesResolved(F(0x4D0001), false, uint32_t(w.now));
    w.step(100);
    CHECK(w.eng.lastButton() == 0);                                                 // declined: "No.", the FIRST button
}
TEST(rules_owned_box_decided_by_the_host_asks_the_guests) {
    World w;
    REQUIRE(w.router->onMessageBox(templeBox()) == Route::Suppressed);              // the host player is the origin
    w.step(100);
    REQUIRE(w.shown(0) == 1);
    w.n[0]->menuReply(w.lastMenuId(0), 0, uint32_t(w.now));
    w.step(100);
    CHECK(w.shown(1) == 1 && w.shown(2) == 1 && w.boxAsked.size() == 1 && w.boxAsked[0].second == 0 && w.eng.lastButton() == kNoButton);
}
TEST(rules_owned_box_other_button_is_no_point_of_no_return) {
    World w;
    OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 2);
    REQUIRE(w.router->onMessageBox(templeBox()) == Route::Suppressed);
    w.step(100);
    w.n[2]->menuReply(w.lastMenuId(2), 1, uint32_t(w.now));                         // "No"
    w.step(100);
    CHECK(w.boxAsked.empty() && w.shown(0) == 0 && w.eng.lastButton() == 1);
}
TEST(condition_view_reads_the_rules_engines_per_player_variables_by_name) {
    World w;
    {
        OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 1);
        CHECK(w.router->viewQuestVar(0x7700, 4, 99.0) == 11.5);                     // variable 4 of quest 0x7700 is fCollarTimer: the evaluating player's copy
    }
    {
        OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 2);
        CHECK(w.router->viewQuestVar(0x7700, 4, 99.0) == 12.5 && w.router->viewQuestVar(0x7700, 5, 99.0) == 99.0);   // another variable index has no name: untouched
    }
    CHECK(w.router->viewQuestVar(0x7701, 4, 99.0) == 99.0);                         // another quest
}

// ---- forced dialogue + condition views through the node ---------------------------------------------------------------------------
TEST(forced_say_in_a_scene_reaches_every_player) {
    World w;
    ForcedCall c;
    c.kind = ForcedCall::Kind::SayTo;
    c.speaker = 0x5000;
    c.target = kPlayerRefID;  // outside any scope: the party
    c.topic = F(0x6000);
    c.info = F(0x6001);
    c.lines = {fnvcoop_dlg::Line{"The legion fell.", "v.ogg", 0, 50}};
    const ForcedOutcome o = w.router->onForced(c);
    REQUIRE(o.captured);
    w.step(100);
    for (PlayerId p = 0; p < 3; ++p) CHECK(w.count(p, Event::ForcedSayShow) == 1);
    const auto& m = std::get<fnvcoop_dlg::ForcedLines>(w.ev[1].back().msg);
    CHECK(m.scene && m.lines.size() == 1 && m.lines[0].text == "The legion fell." && m.txnSeq == w.seqProvided);
}
TEST(forced_say_to_one_player_is_aimed) {
    World w;
    ForcedCall c;
    c.kind = ForcedCall::Kind::SayTo;
    c.speaker = 0x5000;
    c.target = 0x2002;  // a guest ghost = slot 2
    c.topic = F(0x6000);
    c.lines = {fnvcoop_dlg::Line{"Hey you.", "", 0, 50}};
    REQUIRE(w.router->onForced(c).captured);
    w.step(100);
    const auto& m = std::get<fnvcoop_dlg::ForcedLines>(w.ev[2].back().msg);
    CHECK(m.target == 2 && !m.scene);
}
TEST(start_conversation_picks_the_actor_and_opens_it_there) {
    World w;
    w.n[0]->setActorChooser([](const FormRef&, uint8_t mask) { return PlayerId((mask & 2) ? 1 : 0); });
    ForcedCall c;
    c.kind = ForcedCall::Kind::StartConversation;
    c.speaker = 0x5000;
    c.target = kPlayerRefID;
    c.topic = F(0x6000);
    const ForcedOutcome o = w.router->onForced(c);
    CHECK(o.captured && o.target == 1 && o.vetoNative);
    w.step(100);
    CHECK(w.count(1, Event::ForcedOpen) == 1 && w.count(2, Event::ForcedOpen) == 0);
}
TEST(condition_view_uses_the_evaluating_players_followers) {
    World w;
    w.n[0]->companions().addVarRule(F(0x7000), 3, fnvcoop_dlg::VarKind::HasFollower);
    w.n[0]->onCompanionOwner(F(0x7100), 1);
    {
        OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 1);
        CHECK(w.router->viewQuestVar(0x7000, 3, 0) == 1.0);
    }
    {
        OriginScope s(w.router->origin(), OriginContext::Kind::Acting, 2);
        CHECK(w.router->viewQuestVar(0x7000, 3, 1) == 0.0);
    }
    CHECK(w.router->viewQuestVar(0x7001, 3, 5) == 5.0);  // other quests untouched
}

// ---- slideshow -------------------------------------------------------------------------------------------------------------------
static const char* kEnd(int i, char* buf) {
    std::snprintf(buf, 64, "interface\\endgame\\endscrn_WN%02d", i);
    return buf;
}
static bool swapEnding(World& w, int i) {
    char b[64];
    return w.router->onSwapTexture(0x9000, "##GS01:0", kEnd(i, b)), true;
}

TEST(slide_classifier_table) {
    CHECK(classifySlide("interface\\endgame\\endscrn_WN01") == SlideKind::Ending);
    CHECK(classifySlide("Interface/EndGame/endscrn_RM01") == SlideKind::Ending);
    CHECK(classifySlide("NVDLC02\\interface\\endgame\\x") == SlideKind::Ending);
    CHECK(classifySlide("NVDLC01\\interface\\IntroMovie\\NVDLC01_IntroMovie14") == SlideKind::Intro);
    CHECK(classifySlide("NVDLC02\\interface\\intro\\a") == SlideKind::Intro);
    CHECK(classifySlide("clutter\\rorchachtest\\rorschach01") == SlideKind::None);
    CHECK(classifySlide("weapons\\1handpistol\\EuclidScreenSuccess") == SlideKind::None);
    CHECK(classifySlide("") == SlideKind::None);
}
TEST(slide_codec_roundtrip_and_corruption) {
    SlideMsg m;
    m.type = SlideMsg::Slide;
    m.show = 3;
    m.item.seq = 9;
    m.item.kind = SlideKind::Ending;
    m.item.atMs = 123456;
    m.item.ref = F(0xABCDEF);
    m.item.oldTex = "##GS01:0";
    m.item.newTex = "interface\\endgame\\endscrn_WN01";
    auto b = encodeSlide(m);
    SlideMsg d;
    REQUIRE(decodeSlide(b, d) == SlideDecode::Ok);
    CHECK(d.show == 3 && d.item.seq == 9 && d.item.kind == SlideKind::Ending && d.item.atMs == 123456 && d.item.ref == F(0xABCDEF) &&
          d.item.oldTex == m.item.oldTex && d.item.newTex == m.item.newTex);
    for (size_t i = 0; i < b.size(); ++i) {
        auto bad = b;
        bad[i] ^= 0x10;
        SlideMsg x;
        CHECK(decodeSlide(bad, x) != SlideDecode::Ok);
    }
    for (size_t n = 0; n < b.size(); ++n) {
        std::vector<uint8_t> cut(b.begin(), b.begin() + long(n));
        SlideMsg x;
        CHECK(decodeSlide(cut, x) != SlideDecode::Ok);
    }
    SlideMsg e;
    e.type = SlideMsg::End;
    e.show = 3;
    SlideMsg ed;
    CHECK(decodeSlide(encodeSlide(e), ed) == SlideDecode::Ok && ed.type == SlideMsg::End && ed.show == 3);
}
TEST(slides_play_in_order_and_controls_lock_until_the_end) {
    World w;
    for (int i = 1; i <= 5; ++i) swapEnding(w, i);
    w.router->onSwapTexture(0x9100, "x", "clutter\\rorchachtest\\r01");  // not a slide
    w.step(100);
    for (PlayerId p = 1; p <= 2; ++p) {
        REQUIRE(w.cr[p].swaps.size() == 5);
        CHECK(w.cr[p].swaps[0] == "interface\\endgame\\endscrn_WN01" && w.cr[p].swaps[4] == "interface\\endgame\\endscrn_WN05");
        CHECK(w.cr[p].sc->locked());
        CHECK(!w.cr[p].lockLog.empty() && w.cr[p].lockLog[0] == true);
    }
    CHECK(w.cr[0].swaps.empty());  // the host's own scripts already ran the swaps
    w.slideHost->end();
    w.step(100);
    for (PlayerId p = 1; p <= 2; ++p) {
        CHECK(!w.cr[p].sc->locked() && w.cr[p].sc->finished());
        CHECK(w.cr[p].lockLog.size() == 2 && w.cr[p].lockLog[1] == false);
    }
    CHECK(!w.slideHost->active());
}
TEST(slides_survive_duplication_and_reordering) {
    World w(10, 0.6, 60, 11);
    for (int i = 1; i <= 12; ++i) swapEnding(w, i);
    w.slideHost->end();
    w.step(600);
    for (PlayerId p = 1; p <= 2; ++p) {
        REQUIRE(w.cr[p].swaps.size() == 12);
        for (int i = 0; i < 12; ++i) {
            char b[64];
            CHECK(w.cr[p].swaps[size_t(i)] == kEnd(i + 1, b));
        }
        CHECK(!w.cr[p].sc->locked());
    }
}
TEST(late_joiner_gets_the_show_so_far) {
    World w;
    w.slideHost->playerLeft(2);
    for (int i = 1; i <= 3; ++i) swapEnding(w, i);
    w.step(60);
    CHECK(w.cr[2].swaps.empty());
    w.slideHost->playerJoined(2, w.now);
    swapEnding(w, 4);
    w.step(100);
    REQUIRE(w.cr[2].swaps.size() == 4);
    CHECK(w.cr[2].swaps[0] == "interface\\endgame\\endscrn_WN01" && w.cr[2].sc->locked());
    w.slideHost->end();
    w.step(60);
    CHECK(!w.cr[2].sc->locked());
}
TEST(lost_end_packet_is_covered_by_the_slideshow_end_effect) {
    World w;
    for (int i = 1; i <= 2; ++i) swapEnding(w, i);
    w.step(60);
    CHECK(w.cr[1].sc->locked());
    w.cr[1].sc->onSlideshowEndEffect();  // the L4 sticky flag arrives instead of the End packet
    CHECK(!w.cr[1].sc->locked());
}
TEST(host_lost_or_watchdog_releases_the_lock) {
    World w;
    swapEnding(w, 1);
    w.step(60);
    CHECK(w.cr[1].sc->locked() && w.cr[2].sc->locked());
    w.cr[1].sc->onHostLost();
    CHECK(!w.cr[1].sc->locked());
    w.step(25ull * 60 * 1000 + 100);  // no End, no packets: the client watchdog fires (the host closes its show at 20 minutes first)
    CHECK(!w.cr[2].sc->locked());
}
TEST(intro_slides_play_without_locking) {
    World w;
    w.router->onSwapTexture(0x9200, "##NVDLC01_IntroMovie00:0", "NVDLC01\\interface\\IntroMovie\\NVDLC01_IntroMovie01");
    w.step(100);
    CHECK(w.cr[1].swaps.size() == 1 && !w.cr[1].sc->locked() && w.cr[1].lockLog.empty());
}
TEST(second_show_after_the_first_ended) {
    World w;
    swapEnding(w, 1);
    w.slideHost->end();
    w.step(60);
    swapEnding(w, 2);
    w.step(60);
    CHECK(w.cr[1].swaps.size() == 2 && w.cr[1].sc->locked() && w.cr[1].sc->show() == 2);
    w.slideHost->end();
    w.step(60);
    CHECK(!w.cr[1].sc->locked());
}
TEST(stale_packets_of_an_old_show_are_ignored) {
    World w;
    swapEnding(w, 1);
    w.step(60);
    SlideMsg old;
    old.type = SlideMsg::Slide;
    old.show = 0;
    old.item.seq = 1;
    old.item.kind = SlideKind::Ending;
    old.item.newTex = "interface\\endgame\\old";
    CHECK(w.cr[1].sc->onPacket(encodeSlide(old), w.now));
    CHECK(w.cr[1].swaps.size() == 1);
    CHECK(!w.cr[1].sc->onPacket({0xD1, 1, 2}, w.now));  // a dialogue packet is not ours
}

// ---- one total order for slides and narration (showorder.h); twins 17..20 ------------------------------------------------------------
// Twins 17..20 were declared in the sources but nothing ran them and no test could fail them. These do (FNVCOOP_MUT=17..20).
TEST(show_sequencer_orders_drops_duplicates_and_skips_a_lost_item) {
    ShowSequencer s;
    std::vector<int> ran;
    auto act = [&ran](int n) { return [&ran, n] { ran.push_back(n); }; };
    CHECK(s.submit(ShowStamp{2, 1}, 0, act(2)));   // ord 2 arrives first and must wait for ord 1
    CHECK(ran.empty() && s.pending() == 1);
    CHECK(s.submit(ShowStamp{1, 0}, 5, act(1)));
    CHECK((ran == std::vector<int>{1, 2}));
    CHECK(!s.submit(ShowStamp{1, 0}, 10, act(11)));   // a retransmission of a released item is dropped (twin 18)
    CHECK(!s.submit(ShowStamp{2, 1}, 10, act(12)));
    CHECK((ran == std::vector<int>{1, 2}) && s.dups() == 2);
    CHECK(s.submit(ShowStamp{4, 3}, 100, act(4)));    // ord 3 never arrives
    s.tick(100 + 2999);
    CHECK(ran.size() == 2 && s.pending() == 1);        // not yet: the gap timeout has not passed
    s.tick(100 + 3000);                                // twin 19: a lost predecessor must not block the show forever
    CHECK((ran == std::vector<int>{1, 2, 4}) && s.pending() == 0 && s.gapSkips() == 1);
    CHECK(s.submit(ShowStamp(), 200, act(99)) && ran.back() == 99);   // an unstamped item plays at once
}
TEST(narration_gate_strips_the_stamp_and_follows_the_order) {
    ShowSequencer seq;
    std::vector<std::string> got;
    NarrationGate::Sink sink;
    sink.subtitle = [&got](const FormRef&, const std::string& t, float) { got.push_back("N:" + t); };
    sink.voice = [&got](const FormRef&, const std::string& tok) { got.push_back("V:" + tok); };
    NarrationGate g(seq, sink);
    const FormRef npc = F(0x5000);
    g.onSubtitle(npc, "two", 2.0f);
    g.onVoice(npc, withOrder("b.ogg", ShowStamp{2, 1}), 0);   // the second line arrives first
    CHECK(got.empty());                                         // twin 17 (stamp ignored) plays it at once, with the suffix still on the token
    g.onSubtitle(npc, "one", 2.0f);
    g.onVoice(npc, withOrder("a.ogg", ShowStamp{1, 0}), 1);
    CHECK((got == std::vector<std::string>{"N:one", "V:a.ogg", "N:two", "V:b.ogg"}));
    g.onSubtitle(npc, "plain", 2.0f);
    g.onVoice(npc, "c.ogg", 2);                                 // no stamp: delivered at once, like before
    CHECK(got.size() == 6 && got[4] == "N:plain" && got[5] == "V:c.ogg");
}
TEST(scene_narration_is_stamped_by_the_host_and_plays_in_order_with_the_slides) {
    {   // the host stamps every scene line (twin 20: it does not)
        World w;
        w.narrate("The legion fell.", 0x6000);
        w.step(100);
        const Event* shown = nullptr;
        for (auto& e : w.ev[1])
            if (e.kind == Event::ForcedSayShow) shown = &e;
        REQUIRE(shown != nullptr);
        const auto& f = std::get<fnvcoop_dlg::ForcedLines>(shown->msg);
        REQUIRE(f.lines.size() == 1);
        std::string rest;
        const ShowStamp st = parseOrder(f.lines[0].voice, rest);
        VoiceToken vt;
        CHECK(st.valid() && parseVoiceToken(rest, vt) && vt.topic.localId == 0x6000 && vt.info.localId == 0x6001);   // stamp + the topic/INFO voice token
    }
    // narration and slides interleaved on a link that duplicates and reorders: both players see exactly the host's order
    for (uint64_t seed : {11u, 12u, 13u}) {
        World w(10, 0.6, 120, seed);
        std::vector<std::string> want;
        for (int i = 1; i <= 6; ++i) {
            w.narrate("line" + std::to_string(i), 0x6000 + 2u * uint32_t(i));
            want.push_back("N:line" + std::to_string(i));
            want.push_back("V:" + makeVoiceToken(F(0x6000 + 2u * uint32_t(i)), F(0x6001 + 2u * uint32_t(i)), 1));   // every line also names its topic for `Say`
            swapEnding(w, i);
            char b[64];
            want.push_back(std::string("S:") + kEnd(i, b));
        }
        w.slideHost->end();
        w.step(2500);
        for (PlayerId p = 1; p <= 2; ++p) CHECK(w.cr[p].shown == want);
    }
}

// ---- voice token (what Line.voice carries) -----------------------------------------------------------------------------------------
TEST(voice_token_roundtrip_and_garbage) {
    const std::string t = makeVoiceToken(FormRef("falloutnv.esm", 0x1234AB), FormRef("honesthearts.esm", 0x0F00), 3);
    VoiceToken v;
    REQUIRE(parseVoiceToken(t, v));
    CHECK(v.topic == FormRef("falloutnv.esm", 0x1234AB) && v.info == FormRef("honesthearts.esm", 0x0F00) && v.response == 3);
    for (const char* bad : {"", "nvsay", "nvsay|a|b|1|2", "nvsay|a|b|1|2|3|4", "nvsay||b|1|2|3", "nvsay|a|b|zz|2|3", "nvsay|a|b|1000000|2|3", "nvsay|a|b|1|2|0",
                            "nvsay|a|b|1|2|999", "Data/Sound/Voice/x.ogg", "NVSAY|a|b|1|2|3"}) {
        VoiceToken x;
        CHECK(!parseVoiceToken(bad, x));
    }
}

int main() {
    run("voice_token_roundtrip_and_garbage", voice_token_roundtrip_and_garbage);
    run("show_sequencer_orders_drops_duplicates_and_skips_a_lost_item", show_sequencer_orders_drops_duplicates_and_skips_a_lost_item);
    run("narration_gate_strips_the_stamp_and_follows_the_order", narration_gate_strips_the_stamp_and_follows_the_order);
    run("scene_narration_is_stamped_by_the_host_and_plays_in_order_with_the_slides", scene_narration_is_stamped_by_the_host_and_plays_in_order_with_the_slides);
    run("consent_waits_for_the_party_then_resumes_with_confirm", consent_waits_for_the_party_then_resumes_with_confirm);
    run("consent_declined_gives_the_not_ready_button", consent_declined_gives_the_not_ready_button);
    run("plain_box_has_no_consent_round", plain_box_has_no_consent_round);
    run("consent_box_decided_by_the_host_asks_the_guests", consent_box_decided_by_the_host_asks_the_guests);
    run("fork_info_from_the_consent_table_waits_for_the_party", fork_info_from_the_consent_table_waits_for_the_party);
    run("consent_origin_disconnect_takes_the_decline_default", consent_origin_disconnect_takes_the_decline_default);
    run("rules_owned_box_asks_the_party_and_waits_for_the_engines_verdict", rules_owned_box_asks_the_party_and_waits_for_the_engines_verdict);
    run("rules_owned_box_declined_gives_the_last_button", rules_owned_box_declined_gives_the_last_button);
    run("rules_owned_box_reads_yes_and_no_from_the_button_texts", rules_owned_box_reads_yes_and_no_from_the_button_texts);
    run("rules_owned_box_decided_by_the_host_asks_the_guests", rules_owned_box_decided_by_the_host_asks_the_guests);
    run("rules_owned_box_other_button_is_no_point_of_no_return", rules_owned_box_other_button_is_no_point_of_no_return);
    run("condition_view_reads_the_rules_engines_per_player_variables_by_name", condition_view_reads_the_rules_engines_per_player_variables_by_name);
    run("forced_say_in_a_scene_reaches_every_player", forced_say_in_a_scene_reaches_every_player);
    run("forced_say_to_one_player_is_aimed", forced_say_to_one_player_is_aimed);
    run("start_conversation_picks_the_actor_and_opens_it_there", start_conversation_picks_the_actor_and_opens_it_there);
    run("condition_view_uses_the_evaluating_players_followers", condition_view_uses_the_evaluating_players_followers);
    run("slide_classifier_table", slide_classifier_table);
    run("slide_codec_roundtrip_and_corruption", slide_codec_roundtrip_and_corruption);
    run("slides_play_in_order_and_controls_lock_until_the_end", slides_play_in_order_and_controls_lock_until_the_end);
    run("slides_survive_duplication_and_reordering", slides_survive_duplication_and_reordering);
    run("late_joiner_gets_the_show_so_far", late_joiner_gets_the_show_so_far);
    run("lost_end_packet_is_covered_by_the_slideshow_end_effect", lost_end_packet_is_covered_by_the_slideshow_end_effect);
    run("host_lost_or_watchdog_releases_the_lock", host_lost_or_watchdog_releases_the_lock);
    run("intro_slides_play_without_locking", intro_slides_play_without_locking);
    run("second_show_after_the_first_ended", second_show_after_the_first_ended);
    run("stale_packets_of_an_old_show_are_ignored", stale_packets_of_an_old_show_are_ignored);
    std::printf("%d checks, %d failed\n", g_checks, g_fail);
    return g_fail ? 1 : 0;
}
