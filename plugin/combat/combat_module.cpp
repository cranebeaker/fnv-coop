// nvcoop module for combat sync (x86, compiled into nvcoop.dll next to the coop module). NOTHING here has run in a game.
//
//   * the HIT GATE: the engine's hit funnel (ApplyHitData, 0x89A760) is entered from six call sites; they are redirected to HitGateThunk, which
//     asks CombatSync what to do with the hit BEFORE the engine applies it (Proceed / Clamp / Neutralize) and edits the hit data accordingly.
//   * the engine side of CombatSync: IWorldView (ownership via plugin/actors/engine/actor_engine_api.h, positions / health through the xNVSE
//     layouts) and ICombatEngine (the owner applies an accepted hit by replaying it through the same funnel with a fresh ActorHitData; if that is
//     off or faults, console lines run with the victim as calling reference).
//   * the session frame: glueFrame() (called by coop_module.cpp) feeds CombatRuntime, which owns the CombatSync and the 0xCB mux channel.
// Switches in nvcoop.ini [combat]: enabled=1 gate=1 native=1 friendlyfire=0 crime=0 xpmode=0. docs/tools/combat-hooks.md.
#include "coop_runtime.h"   // Runtime::active()->protectedHit (G40)
#include <Windows.h>

#include <chrono>
#include <cstdio>
#include <cstring>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <vector>

#include "GameForms.h"
#include "GameObjects.h"
#include "PluginAPI.h"
#include "actors/engine/actor_engine_api.h"
#include "combat/call_patch.h"
#include "combat/combat_addrs.h"
#include "combat/combat_glue.h"
#include "combat/combat_runtime.h"
#include "combat/hit_edit.h"
#include "common/nvcoop_api.h"
#include "common/registry.h"
#include "coop_runtime.h"
#include "engine_hooks.h"
#include "hook_patch.h"
#include "nvmp/ghost_ref.h"
#include "reload/reload_state.h"
#ifdef NVCOOP_WITH_COMPANIONS
#include "companions/companions_glue.h"   // kill reports, crime reports and the party hit policy ([companions] enabled, default off)
#endif

namespace {

namespace eng = nvcoop::actors::engine;
namespace cc = fnvcoop::combat;
namespace addr = nvcoop::combat::addr;
using nvcoop::combat::CallSitePatches;
using nvcoop::combat::ClientLink;
using nvcoop::combat::CombatRuntime;
using nvcoop::combat::CombatSync;
using nvcoop::combat::EngineHit;
using nvcoop::combat::EntityView;
using nvcoop::combat::FrameInfo;
using nvcoop::combat::GateAction;
using nvcoop::combat::GateDecision;
using nvcoop::combat::ICombatEngine;
using nvcoop::combat::IWorldView;
using nvcoop::combat::SiteResult;
using nvcoop::combat::Stats;
using nvcoop::combat::glueStatusJson;
using fnvcoop::PlayerId;
using fnvcoop::kHostPlayer;
using fnvcoop::kMaxSlots;
using fnvcoop::kNoPlayer;

// ---------------------------------------------------------------------------------------------------------------- raw memory
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
    nvhooks::Addr allocExec(size_t) override { return 0; }
    void freeExec(nvhooks::Addr, size_t) override {}
};
Win32Memory g_mem;

