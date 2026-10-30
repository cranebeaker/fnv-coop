// A Direct3D 9 overlay for the marker HUD (x86, over the game). NOTHING here has run in a game.
//
// The game's device is found the way plugin/bridge/screenshot.cpp finds it: a throw-away device on a hidden window gives the address of the shared
// device vtable, and the Present slot of it is patched once. Each time the game presents a frame, `Provider` is asked what to draw (a PaintList in
// window pixels, see mk_paint.h); the overlay draws it onto the back buffer with its own glyph atlas and one batch of triangles, then lets the real
// Present go on. The device's state is saved with a state block and restored, and the render target is put back, so the game's renderer never sees
// the draw. Raw COM vtable calls only (no d3d9.h / d3d9.lib), every call under SEH; ten failed frames in a row switch the overlay off.
#pragma once
#include <string>

#include "mk_paint.h"

namespace nvcoop::markers::overlay {

// Called on the thread that presents, once per frame, with the back buffer's size in pixels. Fill `out`; return false to draw nothing this frame.
using Provider = bool (*)(void* ctx, float w, float h, nvmk::PaintList& out);

bool install(Provider provider, void* ctx, std::string& err);   // idempotent; false = no overlay (the banner then falls back to the notification line)
bool installed();
void setEnabled(bool on);                                       // the hook stays, drawing stops (the module is off / no session)
std::string statusJson();

}  // namespace nvcoop::markers::overlay
