// Co-op runtime (L10): glues synccore sessions, a transport, L5's CaptureHub and an engine-side world applier.
// Game-free: the engine is reached only through IWorldApplier (in-game: L5 RemoteApplier; tests: a fake world).
//
//   Host   : CaptureHub (outermost capture scope closes) -> HostSession::commit -> broadcast; tick() each frame.
//   Client : ClientSession -> IEffectSink -> IWorldApplier::apply (main thread); tick() each frame.
// Everything runs on the thread that calls frame() (the game main thread). No locks.
#pragma once
#include <cstdint>
#include <deque>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <vector>

#include "capture_hub.h"
#include "fnvcoop/baseline.h"
#include "fnvcoop/replicator.h"
#include "fnvcoop/rules.h"
#include "fnvcoop/session.h"
#include "fnvcoop/tether_watch.h"
#include "fnvcoop/transport_util.h"
#include "fnvcoop/worker.h"
#include "fnvcoop_dlg/adapter.h"
#include "hooks_ui/ui_bridge.h"
#include "fnvcoop/enet_transport.h"
#include "udp_transport.h"

namespace fnvcoop::flight { class FlightRecorder; }   // T1a flight recorder (synccore/flight/flight.h)

namespace nvcoop::coop {

enum class Role { Off, Host, Client };
const char* roleName(Role r);

struct Config {
    bool enable = false;            // [coop] enable=1 (default off: a stray nvcoop.dll must not open sockets)
    Role role = Role::Off;          // [coop] role=host|client|guest (guest == client)
    std::string listen = "127.0.0.1";  // host: bind address (use 0.0.0.0 to accept friends)
    std::string host = "127.0.0.1";    // client: host address
    uint16_t port = 27960;
    std::string transport = "enet";  // "enet" (default, L4; a Steam session is ENet to the local tunnel of fnvsteam.exe, docs/steam.md) | "udp" (L10 stand-in, kept as a test double)
    uint64_t token = 0;             // client identity; 0 = derive from time/pid (set it in the ini for stable slots)
    bool captureEnabled = true;     // host: wire CaptureHub -> commit
    bool applyEnabled = true;       // client: wire sink -> world
    uint32_t maxPlayers = 4;
    int party = 0;                  // [coop] party=N: the launcher lobby's size; a game started from the start save waits for N players (0 = nobody to wait for)
    bool forwardActivate = false;   // [coop] forward_activate=1: guest Activate -> submitAction to the host (suppresses the local call)
    bool clientLoadBlock = true;    // [coop] clientload=block|rejoin: host policy. block (default): a joined guest may not load a save, the game
                                    // refuses it (load_guard.h); rejoin: the guest's load is allowed and it re-baselines from the host
    std::string rulesFile;          // [coop] rules=<path to rules.json>: quest rules engine on the host (empty = off)
    uint8_t compareClasses = 0xDF;  // desync digest: entry classes the dump really covers (0..4 quest/stage/obj/vars/globals, 6 factions, 7 flags; 5 refs only with watch refs)
    uint32_t digestIntervalMs = 10000;   // client desync digest interval (0 = off)
    bool handshake = false;         // check build / rules / volatile / mod-list identity on join (the module turns it on; tests/fakes leave it off)
    bool useWorker = true;          // one background ThreadWorker for hashing / diffing (L4 setWorker)
    bool dialogue = false;          // [coop] dialogue=1: enable the 0xD1 dialogue channel
    bool baseline = false;          // [coop] baseline=1: join-in-progress via world baseline (needs an IWorldDumper)
    size_t logCapacity = 4096;
};

// Parses [coop] from ini text. Unknown keys ignored. Returns false (and *err) on a bad value.
bool ParseConfig(const std::string& iniText, Config* out, std::string* err);

struct ApplyResult {
    bool ok = false;
    std::string status;  // e.g. "Ok", "Unsupported", "FormMissing"
};

// Engine side of a client. Called on the main thread, inside the session's EchoGuard.
class IWorldApplier {
public:
    virtual ~IWorldApplier() = default;
    virtual ApplyResult apply(const fnvcoop::Effect& e, const fnvcoop::IEffectSink::Context& ctx) = 0;
    virtual void onJoined(fnvcoop::PlayerId /*slot*/, const fnvcoop::SharePolicy& /*policy*/) {}
};

// Engine dump of the shared world for the baseline join (L7 dumper in-game; fake world in tests). Main thread.
class IWorldDumper {
public:
    virtual ~IWorldDumper() = default;
    virtual bool dump(fnvcoop::ShadowState* out, uint64_t* contentHash, std::string* err) = 0;
};
// Host: run a requested ActivateRef/TriggerEnter/PickUp on the real engine. The runtime sets the origin of the
// resulting txn to `slot`; the executor opens its own CaptureHub scope so the effects are captured and committed.
using ActionExecutor = std::function<void(fnvcoop::PlayerId slot, fnvcoop::IntentKind kind, const fnvcoop::FormRef& ref)>;
struct DialogueHooks {
    fnvcoop_dlg::ResultRunner run;          // host: run the INFO result script under capture, return committed seq
    fnvcoop_dlg::DialogueNode::Sink sink;   // every node: engine glue (OpenMenu, ShowLines, ...)
    std::optional<fnvcoop_dlg::CompanionRules> companionRules;   // follower caps handed to every node (G33; default 1 humanoid + 1 critter each, party total 4)
};

// L11 hooks_ui wiring seams (production: nvui::router / nvui::slideHost / nvui::slideClient / nvui::setInfoReader; tests: fakes).
struct UiWiring {
    std::function<nvui::UiRouter*()> router;                             // host: its bridge is replaced by the DialogueNode bridge
    std::function<nvui::SlideHost&(nvui::SlideHost::Send)> slideHost;    // host: the send fn rides the 0xE5 mux channel
    std::function<nvui::SlideClient&()> slideClient;                     // guest: receives 0xE5, SlideshowEnd effect, host loss
    std::string consentFile;                                             // plugin/dialogue/data/consent_messages.txt (empty/unreadable = no consent table)
    std::function<void(std::function<std::vector<fnvcoop_dlg::Line>(uint32_t)>)> installInfoReader;   // nvui::setInfoReader
    std::function<std::vector<fnvcoop_dlg::Line>(uint32_t)> infoReader;  // L8 fndlg::makeInfoReader(...)
    std::function<bool(const fnvcoop::FormRef& quest, uint32_t idx, std::string* name)> varName;   // wave 1b: nvhooks::questVarName (per-player condition views)
};

// L12 lane-local world effects (NPC actor values, essential/restrained flags, faction relations, script packages): no synccore Effect type
// exists, so the host ships them as JSON on the session's AppData channel. Entries are keyed: a later push with the same key replaces the
// earlier one (state-like effects), an empty key accumulates (deltas). A guest that joins later gets the whole remembered state.
constexpr uint16_t kAppWorldChannel = 0x0C12;
// Guest side: apply one effect (the worldEffectToJson text); returns "Ok" or an error status.
using WorldEffectApplier = std::function<std::string(const std::string& json)>;

// Player orders (campaign gap G42 / G35 / G10): things that concern ONE machine's own player and are events, not shared state, so they must not ride the
// story log (a late joiner would replay a death). The host sends a JSON order to a guest slot (`{"op":"kill"}`, `{"op":"stash",...}`); a guest sends a
// request to the host (`{"op":"note","note":"<ref>"}`). Best effort on the AppData channel, no replay.
constexpr uint16_t kAppOrderChannel = 0x0C14;
// Guest: perform an order the host sent for THIS machine's player. Returns "Ok" or an error status.
using OrderApplier = std::function<std::string(const std::string& json)>;
// Host: a guest's request. `from` is the sender's slot. Returns "Ok" or an error status.
using OrderHandler = std::function<std::string(fnvcoop::PlayerId from, const std::string& json)>;
// Host: wraps one dialogue result script run (the initiator's player facts, P2 PlayerFactsScope). The returned handle is kept alive for the run.
using ResultScope = std::function<std::shared_ptr<void>(const fnvcoop_dlg::ResultRequest&)>;

using EdidResolver = std::function<bool(const std::string& edid, fnvcoop::FormRef& out)>;
using InfoQuestResolver = std::function<bool(const fnvcoop::FormRef& info, fnvcoop::FormRef& quest)>;
// Host: perform the REAL engine SetStage (capture open) when the rules engine releases a deferral.
using StageApplier = std::function<void(const fnvcoop::FormRef& quest, uint32_t stage)>;

struct LogEntry {
    std::string dir;  // "commit" (host), "apply" (client), "inject"
    uint64_t seq = 0;
    uint64_t atMs = 0;
    fnvcoop::PlayerId origin = 0;
    fnvcoop::Cause cause;
    std::vector<fnvcoop::Effect> effects;
    std::vector<std::string> results;  // client: per-effect apply status
};

class Runtime {
public:
    // hub may be null (client needs none; a host without it can still `inject`).
    Runtime(Config cfg, nvhooks::CaptureHub* hub, IWorldApplier* world, std::function<uint64_t()> nowMs);
    ~Runtime();

