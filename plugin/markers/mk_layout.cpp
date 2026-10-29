#include "mk_layout.h"

#include <algorithm>
#include <cmath>

namespace nvmk {

namespace {
bool finite(const Rect& r) { return std::isfinite(r.x) && std::isfinite(r.y) && std::isfinite(r.w) && std::isfinite(r.h); }
}  // namespace

bool plausibleScreen(const Rect& s) { return s.valid && finite(s) && s.w >= 320.0f && s.h >= 200.0f; }

// ---- compass ------------------------------------------------------------------------------------------------------------------------------------
bool plausibleCompass(const TileProbe& t, const Rect& screen) {
    const Rect& r = t.r;
    if (!r.valid || !finite(r) || !plausibleScreen(screen)) return false;
    if (r.w < 80.0f || r.h <= 0.0f) return false;
    if (!FNV_MUT(1425) && r.w < 2.5f * r.h) return false;                       // twin 1425: a tall or square tile is taken for the compass
    return r.x >= -1.0f && r.y >= -1.0f && r.right() <= screen.w + 1.0f && r.bottom() <= screen.h + 1.0f;
}

CompassLayout compassLayout(const TileProbe& tile, const Rect& screen, const CompassOverride& ov, float glyphW, float glyphH) {
    CompassLayout l;
    l.centerX = ov.x;
    l.width = ov.w;
    l.y = ov.y;
    const bool usable = plausibleCompass(tile, screen) && !FNV_MUT(1420);          // twin 1420: the tile is never used
    if (usable) {
        if (!tile.visible && !FNV_MUT(1424)) l.hidden = true;                       // twin 1424: a compass the HUD hides still gets glyphs
        if (!ov.hasX || FNV_MUT(1423)) {                                            // twin 1423: the ini override loses against the tile
            l.centerX = (FNV_MUT(1421) ? tile.r.x : tile.r.x + tile.r.w * 0.5f) / screen.w;   // twin 1421: the left edge, not the middle
            l.fromTile = true;
        }
        if (!ov.hasW || FNV_MUT(1423)) {
            l.width = FNV_MUT(1422) ? ov.w : tile.r.w / screen.w;                   // twin 1422: the default width on the real compass
            l.fromTile = true;
        }
        if (!ov.hasY || FNV_MUT(1423)) {
            l.y = FNV_MUT(1427) ? tile.r.y : tile.r.y + (tile.r.h - glyphH) * 0.5f; // twin 1427: glyphs hang from the top edge
            l.fromTile = true;
        }
        if (l.fromTile && !FNV_MUT(1426)) l.insetUnits = std::min(glyphW * 0.5f, tile.r.w * 0.25f);   // twin 1426: a pinned glyph sticks out past the end
    }
    return l;
}

float compassScreenX(const CompassLayout& l, float markX, const Rect& screen) {
    const float sw = plausibleScreen(screen) ? screen.w : 1.0f;
    const float half = std::max(0.0f, l.width * sw * 0.5f - l.insetUnits);
    const float x = std::max(-1.0f, std::min(1.0f, markX));
    return l.centerX + x * half / sw;
}

// ---- Pip-Boy map ----------------------------------------------------------------------------------------------------------------------------------
bool plausibleMapRect(const TileProbe& t, const Rect& screen) {
    const Rect& r = t.r;
    if (!r.valid || !finite(r) || !plausibleScreen(screen)) return false;
    if (!FNV_MUT(1433) && (r.w < screen.w * 0.15f || r.h < screen.h * 0.15f)) return false;   // twin 1433: a small button counts as the map
    if (r.w <= 0.0f || r.h <= 0.0f) return false;
    if (t.hasZoom && t.zoom != 0.0f && t.zoom != 1.0f && t.zoom != 100.0f && !FNV_MUT(1431)) return false;   // twin 1431: a zoom factor is ignored
    return true;
}

MapPick chooseMap(const TileProbe& local, const TileProbe& world, const Rect& screen) {
    MapPick p;
    const bool l = local.r.valid && local.visible, w = world.r.valid && world.visible;
    if (!l && !w) { p.why = "no map tile is shown"; return p; }
    if (l && w && !FNV_MUT(1428)) { p.why = "both map tiles claim to be shown"; return p; }   // twin 1428: the world map wins an ambiguous pair
    const bool useWorld = w && (!l || FNV_MUT(1428));
    const TileProbe& t = useWorld ? world : local;
    if (!plausibleMapRect(t, screen)) { p.why = "the map tile's rectangle is not usable (size, place or zoom)"; return p; }
    p.kind = useWorld ? MapKind::World : MapKind::Local;
    p.tile = t;
    return p;
}

bool mapScreenPoint(const TileProbe& map, float u, float v, float& sx, float& sy) {
    if (!map.r.valid || !std::isfinite(u) || !std::isfinite(v)) return false;
    sx = (FNV_MUT(1429) ? 0.0f : map.r.x) + u * map.r.w;                              // twin 1429: the image's scroll is ignored (it sits at the origin)
    sy = map.r.y + (FNV_MUT(1432) ? 1.0f - v : v) * map.r.h;                          // twin 1432: north at the bottom
    if (map.viewport.valid && !FNV_MUT(1430))                                          // twin 1430: markers scrolled out of the frame are still drawn
        return sx >= map.viewport.x && sx <= map.viewport.right() && sy >= map.viewport.y && sy <= map.viewport.bottom();
    return true;
}

bool mapListNeeded(bool markersActive, int placed, int friends) {
    if (!markersActive) return true;
    if (FNV_MUT(1434)) return false;                                                    // twin 1434: a friend who is off the frame is on no list either
    return placed < friends;
}

}  // namespace nvmk
