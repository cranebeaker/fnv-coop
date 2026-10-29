// nvcoop module "markers": the game half of plugin/markers (x86, compiled into nvcoop.dll next to the coop module). NOTHING here has run in a game.
//
//   [markers]                       friend markers (default on)
//   enabled=1                       0 = no tag, no compass strip, no map list (the party table still travels while [revive] is on)
//   tags=1 compass=1 map=1          the three surfaces
//   tag_full_m=20 tag_max_m=100     a name tag is full strength to 20 m and gone at 100 m
//   sector_far_m=170                from this far a friend's place and sector are named
//   stale_ms=4000                   a friend not heard of for this long is not shown
//   beacon_ms=500                   how often a guest tells the host where it is / the host sends the table
//   compass_tile=compass_window     the HUD tile the compass marks go over (its rectangle is read at run time)
//   compass_x=0.5 compass_w=0.30 compass_y=36   optional overrides of the compass tile's centre and width (fractions of the screen width) and of the
//                                   glyph line (pixels from the top); with no readable compass tile they are the strip of our own
//   compass_span_deg=180            how many degrees the compass shows (-half .. +half from the heading); beyond it a mark is pinned to the end
//   banner_y=520 tag_lift=75        pixel placement of the banner and of the tags over the point WorldToScreen gives
//   map_mode=auto                   auto = markers over the map tile's rectangle read at run time (text list when it cannot be read);
//                                   list = the text list only; local / world = markers over that map inside map_rect
//   map_world_tile= map_local_tile= exact tile names of the two maps for auto mode (else found by name; the log lists the map menu's tiles)
//   map_rect=0.2,0.18,0.6,0.62      x,y,w,h as fractions of the screen, for map_mode=local / world only
//   local_map_range_m=117           metres from the middle of the local map to its edge
//
//   [revive]                        revive of a downed friend (default on)
//   enabled=1                       0 = a downed player waits for the policy respawn only, as before
//   hold_ms=3000                    how long the activate key is held
//   reach=150 host_reach=600        game units: the friend's own check / the host's (more generous: lag and blasts)
//   health_pct=30                   the revived player gets up with this share of his health
//   cooldown_ms=3000                after a revive: the downed player's health takes a moment to come back
//
// Environment override for automated runs: NVCOOP_MARKERS=0|1, NVCOOP_REVIVE=0|1.
//
// UNPROVEN IN GAME. What it relies on and where each fact comes from is in docs/markers.md and docs/revive.md.
#include <Windows.h>

#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <map>
#include <memory>
#include <string>

#include "GameAPI.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "PluginAPI.h"
#include "avatar/engine/engine_pins.h"
#include "bots/bots_addrs.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "coop/coop_runtime.h"
#include "intents/engine/game_engine.h"
#include "mk_glue.h"
#include "mk_hud.h"
#include "mk_service.h"
#include "mk_wire.h"
#include "threat/threat_glue.h"   // the threat AI never picks a downed player (inline no-op without NVCOOP_WITH_THREAT)

#ifdef NVCOOP_WITH_LIVE
#include "coop/live/live_glue.h"
#endif
#ifdef NVCOOP_WITH_AVATAR_ENGINE
#include "avatar_glue.h"   // the downed pose of a friend's puppet (docs/revive.md "Downed pose"); without the avatar module nothing is hooked
#endif

