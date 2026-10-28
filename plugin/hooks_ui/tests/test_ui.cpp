// Unit tests for hooks_ui (game-free). Built once (FNVCOOP_RUNTIME_MUT); run once for real and once per FNVCOOP_MUT=<n>; every twin MUST fail.
//   1 origin ignored            2 menu resume not one-shot     3 button off-by-one       4 condition view uses host slot
//   5 NPC-to-NPC lines captured 6 synchronous resume dropped   7 sticky origin forever   8 host DialogMenu opens for a remote actor
//   9 slides recorded out of order
//   21 forced lines carry no voice token  22 MenuMode mirror never opened  23 host-origin party box stays local  24 mirror not closed on the answer
// Optional: NVUI_EXE=<dump of the running game image> also compares every pinned prologue with the real image.
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <iterator>
#include <string>
#include <vector>

#include "fnvcoop_mut.h"
#include "../ui_addrs.h"
#include "../ui_core.h"
#include "../voice_token.h"

using namespace nvui;

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
        CHECK(c);         \
        if (!(c)) return; \
    } while (0)
#define TEST(name) static void name()
static void run(const char* n, void (*f)()) {
    int before = g_fail;
    f();
    std::printf("[%s] %s\n", g_fail == before ? " ok " : "FAIL", n);
}

// ---- fakes -------------------------------------------------------------------------------------------------------------------
struct FakeEngine : IUiEngine {
    uint64_t now = 1000;
    struct Write {
        uint32_t script;
        int32_t button;
    };
    std::vector<Write> writes;
    uint64_t nowMs() override { return now; }
    void setMessageBoxResult(uint32_t s, int32_t b) override { writes.push_back({s, b}); }
    bool refOf(uint32_t id, FormRef& out) override {
        if (id == 0) return false;
        out = FormRef("falloutnv.esm", id);
        return true;
    }
    std::vector<std::string> logs;
    std::vector<uint32_t> mirrorsOpen, mirrorsClosed;
    void log(const std::string& s) override { logs.push_back(s); }
    void openMirror(uint32_t menu, const std::string&, uint8_t) override { mirrorsOpen.push_back(menu); }
    void closeMirror(uint32_t menu) override { mirrorsClosed.push_back(menu); }
    int32_t lastButton() const { return writes.empty() ? -2 : writes.back().button; }
};

struct Rig {
    FakeEngine eng;
    UiRouter r;
    struct Raised {
        PlayerId origin;
        MenuKind kind;
        std::string text;
        std::vector<std::string> buttons;
        int32_t def;
        FormRef subject;
        uint32_t id;
        bool modal;
        ConsentRule rule;
    };
    std::vector<Raised> raised;
    uint32_t nextId = 0;
    bool resumeSync = false;
    int32_t syncChoice = 0;
    std::vector<ForcedSayArgs> said;
    std::vector<FormRef> convs;
    PlayerId convActor = 1;
    int playerVarAsks = 0;
    std::vector<std::tuple<std::string, std::string>> slidesSeen;

