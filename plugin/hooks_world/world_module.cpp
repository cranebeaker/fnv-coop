// nvcoop module for L12: installs the world hooks after L5's hub exists and exposes bridge methods.
//   world.status                  -> {inited, role, installed, failed, hooks:[{name,status}], ledger:{pending,created}, ...}
//   (world.peek lives in world_peek.cpp, always built)
//   world.drain                   -> {effects:[...]}   lane-local WorldEffects recorded on the host since the last drain
//   world.apply {effect:{..}}     -> {status,error}    SpawnRef/DespawnRef/RefScriptVar (synccore Effect JSON) or a WorldEffect
//   world.flush                   -> {created}         create queued spawns whose cell is loaded now, release held follow-ups
//   world.baseline                -> {effects:[...],world:[...]}  host: net spawns / ref vars / NPC state for a joiner
//   world.applybaseline {effects,world} -> {applied}   client: delete stale spawns, apply the baseline
//   world.refvars {}              -> {changes:[...]}   one full RefScriptVar poll pass (diagnostic)
//   world.refvar {ref,index}      -> {value}           read a script variable of a reference
//   world.watch {ref}             -> {}                poll this ref's script variables even outside the player's cell
//   world.tick {}                 -> {}                force a hostTick pass now (ignores the 15-frame throttle)
//   world.probe/scripted/getvar/setvar/execid/weather  diagnostics + test helpers (setvar needs NVCOOP_TEST_HELPERS=1)
// All run on the main thread.
#include <Windows.h>

#include <cstdlib>
#include <string>
#include <vector>

#include "PluginAPI.h"
#include "GameExtraData.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "effect_json.h"
#include "world_hooks.h"
#include "reload/reload_state.h"

using nvcoop::json::Value;

namespace {

nvworld::InitReport g_report;
std::unique_ptr<nvworld::WorldApplier> g_applier;
nvhooks::Role g_role = nvhooks::Role::Host;
bool g_inited = false;
uint32_t g_frame = 0;

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

const char* RoleName(nvhooks::Role r) {
    return r == nvhooks::Role::Host ? "host" : r == nvhooks::Role::Guest ? "guest" : "gateonly";
}

// L5 must be initialised first (we need its hub); retried every frame until it is.
void TryInit() {
    if (g_inited) return;
    nvhooks::CaptureHub* hub = nvhooks::hub();
    if (!hub) return;
    g_inited = true;
    char role[32] = {};
    GetPrivateProfileStringA("coop", "role", "host", role, sizeof role, IniPath().c_str());
    // the NV:MP launcher (TTW-CoOp.cmd) passes the role: it wins over the ini, like in coop_module.cpp LoadConfig
    char envRole[32] = {};
    const DWORD envN = GetEnvironmentVariableA("NVCOOP_ROLE", envRole, sizeof envRole);
    if (envN > 0 && envN < sizeof envRole) std::snprintf(role, sizeof role, "%s", std::string(envRole) == "join" ? "client" : envRole);
    std::string r = role;
    g_role = (r == "client" || r == "guest") ? nvhooks::Role::Guest : r == "gateonly" ? nvhooks::Role::Client : nvhooks::Role::Host;
    g_report = nvworld::Init(*hub, g_role);
    g_applier.reset(new nvworld::WorldApplier(*hub, nvhooks::resolver()));
    g_applier->setExecLine([](const std::string& line, uint32_t refID) { return nvhooks::execConsoleLineOn(line, refID); });
    nvcoop::Log("world: role=%s installed=%d failed=%d", RoleName(g_role), g_report.ok, g_report.failed);
    for (auto& h : g_report.hooks) nvcoop::Log("world:   %-34s %s", h.name.c_str(), nvhooks::statusName(h.status));
}

bool Parse(const std::string& params, Value& p, std::string& out) {
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) {
        out = "bad params: " + err;
        return false;
    }
    return true;
}

bool MStatus(const std::string&, std::string& out) {
    Value hs = Value::array();
    for (auto& h : g_report.hooks) {
        Value e = Value::object();
        e.set("name", h.name).set("status", nvhooks::statusName(h.status));
        hs.push(e);
    }
    Value led = Value::object();
    led.set("pending", (int64_t)nvworld::ledger().pending()).set("created", (int64_t)nvworld::ledger().map().size())
        .set("deferred", (int64_t)nvworld::ledger().deferred().size()).set("deferredDropped", (int64_t)nvworld::ledger().deferred().dropped())
        .set("deferredExpired", (int64_t)nvworld::ledger().deferred().expired());
    Value o = Value::object();
    o.set("inited", g_inited).set("role", RoleName(g_role)).set("installed", g_report.ok).set("failed", g_report.failed)
        .set("hooks", hs).set("ledger", led);
    out = o.dump();
    return true;
}

