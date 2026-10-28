// plugin/actors/engine: the PUBLIC surface of the real actor port (FULL-SYNC-PLAN 3.2). Other lanes (combat-hooks, enemy-anim, spawn-arb,
// avatar-port, dll-wiring) include only this header. Game-free: no xNVSE header, no Windows header, so game-free tests can link it.
//
//   Identity   : RefId  = the runtime form id of a loaded reference (xNVSE `refID`, what a console "ref" resolves to).
//                NetId  = the cross-machine id ActorSync uses (live::NetId, derived from plugin + local id, or host-issued for 0xFF refs).
//   Ownership  : ownerOf()/isRemoteDriven() answer from this machine's committed ownership view (ActorSync::ownerOf). A stand-in clone is
//                always remote-driven.
//   Events     : callbacks for "an actor became tracked / died / went away / is being driven". Registered once, called on the game main
//                thread only, in registration order. A listener may be removed from inside its own callback.
//   Hits       : onLocalHit() is the forwarding hook point: the combat glue reports every hit the LOCAL engine produced; the answer says
//                whether the victim's owner is another machine (then the glue forwards it and must not apply local damage).
//
// Threading: everything here is main-thread only (the game's frame loop). Nothing is thread safe.
//
// Status: unit-tested game-free (plugin/actors/engine/tests). The engine side (engine_real.cpp) has never run in a game.
#pragma once
#include <cstdint>
#include <functional>
#include <string>
#include <vector>

#include "actors/actor_types.h"
#include "fnvcoop/effect.h"