    // How the host receives outermost-scope txns. Default hub->addSink (slot 0 stays with L5's capture.end bridge).
    void setDumper(IWorldDumper* d) { dumper_ = d; }
    // Host: reads one quest's stage / completed / failed from the engine (false = unknown). Feeds HostSession::hostQuestState; before start().
    using QuestStateReader = std::function<bool(const fnvcoop::FormRef& quest, int32_t& stage, bool& completed, bool& failed)>;
    void setQuestStateReader(QuestStateReader f) { questState_ = std::move(f); }
    // Host: reads every objective of one quest (objective id -> status bits 0..3) from the engine (false = unknown quest). Feeds HostSession::hostObjectives; before start().
    using ObjectiveStateReader = std::function<bool(const fnvcoop::FormRef& quest, std::map<uint32_t, uint8_t>& objectives)>;
    void setObjectiveStateReader(ObjectiveStateReader f) { objectiveState_ = std::move(f); }
    // Host: reads every script var (index -> value) of one owner (a quest) from the engine; false = unknown owner / no script variables.
    // Feeds HostSession::hostScriptVars (client-only SetScriptVar keys the baseline-less host never recorded); before start().
    using ScriptVarReader = std::function<bool(const fnvcoop::FormRef& owner, std::map<uint16_t, double>& vars)>;
    void setScriptVarReader(ScriptVarReader f) { scriptVars_ = std::move(f); }
    // Host: reads one quest's running flag from the engine (false = unknown quest). Feeds HostSession::hostQuestRunning when there is no world
    // baseline dump to read it from (a baseline-less host committed "not running" for every quest the client was running, and the client stopped them); before start().
    using QuestRunningReader = std::function<bool(const fnvcoop::FormRef& quest, bool& running)>;
    void setQuestRunningReader(QuestRunningReader f) { questRunning_ = std::move(f); }
    // Host: reads one global's value from the engine (false = unknown). Feeds HostSession::hostGlobals (client-only SetGlobal keys); before start().
    using GlobalReader = std::function<bool(const fnvcoop::FormRef& global, double& value)>;
    void setGlobalReader(GlobalReader f) { globals_ = std::move(f); }
    void setActionExecutor(ActionExecutor f) { executor_ = std::move(f); }
    // "Acting player" context (docs/protocol.md section 12): while the host runs a guest's interaction, the engine layer must resolve
    // `player` inside the target's script block to that guest. The hook is called (slot, true) before and (slot, false) after the executor.
    void setActingHook(std::function<void(fnvcoop::PlayerId slot, bool on)> f) { actingHook_ = std::move(f); }
    fnvcoop::PlayerId actingSlot() const { return actionOrigin_; }   // kNoPlayer outside an executor call
    // plugin/intents: run `body` as the acting guest `slot` exactly like the story-channel executor does (txn origin forced to the slot, the
    // acting hook on/off around it). Nesting-safe. Host only: returns false (body not run) without a host session.
    bool runAsGuest(fnvcoop::PlayerId slot, const std::function<void()>& body);
    // The runtime a plugin module that is not coop_module should talk to: the most recently started one, null after stop() / destruction.
    static Runtime* active() { return active_; }
    void enableDialogue(DialogueHooks h) { dlgHooks_ = std::move(h); dlgEnabled_ = true; }  // before start()
    // plugin/intents: replaces the host's dialogue ResultRunner (coop_module installs a stub that runs nothing) at any time, also after the DialogueNode
    // was built (the node calls through the Runtime). Empty = back to DialogueHooks::run.
    void setDialogueRunner(fnvcoop_dlg::ResultRunner r) { runnerOverride_ = std::move(r); }
    // Host with baseline=1: the exact dump handed to setWorldBaseline (null otherwise). Engine state drifts (timers),
    // so exports for tests must use THIS copy, not a fresh dump.
    const fnvcoop::ShadowState* usedBaseline(uint64_t* hash) const { if (hash) *hash = usedHash_; return usedBaseline_.get(); }
    void enableUi(UiWiring w) { ui_ = std::move(w); uiEnabled_ = true; }   // before start()
    struct UiStats { uint32_t bridges = 0, resumes = 0, slidePackets = 0, slideEnds = 0, hostLost = 0, consentRows = 0; };
    const UiStats& uiStats() const { return uiStats_; }
    // ---- world effects (L12). Host: pushWorldEffect from the engine glue (outside script scopes); guest: setWorldEffectApplier before start().
    void setWorldEffectApplier(WorldEffectApplier f) { worldApplier_ = std::move(f); }
    // `ephemeral` (nvworld::isEphemeral: an image-space fade, a sound): sent to the guests connected now and NOT remembered, so a late joiner never replays it.
    void pushWorldEffect(const std::string& key, const std::string& json, bool ephemeral = false);
    struct WorldStats { uint64_t pushed = 0, sent = 0, replayed = 0, received = 0, applied = 0, failed = 0, stale = 0, ephemeral = 0; };
    const WorldStats& worldStats() const { return worldStats_; }
    // ---- player orders (kAppOrderChannel). Set before start().
    void setOrderApplier(OrderApplier f) { orderApplier_ = std::move(f); }
    void setOrderHandler(OrderHandler f) { orderHandler_ = std::move(f); }
    bool sendOrder(fnvcoop::PlayerId toSlot, const std::string& json);   // host -> one joined guest; false = no such guest / not a host
    bool sendOrderToHost(const std::string& json);                       // guest -> host; false = not joined or still syncing
    // G42: the player of `slot` dies, nobody else. Slot 0 is the host's own player (the caller kills it on this machine); a guest gets a kill order.
    bool killPlayer(fnvcoop::PlayerId slot);
    struct OrderStats { uint64_t sent = 0, received = 0, applied = 0, failed = 0, handled = 0, handlerFailed = 0; };
    const OrderStats& orderStats() const { return orderStats_; }
    void setResultScope(ResultScope f) { resultScope_ = std::move(f); }   // host
    // ---- what this machine's session knows about each player (party_state.h builds the condition providers from these). Host: the shared shadow state.
    std::vector<fnvcoop::PlayerId> joinedSlots() const;                              // host: joined guests (not slot 0), ascending
    bool slotPerkRank(fnvcoop::PlayerId slot, const fnvcoop::FormRef& perk, int32_t* rank) const;   // SetPerk ledger of that slot (false: no such slot / not a host)
    bool sharedJournalHas(const fnvcoop::FormRef& note) const;                       // AddNote applies to everyone: the shared journal
    int companionOwnerOf(const fnvcoop::FormRef& npc) const;                         // slot that recruited it, -1 = nobody / unknown
    // ---- campaign wave 2 (P6): the rules engine wired into the session. Configure BEFORE start() unless noted. ----
    // G02: docs/quest-policies/cause_quests.json text (script / INFO / ref -> owning quest); loaded into the rules engine when it is built.
    void setCauseQuests(const std::string& json) { causeQuestsJson_ = json; }
    // Policy overrides ("death.mode", "tether.mode" ...) from nvcoop.ini [rules]; RulesEngine::Options::policy.
    void setPolicy(std::map<std::string, std::string> p) { policy_ = std::move(p); }
    // Host: effects the rules engine commits BY ITSELF (control-lock holds and timeouts, tether gathers, late-join warps) never ran as a script on this machine;
    // the ones that concern the host's own player (SetPlayerControls for slot 0, MoveRef of its player ref) are applied here. G18, G25, G41.
    using HostEffectApplier = std::function<void(const fnvcoop::Effect&)>;
    void setHostEffectApplier(HostEffectApplier f) { hostApplier_ = std::move(f); }
    // A HUD line on THIS machine (the module: the game's notification). notifyPlayer sends one to a guest's machine as an order.
    void setNotice(std::function<void(const std::string&)> f) { notice_ = std::move(f); }
    // One log line per time the session MOVES or HOLDS a player (a tether hold or gather, a late joiner's hold and warp): what, why, who, from where to where. The module: nvcoop.log.
    void setTrace(std::function<void(const std::string&)> f) { trace_ = std::move(f); }
    bool notifyPlayer(fnvcoop::PlayerId slot, const std::string& text);
    // slot -> that player's FormRef: THE mapping of RulesEngine::Options::playerRef (slot 0 = the host's player, a guest = its ghost / puppet). nvhooks::setPlayerRefProvider uses it too.
    bool playerRefOf(fnvcoop::PlayerId slot, fnvcoop::FormRef& out) { return slotPlayerRef(slot, out); }
    // Orders for the local player ("downed", "respawn", ...) go through the same applier as the orders a guest receives: slot 0 is applied here, a guest by order.
    bool deliverOrder(fnvcoop::PlayerId slot, const std::string& json);
    // G24: read / write the engine variables of a quest by name (the module walks the quest's script variable list).
    struct PlayerVarIO {
        std::function<bool(const fnvcoop::FormRef& quest, const std::vector<std::string>& names, std::vector<fnvcoop::RulesEngine::VarValue>& out)> read;
        std::function<void(const fnvcoop::FormRef& quest, const std::vector<fnvcoop::RulesEngine::VarValue>& values)> write;
        std::function<bool(const fnvcoop::FormRef& quest, uint32_t idx, std::string* name)> name;   // W3a item 4: a script variable index -> its name (nvhooks::questVarName)
    };
    // W3a item 5 (P6 request 1): the SetPlayerControls hook asks, inside the open capture scope, whether a GUEST's lock / release concerns that guest only (the quest's
    // SetPlayerControls rule says scope=initiator). true = the host's own engine call is skipped; the host is not locked for a guest's scene.
    bool controlsGuestOnly(fnvcoop::PlayerId slot, bool disable);
    // W3a item 4: what a script that runs for a guest reads of the companion variables. viewSlotOf(script, acting) is the policy (companions glueViewSlot): the acting slot's own
    // view, or 0xFE (sc::kViewParty) = the shared variable as it is (the party aggregate). followerOf(slot, ...) names the Dead Money ref variable and the form id of the
    // companion that player has in the group (0 = none) for NVDLC01Followers.rCurrentFollower.
    using ViewSlotOf = std::function<uint8_t(uint32_t scriptRefID, uint8_t acting)>;
    struct FollowerVar { fnvcoop::FormRef quest; std::string var; double formId = 0; };
    using FollowerOf = std::function<bool(fnvcoop::PlayerId slot, FollowerVar* out)>;
    void setViewPolicy(ViewSlotOf v, FollowerOf f) { viewSlotOf_ = std::move(v); followerOf_ = std::move(f); }
    // The Script::Execute scope hook (nvhooks::setScriptRunScope): before = true right before the script runs, false right after.
    void onScriptRun(uint32_t scriptRefID, bool before);
    void setPlayerVarIO(PlayerVarIO io) { varIO_ = std::move(io); }
    size_t actingQuestCount() const { return actingQuests_.size(); }
    // G25: positions for the anchor tethers. The host watches every joined player: its own from the engine, a guest's from that guest's puppet.
    struct AnchorProbe {
        std::function<bool(const fnvcoop::FormRef& anchor, float& x, float& y, float& z)> anchorPos;
        std::function<bool(fnvcoop::PlayerId slot, float& x, float& y, float& z)> slotPos;
    };
    void setAnchorProbe(AnchorProbe p) { anchorProbe_ = std::move(p); }
    // W3d item 1 (W3b request 1): RulesEngine::Options::itemCount / playerDistance. The count of an item one player holds (GetItemCount of that player, the engine evaluator
    // on the player's own reference) feeds the pooled hand-ins (PooledItems); the distance of two players is taken from the same positions as the anchor tethers
    // (AnchorProbe::slotPos), game units. Without a probe a pooled item is charged to whoever handed it in and a ranged PartyWarp (the Teleporter) moves the firer alone.
    using ItemCountProbe = std::function<bool(fnvcoop::PlayerId slot, const fnvcoop::FormRef& item, int64_t& count)>;
    void setItemCountProbe(ItemCountProbe f) { itemCountProbe_ = std::move(f); }
    // W3d item 1 (W3b request 3, G24): the scripts the rules run once per joined slot (RulesEngine::perPlayerScripts), each resolved to its form, with the names of its own
    // variables that keep a copy per slot (the `Script.var` entries of perPlayerObjVarNames of its quest, lower case, without the script prefix).
    struct PerSlotScriptInfo { fnvcoop::FormRef script; std::string edid; std::vector<std::string> vars; };
    std::vector<PerSlotScriptInfo> perSlotScripts() const;
    // Runs `body(slot)` once for every joined guest (not the host: the engine's own run is the host's) inside that guest's window: the commit origin is the slot, the quest
    // variables of the script's quest hold the slot's copies (RulesEngine::beginActing) and the host's own copies come back afterwards. Returns how many slots ran.
    size_t runScriptPerSlot(uint32_t scriptRefID, const std::function<void(fnvcoop::PlayerId slot)>& body);
    // Local player is now in `cell` of worldspace `world` (empty = an interior). Exterior tethers name the worldspace, so a guest reports the worldspace as its cell.
    void reportCell(const fnvcoop::FormRef& cell, const fnvcoop::FormRef& world);
    // G26: a guest's SetStage intent on a pass stage (chargen end) is that guest's pass. true = the stage is held until the whole party passed (do NOT run it now).
    bool stageHeldForPass(fnvcoop::PlayerId slot, const fnvcoop::FormRef& quest, uint32_t stage);
    // G40: the combat layer asks before a player's hit lands on an actor. Allow also for an actor no rule protects.
    enum class HitVerdict : uint8_t { Allow, Block, BlockWarn };
    HitVerdict protectedHit(const fnvcoop::FormRef& victim, fnvcoop::PlayerId attacker, bool splash);
    // G41: this machine's player died / was revived. Host: the rules engine decides (downed / reload the host save / normal death); guest: the host is told.
    void localPlayerDied();
    void localPlayerRevived();
    // Host: the verdict for the death of `slot` (also what a guest's "died" order runs). The reaction reaches the machine of that player as an order.
    fnvcoop::RulesEngine::DeathVerdict playerDied(fnvcoop::PlayerId slot);
    // Host: every joined player is down and policy death.all_down is reload-host-save: the module reloads the host's save.
    void setReloadHostSave(std::function<void()> f) { reloadSave_ = std::move(f); }
    // P7: the companion roster as the social module has it (owner and party flag per companion), after every roster broadcast. Feeds the dialogue registry (G33), and on the
    // host commits the registry's reconcile() fixes: the shared "hired" / "in party" quest variables go back in line with who really owns whom.
    struct RosterEntry { fnvcoop::FormRef npc; fnvcoop::PlayerId owner = fnvcoop::kNoPlayer; bool party = false; };
    void onRosterChanged(const std::vector<RosterEntry>& all);
    struct RulesStats { uint64_t rosterFeeds = 0, reconcileFixes = 0, casesGathered = 0, lateJoinHeld = 0, lateJoinWarped = 0, hostEffectsApplied = 0, nearReports = 0, actingBrackets = 0, deaths = 0, rulesLinks = 0, controlsHostSkipped = 0, companionViews = 0, perSlotRuns = 0, itemCountAsks = 0, distanceAsks = 0; };
    const RulesStats& rulesStats() const { return rulesStats_; }
    fnvcoop_dlg::DialogueNode* dialogue() { return dlg_.get(); }  // null until the session is joined
    fnvcoop::MuxTransport* mux() { return mux_.get(); }
    // Client: ask the host to run an activation. Returns 0 (and does nothing) while syncing or not joined.
    uint32_t submitAction(fnvcoop::IntentKind kind, const fnvcoop::FormRef& ref);
    // ---- quest rules engine (L4): host side. Configure BEFORE start().
    bool setRules(const std::string& jsonText, std::string* err);    // or Config::rulesFile
    // Build identity for the join handshake (build number/hash/name, mod load order). rulesHash/volatileHash are filled by the runtime.
    void setHandshakeBase(const fnvcoop::HandshakeInfo& h) { hsBase_ = h; }
    // PartyWarp (rules engine): slot -> that player's ref. Default: slot 0 = the host player (0x14 of falloutnv.esm), guests via the ghost
    // resolver below (none -> the slot is skipped). Evaluated lazily on every use, so it stays correct across loads / rebases.
    void setPlayerRefResolver(std::function<bool(fnvcoop::PlayerId, fnvcoop::FormRef&)> f) { playerRef_ = std::move(f); }
    // Standard wiring used by the module: slot 0 -> the host player (engine refID 0x14), guest slots -> ghostOf(slot) (L6 GhostRefOfPlayer,
    // 0 = no ghost known yet => slot skipped). `idToRef` converts an engine refID to a FormRef (false = unresolvable). Call at session start;
    // onGameLoaded() re-asserts it (counted in status.playerRefs.binds) because ghosts are respawned by a load. Lazy on every use.
    void bindPlayerRefs(std::function<uint32_t(fnvcoop::PlayerId)> ghostOf, std::function<bool(uint32_t, fnvcoop::FormRef&)> idToRef);
    void setRulesFile(const std::string& path) { cfg_.rulesFile = path; }
    void setEdidResolver(EdidResolver f) { edid_ = std::move(f); }
    void setInfoQuestResolver(InfoQuestResolver f) { infoQuest_ = std::move(f); }
    void setStageApplier(StageApplier f) { stageApplier_ = std::move(f); }
    fnvcoop::RulesEngine* rules() { return rules_.get(); }
    // Host: coalescing channel for quest script vars / globals polled from the engine (L5 pollHostState). null on clients.
    fnvcoop::StateReplicator* replicator() { return replicator_.get(); }
    // Host: the engine poll that feeds the replicator (L5 pollHostState), run once per frame right before the replicator flushes. It runs
    // WITH OR WITHOUT a world baseline: quest vars that object / NPC scripts write inline (`set VMS16.nGangerDeathCount to ... + 1` in a
    // ganger's OnDeath) are inside no capture scope, so the poll is the only way they reach the shared state. `first` = the replicator is
    // new (a new session): the poll resets its change tracker. `prime` = first AND no baseline: the machine's current values are taken as the
    // known state (a baseline would have transferred them) and only later changes are reported, instead of flooding every quest var.
    using StatePollFn = std::function<void(fnvcoop::StateReplicator& to, bool first, bool prime)>;
    void setStatePoll(StatePollFn f) { statePoll_ = std::move(f); polledRep_ = nullptr; }
    // Host: the engine wants to advance `quest` to `stage`. false => DEFERRED (skip the engine call; stageApplier fires on release).
    // Calls made by the stage applier itself, and everything while no rules engine is loaded, proceed.
    bool stageRequested(const fnvcoop::FormRef& quest, uint32_t stage, fnvcoop::PlayerId initiator = 0);
    // Host: the quest rules refuse `stage` outright (ExclusiveStages: another outcome of the set was committed first). False without a rules engine.
    bool stageExcluded(const fnvcoop::FormRef& quest, uint32_t stage) const;
    // Local player is now in `cell` (host: tells the rules engine; client: ReportCell action). No-op while syncing.
    void reportCell(const fnvcoop::FormRef& cell);
    // Local player finished their per-player step of `quest` (chargen...): host -> rules engine, client -> ScenePass action.
    void scenePass(const fnvcoop::FormRef& quest);
    bool saveSuppressed() const;   // client: shared state says saving is blocked

