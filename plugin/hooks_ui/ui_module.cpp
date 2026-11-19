// nvcoop module for hooks_ui (L11). Installs the UI hooks at DeferredInit and exposes bridge methods.
//   nvcoop.ini  [coop] role=host|client|guest   (same key as L5; host default)
//               [ui]   selftest=1               loopback Bridge: menus are recorded, answered with ui.reply (in-game proofs, no network)
//   ui.status                          -> {role, installed, failed, hooks:[{name,status}], counters, router stats}
//   ui.origin {slot, on}               -> push/pop a persistent Acting scope (a stand-in for L10's executor window) -- selftest only
//   ui.menus                           -> selftest: menus raised so far [{id, origin, kind, text, buttons}]
//   ui.reply {menu, choice}            -> selftest: the origin answered (calls UiRouter::onMenuResume)
//   ui.watch {quest}                   -> selftest: watch a quest for condition views;  ui.view {quest, idx, slot, value} -> override
//   ui.forced                          -> selftest: forced dialogue captured so far;  ui.slides -> captured SwapTexture list
// L10 wiring (production): nvui::router()->bridge() = {...}; DialogueNode::setMenuResume -> nvui::router()->onMenuResume;
// L8 EngineGlue stubs call nvui::ops::{openMenu, closeMenu, forceCloseDialog, openDialogMenu}; ops::setAnswerSink -> onMenuAnswered.
#include <Windows.h>

#include <map>
#include <string>
#include <tuple>

#include "PluginAPI.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "effect_json.h"
#include "engine_hooks.h"
#include "ui_hooks.h"
#include "reload/reload_state.h"

using nvcoop::json::Value;

