// Campaign fix wave 1, rules engine part (docs/CAMPAIGN-GAPS.md section 3, package P1): cause -> quest (G02), ChoiceConsent (G08), perk recipients (G14),
// acting-player copies (G24), anchor / worldspace tethers (G25), guest stage intents as passes and the INFO ready map (G26 / G27), owner=party companions (G34),
// the host reduction (G37), FriendlyFireGuard (G40), death / late-join policy (G41), per-player state (G42), SetPlayerControls scope + own bits (G18), and the
// rule data in docs/quest-policies/rules.json. Each scenario is a bool function shared by the real test and its mutation twin (group twin, registered in CMakeLists.txt).
#include <cstdio>
#include <memory>
#include "fnvcoop/rules.h"
#include "fnvcoop/tether_watch.h"
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
FormRef PF(uint32_t slot) { return FormRef("falloutnv.esm", 0xF000 + slot); }   // player ref of a slot

struct Rec : IEffectSink {
    void apply(const Effect&, const Context&) override {}
};

struct Rig {
    InMemoryNetwork net{5};
    HostSession host;
    std::vector<std::unique_ptr<Rec>> sinks;
    std::vector<std::unique_ptr<EffectCapture>> caps;
    std::vector<std::unique_ptr<ClientSession>> cs;
    uint64_t now = 0;
    explicit Rig(SharePolicy p = {}) : host(*net.endpoint(0), p) {}
    Rig(SharePolicy p, HostSession::Options o) : host(*net.endpoint(0), p, o) {}
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

RulesEngine::Options withPlayers() {
    RulesEngine::Options o;
    o.playerRef = [](PlayerId s, FormRef& f) { f = PF(s); return true; };
    return o;
}
void hook(RulesEngine& eng, Rig& r) {
    r.host.onAction = [&eng](PlayerId s, IntentKind k, const FormRef& ref, uint32_t, uint32_t) { eng.handleAction(s, k, ref); };
}
Cause cause(CauseKind k, const FormRef& src, int32_t aux = 0) { Cause c; c.kind = k; c.source = src; c.aux = aux; return c; }

const char* kW1 = R"({"schema":1,
 "policy":{"death":{"mode":"downed","revive_s":60,"all_down":"reload-host-save"},"late_join":{"during_scene":"wait","after_scene":"warp-to-zone"}},
 "quests":{
 "Ending":{"tier":"T4","questline":"x","curated":true,"rules":[
   {"kind":"SetPlayerControls","params":{"scope":"all","release":"host-effect","timeout_s":600,"masks":["1 1 1 1 1 1 1"]},"origin":"curated"},
   {"kind":"PartyWarp","params":{"markers":["PlayerSlideshowMarker"],"mode":"slot-offset","absent":"hold"},"origin":"curated"},
   {"kind":"SaveSuppress","params":{},"origin":"curated"}]},
 "Perks":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"RewardAll","params":{},"origin":"derived"},
   {"kind":"PerkRecipient","params":{"all":["PowerArmorTraining"],"initiator":["RangerTakedown"],"recruiter":["CompanionSuite"]},"origin":"questsweep"}]},
 "Fork":{"tier":"T4","questline":"x","curated":true,"rules":[
   {"kind":"ChoiceConsent","params":{"topics":["HonestHearts.esm:010718"],"mode":"vote","timeout_s":90},"origin":"questsweep"},
   {"kind":"ChoiceConsent","params":{"boxes":["FalloutNV.esm:0AAA01"],"mode":"unanimous","timeout_s":120},"origin":"questsweep"},
   {"kind":"ChoiceConsent","params":{"topics":["FalloutNV.esm:0AAA02"],"mode":"host"},"origin":"questsweep"},
   {"kind":"ChoiceConsent","params":{"topics":["FalloutNV.esm:0AAA03"],"mode":"initiator"},"origin":"questsweep"}]},
 "Casino":{"tier":"T3","questline":"x","curated":false,"rules":[{"kind":"PerPlayerVars","params":{"vars":["Banned","Frisked"]},"origin":"derived"}]},
 "Collar":{"tier":"T4","questline":"x","curated":false,"rules":[
   {"kind":"PerPlayerState","params":{"scope":"collar","vars":["iNumRadii","fCollarTimer"]},"origin":"questsweep"}]},
 "Zone":{"tier":"T4","questline":"x","curated":false,"rules":[
   {"kind":"PartyTether","params":{"cells":["HooverDam"],"gather":"auto-warp","timeout_s":30,"stages":[10]},"origin":"questsweep"},
   {"kind":"PartyTether","params":{"cells":["HooverDam"],"gather":"wait","stages":[20]},"origin":"questsweep"}]},
 "Chargen":{"tier":"T4","questline":"x","curated":true,"rules":[
   {"kind":"Barrier","params":{"step":"all-players-pass","stages":[200]},"origin":"curated"}]},
 "House":{"tier":"T4","questline":"x","curated":true,"rules":[
   {"kind":"ReadyCheck","params":{"infos":["FalloutNV.esm:15EB69"],"timeout_s":120,"on_timeout":"cancel","confirm":"all-players"},"origin":"questsweep"}]},
 "Plot":{"tier":"T4","questline":"x","curated":false,"rules":[
   {"kind":"CompanionOwner","params":{"owner":"party","npcs":["EDEnpc"]},"origin":"questsweep"}]},
 "Recruit":{"tier":"T3","questline":"x","curated":false,"rules":[{"kind":"CompanionOwner","params":{"owner":"recruiter"},"origin":"derived"}]},
 "Scale":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"PartyAwareCond","params":{"funcs":{"GetLevel":"host","GetActorValue":"max-over-party"}},"origin":"questsweep"}]},
 "Guard":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"FriendlyFireGuard","params":{"actors":["Joshua","FalloutNV.esm:0BEEF1"],"deliberate_hits":3,"window_s":10},"origin":"questsweep"}]},
 "Think":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"SetPlayerControls","params":{"scope":"initiator","release":"host-effect","timeout_s":120,"masks":["0 0 0 0 1 0 0"]},"origin":"questsweep"}]},
 "Plain":{"tier":"T1","questline":"x","curated":false,"rules":[]}
}})";

// ---- G02 ----------------------------------------------------------------------------------------------------------------------------------------------
const char* kCauseMap = R"({"schema":1,"scripts":{"FalloutNV.esm:005C01":"Ending"},"infos":{"FalloutNV.esm:0DD001":"Ending"},"refs":{"FalloutNV.esm:00AA01":"Collar"}})";

bool causeResolverHolds() {
    RulesDb db; if (!db.load(kW1)) return false;
    const FormRef script("falloutnv.esm", 0x5C01), elevator("falloutnv.esm", 0x5C02);
    auto txn = [&](Rig& r, const FormRef& source) {
        r.host.commit(0, cause(CauseKind::Script, source), {MoveRef{PF(0), QF("Cell"), 10.f, 0.f, 0.f, false}, SetPlayerControls{1, false, Recipient::dflt()}});
    };
    bool ok = true;
    {   // the gap: without the map the quest script's own txn matches no rule (source = the script form)
        Rig r; r.add(1); r.add(2);
        RulesEngine eng(r.host, db, resolveEdid, withPlayers());
        r.connectAll(); r.run(400);
        txn(r, script);
        ok &= r.host.state().placed.count(PF(1)) == 0 && r.host.state().ledger[1].controlsOff == 0 && !r.host.state().saveSuppress;
    }
    {   // with the map: PartyWarp moves the party, the control lock reaches every player, saving is blocked
        Rig r; r.add(1); r.add(2);
        RulesEngine eng(r.host, db, resolveEdid, withPlayers());
        std::string err;
        ok &= eng.loadCauseQuests(kCauseMap, &err) && eng.causeQuestCount() == 3;
        FormRef got; ok &= eng.questOfCause(cause(CauseKind::Script, script), got) && got == QF("Ending");
        FormRef seenSource; r.host.onCommit = [&](const EffectTxn& t) { seenSource = t.cause.source; };
        r.connectAll(); r.run(400);
        txn(r, script);
        for (unsigned s = 0; s < 3; ++s) ok &= r.host.state().placed.count(PF(s)) == 1;
        ok &= r.host.state().ledger[1].controlsOff == 1 && r.host.state().ledger[2].controlsOff == 1 && r.host.state().saveSuppress;
        ok &= seenSource == QF("Ending") && eng.stats().causesResolved == 1;
        // an INFO and a ref map the same way
        Cause ic = cause(CauseKind::DialogueResult, FormRef("falloutnv.esm", 0xDD001));
        ok &= eng.questOfCause(ic, got) && got == QF("Ending");
    }
    {   // an object script the map does not name (an elevator) still moves only the activator
        Rig r; r.add(1); r.add(2);
        RulesEngine eng(r.host, db, resolveEdid, withPlayers());
        eng.loadCauseQuests(kCauseMap);
        r.connectAll(); r.run(400);
        txn(r, elevator);
        ok &= r.host.state().placed.count(PF(0)) == 1 && r.host.state().placed.count(PF(1)) == 0 && r.host.state().ledger[1].controlsOff == 0;
    }
    {   // bad maps are refused whole
        Rig r;
        RulesEngine eng(r.host, db, resolveEdid);
        for (const char* bad : {"", "{", "{\"schema\":2}", "{\"schema\":1,\"scripts\":[]}", "{\"schema\":1,\"scripts\":{\"nope\":\"Ending\"}}", "{\"schema\":1,\"scripts\":{\"FalloutNV.esm:01\":5}}"}) {
            std::string err;
            ok &= !eng.loadCauseQuests(bad, &err) && !err.empty();
        }
        ok &= eng.causeQuestCount() == 0;
    }
    return ok;
}

