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
#include <set>
#include <string>
#include <vector>

#include "capture_hub.h"
#include "fnvcoop/baseline.h"
#include "fnvcoop/replicator.h"
#include "fnvcoop/rules.h"
#include "fnvcoop/session.h"
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
    std::string transport = "enet";  // "enet" (default, L4) | "udp" (L10 stand-in, kept as a test double)
    uint64_t token = 0;             // client identity; 0 = derive from time/pid (set it in the ini for stable slots)
    bool captureEnabled = true;     // host: wire CaptureHub -> commit
    bool applyEnabled = true;       // client: wire sink -> world
    uint32_t maxPlayers = 4;
    bool forwardActivate = false;   // [coop] forward_activate=1: guest Activate -> submitAction to the host (suppresses the local call)
    bool forwardScripted = true;    // [coop] forward_scripted=0 turns it off: a guest's scripted activator (water pipes, switches) runs on the host for them
    bool sharedXp = true;           // [coop] sharedxp=0 turns it off: XP the engine pays one player (kills, locks, terminals, places) goes to every player
    bool containerSync = true;      // [coop] containers=0 turns it off: what a player takes from / puts into a container or corpse happens on the others' copies too
    bool respawnRestore = true;     // [coop] respawn=0 turns it off: after an NV:MP respawn (the character back at its last save) the items and XP from before the death return
    int autosaveMinutes = 5;        // [coop] autosave=N: save every N minutes at a calm moment (CoopAutosave1..3); 0 = off
    bool talkSubtitles = true;      // [coop] subtitles=0 turns it off: the lines of a player's conversation show as HUD messages for the others near the NPC
    bool guestChargen = true;       // [coop] guest_chargen=0 turns it off: a guest opens its own character-creation menus (name, face, traits,
                                    // SPECIAL, tag skills) when the shared story reaches them; the host's quest scripts open them only for the host
    bool deathWatch = true;         // [coop] deaths=0 turns it off: the host reports loaded actors it saw die (combat kills) as KillActor
    bool clientLoadBlock = false;   // [coop] clientload=block: host policy, guests may not load saves while joined (default rejoin)
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
};

// L11 hooks_ui wiring seams (production: nvui::router / nvui::slideHost / nvui::slideClient / nvui::setInfoReader; tests: fakes).
struct UiWiring {
    std::function<nvui::UiRouter*()> router;                             // host: its bridge is replaced by the DialogueNode bridge
    std::function<nvui::SlideHost&(nvui::SlideHost::Send)> slideHost;    // host: the send fn rides the 0xE5 mux channel
    std::function<nvui::SlideClient&()> slideClient;                     // guest: receives 0xE5, SlideshowEnd effect, host loss
    std::string consentFile;                                             // plugin/dialogue/data/consent_messages.txt (empty/unreadable = no consent table)
    std::function<void(std::function<std::vector<fnvcoop_dlg::Line>(uint32_t)>)> installInfoReader;   // nvui::setInfoReader
    std::function<std::vector<fnvcoop_dlg::Line>(uint32_t)> infoReader;  // L8 fndlg::makeInfoReader(...)
};

// L12 lane-local world effects (NPC actor values, essential/restrained flags, faction relations, script packages): no synccore Effect type
// exists, so the host ships them as JSON on the session's AppData channel. Entries are keyed: a later push with the same key replaces the
// earlier one (state-like effects), an empty key accumulates (deltas). A guest that joins later gets the whole remembered state.
constexpr uint16_t kAppWorldChannel = 0x0C12;
// Guest side: apply one effect (the worldEffectToJson text); returns "Ok" or an error status.
using WorldEffectApplier = std::function<std::string(const std::string& json)>;

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
    // Host: reads one quest's running flag from the engine (false = unknown). Without a world baseline (no dump to read it from) it settles
    // the quest-running keys only a client has; without it they defaulted to "not running" and a guest's quest the HOST also runs was
    // stopped (MS11 in Megaton, TTW 2026-10-27). Before start().
    using QuestRunningReader = std::function<bool(const fnvcoop::FormRef& quest, bool& running)>;
    void setQuestRunningReader(QuestRunningReader f) { questRunning_ = std::move(f); }
    void setActionExecutor(ActionExecutor f) { executor_ = std::move(f); }
    // "Acting player" context (docs/protocol.md section 12): while the host runs a guest's interaction, the engine layer must resolve
    // `player` inside the target's script block to that guest. The hook is called (slot, true) before and (slot, false) after the executor.
    void setActingHook(std::function<void(fnvcoop::PlayerId slot, bool on)> f) { actingHook_ = std::move(f); }
    fnvcoop::PlayerId actingSlot() const { return actionOrigin_; }   // kNoPlayer outside an executor call
    // Host, inside the action executor: the cause the guest sent with the action (DialogueResult: speaker + phase).
    fnvcoop::Cause actionCause() const { return host_ ? host_->actionCause() : fnvcoop::Cause(); }
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
    void pushWorldEffect(const std::string& key, const std::string& json);
    struct WorldStats { uint64_t pushed = 0, sent = 0, replayed = 0, received = 0, applied = 0, failed = 0, stale = 0; };
    const WorldStats& worldStats() const { return worldStats_; }
    fnvcoop_dlg::DialogueNode* dialogue() { return dlg_.get(); }  // null until the session is joined
    fnvcoop::MuxTransport* mux() { return mux_.get(); }
    // Client: ask the host to run an activation. Returns 0 (and does nothing) while syncing or not joined.
    uint32_t submitAction(fnvcoop::IntentKind kind, const fnvcoop::FormRef& ref, const fnvcoop::Cause& cause = fnvcoop::Cause());
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
    // Host: commit effects the engine glue observed outside any capture scope (a death in combat). 0 = not committed.
    uint64_t commitHostEffects(const fnvcoop::Cause& cause, std::vector<fnvcoop::Effect> effects) {
        return host_ && running_ ? host_->commit(fnvcoop::kHostPlayer, cause, std::move(effects)) : 0;
    }
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
    QuestRunningReader questRunning_;
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