bool MDrain(const std::string&, std::string& out) {
    Value arr = Value::array();
    for (auto& e : nvworld::drainWorldEffects()) {
        Value v;
        std::string err;
        nvcoop::json::parse(nvworld::worldEffectToJson(e), v, err);
        arr.push(v);
    }
    Value o = Value::object();
    o.set("effects", arr);
    out = o.dump();
    return true;
}

bool MApply(const std::string& params, std::string& out) {
    if (!g_inited || !g_applier) { out = "world hooks not initialised"; return false; }
    Value p;
    if (!Parse(params, p, out)) return false;
    nvhooks::ApplyStatus st;
    fnvcoop::Effect e;
    nvworld::WorldEffect we;
    std::string err;
    const Value& ev = p.get("effect");
    if (nvhooks::effectFromJson(ev, &e, &err) && nvworld::handlesEffect(e)) {
        st = g_applier->apply(e);
    } else if (nvworld::worldEffectFromJson(ev.dump(), &we, &err)) {
        st = g_applier->applyWorld(we);
    } else {
        out = "unsupported effect: " + err;
        return false;
    }
    Value o = Value::object();
    o.set("status", nvhooks::applyStatusName(st)).set("error", g_applier->lastError());
    out = o.dump();
    return true;
}

bool MFlush(const std::string&, std::string& out) {
    if (!g_applier) { out = "world hooks not initialised"; return false; }
    Value o = Value::object();
    o.set("created", (int64_t)g_applier->flushQueued());
    out = o.dump();
    return true;
}

bool MBaseline(const std::string&, std::string& out) {
    nvworld::BaselineSnapshot b = nvworld::hostBaseline();
    Value fx = Value::array(), wf = Value::array();
    for (auto& e : b.effects) fx.push(nvhooks::effectToJson(e));
    for (auto& w : b.world) {
        Value v;
        std::string err;
        nvcoop::json::parse(nvworld::worldEffectToJson(w), v, err);
        wf.push(v);
    }
    Value o = Value::object();
    o.set("effects", fx).set("world", wf);
    out = o.dump();
    return true;
}

bool MApplyBaseline(const std::string& params, std::string& out) {
    if (!g_inited || !g_applier) { out = "world hooks not initialised"; return false; }
    Value p;
    if (!Parse(params, p, out)) return false;
    nvworld::BaselineSnapshot b;
    std::string err;
    for (const Value& v : p.get("effects").items()) {
        fnvcoop::Effect e;
        if (!nvhooks::effectFromJson(v, &e, &err)) { out = "bad effect: " + err; return false; }
        b.effects.push_back(e);
    }
    for (const Value& v : p.get("world").items()) {
        nvworld::WorldEffect w;
        if (!nvworld::worldEffectFromJson(v.dump(), &w, &err)) { out = "bad world effect: " + err; return false; }
        b.world.push_back(w);
    }
    Value o = Value::object();
    o.set("applied", (int64_t)g_applier->applyBaseline(b));
    out = o.dump();
    return true;
}

bool MRefVars(const std::string&, std::string& out) {
    std::vector<fnvcoop::RefScriptVar> v;
    nvworld::pollRefVarsNow(&v);
    Value arr = Value::array();
    for (auto& x : v) {
        Value e = Value::object();
        e.set("ref", nvhooks::refToStr(x.ref)).set("var", (int64_t)x.index).set("value", x.value);
        arr.push(e);
    }
    Value o = Value::object();
    o.set("changes", arr);
    out = o.dump();
    return true;
}

bool MRefVar(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    fnvcoop::FormRef r;
    if (!nvhooks::strToRef(p.get("ref").asString(""), &r)) { out = "bad ref"; return false; }
    uint32_t id = nvhooks::resolver().fromRef(r);
    TESObjectREFR* ref = id ? static_cast<TESObjectREFR*>(nvhooks::resolver().lookup(id)) : nullptr;
    if (!ref) { out = "reference not loaded"; return false; }
    BSExtraData* xd = ref->extraDataList.GetByType(kExtraData_Script);
    ScriptEventList* el = xd ? static_cast<ExtraScript*>(xd)->eventList : nullptr;
    if (!el || !el->m_vars) { out = "no script variables"; return false; }
    int64_t want = p.get("index").asInt(-1);
    Value o = Value::object();
    for (auto it = el->m_vars->Begin(); !it.End(); ++it)
        if (it.Get() && it.Get()->id == want) { o.set("value", it.Get()->data); out = o.dump(); return true; }
    out = "variable index not found";
    return false;
}

bool MWatch(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    fnvcoop::FormRef r;
    if (!nvhooks::strToRef(p.get("ref").asString(""), &r)) { out = "bad ref"; return false; }
    nvworld::watchRef(r);
    out = "{}";
    return true;
}