// ---- G08 ----------------------------------------------------------------------------------------------------------------------------------------------
const FormRef kForkInfo = FormRef::make("HonestHearts.esm", 0x010718), kForkBox = FormRef::make("FalloutNV.esm", 0x0AAA01),
              kHostInfo = FormRef::make("FalloutNV.esm", 0x0AAA02), kSoloInfo = FormRef::make("FalloutNV.esm", 0x0AAA03);

bool choiceConsentHolds() {
    RulesDb db; if (!db.load(kW1)) return false;
    bool ok = true;
    struct Res { std::vector<bool> v; };
    {   // vote, three players: the asking guest + one more yes is a majority
        Rig r; auto& a = r.add(1); auto& b = r.add(2); (void)a;
        RulesEngine eng(r.host, db, resolveEdid); Res res; eng.onChoiceResolved = [&](const FormRef& q, const FormRef&, bool go) { ok &= q == QF("Fork"); res.v.push_back(go); };
        hook(eng, r); r.connectAll(); r.run(400);
        ok &= eng.isChoice(kForkInfo) && eng.choiceMode(kForkInfo) == RulesEngine::ChoiceMode::Vote && !eng.isChoice(QF("Other"));
        ok &= eng.onChoiceRequested(kForkInfo, 1) == StageDecision::Deferred;
        ok &= eng.onChoiceRequested(kForkInfo, 1) == StageDecision::Deferred;      // asking twice does not ask twice
        r.run(200); ok &= res.v.empty();
        b.submitAction(IntentKind::ScenePass, kForkInfo); r.run(300);
        ok &= res.v == std::vector<bool>{true} && eng.pendingDeferrals() == 0;
    }
    {   // vote, three players: two declines cancel it
        Rig r; auto& a = r.add(1); auto& b = r.add(2);
        RulesEngine eng(r.host, db, resolveEdid); Res res; eng.onChoiceResolved = [&](const FormRef&, const FormRef&, bool go) { res.v.push_back(go); };
        hook(eng, r); r.connectAll(); r.run(400);
        ok &= eng.onChoiceRequested(kForkInfo, 0) == StageDecision::Deferred;
        a.submitAction(IntentKind::ScenePass, RulesEngine::choiceDeclineId(kForkInfo)); r.run(300);
        ok &= res.v.empty();
        b.submitAction(IntentKind::ScenePass, RulesEngine::choiceDeclineId(kForkInfo)); r.run(300);
        ok &= res.v == std::vector<bool>{false};
    }
    {   // vote, two players, a tie: the host decides (guest asks, host declines -> cancelled; host asks, guest declines -> host's yes carries it)
        Rig r; auto& a = r.add(1);
        RulesEngine eng(r.host, db, resolveEdid); Res res; eng.onChoiceResolved = [&](const FormRef&, const FormRef&, bool go) { res.v.push_back(go); };
        hook(eng, r); r.connectAll(); r.run(400);
        ok &= eng.onChoiceRequested(kForkInfo, 1) == StageDecision::Deferred;
        eng.handleAction(0, IntentKind::ScenePass, RulesEngine::choiceDeclineId(kForkInfo));
        ok &= res.v == std::vector<bool>{false};
        ok &= eng.onChoiceRequested(kForkInfo, 0) == StageDecision::Deferred;
        a.submitAction(IntentKind::ScenePass, RulesEngine::choiceDeclineId(kForkInfo)); r.run(300);
        ok &= res.v == std::vector<bool>{false, true};
    }
    {   // unanimous: everybody or nothing; a timeout cancels
        Rig r; auto& a = r.add(1); r.add(2);
        RulesEngine eng(r.host, db, resolveEdid); Res res; eng.onChoiceResolved = [&](const FormRef&, const FormRef&, bool go) { res.v.push_back(go); };
        hook(eng, r); r.connectAll(); r.run(400); eng.tick(r.now);
        ok &= eng.onChoiceRequested(kForkBox, 0) == StageDecision::Deferred;
        a.submitAction(IntentKind::ScenePass, kForkBox); r.run(300);
        ok &= res.v.empty();                                                      // 2 of 3 is not everybody
        eng.tick(r.now + 119000); ok &= res.v.empty();
        eng.tick(r.now + 121000); ok &= res.v == std::vector<bool>{false};
    }
    {   // host mode: only the host's word counts; initiator mode and a lone player ask nobody
        Rig r; auto& a = r.add(1);
        RulesEngine eng(r.host, db, resolveEdid); Res res; eng.onChoiceResolved = [&](const FormRef&, const FormRef&, bool go) { res.v.push_back(go); };
        hook(eng, r); r.connectAll(); r.run(400);
        ok &= eng.onChoiceRequested(kHostInfo, 1) == StageDecision::Deferred;
        a.submitAction(IntentKind::ScenePass, kHostInfo); r.run(300);
        ok &= res.v.empty();                                                      // the guest's yes alone does not decide
        eng.handleAction(0, IntentKind::ScenePass, kHostInfo);
        ok &= res.v == std::vector<bool>{true};
        ok &= eng.onChoiceRequested(kHostInfo, 0) == StageDecision::Proceed;      // the host asking is the host deciding
        ok &= eng.onChoiceRequested(kSoloInfo, 1) == StageDecision::Proceed;
        Rig alone; RulesEngine e2(alone.host, db, resolveEdid);
        ok &= e2.onChoiceRequested(kForkInfo, 0) == StageDecision::Proceed;
    }
    {   // a yes given before anybody asked does not count for a later vote; an unknown INFO proceeds
        Rig r; auto& a = r.add(1); r.add(2);
        RulesEngine eng(r.host, db, resolveEdid); Res res; eng.onChoiceResolved = [&](const FormRef&, const FormRef&, bool go) { res.v.push_back(go); };
        hook(eng, r); r.connectAll(); r.run(400);
        a.submitAction(IntentKind::ScenePass, kForkInfo); r.run(300);
        ok &= eng.onChoiceRequested(kForkInfo, 0) == StageDecision::Deferred && res.v.empty();
        ok &= eng.onChoiceRequested(QF("NoSuchInfo"), 0) == StageDecision::Proceed;
    }
    return ok;
}

