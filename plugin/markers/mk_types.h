// plugin/markers: friend markers (name over the puppet, compass, Pip-Boy map) and revive of a downed friend. Game-free value types.
//
// Conventions: game units (70 per metre), heading = the REFR rotZ yaw (0 faces +Y, positive clockwise seen from above; the forward vector of yaw t
// is (sin t, cos t), as in plugin/avatar/loco.h). A player that is in an interior has world == 0.
#pragma once
#include <cmath>
#include <cstdint>
#include <string>
#include <vector>

#include "fnvcoop_mut.h"   // FNV_MUT(n): runtime mutation twins; constant false in production builds

namespace nvmk {

using Slot = uint8_t;                 // co-op party slot, 0 = the host
constexpr Slot kNoSlot = 0xFF;
constexpr Slot kHostSlot = 0;
constexpr Slot kMaxSlots = 8;
constexpr float kUnitsPerMeter = 70.0f;
constexpr float kCellUnits = 4096.0f;  // one exterior cell
constexpr float kPi = 3.14159265358979f;
constexpr float kTwoPi = 6.28318530717959f;
constexpr size_t kMaxNameLen = 24;
constexpr size_t kMaxPlaceLen = 40;

inline float wrapRad(float a) {
    if (!std::isfinite(a)) return 0.0f;
    a = std::fmod(a, kTwoPi);
    if (a > kPi) a -= kTwoPi;
    if (a <= -kPi) a += kTwoPi;
    return a;
}

// Where someone is. `name` is the interior cell's name or the worldspace's name (short, shown to the other players).
struct Place {
    bool valid = false;
    uint32_t world = 0;   // worldspace form id, 0 = an interior
    uint32_t cell = 0;    // interior: the cell; exterior: the grid cell if known
    float x = 0, y = 0, z = 0;
    std::string name;
    bool interior() const { return world == 0; }
    friend bool operator==(const Place& a, const Place& b) {
        return a.valid == b.valid && a.world == b.world && a.cell == b.cell && a.x == b.x && a.y == b.y && a.z == b.z && a.name == b.name;
    }
};

// Two places share a space when the one's puppet could be seen from the other's position: the same interior cell, or the same worldspace outdoors.
inline bool sameSpace(const Place& a, const Place& b) {
    if (!a.valid || !b.valid) return false;
    if (a.interior() || b.interior()) return a.interior() && b.interior() && a.cell == b.cell;
    return a.world == b.world;
}

inline float distance3(const Place& a, const Place& b) {
    const float dx = a.x - b.x, dy = a.y - b.y, dz = a.z - b.z;
    return std::sqrt(dx * dx + dy * dy + dz * dz);
}
inline float distance2(const Place& a, const Place& b) {
    const float dx = a.x - b.x, dy = a.y - b.y;
    return std::sqrt(dx * dx + dy * dy);
}

// One other player as this machine knows it (the host's party table).
struct Member {
    Slot slot = kNoSlot;
    std::string name;
    Place at;                 // where they are now
    Place ext;                // the last place they stood outdoors (where an interior's door is, roughly)
    bool downed = false;      // at 0 health, waiting for a revive or the respawn
    bool checkpointOnly = false;   // down until the party reaches a checkpoint (a collar death): cannot be revived
    uint32_t windowLeftMs = 0;     // downed: time left before the policy respawn
    Slot reviver = kNoSlot;        // somebody is holding the activate key on them
    uint32_t holdLeftMs = 0;       // ... and has this long to go
    uint64_t seenMs = 0;           // when this machine last heard of them (this machine's clock)
};

inline bool revivable(const Member& m) { return m.downed && !m.checkpointOnly && m.windowLeftMs > 0; }

// What the local machine reads about its own player each frame.
struct LocalState {
    bool valid = false;
    std::string name;
    Place at;
    Place ext;
    float heading = 0;        // radians, the player's yaw (the compass reads it)
    bool alive = true;        // not downed
    bool activateHeld = false;
    bool uiBlocked = false;   // a menu, a dialogue or the chat is open: no revive starts, no tags drawn
};

}  // namespace nvmk
