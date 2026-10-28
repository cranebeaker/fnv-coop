// L11 hooks_ui: game-free core. Decides WHERE a host-run script's UI goes and what the script sees afterwards.
//
//  * OriginContext   who is "the player" for the script that is running right now (acting scope > dialogue initiator >
//                    recent activation of the script's object > sticky per script > this machine's slot).
//  * UiRouter        message boxes (ShowMessage + GetButtonPressed), service menus (barter/repair/recipe/caravan/gambling),
//                    GetQuestVariable condition views, forced dialogue (Say/SayTo/StartConversation), ending slideshow.
//
// Nothing here touches the game: engine access goes through IUiEngine, the network/dialogue side through Bridge. The x86 glue
// (ui_hooks.cpp) implements IUiEngine; L10 fills Bridge from fnvcoop_dlg::DialogueNode. Mutation twins: build with
// FNVCOOP_MUT=<n> at run time (see tests/test_ui.cpp for the list); every one of them must fail the tests.
#pragma once
#include <cstdint>
#include <functional>
#include <map>
#include <optional>
#include <set>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "fnvcoop/types.h"
#include "fnvcoop_dlg/messages.h"
#include "showorder.h"

// One out-of-line definition (ui_core.cpp) so header-only code and every TU agree: the twins are only live in the *_rt test libs.
namespace nvui { bool mut(int n); }

namespace nvui {

using fnvcoop::FormRef;
using fnvcoop::PlayerId;
using fnvcoop_dlg::MenuKind;

constexpr PlayerId kHost = fnvcoop::kHostPlayer;
constexpr PlayerId kNoSlot = fnvcoop::kNoPlayer;
constexpr uint32_t kPlayerRefID = 0x14;
constexpr int32_t kNoButton = 0xFF;  // value of ShowMessageBox_button while nothing was pressed (GetButtonPressed then yields -1)

// mut(n) is declared above (showorder.h) and defined in ui_core.cpp

// ------------------------------------------------------------------ origin
class OriginContext {
public:
    enum class Kind : uint8_t { Acting = 0, Dialogue = 1 };
    struct Ttl {
        uint64_t activationMs = 5000;  // OnActivate scripts may run a frame after Activate returned
        uint64_t stickyMs = 30000;     // follow-up box of the same script (a menu chain)
    };

    void setLocalSlot(PlayerId s) { local_ = s; }
    PlayerId localSlot() const { return local_; }
    void setTtl(const Ttl& t) { ttl_ = t; }

    // Scope stack. The top-most scope wins, so a dialogue result script that runs inside an acting scope still belongs to
    // the dialogue initiator.
    void push(Kind k, PlayerId slot) { scopes_.push_back({k, slot}); }
    void pop(Kind k);
    bool scoped() const { return !scopes_.empty(); }

    // Host-side Activate(target, activator): remembered so that a script that runs a moment later (OnActivate is not always
    // executed inside Activate) still resolves to the activator's slot.
    void noteActivation(uint32_t targetRefID, PlayerId slot, uint64_t nowMs);
    // A box was raised for `script` on behalf of `slot`: its next box follows the same player.
    void noteScript(uint32_t scriptKey, PlayerId slot, uint64_t nowMs);

    PlayerId resolve(uint32_t thisObjRefID, uint32_t scriptKey, uint64_t nowMs) const;

private:
    struct Scope {
        Kind kind;
        PlayerId slot;
    };
    struct Stamp {
        PlayerId slot;
        uint64_t atMs;
    };
    PlayerId local_ = kHost;
    Ttl ttl_;
    std::vector<Scope> scopes_;
    std::unordered_map<uint32_t, Stamp> activations_, scripts_;
};

class OriginScope {
public:
    OriginScope(OriginContext& c, OriginContext::Kind k, PlayerId slot) : c_(c), k_(k) { c_.push(k, slot); }
    ~OriginScope() { c_.pop(k_); }
    OriginScope(const OriginScope&) = delete;
    OriginScope& operator=(const OriginScope&) = delete;

private:
    OriginContext& c_;
    OriginContext::Kind k_;
};

// ------------------------------------------------------------------ engine + network seams
struct IUiEngine {
    virtual ~IUiEngine() = default;
    virtual uint64_t nowMs() = 0;
    // What the vanilla ShowMessage callback does: ShowMessageBox_pScriptRefID = scriptRefID, ShowMessageBox_button = button.
    virtual void setMessageBoxResult(uint32_t scriptRefID, int32_t button) = 0;
    virtual bool refOf(uint32_t refID, FormRef& out) = 0;  // runtime refID -> (plugin, local id)
    virtual void log(const std::string& s) = 0;
    // G07 "mirror": a box that was raised to a remote player is shown here as a read-only copy, so the engine really is in message-menu
    // mode and the script's `begin MenuMode 1001` block runs on this machine. closeMirror removes it once the origin answered.
    virtual void openMirror(uint32_t /*menu*/, const std::string& /*text*/, uint8_t /*origin*/) {}
    virtual void closeMirror(uint32_t /*menu*/) {}
};

struct ConsentRule {  // irreversible message box (VEndGameWarningMsg*, Fort/Moore/YesMan confirms, nuke targets, VMS42 charges)
    int32_t confirmButton = 0;
    int32_t declineChoice = -1;
    uint32_t timeoutMs = 0;
    uint8_t mode = 0;          // 0 unanimous, 1 vote (host breaks ties), 2 actor (nobody is asked), 3 rules (the rules engine counts): HostDialogue::ConsentMode
    bool anyButton = false;    // every button except declineChoice is the irreversible one
    FormRef choice;            // mode 3: the box the rules engine's ChoiceConsent rule names (RulesEngine::onChoiceRequested); declineChoice -1 = the last button
};

struct ForcedSayArgs {
    FormRef npc, topic, info;
    PlayerId target = kNoSlot;
    bool scene = false;
    std::vector<fnvcoop_dlg::Line> lines;
};

struct Bridge {
    // --- who am I / who are they
    std::function<PlayerId(uint32_t actorRefID)> slotOfActor;  // runtime ref (player 0x14 / a guest ghost) -> slot, kNoSlot if not a player

