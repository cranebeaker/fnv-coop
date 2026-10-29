#include "combat/hit_edit.h"
#include <cstring>
#include "../../tools/mut/fnvcoop_mut.h"

// Mutation twins 771..779 and 808 (runtime style); tests/test_hit_edit.cpp must catch each.

namespace nvcoop::combat {

namespace {
template <class T>
T rd(const uint8_t* b, uint32_t off) { T v; std::memcpy(&v, b + off, sizeof v); return v; }
template <class T>
void wr(uint8_t* b, uint32_t off, T v) { std::memcpy(b + off, &v, sizeof v); }
}  // namespace

bool decodeHit(const uint8_t* hit, HitBuf* out) {
    if (!hit || !out) return false;
    out->source = rd<uint32_t>(hit, addr::hit::kSource);
    out->weapon = rd<uint32_t>(hit, addr::hit::kWeapon);
    out->projOrExplosion = rd<uint32_t>(hit, addr::hit::kProjOrExplosion);
    out->flags = rd<uint32_t>(hit, addr::hit::kFlags);
    out->location = rd<int32_t>(hit, addr::hit::kHitLocation);
    out->health = rd<float>(hit, addr::hit::kHealthDmg);
    out->fatigue = rd<float>(hit, addr::hit::kFatigueDmg);
    out->limb = rd<float>(hit, addr::hit::kLimbDmg);
    out->impact = {rd<float>(hit, addr::hit::kImpactPos), rd<float>(hit, addr::hit::kImpactPos + 4), rd<float>(hit, addr::hit::kImpactPos + 8)};
    return true;
}

bool applyDecisionToHit(uint8_t* hit, const HitBuf& before, GateAction action, int32_t maxHealthDamage) {
    if (!hit || action == GateAction::Proceed) return false;
    float health = 0;
    if (action == GateAction::Clamp) {
        const float cap = static_cast<float>(maxHealthDamage < 0 ? 0 : maxHealthDamage);
        health = before.health < cap ? before.health : cap;   // never raises what the engine computed
        if (FNV_MUT(771)) health = cap;                       // 771: the cap replaces a smaller engine value (a 5 damage hit becomes 90)
    }
    if (action == GateAction::Neutralize && FNV_MUT(772)) health = before.health;   // 772: a neutralised hit keeps its health damage
    if (health < 0) health = 0;
    wr<float>(hit, addr::hit::kHealthDmg, health);
    if (!FNV_MUT(773)) wr<float>(hit, addr::hit::kFatigueDmg, 0.0f);   // 773: a copy can still be knocked out by a prediction
    if (!FNV_MUT(774)) wr<float>(hit, addr::hit::kLimbDmg, 0.0f);      // 774: a copy can still lose a limb to a prediction
    const uint32_t keep = FNV_MUT(775) ? 0u : addr::hit::kLethalFlags;   // 775: fatal / dismember / explode / cripple stay set
    wr<uint32_t>(hit, addr::hit::kFlags, before.flags & ~keep);
    return true;
}

int32_t hitLocationOfLimb(cc::Limb l) {
    switch (l) {
        case cc::Limb::Head: return 1;
        case cc::Limb::LeftArm: return 3;
        case cc::Limb::RightArm: return 5;
        case cc::Limb::LeftLeg: return 7;
        case cc::Limb::RightLeg: return FNV_MUT(776) ? 7 : 10;   // 776: right and left leg collapse into one body part
        default: return 0;
    }
}

void fillNativeHit(uint8_t* hit, const NativeHitSpec& s) {
    wr<uint32_t>(hit, addr::hit::kSource, s.source);
    wr<uint32_t>(hit, addr::hit::kTarget, s.target);
    wr<int32_t>(hit, addr::hit::kHitLocation, s.location);
    wr<float>(hit, addr::hit::kHealthDmg, s.health);
    wr<float>(hit, addr::hit::kWpnBaseDmg, FNV_MUT(777) ? 0.0f : (s.baseDamage >= 0.0f && !FNV_MUT(808) ? s.baseDamage : s.health));   // 808: a touch's base damage follows its token health   // 777: the base weapon damage stays 0 (the engine scales the reaction from it)
    wr<uint32_t>(hit, addr::hit::kWeapon, s.weapon);
    if (s.haveImpact) {
        wr<float>(hit, addr::hit::kImpactPos, s.impact.x);
        wr<float>(hit, addr::hit::kImpactPos + 4, s.impact.y);
        wr<float>(hit, addr::hit::kImpactPos + 8, s.impact.z);
    }
    wr<uint32_t>(hit, addr::hit::kFlags, s.crit ? addr::hit::kFlagCrit : 0u);
    wr<float>(hit, addr::hit::kDmgMult, 1.0f);
}

}  // namespace nvcoop::combat