namespace nvcoop::actors::engine {

using RefId = uint32_t;                       // runtime form id of a loaded reference; 0 = none
constexpr RefId kLocalPlayerRef = 0x14;       // PlayerCharacter reference

// ---- ownership ----------------------------------------------------------------------------------------------------------------
struct OwnerRef {
    bool known = false;                       // the ref maps to a tracked actor
    NetId netId = 0;
    PlayerId ownerSlot = kNoPlayer;           // committed owner on this machine; kNoPlayer (255) = nobody yet
    OwnerEpoch epoch = 0;
    bool mine = false;                        // this machine owns it: its native AI runs here
    bool pending = false;                     // a handoff is in flight (nobody drives it right now)
    bool standIn = false;                     // the ref is a stand-in clone, never the real actor
    bool dead = false;
};

OwnerRef ownerOf(RefId ref);                  // known=false for the player, unloaded refs and untracked actors
OwnerRef ownerOfNet(NetId id);
// True when this machine does NOT run the actor's AI and shows a copy driven from snapshots: a stand-in, or a tracked actor owned by
// someone else (or in a handoff). The combat glue uses it to refuse local damage / local death on such actors.
bool isRemoteDriven(RefId ref);
NetId netIdOf(RefId ref);                     // 0 = not a tracked actor
RefId refOf(NetId id);                        // the live ref (real or stand-in); 0 = none loaded here
bool isStandIn(RefId ref);

// ---- hits: forwarding hook point ---------------------------------------------------------------------------------------------
// What the LOCAL engine produced for one hit (xNVSE onhit data). Raw engine ids; combat-hooks converts to synccore/combat types.
struct LocalHitInfo {
    RefId attacker = 0;                       // 0 = unknown / environment
    RefId victim = 0;
    uint32_t weaponForm = 0;                  // runtime form id, 0 = none
    uint32_t ammoForm = 0;
    float damage = 0;                         // health damage the local engine computed
    int8_t limb = -1;                         // body part index, -1 = none
    live::Vec3 impact;                        // world position of the impact
    bool projectile = false, explosion = false, melee = false;
    uint32_t tag = 0;                         // free for the caller (e.g. a local sequence number)
};
// How a local hit relates to the actor ownership picture.
enum class HitRoute : uint8_t {
    Untracked,        // the victim is not a synced actor (player, scenery, unloaded): the caller decides on its own
    LocalOwner,       // the victim is mine: resolve here
    ForwardToOwner,   // the victim is owned by another machine (or a stand-in): hand the hit to its owner, do NOT apply local damage
    NoOwner,          // tracked but nobody owns it yet / handoff in flight: the caller queues or applies locally, its policy
};
struct HitRouting {
    HitRoute route = HitRoute::Untracked;
    OwnerRef victim;                          // ownership of the victim
    NetId attackerNet = 0;                    // 0 when the attacker is not a tracked actor (players have their own ids, see setPlayerNetIdResolver)
};
HitRouting routeLocalHit(const LocalHitInfo& hit);       // pure classification, no side effect
// Receiver of ForwardToOwner hits (set by combat-hooks): it sends the hit to the victim's owner over the reliable channel.
using HitForwarder = std::function<void(const LocalHitInfo& hit, const HitRouting& routing)>;
void setHitForwarder(HitForwarder f);
// Convenience used by the engine hook: classifies the hit and, for ForwardToOwner, calls the forwarder. Returns true when the hit was
// forwarded (the engine hook must then NOT apply the damage locally). False for every other route.
bool onLocalHit(const LocalHitInfo& hit);

// ---- deaths ----------------------------------------------------------------------------------------------------------------
// The kill path calls this just before the engine would kill/mark dead a tracked actor. Returns false = do not kill it here
// (not the owner, or a named NPC without a host KillActor). True for untracked refs and for authorised deaths. The port itself never
// calls it for kills it applies on behalf of the network.
bool onLocalDeath(RefId ref);

// ---- events ----------------------------------------------------------------------------------------------------------------------
using ListenerId = uint32_t;                  // 0 = invalid
void removeListener(ListenerId id);

struct TrackedEvent {
    NetId netId = 0;
    RefId ref = 0;
    fnvcoop::FormRef formRef;                 // plugin + local id of the placed ref (or the host-issued id of a created one)
    fnvcoop::FormRef base;
    CellId cell = 0;
    bool named = false, created = false, standIn = false;
};
// An actor the feed handed to ActorSync::track(), or a stand-in clone that was created for it (standIn=true).
ListenerId addActorTrackedListener(std::function<void(const TrackedEvent&)> fn);

struct DiedEvent {
    NetId netId = 0;
    RefId ref = 0;
    bool authorised = true;                   // the death was allowed by the ownership rules (owner, or host KillActor for a named NPC)
    bool fromNetwork = false;                 // applied here because another machine declared it (port.kill)
    bool mine = false;                        // this machine owns the actor
};
ListenerId addActorDiedListener(std::function<void(const DiedEvent&)> fn);

struct LostEvent { NetId netId = 0; RefId ref = 0; bool standIn = false; };
// The ref went away locally (cell unloaded, stand-in removed). The network record stays.
ListenerId addActorLostListener(std::function<void(const LostEvent&)> fn);

// Non-owner animation driving (enemy-anim): called for every actor ActorSync drives, after the port applied position and facing.
struct DriveEvent {
    NetId netId = 0;
    RefId ref = 0;
    bool standIn = false;
    const live::InterpOut* out = nullptr;     // valid only during the callback; carries anim + ai state of the owner
};
ListenerId addDriveListener(std::function<void(const DriveEvent&)> fn);

// Owner side: the port fills pos, rot, vel, cell, AVs, weaponDrawn, sneak, aim, combat target, package and alert itself. enemy-anim may
// refine/replace snap.anim (locomotion code, jump, reload ...) here; it runs last. `snap.netId` is set.
using AnimSampler = std::function<void(RefId ref, live::ActorSnapshot& snap)>;
void setAnimSampler(AnimSampler f);

// Players are not tracked actors: the avatar layer tells the port which NetId stands for a player ref (the local player or a puppet), so
// combat targets between actors and players survive the wire. Without it a player target is sent as 0 (none).
using PlayerNetIdResolver = std::function<NetId(RefId ref)>;      // 0 = not a player ref
using PlayerRefResolver = std::function<RefId(NetId id)>;         // 0 = not a player id
void setPlayerNetIdResolver(PlayerNetIdResolver byRef, PlayerRefResolver byNet);

// G04 kill attribution: asked when this machine kills a COPY of an actor another machine declared dead (the owner's death message). Returns the engine ref of the puppet of the
// REMOTE player that killed it (plugin/combat knows from the last hit), 0 = no remote player (a plain kill: the local player or the environment). The kill then runs as
// `KillActor <puppet>` (nvhooks::killWithAttribution) so the engine stores the puppet as the killer before OnDeath and `IsKiller player` is true for that guest.
using KillerResolver = std::function<RefId(RefId victimRef)>;
void setKillerResolver(KillerResolver f);

// The avatar layer reports every player's place (the local one is read by the port itself). Forwarded to ActorSync::setPlayerPose().
void setPlayerPose(PlayerId slot, const PlayerPose& pose);

// Refs that must NEVER be synced as NPCs: player puppets (plugin/avatar), NV:MP ghosts, anything another layer drives. The avatar layer
// registers its puppet refs here; a tracked actor that becomes excluded is dropped at the next scan.
using RefFilter = std::function<bool(RefId ref)>;
void setRefExclusion(RefFilter f);

// spawn-arb: the identity of a created (0xFF) reference. Returns false when unknown; the actor is then not tracked and the ref is asked
// again at every scan. `ref` is the wire FormRef every machine agrees on (host-issued), `netId` the matching live::NetId.
using CreatedIdResolver = std::function<bool(RefId ref, NetId& netId, fnvcoop::FormRef& wireRef)>;
void setCreatedIdResolver(CreatedIdResolver f);

// Inventory totals of a tracked actor (plugin/loot owns the engine calls: base container + changes). The reader says false when it cannot
// read the actor exactly (an unresolved leveled list in it); the writer makes the engine copy hold exactly the list. Without them an owner's
// item use is not copied to the other machines.
using InventoryReader = std::function<bool(NetId, InvList&)>;
using InventoryWriter = std::function<bool(NetId, const InvList&)>;
void setInventoryAccess(InventoryReader read, InventoryWriter write);

// Story effects applied on this machine (the host's captured ones too): KillActor, ResurrectActor and CompanionOwner reach ActorSync::onEffect().
// Any other effect is ignored here. Called by the effect applier / capture sink wiring.
void onEffect(const fnvcoop::Effect& e);

// A save is about to be written (or the session ends): stand-in clones and leveled-pick clones are removed NOW and the real actors they
// replaced are enabled again, so the save contains neither. The next scan puts them back. Call it before a scripted/bridge save.
void beforeSave();

// The engine the port runs on (engine_iface.h), shared with the other lanes that need engine reads / console writes (dll-wiring hands it to
// spawn-arb's EngineAdapter). null = the port is not installed ([actors] engine=0) or not loaded yet. Main thread only.
class IActorEngine;
IActorEngine* actorEngine();

// ---- configuration (dll-wiring maps nvcoop.ini onto it) --------------------------------------------------------------------------
struct Config {
    bool enabled = true;                      // false: the module installs nothing and the no-op port stays (ini [actors] engine=0)
    uint32_t scanMs = 500;                    // how often loaded actors are enumerated (events fire sooner when xNVSE sends them)
    uint32_t maxScanActors = 400;             // upper bound of actors tracked per machine
    uint32_t maxLinesPerFrame = 24;           // console lines the port may run per frame (SetPos/SetAngle/...), the rest waits a frame
    uint32_t driveMinMs = 33;                 // at most one pose application per actor per this long (30 Hz)
    float posEpsilonXY = 0.25f;               // game units: smaller position changes are not applied
    float posEpsilonZ = 0.75f;
    float yawEpsilonDeg = 1.0f;
    uint32_t pollDeathMs = 100;               // how often tracked actors are checked for a death the engine produced
    uint32_t deathGraceMs = 1500;             // a copy that died locally waits this long for the owner's death message, then it is revived
    bool restrainNonOwned = true;             // AI off = SetRestrained (the only per-actor AI switch known; see docs/tools/actor-port.md)
    bool stopCombatOnRestrain = true;
    bool startCombatOnTakeover = true;        // new owner re-targets from the last known combat target
    bool persistLedger = true;                // write the list of actors this machine restrained, so a save/load cannot leave them frozen
    uint32_t maxStandIns = 64;
    bool swapLeveled = true;                  // leveled pick differs from the local roll: replace the body with a clone of the pick (unverified in game)
    uint32_t saveGraceMs = 3000;              // after beforeSave(): no new clones for this long, so the save does not contain them
    uint32_t deleteQuietMs = 750;             // a removed clone is deleted this long after its last write (0 = at once): the engine may still hold a queued SetPos for its id
};
void configure(const Config& c);
const Config& config();

// ---- counters (diagnostics, coop.owner-style RPCs, tests) --------------------------------------------------------------------------
struct PortStats {
    uint64_t scans = 0, trackedNew = 0, trackedGone = 0, deathsPolled = 0, deathsReportedToSync = 0, deathsRefused = 0;
    uint64_t consoleLines = 0, consoleFailed = 0, linesDeferred = 0;
    uint64_t drives = 0, drivesSkippedSmall = 0, drivesSkippedRate = 0, snaps = 0, places = 0;
    uint64_t aiOff = 0, aiOn = 0, aiFailed = 0, combatRestarted = 0, combatTargetUnresolved = 0;
    uint64_t standInsMade = 0, standInsRemoved = 0, standInsRefused = 0, standInsPurged = 0;
    uint64_t baseSwaps = 0, baseSwapFailed = 0, protectOn = 0, protectOff = 0, kills = 0, revives = 0;
    uint64_t ledgerReleased = 0, ledgerWrites = 0, resyncs = 0;
    uint64_t ledgerDropped = 0;   // stale created-ref entries whose id now names someone else: dropped, never touched
    uint64_t avDrifts = 0;        // a copy's health that no longer matched the owner's (the audit wrote it back); the newest one below
    uint32_t lastDriftNet = 0;
    float lastDriftCur = 0, lastDriftWant = 0;
};
PortStats portStats();

}  // namespace nvcoop::actors::engine
