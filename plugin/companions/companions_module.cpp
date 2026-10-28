// nvcoop module for companions, crime and kill rewards (x86, compiled into nvcoop.dll). NOTHING here has run in a game.
//
//   * the engine side of SocialSync: ISocialWorld (who is who: ownership through plugin/actors/engine/actor_engine_api.h, places and factions through
//     the xNVSE layouts, the player's teammate list) and ISocialEngine (console lines run on a calling reference, the verified condition evaluators
//     GetDetected / IsTrespassing, ExtraOwnership / ExtraContainerChanges / ExtraScript reads), every read behind SEH.
//   * the session frame: glueFrame() (called by plugin/coop/live once the live layer has a SocialService, see companions_glue.h) feeds SocialRuntime,
//     which owns the SocialSync and the 0x5C mux channel.
//   * the story effects the host's capture hub records (CompanionOwner) are queued by a sink and handed to SocialSync from the frame loop.
// Switches in nvcoop.ini [companions] (default OFF):  enabled=0 companions=1 crime=1 rewards=1 foreign=deny|allow xpmode=0|1 xpscope=0|1|2 xpradius=6000
//   karma=1 sharekarma=1 sharerep=1 friendlyfire=0 humanoid=1 critter=1 party=4 feedactors=1 dev=0.   docs/tools/companions-crime.md.
#include <Windows.h>

#include <chrono>
#include <cstdio>
#include <cstring>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <vector>

#include "GameExtraData.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "PluginAPI.h"
#include "actors/engine/actor_engine_api.h"
#include "actors/engine/engine_iface.h"
#include "apply_remote.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "common/registry.h"
#include "companion_sets_default.h"
#include "companions/companions_glue.h"
#include "companions/social_runtime.h"
#include "coop_runtime.h"
#include "engine_hooks.h"
#include "nvmp/ghost_ref.h"
#include "reload/reload_state.h"

