// See mk_hud.h. UNPROVEN IN GAME: compiles against the xNVSE headers; the engine facts are in mk_engine.cpp and docs/markers.md.
#include <Windows.h>
#ifdef min
#undef min
#endif
#ifdef max
#undef max
#endif

#include <cstdint>
#include <cstdio>
#include <cstring>
#include <string>

#include "common/nvcoop_api.h"
#include "mk_engine.h"
#include "mk_hud.h"
#include "mk_overlay.h"

namespace nvcoop::markers {

namespace {
constexpr uint64_t kFrameFreshMs = 600;     // a frame older than this is not drawn (the service stopped ticking: loading screen, menu, session over)
constexpr uint64_t kHudReadMs = 250;        // the compass tile is re-read this often
constexpr uint64_t kRootReadMs = 2000;
uint64_t NowMs() { return GetTickCount64(); }
}  // namespace

struct OverlayHud::Impl {
    mutable CRITICAL_SECTION cs;
    nvmk::HudLayout layout;
    bool installed = false;
    std::string installErr;

    // handed over by the service / the glue (any thread that ticks); read when the game presents
    nvmk::HudFrame frame;
    uint64_t frameAt = 0;
    nvmk::MapContext ctx;

    // render-side caches
    nvmk::Rect root;
    uint64_t rootAt = 0;
    eng::HudTiles hudTiles;
    uint64_t hudAt = 0;
    eng::MapTiles mapTiles;
    bool mapWasOpen = false;
    nvmk::CameraMatrix cam;
    const char* camWhy = "not read yet";
    const char* camWhyLogged = nullptr;
    bool loggedRoot = false, loggedCompass = false, loggedMap = false;
    // counters
    uint64_t frames = 0, painted = 0, tags = 0, compass = 0, mapPlaced = 0, camOk = 0, camFail = 0;
    char lastMapWhy[64] = "";

    Impl() { InitializeCriticalSection(&cs); }
    ~Impl() { DeleteCriticalSection(&cs); }

