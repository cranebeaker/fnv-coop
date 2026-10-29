// See mk_hud.h. UNPROVEN IN GAME: compiles against the xNVSE headers; the script text, the tile XML, the tile names and the placement numbers are not measured.
#include <Windows.h>

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <map>
#include <string>
#include <vector>

#include "GameAPI.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "GameScript.h"
#include "GameUI.h"
#include "PluginAPI.h"
#include "common/nvcoop_api.h"
#include "mk_hud.h"

namespace nvcoop::markers {

using nvmk::HudFrame;
using nvmk::Slot;

namespace {

constexpr int kTagSlots = 4;       // the party has at most four players: three others, one spare
constexpr int kCompassSlots = 4;
constexpr int kMapSlots = 4;

// ---------------------------------------------------------------------------------------------------------------------------------- tile reads
// The tile tree of an open menu, read with the structs of xNVSE GameTiles.h / GameUI.h: Tile::childList (ChildNode next @0, child @8), Tile::values
// (BSSimpleArray of Value {id, parent, num, str, action}), Tile::name. The TileMenu* of a menu is g_tileMenuArray[id - 0x3E9] (xNVSE GameUI.cpp
// InterfaceManager::GetMenuByType; plugin/bots/bot_engine.h pins the same data pointer). A tile's place on the screen is its x / y plus those of its
// ancestors that have a non-zero `locus`, the sum xNVSE's GetUIFloatInherited makes. Everything here is read-only and SEH-guarded.
constexpr unsigned kTileMenuArrayData = 0x11F350C;
constexpr unsigned kMenuIdMin = 0x3E9;
constexpr unsigned kMenuIdHud = 0x3EC;      // xNVSE GameUI.h kMenuType_HUDMain
constexpr unsigned kMenuIdMap = 0x3FF;      // kMenuType_Map
constexpr int kMaxDumpLines = 100;

struct TileQuery {
    const char* exact[3] = {nullptr, nullptr, nullptr};   // tile names (case-insensitive)
    const char* has1 = nullptr;                           // else: a name that contains has1 (and has2)
    const char* has2 = nullptr;
};
struct TileHit {
    bool found = false;
    char name[40] = {};
    float ax = 0, ay = 0, w = 0, h = 0;
    bool visible = true;
    bool hasZoom = false;
    float zoom = 0;
    bool hasView = false;
    float vx = 0, vy = 0, vw = 0, vh = 0;
};
struct TileLine {
    char name[40];
    int depth;
    float ax, ay, w, h;
    bool visible;
};
struct TileScan {
    const TileQuery* q = nullptr;
    TileHit best;
    TileLine* lines = nullptr;
    int nLines = 0;
    int visited = 0;
};

bool TileNum(Tile* t, UInt32 id, float& out) {
    Tile::Value** d = t->values.data;
    const UInt32 n = t->values.size;
    if (!d || n > 1024) return false;
    for (UInt32 i = 0; i < n; ++i) {
        Tile::Value* v = d[i];
        if (v && v->id == id) { out = v->num; return true; }
    }
    return false;
}

char Low(char c) { return (c >= 'A' && c <= 'Z') ? static_cast<char>(c - 'A' + 'a') : c; }
bool SameName(const char* a, const char* b) {
    for (; *a && *b; ++a, ++b)
        if (Low(*a) != Low(*b)) return false;
    return *a == 0 && *b == 0;
}
bool HasName(const char* hay, const char* needle) {
    if (!needle || !*needle) return true;
    for (; *hay; ++hay) {
        const char* h = hay;
        const char* n = needle;
        while (*h && *n && Low(*h) == Low(*n)) { ++h; ++n; }
        if (!*n) return true;
    }
    return false;
}

void WalkTile(Tile* t, int depth, float baseX, float baseY, bool visAbove, bool hasView, float vx, float vy, float vw, float vh, TileScan& s) {
    if (!t || depth > 12 || s.visited >= 1500) return;
    ++s.visited;
    float x = 0, y = 0, w = 0, h = 0, vis = 1, locus = 0, zoom = 0, clips = 0, clipw = 0;
    TileNum(t, Tile::kTileValue_x, x);
    TileNum(t, Tile::kTileValue_y, y);
    TileNum(t, Tile::kTileValue_width, w);
    TileNum(t, Tile::kTileValue_height, h);
    const bool hasVis = TileNum(t, Tile::kTileValue_visible, vis);
    TileNum(t, Tile::kTileValue_locus, locus);
    const bool hasZoom = TileNum(t, Tile::kTileValue_zoom, zoom);
    TileNum(t, Tile::kTileValue_clips, clips);
    TileNum(t, Tile::kTileValue_clipwindow, clipw);
    char name[40] = {};
    if (const char* nm = t->name.m_data)
        for (int i = 0; i < 39 && nm[i]; ++i) name[i] = nm[i];
    const float ax = baseX + x, ay = baseY + y;
    const bool shown = visAbove && (!hasVis || vis != 0.0f);
    if (s.lines && s.nLines < kMaxDumpLines && w >= 60.0f && h >= 40.0f) {
        TileLine& l = s.lines[s.nLines++];
        std::memcpy(l.name, name, sizeof name);
        l.depth = depth; l.ax = ax; l.ay = ay; l.w = w; l.h = h; l.visible = shown;
    }
    if (s.q && name[0]) {
        bool match = false;
        for (const char* e : s.q->exact)
            if (e && *e && SameName(name, e)) match = true;
        if (!match && s.q->has1 && HasName(name, s.q->has1) && HasName(name, s.q->has2)) match = true;
        if (match && w > 0.0f && h > 0.0f && (!s.best.found || w * h > s.best.w * s.best.h)) {
            TileHit& b = s.best;
            b.found = true;
            std::memcpy(b.name, name, sizeof name);
            b.ax = ax; b.ay = ay; b.w = w; b.h = h;
            b.visible = shown;
            b.hasZoom = hasZoom; b.zoom = zoom;
            b.hasView = hasView; b.vx = vx; b.vy = vy; b.vw = vw; b.vh = vh;
        }
    }
    const float nbx = baseX + (locus != 0.0f ? x : 0.0f), nby = baseY + (locus != 0.0f ? y : 0.0f);
    const bool clipping = (clips != 0.0f || clipw != 0.0f) && w > 0.0f && h > 0.0f;
    int n = 0;
    for (Tile::ChildNode* c = t->childList.m_listHead.data; c && n < 256; c = c->next, ++n)
        WalkTile(c->child, depth + 1, nbx, nby, shown, clipping || hasView, clipping ? ax : vx, clipping ? ay : vy, clipping ? w : vw, clipping ? h : vh, s);
}

bool ScanMenu(unsigned menuId, TileScan& s) {
    __try {
        const unsigned arr = *reinterpret_cast<unsigned*>(kTileMenuArrayData);
        if (!arr) return false;
        Tile* root = reinterpret_cast<TileMenu**>(arr)[menuId - kMenuIdMin];
        if (!root) return false;
        WalkTile(root, 0, 0, 0, true, false, 0, 0, 0, 0, s);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

nvmk::TileProbe ProbeOf(const TileHit& h) {
    nvmk::TileProbe p;
    if (!h.found) return p;
    p.r.valid = true;
    p.r.x = h.ax; p.r.y = h.ay; p.r.w = h.w; p.r.h = h.h;
    p.visible = h.visible;
    p.hasZoom = h.hasZoom;
    p.zoom = h.zoom;
    if (h.hasView && h.vw > 0.0f && h.vh > 0.0f) {
        p.viewport.valid = true;
        p.viewport.x = h.vx; p.viewport.y = h.vy; p.viewport.w = h.vw; p.viewport.h = h.vh;
    }
    return p;
}

constexpr const char* kScreenProbe = "NVCoopScreen";   // an injected rect whose width / height are the screen's own
constexpr float kGlyphW = 36.0f, kGlyphH = 20.0f;       // a compass glyph ("[A]"), in the HUD's units: the font is a guess
constexpr float kMarkLift = 28.0f;                      // map markers: the name line sits this far above the "<>" line that is on the point

uint32_t FloatBits(float f) { return *reinterpret_cast<uint32_t*>(&f); }

// One text tile and the compiled script that changes it. `show`: 0 hidden, 1 shown, 2 shown with new text.
struct Tile {
    std::string name;       // tile name, below its parent
    std::string parent;     // "HUDMainMenu" or "MapMenu"
    Script* set = nullptr;
    bool injected = false;
    std::string text;
    float x = -1, y = -1, a = -1;
    bool shown = false;
};

}  // namespace

struct ScriptHud::Impl {
    NVSEScriptInterface* scripts = nullptr;
    NVSEArrayVarInterface* arrays = nullptr;
    HudLayout layout;
    Script* inject[2] = {};          // [0] HUDMainMenu, [1] MapMenu: InjectUIComponent <parent> "%z" xml
    Script* onScreen = nullptr;      // Johnny Guitar's WorldToScreen of a point over a reference
    bool ready = false;
    bool haveHud = false;            // the tile scripts compiled (JIP LN)
    bool haveWorldToScreen = false;  // Johnny Guitar
    std::function<TESObjectREFR*(Slot)> refOf;
    MapContext map;
    Tile tags[kTagSlots], comp[kCompassSlots], bar, banner, mapList, mapMarks[kMapSlots];
    bool hudInjected = false, mapInjected = false;
    uint32_t lastMenuCheck = 0;
    uint64_t scriptCalls = 0, scriptFails = 0, frames = 0;
    std::string lastError;
    // what the tile reads said (refreshed every 250 ms)
    uint32_t lastCompassRead = 0, lastMapRead = 0;
    nvmk::Rect hudScreen, mapScreen;
    nvmk::CompassLayout compassL;
    std::string compassNote, mapNote;      // the last thing logged about each, so a change is logged once
    nvmk::MapPick mapPick;
    bool wantDump = true, dumped = false;   // the map menu's tiles are logged once, the first time it is read

    PlayerCharacter* player() { return g_thePlayer ? *g_thePlayer : nullptr; }

    Script* compile(const char* what, const std::string& text) {
        if (!scripts || !scripts->CompileScript) return nullptr;
        Script* s = scripts->CompileScript(text.c_str());
        if (!s) {
            lastError = std::string("could not compile ") + what;
            nvcoop::Log("markers: hud: %s", lastError.c_str());
        }
        return s;
    }

    static std::string setterText(const std::string& path) {
        return "string_var sText\n"
               "float fX\n"
               "float fY\n"
               "float fA\n"
               "int iShow\n"
               "Begin Function {sText, fX, fY, fA, iShow}\n"
               "  if iShow == 2\n"
               "    SetUIStringEx \"" + path + "/string\" \"%z\" sText\n"
               "  endif\n"
               "  if iShow\n"
               "    SetUIFloat \"" + path + "/user0\" fX\n"
               "    SetUIFloat \"" + path + "/user1\" fY\n"
               "    SetUIFloat \"" + path + "/alpha\" fA\n"
               "  endif\n"
               "  SetUIFloat \"" + path + "/visible\" iShow\n"
               "End\n";
    }

    bool initTile(Tile& t, const std::string& parent, const std::string& name) {
        t.parent = parent;
        t.name = name;
        t.set = compile(name.c_str(), setterText(parent + "/" + name));
        return t.set != nullptr;
    }

    // Port of the other mod's chat hud / friend marker scripts: the XML goes in through a string variable because the tile XML has quotes.
    bool compileAll() {
        inject[0] = compile("inject hud", "string_var sXml\nBegin Function {sXml}\n  InjectUIComponent \"HUDMainMenu\" \"%z\" sXml\nEnd\n");
        inject[1] = compile("inject map", "string_var sXml\nBegin Function {sXml}\n  InjectUIComponent \"MapMenu\" \"%z\" sXml\nEnd\n");
        bool ok = inject[0] != nullptr;
        // where a point 150 units above a reference is on the screen: newer Johnny Guitars take the variables, older ones their names
        for (const char* call : {"WorldToScreen fX fY fZ 0 0 150 2 rWho", "JGLegacyWorldToScreen \"fX\" \"fY\" \"fZ\" 0 0 150 2 rWho", "WorldToScreen \"fX\" \"fY\" \"fZ\" 0 0 150 2 rWho"}) {
            if (onScreen) break;
            onScreen = scripts->CompileScript((std::string(
                "ref rWho\nfloat fX\nfloat fY\nfloat fZ\nint iOn\narray_var aOut\n"
                "Begin Function {rWho}\n  set iOn to ") + call + "\n  let aOut := ar_List iOn fX fY\n  SetFunctionValue aOut\nEnd\n").c_str());
            if (onScreen) nvcoop::Log("markers: hud: places on screen with Johnny Guitar's \"%s\"", call);
        }
        haveWorldToScreen = onScreen != nullptr;
        if (!haveWorldToScreen) nvcoop::Log("markers: hud: no WorldToScreen (Johnny Guitar NVSE): no name tags over heads");
        for (int i = 0; i < kTagSlots; ++i) ok = initTile(tags[i], "HUDMainMenu", "NVCoopTag" + std::to_string(i)) && ok;
        for (int i = 0; i < kCompassSlots; ++i) ok = initTile(comp[i], "HUDMainMenu", "NVCoopComp" + std::to_string(i)) && ok;
        ok = initTile(bar, "HUDMainMenu", "NVCoopCompBar") && ok;
        ok = initTile(banner, "HUDMainMenu", "NVCoopBanner") && ok;
        // the Pip-Boy's tiles are optional: their scripts failing costs the map list only
        initTile(mapList, "MapMenu", "NVCoopMapList");
        for (int i = 0; i < kMapSlots; ++i) initTile(mapMarks[i], "MapMenu", "NVCoopMapMark" + std::to_string(i));
        return ok;
    }

    void injectXml(int which, const std::string& xml) {
        if (!inject[which] || !player()) return;
        ++scriptCalls;
        NVSEArrayVarInterface::Element result;
        if (!scripts->CallFunction(inject[which], player(), nullptr, &result, 1, xml.c_str())) ++scriptFails;
    }

    // text tile XML. x: either a fraction of the screen width taken from user0, or a constant; y likewise.
    static std::string xmlFor(const std::string& name, int kind, const HudLayout& l) {
        char b[900];
        switch (kind) {
            case 0:   // name tag: placed from user0 / user1, lifted by tagLift pixels
                std::snprintf(b, sizeof b,
                    "<text name=\"%s\"><visible>0</visible><user0>0.5</user0><user1>0.5</user1><alpha>255</alpha>"
                    "<x><copy src=\"screen()\" trait=\"width\"/><mul src=\"me()\" trait=\"user0\"/></x>"
                    "<y><copy src=\"screen()\" trait=\"height\"/><mul src=\"me()\" trait=\"user1\"/><sub>%d</sub></y>"
                    "<font>2</font><justify>&center;</justify><wrapwidth>400</wrapwidth><systemcolor>1</systemcolor><string> </string></text>",
                    name.c_str(), l.tagLift);
                break;
            case 1:   // compass glyph: x from user0 (a fraction of the screen width), y from user1 (the HUD's units)
                std::snprintf(b, sizeof b,
                    "<text name=\"%s\"><visible>0</visible><user0>0.5</user0><user1>0</user1><alpha>255</alpha>"
                    "<x><copy src=\"screen()\" trait=\"width\"/><mul src=\"me()\" trait=\"user0\"/></x><y><copy src=\"me()\" trait=\"user1\"/></y>"
                    "<font>2</font><justify>&center;</justify><wrapwidth>120</wrapwidth><systemcolor>1</systemcolor><string> </string></text>",
                    name.c_str());
                break;
            case 2:   // the strip itself and the banner: centred, constant place
                std::snprintf(b, sizeof b,
                    "<text name=\"%s\"><visible>0</visible><user0>0</user0><user1>0</user1><alpha>255</alpha>"
                    "<x><copy src=\"screen()\" trait=\"width\"/><div>2</div></x><y>%d</y>"
                    "<font>1</font><justify>&center;</justify><wrapwidth>%d</wrapwidth><systemcolor>1</systemcolor><string> </string></text>",
                    name.c_str(), name == "NVCoopCompBar" ? static_cast<int>(l.compass.y) + 16 : l.bannerY, name == "NVCoopCompBar" ? 700 : 1000);
                break;
            case 4:   // Pip-Boy marker: centred on the point (user0 / user1 as fractions of the screen)
                std::snprintf(b, sizeof b,
                    "<text name=\"%s\"><visible>0</visible><user0>0.1</user0><user1>0.1</user1><alpha>255</alpha>"
                    "<x><copy src=\"screen()\" trait=\"width\"/><mul src=\"me()\" trait=\"user0\"/></x>"
                    "<y><copy src=\"screen()\" trait=\"height\"/><mul src=\"me()\" trait=\"user1\"/></y>"
                    "<font>2</font><justify>&center;</justify><wrapwidth>260</wrapwidth><systemcolor>1</systemcolor><string> </string></text>",
                    name.c_str());
                break;
            case 5:   // an empty rect that reports the screen's size in the HUD's own units (the tile reads need it)
                std::snprintf(b, sizeof b,
                    "<rect name=\"%s\"><x>0</x><y>0</y><width><copy src=\"screen()\" trait=\"width\"/></width>"
                    "<height><copy src=\"screen()\" trait=\"height\"/></height></rect>",
                    name.c_str());
                break;
            default:  // Pip-Boy list: from user0 / user1 as fractions of the screen
                std::snprintf(b, sizeof b,
                    "<text name=\"%s\"><visible>0</visible><user0>0.1</user0><user1>0.1</user1><alpha>255</alpha>"
                    "<x><copy src=\"screen()\" trait=\"width\"/><mul src=\"me()\" trait=\"user0\"/></x>"
                    "<y><copy src=\"screen()\" trait=\"height\"/><mul src=\"me()\" trait=\"user1\"/></y>"
                    "<font>2</font><justify>&left;</justify><wrapwidth>%d</wrapwidth><systemcolor>1</systemcolor><string> </string></text>",
                    name.c_str(), 700);
                break;
        }
        return b;
    }

    void ensureHudTiles() {
        if (hudInjected || !player()) return;
        for (int i = 0; i < kTagSlots; ++i) injectXml(0, xmlFor(tags[i].name, 0, layout));
        for (int i = 0; i < kCompassSlots; ++i) injectXml(0, xmlFor(comp[i].name, 1, layout));
        injectXml(0, xmlFor(bar.name, 2, layout));
        injectXml(0, xmlFor(banner.name, 2, layout));
        injectXml(0, xmlFor(kScreenProbe, 5, layout));
        hudInjected = true;
    }
    void ensureMapTiles() {
        if (mapInjected || !player()) return;
        injectXml(1, xmlFor(mapList.name, 3, layout));
        for (int i = 0; i < kMapSlots; ++i) injectXml(1, xmlFor(mapMarks[i].name, 4, layout));
        injectXml(1, xmlFor(kScreenProbe, 5, layout));
        mapInjected = true;
        lastMapRead = 0;
    }

    // Change a tile only when something moved enough to see.
    void setTile(Tile& t, const std::string& text, float x, float y, float alpha, bool show) {
        if (!t.set || !player()) return;
        if (!show && !t.shown) return;
        const std::string use = text.empty() ? std::string(" ") : text;
        const bool newText = !t.shown || use != t.text;
        const int mode = !show ? 0 : (newText ? 2 : 1);
        if (show && !newText && std::fabs(x - t.x) < 0.0015f && std::fabs(y - t.y) < 0.0015f && std::fabs(alpha - t.a) < 6.0f) return;
        ++scriptCalls;
        if (!scripts->CallFunction(t.set, player(), nullptr, nullptr, 5, use.c_str(), FloatBits(x), FloatBits(y), FloatBits(alpha), static_cast<uint32_t>(mode))) ++scriptFails;
        if (show) { t.text = use; t.x = x; t.y = y; t.a = alpha; }
        t.shown = show;
    }

    bool projected(TESObjectREFR* who, float& x, float& y) {
        if (!onScreen || !arrays || !who || !player()) return false;
        NVSEArrayVarInterface::Element result;
        ++scriptCalls;
        if (!scripts->CallFunction(onScreen, who, nullptr, &result, 1, who)) { ++scriptFails; return false; }
        if (!result.GetArray() || arrays->GetArraySize(result.GetArray()) != 3) return false;
        NVSEArrayVarInterface::Element e[3];
        if (!arrays->GetElements(result.GetArray(), e, nullptr) || e[0].GetNumber() == 0) return false;
        x = static_cast<float>(e[1].GetNumber());
        y = static_cast<float>(e[2].GetNumber());
        return x >= 0 && x <= 1 && y >= 0 && y <= 1;
    }

    static std::string bar10(float p) {
        const int n = static_cast<int>((std::max)(0.0f, (std::min)(1.0f, p)) * 10.0f + 0.5f);
        return "[" + std::string(static_cast<size_t>(n), '#') + std::string(static_cast<size_t>(10 - n), '-') + "]";
    }

    void drawHud(const HudFrame& f) {
        ensureHudTiles();
        // name tags
        int used = 0;
        for (const nvmk::HudTag& t : f.tags) {
            if (used >= kTagSlots) break;
            float x = 0, y = 0;
            TESObjectREFR* who = refOf ? refOf(t.slot) : nullptr;
            if (!who || !projected(who, x, y)) continue;
            setTile(tags[used], t.text, x, y, 255.0f * t.alpha, true);
            ++used;
        }
        for (int i = used; i < kTagSlots; ++i) setTile(tags[i], "", 0, 0, 0, false);
        // the compass: over the game's own compass tile, else the strip of our own
        refreshCompass();
        int c = 0;
        if (!compassL.hidden) {
            for (const nvmk::HudCompassMark& m : f.compass) {
                if (c >= kCompassSlots) break;
                setTile(comp[c], m.glyph, nvmk::compassScreenX(compassL, m.x, hudScreen), compassL.y, m.downed ? 200.0f : 255.0f, true);
                ++c;
            }
        }
        for (int i = c; i < kCompassSlots; ++i) setTile(comp[i], "", 0, 0, 0, false);
        setTile(bar, "- - - - - - - - - - - - - - - - - - - -", 0, compassL.y + 16.0f, 110.0f, c > 0 && !compassL.fromTile);
        // the banner
        std::string b = f.banner;
        if (!b.empty() && f.bannerProgress >= 0) b += "  " + bar10(f.bannerProgress);
        setTile(banner, b, 0, static_cast<float>(layout.bannerY), 255.0f, !b.empty());
    }

    // The compass tile and the screen size, read every 250 ms (the tile does not move; a mod's HUD may place it anywhere).
    void refreshCompass() {
        const uint32_t now = GetTickCount();
        if (lastCompassRead && now - lastCompassRead < 250) return;
        lastCompassRead = now;
        nvmk::TileProbe tile;
        if (map.engineOk) {
            TileQuery sq;
            sq.exact[0] = kScreenProbe;
            TileScan ss;
            ss.q = &sq;
            if (ScanMenu(kMenuIdHud, ss) && ss.best.found) {
                hudScreen.valid = true;
                hudScreen.x = hudScreen.y = 0;
                hudScreen.w = ss.best.w;
                hudScreen.h = ss.best.h;
            }
            TileQuery cq;
            cq.exact[0] = layout.compassTile.c_str();
            cq.exact[1] = "CompassWindow";
            TileScan cs;
            cs.q = &cq;
            if (ScanMenu(kMenuIdHud, cs)) tile = ProbeOf(cs.best);
        }
        compassL = nvmk::compassLayout(tile, hudScreen, layout.compass, kGlyphW, kGlyphH);
        char note[240];
        if (tile.r.valid)
            std::snprintf(note, sizeof note, "compass tile at %.0f,%.0f size %.0fx%.0f on a %.0fx%.0f screen: %s, centre %.3f width %.3f y %.0f%s", tile.r.x, tile.r.y, tile.r.w, tile.r.h,
                hudScreen.w, hudScreen.h, compassL.fromTile ? "used" : "not plausible, own strip", compassL.centerX, compassL.width, compassL.y, compassL.hidden ? " (hidden)" : "");
        else
            std::snprintf(note, sizeof note, "compass tile \"%s\" not found%s: own strip at centre %.3f width %.3f y %.0f", layout.compassTile.c_str(), map.engineOk ? "" : " (engine pins do not match)", compassL.centerX, compassL.width, compassL.y);
        if (compassNote != note) {
            compassNote = note;
            nvcoop::Log("markers: hud: %s", note);
        }
    }

    // The Pip-Boy's map tiles (auto mode), read every 250 ms while the map is open.
    void refreshMap() {
        const uint32_t now = GetTickCount();
        if (lastMapRead && now - lastMapRead < 250) return;
        lastMapRead = now;
        mapPick = nvmk::MapPick{};
        if (!map.engineOk) { mapPick.why = "the engine pins do not match"; return; }
        TileQuery sq;
        sq.exact[0] = kScreenProbe;
        TileScan ss;
        ss.q = &sq;
        mapScreen = nvmk::Rect{};
        if (ScanMenu(kMenuIdMap, ss) && ss.best.found) {
            mapScreen.valid = true;
            mapScreen.w = ss.best.w;
            mapScreen.h = ss.best.h;
        }
        TileQuery lq, wq;
        if (!layout.mapLocalTile.empty()) lq.exact[0] = layout.mapLocalTile.c_str(); else { lq.has1 = "local"; lq.has2 = "map"; }
        if (!layout.mapWorldTile.empty()) wq.exact[0] = layout.mapWorldTile.c_str(); else { wq.has1 = "world"; wq.has2 = "map"; }
        TileScan ls, ws;
        ls.q = &lq;
        ws.q = &wq;
        const bool okL = ScanMenu(kMenuIdMap, ls), okW = ScanMenu(kMenuIdMap, ws);
        if (wantDump && !dumped) {
            static TileLine dump[kMaxDumpLines];
            TileScan ds;
            ds.lines = dump;
            if (ScanMenu(kMenuIdMap, ds)) {
                dumped = true;
                nvcoop::Log("markers: hud: MapMenu tiles of 60x40 or more (name, depth, screen x y, size, shown):");
                for (int i = 0; i < ds.nLines; ++i)
                    nvcoop::Log("markers: hud:   %s d%d %.0f,%.0f %.0fx%.0f %s", dump[i].name, dump[i].depth, dump[i].ax, dump[i].ay, dump[i].w, dump[i].h, dump[i].visible ? "shown" : "hidden");
            }
        }
        if (!okL && !okW) mapPick.why = "the map menu's tiles cannot be read";
        else mapPick = nvmk::chooseMap(ProbeOf(ls.best), ProbeOf(ws.best), mapScreen);
        char note[260];
        std::snprintf(note, sizeof note, "map: %s%s (local tile \"%s\", world tile \"%s\", screen %.0fx%.0f)",
            mapPick.kind == nvmk::MapKind::World ? "world map" : mapPick.kind == nvmk::MapKind::Local ? "local map" : "no markers, the list: ",
            mapPick.kind == nvmk::MapKind::None ? mapPick.why : "", ls.best.found ? ls.best.name : "-", ws.best.found ? ws.best.name : "-", mapScreen.w, mapScreen.h);
        if (mapNote != note) {
            mapNote = note;
            nvcoop::Log("markers: hud: %s", note);
        }
    }

    // "Alex, 340 m - ..." -> two lines: the name over the "<>" that sits on the point
    static std::string markText(const std::string& label) {
        std::string tag = label;
        const size_t comma = tag.find(',');
        if (comma != std::string::npos) tag = tag.substr(0, comma);
        return tag + "\n<>";
    }

    void drawMap(const HudFrame& f) {
        if (!map.pipboyMapOpen) { mapInjected = false; return; }   // the menu's tiles are gone when it closes: inject again at the next opening
        ensureMapTiles();
        const bool manual = layout.mapMode == "local" || layout.mapMode == "world";
        const bool autoMode = layout.mapMode == "auto";
        if (autoMode) refreshMap();
        int m = 0;
        if (autoMode && mapPick.kind != nvmk::MapKind::None) {
            for (const nvmk::HudMapMark& k : f.map) {
                if (m >= kMapSlots) break;
                float u = 0, v = 0, sx = 0, sy = 0;
                const bool isWorld = mapPick.kind == nvmk::MapKind::World;
                const bool ok = isWorld ? (k.world && nvmk::worldMapUV(k.at, map.bounds, u, v))
                                        : (k.local && nvmk::localMapUV(map.viewer, k.at, layout.localMapHalfRangeUnits, u, v));
                if (!ok || !nvmk::mapScreenPoint(mapPick.tile, u, v, sx, sy)) continue;
                setTile(mapMarks[m], markText(k.label), sx / mapScreen.w, (sy - kMarkLift) / mapScreen.h, 255.0f, true);
                ++m;
            }
        } else if (manual) {
            for (const nvmk::HudMapMark& k : f.map) {
                if (m >= kMapSlots) break;
                float u = 0, v = 0;
                const bool ok = layout.mapMode == "local" ? (k.local && nvmk::localMapUV(map.viewer, k.at, layout.localMapHalfRangeUnits, u, v))
                                                          : (k.world && nvmk::worldMapUV(k.at, map.bounds, u, v));
                if (!ok) continue;
                setTile(mapMarks[m], markText(k.label), layout.mapX + layout.mapW * u, layout.mapY + layout.mapH * v - 0.03f, 255.0f, true);
                ++m;
            }
        }
        for (int i = m; i < kMapSlots; ++i) setTile(mapMarks[i], "", 0, 0, 0, false);
        // the text list: the fallback, and the only place a friend appears who is off the map's frame or in another place
        const bool markersActive = autoMode && mapPick.kind != nvmk::MapKind::None;
        if (manual || nvmk::mapListNeeded(markersActive, m, static_cast<int>(f.list.size()))) {
            std::string list;
            for (const std::string& line : f.list) list += line + "\n";
            setTile(mapList, list.empty() ? std::string("No friends on the map") : list, 0.04f, 0.10f, 255.0f, true);
        } else {
            setTile(mapList, "", 0, 0, 0, false);
        }
    }
};

ScriptHud::ScriptHud() : d_(new Impl) {}
ScriptHud::~ScriptHud() { delete d_; }

bool ScriptHud::init(NVSEInterface* nvse, const HudLayout& layout) {
    d_->layout = layout;
    if (!nvse) return false;
    d_->scripts = static_cast<NVSEScriptInterface*>(nvse->QueryInterface(kInterface_Script));
    d_->arrays = static_cast<NVSEArrayVarInterface*>(nvse->QueryInterface(kInterface_ArrayVar));
    if (!d_->scripts || !d_->scripts->CompileScript || !d_->scripts->CallFunction) {
        nvcoop::Log("markers: hud: this xNVSE has no script interface: notification fallback only");
        return false;
    }
    d_->haveHud = d_->compileAll();
    d_->ready = d_->haveHud;
    nvcoop::Log("markers: hud: tiles %s, name tags %s", d_->haveHud ? "ready" : "NOT available (JIP LN NVSE missing?)", d_->haveWorldToScreen ? "ready" : "off");
    return d_->ready;
}

bool ScriptHud::ready() const { return d_->ready; }
void ScriptHud::setRefLookup(std::function<TESObjectREFR*(Slot)> f) { d_->refOf = std::move(f); }
void ScriptHud::setMapContext(const MapContext& c) { d_->map = c; }

void ScriptHud::show(const HudFrame& f) {
    ++d_->frames;
    if (d_->ready) {
        d_->drawHud(f);
        d_->drawMap(f);
        return;
    }
    // no tile scripts: the glue prints the banner on the notification line instead (ready() is false)
}

void ScriptHud::hide() { show(HudFrame{}); d_->mapInjected = false; }

std::string ScriptHud::statusJson() const {
    char b[420];
    std::snprintf(b, sizeof b, "{\"ready\":%s,\"tiles\":%s,\"nameTags\":%s,\"scriptCalls\":%llu,\"scriptFails\":%llu,\"frames\":%llu,\"mapMode\":\"%s\",\"compassFromTile\":%s,\"mapMarks\":\"%s\",\"error\":\"%s\"}",
        d_->ready ? "true" : "false", d_->haveHud ? "true" : "false", d_->haveWorldToScreen ? "true" : "false",
        (unsigned long long)d_->scriptCalls, (unsigned long long)d_->scriptFails, (unsigned long long)d_->frames, d_->layout.mapMode.c_str(),
        d_->compassL.fromTile ? "true" : "false", d_->mapPick.kind == nvmk::MapKind::World ? "world" : d_->mapPick.kind == nvmk::MapKind::Local ? "local" : "none", d_->lastError.c_str());
    return b;
}

}  // namespace nvcoop::markers