namespace {

nvui::UiRouter* g_router = nullptr;
nvui::InitReport g_report;
bool g_host = true, g_selftest = false, g_inited = false;

struct TestMenu {
    uint32_t id;
    nvui::PlayerId origin;
    int kind;
    std::string text;
    std::vector<std::string> buttons;
};
std::vector<TestMenu> g_menus;
uint32_t g_nextMenu = 0;
std::vector<fnvcoop::FormRef> g_watch;
std::map<std::tuple<std::string, uint32_t, uint32_t, int>, int32_t> g_views;
Value g_forcedLog = Value::array(), g_slideLog = Value::array();
nvui::PlayerId g_testActor = 1;

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

Value RefJson(const fnvcoop::FormRef& r) { return Value(r.plugin + ":" + [&] { char b[16]; snprintf(b, sizeof b, "%X", r.localId); return std::string(b); }()); }

void InstallSelftestBridge(nvui::Bridge& b) {
    b.slotOfActor = [](uint32_t ref) { return ref == nvui::kPlayerRefID ? nvui::kHost : nvui::kNoSlot; };
    b.raiseMenu = [](nvui::PlayerId origin, nvui::MenuKind kind, const std::string& text, std::vector<std::string> buttons, int32_t, const fnvcoop::FormRef&) {
        g_menus.push_back({++g_nextMenu, origin, int(kind), text, std::move(buttons)});
        return g_nextMenu;
    };
    b.watchesQuest = [](uint32_t ref) {
        for (auto& q : g_watch)
            if (nvhooks::resolver().fromRef(q) == ref) return true;
        return false;
    };
    b.viewVar = [](const fnvcoop::FormRef& q, uint32_t idx, nvui::PlayerId slot, int32_t real) {
        auto it = g_views.find({q.plugin, q.localId, idx, int(slot)});
        return it == g_views.end() ? real : it->second;
    };
    b.forcedSay = [](const nvui::ForcedSayArgs& a) {
        Value o = Value::object();
        o.set("kind", "say").set("npc", RefJson(a.npc)).set("topic", RefJson(a.topic)).set("info", RefJson(a.info)).set("target", int(a.target))
            .set("scene", a.scene).set("lines", int(a.lines.size()));
        g_forcedLog.push(o);
    };
    b.forceConversation = [](const fnvcoop::FormRef& npc, const fnvcoop::FormRef& topic, nvui::PlayerId target) {
        Value o = Value::object();
        o.set("kind", "conversation").set("npc", RefJson(npc)).set("topic", RefJson(topic)).set("target", int(target));
        g_forcedLog.push(o);
        return g_testActor;
    };
    b.slideSwap = [](const fnvcoop::FormRef& ref, const std::string& o, const std::string& n) {
        Value v = Value::object();
        v.set("ref", RefJson(ref)).set("old", o).set("new", n);
        g_slideLog.push(v);
    };
}

void InitUi() {
    if (g_inited) return;
    g_inited = true;
    // Test copies only: NVCOOP_NO_UI_HOOKS=1 installs none of these hooks (A/B: is a dialogue bug ours or the game's?).
    char off[8] = {};
    if (GetEnvironmentVariableA("NVCOOP_NO_UI_HOOKS", off, sizeof off) && off[0] == '1') { nvcoop::Log("hooks_ui: NOT installed (NVCOOP_NO_UI_HOOKS=1)"); return; }
    char role[32] = {};
    GetPrivateProfileStringA("coop", "role", "host", role, sizeof role, IniPath().c_str());
    // the NV:MP launcher (TTW-CoOp.cmd) passes the role: it wins over the ini, like in coop_module.cpp LoadConfig
    char envRole[32] = {};
    const DWORD envN = GetEnvironmentVariableA("NVCOOP_ROLE", envRole, sizeof envRole);
    if (envN > 0 && envN < sizeof envRole) std::snprintf(role, sizeof role, "%s", std::string(envRole) == "join" ? "client" : envRole);
    std::string r = role;
    g_host = !(r == "client" || r == "guest");
    g_selftest = GetPrivateProfileIntA("ui", "selftest", 0, IniPath().c_str()) != 0;
    nvui::Bridge b;
    if (g_selftest) InstallSelftestBridge(b);
    g_router = nvui::MakeRouter(std::move(b));
    g_report = nvui::Init(*g_router, g_host);
    nvcoop::Log("hooks_ui: role=%s selftest=%d installed=%d failed=%d", g_host ? "host" : "guest", g_selftest ? 1 : 0, g_report.ok, g_report.failed);
    for (auto& h : g_report.hooks) nvcoop::Log("hooks_ui:   %-24s %s", h.name.c_str(), nvhooks::statusName(h.status));
}

bool Parse(const std::string& params, Value& p, std::string& out) {
    if (params.empty() || params == "null") { p = Value::object(); return true; }
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    return true;
}

bool MStatus(const std::string&, std::string& out) {
    Value o = Value::object(), hs = Value::array();
    for (auto& h : g_report.hooks) {
        Value e = Value::object();
        e.set("name", h.name).set("status", nvhooks::statusName(h.status));
        hs.push(e);
    }
    const auto& c = nvui::counters();
    Value cn = Value::object();
    cn.set("showMessage", (int64_t)c.seenShowMessage).set("boxes", (int64_t)c.seenBoxes).set("service", (int64_t)c.seenService)
        .set("cond", (int64_t)c.seenCond).set("forced", (int64_t)c.seenForced).set("swap", (int64_t)c.seenSwap)
        .set("engineBoxes", (int64_t)c.seenEngineBoxes);
    Value st = Value::object();
    if (g_router) {
        const auto& s = g_router->stats();
        st.set("boxesRouted", (int64_t)s.boxesRouted).set("boxesLocal", (int64_t)s.boxesLocal).set("resumes", (int64_t)s.resumes)
            .set("dupResumes", (int64_t)s.dupResumes).set("serviceRouted", (int64_t)s.serviceRouted).set("serviceLocal", (int64_t)s.serviceLocal)
            .set("condViews", (int64_t)s.condViews).set("condChanged", (int64_t)s.condChanged).set("forced", (int64_t)s.forced)
            .set("vetoes", (int64_t)s.vetoes).set("slides", (int64_t)s.slides).set("pendingBoxes", (int64_t)g_router->pendingBoxes());
    }
    Value rb = Value::array();
    for (auto& b : nvui::recentBoxes()) {
        char sc[16], th[16];
        std::snprintf(sc, sizeof sc, "%08X", b.scriptRefID);
        std::snprintf(th, sizeof th, "%08X", b.thisObj);
        Value e = Value::object();
        e.set("seq", (int64_t)b.seq).set("script", std::string(sc)).set("thisObj", std::string(th))
            .set("message", b.message.plugin.empty() ? std::string() : b.message.plugin + ":" + std::to_string(b.message.localId)).set("text", b.text);
        rb.push(e);
    }
    o.set("role", g_host ? "host" : "guest").set("inited", g_inited).set("selftest", g_selftest).set("installed", g_report.ok)
        .set("failed", g_report.failed).set("hooks", hs).set("counters", cn).set("router", st).set("recentBoxes", rb);
    out = o.dump();
    return true;
}

nvui::OriginScope* g_testScope = nullptr;
std::vector<nvui::OriginScope*> g_actingScopes;   // beginActingFor: what the host runs for a guest right now (innermost last)
bool MOrigin(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    if (!g_selftest || !g_router) { out = "selftest not enabled"; return false; }
    delete g_testScope;
    g_testScope = nullptr;
    if (p.get("on").asBool(true)) g_testScope = new nvui::OriginScope(g_router->origin(), nvui::OriginContext::Kind::Acting, nvui::PlayerId(p.get("slot").asInt(1)));
    out = "{\"on\":" + std::string(g_testScope ? "true" : "false") + "}";
    return true;
}

bool MMenus(const std::string&, std::string& out) {
    Value arr = Value::array();
    for (auto& m : g_menus) {
        Value e = Value::object(), bs = Value::array();
        for (auto& b : m.buttons) bs.push(b);
        e.set("id", (int64_t)m.id).set("origin", int(m.origin)).set("kind", m.kind).set("text", m.text).set("buttons", bs);
        arr.push(e);
    }
    out = arr.dump();
    return true;
}

bool MReply(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    if (!g_router) { out = "no router"; return false; }
    g_router->onMenuResume(uint32_t(p.get("menu").asInt(0)), int32_t(p.get("choice").asInt(-1)), false);
    out = "{}";
    return true;
}

bool MWatch(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    fnvcoop::FormRef r;
    if (!nvhooks::strToRef(p.get("quest").asString(""), &r)) { out = "bad quest"; return false; }
    g_watch.push_back(r);
    out = "{}";
    return true;
}

bool MView(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    fnvcoop::FormRef r;
    if (!nvhooks::strToRef(p.get("quest").asString(""), &r)) { out = "bad quest"; return false; }
    g_views[{r.plugin, r.localId, uint32_t(p.get("idx").asInt(0)), int(p.get("slot").asInt(0))}] = int32_t(p.get("value").asInt(0));
    out = "{}";
    return true;
}

// ---- selftest-only live probes --------------------------------------------------------------------------------------------
// ui.evalvar {quest, idx}: call the engine's condition function (0x59E1A0, hooked) for a quest variable, like a dialogue condition would.
bool MEvalVar(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    fnvcoop::FormRef r;
    if (!nvhooks::strToRef(p.get("quest").asString(""), &r)) { out = "bad quest"; return false; }
    void* q = nvhooks::resolver().lookup(nvhooks::resolver().fromRef(r));
    if (!q) { out = "quest not found"; return false; }
    double v = -1;
    bool ok = reinterpret_cast<bool(__cdecl*)(void*, void*, uint32_t, double*)>(0x59E1A0)(nullptr, q, uint32_t(p.get("idx").asInt(0)), &v);
    Value o = Value::object();
    o.set("ok", ok).set("value", v);
    out = o.dump();
    return true;
}
bool MActor(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    g_testActor = nvui::PlayerId(p.get("slot").asInt(1));
    out = "{}";
    return true;
}
bool MDialogOpen(const std::string&, std::string& out) {
    out = nvui::ops::dialogOpen() ? "{\"open\":true}" : "{\"open\":false}";
    return true;
}
// ui.sayinfo {npc, topic, info}: the NPC says exactly that INFO on this machine (test of the subtitle/voice path).
bool MSayInfo(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    fnvcoop::FormRef npc, topic, info;
    if (!nvhooks::strToRef(p.get("npc").asString(""), &npc) || !nvhooks::strToRef(p.get("topic").asString(""), &topic) ||
        !nvhooks::strToRef(p.get("info").asString(""), &info)) { out = "npc, topic and info are form refs"; return false; }
    const bool ok = nvui::ops::sayInfo(nvhooks::resolver().fromRef(npc), topic, info);
    out = ok ? "{\"said\":true}" : "{\"said\":false}";
    return true;
}
bool MCloseDialog(const std::string&, std::string& out) {
    bool r = nvui::ops::forceCloseDialog();
    out = r ? "{\"closed\":true}" : "{\"closed\":false}";
    return true;
}
// ui.open {text, buttons:[..], id?}: origin-side ops::openMenu(MessageBox) on THIS machine; ui.closemenu {menu} forces it shut.
bool MOpen(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    fnvcoop_dlg::MenuRequest rq;
    rq.menu = uint32_t(p.get("id").asInt(900));
    rq.kind = nvui::MenuKind::MessageBox;
    rq.text = p.get("text").asString("test box");
    for (auto& b : p.get("buttons").items()) rq.buttons.push_back(b.asString(""));
    bool ok = nvui::ops::openMenu(rq);
    Value o = Value::object();
    o.set("ok", ok).set("queued", (int64_t)nvui::ops::queuedBoxes());
    out = o.dump();
    return true;
}
bool MCloseMenu(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    nvui::ops::closeMenu(uint32_t(p.get("menu").asInt(900)));
    Value o = Value::object();
    o.set("queued", (int64_t)nvui::ops::queuedBoxes());
    out = o.dump();
    return true;
}

// ui.swap {ref, old, new}: client-side slideshow playback (ops::swapTexture)
bool MSwap(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    fnvcoop::FormRef r;
    if (!nvhooks::strToRef(p.get("ref").asString(""), &r)) { out = "bad ref"; return false; }
    bool ok = nvui::ops::swapTexture(r, p.get("old").asString(""), p.get("new").asString(""));
    out = ok ? "{\"ok\":true}" : "{\"ok\":false}";
    return true;
}

// ui.answer {button} -> {menu}: answer the message box our ops layer is showing (bot.menu.press / bot.consent use this)
bool MAnswer(const std::string& params, std::string& out) {
    Value p;
    if (!Parse(params, p, out)) return false;
    uint32_t menu = 0;
    if (!nvui::ops::answerShowing(int(p.get("button").asInt(-1)), &menu)) { out = "no message box is being shown"; return false; }
    Value o = Value::object();
    o.set("menu", (int64_t)menu);
    out = o.dump();
    return true;
}

bool MForced(const std::string&, std::string& out) { out = g_forcedLog.dump(); return true; }
bool MSlides(const std::string&, std::string& out) { out = g_slideLog.dump(); return true; }

void OnLoad(void*) {
    nvcoop::RegisterMethod("ui.status", MStatus, true);
    nvcoop::RegisterMethod("ui.origin", MOrigin, true);
    nvcoop::RegisterMethod("ui.menus", MMenus, true);
    nvcoop::RegisterMethod("ui.reply", MReply, true);
    nvcoop::RegisterMethod("ui.watch", MWatch, true);
    nvcoop::RegisterMethod("ui.view", MView, true);
    nvcoop::RegisterMethod("ui.forced", MForced, true);
    nvcoop::RegisterMethod("ui.evalvar", MEvalVar, true);
    nvcoop::RegisterMethod("ui.swap", MSwap, true);
    nvcoop::RegisterMethod("ui.actor", MActor, true);
    nvcoop::RegisterMethod("ui.dialogopen", MDialogOpen, true);
    nvcoop::RegisterMethod("ui.closedialog", MCloseDialog, true);
    nvcoop::RegisterMethod("ui.sayinfo", MSayInfo, true);
    nvcoop::RegisterMethod("ui.open", MOpen, true);
    nvcoop::RegisterMethod("ui.closemenu", MCloseMenu, true);
    nvcoop::RegisterMethod("ui.slides", MSlides, true);
    nvcoop::RegisterMethod("ui.answer", MAnswer, true);
}
void OnMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_DeferredInit) InitUi();
    else if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) nvui::Shutdown();
}
void OnFrame() {
    if (g_inited) nvui::ops::pump();
}

}  // namespace

