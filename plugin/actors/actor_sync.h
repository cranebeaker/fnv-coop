// ActorSync: the ownership service wired into one machine (FULL-SYNC-PLAN 3.2, milestone 3). Game-free; one instance per machine.
//
//   Host   : runs fnvcoop::OwnershipService (decides who owns what, hysteresis, follower exception) and applies its own messages to its
//            OwnershipView like everybody else. Relays snapshots to each client at the interest rate (live::InterestManager).
//   Owner  : the machine whose view says it owns an actor runs the native AI and publishes snapshots (live::SnapshotSender, unreliable)
//            stamped with the owner epoch. Nobody else may publish (counted in Stats::publishedNotOwner, must stay 0).
//   Others : the actor's AI is OFF; its pose is driven from a live::InterpBuffer. An actor with no local ref gets a stand-in clone.
//   Handoff: Transfer(e+1) -> old owner stops AI, sends its final state (reliable) -> new owner places that state (no respawn, no
//            Reset3D) and acks -> host commits -> new owner turns AI on. Every step is idempotent (view results Duplicate/Stale ignored).
//   Named NPCs: owned like any actor for movement; they die only through a host KillActor effect (ActorSync::onEffect).
//   Leveled pick: the first owner report wins on the host and is broadcast; everyone swaps the base form to match.
#pragma once
#include <functional>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <vector>
#include "actors/actor_types.h"
#include "actors/actor_wire.h"
#include "fnvcoop/effect.h"

namespace nvcoop::actors {

class ActorSync {
public:
    ActorSync(const SyncConfig& cfg, IActorPort& port, IActorNet& net);
    ~ActorSync();

    PlayerId self() const { return cfg_.self; }
    bool isHost() const { return cfg_.isHost; }

    // ---- session (host: joined clients; client: link to the host) ----
    void playerJoined(PlayerId slot);   // host: a client joined; it is sent every actor, pick and ownership assignment
    void playerLeft(PlayerId slot);     // host: its actors are released and re-decided at once
    void hostConnected();               // client: the link to the host is up (re-reports every locally tracked actor)
    void hostLost();                    // client: forget ownership, drop stand-ins' drivers
    void setPlayerPose(PlayerId slot, const PlayerPose& p);   // from the avatar layer (every player, incl. self)

    // ---- engine events ----
    bool track(const ActorMeta& meta);          // the engine has this actor loaded here (cell load / spawn). false = id collision
    void untrack(NetId id);                     // the actor is gone for everybody (destroyed / despawned); host releases ownership
    void onActorLoaded(NetId id);               // the REAL ref appeared (a stand-in for it is removed)
    void onActorUnloaded(NetId id);             // the ref went away locally; the network record stays
    bool onLocalDeath(NetId id);                // the engine is about to kill it here: false = do not (not the owner / named NPC)
    void onNativeBase(NetId id, const FormRef& base);   // the engine rolled / changed the leveled base of this actor
    void onEffect(const fnvcoop::Effect& e);    // story effects applied on this machine: KillActor, ResurrectActor, CompanionOwner
    void purgeStandIns();                       // game load / session end: stand-ins are never saved
    void onWorldReloaded();                     // game load: forget the old world's deaths (owners and epochs stay; the engine is the truth again)

    // ---- enemy-anim (plugin/actors/anim): reliable owner-to-all events with an opaque payload (the settled corpse pose) ----
    struct AnimEventIn { NetId id = 0; PlayerId from = kNoPlayer; OwnerEpoch epoch = 0; uint64_t eventId = 0; Bytes payload; };
    using AnimEventHandler = std::function<void(const AnimEventIn&)>;
    void setAnimEventHandler(AnimEventHandler h) { animHandler_ = std::move(h); }
    // Sends `payload` to every other machine. Refused (false) unless this machine is the actor's committed owner (a dead actor keeps its owner record).
    // The host judges a client's event like any owner message (owner + epoch), relays it, and also delivers it locally.
    bool sendAnimEvent(NetId id, const Bytes& payload);

    // ---- network ----
    void receive(PlayerId from, const uint8_t* data, size_t size);   // one datagram from the mux (starts with kChannelActors)
    void tick(uint64_t nowMs);

