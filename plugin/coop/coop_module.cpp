// nvcoop module (L10): starts the co-op Runtime inside the game and exposes the bridge methods.
//   nvcoop.ini  [coop] enable=1  role=host|client|guest  listen=127.0.0.1  host=127.0.0.1  port=27960  token=<n>
//   coop.status                      -> role/running/peers/last seq/txn counts (Runtime::statusJson)
//   coop.log   {n?:int}              -> recent txns (commit on host, apply on client, inject)
//   coop.inject {txn:{cause,origin,effects}} -> host: commit as if captured; client: apply as if received
// Starts lazily from onFrame once L5's hooks are initialised (nvhooks::hub() != null), so the module order of the
// DeferredInit message does not matter. All of it runs on the game main thread.
#include <Windows.h>

#include <algorithm>
#include <cctype>
#include <cstdlib>
#include <cstring>
#include <memory>
#include <set>
#include <unordered_map>
#include <string>

#include "GameAPI.h"
#include "GameData.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "GameScript.h"
#include "intents/engine/game_engine.h"   // engine().notify: the game's HUD line
#include "PluginAPI.h"
#include "apply_remote.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "reload/reload_state.h"
#include "baseline_file.h"
#include "coop_runtime.h"
#ifdef NVCOOP_WITH_FLIGHT
#include "flight/flight_service.h"
#endif
#include "effect_json.h"
#ifdef NVCOOP_WITH_UI
#include "hooks_ui/ui_hooks.h"
#include "info_reader.h"
#endif
#include "engine_hooks.h"
#include "apply_lines.h"
#include "intents/engine/intents_saves.h"
#include "intents/stage_pass.h"
#include "party_state.h"
#include "stash_order.h"
#include "save_sidecars.h"
#include "objvar_sidecar.h"
#include "party_save_glue.h"   // party saves: every host save is a save of every guest too (docs/party-save.md)
#ifdef NVCOOP_WITH_MARKERS
#include "markers/mk_glue.h"   // friend markers and the revive of a downed friend (docs/markers.md, docs/revive.md)
#endif
#ifdef NVCOOP_WITH_UI
#include "guest_dialogue.h"   // fndlg::GuestDialogue: the live half of a guest conversation (G05, G33)
#endif
#ifdef NVCOOP_WITH_NVMP
#include "nvmp/ghost_ref.h"
#endif
#ifdef NVCOOP_WITH_SNAPSHOT
#include "fnvcoop_snapshot/baseline_dump.h"
#endif
#ifdef NVCOOP_WITH_WORLD
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

#ifdef NVCOOP_WITH_WORLD
// L12 world effects: SpawnRef / DespawnRef / RefScriptVar go through nvworld::WorldApplier (host-ref -> local-ref map, cell-not-ready queue,
// duplicate guard); the lane-local WorldEffects (NPC actor values, essential/restrained, relations, packages) ride Runtime::pushWorldEffect.
std::unique_ptr<nvworld::WorldApplier> g_worldApplier;

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
            nvhooks::ApplyStatus st = g_worldApplier->applyAny(e, ctx.origin);   // identity layer: "the player" in it is the sender's
            ApplyResult r;
            r.ok = st == nvhooks::ApplyStatus::Ok;
            r.status = nvhooks::applyStatusName(st);
            if (!r.ok && !g_worldApplier->lastError().empty()) r.status += ": " + g_worldApplier->lastError();
            return r;
        }
#endif
        // session-level effects the engine layer realizes locally (no engine primitive involved). SetPlayerControls is NOT one of them any more (G18):
        // the committed effect carries its recipient and mask, the applier below sets exactly those bits and a release frees only its own.
        if (std::get_if<fnvcoop::SaveSuppress>(&e) || std::get_if<fnvcoop::PartyTether>(&e) || std::get_if<fnvcoop::BarrierRelease>(&e) ||
            std::get_if<fnvcoop::BarrierArrive>(&e))
            return {true, "Ok"};   // state lives in the shared shadow (coop.status.saveSuppress); nothing to write into the engine
        if (!applier) return {false, "NoApplier"};
        if (const auto* ai = std::get_if<fnvcoop::AddItem>(&e))   // reward trace: which txn / effect lands here, and through which path (live, catch-up, baseline, repair)
            nvcoop::Log("coop: apply AddItem %s:%X x%d seq %llu idx %u origin %d epoch %u live=%d catchUp=%d baseline=%d repair=%d snapshot=%d", ai->item.plugin.c_str(), (unsigned)ai->item.localId, (int)ai->count,
                        (unsigned long long)ctx.seq, (unsigned)ctx.index, (int)ctx.origin, (unsigned)ctx.epoch, ctx.seq != 0 && !ctx.baseline && !ctx.repair, (int)ctx.catchUp, (int)ctx.baseline, (int)ctx.repair, (int)ctx.fromSnapshot);
        {   // quest-state trace: which txn starts / stops / stages a quest (Sunny's "Everything all right?" greeting needs VCG02 running; who started it?)
            const fnvcoop::FormRef* q = nullptr; const char* what = nullptr; int stage = -1;
            if (const auto* x = std::get_if<fnvcoop::StartQuest>(&e)) { q = &x->quest; what = "StartQuest"; }
            else if (const auto* x = std::get_if<fnvcoop::StopQuest>(&e)) { q = &x->quest; what = "StopQuest"; }
            else if (const auto* x = std::get_if<fnvcoop::SetStage>(&e)) { q = &x->quest; what = "SetStage"; stage = int(x->stage); }
            if (q) nvcoop::Log("coop: apply %s %s:%X %d seq %llu idx %u origin %d live=%d baseline=%d repair=%d", what, q->plugin.c_str(), (unsigned)q->localId, stage,
                               (unsigned long long)ctx.seq, (unsigned)ctx.index, (int)ctx.origin, ctx.seq != 0 && !ctx.baseline && !ctx.repair, (int)ctx.baseline, (int)ctx.repair);
        }
        // baseline effects FORCE-SET (may lower a stage, full objective flag byte); live txns merge normally
        nvhooks::ApplyStatus st = ctx.baseline ? applier->applyForce(e) : applier->applyFrom(ctx.origin, e);   // identity layer: "the player" in it is the sender's
        ApplyResult r;
        r.ok = st == nvhooks::ApplyStatus::Ok;
        r.status = nvhooks::applyStatusName(st);
        if (!r.ok && !applier->lastError().empty()) r.status += ": " + applier->lastError();
        return r;
    }
    void onJoined(fnvcoop::PlayerId slot, const fnvcoop::SharePolicy&) override {
        nvcoop::Log("coop: joined as slot %d", (int)slot);
    }
    nvhooks::RemoteApplier* applier = nullptr;
};

