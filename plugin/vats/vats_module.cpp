// nvcoop module for real-time VATS (x86, compiled into nvcoop.dll next to the coop module). NOTHING here has run in a game. Inert unless nvcoop.ini
// [vats] enabled=1.
//
//   * the HOOKS: call-site redirects (vats_patch.h) for the three things that stop or slow a shared world in vanilla VATS (docs/re/vats-realtime.md):
//       - MenuMode() in the main loop (2 sites, optionally 9 more)            -> menuModeForWorld()
//       - TimeGlobal::SetTimeScale from the VATS camera code (11 sites)       -> scaleForVatsRequest()
//       - GetPlayerTimeMult / GetTargetTimeMult (5 + 9 sites)                  -> timeMultFor()
//     and, because the world then runs under an open menu, the InterfaceManager update moves from the worker thread to the main thread (the worker kick
//     0x86EC87 and the worker's stage 0, 2 sites)                              -> interfaceUpdateOnMain() / InterfaceHandoff
//     Every thunk calls the real function first, so the engine's side effects stay, and only changes the answer; with no co-op session the policy
//     returns the engine's answer unchanged.
//   * LiveEngine: the IVatsEngine over the static RE of docs/re/vats-realtime.md (all reads and calls SEH-guarded; main thread only).
//   * the session frame: OnFrame builds the FrameInfo from the active coop Runtime and feeds VatsRuntime (the game-free logic).
//   * bridge methods: vats.status / vats.state / vats.select_limb / vats.select_target / vats.fire / vats.cancel (docs/tools/vats.md).
// Switches in nvcoop.ini [vats]: enabled=0 realtime=1 partyslowmo=0 slowscale=0.35 slowmaxms=3000 slowcooldownms=10000 autofire=1 autofiredelay=350
// killcam=1 guard=1 perks=1 pausesites=1 grs_ap=20 grs_all=0 grs_edid=GrimReapersSprint.
#include <Windows.h>

#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "GameForms.h"
#include "GameObjects.h"
#include "PluginAPI.h"
#include "actors/engine/actor_engine_api.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "coop_runtime.h"
#include "engine_hooks.h"
#include "hook_patch.h"
#include "live/live_glue.h"
#include "nvmp/ghost_ref.h"
#include "reload/reload_state.h"
#include "vats/vats_addrs.h"
#include "vats/vats_api.h"
#include "vats/vats_patch.h"
#include "vats/vats_runtime.h"

namespace vt = nvcoop::vats;
namespace ad = nvcoop::vats::addr;
namespace eng = nvcoop::actors::engine;

