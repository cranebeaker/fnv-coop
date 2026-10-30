// nvcoop module for the live sync parity checker (x86, compiled into nvcoop.dll). NOTHING here has run in a game; the logic behind it is tested game-free.
//   * reads nvcoop.ini [parity] (parity_config.h): enabled=0 leaves the module doing nothing but answer parity.status {"enabled":false}
//   * the engine source (GameSource below) reads the game through the avatar layer's WinBackend (a second instance with no console, so it can only READ), the
//     avatar layer's puppet directory and a few raw reads copied from plugin/animprobe. Nothing is written, no script is run.
//   * once per frame: the service (parity_service.h) samples, writes nvcoop_parity.jsonl, and a guest sends to / the host compares on channel 0x9E
//   * bridge methods: parity.status, parity.sample (one sample now as a JSON line)
#include <Windows.h>

#include <cctype>
#include <chrono>
#include <cstdint>
#include <cstring>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "PluginAPI.h"
#include "avatar/avatar_api.h"
#include "avatar/engine/engine_pins.h"
#include "avatar/engine/win/game_backend_win.h"
#include "common/nvcoop_api.h"
#include "coop_runtime.h"
#include "parity/parity_anim.h"
#include "parity/parity_service.h"
#include "reload/reload_state.h"

#include "nvse/GameData.h"
#include "nvse/GameForms.h"

