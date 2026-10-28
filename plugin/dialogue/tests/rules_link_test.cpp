// Wave 1b (G08 / G27): the dialogue host linked to a REAL RulesEngine (rules_link.h). A ReadyCheck INFO (House) and a ChoiceConsent INFO (Daniel, Elijah) hold their END
// script, the other players answer a two-button box, the answers become the engine's votes (ScenePass / choiceDeclineId) and the engine's verdict releases or drops the script.
// Built with the synccore sources (run_tests.py); the mutation twins are the fault switches of mutations.h, run in this same binary.
#include <cstdio>
#include <memory>
#include <string>
#include <vector>

#include "fnvcoop/rules.h"
#include "fnvcoop/session.h"
#include "fnvcoop/transport.h"
#include "fnvcoop_dlg/host.h"
#include "fnvcoop_dlg/rules_link.h"

using namespace fnvcoop_dlg;

static std::string g_why;
#define CHECK(c) do { if (!(c)) { g_why = std::string("line ") + std::to_string(__LINE__) + " " #c; return false; } } while (0)

namespace {
uint32_t h32(const std::string& s) {
    uint32_t h = 2166136261u;
    for (char c : s) { h ^= uint8_t(c); h *= 16777619u; }
    return h;
}
bool resolveEdid(const std::string& edid, FormRef& out) { out = FormRef("falloutnv.esm", h32(edid)); return true; }
const FormRef kHouse("falloutnv.esm", 0x15EB69);          // ReadyCheck INFO of VDialogueMrHouse
const FormRef kDaniel("honesthearts.esm", 0x010718);      // ChoiceConsent INFO of NVDLC02DialogueDaniel (vote)
const FormRef kNpc("falloutnv.esm", 0x2BBB), kTopic("falloutnv.esm", 0x1000);

const char* kRules = R"({"schema":1,"quests":{
 "VDialogueMrHouse":{"tier":"T4","questline":"x","curated":true,"rules":[
   {"kind":"ReadyCheck","params":{"infos":["FalloutNV.esm:15EB69"],"timeout_s":120,"on_timeout":"cancel","confirm":"all-players"},"origin":"questsweep"}]},
 "Casino":{"tier":"T3","questline":"x","curated":false,"rules":[{"kind":"PerPlayerVars","params":{"vars":["Banned"]},"origin":"derived"}]},
 "NVDLC02DialogueDaniel":{"tier":"T4","questline":"x","curated":true,"rules":[
   {"kind":"ChoiceConsent","params":{"topics":["HonestHearts.esm:010718"],"mode":"vote","timeout_s":90},"origin":"questsweep"}]}
}})";

struct NullSink : fnvcoop::IEffectSink {
    void apply(const fnvcoop::Effect&, const Context&) override {}
};

// The stand-in for DialogueNode: the same four members rules_link.h needs, collecting what the host emits.
struct TestNode {
    std::vector<Out> out;
    int endRuns = 0, beginRuns = 0;
    HostDialogue hd;
    TestNode() : hd([this](const ResultRequest& r) { (r.phase == Phase::End ? endRuns : beginRuns)++; return uint64_t(100 + endRuns + beginRuns); }) {
        for (PlayerId p = 0; p < 3; ++p) take(hd.playerJoined(p));
    }
    void take(std::vector<Out> v) { for (auto& o : v) out.push_back(std::move(o)); }
    void setRulesGates(HostDialogue::RulesGate a, HostDialogue::RulesGate b) { hd.setRulesGates(std::move(a), std::move(b)); }
    void setVoteSink(HostDialogue::VoteSink s) { hd.setVoteSink(std::move(s)); }
    void rulesResolved(const FormRef& ref, bool accepted, uint32_t now) { take(hd.rulesResolved(ref, accepted, now)); }
    HostDialogue* host() { return &hd; }
    void send(PlayerId from, const Msg& m, uint32_t now = 0) { take(hd.onMessage(from, m, now)); }
    // the newest consent MenuRequest addressed to `slot` (0 = none)
    uint32_t askedMenu(PlayerId slot) const {
        for (size_t i = out.size(); i-- > 0;)
            if (const auto* rq = std::get_if<MenuRequest>(&out[i].msg)) if ((out[i].to >> slot) & 1u) return rq->menu;
        return 0;
    }
    bool closedWith(CloseReason r) const {
        for (auto& o : out) if (const auto* c = std::get_if<Closed>(&o.msg)) if (c->reason == r) return true;
        return false;
    }
};

