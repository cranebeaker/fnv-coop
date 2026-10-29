// Engine detours + effect capture glue. x86 / MSVC only (thiscall detours are written as __fastcall with a dummy EDX).
// Addresses and call conventions come from docs/engine-addresses.md (verified against FalloutNV.exe 1.4.0.525). A hook whose prologue bytes do not match is skipped, never forced.
#include "engine_hooks.h"

#include <Windows.h>

#include <array>
#include <cmath>
#include <cstdio>
#include <cstring>
#include <memory>
#include <unordered_map>
#include <utility>
#include <vector>

#include "GameAPI.h"
#include "GameData.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "apply_lines.h"
#include "apply_remote.h"
#include "change_tracker.h"
#include "party_cond.h"
#include "player_kill.h"
#include "inventory_diff.h"
#include "GameExtraData.h"
#include "effect_json.h"
#include "engine_addrs.h"
#include "extract_pad.h"
#include "faction_removal.h"
#include "fnvcoop/policy.h"
#include "guest_item.h"
#include "guest_local_run.h"
#include "guest_objvars.h"
#include "kill_attrib.h"
#include "spectator_say.h"
#include "lock_preimage.h"
#include "ref_ops.h"
#include "ref_var.h"
#include "controls_route.h"
#include "story_addrs.h"

using namespace fnvcoop;

