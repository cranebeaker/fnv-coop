// Campaign fix wave 3, package W3b (DLC set pieces, docs/CAMPAIGN-GAPS.md G42 to G45): the engine half of the Dead Money / Old World Blues / Lonesome Road / Honest Hearts
// rules and the shipped rule data (docs/quest-policies/rules.json + cause_quests.json). Each scenario is a bool function shared by the real test and its mutation twin
// (group twin, names rules_w3b_*, registered in synccore/CMakeLists.txt).
//   G42  a collar death is downed until a checkpoint (PerPlayerState down=until-checkpoint), per-player scripts / object variables are declared
//   G43  the Teleporter moves the firer and the party members within range (PartyWarp range), organs and implants go to the player on the table
//   G44  the Canyon Wreckage door leaps the clock once per party event (OncePerEvent), ED-E belongs to the party from the first quest, the nuke vote
//   G45  a hand-in of pooled items is charged to the players who hold them (PooledItems), the Jed gate / ambush / fork data
#include <cstdio>
#include <array>
#include <memory>
#include "fnvcoop/refop.h"
#include "fnvcoop/rules.h"
#include "minitest.h"

using namespace fnvcoop;

#ifndef FNVCOOP_RULES_JSON
#define FNVCOOP_RULES_JSON ""
#endif
#ifndef FNVCOOP_CAUSE_JSON
#define FNVCOOP_CAUSE_JSON ""
#endif

namespace {
uint32_t h32(const std::string& s) {
    uint32_t h = 2166136261u;
    for (char c : s) { h ^= uint8_t(c); h *= 16777619u; }
    return h;
}
bool resolveEdid(const std::string& edid, FormRef& out) { out = FormRef("falloutnv.esm", h32(edid)); return true; }
FormRef QF(const char* edid) { FormRef f; resolveEdid(edid, f); return f; }
FormRef PF(uint32_t slot) { return FormRef("falloutnv.esm", 0xF000 + slot); }
FormRef SF(const char* plugin, uint32_t id) { return FormRef::make(plugin, id); }

struct Rec : IEffectSink {
    int equips = 0;   // RefCommand EquipItem this machine was told to apply
    void apply(const Effect& e, const Context&) override { if (auto* rc = std::get_if<RefCommand>(&e)) if (rc->op == kOpEquipItem) ++equips; }
};

struct Rig {
    InMemoryNetwork net{5};
    HostSession host;
    std::vector<std::unique_ptr<Rec>> sinks;
    std::vector<std::unique_ptr<EffectCapture>> caps;
    std::vector<std::unique_ptr<ClientSession>> cs;
    uint64_t now = 0;
    Rig() : host(*net.endpoint(0), SharePolicy{}) {}
    ClientSession& add(uint64_t token) {
        sinks.push_back(std::make_unique<Rec>());
        caps.push_back(std::make_unique<EffectCapture>());
        cs.push_back(std::make_unique<ClientSession>(*net.endpoint(PeerId(cs.size() + 1)), 0, token, *sinks.back(), caps.back().get()));
        return *cs.back();
    }
    void connectAll() { for (auto& c : cs) c->connect(now, c->lastSession() + 1); }
    void run(uint32_t ms) {
        for (uint32_t t = 0; t < ms; t += 5) {
            now += 5; net.advance(now); host.tick(now);
            for (auto& c : cs) c->tick(now);
            net.advance(now);
        }
    }
};

int64_t IT(const std::array<Ledger, kMaxSlots>& L, unsigned slot, const char* item) {
    auto it = L[slot].items.find(QF(item));
    return it == L[slot].items.end() ? 0 : it->second;
}

Cause cause(CauseKind k, const FormRef& src, int32_t aux = 0) { Cause c; c.kind = k; c.source = src; c.aux = aux; return c; }

std::string readAll(const char* path) {
    std::string out;
    if (FILE* f = std::fopen(path, "rb")) { char buf[4096]; size_t n; while ((n = std::fread(buf, 1, sizeof buf, f)) > 0) out.append(buf, n); std::fclose(f); }
    return out;
}

// A world with `guests` guests, the engine built over `json`.
struct World {
    RulesDb db; Rig r; std::unique_ptr<RulesEngine> eng; bool loaded = false; std::string why;
    World(const std::string& json, int guests, RulesEngine::Options o = {}, bool shippedCauses = false) {
        if (!db.load(json, &why)) return;
        for (int i = 0; i < guests; ++i) r.add(uint64_t(i + 1));
        o.playerRef = [](PlayerId s, FormRef& f) { f = PF(s); return true; };
        eng.reset(new RulesEngine(r.host, db, resolveEdid, o));
        if (shippedCauses) {
            std::string e;
            if (std::string(FNVCOOP_CAUSE_JSON).empty() || !eng->loadCauseQuests(readAll(FNVCOOP_CAUSE_JSON), &e)) { why = "cause map: " + e; return; }
        }
        RulesEngine* ep = eng.get();
        r.host.onAction = [ep](PlayerId s, IntentKind k, const FormRef& ref, uint32_t, uint32_t) { ep->handleAction(s, k, ref); };
        r.connectAll(); r.run(400);
        loaded = true;
    }
};

const char* kW3 = R"({"schema":1,
 "policy":{"death":{"mode":"downed","revive_s":60,"all_down":"reload-host-save"}},
 "quests":{
 "Collar":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"PerPlayerState","params":{"scope":"collar","vars":["iNumRadii","fCollarTimer"],"down":"until-checkpoint","checkpoint":["SafeCell"],
      "scripts":["SpeakerScript","VaultBeepScript"],"objvars":["SpeakerScript.bInRadius"]},"origin":"questsweep"}]},
 "Hand":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"PooledItems","params":{"items":["Totem"]},"origin":"questsweep"},
   {"kind":"QuestItemDup","params":{"recipient":"all"},"origin":"derived"}]},
 "Door":{"tier":"T2","questline":"x","curated":false,"rules":[
   {"kind":"OncePerEvent","params":{"globals":["GameDay","GameHour"],"effects":["AdvanceTime"],"window_s":300},"origin":"questsweep"}]},
 "AutoDoc":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"PerPlayerState","params":{"scope":"autodoc","scripts":["AutoDocScript"],"objvars":["AutoDocScript.bMenu"]},"origin":"questsweep"},
   {"kind":"PerPlayerScene","params":{},"origin":"curated"}]},
 "Tele":{"tier":"T2","questline":"x","curated":false,"rules":[
   {"kind":"PartyWarp","params":{"markers":["ArrivalMarker"],"mode":"slot-offset","absent":"hold","range":500},"origin":"questsweep"}]}
}})";