// One small function per read so __try never meets a C++ object.
bool Rd32(const void* base, uint32_t off, uint32_t* out) {
    __try {
        *out = *reinterpret_cast<const uint32_t*>(reinterpret_cast<const char*>(base) + off);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool RdF(const void* base, uint32_t off, float* out) {
    __try {
        *out = *reinterpret_cast<const float*>(reinterpret_cast<const char*>(base) + off);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool Rd8(const void* base, uint32_t off, uint8_t* out) {
    __try {
        *out = *reinterpret_cast<const uint8_t*>(reinterpret_cast<const char*>(base) + off);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
uint32_t FormIdOf(const void* form) {   // TESForm::refID
    uint32_t id = 0;
    return form && Rd32(form, addr::form::kRefId, &id) ? id : 0;
}

// ---------------------------------------------------------------------------------------------------------------- state
std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    const size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

struct Settings {
    bool enabled = true, gate = true, native = true, friendlyFire = false, crime = false;
    int xpMode = 0;
};
Settings g_set;
bool g_inited = false;
volatile PlayerId g_self = kNoPlayer;
volatile LONG g_nativeFaults = 0;
bool g_nativeOk = false;           // the pins of the hit-data constructor and the funnel matched
uint64_t g_gateLogged = 0, g_gateCalls = 0, g_gateErrors = 0, g_gateOffThread = 0, g_nativeReplays = 0, g_consoleDamage = 0, g_consoleLines = 0, g_consoleFailed = 0;
volatile DWORD g_mainThread = 0;   // the game main thread: set at DeferredInit and every session frame; the gate only acts there

std::function<bool(uint32_t, uint8_t&)> g_slotOfRef;
std::function<uint32_t(uint8_t)> g_refOfSlot;
std::map<uint32_t, bool> g_named;      // NetId -> named (from the actor port's tracked events); its keys are every actor the port ever tracked here

uint64_t NowMs() { return static_cast<uint64_t>(GetTickCount64()); }

uint32_t RefOfSlot(uint8_t slot) {
    if (slot == g_self) return eng::kLocalPlayerRef;
    if (g_refOfSlot) return g_refOfSlot(slot);
    return slot != 0 ? nvmp::GhostRefOfPlayer(slot) : 0;
}
bool SlotOfRef(uint32_t ref, uint8_t& slot) {
    if (ref == eng::kLocalPlayerRef) { slot = g_self; return g_self != kNoPlayer; }
    if (g_slotOfRef) return g_slotOfRef(ref, slot);
    for (uint8_t s = 1; s < kMaxSlots; ++s)
        if (s != g_self && nvmp::GhostRefOfPlayer(s) == ref) { slot = s; return true; }
    return false;
}

// ---------------------------------------------------------------------------------------------------------------- engine reads
struct ActorRead {
    bool ok = false;
    float x = 0, y = 0, z = 0;
    uint32_t cell = 0, life = 0;
    float health = 0, maxHealth = 0;
    uint8_t limbMask = 0;
};

// Plain-data reads of a loaded actor reference (TESObjectREFR/Actor layouts of xNVSE, verified for the fields used in docs/engine-addresses.md).
bool ReadActorRaw(void* obj, ActorRead* r) {
    __try {
        Actor* a = static_cast<Actor*>(obj);
        r->x = a->posX; r->y = a->posY; r->z = a->posZ;
        r->cell = a->parentCell ? a->parentCell->refID : 0;
        r->life = a->lifeState;
        ActorValueOwner* owner = &a->avOwner;
        r->health = owner->Fn_03(16);
        r->maxHealth = owner->Fn_01(16);
        for (unsigned i = 0; i < 6; ++i) {
            static const unsigned kAv[6] = {26, 25, 27, 28, 29, 30};   // torso (never counted), head, left arm, right arm, left leg, right leg
            if (i == 0) continue;
            const float base = owner->Fn_01(kAv[i]);
            const float cur = owner->Fn_03(kAv[i]);
            if (base > 0 && cur <= 0) r->limbMask = static_cast<uint8_t>(r->limbMask | (1u << i));
        }
        r->ok = true;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        r->ok = false;
        return false;
    }
}

void* LookupRef(uint32_t ref) { return ref ? nvhooks::resolver().lookup(ref) : nullptr; }

bool ExecOn(const std::string& line, uint32_t ref) {
    nvhooks::ApplyScope scope;   // our own console line must not be dropped by the guest gate
    ++g_consoleLines;
    const bool ok = nvhooks::execConsoleLineOn(line, ref);
    if (!ok) ++g_consoleFailed;
    return ok;
}

// ---------------------------------------------------------------------------------------------------------------- IWorldView
class LiveWorld : public IWorldView {
public:
    PlayerId self() const override { return g_self; }
    bool entityOfRef(uint32_t ref, cc::Entity& out) const override {
        if (!ref) return false;
        uint8_t slot = 0;
        if (SlotOfRef(ref, slot)) { out = cc::Entity::player(slot); return true; }
        const uint32_t net = eng::netIdOf(ref);
        if (!net) return false;
        out = cc::Entity::actor(net);
        return true;
    }
    bool entity(const cc::Entity& e, EntityView& out) const override {
        out = EntityView{};
        out.who = e;
        uint32_t ref = 0;
        if (e.isPlayer()) {
            if (e.id >= kMaxSlots) return false;
            ref = RefOfSlot(static_cast<uint8_t>(e.id));
            out.owner = static_cast<PlayerId>(e.id);
            out.simulatedHere = e.id == g_self;
        } else if (e.isActor()) {
            const eng::OwnerRef o = eng::ownerOfNet(e.id);
            if (!o.known) return false;
            ref = eng::refOf(e.id);
            out.owner = o.ownerSlot;
            out.epoch = o.epoch;
            out.simulatedHere = o.mine && !o.pending && !o.standIn;
            out.pending = o.pending;
            out.standIn = o.standIn;
            auto nm = g_named.find(e.id);
            out.named = nm != g_named.end() && nm->second;
        } else {
            return false;
        }
        out.ref = ref;
        void* obj = LookupRef(ref);
        if (!obj) { out.known = e.isActor(); out.alive = true; return out.known; }   // tracked but not loaded here
        ActorRead r;
        if (!ReadActorRaw(obj, &r)) { out.known = true; return true; }
        out.known = true;
        out.cell = r.cell; out.pos = {r.x, r.y, r.z}; out.radius = 100.0f;
        out.health = static_cast<int32_t>(r.health + 0.5f);
        out.maxHealth = r.maxHealth > 0 ? static_cast<int32_t>(r.maxHealth + 0.5f) : 0;
        out.limbMask = r.limbMask;
        out.alive = !(r.life == 1 || r.life == 2) && r.health > 0;
        return true;
    }
    std::vector<cc::Entity> entitiesNear(cc::CellId cell, const cc::Vec3& pos, float radius) const override {
        std::vector<cc::Entity> out;
        auto consider = [&](const cc::Entity& e) {
            EntityView v;
            if (!entity(e, v) || !v.known || !v.ref || v.cell != cell) return;
            if (cc::distance(v.pos, pos) <= radius + v.radius) out.push_back(e);
        };
        for (const auto& kv : g_named) consider(cc::Entity::actor(kv.first));
        for (uint8_t s = 0; s < kMaxSlots; ++s) if (s == g_self || RefOfSlot(s)) consider(cc::Entity::player(s));
        return out;
    }
    std::vector<PlayerId> presentPlayers() const override {
        std::vector<PlayerId> v;
        for (uint8_t s = 0; s < kMaxSlots; ++s) if (s == g_self || RefOfSlot(s)) v.push_back(s);
        return v;
    }
};
LiveWorld g_world;

// ---------------------------------------------------------------------------------------------------------------- ICombatEngine
const char* LimbAvName(cc::Limb l) {
    switch (l) {   // the AV names found in FalloutNV.exe (0xC66110..0xC661A4); numbers are xNVSE eActorVal_Head..RightLeg (25..30)
        case cc::Limb::Head: return "PerceptionCondition";
        case cc::Limb::Torso: return "EnduranceCondition";
        case cc::Limb::LeftArm: return "LeftAttackCondition";
        case cc::Limb::RightArm: return "RightAttackCondition";
        case cc::Limb::LeftLeg: return "LeftMobilityCondition";
        case cc::Limb::RightLeg: return "RightMobilityCondition";
        default: return nullptr;
    }
}
using ApplyHitFn = int(__thiscall*)(void* victim, void* hit, int flag);
using HitCtorFn = void*(__thiscall*)(void* self);

// Replays one accepted hit through the engine's own funnel with a fresh, constructor-initialised ActorHitData (the hit reaction, blood, scripts'
// OnHit / OnHitWith, the attacker becoming known to the victim, the death logic: all native). false = it faulted or the pins did not match.
// The field filling is hit_edit.cpp (unit-tested on a buffer); only the two engine calls live here.
bool NativeReplayRaw(void* victim, const nvcoop::combat::NativeHitSpec& spec) {
    __try {
        alignas(16) uint8_t buf[addr::hit::kSize];
        std::memset(buf, 0, sizeof buf);
        reinterpret_cast<HitCtorFn>(0x009B4D90)(buf);
        nvcoop::combat::fillNativeHit(buf, spec);
        reinterpret_cast<ApplyHitFn>(addr::kApplyHit)(victim, buf, 0);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

const uint8_t kHitCtorPrologue[16] = {0x55, 0x8B, 0xEC, 0x51, 0x89, 0x4D, 0xFC, 0x8B, 0x4D, 0xFC, 0x83, 0xC1, 0x38, 0xE8, 0x1E, 0xC8};

float KillXpRaw(void* actor) {
    __try {
        void** vt = *reinterpret_cast<void***>(actor);
        const uint32_t fn = reinterpret_cast<uint32_t>(vt[addr::kActorKillXpVtblOffset / 4]);
        if (fn < 0x00401000u || fn >= 0x00FDF000u) return -1.0f;   // not code of the exe
        return reinterpret_cast<float(__thiscall*)(void*)>(fn)(actor);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1.0f;
    }
}

class LiveEngine : public ICombatEngine {
public:
    bool damageHealth(const cc::Entity& victim, uint32_t victimRef, int32_t amount, uint32_t attackerRef, const cc::HitEvent& hit) override {
        if (amount <= 0 || !victimRef) return false;
        void* v = LookupRef(victimRef);
        if (!v) return false;
        if (g_set.native && g_nativeOk && g_nativeFaults == 0) {
            nvcoop::combat::NativeHitSpec n;
            n.target = reinterpret_cast<uint32_t>(v);
            n.source = attackerRef ? reinterpret_cast<uint32_t>(LookupRef(attackerRef)) : 0;
            const uint32_t wid = hit.weapon.plugin.empty() ? 0 : nvhooks::resolver().fromRef(hit.weapon);
            n.weapon = wid ? reinterpret_cast<uint32_t>(nvhooks::resolver().lookup(wid)) : 0;
            n.health = static_cast<float>(amount);
            n.location = nvcoop::combat::hitLocationOfLimb(hit.limb);
            n.haveImpact = !hit.impactPoint.zero();
            n.impact = hit.impactPoint;
            n.crit = hit.critical;
            if (NativeReplayRaw(v, n)) { ++g_nativeReplays; return true; }
            InterlockedIncrement(&g_nativeFaults);
            nvcoop::Log("combat: the native hit replay FAULTED, falling back to console damage for the rest of the session");
        }
        ++g_consoleDamage;
        char line[64];
        std::snprintf(line, sizeof line, "DamageActorValue Health %d", amount);
        (void)victim;
        return ExecOn(line, victimRef);
    }
    // The owner's own kill, for a victim an accepted lethal hit left standing: the same console command the actor port uses for a death
    // (script_engine.cpp lines::kill); the port's death poll then sees the corpse and declares the death.
    bool killActor(uint32_t ref) override {
        static int logged = 0;
        if (ref && logged < 20) {
            ++logged;
            void* obj = LookupRef(ref);
            ActorRead r;
            const bool ok = obj && ReadActorRaw(obj, &r);
            nvcoop::Log("combat: owner kills %08X (life state %u, health %.1f): the engine left a victim of a lethal hit standing", ref, ok ? r.life : 99u, ok ? static_cast<double>(r.health) : 0.0);
        }
        return ref && ExecOn("KillActor", ref);
    }
    // G04: the kill credited to a remote player's puppet. The engine stores the killer (Actor::Kill, actor+0xC0) before it raises OnDeath, so `IsKiller player` is true for the guest.
    bool killActorBy(uint32_t ref, uint32_t killer) override {
        if (!ref) return false;
        if (killer && nvhooks::killWithAttribution(ref, killer)) return true;
        return killActor(ref);
    }
    bool setKiller(uint32_t ref, uint32_t killer) override { return ref && killer && nvhooks::setKillerOf(ref, killer); }
    bool setLimbCrippled(const cc::Entity&, uint32_t ref, cc::Limb limb) override {
        const char* av = LimbAvName(limb);
        if (!av || !ref) return false;
        return ExecOn(std::string("ForceActorValue ") + av + " 0", ref);
    }
    bool mirrorHealth(uint32_t ref, int32_t health) override {
        if (!ref) return false;
        return ExecOn("ForceActorValue Health " + std::to_string(health), ref);
    }
    bool provoke(uint32_t victimRef, uint32_t attackerRef) override {
        if (!victimRef || !attackerRef) return false;
        char line[48];
        std::snprintf(line, sizeof line, "StartCombat %08X", attackerRef);
        return ExecOn(line, victimRef);
    }
    bool decorateCorpse(uint32_t ref, uint8_t mask) override {
        bool ok = true;
        for (unsigned i = 1; i < cc::kLimbCount; ++i)
            if ((mask >> i) & 1u) ok = setLimbCrippled(cc::Entity::actor(0), ref, static_cast<cc::Limb>(i)) && ok;
        return ok;
    }
    bool isDead(uint32_t ref) override {
        void* obj = LookupRef(ref);
        ActorRead r;
        return obj && ReadActorRaw(obj, &r) && (r.life == 1 || r.life == 2 || r.health <= 0);
    }
    // the life state alone (Actor+0x108 = 1 dying / 2 dead): what the actor port's death poll looks at. A VATS kill can leave health <= 0 with life state 0.
    bool lifeDead(uint32_t ref) override {
        void* obj = LookupRef(ref);
        ActorRead r;
        return obj && ReadActorRaw(obj, &r) && (r.life == 1 || r.life == 2);
    }
    // The engine's own kill-XP award (inside ApplyHitData, 0x89E5xx..0x89E6FE) takes a float from a virtual of the VICTIM at vtable offset 0x730 (called with
    // no arguments), scales it by the GMST difficulty tiers and the player's XP multiplier, and hands the integer to Player vtable +0x488 (RewardXP). We use
    // the raw value (no tier / perk scaling): confidence M, SEH-guarded, -1 (= no award) when it does not look like XP.
    int32_t killXpOf(uint32_t ref) override {
        void* obj = LookupRef(ref);
        if (!obj) return -1;
        const float x = KillXpRaw(obj);
        return x > 0.0f && x <= 20000.0f ? static_cast<int32_t>(x + 0.5f) : -1;
    }
    bool awardXp(int32_t xp) override { return xp > 0 && ExecOn("RewardXP " + std::to_string(xp), eng::kLocalPlayerRef); }
    bool spawnVisualProjectile(const cc::ProjectileSpawn&) override { return false; }   // TODO-RE: no verified way to create a harmless projectile
    bool spawnVisualExplosion(const cc::ExplosionEvent&) override { return false; }     // TODO-RE: PlaceAtMe needs a reference at the blast position
    bool applyHostility(const cc::HostilityEffect& e) override {
        const uint32_t a = nvhooks::resolver().fromRef(e.factionA), b = nvhooks::resolver().fromRef(e.factionB);
        if (e.kind != cc::HostilityKind::StopCombat && (!a || !b)) return false;
        char ha[16], hb[16];
        std::snprintf(ha, sizeof ha, "%08X", a);
        std::snprintf(hb, sizeof hb, "%08X", b);
        const std::string line = cc::consoleLineFor(e, ha, hb);
        return !line.empty() && ExecOn(line, 0);
    }
    bool vatsActive() override {
        uint32_t mode = 0;
        return Rd32(reinterpret_cast<const void*>(addr::kVatsCameraData), addr::kVatsModeOffset, &mode) && mode != 0;
    }
};
LiveEngine g_engine;

// ---------------------------------------------------------------------------------------------------------------- the hit gate
std::unique_ptr<CombatRuntime> g_rt;
std::unique_ptr<CallSitePatches> g_patches;

// Everything the gate reads from the engine's hit data is taken from ONE guarded copy (the byte-level decode / edit is hit_edit.cpp, unit-tested);
// the pointers inside it (attacker, weapon, projectile / explosion) are followed with their own guarded reads.
bool CopyHit(const void* hd, uint8_t* out) {
    __try {
        std::memcpy(out, hd, addr::hit::kSize);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool PokeHit(void* hd, const uint8_t* edited) {   // writes back only the four fields the gate edits
    __try {
        char* h = static_cast<char*>(hd);
        std::memcpy(h + addr::hit::kHealthDmg, edited + addr::hit::kHealthDmg, 4);
        std::memcpy(h + addr::hit::kFatigueDmg, edited + addr::hit::kFatigueDmg, 4);
        std::memcpy(h + addr::hit::kLimbDmg, edited + addr::hit::kLimbDmg, 4);
        std::memcpy(h + addr::hit::kFlags, edited + addr::hit::kFlags, 4);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

struct ObjRead {
    uint8_t type = 0;
    float x = 0, y = 0, z = 0;
    uint32_t base = 0;
};
bool ReadObj(const void* po, ObjRead* r) {   // the projectile / explosion object: form type, position, base form
    uint32_t base = 0;
    bool ok = Rd8(po, addr::form::kTypeId, &r->type) && RdF(po, addr::form::kPosX, &r->x) && RdF(po, addr::form::kPosX + 4, &r->y) &&
              RdF(po, addr::form::kPosX + 8, &r->z);
    if (ok && Rd32(po, addr::form::kBaseForm, &base)) r->base = FormIdOf(reinterpret_cast<const void*>(base));
    return ok;
}

bool IsProjectileType(uint8_t t) {
    return t == addr::form::kTypeMissile || t == addr::form::kTypeGrenade || t == addr::form::kTypeBeam || t == addr::form::kTypeFlame ||
           t == addr::form::kTypeContinuousBeam;
}

int __fastcall HitGateThunk(void* victim, void*, void* hd, int flag) {
    ApplyHitFn orig = reinterpret_cast<ApplyHitFn>(addr::kApplyHit);
    CombatSync* cs = g_rt ? g_rt->sync() : nullptr;
    if (cs && g_mainThread && GetCurrentThreadId() != g_mainThread) { ++g_gateOffThread; cs = nullptr; }   // CombatSync is main-thread only: other threads pass through
    if (cs && hd && victim) {
        ++g_gateCalls;
        uint8_t buf[addr::hit::kSize];
        nvcoop::combat::HitBuf raw;
        if (CopyHit(hd, buf) && nvcoop::combat::decodeHit(buf, &raw)) {
            EngineHit eh;
            eh.attackerRef = FormIdOf(reinterpret_cast<const void*>(raw.source));
            eh.victimRef = FormIdOf(victim);
            eh.healthDmg = raw.health; eh.fatigueDmg = raw.fatigue; eh.limbDmg = raw.limb;
            eh.hitLocation = raw.location;
            eh.impact = raw.impact;
            eh.critical = (raw.flags & addr::hit::kFlagCrit) != 0;
            const uint32_t weaponForm = FormIdOf(reinterpret_cast<const void*>(raw.weapon));
            if (weaponForm) nvhooks::resolver().toRef(weaponForm, &eh.weapon);
            if (raw.projOrExplosion) {
                ObjRead o;
                if (ReadObj(reinterpret_cast<const void*>(raw.projOrExplosion), &o)) {
                    if (IsProjectileType(o.type)) {
                        eh.projectileKey = raw.projOrExplosion;
                        if (o.base) nvhooks::resolver().toRef(o.base, &eh.projectileBase);
                    } else {
                        eh.explosionKey = raw.projOrExplosion;
                        eh.blastCenter = {o.x, o.y, o.z};
                        if (o.base) nvhooks::resolver().toRef(o.base, &eh.explosionBase);
                    }
                }
            }
            eh.vats = g_engine.vatsActive();
            GateDecision d;
            bool decided = false;
            try {
                d = cs->onEngineHit(eh);
                decided = true;
            } catch (...) {
                ++g_gateErrors;
            }
            if (decided && g_gateLogged < 60 && eh.victimRef && (eng::netIdOf(eh.victimRef) || eh.attackerRef == eng::kLocalPlayerRef) && d.action != GateAction::Neutralize) {
                ++g_gateLogged;
                nvcoop::Log("combat: gate %s on victim %08X by %08X, damage %.1f, cap %d: %s", d.action == GateAction::Proceed ? "PROCEED" : "CLAMP", eh.victimRef,
                            eh.attackerRef, static_cast<double>(eh.healthDmg), d.maxHealthDamage, d.why);   // (a victim that is no synced actor reads "not a synced combatant")
            }
            if (decided && nvcoop::combat::applyDecisionToHit(buf, raw, d.action, d.maxHealthDamage) && !PokeHit(hd, buf)) ++g_gateErrors;
        } else {
            ++g_gateErrors;
        }
    }
    return orig(victim, hd, flag);
}

bool InstallGate() {
    if (g_patches && g_patches->installed() > 0) return true;
    if (!g_patches) g_patches = std::make_unique<CallSitePatches>(g_mem);
    const int n = g_patches->install(addr::kApplyHitCallers, addr::kApplyHitCallerCount, addr::kApplyHit, addr::kApplyHitPrologue, 16,
                                     static_cast<uint32_t>(reinterpret_cast<uintptr_t>(&HitGateThunk)));
    for (const SiteResult& r : g_patches->results()) nvcoop::Log("combat: gate site %s @%08X: %s", r.name, r.site, toString(r.status));
    if (!g_patches->prologueOk()) nvcoop::Log("combat: the hit funnel does not match the pinned bytes (%s): combat sync is OFF", g_patches->lastError().c_str());
    nvcoop::Log("combat: hit gate installed on %d of %d call sites", n, static_cast<int>(addr::kApplyHitCallerCount));
    return n > 0;
}

void RemoveGate() {
    if (g_patches) {
        const int restored = g_patches->uninstall();
        nvcoop::Log("combat: hit gate removed (%d sites restored, %d left to another patcher)", restored, g_patches->tampered());
    }
    g_patches.reset();
}

// ---------------------------------------------------------------------------------------------------------------- actor port events
eng::ListenerId g_diedL = 0, g_trackedL = 0, g_lostL = 0;
bool g_listeners = false;

void InstallListeners() {
    if (g_listeners) return;
    g_listeners = true;
    g_trackedL = eng::addActorTrackedListener([](const eng::TrackedEvent& e) { g_named[e.netId] = e.named; });
    g_diedL = eng::addActorDiedListener([](const eng::DiedEvent& e) {
        if (CombatSync* cs = g_rt ? g_rt->sync() : nullptr) cs->onActorDied(e.ref, e.authorised, e.fromNetwork, e.mine, 0);
    });
    g_lostL = eng::addActorLostListener([](const eng::LostEvent& e) {
        if (CombatSync* cs = g_rt ? g_rt->sync() : nullptr) cs->onActorLost(cc::Entity::actor(e.netId));
    });
}
void RemoveListeners() {
    if (!g_listeners) return;
    eng::removeListener(g_diedL);
    eng::removeListener(g_trackedL);
    eng::removeListener(g_lostL);
    g_diedL = g_trackedL = g_lostL = 0;
    g_listeners = false;
}

// ---------------------------------------------------------------------------------------------------------------- module
void Init() {
    if (g_inited) return;
    g_inited = true;
    g_mainThread = GetCurrentThreadId();   // DeferredInit runs on the game main thread
    const std::string ini = IniPath();
    g_set.enabled = GetPrivateProfileIntA("combat", "enabled", 1, ini.c_str()) != 0;
    g_set.gate = GetPrivateProfileIntA("combat", "gate", 1, ini.c_str()) != 0;
    g_set.native = GetPrivateProfileIntA("combat", "native", 1, ini.c_str()) != 0;
    g_set.friendlyFire = GetPrivateProfileIntA("combat", "friendlyfire", 0, ini.c_str()) != 0;
    g_set.crime = GetPrivateProfileIntA("combat", "crime", 0, ini.c_str()) != 0;
    g_set.xpMode = GetPrivateProfileIntA("combat", "xpmode", 0, ini.c_str());
    if (!g_set.enabled) { nvcoop::Log("combat: disabled by nvcoop.ini [combat] enabled=0"); return; }
    // The funnel and the hit-data constructor must be this exe's: otherwise native replay stays off (console damage still works).
    uint8_t got[16] = {};
    g_nativeOk = g_mem.read(addr::kApplyHit, got, 16) && std::memcmp(got, addr::kApplyHitPrologue, 16) == 0 &&
                 g_mem.read(0x009B4D90, got, 16) && std::memcmp(got, kHitCtorPrologue, 16) == 0;
    nvcoop::RegisterMethodClosure("combat.status", [](const std::string&, std::string& out) {
        out = glueStatusJson();
        return true;
    }, true, 0);
    InstallListeners();
    nvcoop::Log("combat: enabled gate=%d native=%d (pins %s) friendlyfire=%d crime=%d", g_set.gate ? 1 : 0, g_set.native ? 1 : 0, g_nativeOk ? "ok" : "MISMATCH",
                g_set.friendlyFire ? 1 : 0, g_set.crime ? 1 : 0);
}

void Uninit() {
    RemoveGate();
    RemoveListeners();
    g_rt.reset();
    g_inited = false;
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_DeferredInit) Init();
    else if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) Uninit();
}

}  // namespace

NVCOOP_MODULE(combat_hooks, "combat_hooks", nullptr, OnMessage, nullptr);

// Hot reload: the call sites are restored and the listeners removed before the core is replaced; the new core installs them again on its next session frame.
NVCOOP_RELOAD_STATE(combat_hooks, "combat_hooks", 1, 40,
    /*canUnload*/ [] {
        const int n = g_patches ? g_patches->audit() : 0;
        return n ? std::string("a hit-gate call site was patched over by another mod") : std::string();
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

namespace nvcoop::combat {

void setPlayerRefMap(std::function<bool(uint32_t, uint8_t&)> slotOfRef, std::function<uint32_t(uint8_t)> refOfSlot) {
    g_slotOfRef = std::move(slotOfRef);
    g_refOfSlot = std::move(refOfSlot);
}

bool glueRunning() { return g_rt && g_rt->sync(); }

GateDecision glueGate(const EngineHit& h) {
    CombatSync* cs = g_rt ? g_rt->sync() : nullptr;
    if (!cs) return GateDecision{};
    try {
        return cs->onEngineHit(h);
    } catch (...) {
        ++g_gateErrors;
        return GateDecision{};
    }
}

void glueFrame(nvcoop::coop::Runtime* rt) {
    if (!g_inited || !g_set.enabled) return;
    g_mainThread = GetCurrentThreadId();
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
        } else if (auto* cl = rt->client()) {
            f.isHost = false;
            f.self = cl->joined() ? cl->slot() : kNoPlayer;
            f.hostLinkUp = cl->joined() && !cl->baselineActive();
        } else {
            f.active = false;
        }
    }
    g_self = f.active ? f.self : kNoPlayer;
    if (!g_rt) {
        if (!f.active) return;
        g_rt.reset(new CombatRuntime(g_world, g_engine));
        // G04: when the actor port kills the host's copy of an actor a guest killed, it asks who to credit (the guest's puppet) and runs the kill as that puppet.
        nvcoop::actors::engine::setKillerResolver([](uint32_t victimRef) -> uint32_t {
            const CombatSync* cs = g_rt ? g_rt->sync() : nullptr;
            return cs ? cs->killerPuppetOf(victimRef) : 0u;
        });
        g_rt->config().friendlyFire = g_set.friendlyFire;
        g_rt->config().crimeSync = g_set.crime;
        g_rt->config().xpMode = g_set.xpMode == 1 ? cc::XpMode::PartyShare : cc::XpMode::KillingBlow;
        g_rt->config().social.blockProtectedHit = [](uint32_t victimRef, fnvcoop::PlayerId attacker, bool splash) {   // G40: rules FriendlyFireGuard (host only; "allow" without them)
            nvcoop::coop::Runtime* rt = nvcoop::coop::Runtime::active();
            fnvcoop::FormRef v;
            return rt && nvhooks::resolver().toRef(victimRef, &v) && rt->protectedHit(v, attacker, splash) != nvcoop::coop::Runtime::HitVerdict::Allow;
        };
#ifdef NVCOOP_WITH_COMPANIONS
        // plugin/companions (companions_glue.h). Each call is a no-op / "allow" while [companions] enabled=0 or no social session runs, and is asked LIVE
        // (the social module may come up after this runtime), so nothing changes for a game that does not switch it on.
        nvcoop::combat::SocialHooks& sh = g_rt->config().social;
        sh.xpHandled = [] { return nvcoop::social::glueXpHandled(); };
        sh.reportKill = [](const fnvcoop::social::KillReport& r) { nvcoop::social::glueReportKill(r); };
        sh.reportCrime = [](const fnvcoop::social::CrimeEvent& e) { nvcoop::social::glueReportCrime(e); };
        sh.neutralizeHit = [](uint32_t attackerRef, uint32_t victimRef) {
            return nvcoop::social::glueHitPolicy(attackerRef, victimRef) == fnvcoop::social::HitPolicy::Neutralize;
        };
#endif
    }
    if (f.active && g_set.gate) InstallGate();
    g_rt->frame(f);
    // the local player's death / respawn (the actor port tracks NPCs, not the player)
    if (CombatSync* cs = g_rt->sync()) {
        EntityView v;
        if (g_world.entity(cc::Entity::player(g_self), v) && v.known) {
            static bool wasDead = false;
            const bool dead = !v.alive;
            if (dead && !wasDead) cs->onPlayerDied(0);
            else if (!dead && wasDead) cs->onPlayerAlive();
            wasDead = dead;
        }
    }
}

namespace {
std::string JsonEscape(const std::string& in) {
    std::string out;
    for (char c : in) {
        if (c == '"' || c == '\\') { out += '\\'; out += c; }
        else if (c == '\n') out += "\\n";
        else if (static_cast<unsigned char>(c) >= 0x20) out += c;
    }
    return out;
}
}  // namespace

std::string glueStatusJson() {
    char b[3072];
    const CombatSync* cs = g_rt ? g_rt->sync() : nullptr;
    const Stats* s = cs ? &cs->stats() : nullptr;
    auto u = [&](uint64_t Stats::*m) { return static_cast<unsigned long long>(s ? s->*m : 0); };
    std::snprintf(b, sizeof b,
                  "{\"enabled\":%s,\"running\":%s,"
                  "\"gate\":{\"sites\":%d,\"calls\":%llu,\"errors\":%llu,\"offThread\":%llu},"
                  "\"native\":{\"pinsOk\":%s,\"faults\":%ld,\"replays\":%llu,\"consoleDamage\":%llu,\"consoleLines\":%llu,\"consoleFailed\":%llu},"
                  "\"stats\":{\"engineHits\":%llu,\"proceeded\":%llu,\"clamped\":%llu,\"neutralized\":%llu,\"ghostHitsNeutralized\":%llu,"
                  "\"hitsSent\":%llu,\"hitsReceived\":%llu,\"hitsApplied\":%llu,\"hitsRejected\":%llu,\"hitDuplicates\":%llu,"
                  "\"nacksSent\":%llu,\"hitRetries\":%llu,\"hitsAbandoned\":%llu,\"damageApplied\":%llu,"
                  "\"deathsDeclared\":%llu,\"deathsAccepted\":%llu,\"xpAwards\":%llu,"
                  "\"deathsRejected\":%llu,\"deathsDeferred\":%llu,\"deathsRefusedLocal\":%llu,\"undecodable\":%llu,\"spoofed\":%llu,\"unroutable\":%llu,"
                  "\"naturalHits\":%llu,\"envHitsNeutralized\":%llu,\"hitsResolvedLate\":%llu,\"killsForced\":%llu,\"damageRefused\":%llu,\"engineRefused\":%llu,"
                  "\"naturalLethal\":%llu,\"naturalLethalStuck\":%llu,\"lastGate\":\"%s\",\"rejections\":\"%s\","
                  "\"rebuilds\":%llu,\"droppedUnknownPeer\":%llu}}",
                  g_set.enabled ? "true" : "false", cs ? "true" : "false",
                  g_patches ? g_patches->installed() : 0, (unsigned long long)g_gateCalls, (unsigned long long)g_gateErrors, (unsigned long long)g_gateOffThread,
                  g_nativeOk ? "true" : "false", (long)g_nativeFaults, (unsigned long long)g_nativeReplays, (unsigned long long)g_consoleDamage,
                  (unsigned long long)g_consoleLines, (unsigned long long)g_consoleFailed,
                  u(&Stats::engineHits), u(&Stats::proceeded), u(&Stats::clamped), u(&Stats::neutralized), u(&Stats::ghostHitsNeutralized),
                  u(&Stats::hitsSent), u(&Stats::hitsReceived), u(&Stats::hitsApplied), u(&Stats::hitsRejected), u(&Stats::hitDuplicates),
                  u(&Stats::nacksSent), u(&Stats::hitRetries), u(&Stats::hitsAbandoned), u(&Stats::damageApplied),
                  u(&Stats::deathsDeclared), u(&Stats::deathsAccepted), u(&Stats::xpAwards),
                  u(&Stats::deathsRejected), u(&Stats::deathsDeferred), u(&Stats::deathsRefusedLocal), u(&Stats::undecodable), u(&Stats::spoofed),
                  u(&Stats::unroutable),   // a death or hit the host had nobody to send to: the client was never seen as joined
                  u(&Stats::naturalHits), u(&Stats::envHitsNeutralized), u(&Stats::hitsResolvedLate), u(&Stats::killsForced), u(&Stats::damageRefused),
                  u(&Stats::engineRefused), u(&Stats::naturalLethal), u(&Stats::naturalLethalStuck), JsonEscape(cs ? cs->lastGate() : std::string()).c_str(), JsonEscape(cs ? cs->lastRejections() : std::string()).c_str(),
                  (unsigned long long)(g_rt ? g_rt->rebuilds() : 0), (unsigned long long)(g_rt ? g_rt->droppedUnknownPeer() : 0));
    return b;
}

}  // namespace nvcoop::combat
