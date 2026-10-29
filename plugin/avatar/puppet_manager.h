// The puppet driver: one restrained stand-in actor per REMOTE player slot, driven from interpolated poses.
//
// Usage (game main thread only):
//   PuppetManager pm(engine, cfg);
//   pm.AddPlayer(slot, appearance);                  // on join (and after a reconnect: replaces the old puppet)
//   pm.SetEquipment(slot, {...});                    // on every published equipment change
//   each frame: pm.SetPose(slot, pose);              // pose from live::InterpBuffer::sample (live_bridge.h)
//               pm.Tick(nowMs);
//   game events: pm.OnSaveBegin()/OnSaveEnd(), pm.OnLoadBegin()/OnLoadEnd()  (Tick also polls WorldEpoch as a backstop)
//   pm.RefOf(slot) / pm.SlotOf(ref) is the ghost-ref map used by the acting-player design (plugin/coop bindPlayerRefs).
//
// The puppet traps this class handles are listed in docs/tools/avatar.md with the test that proves each one.
#pragma once
#include <functional>
#include <map>
#include <optional>
#include <string>
#include <vector>

#include "anim_planner.h"
#include "avatar_types.h"
#include "equip_corrector.h"
#include "puppet_engine.h"

namespace nvcoop::avatar {

class PuppetManager {
public:
    PuppetManager(IPuppetEngine& engine, const Config& cfg, AnimConfig anim = {});
    PuppetManager(const PuppetManager&) = delete;
    PuppetManager& operator=(const PuppetManager&) = delete;

    void SetLog(std::function<void(const std::string&)> log) { log_ = std::move(log); }

    // ---- lifecycle -----------------------------------------------------------------------------------------------
    // False when the slot is the local slot, or the puppet limit is reached. A second call for a known slot replaces
    // the old puppet (reconnect): the old ref is deleted, never left behind.
    bool AddPlayer(Slot slot, const Appearance& a);
    void RemovePlayer(Slot slot);
    // This machine's own slot, known once the session has assigned it (a client joins as slot 1, 2, ...; the ini default is 0). The local
    // slot never gets a puppet: a puppet that already exists for it is removed.
    void SetLocalSlot(Slot slot);
    Slot LocalSlot() const { return cfg_.localSlot; }
    void SetEquipment(Slot slot, const std::vector<EquipEntry>& wanted);
    // A changed look of a known player: same template (race / sex) = applied to the live puppet without a respawn; another template = a new
    // puppet (AddPlayer, equipment kept). False when the slot has no puppet.
    bool SetAppearance(Slot slot, const Appearance& a);
    void SetPose(Slot slot, const PuppetPose& pose);
    // A pose held over the planner's: while it is set the puppet plays `baseGroup` (it is played once, then held, not replayed on the heartbeat),
    // the owner's one-shots are not played, and a puppet the engine calls dead is not respawned (a downed friend lies there; docs/revive.md
    // "Downed pose"). kNoGroup clears it at once: the planned group plays on the next Tick. It belongs to the slot, not to the puppet, so a
    // respawn, a new look or a reconnect keeps it; RemovePlayer drops it. Known before the puppet exists too.
    void SetPoseOverride(Slot slot, int baseGroup);
    int PoseOverride(Slot slot) const;   // kNoGroup when none
    void Tick(uint32_t nowMs);

    // ---- game events ---------------------------------------------------------------------------------------------
    void OnSaveBegin();   // despawn every puppet: it must not be written into the save
    void OnSaveEnd();
    void OnLoadBegin();   // world is being torn down: drop all refs without touching the engine
    void OnLoadEnd();     // schedule the orphan purge; puppets respawn afterwards

    // ---- ghost-ref map -------------------------------------------------------------------------------------------
    RefId RefOf(Slot slot) const;                     // 0 = no usable puppet right now
    std::optional<Slot> SlotOf(RefId ref) const;
    std::vector<std::pair<Slot, RefId>> LivePuppets() const;   // every slot with a usable puppet right now, ascending by slot
    // Where the engine says the puppet really is (for live::InterpBuffer::sample's ActualPose). False when none. A place farther
    // than Config::driftUnits from the last applied one is a push, not a place to ease from: the applied place is reported.
    bool ActualPose(Slot slot, CellId& cell, Vec3& pos) const;

