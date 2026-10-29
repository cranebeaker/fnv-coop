// The live layer inside nvcoop.dll (dll-wiring). See live_glue.h. Game-facing part: reads the ini, turns the coop Runtime into a SessionView,
// forwards NVSE messages, and wraps each lane that exists in this build as a live-hub service. Everything is OFF unless nvcoop.ini [live]
// switches it on (or the environment: NVCOOP_LIVE_ACTORS / _AVATARS / _COMBAT / _SPAWNS / _SOCIAL = 1).
#include "live_glue.h"

#include <Windows.h>

#include <cstdio>
#include <cstdlib>
#include <memory>
#include <string>
#include <vector>

#include "PluginAPI.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "coop_runtime.h"
#include "live_hub.h"
#include "live/entity_map.h"   // S4: the one slot <-> ref table of this process (fed by the avatar module)

#ifdef NVCOOP_WITH_ACTORS
#include "actors/actor_glue.h"
#endif
#ifdef NVCOOP_WITH_NVMP
#include "nvmp/ghost_ref.h"
#endif
#ifdef NVLIVE_ACTOR_ENGINE
#include "actors/engine/actor_engine_api.h"
#include "live_actors_cfg.h"
#endif
#ifdef NVCOOP_WITH_AVATAR_ENGINE
#include "avatar_glue.h"   // plugin/avatar/engine: the module owns the AvatarRuntime; we put it on the network (avatar_net.h)
#include "avatar_tuning.h" // the near-tier rate and the rest of the avatar lag constants (nvcoop.ini [avatar])
#include "avatar_net.h"
#endif
#ifdef NVCOOP_WITH_COMBAT
#include "combat/combat_glue.h"
#endif
#ifdef NVCOOP_WITH_COMPANIONS
#include "companions/companions_glue.h"
#endif
#ifdef NVCOOP_WITH_LOOT
#include "loot/loot_glue.h"
#endif
#ifdef NVCOOP_WITH_SPAWN
#include "GameData.h"
#include "GameForms.h"
#include "spawn/engine_adapter.h"
#include "spawn/spawn_glue.h"
#ifdef NVCOOP_WITH_WORLD
#include "world_hooks.h"
#endif
#endif

