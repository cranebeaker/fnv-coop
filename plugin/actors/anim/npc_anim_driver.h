// Non-owner side of the NPC animation sync (enemy-anim): makes a remote-driven copy of an NPC LOOK right. The position / facing of the copy
// is applied elsewhere (the actor port, from the live::InterpBuffer); this driver adds everything the animation system needs on top:
// the movement group with speed matching, turn in place, weapon draw / holster, aim, sneak, jump, the owner's one-shots (attack, reload,
// stagger ...), knockdown, death and the corpse's resting place, plus the guards against the engine's traps (T-pose after load, animation
// lost after a menu or a cell change, commands refused on a restrained actor).
//
// Timing: it is handed the SAME InterpOut the position was taken from (100 ms behind the newest snapshot), so a one-shot starts when the
// interpolated body reaches the snapshot that announced it, and the movement group follows the DISPLAYED speed (what the body really does
// on this machine, including smoothing and catch-up), not only the owner's code - feet do not skate.
//
// Everything engine-facing is INpcAnimEngine; the driver counts every refusal and never throws.
#pragma once
#include <map>
#include <set>
#include <vector>

#include "anim_sit.h"
#include "corpse.h"
#include "npc_anim_config.h"
#include "npc_anim_engine.h"

namespace nvcoop::actors::anim {

struct DriverStats {
    uint64_t drives = 0, commands = 0, refused = 0, unsupported = 0, deferredBudget = 0, waiting3D = 0;
    uint64_t groupsPlayed = 0, loopChanges = 0, oneShotsPlayed = 0, oneShotsMissed = 0, oneShotsStale = 0;
    uint64_t reasserts = 0, groupMismatches = 0, tposeFixes = 0, rebuilds3D = 0, menuReasserts = 0, worldResets = 0, cellReasserts = 0;
    uint64_t weaponChanges = 0, weaponVerifyFailures = 0, weaponLadderGiveUps = 0, weaponConfirmed = 0, weaponDrifts = 0;
    uint64_t sneakChanges = 0, turnsPlayed = 0, jumpsPlayed = 0, rateChanges = 0;
    uint64_t knockdowns = 0, knockdownFallbacks = 0, getUps = 0;
    uint64_t deaths = 0, killRetries = 0, killGaveUp = 0, corpsePoses = 0, corpseCorrections = 0, corpseWithinTolerance = 0;
    uint64_t corpseGaveUp = 0, corpseMissing = 0, corpseLate = 0;
    uint64_t releases = 0;
    uint64_t sitsIssued = 0, standsIssued = 0, standsConfirmed = 0, standGiveUps = 0, sitDrops = 0, sitDrifts = 0, sitNoFurniture = 0, restrains = 0;
};

// What a test or the dashboard can ask about one driven actor.
struct PuppetInfo {
    bool known = false;
    int loopGroup = -1;            // the movement / idle group the driver last issued
    bool loopHold = true;
    bool weaponDrawn = false;      // what the driver last told the engine
    bool weaponConfirmed = false;  // the engine read it back
    bool sneak = false;
    float speed = 0;               // displayed horizontal speed (units/s)
    float rate = 1;
    int lastActionGroup = 0;
    uint8_t lastActionSeq = 0;
    bool turning = false;
    bool seated = false;           // the driver believes the copy is in the furniture it was told to use
    bool standPending = false;     // a stand-up was issued and the copy has not read "out of the furniture" yet
    uint32_t furniture = 0;
};
struct DeadInfo {
    bool known = false;
    bool verified = false;
    bool havePose = false;
    bool resolved = false;
    uint32_t corrections = 0;
};

class NpcAnimDriver {
public:
    NpcAnimDriver(INpcAnimEngine& engine, const DriverConfig& cfg = {}) : eng_(engine), cfg_(cfg) {}

