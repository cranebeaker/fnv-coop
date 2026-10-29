// Game side of the NPC animation layer. x86, compiled into nvcoop.dll against the xNVSE headers. See anim_game.h.
#include <Windows.h>

#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <map>
#include <string>

#include "actors/engine/actor_engine_api.h"
#include "anim_game.h"
#include "anim_groups.h"
#include "anim_pins.h"

#include "nvse/GameAPI.h"
#include "nvse/GameForms.h"
#include "nvse/GameObjects.h"
#include "nvse/GameProcess.h"

namespace nvcoop::actors::anim::game {

namespace {

namespace P = nvcoop::actors::anim::pins;

// ---- guarded raw access (no C++ objects with destructors in any function that has a __try) -----------------------------------------------
bool SafeRead(uintptr_t addr, void* out, size_t n) {
    __try {
        std::memcpy(out, reinterpret_cast<const void*>(addr), n);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
template <class T>
bool Rd(uintptr_t addr, T& out) { return SafeRead(addr, &out, sizeof(T)); }

typedef uint32_t(__thiscall* Fn0)(void*);
bool CallSlot0(void* obj, uint32_t slot, uint32_t& ret) {
    __try {
        uintptr_t vt = *reinterpret_cast<uintptr_t*>(obj);
        Fn0 f = *reinterpret_cast<Fn0*>(vt + 4 * slot);
        ret = f(obj);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallSetWeaponOut(uintptr_t fn, void* proc, void* actor, bool on) {
    __try {
        reinterpret_cast<void(__thiscall*)(void*, void*, bool)>(fn)(proc, actor, on);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallSetBool(uintptr_t fn, void* proc, bool on) {
    __try {
        reinterpret_cast<void(__thiscall*)(void*, bool)>(fn)(proc, on);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallSetChar(uintptr_t fn, void* proc, char v) {
    __try {
        reinterpret_cast<void(__thiscall*)(void*, char)>(fn)(proc, v);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallPushAway(uintptr_t fn, void* proc, void* actor, float x, float y, float z, float force) {
    __try {
        reinterpret_cast<void(__thiscall*)(void*, void*, float, float, float, float)>(fn)(proc, actor, x, y, z, force);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool WriteFloat(uintptr_t addr, float v) {
    __try {
        *reinterpret_cast<float*>(addr) = v;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallMenuMode(uintptr_t fn, bool& out) {
    __try {
        out = reinterpret_cast<bool(__cdecl*)()>(fn)();
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

struct RefPose {
    float rx = 0, ry = 0, rz = 0, px = 0, py = 0, pz = 0;
    uint32_t cell = 0;
    uint32_t type = 0;
};
bool ReadRefPose(TESObjectREFR* r, RefPose& o) {
    __try {
        o.rx = r->rotX; o.ry = r->rotY; o.rz = r->rotZ;
        o.px = r->posX; o.py = r->posY; o.pz = r->posZ;
        o.cell = r->parentCell ? r->parentCell->refID : 0;
        o.type = r->typeID;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool PlayerReady() {
    __try {
        return g_thePlayer && *g_thePlayer && (*g_thePlayer)->parentCell != nullptr;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
TESForm* LookupSafe(uint32_t id) {
    __try {
        return LookupFormByID(id);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}
// The placed furniture reference nearest to (x, y, z) in a cell, within the pinned reach; 0 = none. Plain data only (SEH).
uint32_t NearestFurnitureRef(TESObjectCELL* cell, float x, float y, float z) {
    uint32_t best = 0;
    float bestD2 = P::kFurnitureReachXY * P::kFurnitureReachXY;
    __try {
        int guard = 0;
        for (auto* node = cell->objectList.Head(); node && guard < 20000; node = node->next, ++guard) {
            TESObjectREFR* r = node->data;
            if (!r || !r->refID || !r->baseForm || (r->flags & P::kFormFlagDeleted)) continue;
            if (r->typeID != kFormType_TESObjectREFR || r->baseForm->typeID != kFormType_TESFurniture) continue;
            const float dx = r->posX - x, dy = r->posY - y, dz = r->posZ - z;
            if (dz > P::kFurnitureReachZ || dz < -P::kFurnitureReachZ) continue;
            const float d2 = dx * dx + dy * dy;
            if (d2 < bestD2) { bestD2 = d2; best = r->refID; }
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
    return best;
}
TESObjectCELL* CellOf(TESObjectREFR* r) {
    __try {
        return r->parentCell;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}

uint32_t FormTypeOf(TESForm* f) {
    __try {
        return f->typeID;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

std::map<NetId, uint32_t>& Overrides() {
    static std::map<NetId, uint32_t> m;
    return m;
}
uint32_t RefFor(NetId id) {
    auto it = Overrides().find(id);
    return it != Overrides().end() ? it->second : engine::refOf(id);
}

struct FurnCache {
    uint32_t ref = 0;
    float x = 0, y = 0, z = 0;
};
std::map<uint32_t, FurnCache>& FurnitureCache() {
    static std::map<uint32_t, FurnCache> m;
    return m;
}

Actor* ActorOf(NetId id) {
    const uint32_t ref = RefFor(id);
    if (!ref) return nullptr;
    TESForm* f = LookupSafe(ref);
    if (!f) return nullptr;
    const uint32_t t = FormTypeOf(f);
    if (t != kFormType_Character && t != kFormType_Creature) return nullptr;
    return static_cast<Actor*>(static_cast<TESObjectREFR*>(f));
}

// The function the actor's process holds in `slot` must be exactly the pinned one (verify_slots.py proved it for the HighProcess vtable).
bool ResolveProcFn(Actor* a, uint32_t slot, uint32_t expectedFn, void*& proc, uintptr_t& fn) {
    uintptr_t p = 0, vt = 0, f = 0;
    if (!Rd(reinterpret_cast<uintptr_t>(a) + P::kActorBaseProcess, p) || !p) return false;
    if (!Rd(p, vt) || !vt || !Rd(vt + 4u * slot, f)) return false;
    if (f != expectedFn) return false;
    proc = reinterpret_cast<void*>(p);
    fn = f;
    return true;
}

struct SeqRead {
    bool ok = false;
    int group = -1;
    int state = -1;
    float last = 0, begin = 0, freq = 1;
    uintptr_t addr = 0;
};
SeqRead ReadSeq(uintptr_t seq) {
    SeqRead s;
    uintptr_t vt = 0;
    if (!seq || !Rd(seq, vt) || (vt != P::kVtblBSAnimGroupSequence && vt != P::kVtblNiControllerSequence)) return s;
    s.ok = true; s.addr = seq;
    uint32_t state = 0;
    float f = 0;
    if (Rd(seq + P::kSeqState, state)) s.state = static_cast<int>(state);
    if (Rd(seq + P::kSeqLast, f)) s.last = f;
    if (Rd(seq + P::kSeqBegin, f)) s.begin = f;
    if (Rd(seq + P::kSeqFreq, f)) s.freq = f;
    if (vt == P::kVtblBSAnimGroupSequence) {
        uintptr_t grp = 0;
        uint8_t id = 0;
        if (Rd(seq + P::kSeqAnimGroup, grp) && grp && Rd(grp + P::kAnimGroupId, id)) s.group = id;
    }
    return s;
}

}  // namespace

// ---- script ------------------------------------------------------------------------------------------------------------------------------

void setRefOverride(NetId id, uint32_t ref) { Overrides()[id] = ref; }

bool GameScript::Exec(NetId id, const std::string& line) {
    const uint32_t ref = RefFor(id);
    if (!ref || !run_) return false;
    return run_(ref, line);
}

// ---- world -------------------------------------------------------------------------------------------------------------------------------

bool GameAccess::WorldReady() { return PlayerReady(); }

bool GameAccess::MenuOpen() {
    if (!menuPinChecked_) {
        menuPinChecked_ = true;
        uint8_t b[8] = {};
        menuPinOk_ = SafeRead(P::kMenuModeAny, b, sizeof b) && std::memcmp(b, P::kMenuModeBytes, sizeof b) == 0;
    }
    if (!menuPinOk_) return false;
    bool open = false;
    return CallMenuMode(P::kMenuModeAny, open) && open;
}

// ---- reads -------------------------------------------------------------------------------------------------------------------------------

uint32_t GameAccess::Generation3D(NetId id) {
    Actor* a = ActorOf(id);
    if (!a) return 0;
    uintptr_t rs = 0;
    uint32_t node = 0;
    if (Rd(reinterpret_cast<uintptr_t>(a) + P::kRefRenderState, rs) && rs) Rd(rs + P::kRenderStateNiNode, node);
    return node;
}

bool GameAccess::GetPose(NetId id, CellId& cell, Vec3& pos, float& yawRad) {
    Actor* a = ActorOf(id);
    RefPose rp;
    if (!a || !ReadRefPose(a, rp)) return false;
    cell = rp.cell;
    pos = {rp.px, rp.py, rp.pz};
    yawRad = rp.rz;   // REFR rotZ is the heading
    return true;
}

bool GameAccess::ReadActor(NetId id, NpcReads& out) {
    out = NpcReads{};
    Actor* a = ActorOf(id);
    if (!a) return false;
    RefPose rp;
    if (!ReadRefPose(a, rp)) return false;
    const uintptr_t ap = reinterpret_cast<uintptr_t>(a);
    out.valid = true;
    out.creature = rp.type == kFormType_Creature;

    uint32_t life = 0;
    if (Rd(ap + P::kActorLifeState, life)) out.dead = (life == 1 || life == 2);   // Actor::IsDead treats 1 and 2 as dead / dying
    uint8_t sw = 0;
    if (Rd(ap + P::kActorSwimming, sw)) out.swimming = sw != 0;
    uintptr_t rag = 0;
    if (Rd(ap + P::kActorRagdoll, rag)) out.ragdoll = rag != 0;

    uintptr_t rs = 0;
    uint32_t node = 0;
    if (Rd(ap + P::kRefRenderState, rs) && rs) Rd(rs + P::kRenderStateNiNode, node);
    out.has3D = node != 0;

    uintptr_t proc = 0;
    Rd(ap + P::kActorBaseProcess, proc);
    if (proc) {
        void* p = reinterpret_cast<void*>(proc);
        uint32_t v = 0;
        if (CallSlot0(p, P::kSlotIsWeaponOut, v)) out.weaponOut = (v & 0xFF) != 0;
        if (CallSlot0(p, 0x101, v)) out.aiming = (v & 0xFF) != 0;                       // IsAiming (slot proven by verify_engine.py)
        if (CallSlot0(p, P::kSlotGetKnockedState, v)) out.knocked = static_cast<int32_t>(v) != 0;
        // sit / sleep state: only from a process whose slot IS the pinned getter (a process of another class reads as "cannot tell")
        {
            void* sp = nullptr;
            uintptr_t sfn = 0;
            if (ResolveProcFn(a, P::kSlotGetSitSleepState, P::kFnGetSitSleepState, sp, sfn) && CallSlot0(sp, P::kSlotGetSitSleepState, v)) {
                out.sitKnown = true;
                out.sitSleep = static_cast<int>(v);
            }
        }

        uint32_t animData = 0;
        if (CallSlot0(p, P::kProcGetAnimData, animData) && animData) {
            // The meaning of the nine slots is NOT proven. Classify by the group each playing sequence holds: a one-shot group is the action,
            // anything else the movement / idle sequence. No sequence at all on a skeleton that exists = the bind pose.
            out.moveGroup = NpcReads::kNoSequence;
            for (int i = 8; i >= 0; --i) {
                uintptr_t seq = 0;
                const uintptr_t at = animData + (i < 8 ? P::kAnimDataSeqBase + 4u * static_cast<uint32_t>(i) : P::kAnimDataSeq100);
                if (!Rd(at, seq) || !seq) continue;
                const SeqRead s = ReadSeq(seq);
                if (!s.ok || s.state != 1 || s.group < 0) continue;
                if (isActionGroup(s.group)) {
                    if (out.actionGroup < 0) { out.actionGroup = s.group; out.actionPlayMs = static_cast<int>((s.last - s.begin) * 1000.0f); }
                } else {
                    out.moveGroup = s.group;
                }
            }
        }
    }

    if (out.sitKnown && out.sitSleep != 0) {
        // The furniture in use: cached per actor while the state stays non-zero and the actor does not move (the cell scan is not free).
        FurnCache& fc = FurnitureCache()[a->refID];
        if (fc.ref == 0 || std::fabs(rp.px - fc.x) > 8.0f || std::fabs(rp.py - fc.y) > 8.0f || std::fabs(rp.pz - fc.z) > 8.0f) {
            fc.ref = NearestFurnitureRef(CellOf(a), rp.px, rp.py, rp.pz);
            fc.x = rp.px; fc.y = rp.py; fc.z = rp.pz;
        }
        out.furniture = fc.ref;
    } else {
        FurnitureCache().erase(a->refID);
    }

    uintptr_t mover = 0;
    if (Rd(ap + P::kActorMover, mover) && mover) {
        uintptr_t vt = 0;
        if (Rd(mover, vt) && (vt == P::kVtblActorMover || vt == P::kVtblPlayerMover)) {
            uint32_t flags = 0;
            if (CallSlot0(reinterpret_cast<void*>(mover), 8, flags)) {
                out.moveFlags = flags;
                out.moveFlagsKnown = true;
                out.sneaking = (flags & (1u << 9)) != 0;
                if ((flags & (1u << 11)) != 0) out.swimming = true;
            }
        }
    }
    return true;
}

// ---- direct calls ------------------------------------------------------------------------------------------------------------------------

uint32_t GameAccess::RefIdOf(NetId id) { return RefFor(id); }

Result GameAccess::SetWeaponOut(NetId id, bool on) {
    if (!cfg_.directWeaponOut) return Result::Unsupported;
    Actor* a = ActorOf(id);
    if (!a) return Result::Refused;
    void* proc = nullptr;
    uintptr_t fn = 0;
    if (!ResolveProcFn(a, P::kSlotSetWeaponOut, P::kFnSetWeaponOut, proc, fn)) return Result::Unsupported;   // another process class: the console fallback takes over
    return CallSetWeaponOut(fn, proc, a, on) ? Result::Ok : Result::Refused;
}

Result GameAccess::SetAiming(NetId id, bool on) {
    if (!cfg_.directAiming) return Result::Unsupported;
    Actor* a = ActorOf(id);
    if (!a) return Result::Refused;
    void* proc = nullptr;
    uintptr_t fn = 0;
    if (!ResolveProcFn(a, P::kSlotSetIsAiming, P::kFnSetIsAiming, proc, fn)) return Result::Refused;   // per actor (a low process): retried later, not given up for everybody
    return CallSetBool(fn, proc, on) ? Result::Ok : Result::Refused;
}

Result GameAccess::SetGroupRate(NetId id, float rate) {
    if (!cfg_.directRate) return Result::Unsupported;
    Actor* a = ActorOf(id);
    if (!a || !(rate > 0.2f && rate < 4.0f)) return Result::Refused;
    uintptr_t proc = 0;
    uint32_t animData = 0;
    if (!Rd(reinterpret_cast<uintptr_t>(a) + P::kActorBaseProcess, proc) || !proc) return Result::Refused;
    if (!CallSlot0(reinterpret_cast<void*>(proc), P::kProcGetAnimData, animData) || !animData) return Result::Refused;
    // the movement sequence = the playing one whose group is not a one-shot (same classification as ReadActor)
    for (int i = 8; i >= 0; --i) {
        uintptr_t seq = 0;
        const uintptr_t at = animData + (i < 8 ? P::kAnimDataSeqBase + 4u * static_cast<uint32_t>(i) : P::kAnimDataSeq100);
        if (!Rd(at, seq) || !seq) continue;
        const SeqRead s = ReadSeq(seq);
        if (!s.ok || s.state != 1 || s.group < 0 || isActionGroup(s.group)) continue;
        return WriteFloat(s.addr + P::kSeqFreq, rate) ? Result::Ok : Result::Refused;
    }
    return Result::Refused;
}

Result GameAccess::KnockDown(NetId id, float yawRad, float force) {
    if (!cfg_.directKnock) return Result::Unsupported;
    Actor* a = ActorOf(id);
    if (!a) return Result::Refused;
    void* proc = nullptr;
    uintptr_t fn = 0;
    if (!ResolveProcFn(a, P::kSlotPushActorAway, P::kFnPushActorAway, proc, fn)) return Result::Refused;
    RefPose rp;
    if (!ReadRefPose(a, rp)) return Result::Refused;
    // push from a point behind the actor (opposite its facing), so it falls forward; the unit of `force` is unmeasured
    const float sx = rp.px - std::sin(yawRad) * 50.0f, sy = rp.py - std::cos(yawRad) * 50.0f;
    return CallPushAway(fn, proc, a, sx, sy, rp.pz, force) ? Result::Ok : Result::Refused;
}

Result GameAccess::GetUp(NetId id) {
    if (!cfg_.directKnock) return Result::Unsupported;
    Actor* a = ActorOf(id);
    if (!a) return Result::Refused;
    void* proc = nullptr;
    uintptr_t fn = 0;
    if (!ResolveProcFn(a, P::kSlotSetKnockedState, P::kFnSetKnockedState, proc, fn)) return Result::Refused;
    return CallSetChar(fn, proc, 0) ? Result::Ok : Result::Refused;
}

std::string GameAccess::pinStatusJson() const {
    auto slotOk = [](uint32_t slot, uint32_t want) {
        uint32_t f = 0;
        return Rd(P::kHighProcessVtable + 4u * slot, f) && f == want;
    };
    uint8_t b[8] = {};
    const bool menu = SafeRead(P::kMenuModeAny, b, sizeof b) && std::memcmp(b, P::kMenuModeBytes, sizeof b) == 0;
    char buf[360];
    std::snprintf(buf, sizeof buf,
        "{\"get_sit_sleep_state\":%s,\"set_weapon_out\":%s,\"set_aiming\":%s,\"push_actor_away\":%s,\"set_knocked_state\":%s,\"menu_mode\":%s,"
        "\"direct_weapon_out\":%s,\"direct_aiming\":%s,\"direct_rate\":%s,\"direct_knock\":%s}",
        slotOk(P::kSlotGetSitSleepState, P::kFnGetSitSleepState) ? "true" : "false",
        slotOk(P::kSlotSetWeaponOut, P::kFnSetWeaponOut) ? "true" : "false", slotOk(P::kSlotSetIsAiming, P::kFnSetIsAiming) ? "true" : "false",
        slotOk(P::kSlotPushActorAway, P::kFnPushActorAway) ? "true" : "false", slotOk(P::kSlotSetKnockedState, P::kFnSetKnockedState) ? "true" : "false",
        menu ? "true" : "false", cfg_.directWeaponOut ? "true" : "false", cfg_.directAiming ? "true" : "false", cfg_.directRate ? "true" : "false",
        cfg_.directKnock ? "true" : "false");
    return buf;
}

}  // namespace nvcoop::actors::anim::game
