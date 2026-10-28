// Glue between the coop Runtime and the intents core (game-free: the Runtime is game-free too; the engine arrives through IIntentEngine).
// intents_module.cpp (x86) owns one IntentService, feeds it the engine, and calls frame() once per game frame.
//
//   host   : registers channel 0x8D on the session's mux, builds an IntentHost whose runAsGuest is Runtime::runAsGuest, takes over the
//            Runtime's story-channel ActionExecutor (so ActivateRef / TriggerEnter / PickUp intents of older guests get the same acting-player
//            context), releases a leaver's locks.
//   guest  : builds an IntentClient whose network is the mux (the host is peer 0) and whose readiness is "joined and not in the baseline
//            transfer"; the module wires onActivate / onEvent into L5's observers.
// A new mux (new session) re-registers everything; handlers of an old session stay inert (generation token).
#pragma once
#include <deque>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <tuple>

#include "coop_runtime.h"
#include "intents/intent_client.h"
#include "intents/intent_host.h"

namespace nvcoop::intents {

struct ServiceDeps {
    IIntentEngine* eng = nullptr;
    std::function<RefId(PlayerId)> puppetOf;           // plugin/avatar: the puppet that stands for a remote player
    std::function<RefId(PlayerId)> ghostOf;            // NV:MP ghost, optional
    std::function<void(PlayerId slot, ActorMode mode, RefId actor)> actorMode;   // host: the acting player of a slot changed (the module logs it)
    std::function<uint64_t()> nowMs;
    std::function<bool()> containerMenuOpen;           // guest: is the loot menu on screen
    std::function<void(const FormRef& npc)> startDialogue;                 // guest: 0xD1 request
    std::function<void(const FormRef& npc, PlayerId slot)> hostDialogue;   // host: optional notice for DialogueStart
    std::function<bool(PlayerId slot, const FormRef& target)> allowActivation;                  // host: plugin/companions, false = refuse (HostDeps::allowActivation)
    std::function<void(PlayerId slot, bool pickup, const FormRef& target)> interacted;          // host: plugin/companions, a guest interaction ran (HostDeps::interacted)
    std::function<bool(PlayerId slot, const FormRef& quest, uint32_t stage)> allowStage;        // host: quest policy for a guest's SetStage (HostDeps::allowStage)
    GuestStagePassFn guestStagePass;                                                           // host: a guest's SetStage on a Barrier stage is its pass (HostDeps::guestStagePass)
    std::function<void(PlayerId slot, const FormRef& quest, uint32_t stage, Result why)> stageRefused;   // host: a guest stage was refused / held / skipped (the module logs it)
    const RetryList* retryList = nullptr;               // host: the scripts the stand-in retry is allowed for (HostDeps::retryList)
    std::function<void(const std::string& line)> log;   // host: one line per refused intent and per dialogue result run / duplicate (HostDeps::log); optional
};

class IntentService {
public:
    IntentService(ServiceDeps deps, Config cfg, ContainerRedirect redirect = ContainerRedirect(), StorageBook stash = StorageBook());
    ~IntentService();

    // Once per game frame, AFTER Runtime::frame() (which polls the mux and so delivers our datagrams). null / not running = no session.
    void frame(nvcoop::coop::Runtime* rt);

    // Guest observers (the module wires them into nvhooks::setActivateObserver / setEventObserver).
    Obs onActivate(const FormRef& target);
    Obs onEvent(uint32_t mask, const FormRef& target);
    // Guest: the SetStage detour dropped a script / console stage advance (nvhooks::setStageRequestObserver). true = forwarded to the host.
    bool onStageBlocked(const FormRef& quest, uint32_t stage);
    // Guest: send an intent by hand (terminal entry hook, RPC). 0 = not sent.
    uint32_t submit(Kind kind, const FormRef& target, const FormRef* aux = nullptr, int32_t value = 0, std::vector<uint8_t> path = {});
    // The host loaded a save: claims, locks and cooldowns of the old world are meaningless.
    void onWorldLoaded();

