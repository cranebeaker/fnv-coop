// nvcoop module: installs the L5 hooks at DeferredInit and exposes bridge methods.
//   nvcoop.ini  [coop] role=host|client|guest|gateonly   (default host; client == guest == all hooks + suppression gate)
//   hooks.status                       -> {role, installed, failed, hooks:[{name,status}], criticalOk}
//   capture.begin                      -> clears the capture buffer, (re)enables capture on a host
//   capture.end                        -> {txns:[...], scriptVars:[...], stats:{...}, unmapped:{...}, scripts:{...}}
//   apply.txn {txn:{..}, gate?:true, slot?:0} -> {applied, total, results:[...], scripts:{gated,ran,ranWhileApplying}}
// All run on the main thread.
#include "activate_args.h"
#include <Windows.h>

#include <string>
#include <vector>

#include "PluginAPI.h"
#include "GameObjects.h"
#include "apply_remote.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "effect_json.h"
#include "engine_hooks.h"
#include "guest_local_run.h"
#include "guest_objvars.h"
#include "reload/reload_state.h"

using nvcoop::json::Value;

namespace {

nvhooks::CaptureHub g_hub;
std::unique_ptr<nvhooks::RemoteApplier> g_applier;
nvhooks::InitReport g_report;
NVSEConsoleInterface* g_console = nullptr;
nvhooks::Role g_role = nvhooks::Role::Host;
bool g_inited = false;
size_t g_localRunListed = 0;
std::vector<Value> g_txns, g_vars;

int RunLineSEH(const char* line, void* thisObj) {
    __try {
        return g_console->RunScriptLine2(line, static_cast<TESObjectREFR*>(thisObj), false) ? 1 : 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                       reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

// GuestLocalRun allow-list (G47): the rule in rules.json IS the list. Read from the same rules file the rules engine loads ([coop] rules=<path>,
// default nvcoop_rules.json next to the dll). nvcoop_guest_local_run.json is only the fallback for an install that has no rules file at all
// (it is generated from the same rules, hooks/tools/gen_guest_local_run.py, and a unit test keeps the two identical). Neither file = empty list
// (every quest script is gated on a guest, the old behaviour).
std::string ReadTextFile(const std::string& path) {
    std::string text;
    HANDLE f = CreateFileA(path.c_str(), GENERIC_READ, FILE_SHARE_READ, nullptr, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, nullptr);
    if (f == INVALID_HANDLE_VALUE) return text;
    char buf[8192];
    DWORD n = 0;
    while (ReadFile(f, buf, sizeof buf, &n, nullptr) && n) text.append(buf, n);
    CloseHandle(f);
    return text;
}
void LoadGuestLocalRun() {
    const std::string ini = IniPath();
    const std::string dir = ini.substr(0, ini.find_last_of("\\/") + 1);
    char rulesIni[MAX_PATH] = {};
    GetPrivateProfileStringA("coop", "rules", "", rulesIni, sizeof rulesIni, ini.c_str());
    std::string rulesPath = rulesIni[0] ? std::string(rulesIni) : dir + "nvcoop_rules.json";
    nvhooks::GuestLocalRunList list;
    std::string err, source;
    const std::string rulesText = ReadTextFile(rulesPath);
    if (!rulesText.empty()) {
        if (nvhooks::parseGuestLocalRunRules(rulesText, &list, &err)) source = rulesPath;
        else nvcoop::Log("hooks: GuestLocalRun: rules file unusable (%s): %s", rulesPath.c_str(), err.c_str());
    }
    if (source.empty()) {
        const std::string legacyPath = dir + "nvcoop_guest_local_run.json";
        const std::string legacy = ReadTextFile(legacyPath);
        if (legacy.empty()) { nvcoop::Log("hooks: GuestLocalRun list not found (rules %s, fallback %s)", rulesPath.c_str(), legacyPath.c_str()); return; }
        if (!nvhooks::parseGuestLocalRun(legacy, &list, &err)) { nvcoop::Log("hooks: GuestLocalRun list invalid: %s", err.c_str()); return; }
        source = legacyPath;
    }
    nvhooks::setGuestLocalRun(list.scripts);
    g_localRunListed = list.scripts.size();
    nvcoop::Log("hooks: GuestLocalRun allow-list from %s: %d scripts (%d malformed)", source.c_str(), static_cast<int>(list.scripts.size()), list.skipped);
    nvcoop::Log("hooks: guest quest-var rollback table: %d scripts, %d variables", static_cast<int>(nvhooks::ObjectVarWrites::builtin().scripts()),
                static_cast<int>(nvhooks::ObjectVarWrites::builtin().rows()));
}

NVSEInterface* g_nvse = nullptr;

// xNVSE's EventManager hooks MarkBaseExtraListScriptEvent (0x5AC750) lazily, once, the first time any game-event handler is set (our
// bots/loot OnDrop handlers do that after Init), and its jump never calls a previous hook. Registering and removing a no-op handler for
// one of those events before Init makes that one-shot install happen first, so our MarkScriptEvent hook chains on top of it
// (hook_patch rule 4): the guest event observer keeps running, and a hot reload can unhook (the old order left our bytes overwritten,
// so every client swap was refused). The handler is removed at once; the xNVSE jump stays for good, as it would anyway.
void PrimeNoopHandler(TESObjectREFR*, void*) {}
void PrimeXnvseEventHook() {
    auto* ev = g_nvse ? static_cast<NVSEEventManagerInterface*>(g_nvse->QueryInterface(kInterface_EventManager)) : nullptr;
    if (!ev || !ev->SetNativeEventHandler || !ev->RemoveNativeEventHandler) {
        nvcoop::Log("hooks: xNVSE event manager unavailable, MarkScriptEvent hook not primed");
        return;
    }
    const bool set = ev->SetNativeEventHandler("OnDrop", &PrimeNoopHandler);
    if (set) ev->RemoveNativeEventHandler("OnDrop", &PrimeNoopHandler);
    nvcoop::Log("hooks: xNVSE event hook primed (%s)", set ? "handler set and removed" : "set refused");
}

const char* RoleName(nvhooks::Role r) {
    return r == nvhooks::Role::Host ? "host" : r == nvhooks::Role::Guest ? "guest" : "gateonly";
}

void InitHooks() {
    if (g_inited) return;
    g_inited = true;
    char role[32] = {};
    GetPrivateProfileStringA("coop", "role", "host", role, sizeof role, IniPath().c_str());
    std::string r = role;
    g_role = (r == "client" || r == "guest") ? nvhooks::Role::Guest : r == "gateonly" ? nvhooks::Role::Client : nvhooks::Role::Host;
    g_hub.addSink([](fnvcoop::EffectTxn&& t) { g_txns.push_back(nvhooks::txnToJson(t)); });
    // story-capture (docs/tools/story-capture.md): ~75 more script commands become RefCommand effects. OFF unless [hooks] story_capture=1.
    nvhooks::setStoryCapture(GetPrivateProfileIntA("hooks", "story_capture", 0, IniPath().c_str()) != 0);
    if (g_role != nvhooks::Role::Client) PrimeXnvseEventHook();   // before Init: MarkScriptEvent is hooked on the host (puppet combat events) and on a guest
    g_report = nvhooks::Init(g_hub, g_role);
    if (const std::string ch = nvhooks::chainedHookTargets(); !ch.empty()) nvcoop::Log("hooks: chained on top of another patcher: %s", ch.c_str());
    LoadGuestLocalRun();
    nvhooks::RemoteApplier::setTrace([](const std::string& line) { nvcoop::Log("%s", line.c_str()); });   // one line per SetStage applied here (story fix: done flags)
    g_applier.reset(new nvhooks::RemoteApplier(g_hub, nvhooks::resolver()));
    auto exec = [](const std::string& line, uint32_t refID) {
        void* ref = refID ? nvhooks::resolver().lookup(refID) : nullptr;
        if (refID && !ref) return false;
        // The calling reference must be a reference (REFR / ACHR / ACRE). Running a line "on" any other form (a cell, a quest)
        // makes the engine treat it as a TESObjectREFR: a guest's cell report run as `Activate` on a cell hung the host's main
        // thread after a save load (two-copy fight test 2026-10-21).
        if (ref) {
            const uint8_t t = static_cast<TESForm*>(ref)->typeID;
            if (t != kFormType_TESObjectREFR && t != kFormType_Character && t != kFormType_Creature) {
                nvcoop::Log("hooks: console line '%s' refused: form %08X is type %02X, not a reference", line.c_str(), refID, t);
                return false;
            }
        }
        return RunLineSEH(line.c_str(), ref) == 1;
    };
    if (g_console) nvhooks::setConsoleExec(exec);
    if (g_console) g_applier->setExecLine(exec);
    nvcoop::Log("hooks: role=%s installed=%d failed=%d critical=%d", RoleName(g_role), g_report.ok, g_report.failed,
                g_report.allCritical() ? 1 : 0);
    for (auto& h : g_report.hooks) nvcoop::Log("hooks:   %-34s %s", h.name.c_str(), nvhooks::statusName(h.status));
}

Value ScriptsJson() {
    const auto& s = nvhooks::scriptStats();
    Value o = Value::object();
    o.set("gated", (int64_t)s.gated).set("ran", (int64_t)s.ran).set("ranWhileApplying", (int64_t)s.ranWhileApplying)
        .set("questVarGuarded", (int64_t)s.questVarGuarded).set("questVarRollbacks", (int64_t)s.questVarRollbacks)
        .set("stagesBlocked", (int64_t)s.stagesBlocked).set("stagesForwarded", (int64_t)s.stagesForwarded);
    return o;
}

bool MStatus(const std::string&, std::string& out) {
    Value o = Value::object(), hs = Value::array();
    for (auto& h : g_report.hooks) {
        Value e = Value::object();
        e.set("name", h.name).set("status", nvhooks::statusName(h.status));
        hs.push(e);
    }
    o.set("role", RoleName(g_role)).set("inited", g_inited).set("installed", g_report.ok).set("failed", g_report.failed)
        .set("criticalOk", g_report.allCritical()).set("hooks", hs);
    out = o.dump();
    return true;
}

bool MBegin(const std::string&, std::string& out) {
    if (!g_inited) { out = "hooks not initialised (DeferredInit not seen yet)"; return false; }
    g_txns.clear();
    g_vars.clear();
    g_hub.resetStats();
    g_hub.setEnabled(g_role == nvhooks::Role::Host);
    Value o = Value::object();
    o.set("capturing", g_hub.enabled()).set("role", RoleName(g_role));
    out = o.dump();
    return true;
}

bool MEnd(const std::string&, std::string& out) {
    Value txns = Value::array(), vars = Value::array();
    for (auto& t : g_txns) txns.push(t);
    for (auto& v : g_vars) vars.push(v);
    const auto& st = g_hub.stats();
    Value stats = Value::object();
    stats.set("recorded", (int64_t)st.recorded).set("droppedNoScope", (int64_t)st.droppedNoScope)
        .set("staleScopes", (int64_t)st.staleScopes).set("weakTxns", (int64_t)st.weakTxns).set("weakFlushes", (int64_t)st.weakFlushes).set("collapsed", (int64_t)st.collapsed).set("droppedEcho", (int64_t)st.droppedEcho).set("droppedLocal", (int64_t)st.droppedLocal).set("txns", (int64_t)st.txns).set("emptyScopes", (int64_t)st.emptyScopes);
    const auto& um = nvhooks::unmapped();
    Value un = Value::object();
    un.set("questCompleted", (int64_t)um.questCompleted).set("itemOnNonPlayer", (int64_t)um.itemOnNonPlayer)
        .set("avOnNonPlayer", (int64_t)um.avOnNonPlayer);
    Value o = Value::object();
    o.set("txns", txns).set("scriptVars", vars).set("stats", stats).set("unmapped", un).set("scripts", ScriptsJson());
    g_txns.clear();
    g_vars.clear();
    out = o.dump();
    return true;
}

// exec.ref {ref:"FalloutNV.esm:104C79", cmd:"KillActor"}: run a console command with that reference as the calling reference
// (test/diagnostic RPC; goes through the same Script::Execute path a script would, so guest suppression applies).
bool MExecRef(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    fnvcoop::FormRef r;
    if (!nvhooks::strToRef(p.get("ref").asString(""), &r)) { out = "bad ref"; return false; }
    uint32_t id = nvhooks::resolver().fromRef(r);
    if (!id) { out = "plugin not loaded"; return false; }
    bool ok;
    std::unique_ptr<nvhooks::ActingPlayerScope> acting;
    if (p.find("acting")) {  // run as a guest's ghost: {ghost:"ref", slot:n} (CompanionOwner.owner etc. take the slot)
        fnvcoop::FormRef gr;
        if (!nvhooks::strToRef(p.get("acting").get("ghost").asString(""), &gr)) { out = "bad acting.ghost"; return false; }
        uint32_t gid = nvhooks::resolver().fromRef(gr);
        if (!gid) { out = "acting.ghost not loaded"; return false; }
        nvhooks::registerPlayerSlot(gid, static_cast<uint8_t>(p.get("acting").get("slot").asInt(1)));
        acting.reset(new nvhooks::ActingPlayerScope(gid));
    }
    if (p.get("scope").asBool(false)) {  // run inside a capture scope so the mutation hooks record it (a bare console line is "ambient")
        fnvcoop::Cause c;
        c.kind = fnvcoop::CauseKind::Console;
        fnvcoop::FormRef src;
        if (p.find("source") && nvhooks::strToRef(p.get("source").asString(""), &src)) {  // pretend a quest stage runs the command (scope needs a source)
            c.kind = fnvcoop::CauseKind::QuestResult;
            c.source = src;
        }
        nvhooks::ScopedCapture sc(g_hub, c);
        ok = nvhooks::execConsoleLineOn(p.get("cmd").asString(""), id);
    } else {
        ok = nvhooks::execConsoleLineOn(p.get("cmd").asString(""), id);
    }
    Value o = Value::object();
    o.set("ok", ok);
    out = o.dump();
    return true;
}

// ref.info {ref}: engine state read directly (no script): position, disabled flag, Actor life state (+0x108: 0 alive, 1 dying, 2 dead).
bool MRefInfo(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    fnvcoop::FormRef r;
    if (!nvhooks::strToRef(p.get("ref").asString(""), &r)) { out = "bad ref"; return false; }
    uint32_t id = nvhooks::resolver().fromRef(r);
    TESObjectREFR* ref = id ? static_cast<TESObjectREFR*>(nvhooks::resolver().lookup(id)) : nullptr;
    if (!ref) { out = "reference not loaded"; return false; }
    Value o = Value::object();
    o.set("x", (double)ref->posX).set("y", (double)ref->posY).set("z", (double)ref->posZ)
        .set("lifeState", (int64_t)*reinterpret_cast<uint32_t*>(reinterpret_cast<char*>(ref) + 0x108))
        .set("flags", (int64_t)ref->flags).set("type", (int64_t)ref->typeID)
        .set("cell", (int64_t)(ref->parentCell ? ref->parentCell->refID : 0));
    out = o.dump();
    return true;
}

// vars.dump {quest}: all (id,value) pairs of the quest script's variable list as pollHostState sees them.
bool MVarsDump(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    Value arr = Value::array();
    for (const Value& q : p.get("quests").items()) {
        fnvcoop::FormRef r;
        Value e = Value::object();
        e.set("quest", q.asString(""));
        Value vars = Value::object();
        if (nvhooks::strToRef(q.asString(""), &r))
            for (auto& kv : nvhooks::questVarsDump(r)) vars.set(std::to_string(kv.first), kv.second);
        e.set("vars", vars);
        arr.push(e);
    }
    Value o = Value::object();
    o.set("quests", arr);
    out = o.dump();
    return true;
}

bool MControls(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    nvhooks::SetPlayerControlsLocked(p.get("locked").asBool(false));
    Value o = Value::object();
    o.set("locked", nvhooks::playerControlsLocked());
    out = o.dump();
    return true;
}

bool MApply(const std::string& params, std::string& out) {
    if (!g_inited || !g_applier) { out = "hooks not initialised"; return false; }
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    fnvcoop::EffectTxn txn;
    if (!nvhooks::txnFromJson(p.get("txn"), &txn, &err)) { out = "bad txn: " + err; return false; }
    g_applier->setUseGate(p.get("gate").asBool(true));
    auto slot = static_cast<fnvcoop::PlayerId>(p.get("slot").asInt(0));
    auto before = nvhooks::scriptStats();
    Value results = Value::array();
    size_t applied = 0;
    for (const fnvcoop::Effect& e : txn.effects) {
        fnvcoop::EffectTxn one;
        one.origin = txn.origin;
        one.effects.push_back(e);
        auto st = g_applier->apply(e);  // recipient filtering is not needed for the single-player proof
        (void)one;
        (void)slot;
        if (st == nvhooks::ApplyStatus::Ok) ++applied;
        Value r = Value::object();
        r.set("type", nvhooks::effectToJson(e).get("type").asString("?")).set("status", nvhooks::applyStatusName(st))
            .set("error", g_applier->lastError());
        results.push(r);
    }
    auto after = nvhooks::scriptStats();
    Value sc = Value::object();
    sc.set("gated", (int64_t)(after.gated - before.gated)).set("ran", (int64_t)(after.ran - before.ran))
        .set("ranWhileApplying", (int64_t)(after.ranWhileApplying - before.ranWhileApplying));
    Value o = Value::object();
    o.set("applied", (int64_t)applied).set("total", (int64_t)txn.effects.size()).set("results", results).set("scripts", sc)
        .set("gate", p.get("gate").asBool(true));
    out = o.dump();
    return true;
}

// observe.set {events:"off|count|suppress", activate:"off|count|suppress"}; observe.stats -> counts + last target. Observers only fire on a guest,
// outside scripts, for the local player (see engine_hooks.cpp), so this is the live check for the guest event / activate observers.
struct ObsState {
    int64_t events = 0, activates = 0, lastMask = 0;
    std::string lastEventTarget, lastActivateTarget;
    std::string eventMode = "off", activateMode = "off";
} g_obs;

bool MObserveSet(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    if (p.find("events")) g_obs.eventMode = p.get("events").asString("off");
    if (p.find("activate")) g_obs.activateMode = p.get("activate").asString("off");
    if (p.get("reset").asBool(false)) { g_obs.events = g_obs.activates = g_obs.lastMask = 0; }
    nvhooks::setEventObserver(g_obs.eventMode == "off" ? nvhooks::EventObserver() : nvhooks::EventObserver([](uint32_t mask, const fnvcoop::FormRef& t) {
        ++g_obs.events;
        g_obs.lastMask = mask;
        g_obs.lastEventTarget = nvhooks::refToStr(t);
        return g_obs.eventMode == "suppress" ? nvhooks::EventAction::Suppress : nvhooks::EventAction::Proceed;
    }));
    nvhooks::setActivateObserver(g_obs.activateMode == "off" ? nvhooks::ActivateObserver() : nvhooks::ActivateObserver([](const fnvcoop::FormRef& t) {
        ++g_obs.activates;
        g_obs.lastActivateTarget = nvhooks::refToStr(t);
        return g_obs.activateMode == "suppress" ? nvhooks::ActivateAction::Suppress : nvhooks::ActivateAction::Proceed;
    }));
    Value o = Value::object();
    o.set("events", g_obs.eventMode).set("activate", g_obs.activateMode);
    out = o.dump();
    return true;
}

bool MObserveStats(const std::string&, std::string& out) {
    Value o = Value::object();
    o.set("events", g_obs.events).set("lastMask", g_obs.lastMask).set("lastEventTarget", g_obs.lastEventTarget).set("activates", g_obs.activates)
        .set("lastActivateTarget", g_obs.lastActivateTarget).set("scriptDepth", (int64_t)nvhooks::suppressStats().mutations);
    out = o.dump();
    return true;
}

// activate.ref {ref}: calls the ENGINE's TESObjectREFR::Activate (0x573170) with the local player as activator, outside any script - exactly
// what a player key press does - so the guest activate observer is exercised. event.mark {ref, mask}: same for MarkBaseExtraListScriptEvent.
using ActivateFn = UInt32(__thiscall*)(void*, void*, UInt8, UInt32, UInt32);
using MarkFn = bool(__cdecl*)(void*, void*, UInt32);
bool CallActivateSEH(void* ref, void* player, UInt32* ret) {
    __try {
        const nvhooks::ActivateArgs a = nvhooks::consoleActivateArgs();   // the console's (0, 0, 1): the last argument 0 deletes a placed item
        *ret = reinterpret_cast<ActivateFn>(0x573170)(ref, player, a.flagA, a.flagB, a.last);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallMarkSEH(void* player, void* xlist, UInt32 mask, bool* ret) {
    __try {
        *ret = reinterpret_cast<MarkFn>(0x5AC750)(player, xlist, mask);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
void* RefFromParam(const Value& p, const char* key) {
    fnvcoop::FormRef r;
    if (!nvhooks::strToRef(p.get(key).asString(""), &r)) return nullptr;
    uint32_t id = nvhooks::resolver().fromRef(r);
    return id ? nvhooks::resolver().lookup(id) : nullptr;
}
bool MActivateRef(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    void* ref = RefFromParam(p, "ref");
    void* player = nvhooks::resolver().lookup(nvhooks::kHostPlayerRef);
    if (!ref || !player) { out = "ref or player not loaded"; return false; }
    UInt32 ret = 0;
    bool ok = CallActivateSEH(ref, player, &ret);
    Value o = Value::object();
    o.set("ok", ok).set("returned", (int64_t)ret);
    out = o.dump();
    return true;
}
bool MEventMark(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    TESObjectREFR* ref = static_cast<TESObjectREFR*>(RefFromParam(p, "ref"));
    void* player = nvhooks::resolver().lookup(nvhooks::kHostPlayerRef);
    if (!ref || !player) { out = "ref or player not loaded"; return false; }
    bool ret = false;
    bool ok = CallMarkSEH(player, &ref->extraDataList, static_cast<UInt32>(p.get("mask").asInt(1)), &ret);
    Value o = Value::object();
    o.set("ok", ok).set("returned", ret);
    out = o.dump();
    return true;
}

// perf.hooks {on:true|false}: QPC accounting of our code inside the Script::Execute detour. Reading it (on:false) returns the totals.
bool MPerf(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    if (p.find("on") && p.get("on").asBool(false)) nvhooks::setHookPerf(true);
    auto s = nvhooks::hookPerf();
    if (p.find("on") && !p.get("on").asBool(false)) nvhooks::setHookPerf(false);
    Value o = Value::object();
    o.set("calls", (int64_t)s.calls).set("overheadUs", s.freq ? 1e6 * (double)s.overheadTicks / (double)s.freq : 0.0)
        .set("usPerCall", s.calls ? 1e6 * (double)s.overheadTicks / (double)s.freq / (double)s.calls : 0.0);
    out = o.dump();
    return true;
}

// guest.stats: GuestLocalRun proof. allowListed = list size, localRan = allow-listed script executions since start, perScript = which ones,
// questScriptsSkipped / mutationsBlocked = what the guest gate suppressed.
bool MGuestStats(const std::string&, std::string& out) {
    const auto& s = nvhooks::scriptStats();
    const auto& sp = nvhooks::suppressStats();
    Value o = Value::object(), per = Value::array();
    for (auto& kv : nvhooks::localRunCounts()) {
        fnvcoop::FormRef r;
        Value e = Value::object();
        e.set("script", nvhooks::resolver().toRef(kv.first, &r) ? nvhooks::refToStr(r) : std::string("?")).set("runs", (int64_t)kv.second);
        per.push(e);
    }
    o.set("role", RoleName(g_role)).set("allowListed", (int64_t)g_localRunListed).set("localRan", (int64_t)s.localRan).set("ran", (int64_t)s.ran)
        .set("questScriptsSkipped", (int64_t)sp.questScripts).set("mutationsBlocked", (int64_t)sp.mutations)
        .set("resultScriptsBlocked", (int64_t)sp.resultScripts).set("perScript", per)
        .set("questVarGuarded", (int64_t)s.questVarGuarded).set("questVarRollbacks", (int64_t)s.questVarRollbacks)
        .set("stagesBlocked", (int64_t)s.stagesBlocked).set("stagesForwarded", (int64_t)s.stagesForwarded)
        .set("objVarScripts", (int64_t)nvhooks::ObjectVarWrites::builtin().scripts()).set("objVarRows", (int64_t)nvhooks::ObjectVarWrites::builtin().rows());
    out = o.dump();
    return true;
}

// party.set {members:[{slot,ref?}], reduce:"initiator"|"min"|"max"|"any"}: registers a test party (slot 0 = the local player; other members need
// a loaded `ref`) and a simple reducer, so Condition evaluation can be proven live without the session. {members:[]} clears it.
std::string g_testReduce = "initiator";
bool MPartySet(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    std::vector<nvhooks::PartyMember> members;
    for (const Value& m : p.get("members").items()) {
        nvhooks::PartyMember pm;
        pm.slot = static_cast<uint8_t>(m.get("slot").asInt(0));
        fnvcoop::FormRef r;
        if (m.find("ref") && nvhooks::strToRef(m.get("ref").asString(""), &r)) pm.refID = nvhooks::resolver().fromRef(r);
        else pm.refID = nvhooks::kHostPlayerRef;
        if (!pm.refID) { out = "member ref not loaded"; return false; }
        members.push_back(pm);
    }
    g_testReduce = p.get("reduce").asString("initiator");
    if (members.empty()) {
        nvhooks::setPartyConditions(nullptr, nullptr);
    } else {
        nvhooks::setPartyConditions(
            [members]() { return members; },
            [](const fnvcoop::FormRef&, const std::string&, const std::vector<nvhooks::CondValue>& v, uint8_t init, fnvcoop::CondPhase) {
                double best = v.front().value;
                if (g_testReduce == "initiator") { for (auto& x : v) if (x.slot == init) return x.value; return v.front().value; }
                for (auto& x : v) {
                    if (g_testReduce == "min") best = x.value < best ? x.value : best;
                    else if (g_testReduce == "max") best = x.value > best ? x.value : best;
                    else if (g_testReduce == "any") { if (x.value != 0) return 1.0; best = 0; }
                }
                return best;
            });
    }
    Value o = Value::object();
    o.set("members", (int64_t)members.size()).set("reduce", g_testReduce);
    out = o.dump();
    return true;
}

// cond.eval {func:"GetDistance", subject:"ref", param1?:"ref", op:"<", value:500}: runs the ENGINE's Condition::Evaluate (0x681600) on a
// synthetic condition, so the real condition path (and our evaluator hooks) is exercised. Returns {result, stats}.
using CondEvalFn = bool(__thiscall*)(Condition*, void*, void*, bool*);
bool EvalCondSEH(CondEvalFn eval, Condition* c, void* subject, bool* flag, bool* result) {
    __try {
        *result = eval(c, subject, nullptr, flag);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool MCondEval(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    static const struct { const char* n; UInt32 idx; } kFuncs[] = {{"GetDistance", 1}, {"GetInCell", 0x43}, {"GetInSameCell", 0x20}, {"IsInCombat", 0x121},
                                                                      {"GetDetected", 0x2d}, {"GetHealthPercentage", 0x1af}};
    UInt32 idx = 0xFFFFFFFF;
    for (auto& f : kFuncs) if (p.get("func").asString("") == f.n) idx = f.idx;
    if (idx == 0xFFFFFFFF) { out = "unknown func"; return false; }
    auto refOf = [&](const char* key) -> void* {
        fnvcoop::FormRef r;
        if (!p.find(key)) return nullptr;
        if (p.get(key).asString("") == "player") return nvhooks::resolver().lookup(nvhooks::kHostPlayerRef);
        if (!nvhooks::strToRef(p.get(key).asString(""), &r)) return nullptr;
        uint32_t id = nvhooks::resolver().fromRef(r);
        return id ? nvhooks::resolver().lookup(id) : nullptr;
    };
    void* subject = refOf("subject");
    void* param1 = refOf("param1");
    if (!subject) { out = "subject not loaded"; return false; }
    static const char* kOps[] = {"==", "!=", ">", ">=", "<", "<="};
    UInt8 op = 0;
    for (UInt8 i = 0; i < 6; ++i) if (p.get("op").asString("==") == kOps[i]) op = i;
    Condition c = {};
    c.type = static_cast<UInt8>(op << 5);
    c.comparisonValue.value = static_cast<float>(p.get("value").asNumber(0));
    c.function = idx;
    c.parameter1.pointer = param1;
    c.runOn = 0;  // Subject
    bool flag = false;
    CondEvalFn eval = reinterpret_cast<CondEvalFn>(0x681600);
    auto before = nvhooks::condStats();
    bool result = false;
    bool crashed = !EvalCondSEH(eval, &c, subject, &flag, &result);
    auto after = nvhooks::condStats();
    Value o = Value::object();
    o.set("result", result).set("crashed", crashed).set("evals", (int64_t)(after.evals - before.evals))
        .set("partyEvals", (int64_t)(after.partyEvals - before.partyEvals)).set("reduced", (int64_t)(after.reduced - before.reduced));
    out = o.dump();
    return true;
}

void OnLoad(void* nvse) {
    g_nvse = static_cast<NVSEInterface*>(nvse);
    if (nvse) g_console = static_cast<NVSEConsoleInterface*>(static_cast<NVSEInterface*>(nvse)->QueryInterface(kInterface_Console));
    nvcoop::RegisterMethod("hooks.status", MStatus, true);
    nvcoop::RegisterMethod("capture.begin", MBegin, true);
    nvcoop::RegisterMethod("capture.end", MEnd, true);
    nvcoop::RegisterMethod("apply.txn", MApply, true);
    nvcoop::RegisterMethod("controls.lock", MControls, true);
    nvcoop::RegisterMethod("exec.ref", MExecRef, true);
    nvcoop::RegisterMethod("ref.info", MRefInfo, true);
    nvcoop::RegisterMethod("vars.dump", MVarsDump, true);
    nvcoop::RegisterMethod("guest.stats", MGuestStats, true);
    nvcoop::RegisterMethod("observe.set", MObserveSet, true);
    nvcoop::RegisterMethod("observe.stats", MObserveStats, true);
    nvcoop::RegisterMethod("activate.ref", MActivateRef, true);
    nvcoop::RegisterMethod("event.mark", MEventMark, true);
    nvcoop::RegisterMethod("perf.hooks", MPerf, true);
    nvcoop::RegisterMethod("party.set", MPartySet, true);
    nvcoop::RegisterMethod("cond.eval", MCondEval, true);
}
void OnMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_DeferredInit) InitHooks();
    else if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console)
        nvhooks::Shutdown();
}

}  // namespace

// Per main-loop frame (outside any script): close capture scopes whose end() never ran.
void OnFrame() {
    if (g_inited) nvhooks::RemoteApplier::tickAll();   // parked MoveRefs whose destination cell has loaded since (about once a second)
    if (g_inited && g_hub.enabled()) {
        size_t n = g_hub.reapStale();
        if (n) nvcoop::Log("hooks: reaped %zu stale capture scope(s)", n);
    }
}

NVCOOP_MODULE(l5_hooks, "l5_hooks", OnLoad, OnMessage, OnFrame);

// Hot reload (T3b, docs/tools/T3b-state.md): hooks are uninstalled (bytes restored and read back) before the core is unloaded
// and reinstalled by the new core. Nothing else here is worth carrying: the capture hub and the applier are rebuilt by InitHooks.
NVCOOP_RELOAD_STATE(l5_hooks, "l5_hooks", 1, 10,
    /*canUnload*/ [] {
        int n = nvhooks::auditTamperedHooks();
        return n ? std::to_string(n) + " engine hook(s) were chained over by another mod (" + nvhooks::tamperedHookTargets() + ")" : std::string();
    },
    /*save*/ [](nvreload::Writer& w) {
        w.boolean(g_inited);
        w.u8(static_cast<uint8_t>(g_role));
        return true;
    },
    /*restore*/ [](uint32_t, nvreload::Reader& r) {
        bool wasInited = r.boolean();
        r.u8();
        if (wasInited) InitHooks();  // idempotent; the module's DeferredInit handler would do the same
        return true;
    },
    /*migrate*/ nullptr,
    /*shutdown*/ [] { if (g_inited) nvhooks::Shutdown(); },
    /*fresh*/ nullptr);