    // Once per actor the port drives, right after it applied position and facing. `out.snapped` means a teleport / cell change.
    void drive(NetId id, const live::InterpOut& out, uint32_t nowMs);
    // The local engine was told the actor is dead (a network death, or the owner machine's own). `fromNetwork` = we applied it, so verify it.
    void onDeath(NetId id, uint32_t nowMs, bool fromNetwork);
    // The owner's settled resting place of a corpse (reliable event).
    void onCorpse(NetId id, const CorpsePose& pose, uint32_t nowMs);
    // The actor stops being driven here (we own it now): release the held animation, then forget it. forget() drops all state silently.
    void release(NetId id);
    void forget(NetId id);
    // The reference went away locally (cell unloaded): the live-copy state goes, a death and the corpse pose we are waiting for stay.
    void unload(NetId id) { puppets_.erase(id); }
    void revive(NetId id);   // scripted resurrect: the dead state is cleared and the copy is driven again
    // Once per frame (after the port's work): world events, verification timers, corpse correction. Resets the command budget.
    void tick(uint32_t nowMs);

    PuppetInfo info(NetId id) const;
    DeadInfo deadInfo(NetId id) const;
    // A stand-up is under way: the copy is still in the furniture. The actor port may keep the copy where it is until this turns false, so a
    // body that is told "standing" is out of the furniture before it is moved (the driver itself plays no movement group until then).
    bool poseLocked(NetId id) const;
    bool isDead(NetId id) const { return dead_.count(id) != 0; }
    std::vector<NetId> deadActors() const { std::vector<NetId> v; for (auto& kv : dead_) v.push_back(kv.first); return v; }
    const DriverStats& stats() const { return stats_; }
    std::vector<NetId> driven() const;
    uint32_t lastDriveMs(NetId id) const;
    const DriverConfig& config() const { return cfg_; }

private:
    enum class Cmd : uint8_t { Ok, Deferred, Refused, Unsupported };
    enum class JumpPhase : uint8_t { None, Start, Loop, Land };
    // Exponential backoff after a refused command: base, 2x, 4x ... capped at 8 s. A success clears it.
    struct Backoff {
        uint32_t fails = 0, retryAt = 0;
        bool active = false;
        bool ready(uint32_t now) const { return !active || static_cast<int32_t>(now - retryAt) >= 0; }
        void fail(uint32_t now, uint32_t baseMs) {
            fails = fails < 5 ? fails + 1 : 5;
            uint32_t d = baseMs << (fails - 1);
            if (d > 8000u) d = 8000u;
            retryAt = now + d; active = true;
        }
        void clear() { fails = 0; active = false; }
    };
    struct Puppet {
        // identity / baseline
        bool started = false;           // the baseline (idle + stance) has been issued after the 3D appeared
        bool creature = false;
        uint32_t gen3D = 0;
        CellId cell = 0;
        bool haveCell = false;
        uint32_t lastDriveMs = 0;
        // displayed motion
        bool havePos = false;
        live::Vec3 lastPos;
        uint32_t lastPosMs = 0;
        live::Vec3 vdisp;               // smoothed displayed horizontal velocity
        float speed = 0;
        bool moving = false;
        float lastYaw = 0;
        float yawRate = 0;              // smoothed rad/s, + = clockwise
        float turnAccum = 0;            // signed rad turned while the yaw rate is above the exit threshold
        uint32_t turnSince = 0;
        bool turning = false;
        // locomotion decision
        bool runNow = false;
        int dirPrev = -1;
        bool disagreeing = false;
        uint32_t disagreeSince = 0;
        uint8_t wantLoco = 0;
        // issued state
        int loop = -1;                  // the movement / idle group last issued (-1 = nothing yet)
        bool loopHold = true;
        uint32_t loopAt = 0;
        Backoff loopBo;
        float rate = 1.0f;
        bool rateUnsupported = false;
        Backoff rateBo, aimBo;
        bool sneak = false, sneakKnown = false;
        Backoff sneakBo;
        // weapon
        bool weaponWant = false, weaponKnown = false;
        bool weaponConfirmed = false;
        uint32_t weaponAt = 0;
        uint32_t weaponTries = 0;
        uint32_t weaponRetryAt = 0;
        bool haveWeaponRetry = false;
        bool aimApplied = false;
        // transitions
        int transient = -1;
        uint32_t transientUntil = 0;
        bool prevJump = false;
        JumpPhase jump = JumpPhase::None;
        uint32_t jumpUntil = 0;
        bool prevKnocked = false;
        bool prevTalk = false;
        uint32_t getUpUntil = 0;
        // one-shots
        bool haveSeq = false;
        uint8_t lastSeq = 0;
        int actionGroup = 0;            // last one-shot played
        uint32_t actionUntil = 0;       // a reaction (stagger ...) is assumed to play until here: the movement group is not forced over it
        int heldAction = 0;             // a held action (block, automatic fire) is playing
        bool afterActionReassert = false;
        // furniture: what the driver believes the copy is doing in the furniture, and what the last read-back said
        enum class Seat : uint8_t { Unknown, Standing, Seated };
        Seat seat = Seat::Unknown;
        uint32_t seatFurniture = 0;
        uint32_t sitCmdAt = 0;          // when the last sit / stand command went out (a read-back before the grace is not judged)
        bool standPending = false;      // stand-up issued, the copy is not out yet: no movement group is played over it
        uint32_t standSince = 0, standTries = 0, standRetryAt = 0, pendingReadAt = 0;
        bool unrestrained = false;      // an escalated stand-up let the actor's AI run: the hold has to go back
        Backoff sitBo, standBo;
        bool poseHeld = false;          // the furniture owns the pose right now (seated, or on the way out): no movement group, no re-assert
        bool sitReadable = false;       // the backend can read the sit state of this copy
        bool obsFresh = false;
        SitView obs;
        uint32_t obsFurniture = 0;
        bool noFurnitureCounted = false;
        // verification
        uint32_t nextVerifyMs = 0;
        bool dirty = true;              // re-issue everything at the next chance (menu closed, cell change ...)
    };
    struct Dead {
        uint32_t since = 0;
        bool fromNetwork = false;
        bool verified = false;
        uint32_t killTries = 0;
        uint32_t nextKillMs = 0;
        bool havePose = false;
        CorpsePose pose;
        uint32_t poseAt = 0;
        bool resolved = false;          // corrected, or found within tolerance
        uint32_t corrections = 0;
        uint32_t nextCorrectMs = 0;
        bool haveLast = false;
        Vec3 lastPos;
        uint32_t lastPosMs = 0;
        bool resting = false;
        uint32_t restSince = 0;
        bool missingCounted = false;
    };

