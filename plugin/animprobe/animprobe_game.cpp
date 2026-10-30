// T7 animprobe, engine side. Dev-only bridge methods, registered only when nvcoop.ini has [dev] animprobe=1:
//   anim.probe  {refs?, near?, bones?, max?, bone_names?, scan_animdata?}   -> per-actor animation / pose samples
//   anim.stats                                                              -> probe cost (microseconds)
//   anim.bones                                                              -> default bone name list
// Everything here is READ ONLY, runs on the main thread, and every raw memory access is SEH-guarded so a wrong offset
// yields a missing field, never a crash. Offsets and vtable slots were checked statically against the reference
// FalloutNV.exe 1.4.0.525 (tools/animcheck/verify_engine.py re-runs those checks); where the xNVSE header was wrong
// (NiAVObject world transform) we use raw offsets instead of the header struct.
#include <Windows.h>

#include <cmath>
#include <cstdint>
#include <cstring>
#include <string>
#include <vector>

#include "animprobe_core.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"

#include "nvse/GameAPI.h"
#include "nvse/GameForms.h"
#include "nvse/GameObjects.h"
#include "nvse/GameProcess.h"

namespace {

using animprobe::ActorSample;
using animprobe::SeqSample;
using animprobe::Vec3;

// ---- guarded raw access ---------------------------------------------------------------------------------------
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

// Calls vtable slot `slot` of `obj` with no arguments (thiscall). Returns false if anything faults.
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

// vtable slots, verified against the exe (see tools/animcheck/verify_engine.py)
constexpr uint32_t kProcGetWeaponInfo = 0x52, kProcGetAnimData = 0x6E, kProcGetCurrentPackage = 0x9F,
                   kProcGetAnimAction = 0xF9, kProcGetSequence = 0xFA, kProcIsAiming = 0x101,
                   kProcGetKnocked = 0x103, kProcIsWeaponOut = 0x115, kProcGetSitSleep = 0x12F;
constexpr uint32_t kRefGetNiNode = 0x74, kActorGetCombatTarget = 0x10B;
constexpr uintptr_t kVtblBSAnimGroupSequence = 0x0101DE3C, kVtblNiControllerSequence = 0x01096584,
                    kVtblActorMover = 0x0109223C, kVtblPlayerMover = 0x0109297C;

// Raw object offsets (static checks in verify_engine.py)
constexpr uintptr_t kActorBaseProcess = 0x68, kActorRagdoll = 0xAC, kActorLifeState = 0x108, kActorSwimming = 0x14D,
                    kActorMover = 0x190;
// NiAVObject / NiNode (the xNVSE struct is wrong here: world transform is rot@0x68, translate@0x8C, scale@0x98)
constexpr uintptr_t kNiName = 0x08, kNiWorldTranslate = 0x8C, kNiChildData = 0xA0, kNiChildFirstFree = 0xA6;
// BSAnimGroupSequence / NiControllerSequence (offsets from the xNVSE header; the vtable check below guards them)
constexpr uintptr_t kSeqBegin = 0x2C, kSeqEnd = 0x30, kSeqLast = 0x34, kSeqFreq = 0x28, kSeqWeight = 0x1C,
                    kSeqState = 0x44, kSeqAnimGroup = 0x74;
// Measured in game 2026-10-21 (world.peek on the player's playing sequences): the TESAnimGroup pointer is the last dword of the
// 0x78-byte BSAnimGroupSequence (+0x74; xNVSE's "//068" holds 0x0000FFFF here), and the group id is the dword at +0x10 of that object:
// low byte = group (mtidle.kf 0 Idle, mtfastforward.kf 7 FastForward, h2hequip.kf 0x18 Equip, h2haim.kf 0x11 Aim), next byte = weapon type.
constexpr uintptr_t kAnimGroupId = 0x10;
// AnimData::animSequence[8] at 0xE0, animSeq100 at 0x100
constexpr uintptr_t kAnimDataSeqBase = 0xE0, kAnimDataSize = 0x12C;

animprobe::CostStats g_cost;
uint64_t g_frame = 0;
LARGE_INTEGER g_qpcFreq, g_qpcStart;

double NowMs() {
    LARGE_INTEGER t;
    QueryPerformanceCounter(&t);
    return 1000.0 * static_cast<double>(t.QuadPart - g_qpcStart.QuadPart) / static_cast<double>(g_qpcFreq.QuadPart);
}

bool IsActorRef(TESObjectREFR* r) {
    return r && (r->typeID == kFormType_Character || r->typeID == kFormType_Creature);
}

// ---- skeleton -------------------------------------------------------------------------------------------------
struct BoneWant {
    std::string name;
    bool found = false;
    Vec3 pos;
};

// Depth first walk of the NiNode tree (children array: data@0xA0, count = firstFreeEntry@0xA6).
void WalkBones(uintptr_t node, std::vector<BoneWant>& want, int& remaining, int& visited, int depth) {
    if (!node || depth > 24 || visited > 700 || remaining <= 0) return;
    ++visited;
    uintptr_t namePtr = 0;
    if (Rd(node + kNiName, namePtr) && namePtr) {
        char buf[48] = {};
        if (SafeRead(namePtr, buf, sizeof buf - 1)) {
            for (BoneWant& w : want) {
                if (!w.found && w.name == buf) {
                    float t[3];
                    if (SafeRead(node + kNiWorldTranslate, t, sizeof t)) {
                        w.pos = {t[0], t[1], t[2]};
                        w.found = true;
                        --remaining;
                    }
                    break;
                }
            }
        }
    }
    uintptr_t data = 0;
    uint16_t count = 0;
    if (!Rd(node + kNiChildData, data) || !Rd(node + kNiChildFirstFree, count) || !data || count > 256) return;
    for (uint16_t i = 0; i < count && remaining > 0; ++i) {
        uintptr_t child = 0;
        if (!Rd(data + 4u * i, child)) break;
        WalkBones(child, want, remaining, visited, depth + 1);
    }
}

// ---- sequences ------------------------------------------------------------------------------------------------
bool g_rawPtrs = false;   // set per call from scan_animdata: also report the raw sequence / group pointers (dev)
bool ReadSeq(uintptr_t seq, int slot, SeqSample& s) {
    uintptr_t vt = 0;
    if (!seq || !Rd(seq, vt) || (vt != kVtblBSAnimGroupSequence && vt != kVtblNiControllerSequence)) return false;
    s.slot = slot;
    uint32_t state = 0;
    float f = 0;
    if (Rd(seq + kSeqState, state)) s.state = static_cast<int>(state);
    if (Rd(seq + kSeqLast, f)) s.t = f;
    if (Rd(seq + kSeqBegin, f)) s.begin = f;
    if (Rd(seq + kSeqEnd, f)) s.end = f;
    if (Rd(seq + kSeqFreq, f)) s.freq = f;
    if (Rd(seq + kSeqWeight, f)) s.weight = f;
    if (vt == kVtblBSAnimGroupSequence) {
        uintptr_t grp = 0;
        uint32_t id = 0;
        if (Rd(seq + kSeqAnimGroup, grp) && grp && Rd(grp + kAnimGroupId, id)) s.group = static_cast<int>(id & 0xFF);
        if (g_rawPtrs) s.grpPtr = static_cast<uint32_t>(grp);
    }
    if (g_rawPtrs) s.seqPtr = static_cast<uint32_t>(seq);
    return true;
}

// ---- one actor ------------------------------------------------------------------------------------------------
void Sample(Actor* a, bool wantBones, const std::vector<std::string>& boneNames, ActorSample& out) {
    const uintptr_t ap = reinterpret_cast<uintptr_t>(a);
    out.ref = a->refID;
    out.base = a->baseForm ? a->baseForm->refID : 0;
    out.isPlayer = (a == *g_thePlayer);
    out.pos = {a->posX, a->posY, a->posZ};
    out.rot = {a->rotX, a->rotY, a->rotZ};
    out.cell = a->parentCell ? a->parentCell->refID : 0;

    uint32_t life = 0;
    if (Rd(ap + kActorLifeState, life)) {
        out.lifeState = static_cast<int>(life);
        out.dead = (life == 1 || life == 2);  // Actor::IsDead (0x8844F0) treats 1 and 2 as dead/dying
    }
    uint8_t sw = 0;
    if (Rd(ap + kActorSwimming, sw)) out.swimming = sw != 0;
    uintptr_t rag = 0;
    if (Rd(ap + kActorRagdoll, rag)) out.ragdollCtrl = rag != 0;

    uintptr_t proc = 0;
    Rd(ap + kActorBaseProcess, proc);
    if (proc) {
        void* p = reinterpret_cast<void*>(proc);
        uint32_t v = 0;
        if (CallSlot(p, kProcGetAnimAction, v)) {
            out.action = static_cast<int16_t>(v & 0xFFFF);
            out.haveAnim = true;
        }
        if (CallSlot(p, kProcGetSequence, v)) out.sequenceRaw = v;
        if (CallSlot(p, kProcIsAiming, v)) out.aiming = (v & 0xFF) != 0;
        if (CallSlot(p, kProcIsWeaponOut, v)) out.weaponOut = (v & 0xFF) != 0;
        if (CallSlot(p, kProcGetKnocked, v)) out.knocked = static_cast<int>(v);
        if (CallSlot(p, kProcGetSitSleep, v)) out.sitSleep = static_cast<int>(v);
        if (CallSlot(p, kProcGetWeaponInfo, v) && v) {
            uint32_t form = 0;  // ExtraContainerChanges::EntryData { extendData; countDelta; type }: type at +8
            if (Rd(static_cast<uintptr_t>(v) + 0x08, form) && form) Rd(static_cast<uintptr_t>(form) + 0x0C, out.weaponForm);
        }
        if (CallSlot(p, kProcGetCurrentPackage, v) && v) Rd(static_cast<uintptr_t>(v) + 0x0C, out.package);

        uint32_t animData = 0;
        auto readSet = [](uintptr_t ad, std::vector<SeqSample>& into) {
            for (int i = 0; i < 9; ++i) {
                uintptr_t seq = 0;
                const uintptr_t at = ad + (i < 8 ? kAnimDataSeqBase + 4u * i : 0x100);
                if (Rd(at, seq) && seq) {
                    SeqSample s;
                    if (ReadSeq(seq, i, s)) into.push_back(s);
                }
            }
        };
        if (CallSlot(p, kProcGetAnimData, animData) && animData) readSet(animData, out.seqs);
        if (out.isPlayer) {   // PlayerCharacter::GetAnimData (0x950A10): [+0x6A0] if set, else (first person) [+0x690], else the process's set above
            uintptr_t fp = 0, alt = 0;
            if (Rd(ap + 0x690, fp) && fp && fp != animData) readSet(fp, out.fpSeqs);
            if (Rd(ap + 0x6A0, alt) && alt && alt != animData && alt != fp) readSet(alt, out.altSeqs);
        }
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

    // Skeleton root: always the reference's own 3D (refr+0x64 -> RenderState, niNode at +0x14; this is what
    // TESObjectREFR::GetNiNode 0x43FCD0 returns). For the player that is the THIRD-PERSON body and it keeps updating
    // in first person, whereas the virtual GetNiNode (slot 0x74) returns the first-person arms node (+0x694) when
    // PlayerCharacter+0x64A is 0. Comparing against a remote puppet must use the third-person body.
    uint32_t root = 0;
    uintptr_t rs = 0;
    if (Rd(ap + 0x64, rs) && rs) Rd(rs + 0x14, root);
    if (!root) CallSlot(a, kRefGetNiNode, root);
    out.valid3d = root != 0;
    if (out.isPlayer) {
        uint8_t notFirst = 1;  // PlayerCharacter::unk64A "= not FirstPerson"
        if (Rd(ap + 0x64A, notFirst)) out.firstPerson = notFirst == 0;
    }
    if (wantBones && root) {
        std::vector<BoneWant> want;
        if (boneNames.empty())
            for (int i = 0; i < animprobe::kDefaultBoneCount; ++i) want.push_back({animprobe::kDefaultBones[i]});
        else
            for (const std::string& n : boneNames) want.push_back({n});
        int remaining = static_cast<int>(want.size()), visited = 0;
        WalkBones(root, want, remaining, visited, 0);
        out.nodesVisited = visited;
        for (const BoneWant& w : want)
            if (w.found) out.bones.emplace_back(w.name, w.pos);
    }
}

// ---- bridge methods -------------------------------------------------------------------------------------------
bool MProbe(const std::string& params, std::string& out) {
    animprobe::Request req;
    std::string err;
    if (!animprobe::ParseRequest(params, req, err)) {
        out = "bad params: " + err;
        return false;
    }
    g_rawPtrs = req.scanAnimData;
    LARGE_INTEGER t0, t1;
    QueryPerformanceCounter(&t0);

    PlayerCharacter* pc = g_thePlayer ? *g_thePlayer : nullptr;
    std::vector<Actor*> actors;
    auto addUnique = [&](Actor* a) {
        for (Actor* x : actors)
            if (x == a) return;
        actors.push_back(a);
    };
    if (req.includePlayer && pc) addUnique(pc);
    std::vector<uint32_t> skipped;
    for (uint32_t id : req.refs) {
        TESForm* f = LookupFormByID(id);
        TESObjectREFR* r = (f && f->typeID >= kFormType_TESObjectREFR && f->typeID <= kFormType_Creature) ? static_cast<TESObjectREFR*>(f) : nullptr;
        if (IsActorRef(r)) addUnique(static_cast<Actor*>(r));
        else skipped.push_back(id);
    }
    if (req.nearby && pc && pc->parentCell) {
        std::vector<Actor*> cand;
        std::vector<float> d2;
        int guard = 0;
        for (auto* n = pc->parentCell->objectList.Head(); n && guard < 20000; n = n->next, ++guard) {
            TESObjectREFR* r = n->data;
            if (!IsActorRef(r) || r == pc) continue;
            const float dx = r->posX - pc->posX, dy = r->posY - pc->posY, dz = r->posZ - pc->posZ;
            cand.push_back(static_cast<Actor*>(r));
            d2.push_back(dx * dx + dy * dy + dz * dz);
        }
        for (int i : animprobe::SelectNearest(d2, req.nearRadius, req.maxActors)) addUnique(cand[i]);
    }
    if (static_cast<int>(actors.size()) > req.maxActors) actors.resize(static_cast<size_t>(req.maxActors));

    nvcoop::json::Value res = nvcoop::json::Value::object();
    nvcoop::json::Value arr = nvcoop::json::Value::array();
    for (Actor* a : actors) {
        ActorSample s;
        Sample(a, req.bones, req.boneNames, s);
        arr.push(animprobe::ToJson(s));
    }
    if (req.scanAnimData && !actors.empty()) {
        // dev aid: which dwords of the first actor's AnimData point at a sequence object
        nvcoop::json::Value scan = nvcoop::json::Value::array();
        uintptr_t proc = 0;
        uint32_t ad = 0;
        if (Rd(reinterpret_cast<uintptr_t>(actors[0]) + kActorBaseProcess, proc) && proc &&
            CallSlot(reinterpret_cast<void*>(proc), kProcGetAnimData, ad) && ad) {
            for (uintptr_t off = 0; off + 4 <= kAnimDataSize; off += 4) {
                uintptr_t p = 0, vt = 0;
                if (Rd(ad + off, p) && p && Rd(p, vt) && (vt == kVtblBSAnimGroupSequence || vt == kVtblNiControllerSequence))
                    scan.push(static_cast<int64_t>(off));
            }
        }
        res.set("animdata_seq_offsets", scan);
    }
    QueryPerformanceCounter(&t1);
    const double us = 1e6 * static_cast<double>(t1.QuadPart - t0.QuadPart) / static_cast<double>(g_qpcFreq.QuadPart);
    g_cost.Add(us, static_cast<int>(actors.size()));

    res.set("v", animprobe::kSchemaVersion).set("frame", static_cast<int64_t>(g_frame)).set("t_ms", NowMs()).set("cost_us", us);
    res.set("cell", animprobe::HexId(pc && pc->parentCell ? pc->parentCell->refID : 0));
    res.set("actors", arr);
    nvcoop::json::Value sk = nvcoop::json::Value::array();
    for (uint32_t id : skipped) sk.push(animprobe::HexId(id));
    res.set("skipped", sk);
    out = res.dump();
    return true;
}

bool MStats(const std::string&, std::string& out) {
    out = g_cost.ToJson().dump();
    return true;
}

bool MBones(const std::string&, std::string& out) {
    nvcoop::json::Value a = nvcoop::json::Value::array();
    for (int i = 0; i < animprobe::kDefaultBoneCount; ++i) a.push(animprobe::kDefaultBones[i]);
    nvcoop::json::Value o = nvcoop::json::Value::object();
    o.set("bones", a);
    out = o.dump();
    return true;
}

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                       reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

void OnLoad(void*) {
    if (GetPrivateProfileIntA("dev", "animprobe", 0, IniPath().c_str()) == 0) return;
    QueryPerformanceFrequency(&g_qpcFreq);
    QueryPerformanceCounter(&g_qpcStart);
    nvcoop::RegisterMethod("anim.probe", MProbe, true);
    nvcoop::RegisterMethod("anim.stats", MStats, true);
    nvcoop::RegisterMethod("anim.bones", MBones, true);
    nvcoop::Log("animprobe: enabled ([dev] animprobe=1): anim.probe anim.stats anim.bones");
}

void OnFrame() { ++g_frame; }

}  // namespace

NVCOOP_MODULE(t7_animprobe, "t7_animprobe", OnLoad, nullptr, OnFrame);