    Rig() : r(eng, Bridge()) {
        r.setHost(true);
        Bridge& b = r.bridge();
        b.slotOfActor = [](uint32_t ref) -> PlayerId { return ref == 0x14 ? kHost : ref == 0x2000 ? PlayerId(1) : ref == 0x2001 ? PlayerId(2) : kNoSlot; };
        b.raiseMenu = [this](PlayerId o, MenuKind k, const std::string& t, std::vector<std::string> bs, int32_t d, const FormRef& s) {
            uint32_t id = ++nextId;
            raised.push_back({o, k, t, bs, d, s, id, false, {}});
            if (resumeSync) r.onMenuResume(id, syncChoice, true);
            return id;
        };
        b.raiseModal = [this](PlayerId o, const std::string& t, std::vector<std::string> bs, int32_t d, const ConsentRule& rule) {
            uint32_t id = ++nextId;
            raised.push_back({o, MenuKind::MessageBox, t, bs, d, FormRef(), id, true, rule});
            return id;
        };
        b.consentFor = [](const FormRef& m) -> std::optional<ConsentRule> {
            if (m.localId == 0xBEEF) return ConsentRule{0, 1, 0};
            if (m.localId == 0xCAE5) return ConsentRule{-1, 0, 0, 2, true};   // a personal decision (Caesar's surgery): the actor alone
            if (m.localId == 0xF0F0) return ConsentRule{1, 0, 0, 1, false};   // a story fork: the party votes
            return std::nullopt;
        };
        b.watchesQuest = [](uint32_t q) { return q == 0x500; };
        b.hasPlayerVars = [this](const FormRef& q) { ++playerVarAsks; return q.localId == 0x600; };   // wave 1b: the rules engine's per-player variables of quest 0x600
        b.viewPlayerVar = [](const FormRef&, uint32_t idx, PlayerId slot, double real) { return idx == 3 ? 0.25 + slot : real; };   // a float variable: slot 0 0.25, slot 1 1.25, slot 2 2.25
        b.viewVar = [](const FormRef&, uint32_t idx, PlayerId slot, int32_t real) { return idx == 47 ? int32_t(slot == 2 ? 1 : 0) : real; };
        b.forcedSay = [this](const ForcedSayArgs& a) { said.push_back(a); };
        b.forceConversation = [this](const FormRef&, const FormRef& t, PlayerId) { convs.push_back(t); return convActor; };
        b.slideSwap = [this](const FormRef&, const std::string& o, const std::string& n) { slidesSeen.emplace_back(o, n); };
    }
};

static MessageBoxCall box(uint32_t script = 77, uint32_t thisObj = 0x300) {
    MessageBoxCall c;
    c.scriptRefID = script;
    c.thisObj = thisObj;
    c.text = "Leave the Dam?";
    c.buttons = {"Yes", "No"};
    return c;
}

// ---- origin ------------------------------------------------------------------------------------------------------------------
TEST(origin_scope_wins_and_nests) {
    Rig g;
    auto& o = g.r.origin();
    CHECK(o.resolve(0, 0, 1000) == kHost);
    {
        OriginScope a(o, OriginContext::Kind::Acting, 2);
        CHECK(o.resolve(0, 0, 1000) == 2);
        {
            OriginScope d(o, OriginContext::Kind::Dialogue, 1);
            CHECK(o.resolve(0, 0, 1000) == 1);  // the dialogue initiator inside an acting window
        }
        CHECK(o.resolve(0, 0, 1000) == 2);
    }
    CHECK(!o.scoped());
    CHECK(o.resolve(0, 0, 1000) == kHost);
}
TEST(origin_activation_and_sticky_expire) {
    Rig g;
    auto& o = g.r.origin();
    o.noteActivation(0x300, 2, 1000);
    CHECK(o.resolve(0x300, 0, 3000) == 2);
    CHECK(o.resolve(0x301, 0, 3000) == kHost);   // another object
    CHECK(o.resolve(0x300, 0, 9000) == kHost);   // 8 s later the activation is stale
    o.noteScript(77, 1, 1000);
    CHECK(o.resolve(0, 77, 20000) == 1);
    CHECK(o.resolve(0, 77, 40000) == kHost);     // sticky window (30 s) over
}