RulesEngine::Options w3Options() {
    RulesEngine::Options o;
    o.itemCount = [](PlayerId s, const FormRef&, int64_t& n) { static const int64_t held[4] = {1, 1, 0, 3}; n = held[s]; return true; };   // slot 3 holds three
    o.playerDistance = [](PlayerId a, PlayerId b, float& d) {
        static const float pos[4] = {0, 200, 900, 0};                  // slot 3 is unknown (no position)
        if (a > 2 || b > 2) return false;
        d = pos[b] > pos[a] ? pos[b] - pos[a] : pos[a] - pos[b]; return true;
    };
    return o;
}

// ---- G42: a collar death is downed until a checkpoint ---------------------------------------------------------------------------------------------------
bool collarDownHolds() {
    World w(kW3, 2, w3Options()); if (!w.loaded) return false;
    RulesEngine& eng = *w.eng;
    std::vector<std::pair<PlayerId, bool>> respawned;
    eng.respawnPlayer = [&](PlayerId s, const FormRef& z) { respawned.push_back({s, !z.plugin.empty()}); };
    w.r.host.commit(0, cause(CauseKind::Script, QF("Collar")), {KillActor{PF(1)}});     // slot 1's collar blows
    bool ok = w.r.host.state().ledger[1].av.count(kAvHealth) == 1 && w.r.host.state().ledger[0].av.count(kAvHealth) == 0;
    const auto v = eng.onPlayerDied(1, 1000);
    ok &= v.kind == RulesEngine::DeathVerdict::Downed && v.untilCheckpoint && eng.downedUntilCheckpoint(1) && eng.isDowned(1);
    eng.tick(1000 + 10 * 60 * 1000);                                                    // ten minutes: a timed revive would be long past
    ok &= respawned.empty() && eng.isDowned(1);
    eng.onCellReported(0, QF("ElsewhereCell"));                                         // a living member in some other cell: not a checkpoint
    ok &= respawned.empty();
    eng.onCellReported(2, QF("SafeCell"));                                              // a living member reaches the checkpoint cell
    ok &= respawned.size() == 1 && respawned[0].first == 1 && respawned[0].second && !eng.isDowned(1) && !eng.downedUntilCheckpoint(1);
    // a death that is not a collar death keeps the timed revive
    const auto v2 = eng.onPlayerDied(2, 5000000);
    ok &= v2.kind == RulesEngine::DeathVerdict::Downed && !v2.untilCheckpoint && v2.respawnAfterMs == 60000;
    // a drain long past does not label an unrelated death as a collar death
    eng.onPlayerRevived(2);
    w.r.host.commit(0, cause(CauseKind::Script, QF("Collar")), {KillActor{PF(2)}});
    const auto v3 = eng.onPlayerDied(2, 9000000);
    ok &= v3.kind == RulesEngine::DeathVerdict::Downed && !v3.untilCheckpoint;
    return ok;
}

bool collarZoneAndAllDownHold() {
    World w(kW3, 2, w3Options()); if (!w.loaded) return false;
    RulesEngine& eng = *w.eng;
    int n = 0; eng.respawnPlayer = [&](PlayerId, const FormRef&) { ++n; };
    int reloads = 0; eng.reloadHostSave = [&] { ++reloads; };
    w.r.host.commit(0, cause(CauseKind::Script, QF("Collar")), {KillActor{PF(2)}});
    eng.onPlayerDied(2, 100);
    bool ok = eng.downedUntilCheckpoint(2);
    eng.setZone(QF("AnyTetherZone"));                                                   // a tether zone becoming satisfied is a checkpoint too
    ok &= n == 1 && !eng.isDowned(2);
    // everybody down (two collar deaths and the host): the host save reloads, nobody waits for a checkpoint nobody can reach
    w.r.host.commit(0, cause(CauseKind::Script, QF("Collar")), {KillActor{PF(0)}, KillActor{PF(1)}});
    eng.onPlayerDied(0, 200); eng.onPlayerDied(1, 201);
    const auto v = eng.onPlayerDied(2, 202);
    ok &= v.kind == RulesEngine::DeathVerdict::ReloadSave && reloads == 1 && !eng.downedUntilCheckpoint(0) && !eng.downedUntilCheckpoint(1);
    return ok;
}

