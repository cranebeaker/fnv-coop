// Dialogue (L8) unit tests + mutation twins. Build: see plugin/dialogue/run_tests.py or CMakeLists.txt.
// Usage: dialogue_tests [--group unit|twin|all] [--name <scenario>]
#include <algorithm>
#include <cstdio>
#include <cstring>
#include <deque>
#include <functional>
#include <map>
#include <sstream>
#include <tuple>
#include <string>
#include <vector>
#include "fnvcoop_dlg/client.h"
#include "fnvcoop_dlg/consent.h"
#include "fnvcoop_dlg/host.h"

using namespace fnvcoop_dlg;

static std::string g_why;
#define CHECK(c) do { if (!(c)) { g_why = std::string(__FILE__) + ":" + std::to_string(__LINE__) + " " #c; return false; } } while (0)

static FormRef F(uint32_t id) { return FormRef("falloutnv.esm", id); }
static Line L(const char* t) { Line l; l.text = t; l.voice = "vfreeformgoodsprings\\x.ogg"; return l; }

// ---- fake network: every message goes through encode/decode; supports latency, duplication, reordering
struct Net {
    struct Pkt { uint32_t at; PlayerId to; std::vector<uint8_t> bytes; };
    std::deque<Pkt> q;
    uint32_t now = 0, latency = 0;
    bool dupAll = false, reverse = false;
    void toClient(PlayerId to, const Msg& m) {
        auto b = encode(m);
        q.push_back({now + latency, to, b});
        if (dupAll) q.push_back({now + latency, to, b});
    }
};

struct World {
    Net net;
    std::vector<ClientDialogue> cl;  // index = slot (0 = host's own client side)
    std::vector<std::vector<Event>> log;
    struct Run { ConvId conv; uint32_t step; Phase ph; PlayerId init; std::vector<Fact> facts; };
    std::vector<Run> runs;
    uint64_t nextTxn = 100;
    HostDialogue host;
    uint32_t uplinkLatency = 0;
    std::deque<std::pair<uint32_t, std::pair<PlayerId, std::vector<uint8_t>>>> up;  // delayed client->host

    explicit World(int n, HostDialogue::Config c = {})
        : host([this](const ResultRequest& r) {
              runs.push_back({r.conv, r.step, r.phase, r.initiator, r.facts ? *r.facts : std::vector<Fact>()});
              return nextTxn++;
          }, c) {
        for (int i = 0; i < n; ++i) { cl.emplace_back(PlayerId(i)); host.playerJoined(PlayerId(i)); }
        log.resize(n);
    }
    void emit(PlayerId from, std::vector<Event> evs) {
        for (auto& e : evs) {
            log[from].push_back(e);
            if (e.kind == Event::Send) up.push_back({net.now + uplinkLatency, {from, encode(e.msg)}});
        }
    }
    void fan(const std::vector<Out>& outs) {
        for (auto& o : outs) for (int i = 0; i < (int)cl.size(); ++i) if ((o.to >> i) & 1) net.toClient(PlayerId(i), o.msg);
    }
    // Pump until quiescent at the current virtual time (or advance the clock by `ms` first).
    void pump(uint32_t ms = 0) {
        for (uint32_t t = 0; t < ms; t += 5) { net.now += 5; step(); }
        step();
    }
    void step() {
        for (int guard = 0; guard < 1000; ++guard) {
            bool did = false;
            for (size_t i = 0; i < up.size();) {
                if (up[i].first <= net.now) {
                    Msg m; auto& pr = up[i].second;
                    if (decode(pr.second, m) == DecodeStatus::Ok) fan(host.onMessage(pr.first, m, net.now));
                    up.erase(up.begin() + i); did = true;
                } else ++i;
            }
            std::vector<Net::Pkt> ready;
            for (size_t i = 0; i < net.q.size();) {
                if (net.q[i].at <= net.now) { ready.push_back(net.q[i]); net.q.erase(net.q.begin() + i); } else ++i;
            }
            if (net.reverse) std::reverse(ready.begin(), ready.end());
            for (auto& p : ready) {
                Msg m;
                if (decode(p.bytes, m) == DecodeStatus::Ok) emit(p.to, cl[p.to].onMessage(m, net.now));
                did = true;
            }
            if (!did) break;
        }
    }
    void tickAll() {
        for (size_t i = 0; i < cl.size(); ++i) emit(PlayerId(i), cl[i].tick(net.now));
        fan(host.tick(net.now));
    }
    int count(PlayerId p, Event::Kind k) const { int n = 0; for (auto& e : log[p]) n += e.kind == k; return n; }
    int runsOf(Phase ph) const { int n = 0; for (auto& r : runs) n += r.ph == ph; return n; }
};