// ---- G14 ----------------------------------------------------------------------------------------------------------------------------------------------
bool perkRoutingHolds() {
    RulesDb db; if (!db.load(kW1)) return false;
    Rig r; r.add(1); r.add(2);
    RulesEngine eng(r.host, db, resolveEdid);
    r.connectAll(); r.run(400);
    bool ok = true;
    const Cause dlg = cause(CauseKind::DialogueResult, QF("Perks"));
    r.host.commit(2, dlg, {SetPerk{QF("RangerTakedown"), 1, Recipient::dflt()}});              // a trainer teaches the one who talks
    ok &= r.host.state().ledger[2].perks.count(QF("RangerTakedown")) == 1 && r.host.state().ledger[0].perks.empty() && r.host.state().ledger[1].perks.empty();
    r.host.commit(1, dlg, {SetPerk{QF("CompanionSuite"), 1, Recipient::dflt()}});              // a recruit perk belongs to the recruiter
    ok &= r.host.state().ledger[1].perks.count(QF("CompanionSuite")) == 1 && r.host.state().ledger[0].perks.count(QF("CompanionSuite")) == 0 &&
          r.host.state().ledger[2].perks.count(QF("CompanionSuite")) == 0;
    r.host.commit(1, dlg, {SetPerk{QF("PowerArmorTraining"), 1, Recipient::dflt()}});          // a quest reward goes to everybody
    for (unsigned s = 0; s < 3; ++s) ok &= r.host.state().ledger[s].perks.count(QF("PowerArmorTraining")) == 1;
    r.host.commit(2, dlg, {SetPerk{QF("CompanionSuite"), 0, Recipient::dflt()}});              // the dismissal removes it from the dismisser's recruit only
    ok &= r.host.state().ledger[1].perks.count(QF("CompanionSuite")) == 1;
    r.host.commit(1, dlg, {SetPerk{QF("SomethingElse"), 1, Recipient::dflt()}});               // a perk the rule does not name keeps the quest default (RewardAll)
    for (unsigned s = 0; s < 3; ++s) ok &= r.host.state().ledger[s].perks.count(QF("SomethingElse")) == 1;
    // a stage result runs as the host: there is no trainer or recruiter to point at, the quest default (everyone) stays
    r.host.commit(0, cause(CauseKind::QuestResult, QF("Perks")), {SetPerk{QF("RangerTakedown"), 1, Recipient::dflt()}});
    for (unsigned s = 0; s < 3; ++s) ok &= r.host.state().ledger[s].perks.count(QF("RangerTakedown")) == 1;
    ok &= eng.stats().perkRoutes >= 4;
    return ok;
}

// ---- G24 ----------------------------------------------------------------------------------------------------------------------------------------------
bool actingCopiesHold() {
    RulesDb db; if (!db.load(kW1)) return false;
    Rig r; r.add(1); r.add(2);
    RulesEngine eng(r.host, db, resolveEdid);
    r.connectAll(); r.run(400);
    const FormRef q = QF("Casino");
    bool ok = true;
    ok &= eng.perPlayerVarNames(q) == std::vector<std::string>{"Banned", "Frisked"} && eng.perPlayerVarNames(QF("Plain")).empty();
    // the engine variable starts at 0 (the seed). Player 1 gets banned in a run for player 1:
    double banned = 0;
    auto live = [&] { return std::vector<RulesEngine::VarValue>{{"Banned", banned}, {"Frisked", 0}}; };
    auto load = [&](const std::vector<RulesEngine::VarValue>& v) { for (auto& x : v) if (x.var == "Banned") banned = x.value; };
    load(eng.beginActing(q, 1, live()));
    ok &= banned == 0 && eng.actingSlot() == 1;
    banned = 1;                                                                        // the script bans whoever is acting
    eng.endActing(q, 1, live());
    ok &= eng.actingSlot() == kNoPlayer;
    // player 2 plays next: the engine variable still holds player 1's ban, player 2 must see their own copy (never banned)
    load(eng.beginActing(q, 2, live()));
    ok &= banned == 0;
    eng.endActing(q, 2, live());
    // and back to player 1: the ban is still theirs
    load(eng.beginActing(q, 1, live()));
    ok &= banned == 1;
    eng.endActing(q, 1, live());
    double v = -1;
    ok &= eng.readPlayerVar(q, "Banned", 1, v) && v == 1.0 && eng.readPlayerVar(q, "Banned", 2, v) && v == 0.0;
    ok &= eng.viewPlayerVar(q, "Banned", 1, 77) == 1.0 && eng.viewPlayerVar(q, "Banned", 0, 77) == 0.0 && eng.viewPlayerVar(q, "Other", 0, 77) == 77.0;
    // while acting, the quest's txns belong to the acting player (a per-player scene reaches only them)
    const FormRef c = QF("Collar");
    eng.beginActing(c, 2, {{"iNumRadii", 0}, {"fCollarTimer", 0}});
    r.host.commit(0, cause(CauseKind::Script, c), {AddItem{QF("Chip"), 1, Recipient::dflt()}});
    eng.endActing(c, 2, {{"iNumRadii", 1}, {"fCollarTimer", 5}});
    ok &= r.host.state().ledger[2].items.count(QF("Chip")) == 1 && r.host.state().ledger[0].items.count(QF("Chip")) == 0 && r.host.state().ledger[1].items.count(QF("Chip")) == 0;
    // the copies survive a host restart
    Bytes saved = eng.saveState();
    Rig r2; RulesEngine e2(r2.host, db, resolveEdid);
    ok &= e2.loadState(saved) && e2.readPlayerVar(c, "fCollarTimer", 2, v) && v == 5.0;
    auto back = e2.beginActing(c, 1, {{"iNumRadii", 9}, {"fCollarTimer", 9}});            // a slot never seen gets the seed (0), not whatever is live
    ok &= back.size() == 2 && back[0].value == 0.0 && back[1].value == 0.0;
    return ok;
}

// ---- G25 ----------------------------------------------------------------------------------------------------------------------------------------------
bool tetherWorldAndTimeoutHold() {
    RulesDb db; if (!db.load(kW1)) return false;
    bool ok = true;
    {   // an exterior tether names the worldspace: any grid cell of it counts
        Rig r; auto& a = r.add(1);
        RulesEngine eng(r.host, db, resolveEdid);
        int n = 0; eng.applyStage = [&](const FormRef&, uint32_t) { ++n; };
        hook(eng, r); r.connectAll(); r.run(400);
        const FormRef q = QF("Zone");
        eng.onCellReported(0, QF("Grid(1,2)"), QF("HooverDam"));
        ok &= eng.onStageRequested(q, 20, 0) == StageDecision::Deferred;            // the guest has not reported yet
        a.submitAction(IntentKind::ReportCell, QF("Grid(3,4)")); r.run(300);
        ok &= n == 0;
        eng.onWorldReported(1, QF("HooverDam")); r.run(100);
        ok &= n == 1;
        FormRef z; ok &= eng.zone(z) && z == QF("HooverDam");                         // the gathered tether is the zone checkpoint
    }
    {   // gather=auto-warp: after timeout_s the absent slots are handed to the plugin to warp, then the stage goes on
        Rig r; r.add(1); r.add(2);
        RulesEngine eng(r.host, db, resolveEdid);
        int n = 0; eng.applyStage = [&](const FormRef&, uint32_t) { ++n; };
        std::vector<PlayerId> warped; FormRef target;
        eng.gatherRest = [&](const FormRef&, uint32_t, const std::vector<PlayerId>& absent, const FormRef& t) { warped = absent; target = t; };
        r.connectAll(); r.run(400); eng.tick(r.now);
        const FormRef q = QF("Zone");
        eng.onCellReported(0, QF("HooverDam")); eng.onCellReported(1, QF("HooverDam")); eng.onCellReported(2, QF("Elsewhere"));
        ok &= eng.onStageRequested(q, 10, 0) == StageDecision::Deferred;
        eng.tick(r.now + 29000); ok &= n == 0 && warped.empty();
        eng.tick(r.now + 31000);
        ok &= n == 1 && warped == std::vector<PlayerId>{2} && target == QF("HooverDam") && eng.stats().tetherAutoWarps == 1;
    }
    {   // 2026-10-30: a hold is never silent. The players it waits for are told at the start (how long, whether they are warped), and an auto-warp is announced 10 s ahead, once
        Rig r; r.add(1); r.add(2);
        RulesEngine eng(r.host, db, resolveEdid);
        int holds = 0, soons = 0; uint32_t waitMs = 0, left = 0; bool autoWarp = false; std::vector<PlayerId> holdOutside, soonOutside;
        eng.tetherHold = [&](const FormRef&, uint32_t, const std::vector<PlayerId>& outside, const FormRef&, uint32_t w, bool aw) { ++holds; holdOutside = outside; waitMs = w; autoWarp = aw; };
        eng.tetherWarpSoon = [&](const FormRef&, uint32_t, const std::vector<PlayerId>& outside, const FormRef&, uint32_t s) { ++soons; soonOutside = outside; left = s; };
        std::vector<PlayerId> warped; eng.gatherRest = [&](const FormRef&, uint32_t, const std::vector<PlayerId>& absent, const FormRef&) { warped = absent; };
        r.connectAll(); r.run(400); eng.tick(r.now);
        eng.onCellReported(0, QF("HooverDam")); eng.onCellReported(1, QF("HooverDam")); eng.onCellReported(2, QF("Elsewhere"));
        ok &= eng.onStageRequested(QF("Zone"), 10, 0) == StageDecision::Deferred;
        ok &= holds == 1 && holdOutside == std::vector<PlayerId>{2} && waitMs == 30000 && autoWarp;
        eng.tick(r.now + 19000); ok &= soons == 0;                                   // 11 s to go: too early
        eng.tick(r.now + 21000); ok &= soons == 1 && soonOutside == std::vector<PlayerId>{2} && left >= 8 && left <= 10 && warped.empty();
        eng.tick(r.now + 25000); ok &= soons == 1;                                   // said once
        eng.tick(r.now + 31000); ok &= warped == std::vector<PlayerId>{2};
    }
    {   // gather=wait: the hold is announced as a hold (no warp is promised) and no warp announcement ever follows
        Rig r; r.add(1);
        RulesEngine eng(r.host, db, resolveEdid);
        int holds = 0, soons = 0; bool autoWarp = true; uint32_t waitMs = 0;
        eng.tetherHold = [&](const FormRef&, uint32_t, const std::vector<PlayerId>&, const FormRef&, uint32_t w, bool aw) { ++holds; waitMs = w; autoWarp = aw; };
        eng.tetherWarpSoon = [&](const FormRef&, uint32_t, const std::vector<PlayerId>&, const FormRef&, uint32_t) { ++soons; };
        r.connectAll(); r.run(400); eng.tick(r.now);
        eng.onCellReported(0, QF("HooverDam")); eng.onCellReported(1, QF("Elsewhere"));
        ok &= eng.onStageRequested(QF("Zone"), 20, 0) == StageDecision::Deferred;
        eng.tick(r.now + 100000);
        ok &= holds == 1 && !autoWarp && soons == 0 && waitMs == 120000;
    }
    {   // gather=wait never warps anybody
        Rig r; r.add(1);
        RulesEngine eng(r.host, db, resolveEdid);
        bool warped = false; eng.gatherRest = [&](const FormRef&, uint32_t, const std::vector<PlayerId>&, const FormRef&) { warped = true; };
        r.connectAll(); r.run(400); eng.tick(r.now);
        eng.onCellReported(0, QF("HooverDam")); eng.onCellReported(1, QF("Elsewhere"));
        ok &= eng.onStageRequested(QF("Zone"), 20, 0) == StageDecision::Deferred;
        eng.tick(r.now + 200000);
        ok &= !warped;
    }
    return ok;
}