// ---- message boxes -----------------------------------------------------------------------------------------------------------
TEST(box_for_host_origin_stays_local) {
    Rig g;
    CHECK(g.r.onMessageBox(box()) == Route::Local);
    CHECK(g.raised.empty());
    CHECK(g.eng.writes.empty());
}
TEST(box_for_guests_is_raised_to_origin_and_resumes_with_choice) {
    Rig g;
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    CHECK(g.r.onMessageBox(box(77, 0x300)) == Route::Suppressed);
    REQUIRE(g.raised.size() == 1);
    CHECK(g.raised[0].origin == 2);
    CHECK(g.raised[0].kind == MenuKind::MessageBox);
    CHECK(g.raised[0].text == "Leave the Dam?");
    CHECK(g.raised[0].buttons.size() == 2 && g.raised[0].buttons[1] == "No");
    CHECK(g.raised[0].def == 1);                    // timeout picks the LAST button, never a point of no return
    CHECK(g.raised[0].subject.localId == 0x300);
    CHECK(g.eng.lastButton() == kNoButton);         // GetButtonPressed answers -1 while the box is open
    CHECK(g.r.pendingBoxes() == 1);
    g.r.onMenuResume(g.raised[0].id, 1, false);
    CHECK(g.eng.lastButton() == 1);
    CHECK(g.eng.writes.back().script == 77);
    CHECK(g.r.pendingBoxes() == 0);
}
TEST(box_first_button_is_zero) {
    Rig g;
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 1);
    g.r.onMessageBox(box());
    REQUIRE(!g.raised.empty());
    g.r.onMenuResume(g.raised[0].id, 0, false);
    CHECK(g.eng.lastButton() == 0);
}
TEST(box_resume_is_one_shot) {
    Rig g;
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    g.r.onMessageBox(box());
    g.r.onMenuResume(1, 0, false);
    const size_t n = g.eng.writes.size();
    g.r.onMenuResume(1, 1, false);  // duplicate delivery
    CHECK(g.eng.writes.size() == n);
    CHECK(g.eng.lastButton() == 0);
    CHECK(g.r.stats().dupResumes == 1);
}
TEST(box_synchronous_resume_during_raise) {
    Rig g;
    g.resumeSync = true;  // origin already gone: HostDialogue resumes with the default before returning the id
    g.syncChoice = 1;
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    CHECK(g.r.onMessageBox(box()) == Route::Suppressed);
    CHECK(g.eng.lastButton() == 1);
    CHECK(g.r.pendingBoxes() == 0);
}
TEST(box_cancel_and_out_of_range_leave_no_button) {
    Rig g;
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    g.r.onMessageBox(box());
    g.r.onMenuResume(1, -1, false);
    CHECK(g.eng.lastButton() == kNoButton);
    g.r.onMessageBox(box());
    g.r.onMenuResume(2, 5, false);  // two buttons only
    CHECK(g.eng.lastButton() == kNoButton);
}
TEST(box_without_buttons_answers_zero) {
    Rig g;
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    MessageBoxCall c = box();
    c.buttons.clear();
    g.r.onMessageBox(c);
    REQUIRE(!g.raised.empty());
    CHECK(g.raised[0].def == -1);
    g.r.onMenuResume(1, 0, false);
    CHECK(g.eng.lastButton() == 0);
}
TEST(box_point_of_no_return_uses_modal) {
    Rig g;
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    MessageBoxCall c = box();
    c.message = FormRef("falloutnv.esm", 0xBEEF);
    g.r.onMessageBox(c);
    REQUIRE(g.raised.size() == 1 && g.raised[0].modal);
    CHECK(g.raised[0].rule.declineChoice == 1);
}
TEST(party_box_decided_by_the_host_is_still_raised_to_the_party) {
    Rig g;
    MessageBoxCall c = box();
    c.message = FormRef("falloutnv.esm", 0xBEEF);   // no scope: the host player is the origin
    CHECK(g.r.onMessageBox(c) == Route::Suppressed);
    REQUIRE(g.raised.size() == 1 && g.raised[0].modal && g.raised[0].origin == kHost);
    g.r.onMenuResume(1, 1, false);
    CHECK(g.eng.lastButton() == 1);
    // a vote-mode fork is raised too, a personal decision is not
    MessageBoxCall fork = box(78);
    fork.message = FormRef("falloutnv.esm", 0xF0F0);
    CHECK(g.r.onMessageBox(fork) == Route::Suppressed);
    REQUIRE(g.raised.size() == 2 && g.raised[1].rule.mode == 1);
    MessageBoxCall personal = box(79);
    personal.message = FormRef("falloutnv.esm", 0xCAE5);
    CHECK(g.r.onMessageBox(personal) == Route::Local);
    CHECK(g.raised.size() == 2 && g.r.stats().hostConsent == 2);
}
TEST(box_follow_up_of_same_script_follows_origin) {
    Rig g;
    {
        OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
        g.r.onMessageBox(box(88));
    }
    g.r.onMenuResume(1, 0, false);
    g.eng.now += 10000;  // the quest script shows its second box a few seconds later, outside any scope
    CHECK(g.r.onMessageBox(box(88, 0)) == Route::Suppressed);
    REQUIRE(g.raised.size() == 2);
    CHECK(g.raised[1].origin == 2);
    g.eng.now += 60000;  // long after: a different play session of the same script belongs to the host again
    g.r.onMenuResume(2, 0, false);
    CHECK(g.r.onMessageBox(box(88, 0)) == Route::Local);
}
TEST(box_on_guest_machine_is_always_local) {
    Rig g;
    g.r.setHost(false);
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    CHECK(g.r.onMessageBox(box()) == Route::Local);
    CHECK(g.raised.empty());
}

