// What goes where on the screen (game-free): one HudFrame plus what the glue read from the engine in, a list of texts and rectangles in window pixels out.
// The Direct3D layer (mk_overlay.*) only draws that list; nothing here touches the game.
//
//   name tags     the friend's feet position + a head height, put through the scene camera (mk_project.*), a tag over it that fades with distance
//   compass       a glyph per friend over the game's compass tile (or a strip of our own), placed by mk_layout.*
//   Pip-Boy map   a marker per friend through the visible map tile's rectangle, and a text list for whoever could not be placed
//   banner        the revive prompt / progress / downed timer, near the middle of the screen
//
// Units: tile rectangles come in the HUD's own units (the menu root's size: the window height is 960 units, the width follows the aspect, see
// docs/markers.md); the painter converts to pixels. Everything drawn is in pixels from the top left of the window.
//
// Mutation twins 1450..1464 (tests in plugin/markers/tests).
#pragma once
#include <cstdint>
#include <string>
#include <vector>

#include "mk_board.h"
#include "mk_layout.h"
#include "mk_project.h"
#include "mk_service.h"

namespace nvmk {

struct HudLayout {
    // the compass: the game's own compass tile gives the strip (tile name below); compass_x / compass_w / compass_y, when set in the ini, override
    // the matching part, and they are the strip when no compass tile can be read
    CompassOverride compass;
    std::string compassTile = "compass_window";
    int bannerY = 520;                // HUD units from the top
    int tagLift = 14;                 // HUD units a tag sits above the projected head point
    float tagHeadUnits = 135.0f;      // game units above a friend's feet where the tag point is
    // the Pip-Boy map: "auto" (the map tile's rectangle is read at run time, the text list when it cannot be), "list" (text list only), "local" or
    // "world" (markers over the map at map_rect = x,y,w,h as fractions of the screen)
    std::string mapMode = "auto";
    std::string mapWorldTile, mapLocalTile;   // optional exact tile names for auto mode (else found by name: "world" + "map" / "local" + "map")
    float mapX = 0.2f, mapY = 0.18f, mapW = 0.6f, mapH = 0.62f;
    float localMapHalfRangeUnits = 8192.0f;
};

// What the HUD needs besides the frame, each tick.
struct MapContext {
    bool pipboyMapOpen = false;               // the Pip-Boy's map menu is the top menu
    bool menuOpen = false;                    // any menu is up (the HUD parts stay hidden then)
    bool engineOk = false;                    // the engine pins matched: the tile tree may be read
    Place viewer;
    WorldMapBounds bounds;                    // of the worldspace the world map shows
};

struct SceneInput {
    float pxW = 0, pxH = 0;                   // the window's back buffer
    Rect hud;                                 // the HUD's screen in HUD units (invalid: derived from the pixel size)
    bool hudShown = true;                     // the HUD is not hidden by the player
    CameraMatrix cam;                         // the scene camera (invalid: no tags)
    TileProbe compassTile, localMap, worldMap;
    MapContext ctx;
};

enum class Align { Left, Center };
struct PText {
    std::string s;
    float x = 0, y = 0;                       // Left: the text's left edge; Center: its middle. y = top of the line.
    float h = 0;                              // line height in pixels
    Align align = Align::Center;
    uint32_t rgb = 0xFFFFFF;
    float a = 1.0f;
    bool pill = false;                        // a dark rounded-ish plate behind it
};
struct PRect {
    float x = 0, y = 0, w = 0, h = 0;
    uint32_t rgb = 0xFFFFFF;
    float a = 1.0f;
};
struct PaintList {
    std::vector<PRect> rects;                 // drawn first
    std::vector<PText> texts;
    int tags = 0, compass = 0, mapPlaced = 0;
    bool list = false;
    bool empty() const { return rects.empty() && texts.empty(); }
};

// The HUD screen in HUD units for a window of w x h pixels: the height is 960 units, the width follows the aspect (a portrait window: 1280 x ...).
Rect deriveHudScreen(float w, float h);

PaintList paintFrame(const HudFrame& f, const HudLayout& layout, const SceneInput& in);

// "Alex, 340 m - Mojave Wasteland, sector 3,-2" -> "Alex"
std::string firstField(const std::string& label);

}  // namespace nvmk