static std::vector<Fact> someFacts() { return {{1, F(0x4BB), 55}, {2, F(0x0F), 120}}; }

// Drive one full line: pick -> (spectators get lines) -> lineDone -> end result.
static void sayLine(World& w, PlayerId who, const FormRef& topic, const FormRef& info, bool goodbye) {
    w.emit(who, w.cl[who].pick(topic, info, 0, goodbye, {L("Howdy.")}, someFacts()));
    w.pump();
    w.emit(who, w.cl[who].lineDone(w.net.now));
    w.pump();
}

// ============ scenarios: return true iff all invariants hold ============
static bool scn_happy(PlayerId initiator) {
    World w(3);
    FormRef npc = F(0x104C4);
    w.emit(initiator, w.cl[initiator].requestStart(npc));
    w.pump();
    CHECK(w.cl[initiator].role() == ClientDialogue::Role::Initiator);
    CHECK(w.count(initiator, Event::OpenMenu) == 1);
    for (PlayerId p = 0; p < 3; ++p) if (p != initiator) CHECK(w.cl[p].role() == ClientDialogue::Role::Spectator);
    w.emit(initiator, w.cl[initiator].offer({{F(0x1000), 1, 0}, {F(0x1001), 0, 0}}));
    w.pump();
    for (PlayerId p = 0; p < 3; ++p) if (p != initiator) CHECK(w.cl[p].spectatorTopics().size() == 2);
    CHECK(w.cl[initiator].topicsReady());
    sayLine(w, initiator, F(0x1000), F(0x2000), false);
    CHECK(w.runsOf(Phase::Begin) == 1 && w.runsOf(Phase::End) == 1);
    CHECK(w.runs[0].init == initiator && w.runs[0].facts.size() == 2);   // scripts evaluated against the initiator
    CHECK(w.count(initiator, Event::TopicsReady) == 1);
    for (PlayerId p = 0; p < 3; ++p) if (p != initiator) {
        CHECK(w.cl[p].history().size() == 1 && w.cl[p].history()[0].lines[0].text == "Howdy.");
    }
    CHECK(w.cl[initiator].history().empty());                            // initiator never gets its own lines echoed
    sayLine(w, initiator, F(0x1001), F(0x2001), true);
    CHECK(w.runsOf(Phase::Begin) == 2 && w.runsOf(Phase::End) == 2);
    for (PlayerId p = 0; p < 3; ++p) CHECK(w.cl[p].role() == ClientDialogue::Role::Idle);
    CHECK(!w.host.npcBusy(npc) && w.host.open() == 0);
    return true;
}
static bool scn_happy_client() { return scn_happy(1); }
static bool scn_happy_host() { return scn_happy(0); }

static bool scn_double_start(bool reverseOrder) {
    World w(3);
    FormRef npc = F(0x104C4);
    PlayerId a = reverseOrder ? 2 : 1, b = reverseOrder ? 1 : 2;
    w.emit(a, w.cl[a].requestStart(npc));
    w.emit(b, w.cl[b].requestStart(npc));
    w.pump();
    int opened = (w.cl[1].role() == ClientDialogue::Role::Initiator) + (w.cl[2].role() == ClientDialogue::Role::Initiator);
    CHECK(opened == 1);
    CHECK(w.cl[a].role() == ClientDialogue::Role::Initiator);            // first arrival wins
    CHECK(w.count(b, Event::Denied) == 1 && w.count(b, Event::OpenMenu) == 0);
    CHECK(w.cl[b].role() == ClientDialogue::Role::Spectator || w.cl[b].role() == ClientDialogue::Role::Idle);
    CHECK(w.host.open() == 1);
    return true;
}
static bool scn_double_start_a() { return scn_double_start(false); }
static bool scn_double_start_b() { return scn_double_start(true); }

static bool scn_parallel_other_npc() {
    World w(3);
    w.emit(1, w.cl[1].requestStart(F(0x100)));
    w.emit(2, w.cl[2].requestStart(F(0x200)));
    w.pump();
    CHECK(w.host.open() == 2);
    CHECK(w.cl[1].role() == ClientDialogue::Role::Initiator && w.cl[2].role() == ClientDialogue::Role::Initiator);
    return true;
}