    // ---- queries ----
    OwnerInfo ownerOf(NetId id) const;
    bool netIdOf(const FormRef& ref, NetId& out) const;
    bool isStandIn(NetId id) const;
    std::vector<NetId> standIns() const;
    bool isDead(NetId id) const;
    bool tracked(NetId id) const { return recs_.count(id) != 0; }
    size_t trackedCount() const { return recs_.size(); }
    std::vector<NetId> ownedActors() const;
    FormRef pickOf(NetId id) const;                  // empty plugin = none yet
    FormRef baseOf(NetId id) const;                  // what this machine believes the engine uses
    bool latestState(NetId id, live::ActorSnapshot& out) const;
    uint64_t ownershipDigest() const { return view_.digest(); }
    const fnvcoop::OwnershipView& view() const { return view_; }
    const fnvcoop::OwnershipService* service() const { return svc_.get(); }
    const Stats& stats() const { return stats_; }
    const live::SenderStats& senderStats() const { return sender_.stats(); }

private:
    struct Rec {
        explicit Rec(const live::InterpConfig& ic) : interp(ic) {}
        ActorMeta meta;
        live::InterpBuffer interp;
        bool local = false;                    // the engine tracked it here (vs. a record learned from the host)
        bool reported = false;                 // client: MTrack sent
        bool haveLatest = false;
        live::ActorSnapshot latest;            // newest accepted (or sampled) state
        std::map<uint8_t, int32_t> av;         // merged actor values
        OwnerEpoch lastEpoch = 0;
        uint32_t lastSeq = 0;
        bool active = false;                   // we run its AI (takeover done)
        OwnerEpoch activeEpoch = 0;
        bool aiKnown = false, aiApplied = false;
        bool handingOff = false;               // we gave it away: AI off, final sent
        bool finalApplied = false;             // incoming handoff: the old owner's final state is placed
        PlayerId awaitPrev = kNoPlayer;        // forced commit: the previous owner may still be driving it; our AI stays off until it proves it stopped
        uint64_t awaitSince = 0;
        bool engineAuthoritative = false;      // the engine's own state is the truth (we owned it and never drove it since)
        OwnerEpoch placedEpoch = 0;
        uint32_t placedSeq = 0;
        bool placedAny = false;
        bool standIn = false;
        bool dead = false, killAuthorized = false, protectedOn = false, killPending = false;
        bool havePick = false, pickReported = false;
        FormRef pick;
        // owner-side publishing
        uint64_t lastPublishMs = 0, lastChangeMs = 0;
        bool published = false, settled = false;
        live::ActorSnapshot lastPub;
        uint64_t lastResyncMs = 0;
        bool resyncedOnce = false;
        uint64_t avAuditAt = 0;               // non-owner: next check of the copy's actor values against the owner's
        // inventory (the owner publishes what its AI used or picked up; everyone else copies it)
        InvList invShared;                    // the inventory everyone agrees on: the owner's last published list, or the last one received
        bool invBase = false;                 // owner: invShared was taken from the engine (a silent baseline: worlds start equal)
        bool invPublishedAny = false;         // a list was published or received: a joining client needs it
        bool invPendingValid = false;
        InvList invPending;                   // owner: a changed list waiting to stay the same for invSettleMs
        uint64_t invPendingAt = 0, invNextMs = 0;
        bool invWantValid = false;            // non-owner: a received list the engine copy does not hold yet (not loaded, or the write was refused)
        InvList invWant;
        int invTries = 0;
    };

    // ownership glue
    bool ownedByMe(const Rec& r) const;
    void emitOwn(const std::vector<fnvcoop::OwnMsg>& out);
    void onOwnApplied(const fnvcoop::OwnMsg& m, fnvcoop::ApplyResult r);
    void maybeAck(Rec& r);
    void evalOwnership();
    std::vector<fnvcoop::Presence> presenceFor(const Rec& r) const;
    void takeOver(Rec& r, OwnerEpoch epoch);
    void loseOwnership(Rec& r);
    void releaseAiHold(Rec& r);
    void onStaleFinal(Rec& r, const Envelope& e, const live::ActorSnapshot& s);
    bool playerHere(PlayerId p) const;
    void syncToView(Rec& r);
    void refreshAi(Rec& r);
    live::ActorSnapshot fullState(const Rec& r) const;
    void placeBest(Rec& r);

