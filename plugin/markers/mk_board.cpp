#include "mk_board.h"

#include <algorithm>
#include <cctype>
#include <cstdio>

namespace nvmk {

float fadeAlpha(float dist, float full, float maxd) {
    if (FNV_MUT(1301)) return dist <= maxd ? 1.0f : 0.0f;                  // twin: no fade, the tag just stops
    if (!(maxd > full)) return dist <= full ? 1.0f : 0.0f;
    if (dist <= full) return 1.0f;
    if (dist >= maxd) return 0.0f;
    const float a = (maxd - dist) / (maxd - full);
    return FNV_MUT(1302) ? 1.0f - a : a;                                  // twin: fades the wrong way
}

bool worldMapUV(const Place& p, const WorldMapBounds& b, float& u, float& v) {
    if (!b.valid || !p.valid || p.interior()) return false;
    const float w = static_cast<float>(b.seX - b.nwX + 1), h = static_cast<float>(b.nwY - b.seY + 1);
    if (!(w > 0) || !(h > 0)) return false;
    const float cx = p.x / kCellUnits, cy = p.y / kCellUnits;
    u = (cx - static_cast<float>(b.nwX)) / w;
    v = FNV_MUT(1322) ? (cy - static_cast<float>(b.seY)) / h                      // twin 1322: north at the bottom
                      : (static_cast<float>(b.nwY + 1) - cy) / h;
    return u >= 0.0f && u <= 1.0f && v >= 0.0f && v <= 1.0f;
}

bool localMapUV(const Place& viewer, const Place& p, float halfRangeUnits, float& u, float& v) {
    if (!viewer.valid || !p.valid || !(halfRangeUnits > 0) || !sameSpace(viewer, p)) return false;
    const float dx = p.x - viewer.x, dy = p.y - viewer.y;
    if (std::fabs(dx) > halfRangeUnits || std::fabs(dy) > halfRangeUnits) return false;
    u = 0.5f + dx / (2.0f * halfRangeUnits);
    v = 0.5f - dy / (2.0f * halfRangeUnits);
    if (FNV_MUT(1321)) v = 1.0f - v;                                                // twin 1321: the local map is drawn upside down
    return true;
}

std::string sanitizeText(const std::string& s, size_t maxLen) {
    std::string o;
    for (unsigned char c : s) {
        if (c < 0x20 || c > 0x7E) continue;
        if (!FNV_MUT(1316) && (c == '"' || c == '%' || c == '\\' || c == '<' || c == '>' || c == '&')) continue;   // twin: a percent sign reaches the HUD's format string
        if (FNV_MUT(1316) && (c == '"' || c == '\\' || c == '<' || c == '>' || c == '&')) continue;
        o.push_back(static_cast<char>(c));
        if (o.size() >= maxLen) break;
    }
    while (!o.empty() && o.back() == ' ') o.pop_back();
    size_t lead = 0;
    while (lead < o.size() && o[lead] == ' ') ++lead;
    return o.substr(lead);
}

int sectorCoord(float v) {
    if (!std::isfinite(v)) return 0;
    if (FNV_MUT(1309)) return static_cast<int>(v / kCellUnits);        // twin: truncates toward zero, so -1..-4095 is sector 0 like 0..4095
    return static_cast<int>(std::floor(v / kCellUnits));
}

static std::string sectorText(const Place& p) {
    char b[48];
    std::snprintf(b, sizeof b, "sector %d,%d", sectorCoord(p.x), sectorCoord(p.y));
    return b;
}

std::string describePlace(const Place& at, const Place& ext) {
    if (!at.valid) return "somewhere";
    if (!at.interior()) return (at.name.empty() ? std::string("outdoors") : at.name) + ", " + sectorText(at);
    std::string s = "inside " + (at.name.empty() ? std::string("a building") : at.name);
    if (ext.valid && !ext.interior()) s += ", near " + sectorText(ext);
    return s;
}

std::string distanceText(float units) {
    if (!std::isfinite(units) || units < 0) units = 0;
    const float m = FNV_MUT(1317) ? units / 7.0f : units / kUnitsPerMeter;   // twin: the wrong scale
    char b[32];
    if (m < 1000.0f) std::snprintf(b, sizeof b, "%d m", static_cast<int>(m + 0.5f));
    else std::snprintf(b, sizeof b, "%.1f km", m / 1000.0f);
    return b;
}

std::string tagText(const std::string& name, bool downed, float units) {
    std::string s = name.empty() ? std::string("?") : name;
    if (downed) s += " (down)";
    if (units >= 30.0f * kUnitsPerMeter) s += "  " + distanceText(units);
    return s;
}

std::string compassGlyph(const std::string& name, bool downed, int edge) {
    const std::string i(1, name.empty() ? '?' : static_cast<char>(std::toupper(static_cast<unsigned char>(name[0]))));
    if (edge < 0) return "<" + i;
    if (edge > 0) return i + ">";
    return (downed && !FNV_MUT(1318)) ? "[" + i + "!]" : "[" + i + "]";   // twin: a downed friend looks like any other
}

std::string clockText(uint32_t ms) {
    const uint32_t s = (ms + 999) / 1000;
    char b[24];
    std::snprintf(b, sizeof b, "%u:%02u", s / 60, s % 60);
    return b;
}

void MarkerBoard::set(Member m, uint64_t nowMs) {
    if (m.slot == kNoSlot || m.slot >= kMaxSlots) return;
    m.name = sanitizeText(m.name, kMaxNameLen);
    m.at.name = sanitizeText(m.at.name, kMaxPlaceLen);
    m.ext.name = sanitizeText(m.ext.name, kMaxPlaceLen);
    m.seenMs = nowMs;
    members_[m.slot] = std::move(m);
}

const Member* MarkerBoard::find(Slot s) const {
    auto it = members_.find(s);
    return it == members_.end() ? nullptr : &it->second;
}

std::vector<MarkerView> MarkerBoard::views(Slot self, const Place& viewer, const Place& viewerExt, float heading, uint64_t nowMs, const Puppets& puppets) const {
    std::vector<MarkerView> out;
    if (!cfg_.enabled && !FNV_MUT(1308)) return out;                                   // twin 1308: the off switch is ignored
    if (!viewer.valid) return out;
    const float headingUsed = FNV_MUT(1315) ? 0.0f : heading;                          // twin: the compass ignores which way the viewer faces
    // the worldspace the Pip-Boy's world map shows: the viewer's own outdoors, or where he last stood outdoors while inside
    const uint32_t viewedWorld = viewer.interior() ? (viewerExt.valid ? viewerExt.world : 0u) : viewer.world;
    for (const auto& kv : members_) {
        const Member& m = kv.second;
        if (m.slot == self && !FNV_MUT(1312)) continue;                                // twin 1312: a player sees himself
        if (nowMs - m.seenMs > cfg_.staleMs && !FNV_MUT(1307)) continue;               // twin 1307: a player who stopped reporting stays on screen
        MarkerView v;
        v.slot = m.slot;
        v.name = m.name;
        v.downed = m.downed;
        Place at = m.at;
        v.sameSpace = sameSpace(viewer, m.at);
        if (v.sameSpace) {
            auto p = puppets.at.find(m.slot);
            if (p != puppets.at.end() && p->second.valid && sameSpace(viewer, p->second) && !FNV_MUT(1313)) at = p->second;   // twin 1313: the table's older place always wins
        }
        // where to point at: the player, or for a player indoors the place he went in from outdoors
        Place target;
        bool haveTarget = false;
        if (v.sameSpace) { target = at; haveTarget = true; }
        else if (!viewer.interior() && m.at.interior() && m.ext.valid && m.ext.world == viewer.world) { target = m.ext; haveTarget = true; }
        if (haveTarget) v.distUnits = v.sameSpace ? distance3(viewer, target) : distance2(viewer, target);
        const bool isFar = !v.sameSpace || v.distUnits > cfg_.sectorFarUnits;
        if (isFar || FNV_MUT(1314)) v.where = FNV_MUT(1314) ? std::string() : describePlace(m.at, m.ext);   // twin 1314: a far friend is not located

        // name tag
        if (cfg_.tags && (v.sameSpace || FNV_MUT(1304))) {
            const float a = fadeAlpha(v.distUnits, cfg_.tagFullUnits, cfg_.tagMaxUnits);
            if (a > 0.0f || FNV_MUT(1303)) {
                v.tag = true;
                v.tagAlpha = FNV_MUT(1303) ? std::max(a, 0.05f) : a;                   // twin 1303: shown (faintly) past the range
                v.tagText = tagText(m.name, m.downed, v.distUnits);
                v.tagAt = at;
            }
        }
        // compass
        if (cfg_.compass && haveTarget) {
            const float dx = target.x - viewer.x, dy = target.y - viewer.y;
            float rel = wrapRad(std::atan2(dx, dy) - headingUsed);
            if (FNV_MUT(1305)) rel = -rel;                                             // twin 1305: left and right swapped
            const float half = cfg_.compassSpanRad * 0.5f;
            float x = half > 0 ? rel / half : 0.0f;
            v.compass = true;
            if (x > 1.0f || x < -1.0f) {
                v.edge = x > 0 ? 1 : -1;
                if (!FNV_MUT(1306)) x = static_cast<float>(v.edge);                                        // twin 1306: past the edge it is simply off the strip
            }
            v.compassX = x;
            v.compassGlyph = compassGlyph(m.name, m.downed, v.edge);
        }
        // map
        if (cfg_.map) {
            v.localMap = v.sameSpace || FNV_MUT(1311);                                 // twin 1311: the local map lists players from other places
            const Place wp = (m.at.interior() && !FNV_MUT(1310)) ? m.ext : m.at;       // twin 1310: an interior player has no outdoor place, so the world map misses him
            if (wp.valid && !wp.interior() && viewedWorld != 0 && wp.world == viewedWorld) v.worldMap = true;
            v.mapAt = v.localMap ? at : wp;
        }
        std::string label = m.name.empty() ? std::string("?") : m.name;
        if (m.downed) label += " (down)";
        if (haveTarget) label += ", " + distanceText(v.distUnits);
        if (!v.where.empty()) label += " - " + v.where;
        v.label = label;
        out.push_back(std::move(v));
    }
    return out;
}

}  // namespace nvmk
