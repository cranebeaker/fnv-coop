// Party saves, engine half. See party_save_glue.h and docs/party-save.md. NOTHING HERE HAS RUN IN A GAME.
#include "party_save_glue.h"

#include <Windows.h>

#include <ctime>
#include <fstream>
#include <memory>
#include <random>
#include <sstream>

#include "common/nvcoop_api.h"
#include "intents/engine/game_engine.h"   // engine().notify: the game's HUD line
#include "party_save.h"
#ifdef NVCOOP_WITH_LIVE
#include "live/live_glue.h"   // glueBeforeSave / glueAfterSave: clones made by the actor port must not be written into the save
#endif

namespace nvcoop::coop::partyglue {
namespace {
namespace pt = nvcoop::coop::party;

// ---- the engine's save (addresses of 1.4.0.525, the same ones plugin/main.cpp uses for the bridge's save) ------------------------------------------
constexpr uint32_t kSaveLoadMgrPtr = 0x11DE134;   // BGSSaveLoadGame* (SaveGameManager)
constexpr uint32_t kSaveGameFn = 0x8503B0;        // bool __thiscall (mgr, const char* name, int slot, bool flag)
constexpr uint32_t kCanSaveNowFn = 0x850FE0;      // bool __thiscall (mgr, bool ignoreMenus): called through the SaveGate detour, so a scene's SaveSuppress answers no

int CanSaveSEH() {
    int r = -1;
    __try {
        void* mgr = *reinterpret_cast<void**>(kSaveLoadMgrPtr);
        if (!mgr) return -2;
        r = reinterpret_cast<bool(__thiscall*)(void*, bool)>(kCanSaveNowFn)(mgr, false) ? 1 : 0;   // false: menus count (nobody saves into the Pip-Boy)
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        r = -3;
    }
    return r;
}

int SaveSEH(const char* name) {
    int r = -1;
    __try {
        void* mgr = *reinterpret_cast<void**>(kSaveLoadMgrPtr);
        if (!mgr) return -2;
        r = reinterpret_cast<bool(__thiscall*)(void*, const char*, int, bool)>(kSaveGameFn)(mgr, name, -1, false) ? 1 : 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        r = -3;
    }
    return r;
}

bool EngineCanSave() { return CanSaveSEH() == 1; }

bool EngineSave(const std::string& name) {
#ifdef NVCOOP_WITH_LIVE
    nvcoop::livewire::glueBeforeSave();
#endif
    const int r = SaveSEH(name.c_str());
#ifdef NVCOOP_WITH_LIVE
    nvcoop::livewire::glueAfterSave();
#endif
    nvcoop::Log("coop: party save: engine save '%s' -> %d", name.c_str(), r);
    return r == 1;
}

// ---- files ------------------------------------------------------------------------------------------------------------------------------------------
std::string GameDir() {
    char b[MAX_PATH] = {};
    GetModuleFileNameA(nullptr, b, MAX_PATH);
    std::string p = b;
    const size_t s = p.find_last_of("\\/");
    return s == std::string::npos ? std::string() : p.substr(0, s + 1);
}

std::string DocumentsDir() {   // shell32 is looked up at run time: the split core does not link it
    std::string out;
    if (HMODULE h = LoadLibraryA("shell32.dll")) {
        using Fn = HRESULT(WINAPI*)(HWND, int, HANDLE, DWORD, LPSTR);
        if (auto f = reinterpret_cast<Fn>(GetProcAddress(h, "SHGetFolderPathA"))) {
            char b[MAX_PATH] = {};
            if (SUCCEEDED(f(nullptr, 5 /*CSIDL_PERSONAL*/, nullptr, 0, b))) out = b;
        }
        FreeLibrary(h);
    }
    return out;
}

// The folder the game saves into, decided the way the game does (the same rule as plugin/main.cpp GameSaveDir and tools/launcher/fnv_party_saves.py).
std::string SaveDir() {
    const std::string docs = DocumentsDir();
    const std::string myGames = docs.empty() ? std::string() : docs + "\\My Games\\FalloutNV\\";
    std::string ini = myGames + "Fallout.ini";
    if (myGames.empty() || GetFileAttributesA(ini.c_str()) == INVALID_FILE_ATTRIBUTES) ini = GameDir() + "Fallout.ini";
    const bool useMyGames = GetPrivateProfileIntA("General", "bUseMyGamesDirectory", 1, ini.c_str()) != 0 && !myGames.empty();
    char local[MAX_PATH] = {};
    GetPrivateProfileStringA("General", "SLocalSavePath", "Saves\\", local, sizeof local, ini.c_str());
    std::string dir = (useMyGames ? myGames : GameDir()) + local;
    if (!dir.empty() && dir.back() != '\\' && dir.back() != '/') dir += '\\';
    return dir;
}

bool Exists(const std::string& p) { return GetFileAttributesA(p.c_str()) != INVALID_FILE_ATTRIBUTES; }

// Where the .fos of a save named by the SaveGame / PreLoadGame message is: a full path as it is, else the game's save folder, else Saves\ under the game folder.
std::string SavePathOf(const std::string& raw) {
    if (raw.size() > 1 && (raw[1] == ':' || raw[0] == '\\' || raw[0] == '/')) return raw;
    const std::string base = pt::saveBaseName(raw);
    const std::string a = SaveDir() + base + ".fos";
    if (Exists(a)) return a;
    const std::string b = GameDir() + "Saves\\" + base + ".fos";
    return Exists(b) ? b : a;
}

std::string ReadFile(const std::string& path) {
    std::ifstream f(path, std::ios::binary);
    if (!f) return std::string();
    std::ostringstream o;
    o << f.rdbuf();
    return o.str();
}

bool WriteFile(const std::string& path, const std::string& text) {   // written beside, then moved over: a reader never sees half a file
    const std::string tmp = path + ".tmp";
    {
        std::ofstream f(tmp, std::ios::binary | std::ios::trunc);
        if (!f) return false;
        f << text;
        if (!f) return false;
    }
    return MoveFileExA(tmp.c_str(), path.c_str(), MOVEFILE_REPLACE_EXISTING) != 0;
}

std::string IndexPath() { return GameDir() + "Data\\NVSE\\Plugins\\" + pt::kIndexFile; }

// ---- the machines -----------------------------------------------------------------------------------------------------------------------------------
std::unique_ptr<pt::HostParty> g_host;
std::unique_ptr<pt::GuestParty> g_guest;
Runtime* g_rt = nullptr;
uint64_t g_loadedCampaign = 0;   // from the loaded save's sidecar; kept here because a save can load before the session exists
nvcoop::rebalance::CampaignSettings g_loadedSettings;   // ... and its rebalance setting (docs/rebalance.md)
nvcoop::rebalance::CampaignSettings g_newCampaign;      // host: what the next campaign this host starts will play (nvcoop.ini [rebalance], set by plugin/rebalance)

uint64_t Random64() {
    std::random_device rd;
    return (static_cast<uint64_t>(rd()) << 32) ^ rd();
}

void Hud(const std::string& s) { nvcoop::intents::game::engine().notify(s); }
void Log(const std::string& s) { nvcoop::Log("coop: %s", s.c_str()); }

void Ensure(Runtime* rt) {
    if (rt != g_rt) {   // another session (a rejoin builds a new Runtime): fresh machines
        g_host.reset();
        g_guest.reset();
        g_rt = rt;
    }
    if (!rt) return;
    if (rt->host() && !g_host) {
        pt::HostEnv e;
        e.joined = [rt] { std::vector<uint8_t> v; for (fnvcoop::PlayerId s : rt->joinedSlots()) v.push_back(static_cast<uint8_t>(s)); return v; };
        e.sendOrder = [rt](uint8_t slot, const std::string& j) { return rt->sendOrder(static_cast<fnvcoop::PlayerId>(slot), j); };
        e.canSaveNow = EngineCanSave;
        e.saveNow = EngineSave;
        e.log = Log;
        e.random64 = Random64;
        e.nowMs = [] { return static_cast<uint64_t>(GetTickCount64()); };
        g_host.reset(new pt::HostParty(e));
        g_host->setCampaign(g_loadedCampaign);
        g_host->setSettings(g_loadedSettings);
        g_host->setNewCampaignSettings(g_newCampaign);
    }
    if (rt->client() && !g_guest) {
        pt::GuestEnv e;
        e.mySlot = [rt] { return rt->client() && rt->client()->joined() ? static_cast<int>(rt->client()->slot()) : -1; };
        e.sendToHost = [rt](const std::string& j) { return rt->sendOrderToHost(j); };
        e.canSaveNow = EngineCanSave;
        e.saveNow = EngineSave;
        e.notify = Hud;
        e.log = Log;
        e.wallSeconds = [] { return static_cast<int64_t>(std::time(nullptr)); };
        e.nowMs = [] { return static_cast<uint64_t>(GetTickCount64()); };
        e.readIndex = [] { return ReadFile(IndexPath()); };
        e.writeIndex = [](const std::string& t) { return WriteFile(IndexPath(), t); };
        g_guest.reset(new pt::GuestParty(e));
        g_guest->setCampaign(g_loadedCampaign);
        g_guest->setSettings(g_loadedSettings);   // until the host says (its word wins)
    }
}

void WriteSidecar(const std::string& rawName, const pt::Meta& m) {
    const std::string path = SavePathOf(rawName) + pt::kSidecarExt;
    const bool ok = WriteFile(path, pt::metaText(m));
    nvcoop::Log("coop: party save: sidecar %s %s", path.c_str(), ok ? "written" : "NOT written");
}
}  // namespace

void frame(Runtime* rt) {
    Ensure(rt);
    if (!rt) return;
    if (g_host) g_host->tick();
    if (g_guest) g_guest->tick();
}

void onSaved(Runtime* rt, const std::string& rawName) {
    Ensure(rt);
    if (!rt || pt::saveBaseName(rawName).empty()) return;
    if (g_host) {
        g_host->onHostSaved(rawName);   // orders every joined guest; makes the campaign when the party has none yet
        pt::Meta m;
        m.campaign = g_host->campaign();
        m.settings = g_host->settings();
        m.host = true;
        m.hostSave = pt::saveBaseName(rawName);
        WriteSidecar(rawName, m);
    } else if (g_guest) {
        g_guest->onLocalSave(rawName);   // a hand save is asked of the host; a save this machine made for a party save order is recognised
        if (g_guest->campaign()) {
            pt::Meta m;
            m.campaign = g_guest->campaign();
            m.settings = g_guest->settings();
            m.host = false;
            m.hostSave = g_guest->ownHostSave(rawName);
            WriteSidecar(rawName, m);
        }
    }
}

void onLoaded(Runtime* rt, const std::string& loadedPath) {
    pt::Meta m;
    g_loadedCampaign = pt::parseMeta(ReadFile(SavePathOf(loadedPath) + pt::kSidecarExt), &m) ? m.campaign : 0;
    g_loadedSettings = g_loadedCampaign ? m.settings : nvcoop::rebalance::CampaignSettings();
    nvcoop::Log("coop: party save: loaded '%s', campaign %s", loadedPath.c_str(), g_loadedCampaign ? pt::campaignHex(g_loadedCampaign).c_str() : "none");
    Ensure(rt);
    if (g_host) { g_host->setCampaign(g_loadedCampaign); g_host->setSettings(g_loadedSettings); }
    if (g_guest && g_loadedCampaign) {
        g_guest->setCampaign(g_loadedCampaign);
        if (!g_guest->settings().chosen) g_guest->setSettings(g_loadedSettings);   // (the host's order, when it came, stays)
    }
}

void startCampaign(Runtime* rt) {
    Ensure(rt);
    if (g_host) g_host->ensureCampaign();
}

nvcoop::rebalance::CampaignSettings campaignSettings() {
    if (g_host) return g_host->settings();
    if (g_guest) return g_guest->settings();
    return nvcoop::rebalance::CampaignSettings();
}

int partySize() {
    if (g_host) return g_host->partySize();
    if (g_guest) return g_guest->players();
    return 0;
}

bool isHost() { return g_host != nullptr; }

void setNewCampaignSettings(const nvcoop::rebalance::CampaignSettings& s) {
    g_newCampaign = s;
    if (g_host) g_host->setNewCampaignSettings(s);
}

bool isGuestOp(const std::string& op) { return pt::isGuestOp(op); }
bool isHostOp(const std::string& op) { return pt::isHostOp(op); }

std::string applyOrder(Runtime* rt, const nvcoop::json::Value& order) {
    Ensure(rt);
    return g_guest ? pt::applyGuestOrder(*g_guest, order) : std::string("NotAGuest");
}

std::string handleOrder(Runtime* rt, fnvcoop::PlayerId from, const nvcoop::json::Value& order) {
    Ensure(rt);
    return g_host ? pt::handleHostOrder(*g_host, static_cast<uint8_t>(from), order) : std::string("NotTheHost");
}

}  // namespace nvcoop::coop::partyglue
