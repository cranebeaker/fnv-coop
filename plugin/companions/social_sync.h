// SocialSync: companions, crime and kill rewards of ONE machine (FULL-SYNC-PLAN 3.4 / 3.8 / 3.10). Game-free: everything it touches is behind
// social_ports.h; the rules are synccore/social. NOTHING here has run in a game.
//
//   COMPANIONS   The host learns recruits and dismisses from the story effects (onEffect), keeps the CompanionBook and tells everybody (MRoster).
//                Every machine reconciles its engine's teammate list with the book (only MY companions may be flagged here). The owner publishes
//                its companions' state (MCmd -> host -> MState) and inventory deltas (MInv -> host -> everybody else); a non-owner can do neither.
//   CRIME        Anybody reports (reportCrime / a trespass poll / the host's reading of a guest's interaction); the host judges ONCE
//                (CrimeJudge: witnesses from every machine's detections, bounty, hostility, alarm) and everybody applies the result once
//                (MBounty -> faction flags, MHostility -> SetEnemy/SetAlly, MAlarm -> each machine alerts the NPCs it simulates).
//   REWARDS      Anybody reports a kill (reportKill); the host settles it ONCE by its XP / karma settings and delivers the result as committed
//                effects (ModActorValue experience, ModKarma, ModReputation with explicit recipients); its own part it applies directly.
//
// Threading: main thread only (the frame loop). No locks.
#pragma once
#include <functional>
#include <map>
#include <set>
#include <vector>
#include "companions/social_ports.h"
#include "social/escort.h"
#include "social/slots.h"

namespace nvcoop::social {

struct Config {
    PlayerId self = kHostPlayer;
    bool isHost = false;
    sc::SocialSettings settings;                 // host: the settings to start with (clients take the host's)
    bool companions = true, crime = true, rewards = true;   // module switches: a part that is off sends and answers nothing
    bool shareReputation = true;                 // SharePolicy::reputation of the session
    sc::CrimeRules crimeRules = sc::defaultCrimeRules();
    cc::CombatRules combatRules = cc::defaultRules();
    uint32_t reconcileMs = 1000;                 // teammate flags vs the book
    uint32_t fixCooldownMs = 3000;               // the same NPC is not flipped again inside this time
    uint32_t inventoryMs = 1000;                 // owner: companion inventory sampling
    uint32_t detectMs = 400;                     // detections poll
    uint32_t detectRefreshMs = 2000;             // a sighting is re-sent this often while it lasts (the host's window is 6 s)
    uint32_t trespassMs = 1000;                  // IsTrespassing poll
    uint32_t trespassCooldownMs = 30000;         // one trespass report per cell and this long
    uint32_t digestMs = 5000;                    // host: digest broadcast
    uint32_t resyncMinMs = 2000;                 // client: at most one resync request per this long
    // The actor layer's follower exception: every CompanionOwner effect the host learns of is fed to plugin/actors (engine::onEffect). Idempotent.
    std::function<void(const Effect&)> feedActors;
    // ---- G34 companion set pieces ----
    sc::SlotRules sets;                          // single-slot groups (Dead Money, Honest Hearts), data-named party companions, the view policy
    std::function<bool(const FormRef&)> isPartyCompanion;   // host: RulesEngine::isPartyCompanion (a CompanionOwner rule with owner=party saw the recruit)
    bool swapGuard = true;                       // host: onTxn reverts a hire that fires another player's companion and turns back a second member of a slot group
    bool escort = true;                          // bring my companions along when my player was moved
    sc::EscortRules escortRules;
    uint32_t escortMs = 500;                     // escort poll
    // Called with the whole roster whenever it changed here (host: after it broadcast it; client: after the host's table was applied). The session wiring feeds the
    // dialogue registry from it (owner, party flag), so the per-owner follower variables and the party companions agree with the book.
    std::function<void(const std::vector<sc::Companion>&)> onRoster;
    uint32_t repairDelayMs = 250;                // host: a repair is committed this long after the transaction that needed it (the session must have seen that one first)
};

struct Stats {
    uint64_t received = 0, dropped = 0, spoofed = 0, notFromHost = 0, sent = 0;
    // companions
    uint64_t rosterSent = 0, rosterApplied = 0, recruitsDropped = 0, capRepairs = 0, released = 0;
    uint64_t teammateUnset = 0, teammateSet = 0, teammateFailed = 0;
    uint64_t cmdIssued = 0, cmdRefused = 0, cmdAccepted = 0, cmdNacked = 0, stateApplied = 0, stateUnmapped = 0, summons = 0;
    uint64_t invSent = 0, invApplied = 0, invRejected = 0, invDuplicates = 0, invFulls = 0;
    uint64_t partyFlagged = 0, partyHanded = 0, swapsReverted = 0, slotDenied = 0, repairsCommitted = 0, seedCommitted = 0, escorted = 0, escortFailed = 0;
    uint64_t activationsDenied = 0, hitsNeutralized = 0;
    // crime
    uint64_t crimesReported = 0, crimesCounted = 0, crimesRefused = 0, detectSent = 0, detectRejected = 0, bountyApplied = 0, hostilityApplied = 0;
    uint64_t alarmsRaised = 0, alarmsDuplicate = 0, actorsAlerted = 0, trespassReports = 0, guestInteractions = 0;
    // rewards
    uint64_t killsReported = 0, killsSettled = 0, xpLocal = 0, xpCommitted = 0, karmaLocal = 0, karmaCommitted = 0, repLocal = 0, repCommitted = 0, commitFailed = 0;
    uint64_t resyncsAsked = 0, resyncsAnswered = 0, digestMismatch = 0, settingsApplied = 0;
};

class SocialSync {
public:
    SocialSync(const Config& cfg, ISocialWorld& world, ISocialEngine& engine, ISocialNet& net, ICommitter& committer);

