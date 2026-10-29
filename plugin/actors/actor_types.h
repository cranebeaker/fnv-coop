// Actor ownership in the plugin (Milestone 3, FULL-SYNC-PLAN 3.2): shared types and the two ports ActorSync talks through.
// Game-free. The engine is reached only through IActorPort (in-game: the hooks glue; tests: a fake engine) and the network only
// through IActorNet (in-game: the coop mux; tests: an in-memory net with loss and duplication).
#pragma once
#include <array>
#include <cstdint>
#include <functional>
#include <string>
#include <vector>
#include "fnvcoop/ownership.h"
#include "fnvcoop/transport.h"
#include "fnvcoop/types.h"
#include "live/live.h"

namespace nvcoop::actors {

namespace live = fnvcoop::live;
using fnvcoop::Bytes;
using fnvcoop::FormRef;
using fnvcoop::OwnerEpoch;
using fnvcoop::PlayerId;
using fnvcoop::kHostPlayer;
using fnvcoop::kNoPlayer;
using live::CellId;
using live::NetId;

// First byte of every datagram this module sends through the coop MuxTransport (0xFC story, 0xD1 dialogue, 0xE5 slides, 0xA1 reserved).
constexpr uint8_t kChannelActors = 0xAC;

// What the module knows about one actor it syncs. `base` is the form the engine actor currently uses (it changes when a leveled pick is
// swapped in); `named` marks story NPCs (death only through a host KillActor effect).
struct ActorMeta {
    NetId netId = 0;
    FormRef ref;
    FormRef base;
    bool named = false;
    bool created = false;        // spawned at run time (SpawnRef); a stand-in clone is never "created"
    CellId cell = 0;
    live::Vec3 pos;
};

// NetId of a placed or created ref: stable on every machine, never zero, never in the 0xFF created-id range.
// (Created refs arrive with a host-allocated id and do not use this.) Collisions are possible in theory; the host refuses a second ref
// with a taken id (Stats::idCollisions).
NetId deriveNetId(const FormRef& ref);

// One entry of an actor's inventory (totals: base container + changes). A list is sorted by (plugin, localId); count > 0.
struct InvItem {
    FormRef form;
    int32_t count = 0;
    friend bool operator==(const InvItem& a, const InvItem& b) { return a.form == b.form && a.count == b.count; }
    friend bool operator!=(const InvItem& a, const InvItem& b) { return !(a == b); }
};
using InvList = std::vector<InvItem>;
// What an actor holds and wears (weapon, armour, clothing): its equipped item forms, sorted, no duplicates.
using EquipList = std::vector<FormRef>;

// State parts (state_parts.h): the non-motion state of an actor the owner's engine changes by itself. On the wire (MPart::kind): append only.
enum class PartKind : uint8_t {
    Inventory = 0,   // item totals (InvList)
    Equipment = 1,   // what it holds and wears (EquipList)
};
constexpr size_t kPartCount = 2;
inline size_t partIndex(PartKind k) { return static_cast<size_t>(k); }
struct PartSpec {
    uint32_t pollMs = 500;     // owner: read the part of an owned actor this often
    uint32_t settleMs = 1000;  // owner: a changed value is published once it stayed the same this long
    size_t maxBytes = 4096;    // a value larger than this is never sent (an engine read that went wrong)
    bool reapplyOnLoad = false;   // copy: write the last known value again when the actor loads here again (the engine rebuilt it from its base form)
};
struct PartStats {
    uint64_t published = 0, applied = 0, rejected = 0, duplicate = 0, applyFailed = 0;
};

// One player as the ownership service needs to see them (fed by the avatar layer).
struct PlayerPose {
    bool present = false;
    CellId cell = 0;
    live::Vec3 pos;
    std::vector<CellId> adjacent;   // loaded neighbouring cells
};

// ---- the engine side ---------------------------------------------------------------------------------------------------------
// Everything is keyed by NetId. A stand-in clone and the real actor share a NetId: only one of them exists on a machine at a time.
// There is deliberately no Respawn / Reset3D call: a handoff must never recreate the actor (the STR #906 lesson).
class IActorPort {
public:
    virtual ~IActorPort() = default;
    virtual bool loaded(NetId id) = 0;                                    // a live ref (real or stand-in) exists here
    virtual bool cellLoaded(CellId cell) = 0;
    // Owner: native state. pos/rot/vel/anim/ai and the actor values that changed since the last call (absolute values).
    virtual bool sample(NetId id, live::ActorSnapshot& out) = 0;
    virtual bool pose(NetId id, live::ActualPose& out) = 0;               // where the ref really is right now (for smoothing)
    // Non-owner: show this pose (restrained / AI-off actor). out.snapped = teleport, no blending.
    virtual void drive(NetId id, const live::InterpOut& out) = 0;
    virtual void applyAv(NetId id, const std::vector<live::AvValue>& values) = 0;
    // New owner: put the actor exactly where the old owner left it (full authoritative state, merged AVs). No respawn, no Reset3D.
    virtual void place(NetId id, const live::ActorSnapshot& authoritative) = 0;
    virtual void setAi(NetId id, bool on) = 0;                            // owner true; everyone else false (AI-off / restrained)
    virtual void setProtected(NetId id, bool on) = 0;                     // named NPC: local damage cannot kill it
    virtual bool spawnStandIn(const ActorMeta& meta, const live::ActorSnapshot& at) = 0;   // clone for an actor with no local ref
    virtual void removeStandIn(NetId id) = 0;
    virtual void setBase(NetId id, const FormRef& pick) = 0;              // leveled pick: swap the base form to match
    virtual void kill(NetId id) = 0;                                      // apply death (called only when authorised)
    virtual void revive(NetId id) = 0;                                    // scripted resurrect (host ResurrectActor effect)
    // Inventory (the owner's AI drinks a Stimpak, the other copies must lose it too). Defaults: not available, nothing is synced.
    virtual bool sampleInventory(NetId id, InvList& out) { (void)id; (void)out; return false; }   // false = cannot read it exactly now
    virtual bool applyInventory(NetId id, const InvList& want) { (void)id; (void)want; return false; }   // make the engine copy hold exactly `want`
    // Equipment (the owner's AI draws a rifle from its pack: the copies must hold the same one, or they draw nothing and cower). Defaults: not available.
    virtual bool sampleEquipment(NetId id, EquipList& out) { (void)id; (void)out; return false; }   // false = cannot read it now
    virtual bool applyEquipment(NetId id, const EquipList& want) { (void)id; (void)want; return false; }   // make the engine copy wear exactly `want`
    // The generic form ActorSync uses: one part's value in its canonical bytes (state_parts.h). The defaults go through the typed calls above.
    virtual bool readPart(NetId id, PartKind kind, Bytes& out);
    virtual bool writePart(NetId id, PartKind kind, const Bytes& value);
    // Non-owner copy: check the mirrored actor values against `expected` (the owner's, merged) and write the ones that differ. Returns how many it had to correct.
    // An AV is mirrored once, when it changes; an engine that puts the actor's damage somewhere else while the actor is not fully loaded gives the copy another
    // health once it loads (vats room behind bad_wifi, run 20261028-032920: the copy read -55 for the owner's 5 until the area digest healed it). Default: write all.
    virtual size_t auditAv(NetId id, const std::vector<live::AvValue>& expected) { applyAv(id, expected); return 0; }
};

// ---- the network side --------------------------------------------------------------------------------------------------------
class IActorNet {
public:
    virtual ~IActorNet() = default;
    // Client: peer is always the host. Host: any joined client. `data` already starts with kChannelActors.
    virtual void send(PlayerId peer, bool reliable, Bytes data) = 0;
};

struct SyncConfig {
    PlayerId self = kHostPlayer;
    bool isHost = true;
    fnvcoop::OwnershipConfig own;
    live::InterpConfig interp;
    live::InterestConfig interest;
    live::SenderConfig sender;
    uint32_t ownEvalMs = 250;      // host: re-decide every tracked actor this often
    uint32_t settleMs = 300;       // owner: an idle actor sends one reliable settle snapshot after this long
    uint32_t resyncMinMs = 500;    // client: at most one MResync per actor per this long
    // State parts (state_parts.h). Inventory settles for 1 s: a replicated AddItem / RemoveItem has landed on every machine by then, so it is not
    // applied twice. Equipment settles for 0.3 s: long enough for a weapon swap in the middle of a draw, short enough that the copy draws the same gun. Equipment is written again when a copy loads again (a cell reload gives an NPC its base outfit back); inventory is not.
    std::array<PartSpec, kPartCount> parts{{PartSpec{500, 1000, 4096, false}, PartSpec{500, 300, 1024, true}}};
    uint32_t avAuditMs = 500;      // non-owner: check a copy's mirrored actor values against the owner's this often (0 = never); under the area digest's settle time (1 s)
    // Switches for an A/B run in game (nvcoop.ini [actors] norepeatfinal=1 nocommitnofinal=1 noavaudit=1 give the behaviour before the lab run of 2026-10-28; a key at 0 is "absent" to the lab's ini check, so the switches are off-by-default); all on by default.
    bool acceptRepeatedFinal = true;   // new owner: a final state that only repeats what the copy already holds still completes the handoff (placed, acknowledged)
    bool commitWithoutFinal = true;    // host: a handoff the host has no last state for commits at once instead of after the timeout
    uint32_t holdAiMaxMs = 4000;   // safety net: a new owner that committed without the old owner's final state waits at most this long before it drives the AI
    std::function<bool(CellId, CellId)> adjacent;   // neighbouring exterior cells (default: same cell only)
};

struct OwnerInfo {
    bool known = false;            // the ref maps to a tracked actor
    NetId netId = 0;
    PlayerId owner = kNoPlayer;    // committed owner on this machine (kNoPlayer = nobody)
    OwnerEpoch epoch = 0;
    bool pending = false;          // a handoff is in flight
    PlayerId pendingTo = kNoPlayer;
    PlayerId self = kNoPlayer;
    bool mine = false;             // owner == self
    bool aiOn = false;             // what this machine told the engine (AI on = we simulate it)
    bool standIn = false;
    bool dead = false;             // died here (the ownership record is kept so a resurrect continues the epoch history)
};

struct Stats {
    uint64_t published = 0;            // snapshots this machine sampled from owned actors
    uint64_t publishedNotOwner = 0;    // must stay 0
    uint64_t snapshotsApplied = 0, snapshotsDropped = 0;
    uint64_t staleEpochDropped = 0, notOwnerDropped = 0, futureEpoch = 0, unknownActor = 0, staleSeqDropped = 0;
    uint64_t finalsApplied = 0, finalsSent = 0, finalsRepeated = 0, acksSent = 0, takeovers = 0, handoffsOut = 0;   // finalsRepeated: a final that only repeated a snapshot already held (still completes the handoff)
    uint64_t standInsMade = 0, standInsRemoved = 0;
    uint64_t deathsApplied = 0, namedDeathsBlocked = 0, killsAuthorised = 0;
    uint64_t picksApplied = 0, picksRejected = 0;
    uint64_t resyncsSent = 0, resyncsAnswered = 0, idCollisions = 0, undecodable = 0, spoofed = 0;
    uint64_t ownershipApplied = 0, ownershipDuplicates = 0, ownershipStale = 0;
    uint64_t recruiterChanges = 0;
    std::array<PartStats, kPartCount> parts{};   // MPart, by PartKind
    uint64_t animEventsSent = 0, animEventsApplied = 0, animEventsRejected = 0, animEventsDuplicate = 0;   // enemy-anim MAnimEvent
    uint64_t avAudits = 0, avDriftFixed = 0;   // non-owner copies checked / actor values that had drifted from the owner's and were written back
    uint64_t aiHolds = 0, aiHoldsReleased = 0, staleFinalsForwarded = 0;   // forced-commit hold of the new owner's AI (see ActorSync::onOwnApplied)
};

}  // namespace nvcoop::actors
