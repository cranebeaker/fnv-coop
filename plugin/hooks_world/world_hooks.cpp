// L12 detours + host polls. x86 / MSVC only. Style follows plugin/hooks/engine_hooks.cpp (L5): every detour is a plain cdecl
// script-command handler, pinned by prologue bytes in world_addrs.h (a mismatch skips the hook, never forces it).
#include "world_hooks.h"

#include <Windows.h>

#include <algorithm>
#include <cmath>
#include <cstring>
#include <set>

#include "GameAPI.h"
#include "GameData.h"
#include "GameExtraData.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "GameProcess.h"
#include "common/nvcoop_api.h"
#include "effect_json.h"
#include "engine_addrs.h"
#include "world_addrs.h"
#include "world_owner_addrs.h"
#include "world_present_addrs.h"
#include "world_query.h"
#include "world_refvar_watch.h"
#include "world_refvars.h"

using namespace fnvcoop;

namespace nvworld {
namespace {

struct Win32Memory : nvhooks::MemoryOps {
    bool read(nvhooks::Addr a, void* out, size_t n) override { return peekMemory(a, out, n); }
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

nvhooks::CaptureHub* g_hub = nullptr;
nvhooks::Role g_role = nvhooks::Role::Host;
Win32Memory g_mem;
std::unique_ptr<nvhooks::HookRegistry> g_reg;
SpawnLedger g_ledger;
WorldLog g_log;
WorldBaseline g_base;
WeatherTimeSync g_weather;
RefVarPoller g_poller;
std::vector<uint32_t> g_staticWatch;   // world_refvar_watch.h resolved to engine ids (once per loaded world)
bool g_staticWatchBuilt = false;
std::function<void(const WorldEffect&)> g_sink;
std::function<void(const EffectTxn&)> g_txnObserver;   // setTxnObserver
std::vector<FormRef> g_watched;

constexpr nvhooks::Addr kThePlayerPtr = 0x11DEA3C;
constexpr uint32_t kPlayerRefID = 0x14;
constexpr uint32_t kTimeScaleGlobal = 0x3A;   // FalloutNV.esm TimeScale

inline void* thePlayer() { return *reinterpret_cast<void**>(kThePlayerPtr); }
inline bool capturing() { return g_hub && g_hub->active(); }
inline uint32_t refIdOf(const void* f) { return f ? static_cast<const TESForm*>(f)->refID : 0; }
inline bool isPlayer(const void* r) { return r && refIdOf(r) == kPlayerRefID; }

// Guest suppression: L5's policy decides (script-driven shared-state mutators are dropped on a client; applier runs pass).
bool blockedNow() { return nvhooks::blockMutationNow(); }

bool formRef(const void* form, FormRef* out) {
    uint32_t id = refIdOf(form);
    if (!id) return false;
    if ((id >> 24) == 0xFFu) {  // created ref: the host's engine id IS the wire id
        *out = FormRef(createdRefPlugin(), id);
        return true;
    }
    return nvhooks::resolver().toRef(id, out);
}

template <class R, class... A>
inline R tc(nvhooks::Addr fn, void* self, A... a) {
    return reinterpret_cast<R(__thiscall*)(void*, A...)>(fn)(self, a...);
}
float getAV(void* actor, uint32_t av) {
    if (!actor) return 0;
    return tc<float, uint32_t>(nvhooks::addr::ActorGetActorValue.target, reinterpret_cast<char*>(actor) + 0xA4, av);
}

using Fn_Cmd = bool(__cdecl*)(void*, void*, void*, void*, void*, void*, double*, UInt32*);
using Fn_Extract = bool(__cdecl*)(void*, void*, UInt32*, void*, void*, void*, void*, ...);
Fn_Cmd o_SetOwnership, o_ClearOwnership, o_SetCellOwnership, o_SetCellPublicFlag;
std::function<bool(uint32_t)> g_advance;
std::function<void(const fnvcoop::ObjectState&)> g_objObserver;
Fn_Cmd o_MarkForDelete, o_SetEssential, o_SetActorRefEssential, o_SetRestrained, o_SetEnemy, o_SetAlly, o_RestoreAV,
    o_IgnoreCrime, o_SetPCEnemyOfFaction, o_ClearFactionPlayerEnemyFlag;

#define CMD_ARGS void* pi, void* sd, void* thisObj, void* contObj, void* script, void* ev, double* res, UInt32* opc
#define CMD_FWD(o) o(pi, sd, thisObj, contObj, script, ev, res, opc)

template <class... Out>
bool parseArgs(CMD_ARGS, Out*... outs) {
    UInt32 off = *opc;  // ExtractArgs advances the offset: work on a copy so the real call sees the original
    return reinterpret_cast<Fn_Extract>(nvhooks::addr::ExtractArgs.target)(pi, sd, &off, thisObj, contObj, script, ev, outs...);
}

void emitLocal(const WorldEffect& e) {
    g_log.push(e);
    g_base.onWorld(e);
    if (g_sink) g_sink(e);
}

bool __cdecl h_MarkForDelete(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef rr;
    uint32_t id = refIdOf(thisObj);
    bool track = capturing() && thisObj && !isPlayer(thisObj) && formRef(thisObj, &rr);
    bool ok = CMD_FWD(o_MarkForDelete);
    if (ok && track) {
        // synccore's DespawnRef names created refs only. An esm reference marked for delete is gone for good, which for the
        // other players is the same as disabled: send DisableRef (idempotent, safe on a replay).
        if ((id >> 24) == 0xFFu) g_hub->record(DespawnRef{id});
        else g_hub->record(DisableRef{rr});
    }
    return ok;
}

bool __cdecl h_SetEssential(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef ar;
    TESForm* base = nullptr;
    int v = 0;
    bool parsed = capturing() && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &base, &v);
    bool ok = CMD_FWD(o_SetEssential);
    if (ok && parsed && base && formRef(base, &ar)) emitLocal(SetEssential{ar, v > 0, false});
    return ok;
}

bool __cdecl h_SetActorRefEssential(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef ar;
    int v = 0;
    bool track = capturing() && thisObj && formRef(thisObj, &ar);
    bool parsed = track && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &v);
    bool ok = CMD_FWD(o_SetActorRefEssential);
    if (ok && parsed) emitLocal(SetEssential{ar, v > 0, true});
    return ok;
}

bool __cdecl h_SetRestrained(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef ar;
    int v = 0;
    bool track = capturing() && thisObj && formRef(thisObj, &ar);
    bool parsed = track && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &v);
    bool ok = CMD_FWD(o_SetRestrained);
    if (ok && parsed) emitLocal(SetRestrained{ar, v != 0});
    return ok;
}

