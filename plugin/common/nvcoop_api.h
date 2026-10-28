// nvcoop public in-process API (owned by L1). Other lanes include this to talk to nvbridge
// and to hook into the plugin lifecycle. Game-free: does not include any NVSE header.
//
//  - PushEvent():          append to the event ring buffer read by the "events" RPC.
//  - SetStateProvider():   L7 registers the function behind the "state" RPC.
//  - RegisterMethod():     add a custom JSON-RPC method (runs on the game main thread by default).
//  - NVCOOP_MODULE():      static registration of load/message/frame callbacks; sources added to the
//                          nvcoop target by a lane's own CMakeLists.txt get them called automatically.
#pragma once
#include <cstdint>
#include <functional>
#include <string>

namespace nvcoop {

constexpr int kBridgeProtocolVersion = 1;

// ---- events ---------------------------------------------------------------
// type: short dotted name, e.g. "quest.setstage". dataJson: a JSON value text ("" == null).
// Thread-safe; callable from any thread. Never blocks on I/O.
void PushEvent(const char* type, const std::string& dataJson = std::string());

// ---- state (L7) -----------------------------------------------------------
// Called on the main thread. sectionsCsv is "" for "all". Returns JSON text of the snapshot.
using StateProvider = std::string (*)(const std::string& sectionsCsv);
void SetStateProvider(StateProvider fn);

// ---- custom methods -------------------------------------------------------
// paramsJson: JSON text of "params" (object or null). On success write JSON text of the result to
// `out` and return true; on failure write a human readable message to `out` and return false.
using MethodFn = bool (*)(const std::string& paramsJson, std::string& out);
// mainThread=true: handler is invoked from the game main loop (safe for engine calls).
// Replaces an existing registration of the same name. Built-in methods cannot be overridden.
void RegisterMethod(const char* name, MethodFn fn, bool mainThread = true);
// Loader support (T3a): same, with a closure and an owner tag (0 = none) so the hot-reload loader can remove all
// methods that belong to a core DLL before unloading it.
using MethodClosure = std::function<bool(const std::string& paramsJson, std::string& out)>;
void RegisterMethodClosure(const char* name, MethodClosure fn, bool mainThread, int owner);
void UnregisterMethodsByOwner(int owner);

// ---- module lifecycle -----------------------------------------------------
// nvseInterface is the NVSEInterface* passed to NVSEPlugin_Load (cast it yourself).
// onMessage mirrors NVSEMessagingInterface::Message (type, data, dataLen).
struct Module {
    const char* name;
    void (*onLoad)(void* nvseInterface);                         // optional
    void (*onMessage)(uint32_t type, void* data, uint32_t len);  // optional
    void (*onFrame)();                                           // optional, main thread, every frame
    int frameOrder = 0;                                          // 0 = the order the modules registered in (link order, not a promise); > 0 = after every module with 0
};
void RegisterModule(const Module& m);

struct ModuleRegistrar {
    explicit ModuleRegistrar(const Module& m) { RegisterModule(m); }
};
#define NVCOOP_MODULE(ident, ...) \
    static const ::nvcoop::ModuleRegistrar nvcoop_module_##ident(::nvcoop::Module{__VA_ARGS__})
// A module whose frame must run AFTER the ordinary ones (the co-op session sends what the avatar / actor modules captured in the same frame; run first it would
// send last frame's). Give all four fields: NVCOOP_MODULE_LATE(ident, name, onLoad, onMessage, onFrame).
#define NVCOOP_MODULE_LATE(ident, ...) \
    static const ::nvcoop::ModuleRegistrar nvcoop_module_##ident(::nvcoop::Module{__VA_ARGS__, 100})

// The one place that decides the order of the per-frame callbacks (plugin/main.cpp and the split loader's core both call it): modules with frameOrder 0 in
// registration order, then the late ones in registration order. Header-only so a game-free test can pin it.
template <class Modules>
inline void RunFrameCallbacks(const Modules& mods) {
    for (int pass = 0; pass < 2; ++pass)
        for (const auto& m : mods)
            if (m.onFrame && (m.frameOrder > 0) == (pass == 1)) m.onFrame();
}

// ---- logging --------------------------------------------------------------
void Log(const char* fmt, ...);

// ---- cosave (NVSE serialization) ------------------------------------------
// A module's cosave save / load callbacks (intfc = the NVSESerializationInterface*; null clears). NVSE takes serialization callbacks
// only during the first plugin load and never drops them, so NVSEPlugin_Load registers forwarders once and they call these. In the
// split build the core's call goes to the permanent loader, which clears them before a core is unloaded: a hot-reloaded core is
// called, never the unloaded one. One owner today (the actor engine's ledger).
void SetCosaveCallbacks(void (*save)(void* intfc), void (*load)(void* intfc));
void RunCosave(bool save, void* intfc);   // the forwarders
bool HasCosaveCallbacks();                 // tests: a save or load callback is set

}  // namespace nvcoop
