// Adapter tests: dialogue over L4's InMemoryNetwork (latency, duplication) with 3 nodes (host + 2 clients).
// Usage: adapter_tests [--group unit|twin|all]
#include <cstdio>
#include <cstring>
#include <string>
#include <tuple>
#include "fnvcoop/transport.h"
#include "fnvcoop_dlg/adapter.h"

using namespace fnvcoop_dlg;
static std::string g_why;
#define CHECK(c) do { if (!(c)) { g_why = std::string("line ") + std::to_string(__LINE__) + " " #c; return false; } } while (0)
static FormRef F(uint32_t id) { return FormRef("falloutnv.esm", id); }

struct Rig {
    fnvcoop::InMemoryNetwork net;
    std::vector<std::unique_ptr<DialogueNode>> n;
    std::vector<std::vector<Event>> ev;
    std::vector<std::pair<ConvId, Phase>> runs;
    uint64_t now = 0;
    explicit Rig(uint32_t latency, double dup = 0, uint64_t seed = 7) : net(seed) {
        fnvcoop::LinkParams lp; lp.minLatencyMs = lp.maxLatencyMs = latency; lp.duplicate = dup;
        net.setLink(lp);
        ev.resize(3);
        for (PlayerId i = 0; i < 3; ++i) {
            DialogueNode::Config c; c.self = i; c.isHost = (i == 0); c.hostPeer = 0;
            c.peerOfSlot = [](PlayerId p) { return PeerId(p); };
            c.slotOfPeer = [](PeerId p) { return PlayerId(p); };
            ResultRunner run;
            if (i == 0) run = [this](const ResultRequest& r) { runs.push_back({r.conv, r.phase}); return uint64_t(runs.size()); };
            n.emplace_back(new DialogueNode(net.endpoint(i), c, run, [this, i](const Event& e) { ev[i].push_back(e); }));
        }
        n[0]->playerJoined(1); n[0]->playerJoined(2);
    }
    void step(uint64_t ms = 0) {
        for (uint64_t t = 0; t <= ms; t += 5) {
            now += (t ? 5 : 0);
            net.advance(now);
            for (PlayerId i = 0; i < 3; ++i) {
                PeerId from; fnvcoop::Bytes b;
                while (net.endpoint(i)->poll(from, b)) {
                    bool dlg = n[i]->onPacket(from, b, uint32_t(now));
                    if (!dlg) foreign++;
                }
                n[i]->tick(uint32_t(now));
            }
        }
    }
    int count(PlayerId p, Event::Kind k) const { int c = 0; for (auto& e : ev[p]) c += e.kind == k; return c; }
    int foreign = 0;
};

static bool t_roundtrip() {
    Rig r(40);
    r.n[1]->requestStart(F(0x104C0F), uint32_t(r.now));
    r.step(150);
    CHECK(r.n[1]->client().role() == ClientDialogue::Role::Initiator && r.count(1, Event::OpenMenu) == 1);
    CHECK(r.n[0]->client().role() == ClientDialogue::Role::Spectator && r.n[2]->client().role() == ClientDialogue::Role::Spectator);
    r.n[1]->pick(F(0x105560), F(0x105583), 0, false, {Line{"House.", "v.ogg", 0, 50}}, {}, uint32_t(r.now));
    r.step(150);
    CHECK(r.n[0]->client().history().size() == 1 && r.n[2]->client().history().size() == 1);   // host as spectator too
    r.n[1]->lineDone(uint32_t(r.now));
    r.step(150);
    CHECK(r.count(1, Event::TopicsReady) == 1);
    CHECK(r.runs.size() == 2);
    r.n[2]->requestStart(F(0x104C0F), uint32_t(r.now));            // parallel start on the same NPC
    r.step(150);
    CHECK(r.count(2, Event::Denied) == 1);
    r.n[1]->leave(uint32_t(r.now));
    r.step(150);
    CHECK(r.n[0]->host()->open() == 0 && r.n[1]->client().role() == ClientDialogue::Role::Idle);
    return true;
}

static bool t_host_initiator() {
    Rig r(40);
    r.n[0]->requestStart(F(0x104E85), uint32_t(r.now));              // loops back, no network hop for the grant
    r.step(100);
    CHECK(r.n[0]->client().role() == ClientDialogue::Role::Initiator && r.count(0, Event::OpenMenu) == 1);
    r.n[0]->pick(F(1), F(2), 0, true, {Line{"Bye.", "", 0, 50}}, {}, uint32_t(r.now));
    r.step(100);
    r.n[0]->lineDone(uint32_t(r.now));
    r.step(100);
    CHECK(r.count(1, Event::ShowLines) == 1);
    CHECK(r.n[1]->client().role() == ClientDialogue::Role::Idle && r.n[0]->host()->open() == 0);   // goodbye closed it
    return true;
}

