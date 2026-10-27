// nvcoop module for the bot input driver (T6a). DEV ONLY: nothing is hooked and no method is registered unless nvcoop.ini has
//   [dev]
//   bots=1
// It detours OSInputGlobals::GetControlState (the one function the game asks "is control X down?") so a bot can hold / tap game
// controls from inside our own process. bot.vats also detours the VATS attack-button test 0x7EEB50 (armed only by a bot.vats job).
// No SendInput, no desktop events, works with the window minimized. See bot_rpc.h for the
// bot.* bridge methods and docs/tools/T6a-actions.md for the in-game proof script.
#include <Windows.h>

#include <cstring>
#include <memory>
#include <string>

#include "PluginAPI.h"
#include "GameObjects.h"
#include "bot_rpc.h"
#include "bots_addrs.h"
#include "common/nvcoop_api.h"
#include "common/registry.h"
#include "effect_json.h"
#include "engine_hooks.h"
#include "hook_patch.h"
#include "reload/reload_state.h"

namespace {

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
nvbots::ForcedInput g_in;
std::unique_ptr<nvbots::BotRpc> g_rpc;
bool g_enabled = false, g_hooked = false, g_inited = false;
long g_queries = 0;

using Fn_GetControlState = int(__thiscall*)(void*, int, int);
Fn_GetControlState o_GetControlState = nullptr;

int __fastcall h_GetControlState(void* self, void*, int ctrl, int state) {
    const int real = o_GetControlState(self, ctrl, state);
    ++g_queries;
    return g_in.filter(ctrl, state, real);
}

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

// ---- live IEngine: raw reads are SEH-guarded, every engine call first needs the pinned prologues to have matched at init ----
bool g_callsOk = false;
// bot.vats accept: the detour on the VATS attack-button test 0x7EEB50 answers from this gate (main thread only: the job arms it in
// OnFrame, VATSMenu::Update calls the detour).
nvbots::VatsAcceptGate g_accept;
bool g_acceptHooked = false;
// The game main thread: written at DeferredInit and on every frame (both run on it). 0 until then, so the native press refuses.
volatile DWORD g_mainThread = 0;

template <class R, class... A>
R CallThis(unsigned fn, void* self, A... a) {
    return reinterpret_cast<R(__thiscall*)(void*, A...)>(fn)(self, a...);
}
// one tiny function per call so __try never meets C++ objects
template <class R, class... A>
bool GuardedThis(R* out, unsigned fn, void* self, A... a) {
    __try {
        *out = CallThis<R, A...>(fn, self, a...);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

struct LiveGame : nvbots::IEngine {
    bool rd32(nvbots::Addr32 a, uint32_t* v) override { return g_mem.read(a, v, 4); }
    bool rd8(nvbots::Addr32 a, uint8_t* v) override { return g_mem.read(a, v, 1); }
    bool wr8(nvbots::Addr32 a, uint8_t v) override { return g_mem.write(a, &v, 1); }
    bool wr32(nvbots::Addr32 a, uint32_t v) override { return g_mem.write(a, &v, 4); }
    uint32_t nowTick() override {
        int r = 0;
        return g_callsOk && GuardedThis<int>(&r, nvbots::addr::Tick.addr, reinterpret_cast<void*>(nvbots::eng::kClockObj)) ? (uint32_t)r : 0;
    }
    bool lbSelect(nvbots::Addr32 lb, int idx) override {
        unsigned r = 0;
        return g_callsOk && GuardedThis<unsigned>(&r, nvbots::addr::LbSelect.addr, reinterpret_cast<void*>(lb), idx) && (r & 0xFF);
    }
    nvbots::Addr32 lbSelected(nvbots::Addr32 lb) override {
        unsigned r = 0;
        return g_callsOk && GuardedThis<unsigned>(&r, nvbots::addr::LbSelected.addr, reinterpret_cast<void*>(lb)) ? r : 0;
    }
    nvbots::Addr32 itemInfo(nvbots::Addr32 item) override {
        unsigned r = 0;
        return g_callsOk && GuardedThis<unsigned>(&r, nvbots::addr::ItemInfo.addr, reinterpret_cast<void*>(item)) ? r : 0;
    }
    bool infoSpecial(nvbots::Addr32 info) override {
        if (!g_callsOk) return true;   // unknown means refuse
        unsigned a = 0;
        if (!GuardedThis<unsigned>(&a, nvbots::addr::InfoFlag80.addr, reinterpret_cast<void*>(info))) return true;
        return (a & 0xFF) != 0;
    }
    bool infoSpeechCheck(nvbots::Addr32 info) override {
        if (!g_callsOk) return false;
        unsigned b = 0, c = 0;
        if (!GuardedThis<unsigned>(&b, nvbots::addr::InfoSpecialB.addr, reinterpret_cast<void*>(info)) ||
            !GuardedThis<unsigned>(&c, nvbots::addr::InfoSpecialC.addr, reinterpret_cast<void*>(info)))
            return false;
        return (b & 0xFF) || (c & 0xFF);
    }
    bool canFastTravel(nvbots::Addr32 player) override {
        unsigned r = 0;
        return g_callsOk && GuardedThis<unsigned>(&r, nvbots::addr::PlayerCanTravel.addr, reinterpret_cast<void*>(player)) && (r & 0xFF);
    }
    nvbots::Addr32 newTravelObj() override {
        if (!g_callsOk) return 0;
        unsigned mem = 0, obj = 0;
        __try {
            mem = reinterpret_cast<unsigned(__cdecl*)(unsigned)>(nvbots::addr::HeapAlloc.addr)(nvbots::addr::kTravelObjSize);
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return 0;
        }
        if (!mem) return 0;
        return GuardedThis<unsigned>(&obj, nvbots::addr::TravelObjCtor.addr, reinterpret_cast<void*>(mem)) ? obj : 0;
    }
    void setPendingTravel(nvbots::Addr32 player, nvbots::Addr32 obj) override {
        unsigned r = 0;
        if (g_callsOk) GuardedThis<unsigned>(&r, nvbots::addr::PendingTravelSet.addr, reinterpret_cast<void*>(player), (unsigned)obj);
    }
    void dlgItemClick(nvbots::Addr32 menu, nvbots::Addr32 item) override {
        unsigned r = 0;
        if (g_callsOk) GuardedThis<unsigned>(&r, nvbots::addr::DlgItemClick.addr, reinterpret_cast<void*>(menu), (unsigned)item);
    }
    void lbAfter(nvbots::Addr32 lb) override {
        unsigned r = 0;
        if (g_callsOk) GuardedThis<unsigned>(&r, nvbots::addr::LbAfter.addr, reinterpret_cast<void*>(lb));
    }
    int dlgSpeak(nvbots::Addr32 menu, int flag) override {
        int r = 0;
        return g_callsOk && GuardedThis<int>(&r, nvbots::addr::DlgSpeak.addr, reinterpret_cast<void*>(menu), flag) ? r : 0;
    }
    void termClick(nvbots::Addr32 menu, nvbots::Addr32 tile) override {
        unsigned r = 0;
        if (g_callsOk) GuardedThis<unsigned>(&r, nvbots::addr::TermClick.addr, reinterpret_cast<void*>(menu), -1, (unsigned)tile);
    }
    uint32_t topMenuId(nvbots::Addr32 mgr) override {
        unsigned r = 0;
        return g_callsOk && GuardedThis<unsigned>(&r, nvbots::addr::TopMenuId.addr, reinterpret_cast<void*>(mgr)) ? r : 0;
    }
    bool onMainThread() override { return g_mainThread != 0 && GetCurrentThreadId() == g_mainThread; }
    // MessageMenu vtable slot 3 = HandleClick(id, tile). The logic layer already checked the vtable; check it again here together with the
    // slot content, so this call can only ever reach the pinned 0x7AA070 on a MessageMenu, on the main thread.
    bool msgMenuHandleClick(nvbots::Addr32 menu, int id, nvbots::Addr32 tile) override {
        if (!g_callsOk || !onMainThread() || !menu || !tile) return false;
        uint32_t vt = 0, fn = 0;
        if (!rd32(menu, &vt) || vt != nvbots::addr::MessageMenuVtbl) return false;
        if (!rd32(vt + nvbots::addr::kMenuSlotHandleClick, &fn) || fn != nvbots::addr::MsgMenuHandleClick.addr) return false;
        unsigned r = 0;
        return GuardedThis<unsigned>(&r, fn, reinterpret_cast<void*>(menu), id, (unsigned)tile);
    }
    // ---- bot.menu.close ----
    bool closeAllMenus() override {
        if (!g_callsOk || !onMainThread()) return false;
        __try {
            reinterpret_cast<void(__cdecl*)()>(nvbots::addr::CloseAllMenusCore.addr)();
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
        return true;
    }
    // ---- bot.vats ----
    nvbots::Addr32 menuByType(uint32_t id) override {
        if (!g_callsOk) return 0;
        unsigned r = 0;
        __try {
            r = reinterpret_cast<unsigned(__cdecl*)(int)>(nvbots::addr::GetMenuByType.addr)((int)id);
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return 0;
        }
        return r;
    }
    bool closeTutorial() override {
        if (!g_callsOk || !onMainThread()) return false;
        __try {
            reinterpret_cast<void(__cdecl*)()>(nvbots::addr::TutorialClose.addr)();
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
        return true;
    }
    // VATSMenu vtable slot 3 = HandleClick(id, tile). The logic layer already checked the vtable. Check it again here together with the
    // slot content, so this call can only reach the pinned 0x7EECB0 on a VATSMenu, on the main thread.
    bool vatsMenuClick(nvbots::Addr32 menu, int id) override {
        if (!g_callsOk || !onMainThread() || !menu) return false;
        uint32_t vt = 0, fn = 0;
        if (!rd32(menu, &vt) || vt != nvbots::addr::VatsMenuVtbl) return false;
        if (!rd32(vt + nvbots::addr::kMenuSlotHandleClick, &fn) || fn != nvbots::addr::VatsMenuHandleClick.addr) return false;
        unsigned r = 0;
        return GuardedThis<unsigned>(&r, fn, reinterpret_cast<void*>(menu), id, 0u);
    }
    int vatsQueueCount() override {
        int r = 0;   // fastcall(ECX = list head) with no stack arguments: a thiscall with no arguments
        return g_callsOk && GuardedThis<int>(&r, nvbots::addr::VatsQueueCount.addr, reinterpret_cast<void*>(nvbots::eng::kVatsCamera)) ? r : -1;
    }
    bool vatsExecute() override {
        if (!g_callsOk || !onMainThread()) return false;
        __try {
            reinterpret_cast<void(__stdcall*)()>(nvbots::addr::VatsExecute.addr)();
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
        return true;
    }
    bool vatsArmAccept(bool on) override {
        if (!on) { g_accept.disarm(); return true; }
        if (!g_acceptHooked || !g_callsOk || !onMainThread()) return false;
        const int n = vatsQueueCount();
        if (n < 0) return false;
        g_accept.arm(n);
        return true;
    }
    nvbots::Addr32 containerData(nvbots::Addr32 ref) override {
        if (!g_callsOk) return 0;
        unsigned r = 0;
        __try {
            r = reinterpret_cast<unsigned(__cdecl*)(unsigned)>(nvbots::addr::GetContainerData.addr)(ref);
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return 0;
        }
        return r;
    }
    int itemTotal(nvbots::Addr32 data, nvbots::Addr32 form) override {
        int r = 0;
        return g_callsOk && GuardedThis<int>(&r, nvbots::addr::ItemTotal.addr, reinterpret_cast<void*>(data), (unsigned)form) ? r : 0;
    }
    bool resolveRef(const std::string& ref, nvbots::Addr32* out) override {
        void* obj = nullptr;
        if (ref.empty()) {
            obj = nvhooks::resolver().lookup(nvhooks::kHostPlayerRef);
        } else {
            fnvcoop::FormRef r;
            if (!nvhooks::strToRef(ref, &r)) return false;
            const uint32_t id = nvhooks::resolver().fromRef(r);
            obj = id ? nvhooks::resolver().lookup(id) : nullptr;
        }
        *out = reinterpret_cast<nvbots::Addr32>(obj);
        return obj != nullptr;
    }
    bool resolveForm(const std::string& form, nvbots::Addr32* out) override { return resolveRef(form, out) && !form.empty(); }
    bool resolveFormId(const std::string& form, uint32_t* refId) override {
        fnvcoop::FormRef r;
        if (!nvhooks::strToRef(form, &r)) return false;
        *refId = nvhooks::resolver().fromRef(r);
        return *refId != 0;
    }
    std::string nameOfForm(nvbots::Addr32 form) override {
        uint32_t id = 0;
        if (!form || !rd32(form + nvbots::eng::kFormId, &id)) return "";
        fnvcoop::FormRef r;
        if (!nvhooks::resolver().toRef(id, &r)) { char b[16]; std::snprintf(b, sizeof b, "%08X", id); return b; }
        return nvhooks::refToStr(r);
    }
};
LiveGame g_game;

// The VATS attack-button test 0x7EEB50(slot, held, binding), cdecl. While a bot accept is armed, the slot 1 press edge answers true, so
// VATSMenu::Update queues the attack through its own path. Every other call goes to the original.
using Fn_VatsAttackButton = bool(__cdecl*)(int, char, int);
Fn_VatsAttackButton o_VatsAttackButton = nullptr;

bool __cdecl h_VatsAttackButton(int slot, char held, int binding) {
    if (g_accept.armed() && g_accept.force(slot, (int)(unsigned char)held, g_game.vatsQueueCount())) return true;
    return o_VatsAttackButton(slot, held, binding);
}

// ---- OnDrop: xNVSE's event manager hands a native handler (dropper, droppedRef) when anything is dropped ----
NVSEInterface* g_nvse = nullptr;
NVSEEventManagerInterface* g_events = nullptr;
bool g_dropHandler = false;

void OnDropEvent(TESObjectREFR*, void* parameters) {
    if (!g_rpc || !parameters) return;
    void** a = static_cast<void**>(parameters);
    const auto dropper = reinterpret_cast<nvbots::Addr32>(a[0]), dropped = reinterpret_cast<nvbots::Addr32>(a[1]);
    if (!dropped) return;
    uint32_t base = 0;
    g_game.rd32(dropped + 0x20, &base);   // TESObjectREFR::baseForm
    g_rpc->drops().record(g_game.nameOfForm(dropped), g_game.nameOfForm(base), g_game.nameOfForm(dropper), GetTickCount64());
}

void InstallDropHandler() {
    if (g_dropHandler || !g_nvse) return;
    g_events = static_cast<NVSEEventManagerInterface*>(g_nvse->QueryInterface(kInterface_EventManager));
    g_dropHandler = g_events && g_events->SetNativeEventHandler && g_events->SetNativeEventHandler("OnDrop", &OnDropEvent);
    if (g_rpc) g_rpc->setDropsLive(g_dropHandler);
    nvcoop::Log("bots: OnDrop native handler: %s", g_dropHandler ? "registered" : "unavailable (bot.drops unsupported, nearby-ref fallback used)");
}

// The always-run reads (bot.move run = speed) need only the kRunPins code to match; they never call the engine.
bool g_runPinsOk = false;

void CheckPins() {
    g_callsOk = true;
    for (const nvbots::addr::CallPin* p : nvbots::addr::kAllCalls) {
        unsigned char got[16] = {};
        bool ok = g_mem.read(p->addr, got, 16);
        if (ok && std::memcmp(got, p->bytes, 16) != 0) {
            // another module of ours may have detoured this entry already (world_time on PendingTravelSet): compare its trampoline instead
            unsigned char tr[16] = {};
            const nvhooks::Addr t = nvhooks::trampolineOfAny(p->addr);
            const bool haveTramp = t && g_mem.read(t, tr, 16);
            ok = nvbots::pinBytesOk(got, p->bytes, haveTramp ? tr : nullptr);
            if (!ok) {
                if (const nvbots::addr::KnownRewrite* r = nvbots::knownRewrite(p->addr, got)) {
                    nvcoop::Log("bots: engine pin %s @%08X holds a known rewrite by another plugin (%s): accepted", p->name, p->addr, r->what);
                    ok = true;
                }
            }
        }
        if (!ok) {
            char hex[16 * 3 + 1] = {};
            for (int i = 0; i < 16; ++i) std::snprintf(hex + i * 3, 4, "%02X ", got[i]);
            nvcoop::Log("bots: engine pin %s @%08X does not match this exe: dialogue/menu/inventory glue disabled; found %s", p->name,
                        p->addr, hex);
            g_callsOk = false;
        }
    }
    for (const nvbots::addr::DataPin* d : nvbots::addr::kAllData) {
        uint32_t got = 0;
        if (!g_mem.read(d->addr, &got, 4) || got != d->value) {
            nvcoop::Log("bots: data pin %s @%08X holds %08X, want %08X: dialogue/menu/inventory glue disabled", d->name, d->addr, got, d->value);
            g_callsOk = false;
        }
    }
    g_runPinsOk = true;
    for (const nvbots::addr::CodePin* p : nvbots::addr::kRunPins) {
        unsigned char got[16] = {};
        if (!g_mem.read(p->addr, got, 16) || std::memcmp(got, p->bytes, 16) != 0) {
            nvcoop::Log("bots: always-run pin %s @%08X does not match this exe: bot.move holds Run for run:true (alwaysRun unknown)", p->name, p->addr);
            g_runPinsOk = false;
        }
    }
}

struct LiveEngine : nvbots::IBotEngine {
    uint64_t nowMs() override { return GetTickCount64(); }
    bool exec(const std::string& line) override {
        static const char kPlayer[] = "player.";
        if (line.compare(0, sizeof kPlayer - 1, kPlayer) == 0) return nvhooks::execConsoleLineOn(line.substr(sizeof kPlayer - 1), 0x14);
        return nvhooks::execConsoleLine(line);
    }
    bool pos(const std::string& ref, double* x, double* y, double* z) override {
        void* obj = nullptr;
        if (ref.empty()) {
            obj = nvhooks::resolver().lookup(nvhooks::kHostPlayerRef);
        } else {
            fnvcoop::FormRef r;
            if (!nvhooks::strToRef(ref, &r)) return false;
            const uint32_t id = nvhooks::resolver().fromRef(r);
            obj = id ? nvhooks::resolver().lookup(id) : nullptr;
        }
        if (!obj) return false;
        auto* t = static_cast<TESObjectREFR*>(obj);
        *x = t->posX; *y = t->posY; *z = t->posZ;
        return true;
    }
    bool hooked() override { return g_hooked; }
    bool testHelpers() override {
        const char* gate = std::getenv("NVCOOP_TEST_HELPERS");
        return gate && gate[0] == '1';
    }
    nvbots::IEngine* engine() override { return g_callsOk ? &g_game : nullptr; }
    int alwaysRun() override { return g_runPinsOk ? nvbots::alwaysRunState(g_game) : -1; }   // SEH-guarded byte reads only
    bool pressBox(int button, uint32_t* menuId, std::string* err) override {
        nvcoop::registry::MethodEntry me;
        if (!nvcoop::registry::FindMethod("ui.answer", me)) { *err = "unsupported: hooks_ui (ui.answer) is not loaded"; return false; }
        std::string out;
        if (!me.fn("{\"button\":" + std::to_string(button) + "}", out)) { *err = out; return false; }
        nvcoop::json::Value v;
        std::string perr;
        *menuId = nvcoop::json::parse(out, v, perr) ? (uint32_t)v.get("menu").asInt(0) : 0;
        return true;
    }
    bool moveTo(const std::string& ref, std::string* err) override {
        fnvcoop::FormRef r;
        const uint32_t id = nvhooks::strToRef(ref, &r) ? nvhooks::resolver().fromRef(r) : 0;
        if (!id) { *err = "marker not loaded"; return false; }
        char line[64];
        std::snprintf(line, sizeof line, "MoveTo %08X", id);
        if (!nvhooks::execConsoleLineOn(line, 0x14)) { *err = "MoveTo refused by the console"; return false; }
        return true;
    }
};
LiveEngine g_engine;

void InstallHook() {
    if (g_hooked) return;
    if (!g_reg) g_reg = std::make_unique<nvhooks::HookRegistry>(g_mem);
    nvhooks::Addr tramp = 0;
    const nvhooks::PatchStatus st = g_reg->acquire(nvbots::addr::GetControlState, reinterpret_cast<nvhooks::Addr>(&h_GetControlState), &tramp);
    if (nvhooks::installed(st)) {
        o_GetControlState = reinterpret_cast<Fn_GetControlState>(tramp);
        g_hooked = true;
    }
    nvcoop::Log("bots: GetControlState hook: %s", nvhooks::statusName(st));
    // bot.vats accept: only with the engine call pins matched (the detour counts the queue through the pinned 0x5AE380)
    if (!g_acceptHooked && g_callsOk) {
        nvhooks::Addr t2 = 0;
        const nvhooks::PatchStatus s2 = g_reg->acquire(nvbots::addr::VatsAttackButton, reinterpret_cast<nvhooks::Addr>(&h_VatsAttackButton), &t2);
        if (nvhooks::installed(s2)) {
            o_VatsAttackButton = reinterpret_cast<Fn_VatsAttackButton>(t2);
            g_acceptHooked = true;
        }
        nvcoop::Log("bots: VATS attack-button hook: %s", nvhooks::statusName(s2));
    }
}

void Uninstall() {
    g_in.releaseAll();
    if (g_dropHandler && g_events && g_events->RemoveNativeEventHandler) g_events->RemoveNativeEventHandler("OnDrop", &OnDropEvent);
    g_dropHandler = false;
    g_accept.disarm();
    g_reg.reset();
    g_hooked = false;
    o_GetControlState = nullptr;
    g_acceptHooked = false;
    o_VatsAttackButton = nullptr;
    g_rpc.reset();
    g_inited = false;
}

void Init() {
    if (g_inited) return;
    g_inited = true;
    g_mainThread = GetCurrentThreadId();   // DeferredInit runs on the game main thread; OnFrame keeps this current
    g_enabled = GetPrivateProfileIntA("dev", "bots", 0, IniPath().c_str()) != 0;
    if (!g_enabled) return;
    CheckPins();
    g_rpc = std::make_unique<nvbots::BotRpc>(g_engine, g_in);
    InstallHook();
    InstallDropHandler();
    static const char* const kMethods[] = {"bot.status", "bot.move", "bot.stop", "bot.look", "bot.fire", "bot.reload", "bot.sneak",
                                           "bot.jump", "bot.activate", "bot.input", "bot.menu", "bot.menu.press", "bot.menu.close", "bot.consent", "bot.inventory",
                                           "bot.say", "bot.dialog", "bot.vats", "test.vats.hit", "test.fasttravel", "bot.fasttravel", "bot.drops", "dev.refsOf", "dev.countRefs",
                                           "dev.terminal", "dev.terminalState"};
    for (const char* m : kMethods) {
        const std::string name = m;
        nvcoop::RegisterMethodClosure(m, [name](const std::string& params, std::string& out) {
            if (!g_rpc) { out = "bots disabled"; return false; }
            return g_rpc->handle(name, params, out);
        }, true, 0);
    }
    nvcoop::Log("bots: enabled (dev), hooked=%d vatsAccept=%d pins=%d runPins=%d", g_hooked ? 1 : 0, g_acceptHooked ? 1 : 0, g_callsOk ? 1 : 0, g_runPinsOk ? 1 : 0);
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_DeferredInit) Init();
    else if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) Uninstall();
}
void OnFrame() {
    g_mainThread = GetCurrentThreadId();   // the main game loop
    if (g_rpc) g_rpc->tick();
}

void OnLoad(void* nvse) { g_nvse = static_cast<NVSEInterface*>(nvse); }

}  // namespace

NVCOOP_MODULE(t6a_bots, "t6a_bots", OnLoad, OnMessage, OnFrame);

// Hot reload (T3b): a core DLL being replaced releases every forced control and restores the original GetControlState bytes.
NVCOOP_RELOAD_STATE(t6a_bots, "t6a_bots", 1, 30,
    /*canUnload*/ [] {
        const int n = g_reg ? g_reg->auditTampered() : 0;
        return n ? std::string("the input hook was chained over by another mod") : std::string();
    },
    /*save*/ [](nvreload::Writer& w) {
        w.boolean(g_enabled);
        return true;
    },
    /*restore*/ [](uint32_t, nvreload::Reader& r) {
        if (r.boolean()) Init();
        return true;
    },
    /*migrate*/ nullptr,
    /*shutdown*/ [] { if (g_inited) Uninstall(); },
    /*fresh*/ nullptr);
