#include "puppet_manager.h"

#include <algorithm>
#include <cmath>
#include <cstdio>

#include "avatar_faults.h"
#include "avatar_groups.h"
#include "loco.h"

namespace nvcoop::avatar {

const char* ToString(Phase p) {
    switch (p) {
        case Phase::Absent: return "Absent";
        case Phase::Loading3D: return "Loading3D";
        case Phase::Driving: return "Driving";
        case Phase::Failed: return "Failed";
    }
    return "?";
}

namespace {
float Dist(const Vec3& a, const Vec3& b) {
    float dx = a.x - b.x, dy = a.y - b.y, dz = a.z - b.z;
    return std::sqrt(dx * dx + dy * dy + dz * dz);
}
}  // namespace

PuppetManager::PuppetManager(IPuppetEngine& engine, const Config& cfg, AnimConfig anim)
    : e_(engine), cfg_(cfg), animCfg_(anim) {}

// ---- lifecycle ------------------------------------------------------------------------------------------------------

PuppetManager::Puppet* PuppetManager::Find(Slot s) {
    auto it = puppets_.find(s);
    return it == puppets_.end() ? nullptr : &it->second;
}
const PuppetManager::Puppet* PuppetManager::Find(Slot s) const {
    auto it = puppets_.find(s);
    return it == puppets_.end() ? nullptr : &it->second;
}

FormId PuppetManager::ChooseTemplate(const Appearance& a) const {
    for (const TemplateEntry& t : cfg_.templates)
        if (t.race == a.race && t.female == a.female && t.base) return t.base;
    return cfg_.defaultTemplate;
}

bool PuppetManager::Budget(unsigned n) {
    if (avatarFaults().ignoreBudget) return true;
    if (callsLeft_ < n) {
        ++stats_.budgetDeferred;
        return false;
    }
    callsLeft_ -= n;
    return true;
}

bool PuppetManager::AddPlayer(Slot slot, const Appearance& a) {
    const AvatarFaults& F = avatarFaults();
    if (slot == cfg_.localSlot && !F.allowLocalSlot) {
        ++stats_.rejectedLocalSlot;
        Log("avatar: refused a puppet for the local slot");
        return false;
    }
    Puppet* old = Find(slot);
    if (!old && puppets_.size() >= cfg_.maxPuppets) return false;
    std::vector<EquipEntry> keepEquip;
    if (old) {
        ++stats_.replacedDuplicates;
        keepEquip = old->equip.Desired();
        if (F.duplicateSlotSpawns) {
            // broken twin: forget the old puppet without deleting it, so it stays in the world as an orphan
            old->ref = 0;
        } else {
            DeleteRef(*old, true);
        }
        puppets_.erase(slot);
    }
    Puppet& p = puppets_[slot];
    p.slot = slot;
    p.app = a;
    p.planner = AnimPlanner(animCfg_);
    p.equip.SetConfig(cfg_);
    if (!keepEquip.empty()) p.equip.SetDesired(keepEquip);
    return true;
}

void PuppetManager::RemovePlayer(Slot slot) {
    Puppet* p = Find(slot);
    if (!p) return;
    DeleteRef(*p, true);
    puppets_.erase(slot);
    if (!avatarFaults().staleMapEntries) staleMap_.erase(slot);
}

void PuppetManager::SetLocalSlot(Slot slot) {
    cfg_.localSlot = slot;
    if (!avatarFaults().allowLocalSlot) RemovePlayer(slot);
}

void PuppetManager::SetEquipment(Slot slot, const std::vector<EquipEntry>& wanted) {
    if (Puppet* p = Find(slot)) p->equip.SetDesired(wanted);
}

bool PuppetManager::SetAppearance(Slot slot, const Appearance& a) {
    Puppet* p = Find(slot);
    if (!p) return false;
    // another race / sex needs another template NPC: that is a new puppet (equipment is kept by AddPlayer)
    if (avatarFaults().appearanceUpdateRespawns || ChooseTemplate(a) != ChooseTemplate(p->app)) return AddPlayer(slot, a);
    p->app = a;
    p->appDirty = true;   // applied by the next Tick of a driving puppet, or by Configure of one that is still being set up
    ++stats_.appearanceUpdates;
    return true;
}

void PuppetManager::SetPose(Slot slot, const PuppetPose& pose) {
    Puppet* p = Find(slot);
    if (!p) return;
    p->pose = pose;
    p->pose.valid = true;
    p->havePose = true;
    p->poseFresh = true;
}

void PuppetManager::DeleteRef(Puppet& p, bool callEngine) {
    if (p.ref) {
        if (callEngine) e_.Delete(p.ref);
        ++stats_.deleted;
    }
    p.ref = 0;
    p.phase = (p.phase == Phase::Failed) ? Phase::Failed : Phase::Absent;
    p.haveApplied = false;
    p.posConfirmed = false;
    p.lastGroup = -1;
    p.mismatchSince = 0;
    p.haveSneak = p.haveDrawn = false;
    p.drawn = p.sneak = false;
    p.needSetup = true;
    p.planner.Reset();
    p.equip.Reset();
}

void PuppetManager::DespawnAll(bool callEngine) {
    for (auto& kv : puppets_)
        if (kv.second.ref) DeleteRef(kv.second, callEngine);
}

void PuppetManager::Purge() {
    for (RefId r : e_.FindTaggedPuppets()) {
        bool ours = false;
        for (auto& kv : puppets_)
            if (kv.second.ref == r) ours = true;
        if (ours) continue;
        e_.Delete(r);
        ++stats_.purged;
    }
}

// ---- game events ----------------------------------------------------------------------------------------------------

void PuppetManager::OnSaveBegin() {
    saving_ = true;
    if (avatarFaults().saveKeepsPuppets) return;
    for (auto& kv : puppets_)
        if (kv.second.ref) {
            DeleteRef(kv.second, true);
            ++stats_.saveDespawns;
        }
}

void PuppetManager::OnSaveEnd() {
    saving_ = false;
}

void PuppetManager::OnLoadBegin() {
    loading_ = true;
    ++stats_.loadResets;
    DespawnAll(false);   // the world is going away: refs are meaningless, never call into it
}

void PuppetManager::OnLoadEnd() {
    loading_ = false;
    worldEpoch_ = e_.WorldEpoch();
    haveEpoch_ = true;
    purgePending_ = true;
}

// ---- map ------------------------------------------------------------------------------------------------------------

RefId PuppetManager::RefOf(Slot slot) const {
    const Puppet* p = Find(slot);
    if (p && p->ref && (p->phase == Phase::Driving || p->phase == Phase::Loading3D)) return p->ref;
    if (avatarFaults().staleMapEntries) {
        auto it = staleMap_.find(slot);
        if (it != staleMap_.end()) return it->second;
    }
    return 0;
}

std::optional<Slot> PuppetManager::SlotOf(RefId ref) const {
    if (!ref) return std::nullopt;
    for (const auto& kv : puppets_)
        if (kv.second.ref == ref && (kv.second.phase == Phase::Driving || kv.second.phase == Phase::Loading3D)) return kv.first;
    if (avatarFaults().staleMapEntries)
        for (const auto& kv : staleMap_)
            if (kv.second == ref) return kv.first;
    return std::nullopt;
}

std::vector<std::pair<Slot, RefId>> PuppetManager::LivePuppets() const {
    std::vector<std::pair<Slot, RefId>> v;
    for (const auto& kv : puppets_) {
        RefId r = RefOf(kv.first);
        if (r) v.emplace_back(kv.first, r);
    }
    return v;
}

bool PuppetManager::ActualPose(Slot slot, CellId& cell, Vec3& pos) const {
    const Puppet* p = Find(slot);
    if (!p || !p->ref || p->phase != Phase::Driving) return false;
    if (!const_cast<IPuppetEngine&>(e_).GetPose(p->ref, cell, pos)) return false;
    // A puppet that is far from where this driver last put it, after it had been seen there, was pushed by the engine (the real
    // player or another body standing in it). Tick undoes that with one SetPos, so the interpolation buffer must not take the
    // pushed place as the start of an easing: it would ease the puppet back over hundreds of milliseconds, a visible teleport
    // out and back. A puppet that was never seen at its place (just spawned or moved) is still reported where it is.
    if (!avatarFaults().adoptPhysicsPush && p->haveApplied && p->posConfirmed && p->cell == cell &&
        Dist(pos, p->appliedPos) > cfg_.driftUnits) {
        pos = p->appliedPos;
        ++stats_.pushesIgnored;
    }
    return true;
}

Phase PuppetManager::PhaseOf(Slot slot) const {
    const Puppet* p = Find(slot);
    return p ? p->phase : Phase::Absent;
}

bool PuppetManager::EquipConverged(Slot slot) const {
    const Puppet* p = Find(slot);
    return p && p->equip.Converged();
}

int PuppetManager::LastGroup(Slot slot) const {
    const Puppet* p = Find(slot);
    return p ? p->lastGroup : -1;
}

std::string PuppetManager::StatusJson() const {
    std::string s = "{\"puppets\":[";
    bool first = true;
    for (const auto& kv : puppets_) {
        const Puppet& p = kv.second;
        char buf[480];
        // pose = where the buffer says it stands now, applied = what this driver last sent to the engine (both game units)
        std::snprintf(buf, sizeof buf,
                      "%s{\"slot\":%d,\"ref\":\"0x%08X\",\"phase\":\"%s\",\"group\":%d,\"cell\":\"0x%08X\",\"equipConverged\":%s,"
                      "\"pose\":[%.1f,%.1f,%.1f],\"applied\":[%.1f,%.1f,%.1f],\"haveApplied\":%s,\"posConfirmed\":%s}",
                      first ? "" : ",", (int)kv.first, (unsigned)p.ref, ToString(p.phase), p.lastGroup, (unsigned)p.cell,
                      p.equip.Converged() ? "true" : "false", p.pose.pos.x, p.pose.pos.y, p.pose.pos.z, p.appliedPos.x, p.appliedPos.y, p.appliedPos.z,
                      p.haveApplied ? "true" : "false", p.posConfirmed ? "true" : "false");
        s += buf;
        first = false;
    }
    char buf[900];
    std::snprintf(buf, sizeof buf,
                  "],\"stats\":{\"spawned\":%llu,\"spawnFailures\":%llu,\"deleted\":%llu,\"refLost\":%llu,\"setPos\":%llu,\"setYaw\":%llu,"
                  "\"playGroup\":%llu,\"groupRefused\":%llu,\"animReasserts\":%llu,\"restrainReasserts\":%llu,\"parked\":%llu,\"purged\":%llu,"
                  "\"saveDespawns\":%llu,\"equipOps\":%llu,\"equipGaveUp\":%llu,\"drift\":%llu,\"budgetDeferred\":%llu,\"load3DTimeouts\":%llu,\"pushesIgnored\":%llu,\"bodyYields\":%llu}}",
                  (unsigned long long)stats_.spawned, (unsigned long long)stats_.spawnFailures, (unsigned long long)stats_.deleted,
                  (unsigned long long)stats_.refLost, (unsigned long long)stats_.setPos, (unsigned long long)stats_.setYaw,
                  (unsigned long long)stats_.playGroup, (unsigned long long)stats_.groupRefused, (unsigned long long)stats_.animReasserts,
                  (unsigned long long)stats_.restrainReasserts, (unsigned long long)stats_.parked, (unsigned long long)stats_.purged,
                  (unsigned long long)stats_.saveDespawns, (unsigned long long)stats_.equipOps, (unsigned long long)stats_.equipGaveUp,
                  (unsigned long long)stats_.drift, (unsigned long long)stats_.budgetDeferred, (unsigned long long)stats_.load3DTimeouts,
                  (unsigned long long)stats_.pushesIgnored, (unsigned long long)stats_.bodyYields);
    s += buf;
    return s;
}

// The place to give the engine for a puppet that should stand at `want`: `want` itself, unless that is inside the body of this machine's own
// player (same cell, nearly the same height), in which case the nearest place at the edge of that body. `from` is where the puppet is now;
// it decides the side when the remote stands exactly where the local player does.
Vec3 PuppetManager::MakeRoom(CellId cell, const Vec3& want, const Vec3& from, bool haveFrom) {
    if (avatarFaults().noBodyRoom || cfg_.bodyRoomUnits <= 0) return want;
    if (!localRead_) {
        localRead_ = true;
        haveLocal_ = e_.GetLocalPose(localCell_, localPos_);
    }
    if (!haveLocal_ || localCell_ != cell) return want;
    if (std::fabs(want.z - localPos_.z) > cfg_.bodyRoomHeightUnits) return want;
    float dx = want.x - localPos_.x, dy = want.y - localPos_.y;
    float d = std::sqrt(dx * dx + dy * dy);
    if (d >= cfg_.bodyRoomUnits) return want;
    if (d < 1.0f) {
        dx = haveFrom ? from.x - localPos_.x : 0.0f;
        dy = haveFrom ? from.y - localPos_.y : 0.0f;
        d = std::sqrt(dx * dx + dy * dy);
        if (d < 1.0f) {
            dx = 1.0f;
            dy = 0.0f;
            d = 1.0f;
        }
    }
    ++stats_.bodyYields;
    Vec3 out = want;
    out.x = localPos_.x + dx / d * cfg_.bodyRoomUnits;
    out.y = localPos_.y + dy / d * cfg_.bodyRoomUnits;
    return out;
}

// ---- spawn / configure ----------------------------------------------------------------------------------------------

void PuppetManager::TrySpawn(Puppet& p, uint32_t now) {
    const AvatarFaults& F = avatarFaults();
    if (static_cast<int32_t>(now - p.nextSpawnAt) < 0) return;
    FormId tmpl = ChooseTemplate(p.app);
    if (!tmpl) {
        p.phase = Phase::Failed;
        Log("avatar: no template NPC for slot " + std::to_string((int)p.slot));
        return;
    }
    if (!Budget()) return;
    float yaw = F.usesPitchAsYaw ? p.pose.rot.x : WrapRad(p.pose.rot.y);
    const Vec3 at = MakeRoom(p.pose.cell, p.pose.pos, p.pose.pos, false);
    RefId r = e_.Spawn(tmpl, p.pose.cell, at, yaw);
    if (!r) {
        ++stats_.spawnFailures;
        ++p.attempts;
        p.backoffMs = p.backoffMs ? std::min(p.backoffMs * 2, cfg_.spawnBackoffMaxMs) : cfg_.spawnBackoffMs;
        p.nextSpawnAt = now + p.backoffMs;
        if (p.attempts >= cfg_.maxSpawnAttempts) {
            p.phase = Phase::Failed;
            Log("avatar: giving up on slot " + std::to_string((int)p.slot) + " after " + std::to_string(p.attempts) + " spawn attempts");
        }
        return;
    }
    p.ref = r;
    p.gen = nextGen_++;
    staleMap_[p.slot] = r;
    p.phase = Phase::Loading3D;
    p.spawnedAt = now;
    p.cell = p.pose.cell;
    p.appliedPos = at;
    p.appliedYaw = yaw;
    p.haveApplied = true;
    p.appliedAt = now;
    p.forceAnim = true;
    p.forcePose = true;
    p.needSetup = true;
    p.gen3d = 0;
    p.lastRestrainCheck = now;
    p.lastHealthCheck = now;
    p.lastHeal = now;
    p.backoffMs = 0;
    ++stats_.spawned;
    if (p.wasParked) {
        ++stats_.unparked;
        p.wasParked = false;
    }
    p.equip.Reset();
    p.planner.Reset();
    if (F.noWait3D) {
        Configure(p, now);   // broken twin: do not wait for 3D
        p.phase = Phase::Driving;
    }
}

bool PuppetManager::Configure(Puppet& p, uint32_t now) {
    if (!Budget(3)) return false;   // SetRestrained + MakeNonCombatant + ApplyAppearance
    if (!e_.SetRestrained(p.ref, true)) return false;   // refused: stay in Loading3D and retry next Tick
    e_.MakeNonCombatant(p.ref);
    if (p.needSetup) {
        e_.ApplyAppearance(p.ref, p.app);   // best effort: the template already has a plausible look
        p.needSetup = false;
        p.appDirty = false;
    }
    p.gen3d = e_.Generation3D(p.ref);
    p.lastRestrainCheck = now;
    p.forceAnim = true;
    p.forcePose = true;
    p.phase = Phase::Driving;
    return true;
}

// ---- per-frame driving ----------------------------------------------------------------------------------------------

void PuppetManager::Tick(uint32_t now) {
    callsLeft_ = cfg_.maxEngineCalls;
    localRead_ = false;
    if (loading_) return;
    if (!e_.WorldReady()) return;
    uint32_t ep = e_.WorldEpoch();
    if (!haveEpoch_) {
        worldEpoch_ = ep;
        haveEpoch_ = true;
        purgePending_ = true;   // the very first Tick of a session: a previous run may have left puppets in the save
    } else if (ep != worldEpoch_) {
        // A load happened without OnLoadBegin / OnLoadEnd (backstop): refs are gone, orphans may exist.
        worldEpoch_ = ep;
        ++stats_.loadResets;
        DespawnAll(false);
        purgePending_ = true;
    }
    if (purgePending_) {
        if (!avatarFaults().noLoadPurge) Purge();
        purgePending_ = false;
    }
    if (saving_ && !avatarFaults().noSaveWait) return;

    bool menu = e_.MenuOpen();
    bool menuClosedNow = prevMenu_ && !menu;
    prevMenu_ = menu;
    if (menu) return;

    // round robin start so a small call budget cannot starve the last slot
    std::vector<Slot> order;
    for (auto& kv : puppets_) order.push_back(kv.first);
    if (!order.empty()) std::rotate(order.begin(), order.begin() + (tickNo_ % order.size()), order.end());
    ++tickNo_;
    for (Slot slot : order) {
        Puppet& p = puppets_[slot];
        if (p.phase == Phase::Failed || !p.havePose) continue;
        if (p.poseFresh) {
            p.poseFresh = false;
            p.poseAt = now;
        }
        // ---- is the remote's cell loaded on this machine? ----
        bool loaded = e_.CellLoaded(p.pose.cell);
        if (!loaded && !avatarFaults().neverPark) {
            if (!p.unloaded) {
                p.unloaded = true;
                p.unloadedSince = now;
            }
            if (p.ref && now - p.unloadedSince >= cfg_.parkDelayMs) {
                DeleteRef(p, true);
                ++stats_.parked;
                p.wasParked = true;
            }
            if (!p.ref) continue;
            continue;   // grace period: hold still, do not drive into an unloaded cell
        }
        p.unloaded = false;

        switch (p.phase) {
            case Phase::Absent:
                TrySpawn(p, now);
                break;
            case Phase::Loading3D: {
                if (!avatarFaults().noStaleRefCheck && !e_.IsValid(p.ref)) {
                    ++stats_.refLost;
                    DeleteRef(p, false);
                    break;
                }
                if (e_.Has3D(p.ref)) {
                    Configure(p, now);
                } else if (now - p.spawnedAt > cfg_.load3DTimeoutMs) {
                    ++stats_.load3DTimeouts;
                    DeleteRef(p, true);
                    ++p.attempts;
                    p.nextSpawnAt = now + cfg_.spawnBackoffMs;
                    if (p.attempts >= cfg_.maxSpawnAttempts) p.phase = Phase::Failed;
                }
                break;
            }
            case Phase::Driving:
                Drive(p, now, menuClosedNow);
                break;
            case Phase::Failed:
                break;
        }
    }
}

void PuppetManager::Drive(Puppet& p, uint32_t now, bool menuClosedNow) {
    const AvatarFaults& F = avatarFaults();
    // 1. the ref must still be the object we spawned (ids are reused after a delete, cells unload refs)
    if (!F.noStaleRefCheck && !e_.IsValid(p.ref)) {
        ++stats_.refLost;
        DeleteRef(p, false);
        p.nextSpawnAt = now;
        return;
    }
    // 2. 3D present and unchanged. A rebuilt 3D (cell reload) drops animation and may drop the restraint.
    if (!F.noWait3D && !e_.Has3D(p.ref)) {
        p.phase = Phase::Loading3D;
        p.spawnedAt = now;
        return;
    }
    uint32_t g3 = e_.Generation3D(p.ref);
    if (g3 != p.gen3d) {
        p.gen3d = g3;
        if (!F.no3DGenReassert) {
            p.forceAnim = true;
            p.forcePose = true;
            p.lastRestrainCheck = 0;   // check the restraint right now
        }
    }
    // 2b. a puppet is a hit target: it must stay standing. A dead one is replaced, a hurt one is topped up.
    if (now - p.lastHealthCheck >= cfg_.healthCheckMs) {
        p.lastHealthCheck = now;
        if (!F.noPuppetDeathRespawn && e_.IsDead(p.ref)) {
            ++stats_.puppetDeaths;
            DeleteRef(p, true);
            p.nextSpawnAt = now + cfg_.deathRespawnMs;
            return;
        }
        if (!F.noPuppetHeal && cfg_.healMs && now - p.lastHeal >= cfg_.healMs && Budget()) {
            p.lastHeal = now;
            if (e_.ResetHealth(p.ref)) ++stats_.heals;
        }
    }
    // 3. restraint: AI takes a loose actor over (walks home, fights, sits down)
    if (!F.noRestrainReassert && now - p.lastRestrainCheck >= cfg_.restrainCheckMs) {
        p.lastRestrainCheck = now;
        if (!e_.IsRestrained(p.ref) && Budget()) {
            e_.SetRestrained(p.ref, true);
            ++stats_.restrainReasserts;
            p.forceAnim = true;
        }
    }
    // 4. menus reset animation (and the world was paused): replay and jump to the latest pose
    if (menuClosedNow) {
        if (!F.noMenuReassert) {
            p.forceAnim = true;
            ++stats_.menuReasserts;
        }
        if (!F.noPostMenuSnap) p.forcePose = true;
    }

    const PuppetPose& pose = p.pose;
    const Vec3 target = MakeRoom(pose.cell, pose.pos, p.appliedPos, p.haveApplied && p.cell == pose.cell);
    const float yaw = F.usesPitchAsYaw ? pose.rot.x : (F.noYawNormalize ? pose.rot.y : WrapRad(pose.rot.y));
    const bool stale = now - p.poseAt > cfg_.staleMs;
    const bool frozen = pose.frozen || stale;
    if (frozen && !p.frozenNow) ++stats_.staleFrozen;
    p.frozenNow = frozen;

    // 5. cell change: SetPos cannot move a ref between interiors / worldspaces, MoveToCell rebuilds the 3D
    if (pose.cell != p.cell) {
        bool sameSpace = e_.SameSpace(p.cell, pose.cell);
        if (!sameSpace && !F.crossCellSetPos) {
            if (!Budget()) return;
            if (e_.MoveToCell(p.ref, pose.cell, target, yaw)) {
                ++stats_.moveToCell;
                ++stats_.teleports;
                p.cell = pose.cell;
                p.appliedPos = target;
                p.appliedYaw = yaw;
                p.appliedAt = now;
                p.haveApplied = true;
                p.posConfirmed = false;
                p.phase = Phase::Loading3D;   // wait for the rebuilt 3D before anything else
                p.spawnedAt = now;
                p.forceAnim = true;
            }
            return;
        }
        p.forcePose = true;   // same space: a plain SetPos below moves it
    }

    // 6. position and heading. Pitch and roll are never applied (a pitched actor is a tilted body).
    bool needPos = p.forcePose || !p.haveApplied || pose.snapped || Dist(p.appliedPos, target) > cfg_.posEpsilon ||
                   now - p.appliedAt >= cfg_.posHeartbeatMs;
    CellId ac = 0;
    Vec3 ap;
    if (e_.GetPose(p.ref, ac, ap)) {
        if (Dist(ap, p.appliedPos) > cfg_.driftUnits) {
            ++stats_.drift;
            needPos = true;
        } else if (p.haveApplied && ac == p.cell) {
            p.posConfirmed = true;
        }
    }
    if (needPos && Budget()) {
        if (e_.SetPos(p.ref, target)) {
            ++stats_.setPos;
            if (pose.snapped) ++stats_.teleports;
            p.appliedPos = target;
            p.appliedAt = now;
            p.cell = pose.cell;
            p.haveApplied = true;
            if (pose.snapped) p.forceAnim = true;
        }
    }
    float dyaw = std::fabs(F.noYawNormalize ? (yaw - p.appliedYaw) : WrapRad(yaw - p.appliedYaw));
    if ((p.forcePose || dyaw > cfg_.yawEpsilonRad) && Budget()) {
        if (e_.SetYaw(p.ref, yaw)) {
            ++stats_.setYaw;
            p.appliedYaw = yaw;
        }
    }
    p.forcePose = false;

    // 7. stance flags
    if ((!p.haveSneak || p.sneak != pose.anim.sneak) && Budget()) {
        if (e_.SetSneak(p.ref, pose.anim.sneak)) {
            p.sneak = pose.anim.sneak;
            p.haveSneak = true;
        }
    }

    // 8. animation. The weapon counts as drawn only once it is equipped (draw with an empty hand = T-pose).
    PlanInput pi;
    pi.anim = pose.anim;
    if (!F.weaponDrawBeforeEquip && !p.equip.WeaponReady()) pi.anim.weaponDrawn = false;
    pi.vel = pose.vel;
    pi.yaw = yaw;
    pi.pitch = pose.rot.x;
    pi.nowMs = now;
    pi.frozen = frozen;
    PlanOut po = p.planner.Plan(pi);

    bool wantDrawn = po.weaponDrawn;
    if ((!p.haveDrawn || p.drawn != wantDrawn) && static_cast<int32_t>(now - p.drawRetryAt) >= 0 && Budget()) {
        if (e_.SetWeaponDrawn(p.ref, wantDrawn)) {
            p.drawn = wantDrawn;
            p.haveDrawn = true;
        } else {
            p.drawRetryAt = now + 1000;   // unsupported / refused: the Equip / Unequip groups carry the pose meanwhile
        }
    }

    int cur = e_.CurrentGroup(p.ref);
    bool changed = po.group != p.lastGroup;
    bool mismatch = false;
    if (cur >= 0 && p.lastGroup >= 0 && cur != p.lastGroup && !IsOneShot(p.lastGroup)) {
        if (!p.mismatchSince) p.mismatchSince = now ? now : 1;
        mismatch = now - p.mismatchSince >= cfg_.animMismatchMs;
    } else {
        p.mismatchSince = 0;
    }
    bool heartbeat = !F.noAnimHeartbeat && cur < 0 && now - p.lastGroupAt >= cfg_.animHeartbeatMs;
    if ((p.forceAnim || changed || mismatch || heartbeat) && Budget()) {
        if (e_.PlayGroup(p.ref, po.group, 1)) {
            ++stats_.playGroup;
            if (!changed) ++stats_.animReasserts;
            p.lastGroup = po.group;
            p.lastGroupAt = now;
            p.forceAnim = false;
            p.mismatchSince = 0;
        } else {
            ++stats_.groupRefused;   // retried next Tick
        }
    }

    // 8b. a look change that did not need a new puppet
    if (p.appDirty && Budget()) {
        e_.ApplyAppearance(p.ref, p.app);
        p.appDirty = false;
    }

    // 9. equipment
    if (callsLeft_ > 0 || F.ignoreBudget) {
        int n = p.equip.Tick(e_, p.ref, now, stats_);
        if (!F.ignoreBudget) callsLeft_ = n >= static_cast<int>(callsLeft_) ? 0 : callsLeft_ - n;
    }
}

}  // namespace nvcoop::avatar
