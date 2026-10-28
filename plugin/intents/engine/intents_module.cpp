// nvcoop module for guest intents + SaveSuppress enforcement (x86, compiled into nvcoop.dll next to the coop module). NOTHING here has run in a game.
//
//   nvcoop.ini
//     [intents]  enable=0 (default OFF) activate trigger pickup terminal container hit dialogue standin standin_unsafe require_puppet check_reach
//                save_gate reach_* rate_* lock_lease_ms trigger_cooldown_ms pending_timeout_ms coalesce_ms redirect_file    (docs/tools/guest-intents.md)
//     Environment NVCOOP_INTENTS_<KEY> beats the ini (automated runs).
//
//   host   : the IntentService (frame) registers channel 0x8D and takes over the story-channel action executor, so every guest interaction is
//            run as the guest's puppet (plugin/coop/live's avatar map, else NV:MP's ghost, else the policy-limited stand-in).
//   guest  : once joined, the activate / script-event observers (L5) feed the IntentClient; a hook on TerminalEntryRun forwards terminal picks.
//   both   : a hook on the engine's CanSaveNow asks the SaveGate, which enforces the shared SaveSuppress flag.
//   RPCs   : intents.status, intents.submit (guest), intents.host.run (host, for the single-instance proof), intents.pins.
#include <Windows.h>

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <memory>
#include <string>

#include "GameData.h"
#include "PluginAPI.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "coop_runtime.h"
#include "effect_json.h"
#include "intents/engine/game_engine.h"
#include "intents/engine/intents_saves.h"
#include "intents/engine/intents_addrs.h"
#include "engine_hooks.h"
#include "hook_patch.h"
#include "intents/container_redirect.h"
#include "intents/intent_service.h"
#include "intents/storage_book.h"
#include "reload/reload_state.h"
#ifdef NVCOOP_WITH_COMPANIONS
#include "companions/companions_glue.h"   // glueAllowActivation / glueGuestInteraction
#endif
#ifdef NVCOOP_WITH_LIVE
#include "live/live_glue.h"   // glueAvatarRef: the puppet of a remote player
#endif
#ifdef NVCOOP_WITH_UI
#include "guest_dialogue.h"   // fndlg::GuestDialogue: the guest half of a conversation (G05); the talk veto
#endif
#ifdef NVCOOP_WITH_NVMP
#include "nvmp/ghost_ref.h"
#endif

