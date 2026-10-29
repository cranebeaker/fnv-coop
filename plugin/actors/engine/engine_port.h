// EnginePort: the real IActorPort (plugin/actors/actor_types.h) over live actor references, built on IActorEngine so every path is unit-tested
// against a fake engine. See docs/tools/actor-port.md for the design, the AI-off method and the engine traps.
//
//   * Identity     : ActorSync speaks NetId; the port keeps NetId <-> RefId (the real actor and, when there is one, a clone that stands in for it).
//   * AI switch    : SetRestrained 1 (+ StopCombat first) for every actor this machine does not own; SetRestrained 0 (+ EvaluatePackage, then
//                    StartCombat on the last known target) for the owner. Each change goes into the ledger, which is persisted.
//   * Driving      : drive() only records the wanted pose; flush() (once per frame) applies it with epsilons, a per-actor rate cap and a
//                    per-frame console-line budget, oldest first, so a crowd cannot starve the frame and nobody starves.
//   * Never        : Respawn / Reset3D / recreate on a handoff (STR #906). The only objects ever created are stand-in clones and leveled-pick
//                    clones, and those are tracked in the ledger so a save/load cannot keep them.
#pragma once
#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "actors/actor_types.h"
#include "actors/engine/actor_engine_api.h"
#include "actors/engine/engine_iface.h"
#include "actors/engine/ledger.h"

namespace nvcoop::actors::engine {

class EnginePort : public IActorPort {
public:
    EnginePort(IActorEngine& eng, Ledger& ledger, const Config& cfg = Config());

    void setConfig(const Config& c) { cfg_ = c; }
    const Config& config() const { return cfg_; }

    // ---- frame ----
    void beginFrame(uint64_t nowMs);   // clock, line budget, per-frame caches
    void flush();                      // apply pending drives (budgeted), retry kills / revives, write the ledger

    // ---- feed side ----
    // The real actor `ref` is loaded here. Idempotent for the same (netId, ref); a different ref for the same netId replaces it.
    void bindReal(const ActorMeta& meta, RefId ref);
    void unbindReal(NetId id);         // the real ref is gone; a stand-in stays
    bool bound(NetId id) const;        // an entry with a live ref (real or clone) exists
    RefId realRef(NetId id) const;
    RefId activeRef(NetId id) const;   // the ref that represents the actor now (pick clone > real > stand-in)
    NetId netOfRef(RefId ref) const;
    bool isOurClone(RefId ref) const;  // stand-in or pick clone made by this port
    bool isStandInRef(RefId ref) const;
    std::vector<NetId> ids() const;
    size_t standInCount() const;
    // The actor's last known AI state, for a takeover (also set by place()). Used by the feed after a sample.
    void rememberAi(NetId id, const live::AiState& ai);
    // Combat targets: NetId -> ref for StartCombat (default: the port's own directory, then the player resolver in the registry).
    RefId resolveTarget(NetId target) const;

    // ---- world / session ----
    void forgetWorld();                // the world was replaced: forget every entry and the live-change set, no engine calls
    void releaseAll();                 // the session is over: undo every change this port made in the live world, then forget it
    // A save is about to be written: remove pick clones (enabling the real actors again) and refuse new clones for Config::saveGraceMs.
    // Returns the actors whose pick swap was reverted; the caller tells ActorSync their native base is back so the swap is redone later.
    std::vector<NetId> beforeSave();
    // Undo ledger entries that do not belong to this world epoch (left by a save, a crash or an earlier session). Returns entries undone.
    size_t undoStale();
    size_t staleCount() const;
    // `ref` is disabled because a pick clone replaced it, but no clone stands in for it any more (its cell reloaded): enable it again.
    bool reclaimDisabled(RefId ref);
    void sweepOrphans();               // live clones without an entry (their cell unloaded first) are removed when reachable
    bool cloneHold() const { return now_ < cloneHoldUntilMs_; }

    // ---- IActorPort ----
    bool loaded(NetId id) override;
    bool cellLoaded(CellId cell) override;
    bool sample(NetId id, live::ActorSnapshot& out) override;
    bool pose(NetId id, live::ActualPose& out) override;
    void drive(NetId id, const live::InterpOut& out) override;
    void applyAv(NetId id, const std::vector<live::AvValue>& values) override;
    size_t auditAv(NetId id, const std::vector<live::AvValue>& expected) override;   // Health only: a read and, when it differs from the owner's, one write
    void place(NetId id, const live::ActorSnapshot& authoritative) override;
    void setAi(NetId id, bool on) override;
    void setProtected(NetId id, bool on) override;
    bool spawnStandIn(const ActorMeta& meta, const live::ActorSnapshot& at) override;
    void removeStandIn(NetId id) override;
    void setBase(NetId id, const FormRef& pick) override;
    void kill(NetId id) override;
    void killRef(RefId r);   // kill (or retry) with the credit of a remote player when plugin/combat names one
    void revive(NetId id) override;
    bool sampleInventory(NetId id, InvList& out) override;
    bool applyInventory(NetId id, const InvList& want) override;
    bool sampleEquipment(NetId id, EquipList& out) override;
    bool applyEquipment(NetId id, const EquipList& want) override;