bool perPlayerDeclarationsHold() {
    World w(kW3, 1); if (!w.loaded) return false;
    RulesEngine& eng = *w.eng;
    const auto scripts = eng.perPlayerScripts();
    bool ok = scripts.size() == 3 && eng.runsPerPlayer("SpeakerScript") && eng.runsPerPlayer("vaultbeepscript") && !eng.runsPerPlayer("DoorScript");
    const auto ov = eng.perPlayerObjVarNames(QF("Collar"));
    ok &= ov.size() == 1 && ov[0] == "SpeakerScript.bInRadius" && eng.perPlayerObjVarNames(QF("Door")).empty();
    ok &= eng.isPerPlayerVar(QF("Collar"), "fCollarTimer");
    return ok;
}

// ---- G45: a pooled hand-in is split over the players who hold the items -----------------------------------------------------------------------------------
bool pooledDebitHolds() {
    World w(kW3, 3, w3Options()); if (!w.loaded) return false;
    // slot 1 hands in 2 totems; it holds 1, slot 0 holds 1, slot 2 none, slot 3 three
    w.r.host.commit(1, cause(CauseKind::DialogueResult, QF("Hand")), {RemoveItem{QF("Totem"), 2, Recipient::dflt()}});
    auto& L = w.r.host.state().ledger;
    bool ok = IT(L, 1, "Totem") == -1 && IT(L, 0, "Totem") == -1 && IT(L, 2, "Totem") == 0 && IT(L, 3, "Totem") == 0;
    ok &= w.eng->stats().pooledDebits == 1;
    // an item the rule does not list keeps the old routing (QuestItemDup: every player pays the full count)
    w.r.host.commit(1, cause(CauseKind::DialogueResult, QF("Hand")), {RemoveItem{QF("Other"), 1, Recipient::dflt()}});
    ok &= IT(L, 0, "Other") == -1 && IT(L, 2, "Other") == -1;
    // more than the party holds: the rest stays with the initiator (the apply clamps)
    w.r.host.commit(2, cause(CauseKind::DialogueResult, QF("Hand")), {RemoveItem{QF("Totem"), 9, Recipient::dflt()}});
    ok &= IT(L, 2, "Totem") == -4 && IT(L, 0, "Totem") == -2 && IT(L, 1, "Totem") == -2 && IT(L, 3, "Totem") == -3;
    return ok;
}

// ---- G44: the Canyon Wreckage door leaps the clock once per party event ------------------------------------------------------------------------------------
bool onceEventHolds() {
    World w(kW3, 2, w3Options()); if (!w.loaded) return false;
    RulesEngine& eng = *w.eng;
    const FormRef day = QF("GameDay"), hour = QF("GameHour"), other = QF("SomethingElse");
    eng.tick(1000);
    w.r.host.commit(0, cause(CauseKind::Script, QF("Door")), {SetGlobal{hour, 13.f}, SetGlobal{day, 12.f}});
    bool ok = w.r.host.state().globals.count(day) == 1 && w.r.host.state().globals.at(day) == 12.f;
    eng.tick(21000);                                                                    // another player opens the same door twenty seconds later
    w.r.host.commit(1, cause(CauseKind::Script, QF("Door")), {SetGlobal{hour, 13.f}, SetGlobal{day, 15.f}, SetGlobal{other, 1.f}, AdvanceTime{4320}});
    ok &= w.r.host.state().globals.at(day) == 12.f && w.r.host.state().globals.count(other) == 1;   // the repeat leap is dropped, an unrelated global is not
    ok &= w.eng->stats().onceEventsDropped >= 3;
    eng.tick(1000 + 301000);                                                            // a new party event after the window
    w.r.host.commit(1, cause(CauseKind::Script, QF("Door")), {SetGlobal{day, 20.f}});
    ok &= w.r.host.state().globals.at(day) == 20.f;
    return ok;
}

// ---- G43: the Teleporter moves the firer and the party members within range --------------------------------------------------------------------------------
bool rangeWarpHolds() {
    World w(kW3, 3, w3Options()); if (!w.loaded) return false;
    w.r.host.commit(0, cause(CauseKind::Script, QF("Tele")), {MoveRef{PF(0), QF("ArrivalMarker"), 10.f, 0.f, 0.f, false}});
    auto& placed = w.r.host.state().placed;
    // slot 1 is 200 away (in range), slot 2 is 900 away (out), slot 3 has no known position (stays)
    bool ok = placed.count(PF(0)) == 1 && placed.count(PF(1)) == 1 && placed.count(PF(2)) == 0 && placed.count(PF(3)) == 0;
    ok &= w.eng->stats().rangeWarpsHeld == 2;
    return ok;
}

// ---- rule validation ------------------------------------------------------------------------------------------------------------------------------------
bool w3ValidationHolds() {
    auto bad = [](const char* rule) {
        RulesDb db; std::string why;
        const std::string j = std::string("{\"schema\":1,\"quests\":{\"Q\":{\"tier\":\"T1\",\"questline\":\"x\",\"curated\":false,\"rules\":[") + rule + "]}}}";
        return !db.load(j, &why);
    };
    bool ok = true;
    ok &= bad(R"({"kind":"PerPlayerState","params":{"scope":"c","down":"forever"}})");
    ok &= bad(R"({"kind":"PerPlayerState","params":{"scope":"c","objvars":["NoDot"]}})");
    ok &= bad(R"({"kind":"PerPlayerState","params":{"scope":"c","scripts":["not an id"]}})");
    ok &= bad(R"({"kind":"PooledItems","params":{}})");
    ok &= bad(R"({"kind":"OncePerEvent","params":{"effects":["RemoveItem"]}})");
    ok &= bad(R"({"kind":"OncePerEvent","params":{"globals":["GameDay"],"window_s":0}})");
    ok &= bad(R"({"kind":"PartyWarp","params":{"markers":["M"],"range":-1}})");
    ok &= !bad(R"({"kind":"PerPlayerState","params":{"scope":"c","down":"until-checkpoint","objvars":["S.v"],"scripts":["S"],"checkpoint":["C"]}})");
    ok &= !bad(R"({"kind":"PooledItems","params":{"items":["Totem"]}})");
    return ok;
}

