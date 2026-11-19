// nvcoop module: applies the NV:MP client.dll patches once NV:MP's client.dll is loaded in this process.
// Registered with NVCOOP_MODULE; compiled into nvcoop.dll by plugin/nvmp/CMakeLists.txt. Game-free
// (includes only nvcoop_api.h), so tests/test_client_patches.cpp exercises it without NVSE.
//
// Config (Data\NVSE\Plugins\nvcoop.ini):
//   [nvmp]
//   enabled=1            0 = never touch NV:MP
//   quested=1            client.quested_not_localized         (recommended first patch)
//   questedRef=0         client.quested_ref_not_localized     (quest items / containers)
//   questedRefScan=0     client.quested_ref_scan_include
//   scriptedRef=0        client.scripted_ref_not_localized
//   ghostDamage=0        client.ghost_damage_* (both)
//   patches=             explicit comma list (full or short names), replaces the flags above
//   autoApply=1          0 = only the RPC applies patches
//   pollFrames=60        fallback poll period while client.dll is not loaded yet
// Env override: NVCOOP_NVMP_PATCHES=quested,questedRef  (same as patches=; empty/"none" disables).
//
// Events (RPC "events"): nvmp.client.detected, nvmp.patch {name,status,rva,ok}, nvmp.patches.done {applied,failed},
// nvmp.client.absent. RPC: nvmp.patches {} -> status; nvmp.patches {"apply":"name"} / {"revert":"name"}.
#include <windows.h>

#include <cstring>
#include <memory>
#include <mutex>
#include <string>

#include "../../common/minijson.h"
#include "../../common/nvcoop_api.h"
#include "../../reload/reload_state.h"
#include "client_patcher.h"
#include "players.h"
#include "../ghost_ref.h"
#include "../sigscan.h"

namespace nvmp::client {

// ASCII, case-insensitive, no CRT: is `n` (not necessarily NUL-terminated, `len` wchar_t) exactly "client.dll"?
bool IsClientDllName(const wchar_t* n, size_t len) {
    static const char kName[] = "client.dll";
    if (len != sizeof(kName) - 1) return false;
    for (size_t i = 0; i < len; ++i) {
        wchar_t c = n[i];
        if (c >= L'A' && c <= L'Z') c = (wchar_t)(c - L'A' + L'a');
        if (c != (wchar_t)kName[i]) return false;
    }
    return true;
}

namespace {

// ---- modules -----------------------------------------------------------------------------------
class WinModules : public ModuleSource {
public:
    bool Find(const std::string& name, ModuleView& out) override {
        HMODULE h = GetModuleHandleA(name.c_str());
        if (!h) h = GetModuleHandleA(("nvmp\\" + name).c_str());
        if (!h) return false;
        // Only NV:MP's own client.dll (it lives in an "nvmp" folder): another plugin may also be called client.dll.
        char path[MAX_PATH] = {};
        GetModuleFileNameA(h, path, MAX_PATH);
        for (char* p = path; *p; ++p) if (*p >= 'A' && *p <= 'Z') *p = (char)(*p - 'A' + 'a');
        if (!std::strstr(path, "nvmp")) return false;
        const uint8_t* b = (const uint8_t*)h;
        const IMAGE_DOS_HEADER* dos = (const IMAGE_DOS_HEADER*)b;
        if (dos->e_magic != IMAGE_DOS_SIGNATURE) return false;
        const IMAGE_NT_HEADERS32* nt = (const IMAGE_NT_HEADERS32*)(b + dos->e_lfanew);
        if (nt->Signature != IMAGE_NT_SIGNATURE || nt->OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR32_MAGIC) return false;
        out.base = (uint8_t*)h;
        out.size = nt->OptionalHeader.SizeOfImage;
        return true;
    }
};

// ---- DLL notification (ntdll) -------------------------------------------------------------------
struct UStr { USHORT Length, MaximumLength; PWSTR Buffer; };
struct LdrNotifyData { ULONG Flags; const UStr* FullDllName; const UStr* BaseDllName; PVOID DllBase; ULONG SizeOfImage; };
typedef VOID(CALLBACK* LdrNotifyFn)(ULONG reason, const LdrNotifyData* data, PVOID ctx);
typedef LONG(NTAPI* LdrRegFn)(ULONG flags, LdrNotifyFn fn, PVOID ctx, PVOID* cookie);

std::unique_ptr<ClientPatcher> g_patcher;
WinModules g_mods;
PVOID g_cookie = nullptr;

VOID CALLBACK OnDllNotify(ULONG reason, const LdrNotifyData* d, PVOID) {
    // Runs under the loader lock on whatever thread loads a DLL (possibly one the CRT never initialised): touch NOTHING
    // but raw memory here. No CRT calls (the previous _wcsnicmp allocates per-thread locale data), no logging, no allocation.
    if (reason != 1 /*LDR_DLL_NOTIFICATION_REASON_LOADED*/ || !d || !d->BaseDllName || !g_patcher) return;
    const UStr* n = d->BaseDllName;
    if (n->Buffer && IsClientDllName(n->Buffer, n->Length / sizeof(wchar_t))) g_patcher->NotifyModuleLoaded();
}

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, (LPCSTR)&IniPath, &self);
    char p[MAX_PATH] = {};
    GetModuleFileNameA(self, p, MAX_PATH);
    std::string s = p;
    size_t sl = s.find_last_of("\\/");
    return (sl == std::string::npos ? std::string() : s.substr(0, sl + 1)) + "nvcoop.ini";
}