bool relationCmd(Fn_Cmd o, Relation kind, CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef fa, fb;
    TESForm *a = nullptr, *b = nullptr;
    int flags1 = 0, flags2 = 0;  // two optional ints follow the factions
    bool parsed = capturing() && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &a, &b, &flags1, &flags2);
    bool ok = CMD_FWD(o);
    if (ok && parsed && a && b && formRef(a, &fa) && formRef(b, &fb)) {
        SetRelation r{fa, fb, kind};
        r.flags1 = static_cast<uint8_t>(flags1 < 0 ? 0 : flags1 > 255 ? 255 : flags1);   // the two optional integers travel as given
        r.flags2 = static_cast<uint8_t>(flags2 < 0 ? 0 : flags2 > 255 ? 255 : flags2);
        emitLocal(r);
    }
    return ok;
}
bool __cdecl h_SetEnemy(CMD_ARGS) { return relationCmd(o_SetEnemy, Relation::Enemy, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_SetAlly(CMD_ARGS) { return relationCmd(o_SetAlly, Relation::Ally, pi, sd, thisObj, contObj, script, ev, res, opc); }

bool __cdecl h_RestoreAV(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef ar;
    UInt32 av = 0xFFFFFFFF;
    float val = 0;
    bool track = capturing() && thisObj && !isPlayer(thisObj) && formRef(thisObj, &ar);
    bool parsed = track && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &av, &val);
    float before = parsed ? getAV(thisObj, av) : 0;
    bool ok = CMD_FWD(o_RestoreAV);
    if (ok && parsed) recordNonPlayerAv(ar, av, AvCmdKind::Restore, before, getAV(thisObj, av));
    return ok;
}

// IgnoreCrime <int> on an actor (opcode 0x10AB): ignore-crime flag = value > 0. The player is per machine, not shared.
bool __cdecl h_IgnoreCrime(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef ar;
    int v = 0;
    bool track = capturing() && thisObj && !isPlayer(thisObj) && formRef(thisObj, &ar);
    bool parsed = track && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &v);
    bool ok = CMD_FWD(o_IgnoreCrime);
    if (ok && parsed) emitLocal(SetIgnoreCrime{ar, v > 0});
    return ok;
}

// SetPCEnemyOfFaction <faction> <int> (0x10C6): player-enemy flag = value != 0. ClearFactionPlayerEnemyFlag <faction> (0x11C9): flag = 0.
bool __cdecl h_SetPCEnemyOfFaction(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef fr;
    TESForm* f = nullptr;
    int v = 0;
    bool parsed = capturing() && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &f, &v);
    bool ok = CMD_FWD(o_SetPCEnemyOfFaction);
    if (ok && parsed && f && formRef(f, &fr)) emitLocal(SetFactionPlayerEnemy{fr, v != 0});
    return ok;
}

bool __cdecl h_ClearFactionPlayerEnemyFlag(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef fr;
    TESForm* f = nullptr;
    bool parsed = capturing() && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &f);
    bool ok = CMD_FWD(o_ClearFactionPlayerEnemyFlag);
    if (ok && parsed && f && formRef(f, &fr)) emitLocal(SetFactionPlayerEnemy{fr, false});
    return ok;
}

// SetOwnership [owner] on a reference. A missing argument means "the player owns it": owner stays empty in the effect.
bool __cdecl h_SetOwnership(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef rr, ow;
    TESForm* owner = nullptr;
    bool track = capturing() && thisObj && formRef(thisObj, &rr);
    bool parsed = track && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &owner);
    bool ok = CMD_FWD(o_SetOwnership);
    if (ok && parsed) {
        SetOwner e;
        e.ref = rr;
        if (owner) {
            if (!formRef(owner, &ow)) return ok;
            e.owner = ow;
        }
        emitLocal(e);
    }
    return ok;
}

bool __cdecl h_ClearOwnership(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef rr;
    bool track = capturing() && thisObj && formRef(thisObj, &rr);
    bool ok = CMD_FWD(o_ClearOwnership);
    if (ok && track) {
        SetOwner e;
        e.ref = rr;
        e.clear = true;
        emitLocal(e);
    }
    return ok;
}

bool __cdecl h_SetCellOwnership(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef cr, ow;
    TESForm *cell = nullptr, *owner = nullptr;
    bool parsed = capturing() && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &cell, &owner);
    bool ok = CMD_FWD(o_SetCellOwnership);
    if (ok && parsed && cell && owner && formRef(cell, &cr) && formRef(owner, &ow)) emitLocal(SetCellOwner{cr, ow});
    return ok;
}

bool __cdecl h_SetCellPublicFlag(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef cr;
    TESForm* cell = nullptr;
    int v = 0;
    bool parsed = capturing() && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &cell, &v);
    bool ok = CMD_FWD(o_SetCellPublicFlag);
    if (ok && parsed && cell && formRef(cell, &cr)) emitLocal(SetCellPublic{cr, v != 0});
    return ok;
}

