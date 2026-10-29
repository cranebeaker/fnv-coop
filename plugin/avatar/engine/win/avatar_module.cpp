// nvcoop module "m2_avatar": owns the AvatarRuntime inside the game. Inert unless nvcoop.ini has [avatar] enabled=1.
//
//   [avatar]
//   enabled=0                       1 = create the runtime on the first frame in which the player exists
//   template=player                 the template NPC base form of every puppet: "player" (the local player's own base form, no ESP
//                                   needed, UNVERIFIED that PlaceAtMe of it works) or a hex form id of an NPC_ record
//   templates=1A2B:m:00123456,1A2B:f:00123457     optional (race hex : m|f : base hex) entries; they win over `template`
//   localSlot=0                     this machine's own player slot (it never gets a puppet)
//   maxPuppets=3
//   loopback=0                      1 = start the dev loopback puppet (a puppet of the local player, shifted by loopbackOffset)
//   loopbackOffset=160,0,0
//
// Bridge methods (all on the main thread):
//   avatar.status                   -> {"v":1,"enabled":..,"puppets":[..],"stats":{..},"tagged":N,"engine":{..},"local":{..},"backend":{..}}
//   avatar.local                    -> the last local sample
//   avatar.trace {n?}               -> the last n published samples of the local player and each puppet's fed poses, on one clock (stage timers)
//   avatar.loopback {on, offset?}   -> start / stop the loopback puppet
//   avatar.add {slot, race?, female?} / avatar.remove {slot} / avatar.feed {slot, x,y,z, yaw, vx,vy,vz, loco, cell?}
//                                   -> dev: a remote player without a network (feed = one snapshot)
//   avatar.api                      -> the ghost-ref map (avatar_api.h): live puppets, their acting refs, round trip ref -> slot -> ref, hit router counters
//   avatar.look                     -> the local player's look as it goes on the wire + the worn items with weapon modification flags
//   avatar.puppetlook {slot}        -> what the puppet really wears, weapon modification flags read back from the engine
//   avatar.hit {slot, damage, limb?, attackerRef?, melee?} -> a hit on the puppet of `slot` through the router (-> combat layer -> the real player)
//   avatar.presave / avatar.postsave -> despawn / allow respawn around a save (NVSE's SaveGame message arrives AFTER the .fos is written)
#include <Windows.h>

#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "avatar_api.h"
#include "avatar_glue.h"
#include "avatar_groups.h"   // kGrpDeath: the lying group of a downed puppet
#include "avatar_runtime.h"
#include "run_state.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "../../puppet_hits.h"
#include "game_backend_win.h"
#include "nvse/PluginAPI.h"
#ifdef NVCOOP_WITH_LIVE
#include "coop/live/live_glue.h"   // dll-wiring: glueSwitches()
#include "live/entity_map.h"        // the process EntityMap: each puppet is its player's entity on this machine
#endif
#ifdef NVCOOP_WITH_COMBAT
#include "combat/combat_glue.h"    // glueRunning() / glueGate(): the combat layer carries the hit to the real player's machine
#endif

