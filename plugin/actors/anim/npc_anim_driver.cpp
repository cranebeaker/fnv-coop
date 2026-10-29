#include "npc_anim_driver.h"

#include <algorithm>
#include <cmath>

#include "anim_groups.h"
#include "avatar/loco.h"
#include "fnvcoop_mut.h"

namespace nvcoop::actors::anim {

namespace {

bool reached(uint32_t now, uint32_t t) { return static_cast<int32_t>(now - t) >= 0; }
// A deadline that may be unset (0): true while it lies in the future. Deadlines are stamped with stamp() so a real one is never 0.
bool pending(uint32_t now, uint32_t until) { return until != 0 && static_cast<int32_t>(until - now) > 0; }
uint32_t stamp(uint32_t t) { return t ? t : 1u; }
float horiz(const Vec3& a, const Vec3& b) { return std::sqrt((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)); }

int groupForLoco(uint8_t loco) {
    switch (loco) {
        case avatar::kLocoWalkFwd: return kGroupForward;
        case avatar::kLocoWalkBack: return kGroupForward + 1;
        case avatar::kLocoWalkLeft: return kGroupForward + 2;
        case avatar::kLocoWalkRight: return kGroupForward + 3;
        case avatar::kLocoRunFwd: return kGroupFastRight - 3;
        case avatar::kLocoRunBack: return kGroupFastRight - 2;
        case avatar::kLocoRunLeft: return kGroupFastRight - 1;
        case avatar::kLocoRunRight: return kGroupFastRight;
        default: return kGroupIdle;
    }
}

}  // namespace

// ---- command plumbing ------------------------------------------------------------------------------------------------------------------

bool NpcAnimDriver::spend(uint32_t cost, bool essential) {
    if (FNV_MUT(45)) { budgetUsed_ += cost; return true; }   // twin 45: no call budget, a crowd floods the console
    const uint32_t cap = essential ? cfg_.hardBudget : cfg_.budgetPerTick;
    if (budgetUsed_ + cost > cap) { ++stats_.deferredBudget; return false; }
    budgetUsed_ += cost;
    return true;
}

NpcAnimDriver::Cmd NpcAnimDriver::note(Result r) {
    ++stats_.commands;
    switch (r) {
        case Result::Ok: return Cmd::Ok;
        case Result::Unsupported: ++stats_.unsupported; return Cmd::Unsupported;
        default: ++stats_.refused; return Cmd::Refused;
    }
}

NpcAnimDriver::Cmd NpcAnimDriver::play(NetId id, int group, bool hold, bool essential) {
    if (!validGroup(group) && group != kGroupIdle) return Cmd::Refused;   // never guess a group
    if (!spend(1, essential)) return Cmd::Deferred;
    const Cmd c = note(eng_.PlayGroup(id, group, hold));
    if (c == Cmd::Ok) ++stats_.groupsPlayed;
    return c;
}

void NpcAnimDriver::resetApplied(Puppet& p) {
    p.loop = -1; p.loopHold = true; p.rate = 1.0f; p.sneakKnown = false; p.weaponKnown = false; p.aimApplied = false; p.dirty = false;
    p.transient = -1; p.jump = JumpPhase::None; p.prevJump = false; p.turning = false; p.loopBo.clear(); p.sneakBo.clear(); p.rateBo.clear(); p.aimBo.clear();
    p.heldAction = 0; p.afterActionReassert = false; p.actionUntil = 0; p.getUpUntil = 0;
    p.seat = Puppet::Seat::Unknown; p.standPending = false; p.standTries = 0; p.standRetryAt = 0; p.sitBo.clear(); p.standBo.clear(); p.poseHeld = false; p.obsFresh = false;   // (p.unrestrained stays: the hold is still owed)
}

bool NpcAnimDriver::blocked(uint32_t now) {
    if (!haveBlocked_ || now != blockedAt_) {
        blockedCache_ = menuWas_ || eng_.MenuOpen() || !eng_.WorldReady();
        blockedAt_ = now; haveBlocked_ = true;
    }
    return blockedCache_;
}

void NpcAnimDriver::markAllDirty() {
    for (auto& kv : puppets_) kv.second.dirty = true;
}

// ---- world events and timers -----------------------------------------------------------------------------------------------------------

void NpcAnimDriver::tick(uint32_t now) {
    budgetUsed_ = 0;
    const uint32_t epoch = eng_.WorldEpoch();
    if (haveWorld_ && epoch != worldEpoch_ && !FNV_MUT(52)) {   // twin 52: a load / new game keeps the previous world's actors
        puppets_.clear(); dead_.clear(); earlyCorpse_.clear();
        ++stats_.worldResets;
    }
    worldEpoch_ = epoch; haveWorld_ = true;
    const bool menu = eng_.MenuOpen();
    if (menuWas_ && !menu && !FNV_MUT(39)) {   // twin 39: animation is not re-asserted after a menu closed (the engine resets it)
        markAllDirty();
        ++stats_.menuReasserts;
    }
    menuWas_ = menu;
    haveBlocked_ = false;
    if (menu || !eng_.WorldReady()) return;

    for (auto& kv : puppets_) {
        if (kv.second.started) verifyPuppet(kv.first, kv.second, now);
    }
    for (auto it = dead_.begin(); it != dead_.end();) {
        tickDead(it->first, it->second, now);
        // resolved bodies are forgotten after a minute; one that never got its pose (its owner left) after ten, so the table cannot grow for ever
        if ((it->second.resolved && elapsed(now, it->second.since) > 60000u) || elapsed(now, it->second.since) > 600000u) it = dead_.erase(it); else ++it;
    }
    for (auto it = earlyCorpse_.begin(); it != earlyCorpse_.end();) {
        if (elapsed(now, it->second.second) > 30000u) it = earlyCorpse_.erase(it); else ++it;
    }
}

// ---- displayed motion ------------------------------------------------------------------------------------------------------------------

void NpcAnimDriver::measure(Puppet& p, const live::InterpOut& out, uint32_t now) {
    const float yaw = out.rot.y;
    auto restart = [&]() {
        p.havePos = true; p.lastPos = out.pos; p.lastPosMs = now;
        p.vdisp = {out.vel.x, out.vel.y, 0};
        p.speed = std::sqrt(p.vdisp.x * p.vdisp.x + p.vdisp.y * p.vdisp.y);
        p.lastYaw = yaw; p.yawRate = 0; p.turnAccum = 0;
    };
    if (!p.havePos || out.snapped) { restart(); return; }
    const uint32_t dt = elapsed(now, p.lastPosMs);
    if (dt > 500) { restart(); return; }   // a gap (menu, frozen stream): measure again from the owner's velocity
    if (dt < 8) return;
    const float sec = static_cast<float>(dt) / 1000.0f;
    const float vx = (out.pos.x - p.lastPos.x) / sec, vy = (out.pos.y - p.lastPos.y) / sec;
    const float alpha = 1.0f - std::exp(-static_cast<float>(dt) / static_cast<float>(cfg_.speedWindowMs ? cfg_.speedWindowMs : 1u));
    p.vdisp.x += (vx - p.vdisp.x) * alpha;
    p.vdisp.y += (vy - p.vdisp.y) * alpha;
    p.speed = std::sqrt(p.vdisp.x * p.vdisp.x + p.vdisp.y * p.vdisp.y);
    p.lastPos = out.pos; p.lastPosMs = now;
    const float dy = avatar::WrapRad(yaw - p.lastYaw);
    p.yawRate += (dy / sec - p.yawRate) * alpha;
    p.lastYaw = yaw;
    if (p.turning || std::fabs(p.yawRate) > cfg_.turnExitRadPerSec) p.turnAccum += dy; else p.turnAccum = 0;
}

// ---- the movement / idle group ---------------------------------------------------------------------------------------------------------

int NpcAnimDriver::pickLoopGroup(Puppet& p, const live::InterpOut& out, uint32_t now, bool& hold) {
    const live::AnimState& a = out.anim;
    hold = true;
    // A dry buffer (the owner stopped sending) means standing still, never running in place.
    const bool frozen = !FNV_MUT(32) && (out.state == live::InterpState::Holding || out.state == live::InterpState::Empty);   // twin 32: it keeps running in place
    uint8_t code = a.locomotion > avatar::kLocoMax ? static_cast<uint8_t>(avatar::kLocoIdle) : a.locomotion;
    if (frozen) code = avatar::kLocoIdle;
    const float speed = frozen ? 0.0f : p.speed;
    p.moving = speed >= (p.moving ? cfg_.standExit : cfg_.standEnter);

    // The owner's code is the truth for walk / run and direction, but the DISPLAYED speed must agree: an idle code with real speed, or a walk
    // code at run speed (or the reverse) that lasts longer than the debounce is corrected from the speed, so the feet never skate.
    uint8_t want = code;
    bool disagree = false;
    const bool codeMoving = avatar::LocoIsMoving(code);
    if (!codeMoving && p.moving) {
        disagree = true;
        const avatar::Vec3 v{p.vdisp.x, p.vdisp.y, 0};
        const int dir = avatar::DirectionOf(v, out.rot.y, p.dirPrev);
        const bool run = p.runNow ? speed > cfg_.runExit : speed > cfg_.runEnter;
        want = avatar::MakeLoco(run, dir);
    } else if (codeMoving && p.moving) {
        const bool runBySpeed = p.runNow ? speed > cfg_.runExit : speed > cfg_.runEnter;
        if (avatar::LocoIsRun(code) != runBySpeed) {
            disagree = true;
            want = avatar::MakeLoco(runBySpeed, avatar::LocoDir(code));
        }
    }
    // (codeMoving && !moving: the owner is running into a wall; the code is kept - unless the stream is dry, handled above)
    if (FNV_MUT(30)) { disagree = false; want = code; }   // twin 30: the owner's code alone decides - a lost / late code makes the feet skate
    if (disagree) {
        if (!p.disagreeing) { p.disagreeing = true; p.disagreeSince = now; }
        if (!FNV_MUT(31) && elapsed(now, p.disagreeSince) < cfg_.speedDebounceMs) want = code;   // twin 31: no debounce, one odd frame flips the group
    } else {
        p.disagreeing = false;
    }
    const bool moving = avatar::LocoIsMoving(want);
    if (moving) { p.runNow = avatar::LocoIsRun(want); p.dirPrev = avatar::LocoDir(want); }
    else { p.runNow = false; p.dirPrev = -1; }
    p.wantLoco = want;

    // ---- jump: start, loop while airborne, land ----
    if (a.jump && !p.prevJump) { p.jump = JumpPhase::Start; p.jumpUntil = stamp(now + cfg_.jumpStartMs); ++stats_.jumpsPlayed; }
    else if (!a.jump && p.prevJump && p.jump != JumpPhase::None) { p.jump = FNV_MUT(49) ? JumpPhase::None : JumpPhase::Land; p.jumpUntil = stamp(now + cfg_.landMs); }   // twin 49: never lands
    if (a.jump && p.jump == JumpPhase::None) p.jump = JumpPhase::Loop;   // the first sample is already in the air
    if (p.jump == JumpPhase::Start && reached(now, p.jumpUntil)) p.jump = JumpPhase::Loop;
    if (p.jump == JumpPhase::Land && reached(now, p.jumpUntil)) p.jump = JumpPhase::None;
    p.prevJump = a.jump;

    // ---- turn in place (humanoids standing still) ----
    bool turnNow = false;
    if (cfg_.playTurns && !p.creature && !moving && !FNV_MUT(47)) {   // twin 47: turn animations are never played, the body rotates on the spot in a plain idle
        if (p.turning) {
            turnNow = std::fabs(p.yawRate) > cfg_.turnExitRadPerSec || elapsed(now, p.turnSince) < cfg_.turnMinMs;
        } else if (std::fabs(p.yawRate) > cfg_.turnEnterRadPerSec && std::fabs(p.turnAccum) >= cfg_.turnMinTotalRad) {
            turnNow = true; p.turnSince = now; ++stats_.turnsPlayed;
        }
    }
    p.turning = turnNow;

    // ---- pick ----
    if (p.jump == JumpPhase::Start) return kGroupJumpStart;
    const bool fwd = moving || speed >= cfg_.standEnter;
    if (p.jump == JumpPhase::Loop) return fwd ? kGroupJumpLoopForward : kGroupJumpLoop;
    if (p.jump == JumpPhase::Land) { hold = false; return fwd ? kGroupJumpLandForward : kGroupJumpLand; }
    if (p.transient >= 0 && pending(now, p.transientUntil) && !moving) { hold = false; return p.transient; }
    if (moving) return groupForLoco(want);
    if (turnNow) return p.yawRate > 0 ? kGroupTurnRight : kGroupTurnLeft;
    if (!p.creature && a.weaponDrawn && a.aim) {
        const float pitch = out.rot.x;
        if (pitch > cfg_.aimPitchDeadRad) return kGroupAim + 1;    // AimUp
        if (pitch < -cfg_.aimPitchDeadRad) return kGroupAim + 2;   // AimDown
        return kGroupAim;
    }
    return kGroupIdle;
}

// ---- weapon ----------------------------------------------------------------------------------------------------------------------------

void NpcAnimDriver::issueWeapon(NetId id, Puppet& p, uint32_t now) {
    ++p.weaponTries;
    p.weaponAt = now;
    if (noWeapon_ || FNV_MUT(33)) return;   // twin 33: the weapon flag is never set, the copy fights with its weapon holstered
    if (!spend(1, true)) { --p.weaponTries; return; }
    const Cmd c = note(eng_.SetWeaponDrawn(id, p.weaponWant));
    if (c == Cmd::Unsupported) noWeapon_ = true;
}

void NpcAnimDriver::applyWeapon(NetId id, Puppet& p, const live::AnimState& a, uint32_t now) {
    if (p.creature) return;
    const bool want = a.weaponDrawn;
    if (!p.weaponKnown) {   // first look at this copy (or after a reset): make the engine match, without playing a draw animation
        p.weaponKnown = true; p.weaponWant = want; p.weaponConfirmed = false; p.weaponTries = 0; p.haveWeaponRetry = false;
        issueWeapon(id, p, now);
    } else if (want != p.weaponWant) {
        p.weaponWant = want; p.weaponConfirmed = false; p.weaponTries = 0; p.haveWeaponRetry = false;
        ++stats_.weaponChanges;
        issueWeapon(id, p, now);
        if (!p.moving) { p.transient = want ? kGroupEquip : kGroupUnequip; p.transientUntil = stamp(now + cfg_.equipAnimMs); }
    }
    if (a.aim != p.aimApplied && !noAim_ && p.aimBo.ready(now)) {
        if (spend(1, false)) {
            const Cmd c = note(eng_.SetAiming(id, a.aim));
            if (c == Cmd::Unsupported) noAim_ = true;
            if (c == Cmd::Ok || c == Cmd::Unsupported) { p.aimApplied = a.aim; p.aimBo.clear(); }
            else p.aimBo.fail(now, cfg_.retryBackoffMs);
        }
    }
}

// ---- the owner's one-shots ---------------------------------------------------------------------------------------------------------------

void NpcAnimDriver::applyOneShot(NetId id, Puppet& p, const live::AnimState& a, uint32_t now) {
    // A held action (block, automatic fire) ends when the owner stops reporting it: the movement group takes over again.
    if (p.heldAction && a.action != p.heldAction) { p.heldAction = 0; p.afterActionReassert = true; }
    if (cfg_.playTalking && !p.creature && !FNV_MUT(55) && a.talk != p.prevTalk) {   // twin 55: the owner's talking is never shown (twin 87: it is played over the furniture pose)
        p.prevTalk = a.talk;
        if (a.talk && !p.moving && (!p.poseHeld || FNV_MUT(87)) && spend(1, false) && play(id, kGroupTalking, false, false) == Cmd::Ok) p.actionGroup = kGroupTalking;
    }
    if (!p.haveSeq) {
        // First look: adopt the counter without playing. An old event (before this copy existed, or before a load) must not be replayed.
        p.haveSeq = true;
        if (FNV_MUT(36)) p.lastSeq = static_cast<uint8_t>(a.actionSeq - 1);   // twin 36: the first state's one-shot is played (old attacks replay after a load / a stand-in appears)
        else { p.lastSeq = a.actionSeq; return; }
    }
    if (a.actionSeq == p.lastSeq) return;
    const bool newer = FNV_MUT(37) ? (a.actionSeq > p.lastSeq) : live::actionSeqNewer(p.lastSeq, a.actionSeq);   // twin 37: the counter does not wrap (255 -> 0 is "older")
    p.lastSeq = a.actionSeq;
    if (!newer) { ++stats_.oneShotsStale; return; }
    if (!cfg_.playAttacks || FNV_MUT(35)) return;   // twin 35: attacks / reactions are never played on the copy
    const int g = a.action;
    if (g == 0 || !validGroup(g) || !isActionGroup(g)) { ++stats_.oneShotsMissed; return; }   // the owner's event was over before this copy's render time reached it
    const bool hold = isHeldAction(g);
    if (play(id, g, hold, true) == Cmd::Ok) {
        ++stats_.oneShotsPlayed;
        p.actionGroup = g;
        p.actionUntil = stamp(now + nominalActionMs(g));
        if (hold) p.heldAction = g;
        if (isReactionGroup(g) || isDodgeGroup(g)) p.afterActionReassert = true;
    }
}

// ---- knockdown -------------------------------------------------------------------------------------------------------------------------

void NpcAnimDriver::applyKnock(NetId id, Puppet& p, const live::InterpOut& out, uint32_t now) {
    const bool k = out.anim.knocked;
    if (cfg_.knockdown && k && !p.prevKnocked) {
        bool done = false;
        if (!noKnock_ && spend(1, true)) {
            const Cmd c = note(eng_.KnockDown(id, out.rot.y, 1.0f));
            if (c == Cmd::Ok) { ++stats_.knockdowns; done = true; }
            else if (c == Cmd::Unsupported) noKnock_ = true;
        }
        if (!done && !FNV_MUT(50)) {   // twin 50: no fallback when the engine cannot knock down: the copy keeps standing while the owner's lies on the floor
            if (play(id, kGroupStagger, false, true) == Cmd::Ok) { ++stats_.knockdownFallbacks; p.actionGroup = kGroupStagger; p.actionUntil = stamp(now + nominalActionMs(kGroupStagger)); }
        }
    } else if (cfg_.knockdown && !k && p.prevKnocked) {
        if (!noKnock_ && spend(1, true)) {
            if (note(eng_.GetUp(id)) == Cmd::Ok) ++stats_.getUps;
        }
        p.loop = -1; p.afterActionReassert = true; p.getUpUntil = stamp(now + 1200);
    }
    p.prevKnocked = k;
}

// ---- furniture -------------------------------------------------------------------------------------------------------------------------
// The copy sits in the same furniture as the owner's actor, or stands up. The order that matters: a copy told "standing" leaves the furniture
// BEFORE any movement group is played over it (drive() plays none while this returns true), and a copy told "seated" gets no movement group at
// all. Commands only go out on a CHANGE of what the stream wants (or when a read-back finds the engine did not do it), never per frame.

void NpcAnimDriver::observeSeat(Puppet& p, const NpcReads& r) {
    p.sitReadable = r.sitKnown;
    if (!r.sitKnown) { p.obsFresh = false; return; }
    p.obs = sitFromEngine(r.sitSleep);
    p.obsFurniture = r.furniture;
    p.obsFresh = true;
}

bool NpcAnimDriver::applySeat(NetId id, Puppet& p, const live::AnimState& a, uint32_t now) {
    if (!cfg_.playSit || noSeat_) { p.poseHeld = false; return false; }
    // twin 81: "leaving" is still treated as seated (the copy stands up only when the owner's state is None, a beat late)
    const bool wantSeat = a.sit == live::SitState::Entering || a.sit == live::SitState::Using || (FNV_MUT(81) && a.sit == live::SitState::Leaving);

    // 1. first look at this copy (or after a reset): is it already in some furniture? (a copy that was loaded sitting stays that way until told)
    if (p.seat == Puppet::Seat::Unknown) {
        p.seat = Puppet::Seat::Standing;
        NpcReads r;
        if (!FNV_MUT(79) && eng_.Read(id, r) && r.valid && r.sitKnown) {   // twin 79: no first look, a copy that loads sitting is assumed standing
            p.sitReadable = true;
            if (sitFromEngine(r.sitSleep).seated()) { p.seat = Puppet::Seat::Seated; p.seatFurniture = r.furniture; p.sitCmdAt = 0; }
        }
    }

    // 2. what the last read-back said (verifyPuppet reads the copy every reassertMs anyway; while a stand-up is under way it is looked at faster,
    //    the movement group waits for it)
    if (p.standPending && !p.obsFresh && reached(now, p.pendingReadAt)) {
        p.pendingReadAt = stamp(now + cfg_.standPollMs);
        NpcReads r;
        if (eng_.Read(id, r) && r.valid) observeSeat(p, r);
    }
    if (p.obsFresh) {
        p.obsFresh = false;
        const bool judged = p.sitCmdAt == 0 || elapsed(now, p.sitCmdAt) >= cfg_.verifyGraceMs;
        if (p.standPending) {
            if (p.obs.state == live::SitState::None && judged) {   // out of the furniture: the movement group may take over
                p.standPending = false; p.standTries = 0; p.loop = -1; ++stats_.standsConfirmed;
            }
        } else if (judged && !FNV_MUT(82)) {   // twin 82: the read-back is never used, a sit the engine dropped (or a copy that sits on its own) is not noticed
            if (p.seat == Puppet::Seat::Seated && p.obs.state == live::SitState::None) {
                p.seat = Puppet::Seat::Standing; ++stats_.sitDrops;   // the engine dropped the pose: the sit below is issued again (with backoff)
                p.sitBo.fail(now, cfg_.retryBackoffMs);
            } else if (p.seat == Puppet::Seat::Standing && p.obs.seated()) {
                p.seat = Puppet::Seat::Seated; p.seatFurniture = p.obsFurniture; ++stats_.sitDrifts;   // in furniture the stream does not (or no longer) want
            }
        }
    }
    if (p.standPending) {
        const uint32_t waited = elapsed(now, p.standSince);
        if (!p.sitReadable && waited >= cfg_.standSettleMs) {   // nothing to read it back with: assume it is out after the settle time
            p.standPending = false; p.standTries = 0; p.loop = -1; ++stats_.standsConfirmed;
        } else if (waited >= cfg_.standWaitMs && !FNV_MUT(80)) {   // twin 80: a stand-up that never reads back as finished blocks the movement group for ever
            p.standPending = false;
            if (p.standTries < cfg_.standMaxTries) {
                p.seat = Puppet::Seat::Seated;   // still in the furniture: the next escalation goes out below
            } else {
                p.standTries = 0; p.standRetryAt = stamp(now + cfg_.sitRetryMs); ++stats_.standGiveUps;
                p.loop = -1;
            }
        }
    }
    // the hold goes back once the actor is out (or we gave up): an escalated stand-up let its AI run
    if (!p.standPending && p.unrestrained && !FNV_MUT(83) && spend(1, true)) {   // twin 83: the hold is never put back, the actor's AI keeps running on a remote-driven copy
        const Cmd c = note(eng_.Restrain(id));
        if (c == Cmd::Ok) { p.unrestrained = false; ++stats_.restrains; }
    }

    // 3. decide
    if (wantSeat) {
        if (p.seat == Puppet::Seat::Seated) {
            if (a.furniture && p.seatFurniture && a.furniture != p.seatFurniture) p.seat = Puppet::Seat::Standing;   // another piece of furniture: use that one
            else {
                if (!p.seatFurniture) p.seatFurniture = a.furniture;
                p.poseHeld = true;
                if (!FNV_MUT(78)) return true;   // twin 78: the belief is ignored, the sit command is repeated every frame
            }
        }
        if (p.standPending) { p.poseHeld = true; return true; }   // still on the way out of the old pose
        if (!a.furniture) {   // the owner could not say which: nothing to sit in (the stream carries it as soon as it can)
            if (!p.noFurnitureCounted) { p.noFurnitureCounted = true; ++stats_.sitNoFurniture; }
            p.poseHeld = p.seat == Puppet::Seat::Seated;
            return p.poseHeld;
        }
        p.noFurnitureCounted = false;
        if (!p.sitBo.ready(now) || FNV_MUT(75)) { p.poseHeld = p.seat == Puppet::Seat::Seated; return p.poseHeld; }   // twin 75: the sit command is never issued
        if (!spend(1, true)) { p.poseHeld = p.seat == Puppet::Seat::Seated; return p.poseHeld; }
        const Cmd c = note(eng_.SitIn(id, a.furniture));
        if (c == Cmd::Ok) {
            p.seat = Puppet::Seat::Seated; p.seatFurniture = a.furniture; p.sitCmdAt = stamp(now); p.sitBo.clear(); p.loop = -1; ++stats_.sitsIssued;
        } else if (c == Cmd::Unsupported) {
            noSeat_ = true;
        } else if (c == Cmd::Refused) {
            p.sitBo.fail(now, cfg_.retryBackoffMs);
        }
        p.poseHeld = p.seat == Puppet::Seat::Seated;
        return p.poseHeld;
    }

    // the stream says standing (or leaving)
    p.noFurnitureCounted = false;
    if (p.seat == Puppet::Seat::Seated && !p.standPending) {
        if (FNV_MUT(76)) { p.seat = Puppet::Seat::Standing; p.poseHeld = false; return false; }   // twin 76: the copy is never stood up, it slides across the room sitting
        const bool coolingDown = p.standRetryAt != 0 && !reached(now, p.standRetryAt);
        if (!coolingDown && p.standBo.ready(now) && spend(1, true)) {
            p.standRetryAt = 0;
            const uint32_t furn = p.seatFurniture ? p.seatFurniture : p.obsFurniture;
            const uint32_t attempt = (furn || p.standTries) ? p.standTries : 1u;   // without a furniture reference there is no gentle way
            const Cmd c = note(eng_.StandUp(id, furn, attempt));
            if (c == Cmd::Ok) {
                ++p.standTries;
                p.seat = Puppet::Seat::Standing; p.standPending = true; p.standSince = now; p.sitCmdAt = stamp(now); p.standBo.clear();
                p.pendingReadAt = stamp(now + cfg_.verifyGraceMs);
                if (attempt >= 1) p.unrestrained = true;
                p.loop = -1; ++stats_.standsIssued;
            } else if (c == Cmd::Unsupported) {
                noSeat_ = true; p.seat = Puppet::Seat::Standing;
            } else if (c == Cmd::Refused) {
                p.standBo.fail(now, cfg_.retryBackoffMs);
            }
        }
    }
    p.poseHeld = p.standPending || p.seat == Puppet::Seat::Seated;
    return p.poseHeld;
}

// ---- driving a live copy ---------------------------------------------------------------------------------------------------------------

void NpcAnimDriver::drive(NetId id, const live::InterpOut& out, uint32_t now) {
    ++stats_.drives;
    if (dead_.count(id) && !FNV_MUT(46)) return;   // twin 46: a dead actor is still driven (its body keeps getting animation commands)
    if (blocked(now) && !FNV_MUT(84)) return;   // a menu is up (it may have opened since the last tick) or no game is loaded (twin 84: commands go out anyway)
    auto pit = puppets_.find(id);
    if (pit == puppets_.end()) pit = puppets_.emplace(id, Puppet{}).first;
    Puppet& p = pit->second;
    p.lastDriveMs = now;
    if (p.haveCell && p.cell != out.cell) { p.dirty = true; ++stats_.cellReasserts; }
    p.cell = out.cell; p.haveCell = true;
    if (out.snapped) p.dirty = true;

    // ---- baseline after the 3D appeared or was rebuilt (T-pose guard) ----
    const uint32_t gen = eng_.Generation3D(id);
    if (!p.started || (gen != p.gen3D && !FNV_MUT(40))) {   // twin 40: a rebuilt skeleton is not noticed
        NpcReads r;
        if (!eng_.Read(id, r) || !r.valid || (!r.has3D && !FNV_MUT(38))) { ++stats_.waiting3D; return; }   // no skeleton yet: nothing can play, try again next drive (twin 38: it is assumed to be there, the refused commands then back off while the copy stands in its bind pose)
        const bool again = p.started;
        resetApplied(p);
        p.gen3D = gen; p.started = true; p.creature = r.creature; p.nextVerifyMs = now + cfg_.reassertMs;
        if (again) ++stats_.rebuilds3D;
        else if (r.moveGroup == NpcReads::kNoSequence) ++stats_.tposeFixes;
    }
    if (p.dirty) resetApplied(p);

    measure(p, out, now);
    const live::AnimState& a = out.anim;

    applyKnock(id, p, out, now);
    applyWeapon(id, p, a, now);      // (before the pick: a draw / holster transition is shown in this very frame)
    const bool furnitureOwnsPose = applySeat(id, p, a, now);   // (before any movement group: a copy told standing leaves the furniture first)
    bool hold = true;
    const int want = pickLoopGroup(p, out, now, hold);
    applyOneShot(id, p, a, now);

    if (a.knocked || a.ragdoll) return;                    // the physics own the pose; no movement group over a body on the floor
    if ((furnitureOwnsPose && !FNV_MUT(77)) || pending(now, p.getUpUntil) || (isReactionGroup(p.actionGroup) && pending(now, p.actionUntil))) {   // twin 77: a movement group is played over the furniture pose
        // in the furniture / getting out of it / getting up / a reaction is playing: do not stomp it with the movement group
    } else {
        if (p.afterActionReassert) { p.loop = -1; p.afterActionReassert = false; }
        if ((want != p.loop || p.loopHold != hold) && p.loopBo.ready(now)) {
            const Cmd c = play(id, want, hold, p.loop < 0);
            if (c == Cmd::Ok) {
                if (p.loop >= 0) ++stats_.loopChanges;
                p.loop = want; p.loopHold = hold; p.loopAt = now; p.loopBo.clear();
                if (p.rate != 1.0f && !p.rateUnsupported) {   // a new group starts at its natural speed
                    if (spend(1, false)) { if (note(eng_.SetGroupRate(id, 1.0f)) == Cmd::Ok) p.rate = 1.0f; }
                }
            } else if (c == Cmd::Refused) {
                p.loopBo.fail(now, cfg_.retryBackoffMs);
            }
        }
        // ---- playback rate follows the displayed speed: no foot sliding between the walk and the run band ----
        if (cfg_.useRate && !noRate_ && !p.rateUnsupported && !FNV_MUT(48) && p.loop == want && groupForLoco(p.wantLoco) == want && isLocomotionGroup(want)) {   // twin 48: the rate is never adapted
            const float nominal = avatar::LocoIsRun(p.wantLoco) ? cfg_.runNominal : cfg_.walkNominal;
            const float rate = std::clamp(p.speed / nominal, cfg_.rateMin, cfg_.rateMax);
            if (std::fabs(rate - p.rate) > 0.08f && p.rateBo.ready(now) && spend(1, false)) {
                const Cmd c = note(eng_.SetGroupRate(id, rate));
                if (c == Cmd::Ok) { p.rate = rate; ++stats_.rateChanges; p.rateBo.clear(); }
                else if (c == Cmd::Unsupported) { noRate_ = true; p.rateUnsupported = true; }
                else p.rateBo.fail(now, cfg_.retryBackoffMs);
            }
        }
    }

    // ---- sneak ----
    if (!p.creature && !FNV_MUT(51) && (!p.sneakKnown || p.sneak != a.sneak) && p.sneakBo.ready(now)) {   // twin 51: sneak is never applied
        if (spend(1, true)) {
            const Cmd c = note(eng_.SetSneak(id, a.sneak));
            if (c == Cmd::Ok) { if (p.sneakKnown) ++stats_.sneakChanges; p.sneak = a.sneak; p.sneakKnown = true; p.sneakBo.clear(); }
            else p.sneakBo.fail(now, cfg_.retryBackoffMs);
        }
    }
}

// ---- verification (read back what the engine really plays) -------------------------------------------------------------------------------

void NpcAnimDriver::verifyPuppet(NetId id, Puppet& p, uint32_t now) {
    if (FNV_MUT(54)) return;   // twin 54: nothing is read back, a group the engine dropped stays dropped
    if (elapsed(now, p.lastDriveMs) > 1500) return;   // not driven lately (owner now / out of range): nothing to hold
    if (!reached(now, p.nextVerifyMs)) return;
    p.nextVerifyMs = now + cfg_.reassertMs;
    NpcReads r;
    if (!eng_.Read(id, r) || !r.valid) return;
    observeSeat(p, r);
    if (!r.has3D) return;
    const bool settled = !pending(now, p.getUpUntil) && !(isReactionGroup(p.actionGroup) && pending(now, p.actionUntil));

    if (settled && p.loop >= 0 && p.loopHold) {
        if (r.moveGroup == NpcReads::kNoSequence) {   // bind pose
            ++stats_.tposeFixes;
            if (play(id, p.loop, true, true) == Cmd::Ok) { p.loopAt = now; ++stats_.reasserts; }
        } else if (r.moveGroup >= 0 && r.moveGroup != p.loop && elapsed(now, p.loopAt) >= cfg_.verifyGraceMs) {
            ++stats_.groupMismatches;
            if (play(id, p.loop, true, false) == Cmd::Ok) { p.loopAt = now; ++stats_.reasserts; }
        } else if (r.moveGroup == NpcReads::kUnknownGroup && p.loop != kGroupIdle && elapsed(now, p.loopAt) >= cfg_.blindReassertMs) {
            if (play(id, p.loop, true, false) == Cmd::Ok) { p.loopAt = now; ++stats_.reasserts; }
        }
    }

    // weapon: read back, and climb the ladder when the engine did not take it. A confirmed state that later drifts (a restrained actor's own
    // logic holsters it) goes through the same ladder again.
    if (!p.creature && p.weaponKnown && p.weaponConfirmed && r.weaponOut != p.weaponWant && !FNV_MUT(34) && !FNV_MUT(56)) {   // twin 56: a confirmed weapon state is never checked again
        p.weaponConfirmed = false; p.weaponTries = 0; p.haveWeaponRetry = false; p.weaponAt = now - cfg_.weaponVerifyMs;
        ++stats_.weaponDrifts;
    }
    if (!p.creature && p.weaponKnown && !p.weaponConfirmed && !FNV_MUT(34)) {   // twin 34: the weapon state is never read back or retried
        if (p.haveWeaponRetry && !reached(now, p.weaponRetryAt)) return;
        if (elapsed(now, p.weaponAt) >= cfg_.weaponVerifyMs) {
            if (r.weaponOut == p.weaponWant) {
                p.weaponConfirmed = true; p.haveWeaponRetry = false; ++stats_.weaponConfirmed;
            } else {
                ++stats_.weaponVerifyFailures;
                if (p.weaponTries < 3) {
                    issueWeapon(id, p, now);
                    if (spend(1, true)) note(eng_.PlayGroup(id, p.weaponWant ? kGroupEquip : kGroupUnequip, false));
                } else {
                    ++stats_.weaponLadderGiveUps;
                    p.weaponTries = 0; p.weaponAt = now;
                    p.weaponRetryAt = now + cfg_.weaponRetryMs; p.haveWeaponRetry = true;
                }
            }
        }
    }
}

// ---- death and the corpse's resting place -------------------------------------------------------------------------------------------------

void NpcAnimDriver::onDeath(NetId id, uint32_t now, bool fromNetwork) {
    if (dead_.count(id)) return;
    Dead d;
    d.since = now; d.fromNetwork = fromNetwork; d.verified = !fromNetwork; d.nextKillMs = now + cfg_.deathVerifyMs; d.nextCorrectMs = now;
    auto ec = earlyCorpse_.find(id);
    if (ec != earlyCorpse_.end()) { d.havePose = true; d.pose = ec->second.first; d.poseAt = ec->second.second; earlyCorpse_.erase(ec); }
    dead_[id] = d;
    puppets_.erase(id);
    ++stats_.deaths;
}

void NpcAnimDriver::onCorpse(NetId id, const CorpsePose& pose, uint32_t now) {
    ++stats_.corpsePoses;
    auto it = dead_.find(id);
    if (it == dead_.end()) { earlyCorpse_[id] = {pose, now}; return; }
    Dead& d = it->second;
    if (d.missingCounted) ++stats_.corpseLate;
    d.havePose = true; d.pose = pose; d.poseAt = now; d.resolved = false; d.corrections = 0; d.nextCorrectMs = now;
}

void NpcAnimDriver::tickDead(NetId id, Dead& d, uint32_t now) {
    // 1. the engine must really have the actor dead (a network death is applied by the port; verify it, ask again if it did not take)
    if (!d.verified && !FNV_MUT(41) && reached(now, d.nextKillMs)) {   // twin 41: the death is never verified, an actor the engine ignored stays on its feet
        NpcReads r;
        if (eng_.Read(id, r) && r.valid) {
            if (r.dead) d.verified = true;
            else if (d.killTries < cfg_.deathMaxRetries) {
                if (spend(1, true)) {
                    ++d.killTries; ++stats_.killRetries;
                    note(eng_.Kill(id));
                    d.nextKillMs = now + 400u * d.killTries;
                }
            } else { d.verified = true; ++stats_.killGaveUp; }
        } else {
            d.nextKillMs = now + 500;   // not loaded here: the port applies the kill when it loads
        }
    }
    // 2. the corpse's resting place
    if (!d.havePose) {
        if (!d.missingCounted && elapsed(now, d.since) >= cfg_.corpsePoseTimeoutMs) { d.missingCounted = true; ++stats_.corpseMissing; }
        return;
    }
    if (d.resolved || FNV_MUT(42)) return;   // twin 42: the owner's resting place is ignored, bodies lie wherever the local ragdoll left them
    CellId cell = 0; Vec3 pos; float yaw = 0;
    if (!eng_.GetPose(id, cell, pos, yaw)) return;
    // is the local body at rest?
    if (d.haveLast) {
        const uint32_t dt = elapsed(now, d.lastPosMs);
        if (dt >= 20) {
            const float sp = horiz(pos, d.lastPos) * 1000.0f / static_cast<float>(dt) + std::fabs(pos.z - d.lastPos.z) * 1000.0f / static_cast<float>(dt);
            if (sp <= cfg_.corpseRestUnitsPerSec) { if (!d.resting) { d.resting = true; d.restSince = now; } }
            else d.resting = false;
            d.lastPos = pos; d.lastPosMs = now;
        }
    } else { d.haveLast = true; d.lastPos = pos; d.lastPosMs = now; }
    const uint32_t age = elapsed(now, d.since);
    const bool atRest = d.resting && elapsed(now, d.restSince) >= cfg_.corpseRestMs;
    const bool waited = age >= cfg_.corpseApplyDelayMs && (atRest || age >= cfg_.corpseMaxWaitMs);
    if (!(waited || FNV_MUT(43)) || !reached(now, d.nextCorrectMs)) return;   // twin 43: corrected at once, while the ragdoll is still falling (it carries on to somewhere else)
    const float err = horiz(pos, d.pose.pos), dz = std::fabs(pos.z - d.pose.pos.z);
    const bool within = err <= cfg_.corpseSnapUnits && dz <= cfg_.corpseSnapZ;
    if (within && !FNV_MUT(44)) {   // twin 44: always moved, even when it already lies where the owner's does (a needless teleport of a settled body)
        d.resolved = true;
        if (d.corrections == 0) ++stats_.corpseWithinTolerance;
        return;
    }
    if (d.corrections >= cfg_.corpseMaxCorrections) { d.resolved = true; ++stats_.corpseGaveUp; return; }
    if (!spend(1, true)) return;
    ++d.corrections;
    ++stats_.corpseCorrections;
    note(eng_.PlaceCorpse(id, d.pose.pos, d.pose.yawRad));
    d.nextCorrectMs = now + 300;
    d.resting = false; d.haveLast = false;
    if (FNV_MUT(44)) d.resolved = true;
}

// ---- lifetime ------------------------------------------------------------------------------------------------------------------------------

void NpcAnimDriver::release(NetId id) {
    auto it = puppets_.find(id);
    if (it != puppets_.end() && it->second.started && !FNV_MUT(53)) {   // twin 53: the held animation is left on an actor whose own AI takes over
        if (spend(1, true)) { note(eng_.ReleaseHold(id)); ++stats_.releases; }
    }
    forget(id);
}

void NpcAnimDriver::forget(NetId id) {
    puppets_.erase(id);
    dead_.erase(id);
    earlyCorpse_.erase(id);
}

void NpcAnimDriver::revive(NetId id) {
    dead_.erase(id);
    earlyCorpse_.erase(id);
    puppets_.erase(id);   // starts from a clean baseline at the next drive
}

PuppetInfo NpcAnimDriver::info(NetId id) const {
    PuppetInfo i;
    auto it = puppets_.find(id);
    if (it == puppets_.end()) return i;
    const Puppet& p = it->second;
    i.known = true; i.loopGroup = p.loop; i.loopHold = p.loopHold; i.weaponDrawn = p.weaponWant; i.weaponConfirmed = p.weaponConfirmed;
    i.sneak = p.sneak; i.speed = p.speed; i.rate = p.rate; i.lastActionGroup = p.actionGroup; i.lastActionSeq = p.lastSeq; i.turning = p.turning;
    i.seated = p.seat == Puppet::Seat::Seated; i.standPending = p.standPending; i.furniture = p.seatFurniture;
    return i;
}

bool NpcAnimDriver::poseLocked(NetId id) const {
    auto it = puppets_.find(id);
    return it != puppets_.end() && it->second.standPending;
}

DeadInfo NpcAnimDriver::deadInfo(NetId id) const {
    DeadInfo i;
    auto it = dead_.find(id);
    if (it == dead_.end()) return i;
    i.known = true; i.verified = it->second.verified; i.havePose = it->second.havePose; i.resolved = it->second.resolved; i.corrections = it->second.corrections;
    return i;
}

std::vector<NetId> NpcAnimDriver::driven() const {
    std::vector<NetId> v;
    for (auto& kv : puppets_) v.push_back(kv.first);
    return v;
}

uint32_t NpcAnimDriver::lastDriveMs(NetId id) const {
    auto it = puppets_.find(id);
    return it == puppets_.end() ? 0 : it->second.lastDriveMs;
}

}  // namespace nvcoop::actors::anim