    // ---- save / load (L4 round 6). Game-free: the module maps NVSE SaveGame/PreLoadGame/PostLoadGame onto these.
    // Host: session sidecar (shared world + epoch + token map). Client: its own view (empty if !canSaveState()).
    fnvcoop::Bytes saveSidecar();
    // PostLoad: `sidecar` = the file stored with the loaded save (null/empty if none). Host: loadSidecar, else reloadWorld from a
    // fresh engine dump -> epoch+1, clients rebase. Client: onLocalSaveLoaded (resume from its sidecar or re-baseline). Returns false
    // when nothing could be done (client policy Block, or host without dumper). `what` describes the path taken.
    bool onGameLoaded(const fnvcoop::Bytes* sidecar, std::string* what);
    bool rebaselineHost(std::string* what);   // test tooling: re-dump the host world as a new epoch (guests rebase)
    // Client: may a load happen now? (false once joined under clientload=block; the engine layer must refuse it)
    bool clientLoadAllowed() const { return !client_ || client_->clientLoadAllowed(); }

    // Client: world baseline transfer in progress -> block local gameplay/intents.
    bool syncing() const { return client_ && client_->baselineActive(); }

    void setSinkInstaller(std::function<void(nvhooks::CaptureHub::Sink)> f) { installSink_ = std::move(f); }

