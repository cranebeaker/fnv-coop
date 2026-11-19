// nvcoop module (L10): starts the co-op Runtime inside the game and exposes the bridge methods.
//   nvcoop.ini  [coop] enable=1  role=host|client|guest  listen=127.0.0.1  host=127.0.0.1  port=27960  token=<n>
//   coop.status                      -> role/running/peers/last seq/txn counts (Runtime::statusJson)
//   coop.log   {n?:int}              -> recent txns (commit on host, apply on client, inject)
//   coop.inject {txn:{cause,origin,effects}} -> host: commit as if captured; client: apply as if received
// Starts lazily from onFrame once L5's hooks are initialised (nvhooks::hub() != null), so the module order of the
// DeferredInit message does not matter. All of it runs on the game main thread.
#include <Windows.h>

#include <algorithm>
#include <cstdlib>
#include <cstring>
#include <memory>
#include <unordered_map>
#include <map>
#include <set>
#include <string>
#include <vector>

#include "GameData.h"
#include "GameObjects.h"
#include "PluginAPI.h"
#include "apply_remote.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "reload/reload_state.h"
#include "baseline_file.h"
#include "coop_runtime.h"
#ifdef NVCOOP_WITH_FLIGHT
#include "flight/flight_service.h"
#include "common/alloc_count.h"
#endif
#include "effect_json.h"
#ifdef NVCOOP_WITH_UI
#include "hooks_ui/ui_hooks.h"
#include "info_reader.h"
#include "talk_subs.h"
#include "shared_xp.h"
#include "container_sync.h"
#include "player_stats.h"
#include "loot/loot_engine_real.h"
#include "GameTiles.h"
#endif
#include "engine_hooks.h"
#ifdef NVCOOP_WITH_NVMP
#include "nvmp/ghost_ref.h"
#endif
#ifdef NVCOOP_WITH_SNAPSHOT
#include "fnvcoop_snapshot/baseline_dump.h"
#endif
#ifdef NVCOOP_WITH_WORLD
#include "GameProcess.h"
#include "world_core.h"
#include "world_hooks.h"
#endif
#ifdef NVCOOP_WITH_ACTORS
#include "actors/actor_glue.h"
#endif
#ifdef NVCOOP_WITH_LIVE
#include "live/live_glue.h"   // dll-wiring: [live] switches, the hub (session / load lifecycle, channels), the lanes' services
#endif
#ifdef NVCOOP_WITH_COMPANIONS
#include "companions/companions_glue.h"   // companions, crime, kill XP ([companions] enabled, default off)
#endif
#ifdef NVCOOP_WITH_AREADIGEST
#include "areadigest/area_glue.h"         // per-area desync digests and heal ([areadigest] enabled, default off)
#endif

