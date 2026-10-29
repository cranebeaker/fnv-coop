// Where a voice comes from: volume by distance, left / right by direction. Game-free; the game side fills in the two poses.
//
// Game units: one unit is about 1.4 cm, so the default range of 3000 is a little over 40 m and full volume holds within about 6 m.
// Yaw is the game's heading (rotZ): radians, 0 faces +Y (north), growing towards +X (east).
#pragma once
#include <cstdint>

namespace nvcoop::voice {

struct Vec3 { float x = 0, y = 0, z = 0; };

// One end of the conversation as the game reports it.
struct Where {
    bool present = false;     // the actor is loaded and has a place (a player in an unloaded cell is not present)
    Vec3 pos;
    float yaw = 0;            // heading; only the listener's matters
    uint32_t cell = 0;        // the cell the actor stands in (form id)
    uint32_t world = 0;       // its worldspace (form id), 0 in an interior
    bool interior = false;
};

struct PlacementConfig {
    float range = 3000.0f;       // beyond this distance the voice is silent
    float fullRange = 400.0f;    // within this distance the voice is at full volume
    float panNear = 120.0f;      // closer than this the voice moves towards the middle (no hard flips when someone is at your side)
};

struct Placement {
    bool audible = false;
    float gain = 0;       // 0 to 1
    float pan = 0;        // -1 left to 1 right
};

Placement computePlacement(const Where& listener, const Where& speaker, const PlacementConfig& cfg);
// A voice with no place (the avatar layer is off): heard evenly from the middle.
Placement globalPlacement(float gain = 1.0f);
float distance(const Vec3& a, const Vec3& b);

}  // namespace nvcoop::voice