namespace nvcoop::markers {
namespace {

using nvmk::Place;
using nvmk::Slot;

struct Cfg {
    nvmk::ServiceConfig svc;
    HudLayout layout;
};
Cfg g_cfg;
NVSEInterface* g_nvse = nullptr;
bool g_inited = false;

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    const size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

float IniFloat(const char* sec, const char* key, float dflt, const std::string& ini) {
    char b[32] = {};
    GetPrivateProfileStringA(sec, key, "", b, sizeof b, ini.c_str());
    if (!b[0]) return dflt;
    const float v = static_cast<float>(std::atof(b));
    return std::isfinite(v) ? v : dflt;
}

bool EnvOverride(const char* name, bool& value) {
    char b[8] = {};
    if (GetEnvironmentVariableA(name, b, sizeof b) == 0) return false;
    if (b[0] == '1') { value = true; return true; }
    if (b[0] == '0') { value = false; return true; }
    return false;
}

void LoadCfg() {
    const std::string ini = IniPath();
    auto flag = [&](const char* sec, const char* key, bool d) { return GetPrivateProfileIntA(sec, key, d ? 1 : 0, ini.c_str()) != 0; };
    auto num = [&](const char* sec, const char* key, int d) { return static_cast<int>(GetPrivateProfileIntA(sec, key, d, ini.c_str())); };
    nvmk::MarkerConfig& m = g_cfg.svc.markers;
    m.enabled = flag("markers", "enabled", true);
    EnvOverride("NVCOOP_MARKERS", m.enabled);
    m.tags = flag("markers", "tags", true);
    m.compass = flag("markers", "compass", true);
    m.map = flag("markers", "map", true);
    m.tagFullUnits = IniFloat("markers", "tag_full_m", 20.0f, ini) * nvmk::kUnitsPerMeter;
    m.tagMaxUnits = IniFloat("markers", "tag_max_m", 100.0f, ini) * nvmk::kUnitsPerMeter;
    m.sectorFarUnits = IniFloat("markers", "sector_far_m", 170.0f, ini) * nvmk::kUnitsPerMeter;
    m.staleMs = static_cast<uint32_t>(num("markers", "stale_ms", 4000));
    g_cfg.svc.beaconMs = g_cfg.svc.tableMs = static_cast<uint32_t>((std::max)(100, num("markers", "beacon_ms", 500)));
    HudLayout& l = g_cfg.layout;
    auto isSet = [&](const char* key) {
        char b[16] = {};
        GetPrivateProfileStringA("markers", key, "", b, sizeof b, ini.c_str());
        return b[0] != 0;
    };
    l.compass.hasX = isSet("compass_x");
    l.compass.hasW = isSet("compass_w");
    l.compass.hasY = isSet("compass_y");
    l.compass.x = IniFloat("markers", "compass_x", l.compass.x, ini);
    l.compass.w = IniFloat("markers", "compass_w", l.compass.w, ini);
    l.compass.y = static_cast<float>(num("markers", "compass_y", static_cast<int>(l.compass.y)));
    char tileName[64] = {};
    GetPrivateProfileStringA("markers", "compass_tile", "", tileName, sizeof tileName, ini.c_str());
    if (tileName[0]) l.compassTile = tileName;
    m.compassSpanRad = (std::max)(30.0f, (std::min)(360.0f, IniFloat("markers", "compass_span_deg", 180.0f, ini))) * nvmk::kPi / 180.0f;
    l.bannerY = num("markers", "banner_y", l.bannerY);
    l.tagLift = num("markers", "tag_lift", l.tagLift);
    char mode[16] = {};
    GetPrivateProfileStringA("markers", "map_mode", "auto", mode, sizeof mode, ini.c_str());
    const std::string mm = mode;
    l.mapMode = (mm == "local" || mm == "world" || mm == "list") ? mm : "auto";
    char wt[64] = {}, lt[64] = {};
    GetPrivateProfileStringA("markers", "map_world_tile", "", wt, sizeof wt, ini.c_str());
    GetPrivateProfileStringA("markers", "map_local_tile", "", lt, sizeof lt, ini.c_str());
    l.mapWorldTile = wt;
    l.mapLocalTile = lt;
    char rect[96] = {};
    GetPrivateProfileStringA("markers", "map_rect", "", rect, sizeof rect, ini.c_str());
    if (rect[0]) std::sscanf(rect, "%f,%f,%f,%f", &l.mapX, &l.mapY, &l.mapW, &l.mapH);
    l.localMapHalfRangeUnits = IniFloat("markers", "local_map_range_m", 117.0f, ini) * nvmk::kUnitsPerMeter;

    nvmk::ReviveRules& r = g_cfg.svc.revive;
    r.enabled = flag("revive", "enabled", true);
    EnvOverride("NVCOOP_REVIVE", r.enabled);
    r.holdMs = static_cast<uint32_t>((std::max)(500, num("revive", "hold_ms", 3000)));
    r.hostReachUnits = IniFloat("revive", "host_reach", 600.0f, ini);
    r.healthFraction = (std::max)(0.05f, (std::min)(1.0f, IniFloat("revive", "health_pct", 30.0f, ini) / 100.0f));
    g_cfg.svc.hold.holdMs = r.holdMs;
    g_cfg.svc.hold.reachUnits = IniFloat("revive", "reach", 150.0f, ini);
    g_cfg.svc.hold.cooldownMs = static_cast<uint32_t>(num("revive", "cooldown_ms", 3000));
}

// ---------------------------------------------------------------------------------------------------------------------------------- engine reads
// Facts used (all pinned elsewhere in the repo): OSInputGlobals** 0x11F35CC and OSInputGlobals::GetControlState 0xA24660 (bots_addrs.h), MenuMode 0x702360
// (avatar engine_pins.h), InterfaceManager** 0x11D8A80 and the top-menu function 0x714F00 (bots_addrs.h, as plugin/world_time uses them). Each call is
// made only after its prologue matched (or a rel32 jump stands there: another module of ours detours GetControlState).
constexpr unsigned kInterfaceMgrPtr = 0x11D8A80;
constexpr unsigned kMenuMap = 0x3FF;   // xNVSE GameUI.h kMenuType_Map: the Pip-Boy's MAP tab
constexpr int kCtrlActivate = 5;

bool PeekOk(unsigned addr, const unsigned char* want, size_t n, bool allowJump) {
    unsigned char got[16] = {};
    __try {
        std::memcpy(got, reinterpret_cast<const void*>(addr), n);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
    if (std::memcmp(got, want, n) == 0) return true;
    return allowJump && got[0] == 0xE9;
}

bool PinsOk() {
    static int state = -1;
    if (state >= 0) return state == 1;
    namespace b = nvbots::addr;
    const bool ok = PeekOk(b::GetControlState.target, b::GetControlState.expected, 16, true) &&
                    PeekOk(b::TopMenuId.addr, b::TopMenuId.bytes, 16, false) &&
                    PeekOk(nvcoop::avatar::pins::MenuModeAny.addr, nvcoop::avatar::pins::MenuModeAny.bytes, 16, false);
    state = ok ? 1 : 0;
    nvcoop::Log("markers: engine pins %s (GetControlState, TopMenuId, MenuMode)", ok ? "match" : "DO NOT match: no key reads, no menu checks");
    return ok;
}

bool ControlHeldSEH(int ctrl, bool& held) {
    __try {
        void* mgr = *reinterpret_cast<void**>(nvbots::addr::OSInputGlobalsPtr);
        if (!mgr) return false;
        using Fn = int(__thiscall*)(void*, int, int);
        held = reinterpret_cast<Fn>(nvbots::addr::GetControlState.target)(mgr, ctrl, 0) != 0;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool MenuModeSEH(bool& open) {
    __try {
        open = reinterpret_cast<bool(__cdecl*)()>(nvcoop::avatar::pins::MenuModeAny.addr)();
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool TopMenuSEH(unsigned& id) {
    __try {
        void* mgr = *reinterpret_cast<void**>(kInterfaceMgrPtr);
        if (!mgr) return false;
        id = reinterpret_cast<unsigned(__thiscall*)(void*)>(nvbots::addr::TopMenuId.addr)(mgr);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

struct RawLocal {
    bool ok = false;
    char name[64] = {};
    char place[64] = {};
    uint32_t world = 0, cell = 0, life = 0;
    float x = 0, y = 0, z = 0, heading = 0;
    int nwX = 0, nwY = 0, seX = 0, seY = 0;
    bool haveMap = false;
};

void CopyName(char* dst, size_t n, const char* src) {
    if (!src) return;
    std::strncpy(dst, src, n - 1);
    dst[n - 1] = 0;
}

void ReadPlayerRaw(RawLocal& o) {
    __try {
        PlayerCharacter* pc = g_thePlayer ? *g_thePlayer : nullptr;
        if (!pc || !pc->parentCell) return;
        if (pc->baseForm && pc->baseForm->typeID == kFormType_TESNPC) CopyName(o.name, sizeof o.name, static_cast<TESNPC*>(pc->baseForm)->fullName.name.m_data);
        TESObjectCELL* cell = pc->parentCell;
        o.cell = cell->refID;
        o.world = cell->worldSpace ? cell->worldSpace->refID : 0;
        if (cell->worldSpace) {
            CopyName(o.place, sizeof o.place, cell->worldSpace->fullName.name.m_data);
            const TESWorldSpace::MapData& md = cell->worldSpace->mapData;
            o.nwX = md.cellNWCoordinates.X; o.nwY = md.cellNWCoordinates.Y;
            o.seX = md.cellSECoordinates.X; o.seY = md.cellSECoordinates.Y;
            o.haveMap = o.seX > o.nwX && o.nwY > o.seY;
        } else {
            CopyName(o.place, sizeof o.place, cell->fullName.name.m_data);
        }
        o.x = pc->posX; o.y = pc->posY; o.z = pc->posZ;
        o.heading = pc->rotZ;
        o.life = pc->lifeState;
        o.ok = true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        o.ok = false;
    }
}

struct RawRef {
    bool ok = false;
    uint32_t world = 0, cell = 0;
    float x = 0, y = 0, z = 0;
    char place[64] = {};
};
void ReadRefRaw(TESObjectREFR* r, RawRef& o) {
    __try {
        if (!r || !r->parentCell) return;
        o.cell = r->parentCell->refID;
        o.world = r->parentCell->worldSpace ? r->parentCell->worldSpace->refID : 0;
        o.x = r->posX; o.y = r->posY; o.z = r->posZ;
        o.ok = true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        o.ok = false;
    }
}

// plugin/hooks' resolver would pull the whole hooks library in; the one lookup we need is the engine's own (avatar pin LookupFormByID, checked in PinsOk).
TESForm* LookupFormByIdSafe(uint32_t id) {
    __try {
        return reinterpret_cast<TESForm*(__cdecl*)(uint32_t)>(nvcoop::avatar::pins::LookupFormByID.addr)(id);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}

TESObjectREFR* RefById(uint32_t id) {
    static bool checked = false, ok = false;
    if (!checked) {
        checked = true;
        ok = PeekOk(nvcoop::avatar::pins::LookupFormByID.addr, nvcoop::avatar::pins::LookupFormByID.bytes, 16, false);
        if (!ok) nvcoop::Log("markers: LookupFormByID prologue does not match: no puppet places");
    }
    return (id && ok) ? static_cast<TESObjectREFR*>(LookupFormByIdSafe(id)) : nullptr;
}

// ---------------------------------------------------------------------------------------------------------------------------------- the port
nvcoop::coop::Runtime* Rt() { return nvcoop::coop::Runtime::active(); }

class Port : public nvmk::IMarkerPort {
public:
    nvmk::WorldMapBounds bounds;
    bool pipboyMapOpen = false;
    Place viewer;

    Slot localSlot() override {
        nvcoop::coop::Runtime* rt = Rt();
        return rt && rt->client() ? static_cast<Slot>(rt->client()->slot()) : nvmk::kHostSlot;
    }
    bool isHost() override {
        nvcoop::coop::Runtime* rt = Rt();
        return rt && rt->host() != nullptr;
    }
    bool readLocal(nvmk::LocalState& out) override {
        RawLocal raw;
        ReadPlayerRaw(raw);
        if (!raw.ok) return false;
        out = nvmk::LocalState{};
        out.valid = true;
        out.name = raw.name;
        out.at.valid = true;
        out.at.world = raw.world;
        out.at.cell = raw.cell;
        out.at.x = raw.x; out.at.y = raw.y; out.at.z = raw.z;
        out.at.name = raw.place;
        if (!out.at.interior()) ext_ = out.at;
        out.ext = ext_;
        out.heading = raw.heading;
        out.alive = raw.life == 0;
        bool held = false;
        if (PinsOk() && ControlHeldSEH(kCtrlActivate, held)) out.activateHeld = held;
        bool menu = false;
        if (PinsOk() && MenuModeSEH(menu)) out.uiBlocked = menu;
        viewer = out.at;
        bounds = nvmk::WorldMapBounds{};
        if (raw.haveMap && raw.world) { bounds.valid = true; bounds.nwX = raw.nwX; bounds.nwY = raw.nwY; bounds.seX = raw.seX; bounds.seY = raw.seY; }
        unsigned top = 0;
        pipboyMapOpen = PinsOk() && TopMenuSEH(top) && top == kMenuMap;
        return true;
    }
    bool puppetPlace(Slot s, Place& out) override {
        uint32_t ref = 0;
#ifdef NVCOOP_WITH_LIVE
        ref = nvcoop::livewire::glueAvatarRef(static_cast<uint8_t>(s));
#else
        (void)s;
#endif
        if (!ref) return false;
        RawRef raw;
        ReadRefRaw(RefById(ref), raw);
        if (!raw.ok) return false;
        out = Place{};
        out.valid = true;
        out.world = raw.world;
        out.cell = raw.cell;
        out.x = raw.x; out.y = raw.y; out.z = raw.z;
        return true;
    }
    std::vector<Slot> guests() override {
        std::vector<Slot> g;
        if (nvcoop::coop::Runtime* rt = Rt())
            for (fnvcoop::PlayerId p : rt->joinedSlots()) g.push_back(static_cast<Slot>(p));
        return g;
    }
    bool sendToHost(const std::string& json) override {
        nvcoop::coop::Runtime* rt = Rt();
        return rt && rt->sendOrderToHost(json);
    }
    bool sendToGuest(Slot s, const std::string& json) override {
        nvcoop::coop::Runtime* rt = Rt();
        return rt && rt->sendOrder(s, json);
    }
    void notify(const std::string& t) override { nvcoop::intents::game::engine().notify(t); }
    void notifyGuest(Slot s, const std::string& t) override {
        if (nvcoop::coop::Runtime* rt = Rt()) rt->notifyPlayer(s, t);
    }
    bool ruleDowned(Slot s) override {
        nvcoop::coop::Runtime* rt = Rt();
        return rt && rt->rules() && rt->rules()->isDowned(s);
    }
    bool ruleCheckpointOnly(Slot s) override {
        nvcoop::coop::Runtime* rt = Rt();
        return rt && rt->rules() && rt->rules()->downedUntilCheckpoint(s);
    }
    uint32_t reviveWindowMs() override {
        nvcoop::coop::Runtime* rt = Rt();
        if (!rt || !rt->rules()) return 0;
        const std::string mode = rt->rules()->policy("death.mode", "downed");
        if (mode != "downed") return 0;   // respawn: up at once, normal: no downed state at all
        const double s = std::atof(rt->rules()->policy("death.revive_s", "60").c_str());
        return s > 0 ? static_cast<uint32_t>(s * 1000.0) : 0u;
    }
    void reviveInPlace(Slot s, float frac, const std::string& by) override {
        nvcoop::coop::Runtime* rt = Rt();
        if (!rt || !rt->rules()) return;
        rt->rules()->onPlayerRevived(s);   // no timed respawn at the checkpoint any more
        rt->deliverOrder(s, nvmk::encodeInPlaceRespawn(frac, by));
        nvcoop::Log("markers: revive: player %d gets up in place (%.0f%% health)", (int)s, frac * 100.0f);
    }

private:
    Place ext_;
};

// ---------------------------------------------------------------------------------------------------------------------------------- the module
std::unique_ptr<Port> g_port;
std::unique_ptr<ScriptHud> g_hud;
std::unique_ptr<nvmk::MarkerService> g_svc;
bool g_hudTried = false;
bool g_hadSession = false;
uint32_t g_pendingDownedMs = 0;
bool g_pendingDowned = false;
DownedPoseHook g_poseHook;
std::map<Slot, bool> g_wasDowned;

uint64_t NowMs() { return GetTickCount64(); }

void ResetService() {
    if (g_svc) g_svc->onSessionEnd();
}

void OnLoad(void* nvse) {
    g_nvse = static_cast<NVSEInterface*>(nvse);
    LoadCfg();
    nvcoop::Log("markers: markers %s, revive %s (nvcoop.ini [markers] / [revive])", g_cfg.svc.markers.enabled ? "on" : "off", g_cfg.svc.revive.enabled ? "on" : "off");
    nvcoop::RegisterMethod("markers.status", [](const std::string&, std::string& out) {
        char b[64];
        std::snprintf(b, sizeof b, "{\"v\":1,\"markers\":%s,\"revive\":%s", g_cfg.svc.markers.enabled ? "true" : "false", g_cfg.svc.revive.enabled ? "true" : "false");
        out = b;
        out += ",\"service\":" + (g_svc ? g_svc->statusJson(NowMs()) : std::string("null"));
        out += ",\"hud\":" + (g_hud ? g_hud->statusJson() : std::string("null")) + "}";
        return true;
    });
#ifdef NVCOOP_WITH_AVATAR_ENGINE
    setDownedPoseHook([](uint8_t slot, bool downed) { nvcoop::avatar::glue::SetDownedPose(slot, downed); });
#endif
}

void OnMessage(uint32_t type, void*, uint32_t) {
    switch (type) {
        case NVSEMessagingInterface::kMessage_PreLoadGame:
        case NVSEMessagingInterface::kMessage_NewGame:
        case NVSEMessagingInterface::kMessage_ExitToMainMenu:
            ResetService();
            break;
        default: break;
    }
}

void MakeService() {
    g_port.reset(new Port());
    if (!g_hudTried) {
        g_hudTried = true;
        g_hud.reset(new ScriptHud());
        if (!g_hud->init(g_nvse, g_cfg.layout)) nvcoop::Log("markers: no HUD scripts: the banner goes to the notification line");
        g_hud->setRefLookup([](Slot s) -> TESObjectREFR* {
            uint32_t ref = 0;
#ifdef NVCOOP_WITH_LIVE
            ref = nvcoop::livewire::glueAvatarRef(static_cast<uint8_t>(s));
#else
            (void)s;
#endif
            return RefById(ref);
        });
    }
    g_svc.reset(new nvmk::MarkerService(*g_port, g_hud.get(), g_cfg.svc));
    nvcoop::threat::setDownedQuery([](uint8_t slot) {   // the host's rules know every player; a guest knows its friends from the party table
        nvcoop::coop::Runtime* rt = Rt();
        if (rt && rt->rules() && rt->rules()->isDowned(static_cast<Slot>(slot))) return true;
        if (!g_svc) return false;
        const auto& m = g_svc->board().members();
        const auto it = m.find(static_cast<Slot>(slot));
        return it != m.end() && it->second.downed;
    });
    if (g_pendingDowned) {   // an order that came before the service existed
        g_svc->onLocalDowned(g_pendingDownedMs, NowMs());
        g_pendingDowned = false;
    }
}

void OnFrame() {
    if (!g_cfg.svc.markers.enabled && !g_cfg.svc.revive.enabled) return;
    nvcoop::coop::Runtime* rt = Rt();
    const bool session = rt && (rt->host() || (rt->client() && rt->client()->joined()));
    if (!session) {
        if (g_hadSession) { ResetService(); g_hadSession = false; }
        return;
    }
    g_hadSession = true;
    if (!g_svc) MakeService();
    g_svc->tick(NowMs());
    if (g_poseHook) {   // a friend went down / got up: whoever owns the puppets changes its pose
        for (const auto& kv : g_svc->board().members()) {
            bool& was = g_wasDowned[kv.first];
            if (was != kv.second.downed) { was = kv.second.downed; g_poseHook(kv.first, was); }
        }
        for (auto it = g_wasDowned.begin(); it != g_wasDowned.end();) {   // a downed friend who left the board stands up with his puppet
            if (!g_svc->board().members().count(it->first)) {
                if (it->second) g_poseHook(it->first, false);
                it = g_wasDowned.erase(it);
            } else {
                ++it;
            }
        }
    }
    if (g_hud && g_hud->ready()) {
        MapContext mc;
        mc.pipboyMapOpen = g_port->pipboyMapOpen;
        mc.engineOk = PinsOk();   // the tile reads (menu array 0x11F350C) only on the exe the other pins were taken from
        mc.viewer = g_port->viewer;
        mc.bounds = g_port->bounds;
        g_hud->setMapContext(mc);
    } else if (!g_svc->lastFrame().banner.empty()) {
        // no HUD scripts: the banner on the notification line, when it changes, at most every 8 s
        static std::string last;
        static uint64_t at = 0;
        const std::string b = g_svc->lastFrame().banner;
        if (b != last && NowMs() - at > 8000) {
            last = b;
            at = NowMs();
            g_port->notify(b);
        }
    }
}

}  // namespace

// ---------------------------------------------------------------------------------------------------------------------------------- what coop_module calls
bool isGuestOp(const std::string& op) { return nvmk::isGuestOp(op); }
bool isHostOp(const std::string& op) { return nvmk::isHostOp(op); }

std::string applyGuestOrder(const std::string& json) {
    if (!g_cfg.svc.markers.enabled && !g_cfg.svc.revive.enabled) return "Ok";
    if (!g_svc) MakeService();
    return g_svc->onGuestOrder(json, NowMs());
}

std::string handleHostOrder(uint8_t from, const std::string& json) {
    if (!g_cfg.svc.markers.enabled && !g_cfg.svc.revive.enabled) return "Ok";
    if (!g_svc) MakeService();
    return g_svc->onHostOrder(from, json, NowMs());
}

void setDownedPoseHook(DownedPoseHook hook) { g_poseHook = std::move(hook); }

void onLocalDowned(uint32_t respawnMs) {
    if (g_svc) g_svc->onLocalDowned(respawnMs, NowMs());
    else { g_pendingDowned = true; g_pendingDownedMs = respawnMs; }
}

void onLocalUp(const std::string& revivedBy) {
    g_pendingDowned = false;
    if (g_svc) g_svc->onLocalUp(revivedBy);
}

}  // namespace nvcoop::markers

NVCOOP_MODULE(markers, "markers", nvcoop::markers::OnLoad, nvcoop::markers::OnMessage, nvcoop::markers::OnFrame);
