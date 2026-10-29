// The marker board: which other players this machine knows, and, for the local viewer, what to show of each (game-free).
//
//   name tag      over the puppet when it is in the viewer's space and within tagMaxUnits; full strength up to tagFullUnits, then fading out
//   compass       the bearing against the viewer's heading, -1..1 across the strip; a player off the strip is pinned to its edge (nobody is lost)
//   map           the local map draws players in the viewer's space; the world map draws players outdoors in the worldspace the map shows
//                 (a player inside an interior at the place they went in)
//   where         the place and sector text of a player who is far away or elsewhere
//
// Mutation twins 1301..1330 (tests in plugin/markers/tests).
#pragma once
#include <map>
#include <string>
#include <vector>

#include "mk_types.h"

namespace nvmk {

struct MarkerConfig {
    bool enabled = true;           // [markers] enabled
    bool tags = true;              // [markers] tags
    bool compass = true;           // [markers] compass
    bool map = true;               // [markers] map
    float tagFullUnits = 1400.0f;  // full strength to 20 m
    float tagMaxUnits = 7000.0f;   // gone at 100 m
    float compassSpanRad = kPi;    // the strip shows 180 degrees: -90 .. +90 from the heading
    float sectorFarUnits = 12000.0f;   // from this far (about 170 m) the place and sector are named too
    uint32_t staleMs = 4000;       // a player not heard of for this long is not shown
};

struct MarkerView {
    Slot slot = kNoSlot;
    std::string name;
    bool downed = false;
    bool sameSpace = false;
    float distUnits = 0;           // 3D when in the same space, else the 2D distance of the world positions (0 when it has none)
    // name tag
    bool tag = false;
    float tagAlpha = 0;            // 0..1
    std::string tagText;
    Place tagAt;                   // the point to draw the tag over (feet; the HUD adds the head height)
    // compass
    bool compass = false;
    float compassX = 0;            // -1 (left edge) .. 1 (right edge)
    int edge = 0;                  // -1 / +1: beyond the strip's edge, drawn pinned to it
    std::string compassGlyph;
    // map
    bool localMap = false;
    bool worldMap = false;
    Place mapAt;                   // where to draw it
    // text for the "who is where" list
    std::string where;             // "" for a player in view range; else place and sector text
    std::string label;             // "Alex, 340 m" (+ where)
};

class MarkerBoard {
public:
    explicit MarkerBoard(MarkerConfig cfg = {}) : cfg_(cfg) {}
    void setConfig(const MarkerConfig& c) { cfg_ = c; }
    const MarkerConfig& config() const { return cfg_; }

    // One entry per other player. `nowMs` stamps it (this machine's clock).
    void set(Member m, uint64_t nowMs);
    void remove(Slot s) { members_.erase(s); }
    void clear() { members_.clear(); }
    const Member* find(Slot s) const;
    const std::map<Slot, Member>& members() const { return members_; }

    // The views for `viewer` (its slot is never shown). `puppetAt` gives the exact place of a puppet that is loaded here (it is smoother than
    // the table's), used for the tag and the distance when it is in the viewer's space. Ascending by slot. Empty when disabled.
    struct Puppets { std::map<Slot, Place> at; };
    std::vector<MarkerView> views(Slot self, const Place& viewer, const Place& viewerExt, float heading, uint64_t nowMs, const Puppets& puppets = {}) const;

private:
    MarkerConfig cfg_;
    std::map<Slot, Member> members_;
};

float fadeAlpha(float dist, float full, float maxd);               // 1 up to `full`, 0 from `maxd`, linear between

// ---- map projection (the glue draws markers over the Pip-Boy's map with these) ------------------------------------------------------------------
struct WorldMapBounds {
    bool valid = false;
    int nwX = 0, nwY = 0, seX = 0, seY = 0;   // cell coordinates of the map's north-west and south-east corners (TESWorldSpace::mapData)
};
// 0..1 across (west to east) and down (north to south) of the world map; false outside the map or without bounds.
bool worldMapUV(const Place& p, const WorldMapBounds& b, float& u, float& v);
// The local map: north up, the viewer in the middle, `halfRangeUnits` from the middle to an edge. False outside.
bool localMapUV(const Place& viewer, const Place& p, float halfRangeUnits, float& u, float& v);

// ---- text -------------------------------------------------------------------------------------------------------------------------------------
std::string sanitizeText(const std::string& s, size_t maxLen);   // printable ASCII only, no quotes / percent signs / backslashes (a name goes into HUD format strings)
int sectorCoord(float v);                                         // floor(v / 4096)
std::string describePlace(const Place& at, const Place& ext);     // "Mojave Wasteland, sector 3,-2" / "inside Vault 21, near sector 3,-2"
std::string distanceText(float units);                            // "340 m", "1.2 km"
std::string tagText(const std::string& name, bool downed, float units);
std::string compassGlyph(const std::string& name, bool downed, int edge);
std::string clockText(uint32_t ms);                               // "0:42"

}  // namespace nvmk