namespace nvcoop::avatar {
namespace {

using engine::AvatarRuntime;
using engine::GameEvent;
using engine::RuntimeConfig;
using engine::WinBackend;
using nvcoop::json::Value;

#ifdef NVCOOP_WITH_COMBAT
// The production port of the hit router: asks the combat layer's gate, which maps the puppet to its player (setPlayerRefMap), sends the
// HitEvent to that player's machine and tells us what the LOCAL engine may do with the damage (Clamp). A gate that answers Proceed did not
// treat the victim as a player (map missing): that is not a forwarded hit.
struct CombatGatePort : IPuppetHitPort {
    bool Forward(Slot, const PuppetHit& h, HitAction& action) override {
        if (!nvcoop::combat::glueRunning()) return false;
        nvcoop::combat::EngineHit e;
        e.attackerRef = h.attackerRef;
        e.victimRef = h.victimRef;
        e.healthDmg = h.healthDmg;
        e.limbDmg = h.limbDmg;
        e.hitLocation = h.hitLocation;
        e.impact = nvcoop::combat::cc::Vec3{h.impact.x, h.impact.y, h.impact.z};
        e.projectileKey = h.projectileKey;
        e.explosionKey = h.explosionKey;
        e.melee = h.melee;
        e.critical = h.critical;
        e.vats = h.vats;
        const nvcoop::combat::GateDecision d = nvcoop::combat::glueGate(e);
        switch (d.action) {
            case nvcoop::combat::GateAction::Proceed: return false;
            case nvcoop::combat::GateAction::Clamp: action = HitAction::Clamp; return true;
            case nvcoop::combat::GateAction::Neutralize: action = HitAction::Neutralize; return true;
        }
        return false;
    }
};
#endif

struct State {
    bool enabled = false;
    NVSEConsoleInterface* console = nullptr;
    std::unique_ptr<WinBackend> backend;
    std::unique_ptr<AvatarRuntime> rt;
    RuntimeConfig cfg;
    bool templatePlayer = false;
    bool loopbackAtStart = false;
    // remembered until the runtime exists
    std::function<void(const live::ActorSnapshot&)> snapSink;
    std::function<void(const std::vector<EquipEntry>&)> equipSink;
    live::NetId netId = 0;
    live::OwnerEpoch epoch = 0;
    std::map<Slot, Appearance> pendingRemotes;
    std::map<Slot, std::vector<EquipEntry>> pendingEquip;   // equipment that arrived before the runtime existed
    std::map<Slot, int> pendingPose;                        // downed poses set before the runtime existed (replayed when it is created)
    std::unique_ptr<ManagerDirectory> dir;                  // the ghost-ref map (avatar_api.h), registered while the runtime lives
    uint32_t frames = 0;
    bool presaved = false;
    std::map<Slot, uint32_t> feedSeq;
    PuppetHitRouter hits;                                   // hits on a puppet -> the real player's machine (puppet_hits.h)
#ifdef NVCOOP_WITH_COMBAT
    CombatGatePort hitPort;
#endif
};
State g;

std::string IniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&IniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

std::string IniStr(const char* key, const char* def) {
    char buf[512];
    GetPrivateProfileStringA("avatar", key, def, buf, sizeof buf, IniPath().c_str());
    return buf;
}

uint32_t Hex(const std::string& s) { return static_cast<uint32_t>(std::strtoul(s.c_str(), nullptr, 16)); }

uint32_t NowMs() {
    static LARGE_INTEGER f = [] {
        LARGE_INTEGER x;
        QueryPerformanceFrequency(&x);
        return x;
    }();
    LARGE_INTEGER t;
    QueryPerformanceCounter(&t);
    return static_cast<uint32_t>(static_cast<uint64_t>(t.QuadPart) * 1000ull / static_cast<uint64_t>(f.QuadPart));
}

void ParseConfig() {
    RuntimeConfig& c = g.cfg;
    c.puppet.localSlot = static_cast<Slot>(GetPrivateProfileIntA("avatar", "localSlot", 0, IniPath().c_str()));
    c.puppet.maxPuppets = static_cast<unsigned>(GetPrivateProfileIntA("avatar", "maxPuppets", 3, IniPath().c_str()));
    std::string tmpl = IniStr("template", "player");
    g.templatePlayer = tmpl == "player" || tmpl.empty();
    if (!g.templatePlayer) c.puppet.defaultTemplate = Hex(tmpl);
    std::string list = IniStr("templates", "");
    size_t pos = 0;
    while (pos < list.size()) {
        size_t end = list.find(',', pos);
        if (end == std::string::npos) end = list.size();
        std::string item = list.substr(pos, end - pos);
        pos = end + 1;
        size_t a = item.find(':'), b = item.find(':', a == std::string::npos ? 0 : a + 1);
        if (a == std::string::npos || b == std::string::npos) continue;
        TemplateEntry t;
        t.race = Hex(item.substr(0, a));
        t.female = item.substr(a + 1, b - a - 1) == "f";
        t.base = Hex(item.substr(b + 1));
        if (t.base) c.puppet.templates.push_back(t);
    }
    if (!c.puppet.templates.empty() && g.templatePlayer) g.templatePlayer = false;
    g.loopbackAtStart = GetPrivateProfileIntA("avatar", "loopback", 0, IniPath().c_str()) != 0;
    std::string off = IniStr("loopbackOffset", "160,0,0");
    float v[3] = {160, 0, 0};
    std::sscanf(off.c_str(), "%f,%f,%f", &v[0], &v[1], &v[2]);
    c.loopbackOffset = Vec3{v[0], v[1], v[2]};
    c.loopbackSlot = static_cast<Slot>(c.puppet.localSlot == 1 ? 2 : 1);
    // sender rate / velocity filter and the playback buffer: [avatar] nearHz, lowLatency, minDelayMs, jitterFactor, maxDelayMs, extrapolateTauMs, maxExtrapolateMs,
    // blendMs, slewPerMille, velAlpha (avatar_tuning.h; the near-tier relay rate is read by live_glue.cpp from the same keys)
    engine::LagTuning lag = engine::DefaultLagTuning();
    engine::ApplyLagIni(lag, [](const char* key, const char* def) { return IniStr(key, def); });
    c.sampler = lag.sampler;
    c.interp = lag.interp;
}

bool PlayerReady() {
    if (!g.backend) return false;
    engine::RefInfo ri;
    return g.backend->ReadRef(engine::kPlayerRef, ri) && ri.exists && ri.cell != 0 && ri.has3D;
}

void CreateRuntime() {
    if (g.rt || !g.backend) return;
    RuntimeConfig c = g.cfg;
    if (g.templatePlayer) {
        engine::RefInfo pl;
        if (!g.backend->ReadRef(engine::kPlayerRef, pl) || !pl.base) return;
        c.puppet.defaultTemplate = pl.base;
        nvcoop::Log("avatar: template = the player's own base form %08X (unverified that PlaceAtMe works with it)", (unsigned)pl.base);
    } else if (c.puppet.templates.empty() && !c.puppet.defaultTemplate) {
        nvcoop::Log("avatar: no template NPC configured ([avatar] template= / templates=)");
    }
    g.rt = std::make_unique<AvatarRuntime>(*g.backend, c);
    g.rt->SetLog([](const std::string& s) { nvcoop::Log("%s", s.c_str()); });
    g.rt->SetLocalIdentity(g.netId, g.epoch);
    if (g.snapSink) g.rt->SetSnapshotSink(g.snapSink);
    if (g.equipSink) g.rt->SetEquipSink(g.equipSink);
    for (auto& kv : g.pendingRemotes) g.rt->AddRemote(kv.first, kv.second);
    for (auto& kv : g.pendingEquip)
        if (g.pendingRemotes.count(kv.first)) g.rt->OnRemoteEquipment(kv.first, kv.second);
    for (auto& kv : g.pendingPose) g.rt->SetPoseOverride(kv.first, kv.second);
    g.rt->manager().SetBindingHook([](Slot slot, RefId ref) {   // S4: told from inside the call that changed it, so the map never holds a stale id
        fnvcoop::live::EntityMap& m = fnvcoop::live::processEntities();
        if (ref) m.bind(fnvcoop::live::playerNet(slot), ref, fnvcoop::live::EntityRole::Puppet);
        else m.unbind(fnvcoop::live::playerNet(slot));
    });
    fnvcoop::live::processEntities().setLocalSlot(c.puppet.localSlot);
    g.dir = std::make_unique<ManagerDirectory>(g.rt->manager(), c.puppet.localSlot);
    setPuppetDirectory(g.dir.get());
    if (g.loopbackAtStart) g.rt->SetLoopback(true);
    nvcoop::Log("avatar: runtime created (localSlot %d, maxPuppets %u, loopback %d)", (int)c.puppet.localSlot, c.puppet.maxPuppets, g.loopbackAtStart ? 1 : 0);
}

// ---- bridge --------------------------------------------------------------------------------------------------------------

bool Parse(const std::string& params, Value& v, std::string& out) {
    if (params.empty()) {
        v = Value::object();
        return true;
    }
    std::string err;
    if (!nvcoop::json::parse(params, v, err)) {
        out = "bad params: " + err;
        return false;
    }
    return true;
}

bool NotReady(std::string& out) {
    out = g.enabled ? "avatar runtime not created yet (no player in a cell)" : "avatar layer disabled: nvcoop.ini [avatar] enabled=1";
    return false;
}

bool MStatus(const std::string&, std::string& out) {
    if (!g.enabled || !g.rt) {
        char b[220];
        std::snprintf(b, sizeof b, "{\"v\":1,\"enabled\":%s,\"running\":false,\"reason\":\"%s\"}", g.enabled ? "true" : "false",
                      g.enabled ? "waiting for the player" : "[avatar] enabled=0");
        out = b;
        return true;
    }
    std::string s = g.rt->StatusJson();
    if (!s.empty() && s.back() == '}') s.pop_back();
    s += ",\"running\":true,\"backend\":" + g.backend->PinsJson() + "}";
    out = s;
    return true;
}

// avatar.trace {n?} -> the last n published samples of the local player and the last 2n frames of each puppet's fed pose, on one clock (AvatarRuntime::TraceJson)
bool MTrace(const std::string& params, std::string& out) {
    if (!g.rt) return NotReady(out);
    Value p;
    if (!Parse(params, p, out)) return false;
    const int n = static_cast<int>(p.get("n").asInt(300));
    out = g.rt->TraceJson(static_cast<size_t>(n < 1 ? 1 : (n > 900 ? 900 : n)));
    return true;
}

bool MLocal(const std::string&, std::string& out) {
    if (!g.rt) return NotReady(out);
    out = g.rt->LocalJson();
    return true;
}

bool MLoopback(const std::string& params, std::string& out) {
    if (!g.rt) return NotReady(out);
    Value p;
    if (!Parse(params, p, out)) return false;
    const Value* off = p.find("offset");
    if (off && off->items().size() == 3)
        g.rt->SetLoopbackOffset(Vec3{static_cast<float>(off->items()[0].asNumber()), static_cast<float>(off->items()[1].asNumber()),
                                     static_cast<float>(off->items()[2].asNumber())});
    bool on = p.get("on").asBool(true);
    bool ok = g.rt->SetLoopback(on);
    out = std::string("{\"loopback\":") + (g.rt->loopback() ? "true" : "false") + ",\"ok\":" + (ok ? "true" : "false") + "}";
    return ok;
}

bool MAdd(const std::string& params, std::string& out) {
    if (!g.rt) return NotReady(out);
    Value p;
    if (!Parse(params, p, out)) return false;
    Appearance a;
    a.race = static_cast<uint32_t>(p.get("race").isString() ? Hex(p.get("race").asString()) : static_cast<uint32_t>(p.get("race").asInt(0)));
    a.female = p.get("female").asBool(false);
    a.name = p.get("name").asString("remote");
    Slot slot = static_cast<Slot>(p.get("slot").asInt(2));
    bool ok = g.rt->AddRemote(slot, a);
    out = std::string("{\"slot\":") + std::to_string((int)slot) + ",\"ok\":" + (ok ? "true" : "false") + "}";
    return ok;
}

bool MRemove(const std::string& params, std::string& out) {
    if (!g.rt) return NotReady(out);
    Value p;
    if (!Parse(params, p, out)) return false;
    g.rt->RemoveRemote(static_cast<Slot>(p.get("slot").asInt(2)));
    out = "{\"ok\":true}";
    return true;
}

bool MFeed(const std::string& params, std::string& out) {
    if (!g.rt) return NotReady(out);
    Value p;
    if (!Parse(params, p, out)) return false;
    Slot slot = static_cast<Slot>(p.get("slot").asInt(2));
    live::ActorSnapshot s;
    s.netId = 0x7F00u + slot;
    s.ownerEpoch = 1;
    s.seq = ++g.feedSeq[slot];
    uint32_t now = NowMs();
    s.tick = now;
    engine::RefInfo pl;
    g.backend->ReadRef(engine::kPlayerRef, pl);
    s.cell = static_cast<live::CellId>(p.get("cell").isString() ? Hex(p.get("cell").asString()) : static_cast<uint32_t>(p.get("cell").asInt(pl.cell)));
    s.pos = live::Vec3{static_cast<float>(p.get("x").asNumber(pl.pos.x)), static_cast<float>(p.get("y").asNumber(pl.pos.y)),
                       static_cast<float>(p.get("z").asNumber(pl.pos.z))};
    s.rot = live::Vec3{0.0f, static_cast<float>(p.get("yaw").asNumber(0)), 0.0f};
    s.vel = live::Vec3{static_cast<float>(p.get("vx").asNumber(0)), static_cast<float>(p.get("vy").asNumber(0)), static_cast<float>(p.get("vz").asNumber(0))};
    s.anim.locomotion = static_cast<uint8_t>(p.get("loco").asInt(0));
    s.anim.weaponDrawn = p.get("weaponDrawn").asBool(false);
    s.anim.sneak = p.get("sneak").asBool(false);
    g.rt->OnRemoteSnapshot(slot, s, now);
    out = "{\"ok\":true,\"seq\":" + std::to_string(s.seq) + "}";
    return true;
}

std::string RefHex(RefId r) {
    char b[16];
    std::snprintf(b, sizeof b, "0x%08X", (unsigned)r);
    return b;
}

// avatar.api -> the ghost-ref map as other modules see it (avatar_api.h): every live puppet, the acting reference of every slot
bool MApi(const std::string&, std::string& out) {
    if (!g.rt) return NotReady(out);
    const IPuppetDirectory* d = puppetDirectory();
    std::string s = "{\"registered\":" + std::string(d ? "true" : "false") + ",\"localSlot\":" + std::to_string(d ? (int)d->LocalSlot() : -1) + ",\"puppets\":[";
    bool first = true;
    forEachPuppet([&](Slot slot, RefId ref) {
        if (!first) s += ",";
        first = false;
        Slot back = kNoSlot;
        const bool rt = slotOfPuppet(ref, back) && back == slot;
        s += "{\"slot\":" + std::to_string((int)slot) + ",\"ref\":\"" + RefHex(ref) + "\",\"acting\":\"" + RefHex(actingRefOf(slot)) + "\",\"roundTrip\":" + (rt ? "true" : "false") + "}";
    });
    s += "],\"localActing\":\"" + RefHex(d ? actingRefOf(d->LocalSlot()) : 0) + "\",\"hits\":" + g.hits.StatusJson() + "}";
    out = s;
    return true;
}

// avatar.look -> the local player's look as it goes on the wire (race, sex, hair, eyes, body, face data) and the worn items with weapon mods
bool MLook(const std::string&, std::string& out) {
    if (!g.rt) return NotReady(out);
    Appearance a;
    const bool ok = g.rt->ReadLocalAppearance(a);
    std::string s = "{\"ok\":" + std::string(ok ? "true" : "false");
    if (ok) {
        s += ",\"race\":\"" + RefHex(a.race) + "\",\"female\":" + (a.female ? "true" : "false") + ",\"hair\":\"" + RefHex(a.hair) + "\",\"eyes\":\"" + RefHex(a.eyes) +
             "\",\"hairColor\":" + std::to_string(a.hairColor) + ",\"height\":" + std::to_string(a.height) + ",\"weight\":" + std::to_string(a.weight) +
             ",\"faceCoeffs\":" + std::to_string(a.faceCoeffs.size()) + ",\"faceHash\":\"" + RefHex(a.faceHash) + "\",\"name\":\"" + a.name + "\"";
    }
    std::vector<engine::EquippedItem> items;
    s += ",\"worn\":[";
    if (g.backend->ReadEquipped(engine::kPlayerRef, items)) {
        for (size_t i = 0; i < items.size(); ++i) {
            if (i) s += ",";
            s += "{\"form\":\"" + RefHex(items[i].form) + "\",\"weapon\":" + (items[i].formType == engine::kFormTypeWeapon ? "true" : "false") + ",\"mods\":" + std::to_string((int)items[i].mods) + "}";
        }
    }
    s += "]}";
    out = s;
    return true;
}

// avatar.puppetlook {slot} -> what the puppet of `slot` really wears (worn items with the weapon modification flags READ BACK from the engine)
bool MPuppetLook(const std::string& params, std::string& out) {
    if (!g.rt) return NotReady(out);
    Value p;
    if (!Parse(params, p, out)) return false;
    const Slot slot = static_cast<Slot>(p.get("slot").asInt(1));
    const RefId ref = puppetRefOf(slot);
    std::string s = "{\"slot\":" + std::to_string((int)slot) + ",\"ref\":\"" + RefHex(ref) + "\",\"worn\":[";
    std::vector<engine::EquippedItem> items;
    if (ref && g.backend->ReadEquipped(ref, items)) {
        for (size_t i = 0; i < items.size(); ++i) {
            if (i) s += ",";
            s += "{\"form\":\"" + RefHex(items[i].form) + "\",\"weapon\":" + (items[i].formType == engine::kFormTypeWeapon ? "true" : "false") + ",\"mods\":" +
                 std::to_string((int)items[i].mods) + ",\"modsKnown\":" + (items[i].modsKnown ? "true" : "false") + "}";
        }
    }
    s += "]}";
    out = s;
    return ref != 0;
}

// avatar.hit {slot, damage, limb?, attackerRef?} -> a hit on the puppet of `slot`, as if an enemy of this machine had shot it. Goes through the
// same router the hit gate uses; the answer says where it went. Dev/proof method: the combat layer ([live] combat=1) must be running for it to be forwarded.
bool MHit(const std::string& params, std::string& out) {
    if (!g.rt) return NotReady(out);
    Value p;
    if (!Parse(params, p, out)) return false;
    const Slot slot = static_cast<Slot>(p.get("slot").asInt(1));
    PuppetHit h;
    h.victimRef = puppetRefOf(slot);
    h.attackerRef = static_cast<RefId>(p.get("attackerRef").isString() ? Hex(p.get("attackerRef").asString()) : static_cast<uint32_t>(p.get("attackerRef").asInt(0)));
    h.healthDmg = static_cast<float>(p.get("damage").asNumber(10));
    h.hitLocation = static_cast<int32_t>(p.get("limb").asInt(-1));
    h.melee = p.get("melee").asBool(false);
#ifdef NVCOOP_WITH_COMBAT
    g.hits.SetPort(&g.hitPort);
#else
    g.hits.SetPort(nullptr);
#endif
    const HitOutcome o = g.hits.OnHit(h);
    static const char* kRoute[] = {"notAPuppet", "ignored", "forwarded", "noPort", "portRefused"};
    static const char* kAct[] = {"proceed", "clamp", "neutralize"};
    out = std::string("{\"victimRef\":\"") + RefHex(h.victimRef) + "\",\"route\":\"" + kRoute[static_cast<int>(o.route)] + "\",\"action\":\"" +
          kAct[static_cast<int>(o.action)] + "\",\"slot\":" + std::to_string(o.victimSlot == kNoSlot ? -1 : (int)o.victimSlot) + ",\"router\":" + g.hits.StatusJson() + "}";
    return o.route != HitRoute::NotAPuppet;
}

bool MPreSave(const std::string&, std::string& out) {
    if (!g.rt) return NotReady(out);
    g.rt->OnEvent(GameEvent::SaveBegin);
    g.presaved = true;
    out = "{\"ok\":true}";
    return true;
}

bool MPostSave(const std::string&, std::string& out) {
    if (!g.rt) return NotReady(out);
    g.rt->OnEvent(GameEvent::SaveEnd);
    g.presaved = false;
    out = "{\"ok\":true}";
    return true;
}

// ---- module callbacks ------------------------------------------------------------------------------------------------------

void OnLoad(void* nvse) {
    g.enabled = GetPrivateProfileIntA("avatar", "enabled", 0, IniPath().c_str()) != 0;
#ifdef NVCOOP_WITH_LIVE
    // dll-wiring: [live] avatars=1 (read in NVSEPlugin_Load before any module loads) also creates the runtime, so one switch is enough in a co-op game
    if (nvcoop::livewire::glueSwitches().avatars) g.enabled = true;
#endif
    if (!g.enabled) return;
    if (nvse) g.console = static_cast<NVSEConsoleInterface*>(static_cast<NVSEInterface*>(nvse)->QueryInterface(kInterface_Console));
    if (!g.console) {
        nvcoop::Log("avatar: enabled but the NVSE console interface is missing: layer off");
        g.enabled = false;
        return;
    }
    g.backend = std::make_unique<WinBackend>(g.console);
    ParseConfig();
    nvcoop::RegisterMethod("avatar.status", MStatus, true);
    nvcoop::RegisterMethod("avatar.local", MLocal, true);
    nvcoop::RegisterMethod("avatar.trace", MTrace, true);
    nvcoop::RegisterMethod("avatar.loopback", MLoopback, true);
    nvcoop::RegisterMethod("avatar.add", MAdd, true);
    nvcoop::RegisterMethod("avatar.remove", MRemove, true);
    nvcoop::RegisterMethod("avatar.feed", MFeed, true);
    nvcoop::RegisterMethod("avatar.api", MApi, true);
    nvcoop::RegisterMethod("avatar.look", MLook, true);
    nvcoop::RegisterMethod("avatar.hit", MHit, true);
    nvcoop::RegisterMethod("avatar.puppetlook", MPuppetLook, true);
    nvcoop::RegisterMethod("avatar.presave", MPreSave, true);
    nvcoop::RegisterMethod("avatar.postsave", MPostSave, true);
    nvcoop::Log("avatar: enabled ([avatar] enabled=1), backend ready=%d, template=%s", g.backend->Ready() ? 1 : 0, g.templatePlayer ? "player" : "configured");
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (!g.enabled || !g.backend) return;
    switch (type) {
        case NVSEMessagingInterface::kMessage_PreLoadGame:
            g.backend->NoteWorldChanged();
            if (g.rt) g.rt->OnEvent(GameEvent::LoadBegin);
            break;
        case NVSEMessagingInterface::kMessage_PostLoadGame:
            g.backend->NoteWorldChanged();
            if (g.rt) g.rt->OnEvent(GameEvent::LoadEnd);
            break;
        case NVSEMessagingInterface::kMessage_NewGame:
        case NVSEMessagingInterface::kMessage_ExitToMainMenu:
            g.backend->NoteWorldChanged();
            if (g.rt) g.rt->OnEvent(GameEvent::NewGame);
            break;
        default: break;
    }
}

void OnFrame() {
    if (!g.enabled || !g.backend) return;
    ++g.frames;
    if (!g.rt) {
        if (nvcoop::avatar::shouldCreateRuntime(g.enabled, g.backend != nullptr, false, g.frames, (g.frames % nvcoop::avatar::kRuntimeCheckEveryFrames) == 0 && PlayerReady())) CreateRuntime();
        if (!g.rt) return;
    }
    g.rt->Tick(NowMs());
}

}  // namespace

NVCOOP_MODULE(m2_avatar, "m2_avatar", OnLoad, OnMessage, OnFrame);

// ---- glue for plugin/coop ------------------------------------------------------------------------------------------------------

namespace glue {

bool Enabled() { return g.enabled; }
bool Running() { return g.rt != nullptr; }
const char* NotRunningBecause() {
    return nvcoop::avatar::notRunningBecause(g.enabled, g.rt != nullptr);
}

void SetSnapshotSink(std::function<void(const live::ActorSnapshot&)> sink) {
    g.snapSink = sink;
    if (g.rt) g.rt->SetSnapshotSink(std::move(sink));
}
void SetEquipSink(std::function<void(const std::vector<EquipEntry>&)> sink) {
    g.equipSink = sink;
    if (g.rt) g.rt->SetEquipSink(std::move(sink));
}
void SetLocalSlot(Slot slot) {
    g.cfg.puppet.localSlot = slot;      // a runtime that does not exist yet is created with it
    g.cfg.loopbackSlot = static_cast<Slot>(slot == 1 ? 2 : 1);
    fnvcoop::live::processEntities().setLocalSlot(slot);
    if (g.rt) {
        g.rt->SetLocalSlot(slot);
        if (g.dir) g.dir->SetLocalSlot(slot);
    }
}
void SetLocalIdentity(live::NetId netId, live::OwnerEpoch epoch) {
    g.netId = netId;
    g.epoch = epoch;
    if (g.rt) g.rt->SetLocalIdentity(netId, epoch);
}
bool AddRemote(Slot slot, const Appearance& a) {
    if (!g.enabled) return false;
    g.pendingRemotes[slot] = a;
    return g.rt ? g.rt->AddRemote(slot, a) : true;
}
void RemoveRemote(Slot slot) {
    g.pendingRemotes.erase(slot);
    g.pendingEquip.erase(slot);
    g.pendingPose.erase(slot);
    if (g.rt) g.rt->RemoveRemote(slot);
}
bool SetRemoteAppearance(Slot slot, const Appearance& a) {
    if (!g.enabled) return false;
    g.pendingRemotes[slot] = a;   // remembered for a runtime that does not exist yet (AddRemote replays it)
    return g.rt ? g.rt->SetRemoteAppearance(slot, a) : true;
}
bool ReadLocalAppearance(Appearance& out) { return g.rt && g.rt->ReadLocalAppearance(out); }
bool ReadLocalFacts(PlayerFacts& out) { return g.rt && g.rt->ReadLocalFacts(out); }
bool ReadLocalPack(const std::vector<FormId>& watch, PlayerPack& out) { return g.rt && g.rt->ReadLocalPack(watch, out); }
bool ReadLocalEquipment(std::vector<EquipEntry>& out) { return g.rt && g.rt->ReadLocalEquipment(out); }
void OnRemoteSnapshot(Slot slot, const live::ActorSnapshot& s, uint32_t arrivalMs) {
    if (g.rt) g.rt->OnRemoteSnapshot(slot, s, arrivalMs);
}
void NoteStage(NetStage st, double ms) {
    if (g.rt) g.rt->NoteStage(st, ms);
}
void OnRemoteEquipment(Slot slot, const std::vector<EquipEntry>& wanted) {
    if (g.rt) g.rt->OnRemoteEquipment(slot, wanted);
    else if (g.enabled) g.pendingEquip[slot] = wanted;   // the reliable look message is not repeated: keep it until the runtime exists
}
void PreSave() {
    if (g.rt) g.rt->OnEvent(GameEvent::SaveBegin);
    g.presaved = g.rt != nullptr;
}
void PostSave() {
    if (g.rt) g.rt->OnEvent(GameEvent::SaveEnd);
    g.presaved = false;
}
void SetPoseOverride(Slot slot, int baseGroup) {
    if (!g.enabled) return;
    if (baseGroup == kNoGroup) g.pendingPose.erase(slot);
    else g.pendingPose[slot] = baseGroup;
    if (g.rt) g.rt->SetPoseOverride(slot, baseGroup);
}
void SetDownedPose(Slot slot, bool downed) { SetPoseOverride(slot, downed ? static_cast<int>(kGrpDeath) : kNoGroup); }
RefId PuppetRef(Slot slot) { return g.rt ? g.rt->RefOf(slot) : 0; }
std::optional<Slot> PuppetSlot(RefId ref) { return g.rt ? g.rt->SlotOf(ref) : std::nullopt; }

}  // namespace glue
}  // namespace nvcoop::avatar
