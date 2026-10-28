// plugin/world_time glue (x86, compiled into nvcoop.dll next to the coop module). Everything here is OFF until nvcoop.ini says
//
//   [worldtime]
//   enabled=1          the whole module (clock authority, wait / sleep / travel consent, save policy, object poll)
//   travel=free        free | party   (party: every fast trip is a party vote and moves everybody; clients learn it from the host's beacons)
//   restMinutes=60     what a wait / sleep costs when the game's own menu is turned into a party question (the menu's slider is not read)
//   objects=1          the native door / lock / destroyed poll (host sends, clients heal)
//   objectsPerTick=40  refs examined per poll tick (every 10th frame)
//   saves=1            switch the engine's own autosaves off on clients and during a no-save story moment
//
// Environment override for automated runs: NVCOOP_WORLDTIME=1 / 0. Nothing is hooked, polled or sent while it is off.
//
// UNPROVEN IN GAME: every engine-facing piece here compiles and is unit-tested through its interface (wt_engine.h) only. The pieces:
//   * clock = direct writes of the TESGlobals GameYear..GameDaysPassed (0x35..0x39 in FalloutNV.esm)
//   * wait / sleep: the SleepWaitMenu (id 0x3F4) is detected as the top menu and closed; the question goes through L8's raiseModal
//   * fast travel: a detour on the pending-travel setter 0x93BE30 (bots_addrs.h PendingTravelSet), re-issued through its trampoline
//   * autosaves: SetINISetting "bSaveOn...:GamePlay"
// TODO-RE (not found statically): the engine's own time-advance routine and the SleepWaitMenu's chosen amount.
#include <Windows.h>

#include <cstdio>
#include <cstring>
#include <deque>
#include <memory>
#include <set>
#include <string>

#include "GameAPI.h"
#include "GameData.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "PluginAPI.h"
#include "bots/bots_addrs.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "coop/coop_runtime.h"
#include "capture_hub.h"
#include "effect_json.h"
#include "engine_hooks.h"
#include "hooks_ui/ui_addrs.h"
#include "hooks_ui/ui_hooks.h"
#include "hooks_world/world_hooks.h"
#include "wt_objects.h"
#include "wt_save.h"
#include "wt_session.h"

using nvcoop::json::Value;

