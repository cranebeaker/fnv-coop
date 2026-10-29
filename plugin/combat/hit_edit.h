// The pure byte-level half of the hit gate: decode the fields of an engine ActorHitData (0x64 bytes), edit them according to a gate decision, and fill a
// freshly constructed one for the native replay. No engine call, no pointer is followed (the module does that with SEH), so the offsets and the clamp /
// neutralise arithmetic are unit-tested on a plain buffer (tests/test_hit_edit.cpp).
#pragma once
#include <cstdint>
#include "combat/combat_addrs.h"
#include "combat/combat_sync.h"

namespace nvcoop::combat {

struct HitBuf {
    uint32_t source = 0, weapon = 0, projOrExplosion = 0;   // raw pointers as 32-bit values (x86)
    uint32_t flags = 0;
    int32_t location = -1;
    float health = 0, fatigue = 0, limb = 0;
    cc::Vec3 impact;
};

// Reads the fields from a copy of the hit data. false = null buffer.
bool decodeHit(const uint8_t* hit, HitBuf* out);

// Edits `hit` for a decision: Proceed leaves every byte alone; Clamp limits the health damage to maxHealthDamage (never raises it), zeroes fatigue and limb
// damage and clears the fatal / dismember / explode / cripple flags; Neutralize zeroes health too. Returns true when anything was written.
bool applyDecisionToHit(uint8_t* hit, const HitBuf& before, GateAction action, int32_t maxHealthDamage);

// Multiplies the health and limb damage of `hit` by `factor` (0..10; 1 and anything out of range write nothing) and updates `before` to match.
// Returns true when anything was written.
bool scaleHitDamage(uint8_t* hit, HitBuf& before, float factor);

// What the native replay puts into a constructor-initialised hit.
struct NativeHitSpec {
    uint32_t source = 0, target = 0, weapon = 0;   // 32-bit pointers
    float health = 0;
    float baseDamage = -1;   // the weapon base damage the engine scales its reaction from; negative = same as `health` (a touch passes a token health and a real base)
    int32_t location = 0;
    bool haveImpact = false;
    cc::Vec3 impact;
    bool crit = false;
};
void fillNativeHit(uint8_t* hit, const NativeHitSpec& s);

// Combat limb -> GECK humanoid body-part index (ActorHitData+0x10); inverse of limbOfLocation for the first index of each limb.
int32_t hitLocationOfLimb(cc::Limb l);

}  // namespace nvcoop::combat