// A host with `guests` joined ClientSessions behind a RulesEngine over the test rules, linked to a TestNode.
struct Rig {
    fnvcoop::InMemoryNetwork net{5};
    fnvcoop::HostSession session{*net.endpoint(0)};
    std::vector<std::unique_ptr<NullSink>> sinks;
    std::vector<std::unique_ptr<fnvcoop::EffectCapture>> caps;
    std::vector<std::unique_ptr<fnvcoop::ClientSession>> clients;
    fnvcoop::RulesDb db;
    std::unique_ptr<fnvcoop::RulesEngine> eng;
    TestNode node;
    uint64_t now = 0;
    explicit Rig(int guests) {
        for (int i = 0; i < guests; ++i) {
            sinks.push_back(std::make_unique<NullSink>());
            caps.push_back(std::make_unique<fnvcoop::EffectCapture>());
            clients.push_back(std::make_unique<fnvcoop::ClientSession>(*net.endpoint(fnvcoop::PeerId(i + 1)), 0, 1000 + i, *sinks.back(), caps.back().get()));
            clients.back()->connect(now, clients.back()->lastSession() + 1);
        }
        db.load(kRules);
        eng.reset(new fnvcoop::RulesEngine(session, db, resolveEdid));
        session.onAction = [this](fnvcoop::PlayerId s, fnvcoop::IntentKind k, const fnvcoop::FormRef& r, uint32_t, uint32_t) { eng->handleAction(s, k, r); };
        run(400);
        linkRules(node, *eng, [this] { return uint32_t(now); });
    }
    void run(uint32_t ms) {
        for (uint32_t t = 0; t < ms; t += 5) {
            now += 5; net.advance(now); session.tick(now);
            for (auto& c : clients) c->tick(now);
            net.advance(now); eng->tick(now);
        }
    }
    // initiator `who` picks `info` and finishes the line
    void say(PlayerId who, const FormRef& info) {
        node.send(who, StartReq{1, kNpc});
        Pick p; p.conv = 1; p.step = 1; p.topic = kTopic; p.info = info; p.choice = 0; Line l; l.text = "Go."; p.lines = {l};
        node.send(who, p);
        node.send(who, LineDone{1, 1});
    }
};

bool scn_ready_check_waits_for_everybody_then_runs_the_end_script() {
    Rig r(2);
    r.say(1, kHouse);                                                             // guest 1 talks to the NPC and reaches the irreversible line
    CHECK(r.node.beginRuns == 1 && r.node.endRuns == 0);                          // the END script is held by the ReadyCheck
    const uint32_t m0 = r.node.askedMenu(0), m2 = r.node.askedMenu(2);
    CHECK(m0 != 0 && m2 != 0 && r.node.askedMenu(1) == 0);                        // the others are asked, the initiator is not
    r.node.send(0, MenuReply{m0, 0});                                              // the host is ready
    CHECK(r.node.endRuns == 0 && r.node.host()->heldByRules() == 1);              // everybody is needed
    r.node.send(2, MenuReply{m2, 0});                                              // so is guest 2
    CHECK(r.node.endRuns == 1 && r.node.host()->heldByRules() == 0);              // the engine released it, the END script ran exactly once
    return true;
}
bool scn_ready_check_timeout_cancels_and_tells_the_initiator() {
    Rig r(2);
    r.say(1, kHouse);
    r.node.send(0, MenuReply{r.node.askedMenu(0), 0});
    r.run(125000);                                                                // guest 2 never answers: the engine times the check out (cancel)
    CHECK(r.node.endRuns == 0 && r.node.host()->heldByRules() == 0);
    CHECK(r.node.closedWith(CloseReason::ConsentDenied));
    return true;
}
bool scn_choice_vote_majority_agrees() {
    Rig r(2);
    r.say(1, kDaniel);
    r.node.send(0, MenuReply{r.node.askedMenu(0), 0});                            // the initiator (asking is agreeing) + the host = 2 of 3
    CHECK(r.node.endRuns == 1 && !r.node.closedWith(CloseReason::ConsentDenied));
    return true;
}
bool scn_choice_vote_majority_declines() {
    Rig r(2);
    r.say(1, kDaniel);
    r.node.send(0, MenuReply{r.node.askedMenu(0), 1});                            // Not yet
    CHECK(r.node.endRuns == 0 && r.node.host()->heldByRules() == 1);              // one no does not decide a vote of three
    r.node.send(2, MenuReply{r.node.askedMenu(2), 1});                            // the second no is a majority
    CHECK(r.node.endRuns == 0 && r.node.host()->heldByRules() == 0 && r.node.closedWith(CloseReason::ConsentDenied));
    return true;
}
bool scn_a_party_of_one_is_not_asked() {
    Rig r(0);                                                                     // the host plays alone
    r.node.send(0, StartReq{1, kNpc});
    Pick p; p.conv = 1; p.step = 1; p.topic = kTopic; p.info = kHouse; Line l; l.text = "Go."; p.lines = {l};
    r.node.send(0, p);
    r.node.send(0, LineDone{1, 1});
    CHECK(r.node.endRuns == 1 && r.node.askedMenu(1) == 0);
    return true;
}
bool scn_other_infos_run_at_once() {
    Rig r(2);
    r.say(1, FormRef("falloutnv.esm", 0x777));
    CHECK(r.node.endRuns == 1 && r.node.host()->heldByRules() == 0);
    return true;
}
bool scn_old_consent_table_does_not_ask_twice() {
    Rig r(2);   // the table asked for the same INFO at the pick; once the engine owns it, only the engine asks
    std::map<FormRef, HostDialogue::ChoiceRule> table;
    table[kDaniel] = HostDialogue::ChoiceRule();
    r.node.host()->setChoiceRules(table);
    linkRules(r.node, *r.eng, [&r] { return uint32_t(r.now); });
    r.say(1, kDaniel);
    CHECK(r.node.beginRuns == 1);                                                  // not held at the pick
    return true;
}