    // ---- lifecycle -----------------------------------------------------------------------------------------------------------------
    void tick(uint64_t nowMs);                                       // once per frame; nowMs = this machine's clock (host: the host clock)
    void receive(PlayerId from, const uint8_t* data, size_t size);   // one datagram (the whole datagram, channel byte included)
    void playerJoined(PlayerId p);                                   // host: a client became part of the session
    void playerLeft(PlayerId p);                                     // host: a client left (its companions are released)
    void hostConnected();                                            // client: joined (asks for everything)
    void hostLost();                                                 // client: the link to the host is down (stops enforcing until it is back)
    // Story effects the HOST learns of (the capture sink's CompanionOwner). Ignored on clients (the roster message carries the same facts).
    void onEffect(const Effect& e);
    // The same for a whole committed transaction (its effects in order): judged by the swap guard first (G34), then learnt. Prefer this over onEffect for captured
    // transactions: a hire that fires somebody else's companion is only recognisable with the whole transaction in view.
    void onTxn(const std::vector<Effect>& effects);
    // W3a item 6 (host): the CompanionOwner effects of a txn the session is about to commit, judged against the book as it is now (HostSession::companionVet). The
    // dismissal of another player's companion that rides with a recruit of the same slot group, and a recruit that would give a player a second member of a one-slot
    // group, are removed from `effects`. Returns how many effects went.
    size_t vetOutgoing(std::vector<Effect>& effects);

    // ---- inputs from the other lanes ---------------------------------------------------------------------------------------------------
    // A crime the LOCAL machine saw (combat glue: a witnessed assault / murder; the trespass poll). id / reporter are filled in when 0.
    void reportCrime(sc::CrimeEvent e);
    // A kill (combat glue: from the death declaration). Fills the reporter.
    void reportKill(sc::KillReport r);
    // HOST: a guest's interaction arrived as an intent (activate / pick up). Reads the target's ownership and judges the theft / break-in / trespass.
    void onGuestInteraction(PlayerId slot, bool pickup, const FormRef& target);
    // Pardon (host): quest, bribe, dev bridge. Clears the faction's wanted score and flags and turns the war back into peace.
    void forgive(const FormRef& faction);
    // HOST: change the settings (xp mode, scope, caps, friendly fire ...). Broadcast to every machine.
    void setSettings(const sc::SocialSettings& s);

    // ---- companion commands (the OWNER's machine) -----------------------------------------------------------------------------------------
    // The owner's wheel did something. false = refused locally (not my companion, flood, bad argument). State changes apply to the engine at once.
    bool issueCommand(sc::CmdKind kind, const FormRef& npc, uint8_t arg = 0);

    // ---- questions the engine hooks ask -------------------------------------------------------------------------------------------------
    // May `who` activate `npc` (opens the wheel / talks)? false = suppress it. Non-companions are always allowed.
    bool allowActivation(PlayerId who, const FormRef& npc);
    // A hit between two combatants: Neutralize inside the party (players and companions) unless friendly fire is on.
    sc::HitPolicy hitPolicy(const cc::Entity& attacker, const cc::Entity& victim);
    bool vatsTargetAllowed(const cc::Entity& target);
    // The player answerable for an entity's actions (a companion's recruiter, a player itself); kNoPlayer = nobody.
    PlayerId responsiblePlayer(const cc::Entity& e);

    // The companion `slot` has in a single-slot group (Dead Money's rCurrentFollower for that player); an empty FormRef = none.
    FormRef currentFollower(PlayerId slot, const std::string& group) const { return sc::currentMember(book_, cfg_.sets, group, slot); }