    bool spend(uint32_t cost, bool essential);
    Cmd note(Result r);
    Cmd play(NetId id, int group, bool hold, bool essential);
    void resetApplied(Puppet& p);
    void measure(Puppet& p, const live::InterpOut& out, uint32_t now);
    int pickLoopGroup(Puppet& p, const live::InterpOut& out, uint32_t now, bool& hold);
    void applyWeapon(NetId id, Puppet& p, const live::AnimState& a, uint32_t now);
    void issueWeapon(NetId id, Puppet& p, uint32_t now);
    void applyOneShot(NetId id, Puppet& p, const live::AnimState& a, uint32_t now);
    void applyKnock(NetId id, Puppet& p, const live::InterpOut& out, uint32_t now);
    bool applySeat(NetId id, Puppet& p, const live::AnimState& a, uint32_t now);
    void observeSeat(Puppet& p, const NpcReads& r);
    void verifyPuppet(NetId id, Puppet& p, uint32_t now);
    void tickDead(NetId id, Dead& d, uint32_t now);
    void markAllDirty();
    bool blocked(uint32_t now);   // a menu is up or no game is loaded (cached per millisecond: drive() runs per actor per frame)

    INpcAnimEngine& eng_;
    DriverConfig cfg_;
    std::map<NetId, Puppet> puppets_;
    std::map<NetId, Dead> dead_;
    std::map<NetId, std::pair<CorpsePose, uint32_t>> earlyCorpse_;   // owner's pose that arrived before we knew the actor is dead
    DriverStats stats_;
    uint32_t budgetUsed_ = 0;
    uint32_t worldEpoch_ = 0;
    bool haveWorld_ = false;
    bool menuWas_ = false;
    bool blockedCache_ = false, haveBlocked_ = false;
    uint32_t blockedAt_ = 0;
    bool noRate_ = false, noWeapon_ = false, noAim_ = false, noKnock_ = false, noSeat_ = false;   // the backend answered Unsupported once: never asked again
};

}  // namespace nvcoop::actors::anim
