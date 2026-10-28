// Quest rules engine (host side, game-free).
//
// Input: docs/quest-policies/rules.json (L9; kinds documented in tools/coverage/rules.py), keyed by quest EDID.
// Output: the session-level effects synccore already has (BarrierArrive/Release, PartyTether, SetPlayerControls,
// SaveSuppress, recipient rewrites for rewards / quest items / hand-ins / karma-rep / companion ownership).
//
// How the host plugin uses it
//   RulesEngine eng(host, db, resolver);              // resolver maps quest/cell EDIDs to FormRefs
//   eng.applyStage = [](quest, stage){ /* real SetStage on the host, capture open */ };
//   host.onAction  = [&](slot, kind, ref, ...){ eng.handleAction(slot, kind, ref); ... };   // ScenePass / ReportCell
//   hook for "a quest wants to advance": if (eng.onStageRequested(q, stage, initiator) == Deferred) skip the engine call now;
//       the engine calls applyStage(q, stage) when the barrier / tether releases.
//   every host frame: eng.tick(nowMs).
// Rule kinds handled: Barrier, PartyTether, SetPlayerControls (scope all + timeout + SaveSuppress tie), SaveSuppress,
// RewardAll, QuestItemDup, RemoveItemPerOwner, ShareKarmaRep, CompanionOwner, SlideshowEnd (host once), PartyAwareCond
// (reduceCondition), ExcludeFromGate (excludedFromGate), PerPlayerScene (effects go to the acting player; run=every-machine +
// scope=local-machine makes the quest guest-local), PerPlayerVars (per-slot copies of listed quest vars, host side), ExclusiveStages
// (first committed outcome wins), ReadyCheck (party consent before an INFO's end script), wait/sleep consent (onTimeSkipRequested),
// per-stage `stages` params, PartyTether anchor+radius, max/sum-over-party reductions, the slideshow control hold
// (Barrier before=slideshow / SlideshowEnd lock=all). Kinds with no session effect (ShareMapMarkers, LocalQuestScript...) are loaded,
// counted, and reported via unsupportedKinds().
// Campaign fix wave 1 adds: cause -> quest resolution (a quest's own script / an INFO / a ref maps back to the rules quest), ChoiceConsent (party vote on
// an irreversible fork), PerkRecipient (who gets a perk), FriendlyFireGuard, PerPlayerState + the acting-player copy API, owner=party companions,
// SetPlayerControls scope=initiator with per-source bits, the `host` reduction, tether worldspaces / auto-warp / timeouts, the guest stage-intent pass,
// death / late-join policy (the optional top-level "policy" object of rules.json, overridable per setting through Options::policy).
#pragma once
#include <functional>
#include <map>
#include <optional>
#include <set>
#include <string>
#include <vector>
#include "fnvcoop/session.h"