static bool scn_spectator_starts_other_npc() {
    World w(3);
    w.emit(1, w.cl[1].requestStart(F(0x100)));
    w.pump();
    CHECK(w.cl[2].role() == ClientDialogue::Role::Spectator);
    w.emit(2, w.cl[2].requestStart(F(0x200)));                          // spectator walks away and talks to another NPC
    w.pump();
    CHECK(w.cl[2].role() == ClientDialogue::Role::Initiator && w.host.open() == 2);
    sayLine(w, 1, F(1), F(2), true);                                     // the first conversation closing must not disturb p2
    CHECK(w.cl[2].role() == ClientDialogue::Role::Initiator && w.host.npcBusy(F(0x200)));
    return true;
}

static bool scn_same_player_two_npcs() {
    World w(2);
    w.emit(1, w.cl[1].requestStart(F(0x100)));
    w.pump();
    // second request from a (buggy/odd) caller bypassing the client guard
    w.fan(w.host.onMessage(1, StartReq{99, F(0x200)}, 0));
    w.pump();
    CHECK(w.host.open() == 1 && !w.host.npcBusy(F(0x200)));
    return true;
}

static bool scn_retransmit_start() {
    World w(2);
    w.emit(1, w.cl[1].requestStart(F(0x100)));
    w.pump();
    w.fan(w.host.onMessage(1, StartReq{1, F(0x100)}, 0));  // lost-ack retransmit
    w.pump();
    CHECK(w.host.open() == 1);
    return true;
}

static bool scn_non_initiator_pick() {
    World w(3);
    w.emit(1, w.cl[1].requestStart(F(0x100)));
    w.pump();
    Pick p; p.conv = 1; p.step = 1; p.topic = F(1); p.info = F(2); p.lines = {L("Hijack")};
    w.fan(w.host.onMessage(2, p, 0));
    w.fan(w.host.onMessage(2, LineDone{1, 1}, 0));
    w.fan(w.host.onMessage(2, EndReq{1, CloseReason::InitiatorLeft}, 0));
    w.pump();
    CHECK(w.runs.empty());
    CHECK(w.host.open() == 1 && w.cl[1].role() == ClientDialogue::Role::Initiator);
    CHECK(w.host.rejected() == 3);
    return true;
}

static bool scn_duplicate_pick() {
    World w(3);
    w.net.dupAll = true;
    w.emit(1, w.cl[1].requestStart(F(0x100)));
    w.pump();
    Pick p; p.conv = 1; p.step = 1; p.topic = F(1); p.info = F(2); p.lines = {L("Once")};
    for (int i = 0; i < 3; ++i) w.fan(w.host.onMessage(1, p, 0));
    for (int i = 0; i < 3; ++i) w.fan(w.host.onMessage(1, LineDone{1, 1}, 0));
    w.pump();
    CHECK(w.runsOf(Phase::Begin) == 1 && w.runsOf(Phase::End) == 1);
    CHECK(w.cl[2].history().size() == 1);                               // spectator shows the line once
    return true;
}

static bool scn_out_of_order_step() {
    World w(2);
    w.emit(1, w.cl[1].requestStart(F(0x100)));
    w.pump();
    Pick p; p.conv = 1; p.step = 5; p.topic = F(1); p.info = F(2);
    w.fan(w.host.onMessage(1, p, 0));
    CHECK(w.runs.empty() && w.host.rejected() == 1);
    return true;
}

static bool scn_disconnect_mid_line() {
    World w(3);
    FormRef npc = F(0x100);
    w.emit(1, w.cl[1].requestStart(npc));
    w.pump();
    w.emit(1, w.cl[1].pick(F(1), F(2), 0, false, {L("Mid")}, someFacts()));
    w.pump();
    CHECK(w.runsOf(Phase::Begin) == 1 && w.runsOf(Phase::End) == 0);
    w.fan(w.host.playerLeft(1, w.net.now));
    w.pump();
    CHECK(w.runsOf(Phase::End) == 1);                                    // begun lines are finished, never half-applied
    CHECK(w.cl[2].role() == ClientDialogue::Role::Idle && w.count(2, Event::CloseMenu) == 1);
    CHECK(!w.host.npcBusy(npc) && w.host.open() == 0);
    // NPC is immediately usable by someone else
    w.emit(2, w.cl[2].requestStart(npc));
    w.pump();
    CHECK(w.cl[2].role() == ClientDialogue::Role::Initiator);
    return true;
}