namespace nvui {
void beginActingFor(uint8_t slot) {
    g_actingScopes.push_back(g_router ? new OriginScope(g_router->origin(), OriginContext::Kind::Acting, PlayerId(slot)) : nullptr);
}
void endActingFor() {
    if (g_actingScopes.empty()) return;
    delete g_actingScopes.back();
    g_actingScopes.pop_back();
}
}  // namespace nvui

NVCOOP_MODULE(l11_hooks_ui, "l11_hooks_ui", OnLoad, OnMessage, OnFrame);

// Hot reload (T3b): see hooks/hooks_module.cpp. The UI hooks are reinstalled by InitUi in the new core.
NVCOOP_RELOAD_STATE(l11_hooks_ui, "l11_hooks_ui", 1, 20,
    /*canUnload*/ [] {
        int n = nvui::auditTamperedHooks();
        return n ? std::to_string(n) + " UI hook(s) were chained over by another mod" : std::string();
    },
    /*save*/ [](nvreload::Writer& w) {
        w.boolean(g_inited);
        return true;
    },
    /*restore*/ [](uint32_t, nvreload::Reader& r) {
        if (r.boolean()) InitUi();
        return true;
    },
    /*migrate*/ nullptr,
    /*shutdown*/ [] { if (g_inited) nvui::Shutdown(); },
    /*fresh*/ nullptr);