namespace fnvcoop {

constexpr uint32_t kAllControls = 0xFFFFFFFFu;   // SetPlayerControls mask meaning "every control" (unknown bits are ignored by the applier)

struct Rule {
    std::string kind;
    std::string origin;                                             // "curated" | "derived"
    std::map<std::string, std::string> scalars;                     // params with scalar values (as text)
    std::map<std::string, std::vector<std::string>> lists;          // params with array values (cells, markers, masks)
    std::map<std::string, std::string> funcs;                       // PartyAwareCond: function -> reduction
    // Per-stage params: `stages` [n, ...] limits the rule to those stages (absent / empty = every stage). Validated numeric at load.
    bool appliesToStage(uint32_t stage) const;
    std::string scalar(const std::string& key, const std::string& dflt = std::string()) const {
        auto it = scalars.find(key);
        return it == scalars.end() ? dflt : it->second;
    }
    double number(const std::string& key, double dflt) const;
};

struct QuestRules {
    std::string edid, tier, questline;
    bool curated = false;
    std::vector<Rule> rules;
    const Rule* find(const std::string& kind) const {
        for (auto& r : rules) if (r.kind == kind) return &r;
        return nullptr;
    }
    // First rule of `kind` that applies to `stage` (a rule without a `stages` list applies to every stage).
    const Rule* findFor(const std::string& kind, uint32_t stage) const {
        for (auto& r : rules) if (r.kind == kind && r.appliesToStage(stage)) return &r;
        return nullptr;
    }
};

class RulesDb {
public:
    // Strict: malformed JSON, wrong schema or a rule without a kind fails the whole load (nothing is kept). Known kinds are validated:
    // a PartyTether needs a non-empty `cells` list or `anchor`+`radius`; `stages` lists must be numbers; ExclusiveStages needs >= 2 stages;
    // ReadyCheck needs `infos`; PerPlayerVars needs `vars`; Barrier.before may only be `slideshow`.
    bool load(const std::string& jsonText, std::string* err = nullptr);
    bool loadFile(const std::string& path, std::string* err = nullptr);
    const QuestRules* find(const std::string& edid) const;
    uint64_t textHash() const { return textHash_; }   // hash of the exact rules.json text that was loaded (handshake)
    // Optional top-level "policy": {"death":{"mode":"downed"},...}: global settings with no quest (keys are "section.key", values as text).
    std::string policy(const std::string& key, const std::string& dflt = std::string()) const {
        auto it = policy_.find(key);
        return it == policy_.end() ? dflt : it->second;
    }
    const std::map<std::string, std::string>& policies() const { return policy_; }
    const std::map<std::string, QuestRules>& quests() const { return quests_; }
    std::map<std::string, size_t> kindCounts() const;
private:
    std::map<std::string, QuestRules> quests_;
    std::map<std::string, std::string> policy_;
    uint64_t textHash_ = 0;
};

enum class StageDecision { Proceed, Deferred };
enum class CondPhase { Entry, Exit };
struct PartyValue { PlayerId slot = 0; double value = 0; };

class RulesEngine {
public:
    struct Options {
        uint32_t barrierTimeoutMs = 120000, controlTimeoutMs = 120000;
        // PartyWarp: maps a slot to its player ref (host player = slot 0). Without it PartyWarp rules are inactive.
        std::function<bool(PlayerId slot, FormRef& playerRef)> playerRef;
        float warpSpacing = 64.f;   // slot offset along x between warped party members
        uint32_t slideshowHoldMs = 600000;   // failsafe for the control lock held until SlideshowEnd
        uint32_t consentTimeoutMs = 60000;   // wait / sleep consent window
        // Per-setting overrides of the rules.json "policy" object ("death.mode" -> "respawn"); the plugin fills it from nvcoop.ini [rules]. Empty = the file decides.
        std::map<std::string, std::string> policy;
        // Items a slot holds right now (the plugin answers from the party state provider, GetItemCount of that player). PooledItems splits a hand-in over the party with it;
        // without it a pooled item is charged to whoever handed it in (the old behaviour).
        std::function<bool(PlayerId slot, const FormRef& item, int64_t& count)> itemCount;
        // Distance between two players in game units (the plugin answers from the avatar positions). PartyWarp `range` moves only the party members within it of the firer
        // (the Old World Blues Teleporter); a slot whose distance is unknown stays where it is.
        std::function<bool(PlayerId a, PlayerId b, float& dist)> playerDistance;
    };
    struct Stats {
        uint64_t stagesDeferred = 0, stagesReleased = 0, barrierTimeouts = 0, arrivals = 0, tetherGathers = 0, controlTimeouts = 0,
                 recipientRewrites = 0, locksTracked = 0, unknownReductions = 0, ignoredPasses = 0, dialogueRejected = 0,
                 warpsExpanded = 0, travelLockChanges = 0, guestLocalDropped = 0, exclusiveRejected = 0, readyChecks = 0, readyTimeouts = 0,
                 slideshowHolds = 0, perPlayerRouted = 0, timeSkipsAsked = 0, timeSkipsGranted = 0, timeSkipsDenied = 0,
                 causesResolved = 0, choicesAsked = 0, choicesAccepted = 0, choicesCancelled = 0, perkRoutes = 0, friendlyFireBlocked = 0, friendlyFireAllowed = 0,
                 passIntents = 0, tetherAutoWarps = 0, playersDowned = 0, respawns = 0, saveReloads = 0, lateJoinHolds = 0, lateJoinWarps = 0, killsRerouted = 0,
                 controlsHeldBack = 0, actingRuns = 0, tetherWarnings = 0, tethersIgnored = 0,
                 checkpointDowns = 0, checkpointRevives = 0, pooledDebits = 0, onceEventsDropped = 0, rangeWarpsHeld = 0;
    };
    using Resolver = std::function<bool(const std::string& edid, FormRef& out)>;