// ---- service menus -----------------------------------------------------------------------------------------------------------
TEST(service_menus_route_by_origin) {
    Rig g;
    ServiceMenuCall c;
    c.kind = MenuKind::Barter;
    c.thisObj = 0x44;
    CHECK(g.r.onServiceMenu(c) == Route::Local);
    OriginScope s(g.r.origin(), OriginContext::Kind::Dialogue, 1);
    CHECK(g.r.onServiceMenu(c) == Route::Suppressed);
    REQUIRE(g.raised.size() == 1);
    CHECK(g.raised[0].kind == MenuKind::Barter && g.raised[0].origin == 1 && g.raised[0].subject.localId == 0x44);
}

// ---- conditions --------------------------------------------------------------------------------------------------------------
TEST(condition_view_is_per_evaluating_slot) {
    Rig g;
    CHECK(g.r.viewQuestVar(0x500, 47, 0.0) == 0.0);  // host evaluating: no humanoid for the host's slot
    {
        OriginScope s(g.r.origin(), OriginContext::Kind::Dialogue, 2);
        CHECK(g.r.viewQuestVar(0x500, 47, 0.0) == 1.0);  // result script running for slot 2
    }
    CHECK(g.r.viewQuestVar(0x500, 47, 0.0) == 0.0);
    CHECK(g.r.stats().condChanged == 1);
}
TEST(condition_view_ignores_unwatched_quests_and_other_vars) {
    Rig g;
    OriginScope s(g.r.origin(), OriginContext::Kind::Dialogue, 2);
    CHECK(g.r.viewQuestVar(0x501, 47, 5.0) == 5.0);  // not watched
    CHECK(g.r.viewQuestVar(0x500, 12, 5.0) == 5.0);  // watched quest, var without a rule
    CHECK(g.r.stats().condViews == 1);
}
TEST(condition_view_shows_each_player_its_own_float_variable) {
    Rig g;
    CHECK(g.r.viewQuestVar(0x600, 3, 9.5) == 0.25);                                       // the host's own copy, a float kept as a float
    {
        OriginScope s(g.r.origin(), OriginContext::Kind::Dialogue, 1);
        CHECK(g.r.viewQuestVar(0x600, 3, 9.5) == 1.25);                                   // a guest's result script reads the guest's copy
    }
    {
        OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
        CHECK(g.r.viewQuestVar(0x600, 3, 9.5) == 2.25 && g.r.viewQuestVar(0x600, 8, 9.5) == 9.5);   // a variable without a copy is the real value
    }
    CHECK(g.r.viewQuestVar(0x601, 3, 9.5) == 9.5 && g.r.viewQuestVar(0x601, 3, 9.5) == 9.5);   // a quest without per-player variables is untouched ...
    CHECK(g.playerVarAsks == 2);                                                          // ... and each quest is only asked about once (conditions run every frame)
    g.r.invalidateViews();
    CHECK(g.r.viewQuestVar(0x600, 3, 9.5) == 0.25 && g.playerVarAsks == 3);
}
TEST(condition_view_without_bridge_is_identity) {
    FakeEngine e;
    UiRouter r(e, Bridge());
    CHECK(r.viewQuestVar(0x500, 47, 3.0) == 3.0);
}
TEST(condition_local_slot_on_a_client) {
    Rig g;
    g.r.origin().setLocalSlot(2);  // this machine is player 2: its conditions see its own view without any scope
    CHECK(g.r.viewQuestVar(0x500, 47, 0.0) == 1.0);
}