// ---- the shipped data (docs/quest-policies/rules.json + cause_quests.json) ----------------------------------------------------------------------------------
struct Shipped : World {
    explicit Shipped(int guests = 2, RulesEngine::Options o = {}) : World(readAll(FNVCOOP_RULES_JSON), guests, o, true) {
        if (std::string(FNVCOOP_RULES_JSON).empty() || readAll(FNVCOOP_RULES_JSON).empty()) loaded = false;
    }
};
// ---- G42 Dead Money -----------------------------------------------------------------------------------------------------------------------------------
bool shippedCollarDownHolds() {   // the collar, the vault timer and the Suites timer kill their own wearer, and a collar death waits for a checkpoint
    Shipped w(2); if (!w.loaded) return false;
    RulesEngine& eng = *w.eng;
    std::vector<PlayerId> back;
    eng.respawnPlayer = [&](PlayerId s, const FormRef&) { back.push_back(s); };
    bool ok = true;
    // the collar quest script, the vault beep script (MQ03), the Suites setup script (MQ03c), Dog (MQ03) and the kitchen gas trap (MQ03): each kill lands on the slot named,
    // the host and the others are untouched
    struct Src { FormRef src; PlayerId victim; };
    for (const Src& s : {Src{QF("NVDLC01BombCollarQuest"), 1}, Src{SF("DeadMoney.esm", 0x12268), 2}, Src{SF("DeadMoney.esm", 0x6618), 1}, Src{SF("DeadMoney.esm", 0x5D10), 2},
                         Src{SF("DeadMoney.esm", 0x10DAB), 1}}) {
        Shipped x(2); if (!x.loaded) return false;
        x.r.host.commit(0, cause(CauseKind::Script, s.src), {KillActor{PF(s.victim)}});
        const auto& L = x.r.host.state().ledger;
        ok &= L[s.victim].av.count(kAvHealth) == 1 && L[0].av.count(kAvHealth) == 0 && x.r.host.state().kills.count(PF(s.victim)) == 0;
    }
    w.r.host.commit(0, cause(CauseKind::Script, QF("NVDLC01BombCollarQuest")), {KillActor{PF(1)}});
    const auto v = eng.onPlayerDied(1, 1000);
    ok &= v.kind == RulesEngine::DeathVerdict::Downed && v.untilCheckpoint && eng.downedUntilCheckpoint(1);
    eng.tick(1000 + 3600 * 1000);
    ok &= back.empty() && eng.isDowned(1);                     // an hour later: still down (a timed revive would be long over)
    eng.onCellReported(0, QF("NVDLC01Suites"));                // the party reaches the Suites: that is a checkpoint
    ok &= back == std::vector<PlayerId>{1} && !eng.isDowned(1);
    // the declarations the plugin reads
    ok &= eng.runsPerPlayer("NVDLC01RadioSpeakerSCRIPT") && eng.runsPerPlayer("NVDLC01VaultCollarBeepScript") && eng.runsPerPlayer("NVDLC01VillaTravelTriggerScript") &&
          eng.runsPerPlayer("NVDLC01GlobalToxicDamageSCRIPT") && eng.runsPerPlayer("NVDLC01SuitesSetupSCRIPT") && !eng.runsPerPlayer("NVDLC01Dummy");
    const auto ov = eng.perPlayerObjVarNames(QF("NVDLC01BombCollarQuest"));
    ok &= ov.size() == 1 && ov[0] == "NVDLC01RadioSpeakerSCRIPT.bInRadius";
    ok &= eng.isPerPlayerVar(QF("NVDLC01BombCollarQuest"), "fCollarTimer") && eng.isPerPlayerVar(QF("NVDLC01BombCollarQuest"), "bBoom") && eng.isPerPlayerVar(QF("NVDLC01BombCollarQuest"), "iNumRadii");
    return ok;
}

bool shippedCollarCountersHold() {   // each wearer has their own countdown: slot 1 is deep in two radii while slot 2 starts from the clear state
    Shipped w(2); if (!w.loaded) return false;
    RulesEngine& eng = *w.eng;
    const FormRef q = QF("NVDLC01BombCollarQuest");
    auto s1 = eng.beginActing(q, 1, {{"iNumRadii", 0}, {"fCollarTimer", 10}});
    eng.endActing(q, 1, {{"iNumRadii", 2}, {"fCollarTimer", 6.5}});
    auto s2 = eng.beginActing(q, 2, {{"iNumRadii", 2}, {"fCollarTimer", 6.5}});
    bool ok = !s1.empty();
    double n2 = -1, t2 = -1;
    for (auto& v : s2) { if (v.var == "iNumRadii") n2 = v.value; if (v.var == "fCollarTimer") t2 = v.value; }
    ok &= n2 == 0 && t2 == 10;                                   // slot 2 starts from the seed, not from slot 1's countdown
    eng.endActing(q, 2, {{"iNumRadii", 0}, {"fCollarTimer", 10}});
    auto s1b = eng.beginActing(q, 1, {{"iNumRadii", 0}, {"fCollarTimer", 10}});
    double n1 = -1, t1 = -1;
    for (auto& v : s1b) { if (v.var == "iNumRadii") n1 = v.value; if (v.var == "fCollarTimer") t1 = v.value; }
    ok &= n1 == 2 && t1 == 6.5;                                  // and slot 1 gets its own back
    eng.endActing(q, 1, {});
    return ok;
}

