// plugin/combat: the byte-level half of the hit gate on a plain 0x64-byte buffer laid out like the engine's ActorHitData.
#include <cstring>
#include "combat/hit_edit.h"
#include "minitest.h"

using namespace nvcoop::combat;

namespace {

struct Buf {
    uint8_t b[addr::hit::kSize];
    Buf() { std::memset(b, 0, sizeof b); }
    template <class T> void set(uint32_t off, T v) { std::memcpy(b + off, &v, sizeof v); }
    template <class T> T get(uint32_t off) const { T v; std::memcpy(&v, b + off, sizeof v); return v; }
};

Buf sample() {
    Buf h;
    h.set<uint32_t>(addr::hit::kSource, 0x11111111u);
    h.set<uint32_t>(addr::hit::kTarget, 0x22222222u);
    h.set<uint32_t>(addr::hit::kProjOrExplosion, 0x33333333u);
    h.set<int32_t>(addr::hit::kHitLocation, 4);
    h.set<float>(addr::hit::kHealthDmg, 42.5f);
    h.set<float>(addr::hit::kWpnBaseDmg, 50.0f);
    h.set<float>(addr::hit::kFatigueDmg, 7.0f);
    h.set<float>(addr::hit::kLimbDmg, 12.0f);
    h.set<uint32_t>(addr::hit::kWeapon, 0x44444444u);
    h.set<float>(addr::hit::kImpactPos, 1.0f);
    h.set<float>(addr::hit::kImpactPos + 4, 2.0f);
    h.set<float>(addr::hit::kImpactPos + 8, 3.0f);
    h.set<uint32_t>(addr::hit::kFlags, addr::hit::kFlagCrit | addr::hit::kLethalFlags);
    return h;
}

TEST(combat_hit_edit, decode_reads_every_field_the_gate_uses) {
    Buf h = sample();
    HitBuf d;
    REQUIRE(decodeHit(h.b, &d));
    CHECK(d.source == 0x11111111u);
    CHECK(d.weapon == 0x44444444u);
    CHECK(d.projOrExplosion == 0x33333333u);
    CHECK(d.location == 4);
    CHECK(d.health == 42.5f);
    CHECK(d.fatigue == 7.0f);
    CHECK(d.limb == 12.0f);
    CHECK(d.impact == (cc::Vec3{1, 2, 3}));
    CHECK((d.flags & addr::hit::kFlagCrit) != 0);
    CHECK(!decodeHit(nullptr, &d));
}

TEST(combat_hit_edit, proceed_changes_nothing) {
    Buf h = sample();
    const Buf before = h;
    HitBuf d;
    decodeHit(h.b, &d);
    CHECK(!applyDecisionToHit(h.b, d, GateAction::Proceed, 0));
    CHECK(std::memcmp(h.b, before.b, sizeof h.b) == 0);
}

TEST(combat_hit_edit, clamp_limits_health_never_raises_it_and_strips_fatigue_limb_and_lethal_flags) {
    Buf h = sample();
    HitBuf d;
    decodeHit(h.b, &d);
    CHECK(applyDecisionToHit(h.b, d, GateAction::Clamp, 10));
    CHECK(h.get<float>(addr::hit::kHealthDmg) == 10.0f);
    CHECK(h.get<float>(addr::hit::kFatigueDmg) == 0.0f);
    CHECK(h.get<float>(addr::hit::kLimbDmg) == 0.0f);
    const uint32_t flags = h.get<uint32_t>(addr::hit::kFlags);
    CHECK((flags & addr::hit::kLethalFlags) == 0);
    CHECK((flags & addr::hit::kFlagCrit) != 0);                       // the crit flag is for the reaction, it stays
    CHECK(h.get<uint32_t>(addr::hit::kSource) == 0x11111111u);        // nothing else was touched
    CHECK(h.get<uint32_t>(addr::hit::kWeapon) == 0x44444444u);
    CHECK(h.get<int32_t>(addr::hit::kHitLocation) == 4);
    CHECK(h.get<float>(addr::hit::kWpnBaseDmg) == 50.0f);

    Buf h2 = sample();
    decodeHit(h2.b, &d);
    applyDecisionToHit(h2.b, d, GateAction::Clamp, 90);                 // a cap above the computed damage must not RAISE it
    CHECK(h2.get<float>(addr::hit::kHealthDmg) == 42.5f);
    Buf h3 = sample();
    decodeHit(h3.b, &d);
    applyDecisionToHit(h3.b, d, GateAction::Clamp, -5);                 // a negative cap is zero
    CHECK(h3.get<float>(addr::hit::kHealthDmg) == 0.0f);
}

TEST(combat_hit_edit, neutralize_zeroes_all_damage) {
    Buf h = sample();
    HitBuf d;
    decodeHit(h.b, &d);
    CHECK(applyDecisionToHit(h.b, d, GateAction::Neutralize, 99));
    CHECK(h.get<float>(addr::hit::kHealthDmg) == 0.0f);
    CHECK(h.get<float>(addr::hit::kFatigueDmg) == 0.0f);
    CHECK(h.get<float>(addr::hit::kLimbDmg) == 0.0f);
    CHECK((h.get<uint32_t>(addr::hit::kFlags) & addr::hit::kLethalFlags) == 0);
}

TEST(combat_hit_edit, the_native_hit_gets_the_fields_the_engine_reads) {
    Buf h;
    NativeHitSpec s;
    s.source = 0xA0; s.target = 0xB0; s.weapon = 0xC0; s.health = 33.0f; s.location = hitLocationOfLimb(cc::Limb::RightLeg);
    s.haveImpact = true; s.impact = {4, 5, 6}; s.crit = true;
    fillNativeHit(h.b, s);
    CHECK(h.get<uint32_t>(addr::hit::kSource) == 0xA0);
    CHECK(h.get<uint32_t>(addr::hit::kTarget) == 0xB0);
    CHECK(h.get<uint32_t>(addr::hit::kWeapon) == 0xC0);
    CHECK(h.get<float>(addr::hit::kHealthDmg) == 33.0f);
    CHECK(h.get<float>(addr::hit::kWpnBaseDmg) == 33.0f);
    CHECK(h.get<int32_t>(addr::hit::kHitLocation) == 10);
    CHECK(h.get<float>(addr::hit::kImpactPos + 4) == 5.0f);
    CHECK(h.get<uint32_t>(addr::hit::kFlags) == addr::hit::kFlagCrit);
    CHECK(h.get<float>(addr::hit::kDmgMult) == 1.0f);
    Buf g;
    s.haveImpact = false; s.crit = false;
    fillNativeHit(g.b, s);
    CHECK(g.get<float>(addr::hit::kImpactPos) == 0.0f);   // the constructor's default impact stays when none is known
    CHECK(g.get<uint32_t>(addr::hit::kFlags) == 0);
}

TEST(combat_hit_edit, a_touch_replays_a_token_health_over_a_real_base_damage) {
    // a blow that did no health damage is replayed for its reaction: the health is a token (the engine's damage path only runs for a positive amount), the base damage
    // the engine scales the reaction from is the real weapon base
    Buf h;
    NativeHitSpec s;
    s.source = 0xA0; s.target = 0xB0; s.health = 0.01f; s.baseDamage = 2.0f; s.location = 0;
    fillNativeHit(h.b, s);
    CHECK(h.get<float>(addr::hit::kHealthDmg) == 0.01f);
    CHECK(h.get<float>(addr::hit::kWpnBaseDmg) == 2.0f);
    Buf g;
    s.baseDamage = -1;   // none given: follows the health, as every real hit does
    fillNativeHit(g.b, s);
    CHECK(g.get<float>(addr::hit::kWpnBaseDmg) == 0.01f);
}

TEST(combat_hit_edit, limbs_map_to_distinct_body_parts_and_back) {
    const cc::Limb all[] = {cc::Limb::Torso, cc::Limb::Head, cc::Limb::LeftArm, cc::Limb::RightArm, cc::Limb::LeftLeg, cc::Limb::RightLeg};
    for (cc::Limb l : all) CHECK_MSG(limbOfLocation(hitLocationOfLimb(l)) == l, cc::toString(l));
    CHECK(hitLocationOfLimb(cc::Limb::LeftLeg) != hitLocationOfLimb(cc::Limb::RightLeg));
}

}  // namespace
