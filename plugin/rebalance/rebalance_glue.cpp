// plugin/rebalance, the engine half (docs/rebalance.md): the module that runs the Director against the real game. NOTHING here has run in a game.
//   * the GMST port: the game's settings through xNVSE's own pinned addresses (GameSettings.cpp g_GameSettingCollection, NiTypes.h _NiTMap_Lookup), each answer checked
//     against the setting's own name, the whole thing SEH-guarded (one fault turns the numbers off for the session)
//   * the fight observer (host): the actors this host owns from its engine, the actors a guest owns from the owner's reported combat target (the actor sync's drive events)
//   * the extras: PlaceAtMe next to a fighter through the actor port's engine, then StartCombat on the party player it was fighting
//   * the HUD line, the friendly fire switch of plugin/combat, the ini, the bridge method rebalance.status
// nvcoop.ini [rebalance]: enabled=1 mode=normal|balanced friendlyfire=0|1 (what a NEW campaign this host starts will play; a loaded campaign keeps its own)
//                         extras=1 rampms=10000
#include <Windows.h>

#include <cmath>
#include <cstdio>
#include <cstring>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "actors/engine/actor_engine_api.h"
#include "actors/engine/engine_iface.h"
#include "common/nvcoop_api.h"
#include "coop_runtime.h"
#include "intents/engine/game_engine.h"
#include "party_save_glue.h"
#include "reload/reload_state.h"
#include "rebalance/director.h"
#include "rebalance/armor_real.h"
#include "rebalance/effects_real.h"
#include "rebalance/rebalance_glue.h"
#include "threat/threat_glue.h"
#ifdef NVCOOP_REBALANCE_AVATAR
#include "avatar/avatar_api.h"
#endif
#ifdef NVCOOP_WITH_LOOT
#include "loot/loot_glue.h"
#endif
#ifdef NVCOOP_WITH_COMBAT
#include "combat/combat_glue.h"
#endif