namespace {
using namespace nvcoop::coop;

bool g_scriptedLock = false;   // SetPlayerControls from the host's scripts (rules engine scope=all)

#ifdef NVCOOP_WITH_WORLD
// L12 world effects: SpawnRef / DespawnRef / RefScriptVar go through nvworld::WorldApplier (host-ref -> local-ref map, cell-not-ready queue,
// duplicate guard); the lane-local WorldEffects (NPC actor values, essential/restrained, relations, packages) ride Runtime::pushWorldEffect.
std::unique_ptr<nvworld::WorldApplier> g_worldApplier;

// Coalescing key: state-like effects replace the earlier push for the same subject, deltas accumulate (empty key).
std::string WorldEffectKey(const nvworld::WorldEffect& e) {
    auto r = [](const fnvcoop::FormRef& f) { return nvhooks::refToStr(f); };
    if (auto* x = std::get_if<nvworld::SetActorValueRef>(&e))
        return x->mode == nvworld::AvMode::Set ? "av:" + r(x->actor) + ":" + std::to_string(x->av) : std::string();
    if (auto* x = std::get_if<nvworld::SetEssential>(&e)) return "ess:" + r(x->actor) + (x->refOnly ? ":ref" : ":base");
    if (auto* x = std::get_if<nvworld::SetRestrained>(&e)) return "rst:" + r(x->actor);
    if (auto* x = std::get_if<nvworld::SetRelation>(&e)) return "rel:" + r(x->a) + ":" + r(x->b);
    if (auto* x = std::get_if<nvworld::SetScriptPackage>(&e)) return "pkg:" + r(x->actor) + ":" + r(x->package);
    return std::string();
}
#endif

class EngineWorld : public IWorldApplier {
public:
    ApplyResult apply(const fnvcoop::Effect& e, const fnvcoop::IEffectSink::Context& ctx) override {
#ifdef NVCOOP_WITH_LIVE
        nvcoop::livewire::glueOnEffect(e);   // the ownership service must know a story KillActor / ResurrectActor / CompanionOwner before the engine acts on it
#endif
#ifdef NVCOOP_WITH_WORLD
        if (nvworld::handlesEffect(e)) {
            if (!g_worldApplier) return {false, "NoWorldApplier"};
            nvhooks::ApplyStatus st = g_worldApplier->apply(e);
            ApplyResult r;
            r.ok = st == nvhooks::ApplyStatus::Ok;
            r.status = nvhooks::applyStatusName(st);
            if (!r.ok && !g_worldApplier->lastError().empty()) r.status += ": " + g_worldApplier->lastError();
            return r;
        }
#endif
        // session-level effects the engine layer realizes locally (no engine primitive involved)
        if (auto* sc = std::get_if<fnvcoop::SetPlayerControls>(&e)) {
            g_scriptedLock = !sc->enable;   // coarse v1: any disable locks all seven control classes (mask granularity: L5 TODO)
            return {true, "Ok"};
        }
        if (std::get_if<fnvcoop::SaveSuppress>(&e) || std::get_if<fnvcoop::PartyTether>(&e) || std::get_if<fnvcoop::BarrierRelease>(&e) ||
            std::get_if<fnvcoop::BarrierArrive>(&e))
            return {true, "Ok"};   // state lives in the shared shadow (coop.status.saveSuppress); nothing to write into the engine
        // a stash move is an event: during a join (history catch-up, snapshot, baseline) it must not run again, that would move the items twice
        if (std::get_if<fnvcoop::ContainerTransfer>(&e) && (ctx.catchUp || ctx.fromSnapshot || ctx.baseline)) return {true, "Ok"};
        if (!applier) return {false, "NoApplier"};
        // baseline effects FORCE-SET (may lower a stage, full objective flag byte); live txns merge normally
        nvhooks::ApplyStatus st = ctx.baseline ? applier->applyForce(e) : applier->apply(e);
        ApplyResult r;
        // NotForMe is a deliberate skip (another machine runs that actor's AI, a conversation belongs to its own player): handled
        // here, not a failure. It keeps its own name in the tally ("live RefCommand:NotForMe").
        r.ok = st == nvhooks::ApplyStatus::Ok || st == nvhooks::ApplyStatus::NotForMe;
        r.status = nvhooks::applyStatusName(st);
        if (!r.ok && !applier->lastError().empty()) r.status += ": " + applier->lastError();
        return r;
    }
    void onJoined(fnvcoop::PlayerId slot, const fnvcoop::SharePolicy&) override {
        nvcoop::Log("coop: joined as slot %d", (int)slot);
        nvcoop::PushEvent("coop.joined", "{\"slot\":" + std::to_string((int)slot) + "}");
    }
    nvhooks::RemoteApplier* applier = nullptr;
};

// Engine dumper for the baseline join: L7's DumpBaselineShadow (needs the snapshot library linked in).
class EngineDumper : public IWorldDumper {
public:
    bool dump(fnvcoop::ShadowState* out, uint64_t* hash, std::string* err) override {
#ifdef NVCOOP_WITH_SNAPSHOT
        LARGE_INTEGER f, a, b;
        QueryPerformanceFrequency(&f);
        QueryPerformanceCounter(&a);
        bool ok = fnvcoop::snapshot::DumpBaselineShadow(out, hash, err);
        QueryPerformanceCounter(&b);
        nvcoop::Log("coop: baseline dump %s in %.1f ms; %s", ok ? "ok" : "FAILED", double(b.QuadPart - a.QuadPart) * 1000.0 / double(f.QuadPart),
                    fnvcoop::snapshot::LastBaselineStats().summary.c_str());
        return ok;
#else
        (void)out; (void)hash;
        *err = "plugin built without the snapshot library (NVCOOP_LINK_SNAPSHOT=OFF)";
        return false;
#endif
    }
};
EngineDumper g_dumper;
bool g_gameLoaded = false;

Config g_cfg;
bool g_cfgLoaded = false;
std::unique_ptr<Runtime> g_rt;
std::unique_ptr<nvhooks::RemoteApplier> g_applier;
EngineWorld g_world;
bool g_failed = false;
std::string g_failMsg;
uint64_t g_waitFrames = 0;

// L5 pollHostState -> L4 StateReplicator (quest vars / globals are host-authoritative and never ride in capture txns)
class PollSink : public nvhooks::StateSink {
public:
    void scriptVar(const fnvcoop::FormRef& quest, uint16_t index, double value) override {
        if (g_rt && g_rt->replicator()) g_rt->replicator()->setScriptVar(quest, index, value);
    }
    void global(const fnvcoop::FormRef& g, float value) override {
        if (g_rt && g_rt->replicator()) g_rt->replicator()->setGlobal(g, value);
    }
};
PollSink g_pollSink;
// A sink that drops everything: the priming pass of a host without a world baseline (change_tracker.h).
class NullSink : public nvhooks::StateSink {
public:
    void scriptVar(const fnvcoop::FormRef&, uint16_t, double) override {}
    void global(const fnvcoop::FormRef&, float) override {}
};

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                       reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

std::string ReadWholeFile(const std::string& path) {
    HANDLE h = CreateFileA(path.c_str(), GENERIC_READ, FILE_SHARE_READ | FILE_SHARE_WRITE, nullptr, OPEN_EXISTING, 0, nullptr);
    if (h == INVALID_HANDLE_VALUE) return {};
    std::string s;
    char buf[4096];
    DWORD n = 0;
    while (ReadFile(h, buf, sizeof buf, &n, nullptr) && n) s.append(buf, n);
    CloseHandle(h);
    return s;
}

uint64_t NowMs() { return GetTickCount64(); }

std::string DllDir() {
    std::string p = IniPath();
    size_t s = p.find_last_of("\\/");
    return s == std::string::npos ? std::string() : p.substr(0, s + 1);
}

// nvcoop_edids.json {"VCG01":"falloutnv.esm:104c1c",...}, nvcoop_infoquest.json {"<info>":"<quest>"} (tools/gen_coop_data.py)
std::unordered_map<std::string, std::string> LoadStringMap(const std::string& path) {
    std::unordered_map<std::string, std::string> m;
    std::string text = ReadWholeFile(path);
    if (text.empty()) return m;
    nvcoop::json::Value v;
    std::string err;
    if (!nvcoop::json::parse(text, v, err)) { nvcoop::Log("coop: cannot parse %s: %s", path.c_str(), err.c_str()); return m; }
    for (auto& kv : v.members()) m[kv.first] = kv.second.asString("");
    return m;
}
std::unordered_map<std::string, std::string> g_edids, g_infoQuest;
// While alive: the host runs something for guest `slot`, so the menus it opens belong to that guest (hooks_ui beginActingFor).
struct ActingFor {
    explicit ActingFor(uint8_t slot) {
#ifdef NVCOOP_WITH_UI
        nvui::beginActingFor(slot);
#else
        (void)slot;
#endif
    }
    ~ActingFor() {
#ifdef NVCOOP_WITH_UI
        nvui::endActingFor();
#endif
    }
    ActingFor(const ActingFor&) = delete;
    ActingFor& operator=(const ActingFor&) = delete;
};
// A script the host runs for a guest (a scripted activator, a conversation line) says `player`, and here that is the host's own player: what
// it handed out reached the guest by origin AND stayed with the host (a guest's mutfruit pick gave both players the fruit, 2026-11-12).
// restore() puts the host's inventory back to what it was before the script ran (items it gained go, items it lost come back).
struct StandInInventory {
    nvcoop::coop::containers::Items before;
    bool ok = false;
    StandInInventory() {
        bool exact = true;
        ok = nvcoop::loot::readRefContents(0x14, before, exact) && exact;
    }
    // `picked`: the base of the object the guest activated. An OnActivate script that picks it up (Activate) put it in the HOST's pack, and
    // a pickup is no AddItem the capture sends: that one goes to the guest here (call with the intent's capture scope still open).
    void restore(const fnvcoop::FormRef& picked = fnvcoop::FormRef()) {
        nvcoop::coop::containers::Items now;
        bool exact = true;
        if (!ok || !nvcoop::loot::readRefContents(0x14, now, exact) || !exact) return;
        ok = false;
        const auto delta = nvcoop::coop::containers::diff(now, before);
        if (delta.empty()) return;
        // Nothing of this is shared: it only undoes the host's copy. The capture may still be open around us (the intent's own scope):
        // recorded, the RemoveItem went to the guest by origin too and took their fruit away again.
        nvhooks::CaptureHub* hub = nvhooks::hub();
        if (hub) hub->pushMute();
        {
            nvhooks::ApplyScope scope;
            for (const auto& kv : delta) {
                const uint32_t item = nvhooks::resolver().fromRef(kv.first);
                if (!item) continue;
                char line[96];
                std::snprintf(line, sizeof line, kv.second > 0 ? "AddItem %08X %d 1" : "RemoveItem %08X %d 1", item, kv.second > 0 ? kv.second : -kv.second);
                nvhooks::execConsoleLineOn(line, 0x14);
            }
        }
        if (hub) hub->popMute();
        auto it = picked.plugin.empty() ? delta.end() : delta.find(picked);
        if (hub && it != delta.end() && it->second < 0) {
            hub->record(fnvcoop::AddItem{picked, -it->second, fnvcoop::Recipient::dflt()});
            nvcoop::Log("coop: the guest's activation picked up %s here: %d sent to the guest", nvhooks::refToStr(picked).c_str(), -it->second);
        }
        nvcoop::Log("coop: a script run for a guest changed %u item kind(s) on this player: put back", (unsigned)delta.size());
    }
};
constexpr uint32_t kAvHealth = 16, kAvRadiationRads = 54, kAvVariable01 = 62;
float ActorValueSEH(uint32_t refId, uint32_t av);
void ExecMuted(nvhooks::CaptureHub& hub, const char* line, uint32_t ref);
// The guests' last SPECIAL / skills (player_stats.h), by slot, with when they came.
struct GuestStats { nvcoop::coop::pstats::Values values; uint64_t atMs = 0; };
std::map<uint8_t, GuestStats> g_guestStats;
uint64_t g_statSwaps = 0;
// While alive, this player's SPECIAL and skills read as guest `slot`'s (ModAV by the difference, taken back at the end), so a script run here
// for that guest checks THEIR skill (Megaton's bomb: `player.GetAV Explosives`). Nothing of it is captured or sent.
struct StatSwap {
    std::vector<std::pair<uint8_t, float>> done;
    nvhooks::CaptureHub* hub = nullptr;
    explicit StatSwap(uint8_t slot) {
        hub = nvhooks::hub();
        auto it = g_guestStats.find(slot);
        if (!hub || it == g_guestStats.end() || GetTickCount64() - it->second.atMs > 120000) return;
        for (const auto& kv : it->second.values) {
            const char* name = nvcoop::coop::pstats::avName(kv.first);
            const float mine = ActorValueSEH(0x14, kv.first);
            if (!name || mine < 0) continue;
            const float d = kv.second - mine;
            if (d > -0.5f && d < 0.5f) continue;
            char line[64];
            std::snprintf(line, sizeof line, "ModAV %s %.0f", name, d);
            ExecMuted(*hub, line, 0x14);
            done.push_back({kv.first, d});
        }
        if (!done.empty()) ++g_statSwaps;
    }
    ~StatSwap() {
        for (const auto& x : done) {
            char line[64];
            std::snprintf(line, sizeof line, "ModAV %s %.0f", nvcoop::coop::pstats::avName(x.first), -x.second);
            ExecMuted(*hub, line, 0x14);
        }
    }
    StatSwap(const StatSwap&) = delete;
    StatSwap& operator=(const StatSwap&) = delete;
};
// Actor value of a loaded actor (ActorValueOwner at +0xA4, GetActorValue float = vtable slot 3); -1 = not an actor / not loaded.
float ActorValueSEH(uint32_t refId, uint32_t av) {
    __try {
        auto* r = static_cast<TESObjectREFR*>(nvhooks::resolver().lookup(refId));
        if (!r) return -1;
        const uint8_t t = *reinterpret_cast<const uint8_t*>(reinterpret_cast<const char*>(r) + 4);
        if (t != kFormType_Character && t != kFormType_Creature) return -1;
        void* owner = reinterpret_cast<char*>(r) + 0xA4;
        auto fn = (*reinterpret_cast<float(__thiscall***)(void*, uint32_t)>(owner))[3];
        return fn(owner, av);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}
void ExecMuted(nvhooks::CaptureHub& hub, const char* line, uint32_t ref) {   // this machine only: not captured, not gated
    hub.pushMute();
    {
        nvhooks::ApplyScope scope;
        nvhooks::execConsoleLineOn(line, ref);
    }
    hub.popMute();
}
// Doctors (DoctorTemplateSCRIPT, Doc Church and the other Capital Wasteland doctors under TTW): the paid reply sets the doctor's Variable01
// to 1 (heal) or 2 (addictions); the doctor's own script then locks the controls, fades to black and sets VDoctors stage 10 / 20, whose
// result heals `player`. Run for a guest, all of that happened to the HOST: the host was healed and the guest never was (2026-10-28 list).
// Here, inside the capture scope of that reply, the doctor goes straight back to idle (Variable01 0, for every copy) and the stage runs as
// the guest's action, so its ResetHealth / addiction cure reaches the guest by origin. The host's own health is put back to what it was
// (the stage result heals this player as well).
void TreatGuestPatient(nvhooks::CaptureHub& hub, uint8_t slot, uint32_t npc, float before) {
    const float v = ActorValueSEH(npc, kAvVariable01);
    if (v == before || (v != 1 && v != 2)) return;
    const int stage = v == 1 ? 10 : 20;
    const float hp = ActorValueSEH(0x14, kAvHealth);
    char line[64];
    std::snprintf(line, sizeof line, "SetStage VDoctors %d", stage);
    bool ok = false;
    {
        ActingFor acting(slot);
        nvhooks::execConsoleLineOn("SetAV Variable01 0", npc);
        ok = nvhooks::execConsoleLineOn(line, npc);
    }
    const float healed = ActorValueSEH(0x14, kAvHealth);
    if (hp > 0 && healed > hp + 0.5f) {
        char back[64];
        std::snprintf(back, sizeof back, "DamageAV Health %.1f", healed - hp);
        ExecMuted(hub, back, 0x14);
    }
    nvcoop::Log("coop: doctor %08X treats guest slot %d: '%s' as their action: %s", npc, (int)slot, line, ok ? "issued" : "FAILED");
}
// slot -> the guest's ghost refID on the host (L6 nvmp::GhostRefOfPlayer); 0 = unknown / NV:MP not present => stand-in player (ACTING-STANDIN)
uint32_t GhostOf(fnvcoop::PlayerId slot) {
#ifdef NVCOOP_WITH_LIVE
    if (slot != fnvcoop::kHostPlayer)
        if (uint32_t puppet = nvcoop::livewire::glueAvatarRef(static_cast<uint8_t>(slot))) return puppet;   // our own avatar layer first ([live] avatars=1)
#endif
#ifdef NVCOOP_WITH_NVMP
    return slot == fnvcoop::kHostPlayer ? 0u : nvmp::GhostRefOfPlayer(static_cast<uint8_t>(slot));
#else
    (void)slot;
    return 0u;
#endif
}
fnvcoop::FormRef g_passQuest;      // [coop] scenepass=<EDID>:<stage>: local player counts as "finished" when the quest reaches the stage
int g_passStage = -1;
bool g_passSent = false;
bool g_rejectLogged = false;

// Build identity for the join handshake: hash of this very dll, its PE link timestamp as the build number, and the mod load order.
uint64_t Fnv64File(const std::string& path) {
    std::string b = ReadWholeFile(path);
    uint64_t h = 1469598103934665603ull;
    for (unsigned char c : b) { h ^= c; h *= 1099511628211ull; }
    return h;
}

fnvcoop::HandshakeInfo MakeHandshake() {
    fnvcoop::HandshakeInfo hs;
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&MakeHandshake), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    hs.buildHash = Fnv64File(buf);
    auto* dos = reinterpret_cast<const IMAGE_DOS_HEADER*>(self);
    auto* nt = reinterpret_cast<const IMAGE_NT_HEADERS*>(reinterpret_cast<const char*>(self) + dos->e_lfanew);
    hs.buildNumber = nt->FileHeader.TimeDateStamp;    // link time: monotonically increasing per build
    hs.buildName = std::string("nvcoop ") + __DATE__ + " " + __TIME__;
    if (DataHandler* dh = DataHandler::Get()) {
        UInt32 n = dh->GetActiveModCount();
        for (UInt32 i = 0; i < 255 && i < n + 8; ++i) {
            const char* name = dh->GetNthModName(i);
            if (name && *name) hs.plugins.push_back(name);
        }
    }
    return hs;
}

#ifdef NVCOOP_WITH_WORLD
bool Has3DSEH(uint32_t id);   // the actor's 3D is loaded (defined with the death watch below)
void QueueDoorUse(uint32_t refID, bool open);   // the door channel (TickDoorSync)
uint32_t BaseOfRefSEH(uint32_t id);   // the ref's base form id (0 = none)
#else
inline bool Has3DSEH(uint32_t) { return false; }
inline uint32_t BaseOfRefSEH(uint32_t) { return 0; }
#endif
bool g_joinedSelf = false;   // LoadConfig turned a "Join" to this PC's own address into hosting (TickRespawnRestore tells the player)
// True when `host` is a dotted IPv4 address of THIS machine other than loopback (Radmin / Hamachi / LAN). The test copies join 127.0.0.1 on
// one PC on purpose; nobody joins their own Radmin address on purpose.
bool IsOwnNonLoopbackAddress(const std::string& host) {
    unsigned a = 0, b = 0, c = 0, d = 0;
    char tail = 0;
    if (std::sscanf(host.c_str(), "%u.%u.%u.%u%c", &a, &b, &c, &d, &tail) != 4 || a > 255 || b > 255 || c > 255 || d > 255 || a == 127) return false;
    const uint32_t want = a | b << 8 | c << 16 | d << 24;   // network byte order, as the table keeps it
    struct Row { DWORD addr, index, mask, bcast, reasm; unsigned short unused1, type; };
    struct Table { DWORD n; Row rows[1]; };
    HMODULE ip = LoadLibraryA("iphlpapi.dll");
    if (!ip) return false;
    auto getTable = reinterpret_cast<DWORD(WINAPI*)(void*, ULONG*, BOOL)>(GetProcAddress(ip, "GetIpAddrTable"));
    bool own = false;
    ULONG size = 0;
    if (getTable && getTable(nullptr, &size, FALSE) == ERROR_INSUFFICIENT_BUFFER && size > 0 && size < (1u << 20)) {
        std::vector<uint8_t> buf(size);
        if (getTable(buf.data(), &size, FALSE) == NO_ERROR) {
            const Table* t = reinterpret_cast<const Table*>(buf.data());
            for (DWORD i = 0; i < t->n && !own; ++i) own = t->rows[i].addr == want;
        }
    }
    FreeLibrary(ip);
    return own;
}

void LoadConfig() {
    if (g_cfgLoaded) return;
    g_cfgLoaded = true;
    g_cfg.handshake = true;   // real installs check build / rules / volatile / mod list on join; ini handshake=0 for fake-peer tests
    std::string err;
    if (!ParseConfig(ReadWholeFile(IniPath()), &g_cfg, &err)) {
        g_failed = true;
        g_failMsg = "nvcoop.ini [coop]: " + err;
        nvcoop::Log("coop: %s", g_failMsg.c_str());
        return;
    }
    // The NV:MP launcher (TTW-CoOp.cmd) says who hosts: NVCOOP_ROLE=host|client, NVCOOP_HOST=<host address>. It overrides the ini.
    char env[256];
    DWORD en = GetEnvironmentVariableA("NVCOOP_ROLE", env, sizeof env);
    if (en > 0 && en < sizeof env) {
        const std::string r = env;
        if (r == "host") { g_cfg.role = Role::Host; g_cfg.enable = true; if (g_cfg.listen == "127.0.0.1") g_cfg.listen = "0.0.0.0"; }
        else if (r == "client" || r == "join") { g_cfg.role = Role::Client; g_cfg.enable = true; }
        en = GetEnvironmentVariableA("NVCOOP_HOST", env, sizeof env);
        if (g_cfg.role == Role::Client && en > 0 && en < sizeof env) g_cfg.host = env;
        nvcoop::Log("coop: role from the launcher: %s", r.c_str());
    }
    // "Join" pointed at this PC's own address: the player runs the NV:MP server here and meant to host. As a guest nobody ran the story (quest
    // scripts run on the host only): Galaxy News Radio's battle never went on (2026-11-13, the host's game started with host=its own Radmin
    // address). This game hosts instead.
    if (g_cfg.role == Role::Client && IsOwnNonLoopbackAddress(g_cfg.host)) {
        nvcoop::Log("coop: told to join %s, which is THIS PC: hosting instead", g_cfg.host.c_str());
        g_cfg.role = Role::Host;
        g_cfg.listen = "0.0.0.0";
        g_joinedSelf = true;
        // the hooks / world / UI layers read the role from the launcher's NVCOOP_ROLE (or the ini) themselves, after this: tell them too.
        // Without it they ran as a GUEST in the host's game (2026-11-14 Megaton: the host blocked its own result scripts, barter and the
        // other player's dialogue failed).
        SetEnvironmentVariableA("NVCOOP_ROLE", "host");
    }
    nvcoop::Log("coop: enable=%d role=%s listen=%s host=%s port=%u", g_cfg.enable ? 1 : 0, roleName(g_cfg.role), g_cfg.listen.c_str(),
                g_cfg.host.c_str(), (unsigned)g_cfg.port);
}

void ParseScenePass() {
    char buf[128] = {};
    GetPrivateProfileStringA("coop", "scenepass", "", buf, sizeof buf, IniPath().c_str());
    std::string v = buf;
    size_t c = v.find(':');
    if (v.empty() || c == std::string::npos) return;
    auto it = g_edids.find(v.substr(0, c));
    if (it == g_edids.end()) {   // clients do not load the host rules; load the EDID map just for this
        g_edids = LoadStringMap(DllDir() + "nvcoop_edids.json");
        it = g_edids.find(v.substr(0, c));
    }
    if (it != g_edids.end() && nvhooks::strToRef(it->second, &g_passQuest)) {
        g_passStage = std::atoi(v.c_str() + c + 1);
        nvcoop::Log("coop: scenepass trigger %s stage >= %d", it->second.c_str(), g_passStage);
    }
}

void TryStart() {
    if (g_rt || g_failed || !g_cfg.enable || g_cfg.role == Role::Off) return;
    if (g_cfg.baseline && !g_gameLoaded) return;   // the dump needs a loaded save
    // A guest joins only once it has a world of its own (a loaded save or a new game). At the main menu the engine still runs the menu's
    // stand-in world: applying the host's quests there started their scripts in it (a host's TTW new game put a guest at the main menu
    // into New Vegas character creation with its main thread stuck).
    if (g_cfg.role == Role::Client && !g_gameLoaded) {
        static bool logged = false;
        if (!logged) { logged = true; nvcoop::Log("coop: guest waits for a loaded save or a new game before joining"); }
        return;
    }
    nvhooks::CaptureHub* hub = nvhooks::hub();
    if (!hub) {  // hooks not initialised yet (L5 inits at DeferredInit); wait
        ++g_waitFrames;
        return;
    }
    g_applier.reset(new nvhooks::RemoteApplier(*hub, nvhooks::resolver()));
    g_world.applier = g_applier.get();
#ifdef NVCOOP_WITH_WORLD
    g_worldApplier.reset(new nvworld::WorldApplier(*hub, nvhooks::resolver()));
    g_worldApplier->setExecLine([](const std::string& line, uint32_t refID) { return nvhooks::execConsoleLineOn(line, refID); });
    nvhooks::setDoorStateQuery([](uint32_t refID) {
        double v = 0;
        return nvworld::queryCondition("GetOpenState", refID, 0, &v) ? static_cast<int>(v) : 0;
    });
#ifdef NVCOOP_WITH_LIVE
    // spawn arbitration makes this machine's own copy of a spawn it did not roll through the same applier as every other SpawnRef
    nvcoop::livewire::glueSetSpawnApplier([](const fnvcoop::SpawnRef& s) {
        return g_worldApplier && g_worldApplier->apply(fnvcoop::Effect(s)) == nvhooks::ApplyStatus::Ok;
    });
#endif
#endif
    g_rt.reset(new Runtime(g_cfg, hub, &g_world, NowMs));
    g_rt->setDumper(&g_dumper);
    g_rt->setQuestStateReader([](const fnvcoop::FormRef& q, int32_t& stage, bool& completed, bool& failed) {
        const int st = nvhooks::localQuestStage(q), fl = nvhooks::localQuestFlags(q);
        if (st < 0 || fl < 0) return false;
        stage = st;
        completed = (fl & 0x02) != 0;   // a failed quest also has the completed bit
        failed = (fl & 0x40) != 0;
        return true;
    });
    g_rt->setQuestRunningReader([](const fnvcoop::FormRef& q, bool& running) {
        const int fl = nvhooks::localQuestFlags(q);
        if (fl < 0) return false;
        running = (fl & 0x01) != 0;   // TESQuest flags bit 0: running (snapshot_nvse.cpp reads the same bit)
        return true;
    });
#ifdef NVCOOP_WITH_FLIGHT
    if (auto* fr = nvcoop::flight::service().recorder()) g_rt->setFlight(fr);   // T1a: record every datagram and applied / committed effect
#endif
    if (g_cfg.handshake) {
        g_rt->setHandshakeBase(MakeHandshake());
        if (g_cfg.rulesFile.empty()) {   // clients hash the same rules.json the host loads
            std::string def = DllDir() + "nvcoop_rules.json";
            if (!ReadWholeFile(def).empty()) { g_cfg.rulesFile = def; g_rt->setRulesFile(def); }
        }
    }
    ParseScenePass();
    if (g_cfg.role == Role::Host) {
        // Quest vars / globals are host-authoritative and polled every frame, with or without a world baseline (the old gate polled only
        // with baseline=1, so a baseline-less host never shared a var an object script wrote, e.g. VMS16.nGangerDeathCount from the gangers' OnDeath).
        g_rt->setStatePoll([](fnvcoop::StateReplicator& to, bool first, bool prime) {
            (void)to;
            if (first) {
                nvhooks::enableStatePolling(true, prime);   // clears the tracker; prime = take the present values as known
                if (prime) { NullSink drop; nvhooks::pollHostState(drop); nvcoop::Log("coop: state poll primed (no world baseline)"); return; }
            }
            nvhooks::pollHostState(g_pollSink);
        });
        // quest rules engine: rules.json (ini rules=, default nvcoop_rules.json next to the dll) + EDID / INFO->quest maps
        if (g_cfg.rulesFile.empty()) {
            std::string def = DllDir() + "nvcoop_rules.json";
            if (!ReadWholeFile(def).empty()) g_cfg.rulesFile = def;
        }
        if (!g_cfg.rulesFile.empty()) {
            g_rt->setRulesFile(g_cfg.rulesFile);
            g_edids = LoadStringMap(DllDir() + "nvcoop_edids.json");
            g_infoQuest = LoadStringMap(DllDir() + "nvcoop_infoquest.json");
            nvcoop::Log("coop: rules=%s edids=%u infoquest=%u", g_cfg.rulesFile.c_str(), (unsigned)g_edids.size(), (unsigned)g_infoQuest.size());
            g_rt->setEdidResolver([](const std::string& edid, fnvcoop::FormRef& out) {
                auto it = g_edids.find(edid);
                return it != g_edids.end() && nvhooks::strToRef(it->second, &out);
            });
            g_rt->setInfoQuestResolver([](const fnvcoop::FormRef& info, fnvcoop::FormRef& q) {
                auto it = g_infoQuest.find(nvhooks::refToStr(info));
                return it != g_infoQuest.end() && nvhooks::strToRef(it->second, &q);
            });
            g_rt->setStageApplier([](const fnvcoop::FormRef& q, uint32_t st) {
                // re-issue the deferred stage as the engine would have: scripts run, capture opens (host hub), gate bypassed
                uint32_t id = nvhooks::resolver().fromRef(q);
                char line[64];
                std::snprintf(line, sizeof line, "setstage %08X %u", id, st);
                nvhooks::StageGateBypass bypass;
                bool ok = id != 0 && nvhooks::execConsoleLine(line);
                nvcoop::Log("coop: rules released stage %s=%u -> '%s' %s", nvhooks::refToStr(q).c_str(), st, line, ok ? "issued" : "FAILED");
                nvcoop::PushEvent("coop.stageReleased", std::string("{\"quest\":\"") + nvhooks::refToStr(q) + "\",\"stage\":" + std::to_string(st) +
                                                            ",\"issued\":" + (ok ? "true" : "false") + "}");
            });
            nvhooks::setStageGate([](const fnvcoop::FormRef& q, uint32_t st) { return g_rt ? g_rt->stageRequested(q, st, 0) : true; });
        }
    }
    g_rt->setActionExecutor([](fnvcoop::PlayerId slot, fnvcoop::IntentKind kind, const fnvcoop::FormRef& ref) {
        // Guest interaction (docs/protocol.md section 12): the host runs the REAL activation of `ref` inside a capture scope whose
        // origin the runtime forces to `slot`, so rewards / karma / XP route to the guest and stage changes reach everybody.
        nvhooks::CaptureHub* hub = nvhooks::hub();
        uint32_t target = nvhooks::resolver().fromRef(ref);
        if (kind == fnvcoop::IntentKind::ReportKill) {   // a guest's player killed it there. The shared world records the death whether or not the
            fnvcoop::Cause c;                              // host has the actor loaded; every machine then kills it whenever it is loaded and alive there
            c.kind = fnvcoop::CauseKind::Combat;           // (EnforceKills: also when NV:MP brings it back, and when a player walks in later)
            const uint64_t seq = g_rt->commitHostEffects(c, {fnvcoop::KillActor{ref}});
            bool ok = false;
            if (target && Has3DSEH(target)) {
                nvhooks::ApplyScope scope;
                ok = nvhooks::execConsoleLineOn("KillActor", target);
            }
            nvcoop::Log("coop: guest slot %d killed %s -> KillActor seq %llu, here: %s", (int)slot, nvhooks::refToStr(ref).c_str(), (unsigned long long)seq,
                        !target ? "not loaded" : ok ? "killed" : "not killed yet");
            return;
        }
        if (!hub || !target) { nvcoop::Log("coop: guest action %d on %s from slot %d: target not found", (int)kind, nvhooks::refToStr(ref).c_str(), (int)slot); return; }
        if (kind == fnvcoop::IntentKind::TriggerEnter) {
            nvcoop::Log("coop: guest TriggerEnter on %s from slot %d: not drivable yet (needs the L5 event-dispatch hook)", nvhooks::refToStr(ref).c_str(), (int)slot);
            return;
        }
#ifdef NVCOOP_WITH_COMPANIONS
        if (kind == fnvcoop::IntentKind::ActivateRef && !nvcoop::social::glueAllowActivation(static_cast<uint8_t>(slot), ref)) {   // somebody else's companion
            nvcoop::Log("coop: guest ActivateRef on %s from slot %d refused: another player's companion", nvhooks::refToStr(ref).c_str(), (int)slot);
            return;
        }
#endif
        // v1 acting player = the host's own player (0x14) so `IsActionRef player` / `GetDistance player` pass; the guest's ghost ref
        // (L6) replaces it once available. Effects are attributed to the guest by origin (known limitation: the host's player
        // really receives the item too).
        uint32_t actor = GhostOf(slot);
        static std::unordered_map<int, bool> logged;   // log the mode once per slot and mode change
        bool standIn = actor == 0;
        if (standIn) actor = 0x14;
        if (!logged.count((int)slot) || logged[(int)slot] != standIn) {
            logged[(int)slot] = standIn;
            const char* why = "no NV:MP ghost";
#ifdef NVCOOP_WITH_LIVE
            why = nvcoop::livewire::glueSwitches().avatars ? "[live] avatars=1 but this slot's puppet does not exist yet, and no NV:MP ghost"
                                                           : "[live] avatars=0, so there are no puppets, and no NV:MP ghost";
#endif
            if (standIn) nvcoop::Log("coop: ACTING-STANDIN slot %d: %s; host player 14 acts", (int)slot, why);
            else nvcoop::Log("coop: acting player for slot %d = puppet / ghost ref %08X", (int)slot, actor);
        }
        StatSwap stats(static_cast<uint8_t>(slot));   // ... and its skill checks read the guest's SPECIAL / skills
        StandInInventory keep;   // a script's `player` is this machine's player: what it gave or took for the guest is put back when it ran
        if (kind == fnvcoop::IntentKind::DialogueResult) {   // the guest's own conversation picked INFO `ref`: its result script runs here, captured
            const fnvcoop::Cause why = g_rt->actionCause();      // source = the speaker, aux = 0 begin / 1 end
            const uint32_t npc = nvhooks::resolver().fromRef(why.source);
            bool ok = false;
            const float service = npc ? ActorValueSEH(npc, kAvVariable01) : -1;
            const float rads = ActorValueSEH(0x14, kAvRadiationRads);   // a doctor's radiation cure (Doctors stage 30) also cures `player` here
            {
                // One txn for the line AND what TreatGuestPatient adds: a guest applies it in one go, so its copy of a doctor never runs a frame
                // with the treatment flag set and nothing to clear it (its script looped: controls off, ResetAI, "in combat" for good).
                fnvcoop::Cause dc;
                dc.kind = fnvcoop::CauseKind::DialogueResult;
                dc.aux = why.aux;
                dc.source = ref;
                nvhooks::ScopedCapture scope(*hub, dc);
                {
                    ActingFor acting(static_cast<uint8_t>(slot));   // its menus (trade, repair, message boxes) open on the guest's screen
                    ok = nvhooks::runInfoResultFor(actor, static_cast<uint8_t>(slot), target, npc, why.aux != 0);
                }
                keep.restore();
                if (npc) TreatGuestPatient(*hub, static_cast<uint8_t>(slot), npc, service);
                // the cure itself went to the guest (RestoreActorValue is a RefCommand for the acting player); this player keeps their rads
                const float radsNow = ActorValueSEH(0x14, kAvRadiationRads);
                if (rads > 0 && radsNow >= 0 && radsNow < rads - 0.5f) {
                    char back[64];
                    std::snprintf(back, sizeof back, "DamageAV RadiationRads %.1f", rads - radsNow);
                    ExecMuted(*hub, back, 0x14);
                    nvcoop::Log("coop: guest slot %d's cure: this player's %.0f rads put back", (int)slot, rads - radsNow);
                }
            }
            nvcoop::Log("coop: guest slot %d dialogue %s (%s) with %s as %08X: %s", (int)slot, nvhooks::refToStr(ref).c_str(), why.aux ? "end" : "begin",
                        nvhooks::refToStr(why.source).c_str(), actor, ok ? "ran" : "FAILED");
            return;
        }
        fnvcoop::Cause c;
        c.kind = fnvcoop::CauseKind::Intent;
        c.source = ref;
        c.aux = static_cast<int32_t>(kind);
        // A scripted activator's OnActivate usually checks `IsActionRef player`: activated by the guest's NV:MP character it did nothing.
        // The host's player acts; what the script gives goes to the guest by origin, its message boxes to the guest's screen (ActingFor).
        if (kind == fnvcoop::IntentKind::ActivateRef && nvhooks::isScriptedActivator(target)) actor = 0x14;
        // A guest's door: opened here by this player, so the door's script takes it as the player's (Tenpenny's doors close behind NPCs)
        // and NV:MP carries the open door to the guest.
        if (kind == fnvcoop::IntentKind::ActivateRef && nvhooks::isForwardableDoor(target)) actor = 0x14;
        // Locked here but not for the guest, who only sends unlocked doors: they picked the lock (or had the key) on their copy. Unlocked
        // for everybody first (captured), else the host's player got the lockpicking screen for the guest's door.
        const bool unlockFirst = kind == fnvcoop::IntentKind::ActivateRef && nvhooks::isLockedPlainDoor(target);
        if (unlockFirst) actor = 0x14;
        char line[64];
        std::snprintf(line, sizeof line, "Activate %08X 1", actor);
        bool ok = false;
        fnvcoop::FormRef picked;
        if (kind == fnvcoop::IntentKind::ActivateRef && actor == 0x14) nvhooks::resolver().toRef(BaseOfRefSEH(target), &picked);
        {
            nvhooks::ScopedCapture scope(*hub, c);
            {
                ActingFor acting(static_cast<uint8_t>(slot));
                if (unlockFirst) nvhooks::execConsoleLineOn("Unlock", target);
                ok = nvhooks::execConsoleLineOn(line, target);
                if (unlockFirst || nvhooks::isForwardableDoor(target)) nvhooks::holdDoorForGuest(target);
            }
            keep.restore(picked);
        }
        nvcoop::Log("coop: guest action %d on %s from slot %d -> '%s' as %08X: %s", (int)kind, nvhooks::refToStr(ref).c_str(), (int)slot, line, actor, ok ? "issued" : "FAILED");
        nvcoop::PushEvent("coop.guestAction", std::string("{\"slot\":") + std::to_string((int)slot) + ",\"ref\":\"" + nvhooks::refToStr(ref) + "\",\"issued\":" + (ok ? "true" : "false") + "}");
#ifdef NVCOOP_WITH_COMPANIONS
        if (ok) nvcoop::social::glueGuestInteraction(static_cast<uint8_t>(slot), kind == fnvcoop::IntentKind::PickUp, ref);   // theft / trespass / break-in by the guest
#endif
    });
    nvhooks::setDoorUseObserver([](uint32_t refID, bool open) { QueueDoorUse(refID, open); });   // both roles (TickDoorSync)
    if (g_cfg.role == Role::Client && (g_cfg.forwardActivate || g_cfg.forwardScripted)) {
        nvhooks::setActivateObserver([](const fnvcoop::FormRef& target) {
            if (!g_rt || g_rt->syncing()) return nvhooks::ActivateAction::Proceed;   // (gameplay is locked while syncing anyway)
            // forward_scripted (default on): an activator whose OnActivate script does the work (Megaton's water pipes: a repair check and a
            // quest step) did nothing on a guest, whose gate drops the script's shared effects. The host runs it for this player instead:
            // its message boxes come to this screen (hooks_ui beginActingFor), its quest steps reach everybody. Doors, containers, pickups
            // and NPCs stay local as before.
            const uint32_t tid = nvhooks::resolver().fromRef(target);
            // Doors stay local: where they end up goes to the others through the door channel (TickDoorSync).
            if (!g_cfg.forwardActivate && !nvhooks::isScriptedActivator(tid)) return nvhooks::ActivateAction::Proceed;
            const bool sent = g_rt->submitAction(fnvcoop::IntentKind::ActivateRef, target) != 0;
            nvcoop::Log("coop: this player activated %s -> host runs it%s", nvhooks::refToStr(target).c_str(), sent ? "" : " (NOT sent: activated here)");
            return sent ? nvhooks::ActivateAction::Suppress : nvhooks::ActivateAction::Proceed;
        });
    }
    if (g_cfg.role == Role::Client) {   // this player's own conversations count: the host runs their INFO result scripts
        nvhooks::setDialogueResultObserver([](const fnvcoop::FormRef& info, const fnvcoop::FormRef& npc, bool onEnd) {
            if (!g_rt) return;
            fnvcoop::Cause c;
            c.kind = fnvcoop::CauseKind::DialogueResult;
            c.source = npc;
            c.aux = onEnd ? 1 : 0;
            const uint32_t id = g_rt->submitAction(fnvcoop::IntentKind::DialogueResult, info, c);
            nvcoop::Log("coop: own dialogue %s (%s) with %s -> host intent %u", nvhooks::refToStr(info).c_str(), onEnd ? "end" : "begin",
                        nvhooks::refToStr(npc).c_str(), id);
        });
    }
    if (g_cfg.dialogue) {
        DialogueHooks h;
        // TODO(L5/L8): run the INFO result script under capture; 0 = nothing committed
        h.run = [](const fnvcoop_dlg::ResultRequest&) -> uint64_t { return 0; };
        h.sink = [](const fnvcoop_dlg::Event& e) {
            nvcoop::PushEvent("dlg.event", std::string("{\"kind\":") + std::to_string((int)e.kind) + "}");
#ifdef NVCOOP_WITH_UI
            // A box / barter / repair menu the host raised for THIS player (a script it ran for us): open it here; the answer goes back
            // through ops::setAnswerSink below. Without this the guest never saw them (pipes' repair check, ShowBarterMenu, 2026-11-11).
            using K = fnvcoop_dlg::Event::Kind;
            if (e.kind == K::MenuShow) {
                const auto& rq = std::get<fnvcoop_dlg::MenuRequest>(e.msg);
                const bool ok = nvui::ops::openMenu(rq);
                nvcoop::Log("coop: host menu %u kind %d for this player, %u button(s), \"%.60s\" -> %s", rq.menu, int(rq.kind), (unsigned)rq.buttons.size(),
                            rq.text.c_str(), ok ? "shown" : "FAILED");
            } else if (e.kind == K::MenuEnd && e.timedOut) {
                nvui::ops::closeMenu(std::get<fnvcoop_dlg::MenuClosed>(e.msg).menu);
            }
#endif
        };
        g_rt->enableDialogue(h);
#ifdef NVCOOP_WITH_UI
        nvui::ops::setAnswerSink([](uint32_t menu, int32_t choice) {
            if (!g_rt || !g_rt->dialogue()) return;
            g_rt->dialogue()->menuReply(menu, choice, static_cast<uint32_t>(NowMs()));
            nvcoop::Log("coop: answered host menu %u with %d", menu, choice);
        });
#endif
    }
#ifdef NVCOOP_WITH_UI
    {   // L11 wiring (internal lane notes): router bridge, 0xE5 slideshow channel, SlideshowEnd, INFO reader
        struct ProcMem : fndlg::IMemory {
            bool read(fndlg::Addr32 a, void* out, size_t n) const override {
                SIZE_T got = 0;
                return a >= 0x10000 && ReadProcessMemory(GetCurrentProcess(), reinterpret_cast<LPCVOID>(uintptr_t(a)), out, n, &got) && got == n;
            }
        };
        static ProcMem mem;
        static fndlg::ResponseLayout layout;   // headOffset -1 until probeResponseOffset finds it: the reader then returns no lines
        UiWiring w;
        w.router = []() { return nvui::router(); };
        w.slideHost = [](nvui::SlideHost::Send s) -> nvui::SlideHost& { return nvui::slideHost(std::move(s)); };
        w.slideClient = []() -> nvui::SlideClient& { return nvui::slideClient(); };
        w.consentFile = DllDir() + "nvcoop_consent_messages.txt";
        w.installInfoReader = [](std::function<std::vector<fnvcoop_dlg::Line>(uint32_t)> f) { nvui::setInfoReader(std::move(f)); };
        w.infoReader = fndlg::makeInfoReader(mem, &layout);
        g_rt->enableUi(std::move(w));
    }
#endif
    g_rt->bindPlayerRefs([](fnvcoop::PlayerId s) { return GhostOf(s); },
                         [](uint32_t id, fnvcoop::FormRef& out) { return nvhooks::resolver().toRef(id, &out); });   // PartyWarp needs this
#ifdef NVCOOP_WITH_WORLD
    if (g_cfg.role == Role::Client)
        g_rt->setWorldEffectApplier([](const std::string& json) -> std::string {
            nvworld::WorldEffect we;
            std::string werr;
            if (!g_worldApplier) return "NoWorldApplier";
            if (!nvworld::worldEffectFromJson(json, &we, &werr)) return "BadWorldEffect: " + werr;
            nvhooks::ApplyStatus st = g_worldApplier->applyWorld(we);
            std::string r = nvhooks::applyStatusName(st);
            if (st != nvhooks::ApplyStatus::Ok) {   // the guest's log shows which world effect failed and why
                if (!g_worldApplier->lastError().empty()) r += ": " + g_worldApplier->lastError();
                r += " | " + json.substr(0, 300);
            }
            return r;
        });
#endif
    std::string err;
    if (!g_rt->start(&err)) {
        g_failed = true;
        g_failMsg = err;
        nvcoop::Log("coop: start failed: %s", err.c_str());
        return;
    }
    nvcoop::Log("coop: started role=%s port=%u", roleName(g_cfg.role), (unsigned)g_rt->boundPort());
    nvcoop::PushEvent("coop.started", g_rt->statusJson());
}

// coop.rebaseline (host, test tooling only: needs env NVCOOP_TEST_HELPERS=1): a test helper changed the host world outside the
// session (test.quest.reset), so re-dump it as a new world epoch; guests rebase onto it like after a host load.
bool MRebaseline(const std::string&, std::string& out) {
    const char* gate = std::getenv("NVCOOP_TEST_HELPERS");
    if (!gate || gate[0] != '1') { out = "test helpers disabled (start the game with NVCOOP_TEST_HELPERS=1)"; return false; }
    if (!g_rt) { out = "co-op runtime not running"; return false; }
    std::string what;
    const bool ok = g_rt->rebaselineHost(&what);
    nvcoop::Log("coop: rebaseline: %s", what.c_str());
    if (!ok) { out = what; return false; }
    out = std::string("{\"ok\":true,\"what\":\"") + what + "\"}";
    return true;
}

bool MDeaths(const std::string& params, std::string& out);
#ifdef NVCOOP_WITH_UI
bool MSubs(const std::string&, std::string& out);
bool MTalkState(const std::string& params, std::string& out);
#endif
bool MSharedXp(const std::string&, std::string& out);
bool MContainers(const std::string&, std::string& out);
bool MDoors(const std::string&, std::string& out);
bool MRespawn(const std::string&, std::string& out);
bool MAutosave(const std::string& params, std::string& out);
bool MStatus(const std::string&, std::string& out) {
    if (g_rt) { out = g_rt->statusJson(); return true; }
    nvcoop::json::Value o = nvcoop::json::Value::object();
    o.set("role", roleName(g_cfg.role)).set("enabled", g_cfg.enable).set("running", false).set("error", g_failMsg)
        .set("waitingForHooks", g_cfg.enable && !g_failed && nvhooks::hub() == nullptr).set("waitFrames", (int64_t)g_waitFrames);
    out = o.dump();
    return true;
}

bool MLog(const std::string& params, std::string& out) {
    if (!g_rt) { out = "coop not running"; return false; }
    nvcoop::json::Value p;
    std::string err;
    size_t n = 64;
    if (!params.empty() && nvcoop::json::parse(params, p, err)) n = static_cast<size_t>(p.get("n").asInt(64));
    out = g_rt->logJson(n);
    return true;
}

bool MAction(const std::string& params, std::string& out) {
    if (!g_rt || !g_rt->client()) { out = "coop client not running"; return false; }
    nvcoop::json::Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    fnvcoop::FormRef ref;
    if (!nvhooks::strToRef(p.get("ref").asString(""), &ref)) { out = "ref must be '<plugin>:<hex>'"; return false; }
    std::string k = p.get("kind").asString("activate");
    auto kind = k == "pickup" ? fnvcoop::IntentKind::PickUp : k == "trigger" ? fnvcoop::IntentKind::TriggerEnter : fnvcoop::IntentKind::ActivateRef;
    uint32_t id = g_rt->submitAction(kind, ref);
    out = std::string("{\"id\":") + std::to_string(id) + ",\"syncing\":" + (g_rt->syncing() ? "true" : "false") + "}";
    return id != 0 ? true : (out = "not joined or still syncing: " + out, false);
}

// coop.baseline.export {path}: dump the live world (L7) to a baseline file; returns entry counts. Main thread.
bool MExport(const std::string& params, std::string& out) {
    nvcoop::json::Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    std::string path = p.get("path").asString("");
    if (path.empty()) { out = "path required"; return false; }
    fnvcoop::ShadowState st;
    uint64_t hash = 0;
    if (!p.get("fresh").asBool(false) && g_rt && g_rt->usedBaseline(&hash)) st = *g_rt->usedBaseline(&hash);   // exactly what the host session uses
    else if (!g_dumper.dump(&st, &hash, &err)) { out = err; return false; }
    if (!SaveBaselineFile(path, st, hash)) { out = "cannot write " + path; return false; }
    nvcoop::json::Value o = nvcoop::json::Value::object();
    o.set("path", path).set("hash", std::to_string(hash)).set("stages", (int64_t)st.stages.size()).set("objectives", (int64_t)st.objectives.size())
        .set("globals", (int64_t)st.globals.size()).set("scriptVars", (int64_t)st.scriptVars.size()).set("questRunning", (int64_t)st.questRunning.size())
        .set("factionRank", (int64_t)st.factionRank.size()).set("sayOnce", (int64_t)st.sayOnce.size()).set("enabled", (int64_t)st.enabled.size())
        .set("kills", (int64_t)st.kills.size()).set("karma", st.ledger[0].karma);
    out = o.dump();
    return true;
}

bool ParseRef(const nvcoop::json::Value& p, const char* key, fnvcoop::FormRef* out, std::string& err) {
    if (!nvhooks::strToRef(p.get(key).asString(""), out)) { err = std::string(key) + " must be '<plugin>:<hex>'"; return false; }
    return true;
}

// coop.scenePass {quest}: the local player finished their per-player step (chargen). TEST/automation entry until L5 detects it.
bool MScenePass(const std::string& params, std::string& out) {
    if (!g_rt) { out = "coop not running"; return false; }
    nvcoop::json::Value p;
    std::string err;
    fnvcoop::FormRef q;
    if (!nvcoop::json::parse(params, p, err) || !ParseRef(p, "quest", &q, err)) { out = err; return false; }
    g_rt->scenePass(q);
    out = "{}";
    return true;
}

// coop.stageRequested {quest, stage}: ask the rules engine whether the host may advance now (the SetStage gate calls the same function)
bool MStageRequested(const std::string& params, std::string& out) {
    if (!g_rt) { out = "coop not running"; return false; }
    nvcoop::json::Value p;
    std::string err;
    fnvcoop::FormRef q;
    if (!nvcoop::json::parse(params, p, err) || !ParseRef(p, "quest", &q, err)) { out = err; return false; }
    bool proceed = g_rt->stageRequested(q, (uint32_t)p.get("stage").asInt(0));
    out = std::string("{\"proceed\":") + (proceed ? "true" : "false") + "}";
    return true;
}

// coop.baseline.vars {quests:["falloutnv.esm:104c1c",..]}: script vars of those owners as seen by the baseline the host session uses vs the
// host's current session world (baseline + commits). Diagnostic for baseline/poll disagreements.
bool MBaselineVars(const std::string& params, std::string& out) {
    if (!g_rt || !g_rt->host()) { out = "coop host not running"; return false; }
    nvcoop::json::Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    uint64_t h = 0;
    const fnvcoop::ShadowState* base = g_rt->usedBaseline(&h);
    const fnvcoop::ShadowState& world = g_rt->host()->world();
    nvcoop::json::Value res = nvcoop::json::Value::object();
    for (const auto& q : p.get("quests").items()) {
        fnvcoop::FormRef ref;
        if (!nvhooks::strToRef(q.asString(""), &ref)) continue;
        nvcoop::json::Value o = nvcoop::json::Value::object();
        auto dumpMap = [&](const fnvcoop::ShadowState& st) {
            nvcoop::json::Value m = nvcoop::json::Value::object();
            for (auto& kv : st.scriptVars)
                if (kv.first.first == ref) m.set(std::to_string(kv.first.second), kv.second);
            return m;
        };
        if (base) o.set("baseline", dumpMap(*base));
        o.set("sessionWorld", dumpMap(world));
        res.set(q.asString(""), o);
    }
    out = res.dump();
    return true;
}

// coop.explain {quest, var, slot?}: HostSession::explainJoinEntry for a script var (L4 diagnostics: present/absent, hashes, VOLATILE)
bool MExplain(const std::string& params, std::string& out) {
    if (!g_rt || !g_rt->host()) { out = "coop host not running"; return false; }
    nvcoop::json::Value p;
    std::string err;
    fnvcoop::FormRef q;
    if (!nvcoop::json::parse(params, p, err) || !ParseRef(p, "quest", &q, err)) { out = err; return false; }
    fnvcoop::SetScriptVar probe;
    probe.owner = q;
    probe.index = static_cast<uint16_t>(p.get("var").asInt(0));
    nvcoop::json::Value o = nvcoop::json::Value::object();
    o.set("explain", g_rt->host()->explainJoinEntry(static_cast<fnvcoop::PlayerId>(p.get("slot").asInt(1)), probe))
        .set("unresolvedVolatileNames", (int64_t)fnvcoop::volatileKeys().unresolvedNames());
    out = o.dump();
    return true;
}

// coop.leave {}: drop out of the session cleanly (peers see a disconnect now); the game keeps running and the world stays as it is.
bool MLeave(const std::string&, std::string& out) {
    if (!g_rt) { out = "coop not running"; return false; }
    std::string err;
    if (!g_rt->leave(&err)) { out = err; return false; }
    nvcoop::PushEvent("coop.left", "{}");
    out = "{\"ok\":true}";
    return true;
}

// coop.join {host?, port?}: build a new session (the baseline / rejoin path) with the configured or given address.
bool MJoin(const std::string& params, std::string& out) {
    if (!g_rt) { out = "coop not running"; return false; }
    nvcoop::json::Value p;
    std::string err;
    if (!params.empty() && !nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    if (!g_rt->join(p.get("host").asString(""), (uint16_t)p.get("port").asInt(0), &err)) { out = err; return false; }
    nvcoop::PushEvent("coop.joining", g_rt->statusJson());
    out = "{\"ok\":true}";
    return true;
}

// coop.owner {ref}: who owns that actor on THIS machine, from plugin/actors' ownership view: {"owner": n, "epoch": n, "known": b, ...}
// (255 = nobody). Built without plugin/actors it answers unsupported.
bool MOwner(const std::string& params, std::string& out) {
#ifdef NVCOOP_WITH_ACTORS
    return nvcoop::actors::glueOwnerRpc(params, out);
#else
    (void)params;
    out = "{\"unsupported\":true,\"reason\":\"actor-ownership service is not in the plugin build\"}";
    return true;
#endif
}

bool MInject(const std::string& params, std::string& out) {
    if (!g_rt) { out = "coop not running"; return false; }
    std::string res, err;
    if (!g_rt->inject(params, &res, &err)) { out = err; return false; }
    out = res;
    return true;
}

void OnLoad(void*) {
    fnvcoop::EnetTransport::setAllocator(&nvcoop::enetCountedMalloc, &nvcoop::enetCountedFree);   // before any transport opens
    LoadConfig();
#ifdef NVCOOP_WITH_LIVE
    nvcoop::livewire::glueLoad();   // main.cpp already ran glueConfigure() (before any module loaded)
#endif
    nvcoop::RegisterMethod("coop.status", MStatus, true);
#ifdef NVCOOP_WITH_UI
    nvcoop::RegisterMethod("coop.subs", MSubs, true);
    nvcoop::RegisterMethod("coop.talkstate", MTalkState, true);
#endif
    nvcoop::RegisterMethod("coop.xp", MSharedXp, true);
    nvcoop::RegisterMethod("coop.containers", MContainers, true);
    nvcoop::RegisterMethod("coop.doors", MDoors, true);
    nvcoop::RegisterMethod("coop.respawn", MRespawn, true);
    nvcoop::RegisterMethod("coop.autosave", MAutosave, true);
    nvcoop::RegisterMethod("coop.deaths", MDeaths, true);
    nvcoop::RegisterMethod("coop.log", MLog, true);
    nvcoop::RegisterMethod("coop.inject", MInject, true);
    nvcoop::RegisterMethod("coop.baseline.export", MExport, true);
    nvcoop::RegisterMethod("coop.baseline.vars", MBaselineVars, true);
    nvcoop::RegisterMethod("coop.explain", MExplain, true);
    nvcoop::RegisterMethod("coop.action", MAction, true);
    nvcoop::RegisterMethod("coop.leave", MLeave, true);
    nvcoop::RegisterMethod("coop.join", MJoin, true);
    nvcoop::RegisterMethod("coop.owner", MOwner, true);
    nvcoop::RegisterMethod("coop.rebaseline", MRebaseline, true);
    nvcoop::RegisterMethod("coop.scenePass", MScenePass, true);
    nvcoop::RegisterMethod("coop.stageRequested", MStageRequested, true);   // client: {kind:"activate|pickup|trigger", ref:"falloutnv.esm:00ABCD"}
}

fnvcoop::Bytes g_reloadSidecar;   // hot reload: the previous core's session sidecar, applied once the new runtime is running

// The session's ThreadWorker waits for jobs inside the core: adopted, the hot-reload drain ignores it, and ReloadShutdown joins it
// (g_rt->stop()) right after the drain. Without it every reload with a co-op session running was refused (in game 2026-10-21).
unsigned long g_adoptedWorker = 0;
void AdoptWorkerThread(void* handle) {
    const unsigned long tid = handle ? GetThreadId(reinterpret_cast<HANDLE>(handle)) : 0;
    if (tid == g_adoptedWorker) return;
    if (g_adoptedWorker) nvreload::releaseThread(g_adoptedWorker);
    if (tid) nvreload::adoptThread(tid);
    g_adoptedWorker = tid;
}

#ifdef NVCOOP_WITH_WORLD
// Deaths no script command reports (a kill in combat, a fall): the host looks at the loaded actors a few times a second and commits a
// KillActor for each one it saw alive and now sees dead. Guests apply it like a scripted kill (shared kills, shadow union).
nvworld::DeathWatch g_deaths;
uint64_t g_deathScanFrame = 0;
// Actor::lifeState at +0x108: 0 alive, 1 dying, 2 dead (the actors module's ActorRaw constant; Actor::IsDead treats 1 and 2 as dead).
// The attacker of the last hit: Actor::baseProcess (+0x68), processLevel (+0x28, 0 high .. 3 low), MiddleHighProcess::lastHitData (+0x240,
// high and middle-high only), ActorHitData::source (+0x00) (xNVSE GameObjects.h / GameProcess.h).
constexpr uint32_t kActorLifeStateOffset = 0x108, kActorProcessOffset = 0x68, kProcessLevelOffset = 0x28, kLastHitDataOffset = 0x240;
template <class T>
T At(const void* base, uint32_t off) { return *reinterpret_cast<const T*>(reinterpret_cast<const char*>(base) + off); }
size_t WalkLoadedActorsSEH(uint32_t* ids, uint8_t* dead, uint8_t* byPlayer, size_t cap) {
    size_t n = 0;
    const void* player = g_thePlayer ? *g_thePlayer : nullptr;
    auto add = [&](TESObjectREFR* r) {
        if (n >= cap || !r || (r->typeID != kFormType_Character && r->typeID != kFormType_Creature)) return;
        const uint32_t st = At<uint32_t>(r, kActorLifeStateOffset);
        const void* proc = At<const void*>(r, kActorProcessOffset);
        const void* hit = proc && At<uint32_t>(proc, kProcessLevelOffset) <= 1 ? At<const void*>(proc, kLastHitDataOffset) : nullptr;
        ids[n] = r->refID;
        byPlayer[n] = hit && player && At<const void*>(hit, 0) == player;
        dead[n++] = st == 1 || st == 2;
    };
    __try {
        const tList<Actor>* lists[2] = {&g_actorProcessManager->highActors, &g_actorProcessManager->middleHighActors.head};
        for (const tList<Actor>* l : lists) {
            int guard = 0;
            for (auto* node = l->Head(); node && guard < 6000; node = node->next, ++guard) add(node->data);
        }
        TESObjectCELL* cell = g_thePlayer ? (*g_thePlayer)->parentCell : nullptr;   // actors in low process are in the player's cell list
        int guard = 0;
        if (cell)
            for (auto* node = cell->objectList.Head(); node && guard < 20000; node = node->next, ++guard) add(node->data);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
    return n;
}
// Every machine: an actor the shared world has dead (a KillActor in the shadow) but that is alive here is killed again. Under NV:MP the
// NV:MP server restores a nearby NPC from its own copy after our kill ran: on the guest (a host kill by gunfire stayed alive there) and,
// fed back by the guest's client, on the host too. At most once per actor every 3 s.
std::unordered_map<uint32_t, ULONGLONG> g_rekilled;
struct RekillWindow { ULONGLONG since = 0; int n = 0; };
std::unordered_map<uint32_t, RekillWindow> g_rekillWin;
const void* g_enforceCell = nullptr;
ULONGLONG g_enforceCellAt = 0;
// The actor's 3D is loaded (TESObjectREFR::renderState +0x64, RenderState::niNode +0x14): killing an actor whose cell is still loading
// crashed the host under NV:MP (0x0044DD9D, 2026-10-27).
bool Has3DSEH(uint32_t id) {
    __try {
        auto* r = static_cast<TESObjectREFR*>(nvhooks::resolver().lookup(id));
        return r && r->renderState && r->renderState->niNode;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
uint32_t BaseOfRefSEH(uint32_t id) {
    __try {
        auto* r = static_cast<TESObjectREFR*>(nvhooks::resolver().lookup(id));
        return r && r->baseForm ? r->baseForm->refID : 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}
uint64_t g_rekills = 0, g_deathScans = 0, g_lastSeen = 0, g_lastKills = 0;
void EnforceKills(const std::vector<std::pair<uint32_t, bool>>& seen) {
    const auto& kills = g_rt->host() ? g_rt->host()->world().kills : g_rt->client()->state().kills;
    ++g_deathScans;
    g_lastSeen = seen.size();
    g_lastKills = kills.size();
    if (kills.empty()) return;
    const ULONGLONG now = GetTickCount64();
    const void* cell = g_thePlayer && *g_thePlayer ? (*g_thePlayer)->parentCell : nullptr;
    if (cell != g_enforceCell) { g_enforceCell = cell; g_enforceCellAt = now; }
    if (!cell || now - g_enforceCellAt < 1500) return;   // a cell that is still loading: its actors' 3D is not ready (Has3DSEH checks each one too)
    for (const auto& [id, dead] : seen) {
        if (dead) continue;
        fnvcoop::FormRef r;
        if (!nvhooks::resolver().toRef(id, &r) || !kills.count(r)) continue;
        ULONGLONG& last = g_rekilled[id];
        if (last && now - last < 3000) continue;
        RekillWindow& w = g_rekillWin[id];
        if (now - w.since > 60000) { w.since = now; w.n = 0; }
        if (w.n >= 5) continue;   // something keeps reviving it (NV:MP's server copy): stop fighting it for this minute
        if (!Has3DSEH(id)) continue;
        last = now;
        ++w.n;
        nvhooks::ApplyScope scope;   // host-authored state: lift the guest's suppression for the console line
        const bool ok = nvhooks::execConsoleLineOn("KillActor", id);
        if (++g_rekills <= 50) nvcoop::Log("coop: %08X (%s) is dead in the shared world but alive here -> KillActor again: %s", id, nvhooks::refToStr(r).c_str(), ok ? "ok" : "FAILED");
    }
}
// coop.deaths {ref?}: the death watch's counters; with ref, whether the shared world has it dead and what this machine sees.
bool MDeaths(const std::string& params, std::string& out) {
    nvcoop::json::Value p, o = nvcoop::json::Value::object();
    std::string err;
    if (!params.empty()) nvcoop::json::parse(params, p, err);
    o.set("scans", (int64_t)g_deathScans).set("lastSeen", (int64_t)g_lastSeen).set("kills", (int64_t)g_lastKills).set("rekills", (int64_t)g_rekills)
        .set("tracked", (int64_t)g_deaths.tracked());
    const std::string ref = p.isObject() ? p.get("ref").asString() : "";
    if (!ref.empty() && g_rt) {
        fnvcoop::FormRef r;
        if (nvhooks::strToRef(ref, &r)) {
            const auto* k = g_rt->host() ? &g_rt->host()->world().kills : g_rt->client() ? &g_rt->client()->state().kills : nullptr;
            o.set("sharedDead", k && k->count(r) > 0);
            const uint32_t id = nvhooks::resolver().fromRef(r);
            o.set("id", (int64_t)id).set("rekilledAt", (int64_t)(g_rekilled.count(id) ? g_rekilled[id] : 0));
        }
    }
    out = o.dump();
    return true;
}
// Host: every death it sees is the shared truth. Guest: only a death its own player caused is reported (ReportKill; the host kills the actor
// and its KillActor reaches everybody). A guest's NPC-vs-NPC fight is its local AI and is not reported.
void ScanDeaths() {
    if (!g_cfg.deathWatch || !g_rt || ++g_deathScanFrame % 10) return;
    const bool host = g_rt->host() != nullptr;
    if (!host && !(g_rt->client() && g_rt->client()->joined())) return;
    static uint32_t ids[8192];
    static uint8_t dead[8192], byPlayer[8192];
    const size_t n = WalkLoadedActorsSEH(ids, dead, byPlayer, 8192);
    std::vector<std::pair<uint32_t, bool>> seen;
    std::unordered_map<uint32_t, bool> mine;
    seen.reserve(n);
    for (size_t i = 0; i < n; ++i) {
        if (!ids[i] || ids[i] == 0x14 || (ids[i] >> 24) == 0xFF) continue;   // the player; created refs have no shared id
        // The engine's own GetDead: NV:MP revives an NPC (health back) without resetting the raw life state, which then still read "dead"
        // and hid the revival from EnforceKills. The raw field is the fallback when the evaluator is not callable.
        double v = 0;
        seen.emplace_back(ids[i], nvworld::queryCondition("GetDead", ids[i], 0, &v) ? v != 0 : dead[i] != 0);
        if (byPlayer[i]) mine[ids[i]] = true;
    }
    EnforceKills(seen);
    for (uint32_t id : g_deaths.update(seen)) {
        fnvcoop::FormRef r;
        if (!nvhooks::resolver().toRef(id, &r)) continue;
        if (host) {
            fnvcoop::Cause c;
            c.kind = fnvcoop::CauseKind::Combat;
            const uint64_t seq = g_rt->commitHostEffects(c, {fnvcoop::KillActor{r}});
            nvcoop::Log("coop: %08X (%s) died on the host -> KillActor seq %llu", id, nvhooks::refToStr(r).c_str(), (unsigned long long)seq);
        } else if (mine.count(id)) {
            const uint32_t intent = g_rt->submitAction(fnvcoop::IntentKind::ReportKill, r);
            nvcoop::Log("coop: this player killed %08X (%s) -> ReportKill intent %u", id, nvhooks::refToStr(r).c_str(), intent);
        }
    }
}
#endif

// ---- guest character creation -------------------------------------------------------------------------------------------------------------
// The birth scene's menus are opened by quest scripts, which never run on a guest: a guest followed the host's TTW Fallout 3 start with the
// default character. The guest opens the same menu for its own player when the shared story reaches the step, one at a time, only while no
// other menu is up. Its choices are local (CG00 / CG03 are PerPlayerScene in the TTW rules: the host's own picks stay with the host).
struct ChargenStep {
    const char* quest;   // "plugin:hex" of the quest
    int stage;           // the step starts when the quest reaches this stage
    const char* line;    // console line that opens the menu for the local player
    uint32_t menuId;     // the menu it opens (done when it closes again)
    const char* what;
};
const ChargenStep kChargenSteps[] = {
    {"falloutnv.esm:01F388", 38, "ShowNameMenu", 1051, "name (CG00, Dad asks)"},
    {"falloutnv.esm:01F388", 60, "ShowRaceMenu", 1036, "face and sex (CG00 Gene Projector)"},
    {"falloutnv.esm:01F388", 70, "ShowTraitMenu", 1084, "traits (CG00, TTW)"},
    {"fallout3.esm:014E83", 50, "ShowSPECIALBookMenu", 1060, "S.P.E.C.I.A.L. (CG01 book)"},
    {"fallout3.esm:014E85", 80, "SetTagSkills 3", 1048, "tag skills (CG03 G.O.A.T.)"},
    // TTW's pre-order pack menu (stash packs) starts when the Vault is left. Its quest and the VPreorder* item quests are GuestLocalRun and
    // volatile (each player picks and gets their own packs), so the host's StartQuest does not reach the guest: the guest starts its own.
    {"fallout3.esm:014E86", 200, "StartQuest TTWPreorderMenuQuest", 1001, "pre-order packs (CG04, leaving the Vault)"},
};
constexpr size_t kChargenCount = sizeof kChargenSteps / sizeof kChargenSteps[0];
struct ChargenState {
    bool primed = false;
    bool done[kChargenCount] = {};
    std::vector<size_t> queue;
    size_t active = SIZE_MAX;
    bool sawMenu = false;
    ULONGLONG startedAt = 0, waitUntil = 0;
    uint64_t frame = 0;
} g_chargen;
constexpr uint32_t kTopMenuIdFn = 0x714F00, kInterfaceMgrPtr = 0x11D8A80;
uint32_t TopMenuIdSEH() {
    static int pinOk = -1;   // the entry still has the original prologue (another patcher's detour would change the calling convention)
    if (pinOk < 0) {
        static const uint8_t kPin[16] = {0x55, 0x8b, 0xec, 0x83, 0xec, 0x08, 0x89, 0x4d, 0xf8, 0xc7, 0x45, 0xfc, 0x00, 0x00, 0x00, 0x00};
        pinOk = std::memcmp(reinterpret_cast<const void*>(kTopMenuIdFn), kPin, sizeof kPin) == 0 ? 1 : 0;
        if (!pinOk) nvcoop::Log("coop: guest chargen off: TopMenuId 0x714F00 is not the original code");
    }
    if (pinOk != 1) return 0xFFFFFFFF;
    __try {
        void* mgr = *reinterpret_cast<void**>(kInterfaceMgrPtr);
        return mgr ? reinterpret_cast<uint32_t(__thiscall*)(void*)>(kTopMenuIdFn)(mgr) : 0xFFFFFFFF;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0xFFFFFFFF;
    }
}
void ResetGuestChargen() { g_chargen = ChargenState(); }
void TickGuestChargen() {
    if (!g_cfg.guestChargen || !g_rt || !g_rt->client() || !g_rt->client()->joined() || g_rt->syncing() || ++g_chargen.frame % 15) return;
    ChargenState& c = g_chargen;
    for (size_t i = 0; i < kChargenCount; ++i) {
        fnvcoop::FormRef q;
        if (c.done[i] || !nvhooks::strToRef(kChargenSteps[i].quest, &q)) continue;
        const int st = nvhooks::localQuestStage(q);
        if (st < kChargenSteps[i].stage) continue;
        c.done[i] = true;
        if (c.primed) {   // reached while we watched; a step the loaded game had already passed is not repeated
            c.queue.push_back(i);
            nvcoop::Log("coop: guest chargen: the story reached %s -> queued", kChargenSteps[i].what);
        }
    }
    c.primed = true;
    const ULONGLONG now = GetTickCount64();
    const uint32_t top = TopMenuIdSEH();
    if (top == 0xFFFFFFFF) return;
    if (c.active != SIZE_MAX) {
        const ChargenStep& s = kChargenSteps[c.active];
        if (top == s.menuId) c.sawMenu = true;
        else if (c.sawMenu) {
            nvcoop::Log("coop: guest chargen: %s done", s.what);
            c.active = SIZE_MAX;
            c.waitUntil = now + 1500;
        } else if (now - c.startedAt > 8000) {
            nvcoop::Log("coop: guest chargen: %s did not open (menu %u); skipped", s.what, top);
            c.active = SIZE_MAX;
            c.waitUntil = now + 1500;
        }
        return;
    }
    if (c.queue.empty() || now < c.waitUntil || top != 0) return;   // only when no other menu (a box, a conversation) is up
    const size_t i = c.queue.front();
    c.queue.erase(c.queue.begin());
    nvhooks::ApplyScope scope;   // SetTagSkills is a captured command a guest's own scripts may not run; this one is for its own player
    const bool ok = nvhooks::execConsoleLineOn(kChargenSteps[i].line, 0);
    nvcoop::Log("coop: guest chargen: opening %s ('%s'): %s", kChargenSteps[i].what, kChargenSteps[i].line, ok ? "ok" : "FAILED");
    c.active = i;
    c.sawMenu = false;
    c.startedAt = now;
}

#ifdef NVCOOP_WITH_UI
// ---- conversation subtitles (talk_subs.h): this player's DialogMenu lines -> everybody else, shown near the NPC ----
namespace talksubs = nvcoop::coop::talksubs;
struct TalkSubs {
    fnvcoop::MuxTransport* mux = nullptr;
    talksubs::Watch watch;
    talksubs::ReplyWatch replyWatch;
    std::vector<talksubs::Line> inbox;
    uint32_t frame = 0;
    uint64_t sent = 0, received = 0, shown = 0, notNear = 0, relayed = 0, replies = 0, voiced = 0;
    std::unordered_map<uint32_t, fnvcoop::FormRef> lastVoiced;   // npc refID -> the INFO its replay last spoke here
    uint64_t clearAt = 0;          // the subtitle area shows our line until then (0 = not ours)
    std::string reply;             // the talking player's last pick, shown above the NPC's answer
    uint64_t replyAt = 0;
    uint64_t inArea = 0;           // lines shown in the game's subtitle area (the rest went to the message corner)
    std::unordered_map<uint32_t, uint64_t> unstickAt;   // npc refID -> when its replayed line is surely over (see UnstickReplayedTalkers)
    uint64_t unstuck = 0;          // NPCs whose "talking" flag a replay had left set, cleared
    struct SitWatch { uint64_t until = 0, stuckSince = 0; };
    std::unordered_map<uint32_t, SitWatch> sitWatch;   // npc refID -> watched for a hung sit / stand transition (see UnstickSitters)
    uint64_t sitResets = 0;
} g_subs;
struct TalkState { bool ok = false, flag = false, process = false; };
TalkState ReadTalkStateSEH(uint32_t refId);
int SitSleepStateSEH(uint32_t refId);
bool ClearTalkFlagSEH(uint32_t refId);
uint32_t DialogPartnerSEH();
// A replayed line (Say) on an NPC this machine does not run the AI of (NV:MP drives it from the other player's game) could leave the NPC
// "talking" for good, and a talking NPC does not start a conversation: seen once in a two-copy test (the host could no longer talk to Lucas
// Simms after the guest's copy replayed his lines). A few seconds after the line must have ended, a flag still set is cleared, unless this
// player is talking to that NPC right now.
void UnstickReplayedTalkers(uint64_t nowMs) {
    for (auto it = g_subs.unstickAt.begin(); it != g_subs.unstickAt.end();) {
        if (nowMs < it->second) { ++it; continue; }
        if (DialogPartnerSEH() != it->first) {
            const TalkState t = ReadTalkStateSEH(it->first);
            if (t.ok && t.flag && ClearTalkFlagSEH(it->first)) {
                ++g_subs.unstuck;
                nvcoop::Log("coop: subtitles: %08X was left talking after a replayed line: cleared (process says %s)", it->first, t.process ? "talking" : "not talking");
            }
        }
        it = g_subs.unstickAt.erase(it);
    }
}
// An NPC the OTHER player talks to gets up from its chair or desk in their game; NV:MP plays that here, and here it could hang half way
// (GetSitting 4 "getting up" or 2 "sitting down" for minutes: Doc Church after a guest's treatment, Moira after a trade, 2026-11-12), and a
// hung NPC does not talk to anybody here. For two minutes after the other player's last line, an NPC hung like that for 12 s gets ResetAI
// on this machine only (it then stands or sits properly and talks again; tested by hand on Doc Church).
void UnstickSitters(uint64_t nowMs) {
    if (g_subs.sitWatch.empty()) return;
    nvhooks::CaptureHub* hub = nvhooks::hub();
    for (auto it = g_subs.sitWatch.begin(); it != g_subs.sitWatch.end();) {
        if (nowMs > it->second.until) { it = g_subs.sitWatch.erase(it); continue; }
        const int st = SitSleepStateSEH(it->first);
        if ((st != 2 && st != 4) || DialogPartnerSEH() == it->first) { it->second.stuckSince = 0; ++it; continue; }
        if (!it->second.stuckSince) it->second.stuckSince = nowMs;
        if (hub && nowMs - it->second.stuckSince > 12000) {
            ExecMuted(*hub, "ResetAI", it->first);
            ++g_subs.sitResets;
            it->second.stuckSince = 0;
            it->second.until = 0;   // once per watch: a second hang right after is NV:MP replaying the other copy's animation, not ours to fight
            nvcoop::Log("coop: %08X hung %s after the other player's conversation: ResetAI here", it->first, st == 4 ? "getting up" : "sitting down");
        }
        ++it;
    }
}
constexpr uint32_t kDialogMenuPtr = 0x11D9510, kDialogPartnerRef = 0x80, kInfoSpokenPtr = 0x11D9518, kInfoPrompt = 0x28, kQueueUIMessage = 0x7052F0;

// The INFO the player picked (the click handler stores it in 0x11D9518 for the END result script) and its own prompt text
// (String {char*; u16 len}); 0 = none / fault.
uint32_t DialogSpokenInfoSEH(char* prompt, size_t n) {
    __try {
        prompt[0] = 0;
        const uint32_t menu = *reinterpret_cast<const uint32_t*>(kDialogMenuPtr);
        const uint32_t info = menu ? *reinterpret_cast<const uint32_t*>(kInfoSpokenPtr) : 0;
        if (!info) return 0;
        const char* s = *reinterpret_cast<const char* const*>(info + kInfoPrompt);
        const uint16_t len = *reinterpret_cast<const uint16_t*>(info + kInfoPrompt + 4);
        if (s && len) strncpy_s(prompt, n, s, len < n - 1 ? len : n - 1);
        return info;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}
// What the player clicked for that INFO: its prompt, else its topic's name ("<no name>" = none: the NPC's own greeting).
std::string ReplyTextOf(uint32_t info, const char* prompt) {
    if (prompt && *prompt) return prompt;
    auto* topic = reinterpret_cast<TESForm*>(static_cast<uintptr_t>(nvui::ops::topicOfInfo(info)));
    if (!topic || topic->refID == 0xC8) return std::string();   // GREETING: the NPC opened, the player clicked nothing
    const char* n = GetFullName(topic);
    return n && *n && std::strcmp(n, "<no name>") != 0 ? n : std::string();
}

// DialogMenu tile text ("NOGLOW_BRANCH/DM_SpeakerText/string"): the menu's TileMenu is Menu+4 (xNVSE GameUI.h). False = no menu / fault.
bool DialogTileTextSEH(const char* path, char* out, size_t n) {
    __try {
        out[0] = 0;
        const uint32_t menu = *reinterpret_cast<const uint32_t*>(kDialogMenuPtr);
        if (!menu) return false;
        Tile* root = *reinterpret_cast<Tile**>(menu + 4);
        if (!root) return false;
        char p[96];
        strncpy_s(p, path, _TRUNCATE);   // GetComponentValue cuts the path at its slashes
        Tile::Value* v = root->GetComponentValue(p);
        if (v && v->str) strncpy_s(out, n, v->str, _TRUNCATE);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
uint32_t DialogPartnerSEH() {
    __try {
        const uint32_t menu = *reinterpret_cast<const uint32_t*>(kDialogMenuPtr);
        const uint32_t ref = menu ? *reinterpret_cast<const uint32_t*>(menu + kDialogPartnerRef) : 0;
        return ref ? *reinterpret_cast<const uint32_t*>(ref + 0x0C) : 0;   // TESForm::refID
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}
// The game's own subtitle line: HUDMainMenu/Subtitles/justify_center_text (the engine fills it for general subtitles and hides it with
// visible = 0). Tile::SetStringValue 0xA01350 / SetFloatValue 0xA012D0 (xNVSE GameTiles.h), traits string 0xFC4 / visible 0xFA3.
constexpr uint32_t kHudMainMenuPtr = 0x11D96C0, kTileSetString = 0xA01350, kTileSetFloat = 0xA012D0, kTraitString = 0xFC4, kTraitVisible = 0xFA3;
bool SubtitleAreaSEH(const char* text, bool show) {
    __try {
        const uint32_t hud = *reinterpret_cast<const uint32_t*>(kHudMainMenuPtr);
        Tile* root = hud ? *reinterpret_cast<Tile**>(hud + 4) : nullptr;
        if (!root) return false;
        char path[] = "Subtitles/justify_center_text";   // GetComponentTile cuts the path at its slashes
        Tile* t = root->GetComponentTile(path);
        if (!t) return false;
        reinterpret_cast<void(__thiscall*)(Tile*, uint32_t, const char*, bool)>(kTileSetString)(t, kTraitString, show ? text : " ", true);
        reinterpret_cast<void(__thiscall*)(Tile*, uint32_t, float, bool)>(kTileSetFloat)(t, kTraitVisible, show ? 1.0f : 0.0f, true);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool HudMessageSEH(const char* text, float seconds) {
    __try {
        reinterpret_cast<bool(__cdecl*)(const char*, uint32_t, const char*, const char*, float, bool)>(kQueueUIMessage)(text, 0, nullptr, nullptr, seconds, false);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
std::string LocalPlayerName() {
    auto* p = static_cast<TESObjectREFR*>(nvhooks::resolver().lookup(0x14));
    const char* n = p && p->baseForm ? GetFullName(p->baseForm) : nullptr;
    return n && *n ? n : "Player";
}

void TickTalkSubtitles() {
    if (!g_cfg.talkSubtitles || !g_rt || !g_rt->running()) return;
    fnvcoop::MuxTransport* mux = g_rt->mux();
    const bool host = g_rt->host() != nullptr;
    if (mux && mux != g_subs.mux) {   // a (re)started session has a new mux: listen on it
        g_subs.mux = mux;
        mux->registerChannel(talksubs::kChannel, [mux](fnvcoop::PeerId from, fnvcoop::Bytes&& d) {
            talksubs::Line l;
            if (!g_rt || g_subs.mux != mux || !talksubs::decode(d.data(), d.size(), &l)) return;
            ++g_subs.received;
            if (g_rt->host()) {   // a guest's line: the slot is the sender's, and every other guest gets it too
                for (const auto& c : g_rt->host()->clients()) {
                    if (c.peer == from) l.slot = static_cast<uint8_t>(c.slot);
                    else if (c.joined) { mux->send(c.peer, d); ++g_subs.relayed; }
                }
            }
            if (g_subs.inbox.size() < 64) g_subs.inbox.push_back(std::move(l));
        });
    }
    for (const talksubs::Line& l : g_subs.inbox) {   // show what the others say, when this player is near enough to hear it
        const uint32_t npc = nvhooks::resolver().fromRef(l.npc);
        // Watched for a hung chair even out of earshot: the hang comes with NV:MP's copy of the other player's talk, near or not (2026-11-16:
        // the host stood across the shop, 24 of 30 lines "not near", and Moira stayed half out of her chair there; the host never could
        // trade or repair with her again).
        if (npc) g_subs.sitWatch[npc].until = GetTickCount64() + 120000;
        if (!npc || !nvui::ops::nearPlayer(npc, talksubs::kHearDistance)) { ++g_subs.notNear; continue; }
        // A new INFO of the NPC: it says that very INFO here, so the voice starts with the text (its result scripts stay with the talker:
        // markEchoLine). Not while this player talks to the same NPC themselves.
        const bool reply = l.player.empty();   // "<player>: <reply>" lines carry no "(to ...)"
        const uint32_t ownPartner = DialogPartnerSEH();
        // Never on an NPC in furniture: Say on a seated (or sitting-down / getting-up) NPC left it stuck mid-transition on this machine, where
        // NV:MP drives its animations, and nobody here could talk to it any more (Moira, GetSitting 2/4 for good, 2026-11-12). Text only then.
        if (!reply && l.info.localId && npc != ownPartner && !(g_subs.lastVoiced.count(npc) && g_subs.lastVoiced[npc] == l.info)
            && SitSleepStateSEH(npc) == 0) {
            g_subs.lastVoiced[npc] = l.info;
            if (nvui::ops::sayInfo(npc, l.topic, l.info)) {
                ++g_subs.voiced;
                g_subs.unstickAt[npc] = GetTickCount64() + static_cast<uint64_t>(talksubs::seconds(l.text) * 1000.0f) + 6000;
            }
        }
        // In the game's subtitle line, replacing whatever we showed before (no queue: it appears when the voice starts). The pick stays above
        // the NPC's answer for a moment. While this player is in a conversation of their own the HUD is hidden: the message corner then.
        const uint64_t nowMs = GetTickCount64();
        std::string text = talksubs::chunks(l, 4096).empty() ? std::string() : talksubs::chunks(l, 4096)[0];
        if (reply) { g_subs.reply = text; g_subs.replyAt = nowMs; }
        else if (!g_subs.reply.empty() && nowMs - g_subs.replyAt < 3000) text = g_subs.reply + "\n" + text;
        const float secs = talksubs::seconds(text, reply) + (reply ? 0.0f : 0.5f);
        if (ownPartner == 0 && SubtitleAreaSEH(text.c_str(), true)) {
            g_subs.clearAt = nowMs + static_cast<uint64_t>(secs * 1000.0f);
            ++g_subs.inArea;
        } else {
            for (const std::string& piece : talksubs::chunks(l)) HudMessageSEH(piece.c_str(), talksubs::seconds(piece, reply));
        }
        ++g_subs.shown;
    }
    g_subs.inbox.clear();
    UnstickReplayedTalkers(GetTickCount64());
    UnstickSitters(GetTickCount64());
    if (g_subs.clearAt && GetTickCount64() >= g_subs.clearAt) { SubtitleAreaSEH("", false); g_subs.clearAt = 0; }   // the line's time is up
    if (!mux || ++g_subs.frame % 6) return;   // this player's own conversation, ten times a second
    if (host ? !g_rt->host() : !(g_rt->client() && g_rt->client()->joined())) return;
    char text[talksubs::kMaxText + 1], name[talksubs::kMaxName + 1];
    const bool open = DialogTileTextSEH("NOGLOW_BRANCH/DM_SpeakerText/string", text, sizeof text);
    const uint32_t npc = open ? DialogPartnerSEH() : 0;
    // This player's own partner too: on a guest NV:MP drives the NPC from the host, and after the talk it could hang there sitting back down
    // (Doc Church, GetSitting 2 for minutes on the guest, nobody there could talk to him again).
    if (npc) g_subs.sitWatch[npc].until = GetTickCount64() + 120000;
    char prompt[talksubs::kMaxText + 1];
    const uint32_t info = open && npc ? DialogSpokenInfoSEH(prompt, sizeof prompt) : 0;
    const std::string reply = info ? ReplyTextOf(info, prompt) : std::string();
    const bool newReply = g_subs.replyWatch.update(open && npc, info, reply);
    const bool newLine = g_subs.watch.update(open && npc, npc, text);
    if (!newReply && !newLine) return;
    talksubs::Line l;
    if (!nvhooks::resolver().toRef(npc, &l.npc)) return;
    auto send = [&](const talksubs::Line& x) {
        const fnvcoop::Bytes b = talksubs::encode(x);
        if (host) {
            for (const auto& c : g_rt->host()->clients())
                if (c.joined) mux->send(c.peer, b);
        } else {
            mux->send(0, b);   // a client's only peer is the host, which relays
        }
    };
    l.slot = host ? static_cast<uint8_t>(fnvcoop::kHostPlayer) : static_cast<uint8_t>(g_rt->client()->slot());
    if (newReply) {   // "<player>: <reply>" first, then the NPC's answer
        talksubs::Line r = l;
        r.speaker = LocalPlayerName();
        r.text = reply;
        send(r);
        if (++g_subs.replies <= 5) nvcoop::Log("coop: subtitles: reply '%s'", reply.c_str());
    }
    if (!newLine) return;
    if (info) {   // the INFO this line belongs to, so the others' NPC can say it
        const uint32_t topic = nvui::ops::topicOfInfo(info);
        nvhooks::resolver().toRef(*reinterpret_cast<const uint32_t*>(info + 0x0C), &l.info);
        if (topic) nvhooks::resolver().toRef(*reinterpret_cast<const uint32_t*>(topic + 0x0C), &l.topic);
        if (!topic) l.info = fnvcoop::FormRef();   // without its topic the replay cannot pick it: text only
    }
    DialogTileTextSEH("NOGLOW_BRANCH/DM_SpeakerNameLabel/string", name, sizeof name);
    l.speaker = name;
    l.player = LocalPlayerName();
    l.text = text;
    send(l);
    ++g_subs.sent;
}
// An actor's "talking" state, as the IsTalking condition reads it (0x8A67F0): the byte Actor+0x7D, or the process's own answer (vtable +0x4B8).
TalkState ReadTalkStateSEH(uint32_t refId) {
    TalkState t;
    __try {
        const uint32_t a = reinterpret_cast<uint32_t>(nvhooks::resolver().lookup(refId));
        if (!a) return t;
        const uint8_t type = *reinterpret_cast<const uint8_t*>(a + 4);
        if (type != 0x3B && type != 0x3C) return t;   // Character / Creature
        t.flag = *reinterpret_cast<const uint8_t*>(a + 0x7D) != 0;
        const uint32_t proc = *reinterpret_cast<const uint32_t*>(a + 0x68);
        if (proc) {
            const uint32_t fn = (*reinterpret_cast<const uint32_t* const*>(proc))[0x4B8 / 4];
            t.process = reinterpret_cast<bool(__thiscall*)(uint32_t, uint32_t)>(fn)(proc, a);
        }
        t.ok = true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        t.ok = false;
    }
    return t;
}
// BaseProcess::GetSitSleepState (vtable slot 0x12F, as in animprobe_game.cpp): 0 = not in furniture; 1..4 = sitting down, seated, getting up.
// -1 = not an actor / no process.
int SitSleepStateSEH(uint32_t refId) {
    __try {
        const uint32_t a = reinterpret_cast<uint32_t>(nvhooks::resolver().lookup(refId));
        if (!a) return -1;
        const uint8_t type = *reinterpret_cast<const uint8_t*>(a + 4);
        if (type != 0x3B && type != 0x3C) return -1;
        const uint32_t proc = *reinterpret_cast<const uint32_t*>(a + 0x68);
        if (!proc) return -1;
        const uint32_t fn = (*reinterpret_cast<const uint32_t* const*>(proc))[0x12F];
        return static_cast<int>(reinterpret_cast<uint32_t(__thiscall*)(uint32_t)>(fn)(proc));
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}
bool ClearTalkFlagSEH(uint32_t refId) {
    __try {
        const uint32_t a = reinterpret_cast<uint32_t>(nvhooks::resolver().lookup(refId));
        if (!a) return false;
        const uint8_t type = *reinterpret_cast<const uint8_t*>(a + 4);
        if (type != 0x3B && type != 0x3C) return false;
        *reinterpret_cast<uint8_t*>(a + 0x7D) = 0;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
// coop.talkstate {ref, clear?}: diagnostics for an NPC left "talking" after a replayed line.
bool MTalkState(const std::string& params, std::string& out) {
    nvcoop::json::Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err)) { out = "bad params: " + err; return false; }
    fnvcoop::FormRef r;
    if (!nvhooks::strToRef(p.get("ref").asString(""), &r)) { out = "bad ref"; return false; }
    const uint32_t id = nvhooks::resolver().fromRef(r);
    TalkState before = ReadTalkStateSEH(id);
    bool cleared = false;
    if (p.get("clear").asBool(false)) cleared = ClearTalkFlagSEH(id);
    TalkState after = ReadTalkStateSEH(id);
    char b[200];
    std::snprintf(b, sizeof b, "{\"ok\":%s,\"flag\":%s,\"process\":%s,\"cleared\":%s,\"flagAfter\":%s,\"processAfter\":%s}", before.ok ? "true" : "false",
                  before.flag ? "true" : "false", before.process ? "true" : "false", cleared ? "true" : "false", after.flag ? "true" : "false", after.process ? "true" : "false");
    out = b;
    return true;
}

// coop.subs: subtitle counters (sent = own lines out, received / relayed / shown / notNear = the others' lines here).
bool MSubs(const std::string&, std::string& out) {
    char b[480];
    std::snprintf(b, sizeof b, "{\"enabled\":%s,\"sent\":%llu,\"received\":%llu,\"relayed\":%llu,\"shown\":%llu,\"notNear\":%llu,\"replies\":%llu,\"voiced\":%llu,\"echoSkipped\":%llu,\"inArea\":%llu,\"unstuck\":%llu,\"sitResets\":%llu,\"sitWatched\":%u}",
                  g_cfg.talkSubtitles ? "true" : "false", (unsigned long long)g_subs.sent, (unsigned long long)g_subs.received,
                  (unsigned long long)g_subs.relayed, (unsigned long long)g_subs.shown, (unsigned long long)g_subs.notNear,
                  (unsigned long long)g_subs.replies, (unsigned long long)g_subs.voiced, (unsigned long long)nvhooks::echoLinesSkipped(), (unsigned long long)g_subs.inArea, (unsigned long long)g_subs.unstuck,
                  (unsigned long long)g_subs.sitResets, (unsigned)g_subs.sitWatch.size());
    out = b;
    return true;
}
#endif

// ---- shared experience (shared_xp.h): what the engine pays this player (kills, locks, terminals, places) -> every other player ----
namespace sharedxp = nvcoop::coop::sharedxp;
// Player vtable 0x108AA3C, slot +0x488 = RewardXP(int), thiscall, ret 4 (read in game: 0x94FD10 `push ebp ... add ecx,0x878 ... ret 4`;
// Cmd_RewardXP and the kill-XP code call it through this slot).
constexpr uint32_t kPlayerVtbl = 0x0108AA3C, kRewardXpSlot = kPlayerVtbl + 0x488;
using RewardXpFn = void(__thiscall*)(void*, int);
struct SharedXpState {
    RewardXpFn orig = nullptr;
    bool tried = false, live = false, applying = false;
    sharedxp::Outbox out;
    fnvcoop::MuxTransport* mux = nullptr;
    std::vector<int32_t> inbox;
    uint64_t earned = 0, sent = 0, received = 0, applied = 0, relayed = 0, xpOut = 0, xpIn = 0;
    std::map<uint8_t, std::pair<int32_t, uint64_t>> totals;   // slot -> (that player's whole XP, when heard)
    sharedxp::CatchUp catchUp;
    uint64_t nextTotal = 0, caughtUp = 0, xpCaughtUp = 0;
} g_xp;
bool PlayerXpSEH(int32_t* out) {   // the player's whole XP: a float at PlayerCharacter+0x3D8 (read in game: RewardXP 10 moved it 1 -> 11)
    __try {
        const uint32_t p = *reinterpret_cast<const uint32_t*>(0x011DEA3C);
        if (!p) return false;
        *out = static_cast<int32_t>(*reinterpret_cast<const float*>(p + 0x3D8));
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
void __fastcall RewardXpThunk(void* self, void*, int xp) {
    g_xp.orig(self, xp);
    if (xp > 0 && g_xp.live && !g_xp.applying && !nvhooks::applying() && !nvhooks::inRewardXpCommand()) {
        g_xp.out.earned(xp);
        ++g_xp.earned;
    }
}
bool InstallXpHookSEH() {
    __try {
        uint32_t* slot = reinterpret_cast<uint32_t*>(kRewardXpSlot);
        const void* player = *reinterpret_cast<void**>(0x011DEA3C);
        if (!player || *reinterpret_cast<const uint32_t*>(player) != kPlayerVtbl || !*slot) return false;
        if (*slot == static_cast<uint32_t>(reinterpret_cast<uintptr_t>(&RewardXpThunk))) return true;
        DWORD old = 0;
        if (!VirtualProtect(slot, 4, PAGE_READWRITE, &old)) return false;
        g_xp.orig = reinterpret_cast<RewardXpFn>(*slot);   // whatever is there (the game's, or another mod's that calls it)
        *slot = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(&RewardXpThunk));
        VirtualProtect(slot, 4, old, &old);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool ApplyXpSEH(int32_t xp) {
    __try {
        void* player = *reinterpret_cast<void**>(0x011DEA3C);
        if (!player || !g_xp.orig) return false;
        g_xp.applying = true;
        g_xp.orig(player, xp);
        g_xp.applying = false;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        g_xp.applying = false;
        return false;
    }
}
// ---- doors: where a player's own door use ends up -> every other player ----
// NV:MP keeps a door's state on its server, it takes that state only from a real activation on the copy of the cell's "owner" (whoever is
// first in, either player), and it puts the other copy back to it. A door one player opened slammed shut when they were not the owner; a
// SetOpenState on the owner's copy changed nothing for the server; two activations (one per copy) counted as two toggles (2026-11-12).
// So: the opener's game sends where the door ended up, the others set their copy quietly (SetOpenState). If the opener's own copy is put back
// within 3 s, the opener is not the owner: it asks the others for ONE real activation ("by activation"), which the owner's copy makes from
// the opposite state, so it lands right and the server has it, and NV:MP brings the opener's copy along.
//   [0xD9][ver=1][plugin: u8 len][bytes][localId u32 LE][want u8: 0 closed, 1 open, 2 closed by activation, 3 open by activation]
constexpr uint8_t kDoorChannel = 0xD9;
struct DoorMsg { fnvcoop::FormRef ref; bool open = false; bool byActivation = false; };
struct DoorSyncState {
    fnvcoop::MuxTransport* mux = nullptr;
    std::vector<DoorMsg> outbox, inbox;
    struct Recheck { uint32_t ref = 0; bool open = false; uint64_t at = 0; };
    std::vector<Recheck> rechecks;   // this player's own door uses, looked at again 3 s later
    std::vector<std::pair<uint32_t, uint64_t>> activateAt;   // owner activations waiting for the door to read the opposite state
    uint64_t sent = 0, received = 0, applied = 0, activated = 0, alreadyThere = 0, notLoaded = 0, askedOwner = 0;
} g_doors;
fnvcoop::Bytes EncodeDoor(const DoorMsg& m) {
    const uint8_t n = static_cast<uint8_t>(m.ref.plugin.size() < 255 ? m.ref.plugin.size() : 255);
    fnvcoop::Bytes b{kDoorChannel, 1, n};
    b.insert(b.end(), m.ref.plugin.begin(), m.ref.plugin.begin() + n);
    for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(m.ref.localId >> (8 * i)));
    b.push_back(static_cast<uint8_t>((m.open ? 1 : 0) + (m.byActivation ? 2 : 0)));
    return b;
}
bool DecodeDoor(const uint8_t* p, size_t n, DoorMsg* m) {
    if (n < 3 || p[0] != kDoorChannel || p[1] != 1 || p[2] == 0 || n != size_t(3) + p[2] + 5) return false;
    m->ref.plugin.assign(reinterpret_cast<const char*>(p + 3), p[2]);
    const uint8_t* q = p + 3 + p[2];
    m->ref.localId = uint32_t(q[0]) | uint32_t(q[1]) << 8 | uint32_t(q[2]) << 16 | uint32_t(q[3]) << 24;
    m->open = (q[4] & 1) != 0;
    m->byActivation = (q[4] & 2) != 0;
    return q[4] <= 3;
}
void SendDoor(const fnvcoop::Bytes& b, bool fromGuest, fnvcoop::PeerId except) {
    fnvcoop::MuxTransport* mux = g_rt->mux();
    if (!mux) return;
    if (g_rt->host()) {
        for (const auto& c : g_rt->host()->clients())
            if (c.joined && !(fromGuest && c.peer == except)) mux->send(c.peer, b);
    } else {
        mux->send(0, b);
    }
}
void TickDoorSync() {
    if (!g_rt || !g_rt->running() || g_rt->syncing()) { g_doors.outbox.clear(); return; }
    fnvcoop::MuxTransport* mux = g_rt->mux();
    if (!mux) return;
    if (mux != g_doors.mux) {
        g_doors.mux = mux;
        mux->registerChannel(kDoorChannel, [mux](fnvcoop::PeerId from, fnvcoop::Bytes&& d) {
            DoorMsg m;
            if (!g_rt || g_doors.mux != mux || !DecodeDoor(d.data(), d.size(), &m)) return;
            ++g_doors.received;
            if (g_rt->host()) SendDoor(d, true, from);   // a guest's door: the other guests get it too
            if (g_doors.inbox.size() < 64) g_doors.inbox.push_back(std::move(m));
        });
    }
    for (const auto& o : g_doors.outbox) { SendDoor(EncodeDoor(o), false, 0); ++g_doors.sent; }
    g_doors.outbox.clear();
    nvhooks::CaptureHub* hub = nvhooks::hub();
    for (const auto& in : g_doors.inbox) {
        const uint32_t ref = nvhooks::resolver().fromRef(in.ref);
        if (!ref || !Has3DSEH(ref) || !hub) { ++g_doors.notLoaded; continue; }   // not here: NV:MP / its scripts set it when it loads
        const int st = nvhooks::doorOpenStateNow(ref);
        if (st < 1 || st > 4) { ++g_doors.notLoaded; continue; }
        const bool isOpen = st == 1 || st == 2;
        if (in.open && nvhooks::isLockedPlainDoor(ref)) ExecMuted(*hub, "Unlock", ref);   // they picked the lock on their copy
        if (in.byActivation) {   // the opener is not the owner: this copy makes the one real activation, from the opposite state
            // the opposite state first, the activation half a second later: in the same frame the door still read as before and the
            // activation turned it the wrong way (both copies ended shut)
            if (isOpen == in.open) {
                ExecMuted(*hub, in.open ? "SetOpenState 0" : "SetOpenState 1", ref);
                g_doors.activateAt.push_back({ref, GetTickCount64() + 500});
            } else {
                ExecMuted(*hub, "Activate player 1", ref);
            }
            ++g_doors.activated;
        } else {
            if (isOpen == in.open) { ++g_doors.alreadyThere; continue; }
            ExecMuted(*hub, in.open ? "SetOpenState 1" : "SetOpenState 0", ref);
            ++g_doors.applied;
        }
        if (in.open) nvhooks::holdDoorForGuest(ref);   // its own auto-close script would shut it again (TenpennyDoorScript)
    }
    g_doors.inbox.clear();
    const uint64_t now = GetTickCount64();
    for (auto it = g_doors.activateAt.begin(); it != g_doors.activateAt.end();) {
        if (now < it->second) { ++it; continue; }
        // only once the door has come to rest in the opposite state (an activation while it still swings is lost), at most 4 s
        const int st = Has3DSEH(it->first) ? nvhooks::doorOpenStateNow(it->first) : 0;
        if ((st == 2 || st == 4) && now < it->second + 4000) { ++it; continue; }
        if (hub && (st == 1 || st == 3)) ExecMuted(*hub, "Activate player 1", it->first);
        it = g_doors.activateAt.erase(it);
    }
    for (auto it = g_doors.rechecks.begin(); it != g_doors.rechecks.end();) {
        if (now < it->at) { ++it; continue; }
        const int st = Has3DSEH(it->ref) ? nvhooks::doorOpenStateNow(it->ref) : 0;
        const bool isOpen = st == 1 || st == 2;
        fnvcoop::FormRef f;
        if (st >= 1 && st <= 4 && isOpen != it->open && nvhooks::resolver().toRef(it->ref, &f) && g_doors.outbox.size() < 64) {
            g_doors.outbox.push_back({f, it->open, true});   // put back on this copy: this player is not the owner, the owner activates
            ++g_doors.askedOwner;
        }
        it = g_doors.rechecks.erase(it);
    }
}
bool MDoors(const std::string&, std::string& out) {
    char b[240];
    std::snprintf(b, sizeof b, "{\"sent\":%llu,\"received\":%llu,\"applied\":%llu,\"activated\":%llu,\"alreadyThere\":%llu,\"notLoaded\":%llu,\"askedOwner\":%llu}",
                  (unsigned long long)g_doors.sent, (unsigned long long)g_doors.received, (unsigned long long)g_doors.applied,
                  (unsigned long long)g_doors.activated, (unsigned long long)g_doors.alreadyThere, (unsigned long long)g_doors.notLoaded,
                  (unsigned long long)g_doors.askedOwner);
    out = b;
    return true;
}
void QueueDoorUse(uint32_t refID, bool open) {
    fnvcoop::FormRef f;
    if (g_doors.outbox.size() < 64 && nvhooks::resolver().toRef(refID, &f)) g_doors.outbox.push_back({f, open, false});
    for (auto it = g_doors.rechecks.begin(); it != g_doors.rechecks.end();)   // a newer use of the same door replaces the older one
        it = it->ref == refID ? g_doors.rechecks.erase(it) : it + 1;
    if (g_doors.rechecks.size() < 32) g_doors.rechecks.push_back({refID, open, GetTickCount64() + 3000});
}
// ---- player stats (player_stats.h): a guest's SPECIAL / skills -> the host, for the scripts it runs for that guest (StatSwap) ----
namespace pstats = nvcoop::coop::pstats;
struct PlayerStatsState {
    fnvcoop::MuxTransport* mux = nullptr;
    uint64_t nextSend = 0, sent = 0, received = 0;
} g_pstats;
void TickPlayerStats() {
    if (!g_rt || !g_rt->running() || g_rt->syncing()) return;
    fnvcoop::MuxTransport* mux = g_rt->mux();
    if (!mux) return;
    if (mux != g_pstats.mux) {
        g_pstats.mux = mux;
        g_guestStats.clear();
        mux->registerChannel(pstats::kChannel, [mux](fnvcoop::PeerId from, fnvcoop::Bytes&& d) {
            pstats::Values v;
            if (!g_rt || g_pstats.mux != mux || !g_rt->host() || !pstats::decode(d.data(), d.size(), &v)) return;
            for (const auto& c : g_rt->host()->clients())
                if (c.peer == from) {
                    g_guestStats[static_cast<uint8_t>(c.slot)] = {std::move(v), GetTickCount64()};
                    ++g_pstats.received;
                    return;
                }
        });
    }
    if (g_rt->host() || !g_rt->client() || !g_rt->client()->joined()) return;   // only guests send theirs
    const uint64_t now = GetTickCount64();
    if (now < g_pstats.nextSend) return;
    g_pstats.nextSend = now + 5000;
    pstats::Values v;
    for (uint8_t av : pstats::trackedValues()) {
        const float x = ActorValueSEH(0x14, av);
        if (x >= 0) v[av] = x;
    }
    if (v.empty()) return;
    mux->send(0, pstats::encode(v));
    ++g_pstats.sent;
}
void TickSharedXp() {
    const bool host = g_rt && g_rt->host() != nullptr;
    g_xp.live = g_cfg.sharedXp && g_rt && g_rt->running() && !g_rt->syncing() && (host || (g_rt->client() && g_rt->client()->joined()));
    if (!g_xp.live) return;
    if (!g_xp.orig && !g_xp.tried) {
        g_xp.tried = InstallXpHookSEH();   // tried again every frame until the player exists
        if (g_xp.tried) nvcoop::Log("coop: shared XP on (Player RewardXP slot %08X -> %p)", kRewardXpSlot, (void*)g_xp.orig);
    }
    if (!g_xp.orig) return;
    fnvcoop::MuxTransport* mux = g_rt->mux();
    if (!mux) return;
    if (mux != g_xp.mux) {
        g_xp.mux = mux;
        mux->registerChannel(sharedxp::kChannel, [mux](fnvcoop::PeerId from, fnvcoop::Bytes&& d) {
            if (!g_rt || g_xp.mux != mux) return;
            uint8_t tslot = 0;
            int32_t total = 0;
            if (sharedxp::decodeTotal(d.data(), d.size(), &tslot, &total)) {
                if (g_rt->host())
                    for (const auto& c : g_rt->host()->clients()) {
                        if (c.peer == from) tslot = static_cast<uint8_t>(c.slot);   // a client speaks only for itself
                        else if (c.joined) mux->send(c.peer, d);
                    }
                g_xp.totals[tslot] = {total, GetTickCount64()};
                return;
            }
            sharedxp::Share s;
            if (!sharedxp::decode(d.data(), d.size(), &s)) return;
            ++g_xp.received;
            if (g_rt->host())   // a guest's XP: every other guest gets it too
                for (const auto& c : g_rt->host()->clients())
                    if (c.peer != from && c.joined) { mux->send(c.peer, d); ++g_xp.relayed; }
            if (g_xp.inbox.size() < 256) g_xp.inbox.push_back(s.xp);
        });
    }
    for (int32_t x : g_xp.inbox)
        if (ApplyXpSEH(x)) { ++g_xp.applied; g_xp.xpIn += static_cast<uint64_t>(x); }
    g_xp.inbox.clear();
    const uint64_t nowMs = GetTickCount64();
    const uint8_t self = host ? static_cast<uint8_t>(fnvcoop::kHostPlayer) : static_cast<uint8_t>(g_rt->client()->slot());
    int32_t mine = 0;
    if (nowMs >= g_xp.nextTotal && g_xp.out.pending() == 0 && PlayerXpSEH(&mine)) {   // every 5 s: this player's whole XP out, and catch up if behind
        g_xp.nextTotal = nowMs + 5000;
        const fnvcoop::Bytes tb = sharedxp::encodeTotal(self, mine);
        if (host) {
            for (const auto& c : g_rt->host()->clients())
                if (c.joined) mux->send(c.peer, tb);
        } else {
            mux->send(0, tb);
        }
        int32_t best = 0;
        for (const auto& kv : g_xp.totals)
            if (kv.first != self && nowMs - kv.second.second < 15000 && kv.second.first > best) best = kv.second.first;
        const int32_t add = best ? g_xp.catchUp.check(mine, best) : 0;
        if (add > 0 && ApplyXpSEH(add)) {
            ++g_xp.caughtUp;
            g_xp.xpCaughtUp += static_cast<uint64_t>(add);
            nvcoop::Log("coop: shared XP: this player had %d XP, the party %d (a respawn or an older save): +%d", mine, best, add);
        }
    }
    const int32_t x = g_xp.out.take(GetTickCount64());
    if (x <= 0) return;
    sharedxp::Share s;
    s.slot = host ? static_cast<uint8_t>(fnvcoop::kHostPlayer) : static_cast<uint8_t>(g_rt->client()->slot());
    s.xp = x;
    const fnvcoop::Bytes b = sharedxp::encode(s);
    if (host) {
        for (const auto& c : g_rt->host()->clients())
            if (c.joined) mux->send(c.peer, b);
    } else {
        mux->send(0, b);
    }
    ++g_xp.sent;
    g_xp.xpOut += static_cast<uint64_t>(x);
}
// ---- container contents (container_sync.h): what this player takes from / puts into a container or a corpse -> every other player ----
namespace cs = nvcoop::coop::containers;
constexpr uint32_t kContainerMenuPtr = 0x11D93F8, kContainerMenuRef = 0x74, kMenuIdContainer = 1008;   // ContainerMenu*, its TESObjectREFR* (read in game)
struct PendingChange { cs::Change c; ULONGLONG since = 0; };
struct ContainerSyncState {
    fnvcoop::MuxTransport* mux = nullptr;
    uint32_t openRef = 0;              // the container this player has open (0 = none)
    fnvcoop::FormRef openForm, openBase;   // openBase: what that ref is here (a leveled actor rolls per machine)
    cs::Items seen;                    // this PLAYER's inventory as last compared (see CompareOpenContainer)
    bool seenExact = false;
    uint64_t frame = 0;
    std::vector<cs::Change> inbox;
    std::vector<PendingChange> pending;   // for containers not loaded here yet
    std::set<fnvcoop::FormRef> known;     // containers whose contents everybody already shares this session (opened somewhere, or a change came in)
    uint64_t sent = 0, received = 0, applied = 0, deferred = 0, expired = 0, relayed = 0, itemsOut = 0, itemsIn = 0, inexact = 0, fullSent = 0, otherBase = 0, heldOpen = 0;
} g_cont;
uint32_t OpenContainerRefSEH() {
    __try {
        const uint32_t menu = *reinterpret_cast<const uint32_t*>(kContainerMenuPtr);
        if (!menu) return 0;
        const uint32_t ref = *reinterpret_cast<const uint32_t*>(menu + kContainerMenuRef);
        if (!ref) return 0;
        const uint8_t type = *reinterpret_cast<const uint8_t*>(ref + 4);
        if (type != kFormType_TESObjectREFR && type != kFormType_Character && type != kFormType_Creature) return 0;
        return *reinterpret_cast<const uint32_t*>(ref + 0x0C);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}
// Applies another player's change to this machine's copy of the container. false = not loaded here yet (try again later).
bool ApplyContainerChange(const cs::Change& c) {
    const uint32_t ref = nvhooks::resolver().fromRef(c.container);
    if (!ref || !Has3DSEH(ref)) return false;
    // Not while this player has it open: the loot screen keeps pointers to the container's item entries, our AddItem / RemoveItem freed
    // them, and the next "Take" crashed inside NV:MP's transfer hook (the friend's Super-Duper Mart / Springvale crashes, 10-28: always
    // a raider corpse and its ammo). The change waits in g_cont.pending until the menu is closed.
    if (TopMenuIdSEH() == kMenuIdContainer && OpenContainerRefSEH() == ref) { ++g_cont.heldOpen; return false; }
    g_cont.known.insert(c.container);
    fnvcoop::FormRef base;
    if (!c.base.plugin.empty() && nvhooks::resolver().toRef(BaseOfRefSEH(ref), &base) && !(base == c.base)) {
        ++g_cont.otherBase;   // a leveled actor that rolled something else here: its loot is not that one's (dogs dropped mole rat meat)
        nvcoop::Log("coop: container %s is %s here, %s for the sender: keeping this copy's contents", nvhooks::refToStr(c.container).c_str(),
                    nvhooks::refToStr(base).c_str(), nvhooks::refToStr(c.base).c_str());
        return true;
    }
    cs::Items delta = c.delta;
    if (c.full) {   // the first opener's contents: this copy becomes exactly that (its own leveled roll goes)
        cs::Items own;
        bool exact = true;
        if (!nvcoop::loot::readRefContents(ref, own, exact)) return false;
        delta = cs::diff(own, c.delta);
        if (delta.empty()) { ++g_cont.applied; return true; }
    }
    nvhooks::ApplyScope scope;   // host-authored state: a guest's own gate would drop the console line
    for (const auto& kv : delta) {
        const uint32_t item = nvhooks::resolver().fromRef(kv.first);
        if (!item) continue;
        char line[96];
        std::snprintf(line, sizeof line, kv.second > 0 ? "AddItem %08X %d 1" : "RemoveItem %08X %d 1", item, kv.second > 0 ? kv.second : -kv.second);
        nvhooks::execConsoleLineOn(line, ref);
    }
    ++g_cont.applied;
    for (const auto& kv : delta) g_cont.itemsIn += static_cast<uint64_t>(kv.second > 0 ? kv.second : -kv.second);
    return true;
}
void SendContainerChange(const cs::Change& c) {
    fnvcoop::MuxTransport* mux = g_rt->mux();
    if (!mux) return;
    const fnvcoop::Bytes b = cs::encode(c);
    if (g_rt->host()) {
        for (const auto& cl : g_rt->host()->clients())
            if (cl.joined) mux->send(cl.peer, b);
    } else {
        mux->send(0, b);
    }
    ++g_cont.sent;
    for (const auto& kv : c.delta) g_cont.itemsOut += static_cast<uint64_t>(kv.second > 0 ? kv.second : -kv.second);
}
// While this player has a container open, their OWN inventory is compared every few frames: what they gained came out of the container, what
// they lost went into it. (Not the container's contents: on a guest NV:MP puts the host's copy of a container back at once, so a guest's take
// left the container looking unchanged while the guest kept the items: both players could loot everything.) A change goes out at once.
void CompareOpenContainer(bool closing) {
    cs::Items now;
    bool exact = true;
    if (!nvcoop::loot::readRefContents(0x14, now, exact)) return;
    if (!exact) ++g_cont.inexact;
    cs::Change c;
    c.slot = g_rt->host() ? static_cast<uint8_t>(fnvcoop::kHostPlayer) : static_cast<uint8_t>(g_rt->client()->slot());
    c.container = g_cont.openForm;
    c.base = g_cont.openBase;
    for (const auto& kv : cs::diff(g_cont.seen, now)) c.delta[kv.first] = -kv.second;   // the player's +n is the container's -n
    g_cont.seen = std::move(now);
    if (!c.delta.empty()) {
        SendContainerChange(c);
        nvcoop::Log("coop: container %s: %u item kind(s) changed by this player%s", nvhooks::refToStr(c.container).c_str(), (unsigned)c.delta.size(), closing ? " (closed)" : "");
    }
}
void TickContainerSync() {
    const bool host = g_rt && g_rt->host() != nullptr;
    const bool live = g_cfg.containerSync && g_rt && g_rt->running() && !g_rt->syncing() && (host || (g_rt->client() && g_rt->client()->joined()));
    if (!live) { g_cont.openRef = 0; return; }
    fnvcoop::MuxTransport* mux = g_rt->mux();
    if (!mux) return;
    if (mux != g_cont.mux) {
        g_cont.mux = mux;
        g_cont.known.clear();   // a new session: nothing is shared yet
        mux->registerChannel(cs::kChannel, [mux](fnvcoop::PeerId from, fnvcoop::Bytes&& d) {
            cs::Change c;
            if (!g_rt || g_cont.mux != mux || !cs::decode(d.data(), d.size(), &c)) return;
            ++g_cont.received;
            if (g_rt->host())   // a guest's change: every other guest gets it too
                for (const auto& cl : g_rt->host()->clients())
                    if (cl.peer != from && cl.joined) { mux->send(cl.peer, d); ++g_cont.relayed; }
            if (g_cont.inbox.size() < 256) g_cont.inbox.push_back(std::move(c));
        });
    }
    for (const cs::Change& c : g_cont.inbox) {
        if (ApplyContainerChange(c)) continue;
        if (g_cont.pending.size() < 512) { g_cont.pending.push_back({c, GetTickCount64()}); ++g_cont.deferred; }
    }
    g_cont.inbox.clear();
    if (++g_cont.frame % 5) return;
    if (!g_cont.pending.empty() && g_cont.frame % 60 == 0) {   // twice a second: containers that loaded meanwhile
        const ULONGLONG now = GetTickCount64();
        std::vector<PendingChange> keep;
        for (PendingChange& p : g_cont.pending) {
            if (ApplyContainerChange(p.c)) continue;
            if (now - p.since > 30ull * 60 * 1000) { ++g_cont.expired; continue; }
            keep.push_back(std::move(p));
        }
        g_cont.pending.swap(keep);
    }
    const uint32_t ref = TopMenuIdSEH() == kMenuIdContainer ? OpenContainerRefSEH() : 0;
    if (g_cont.openRef && ref != g_cont.openRef) { CompareOpenContainer(true); g_cont.openRef = 0; }   // closed (or another one opened)
    if (ref && ref != g_cont.openRef) {
        fnvcoop::FormRef f;
        bool exact = true;
        if ((ref >> 24) == 0xFF || !nvhooks::resolver().toRef(ref, &f) || !nvcoop::loot::readRefContents(0x14, g_cont.seen, exact)) return;   // created refs have no shared id
        g_cont.openRef = ref;
        g_cont.openForm = f;
        g_cont.openBase = fnvcoop::FormRef();
        nvhooks::resolver().toRef(BaseOfRefSEH(ref), &g_cont.openBase);
        if (!g_cont.known.count(f)) {   // first opened anywhere: everybody gets this copy's contents, so all loot the same items
            g_cont.known.insert(f);
            cs::Change full;
            full.slot = host ? static_cast<uint8_t>(fnvcoop::kHostPlayer) : static_cast<uint8_t>(g_rt->client()->slot());
            full.container = f;
            full.base = g_cont.openBase;
            full.full = true;
            bool cexact = true;
            if (nvcoop::loot::readRefContents(ref, full.delta, cexact) && cexact) {
                SendContainerChange(full);
                ++g_cont.fullSent;
            }
        }
        return;
    }
    if (g_cont.openRef) CompareOpenContainer(false);
}
// ---- respawn restore: NV:MP brings a dead player back as their LAST SAVE (place, level, XP, inventory) while the shared world goes on ----
// This machine keeps a snapshot of its own player while alive (inventory + XP, every few seconds) and, once the player is back after a death,
// puts the inventory back to the snapshot (items gained since the save return, items the save still had but the player had used are taken)
// and tops XP up (levels come back through the normal level-up menus). [coop] respawn=0 turns it off.
struct RespawnGuard {
    cs::Items items;
    int32_t xp = 0;
    bool haveSnap = false, wasDead = false, pending = false;
    uint64_t nextSnap = 0, aliveAt = 0, frame = 0;
    uint64_t deaths = 0, restores = 0, itemsBack = 0, itemsTaken = 0, xpBack = 0;
} g_resp;
bool PlayerDeadSEH(bool* dead) {   // Actor::lifeState (+0x108): 1 dying, 2 dead
    __try {
        const uint32_t p = *reinterpret_cast<const uint32_t*>(0x011DEA3C);
        if (!p) return false;
        const uint32_t st = *reinterpret_cast<const uint32_t*>(p + 0x108);
        *dead = st == 1 || st == 2;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
void CornerNotice(const char* text) {
    __try {
        reinterpret_cast<bool(__cdecl*)(const char*, UInt32, const char*, const char*, float, bool)>(0x7052F0)(text, 0, nullptr, nullptr, 6.0f, false);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
}
// TTW's doctors: the price line ("moderate lacerations... 75 caps", DoctorMedical) offers its "Here's your caps" / "I'll be fine" answers
// (DoctorMedical33YES / 66YES / 99YES / DoctorMedicalNO, FalloutNV.esm topics shared by both games' doctors) only once the player knows
// those topics. Nothing teaches them to a character that never met a Mojave doctor: Doc Church named his price and the talk went back to
// the main list, nobody could pay or be healed (2026-11-16; forcing the topic or AddTopic fixed it). Taught after every load / new game.
// Under NV:MP the first load sends LoadGame but no PostLoadGame: either arms it, and it runs once play has gone on for 5 s with no menu open
// (a LoadGame comes while the save still loads).
bool g_teachDoctorTopics = false;
uint64_t g_teachDoctorPlaySince = 0;
void TickDoctorTopics() {
    if (!g_teachDoctorTopics) return;
    const uint64_t now = GetTickCount64();
    if (TopMenuIdSEH() != 0) { g_teachDoctorPlaySince = 0; return; }
    if (!g_teachDoctorPlaySince) g_teachDoctorPlaySince = now;
    nvhooks::CaptureHub* hub = nvhooks::hub();
    if (!hub || now - g_teachDoctorPlaySince < 5000) return;
    g_teachDoctorTopics = false;
    g_teachDoctorPlaySince = 0;
    // + the radiation and addiction cures' answers ("Can you heal my radiation?" never went on, 2026-11-16)
    for (const char* t : {"DoctorMedical33YES", "DoctorMedical66YES", "DoctorMedical99YES", "DoctorMedicalNO", "DoctorRadiationYES",
                          "DoctorRadiationNO", "DoctorAddictionYES", "DoctorAddictionNO"})
        ExecMuted(*hub, (std::string("AddTopic ") + t).c_str(), 0);
    nvcoop::Log("coop: doctor answer topics taught to this player (TTW doctors' \"here's your caps\")");
}
void TickRespawnRestore() {
    const bool host = g_rt && g_rt->host() != nullptr;
    if (g_joinedSelf && host && g_rt->running() && TopMenuIdSEH() == 0) {   // once, in play (LoadConfig switched a "Join" to this PC to hosting)
        g_joinedSelf = false;
        CornerNotice("Co-op: you started \"Join\" with this PC's own address - this game HOSTS. Friends join you; next time use Host.");
    }
    if (!g_cfg.respawnRestore || !g_rt || !g_rt->running() || g_rt->syncing() || !(host || (g_rt->client() && g_rt->client()->joined()))) return;
    if (++g_resp.frame % 20) return;
    bool dead = false;
    if (!PlayerDeadSEH(&dead)) return;
    const uint64_t now = GetTickCount64();
    if (dead) {
        if (!g_resp.wasDead) { ++g_resp.deaths; nvcoop::Log("coop: respawn: this player died; %u item kind(s), %d XP kept to restore", (unsigned)g_resp.items.size(), g_resp.xp); }
        g_resp.wasDead = true;   // the snapshot stays as it was before the death
        return;
    }
    if (g_resp.wasDead) { g_resp.wasDead = false; g_resp.pending = g_resp.haveSnap; g_resp.aliveAt = now; }
    if (g_resp.pending) {
        if (now - g_resp.aliveAt < 4000 || TopMenuIdSEH() != 0) return;   // NV:MP is still putting the player back; or a menu is up
        g_resp.pending = false;
        cs::Items cur;
        bool exact = true;
        if (!nvcoop::loot::readRefContents(0x14, cur, exact)) return;
        const cs::Items delta = cs::diff(cur, g_resp.items);
        {
            nvhooks::ApplyScope scope;
            for (const auto& kv : delta) {
                const uint32_t item = nvhooks::resolver().fromRef(kv.first);
                if (!item) continue;
                char line[96];
                std::snprintf(line, sizeof line, kv.second > 0 ? "AddItem %08X %d 1" : "RemoveItem %08X %d 1", item, kv.second > 0 ? kv.second : -kv.second);
                nvhooks::execConsoleLineOn(line, 0x14);
                (kv.second > 0 ? g_resp.itemsBack : g_resp.itemsTaken) += static_cast<uint64_t>(kv.second > 0 ? kv.second : -kv.second);
            }
        }
        int32_t xpNow = 0, xpAdd = 0;
        if (PlayerXpSEH(&xpNow) && g_resp.xp > xpNow + sharedxp::kCatchUpSlack && g_xp.orig) {
            xpAdd = g_resp.xp - xpNow;
            if (ApplyXpSEH(xpAdd)) g_resp.xpBack += static_cast<uint64_t>(xpAdd);
            else xpAdd = 0;
        }
        ++g_resp.restores;
        nvcoop::Log("coop: respawn: restored %u item kind(s) and %d XP from before the death", (unsigned)delta.size(), xpAdd);
        if (!delta.empty() || xpAdd > 0) CornerNotice("Co-op: you came back at your last save. Your items and XP from before you died are back.");
        g_resp.nextSnap = now + 3000;
        return;
    }
    if (now < g_resp.nextSnap) return;   // alive: a fresh snapshot every 3 s
    g_resp.nextSnap = now + 3000;
    cs::Items snap;
    bool exact = true;
    int32_t xp = 0;
    if (nvcoop::loot::readRefContents(0x14, snap, exact) && PlayerXpSEH(&xp)) {
        g_resp.items = std::move(snap);
        g_resp.xp = xp;
        g_resp.haveSnap = true;
    }
}

// ---- autosave: NV:MP brings a dead player back as their last save, so a recent save makes a death cheap ----
// Every [coop] autosave minutes (default 5, 0 = off) this player's game is saved to CoopAutosave1..3 in turn, only at a calm moment: alive,
// not in combat, no menu or conversation open, no world sync or respawn restore running. Not calm: tried again 10 s later.
struct AutosaveState { uint64_t nextAt = 0, saves = 0, deferred = 0; int slot = 0; } g_autosave;
bool PlayerInCombat() {
#ifdef NVCOOP_WITH_WORLD
    double v = 0;
    return nvworld::queryCondition("IsInCombat", 0x14, 0, &v) && v != 0;
#else
    return false;
#endif
}
// Two real games ran out of address space (2026-11-16, 22:11 / 22:18): flat at 2.95 GB for minutes, then 15-30 MB a second until the
// 4 GB were gone, with the game's own heap (heap replacer) not growing. Every 2 s: the address space in use and the biggest Win32 heaps;
// once it grows 48 MB+ within 4 s, a line every 2 s with this session's counters, to tell which part keeps allocating.
struct MemWatch { uint64_t nextMs = 0, lastLogMs = 0; uint64_t prevUsed = 0, prev2Used = 0; uint32_t spikes = 0; uint64_t backlogDrops = 0; uint32_t run = 0; bool dumped = false; } g_mem;
void TickMemoryWatch() {
    const uint64_t now = GetTickCount64();
    if (now < g_mem.nextMs) return;
    g_mem.nextMs = now + 2000;
    MEMORYSTATUSEX ms{};
    ms.dwLength = sizeof ms;
    if (!GlobalMemoryStatusEx(&ms)) return;
    const uint64_t used = ms.ullTotalVirtual - ms.ullAvailVirtual;
    const bool spike = g_mem.prev2Used && used > g_mem.prev2Used + (48ull << 20);
    if (g_rt && g_rt->backlogDrops() > g_mem.backlogDrops) {
        g_mem.backlogDrops = g_rt->backlogDrops();
        nvcoop::Log("coop: memory: a player's network send queue passed %u MB: dropped that link to stop it eating memory (it reconnects). "
                    "Session: %s", (unsigned)(fnvcoop::EnetTransport::kDefaultMaxBacklogBytes >> 20), g_rt->statusJson().substr(0, 1500).c_str());
    }
    // The first sustained climb (3 fast checks in a row, 6 s; a cell load makes 1-2) gets a flight recording: the per-session desync
    // dumps stop after 8, so the real out-of-memory sessions (2026-11-16 22:11 / 22:18) left none of the minutes before the crash.
    g_mem.run = spike ? g_mem.run + 1 : 0;
    if (g_mem.run >= 3 && !g_mem.dumped && g_rt && g_rt->running()) {
        g_mem.dumped = true;
        if (auto* fr = nvcoop::flight::service().recorder()) {
            const std::string path = fr->dumpToDir(fnvcoop::flight::kDumpManual);
            nvcoop::Log("coop: memory: still growing fast after 6 s: flight recording %s", path.empty() ? "FAILED" : path.c_str());
        }
    }
    g_mem.prev2Used = g_mem.prevUsed;
    g_mem.prevUsed = used;
    if (!spike && now - g_mem.lastLogMs < 60000) return;
    g_mem.lastLogMs = now;
    if (spike) ++g_mem.spikes;
    HANDLE heaps[64];
    const DWORD nh = GetProcessHeaps(64, heaps);
    std::vector<std::pair<uint64_t, HANDLE>> sizes;
    uint64_t heapTotal = 0;
    for (DWORD i = 0; i < nh && i < 64; ++i) {
        HEAP_SUMMARY s{};
        s.cb = sizeof s;
        if (!HeapSummary(heaps[i], 0, &s)) continue;
        sizes.push_back({s.cbCommitted, heaps[i]});
        heapTotal += s.cbCommitted;
    }
    std::sort(sizes.begin(), sizes.end(), [](const auto& a, const auto& b) { return a.first > b.first; });
    char top[160] = {};
    size_t at = 0;
    for (size_t i = 0; i < sizes.size() && i < 3; ++i)
        at += std::snprintf(top + at, sizeof top - at, "%s%p%s %.0f MB", i ? ", " : "", sizes[i].second, sizes[i].second == GetProcessHeap() ? " (process)" : "",
                            sizes[i].first / 1048576.0);
    const nvcoop::AllocStats mine = nvcoop::allocStats();
    nvcoop::Log("coop: memory t=%llu s: %.0f MB of %.0f MB address space in use%s; %u Win32 heaps %.0f MB (%s); nvcoop itself %.1f MB (peak %.1f), "
                "its network buffers %.1f MB", now / 1000, used / 1048576.0, ms.ullTotalVirtual / 1048576.0, spike ? " GROWING FAST" : "", (unsigned)nh,
                heapTotal / 1048576.0, top, mine.live / 1048576.0, mine.peak / 1048576.0, mine.enetLive / 1048576.0);
    if (spike && g_rt && g_mem.spikes <= 40) nvcoop::Log("coop: memory: session %s", g_rt->statusJson().substr(0, 1500).c_str());
}
void TickAutosave() {
    const bool host = g_rt && g_rt->host() != nullptr;
    if (g_cfg.autosaveMinutes <= 0 || !g_rt || !g_rt->running() || !(host || (g_rt->client() && g_rt->client()->joined()))) { g_autosave.nextAt = 0; return; }
    const uint64_t now = GetTickCount64();
    const uint64_t every = static_cast<uint64_t>(g_cfg.autosaveMinutes) * 60000;
    if (!g_autosave.nextAt) { g_autosave.nextAt = now + every; return; }   // the first save a full interval after joining
    if (now < g_autosave.nextAt) return;
    bool dead = false;
    if (g_rt->syncing() || g_resp.pending || !PlayerDeadSEH(&dead) || dead || TopMenuIdSEH() != 0 || DialogPartnerSEH() || PlayerInCombat()) {
        g_autosave.nextAt = now + 10000;
        ++g_autosave.deferred;
        return;
    }
    g_autosave.slot = g_autosave.slot % 3 + 1;
    char line[48];
    std::snprintf(line, sizeof line, "Save CoopAutosave%d", g_autosave.slot);
    const bool ok = nvhooks::execConsoleLine(line);
    g_autosave.nextAt = now + every;
    if (ok) { ++g_autosave.saves; CornerNotice("Co-op: autosaved"); }
    nvcoop::Log("coop: autosave '%s': %s", line, ok ? "saved" : "FAILED");
}

// coop.respawn: respawn-restore counters (deaths seen, restores done, items given back / taken, XP given back).
bool MRespawn(const std::string&, std::string& out) {
    char b[300];
    std::snprintf(b, sizeof b, "{\"enabled\":%s,\"snapshot\":%s,\"items\":%u,\"xp\":%d,\"deaths\":%llu,\"restores\":%llu,\"itemsBack\":%llu,\"itemsTaken\":%llu,\"xpBack\":%llu,\"pending\":%s}",
                  g_cfg.respawnRestore ? "true" : "false", g_resp.haveSnap ? "true" : "false", (unsigned)g_resp.items.size(), g_resp.xp,
                  (unsigned long long)g_resp.deaths, (unsigned long long)g_resp.restores, (unsigned long long)g_resp.itemsBack,
                  (unsigned long long)g_resp.itemsTaken, (unsigned long long)g_resp.xpBack, g_resp.pending ? "true" : "false");
    out = b;
    return true;
}

// coop.containers: container-sync counters (sent / itemsOut = this player's changes, applied / itemsIn = the others', deferred = waited for a load).
// coop.autosave {now?}: autosave counters; now=true saves at the next calm moment (tests).
bool MAutosave(const std::string& params, std::string& out) {
    if (params.find("\"now\":true") != std::string::npos) g_autosave.nextAt = 1;
    char b[160];
    std::snprintf(b, sizeof b, "{\"minutes\":%d,\"saves\":%llu,\"deferred\":%llu,\"slot\":%d}", g_cfg.autosaveMinutes, (unsigned long long)g_autosave.saves,
                  (unsigned long long)g_autosave.deferred, g_autosave.slot);
    out = b;
    return true;
}

bool MContainers(const std::string&, std::string& out) {
    char b[400];
    std::snprintf(b, sizeof b, "{\"top\":%u,\"menuRef\":\"%08X\",\"enabled\":%s,\"open\":\"%08X\",\"sent\":%llu,\"itemsOut\":%llu,\"received\":%llu,\"applied\":%llu,\"itemsIn\":%llu,\"deferred\":%llu,\"pending\":%u,\"expired\":%llu,\"relayed\":%llu,\"inexact\":%llu,\"fullSent\":%llu,\"otherBase\":%llu,\"heldOpen\":%llu}",
                  TopMenuIdSEH(), OpenContainerRefSEH(), g_cfg.containerSync ? "true" : "false", g_cont.openRef, (unsigned long long)g_cont.sent, (unsigned long long)g_cont.itemsOut,
                  (unsigned long long)g_cont.received, (unsigned long long)g_cont.applied, (unsigned long long)g_cont.itemsIn, (unsigned long long)g_cont.deferred,
                  (unsigned)g_cont.pending.size(), (unsigned long long)g_cont.expired, (unsigned long long)g_cont.relayed, (unsigned long long)g_cont.inexact, (unsigned long long)g_cont.fullSent, (unsigned long long)g_cont.otherBase, (unsigned long long)g_cont.heldOpen);
    out = b;
    return true;
}

// coop.xp: shared-XP counters (earned = engine payouts seen here, xpOut / xpIn = XP sent / added from the others).
bool MSharedXp(const std::string&, std::string& out) {
    char b[480];
    std::snprintf(b, sizeof b, "{\"enabled\":%s,\"hooked\":%s,\"live\":%s,\"earned\":%llu,\"sent\":%llu,\"xpOut\":%llu,\"received\":%llu,\"applied\":%llu,\"xpIn\":%llu,\"relayed\":%llu,\"pending\":%lld,\"caughtUp\":%llu,\"xpCaughtUp\":%llu}",
                  g_cfg.sharedXp ? "true" : "false", g_xp.orig ? "true" : "false", g_xp.live ? "true" : "false", (unsigned long long)g_xp.earned,
                  (unsigned long long)g_xp.sent, (unsigned long long)g_xp.xpOut, (unsigned long long)g_xp.received, (unsigned long long)g_xp.applied,
                  (unsigned long long)g_xp.xpIn, (unsigned long long)g_xp.relayed, (long long)g_xp.out.pending(), (unsigned long long)g_xp.caughtUp,
                  (unsigned long long)g_xp.xpCaughtUp);
    out = b;
    return true;
}

void OnFrame() {
    if (!g_rt) TryStart();
    if (g_rt && g_rt->running() && !g_reloadSidecar.empty()) {
        std::string what;
        bool ok = g_rt->onGameLoaded(&g_reloadSidecar, &what);
        nvcoop::Log("coop: hot reload resumed the session (%s): %s", what.c_str(), ok ? "ok" : "not handled");
        g_reloadSidecar.clear();
    }
    if (g_rt) {
#ifdef NVCOOP_WITH_WORLD
        if (g_rt->host()) {   // NPC state captured by L12's hooks since the last frame -> every guest (and late joiners)
            for (const auto& we : nvworld::drainWorldEffects()) g_rt->pushWorldEffect(WorldEffectKey(we), nvworld::worldEffectToJson(we));
            ScanDeaths();
        } else if (g_worldApplier) {
            g_worldApplier->flushQueued();   // spawns that waited for their cell to load
            ScanDeaths();
            TickGuestChargen();
        }
#endif
        g_rt->frame();
#ifdef NVCOOP_WITH_UI
        TickTalkSubtitles();
#endif
        if (auto* h = g_rt->host()) {   // which guests are connected now: NV:MP characters are matched to them by rank (ghost_ref.h)
            std::vector<uint8_t> slots;
            for (const auto& c : h->clients()) if (c.joined) slots.push_back(static_cast<uint8_t>(c.slot));
            std::sort(slots.begin(), slots.end());
            nvmp::SetConnectedGuestSlots(slots);
        }
        TickSharedXp();
        TickPlayerStats();
        TickDoorSync();
        TickContainerSync();
        TickRespawnRestore();
        TickDoctorTopics();
        TickAutosave();
        TickMemoryWatch();
        AdoptWorkerThread(g_rt->workerHandle());   // a session (re)start or leave swaps the worker: keep the drain's ignore list current
#if defined(NVCOOP_WITH_ACTORS) && !defined(NVCOOP_WITH_LIVE)
        nvcoop::actors::glueFrame(g_rt.get());   // actor ownership: after the session ticked (it polls the mux; ours is a channel on it)
#endif
        // v1 gameplay block while the world-baseline transfer runs (L5 DisablePlayerControls path; idempotent)
        if (g_rt->client() && g_rt->client()->rejected() && !g_rejectLogged) {
            g_rejectLogged = true;
            nvcoop::Log("coop: JOIN REJECTED: %s", g_rt->client()->rejectText().c_str());
            nvcoop::PushEvent("coop.rejected", std::string("{\"reason\":") + std::to_string((int)g_rt->client()->rejectReason()) + "}");
        }
        if (g_passStage >= 0 && !g_passSent && nvhooks::localQuestStage(g_passQuest) >= g_passStage) {
            g_passSent = true;
            nvcoop::Log("coop: local player reached the scenepass stage -> ScenePass");
            g_rt->scenePass(g_passQuest);
        }
        bool lock = g_rt->syncing() || g_scriptedLock;
        {   // report the player's cell every 30 frames (party tether / presence): host -> rules engine, client -> ReportCell action
            static uint64_t frames = 0;
            static uint32_t lastCell = 0;
            if (++frames % 30 == 0) {
                PlayerCharacter* pc = g_thePlayer ? *g_thePlayer : nullptr;
                uint32_t cell = pc && pc->parentCell ? pc->parentCell->refID : 0;
                if (cell && cell != lastCell) {
                    fnvcoop::FormRef ref;
                    if (nvhooks::resolver().toRef(cell, &ref)) { lastCell = cell; g_rt->reportCell(ref); }
                }
            }
        }
        static bool lastLock = false;
        nvhooks::SetPlayerControlsLocked(lock);
        if (lock != lastLock) {
            lastLock = lock;
            nvcoop::PushEvent("coop.controlsLocked", lock ? "{\"locked\":true}" : "{\"locked\":false}");
            nvcoop::Log("coop: gameplay controls %s (baseline sync)", lock ? "LOCKED" : "unlocked");
        }
    }
#ifdef NVCOOP_WITH_LIVE
    // The live layer (actors / avatars / combat / spawns, each behind its [live] switch): after the runtime ticked, because that is what
    // polls the mux and delivers the datagrams its channels receive. Also runs with no session (post-load purges).
    nvcoop::livewire::glueFrame(g_rt.get(), NowMs());
#endif
#ifdef NVCOOP_WITH_COMPANIONS
    nvcoop::social::glueFrame(g_rt.get());       // after the live layer (needs actor ownership); null runtime = session gone
#endif
#ifdef NVCOOP_WITH_AREADIGEST
    nvcoop::areadigest::glueFrame(g_rt.get());   // after the session polled the mux; no-op unless enabled
#endif
}

bool WriteBytes(const std::string& path, const fnvcoop::Bytes& b) {
    HANDLE h = CreateFileA(path.c_str(), GENERIC_WRITE, 0, nullptr, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, nullptr);
    if (h == INVALID_HANDLE_VALUE) return false;
    DWORD n = 0;
    bool ok = WriteFile(h, b.data(), (DWORD)b.size(), &n, nullptr) && n == b.size();
    CloseHandle(h);
    return ok;
}

// NVSE passes the file name as given ("autosave_x.fos"), sometimes a full path: resolve against the game's Saves dir
std::string ResolveSavePath(const std::string& p) {
    if (p.size() > 1 && (p[1] == ':' || p[0] == '\\' || p[0] == '/')) return p;
    if (p.find_first_of("\\/") != std::string::npos) return p;
    return "Saves\\" + p;
}
std::string g_loadPath;   // .fos path announced by PreLoadGame (PostLoadGame carries none)

void OnMessage(uint32_t type, void* data, uint32_t len) {
#ifdef NVCOOP_WITH_LIVE
    nvcoop::livewire::glueMessage(type, data, len);   // PreLoad / PostLoad / NewGame purge stand-ins and puppets; Exit ends the session part
#endif
    // The in-game lobby (main.cpp, [coop] menu=1) writes the player's pick into nvcoop.ini at the main menu and only then sends
    // DeferredInit: read [coop] again so the session starts with that role and address.
    if (type == NVSEMessagingInterface::kMessage_DeferredInit && !g_rt) {
        g_cfg = Config{};
        g_cfgLoaded = false;
        g_failed = false;
        g_failMsg.clear();
        LoadConfig();
    }
    // Save/load: sidecar `<save>.fos.fnvcoop` next to the save (L4 round 6). Host sidecar = shared world + epoch + token map.
    if (type == NVSEMessagingInterface::kMessage_SaveGame && data && g_rt && g_cfg.baseline) {
        std::string path = ResolveSavePath(std::string(static_cast<const char*>(data), len ? strnlen(static_cast<const char*>(data), len) : strlen(static_cast<const char*>(data))));
        fnvcoop::Bytes b = g_rt->saveSidecar();
        if (!b.empty() && WriteBytes(path + ".fnvcoop", b)) nvcoop::Log("coop: wrote sidecar %s.fnvcoop (%u bytes)", path.c_str(), (unsigned)b.size());
        else nvcoop::Log("coop: no sidecar written for %s (empty or write failed)", path.c_str());
#ifdef NVCOOP_WITH_LIVE
        if (g_rt->host()) {   // the host's spawn-site table (which sites rolled what, who owns them) travels with its save
            fnvcoop::Bytes sp = nvcoop::livewire::glueSpawnSidecar();
            if (!sp.empty() && WriteBytes(path + ".fnvspawn", sp)) nvcoop::Log("coop: wrote spawn sidecar %s.fnvspawn (%u bytes)", path.c_str(), (unsigned)sp.size());
        }
#endif
    }
#ifdef NVCOOP_WITH_WORLD
    if (type == NVSEMessagingInterface::kMessage_PostLoadGame || type == NVSEMessagingInterface::kMessage_NewGame) { g_deaths.reset(); g_rekilled.clear(); g_rekillWin.clear(); ResetGuestChargen(); }   // corpses of a load are not news
#endif
    if (type == NVSEMessagingInterface::kMessage_PreLoadGame && data)
        g_loadPath = ResolveSavePath(std::string(static_cast<const char*>(data), len ? strnlen(static_cast<const char*>(data), len) : strlen(static_cast<const char*>(data))));
    if (type == NVSEMessagingInterface::kMessage_PostLoadGame && data && g_rt && g_rt->running() && g_cfg.baseline) {
        std::string raw = ReadWholeFile(g_loadPath + ".fnvcoop");
        fnvcoop::Bytes side(raw.begin(), raw.end());
        std::string what;
        bool ok = g_rt->onGameLoaded(&side, &what);
        nvcoop::Log("coop: game loaded (%s): %s -> %s", g_loadPath.c_str(), what.c_str(), ok ? "ok" : "not handled");
        nvcoop::PushEvent("coop.loaded", std::string("{\"ok\":") + (ok ? "true" : "false") + "}");
#ifdef NVCOOP_WITH_LIVE
        if (g_rt->host()) {   // (the live hub already purged the world-bound state of the old world in glueMessage above)
            std::string sraw = ReadWholeFile(g_loadPath + ".fnvspawn");
            if (!sraw.empty()) nvcoop::livewire::glueSpawnSidecarLoaded(fnvcoop::Bytes(sraw.begin(), sraw.end()));
        }
#endif
    }
    // LoadGame (a save's data is being read) counts too: NV:MP loads its world without the PostLoadGame message, and its guests never joined
    if ((type == NVSEMessagingInterface::kMessage_PostLoadGame && data) || type == NVSEMessagingInterface::kMessage_NewGame ||
        type == NVSEMessagingInterface::kMessage_LoadGame)
        g_gameLoaded = true;
    if ((type == NVSEMessagingInterface::kMessage_PostLoadGame && data) || type == NVSEMessagingInterface::kMessage_LoadGame ||
        type == NVSEMessagingInterface::kMessage_NewGame) {
        g_teachDoctorTopics = true;
        g_teachDoctorPlaySince = 0;
    }
    if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) {
        if (g_rt) { nvhooks::SetPlayerControlsLocked(false); g_rt->stop(); }
    }
}

}  // namespace

NVCOOP_MODULE(l10_coop, "l10_coop", OnLoad, OnMessage, OnFrame);

// Hot reload (T3b, docs/tools/T3b-state.md). The session survives through the same sidecar a save/load uses: the host keeps its
// world, epoch and token map, so rejoining clients are recognised. The transport is closed and reopened (ENet cannot adopt a
// socket), so peers see a short disconnect and rejoin. The client token is carried over so a client keeps its slot.
namespace {
void ReloadShutdown() {
#ifdef NVCOOP_WITH_COMPANIONS
    nvcoop::social::glueFrame(nullptr);         // session gone
#endif
#ifdef NVCOOP_WITH_AREADIGEST
    nvcoop::areadigest::glueFrame(nullptr);
#endif
#ifdef NVCOOP_WITH_LIVE
    nvcoop::livewire::glueShutdown();   // puppets down, ActorSync dropped, before the runtime (and its mux) goes away
#endif
    if (g_rt) {
        nvhooks::SetPlayerControlsLocked(false);
        g_rt->stop();
    }
    g_rt.reset();
    AdoptWorkerThread(nullptr);   // joined by stop(): off the drain's ignore list
    g_applier.reset();
#ifdef NVCOOP_WITH_WORLD
    g_worldApplier.reset();
#endif
}
}  // namespace

NVCOOP_RELOAD_STATE(l10_coop, "l10_coop", 1, 50,
    /*canUnload*/ [] { return g_rt && g_rt->syncing() ? std::string("world baseline transfer in progress") : std::string(); },
    /*save*/ [](nvreload::Writer& w) {
        w.boolean(g_gameLoaded);
        w.boolean(g_passSent);
        w.boolean(g_scriptedLock);
        bool live = g_rt && g_rt->running();
        w.boolean(live);
        w.u64(g_rt ? g_rt->config().token : g_cfg.token);
        fnvcoop::Bytes side;
        if (live && g_cfg.baseline) side = g_rt->saveSidecar();
        w.bytes(side);
        return true;
    },
    /*restore*/ [](uint32_t, nvreload::Reader& r) {
        g_gameLoaded = r.boolean();
        g_passSent = r.boolean();
        g_scriptedLock = r.boolean();
        bool live = r.boolean();
        uint64_t token = r.u64();
        fnvcoop::Bytes side = r.bytes();
        if (live && token && !g_cfg.token) g_cfg.token = token;
        g_reloadSidecar = std::move(side);
        return true;
    },
    /*migrate*/ nullptr,
    /*shutdown*/ [] { ReloadShutdown(); },
    /*fresh*/ nullptr);
