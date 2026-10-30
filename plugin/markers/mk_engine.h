// Read-only engine reads of the marker HUD (x86, over the game): the scene camera's matrix and the tile tree of the HUD and the Pip-Boy map.
// NOTHING here has run in a game. Every read is SEH-guarded and only done after the pins matched; nothing is written, nothing is called.
// The facts and where each comes from are listed in docs/markers.md ("Engine addresses").
#pragma once
#include "mk_paint.h"

namespace nvcoop::markers::eng {

// The exe-identity checks (prologue bytes of three engine functions whose bodies use the offsets below), done once. False = every read here refuses.
bool pinsOk();

// The world scene camera: the 16 floats of its world-to-camera matrix, its viewport, and the checks that it is the camera of the screen (near the
// player, a matrix that projects a point ahead of it into the middle of the view). `viewer` is the local player's place.
bool readCamera(const nvmk::Place& viewer, nvmk::CameraMatrix& out, const char** why);

// The HUD's own screen size in HUD units (the interface manager's menu root: height 960, width follows the aspect), false when it cannot be read.
bool readHudRoot(nvmk::Rect& out);

struct HudTiles {
    bool rootFound = false;
    bool shown = true;                  // HUDMainMenu's own visible trait (the player can hide the HUD)
    bool compassFound = false;
    nvmk::TileProbe compass;
    char compassName[40] = {};
};
bool readHudTiles(const char* compassTileName, HudTiles& out);

struct MapTiles {
    bool menuFound = false;
    bool localFound = false, worldFound = false;
    nvmk::TileProbe local, world;
    char localName[40] = {}, worldName[40] = {};
};
// `worldName` / `localName`: exact tile names from the ini, or empty (found by name: "world" + "map" / "local" + "map"). `dump`: log the tiles of
// 60 x 40 HUD units or more (once per opening of the map) so the names can be corrected in the ini.
bool readMapTiles(const char* worldName, const char* localName, MapTiles& out, bool dump);

}  // namespace nvcoop::markers::eng
