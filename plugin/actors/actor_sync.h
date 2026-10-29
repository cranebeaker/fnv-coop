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
#include <array>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <vector>
#include "actors/actor_types.h"
#include "actors/actor_wire.h"
#include "fnvcoop/effect.h"
#include "live/stream_rate.h"

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
    // Player entities (docs/entity-core-plan.md S3). The host keeps one record per player: id live::playerNet(slot), meta.ref
    // live::playerFormRef(slot), owner pinned to the slot (never evaluated, never handed off). Its own player at start, a client when it joins
    // (playerJoined does it); playerLeft releases the pin and sends MUntrack. Clients learn the records from the host. The NPC paths (AI,
    // takeover, placing, picks, stand-ins, protection, parts, the stream) never touch a player record: the avatar layer drives puppets.
    bool isPlayerRecord(NetId id) const;
    // ---- player entities: parts and stream (docs/entity-core-plan.md S5, S10) ----
    // `port` answers every call for a player entity (live::isPlayerNet): the local player's parts and samples, a puppet's writes, the playback of a stream. Without
    // one a player record is only an identity. `parts`: the player parts (Look, Equipment, WeaponState, Facts, Pack) are replicated here; `stream`: the player's
    // snapshots go through the player stream profile (own sender and receiver, SubBatchPlayer / SubAckPlayer) instead of the avatar channel. Both are read every
    // tick, so the live layer may switch them on once per session. Nothing of an NPC changes.
    void enablePlayerEntities(IActorPort* port, bool parts, bool stream);
    std::vector<PlayerId> playerSlots() const;               // the slots of every player record this machine knows (the roster)
    StreamProfile profileFor(NetId id) const;                // the policy of an entity's stream
    // The puppet (or the local player) of a player entity was made again: every part is written onto it again (the "applied" memory is cleared).
    // Same call as onActorLoaded for an NPC; the live layer makes it when the entity map binds a new ref.
    void onPlayerBound(NetId id);
    bool partValue(NetId id, PartKind kind, Bytes& out) const;   // the value everybody agrees on (owner: last published, others: last received)

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

    // ---- the event bus (docs/entity-core-plan.md S6, wire MEvent) ----
    struct EventIn { NetId id = 0; uint8_t kind = 0; PlayerId from = kNoPlayer; PlayerId origin = kNoPlayer; OwnerEpoch epoch = 0; uint64_t eventId = 0;
                     bool toOwner = false; bool nack = false; Bytes payload; };
    using EventHandler = std::function<void(const EventIn&)>;
    void setEventHandler(uint8_t kind, EventHandler h) { eventHandlers_[kind] = std::move(h); }
    // Owner to everyone. Refused (false) unless this machine is the entity's committed owner and is not handing it off.
    bool sendOwnerEvent(NetId id, uint8_t kind, const Bytes& payload);
    // To the entity's current owner (delivered here at once when that is us). The host routes it, re-routes it while the owner changes,
    // and answers a Nack to the origin when the entity has no owner. false: the entity is unknown here, or the payload is too large.
    bool sendToOwner(NetId id, uint8_t kind, const Bytes& payload);

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
    const live::SenderStats& playerSenderStats() const { return senderP_.stats(); }
    const live::ReceiverStats& playerReceiverStats() const { return receiverP_.stats(); }
    const live::StreamRatePlanner& playerPlanner() const { return planner_; }

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
        // state parts (state_parts.h): the owner publishes what its engine changed by itself; everyone else copies it
        struct Part {
            Bytes shared;                     // the value everyone agrees on: the owner's last published value, or the last one received
            bool base = false;                // owner: `shared` was taken from the engine (a silent baseline: worlds start equal)
            bool publishedAny = false;        // a value was published or received: a joining client needs it
            bool pendingValid = false;
            Bytes pending;                    // owner: a changed value waiting to stay the same for the part's settle time
            uint64_t pendingAt = 0, nextMs = 0;
            bool wantValid = false;           // non-owner: a received value the engine copy does not hold yet (not loaded, or the write was refused)
            Bytes want;
            int tries = 0;
        };
        std::array<Part, kPartCount> parts;
        bool player = false;                  // a player entity (S3): identity, ownership, and (S5, S10) its parts and its stream when the live layer enables them
    };

    struct Early { PlayerId from = kNoPlayer; live::ActorSnapshot s; uint64_t at = 0; };
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

    // the player stream (StreamProfile of a player; plugin docs S10)
    void samplePlayer(uint32_t now);                  // owner: the local player's newest snapshots from the port, stamped and queued
    void queuePlayer(PlayerId peer, PlayerId subject, const live::ActorSnapshot& s, uint32_t now);   // through the rate planner; peer kNoPlayer = the uplink
    void releasePlayerPending(uint32_t now);
    void flushPlayerPeers(uint32_t now);
    void onPlayerBatch(PlayerId from, const uint8_t* data, size_t size, uint32_t now);
    void acceptPlayerSnapshot(Rec& r, const live::ReceivedSnapshot& res, PlayerId from, uint32_t now);
    std::vector<PlayerId> uplinkObservers() const;
    void applyEarlyPlayer(Rec& r, const Early& e, uint32_t now);

    // other messages
    void sendJoinState(PlayerId slot);
    void trackPlayer(PlayerId slot);                  // host
    void untrackPlayer(PlayerId slot);                // host, after the service released the pin
    void onUntrack(const MUntrack& m);                // client
    void dropRecord(NetId id);
    void holdEarly(PlayerId from, const live::ActorSnapshot& s);
    void feedEarly(NetId id, uint32_t now);           // a held snapshot whose record and ownership are known now
    void dispatch(PlayerId from, const Envelope& e, uint32_t now);
    void onTrack(PlayerId from, const MTrack& m);
    void onDeath(const Envelope& e, const MDeath& m);
    void onPick(const MPick& m);
    void onPickReport(const Envelope& e, const MPickReport& m);
    void onResync(const Envelope& e, const MResync& m);
    void onAnimEvent(const Envelope& e, const MAnimEvent& m);
    void onEvent(const Envelope& e, const MEvent& m);
    void deliverEvent(const MEvent& m, PlayerId from);
    void routeToOwner(MEvent m, PlayerId from);   // host
    void retryHeldEvents();
    // state parts (state_parts.h), one generic replicator for every PartKind
    void onPart(const Envelope& e, const MPart& m);
    IActorPort& portFor(NetId id);                    // the player port for a player entity, the engine port for everything else
    const PartSpec& specFor(const Rec& r, size_t k) const { return r.player ? cfg_.playerPartSpecs[k] : cfg_.parts[k]; }
    bool partApplies(const Rec& r, size_t k) const { return r.player ? partOfPlayer(k) : partOfNpc(k); }
    void tickPlayerRec(Rec& r);                       // a player record's whole tick: its parts (the NPC tick never runs on it)
    void resetOwnParts();                             // client: the host (maybe a new one) learns every part of the local player again
    void pollParts(Rec& r);                           // owner: read, debounce, publish
    void flushParts(Rec& r);                          // owner: publish every changed part now (death, handoff)
    bool publishPart(Rec& r, PartKind k, const Bytes& cur);
    void rewantParts(Rec& r);                         // copy: a new engine object (clone, or the real ref coming back) stands for the actor: write the known values again
    void applyWantedParts(Rec& r);                    // non-owner: write the last received values when the actor is here
    bool readPart(Rec& r, PartKind k, Bytes& out);    // the engine's value, or false (unreadable / too large)
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
    live::SnapshotSender senderP_;                         // the player stream's own pair (a player never waits behind an NPC burst in a packet)
    live::SnapshotReceiver receiverP_;
    live::StreamRatePlanner planner_;
    IActorPort* playerPort_ = nullptr;
    uint64_t lastAckP_ = 0;
    live::SeqAllocator seqs_;
    live::InterestManager interest_;
    std::map<NetId, Rec> recs_;
    std::map<FormRef, NetId> refToNet_;
    std::map<NetId, FormRef> pendingPick_;                 // pick known before the actor was tracked here
    std::map<FormRef, PlayerId> pendingRecruiter_;         // host: CompanionOwner effect that arrived before the actor was tracked
    std::map<std::pair<PlayerId, NetId>, uint32_t> relayAt_;   // host: fallback rate limit for observers without a known pose
    std::map<PlayerId, PlayerPose> players_;
    std::map<NetId, Early> early_;                         // the newest snapshot of a record not known yet (S3): a late join must not lose it
    std::set<PlayerId> joined_;                            // host: joined clients
    std::set<NetId> acked_;
    std::set<uint64_t> deathEvents_;
    std::set<uint64_t> animEvents_;                        // de-duplication of MAnimEvent (bounded, oldest ids dropped)
    AnimEventHandler animHandler_;
    std::map<uint8_t, EventHandler> eventHandlers_;
    std::set<uint64_t> busSeen_;                           // de-duplication of MEvent deliveries
    struct HeldEvent { Envelope e; uint64_t at = 0; };
    std::vector<HeldEvent> heldEvents_;                    // events for an entity this machine does not know yet
    uint64_t busCounter_ = 0;
    uint64_t animEventCounter_ = 0, partEventCounter_ = 0;
    std::set<uint64_t> partEvents_;                        // de-duplication of MPart
    uint32_t partBudget_ = 0;                              // part reads left in this tick
    uint32_t playerPartBudget_ = 0;                        // ... for player records (they sort after every NPC in the map and must not starve)
    uint32_t avAuditBudget_ = 0;                           // copies the actor values of which may be read in this tick
    uint64_t now_ = 0, nextEvalMs_ = 0, eventCounter_ = 0;
    bool hostUp_ = true;                                   // client: link to the host
    Stats stats_;
};

}  // namespace nvcoop::actors