// ---- RPC ----------------------------------------------------------------------------------------
bool RpcPatches(const std::string& paramsJson, std::string& out) {
    if (!g_patcher) { out = "nvmp client patcher is not active"; return false; }
    nvcoop::json::Value v;
    std::string err;
    if (!paramsJson.empty() && paramsJson != "null" && !nvcoop::json::parse(paramsJson, v, err)) { out = "bad params: " + err; return false; }
    for (const char* key : {"apply", "revert"}) {
        const nvcoop::json::Value* n = v.find(key);
        if (!n || !n->isString()) continue;
        const bool apply = key[0] == 'a';
        std::vector<std::string> names = ExpandName(n->asString());
        if (names.empty()) { out = "unknown patch: " + n->asString(); return false; }
        for (auto& name : names) {
            PatchStatus s = apply ? g_patcher->Apply(name) : g_patcher->Revert(name);
            if (s != PatchStatus::Applied && s != PatchStatus::AlreadyApplied && s != PatchStatus::Reverted) {
                out = name + ": " + ToString(s);
                return false;
            }
        }
    }
    out = g_patcher->StatusJson();
    return true;
}

// ---- nvmp.players ---------------------------------------------------------------------------------
// [{netId, name, refId, isLocal}] for every NV:MP character the client knows (remote players are ghosts in the host's world).
std::mutex g_bindMu;
std::vector<SlotBindings> g_bindings;   // explicit coop slot -> NV:MP net id

// Resolves the natives once and walks the character list. Main thread only.
bool Snapshot(std::vector<PlayerInfo>& ps, std::string& out) {
    static PlayersNatives natives;
    static bool resolved = false;
    ModuleView mv;
    if (!g_mods.Find("client.dll", mv)) { out = "NV:MP client.dll is not loaded"; return false; }
    if (!resolved || !natives.managerGlobal) {
        ResolvedPlayers r = ResolvePlayers(mv.base, mv.size);
        if (!r.ok()) { out = "NV:MP layout not recognised: " + r.error; return false; }
#if defined(_M_IX86)
        natives.getComponent = (decltype(natives.getComponent))(mv.base + r.getComponentRva);
        natives.getGameRef = (decltype(natives.getGameRef))(mv.base + r.getGameRefRva);
#endif
        natives.managerGlobal = r.managerGlobal;
        natives.thePlayerGlobal = kThePlayerGlobal_1_4_0_525;
        resolved = true;
        nvcoop::Log("nvmp players: GetComponent=%p GetGameRef=%p managerGlobal=0x%08X", (void*)(mv.base + r.getComponentRva),
                    (void*)(mv.base + r.getGameRefRva), (unsigned)r.managerGlobal);
    }
    bool faulted = false;
    ps = EnumeratePlayers(natives, PlayersLayout{}, &faulted);
    if (faulted) nvcoop::Log("nvmp players: memory fault while walking the character list (partial result)");
    return true;
}