    RulesEngine(HostSession& host, const RulesDb& db, Resolver resolve);
    RulesEngine(HostSession& host, const RulesDb& db, Resolver resolve, Options opt);
    ~RulesEngine();
    RulesEngine(const RulesEngine&) = delete;
    RulesEngine& operator=(const RulesEngine&) = delete;

    // The host plugin performs the real SetStage when a deferral releases (it runs the stage's result scripts, captured as usual).
    std::function<void(const FormRef& quest, uint32_t stage)> applyStage;

    // A quest wants to advance to `stage` on the host. Deferred => do not perform it now; applyStage fires on release.
    StageDecision onStageRequested(const FormRef& quest, uint32_t stage, PlayerId initiator);
    // ExclusiveStages, asked BEFORE anything runs: true when `stage` is an alternative outcome and a different one of the same set was committed first (or is the
    // quest's present stage). filter() would drop that SetStage from the txn anyway, but only after its result scripts ran on the host; a guest's stage
    // request (plugin/intents Kind::SetStage) is refused up front with this. Read-only.
    bool stageExcluded(const FormRef& quest, uint32_t stage) const;
    // A player finished their per-player step of `quest` (chargen etc.). Slot 0 = the host player.
    void onPlayerPassed(PlayerId slot, const FormRef& quest);
    // A player is now in `cell` (clients report via IntentKind::ReportCell; the host reports itself). `world` is the worldspace the cell belongs to
    // (empty for an interior): exterior tethers name the worldspace (HooverDam, NVDLC04Road02World), not the grid cell, so either one satisfies them.
    void onCellReported(PlayerId slot, const FormRef& cell, const FormRef& world = FormRef());
    // The worldspace alone changed (the grid cell did not matter): same as onCellReported with the previous cell kept.
    void onWorldReported(PlayerId slot, const FormRef& world);
    // A PartyTether with gather=auto-warp ran out of time (`timeout_s` of the rule, else barrierTimeoutMs): the plugin warps these slots to `target` (the
    // tether's first cell or its anchor). The tether is released right after, as for a plain timeout.
    std::function<void(const FormRef& quest, uint32_t stage, const std::vector<PlayerId>& absent, const FormRef& target)> gatherRest;
    // Policy "tether.mode" (nvcoop.ini [rules] tether=on|warn|off, default on): on = the PartyTether rules hold the stage and may auto-warp (above); warn = a tether never holds
    // or warps anybody, `tetherWarn` is called once per stage request with the players outside the scene zone (the plugin tells each of them); off = PartyTether rules are
    // ignored (no hold, no warning, no zone checkpoint).
    std::function<void(const FormRef& quest, uint32_t stage, const std::vector<PlayerId>& outside, const FormRef& zone)> tetherWarn;
    // A player is now within (near=true) / outside the radius of a PartyTether `anchor` ref (IntentKind::ReportNear / ReportFar).
    void onNearReported(PlayerId slot, const FormRef& anchor, bool isNear);
    // ReadyCheck: the INFO `info` of `quest` wants to run its end script (irreversible step). Deferred => do not run it now; every joined
    // player must confirm (IntentKind::ScenePass with ref = info). onReadyResolved(quest, info, proceed) fires on release: proceed=true when
    // everyone confirmed (or on_timeout=proceed), false = cancelled.
    StageDecision onReadyRequested(const FormRef& quest, const FormRef& info, PlayerId initiator);
    std::function<void(const FormRef& quest, const FormRef& info, bool proceed)> onReadyResolved;
    // Wait / sleep consent: the time skip only happens when every joined player agrees (ScenePass with ref = timeSkipId()), otherwise it is
    // cancelled at the deadline. applyTimeSkip(minutes) fires on grant (the plugin performs it and captures AdvanceTime as usual).
    StageDecision onTimeSkipRequested(PlayerId initiator, uint32_t minutes);
    std::function<void(uint32_t minutes)> applyTimeSkip;
    std::function<void(uint32_t minutes)> timeSkipDenied;
    static FormRef timeSkipId();
    // PerPlayerVars: per-slot copies of quest vars listed by a PerPlayerVars rule. The host plugin loads the acting player's copy into the
    // quest var before running a script / INFO for that player and stores the result back afterwards. Host side only (clients never run
    // quest scripts); saveVars / loadVars persist it with the host save.
    bool isPerPlayerVar(const FormRef& quest, const std::string& var) const;
    void writePlayerVar(const FormRef& quest, const std::string& var, PlayerId slot, double value);
    bool readPlayerVar(const FormRef& quest, const std::string& var, PlayerId slot, double& out) const;
    std::vector<std::pair<std::string, double>> playerVarsOf(const FormRef& quest, PlayerId slot) const;
    // ---- Acting-player copies of per-player state (G24 / G42) ------------------------------------------------------------------------------------------------
    // The vars of a PerPlayerVars / PerPlayerState rule live in ONE engine variable each; the engine holds one copy per slot. The plugin brackets every script / INFO /
    // timer run it does on behalf of a player: beginActing(quest, slot, live) returns the values to WRITE into the engine variables (the slot own copy; a slot seen
    // for the first time gets the seed = the values the engine held when the quest was first seen), endActing(quest, slot, live) stores what the run left behind.
    // While acting, committed txns of the quest (and txns whose cause maps to it) are attributed to `slot`, so their player-targeted effects reach that player.
    struct VarValue { std::string var; double value = 0; };
    std::vector<std::string> perPlayerVarNames(const FormRef& quest) const;
    std::vector<VarValue> beginActing(const FormRef& quest, PlayerId slot, const std::vector<VarValue>& live);
    void endActing(const FormRef& quest, PlayerId slot, const std::vector<VarValue>& live);
    PlayerId actingSlot() const { return acting_; }
    // What a condition evaluated for `slot` should read (a spectator view): the slot copy, else the seed, else `real`.
    double viewPlayerVar(const FormRef& quest, const std::string& var, PlayerId slot, double real) const;

