// The armour class of what an actor wears, for Co-op balanced's armour factor (balance.h bodyHitFactor). x86, compiled into nvcoop.dll only.
//
// The worn entries come from the actors engine's raw reader (plugin/actors/engine/equipment_real.cpp). For each ARMO (xNVSE GameForms.h kFormType_TESObjectARMO,
// 0x18) the form is looked up with xNVSE's LookupFormByID and its TESBipedModelForm flags are read: GameForms.h, TESObjectARMO::bipedModel at +0x70, and in it
// bipedFlags at +0x08 (TESBipedModelForm::eBipedFlag_MediumArmor 0x08, eBipedFlag_HeavyArmor 0x80). The heaviest piece counts. NOTHING here has run in a game.
#include "rebalance/armor_real.h"

#include <Windows.h>

#include <vector>

#include "nvse/GameAPI.h"
#include "nvse/GameForms.h"
#include "actors/engine/equipment_real.h"

namespace nvcoop::rebalance {

namespace {
// Named after xNVSE (third_party/NVSE nvse/nvse/GameForms.h); the offsets are the comments in that header.
constexpr uint8_t kFormTypeARMO = static_cast<uint8_t>(kFormType_TESObjectARMO);   // 0x18
constexpr uint32_t kArmoBipedModelOffset = 0x70;                                   // TESObjectARMO::bipedModel
constexpr uint32_t kBipedFlagsOffset = 0x08;                                       // TESBipedModelForm::bipedFlags
constexpr uint32_t kArmorBipedFlags = kArmoBipedModelOffset + kBipedFlagsOffset;
constexpr uint8_t kBipedMedium = static_cast<uint8_t>(TESBipedModelForm::eBipedFlag_MediumArmor);   // 0x08
constexpr uint8_t kBipedHeavy = static_cast<uint8_t>(TESBipedModelForm::eBipedFlag_HeavyArmor);     // 0x80
static_assert(kFormTypeARMO == 0x18 && kBipedMedium == 0x08 && kBipedHeavy == 0x80, "xNVSE GameForms.h changed");

// 0 light, 1 medium, 2 heavy, -1 not an armour form or the read faulted. Plain data only inside the __try.
int ClassOfForm(uint32_t formId) {
    __try {
        TESForm* f = LookupFormByID(formId);
        if (!f || f->typeID != kFormTypeARMO) return -1;
        const uint8_t flags = *reinterpret_cast<const uint8_t*>(reinterpret_cast<const uint8_t*>(f) + kArmorBipedFlags);
        if (flags & kBipedHeavy) return 2;
        if (flags & kBipedMedium) return 1;
        return 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}
}  // namespace

ArmorClass armorClassOf(uint32_t ref) {
    if (!ref) return ArmorClass::None;
    std::vector<nvcoop::actors::engine::WornEntry> worn;
    if (!nvcoop::actors::engine::readWornEntries(ref, worn)) return ArmorClass::None;
    int best = -1;
    for (const auto& w : worn) {
        if (w.formType != kFormTypeARMO) continue;
        const int c = ClassOfForm(w.form);
        if (c > best) best = c;
    }
    switch (best) {
        case 2: return ArmorClass::Heavy;
        case 1: return ArmorClass::Medium;
        case 0: return ArmorClass::Light;
        default: return ArmorClass::None;
    }
}

}  // namespace nvcoop::rebalance