// ---- owner request: policy tether.mode on | warn | off -----------------------------------------------------------------------------------------------------
bool tetherModeHolds() {
    RulesDb db; if (!db.load(kW1)) return false;
    struct Out { StageDecision dec = StageDecision::Proceed; int warned = 0; std::vector<PlayerId> outside; size_t pending = 0; uint64_t ignored = 0; };
    auto run = [&](const char* mode) {
        Out o;
        Rig r; r.add(1);
        RulesEngine::Options opt;
        if (mode) opt.policy["tether.mode"] = mode;
        RulesEngine eng(r.host, db, resolveEdid, opt);
        eng.tetherWarn = [&](const FormRef&, uint32_t, const std::vector<PlayerId>& outside, const FormRef&) { ++o.warned; o.outside = outside; };
        r.connectAll(); r.run(400); eng.tick(r.now);
        eng.onCellReported(0, QF("HooverDam")); eng.onCellReported(1, QF("Elsewhere"));
        o.dec = eng.onStageRequested(QF("Zone"), 20, 0);
        eng.tick(r.now + 1000);
        o.pending = eng.pendingDeferrals();
        o.ignored = eng.stats().tethersIgnored;
        return o;
    };
    bool ok = true;
    Out on = run(nullptr), on2 = run("on"), warn = run("warn"), off = run("off");
    ok &= on.dec == StageDecision::Deferred && on.pending == 1 && on.warned == 0;              // default: the tether holds
    ok &= on2.dec == StageDecision::Deferred && on2.pending == 1;
    ok &= warn.dec == StageDecision::Proceed && warn.pending == 0 && warn.warned == 1 && warn.outside == std::vector<PlayerId>{1};   // warn: no hold, the far player is named
    ok &= off.dec == StageDecision::Proceed && off.pending == 0 && off.warned == 0 && off.ignored == 1;                             // off: ignored
    return ok;
}

bool anchorWatchHolds() {
    AnchorWatch w;
    const FormRef sat = QF("SatelliteRef"), far = QF("FarRef");
    w.set({{sat, 0, 0, 0, 1500.f}, {far, 100000, 0, 0, 500.f}});
    bool ok = true;
    auto c = w.update(5000, 0, 0);                                                 // first call reports every anchor once (both far)
    ok &= c.size() == 2 && !c[0].within && !c[1].within;
    ok &= w.update(5100, 0, 0).empty();                                            // no change, nothing sent
    c = w.update(1400, 0, 0);
    ok &= c.size() == 1 && c[0].anchor == sat && c[0].within;
    ok &= w.update(1550, 0, 0).empty();                                            // standing a little outside the radius does not flap
    ok &= w.update(1450, 0, 0).empty();
    c = w.update(1700, 0, 0);
    ok &= c.size() == 1 && !c[0].within;
    w.reset();                                                                     // a rejoin re-reports everything
    ok &= w.update(0, 0, 0).size() == 2 && w.isNear(sat);
    ok &= w.allFar().size() == 1 && !w.isNear(sat);
    return ok;
}

// ---- G26 / G27 ----------------------------------------------------------------------------------------------------------------------------------------
bool passIntentsHold() {
    RulesDb db; if (!db.load(kW1)) return false;
    Rig r; r.add(1);
    RulesEngine eng(r.host, db, resolveEdid);
    int n = 0; eng.applyStage = [&](const FormRef&, uint32_t) { ++n; };
    hook(eng, r); r.connectAll(); r.run(400);
    const FormRef q = QF("Chargen");
    bool ok = true;
    ok &= eng.isPassStage(q, 200) && !eng.isPassStage(q, 100) && !eng.isPassStage(QF("Plain"), 200);
    ok &= eng.onGuestStageIntent(1, q, 100) == StageDecision::Proceed;             // no barrier on this stage
    ok &= eng.onGuestStageIntent(1, q, 200) == StageDecision::Deferred;            // the guest finished chargen: its pass is recorded, the stage waits for the host
    ok &= n == 0 && eng.pendingDeferrals() == 1;
    ok &= eng.onGuestStageIntent(0, q, 200) == StageDecision::Deferred || n == 1;   // the host finishes too: everybody passed
    r.run(300);
    ok &= n == 1 && eng.pendingDeferrals() == 0;
    return ok;
}

bool readyInfoMapHolds() {
    RulesDb db; if (!db.load(kW1)) return false;
    Rig r; auto& a = r.add(1);
    RulesEngine eng(r.host, db, resolveEdid);
    std::vector<FormRef> quests; std::vector<bool> res;
    eng.onReadyResolved = [&](const FormRef& q, const FormRef&, bool go) { quests.push_back(q); res.push_back(go); };
    hook(eng, r); r.connectAll(); r.run(400);
    const FormRef info = FormRef::make("FalloutNV.esm", 0x15EB69);
    bool ok = true;
    FormRef q; ok &= eng.readyQuest(info, q) && q == QF("House");
    ok &= eng.onInfoResult(info, 0) == StageDecision::Deferred;                    // only the INFO is known to the caller
    a.submitAction(IntentKind::ScenePass, info); r.run(300);
    ok &= res == std::vector<bool>{true} && quests.size() == 1 && quests[0] == QF("House");
    ok &= eng.onInfoResult(QF("PlainInfo"), 0) == StageDecision::Proceed;
    // a ChoiceConsent INFO is asked through the same entry point
    ok &= eng.onInfoResult(kForkInfo, 0) == StageDecision::Deferred;
    return ok;
}