    // --- menus (host side). Return the menu id; 0 means "already resumed" (origin gone) and onMenuResume ran synchronously.
    std::function<uint32_t(PlayerId origin, MenuKind kind, const std::string& text, std::vector<std::string> buttons,
                           int32_t defaultChoice, const FormRef& subject)>
        raiseMenu;
    std::function<uint32_t(PlayerId origin, const std::string& text, std::vector<std::string> buttons, int32_t defaultChoice,
                           const ConsentRule& rule)>
        raiseModal;
    std::function<std::optional<ConsentRule>(const FormRef& message)> consentFor;  // nullopt = plain box

    // --- condition views
    std::function<bool(uint32_t questRefID)> watchesQuest;  // cheap pre-filter (hot path: conditions are evaluated every frame)
    std::function<int32_t(const FormRef& quest, uint32_t idx, PlayerId slot, int32_t real)> viewVar;
    // wave 1b (G24): the rules engine's per-player quest variables (PerPlayerVars / PerPlayerState; the values may be floats, hence double). Applied after viewVar to the
    // evaluating player's slot. hasPlayerVars is asked ONCE per quest (the answer is cached by UiRouter until invalidateViews()), so it may be slow-ish.
    std::function<bool(const FormRef& quest)> hasPlayerVars;
    std::function<double(const FormRef& quest, uint32_t idx, PlayerId slot, double real)> viewPlayerVar;

    // --- forced dialogue (host)
    std::function<void(const ForcedSayArgs&)> forcedSay;
    std::function<PlayerId(const FormRef& npc, const FormRef& topic, PlayerId target)> forceConversation;  // returns the actor slot

    // --- ending slideshow (host capture -> everybody)
    std::function<void(const FormRef& ref, const std::string& oldTex, const std::string& newTex)> slideSwap;
    // The host's ShowClock (shared with the slide host): every scene line gets the next stamp so receivers can order narration against
    // slides (showorder.h). Absent = lines are sent unstamped.
    std::function<ShowStamp()> nextStamp;
};

// ------------------------------------------------------------------ routing
enum class Route : uint8_t { Local, Suppressed };

struct MessageBoxCall {
    uint32_t scriptRefID = 0;  // what the vanilla command stored in ShowMessageBox_pScriptRefID
    uint32_t thisObj = 0;      // calling reference (refID), 0 = none (quest script)
    FormRef message;           // the MESG form (empty if unknown)
    FormRef script;            // the running script (SCPT form): does it have a `begin MenuMode 1001` block that needs the box open?
    std::string text;
    std::vector<std::string> buttons;
};

struct ServiceMenuCall {
    MenuKind kind = MenuKind::Other;
    uint32_t thisObj = 0;  // the NPC / terminal the menu belongs to
    uint32_t scriptKey = 0;
    FormRef subject;
};

struct ForcedCall {
    enum class Kind : uint8_t { Say, SayTo, StartConversation } kind = Kind::Say;
    uint32_t speaker = 0;  // calling reference
    uint32_t target = 0;   // SayTo / StartConversation target (0 for Say)
    FormRef topic;
    FormRef info;          // INFO the engine chose (captured through GetTopicInfo)
    std::vector<fnvcoop_dlg::Line> lines;
    bool scene = false;
};
struct ForcedOutcome {
    bool captured = false;     // forwarded to L8 (host/party-visible)
    bool vetoNative = false;   // StartConversation: do not open the DialogMenu on this machine
    PlayerId target = kNoSlot;
};

class UiRouter {
public:
    UiRouter(IUiEngine& eng, Bridge bridge) : eng_(eng), br_(std::move(bridge)) {}

    OriginContext& origin() { return origin_; }
    Bridge& bridge() { return br_; }
    void setHost(bool h) { host_ = h; }
    bool isHost() const { return host_; }