namespace nvcoop::livewire {

namespace {

// ---- ini + environment ------------------------------------------------------------------------------------------------------------------
std::string g_iniPath;

std::string upper(std::string s) {
    for (auto& c : s) c = static_cast<char>(std::toupper(static_cast<unsigned char>(c)));
    return s;
}

std::string iniOrEnv(const char* section, const char* key, const char* def) {
    char name[160];
    std::snprintf(name, sizeof name, "NVCOOP_%s_%s", upper(section).c_str(), upper(key).c_str());
    char buf[512];
    DWORD n = GetEnvironmentVariableA(name, buf, sizeof buf);
    if (n > 0 && n < sizeof buf) return buf;
    if (g_iniPath.empty()) return def;
    GetPrivateProfileStringA(section, key, def, buf, sizeof buf, g_iniPath.c_str());
    return buf;
}

// ---- services ---------------------------------------------------------------------------------------------------------------------------
void bindPlayerEntities();   // defined below (avatar service): gives ActorSync the player port for the [entities] modes that run
// A switched-off or not-built lane still appears in the status, so "why does nothing happen" has an answer.
class PlaceholderService : public ILiveService {
public:
    explicit PlaceholderService(const char* n) : name_(n) {}
    const char* name() const override { return name_; }
private:
    const char* name_;
};

nvcoop::coop::Runtime* g_rt = nullptr;
bool glueAvatarIsPuppet(uint32_t ref);   // defined below (avatars service)

#ifdef NVCOOP_WITH_ACTORS
// plugin/actors: the ownership service builds its own ActorSync from the Runtime (and registers its own mux channel), so this service has no
// channels; the hub gives it the session lifecycle. The real engine port is installed by the actor-port module when [actors] allows it.
class ActorsService : public ILiveService {
public:
    ActorsService() {
#ifdef NVLIVE_ACTOR_ENGINE
        // Refs another layer drives are never NPCs: our player puppets, and the NV:MP ghosts while that mod is around. A tracked actor that
        // becomes excluded is dropped by the port at its next scan.
        nvcoop::actors::engine::setRefExclusion([](nvcoop::actors::engine::RefId ref) {
            if (glueAvatarIsPuppet(ref)) return true;
#ifdef NVCOOP_WITH_NVMP
            for (uint8_t slot = 1; slot < fnvcoop::kMaxSlots; ++slot)
                if (nvmp::GhostRefOfPlayer(slot) == ref) return true;
#endif
            return false;
        });
#endif
    }
    ~ActorsService() override {
#ifdef NVLIVE_ACTOR_ENGINE
        nvcoop::actors::engine::setRefExclusion(nullptr);
#endif
    }
    const char* name() const override { return "actors"; }
    void tick(uint64_t) override {
        nvcoop::actors::glueFrame(g_rt);
        bindPlayerEntities();   // a new ActorSync (a new session) gets the player port and the modes before its next tick
    }
    void sessionEnd() override { nvcoop::actors::glueFrame(nullptr); }   // tears the ActorSync down
    void loadEnd() override { purge(); }
    void gameExit() override { purge(); }
private:
    static void purge() {
        if (auto* s = nvcoop::actors::glueSync()) s->purgeStandIns();   // stand-ins are never saved and never survive a load
    }
};
#endif

#ifdef NVCOOP_WITH_AVATAR_ENGINE
// IAvatarSide over the avatar module's glue (avatar_glue.h). That module ticks the runtime and handles the NVSE load events itself, so the
// side has nothing to do for tick / events; only the clock matters: the module's interpolation buffers run on its QueryPerformanceCounter ms.
uint32_t QpcMs() {
    static LARGE_INTEGER f = [] {
        LARGE_INTEGER x;
        QueryPerformanceFrequency(&x);
        return x;
    }();
    LARGE_INTEGER t;
    QueryPerformanceCounter(&t);
    return static_cast<uint32_t>(static_cast<uint64_t>(t.QuadPart) * 1000ull / static_cast<uint64_t>(f.QuadPart));
}

class GlueSide : public IAvatarSide {
public:
    void setSnapshotSink(SnapshotSink s) override { nvcoop::avatar::glue::SetSnapshotSink(std::move(s)); }
    void setEquipSink(EquipSink s) override { nvcoop::avatar::glue::SetEquipSink(std::move(s)); }
    void setLocalIdentity(fnvcoop::live::NetId n, fnvcoop::live::OwnerEpoch e) override { nvcoop::avatar::glue::SetLocalIdentity(n, e); }
    void setLocalSlot(nvcoop::avatar::Slot s) override { nvcoop::avatar::glue::SetLocalSlot(s); }
    bool addRemote(nvcoop::avatar::Slot s, const nvcoop::avatar::Appearance& a) override { return nvcoop::avatar::glue::AddRemote(s, a); }
    void removeRemote(nvcoop::avatar::Slot s) override { nvcoop::avatar::glue::RemoveRemote(s); }
    bool setRemoteAppearance(nvcoop::avatar::Slot s, const nvcoop::avatar::Appearance& a) override { return nvcoop::avatar::glue::SetRemoteAppearance(s, a); }
    bool readLocalAppearance(nvcoop::avatar::Appearance& out) override { return nvcoop::avatar::glue::ReadLocalAppearance(out); }
    bool readLocalFacts(nvcoop::avatar::PlayerFacts& out) override { return nvcoop::avatar::glue::ReadLocalFacts(out); }   // the shipped side never forwarded these two: no Facts message ever left a guest in game (2026-10-28)
    bool readLocalPack(const std::vector<nvcoop::avatar::FormId>& watch, nvcoop::avatar::PlayerPack& out) override { return nvcoop::avatar::glue::ReadLocalPack(watch, out); }
    bool readLocalEquipment(std::vector<nvcoop::avatar::EquipEntry>& out) override { return nvcoop::avatar::glue::ReadLocalEquipment(out); }
    void onRemoteSnapshot(nvcoop::avatar::Slot s, const fnvcoop::live::ActorSnapshot& snap, uint32_t) override {
        nvcoop::avatar::glue::OnRemoteSnapshot(s, snap, QpcMs());   // arrival on the module's own clock
    }
    void onRemoteEquipment(nvcoop::avatar::Slot s, const std::vector<nvcoop::avatar::EquipEntry>& w) override {
        nvcoop::avatar::glue::OnRemoteEquipment(s, w);
    }
    uint32_t clockMs() const override { return QpcMs(); }
    void noteStage(nvcoop::avatar::NetStage st, double ms) override { nvcoop::avatar::glue::NoteStage(st, ms); }
    void tick(uint32_t) override {}
    void onEvent(Event) override {}
    bool purgePending() const override { return false; }
    nvcoop::avatar::RefId refOf(nvcoop::avatar::Slot s) const override { return nvcoop::avatar::glue::PuppetRef(s); }
    bool slotOf(nvcoop::avatar::RefId ref, nvcoop::avatar::Slot& slot) const override {
        auto s = nvcoop::avatar::glue::PuppetSlot(ref);
        if (!s) return false;
        slot = *s;
        return true;
    }
    std::string statusJson() override { return "{}"; }
};

struct GlueSideHolder { GlueSide side; };

AvatarNetOptions avatarOptions() {
    AvatarNetOptions o;
    o.tickSide = false;   // the avatar module ticks its runtime itself
    o.partsUnified = glueSwitches().partsUnified();    // [entities] parts / stream (docs/entity-core-plan.md): read once, when the service is made
    o.streamUnified = glueSwitches().streamUnified();
    // The near-tier rate (30 Hz) comes from the same table as the sender and the playback buffer; nvcoop.ini [avatar] nearHz overrides it (the avatar module reads
    // the other keys). The far and out rates stay as they are.
    nvcoop::avatar::engine::LagTuning lag = nvcoop::avatar::engine::DefaultLagTuning();
    nvcoop::avatar::engine::ApplyLagIni(lag, [](const char* key, const char* def) { return iniOrEnv("avatar", key, def); });
    o.rate = lag.rate;
    return o;
}

#ifdef NVCOOP_WITH_ACTORS
// The roster of parts=unified: the player records of this machine's ActorSync.
class ActorDirectory : public IEntityDirectory {
public:
    std::vector<PlayerId> playerSlots() const override {
        const nvcoop::actors::ActorSync* s = nvcoop::actors::glueSync();
        return s ? s->playerSlots() : std::vector<PlayerId>();
    }
};
#endif

class AvatarsService : private GlueSideHolder, public AvatarNet {
public:
    AvatarsService() : GlueSideHolder(), AvatarNet(side, avatarOptions()) {
#ifdef NVCOOP_WITH_ACTORS
        setEntityDirectory(&dir_);
        // A puppet that was bound again (a respawn after a save, a cell park, another template) is dressed again from the last parts known.
        mapListener_ = fnvcoop::live::processEntities().onChange([](fnvcoop::live::EntityNetId id, uint32_t, fnvcoop::live::EntityRole role, bool bound) {
            if (bound && role == fnvcoop::live::EntityRole::Puppet && fnvcoop::live::isPlayerNet(id))
                if (nvcoop::actors::ActorSync* s = nvcoop::actors::glueSync()) s->onPlayerBound(id);
        });
        haveListener_ = true;
#endif
        setPoseSink([this](PlayerId slot, bool present, fnvcoop::live::CellId cell, const fnvcoop::live::Vec3& pos) {
#ifdef NVLIVE_ACTOR_ENGINE
            if (slot != self()) {   // the local player is read by the port itself (it also knows the adjacent cells)
                nvcoop::actors::PlayerPose p;   // the ownership service wants every remote player's place
                p.present = present;
                p.cell = cell;
                p.pos = pos;
                nvcoop::actors::engine::setPlayerPose(slot, p);
            }
#endif
#ifdef NVCOOP_WITH_SPAWN
            nvcoop::spawn::setPlayerPose(slot, present, cell, nvcoop::spawn::sp::Vec3{pos.x, pos.y, pos.z});   // every slot, the local one too
#endif
            (void)slot; (void)present; (void)cell; (void)pos;
        });
    }
    ~AvatarsService() override {
#ifdef NVCOOP_WITH_ACTORS
        if (haveListener_) fnvcoop::live::processEntities().removeListener(mapListener_);
#endif
    }
    void sessionBegin(const SessionInfo& info, ILiveLink& link) override {
        AvatarNet::sessionBegin(info, link);
#ifdef NVLIVE_ACTOR_ENGINE
        // combat targets between actors and players survive the wire: the port asks which net id stands for a player ref
        nvcoop::actors::engine::setPlayerNetIdResolver(
            [](nvcoop::actors::engine::RefId ref) -> nvcoop::actors::NetId {
                const fnvcoop::live::EntityNetId n = fnvcoop::live::processEntities().netOf(ref);
                return fnvcoop::live::isPlayerNet(n) ? n : 0;   // the local player (0x14) and every usable puppet
            },
            [](nvcoop::actors::NetId id) -> nvcoop::actors::engine::RefId {
                return fnvcoop::live::isPlayerNet(id) ? fnvcoop::live::processEntities().refOf(id) : 0;
            });
#endif
    }
    void sessionEnd() override {
#ifdef NVLIVE_ACTOR_ENGINE
        nvcoop::actors::engine::setPlayerNetIdResolver(nullptr, nullptr);
#endif
        AvatarNet::sessionEnd();
    }
#ifdef NVCOOP_WITH_ACTORS
private:
    ActorDirectory dir_;
    size_t mapListener_ = 0;
    bool haveListener_ = false;
#endif
};
AvatarNet* g_avatarNet = nullptr;

void bindPlayerEntities() {
    if (!g_avatarNet || !(g_avatarNet->partsUnified() || g_avatarNet->streamUnified())) return;
#ifdef NVCOOP_WITH_ACTORS
    if (nvcoop::actors::ActorSync* s = nvcoop::actors::glueSync()) s->enablePlayerEntities(&g_avatarNet->playerPort(), g_avatarNet->partsUnified(), g_avatarNet->streamUnified());
#endif
}
std::function<std::vector<uint32_t>()> g_packWatch;   // coop_module's resolver of nvcoop_pack_watch.json (guest pack facts)
#else
void bindPlayerEntities() {}   // no avatar layer in this build: nothing to hand ActorSync
#endif

#ifdef NVCOOP_WITH_SPAWN
std::function<bool(const fnvcoop::SpawnRef&)> g_spawnApplier;   // coop_module's WorldApplier: makes this machine's own copy of a spawn it did not roll

// Host respawn timers run on GAME time (Wait / Sleep must count): GameDaysPassed * 24 h. SEH-guarded, no C++ objects in the frame.
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
uint64_t GameMsNow() {
    float days = 0;
    if (GameDaysSEH(&days) && days >= 0) return static_cast<uint64_t>(static_cast<double>(days) * 24.0 * 3600.0 * 1000.0);
    return GetTickCount64();   // not readable (yet): the wall clock never stalls a respawn
}

// plugin/spawn: same shape as the actors glue (own runtime and mux channel). The engine events the lane documents in spawn_glue.h come from
// the actor port's listeners; the engine adapter is built over the port's engine as soon as it exists.
class SpawnService : public ILiveService {
public:
    SpawnService() {
#ifdef NVLIVE_ACTOR_ENGINE
        using namespace nvcoop::actors::engine;
        tracked_ = addActorTrackedListener([](const TrackedEvent& e) { if (!e.standIn) nvcoop::spawn::onPlacedLoaded(e.ref); });
        died_ = addActorDiedListener([](const DiedEvent& e) { nvcoop::spawn::onActorDied(e.netId); });
        setCreatedIdResolver(nvcoop::spawn::createdIdResolver);   // the actor feed asks the spawn lane who a created actor is
#endif
        nvcoop::spawn::setIgnore([](nvcoop::spawn::RefId r) {   // puppets and the port's stand-ins are never rolls
#ifdef NVLIVE_ACTOR_ENGINE
            if (nvcoop::actors::engine::isStandIn(r)) return true;
#endif
            return glueAvatarIsPuppet(r);
        });
        nvcoop::spawn::setGameClock(GameMsNow);
#ifdef NVCOOP_WITH_WORLD
        // the host's script PlaceAtMe (a SpawnRef with the script as its cause), seen before the coop runtime takes the txn
        nvworld::setTxnObserver([this](const fnvcoop::EffectTxn& t) {
            for (const fnvcoop::Effect& e : t.effects)
                if (const auto* sr = std::get_if<fnvcoop::SpawnRef>(&e)) onHostSpawn(t.cause, *sr);
        });
#endif
    }
    ~SpawnService() override {
#ifdef NVCOOP_WITH_WORLD
        nvworld::setTxnObserver(nullptr);
#endif
        nvcoop::spawn::setEngine(nullptr);
        nvcoop::spawn::setGameClock(nullptr);
#ifdef NVLIVE_ACTOR_ENGINE
        nvcoop::actors::engine::removeListener(tracked_);
        nvcoop::actors::engine::removeListener(died_);
        nvcoop::actors::engine::setCreatedIdResolver(nullptr);
#endif
        nvcoop::spawn::setIgnore(nullptr);
    }
    const char* name() const override { return "spawns"; }
    void sessionBegin(const SessionInfo&, ILiveLink&) override { ensureEngine(); }
    void tick(uint64_t) override {
        nvcoop::spawn::glueFrame(g_rt);
        drainHostSpawns();   // after the lane ticked: its SpawnSync exists (the first frame of a session builds it)
    }
    void sessionEnd() override {
        pendingSpawns_.clear();
        nvcoop::spawn::glueFrame(nullptr);
    }
    void loadEnd() override { pendingSpawns_.clear(); nvcoop::spawn::onWorldLoaded(); }
    void gameExit() override { nvcoop::spawn::onWorldLoaded(); }
    bool engineWired() const { return adapter_ != nullptr; }

private:
    // Called from inside the capture hub's sink, i.e. while a script runs: only note it here; the lane (which may send, hide or show things)
    // gets it at this frame's tick, outside the script.
    void onHostSpawn(const fnvcoop::Cause& cause, const fnvcoop::SpawnRef& s) {
        if (cause.source.plugin.empty()) return;   // no script form to name the site after
        if (pendingSpawns_.size() < 256) pendingSpawns_.emplace_back(cause.source, s);
    }
    void drainHostSpawns() {
        std::vector<std::pair<fnvcoop::FormRef, fnvcoop::SpawnRef>> now;
        now.swap(pendingSpawns_);
        for (const auto& p : now) {
            const fnvcoop::SpawnRef& s = p.second;
            nvcoop::spawn::sp::SiteDef def;
            def.kind = nvcoop::spawn::sp::SiteKind::Script;
            def.anchor = p.first;
            def.id = nvcoop::spawn::sp::siteIdOf(def.kind, def.anchor);
            def.cellForm = s.cell;
            def.cell = nvhooks::resolver().fromRef(s.cell);
            def.pos = nvcoop::spawn::sp::Vec3{s.x, s.y, s.z};
            nvcoop::spawn::RefInfo info;
            if (adapter_ && adapter_->describe(s.id, info)) def.interior = info.interior;
            nvcoop::spawn::onHostSpawnEffect(def, s);
        }
    }
    std::vector<std::pair<fnvcoop::FormRef, fnvcoop::SpawnRef>> pendingSpawns_;
    // Built once the actor port's engine exists (its module may load after ours). Until then the lane runs with its no-op engine: the site table
    // is mirrored, nothing is ever rolled, held or deleted.
    void ensureEngine() {
        if (adapter_) return;
#ifdef NVLIVE_ACTOR_ENGINE
        nvcoop::actors::engine::IActorEngine* eng = nvcoop::actors::engine::actorEngine();
        if (!eng) {
            if (!warned_) nvcoop::Log("live: spawns=1 but the actor port's engine is not installed ([actors] engine=1 is needed): spawn arbitration stays passive");
            warned_ = true;
            return;
        }
        nvcoop::spawn::AdapterHooks h;
        h.placedNetId = [](nvcoop::spawn::RefId, const fnvcoop::FormRef& wire) { return nvcoop::actors::deriveNetId(wire); };
        h.refOfNet = [](nvcoop::spawn::NetId id) { return nvcoop::actors::engine::refOf(id); };
#ifdef NVCOOP_WITH_WORLD
        h.bindCreated = [](nvcoop::spawn::RefId ref, nvcoop::spawn::NetId id) { nvworld::ledger().onCreated(fnvcoop::createdRef(id), ref); };
        h.isCreatedBound = [](nvcoop::spawn::NetId id) { return nvworld::ledger().map().local(fnvcoop::createdRef(id)) != 0; };
        h.netOfCreated = [](nvcoop::spawn::RefId ref, nvcoop::spawn::NetId& id) {
            fnvcoop::FormRef host;
            if (!nvworld::ledger().map().hostOf(ref, &host)) return false;
            id = host.localId;
            return true;
        };
#endif
        h.createFromSpawnRef = [](const fnvcoop::SpawnRef& s) { return g_spawnApplier && g_spawnApplier(s); };
        // rebase (a leveled pick swap) and extras (ExtraLeveledCreature / ExtraEncounterZone reads) have no verified engine path yet: TODO-RE in
        // plugin/spawn/engine_adapter.h. Absent hooks are counted by the lane (Stats::rebaseFailed), never faked.
        adapter_.reset(new nvcoop::spawn::EngineAdapter(*eng, std::move(h)));
        nvcoop::spawn::setEngine(adapter_.get());
        nvcoop::Log("live: spawn arbitration wired to the actor port's engine");
#endif
    }
    std::unique_ptr<nvcoop::spawn::EngineAdapter> adapter_;
    bool warned_ = false;
#ifdef NVLIVE_ACTOR_ENGINE
    nvcoop::actors::engine::ListenerId tracked_ = 0, died_ = 0;
#endif
};
#endif

#ifdef NVCOOP_WITH_COMBAT
// plugin/combat: the hit gate and the engine effects live in its own module; this service gives it the session lifecycle.
class CombatService : public ILiveService {
public:
    const char* name() const override { return "combat"; }
    void tick(uint64_t) override { nvcoop::combat::glueFrame(g_rt); }
    void sessionEnd() override { nvcoop::combat::glueFrame(nullptr); }
};
#endif

#ifdef NVCOOP_WITH_COMPANIONS
// plugin/companions: companions, crime, shared kill rewards. Its frame is NOT driven from here: coop_module.cpp calls nvcoop::social::glueFrame() once per
// frame, right after livewire::glueFrame() (so after the actor layer ran), and that stays the ONE call site; a second call from this tick would run the
// social session twice per frame. What the live layer adds is the service entry (coop.live status shows "social"), the [live] social=1 dependency on
// actors=1 and, with avatars=1, the avatar layer's puppet map (a remote player's puppet is a "player" to the party rules, like combat's).
class SocialService : public ILiveService {
public:
    const char* name() const override { return "social"; }
};
#endif

struct Glue {
    LiveSwitches sw;
    bool configured = false, loaded = false;
    std::string actorsNotes;
    std::vector<std::unique_ptr<ILiveService>> owned;   // declared BEFORE the hub: members die in reverse order, and ~LiveHub ends the session
    LiveHub hub;                                        // part of every service, so the services must still exist then
    uint64_t lastNow = 0;
};
Glue& G() {
    static Glue g;
    return g;
}

SessionView makeView(nvcoop::coop::Runtime* rt, uint64_t nowMs) {
    SessionView v;
    v.nowMs = nowMs;
    if (!rt || !rt->running()) return v;
    v.active = true;
    v.mux = rt->mux();
    if (auto* host = rt->host()) {
        v.isHost = true;
        v.self = fnvcoop::kHostPlayer;
        for (const auto& cv : host->clients()) {
            ClientLink l;
            l.slot = cv.slot;
            l.peer = cv.peer;
            l.joined = cv.joined;
            v.clients.push_back(l);
        }
    } else if (auto* cl = rt->client()) {
        v.isHost = false;
        v.self = cl->joined() ? cl->slot() : fnvcoop::kNoPlayer;
        v.hostLinkUp = cl->joined() && !cl->baselineActive();
    } else {
        v.active = false;
    }
    return v;
}

// coop.live {}: what the live layer is doing on this machine.
// coop.entities (docs/entity-core-plan.md S13b): this machine's identity table. One row per player slot that has an entity here (its pinned owner and
// epoch from the actor port, how this machine binds it: 0x14 for the local player, the puppet for the others), plus every other binding of the EntityMap.
bool MEntities(const std::string&, std::string& out) {
    const fnvcoop::live::EntityMap& m = fnvcoop::live::processEntities();
    char b[160];
    out = "{\"v\":1,\"self\":" + std::to_string(m.localSlot() == kNoPlayer ? -1 : static_cast<int>(m.localSlot())) + ",\"players\":[";
    bool first = true;
    for (PlayerId s = 0; s < fnvcoop::kMaxSlots; ++s) {
        const fnvcoop::live::EntityNetId id = fnvcoop::live::playerNet(s);
        bool known = false;
        int owner = -1;
        unsigned epoch = 0;
#ifdef NVLIVE_ACTOR_ENGINE
        const nvcoop::actors::engine::OwnerRef o = nvcoop::actors::engine::ownerOfNet(id);
        known = o.known;
        owner = o.ownerSlot == kNoPlayer ? -1 : static_cast<int>(o.ownerSlot);
        epoch = o.epoch;
#endif
        const fnvcoop::live::EntityBinding* bd = m.binding(id);
        if (!known && !bd) continue;
        std::snprintf(b, sizeof b, "%s{\"slot\":%u,\"net\":\"0x%08X\",\"known\":%s,\"owner\":%d,\"epoch\":%u,\"role\":\"%s\",\"ref\":\"0x%08X\"}", first ? "" : ",",
                      static_cast<unsigned>(s), static_cast<unsigned>(id), known ? "true" : "false", owner, epoch, fnvcoop::live::toString(bd ? bd->role : fnvcoop::live::EntityRole::None),
                      static_cast<unsigned>(bd ? bd->ref : 0));
        out += b;
        first = false;
    }
    out += "],\"bindings\":" + std::to_string(m.size());
    const fnvcoop::live::EntityMapStats& st = m.stats();
    std::snprintf(b, sizeof b, ",\"stats\":{\"binds\":%llu,\"unbinds\":%llu,\"rebinds\":%llu,\"conflicts\":%llu}}", (unsigned long long)st.binds,
                  (unsigned long long)st.unbinds, (unsigned long long)st.rebinds, (unsigned long long)st.conflicts);
    out += b;
    return true;
}

bool MLive(const std::string&, std::string& out) {
    Glue& g = G();
    nvcoop::json::Value o = nvcoop::json::Value::object();
    o.set("switches", describeSwitches(g.sw)).set("warnings", g.sw.warnings).set("configured", g.configured).set("loaded", g.loaded);
    o.set("entities", describeEntities(g.sw));   // "parts=unified stream=unified": how the player is carried (a new key; "switches" keeps its four lanes)
    o.set("actorsConfigNotes", g.actorsNotes);
    out = o.dump();
    // splice the hub (and avatar) status in as raw JSON (already JSON text)
    if (!out.empty() && out.back() == '}') {
        out.pop_back();
        out += ",\"hub\":" + g.hub.statusJson();
#ifdef NVCOOP_WITH_AVATAR_ENGINE
        out += std::string(",\"avatarModule\":{\"enabled\":") + (nvcoop::avatar::glue::Enabled() ? "true" : "false") + ",\"running\":" +
               (nvcoop::avatar::glue::Running() ? "true" : "false") + ",\"why\":\"" + nvcoop::avatar::glue::NotRunningBecause() + "\"}";
        if (g_avatarNet) out += ",\"avatars\":" + g_avatarNet->statusJson();
#endif
        out += "}";
    }
    return true;
}

}  // namespace

IniGetter glueIni() { return [](const char* s, const char* k, const char* d) { return iniOrEnv(s, k, d); }; }

namespace {
// Where nvcoop.ini is when nobody told us (split-loader builds have no main.cpp): next to this DLL, like every module reads it.
std::string DefaultIniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&DefaultIniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}
void ensureConfigured() {
    if (!G().configured) glueConfigure(g_iniPath.empty() ? DefaultIniPath() : g_iniPath);
}
}  // namespace