    // Host-side rules state worth keeping across a host restart: the per-slot var copies and the ExclusiveStages winners.
    Bytes saveState() const;
    bool loadState(const Bytes& b);
    // Routes ScenePass / ReportCell / ReportNear / ReportFar action requests from HostSession::onAction. True if consumed.
    bool handleAction(PlayerId slot, IntentKind kind, const FormRef& ref);
    void tick(uint64_t nowMs);

    // PartyAwareCond: reduce one condition function over the party. Reductions: initiator, any-party-member, any,
    // min-over-party, max-over-party, sum-over-party (collection quests pool what the party carries), "entry:all-in,exit:any-in".
    // Unknown / missing => initiator (counted).
    double reduceCondition(const FormRef& quest, const std::string& func, const std::vector<PartyValue>& values,
                           PlayerId initiator, CondPhase phase = CondPhase::Entry);

    // InitiatorDialogue: while a dialogue is running only its initiator's result txns (cause DialogueResult) of a rule-bearing quest are
    // committed; other players' attempts are dropped. Everyone still receives the initiator's results (broadcast).
    void onDialogueStart(PlayerId initiator) { dialogueInitiator_ = initiator; }
    void onDialogueEnd() { dialogueInitiator_ = kNoPlayer; }

    // GuestLocalRun (allow-list): quest scripts that run locally on EVERY machine and are never captured / replicated. L5 feeds this into
    // SuppressPolicy::skipScript (guests must NOT suppress these scripts) and the host drops any txn they produce.
    bool guestLocalRun(const FormRef& quest) const;
    std::vector<FormRef> guestLocalQuests() const;
    // Script EDIDs named by GuestLocalRun rules (param "scripts"), for object/effect scripts that are not quest scripts.
    std::vector<std::string> guestLocalScripts() const;