// ---- G34 ----------------------------------------------------------------------------------------------------------------------------------------------
bool partyOwnerHolds() {
    RulesDb db; if (!db.load(kW1)) return false;
    HostSession::Options ho; ho.followers.humanoidPerOwner = 1;
    Rig r(SharePolicy(), ho); r.add(1);
    RulesEngine eng(r.host, db, resolveEdid);
    r.connectAll(); r.run(400);
    bool ok = true;
    auto recruit = [&](const char* quest, const char* npc, PlayerId who, bool active = true) {
        return r.host.commit(who, cause(CauseKind::DialogueResult, QF(quest)), {CompanionOwner{QF(npc), 0, active, 0}}) != 0;
    };
    ok &= recruit("Plot", "EDEnpc", 1);                                             // ED-E joins the party's plot
    ok &= eng.isPartyCompanion(QF("EDEnpc")) && eng.partyCompanions().size() == 1;
    ok &= recruit("Recruit", "Veronica", 1);                                         // player 1 still has their own humanoid slot
    ok &= !eng.isPartyCompanion(QF("Veronica"));
    ok &= !recruit("Recruit", "Boone", 1);                                           // ...and only one: the cap holds
    ok &= recruit("Plot", "EDEnpc", 1, false);                                       // dismissing ED-E ends it
    ok &= !eng.isPartyCompanion(QF("EDEnpc"));
    // the flag survives a host restart
    recruit("Plot", "EDEnpc", 1);
    Bytes saved = eng.saveState();
    Rig r2; RulesEngine e2(r2.host, db, resolveEdid);
    ok &= e2.loadState(saved) && e2.isPartyCompanion(QF("EDEnpc"));
    return ok;
}

// ---- G37 ----------------------------------------------------------------------------------------------------------------------------------------------
bool hostReductionHolds() {
    RulesDb db; if (!db.load(kW1)) return false;
    Rig r;
    RulesEngine eng(r.host, db, resolveEdid);
    const FormRef q = QF("Scale");
    bool ok = true;
    ok &= eng.reduceCondition(q, "GetLevel", {{0, 12}, {1, 30}, {2, 5}}, 2) == 12.0;       // XP tiers follow the host level (D8)
    ok &= eng.reduceCondition(q, "GetActorValue", {{0, 12}, {1, 30}, {2, 5}}, 2) == 30.0;  // scaling reads the strongest member
    ok &= eng.reduceCondition(q, "GetLevel", {{1, 30}, {2, 5}}, 2) == 5.0;                 // no host value: the initiator
    return ok;
}

// ---- G40 ----------------------------------------------------------------------------------------------------------------------------------------------
bool friendlyFireHolds() {
    RulesDb db; if (!db.load(kW1)) return false;
    Rig r;
    RulesEngine eng(r.host, db, resolveEdid);
    const FormRef joshua = QF("Joshua"), stranger = QF("Stranger"), other = FormRef::make("FalloutNV.esm", 0xBEEF1);
    using V = RulesEngine::FfVerdict;
    bool ok = true;
    ok &= eng.isProtectedActor(joshua) && eng.isProtectedActor(other) && !eng.isProtectedActor(stranger);
    ok &= eng.onProtectedHit(stranger, 1, false, 0) == V::NotProtected;
    ok &= eng.onProtectedHit(joshua, 1, true, 1000) == V::BlockWarn;                // a grenade: blocked, warned once
    ok &= eng.onProtectedHit(joshua, 1, true, 1100) == V::Block;
    ok &= eng.onProtectedHit(joshua, 1, false, 2000) == V::Block;                   // a first direct hit is still an accident
    ok &= eng.onProtectedHit(joshua, 1, false, 3000) == V::Block;
    ok &= eng.onProtectedHit(joshua, 1, false, 4000) == V::Allow;                   // three in a row is deliberate: the line fails for everyone
    ok &= eng.onProtectedHit(joshua, 1, true, 5000) == V::Allow;                    // and stays so
    ok &= eng.onProtectedHit(joshua, 2, false, 1000) == V::BlockWarn;               // another player starts over
    ok &= eng.onProtectedHit(joshua, 2, false, 12000) == V::Block;                  // hits outside the window do not add up
    ok &= eng.onProtectedHit(joshua, 2, false, 24000) == V::Block;
    eng.confirmDeliberate(other, 3);                                                // the host confirms a deliberate attack
    ok &= eng.onProtectedHit(other, 3, false, 25000) == V::Allow;
    ok &= eng.stats().friendlyFireBlocked >= 6 && eng.stats().friendlyFireAllowed >= 3;
    return ok;
}

// ---- G41 ----------------------------------------------------------------------------------------------------------------------------------------------
bool deathAndLateJoinHold() {
    RulesDb db; if (!db.load(kW1)) return false;
    bool ok = true;
    {   // downed, then respawned at the zone checkpoint; a revive in time cancels the respawn
        Rig r; r.add(1); r.add(2);
        RulesEngine eng(r.host, db, resolveEdid);
        std::vector<std::pair<PlayerId, FormRef>> respawns; eng.respawnPlayer = [&](PlayerId s, const FormRef& z) { respawns.push_back({s, z}); };
        r.connectAll(); r.run(400);
        eng.setZone(QF("HooverDam"));
        eng.tick(1000);
        auto v = eng.onPlayerDied(1, 1000);
        ok &= v.kind == RulesEngine::DeathVerdict::Downed && v.hasCheckpoint && v.checkpoint == QF("HooverDam") && v.respawnAfterMs == 60000 && eng.isDowned(1);
        eng.tick(60000); ok &= respawns.empty();
        eng.tick(62000);
        ok &= respawns.size() == 1 && respawns[0].first == 1 && respawns[0].second == QF("HooverDam") && !eng.isDowned(1);
        eng.onPlayerDied(2, 70000); eng.onPlayerRevived(2); eng.tick(200000);
        ok &= respawns.size() == 1;
        // everybody down: the host save is reloaded
        int reloads = 0; eng.reloadHostSave = [&] { ++reloads; };
        eng.onPlayerDied(0, 300000); eng.onPlayerDied(1, 300001);
        auto last = eng.onPlayerDied(2, 300002);
        ok &= last.kind == RulesEngine::DeathVerdict::ReloadSave && reloads == 1 && !eng.isDowned(0);
    }
    {   // the policy can be overridden per setting (nvcoop.ini): a normal death
        Rig r; r.add(1);
        RulesEngine::Options o; o.policy["death.mode"] = "normal";
        RulesEngine eng(r.host, db, resolveEdid, o);
        r.connectAll(); r.run(400);
        ok &= eng.policy("death.mode", "x") == "normal" && eng.policy("death.revive_s", "x") == "60";
        ok &= eng.onPlayerDied(1, 100).kind == RulesEngine::DeathVerdict::Normal && !eng.isDowned(1);
    }
    {   // death.mode=respawn: no downed wait, the respawn happens at the next tick
        Rig r; r.add(1);
        RulesEngine::Options o; o.policy["death.mode"] = "respawn";
        RulesEngine eng(r.host, db, resolveEdid, o);
        std::vector<PlayerId> respawned; eng.respawnPlayer = [&](PlayerId s, const FormRef&) { respawned.push_back(s); };
        r.connectAll(); r.run(400); eng.tick(1000);
        auto v = eng.onPlayerDied(1, 1000);
        ok &= v.kind == RulesEngine::DeathVerdict::Downed && v.respawnAfterMs == 0;
        eng.tick(1001);
        ok &= respawned == std::vector<PlayerId>{1};
    }
    {   // late joiners wait for a running scene, then are warped to the zone; with nothing to catch up on they just join
        Rig r; r.add(1);
        RulesEngine eng(r.host, db, resolveEdid);
        r.connectAll(); r.run(400);
        ok &= eng.onPlayerJoining(1).kind == RulesEngine::JoinVerdict::JoinNow;     // no zone yet
        const FormRef q = QF("Chargen");
        eng.onStageRequested(q, 200, 0);                                           // a barrier scene is running
        ok &= eng.sceneActive() && eng.onPlayerJoining(2).kind == RulesEngine::JoinVerdict::WaitForScene;
        eng.onPlayerPassed(0, q); eng.onPlayerPassed(1, q); r.run(100);
        ok &= !eng.sceneActive();
        eng.setZone(QF("HooverDam"));
        auto jv = eng.onPlayerJoining(2);
        ok &= jv.kind == RulesEngine::JoinVerdict::WarpToZone && jv.zone == QF("HooverDam");
        // 2026-10-30 (a guest "pulled back while running"): a player who already reports being in the zone is not warped again; one reported elsewhere still is
        eng.onCellReported(2, QF("Goodsprings"));
        ok &= eng.onPlayerJoining(2).kind == RulesEngine::JoinVerdict::WarpToZone;
        eng.onCellReported(2, QF("HooverDam"));
        ok &= eng.onPlayerJoining(2).kind == RulesEngine::JoinVerdict::JoinNow;
        eng.onCellReported(2, QF("Goodsprings"), QF("HooverDam"));                  // an interior of a worldspace-zone: the worldspace is in the zone
        ok &= eng.onPlayerJoining(2).kind == RulesEngine::JoinVerdict::JoinNow;
    }
    return ok;
}