// Lazy: a module's onLoad may ask before anybody called glueConfigure (the order of modules is not defined).
const LiveSwitches& glueSwitches() {
    ensureConfigured();
    return G().sw;
}

void glueSetSpawnApplier(std::function<bool(const fnvcoop::SpawnRef&)> f) {
#ifdef NVCOOP_WITH_SPAWN
    g_spawnApplier = std::move(f);
#else
    (void)f;
#endif
}

void glueOnEffect(const fnvcoop::Effect& e) {
#ifdef NVLIVE_ACTOR_ENGINE
    if (G().sw.actors) nvcoop::actors::engine::onEffect(e);
#else
    (void)e;
#endif
}

void glueBeforeSave() {
#ifdef NVLIVE_ACTOR_ENGINE
    if (G().sw.actors) nvcoop::actors::engine::beforeSave();
#endif
#ifdef NVCOOP_WITH_AVATAR_ENGINE
    if (nvcoop::avatar::glue::Enabled()) nvcoop::avatar::glue::PreSave();   // also for the dev loopback ([avatar] enabled=1 alone)
#endif
}

void glueAfterSave() {
#ifdef NVCOOP_WITH_AVATAR_ENGINE
    if (nvcoop::avatar::glue::Enabled()) nvcoop::avatar::glue::PostSave();
#endif
}