namespace {

using namespace nvcoop::intents;

struct Win32Memory : nvhooks::MemoryOps {
    bool read(nvhooks::Addr a, void* out, size_t n) override {
        __try {
            std::memcpy(out, reinterpret_cast<const void*>(a), n);
            return true;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
    }
    bool write(nvhooks::Addr a, const void* in, size_t n) override {
        DWORD old = 0;
        void* p = reinterpret_cast<void*>(a);
        if (!VirtualProtect(p, n, PAGE_EXECUTE_READWRITE, &old)) return false;
        std::memcpy(p, in, n);
        FlushInstructionCache(GetCurrentProcess(), p, n);
        DWORD tmp;
        VirtualProtect(p, n, old, &tmp);
        return true;
    }
    nvhooks::Addr allocExec(size_t n) override {
        return reinterpret_cast<nvhooks::Addr>(VirtualAlloc(nullptr, n, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE));
    }
    void freeExec(nvhooks::Addr a, size_t) override { VirtualFree(reinterpret_cast<void*>(a), 0, MEM_RELEASE); }
};

Win32Memory g_mem;
std::unique_ptr<nvhooks::HookRegistry> g_reg;
std::unique_ptr<IntentService> g_svc;
nvcoop::intents::RetryList g_retry;   // the activators the stand-in retry is allowed for (nvcoop_intents.json retryScripts); empty = no retry
std::unique_ptr<SaveGate> g_gate;
Config g_cfg;
bool g_inited = false, g_canSaveHooked = false, g_termHooked = false, g_observersOn = false;
bool g_talkDeferred = false;   // the activate observer's last decision: GuestDialogue took the talk (vanilla activation is deferred until the host grants it)
uint64_t g_observedSessions = 0;
std::string g_note;   // why the module is idle, for intents.status

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    return std::string(buf);
}
std::string DllDir() {
    std::string p = IniPath();
    size_t s = p.find_last_of("\\/");
    return s == std::string::npos ? std::string() : p.substr(0, s + 1);
}
std::string ReadWholeFile(const std::string& path) {
    HANDLE h = CreateFileA(path.c_str(), GENERIC_READ, FILE_SHARE_READ | FILE_SHARE_WRITE, nullptr, OPEN_EXISTING, 0, nullptr);
    if (h == INVALID_HANDLE_VALUE) return {};
    std::string s;
    char buf[4096];
    DWORD n = 0;
    while (ReadFile(h, buf, sizeof buf, &n, nullptr) && n) s.append(buf, n);
    CloseHandle(h);
    return s;
}
bool WriteWholeFile(const std::string& path, const std::string& text) {
    HANDLE h = CreateFileA(path.c_str(), GENERIC_WRITE, FILE_SHARE_READ, nullptr, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, nullptr);
    if (h == INVALID_HANDLE_VALUE) return false;
    DWORD n = 0;
    bool ok = WriteFile(h, text.data(), static_cast<DWORD>(text.size()), &n, nullptr) && n == text.size();
    CloseHandle(h);
    return ok;
}
uint64_t NowMs() { return GetTickCount64(); }
constexpr const char* kStashStateFile = "nvcoop_intents_state.json";   // the host's gear ledger (storage_book.h), next to the dll
uint64_t g_lastStateWriteMs = 0;
void SaveStashState(bool force) {
    if (!g_svc) return;
    StorageBook& b = g_svc->stash();
    if (!b.dirty()) return;
    const uint64_t t = NowMs();
    if (!force && t - g_lastStateWriteMs < 2000) return;
    g_lastStateWriteMs = t;
    if (WriteWholeFile(DllDir() + kStashStateFile, b.stateJson())) b.markClean();
    else nvcoop::Log("intents: could not write %s (gear stashes are kept in memory only)", kStashStateFile);
}

// ---- ini -----------------------------------------------------------------------------------------------------------------------------

std::string ReadIni(const char* key, const char* def) {
    std::string env = "NVCOOP_INTENTS_";
    for (const char* c = key; *c; ++c) env += static_cast<char>(*c >= 'a' && *c <= 'z' ? *c - 32 : *c);
    if (const char* v = std::getenv(env.c_str())) return v;
    char buf[512] = {};
    GetPrivateProfileStringA("intents", key, def, buf, sizeof buf, (DllDir() + "nvcoop.ini").c_str());
    return buf;
}

// ---- who acts for a guest ------------------------------------------------------------------------------------------------------------

RefId PuppetOf(PlayerId slot) {
#ifdef NVCOOP_WITH_LIVE
    return nvcoop::livewire::glueAvatarRef(static_cast<uint8_t>(slot));
#else
    (void)slot;
    return 0;
#endif
}
// Why a guest has no puppet, for the stand-in log line.
const char* AvatarsNote() {
#ifdef NVCOOP_WITH_LIVE
    return nvcoop::livewire::glueSwitches().avatars ? "[live] avatars=1 but this slot's puppet does not exist yet (not spawned, other cell or session not synced)"
                                                     : "[live] avatars=0, so there are no puppets";
#else
    return "this build has no avatar layer";
#endif
}
RefId GhostOf(PlayerId slot) {
#ifdef NVCOOP_WITH_NVMP
    return slot == fnvcoop::kHostPlayer ? 0u : nvmp::GhostRefOfPlayer(static_cast<uint8_t>(slot));
#else
    (void)slot;
    return 0;
#endif
}

// ---- detours -------------------------------------------------------------------------------------------------------------------------

using Fn_CanSave = bool(__thiscall*)(void*, bool);
Fn_CanSave o_CanSave = nullptr;
bool __fastcall h_CanSave(void* mgr, void*, bool ignoreMenus) {
    bool allow = o_CanSave(mgr, ignoreMenus);
    return g_gate ? g_gate->allow(allow) : allow;
}

using Fn_TermRun = void(__thiscall*)(void*, void*);
Fn_TermRun o_TermRun = nullptr;
void __fastcall h_TermRun(void* entry, void*, void* terminalRef) {
    // Guest only (installed on a client session): the pick is forwarded, the local call goes on (menus, the entry text, UI-only script parts;
    // the guest gate already drops the script's shared-state writes).
    if (g_svc && g_svc->client()) {
        uint32_t rid = nvcoop::intents::game::refIdOf(terminalRef);
        fnvcoop::FormRef tr;
        std::vector<uint8_t> path;
        if (rid && nvhooks::resolver().toRef(rid, &tr) && nvcoop::intents::game::terminalPathOf(rid, entry, &path))
            g_svc->submit(Kind::TerminalUse, tr, nullptr, 0, std::move(path));
    }
    o_TermRun(entry, terminalRef);
}

bool HookOne(const nvhooks::HookSpec& spec, void* detour, void** orig, const char* what) {
    if (!g_reg) g_reg = std::make_unique<nvhooks::HookRegistry>(g_mem);
    nvhooks::Addr tramp = 0;
    nvhooks::PatchStatus st = g_reg->acquire(spec, reinterpret_cast<nvhooks::Addr>(detour), &tramp, reinterpret_cast<nvhooks::Addr*>(orig));
    if (!nvhooks::installed(st)) {
        nvcoop::Log("intents: %s hook NOT installed (%s); that path stays off", what, nvhooks::statusName(st));
        return false;
    }
    *orig = reinterpret_cast<void*>(tramp);
    nvcoop::Log("intents: %s hook installed (%s)", what, nvhooks::statusName(st));
    return true;
}

// ---- guest observers -----------------------------------------------------------------------------------------------------------------

void InstallObservers() {
#ifdef NVCOOP_WITH_UI
    fnvcoop_dlg::traceSink() = [](const std::string& s) { nvcoop::Log("%s", s.c_str()); };   // the conversation start steps (request, grant, refusal, menu event), one line each
#endif
    nvhooks::setActivateObserver([](const fnvcoop::FormRef& t) {
        g_talkDeferred = false;
        const bool suppress = g_svc && g_svc->onActivate(t) == Obs::Suppress;
        return suppress || g_talkDeferred ? nvhooks::ActivateAction::Suppress : nvhooks::ActivateAction::Proceed;
    });
    nvhooks::setEventObserver([](uint32_t mask, const fnvcoop::FormRef& t) {
        return g_svc && g_svc->onEvent(mask, t) == Obs::Suppress ? nvhooks::EventAction::Suppress : nvhooks::EventAction::Proceed;
    });
    // A script / console SetStage the guest suppression dropped becomes a request the host judges (Kind::SetStage). Without it the stage would just vanish.
    nvhooks::setStageRequestObserver([](const fnvcoop::FormRef& quest, uint32_t stage) { return g_svc && g_svc->onStageBlocked(quest, stage); });
    g_observersOn = true;
    nvcoop::Log("intents: guest observers installed (this replaces coop's forward_activate observer)");
}

// ---- bridge --------------------------------------------------------------------------------------------------------------------------

bool MStatus(const std::string&, std::string& out) {
    nvcoop::json::Value o = nvcoop::json::Value::object();
    o.set("enabled", g_cfg.enable).set("inited", g_inited).set("note", g_note).set("pinsOk", nvcoop::intents::game::pinsOk())
        .set("canSaveHook", g_canSaveHooked).set("terminalHook", g_termHooked).set("observers", g_observersOn);
    if (g_gate) o.set("saveGate", nvcoop::json::Value::object().set("asked", (int64_t)g_gate->stats().asked).set("blocked", (int64_t)g_gate->stats().blocked)
                                      .set("notices", (int64_t)g_gate->stats().notices));
    std::string base = o.dump();
    std::string svc = g_svc ? g_svc->statusJson() : "null";
    out = base.substr(0, base.size() - 1) + ",\"service\":" + svc + "}";
    return true;
}

Kind KindOfName(const std::string& k) {
    return k == "pickup" ? Kind::PickUp : k == "trigger" ? Kind::TriggerEnter : k == "triggerleave" ? Kind::TriggerLeave : k == "stay" ? Kind::TriggerStay
         : k == "combat" ? Kind::StartCombat : k == "hit" ? Kind::Hit : k == "terminal" ? Kind::TerminalUse : k == "container" ? Kind::ContainerOpen
         : k == "close" ? Kind::ContainerClose : k == "dialogue" ? Kind::DialogueStart : k == "stage" ? Kind::SetStage : Kind::Activate;
}

bool MSubmit(const std::string& params, std::string& out) {
    if (!g_svc || !g_svc->client()) { out = "intents client not running (not a joined guest, or [intents] enable=0)"; return false; }
    nvcoop::json::Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    fnvcoop::FormRef ref;
    if (!nvhooks::strToRef(p.get("ref").asString(""), &ref)) { out = "ref must be '<plugin>:<hex>'"; return false; }
    std::string k = p.get("kind").asString("activate");
    Kind kind = KindOfName(k);
    std::vector<uint8_t> path;
    for (const auto& v : p.get("path").items()) path.push_back(static_cast<uint8_t>(v.asInt(0)));
    uint32_t id = g_svc->submit(kind, ref, nullptr, static_cast<int32_t>(p.get("value").asInt(0)), std::move(path));
    out = std::string("{\"id\":") + std::to_string(id) + "}";
    return id != 0 ? true : (out = "not sent (not joined, still syncing, or too many in flight)", false);
}

// intents.host.run {slot, kind, ref, session?}: the host handles an intent as if joined player `slot` had sent it (single-instance proof with a
// fake client process: no second game needed). The id is taken from a counter, so every call is a new intent.
bool MHostRun(const std::string& params, std::string& out) {
    if (!g_svc || !g_svc->host()) { out = "intents host not running"; return false; }
    nvcoop::json::Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    Intent in;
    in.session = 0xFFFF0001u;
    static uint32_t counter = 0;
    in.id = ++counter;
    std::string k = p.get("kind").asString("activate");
    in.kind = KindOfName(k);
    if (!nvhooks::strToRef(p.get("ref").asString(""), &in.target)) { out = "ref must be '<plugin>:<hex>'"; return false; }
    for (const auto& v : p.get("path").items()) in.path.push_back(static_cast<uint8_t>(v.asInt(0)));
    in.value = static_cast<int32_t>(p.get("value").asInt(0));
    Verdict v = g_svc->host()->handle(static_cast<PlayerId>(p.get("slot").asInt(1)), in);
    nvcoop::json::Value o = nvcoop::json::Value::object();
    o.set("result", resultName(v.result)).set("holder", (int64_t)v.holder).set("redirect", v.hasRedirect ? nvhooks::refToStr(v.redirect) : std::string());
    out = o.dump();
    return true;
}

// intents.cansave {}: asks the engine's CanSaveNow (through our detour, so the SaveGate answers) without saving anything. For the proof.
bool CanSaveSEH(bool* out) {
    __try {
        void* mgr = *reinterpret_cast<void**>(nvcoop::intents::addr::SaveLoadMgrPtr);
        if (!mgr) return false;
        *out = reinterpret_cast<bool(__thiscall*)(void*, bool)>(nvcoop::intents::addr::CanSaveNow.target)(mgr, false);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool MCanSave(const std::string&, std::string& out) {
    bool allowed = false;
    if (!CanSaveSEH(&allowed)) { out = "CanSaveNow could not be called (no save manager yet?)"; return false; }
    out = std::string("{\"allowed\":") + (allowed ? "true" : "false") + "}";
    return true;
}

// ---- lifecycle -----------------------------------------------------------------------------------------------------------------------

void Init() {
    if (g_inited) return;
    // always available, so a bad ini or enable=0 can be diagnosed over the bridge
    nvcoop::RegisterMethod("intents.status", MStatus, true);
    nvcoop::RegisterMethod("intents.submit", MSubmit, true);
    nvcoop::RegisterMethod("intents.host.run", MHostRun, true);
    nvcoop::RegisterMethod("intents.cansave", MCanSave, true);
    std::string err;
    Config c;
    if (!readConfig(&ReadIni, &c, &err)) {
        g_note = "bad nvcoop.ini: " + err;
        nvcoop::Log("intents: %s -> staying off", g_note.c_str());
        return;
    }
    g_cfg = c;
    if (!g_cfg.enable) { g_note = "[intents] enable=0"; return; }
    g_inited = true;
    nvcoop::intents::game::pinsOk();
    ContainerRedirect redirect;
    StorageBook stash;
    if (!g_cfg.redirectFile.empty()) {
        std::string text = ReadWholeFile(DllDir() + g_cfg.redirectFile);
        if (text.empty()) nvcoop::Log("intents: data file %s not found: shared storage containers stay shared", g_cfg.redirectFile.c_str());
        else {
            if (!redirect.load(text, &err)) nvcoop::Log("intents: data file %s (redirects): %s", g_cfg.redirectFile.c_str(), err.c_str());
            else nvcoop::Log("intents: %u shared storage containers are redirected per player", (unsigned)redirect.size());
            if (!g_retry.load(text, &err)) { nvcoop::Log("intents: data file %s (retryScripts): %s: no activation is redone by the stand-in", g_cfg.redirectFile.c_str(), err.c_str()); g_retry.clear(); }
            else nvcoop::Log("intents: %u activator scripts may be redone by the host's player after a puppet's activation did nothing", (unsigned)g_retry.size());
            if (!stash.load(text, &err)) nvcoop::Log("intents: data file %s (stashes): %s", g_cfg.redirectFile.c_str(), err.c_str());
            else if (g_cfg.storage) {
                nvcoop::Log("intents: %u shared gear containers keep a stash per player", (unsigned)stash.entries().size());
                std::string state = ReadWholeFile(DllDir() + kStashStateFile);
                if (!state.empty() && !stash.loadState(state, &err)) nvcoop::Log("intents: %s: %s (starting with empty stashes)", kStashStateFile, err.c_str());
            }
        }
    }
    ServiceDeps d;
    d.eng = &nvcoop::intents::game::engine();
    d.puppetOf = PuppetOf;
    d.ghostOf = GhostOf;
    d.actorMode = [](fnvcoop::PlayerId slot, nvcoop::intents::ActorMode mode, nvcoop::intents::RefId actor) {
        // Logged once per slot and mode change. A stand-in is the HOST'S OWN player acting for the guest (rewards and karma still go to the guest by
        // origin, but `player` inside the scripts is the host): it is only expected with [live] avatars=0 and no NV:MP ghost.
        switch (mode) {
            case nvcoop::intents::ActorMode::Puppet: nvcoop::Log("intents: acting player for slot %d = avatar puppet %08X", (int)slot, (unsigned)actor); break;
            case nvcoop::intents::ActorMode::Ghost: nvcoop::Log("intents: acting player for slot %d = NV:MP ghost %08X", (int)slot, (unsigned)actor); break;
            case nvcoop::intents::ActorMode::StandIn: nvcoop::Log("intents: ACTING-STANDIN slot %d: %s; host player 14 acts", (int)slot, AvatarsNote()); break;
        }
    };
    d.nowMs = NowMs;
    d.log = [](const std::string& line) { nvcoop::Log("intents: %s", line.c_str()); };
    // Kind::SetStage policy (the host keeps the say): an ExclusiveStages loser is refused before its result scripts run. The rules engine's barrier / tether
    // answer through the engine's stage gate (verdict Deferred). Every refusal is a log line here and a counter in intents.status (service.detail.stage).
    d.allowStage = [](fnvcoop::PlayerId slot, const fnvcoop::FormRef& quest, uint32_t stage) {
        nvcoop::coop::Runtime* rt = nvcoop::coop::Runtime::active();
        // G26: a guest's stage on a pass stage (chargen end) is that guest's pass; the stage itself waits for the whole party (the rules engine releases it)
        (void)slot;   // (the barrier pass itself is d.guestStagePass below, wave 1b)
        return !rt || !rt->stageExcluded(quest, stage);
    };
    // A guest's SetStage on a Barrier stage is its pass (G26): the session code that owns the rules engine sets nvcoop::intents::setGuestStagePassHook; unset = NotPass.
    d.guestStagePass = [](fnvcoop::PlayerId slot, const fnvcoop::FormRef& quest, uint32_t stage) {
        const nvcoop::intents::GuestStagePassFn& f = nvcoop::intents::guestStagePassHook();
        return f ? f(slot, quest, stage) : nvcoop::intents::StagePass::NotPass;
    };
    d.stageRefused = [](fnvcoop::PlayerId slot, const fnvcoop::FormRef& quest, uint32_t stage, nvcoop::intents::Result why) {
        nvcoop::Log("intents: guest stage %s=%u from slot %d not run: %s", nvhooks::refToStr(quest).c_str(), (unsigned)stage, (int)slot, resultName(why));
    };
    d.log = [](const std::string& line) { nvcoop::Log("%s", line.c_str()); };
    d.retryList = &g_retry;
    d.containerMenuOpen = [] { return nvcoop::intents::game::containerMenuOpen(); };
    d.startDialogue = [](const fnvcoop::FormRef& npc) {
#ifdef NVCOOP_WITH_UI
        // The guest glue asks the host for the conversation and defers the vanilla talk until it is granted (the activate observer then suppresses it).
        const bool deferred = fndlg::GuestDialogue::instance().onTalkRequested(npc, static_cast<uint32_t>(NowMs()));
        nvcoop::Log("intents: guest talk to %s: %s", nvhooks::refToStr(npc).c_str(), deferred ? "deferred until the host grants it" : "NOT deferred (the dialogue glue is not ready or a conversation is already open), the vanilla talk goes on");
        if (deferred) { g_talkDeferred = true; return; }
#endif
        nvcoop::coop::Runtime* rt = nvcoop::coop::Runtime::active();
        if (rt && rt->dialogue()) rt->dialogue()->requestStart(npc, static_cast<uint32_t>(NowMs()));
    };
#ifdef NVCOOP_WITH_COMPANIONS
    // plugin/companions (companions_glue.h): "allow" / no-op while [companions] enabled=0 or no social session runs.
    d.allowActivation = [](fnvcoop::PlayerId slot, const fnvcoop::FormRef& target) { return nvcoop::social::glueAllowActivation(slot, target); };
    d.interacted = [](fnvcoop::PlayerId slot, bool pickup, const fnvcoop::FormRef& target) { nvcoop::social::glueGuestInteraction(slot, pickup, target); };
#endif
    g_svc.reset(new IntentService(std::move(d), g_cfg, std::move(redirect), std::move(stash)));
    if (g_cfg.saveGate) {
        SaveGate::Deps gd;
        gd.suppressed = [] { return saveSuppressedBy(nvcoop::coop::Runtime::active()); };
        gd.notify = [](const std::string& s) { nvcoop::intents::game::engine().notify(s); };
        gd.nowMs = NowMs;
        g_gate.reset(new SaveGate(std::move(gd)));
        g_canSaveHooked = HookOne(nvcoop::intents::addr::CanSaveNow, reinterpret_cast<void*>(&h_CanSave), reinterpret_cast<void**>(&o_CanSave), "CanSaveNow");
    }
    nvcoop::Log("intents: enabled (activate=%d trigger=%d pickup=%d terminal=%d container=%d hit=%d dialogue=%d standin=%d%s)", g_cfg.activate, g_cfg.trigger,
                g_cfg.pickup, g_cfg.terminal, g_cfg.container, g_cfg.hit, g_cfg.dialogue, g_cfg.standIn, g_cfg.requirePuppet ? " requirePuppet" : "");
}

void Uninit() {
    SaveStashState(true);
    nvhooks::setActivateObserver(nullptr);
    nvhooks::setEventObserver(nullptr);
    nvhooks::setStageRequestObserver(nullptr);
    g_observersOn = false;
    g_reg.reset();   // restores every patched entry
    g_canSaveHooked = g_termHooked = false;
    g_svc.reset();
    g_gate.reset();
    g_inited = false;
}

void OnFrame() {
    if (!g_inited || !g_svc) return;
    nvcoop::coop::Runtime* rt = nvcoop::coop::Runtime::active();
    g_svc->frame(rt);
    SaveStashState(false);
    if (g_svc->client() && g_svc->sessions() != g_observedSessions) {   // a guest session began: take the observers (after coop's own, which are set at start)
        g_observedSessions = g_svc->sessions();
        InstallObservers();
        if (g_cfg.terminal && !g_termHooked)
            g_termHooked = HookOne(nvcoop::intents::addr::TerminalEntryRun, reinterpret_cast<void*>(&h_TermRun), reinterpret_cast<void**>(&o_TermRun), "TerminalEntryRun");
    }
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_DeferredInit) Init();
    else if (type == NVSEMessagingInterface::kMessage_PostLoadGame || type == NVSEMessagingInterface::kMessage_NewGame) { if (g_svc) g_svc->onWorldLoaded(); }
    else if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) Uninit();
}

}  // namespace