namespace {

namespace eng = nvcoop::actors::engine;
namespace ns = nvcoop::social;
namespace sc = fnvcoop::social;
namespace cc = fnvcoop::combat;
using fnvcoop::FormRef;
using fnvcoop::PlayerId;
using fnvcoop::kHostPlayer;
using fnvcoop::kMaxSlots;
using fnvcoop::kNoPlayer;

// Engine facts used here (all from the engine's own tables or xNVSE headers; nothing invented):
//  * condition evaluators = the 10th field of the command table entry (plugin/hooks/tools/re/command_table.json -> CommandInfo in <reference exe>):
//      GetDetected   0x59D6E0  (calling ref = the observer, param = the actor it may see)   prologue 55 8B EC 83 EC 18
//      IsTrespassing 0x5A1900  (calling ref = the actor that may trespass)                  prologue 55 8B EC 51 8B 45 14
//    signature cdecl bool(thisObj, p1, p2, double* result), the same call plugin/hooks_world/world_query.cpp makes.
//  * console syntax (read from the same table): SetPlayerTeammate <int> (on the NPC), SetPCFactionAttack|SetPCFactionMurder|SetPCEnemyOfFaction <faction> <int>,
//    ClearFactionPlayerEnemyFlag <faction>, StartCombat <actor> (on the NPC), RewardXP <int>, RewardKarma <int>, MoveTo <ref>, AddItem / RemoveItem <item> <n>.
//  * `<ref>.SetAV Aggression 0|1|2` and the ref script variable `Waiting` (0|1) are what the game's own companion scripts use (out/questdata scripts).
//  * PlayerCharacter::teammates (tList<Actor>) at +0x5FC and the actor flag at +0x18D (xNVSE GameObjects.h / plugin/hooks/engine_hooks.cpp h_Teammate).
constexpr uint32_t kEvalGetDetected = 0x0059D6E0, kEvalIsTrespassing = 0x005A1900;
constexpr uint32_t kPlayerRef = 0x14;

struct Win32Memory {
    bool read(uint32_t a, void* out, size_t n) {
        __try {
            std::memcpy(out, reinterpret_cast<const void*>(a), n);
            return true;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
    }
} g_mem;

struct RawNode { void* data; RawNode* next; };   // tList node (xNVSE Utilities.h): data, next

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

std::string DllDir() {
    const std::string p = IniPath();
    const size_t s = p.find_last_of("\\/");
    return s == std::string::npos ? std::string() : p.substr(0, s + 1);
}

struct Settings {
    bool enabled = false, companions = true, crime = true, rewards = true, feedActors = true, dev = false;
    sc::SocialSettings host;   // what the host starts with (clients take the host's)
    bool shareRep = true;
    // G34 set pieces
    bool swapGuard = true, escort = true, partyOwned = true;
    uint32_t escortGraceMs = 4000, escortCooldownMs = 12000;
    float escortFar = 3500;
    sc::SlotRules sets;        // embedded companion_sets.txt, or the file next to the DLL
};
std::function<void(const std::vector<sc::Companion>&)> g_rosterListener;
fnvcoop::HostSession* g_host = nullptr;   // the running host session (set every frame by glueFrame); its isPartyCompanion is the rules engine's owner=party test
Settings g_set;
bool g_inited = false;
volatile PlayerId g_self = kNoPlayer;
uint64_t g_lines = 0, g_linesFailed = 0, g_evalCalls = 0, g_evalFaults = 0;

std::function<bool(uint32_t, uint8_t&)> g_slotOfRef;
std::function<uint32_t(uint8_t)> g_refOfSlot;

uint32_t RefOfSlot(uint8_t slot) {
    if (slot == g_self) return kPlayerRef;
    if (g_refOfSlot) return g_refOfSlot(slot);
    return slot != 0 ? nvmp::GhostRefOfPlayer(slot) : 0;
}
bool SlotOfRef(uint32_t ref, uint8_t& slot) {
    if (ref == kPlayerRef) { slot = g_self; return g_self != kNoPlayer; }
    if (g_slotOfRef) return g_slotOfRef(ref, slot);
    for (uint8_t s = 1; s < kMaxSlots; ++s)
        if (s != g_self && nvmp::GhostRefOfPlayer(s) == ref) { slot = s; return true; }
    return false;
}

void* LookupRef(uint32_t id) { return id ? nvhooks::resolver().lookup(id) : nullptr; }
uint32_t IdOfForm(const FormRef& f) { return f.plugin.empty() ? 0 : nvhooks::resolver().fromRef(f); }
bool FormOfId(uint32_t id, FormRef& out) { return id && nvhooks::resolver().toRef(id, &out); }

bool ExecOn(const std::string& line, uint32_t ref) {
    nvhooks::ApplyScope scope;   // our own console line must not be dropped by the guest gate, nor captured as a story effect
    ++g_lines;
    const bool ok = nvhooks::execConsoleLineOn(line, ref);
    if (!ok) ++g_linesFailed;
    return ok;
}
std::string Hex(uint32_t v) { char b[16]; std::snprintf(b, sizeof b, "%08X", v); return b; }

// ---------------------------------------------------------------------------------------------------------------- guarded reads (POD only)
struct Place {
    bool ok = false;
    float x = 0, y = 0, z = 0;
    uint32_t cell = 0, life = 0;
    void* base = nullptr;
};
bool ReadPlace(void* obj, Place* p) {
    __try {
        Actor* a = static_cast<Actor*>(obj);
        p->x = a->posX; p->y = a->posY; p->z = a->posZ;
        p->cell = a->parentCell ? a->parentCell->refID : 0;
        p->life = a->lifeState;
        p->base = a->baseForm;
        p->ok = true;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        p->ok = false;
        return false;
    }
}

// Position and cell of a plain TESObjectREFR (no Actor fields: a container, a bed, a door).
bool ReadRefPlace(void* obj, Place* p) {
    __try {
        TESObjectREFR* r = static_cast<TESObjectREFR*>(obj);
        p->x = r->posX; p->y = r->posY; p->z = r->posZ;
        p->cell = r->parentCell ? r->parentCell->refID : 0;
        p->ok = true;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        p->ok = false;
        return false;
    }
}

// The first faction on the NPC's base form. TODO-RE: the NPC's own "crime faction" field is not located; the first listed faction is the stand-in.
uint32_t FirstFactionId(void* baseForm, bool* critter) {
    __try {
        TESForm* f = static_cast<TESForm*>(baseForm);
        if (!f) return 0;
        if (critter) *critter = f->typeID == kFormType_TESCreature;
        if (f->typeID != kFormType_TESNPC && f->typeID != kFormType_TESCreature) return 0;
        TESActorBase* ab = static_cast<TESActorBase*>(f);
        RawNode* n = reinterpret_cast<RawNode*>(&ab->baseData.factionList);
        for (int i = 0; n && i < 32; ++i, n = n->next) {
            auto* d = static_cast<TESActorBaseData::FactionListData*>(n->data);
            if (d && d->faction && d->rank < 0x80) return d->faction->refID;
        }
        return 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

int ReadTeammateIds(uint32_t* out, int max) {
    void* player = LookupRef(kPlayerRef);
    if (!player) return 0;
    __try {
        RawNode* n = reinterpret_cast<RawNode*>(&reinterpret_cast<PlayerCharacter*>(player)->teammates);
        int c = 0;
        for (int i = 0; n && i < 64 && c < max; ++i, n = n->next)
            if (n->data) out[c++] = static_cast<TESForm*>(n->data)->refID;
        return c;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

struct InvItem { uint32_t id; int32_t n; };
// The inventory as the engine stores it for a non-player reference: ExtraContainerChanges entries, item -> countDelta (the change against the base container).
int ReadInventoryRaw(void* ref, InvItem* out, int max) {
    __try {
        auto* xc = static_cast<ExtraContainerChanges*>(static_cast<TESObjectREFR*>(ref)->extraDataList.GetByType(kExtraData_ContainerChanges));
        ExtraContainerChanges::EntryDataList* list = xc ? xc->GetEntryDataList() : nullptr;
        if (!list) return 0;
        RawNode* n = reinterpret_cast<RawNode*>(list);
        int c = 0;
        for (int i = 0; n && i < 2048 && c < max; ++i, n = n->next) {
            auto* e = static_cast<ExtraContainerChanges::EntryData*>(n->data);
            if (e && e->type) { out[c].id = e->type->refID; out[c].n = e->countDelta; ++c; }
        }
        return c;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}

// Owner of a reference, else of its cell: (form id, form type). false = not owned (or unreadable).
bool ReadOwner(void* ref, bool cellToo, uint32_t* ownerId, uint32_t* ownerType) {
    __try {
        TESObjectREFR* r = static_cast<TESObjectREFR*>(ref);
        auto* xo = static_cast<ExtraOwnership*>(r->extraDataList.GetByType(kExtraData_Ownership));
        if (!xo && cellToo && r->parentCell) xo = static_cast<ExtraOwnership*>(r->parentCell->extraDataList.GetByType(kExtraData_Ownership));
        if (!xo || !xo->owner) return false;
        *ownerId = xo->owner->refID;
        *ownerType = xo->owner->typeID;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool ReadLocked(void* ref) {
    __try {
        auto* lk = static_cast<ExtraLock*>(static_cast<TESObjectREFR*>(ref)->extraDataList.GetByType(kExtraData_Lock));
        return lk && lk->data && (lk->data->flags & 1) != 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
uint32_t ReadBaseType(void* ref, uint32_t* baseId) {
    __try {
        TESObjectREFR* r = static_cast<TESObjectREFR*>(ref);
        if (!r->baseForm) return 0;
        if (baseId) *baseId = r->baseForm->refID;
        return r->baseForm->typeID;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

// Writes the script variable `name` of the reference's own script (ExtraScript -> Script::varList names -> ScriptEventList). false = no such variable.
bool WriteRefScriptVar(void* ref, const char* name, double value) {
    __try {
        auto* xs = static_cast<ExtraScript*>(static_cast<TESObjectREFR*>(ref)->extraDataList.GetByType(kExtraData_Script));
        if (!xs || !xs->script || !xs->eventList || !xs->eventList->m_vars) return false;
        uint32_t idx = 0xFFFFFFFFu;
        RawNode* n = reinterpret_cast<RawNode*>(&xs->script->varList);
        for (int i = 0; n && i < 256; ++i, n = n->next) {
            auto* v = static_cast<VariableInfo*>(n->data);
            if (v && v->name.m_data && std::strcmp(v->name.m_data, name) == 0) { idx = v->idx; break; }
        }
        if (idx == 0xFFFFFFFFu) return false;
        RawNode* m = reinterpret_cast<RawNode*>(xs->eventList->m_vars);
        for (int i = 0; m && i < 512; ++i, m = m->next) {
            auto* sv = static_cast<ScriptLocal*>(m->data);
            if (sv && sv->id == idx) { sv->data = value; return true; }
        }
        return false;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// Reads the script variable `name` of the reference's own script (the companion scripts' `Waiting`). false = no such variable.
bool ReadRefScriptVar(void* ref, const char* name, double* out) {
    __try {
        auto* xs = static_cast<ExtraScript*>(static_cast<TESObjectREFR*>(ref)->extraDataList.GetByType(kExtraData_Script));
        if (!xs || !xs->script || !xs->eventList || !xs->eventList->m_vars) return false;
        uint32_t idx = 0xFFFFFFFFu;
        RawNode* n = reinterpret_cast<RawNode*>(&xs->script->varList);
        for (int i = 0; n && i < 256; ++i, n = n->next) {
            auto* v = static_cast<VariableInfo*>(n->data);
            if (v && v->name.m_data && std::strcmp(v->name.m_data, name) == 0) { idx = v->idx; break; }
        }
        if (idx == 0xFFFFFFFFu) return false;
        RawNode* m = reinterpret_cast<RawNode*>(xs->eventList->m_vars);
        for (int i = 0; m && i < 512; ++i, m = m->next) {
            auto* sv = static_cast<ScriptLocal*>(m->data);
            if (sv && sv->id == idx) { *out = sv->data; return true; }
        }
        return false;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

using CondFn = bool(__cdecl*)(void* thisObj, void* p1, void* p2, double* result);
// Calls an engine condition evaluator. Refused when the entry no longer has its original prologue (somebody detoured it).
bool CallCondition(uint32_t addr, void* thisObj, void* p1, double* out) {
    unsigned char pro[3] = {};
    if (!g_mem.read(addr, pro, 3) || pro[0] != 0x55 || pro[1] != 0x8B || pro[2] != 0xEC) return false;
    ++g_evalCalls;
    __try {
        double r = 0;
        const bool ok = reinterpret_cast<CondFn>(addr)(thisObj, p1, nullptr, &r);
        *out = r;
        return ok;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        ++g_evalFaults;
        return false;
    }
}

// ---------------------------------------------------------------------------------------------------------------- ISocialWorld
class LiveWorld : public ns::ISocialWorld {
public:
    bool viewOf(uint32_t ref, ns::ActorView& v) const {
        v = ns::ActorView{};
        const eng::OwnerRef o = eng::ownerOf(ref);
        if (!o.known) return false;
        FormRef f;
        if (!FormOfId(ref, f)) return false;   // a created (0xFF) reference has no wire id the book could use
        void* obj = LookupRef(ref);
        v.known = true; v.npc = f; v.netId = static_cast<uint32_t>(o.netId); v.ref = obj ? ref : 0;
        v.owner = o.ownerSlot; v.simulatedHere = o.mine && !o.pending && !o.standIn; v.standIn = o.standIn; v.alive = !o.dead;
        Place p;
        if (obj && ReadPlace(obj, &p)) {
            v.cell = p.cell; v.pos = {p.x, p.y, p.z};
            v.alive = v.alive && !(p.life == 1 || p.life == 2);
            double w = 0;
            v.waiting = ReadRefScriptVar(obj, "Waiting", &w) && w != 0;   // the companion script's own wait flag (set by the wheel, the owner's wait command and the story)
            bool critter = false;
            const uint32_t fid = FirstFactionId(p.base, &critter);
            v.critter = critter;
            FormRef fr;
            if (fid && FormOfId(fid, fr)) v.faction = fr;
        }
        return true;
    }
    bool actor(const FormRef& npc, ns::ActorView& out) const override {
        const uint32_t ref = IdOfForm(npc);
        return ref && viewOf(ref, out);
    }
    bool actorByNet(uint32_t netId, ns::ActorView& out) const override {
        const uint32_t ref = eng::refOf(netId);
        if (ref && viewOf(ref, out)) return true;
        const eng::OwnerRef o = eng::ownerOfNet(netId);   // not loaded here: the owner is still known
        if (!o.known) return false;
        out = ns::ActorView{};
        out.known = true; out.netId = netId; out.owner = o.ownerSlot; out.standIn = o.standIn; out.alive = !o.dead;
        return true;
    }
    std::vector<sc::PlayerPlace> players() const override {
        std::vector<sc::PlayerPlace> v;
        for (uint8_t s = 0; s < kMaxSlots; ++s) {
            const uint32_t ref = RefOfSlot(s);
            void* obj = LookupRef(ref);
            Place p;
            if (!obj || !ReadPlace(obj, &p)) continue;
            sc::PlayerPlace pp;
            pp.slot = s; pp.connected = true; pp.cell = p.cell; pp.pos = {p.x, p.y, p.z};
            v.push_back(pp);
        }
        return v;
    }
    uint32_t refOfPlayer(PlayerId slot) const override { return RefOfSlot(slot); }
    std::vector<FormRef> teammatesHere() const override {
        uint32_t ids[64];
        const int n = ReadTeammateIds(ids, 64);
        std::vector<FormRef> v;
        for (int i = 0; i < n; ++i) {
            FormRef f;
            if (FormOfId(ids[i], f)) v.push_back(f);
        }
        return v;
    }
    std::vector<FormRef> factionActorsHere(const FormRef& faction, uint32_t cell, const cc::Vec3& where, float radius) const override {
        std::vector<FormRef> out;
        eng::IActorEngine* ae = eng::actorEngine();
        if (!ae) return out;
        std::vector<eng::RefId> refs;
        ae->loadedActors(refs, 400);
        const uint32_t fid = IdOfForm(faction);
        if (!fid) return out;
        for (eng::RefId r : refs) {
            const eng::OwnerRef o = eng::ownerOf(r);
            if (!o.known || !o.mine || o.pending || o.standIn || o.dead) continue;
            void* obj = LookupRef(r);
            Place p;
            if (!obj || !ReadPlace(obj, &p) || p.cell != cell || p.life == 1 || p.life == 2) continue;
            if (FirstFactionId(p.base, nullptr) != fid) continue;
            const cc::Vec3 pos = {p.x, p.y, p.z};
            if (cc::distance(pos, where) > radius) continue;
            FormRef f;
            if (FormOfId(r, f)) out.push_back(f);
        }
        return out;
    }
};
LiveWorld g_world;

// ---------------------------------------------------------------------------------------------------------------- ISocialEngine
nvhooks::RemoteApplier* Applier() {
    static std::unique_ptr<nvhooks::RemoteApplier> a;
    if (!a && nvhooks::hub()) a.reset(new nvhooks::RemoteApplier(*nvhooks::hub(), nvhooks::resolver()));
    return a.get();
}

class LiveEngine : public ns::ISocialEngine {
public:
    // ---- companions
    bool setTeammate(const FormRef& npc, bool on) override {
        const uint32_t ref = IdOfForm(npc);
        return ref && ExecOn(on ? "SetPlayerTeammate 1" : "SetPlayerTeammate 0", ref);
    }
    // Stance = the Aggression actor value (0|1|2); mode = the companion script's own `Waiting` variable. Range has no engine mapping yet (TODO-RE).
    bool applyCompanionState(const FormRef& npc, const sc::CompanionState& s) override {
        const uint32_t ref = IdOfForm(npc);
        void* obj = LookupRef(ref);
        if (!ref || !obj) return false;
        bool any = ExecOn("SetAV Aggression " + std::to_string(static_cast<int>(s.stance)), ref);
        any = WriteRefScriptVar(obj, "Waiting", s.mode == sc::FollowMode::Wait ? 1.0 : 0.0) || any;
        return any;
    }
    bool summonCompanion(const FormRef& npc, PlayerId owner) override {
        const uint32_t ref = IdOfForm(npc);
        if (!ref || owner != g_self) return false;
        return ExecOn("MoveTo player", ref);
    }
    bool readInventory(const FormRef& npc, sc::ItemCounts& out) override {
        out.clear();
        void* obj = LookupRef(IdOfForm(npc));
        if (!obj) return false;
        static InvItem buf[512];
        const int n = ReadInventoryRaw(obj, buf, 512);
        if (n < 0) return false;
        for (int i = 0; i < n; ++i) {
            FormRef f;
            if (buf[i].n != 0 && FormOfId(buf[i].id, f)) out[f] += buf[i].n;
        }
        return true;
    }
    bool applyInventoryDelta(const FormRef& npc, const FormRef& item, int32_t delta) override {
        const uint32_t ref = IdOfForm(npc), it = IdOfForm(item);
        if (!ref || !it || delta == 0) return false;
        char line[64];
        std::snprintf(line, sizeof line, "%s %08X %d", delta > 0 ? "AddItem" : "RemoveItem", it, delta > 0 ? delta : -delta);
        return ExecOn(line, ref);
    }
    // ---- crime
    bool applyStanding(const FormRef& faction, const sc::FactionStanding& st) override {
        const uint32_t f = IdOfForm(faction);
        if (!f) return false;
        const std::string h = Hex(f);
        bool ok = ExecOn("SetPCFactionAttack " + h + (st.attack ? " 1" : " 0"), 0);
        ok = ExecOn("SetPCFactionMurder " + h + (st.murder ? " 1" : " 0"), 0) && ok;
        ok = ExecOn("SetPCEnemyOfFaction " + h + (st.enemy ? " 1" : " 0"), 0) && ok;
        if (st.clean()) ok = ExecOn("ClearFactionPlayerEnemyFlag " + h, 0) && ok;
        return ok;
    }
    bool applyHostility(const cc::HostilityEffect& e) override {
        const uint32_t a = IdOfForm(e.factionA), b = IdOfForm(e.factionB);
        if (e.kind != cc::HostilityKind::StopCombat && (!a || !b)) return false;
        const std::string line = cc::consoleLineFor(e, Hex(a), Hex(b));
        return !line.empty() && ExecOn(line, 0);
    }
    bool alertActor(const FormRef& npc, uint32_t criminalRef) override {
        const uint32_t ref = IdOfForm(npc);
        return ref && criminalRef && ExecOn("StartCombat " + Hex(criminalRef), ref);
    }
    void observeDetections(std::vector<sc::Detection>& out) override {
        out.clear();
        eng::IActorEngine* ae = eng::actorEngine();
        if (!ae) return;
        std::vector<eng::RefId> refs;
        ae->loadedActors(refs, 400);
        // players this machine can see as references (the local player and the puppets)
        std::vector<std::pair<uint8_t, void*>> targets;
        for (uint8_t s = 0; s < kMaxSlots; ++s)
            if (void* t = LookupRef(RefOfSlot(s))) targets.push_back({s, t});
        if (targets.empty() || refs.empty()) return;
        constexpr size_t kBudget = 32;   // condition calls per poll; the start rotates so every NPC is served
        size_t budget = kBudget;
        if (start_ >= refs.size()) start_ = 0;
        for (size_t k = 0; k < refs.size() && budget > 0; ++k) {
            const eng::RefId r = refs[(start_ + k) % refs.size()];
            const eng::OwnerRef o = eng::ownerOf(r);
            if (!o.known || !o.mine || o.pending || o.standIn || o.dead) continue;
            void* obj = LookupRef(r);
            Place p;
            if (!obj || !ReadPlace(obj, &p) || p.life == 1 || p.life == 2) continue;
            const uint32_t fid = FirstFactionId(p.base, nullptr);
            FormRef ff;
            if (!fid || !FormOfId(fid, ff)) continue;
            for (auto& t : targets) {
                if (budget == 0) break;
                --budget;
                double v = 0;
                if (CallCondition(kEvalGetDetected, obj, t.second, &v) && v > 0.5) {
                    sc::Detection d;
                    d.observer = cc::Entity::actor(static_cast<uint32_t>(o.netId)); d.faction = ff; d.target = t.first; d.cell = p.cell; d.seen = true;
                    out.push_back(d);
                }
            }
        }
        start_ = (start_ + 1) % (refs.empty() ? 1 : refs.size());
    }
    bool localTrespass(sc::InteractionFacts& f, uint32_t& cell, cc::Vec3& where) override {
        void* player = LookupRef(kPlayerRef);
        Place p;
        double v = 0;
        if (!player || !ReadPlace(player, &p) || !CallCondition(kEvalIsTrespassing, player, nullptr, &v) || v < 0.5) return false;
        uint32_t ownerId = 0, ownerType = 0;
        if (!ReadOwnerOfCell(player, &ownerId, &ownerType)) return false;
        FormRef faction;
        if (!FactionOfOwner(ownerId, ownerType, faction)) return false;
        f = sc::InteractionFacts{};
        f.what = sc::Interaction::EnterOwnedCell;
        f.own.owned = true; f.own.crimeFaction = faction;
        FormRef ownerRef;
        if (FormOfId(ownerId, ownerRef)) f.own.ownerForm = ownerRef;
        cell = p.cell;
        where = {p.x, p.y, p.z};
        return true;
    }
    bool readInteraction(const FormRef& target, bool pickup, sc::InteractionFacts& f, uint32_t& cell, cc::Vec3& where) override {
        const uint32_t ref = IdOfForm(target);
        void* obj = LookupRef(ref);
        Place place;
        if (!obj) return false;
        uint32_t baseId = 0;
        const uint32_t baseType = ReadBaseType(obj, &baseId);
        f = sc::InteractionFacts{};
        if (pickup) f.what = sc::Interaction::TakeItem;
        else if (baseType == kFormType_TESObjectCONT) f.what = sc::Interaction::OpenContainer;
        else if (baseType == kFormType_TESFurniture) f.what = sc::Interaction::UseBed;   // owned furniture: sleeping / sitting in somebody's place
        else return false;
        f.locked = ReadLocked(obj);
        uint32_t ownerId = 0, ownerType = 0;
        if (!ReadOwner(obj, true, &ownerId, &ownerType)) return false;   // nobody's
        FormRef faction;
        if (!FactionOfOwner(ownerId, ownerType, faction)) return false;
        f.own.owned = true; f.own.crimeFaction = faction;
        FormRef ownerRef;
        if (FormOfId(ownerId, ownerRef)) f.own.ownerForm = ownerRef;
        ReadRefPlace(obj, &place);
        cell = place.cell;
        where = {place.x, place.y, place.z};
        return true;
    }
    // ---- rewards (the local player)
    bool awardXp(int32_t xp) override { return xp > 0 && ExecOn("RewardXP " + std::to_string(xp), kPlayerRef); }
    bool rewardKarma(int32_t delta) override { return delta != 0 && ExecOn("RewardKarma " + std::to_string(delta), kPlayerRef); }
    bool modReputation(const FormRef& faction, int fame, int infamy) override {
        nvhooks::RemoteApplier* a = Applier();
        if (!a) return false;
        fnvcoop::ModReputation m;
        m.faction = faction; m.fame = fame; m.infamy = infamy; m.to = fnvcoop::Recipient::all();
        return a->apply(m) == nvhooks::ApplyStatus::Ok;
    }

private:
    static bool ReadOwnerOfCell(void* player, uint32_t* id, uint32_t* type) {
        __try {
            TESObjectREFR* r = static_cast<TESObjectREFR*>(player);
            if (!r->parentCell) return false;
            auto* xo = static_cast<ExtraOwnership*>(r->parentCell->extraDataList.GetByType(kExtraData_Ownership));
            if (!xo || !xo->owner) return false;
            *id = xo->owner->refID;
            *type = xo->owner->typeID;
            return true;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
    }
    // An owner is a faction (its own id) or an NPC (its first faction).
    static bool FactionOfOwner(uint32_t ownerId, uint32_t ownerType, FormRef& out) {
        uint32_t fid = 0;
        if (ownerType == kFormType_TESFaction) fid = ownerId;
        else if (void* base = LookupRef(ownerId)) fid = FirstFactionId(base, nullptr);
        return fid && FormOfId(fid, out);
    }
    size_t start_ = 0;
};
LiveEngine g_engine;

// ---------------------------------------------------------------------------------------------------------------- session
std::unique_ptr<ns::SocialRuntime> g_rt;
std::vector<std::vector<fnvcoop::Effect>> g_pendingTxns;   // each captured transaction's CompanionOwner effects, kept together (the swap guard needs the whole transaction)
bool g_sinkInstalled = false;

void InstallEffectSink() {
    if (g_sinkInstalled) return;
    nvhooks::CaptureHub* hub = nvhooks::hub();
    if (!hub) return;
    hub->addSink([](fnvcoop::EffectTxn&& t) {
        if (!g_inited) return;
        std::vector<fnvcoop::Effect> mine;
        for (const fnvcoop::Effect& e : t.effects)
            if (std::holds_alternative<fnvcoop::CompanionOwner>(e)) mine.push_back(e);
        if (!mine.empty() && g_pendingTxns.size() < 512) g_pendingTxns.push_back(std::move(mine));
    });
    g_sinkInstalled = true;
}

void ReadSettings() {
    const std::string ini = IniPath();
    auto geti = [&](const char* k, int d) { return static_cast<int>(GetPrivateProfileIntA("companions", k, d, ini.c_str())); };
    char buf[32] = {};
    g_set.enabled = geti("enabled", 0) != 0;
    g_set.companions = geti("companions", 1) != 0;
    g_set.crime = geti("crime", 1) != 0;
    g_set.rewards = geti("rewards", 1) != 0;
    g_set.feedActors = geti("feedactors", 1) != 0;
    g_set.dev = geti("dev", 0) != 0;
    sc::SocialSettings& s = g_set.host;
    s.xpMode = geti("xpmode", 0) == 1 ? cc::XpMode::PartyShare : cc::XpMode::KillingBlow;
    const int scope = geti("xpscope", 0);
    s.scope = scope == 1 ? sc::XpScope::SameCell : scope == 2 ? sc::XpScope::Near : sc::XpScope::AllConnected;
    s.nearRadius = static_cast<float>(geti("xpradius", 6000));
    s.karmaOnKills = geti("karma", 1) != 0;
    s.shareKarma = geti("sharekarma", 1) != 0;
    s.crime = g_set.crime;
    s.friendlyFire = geti("friendlyfire", 0) != 0;
    GetPrivateProfileStringA("companions", "foreign", "deny", buf, sizeof buf, ini.c_str());
    s.foreignCompanions = (buf[0] == 'a' || buf[0] == 'A') ? sc::ForeignPolicy::Allow : sc::ForeignPolicy::Deny;
    s.caps.humanoidPerOwner = static_cast<uint8_t>(geti("humanoid", 1));
    s.caps.critterPerOwner = static_cast<uint8_t>(geti("critter", 1));
    s.caps.partyTotal = static_cast<uint8_t>(geti("party", 4));
    g_set.shareRep = geti("sharerep", 1) != 0;
    g_set.swapGuard = geti("swapguard", 1) != 0;                // a hire that fires another player's companion is put back; a second Dead Money / Honest Hearts companion is turned back
    g_set.escort = geti("escort", 1) != 0;                      // my companions are brought along when a script moved my player
    g_set.partyOwned = geti("partyowned", 1) != 0;              // the rules engine's owner=party companions (ED-E in Lonesome Road) belong to the whole party
    g_set.escortGraceMs = static_cast<uint32_t>(geti("escortgrace", 4000));
    g_set.escortCooldownMs = static_cast<uint32_t>(geti("escortcooldown", 12000));
    g_set.escortFar = static_cast<float>(geti("escortfar", 3500));
    // data: the embedded table, replaced by nvcoop_companion_sets.txt next to the DLL when it exists
    g_set.sets = sc::SlotRules();
    std::string text = ns::kDefaultCompanionSets;
    {
        std::FILE* f = std::fopen((DllDir() + "nvcoop_companion_sets.txt").c_str(), "rb");
        if (f) {
            std::string t;
            char b[4096];
            size_t n;
            while ((n = std::fread(b, 1, sizeof b, f)) > 0) t.append(b, n);
            std::fclose(f);
            if (!t.empty()) text = t;
        }
    }
    const sc::SlotRules::LoadStats st = g_set.sets.load(text);
    if (st.bad) nvcoop::Log("companions: companion_sets: %zu line(s) did not parse", st.bad);
}

bool ParseForm(const std::string& text, FormRef& out) {
    const size_t c = text.find(':');
    if (c == std::string::npos || c == 0 || c + 1 >= text.size()) return false;
    char* end = nullptr;
    const unsigned long v = std::strtoul(text.c_str() + c + 1, &end, 16);
    if (!end || *end != 0) return false;
    out = FormRef::make(text.substr(0, c), static_cast<uint32_t>(v));
    return true;
}

bool DevMethod(const std::string& params, std::string& out) {
    using nvcoop::json::Value;
    ns::SocialSync* sync = g_rt ? g_rt->sync() : nullptr;
    if (!sync) { out = "no running social session"; return false; }
    Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad json: " + err; return false; }
    const std::string op = p.get("op").asString();
    FormRef f;
    if (op == "crime") {
        sc::CrimeEvent e;
        if (!ParseForm(p.get("faction").asString(), f)) { out = "faction must be \"Plugin.esm:HEX\""; return false; }
        e.kind = static_cast<sc::CrimeKind>(p.get("kind").asInt(0));
        e.faction = f;
        e.criminal = static_cast<PlayerId>(p.get("criminal").asInt(g_self));
        e.cell = static_cast<uint32_t>(p.get("cell").asInt(0));
        e.victimDead = p.get("dead").asBool(false);
        if (p.find("victimNet")) e.victim = cc::Entity::actor(static_cast<uint32_t>(p.get("victimNet").asInt(0)));
        e.nativeMask = static_cast<uint8_t>(p.get("native").asInt(0));
        sync->reportCrime(e);
    } else if (op == "kill") {
        sc::KillReport r;
        r.deathId = static_cast<uint64_t>(p.get("id").asInt(1));
        r.victim.who = cc::Entity::actor(static_cast<uint32_t>(p.get("victimNet").asInt(1)));
        r.victim.xp = static_cast<int32_t>(p.get("xp").asInt(0));
        r.victim.karma = static_cast<sc::KarmaClass>(p.get("karma").asInt(0));
        r.victim.hostileToParty = p.get("hostile").asBool(false);
        r.killer = cc::Entity::player(static_cast<PlayerId>(p.get("killer").asInt(g_self)));
        if (p.find("recruiter")) { r.killer = cc::Entity::actor(static_cast<uint32_t>(p.get("killerNet").asInt(1))); r.killerRecruiter = static_cast<PlayerId>(p.get("recruiter").asInt(0)); }
        r.nativeMask = static_cast<uint8_t>(p.get("native").asInt(0));
        r.cell = static_cast<uint32_t>(p.get("cell").asInt(0));
        sync->reportKill(r);
    } else if (op == "forgive") {
        if (!ParseForm(p.get("faction").asString(), f)) { out = "faction must be \"Plugin.esm:HEX\""; return false; }
        sync->forgive(f);
    } else if (op == "cmd") {
        if (!ParseForm(p.get("npc").asString(), f)) { out = "npc must be \"Plugin.esm:HEX\""; return false; }
        if (!sync->issueCommand(static_cast<sc::CmdKind>(p.get("kind").asInt(1)), f, static_cast<uint8_t>(p.get("arg").asInt(0)))) { out = "refused"; return false; }
    } else if (op == "interact") {
        if (!ParseForm(p.get("target").asString(), f)) { out = "target must be \"Plugin.esm:HEX\""; return false; }
        sync->onGuestInteraction(static_cast<PlayerId>(p.get("slot").asInt(1)), p.get("pickup").asBool(true), f);
    } else if (op == "settings") {
        sc::SocialSettings s = sync->settings();
        if (p.find("xpmode")) s.xpMode = p.get("xpmode").asInt(0) == 1 ? cc::XpMode::PartyShare : cc::XpMode::KillingBlow;
        if (p.find("scope")) s.scope = static_cast<sc::XpScope>(p.get("scope").asInt(0));
        if (p.find("friendly")) s.friendlyFire = p.get("friendly").asBool(false);
        if (p.find("foreign")) s.foreignCompanions = p.get("foreign").asInt(0) ? sc::ForeignPolicy::Allow : sc::ForeignPolicy::Deny;
        if (p.find("crime")) s.crime = p.get("crime").asBool(true);
        sync->setSettings(s);
    } else {
        out = "unknown op (crime kill forgive cmd interact settings)";
        return false;
    }
    out = "{\"ok\":true}";
    return true;
}

void Init() {
    if (g_inited) return;
    ReadSettings();
    if (!g_set.enabled) { nvcoop::Log("companions: disabled (nvcoop.ini [companions] enabled=0)"); return; }
    g_inited = true;
    nvcoop::RegisterMethodClosure("social.status", [](const std::string&, std::string& out) {
        out = ns::glueStatusJson();
        return true;
    }, true, 0);
    if (g_set.dev) nvcoop::RegisterMethodClosure("social.dev", DevMethod, true, 0);
    InstallEffectSink();
    nvcoop::Log("companions: enabled companions=%d crime=%d rewards=%d xpmode=%d feedactors=%d dev=%d (waits for the live layer's glueFrame)", g_set.companions ? 1 : 0,
                g_set.crime ? 1 : 0, g_set.rewards ? 1 : 0, g_set.host.xpMode == cc::XpMode::PartyShare ? 1 : 0, g_set.feedActors ? 1 : 0, g_set.dev ? 1 : 0);
}

void Uninit() {
    g_rt.reset();
    g_pendingTxns.clear();
    g_inited = false;   // (a capture sink cannot be removed from the hub: it checks g_inited)
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_DeferredInit) Init();
    else if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) Uninit();
}

void OnFrame() {
    if (!g_inited) return;
    InstallEffectSink();
    if (!g_pendingTxns.empty() && g_rt && g_rt->sync()) {
        std::vector<std::vector<fnvcoop::Effect>> txns;
        txns.swap(g_pendingTxns);
        for (const auto& fx : txns) g_rt->sync()->onTxn(fx);   // (a client ignores them: the roster message carries the same facts)
    }
}

}  // namespace

NVCOOP_MODULE(companions_crime, "companions_crime", nullptr, OnMessage, OnFrame);

NVCOOP_RELOAD_STATE(companions_crime, "companions_crime", 1, 46,
    /*canUnload*/ nullptr,
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

namespace nvcoop::social {

void setPlayerRefMap(std::function<bool(uint32_t, uint8_t&)> slotOfRef, std::function<uint32_t(uint8_t)> refOfSlot) {
    g_slotOfRef = std::move(slotOfRef);
    g_refOfSlot = std::move(refOfSlot);
}

bool glueEnabled() { return g_inited && g_set.enabled; }
bool glueRunning() { return g_rt && g_rt->sync(); }
bool glueXpHandled() { return glueRunning() && g_set.rewards; }

bool glueAllowActivation(uint8_t slot, const fnvcoop::FormRef& npc) {
    SocialSync* s = g_rt ? g_rt->sync() : nullptr;
    return !s || s->allowActivation(slot, npc);
}

sc::HitPolicy glueHitPolicy(uint32_t attackerRef, uint32_t victimRef) {
    SocialSync* s = g_rt ? g_rt->sync() : nullptr;
    if (!s) return sc::HitPolicy::Allow;
    auto entityOf = [](uint32_t ref, cc::Entity& e) {
        uint8_t slot = 0;
        if (SlotOfRef(ref, slot)) { e = cc::Entity::player(slot); return true; }
        const uint32_t net = static_cast<uint32_t>(eng::netIdOf(ref));
        if (!net) return false;
        e = cc::Entity::actor(net);
        return true;
    };
    cc::Entity a, v;
    if (!entityOf(attackerRef, a) || !entityOf(victimRef, v)) return sc::HitPolicy::Allow;
    return s->hitPolicy(a, v);
}

bool glueVatsTargetAllowed(uint32_t targetRef) {
    SocialSync* s = g_rt ? g_rt->sync() : nullptr;
    if (!s) return true;
    uint8_t slot = 0;
    cc::Entity e;
    if (SlotOfRef(targetRef, slot)) e = cc::Entity::player(slot);
    else {
        const uint32_t net = static_cast<uint32_t>(eng::netIdOf(targetRef));
        if (!net) return true;
        e = cc::Entity::actor(net);
    }
    return s->vatsTargetAllowed(e);
}

void glueSetRosterListener(std::function<void(const std::vector<sc::Companion>&)> onRoster) { g_rosterListener = std::move(onRoster); }

bool glueCurrentFollower(uint8_t slot, const std::string& group, fnvcoop::FormRef& out) {
    SocialSync* s = g_rt ? g_rt->sync() : nullptr;
    if (!s) return false;
    out = s->currentFollower(slot, group);
    return out.localId != 0;
}

std::vector<std::string> glueViewPartyScripts() {
    if (!g_inited || !g_set.enabled) return {};
    return std::vector<std::string>(g_set.sets.viewPartyNames().begin(), g_set.sets.viewPartyNames().end());
}

uint8_t glueViewSlot(const std::string& scriptEdid, uint8_t actingSlot) {
    if (!g_inited || !g_set.enabled) return actingSlot;
    return sc::viewSlotFor(g_set.sets, scriptEdid, actingSlot);
}

void glueReportCrime(const sc::CrimeEvent& e) { if (SocialSync* s = g_rt ? g_rt->sync() : nullptr) s->reportCrime(e); }
void glueReportKill(const sc::KillReport& r) { if (SocialSync* s = g_rt ? g_rt->sync() : nullptr) s->reportKill(r); }
void glueGuestInteraction(uint8_t slot, bool pickup, const fnvcoop::FormRef& target) { if (SocialSync* s = g_rt ? g_rt->sync() : nullptr) s->onGuestInteraction(slot, pickup, target); }

void glueFrame(nvcoop::coop::Runtime* rt) {
    if (!g_inited || !g_set.enabled) return;
    FrameInfo f;
    if (rt && rt->running()) {
        f.active = true;
        f.mux = rt->mux();
        f.nowMs = NowMs();
        if (auto* host = rt->host()) {
            f.isHost = true;
            f.self = kHostPlayer;
            for (const auto& cv : host->clients()) {
                ClientLink l;
                l.slot = cv.slot; l.peer = cv.peer; l.joined = cv.joined;
                f.clients.push_back(l);
            }
            f.commit = [host](const std::vector<fnvcoop::Effect>& fx, const char* what) -> uint64_t {
                fnvcoop::Cause c;
                c.kind = (what && std::strncmp(what, "crime", 5) == 0) || (what && std::strcmp(what, "kill") == 0) ? fnvcoop::CauseKind::Combat : fnvcoop::CauseKind::Script;
                return host->commit(kHostPlayer, c, fx);
            };
        } else if (auto* cl = rt->client()) {
            f.isHost = false;
            f.self = cl->joined() ? cl->slot() : kNoPlayer;
            f.hostLinkUp = cl->joined() && !cl->baselineActive();
        } else {
            f.active = false;
        }
    }
    g_self = f.active ? f.self : kNoPlayer;
    g_host = f.active && rt ? rt->host() : nullptr;
    if (g_host && !g_host->companionVet)   // W3a item 6: the commit asks the companion book first (swap dismissals and slot overruns never reach the wire)
        g_host->companionVet = [](fnvcoop::EffectTxn& t) -> size_t { return g_rt && g_rt->sync() ? g_rt->sync()->vetOutgoing(t.effects) : 0; };
    if (!g_rt) {
        if (!f.active) return;
        g_rt.reset(new ns::SocialRuntime(g_world, g_engine));
        ns::Config& c = g_rt->config();
        c.settings = g_set.host;
        c.companions = g_set.companions; c.crime = g_set.crime; c.rewards = g_set.rewards;
        c.shareReputation = g_set.shareRep;
        c.sets = g_set.sets;
        c.swapGuard = g_set.swapGuard;
        c.escort = g_set.escort;
        c.escortRules.graceMs = g_set.escortGraceMs; c.escortRules.cooldownMs = g_set.escortCooldownMs; c.escortRules.farDistance = g_set.escortFar;
        c.onRoster = [](const std::vector<sc::Companion>& all) { if (g_rosterListener) g_rosterListener(all); };
        if (g_set.partyOwned) c.isPartyCompanion = [](const FormRef& n) { return g_host && g_host->isPartyCompanion && g_host->isPartyCompanion(n); };
        if (g_set.feedActors) c.feedActors = [](const fnvcoop::Effect& e) { eng::onEffect(e); };
    }
    g_rt->frame(f);
}

std::string glueStatusJson() {
    char head[256];
    std::snprintf(head, sizeof head, "{\"enabled\":%s,\"running\":%s,\"engine\":{\"lines\":%llu,\"linesFailed\":%llu,\"evalCalls\":%llu,\"evalFaults\":%llu},\"sync\":",
                  g_set.enabled ? "true" : "false", glueRunning() ? "true" : "false", static_cast<unsigned long long>(g_lines),
                  static_cast<unsigned long long>(g_linesFailed), static_cast<unsigned long long>(g_evalCalls), static_cast<unsigned long long>(g_evalFaults));
    const SocialSync* s = g_rt ? g_rt->sync() : nullptr;
    return std::string(head) + (s ? s->statusJson() : std::string("null")) + "}";
}

}  // namespace nvcoop::social
