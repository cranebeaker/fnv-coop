// Read-only parity probe for the lab's parity room (tools/lab/lab_parity.py, docs/LAB.md). Dev-only bridge method, registered only when
// nvcoop.ini has [dev] parityprobe=1 (the lab's ini switches it on):
//
//   parity.look {refs: ["player" | "0xREF" | "REF", ...]}
//     -> {v, player, time:{current, target, ok}, vatsMode, pausedByMenu,
//         actors:[{ref, ok, actor, creature, base, race, female, hair, eyes, name, worn:[{form, type, mods}], wornKnown}]}
//
// What it gives the lab that anim.probe / actors.read / avatar.* do not: the full worn list of ANY actor (a puppet, an NPC copy), the look of its base
// form (race, sex, hair, eyes), and the global time multiplier of the machine (0x11AC3A0 current, 0x11AC3A4 target) with the VATS camera mode, so a
// time multiplier that is not 1.0 can be told from VATS slow motion. Animation group, weapon out and sit state are NOT here: anim.probe has them.
//
// Rules (same as plugin/avatar/engine/win/game_backend_win.cpp, whose reads this copies):
//   * READ ONLY. Nothing is written, no engine function is called except LookupFormByID, and that only after its pinned prologue matched.
//   * Every raw memory access is inside a __try helper that touches plain data only; an access violation yields a missing field, never a crash.
//   * Addresses come from plugin/avatar/engine/engine_pins.h and plugin/vats/vats_addrs.h (both documented there). None is new.
#include <Windows.h>

#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>

#include "avatar/engine/engine_pins.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "vats/vats_addrs.h"

#include "nvse/GameAPI.h"
#include "nvse/GameForms.h"
#include "nvse/GameObjects.h"
#include "nvse/GameProcess.h"