bool RpcPlayers(const std::string&, std::string& out) {
    std::vector<PlayerInfo> ps;
    if (!Snapshot(ps, out)) return false;
    out = PlayersJson(ps);
    return true;
}

// {"slot":1,"netId":11} binds a coop slot to an NV:MP character; {"slot":1,"netId":null} or {"clear":true} removes bindings.
bool RpcBindSlot(const std::string& paramsJson, std::string& out) {
    nvcoop::json::Value v;
    std::string err;
    if (!nvcoop::json::parse(paramsJson, v, err)) { out = "bad params: " + err; return false; }
    std::lock_guard<std::mutex> g(g_bindMu);
    if (v.get("clear").asBool(false)) g_bindings.clear();
    const nvcoop::json::Value* s = v.find("slot");
    if (s && s->isNumber()) {
        const int slot = (int)s->asInt();
        if (slot < 1 || slot > 255) { out = "slot must be 1..255"; return false; }
        for (size_t i = 0; i < g_bindings.size(); ++i)
            if (g_bindings[i].slot == slot) { g_bindings.erase(g_bindings.begin() + (long)i); break; }
        const nvcoop::json::Value* n = v.find("netId");
        if (n && n->isNumber()) g_bindings.push_back({(uint8_t)slot, (uint32_t)n->asInt()});
    }
    out = "[";
    for (size_t i = 0; i < g_bindings.size(); ++i)
        out += (i ? "," : "") + std::string("{\"slot\":") + std::to_string(g_bindings[i].slot) + ",\"netId\":" + std::to_string(g_bindings[i].netId) + "}";
    out += "]";
    return true;
}

// ---- module -------------------------------------------------------------------------------------
uint8_t* g_rdCallRel = nullptr;   // the redirected call's rel32 in client.dll (friendly-fire patch below)
uint64_t g_rdBlocked = 0, g_rdPassed = 0;

void OnLoad(void*) {
    const std::string ini = IniPath();
    ConfigGetter get = [&](const char* key, const char* def) {
        char buf[512];
        GetPrivateProfileStringA("nvmp", key, def, buf, sizeof buf, ini.c_str());
        return std::string(buf);
    };
    std::vector<std::string> warns;
    Settings s = ParseSettings(get, &warns);
    char env[512];
    DWORD n = GetEnvironmentVariableA("NVCOOP_NVMP_PATCHES", env, sizeof env);
    if (n > 0 && n < sizeof env) {
        ConfigGetter eg = [&](const char* key, const char* def) { return std::string(key) == "patches" ? std::string(env) : get(key, def); };
        s = ParseSettings(eg, &warns);
    }
    for (auto& w : warns) nvcoop::Log("nvmp patches: config warning: %s", w.c_str());
    std::string names;
    for (auto& e : s.enabled) names += (names.empty() ? "" : ",") + e;
    nvcoop::Log("nvmp patches: enabled=[%s] autoApply=%d", names.c_str(), s.autoApply);

    g_patcher = std::make_unique<ClientPatcher>(
        std::move(s), g_mods, ProtectedWriter(), [](const char* type, const std::string& json) {
            nvcoop::PushEvent(type, json);
            nvcoop::Log("nvmp event %s %s", type, json.c_str());
        });
    // The loader-lock notification is OPT-IN ([nvmp] ldrNotify=1): polling every pollFrames frames is enough, and a
    // callback invoked on arbitrary loader threads is the riskiest thing this module could do inside the game.
    if (get("ldrNotify", "0") == "1") {
        if (HMODULE nt = GetModuleHandleA("ntdll.dll")) {
            if (auto reg = (LdrRegFn)GetProcAddress(nt, "LdrRegisterDllNotification")) reg(0, OnDllNotify, nullptr, &g_cookie);
        }
        nvcoop::Log("nvmp patches: LdrRegisterDllNotification enabled");
    }
    nvcoop::RegisterMethod("nvmp.patches", RpcPatches, true);
    nvcoop::RegisterMethod("nvmp.players", RpcPlayers, true);
    nvcoop::RegisterMethod("nvmp.bindslot", RpcBindSlot, true);
    nvcoop::RegisterMethod("nvmp.friendlyfire", [](const std::string&, std::string& out) {
        out = "{\"patched\":" + std::string(g_rdCallRel ? "true" : "false") + ",\"blocked\":" + std::to_string(g_rdBlocked) + ",\"passed\":" +
              std::to_string(g_rdPassed) + "}";
        return true;
    }, true);
}