namespace nvhooks {
namespace {

// ------------------------------------------------------------------ real memory
struct Win32Memory : MemoryOps {
    bool read(Addr a, void* out, size_t n) override {
        __try {
            std::memcpy(out, reinterpret_cast<const void*>(a), n);
            return true;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
    }
    bool write(Addr a, const void* in, size_t n) override {
        DWORD old = 0;
        void* p = reinterpret_cast<void*>(a);
        if (!VirtualProtect(p, n, PAGE_EXECUTE_READWRITE, &old)) return false;
        std::memcpy(p, in, n);
        FlushInstructionCache(GetCurrentProcess(), p, n);
        DWORD tmp;
        VirtualProtect(p, n, old, &tmp);
        return true;
    }
    Addr allocExec(size_t n) override {
        void* p = VirtualAlloc(nullptr, n, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
        return reinterpret_cast<Addr>(p);
    }
    void freeExec(Addr a, size_t) override { VirtualFree(reinterpret_cast<void*>(a), 0, MEM_RELEASE); }
};

// ------------------------------------------------------------------ state
CaptureHub* g_hub = nullptr;
FormResolver* g_res = nullptr;
Win32Memory g_mem;
std::unique_ptr<HookRegistry> g_reg;
int g_gate = 0;
SuppressPolicy g_pol;  // all guest-suppression decisions (unit-tested in suppress_policy.cpp)
ActingContext g_acting;
SpectatorSayGate g_spectatorSay;   // the engine's own result-script run for a spectator's Say (spectator_say.h)
GuestItems g_guestItems;   // guest_item.h: a guest's hand-in is not taken from the host's pack
inline bool blockedNow() { return g_pol.blockMutation(); }
// Quest start / stage / objective calls: a guest's DLC setup script (allow-listed, per machine) may not make them (the host's story state), see SuppressPolicy::blockQuestState.
inline bool questBlockedNow() { return g_pol.blockQuestState(); }
ScriptStats g_sstats;
// Host: a DLC setup script (GuestLocalRun, per machine, its effects muted) calls StartQuest / SetStage / an objective call: those are the host's shared story state, so the mute is
// lifted for the call and an implicit script scope is opened, which makes the hub capture it (and the stage result it runs) like any other quest script's. Guests: nothing (their
// calls are dropped by questBlockedNow()).
struct QuestStateLift {
    bool lifted = false, weak = false;
    int saved = 0;
    QuestStateLift() {
        uint32_t script = 0;
        if (!g_hub || g_pol.guest() || !g_hub->muted() || !g_pol.inHostQuestStateScript(&script)) return;
        saved = g_hub->liftMute();
        lifted = true;
        weak = g_hub->beginWeak(script);
    }
    ~QuestStateLift() {
        if (!lifted) return;
        if (weak) g_hub->endWeak();
        g_hub->restoreMute(saved);
    }
    QuestStateLift(const QuestStateLift&) = delete;
    QuestStateLift& operator=(const QuestStateLift&) = delete;
};
int g_applierExec = 0;
DWORD g_mainTid = 0;   // the thread Init ran on (the game's main thread, DeferredInit): the only one whose LocalPlayerScope counts
Unmapped g_unmapped;
constexpr uint32_t kPlayerRefID = 0x14;
constexpr uint32_t kKarmaAV = 23;  // eActorVal_Karma (NVSE) -- confirmed: Player::RewardKarma passes 0x17 to the AV owner
constexpr Addr kThePlayerPtr = 0x11DEA3C;
constexpr Addr kDataHandlerPtr = 0x11C3F2C;

inline void* thePlayer() { return *reinterpret_cast<void**>(kThePlayerPtr); }
inline bool capturing() { return g_hub && g_hub->active(); }
inline uint32_t refIdOf(const void* form) { return form ? static_cast<const TESForm*>(form)->refID : 0; }

bool formRef(const void* form, FormRef* out) {
    uint32_t id = refIdOf(form);
    return id && g_res && g_res->toRef(id, out);
}
inline bool isPlayer(const void* ref) { return ref && refIdOf(ref) == kPlayerRefID; }

// engine call helpers (callee-clean thiscall)
template <class R, class... A>
inline R tc(Addr fn, void* self, A... a) {
    return reinterpret_cast<R(__thiscall*)(void*, A...)>(fn)(self, a...);
}
// float-returning virtual: ActorValueOwner::GetActorValue lives at vtable slot 3 (offset 0xC)
float getAV(void* actor, uint32_t av) {
    if (!actor) return 0;
    void* owner = reinterpret_cast<char*>(actor) + 0xA4;  // PlayerCharacter/Actor avOwner (see RewardKarma disassembly)
    return tc<float, uint32_t>(addr::ActorGetActorValue.target, owner, av);
}

// ------------------------------------------------------------------ default resolver
struct DefaultResolver : FormResolver {
    bool toRef(uint32_t refID, FormRef* out) override {
        DataHandler* dh = *reinterpret_cast<DataHandler**>(kDataHandlerPtr);
        if (!dh) return false;
        uint32_t idx = refID >> 24;
        const ModInfo* mi = dh->modList.modInfoList.GetNthItem(idx);
        if (!mi) return false;
        *out = makeRef(mi->name, refID & 0x00FFFFFFu);
        return true;
    }
    uint32_t fromRef(const FormRef& r) override {
        DataHandler* dh = *reinterpret_cast<DataHandler**>(kDataHandlerPtr);
        if (!dh) return 0;
        uint32_t i = 0;
        for (auto it = dh->modList.modInfoList.Begin(); !it.End(); ++it, ++i) {
            if (!it.Get()) continue;
            if (_stricmp(it.Get()->name, r.plugin.c_str()) == 0) return (i << 24) | (r.localId & 0x00FFFFFFu);
        }
        return 0;
    }
    void* lookup(uint32_t refID) override {
        using F = void*(__cdecl*)(uint32_t);
        return refID ? reinterpret_cast<F>(addr::LookupFormByID.target)(refID) : nullptr;
    }
};
DefaultResolver g_defaultResolver;

// ------------------------------------------------------------------ TESGlobal diff (no usable setter hook)
// `set MyGlobal to x` is executed inline by the script interpreter, so there is no function to detour. Instead the
// hub snapshots every TESGlobal (type 6 forms in DataHandler::globalList, float at +0x24) when the OUTERMOST capture
// scope opens and records SetGlobal for each one that differs when it closes.
std::vector<std::pair<TESGlobal*, float>> g_globalSnap;
struct VarSnap {
    TESQuest* q;
    uint32_t id;
    double v;
};
std::vector<VarSnap> g_varSnap;  // quest script variables (ScriptEventList at TESQuest+0x5C)
template <class F>
void forQuestVars(F f) {
    DataHandler* dh = *reinterpret_cast<DataHandler**>(kDataHandlerPtr);
    if (!dh) return;
    for (auto it = dh->questList.Begin(); !it.End(); ++it) {
        TESQuest* q = it.Get();
        ScriptEventList* el = q ? q->scriptEventList : nullptr;
        if (!el || !el->m_vars) continue;
        for (auto v = el->m_vars->Begin(); !v.End(); ++v)
            if (v.Get()) f(q, v.Get()->id, v.Get()->data);
    }
}
bool g_statePolling = false;
ChangeTracker g_tracker;
// A quest whose script is on the GuestLocalRun allow-list is per-machine: its variables are local state on every machine and are never
// captured, polled or sent.
inline bool isLocalQuest(TESQuest* q) {
    return q && q->scriptable.script && !g_pol.sharesQuestVars(refIdOf(q->scriptable.script));
}

void globalsBegin() {
    if (g_statePolling) return;
    g_globalSnap.clear();
    g_varSnap.clear();
    forQuestVars([](TESQuest* q, uint32_t id, double v) { if (!isLocalQuest(q)) g_varSnap.push_back({q, id, v}); });
    DataHandler* dh = *reinterpret_cast<DataHandler**>(kDataHandlerPtr);
    if (!dh) return;
    for (auto it = dh->globalList.Begin(); !it.End(); ++it)
        if (it.Get()) g_globalSnap.emplace_back(it.Get(), it.Get()->data);
}
void globalsEnd() {
    if (g_statePolling) return;
    if (!g_varSnap.empty()) {
        size_t i = 0;  // same traversal order as the snapshot unless variables were added/removed mid-script
        forQuestVars([&](TESQuest* q, uint32_t id, double v) {
            if (isLocalQuest(q)) return;
            if (i < g_varSnap.size() && g_varSnap[i].q == q && g_varSnap[i].id == id) {
                FormRef qr;
                if (g_varSnap[i].v != v && formRef(q, &qr)) g_hub->recordScriptVar({qr, id, v});  // -> Effect SetScriptVar{qr, id, v}
            }
            ++i;
        });
        g_varSnap.clear();
    }
    for (auto& [g, before] : g_globalSnap) {
        if (g->data == before) continue;
        FormRef gr;
        if (formRef(g, &gr)) g_hub->record(SetGlobal{gr, g->data});
    }
    g_globalSnap.clear();
}

// ------------------------------------------------------------------ detours
// Each detour: (1) cheap early-out when the hub is off, (2) record, (3) call the original via trampoline.

using Fn_SetStage = bool(__thiscall*)(TESQuest*, UInt8);
Fn_SetStage o_SetStage;
StageGate g_stageGate;
StageGateFn g_stageGateFn;
StageRequestObserver g_stageReqObs;
// Guest: the stage advance a script / console line just tried is not applied here (the host owns quest progress). It is not lost: the observer turns it
// into a request the host judges and, if it passes, runs for real and broadcasts. The guest's own copy changes when that txn arrives.
void forwardBlockedStage(TESQuest* q, UInt8 stage) {
    ++g_sstats.stagesBlocked;
    FormRef qr;
    if (!g_stageReqObs || !formRef(q, &qr)) return;
    if (g_stageReqObs(qr, stage)) ++g_sstats.stagesForwarded;
}
bool __fastcall h_SetStage(TESQuest* q, void*, UInt8 stage) {
    if (questBlockedNow()) {
        if (g_pol.localScriptDepth() == 0) forwardBlockedStage(q, stage);   // a per-machine DLC setup script is not a request to the host: the host runs its own copy
        return false;
    }
    QuestStateLift lift;
    if (g_stageGateFn && g_hub && g_hub->enabled() && !g_stageGate.allow(q ? q->refID : 0, stage, g_pol.applying())) return false;
    if (!g_hub || !g_hub->enabled()) return o_SetStage(q, stage);
    Cause c;
    c.kind = CauseKind::QuestResult;
    c.aux = stage;
    FormRef qr;
    bool have = formRef(q, &qr);
    if (have) c.source = qr;
    ScopedCapture sc(*g_hub, c);
    if (have) g_hub->record(SetStage{qr, stage});  // on ENTRY: replicas replay stage first, then its consequences
    return o_SetStage(q, stage);
}

using Fn_TopicRun = void(__thiscall*)(void*, UInt32, void*);
Fn_TopicRun o_TopicRun;
void __fastcall h_TopicRun(void* info, void*, UInt32 onEnd, void* actor) {
    if (g_pol.blockResultScript()) return;  // guests never run INFO result scripts
    // The host plays a guest's line with `<npc>.Say`; the engine then runs the INFO's scripts again by itself. The runner's run (an acting window is open) is the only one.
    if (g_spectatorSay.blocks(refIdOf(actor), g_acting.depth() > 0, GetTickCount64())) {
        if (traceEnabled()) traceLine(std::string("capture: INFO result script ") + std::to_string(refIdOf(info)) + (onEnd ? " End" : " Begin") + " NOT run: the engine's copy for a spectator voice (speaker " + std::to_string(refIdOf(actor)) + ")");
        return;
    }
    if (!g_hub || !g_hub->enabled()) return o_TopicRun(info, onEnd, actor);
    Cause c;
    c.kind = CauseKind::DialogueResult;
    c.aux = static_cast<int32_t>(onEnd);
    FormRef ir;
    if (formRef(info, &ir)) c.source = ir;
    if (traceEnabled()) traceLine(std::string("capture: INFO result script ") + std::to_string(c.source.localId) + (onEnd ? " End" : " Begin"));
    ScopedCapture sc(*g_hub, c);
    o_TopicRun(info, onEnd, actor);
}

using Fn_ObjState = void(__thiscall*)(BGSQuestObjective*, UInt32);
Fn_ObjState o_ObjState;
void __fastcall h_ObjState(BGSQuestObjective* o, void*, UInt32 flags) {
    if (questBlockedNow()) return;
    QuestStateLift lift;
    FormRef qr;
    if (capturing() && o && formRef(o->quest, &qr))
        g_hub->record(SetObjective{qr, o->objectiveId, static_cast<uint8_t>(flags & 3u)});
    o_ObjState(o, flags);
}

using Fn_QuestBool = void(__thiscall*)(TESQuest*, bool);
Fn_QuestBool o_SetRunning, o_SetCompleted;
void __fastcall h_SetRunning(TESQuest* q, void*, bool on) {
    if (questBlockedNow()) return;
    QuestStateLift lift;
    FormRef qr;
    if (capturing() && formRef(q, &qr)) {
        if (on) g_hub->record(StartQuest{qr});
        else g_hub->record(StopQuest{qr});
    }
    o_SetRunning(q, on);
}
void __fastcall h_SetCompleted(TESQuest* q, void*, bool on) {
    if (questBlockedNow()) return;
    QuestStateLift lift;
    FormRef qr;
    if (capturing() && on && formRef(q, &qr)) g_hub->record(CompleteQuest{qr});
    o_SetCompleted(q, on);
}

// TESReputation: fields +0x44 = positive ("fame"), +0x48 = negative ("infamy"); wrappers diff the fields so the
// clamp/threshold logic of the engine is captured as the net result, not re-derived.
using Fn_Rep = void(__thiscall*)(void*, int, int);
using Fn_RepF = void(__thiscall*)(void*, int, float);
Fn_Rep o_RepAdd, o_RepRemove;
Fn_RepF o_RepSet, o_RepAddExact;
inline float& fame(void* r) { return *reinterpret_cast<float*>(reinterpret_cast<char*>(r) + 0x44); }
inline float& infamy(void* r) { return *reinterpret_cast<float*>(reinterpret_cast<char*>(r) + 0x48); }
// A guest acts (its INFO result, or any script the host runs for it) and the host is not applying a committed effect (the applier runs under an EchoGuard): what the script
// gives the player is the guest's, the host's player is not touched by the engine (guest_item.h planGuestReward); the session gives the host its share if the mask names it.
bool guestWindowNow() {
    if (!(g_guestItems.active() || !g_acting.playerTargetOf(kPlayerRefID).onHost)) return false;
    return !(g_hub && g_hub->echo().suppressed());
}
void recordRepDiff(void* rep, float f0, float i0) {
    FormRef fr;
    if (!capturing() || !formRef(rep, &fr)) return;
    int df = static_cast<int>(std::lround(fame(rep) - f0)), di = static_cast<int>(std::lround(infamy(rep) - i0));
    if (df || di) g_hub->record(ModReputation{fr, df, di, Recipient::dflt()});
    // the engine moved the standing already (clamps, thresholds, so the net is read back): a guest's reward is put back, the host's share is applied by the session
    if ((df || di) && !planGuestReward(guestWindowNow(), true, true).runEngine) {
        fame(rep) = f0;
        infamy(rep) = i0;
        if (traceEnabled()) traceLine("capture: reputation " + fr.plugin + ":" + std::to_string(fr.localId) + " " + std::to_string(df) + "/" + std::to_string(di) + " for a guest: the host's standing is put back");
    }
}
void __fastcall h_RepAdd(void* r, void*, int t, int l) { if (blockedNow()) return; float f = fame(r), i = infamy(r); o_RepAdd(r, t, l); recordRepDiff(r, f, i); }
void __fastcall h_RepRemove(void* r, void*, int t, int l) { if (blockedNow()) return; float f = fame(r), i = infamy(r); o_RepRemove(r, t, l); recordRepDiff(r, f, i); }
void __fastcall h_RepSet(void* r, void*, int t, float v) { if (blockedNow()) return; float f = fame(r), i = infamy(r); o_RepSet(r, t, v); recordRepDiff(r, f, i); }
void __fastcall h_RepAddExact(void* r, void*, int t, float v) { if (blockedNow()) return; float f = fame(r), i = infamy(r); o_RepAddExact(r, t, v); recordRepDiff(r, f, i); }

using Fn_Karma = void(__thiscall*)(void*, int);
Fn_Karma o_Karma;
void __fastcall h_Karma(void* player, void*, int amount) {
    if (blockedNow()) return;
    if (capturing() && amount) {
        g_hub->record(ModKarma{amount, Recipient::dflt()});
        if (!planGuestReward(guestWindowNow(), true, true).runEngine) {   // a guest's karma: the host's player does not get it from the engine
            if (traceEnabled()) traceLine("capture: RewardKarma " + std::to_string(amount) + " for a guest: the host's karma is not touched");
            return;
        }
    }
    o_Karma(player, amount);
}

using Fn_Disable = void(__thiscall*)(void*, bool);
Fn_Disable o_SetDisabled;
void __fastcall h_SetDisabled(void* ref, void*, bool disable) {
    if (blockedNow() && !isPlayer(ref)) return;
    FormRef rr;
    bool was = false, track = capturing() && !isPlayer(ref) && formRef(ref, &rr);
    if (track) was = tc<bool>(addr::RefIsDisabled.target, ref);
    o_SetDisabled(ref, disable);
    if (track && was != disable) {
        if (disable) g_hub->record(DisableRef{rr});
        else g_hub->record(EnableRef{rr});
    }
}

std::vector<std::pair<uint32_t, uint32_t>> g_localRunCounts;
void noteLocalRun(uint32_t id) {
    for (auto& e : g_localRunCounts) if (e.first == id) { ++e.second; return; }
    if (g_localRunCounts.size() < 64) g_localRunCounts.emplace_back(id, 1u);
}

// ---- guest: shared quest variables that an object / NPC / effect script writes inline (guest_objvars.h) ----
// The slot (ScriptLocal::data) of the quest variable `name`, plus its index; false when the quest has no script / event list / such a variable.
bool questVarSlot(void* questForm, const char* name, uint32_t* idxOut, double** slotOut) {
    __try {
        TESQuest* q = static_cast<TESQuest*>(questForm);
        Script* sc = q ? q->scriptable.script : nullptr;
        ScriptEventList* el = q ? q->scriptEventList : nullptr;
        if (!sc || !el || !el->m_vars) return false;
        bool found = false;
        UInt32 idx = 0;
        for (auto v = sc->varList.Begin(); !v.End(); ++v) {
            VariableInfo* vi = v.Get();
            if (vi && vi->name.m_data && _stricmp(vi->name.m_data, name) == 0) { idx = vi->idx; found = true; break; }
        }
        if (!found) return false;
        for (auto v = el->m_vars->Begin(); !v.End(); ++v) {
            if (v.Get() && v.Get()->id == idx) {
                *idxOut = idx;
                *slotOut = &v.Get()->data;
                return true;
            }
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
    return false;
}
struct GuardedScript {
    const std::vector<ObjVarWrite>* writes = nullptr;   // null: this script writes no shared quest variable
    std::vector<uint32_t> questIds;                      // parallel to *writes: engine refID of the quest (0 = plugin not loaded)
};
std::unordered_map<uint32_t, GuardedScript> g_guardedScripts;   // by script refID (negative answers included)
const GuardedScript& guardedScript(uint32_t scriptRefID) {
    auto it = g_guardedScripts.find(scriptRefID);
    if (it != g_guardedScripts.end()) return it->second;
    GuardedScript gs;
    FormRef sr;
    if (g_res && scriptRefID && g_res->toRef(scriptRefID, &sr)) {
        gs.writes = ObjectVarWrites::builtin().of(sr);
        if (gs.writes) for (const ObjVarWrite& w : *gs.writes) gs.questIds.push_back(g_res->fromRef(w.quest));
    }
    return g_guardedScripts.emplace(scriptRefID, std::move(gs)).first->second;
}
// Called on a guest before an object / NPC / effect script runs: remembers every shared quest variable it is known to write.
bool guestRememberQuestVars(uint32_t scriptRefID, VarRollback& rb) {
    const GuardedScript& gs = guardedScript(scriptRefID);
    if (!gs.writes) return false;
    ++g_sstats.questVarGuarded;
    for (size_t i = 0; i < gs.writes->size(); ++i) {
        const ObjVarWrite& w = (*gs.writes)[i];
        void* q = gs.questIds[i] && g_res ? g_res->lookup(gs.questIds[i]) : nullptr;
        if (!q || isLocalQuest(static_cast<TESQuest*>(q))) continue;   // GuestLocalRun quests keep their per-machine variables
        uint32_t idx = 0;
        double* slot = nullptr;
        if (!questVarSlot(q, w.var.c_str(), &idx, &slot)) continue;
        if (volatileKeys().isVolatileVar(w.quest, static_cast<uint16_t>(idx))) continue;   // per-machine by the volatile list
        rb.remember(slot);
    }
    return rb.size() > 0;
}

bool g_perfOn = false;
HookPerf g_perf;
inline uint64_t qpc() {
    LARGE_INTEGER v;
    QueryPerformanceCounter(&v);
    return static_cast<uint64_t>(v.QuadPart);
}

// A console / bridge line runs as a temporary script (refID 0 or 0xFF......) that keeps its source text (Script::text, runtime
// offset 0x2C, xNVSE GameScript.h). On a guest, a single line on the local player is per-machine (suppress_policy.h), and so is any
// console line run inside a LocalPlayerScope (SuppressPolicy::runsLocal).
const char* consoleLineText(void* script) {
    if (!script) return nullptr;
    __try {
        return *reinterpret_cast<const char* const*>(reinterpret_cast<char*>(script) + 0x2C);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}
// ---- W3d item 1: scripts that run once per joined slot (per_slot_run.h) ----
// The slot of the variable `name` in the event list the engine passes to Script::Execute (`script` is the Script, `ev` its ScriptEventList). POD only: SEH.
bool eventListVarSlot(void* scriptP, void* evP, const char* name, double** slotOut) {
    __try {
        Script* sc = static_cast<Script*>(scriptP);
        ScriptEventList* el = static_cast<ScriptEventList*>(evP);
        if (!sc || !el || !el->m_vars) return false;
        bool found = false;
        UInt32 idx = 0;
        for (auto v = sc->varList.Begin(); !v.End(); ++v) {
            VariableInfo* vi = v.Get();
            if (vi && vi->name.m_data && _stricmp(vi->name.m_data, name) == 0) { idx = vi->idx; found = true; break; }
        }
        if (!found) return false;
        for (auto v = el->m_vars->Begin(); !v.End(); ++v) {
            if (v.Get() && v.Get()->id == idx) {
                *slotOut = &v.Get()->data;
                return true;
            }
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
    return false;
}
std::vector<double*> objVarSlots(void* script, void* ev, const std::vector<std::string>& names) {
    std::vector<double*> out(names.size(), nullptr);
    for (size_t i = 0; i < names.size(); ++i) {
        double* p = nullptr;
        if (eventListVarSlot(script, ev, names[i].c_str(), &p)) out[i] = p;
    }
    return out;
}
PerSlotScripts g_perSlot;
PerSlotRunner g_perSlotRunner;
ObjVarCopies g_objCopies;
PerSlotStats g_perSlotStats;
int g_perSlotDepth = 0;
inline uint64_t instanceOf(uint32_t scriptId, void* thisObj) { return objVarInstance(scriptId, thisObj ? refIdOf(thisObj) : 0u); }   // not the event list's address: that changes with every load

using Fn_ScriptExec = bool(__thiscall*)(void*, void*, void*, void*, bool);
Fn_ScriptExec o_ScriptExec;

// The same script once more for every joined guest with a puppet, right after the engine's own run (which is the host's). Each run is its own weak capture
// scope, so its txn is committed while the session's window for that slot is open (the origin is the slot), and `player` inside it is that guest's puppet.
void runPerSlot(const PerSlotScript& ps, void* script, void* thisObj, void* ev, void* contObj, bool arg3) {
    ++g_perSlotDepth;
    const uint64_t instance = instanceOf(ps.scriptId, thisObj);
    g_perSlotRunner(ps.scriptId, [&](uint8_t slot, uint32_t ghost) {
        ActingPlayerScope acting(ghost);
        std::vector<double*> live;
        ObjVarCopies::Bracket br;
        if (ev && !ps.vars.empty()) {
            live = objVarSlots(script, ev, ps.vars);
            g_objCopies.enter(instance, slot, ps.vars, live, br);
            ++g_perSlotStats.objVarSwaps;
        }
        const bool weak = g_hub && g_hub->enabled() && g_hub->beginWeak(ps.scriptId);
        g_pol.enterScript();
        o_ScriptExec(script, thisObj, ev, contObj, arg3);
        g_pol.leaveScript();
        if (weak) g_hub->endWeak();
        if (!live.empty()) g_objCopies.leave(instance, slot, ps.vars, live, br);
        ++g_perSlotStats.runs;
    });
    --g_perSlotDepth;
}

ScriptRunObserver g_scriptRunObs;
ScriptRunScopeHook g_scriptRunScope;

bool __fastcall h_ScriptExec(void* script, void*, void* thisObj, void* ev, void* contObj, bool arg3) {
    const uint64_t t0 = g_perfOn ? qpc() : 0;
    if (g_gate > 0) {  // gate closed: pretend the script ran
        ++g_sstats.gated;
        return true;
    }
    if (g_pol.skipScript(*reinterpret_cast<uint16_t*>(reinterpret_cast<char*>(script) + 0x28), refIdOf(script))) return true;  // Script::info.type + GuestLocalRun
    if (g_applierExec > 0) {
        ++g_sstats.applierLines;
    } else {
        ++g_sstats.ran;
        if (g_pol.applying()) ++g_sstats.ranWhileApplying;
    }
    // Host: any script that runs outside an explicit capture scope (quest GameMode/MenuMode blocks, object and effect scripts) gets an
    // implicit weak scope so its mutations are captured (cause = Script / the script form) instead of dropped as ambient.
    const bool local = g_pol.runsLocal(refIdOf(script), [&] { return consoleLineText(script); });
    if (local && g_hub) g_hub->pushMute();  // per-machine: effects stay local, never captured
    bool weak = g_hub && g_hub->enabled() && g_applierExec == 0 && g_hub->beginWeak(refIdOf(script));
    if (local) {
        ++g_sstats.localRan;
        noteLocalRun(refIdOf(script));
        g_pol.enterLocalScript(refIdOf(script));
    } else {
        g_pol.enterScript();
    }
    // Guest: the host owns quest variables. An object / NPC / effect script that writes one inline (nothing to detour) has its writes put back.
    VarRollback rollback;
    const bool rolled = g_pol.guest() && !local && g_applierExec == 0 && !g_pol.applying() &&
                        (*reinterpret_cast<uint16_t*>(reinterpret_cast<char*>(script) + 0x28) != kScriptTypeQuest) &&
                        guestRememberQuestVars(refIdOf(script), rollback);
    // W3d: a script a PerPlayerState rule runs once per slot. Only the engine's own run (not an applier line, not a re-run, not a per-machine script) is followed by the re-runs.
    const PerSlotScript* perSlot = (!g_perSlot.empty() && g_perSlotRunner && !g_pol.guest() && !local && g_applierExec == 0 && g_perSlotDepth == 0) ? g_perSlot.find(refIdOf(script)) : nullptr;
    if (perSlot && ev && !perSlot->vars.empty()) g_objCopies.seed(instanceOf(perSlot->scriptId, thisObj), perSlot->vars, objVarSlots(script, ev, perSlot->vars));   // the pristine values, before the first run
    const uint64_t t1 = g_perfOn ? qpc() : 0;
    if (g_scriptRunScope) g_scriptRunScope(refIdOf(script), true);
    bool r = o_ScriptExec(script, thisObj, ev, contObj, arg3);
    if (g_scriptRunScope) g_scriptRunScope(refIdOf(script), false);
    if (g_scriptRunObs && thisObj) g_scriptRunObs(refIdOf(thisObj));
    const uint64_t t2 = g_perfOn ? qpc() : 0;
    if (rolled) g_sstats.questVarRollbacks += rollback.restore();
    if (local) g_pol.leaveLocalScript(refIdOf(script)); else g_pol.leaveScript();
    if (weak) g_hub->endWeak();
    if (local && g_hub) g_hub->popMute();
    if (perSlot) runPerSlot(*perSlot, script, thisObj, ev, contObj, arg3);
    if (g_perfOn) {
        ++g_perf.calls;
        g_perf.overheadTicks += (t1 - t0) + (qpc() - t2);
    }
    return r;
}

ActivateObserver g_activateObs;
using Fn_Activate = UInt32(__thiscall*)(void*, void*, UInt8, UInt32, UInt32);
Fn_Activate o_Activate;
UInt32 __fastcall h_Activate(void* ref, void*, void* activator, UInt8 a, UInt32 b, UInt32 c) {
    if (g_activateObs && g_pol.guest() && !g_pol.applying() && g_pol.scriptDepth() == 0 && activator &&
        refIdOf(activator) == kPlayerRefID) {
        FormRef fr;
        if (formRef(ref, &fr) && g_activateObs(fr) == ActivateAction::Suppress) return 0;
    }
    return o_Activate(ref, activator, a, b, c);
}

// CompleteAllObjectives: Cmd_CompleteAllObjectives (0x5C9740) -> TESQuest::CompleteAllObjectives(quest, 1) @0x60C950, ret 4.
Fn_QuestBool o_CompleteAll;
void __fastcall h_CompleteAll(TESQuest* q, void*, UInt32 a) {
    if (questBlockedNow()) return;
    QuestStateLift lift;
    FormRef qr;
    if (capturing() && formRef(q, &qr)) g_hub->record(CompleteAllObjectives{qr});
    reinterpret_cast<void(__thiscall*)(TESQuest*, UInt32)>(o_CompleteAll)(q, a);
}

// Disable/EnablePlayerControls both end in PlayerCharacter::SetControlsDisabled(bool disable, u8 mask) @0x95F530 (ret 8):
// player+0x680 |= mask (disable) or &= ~mask (enable). Clean primitive, so capture here rather than in the commands.
using Fn_Controls = void(__thiscall*)(void*, UInt32, UInt32);
Fn_Controls o_Controls;
ControlsRouter g_controlsRouter;
void __fastcall h_Controls(void* player, void*, UInt32 disable, UInt32 mask) {
    if (blockedNow()) return;
    const bool cap = capturing() && (mask & 0xFF);
    if (cap) g_hub->record(SetPlayerControls{mask & 0xFFu, disable == 0, Recipient::dflt()});
    // A guest's scene locks THAT guest, not the host (scope=initiator): the effect is recorded (the rules engine addresses it to the initiator), the host's own call is not made.
    if (skipHostControlsCall(cap, g_acting.slot(), disable != 0, g_controlsRouter)) return;
    o_Controls(player, disable, mask);
}

// MarkBaseExtraListScriptEvent(TESForm* object, BaseExtraList* targetList, UInt32 mask) @0x5AC750, cdecl, returns bool.
EventObserver g_eventObs;
using Fn_MarkEvent = bool(__cdecl*)(void*, void*, UInt32);
Fn_MarkEvent o_MarkEvent;
// Is `xlist` the extraDataList embedded in a reference? Returns that reference or nullptr. Raw pointers only (SEH-safe, no C++ objects).
static void* refFromExtraList(void* xlist) {
    char* cand = static_cast<char*>(xlist) - 0x44;
    __try {
        uint8_t t = static_cast<TESForm*>(static_cast<void*>(cand))->typeID;
        bool isRef = (t == 0x3A || t == 0x3B || t == 0x3C) && &static_cast<TESObjectREFR*>(static_cast<void*>(cand))->extraDataList == xlist;
        return isRef ? cand : nullptr;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}
bool __cdecl h_MarkEvent(void* object, void* xlist, UInt32 mask) {
    if (g_eventObs && g_pol.guest() && !g_pol.applying() && g_pol.scriptDepth() == 0 && object && refIdOf(object) == kPlayerRefID && xlist) {
        void* ref = refFromExtraList(xlist);
        FormRef tr;
        if (ref && formRef(ref, &tr) && g_eventObs(mask, tr) == EventAction::Suppress) return false;
    }
    // Host: a host-owned NPC starts combat with a guest's puppet. The block that runs must see that guest as `player` and its effects belong to an
    // Intent cause naming the NPC, so run the original as the guest inside a capture scope (ActingContext::hostPuppetEvent says which events).
    if (g_hub && g_hub->enabled() && !g_pol.guest() && !g_pol.applying() && object && xlist && g_acting.hostPuppetEvent(refIdOf(object), mask)) {
        void* ref = refFromExtraList(xlist);
        FormRef tr;
        if (ref && formRef(ref, &tr)) {
            Cause c;
            c.kind = CauseKind::Intent;
            c.source = tr;
            c.aux = static_cast<int32_t>(mask);
            g_acting.push(refIdOf(object));
            bool r;
            {
                ScopedCapture sc(*g_hub, c);
                r = o_MarkEvent(object, xlist, mask);
            }
            g_acting.pop();
            return r;
        }
    }
    return o_MarkEvent(object, xlist, mask);
}

// Actor::SetPlayerTeammate(bool) @0x8BCA90 (ret 4; flag at actor+0x18D, adds/removes the actor in the player's teammate list).
// Captured as CompanionOwner{npc, owner = acting player's slot, active = on, kind = humanoid/critter}. owner is the ActingPlayerScope slot
// (0 = host's player). Apply runs `SetPlayerTeammate` only on the owner's machine (RemoteApplier::companionForMe).
Fn_Disable o_Teammate;
void __fastcall h_Teammate(void* actor, void*, bool on) {
    if (blockedNow()) return;
    FormRef nr;
    bool known = capturing() && actor && !isPlayer(actor) && formRef(actor, &nr);
    bool was = known && *reinterpret_cast<uint8_t*>(static_cast<char*>(actor) + 0x18D) != 0;
    o_Teammate(actor, on);
    if (known && was != on) {
        CompanionOwner c;
        c.npc = nr;
        c.owner = actingSlot();
        c.active = on;
        c.kind = static_cast<TESForm*>(actor)->typeID == 0x3C /*ACRE*/ ? 1 : 0;  // creature companions count against the critter cap
        g_hub->record(c);
    }
}

// FailQuest primitive (QuestStageItem::RunResult / script): blocked on guests, no Effect type yet.
Fn_QuestBool o_SetFailed;
void __fastcall h_SetFailed(TESQuest* q, void*, bool on) {
    if (blockedNow()) return;
    FormRef qr;
    if (capturing() && on && formRef(q, &qr)) g_hub->record(FailQuest{qr});
    o_SetFailed(q, on);
}

// ---- script-command level hooks (things with no usable single primitive). Standard NVSE command signature. ----
using Fn_Cmd = bool(__cdecl*)(void* pi, void* sd, void* thisObj, void* contObj, void* script, void* ev, double* res, UInt32* opc);
using Fn_Extract = bool(__cdecl*)(void* pi, void* sd, UInt32* opc, void* thisObj, void* contObj, void* script, void* ev, ...);
inline Fn_Extract extractArgs() { return reinterpret_cast<Fn_Extract>(addr::ExtractArgs.target); }

// Commands with no Effect type: pass-through on the host, dropped on a guest when a local script calls them.
Fn_Cmd o_SetFactionRank, o_ModFactionRank, o_AddToFaction, o_RemoveFromFaction, o_AddPerk, o_RemovePerk, o_Lock, o_UnLock,
    o_SetOpenState, o_SetDestroyed, o_AddNote, o_RemoveNote, o_AddTopic, o_SetWeather, o_ForceWeather, o_SetTimeMult, o_Autosave,
    o_SystemSave, o_ForceSave, o_RemoveAllItems, o_PlaceAtMe, o_PlaceLeveled,
        o_AddScriptPackage, o_RemoveScriptPackage, o_StartCombat, o_StopCombat, o_ActivateCmd;
Fn_Cmd o_RemoveAllTyped, o_MoveToFade, o_SetPos, o_SetAngle, o_EnableCmd, o_DisableCmd;
Fn_Cmd o_AddItem, o_RemoveItem, o_RewardXP, o_Kill, o_Resurrect, o_MoveTo, o_ModAV, o_SetAV, o_ForceAV, o_DamageAV;

#define CMD_ARGS void* pi, void* sd, void* thisObj, void* contObj, void* script, void* ev, double* res, UInt32* opc
#define CMD_FWD(o) o(pi, sd, thisObj, contObj, script, ev, res, opc)

#define BLOCK_CMD(name)                                                      \
    bool __cdecl h_##name(CMD_ARGS) {                                        \
        if (blockedNow()) return true;                                       \
        return CMD_FWD(o_##name);                                            \
    }

// ---- story capture: the generic RefCommand hooks (ref_ops.h) --------------------------------------------------------------------------
bool g_storyOn = false;
// One detour for every story command. Arguments are decoded BEFORE the original runs (a command may delete what it was given), the command is
// recorded after it succeeded. With story capture off the three shared handlers (StartCombat, StopCombat, Activate) behave as they always did.
bool storyCmd(const RefOpSpec* s, Fn_Cmd o, void* pi, void* sd, void* thisObj, void* contObj, void* script, void* ev, double* res, UInt32* opc) {
    if (blockedNow()) return true;
    if (!g_storyOn || !s || !capturing()) return CMD_FWD(o);
    uint32_t raw[kMaxStoryArgs] = {};
    char text[kMaxStoryArgs][kStoryTextMax] = {};   // a text parameter is copied into a buffer (xNVSE: char name[256]; ExtractArgs(..., &name))
    const size_t n = std::strlen(s->sig);
    void* out[kMaxStoryArgs];
    for (int k = 0; k < kMaxStoryArgs; ++k) out[k] = (static_cast<size_t>(k) < n && s->sig[k] == 's') ? static_cast<void*>(text[k]) : static_cast<void*>(&raw[k]);
    UInt32 off = *opc;  // ExtractArgs advances the offset: work on a copy so the real call sees the original
    // kMaxStoryArgs output pointers whatever the command takes: ExtractArgs fills only as many as the command has parameters
    const bool parsed = n == 0 || extractPadded(extractArgs(), pi, sd, &off, thisObj, contObj, script, ev, {out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7], out[8]});
    RefCommand rc;
    bool have = false;
    if (parsed) {
        ArgValue av[kMaxStoryArgs];
        for (size_t k = 0; k < n && k < static_cast<size_t>(kMaxStoryArgs); ++k) {
            switch (s->sig[k]) {
                case 'o': av[k].form = raw[k] ? refIdOf(reinterpret_cast<void*>(static_cast<uintptr_t>(raw[k]))) : 0; break;
                case 'f': std::memcpy(&av[k].f, &raw[k], sizeof(float)); break;
                case 's': text[k][kStoryTextMax - 1] = 0; av[k].s = text[k]; break;
                default: av[k].i = static_cast<int32_t>(raw[k]); break;
            }
        }
        have = buildRefCommand(*s, refIdOf(thisObj), av, [](uint32_t id, FormRef* out) { return g_res && g_res->toRef(id, out); }, &rc);
        // a command on anything but the player (actor, container, list, quest) concerns every machine; one on the player follows the reward rule of the policy
        if (have && rc.to.mode == Recipient::Default && !isPlayer(thisObj)) rc.to = Recipient::all();
    }
    const bool ok = CMD_FWD(o);
    if (ok && have && capturing()) g_hub->record(rc);
    else if (ok && parsed) ++g_unmapped.storyDropped;
    return ok;
}
// ForceActiveQuest <quest>: an existing Effect type (the journal focus of the acting player).
Fn_Cmd o_ForceActiveQuest;
bool __cdecl h_ForceActiveQuest(void* pi, void* sd, void* thisObj, void* contObj, void* script, void* ev, double* res, UInt32* opc) {
    if (blockedNow()) return true;
    TESForm* q = nullptr;
    FormRef qr;
    UInt32 off = *opc;
    const bool have = capturing() && extractPadded(extractArgs(), pi, sd, &off, thisObj, contObj, script, ev, {&q}) && q && formRef(q, &qr);
    const bool ok = CMD_FWD(o_ForceActiveQuest);
    if (ok && have) g_hub->record(ForceActiveQuest{qr, Recipient::dflt()});   // the host policy narrows it to the acting player
    return ok;
}
template <size_t N>
struct StoryDet {
    static inline Fn_Cmd orig = nullptr;
    static bool __cdecl det(void* pi, void* sd, void* thisObj, void* contObj, void* script, void* ev, double* res, UInt32* opc) {
        return storyCmd(&kStorySpecs[N], orig, pi, sd, thisObj, contObj, script, ev, res, opc);
    }
};
constexpr size_t kStoryN = fnvcoop::kOpCount_ - 1;   // one spec per op (checked at Init and by the unit tests)
struct StoryEntry {
    const RefOpSpec* spec;
    void* detour;
    void* origOut;
};
template <size_t... I>
std::array<StoryEntry, sizeof...(I)> makeStoryEntries(std::index_sequence<I...>) {
    return {{StoryEntry{&kStorySpecs[I], reinterpret_cast<void*>(&StoryDet<I>::det), reinterpret_cast<void*>(&StoryDet<I>::orig)}...}};
}

// AddScriptPackage / RemoveScriptPackage: observer (L12 world state) called after the original succeeded, host capturing only.
ScriptPackageObserver g_pkgObs;
bool pkgCmd(Fn_Cmd o, bool add, CMD_ARGS) {
    if (blockedNow()) return true;
    TESForm* pkg = nullptr;
    bool want = g_pkgObs && capturing() && thisObj;
    if (want) {
        UInt32 off = *opc;
        want = extractPadded(extractArgs(), pi, sd, &off, thisObj, contObj, script, ev, {&pkg}) && pkg;
    }
    bool ok = CMD_FWD(o);
    FormRef ar, pr;
    if (ok && want && formRef(thisObj, &ar) && formRef(pkg, &pr)) g_pkgObs(ar, pr, add);
    return ok;
}
bool __cdecl h_AddScriptPackage(CMD_ARGS) { return pkgCmd(o_AddScriptPackage, true, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_RemoveScriptPackage(CMD_ARGS) { return pkgCmd(o_RemoveScriptPackage, false, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_ModFactionRank(CMD_ARGS) { return storyCmd(storySpecFor(kOpModFactionRank), o_ModFactionRank, pi, sd, thisObj, contObj, script, ev, res, opc); }
// StartCombat / StopCombat / Activate: blocked on a guest's local script as before; with story capture on they are also recorded.
bool __cdecl h_StartCombat(CMD_ARGS) { return storyCmd(storySpecFor(kOpStartCombat), o_StartCombat, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_StopCombat(CMD_ARGS) { return storyCmd(storySpecFor(kOpStopCombat), o_StopCombat, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_ActivateCmd(CMD_ARGS) { return storyCmd(storySpecFor(kOpActivate), o_ActivateCmd, pi, sd, thisObj, contObj, script, ev, res, opc); }

// ---- command-level captures for the census backlog (ExtractArgs re-parses the same script data on a COPY of the offset) ----
template <class... Out>
bool parseArgs(CMD_ARGS, Out*... outs) {
    UInt32 off = *opc;
    return extractPadded(extractArgs(), pi, sd, &off, thisObj, contObj, script, ev, {static_cast<void*>(outs)...});
}
inline FormRef actorRef(void* thisObj) {
    FormRef r;
    formRef(thisObj, &r);
    return r;
}

// Faction membership: SetFactionRank / AddToFaction (faction, rank), RemoveFromFaction (faction) -> SetFactionRank{actor, faction, rank<0 = removed}
bool factionCmd(Fn_Cmd o, int kind /*0 set/add, 1 remove*/, CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef ar, fr;
    TESForm* fac = nullptr;
    int rank = 0;
    bool track = capturing() && thisObj && formRef(thisObj, &ar);
    bool parsed = track && (kind == 0 ? parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &fac, &rank)
                                      : parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &fac));
    bool ok = CMD_FWD(o);
    if (ok && parsed && fac && formRef(fac, &fr)) g_hub->record(SetFactionRank{ar, fr, kind == 1 ? -1 : rank});
    return ok;
}
bool __cdecl h_SetFactionRank(CMD_ARGS) { return factionCmd(o_SetFactionRank, 0, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_AddToFaction(CMD_ARGS) { return factionCmd(o_AddToFaction, 0, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_RemoveFromFaction(CMD_ARGS) { return factionCmd(o_RemoveFromFaction, 1, pi, sd, thisObj, contObj, script, ev, res, opc); }
// ModFactionRank is a DELTA: recorded as the command itself (RefCommand ModFactionRank) when story capture is on, see h_ModFactionRank below storyCmd

// Perks (player): AddPerk / RemovePerk (perk, alt) -> SetPerk{perk, rank 1 | 0}
bool perkCmd(Fn_Cmd o, bool add, CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef pr;
    TESForm* perk = nullptr;
    int alt = 0;
    bool track = capturing() && isPlayer(thisObj);
    bool parsed = track && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &perk, &alt);
    bool ok = CMD_FWD(o);
    if (ok && parsed && perk && formRef(perk, &pr)) g_hub->record(SetPerk{pr, add ? 1 : 0, Recipient::dflt()});
    return ok;
}
bool __cdecl h_AddPerk(CMD_ARGS) { return perkCmd(o_AddPerk, true, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_RemovePerk(CMD_ARGS) { return perkCmd(o_RemovePerk, false, pi, sd, thisObj, contObj, script, ev, res, opc); }

// Lock state of a reference through the engine's own condition evaluators GetLocked 0x59D010 / GetLockLevel 0x59D100 (cdecl bool(thisObj, p1, p2,
// double*)); not hooked by us, called straight. Raw pointers under SEH.
bool readLockSeh(void* ref, double* locked, double* level) {
    using Eval = bool(__cdecl*)(void*, void*, void*, double*);
    __try {
        *locked = 0;
        *level = 0;
        reinterpret_cast<Eval>(addr::Cond_GetLocked.target)(ref, nullptr, nullptr, locked);
        reinterpret_cast<Eval>(addr::Cond_GetLockLevel.target)(ref, nullptr, nullptr, level);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
void readLock(void* ref, bool* wasLocked, int* wasLevel) {
    double l = 0, v = 0;
    if (readLockSeh(ref, &l, &v)) { *wasLocked = l != 0; *wasLevel = static_cast<int>(v); }
}

// Object state: Lock [level] / UnLock / SetOpenState n / SetDestroyed n
bool objectStateCmd(Fn_Cmd o, int kind /*0 lock 1 unlock 2 open 3 destroyed*/, CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef rr;
    int a = -1, b = 0;
    bool track = capturing() && thisObj && formRef(thisObj, &rr);
    bool parsed = track && ((kind == 0) ? parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &a, &b)
                                        : (kind == 1) ? true : parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &a));
    bool wasLocked = false;
    int wasLevel = 0;
    if (parsed && kind <= 1) readLock(thisObj, &wasLocked, &wasLevel);   // pre-image of the lock, read before the original changes it
    bool ok = CMD_FWD(o);
    if (ok && parsed) {
        ObjectState os;
        os.ref = rr;
        if (kind == 0) { os.mask = kObjLock; os.locked = true; os.level = a < 0 ? 0 : a; }
        else if (kind == 1) { os.mask = kObjLock; os.locked = false; }
        else if (kind == 2) { os.mask = kObjOpen; os.open = a != 0; }
        else { os.mask = kObjDestroyed; os.destroyed = a != 0; }
        if (kind <= 1) applyLockPreImage(&os, wasLocked, wasLevel);
        g_hub->record(os);
    }
    return ok;
}
bool __cdecl h_Lock(CMD_ARGS) { return objectStateCmd(o_Lock, 0, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_UnLock(CMD_ARGS) { return objectStateCmd(o_UnLock, 1, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_SetOpenState(CMD_ARGS) { return objectStateCmd(o_SetOpenState, 2, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_SetDestroyed(CMD_ARGS) { return objectStateCmd(o_SetDestroyed, 3, pi, sd, thisObj, contObj, script, ev, res, opc); }

// Journal / weather / time / scripted saves
enum class JournalKind { AddNote, RemoveNote, AddTopic, SetWeather, ForceWeather };
bool journalCmd(Fn_Cmd o, JournalKind k, CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef fr;
    TESForm* f = nullptr;
    int opt = 0;
    bool parsed = capturing() && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &f, &opt);
    bool ok = CMD_FWD(o);
    if (ok && parsed && f && formRef(f, &fr)) {
        switch (k) {
            case JournalKind::AddNote: break;   // recorded by h_PlayerAddNote (the primitive every path ends in: script, pickup, Pip-Boy)
            case JournalKind::RemoveNote: g_hub->record(RemoveNote{fr}); break;
            case JournalKind::AddTopic: g_hub->record(AddTopic{fr}); break;
            case JournalKind::SetWeather: g_hub->record(SetWeather{fr, false}); break;
            case JournalKind::ForceWeather: g_hub->record(SetWeather{fr, true}); break;
        }
    }
    return ok;
}
bool __cdecl h_AddNote(CMD_ARGS) { return journalCmd(o_AddNote, JournalKind::AddNote, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_RemoveNote(CMD_ARGS) { return journalCmd(o_RemoveNote, JournalKind::RemoveNote, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_AddTopic(CMD_ARGS) { return journalCmd(o_AddTopic, JournalKind::AddTopic, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_SetWeather(CMD_ARGS) { return journalCmd(o_SetWeather, JournalKind::SetWeather, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_ForceWeather(CMD_ARGS) { return journalCmd(o_ForceWeather, JournalKind::ForceWeather, pi, sd, thisObj, contObj, script, ev, res, opc); }

// PlayerCharacter::AddNote(note, u8 verbose) @0x966A70, thiscall, ret 8: the one function behind `AddNote` (Cmd_AddNote 0x5D5230) AND the pickup of a
// note object (callers 0x4C37D0, 0x5E9360, 0x757F70, 0x892520). Hooking it, not the command, is what makes a looted Jessup note a journal entry
// that every machine learns about (G10). The note list is player+0x5E4; HasNote @0x966C10 (thiscall, ret 4) tells whether the add took.
using Fn_PlayerAddNote = UInt32(__thiscall*)(void*, void*, UInt8);
Fn_PlayerAddNote o_PlayerAddNote;
NoteObserver g_noteObs;
bool playerHasNote(void* player, void* note) {
    return player && note && (reinterpret_cast<UInt32(__thiscall*)(void*, void*)>(addr::PlayerHasNote.target)(player, note) & 0xFF) != 0;
}
UInt32 __fastcall h_PlayerAddNote(void* player, void*, void* note, UInt8 verbose) {
    const bool watch = note && (g_hub || g_noteObs);
    const bool had = watch && playerHasNote(player, note);
    const UInt32 r = o_PlayerAddNote(player, note, verbose);
    if (!watch || had || !playerHasNote(player, note)) return r;
    FormRef nr;
    if (!formRef(note, &nr)) return r;
    if (g_hub && g_hub->enabled() && !g_hub->echo().suppressed()) {
        // Host. Inside a capture scope the note rides the scope's txn; an ambient pickup (the host player looting a note, a guest's pickup intent
        // run on the host) gets a one-effect weak scope of its own.
        const bool weak = !capturing() && g_hub->beginWeak(0);
        if (capturing()) g_hub->record(AddNote{nr});
        if (weak) g_hub->endWeak();
    } else if (g_noteObs && !g_pol.applying() && g_applierExec == 0 && g_pol.scriptDepth() == 0) {
        g_noteObs(nr);   // a machine that does not capture (a guest) picked a note up by itself: the session forwards it to the host
    }
    return r;
}

// ShowMap marker [enableTravel]: reveals a map marker (shared journal state), optionally enabling fast travel to it.
Fn_Cmd o_ShowMap;
bool __cdecl h_ShowMap(CMD_ARGS) {
    if (blockedNow()) return true;
    TESObjectREFR* marker = nullptr;
    int travel = 0;
    bool parsed = capturing() && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &marker, &travel);
    bool ok = CMD_FWD(o_ShowMap);
    FormRef mr;
    if (ok && parsed && marker && formRef(marker, &mr)) g_hub->record(SetMapMarker{mr, static_cast<uint8_t>(kMarkerVisible | (travel ? kMarkerTravel : 0))});
    return ok;
}

// EnableFastTravel 0|1: global fast-travel switch. Captured as SetFastTravelLock{scope = the cause quest/script form, on = (arg == 0)}; the
// applier keeps the set of locked scopes and re-enables travel only when the last scope unlocks. Outside any capture scope there is no cause
// to name, so the call is ambient and not captured.
Fn_Cmd o_EnableFastTravel;
bool __cdecl h_EnableFastTravel(CMD_ARGS) {
    if (blockedNow()) return true;
    int enable = 1;
    FormRef scope;
    bool parsed = capturing() && g_hub->currentSource(&scope) && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &enable);
    bool ok = CMD_FWD(o_EnableFastTravel);
    if (ok && parsed) g_hub->record(SetFastTravelLock{scope, enable == 0});
    return ok;
}

bool __cdecl h_SetTimeMult(CMD_ARGS) {
    if (blockedNow()) return true;
    float scale = 0;
    int opt = 0;
    bool parsed = capturing() && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &scale, &opt);
    bool ok = CMD_FWD(o_SetTimeMult);
    if (ok && parsed) g_hub->record(SetTimeScale{scale});
    return ok;
}

bool saveCmdHook(Fn_Cmd o, uint8_t kind, CMD_ARGS) {
    if (blockedNow()) return true;
    if (capturing()) g_hub->record(RequestSave{kind});
    return CMD_FWD(o);
}
bool __cdecl h_Autosave(CMD_ARGS) { return saveCmdHook(o_Autosave, kSaveAuto, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_SystemSave(CMD_ARGS) { return saveCmdHook(o_SystemSave, kSaveSystem, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_ForceSave(CMD_ARGS) { return saveCmdHook(o_ForceSave, kSaveForce, pi, sd, thisObj, contObj, script, ev, res, opc); }

// RemoveAllItems [container] [ownershipAdded] [noMessage] (G35). On a non-player reference it clears that inventory (RemoveAllItems effect, as before).
// Gear shuffles that involve a PLAYER are reported to the PlayerStashObserver (when the session registered one) as who, from/to which container and
// what moved, because no effect can say "this player's own gear goes into THEIR copy of the shared storage": the Fort confiscation
// (`player.RemoveAllItems NVCCPlayerStorageREF`), its return (`NVCCPlayerStorageREF.RemoveAllItems player`), the Dead Money entry strip, frisk lockers.
// A script acting for a guest never empties the host's own player or hands it the guest's items: the original is skipped and the event names the guest.
Inventory readInventory(void* ref);
PlayerStashObserver g_stashObs;
std::vector<std::pair<FormRef, int32_t>> stashItems(const Inventory& inv) {
    std::vector<std::pair<FormRef, int32_t>> out;
    for (const auto& kv : inv) {
        FormRef ir;
        if (kv.second > 0 && g_res && g_res->toRef(kv.first, &ir)) out.emplace_back(ir, kv.second);
    }
    return out;
}
bool __cdecl h_RemoveAllItems(CMD_ARGS) {
    if (blockedNow()) return true;
    if (g_stashObs && thisObj) {
        TESObjectREFR* dest = nullptr;
        int ownership = 0, noMsg = 0;
        const bool parsed = parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &dest, &ownership, &noMsg);
        const ActingContext::PlayerTarget src = g_acting.playerTargetOf(refIdOf(thisObj));
        const ActingContext::PlayerTarget dst = (parsed && dest) ? g_acting.playerTargetOf(refIdOf(dest)) : ActingContext::PlayerTarget{};
        if (parsed && (src.player || dst.player)) {
            PlayerStashEvent e;
            e.intoPlayer = !src.player;
            e.slot = src.player ? src.slot : dst.slot;
            e.retainOwnership = ownership != 0;
            const bool engineRuns = src.player ? src.onHost : dst.onHost;   // the host's own player is really involved
            FormRef cr;
            if (src.player) { if (dest) formRef(dest, &cr); }
            else formRef(thisObj, &cr);
            e.container = cr;
            if (engineRuns) e.items = stashItems(readInventory(thisObj));
            g_stashObs(e);
            return engineRuns ? CMD_FWD(o_RemoveAllItems) : true;
        }
    }
    FormRef rr;
    bool track = capturing() && thisObj && !isPlayer(thisObj) && formRef(thisObj, &rr);
    bool ok = CMD_FWD(o_RemoveAllItems);
    if (ok && track) g_hub->record(RemoveAllItems{rr});
    return ok;
}

// RemoveFromAllFactions (G12): the engine clears the runtime faction list of the actor; the effect list has no such effect, so the capture names
// every faction the actor was in just before (faction_removal.h) as SetFactionRank{actor, faction, -1}. Base factions: TESActorBase::baseData
// (xNVSE layout), runtime ones: ExtraFactionChanges (extra 0x5E). Raw reads under SEH, no C++ objects in the guarded function.
struct RawFaction {
    uint32_t id;
    int rank;
};
int rawFactionList(TESObjectREFR* ref, bool runtimeList, RawFaction* out, int cap) {
    int n = 0;
    __try {
        if (runtimeList) {
            auto* x = static_cast<ExtraFactionChanges*>(ref->extraDataList.GetByType(kExtraData_FactionChanges));
            if (x && x->data)
                for (auto it = x->data->Begin(); !it.End() && n < cap; ++it)
                    if (it.Get() && it.Get()->faction) { out[n].id = it.Get()->faction->refID; out[n].rank = static_cast<int8_t>(it.Get()->rank); ++n; }
        } else if (ref->baseForm && (ref->baseForm->typeID == 0x2A || ref->baseForm->typeID == 0x2B)) {   // NPC_ / CREA
            auto* ab = static_cast<TESActorBase*>(ref->baseForm);
            for (auto it = ab->baseData.factionList.Begin(); !it.End() && n < cap; ++it)
                if (it.Get() && it.Get()->faction) { out[n].id = it.Get()->faction->refID; out[n].rank = static_cast<int8_t>(it.Get()->rank); ++n; }
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return n;
    }
    return n;
}
Fn_Cmd o_RemoveFromAllFactions;
bool __cdecl h_RemoveFromAllFactions(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef ar;
    const bool track = capturing() && thisObj && formRef(thisObj, &ar);
    std::vector<uint32_t> lost;
    if (track) {
        RawFaction rb[96], rr[96];
        const int nb = rawFactionList(static_cast<TESObjectREFR*>(thisObj), false, rb, 96);
        const int nr = rawFactionList(static_cast<TESObjectREFR*>(thisObj), true, rr, 96);
        std::vector<FactionRankEntry> base, run;
        for (int i = 0; i < nb; ++i) base.push_back({rb[i].id, rb[i].rank});
        for (int i = 0; i < nr; ++i) run.push_back({rr[i].id, rr[i].rank});
        lost = factionsLeftBy(base, run);
    }
    const bool ok = CMD_FWD(o_RemoveFromAllFactions);
    if (ok && track)
        for (uint32_t id : lost) {
            FormRef fr;
            if (g_res->toRef(id, &fr)) g_hub->record(SetFactionRank{ar, fr, -1});
        }
    return ok;
}

// PlaceAtMe / PlaceLeveledActorAtMe: the new reference's FormID (0xFFxxxxxx) comes back in *result. SpawnRef carries it as-is
// (kCreatedRefPrefix == the engine's dynamic-ref prefix); the host session maps/allocates the stable id.
bool placeCmd(Fn_Cmd o, bool leveled, CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef br;
    TESForm* base = nullptr;
    int n = 1, dist = 0, dir = 0;
    bool parsed = capturing() && !leveled && parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &base, &n, &dist, &dir);
    if (capturing() && leveled) parsed = parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &base, &n, &dist);
    bool ok = CMD_FWD(o);
    if (ok && parsed && base && res && formRef(base, &br)) {
        uint32_t newId = *reinterpret_cast<uint32_t*>(res);
        TESObjectREFR* nr = newId ? static_cast<TESObjectREFR*>(g_res->lookup(newId)) : nullptr;
        if (nr) {
            SpawnRef sp;
            sp.id = newId;
            // PlaceLeveledActorAtMe: the LIST was passed but the engine rolled a real base - send the real one (a client placing the list would re-roll)
            FormRef realBase;
            sp.base = (nr->baseForm && formRef(nr->baseForm, &realBase)) ? realBase : br;
            formRef(nr->parentCell, &sp.cell);
            sp.x = nr->posX; sp.y = nr->posY; sp.z = nr->posZ;
            sp.leveled = leveled;
            g_hub->record(sp);
        }
    }
    return ok;
}
bool __cdecl h_PlaceAtMe(CMD_ARGS) { return placeCmd(o_PlaceAtMe, false, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_PlaceLeveled(CMD_ARGS) { return placeCmd(o_PlaceLeveled, true, pi, sd, thisObj, contObj, script, ev, res, opc); }

// Current inventory of a reference as the engine stores it: ExtraContainerChanges (extra 0x15) entry list, item refID -> countDelta.
Inventory readInventory(void* ref) {
    Inventory inv;
    if (!ref) return inv;
    auto* xc = static_cast<ExtraContainerChanges*>(static_cast<TESObjectREFR*>(ref)->extraDataList.GetByType(kExtraData_ContainerChanges));
    ExtraContainerChanges::EntryDataList* list = xc ? xc->GetEntryDataList() : nullptr;
    if (!list) return inv;
    for (auto it = list->Begin(); !it.End(); ++it)
        if (it.Get() && it.Get()->type) inv[it.Get()->type->refID] += it.Get()->countDelta;
    return inv;
}

// Record what really changed in `ref`'s inventory (used when the command's item form resolves to something else - leveled
// lists / form lists - or removes a variable set, e.g. RemoveAllTypedItems).
void recordInventoryDelta(void* ref, const Inventory& before, bool gained, bool lost) {
    Inventory after = readInventory(ref);
    bool player = isPlayer(ref);
    FormRef cr;
    if (!player && !formRef(ref, &cr)) return;
    auto emit = [&](uint32_t itemId, int32_t n, bool add) {
        FormRef ir;
        if (!g_res->toRef(itemId, &ir)) return;
        if (player) {
            if (add) g_hub->record(AddItem{ir, n, Recipient::dflt()});
            else g_hub->record(RemoveItem{ir, n, Recipient::dflt()});
        } else {
            if (add) g_hub->record(RefAddItem{cr, ir, n});
            else g_hub->record(RefRemoveItem{cr, ir, n});
        }
    };
    if (gained)
        for (auto& [id, n] : inventoryGained(before, after)) emit(id, n, true);
    if (lost)
        for (auto& [id, n] : inventoryLost(before, after)) emit(id, n, false);
}

bool itemCmd(Fn_Cmd o, bool add, CMD_ARGS) {
    if (blockedNow()) return true;
    if (!capturing()) return CMD_FWD(o);
    TESForm* item = nullptr;
    int count = 0;
    UInt32 off = *opc;  // ExtractArgs advances the offset: work on a copy so the real call sees the original
    // AddItem / RemoveItem have a third, optional "hide message" flag (AddItem <item> <n> 1): padded, it lands in scratch.
    bool parsed = extractPadded(extractArgs(), pi, sd, &off, thisObj, contObj, script, ev, {&item, &count});
    bool resolving = parsed && item && add && isResolvingItemType(item->typeID);
    Inventory before;
    if (resolving) before = readInventory(thisObj);
    // A guest's hand-in (dialogue result of a guest's pick): `player` is the host's player here, but the pack that pays is the guest's. The engine call is skipped; the
    // removal is recorded and the rules route it (the guest's machine takes its part, the session takes the host's own share).
    // A guest acts when its INFO result runs (GuestItemScope) and in any other script the host runs for it (the acting window: an activator, a terminal, a stage result).
    const bool guestActing = g_guestItems.active() || !g_acting.playerTargetOf(kPlayerRefID).onHost;
    const RemovePlan plan = add ? planPlayerAddItem(isPlayer(thisObj), guestActing, capturing(), parsed && item && count > 0, resolving)
                                : planPlayerRemoveItem(isPlayer(thisObj), guestActing, capturing(), parsed && item && count > 0);
    if (!plan.runEngine) {
        FormRef gr;
        if (formRef(item, &gr)) {
            if (add) g_hub->record(AddItem{gr, count, Recipient::dflt()});
            else g_hub->record(RemoveItem{gr, count, Recipient::dflt()});
            if (traceEnabled()) traceLine(std::string("capture: ") + (add ? "AddItem " : "RemoveItem ") + gr.plugin + ":" + std::to_string(gr.localId) + " x" + std::to_string(count) + " for a guest (slot " + std::to_string(int(g_guestItems.active() ? g_guestItems.slot() : g_acting.slot())) + "): the host's pack is not touched");
            return true;
        }
    }
    bool ok = CMD_FWD(o);
    if (ok && resolving && thisObj) {  // leveled list / form list: send what actually landed, never the list
        recordInventoryDelta(thisObj, before, true, false);
        return ok;
    }
    FormRef ir;
    if (ok && parsed && item && count > 0 && formRef(item, &ir)) {
        if (isPlayer(thisObj)) {
            if (traceEnabled()) traceLine(std::string("capture: ") + (add ? "AddItem " : "RemoveItem ") + ir.plugin + ":" + std::to_string(ir.localId) + " x" + std::to_string(count) + " on the local player, count now " + std::to_string(playerItemCountForTrace(refIdOf(item))));
            if (add) g_hub->record(AddItem{ir, count, Recipient::dflt()});
            else g_hub->record(RemoveItem{ir, count, Recipient::dflt()});
        } else {
            FormRef cr;
            if (thisObj && formRef(thisObj, &cr)) {
                if (add) g_hub->record(RefAddItem{cr, ir, count});
                else g_hub->record(RefRemoveItem{cr, ir, count});
            } else {
                ++g_unmapped.itemOnNonPlayer;
            }
        }
    }
    return ok;
}

// RemoveAllTypedItems [container] type: removes a variable set -> record the real removals.
// W3d item 4 (G35): `player.RemoveAllTypedItems container [ownership] [noMessage] type [holdout]` is the failure ending of Dead Money (weapons only, type 40) and the Fort's
// 131 gear moves. A script acting for a guest used to strip the HOST's player of that type; now it is the same stash event as RemoveAllItems with the type filter, and the
// original is skipped for a guest (the host's own player is unchanged).
bool __cdecl h_RemoveAllTyped(CMD_ARGS) {
    if (blockedNow()) return true;
    if (g_stashObs && thisObj) {
        TESObjectREFR* dest = nullptr;
        TESForm* holdout = nullptr;
        int ownership = 0, noMsg = 0, type = -1;
        const bool parsed = parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &dest, &ownership, &noMsg, &type, &holdout);
        const ActingContext::PlayerTarget src = g_acting.playerTargetOf(refIdOf(thisObj));
        const ActingContext::PlayerTarget dst = (parsed && dest) ? g_acting.playerTargetOf(refIdOf(dest)) : ActingContext::PlayerTarget{};
        const bool forGuest = src.player ? !src.onHost : dst.player && !dst.onHost;
        if (parsed && forGuest) {   // the host's own player keeps the old path below (the engine call and the recorded inventory change)
            PlayerStashEvent e;
            e.intoPlayer = !src.player;
            e.slot = src.player ? src.slot : dst.slot;
            e.retainOwnership = ownership != 0;
            e.noMessage = noMsg != 0;
            e.typeFilter = type;
            if (holdout) formRef(holdout, &e.holdout);
            FormRef cr;
            if (src.player) { if (dest) formRef(dest, &cr); }
            else formRef(thisObj, &cr);
            e.container = cr;
            g_stashObs(e);
            return true;   // the guest's own machine strips the guest; the host's player is not the guest's
        }
    }
    if (!capturing() || !thisObj) return CMD_FWD(o_RemoveAllTyped);
    Inventory before = readInventory(thisObj);
    bool ok = CMD_FWD(o_RemoveAllTyped);
    if (ok) recordInventoryDelta(thisObj, before, false, true);
    return ok;
}
bool __cdecl h_AddItem(CMD_ARGS) { return itemCmd(o_AddItem, true, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_RemoveItem(CMD_ARGS) { return itemCmd(o_RemoveItem, false, pi, sd, thisObj, contObj, script, ev, res, opc); }

constexpr uint32_t kPseudoAV_XP = fnvcoop::kAvExperience;  // XP is not a real actor value (L4 contract: 0x10000)
bool __cdecl h_RewardXP(CMD_ARGS) {
    if (blockedNow()) return true;
    int amount = 0;
    bool parsed = false;
    if (capturing()) {
        UInt32 off = *opc;
        parsed = extractPadded(extractArgs(), pi, sd, &off, thisObj, contObj, script, ev, {&amount});
    }
    if (parsed && amount && isPlayer(thisObj) && !planGuestReward(guestWindowNow(), capturing(), true).runEngine) {   // a guest's XP: recorded only, the session gives the host its share
        g_hub->record(ModActorValue{kPseudoAV_XP, amount, Recipient::dflt()});
        if (traceEnabled()) traceLine("capture: RewardXP " + std::to_string(amount) + " for a guest: the host's XP is not touched");
        return true;
    }
    bool ok = CMD_FWD(o_RewardXP);
    if (ok && parsed && amount) g_hub->record(ModActorValue{kPseudoAV_XP, amount, Recipient::dflt()});
    return ok;
}

// `player.KillActor` / a guest's ghost: ONE player dies, not the party (G42). The kill of a player is captured as KillActor{<that player's own ref>} (the player ref
// for the host's own player, the ref the session registered with setPlayerRefProvider for a guest's slot); a PerPlayerState rule of the quest (collar, cloud, gas trap)
// rewrites it into a Health drain addressed to that slot, and a quest without the rule keeps the old shared kill. The engine kill runs only on a machine whose own
// player it concerns: a kill meant for a guest is skipped on the host (it would hit the host's player instead). A PlayerKillObserver, when registered, is told as well.
// With neither a registered player ref nor an observer the old path runs (KillActor{operand} recorded, engine kill run).
PlayerKillObserver g_playerKillObs;
PlayerRefOf g_playerRefOf;
bool __cdecl h_Kill(CMD_ARGS) {
    if (blockedNow()) return true;
    if (thisObj) {
        const ActingContext::PlayerTarget pt = g_acting.playerTargetOf(refIdOf(thisObj));
        if (pt.player) {
            const PlayerKillPlan plan = planPlayerKill(pt, static_cast<bool>(g_playerKillObs), g_playerRefOf, capturing());
            if (!plan.legacy) {
                if (plan.report) g_playerKillObs(pt.slot, pt.onHost);
                const bool ok = plan.runEngine ? CMD_FWD(o_Kill) : true;
                if (ok && plan.record) g_hub->record(KillActor{plan.ref});
                return ok;
            }
        }
    }
    FormRef ar;
    bool track = capturing() && formRef(thisObj, &ar);
    bool ok = CMD_FWD(o_Kill);
    if (ok && track) g_hub->record(KillActor{ar});
    return ok;
}
bool __cdecl h_Resurrect(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef ar;
    bool track = capturing() && formRef(thisObj, &ar);
    bool ok = CMD_FWD(o_Resurrect);
    if (ok && track) g_hub->record(ResurrectActor{ar});
    return ok;
}
// Enable / Disable are captured at COMMAND level. Live evidence (L5 proof_enable, L7 sweep): Cmd_Enable on an initially-disabled
// ref changes the form flags (0xC09 -> 0x40B) without ever reaching a state-changing SetDisabled (0x5650D0 only follows the
// ExtraFlags bit, 0x56C780 touches a different bit), so no single primitive sees every enable. The script said "enable X": record that.
bool enableDisableCmd(Fn_Cmd o, bool enable, CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef rr;
    bool track = capturing() && !isPlayer(thisObj) && formRef(thisObj, &rr);
    bool ok = CMD_FWD(o);
    if (ok && track) {
        if (enable) g_hub->record(EnableRef{rr});
        else g_hub->record(DisableRef{rr});
    }
    return ok;
}
bool __cdecl h_EnableCmd(CMD_ARGS) { return enableDisableCmd(o_EnableCmd, true, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_DisableCmd(CMD_ARGS) { return enableDisableCmd(o_DisableCmd, false, pi, sd, thisObj, contObj, script, ev, res, opc); }

bool moveCmd(Fn_Cmd o, CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef rr;
    bool track = capturing() && formRef(thisObj, &rr);
    // MoveToMarker / MoveToFade params: marker ref, [x] [y] [z] offsets (floats). Capture the MARKER + offsets so the replica
    // can issue the very same command; the final position is read back only when no marker was given.
    TESObjectREFR* marker = nullptr;
    float ox = 0, oy = 0, oz = 0;
    bool parsed = false;
    if (track) {
        UInt32 off = *opc;
        parsed = extractPadded(extractArgs(), pi, sd, &off, thisObj, contObj, script, ev, {&marker, &ox, &oy, &oz});
    }
    bool ok = CMD_FWD(o);
    if (ok && track) {
        FormRef mr;
        if (parsed && marker && formRef(marker, &mr)) {
            g_hub->record(MoveRef{rr, mr, ox, oy, oz});  // cell == marker ref, x/y/z == offsets from the marker
        } else {
            TESObjectREFR* r = static_cast<TESObjectREFR*>(thisObj);
            FormRef cell;
            formRef(r->parentCell, &cell);
            g_hub->record(MoveRef{rr, cell, r->posX, r->posY, r->posZ});
        }
    }
    return ok;
}
bool __cdecl h_MoveTo(CMD_ARGS) { return moveCmd(o_MoveTo, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_MoveToFade(CMD_ARGS) { return moveCmd(o_MoveToFade, pi, sd, thisObj, contObj, script, ev, res, opc); }

// SetPos axis value: same-cell absolute move; the final position + parent cell are captured after the call.
bool __cdecl h_SetPos(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef rr;
    bool track = capturing() && formRef(thisObj, &rr);
    bool ok = CMD_FWD(o_SetPos);
    if (ok && track) {
        TESObjectREFR* r = static_cast<TESObjectREFR*>(thisObj);
        FormRef cell;
        formRef(r->parentCell, &cell);
        g_hub->record(MoveRef{rr, cell, r->posX, r->posY, r->posZ});
    }
    return ok;
}

// SetAngle axis degrees: the final rotation of the commanded axis (engine radians -> degrees) is captured after the call, so engine
// normalisation is respected; when the Axis argument cannot be parsed all three axes are captured (the shadow dedupes unchanged ones).
bool __cdecl h_SetAngle(CMD_ARGS) {
    if (blockedNow()) return true;
    FormRef rr;
    bool track = capturing() && formRef(thisObj, &rr);
    UInt32 off = *opc;
    UInt32 axis = 0;
    float val = 0;
    bool parsed = track && extractPadded(extractArgs(), pi, sd, &off, thisObj, contObj, script, ev, {&axis, &val});
    // The pre-image (angle before the command) lets the shadow restore an axis after a host load; see RefAngle in effect.h.
    constexpr float kRadToDeg = 57.29577951f;
    float pre[3] = {0, 0, 0};
    if (track) {
        TESObjectREFR* r0 = static_cast<TESObjectREFR*>(thisObj);
        pre[0] = r0->rotX * kRadToDeg; pre[1] = r0->rotY * kRadToDeg; pre[2] = r0->rotZ * kRadToDeg;
    }
    bool ok = CMD_FWD(o_SetAngle);
    if (ok && track) {
        TESObjectREFR* r = static_cast<TESObjectREFR*>(thisObj);
        const float rot[3] = {r->rotX, r->rotY, r->rotZ};
        const int a = parsed ? axisIndexOf(axis & 0xFF) : -1;
        for (int i = 0; i < 3; ++i)
            if (a < 0 || i == a) g_hub->record(RefAngle{rr, static_cast<uint8_t>(i), rot[i] * kRadToDeg, true, pre[i]});
    }
    return ok;
}

// ActorValue commands (Set/Force/Mod/Damage): record the NET change on the player as an integer delta.
NonPlayerAvObserver g_npAvObs;
bool avCmd(Fn_Cmd o, int kind /*0 set 1 mod 2 force 3 damage*/, CMD_ARGS) {
    if (blockedNow()) return true;
    if (!capturing() || !isPlayer(thisObj)) {
        if (capturing() && thisObj) {
            ++g_unmapped.avOnNonPlayer;
            FormRef ar;
            if (g_npAvObs && formRef(thisObj, &ar)) {  // L12: non-player actor values are turned into effects by the world-state lane
                UInt32 off2 = *opc;
                UInt32 av2 = 0xFFFFFFFF;
                float val2 = 0;
                bool parsed2 = extractPadded(extractArgs(), pi, sd, &off2, thisObj, contObj, script, ev, {&av2, &val2});
                float before2 = parsed2 ? getAV(thisObj, av2) : 0;
                bool ok2 = CMD_FWD(o);
                if (ok2 && parsed2) g_npAvObs(ar, av2, static_cast<uint8_t>(kind), before2, getAV(thisObj, av2));
                return ok2;
            }
        }
        return CMD_FWD(o);
    }
    UInt32 off = *opc;
    UInt32 av = 0xFFFFFFFF;
    float val = 0;
    bool parsed = extractPadded(extractArgs(), pi, sd, &off, thisObj, contObj, script, ev, {&av, &val});
    float before = parsed ? getAV(thisObj, av) : 0;
    bool ok = CMD_FWD(o);
    if (ok && parsed) {
        int delta = static_cast<int>(std::lround(getAV(thisObj, av) - before));
        if (delta) {
            if (av == kKarmaAV) {
                g_hub->record(ModKarma{delta, Recipient::dflt()});
                if (!planGuestReward(guestWindowNow(), true, true).runEngine) {   // a guest's karma through ModActorValue: the engine moved the host's AV, put it back (same virtual as the applier's)
                    void** vt = *reinterpret_cast<void***>(thisObj);
                    reinterpret_cast<void(__thiscall*)(void*, uint32_t, float, void*)>(vt[0x3A8 / 4])(thisObj, av, -static_cast<float>(delta), nullptr);
                }
            } else {
                g_hub->record(ModActorValue{av, delta, Recipient::dflt()});
            }
        }
    }
    return ok;
}
bool __cdecl h_ModAV(CMD_ARGS) { return avCmd(o_ModAV, 1, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_SetAV(CMD_ARGS) { return avCmd(o_SetAV, 0, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_ForceAV(CMD_ARGS) { return avCmd(o_ForceAV, 2, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_DamageAV(CMD_ARGS) { return avCmd(o_DamageAV, 3, pi, sd, thisObj, contObj, script, ev, res, opc); }

// ------------------------------------------------------------------ party-aware condition evaluators
// Evaluator = bool(thisObj, param1, param2, double* result), cdecl. When the local player is an operand of a party-aware function and the
// session registered a party, the condition is evaluated once per member with the player operand swapped for the member's ghost, and the
// reducer (RulesEngine::reduceCondition) picks the answer. Quest for the reducer: ConditionQuestScope, else the open capture scope's source.
using CondFn = bool(__cdecl*)(void*, void*, void*, double*);
PartyProvider g_partyProvider;
ConditionReducer g_condReducer;
CondStats g_condStats;
bool g_inPartyEval = false;
struct CondScope {
    FormRef quest;
    fnvcoop::CondPhase phase;
};
std::vector<CondScope> g_condQuests;

// ------------------------------------------------------------------ W3d item 2: the quest of an INFO's conditions
// TESTopicInfo::CheckConditions (0x61E600, thiscall(info, 4 stack args), ret 0x10) is the one function that evaluates an INFO's condition list (called while the dialogue menu is
// built: GetTopicInfo 0x61A7D0 and 0x61AF30). The evaluators it reaches reduce under the quest that scope names.
InfoQuestScopes g_infoScopes;
uint64_t g_infoScopesOpened = 0;
using Fn_InfoConds = bool(__thiscall*)(void*, void*, void*, void*, void*);
Fn_InfoConds o_InfoConds;
bool __fastcall h_InfoConditions(void* info, void*, void* a1, void* a2, void* a3, void* a4) {
    FormRef q;
    if (g_condReducer && g_infoScopes.active() && g_infoScopes.questOf(refIdOf(info), &q)) {
        ConditionQuestScope scope(q);
        ++g_infoScopesOpened;
        return o_InfoConds(info, a1, a2, a3, a4);
    }
    return o_InfoConds(info, a1, a2, a3, a4);
}

// ------------------------------------------------------------------ player facts (P3 R1)
const std::vector<PlayerFact>* g_facts = nullptr;
// The initiator's reported value for what the script / condition reads on the player. `form` is the item / perk form, `avCode` the actor value code.
bool factAnswer(uint8_t kind, void* form, uint32_t avCode, double* out) {
    if (!g_facts || !kind) return false;
    FormRef w;
    if (kind == kFactActorValue) w = FormRef(std::string(), avCode);
    else if (!formRef(form, &w)) return false;
    int32_t v = 0;
    if (!lookupFact(*g_facts, kind, w, &v)) return false;
    *out = v;
    return true;
}
Fn_Cmd o_GetItemCountCmd, o_GetAVCmd, o_HasPerkCmd, o_GetEquippedCmd;
bool factCmd(uint8_t kind, Fn_Cmd o, CMD_ARGS) {
    if (g_facts && res && isPlayer(thisObj)) {
        TESForm* f = nullptr;
        UInt32 av = 0;
        int alt = 0;
        const bool parsed = kind == kFactActorValue ? parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &av)
                            : kind == kFactHasPerk  ? parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &f, &alt)
                                                    : parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &f);
        double v = 0;
        if (parsed && factAnswer(kind, f, av, &v)) { *res = v; return true; }
    }
    return CMD_FWD(o);
}
bool __cdecl h_GetItemCountCmd(CMD_ARGS) { return factCmd(kFactItemCount, o_GetItemCountCmd, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_GetAVCmd(CMD_ARGS) { return factCmd(kFactActorValue, o_GetAVCmd, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_HasPerkCmd(CMD_ARGS) { return factCmd(kFactHasPerk, o_HasPerkCmd, pi, sd, thisObj, contObj, script, ev, res, opc); }
bool __cdecl h_GetEquippedCmd(CMD_ARGS) { return factCmd(kFactEquipped, o_GetEquippedCmd, pi, sd, thisObj, contObj, script, ev, res, opc); }

StateProvider g_stateProvider;

// One party-aware answer: the members' values under the function's MemberView (party_cond.cpp evalMember), reduced by the rules engine.
// `ops.evalLocal` etc. close over the original call. False = fewer than two members could be evaluated (the caller keeps the plain result).
bool reduceOverParty(CondFunc f, const std::vector<PartyMember>& members, const StateQuery& base, const CondOps& ops, double* reduced) {
    return evaluateOverParty(
        f, members, [&](const PartyMember& m, double* v) { return evalMember(f, m, base, ops, v); },
        [&](const std::string& fn, const std::vector<CondValue>& vals, uint8_t init) {
            FormRef q;
            fnvcoop::CondPhase phase = fnvcoop::CondPhase::Entry;
            if (!g_condQuests.empty()) { q = g_condQuests.back().quest; phase = g_condQuests.back().phase; }
            else if (g_hub) g_hub->currentSource(&q);
            return g_condReducer ? g_condReducer(q, fn, vals, init, phase) : vals.front().value;
        },
        actingSlot(), reduced);
}
// runtime id of a form operand; guarded because a few condition parameters are plain integers, not forms
uint32_t operandFormId(void* p) {
    if (reinterpret_cast<uintptr_t>(p) < 0x10000) return 0;
    __try {
        return static_cast<TESForm*>(p)->refID;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

bool condEval(CondFunc f, CondFn orig, void* t, void* p1, void* p2, double* res) {
    ++g_condStats.evals;
    if (g_facts && t && refIdOf(t) == kPlayerRefID) {   // a guest's dialogue result script reads the initiator's facts, not the host's own player
        double fv = 0;
        if (factAnswer(factKindOf(f), p1, static_cast<uint32_t>(reinterpret_cast<uintptr_t>(p1)), &fv)) { *res = fv; return true; }
    }
    const bool always = planSwap(f, 0, 0).always;   // the evaluator ignores / does not carry the player operand (GetHasNote, IsPlayerInRegion, GetPlayerTeammate)
    if (!g_partyProvider || g_inPartyEval || (!always && !t && !p1)) return orig(t, p1, p2, res);
    SwapPlan plan = planSwap(f, t ? refIdOf(t) : 0, p1 && param1IsReference(f) ? refIdOf(p1) : 0);
    if (!plan.any()) return orig(t, p1, p2, res);
    std::vector<PartyMember> members = g_partyProvider();
    if (members.size() < 2) return orig(t, p1, p2, res);
    ++g_condStats.partyEvals;
    g_inPartyEval = true;
    bool lastOk = true;
    double reduced = 0;
    StateQuery base;
    base.thisRefID = t ? refIdOf(t) : kHostPlayerRef;
    base.formID = memberView(f) == MemberView::Ghost ? 0 : operandFormId(p1);
    base.arg = static_cast<int32_t>(reinterpret_cast<intptr_t>(p2));
    if (reinterpret_cast<uintptr_t>(p1) < 0x10000) base.arg1 = static_cast<int32_t>(reinterpret_cast<intptr_t>(p1));   // GetIsSex / IsPlayerActionActive: a plain integer, not a form
    CondOps ops;
    ops.evalLocal = [&](double* v) {
        double out = 0;
        lastOk = orig(t, p1, p2, &out);
        *v = out;
        return true;
    };
    ops.evalGhost = [&](uint32_t ghost, double* v) {
        void* who = g_res->lookup(ghost);
        if (!who) return false;  // ghost not loaded
        double out = 0;
        lastOk = orig(plan.thisObj ? who : t, plan.param1 ? who : p1, p2, &out);
        *v = out;
        return true;
    };
    ops.state = g_stateProvider;
    bool done = reduceOverParty(f, members, base, ops, &reduced);
    g_inPartyEval = false;
    if (!done) return orig(t, p1, p2, res);
    *res = reduced;
    ++g_condStats.reduced;
    return lastOk;
}
#define COND_HOOK(NAME) \
    CondFn o_Cond_##NAME; \
    bool __cdecl h_Cond_##NAME(void* t, void* p1, void* p2, double* r) { return condEval(CondFunc::NAME, o_Cond_##NAME, t, p1, p2, r); }
COND_HOOK(GetDistance)
COND_HOOK(GetInCell)
COND_HOOK(GetInSameCell)
COND_HOOK(IsInCombat)
COND_HOOK(GetDetected)
COND_HOOK(GetHealthPercentage)
COND_HOOK(GetItemCount)
COND_HOOK(GetActorValue)
COND_HOOK(GetInWorldspace)
COND_HOOK(IsKiller)
COND_HOOK(GetEquipped)
COND_HOOK(GetPlayerTeammate)
COND_HOOK(GetHasNote)
COND_HOOK(HasPerk)
COND_HOOK(IsSneaking)
COND_HOOK(IsInInterior)
// Wave 1b. GetLevel's evaluator is also what the `Player.GetLevel` command calls (Cmd_GetLevel 0x5C84D0 = `call 0x5A00C0`), so this one detour covers the script
// command, the condition and the XP-tier scripts alike.
COND_HOOK(GetLevel)
COND_HOOK(GetIsSex)
COND_HOOK(GetPCIsSex)
COND_HOOK(GetSitting)
COND_HOOK(GetShouldAttack)
COND_HOOK(IsPlayerActionActive)

// IsPlayerInRegion has no condition evaluator (its eval column is 0), only the script command handler. The engine reads the global player's
// region list (player+0x764), which no ghost has, so every other member's answer comes from the StateProvider (formID = the region).
Fn_Cmd o_IsPlayerInRegion;
bool __cdecl h_IsPlayerInRegion(CMD_ARGS) {
    if (!g_partyProvider || g_inPartyEval || !res) return CMD_FWD(o_IsPlayerInRegion);
    std::vector<PartyMember> members = g_partyProvider();
    if (members.size() < 2) return CMD_FWD(o_IsPlayerInRegion);
    TESForm* region = nullptr;
    const bool parsed = parseArgs(pi, sd, thisObj, contObj, script, ev, res, opc, &region);
    const bool ok = CMD_FWD(o_IsPlayerInRegion);
    if (!ok || !parsed) return ok;
    ++g_condStats.evals;
    ++g_condStats.partyEvals;
    const double local = *res;
    StateQuery base;
    base.thisRefID = kHostPlayerRef;
    base.formID = region ? region->refID : 0;
    CondOps ops;
    ops.evalLocal = [&](double* v) { *v = local; return true; };
    ops.state = g_stateProvider;
    double reduced = 0;
    g_inPartyEval = true;
    const bool done = reduceOverParty(CondFunc::IsPlayerInRegion, members, base, ops, &reduced);
    g_inPartyEval = false;
    if (done) { *res = reduced; ++g_condStats.reduced; }
    return ok;
}

// ------------------------------------------------------------------ install table
struct Entry {
    const HookSpec* spec;
    void* detour;
    void* origOut;  // &o_xxx
    bool hostOnly;  // false = needed on clients too
    bool guestOnly = false;  // installed on Role::Guest only
    bool chainForeign = false;  // hook_patch rule 4: install on top of another patcher's entry jump (xNVSE's EventManager hook)
};
#define HK(spec, det, orig, host) {&addr::spec, reinterpret_cast<void*>(&det), reinterpret_cast<void*>(&orig), host}
const Entry kEntries[] = {
    HK(ScriptExecute, h_ScriptExec, o_ScriptExec, false),
    HK(SetStage, h_SetStage, o_SetStage, true),
    HK(TopicInfoRunResult, h_TopicRun, o_TopicRun, true),
    HK(InfoConditions, h_InfoConditions, o_InfoConds, true),
    HK(ObjectiveSetState, h_ObjState, o_ObjState, true),
    HK(QuestSetRunning, h_SetRunning, o_SetRunning, true),
    HK(QuestSetCompleted, h_SetCompleted, o_SetCompleted, true),
    HK(RepAdd, h_RepAdd, o_RepAdd, true),
    HK(RepRemove, h_RepRemove, o_RepRemove, true),
    HK(RepSet, h_RepSet, o_RepSet, true),
    HK(RepAddExact, h_RepAddExact, o_RepAddExact, true),
    HK(PlayerRewardKarma, h_Karma, o_Karma, true),
    HK(RefSetDisabled, h_SetDisabled, o_SetDisabled, true),
    HK(Cmd_Enable, h_EnableCmd, o_EnableCmd, true),
    HK(Cmd_Disable, h_DisableCmd, o_DisableCmd, true),
    HK(Cmd_AddItem, h_AddItem, o_AddItem, true),
    HK(Cmd_RemoveItem, h_RemoveItem, o_RemoveItem, true),
    HK(Cmd_RewardXP, h_RewardXP, o_RewardXP, true),
    HK(Cmd_KillActor, h_Kill, o_Kill, true),
    HK(Cmd_ResurrectActor, h_Resurrect, o_Resurrect, true),
    HK(Cmd_MoveToMarker, h_MoveTo, o_MoveTo, true),
    HK(Cmd_MoveToMarkerWithFade, h_MoveToFade, o_MoveToFade, true),
    HK(Cmd_SetPos, h_SetPos, o_SetPos, true),
    HK(Cmd_SetAngle, h_SetAngle, o_SetAngle, true),
    HK(Cmd_ModActorValue, h_ModAV, o_ModAV, true),
    HK(Cmd_SetActorValue, h_SetAV, o_SetAV, true),
    HK(Cmd_ForceActorValue, h_ForceAV, o_ForceAV, true),
    HK(Cmd_DamageActorValue, h_DamageAV, o_DamageAV, true),
    HK(QuestSetFailed, h_SetFailed, o_SetFailed, true),
    HK(ActorSetTeammate, h_Teammate, o_Teammate, true),
    // xNVSE's EventManager writes `jmp MarkEventHook` here the first time anything registers a game-event handler (our own OnDrop
    // handlers do), and its stub re-runs the prologue and jumps to 0x5AC756: it never calls a previous hook. hooks_module primes that
    // one-shot install before Init, so this hook chains on top of xNVSE's jump instead of being silently overwritten by it.
    {&addr::MarkScriptEvent, reinterpret_cast<void*>(&h_MarkEvent), reinterpret_cast<void*>(&o_MarkEvent), true, false, true},
    HK(PlayerSetControlsDisabled, h_Controls, o_Controls, true),
    HK(QuestCompleteAllObjectives, h_CompleteAll, o_CompleteAll, true),
    {&addr::RefActivate, reinterpret_cast<void*>(&h_Activate), reinterpret_cast<void*>(&o_Activate), true, true},
    HK(Cmd_SetFactionRank, h_SetFactionRank, o_SetFactionRank, true),
    HK(Cmd_ModFactionRank, h_ModFactionRank, o_ModFactionRank, true),
    HK(Cmd_AddToFaction, h_AddToFaction, o_AddToFaction, true),
    HK(Cmd_RemoveFromFaction, h_RemoveFromFaction, o_RemoveFromFaction, true),
    HK(Cmd_AddPerk, h_AddPerk, o_AddPerk, true),
    HK(Cmd_RemovePerk, h_RemovePerk, o_RemovePerk, true),
    HK(Cmd_AddScriptPackage, h_AddScriptPackage, o_AddScriptPackage, true),
    HK(Cmd_RemoveScriptPackage, h_RemoveScriptPackage, o_RemoveScriptPackage, true),
    HK(Cmd_StartCombat, h_StartCombat, o_StartCombat, true),
    HK(Cmd_StopCombat, h_StopCombat, o_StopCombat, true),
    HK(Cmd_Lock, h_Lock, o_Lock, true),
    HK(Cmd_UnLock, h_UnLock, o_UnLock, true),
    HK(Cmd_Activate, h_ActivateCmd, o_ActivateCmd, true),
    HK(Cmd_SetOpenState, h_SetOpenState, o_SetOpenState, true),
    HK(Cmd_SetDestroyed, h_SetDestroyed, o_SetDestroyed, true),
    HK(Cmd_AddNote, h_AddNote, o_AddNote, true),
    HK(Cmd_ShowMap, h_ShowMap, o_ShowMap, true),
    HK(Cmd_EnableFastTravel, h_EnableFastTravel, o_EnableFastTravel, true),
    HK(Cond_GetDistance, h_Cond_GetDistance, o_Cond_GetDistance, true),
    HK(Cond_GetInCell, h_Cond_GetInCell, o_Cond_GetInCell, true),
    HK(Cond_GetInSameCell, h_Cond_GetInSameCell, o_Cond_GetInSameCell, true),
    HK(Cond_IsInCombat, h_Cond_IsInCombat, o_Cond_IsInCombat, true),
    HK(Cond_GetDetected, h_Cond_GetDetected, o_Cond_GetDetected, true),
    HK(Cond_GetHealthPercentage, h_Cond_GetHealthPercentage, o_Cond_GetHealthPercentage, true),
    HK(Cond_GetItemCount, h_Cond_GetItemCount, o_Cond_GetItemCount, true),
    HK(Cond_GetActorValue, h_Cond_GetActorValue, o_Cond_GetActorValue, true),
    HK(Cond_GetInWorldspace, h_Cond_GetInWorldspace, o_Cond_GetInWorldspace, true),
    HK(Cond_IsKiller, h_Cond_IsKiller, o_Cond_IsKiller, true),
    HK(Cond_GetEquipped, h_Cond_GetEquipped, o_Cond_GetEquipped, true),
    HK(Cond_GetPlayerTeammate, h_Cond_GetPlayerTeammate, o_Cond_GetPlayerTeammate, true),
    HK(Cond_GetHasNote, h_Cond_GetHasNote, o_Cond_GetHasNote, true),
    HK(Cond_HasPerk, h_Cond_HasPerk, o_Cond_HasPerk, true),
    HK(Cond_IsSneaking, h_Cond_IsSneaking, o_Cond_IsSneaking, true),
    HK(Cond_IsInInterior, h_Cond_IsInInterior, o_Cond_IsInInterior, true),
    HK(Cond_GetLevel, h_Cond_GetLevel, o_Cond_GetLevel, true),
    HK(Cond_GetIsSex, h_Cond_GetIsSex, o_Cond_GetIsSex, true),
    HK(Cond_GetPCIsSex, h_Cond_GetPCIsSex, o_Cond_GetPCIsSex, true),
    HK(Cond_GetSitting, h_Cond_GetSitting, o_Cond_GetSitting, true),
    HK(Cond_GetShouldAttack, h_Cond_GetShouldAttack, o_Cond_GetShouldAttack, true),
    HK(Cond_IsPlayerActionActive, h_Cond_IsPlayerActionActive, o_Cond_IsPlayerActionActive, true),
    HK(Cmd_IsPlayerInRegion, h_IsPlayerInRegion, o_IsPlayerInRegion, true),
    HK(PlayerAddNote, h_PlayerAddNote, o_PlayerAddNote, true),
    HK(Cmd_GetItemCount, h_GetItemCountCmd, o_GetItemCountCmd, true),
    HK(Cmd_GetActorValue, h_GetAVCmd, o_GetAVCmd, true),
    HK(Cmd_HasPerk, h_HasPerkCmd, o_HasPerkCmd, true),
    HK(Cmd_GetEquipped, h_GetEquippedCmd, o_GetEquippedCmd, true),
    HK(Cmd_RemoveNote, h_RemoveNote, o_RemoveNote, true),
    HK(Cmd_AddTopic, h_AddTopic, o_AddTopic, true),
    HK(Cmd_SetWeather, h_SetWeather, o_SetWeather, true),
    HK(Cmd_ForceWeather, h_ForceWeather, o_ForceWeather, true),
    HK(Cmd_SetGlobalTimeMultiplier, h_SetTimeMult, o_SetTimeMult, true),
    HK(Cmd_Autosave, h_Autosave, o_Autosave, true),
    HK(Cmd_SystemSave, h_SystemSave, o_SystemSave, true),
    HK(Cmd_ForceSave, h_ForceSave, o_ForceSave, true),
    HK(Cmd_RemoveAllItems, h_RemoveAllItems, o_RemoveAllItems, true),
    HK(Cmd_RemoveFromAllFactions, h_RemoveFromAllFactions, o_RemoveFromAllFactions, true),
    HK(Cmd_RemoveAllTypedItems, h_RemoveAllTyped, o_RemoveAllTyped, true),
    HK(Cmd_PlaceAtMe, h_PlaceAtMe, o_PlaceAtMe, true),
    HK(Cmd_PlaceLeveledActorAtMe, h_PlaceLeveled, o_PlaceLeveled, true),
};

// ------------------------------------------------------------------ calling-convention pins
// Every detour's stack-argument size must equal the callee-popped bytes (`ret N`) found in the engine function
// (HookSpec::retBytes, from the disassembly). A thiscall detour is written as __fastcall(ecx, edx-dummy, args...): its
// args after the first two must add up to retBytes. A cdecl command handler must have retBytes == 0. A mismatch is a
// compile error, i.e. the Console_Print crash class (cdecl vs thiscall mix-up) cannot be reintroduced silently.
template <class T> struct FastArgBytes;
template <class R, class A, class B, class... Rest>
struct FastArgBytes<R(__fastcall*)(A, B, Rest...)> {
    static constexpr int value = (0 + ... + static_cast<int>((sizeof(Rest) + 3) & ~size_t(3)));
};
#define PIN_F(spec, det) static_assert(FastArgBytes<decltype(&det)>::value == addr::spec.retBytes, "stack args of " #det " != ret N of " #spec)
#define PIN_C(spec) static_assert(addr::spec.retBytes == 0, #spec " is not a plain `ret` (cdecl) function")
PIN_F(ScriptExecute, h_ScriptExec);
PIN_F(SetStage, h_SetStage);
PIN_F(TopicInfoRunResult, h_TopicRun);
PIN_F(InfoConditions, h_InfoConditions);
PIN_F(ObjectiveSetState, h_ObjState);
PIN_F(QuestSetRunning, h_SetRunning);
PIN_F(QuestSetCompleted, h_SetCompleted);
PIN_F(QuestSetFailed, h_SetFailed);
PIN_F(ActorSetTeammate, h_Teammate);
PIN_F(PlayerSetControlsDisabled, h_Controls);
PIN_F(QuestCompleteAllObjectives, h_CompleteAll);
PIN_F(RepAdd, h_RepAdd);
PIN_F(RepRemove, h_RepRemove);
PIN_F(RepSet, h_RepSet);
PIN_F(RepAddExact, h_RepAddExact);
PIN_F(PlayerRewardKarma, h_Karma);
PIN_F(RefSetDisabled, h_SetDisabled);
PIN_C(Cmd_Enable); PIN_C(Cmd_Disable);
PIN_F(RefActivate, h_Activate);
PIN_C(Cmd_AddItem); PIN_C(Cmd_RemoveItem); PIN_C(Cmd_RewardXP); PIN_C(Cmd_KillActor); PIN_C(Cmd_ResurrectActor);
PIN_C(Cmd_MoveToMarker); PIN_C(Cmd_ModActorValue); PIN_C(Cmd_SetActorValue); PIN_C(Cmd_ForceActorValue);
PIN_C(Cmd_DamageActorValue); PIN_C(Cmd_SetFactionRank); PIN_C(Cmd_ModFactionRank); PIN_C(Cmd_AddToFaction);
PIN_C(Cmd_RemoveFromFaction); PIN_C(Cmd_AddPerk); PIN_C(Cmd_RemovePerk); PIN_C(Cmd_AddScriptPackage);
PIN_C(Cmd_RemoveScriptPackage); PIN_C(Cmd_StartCombat); PIN_C(Cmd_StopCombat); PIN_C(Cmd_Lock); PIN_C(Cmd_UnLock);
PIN_C(MarkScriptEvent); PIN_C(Cmd_Activate); PIN_C(Cmd_SetOpenState); PIN_C(Cmd_SetDestroyed); PIN_C(Cmd_AddNote); PIN_C(Cmd_ShowMap); PIN_C(Cond_GetDistance); PIN_C(Cond_GetInCell); PIN_C(Cond_GetInSameCell); PIN_C(Cond_IsInCombat); PIN_C(Cond_GetDetected); PIN_C(Cond_GetHealthPercentage); PIN_C(Cond_GetItemCount); PIN_C(Cond_GetActorValue); PIN_C(Cmd_EnableFastTravel); PIN_C(Cmd_RemoveNote);
PIN_C(Cond_GetInWorldspace); PIN_C(Cond_IsKiller); PIN_C(Cond_GetEquipped); PIN_C(Cond_GetPlayerTeammate); PIN_C(Cond_GetHasNote); PIN_C(Cond_HasPerk); PIN_C(Cond_IsSneaking); PIN_C(Cond_IsInInterior); PIN_C(Cond_GetLevel); PIN_C(Cond_GetIsSex); PIN_C(Cond_GetPCIsSex); PIN_C(Cond_GetSitting); PIN_C(Cond_GetShouldAttack); PIN_C(Cond_IsPlayerActionActive); PIN_C(Cmd_IsPlayerInRegion); PIN_C(Cmd_RemoveFromAllFactions); PIN_C(Cmd_GetItemCount); PIN_C(Cmd_GetActorValue); PIN_C(Cmd_HasPerk); PIN_C(Cmd_GetEquipped); PIN_F(PlayerAddNote, h_PlayerAddNote);
PIN_C(Cmd_AddTopic); PIN_C(Cmd_SetWeather); PIN_C(Cmd_ForceWeather); PIN_C(Cmd_SetGlobalTimeMultiplier); PIN_C(Cmd_Autosave);
PIN_C(Cmd_SystemSave); PIN_C(Cmd_ForceSave); PIN_C(Cmd_RemoveAllItems); PIN_C(Cmd_RemoveAllTypedItems); PIN_C(Cmd_PlaceAtMe); PIN_C(Cmd_PlaceLeveledActorAtMe); PIN_C(Cmd_MoveToMarkerWithFade); PIN_C(Cmd_SetPos); PIN_C(Cmd_SetAngle);

}  // namespace

bool InitReport::allCritical() const {
    int found = 0;
    for (auto& h : hooks)
        if (installed(h.status) && (h.name == addr::SetStage.name || h.name == addr::TopicInfoRunResult.name ||
                                    h.name == addr::ScriptExecute.name))
            ++found;
    return found == 3;
}

CaptureHub* hub() { return g_hub; }
FormResolver& resolver() { return *g_res; }
FormResolver& defaultResolver() { return g_defaultResolver; }
const Unmapped& unmapped() { return g_unmapped; }

InitReport Init(CaptureHub& h, Role role, FormResolver* res) {
    InitReport rep;
    g_hub = &h;
    g_mainTid = GetCurrentThreadId();
    g_res = res ? res : &g_defaultResolver;
    g_pol.setGuest(role == Role::Guest);
    h.setEnabled(role == Role::Host);
    h.setWeakCauseResolver([](uint32_t scriptId) {
        Cause c;
        c.kind = CauseKind::Script;
        FormRef r;
        if (g_res && g_res->toRef(scriptId, &r)) c.source = r;
        return c;
    });
    if (role == Role::Host) h.setScopeObservers(globalsBegin, globalsEnd);
    if (!g_reg) g_reg = std::make_unique<HookRegistry>(g_mem);
    for (const Entry& e : kEntries) {
        if (e.hostOnly && role == Role::Client) continue;
        if (e.guestOnly && role != Role::Guest) continue;
        Addr tramp = 0;
        PatchStatus st = g_reg->acquire(*e.spec, reinterpret_cast<Addr>(e.detour), &tramp, reinterpret_cast<Addr*>(e.origOut), e.chainForeign);
        if (installed(st)) {
            *reinterpret_cast<void**>(e.origOut) = reinterpret_cast<void*>(tramp);
            ++rep.ok;
        } else {
            ++rep.failed;
        }
        rep.hooks.push_back({e.spec->name, st});
    }
    if (g_storyOn && role != Role::Client) {   // story-capture: the RefCommand hooks + ForceActiveQuest (the three shared handlers are in kEntries)
        auto install = [&](const HookSpec& spec, void* detour, void* origOut) {
            Addr tramp = 0;
            PatchStatus st = g_reg->acquire(spec, reinterpret_cast<Addr>(detour), &tramp, reinterpret_cast<Addr*>(origOut));
            if (installed(st)) {
                *reinterpret_cast<void**>(origOut) = reinterpret_cast<void*>(tramp);
                ++rep.ok;
            } else {
                ++rep.failed;
            }
            rep.hooks.push_back({spec.name, st});
        };
        if (kStorySpecCount == kStoryN) {
            static const auto kStory = makeStoryEntries(std::make_index_sequence<kStoryN>{});
            for (const StoryEntry& e : kStory)
                if (!(e.spec->flags & kRoSharedHook)) install(*e.spec->hook, e.detour, e.origOut);
        } else {
            ++rep.failed;   // the op table and the spec table drifted apart: install nothing rather than the wrong detours
            rep.hooks.push_back({"story specs", PatchStatus::BadSpec});
        }
        install(story_addr::Cmd_ForceActiveQuest, reinterpret_cast<void*>(&h_ForceActiveQuest), reinterpret_cast<void*>(&o_ForceActiveQuest));
    }
    return rep;
}

int auditTamperedHooks() { return g_reg ? g_reg->auditTampered() : 0; }
std::string tamperedHookTargets() {
    std::vector<Addr> t;
    if (g_reg) g_reg->auditTampered(&t);
    std::string s;
    char b[16];
    for (Addr a : t) { std::snprintf(b, sizeof b, "%s0x%08X", s.empty() ? "" : " ", static_cast<unsigned>(a)); s += b; }
    return s;
}

std::string chainedHookTargets() {
    std::string s;
    if (!g_reg) return s;
    char b[32];
    for (const Entry& e : kEntries) {
        if (!e.chainForeign) continue;
        if (Addr to = g_reg->chainedOver(e.spec->target)) {
            std::snprintf(b, sizeof b, "%s0x%08X->0x%08X", s.empty() ? "" : " ", static_cast<unsigned>(e.spec->target), static_cast<unsigned>(to));
            s += b;
        }
    }
    return s;
}

void Shutdown() {
    if (g_hub) g_hub->setEnabled(false);
    g_reg.reset();  // destructor restores every patched entry that is still ours
    g_hub = nullptr;
    g_pol.setGuest(false);
}

namespace {
std::function<bool(const std::string&, uint32_t)> g_consoleExec;
bool g_controlsLocked = false;
}  // namespace
void setConsoleExec(std::function<bool(const std::string&, uint32_t)> f) { g_consoleExec = std::move(f); }
std::function<bool(const std::string&, uint32_t)> consoleExec() { return g_consoleExec; }
void setStoryCapture(bool on) { g_storyOn = on; }
bool storyCaptureEnabled() { return g_storyOn; }
bool playerControlsLocked() { return g_controlsLocked; }
void SetPlayerControlsLocked(bool locked) {
    if (locked == g_controlsLocked || !g_consoleExec) return;
    g_pol.beginApply();  // our own console line must not be dropped by the guest gate
    ApplierExecScope scope;
    bool ok = g_consoleExec(locked ? "DisablePlayerControls 1 1 1 1 1 1 1" : "EnablePlayerControls 1 1 1 1 1 1 1", 0);
    g_pol.endApply();
    if (ok) g_controlsLocked = locked;
}

void setGuestLocalRun(const std::vector<FormRef>& scripts) {
    std::unordered_set<uint32_t> ids;
    for (const FormRef& r : scripts) {
        uint32_t id = g_res ? g_res->fromRef(r) : 0;
        if (id) ids.insert(id);
    }
    g_pol.setGuestLocalRun(std::move(ids));
    std::unordered_set<uint32_t> hostState;   // the DLC setup scripts whose quest state is the host's (guest_local_run.h); only those the allow-list also has count
    for (const FormRef& r : hostQuestStateScripts()) {
        uint32_t id = g_res ? g_res->fromRef(r) : 0;
        if (id) hostState.insert(id);
    }
    g_pol.setHostQuestState(std::move(hostState));
}

void setStageGate(StageGateFn f) {
    g_stageGateFn = f;
    if (!f) { g_stageGate.set(nullptr); return; }
    g_stageGate.set([](uint32_t quest, uint32_t stage) {
        FormRef qr;
        return !g_res || !g_res->toRef(quest, &qr) || g_stageGateFn(qr, stage);
    });
}
void setStageRequestObserver(StageRequestObserver obs) { g_stageReqObs = std::move(obs); }
uint64_t stageGateDeferred() { return g_stageGate.deferred(); }
StageGateBypass::StageGateBypass() { g_stageGate.beginBypass(); }
StageGateBypass::~StageGateBypass() { g_stageGate.endBypass(); }
std::vector<std::pair<uint32_t, double>> questVarsDump(const FormRef& quest) {
    std::vector<std::pair<uint32_t, double>> out;
    uint32_t id = g_res ? g_res->fromRef(quest) : 0;
    TESQuest* q = static_cast<TESQuest*>(id ? g_res->lookup(id) : nullptr);
    ScriptEventList* el = q ? q->scriptEventList : nullptr;
    if (!el || !el->m_vars) return out;
    for (auto v = el->m_vars->Begin(); !v.End(); ++v)
        if (v.Get()) out.emplace_back(v.Get()->id, v.Get()->data);
    return out;
}
// The variable NAME of a quest script variable index (the rules engine knows per-player variables by name, the engine's ScriptEventList by index).
bool questVarName(const FormRef& quest, uint32_t idx, std::string* name) {
    uint32_t id = g_res ? g_res->fromRef(quest) : 0;
    TESQuest* q = static_cast<TESQuest*>(id ? g_res->lookup(id) : nullptr);
    char buf[128] = {};
    bool found = false;
    __try {
        Script* sc = q ? q->scriptable.script : nullptr;
        if (sc) {
            for (auto v = sc->varList.Begin(); !v.End(); ++v) {
                VariableInfo* vi = v.Get();
                if (vi && vi->idx == idx && vi->name.m_data) {
                    strncpy_s(buf, sizeof buf, vi->name.m_data, _TRUNCATE);
                    found = true;
                    break;
                }
            }
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        found = false;
    }
    if (found && name) *name = buf;
    return found;
}
namespace {
// Is variable `idx` of the quest script a REF variable? The script's refList has one entry per ref local (varIdx = the local's index; immediates have 0).
bool questVarIsRef(void* questForm, uint32_t idx) {
    __try {
        TESQuest* q = static_cast<TESQuest*>(questForm);
        Script* sc = q ? q->scriptable.script : nullptr;
        if (!sc) return false;
        for (auto it = sc->refList.Begin(); !it.End(); ++it) {
            Script::RefVariable* rv = it.Get();
            if (rv && rv->varIdx == idx && idx != 0) return true;
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
    return false;
}
}  // namespace
// A ref variable holds its form id in the low dword of the 8 byte slot (ref_var.h); value / *out are the form id as a plain number for those.
bool questVarValue(const FormRef& quest, const std::string& name, double* out) {
    uint32_t id = g_res ? g_res->fromRef(quest) : 0;
    void* q = id ? g_res->lookup(id) : nullptr;
    uint32_t idx = 0;
    double* slot = nullptr;
    if (!q || !questVarSlot(q, name.c_str(), &idx, &slot)) return false;
    if (out) *out = questVarIsRef(q, idx) ? refVarNumberFromSlot(*slot) : *slot;
    return true;
}
bool setQuestVarValue(const FormRef& quest, const std::string& name, double value) {
    uint32_t id = g_res ? g_res->fromRef(quest) : 0;
    void* q = id ? g_res->lookup(id) : nullptr;
    uint32_t idx = 0;
    double* slot = nullptr;
    if (!q || !questVarSlot(q, name.c_str(), &idx, &slot)) return false;
    *slot = questVarIsRef(q, idx) ? refVarSlotFromNumber(value) : value;
    return true;
}
int localQuestStage(const FormRef& quest) {
    uint32_t id = g_res ? g_res->fromRef(quest) : 0;
    char* q = static_cast<char*>(id ? g_res->lookup(id) : nullptr);
    return q ? *reinterpret_cast<UInt8*>(q + 0x60) : -1;
}
int localQuestFlags(const FormRef& quest) {
    uint32_t id = g_res ? g_res->fromRef(quest) : 0;
    TESForm* f = id ? static_cast<TESForm*>(g_res->lookup(id)) : nullptr;
    if (!f || f->typeID != kFormType_TESQuest) return -1;
    return static_cast<TESQuest*>(f)->flags;
}
bool localGlobalValue(const FormRef& global, double& out) {
    uint32_t id = g_res ? g_res->fromRef(global) : 0;
    TESForm* f = id ? static_cast<TESForm*>(g_res->lookup(id)) : nullptr;
    if (!f || f->typeID != kFormType_TESGlobal) return false;
    out = static_cast<TESGlobal*>(f)->data;
    return true;
}
bool localQuestObjectives(const FormRef& quest, std::map<uint32_t, uint8_t>& out) {
    uint32_t id = g_res ? g_res->fromRef(quest) : 0;
    TESForm* f = id ? static_cast<TESForm*>(g_res->lookup(id)) : nullptr;
    if (!f || f->typeID != kFormType_TESQuest) return false;
    TESQuest* q = static_cast<TESQuest*>(f);
    // The list holds objectives and script variables; the data pointers ARE BGSQuestObjective* (same read as the baseline dump).
    for (auto l = q->lVarOrObjectives.Begin(); !l.End(); ++l) {
        BGSQuestObjective* ob = reinterpret_cast<BGSQuestObjective*>(l.Get());
        if (ob && ob->quest == q) out[ob->objectiveId] = static_cast<uint8_t>(ob->status & 3u);
    }
    return true;
}
bool execConsoleLine(const std::string& line) { return g_consoleExec && g_consoleExec(line, 0); }
bool execConsoleLineOn(const std::string& line, uint32_t refID) {
    if (isSayLine(line.c_str())) g_spectatorSay.arm(refID, GetTickCount64());   // the engine's Say runs the INFO's result scripts itself: see spectator_say.h
    return g_consoleExec && g_consoleExec(line, refID);
}
uint64_t spectatorSayBlocked() { return g_spectatorSay.blockedCount(); }

bool killWithAttribution(uint32_t victimRefID, uint32_t killerRefID) {
    if (!victimRefID || !g_res || !g_res->lookup(victimRefID)) return false;
    g_pol.beginApply();   // our own console line must not be dropped by the guest gate
    ApplierExecScope scope;
    const bool ok = execConsoleLineOn(killerRefID ? killByCmd(killerRefID) : killCmd(), victimRefID);
    g_pol.endApply();
    return ok;
}
static bool storeKillerSeh(void* victim, void* killer) {
    __try {
        return storeKiller(victim, static_cast<TESForm*>(victim)->typeID, killer);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool setKillerOf(uint32_t victimRefID, uint32_t killerRefID) {
    if (!g_res) return false;
    void* v = g_res->lookup(victimRefID);
    void* k = killerRefID ? g_res->lookup(killerRefID) : nullptr;
    return v && (k || !killerRefID) && storeKillerSeh(v, k);
}
uint32_t killerOf(uint32_t victimRefID) {
    void* v = g_res ? g_res->lookup(victimRefID) : nullptr;
    if (!v) return 0;
    __try {
        void* k = loadKiller(v, static_cast<TESForm*>(v)->typeID);
        return k ? static_cast<TESForm*>(k)->refID : 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

void setActivateObserver(ActivateObserver obs) { g_activateObs = std::move(obs); }
void setEventObserver(EventObserver obs) { g_eventObs = std::move(obs); }
void setNonPlayerAvObserver(NonPlayerAvObserver obs) { g_npAvObs = std::move(obs); }
bool blockMutationNow() { return g_pol.blockMutation(); }

void enableStatePolling(bool on, bool prime) {
    g_statePolling = on;
    g_tracker.clear();
    g_tracker.endPrime();
    if (on && prime) g_tracker.beginPrime();
}

size_t pollHostState(StateSink& sink) {
    size_t n = 0;
    forQuestVars([&](TESQuest* q, uint32_t id, double v) {
        if (id > 0xFFFFu || isLocalQuest(q)) return;
        if (!g_tracker.update(ChangeTracker::varKey(q->refID, static_cast<uint16_t>(id)), v)) return;
        FormRef qr;
        if (formRef(q, &qr)) {
            sink.scriptVar(qr, static_cast<uint16_t>(id), v);
            ++n;
        }
    });
    DataHandler* dh = *reinterpret_cast<DataHandler**>(kDataHandlerPtr);
    if (dh) {
        for (auto it = dh->globalList.Begin(); !it.End(); ++it) {
            TESGlobal* g = it.Get();
            if (!g || !g_tracker.update(ChangeTracker::globalKey(g->refID), g->data)) continue;
            FormRef gr;
            if (formRef(g, &gr)) {
                sink.global(gr, g->data);
                ++n;
            }
        }
    }
    g_tracker.endPrime();  // a priming pass is one full pass: from now on changes are reported
    return n;
}

GuestItemScope::GuestItemScope(uint8_t slot) { g_guestItems.push(slot); }
GuestItemScope::~GuestItemScope() { g_guestItems.pop(); }
ActingPlayerScope::ActingPlayerScope(uint32_t id, int forcedSlot) { g_acting.push(id, forcedSlot); }
ActingPlayerScope::~ActingPlayerScope() { g_acting.pop(); }
void setScriptPackageObserver(ScriptPackageObserver obs) { g_pkgObs = std::move(obs); }
void setPartyStateProvider(StateProvider provider) { g_stateProvider = std::move(provider); }
void setPlayerFacts(const std::vector<PlayerFact>* facts) { g_facts = facts; }
PlayerFactsScope::PlayerFactsScope(const std::vector<PlayerFact>* facts) : prev_(g_facts) { g_facts = facts; }
PlayerFactsScope::~PlayerFactsScope() { g_facts = prev_; }
void setScriptRunObserver(ScriptRunObserver obs) { g_scriptRunObs = std::move(obs); }
void setScriptRunScope(ScriptRunScopeHook hook) { g_scriptRunScope = std::move(hook); }
void setControlsRouter(ControlsRouter router) { g_controlsRouter = std::move(router); }
void setPlayerKillObserver(PlayerKillObserver obs) { g_playerKillObs = std::move(obs); }
void setPlayerRefProvider(PlayerRefOf f) { g_playerRefOf = std::move(f); }
void setPlayerStashObserver(PlayerStashObserver obs) { g_stashObs = std::move(obs); }
void setPerSlotScripts(std::vector<PerSlotScript> scripts, PerSlotRunner runner) {
    g_perSlot.set(std::move(scripts));
    g_perSlotRunner = std::move(runner);
    // (the object variable copies are NOT cleared here: they belong to the host's save game and were restored from its sidecar before the session wired this up)
}
ObjVarCopies& objVarCopies() { return g_objCopies; }
const PerSlotStats& perSlotStats() { return g_perSlotStats; }
void setInfoQuestScopes(InfoQuestScopes::Lookup lookup) { g_infoScopes.set(std::move(lookup)); }
uint64_t infoScopesOpened() { return g_infoScopesOpened; }
bool playerItemCount(uint32_t whoRefID, const FormRef& item, int64_t* count) {
    if (!g_res || !o_Cond_GetItemCount) return false;
    void* who = whoRefID == kPlayerRefID ? thePlayer() : g_res->lookup(whoRefID);
    void* it = g_res->lookup(g_res->fromRef(item));
    if (!who || !it) return false;
    double v = 0;
    bool ok = false;
    __try {
        ok = o_Cond_GetItemCount(who, it, nullptr, &v);   // the engine's own evaluator: no party reduction, the count of exactly this reference
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
    if (count) *count = static_cast<int64_t>(v);
    return ok;
}
void setNoteObserver(NoteObserver obs) { g_noteObs = std::move(obs); }
void setPartyConditions(PartyProvider provider, ConditionReducer reducer) {
    g_partyProvider = std::move(provider);
    g_condReducer = std::move(reducer);
}
const CondStats& condStats() { return g_condStats; }
ConditionQuestScope::ConditionQuestScope(const FormRef& quest, fnvcoop::CondPhase phase) { g_condQuests.push_back({quest, phase}); }
ConditionQuestScope::~ConditionQuestScope() { if (!g_condQuests.empty()) g_condQuests.pop_back(); }
std::vector<std::pair<uint32_t, uint32_t>> localRunCounts() { return g_localRunCounts; }
void setHookPerf(bool on) {
    if (on) {
        g_perf = {};
        LARGE_INTEGER f;
        QueryPerformanceFrequency(&f);
        g_perf.freq = static_cast<uint64_t>(f.QuadPart);
    }
    g_perfOn = on;
}
HookPerf hookPerf() { return g_perf; }
const Suppressed& suppressStats() { return g_pol.stats(); }
uint32_t actingPlayer() { return g_acting.current(); }
void registerPlayerSlot(uint32_t ghostRefID, uint8_t slot) { g_acting.registerSlot(ghostRefID, slot); }
void clearPlayerSlots() { g_acting.clearSlots(); }
uint8_t actingSlot() { return g_acting.slot(); }

LocalPlayerScope::LocalPlayerScope() : active_(g_mainTid == 0 || GetCurrentThreadId() == g_mainTid) {
    if (active_) g_pol.enterLocalPlayerScope();
}
LocalPlayerScope::~LocalPlayerScope() {
    if (active_) g_pol.leaveLocalPlayerScope();
}
bool inLocalPlayerScope() { return g_pol.inLocalPlayerScope(); }

ApplierExecScope::ApplierExecScope() { ++g_applierExec; }
ApplierExecScope::~ApplierExecScope() { --g_applierExec; }

ApplyScope::ApplyScope() { g_pol.beginApply(); }
ApplyScope::~ApplyScope() { g_pol.endApply(); }
void setGuestQuestGate(bool on) { g_pol.setQuestGate(on); }
const Suppressed& suppressed() { return g_pol.stats(); }
const ScriptStats& scriptStats() { return g_sstats; }
bool applying() { return g_pol.applying(); }

ScriptGate::ScriptGate() { ++g_gate; }
ScriptGate::~ScriptGate() { --g_gate; }
int scriptGateDepth() { return g_gate; }

}  // namespace nvhooks
