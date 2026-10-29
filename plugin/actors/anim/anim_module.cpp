// The NPC animation module inside nvcoop.dll (enemy-anim). Self-registering (NVCOOP_MODULE); needs no edit in main.cpp.
//
//   nvcoop.ini [anim]   enabled=1            0 = the layer installs nothing (the port still drives position, copies just do not animate)
//                       directweaponout=1    call BaseProcess::SetWeaponOut (pinned slot) before the SetAlert fallback
//                       directaiming=1       call BaseProcess::SetIsAiming (pinned slot)
//                       directrate=0         write the movement sequence frequency for speed matching (offset not proven)
//                       directknock=0        PushActorAway / SetKnockedState for knockdowns (units and effect not measured)
//                       attacks=1 turns=1 talking=1 rate=1 knockdown=1 sit=1   feature switches of the driver
//   bridge method       anim.npc             counters, the pin status of the direct calls, and the system state
//   bridge method       anim.npc.do          DEV ONLY ([dev] animprobe=1): run ONE backend operation on a plain reference, for the in-game proof
//                                            (tools/animcheck/npc_proof.py): {ref, op: play|release|weapon|aim|sneak|rate|knock|getup|kill|place|sit|stand|restrain|read, ...}
//                                            (sit / stand take "furniture" as a hex reference string; stand also "attempt": 0 gentle, 1 escalated)
//
// Wiring (all through plugin/actors/engine/actor_engine_api.h): setAnimSampler (owner side), addDriveListener (non-owner side),
// addActorDiedListener, addActorLostListener; the corpse event travels as an ActorSync MAnimEvent (ActorSync::setAnimEventHandler /
// sendAnimEvent), attached whenever the coop session's ActorSync changes.
#include <Windows.h>

#include <memory>
#include <string>

#include "actors/engine/actor_engine_api.h"
#include "anim_game.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "hooks/engine_hooks.h"
#include "npc_anim_system.h"
#include "script_npc_engine.h"

#ifdef NVCOOP_WITH_ACTORS
#include "actors/actor_glue.h"
#endif

#include "nvse/PluginAPI.h"