// ---- G42 ----------------------------------------------------------------------------------------------------------------------------------------------
bool perPlayerKillHolds() {
    RulesDb db; if (!db.load(kW1)) return false;
    Rig r; r.add(1); r.add(2);
    RulesEngine eng(r.host, db, resolveEdid, withPlayers());
    r.connectAll(); r.run(400);
    bool ok = true;
    // player 2's collar explodes: the script runs player.kill on player 2's ref. Only that player dies.
    r.host.commit(0, cause(CauseKind::Script, QF("Collar")), {KillActor{PF(2)}});
    ok &= r.host.state().ledger[2].av.count(kAvHealth) == 1 && r.host.state().ledger[2].av.at(kAvHealth) == -kKillDrain;
    ok &= r.host.state().ledger[0].av.count(kAvHealth) == 0 && r.host.state().ledger[1].av.count(kAvHealth) == 0;
    ok &= r.host.state().kills.count(PF(2)) == 0 && eng.stats().killsRerouted == 1;
    // a kill of something that is not a player is an ordinary shared kill
    r.host.commit(0, cause(CauseKind::Script, QF("Collar")), {KillActor{QF("Guard")}});
    ok &= r.host.state().kills.count(QF("Guard")) == 1;
    // another quest without the rule is not touched
    r.host.commit(0, cause(CauseKind::Script, QF("Plain")), {KillActor{PF(1)}});
    ok &= r.host.state().kills.count(PF(1)) == 1;
    return ok;
}

// ---- G18 ----------------------------------------------------------------------------------------------------------------------------------------------
bool controlScopeAndBitsHold() {
    RulesDb db; if (!db.load(kW1)) return false;
    Rig r; r.add(1); r.add(2);
    RulesEngine eng(r.host, db, resolveEdid);
    r.connectAll(); r.run(400); eng.tick(1000);
    bool ok = true;
    // scope=initiator: a forced greeting / weapon lock holds only the player it is about
    r.host.commit(2, cause(CauseKind::Script, QF("Think")), {SetPlayerControls{0x4, false, Recipient::dflt()}});
    ok &= r.host.state().ledger[2].controlsOff == 0x4 && r.host.state().ledger[0].controlsOff == 0 && r.host.state().ledger[1].controlsOff == 0;
    // overlapping locks: a cinematic holds 0x1 | 0x4 for everybody; the weapon lock releasing its own bit 0x4 must not free the cinematic's hold
    r.host.commit(0, cause(CauseKind::Script, QF("Ending")), {SetPlayerControls{0x5, false, Recipient::dflt()}});
    ok &= r.host.state().ledger[2].controlsOff == 0x5 && r.host.state().ledger[1].controlsOff == 0x5;
    r.host.commit(2, cause(CauseKind::Script, QF("Think")), {SetPlayerControls{0x4, true, Recipient::dflt()}});
    ok &= (r.host.state().ledger[2].controlsOff & 0x4) == 0x4;                    // still held by the cinematic
    r.host.commit(0, cause(CauseKind::Script, QF("Ending")), {SetPlayerControls{0x5, true, Recipient::dflt()}});
    ok &= r.host.state().ledger[0].controlsOff == 0 && r.host.state().ledger[1].controlsOff == 0 && r.host.state().ledger[2].controlsOff == 0;
    // a missed release times out for the player it was about
    r.host.commit(1, cause(CauseKind::Script, QF("Think")), {SetPlayerControls{0x4, false, Recipient::dflt()}});
    ok &= r.host.state().ledger[1].controlsOff == 0x4;
    eng.tick(1000 + 121000);
    ok &= r.host.state().ledger[1].controlsOff == 0;
    return ok;
}

// ---- the shipped data (docs/quest-policies/rules.json + cause_quests.json): one scenario per gap, each broken by its mutation twin ------------------------------
struct Shipped {
    RulesDb db; Rig r; std::unique_ptr<RulesEngine> eng; bool loaded = false; std::string why;
    explicit Shipped(int guests = 2) {
        if (std::string(FNVCOOP_RULES_JSON).empty() || !db.loadFile(FNVCOOP_RULES_JSON, &why)) return;
        for (int i = 0; i < guests; ++i) r.add(uint64_t(i + 1));
        eng.reset(new RulesEngine(r.host, db, resolveEdid, withPlayers()));
        std::string e;
        if (std::string(FNVCOOP_CAUSE_JSON).empty() || !eng->loadCauseQuests(readAll(FNVCOOP_CAUSE_JSON), &e)) { why = "cause map: " + e; return; }
        hook(*eng, r);
        r.connectAll(); r.run(400);
        loaded = true;
    }
    static std::string readAll(const char* path) {
        std::string out; if (FILE* f = std::fopen(path, "rb")) { char buf[4096]; size_t n; while ((n = std::fread(buf, 1, sizeof buf, f)) > 0) out.append(buf, n); std::fclose(f); }
        return out;
    }
};
FormRef SF(const char* plugin, uint32_t id) { return FormRef::make(plugin, id); }

bool shippedCauseHolds() {   // G02: the House ending start moves, locks and save-blocks every player (MrHouseSCRIPT -> VHDHouseBattle), through the real rules
    Shipped w; if (!w.loaded) return false;
    w.r.host.commit(0, cause(CauseKind::Script, SF("FalloutNV.esm", 0x144484)), {MoveRef{PF(0), QF("PlayerSlideshowMarker"), 10.f, 0.f, 0.f, false}, SetPlayerControls{0x7F, false, Recipient::dflt()}});
    bool ok = true;
    for (unsigned s = 0; s < 3; ++s) ok &= w.r.host.state().placed.count(PF(s)) == 1 && w.r.host.state().ledger[s].controlsOff == 0x7F;
    ok &= w.r.host.state().saveSuppress;
    // an unrelated door script still moves only the activator
    w.r.host.commit(0, cause(CauseKind::Script, SF("FalloutNV.esm", 0x5C02)), {MoveRef{PF(1), QF("Cell"), 0.f, 0.f, 0.f, false}});
    ok &= w.eng->stats().warpsExpanded == 1;
    return ok;
}

bool shippedPerksHold() {   // G14: Ranger Takedown reaches only the trainee, a recruit perk only the recruiter, a quest-reward perk every player (3 players)
    Shipped w; if (!w.loaded) return false;
    bool ok = true;
    w.r.host.commit(2, cause(CauseKind::DialogueResult, QF("VDialogueNovac")), {SetPerk{QF("RangerTakedown"), 1, Recipient::dflt()}});
    ok &= w.r.host.state().ledger[2].perks.count(QF("RangerTakedown")) == 1 && w.r.host.state().ledger[0].perks.empty() && w.r.host.state().ledger[1].perks.empty();
    w.r.host.commit(1, cause(CauseKind::DialogueResult, QF("VDialogueRaul")), {SetPerk{QF("OldVaquero"), 1, Recipient::dflt()}});
    ok &= w.r.host.state().ledger[1].perks.count(QF("OldVaquero")) == 1 && w.r.host.state().ledger[0].perks.count(QF("OldVaquero")) == 0 &&
          w.r.host.state().ledger[2].perks.count(QF("OldVaquero")) == 0;
    w.r.host.commit(1, cause(CauseKind::DialogueResult, QF("VDialogueHiddenValley")), {SetPerk{QF("PowerArmorTraining"), 1, Recipient::dflt()}});
    for (unsigned s = 0; s < 3; ++s) ok &= w.r.host.state().ledger[s].perks.count(QF("PowerArmorTraining")) == 1;
    return ok;
}

