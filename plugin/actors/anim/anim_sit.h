// Furniture (sit / sleep) state as the NPC animation layer sees it. The engine side is BaseProcess::GetSitSleepState (vtable slot 0x12F,
// pinned in anim_pins.h): a byte of the process whose values are the xNVSE enum kSitSleepState_* (GameProcess.h). The wire side is
// live::SitState + a bed flag (synccore/live/live.h, schema v3). This file is the one mapping between the two.
#pragma once
#include <cstdint>

#include "live/live.h"

namespace nvcoop::actors::anim {

namespace live = fnvcoop::live;

// xNVSE GameProcess.h, BaseProcess::kSitSleepState_*.
enum EngineSitSleep : int {
    kSitSleepNormal = 0,
    kSitSleepLoadSitIdle = 1,
    kSitSleepWantToSit = 2,
    kSitSleepWaitingForSitAnim = 3,
    kSitSleepSitting = 4,
    kSitSleepWantToStand = 5,
    kSitSleepLoadingSleepIdle = 6,
    kSitSleepWantToSleep = 7,
    kSitSleepWaitingForSleepAnim = 8,
    kSitSleepSleeping = 9,
    kSitSleepWantToWake = 10,
};

struct SitView {
    live::SitState state = live::SitState::None;
    bool bed = false;
    friend bool operator==(const SitView& a, const SitView& b) { return a.state == b.state && a.bed == b.bed; }
    // The body is in the furniture or going into it (what a copy told "seated" has to match); Leaving is already on its way out.
    bool seated() const { return state == live::SitState::Entering || state == live::SitState::Using; }
};

// Anything outside the enum reads as standing: a value this layer does not know is never turned into a pose.
inline SitView sitFromEngine(int raw) {
    SitView v;
    switch (raw) {
        case kSitSleepLoadSitIdle: case kSitSleepWantToSit: case kSitSleepWaitingForSitAnim: v.state = live::SitState::Entering; break;
        case kSitSleepSitting: v.state = live::SitState::Using; break;
        case kSitSleepWantToStand: v.state = live::SitState::Leaving; break;
        case kSitSleepLoadingSleepIdle: case kSitSleepWantToSleep: case kSitSleepWaitingForSleepAnim: v.state = live::SitState::Entering; v.bed = true; break;
        case kSitSleepSleeping: v.state = live::SitState::Using; v.bed = true; break;
        case kSitSleepWantToWake: v.state = live::SitState::Leaving; v.bed = true; break;
        default: break;
    }
    return v;
}

}  // namespace nvcoop::actors::anim
