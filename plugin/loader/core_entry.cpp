// Entry points of the reloadable core DLL (split build). The permanent loader (main.cpp built with
// NVCOOP_LOADER) calls these through loader_abi.h. This file does what the monolith's main.cpp does for
// modules: run onLoad / onMessage / onFrame, and own the "state" and "plugins" bridge methods.
#include "nvse/PluginAPI.h"
#include "nvse/GameData.h"

#include <cstdlib>
#include <string>

#include "common/nvcoop_api.h"
#include "common/registry.h"
#include "core_thunks.h"

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <windows.h>

#ifdef NVCOOP_WITH_SNAPSHOT
#include "fnvcoop_snapshot/snapshot.h"
#endif

// T3b-state's reload contract (module onBeforeUnload / onAfterReload registry). Optional until it exists.
#if __has_include("common/reload.h")
#include "common/reload.h"
#define NVCORE_HAVE_RELOAD_REGISTRY 1
#endif

namespace {

using nvcoop::Log;
const NvLoaderApi* g_api = nullptr;

#ifdef NVCOOP_WITH_SNAPSHOT
// Same logic as main.cpp (monolith). Kept in sync by hand; both are tiny.
std::string IniPath() { return std::string(g_api->game_dir) + "Data\\NVSE\\Plugins\\nvcoop.ini"; }
std::string ReadIni(const char* section, const char* key, const char* def) {
    char buf[512];
    GetPrivateProfileStringA(section, key, def, buf, sizeof buf, IniPath().c_str());
    return buf;
}
std::string EnvOr(const char* name, const std::string& fallback) {
    char buf[512];
    DWORD n = GetEnvironmentVariableA(name, buf, sizeof buf);
    return (n > 0 && n < sizeof buf) ? std::string(buf) : fallback;
}

std::string StateProviderImpl(const std::string& sectionsCsv) {
    fnvcoop::snapshot::SnapshotOptions o;
    o.playerId = "host";
    o.instance = "host";
    o.indent = -1;
    {
        std::string wr = EnvOr("FNVCOOP_WATCHREFS", ReadIni("bridge", "watchRefs", ""));
        if (!wr.empty()) o.watchRefsFile = wr;
    }
    if (!sectionsCsv.empty()) {
        o.shared = sectionsCsv.find("shared") != std::string::npos;
        o.perPlayer = sectionsCsv.find("perPlayer") != std::string::npos;
    }
    return fnvcoop::snapshot::SnapshotJson(o);
}

bool PluginsMethod(const std::string&, std::string& out) {
    DataHandler* dh = DataHandler::Get();
    if (!dh) { out = "DataHandler not available"; return false; }
    out = "{\"count\":" + std::to_string(dh->GetActiveModCount()) + ",\"plugins\":[";
    const UInt32 n = dh->GetActiveModCount();
    for (UInt32 i = 0, first = 1; i < 255 && i < n + 8; ++i) {
        const char* name = dh->GetNthModName(i);
        if (!name || !*name) continue;
        if (!first) out += ",";
        first = 0;
        out += "{\"index\":" + std::to_string(i) + ",\"name\":\"" + name + "\"}";
    }
    out += "]}";
    return true;
}
#endif

int CoreInit() {
#ifdef NVCOOP_WITH_SNAPSHOT
    nvcoop::RegisterMethod("plugins", PluginsMethod, true);
    nvcoop::SetStateProvider(StateProviderImpl);
#endif
    for (auto& m : nvcoop::registry::Modules()) {
        Log("module %s", m.name ? m.name : "?");
        if (m.onLoad) m.onLoad(g_api->nvse_interface);
    }
#ifdef NVCORE_HAVE_RELOAD_REGISTRY
    if (!nvcoop::reload::AfterLoad(g_api)) {
        Log("core: onAfterReload failed");
        return 0;
    }
#endif
    return 1;
}

void CoreOnMessage(uint32_t type, void* data, uint32_t len) {
    for (auto& m : nvcoop::registry::Modules())
        if (m.onMessage) m.onMessage(type, data, len);
}

void CoreOnFrame() { nvcoop::RunFrameCallbacks(nvcoop::registry::Modules()); }   // frameOrder: the co-op session runs after the capture modules (nvcoop_api.h)

int CoreBeforeUnload() {
#ifdef NVCORE_HAVE_RELOAD_REGISTRY
    return nvcoop::reload::BeforeUnload(g_api) ? 1 : 0;
#else
    // No reload contract compiled in: modules cannot prove they are safe to unload.
    Log("core: reload registry not built in; refusing unload");
    return 0;
#endif
}

}  // namespace

extern "C" __declspec(dllexport) int NvCore_Init(const NvLoaderApi* api, NvCoreApi* out) {
    if (!api || api->abi_version != NVLOADER_ABI_VERSION || api->size < sizeof(NvLoaderApi)) return 0;
    g_api = api;
    nvcoop::loader_core::SetApi(api);
    out->abi_version = NVLOADER_ABI_VERSION;
    out->size = sizeof(NvCoreApi);
    out->init = &CoreInit;
    out->on_message = &CoreOnMessage;
    out->on_frame = &CoreOnFrame;
    out->before_unload = &CoreBeforeUnload;
    return 1;
}