namespace {

namespace P = ::nvcoop::avatar::pins;
namespace par = ::nvcoop::parity;
using nvcoop::avatar::engine::Cond;
using nvcoop::avatar::engine::WinBackend;

// .text / .rdata of FalloutNV.exe 1.4.0.525 (the same bounds game_backend_win.cpp uses to refuse a vtable that is not the game's)
constexpr uintptr_t kTextLo = 0x401000, kTextHi = 0xFDE600, kRdataLo = 0xFDF000, kRdataHi = 0x1183000;
constexpr uint32_t kProcGetSitSleep = 0x12F;   // BaseProcess::GetSitSleepState (plugin/actors/anim/anim_pins.h); values are the xNVSE kSitSleepState_* enum
constexpr int kAvHealth = 16;

bool SafeRead(uintptr_t addr, void* out, size_t n) {
    __try {
        std::memcpy(out, reinterpret_cast<const void*>(addr), n);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool CallSlot(void* obj, uint32_t slot, uint32_t& ret) {
    __try {
        if (!obj) return false;
        uintptr_t vt = *reinterpret_cast<uintptr_t*>(obj);
        if (vt < kRdataLo || vt >= kRdataHi) return false;
        uintptr_t fn = *reinterpret_cast<uintptr_t*>(vt + 4u * slot);
        if (fn < kTextLo || fn >= kTextHi) return false;
        ret = reinterpret_cast<uint32_t(__thiscall*)(void*)>(fn)(obj);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// GameDaysPassed, read the way plugin/coop/live/live_glue.cpp reads it. No C++ objects with destructors in the frame.
bool GameDaysSEH(float* out) {
    static TESGlobal* cached = nullptr;
    __try {
        if (cached && cached->name.m_data && _stricmp(cached->name.m_data, "GameDaysPassed") == 0) {
            *out = cached->data;
            return true;
        }
        DataHandler* dh = DataHandler::Get();
        if (!dh) return false;
        for (auto it = dh->globalList.Begin(); !it.End(); ++it) {
            TESGlobal* g = it.Get();
            if (g && g->name.m_data && _stricmp(g->name.m_data, "GameDaysPassed") == 0) {
                cached = g;
                *out = g->data;
                return true;
            }
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        cached = nullptr;
    }
    return false;
}

uint64_t monoMs() {
    return static_cast<uint64_t>(std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now().time_since_epoch()).count());
}

uint64_t wallMs() {
    FILETIME ft;
    GetSystemTimeAsFileTime(&ft);
    const uint64_t t = (uint64_t(ft.dwHighDateTime) << 32) | ft.dwLowDateTime;   // 100 ns since 1601
    return t / 10000ull - 11644473600000ull;
}

// ---- the engine source ----------------------------------------------------------------------------------------------------------------------------------

class GameSource : public par::IParitySource {
public:
    GameSource() : be_(nullptr) {}   // no console: RunLine answers -2, so this backend can only read

    void setLocalSlot(uint8_t s) { slot_ = s; }
    uint8_t localSlot() override { return slot_; }

    bool ready() override {
        if (!be_.PinOk("LookupFormByID")) return false;
        uintptr_t p = 0;
        return SafeRead(P::kThePlayerPtr, &p, sizeof p) && p != 0;
    }

    bool player(par::RawPlayer& o) override {
        nvcoop::avatar::engine::RefInfo ri;
        if (!be_.ReadRef(0x14, ri) || !ri.exists) return false;
        o.x = ri.pos.x; o.y = ri.pos.y; o.z = ri.pos.z;
        o.cell = ri.cell;
        nvcoop::avatar::engine::CellInfo ci;
        o.ws = be_.ReadCell(ri.cell, ci) ? ci.world : 0;
        o.menu = be_.MenuOpen();
        return true;
    }

    void actors(std::vector<par::RawActor>& out) override {
        out.clear();
        std::vector<uint32_t> cells;
        be_.LoadedCells(cells);
        std::vector<nvcoop::avatar::engine::ActorEntry> list;
        for (uint32_t c : cells) {
            be_.ListActors(c, list);
            for (const auto& e : list) {
                par::RawActor a;
                if (actor(e.ref, a)) out.push_back(a);
                if (out.size() >= 1500) return;
            }
        }
    }

    bool actor(uint32_t ref, par::RawActor& o) override {
        nvcoop::avatar::engine::RefInfo ri;
        if (!be_.ReadRef(ref, ri) || !ri.exists || ri.deleted) return false;
        o.ref = ref;
        o.base = ri.base;
        o.x = ri.pos.x; o.y = ri.pos.y; o.z = ri.pos.z;
        o.disabled = ri.disabled;
        return true;
    }

    bool groups(uint32_t ref, bool isPlayer, std::vector<int>& out) override {
        out.clear();
        nvcoop::avatar::engine::AnimRead ar;
        if (!be_.ReadAnim(ref, ar) || !ar.valid) return false;
        for (int g : ar.groups) if (g >= 0) out.push_back(g);
        if (isPlayer)   // a player in first person plays his attacks on the other AnimData sets
            for (const auto& set : ar.alt) if (set.valid) for (int g : set.groups) if (g >= 0) out.push_back(g);
        return true;
    }

    bool state(uint32_t ref, bool isPlayer, par::RawState& o) override {
        o = par::RawState{};
        nvcoop::avatar::engine::AnimRead ar;
        if (be_.ReadAnim(ref, ar) && ar.valid) {
            o.ok = true;
            groups(ref, isPlayer, o.groups);
            o.dead = ar.dead;
            o.weaponOut = ar.weaponOut;
        }
        double hp = 0;
        if (be_.Eval(Cond::GetActorValue, ref, kAvHealth, hp) && hp > -1e6 && hp < 1e6) {
            o.hpOk = true;
            o.hp = static_cast<float>(hp);
            if (o.hp <= 0) o.dead = true;
        }
        uint32_t sit = 0;
        if (sitSleep(ref, sit)) {
            o.seatedOk = true;
            // going into furniture or in it (sitting or sleeping); WantToStand (5) and WantToWake (10) are already on the way out
            o.seated = (sit >= 1 && sit <= 4) || (sit >= 6 && sit <= 9);
        }
        return true;
    }

    bool look(uint32_t ref, par::RawLook& o) override {
        o = par::RawLook{};
        nvcoop::avatar::Appearance ap;
        if (!be_.ReadAppearance(ref, ap) || !ap.known()) return false;
        o.ok = true;
        o.look.known = true;
        o.look.race = ap.race;
        o.look.female = ap.female;
        o.look.hair = ap.hair;
        o.look.eyes = ap.eyes;
        o.look.faceHash = ap.faceHash;
        return true;
    }

    void puppets(std::vector<std::pair<uint8_t, uint32_t>>& out) override {
        out.clear();
        nvcoop::avatar::forEachPuppet([&](nvcoop::avatar::Slot s, nvcoop::avatar::RefId r) { out.emplace_back(static_cast<uint8_t>(s), static_cast<uint32_t>(r)); });
    }

    double gameTime() override {
        float d = 0;
        return GameDaysSEH(&d) ? static_cast<double>(d) : 0.0;
    }

private:
    bool sitSleep(uint32_t ref, uint32_t& out) {
        // the process pointer: Actor + 0x68, found through the reference table the backend already uses
        nvcoop::avatar::engine::RefInfo ri;
        if (!be_.ReadRef(ref, ri) || !ri.exists || !ri.isActor) return false;
        uintptr_t actorPtr = ri.isPlayer ? playerPtr() : lookup(ref);
        if (!actorPtr) return false;
        uintptr_t proc = 0;
        if (!SafeRead(actorPtr + P::kActorBaseProcess, &proc, sizeof proc) || !proc) return false;
        return CallSlot(reinterpret_cast<void*>(proc), kProcGetSitSleep, out);
    }
    static uintptr_t playerPtr() {
        uintptr_t p = 0;
        SafeRead(P::kThePlayerPtr, &p, sizeof p);
        return p;
    }
    static uintptr_t lookup(uint32_t id) {
        __try {
            return reinterpret_cast<uintptr_t>(reinterpret_cast<TESForm*(__cdecl*)(UInt32)>(P::LookupFormByID.addr)(id));
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return 0;
        }
    }

    WinBackend be_;
    uint8_t slot_ = par::kNoSlot;
};

// ---- module state -----------------------------------------------------------------------------------------------------------------------------------------

struct State {
    par::Settings set;
    bool configured = false;
    std::string dir;
    std::unique_ptr<GameSource> src;
    std::unique_ptr<par::ParityService> svc;
};
State g;

std::string moduleDir() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&moduleDir), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    const size_t s = p.find_last_of("\\/");
    return s == std::string::npos ? std::string() : p.substr(0, s + 1);
}

par::IniGetter makeGetter() {
    const std::string ini = g.dir + "nvcoop.ini";
    return [ini](const char* section, const char* key, const char* def) -> std::string {
        std::string env = "NVCOOP_";
        for (const char* p = section; *p; ++p) env += static_cast<char>(std::toupper(static_cast<unsigned char>(*p)));
        env += '_';
        for (const char* p = key; *p; ++p) env += static_cast<char>(std::toupper(static_cast<unsigned char>(*p)));
        char ebuf[256] = {};
        if (GetEnvironmentVariableA(env.c_str(), ebuf, sizeof ebuf) > 0) return ebuf;
        char buf[256] = {};
        GetPrivateProfileStringA(section, key, def, buf, sizeof buf, ini.c_str());
        return buf;
    };
}

void frameInfoFrom(nvcoop::coop::Runtime* rt, par::FrameInfo& f) {
    f.nowMs = monoMs();
    f.wallMs = wallMs();
    if (!rt || !rt->running()) return;
    f.active = true;
    f.mux = rt->mux();
    if (rt->host()) {
        f.isHost = true;
        f.self = 0;
        for (const auto& cv : rt->host()->clients()) f.clients.push_back(par::Link{static_cast<uint8_t>(cv.slot), cv.peer, cv.joined});
    } else if (auto* cl = rt->client()) {
        f.isHost = false;
        f.self = cl->joined() ? static_cast<uint8_t>(cl->slot()) : par::kNoSlot;
        f.hostLinkUp = cl->joined() && !cl->baselineActive();
    } else {
        f.active = false;
    }
}

void stop() {
    g.svc.reset();
    g.src.reset();
}

void onFrame() {
    if (!g.configured || !g.set.enabled) return;
    par::FrameInfo f;
    frameInfoFrom(nvcoop::coop::Runtime::active(), f);
    if (!g.svc) {
        if (!f.active) return;
        g.src.reset(new GameSource());
        g.svc.reset(new par::ParityService(g.set, *g.src, [](const std::string& l) { nvcoop::Log("%s", l.c_str()); },
                                           g.set.file.empty() ? std::string() : g.dir + g.set.file));
        nvcoop::Log("parity: running, samples go to %s", g.set.file.empty() ? "(no file)" : (g.dir + g.set.file).c_str());
    }
    g.src->setLocalSlot(f.isHost ? 0 : f.self);
    g.svc->frame(f);
}

void onLoad(void*) {
    if (g.configured) return;
    g.dir = moduleDir();
    g.set = par::readSettings(makeGetter());
    g.configured = true;
    nvcoop::Log("parity: %s", par::describe(g.set).c_str());
    if (!g.set.warnings.empty()) nvcoop::Log("parity: %s", g.set.warnings.c_str());
    nvcoop::RegisterMethodClosure("parity.status", [](const std::string&, std::string& out) {
        if (!g.set.enabled) { out = "{\"enabled\":false}"; return true; }
        out = g.svc ? g.svc->statusJson() : "{\"enabled\":true,\"up\":false}";
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("parity.sample", [](const std::string&, std::string& out) {
        if (!g.svc) { out = "the parity checker is not running (parity enabled=0, or no co-op session yet)"; return false; }
        std::string line;
        if (!g.svc->takeNow(monoMs(), wallMs(), line)) { out = "no world to sample"; return false; }
        out = line;
        return true;
    }, true, 0);
}

void onMessage(uint32_t type, void*, uint32_t) {
    switch (type) {
        case NVSEMessagingInterface::kMessage_PostLoadGame:
        case NVSEMessagingInterface::kMessage_NewGame: stop(); break;   // a different world: rebuilt on the next session frame
        case NVSEMessagingInterface::kMessage_ExitGame:
        case NVSEMessagingInterface::kMessage_ExitGame_Console: stop(); break;
        default: break;
    }
}

}  // namespace

NVCOOP_MODULE(sync_parity, "sync_parity", onLoad, onMessage, onFrame);

// Hot reload: the service (and its file) is dropped before the core is replaced and rebuilt on the new core's first session frame.
NVCOOP_RELOAD_STATE(sync_parity, "sync_parity", 1, 49,
    /*canUnload*/ nullptr,
    /*save*/ [](nvreload::Writer& w) { w.boolean(g.set.enabled); return true; },
    /*restore*/ [](uint32_t, nvreload::Reader& r) { (void)r.boolean(); return true; },
    /*migrate*/ nullptr,
    /*shutdown*/ [] { stop(); },
    /*fresh*/ nullptr);