    // ---- diagnostics ----
    const PortStats& stats() const { return st_; }
    bool aiOff(NetId id) const;
    bool isProtected(NetId id) const;
    bool pendingDrive(NetId id) const;
    uint64_t now() const { return now_; }

private:
    enum class CloneKind : uint8_t { None, StandIn, PickSwap };
    struct Entry {
        NetId id = 0;
        ActorMeta meta;
        RefId real = 0;
        RefId clone = 0;
        CloneKind kind = CloneKind::None;
        uint32_t cloneBase = 0;
        bool aiOff = false;
        RefId aiRef = 0;                 // the ref WE restrained (0 = none, e.g. a script already held it)
        bool protOn = false, protBaseEssential = false;
        RefId protRef = 0;
        live::AiState lastAi;
        bool haveAi = false;
        // driving
        bool pending = false;
        live::InterpOut want;
        bool haveApplied = false;
        live::Vec3 appliedPos;
        float appliedYaw = 0;
        CellId appliedCell = 0;
        uint64_t appliedMs = 0;
        // sampling
        live::AiState sampledAi;           // what the AI was doing at the last sample taken while it ran (the handoff's final state is sampled after AI-off)
        bool haveSampledAi = false;
        bool havePrev = false;
        live::Vec3 prevPos, prevVel;
        uint64_t prevMs = 0;
        // kill / revive retry
        int killTries = 0, reviveTries = 0;
        bool killWanted = false, reviveWanted = false;
        uint64_t nextTryMs = 0;
    };

    Entry* find(NetId id) { auto it = e_.find(id); return it == e_.end() ? nullptr : &it->second; }
    const Entry* find(NetId id) const { auto it = e_.find(id); return it == e_.end() ? nullptr : &it->second; }
    RefId activeOf(const Entry& e) const;
    void eraseIfEmpty(NetId id);
    void index(NetId id, RefId ref);
    void unindex(RefId ref);

    // ledger with a live/stale split
    void ledgerAdd(LedgerKind k, RefId ref);
    void ledgerRemove(LedgerKind k, RefId ref);
    // undoStale: a stale created-ref (0xFF) entry whose id now names something that is not a restrained actor (the id was reused
    // after a load / restart) is dropped from the ledger and true is returned; the engine is not touched.
    bool staleCreatedRefIsForeign(LedgerKind k, uint32_t ref);

    bool linesLeft() const;
    uint64_t lines() const { return eng_.linesRun(); }
    bool applyPose(Entry& e, const live::InterpOut& want, bool force);
    bool writePose(Entry& e, RefId r, CellId cell, const live::Vec3& pos, float yaw, bool force, bool allAxes);
    void applyAvs(Entry& e, RefId r, const std::vector<live::AvValue>& v);
    void applyAi(Entry& e, bool on);
    bool deleteRef(RefId r);          // disable + mark for delete; false = the engine refused (retried by sweepOrphans)
    bool makeClone(Entry& e, CloneKind kind, uint32_t baseId, RefId anchor, CellId cell, const live::Vec3& pos, float yaw);
    // `immediate`: delete now (a save is about to be written, a session ends); otherwise the ref waits Config::deleteQuietMs after its last
    // write in `doomed_` (the engine may still hold a queued SetPos for its id: in game 2026-10-22 it ran one on a recycled id and crashed).
    void dropClone(Entry& e, bool reenableReal, bool immediate = false);
    void retireClone(RefId c, bool immediate);   // take a clone out of the world: now, or once the quiet period has passed
    void reapDoomed(bool all);         // delete the quiet-period refs that are due (all: now, whatever the clock says)
    bool isDoomed(RefId r) const;
    bool sameSpace(CellId a, CellId b);

    IActorEngine& eng_;
    Ledger& led_;
    Config cfg_;
    std::map<NetId, Entry> e_;
    std::map<RefId, NetId> byRef_;
    std::set<std::pair<char, uint32_t>> live_;     // ledger entries made in this world epoch
    std::map<CellId, bool> cellCache_;
    uint64_t now_ = 0;
    uint64_t frameStartLines_ = 0;
    uint64_t lastSweepMs_ = 0, cloneHoldUntilMs_ = 0;
    std::map<NetId, uint64_t> spawnRetryAt_;        // stand-in attempts that failed: not before this time
    std::vector<std::pair<RefId, uint64_t>> doomed_;   // removed clones waiting out the quiet period: (ref, delete not before)
    PortStats st_;
};

}  // namespace nvcoop::actors::engine