// The host's site table travels with its save as <save>.fnvspawn (next to the coop sidecar <save>.fnvcoop); a client has none.
fnvcoop::Bytes glueSpawnSidecar() {
#ifdef NVCOOP_WITH_SPAWN
    if (G().sw.spawns) return nvcoop::spawn::saveState();
#endif
    return {};
}
void glueSpawnSidecarLoaded(const fnvcoop::Bytes& b) {
#ifdef NVCOOP_WITH_SPAWN
    if (G().sw.spawns && !b.empty() && !nvcoop::spawn::loadState(b)) nvcoop::Log("live: spawn sidecar could not be read");
#else
    (void)b;
#endif
}

void glueConfigure(const std::string& iniPath) {
    Glue& g = G();
    g_iniPath = iniPath;
    g.sw = readLiveSwitches(glueIni());
    g.configured = true;
    nvcoop::Log("live: switches %s", describeSwitches(g.sw).c_str());
    nvcoop::Log("live: entities %s", describeEntities(g.sw).c_str());
    if (!g.sw.warnings.empty()) nvcoop::Log("live: %s", g.sw.warnings.c_str());
#ifdef NVLIVE_ACTOR_ENGINE
    {   // the real actor port is tuned from [actors] (its module reads the same values through readActorsConfig). [live] actors=0 keeps the port
        // installed but unfed: it only undoes the restraints a crashed run left in its ledger, and never tracks an actor without a session.
        nvcoop::actors::engine::Config c = readActorsConfig(glueIni(), nvcoop::actors::engine::Config(), &g.actorsNotes);
        nvcoop::actors::engine::configure(c);
        if (!g.actorsNotes.empty()) nvcoop::Log("live: %s", g.actorsNotes.c_str());
    }
#endif
}