namespace nvcoop::intents {
std::string LedgerSnapshot() { return g_svc && g_cfg.storage ? g_svc->stash().stateJson() : std::string(); }
void LedgerRestore(const std::string& text) {
    if (!g_svc) return;
    StorageBook& b = g_svc->stash();
    std::string err;
    if (text.empty()) b.clear();
    else if (!b.loadState(text, &err)) { nvcoop::Log("intents: the gear ledger of this save could not be read (%s): starting with empty stashes", err.c_str()); b.clear(); }
    if (WriteWholeFile(DllDir() + kStashStateFile, b.stateJson())) b.markClean();   // the file next to the dll follows the loaded save too
    nvcoop::Log("intents: gear ledger %s", text.empty() ? "reset (this save has none)" : "restored from the save");
}
}  // namespace nvcoop::intents

NVCOOP_MODULE(guest_intents, "guest_intents", nullptr, OnMessage, OnFrame);

// Hot reload: the hooks are restored and the observers cleared before the core is replaced; the new core installs them again at its next session.
NVCOOP_RELOAD_STATE(guest_intents, "guest_intents", 1, 45,
    /*canUnload*/ [] {
        return g_reg && g_reg->auditTampered() ? std::string("an intents hook was patched over by another mod") : std::string();
    },
    /*save*/ [](nvreload::Writer& w) {
        w.boolean(g_inited);
        return true;
    },
    /*restore*/ [](uint32_t, nvreload::Reader& r) {
        if (r.boolean()) Init();
        return true;
    },
    /*migrate*/ nullptr,
    /*shutdown*/ [] { if (g_inited) Uninit(); },
    /*fresh*/ nullptr);
