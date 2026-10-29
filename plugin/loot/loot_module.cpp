// nvcoop module for loot sync (x86, compiled into nvcoop.dll). NOTHING here has run in a game. Switches in nvcoop.ini:
//   [loot]
//   enabled=0        default OFF. 1 = the service exists and (once dll-wiring adds it to the live hub, see loot_glue.h) syncs containers, drops, pickups
//   poll_ms=50       how often an open container menu is read
//   sweep=1          host: scripts' changes to unheld containers follow into the table
//   resolve=1        register containers when told their cell loaded (corpses are registered shortly after death regardless)
//   corpse_delay_ms=700
//   menuless=1       a take / put the engine did on a closed container without the menu (console RemoveItem + AddItem, quick-loot mods) goes through the lock too
#include <Windows.h>

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <memory>
#include <string>

#include "GameForms.h"
#include "GameObjects.h"
#include "PluginAPI.h"
#include "actors/engine/actor_engine_api.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "common/registry.h"
#include "engine_hooks.h"
#include "loot/loot_engine_real.h"
#include "loot/loot_glue.h"

namespace nvcoop::loot {
namespace {

bool g_created = false, g_inited = false, g_enabled = false;
std::unique_ptr<RealLootEngine> g_engine;
std::unique_ptr<LootSync> g_sync;
TrimHook g_trim;   // plugin/rebalance's scarcity hook, handed to every service this module builds
NVSEInterface* g_nvse = nullptr;
NVSEEventManagerInterface* g_events = nullptr;
bool g_dropHandler = false, g_activateHandler = false;
nvcoop::actors::engine::ListenerId g_diedListener = 0;

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

uint32_t RefIdOf(void* ref) {
    if (!ref) return 0;
    __try {
        return static_cast<TESForm*>(ref)->refID;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

// OnDrop: params are void*[2] = {dropper, droppedRef} (the same event plugin/bots reads)
void OnDropEvent(TESObjectREFR*, void* parameters) {
    if (!g_sync || !parameters) return;
    void** a = static_cast<void**>(parameters);
    g_sync->onDrop(RefIdOf(a[0]), RefIdOf(a[1]));
}

// OnActivate: TODO-RE the parameter order was not verified. The player is recognised by its ref id (0x14), the other ref is the activated one;
// the service only claims the activated ref if it is a world item (describeItem refuses everything else).
void OnActivateEvent(TESObjectREFR* thisObj, void* parameters) {
    if (!g_sync) return;
    uint32_t refs[3] = {RefIdOf(thisObj), 0, 0};
    if (parameters) {
        void** a = static_cast<void**>(parameters);
        refs[1] = RefIdOf(a[0]);
        refs[2] = RefIdOf(a[1]);
    }
    bool playerSeen = false;
    for (uint32_t r : refs) if (r == kPlayerRef) playerSeen = true;
    g_sync->noteActivate(playerSeen);
    if (!playerSeen) return;
    // One pickup is one claim: the same ref can sit in more than one slot (this / argument 0), and a second claim for an item the player already won
    // is a loss to the host (the item would be taken back from the player).
    uint32_t done[3] = {0, 0, 0};
    int n = 0;
    for (uint32_t r : refs) {
        if (!r || r == kPlayerRef) continue;
        bool dup = false;
        for (int i = 0; i < n; ++i) if (done[i] == r) dup = true;
        if (dup) continue;
        done[n++] = r;
        g_sync->onPickup(kPlayerRef, r);
    }
}

void InstallHandlers() {
    if (!g_nvse || g_events) return;
    g_events = static_cast<NVSEEventManagerInterface*>(g_nvse->QueryInterface(kInterface_EventManager));
    g_dropHandler = g_events && g_events->SetNativeEventHandler && g_events->SetNativeEventHandler("OnDrop", &OnDropEvent);
    g_activateHandler = g_events && g_events->SetNativeEventHandler && g_events->SetNativeEventHandler("OnActivate", &OnActivateEvent);
    nvcoop::Log("loot: OnDrop handler %s, OnActivate handler %s", g_dropHandler ? "registered" : "unavailable", g_activateHandler ? "registered" : "unavailable");
}

void RemoveHandlers() {
    if (g_events && g_events->RemoveNativeEventHandler) {
        if (g_dropHandler) g_events->RemoveNativeEventHandler("OnDrop", &OnDropEvent);
        if (g_activateHandler) g_events->RemoveNativeEventHandler("OnActivate", &OnActivateEvent);
    }
    g_events = nullptr;
    g_dropHandler = g_activateHandler = false;
}

// The service itself: ini, engine, LootSync. The live hub is built before DeferredInit (and before this module's OnLoad ran), and needs the service
// to register it, so this half is asked for early (service()); handlers and listeners need the NVSE interface and wait for Init().
void Create() {
    if (g_created) return;
    g_created = true;
    const std::string ini = IniPath();
    SyncConfig cfg;
    cfg.enabled = GetPrivateProfileIntA("loot", "enabled", 0, ini.c_str()) != 0;
    cfg.pollMs = static_cast<uint32_t>(GetPrivateProfileIntA("loot", "poll_ms", 50, ini.c_str()));
    cfg.hostSweep = GetPrivateProfileIntA("loot", "sweep", 1, ini.c_str()) != 0;
    cfg.resolveOnLoad = GetPrivateProfileIntA("loot", "resolve", 1, ini.c_str()) != 0;
    cfg.menuless = GetPrivateProfileIntA("loot", "menuless", 1, ini.c_str()) != 0;
    cfg.discover = GetPrivateProfileIntA("loot", "discover", 1, ini.c_str()) != 0;
    cfg.discoverCells = static_cast<uint32_t>(GetPrivateProfileIntA("loot", "discover_cells", 3, ini.c_str()));
    cfg.corpseDelayMs = static_cast<uint32_t>(GetPrivateProfileIntA("loot", "corpse_delay_ms", 700, ini.c_str()));
    g_enabled = cfg.enabled;
    g_engine = std::make_unique<RealLootEngine>();
    // The actor port copies an owner's item use (a ganger drinks a Stimpak) to the other machines; the engine calls are this module's.
    nvcoop::actors::engine::setInventoryAccess(
        [](nvcoop::actors::NetId net, nvcoop::actors::InvList& out) {
            if (!g_engine) return false;
            Items have;
            bool exact = true;
            if (!g_engine->readContents(net, have, exact) || !exact) return false;
            out.clear();
            for (const auto& kv : have)
                if (kv.second > 0) out.push_back({kv.first, kv.second});
            return true;
        },
        [](nvcoop::actors::NetId net, const nvcoop::actors::InvList& want) {
            if (!g_engine) return false;
            Items items;
            for (const auto& it : want) items[it.form] += it.count;
            return g_engine->setContents(net, items);
        });
    g_sync = std::make_unique<LootSync>(*g_engine, cfg);
    g_sync->setTrimHook(g_trim);
}

void Init() {
    if (g_inited) return;
    g_inited = true;
    Create();
    if (!g_enabled) { nvcoop::Log("loot: disabled (nvcoop.ini [loot] enabled=0)"); return; }
    const std::string ini = IniPath();
    if (GetPrivateProfileIntA("intents", "enable", 0, ini.c_str()) != 0 &&
        (GetPrivateProfileIntA("intents", "pickup", 1, ini.c_str()) != 0 || GetPrivateProfileIntA("intents", "container", 1, ini.c_str()) != 0))
        nvcoop::Log("loot: WARNING nvcoop.ini [intents] pickup / container are on as well: a pickup or a container would be handled twice. Set [intents] pickup=0 and container=0 (docs/tools/loot.md)");
    InstallHandlers();
    g_diedListener = nvcoop::actors::engine::addActorDiedListener([](const nvcoop::actors::engine::DiedEvent& e) {
        if (g_sync && e.mine && e.authorised) g_sync->onActorDied(e.netId, e.ref);
    });
    nvcoop::Log("loot: enabled (the live hub runs it in a session: live_glue.cpp adds nvcoop::loot::service())");
}

void Uninit() {
    nvcoop::actors::engine::setInventoryAccess(nullptr, nullptr);
    if (g_diedListener) { nvcoop::actors::engine::removeListener(g_diedListener); g_diedListener = 0; }
    RemoveHandlers();
    g_sync.reset();
    g_engine.reset();
    g_created = false;
    g_inited = false;
    g_enabled = false;
}

bool RpcLoot(const std::string& p, std::string& out) { return glueRpc(p, out); }

void OnLoad(void* nvse) {
    g_nvse = static_cast<NVSEInterface*>(nvse);
    nvcoop::RegisterMethod("coop.loot", RpcLoot, true);
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_DeferredInit) Init();
    else if (type == NVSEMessagingInterface::kMessage_PostLoadGame && g_engine) g_engine->bumpEpoch();
    else if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) Uninit();
}

}  // namespace

bool enabled() { return g_enabled && g_sync != nullptr; }

void setTrimHook(TrimHook hook) {
    g_trim = std::move(hook);
    if (g_sync) g_sync->setTrimHook(g_trim);
}

LootSync& service() {
    Create();   // asked before DeferredInit (the hub is built early): read the ini now, the handlers follow in Init()
    return *g_sync;
}

// A form / ref id as a hex string ("104C70", "0x104C70") or a number
static uint32_t IdOf(const nvcoop::json::Value& v) {
    if (v.isNumber()) return static_cast<uint32_t>(v.asInt());
    if (v.isString()) return static_cast<uint32_t>(std::strtoul(v.asString().c_str(), nullptr, 16));
    return 0;
}

// Test helper, needs env NVCOOP_TEST_HELPERS=1 (the benchmark sets it):  coop.loot {"grab":{"container":"<runtime ref hex>","item":"<runtime form hex>","count":1,"phase":"both|remove|add"}}
// A quick-loot style take: RemoveItem on the container + AddItem on the local player, with the guest gate lifted. The benchmark cannot do this with console lines on a
// GUEST: "RemoveItem <item> 1 1" with the corpse as calling reference is a script mutation and the guest gate (plugin/hooks/suppress_policy.cpp) drops it, so the old
// grab took nothing out of the guest's corpse while the "player.AddItem" line (a local-player line) still gave the guest the item.
static bool GrabRpc(const nvcoop::json::Value& g, std::string& out) {
    const char* gate = std::getenv("NVCOOP_TEST_HELPERS");
    if (!gate || gate[0] != '1') { out = "test helpers disabled (start the game with NVCOOP_TEST_HELPERS=1)"; return false; }
    if (!g_engine) { out = "loot engine not created"; return false; }
    const std::string phase = g.get("phase").asString("both");
    if (phase != "both" && phase != "remove" && phase != "add") { out = "grab.phase must be both, remove or add"; return false; }
    std::string why;
    const bool ok = g_engine->testGrab(IdOf(g.get("container")), IdOf(g.get("item")), static_cast<int32_t>(g.get("count").asInt(1)), phase != "add", phase != "remove", &why);
    if (!ok) { out = "grab refused: " + why; return false; }
    out = "{\"grab\":true,\"phase\":\"" + phase + "\"}";
    return true;
}

bool glueRpc(const std::string& paramsJson, std::string& out) {
    if (!g_sync) { out = "{\"enabled\":false}"; return true; }
    nvcoop::json::Value p;
    std::string err;
    if (!paramsJson.empty() && paramsJson != "null" && !nvcoop::json::parse(paramsJson, p, err)) { out = "bad params: " + err; return false; }
    if (p.isObject() && p.get("grab").isObject()) return GrabRpc(p.get("grab"), out);
    if (p.isObject() && p.get("container").isNumber()) {
        const lc::ContainerRec* c = g_sync->client() ? g_sync->client()->container(static_cast<NetId>(p.get("container").asInt())) : nullptr;
        if (!c) { out = "{\"known\":false}"; return true; }
        char b[256];
        std::snprintf(b, sizeof b, "{\"known\":true,\"kind\":\"%s\",\"rev\":%u,\"holder\":%d,\"stacks\":%zu,\"digest\":\"%016llx\"}", lc::toString(c->kind), c->rev,
                      c->holder == kNoPlayer ? -1 : static_cast<int>(c->holder), c->items.size(), static_cast<unsigned long long>(lc::digestOf(c->items)));
        out = b;
        return true;
    }
    out = g_sync->statusJson();
    return true;
}

}  // namespace nvcoop::loot

NVCOOP_MODULE(loot_sync, "loot_sync", nvcoop::loot::OnLoad, nvcoop::loot::OnMessage, nullptr);