bool shippedHoldsAreScopedHold() {   // the tethers hold the set-piece stage, not every stage of the quest (and no Barrier waits for passes nothing sends)
    Shipped w(2); if (!w.loaded) return false;
    RulesEngine& eng = *w.eng;
    struct Hold { const char* quest; uint32_t free; uint32_t held; };
    bool ok = true;
    for (const Hold& h : {Hold{"NVDLC01MQ00", 10, 100}, Hold{"NVDLC01MQ02", 10, 50}, Hold{"NVDLC01MQ03", 100, 30}, Hold{"NVDLC01MQ03a", 5, 20}, Hold{"NVDLC01MQ03c", 10, 20},
                          Hold{"NVDLC03MQ03", 10, 20}, Hold{"NVDLC03MQ05", 40, 60}, Hold{"NVDLC04MQ03b", 10, 255}, Hold{"NVDLC04MQ04", 100, 20}}) {
        ok &= eng.onStageRequested(QF(h.quest), h.free, 0) == StageDecision::Proceed;
        ok &= eng.onStageRequested(QF(h.quest), h.held, 0) == StageDecision::Deferred;
    }
    ok &= eng.onStageRequested(QF("NVDLC03MQ05"), 70, 0) == StageDecision::Deferred && eng.onStageRequested(QF("NVDLC03MQ05"), 100, 0) == StageDecision::Proceed;   // the ending decision waits, the Sink hand-over does not
    return ok;
}

bool shippedCauseMapW3Holds() {
    Shipped w(1); if (!w.loaded) return false;
    struct M { const char* plugin; uint32_t id; const char* quest; };
    bool ok = true;
    for (const M& m : {M{"DeadMoney.esm", 0x6E8E, "NVDLC01BombCollarQuest"}, M{"DeadMoney.esm", 0xE2A9, "NVDLC01MQ03"}, M{"DeadMoney.esm", 0x6618, "NVDLC01MQ03c"},
                       M{"HonestHearts.esm", 0x9F67, "NVDLC02MQ00"}, M{"HonestHearts.esm", 0x101B7, "NVDLC02MQ00"}, M{"LonesomeRoad.esm", 0x9C97, "NVDLC04MQ03b"},
                       M{"LonesomeRoad.esm", 0x937F, "NVDLC04MQ03a"}, M{"LonesomeRoad.esm", 0x39E6, "NVDLC04MQ00"}, M{"OldWorldBlues.esm", 0xF074, "NVDLC03HQBuddy"}}) {
        FormRef q;
        Cause c = cause(CauseKind::Script, SF(m.plugin, m.id));
        const bool one = w.eng->questOfCause(c, q) && q == QF(m.quest);
        if (!one) std::printf("    cause %s:%06X does not map to %s\n", m.plugin, m.id, m.quest);
        ok &= one;
    }
    // the synthetic entries (object scripts that are no quest) stand for themselves: the entry is the form of its first script ("keys" of cause_quests.json)
    for (uint32_t id : {0xF071u, 0xE1AAu}) {
        FormRef q;
        const FormRef sc = SF("OldWorldBlues.esm", id);
        ok &= w.eng->questOfCause(cause(CauseKind::Script, sc), q) && q == sc && w.eng->rulesFor(sc) != nullptr;
    }
    ok &= w.eng->rulesFor(SF("OldWorldBlues.esm", 0xF071))->find("PerkRecipient") != nullptr && w.eng->rulesFor(SF("OldWorldBlues.esm", 0xE1AA))->find("PartyWarp") != nullptr;
    return ok;
}

// ---- G43 Old World Blues ------------------------------------------------------------------------------------------------------------------------------
bool shippedAutoDocHolds() {   // the patient gets their own perk swap, pays their own caps and is the only one locked
    Shipped w(2); if (!w.loaded) return false;
    const FormRef ad = SF("OldWorldBlues.esm", 0xF071);
    w.r.host.commit(2, cause(CauseKind::Script, ad), {SetPerk{SF("OldWorldBlues.esm", 0x98BE), 0, Recipient::dflt()}, SetPerk{SF("OldWorldBlues.esm", 0x15BBA), 1, Recipient::dflt()},
                                                      RemoveItem{QF("Caps001"), 8000, Recipient::dflt()}, SetPlayerControls{0x7F, false, Recipient::dflt()}});
    const auto& L = w.r.host.state().ledger;
    bool ok = L[2].perks.count(SF("OldWorldBlues.esm", 0x15BBA)) == 1 && L[0].perks.empty() && L[1].perks.empty();
    auto it = L[2].items.find(QF("Caps001"));
    ok &= it != L[2].items.end() && it->second == -8000 && L[0].items.empty() && L[1].items.empty();
    ok &= L[2].controlsOff == 0x7F && L[0].controlsOff == 0 && L[1].controlsOff == 0;
    // the jar swap in conversation (DialoguePlayerBrain INFOs) belongs to the player who talks: Big Brained is no longer a perk for everybody
    w.r.host.commit(1, cause(CauseKind::DialogueResult, QF("NVDLC03DialoguePlayerBrain")), {SetPerk{SF("OldWorldBlues.esm", 0x15BBA), 1, Recipient::dflt()}});
    ok &= L[1].perks.count(SF("OldWorldBlues.esm", 0x15BBA)) == 1 && L[0].perks.empty();
    // the declarations: the AutoDoc runs per patient, its menu state has one copy per slot, the organs are per player
    ok &= w.eng->runsPerPlayer("NVDLC03SinkAutoDocSCRIPT") && w.eng->isPerPlayerVar(QF("NVDLC03AutoDoc"), "NVDLC03SinkAutoDocSCRIPT.bRestoreBodyParts");
    ok &= w.eng->isPerPlayerVar(QF("NVDLC03MQ05"), "bRestoredBrain") && w.eng->isPerPlayerVar(QF("NVDLC03MQ05"), "bRestoredHeart") && !w.eng->isPerPlayerVar(QF("NVDLC03MQ05"), "bRestoreBodyParts");
    return ok;
}