static bool scn_npc_dies() {
    World w(3);
    FormRef npc = F(0x100);
    w.emit(1, w.cl[1].requestStart(npc));
    w.pump();
    w.fan(w.host.npcDied(npc, 0));
    w.pump();
    CHECK(w.cl[1].role() == ClientDialogue::Role::Idle && w.count(1, Event::CloseMenu) == 1);
    CHECK(w.cl[2].role() == ClientDialogue::Role::Idle);
    w.emit(2, w.cl[2].requestStart(npc));
    w.pump();
    CHECK(w.count(2, Event::Denied) == 1 && w.log[2].back().deny == DenyReason::NpcDead);
    return true;
}

static bool scn_combat() {
    World w(2);
    FormRef npc = F(0x100);
    w.emit(1, w.cl[1].requestStart(npc));
    w.pump();
    w.fan(w.host.npcCombat(npc, true, 0));
    w.pump();
    CHECK(w.cl[1].role() == ClientDialogue::Role::Idle);
    CHECK(w.log[1].back().kind == Event::CloseMenu && w.log[1].back().reason == CloseReason::Combat);
    w.emit(1, w.cl[1].requestStart(npc));
    w.pump();
    CHECK(w.count(1, Event::Denied) == 1);
    w.fan(w.host.npcCombat(npc, false, 0));
    w.emit(1, w.cl[1].requestStart(npc));
    w.pump();
    CHECK(w.cl[1].role() == ClientDialogue::Role::Initiator);
    return true;
}

static bool scn_lease_timeout() {
    HostDialogue::Config c; c.leaseMs = 1000;
    World w(2, c);
    FormRef npc = F(0x100);
    w.emit(1, w.cl[1].requestStart(npc));
    w.pump();
    w.fan(w.host.onMessage(1, Heartbeat{1}, 800));
    w.fan(w.host.tick(1500)); w.pump();
    CHECK(w.host.open() == 1);                                           // heartbeat refreshed the lease
    w.fan(w.host.tick(2000)); w.pump();
    CHECK(w.host.open() == 0 && !w.host.npcBusy(npc));
    CHECK(w.log[1].back().kind == Event::CloseMenu && w.log[1].back().reason == CloseReason::Timeout);
    return true;
}

static bool scn_topic_gate() {
    World w(2);
    w.net.latency = 100; w.uplinkLatency = 100;                           // RTT 200 ms
    w.emit(1, w.cl[1].requestStart(F(0x100)));
    w.pump(300);
    CHECK(w.cl[1].topicsReady());
    w.emit(1, w.cl[1].pick(F(1), F(2), 0, false, {L("A")}, {}));
    w.pump(0);
    CHECK(!w.cl[1].topicsReady());
    w.pump(1000);                                                         // line plays ~1 s; Begin result is back
    w.emit(1, w.cl[1].lineDone(w.net.now));
    w.pump(0);
    CHECK(!w.cl[1].topicsReady());                                       // end script not yet confirmed
    // cannot pick the next topic through the gate
    CHECK(w.cl[1].pick(F(3), F(4), 0, false, {L("B")}, {}).empty());
    uint32_t t0 = w.net.now;
    w.pump(200);                                                          // one RTT later
    CHECK(w.cl[1].topicsReady());
    CHECK(w.net.now - t0 <= 250);                                         // latency budget: <= RTT + slack
    CHECK(w.count(1, Event::TopicsReady) == 1 && !w.log[1].back().timedOut);
    return true;
}

static bool scn_gate_timeout() {
    World w(2);
    w.emit(1, w.cl[1].requestStart(F(0x100)));
    w.pump();
    w.emit(1, w.cl[1].pick(F(1), F(2), 0, false, {L("A")}, {}));
    w.pump();
    w.emit(1, w.cl[1].lineDone(w.net.now));            // sent into the void: drop uplink
    w.up.clear();
    w.net.now += 1600;
    w.emit(1, w.cl[1].tick(w.net.now));
    CHECK(w.cl[1].topicsReady());
    bool sawTimeout = false;
    for (auto& e : w.log[1]) sawTimeout |= (e.kind == Event::TopicsReady && e.timedOut);
    CHECK(sawTimeout);
    return true;
}

static bool scn_spectator_stale_lines() {
    World w(3);
    w.emit(1, w.cl[1].requestStart(F(0x100)));
    w.pump();
    w.net.reverse = true; w.net.dupAll = true;
    sayLine(w, 1, F(1), F(2), false);
    sayLine(w, 1, F(3), F(4), false);
    CHECK(w.cl[2].history().size() == 2);
    CHECK(w.cl[2].history()[0].step == 1 && w.cl[2].history()[1].step == 2);
    CHECK(w.count(2, Event::ShowLines) == 2);
    return true;
}