namespace {

namespace rb = nvcoop::rebalance;
namespace eng = nvcoop::actors::engine;

// ---- engine facts (all from files in this repo) ------------------------------------------------------------------------------------------------------
constexpr uintptr_t kPlayerPtr = 0x11DEA3C;            // plugin/main.cpp kPlayerPtr: the PlayerCharacter pointer, null at the main menu
constexpr uintptr_t kGmstCollection = 0x011C8048;      // third_party/NVSE nvse/GameSettings.cpp g_GameSettingCollection (GameSettingCollection**)
constexpr uintptr_t kMapLookupFn = 0x00853130;         // third_party/NVSE nvse/NiTypes.h _NiTMap_Lookup: bool __thiscall NiTMapBase::Lookup(key, T_Data* out)
constexpr uintptr_t kSettingMapOffset = 0x10C;         // GameSettingCollection::settingMap
constexpr uint32_t kPlayerNetBase = 0xFE000000u;       // plugin/coop/live/avatar_wire.h playerNetId(slot): the net id of a player (the target of an actor's combat)
constexpr float kLeftBehind = 6144.0f;                 // an extra this far from every player (or in another place) is cleared away: 1.5 exterior cells (a cell is 4096 units wide), well outside what a fight spans

struct RawSetting { void* vtbl; uint32_t data; const char* name; };   // xNVSE Setting: vtable at +0, value at +4, name at +8

// A setting, looked up by name and checked against its own name. Static objects: a found pointer stays good for the whole run.
RawSetting* FindSetting(const char* name, int* status) {
    __try {
        void* collection = *reinterpret_cast<void**>(kGmstCollection);
        if (!collection) { *status = 1; return nullptr; }
        RawSetting* s = nullptr;
        auto lookup = reinterpret_cast<bool(__thiscall*)(void*, const char*, RawSetting**)>(kMapLookupFn);
        if (!lookup(reinterpret_cast<uint8_t*>(collection) + kSettingMapOffset, name, &s) || !s) { *status = 2; return nullptr; }
        if (!s->name || _stricmp(s->name, name) != 0) { *status = 3; return nullptr; }
        *status = 0;
        return s;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        *status = 9;
        return nullptr;
    }
}

bool PlayerLoaded() {
    __try {
        return *reinterpret_cast<void**>(kPlayerPtr) != nullptr;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

class GameGmst : public rb::IGmst {
public:
    bool read(const std::string& name, float& out) override {
        RawSetting* s = find(name);
        if (!s) return false;
        if (name[0] == 'f') { std::memcpy(&out, &s->data, 4); return true; }
        if (name[0] == 'i') { out = static_cast<float>(static_cast<int32_t>(s->data)); return true; }
        return false;
    }
    bool write(const std::string& name, float value) override {
        RawSetting* s = find(name);
        if (!s) return false;
        __try {
            if (name[0] == 'f') { std::memcpy(&s->data, &value, 4); return true; }
            if (name[0] == 'i') { s->data = static_cast<uint32_t>(static_cast<int32_t>(std::lround(value))); return true; }
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            broken_ = true;
        }
        return false;
    }
    bool broken() const { return broken_; }
private:
    RawSetting* find(const std::string& name) {
        if (broken_) return nullptr;
        auto it = cache_.find(name);
        if (it != cache_.end()) return it->second;
        int status = 0;
        RawSetting* s = FindSetting(name.c_str(), &status);
        if (status == 9) { broken_ = true; nvcoop::Log("rebalance: the game's settings could not be read safely (fault): the numbers stay as the game has them"); return nullptr; }
        if (!s) {
            if (status != 1 && !missing_.count(name)) { missing_[name] = true; nvcoop::Log("rebalance: setting %s not found (status %d)", name.c_str(), status); }
            return nullptr;
        }
        cache_[name] = s;
        return s;
    }
    std::map<std::string, RawSetting*> cache_;
    std::map<std::string, bool> missing_;
    bool broken_ = false;
};

// ---- the module's state ----------------------------------------------------------------------------------------------------------------------------
struct Seen { uint32_t targetNet = 0; uint64_t at = 0; };

struct State {
    rb::DirectorConfig cfg;
    std::string iniMode = "normal";
    bool iniFriendlyFire = false;
    GameGmst gmst;
    rb::GameEffects effects;
    std::unique_ptr<rb::Director> director;
    std::map<uint32_t, Seen> driveTargets;   // net id of an actor a guest owns -> the combat target its owner reported (a player's net id) and when
    eng::ListenerId driveListener = 0;
    rb::CampaignSettings picked;             // the last pick handed to the party code
    rb::GmstApplier::State carried;          // a hot reload brings the applier's originals back
    rb::EffectApplier::State carriedEffects; // ... and the Stimpak forms' originals
    bool haveCarried = false;
    bool announcedBroken = false;
} g;

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    const size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

uint64_t NowMs() { return static_cast<uint64_t>(GetTickCount64()); }

// ---- who is a player ----------------------------------------------------------------------------------------------------------------------------------
// The reference of the player in slot `slot` on THIS machine: slot 0 is the host. 0 = unknown.
uint32_t PlayerRefOfSlot(uint8_t slot, bool iAmHost) {
#ifdef NVCOOP_REBALANCE_AVATAR
    if (iAmHost) return slot == 0 ? eng::kLocalPlayerRef : nvcoop::avatar::puppetRefOf(slot);
#else
    if (iAmHost && slot == 0) return eng::kLocalPlayerRef;
#endif
    return 0;
}

bool IsPartyRef(uint32_t ref) {
    if (ref == eng::kLocalPlayerRef) return true;
#ifdef NVCOOP_REBALANCE_AVATAR
    return ref && nvcoop::avatar::isPuppet(ref);
#else
    return false;
#endif
}

// Where every player is, as the host's engine has it: the local player and each puppet.
struct Where { uint32_t cell; fnvcoop::live::Vec3 pos; };
std::vector<Where> PlayerPlaces(eng::IActorEngine& e) {
    std::vector<Where> out;
    auto add = [&](uint32_t ref) {
        fnvcoop::live::CellId cell = 0;
        fnvcoop::live::Vec3 pos;
        float rot = 0;
        if (e.readPose(ref, cell, pos, rot)) out.push_back({cell, pos});
    };
    add(eng::kLocalPlayerRef);
#ifdef NVCOOP_REBALANCE_AVATAR
    nvcoop::avatar::forEachPuppet([&](nvcoop::avatar::Slot, nvcoop::avatar::RefId ref) { add(ref); });
#endif
    return out;
}

bool NearAnyPlayer(const std::vector<Where>& players, uint32_t cell, const fnvcoop::live::Vec3& pos) {
    for (const Where& w : players) {
        if (w.cell != cell) continue;
        const float dx = w.pos.x - pos.x, dy = w.pos.y - pos.y;
        if (std::sqrt(dx * dx + dy * dy) < kLeftBehind) return true;
    }
    return false;
}

// ---- the host's fight observer and the extras -------------------------------------------------------------------------------------------------------
std::vector<rb::Fighter> ObserveFighters() {
    std::vector<rb::Fighter> out;
    eng::IActorEngine* e = eng::actorEngine();
    if (!e || !e->worldReady()) return out;
    std::vector<eng::RefId> refs;
    e->loadedActors(refs, 400);
    const uint64_t now = NowMs();
    for (eng::RefId ref : refs) {
        const eng::OwnerRef o = eng::ownerOf(ref);
        if (!o.known || o.standIn || o.pending) continue;      // the player, an untracked actor, a stand-in clone, a handoff in flight
        eng::ActorRaw raw;
        if (!e->read(ref, raw) || !raw.valid || raw.isPlayer || raw.disabled || raw.deleted) continue;
        rb::Fighter f;
        f.netId = o.netId;
        f.ref = ref;
        f.baseId = raw.baseId;
        f.generic = !raw.baseUnique;
        f.dead = raw.dead();
        if (o.mine) {
            // fights this host runs itself: the engine's own combat target
            if (raw.inCombat && IsPartyRef(raw.combatTarget)) { f.fightsParty = true; f.targetRef = raw.combatTarget; }
        } else {
            // fights a guest runs: the target its owner reported with the last snapshot (a player's net id)
            auto it = g.driveTargets.find(o.netId);
            if (it != g.driveTargets.end() && now - it->second.at < 2500 && (it->second.targetNet >> 8) == (kPlayerNetBase >> 8)) {
                const uint32_t target = PlayerRefOfSlot(static_cast<uint8_t>(it->second.targetNet & 0xFF), true);
                if (target) { f.fightsParty = true; f.targetRef = target; }
            }
        }
        if (f.fightsParty && f.baseId) out.push_back(f);
    }
    if (g.driveTargets.size() > 2048) g.driveTargets.clear();
    return out;
}

uint32_t SpawnExtra(const rb::Fighter& like, uint32_t targetRef) {
    eng::IActorEngine* e = eng::actorEngine();
    if (!e || !like.baseId || !like.ref) return 0;
    const eng::RefId made = e->placeAtMe(like.ref, like.baseId);
    if (!made) return 0;
    if (targetRef) e->startCombat(made, targetRef);
    nvcoop::Log("rebalance: extra %08X made next to %08X (base %08X), fighting %08X", made, like.ref, like.baseId, targetRef);
    return made;
}

rb::FightDirector::ExtraInfo InspectExtra(uint32_t ref) {
    rb::FightDirector::ExtraInfo info;
    eng::IActorEngine* e = eng::actorEngine();
    eng::ActorRaw raw;
    if (!e || !e->read(ref, raw) || !raw.valid || raw.deleted) return info;
    info.exists = true;
    info.dead = raw.dead();
    info.nearAnyPlayer = NearAnyPlayer(PlayerPlaces(*e), raw.cell, raw.pos);
    return info;
}

void RemoveExtra(uint32_t ref) {
    if (eng::IActorEngine* e = eng::actorEngine()) e->markForDelete(ref);
}

// ---- supplies and ammo: the hook plugin/loot calls for every container or corpse this machine registers (once, before the contents are shared) ----------------
#ifdef NVCOOP_WITH_LOOT
bool TrimLootHook(const nvcoop::loot::TrimContext& c, nvcoop::loot::Items& items) {
    namespace nl = nvcoop::loot;
    if (!g.director || !g.director->balancedOn()) return false;
    std::vector<rb::LootStack> stacks;
    std::vector<fnvcoop::FormRef> bases;
    for (const auto& kv : items) {
        rb::LootStack s;
        s.id = kv.first.plugin + ":" + std::to_string(kv.first.localId);
        s.count = kv.second;
        const nl::ItemClass k = c.classify ? c.classify(kv.first) : nl::ItemClass::Other;
        s.kind = k == nl::ItemClass::Ammo ? rb::ItemKind::Ammo : k == nl::ItemClass::Supply ? rb::ItemKind::Supply : rb::ItemKind::Other;
        stacks.push_back(s);
        bases.push_back(kv.first);
    }
    rb::LootSource src;
    src.net = c.net;
    src.corpse = c.kind == nl::lc::ContainerKind::Corpse;
    src.uniqueActor = c.uniqueActor;
    if (g.director->trimLoot(src, stacks) <= 0) return false;
    for (size_t i = 0; i < stacks.size(); ++i) {
        if (stacks[i].count > 0) items[bases[i]] = stacks[i].count;
        else items.erase(bases[i]);
    }
    return true;
}
#endif

// ---- the director over the real game --------------------------------------------------------------------------------------------------------------
void Create() {
    const std::string ini = IniPath();
    g.cfg.enabled = GetPrivateProfileIntA("rebalance", "enabled", 1, ini.c_str()) != 0;
    g.cfg.extras = GetPrivateProfileIntA("rebalance", "extras", 1, ini.c_str()) != 0;
    g.cfg.rampMs = static_cast<uint32_t>(GetPrivateProfileIntA("rebalance", "rampms", 10000, ini.c_str()));
    g.iniFriendlyFire = GetPrivateProfileIntA("rebalance", "friendlyfire", 0, ini.c_str()) != 0;
    char mode[32] = {};
    GetPrivateProfileStringA("rebalance", "mode", "normal", mode, sizeof mode, ini.c_str());
    g.iniMode = mode;

    rb::DirectorEnv env;
    env.worldReady = [] {
        if (g.gmst.broken()) return false;
        eng::IActorEngine* e = eng::actorEngine();
        return PlayerLoaded() && (!e || e->worldReady());
    };
    env.isHost = [] { return nvcoop::coop::partyglue::isHost(); };
    env.settings = [] { return nvcoop::coop::partyglue::campaignSettings(); };
    env.players = [] { return nvcoop::coop::partyglue::partySize(); };
    env.nowMs = NowMs;
    env.setFriendlyFire = [](int ff) {
#ifdef NVCOOP_WITH_COMBAT
        nvcoop::combat::glueSetFriendlyFire(ff);
#else
        (void)ff;
#endif
    };
    env.hud = [](const std::string& line) { nvcoop::intents::game::engine().notify(line); };
    env.log = [](const std::string& line) { nvcoop::Log("%s", line.c_str()); };
    env.fighters = ObserveFighters;
    env.fight.spawn = SpawnExtra;
    env.fight.inspect = InspectExtra;
    env.fight.remove = RemoveExtra;
    env.fight.log = env.log;
    g.director.reset(new rb::Director(g.cfg, env, g.gmst, g.effects));
#ifdef NVCOOP_WITH_LOOT
    nvcoop::loot::setTrimHook(TrimLootHook);   // the loot service thins what a container or corpse holds, once, on the machine that registers it
#endif
#ifdef NVCOOP_WITH_COMBAT
    nvcoop::combat::glueSetHitScale([](uint32_t victimRef, int limb) -> float {   // head, limb and armour factor, Co-op balanced only (balance.cpp)
        if (!g.director || !g.director->balancedOn() || limb < 0 || limb > 5) return 1.0f;
        // The head and limb factors are there to give the party's aimed shots back what toughness took off, so they apply to hits on enemies only; a hit on a
        // player counts as a torso hit (the armour factor still applies to both).
        const uint8_t where = IsPartyRef(victimRef) ? 0 : static_cast<uint8_t>(limb);
        return rb::bodyHitFactor(g.director->profile(), where, rb::armorClassOf(victimRef));
    });
#endif
    nvcoop::threat::setBalancedQuery([] { return nvcoop::rebalance::glueBalancedOn(); });   // the threat AI's cover / side-step / flee tactics follow the campaign
    if (g.haveCarried) { g.director->applier().restoreState(g.carried); g.director->effects().restoreState(g.carriedEffects); }
    g.driveListener = eng::addDriveListener([](const eng::DriveEvent& ev) {
        if (!ev.out || ev.standIn) return;
        g.driveTargets[ev.netId] = Seen{static_cast<uint32_t>(ev.out->ai.combatTarget), NowMs()};
    });
    nvcoop::RegisterMethodClosure("rebalance.status", [](const std::string&, std::string& out) {
        out = g.director ? g.director->statusJson() : std::string("{\"enabled\":false}");
        return true;
    }, true, 0);
    nvcoop::Log("rebalance: %s (new campaigns play %s, friendly fire %s; extras %s, ramp %u ms)", g.cfg.enabled ? "on" : "off by nvcoop.ini [rebalance] enabled=0", g.iniMode.c_str(),
                g.iniFriendlyFire ? "on" : "off", g.cfg.extras ? "on" : "off", g.cfg.rampMs);
}

void Frame() {
    if (!g.director) Create();
    if (!g.cfg.enabled) return;
    // what the next campaign this host starts will play (locked into the campaign when it is minted)
    const rb::CampaignSettings pick = rb::chooseSettings(g.iniMode, g.iniFriendlyFire);
    if (!(pick == g.picked)) {
        g.picked = pick;
        nvcoop::coop::partyglue::setNewCampaignSettings(pick);
    }
    g.director->tick();
    if (g.gmst.broken() && !g.announcedBroken) {
        g.announcedBroken = true;
        nvcoop::Log("rebalance: off for this session (the game's settings could not be reached)");
    }
}

}  // namespace

namespace nvcoop::rebalance {

// What other modules read (docs/rebalance.md): this campaign plays Co-op balanced.
bool glueBalancedOn() { return g.director && g.director->balancedOn(); }
CampaignSettings glueSettings() { return g.director ? g.director->settings() : CampaignSettings(); }
int gluePartySize() { return g.director ? g.director->players() : 0; }

}  // namespace nvcoop::rebalance

NVCOOP_MODULE_LATE(rebalance, "rebalance", nullptr, nullptr, Frame);

namespace {

// A hot reload must not lose what the old core changed: the new one would take a changed value for the game's own. Version 2 adds the Stimpak forms.
bool SaveState(nvreload::Writer& w) {
    const rb::GmstApplier::State st = g.director ? g.director->applier().state() : g.carried;
    w.u32(static_cast<uint32_t>(st.originals.size()));
    for (const auto& [k, v] : st.originals) { w.str(k); w.f64(v); }
    w.u32(static_cast<uint32_t>(st.written.size()));
    for (const auto& [k, v] : st.written) { w.str(k); w.f64(v); }
    const rb::EffectApplier::State fx = g.director ? g.director->effects().state() : g.carriedEffects;
    w.u32(static_cast<uint32_t>(fx.originals.size()));
    for (const auto& [k, v] : fx.originals) { w.u32(k); w.f64(v); }
    w.u32(static_cast<uint32_t>(fx.written.size()));
    for (const auto& [k, v] : fx.written) { w.u32(k); w.f64(v); }
    return true;
}

bool RestoreState(uint32_t version, nvreload::Reader& r) {
    rb::GmstApplier::State st;
    for (uint32_t n = r.u32(), i = 0; r.ok() && i < n && i < 256; ++i) { std::string k = r.str(); st.originals[k] = static_cast<float>(r.f64()); }
    for (uint32_t n = r.u32(), i = 0; r.ok() && i < n && i < 256; ++i) { std::string k = r.str(); st.written[k] = static_cast<float>(r.f64()); }
    rb::EffectApplier::State fx;
    if (version >= 2) {
        for (uint32_t n = r.u32(), i = 0; r.ok() && i < n && i < 64; ++i) { uint32_t k = r.u32(); fx.originals[k] = static_cast<float>(r.f64()); }
        for (uint32_t n = r.u32(), i = 0; r.ok() && i < n && i < 64; ++i) { uint32_t k = r.u32(); fx.written[k] = static_cast<float>(r.f64()); }
    }
    if (r.ok()) { g.carried = st; g.carriedEffects = fx; g.haveCarried = true; }
    return true;
}

void ShutdownState() {
#ifdef NVCOOP_WITH_LOOT
    nvcoop::loot::setTrimHook(nullptr);
#endif
#ifdef NVCOOP_WITH_COMBAT
    nvcoop::combat::glueSetHitScale(nullptr);
#endif
    if (g.driveListener) eng::removeListener(g.driveListener);
    g.driveListener = 0;
    g.director.reset();
}

}  // namespace

NVCOOP_RELOAD_STATE(rebalance, "rebalance", 2, 41,
    /*canUnload*/ [] { return std::string(); },
    /*save*/ [](nvreload::Writer& w) { return SaveState(w); },
    /*restore*/ [](uint32_t version, nvreload::Reader& r) { return RestoreState(version, r); },
    /*migrate*/ [](uint32_t version, nvreload::Reader& r) { return RestoreState(version, r); },   // version 1 has no Stimpak forms
    /*shutdown*/ [] { ShutdownState(); },
    /*fresh*/ nullptr);