    bool start(std::string* err);  // opens the transport and builds the session
    void stop();
    // coop.leave / coop.join: drop out of the session and back in without ending the game process. leave() is stop() plus a flag; the
    // local world is left as it is. join() builds a fresh session (client: new ClientSession, baseline re-dump when baseline=1, so the host
    // sends only the difference to what this machine already has: nothing earned before leaving is granted twice).
    bool leave(std::string* err);
    bool join(const std::string& host, uint16_t port, std::string* err);   // empty host / port 0 keep the configured ones
    bool left() const { return left_; }
    bool running() const { return running_; }
    void* workerHandle() { return worker_ ? reinterpret_cast<void*>(worker_->nativeHandle()) : nullptr; }   // null: no worker thread
    void frame();                  // one tick on the main thread

    // coop.inject: host = commit as if captured; client = deliver as if received (applied through the sink path).
    bool inject(const std::string& txnJson, std::string* resultJson, std::string* err);
    std::string statusJson() const;
    std::string logJson(size_t maxEntries) const;

    // Observer for every log entry (peer tooling prints these).
    std::function<void(const LogEntry&)> onLog;

    const Config& config() const { return cfg_; }
    fnvcoop::HostSession* host() { return host_.get(); }
    fnvcoop::ClientSession* client() { return client_.get(); }
    // T1a flight recorder: call before start(). Every datagram and every applied/committed effect is then recorded (null = off).
    void setFlight(fnvcoop::flight::FlightRecorder* r) { flight_ = r; }
    UdpTransport* transport() { return udp_; }            // null unless transport=udp
    fnvcoop::EnetTransport* enet() { return enet_; }      // null unless transport=enet
    uint16_t boundPort() const { return enet_ ? enet_->port() : udp_ ? udp_->localPort() : 0; }
    const std::string& lastError() const { return lastError_; }