bool scn_condition_views_follow_the_rules_engine() {
    Rig r(2);
    RulesViews v = rulesViews(*r.eng);
    FormRef casino, other;
    resolveEdid("Casino", casino);
    resolveEdid("Other", other);
    r.eng->writePlayerVar(casino, "Banned", 1, 7.0);                                  // guest 1 was banned from the casino, the host and guest 2 were not
    CHECK(v.hasPlayerVars(casino) && !v.hasPlayerVars(other));
    CHECK(v.viewPlayerVar(casino, "Banned", 1, 0.0) == 7.0);                         // guest 1 sees its own copy ...
    CHECK(v.viewPlayerVar(casino, "Banned", 2, 3.0) == 3.0);                         // ... guest 2 has none: the real value
    CHECK(v.viewPlayerVar(casino, "NoSuchVar", 1, 5.0) == 5.0 && v.viewPlayerVar(other, "Banned", 1, 5.0) == 5.0);
    CHECK(v.isRulesChoice(kDaniel) && !v.isRulesChoice(kHouse) && !v.isRulesChoice(FormRef("falloutnv.esm", 0x4D0001)));   // (a ReadyCheck INFO is no choice)
    return true;
}

struct Case { const char* name; bool (*fn)(); const char* twin; Mutations mut; };
template <class F> Mutations M(F f) { Mutations m; f(m); return m; }
}  // namespace

int main() {
    const Case cases[] = {
        {"ready_check_waits_for_everybody_then_runs_the_end_script", scn_ready_check_waits_for_everybody_then_runs_the_end_script, "no_info_gate", M([](Mutations& m) { m.noInfoGate = true; })},
        {"ready_check_release", scn_ready_check_waits_for_everybody_then_runs_the_end_script, "held_end_never_released", M([](Mutations& m) { m.heldEndNeverReleased = true; })},
        {"ready_check_answers_are_votes", scn_ready_check_waits_for_everybody_then_runs_the_end_script, "vote_answer_dropped", M([](Mutations& m) { m.voteAnswerDropped = true; })},
        {"ready_check_timeout_cancels", scn_ready_check_timeout_cancels_and_tells_the_initiator, "held_end_runs_on_deny", M([](Mutations& m) { m.heldEndRunsOnDeny = true; })},
        {"choice_vote_majority_agrees", scn_choice_vote_majority_agrees, "vote_answer_inverted", M([](Mutations& m) { m.voteAnswerInverted = true; })},
        {"choice_vote_majority_declines", scn_choice_vote_majority_declines, "decline_run", M([](Mutations& m) { m.heldEndRunsOnDeny = true; })},
        {"choice_vote_decline_is_a_decline", scn_choice_vote_majority_declines, "inverted_decline", M([](Mutations& m) { m.voteAnswerInverted = true; })},
        {"condition_views_follow_the_rules_engine", scn_condition_views_follow_the_rules_engine, nullptr, {}},
        {"party_of_one_is_not_asked", scn_a_party_of_one_is_not_asked, nullptr, {}},
        {"other_infos_run_at_once", scn_other_infos_run_at_once, nullptr, {}},
        {"old_consent_table_does_not_ask_twice", scn_old_consent_table_does_not_ask_twice, "old_hold_kept", M([](Mutations& m) { m.rulesOwnerKeepsOldHold = true; })},
    };
    int fail = 0, ran = 0;
    for (const Case& c : cases) {
        g_why.clear();
        bool ok = c.fn();
        std::printf("[%s] unit %s%s%s\n", ok ? "PASS" : "FAIL", c.name, ok ? "" : " : ", ok ? "" : g_why.c_str());
        ++ran; fail += !ok;
        if (c.twin) {
            g_why.clear();
            bool survived;
            { MutationScope ms(c.mut); survived = c.fn(); }
            std::printf("[%s] twin %s -> %s (%s)\n", survived ? "FAIL" : "PASS", c.twin, c.name, survived ? "mutant survived" : g_why.c_str());
            ++ran; fail += survived;
        }
    }
    std::printf("%d run, %d failed\n", ran, fail);
    return fail ? 1 : 0;
}