    // ---- Cause -> quest (G02) ------------------------------------------------------------------------------------------------------------------------------
    // Rules are keyed by quest, but a quest own GameMode script, an object script that drives a quest and a guest intent commit txns whose cause.source is the
    // SCRIPT form (or the target ref). The map script / INFO / ref -> quest (docs/quest-policies/cause_quests.json, generated by tools/questsweep/gen_cause_quests.py)
    // lets the engine rewrite cause.source to the owning quest before any rule is looked up. A source that already is a quest with rules is left alone.
    void setCauseQuests(std::map<FormRef, FormRef> m) { causeQuests_ = std::move(m); }
    // JSON {"schema":1,"scripts":{"Plugin.esm:HEX":"QuestEdid"},"infos":{...},"refs":{...},"keys":{"RulesEntryEdid":"Plugin.esm:HEX"}}; the quest EDIDs go through the
    // resolver. "keys" binds rules entries that are no quest (a trigger script with its own SetPlayerControls scope, shared by the scripts that map to it) to a form, and
    // those names can be used as the quest of an entry. False (nothing kept) on bad input.
    bool loadCauseQuests(const std::string& json, std::string* err = nullptr);
    bool questOfCause(const Cause& c, FormRef& quest) const;
    size_t causeQuestCount() const { return causeQuests_.size(); }

    // ---- ChoiceConsent (G08) ---------------------------------------------------------------------------------------------------------------------------------
    // An irreversible fork outside the end-game boxes (Joshua or Daniel, the Elijah team-up, the Caesar surgery ...) is a ChoiceConsent rule naming the INFOs / message boxes
    // (`topics` / `boxes`) that fork the story. mode: vote (default: a strict majority of the joined players, the host vote breaks a tie), unanimous (everybody, any
    // decline cancels), host (the host decides), initiator (nobody is asked). Votes arrive as ScenePass with ref = the choice (yes) or choiceDeclineId(choice) (no); asking is
    // a yes. timeout_s (default 90): vote / host mode decide with the votes so far (the host vote, else cancel), unanimous cancels.
    enum class ChoiceMode : uint8_t { Vote, Unanimous, Host, Initiator };
    bool isChoice(const FormRef& choice) const { return choices_.count(choice) > 0; }
    bool choiceQuest(const FormRef& choice, FormRef& quest) const;
    ChoiceMode choiceMode(const FormRef& choice) const;
    StageDecision onChoiceRequested(const FormRef& choice, PlayerId initiator);
    static FormRef choiceDeclineId(const FormRef& choice);
    std::function<void(const FormRef& quest, const FormRef& choice, bool accepted)> onChoiceResolved;
    // The INFO `info` wants to run its end script: a ReadyCheck or ChoiceConsent INFO asks the party first (Deferred; onReadyResolved / onChoiceResolved fires), any other
    // INFO proceeds. Finds the quest by itself (G27): the plugin only needs the INFO.
    StageDecision onInfoResult(const FormRef& info, PlayerId initiator);
    bool readyQuest(const FormRef& info, FormRef& quest) const;

