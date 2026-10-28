#include "local_sampler.h"

#include <algorithm>
#include <cmath>

#include "../loco.h"
#include "fnvcoop_mut.h"

// Mutation twins in this file (tests/test_engine.cpp must fail for each):
// 40 teleport not reset (a 28 m jump becomes a huge speed)   41 cell change keeps the old position   42 mover flags always trusted
// 43 jump never set                                            44 reload never set                    45 sneak only from the mover
// 46 sequence number does not rise                             47 publishes on every call            48 heading read from rotY
// 49 publishes when the world is not ready                     50 equipment change published at once  51 weapon flag never set
// 52 equipment order not normalised                            53 tick does not advance               54 speed not clamped
// 55 snapshot ignores the aim flag                             56 cell not part of the snapshot

namespace nvcoop::avatar::engine {

bool IsReloadGroupId(int g) { return g >= 173 && g <= 199; }
bool IsJumpGroupId(int g) { return g == 227 || g == 228 || g == 229 || (g >= 236 && g <= 239) || (g >= 241 && g <= 244); }
bool IsAttackGroupId(int g) { return g >= 26 && g <= 167; }

namespace {
bool FiniteV(const Vec3& v) { return std::isfinite(v.x) && std::isfinite(v.y) && std::isfinite(v.z); }
constexpr uint32_t kMoverWalk = 1u << 7, kMoverRun = 1u << 8, kMoverSneak = 1u << 9;
}  // namespace

LocalFrame ReadLocalFrame(IGameBackend& be) {
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
        for (int i = 0; i < 9; ++i) {
            if (ar.groups[i] < 0) continue;
            if (IsJumpGroupId(ar.groups[i])) jump = true;
            if (IsReloadGroupId(ar.groups[i])) reload = true;
        }
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
    return f;
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
}

bool LocalSampler::Sample(IGameBackend& be, uint32_t nowMs, live::ActorSnapshot& out) {
    if (!Due(nowMs)) return false;
    LocalFrame f = ReadLocalFrame(be);
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