    struct Counters {
        uint64_t frames = 0, captured = 0, committed = 0, droppedEmpty = 0;
        uint64_t applied = 0, applyFailed = 0, injected = 0, actions = 0, rulesActions = 0, disconnects = 0, baselineEffects = 0, saves = 0, loads = 0, loadsBlocked = 0;
        uint64_t reportsDropped = 0;   // host: guest presence reports no rules engine consumed (never run as an activation)
        uint64_t reconnects = 0;   // client: link re-created after the transport dropped (a stall, a long load, a network blip)
    };
    static constexpr uint64_t kReconnectDelayMs = 2000;
    const Counters& counters() const { return counters_; }

private:
    class Sink;
    bool makeTransport(bool server, std::string* err);
    fnvcoop::ITransport& sessionTransport();   // transport_, or the flight-recording wrapper around it
    void tickFlight();
    void flightDesync(const std::string& why);
    void pushLog(LogEntry e);
    void onCaptured(fnvcoop::EffectTxn&& t);
    void tickDialogue();
    void tickUi();
    void tickWorld();
    void noteCompanion(const fnvcoop::CompanionOwner& c);
    void syncCompanions();
    void onWorldPacket(const fnvcoop::Bytes& payload);
    void onOrderPacket(fnvcoop::PlayerId from, const fnvcoop::Bytes& payload);
    void wireRulesCallbacks();
    void buildRulesIndexes();
    void tickRulesSession(uint64_t t);
    void tickAnchors(uint64_t t);
    void tickLateJoin(uint64_t t);
    uint64_t dialogueResult(const fnvcoop_dlg::ResultRequest& rq);
    bool slotPlayerRef(fnvcoop::PlayerId slot, fnvcoop::FormRef& out);
    void actingBegin(fnvcoop::PlayerId slot);
    void actingBeginQuest(const fnvcoop::FormRef& quest, fnvcoop::PlayerId slot);
    void actingEndQuest(const fnvcoop::FormRef& quest, fnvcoop::PlayerId slot);
    void actingEnd(fnvcoop::PlayerId slot);
    void applyCompanionView(fnvcoop::PlayerId view);
    void onHostCommitted(const fnvcoop::EffectTxn& t);
    void applyWorldEntry(uint64_t id, const std::string& key, const std::string& json);
    void wireUiHost();
    void uiSlideEnded();
    nvui::SlideHost* slideHost();
    nvui::SlideHost* slideHost_ = nullptr;