bool shippedExclusiveHolds() {   // G28: the named forks end in the first committed outcome
    Shipped w; if (!w.loaded) return false;
    bool ok = true;
    struct Fork { const char* q; uint32_t a, b; };
    for (const Fork& f : {Fork{"VMS35", 100, 105}, Fork{"VMS20", 100, 110}, Fork{"NVDLC02MQ04", 100, 255}, Fork{"NVDLC01MQ03", 255, 100}}) {
        const FormRef quest = QF(f.q);
        ok &= w.r.host.commit(0, cause(CauseKind::QuestResult, quest, int32_t(f.a)), {SetStage{quest, f.a}}) != 0;
        ok &= w.r.host.commit(1, cause(CauseKind::QuestResult, quest, int32_t(f.b)), {SetStage{quest, f.b}}) == 0;
        ok &= w.r.host.state().stages.at(quest) == f.a;
    }
    return ok;
}

bool shippedPooledHolds() {   // G45: the Jed gate reads the heaviest pack, the totems of MQ03a are counted over the party
    Shipped w(2); if (!w.loaded) return false;
    bool ok = true;
    ok &= w.eng->reduceCondition(QF("NVDLC02DialogueCaravanMaster"), "GetActorValue", {{0, 60}, {1, 101}, {2, 80}}, 0) == 101.0;
    ok &= w.eng->reduceCondition(QF("NVDLC02MQ03a"), "GetItemCount", {{0, 1}, {1, 1}, {2, 0}}, 0) == 2.0;
    ok &= w.eng->reduceCondition(QF("NVDLC02MS01"), "GetItemCount", {{0, 2}, {1, 1}}, 0) == 3.0;
    ok &= w.eng->reduceCondition(QF("NVDLC02MQ00"), "IsKiller", {{0, 0}, {1, 1}, {2, 0}}, 0) == 1.0;   // the caravan ambush: any party member's kill counts
    return ok;
}

bool shippedPassHolds() {   // G26: the chargen quests hold only the hand-over stage, and a guest finishing chargen is a pass
    Shipped w(1); if (!w.loaded) return false;
    bool ok = w.eng->isPassStage(QF("VCG01"), 200) && !w.eng->isPassStage(QF("VCG01"), 100) && w.eng->isPassStage(QF("VCG00"), 100);
    int n = 0; w.eng->applyStage = [&](const FormRef&, uint32_t) { ++n; };
    ok &= w.eng->onGuestStageIntent(1, QF("VCG01"), 200) == StageDecision::Deferred && n == 0;
    w.eng->onPlayerPassed(0, QF("VCG01")); w.r.run(200);
    ok &= n == 1;
    return ok;
}

bool shippedFriendlyHolds() {   // G40: Joshua and Daniel are guarded; a grenade does not end the DLC, a deliberate attack still does
    Shipped w(1); if (!w.loaded) return false;
    const FormRef joshua = SF("HonestHearts.esm", 0x93BE);
    bool ok = w.eng->isProtectedActor(joshua) && w.eng->isProtectedActor(SF("HonestHearts.esm", 0x93C2)) && w.eng->isProtectedActor(QF("VHDColonelMooreREF"));
    ok &= w.eng->onProtectedHit(joshua, 1, true, 100) != RulesEngine::FfVerdict::Allow;
    for (uint64_t t : {1000, 2000, 3000}) w.eng->onProtectedHit(joshua, 2, false, t);
    ok &= w.eng->onProtectedHit(joshua, 2, false, 4000) == RulesEngine::FfVerdict::Allow;
    return ok;
}

bool shippedPartyOwnerHolds() {   // G34: ED-E belongs to the whole party
    Shipped w(1); if (!w.loaded) return false;
    w.r.host.commit(1, cause(CauseKind::DialogueResult, QF("NVDLC04DialogueEDE")), {CompanionOwner{QF("EDE"), 0, true, 1}});
    return w.eng->isPartyCompanion(QF("EDE")) && !w.eng->isPartyCompanion(QF("Boone"));
}

bool shippedHostLevelHolds() {   // G37: XP tiers follow the host level, the Lonesome Road scaling the strongest member
    Shipped w(2); if (!w.loaded) return false;
    return w.eng->reduceCondition(QF("NVDLC01MQ01"), "GetLevel", {{0, 12}, {1, 30}, {2, 5}}, 2) == 12.0 &&
           w.eng->reduceCondition(QF("NVDLC04MQ03a"), "GetLevel", {{0, 12}, {1, 30}, {2, 5}}, 2) == 30.0;
}

bool shippedScopeHolds() {   // G18: the Think Tank pacification lock holds only the player who stands in the zone
    Shipped w(2); if (!w.loaded) return false;
    w.r.host.commit(2, cause(CauseKind::Script, SF("OldWorldBlues.esm", 0xD6D0)), {SetPlayerControls{0x4, false, Recipient::dflt()}});
    return w.r.host.state().ledger[2].controlsOff == 0x4 && w.r.host.state().ledger[0].controlsOff == 0 && w.r.host.state().ledger[1].controlsOff == 0;
}

bool shippedCollarHolds() {   // G42 + G25 + G41: the collar kills only its wearer; the satellite waits then auto-warps; the policy is the shipped one
    Shipped w(2); if (!w.loaded) return false;
    w.r.host.commit(0, cause(CauseKind::Script, QF("NVDLC01BombCollarQuest")), {KillActor{PF(1)}});
    bool ok = w.r.host.state().ledger[1].av.count(kAvHealth) == 1 && w.r.host.state().ledger[0].av.count(kAvHealth) == 0 && w.r.host.state().kills.count(PF(1)) == 0;
    ok &= w.eng->policy("death.mode", "") == "downed" && w.eng->policy("late_join.during_scene", "") == "wait";
    std::vector<PlayerId> warped; w.eng->gatherRest = [&](const FormRef&, uint32_t, const std::vector<PlayerId>& a, const FormRef&) { warped = a; };
    w.eng->tick(w.r.now);
    w.eng->onNearReported(0, QF("NVDLC03CrashedSatelliteREF"), true); w.eng->onNearReported(2, QF("NVDLC03CrashedSatelliteREF"), true);
    ok &= w.eng->onStageRequested(QF("NVDLC03MQ00"), 100, 0) == StageDecision::Deferred;
    w.eng->tick(w.r.now + 119000); ok &= warped.empty();
    w.eng->tick(w.r.now + 121000); ok &= warped == std::vector<PlayerId>{1};
    return ok;
}
}  // namespace

TEST(rules_w1, cause_source_is_mapped_back_to_the_owning_quest) { CHECK(causeResolverHolds()); }
TEST(rules_w1, choice_consent_asks_the_party_and_host_breaks_ties) { CHECK(choiceConsentHolds()); }
TEST(rules_w1, perks_follow_the_recipient_rules) { CHECK(perkRoutingHolds()); }
TEST(rules_w1, acting_player_copies_of_per_player_vars) { CHECK(actingCopiesHold()); }
TEST(rules_w1, tether_worldspace_timeout_and_auto_warp) { CHECK(tetherWorldAndTimeoutHold()); }
TEST(rules_w1, tether_mode_on_warn_off) { CHECK(tetherModeHolds()); }
TEST(rules_w1, anchor_watch_reports_transitions_with_hysteresis) { CHECK(anchorWatchHolds()); }
TEST(rules_w1, guest_stage_intents_count_as_barrier_passes) { CHECK(passIntentsHold()); }
TEST(rules_w1, info_results_find_their_ready_check_quest) { CHECK(readyInfoMapHolds()); }
TEST(rules_w1, party_owned_companions_do_not_count_against_caps) { CHECK(partyOwnerHolds()); }
TEST(rules_w1, host_reduction_for_xp_tiers) { CHECK(hostReductionHolds()); }
TEST(rules_w1, friendly_fire_guard_blocks_accidents_not_deliberate_attacks) { CHECK(friendlyFireHolds()); }
TEST(rules_w1, death_downed_respawn_and_late_join_policy) { CHECK(deathAndLateJoinHold()); }
TEST(rules_w1, player_kill_in_per_player_state_hits_only_that_player) { CHECK(perPlayerKillHolds()); }
TEST(rules_w1, control_scope_initiator_and_overlapping_bits) { CHECK(controlScopeAndBitsHold()); }