namespace {

namespace P = ::nvcoop::avatar::pins;
namespace A = ::nvcoop::vats::addr;
using nvcoop::json::Value;

bool SafeRead(uintptr_t addr, void* out, size_t n) {
    __try {
        std::memcpy(out, reinterpret_cast<const void*>(addr), n);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// The pinned function is called only when the live bytes are the recorded prologue (once per process; -1 = mismatch, never called).
int g_lookupPin = 0;
bool LookupPinOk() {
    if (g_lookupPin == 0) {
        uint8_t live[16] = {};
        g_lookupPin = SafeRead(P::LookupFormByID.addr, live, sizeof live) && std::memcmp(live, P::LookupFormByID.bytes, sizeof live) == 0 ? 1 : -1;
    }
    return g_lookupPin > 0;
}

TESForm* LookupSEH(uint32_t id) {
    __try {
        return reinterpret_cast<TESForm*(__cdecl*)(UInt32)>(P::LookupFormByID.addr)(id);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}

std::string Hex(uint32_t v) {
    char b[16];
    std::snprintf(b, sizeof b, "0x%08X", static_cast<unsigned>(v));
    return b;
}

// ---- plain-data readers ----------------------------------------------------------------------------------------------------------

struct Worn {
    uint32_t form = 0;
    uint8_t type = 0, mods = 0;
};
constexpr int kMaxWorn = 48;

// Worn items through the reference's ExtraContainerChanges (x86 layouts from xNVSE GameExtraData.h; the same walk as the avatar backend). -1 = unreadable.
int ReadWornRaw(uintptr_t ref, Worn* out, int max) {
    __try {
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
        if (!ecc) return 0;
        uintptr_t data = 0;
        if (!SafeRead(ecc + 0x0C, &data, sizeof data) || !data) return 0;
        uintptr_t objList = 0;
        if (!SafeRead(data, &objList, sizeof objList) || !objList) return 0;
        int n = 0;
        uintptr_t node = objList;   // tList: the list object is its own first node { data, next }
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
                            uint8_t listMods = 0;
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
                    SafeRead(type + P::kFormRefId, &form, sizeof form);
                    SafeRead(type + 4, &ftype, 1);
                    if (form) {
                        out[n].form = form;
                        out[n].type = ftype;
                        out[n].mods = modFlags;
                        ++n;
                    }
                }
            }
            node = next;
        }
        return n;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}

struct RawLook {
    bool ok = false, actor = false, creature = false, haveBase = false, female = false;
    uint32_t refId = 0, base = 0, race = 0, hair = 0, eyes = 0;
    char name[48] = {};
    Worn worn[kMaxWorn];
    int nWorn = -1;
};

void ReadLookRaw(TESForm* f, RawLook& o) {
    __try {
        if (!f || f->typeID < kFormType_TESObjectREFR || f->typeID > kFormType_Creature) return;
        TESObjectREFR* r = static_cast<TESObjectREFR*>(f);
        o.refId = r->refID;
        o.actor = r->typeID == kFormType_Character || r->typeID == kFormType_Creature;
        o.creature = r->typeID == kFormType_Creature;
        TESForm* base = r->baseForm;
        o.base = base ? base->refID : 0;
        if (base && base->typeID == kFormType_TESNPC) {
            TESNPC* npc = static_cast<TESNPC*>(base);
            o.haveBase = true;
            o.female = (npc->baseData.flags & TESActorBaseData::kFlags_Female) != 0;
            o.race = npc->race.race ? npc->race.race->refID : 0;
            o.hair = npc->hair ? npc->hair->refID : 0;
            o.eyes = npc->eyes ? npc->eyes->refID : 0;
            const char* nm = npc->fullName.name.m_data;
            if (nm) {
                std::strncpy(o.name, nm, sizeof o.name - 1);
                o.name[sizeof o.name - 1] = 0;
            }
        }
        o.ok = true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        o.ok = false;
    }
    if (o.ok && o.actor) o.nWorn = ReadWornRaw(reinterpret_cast<uintptr_t>(f), o.worn, kMaxWorn);
}

struct RawTime {
    bool ok = false;
    float current = 0, target = 0;
    uint32_t vatsMode = 0;
    bool haveVats = false;
    uint8_t pausedByMenu = 0;
    bool havePaused = false;
};

RawTime ReadTime() {
    RawTime t;
    t.ok = SafeRead(A::kTimeScaleCurrent, &t.current, sizeof(float)) && SafeRead(A::kTimeScaleTarget, &t.target, sizeof(float));
    t.haveVats = SafeRead(A::kVatsCamera + A::kCamMode, &t.vatsMode, sizeof(uint32_t));
    t.havePaused = SafeRead(A::kPausedByMenu, &t.pausedByMenu, 1);
    return t;
}

uint32_t PlayerRefId() {
    uintptr_t p = 0;
    if (!SafeRead(P::kThePlayerPtr, &p, sizeof p) || !p) return 0;
    uint32_t id = 0;
    return SafeRead(p + P::kFormRefId, &id, sizeof id) ? id : 0;
}

bool ParseRef(const Value& v, uint32_t& out, bool& isPlayer) {
    isPlayer = false;
    if (v.isNumber()) {
        out = static_cast<uint32_t>(v.asInt());
        return out != 0;
    }
    if (!v.isString()) return false;
    const std::string s = v.asString();
    if (s == "player") {
        isPlayer = true;
        out = PlayerRefId();
        return out != 0;
    }
    char* end = nullptr;
    out = static_cast<uint32_t>(std::strtoul(s.c_str(), &end, 16));
    return end && *end == 0 && out != 0;
}

// ---- the bridge method -----------------------------------------------------------------------------------------------------------

bool MLook(const std::string& params, std::string& out) {
    Value p;
    std::string err;
    if (!params.empty() && params != "null" && !nvcoop::json::parse(params, p, err)) {
        out = "bad params: " + err;
        return false;
    }
    if (!LookupPinOk()) {
        out = "LookupFormByID does not match its pinned prologue in this build; the probe will not call it";
        return false;
    }
    Value res = Value::object();
    res.set("v", 1).set("player", Hex(PlayerRefId()));
    const RawTime t = ReadTime();
    Value tv = Value::object();
    tv.set("ok", t.ok);
    if (t.ok) tv.set("current", static_cast<double>(t.current)).set("target", static_cast<double>(t.target));
    res.set("time", tv);
    if (t.haveVats) res.set("vatsMode", static_cast<int64_t>(t.vatsMode));
    if (t.havePaused) res.set("pausedByMenu", t.pausedByMenu != 0);

    Value arr = Value::array();
    const Value* refs = p.find("refs");
    int count = 0;
    if (refs) {
        for (const Value& rv : refs->items()) {
            if (++count > 96) break;
            uint32_t id = 0;
            bool isPlayer = false;
            Value a = Value::object();
            if (!ParseRef(rv, id, isPlayer)) {
                a.set("ref", rv.isString() ? rv.asString() : std::string("?")).set("ok", false).set("why", "bad ref");
                arr.push(a);
                continue;
            }
            a.set("ref", Hex(id));
            TESForm* f = LookupSEH(id);
            RawLook l;
            if (f) ReadLookRaw(f, l);
            if (!f || !l.ok) {
                a.set("ok", false).set("why", f ? "unreadable" : "not a loaded form");
                arr.push(a);
                continue;
            }
            a.set("ok", true).set("actor", l.actor).set("creature", l.creature).set("base", Hex(l.base));
            if (isPlayer) a.set("player", true);
            if (l.haveBase) a.set("race", Hex(l.race)).set("female", l.female).set("hair", Hex(l.hair)).set("eyes", Hex(l.eyes)).set("name", std::string(l.name));
            a.set("wornKnown", l.nWorn >= 0);
            Value w = Value::array();
            for (int i = 0; i < l.nWorn; ++i) {
                Value e = Value::object();
                e.set("form", Hex(l.worn[i].form)).set("type", static_cast<int64_t>(l.worn[i].type)).set("mods", static_cast<int64_t>(l.worn[i].mods));
                w.push(e);
            }
            a.set("worn", w);
            arr.push(a);
        }
    }
    res.set("actors", arr);
    out = res.dump();
    return true;
}

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string s = buf;
    size_t k = s.find_last_of("\\/");
    return (k == std::string::npos ? std::string() : s.substr(0, k + 1)) + "nvcoop.ini";
}

void OnLoad(void*) {
    if (GetPrivateProfileIntA("dev", "parityprobe", 0, IniPath().c_str()) == 0) return;
    nvcoop::RegisterMethod("parity.look", MLook, true);
    nvcoop::Log("parity probe: enabled ([dev] parityprobe=1): parity.look (read only)");
}

}  // namespace

NVCOOP_MODULE(l13_parity_probe, "l13_parity_probe", OnLoad, nullptr, nullptr);
