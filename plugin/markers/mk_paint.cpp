#include "mk_paint.h"

#include <algorithm>
#include <cmath>

namespace nvmk {

namespace {

constexpr uint32_t kFriend = 0xB8FFB8;   // a friend that is fine
constexpr uint32_t kDowned = 0xFF6A5A;   // a friend who is down
constexpr uint32_t kBanner = 0xFFFFFF;
constexpr uint32_t kBannerDown = 0xFF8070;
constexpr uint32_t kBarBack = 0x000000;
constexpr uint32_t kBarFill = 0x9CFF9C;
constexpr size_t kMaxListLines = 8;

float lineHeight(float pxH, float fraction, float minPx) { return std::max(minPx, pxH * fraction); }
float clamp01(float v) { return v < 0.0f ? 0.0f : (v > 1.0f ? 1.0f : v); }
bool isDownText(const std::string& s) { return s.find("(down)") != std::string::npos; }

void paintTags(const HudFrame& f, const HudLayout& L, const SceneInput& in, float sy, PaintList& out) {
    const float h = lineHeight(in.pxH, 0.026f, 12.0f);
    for (const HudTag& t : f.tags) {
        if (!t.at.valid || !(t.alpha >= 0.03f)) continue;
        float px = 0, py = 0, depth = 0;
        const float head = FNV_MUT(1452) ? 0.0f : L.tagHeadUnits;                         // twin 1452: the tag sits at the feet
        if (!projectPoint(in.cam, t.at.x, t.at.y, t.at.z + head, in.pxW, in.pxH, 0.05f, px, py, depth)) continue;
        PText p;
        p.s = t.text;
        p.align = Align::Center;
        p.h = h;
        p.x = px;
        p.y = py - (FNV_MUT(1450) ? 0.0f : static_cast<float>(L.tagLift) * sy) - h;        // twin 1450: the lift is ignored
        p.rgb = (isDownText(t.text) && !FNV_MUT(1453)) ? kDowned : kFriend;                 // twin 1453: a downed friend's tag looks like any other
        p.a = FNV_MUT(1451) ? 1.0f : clamp01(t.alpha);                                      // twin 1451: no fade with distance
        p.pill = true;
        out.texts.push_back(std::move(p));
        ++out.tags;
    }
}

void paintCompass(const HudFrame& f, const HudLayout& L, const SceneInput& in, const Rect& hud, float sx, float sy, PaintList& out) {
    if (f.compass.empty()) return;
    const CompassLayout cl = compassLayout(in.compassTile, hud, L.compass);
    if (cl.hidden) return;
    const float glyphH = 20.0f * sy;
    if (!cl.fromTile) {   // a strip of our own: a thin line the glyphs sit on
        PRect r;
        r.w = cl.width * in.pxW;
        r.x = cl.centerX * in.pxW - r.w * 0.5f;
        r.h = std::max(1.0f, in.pxH * 0.0015f);
        r.y = cl.y * sy + glyphH * 0.5f;
        r.rgb = kFriend;
        r.a = 0.35f;
        out.rects.push_back(r);
    }
    for (const HudCompassMark& m : f.compass) {
        const float frac = compassScreenX(cl, m.x, hud);
        PText p;
        p.s = m.glyph;
        p.align = Align::Center;
        p.h = glyphH;
        p.x = frac * (FNV_MUT(1454) ? hud.w : in.pxW);                                      // twin 1454: a fraction of the HUD's units, not of the window
        p.y = cl.y * sy;
        p.rgb = m.downed ? kDowned : kFriend;
        p.a = 1.0f;
        p.pill = true;
        out.texts.push_back(std::move(p));
        ++out.compass;
    }
}

void paintBanner(const HudFrame& f, const HudLayout& L, const SceneInput& in, float sy, PaintList& out) {
    if (f.banner.empty()) return;
    PText p;
    p.s = f.banner;
    p.align = Align::Center;
    p.h = lineHeight(in.pxH, 0.034f, 14.0f);
    p.x = in.pxW * 0.5f;
    p.y = FNV_MUT(1455) ? 0.0f : static_cast<float>(L.bannerY) * sy;                      // twin 1455: bannerY is ignored
    p.rgb = f.local ? kBannerDown : kBanner;
    p.pill = true;
    const float y = p.y, h = p.h;
    out.texts.push_back(std::move(p));
    if (f.bannerProgress >= 0.0f) {
        const float bw = in.pxW * 0.22f, bh = std::max(4.0f, in.pxH * 0.008f);
        PRect back;
        back.w = bw; back.h = bh; back.x = (in.pxW - bw) * 0.5f; back.y = y + h * 1.4f;
        back.rgb = kBarBack; back.a = 0.6f;
        PRect fill = back;
        fill.w = bw * (FNV_MUT(1460) ? 1.0f : clamp01(f.bannerProgress));                  // twin 1460: the bar is always full
        fill.rgb = kBarFill; fill.a = 0.95f;
        out.rects.push_back(back);
        out.rects.push_back(fill);
    }
}

void paintList(const HudFrame& f, const SceneInput& in, PaintList& out) {
    const float h = lineHeight(in.pxH, 0.024f, 12.0f);
    float y = in.pxH * 0.72f;
    size_t n = 0;
    for (const std::string& line : f.list) {
        if (n++ >= kMaxListLines) break;
        PText p;
        p.s = line;
        p.align = Align::Center;
        p.h = h;
        p.x = in.pxW * 0.5f;
        p.y = y;
        p.rgb = kFriend;
        p.pill = true;
        out.texts.push_back(std::move(p));
        y += h * 1.35f;
    }
    out.list = true;
}

void paintMap(const HudFrame& f, const HudLayout& L, const SceneInput& in, const Rect& hud, float sx, float sy, PaintList& out) {
    MapPick pick;
    bool manual = false;
    if (L.mapMode == "auto") {
        pick = chooseMap(in.localMap, in.worldMap, hud);
    } else if (L.mapMode == "local" || L.mapMode == "world") {
        manual = true;
        pick.kind = L.mapMode == "world" ? MapKind::World : MapKind::Local;
        pick.tile.r.valid = true;
        pick.tile.r.x = L.mapX * hud.w; pick.tile.r.y = L.mapY * hud.h;
        pick.tile.r.w = L.mapW * hud.w; pick.tile.r.h = L.mapH * hud.h;
    }
    const float markH = lineHeight(in.pxH, 0.022f, 11.0f);
    const float sq = std::max(6.0f, in.pxH * 0.012f);
    int placed = 0;
    if (pick.kind != MapKind::None) {
        for (const HudMapMark& m : f.map) {
            float u = 0, v = 0, hx = 0, hy = 0;
            bool ok = false;
            if (pick.kind == MapKind::World && m.world) ok = worldMapUV(m.at, in.ctx.bounds, u, v);
            else if (pick.kind == MapKind::Local && m.local) ok = localMapUV(in.ctx.viewer, m.at, L.localMapHalfRangeUnits, u, v);
            if (!ok || !mapScreenPoint(pick.tile, u, v, hx, hy)) continue;
            const float px = hx * (FNV_MUT(1459) ? 1.0f : sx), py = hy * (FNV_MUT(1459) ? 1.0f : sy);   // twin 1459: HUD units used as pixels
            const uint32_t col = m.downed ? kDowned : kFriend;
            PRect edge; edge.x = px - sq * 0.5f - 1.0f; edge.y = py - sq * 0.5f - 1.0f; edge.w = edge.h = sq + 2.0f; edge.rgb = 0x000000; edge.a = 0.85f;
            PRect dot; dot.x = px - sq * 0.5f; dot.y = py - sq * 0.5f; dot.w = dot.h = sq; dot.rgb = col; dot.a = 1.0f;
            out.rects.push_back(edge);
            out.rects.push_back(dot);
            PText t;
            t.s = FNV_MUT(1462) ? m.label : firstField(m.label);                              // twin 1462: the whole list line next to the marker
            t.align = Align::Left;
            t.h = markH;
            t.x = px + sq;
            t.y = py - markH * 0.5f;
            t.rgb = col;
            t.pill = true;
            out.texts.push_back(std::move(t));
            ++placed;
        }
    }
    out.mapPlaced = placed;
    const bool markers = pick.kind != MapKind::None;
    bool listNeeded = manual || L.mapMode == "list" || mapListNeeded(markers, placed, static_cast<int>(f.list.size()));
    if (FNV_MUT(1458)) listNeeded = false;                                                  // twin 1458: a friend who could not be placed is on no list
    if (listNeeded && !f.list.empty()) paintList(f, in, out);
}

}  // namespace

Rect deriveHudScreen(float w, float h) {
    Rect r;
    if (!(w > 0.0f) || !(h > 0.0f)) return r;
    r.valid = true;
    r.x = r.y = 0;
    r.w = (w > h) ? w / h * 960.0f : 1280.0f;
    r.h = (w >= h) ? 960.0f : h / w * 1280.0f;
    if (FNV_MUT(1464)) { r.w = w; r.h = h; }                                                // twin 1464: pixels taken for the HUD's units
    return r;
}

std::string firstField(const std::string& label) {
    const size_t c = label.find(',');
    return c == std::string::npos ? label : label.substr(0, c);
}

PaintList paintFrame(const HudFrame& f, const HudLayout& layout, const SceneInput& in) {
    PaintList out;
    if (!(in.pxW > 0.0f) || !(in.pxH > 0.0f)) return out;
    const Rect hud = plausibleScreen(in.hud) ? in.hud : deriveHudScreen(in.pxW, in.pxH);
    if (!hud.valid || !(hud.w > 0.0f) || !(hud.h > 0.0f)) return out;
    const float sx = in.pxW / hud.w, sy = in.pxH / hud.h;
    const bool pip = in.ctx.pipboyMapOpen;
    if (pip || FNV_MUT(1457)) {                                                             // twin 1457: the map is painted while playing too
        if (!f.map.empty() || !f.list.empty()) paintMap(f, layout, in, hud, sx, sy, out);
        if (!pip) return out;
    }
    const bool playing = (!in.ctx.menuOpen || FNV_MUT(1456)) && !pip;                       // twin 1456: HUD parts are drawn over a menu
    if (!playing) return out;
    if (!in.hudShown && !FNV_MUT(1461)) return out;                                         // twin 1461: a hidden HUD still gets markers
    paintTags(f, layout, in, sy, out);
    paintCompass(f, layout, in, hud, sx, sy, out);
    paintBanner(f, layout, in, sy, out);
    return out;
}

}  // namespace nvmk