namespace {

using namespace nvcoop::actors;
using namespace nvcoop::actors::anim;

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
bool IniBool(const char* key, bool def) { return GetPrivateProfileIntA("anim", key, def ? 1 : 0, IniPath().c_str()) != 0; }

uint32_t g_epoch = 1;   // bumped by every load / new game / return to the main menu

struct SyncNet : IAnimNet {
    bool sendEvent(NetId id, const std::vector<uint8_t>& payload) override {
#ifdef NVCOOP_WITH_ACTORS
        ActorSync* s = glueSync();
        return s && s->sendAnimEvent(id, payload);
#else
        (void)id; (void)payload;
        return false;
#endif
    }
};

struct State {
    SyncNet net;
    std::unique_ptr<game::GameScript> script;
    std::unique_ptr<game::GameAccess> access;
    std::unique_ptr<ScriptNpcAnimEngine> backend;
    std::unique_ptr<NpcAnimSystem> sys;
    ActorSync* attached = nullptr;
    engine::ListenerId drive = 0, died = 0, lost = 0;
};
State* g_state = nullptr;

uint32_t NowMs() { return static_cast<uint32_t>(GetTickCount64()); }

bool MAnimNpc(const std::string&, std::string& out) {
    if (!g_state || !g_state->sys) { out = "{\"enabled\":false}"; return true; }
    out = "{\"enabled\":true,\"pins\":" + g_state->access->pinStatusJson() + ",\"stats\":" + g_state->sys->statsJson() + "}";
    return true;
}

bool g_devProof = false;

// One backend operation on a plain reference (not tracked by the actor port) so each primitive can be proven alone, in a game, with anim.probe
// as the witness. The ref gets a private NetId; nothing here touches ActorSync.
bool MAnimDo(const std::string& params, std::string& out) {
    if (!g_state || !g_state->backend || !g_devProof) { out = "anim.npc.do is off ([anim] enabled=1 and [dev] animprobe=1 are needed)"; return false; }
    nvcoop::json::Value p;
    std::string err;
    if (!nvcoop::json::parse(params, p, err) || !p.isObject()) { out = "bad params: " + err; return false; }
    const nvcoop::json::Value& rv = p.get("ref");
    uint32_t ref = 0;
    if (rv.isNumber()) ref = static_cast<uint32_t>(rv.asInt());
    else if (rv.isString()) ref = static_cast<uint32_t>(std::strtoul(rv.asString().c_str(), nullptr, 16));
    if (!ref) { out = "ref is required (hex string or number)"; return false; }
    const NetId id = 0xFFFF0000u | (ref & 0xFFFFu);
    game::setRefOverride(id, ref);
    ScriptNpcAnimEngine& e = *g_state->backend;
    const std::string op = p.get("op").asString("");
    Result r = Result::Refused;
    if (op == "read") {
        NpcReads rd;
        const bool ok = e.Read(id, rd);
        nvcoop::json::Value o = nvcoop::json::Value::object();
        o.set("ok", ok).set("valid", rd.valid).set("has3D", rd.has3D).set("dead", rd.dead).set("creature", rd.creature).set("knocked", rd.knocked).set("ragdoll", rd.ragdoll)
         .set("swimming", rd.swimming).set("sneaking", rd.sneaking).set("weaponOut", rd.weaponOut).set("aiming", rd.aiming).set("moveFlagsKnown", rd.moveFlagsKnown)
         .set("moveFlags", static_cast<int64_t>(rd.moveFlags)).set("moveGroup", rd.moveGroup).set("actionGroup", rd.actionGroup).set("actionPlayMs", rd.actionPlayMs)
         .set("sitKnown", rd.sitKnown).set("sitSleep", rd.sitSleep).set("furniture", static_cast<int64_t>(rd.furniture))
         .set("generation3D", static_cast<int64_t>(e.Generation3D(id)));
        CellId cell = 0; Vec3 pos; float yaw = 0;
        if (e.GetPose(id, cell, pos, yaw)) o.set("cell", static_cast<int64_t>(cell)).set("x", pos.x).set("y", pos.y).set("z", pos.z).set("yaw", yaw);
        out = o.dump();
        return true;
    }
    if (op == "play") r = e.PlayGroup(id, static_cast<int>(p.get("group").asInt(0)), p.get("hold").asBool(true));
    else if (op == "release") r = e.ReleaseHold(id);
    else if (op == "weapon") r = e.SetWeaponDrawn(id, p.get("on").asBool(true));
    else if (op == "aim") r = e.SetAiming(id, p.get("on").asBool(true));
    else if (op == "sneak") r = e.SetSneak(id, p.get("on").asBool(true));
    else if (op == "rate") r = e.SetGroupRate(id, static_cast<float>(p.get("value").asNumber(1.0)));
    else if (op == "knock") r = e.KnockDown(id, static_cast<float>(p.get("yaw").asNumber(0)), static_cast<float>(p.get("force").asNumber(1.0)));
    else if (op == "getup") r = e.GetUp(id);
    else if (op == "kill") r = e.Kill(id);
    else if (op == "sit") r = e.SitIn(id, static_cast<uint32_t>(std::strtoul(p.get("furniture").asString("0").c_str(), nullptr, 16)));
    else if (op == "stand") r = e.StandUp(id, static_cast<uint32_t>(std::strtoul(p.get("furniture").asString("0").c_str(), nullptr, 16)), static_cast<uint32_t>(p.get("attempt").asInt(0)));
    else if (op == "restrain") r = e.Restrain(id);
    else if (op == "place") r = e.PlaceCorpse(id, Vec3{static_cast<float>(p.get("x").asNumber()), static_cast<float>(p.get("y").asNumber()), static_cast<float>(p.get("z").asNumber())},
                                              static_cast<float>(p.get("yaw").asNumber(0)));
    else { out = "unknown op '" + op + "'"; return false; }
    out = std::string("{\"result\":\"") + (r == Result::Ok ? "ok" : r == Result::Unsupported ? "unsupported" : "refused") + "\"}";
    return true;
}

void OnLoad(void*) {
    if (!IniBool("enabled", true)) {
        nvcoop::Log("enemy-anim: disabled ([anim] enabled=0)");
        return;
    }
    g_state = new State();
    State& st = *g_state;

    game::GameConfig gc;
    gc.directWeaponOut = IniBool("directweaponout", gc.directWeaponOut);
    gc.directAiming = IniBool("directaiming", gc.directAiming);
    gc.directRate = IniBool("directrate", gc.directRate);
    gc.directKnock = IniBool("directknock", gc.directKnock);
    SystemConfig sc;
    sc.driver.playAttacks = IniBool("attacks", sc.driver.playAttacks);
    sc.driver.playTurns = IniBool("turns", sc.driver.playTurns);
    sc.driver.playTalking = IniBool("talking", sc.driver.playTalking);
    sc.driver.useRate = IniBool("rate", sc.driver.useRate);
    sc.driver.knockdown = IniBool("knockdown", sc.driver.knockdown);
    sc.driver.playSit = IniBool("sit", sc.driver.playSit);

    st.script.reset(new game::GameScript([](uint32_t ref, const std::string& line) { return nvhooks::execConsoleLineOn(line, ref); }));
    st.access.reset(new game::GameAccess(gc, [] { return g_epoch; }));
    st.backend.reset(new ScriptNpcAnimEngine(*st.script, *st.access));
    st.sys.reset(new NpcAnimSystem(*st.backend, st.net, sc));

    NpcAnimSystem* sys = st.sys.get();
    engine::setAnimSampler([sys](engine::RefId, live::ActorSnapshot& snap) { sys->sampleOwned(snap.netId, snap, NowMs()); });
    st.drive = engine::addDriveListener([sys](const engine::DriveEvent& e) { if (e.out) sys->drive(e.netId, *e.out, NowMs()); });
    st.died = engine::addActorDiedListener([sys](const engine::DiedEvent& e) { sys->onDied(e.netId, e.mine, e.fromNetwork, NowMs()); });
    st.lost = engine::addActorLostListener([sys](const engine::LostEvent& e) { sys->onLost(e.netId); });
    engine::setPoseLock([sys](NetId id) { return sys->driver().poseLocked(id); });   // a copy getting out of furniture is not moved yet

    nvcoop::RegisterMethod("anim.npc", MAnimNpc, true);
    g_devProof = GetPrivateProfileIntA("dev", "animprobe", 0, IniPath().c_str()) != 0;
    if (g_devProof) nvcoop::RegisterMethod("anim.npc.do", MAnimDo, true);
    nvcoop::Log("enemy-anim: installed (directWeaponOut=%d directAiming=%d directRate=%d directKnock=%d); bridge method anim.npc", gc.directWeaponOut, gc.directAiming, gc.directRate, gc.directKnock);
}

void OnMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_PostLoadGame || type == NVSEMessagingInterface::kMessage_NewGame ||
        type == NVSEMessagingInterface::kMessage_ExitToMainMenu)
        ++g_epoch;
}

void OnFrame() {
    if (!g_state || !g_state->sys) return;
    State& st = *g_state;
#ifdef NVCOOP_WITH_ACTORS
    ActorSync* s = glueSync();
    if (s != st.attached) {   // a new coop session (or none): the corpse events hook into the new ActorSync
        st.attached = s;
        if (s) {
            NpcAnimSystem* sys = st.sys.get();
            s->setAnimEventHandler([sys](const ActorSync::AnimEventIn& e) { sys->onNetEvent(e.id, e.payload, NowMs()); });
        }
    }
#endif
    const uint32_t now = NowMs();
    // scripted resurrects: the actor port clears the dead flag in ActorSync; the animation layer starts driving the copy again
    for (NetId id : st.sys->driver().deadActors()) {
        const engine::OwnerRef o = engine::ownerOfNet(id);
        if (o.known && !o.dead) st.sys->onRevived(id);
    }
    st.sys->tick(now, [](NetId id) { return engine::ownerOfNet(id).mine; });
}

}  // namespace

NVCOOP_MODULE(enemy_anim, "enemy_anim", OnLoad, OnMessage, OnFrame);