TEST(rules_w1, policy_section_is_validated_and_new_kinds_are_strict) {
    RulesDb db;
    std::string err;
    CHECK(db.load(kW1, &err));
    CHECK(db.policy("death.mode") == "downed" && db.policy("death.revive_s") == "60" && db.policy("nope", "d") == "d");
    auto bad = [&](const std::string& body) {
        std::string t = R"({"schema":1,"quests":{"Q":{"rules":[)" + body + "]}}}";
        RulesDb d; std::string e;
        return !d.load(t, &e) && !e.empty();
    };
    CHECK(bad(R"({"kind":"ChoiceConsent","params":{}})"));
    CHECK(bad(R"({"kind":"ChoiceConsent","params":{"topics":["not an id"]}})"));
    CHECK(bad(R"({"kind":"ChoiceConsent","params":{"topics":["A.esm:01"],"mode":"coin-flip"}})"));
    CHECK(bad(R"({"kind":"PerkRecipient","params":{"default":"nobody"}})"));
    CHECK(bad(R"({"kind":"PerkRecipient","params":{}})"));
    CHECK(bad(R"({"kind":"FriendlyFireGuard","params":{}})"));
    CHECK(bad(R"({"kind":"FriendlyFireGuard","params":{"actors":["A"],"deliberate_hits":0}})"));
    CHECK(bad(R"({"kind":"PerPlayerState","params":{}})"));
    CHECK(bad(R"({"kind":"SetPlayerControls","params":{"scope":"nobody"}})"));
    CHECK(bad(R"({"kind":"CompanionOwner","params":{"owner":"nobody"}})"));
    for (const char* badPolicy : {R"({"schema":1,"quests":{},"policy":[]})", R"({"schema":1,"quests":{},"policy":{"death":1}})", R"({"schema":1,"quests":{},"policy":{"death":{"mode":["x"]}}})"}) {
        RulesDb d; std::string e;
        CHECK(!d.load(badPolicy, &e));
    }
}

// ---- the shipped rules.json -----------------------------------------------------------------------------------------------------------------------------
TEST(rules_w1, the_shipped_rules_load_with_the_wave1_kinds) {
    RulesDb db;
    std::string err;
    if (std::string(FNVCOOP_RULES_JSON).empty() || !db.loadFile(FNVCOOP_RULES_JSON, &err)) { CHECK_MSG(false, "cannot load real rules.json: " + err); return; }
    auto counts = db.kindCounts();
    CHECK(counts["ChoiceConsent"] >= 3);
    CHECK(counts["PerkRecipient"] >= 8);
    CHECK(counts["FriendlyFireGuard"] >= 3);
    CHECK(counts["PerPlayerState"] >= 1);
}

TEST(rules_w1, shipped_data_house_ending_script_moves_locks_and_blocks_saving_for_everyone) { CHECK(shippedCauseHolds()); }
TEST(rules_w1, shipped_data_perk_recipients) { CHECK(shippedPerksHold()); }
TEST(rules_w1, shipped_data_named_forks_are_exclusive) { CHECK(shippedExclusiveHolds()); }
TEST(rules_w1, shipped_data_honest_hearts_pooled_counts_and_weight_gate) { CHECK(shippedPooledHolds()); }
TEST(rules_w1, shipped_data_chargen_pass_stages) { CHECK(shippedPassHolds()); }
TEST(rules_w1, shipped_data_friendly_fire_guard) { CHECK(shippedFriendlyHolds()); }
TEST(rules_w1, shipped_data_ede_belongs_to_the_party) { CHECK(shippedPartyOwnerHolds()); }
TEST(rules_w1, shipped_data_xp_tiers_follow_the_host) { CHECK(shippedHostLevelHolds()); }
TEST(rules_w1, shipped_data_think_tank_weapon_lock_is_per_player) { CHECK(shippedScopeHolds()); }
TEST(rules_w1, shipped_data_collar_satellite_and_policy) { CHECK(shippedCollarHolds()); }

// ---- twins: each re-introduces its gap and the matching scenario must fail ------------------------------------------------------------------------------
TEST(twin, rules_w1_cause_not_resolved) { Mutations m; m.causeNotResolved = true; MutationScope s(m); CHECK(!causeResolverHolds()); }
TEST(twin, rules_w1_choice_initiator_decides) { Mutations m; m.choiceInitiatorDecides = true; MutationScope s(m); CHECK(!choiceConsentHolds()); }
TEST(twin, rules_w1_perk_route_ignored) { Mutations m; m.perkRouteIgnored = true; MutationScope s(m); CHECK(!perkRoutingHolds()); }
TEST(twin, rules_w1_acting_copy_shared) { Mutations m; m.actingCopyShared = true; MutationScope s(m); CHECK(!actingCopiesHold()); }
TEST(twin, rules_w1_tether_ignores_world) { Mutations m; m.tetherIgnoresWorld = true; MutationScope s(m); CHECK(!tetherWorldAndTimeoutHold()); }
TEST(twin, rules_w1_tether_mode_ignored) { Mutations m; m.tetherModeIgnored = true; MutationScope s(m); CHECK(!tetherModeHolds()); }
TEST(twin, rules_w1_anchor_no_hysteresis) { Mutations m; m.anchorWatchNoHysteresis = true; MutationScope s(m); CHECK(!anchorWatchHolds()); }
TEST(twin, rules_w1_pass_intent_ignored) { Mutations m; m.passIntentIgnored = true; MutationScope s(m); CHECK(!passIntentsHold()); }
TEST(twin, rules_w1_ready_info_unmapped) { Mutations m; m.readyInfoUnmapped = true; MutationScope s(m); CHECK(!readyInfoMapHolds()); }
TEST(twin, rules_w1_party_owner_ignored) { Mutations m; m.partyOwnerIgnored = true; MutationScope s(m); CHECK(!partyOwnerHolds()); }
TEST(twin, rules_w1_host_level_as_initiator) { Mutations m; m.hostLevelAsInitiator = true; MutationScope s(m); CHECK(!hostReductionHolds()); }
TEST(twin, rules_w1_friendly_fire_off) { Mutations m; m.friendlyFireOff = true; MutationScope s(m); CHECK(!friendlyFireHolds()); }
TEST(twin, rules_w1_death_policy_off) { Mutations m; m.deathPolicyOff = true; MutationScope s(m); CHECK(!deathAndLateJoinHold()); }
TEST(twin, rules_w1_per_player_state_kills_all) { Mutations m; m.perPlayerStateKillsAll = true; MutationScope s(m); CHECK(!perPlayerKillHolds()); }
TEST(twin, rules_w1_controls_scope_ignored) { Mutations m; m.controlsScopeIgnored = true; MutationScope s(m); CHECK(!controlScopeAndBitsHold()); }
TEST(twin, rules_w1_controls_release_all) { Mutations m; m.controlsReleaseAll = true; MutationScope s(m); CHECK(!controlScopeAndBitsHold()); }
TEST(twin, rules_w1_shipped_cause) { Mutations m; m.causeNotResolved = true; MutationScope s(m); CHECK(!shippedCauseHolds()); }
TEST(twin, rules_w1_shipped_perks) { Mutations m; m.perkRouteIgnored = true; MutationScope s(m); CHECK(!shippedPerksHold()); }
TEST(twin, rules_w1_shipped_exclusive) { Mutations m; m.exclusiveLastWins = true; MutationScope s(m); CHECK(!shippedExclusiveHolds()); }
TEST(twin, rules_w1_shipped_pooled) { Mutations m; m.sumAsAny = true; MutationScope s(m); CHECK(!shippedPooledHolds()); }
TEST(twin, rules_w1_shipped_pass) { Mutations m; m.passIntentIgnored = true; MutationScope s(m); CHECK(!shippedPassHolds()); }
TEST(twin, rules_w1_shipped_friendly) { Mutations m; m.friendlyFireOff = true; MutationScope s(m); CHECK(!shippedFriendlyHolds()); }
TEST(twin, rules_w1_shipped_party_owner) { Mutations m; m.partyOwnerIgnored = true; MutationScope s(m); CHECK(!shippedPartyOwnerHolds()); }
TEST(twin, rules_w1_shipped_host_level) { Mutations m; m.hostLevelAsInitiator = true; MutationScope s(m); CHECK(!shippedHostLevelHolds()); }
TEST(twin, rules_w1_shipped_scope) { Mutations m; m.controlsScopeIgnored = true; MutationScope s(m); CHECK(!shippedScopeHolds()); }
TEST(twin, rules_w1_shipped_collar) { Mutations m; m.perPlayerStateKillsAll = true; MutationScope s(m); CHECK(!shippedCollarHolds()); }