    Config cfg_;
    nvhooks::CaptureHub* hub_;
    IWorldApplier* world_;
    std::function<uint64_t()> now_;
    std::function<void(nvhooks::CaptureHub::Sink)> installSink_;

    std::unique_ptr<fnvcoop::ITransport> transport_;
    UdpTransport* udp_ = nullptr;
    fnvcoop::EnetTransport* enet_ = nullptr;
    fnvcoop::ILinkTransport* link_ = nullptr;   // enet_: the connection events and the reconnect of a client
    fnvcoop::HandshakeInfo hsBase_;
    std::unique_ptr<fnvcoop::ThreadWorker> worker_;
    fnvcoop::RulesDb rulesDb_;
    bool haveRules_ = false;
    std::unique_ptr<fnvcoop::RulesEngine> rules_;
    std::unique_ptr<fnvcoop::StateReplicator> replicator_;
    StatePollFn statePoll_;
    const fnvcoop::StateReplicator* polledRep_ = nullptr;   // the replicator the poll last ran for (a new one = a new session)
    EdidResolver edid_;
    std::function<bool(fnvcoop::PlayerId, fnvcoop::FormRef&)> playerRef_;
    std::function<uint32_t(fnvcoop::PlayerId)> ghostOf_;
    std::function<bool(uint32_t, fnvcoop::FormRef&)> idToRef_;
    uint32_t playerRefBinds_ = 0;
    InfoQuestResolver infoQuest_;
    StageApplier stageApplier_;
    bool applyingDeferred_ = false;
    std::unique_ptr<fnvcoop::MuxTransport> mux_;
    IWorldDumper* dumper_ = nullptr;
    QuestStateReader questState_;
    ObjectiveStateReader objectiveState_;
    ScriptVarReader scriptVars_;
    QuestRunningReader questRunning_;
    GlobalReader globals_;
    std::unique_ptr<fnvcoop::ShadowState> usedBaseline_;
    uint64_t usedHash_ = 0;
    ActionExecutor executor_;
    std::function<void(fnvcoop::PlayerId, bool)> actingHook_;
    DialogueHooks dlgHooks_;
    bool dlgEnabled_ = false;
    std::unique_ptr<fnvcoop_dlg::DialogueNode> dlg_;
    std::set<fnvcoop::PlayerId> dlgJoined_;
    bool dlgWasLive_ = false;
    UiWiring ui_;
    bool uiEnabled_ = false, uiBridged_ = false, uiWasJoined_ = false;
    std::set<fnvcoop::PlayerId> uiJoined_;
    UiStats uiStats_;
    fnvcoop::PlayerId actionOrigin_ = fnvcoop::kNoPlayer;
    std::map<fnvcoop::FormRef, fnvcoop::PlayerId> companionOwners_;   // replicated CompanionOwner state (the dialogue node may not exist yet)
    WorldEffectApplier worldApplier_;
    struct WorldEntry { uint64_t id = 0; std::string json; };
    std::map<std::string, WorldEntry> worldState_;   // host: latest entry per key (replayed to late joiners)
    uint64_t worldId_ = 0;
    std::set<fnvcoop::PlayerId> worldJoined_;
    std::deque<std::pair<std::string, WorldEntry>> worldQueue_;   // guest: received before the baseline finished
    std::map<std::string, uint64_t> worldApplied_;                // guest: newest id applied per key (duplicate / stale guard)
    WorldStats worldStats_;
    OrderApplier orderApplier_;
    OrderHandler orderHandler_;
    OrderStats orderStats_;
    ResultScope resultScope_;
    // rules wiring state (wave 2)
    std::string causeQuestsJson_;
    std::map<std::string, std::string> policy_;
    HostEffectApplier hostApplier_;
    std::function<void(const std::string&)> notice_;
    std::function<void(const std::string&)> trace_;
    void traceLine(const char* fmt, ...);   // printf-style into trace_ (no-op without one)
    std::string slotPosText(fnvcoop::PlayerId slot);   // "(x,y,z)" of the slot's player from the anchor probe, "(?)" when unknown
    std::function<void()> reloadSave_;
    PlayerVarIO varIO_;
    std::vector<std::pair<fnvcoop::FormRef, std::vector<std::string>>> actingQuests_;   // quests with PerPlayerVars / PerPlayerState and their variable names
    AnchorProbe anchorProbe_;
    ItemCountProbe itemCountProbe_;
    std::map<fnvcoop::FormRef, float> anchorRadius_;                                      // PartyTether anchors of the rules and their radius
    std::map<fnvcoop::PlayerId, fnvcoop::AnchorWatch> anchorWatch_;
    uint64_t anchorAt_ = 0, lateJoinAt_ = 0;
    std::set<fnvcoop::PlayerId> lateJoinSeen_, lateJoinPending_, lateJoinLocked_;
    bool inCapturedCommit_ = false;
    bool rulesLinked_ = false;                                                            // fnvcoop_dlg::linkRules ran for the current dialogue node
    std::vector<fnvcoop::Effect> capturedSnapshot_;
    RulesStats rulesStats_;
    uint32_t skippedLocks_ = 0;                     // controlsGuestOnly said "skip the host's call" for this many locks the committed-effect give-back must not undo
    ViewSlotOf viewSlotOf_;
    FollowerOf followerOf_;
    fnvcoop::PlayerId companionView_ = 0xFE;        // whose view of the companion variables is in the engine now (0xFE = the party aggregate)
    std::vector<fnvcoop::PlayerId> viewStack_;      // views to return to when the nested scripts of a guest's run end
    uint32_t idleRuns_ = 0;                         // scripts running outside any guest window (nothing to restore)
    bool followerSaved_ = false;
    double followerReal_ = 0;
    std::unique_ptr<fnvcoop::HostSession> host_;
    std::unique_ptr<Sink> sink_;
    std::unique_ptr<fnvcoop::ClientSession> client_;
    std::deque<LogEntry> log_;
    std::map<std::string, uint64_t> tally_;   // "<baseline|snapshot|live> <Effect>:<status>" -> count (survives log eviction)
    std::vector<std::string> firstErrors_;
    Counters counters_;
    uint64_t reconnectAt_ = 0;   // client: when to re-create the transport after it dropped (0 = connected or not needed)
    std::string lastError_;
    bool running_ = false;
    fnvcoop::Bytes leaveView_;   // client: saveState() taken by leave(), consumed by join()
    bool left_ = false;   // leave() was called and join() has not rebuilt the session yet
    uint64_t startMs_ = 0;
    bool captureOn_ = false, sinkInstalled_ = false;
    fnvcoop::flight::FlightRecorder* flight_ = nullptr;
    std::shared_ptr<fnvcoop::ITransport> flightT_;     // RecordingTransport over transport_ (only with flight_)
    std::unique_ptr<fnvcoop::ShadowState> flightWorld_;   // client: engine world of the last digest
    std::shared_ptr<fnvcoop::IEffectSink> flightS_;    // RecordingSink over sink_
    uint64_t flightIdentAt_ = 0, flightSeenSeq_ = ~0ull, flightSeenAt_ = 0, flightDetailAt_ = 0, flightDigestAt_ = 0, flightDigestSeq_ = ~0ull, flightHostMismatch_ = 0, flightClientRepairs_ = 0;
    std::shared_ptr<bool> alive_ = std::make_shared<bool>(true);
    static inline Runtime* active_ = nullptr;   // see active()
    fnvcoop_dlg::ResultRunner runnerOverride_;  // see setDialogueRunner()
};

}  // namespace nvcoop::coop
