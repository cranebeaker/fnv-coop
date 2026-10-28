// IGameBackend on the running game. See game_backend_win.h. Compiled into nvcoop.dll only (x86, xNVSE headers, no NVSE .cpp needed).
//
// Rules this file follows (docs/tools/avatar.md "Engine facts"):
//   * addresses only from ../engine_pins.h (verified byte for byte by tools/avatar/verify_engine_pins.py) and checked again at run time;
//   * every raw read / call is inside a __try helper that touches only plain data (no C++ objects with destructors);
//   * a vtable slot is called only when the object's vtable pointer lies in .rdata and the slot's target lies in .text of the exe;
//   * console lines run with the hook layer told they are ours: the capture hub is muted (a host must not replicate puppet commands
//     as story effects) and the guest suppression gate is lifted (a guest must be allowed to Disable / AddItem its own puppets).
#include <Windows.h>

#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <string>
#include <vector>

#include "game_backend_win.h"

#include "nvse/GameAPI.h"
#include "nvse/GameForms.h"
#include "nvse/GameObjects.h"
#include "nvse/GameProcess.h"
#include "nvse/PluginAPI.h"

#include "../engine_pins.h"
#include "engine_hooks.h"

namespace nvcoop::avatar::engine {

namespace {

namespace P = ::nvcoop::avatar::pins;

// .text / .rdata of FalloutNV.exe 1.4.0.525 (reference image, tools/avatar/verify_engine_pins.py prints them)
constexpr uintptr_t kTextLo = 0x401000, kTextHi = 0xFDE600, kRdataLo = 0xFDF000, kRdataHi = 0x1183000;

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

// thiscall with no arguments through a vtable slot of `obj`; refuses any vtable / target outside the exe image
bool CallSlot(void* obj, uint32_t slot, uint32_t& ret) {
    __try {
        if (!obj) return false;
        uintptr_t vt = *reinterpret_cast<uintptr_t*>(obj);
        if (vt < kRdataLo || vt >= kRdataHi) return false;
        uintptr_t fn = *reinterpret_cast<uintptr_t*>(vt + 4u * slot);
        if (fn < kTextLo || fn >= kTextHi) return false;
        ret = reinterpret_cast<uint32_t(__thiscall*)(void*)>(fn)(obj);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

TESForm* LookupSEH(uint32_t id) {
    __try {
        return id ? reinterpret_cast<TESForm*(__cdecl*)(UInt32)>(P::LookupFormByID.addr)(id) : nullptr;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}

bool IsRefType(const TESForm* f) { return f && f->typeID >= kFormType_TESObjectREFR && f->typeID <= kFormType_Creature; }
bool IsActorType(const TESForm* f) { return f && (f->typeID == kFormType_Character || f->typeID == kFormType_Creature); }

uintptr_t PlayerPtr() {
    uintptr_t p = 0;
    Rd(P::kThePlayerPtr, p);
    return p;
}

// ---- raw readers (plain data in, plain data out) ---------------------------------------------------------------------------

struct RawRef {
    bool ok = false;
    uint8_t type = 0;
    uint32_t flags = 0, base = 0, cell = 0, node = 0;
    float pos[3] = {0, 0, 0}, rot[3] = {0, 0, 0};
    bool havePos3D = false;
    float pos3D[3] = {0, 0, 0};
};

void ReadRefRaw(TESObjectREFR* r, RawRef& o) {
    __try {
        o.type = r->typeID;
        o.flags = r->flags;
        o.pos[0] = r->posX;
        o.pos[1] = r->posY;
        o.pos[2] = r->posZ;
        o.rot[0] = r->rotX;
        o.rot[1] = r->rotY;
        o.rot[2] = r->rotZ;
        o.base = r->baseForm ? r->baseForm->refID : 0;
        o.cell = r->parentCell ? r->parentCell->refID : 0;
        uintptr_t rs = *reinterpret_cast<uintptr_t*>(reinterpret_cast<uintptr_t>(r) + P::kRefRenderState);
        if (rs) {
            uintptr_t node = *reinterpret_cast<uintptr_t*>(rs + P::kRenderStateNiNode);
            o.node = static_cast<uint32_t>(node);
            if (node) {
                float t[3];
                std::memcpy(t, reinterpret_cast<const void*>(node + P::kNiWorldTranslate), sizeof t);
                o.pos3D[0] = t[0];
                o.pos3D[1] = t[1];
                o.pos3D[2] = t[2];
                o.havePos3D = std::isfinite(t[0]) && std::isfinite(t[1]) && std::isfinite(t[2]);
            }
        }
        o.ok = true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        o.ok = false;
    }
}

bool IsDisabledSEH(TESObjectREFR* r, bool& out) {
    __try {
        out = reinterpret_cast<bool(__thiscall*)(void*)>(P::RefIsDisabled.addr)(r);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

struct RawAnim {
    bool ok = false, haveProc = false, haveMover = false, aiming = false, weaponOut = false, swimming = false, dead = false, knocked = false;
    int groups[9] = {-1, -1, -1, -1, -1, -1, -1, -1, -1};
    int nGroups = 0;
    int action = -1;
    uint32_t moverFlags = 0;
    bool firstPerson = false;
};

void ReadAnimRaw(Actor* a, bool isPlayer, RawAnim& o) {
    __try {
        const uintptr_t ap = reinterpret_cast<uintptr_t>(a);
        uint32_t life = 0;
        if (SafeRead(ap + P::kActorLifeState, &life, 4)) o.dead = (life == 1 || life == 2);
        uint8_t sw = 0;
        if (SafeRead(ap + P::kActorSwimming, &sw, 1)) o.swimming = sw != 0;
        uintptr_t proc = 0;
        SafeRead(ap + P::kActorBaseProcess, &proc, sizeof proc);
        if (proc) {
            void* p = reinterpret_cast<void*>(proc);
            uint32_t v = 0;
            o.haveProc = true;
            if (CallSlot(p, P::kProcGetAnimAction, v)) o.action = static_cast<int16_t>(v & 0xFFFF);
            if (CallSlot(p, P::kProcIsAiming, v)) o.aiming = (v & 0xFF) != 0;
            if (CallSlot(p, P::kProcIsWeaponOut, v)) o.weaponOut = (v & 0xFF) != 0;
            if (CallSlot(p, P::kProcGetKnocked, v)) o.knocked = v != 0;
            uint32_t animData = 0;
            if (CallSlot(p, P::kProcGetAnimData, animData) && animData) {
                for (int i = 0; i < 9; ++i) {
                    uintptr_t seq = 0;
                    const uintptr_t at = animData + (i < 8 ? P::kAnimDataSeqBase + 4u * i : P::kAnimDataSeq9);
                    if (!SafeRead(at, &seq, sizeof seq) || !seq) continue;
                    uintptr_t vt = 0;
                    if (!SafeRead(seq, &vt, sizeof vt) || vt != P::kVtblBSAnimGroupSequence) continue;
                    uintptr_t grp = 0;
                    uint8_t id = 0;
                    if (SafeRead(seq + P::kSeqAnimGroup, &grp, sizeof grp) && grp && SafeRead(grp + P::kAnimGroupId, &id, 1)) {
                        o.groups[i] = id;
                        ++o.nGroups;
                    }
                }
            }
        }
        uintptr_t mover = 0;
        if (SafeRead(ap + P::kActorMover, &mover, sizeof mover) && mover) {
            uintptr_t vt = 0;
            if (SafeRead(mover, &vt, sizeof vt) && (vt == P::kVtblActorMover || vt == P::kVtblPlayerMover)) {
                uint32_t flags = 0;
                if (CallSlot(reinterpret_cast<void*>(mover), 8, flags)) {
                    o.moverFlags = flags;
                    o.haveMover = true;
                }
            }
        }
        if (isPlayer) {
            uint8_t notFirst = 1;
            if (SafeRead(ap + P::kPlayerNotFirstPerson, &notFirst, 1)) o.firstPerson = notFirst == 0;
        }
        o.ok = o.haveProc;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        o.ok = false;
    }
}

// Worn items through the reference's ExtraContainerChanges (x86 layouts from xNVSE GameExtraData.h). Returns -1 when unreadable.
int ReadEquippedRaw(TESObjectREFR* r, EquippedItem* out, int max) {
    __try {
        const uintptr_t ref = reinterpret_cast<uintptr_t>(r);
        // extraDataList: vtbl @ +0x44, m_data @ +0x48 -> chain of BSExtraData { vtbl, type(u8) @ +4, next @ +8 }
        uintptr_t x = 0;
        if (!SafeRead(ref + P::kRefExtraDataList + 4, &x, sizeof x)) return -1;
        uintptr_t ecc = 0;
        for (int guard = 0; x && guard < 64; ++guard) {
            uint8_t type = 0;
            if (!SafeRead(x + 4, &type, 1)) return -1;
            if (type == P::kExtraContainerChanges) {
                ecc = x;
                break;
            }
            if (!SafeRead(x + 8, &x, sizeof x)) return -1;
        }
        if (!ecc) return 0;   // an actor without container changes wears nothing we know of
        uintptr_t data = 0;
        if (!SafeRead(ecc + 0x0C, &data, sizeof data) || !data) return 0;
        uintptr_t objList = 0;
        if (!SafeRead(data, &objList, sizeof objList) || !objList) return 0;
        int n = 0;
        // tList: the list object is its own first node { data, next }
        uintptr_t node = objList;
        for (int guard = 0; node && guard < 1024; ++guard) {
            uintptr_t entry = 0, next = 0;
            if (!SafeRead(node, &entry, sizeof entry) || !SafeRead(node + 4, &next, sizeof next)) return n;
            if (entry) {
                uintptr_t ext = 0, type = 0;
                SafeRead(entry, &ext, sizeof ext);
                SafeRead(entry + 8, &type, sizeof type);
                bool worn = false;
                uint8_t modFlags = 0;
                if (ext && type) {
                    uintptr_t en = ext;   // ExtendDataList: tList<ExtraDataList>
                    for (int g2 = 0; en && g2 < 64; ++g2) {
                        uintptr_t edl = 0, enext = 0;
                        if (!SafeRead(en, &edl, sizeof edl) || !SafeRead(en + 4, &enext, sizeof enext)) break;
                        if (edl) {
                            uintptr_t b = 0;   // BaseExtraList: vtbl @ 0, m_data @ +4
                            SafeRead(edl + 4, &b, sizeof b);
                            bool listWorn = false;
                            uint8_t listMods = 0;   // the modification flags live in the SAME instance (extra data list) that is worn
                            for (int g3 = 0; b && g3 < 64; ++g3) {
                                uint8_t t = 0;
                                if (!SafeRead(b + 4, &t, 1)) break;
                                if (t == P::kExtraWorn || t == P::kExtraWornLeft) listWorn = true;
                                if (t == P::kExtraWeaponModFlags && !SafeRead(b + P::kExtraWeaponModFlagsOffset, &listMods, 1)) listMods = 0;
                                if (!SafeRead(b + 8, &b, sizeof b)) break;
                            }
                            if (listWorn && !worn) {
                                worn = true;
                                modFlags = listMods;
                            }
                        }
                        en = enext;
                    }
                }
                if (worn && n < max) {
                    uint32_t form = 0;
                    uint8_t ftype = 0;
                    SafeRead(type + 0x0C, &form, sizeof form);
                    SafeRead(type + 4, &ftype, 1);
                    // a worn entry's extra data was walked: no ExtraWeaponModFlags in the worn instance means "no mods", which is knowledge
                    if (form) out[n++] = EquippedItem{form, ftype, static_cast<uint8_t>(modFlags & kWeaponModMask), true};
                }
            }
            node = next;
        }
        return n;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}

struct RawCell {
    bool ok = false, attached = false, interior = false;
    uint32_t world = 0;
};

void ReadCellRaw(TESObjectCELL* c, RawCell& o) {
    __try {
        o.attached = c->cellState == TESObjectCELL::kCellLoadState_Attached;
        o.interior = c->worldSpace == nullptr;
        o.world = c->worldSpace ? c->worldSpace->refID : 0;
        o.ok = true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        o.ok = false;
    }
}

int ListActorsRaw(TESObjectCELL* c, ActorEntry* out, int max) {
    int n = 0;
    __try {
        int guard = 0;
        for (auto* node = c->objectList.Head(); node && guard < 30000; node = node->next, ++guard) {
            TESObjectREFR* r = node->data;
            if (!r || !IsActorType(r)) continue;
            if (n < max) {
                out[n].ref = r->refID;
                out[n].base = r->baseForm ? r->baseForm->refID : 0;
                ++n;
            }
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
    return n;
}

// A placed non-actor reference with a loaded 3D if there is one (a position that is certainly in the cell), else any.
uint32_t AnchorRaw(TESObjectCELL* c, float* pos) {
    uint32_t any = 0, anyOk = 0;
    float anyPos[3] = {0, 0, 0};
    __try {
        int guard = 0;
        for (auto* node = c->objectList.Head(); node && guard < 30000; node = node->next, ++guard) {
            TESObjectREFR* r = node->data;
            if (!r || r->typeID != kFormType_TESObjectREFR || !r->baseForm || (r->flags & P::kFormFlagDeleted)) continue;
            if (!std::isfinite(r->posX) || !std::isfinite(r->posY) || !std::isfinite(r->posZ)) continue;
            if (!any) {
                any = r->refID;
                anyPos[0] = r->posX;
                anyPos[1] = r->posY;
                anyPos[2] = r->posZ;
            }
            if (r->renderState && r->renderState->niNode) {
                pos[0] = r->posX;
                pos[1] = r->posY;
                pos[2] = r->posZ;
                return r->refID;
            }
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
    (void)anyOk;
    if (any) std::memcpy(pos, anyPos, sizeof anyPos);
    return any;
}

// Cells of the player's worldspace that the engine currently holds attached (the grid around the player).
int LoadedCellsRaw(uint32_t* out, int max) {
    int n = 0;
    __try {
        PlayerCharacter* pc = reinterpret_cast<PlayerCharacter*>(PlayerPtr());
        if (!pc || !pc->parentCell) return 0;
        TESObjectCELL* pcell = pc->parentCell;
        out[n++] = pcell->refID;
        TESWorldSpace* ws = pcell->worldSpace;
        if (!ws || !ws->cellMap) return n;
        // NiTPointerMap<TESObjectCELL>: vtbl, numBuckets @ +4, buckets @ +8; Entry { next, key, data }
        const uintptr_t map = reinterpret_cast<uintptr_t>(ws->cellMap);
        uint32_t nb = 0;
        uintptr_t buckets = 0;
        if (!SafeRead(map + 4, &nb, 4) || !SafeRead(map + 8, &buckets, sizeof buckets) || !buckets || nb == 0 || nb > 4096) return n;
        int seen = 0;
        for (uint32_t b = 0; b < nb && seen < 4000; ++b) {
            uintptr_t e = 0;
            if (!SafeRead(buckets + 4u * b, &e, sizeof e)) continue;
            for (int g = 0; e && g < 64; ++g, ++seen) {
                uintptr_t next = 0, data = 0;
                if (!SafeRead(e, &next, sizeof next) || !SafeRead(e + 8, &data, sizeof data)) break;
                TESObjectCELL* c = reinterpret_cast<TESObjectCELL*>(data);
                if (c && c != pcell && c->typeID == kFormType_TESObjectCELL && c->cellState == TESObjectCELL::kCellLoadState_Attached && n < max)
                    out[n++] = c->refID;
                e = next;
            }
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
    return n;
}

struct RawAppearance {
    bool ok = false, female = false;
    uint32_t race = 0, hair = 0, eyes = 0, hairColor = 0;
    float height = 0, weight = 0;
    char name[48] = {};
};

void ReadAppearanceRaw(TESForm* base, RawAppearance& o) {
    __try {
        if (!base || base->typeID != kFormType_TESNPC) return;
        TESNPC* npc = static_cast<TESNPC*>(base);
        o.female = (npc->baseData.flags & TESActorBaseData::kFlags_Female) != 0;
        o.race = npc->race.race ? npc->race.race->refID : 0;
        o.hair = npc->hair ? npc->hair->refID : 0;
        o.eyes = npc->eyes ? npc->eyes->refID : 0;
        o.hairColor = npc->hairColor;
        o.height = npc->height;
        o.weight = npc->weight;
        const char* nm = npc->fullName.name.m_data;
        if (nm) {
            std::strncpy(o.name, nm, sizeof o.name - 1);
            o.name[sizeof o.name - 1] = 0;
        }
        o.ok = true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        o.ok = false;
    }
}

// FaceGen morph coefficients of an NPC base (TESNPC::faceGenData[3], xNVSE GameForms.h). The layout is NOT proven (the header calls `values` "an array
// of arrays" and `useOffset` "participates in the array base calculation"): two readings are tried per block, a reading counts only when every
// float is finite and plausible (|v| <= 16) and not all zero, and the numbers are quantised to 1/16. Whatever this returns is sent with the Look
// message and logged by the in-game proof; nothing applies it (no write path is known), so a wrong reading is harmless and visible.
int ReadFaceCoeffsRaw(TESForm* base, int8_t* out, int max) {
    if (!base || base->typeID != kFormType_TESNPC) return 0;
    TESNPC* npc = static_cast<TESNPC*>(base);
    int n = 0;
    for (int blk = 0; blk < 3; ++blk) {
        uintptr_t fg = reinterpret_cast<uintptr_t>(&npc->faceGenData[blk]);
        uint32_t values = 0, useOffset = 0, count = 0, size = 0;
        if (!Rd(fg + 0x0C, values) || !Rd(fg + 0x10, useOffset) || !Rd(fg + 0x18, count) || !Rd(fg + 0x1C, size)) continue;
        if (!values || count == 0 || count > 64 || size != 1 || n + static_cast<int>(count) > max) continue;
        uint32_t candidates[2] = {0, 0};
        Rd(values, candidates[0]);                                        // reading A: values[0] points at the floats
        candidates[1] = values + (useOffset <= 0x400 ? useOffset : 0);    // reading B: the floats start at values + useOffset
        for (int c = 0; c < 2; ++c) {
            if (!candidates[c]) continue;
            int8_t q[64] = {};
            bool ok = true, any = false;
            for (uint32_t i = 0; i < count && ok; ++i) {
                float f = 0;
                if (!Rd(candidates[c] + 4u * i, f) || !std::isfinite(f) || std::fabs(f) > 16.0f) {
                    ok = false;
                    break;
                }
                int v = static_cast<int>(std::lround(f * 16.0f));
                v = v < -127 ? -127 : (v > 127 ? 127 : v);
                q[i] = static_cast<int8_t>(v);
                any = any || v != 0;
            }
            if (ok && any) {
                for (uint32_t i = 0; i < count; ++i) out[n++] = q[i];
                break;
            }
        }
    }
    return n;
}

int ReadFaceCoeffsSEH(TESForm* base, int8_t* out, int max) {
    __try {
        return ReadFaceCoeffsRaw(base, out, max);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

// cdecl bool eval(thisObj, p1, p2, double* result)
bool EvalSEH(uint32_t addr, void* ref, void* p1, double* out) {
    __try {
        return reinterpret_cast<bool(__cdecl*)(void*, void*, void*, double*)>(addr)(ref, p1, nullptr, out);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool MenuModeSEH(bool& out) {
    __try {
        out = reinterpret_cast<bool(__cdecl*)()>(P::MenuModeAny.addr)();
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

int RunLineSEH(NVSEConsoleInterface* con, const char* line, TESObjectREFR* thisObj) {
    __try {
        return con->RunScriptLine2(line, thisObj, true) ? 1 : 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}

// Tells the hook layer that the console lines in scope are ours (see the file comment).
struct OursScope {
    nvhooks::CaptureHub* hub;
    nvhooks::ApplyScope apply;
    nvhooks::ApplierExecScope exec;
    OursScope() : hub(nvhooks::hub()) {
        if (hub) hub->pushMute();
    }
    ~OursScope() {
        if (hub) hub->popMute();
    }
};

uint32_t Fnv1a(uint32_t h, uint32_t v) {
    for (int i = 0; i < 4; ++i) {
        h ^= (v >> (8 * i)) & 0xFF;
        h *= 16777619u;
    }
    return h;
}

}  // namespace

// ---------------------------------------------------------------------------------------------------------------------------

WinBackend::WinBackend(NVSEConsoleInterface* console) : console_(console) {
    LARGE_INTEGER f;
    QueryPerformanceFrequency(&f);
    qpcFreq_ = f.QuadPart > 0 ? static_cast<uint64_t>(f.QuadPart) : 1;
    for (size_t i = 0; i < P::kPinCount && i < 16; ++i) {
        uint8_t live[16] = {};
        pinOk_[i] = SafeRead(P::kAllPins[i]->addr, live, 16) && std::memcmp(live, P::kAllPins[i]->bytes, 16) == 0;
    }
    lookupOk_ = PinOk("LookupFormByID");
    lastPlayerPtr_ = static_cast<uint32_t>(PlayerPtr());
}

bool WinBackend::PinOk(const char* name) const {
    for (size_t i = 0; i < P::kPinCount && i < 16; ++i)
        if (std::strcmp(P::kAllPins[i]->name, name) == 0) return pinOk_[i];
    return false;
}

std::string WinBackend::PinsJson() const {
    std::string s = "{\"consoleOk\":";
    s += console_ ? "true" : "false";
    s += ",\"pins\":[";
    for (size_t i = 0; i < P::kPinCount && i < 16; ++i) {
        char b[160];
        std::snprintf(b, sizeof b, "%s{\"name\":\"%s\",\"addr\":\"0x%08X\",\"ok\":%s}", i ? "," : "", P::kAllPins[i]->name,
                      (unsigned)P::kAllPins[i]->addr, pinOk_[i] ? "true" : "false");
        s += b;
    }
    s += "]}";
    return s;
}

uint32_t WinBackend::WorldEpoch() {
    // bumped by NVSE messages (NoteWorldChanged) and by the player object changing under us (a load that no message announced)
    uint32_t p = static_cast<uint32_t>(PlayerPtr());
    if (p != lastPlayerPtr_) {
        lastPlayerPtr_ = p;
        ++playerSwaps_;
    }
    return epochBase_ + playerSwaps_;
}

bool WinBackend::MenuOpen() {
    if (!PinOk("MenuModeAny")) return false;
    bool v = false;
    return MenuModeSEH(v) && v;
}

bool WinBackend::ReadCell(CellId cell, CellInfo& out) {
    out = CellInfo{};
    if (!lookupOk_) return false;
    TESForm* f = LookupSEH(cell);
    if (!f || f->typeID != kFormType_TESObjectCELL) return true;   // not a loaded cell: answered, exists = false
    RawCell rc;
    ReadCellRaw(static_cast<TESObjectCELL*>(f), rc);
    if (!rc.ok) return false;
    out.exists = true;
    out.attached = rc.attached;
    out.interior = rc.interior;
    out.world = rc.world;
    // the player's own cell is by definition live even while its state flag is mid-transition
    PlayerCharacter* pc = reinterpret_cast<PlayerCharacter*>(PlayerPtr());
    if (pc && static_cast<TESObjectCELL*>(f) == pc->parentCell) out.attached = true;
    return true;
}

RefId WinBackend::AnchorRef(CellId cell, Vec3& pos) {
    if (!lookupOk_) return 0;
    TESForm* f = LookupSEH(cell);
    if (!f || f->typeID != kFormType_TESObjectCELL) return 0;
    float p[3] = {0, 0, 0};
    uint32_t id = AnchorRaw(static_cast<TESObjectCELL*>(f), p);
    if (id) pos = Vec3{p[0], p[1], p[2]};
    return id;
}

void WinBackend::ListActors(CellId cell, std::vector<ActorEntry>& out) {
    out.clear();
    if (!lookupOk_) return;
    TESForm* f = LookupSEH(cell);
    if (!f || f->typeID != kFormType_TESObjectCELL) return;
    static ActorEntry buf[4096];
    int n = ListActorsRaw(static_cast<TESObjectCELL*>(f), buf, 4096);
    out.assign(buf, buf + n);
}

void WinBackend::LoadedCells(std::vector<CellId>& out) {
    out.clear();
    uint32_t cells[512];
    int n = LoadedCellsRaw(cells, 512);
    out.assign(cells, cells + n);
}

bool WinBackend::ReadRef(RefId ref, RefInfo& out) {
    out = RefInfo{};
    if (!lookupOk_) return false;
    TESForm* f = LookupSEH(ref);
    if (!IsRefType(f)) return true;   // not a loaded reference
    TESObjectREFR* r = static_cast<TESObjectREFR*>(f);
    RawRef rr;
    ReadRefRaw(r, rr);
    if (!rr.ok) return false;
    out.exists = true;
    out.isActor = rr.type == kFormType_Character || rr.type == kFormType_Creature;
    out.isPlayer = reinterpret_cast<uintptr_t>(r) == PlayerPtr();
    out.deleted = (rr.flags & P::kFormFlagDeleted) != 0;
    if (PinOk("TESObjectREFR::IsDisabled")) {
        bool d = false;
        if (IsDisabledSEH(r, d)) out.disabled = d;
    }
    out.base = rr.base;
    out.cell = rr.cell;
    out.pos = Vec3{rr.pos[0], rr.pos[1], rr.pos[2]};
    out.rot = Vec3{rr.rot[0], rr.rot[1], rr.rot[2]};
    out.has3D = rr.node != 0;
    out.node3D = rr.node;
    out.havePos3D = rr.havePos3D;
    out.pos3D = Vec3{rr.pos3D[0], rr.pos3D[1], rr.pos3D[2]};
    return true;
}

bool WinBackend::ReadAnim(RefId ref, AnimRead& out) {
    out = AnimRead{};
    if (!lookupOk_) return false;
    TESForm* f = LookupSEH(ref);
    if (!IsActorType(f)) return false;
    RawAnim ra;
    ReadAnimRaw(static_cast<Actor*>(f), reinterpret_cast<uintptr_t>(f) == PlayerPtr(), ra);
    if (!ra.ok) return false;
    out.valid = true;
    for (int i = 0; i < 9; ++i) out.groups[i] = ra.groups[i];
    out.nGroups = ra.nGroups;
    out.action = ra.action;
    out.aiming = ra.aiming;
    out.weaponOut = ra.weaponOut;
    out.haveMover = ra.haveMover;
    out.moverFlags = ra.moverFlags;
    out.swimming = ra.swimming;
    out.dead = ra.dead;
    out.knocked = ra.knocked;
    out.firstPerson = ra.firstPerson;
    return true;
}

bool WinBackend::ReadEquipped(RefId ref, std::vector<EquippedItem>& out) {
    out.clear();
    if (!lookupOk_) return false;
    TESForm* f = LookupSEH(ref);
    if (!IsActorType(f)) return false;
    EquippedItem buf[64];
    int n = ReadEquippedRaw(static_cast<TESObjectREFR*>(f), buf, 64);
    if (n < 0) return false;
    out.assign(buf, buf + n);
    return true;
}

bool WinBackend::ReadAppearance(RefId ref, Appearance& out) {
    if (!lookupOk_) return false;
    TESForm* f = LookupSEH(ref);
    if (!IsActorType(f)) return false;
    RawRef rr;
    ReadRefRaw(static_cast<TESObjectREFR*>(f), rr);
    if (!rr.ok || !rr.base) return false;
    TESForm* base = LookupSEH(rr.base);
    RawAppearance ra;
    ReadAppearanceRaw(base, ra);
    if (!ra.ok) return false;
    out.race = ra.race;
    out.female = ra.female;
    out.hair = ra.hair;
    out.eyes = ra.eyes;
    uint32_t h = 2166136261u;
    h = Fnv1a(h, ra.hairColor);
    h = Fnv1a(h, static_cast<uint32_t>(ra.height * 1000.0f));
    h = Fnv1a(h, static_cast<uint32_t>(ra.weight * 1000.0f));
    int8_t coeffs[kMaxFaceCoeffs] = {};
    const int nc = ReadFaceCoeffsSEH(base, coeffs, static_cast<int>(kMaxFaceCoeffs));
    out.faceCoeffs.assign(coeffs, coeffs + nc);
    for (int i = 0; i < nc; ++i) h = Fnv1a(h, static_cast<uint32_t>(static_cast<uint8_t>(coeffs[i])));
    out.faceHash = h;   // identity of hair colour + body + the face coefficients (TODO-RE: FaceGenData layout unproven, see ReadFaceCoeffsRaw)
    out.hairColor = ra.hairColor;
    out.height = static_cast<uint16_t>(ra.height > 0 && ra.height < 65.0f ? std::lround(ra.height * 1000.0f) : 0);
    out.weight = static_cast<uint16_t>(ra.weight > 0 && ra.weight < 65.0f ? std::lround(ra.weight * 1000.0f) : 0);
    out.name = ra.name;
    return true;
}

int WinBackend::RunLine(RefId ctx, const std::string& line) {
    if (!console_ || !lookupOk_) return -2;
    TESForm* f = LookupSEH(ctx);
    if (!IsRefType(f)) return -2;
    OursScope scope;
    return RunLineSEH(console_, line.c_str(), static_cast<TESObjectREFR*>(f));
}

bool WinBackend::Eval(Cond fn, RefId ref, FormId param, double& out) {
    out = 0;
    if (!lookupOk_) return false;
    const P::Pin* pin = nullptr;
    bool formParam = false, intParam = false;
    switch (fn) {
        case Cond::GetRestrained: pin = &P::EvalGetRestrained; break;
        case Cond::IsWeaponOut: pin = &P::EvalIsWeaponOut; break;
        case Cond::IsSneaking: pin = &P::EvalIsSneaking; break;
        case Cond::GetDead: pin = &P::EvalGetDead; break;
        case Cond::GetItemCount: pin = &P::EvalGetItemCount; formParam = true; break;
        case Cond::GetEquipped: pin = &P::EvalGetEquipped; formParam = true; break;
        case Cond::GetLevel: pin = &P::EvalGetLevel; break;
        case Cond::GetSitting: pin = &P::EvalGetSitting; break;
        case Cond::IsPlayerActionActive: pin = &P::EvalIsPlayerActionActive; intParam = true; break;
        case Cond::GetActorValue: pin = &P::EvalGetActorValue; intParam = true; break;
    }
    if (!pin || !PinOk(pin->name)) return false;
    TESForm* r = LookupSEH(ref);
    if (!IsRefType(r)) return false;
    void* arg = nullptr;
    if (formParam) {
        arg = LookupSEH(param);
        if (!arg) return false;
    } else if (intParam) {
        arg = reinterpret_cast<void*>(static_cast<uintptr_t>(param));   // IsPlayerActionActive takes the action id itself, not a form
    }
    double v = 0;
    if (!EvalSEH(pin->addr, r, arg, &v)) return false;
    out = v;
    return true;
}

// PlayerCharacter::regionsList: a BSSimpleList node chain {item, next} starting inline at player+0x764; every item is a TESRegion form (refID at +0x0C). Guarded reads only.
bool WinBackend::ReadPlayerRegions(std::vector<FormId>& out, size_t max) {
    out.clear();
    const uintptr_t player = PlayerPtr();
    if (!player) return false;
    uintptr_t node = player + P::kPlayerRegionList;
    for (int guard = 0; node && guard < 64 && out.size() < max; ++guard) {
        uint32_t cell[2] = {0, 0};   // {item, next}
        if (!SafeRead(node, cell, sizeof cell)) return false;
        if (cell[0]) {
            uint32_t id = 0;
            if (!SafeRead(cell[0] + P::kFormRefId, &id, sizeof id)) return false;
            if (id) out.push_back(id);
        }
        node = cell[1];
    }
    return true;
}

uint64_t WinBackend::NowUs() {
    LARGE_INTEGER t;
    QueryPerformanceCounter(&t);
    return static_cast<uint64_t>(t.QuadPart) * 1000000ull / qpcFreq_;
}

}  // namespace nvcoop::avatar::engine
