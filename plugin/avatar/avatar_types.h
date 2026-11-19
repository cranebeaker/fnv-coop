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
#include <cstdint>
#include <string>
#include <vector>

namespace nvcoop::avatar {

using Slot = uint8_t;      // coop player slot, 0 = host
using RefId = uint32_t;    // engine refID, 0 = none
using FormId = uint32_t;   // engine form id (base form, item); 0 = none
using CellId = uint32_t;
constexpr Slot kNoSlot = 0xFF;

struct Vec3 {
    float x = 0, y = 0, z = 0;
};

// Mirrors synccore/live AnimState. `loco` codes are defined in loco.h (the live layer treats them as opaque).
struct Anim {
    uint8_t loco = 0;
    bool weaponDrawn = false, sneak = false, jump = false, reload = false, aim = false;
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
    std::vector<int8_t> faceCoeffs;        // FaceGen morph coefficients, quantised to 1/16 (at most kMaxFaceCoeffs); empty = not readable
    std::string name;
};
constexpr size_t kMaxFaceCoeffs = 160;     // 50 + 30 + 50 floats are what the three FaceGenData blocks of the player hold

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
struct Config {
    Slot localSlot = 0;                      // this machine's own slot: it never gets a puppet
    unsigned maxPuppets = 3;                 // 4 players => 3 remote puppets
    std::vector<TemplateEntry> templates;
    FormId defaultTemplate = 0;              // used when no entry matches (appearance is then only approximate)

    // spawn
    uint32_t spawnBackoffMs = 500;           // doubled per failure, capped at spawnBackoffMaxMs
    uint32_t spawnBackoffMaxMs = 8000;
    uint32_t load3DTimeoutMs = 5000;         // a spawned ref that never gets 3D is deleted and respawned
    unsigned maxSpawnAttempts = 8;           // then the slot is Failed for failedRetryMs (or until AddPlayer is called for it again)
    uint32_t failedRetryMs = 30000;          // a Failed slot starts a fresh round of attempts after this (0 = never)
    uint32_t parkDelayMs = 500;              // remote's cell must stay unloaded this long before the puppet is deleted
    uint32_t staleMs = 3000;                 // no SetPose for this long: treat as frozen (stop locomotion)

    // pose
    float posEpsilon = 0.5f;                 // units: smaller moves are not sent
    float yawEpsilonRad = 0.0087f;           // 0.5 degree
    uint32_t posHeartbeatMs = 1000;          // re-send an unchanged position (physics / AI may have moved it)
    float driftUnits = 140.0f;               // actual vs applied farther than this (2 m) forces a re-send and is counted
    uint32_t restrainCheckMs = 250;          // how often IsRestrained is polled

    // animation
    uint32_t animHeartbeatMs = 1500;         // re-issue the current group at least this often (engine may reset to idle)
    uint32_t animMismatchMs = 300;           // CurrentGroup() differs from the wanted group this long: re-issue

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
};

}  // namespace nvcoop::avatar