namespace {

// ---------------------------------------------------------------------------------------------------------------- raw memory
struct Win32Memory : nvhooks::MemoryOps {
    bool read(nvhooks::Addr a, void* out, size_t n) override {
        __try {
            std::memcpy(out, reinterpret_cast<const void*>(static_cast<uintptr_t>(a)), n);
            return true;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
    }
    bool write(nvhooks::Addr a, const void* in, size_t n) override {
        DWORD old = 0;
        void* p = reinterpret_cast<void*>(static_cast<uintptr_t>(a));
        if (!VirtualProtect(p, n, PAGE_EXECUTE_READWRITE, &old)) return false;
        std::memcpy(p, in, n);
        FlushInstructionCache(GetCurrentProcess(), p, n);
        DWORD tmp;
        VirtualProtect(p, n, old, &tmp);
        return true;
    }
    nvhooks::Addr allocExec(size_t) override { return 0; }
    void freeExec(nvhooks::Addr, size_t) override {}
};
Win32Memory g_mem;

// One small function per access so __try never meets a C++ object.
bool Rd32(uint32_t a, uint32_t* out) {
    __try {
        *out = *reinterpret_cast<const uint32_t*>(static_cast<uintptr_t>(a));
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool RdF(uint32_t a, float* out) {
    __try {
        *out = *reinterpret_cast<const float*>(static_cast<uintptr_t>(a));
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool Wr32(uint32_t a, uint32_t v) {
    __try {
        *reinterpret_cast<uint32_t*>(static_cast<uintptr_t>(a)) = v;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool WrF(uint32_t a, float v) {
    __try {
        *reinterpret_cast<float*>(static_cast<uintptr_t>(a)) = v;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
uint32_t FormIdOf(uint32_t formPtr) {   // TESForm::refID
    uint32_t id = 0;
    return formPtr && Rd32(formPtr + 0x0C, &id) ? id : 0;
}

uint64_t NowMs() { return static_cast<uint64_t>(GetTickCount64()); }

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    const size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

// ---------------------------------------------------------------------------------------------------------------- state
struct ModuleSettings {
    bool enabled = false;
    int pauseSites = 1;              // 0 none, 1 main loop (2 sites), 2 main loop + 9 more
    vt::Config cfg;
};
ModuleSettings g_set;
bool g_inited = false;
volatile DWORD g_mainThread = 0;

struct Pins { bool setScale = false, getMult = false, menuMode = false, menuModeOf = false, accept = false, unclick = false, limb = false, handleClick = false, getForm = false; };
Pins g_pins;
uint64_t g_thunkScale = 0, g_thunkScaleChanged = 0, g_thunkMult = 0, g_thunkMultChanged = 0, g_thunkPause = 0, g_thunkPauseChanged = 0, g_thunkOffThread = 0, g_thunkErrors = 0;
uint64_t g_calls = 0, g_callsFailed = 0, g_pinRefusals = 0;

std::unique_ptr<vt::VatsRuntime> g_rt;
vt::VatsSync* Sync() { return g_rt ? g_rt->sync() : nullptr; }

const uint8_t kGetFormPrologue[16] = {0x55, 0x8B, 0xEC, 0x51, 0xC7, 0x45, 0xFC, 0x00, 0x00, 0x00, 0x00, 0x83, 0x7D, 0x08, 0x00, 0x74};
constexpr uint32_t kGetFormByEdid = 0x00483A00;   // xNVSE GetFormByID(const char* editorID), cdecl

bool PinOk(uint32_t fn, const uint8_t* bytes) {
    uint8_t got[16] = {};
    return g_mem.read(fn, got, 16) && std::memcmp(got, bytes, 16) == 0;
}

// ---------------------------------------------------------------------------------------------------------------- guarded engine calls
using SetScaleFn = void(__thiscall*)(void* self, float scale, uint32_t immediate);
using HandleClickFn = void(__thiscall*)(void* menu, int tile, void* tilePtr);
using UnclickFn = void(__thiscall*)(void* cam, uint32_t* actionType, uint8_t* byteOut);
using LimbOfPartFn = int(__thiscall*)(void* menu, int bodyPart);
using MenuModeOfFn = char(__cdecl*)(int id);
using GetFormFn = void*(__cdecl*)(const char* edid);

bool CallSetScale(float scale, bool immediate) {
    __try {
        reinterpret_cast<SetScaleFn>(static_cast<uintptr_t>(ad::kSetTimeScale))(reinterpret_cast<void*>(static_cast<uintptr_t>(ad::kTimeGlobal)), scale, immediate ? 1u : 0u);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallAccept() {
    __try {
        reinterpret_cast<void(__stdcall*)()>(static_cast<uintptr_t>(ad::kAccept))();
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallUnclick() {
    __try {
        uint32_t action = 0;
        uint8_t b = 0;
        reinterpret_cast<UnclickFn>(static_cast<uintptr_t>(ad::kUnclick))(reinterpret_cast<void*>(static_cast<uintptr_t>(ad::kVatsCamera)), &action, &b);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallHandleClick(uint32_t menu, uint32_t fn, int tile) {
    __try {
        reinterpret_cast<HandleClickFn>(static_cast<uintptr_t>(fn))(reinterpret_cast<void*>(static_cast<uintptr_t>(menu)), tile, nullptr);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallLimbOfPart(uint32_t menu, int bodyPart, int* out) {
    __try {
        *out = reinterpret_cast<LimbOfPartFn>(static_cast<uintptr_t>(ad::kLimbOfPart))(reinterpret_cast<void*>(static_cast<uintptr_t>(menu)), bodyPart);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallMenuModeOf(int id, bool* out) {
    __try {
        *out = reinterpret_cast<MenuModeOfFn>(static_cast<uintptr_t>(ad::kMenuModeOf))(id) != 0;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool ReadAp(int* cur, int* mx) {
    __try {
        Actor* a = *reinterpret_cast<Actor**>(static_cast<uintptr_t>(ad::kPlayerPtr));
        if (!a) return false;
        ActorValueOwner* o = &a->avOwner;
        *cur = static_cast<int>(o->Fn_03(12) + 0.5f);   // eActorVal_ActionPoints
        *mx = static_cast<int>(o->Fn_01(12) + 0.5f);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
void* LookupFormByEdid(const char* edid) {
    __try {
        return reinterpret_cast<GetFormFn>(static_cast<uintptr_t>(kGetFormByEdid))(edid);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}
int PerkRankOfPlayer(void* perkForm) {
    __try {
        Actor* a = *reinterpret_cast<Actor**>(static_cast<uintptr_t>(ad::kPlayerPtr));
        if (!a || !perkForm) return -1;
        TESForm* f = static_cast<TESForm*>(perkForm);
        if (f->typeID != kFormType_BGSPerk) return -2;
        return a->GetPerkRank(static_cast<BGSPerk*>(perkForm), false);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -3;
    }
}

// ---------------------------------------------------------------------------------------------------------------- the engine port
class LiveEngine : public vt::IVatsEngine {
public:
    vt::Mode mode() override {
        uint32_t m = 0;
        return Rd32(ad::kVatsCamera + ad::kCamMode, &m) ? vt::modeFromEngine(m) : vt::Mode::None;
    }
    bool vatsMenuOpen() override {
        if (mode() == vt::Mode::None) return false;
        uint32_t menu = 0;
        if (!Rd32(ad::kVatsMenuPtr, &menu) || !menu) return false;
        bool top = false;
        if (g_pins.menuModeOf && CallMenuModeOf(ad::kVatsMenuId, &top)) return top;
        return true;   // pins differ: a VATS mode with a VATS menu object is taken as "the VATS menu is open"
    }
    uint32_t selectedTargetRef() override {
        uint32_t actor = 0;
        return Rd32(ad::kSelectedTarget, &actor) ? FormIdOf(actor) : 0;
    }
    std::vector<vt::TargetInfo> targets() override {
        std::vector<vt::TargetInfo> out;
        if (mode() == vt::Mode::None) return out;
        uint32_t selActor = 0, menu = 0;
        Rd32(ad::kSelectedTarget, &selActor);
        Rd32(ad::kVatsMenuPtr, &menu);
        uint32_t node = ad::kTargetListHead;
        for (int i = 0; i < 32 && node; ++i) {
            uint32_t rec = 0, next = 0;
            if (!Rd32(node, &rec) || !Rd32(node + 4, &next)) break;
            if (rec) {
                vt::TargetInfo t;
                t.rec = rec;
                uint32_t actor = 0;
                Rd32(rec + ad::kRecActor, &actor);
                t.actorRef = FormIdOf(actor);
                Rd32(rec + ad::kRecKind, &t.kind);
                const bool isSelected = actor != 0 && actor == selActor;
                uint32_t pn = rec + ad::kRecParts;
                for (unsigned j = 0; j < vt::kLimbMax * 2 && pn; ++j) {
                    uint32_t part = 0, pnext = 0;
                    if (!Rd32(pn, &part) || !Rd32(pn + 4, &pnext)) break;
                    if (part) {
                        vt::PartInfo p;
                        p.rec = part;
                        uint32_t body = 0;
                        if (Rd32(part + ad::kPartBodyIndex, &body) && body < 16) {
                            p.bodyIndex = static_cast<int32_t>(body);
                            // LimbOfPart asks the SELECTED actor's body-part data, so it is only meaningful for the selected target
                            int limb = -1;
                            if (isSelected && menu && g_pins.limb && CallLimbOfPart(menu, static_cast<int>(body), &limb)) p.limb = vt::limbFromAv(static_cast<int8_t>(limb));
                        }
                        t.parts.push_back(p);
                    }
                    pn = pnext;
                }
                out.push_back(std::move(t));
            }
            node = next;
        }
        return out;
    }
    uint32_t highlightedPartRec() override {
        uint32_t menu = 0, part = 0;
        return Rd32(ad::kVatsMenuPtr, &menu) && menu && Rd32(menu + ad::kMenuHighlightPart, &part) ? part : 0;
    }
    vt::LimbId rememberedLimb() override {
        uint32_t v = 0;
        return Rd32(ad::kRememberedLimb, &v) ? vt::limbFromAv(static_cast<int>(v)) : vt::LimbId::None;
    }
    int queuedCount() override {
        uint32_t node = 0;
        if (!Rd32(ad::kVatsCamera + ad::kCamTargets, &node)) return 0;
        int n = 0;
        while (node && n < 32) {
            uint32_t data = 0, next = 0;
            if (!Rd32(node, &data) || !Rd32(node + 4, &next)) break;
            if (data) ++n;
            node = next;
        }
        return n;
    }
    float timeScaleCurrent() override { float v = 1.0f; RdF(ad::kTimeScaleCurrent, &v); return v; }
    float timeScaleTarget() override { float v = 1.0f; RdF(ad::kTimeScaleTarget, &v); return v; }
    uint32_t localPlayerRef() override { return eng::kLocalPlayerRef; }
    uint32_t netIdOfRef(uint32_t ref) override { return ref ? eng::netIdOf(ref) : 0; }
    int actionPoints() override { int c = -1, m = -1; return ReadAp(&c, &m) ? c : -1; }
    int maxActionPoints() override { int c = -1, m = -1; return ReadAp(&c, &m) ? m : -1; }
    bool playerHasPerk(const std::string& edid) override {
        if (!g_pins.getForm) return false;
        void* form = nullptr;
        auto it = perkCache_.find(edid);
        if (it != perkCache_.end()) form = reinterpret_cast<void*>(static_cast<uintptr_t>(it->second));
        if (!form) {
            form = LookupFormByEdid(edid.c_str());
            if (form) perkCache_[edid] = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(form));
        }
        if (!form) {
            if (!perkLookupLogged_) { perkLookupLogged_ = true; nvcoop::Log("vats: perk '%s' was not found by editor id: the Grim Reaper's Sprint refund is off (set grs_edid)", edid.c_str()); }
            return false;
        }
        return PerkRankOfPlayer(form) > 0;
    }
    bool isPartyRef(uint32_t ref) override {
        if (!ref) return false;
        for (uint8_t s = 1; s < fnvcoop::kMaxSlots; ++s) {
            if (nvcoop::livewire::glueAvatarRef(s) == ref) return true;
            if (nvmp::GhostRefOfPlayer(s) == ref) return true;
        }
        return false;
    }

    bool commitHighlight(uint32_t partRec, vt::LimbId limb) override {
        uint32_t menu = 0;
        if (!Rd32(ad::kVatsMenuPtr, &menu) || !menu || !partRec) return fail();
        // only a part record that the engine's own list holds right now may be written
        bool inList = false;
        for (const vt::TargetInfo& t : targets())
            if (t.actorRef == selectedTargetRef())
                for (const vt::PartInfo& p : t.parts) inList = inList || p.rec == partRec;
        if (!inList) return fail();
        const bool ok = Wr32(menu + ad::kMenuHighlightPart, partRec) && Wr32(ad::kRememberedLimb, static_cast<uint32_t>(static_cast<int>(limb)));
        ++g_calls;
        return ok || fail();
    }
    bool cycleTarget(bool forward) override {
        uint32_t menu = 0, vtbl = 0, fn = 0;
        if (!Rd32(ad::kVatsMenuPtr, &menu) || !menu || !Rd32(menu, &vtbl) || vtbl != ad::kMenuVtable) return fail();
        if (!Rd32(vtbl + 4 * ad::kMenuVtblHandleClick, &fn) || fn != ad::kMenuHandleClickFn || !g_pins.handleClick) return refuse();
        ++g_calls;
        return CallHandleClick(menu, fn, forward ? ad::kTileNextTarget : ad::kTilePrevTarget) || fail();
    }
    bool accept() override {
        if (!g_pins.accept) return refuse();
        ++g_calls;
        return CallAccept() || fail();
    }
    bool unclickLast() override {
        if (!g_pins.unclick) return refuse();
        ++g_calls;
        return CallUnclick() || fail();
    }
    bool setTimeScale(float scale, bool immediate) override {
        if (!g_pins.setScale) return refuse();
        ++g_calls;
        return CallSetScale(scale, immediate) || fail();
    }
    bool restoreActionPoints(int amount) override {
        if (amount <= 0) return false;
        char line[64];
        std::snprintf(line, sizeof line, "RestoreActorValue ActionPoints %d", amount);
        nvhooks::ApplyScope scope;   // our own console line must not be dropped by the guest gate
        ++g_calls;
        return nvhooks::execConsoleLineOn(line, eng::kLocalPlayerRef) || fail();
    }
    bool applyCameraTimes(bool shortened) override {
        const ad::GameSetting* gs[2] = {&ad::kSettingCamMinTime, &ad::kSettingCamMaxTime};
        const float shortValues[2] = {0.0f, 0.3f};
        if (shortened) {
            for (int i = 0; i < 2; ++i) {
                float cur = 0;
                if (!RdF(gs[i]->object + ad::kSettingValueOffset, &cur)) return fail();
                if (!camSaved_) saved_[i] = cur;
            }
            camSaved_ = true;
            for (int i = 0; i < 2; ++i) if (!WrF(gs[i]->object + ad::kSettingValueOffset, shortValues[i])) return fail();
            return true;
        }
        if (!camSaved_) return true;
        for (int i = 0; i < 2; ++i) WrF(gs[i]->object + ad::kSettingValueOffset, saved_[i]);
        camSaved_ = false;
        return true;
    }

    // the multiplier settings fallback (used when the multiplier call sites could not be patched)
    void setMultSettings(bool on) {
        const ad::GameSetting* gs[2] = {&ad::kSettingPlayerTimeMult, &ad::kSettingTargetTimeMult};
        for (int i = 0; i < 2; ++i) {
            const uint32_t a = gs[i]->object + ad::kSettingValueOffset;
            if (on) {
                if (!multSaved_) { RdF(a, &multOrig_[i]); }
                WrF(a, 1.0f);
            } else if (multSaved_) {
                WrF(a, multOrig_[i]);
            }
        }
        multSaved_ = on;
    }
    bool multSettingsActive() const { return multSaved_; }

private:
    bool fail() { ++g_callsFailed; return false; }
    bool refuse() { ++g_pinRefusals; return false; }
    std::map<std::string, uint32_t> perkCache_;
    bool perkLookupLogged_ = false;
    float saved_[2] = {0, 0};
    bool camSaved_ = false;
    float multOrig_[2] = {3.0f, 1.0f};
    bool multSaved_ = false;
};
LiveEngine g_engine;

// ---------------------------------------------------------------------------------------------------------------- the hooks
struct HookGroup {
    const char* name = "";
    const ad::Site* sites = nullptr;
    size_t count = 0;
    uint32_t target = 0;
    const uint8_t* prologue = nullptr;
    uint32_t thunk = 0;
    std::unique_ptr<vt::SitePatcher> patcher;
};
HookGroup g_groups[7];
bool g_hooksTried = false;

bool OnMainThread() { return g_mainThread == 0 || GetCurrentThreadId() == g_mainThread; }

// TimeGlobal::SetTimeScale(this, float scale, uint32 immediate), thiscall, ret 8. Fastcall with a dummy edx reproduces that stack layout.
void __fastcall SetScaleThunk(void* self, void*, float scale, uint32_t immediate) {
    float use = scale;
    vt::VatsSync* s = Sync();
    if (s && OnMainThread()) {
        try {
            use = vt::scaleForVatsRequest(s->policyInput(), scale);
        } catch (...) {
            ++g_thunkErrors;
            use = scale;
        }
    } else if (s) {
        ++g_thunkOffThread;
    }
    ++g_thunkScale;
    if (use != scale) ++g_thunkScaleChanged;
    reinterpret_cast<SetScaleFn>(static_cast<uintptr_t>(ad::kSetTimeScale))(self, use, immediate);
}

using MultFn = double(__thiscall*)(void* cam);
double __fastcall PlayerMultThunk(void* cam) {
    double r = reinterpret_cast<MultFn>(static_cast<uintptr_t>(ad::kGetPlayerTimeMult))(cam);
    vt::VatsSync* s = Sync();
    if (s && OnMainThread()) {
        try {
            const double use = vt::timeMultFor(s->policyInput(), r);
            if (use != r) ++g_thunkMultChanged;
            r = use;
        } catch (...) {
            ++g_thunkErrors;
        }
    }
    ++g_thunkMult;
    return r;
}
double __fastcall TargetMultThunk(void* cam) {
    double r = reinterpret_cast<MultFn>(static_cast<uintptr_t>(ad::kGetTargetTimeMult))(cam);
    vt::VatsSync* s = Sync();
    if (s && OnMainThread()) {
        try {
            const double use = vt::timeMultFor(s->policyInput(), r);
            if (use != r) ++g_thunkMultChanged;
            r = use;
        } catch (...) {
            ++g_thunkErrors;
        }
    }
    ++g_thunkMult;
    return r;
}

bool __cdecl MenuModeThunk() {
    bool r = reinterpret_cast<bool(__cdecl*)()>(static_cast<uintptr_t>(ad::kMenuMode))();
    vt::VatsSync* s = Sync();
    if (s && OnMainThread()) {
        try {
            const bool use = vt::menuModeForWorld(s->policyInput(), r);
            if (use != r) ++g_thunkPauseChanged;
            r = use;
        } catch (...) {
            ++g_thunkErrors;
        }
    }
    ++g_thunkPause;
    return r;
}

// The InterfaceManager update and the worker kick (vats_addrs.h: the root cause of the VATS-entry crash at 0x800F66 and of the earlier hang). Vanilla runs
// the update on the main thread whenever a menu is open, because an open menu pauses the world and the workers are not kicked. Real-time VATS kicks them
// under the open VATS menu; then the main thread runs the update itself just before the kick, and the worker's stage 0 skips it. The menu update's wait
// for the worker (0x713C8D) is left as the game has it: the worker signals that event at once.
using VoidThisFn = void(__thiscall*)(void* self);
vt::InterfaceHandoff g_imHandoff;
uint64_t g_imOnMain = 0, g_imWorkerSkips = 0;   // g_imWorkerSkips is written by the worker thread only
constexpr int kGroupWorkerUpdate = 5, kGroupKick = 6;

bool RealMenuMode() { return reinterpret_cast<bool(__cdecl*)()>(static_cast<uintptr_t>(ad::kMenuMode))(); }

// 0x8C78C0, the main loop's worker kick (call 0x86EC87): thiscall(manager), no arguments, plain ret.
void __fastcall WorkerKickThunk(void* mgr, void*) {
    bool wantMain = false;
    vt::VatsSync* s = Sync();
    if (s && OnMainThread()) {
        try {
            wantMain = vt::interfaceUpdateOnMain(s->policyInput(), RealMenuMode());
        } catch (...) {
            ++g_thunkErrors;
        }
    }
    const HookGroup& w = g_groups[kGroupWorkerUpdate];
    const bool workerPatched = w.patcher && w.patcher->installed() == static_cast<int>(w.count);
    static bool s_wasMain = false;
    const bool onMain = g_imHandoff.atKick(wantMain, workerPatched);   // decided before the workers are resumed, so the worker reads this frame's decision
    if (onMain != s_wasMain) {   // one line per edge (VATS opened / closed with the world running): the in-game proof of this path
        nvcoop::Log("vats: interface update on the %s thread (main-thread frames %llu, worker skips %llu)", onMain ? "main" : "worker",
                    (unsigned long long)g_imOnMain, (unsigned long long)g_imWorkerSkips);
        s_wasMain = onMain;
    }
    if (onMain) {
        reinterpret_cast<VoidThisFn>(static_cast<uintptr_t>(ad::kInterfaceUpdate))(*reinterpret_cast<void**>(static_cast<uintptr_t>(ad::kMainObjectPtr)));
        ++g_imOnMain;
    }
    reinterpret_cast<VoidThisFn>(static_cast<uintptr_t>(ad::kWorkerKick))(mgr);
}

// 0x86FD70 as the worker threads' stage 0 (0x8C7DB9, 0x8C7BE9): runs on the worker thread.
void __fastcall WorkerInterfaceThunk(void* self, void*) {
    if (!g_imHandoff.workerRuns()) {
        ++g_imWorkerSkips;
        return;
    }
    reinterpret_cast<VoidThisFn>(static_cast<uintptr_t>(ad::kInterfaceUpdate))(self);
}

uint32_t ThunkAddr(void* fn) { return static_cast<uint32_t>(reinterpret_cast<uintptr_t>(fn)); }

void InstallHooks() {
    if (g_hooksTried) return;
    g_hooksTried = true;
    g_groups[0] = {"time_scale", ad::kTimeScaleSites, ad::kTimeScaleSiteCount, ad::kSetTimeScale, ad::kSetTimeScalePrologue, ThunkAddr(reinterpret_cast<void*>(&SetScaleThunk)), nullptr};
    g_groups[1] = {"player_mult", ad::kPlayerMultSites, ad::kPlayerMultSiteCount, ad::kGetPlayerTimeMult, ad::kGetTimeMultPrologue, ThunkAddr(reinterpret_cast<void*>(&PlayerMultThunk)), nullptr};
    g_groups[2] = {"target_mult", ad::kTargetMultSites, ad::kTargetMultSiteCount, ad::kGetTargetTimeMult, ad::kGetTimeMultPrologue, ThunkAddr(reinterpret_cast<void*>(&TargetMultThunk)), nullptr};
    g_groups[3] = {"pause_main", ad::kPauseSitesMain, ad::kPauseSiteMainCount, ad::kMenuMode, ad::kMenuModePrologue, ThunkAddr(reinterpret_cast<void*>(&MenuModeThunk)), nullptr};
    g_groups[4] = {"pause_ext", ad::kPauseSitesExtended, ad::kPauseSiteExtendedCount, ad::kMenuMode, ad::kMenuModePrologue, ThunkAddr(reinterpret_cast<void*>(&MenuModeThunk)), nullptr};
    g_groups[kGroupWorkerUpdate] = {"worker_ui_update", ad::kWorkerInterfaceSites, ad::kWorkerInterfaceSiteCount, ad::kInterfaceUpdate, ad::kInterfaceUpdatePrologue,
                                    ThunkAddr(reinterpret_cast<void*>(&WorkerInterfaceThunk)), nullptr};
    g_groups[kGroupKick] = {"worker_kick", ad::kWorkerKickSites, ad::kWorkerKickSiteCount, ad::kWorkerKick, ad::kWorkerKickPrologue, ThunkAddr(reinterpret_cast<void*>(&WorkerKickThunk)), nullptr};
    g_imHandoff.reset();
    for (int i = 0; i < 7; ++i) {   // the worker sites (5) go in before the kick (6), whose thunk checks them
        HookGroup& g = g_groups[i];
        if ((i == 3 || i == kGroupWorkerUpdate || i == kGroupKick) && g_set.pauseSites < 1) continue;
        if (i == 4 && g_set.pauseSites < 2) continue;
        g.patcher.reset(new vt::SitePatcher(g_mem));
        const int n = g.patcher->install(g.sites, g.count, g.target, g.prologue, 16, g.thunk);
        for (const vt::SiteResult& r : g.patcher->results()) nvcoop::Log("vats: %s site %s @%08X: %s", g.name, r.name, r.site, vt::toString(r.status));
        if (!g.patcher->prologueOk()) nvcoop::Log("vats: %s: the target function does not match the pinned bytes (%s): NOT patched", g.name, g.patcher->lastError().c_str());
        nvcoop::Log("vats: %s: %d of %d call sites redirected", g.name, n, static_cast<int>(g.count));
    }
    // the multipliers could not be hooked: fall back to the two game settings (the playback 3x then becomes 1x without a hook)
    const bool multHooked = g_groups[1].patcher && g_groups[1].patcher->installed() > 0 && g_groups[2].patcher && g_groups[2].patcher->installed() > 0;
    if (!multHooked) {
        g_engine.setMultSettings(true);
        nvcoop::Log("vats: the time-multiplier call sites are not patched: fVATSPlayerTimeUpdateMult / fVATSTargetTimeUpdateMult set to 1.0 instead");
    }
}

void RemoveHooks() {
    // the kick first: from then on nobody hands the update to the main thread, and the worker (joined at the end of every frame) runs it again
    for (int i : {kGroupKick, kGroupWorkerUpdate}) {
        HookGroup& g = g_groups[i];
        if (!g.patcher) continue;
        const int restored = g.patcher->uninstall();
        nvcoop::Log("vats: %s removed (%d sites restored, %d left to another patcher)", g.name, restored, g.patcher->tampered());
        g.patcher.reset();
        if (i == kGroupKick) g_imHandoff.reset();
    }
    for (HookGroup& g : g_groups) {
        if (g.patcher) {
            const int restored = g.patcher->uninstall();
            nvcoop::Log("vats: %s removed (%d sites restored, %d left to another patcher)", g.name, restored, g.patcher->tampered());
            g.patcher.reset();
        }
    }
    if (g_engine.multSettingsActive()) g_engine.setMultSettings(false);
    g_hooksTried = false;
}

// ---------------------------------------------------------------------------------------------------------------- actor port events
eng::ListenerId g_diedL = 0;
void InstallListeners() {
    if (g_diedL) return;
    g_diedL = eng::addActorDiedListener([](const eng::DiedEvent& e) {
        if (vt::VatsSync* s = Sync()) s->onActorDied(e.ref, e.mine, e.fromNetwork, NowMs());
    });
}
void RemoveListeners() {
    if (!g_diedL) return;
    eng::removeListener(g_diedL);
    g_diedL = 0;
}

// ---------------------------------------------------------------------------------------------------------------- ini
// (xNVSE's headers define min() / max() macros, so std::min / std::max cannot be used in this file)
int ClampI(int v, int lo, int hi) { return v < lo ? lo : (v > hi ? hi : v); }

float IniFloat(const char* key, float def, const std::string& ini) {
    char buf[64] = {};
    char d[32];
    std::snprintf(d, sizeof d, "%g", def);
    GetPrivateProfileStringA("vats", key, d, buf, sizeof buf, ini.c_str());
    return static_cast<float>(std::atof(buf));
}

void ReadIni() {
    const std::string ini = IniPath();
    auto I = [&](const char* k, int d) { return GetPrivateProfileIntA("vats", k, d, ini.c_str()); };
    g_set.enabled = I("enabled", 0) != 0;
    vt::Settings s;
    s.realtime = I("realtime", 1) != 0;
    s.partySlowMo = I("partyslowmo", 0) != 0;
    s.slowScale = IniFloat("slowscale", 0.35f, ini);
    s.slowMaxMs = static_cast<uint32_t>(ClampI(I("slowmaxms", 3000), 0, 3600000));
    s.slowCooldownMs = static_cast<uint32_t>(ClampI(I("slowcooldownms", 10000), 0, 3600000));
    s.autoFire = I("autofire", 1) != 0;
    s.autoFireDelayMs = static_cast<uint32_t>(ClampI(I("autofiredelay", 350), 0, 3600000));
    s.killCam = I("killcam", 1) != 0;
    s.guardFriendly = I("guard", 1) != 0;
    s.perks = I("perks", 1) != 0;
    g_set.cfg.settings = vt::sanitized(s);
    g_set.pauseSites = ClampI(I("pausesites", 1), 0, 2);
    g_set.cfg.perks.grsApRestore = I("grs_ap", 20);
    g_set.cfg.perks.grsRestoreAll = I("grs_all", 0) != 0;
    char edid[96] = {};
    GetPrivateProfileStringA("vats", "grs_edid", "GrimReapersSprint", edid, sizeof edid, ini.c_str());
    if (edid[0]) g_set.cfg.perks.grsEditorId = edid;
}

// ---------------------------------------------------------------------------------------------------------------- bridge methods
std::string Esc(const std::string& s) {
    std::string o;
    for (char c : s) {
        if (c == '"' || c == '\\') { o.push_back('\\'); o.push_back(c); }
        else if (static_cast<unsigned char>(c) < 0x20) o.push_back(' ');
        else o.push_back(c);
    }
    return o;
}

std::string ResultJson(const vt::ActionResult& r) {
    return std::string("{\"ok\":") + (r.ok ? "true" : "false") + ",\"limb\":\"" + vt::toString(r.limb) + "\",\"limbAv\":" + std::to_string(static_cast<int>(r.limb)) +
           ",\"target\":" + std::to_string(r.targetRef) + ",\"reason\":\"" + Esc(r.reason) + "\"}";
}

uint32_t ParseRef(const nvcoop::json::Value& v) {
    if (v.isNumber()) return static_cast<uint32_t>(v.asInt(0));
    if (v.isString()) return static_cast<uint32_t>(std::strtoul(v.asString().c_str(), nullptr, 0));
    return 0;
}

bool ParseParams(const std::string& text, nvcoop::json::Value& out) {
    std::string err;
    if (text.empty() || text == "null") { out = nvcoop::json::Value::object(); return true; }
    return nvcoop::json::parse(text, out, err);
}

void RegisterBridge() {
    nvcoop::RegisterMethodClosure("vats.status", [](const std::string&, std::string& out) {
        out = nvcoop::vats::liveStatusJson();
        return true;
    }, true, 0);
    if (!g_set.enabled) return;
    nvcoop::RegisterMethodClosure("vats.state", [](const std::string&, std::string& out) {
        out = vt::toJson(nvcoop::vats::liveView());
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("vats.select_limb", [](const std::string& params, std::string& out) {
        nvcoop::json::Value p;
        if (!ParseParams(params, p)) { out = "bad params"; return false; }
        const vt::LimbId want = vt::limbFromName(p.get("limb").isNumber() ? std::to_string(p.get("limb").asInt(0)) : p.get("limb").asString(""));
        out = ResultJson(nvcoop::vats::liveSelectLimb(want));
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("vats.select_target", [](const std::string& params, std::string& out) {
        nvcoop::json::Value p;
        if (!ParseParams(params, p)) { out = "bad params"; return false; }
        out = ResultJson(nvcoop::vats::liveSelectTarget(ParseRef(p.get("ref"))));
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("vats.set", [](const std::string& params, std::string& out) {
        // host only: {"killcam":0|1, "partyslowmo":0|1, "autofire":0|1, "realtime":0|1, "slowscale":0.35}; the host's values win on every machine
        nvcoop::json::Value p;
        if (!ParseParams(params, p)) { out = "bad params"; return false; }
        vt::VatsSync* s = Sync();
        if (!s) { out = "{\"ok\":false,\"reason\":\"no co-op session\"}"; return true; }
        vt::Settings cur = s->settings();
        auto flag = [&](const char* k, bool& dst) {
            const nvcoop::json::Value* v = p.find(k);
            if (v) dst = v->type() == nvcoop::json::Value::Bool ? v->asBool(dst) : v->asInt(dst ? 1 : 0) != 0;
        };
        flag("killcam", cur.killCam);
        flag("partyslowmo", cur.partySlowMo);
        flag("autofire", cur.autoFire);
        flag("realtime", cur.realtime);
        flag("guard", cur.guardFriendly);
        if (const nvcoop::json::Value* v = p.find("slowscale")) cur.slowScale = static_cast<float>(v->asNumber(cur.slowScale));
        const bool changed = s->setSettings(cur, NowMs());
        const bool ok = changed || s->isHost();
        out = std::string("{\"ok\":") + (ok ? "true" : "false") + ",\"changed\":" + (changed ? "true" : "false") + ",\"reason\":\"" +
              (ok ? "" : "only the host sets the VATS settings") + "\"}";
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("vats.fire", [](const std::string&, std::string& out) {
        out = ResultJson(nvcoop::vats::liveFire());
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("vats.cancel", [](const std::string&, std::string& out) {
        out = ResultJson(vt::cancelAll(g_engine));
        return true;
    }, true, 0);
}

// ---------------------------------------------------------------------------------------------------------------- module
void Init() {
    if (g_inited) return;
    g_inited = true;
    g_mainThread = GetCurrentThreadId();   // DeferredInit runs on the game main thread
    ReadIni();
    RegisterBridge();
    if (!g_set.enabled) return;
    g_pins.setScale = PinOk(ad::kSetTimeScale, ad::kSetTimeScalePrologue);
    g_pins.getMult = PinOk(ad::kGetPlayerTimeMult, ad::kGetTimeMultPrologue) && PinOk(ad::kGetTargetTimeMult, ad::kGetTimeMultPrologue);
    g_pins.menuMode = PinOk(ad::kMenuMode, ad::kMenuModePrologue);
    g_pins.menuModeOf = PinOk(ad::kMenuModeOf, ad::kMenuModeOfPrologue);
    g_pins.accept = PinOk(ad::kAccept, ad::kAcceptPrologue);
    g_pins.unclick = PinOk(ad::kUnclick, ad::kUnclickPrologue);
    g_pins.limb = PinOk(ad::kLimbOfPart, ad::kLimbOfPartPrologue);
    g_pins.handleClick = PinOk(ad::kMenuHandleClickFn, ad::kMenuHandleClickPrologue);
    g_pins.getForm = PinOk(kGetFormByEdid, kGetFormPrologue);
    InstallListeners();
    nvcoop::Log("vats: enabled realtime=%d partyslowmo=%d autofire=%d pausesites=%d pins: scale=%d mult=%d menu=%d menuOf=%d accept=%d unclick=%d limb=%d click=%d form=%d",
                g_set.cfg.settings.realtime ? 1 : 0, g_set.cfg.settings.partySlowMo ? 1 : 0, g_set.cfg.settings.autoFire ? 1 : 0, g_set.pauseSites,
                g_pins.setScale ? 1 : 0, g_pins.getMult ? 1 : 0, g_pins.menuMode ? 1 : 0, g_pins.menuModeOf ? 1 : 0, g_pins.accept ? 1 : 0, g_pins.unclick ? 1 : 0,
                g_pins.limb ? 1 : 0, g_pins.handleClick ? 1 : 0, g_pins.getForm ? 1 : 0);
}

void Uninit() {
    g_rt.reset();            // restores the time scale and the camera settings
    RemoveHooks();
    RemoveListeners();
    g_engine.applyCameraTimes(false);
    g_inited = false;
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_DeferredInit) Init();
    else if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) Uninit();
}

void OnFrame() {
    if (!g_inited || !g_set.enabled) return;
    g_mainThread = GetCurrentThreadId();
    nvcoop::coop::Runtime* rt = nvcoop::coop::Runtime::active();
    vt::FrameInfo f;
    if (rt && rt->running()) {
        f.active = true;
        f.mux = rt->mux();
        f.nowMs = NowMs();
        if (auto* host = rt->host()) {
            f.isHost = true;
            f.self = fnvcoop::kHostPlayer;
            for (const auto& cv : host->clients()) {
                vt::ClientLink l;
                l.slot = cv.slot; l.peer = cv.peer; l.joined = cv.joined;
                f.clients.push_back(l);
            }
        } else if (auto* cl = rt->client()) {
            f.isHost = false;
            f.self = cl->joined() ? cl->slot() : fnvcoop::kNoPlayer;
            f.hostLinkUp = cl->joined() && !cl->baselineActive();
        } else {
            f.active = false;
        }
    }
    if (!g_rt) {
        if (!f.active) return;
        g_rt.reset(new vt::VatsRuntime(g_engine, g_set.cfg));
    }
    if (f.active && g_set.cfg.settings.realtime) InstallHooks();
    g_rt->frame(f);
}

}  // namespace

NVCOOP_MODULE(vats, "vats", nullptr, OnMessage, OnFrame);

// Hot reload: the call sites are restored and the listener removed before the core is replaced; the new core installs them again on its next session frame.
NVCOOP_RELOAD_STATE(vats, "vats", 1, 42,
    /*canUnload*/ [] {
        int bad = 0;
        for (HookGroup& g : g_groups) if (g.patcher) bad += g.patcher->audit();
        return bad ? std::string("a VATS call site was patched over by another mod") : std::string();
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

// ---------------------------------------------------------------------------------------------------------------- public live API (vats_api.h)
namespace nvcoop::vats {

bool liveAvailable() { return g_inited && g_set.enabled; }
VatsView liveView() { return readView(g_engine); }
ActionResult liveSelectLimb(LimbId want) { return selectLimb(g_engine, want); }
ActionResult liveSelectTarget(uint32_t ref) { return selectTarget(g_engine, ref); }
ActionResult liveFire() { return fire(g_engine); }

uint32_t liveAttackerFor(uint32_t attackerRef) {
    VatsSync* s = Sync();
    return s ? s->attackerFor(attackerRef, NowMs()) : attackerRef;
}

std::string liveStatusJson() {
    char b[1536];
    std::string sync = "null";
    if (VatsSync* s = Sync()) sync = s->statusJson(NowMs());
    std::string groups = "[";
    for (int i = 0; i < 7; ++i) {
        const HookGroup& g = g_groups[i];
        if (i) groups += ",";
        char gb[160];
        std::snprintf(gb, sizeof gb, "{\"name\":\"%s\",\"sites\":%d,\"total\":%d,\"pinsOk\":%s}", g.name ? g.name : "", g.patcher ? g.patcher->installed() : 0,
                      static_cast<int>(g.count), g.patcher && g.patcher->prologueOk() ? "true" : "false");
        groups += gb;
    }
    groups += "]";
    std::snprintf(b, sizeof b,
                  "{\"enabled\":%s,\"inited\":%s,\"running\":%s,\"pauseSites\":%d,"
                  "\"pins\":{\"setScale\":%s,\"getMult\":%s,\"menuMode\":%s,\"menuModeOf\":%s,\"accept\":%s,\"unclick\":%s,\"limb\":%s,\"handleClick\":%s,\"getForm\":%s},"
                  "\"thunks\":{\"scale\":%llu,\"scaleChanged\":%llu,\"mult\":%llu,\"multChanged\":%llu,\"pause\":%llu,\"pauseChanged\":%llu,\"offThread\":%llu,\"errors\":%llu,"
                  "\"uiUpdateOnMain\":%llu,\"workerUiSkips\":%llu},"
                  "\"engineCalls\":{\"calls\":%llu,\"failed\":%llu,\"pinRefusals\":%llu},\"multSettingsFallback\":%s,",
                  g_set.enabled ? "true" : "false", g_inited ? "true" : "false", Sync() ? "true" : "false", g_set.pauseSites,
                  g_pins.setScale ? "true" : "false", g_pins.getMult ? "true" : "false", g_pins.menuMode ? "true" : "false", g_pins.menuModeOf ? "true" : "false",
                  g_pins.accept ? "true" : "false", g_pins.unclick ? "true" : "false", g_pins.limb ? "true" : "false", g_pins.handleClick ? "true" : "false",
                  g_pins.getForm ? "true" : "false", (unsigned long long)g_thunkScale, (unsigned long long)g_thunkScaleChanged, (unsigned long long)g_thunkMult,
                  (unsigned long long)g_thunkMultChanged, (unsigned long long)g_thunkPause, (unsigned long long)g_thunkPauseChanged, (unsigned long long)g_thunkOffThread,
                  (unsigned long long)g_thunkErrors, (unsigned long long)g_imOnMain, (unsigned long long)g_imWorkerSkips, (unsigned long long)g_calls, (unsigned long long)g_callsFailed, (unsigned long long)g_pinRefusals,
                  g_engine.multSettingsActive() ? "true" : "false");
    return std::string(b) + "\"hooks\":" + groups + ",\"sync\":" + sync + "}";
}

}  // namespace nvcoop::vats