    // ---- message boxes ----------------------------------------------------------------------------------------------
    // Host, inside Cmd_ShowMessage, at ShowMessageBox. Suppressed = the box was NOT opened here; the script resumes through
    // onMenuResume() (exactly once) which writes the origin's choice into ShowMessageBox_button.
    Route onMessageBox(const MessageBoxCall& c);
    // L8 HostDialogue::MenuResume.
    void onMenuResume(uint32_t menu, int32_t choice, bool timedOut);
    size_t pendingBoxes() const { return pending_.size(); }
    // G07. Off: a box raised to a remote player leaves the script's MenuMode block un-run (logged once per script). Mirror: the host shows a
    // read-only copy of that box until the origin answered, so the block runs. `hasBlock(script)` says which scripts have one.
    enum class MenuModeMode : uint8_t { Off = 0, Mirror = 1 };
    void setMenuMode(MenuModeMode m, std::function<bool(const FormRef&)> hasBlock) { mmMode_ = m; mmHasBlock_ = std::move(hasBlock); }
    MenuModeMode menuModeMode() const { return mmMode_; }
    size_t mirrors() const { return mirrors_.size(); }

    // ---- service menus ----------------------------------------------------------------------------------------------
    // Host, inside ShowBarterMenu & co. Suppressed = raised to the origin instead; the script continues immediately.
    Route onServiceMenu(const ServiceMenuCall& c);

    // ---- GetQuestVariable / GetScriptVariable conditions ----------------------------------------------------------------
    // Returns the value the evaluating player sees. `real` is what the engine computed.
    double viewQuestVar(uint32_t questRefID, uint32_t idx, double real);
    // The per-quest answer of Bridge::hasPlayerVars is cached (conditions are evaluated every frame); call this when the rules were reloaded.
    void invalidateViews() { playerVarQuests_.clear(); }

    // ---- forced dialogue ------------------------------------------------------------------------------------------------
    // Called after the engine ran the command on the host (Say/SayTo) or before it (StartConversation: the veto decision).
    ForcedOutcome onForced(const ForcedCall& c);
    // Who a script's "player" argument means. The host's own ref (0x14) inside a script that runs for some acting player /
    // dialogue initiator means THAT player; outside any scope (quest timers, scenes) it means the whole party.
    struct TargetInfo {
        bool player = false;         // the target is a player (host ref or a guest ghost)
        bool party = false;          // ... and it stands for the whole party (scene)
        PlayerId slot = kNoSlot;     // the player, when not party
    };
    TargetInfo classifyTarget(uint32_t refID, uint32_t speaker) const;

    // ---- ending slideshow -------------------------------------------------------------------------------------------------
    struct Slide {
        uint32_t seq = 0;
        FormRef ref;
        std::string oldTex, newTex;
    };
    // Host: SwapTexture ran in a script. Returns the recorded slide (also forwarded through bridge.slideSwap).
    void onSwapTexture(uint32_t refID, const std::string& oldTex, const std::string& newTex);
    const std::vector<Slide>& slides() const { return slides_; }
    void clearSlides() { slides_.clear(); }

    // counters for status/telemetry
    struct Stats {
        uint64_t boxesRouted = 0, boxesLocal = 0, resumes = 0, dupResumes = 0, serviceRouted = 0, serviceLocal = 0;
        uint64_t condViews = 0, condChanged = 0, forced = 0, forcedIgnored = 0, vetoes = 0, slides = 0;
        uint64_t mirrorsOpened = 0, mirrorsClosed = 0, menuModeSkipped = 0, hostConsent = 0;
    };
    const Stats& stats() const { return stats_; }

private:
    struct Pending {
        uint32_t scriptRefID = 0;
        int32_t buttons = 0;
    };
    void finishBox(const Pending& p, int32_t choice);
    void closeMirror(uint32_t menu);

    IUiEngine& eng_;
    Bridge br_;
    OriginContext origin_;
    bool host_ = false;
    std::map<uint32_t, Pending> pending_;
    // raiseMenu may resume synchronously (before it returns the id): remember a resume that arrived while we were raising
    bool raising_ = false;
    std::optional<std::pair<uint32_t, int32_t>> early_;
    std::vector<Slide> slides_;
    uint32_t slideSeq_ = 0;
    MenuModeMode mmMode_ = MenuModeMode::Off;
    std::function<bool(const FormRef&)> mmHasBlock_;
    std::set<uint32_t> mirrors_;
    std::set<FormRef> mmWarned_;
    std::map<uint32_t, bool> playerVarQuests_;   // questRefID -> Bridge::hasPlayerVars (cached)
    Stats stats_;
};

// Reading time helper shared with tests: default choice for a box when the bridge has no better idea. The LAST button is the
// conventional "No / Cancel / Not yet", so a timeout never commits a point of no return.
int32_t defaultChoiceFor(size_t buttonCount);

}  // namespace nvui