// ---- forced dialogue ---------------------------------------------------------------------------------------------------------
static ForcedCall sayTo(uint32_t target, uint32_t speaker = 0x600) {
    ForcedCall c;
    c.kind = ForcedCall::Kind::SayTo;
    c.speaker = speaker;
    c.target = target;
    c.topic = FormRef("falloutnv.esm", 0x700);
    c.info = FormRef("falloutnv.esm", 0x701);
    fnvcoop_dlg::Line l;
    l.text = "Welcome.";
    c.lines.push_back(l);
    return c;
}
TEST(sayto_player_inside_acting_scope_targets_that_player) {
    Rig g;
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    auto o = g.r.onForced(sayTo(0x14));
    CHECK(o.captured && o.target == 2);
    REQUIRE(g.said.size() == 1);
    CHECK(g.said[0].target == 2 && !g.said[0].scene);
    CHECK(g.said[0].lines.size() == 1 && g.said[0].info.localId == 0x701);
}
TEST(sayto_player_outside_any_scope_is_a_party_scene) {
    Rig g;
    auto o = g.r.onForced(sayTo(0x14));
    CHECK(o.captured && o.target == kNoSlot);
    REQUIRE(g.said.size() == 1);
    CHECK(g.said[0].scene && g.said[0].target == kNoSlot);
}
TEST(sayto_a_guest_ghost_targets_the_guest) {
    Rig g;
    auto o = g.r.onForced(sayTo(0x2001));
    CHECK(o.captured && o.target == 2);
    REQUIRE(g.said.size() == 1);
    CHECK(!g.said[0].scene && g.said[0].target == 2);
}
TEST(sayto_npc_to_npc_is_not_forwarded) {
    Rig g;
    auto o = g.r.onForced(sayTo(0x9999));
    CHECK(!o.captured);
    CHECK(g.said.empty());
    CHECK(g.r.stats().forcedIgnored == 1);
}
TEST(say_is_forwarded_as_party_line_without_scope) {
    Rig g;
    ForcedCall c = sayTo(0);
    c.kind = ForcedCall::Kind::Say;
    auto o = g.r.onForced(c);
    CHECK(o.captured);
    REQUIRE(g.said.size() == 1);
    CHECK(g.said[0].scene);
}
TEST(player_speaking_is_not_an_npc_line) {
    Rig g;
    ForcedCall c = sayTo(0x14, 0x14);
    CHECK(!g.r.onForced(c).captured);
}
TEST(startconversation_for_remote_actor_vetoes_host_dialog) {
    Rig g;
    g.convActor = 2;
    ForcedCall c;
    c.kind = ForcedCall::Kind::StartConversation;
    c.speaker = 0x600;
    c.target = 0x14;
    c.topic = FormRef("falloutnv.esm", 0x700);
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    auto o = g.r.onForced(c);
    CHECK(o.captured && o.vetoNative && o.target == 2);
    CHECK(g.convs.size() == 1);
}
TEST(startconversation_for_host_actor_runs_natively) {
    Rig g;
    g.convActor = kHost;
    ForcedCall c;
    c.kind = ForcedCall::Kind::StartConversation;
    c.speaker = 0x600;
    c.target = 0x14;
    c.topic = FormRef("falloutnv.esm", 0x700);
    auto o = g.r.onForced(c);  // no scope: party scene, L8 picks the actor (the host here)
    CHECK(o.captured && !o.vetoNative);
    CHECK(g.convs.size() == 1);
}
TEST(forced_on_a_guest_machine_does_nothing) {
    Rig g;
    g.r.setHost(false);
    CHECK(!g.r.onForced(sayTo(0x14)).captured);
    CHECK(g.said.empty());
}

// ---- slideshow ---------------------------------------------------------------------------------------------------------------
TEST(slides_are_recorded_in_order_and_forwarded) {
    Rig g;
    g.r.onSwapTexture(0x800, "a.dds", "b.dds");
    g.r.onSwapTexture(0x800, "b.dds", "c.dds");
    g.r.onSwapTexture(0x801, "x.dds", "y.dds");
    REQUIRE(g.r.slides().size() == 3);
    CHECK(g.r.slides()[0].seq == 1 && g.r.slides()[0].newTex == "b.dds");
    CHECK(g.r.slides()[2].seq == 3 && g.r.slides()[2].newTex == "y.dds");
    REQUIRE(g.slidesSeen.size() == 3);
    CHECK(std::get<1>(g.slidesSeen[1]) == "c.dds");
    CHECK(g.r.slides()[1].seq == 2);
}
TEST(slides_ignored_on_guest) {
    Rig g;
    g.r.setHost(false);
    g.r.onSwapTexture(0x800, "a.dds", "b.dds");
    CHECK(g.r.slides().empty());
}