bool IdFromParams(const Value& p, uint32_t* id, std::string& out) {
    if (p.get("id").isNumber()) {
        *id = static_cast<uint32_t>(p.get("id").asInt());
        return true;
    }
    std::string s = p.get("id").asString("");
    if (!s.empty()) {
        *id = static_cast<uint32_t>(std::strtoul(s.c_str(), nullptr, 16));
        return *id != 0;
    }
    fnvcoop::FormRef r;
    if (!nvhooks::strToRef(p.get("ref").asString(""), &r)) { out = "need id or ref"; return false; }
    *id = nvworld::ledger().map().local(r);  // created ref: the local object bound to that host ref
    if (!*id) *id = nvhooks::resolver().fromRef(r);
    if (!*id) out = "reference not resolvable here";
    return *id != 0;
}

bool TestHelpersOn() {
    const char* g = std::getenv("NVCOOP_TEST_HELPERS");
    return g && g[0] == '1';
}

// world.probe {id|ref} -> {exists,id,base,cell,flags,x,y,z,nvars}
bool MProbe(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    uint32_t id = 0;
    if (!IdFromParams(p, &id, out)) return false;
    nvworld::RefProbe pr;
    nvworld::probeRef(id, &pr);
    Value o = Value::object();
    o.set("exists", pr.exists).set("id", (int64_t)pr.refID).set("base", (int64_t)pr.baseID).set("cell", (int64_t)pr.cellID)
        .set("flags", (int64_t)pr.flags).set("x", (double)pr.x).set("y", (double)pr.y).set("z", (double)pr.z).set("nvars", (int64_t)pr.nvars);
    out = o.dump();
    return true;
}

// world.scripted {max} -> {refs:[{id,ref,base,nvars}]}  scripted references in the player's cell
bool MScripted(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    Value arr = Value::array();
    for (auto& s : nvworld::scriptedRefsInPlayerCell(static_cast<size_t>(p.get("max").asInt(50)))) {
        Value e = Value::object();
        e.set("id", (int64_t)s.refID).set("ref", s.ref).set("base", (int64_t)s.baseID).set("nvars", (int64_t)s.nvars);
        arr.push(e);
    }
    Value o = Value::object();
    o.set("refs", arr).set("cell", (int64_t)nvworld::playerCellID());
    out = o.dump();
    return true;
}

// world.getvar {id|ref, index} -> {value};  world.setvar (NVCOOP_TEST_HELPERS=1) writes the variable the way `set ref.var to` does
bool MGetVar(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    uint32_t id = 0;
    if (!IdFromParams(p, &id, out)) return false;
    double v = 0;
    if (!nvworld::getRefVar(id, static_cast<uint16_t>(p.get("index").asInt()), &v)) { out = "variable not found"; return false; }
    Value o = Value::object();
    o.set("value", v);
    out = o.dump();
    return true;
}
bool MSetVar(const std::string& params, std::string& out) {
    if (!TestHelpersOn()) { out = "test helpers disabled (start the game with NVCOOP_TEST_HELPERS=1)"; return false; }
    Value p;
    if (!Parse(params, p, out)) return false;
    uint32_t id = 0;
    if (!IdFromParams(p, &id, out)) return false;
    if (!nvworld::setRefVarDirect(id, static_cast<uint16_t>(p.get("index").asInt()), p.get("value").asNumber())) { out = "variable not found"; return false; }
    out = "{}";
    return true;
}

// world.execid {id|ref, cmd, scope?, noref?} -> {ok}: console command with that reference as calling reference (scope: inside a
// capture scope, cause Console, like exec.ref but it also reaches created refs through the spawn ledger / host ids)
bool MExecId(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    uint32_t id = 0;
    if (!p.get("noref").asBool(false) && !IdFromParams(p, &id, out)) return false;
    bool ok;
    if (p.get("scope").asBool(false) && nvhooks::hub()) {
        fnvcoop::Cause c;
        c.kind = fnvcoop::CauseKind::Console;
        nvhooks::ScopedCapture sc(*nvhooks::hub(), c);
        ok = nvhooks::execConsoleLineOn(p.get("cmd").asString(""), id);
    } else {
        ok = nvhooks::execConsoleLineOn(p.get("cmd").asString(""), id);
    }
    Value o = Value::object();
    o.set("ok", ok);
    out = o.dump();
    return true;
}

// world.query {id|ref, fn, p1?} -> {value}: engine condition function on a reference (GetRestrained, IsEssential, GetActorValue, GetFactionRelation)
bool MQuery(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    uint32_t id = 0;
    if (!IdFromParams(p, &id, out)) return false;
    uint32_t p1 = 0;
    if (p.get("p1").isNumber()) p1 = static_cast<uint32_t>(p.get("p1").asInt());
    else p1 = static_cast<uint32_t>(std::strtoul(p.get("p1").asString("0").c_str(), nullptr, 16));
    double v = 0;
    if (!nvworld::queryCondition(p.get("fn").asString(""), id, p1, &v)) { out = "query failed (unknown fn, ref not loaded or evaluator drifted)"; return false; }
    Value o = Value::object();
    o.set("value", v);
    out = o.dump();
    return true;
}