    // ---- views --------------------------------------------------------------------------------------------------------------------------
    const sc::CompanionBook& book() const { return book_; }
    const sc::CrimeJudge& crimeJudge() const { return crime_; }
    const sc::CompanionInventory& inventory() const { return inv_; }
    const sc::SocialSettings& settings() const { return settings_; }
    const Stats& stats() const { return stats_; }
    const Config& config() const { return cfg_; }
    bool isHost() const { return cfg_.isHost; }
    PlayerId self() const { return cfg_.self; }
    bool synced() const { return synced_; }
    bool xpHandled() const { return cfg_.rewards; }
    std::string statusJson() const;

private:
    // sending
    void send(PlayerId dest, const sc::Body& b, PlayerId src = kNoPlayer);        // dest = a slot; client: always the host
    void sendAll(const sc::Body& b, PlayerId except = kNoPlayer, PlayerId src = kNoPlayer);   // host: every connected client but `except`
    void sendJoinState(PlayerId p);
    uint8_t connectedMask() const;
    // dispatch
    void onMessage(const sc::Envelope& env);
    // host side
    void hCrime(PlayerId src, const sc::MCrime& m);
    void hDetect(PlayerId src, const sc::MDetect& m);
    void hKill(PlayerId src, const sc::MKill& m);
    void hCmd(PlayerId src, const sc::MCmd& m);
    void hInv(PlayerId src, const sc::MInv& m);
    void hInvFull(PlayerId src, const sc::MInvFull& m);
    void hInvAsk(PlayerId src, const sc::MInvAsk& m);
    // client side
    void cBounty(const sc::MBounty& m);
    void cAlarm(const sc::MAlarm& m);
    void cHostility(const sc::MHostility& m);
    void cRoster(const sc::MRoster& m);
    void cState(const sc::MState& m);
    void cInv(const sc::MInv& m);
    void cInvFull(const sc::MInvFull& m);
    void cSettings(const sc::MSettings& m);
    void cDigest(const sc::MDigest& m);
    void cCmdNack(const sc::MCmdNack& m);
    // shared pieces
    void judgeCrime(const sc::CrimeEvent& e, PlayerId sender);
    void applyDecision(const sc::CrimeDecision& d);
    void settleKill(const sc::KillReport& r);
    void raiseAlarmLocal(const sc::AlarmOrder& o);
    void applyStandingLocal(const sc::BountyUpdate& u);
    void releaseCompanion(const FormRef& npc, const char* why);
    void handPartyCompanions(PlayerId leaver);
    void refreshParty();
    void flushRepairs();
    void pollEscort();
    void broadcastRoster();
    void applyOwnState(const sc::Companion& c);
    void askResync();
    // periodic
    void seedHostCompanions();
    void reconcileTeammates();
    void sampleInventories();
    void pollDetections();
    void pollTrespass();
    uint64_t nextId() { return (static_cast<uint64_t>(cfg_.self) << 48) | ++idCounter_; }
    bool applyInvMirror(const sc::InvDelta& d);

    Config cfg_;
    ISocialWorld& world_;
    ISocialEngine& engine_;
    ISocialNet& net_;
    ICommitter& commit_;
    sc::CompanionBook book_;
    sc::CommandJudge localJudge_;       // my own pre-check of my own commands
    sc::CommandJudge hostJudge_;        // host: the authoritative judge of everybody else's
    sc::CompanionInventory inv_;
    sc::Escort escort_;
    std::vector<Effect> repairs_;       // host: effects to commit once repairAtMs_ has passed
    uint64_t repairAtMs_ = 0;
    uint64_t lastEscortMs_ = 0;
    sc::DetectionBook det_;
    sc::CrimeJudge crime_;
    sc::RewardLedger rewards_;
    sc::SocialSettings settings_;
    Stats stats_;

    uint64_t nowMs_ = 0;
    bool synced_ = false;               // the host's picture has arrived (always true on the host)
    bool hostUp_ = true;
    bool dirtyReconcile_ = false;
    bool seeded_ = false;               // host: the companions it already had when the session began were adopted
    std::set<PlayerId> connected_;      // host: joined clients (and the host itself)
    std::set<uint64_t> alarmsSeen_;
    std::set<uint64_t> mirrorSeen_;     // mirrors: inventory txn ids already applied
    std::vector<uint64_t> mirrorOrder_;
    std::map<FormRef, uint64_t> lastFixMs_;
    std::map<FormRef, sc::CompanionState> appliedState_;
    std::map<std::pair<uint32_t, PlayerId>, uint64_t> detectSent_;   // (observer net id, target) -> last time sent
    std::map<std::pair<uint32_t, PlayerId>, sc::Detection> lastSeen_;   // what was sent last (the "lost sight" message repeats it)
    std::map<uint32_t, uint64_t> trespassAt_;                        // cell -> last report
    uint64_t idCounter_ = 0, invCounter_ = 0;
    uint64_t lastReconcileMs_ = 0, lastInvMs_ = 0, lastDetectMs_ = 0, lastTrespassMs_ = 0, lastDigestMs_ = 0, lastCrimeTickMs_ = 0, lastResyncMs_ = 0;
};

}  // namespace nvcoop::social