bool shippedTeleporterHolds() {   // D10: the Teleporter takes the firer and the party members within range
    RulesEngine::Options o;
    o.playerDistance = [](PlayerId a, PlayerId b, float& d) { d = (a == 0 && b == 1) || (a == 1 && b == 0) ? 300.f : 4000.f; return true; };
    Shipped w(2, o); if (!w.loaded) return false;
    w.r.host.commit(0, cause(CauseKind::Script, SF("OldWorldBlues.esm", 0xE1AA)), {MoveRef{PF(0), QF("NVDLC03SinkArrivalMarker"), 0.f, 0.f, 0.f, false}});
    auto& placed = w.r.host.state().placed;
    return placed.count(PF(0)) == 1 && placed.count(PF(1)) == 1 && placed.count(PF(2)) == 0;
}

// ---- G44 Lonesome Road --------------------------------------------------------------------------------------------------------------------------------
bool shippedLonesomeRoadHolds() {
    Shipped w(2); if (!w.loaded) return false;
    RulesEngine& eng = *w.eng;
    bool ok = true;
    // ED-E is the party's from the first quest he acts in; Rex stays the recruiter's
    w.r.host.commit(1, cause(CauseKind::QuestResult, QF("NVDLC04MQ03a"), 20), {CompanionOwner{SF("LonesomeRoad.esm", 0x4D16), 1, true, 1}, CompanionOwner{QF("Rex"), 1, true, 0}});
    ok &= eng.isPartyCompanion(SF("LonesomeRoad.esm", 0x4D16)) && !eng.isPartyCompanion(QF("Rex"));
    // the nuke boxes are a party vote by form id as well as by EditorID
    FormRef q;
    for (uint32_t id : {0x9C99u, 0x9C98u, 0xBF02u}) ok &= eng.isChoice(SF("LonesomeRoad.esm", id)) && eng.choiceQuest(SF("LonesomeRoad.esm", id), q) && q == QF("NVDLC04MQ03b");
    ok &= eng.isChoice(QF("NVDLC04TempleCaesarConfirmMSG")) && !eng.isChoice(SF("LonesomeRoad.esm", 0x9C95));   // the launch menu itself is the actor's choice, not a vote
    // the strongest player decides the wave scaling
    ok &= eng.reduceCondition(QF("NVDLC04MQ03a"), "GetActorValue", {{0, 40}, {1, 90}, {2, 55}}, 0) == 90.0;
    // the Canyon Wreckage door and the Mojave door leap the clock once per party event
    const FormRef day = SF("FalloutNV.esm", 0x37), hour = SF("FalloutNV.esm", 0x38);
    eng.tick(5000);
    w.r.host.commit(0, cause(CauseKind::Script, SF("LonesomeRoad.esm", 0x39E4)), {SetGlobal{hour, 13.f}, SetGlobal{day, 18.f}});
    eng.tick(15000);
    w.r.host.commit(1, cause(CauseKind::Script, SF("LonesomeRoad.esm", 0x39E6)), {SetGlobal{hour, 13.f}, SetGlobal{day, 21.f}});
    ok &= w.r.host.state().globals.count(day) == 1 && w.r.host.state().globals.at(day) == 18.f;
    return ok;
}