    // ---- Guest stage intents as passes (G26) --------------------------------------------------------------------------------------------------------------------
    // A guest finished its per-player step when its own game tries the shared stage (chargen end): the SetStage intent of a quest with a Barrier on that stage counts as the
    // player pass (the host own chargen end passes too: slot 0). Deferred = the stage is held until everyone passed (applyStage fires on release); Proceed = no barrier,
    // handle the intent as usual.
    StageDecision onGuestStageIntent(PlayerId slot, const FormRef& quest, uint32_t stage);
    bool isPassStage(const FormRef& quest, uint32_t stage) const;

    // ---- Companions that belong to the whole party (G34) ----------------------------------------------------------------------------------------------------------
    // A CompanionOwner rule with owner=party (optionally limited to `npcs`) marks the companion as the party own (ED-E in Lonesome Road): the effect keeps the recruiter slot
    // on the wire, but the follower caps skip it and the companion layer lets every player command it.
    bool isPartyCompanion(const FormRef& npc) const { return partyCompanions_.count(npc) > 0; }
    std::vector<FormRef> partyCompanions() const { return std::vector<FormRef>(partyCompanions_.begin(), partyCompanions_.end()); }

    // ---- FriendlyFireGuard (G40) -----------------------------------------------------------------------------------------------------------------------------
    // Named story NPCs (`actors`: EDIDs or Plugin.esm:HEX) whose death or hostility fails a line for everybody. A hit by a party member on one of them is blocked while it
    // looks accidental: splash / explosion damage, or fewer than `deliberate_hits` (3) direct hits by the same player inside `window_s` (10 s). Reaching the count, or the
    // host confirmDeliberate(), lets it through (the line fails for everyone, as in single player). The combat layer asks before it applies a hit.
    enum class FfVerdict : uint8_t { NotProtected, Allow, Block, BlockWarn };
    bool isProtectedActor(const FormRef& actor) const { return protected_.count(actor) > 0; }
    FfVerdict onProtectedHit(const FormRef& victim, PlayerId attacker, bool splash, uint64_t nowMs);
    void confirmDeliberate(const FormRef& victim, PlayerId attacker);

    // ---- Death, downed state and late joiners in story scenes (G41) -----------------------------------------------------------------------------------------------
    // Policy (rules.json "policy" or Options::policy): death.mode downed (default) | respawn | normal, death.revive_s (60: how long a downed player waits for a revive before
    // the respawn), death.all_down reload-host-save (default) | respawn, late_join.during_scene wait (default) | join, late_join.after_scene warp-to-zone (default) | stay.
    // The zone checkpoint is the last PartyTether that was satisfied (or setZone): a downed player respawns there and a late joiner is warped there.
    struct DeathVerdict { enum Kind : uint8_t { Normal, Downed, ReloadSave } kind = Normal; FormRef checkpoint; bool hasCheckpoint = false; uint32_t respawnAfterMs = 0; bool untilCheckpoint = false; };
    DeathVerdict onPlayerDied(PlayerId slot, uint64_t nowMs);
    void onPlayerRevived(PlayerId slot);
    std::function<void(PlayerId slot, const FormRef& checkpoint)> respawnPlayer;   // a downed player ran out of time: the plugin respawns it at the checkpoint
    std::function<void()> reloadHostSave;                                          // every joined player is down: the plugin reloads the host save (guests wait in the lobby)
    bool isDowned(PlayerId slot) const { return downed_.count(slot) > 0; }
    // G42 / D9: a collar death (a player.kill rerouted by a PerPlayerState rule with down=until-checkpoint) is a downed state with NO timed respawn: the player stays down
    // until the party reaches a checkpoint (a PartyTether zone becoming satisfied, setZone, onCheckpoint, or a living member reporting one of the rule's `checkpoint` cells),
    // then respawnPlayer(slot, checkpoint) fires. A death that is not a collar death keeps the timed revive.
    bool downedUntilCheckpoint(PlayerId slot) const { return holdDowned_.count(slot) > 0; }
    void onCheckpoint(const FormRef& zone) { setZone(zone); }
    void setZone(const FormRef& zone) { zone_ = zone; hasZone_ = true; releaseCheckpointDowned(); }
    bool zone(FormRef& out) const { out = zone_; return hasZone_; }
    // A scene is running: a deferral is pending, a control lock or the slideshow hold is held.
    bool sceneActive() const;
    struct JoinVerdict { enum Kind : uint8_t { JoinNow, WaitForScene, WarpToZone } kind = JoinNow; FormRef zone; };
    JoinVerdict onPlayerJoining(PlayerId slot);   // call when a slot joins; WaitForScene: ask again when sceneActive() turns false

