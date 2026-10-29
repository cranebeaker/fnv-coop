// See mk_hud.h. UNPROVEN IN GAME: compiles against the xNVSE headers; the script text, the tile XML and the placement numbers are not measured.
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
            case 1:   // compass glyph: x from user0, y constant
                std::snprintf(b, sizeof b,
                    "<text name=\"%s\"><visible>0</visible><user0>0.5</user0><user1>0</user1><alpha>255</alpha>"
                    "<x><copy src=\"screen()\" trait=\"width\"/><mul src=\"me()\" trait=\"user0\"/></x><y>%d</y>"
                    "<font>2</font><justify>&center;</justify><wrapwidth>120</wrapwidth><systemcolor>1</systemcolor><string> </string></text>",
                    name.c_str(), l.compassY);
                break;
            case 2:   // the strip itself and the banner: centred, constant place
                std::snprintf(b, sizeof b,
                    "<text name=\"%s\"><visible>0</visible><user0>0</user0><user1>0</user1><alpha>255</alpha>"
                    "<x><copy src=\"screen()\" trait=\"width\"/><div>2</div></x><y>%d</y>"
                    "<font>1</font><justify>&center;</justify><wrapwidth>%d</wrapwidth><systemcolor>1</systemcolor><string> </string></text>",
                    name.c_str(), name == "NVCoopCompBar" ? l.compassY + 16 : l.bannerY, name == "NVCoopCompBar" ? 700 : 1000);
                break;
            default:  // Pip-Boy list / marks: from user0 / user1 as fractions of the screen
                std::snprintf(b, sizeof b,
                    "<text name=\"%s\"><visible>0</visible><user0>0.1</user0><user1>0.1</user1><alpha>255</alpha>"
                    "<x><copy src=\"screen()\" trait=\"width\"/><mul src=\"me()\" trait=\"user0\"/></x>"
                    "<y><copy src=\"screen()\" trait=\"height\"/><mul src=\"me()\" trait=\"user1\"/></y>"
                    "<font>2</font><justify>&left;</justify><wrapwidth>%d</wrapwidth><systemcolor>1</systemcolor><string> </string></text>",
                    name.c_str(), name == "NVCoopMapList" ? 700 : 260);
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
        hudInjected = true;
    }
    void ensureMapTiles() {
        if (mapInjected || !player()) return;
        injectXml(1, xmlFor(mapList.name, 3, layout));
        for (int i = 0; i < kMapSlots; ++i) injectXml(1, xmlFor(mapMarks[i].name, 3, layout));
        mapInjected = true;
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
        // the compass strip
        int c = 0;
        for (const nvmk::HudCompassMark& m : f.compass) {
            if (c >= kCompassSlots) break;
            const float x = layout.compassX + m.x * layout.compassW * 0.5f;
            setTile(comp[c], m.glyph, x, 0, m.downed ? 200.0f : 255.0f, true);
            ++c;
        }
        for (int i = c; i < kCompassSlots; ++i) setTile(comp[i], "", 0, 0, 0, false);
        setTile(bar, "- - - - - - - - - - - - - - - - - - - -", 0, 0, 110.0f, c > 0);
        // the banner
        std::string b = f.banner;
        if (!b.empty() && f.bannerProgress >= 0) b += "  " + bar10(f.bannerProgress);
        setTile(banner, b, 0, 0, 255.0f, !b.empty());
    }

    void drawMap(const HudFrame& f) {
        if (!map.pipboyMapOpen) { mapInjected = false; return; }   // the menu's tiles are gone when it closes: inject again at the next opening
        ensureMapTiles();
        std::string list;
        for (const std::string& line : f.list) list += line + "\n";
        setTile(mapList, list.empty() ? std::string("No friends on the map") : list, 0.04f, 0.10f, 255.0f, true);
        int m = 0;
        if (layout.mapMode == "local" || layout.mapMode == "world") {
            for (const nvmk::HudMapMark& k : f.map) {
                if (m >= kMapSlots) break;
                float u = 0, v = 0;
                const bool ok = layout.mapMode == "local" ? (k.local && nvmk::localMapUV(map.viewer, k.at, layout.localMapHalfRangeUnits, u, v))
                                                          : (k.world && nvmk::worldMapUV(k.at, map.bounds, u, v));
                if (!ok) continue;
                std::string tag = k.label;
                const size_t comma = tag.find(',');
                if (comma != std::string::npos) tag = tag.substr(0, comma);
                setTile(mapMarks[m], "<> " + tag, layout.mapX + layout.mapW * u, layout.mapY + layout.mapH * v, 255.0f, true);
                ++m;
            }
        }
        for (int i = m; i < kMapSlots; ++i) setTile(mapMarks[i], "", 0, 0, 0, false);
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
    char b[320];
    std::snprintf(b, sizeof b, "{\"ready\":%s,\"tiles\":%s,\"nameTags\":%s,\"scriptCalls\":%llu,\"scriptFails\":%llu,\"frames\":%llu,\"mapMode\":\"%s\",\"error\":\"%s\"}",
        d_->ready ? "true" : "false", d_->haveHud ? "true" : "false", d_->haveWorldToScreen ? "true" : "false",
        (unsigned long long)d_->scriptCalls, (unsigned long long)d_->scriptFails, (unsigned long long)d_->frames, d_->layout.mapMode.c_str(), d_->lastError.c_str());
    return b;
}

}  // namespace nvcoop::markers
