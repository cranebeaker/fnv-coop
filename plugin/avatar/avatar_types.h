// Player puppets (FULL-SYNC-PLAN 3.1), game-free core: shared value types.
//
// A puppet is the local stand-in for a REMOTE player: a restrained actor made from a template NPC, driven every
// frame by SetPos / SetAngle / PlayGroup from the interpolated snapshot (synccore/live InterpBuffer), with an
// equipment correction loop. It is never saved and is purged on load. Nothing in plugin/avatar touches the game
// directly: every engine call goes through IPuppetEngine (puppet_engine.h) and is unit tested with a fake.
//
// Conventions (all UNMEASURED in the running game, see docs/tools/avatar.md "TODO-RE"):
//   units      FNV game units (70 units are about 1 m), radians in this layer, degrees only inside the script backend.
//   rot        (pitch, yaw, roll), the same order as synccore/live ActorSnapshot::rot. yaw is the REFR rotZ heading,
//              0 = facing +Y, positive = clockwise seen from above. Only yaw is ever applied to the actor.
//   heading    forward vector of yaw t is (sin t, cos t).
#pragma once
#include <algorithm>
#include <cstdint>
#include <string>
#include <vector>

namespace nvcoop::avatar {

using Slot = uint8_t;      // coop player slot, 0 = host
using RefId = uint32_t;    // engine refID, 0 = none
using FormId = uint32_t;   // engine form id (base form, item); 0 = none
using CellId = uint32_t;
constexpr Slot kNoSlot = 0xFF;
constexpr int kNoGroup = -1;   // PuppetManager::SetPoseOverride: no override, the planned group plays

struct Vec3 {
    float x = 0, y = 0, z = 0;
};

// Mirrors synccore/live AnimState. `loco` codes are defined in loco.h (the live layer treats them as opaque).
struct Anim {
    uint8_t loco = 0;
    bool weaponDrawn = false, sneak = false, jump = false, reload = false, aim = false;
    // The owner's one-shot (live::AnimState::action / actionSeq): the base group id of the attack swing / shot / throw, power attack, block,
    // stagger ... it is playing right now, and a counter that bumps every time one STARTS (wraps at 256). 0 = none. `block` = the block group is held.
    uint8_t action = 0, actionSeq = 0;
    bool block = false;
    // The owner's furniture (live::AnimState::sit / sleep / furniture): 0 = standing, 1 = going in, 2 = in it, 3 = getting out (live::SitState); sleep = it is a bed;
    // furniture = the reference id of the chair / bench / bed on the OWNER's machine (the same load order makes it the same here). The puppet sits in it.
    uint8_t sit = 0;
    bool sleep = false;
    uint32_t furniture = 0;
};

// One interpolated sample for a puppet (what live::InterpBuffer::sample returns, see live_bridge.h).
struct PuppetPose {
    bool valid = false;      // false until the first sample arrives
    CellId cell = 0;
    Vec3 pos;
    Vec3 rot;                // (pitch, yaw, roll) radians
    Vec3 vel;                // units per second
    Anim anim;
    bool snapped = false;    // the interpolation buffer jumped on this sample (teleport / cell change)
    bool frozen = false;     // the buffer has run dry (Holding / Empty): the remote stopped sending
};

// What the owner's player looks like. Race / face data is applied through IPuppetEngine::ApplyAppearance; the
// template NPC base is chosen from Config::templates by (race, female).
struct Appearance {
    uint32_t race = 0;       // race form id (plugin-local mapping done by the caller)
    bool female = false;
    uint32_t hair = 0, eyes = 0;
    uint32_t faceHash = 0;   // opaque face-gen identity, engine specific (TODO-RE)
    // Body and look data of the NPC base (round of 2026-10-22). All of it travels in the Look message (plugin/coop/live/avatar_wire.h).
    uint32_t hairColor = 0;                // TESNPC hair colour (packed)
    uint16_t height = 0, weight = 0;       // TESNPC height / weight in thousandths (1000 = 1.0), 0 = unknown
    std::vector<int8_t> faceCoeffs;        // FaceGen morph coefficients, quantised to 1/16 (at most kMaxFaceCoeffs); empty = not readable. All three blocks in a fixed order (engine/face_blocks.h)
    std::vector<uint32_t> headParts;       // BGSHeadPart form ids of the NPC base (eyebrows, facial hair, mouth ...; TESNPC +0x1DC), in list order; empty = not readable / none
    std::string name;
    bool known() const { return race != 0; }   // a real look was read (an Appearance{} is "nothing known yet": it must not be written into a puppet)
};
constexpr size_t kMaxFaceCoeffs = 160;     // 50 + 30 + 50 floats are what the three FaceGenData blocks of the player hold
constexpr size_t kMaxHeadParts = 16;

// Facts about the LOCAL player that host-run conditions ask about (GetLevel, GetSitting, IsPlayerActionActive, IsPlayerInRegion) and that no snapshot carries (W3a item 3).
// Read at a low rate by the guest's avatar layer and sent to the host only when they change (plugin/coop/live/avatar_wire.h, Facts message).
constexpr size_t kMaxFactRegions = 16;
constexpr unsigned kFactActionIds = 32;   // IsPlayerActionActive ids 0..31 are sampled (the rules ask for id 9)
struct PlayerFacts {
    uint16_t level = 0;                // GetLevel; 0 = unknown
    uint8_t sitting = 0;               // GetSitting (the engine's sit state, 0 = not sitting)
    uint32_t actions = 0;              // bit n = IsPlayerActionActive n
    std::vector<FormId> regions;       // TESRegion form ids of PlayerCharacter::regionsList, at most kMaxFactRegions
    friend bool operator==(const PlayerFacts& a, const PlayerFacts& b) {
        return a.level == b.level && a.sitting == b.sitting && a.actions == b.actions && a.regions == b.regions;
    }
};

// What a guest carries, as far as host-run conditions and rules ask (guest pack facts). Total weight (the InventoryWeight actor value, the Honest Hearts Jed gate) and the counts of
// the watched item forms (plugin/coop/tools/gen_coop_data.py nvcoop_pack_watch.json: the items the rules and quest conditions count), never the whole inventory. Read at a low rate
// by the guest's avatar layer and sent to the host only when it changed (plugin/coop/live/avatar_wire.h, Pack message).
constexpr size_t kMaxPackItems = 255;       // entries with a non-zero count in one message
constexpr unsigned kAvInventoryWeight = 46; // actor value code of InventoryWeight (tools: snapshot_nvse.cpp)
struct PackEntry {
    FormId form = 0;
    int32_t count = 0;
    friend bool operator==(const PackEntry& a, const PackEntry& b) { return a.form == b.form && a.count == b.count; }
};
struct PlayerPack {
    uint32_t watchId = 0;              // identifies the watch list the counts were taken against (packWatchId); the host trusts a zero only for the same list
    bool haveWeight = false;
    uint32_t weightMilli = 0;          // InventoryWeight * 1000, rounded
    std::vector<PackEntry> items;      // watched forms with a count above zero, sorted by form
    friend bool operator==(const PlayerPack& a, const PlayerPack& b) {
        return a.watchId == b.watchId && a.haveWeight == b.haveWeight && a.weightMilli == b.weightMilli && a.items == b.items;
    }
};
// FNV-1a over the sorted, de-duplicated forms: two machines with the same list get the same id.
inline uint32_t packWatchId(std::vector<FormId> forms) {
    std::sort(forms.begin(), forms.end());
    forms.erase(std::unique(forms.begin(), forms.end()), forms.end());
    uint32_t h = 2166136261u;
    for (FormId f : forms)
        for (int i = 0; i < 4; ++i) { h ^= (f >> (8 * i)) & 0xFFu; h *= 16777619u; }
    return h;
}

struct EquipEntry {
    FormId form = 0;
    bool weapon = false;     // weapons are equipped last and drawn only once equipped; the one weapon flagged is the HELD weapon
    uint8_t mods = 0;        // weapon modification flags (ExtraWeaponModFlags: bit 0 = mod 1, bit 1 = mod 2, bit 2 = mod 3); 0 for armor
};
constexpr uint8_t kWeaponModMask = 0x07;

struct TemplateEntry {
    uint32_t race = 0;
    bool female = false;
    FormId base = 0;         // template NPC base form: supplied by the plugin config / our ESP, never invented here
};

// Everything numeric is a first guess that T7 (tools/animcheck, proof_puppet.py) must tune. Units/ms as named.
// Timing figures the network layer measures and hands to the avatar layer, which reports them in avatar.status ("stages"). Milliseconds on the avatar
// layer's clock (the one the snapshot ticks are stamped with), except NetFrame, which carries that clock's reading itself.
enum class NetStage : uint8_t {
    Send,      // a snapshot's sampling time -> the moment its datagram is handed to the transport (sender, same clock)
    Rx,        // a snapshot's sampling time -> the moment the other copy's network layer has it (a shared clock on one PC; across PCs it includes the clock offset)
    NetFrame,  // the network layer is running a frame now: the value is the clock reading (the runtime derives netPhase and netGap from it)
};

struct Config {
    Slot localSlot = 0;                      // this machine's own slot: it never gets a puppet
    unsigned maxPuppets = 3;                 // 4 players => 3 remote puppets
    std::vector<TemplateEntry> templates;
    FormId defaultTemplate = 0;              // used when no entry matches (appearance is then only approximate)