static bool scn_spectator_leave() {
    World w(3);
    w.emit(1, w.cl[1].requestStart(F(0x100)));
    w.pump();
    w.emit(2, w.cl[2].leave());                                          // spectator hides overlay: no message, no effect
    CHECK(w.log[2].empty() || w.count(2, Event::Send) == 0);
    CHECK(!w.cl[2].spectatorVisible() && w.host.open() == 1);
    sayLine(w, 1, F(1), F(2), false);
    CHECK(w.cl[2].history().size() == 1);                                // still tracks state for the Closed/end cleanup
    CHECK(w.runsOf(Phase::Begin) == 1);
    return true;
}

static bool scn_control_lock() {
    World w(2);
    FormRef npc = F(0x100);
    w.emit(1, w.cl[1].requestStart(npc));
    w.pump();
    w.emit(1, w.cl[1].pick(F(1), F(2), 0, false, {L("A")}, {}));
    w.pump();
    w.fan(w.host.controlLock(1, 0x3F, true));                            // DisablePlayerControls in begin script
    w.pump();
    CHECK(w.count(1, Event::ControlLock) == 1 && w.host.controlsLocked(1));
    w.emit(1, w.cl[1].leave());                                          // initiator walks away before enabling
    w.pump();
    CHECK(w.count(1, Event::ControlLock) == 2);
    bool released = false;
    for (auto& e : w.log[1]) if (e.kind == Event::ControlLock) { auto* c = std::get_if<Control>(&e.msg); if (c && !c->locked) released = true; }
    CHECK(released);                                                     // lease released by Closed
    return true;
}

static bool scn_lock_release() {
    World w(2);
    FormRef npc = F(0x100);
    w.emit(1, w.cl[1].requestStart(npc));
    w.pump();
    w.emit(1, w.cl[1].leave());
    w.pump();
    CHECK(!w.host.npcBusy(npc));
    w.emit(0, w.cl[0].requestStart(npc));
    w.pump();
    CHECK(w.cl[0].role() == ClientDialogue::Role::Initiator);
    return true;
}

static bool scn_codec_roundtrip() {
    std::vector<Msg> ms;
    ms.push_back(StartReq{7, F(5)});
    ms.push_back(Offer{3, 2, {{F(1), 1, 3}, {F(2), 0, 0}}});
    Pick p; p.conv = 3; p.step = 2; p.topic = F(1); p.info = F(9); p.choice = -1; p.goodbye = true;
    p.lines = {L("hi"), L("there")}; p.facts = someFacts();
    ms.push_back(p);
    ms.push_back(LineDone{3, 2}); ms.push_back(EndReq{3, CloseReason::Combat}); ms.push_back(Heartbeat{3});
    ms.push_back(StartDenied{7, DenyReason::Busy, 2}); ms.push_back(Opened{3, 7, F(5), 1});
    Lines l; l.conv = 3; l.step = 2; l.topic = F(1); l.info = F(9); l.lines = {L("hi")}; ms.push_back(l);
    ms.push_back(Result{3, 2, Phase::End, 0xFFFFFFFFFFull}); ms.push_back(Control{3, 5, true}); ms.push_back(Closed{3, CloseReason::Timeout});
    ms.push_back(SayOnceMark{F(0x10A1E6)});
    for (auto& m : ms) {
        auto b = encode(m);
        Msg d;
        CHECK(decode(b, d) == DecodeStatus::Ok);
        CHECK(d.index() == m.index() && encode(d) == b);
        CHECK(decode(b.data(), b.size() - 1, d) != DecodeStatus::Ok);    // truncated
        auto c = b; c[c.size() / 2] ^= 0x40;
        CHECK(decode(c, d) != DecodeStatus::Ok);                          // crc catches flips
    }
    return true;
}

static bool scn_codec_fuzz() {
    uint32_t s = 12345;
    auto rnd = [&]() { s = s * 1664525u + 1013904223u; return s >> 8; };
    Pick p; p.conv = 3; p.step = 2; p.topic = F(1); p.info = F(9); p.lines = {L("hello world")}; p.facts = someFacts();
    auto base = encode(p);
    for (int i = 0; i < 20000; ++i) {
        auto b = base;
        int flips = 1 + rnd() % 4;
        for (int k = 0; k < flips; ++k) b[rnd() % b.size()] = uint8_t(rnd());
        if (rnd() & 1) b.resize(rnd() % (b.size() + 1));
        Msg d; (void)decode(b.data(), b.size(), d, false);                // no crc: parser itself must be safe
    }
    return true;
}

