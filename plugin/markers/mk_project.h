// World point -> screen point through the scene camera (game-free).
//
// The engine's scene camera (an NiCamera) keeps a 4x4 "world to camera" matrix, row-major, with the projection already folded in: for a world
// point p, w = row3 . (p,1), and (row0 . (p,1) / w, row1 . (p,1) / w) are the horizontal and vertical position inside the view, -1 .. +1 across.
// The camera also has a viewport rectangle (left, right, top, bottom as fractions of the window, normally 0, 1, 1, 0). The glue copies the 16
// floats and the four viewport floats out of the camera object; everything from there is arithmetic, tested without the game.
//
// Mutation twins 1441..1447 (tests in plugin/markers/tests).
#pragma once
#include "mk_types.h"

namespace nvmk {

struct CameraMatrix {
    bool valid = false;
    float m[16] = {};                                      // m[row * 4 + col]; row 3 is the depth row
    float portL = 0.0f, portR = 1.0f, portT = 1.0f, portB = 0.0f;
};

// Where `world` lands in a window of w x h pixels (origin top-left). False when the point is behind the camera, or outside the view by more than
// `margin` (a fraction of the half view: 0.05 lets a name tag that is half off the edge still show). `depth` is the distance along the view axis.
bool projectPoint(const CameraMatrix& cam, float wx, float wy, float wz, float w, float h, float margin, float& sx, float& sy, float& depth);

// A matrix read out of memory is only used when it behaves like a camera: finite, and the point 100 units straight ahead of the camera sits in
// front of it near the middle of the view. `ahead` is the camera's view axis (unit length), `pos` where it stands.
bool plausibleCamera(const CameraMatrix& cam, const float pos[3], const float ahead[3]);

// The scene camera follows the player (first or third person): a camera far from the player is not the one the screen shows.
bool cameraNearPlayer(const float camPos[3], float px, float py, float pz, float maxUnits);

}  // namespace nvmk