    bool sessionUp() const { return up_; }
    bool isHost() const { return up_ && isHost_; }
    IntentHost* host() { return host_.get(); }
    IntentClient* client() { return client_.get(); }
    // The gear ledger (storage_book.h). It outlives sessions: the module saves stateJson() when dirty() and reloads it at start.
    StorageBook& stash() { return stash_; }
    const Config& config() const { return cfg_; }
    uint64_t sessions() const { return sessions_; }
    std::string statusJson() const;

    // Host: a guest's conversation reached an INFO result script (the DialogueNode's ResultRunner). Runs it through the engine with the initiator's
    // puppet as the acting player and returns the committed story seq (0 = nothing committed / refused). Installed on the Runtime at session begin
    // when [intents] dialogue_results=1, replacing coop_module's stub.
    uint64_t runDialogueResult(const fnvcoop_dlg::ResultRequest& rq);
    // deduped: a runner call for a (session, conversation, step, phase) that already ran (a retry, a resend, a second delivery): answered with the first run's
    // txn seq and NOT run again (a result script gives its rewards once per pick).
    struct DialogueStats { uint64_t ran = 0, committed = 0, unsupported = 0, failed = 0, refused = 0, asHost = 0, deduped = 0; };
    const DialogueStats& dialogueStats() const { return dlgStats_; }

private:
    void begin(nvcoop::coop::Runtime* rt, fnvcoop::MuxTransport* mux, bool isHost);
    void end(bool runtimeAlive);
    PlayerId slotOfPeer(fnvcoop::PeerId peer) const;

    ServiceDeps deps_;
    Config cfg_;
    ContainerRedirect redirect_;
    StorageBook stash_;
    nvcoop::coop::Runtime* rt_ = nullptr;
    fnvcoop::MuxTransport* mux_ = nullptr;
    bool up_ = false, isHost_ = false;
    std::shared_ptr<uint64_t> live_ = std::make_shared<uint64_t>(0);   // generation seen by mux handlers (0 = dead)
    uint64_t gen_ = 0, sessions_ = 0;
    std::unique_ptr<IntentHost> host_;
    std::unique_ptr<IntentClient> client_;
    std::set<PlayerId> joined_;
    DialogueStats dlgStats_;
    struct DlgKey {
        uint64_t session = 0;     // gen_ of the host session: conversation ids restart with the dialogue host
        uint32_t conv = 0, step = 0;
        uint8_t phase = 0;
        bool operator<(const DlgKey& o) const { return std::tie(session, conv, step, phase) < std::tie(o.session, o.conv, o.step, o.phase); }
    };
    std::map<DlgKey, uint64_t> dlgDone_;     // runs of the result runner by key -> the txn seq it returned
    std::deque<DlgKey> dlgOrder_;            // insertion order, to keep the table small
    void dlgLog(const std::string& line) { if (deps_.log) deps_.log(line); }
};

// ---- SaveSuppress enforcement (internal lane notes "SaveSuppress enforcement (L5)") --------------------------------------------------------
// The detour on the engine's CanSaveNow asks the gate. A quest that forbids saving (SaveSuppress effect, shared state) blocks the manual save
// menu, quicksave and autosave on every machine until the flag is cleared. The player gets one HUD line per 5 s, not one per refused call.
class SaveGate {
public:
    struct Deps {
        std::function<bool()> suppressed;                       // the shared flag, read from the session
        std::function<void(const std::string&)> notify;         // HUD message (optional)
        std::function<uint64_t()> nowMs;
    };
    explicit SaveGate(Deps d) : d_(std::move(d)) {}
    // `engineAllows` is what the original CanSaveNow said. Returns what the detour should return.
    bool allow(bool engineAllows);
    struct Stats { uint64_t asked = 0, blocked = 0, notices = 0; };
    const Stats& stats() const { return stats_; }

private:
    Deps d_;
    Stats stats_;
    uint64_t lastNoticeMs_ = 0;
    bool noticed_ = false;
};

// The shared SaveSuppress flag as the session sees it: host = its own world, guest = the state it applied; false without a session.
bool saveSuppressedBy(nvcoop::coop::Runtime* rt);

}  // namespace nvcoop::intents