void glueLoad() {
    Glue& g = G();
    if (g.loaded) return;
    ensureConfigured();
    g.loaded = true;
    auto add = [&](std::unique_ptr<ILiveService> svc, bool enabled) {
        if (!g.hub.add(svc.get(), enabled)) nvcoop::Log("live: service %s refused by the hub (channel clash)", svc->name());
        g.owned.push_back(std::move(svc));
    };
#ifdef NVCOOP_WITH_ACTORS
    if (g.sw.actors) add(std::unique_ptr<ILiveService>(new ActorsService()), true);
    else add(std::unique_ptr<ILiveService>(new PlaceholderService("actors")), false);
#else
    add(std::unique_ptr<ILiveService>(new PlaceholderService("actors")), false);
#endif
#ifdef NVCOOP_WITH_AVATAR_ENGINE
    if (g.sw.avatars) {
        // [live] avatars=1 also switches the avatar module on (avatar_module.cpp OnLoad), but that module loads AFTER this point: Enabled() is still
        // false here, so the old "module is off" warning fired on every correct setup (2026-10-22). Its own load line says if it stays off.
        std::unique_ptr<AvatarsService> a(new AvatarsService());
        g_avatarNet = a.get();
        if (g_packWatch) g_avatarNet->setPackWatch(g_packWatch);
        add(std::move(a), true);
    } else {
        add(std::unique_ptr<ILiveService>(new PlaceholderService("avatars")), false);
    }
#else
    add(std::unique_ptr<ILiveService>(new PlaceholderService("avatars")), false);   // not in this build
#endif
#ifdef NVCOOP_WITH_COMBAT
    if (g.sw.combat) add(std::unique_ptr<ILiveService>(new CombatService()), true);
    else add(std::unique_ptr<ILiveService>(new PlaceholderService("combat")), false);
#else
    add(std::unique_ptr<ILiveService>(new PlaceholderService("combat")), false);   // not in this build
#endif
#ifdef NVCOOP_WITH_SPAWN
    if (g.sw.spawns) add(std::unique_ptr<ILiveService>(new SpawnService()), true);
    else add(std::unique_ptr<ILiveService>(new PlaceholderService("spawns")), false);
#else
    add(std::unique_ptr<ILiveService>(new PlaceholderService("spawns")), false);   // not in this build
#endif
#ifdef NVCOOP_WITH_COMPANIONS
    if (g.sw.social) add(std::unique_ptr<ILiveService>(new SocialService()), true);
    else add(std::unique_ptr<ILiveService>(new PlaceholderService("social")), false);
#else
    add(std::unique_ptr<ILiveService>(new PlaceholderService("social")), false);   // not in this build
#endif
#ifdef NVCOOP_WITH_LOOT
    {   // plugin/loot: containers, drops, first-claim pickups (channel 0xB1). Its own switch is nvcoop.ini [loot] enabled=1 (no [live] key). The hub is built before the
        // loot module's DeferredInit, so the service is asked for FIRST (it reads the ini itself) and only then is enabled() true: the order of the two
        // arguments of one call is unspecified. Left out of the hub, loot_proof saw "in the session" = false on every copy (sweep 2026-10-22).
        nvcoop::loot::LootSync& lootService = nvcoop::loot::service();
        const bool lootOn = nvcoop::loot::enabled();
        if (!g.hub.add(&lootService, lootOn)) nvcoop::Log("live: service loot refused by the hub (channel clash)");
    }
#endif
#if defined(NVCOOP_WITH_COMPANIONS) && defined(NVCOOP_WITH_AVATAR_ENGINE)
    if (g.sw.social && g.sw.avatars)   // a puppet is a "player" to the party rules: a hit on it is a hit on its real player
        nvcoop::social::setPlayerRefMap(
            [](uint32_t ref, uint8_t& slot) {
                if (!glueAvatarIsPuppet(ref)) return false;
                slot = fnvcoop::live::processEntities().slotOfRef(ref);
                return slot != kNoPlayer;
            },
            [](uint8_t slot) { return glueAvatarRef(slot); });
#endif
#if defined(NVCOOP_WITH_COMBAT) && defined(NVCOOP_WITH_AVATAR_ENGINE)
    if (g.sw.combat && g.sw.avatars)   // a puppet is a "player" to combat: hits on it go to its real player
        nvcoop::combat::setPlayerRefMap(
            [](uint32_t ref, uint8_t& slot) {
                if (!glueAvatarIsPuppet(ref)) return false;
                slot = fnvcoop::live::processEntities().slotOfRef(ref);
                return slot != kNoPlayer;
            },
            [](uint8_t slot) { return glueAvatarRef(slot); });
#endif
    nvcoop::RegisterMethod("coop.live", MLive, true);
    nvcoop::RegisterMethod("coop.entities", MEntities, true);
#ifdef NVCOOP_WITH_SPAWN
    nvcoop::RegisterMethod("coop.spawn", [](const std::string& params, std::string& out) { return nvcoop::spawn::glueRpc(params, out); }, true);
#endif
    nvcoop::Log("live: hub ready, %u of %u services enabled", (unsigned)g.hub.enabledCount(), (unsigned)g.hub.serviceCount());
}

