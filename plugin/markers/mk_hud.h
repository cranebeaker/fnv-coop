// The HUD half of the marker service, over the game (x86, xNVSE). NOTHING here has run in a game.
//
// How it draws (the way the other co-op mod puts its friend markers on the screen, adapted to our puppets and extended):
//   * JIP LN's InjectUIComponent puts small text tiles into HUDMainMenu (and MapMenu for the Pip-Boy list); SetUIStringEx / SetUIFloat change them.
//   * Johnny Guitar's WorldToScreen says where a point above a puppet is on the screen (0..1 across and down; 0 when off screen or behind).
//   * Both are called from scripts compiled at start with xNVSE's NVSEScriptInterface::CompileScript. A script that does not compile (the plugin is not
//     installed) switches the surface off; the banner then falls back to the game's notification line.
//
// Surfaces: name tag over each puppet in view (fades with distance), a glyph per friend over the game's compass tile (a strip of our own where that tile
// cannot be read), the banner (revive prompt / progress / downed timer), and the Pip-Boy's map: markers over the map tile's rectangle read at run time
// (map_mode=auto), with a text list as the fallback.
//
// Tile reads walk the tile tree of HUDMainMenu / MapMenu with the structs of xNVSE's GameTiles.h and GameUI.h (children, values, locus-relative x / y,
// the way xNVSE's GetUIFloatInherited adds them up), SEH-guarded; the layout decisions are in mk_layout.*.
#pragma once
#include <functional>
#include <string>

#include "mk_board.h"
#include "mk_layout.h"
#include "mk_service.h"

struct NVSEInterface;
class TESObjectREFR;

namespace nvcoop::markers {

struct HudLayout {
    // the compass: the game's own compass tile gives the strip (tile name below); compass_x / compass_w / compass_y, when set in the ini, override
    // the matching part, and they are the strip when no compass tile can be read
    nvmk::CompassOverride compass;
    std::string compassTile = "compass_window";
    int bannerY = 520;
    int tagLift = 75;                 // pixels above the point WorldToScreen gives (150 units over the puppet's feet)
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
    bool engineOk = false;                    // the engine pins matched: the tile tree may be read
    nvmk::Place viewer;
    nvmk::WorldMapBounds bounds;              // of the worldspace the world map shows
};

class ScriptHud : public nvmk::IMarkerHud {
public:
    ScriptHud();
    ~ScriptHud();
    bool init(NVSEInterface* nvse, const HudLayout& layout);   // compiles the scripts; false = no HUD (notification fallback)
    bool ready() const;
    void setRefLookup(std::function<TESObjectREFR*(nvmk::Slot)> f);
    void setMapContext(const MapContext& c);
    void show(const nvmk::HudFrame& f) override;
    void hide();
    std::string statusJson() const;

private:
    struct Impl;
    Impl* d_;
};

}  // namespace nvcoop::markers