TEST(default_choice) {
    CHECK(defaultChoiceFor(0) == -1);
    CHECK(defaultChoiceFor(1) == 0);
    CHECK(defaultChoiceFor(2) == 1);
    CHECK(defaultChoiceFor(4) == 3);
}

// ---- G07: a MenuMode 1001 block needs the box open on the machine that runs the script ----------------------------------------------
static bool hasBlock(const FormRef& s) { return s.localId == 0x137622; }
static MessageBoxCall boxOf(uint32_t scriptForm, uint32_t script = 77) {
    MessageBoxCall c = box(script);
    c.script = FormRef("falloutnv.esm", scriptForm);
    return c;
}
TEST(menumode_box_is_mirrored_on_the_host_until_the_guest_answers) {
    Rig g;
    g.r.setMenuMode(UiRouter::MenuModeMode::Mirror, hasBlock);
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    const Route routed = g.r.onMessageBox(boxOf(0x137622));
    REQUIRE(routed == Route::Suppressed);
    CHECK(g.eng.mirrorsOpen.size() == 1 && g.eng.mirrorsOpen[0] == 1 && g.r.mirrors() == 1);
    CHECK(g.eng.lastButton() == kNoButton && g.eng.mirrorsClosed.empty());   // the block polls GetButtonPressed: nothing yet
    g.r.onMenuResume(1, 1, false);
    CHECK(g.eng.lastButton() == 1);                                           // the answer reaches the script...
    CHECK(g.eng.mirrorsClosed.size() == 1 && g.r.mirrors() == 0);             // ...and the host's copy goes away
    g.r.onMenuResume(1, 1, false);                                            // a duplicate answer closes nothing twice
    CHECK(g.eng.mirrorsClosed.size() == 1);
}
TEST(menumode_off_logs_the_dead_block_once_and_mirrors_nothing) {
    Rig g;
    g.r.setMenuMode(UiRouter::MenuModeMode::Off, hasBlock);
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    g.r.onMessageBox(boxOf(0x137622));
    g.r.onMenuResume(1, 0, false);
    g.r.onMessageBox(boxOf(0x137622));
    CHECK(g.eng.mirrorsOpen.empty() && g.eng.logs.size() == 1 && g.r.stats().menuModeSkipped == 1);
}
TEST(menumode_mirror_only_for_scripts_with_a_block_and_remote_origins) {
    Rig g;
    g.r.setMenuMode(UiRouter::MenuModeMode::Mirror, hasBlock);
    {
        OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
        g.r.onMessageBox(boxOf(0x999999, 91));                                // a script without a MenuMode block
    }
    CHECK(g.eng.mirrorsOpen.empty());
    CHECK(g.r.onMessageBox(boxOf(0x137622, 92)) == Route::Local);            // the host plays: its own box already is the menu
    CHECK(g.eng.mirrorsOpen.empty());
    g.resumeSync = true;                                                      // answered before the router returned: no mirror to leave behind
    OriginScope s2(g.r.origin(), OriginContext::Kind::Acting, 2);
    g.r.onMessageBox(boxOf(0x137622, 93));
    CHECK(g.eng.mirrorsOpen.empty());
}

TEST(forced_lines_carry_the_voice_token_of_their_topic_and_info) {
    Rig g;
    OriginScope s(g.r.origin(), OriginContext::Kind::Acting, 2);
    ForcedCall c = sayTo(0x14);
    c.lines.push_back(c.lines[0]);
    g.r.onForced(c);
    REQUIRE(g.said.size() == 1 && g.said[0].lines.size() == 2);
    VoiceToken v;
    REQUIRE(parseVoiceToken(g.said[0].lines[1].voice, v));
    CHECK(v.topic.localId == 0x700 && v.info.localId == 0x701 && v.response == 2);
    REQUIRE(parseVoiceToken(g.said[0].lines[0].voice, v));
    CHECK(v.response == 1);
}