    void renderSide(float w, float h, const nvmk::HudFrame& f, const nvmk::MapContext& c, nvmk::PaintList& out);
    static bool provide(void* self, float w, float h, nvmk::PaintList& out);
};

void OverlayHud::Impl::renderSide(float w, float h, const nvmk::HudFrame& f, const nvmk::MapContext& c, nvmk::PaintList& out) {
    const uint64_t now = NowMs();
    nvmk::SceneInput s;
    s.pxW = w;
    s.pxH = h;
    s.ctx = c;
    const bool pip = c.pipboyMapOpen;
    const bool engine = c.engineOk && eng::pinsOk();

    if (engine) {
        if (now - rootAt >= kRootReadMs || !root.valid) {
            rootAt = now;
            nvmk::Rect r;
            if (eng::readHudRoot(r)) root = r;
            if (root.valid && !loggedRoot) {
                loggedRoot = true;
                nvcoop::Log("markers: hud: the UI root is %.0f x %.0f units for a %.0f x %.0f window", root.w, root.h, w, h);
            }
        }
        s.hud = root;
        if (pip) {
            eng::MapTiles mt;
            eng::readMapTiles(layout.mapWorldTile.c_str(), layout.mapLocalTile.c_str(), mt, !mapWasOpen && !loggedMap);
            mapTiles = mt;
            if (!mapWasOpen && !loggedMap) {
                loggedMap = true;
                nvcoop::Log("markers: hud: map menu %s; world tile %s, local tile %s", mt.menuFound ? "read" : "NOT readable", mt.worldFound ? mt.worldName : "not found",
                            mt.localFound ? mt.localName : "not found");
            }
            s.worldMap = mt.world;
            s.localMap = mt.local;
        } else if (now - hudAt >= kHudReadMs) {
            hudAt = now;
            eng::HudTiles ht;
            eng::readHudTiles(layout.compassTile.c_str(), ht);
            hudTiles = ht;
            if (!loggedCompass && ht.rootFound) {
                loggedCompass = true;
                if (ht.compassFound)
                    nvcoop::Log("markers: hud: compass tile \"%s\" at %.0f,%.0f  %.0fx%.0f  %s", ht.compassName, ht.compass.r.x, ht.compass.r.y, ht.compass.r.w, ht.compass.r.h,
                                ht.compass.visible ? "shown" : "hidden");
                else
                    nvcoop::Log("markers: hud: no tile named \"%s\" under HUDMainMenu: the compass strip is our own", layout.compassTile.c_str());
            }
        }
        if (!pip) {
            s.compassTile = hudTiles.compassFound ? hudTiles.compass : nvmk::TileProbe{};
            s.hudShown = !hudTiles.rootFound || hudTiles.shown;
        }
    }
    mapWasOpen = pip;

    if (!pip && !c.menuOpen && !f.tags.empty()) {
        const char* why = nullptr;
        if (eng::readCamera(c.viewer, cam, &why)) {
            ++camOk;
            camWhy = "ok";
        } else {
            ++camFail;
            camWhy = why ? why : "unknown";
        }
        if (camWhy != camWhyLogged) {
            camWhyLogged = camWhy;
            nvcoop::Log("markers: hud: scene camera: %s", camWhy);
        }
        s.cam = cam;
    }
    out = nvmk::paintFrame(f, layout, s);
    ++painted;
    tags += static_cast<uint64_t>(out.tags);
    compass += static_cast<uint64_t>(out.compass);
    mapPlaced += static_cast<uint64_t>(out.mapPlaced);
}

bool OverlayHud::Impl::provide(void* self, float w, float h, nvmk::PaintList& out) {
    Impl& d = *static_cast<Impl*>(self);
    nvmk::HudFrame f;
    nvmk::MapContext c;
    EnterCriticalSection(&d.cs);
    ++d.frames;
    const bool fresh = d.frameAt != 0 && NowMs() - d.frameAt <= kFrameFreshMs;
    if (fresh) {
        f = d.frame;
        c = d.ctx;
    }
    LeaveCriticalSection(&d.cs);
    if (!fresh) return false;
    d.renderSide(w, h, f, c, out);
    return !out.empty();
}

OverlayHud::OverlayHud() : d_(new Impl()) {}
OverlayHud::~OverlayHud() {
    overlay::setEnabled(false);
    hide();
    // the Present hook stays in the device's vtable for the life of the process and its provider pointer is cleared by the disabled flag; the Impl
    // lives on so that a late Present call never reads freed memory.
    d_ = nullptr;
}

bool OverlayHud::init(const HudLayout& layout) {
    d_->layout = layout;
    std::string err;
    d_->installed = overlay::install(&Impl::provide, d_, err);
    d_->installErr = err;
    if (d_->installed) overlay::setEnabled(true);
    else nvcoop::Log("markers: hud: no overlay: %s", err.c_str());
    return d_->installed;
}

bool OverlayHud::ready() const { return d_ && d_->installed && overlay::installed(); }

void OverlayHud::setMapContext(const MapContext& c) {
    if (!d_) return;
    EnterCriticalSection(&d_->cs);
    d_->ctx = c;
    LeaveCriticalSection(&d_->cs);
}

void OverlayHud::show(const nvmk::HudFrame& f) {
    if (!d_) return;
    EnterCriticalSection(&d_->cs);
    d_->frame = f;
    d_->frameAt = NowMs();
    LeaveCriticalSection(&d_->cs);
}

void OverlayHud::hide() {
    if (!d_) return;
    EnterCriticalSection(&d_->cs);
    d_->frame = nvmk::HudFrame{};
    d_->frameAt = 0;
    LeaveCriticalSection(&d_->cs);
}

std::string OverlayHud::statusJson() const {
    if (!d_) return "null";
    EnterCriticalSection(&d_->cs);
    char b[704];
    std::snprintf(b, sizeof b,
        "{\"installed\":%s,\"error\":\"%s\",\"camera\":\"%s\",\"cameraOk\":%llu,\"cameraFail\":%llu,\"presents\":%llu,\"painted\":%llu,\"tags\":%llu,\"compass\":%llu,"
        "\"mapPlaced\":%llu,\"root\":\"%.0fx%.0f\",\"compassTile\":\"%s\",\"worldTile\":\"%s\",\"localTile\":\"%s\",\"overlay\":",
        d_->installed ? "true" : "false", d_->installErr.c_str(), d_->camWhy, (unsigned long long)d_->camOk, (unsigned long long)d_->camFail, (unsigned long long)d_->frames,
        (unsigned long long)d_->painted, (unsigned long long)d_->tags, (unsigned long long)d_->compass, (unsigned long long)d_->mapPlaced, d_->root.w, d_->root.h,
        d_->hudTiles.compassFound ? d_->hudTiles.compassName : "", d_->mapTiles.worldFound ? d_->mapTiles.worldName : "", d_->mapTiles.localFound ? d_->mapTiles.localName : "");
    std::string out = b;
    LeaveCriticalSection(&d_->cs);
    out += overlay::statusJson() + "}";
    return out;
}

}  // namespace nvcoop::markers