    // ---- introspection (tests, bridge) -----------------------------------------------------------------------------
    Phase PhaseOf(Slot slot) const;
    bool Has(Slot slot) const { return puppets_.count(slot) != 0; }
    size_t Count() const { return puppets_.size(); }
    const Stats& stats() const { return stats_; }
    bool PurgePending() const { return purgePending_; }
    bool EquipConverged(Slot slot) const;
    int LastGroup(Slot slot) const;
    std::string StatusJson() const;   // for the avatar.status bridge method

private:
    struct Puppet {
        Slot slot = kNoSlot;
        Appearance app;
        RefId ref = 0;
        Phase phase = Phase::Absent;
        uint64_t gen = 0;                  // bumps on every new ref, so stale per-ref state is easy to spot
        // spawn bookkeeping
        uint32_t nextSpawnAt = 0, spawnedAt = 0, backoffMs = 0;
        unsigned attempts = 0;
        // target and what was applied
        PuppetPose pose;
        uint32_t poseAt = 0;
        bool havePose = false;
        bool poseFresh = false;
        CellId cell = 0;                   // cell the puppet is in right now
        Vec3 appliedPos;
        float appliedYaw = 0;
        bool haveApplied = false;
        bool posConfirmed = false;         // the engine place was seen within driftUnits of appliedPos since the last (re)spawn / cell move
        uint32_t appliedAt = 0;
        bool forcePose = false;
        uint32_t unloadedSince = 0;
        bool unloaded = false;
        // animation
        AnimPlanner planner;
        int lastGroup = -1;
        uint32_t lastGroupAt = 0, mismatchSince = 0;
        bool forceAnim = false;
        // the owner's one-shots (attack, block, stagger ...): the counter last seen, whether it was read since the puppet last started driving,
        // the one-shot playing now (until when; a held one until the owner stops reporting it), and the engine refusals of the current one
        bool actKnown = false;
        uint8_t actSeq = 0;
        uint8_t actStaleSeq = 0;           // an older counter value that keeps being seen (a restarted owner), since when (0 = none)
        uint32_t actStaleSince = 0;
        bool actPlaying = false;
        int actHeld = 0;
        uint32_t actUntil = 0;
        unsigned actTries = 0;
        bool sneak = false, haveSneak = false;
        bool drawn = false, haveDrawn = false;
        uint32_t gen3d = 0;
        bool needSetup = true;             // appearance + first-time configuration still to do
        bool appDirty = false;             // the look changed while the puppet lives: ApplyAppearance again
        bool wasParked = false;
        uint32_t drawRetryAt = 0;
        bool frozenNow = false;
        uint32_t lastRestrainCheck = 0;
        uint32_t lastHealthCheck = 0, lastHeal = 0;
        // equipment
        EquipCorrector equip;
    };

    Puppet* Find(Slot s);
    const Puppet* Find(Slot s) const;
    void DeleteRef(Puppet& p, bool callEngine);
    void TrySpawn(Puppet& p, uint32_t now);
    bool Configure(Puppet& p, uint32_t now);
    void Drive(Puppet& p, uint32_t now, bool menuClosedNow);
    void EndOneShot(Puppet& p, uint32_t now, bool frozen);
    void PlayOneShot(Puppet& p, uint32_t now, bool frozen);
    void DespawnAll(bool callEngine);
    void Purge();
    FormId ChooseTemplate(const Appearance& a) const;
    bool Budget(unsigned n = 1);
    Vec3 MakeRoom(CellId cell, const Vec3& want, const Vec3& from, bool haveFrom);
    void Log(const std::string& s) const { if (log_) log_(s); }

    IPuppetEngine& e_;
    Config cfg_;
    AnimConfig animCfg_;
    std::map<Slot, Puppet> puppets_;
    std::map<Slot, int> poseOverride_;   // slot -> held base group; absent = none
    mutable Stats stats_;   // mutable: ActualPose (const) counts the pushes it hides
    std::function<void(const std::string&)> log_;
    uint64_t nextGen_ = 1;
    uint32_t worldEpoch_ = 0;
    bool haveEpoch_ = false;
    bool loading_ = false, saving_ = false, purgePending_ = false;
    bool prevMenu_ = false;
    unsigned callsLeft_ = 0;
    uint32_t tickNo_ = 0;
    bool localRead_ = false, haveLocal_ = false;   // the local player's place, read at most once per Tick
    CellId localCell_ = 0;
    Vec3 localPos_;
    std::map<Slot, RefId> staleMap_;   // only used by the staleMapEntries fault
};

}  // namespace nvcoop::avatar