// ---- G45 Honest Hearts --------------------------------------------------------------------------------------------------------------------------------
bool shippedHonestHeartsHolds() {
    RulesEngine::Options o;
    o.itemCount = [](PlayerId s, const FormRef&, int64_t& n) { n = s < 2 ? 1 : 0; return true; };   // slots 0 and 1 hold one totem each
    Shipped w(2, o); if (!w.loaded) return false;
    RulesEngine& eng = *w.eng;
    const FormRef totem = SF("HonestHearts.esm", 0xA84A);
    bool ok = true;
    // the second totem is in somebody else's pack: Daniel's INFO takes both, one from each holder
    w.r.host.commit(1, cause(CauseKind::DialogueResult, QF("NVDLC02DialogueDaniel")), {RemoveItem{totem, 2, Recipient::dflt()}});
    const auto& L = w.r.host.state().ledger;
    auto cnt = [&](unsigned s) { auto it = L[s].items.find(totem); return it == L[s].items.end() ? int64_t(0) : it->second; };
    ok &= cnt(0) == -1 && cnt(1) == -1 && cnt(2) == 0;
    // roots for the Shaman INFO and the quest are counted over the party
    ok &= eng.reduceCondition(QF("NVDLC02DialogueShaman"), "GetItemCount", {{0, 2}, {1, 1}}, 0) == 3.0 && eng.reduceCondition(QF("NVDLC02MS01"), "GetItemCount", {{0, 2}, {1, 1}}, 0) == 3.0;
    // the Jed gate reads the heaviest pack, the ambush any member's kill
    ok &= eng.reduceCondition(QF("NVDLC02DialogueCaravanMaster"), "GetActorValue", {{0, 60}, {1, 101}}, 0) == 101.0;
    ok &= eng.reduceCondition(QF("NVDLC02MQ00"), "IsKiller", {{0, 0}, {1, 1}}, 0) == 1.0;
    // arrival and the fail path are alternatives: one player's stray kill cannot fail an arrived party
    ok &= w.r.host.commit(0, cause(CauseKind::QuestResult, QF("NVDLC02MQ00"), 100), {SetStage{QF("NVDLC02MQ00"), 100}}) != 0;
    ok &= w.r.host.commit(1, cause(CauseKind::QuestResult, QF("NVDLC02MQ00"), 255), {SetStage{QF("NVDLC02MQ00"), 255}}) == 0;
    // the caravan is guarded like Joshua and Daniel, and the MQ06 fail line is a vote
    ok &= eng.isProtectedActor(SF("HonestHearts.esm", 0x9E7C)) && eng.isProtectedActor(SF("HonestHearts.esm", 0xF1A5));
    FormRef q;
    ok &= eng.isChoice(SF("HonestHearts.esm", 0x1049F)) && eng.choiceQuest(SF("HonestHearts.esm", 0x1049F), q) && q == QF("NVDLC02DialogueDaniel");
    // the 14 day leap: the Narrator's, then a Zion door repeat from another player is dropped
    const FormRef day = SF("FalloutNV.esm", 0x37);
    eng.tick(9000);
    w.r.host.commit(0, cause(CauseKind::Script, SF("HonestHearts.esm", 0x10136)), {SetGlobal{day, 20.f}});
    eng.tick(19000);
    w.r.host.commit(1, cause(CauseKind::Script, SF("HonestHearts.esm", 0x101B7)), {SetGlobal{day, 34.f}});
    ok &= w.r.host.state().globals.at(day) == 20.f;
    return ok;
}

// ---- G42: the collar is added AND equipped for every player after MQ01 stage 5; a later EquipItem from a script stays with its own player -----------------------------
bool shippedCollarLoadoutHolds() {
    Shipped w(2); if (!w.loaded) return false;
    const FormRef collar = SF("DeadMoney.esm", 0x6E8F);
    RefCommand eq; eq.op = kOpEquipItem; eq.ref = PF(0); eq.a = collar; eq.i = 1; eq.j = 1;
    w.r.host.commit(0, cause(CauseKind::QuestResult, QF("NVDLC01MQ01"), 5), {AddItem{collar, 1, Recipient::dflt()}, eq});
    w.r.run(300);
    const auto& L = w.r.host.state().ledger;
    auto has = [&](unsigned s) { auto it = L[s].items.find(collar); return it != L[s].items.end() && it->second == 1; };
    bool ok = has(0) && has(1) && has(2) && w.r.sinks[0]->equips == 1 && w.r.sinks[1]->equips == 1;
    // a script that equips something on the player later (the host's own run) is not a reward: the guests are not told
    w.r.host.commit(0, cause(CauseKind::Script, QF("NVDLC01MQ03")), {eq});
    w.r.run(300);
    ok &= w.r.sinks[0]->equips == 1 && w.r.sinks[1]->equips == 1;
    return ok;
}

// ---- the object-script acting context (a PerPlayerState with only scripts / objvars still opens beginActing) --------------------------------------------------
bool scriptOnlyActingHolds() {
    World w(kW3, 2, w3Options()); if (!w.loaded) return false;
    RulesEngine& eng = *w.eng;
    const FormRef q = QF("AutoDoc");
    auto a1 = eng.beginActing(q, 1, {{"AutoDocScript.bMenu", 5}});
    eng.endActing(q, 1, {{"AutoDocScript.bMenu", 7}});
    auto a2 = eng.beginActing(q, 2, {{"AutoDocScript.bMenu", 7}});
    // while slot 2 is the patient, the script's commit belongs to slot 2 and pays slot 2's caps
    w.r.host.commit(0, cause(CauseKind::Script, q), {RemoveItem{QF("Caps"), 100, Recipient::dflt()}});
    eng.endActing(q, 2, {{"AutoDocScript.bMenu", 9}});
    auto a1b = eng.beginActing(q, 1, {{"AutoDocScript.bMenu", 9}});
    eng.endActing(q, 1, {});
    const auto& L = w.r.host.state().ledger;
    return a1.size() == 1 && a1[0].value == 5 && a2.size() == 1 && a2[0].value == 5 && a1b.size() == 1 && a1b[0].value == 7 &&
           IT(L, 2, "Caps") == -100 && IT(L, 0, "Caps") == 0;
}
}  // namespace

