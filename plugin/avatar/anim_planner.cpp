#include "anim_planner.h"

#include <cmath>

#include "avatar_faults.h"

namespace nvcoop::avatar {

int GroupForLoco(uint8_t loco) {
    switch (loco) {
        case kLocoWalkFwd: return kGrpForward;
        case kLocoWalkBack: return kGrpBackward;
        case kLocoWalkLeft: return kGrpLeft;
        case kLocoWalkRight: return kGrpRight;
        case kLocoRunFwd: return kGrpFastForward;
        case kLocoRunBack: return kGrpFastBackward;
        case kLocoRunLeft: return kGrpFastLeft;
        case kLocoRunRight: return kGrpFastRight;
        default: return kGrpIdle;
    }
}

void AnimPlanner::Reset() {
    *this = AnimPlanner(cfg_);
}

PlanOut AnimPlanner::Plan(const PlanInput& inRaw) {
    const AvatarFaults& F = avatarFaults();
    PlanInput in = inRaw;
    if (in.frozen && !F.frozenKeepsRunning) {
        // The remote stopped sending: hold still. Keep stance (sneak, weapon) but never keep a moving pose.
        in.anim.loco = kLocoIdle;
        in.anim.jump = false;
        in.anim.reload = false;
        in.vel = Vec3{};
    }
    PlanOut out;
    out.sneak = in.anim.sneak;
    out.weaponDrawn = in.anim.weaponDrawn;
    const uint32_t now = in.nowMs;

    // ---- locomotion with speed matching ------------------------------------------------------------------------
    uint8_t code = in.anim.loco > kLocoMax ? static_cast<uint8_t>(kLocoIdle) : in.anim.loco;
    float speed = HorizontalSpeed(in.vel);
    bool codeMoving = LocoIsMoving(code);
    bool speedMoving = speed >= cfg_.bands.standMax;
    uint8_t want = code;
    bool disagree = false;
    if (!F.trustLocoCode) {
        if (!codeMoving && speedMoving) {
            disagree = true;
            int dir = DirectionOf(in.vel, in.yaw, dirPrev_);
            bool run = runNow_ ? speed > cfg_.bands.runExit : speed > cfg_.bands.runEnter;
            want = MakeLoco(run, dir);
        } else if (codeMoving && speedMoving) {
            bool runBySpeed = runNow_ ? speed > cfg_.bands.runExit : speed > cfg_.bands.runEnter;
            if (LocoIsRun(code) != runBySpeed) {
                disagree = true;
                want = MakeLoco(runBySpeed, LocoDir(code));
            }
        }
        // codeMoving && !speedMoving: running against a wall. Keep the owner's code.
    }
    if (disagree) {
        if (!disagreeing_) {
            disagreeing_ = true;
            disagreeSince_ = now;
        }
        if (now - disagreeSince_ < cfg_.speedDebounceMs) want = code;
    } else {
        disagreeing_ = false;
    }
    out.speedCorrected = (want != code);
    if (LocoIsMoving(want)) {
        runNow_ = LocoIsRun(want);
        dirPrev_ = LocoDir(want);
    } else {
        runNow_ = false;
        dirPrev_ = -1;
    }
    bool moving = LocoIsMoving(want);

    // ---- jump --------------------------------------------------------------------------------------------------
    bool jumpNow = in.anim.jump;
    if (havePrev_) {
        if (jumpNow && !prevJump_) {
            jump_ = JumpPhase::Start;
            jumpUntil_ = now + cfg_.jumpStartMs;
        } else if (!jumpNow && prevJump_) {   // falling edge, from Start or Loop
            jump_ = F.noJumpLand ? JumpPhase::None : JumpPhase::Land;
            jumpUntil_ = now + cfg_.landMs;
        }
    } else if (jumpNow) {
        jump_ = JumpPhase::Loop;   // first sample already in the air
    }
    if (jump_ == JumpPhase::Start && now >= jumpUntil_) jump_ = JumpPhase::Loop;
    if (jump_ == JumpPhase::Land && now >= jumpUntil_) jump_ = JumpPhase::None;

    // ---- weapon transition ---------------------------------------------------------------------------------------
    if (havePrev_ && in.anim.weaponDrawn != prevDrawn_) {
        transient_ = in.anim.weaponDrawn ? kGrpEquip : kGrpUnequip;
        transientUntil_ = now + cfg_.equipAnimMs;
    }
    if (transient_ >= 0 && now >= transientUntil_) transient_ = -1;
    prevJump_ = jumpNow;
    prevDrawn_ = in.anim.weaponDrawn;
    havePrev_ = true;

    // ---- pick ----------------------------------------------------------------------------------------------------
    const bool fwdVariant = speed >= cfg_.bands.standMax || moving;
    if (jump_ == JumpPhase::Start) {
        out.group = kGrpJumpStart;
    } else if (jump_ == JumpPhase::Loop) {
        out.group = fwdVariant ? kGrpJumpLoopForward : kGrpJumpLoop;
    } else if (jump_ == JumpPhase::Land) {
        out.group = fwdVariant ? kGrpJumpLandForward : kGrpJumpLand;
    } else if (in.anim.reload) {
        // the owner's own reload group (it depends on the weapon's animation type) when the stream carries it, else the configured default
        out.group = (IsReloadActionGroup(in.anim.action) && !F.reloadGroupFixed) ? in.anim.action : cfg_.reloadGroup;
    } else if (transient_ >= 0 && !moving) {
        out.group = transient_;
    } else if (moving) {
        out.group = GroupForLoco(want);
    } else if (in.anim.weaponDrawn) {
        bool is = in.anim.aim;
        if (in.pitch > cfg_.aimPitchDeadRad) out.group = is ? kGrpAimISUp : kGrpAimUp;
        else if (in.pitch < -cfg_.aimPitchDeadRad) out.group = is ? kGrpAimISDown : kGrpAimDown;
        else out.group = is ? kGrpAimIS : kGrpAim;
    } else {
        out.group = kGrpIdle;
    }
    return out;
}

}  // namespace nvcoop::avatar
