// The HUD half of the marker service, over the game (x86). NOTHING here has run in a game.
//
// How it draws: the marker service hands over one HudFrame per tick (show). Each time the game presents a frame, a Direct3D 9 overlay (mk_overlay.*)
// asks this class what to put on the window. This class then reads, read-only and under SEH, what the painter needs from the engine (mk_engine.*):
//   * the scene camera's world-to-camera matrix, to put a point over each friend's head on the window ourselves (mk_project.*);
//   * the rectangles of the game's compass tile and of the Pip-Boy's map tiles (the tile tree of HUDMainMenu / MapMenu), and the HUD's own screen size;
// and the painter (mk_paint.*, game-free and tested) turns frame + reads into texts and rectangles in window pixels. No script is compiled, no
// UI tile is created or changed, and no other plugin is needed.
//
// Surfaces: name tag over each friend in view (fades with distance), a glyph per friend over the game's compass tile (a strip of our own where that
// tile cannot be read), the banner (revive prompt / progress / downed timer), and the Pip-Boy's map: markers over the map tile's current rectangle
// (map_mode=auto), with a text list as the fallback. Without the overlay (the Present hook did not go in) the banner goes to the notification line.
#pragma once
#include <string>

#include "mk_paint.h"
#include "mk_service.h"

namespace nvcoop::markers {

using nvmk::HudLayout;
using nvmk::MapContext;

class OverlayHud : public nvmk::IMarkerHud {
public:
    OverlayHud();
    ~OverlayHud();
    bool init(const HudLayout& layout);   // hooks the device's Present; false = no HUD (notification fallback)
    bool ready() const;
    void setMapContext(const MapContext& c);
    void show(const nvmk::HudFrame& f) override;
    void hide();
    std::string statusJson() const;

private:
    struct Impl;
    Impl* d_;
};

}  // namespace nvcoop::markers
