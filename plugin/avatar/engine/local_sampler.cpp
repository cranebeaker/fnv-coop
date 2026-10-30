#include "local_sampler.h"

#include <algorithm>
#include <cmath>

#include "../avatar_groups.h"
#include "../loco.h"
#include "fnvcoop_mut.h"

// Mutation twins in this file (tests/test_engine.cpp must fail for each):
// 40 teleport not reset (a 28 m jump becomes a huge speed)   41 cell change keeps the old position   42 mover flags always trusted
// 43 jump never set                                            44 reload never set                    45 sneak only from the mover
// 46 sequence number does not rise                             47 publishes on every call            48 heading read from rotY
// 49 publishes when the world is not ready                     50 equipment change published at once  51 weapon flag never set
// 52 equipment order not normalised                            53 tick does not advance               54 speed not clamped
// 55 snapshot ignores the aim flag                             56 cell not part of the snapshot
// 87 the player's action is never published                    88 a restart of the same group is not an event (two punches look like one)
// 89 every sample while it plays is a new event                90 a one-shot is not held in the stream
// 91 a dead player keeps publishing his last action            92 block is never published
// 93 a finished sequence still counts as playing               94 a held action (block) is held as long as a one-shot
// 95 the counter is reset by a world gap                       96 the first-person animation set is never read (a human's first-person attack is invisible)
// 97 the player's furniture is never read                      98 "wants to stand" is reported as leaving (the other player's puppet stands up while he sits)

