// Guest intents (FULL-SYNC-PLAN 3.7, docs/tools/guest-intents.md): everything a guest DOES to the shared world is forwarded to the host as an
// intent. The host runs the real interaction inside a capture scope with the acting-player context set to the guest's puppet, and commits
// whatever comes out with the guest as the txn origin. Game-free value types; the engine is reached through intent_engine.h.
#pragma once
#include <cstdint>
#include <string>
#include <vector>

#include "fnvcoop/transport.h"
#include "fnvcoop/types.h"

namespace nvcoop::intents {

using fnvcoop::Bytes;
using fnvcoop::Cause;
using fnvcoop::CauseKind;
using fnvcoop::FormRef;
using fnvcoop::PlayerId;
using fnvcoop::kHostPlayer;
using fnvcoop::kNoPlayer;

using RefId = uint32_t;   // engine refID (0 = none)

// docs/protocol.md section 19. A new mux channel with its own version byte; the story envelope (kWireVersion 5) is untouched.
constexpr uint8_t kChannelIntents = 0x8D;
constexpr uint8_t kIntentsWire = 1;

// What the guest did. Values are on the wire: append only.
enum class Kind : uint8_t {
    Activate = 1,       // used a door, switch, lever, activator, furniture, terminal (the OnActivate half)
    TriggerEnter = 2,   // walked into a trigger volume the host cannot see (OnTriggerEnter player)
    PickUp = 3,         // took a world item: a CLAIM, the first one wins (target = the item ref)
    Hit = 4,            // OnHit blocks of a non-actor target (destructibles, activators). Actors are the combat channel's business
    DialogueStart = 5,  // started talking to an NPC: routed to the 0xD1 dialogue adapter (lock, spectators), never run as an activation
    TerminalUse = 6,    // picked a terminal entry (target = terminal ref, path = entry indices from the top menu down)
    ContainerOpen = 7,  // opened a container: one user at a time (the "in use" lock), per-slot storage is redirected
    ContainerClose = 8, // closed it: releases the lock
    TriggerLeave = 9,   // left the trigger volume (OnTriggerLeave player)
    SetStage = 10,      // a quest stage advance the guest's own script / console tried to make (target = the quest, value = the stage). A REQUEST: the guest
                        // applies nothing; the host runs the real SetStage (result scripts included) when the quest is behind that stage and its rules allow it
    TriggerStay = 11,   // the guest's player is STILL inside a trigger volume (its per-frame OnTrigger events, throttled): keeps the host's presence alive, which
                        // re-raises OnTrigger for the puppet every host frame until TriggerLeave / the lease runs out (G04)
    StartCombat = 12,   // an actor (target) started combat with the guest's player: runs the actor's OnStartCombat blocks with the puppet as `player` (G04)
    Count_
};
const char* kindName(Kind k);

// What the host answers. Values are on the wire: append only.
enum class Result : uint8_t {
    Executed = 0,     // the host ran it; every effect travels as a normal txn with the guest as origin
    Duplicate,        // an intent with this id was already handled; the first verdict is repeated, nothing runs twice
    Disabled,         // this kind is switched off in the host's nvcoop.ini [intents]
    NotJoined,        // the sender is not a joined player (host-side guard; never normally seen by a guest)
    RateLimited,      // too many intents from this player; the guest backs off
    BadTarget,        // the ref is not something this kind can act on
    NotLoaded,        // the host has not loaded that reference
    OutOfRange,       // the player's puppet is too far from the target
    Busy,             // container in use by `holder`
    Taken,            // the item is gone or somebody else claimed it first
    Denied,           // refused by policy (e.g. a Hit on an actor)
    NoPuppet,         // no puppet for this player and the stand-in policy forbids acting as the host's player on this target
    Unsupported,      // the engine path is not available (TODO-RE); nothing ran
    Failed,           // the engine refused or faulted
    Stale,            // older than the newest handled intent of this player (out of order)
    NoAdvance,        // SetStage: the quest is already at or past that stage on the host (nothing ran: the stage merge is max, result scripts never re-run)
    Deferred,         // SetStage: the host's rules hold the stage (barrier / party tether); the host releases it itself later and the txn reaches everybody
    Count_
};
const char* resultName(Result r);
inline bool isRejection(Result r) { return r != Result::Executed && r != Result::Duplicate; }

constexpr size_t kMaxPath = 8;

// guest -> host
struct Intent {
    uint32_t session = 0;          // the guest's story session id; changes on every (re)join, so ids restart cleanly
    uint32_t id = 0;               // 1.. per session, strictly increasing
    Kind kind = Kind::Activate;
    FormRef target;
    FormRef aux;                   // PickUp: the item's base form when the guest knows it (informational); Hit: weapon
    bool hasAux = false;
    int32_t value = 0;             // Hit: damage as the guest saw it (informational); others 0
    std::vector<uint8_t> path;     // TerminalUse: entry indices
};

// host -> guest
struct Verdict {
    uint32_t session = 0;
    uint32_t id = 0;
    Result result = Result::Executed;
    PlayerId holder = kNoPlayer;   // Busy: who holds the container
    bool hasRedirect = false;
    FormRef redirect;              // ContainerOpen on shared storage: open THIS container instead (the player's own)
};

// ---- ini [intents] ---------------------------------------------------------------------------------------------------------------
struct Config {
    bool enable = false;           // master switch (default OFF, like every live-layer part)
    bool activate = true, trigger = true, pickup = true, terminal = true, container = true;
    bool stage = true;             // Kind::SetStage: a guest's blocked quest stage advance is forwarded (guest) and judged (host)
    bool triggerStay = true;       // Kind::TriggerStay: per-frame OnTrigger blocks (needs trigger=1 as well)
    bool combat = true;            // Kind::StartCombat: OnStartCombat blocks that filter on the player
    bool hit = false;              // see Kind::Hit: default off, the combat channel already raises OnHit for actors
    bool dialogue = false;         // DialogueStart forwarding needs L8's engine glue (not wired in this build)
    bool dialogueResults = true;   // host: run a guest conversation's INFO result scripts (replaces coop_module's stub runner)
    bool standIn = true;           // act as the host's own player when a guest has no puppet ...
    bool standInUnsafe = false;    // ... also for targets that open a menu for the player (container, terminal, NPC): default NO
    bool requirePuppet = false;    // never stand in (reject NoPuppet)
    bool standInRetry = true;      // host: an activator the guest's puppet activated to no effect (its script wants `GetActionRef == player`) is activated once more by the host's player
    bool runUnloaded = true;       // host: an Activate on an Activator in a cell the host has not loaded still runs its script (a guest alone in an area; the reach cannot be judged there)
    bool checkReach = true;        // compare the puppet's position with the target's (skipped when either is unknown)
    bool saveGate = true;          // enforce the shared SaveSuppress flag by vetoing the engine's CanSaveNow (SaveGate)
    float reachActivate = 450.f;   // game units (vanilla activation reach is about 150; the puppet lags a snapshot behind)
    float reachPickup = 450.f;
    float reachTrigger = 4000.f;   // a big volume: the point is only to reject an absurd claim
    float reachContainer = 450.f;
    float reachTerminal = 450.f;
    float reachHit = 6000.f;       // ranged weapons
    uint32_t rateBurst = 40;       // per player: token bucket
    float ratePerSec = 20.f;
    uint32_t lockLeaseMs = 120000; // a container lock the guest never released (crash, alt-F4) expires
    uint32_t triggerCooldownMs = 800;   // per (player, trigger): re-entry inside the volume is not a new event
    uint32_t pendingTimeoutMs = 6000;   // guest: no verdict by then = lost (counted, the UI is released)
    float reachCombat = 6000.f;    // an NPC that opens fire from across the street still starts combat with the puppet
    uint32_t stayKeepaliveMs = 1000;    // guest: one TriggerStay per trigger per this many ms while its OnTrigger events keep coming
    uint32_t stayLeaseMs = 3500;        // host: a presence nobody refreshed (no TriggerStay, no TriggerLeave) ends after this long
    uint32_t stayRaiseMs = 0;           // host: OnTrigger is raised for every present puppet at most every this many ms (0 = every host frame, like the engine)
    uint32_t stayMax = 64;              // host: presences held at once (a flood cannot make the host raise thousands of events a frame)
    bool storage = true;           // host: per-player stashes on shared gear containers (storage_book.h); needs the stashes in nvcoop_intents.json
    uint32_t coalesceMs = 250;     // guest: the same (kind, target) again inside this window is a double click, not a second intent
    uint32_t claimMemory = 4096;   // host: how many claimed items are remembered
    std::string redirectFile = "nvcoop_intents.json";   // next to the dll: per-slot container redirects (container_redirect.h) and gear stashes (storage_book.h)
};

}  // namespace nvcoop::intents
