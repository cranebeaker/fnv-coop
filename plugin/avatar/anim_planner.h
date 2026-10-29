// Chooses the animation group a puppet should play from the owner's AnimState, with speed matching.
//
// Rules (each one is a puppet trap with a test in tests/test_avatar.cpp):
//   * the owner's locomotion code is the truth for walk/run and direction, BUT the velocity must agree: an idle code
//     with real speed (or a walk code at run speed, or the other way round) for longer than speedDebounceMs is
//     corrected from the velocity, so feet never skate; a moving code with zero velocity (running into a wall) is kept.
//   * a frozen interpolation buffer (the remote stopped sending) means idle at once, never run-in-place.
//   * jump = JumpStart, JumpLoop, then JumpLand for landMs; a forward variant when moving.
//   * reload beats everything but jump (it plays the owner's reload group when the stream carries one); draw / holster play a transition group for equipAnimMs when standing still.
//   * weapon drawn and standing: Aim (or AimIS), AimUp / AimDown when the pitch is beyond the dead zone.
//   * the owner's other one-shots (attack, block, stagger ...) are NOT planned here: PuppetManager plays them once per counter bump, on top of this.
#pragma once
#include <cstdint>

#include "avatar_groups.h"
#include "avatar_types.h"
#include "loco.h"

namespace nvcoop::avatar {

struct AnimConfig {
    SpeedBands bands;
    uint32_t speedDebounceMs = 150;
    uint32_t equipAnimMs = 450;
    uint32_t jumpStartMs = 200;
    uint32_t landMs = 350;
    float aimPitchDeadRad = 0.26f;   // about 15 degrees
    int reloadGroup = kGrpReloadA;   // TODO-RE: the real reload group depends on the weapon's animation type
};

struct PlanInput {
    Anim anim;
    Vec3 vel;
    float yaw = 0;
    float pitch = 0;
    uint32_t nowMs = 0;
    bool frozen = false;
};

struct PlanOut {
    int group = kGrpIdle;
    bool sneak = false;
    bool weaponDrawn = false;
    bool speedCorrected = false;   // the group differs from what the owner's locomotion code alone would give
};

class AnimPlanner {
public:
    explicit AnimPlanner(const AnimConfig& cfg = {}) : cfg_(cfg) {}
    PlanOut Plan(const PlanInput& in);
    void Reset();

private:
    enum class JumpPhase : uint8_t { None, Start, Loop, Land };
    AnimConfig cfg_;
    bool havePrev_ = false;
    bool prevJump_ = false, prevDrawn_ = false;
    bool runNow_ = false;
    int dirPrev_ = -1;
    uint32_t disagreeSince_ = 0;
    bool disagreeing_ = false;
    JumpPhase jump_ = JumpPhase::None;
    uint32_t jumpUntil_ = 0;
    int transient_ = -1;
    uint32_t transientUntil_ = 0;
};

// Group for a moving code (walk/run, direction), 0 idle otherwise.
int GroupForLoco(uint8_t loco);

}  // namespace nvcoop::avatar
