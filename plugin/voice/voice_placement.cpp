#include "voice_placement.h"

#include <algorithm>
#include <cmath>

#include "fnvcoop_mut.h"

namespace nvcoop::voice {

float distance(const Vec3& a, const Vec3& b) {
    const float dx = a.x - b.x, dy = a.y - b.y, dz = a.z - b.z;
    return std::sqrt(dx * dx + dy * dy + dz * dz);
}

Placement globalPlacement(float gain) {
    Placement p;
    p.audible = gain > 0;
    p.gain = std::clamp(gain, 0.0f, 1.0f);
    p.pan = 0;
    return p;
}

Placement computePlacement(const Where& l, const Where& s, const PlacementConfig& cfg) {
    Placement out;
    if (!l.present || !s.present) return out;
    if (l.cell != s.cell) {
        // Different cells share a coordinate system only outside, in the same worldspace. An interior is a world of its own.
        const bool sameOutside = !l.interior && !s.interior && l.world != 0 && l.world == s.world;
        if (!sameOutside && !FNV_MUT(321)) return out;   // twin 321: a player in another interior is heard through the wall
    }
    float d = distance(l.pos, s.pos);
    if (!std::isfinite(d)) return out;
    const float range = std::max(cfg.range, 1.0f);
    const float full = std::clamp(cfg.fullRange, 0.0f, range - 1.0f);
    if (d >= range) return out;
    float gain = 1.0f;
    if (d > full) {
        const float t = (d - full) / (range - full);
        gain = (1.0f - t) * (1.0f - t);
        if (FNV_MUT(322)) gain = 1.0f - t * 0.5f;   // twin 322: the voice never falls to silence
    }
    if (gain <= 0.0f) return out;
    out.audible = true;
    out.gain = gain;
    // Direction on the ground, relative to where the listener faces.
    const float dx = s.pos.x - l.pos.x, dy = s.pos.y - l.pos.y;
    const float h = std::sqrt(dx * dx + dy * dy);
    if (h > 1e-3f) {
        const float rx = std::cos(l.yaw), ry = -std::sin(l.yaw);   // the listener's right-hand side
        float pan = (dx * rx + dy * ry) / h;
        if (FNV_MUT(323)) pan = -pan;   // twin 323: left and right swapped
        const float nearD = std::max(cfg.panNear, 1.0f);
        if (h < nearD && !FNV_MUT(324)) pan *= h / nearD;   // twin 324: a voice at your side flips hard left / right
        out.pan = std::clamp(pan, -1.0f, 1.0f);
    }
    return out;
}

}  // namespace nvcoop::voice