void glueFrame(nvcoop::coop::Runtime* rt, uint64_t nowMs) {
    Glue& g = G();
    if (!g.loaded) return;
    g_rt = rt;
    g.lastNow = nowMs;
    g.hub.frame(makeView(rt, nowMs));
}

void glueMessage(uint32_t type, void* data, uint32_t) {
    Glue& g = G();
    if (!g.loaded) return;
    switch (type) {
        case NVSEMessagingInterface::kMessage_PreLoadGame: g.hub.onLoadBegin(); break;
        case NVSEMessagingInterface::kMessage_PostLoadGame:
            if (data) g.hub.onLoadEnd();   // data = success
            break;
        case NVSEMessagingInterface::kMessage_NewGame:
            g.hub.onLoadBegin();   // the world is replaced without a Pre / PostLoad pair
            g.hub.onLoadEnd();
            break;
        case NVSEMessagingInterface::kMessage_SaveGame: g.hub.onSaved(); break;
        case NVSEMessagingInterface::kMessage_ExitToMainMenu:
        case NVSEMessagingInterface::kMessage_ExitGame:
        case NVSEMessagingInterface::kMessage_ExitGame_Console: g.hub.onGameExit(); break;
        default: break;
    }
}

void glueShutdown() {
    Glue& g = G();
    if (!g.loaded) return;
    g_rt = nullptr;
    g.hub.frame(SessionView());   // ends the session part of every service (puppets down, ActorSync dropped)
}

