// RealEngineHost (see engine_real.h). x86 only; compiled against the xNVSE headers.
#include <Windows.h>

#include <algorithm>
#include <cstdint>
#include <cstring>
#include <string>
#include <vector>

#include "actors/engine/av_read.h"
#include "actors/engine/engine_real.h"
#include "common/nvcoop_api.h"

#include "nvse/GameAPI.h"
#include "nvse/GameForms.h"
#include "nvse/GameObjects.h"
#include "nvse/GameProcess.h"

#include "engine_addrs.h"   // plugin/hooks: pinned prologues (Actor::GetActorValue, TESObjectREFR::IsDisabled)
#include "engine_hooks.h"   // plugin/hooks: FormResolver, ApplyScope, console executor
#include "world_addrs.h"    // plugin/hooks_world: Call_PlaceAtMe
#include "world_hooks.h"    // plugin/hooks_world: queryCondition (GetRestrained)

namespace nvcoop::actors::engine {

namespace {

// ---- guarded raw access (same helpers and the same statically checked offsets as plugin/animprobe/animprobe_game.cpp) ----------------------
bool SafeRead(uintptr_t addr, void* out, size_t n) {
    __try {
        std::memcpy(out, reinterpret_cast<const void*>(addr), n);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
template <class T>
bool Rd(uintptr_t addr, T& out) {
    return SafeRead(addr, &out, sizeof(T));
}

typedef uint32_t(__thiscall* Fn0)(void*);
bool CallSlot(void* obj, uint32_t slot, uint32_t& ret) {
    __try {
        uintptr_t vt = *reinterpret_cast<uintptr_t*>(obj);
        Fn0 f = *reinterpret_cast<Fn0*>(vt + 4 * slot);
        ret = f(obj);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// BaseProcess vtable slots and Actor vtable slot (animprobe: verified against a reference exe)
constexpr uint32_t kProcGetCurrentPackage = 0x9F, kProcIsAiming = 0x101, kProcIsWeaponOut = 0x115;
constexpr uint32_t kActorGetCombatTarget = 0x10B;
constexpr uintptr_t kActorBaseProcess = 0x68, kActorLifeState = 0x108, kActorSwimming = 0x14D, kActorMover = 0x190;
constexpr uintptr_t kVtblActorMover = 0x0109223C, kVtblPlayerMover = 0x0109297C;
// ACBS flags of the base form (xNVSE TESActorBaseData): bit 1 essential. Bit 5 is "unique" for NPCs (FNVEdit; xNVSE names the creature meaning), TODO-RE: not checked in the exe.
constexpr uint32_t kAcbsEssential = 0x2, kAcbsUniqueNpc = 0x20;
constexpr UInt32 kRefDeleted = 0x20;   // TESObjectREFR::kFlags_Deleted

TESForm* LookupSEH(uint32_t id) {
    if (!id) return nullptr;
    __try {
        return LookupFormByID(id);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}

Actor* ActorSEH(uint32_t id) {
    TESForm* f = LookupSEH(id);
    if (!f) return nullptr;
    if (f->typeID != kFormType_Character && f->typeID != kFormType_Creature) return nullptr;
    return static_cast<Actor*>(f);
}

bool FillRawSEH(Actor* a, ActorRaw& out) {
    __try {
        const uintptr_t ap = reinterpret_cast<uintptr_t>(a);
        out.valid = true;
        out.ref = a->refID;
        out.isPlayer = g_thePlayer && a == *g_thePlayer;
        out.deleted = (a->flags & kRefDeleted) != 0;
        out.baseId = a->baseForm ? a->baseForm->refID : 0;
        out.cell = a->parentCell ? a->parentCell->refID : 0;
        out.pos = {a->posX, a->posY, a->posZ};
        out.rot = {a->rotX, a->rotY, a->rotZ};
        uint32_t life = 0;
        if (Rd(ap + kActorLifeState, life)) out.lifeState = life;
        uint8_t sw = 0;
        if (Rd(ap + kActorSwimming, sw)) out.swimming = sw != 0;
        uintptr_t rs = 0, node = 0;
        if (Rd(ap + 0x64, rs) && rs && Rd(rs + 0x14, node)) out.has3D = node != 0;

        TESForm* base = a->baseForm;
        if (base && (base->typeID == kFormType_TESNPC || base->typeID == kFormType_TESCreature)) {
            const UInt32 fl = static_cast<TESActorBase*>(base)->baseData.flags;
            out.baseEssential = (fl & kAcbsEssential) != 0;
            out.baseUnique = base->typeID == kFormType_TESNPC && (fl & kAcbsUniqueNpc) != 0;
        }

        uintptr_t proc = 0;
        Rd(ap + kActorBaseProcess, proc);
        if (proc) {
            void* p = reinterpret_cast<void*>(proc);
            uint32_t v = 0;
            out.haveProc = true;
            if (CallSlot(p, kProcIsAiming, v)) out.aiming = (v & 0xFF) != 0;
            if (CallSlot(p, kProcIsWeaponOut, v)) out.weaponOut = (v & 0xFF) != 0;
            if (CallSlot(p, kProcGetCurrentPackage, v) && v) Rd(static_cast<uintptr_t>(v) + 0x0C, out.package);
        }
        uintptr_t mover = 0;
        if (Rd(ap + kActorMover, mover) && mover) {
            uintptr_t vt = 0;
            if (Rd(mover, vt) && (vt == kVtblActorMover || vt == kVtblPlayerMover)) {
                uint32_t flags = 0;
                if (CallSlot(reinterpret_cast<void*>(mover), 8, flags)) {
                    out.moveFlags = flags;
                    out.haveMove = true;
                }
            }
        }
        uint32_t tgt = 0;
        if (CallSlot(a, kActorGetCombatTarget, tgt) && tgt) {
            out.inCombat = true;
            Rd(static_cast<uintptr_t>(tgt) + 0x0C, out.combatTarget);
        }
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool PoseSEH(Actor* a, CellId* cell, live::Vec3* pos, float* rotZ) {
    __try {
        *cell = a->parentCell ? a->parentCell->refID : 0;
        *pos = {a->posX, a->posY, a->posZ};
        *rotZ = a->rotZ;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool IsDisabledSEH(void* ref, bool& out) {
    typedef bool(__thiscall * Fn)(void*);
    __try {
        out = reinterpret_cast<Fn>(0x565090)(ref);   // TESObjectREFR::IsDisabled, prologue pinned in checkPins()
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool GetAvSEH(Actor* a, uint32_t av, float& out) {
    typedef float(__thiscall * Fn)(void*, uint32_t);
    __try {
        out = reinterpret_cast<Fn>(0x66EF50)(reinterpret_cast<char*>(a) + 0xA4, av);   // Actor::GetActorValue(this = avOwner), pinned in checkPins()
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

void WalkActorsSEH(const tList<Actor>* list, RefId* buf, size_t cap, size_t* n) {
    __try {
        int guard = 0;
        for (auto* node = list->Head(); node && guard < 6000; node = node->next, ++guard) {
            Actor* a = node->data;
            if (!a) continue;
            if (a->typeID != kFormType_Character && a->typeID != kFormType_Creature) continue;
            if (*n < cap) buf[(*n)++] = a->refID;
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
}

void WalkCellActorsSEH(TESObjectCELL* cell, RefId* buf, size_t cap, size_t* n) {
    __try {
        int guard = 0;
        for (auto* node = cell->objectList.Head(); node && guard < 20000; node = node->next, ++guard) {
            TESObjectREFR* r = node->data;
            if (!r) continue;
            if (r->typeID != kFormType_Character && r->typeID != kFormType_Creature) continue;
            if (*n < cap) buf[(*n)++] = r->refID;
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
}

RefId FirstRefInCellSEH(TESObjectCELL* cell, live::Vec3* pos) {
    __try {
        int guard = 0;
        for (auto* node = cell->objectList.Head(); node && guard < 20000; node = node->next, ++guard) {
            TESObjectREFR* r = node->data;
            if (!r || !r->refID || (r->flags & kRefDeleted)) continue;
            if (r->typeID < kFormType_TESObjectREFR || r->typeID > kFormType_Creature) continue;
            *pos = {r->posX, r->posY, r->posZ};
            return r->refID;
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
    return 0;
}

bool PlayerCellSEH(RefId* cell, live::Vec3* pos) {
    __try {
        PlayerCharacter* pc = g_thePlayer ? *g_thePlayer : nullptr;
        if (!pc || !pc->parentCell) return false;
        *cell = pc->parentCell->refID;
        *pos = {pc->posX, pc->posY, pc->posZ};
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

TESObjectCELL* PlayerCellPtrSEH() {
    __try {
        PlayerCharacter* pc = g_thePlayer ? *g_thePlayer : nullptr;
        return pc ? pc->parentCell : nullptr;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}

bool CellInfoSEH(TESObjectCELL* c, uint8_t* state, CellInfo* out) {
    __try {
        *state = c->cellState;
        out->known = true;
        out->exterior = c->worldSpace != nullptr;
        if (c->worldSpace) {
            out->world = c->worldSpace->refID;
            if (c->cellData) {
                out->x = static_cast<int32_t>(c->cellData->x);
                out->y = static_cast<int32_t>(c->cellData->y);
            }
        }
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

TESObjectREFR* PlaceSEH(TESObjectREFR* anchor, TESForm* base) {
    typedef TESObjectREFR*(__cdecl * Fn)(TESObjectREFR*, TESForm*, UInt32, UInt32, UInt32, float);
    __try {
        return reinterpret_cast<Fn>(world_addrs::Call_PlaceAtMe.addr)(anchor, base, 1, 0, 0, 1.0f);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}

}  // namespace

RealEngineHost::RealEngineHost() = default;

// One reading of one pin. A function another plugin (or our own hook lane) detoured starts with a jump and still forwards to the original: callable.
// A refused pin logs the bytes it found, once per distinct reading (pin_check.h has the reasons the same exe can answer differently).
static bool CheckPin(const char* name, uintptr_t addr, const unsigned char* expected, std::string& lastSeen) {
    unsigned char b[16] = {};
    const bool readable = SafeRead(addr, b, 16);
    const PinState st = classifyPin(readable, b, expected);
    if (!pinOk(st)) {
        const std::string line = pinRefusal(name, addr, st, b, expected);
        if (line != lastSeen) {
            lastSeen = line;
            nvcoop::Log("actors: %s", line.c_str());
        }
    } else if (st == PinState::Detoured && lastSeen != "detoured") {
        lastSeen = "detoured";
        nvcoop::Log("actors: pin %s at %08X is detoured by another patch (starts with E9); calling through it", name, static_cast<unsigned>(addr));
    }
    return pinOk(st);
}

void RealEngineHost::checkPins() {
    static std::string seenPlace, seenAv, seenDis;
    const Pins before = pins_;
    pins_.placeAtMe = CheckPin("Call_PlaceAtMe", world_addrs::Call_PlaceAtMe.addr, world_addrs::Call_PlaceAtMe.bytes, seenPlace);
    pins_.getActorValue = CheckPin("Actor::GetActorValue", nvhooks::addr::ActorGetActorValue.target, nvhooks::addr::ActorGetActorValue.expected, seenAv);
    pins_.isDisabled = CheckPin("TESObjectREFR::IsDisabled", nvhooks::addr::RefIsDisabled.target, nvhooks::addr::RefIsDisabled.expected, seenDis);
    const bool all = pins_.placeAtMe && pins_.getActorValue && pins_.isDisabled;
    pinPolicy_.record(all, GetTickCount64());
    if (pinPolicy_.checks() == 1 || all || pinPolicy_.final() || before.placeAtMe != pins_.placeAtMe || before.getActorValue != pins_.getActorValue ||
        before.isDisabled != pins_.isDisabled)
        nvcoop::Log("actors: engine pins placeAtMe=%d getActorValue=%d isDisabled=%d (check %d%s)", pins_.placeAtMe, pins_.getActorValue, pins_.isDisabled,
                    pinPolicy_.checks(), pinPolicy_.final() ? ", final" : ", will look again");
}

void RealEngineHost::ensurePins() {
    if (pinPolicy_.due(GetTickCount64())) checkPins();
}

bool RealEngineHost::worldReady() {
    if (loading_) return false;
    RefId cell = 0;
    live::Vec3 p;
    return PlayerCellSEH(&cell, &p);
}

bool RealEngineHost::read(RefId ref, ActorRaw& out) {
    const bool wantRestrained = out.wantRestrained;
    out = ActorRaw{};
    ensurePins();
    Actor* a = ActorSEH(ref);
    if (!a) return false;
    if (!FillRawSEH(a, out)) {
        out = ActorRaw{};
        ++sehFaults_;
        return false;
    }
    if (pins_.isDisabled) {
        bool d = false;
        if (IsDisabledSEH(a, d)) out.disabled = d;
    }
    if (wantRestrained) {
        double v = 0;
        if (nvworld::queryCondition("GetRestrained", ref, 0, &v)) {
            out.restrainedKnown = true;
            out.restrained = v != 0;
        }
    }
    return true;
}

bool RealEngineHost::readPose(RefId ref, CellId& cell, live::Vec3& pos, float& rotZ) {
    Actor* a = ActorSEH(ref);
    return a && PoseSEH(a, &cell, &pos, &rotZ);
}

bool RealEngineHost::actorValue(RefId ref, uint8_t av, float& out) {
    ensurePins();
    Actor* a = ActorSEH(ref);
    if (!a) return false;
    AvReads r;
    double v = 0;
    if (nvworld::queryCondition("GetActorValue", ref, av, &v)) {   // the console's own path: includes the damage of an actor that is not loaded
        r.evalOk = true;
        r.eval = static_cast<float>(v);
    }
    if (pins_.getActorValue) {
        if (GetAvSEH(a, av, r.direct)) r.directOk = true;
        else ++sehFaults_;
    }
    return pickActorValue(r, out);
}

void RealEngineHost::loadedActors(std::vector<RefId>& out, size_t max) {
    out.clear();
    static RefId buf[8192];
    size_t n = 0;
    const size_t cap = std::min<size_t>(max ? max : 8192, 8192);
    if (g_actorProcessManager) {
        WalkActorsSEH(&g_actorProcessManager->highActors, buf, cap, &n);
        WalkActorsSEH(&g_actorProcessManager->middleHighActors.head, buf, cap, &n);
    }
    // the player's own cell also holds the actors that are in low process
    TESObjectCELL* cell = PlayerCellPtrSEH();
    if (cell) WalkCellActorsSEH(cell, buf, cap, &n);
    std::sort(buf, buf + n);
    n = static_cast<size_t>(std::unique(buf, buf + n) - buf);
    out.assign(buf, buf + n);
}

bool RealEngineHost::cellLoaded(CellId cell) {
    RefId pcell = 0;
    live::Vec3 p;
    if (PlayerCellSEH(&pcell, &p) && pcell == cell) return true;
    TESForm* f = LookupSEH(cell);
    if (!f || f->typeID != kFormType_TESObjectCELL) return false;
    uint8_t state = 0;
    CellInfo ci;
    return CellInfoSEH(static_cast<TESObjectCELL*>(f), &state, &ci) && state == TESObjectCELL::kCellLoadState_Attached;
}

bool RealEngineHost::cellInfo(CellId cell, CellInfo& out) {
    out = CellInfo{};
    TESForm* f = LookupSEH(cell);
    if (!f || f->typeID != kFormType_TESObjectCELL) return false;
    uint8_t state = 0;
    return CellInfoSEH(static_cast<TESObjectCELL*>(f), &state, &out);
}

bool RealEngineHost::toFormRef(uint32_t formId, fnvcoop::FormRef& out) {
    if ((formId >> 24) == 0xFF) return false;
    return nvhooks::resolver().toRef(formId, &out);
}

uint32_t RealEngineHost::fromFormRef(const fnvcoop::FormRef& r) { return nvhooks::resolver().fromRef(r); }

RefId RealEngineHost::anchorInCell(CellId cell, live::Vec3& p) {
    RefId pcell = 0;
    live::Vec3 ppos;
    if (PlayerCellSEH(&pcell, &ppos) && pcell == cell) {
        p = ppos;
        return kLocalPlayerRef;
    }
    TESForm* f = LookupSEH(cell);
    if (!f || f->typeID != kFormType_TESObjectCELL) return 0;
    TESObjectCELL* c = static_cast<TESObjectCELL*>(f);
    uint8_t state = 0;
    CellInfo ci;
    if (!CellInfoSEH(c, &state, &ci) || state != TESObjectCELL::kCellLoadState_Attached) return 0;
    return FirstRefInCellSEH(c, &p);
}

RefId RealEngineHost::placeAtMe(RefId anchor, uint32_t baseId) {
    ensurePins();
    if (!pins_.placeAtMe) return 0;
    TESForm* af = LookupSEH(anchor);
    TESForm* base = LookupSEH(baseId);
    if (!af || !base) return 0;
    if (af->typeID < kFormType_TESObjectREFR || af->typeID > kFormType_Creature) return 0;
    TESObjectREFR* made = PlaceSEH(static_cast<TESObjectREFR*>(af), base);
    if (!made) {
        ++sehFaults_;
        return 0;
    }
    return made->refID;
}

bool RealEngineHost::console(const std::string& line, RefId on) {
    nvhooks::ApplyScope apply;               // a guest must be allowed to run it: these are the sync layer's own lines, not a script's
    nvhooks::ApplierExecScope exec;          // and they are not a leaked result script
    return nvhooks::execConsoleLineOn(line, on);
}

bool RealEngineHost::killWithAttribution(RefId victim, RefId killer) {
    return nvhooks::killWithAttribution(victim, killer);   // runs `KillActor <killer>` on the victim; the engine stores the killer before it raises OnDeath
}

}  // namespace nvcoop::actors::engine
