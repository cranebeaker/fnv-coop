// Guest side of the intents (FULL-SYNC-PLAN 3.7). Game-free: the engine and the network are injected.
//
// The module's observers call onActivate / onEvent from inside the game's own Activate / MarkScriptEvent detours. This class decides (policy)
// whether the interaction is forwarded and whether the local engine call goes on, sends the intent, and acts on the host's verdict: close a
// container menu the host refused, tell the player an item is gone, open the player's own storage instead of the shared one, release the
// container lock once the menu is closed. Nothing is ever applied locally from an intent: effects come back as ordinary host txns.
#pragma once
#include <deque>
#include <functional>
#include <map>
#include <string>
#include <vector>

#include "intents/intent_engine.h"
#include "intents/intent_policy.h"
#include "intents/intent_wire.h"

namespace nvcoop::intents {

struct ClientDeps {
    IIntentEngine* eng = nullptr;
    std::function<bool(Bytes data)> sendToHost;           // reliable
    std::function<bool()> ready;                          // joined and not in the baseline transfer
    std::function<uint32_t()> session;                    // the story session id; a new value = a new join = ids restart
    std::function<uint64_t()> nowMs;
    std::function<void(const FormRef& npc)> startDialogue;   // 0xD1 dialogue adapter (optional)
    std::function<bool()> containerMenuOpen;              // is the container / loot menu on screen right now (optional)
    std::function<void(const std::string& line)> log;     // optional: one line per activation the guest observed (what it decided and whether the local one goes on)
};

enum class Obs { Proceed, Suppress };

struct ClientStats {
    uint64_t observedActivate = 0, observedEvent = 0, forwarded = 0, suppressed = 0, proceeded = 0, coalesced = 0, notReady = 0, pendingCapped = 0;
    uint64_t sendFailed = 0, verdicts = 0, unknownVerdicts = 0, timeouts = 0, badDatagrams = 0, dialogueStarts = 0, redirectsOpened = 0, closesSent = 0;
    uint64_t observedStage = 0, stageOff = 0, stageBad = 0;   // SetStage: blocked advances seen by the hook / not forwarded (switched off, or no stage to ask for)
    uint64_t byKind[static_cast<size_t>(Kind::Count_)] = {};
    uint64_t rejected[static_cast<size_t>(Result::Count_)] = {};
};

class IntentClient {
public:
    IntentClient(ClientDeps deps, Config cfg);

    Obs onActivate(const FormRef& target);                   // nvhooks::setActivateObserver
    Obs onEvent(uint32_t mask, const FormRef& target);       // nvhooks::setEventObserver
    // nvhooks::setStageRequestObserver: the guest's SetStage detour dropped a script / console stage advance. true = it was sent to the host as
    // Kind::SetStage (the host decides); false = it was not (intents off, not joined yet, too many in flight, or a double of one just sent).
    bool onStageBlocked(const FormRef& quest, uint32_t stage);
    // Send an intent (terminal entry, RPC, tests). 0 = not sent (not ready, send failed, too many in flight); else the intent id.
    uint32_t submit(Kind kind, const FormRef& target, const FormRef* aux = nullptr, int32_t value = 0, std::vector<uint8_t> path = {});

    void onDatagram(const uint8_t* data, size_t size);       // verdicts from the host
    void tick();                                             // timeouts, container-menu watcher

    size_t pending() const { return pending_.size(); }
    const FormRef* openContainer() const { return watching_ ? &watched_ : nullptr; }
    const ClientStats& stats() const { return stats_; }
    std::string statusJson() const;

private:
    struct Pending { Intent in; uint64_t sentMs = 0; };
    void syncSession();
    void onVerdict(const Verdict& v, const Pending& p);
    Obs finish(const GuestDecision& d, const FormRef& target);
    uint64_t now() const { return deps_.nowMs ? deps_.nowMs() : 0; }

    ClientDeps deps_;
    Config cfg_;
    uint32_t session_ = 0, nextId_ = 1;
    std::map<uint32_t, Pending> pending_;
    std::map<std::pair<FormRef, uint32_t>, uint64_t> recentStage_;   // (quest, stage) -> last forwarded: a GameMode script that retries every frame
    std::map<std::pair<Kind, FormRef>, uint64_t> recent_;   // (kind, target) -> last forwarded, for double-click coalescing
    bool inObserver_ = false;       // our own openContainer call must not be forwarded again
    // container-menu watcher: after an accepted ContainerOpen, send ContainerClose when the menu goes away
    bool watching_ = false, sawOpen_ = false;
    FormRef watched_;
    uint64_t watchSince_ = 0;
    ClientStats stats_;
};

}  // namespace nvcoop::intents
