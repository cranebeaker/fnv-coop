// The first effect of a Stimpak form, in memory. x86, compiled against the xNVSE headers (FalloutNV.exe 1.4.0.525). NEVER RUN IN A GAME YET.
//
// Layout (xNVSE GameForms.h): AlchemyItem is a TESBoundObject with TESFullName at +0x30 (String name at +0x34) and an EffectItemList at +0x3C. The list is a
// BSSimpleList: a vtable pointer at +0, then a tList whose first node is embedded at +4 { data, next }. Each node's data is an EffectItem, whose first four bytes
// are the magnitude (a float, stored as UInt32 in the header). The form is found with xNVSE's LookupFormByID, checked for its type (kFormType_AlchemyItem) and for
// its own NAME (the table's "Stimpak" / "Super Stimpak"), and the magnitude must be a believable number before it is trusted; every access is SEH guarded, one fault
// turns the whole thing off for the session.
#include <Windows.h>

#include <cmath>
#include <cstdint>
#include <cstring>

#include "common/nvcoop_api.h"
#include "nvse/GameAPI.h"
#include "nvse/GameForms.h"
#include "nvse/GameObjects.h"
#include "rebalance/effects_real.h"

namespace nvcoop::rebalance {

namespace {

constexpr uint32_t kEffectsListOffset = 0x3C;   // AlchemyItem::effects (xNVSE GameForms.h)
constexpr uint32_t kListHeadOffset = 4;         // BSSimpleList: vtable, then the tList whose head node is embedded { data, next }

// Plain data only inside the __try (no C++ objects with destructors). status: 0 ok, 1 no such form, 2 not an AlchemyItem, 3 not the item expected,
// 4 no effect, 5 the magnitude is not believable, 9 fault.
uint32_t* FindMagnitude(uint32_t formId, const char* expectName, int* status) {
    __try {
        TESForm* f = LookupFormByID(formId);
        if (!f) { *status = 1; return nullptr; }
        if (f->typeID != kFormType_AlchemyItem) { *status = 2; return nullptr; }
        AlchemyItem* a = static_cast<AlchemyItem*>(f);
        const char* name = a->fullName.name.m_data;
        if (!name || _stricmp(name, expectName) != 0) { *status = 3; return nullptr; }
        uint8_t* node = reinterpret_cast<uint8_t*>(f) + kEffectsListOffset + kListHeadOffset;
        uint8_t* effect = *reinterpret_cast<uint8_t**>(node);
        if (!effect) { *status = 4; return nullptr; }
        float m;
        std::memcpy(&m, effect, sizeof m);
        if (!(m > 0.0f && m < 10000.0f)) { *status = 5; return nullptr; }
        *status = 0;
        return reinterpret_cast<uint32_t*>(effect);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        *status = 9;
        return nullptr;
    }
}

bool ReadFloat(const uint32_t* p, float* out) {
    __try {
        std::memcpy(out, p, sizeof *out);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool WriteFloat(uint32_t* p, float v) {
    __try {
        std::memcpy(p, &v, sizeof v);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

const char* NameOf(uint32_t formId) {
    for (const EffectEntry& e : effectTable())
        if (e.formId == formId) return e.name;
    return nullptr;
}

}  // namespace

uint32_t* GameEffects::find(uint32_t formId) {
    if (broken_) return nullptr;
    auto it = cache_.find(formId);
    if (it != cache_.end()) return it->second;
    const char* expect = NameOf(formId);
    if (!expect) return nullptr;
    int status = 0;
    uint32_t* p = FindMagnitude(formId, expect, &status);
    if (status == 9) { broken_ = true; nvcoop::Log("rebalance: the Stimpak forms could not be read safely (fault): they stay as the game has them"); return nullptr; }
    if (!p) {
        if (status != 1 && !missing_.count(formId)) { missing_[formId] = true; nvcoop::Log("rebalance: form %08X (%s) not usable (status %d)", formId, expect, status); }
        return nullptr;
    }
    cache_[formId] = p;
    return p;
}

bool GameEffects::read(uint32_t formId, float& magnitude) {
    uint32_t* p = find(formId);
    return p && ReadFloat(p, &magnitude);
}

bool GameEffects::write(uint32_t formId, float magnitude) {
    uint32_t* p = find(formId);
    if (!p) return false;
    if (!(magnitude >= 0.0f && magnitude < 10000.0f)) return false;
    if (WriteFloat(p, magnitude)) return true;
    broken_ = true;
    return false;
}

}  // namespace nvcoop::rebalance