// Presentation (G39): ApplyImageSpaceModifier / RemoveImageSpaceModifier <imod> [strength], PlayMusic <music>, PlaySound <sound> [int]. Never blocked (they change
// no shared state, a guest's own scripts must still show their own fades); captured on the HOST only, inside a capture scope.
Fn_Cmd o_ApplyImod, o_RemoveImod, o_PlayMusic, o_PlaySound;
bool presentCmd(Fn_Cmd o, PresentKind kind, CMD_ARGS) {
    FormRef fr;
    TESForm* f = nullptr;
    float strength = 0;
    int opt = 0;
    const bool takesFloat = kind == PresentKind::ApplyImageSpace || kind == PresentKind::RemoveImageSpace;
    const bool takesInt = kind == PresentKind::PlaySound;
    const bool track = g_role == nvhooks::Role::Host && capturing();
    bool parsed = false;
    if (track) {
        if (takesFloat) parsed = parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &f, &strength);
        else if (takesInt) parsed = parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &f, &opt);
        else parsed = parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &f);
    }
    bool ok = CMD_FWD(o);
    if (ok && parsed && f && formRef(f, &fr)) {
        Present p;
        p.kind = kind;
        p.form = fr;
        if (takesFloat && strength != 0) { p.hasValue = true; p.value = strength; }
        if (takesInt && opt != 0) { p.hasValue = true; p.value = static_cast<float>(opt); }
        emitLocal(p);
    }
    return ok;
}
bool __cdecl h_ApplyImod(CMD_ARGS) { return presentCmd(o_ApplyImod, PresentKind::ApplyImageSpace, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_RemoveImod(CMD_ARGS) { return presentCmd(o_RemoveImod, PresentKind::RemoveImageSpace, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_PlayMusic(CMD_ARGS) { return presentCmd(o_PlayMusic, PresentKind::PlayMusic, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_PlaySound(CMD_ARGS) { return presentCmd(o_PlaySound, PresentKind::PlaySound, pi, sd, thisObj, contObj, script, ev, res, opc); }

struct Entry {
    const nvhooks::HookSpec* spec;
    void* detour;
    void* origOut;
};
#define HK(spec, det, orig) {&world_addrs::spec, reinterpret_cast<void*>(&det), reinterpret_cast<void*>(&orig)}
const Entry kEntries[] = {
    HK(Cmd_MarkForDelete, h_MarkForDelete, o_MarkForDelete),
    HK(Cmd_SetEssential, h_SetEssential, o_SetEssential),
    HK(Cmd_SetActorRefEssential, h_SetActorRefEssential, o_SetActorRefEssential),
    HK(Cmd_SetRestrained, h_SetRestrained, o_SetRestrained),
    HK(Cmd_SetEnemy, h_SetEnemy, o_SetEnemy),
    HK(Cmd_SetAlly, h_SetAlly, o_SetAlly),
    HK(Cmd_RestoreActorValue, h_RestoreAV, o_RestoreAV),
    HK(Cmd_IgnoreCrime, h_IgnoreCrime, o_IgnoreCrime),
    HK(Cmd_SetPCEnemyOfFaction, h_SetPCEnemyOfFaction, o_SetPCEnemyOfFaction),
    HK(Cmd_ClearFactionPlayerEnemyFlag, h_ClearFactionPlayerEnemyFlag, o_ClearFactionPlayerEnemyFlag),
};
#define HKO(spec, det, orig) {&world_owner_addrs::spec, reinterpret_cast<void*>(&det), reinterpret_cast<void*>(&orig)}
const Entry kOwnerEntries[] = {
    HKO(Cmd_SetOwnership, h_SetOwnership, o_SetOwnership),
    HKO(Cmd_ClearOwnership, h_ClearOwnership, o_ClearOwnership),
    HKO(Cmd_SetCellOwnership, h_SetCellOwnership, o_SetCellOwnership),
    HKO(Cmd_SetCellPublicFlag, h_SetCellPublicFlag, o_SetCellPublicFlag),
};
#define HKP(spec, det, orig) {&world_present_addrs::spec, reinterpret_cast<void*>(&det), reinterpret_cast<void*>(&orig)}
const Entry kPresentEntries[] = {
    HKP(Cmd_ApplyImageSpaceModifier, h_ApplyImod, o_ApplyImod),
    HKP(Cmd_RemoveImageSpaceModifier, h_RemoveImod, o_RemoveImod),
    HKP(Cmd_PlayMusic, h_PlayMusic, o_PlayMusic),
    HKP(Cmd_PlaySound, h_PlaySound, o_PlaySound),
};

// ------------------------------------------------------------------ polls
// Current weather through the engine's own accessor chain (the one Cmd_GetIsCurrentWeather's evaluator 0x59EBB0 uses):
// obj = 0x46DD00() ; weather = thiscall 0x44EDB0(obj). Both are re-verified against their recorded prologue bytes first.
bool callsVerified() {
    static int state = 0;  // 0 unknown, 1 ok, -1 bad
    if (state) return state > 0;
    state = 1;
    for (size_t i = 0; i < world_addrs::kCallCount; ++i) {
        const world_addrs::CallSpec& c = *world_addrs::kCalls[i];
        unsigned char now[16] = {};
        if (!peekMemory(c.addr, now, 16) || std::memcmp(now, c.bytes, 16) != 0) state = -1;
    }
    return state > 0;
}

void* skyWeather() {
    if (!callsVerified()) return nullptr;
    void* obj = reinterpret_cast<void*(__cdecl*)()>(world_addrs::Call_SkySingleton.addr)();
    if (!obj) return nullptr;
    void* w = reinterpret_cast<void*(__thiscall*)(void*)>(world_addrs::Call_SkyCurrentWeather.addr)(obj);
    if (!w || static_cast<const TESForm*>(w)->typeID != kFormType_TESWeather) return nullptr;
    return w;
}

float timeScale() {
    uint32_t id = nvhooks::resolver().fromRef(FormRef("falloutnv.esm", kTimeScaleGlobal));
    TESGlobal* g = static_cast<TESGlobal*>(id ? nvhooks::resolver().lookup(id) : nullptr);
    return g ? g->data : 0;
}

template <class F>
void forRefVars(TESObjectREFR* r, F f) {
    if (!r) return;
    BSExtraData* xd = r->extraDataList.GetByType(kExtraData_Script);
    ScriptEventList* el = xd ? static_cast<ExtraScript*>(xd)->eventList : nullptr;
    if (!el || !el->m_vars) return;
    for (auto v = el->m_vars->Begin(); !v.End(); ++v)
        if (v.Get() && v.Get()->id <= 0xFFFFu) f(static_cast<uint16_t>(v.Get()->id), v.Get()->data);
}

TESObjectCELL* playerCell() {
    TESObjectREFR* p = static_cast<TESObjectREFR*>(thePlayer());
    return p ? p->parentCell : nullptr;
}

// Loaded actors, every cell the engine keeps in a process list (the player's cell is walked separately). Plain arrays: SEH cannot live next to C++ objects.
void walkActorList(const tList<Actor>* list, uint32_t* buf, size_t cap, size_t* n) {
    __try {
        int guard = 0;
        for (auto* node = list->Head(); node && guard < 6000; node = node->next, ++guard) {
            Actor* a = node->data;
            if (!a || (a->typeID != kFormType_Character && a->typeID != kFormType_Creature)) continue;
            if (*n < cap) buf[(*n)++] = a->refID;
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
}

class EngineRefVarSource : public IRefVarSource {
public:
    uint32_t playerCell() override { return refIdOf(nvworld::playerCell()); }
    void cellRefs(std::vector<uint32_t>& out) override {
        TESObjectCELL* cell = nvworld::playerCell();
        if (!cell) return;
        for (auto it = cell->objectList.Begin(); !it.End(); ++it) {
            TESObjectREFR* r = it.Get();
            if (r && r->refID && r->extraDataList.GetByType(kExtraData_Script)) out.push_back(r->refID);
        }
    }
    void actorRefs(std::vector<uint32_t>& out) override {
        if (!g_actorProcessManager) return;
        static uint32_t buf[8192];
        size_t n = 0;
        walkActorList(&g_actorProcessManager->highActors, buf, 8192, &n);
        walkActorList(&g_actorProcessManager->middleHighActors.head, buf, 8192, &n);
        out.insert(out.end(), buf, buf + n);
    }
    void watchedRefs(std::vector<uint32_t>& out) override {
        if (!g_staticWatchBuilt) {
            g_staticWatchBuilt = true;
            g_staticWatch.clear();
            for (size_t i = 0; i < world_refvar_watch::kCount; ++i) {
                uint32_t id = nvhooks::resolver().fromRef(FormRef(world_refvar_watch::kRefs[i].plugin, world_refvar_watch::kRefs[i].localId));
                if (id) g_staticWatch.push_back(id);
            }
        }
        out.insert(out.end(), g_staticWatch.begin(), g_staticWatch.end());
        auto add = [&](const FormRef& w) {
            uint32_t id = nvhooks::resolver().fromRef(w);
            if (id) out.push_back(id);
        };
        for (const FormRef& w : g_watched) add(w);
        for (const FormRef& w : g_base.liveSpawnRefs()) add(w);   // spawned refs keep being polled when the player is elsewhere
    }
    bool readVars(uint32_t refID, std::vector<std::pair<uint16_t, double>>& out) override {
        TESObjectREFR* r = static_cast<TESObjectREFR*>(nvhooks::resolver().lookup(refID));
        if (!r) return false;
        BSExtraData* xd = r->extraDataList.GetByType(kExtraData_Script);
        ScriptEventList* el = xd ? static_cast<ExtraScript*>(xd)->eventList : nullptr;
        if (!el || !el->m_vars) return false;
        forRefVars(r, [&](uint16_t idx, double val) { out.emplace_back(idx, val); });
        return true;
    }
};
EngineRefVarSource g_src;

// RefVarChange -> RefScriptVar effect (a ref the wire cannot name is dropped).
void toEffects(const std::vector<RefVarChange>& in, std::vector<RefScriptVar>* out) {
    for (const RefVarChange& c : in) {
        TESObjectREFR* r = static_cast<TESObjectREFR*>(nvhooks::resolver().lookup(c.refID));
        FormRef rr;
        if (r && formRef(r, &rr)) out->push_back(RefScriptVar{rr, c.index, c.value});
    }
}

}  // namespace

// ------------------------------------------------------------------ public API
nvhooks::FormResolver& makeResolver(bool hostSide) {
    static std::unique_ptr<WorldResolver> r;
    r.reset(new WorldResolver(nvhooks::defaultResolver(), g_ledger.map(), hostSide));
    return *r;
}
SpawnLedger& ledger() { return g_ledger; }

InitReport Init(nvhooks::CaptureHub& hub, nvhooks::Role role) {
    InitReport rep;
    g_hub = &hub;
    g_role = role;
    if (!g_reg) g_reg = std::make_unique<nvhooks::HookRegistry>(g_mem);
    if (role == nvhooks::Role::Client) return rep;  // gate-only role: no capture hooks
    for (const Entry& e : kEntries) {
        nvhooks::Addr tramp = 0;
        nvhooks::PatchStatus st = g_reg->acquire(*e.spec, reinterpret_cast<nvhooks::Addr>(e.detour), &tramp, reinterpret_cast<nvhooks::Addr*>(e.origOut));
        if (nvhooks::installed(st)) {
            *reinterpret_cast<void**>(e.origOut) = reinterpret_cast<void*>(tramp);
            ++rep.ok;
        } else {
            ++rep.failed;
        }
        rep.hooks.push_back({e.spec->name, st});
    }
    for (const Entry& e : kOwnerEntries) {   // ownership commands (world-time lane)
        nvhooks::Addr tramp = 0;
        nvhooks::PatchStatus st = g_reg->acquire(*e.spec, reinterpret_cast<nvhooks::Addr>(e.detour), &tramp, reinterpret_cast<nvhooks::Addr*>(e.origOut));
        if (nvhooks::installed(st)) {
            *reinterpret_cast<void**>(e.origOut) = reinterpret_cast<void*>(tramp);
            ++rep.ok;
        } else {
            ++rep.failed;
        }
        rep.hooks.push_back({e.spec->name, st});
    }
        if (role == nvhooks::Role::Host) {   // presentation of host-run scripts (G39)
        for (const Entry& e : kPresentEntries) {
            nvhooks::Addr tramp = 0;
            nvhooks::PatchStatus st = g_reg->acquire(*e.spec, reinterpret_cast<nvhooks::Addr>(e.detour), &tramp, reinterpret_cast<nvhooks::Addr*>(e.origOut));
            if (nvhooks::installed(st)) {
                *reinterpret_cast<void**>(e.origOut) = reinterpret_cast<void*>(tramp);
                ++rep.ok;
            } else {
                ++rep.failed;
            }
            rep.hooks.push_back({e.spec->name, st});
        }
    }
    // L5 detours these commands; it reports non-player AV changes and script packages through observers.
    nvhooks::setNonPlayerAvObserver([](const FormRef& actor, uint32_t av, uint8_t kind, float before, float after) {
        if (kind <= static_cast<uint8_t>(AvCmdKind::Damage)) recordNonPlayerAv(actor, av, static_cast<AvCmdKind>(kind), before, after);
    });
    nvhooks::setScriptPackageObserver([](const FormRef& actor, const FormRef& package, bool add) {
        if (capturing()) emitLocal(SetScriptPackage{actor, package, add});
    });
    // Script-driven SetWeather/SetTimeScale that already went out through a txn must not be re-sent by the next poll.
    hub.addSink([](EffectTxn&& t) {
        if (g_txnObserver) g_txnObserver(t);
        for (const Effect& e : t.effects) {
            g_weather.noteSent(e);
            g_base.onEffect(e);  // net spawn / ref var state for joiners and rejoiners
            if (auto* os = std::get_if<fnvcoop::ObjectState>(&e)) {
                if (g_objObserver) g_objObserver(*os);   // the world-time poll must not send what a script command already sent
                // A script's (or a guest intent's) Lock / UnLock / SetOpenState / SetDestroyed also goes out as a world effect: the txn copy reaches a client only
                // through the shadow's change filter, which does not know the engine's own lock state, and a client that joins later has to be told too.
                // The world-time poll's own effects are marked (cause.aux) and not repeated.
                if (g_role == nvhooks::Role::Host)
                    if (auto w = worldCopyOfObjectState(*os, t.cause.aux)) emitLocal(*w);
            }
        }
    });
    return rep;
}

int auditTamperedHooks() { return g_reg ? g_reg->auditTampered() : 0; }

void Shutdown() {
    nvhooks::setNonPlayerAvObserver(nullptr);
    nvhooks::setScriptPackageObserver(nullptr);
    g_reg.reset();
    g_hub = nullptr;
}

void resetSession() {
    g_ledger.clear();
    g_base.clear();
    g_poller.reset();
    g_staticWatchBuilt = false;
    g_weather.reset();
    g_log.drain();
}

std::vector<WorldEffect> drainWorldEffects() { return g_log.drain(); }
BaselineSnapshot hostBaseline() { return g_base.snapshot(); }
void setWorldEffectSink(std::function<void(const WorldEffect&)> sink) { g_sink = std::move(sink); }

void setTxnObserver(std::function<void(const EffectTxn&)> observer) { g_txnObserver = std::move(observer); }

void recordNonPlayerAv(const FormRef& actor, uint32_t av, AvCmdKind kind, float before, float after) {
    if (!capturing()) return;
    if (auto e = makeAvEffect(actor, av, kind, before, after)) emitLocal(*e);
}

void watchRef(const FormRef& r) {
    if (std::find(g_watched.begin(), g_watched.end(), r) == g_watched.end()) g_watched.push_back(r);
}
void clearWatched() { g_watched.clear(); }

size_t pollRefVarsNow(std::vector<RefScriptVar>* out) {
    std::vector<RefScriptVar> tmp;
    if (!out) out = &tmp;
    std::vector<RefVarChange> ch;
    g_poller.pollAll(g_src, ch);
    const size_t before = out->size();
    toEffects(ch, out);
    return out->size() - before;
}

void setRefVarPollConfig(const RefVarPollCfg& cfg) { g_poller.setConfig(cfg); }
RefVarPollStats refVarPollStats() {
    RefVarPollStats st;
    st.sweeps = g_poller.sweeps();
    st.fastReads = g_poller.fastReads();
    st.tracked = g_poller.tracker().size();
    st.pending = g_poller.pending();
    st.watched = g_staticWatch.size() + g_watched.size();
    return st;
}
void noteRefScriptRan(uint32_t refID) {
    if (!g_hub || !g_hub->enabled() || g_role != nvhooks::Role::Host) return;
    g_poller.afterScript(refID, g_src);
}

void hostTick() {
    static uint32_t frame = 0;
    if (!g_hub || !g_hub->enabled() || g_role != nvhooks::Role::Host) return;
    ++frame;
    if (g_hub->depth() != 0) return;  // never inside a script scope

    std::vector<RefVarChange> ch;
    g_poller.tick(frame, g_src, ch);
    std::vector<Effect> fx;
    if (frame % 15 == 0) {
        FormRef wr;
        void* w = skyWeather();
        bool haveW = w && formRef(w, &wr);
        fx = g_weather.poll(wr, haveW, timeScale());
    }
    std::vector<RefScriptVar> vars;
    toEffects(ch, &vars);
    for (auto& v : vars) fx.push_back(v);
    if (fx.empty()) return;
    Cause c;
    c.kind = CauseKind::Script;
    nvhooks::ScopedCapture sc(*g_hub, c);
    for (const Effect& e : fx) g_hub->record(e);
}

bool probeRef(uint32_t refID, RefProbe* out) {
    TESObjectREFR* r = static_cast<TESObjectREFR*>(refID ? nvhooks::resolver().lookup(refID) : nullptr);
    RefProbe p;
    if (r) {
        p.exists = true;
        p.refID = r->refID;
        p.baseID = refIdOf(r->baseForm);
        p.cellID = refIdOf(r->parentCell);
        p.flags = r->flags;
        p.x = r->posX;
        p.y = r->posY;
        p.z = r->posZ;
        BSExtraData* xd = r->extraDataList.GetByType(kExtraData_Script);
        ScriptEventList* el = xd ? static_cast<ExtraScript*>(xd)->eventList : nullptr;
        if (el && el->m_vars) {
            p.nvars = 0;
            for (auto v = el->m_vars->Begin(); !v.End(); ++v) ++p.nvars;
        }
    }
    *out = p;
    return p.exists;
}

std::vector<ScriptedRef> scriptedRefsInPlayerCell(size_t max) {
    std::vector<ScriptedRef> out;
    TESObjectCELL* cell = playerCell();
    if (!cell) return out;
    for (auto it = cell->objectList.Begin(); !it.End() && out.size() < max; ++it) {
        TESObjectREFR* r = it.Get();
        if (!r || !r->refID) continue;
        RefProbe p;
        probeRef(r->refID, &p);
        if (p.nvars <= 0) continue;
        ScriptedRef s;
        s.refID = r->refID;
        s.baseID = p.baseID;
        s.nvars = p.nvars;
        FormRef fr;
        if (formRef(r, &fr)) s.ref = nvhooks::refToStr(fr);
        out.push_back(s);
    }
    return out;
}

bool getRefVar(uint32_t refID, uint16_t index, double* out) {
    TESObjectREFR* r = static_cast<TESObjectREFR*>(refID ? nvhooks::resolver().lookup(refID) : nullptr);
    bool found = false;
    if (r)
        forRefVars(r, [&](uint16_t idx, double v) {
            if (idx == index) { *out = v; found = true; }
        });
    return found;
}

bool setRefVarDirect(uint32_t refID, uint16_t index, double value) {
    TESObjectREFR* r = static_cast<TESObjectREFR*>(refID ? nvhooks::resolver().lookup(refID) : nullptr);
    BSExtraData* xd = r ? r->extraDataList.GetByType(kExtraData_Script) : nullptr;
    ScriptEventList* el = xd ? static_cast<ExtraScript*>(xd)->eventList : nullptr;
    if (!el || !el->m_vars) return false;
    for (auto it = el->m_vars->Begin(); !it.End(); ++it)
        if (it.Get() && it.Get()->id == index) {
            it.Get()->data = value;
            return true;
        }
    return false;
}

bool currentWeather(FormRef* out) {
    void* w = skyWeather();
    return w && formRef(w, out);
}
float currentTimeScale() { return timeScale(); }
uint32_t playerCellID() { return refIdOf(playerCell()); }

bool queryCondition(const std::string& fn, uint32_t refID, uint32_t p1, double* out) {
    // evaluator = 4th field of the command table entry (0x118E8E0, stride 0x28); signature cdecl(thisObj, p1, p2, double* result).
    // world_query.cpp decides what to CALL: the live entry when it is still the original prologue, the hook trampoline when the entry is
    // OUR detour (Cond_GetActorValue, Cond_GetItemCount ...: a call through the detour would also be recorded as gameplay), nothing otherwise.
    const Evaluator* q = findEvaluator(fn);
    if (!q) return false;
    EvalPlan plan = resolveEvaluator(*q, [](uint32_t a, void* o, size_t n) { return peekMemory(a, o, n); },
                                     [](uint32_t target) { return static_cast<uint32_t>(nvhooks::trampolineOfAny(target)); });
    if (plan.kind == EvalCall::Refuse) return false;
    void* ref = refID ? nvhooks::resolver().lookup(refID) : nullptr;
    if (!ref) return false;
    double r = 0;
    void* arg = reinterpret_cast<void*>(static_cast<uintptr_t>(p1));
    if (q->formParam && !(arg = nvhooks::resolver().lookup(p1))) return false;
    bool ok = reinterpret_cast<bool(__cdecl*)(void*, void*, void*, double*)>(plan.addr)(ref, arg, nullptr, &r);
    *out = r;
    return ok;
}

void setAdvanceClockHandler(std::function<bool(uint32_t minutes)> f) { g_advance = std::move(f); }
void setObjectStateObserver(std::function<void(const fnvcoop::ObjectState&)> f) { g_objObserver = std::move(f); }

bool refToString(uint32_t refID, FormRef* out) {
    TESObjectREFR* r = static_cast<TESObjectREFR*>(refID ? nvhooks::resolver().lookup(refID) : nullptr);
    return r && formRef(r, out);
}

std::vector<WorldObject> objectsInPlayerCell(size_t start, size_t max, size_t* total) {
    std::vector<WorldObject> out;
    size_t seen = 0;
    TESObjectCELL* cell = playerCell();
    if (cell) {
        for (auto it = cell->objectList.Begin(); !it.End(); ++it) {
            TESObjectREFR* r = it.Get();
            if (!r || !r->refID || r->refID == kPlayerRefID || !r->baseForm) continue;
            const uint32_t t = r->baseForm->typeID;
            int kind = -1;
            if (t == kFormType_TESObjectDOOR) kind = 0;
            else if (t == kFormType_TESObjectCONT) kind = 1;
            else if (t == kFormType_BGSTerminal) kind = 2;
            if (kind < 0) continue;
            if (seen >= start && out.size() < max) {
                WorldObject o;
                o.refID = r->refID;
                o.baseType = t;
                o.kind = kind;
                FormRef fr;
                if (formRef(r, &fr)) o.ref = nvhooks::refToStr(fr);
                if (!o.ref.empty()) out.push_back(o);
            }
            ++seen;
        }
    }
    if (total) *total = seen;
    return out;
}

ObjectReading readObject(const WorldObject& o) {
    ObjectReading r;
    double v = 0;
    if (!queryCondition("GetLocked", o.refID, 0, &v)) return r;
    r.locked = v != 0;
    if (queryCondition("GetLockLevel", o.refID, 0, &v)) r.lockLevel = static_cast<int>(v);
    if (o.kind == 0 && queryCondition("GetOpenState", o.refID, 0, &v)) r.openState = static_cast<int>(v);
    if (o.kind != 2 && queryCondition("GetDestroyed", o.refID, 0, &v)) r.destroyed = v != 0;
    r.ok = true;
    return r;
}

bool queryInCombat(uint32_t refID, bool* out) {
    static const Evaluator q = {"IsInCombat", 0x5A3180, false};   // Cond_IsInCombat (plugin/hooks/engine_addrs.h); L5 hooks it, so the trampoline may be needed
    EvalPlan plan = resolveEvaluator(q, [](uint32_t a, void* o, size_t n) { return peekMemory(a, o, n); },
                                     [](uint32_t target) { return static_cast<uint32_t>(nvhooks::trampolineOfAny(target)); });
    if (plan.kind == EvalCall::Refuse) return false;
    void* ref = refID ? nvhooks::resolver().lookup(refID) : nullptr;
    if (!ref) return false;
    double r = 0;
    bool ok = reinterpret_cast<bool(__cdecl*)(void*, void*, void*, double*)>(plan.addr)(ref, nullptr, nullptr, &r);
    if (ok && out) *out = r != 0;
    return ok;
}

std::vector<ActorRef> actorsInPlayerCell(size_t max, uint32_t formType) {
    std::vector<ActorRef> out;
    TESObjectCELL* cell = playerCell();
    if (!cell) return out;
    for (auto it = cell->objectList.Begin(); !it.End() && out.size() < max; ++it) {
        TESObjectREFR* r = it.Get();
        if (!r || !r->refID || r->refID == kPlayerRefID) continue;
        // a placed object is a REFR whose BASE form has the object type (container, door); actors are their own ref type
        if (!nvworld::actorTypeMatches(formType, r->typeID, r->baseForm ? r->baseForm->typeID : 0)) continue;
        ActorRef a;
        a.refID = r->refID;
        a.baseID = refIdOf(r->baseForm);
        FormRef fr;
        if (formRef(r, &fr)) a.ref = nvhooks::refToStr(fr);
        out.push_back(a);
    }
    return out;
}

// ------------------------------------------------------------------ client apply
namespace {
bool cellIsReady(const FormRef& cellRef) {
    uint32_t id = nvhooks::resolver().fromRef(cellRef);
    TESObjectCELL* c = static_cast<TESObjectCELL*>(id ? nvhooks::resolver().lookup(id) : nullptr);
    if (!c) return false;
    return c == playerCell() || c->cellState == TESObjectCELL::kCellLoadState_Attached;
}
}  // namespace

nvhooks::ApplyStatus WorldApplier::run(const Plan& p) {
    using nvhooks::ApplyStatus;
    err_ = p.error;
    switch (p.status) {
        case PlanStatus::FormMissing: return ApplyStatus::FormMissing;
        case PlanStatus::Unsupported: return ApplyStatus::Unsupported;
        case PlanStatus::Queued: return ApplyStatus::Ok;  // accepted; created when the cell loads
        case PlanStatus::Ok: break;
    }
    for (const Step& s : p.steps) {
        // The gate is held per step, not around the whole plan: a console line is a script and a gated Script::Execute only pretends it ran.
        std::unique_ptr<nvhooks::ScriptGate> stepGate;
        if (gateSteps_ > 0 && !stepRunsScripts(s.kind)) stepGate.reset(new nvhooks::ScriptGate);
        switch (s.kind) {
            case StepKind::ConsoleOnRef:
            case StepKind::ConsoleGlobal: {
                if (!exec_) { err_ = "no console executor"; return ApplyStatus::Unsupported; }
                nvhooks::ApplierExecScope scope;
                {
                    void* t = s.refID ? res_.lookup(s.refID) : nullptr;
                    nvcoop::Log("world: apply '%s' on %08X (resolves=%d)", s.line.c_str(), s.refID, t ? 1 : 0);
                }
                if (!exec_(s.line, s.kind == StepKind::ConsoleOnRef ? s.refID : 0)) {
                    err_ = "console command failed: " + s.line;
                    return ApplyStatus::EngineRejected;
                }
                break;
            }
            case StepKind::AdvanceClock: {
                if (!g_advance) { err_ = "no clock handler (the world-time module is off)"; return ApplyStatus::Unsupported; }
                if (!g_advance(s.minutes)) { err_ = "the clock handler refused the skip"; return ApplyStatus::EngineRejected; }
                break;
            }
            case StepKind::SetRefVar: {
                TESObjectREFR* r = static_cast<TESObjectREFR*>(res_.lookup(s.refID));
                bool done = false;
                BSExtraData* xd = r ? r->extraDataList.GetByType(kExtraData_Script) : nullptr;
                ScriptEventList* el = xd ? static_cast<ExtraScript*>(xd)->eventList : nullptr;
                if (el && el->m_vars)
                    for (auto it = el->m_vars->Begin(); !it.End(); ++it)
                        if (it.Get() && it.Get()->id == s.varIndex) {
                            it.Get()->data = s.varValue;
                            done = true;
                            break;
                        }
                if (!done) { err_ = "ref has no such script variable (script not instantiated here?)"; return ApplyStatus::EngineRejected; }
                break;
            }
            case StepKind::CreateSpawn: {
                TESObjectCELL* cell = static_cast<TESObjectCELL*>(s.cellID ? res_.lookup(s.cellID) : nullptr);
                void* player = thePlayer();
                TESObjectREFR* anchor = nullptr;
                if (cell && cell == playerCell()) anchor = static_cast<TESObjectREFR*>(player);
                else if (cell)
                    for (auto it = cell->objectList.Begin(); !it.End() && !anchor; ++it)
                        if (it.Get() && it.Get()->refID) anchor = it.Get();
                if (!anchor) anchor = static_cast<TESObjectREFR*>(player);
                if (!anchor) { err_ = "no anchor reference to place from"; return ApplyStatus::EngineRejected; }
                if (!callsVerified()) { err_ = "engine PlaceAtMe worker prologue differs; refusing to call it"; return ApplyStatus::EngineRejected; }
                TESForm* base = static_cast<TESForm*>(res_.lookup(s.baseID));
                if (!base) { err_ = "base form missing"; return ApplyStatus::FormMissing; }
                // Cmd_PlaceAtMe's own worker: cdecl(ref, form, count, distance, direction, health) -> first placed ref
                TESObjectREFR* made = reinterpret_cast<TESObjectREFR*(__cdecl*)(TESObjectREFR*, TESForm*, UInt32, UInt32, UInt32, float)>(
                    world_addrs::Call_PlaceAtMe.addr)(anchor, base, 1, 0, 0, 1.0f);
                if (!made) { err_ = "PlaceAtMe produced no reference"; return ApplyStatus::EngineRejected; }
                const char axes[3] = {'X', 'Y', 'Z'};
                const float v[3] = {s.spawn.x, s.spawn.y, s.spawn.z};
                for (int i = 0; i < 3 && exec_; ++i) {
                    nvhooks::ApplierExecScope scope;
                    exec_(setPosLine(axes[i], v[i]), made->refID);
                }
                g_ledger.onCreated(s.spawn.ref(), made->refID);
                break;
            }
        }
    }
    return ApplyStatus::Ok;
}

namespace {
using nvhooks::ApplyStatus;
Outcome toOutcome(ApplyStatus s) {
    switch (s) {
        case ApplyStatus::Ok: return Outcome::Ok;
        case ApplyStatus::FormMissing: return Outcome::Missing;
        case ApplyStatus::EngineRejected: return Outcome::Rejected;
        default: return Outcome::Other;
    }
}
ApplyStatus toStatus(Outcome o) {
    switch (o) {
        case Outcome::Ok: return ApplyStatus::Ok;
        case Outcome::Missing: return ApplyStatus::FormMissing;
        case Outcome::Rejected: return ApplyStatus::EngineRejected;
        default: return ApplyStatus::Unsupported;
    }
}
// While alive, WorldApplier::run holds a ScriptGate around the steps that must not run scripts (everything but console lines).
struct GateSteps {
    int& n;
    explicit GateSteps(int& counter) : n(counter) { ++n; }
    ~GateSteps() { --n; }
    GateSteps(const GateSteps&) = delete;
    GateSteps& operator=(const GateSteps&) = delete;
};
}  // namespace

ApplyStatus WorldApplier::applyAny(const Effect& e) {
    if (auto* os = std::get_if<fnvcoop::ObjectState>(&e))
        if (g_objObserver) g_objObserver(*os);   // the host's word, noted before the apply (the object may not be loaded here yet)
    fnvcoop::EchoGuard echo(&hub_.echo());
    nvhooks::ApplyScope applyScope;
    GateSteps gs(gateSteps_);
    PlanEnv env{res_, g_ledger, cellIsReady};
    Outcome o = applyRouted(
        e, env, [this](const Plan& p) { return toOutcome(run(p)); },
        [this](const Effect& x) { nvhooks::ScriptGate gate; return fallback_ ? toOutcome(fallback_(x)) : Outcome::Other; });
    return toStatus(o);
}

ApplyStatus WorldApplier::applyWorld(const WorldEffect& e) {
    if (auto* os = std::get_if<SetObjectState>(&e))
        if (g_objObserver) {   // the host's word, noted before the apply (the object may not be loaded here yet); the poll heals toward it
            fnvcoop::ObjectState o;
            o.ref = os->ref; o.mask = os->mask; o.open = os->open; o.locked = os->locked; o.level = os->level; o.destroyed = os->destroyed;
            g_objObserver(o);
        }
    fnvcoop::EchoGuard echo(&hub_.echo());
    nvhooks::ApplyScope applyScope;
    PlanEnv env{res_, g_ledger, cellIsReady};
    return toStatus(applyWorldRouted(e, env, [this](const Plan& p) { return toOutcome(run(p)); }));
}

ApplyStatus WorldApplier::apply(const Effect& e) { return applyAny(e); }

size_t WorldApplier::tick(bool retry) {
    fnvcoop::EchoGuard echo(&hub_.echo());
    nvhooks::ApplyScope applyScope;
    GateSteps gs(gateSteps_);
    PlanEnv env{res_, g_ledger, cellIsReady};
    RunPlan runner = [this](const Plan& p) { return toOutcome(run(p)); };
    FallbackApply other = [this](const Effect& x) { nvhooks::ScriptGate gate; return fallback_ ? toOutcome(fallback_(x)) : Outcome::Other; };
    size_t n = 0;
    for (const Plan& p : planQueuedSpawns(env))
        if (run(p) == ApplyStatus::Ok) ++n;
    n += flushDeferred(env, runner, other, retry);
    return n;
}

size_t WorldApplier::flushQueued() { return tick(false); }

size_t WorldApplier::applyBaseline(const BaselineSnapshot& b) {
    size_t n = 0;
    {
        fnvcoop::EchoGuard echo(&hub_.echo());
        nvhooks::ApplyScope applyScope;
        GateSteps gs(gateSteps_);
        PlanEnv env{res_, g_ledger, cellIsReady};
        std::vector<fnvcoop::SpawnRef> live;
        for (const Effect& e : b.effects)
            if (auto* sp = std::get_if<fnvcoop::SpawnRef>(&e)) live.push_back(*sp);
        run(planReconcile(live, env));  // spawns the host despawned while this client was away
    }
    for (const Effect& e : b.effects)
        if (applyAny(e) == ApplyStatus::Ok) ++n;
    for (const WorldEffect& w : b.world)
        if (applyWorld(w) == ApplyStatus::Ok) ++n;
    return n;
}

}  // namespace nvworld