namespace nvcoop::avatar::engine {

bool IsReloadGroupId(int g) { return g >= 173 && g <= 199; }
bool IsJumpGroupId(int g) { return g == 227 || g == 228 || g == 229 || (g >= 236 && g <= 239) || (g >= 241 && g <= 244); }
bool IsAttackGroupId(int g) { return g >= 26 && g <= 167; }

namespace {
bool FiniteV(const Vec3& v) { return std::isfinite(v.x) && std::isfinite(v.y) && std::isfinite(v.z); }
constexpr uint32_t kMoverWalk = 1u << 7, kMoverRun = 1u << 8, kMoverSneak = 1u << 9;
}  // namespace

void SitFromEngine(int state, uint8_t& sit, bool& sleep) {
    sit = 0;
    sleep = false;
    switch (state) {
        case 1: case 2: case 3: sit = 1; break;                          // loading the sit idle, wants to sit, waiting for the sit animation
        case 4: sit = 2; break;                                          // sitting
        case 5: sit = FNV_MUT(98) ? 3 : 2; break;                        // wants to stand (or the getter's rewrite of 4): twin 98 reports it as leaving, so the puppet gets up while its owner sits
        case 6: case 7: case 8: sit = 1; sleep = true; break;            // going to bed
        case 9: sit = 2; sleep = true; break;                            // sleeping
        case 10: sit = FNV_MUT(98) ? 3 : 2; sleep = true; break;
        default: break;
    }
}

namespace {
// The furniture reference the player sits in: the nearest one, asked again when he has moved more than 8 units or nothing was found (at most every 500 ms).
RefId LocalFurniture(IGameBackend& be, const Vec3& pos, SeatCache* sc) {
    if (!sc) return be.NearestFurniture(kPlayerRef);
    const uint32_t nowMs = static_cast<uint32_t>(be.NowUs() / 1000);
    const bool moved = !sc->have || std::fabs(pos.x - sc->at.x) > 8.0f || std::fabs(pos.y - sc->at.y) > 8.0f || std::fabs(pos.z - sc->at.z) > 8.0f;
    if (moved || (sc->furniture == 0 && static_cast<int32_t>(nowMs - sc->askedAtMs) >= 500)) {
        sc->furniture = be.NearestFurniture(kPlayerRef);
        sc->at = pos;
        sc->have = true;
        sc->askedAtMs = nowMs;
    }
    return sc->furniture;
}
}  // namespace

LocalFrame ReadLocalFrame(IGameBackend& be, SeatCache* seat) {
    LocalFrame f;
    RefInfo ri;
    if (!be.ReadRef(kPlayerRef, ri) || !ri.exists || !ri.isPlayer || ri.cell == 0 || !ri.has3D) return f;
    if (!FiniteV(ri.pos) || !FiniteV(ri.rot)) return f;
    f.valid = true;
    f.cell = ri.cell;
    f.pos = ri.pos;
    f.havePos3D = ri.havePos3D;
    f.pos3D = ri.pos3D;
    f.rot = FNV_MUT(48) ? Vec3{ri.rot.x, ri.rot.y, ri.rot.z} : Vec3{ri.rot.x, ri.rot.z, ri.rot.y};
    AnimRead ar;
    f.animReadable = be.ReadAnim(kPlayerRef, ar) && ar.valid;
    double v = 0;
    if (be.Eval(Cond::IsWeaponOut, kPlayerRef, 0, v)) f.anim.weaponDrawn = v != 0;
    else f.anim.weaponDrawn = f.animReadable && ar.weaponOut;
    bool sneakEval = be.Eval(Cond::IsSneaking, kPlayerRef, 0, v);
    if (f.animReadable) {
        f.anim.aim = ar.aiming;
        f.swimming = ar.swimming;
        f.dead = ar.dead;
        f.firstPerson = ar.firstPerson;
        bool jump = false, reload = false;
        // The sets the player's animation lives in: the process's (third person) and, for the local player, the first-person one (PlayerCharacter+0x690) and
        // the alternate (+0x6A0). A player in first person plays his swings and shots on the first-person set (engine_pins.h), so that set is asked first then.
        struct SetRef { const int* groups; const int* state; const int* playMs; int id; };
        SetRef sets[3];
        int nSets = 0;
        const SetRef third{ar.groups, ar.state, ar.playMs, kSetThird};
        const SetRef first{ar.alt[0].groups, ar.alt[0].state, ar.alt[0].playMs, kSetFirstPerson};
        const SetRef alt{ar.alt[1].groups, ar.alt[1].state, ar.alt[1].playMs, kSetAlt};
        if (ar.firstPerson && ar.alt[0].valid && !FNV_MUT(96)) sets[nSets++] = first;   // twin 96: the first-person set is never read
        sets[nSets++] = third;
        if (!ar.firstPerson && ar.alt[0].valid && !FNV_MUT(96)) sets[nSets++] = first;
        if (ar.alt[1].valid && nSets < 3 && !FNV_MUT(96)) sets[nSets++] = alt;
        f.setCount = nSets;
        for (int k = 0; k < nSets; ++k) {
            for (int i = 0; i < 9; ++i) {
                if (sets[k].groups[i] < 0) continue;
                if (IsJumpGroupId(sets[k].groups[i])) jump = true;
                if (IsReloadGroupId(sets[k].groups[i])) reload = true;
            }
        }
        // The action: the last slot first (the NPC layer reads the nine slots the same way, plugin/actors/anim/anim_game.cpp). A sequence the engine
        // reports as not playing (state other than animated / easing in / easing out) is a leftover in its slot, not an action.
        for (int k = 0; k < nSets && f.actionGroup < 0; ++k) {
            for (int i = 8; i >= 0; --i) {
                const int g = sets[k].groups[i];
                if (g < 0 || !IsActionGroup(g)) continue;
                const int st = sets[k].state[i];
                if (!FNV_MUT(93) && st >= 0 && (st < 1 || st > 3)) continue;
                f.actionGroup = g;
                f.actionPlayMs = sets[k].playMs[i];
                f.actionSet = sets[k].id;
                break;
            }
        }
        f.procAction = ar.action;
        f.anim.jump = FNV_MUT(43) ? false : jump;
        f.anim.reload = FNV_MUT(44) ? false : reload;
        if (ar.haveMover) {
            const bool informative = (ar.moverFlags & (kMoverWalk | kMoverRun)) != 0;
            f.haveMoveFlags = informative || FNV_MUT(42);
            f.moverRun = (ar.moverFlags & kMoverRun) != 0;
            f.moverWalk = (ar.moverFlags & kMoverWalk) != 0;
            f.moverSneak = (ar.moverFlags & kMoverSneak) != 0;
        }
    }
    if (FNV_MUT(45)) f.anim.sneak = f.moverSneak;
    else f.anim.sneak = sneakEval ? v != 0 : f.moverSneak;
    // furniture: a chair, a bench or a bed the player uses. The process's own sit state when it can be read, else the GetSitting evaluator (any non-zero = in it).
    if (!FNV_MUT(97) && !f.dead) {   // twin 97: the player's furniture is never read (a couch is invisible to the other player)
        int ss = -1;
        if (!be.ReadSitSleep(kPlayerRef, ss)) {
            double gs = 0;
            ss = be.Eval(Cond::GetSitting, kPlayerRef, 0, gs) ? (gs > 0.5 ? 4 : 0) : -1;
        }
        f.sitSleep = ss;
        uint8_t sit = 0;
        bool sleep = false;
        SitFromEngine(ss, sit, sleep);
        if (sit) {
            f.anim.sit = sit;
            f.anim.sleep = sleep;
            f.anim.furniture = LocalFurniture(be, f.pos, seat);
        } else if (seat) {
            seat->furniture = 0;
            seat->have = false;
        }
    }
    return f;
}

// ---- the player's one-shot -------------------------------------------------------------------------------------------------

void ActionTracker::Clear() {
    rawPrev_ = playPrev_ = -1;
    cur_ = 0;
    holding_ = false;
    if (FNV_MUT(95)) seq_ = 0;   // twin 95: the counter restarts after a loading screen / death (a copy that remembers 9 reads the next attack as older)
}

ActionTracker::Out ActionTracker::Step(uint32_t nowMs, int rawGroup, int rawPlayMs) {
    started_ = false;
    const bool raw = rawGroup > 0 && IsActionGroup(rawGroup);
    if (raw) {
        const bool newGroup = rawGroup != rawPrev_;
        const bool restarted = !newGroup && rawPlayMs >= 0 && playPrev_ >= 0 && rawPlayMs + cfg_.restartBackstepMs < playPrev_;
        bool started = newGroup || (restarted && !FNV_MUT(88));   // twin 88: a restart of the same group is no event (two punches in a row look like one)
        if (FNV_MUT(89)) started = true;                          // twin 89: every sample while it plays is a new event (the attack is replayed over and over)
        started_ = started;
        if (started) {
            ++seq_;
            cur_ = rawGroup;
            holding_ = true;
        }
        const bool held = IsHeldActionGroup(rawGroup) && !FNV_MUT(94);   // twin 94: block lingers as long as a swing does
        heldUntil_ = nowMs + (FNV_MUT(90) ? 0u : (held ? cfg_.heldActionHoldMs : cfg_.actionHoldMs));   // twin 90: no hold, the event lives only as long as the engine's sequence
    }
    rawPrev_ = raw ? rawGroup : -1;
    playPrev_ = raw ? rawPlayMs : -1;
    if (holding_ && !raw && static_cast<int32_t>(heldUntil_ - nowMs) <= 0) {
        holding_ = false;
        cur_ = 0;
    }
    Out o;
    o.started = raw && started_;
    o.action = static_cast<uint8_t>(holding_ ? cur_ : 0);
    o.actionSeq = seq_;
    o.block = !FNV_MUT(92) && holding_ && cur_ == actors::anim::kGroupBlockIdle;   // twin 92: blocking is never published
    return o;
}

// ---- sampler ---------------------------------------------------------------------------------------------------------

void LocalSampler::SetIdentity(live::NetId netId, live::OwnerEpoch epoch) {
    if (netId != netId_ || epoch != epoch_) seq_ = 0;   // a new identity restarts the per-(actor, epoch) sequence
    netId_ = netId;
    epoch_ = epoch;
}

bool LocalSampler::Due(uint32_t nowMs) const {
    if (FNV_MUT(47)) return true;
    return !haveAt_ || static_cast<int32_t>(nowMs - nextAt_) >= 0;
}

void LocalSampler::Reset() {
    havePrev_ = false;
    vel_ = Vec3{};
    loco_ = 0;
    action_.Clear();
}

bool LocalSampler::Sample(IGameBackend& be, uint32_t nowMs, live::ActorSnapshot& out) {
    if (!Due(nowMs)) return false;
    LocalFrame f = ReadLocalFrame(be, &seat_);
    return Update(nowMs, f, out);
}

bool LocalSampler::Update(uint32_t nowMs, const LocalFrame& f, live::ActorSnapshot& out) {
    if (!Due(nowMs)) return false;
    // the schedule advances by one period (not "now + period"), so 16.7 ms frames still average 20 Hz instead of 15 Hz
    if (!haveAt_ || static_cast<int32_t>(nowMs - nextAt_) > static_cast<int32_t>(cfg_.periodMs)) nextAt_ = nowMs + cfg_.periodMs;
    else nextAt_ += cfg_.periodMs;
    haveAt_ = true;
    if (!f.valid && !FNV_MUT(49)) {
        ++c_.notReady;
        havePrev_ = false;   // a loading screen / menu gap: the next position is not comparable
        action_.Clear();
        return false;
    }
    last_ = f;

    // velocity: finite difference between samples, restarted by a cell change, a long gap or a teleport
    Vec3 raw;
    bool restart = !havePrev_ || (prevCell_ != f.cell && !FNV_MUT(41)) || (nowMs - prevAt_) > cfg_.maxGapMs || nowMs == prevAt_;
    if (havePrev_ && prevCell_ != f.cell) ++c_.cellChanges;
    if (!restart) {
        const float dt = static_cast<float>(nowMs - prevAt_) * 0.001f;
        Vec3 d{f.pos.x - prevPos_.x, f.pos.y - prevPos_.y, f.pos.z - prevPos_.z};
        const float len = std::sqrt(d.x * d.x + d.y * d.y + d.z * d.z);
        // faster than anything that walks, runs or falls (maxSpeed) and far enough to be no jitter: a teleport. (The first version compared with 350 units whatever
        // the period: a 200 unit jump in 33 ms was a speed of 6000 u/s, clamped to 2200, and the other copy extrapolated it 330 units past the target.)
        if (len > (cfg_.speedTeleport ? std::max(cfg_.teleportUnits, cfg_.maxSpeed * dt) : 350.0f) && !FNV_MUT(40)) {
            ++c_.teleports;
            restart = true;
        } else {
            raw = Vec3{d.x / dt, d.y / dt, d.z / dt};
        }
    }
    if (restart) {
        vel_ = Vec3{};
    } else {
        const float a = cfg_.velAlpha;
        vel_ = Vec3{vel_.x + (raw.x - vel_.x) * a, vel_.y + (raw.y - vel_.y) * a, vel_.z + (raw.z - vel_.z) * a};
    }
    if (!FNV_MUT(54)) {
        const float sp = std::sqrt(vel_.x * vel_.x + vel_.y * vel_.y + vel_.z * vel_.z);
        if (sp > cfg_.maxSpeed) {
            const float k = cfg_.maxSpeed / sp;
            vel_ = Vec3{vel_.x * k, vel_.y * k, vel_.z * k};
        }
    }
    havePrev_ = true;
    prevCell_ = f.cell;
    prevPos_ = f.pos;
    prevAt_ = nowMs;

    LocalRead lr;
    lr.haveMoveFlags = f.haveMoveFlags;
    lr.moverRun = f.moverRun;
    lr.moverWalk = f.moverWalk;
    lr.moverSneak = f.moverSneak;
    Anim anim = f.anim;
    if (FNV_MUT(55)) anim.aim = false;
    if (f.dead && !FNV_MUT(91)) {   // twin 91: a dead player keeps publishing his last action
        action_.Clear();
        anim.actionSeq = action_.seq();   // nothing plays on a dead player, but the counter goes on
    } else {
        const ActionTracker::Out ao = action_.Step(nowMs, f.actionGroup, f.actionPlayMs);
        if (ao.started) {   // a new event: remembered for the log line (avatar_runtime.cpp PublishLocal)
            ++c_.actionEvents;
            lastEvent_ = ActionEvent{f.actionGroup, f.actionSet, f.procAction, f.firstPerson, f.setCount, ao.actionSeq, nowMs};
        }
        if (!FNV_MUT(87)) {   // twin 87: the player's attacks and reactions never reach the snapshot
            anim.action = ao.action;
            anim.actionSeq = ao.actionSeq;
            anim.block = ao.block;
        }
    }
    if (!FNV_MUT(46)) ++seq_;
    else seq_ = 1;
    out = MakeLocalSnapshot(netId_, epoch_, seq_, FNV_MUT(53) ? 0u : nowMs, FNV_MUT(56) ? 0u : f.cell, f.pos, f.rot, vel_, anim, &lr);
    loco_ = out.anim.locomotion;
    ++c_.published;
    return true;
}

// ---- equipment -------------------------------------------------------------------------------------------------------

void EquipWatcher::Reset() {
    havePolled_ = false;
    havePublished_ = false;
    havePending_ = false;
    published_.clear();
    pending_.clear();
}

namespace {
bool SameEquip(const std::vector<EquipEntry>& a, const std::vector<EquipEntry>& b) {
    if (a.size() != b.size()) return false;
    for (size_t i = 0; i < a.size(); ++i)
        if (a[i].form != b[i].form || a[i].weapon != b[i].weapon || (!FNV_MUT(83) && a[i].mods != b[i].mods)) return false;
    return true;
}
}  // namespace

bool EquipWatcher::Poll(IGameBackend& be, uint32_t nowMs, std::vector<EquipEntry>& out) {
    if (havePolled_ && static_cast<int32_t>(nowMs - lastPoll_) < static_cast<int32_t>(pollMs_)) return false;
    havePolled_ = true;
    lastPoll_ = nowMs;
    std::vector<EquippedItem> items;
    if (!be.ReadEquipped(kPlayerRef, items)) return false;   // unreadable: keep what was published
    std::vector<EquipEntry> cur;
    for (const EquippedItem& it : items) {
        if (!it.form) continue;
        EquipEntry e;
        e.form = it.form;
        e.weapon = FNV_MUT(51) ? false : it.formType == kFormTypeWeapon;
        e.mods = (e.weapon && !FNV_MUT(84)) ? static_cast<uint8_t>(it.mods & kWeaponModMask) : 0;   // armor has no modification flags
        cur.push_back(e);
    }
    if (!FNV_MUT(52)) {
        std::sort(cur.begin(), cur.end(), [](const EquipEntry& a, const EquipEntry& b) { return a.form < b.form; });
        cur.erase(std::unique(cur.begin(), cur.end(), [](const EquipEntry& a, const EquipEntry& b) { return a.form == b.form; }), cur.end());
    }
    if (havePublished_ && SameEquip(cur, published_)) {
        havePending_ = false;
        return false;
    }
    if (FNV_MUT(50)) {
        published_ = cur;
        havePublished_ = true;
        out = cur;
        return true;
    }
    if (!havePending_ || !SameEquip(cur, pending_)) {
        pending_ = cur;
        havePending_ = true;
        pendingSince_ = nowMs;
        return false;
    }
    if (static_cast<int32_t>(nowMs - pendingSince_) < static_cast<int32_t>(stableMs_)) return false;
    published_ = cur;
    havePublished_ = true;
    havePending_ = false;
    out = cur;
    return true;
}

}  // namespace nvcoop::avatar::engine