// world.actors {max, type?} -> {actors:[{id,ref,base}]}  characters (type 0x3B), containers (0x1B) or doors (0x1C) in the player's cell
bool MActors(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    Value arr = Value::array();
    for (auto& a : nvworld::actorsInPlayerCell(static_cast<size_t>(p.get("max").asInt(30)), static_cast<uint32_t>(p.get("type").asInt(0x3B)))) {
        Value e = Value::object();
        e.set("id", (int64_t)a.refID).set("ref", a.ref).set("base", (int64_t)a.baseID);
        arr.push(e);
    }
    Value o = Value::object();
    o.set("actors", arr);
    out = o.dump();
    return true;
}

// world.weather -> {weather:"plugin:id"|null, timeScale}
bool MWeather(const std::string&, std::string& out) {
    fnvcoop::FormRef w;
    Value o = Value::object();
    if (nvworld::currentWeather(&w)) o.set("weather", nvhooks::refToStr(w));
    else o.set("weather", nullptr);
    o.set("timeScale", (double)nvworld::currentTimeScale());
    out = o.dump();
    return true;
}

bool MReset(const std::string&, std::string& out) {
    nvworld::resetSession();
    out = "{}";
    return true;
}

bool MTick(const std::string&, std::string& out) {
    for (int i = 0; i < 15; ++i) nvworld::hostTick();
    out = "{}";
    return true;
}

void OnLoad(void*) {
    nvcoop::RegisterMethod("world.status", MStatus, true);
    nvcoop::RegisterMethod("world.drain", MDrain, true);
    nvcoop::RegisterMethod("world.apply", MApply, true);
    nvcoop::RegisterMethod("world.flush", MFlush, true);
    nvcoop::RegisterMethod("world.baseline", MBaseline, true);
    nvcoop::RegisterMethod("world.applybaseline", MApplyBaseline, true);
    nvcoop::RegisterMethod("world.refvars", MRefVars, true);
    nvcoop::RegisterMethod("world.refvar", MRefVar, true);
    nvcoop::RegisterMethod("world.watch", MWatch, true);
    nvcoop::RegisterMethod("world.tick", MTick, true);
    nvcoop::RegisterMethod("world.reset", MReset, true);
    nvcoop::RegisterMethod("world.probe", MProbe, true);
    nvcoop::RegisterMethod("world.scripted", MScripted, true);
    nvcoop::RegisterMethod("world.getvar", MGetVar, true);
    nvcoop::RegisterMethod("world.setvar", MSetVar, true);
    nvcoop::RegisterMethod("world.execid", MExecId, true);
    nvcoop::RegisterMethod("world.weather", MWeather, true);
    nvcoop::RegisterMethod("world.query", MQuery, true);
    nvcoop::RegisterMethod("world.actors", MActors, true);
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console)
        nvworld::Shutdown();
    else if (type == NVSEMessagingInterface::kMessage_PostLoadGame || type == NVSEMessagingInterface::kMessage_NewGame ||
             type == NVSEMessagingInterface::kMessage_ExitToMainMenu)
        nvworld::resetSession();
    else if (type == NVSEMessagingInterface::kMessage_OnCellRefsLoaded && g_applier)
        g_applier->flushQueued();  // spawns queued for a cell that just finished loading
}

}  // namespace

void L12OnFrame() {
    if (!g_inited) TryInit();
    if (!g_inited) return;
    ++g_frame;
    if (g_role == nvhooks::Role::Host) nvworld::hostTick();
    else if (g_applier && g_frame % 30 == 0) g_applier->tick(g_frame % 120 == 0);
}

NVCOOP_MODULE(l12_world, "l12_world", OnLoad, OnMessage, L12OnFrame);

// Hot reload (T3b): see hooks/hooks_module.cpp. TryInit needs L5's hub, which the l5_hooks participant (order 10) restored first.
NVCOOP_RELOAD_STATE(l12_world, "l12_world", 1, 30,
    /*canUnload*/ [] {
        int n = nvworld::auditTamperedHooks();
        return n ? std::to_string(n) + " world hook(s) were chained over by another mod" : std::string();
    },
    /*save*/ [](nvreload::Writer& w) {
        w.boolean(g_inited);
        return true;
    },
    /*restore*/ [](uint32_t, nvreload::Reader& r) {
        if (r.boolean()) TryInit();
        return true;
    },
    /*migrate*/ nullptr,
    /*shutdown*/ [] { if (g_inited) nvworld::Shutdown(); },
    /*fresh*/ nullptr);