    // spawn
    uint32_t lookWaitMs = 4000;              // an engine that builds a puppet from its player's look (LooksByBase) waits this long for the first look before it spawns from the bare template
    uint32_t spawnBackoffMs = 500;           // doubled per failure, capped at spawnBackoffMaxMs
    uint32_t spawnBackoffMaxMs = 8000;
    uint32_t load3DTimeoutMs = 5000;         // a spawned ref that never gets 3D is deleted and respawned
    unsigned maxSpawnAttempts = 8;           // then the slot is Failed until AddPlayer is called for it again
    uint32_t parkDelayMs = 500;              // remote's cell must stay unloaded this long before the puppet is deleted
    uint32_t staleMs = 3000;                 // no SetPose for this long: treat as frozen (stop locomotion)

    // pose
    float posEpsilon = 0.5f;                 // units: smaller moves are not sent
    float yawEpsilonRad = 0.0087f;           // 0.5 degree
    uint32_t posHeartbeatMs = 1000;          // re-send an unchanged position (physics / AI may have moved it)
    float driftUnits = 140.0f;               // actual vs applied farther than this (2 m) forces a re-send and is counted
    // A puppet is never put inside the body of this machine's own player: two overlapping bodies make the engine push them apart, and the
    // driver's SetPos back into the same spot every frame turns that into a continuous shove of the REAL player. The place sent to the
    // engine is moved out to this horizontal distance (1 m, a little more than two body radii) when the remote stands closer than that.
    float bodyRoomUnits = 70.0f;
    float bodyRoomHeightUnits = 110.0f;      // only when the two are within this height of each other (not on different floors)
    uint32_t restrainCheckMs = 250;          // how often IsRestrained is polled

