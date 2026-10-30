// nvcoop module for threat (x86, compiled into nvcoop.dll). NOTHING here has run in a game.
//
// The engine side of plugin/threat (docs/threat.md): the IThreatPort over the actors engine (who is loaded, who owns it, what it fights, StopCombat /
// StartCombat), the avatar directory (the local player and the puppets are the players) and the engine's own "has it noticed" check; the combat styles
// the tactics change; the ini; the per-frame call. Everything it reads is behind SEH. It adds no engine address of its own:
//  * GetDetected 0x0059D6E0 (calling ref = the observer, param = the actor it may see; prologue 55 8B EC checked before every call) is the evaluator
//    plugin/companions already calls, with the same cdecl bool(thisObj, p1, p2, double* result) signature;
//  * the teammate flag is the byte at actor+0x18D (plugin/hooks/engine_hooks.cpp h_Teammate, Actor::SetPlayerTeammate 0x8BCA90);
//  * a base form's combat style: TESCreature::combatStyle (+0x140) and TESNPC +0x1D4 (xNVSE GameForms.h); the pointer is only used when the form it
//    points at has the combat style form type, so a wrong offset costs the tactics, never a crash;
//  * TESCombatStyle fields +0x18 coverSearchRadius, +0x1C takeCoverChance, +0x59 LRChance, +0xA8 csFlags (xNVSE GameForms.h).
// Switches in nvcoop.ini [threat] (docs/threat.md): enabled tick_ms fade_ms near_bonus near_range seen_bonus engaged_bonus switch_pct switch_abs min_on_target_ms
//   spread group_radius reach max_switches named infer_damage tactics tactics_cover tactics_radius_pct tactics_sidestep tactics_flee. Every default is the one in
//   threat_types.h Config / tactics.h TacticsConfig (the values and the reasons for them are there); nothing is repeated in this file.
#include <Windows.h>

#include <cstdio>
#include <cstring>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "GameForms.h"
#include "GameObjects.h"
#include "PluginAPI.h"
#include "actors/engine/actor_engine_api.h"
#include "actors/engine/engine_iface.h"
#include "avatar/avatar_api.h"
#include "common/nvcoop_api.h"
#include "engine_hooks.h"
#include "reload/reload_state.h"
#include "threat/tactics.h"
#include "threat/threat_director.h"
#include "threat/threat_glue.h"

