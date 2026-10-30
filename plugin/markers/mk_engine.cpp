// See mk_engine.h. UNPROVEN IN GAME: the addresses and offsets come from static reading of the 1.4.0.525 exe (docs/markers.md, "Engine addresses").
#include <Windows.h>

#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstring>

#include "GameAPI.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "GameScript.h"
#include "GameUI.h"
#include "PluginAPI.h"
#include "bots/bots_addrs.h"
#include "common/nvcoop_api.h"
#include "mk_engine.h"
#include "mk_project.h"

namespace nvcoop::markers::eng {

namespace {

// ---------------------------------------------------------------------------------------------------------------------------------- the pins
// Data: the exe's .rdata (vtables) is 0xFDF000 .. 0x1183000.
constexpr unsigned kRdataLo = 0xFDF000u, kRdataHi = 0x1183000u;

// The world scene graph: a smart pointer at 0x11DEB7C. Found in the exe's start-up code that builds the scene graph named "World" (constructor
// 0x878610, vtable 0x1083B5C) and stores it there; the same code then reads the camera through 0x6629F0.
constexpr unsigned kWorldSceneGraphPtr = 0x11DEB7Cu;
constexpr unsigned kSceneGraphVtbl = 0x1083B5Cu;     // RTTI SceneGraph (type descriptor 0x11A31A8) -> vtable
constexpr unsigned kSceneGraphCamera = 0xACu;        // 0x6629F0 returns [this + 0xAC]
constexpr unsigned kNiCameraVtbl = 0x109CB9Cu;       // RTTI NiCamera (type descriptor 0x11A9F38) -> vtable
// NiCamera: the world-to-camera matrix at +0x9C (16 floats, row 3 = depth), the viewport rectangle at +0x100 (left, right, top, bottom), the frustum
// at +0xDC. All three are read by NiCamera::WorldPtToScreenPt3 at 0xA6FC50 (rows at +0x9C, +0xAC, +0xBC, +0xCC; viewport at +0x100..+0x10C).
constexpr unsigned kCamWorldToCam = 0x9Cu;
constexpr unsigned kCamPort = 0x100u;
// NiAVObject: the world rotation (3 x 3, row-major) at +0x68, the world translation at +0x8C. The camera's view axis is the first COLUMN of the
// rotation (+0x68, +0x74, +0x80): the code at 0xA70080 takes dot(point - translation, column 0) as the depth along the view.
constexpr unsigned kNiWorldRot = 0x68u;
constexpr unsigned kNiWorldPos = 0x8Cu;

// The interface manager (0x11D8A80, also pinned by mk_glue.cpp) holds the root tile of the whole UI at +0x9C; 0x715E00 (resolution set-up) writes its
// width and height from 0x715D40 / 0x715DA0 (height 960 units, width = 960 x aspect on a landscape window).
constexpr unsigned kInterfaceMgrPtr = 0x11D8A80u;
constexpr unsigned kMenuRootOff = 0x9Cu;
// The menu tiles: g_tileMenuArray (xNVSE GameUI.cpp) at 0x11F350C, indexed by menu id - 0x3E9; the lookup function is pinned in plugin/bots.
constexpr unsigned kTileMenuArrayData = 0x11F350Cu;
constexpr unsigned kMenuIdMin = 0x3E9u, kMenuIdHud = 0x3ECu, kMenuIdMap = 0x3FFu;   // xNVSE GameUI.h kMenuType_*

struct CodePin {
    unsigned addr;
    unsigned char bytes[16];
    const char* what;
};
constexpr CodePin kCamPins[] = {
    {0xA6FC50u, {0x8b, 0x54, 0x24, 0x04, 0xd9, 0x81, 0xcc, 0x00, 0x00, 0x00, 0xd8, 0x0a, 0x56, 0xd9, 0x81, 0xd0}, "NiCamera::WorldPtToScreenPt3 (matrix +0x9C.., viewport +0x100..)"},
    {0x6629F0u, {0x55, 0x8b, 0xec, 0x51, 0x89, 0x4d, 0xfc, 0x8b, 0x4d, 0xfc, 0x81, 0xc1, 0xac, 0x00, 0x00, 0x00}, "SceneGraph::GetCamera (+0xAC)"},
    {0xA712F0u, {0x83, 0xec, 0x0c, 0x53, 0x56, 0x57, 0x8b, 0xf1, 0xe8, 0x23, 0x97, 0xfe, 0xff, 0x8d, 0xbe, 0xdc}, "NiCamera constructor (frustum +0xDC)"},
};
constexpr CodePin kRootPins[] = {
    {0x715E00u, {0x55, 0x8b, 0xec, 0x51, 0x89, 0x4d, 0xfc, 0x83, 0x7d, 0x08, 0x00, 0x7e, 0x32, 0x8b, 0x45, 0x08}, "InterfaceManager resolution set-up (root tile at +0x9C)"},
    {0x715D40u, {0x55, 0x8b, 0xec, 0x83, 0xec, 0x14, 0xe8, 0xa5, 0x64, 0xdc, 0xff, 0x89, 0x45, 0xf4, 0xdb, 0x45}, "UI width from the window size"},
};

bool PeekOk(unsigned addr, const unsigned char* want, size_t n) {
    unsigned char got[16] = {};
    __try {
        std::memcpy(got, reinterpret_cast<const void*>(addr), n);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
    return std::memcmp(got, want, n) == 0;
}

bool InRdata(unsigned v) { return v >= kRdataLo && v < kRdataHi; }

// ---------------------------------------------------------------------------------------------------------------------------------- tile reads
// Tile (xNVSE GameTiles.h): child list = a doubly linked list (ChildNode: next, prev, child) whose first node is at +0x04; values = array of
// Value* at +0x10 (id, parent, num, str, action); name at +0x20. A tile's place on the screen is its x / y plus the x / y of those ancestors
// that have a non-zero `locus` (the sum xNVSE's GetUIFloatInherited makes).
struct TileInfo {
    Tile* tile;
    const char* name;      // up to 39 chars, copied
    char nameBuf[40];
    int depth;
    float ax, ay, w, h;
    bool shown;
    bool hasZoom;
    float zoom;
    bool hasView;
    float vx, vy, vw, vh;
};
using Visit = void (*)(void* ctx, const TileInfo& t);

struct Walker {
    Visit visit;
    void* ctx;
    int visited;
};

bool NumOf(Tile* t, UInt32 id, float& out) {
    Tile::Value** d = t->values.data;
    const UInt32 n = t->values.size;
    if (!d || n > 1024) return false;
    for (UInt32 i = 0; i < n; ++i) {
        const Tile::Value* v = d[i];
        if (v && v->id == id) {
            out = v->num;
            return true;
        }
    }
    return false;
}

bool LooksLikeTile(const void* p) {
    const unsigned a = reinterpret_cast<unsigned>(p);
    if (a < 0x10000u || (a & 3u)) return false;
    return InRdata(*reinterpret_cast<const unsigned*>(a));
}

void WalkTile(Tile* t, int depth, float baseX, float baseY, bool shownAbove, bool hasView, float vx, float vy, float vw, float vh, Walker& wk) {
    if (!t || depth > 14 || wk.visited >= 2000 || !LooksLikeTile(t)) return;
    ++wk.visited;
    float x = 0, y = 0, w = 0, h = 0, vis = 1, locus = 0, zoom = 0, clips = 0, clipw = 0;
    NumOf(t, Tile::kTileValue_x, x);
    NumOf(t, Tile::kTileValue_y, y);
    NumOf(t, Tile::kTileValue_width, w);
    NumOf(t, Tile::kTileValue_height, h);
    const bool hasVis = NumOf(t, Tile::kTileValue_visible, vis);
    NumOf(t, Tile::kTileValue_locus, locus);
    const bool hasZoom = NumOf(t, Tile::kTileValue_zoom, zoom);
    NumOf(t, Tile::kTileValue_clips, clips);
    NumOf(t, Tile::kTileValue_clipwindow, clipw);
    TileInfo ti;
    std::memset(&ti, 0, sizeof ti);
    ti.tile = t;
    if (const char* nm = t->name.m_data)
        for (int i = 0; i < 39 && nm[i]; ++i) ti.nameBuf[i] = nm[i];
    ti.name = ti.nameBuf;
    ti.depth = depth;
    ti.ax = baseX + x;
    ti.ay = baseY + y;
    ti.w = w;
    ti.h = h;
    ti.shown = shownAbove && (!hasVis || vis != 0.0f);
    ti.hasZoom = hasZoom;
    ti.zoom = zoom;
    ti.hasView = hasView;
    ti.vx = vx; ti.vy = vy; ti.vw = vw; ti.vh = vh;
    if (wk.visit) wk.visit(wk.ctx, ti);
    const float nbx = baseX + (locus != 0.0f ? x : 0.0f), nby = baseY + (locus != 0.0f ? y : 0.0f);
    const bool clipping = (clips != 0.0f || clipw != 0.0f) && w > 0.0f && h > 0.0f;
    int n = 0;
    for (Tile::ChildNode* c = t->childList.m_listHead.data; c && n < 256; c = c->next, ++n)
        WalkTile(c->child, depth + 1, nbx, nby, ti.shown, clipping || hasView, clipping ? ti.ax : vx, clipping ? ti.ay : vy, clipping ? w : vw, clipping ? h : vh, wk);
}

// The root tile of an open menu (0 when the menu is not open).
Tile* MenuRoot(unsigned menuId) {
    const unsigned arr = *reinterpret_cast<const unsigned*>(kTileMenuArrayData);
    if (!arr) return nullptr;
    Tile* root = reinterpret_cast<Tile* const*>(arr)[menuId - kMenuIdMin];
    return LooksLikeTile(root) ? root : nullptr;
}

bool ScanMenu(unsigned menuId, Walker& wk, bool* rootShown) {
    __try {
        Tile* root = MenuRoot(menuId);
        if (!root) return false;
        if (rootShown) {
            float vis = 1;
            *rootShown = !NumOf(root, Tile::kTileValue_visible, vis) || vis != 0.0f;
        }
        WalkTile(root, 0, 0, 0, true, false, 0, 0, 0, 0, wk);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

char Low(char c) { return (c >= 'A' && c <= 'Z') ? static_cast<char>(c - 'A' + 'a') : c; }
bool SameName(const char* a, const char* b) {
    for (; *a && *b; ++a, ++b)
        if (Low(*a) != Low(*b)) return false;
    return *a == 0 && *b == 0;
}
bool HasWord(const char* hay, const char* needle) {
    for (; *hay; ++hay) {
        const char* h = hay;
        const char* n = needle;
        while (*h && *n && Low(*h) == Low(*n)) { ++h; ++n; }
        if (!*n) return true;
    }
    return false;
}

void FillProbe(nvmk::TileProbe& p, const TileInfo& t) {
    p.r.valid = true;
    p.r.x = t.ax; p.r.y = t.ay; p.r.w = t.w; p.r.h = t.h;
    p.visible = t.shown;
    p.hasZoom = t.hasZoom;
    p.zoom = t.zoom;
    if (t.hasView) {
        p.viewport.valid = true;
        p.viewport.x = t.vx; p.viewport.y = t.vy; p.viewport.w = t.vw; p.viewport.h = t.vh;
    }
}
void CopyName(char* dst, const char* src) {
    std::memcpy(dst, src, 39);
    dst[39] = 0;
}

struct HudCtx {
    const char* want;
    eng::HudTiles* out;
};
void VisitHud(void* c, const TileInfo& t) {
    HudCtx& cx = *static_cast<HudCtx*>(c);
    if (!t.nameBuf[0] || !(t.w > 0.0f) || !(t.h > 0.0f)) return;
    if (!SameName(t.nameBuf, cx.want) && !SameName(t.nameBuf, "CompassWindow")) return;
    HudTiles& o = *cx.out;
    if (o.compassFound && t.w * t.h <= o.compass.r.w * o.compass.r.h) return;   // the biggest of the namesakes
    o.compassFound = true;
    o.compass = nvmk::TileProbe{};
    FillProbe(o.compass, t);
    CopyName(o.compassName, t.nameBuf);
}

struct MapCtx {
    const char* worldName;
    const char* localName;
    MapTiles* out;
    bool dump;
    int dumped;
};
void VisitMap(void* c, const TileInfo& t) {
    MapCtx& cx = *static_cast<MapCtx*>(c);
    if (cx.dump && cx.dumped < 100 && t.w >= 60.0f && t.h >= 40.0f) {
        ++cx.dumped;
        nvcoop::Log("markers: map tile %*s%s  at %.0f,%.0f  %.0fx%.0f  %s", t.depth * 2, "", t.nameBuf, t.ax, t.ay, t.w, t.h, t.shown ? "shown" : "hidden");
    }
    if (!t.nameBuf[0] || !(t.w > 0.0f) || !(t.h > 0.0f)) return;
    MapTiles& o = *cx.out;
    const bool world = (cx.worldName && cx.worldName[0]) ? SameName(t.nameBuf, cx.worldName) : (HasWord(t.nameBuf, "world") && HasWord(t.nameBuf, "map"));
    const bool local = (cx.localName && cx.localName[0]) ? SameName(t.nameBuf, cx.localName) : (HasWord(t.nameBuf, "local") && HasWord(t.nameBuf, "map"));
    if (world && (!o.worldFound || t.w * t.h > o.world.r.w * o.world.r.h)) {
        o.worldFound = true;
        o.world = nvmk::TileProbe{};
        FillProbe(o.world, t);
        CopyName(o.worldName, t.nameBuf);
    }
    if (local && (!o.localFound || t.w * t.h > o.local.r.w * o.local.r.h)) {
        o.localFound = true;
        o.local = nvmk::TileProbe{};
        FillProbe(o.local, t);
        CopyName(o.localName, t.nameBuf);
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------- camera
struct RawCam {
    float m[16];
    float port[4];
    float pos[3];
    float ahead[3];
    unsigned vtbl;
    int why;   // 0 ok, 1 no scene graph, 2 scene graph is not the one we know, 3 no camera, 4 camera object is not a camera
};

bool ReadCameraRaw(RawCam& o) {
    __try {
        const unsigned sg = *reinterpret_cast<const unsigned*>(kWorldSceneGraphPtr);
        if (!sg) { o.why = 1; return false; }
        if (*reinterpret_cast<const unsigned*>(sg) != kSceneGraphVtbl) { o.why = 2; return false; }
        const unsigned cam = *reinterpret_cast<const unsigned*>(sg + kSceneGraphCamera);
        if (!cam || (cam & 3u)) { o.why = 3; return false; }
        o.vtbl = *reinterpret_cast<const unsigned*>(cam);
        if (!InRdata(o.vtbl)) { o.why = 4; return false; }
        std::memcpy(o.m, reinterpret_cast<const void*>(cam + kCamWorldToCam), sizeof o.m);
        std::memcpy(o.port, reinterpret_cast<const void*>(cam + kCamPort), sizeof o.port);
        const float* rot = reinterpret_cast<const float*>(cam + kNiWorldRot);
        o.ahead[0] = rot[0]; o.ahead[1] = rot[3]; o.ahead[2] = rot[6];     // column 0 of a row-major 3 x 3
        std::memcpy(o.pos, reinterpret_cast<const void*>(cam + kNiWorldPos), sizeof o.pos);
        o.why = 0;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        o.why = 3;
        return false;
    }
}

bool ReadRootRaw(float& w, float& h) {
    __try {
        const unsigned mgr = *reinterpret_cast<const unsigned*>(kInterfaceMgrPtr);
        if (!mgr) return false;
        Tile* root = *reinterpret_cast<Tile* const*>(mgr + kMenuRootOff);
        if (!LooksLikeTile(root)) return false;
        return NumOf(root, Tile::kTileValue_width, w) && NumOf(root, Tile::kTileValue_height, h);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

}  // namespace

bool pinsOk() {
    static int state = -1;
    if (state >= 0) return state == 1;
    bool ok = true;
    for (const CodePin& p : kCamPins)
        if (!PeekOk(p.addr, p.bytes, 16)) { ok = false; nvcoop::Log("markers: engine pin does not match: %s at 0x%X", p.what, p.addr); }
    for (const CodePin& p : kRootPins)
        if (!PeekOk(p.addr, p.bytes, 16)) { ok = false; nvcoop::Log("markers: engine pin does not match: %s at 0x%X", p.what, p.addr); }
    namespace b = nvbots::addr;
    if (!PeekOk(b::GetMenuByType.addr, b::GetMenuByType.bytes, 16)) { ok = false; nvcoop::Log("markers: engine pin does not match: GetMenuByType at 0x%X", b::GetMenuByType.addr); }
    state = ok ? 1 : 0;
    nvcoop::Log("markers: HUD engine pins %s (camera, UI root, menu tiles)", ok ? "match" : "DO NOT match: no tags over heads, no tile reads");
    return ok;
}

bool readCamera(const nvmk::Place& viewer, nvmk::CameraMatrix& out, const char** why) {
    static const char* const kWhy[] = {"ok", "no world scene graph yet", "the object at the scene graph pointer is not a SceneGraph", "the scene graph has no camera",
                                       "the camera object has no vtable in the exe's data", "the matrix does not behave like a camera", "the camera is far from the player", "the engine pins do not match"};
    auto fail = [&](int i) {
        if (why) *why = kWhy[i];
        out = nvmk::CameraMatrix{};
        return false;
    };
    if (!pinsOk()) return fail(7);
    RawCam raw;
    std::memset(&raw, 0, sizeof raw);
    if (!ReadCameraRaw(raw)) return fail(raw.why);
    nvmk::CameraMatrix c;
    c.valid = true;
    std::memcpy(c.m, raw.m, sizeof c.m);
    c.portL = raw.port[0]; c.portR = raw.port[1]; c.portT = raw.port[2]; c.portB = raw.port[3];
    if (!nvmk::plausibleCamera(c, raw.pos, raw.ahead)) return fail(5);
    if (viewer.valid && !nvmk::cameraNearPlayer(raw.pos, viewer.x, viewer.y, viewer.z, 1500.0f)) return fail(6);
    out = c;
    if (why) *why = kWhy[0];
    return true;
}

bool readHudRoot(nvmk::Rect& out) {
    if (!pinsOk()) return false;
    float w = 0, h = 0;
    if (!ReadRootRaw(w, h) || !(w >= 320.0f) || !(h >= 200.0f) || !std::isfinite(w) || !std::isfinite(h)) return false;
    out = nvmk::Rect{};
    out.valid = true;
    out.w = w; out.h = h;
    return true;
}

bool readHudTiles(const char* compassTileName, HudTiles& out) {
    out = HudTiles{};
    if (!pinsOk()) return false;
    HudCtx cx{compassTileName && compassTileName[0] ? compassTileName : "compass_window", &out};
    Walker wk{VisitHud, &cx, 0};
    out.rootFound = ScanMenu(kMenuIdHud, wk, &out.shown);
    return out.rootFound;
}

bool readMapTiles(const char* worldName, const char* localName, MapTiles& out, bool dump) {
    out = MapTiles{};
    if (!pinsOk()) return false;
    MapCtx cx{worldName, localName, &out, dump, 0};
    Walker wk{VisitMap, &cx, 0};
    out.menuFound = ScanMenu(kMenuIdMap, wk, nullptr);
    return out.menuFound;
}

}  // namespace nvcoop::markers::eng