    // animation
    uint32_t animHeartbeatMs = 1500;         // re-issue the current group at least this often (engine may reset to idle)
    uint32_t animMismatchMs = 300;           // CurrentGroup() differs from the wanted group this long: re-issue

    // furniture: the owner sits in a chair / bench / bed and his puppet is told to use the same one
    uint32_t seatRetryMs = 1500;             // after a refused sit (or a sit the engine dropped) the command is not sent again for this long
    uint32_t seatVerifyMs = 2500;            // a sit command is judged by the engine's own sit state only after this long (the walk-in and the sit-down animation take a while)
    uint32_t standRetryMs = 1200;            // between stand-up commands while the engine still says the puppet is in the furniture
    uint32_t standSettleMs = 1500;           // a puppet whose sit state cannot be read is taken for standing this long after the stand-up command
    uint32_t standGiveUpMs = 8000;           // a puppet that does not get out of the furniture by then is driven again anyway (the movement code puts it where its owner is)
    uint32_t lookCheckDelayMs = 500;         // a spawned puppet's look is read back from the engine this long after it started driving

    // a puppet is a hit target (enemies shoot it; combat forwards the damage to the real player): it must stay standing
    uint32_t healthCheckMs = 250;            // how often IsDead is polled
    uint32_t healMs = 2000;                  // ResetHealth at least this often (0 = never); the real player's health lives on the real player's machine
    uint32_t deathRespawnMs = 1000;          // a puppet that died anyway is deleted and respawned after this long