static bool t_duplicates() {
    Rig r(30, 0.6, 99);
    r.n[1]->requestStart(F(0x104C0F), uint32_t(r.now));
    r.step(120);
    r.n[1]->pick(F(1), F(2), 0, false, {Line{"A", "", 0, 50}}, {}, uint32_t(r.now));
    r.step(120);
    r.n[1]->lineDone(uint32_t(r.now));
    r.step(120);
    CHECK(r.runs.size() == 2);                                      // each script exactly once despite duplicated packets
    CHECK(r.n[2]->client().history().size() == 1);
    return true;
}

static bool t_foreign_and_garbage() {
    Rig r(10);
    fnvcoop::Bytes l4 = {0xFC, 0x4E, 1, 0, 0, 0, 0, 0};            // an L4 envelope: must be passed through untouched
    CHECK(!DialogueNode::isDialoguePacket(l4));
    r.net.endpoint(1)->send(0, l4);
    fnvcoop::Bytes bad = {0xD1, 1, 3, 9, 9, 9, 9, 9, 9};            // dialogue magic, garbage body
    r.net.endpoint(1)->send(0, bad);
    r.step(50);
    CHECK(r.foreign == 1);
    CHECK(r.n[0]->badPackets() == 1);
    return true;
}

static bool t_view_var() {
    // Per-owner rewrite through the node: every machine has its own registry fed by replicated CompanionOwner effects.
    Rig r(10);
    const FormRef quest("falloutnv.esm", 0x0B16D0), boone("falloutnv.esm", 0x5001);
    for (PlayerId i = 0; i < 3; ++i) {
        registerVanillaFollowers(r.n[i]->companions(), quest, {{"Boone", boone}});
        r.n[i]->onCompanionOwner(boone, 1);
    }
    CHECK(r.n[1]->viewQuestVar(quest, vnpc::kHumanoidInParty, 0) == 1);   // owner sees a full humanoid slot
    CHECK(r.n[2]->viewQuestVar(quest, vnpc::kHumanoidInParty, 1) == 0);   // another player still can recruit
    CHECK(r.n[0]->viewQuestVarFor(quest, vnpc::kHumanoidInParty, 1, 0) == 1);   // host evaluating for the initiator
    CHECK(r.n[2]->viewQuestVar(quest, 99, 5) == 5);
    r.n[2]->onCompanionOwner(boone, fnvcoop::kNoPlayer);                  // dismissed (effect replicated)
    CHECK(r.n[2]->viewQuestVar(quest, vnpc::kCurrentFollowers, 3) == 0);
    return true;
}

static bool t_menu_over_transport() {
    Rig r(30);
    std::vector<std::tuple<uint32_t, int32_t, bool>> res;
    r.n[0]->setMenuResume([&](uint32_t m, int32_t c, bool t) { res.emplace_back(m, c, t); });
    uint32_t id = r.n[0]->raiseMenu(2, MenuKind::MessageBox, "Open the vault?", {"Yes", "No"}, 1, FormRef(), uint32_t(r.now));
    r.step(100);
    CHECK(r.count(2, Event::MenuShow) == 1 && r.count(1, Event::MenuShow) == 0 && r.count(0, Event::MenuShow) == 0);
    CHECK(r.count(1, Event::MenuNoticeShow) == 1 && r.count(0, Event::MenuNoticeShow) == 1);
    r.n[1]->menuReply(id, 0, uint32_t(r.now));                            // bystander answers: ignored (client has no such menu)
    r.step(100);
    CHECK(res.empty());
    r.n[2]->menuReply(id, 1, uint32_t(r.now));
    r.step(100);
    CHECK(res.size() == 1 && std::get<1>(res[0]) == 1 && !std::get<2>(res[0]));
    CHECK(r.count(0, Event::MenuEnd) == 1 && r.count(1, Event::MenuEnd) == 1 && r.count(2, Event::MenuEnd) == 1);
    return true;
}

static bool t_menu_duplicates() {
    Rig r(20, 0.7, 5);                                                    // heavy duplication must not resume twice
    int resumes = 0;
    r.n[0]->setMenuResume([&](uint32_t, int32_t, bool) { ++resumes; });
    uint32_t id = r.n[0]->raiseMenu(1, MenuKind::Barter, "Barter", {}, -1, FormRef("falloutnv.esm", 9), uint32_t(r.now));
    r.step(100);
    r.n[1]->menuReply(id, -1, uint32_t(r.now));
    r.step(150);
    CHECK(resumes == 1);
    return true;
}

// Story run S06: a guest's pick ran its result four times (ammo 60 became 240). A pick is run once per (conversation, step, phase), and the same INFO
// again with no topic list offered since is the line going on, not a second pick.
static bool t_pick_retries_run_once() {
    Rig r(30, 0.6, 5);                                               // duplicated packets plus three hand-made resends of the very same Pick
    r.n[1]->requestStart(F(0x104C0F), uint32_t(r.now));
    r.step(120);
    r.n[1]->pick(F(1), F(0x10A1EB), 0, true, {Line{"Shouldn't take more than a couple minutes.", "", 0, 50}, Line{"Come with me.", "", 0, 50}}, {}, uint32_t(r.now));
    Pick again;
    again.conv = r.n[1]->client().conv(); again.step = r.n[1]->client().step(); again.topic = F(1); again.info = F(0x10A1EB); again.goodbye = true;
    for (int i = 0; i < 3; ++i) { r.net.endpoint(1)->send(0, encode(Msg(again))); r.step(30); }
    r.step(120);
    CHECK(r.runs.size() == 1);                                       // the Begin script, once
    r.n[1]->lineDone(uint32_t(r.now));
    LineDone ld{again.conv, again.step};
    for (int i = 0; i < 3; ++i) { r.net.endpoint(1)->send(0, encode(Msg(ld))); r.step(30); }
    r.step(150);
    CHECK(r.runs.size() == 2 && r.n[0]->host()->scriptRuns() == 2);  // and the End script, once
    CHECK(r.n[0]->host()->open() == 0);
    return true;
}