    // ---- Per-player state declarations (G42): what a PerPlayerState rule asks the plugin to run per slot --------------------------------------------------------------
    // `scripts`: object / quest scripts (EditorIDs) whose per-frame or trigger block must run once for each joined slot under beginActing (the speakers' radius test, the
    // vault and suites timers, the kitchen traps, the cloud tick); `objvars` "Script.var": variables of those scripts' own refs that keep one copy per slot (bInRadius).
    // The quest variables stay in `vars` (perPlayerVarNames). Names only: the plugin owns the reading and writing.
    std::vector<std::string> perPlayerScripts() const;
    bool runsPerPlayer(const std::string& scriptEdid) const;
    std::vector<std::string> perPlayerObjVarNames(const FormRef& quest) const;

    // The policy value in force (Options::policy first, then rules.json, then `dflt`).
    std::string policy(const std::string& key, const std::string& dflt) const;

    bool excludedFromGate(const FormRef& quest) const;
    const QuestRules* rulesFor(const FormRef& quest) const;
    size_t pendingDeferrals() const { return pending_.size(); }
    const Stats& stats() const { return stats_; }
    const std::set<std::string>& unsupportedKinds() const { return unsupported_; }
    // Barrier id used in BarrierArrive/BarrierRelease for (quest, stage).
    static FormRef barrierId(const FormRef& quest, uint32_t stage);
    // G26, the guest side: the quests of the barriers this guest should pass by itself. A barrier step (Doc's farewell, Elijah's talks, an ending) runs only in the host's
    // game, so a guest's own game never asks for the stage and its SetStage intent never comes; the guest passes once a barrier is open (someone arrived, the guest is
    // required, it has not arrived, not released). The plugin sends ScenePass for each while its player is free (no menu), so a barrier still waits for a player busy
    // in a menu, and the host no longer stalls for barrierTimeoutMs.
    static std::vector<FormRef> guestBarrierPasses(const ShadowState& s, PlayerId slot);

private:
    struct Pending {
        FormRef quest; uint32_t stage = 0; PlayerId initiator = 0;
        bool tether = false; FormRef barrier; uint64_t since = 0; std::set<PlayerId> passed; std::set<FormRef> cells;
        FormRef anchor; bool hasAnchor = false;                       // tether anchor ref (+ radius, checked by the clients' near reports)
        enum Kind : uint8_t { Stage, Ready, TimeSkip, Choice } kind = Stage;  // what is waiting
        FormRef info; uint32_t timeoutMs = 0; bool proceedOnTimeout = false; uint32_t minutes = 0;
        bool slideshowAfter = false;                                  // Barrier before=slideshow: hold the controls once released
        bool autoWarp = false;                                        // tether gather=auto-warp: warp the absent players on a timeout
        std::set<PlayerId> declined;                                  // a Choice vote: the "no" votes (passed holds the "yes" votes)
    };
    struct Lock { uint32_t mask = 0; uint64_t since = 0; };
    struct ChoiceInfo { FormRef quest; ChoiceMode mode = ChoiceMode::Vote; uint32_t timeoutMs = 90000; };
    struct FfState { std::vector<uint64_t> hits; bool warned = false; bool deliberate = false; };
    uint32_t heldByOthers(const FormRef& source, PlayerId who) const;
    std::vector<PlayerId> tetherAbsent(const std::set<FormRef>& cells, const FormRef& anchor, bool hasAnchor) const;
    bool hasLock(const FormRef& quest) const;
    bool perkRecipient(const QuestRules& q, const FormRef& perk, bool acting, Recipient& out) const;
    bool listedForm(const Rule& r, const std::string& key, const FormRef& f) const;
    int choiceDecision(const Pending& p, bool timedOut) const;
    void filter(EffectTxn& t);
    void releaseCheckpointDowned();
    bool checkpointCell(const FormRef& cell) const;
    void expandPooled(const QuestRules& q, EffectTxn& t);
    void dropRepeatedEvent(const QuestRules& q, EffectTxn& t);
    uint8_t joinedMask() const;
    bool tetherSatisfied(const std::set<FormRef>& cells, const FormRef& anchor, bool hasAnchor) const;
    void startVote(Pending p);
    bool askerIsAlone(PlayerId asker) const;
    void holdForSlideshow(const FormRef& quest);
    void release(size_t index, bool timedOut);
    void checkPending();
    bool desiredTravelLock(const FormRef& quest) const;
    void syncTravelLock(const FormRef& quest);