// ---- engine address pins -----------------------------------------------------------------------------------------------------
struct Pin {
    const char* name;
    unsigned addr;
    const char* hex;
    int stolen, ret;
    bool hook;
};
static const Pin kPins[] = {
#include "pins.inc"
};
static std::string hexOf(const unsigned char* b, size_t n) {
    static const char* d = "0123456789abcdef";
    std::string s;
    for (size_t i = 0; i < n; ++i) {
        s += d[b[i] >> 4];
        s += d[b[i] & 15];
    }
    return s;
}
TEST(pins_match_generated_header) {
    size_t seenHooks = 0, seenCalls = 0;
    for (const Pin& p : kPins) {
        if (p.hook) {
            const nvhooks::HookSpec* found = nullptr;
            for (auto* h : addr::kAllHooks)
                if (std::strcmp(h->name, p.name) == 0) found = h;
            CHECK(found != nullptr);
            if (!found) continue;
            ++seenHooks;
            CHECK(found->target == p.addr);
            CHECK(hexOf(found->expected, 16) == p.hex);
            CHECK(found->stolen == p.stolen);
            CHECK(found->retBytes == p.ret);
            CHECK(found->expectedLen == 16);
            std::string why;
            CHECK(nvhooks::specIsValid(*found, &why));
        } else {
            const addr::CallSpec* found = nullptr;
            for (auto* c : addr::kAllCalls)
                if (std::strcmp(c->name, p.name) == 0) found = c;
            CHECK(found != nullptr);
            if (!found) continue;
            ++seenCalls;
            CHECK(found->addr == p.addr);
            CHECK(hexOf(found->bytes, 16) == p.hex);
        }
    }
    CHECK(seenHooks == sizeof(addr::kAllHooks) / sizeof(addr::kAllHooks[0]));
    CHECK(seenCalls == sizeof(addr::kAllCalls) / sizeof(addr::kAllCalls[0]));
    // every hook must be a plain cdecl `ret` function: the detours are written as cdecl (static_assert in ui_hooks.cpp as well)
    for (auto* h : addr::kAllHooks) CHECK(h->retBytes == 0);
    // no two hooks on the same entry
    for (size_t i = 0; i < sizeof(addr::kAllHooks) / sizeof(addr::kAllHooks[0]); ++i)
        for (size_t j = i + 1; j < sizeof(addr::kAllHooks) / sizeof(addr::kAllHooks[0]); ++j) CHECK(addr::kAllHooks[i]->target != addr::kAllHooks[j]->target);
}

// Compare with the real image when NVUI_EXE points at a dump of the running game image (PE section walk, image base 0x400000).
TEST(pins_match_real_exe_if_available) {
    const char* path = std::getenv("NVUI_EXE");
    if (!path || !*path) {
        std::printf("  (NVUI_EXE not set: real-image check skipped)\n");
        return;
    }
    std::ifstream f(path, std::ios::binary);
    std::vector<unsigned char> d((std::istreambuf_iterator<char>(f)), std::istreambuf_iterator<char>());
    CHECK(d.size() > 0x1000);
    if (d.size() < 0x1000) return;
    auto u32 = [&](size_t o) { uint32_t v; std::memcpy(&v, &d[o], 4); return v; };
    auto u16 = [&](size_t o) { uint16_t v; std::memcpy(&v, &d[o], 2); return v; };
    size_t e = u32(0x3C);
    size_t nsec = u16(e + 6), opt = u16(e + 20);
    auto rd = [&](unsigned a, unsigned char* out) {
        for (size_t i = 0; i < nsec; ++i) {
            size_t o = e + 24 + opt + i * 40;
            unsigned vs = u32(o + 8), va = u32(o + 12) + 0x400000, rs = u32(o + 16), rp = u32(o + 20);
            if (a >= va && a < va + (vs > rs ? vs : rs)) {
                std::memcpy(out, &d[rp + (a - va)], 16);
                return true;
            }
        }
        return false;
    };
    int n = 0;
    for (const Pin& p : kPins) {
        unsigned char b[16];
        CHECK(rd(p.addr, b));
        CHECK(hexOf(b, 16) == p.hex);
        ++n;
    }
    std::printf("  real image: %d prologues compared\n", n);
}