static bool t_repeat_pick_without_list_runs_once() {
    Rig r(20);
    r.n[1]->requestStart(F(0x104C0F), uint32_t(r.now));
    r.step(100);
    r.n[1]->pick(F(1), F(0x10A1EB), 0, false, {Line{"Come with me.", "", 0, 50}}, {}, uint32_t(r.now));
    r.step(100);
    r.n[1]->lineDone(uint32_t(r.now));
    r.step(100);
    CHECK(r.runs.size() == 2 && r.count(1, Event::TopicsReady) == 1);
    // the engine shows the same INFO as spoken again (a response advance seen as a new pick): same INFO, no list offered since
    r.n[1]->pick(F(1), F(0x10A1EB), 0, false, {Line{"Come with me.", "", 0, 50}}, {}, uint32_t(r.now));
    r.step(100);
    r.n[1]->lineDone(uint32_t(r.now));
    r.step(100);
    CHECK(r.runs.size() == 2);                                       // nothing ran again
    CHECK(r.n[0]->host()->repeatPicks() == 1);
    CHECK(r.n[2]->client().history().size() == 1);                   // spectators saw the line once
    CHECK(r.count(1, Event::TopicsReady) == 2);                      // the initiator is not left waiting: both steps were answered
    // a real second pick of the same INFO: the list was offered in between
    r.n[1]->offer({TopicEntry{F(1), 0, 0}}, uint32_t(r.now));
    r.step(100);
    r.n[1]->pick(F(1), F(0x10A1EB), 0, false, {Line{"Come with me.", "", 0, 50}}, {}, uint32_t(r.now));
    r.step(100);
    r.n[1]->lineDone(uint32_t(r.now));
    r.step(100);
    CHECK(r.runs.size() == 4 && r.n[0]->host()->repeatPicks() == 1);
    return true;
}

struct Case { const char* name; bool (*fn)(); };
int main(int argc, char** argv) {
    (void)argc; (void)argv;
    Case cases[] = {{"adapter_roundtrip", t_roundtrip}, {"adapter_host_initiator", t_host_initiator},
                    {"adapter_duplicates", t_duplicates}, {"adapter_foreign_and_garbage", t_foreign_and_garbage},
                    {"adapter_view_var", t_view_var}, {"adapter_menu_over_transport", t_menu_over_transport}, {"adapter_menu_duplicates", t_menu_duplicates},
                    {"adapter_pick_retries_run_once", t_pick_retries_run_once}, {"adapter_repeat_pick_without_list_runs_once", t_repeat_pick_without_list_runs_once}};
    int fail = 0;
    for (auto& c : cases) {
        g_why.clear();
        bool ok = c.fn();
        std::printf("[%s] %s%s%s\n", ok ? "PASS" : "FAIL", c.name, ok ? "" : " : ", g_why.c_str());
        fail += !ok;
    }
    // twin: with the idempotency fix removed the duplicate scenario must fail
    { MutationScope ms([] { Mutations m; m.replayDuplicatePick = true; return m; }());
      g_why.clear();
      bool ok = t_duplicates();
      std::printf("[%s] twin replay_duplicate_pick -> adapter_duplicates (%s)\n", ok ? "FAIL" : "PASS", ok ? "mutant survived" : g_why.c_str());
      fail += ok; }
    // twin: a re-delivered Pick (retry, resend) re-runs the begin script
    { MutationScope ms([] { Mutations m; m.replayDuplicatePick = true; return m; }());
      g_why.clear();
      bool ok = t_pick_retries_run_once();
      std::printf("[%s] twin replay_duplicate_pick -> adapter_pick_retries_run_once (%s)\n", ok ? "FAIL" : "PASS", ok ? "mutant survived" : g_why.c_str());
      fail += ok; }
    // twin: a second pick of the INFO just run (no list offered in between) runs its scripts again
    { MutationScope ms([] { Mutations m; m.repeatPickRuns = true; return m; }());
      g_why.clear();
      bool ok = t_repeat_pick_without_list_runs_once();
      std::printf("[%s] twin repeat_pick_runs -> adapter_repeat_pick_without_list_runs_once (%s)\n", ok ? "FAIL" : "PASS", ok ? "mutant survived" : g_why.c_str());
      fail += ok; }
    std::printf("%d failed\n", fail);
    return fail ? 1 : 0;
}