namespace {

namespace eng = nvcoop::actors::engine;
namespace th = nvcoop::threat;

constexpr uint32_t kEvalGetDetected = 0x0059D6E0;
constexpr uint32_t kPlayerRef = 0x14;
constexpr uint32_t kTeammateFlag = 0x18D;
constexpr uint32_t kMaxLoaded = 400;

static_assert(th::kCsFlagFleeBasedOnPersonalSurvival == TESCombatStyle::kFlag_FleeBasedOnPersonalSurvival, "tactics.h flag value must match xNVSE GameForms.h");

uint64_t NowMs() { return static_cast<uint64_t>(GetTickCount64()); }

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    const size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

struct Win32Memory {
    bool read(uint32_t a, void* out, size_t n) {
        __try {
            std::memcpy(out, reinterpret_cast<const void*>(a), n);
            return true;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
    }
} g_mem;

struct Settings {
    th::Config cfg;
    th::TacticsConfig tactics;
    bool tacticsIni = false;     // [threat] tactics= : used only while no rebalance-mode query is registered
};

Settings g_set;
bool g_inited = false;
uint64_t g_evalCalls = 0, g_evalFaults = 0;
std::function<bool()> g_balancedQuery;
std::function<bool(uint8_t)> g_downedQuery;

void* LookupRef(uint32_t id) { return id ? nvhooks::resolver().lookup(id) : nullptr; }

// ---------------------------------------------------------------------------------------------------------------- guarded reads
using CondFn = bool(__cdecl*)(void* thisObj, void* p1, void* p2, double* result);
// An engine condition evaluator (as plugin/companions calls it). Refused when the entry no longer has its original prologue (somebody detoured it).
bool CallCondition(uint32_t addr, void* thisObj, void* p1, double* out) {
    unsigned char pro[3] = {};
    if (!g_mem.read(addr, pro, 3) || pro[0] != 0x55 || pro[1] != 0x8B || pro[2] != 0xEC) return false;
    ++g_evalCalls;
    __try {
        double r = 0;
        const bool ok = reinterpret_cast<CondFn>(addr)(thisObj, p1, nullptr, &r);
        *out = r;
        return ok;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        ++g_evalFaults;
        return false;
    }
}

bool ReadTeammate(void* actor) {
    uint8_t flag = 0;
    return actor && g_mem.read(reinterpret_cast<uint32_t>(actor) + kTeammateFlag, &flag, 1) && flag != 0;
}

// The combat style form of an actor's base form (0 = none / unreadable).
uint32_t StyleOf(void* actor) {
    __try {
        TESForm* base = static_cast<TESObjectREFR*>(actor)->baseForm;
        if (!base) return 0;
        TESCombatStyle* style = nullptr;
        if (base->typeID == kFormType_TESCreature) style = static_cast<TESCreature*>(base)->combatStyle;
        else if (base->typeID == kFormType_TESNPC) style = static_cast<TESNPC*>(base)->unk1D4;
        return reinterpret_cast<uint32_t>(style);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

class LiveStyles : public th::IStyleStore {
public:
    bool read(uint32_t style, th::StyleValues& out) override {
        __try {
            TESCombatStyle* s = reinterpret_cast<TESCombatStyle*>(style);
            if (!s || s->typeID != kFormType_TESCombatStyle) return false;
            out.coverSearchRadius = s->coverSearchRadius;
            out.takeCoverChance = s->takeCoverChance;
            out.lrChance = s->LRChance;
            out.csFlags = s->csFlags;
            return true;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
    }
    bool write(uint32_t style, const th::StyleValues& v) override {
        __try {
            TESCombatStyle* s = reinterpret_cast<TESCombatStyle*>(style);
            if (!s || s->typeID != kFormType_TESCombatStyle) return false;
            s->coverSearchRadius = v.coverSearchRadius;
            s->takeCoverChance = v.takeCoverChance;
            s->LRChance = v.lrChance;
            s->csFlags = v.csFlags;
            return true;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
    }
};

// ---------------------------------------------------------------------------------------------------------------- the port
class LivePort : public th::IThreatPort {
public:
    void players(std::vector<th::PlayerView>& out) override {
        out.clear();
        eng::IActorEngine* ae = eng::actorEngine();
        const nvcoop::avatar::IPuppetDirectory* dir = nvcoop::avatar::puppetDirectory();
        if (!ae || !dir || !ae->worldReady()) return;
        const nvcoop::avatar::Slot local = dir->LocalSlot();
        th::PlayerView v;
        if (local != nvcoop::avatar::kNoSlot && readPlayer(ae, local, kPlayerRef, v)) out.push_back(v);
        for (const auto& kv : dir->Live())
            if (kv.first != local && readPlayer(ae, kv.first, kv.second, v)) out.push_back(v);
    }

    void enemies(std::vector<th::EnemyView>& out) override {
        out.clear();
        eng::IActorEngine* ae = eng::actorEngine();
        if (!ae || !ae->worldReady()) return;
        std::vector<eng::RefId> refs;
        ae->loadedActors(refs, kMaxLoaded);
        for (eng::RefId r : refs) {
            if (r == kPlayerRef || nvcoop::avatar::isPuppet(r)) continue;
            const eng::OwnerRef o = eng::ownerOf(r);
            if (!o.known) continue;
            th::EnemyView e;
            e.ref = r;
            e.standIn = o.standIn;
            e.dead = o.dead;
            e.mine = o.mine && !o.pending && !o.standIn && !o.dead;
            if (e.mine) {
                eng::ActorRaw raw;
                if (!ae->read(r, raw) || !raw.valid || raw.disabled || raw.deleted) continue;
                e.dead = raw.dead();
                e.inCombat = raw.inCombat;
                e.combatTarget = raw.combatTarget;
                e.unique = raw.baseUnique;
                e.space = spaceOf(ae, raw.cell);
                e.pos = {raw.pos.x, raw.pos.y, raw.pos.z};
                e.teammate = ReadTeammate(LookupRef(r));
            }
            out.push_back(e);
        }
    }

    th::Sight sight(th::RefId enemy, th::RefId player) override {
        void* o = LookupRef(enemy);
        void* t = LookupRef(player);
        double v = 0;
        if (!o || !t || !CallCondition(kEvalGetDetected, o, t, &v)) return th::Sight::Unknown;
        return v > 0.5 ? th::Sight::Seen : th::Sight::Hidden;
    }

    bool health(th::RefId enemy, float& out) override {
        eng::IActorEngine* ae = eng::actorEngine();
        return ae && ae->actorValue(enemy, eng::kAvHealth, out);
    }

    // Its fight is stopped and started again on the player (StartCombat does nothing while it is fighting someone else).
    // When the new fight cannot be started the old one is put back, so a refused retarget never leaves an enemy standing idle.
    bool retarget(th::RefId enemy, th::RefId player) override {
        eng::IActorEngine* ae = eng::actorEngine();
        if (!ae) return false;
        eng::ActorRaw before;
        const bool had = ae->read(enemy, before) && before.valid && before.inCombat && before.combatTarget;
        ae->stopCombat(enemy);
        if (ae->startCombat(enemy, player)) return true;
        if (had) ae->startCombat(enemy, before.combatTarget);
        return false;
    }

    void epoch(eng::IActorEngine* ae) {
        const uint32_t e = ae->worldEpoch();
        if (e != epoch_) {
            epoch_ = e;
            spaces_.clear();
        }
    }

private:
    bool readPlayer(eng::IActorEngine* ae, nvcoop::avatar::Slot slot, eng::RefId ref, th::PlayerView& v) {
        eng::ActorRaw raw;
        if (!ref || !ae->read(ref, raw) || !raw.valid || raw.disabled || raw.deleted || !raw.cell) return false;
        v = th::PlayerView{};
        v.slot = slot;
        v.ref = ref;
        v.space = spaceOf(ae, raw.cell);
        v.pos = {raw.pos.x, raw.pos.y, raw.pos.z};
        v.yaw = raw.rot.z;
        v.present = true;
        v.weaponOut = raw.weaponOut;
        float hp = 1;
        v.downed = raw.dead() || (ae->actorValue(ref, eng::kAvHealth, hp) && hp <= 0) || (g_downedQuery && g_downedQuery(slot));
        return true;
    }

    // Positions compare within one worldspace or one interior cell (an exterior cell is one of many grid cells of its world).
    uint32_t spaceOf(eng::IActorEngine* ae, uint32_t cell) {
        epoch(ae);
        auto it = spaces_.find(cell);
        if (it != spaces_.end()) return it->second;
        eng::CellInfo ci;
        const uint32_t space = ae->cellInfo(cell, ci) && ci.known && ci.exterior && ci.world ? ci.world : cell;
        spaces_[cell] = space;
        return space;
    }

    uint32_t epoch_ = 0;
    std::map<uint32_t, uint32_t> spaces_;
};

LivePort g_port;
LiveStyles g_styleStore;
std::unique_ptr<th::ThreatDirector> g_director;
std::unique_ptr<th::StyleBook> g_styles;
uint32_t g_lastEpoch = 0;
uint64_t g_nextStyleScan = 0;

bool TacticsWanted() { return g_balancedQuery ? g_balancedQuery() : g_set.tacticsIni; }

void ReadSettings() {
    const std::string ini = IniPath();
    auto geti = [&](const char* k, int d) { return static_cast<int>(GetPrivateProfileIntA("threat", k, d, ini.c_str())); };
    th::Config& c = g_set.cfg;
    c = th::Config{};
    c.enabled = geti("enabled", 1) != 0;
    c.tickMs = static_cast<uint32_t>((std::max)(100, geti("tick_ms", static_cast<int>(c.tickMs))));
    c.fadeMs = static_cast<uint32_t>((std::max)(1000, geti("fade_ms", static_cast<int>(c.fadeMs))));
    c.nearBonus = static_cast<float>(geti("near_bonus", static_cast<int>(c.nearBonus)));
    c.nearRange = static_cast<float>((std::max)(1, geti("near_range", static_cast<int>(c.nearRange))));
    c.seenBonus = static_cast<float>(geti("seen_bonus", static_cast<int>(c.seenBonus)));
    c.engagedBonus = static_cast<float>(geti("engaged_bonus", static_cast<int>(c.engagedBonus)));
    c.switchMargin = static_cast<float>((std::max)(100, geti("switch_pct", static_cast<int>(c.switchMargin * 100.0f + 0.5f)))) / 100.0f;
    c.switchAbsolute = static_cast<float>(geti("switch_abs", static_cast<int>(c.switchAbsolute)));
    c.minOnTargetMs = static_cast<uint32_t>((std::max)(0, geti("min_on_target_ms", static_cast<int>(c.minOnTargetMs))));
    c.spreadPenalty = static_cast<float>(geti("spread", static_cast<int>(c.spreadPenalty)));
    c.groupRadius = static_cast<float>(geti("group_radius", static_cast<int>(c.groupRadius)));
    c.reach = static_cast<float>(geti("reach", static_cast<int>(c.reach)));
    c.maxSwitches = static_cast<uint32_t>((std::max)(1, geti("max_switches", static_cast<int>(c.maxSwitches))));
    c.includeNamed = geti("named", c.includeNamed ? 1 : 0) != 0;
    c.inferDamage = geti("infer_damage", c.inferDamage ? 1 : 0) != 0;
    g_set.tacticsIni = geti("tactics", 0) != 0;
    th::TacticsConfig& t = g_set.tactics;
    t = th::TacticsConfig{};
    t.takeCoverAtLeast = static_cast<float>((std::min)(100, (std::max)(0, geti("tactics_cover", static_cast<int>(t.takeCoverAtLeast)))));
    t.coverRadiusMult = static_cast<float>((std::max)(100, geti("tactics_radius_pct", static_cast<int>(t.coverRadiusMult * 100.0f + 0.5f)))) / 100.0f;
    t.sideStepAtLeast = static_cast<uint8_t>((std::min)(100, (std::max)(0, geti("tactics_sidestep", t.sideStepAtLeast))));
    t.fleeWhenHurt = geti("tactics_flee", t.fleeWhenHurt ? 1 : 0) != 0;
}

void Init() {
    if (g_inited) return;
    ReadSettings();
    g_director.reset(new th::ThreatDirector(g_set.cfg, g_port));
    g_director->setLog([](const std::string& line) { nvcoop::Log("%s", line.c_str()); });
    g_styles.reset(new th::StyleBook(g_styleStore));
    g_styles->configure(g_set.tactics);
    g_inited = true;
    nvcoop::RegisterMethodClosure("threat.status", [](const std::string&, std::string& out) {
        out = nvcoop::threat::statusJson();
        return true;
    }, true, 0);
    nvcoop::Log("threat: %s tick=%ums tactics=%s (%s)", g_set.cfg.enabled ? "enabled" : "disabled (nvcoop.ini [threat] enabled=0)", g_set.cfg.tickMs,
                g_set.tacticsIni ? "on" : "off", g_balancedQuery ? "rebalance mode decides" : "nvcoop.ini [threat] tactics=");
}

void Uninit() {
    if (g_styles) g_styles->restoreAll();   // every combat style goes back to the numbers the game had
    g_styles.reset();
    g_director.reset();
    g_inited = false;
}

// Every second: the combat styles of the actors this machine runs are noted (and patched while tactics are on); tactics going off puts them all back.
void ScanStyles(uint64_t now) {
    if (!g_styles) return;
    const bool want = TacticsWanted() && g_set.cfg.enabled;
    if (want != g_styles->on()) {
        g_styles->setOn(want);
        nvcoop::Log("threat: tactics %s (%zu combat styles known, %zu changed)", want ? "on" : "off", g_styles->known(), g_styles->patched());
    }
    if (!want || now < g_nextStyleScan) return;
    g_nextStyleScan = now + 1000;
    eng::IActorEngine* ae = eng::actorEngine();
    if (!ae || !ae->worldReady()) return;
    std::vector<eng::RefId> refs;
    ae->loadedActors(refs, kMaxLoaded);
    for (eng::RefId r : refs) {
        if (r == kPlayerRef || nvcoop::avatar::isPuppet(r)) continue;
        const eng::OwnerRef o = eng::ownerOf(r);
        if (!o.known || !o.mine || o.standIn) continue;     // only what this machine's AI runs
        if (const uint32_t style = StyleOf(LookupRef(r))) g_styles->see(style);
    }
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_DeferredInit) Init();
    else if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) Uninit();
}

void OnFrame() {
    if (!g_inited || !g_director) return;
    eng::IActorEngine* ae = eng::actorEngine();
    if (!ae) return;
    const uint32_t epoch = ae->worldEpoch();
    if (epoch != g_lastEpoch) {                 // a load, a new game, the main menu: nothing remembered is about this world
        g_lastEpoch = epoch;
        g_director->reset();
    }
    const uint64_t now = NowMs();
    ScanStyles(now);
    if (!ae->worldReady()) return;
    g_director->tick(now);
}

}  // namespace

NVCOOP_MODULE(threat, "threat", nullptr, OnMessage, OnFrame);

NVCOOP_RELOAD_STATE(threat, "threat", 1, 48,
    /*canUnload*/ nullptr,
    /*save*/ [](nvreload::Writer& w) {
        w.boolean(g_inited);
        return true;
    },
    /*restore*/ [](uint32_t, nvreload::Reader& r) {
        if (r.boolean()) Init();
        return true;
    },
    /*migrate*/ nullptr,
    /*shutdown*/ [] { if (g_inited) Uninit(); },
    /*fresh*/ nullptr);

namespace nvcoop::threat {

void noteHit(uint32_t victimRef, uint32_t attackerRef, float damage) {
    if (!g_inited || !g_director) return;
    nvcoop::avatar::Slot slot = nvcoop::avatar::kNoSlot;
    if (!nvcoop::avatar::slotOfActingRef(attackerRef, slot)) return;   // 0x14 is the local player, a puppet is its player; an NPC's blow is not a player's
    g_director->noteHit(victimRef, slot, damage, NowMs());
}

void setBalancedQuery(std::function<bool()> balancedOn) { g_balancedQuery = std::move(balancedOn); }
void setDownedQuery(std::function<bool(uint8_t)> downed) { g_downedQuery = std::move(downed); }
bool running() { return g_inited && g_director && g_set.cfg.enabled; }

std::string statusJson() {
    if (!g_inited || !g_director) return "{\"running\":false}";
    char tail[360];
    std::snprintf(tail, sizeof tail,
        "\"tactics\":{\"wanted\":%s,\"source\":\"%s\",\"on\":%s,\"styles\":%zu,\"patched\":%zu,\"foreign\":%zu},\"conditionCalls\":%llu,\"conditionFaults\":%llu}",
        TacticsWanted() ? "true" : "false", g_balancedQuery ? "rebalance" : "ini", g_styles && g_styles->on() ? "true" : "false",
        g_styles ? g_styles->known() : size_t(0), g_styles ? g_styles->patched() : size_t(0), g_styles ? g_styles->foreign() : size_t(0),
        (unsigned long long)g_evalCalls, (unsigned long long)g_evalFaults);
    return "{\"running\":true,\"director\":" + g_director->statusJson() + "," + tail;
}

}  // namespace nvcoop::threat