namespace {

// ------------------------------------------------------------------------------------------------------------------------ config
struct Cfg {
    bool enabled = false;
    bool partyTravel = false;
    uint32_t restMinutes = 60;
    bool objects = true;
    uint32_t objectsPerTick = 40;
    bool saves = true;
    bool onceEvent = true;           // D7: a scripted time skip happens once per party event
    double onceMinHours = 12.0;
    uint32_t onceWindowS = 120;
    double onceToleranceMin = 60.0;
};
Cfg g_cfg;

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

void LoadCfg() {
    const std::string ini = IniPath();
    g_cfg.enabled = GetPrivateProfileIntA("worldtime", "enabled", 0, ini.c_str()) != 0;
    char b[32] = {};
    GetEnvironmentVariableA("NVCOOP_WORLDTIME", b, sizeof b);
    if (b[0] == '1') g_cfg.enabled = true;
    if (b[0] == '0') g_cfg.enabled = false;
    char t[32] = {};
    GetPrivateProfileStringA("worldtime", "travel", "free", t, sizeof t, ini.c_str());
    g_cfg.partyTravel = std::string(t) == "party";
    g_cfg.restMinutes = static_cast<uint32_t>(GetPrivateProfileIntA("worldtime", "restMinutes", 60, ini.c_str()));
    if (g_cfg.restMinutes == 0) g_cfg.restMinutes = 60;
    g_cfg.objects = GetPrivateProfileIntA("worldtime", "objects", 1, ini.c_str()) != 0;
    g_cfg.objectsPerTick = static_cast<uint32_t>(GetPrivateProfileIntA("worldtime", "objectsPerTick", 40, ini.c_str()));
    if (g_cfg.objectsPerTick == 0) g_cfg.objectsPerTick = 40;
    g_cfg.saves = GetPrivateProfileIntA("worldtime", "saves", 1, ini.c_str()) != 0;
    g_cfg.onceEvent = GetPrivateProfileIntA("worldtime", "once_event", 1, ini.c_str()) != 0;
    g_cfg.onceMinHours = static_cast<double>(GetPrivateProfileIntA("worldtime", "once_min_hours", 12, ini.c_str()));
    g_cfg.onceWindowS = static_cast<uint32_t>(GetPrivateProfileIntA("worldtime", "once_window_s", 120, ini.c_str()));
    g_cfg.onceToleranceMin = static_cast<double>(GetPrivateProfileIntA("worldtime", "once_tolerance_min", 60, ini.c_str()));
}

uint64_t NowMs() { return GetTickCount64(); }

// ------------------------------------------------------------------------------------------------------------------------ hooking
struct Win32Memory : nvhooks::MemoryOps {
    bool read(nvhooks::Addr a, void* out, size_t n) override { return nvworld::peekMemory(a, out, n); }
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

// thiscall(player, travelObj), ret 4: the pending-travel setter the map menu's "Yes" worker calls (docs/tools/T6a-actions.md).
// Prologue bytes are bots_addrs.h PendingTravelSet; 55 8B EC 83 EC 08 are whole instructions (6 stolen).
const nvhooks::HookSpec kPendingTravel = {"WT_PendingTravelSet", 0x93BE30u,
    {0x55, 0x8b, 0xec, 0x83, 0xec, 0x08, 0x89, 0x4d, 0xf8, 0x8b, 0x45, 0xf8, 0x83, 0xb8, 0xec, 0x01}, 16, 6, 4};
using PendingFn = unsigned(__thiscall*)(void*, void*);
PendingFn o_PendingTravel = nullptr;
bool g_reissue = false;   // our own re-issue after a grant: let it through

// ------------------------------------------------------------------------------------------------------------------------ small engine helpers
bool PinOk(unsigned addr, const unsigned char* bytes) {
    unsigned char got[16] = {};
    return nvworld::peekMemory(addr, got, 16) && std::memcmp(got, bytes, 16) == 0;
}
bool g_pinsChecked = false, g_pinsOk = false;
bool Pins() {
    if (g_pinsChecked) return g_pinsOk;
    g_pinsChecked = true;
    namespace a = nvbots::addr;
    g_pinsOk = PinOk(a::TopMenuId.addr, a::TopMenuId.bytes) && PinOk(a::PlayerCanTravel.addr, a::PlayerCanTravel.bytes) &&
               PinOk(a::TravelObjCtor.addr, a::TravelObjCtor.bytes) && PinOk(a::HeapAlloc.addr, a::HeapAlloc.bytes) &&
               PinOk(nvui::addr::CloseAllMenusCore.addr, nvui::addr::CloseAllMenusCore.bytes);
    if (!g_pinsOk) nvcoop::Log("worldtime: an engine pin (menu / travel helpers) does not match this exe: menu and travel interception off");
    return g_pinsOk;
}

// one tiny function per call so __try never meets C++ objects
bool SafeThis1(unsigned fn, void* self, unsigned* out) {
    __try {
        *out = reinterpret_cast<unsigned(__thiscall*)(void*)>(fn)(self);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool SafeCdecl0(unsigned fn) {
    __try {
        reinterpret_cast<void(__cdecl*)()>(fn)();
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool SafeAlloc(unsigned size, unsigned* out) {
    __try {
        *out = reinterpret_cast<unsigned(__cdecl*)(unsigned)>(nvbots::addr::HeapAlloc.addr)(size);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool SafePending(void* player, void* obj) {
    __try {
        o_PendingTravel(player, obj);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

constexpr unsigned kInterfaceMgrPtr = 0x11D8A80, kThePlayerPtr = 0x11DEA3C;
constexpr unsigned kMenuSleepWait = 0x3F4;

unsigned TopMenu() {
    if (!Pins()) return 0;
    unsigned mgr = 0, id = 0;
    if (!nvworld::peekMemory(kInterfaceMgrPtr, &mgr, 4) || !mgr) return 0;
    return SafeThis1(nvbots::addr::TopMenuId.addr, reinterpret_cast<void*>(mgr), &id) ? id : 0;
}
void CloseMenus() {
    if (Pins()) SafeCdecl0(nvui::addr::CloseAllMenusCore.addr);
}

TESGlobal* Global(uint32_t localId) {
    uint32_t id = nvhooks::resolver().fromRef(nvwt::FormRef("falloutnv.esm", localId));
    return static_cast<TESGlobal*>(id ? nvhooks::resolver().lookup(id) : nullptr);
}

// ------------------------------------------------------------------------------------------------------------------------ the engine port
class GameEngine : public nvwt::IWorldTimeEngine {
public:
    bool readClock(nvwt::GameTime* out) override {
        TESGlobal *y = Global(0x35), *mo = Global(0x36), *d = Global(0x37), *h = Global(0x38), *dp = Global(0x39);
        if (!y || !mo || !d || !h || !dp) return false;
        out->year = static_cast<int32_t>(y->data + 0.5f);
        out->month = static_cast<int32_t>(mo->data + 0.5f);
        out->day = static_cast<int32_t>(d->data + 0.5f);
        out->hour = h->data;
        out->daysPassed = dp->data;
        return true;
    }
    bool writeClock(const nvwt::GameTime& t) override {
        TESGlobal *y = Global(0x35), *mo = Global(0x36), *d = Global(0x37), *h = Global(0x38), *dp = Global(0x39);
        if (!y || !mo || !d || !h || !dp) return false;
        y->data = static_cast<float>(t.year);
        mo->data = static_cast<float>(t.month);
        d->data = static_cast<float>(t.day);
        h->data = static_cast<float>(t.hour);
        dp->data = static_cast<float>(t.daysPassed);
        return true;
    }
    float timeScale() override { return nvworld::currentTimeScale(); }
    void notify(const std::string& text) override { QueueUIMessage(text.c_str(), 0, nullptr, nullptr, 3.0f, false); }
    bool startTravel(const nvwt::FormRef& marker) override {
        if (!Pins() || !o_PendingTravel) return false;
        unsigned player = 0;
        if (!nvworld::peekMemory(kThePlayerPtr, &player, 4) || !player) return false;
        const uint32_t id = nvhooks::resolver().fromRef(marker);
        void* ref = id ? nvhooks::resolver().lookup(id) : nullptr;
        if (!ref) {
            nvcoop::Log("worldtime: travel marker %s is not known here", nvhooks::refToStr(marker).c_str());
            return false;
        }
        unsigned ok = 0;
        // the engine's own may-travel check shows its own refusal message (combat, hostiles, overencumbered ...)
        if (!SafeThis1(nvbots::addr::PlayerCanTravel.addr, reinterpret_cast<void*>(player), &ok) || !(ok & 0xFF)) return false;
        unsigned mem = 0;
        if (!SafeAlloc(nvbots::addr::kTravelObjSize, &mem) || !mem) return false;
        unsigned obj = 0;
        if (!SafeThis1(nvbots::addr::TravelObjCtor.addr, reinterpret_cast<void*>(mem), &obj) || !obj) return false;
        *reinterpret_cast<void**>(obj + nvbots::addr::kTravelObjMarker) = ref;
        g_reissue = true;
        const bool done = SafePending(reinterpret_cast<void*>(player), reinterpret_cast<void*>(obj));
        g_reissue = false;
        return done;
    }
};
GameEngine g_engine;

// ------------------------------------------------------------------------------------------------------------------------ state
using nvcoop::coop::Runtime;
struct State {
    Runtime* rt = nullptr;
    nvwt::Role role = nvwt::Role::Host;
    nvwt::PlayerId self = nvwt::kHostPlayer;
    std::unique_ptr<nvwt::WorldTimeSession> session;
    std::unique_ptr<nvwt::ObjectWatcher> watcher;
    std::set<uint32_t> prompts;                              // menu ids we raised
    struct Result { uint32_t id; int32_t choice; bool timedOut; };
    std::deque<Result> results;
    std::set<nvwt::PlayerId> joined;
    void* wrappedNode = nullptr;
    bool menuWasOpen = false;
    size_t cursor = 0;
    uint64_t lastObjMs = 0;
    uint32_t frame = 0;
    bool hostCombat = false;
};
State g_st;
nvwt::SaveGuard g_guard;
bool g_hooked = false;

nvwt::PlayerId SlotOfPeer(fnvcoop::PeerId peer) {
    if (g_st.role == nvwt::Role::Client) return nvwt::kHostPlayer;
    if (auto* h = g_st.rt ? g_st.rt->host() : nullptr)
        for (const auto& cv : h->clients())
            if (cv.peer == peer && cv.joined) return cv.slot;
    return nvwt::kNoPlayer;
}

bool AnyTrue(const std::map<nvwt::FormRef, bool>& m) {
    for (const auto& kv : m) if (kv.second) return true;
    return false;
}

void RecordEffects(const std::vector<fnvcoop::Effect>& fx) {
    nvhooks::CaptureHub* hub = nvhooks::hub();
    if (!hub || !hub->enabled() || fx.empty() || hub->depth() != 0) return;
    fnvcoop::Cause c;
    c.kind = fnvcoop::CauseKind::Script;
    c.aux = nvworld::kObjectPollAux;   // hooks_world sends a script's ObjectState a second time as a world effect; the poll's own is not repeated
    nvhooks::ScopedCapture sc(*hub, c);
    for (const auto& e : fx) hub->record(e);
}

// ------------------------------------------------------------------------------------------------------------------------ session lifecycle
void Wrapper(uint32_t menu, int32_t choice, bool timedOut) {
    if (g_st.prompts.count(menu)) {
        g_st.results.push_back({menu, choice, timedOut});
        return;
    }
    if (auto* r = nvui::router()) r->onMenuResume(menu, choice, timedOut);   // not ours: L11 keeps getting its boxes
}

uint32_t AskParty(nvwt::PlayerId initiator, nvwt::ReqKind kind, const std::string& text, uint32_t timeoutMs) {
    auto* dlg = g_st.rt ? g_st.rt->dialogue() : nullptr;
    if (!dlg) {
        nvcoop::Log("worldtime: no dialogue node, the party question cannot be asked (%s)", text.c_str());
        return 0;
    }
    fnvcoop_dlg::HostDialogue::ModalOptions opt;
    opt.partyConsent = true;
    opt.confirmButton = 1;
    opt.declineChoice = 0;
    opt.timeoutMs = timeoutMs;
    const uint32_t id = dlg->raiseModal(initiator, text, {"Not now", "Yes"}, 0, opt, static_cast<uint32_t>(GetTickCount64()));
    if (id) g_st.prompts.insert(id);
    (void)kind;
    return id;
}

void TearDown() {
    if (g_st.session || g_st.watcher) nvcoop::Log("worldtime: session ended");
    g_st.session.reset();
    g_st.watcher.reset();
    g_st.prompts.clear();
    g_st.results.clear();
    g_st.joined.clear();
    g_st.wrappedNode = nullptr;
    g_st.rt = nullptr;
    g_st.cursor = 0;
    nvworld::setAdvanceClockHandler(nullptr);
    nvworld::setObjectStateObserver(nullptr);
}

void Build(Runtime* rt) {
    g_st.rt = rt;
    g_st.role = rt->host() ? nvwt::Role::Host : nvwt::Role::Client;
    g_st.self = g_st.role == nvwt::Role::Host ? nvwt::kHostPlayer : (rt->client() && rt->client()->joined() ? rt->client()->slot() : nvwt::kNoPlayer);
    nvwt::SessionConfig cfg;
    cfg.broker.travel = g_cfg.partyTravel ? nvwt::TravelMode::Party : nvwt::TravelMode::Free;
    cfg.authority.onceEnabled = g_cfg.onceEvent;
    cfg.authority.onceMinMinutes = g_cfg.onceMinHours * 60.0;
    cfg.authority.onceWindowMs = g_cfg.onceWindowS * 1000u;
    cfg.authority.onceToleranceMinutes = g_cfg.onceToleranceMin;
    nvwt::SessionEnv env;
    env.engine = &g_engine;
    fnvcoop::MuxTransport* mux = rt->mux();
    if (g_st.role == nvwt::Role::Host) {
        env.send = [rt, mux](nvwt::PlayerId to, nvwt::Bytes d) {
            auto* h = rt->host();
            if (!h || !mux) return;
            for (const auto& cv : h->clients())
                if (cv.joined && (to == nvwt::kNoPlayer || cv.slot == to)) mux->send(cv.peer, d);
        };
        env.party.joined = [rt]() {
            uint32_t m = 1;
            if (auto* h = rt->host()) for (const auto& cv : h->clients()) if (cv.joined) m |= nvwt::bit(cv.slot);
            return m;
        };
        env.party.inCombat = []() { return g_st.hostCombat ? 1u : 0u; };
        env.party.tethered = [rt]() { auto* h = rt->host(); return h && AnyTrue(h->state().tethers); };
        env.party.travelLocked = [rt]() { auto* h = rt->host(); return h && AnyTrue(h->state().fastTravelLock); };
        env.ask = AskParty;
        env.record = [](const fnvcoop::Effect& e) { RecordEffects({e}); };
    } else {
        env.send = [mux](nvwt::PlayerId, nvwt::Bytes d) { if (mux) mux->send(0, std::move(d)); };
        env.party.tethered = [rt]() { auto* c = rt->client(); return c && AnyTrue(c->state().tethers); };
        env.localInCombat = []() { bool c = false; return nvworld::queryInCombat(0x14, &c) && c; };
    }
    g_st.session.reset(new nvwt::WorldTimeSession(g_st.role, g_st.self, cfg, env));
    g_st.watcher.reset(new nvwt::ObjectWatcher(g_st.role));
    if (mux) {
        mux->registerChannel(nvwt::kChannelWorldTime, [](fnvcoop::PeerId from, fnvcoop::Bytes&& d) {
            if (!g_st.session) return;
            const nvwt::PlayerId slot = SlotOfPeer(from);
            if (slot == nvwt::kNoPlayer) return;
            g_st.session->onPacket(slot, d, NowMs());
        });
    }
    nvworld::setAdvanceClockHandler([](uint32_t minutes) {
        if (g_st.session) g_st.session->onAdvanceTimeEffect(minutes, NowMs());
        return true;
    });
    nvworld::setObjectStateObserver([](const fnvcoop::ObjectState& s) {
        if (g_st.role == nvwt::Role::Client)
            nvcoop::Log("worldtime: host's word on %s mask=%u open=%d locked=%d", nvhooks::refToStr(s.ref).c_str(), (unsigned)s.mask, (int)s.open, (int)s.locked);
        if (g_st.watcher) g_st.watcher->onAuthoritative(s);
    });
    nvcoop::Log("worldtime: session started role=%s travel=%s", g_st.role == nvwt::Role::Host ? "host" : "client", g_cfg.partyTravel ? "party" : "free");
}

// ------------------------------------------------------------------------------------------------------------------------ travel detour
unsigned __fastcall h_PendingTravel(void* player, void* /*edx*/, void* obj) {
    if (!g_reissue && obj && g_st.session && !g_st.session->awaitingAnswer() ) {
        unsigned markerPtr = 0;
        nvwt::FormRef mref;
        if (nvworld::peekMemory(reinterpret_cast<unsigned>(obj) + nvbots::addr::kTravelObjMarker, &markerPtr, 4) && markerPtr &&
            nvworld::refToString(static_cast<const TESForm*>(reinterpret_cast<const void*>(markerPtr))->refID, &mref)) {
            if (!g_st.session->onLocalTravel(mref, NowMs())) return 0;   // vetoed: the party decides, startTravel re-issues it
        }
    } else if (!g_reissue && obj && g_st.session && g_st.session->awaitingAnswer()) {
        return 0;   // a second click while the first question is open
    }
    return o_PendingTravel(player, obj);
}

// ------------------------------------------------------------------------------------------------------------------------ per frame
void ObjectTick(uint64_t now) {
    if (!g_cfg.objects || !g_st.watcher) return;
    if (g_st.role == nvwt::Role::Host) {   // what the watcher sends must be recordable now, or the book would say "sent" for a change nobody got
        nvhooks::CaptureHub* hub = nvhooks::hub();
        if (!hub || !hub->enabled() || hub->depth() != 0) return;
    }
    size_t total = 0;
    auto batch = nvworld::objectsInPlayerCell(g_st.cursor, g_cfg.objectsPerTick, &total);
    g_st.cursor = (total == 0 || g_st.cursor + g_cfg.objectsPerTick >= total) ? 0 : g_st.cursor + g_cfg.objectsPerTick;
    std::vector<fnvcoop::Effect> record;
    for (const auto& o : batch) {
        const nvworld::ObjectReading rd = nvworld::readObject(o);
        if (!rd.ok) continue;
        nvwt::ObjObs obs;
        nvhooks::strToRef(o.ref, &obs.ref);
        obs.kind = o.kind == 0 ? nvwt::ObjKind::Door : o.kind == 1 ? nvwt::ObjKind::Container : nvwt::ObjKind::Terminal;
        obs.openState = rd.openState;
        obs.locked = rd.locked;
        obs.lockLevel = rd.lockLevel;
        obs.destroyed = rd.destroyed;
        if (g_st.role == nvwt::Role::Host) {
            for (auto& s : g_st.watcher->observeHost(obs, now)) {
                nvcoop::Log("worldtime: poll sends ObjectState %s mask=%u open=%d locked=%d", o.ref.c_str(), (unsigned)s.mask, (int)s.open, (int)s.locked);
                record.push_back(s);
            }
        } else if (auto heal = g_st.watcher->observeGuest(obs, now)) {
            // the host's word wins: re-apply it the way the applier would (console lines on the object, with the block lifted)
            fnvcoop::EchoGuard echo(nvhooks::hub() ? &nvhooks::hub()->echo() : nullptr);
            nvhooks::ApplyScope ap;
            nvhooks::ApplierExecScope ex;   // no ScriptGate: a console line is a script and an open gate only pretends it ran (a heal did nothing, again and again)
            if (heal->mask & fnvcoop::kObjLock) nvhooks::execConsoleLineOn(heal->locked ? nvhooks::lockCmd(heal->level) : nvhooks::unlockCmd(), o.refID);
            if (heal->mask & fnvcoop::kObjOpen) nvhooks::execConsoleLineOn(nvhooks::setOpenStateCmd(heal->open), o.refID);
            if (heal->mask & fnvcoop::kObjDestroyed) nvhooks::execConsoleLineOn(nvhooks::setDestroyedCmd(heal->destroyed), o.refID);
            nvcoop::Log("worldtime: healed %s (mask %u) to the host's state", o.ref.c_str(), (unsigned)heal->mask);
        }
    }
    RecordEffects(record);
}

void MenuTick() {
    if (!g_st.session) return;
    const bool open = TopMenu() == kMenuSleepWait;
    if (open && !g_st.menuWasOpen) {
        if (!g_st.session->onLocalRest(nvwt::ReqKind::Wait, g_cfg.restMinutes, NowMs())) CloseMenus();   // vetoed: the party is asked instead
    }
    g_st.menuWasOpen = open;
}

void SaveTick(Runtime* rt) {
    if (!g_cfg.saves) return;
    nvwt::SaveContext c;
    c.inSession = rt != nullptr;
    if (rt) {
        c.role = rt->host() ? nvwt::Role::Host : nvwt::Role::Client;
        c.joined = rt->client() ? rt->client()->joined() : true;
        c.saveSuppressed = rt->host() ? rt->host()->state().saveSuppress : (rt->client() && rt->client()->state().saveSuppress);
        c.syncing = rt->syncing();
        if (c.role == nvwt::Role::Client && !c.joined) c.inSession = false;   // not in yet: the player's own game still saves as usual
    }
    for (const auto& line : g_guard.update(c)) {
        nvcoop::Log("worldtime: %s", line.c_str());
        nvhooks::execConsoleLine(line);
    }
}

void OnFrame() {
    if (!g_cfg.enabled) return;
    Runtime* rt = Runtime::active();
    if (rt && !rt->running()) rt = nullptr;
    if (!rt) {
        if (g_st.session) TearDown();
        SaveTick(nullptr);
        return;
    }
    const nvwt::Role role = rt->host() ? nvwt::Role::Host : nvwt::Role::Client;
    const nvwt::PlayerId self = role == nvwt::Role::Host ? nvwt::kHostPlayer : (rt->client() && rt->client()->joined() ? rt->client()->slot() : nvwt::kNoPlayer);
    if (!g_st.session || g_st.rt != rt || g_st.role != role || g_st.self != self) {
        if (g_st.session) TearDown();
        if (role == nvwt::Role::Client && self == nvwt::kNoPlayer) {   // not joined yet: wait
            SaveTick(rt);
            return;
        }
        Build(rt);
    }
    const uint64_t now = NowMs();
    ++g_st.frame;
    if (g_st.frame % 15 == 0 && role == nvwt::Role::Host) {
        bool c = false;
        g_st.hostCombat = nvworld::queryInCombat(0x14, &c) && c;
    }
    // the party question goes through L8's DialogueNode: our answers come back through a wrapper that leaves L11's boxes alone
    if (auto* dlg = rt->dialogue()) {
        dlg->setMenuResume(Wrapper);
        g_st.wrappedNode = dlg;
    }
    if (role == nvwt::Role::Host) {
        std::set<nvwt::PlayerId> nowJoined;
        if (auto* h = rt->host()) for (const auto& cv : h->clients()) if (cv.joined) nowJoined.insert(cv.slot);
        for (auto p : nowJoined) if (!g_st.joined.count(p)) g_st.session->onPlayerJoined(p, now);
        for (auto p : g_st.joined) if (!nowJoined.count(p)) g_st.session->onPlayerLeft(p, now);
        g_st.joined = nowJoined;
    }
    while (!g_st.results.empty()) {
        const auto r = g_st.results.front();
        g_st.results.pop_front();
        g_st.prompts.erase(r.id);
        g_st.session->onPromptResult(r.id, !r.timedOut && r.choice == 1, r.timedOut, now);
    }
    g_st.session->frame(now);
    if (g_st.frame % 10 == 0 && !rt->syncing()) ObjectTick(now);
    MenuTick();
    SaveTick(rt);
}

// ------------------------------------------------------------------------------------------------------------------------ module + bridge
void OnLoad(void*) {
    LoadCfg();
    if (!g_cfg.enabled) return;
    if (!g_reg) g_reg = std::make_unique<nvhooks::HookRegistry>(g_mem);
    nvhooks::Addr tramp = 0;
    nvhooks::PatchStatus st = g_reg->acquire(kPendingTravel, reinterpret_cast<nvhooks::Addr>(&h_PendingTravel), &tramp, reinterpret_cast<nvhooks::Addr*>(&o_PendingTravel));
    if (nvhooks::installed(st)) {
        o_PendingTravel = reinterpret_cast<PendingFn>(tramp);
        g_hooked = true;
    }
    nvcoop::Log("worldtime: enabled travel=%s restMinutes=%u objects=%d saves=%d; pending-travel hook %s", g_cfg.partyTravel ? "party" : "free",
                g_cfg.restMinutes, g_cfg.objects ? 1 : 0, g_cfg.saves ? 1 : 0, nvhooks::statusName(st));
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (!g_cfg.enabled) return;
    if (type == NVSEMessagingInterface::kMessage_PostLoadGame || type == NVSEMessagingInterface::kMessage_NewGame) {
        if (g_st.session) g_st.session->onHostLoaded();
        if (g_st.watcher) g_st.watcher->clear();   // another world: whatever we knew about its objects is moot
        g_st.cursor = 0;
    }
    if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) {
        for (const auto& line : g_guard.restore()) nvhooks::execConsoleLine(line);
        if (g_reg) { g_reg.reset(); g_hooked = false; }
    }
}

bool MStatus(const std::string&, std::string& out) {
    Value o = Value::object();
    o.set("enabled", g_cfg.enabled).set("hooked", g_hooked).set("pins", Pins())
        .set("partyTravel", g_st.session ? g_st.session->partyTravel() : g_cfg.partyTravel)   /* a client: the host's last beacon */
        .set("session", g_st.session != nullptr).set("role", g_st.role == nvwt::Role::Host ? "host" : "client");
    if (g_st.session) {
        const auto& s = g_st.session->stats();
        Value st = Value::object();
        st.set("beaconsSent", (int64_t)s.beaconsSent).set("beaconsRecv", (int64_t)s.beaconsRecv).set("snaps", (int64_t)s.snaps)
            .set("advances", (int64_t)s.advances).set("skipsExecuted", (int64_t)s.skipsExecuted).set("requestsHandled", (int64_t)s.requestsHandled)
            .set("verdictsSent", (int64_t)s.verdictsSent).set("verdictsRecv", (int64_t)s.verdictsRecv).set("travelsStarted", (int64_t)s.travelsStarted)
            .set("badPackets", (int64_t)s.badPackets).set("unexpected", (int64_t)s.unexpected);
        o.set("stats", st).set("awaiting", g_st.session->awaitingAnswer()).set("drift", g_st.session->follower().lastDrift())
            .set("epochSeen", (int64_t)g_st.session->follower().epochSeen());
    }
    if (g_st.watcher) {
        const auto& s = g_st.watcher->stats();
        Value w = Value::object();
        w.set("book", (int64_t)g_st.watcher->size()).set("firstSight", (int64_t)s.firstSight).set("emitted", (int64_t)s.emitted).set("healed", (int64_t)s.healed)
            .set("echoSuppressed", (int64_t)s.echoSuppressed);
        o.set("objects", w);
    }
    o.set("autosavesOff", g_guard.autosavesOff());
    out = o.dump();
    return true;
}

// worldtime.clock {} -> the five globals; {set:{year,month,day,hour,daysPassed}} writes them (proof helper: needs NVCOOP_TEST_HELPERS=1)
bool MClock(const std::string& params, std::string& out) {
    nvwt::GameTime t;
    Value p;
    std::string err;
    if (!params.empty()) nvcoop::json::parse(params, p, err);
    if (p.find("set")) {
        char env[8] = {};
        if (!GetEnvironmentVariableA("NVCOOP_TEST_HELPERS", env, sizeof env) || env[0] != '1') { out = "worldtime.clock set needs NVCOOP_TEST_HELPERS=1"; return false; }
        const Value& s = p.get("set");
        t.year = (int32_t)s.get("year").asInt();
        t.month = (int32_t)s.get("month").asInt();
        t.day = (int32_t)s.get("day").asInt();
        t.hour = s.get("hour").asNumber();
        t.daysPassed = s.get("daysPassed").asNumber();
        if (!nvwt::validTime(t) || !g_engine.writeClock(t)) { out = "invalid time or globals missing"; return false; }
    }
    if (!g_engine.readClock(&t)) { out = "clock globals not found"; return false; }
    Value o = Value::object();
    o.set("year", (int64_t)t.year).set("month", (int64_t)t.month).set("day", (int64_t)t.day).set("hour", t.hour).set("daysPassed", t.daysPassed)
        .set("timeScale", (double)g_engine.timeScale());
    out = o.dump();
    return true;
}

// worldtime.rest {kind:"wait|sleep", minutes} / worldtime.travel {marker:"falloutnv.esm:XXXX"}: the same entry points the menu poll and the
// travel detour use, for the in-game proof. Reply: {proceed:bool}.
bool MRest(const std::string& params, std::string& out) {
    if (!g_st.session) { out = "no world-time session"; return false; }
    Value p;
    std::string err;
    nvcoop::json::parse(params, p, err);
    const bool sleep = p.get("kind").asString("wait") == "sleep";
    const uint32_t minutes = static_cast<uint32_t>(p.get("minutes").asInt(g_cfg.restMinutes));
    const bool go = g_st.session->onLocalRest(sleep ? nvwt::ReqKind::Sleep : nvwt::ReqKind::Wait, minutes, NowMs());
    Value o = Value::object();
    o.set("proceed", go).set("awaiting", g_st.session->awaitingAnswer());
    out = o.dump();
    return true;
}
bool MTravel(const std::string& params, std::string& out) {
    if (!g_st.session) { out = "no world-time session"; return false; }
    Value p;
    std::string err;
    nvcoop::json::parse(params, p, err);
    nvwt::FormRef m;
    if (!nvhooks::strToRef(p.get("marker").asString(""), &m)) { out = "bad marker"; return false; }
    const bool go = g_st.session->onLocalTravel(m, NowMs());
    Value o = Value::object();
    o.set("proceed", go).set("awaiting", g_st.session->awaitingAnswer());
    out = o.dump();
    return true;
}

// worldtime.objects {max} -> what the poll would see in the player's cell (ref, kind, readings): the proof compares host and client.
bool MObjects(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    nvcoop::json::parse(params, p, err);
    const size_t max = static_cast<size_t>(p.get("max").asInt(64));
    size_t total = 0;
    auto list = nvworld::objectsInPlayerCell(0, max, &total);
    Value arr = Value::array();
    for (const auto& o : list) {
        const auto rd = nvworld::readObject(o);
        Value e = Value::object();
        e.set("ref", o.ref).set("kind", (int64_t)o.kind).set("ok", rd.ok).set("open", (int64_t)rd.openState).set("locked", rd.locked)
            .set("level", (int64_t)rd.lockLevel).set("destroyed", rd.destroyed);
        arr.push(e);
    }
    Value o = Value::object();
    o.set("total", (int64_t)total).set("objects", arr);
    out = o.dump();
    return true;
}

}  // namespace

// The module registration lives here (a direct source of the final DLL, so the registrar is not dropped by the linker).
static void WtLoad(void* nvse) {
    LoadCfg();
    OnLoad(nvse);
    nvcoop::RegisterMethod("worldtime.status", MStatus, true);
    nvcoop::RegisterMethod("worldtime.clock", MClock, true);
    nvcoop::RegisterMethod("worldtime.rest", MRest, true);
    nvcoop::RegisterMethod("worldtime.travel", MTravel, true);
    nvcoop::RegisterMethod("worldtime.objects", MObjects, true);
}
NVCOOP_MODULE(world_time, "world_time", WtLoad, OnMessage, OnFrame);