uint32_t glueAvatarRef(uint8_t slot) {
#ifdef NVCOOP_WITH_AVATAR_ENGINE
    if (slot >= fnvcoop::kMaxSlots) return 0;
    const fnvcoop::live::EntityMap& m = fnvcoop::live::processEntities();
    const fnvcoop::live::EntityNetId id = fnvcoop::live::playerNet(slot);
    return m.roleOf(id) == fnvcoop::live::EntityRole::Puppet ? m.refOf(id) : 0;   // a puppet only: the local slot answers 0, as before
#else
    (void)slot;
    return 0;
#endif
}

bool glueSlotFacts(uint8_t slot, nvcoop::SlotFacts* out) {
#ifdef NVCOOP_WITH_AVATAR_ENGINE
    if (!g_avatarNet || !out || slot == g_avatarNet->self()) return false;
    nvcoop::SlotFacts f;
    bool any = false;
    if (const AvLook* l = g_avatarNet->lookOf(slot)) {
        f.haveSex = true;
        f.sex = l->appearance.female ? 1 : 0;
        f.haveEquip = true;
        for (const auto& e : l->equipment) f.equipped.push_back(e.form);
        any = true;
    }
    bool sneak = false;
    if (g_avatarNet->sneakingOf(slot, &sneak)) { f.haveSneak = true; f.sneaking = sneak; any = true; }
    if (const nvcoop::avatar::PlayerFacts* pf = g_avatarNet->factsOf(slot)) {
        f.haveFacts = true;
        f.level = pf->level;
        f.sitting = pf->sitting;
        f.actions = pf->actions;
        f.regions.assign(pf->regions.begin(), pf->regions.end());
        any = true;
    }
    if (any) *out = std::move(f);
    return any;
#else
    (void)slot; (void)out;
    return false;
#endif
}

