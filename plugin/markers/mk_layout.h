// Where the markers go on the game's own screen furniture: the compass tile and the Pip-Boy map tiles (game-free). The glue reads the tiles
// (their screen rectangles, in the HUD's own units); this file decides what to do with what it read.
//
//   compass   a tile that is plausibly a compass gives the strip: centre, width and a line to centre the glyphs on. The [markers] compass_x /
//             compass_w / compass_y keys, when set, override the matching part. No usable tile: the ini (or default) strip as before.
//   map       the local and the world map are two tiles of which one is visible; a marker is placed through the tile's CURRENT rectangle (so a
//             scrolled or zoomed image moves its markers with it) and only while it lies inside the viewport; a tile that carries a zoom factor
//             the projection does not cover, an ambiguous pair, or an implausible rectangle means: no markers, the text list instead.
//
// Mutation twins 1420..1434 (tests in plugin/markers/tests).
#pragma once
#include "mk_types.h"

namespace nvmk {

// One tile's rectangle on the screen (x, y = top-left, ancestors summed), in the HUD's own units (the ones `screen()` width / height are in).
struct Rect {
    bool valid = false;
    float x = 0, y = 0, w = 0, h = 0;
    float right() const { return x + w; }
    float bottom() const { return y + h; }
};

struct TileProbe {
    Rect r;
    bool visible = true;       // the tile and every ancestor
    bool hasZoom = false;      // the tile has a `zoom` trait ...
    float zoom = 1.0f;         // ... with this value (1 and 100 both mean "not zoomed")
    Rect viewport;             // the nearest clipping ancestor, when there is one
};

bool plausibleScreen(const Rect& s);   // at least 320 x 200, finite

// ---- compass ------------------------------------------------------------------------------------------------------------------------------------
struct CompassOverride {       // the [markers] compass_x / compass_w / compass_y keys that were set
    bool hasX = false, hasW = false, hasY = false;
    float x = 0.5f, w = 0.30f; // fractions of the screen width
    float y = 36;              // pixels from the top
};
struct CompassLayout {
    bool fromTile = false;     // the game's compass tile was used (for any of the three parts)
    bool hidden = false;       // the compass tile is not shown right now (the HUD hides it): draw nothing
    float centerX = 0.5f;      // fraction of the screen width
    float width = 0.30f;       // fraction of the screen width
    float y = 36;              // top of the glyph text, in the HUD's units
    float insetUnits = 0;      // pinned glyphs keep this far from the strip's ends (half a glyph), in the HUD's units
};
// A tile is taken for a compass when it is wide, flat and on the screen.
bool plausibleCompass(const TileProbe& t, const Rect& screen);
CompassLayout compassLayout(const TileProbe& tile, const Rect& screen, const CompassOverride& ov, float glyphW = 36.0f, float glyphH = 20.0f);
// -1..1 along the strip -> fraction of the screen width where the glyph's centre goes.
float compassScreenX(const CompassLayout& l, float markX, const Rect& screen);

// ---- Pip-Boy map ----------------------------------------------------------------------------------------------------------------------------------
enum class MapKind { None, Local, World };
struct MapPick {
    MapKind kind = MapKind::None;
    TileProbe tile;
    const char* why = "";      // for the log: why there is no map to draw on
};
bool plausibleMapRect(const TileProbe& t, const Rect& screen);   // big enough, on the screen, a zoom the projection covers
MapPick chooseMap(const TileProbe& local, const TileProbe& world, const Rect& screen);
// u, v (0..1 across and down the whole image) -> where it is on the screen through the tile's current rectangle; false when it is outside the
// viewport (scrolled out of the frame).
bool mapScreenPoint(const TileProbe& map, float u, float v, float& sx, float& sy);
// The text list stays up unless every friend sits on the map as a marker.
bool mapListNeeded(bool markersActive, int placed, int friends);

}  // namespace nvmk
