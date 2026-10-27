#include "apply_remote.h"

#include <Windows.h>

#include <algorithm>
#include <cmath>
#include <memory>
#include <vector>

#include "GameAPI.h"
#include "GameExtraData.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "apply_lines.h"
#include "effect_json.h"
#include "engine_addrs.h"
#include "ref_ops.h"
#include "said_once.h"

using namespace fnvcoop;

namespace nvhooks {
namespace {

constexpr Addr kThePlayerPtr = 0x11DEA3C;
constexpr uint32_t kPseudoAV_XP = fnvcoop::kAvExperience;
constexpr uint32_t kKarmaAV = 23;

inline void* thePlayer() { return *reinterpret_cast<void**>(kThePlayerPtr); }

template <class R, class... A>
inline R tc(Addr fn, void* self, A... a) {
    return reinterpret_cast<R(__thiscall*)(void*, A...)>(fn)(self, a...);
}
// virtual call by byte offset into the vtable
template <class R, class... A>
inline R vc(void* self, uint32_t byteOff, A... a) {
    void** vt = *reinterpret_cast<void***>(self);
    return reinterpret_cast<R(__thiscall*)(void*, A...)>(vt[byteOff / 4])(self, a...);
}
inline float& repField(void* rep, int type) {
    return *reinterpret_cast<float*>(reinterpret_cast<char*>(rep) + (type == 1 ? 0x44 : 0x48));
}
inline float repMax(void* rep) { return *reinterpret_cast<float*>(reinterpret_cast<char*>(rep) + 0x40); }

}  // namespace

namespace {
std::vector<RemoteApplier*>& instances() {
    static std::vector<RemoteApplier*> v;
    return v;
}
std::function<bool(uint32_t)>& defaultRemoteDriven() {
    static std::function<bool(uint32_t)> f;
    return f;
}
}  // namespace

const char* applyStatusName(ApplyStatus s) {
    switch (s) {
        case ApplyStatus::Ok: return "Ok";
        case ApplyStatus::Unsupported: return "Unsupported";
        case ApplyStatus::FormMissing: return "FormMissing";
        case ApplyStatus::NotForMe: return "NotForMe";
        case ApplyStatus::EngineRejected: return "EngineRejected";
    }
    return "?";
}

bool RemoteApplier::forMe(const Recipient& r, PlayerId origin, PlayerId me) const {
    switch (r.mode) {
        case Recipient::All: return true;
        case Recipient::Origin: return origin == me;
        case Recipient::Mask: return r.includes(me);
        case Recipient::Default: return true;  // uncommitted: host policy has not narrowed it; apply
    }
    return false;
}

namespace {
// First enabled, positioned reference in a loaded cell (a MoveTo anchor for cross-cell absolute moves).
const TESObjectREFR* cellAnchor(TESObjectCELL* cell) {
    int guard = 0;
    for (auto* n = &cell->objectList.m_listHead; n && guard < 8192; n = n->next, ++guard) {
        TESObjectREFR* r = n->data;
        if (!r || !r->baseForm || r->typeID != kFormType_TESObjectREFR) continue;
        if (r->flags & 0x800) continue;  // initially disabled
        return r;
    }
    return nullptr;
}
}  // namespace

ApplyStatus RemoteApplier::apply(const Effect& e) {
    EchoGuard echo(&hub_.echo());  // hooks must not re-capture what we apply
    ApplyScope applyScope;         // lifts the guest suppression for host-authored effects
    std::unique_ptr<ScriptGate> gate;  // and no result script may run as a side effect
    if (useGate_) gate.reset(new ScriptGate);
    err_.clear();
    const ExecFn exec = pickExec(exec_, consoleExec());   // coop_module.cpp builds its applier without an executor of its own

    auto form = [&](const FormRef& r) -> void* {
        uint32_t id = res_.fromRef(r);
        void* f = id ? res_.lookup(id) : nullptr;
        if (!f) err_ = "form missing: " + r.plugin;
        return f;
    };

    if (auto* x = std::get_if<SetStage>(&e)) {
        void* q = form(x->quest);
        if (!q) return ApplyStatus::FormMissing;
        bool ok = tc<bool, UInt8>(addr::SetStage.target, q, static_cast<UInt8>(x->stage));
        if (!ok) err_ = "engine SetStage returned false (stage not defined in this quest, or quest data missing)";
        return ok ? ApplyStatus::Ok : ApplyStatus::EngineRejected;
    }
    if (auto* x = std::get_if<SetObjective>(&e)) {
        void* q = form(x->quest);
        if (!q) return ApplyStatus::FormMissing;
        void* obj = tc<void*, UInt32>(addr::QuestGetObjective.target, q, x->objective);
        if (!obj) { err_ = "objective " + std::to_string(x->objective) + " not found in quest"; return ApplyStatus::EngineRejected; }
        tc<void, UInt32>(addr::ObjectiveSetState.target, obj, x->flags & 3u);
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<SetGlobal>(&e)) {
        void* g = form(x->global);
        if (!g) return ApplyStatus::FormMissing;
        *reinterpret_cast<float*>(reinterpret_cast<char*>(g) + 0x24) = x->value;  // TESGlobal::data
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<EnableRef>(&e)) {
        void* r = form(x->ref);
        if (!r) return ApplyStatus::FormMissing;
        if (exec) {
            gate.reset();
            ApplierExecScope scope;
            return exec("Enable", res_.fromRef(x->ref)) ? ApplyStatus::Ok : ApplyStatus::EngineRejected;
        }
        tc<void, bool>(addr::RefSetDisabled.target, r, false);
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<DisableRef>(&e)) {
        void* r = form(x->ref);
        if (!r) return ApplyStatus::FormMissing;
        if (exec) {
            gate.reset();
            ApplierExecScope scope;
            return exec("Disable", res_.fromRef(x->ref)) ? ApplyStatus::Ok : ApplyStatus::EngineRejected;
        }
        tc<void, bool>(addr::RefSetDisabled.target, r, true);
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<AddItem>(&e)) {
        void* it = form(x->item);
        void* p = thePlayer();
        if (!it || !p) return ApplyStatus::FormMissing;
        static_cast<TESObjectREFR*>(p)->AddItem(static_cast<TESForm*>(it), nullptr, static_cast<UInt32>(x->count));
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<RemoveItem>(&e)) {
        void* it = form(x->item);
        void* p = thePlayer();
        if (!it || !p) return ApplyStatus::FormMissing;
        // Same trailing arguments NVSE's own RemoveItem wrappers use: no extra list, not dropped into the world.
        static_cast<TESObjectREFR*>(p)->RemoveItem(static_cast<TESForm*>(it), nullptr, static_cast<UInt32>(x->count), 0, 0,
                                                    nullptr, 0, 0, 0x3F800000, 0);
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<ModActorValue>(&e)) {
        void* p = thePlayer();
        if (!p) return ApplyStatus::FormMissing;
        if (x->av == kPseudoAV_XP) { vc<void, int>(p, 0x488, x->delta); return ApplyStatus::Ok; }
        vc<void, uint32_t, float, void*>(p, 0x3A8, x->av, static_cast<float>(x->delta), nullptr);
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<ModKarma>(&e)) {
        void* p = thePlayer();
        if (!p) return ApplyStatus::FormMissing;
        tc<void, int>(addr::PlayerRewardKarma.target, p, x->delta);
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<ModReputation>(&e)) {
        void* rep = form(x->faction);
        if (!rep) return ApplyStatus::FormMissing;
        auto bump = [&](int type, int delta) {
            if (!delta) return;
            float v = repField(rep, type) + static_cast<float>(delta);
            if (v < 0) v = 0;
            if (repMax(rep) > 0 && v > repMax(rep)) v = repMax(rep);
            tc<void, int, float>(addr::RepSet.target, rep, type, v);
        };
        bump(1, x->fame);
        bump(0, x->infamy);
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<SetScriptVar>(&e)) {
        // Quest owner: TESQuest+0x5C. Ref owner: the ScriptEventList in the ref's ExtraScript (extra data 0x0D).
        TESForm* f = static_cast<TESForm*>(form(x->owner));
        if (!f) return ApplyStatus::FormMissing;
        ScriptEventList* el = nullptr;
        if (f->typeID == kFormType_TESQuest) {
            el = static_cast<TESQuest*>(f)->scriptEventList;
        } else if (f->typeID == kFormType_TESObjectREFR || f->typeID == kFormType_Character || f->typeID == kFormType_Creature) {
            BSExtraData* xd = static_cast<TESObjectREFR*>(f)->extraDataList.GetByType(kExtraData_Script);
            el = xd ? static_cast<ExtraScript*>(xd)->eventList : nullptr;
        } else {
            err_ = "SetScriptVar: owner is neither a quest nor a reference";
            return ApplyStatus::Unsupported;
        }
        if (!el || !el->m_vars) { err_ = "owner has no script variables (script not instantiated yet?)"; return ApplyStatus::EngineRejected; }
        for (auto it = el->m_vars->Begin(); !it.End(); ++it)
            if (it.Get() && it.Get()->id == x->index) { it.Get()->data = x->value; return ApplyStatus::Ok; }
        err_ = "variable index not found";
        return ApplyStatus::EngineRejected;
    }
    if (auto* x = std::get_if<CompleteQuest>(&e)) {
        void* q = form(x->quest);
        if (!q) return ApplyStatus::FormMissing;
        tc<void, bool>(addr::QuestSetCompleted.target, q, true);
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<CompleteAllObjectives>(&e)) {
        void* q = form(x->quest);
        if (!q) return ApplyStatus::FormMissing;
        tc<void, UInt32>(addr::QuestCompleteAllObjectives.target, q, 1);
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<SetPlayerControls>(&e)) {
        void* p = thePlayer();
        if (!p) return ApplyStatus::FormMissing;
        tc<void, UInt32, UInt32>(addr::PlayerSetControlsDisabled.target, p, x->enable ? 0u : 1u, x->controls & 0xFFu);
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<FailQuest>(&e)) {
        void* q = form(x->quest);
        if (!q) return ApplyStatus::FormMissing;
        tc<void, bool>(addr::QuestSetFailed.target, q, true);  // sets flags 0x40|0x02 (failed implies completed)
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<StartQuest>(&e)) {
        void* q = form(x->quest);
        if (!q) return ApplyStatus::FormMissing;
        tc<void, bool>(addr::QuestSetRunning.target, q, true);
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<StopQuest>(&e)) {
        void* q = form(x->quest);
        if (!q) return ApplyStatus::FormMissing;
        tc<void, bool>(addr::QuestSetRunning.target, q, false);
        return ApplyStatus::Ok;
    }
    if (exec) {
        auto run = [&](const std::string& cmd, uint32_t refID) {
            ApplierExecScope scope;  // not a leaked result script: counted separately
            if (exec(cmd, refID)) return ApplyStatus::Ok;
            err_ = "console command failed: " + cmd;
            return ApplyStatus::EngineRejected;
        };
        if (auto* x = std::get_if<KillActor>(&e)) {
            uint32_t id = res_.fromRef(x->actor);
            if (!id || !res_.lookup(id)) { err_ = "form missing: " + x->actor.plugin; return ApplyStatus::FormMissing; }
            gate.reset();
            return run(killCmd(), id);
        }
        if (auto* x = std::get_if<ResurrectActor>(&e)) {
            uint32_t id = res_.fromRef(x->actor);
            if (!id || !res_.lookup(id)) { err_ = "form missing: " + x->actor.plugin; return ApplyStatus::FormMissing; }
            gate.reset();
            return run(resurrectCmd(false), id);
        }
        // ---- effects applied by running the engine's own console command ON the target reference (calling reference) ----
        auto idOf = [&](const FormRef& r, uint32_t* out) {
            uint32_t id = res_.fromRef(r);
            if (!id || !res_.lookup(id)) { err_ = "form missing: " + r.plugin; return false; }
            *out = id;
            return true;
        };
        if (auto* x = std::get_if<SetFactionRank>(&e)) {
            uint32_t a, f;
            if (!idOf(x->actor, &a) || !idOf(x->faction, &f)) return ApplyStatus::FormMissing;
            gate.reset();
            return run(x->rank < 0 ? removeFromFactionCmd(f) : setFactionRankCmd(f, x->rank), a);
        }
        if (auto* x = std::get_if<SetPerk>(&e)) {
            uint32_t p;
            if (!idOf(x->perk, &p)) return ApplyStatus::FormMissing;
            gate.reset();
            return run(x->rank > 0 ? addPerkCmd(p) : removePerkCmd(p), 0x14);
        }
        if (auto* x = std::get_if<ObjectState>(&e)) {
            uint32_t r;
            if (!idOf(x->ref, &r)) return ApplyStatus::FormMissing;
            gate.reset();
            ApplyStatus st = ApplyStatus::Ok;
            if (st == ApplyStatus::Ok && (x->mask & kObjLock)) st = run(x->locked ? lockCmd(x->level) : unlockCmd(), r);
            if (st == ApplyStatus::Ok && (x->mask & kObjOpen)) st = run(setOpenStateCmd(x->open), r);
            if (st == ApplyStatus::Ok && (x->mask & kObjDestroyed)) st = run(setDestroyedCmd(x->destroyed), r);
            return st;
        }
        if (auto* x = std::get_if<RefAddItem>(&e)) {
            uint32_t r, i;
            if (!idOf(x->ref, &r) || !idOf(x->item, &i)) return ApplyStatus::FormMissing;
            gate.reset();
            return run(addItemCmd(i, x->count), r);
        }
        if (auto* x = std::get_if<RefRemoveItem>(&e)) {
            uint32_t r, i;
            if (!idOf(x->ref, &r) || !idOf(x->item, &i)) return ApplyStatus::FormMissing;
            gate.reset();
            return run(removeItemCmd(i, x->count), r);
        }
        if (auto* x = std::get_if<RemoveAllItems>(&e)) {
            uint32_t r;
            if (!idOf(x->ref, &r)) return ApplyStatus::FormMissing;
            gate.reset();
            if (x->undo) {   // put back the contents the clear discarded (rebase after a host load)
                ApplyStatus st = ApplyStatus::Ok;
                for (auto& [f, c] : x->items) {
                    uint32_t i;
                    if (!idOf(f, &i)) return ApplyStatus::FormMissing;
                    st = run(addItemCmd(i, c), r);
                    if (st != ApplyStatus::Ok) return st;
                }
                return st;
            }
            return run(removeAllItemsCmd(), r);
        }
        if (auto* x = std::get_if<ContainerTransfer>(&e)) {   // this machine's own player and its own copy of the container
            uint32_t src = 0x14, dst = 0;   // 0x14: the player
            if (!x->srcPlayer && !idOf(x->src, &src)) return ApplyStatus::FormMissing;
            if (!x->dstPlayer && !x->dst.plugin.empty() && !idOf(x->dst, &dst)) return ApplyStatus::FormMissing;
            gate.reset();
            return run(removeAllItemsToCmd(dst, x->dstPlayer), src);
        }
        if (auto* x = std::get_if<AddNote>(&e)) { uint32_t n; if (!idOf(x->note, &n)) return ApplyStatus::FormMissing; gate.reset(); return run(addNoteCmd(n), 0); }
        if (auto* x = std::get_if<RemoveNote>(&e)) { uint32_t n; if (!idOf(x->note, &n)) return ApplyStatus::FormMissing; gate.reset(); return run(removeNoteCmd(n), 0); }
        if (auto* x = std::get_if<SayOnceFlag>(&e)) {   // a client's INFO learns it was spoken (said_once.h)
            TESForm* f = static_cast<TESForm*>(form(x->info));
            if (!f) return ApplyStatus::FormMissing;
            if (f->typeID != kFormType_TESTopicInfo) { err_ = "SayOnceFlag: the form is not an INFO"; return ApplyStatus::Unsupported; }
            markInfoSaidOnce(f);
            return ApplyStatus::Ok;
        }
        if (auto* x = std::get_if<AddTopic>(&e)) { uint32_t t; if (!idOf(x->topic, &t)) return ApplyStatus::FormMissing; gate.reset(); return run(addTopicCmd(t), 0); }
        if (auto* x = std::get_if<SetWeather>(&e)) { uint32_t w; if (!idOf(x->weather, &w)) return ApplyStatus::FormMissing; gate.reset(); return run(setWeatherCmd(w, x->force), 0); }
        if (auto* x = std::get_if<SetMapMarker>(&e)) {
            uint32_t m;
            if (!idOf(x->marker, &m)) return ApplyStatus::FormMissing;
            gate.reset();
            return run(showMapCmd(m, (x->flags & kMarkerTravel) != 0), 0);
        }
        if (auto* x = std::get_if<SetFastTravelLock>(&e)) {
            gate.reset();
            if (!travelLocks_.set(x->scope, x->on)) return ApplyStatus::Ok;  // another scope still holds / already released: no engine change
            return run(enableFastTravelCmd(travelLocks_.travelEnabled()), 0);
        }
        if (auto* x = std::get_if<CompanionOwner>(&e)) {
            if (!companionForMe(x->owner, localSlot_)) return ApplyStatus::NotForMe;
            uint32_t id;
            if (!idOf(x->npc, &id)) return ApplyStatus::FormMissing;
            gate.reset();
            return run(setTeammateCmd(x->active), id);
        }
        if (auto* x = std::get_if<SetTimeScale>(&e)) { gate.reset(); return run(setTimeScaleCmd(x->scale), 0); }
        if (auto* x = std::get_if<RefAngle>(&e)) {
            uint32_t id;
            if (!idOf(x->ref, &id)) return ApplyStatus::FormMissing;
            if (x->axis > kAxisZ) { err_ = "RefAngle: bad axis"; return ApplyStatus::Unsupported; }
            gate.reset();
            return run(setAngleCmd("XYZ"[x->axis], x->degrees), id);
        }
        if (auto* x = std::get_if<MoveRef>(&e)) return applyMove(*x, exec, true, &gate);
        if (auto* x = std::get_if<RefCommand>(&e)) return applyRefCommand(*x, exec, &gate);
        if (auto* x = std::get_if<ForceActiveQuest>(&e)) {
            uint32_t q;
            if (!idOf(x->quest, &q)) return ApplyStatus::FormMissing;
            gate.reset();
            return run(forceActiveQuestCmd(q), 0);
        }
    }
    err_ = std::string("unsupported effect: ") + effectToJson(e).get("type").asString("?");
    return ApplyStatus::Unsupported;  // MoveRef, KillActor, ResurrectActor
}

RemoteApplier::RemoteApplier(CaptureHub& hub, FormResolver& res) : hub_(hub), res_(res) { instances().push_back(this); }
RemoteApplier::~RemoteApplier() {
    auto& v = instances();
    v.erase(std::remove(v.begin(), v.end(), this), v.end());
}

// MoveRef: `cell` carries a marker reference (x/y/z are offsets from it) or a cell (x/y/z are absolute). Same-cell absolute moves use SetPos per
// axis; a cross-cell move uses any loaded reference of the destination cell as the MoveTo anchor. When the destination cell has no loaded reference
// here, the move is PARKED (park = true) and retried by tickPending() until that cell loads: a reference whose cell is away is not simulated on this
// machine, and where it ends up has to be where the host left it when the cell comes back.
ApplyStatus RemoteApplier::applyMove(const MoveRef& x, const ExecFn& exec, bool park, std::unique_ptr<ScriptGate>* gate) {
    auto run = [&](const std::string& cmd, uint32_t refID) {
        ApplierExecScope scope;
        if (exec(cmd, refID)) return ApplyStatus::Ok;
        err_ = "console command failed: " + cmd;
        return ApplyStatus::EngineRejected;
    };
    if (!exec) { err_ = "unsupported effect: MoveRef (no console executor)"; return ApplyStatus::Unsupported; }
    uint32_t id = res_.fromRef(x.ref), cellOrMarker = res_.fromRef(x.cell);
    TESObjectREFR* r = id ? static_cast<TESObjectREFR*>(res_.lookup(id)) : nullptr;
    if (!r) { err_ = "form missing: " + x.ref.plugin; return ApplyStatus::FormMissing; }
    TESForm* m = cellOrMarker ? static_cast<TESForm*>(res_.lookup(cellOrMarker)) : nullptr;
    if (gate) gate->reset();
    if (m && isReferenceFormType(m->typeID))  // `cell` carries a marker reference: x/y/z are offsets from it
        return run(moveToMarkerCmd(cellOrMarker, x.x, x.y, x.z), id);
    if (!r->parentCell || r->parentCell->refID != cellOrMarker) {
        const TESObjectREFR* anchor = m && m->typeID == kFormType_TESObjectCELL ? cellAnchor(static_cast<TESObjectCELL*>(m)) : nullptr;
        if (!anchor) {
            if (m && m->typeID == kFormType_TESObjectCELL && park) {
                moves_.add(x);
                err_.clear();
                return ApplyStatus::Ok;   // parked: reported Ok like a held world effect; tickPending() performs it
            }
            err_ = m && m->typeID == kFormType_TESObjectCELL ? "MoveRef: destination cell has no loaded reference to anchor on"
                                                             : "MoveRef: cross-cell move needs a loaded destination cell or a marker reference";
            return ApplyStatus::Unsupported;
        }
        return run(moveToMarkerCmd(anchor->refID, x.x - anchor->posX, x.y - anchor->posY, x.z - anchor->posZ), id);
    }
    const char axes[3] = {'X', 'Y', 'Z'};
    const float v[3] = {x.x, x.y, x.z};
    for (int i = 0; i < 3; ++i) {
        ApplyStatus st = run(setPosCmd(axes[i], v[i]), id);
        if (st != ApplyStatus::Ok) return st;
    }
    return ApplyStatus::Ok;
}

ApplyStatus RemoteApplier::applyRefCommand(const RefCommand& x, const ExecFn& exec, std::unique_ptr<ScriptGate>* gate) {
    const RefOpSpec* s = storySpecFor(x.op);
    if (!s) { err_ = "unknown RefCommand op"; return ApplyStatus::Unsupported; }
    if (!exec) { err_ = "unsupported effect: RefCommand (no console executor)"; return ApplyStatus::Unsupported; }
    LineResult lr = buildRefCommandLine(x, [&](const FormRef& r) -> uint32_t {
        uint32_t id = res_.fromRef(r);
        return id && res_.lookup(id) ? id : 0u;
    });
    if (!lr.ok) {
        err_ = lr.why;
        return lr.formMissing ? ApplyStatus::FormMissing : ApplyStatus::Unsupported;
    }
    if (!opPerformedHere(*s, lr.callingRef, remoteDriven_ ? remoteDriven_ : defaultRemoteDriven())) return ApplyStatus::NotForMe;   // another machine runs this actor's AI
    if (s->flags & kRoActivate) {
        const TESObjectREFR* target = lr.callingRef ? static_cast<TESObjectREFR*>(res_.lookup(lr.callingRef)) : nullptr;
        if (!target || !target->baseForm || !activateTypeSafe(target->baseForm->typeID)) return ApplyStatus::NotForMe;   // a menu / conversation belongs to its player
    }
    if (gate) gate->reset();   // the console line is a script: the ScriptGate must be open for it
    ApplierExecScope scope;
    if (exec(lr.line, lr.callingRef)) return ApplyStatus::Ok;
    err_ = "console command failed: " + lr.line;
    return ApplyStatus::EngineRejected;
}

void RemoteApplier::setDefaultRemoteDrivenQuery(std::function<bool(uint32_t refID)> f) { defaultRemoteDriven() = std::move(f); }

size_t RemoteApplier::tickPending() {
    if (moves_.size() == 0) return 0;
    const ExecFn exec = pickExec(exec_, consoleExec());
    if (!exec) return 0;
    EchoGuard echo(&hub_.echo());
    ApplyScope applyScope;
    return moves_.run([&](const MoveRef& m) { return applyMove(m, exec, false, nullptr) == ApplyStatus::Ok; });
}

size_t RemoteApplier::tickAll() {
    static ULONGLONG last = 0;
    ULONGLONG now = GetTickCount64();
    if (now - last < 1000) return 0;
    last = now;
    size_t n = 0;
    for (RemoteApplier* a : instances()) n += a->tickPending();
    return n;
}

ApplyStatus RemoteApplier::applyForce(const Effect& e) {
    EchoGuard echo(&hub_.echo());
    ApplyScope applyScope;
    ScriptGate gate;  // baseline never runs scripts, regardless of setUseGate (that switch is for the mutation twin only)
    err_.clear();
    auto form = [&](const FormRef& r) -> void* {
        uint32_t id = res_.fromRef(r);
        void* f = id ? res_.lookup(id) : nullptr;
        if (!f) err_ = "form missing: " + r.plugin;
        return f;
    };
    if (auto* x = std::get_if<SetStage>(&e)) {
        char* q = static_cast<char*>(form(x->quest));
        if (!q) return ApplyStatus::FormMissing;
        UInt8 cur = *reinterpret_cast<UInt8*>(q + 0x60);  // TESQuest::currentStage
        if (x->stage > cur) {
            // raising: let the engine do the native part (journal text, flags) with scripts gated off
            if (tc<bool, UInt8>(addr::SetStage.target, q, static_cast<UInt8>(x->stage))) return ApplyStatus::Ok;
            // The engine refuses stages that are not defined in the quest's stage list (or that fail its own checks).
            // Baseline wants the host's value to win: write the byte directly.
            *reinterpret_cast<UInt8*>(q + 0x60) = static_cast<UInt8>(x->stage);
            err_ = "engine SetStage returned false (stage not defined?); currentStage written directly";
            return ApplyStatus::Ok;
        }
        *reinterpret_cast<UInt8*>(q + 0x60) = static_cast<UInt8>(x->stage);  // lowering / equal: direct write
        return ApplyStatus::Ok;
    }
    if (auto* x = std::get_if<SetObjective>(&e)) {
        void* q = form(x->quest);
        if (!q) return ApplyStatus::FormMissing;
        char* obj = static_cast<char*>(tc<void*, UInt32>(addr::QuestGetObjective.target, q, x->objective));
        if (!obj) { err_ = "objective " + std::to_string(x->objective) + " not found in quest"; return ApplyStatus::EngineRejected; }
        UInt32* status = reinterpret_cast<UInt32*>(obj + 0x20);  // BGSQuestObjective::status
        UInt32 want = x->flags & 3u;
        if (*status != want) tc<void, UInt32>(addr::ObjectiveSetState.target, obj, want);
        if (*status != want) *status = want;  // SetState refuses some transitions (e.g. un-complete): force the byte
        return ApplyStatus::Ok;
    }
    return apply(e);  // nested EchoGuard/ApplyScope/ScriptGate are re-entrant
}

size_t RemoteApplier::applyTxn(const EffectTxn& t, PlayerId me) {
    size_t n = 0;
    localSlot_ = me;
    for (const Effect& e : t.effects) {
        // player-targeted effects honour Recipient; world effects (stage, refs, quests, globals) always apply
        const Recipient* r = nullptr;
        if (auto* a = std::get_if<AddItem>(&e)) r = &a->to;
        else if (auto* b = std::get_if<RemoveItem>(&e)) r = &b->to;
        else if (auto* c = std::get_if<ModActorValue>(&e)) r = &c->to;
        else if (auto* d = std::get_if<ModReputation>(&e)) r = &d->to;
        else if (auto* k = std::get_if<ModKarma>(&e)) r = &k->to;
        else if (auto* p = std::get_if<SetPerk>(&e)) r = &p->to;
        else if (auto* c2 = std::get_if<SetPlayerControls>(&e)) r = &c2->to;
        else if (auto* rc = std::get_if<RefCommand>(&e)) r = &rc->to;
        if (r && !forMe(*r, t.origin, me)) continue;
        if (apply(e) == ApplyStatus::Ok) ++n;
    }
    return n;
}

}  // namespace nvhooks