TEST(rules_w3b, collar_death_is_downed_until_a_checkpoint) { CHECK(collarDownHolds()); }
TEST(rules_w3b, collar_checkpoint_by_zone_and_all_down_reloads) { CHECK(collarZoneAndAllDownHold()); }
TEST(rules_w3b, per_player_scripts_and_object_variables_are_declared) { CHECK(perPlayerDeclarationsHold()); }
TEST(rules_w3b, pooled_hand_in_is_split_over_the_holders) { CHECK(pooledDebitHolds()); }
TEST(rules_w3b, canyon_wreckage_leap_happens_once_per_party_event) { CHECK(onceEventHolds()); }
TEST(rules_w3b, teleporter_takes_party_members_in_range) { CHECK(rangeWarpHolds()); }
TEST(rules_w3b, new_params_are_validated_strictly) { CHECK(w3ValidationHolds()); }
TEST(rules_w3b, shipped_data_collar_death_waits_for_a_checkpoint) { CHECK(shippedCollarDownHolds()); }
TEST(rules_w3b, shipped_data_each_wearer_has_their_own_countdown) { CHECK(shippedCollarCountersHold()); }
TEST(rules_w3b, shipped_data_dlc_holds_are_scoped_to_their_set_piece_stage) { CHECK(shippedHoldsAreScopedHold()); }
TEST(rules_w3b, shipped_data_set_piece_scripts_find_their_rules) { CHECK(shippedCauseMapW3Holds()); }
TEST(rules_w3b, shipped_data_autodoc_serves_one_patient) { CHECK(shippedAutoDocHolds()); }
TEST(rules_w3b, shipped_data_teleporter_range) { CHECK(shippedTeleporterHolds()); }
TEST(rules_w3b, shipped_data_lonesome_road_party_ede_nuke_vote_and_the_clock) { CHECK(shippedLonesomeRoadHolds()); }
TEST(rules_w3b, shipped_data_honest_hearts_pooled_hand_ins_gate_and_fork) { CHECK(shippedHonestHeartsHolds()); }
TEST(rules_w3b, shipped_data_collar_is_added_and_equipped_for_everyone) { CHECK(shippedCollarLoadoutHolds()); }
TEST(rules_w3b, script_only_per_player_state_opens_the_acting_context) { CHECK(scriptOnlyActingHolds()); }

TEST(twin, rules_w3b_collar_timed) { Mutations m; m.collarDownTimed = true; MutationScope s(m); CHECK(!collarDownHolds()); }
TEST(twin, rules_w3b_collar_zone_timed) { Mutations m; m.collarDownTimed = true; MutationScope s(m); CHECK(!collarZoneAndAllDownHold()); }
TEST(twin, rules_w3b_per_player_scripts_ignored) { Mutations m; m.perPlayerScriptsIgnored = true; MutationScope s(m); CHECK(!perPlayerDeclarationsHold()); }
TEST(twin, rules_w3b_pooled_on_origin) { Mutations m; m.pooledDebitOnOrigin = true; MutationScope s(m); CHECK(!pooledDebitHolds()); }
TEST(twin, rules_w3b_once_event_off) { Mutations m; m.oncePerEventOff = true; MutationScope s(m); CHECK(!onceEventHolds()); }
TEST(twin, rules_w3b_warp_range_ignored) { Mutations m; m.warpRangeIgnored = true; MutationScope s(m); CHECK(!rangeWarpHolds()); }
TEST(twin, rules_w3b_shipped_collar_timed) { Mutations m; m.collarDownTimed = true; MutationScope s(m); CHECK(!shippedCollarDownHolds()); }
TEST(twin, rules_w3b_shipped_collar_kills_all) { Mutations m; m.perPlayerStateKillsAll = true; MutationScope s(m); CHECK(!shippedCollarDownHolds()); }
TEST(twin, rules_w3b_shipped_collar_scripts_ignored) { Mutations m; m.perPlayerScriptsIgnored = true; MutationScope s(m); CHECK(!shippedCollarDownHolds()); }
TEST(twin, rules_w3b_shipped_countdown_shared) { Mutations m; m.actingCopyShared = true; MutationScope s(m); CHECK(!shippedCollarCountersHold()); }
TEST(twin, rules_w3b_shipped_holds_unscoped) { Mutations m; m.stagesParamIgnored = true; MutationScope s(m); CHECK(!shippedHoldsAreScopedHold()); }
TEST(twin, rules_w3b_shipped_cause_unmapped) { Mutations m; m.causeNotResolved = true; MutationScope s(m); CHECK(!shippedCauseMapW3Holds()); }
TEST(twin, rules_w3b_shipped_autodoc_perk_route) { Mutations m; m.perkRouteIgnored = true; MutationScope s(m); CHECK(!shippedAutoDocHolds()); }
TEST(twin, rules_w3b_shipped_autodoc_lock_all) { Mutations m; m.controlsScopeIgnored = true; MutationScope s(m); CHECK(!shippedAutoDocHolds()); }
TEST(twin, rules_w3b_shipped_teleporter_range_ignored) { Mutations m; m.warpRangeIgnored = true; MutationScope s(m); CHECK(!shippedTeleporterHolds()); }
TEST(twin, rules_w3b_shipped_lr_ede_not_party) { Mutations m; m.partyOwnerIgnored = true; MutationScope s(m); CHECK(!shippedLonesomeRoadHolds()); }
TEST(twin, rules_w3b_shipped_lr_clock_twice) { Mutations m; m.oncePerEventOff = true; MutationScope s(m); CHECK(!shippedLonesomeRoadHolds()); }
TEST(twin, rules_w3b_shipped_hh_pooled_on_origin) { Mutations m; m.pooledDebitOnOrigin = true; MutationScope s(m); CHECK(!shippedHonestHeartsHolds()); }
TEST(twin, rules_w3b_shipped_hh_sum_as_any) { Mutations m; m.sumAsAny = true; MutationScope s(m); CHECK(!shippedHonestHeartsHolds()); }
TEST(twin, rules_w3b_shipped_hh_fail_path_wins) { Mutations m; m.exclusiveLastWins = true; MutationScope s(m); CHECK(!shippedHonestHeartsHolds()); }
TEST(twin, rules_w3b_shipped_collar_equip_shared) { Mutations m; m.refCmdPlayerShared = true; MutationScope s(m); CHECK(!shippedCollarLoadoutHolds()); }
TEST(twin, rules_w3b_script_only_copy_shared) { Mutations m; m.actingCopyShared = true; MutationScope s(m); CHECK(!scriptOnlyActingHolds()); }