    // snapshots
    void publishOwned(Rec& r, uint32_t now);
    bool sendFull(Rec& r, bool final, bool settle);   // false = nothing was sent (not the owner, or no state to send)
    bool acceptSnapshot(Rec& r, const live::ActorSnapshot& s, uint32_t now);
    void onLiveBatch(PlayerId from, const uint8_t* data, size_t size, uint32_t now);
    bool onSnapFull(const Envelope& e, const MSnapFull& m, uint32_t now);
    void driveNonOwner(Rec& r, uint32_t now);
    void auditAvs(Rec& r);                            // non-owner: the copy's mirrored actor values still match the owner's
    uint32_t ownerPeriod(const Rec& r) const;
    void relayToClients(uint32_t now);
    void flushPeers(uint32_t now);

    // other messages
    void sendJoinState(PlayerId slot);
    void dispatch(PlayerId from, const Envelope& e, uint32_t now);
    void onTrack(PlayerId from, const MTrack& m);
    void onDeath(const Envelope& e, const MDeath& m);
    void onPick(const MPick& m);
    void onPickReport(const Envelope& e, const MPickReport& m);
    void onResync(const Envelope& e, const MResync& m);
    void onAnimEvent(const Envelope& e, const MAnimEvent& m);
    void onInventory(const Envelope& e, const MInventory& m);
    void pollInventory(Rec& r);                       // owner: read, debounce, publish
    void flushInventory(Rec& r);                      // owner: publish a changed list now (death, handoff)
    bool publishInventory(Rec& r, const InvList& cur);
    void applyWantedInventory(Rec& r);                // non-owner: write the last received list when the actor is here
    void applyDeath(Rec& r, bool engineKills);
    void requestResync(Rec& r);
    void hostRegister(Rec& r);
    void ensureProtected(Rec& r);
    void considerPick(Rec& r);
    void hostSetPick(Rec& r, const FormRef& pick);
    void applyPick(Rec& r, const FormRef& pick);
    void ensureStandIn(Rec& r);
    void dropStandIn(Rec& r);

    // plumbing
    Rec* find(NetId id) { auto it = recs_.find(id); return it == recs_.end() ? nullptr : &it->second; }
    const Rec* find(NetId id) const { auto it = recs_.find(id); return it == recs_.end() ? nullptr : &it->second; }
    Rec* ensureRec(const ActorMeta& m, bool local);
    void sendBody(Body b, PlayerId dest);                  // client: always to the host; host: direct or fan-out
    void relay(const Envelope& e, PlayerId except);        // host: forward a client's broadcast
    std::vector<PlayerId> peers() const;
    bool cellsAdjacent(CellId a, CellId b) const;

    SyncConfig cfg_;
    IActorPort& port_;
    IActorNet& net_;
    fnvcoop::OwnershipView view_;
    std::unique_ptr<fnvcoop::OwnershipService> svc_;       // host only
    live::SnapshotSender sender_;
    live::SnapshotReceiver receiver_;
    live::SeqAllocator seqs_;
    live::InterestManager interest_;
    std::map<NetId, Rec> recs_;
    std::map<FormRef, NetId> refToNet_;
    std::map<NetId, FormRef> pendingPick_;                 // pick known before the actor was tracked here
    std::map<FormRef, PlayerId> pendingRecruiter_;         // host: CompanionOwner effect that arrived before the actor was tracked
    std::map<std::pair<PlayerId, NetId>, uint32_t> relayAt_;   // host: fallback rate limit for observers without a known pose
    std::map<PlayerId, PlayerPose> players_;
    std::set<PlayerId> joined_;                            // host: joined clients
    std::set<NetId> acked_;
    std::set<uint64_t> deathEvents_;
    std::set<uint64_t> animEvents_;                        // de-duplication of MAnimEvent (bounded, oldest ids dropped)
    AnimEventHandler animHandler_;
    uint64_t animEventCounter_ = 0, invEventCounter_ = 0;
    std::set<uint64_t> invEvents_;                         // de-duplication of MInventory
    uint32_t invBudget_ = 0;                               // reads left in this tick
    uint32_t avAuditBudget_ = 0;                           // copies the actor values of which may be read in this tick
    uint64_t now_ = 0, nextEvalMs_ = 0, eventCounter_ = 0;
    bool hostUp_ = true;                                   // client: link to the host
    Stats stats_;
};

}  // namespace nvcoop::actors