int main() {
    std::printf("hooks_ui tests (FNVCOOP_MUT=%d)\n", fnvcoop_mut::current());
    run("origin_scope_wins_and_nests", origin_scope_wins_and_nests);
    run("origin_activation_and_sticky_expire", origin_activation_and_sticky_expire);
    run("box_for_host_origin_stays_local", box_for_host_origin_stays_local);
    run("box_for_guests_is_raised_to_origin_and_resumes_with_choice", box_for_guests_is_raised_to_origin_and_resumes_with_choice);
    run("box_first_button_is_zero", box_first_button_is_zero);
    run("box_resume_is_one_shot", box_resume_is_one_shot);
    run("box_synchronous_resume_during_raise", box_synchronous_resume_during_raise);
    run("box_cancel_and_out_of_range_leave_no_button", box_cancel_and_out_of_range_leave_no_button);
    run("box_without_buttons_answers_zero", box_without_buttons_answers_zero);
    run("box_point_of_no_return_uses_modal", box_point_of_no_return_uses_modal);
    run("box_follow_up_of_same_script_follows_origin", box_follow_up_of_same_script_follows_origin);
    run("box_on_guest_machine_is_always_local", box_on_guest_machine_is_always_local);
    run("service_menus_route_by_origin", service_menus_route_by_origin);
    run("condition_view_is_per_evaluating_slot", condition_view_is_per_evaluating_slot);
    run("condition_view_ignores_unwatched_quests_and_other_vars", condition_view_ignores_unwatched_quests_and_other_vars);
    run("condition_view_without_bridge_is_identity", condition_view_without_bridge_is_identity);
    run("condition_view_shows_each_player_its_own_float_variable", condition_view_shows_each_player_its_own_float_variable);
    run("condition_local_slot_on_a_client", condition_local_slot_on_a_client);
    run("sayto_player_inside_acting_scope_targets_that_player", sayto_player_inside_acting_scope_targets_that_player);
    run("sayto_player_outside_any_scope_is_a_party_scene", sayto_player_outside_any_scope_is_a_party_scene);
    run("sayto_a_guest_ghost_targets_the_guest", sayto_a_guest_ghost_targets_the_guest);
    run("sayto_npc_to_npc_is_not_forwarded", sayto_npc_to_npc_is_not_forwarded);
    run("say_is_forwarded_as_party_line_without_scope", say_is_forwarded_as_party_line_without_scope);
    run("player_speaking_is_not_an_npc_line", player_speaking_is_not_an_npc_line);
    run("startconversation_for_remote_actor_vetoes_host_dialog", startconversation_for_remote_actor_vetoes_host_dialog);
    run("startconversation_for_host_actor_runs_natively", startconversation_for_host_actor_runs_natively);
    run("forced_on_a_guest_machine_does_nothing", forced_on_a_guest_machine_does_nothing);
    run("slides_are_recorded_in_order_and_forwarded", slides_are_recorded_in_order_and_forwarded);
    run("slides_ignored_on_guest", slides_ignored_on_guest);
    run("default_choice", default_choice);
    run("party_box_decided_by_the_host_is_still_raised_to_the_party", party_box_decided_by_the_host_is_still_raised_to_the_party);
    run("menumode_box_is_mirrored_on_the_host_until_the_guest_answers", menumode_box_is_mirrored_on_the_host_until_the_guest_answers);
    run("menumode_off_logs_the_dead_block_once_and_mirrors_nothing", menumode_off_logs_the_dead_block_once_and_mirrors_nothing);
    run("menumode_mirror_only_for_scripts_with_a_block_and_remote_origins", menumode_mirror_only_for_scripts_with_a_block_and_remote_origins);
    run("forced_lines_carry_the_voice_token_of_their_topic_and_info", forced_lines_carry_the_voice_token_of_their_topic_and_info);
    run("pins_match_generated_header", pins_match_generated_header);
    run("pins_match_real_exe_if_available", pins_match_real_exe_if_available);
    std::printf("%d checks, %d failed\n", g_checks, g_fail);
    return g_fail ? 1 : 0;
}
