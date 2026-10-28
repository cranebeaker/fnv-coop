// TEST-ONLY fault switches for the dialogue mutation twins. All false in production.
// Each flag re-introduces one classic bug; the matching scenario test must then fail.
#pragma once
namespace fnvcoop_dlg {
struct Mutations {
    bool noNpcLock = false;          // a second player may start a conversation with an NPC already in use
    bool anyonePicks = false;        // host accepts Pick/LineDone/EndReq from non-initiators
    bool replayDuplicatePick = false;// a re-delivered Pick re-runs the begin script (double rewards)
    bool dropEndOnClose = false;     // closing mid-line (disconnect/NPC death) discards the pending end script
    bool leakLockOnClose = false;    // NPC lock and control lock are not released on close
    bool spectatorStaleLines = false;// client applies Lines with an old step (out-of-order / duplicate delivery)
    bool noTopicGate = false;        // initiator builds the next topic list before the end-script result arrived
    bool skipControlRelease = false; // control lock lease is not released when the conversation closes
    bool noJoinCatchUp = false;      // a player who joins mid-conversation is not told about it (stays Idle, ignores Lines/Closed)
    bool ignoreResultLink = false;   // TopicsReady fires on Result without waiting for the result txn to be applied locally
    bool noResync = false;
    // --- round 3 (companions, menus)
    bool capIsPartyWide = false;      // follower cap is the old shared "2 total" instead of per recruiting player
    bool anyoneDismisses = false;     // a non-owner may dismiss someone else's companion
    bool keepCompanionsOfLeaver = false; // companions of a departed owner stay owned by the ghost slot
    bool noOwnerRewrite = false;      // conditions read the shared aggregate (everybody sees everybody's party)
    bool nonOwnerCommands = false;    // host accepts owner-only companion topics from non-owners
    bool spectatorsSeeCommands = false; // owner-only picks/offers are still broadcast to spectators
    bool noPreempt = false;           // the owner cannot take over their companion from a non-owner conversation
    bool menuOnHost = false;          // menu requests open on the host player instead of the origin player
    bool anyoneRepliesMenu = false;   // any player may answer someone else's menu
    bool resumeMenuTwice = false;     // a duplicate MenuReply resumes the script again
    bool noMenuDefault = false;       // timeout / disconnect never resumes the waiting script
    bool anyMenuChoice = false;       // out-of-range button indices are accepted
    bool forcedStartToAll = false;    // ForcedStart (open the real menu) goes to every player instead of the target only
    bool forcedNoPreempt = false;     // a scripted conversation does not displace an existing one (it silently never starts)
    bool forcedAlwaysHost = false;    // the acting player of a party scene is always the host
    bool forcedNoDedupe = false;      // a re-delivered ForcedLines is shown again
    bool skipConsent = false;         // party-consent prompts resume the script on the origin's answer alone
    bool menuToSpectators = false;    // spectators receive the interactive MenuRequest (can answer it)           // a rejoining client keeps its stale Initiator/Spectator role
    // --- round 5 (campaign fix wave 1, P3: guest dialogue end to end)
    bool dlgReadWrongInfo = false;    // the response reader trusts the engine cache even when it holds another INFO's chain
    bool sayOnceNeverShared = false;  // watching a line never marks the INFO said (player B can repeat player A's Speech success)
    bool sayOnceAllShared = false;    // trainer / Speech-XP lines are shared too (only the first player gets them)
    bool sayOnceKeepHostMark = false; // the host keeps the said-mark its RunResultScript made for a guest's per-player line
    bool pollNoPick = false;          // the menu poller never reports the chosen INFO (the host never learns the pick)
    bool pollPickEveryFrame = false;  // the menu poller re-reports the spoken INFO every frame (duplicate picks)
    bool grantNoWatchdog = false;     // a granted conversation whose menu never opens is never given back (the NPC stays locked on the host, the guest stays the initiator)
    bool pollNoLineDone = false;      // the menu poller never reports the end of a line (the gate waits for the timeout)
    bool pollLeaveAfterGoodbye = false; // closing the menu after a finished goodbye line is also reported as the player leaving
    bool nonOwnerPicksCommand = false;// the glue lets a non-owner send an owner-only companion command
    bool noCompanionCanon = false;    // a companion known under several forms (base, placed refs) is tracked per form: ownership is lost
    bool forcedHostReplays = false;   // the host replays a scripted line its own engine already played (double subtitles and voice)
    // --- G08 choice consent (D3: party vote, the host breaks ties; points of no return need everybody)
    bool voteIsUnanimous = false;     // a party vote needs every player to agree (one "not yet" kills a story fork)
    bool voteNoHostTieBreak = false;  // a tied vote is declined instead of the host deciding
    bool voteTimeoutApproves = false; // the consent window running out approves the choice
    bool noChoiceConsent = false;     // a fork INFO runs its scripts at the pick, before the party was asked
    bool denyKeepsSaid = false;       // a denied fork leaves the INFO marked as said for the initiator (the choice is gone for good)
    bool actorAsksParty = false;      // a personal decision (Caesar's surgery, a trooper's treatment) is put to a vote too
    bool partyCompanionCounts = false;// a party companion (ED-E) counts against its recruiter's slots and the party total
    bool partyHiredPerSlot = false;   // the Hired variable of a party companion is per slot (only the recruiter sees ED-E as hired)
    bool hostTalkSilent = false;      // the host player's own conversation is not shown to the other players
    // --- wave 1b (G08 / G27: the rules engine's ReadyCheck / ChoiceConsent votes carried by the dialogue host)
    bool noInfoGate = false;          // an INFO's END script runs at once, the rules engine is never asked (an irreversible fork or a ready check is skipped)
    bool heldEndRunsOnDeny = false;   // a held END script runs although the party declined / the ready check timed out
    bool heldEndNeverReleased = false;// an accepted vote does not release the held END script (the conversation hangs at that line)
    bool voteAnswerDropped = false;   // the other players' Agree / Not yet never reach the rules engine
    bool voteAnswerInverted = false;  // Agree is sent as a no and Not yet as a yes
    bool boxGateIgnored = false;      // a rules-owned message box resumes its script on the origin's answer alone
    bool rulesOwnerKeepsOldHold = false; // an INFO the rules engine owns is also held at the pick by the old consent table (the party is asked twice)
    // --- story run S06 (a guest's pick ran its result four times: the same INFO picked again right after its own line, no list in between)
    bool repeatPickRuns = false;      // the host runs the scripts of a second pick of the INFO it just ran, although the initiator never offered a list in between
    bool noPickLatch = false;         // the glue sends a second Pick for the INFO it already picked while the menu never went back to waiting for a pick
    // --- story run S06 (say-once marks the engine makes on one machine only: scripted SayTo lines of a forced stage chain; D11)
    bool sayOnceMarkNotReported = false;   // a guest's engine mark (TESTopicInfo::MarkSaid) is never reported to the host
    bool sayOnceMarkTrainerShared = false; // a trainer / Speech-XP line is reported and committed like any other (the first player uses it up for all)
    bool sayOnceMarkHostNoCommit = false;  // the host marks its own copy but never commits the SayOnceFlag effect (the other guests keep it unsaid)
    bool sayOnceMarkHostNoMark = false;    // the host commits the effect for a guest's report but never marks its own copy (the flag stays absent on the host)
    bool sayOnceMarkHostDrops = false;     // the host ignores a guest's report packet
};
inline Mutations& mutations() {
    static Mutations m;
    return m;
}
struct MutationScope {
    Mutations saved;
    explicit MutationScope(const Mutations& m) : saved(mutations()) { mutations() = m; }
    ~MutationScope() { mutations() = saved; }
};
}  // namespace fnvcoop_dlg