// Engine dumper for the baseline join: L7's DumpBaselineShadow (needs the snapshot library linked in).
class EngineDumper : public IWorldDumper {
public:
    bool dump(fnvcoop::ShadowState* out, uint64_t* hash, std::string* err) override {
#ifdef NVCOOP_WITH_SNAPSHOT
        bool ok = fnvcoop::snapshot::DumpBaselineShadow(out, hash, err);
        nvcoop::Log("coop: baseline dump %s; %s", ok ? "ok" : "FAILED", fnvcoop::snapshot::LastBaselineStats().summary.c_str());
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
// Guest pack facts: nvcoop_pack_watch.json {"schema":1,"items":["falloutnv.esm:0132ab", ...]} (plugin/coop/tools/gen_coop_data.py): the item forms the rules and the quest conditions count.
// Both roles load it: the guest reads those counts from its own pack, the host reads an absent form as zero only for a form on its own list.
std::vector<fnvcoop::FormRef> g_packWatchRefs;
void LoadPackWatch() {
    g_packWatchRefs.clear();
    const std::string path = DllDir() + "nvcoop_pack_watch.json";
    const std::string text = ReadWholeFile(path);
    if (text.empty()) { nvcoop::Log("coop: nvcoop_pack_watch.json not found: a guest's item counts and weight stay the puppet's"); return; }
    nvcoop::json::Value v;
    std::string err;
    if (!nvcoop::json::parse(text, v, err)) { nvcoop::Log("coop: cannot parse %s: %s", path.c_str(), err.c_str()); return; }
    for (const auto& it : v.get("items").items()) {
        fnvcoop::FormRef r;
        if (it.isString() && nvhooks::strToRef(it.asString(), &r)) g_packWatchRefs.push_back(r);
    }
    nvcoop::Log("coop: pack watch list: %u item forms", (unsigned)g_packWatchRefs.size());
}
std::unordered_map<uint32_t, std::string> g_scriptNames;   // runtime form id -> lower-case EDID of the scripts that read the companion flags party-wide (companion_sets.txt view-party)
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

// G26, the guest side: a barrier step (Doc's farewell, Elijah's talks, an ending) runs in the host's game only, so this guest's game never asks for the stage.
// Pass every open barrier once the player is free (MenuMode off: no message box, talk or Pip-Boy open) and the barrier has been open for a moment; resend at most
// every 2 s until the host's arrival comes back (the list then drops it). Story run 2026-10-28: the host stood 120 s after Doc's farewell without this.
std::map<fnvcoop::FormRef, uint64_t> g_barrierSeen, g_barrierSent;
bool GuestChargenPending();   // below: this guest is still making its character (Doc's goodbye must wait for it)
void StartGuestChargen();
void GuestBarrierPasses() {
    fnvcoop::ClientSession* c = g_rt ? g_rt->client() : nullptr;
    if (!c || !c->joined() || c->baselineActive() || GuestChargenPending()) return;
    const uint64_t now = NowMs();
    const std::vector<fnvcoop::FormRef> open = fnvcoop::RulesEngine::guestBarrierPasses(c->state(), c->slot());
    for (auto it = g_barrierSeen.begin(); it != g_barrierSeen.end();)
        it = std::find(open.begin(), open.end(), it->first) == open.end() ? g_barrierSeen.erase(it) : std::next(it);
    const bool menu = *reinterpret_cast<volatile const uint8_t*>(0x11DEA2B) != 0;   // the main loop's MenuMode() byte (plugin/vats/vats_policy.h)
    for (const auto& q : open) {
        const uint64_t seen = g_barrierSeen.emplace(q, now).first->second;
        if (menu || now - seen < 1500) continue;
        auto s = g_barrierSent.find(q);
        if (s != g_barrierSent.end() && now - s->second < 2000) continue;
        g_barrierSent[q] = now;
        nvcoop::Log("coop: barrier of %s:%X is open and this player is free -> ScenePass", q.plugin.c_str(), q.localId);
        g_rt->scenePass(q);
    }
}

// The party's start (launcher lobby, main.cpp PrepareStartSave): every machine loads nvcoop_start, taken right after the opening movie:
// a black screen at Doc Mitchell's, VCG00 at its blackout stage with its timer stopped (bRunTimer 0). Everybody waits there until the whole
// party is in ([coop] party = the lobby's size); then each machine plays the opening movie at the same moment and VCG00's timer runs again.
// VCG00's Barrier rule holds its stage 100 (waking up at Doc's) until the last player is through the movie, so the party wakes up together.
// Host: counts the guests that said they are in and orders the start. Guest: says it is in once joined, starts on the host's order.
struct IntroHold {
    bool check = false, armed = false, started = false, woke = false;
    uint64_t armedAt = 0, goAt = 0, lastReady = 0;
    int shown = -1;
    std::set<fnvcoop::PlayerId> ready;
};
IntroHold g_intro;
constexpr uint64_t kIntroGiveUpMs = 5 * 60 * 1000;   // a player who never arrives does not keep the others on a black screen for good
constexpr uint64_t kIntroOrderLeadMs = 500;           // the host's own movie blocks its main loop: let the start orders go out first

const fnvcoop::FormRef& Vcg00() {
    static const fnvcoop::FormRef r = [] { fnvcoop::FormRef f; nvhooks::strToRef("falloutnv.esm:102037", &f); return f; }();
    return r;
}

// The state the start save was taken in: VCG00 past the movie (stage 90 or 95, black screen), not yet at 100 (waking up at Doc's).
// Its timer is stopped here (the start save has it stopped already); the movie's end starts it again.
bool AtStartSaveHold() {
    const int st = nvhooks::localQuestStage(Vcg00());
    if (st < 90 || st >= 100) return false;
    double timer = 0;
    if (nvhooks::questVarValue(Vcg00(), "bRunTimer", &timer) && timer != 0) nvhooks::setQuestVarValue(Vcg00(), "bRunTimer", 0);
    return true;
}

void IntroNotice(const std::string& s) { nvcoop::intents::game::engine().notify(s); }

void StartIntro(const char* why) {
    if (g_intro.started) return;
    g_intro.started = true;
    nvcoop::Log("coop: intro starts (%s)", why);
    nvhooks::ApplierExecScope exec;
    nvhooks::LocalPlayerScope own;   // the movie is this machine's own screen
    // VCG00 stage 0's own line. It blocks this frame until the movie ends or the player skips it.
    const bool bink = nvhooks::execConsoleLine("PlayBink \"FNVIntro.bik\" 1 1 0 1");
    const bool timer = nvhooks::setQuestVarValue(Vcg00(), "bRunTimer", 1);
    nvcoop::Log("coop: intro movie %s, VCG00 timer %s", bink ? "played" : "FAILED", timer ? "running again" : "NOT restarted");
}

// Guest: the host's VCG00 stage 100 (wake up at Doc's) reached this machine. The stage's own lines for the player's screen ran in the host's
// game only, so this guest still sees the intro's black screen: run them here for this machine's player (VCG00 stage 100 and the fade-in
// of VCG01 stage 0, the white of waking up). The host's own game ran them itself.
void IntroWakeUp() {
    g_intro.woke = true;
    nvhooks::ApplierExecScope exec;
    nvhooks::LocalPlayerScope own;   // these lines are this machine's own player and screen: a guest's game does not drop them, a host never captures them
    static const char* const kLines[] = {
        "RemoveImageSpaceModifier 00026370",   // MQ08AllBlackedOutISFX, the intro's black screen
        "RemoveImageSpaceModifier 000AC03C",   // WhiteISFX
        "ReleaseWeatherOverride",
        "ClearScreenSplatter",
        "ApplyImageSpaceModifier 00120E95",    // VCG01FadeInFromBlackISFX
    };
    int ok = 0;
    for (const char* l : kLines) ok += nvhooks::execConsoleLine(l) ? 1 : 0;
    ok += nvhooks::execConsoleLineOn("SetScale 1", 0x14) ? 1 : 0;   // SetScale needs a calling reference: this machine's own player (0x14)
    nvcoop::Log("coop: intro over, this guest wakes up at Doc's (%d of 6 lines ran)", ok);
    StartGuestChargen();
}

// Guest character creation. Doc's scene (VCG01) runs in the host's game, so its menus open on the host's screen only. A guest gets the
// same menus one after another as soon as it is awake, while the host goes through Doc's scene: name, face, SPECIAL, tag skills, traits.
// The order and the lines are the game's own: VCG04ActivatorScript (Doc's "edit your character" machine) and the Vit-o-Matic
// (VCG01VigorTesterSCRIPT) for the SPECIAL points. Each menu opens once the previous one is closed. Doc's goodbye (VCG01 stage 200) is a
// Barrier stage: GuestBarrierPasses does not pass it for this guest before the last menu is closed, so nobody leaves without a character.
struct GuestChargen {
    bool running = false, done = false;
    int step = 0;
    uint64_t nextAt = 0, issuedAt = 0;
    std::set<int> before;   // menus open when the step's line ran: the one that appears after it is the step's own menu
    int menu = 0;           // that menu's id (0 = not seen yet)
    int closedFrames = 0;
    int blockedBy = 0;      // another menu the next step waits for (logged once)
};
GuestChargen g_chargen;
constexpr uint64_t kChargenAfterWakeMs = 4000;   // the wake-up fade first
constexpr uint64_t kChargenOpenWaitMs = 3000;    // a menu that never opens (a line the game refused) is skipped
// Lines that run first (VCG04's own resets), then the line that opens the step's menu. SPECIAL and tag skills come from Doc's machine, not the
// Vit-o-Matic: its menu (ShowLoveTesterMenuParams, love_tester_menu.xml) is drawn for the machine in front of the player and a guest saw only the
// blurred room (2026-10-29); SetSPECIALPoints opens the plain SPECIAL screen (char_gen_menu.xml), 40 points as in Doc's scene, and the tag skills
// use VCG01 stage 90's own line. SetAV / SetSPECIALPoints / SetTagSkills are character changes a guest's game drops inside a script
// (SuppressPolicy::blockMutation), so GuestChargenFrame runs every step inside a nvhooks::LocalPlayerScope (this machine's own player; a host would not
// capture them either). The lines below carry no `player.` prefix: the ones marked onPlayer need a calling reference, so they run with
// execConsoleLineOn(line, 0x14) (the same as `player.<command>`); the three menu openers take none.
constexpr const char* kSpecialReset[] = {"SetAV Strength 5", "SetAV Perception 5", "SetAV Endurance 5", "SetAV Charisma 5",
                                         "SetAV Intelligence 5", "SetAV Agility 5", "SetAV Luck 5", nullptr};
constexpr const char* kSkillReset[] = {"SetAV Barter 0", "SetAV EnergyWeapons 0", "SetAV Explosives 0", "SetAV Guns 0",
                                       "SetAV Lockpick 0", "SetAV Medicine 0", "SetAV MeleeWeapons 0", "SetAV Repair 0",
                                       "SetAV Science 0", "SetAV Sneak 0", "SetAV Speech 0", "SetAV Survival 0",
                                       "SetAV Unarmed 0", nullptr};
struct ChargenStep { const char* what; const char* line; const char* const* first; bool onPlayer; };   // onPlayer: line and `first` need the player as calling reference
constexpr ChargenStep kChargenSteps[] = {
    {"name", "GetPlayerName", nullptr, false},
    {"face", "ShowRaceMenu", nullptr, false},
    {"SPECIAL", "SetSPECIALPoints 40", kSpecialReset, true},
    {"tag skills", "SetTagSkills 3 1", kSkillReset, true},
    {"traits", "ShowTraitMenu", nullptr, false},
};
constexpr uint32_t kOwnPlayerRef = 0x14;   // the calling reference of this machine's own player

// The open menus by id. MenuMode(id) 0x702450 (plugin/vats/vats_addrs.h kMenuModeOf), the engine function the console's `MenuMode <id>` runs:
// true for every open menu with that id, not only the top one (the escape menu 1013 and the menu under it both answer 1).
constexpr uint32_t kMenuModeOf = 0x00702450;
constexpr uint8_t kMenuModeOfPrologue[16] = {0x55, 0x8B, 0xEC, 0x51, 0xE8, 0xB7, 0x4D, 0xDB, 0xFF, 0x85, 0xC0, 0x74, 0x74, 0xE8, 0xAE, 0x4D};
constexpr int kHudMenu = 1004, kEscapeMenu = 1013;   // always open / opened by the engine when the window loses focus: never a chargen menu
std::set<int> OpenMenus() {
    static const bool pinOk = std::memcmp(reinterpret_cast<const void*>(kMenuModeOf), kMenuModeOfPrologue, sizeof kMenuModeOfPrologue) == 0;
    std::set<int> out;
    if (!pinOk) return out;
    using Fn = bool(__cdecl*)(int);
    for (int id = 1001; id <= 1100; ++id)
        if (id != kHudMenu && reinterpret_cast<Fn>(static_cast<uintptr_t>(kMenuModeOf))(id)) out.insert(id);
    return out;
}
bool GuestChargenPending() { return g_chargen.running && !g_chargen.done; }
// This guest has made its character and Doc's scene (VCG01) is not over yet: the scene's control locks are the host's.
bool GuestFreeAfterChargen() {
    static const fnvcoop::FormRef vcg01 = [] { fnvcoop::FormRef f; nvhooks::strToRef("falloutnv.esm:104C1C", &f); return f; }();
    return g_chargen.done && nvhooks::localQuestStage(vcg01) < 200;
}

// Identity layer (plugin/hooks/identity.h), guest only: every effect that arrives from another player's machine names that player as 0x14; the layer maps it to
// that player's puppet here or drops it (the 2026-10-29 wake-up loop). It needs this machine's slot, how the session names a slot's player, and the puppets.
void ConfigureIdentity() {
    nvhooks::IdentityHooks h;
    h.self = []() -> fnvcoop::PlayerId { return g_rt && g_rt->client() ? g_rt->client()->slot() : fnvcoop::kNoPlayer; };
    h.wireRefOf = [](fnvcoop::PlayerId s, fnvcoop::FormRef* out) { return g_rt && out && g_rt->playerRefOf(s, *out); };
    h.puppetOf = [](fnvcoop::PlayerId s) -> uint32_t {   // GhostOf, but also for the host's player (GhostOf is the host's view of its guests)
#ifdef NVCOOP_WITH_LIVE
        if (uint32_t puppet = nvcoop::livewire::glueAvatarRef(static_cast<uint8_t>(s))) return puppet;
#endif
#ifdef NVCOOP_WITH_NVMP
        return s == fnvcoop::kHostPlayer ? 0u : nvmp::GhostRefOfPlayer(static_cast<uint8_t>(s));
#else
        (void)s;
        return 0u;
#endif
    };
    nvhooks::configureIdentity(std::move(h));
}
void StartGuestChargen() {
    g_chargen = GuestChargen{};
    g_chargen.running = true;
    g_chargen.nextAt = NowMs() + kChargenAfterWakeMs;
    nvcoop::Log("coop: chargen for this guest starts in %llu ms (name, face, SPECIAL, tag skills, traits)", (unsigned long long)kChargenAfterWakeMs);
}

void GuestChargenFrame() {
    if (!g_chargen.running || g_chargen.done) return;
    const uint64_t now = NowMs();
    if (now < g_chargen.nextAt) return;
    constexpr int kSteps = static_cast<int>(sizeof kChargenSteps / sizeof kChargenSteps[0]);
    const std::set<int> open = OpenMenus();
    if (g_chargen.issuedAt) {   // the step's menu is up (or about to be): wait for that menu to close
        if (!g_chargen.menu) {
            for (int id : open)
                if (id != kEscapeMenu && !g_chargen.before.count(id)) { g_chargen.menu = id; break; }
            if (g_chargen.menu) nvcoop::Log("coop: chargen %s: menu %d is open", kChargenSteps[g_chargen.step].what, g_chargen.menu);
            else if (now - g_chargen.issuedAt < kChargenOpenWaitMs) return;
        }
        if (g_chargen.menu && open.count(g_chargen.menu)) { g_chargen.closedFrames = 0; return; }
        if (g_chargen.menu && ++g_chargen.closedFrames < 20) return;   // closed for a moment, not between two of its own screens
        nvcoop::Log("coop: chargen %s %s", kChargenSteps[g_chargen.step].what, g_chargen.menu ? "done" : "never opened, skipped");
        g_chargen.issuedAt = 0;
        ++g_chargen.step;
    }
    for (int id : open)
        if (id != kEscapeMenu) {   // something else is open (a message box, Doc's talk): our next menu waits. The escape menu does not count.
            if (g_chargen.blockedBy != id) { g_chargen.blockedBy = id; nvcoop::Log("coop: chargen waits for menu %d to close", id); }
            return;
        }
    g_chargen.blockedBy = 0;
    if (g_chargen.step >= kSteps) {
        g_chargen.done = true;
        {   // the host's dazed white flashes that reached this guest before its menus started (VCG01PlayerStrainISFX, VCG01PlayerStrain3ISFX)
            nvhooks::ApplierExecScope exec;
            nvhooks::LocalPlayerScope own;
            nvhooks::execConsoleLine("RemoveImageSpaceModifier 00120E96");
            nvhooks::execConsoleLine("RemoveImageSpaceModifier 0016A8A3");
        }
        // Free to walk around now. Doc's scene (VCG01, host's game) locks the host's player until Doc lets it walk (stage 55) and again for the
        // goodbye (stage 110); those locks reach every guest, so until Doc's goodbye is over (VCG01 stage 200) they are not this guest's.
        nvhooks::setControlsLockVeto([] { return GuestFreeAfterChargen(); });
        const bool freed = nvhooks::enableAllPlayerControls();
        IntroNotice("Character ready. You can look around while the host finishes with Doc.");
        nvcoop::Log("coop: chargen finished for this guest (controls %s)", freed ? "free" : "NOT freed");
        return;
    }
    nvhooks::ApplierExecScope exec;
    nvhooks::LocalPlayerScope own;   // the guest's own character: not dropped by its game's script suppression, not captured on a host
    const ChargenStep& st = kChargenSteps[g_chargen.step];
    auto run = [&](const char* line) { return st.onPlayer ? nvhooks::execConsoleLineOn(line, kOwnPlayerRef) : nvhooks::execConsoleLine(line); };
    if (const char* const* p = st.first)
        for (; *p; ++p) run(*p);
    const bool ok = run(st.line);
    nvcoop::Log("coop: chargen %s: '%s' %s", kChargenSteps[g_chargen.step].what, kChargenSteps[g_chargen.step].line, ok ? "issued" : "FAILED");
    g_chargen.issuedAt = now;
    g_chargen.before = open;
    g_chargen.menu = 0;
    g_chargen.closedFrames = 0;
}

// A guest that joined a campaign already under way (the host Continued a save; this guest had none of its own and loaded the start save): the host says
// "noIntro", this machine skips the opening movie, makes its character (GuestChargenFrame) and then walks up to the host (JoinHostWarp). docs/party-save.md.
struct NoIntroJoin {
    bool on = false, done = false;
    uint64_t nextAt = 0, tries = 0;
};
NoIntroJoin g_noIntro;

// After the character is made: move this guest's player next to the host's. The rules' own late-join warp (policy late_join.after_scene) needs a checkpoint zone
// the host may not have visited; this one needs only the host's puppet on this machine.
void JoinHostWarp(uint64_t now) {
    g_noIntro.nextAt = now + 1000;
    if (++g_noIntro.tries > 90) { g_noIntro.done = true; nvcoop::Log("coop: join warp given up: the host's puppet never appeared"); return; }
#ifdef NVCOOP_WITH_LIVE
    fnvcoop::FormRef me, to;
    const uint32_t puppet = nvcoop::livewire::glueAvatarRef(static_cast<uint8_t>(fnvcoop::kHostPlayer));
    if (!puppet || !g_applier || !nvhooks::resolver().toRef(0x14, &me) || !nvhooks::resolver().toRef(puppet, &to)) return;
    nvhooks::ApplierExecScope exec;
    const nvhooks::ApplyStatus st = g_applier->apply(fnvcoop::MoveRef{me, to, 0, 60, 0, true});
    nvcoop::Log("coop: join warp to the host: %s", nvhooks::applyStatusName(st));
    if (st == nvhooks::ApplyStatus::Ok) {
        g_noIntro.done = true;
        IntroNotice("You joined your party.");
    }
#else
    g_noIntro.done = true;   // no avatar layer: the rules' late-join warp (or the player's own legs)
#endif
}

void IntroFrame() {
    const uint64_t now = NowMs();
    partyglue::frame(g_rt.get());   // party saves: the host's and the guest's machine tick (no-op without a session)
    if (g_noIntro.on && !g_noIntro.done && g_chargen.done && now >= g_noIntro.nextAt) JoinHostWarp(now);
    if (g_intro.check) {
        g_intro.check = false;
        g_intro.armed = AtStartSaveHold();
        g_intro.armedAt = now;
        if (g_intro.armed) nvcoop::Log("coop: start save loaded: holding the party on the black screen (party of %d)", g_cfg.party);
    }
    if (g_intro.armed && g_intro.started && !g_intro.woke && g_rt && g_rt->client() && nvhooks::localQuestStage(Vcg00()) >= 100) IntroWakeUp();
    GuestChargenFrame();
    if (!g_intro.armed || g_intro.started) return;
    const bool giveUp = now - g_intro.armedAt >= kIntroGiveUpMs;
    if (!g_rt || g_rt->host()) {
        if (g_intro.goAt) {
            if (now >= g_intro.goAt) StartIntro("host");
            return;
        }
        int here = 1;
        if (g_rt)
            for (fnvcoop::PlayerId s : g_rt->joinedSlots()) here += g_intro.ready.count(s) ? 1 : 0;
        const int party = g_rt && g_cfg.party > 1 ? g_cfg.party : 1;
        if (here != g_intro.shown) {
            g_intro.shown = here;
            if (here < party) IntroNotice("Waiting for the party: " + std::to_string(here) + " of " + std::to_string(party) + " here");
        }
        if (here < party && !giveUp) return;
        if (here < party) IntroNotice("Starting without the players who did not arrive");
        partyglue::startCampaign(g_rt.get());   // the party starts from the start save: a new campaign (a Continue keeps the loaded one)
        if (g_rt)
            for (fnvcoop::PlayerId s : g_rt->joinedSlots()) g_rt->sendOrder(s, "{\"op\":\"intro\"}");
        g_intro.goAt = now + kIntroOrderLeadMs;
        nvcoop::Log("coop: party of %d, %d here -> intro for everybody in %llu ms", party, here, (unsigned long long)kIntroOrderLeadMs);
        return;
    }
    fnvcoop::ClientSession* c = g_rt->client();
    if (c && c->joined() && !c->baselineActive() && now - g_intro.lastReady >= 2000) {   // resent until the start comes: the host counts each guest once
        if (g_intro.lastReady == 0) IntroNotice("Waiting for the party");
        g_intro.lastReady = now;
        g_rt->sendOrderToHost("{\"op\":\"introReady\"}");
    }
    if (now - g_intro.armedAt >= kIntroGiveUpMs + 60 * 1000) StartIntro("no word from the host");
}

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

// ---- campaign wave 2 (P6): the session half of the engine hook observers --------------------------------------------------------------
// The game-free logic is in party_state.h / save_sidecars.h / coop_runtime (unit tested, twins 9..19); this is the thin layer that hands the
// engine things to them. plugin/coop/tests/check_module_wiring.py fails the build when one of these calls goes missing.
bool WriteBytes(const std::string& path, const fnvcoop::Bytes& b);

// G25: positions for the anchor tethers. A guest's position is its puppet's.
bool RefPosition(uint32_t refID, float& x, float& y, float& z) {
    if (!refID) return false;
    auto* r = static_cast<TESObjectREFR*>(nvhooks::resolver().lookup(refID));
    if (!r) return false;
    x = r->posX;
    y = r->posY;
    z = r->posZ;
    return true;
}

SaveSidecars g_sidecars;
SlotRegistrar g_slotRegistrar;

// nvcoop.ini [rules]: tether=on|warn|off (owner switch), death.mode / death.revive_s / death.all_down, late_join.during_scene / late_join.after_scene.
// Empty or missing = the rules.json policy decides. A bad value is logged and ignored.
std::map<std::string, std::string> ReadRulesPolicy() {
    std::map<std::string, std::string> p;
    const std::string ini = IniPath();
    auto get = [&](const char* key) {
        char buf[64] = {};
        GetPrivateProfileStringA("rules", key, "", buf, sizeof buf, ini.c_str());
        std::string v = buf;
        for (char& c : v) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
        return v;
    };
    auto one = [&](const char* iniKey, const char* policyKey, std::initializer_list<const char*> allowed) {
        const std::string v = get(iniKey);
        if (v.empty()) return;
        for (const char* a : allowed) if (v == a) { p[policyKey] = v; return; }
        nvcoop::Log("coop: nvcoop.ini [rules] %s=%s is not valid, ignored", iniKey, v.c_str());
    };
    one("tether", "tether.mode", {"on", "warn", "off"});
    one("death.mode", "death.mode", {"downed", "respawn", "normal"});
    one("death.all_down", "death.all_down", {"reload-host-save", "respawn"});
    one("late_join.during_scene", "late_join.during_scene", {"wait", "join"});
    one("late_join.after_scene", "late_join.after_scene", {"warp-to-zone", "stay"});
    const std::string rev = get("death.revive_s");
    if (!rev.empty() && std::atoi(rev.c_str()) > 0) p["death.revive_s"] = rev;
    return p;
}

bool g_downed = false;   // G41: the local player is down (controls held, waiting for the respawn order)

SaveSidecars::Io SidecarIo() {
    SaveSidecars::Io io;
    io.write = [](const std::string& path, const std::string& text) { return WriteBytes(path, fnvcoop::Bytes(text.begin(), text.end())); };
    io.read = [](const std::string& path, std::string* text) { *text = ReadWholeFile(path); return !text->empty(); };
    io.log = [](const std::string& line) { nvcoop::Log("coop: %s", line.c_str()); };
    return io;
}

// P2 PlayerFactsScope (handover R1): while a guest's INFO result script runs, player.GetItemCount / GetAV / HasPerk / GetEquipped answer for the guest.
std::shared_ptr<void> OpenFactsScope(const fnvcoop_dlg::ResultRequest& rq) {
    const bool guest = rq.initiator != fnvcoop::kHostPlayer;
    const bool haveFacts = rq.facts && !rq.facts->empty();
    if (!haveFacts && !guest) return nullptr;
    struct Hold {
        std::vector<nvhooks::PlayerFact> facts;
        std::unique_ptr<nvhooks::GuestItemScope> items;      // guest pack facts: a hand-in is not taken from the host's pack
        std::unique_ptr<nvhooks::PlayerFactsScope> scope;   // declared last: closed before the vector it points to
    };
    auto h = std::make_shared<Hold>();
    if (haveFacts) {
        for (const auto& f : *rq.facts) h->facts.push_back({f.kind, f.what, f.value});
        h->scope.reset(new nvhooks::PlayerFactsScope(&h->facts));
    }
    if (guest) h->items.reset(new nvhooks::GuestItemScope(static_cast<uint8_t>(rq.initiator)));
    return h;
}

// Guest: an order the host sent for THIS machine's player (kAppOrderChannel).
std::string ApplyOrder(const std::string& json) {
    nvcoop::json::Value v;
    std::string err;
    if (!nvcoop::json::parse(json, v, err)) return "BadOrder: " + err;
    const std::string op = v.get("op").asString("");
    nvhooks::ApplierExecScope exec;   // not a leaked result script
    if (op == "kill") {   // G42: one player dies, not the party
        const bool ok = nvhooks::execConsoleLineOn(nvhooks::killCmd(), 0x14);
        nvcoop::Log("coop: order kill (my player) -> %s", ok ? "issued" : "FAILED");
        return ok ? "Ok" : "EngineRejected";
    }
    if (op == "downed") {   // G41: this player is down until the party's checkpoint (or until a friend gets him up, docs/revive.md); no death screen, no quest failure
        g_downed = true;
        nvhooks::SetPlayerControlsLocked(true);
        nvcoop::intents::game::engine().notify("You are down. You get up at your party's last checkpoint.");
        nvcoop::Log("coop: my player is downed (respawn in %lld ms)", (long long)v.get("respawnMs").asInt(0));
#ifdef NVCOOP_WITH_MARKERS
        nvcoop::markers::onLocalDowned(static_cast<uint32_t>(v.get("respawnMs").asInt(0)));   // the revive timer and the downed pose
#endif
        return "Ok";
    }
    if (op == "respawn") {   // G41: up again, at the checkpoint; or in place with part of his health when a friend revived him ("inPlace")
        g_downed = false;
        nvhooks::SetPlayerControlsLocked(false);
        bool ok = nvhooks::execConsoleLineOn(nvhooks::resurrectCmd(false), 0x14);
        ok = nvhooks::execConsoleLineOn("RestoreAV Health 100000", 0x14) && ok;
        float reviveFrac = 1.0f;
        std::string revivedBy;
        bool inPlace = false;
#ifdef NVCOOP_WITH_MARKERS
        inPlace = nvmk::readInPlaceRespawn(v, reviveFrac, revivedBy);
#endif
        fnvcoop::FormRef cp;
        if (inPlace) {   // where he lies, with `reviveFrac` of his health: no warp to the checkpoint
            if (reviveFrac < 0.999f) nvhooks::execConsoleLineOn(nvmk::inPlaceDamageLine(reviveFrac), 0x14);   // a failed line only leaves him at full health
        } else if (nvhooks::strToRef(v.get("checkpoint").asString(""), &cp) && g_applier) {
            fnvcoop::FormRef me;
            if (nvhooks::resolver().toRef(0x14, &me)) g_applier->applyLocal(fnvcoop::MoveRef{me, cp, 0, 0, 0, true});   // my own player: not an effect from another machine
        }
#ifdef NVCOOP_WITH_MARKERS
        nvcoop::markers::onLocalUp(revivedBy);
#endif
        nvcoop::Log("coop: my player respawns %s -> %s", inPlace ? "in place (revived)" : "at the checkpoint", ok ? "issued" : "FAILED");
        return ok ? "Ok" : "EngineRejected";
    }
    if (op == "stash") {   // G35: my own gear goes into / comes out of MY copy of the shared storage (W3d item 4: a typed strip too)
        const StashOrder so = stashOrderFrom(v);
        fnvcoop::FormRef c;
        uint32_t cid = 0;
        const bool destroy = so.container.empty() && !so.into;   // `player.RemoveAllItems` / `RemoveAllTypedItems` with no container (stash_order.h)
        if (!destroy) {
            if (!nvhooks::strToRef(so.container, &c)) return "BadContainer";
            cid = nvhooks::resolver().fromRef(c);
            if (!cid) return "FormMissing";
        }
        uint32_t holdoutId = 0;
        fnvcoop::FormRef ho;
        if (!so.holdout.empty() && nvhooks::strToRef(so.holdout, &ho)) holdoutId = nvhooks::resolver().fromRef(ho);
        const StashLine sl = stashLine(so, cid, holdoutId);
        const bool ok = nvhooks::execConsoleLineOn(sl.text, sl.onRef);
        nvcoop::Log("coop: order stash '%s' -> %s", sl.text.c_str(), ok ? "issued" : "FAILED");
        return ok ? "Ok" : "EngineRejected";
    }
    if (op == "intro") {   // the whole party is in: the opening movie for everybody at once
        if (!g_intro.armed) return "NotHolding";
        StartIntro("host's order");
        return "Ok";
    }
    if (op == "noIntro") {   // the host is mid-campaign: no opening movie is coming. Skip it; the host's world takes this machine past VCG00 stage 100 and the wake-up and character creation follow
        if (!g_intro.armed || g_intro.started) return "NotHolding";
        g_intro.started = true;
        g_noIntro = NoIntroJoin{};
        g_noIntro.on = true;
        IntroNotice("The party is already underway. You make your character, then join the host.");
        nvcoop::Log("coop: host says the campaign is under way: no opening movie, character creation, then the join warp");
        return "Ok";
    }
    if (partyglue::isGuestOp(op)) return partyglue::applyOrder(g_rt.get(), v);   // party saves: campaign, partySave, partySaveWait
#ifdef NVCOOP_WITH_MARKERS
    if (nvcoop::markers::isGuestOp(op)) return nvcoop::markers::applyGuestOrder(json);   // the host's party table: friend markers and the revive state
#endif
    return "UnknownOrder";
}

// Host: a request from a guest (G10: a note the guest picked up by itself; the host's journal and GetHasNote must see it).
std::string HandleOrder(fnvcoop::PlayerId from, const std::string& json) {
    nvcoop::json::Value v;
    std::string err;
    if (!nvcoop::json::parse(json, v, err)) return "BadOrder: " + err;
    if (partyglue::isHostOp(v.get("op").asString(""))) return partyglue::handleOrder(g_rt.get(), from, v);   // party saves: partySaveRequest, partySaved
#ifdef NVCOOP_WITH_MARKERS
    if (nvcoop::markers::isHostOp(v.get("op").asString(""))) return nvcoop::markers::handleHostOrder(static_cast<uint8_t>(from), json);   // revive (and the friend markers' beacons)
#endif
    if (v.get("op").asString("") == "introReady") {   // a guest loaded the start save and joined: one more player on the black screen
        // This host is not at the start save: it Continued a campaign (or the intro is long over). Nothing to wait for, no movie to play: the guest skips it.
        const bool underway = g_gameLoaded && !g_intro.check && (!g_intro.armed || (g_intro.started && nvhooks::localQuestStage(Vcg00()) >= 100));
        if (underway && g_rt) {
            g_rt->sendOrder(from, "{\"op\":\"noIntro\"}");
            nvcoop::Log("coop: guest slot %d asks for the intro, the campaign is under way: no intro", (int)from);
            return "Ok";
        }
        if (g_intro.ready.insert(from).second) nvcoop::Log("coop: guest slot %d is in for the intro", (int)from);
        if (g_intro.started && g_rt) g_rt->sendOrder(from, "{\"op\":\"intro\"}");   // late: the others are already watching
        return "Ok";
    }
    if (v.get("op").asString("") != "note") return "UnknownOrder";
    fnvcoop::FormRef note;
    if (!nvhooks::strToRef(v.get("note").asString(""), &note)) return "BadNote";
    const uint32_t id = nvhooks::resolver().fromRef(note);
    nvhooks::CaptureHub* hub = nvhooks::hub();
    if (!id || !hub || !g_rt) return "FormMissing";
    fnvcoop::Cause c;
    c.kind = fnvcoop::CauseKind::Intent;
    c.source = note;
    c.aux = static_cast<int32_t>(fnvcoop::IntentKind::PickUp);
    bool ok = false;
    g_rt->runAsGuest(from, [&] {
        nvhooks::ScopedCapture scope(*hub, c);   // the AddNote hook records one AddNote effect (shared journal) inside it
        ok = nvhooks::execConsoleLine(nvhooks::addNoteCmd(id));
    });
    nvcoop::Log("coop: guest slot %d picked up a note (%s) -> host journal %s", (int)from, nvhooks::refToStr(note).c_str(), ok ? "updated" : "NOT updated");
    return ok ? "Ok" : "EngineRejected";
}

// Observers and providers of plugin/hooks that need the session. Host: party conditions (G23), kill / stash of a guest's player (G42, G35).
// Guest: a note it picked up by itself goes to the host (G10).
void WireHookObservers() {
    nvhooks::setScriptRunObserver(nullptr);
#ifdef NVCOOP_WITH_WORLD
    nvhooks::setScriptRunObserver([](uint32_t refID) { nvworld::noteRefScriptRan(refID); });   // G15: a ref var flipped inside a script this frame is seen
#endif
    if (!g_rt) return;
    if (g_cfg.role == Role::Host) {
        // wave 1b: the kill itself is recorded as KillActor{that player's ref} (setPlayerRefProvider below) and the rules engine turns it into a Health drain for that slot; the
        // observer is only told, it must not kill the guest a second time.
        nvhooks::setScriptRunScope([](uint32_t script, bool before) { if (g_rt) g_rt->onScriptRun(script, before); });   // W3a item 4: the acting guest's view of the companion flags around each script
        nvhooks::setControlsRouter([](uint8_t slot, bool disable) { return g_rt && g_rt->controlsGuestOnly(slot, disable); });   // W3a item 5: a guest-only lock does not lock the host
        nvhooks::setPlayerKillObserver([](uint8_t slot, bool onHost) {
            nvcoop::Log("coop: script kill of slot %d's player (engine kill on this machine: %s)", (int)slot, onHost ? "yes" : "no");
        });
        nvhooks::setPlayerRefProvider([](uint8_t slot, fnvcoop::FormRef* out) { return g_rt && out && g_rt->playerRefOf(slot, *out); });   // G42: the same mapping as RulesEngine::Options::playerRef
        nvcoop::intents::setGuestStagePassHook([](fnvcoop::PlayerId s, const fnvcoop::FormRef& q, uint32_t st) {   // G26: a guest's SetStage on a barrier stage is its pass
            fnvcoop::RulesEngine* r = g_rt ? g_rt->rules() : nullptr;
            return r ? nvcoop::intents::stagePassOf(*r, s, q, st) : nvcoop::intents::StagePass::NotPass;
        });
        nvhooks::setPlayerStashObserver([](const nvhooks::PlayerStashEvent& e) {
            if (!g_rt || e.slot == 0) return;   // the host's own gear moved in the engine already
            StashOrder so;
            so.into = e.intoPlayer;
            so.container = e.container.plugin.empty() ? std::string() : nvhooks::refToStr(e.container);   // no container: the guest's items are destroyed
            so.retain = e.retainOwnership;
            so.type = e.typeFilter;
            so.noMsg = e.noMessage;
            if (!e.holdout.plugin.empty()) so.holdout = nvhooks::refToStr(e.holdout);
            const bool sent = g_rt->sendOrder(e.slot, stashOrderJson(so));
            nvcoop::Log("coop: gear move for slot %d (%s %s) -> %s", (int)e.slot, e.intoPlayer ? "from" : "into", so.container.empty() ? "nowhere" : so.container.c_str(),
                        sent ? "order sent to that guest" : "NOT sent (guest not joined)");
        });
        if (g_rt->rules()) {   // the evaluators loop over the party only when a rules engine reduces the values
            nvhooks::setPartyConditions([]() { return g_rt ? presentMembers(*g_rt, [](fnvcoop::PlayerId s) { return GhostOf(s); }) : std::vector<nvhooks::PartyMember>(); },
                                        makeSessionReducer([]() { return g_rt ? g_rt->rules() : nullptr; }));
            PartyStateSource pss = sourceFromRuntime(*g_rt, [](uint32_t id, fnvcoop::FormRef* out) { return nvhooks::resolver().toRef(id, out); });
            // W3a item 3: GetEquipped, IsSneaking, IsPlayerInRegion, GetLevel, GetIsSex / GetPCIsSex, GetSitting and IsPlayerActionActive of a guest come from the avatar layer
            // (its Look, its snapshot and its Facts message); a guest that has not delivered a group yet stays unknown for it.
            bindSlotFacts(pss, [](uint8_t slot, nvcoop::SlotFacts* out) { return nvcoop::livewire::glueSlotFacts(slot, out); });
            // Guest pack facts: GetItemCount of a watched item form and InventoryWeight of a guest come from its Pack message; a form nobody reported stays the puppet's
            bindSlotPack(pss, [](uint8_t slot, uint32_t item, int64_t* n) { return nvcoop::livewire::gluePackCount(slot, item, n); },
                         [](uint8_t slot, double* w) { return nvcoop::livewire::gluePackWeight(slot, w); });
            nvhooks::setPartyStateProvider(makePartyStateProvider(std::move(pss)));
            // W3d item 1 (W3b requests 1 and 3): what a slot holds (pooled hand-ins: Honest Hearts totems, roots, lunchboxes) is the engine's GetItemCount on that player's own
            // reference, the same reading the party conditions take; the scripts the rules run once per slot run again for every guest with a puppet.
            g_rt->setItemCountProbe([](fnvcoop::PlayerId slot, const fnvcoop::FormRef& item, int64_t& count) {
                // a guest's pack is what it reported (Pack message): the puppet carries none of it; an item nobody reported falls through to the puppet as before
                if (slot != fnvcoop::kHostPlayer) {
                    const uint32_t id = nvhooks::resolver().fromRef(item);
                    if (id && nvcoop::livewire::gluePackCount(static_cast<uint8_t>(slot), id, &count)) return true;
                }
                const uint32_t who = slot == fnvcoop::kHostPlayer ? nvhooks::kHostPlayerRef : GhostOf(slot);
                return who != 0 && nvhooks::playerItemCount(who, item, &count);
            });
            {
                std::vector<nvhooks::PerSlotScript> table;
                for (const Runtime::PerSlotScriptInfo& info : g_rt->perSlotScripts())
                    if (const uint32_t id = nvhooks::resolver().fromRef(info.script)) table.push_back({id, info.vars});
                nvcoop::Log("coop: %u scripts run once per player slot (%u declared by the rules)", (unsigned)table.size(), (unsigned)g_rt->rules()->perPlayerScripts().size());
                nvhooks::setPerSlotScripts(std::move(table), [](uint32_t script, const nvhooks::PerSlotEach& each) {
                    if (!g_rt) return;
                    g_rt->runScriptPerSlot(script, [&each](fnvcoop::PlayerId slot) {
                        if (const uint32_t ghost = GhostOf(slot)) each(static_cast<uint8_t>(slot), ghost);   // a guest whose puppet does not exist yet has no run
                    });
                });
            }
            // W3d item 2 (W3b request 6): the conditions of an INFO are reduced under that INFO's quest (the Jed weight gate is the heaviest pack of the party).
            nvhooks::setInfoQuestScopes([](uint32_t infoId, fnvcoop::FormRef* quest) {
                fnvcoop::FormRef info;
                if (!nvhooks::resolver().toRef(infoId, &info)) return false;
                auto it = g_infoQuest.find(nvhooks::refToStr(info));
                return it != g_infoQuest.end() && quest && nvhooks::strToRef(it->second, quest);
            });
        }
    } else if (g_cfg.role == Role::Client) {
        nvhooks::setNoteObserver([](const fnvcoop::FormRef& note) {
            if (!g_rt) return;
            nvcoop::json::Value o = nvcoop::json::Value::object();
            o.set("op", "note").set("note", nvhooks::refToStr(note));
            g_rt->sendOrderToHost(o.dump());
        });
    }
}

void UnwireHookObservers() {
    nvhooks::setScriptRunObserver(nullptr);
    nvhooks::setScriptRunScope(nullptr);
    nvhooks::setControlsRouter(nullptr);
    nvhooks::setPlayerKillObserver(nullptr);
    nvhooks::setPlayerStashObserver(nullptr);
    nvhooks::setNoteObserver(nullptr);
    nvhooks::setPartyConditions(nullptr, nullptr);
    nvhooks::setPartyStateProvider(nullptr);
    nvhooks::setPerSlotScripts({}, nullptr);
    nvhooks::setInfoQuestScopes(nullptr);
    nvhooks::setPlayerRefProvider(nullptr);
    nvcoop::intents::setGuestStagePassHook(nullptr);
}

void TryStart() {
    if (g_rt || g_failed || !g_cfg.enable || g_cfg.role == Role::Off) return;
    if (g_cfg.baseline && !g_gameLoaded) return;   // the dump needs a loaded save
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
        running = (fl & 0x01) != 0;   // the same bit the world dump reads (baseline_nvse.cpp: q->flags & 0x01)
        return true;
    });
    g_rt->setGlobalReader([](const fnvcoop::FormRef& g, double& value) { return nvhooks::localGlobalValue(g, value); });
    g_rt->setObjectiveStateReader([](const fnvcoop::FormRef& q, std::map<uint32_t, uint8_t>& objectives) {
        return nvhooks::localQuestObjectives(q, objectives);
    });
    g_rt->setScriptVarReader([](const fnvcoop::FormRef& q, std::map<uint16_t, double>& vars) {
        for (auto& kv : nvhooks::questVarsDump(q))   // the ScriptLocals the host poll reads (empty: not a quest, or no script event list yet)
            if (kv.first <= 0xFFFFu) vars[static_cast<uint16_t>(kv.first)] = kv.second;
        return !vars.empty();
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
#ifdef NVCOOP_WITH_SNAPSHOT
            {   // The volatile list is otherwise only loaded by a world dump, which this host never runs: load it (and resolve the var names) here.
                static int tries = 0;
                static uint32_t calls = 0;
                if (first || (tries > 0 && tries < 40 && (++calls % 300) == 0 && fnvcoop::volatileKeys().unresolvedNames() > 0)) {
                    std::string sum;
                    const bool done = fnvcoop::snapshot::PrepareVolatileKeys(fnvcoop::snapshot::BaselineDumpOptions(), &sum);
                    ++tries;
                    if (first || done) nvcoop::Log("coop: host %s%s", sum.c_str(), done ? "" : " (will retry)");
                    if (done) tries = 40;
                }
            }
#endif
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
            {   // G02: script / INFO / ref -> the quest whose rules apply (docs/quest-policies/cause_quests.json, generated)
                const std::string causes = ReadWholeFile(DllDir() + "nvcoop_cause_quests.json");
                if (causes.empty()) nvcoop::Log("coop: nvcoop_cause_quests.json not found: a quest's own scripts will not match its rules");
                g_rt->setCauseQuests(causes);
            }
            g_rt->setPolicy(ReadRulesPolicy());   // nvcoop.ini [rules]: tether=on|warn|off, death.mode ...
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
#ifdef NVCOOP_WITH_COMPANIONS
    nvcoop::social::glueSetRosterListener([](const std::vector<fnvcoop::social::Companion>& all) {   // P7: owners and party flags into the dialogue registry
        if (!g_rt) return;
        std::vector<Runtime::RosterEntry> r;
        for (const auto& c : all) r.push_back({c.npc, c.owner, c.party});
        g_rt->onRosterChanged(r);
    });
#endif
#ifdef NVCOOP_WITH_COMPANIONS
    {   // W3a item 4: a script that runs for a guest reads that guest's hired flags (and Dead Money rCurrentFollower); the view-party movers read the aggregate.
        g_scriptNames.clear();
        const std::vector<std::string> names = nvcoop::social::glueViewPartyScripts();
        const std::set<std::string> want(names.begin(), names.end());
        for (const auto& kv : g_edids) {
            std::string low = kv.first;
            for (char& ch : low) ch = static_cast<char>(std::tolower(static_cast<unsigned char>(ch)));
            fnvcoop::FormRef r;
            if (want.count(low) && nvhooks::strToRef(kv.second, &r))
                if (uint32_t id = nvhooks::resolver().fromRef(r)) g_scriptNames[id] = low;
        }
        g_rt->setViewPolicy(
            [](uint32_t script, uint8_t acting) -> uint8_t {
                auto it = g_scriptNames.find(script);
                return nvcoop::social::glueViewSlot(it == g_scriptNames.end() ? std::string() : it->second, acting);
            },
            [](fnvcoop::PlayerId slot, Runtime::FollowerVar* out) {
                auto q = g_edids.find("NVDLC01Followers");
                fnvcoop::FormRef qr, cr;
                if (q == g_edids.end() || !nvhooks::strToRef(q->second, &qr) || !out) return false;
                out->quest = qr;
                out->var = "rCurrentFollower";   // a REF variable: setQuestVarValue stores the form id in the slot's low dword (hooks/ref_var.h)
                out->formId = nvcoop::social::glueCurrentFollower(static_cast<uint8_t>(slot), "deadmoney", cr) ? static_cast<double>(nvhooks::resolver().fromRef(cr)) : 0.0;
                return true;
            });
        nvcoop::Log("coop: companion views: %u view-party scripts named of %u", (unsigned)g_scriptNames.size(), (unsigned)names.size());
    }
#endif
    g_rt->setNotice([](const std::string& s) { nvcoop::intents::game::engine().notify(s); });
    g_rt->setHostEffectApplier([](const fnvcoop::Effect& e) {   // G18 / G25 / G41: what the rules engine commits by itself, for the host's own player
        if (g_applier) g_applier->applyLocal(e);
    });
    g_rt->setPlayerVarIO({[](const fnvcoop::FormRef& q, const std::vector<std::string>& names, std::vector<fnvcoop::RulesEngine::VarValue>& out) {   // G24
                              for (const std::string& n : names) { double v = 0; if (nvhooks::questVarValue(q, n, &v)) out.push_back({n, v}); }
                              return !out.empty();
                          },
                          [](const fnvcoop::FormRef& q, const std::vector<fnvcoop::RulesEngine::VarValue>& values) {
                              for (const auto& v : values) nvhooks::setQuestVarValue(q, v.var, v.value);
                          },
                          [](const fnvcoop::FormRef& q, uint32_t idx, std::string* name) { return nvhooks::questVarName(q, idx, name); }});
    {
        Runtime::AnchorProbe probe;                              // G25
        probe.anchorPos = [](const fnvcoop::FormRef& a, float& x, float& y, float& z) { return RefPosition(nvhooks::resolver().fromRef(a), x, y, z); };
        probe.slotPos = [](fnvcoop::PlayerId s, float& x, float& y, float& z) {
            if (s == fnvcoop::kHostPlayer) return RefPosition(0x14, x, y, z);
            return RefPosition(GhostOf(s), x, y, z);
        };
        g_rt->setAnchorProbe(probe);
    }
    g_rt->setReloadHostSave([]() {   // G41: everybody is down and policy death.all_down is reload-host-save
        nvcoop::Log("coop: every player is down: the host save should be reloaded");
        nvcoop::intents::game::engine().notify("Everyone is down. Load your last save to go on.");
        nvcoop::PushEvent("coop.reloadHostSave", "{}");
    });
    g_rt->setActionExecutor([](fnvcoop::PlayerId slot, fnvcoop::IntentKind kind, const fnvcoop::FormRef& ref) {
        // Guest interaction (docs/protocol.md section 12): the host runs the REAL activation of `ref` inside a capture scope whose
        // origin the runtime forces to `slot`, so rewards / karma / XP route to the guest and stage changes reach everybody.
        nvhooks::CaptureHub* hub = nvhooks::hub();
        uint32_t target = nvhooks::resolver().fromRef(ref);
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
        fnvcoop::Cause c;
        c.kind = fnvcoop::CauseKind::Intent;
        c.source = ref;
        c.aux = static_cast<int32_t>(kind);
        nvhooks::ScopedCapture scope(*hub, c);
        char line[64];
        std::snprintf(line, sizeof line, "Activate %08X 1", actor);
        bool ok = nvhooks::execConsoleLineOn(line, target);
        nvcoop::Log("coop: guest action %d on %s from slot %d -> '%s' as %08X: %s", (int)kind, nvhooks::refToStr(ref).c_str(), (int)slot, line, actor, ok ? "issued" : "FAILED");
        nvcoop::PushEvent("coop.guestAction", std::string("{\"slot\":") + std::to_string((int)slot) + ",\"ref\":\"" + nvhooks::refToStr(ref) + "\",\"issued\":" + (ok ? "true" : "false") + "}");
#ifdef NVCOOP_WITH_COMPANIONS
        if (ok) nvcoop::social::glueGuestInteraction(static_cast<uint8_t>(slot), kind == fnvcoop::IntentKind::PickUp, ref);   // theft / trespass / break-in by the guest
#endif
    });
    if (g_cfg.role == Role::Client && g_cfg.forwardActivate) {
        nvhooks::setActivateObserver([](const fnvcoop::FormRef& target) {
            if (!g_rt || g_rt->syncing()) return nvhooks::ActivateAction::Proceed;   // (gameplay is locked while syncing anyway)
            return g_rt->submitAction(fnvcoop::IntentKind::ActivateRef, target) ? nvhooks::ActivateAction::Suppress
                                                                              : nvhooks::ActivateAction::Proceed;
        });
    }
#ifdef NVCOOP_WITH_UI
    {   // G05 (P3 handover, call sites 1 and 2): the guest half of a conversation. init also wires the answer sink of boxes a host script raised.
        fndlg::GuestDialogueConfig gc;
        gc.dataDir = DllDir();
        gc.report = [](const char* s) { nvcoop::Log("dialogue: %s", s); };
        fndlg::GuestDialogue::instance().init(gc);
    }
#endif
    if (g_cfg.dialogue) {
        DialogueHooks h;
        // The host's INFO result runner is plugin/intents' IntentService::runDialogueResult ([intents] dialogue_results=1 replaces this one at session begin).
        h.run = [](const fnvcoop_dlg::ResultRequest&) -> uint64_t { return 0; };
        h.sink = [](const fnvcoop_dlg::Event& e) {
#ifdef NVCOOP_WITH_UI
            fndlg::GuestDialogue::instance().sink()(e);   // OpenMenu / ShowLines / CloseMenu ... reach the engine glue
#endif
            nvcoop::PushEvent("dlg.event", std::string("{\"kind\":") + std::to_string((int)e.kind) + "}");
        };
        {   // G33: follower caps per recruiting player and for the whole party (decision D2: 1 humanoid + 1 critter each, 4 in all)
            const std::string ini = IniPath();
            fnvcoop_dlg::CompanionRules r;
            r.humanoidCap = static_cast<uint8_t>(GetPrivateProfileIntA("dialogue", "humanoid_cap", r.humanoidCap, ini.c_str()));
            r.critterCap = static_cast<uint8_t>(GetPrivateProfileIntA("dialogue", "critter_cap", r.critterCap, ini.c_str()));
            r.partyTotal = static_cast<uint8_t>(GetPrivateProfileIntA("dialogue", "party_total", r.partyTotal, ini.c_str()));
            h.companionRules = r;
        }
        g_rt->enableDialogue(h);
    }
    g_rt->setResultScope(OpenFactsScope);   // P2 PlayerFactsScope around every host-run dialogue result script
    g_rt->setOrderApplier(ApplyOrder);      // guest: kill / gear orders for this machine's player
    g_rt->setOrderHandler(HandleOrder);     // host: a guest's note
#ifdef NVCOOP_WITH_UI
    {   // L11 wiring (internal lane notes): router bridge, 0xE5 slideshow channel, SlideshowEnd, INFO reader
        UiWiring w;
        w.router = []() { return nvui::router(); };
        w.slideHost = [](nvui::SlideHost::Send s) -> nvui::SlideHost& { return nvui::slideHost(std::move(s)); };
        w.slideClient = []() -> nvui::SlideClient& { return nvui::slideClient(); };
        w.consentFile = DllDir() + "nvcoop_consent_messages.txt";
        w.installInfoReader = [](std::function<std::vector<fnvcoop_dlg::Line>(uint32_t)> f) { nvui::setInfoReader(std::move(f)); };
        w.varName = [](const fnvcoop::FormRef& q, uint32_t idx, std::string* name) { return nvhooks::questVarName(q, idx, name); };   // wave 1b: per-player condition views
        w.infoReader = fndlg::GuestDialogue::instance().infoReader();   // P3 call site 3: the engine's own response loader behind a cache check
        g_rt->enableUi(std::move(w));
    }
#endif
    g_rt->bindPlayerRefs([](fnvcoop::PlayerId s) { return GhostOf(s); },
                         [](uint32_t id, fnvcoop::FormRef& out) { return nvhooks::resolver().toRef(id, &out); });   // PartyWarp needs this
    if (g_cfg.role == Role::Client) ConfigureIdentity();
#ifdef NVCOOP_WITH_WORLD
    if (g_cfg.role == Role::Client)
        g_rt->setWorldEffectApplier([](const std::string& json) -> std::string {
            nvworld::WorldEffect we;
            std::string werr;
            if (!g_worldApplier) return "NoWorldApplier";
            if (!nvworld::worldEffectFromJson(json, &we, &werr)) return "BadWorldEffect: " + werr;
            // While this guest makes its character, the host is in Doc's scene: its screen effects there (the dazed white flashes of VCG01)
            // are the host's own and would cover this guest's menus. Its sounds and music still play.
            if (GuestChargenPending()) {
                if (const auto* p = std::get_if<nvworld::Present>(&we);
                    p && (p->kind == nvworld::PresentKind::ApplyImageSpace || p->kind == nvworld::PresentKind::RemoveImageSpace))
                    return "Ok";
            }
            nvhooks::ApplyStatus st = g_worldApplier->applyWorld(we);
            return nvhooks::applyStatusName(st);
        });
#endif
    std::string err;
    if (!g_rt->start(&err)) {
        g_failed = true;
        g_failMsg = err;
        nvcoop::Log("coop: start failed: %s", err.c_str());
        return;
    }
    WireHookObservers();
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

bool MStatus(const std::string&, std::string& out) {
    if (g_rt) {
        out = g_rt->statusJson();
        {   // W3d: what the in-game proofs of the per-slot scripts and the INFO scope read (hooks counters)
            const nvhooks::PerSlotStats& ps = nvhooks::perSlotStats();
            out = out.substr(0, out.size() - 1) + ",\"w3d\":{\"perSlotHookRuns\":" + std::to_string((unsigned long long)ps.runs) + ",\"objVarSwaps\":" +
                  std::to_string((unsigned long long)ps.objVarSwaps) + ",\"infoScopesOpened\":" + std::to_string((unsigned long long)nvhooks::infoScopesOpened()) + "}}";
        }
#ifdef NVCOOP_WITH_UI
        out = out.substr(0, out.size() - 1) + ",\"dialogue\":" + fndlg::GuestDialogue::instance().statusJson() + "}";   // P3 call site 8
#endif
        return true;
    }
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

// coop.ready {info}: the local player confirms a ready check ("is the party ready for this?") that a story line raised (G27). Without a prompt UI this is the
// button: the host's own confirmation, or a guest's ScenePass for the INFO.
bool MReady(const std::string& params, std::string& out) {
    if (!g_rt) { out = "coop not running"; return false; }
    nvcoop::json::Value p;
    std::string err;
    fnvcoop::FormRef info;
    if (!nvcoop::json::parse(params, p, err) || !ParseRef(p, "info", &info, err)) { out = err; return false; }
    g_rt->scenePass(info);
    out = "{}";
    return true;
}

bool MInject(const std::string& params, std::string& out) {
    if (!g_rt) { out = "coop not running"; return false; }
    std::string res, err;
    if (!g_rt->inject(params, &res, &err)) { out = err; return false; }
    out = res;
    return true;
}

void OnLoad(void*) {
    LoadConfig();
    g_sidecars.add({".fnvobjvars", []() { return nvcoop::coop::objVarSnapshot(nvhooks::objVarCopies()); }, [](const std::string& text) { nvcoop::coop::objVarRestore(nvhooks::objVarCopies(), text); }});   // follow-ups 1 item 3: the per-slot object variable copies
    g_sidecars.add({".fnvintents", []() { return nvcoop::intents::LedgerSnapshot(); }, [](const std::string& text) { nvcoop::intents::LedgerRestore(text); }});
#ifdef NVCOOP_WITH_LIVE
    nvcoop::livewire::glueLoad();   // main.cpp already ran glueConfigure() (before any module loaded)
    LoadPackWatch();
    nvcoop::livewire::glueSetPackWatch([]() {   // runtime form ids, resolved when the avatar service asks (session start, after a load): the game data is up by then
        std::vector<uint32_t> ids;
        for (const auto& r : g_packWatchRefs)
            if (const uint32_t id = nvhooks::resolver().fromRef(r)) ids.push_back(id);
        return ids;
    });
#endif
    nvcoop::RegisterMethod("coop.status", MStatus, true);
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
    nvcoop::RegisterMethod("coop.ready", MReady, true);
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

void OnFrame() {
    if (!g_rt) TryStart();
    if (g_rt && g_rt->running() && !g_reloadSidecar.empty()) {
        std::string what;
        bool ok = g_rt->onGameLoaded(&g_reloadSidecar, &what);
        nvcoop::Log("coop: hot reload resumed the session (%s): %s", what.c_str(), ok ? "ok" : "not handled");
        g_reloadSidecar.clear();
    }
#ifdef NVCOOP_WITH_UI
    if (!g_rt) fndlg::GuestDialogue::instance().frame(nullptr, static_cast<uint32_t>(NowMs()));   // the node died with the runtime: drop the glue's pointer
#endif
    if (g_rt) {
#ifdef NVCOOP_WITH_WORLD
        if (g_rt->host()) {   // NPC state captured by L12's hooks since the last frame -> every guest (and late joiners)
            for (const auto& we : nvworld::drainWorldEffects()) g_rt->pushWorldEffect(nvworld::worldEffectKey(we), nvworld::worldEffectToJson(we), nvworld::isEphemeral(we));   // P5: key per subject, a fade or a sound is one-shot
        } else if (g_worldApplier) {
            g_worldApplier->flushQueued();   // spawns that waited for their cell to load
        }
#endif
        g_rt->frame();
#ifdef NVCOOP_WITH_UI
        fndlg::GuestDialogue::instance().frame(g_rt->dialogue(), static_cast<uint32_t>(NowMs()));   // P3 call site 4: after the runtime ticked
#endif
        if (g_rt->host()) g_slotRegistrar.sync(*g_rt, [](fnvcoop::PlayerId s) { return GhostOf(s); }, [](uint32_t ref, uint8_t slot) { nvhooks::registerPlayerSlot(ref, slot); });
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
        GuestBarrierPasses();
        IntroFrame();
        bool lock = g_rt->syncing() || g_downed;
        {   // report the player's cell every 30 frames (party tether / presence): host -> rules engine, client -> ReportCell action
            static uint64_t frames = 0;
            static uint32_t lastCell = 0, lastWorld = 0;
            if (++frames % 30 == 0) {
                PlayerCharacter* pc = g_thePlayer ? *g_thePlayer : nullptr;
                uint32_t cell = pc && pc->parentCell ? pc->parentCell->refID : 0;
                const uint32_t world = pc && pc->parentCell && pc->parentCell->worldSpace ? pc->parentCell->worldSpace->refID : 0;   // 0 = an interior
                if (cell && (cell != lastCell || world != lastWorld)) {
                    fnvcoop::FormRef ref, wref;
                    if (nvhooks::resolver().toRef(cell, &ref)) {
                        if (world) nvhooks::resolver().toRef(world, &wref);   // G25: an exterior tether names the worldspace, not the grid cell
                        lastCell = cell;
                        lastWorld = world;
                        g_rt->reportCell(ref, wref);
                    }
                }
                // G41: the local player died / got up. The host's rules decide what a death means (downed, reload, normal).
                static bool wasDead = false;
                const bool dead = pc && (pc->lifeState == 1 || pc->lifeState == 2) && !g_downed;
                if (dead && !wasDead) g_rt->localPlayerDied();
                else if (!dead && wasDead) g_rt->localPlayerRevived();
                wasDead = dead;
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
    if (!g_rt) IntroFrame();   // the start save loaded with co-op off: nobody to wait for, the movie plays at once
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
    if (type == NVSEMessagingInterface::kMessage_SaveGame && data)   // P4 handover 8: the gear ledger is written next to the save it belongs to
        g_sidecars.onSave(ResolveSavePath(std::string(static_cast<const char*>(data), len ? strnlen(static_cast<const char*>(data), len) : strlen(static_cast<const char*>(data)))), SidecarIo());
    if (type == NVSEMessagingInterface::kMessage_SaveGame && data)   // party saves: the host's save orders every guest to save; a guest's hand save is asked of the host
        partyglue::onSaved(g_rt.get(), std::string(static_cast<const char*>(data), len ? strnlen(static_cast<const char*>(data), len) : strlen(static_cast<const char*>(data))));
    if (type == NVSEMessagingInterface::kMessage_PreLoadGame && data) {
        g_loadPath = ResolveSavePath(std::string(static_cast<const char*>(data), len ? strnlen(static_cast<const char*>(data), len) : strlen(static_cast<const char*>(data))));
        g_sidecars.onPreLoad(g_loadPath);
    }
    if (type == NVSEMessagingInterface::kMessage_PostLoadGame && data) {   // the ledger of the loaded save comes back (none = reset); puppets register again
        g_sidecars.onPostLoad(SidecarIo());
        nvhooks::clearPlayerSlots();
        g_slotRegistrar.reset();
    }
    if (type == NVSEMessagingInterface::kMessage_NewGame) {   // a new game has no ledger and no known puppets
        g_sidecars.onPreLoad(std::string());
        g_sidecars.onPostLoad(SidecarIo());
        nvhooks::clearPlayerSlots();
        g_slotRegistrar.reset();
    }
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
    if (type == NVSEMessagingInterface::kMessage_PostLoadGame && data) {
        g_gameLoaded = true;
        partyglue::onLoaded(g_rt.get(), g_loadPath);   // the campaign of the loaded save (its .fnvparty sidecar)
        g_noIntro = NoIntroJoin{};
        g_intro = IntroHold{};
        g_intro.check = true;   // IntroFrame looks at the loaded state once: the start save holds the party
        g_chargen = GuestChargen{};
    }
    if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) {
        UnwireHookObservers();
        if (g_rt) { nvhooks::SetPlayerControlsLocked(false); g_rt->stop(); }
    }
}

}  // namespace

// LATE: this frame sends what the avatar module (m2_avatar) captured in the same frame; run first, every avatar snapshot would leave one frame late.
NVCOOP_MODULE_LATE(l10_coop, "l10_coop", OnLoad, OnMessage, OnFrame);

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
    UnwireHookObservers();
#ifdef NVCOOP_WITH_UI
    fndlg::GuestDialogue::instance().frame(nullptr, static_cast<uint32_t>(NowMs()));
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
        w.boolean(false);   // (a retired flag; the schema keeps the slot)
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
        (void)r.boolean();
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