void glueSetPackWatch(std::function<std::vector<uint32_t>()> watch) {
#ifdef NVCOOP_WITH_AVATAR_ENGINE
    g_packWatch = std::move(watch);
    if (g_avatarNet) g_avatarNet->setPackWatch(g_packWatch);
#else
    (void)watch;
#endif
}

bool gluePackCount(uint8_t slot, uint32_t item, int64_t* count) {
#ifdef NVCOOP_WITH_AVATAR_ENGINE
    return g_avatarNet && slot != g_avatarNet->self() && g_avatarNet->packCount(slot, item, count);
#else
    (void)slot; (void)item; (void)count;
    return false;
#endif
}

bool gluePackWeight(uint8_t slot, double* weight) {
#ifdef NVCOOP_WITH_AVATAR_ENGINE
    return g_avatarNet && slot != g_avatarNet->self() && g_avatarNet->packWeight(slot, weight);
#else
    (void)slot; (void)weight;
    return false;
#endif
}

namespace {
bool glueAvatarIsPuppet(uint32_t ref) {
#ifdef NVCOOP_WITH_AVATAR_ENGINE
    const fnvcoop::live::EntityMap& m = fnvcoop::live::processEntities();
    return ref && m.roleOf(m.netOf(ref)) == fnvcoop::live::EntityRole::Puppet;
#else
    (void)ref;
    return false;
#endif
}
}  // namespace

}  // namespace nvcoop::livewire
