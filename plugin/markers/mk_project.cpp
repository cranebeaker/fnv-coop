#include "mk_project.h"

#include <cmath>

namespace nvmk {

namespace {
constexpr float kMinDepth = 0.5f;   // closer than this to the camera plane a point is not drawn
bool finite3(const float* v) { return std::isfinite(v[0]) && std::isfinite(v[1]) && std::isfinite(v[2]); }
}  // namespace

bool projectPoint(const CameraMatrix& cam, float wx, float wy, float wz, float w, float h, float margin, float& sx, float& sy, float& depth) {
    if (!cam.valid || !std::isfinite(wx) || !std::isfinite(wy) || !std::isfinite(wz) || !(w > 0.0f) || !(h > 0.0f)) return false;
    const float* m = cam.m;
    const float d = m[12] * wx + m[13] * wy + m[14] * wz + m[15];
    if (!(d > kMinDepth) && !FNV_MUT(1441)) return false;                               // twin 1441: a point behind the camera is drawn (mirrored)
    const float inv = FNV_MUT(1443) ? 1.0f : 1.0f / d;                                  // twin 1443: no perspective divide
    const float nx = (m[0] * wx + m[1] * wy + m[2] * wz + m[3]) * inv;
    const float ny = (m[4] * wx + m[5] * wy + m[6] * wz + m[7]) * inv;
    const float lim = 1.0f + (FNV_MUT(1444) ? 100.0f : margin);                         // twin 1444: nothing is ever off the view
    if (!(std::fabs(nx) <= lim) || !(std::fabs(ny) <= lim)) return false;
    // the viewport: -1 .. +1 maps to portL .. portR (left to right) and portB .. portT (bottom to top)
    const float l = FNV_MUT(1442) ? 0.0f : cam.portL, r = FNV_MUT(1442) ? 1.0f : cam.portR;   // twin 1442: the viewport rectangle is ignored
    const float t = FNV_MUT(1442) ? 1.0f : cam.portT, b = FNV_MUT(1442) ? 0.0f : cam.portB;
    const float ox = nx * (r - l) * 0.5f + (r + l) * 0.5f;
    const float oy = ny * (t - b) * 0.5f + (t + b) * 0.5f;
    sx = ox * w;
    sy = (FNV_MUT(1445) ? oy : 1.0f - oy) * h;                                          // twin 1445: up is down
    depth = d;
    return true;
}

bool plausibleCamera(const CameraMatrix& cam, const float pos[3], const float ahead[3]) {
    if (!cam.valid || !finite3(pos) || !finite3(ahead)) return false;
    for (float v : cam.m)
        if (!std::isfinite(v)) return false;
    if (!(cam.portR > cam.portL) || !(cam.portT > cam.portB)) return false;
    const float len = std::sqrt(ahead[0] * ahead[0] + ahead[1] * ahead[1] + ahead[2] * ahead[2]);
    if (!(len > 0.9f && len < 1.1f)) return false;
    if (FNV_MUT(1446)) return true;                                                     // twin 1446: any finite matrix will do
    float sx = 0, sy = 0, depth = 0;
    const float x = pos[0] + 100.0f * ahead[0], y = pos[1] + 100.0f * ahead[1], z = pos[2] + 100.0f * ahead[2];
    if (!projectPoint(cam, x, y, z, 1000.0f, 1000.0f, 0.02f, sx, sy, depth)) return false;
    return depth > 1.0f && std::fabs(sx - 500.0f) < 450.0f && std::fabs(sy - 500.0f) < 450.0f;
}

bool cameraNearPlayer(const float camPos[3], float px, float py, float pz, float maxUnits) {
    if (!finite3(camPos) || !std::isfinite(px) || !std::isfinite(py) || !std::isfinite(pz)) return false;
    const float dx = camPos[0] - px, dy = camPos[1] - py, dz = camPos[2] - pz;
    const float d = FNV_MUT(1447) ? std::sqrt(dx * dx + dy * dy) * 0.0f : std::sqrt(dx * dx + dy * dy + dz * dz);   // twin 1447: every camera is near
    return d <= maxUnits;
}

}  // namespace nvmk