#include "scn_round3.inc"
#include "scn_round4.inc"
#include "scn_round5.inc"
#include "scn_round6.inc"

struct Case { const char* name; bool (*fn)(); const char* twin; Mutations mut; };
static Mutations M(void (*f)(Mutations&)) { Mutations m; f(m); return m; }

int main(int argc, char** argv) {
    std::string group = "all", only;
    for (int i = 1; i < argc; ++i) {
        if (!strcmp(argv[i], "--group") && i + 1 < argc) group = argv[++i];
        else if (!strcmp(argv[i], "--name") && i + 1 < argc) only = argv[++i];
    }
    std::vector<Case> cases = {
        {"happy_client_initiator", scn_happy_client, nullptr, {}},
        {"happy_host_initiator", scn_happy_host, nullptr, {}},
        {"double_start_order_a", scn_double_start_a, "no_npc_lock", M([](Mutations& m) { m.noNpcLock = true; })},
        {"double_start_order_b", scn_double_start_b, nullptr, {}},
        {"parallel_other_npc", scn_parallel_other_npc, nullptr, {}},
        {"same_player_two_npcs", scn_same_player_two_npcs, nullptr, {}},
        {"spectator_starts_other_npc", scn_spectator_starts_other_npc, nullptr, {}},
        {"retransmit_start", scn_retransmit_start, nullptr, {}},
        {"non_initiator_pick", scn_non_initiator_pick, "anyone_picks", M([](Mutations& m) { m.anyonePicks = true; })},
        {"duplicate_pick", scn_duplicate_pick, "replay_duplicate_pick", M([](Mutations& m) { m.replayDuplicatePick = true; })},
        {"out_of_order_step", scn_out_of_order_step, nullptr, {}},
        {"disconnect_mid_line", scn_disconnect_mid_line, "drop_end_on_close", M([](Mutations& m) { m.dropEndOnClose = true; })},
        {"npc_dies", scn_npc_dies, nullptr, {}},
        {"combat_mid_dialogue", scn_combat, nullptr, {}},
        {"lease_timeout", scn_lease_timeout, nullptr, {}},
        {"topic_gate_latency", scn_topic_gate, "no_topic_gate", M([](Mutations& m) { m.noTopicGate = true; })},
        {"gate_timeout", scn_gate_timeout, nullptr, {}},
        {"spectator_stale_lines", scn_spectator_stale_lines, "spectator_stale_lines", M([](Mutations& m) { m.spectatorStaleLines = true; })},
        {"spectator_leave", scn_spectator_leave, nullptr, {}},
        {"control_lock_release", scn_control_lock, "skip_control_release", M([](Mutations& m) { m.skipControlRelease = true; })},
        {"lock_release", scn_lock_release, "leak_lock_on_close", M([](Mutations& m) { m.leakLockOnClose = true; })},
        {"codec_roundtrip", scn_codec_roundtrip, nullptr, {}},
        {"codec_fuzz", scn_codec_fuzz, nullptr, {}},
        {"comp_command_hidden_from_spectators", scn_comp_command_owner_only_hidden_from_spectators, "spectators_see_commands", M([](Mutations& m) { m.spectatorsSeeCommands = true; })},
        {"comp_command_rejected_non_owner", scn_comp_command_rejected_for_non_owner, "non_owner_commands", M([](Mutations& m) { m.nonOwnerCommands = true; })},
        {"comp_owner_preempts", scn_comp_owner_preempts, "no_preempt", M([](Mutations& m) { m.noPreempt = true; })},
        {"comp_per_owner_view", scn_comp_per_owner_view, "no_owner_rewrite", M([](Mutations& m) { m.noOwnerRewrite = true; })},
        {"comp_caps_per_owner", scn_comp_caps_per_owner, "cap_is_party_wide", M([](Mutations& m) { m.capIsPartyWide = true; })},
        {"comp_dismiss_owner_only", scn_comp_dismiss_owner_only, "anyone_dismisses", M([](Mutations& m) { m.anyoneDismisses = true; })},
        {"comp_owner_leaves", scn_comp_owner_leaves, "keep_companions_of_leaver", M([](Mutations& m) { m.keepCompanionsOfLeaver = true; })},
        {"comp_reconcile", scn_comp_reconcile, nullptr, {}},
        {"comp_topic_file", scn_comp_topic_file, nullptr, {}},
        {"menu_opens_on_origin_only", scn_menu_opens_on_origin_only, "menu_on_host", M([](Mutations& m) { m.menuOnHost = true; })},
        {"menu_request_mask_origin_only", scn_menu_request_mask_is_origin_only, "menu_to_spectators", M([](Mutations& m) { m.menuToSpectators = true; })},
        {"menu_only_origin_replies", scn_menu_only_origin_replies, "anyone_replies_menu", M([](Mutations& m) { m.anyoneRepliesMenu = true; })},
        {"menu_reply_resumes_once", scn_menu_reply_resumes_once, "resume_menu_twice", M([](Mutations& m) { m.resumeMenuTwice = true; })},
        {"menu_choice_range", scn_menu_choice_range, "any_menu_choice", M([](Mutations& m) { m.anyMenuChoice = true; })},
        {"menu_timeout_takes_default", scn_menu_timeout_takes_default, "no_menu_default", M([](Mutations& m) { m.noMenuDefault = true; })},
        {"menu_origin_disconnects", scn_menu_origin_disconnects, "no_menu_default_disconnect", M([](Mutations& m) { m.noMenuDefault = true; })},
        {"menu_origin_already_gone", scn_menu_origin_already_gone, nullptr, {}},
        {"menu_host_is_origin", scn_menu_host_is_origin, nullptr, {}},
        {"menu_codec", scn_menu_codec, nullptr, {}},
        {"forced_say_reaches_everyone_once", scn_forced_say_reaches_everyone_once, "forced_no_dedupe", M([](Mutations& m) { m.forcedNoDedupe = true; })},
        {"forced_conversation_target_only", scn_forced_conversation_opens_on_target_only, "forced_start_to_all", M([](Mutations& m) { m.forcedStartToAll = true; })},
        {"forced_conversation_displaces_others", scn_forced_conversation_displaces_others, "forced_no_preempt", M([](Mutations& m) { m.forcedNoPreempt = true; })},
        {"forced_party_scene_actor_choice", scn_forced_party_scene_actor_choice, "forced_always_host", M([](Mutations& m) { m.forcedAlwaysHost = true; })},
        {"forced_target_gone_falls_back", scn_forced_target_gone_falls_back, nullptr, {}},
        {"forced_dead_npc_ignored", scn_forced_dead_npc_ignored, nullptr, {}},
        {"forced_codec", scn_forced_codec, nullptr, {}},
        {"consent_message_table", scn_consent_message_table, nullptr, {}},
        {"consent_all_ready", scn_consent_all_ready, "skip_consent", M([](Mutations& m) { m.skipConsent = true; })},
        {"consent_declined", scn_consent_declined, nullptr, {}},
        {"consent_timeout_declines", scn_consent_timeout_declines, "no_menu_default_consent", M([](Mutations& m) { m.noMenuDefault = true; })},
        {"consent_leaver_does_not_block", scn_consent_leaver_does_not_block, nullptr, {}},
        {"consent_not_needed_for_other_buttons", scn_consent_not_needed_for_other_buttons, nullptr, {}},
        {"consent_only_required_players_answer", scn_consent_only_required_players_answer, "anyone_replies_consent", M([](Mutations& m) { m.anyoneRepliesMenu = true; })},
        // G08 (D3): party vote with the host breaking ties, personal decisions, fork INFOs held before any script
        {"vote_majority_approves", scn_vote_majority_approves, "vote_is_unanimous", M([](Mutations& m) { m.voteIsUnanimous = true; })},
        {"vote_one_dissent_does_not_veto", scn_vote_one_dissent_does_not_veto, "vote_is_unanimous_dissent", M([](Mutations& m) { m.voteIsUnanimous = true; })},
        {"vote_majority_declines", scn_vote_majority_declines, nullptr, {}},
        {"vote_tie_goes_to_the_host", scn_vote_tie_goes_to_the_host, "vote_no_host_tie_break", M([](Mutations& m) { m.voteNoHostTieBreak = true; })},
        {"vote_timeout_counts_the_answers", scn_vote_timeout_counts_the_answers, "vote_timeout_approves", M([](Mutations& m) { m.voteTimeoutApproves = true; })},
        {"vote_any_button_is_irreversible", scn_vote_any_button_is_irreversible, "vote_skip_consent", M([](Mutations& m) { m.skipConsent = true; })},
        {"actor_decides_alone", scn_actor_decides_alone, "actor_asks_party", M([](Mutations& m) { m.actorAsksParty = true; })},
        {"fork_info_waits_for_the_party", scn_fork_info_waits_for_the_party, "no_choice_consent", M([](Mutations& m) { m.noChoiceConsent = true; })},
        {"fork_info_denied_runs_nothing", scn_fork_info_denied_runs_nothing, "no_choice_consent_denied", M([](Mutations& m) { m.noChoiceConsent = true; })},
        {"fork_info_other_infos_are_not_held", scn_fork_info_other_infos_are_not_held, nullptr, {}},
        {"fork_info_leaver_does_not_block", scn_fork_info_leaver_does_not_block, nullptr, {}},
        {"consent_table_modes", scn_consent_table_modes, nullptr, {}},
        {"consent_denied_codec", scn_consent_denied_codec, nullptr, {}},
        // wave 1b (G08 / G27): votes the rules engine owns, carried by the dialogue host
        {"rules_info_end_waits_for_the_verdict", scn_rules_info_end_waits_for_the_verdict, "no_info_gate", M([](Mutations& m) { m.noInfoGate = true; })},
        {"rules_info_end_released_runs_once", scn_rules_info_end_waits_for_the_verdict, "held_end_never_released", M([](Mutations& m) { m.heldEndNeverReleased = true; })},
        {"rules_info_declined_drops_the_end_script", scn_rules_info_declined_drops_the_end_script_and_tells_the_origin, "held_end_runs_on_deny", M([](Mutations& m) { m.heldEndRunsOnDeny = true; })},
        {"rules_info_accepted_release_is_not_lost", scn_rules_info_accepted_release_is_not_lost, "held_end_never_released_goodbye", M([](Mutations& m) { m.heldEndNeverReleased = true; })},
        {"rules_answers_reach_the_vote_sink", scn_rules_answers_reach_the_vote_sink, "vote_answer_dropped", M([](Mutations& m) { m.voteAnswerDropped = true; })},
        {"rules_answers_keep_their_meaning", scn_rules_answers_reach_the_vote_sink, "vote_answer_inverted", M([](Mutations& m) { m.voteAnswerInverted = true; })},
        {"rules_gate_run_does_not_hold", scn_rules_gate_run_does_not_hold, nullptr, {}},
        {"rules_box_waits_for_the_verdict", scn_rules_box_waits_for_the_verdict, "box_gate_ignored", M([](Mutations& m) { m.boxGateIgnored = true; })},
        {"rules_box_gate_run_resumes_at_once", scn_rules_box_gate_run_resumes_at_once, nullptr, {}},
        {"rules_owned_info_is_not_held_twice", scn_rules_owned_info_is_not_held_twice, "rules_owner_keeps_old_hold", M([](Mutations& m) { m.rulesOwnerKeepsOldHold = true; })},
        {"rules_origin_leaves_while_held", scn_rules_origin_leaves_while_held, nullptr, {}},
        {"comp_party_owned_uses_no_slot", scn_comp_party_owned, "party_companion_counts", M([](Mutations& m) { m.partyCompanionCounts = true; })},
        {"comp_party_owned_hired_for_everybody", scn_comp_party_owned, "party_hired_per_slot", M([](Mutations& m) { m.partyHiredPerSlot = true; })},
        {"rules_failsafe_declines_when_the_engine_never_answers", scn_rules_failsafe_declines_when_the_engine_never_answers, "no_menu_default_rules", M([](Mutations& m) { m.noMenuDefault = true; })},
    };
    int fail = 0, ran = 0;
    for (auto& c : cases) {
        if (!only.empty() && only != c.name && (!c.twin || only != c.twin)) continue;
        bool unit = group == "all" || group == "unit";
        bool twin = group == "all" || group == "twin";
        if (unit && (only.empty() || only == c.name)) {
            g_why.clear();
            bool ok = c.fn();
            std::printf("[%s] unit %s%s%s\n", ok ? "PASS" : "FAIL", c.name, ok ? "" : " : ", ok ? "" : g_why.c_str());
            ++ran; fail += !ok;
        }
        if (twin && c.twin && (only.empty() || only == c.twin)) {
            g_why.clear();
            bool ok;
            { MutationScope ms(c.mut); ok = c.fn(); }
            // a twin PASSES when the mutated implementation is caught (scenario fails)
            std::printf("[%s] twin %s -> %s (%s)\n", ok ? "FAIL" : "PASS", c.twin, c.name, ok ? "mutant survived" : g_why.c_str());
            ++ran; fail += ok;
        }
    }
    std::printf("%d run, %d failed\n", ran, fail);
    return fail ? 1 : 0;
}