    HostSession& host_;
    const RulesDb& db_;
    Options opt_;
    std::map<FormRef, const QuestRules*> byForm_;
    std::map<FormRef, std::string> cellsByEdid_;
    std::vector<Pending> pending_;
    std::map<FormRef, std::set<PlayerId>> earlyPasses_;
    std::map<PlayerId, FormRef> cellOf_, worldOf_;
    std::map<PlayerId, std::set<FormRef>> nearOf_;     // anchors each slot reported being within the radius of
    std::map<FormRef, uint32_t> exclusiveWinner_;      // quest -> the ExclusiveStages outcome that was committed first
    std::set<FormRef> readyInfos_;                     // every INFO a ReadyCheck rule names (a consent for an unasked check is dropped)
    std::map<FormRef, uint64_t> slideshowHold_;        // quest -> when its control hold began (released by SlideshowEnd or the failsafe)
    std::map<FormRef, FormRef> causeQuests_;           // script / INFO / ref -> the quest whose rules apply to what it commits
    std::map<FormRef, ChoiceInfo> choices_;            // ChoiceConsent: the INFO / box -> its quest and mode
    std::map<FormRef, FormRef> readyQuests_;           // ReadyCheck INFO -> its quest
    std::set<FormRef> partyCompanions_;
    std::map<FormRef, const Rule*> protected_;         // FriendlyFireGuard actors -> their rule
    std::map<std::pair<PlayerId, FormRef>, FfState> ff_;
    std::map<PlayerId, uint64_t> downed_;              // slot -> when it went down
    std::set<PlayerId> holdDowned_;                    // downed slots that only a checkpoint revives (collar deaths)
    std::map<PlayerId, uint64_t> collarKilled_;        // slots a PerPlayerState kill reroute just drained -> when (an onPlayerDied within 30 s of it is a collar death)
    std::set<FormRef> checkpointCells_;                // cells / worldspaces a PerPlayerState `checkpoint` list names
    std::map<FormRef, std::pair<uint64_t, PlayerId>> onceEvents_;   // OncePerEvent: quest -> when its event first committed, and by whom
    FormRef zone_; bool hasZone_ = false;
    std::map<FormRef, std::map<std::string, double>> seeds_;   // quest -> per-player var -> the value the engine held when it was first seen
    std::map<FormRef, PlayerId> loadedSlot_;           // quest -> whose copy is in the engine variables now
    PlayerId acting_ = kNoPlayer; FormRef actingQuest_;
    std::map<std::tuple<FormRef, std::string, PlayerId>, double> playerVars_;
    std::map<std::pair<FormRef, PlayerId>, Lock> locks_;   // (source quest, scope: kNoPlayer = everyone, else that one player)
    std::set<FormRef> travelLocked_;
    PlayerId dialogueInitiator_ = kNoPlayer;
    std::set<std::string> unsupported_;
    Resolver resolve_;
    uint64_t now_ = 0;
    Stats stats_;
};

}  // namespace fnvcoop