// Last line of defence: an access violation while scanning/patching the foreign module must never take the game down.
// (Own function: __try cannot share a function with objects that need unwinding.)
bool GuardedFrame(ClientPatcher* p) {
    __try {
        p->OnFrame();
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool g_faulted = false;

// ---- friendly fire: NV:MP's received damage ------------------------------------------------------------------------------
// A player's hit on another player is sent by NV:MP itself and applied on the victim's machine ("[RD] applying %.3f health damage to
// %s from %s"), past the engine's hit function where nvcoop's hit gate stops it on the attacker's side. In that handler (9.43 RVA
// 0x3E935) NV:MP checks that attacker (ecx) and victim ([esp+0x14]) are alive, then calls the applier (RVA 0x3E4A0) with the event on
// the stack and attacker / victim in ecx / edx. That call is redirected to RdApplyThunk: damage from one player to another is dropped
// (unless [combat] friendlyfire=1), everything else goes on to NV:MP's applier untouched.
const char* const kRdApplySig =
    "8B 81 08 01 00 00 83 F8 02 74 ?? 83 F8 01 74 ?? 8B 54 24 14 8B 82 08 01 00 00 83 F8 02 74 ?? 83 F8 01 74 ?? 56 E8";
uint32_t g_rdOrig = 0;          // NV:MP's applier
int32_t g_rdOldRel = 0;
bool g_rdTried = false;

bool IsPlayerActorSEH(const void* actor) {
    __try {
        if (!actor) return false;
        if (actor == *reinterpret_cast<void* const*>(kThePlayerGlobal_1_4_0_525)) return true;
        return nvmp::IsRemotePlayerRef(*reinterpret_cast<const uint32_t*>(static_cast<const char*>(actor) + 0x0C));
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool __cdecl RdAllowed(const void* attacker, const void* victim) {
    if (attacker != victim && IsPlayerActorSEH(attacker) && IsPlayerActorSEH(victim)) {
        if (++g_rdBlocked <= 20) nvcoop::Log("nvmp: damage from player to player dropped (friendly fire is off)");
        return false;
    }
    ++g_rdPassed;
    return true;
}
__declspec(naked) void RdApplyThunk() {
    __asm {
        push ecx
        push edx
        push edx
        push ecx
        call RdAllowed
        add esp, 8
        pop edx
        pop ecx
        test al, al
        jz blocked
        jmp dword ptr [g_rdOrig]
    blocked:
        ret
    }
}
void InstallRdPatch() {
    g_rdTried = true;
    if (GetPrivateProfileIntA("combat", "friendlyfire", 0, IniPath().c_str()) != 0) { nvcoop::Log("nvmp: friendly fire on: received damage untouched"); return; }
    ModuleView mv;
    if (!g_mods.Find("client.dll", mv)) { g_rdTried = false; return; }   // not loaded yet: try again later
    const ScanResult r = ScanImage(mv.base, mv.size, kRdApplySig);
    if (!r.ok()) { nvcoop::Log("nvmp: friendly-fire patch: received-damage call not found (%s): players can still hurt each other", ToString(r.status)); return; }
    uint8_t* rel = mv.base + r.offset + 38;   // after the E8
    const int32_t old = *reinterpret_cast<int32_t*>(rel);
    const uint32_t target = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(rel + 4)) + static_cast<uint32_t>(old);
    const uint32_t thunk = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(&RdApplyThunk));
    DWORD prot = 0;
    if (!VirtualProtect(rel, 4, PAGE_EXECUTE_READWRITE, &prot)) return;
    g_rdOrig = target;
    g_rdOldRel = old;
    *reinterpret_cast<int32_t*>(rel) = static_cast<int32_t>(thunk - static_cast<uint32_t>(reinterpret_cast<uintptr_t>(rel + 4)));
    VirtualProtect(rel, 4, prot, &prot);
    FlushInstructionCache(GetCurrentProcess(), rel - 1, 5);
    g_rdCallRel = rel;
    nvcoop::Log("nvmp: friendly-fire patch on: client.dll+%X calls the received-damage filter (applier client.dll+%X)", (unsigned)(r.offset + 37),
                (unsigned)(target - static_cast<uint32_t>(reinterpret_cast<uintptr_t>(mv.base))));
}
void RemoveRdPatch() {
    if (!g_rdCallRel) return;
    DWORD prot = 0;
    if (VirtualProtect(g_rdCallRel, 4, PAGE_EXECUTE_READWRITE, &prot)) {
        *reinterpret_cast<int32_t*>(g_rdCallRel) = g_rdOldRel;
        VirtualProtect(g_rdCallRel, 4, prot, &prot);
        FlushInstructionCache(GetCurrentProcess(), g_rdCallRel - 1, 5);
    }
    g_rdCallRel = nullptr;
}
bool GuardedRdInstall() {
    __try {
        InstallRdPatch();
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

void OnFrame() {
    if (!g_rdTried && !GuardedRdInstall()) nvcoop::Log("nvmp: exception while installing the friendly-fire patch");
    if (!g_patcher || g_faulted) return;
    if (!GuardedFrame(g_patcher.get())) {
        g_faulted = true;   // never touch NV:MP again this session
        nvcoop::Log("nvmp patches: exception while checking/patching client.dll; module disabled");
        nvcoop::PushEvent("nvmp.fault", "{\"where\":\"OnFrame\"}");
    }
}

}  // namespace

// exposed for tests
ClientPatcher* ActivePatcher() { return g_patcher.get(); }

}  // namespace nvmp::client

namespace nvmp {

namespace {
const std::vector<uint32_t>& RemoteRefs();   // below: the other players' characters in NV:MP's list order
std::vector<uint8_t> g_guestSlots;   // the co-op slots of the guests connected right now, ascending (the host's session tells us)
}
void SetConnectedGuestSlots(const std::vector<uint8_t>& slots) { g_guestSlots = slots; }

uint32_t GhostRefOfPlayer(uint8_t coopSlot) {
    // A reconnecting guest gets a NEW co-op slot (2, 3, ...) while NV:MP lists only the characters there are now: slot k is the k-th
    // connected guest, matched to the k-th other character in NV:MP's list (both in join order). Measured 2026-10-28: one friend on slot 3
    // found no character by its slot number, and their conversations ran with the host's own player as the actor.
    {
        std::lock_guard<std::mutex> g(client::g_bindMu);
        bool bound = false;
        for (const auto& b : client::g_bindings) if (b.slot == coopSlot) bound = true;
        if (!bound) {
            for (size_t i = 0; i < g_guestSlots.size(); ++i) {
                if (g_guestSlots[i] != coopSlot) continue;
                const auto& rs = RemoteRefs();
                if (i < rs.size()) return rs[i];
                break;
            }
        }
    }
    std::vector<client::PlayerInfo> ps;
    std::string err;
    if (!client::Snapshot(ps, err)) return 0;
    std::lock_guard<std::mutex> g(client::g_bindMu);
    return client::SelectGhostRef(ps, coopSlot, client::g_bindings);
}

void BindSlotToNetId(uint8_t coopSlot, uint32_t netId) {
    if (coopSlot == 0) return;
    std::lock_guard<std::mutex> g(client::g_bindMu);
    for (size_t i = 0; i < client::g_bindings.size(); ++i)
        if (client::g_bindings[i].slot == coopSlot) { client::g_bindings.erase(client::g_bindings.begin() + (long)i); break; }
    if (netId) client::g_bindings.push_back({coopSlot, netId});
}

uint32_t NetIdByName(const std::string& name) {
    std::vector<client::PlayerInfo> ps;
    std::string err;
    if (!client::Snapshot(ps, err)) return 0;
    auto low = [](std::string x) { for (auto& c : x) if (c >= 'A' && c <= 'Z') c = (char)(c - 'A' + 'a'); return x; };
    const std::string want = low(name);
    for (const auto& p : ps)
        if (!p.isLocal && low(p.name) == want) return p.netId;
    return 0;
}

namespace {
std::vector<uint32_t> g_remoteRefs;   // other players' characters, in list order
ULONGLONG g_remoteAt = 0;
const std::vector<uint32_t>& RemoteRefs() {
    const ULONGLONG now = GetTickCount64();
    if (g_remoteAt && now - g_remoteAt < 500) return g_remoteRefs;
    g_remoteAt = now;
    g_remoteRefs.clear();
    std::vector<client::PlayerInfo> ps;
    std::string err;
    if (client::Snapshot(ps, err))
        for (const auto& p : ps)
            if (!p.isLocal && p.hasRef && p.refId) g_remoteRefs.push_back(p.refId);
    return g_remoteRefs;
}
}  // namespace

bool IsRemotePlayerRef(uint32_t ref) {
    if (!ref) return false;
    for (uint32_t r : RemoteRefs()) if (r == ref) return true;
    return false;
}

uint32_t FirstRemotePlayerRef() {
    const auto& rs = RemoteRefs();
    return rs.empty() ? 0 : rs.front();
}

namespace client {
NVCOOP_MODULE(nvmp_client_patches, "nvmp_client_patches", OnLoad, nullptr, OnFrame);

// Hot reload (T3b): our edits inside NV:MP's client.dll are reverted (the next core applies them again from its own config), the
// optional loader-lock DLL notification is unregistered (it points into this DLL), and the slot -> net id bindings are carried over.
typedef LONG(NTAPI* LdrUnregFn)(PVOID cookie);
NVCOOP_RELOAD_STATE(nvmp_client_patches, "nvmp_client_patches", 1, 70, nullptr,
    /*save*/ [](nvreload::Writer& w) {
        std::lock_guard<std::mutex> g(g_bindMu);
        w.u32(static_cast<uint32_t>(g_bindings.size()));
        for (const auto& b : g_bindings) { w.u8(b.slot); w.u32(b.netId); }
        return true;
    },
    /*restore*/ [](uint32_t, nvreload::Reader& r) {
        std::vector<SlotBindings> v;
        uint32_t n = r.u32();
        for (uint32_t i = 0; i < n && r.ok(); ++i) {
            SlotBindings b{};
            b.slot = r.u8();
            b.netId = r.u32();
            v.push_back(b);
        }
        if (!r.ok()) return false;
        std::lock_guard<std::mutex> g(g_bindMu);
        g_bindings = std::move(v);
        return true;
    },
    /*migrate*/ nullptr,
    /*shutdown*/ [] {
        if (g_cookie) {
            if (HMODULE nt = GetModuleHandleA("ntdll.dll"))
                if (auto unreg = (LdrUnregFn)GetProcAddress(nt, "LdrUnregisterDllNotification")) unreg(g_cookie);
            g_cookie = nullptr;
        }
        RemoveRdPatch();
        if (g_patcher) {
            int bad = g_patcher->RevertAll();
            if (bad) nvcoop::Log("nvmp patches: %d patch(es) could not be reverted for the reload", bad);
        }
        g_patcher.reset();
    },
    /*fresh*/ nullptr);
}  // namespace client

}  // namespace nvmp