    // budget
    unsigned maxEngineCalls = 24;            // engine write calls per Tick across all puppets

    // equipment
    uint32_t equipOpIntervalMs = 250;        // at most one equipment write per puppet per interval
    uint32_t equipPollMsConverged = 1000;
    unsigned equipMaxFails = 6;              // consecutive failures of the same item before giving up on it
    uint32_t equipBackoffMaxMs = 4000;
    unsigned equipFlapLimit = 3;             // the same item toggling this often inside equipFlapWindowMs => RemoveItem
    uint32_t equipFlapWindowMs = 4000;
};

enum class Phase : uint8_t {
    Absent,      // no ref (not spawned yet, parked, deleted, or waiting for a retry)
    Loading3D,   // spawned, waiting for 3D before any command that needs it
    Driving,     // fully set up and driven every Tick
    Failed,      // gave up after maxSpawnAttempts
};
const char* ToString(Phase p);

struct Stats {
    uint64_t spawned = 0, spawnFailures = 0, deleted = 0, refLost = 0, load3DTimeouts = 0;
    uint64_t setPos = 0, setYaw = 0, moveToCell = 0, teleports = 0, drift = 0;
    uint64_t playGroup = 0, groupRefused = 0, animReasserts = 0, menuReasserts = 0, restrainReasserts = 0;
    uint64_t parked = 0, unparked = 0, purged = 0, saveDespawns = 0, loadResets = 0;
    uint64_t equipOps = 0, equipFailures = 0, equipGaveUp = 0, equipFlapRemovals = 0;
    uint64_t budgetDeferred = 0, rejectedLocalSlot = 0, replacedDuplicates = 0, staleFrozen = 0;
    uint64_t puppetDeaths = 0, heals = 0;                            // a hit-target puppet that died (respawned) / health top-ups sent
    uint64_t appearanceUpdates = 0;                                  // a look change applied to an existing puppet (no respawn)
    uint64_t modOps = 0, modFailures = 0, modGaveUp = 0;             // weapon modification writes (EnableWeaponMod) and their outcome
    uint64_t pushesIgnored = 0;                                      // engine pushes (body in the way) not handed to the interpolation buffer
    uint64_t bodyYields = 0;                                         // driven places moved out of the local player's body (frames, not episodes)
    uint64_t oneShots = 0, oneShotsStale = 0, oneShotsMissed = 0;    // the owner's attacks / reactions played; counter replays or reordering dropped; events over before this copy saw them
    uint64_t oneShotsFrozen = 0, oneShotsFailed = 0;                 // dropped because the stream was dry; refused by the engine for good
    uint64_t oneShotsSeen = 0, oneShotsNotSeen = 0;                  // a played one-shot the engine then showed running / did not show running ~150 ms later
    uint64_t attackDraws = 0;                                        // the puppet's weapon was drawn first so that an unarmed or sheathed attack can play
    uint64_t attackRetries = 0;                                      // a swing the engine did not show was played again
    uint64_t seatsIssued = 0, standsIssued = 0, seatDrops = 0;       // the puppet was told to sit / to stand up; the engine dropped a sit it had accepted
    uint64_t looksChecked = 0, looksWrong = 0;                       // a spawned puppet's look read back from the engine; ... and found different from what was asked
};

}  // namespace nvcoop::avatar
