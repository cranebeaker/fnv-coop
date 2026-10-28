// social/companions: game-free rules for FOLLOWERS (FULL-SYNC-PLAN 3.10). C++17, no engine, no network, no clock: time is passed in.
//
// The model (one sentence per rule, all enforced by code in companions.cpp and by unit tests with mutation twins 801..829):
//   * A companion belongs to ONE recruiter, wherever it goes. The committed CompanionOwner effect is the fact; CompanionBook is every machine's
//     replica of it. The ownership service (synccore/ownership.h) already makes the recruiter's machine the actor's owner (follower exception).
//   * Follower caps are per player and per kind (humanoid / critter) plus a party total, the same numbers HostSession enforces at commit time.
//     The book re-checks them (`overCap`) so a cap breach that slipped through (a rejoin, a save) is found and repaired.
//   * COMMANDS (wait, follow, stance, range, wheel, trade, equip, heal, summon, dismiss) are OWNER-ONLY. The wheel opens on the owner's machine
//     only (gateActivation); a command a non-owner sends is refused by the host (CommandJudge) and never changes the companion's state.
//   * The companion's mode / stance / range is shared state with a revision (`rev`): the owner publishes, everyone mirrors, an older rev is ignored.
//   * The companion's INVENTORY belongs to its owner: only the owner's machine changes it, as idempotent signed deltas (CompanionInventory), and
//     every other machine mirrors them onto its copy of the NPC (so its corpse and its visible equipment match on every screen).
//   * A PARTY companion (ED-E in Lonesome Road, owner decision D2) belongs to the whole party: the recruiter slot stays on the wire (a restart or a late
//     joiner still sees a valid owner, and the recruiter's machine simulates it), but it counts against no follower cap, any player may command it
//     and open its wheel, any player's inventory change on it is accepted, and when its recruiter leaves it is handed to the host instead of released.
//   * A machine's engine may only have the companions of ITS player flagged as teammates (the follower AI follows the LOCAL player):
//     reconcileTeammates() lists what to unset (flagged but somebody else's) and set (mine but not flagged).
//   * Companions and players are allies: a hit between two party members (player, companion) is neutralised unless friendly fire is on, and a
//     party member is never a VATS target (companionHitPolicy, vatsTargetAllowed).
//
// Wire: every struct that travels has a symmetric io(Archive&, T&) (synccore/include/fnvcoop/wire.h).
#pragma once
#include <cstdint>
#include <functional>
#include <map>
#include <set>
#include <string>
#include <vector>
#include "fnvcoop/effect.h"
#include "fnvcoop/types.h"
#include "fnvcoop/wire.h"

namespace fnvcoop::social {

enum class CompanionKind : uint8_t { Humanoid = 0, Critter = 1 };   // the CompanionOwner effect's `kind`

// Same defaults as HostSession::Options::FollowerCaps (session.h): 1 humanoid + 1 critter per owner, 4 for the party.
struct FollowerCaps {
    uint8_t humanoidPerOwner = 1, critterPerOwner = 1, partyTotal = 4;
    friend bool operator==(const FollowerCaps& a, const FollowerCaps& b) {
        return a.humanoidPerOwner == b.humanoidPerOwner && a.critterPerOwner == b.critterPerOwner && a.partyTotal == b.partyTotal;
    }
};

enum class FollowMode : uint8_t { Follow = 0, Wait = 1 };
// The engine's Aggression actor value of the NPC (read from the game's own companion scripts: `<ref>.SetAV Aggression 0|1|2`): 0 unaggressive, 1 aggressive,
// 2 very aggressive. Our names for the three levels: Passive / Defensive / Aggressive.
enum class Stance : uint8_t { Passive = 0, Defensive = 1, Aggressive = 2 };
enum class Range : uint8_t { Close = 0, Medium = 1, Far = 2 };                // how far it follows / fights from
struct CompanionState {
    FollowMode mode = FollowMode::Follow;
    Stance stance = Stance::Defensive;
    Range range = Range::Medium;
    uint32_t rev = 0;               // rises with every accepted change; the owner is the only writer, a lower or equal rev is ignored
    friend bool operator==(const CompanionState& a, const CompanionState& b) {
        return a.mode == b.mode && a.stance == b.stance && a.range == b.range && a.rev == b.rev;
    }
};
void io(Archive& a, CompanionState& s);

struct Companion {
    FormRef npc;                    // the NPC's placed reference (plugin + local id), what CompanionOwner carries
    PlayerId owner = kNoPlayer;     // the recruiter
    CompanionKind kind = CompanionKind::Humanoid;
    CompanionState state;
    bool party = false;             // owner=party (G34): belongs to everybody; `owner` stays the recruiter (simulates it, wire still valid)
};
void io(Archive& a, Companion& c);

enum class RecruitReason : uint8_t { Ok = 0, AlreadyYours, OwnedByOther, OwnerCap, PartyCap, BadOwner, Count_ };   // (party companions never hit a cap)
const char* toString(RecruitReason r);
struct RecruitVerdict {
    bool ok = false;
    RecruitReason reason = RecruitReason::Ok;
};

class CompanionBook {
public:
    explicit CompanionBook(FollowerCaps caps = {}) : caps_(caps) {}
    const FollowerCaps& caps() const { return caps_; }
    void setCaps(const FollowerCaps& c) { caps_ = c; }

    // ---- party companions (G34) ----
    // Which NPCs belong to the whole party. `fixed` = a data list (always party); the oracle = the rules engine (RulesEngine::isPartyCompanion: a CompanionOwner
    // rule with owner=party saw the recruit). The flag of a book entry is refreshed by refreshParty(); a new recruit asks at once.
    void setPartyOracle(std::function<bool(const FormRef&)> o) { oracle_ = std::move(o); }
    void addFixedParty(const FormRef& npc) { fixedParty_.insert(npc); }
    bool wantsParty(const FormRef& npc) const;                    // fixed list or oracle says so
    std::vector<FormRef> refreshParty();                          // re-asks for every entry; returns the NPCs whose flag changed
    bool isParty(const FormRef& npc) const { const Companion* c = find(npc); return c && c->party; }
    // The recruiter left: its party companions are handed to `to` (they stay in the book, flagged party). Returns the NPCs handed over.
    std::vector<FormRef> handParty(PlayerId from, PlayerId to);

    // Would a recruit be allowed right now? (owner connected is the caller's business; `kNoPlayer` / >= kMaxSlots are refused.)
    // AlreadyYours is ok=true (idempotent): the same owner re-recruiting changes nothing.
    RecruitVerdict check(const FormRef& npc, PlayerId owner, CompanionKind kind) const;
    RecruitVerdict recruit(const FormRef& npc, PlayerId owner, CompanionKind kind);   // check + apply

    // What an applied fact changed (for the plugin: the previous owner must un-flag the NPC, the new one flag it).
    struct Change {
        bool changed = false;
        FormRef npc;
        PlayerId from = kNoPlayer;      // previous owner (kNoPlayer = was nobody's)
        PlayerId to = kNoPlayer;        // new owner (kNoPlayer = dismissed)
    };
    // A COMMITTED fact (the host already judged it): applied without cap checks; a recruit by another player takes the NPC over; a dismiss by
    // anybody removes it. Anything that is not a CompanionOwner effect changes nothing. Idempotent.
    Change applyEffect(const Effect& e);
    // The owner or the host dismisses. false = not a companion, or `by` may not.
    bool dismiss(const FormRef& npc, PlayerId by, PlayerId host = kHostPlayer);
    // A player left the session: every companion of theirs is released and returned (the host commits a dismiss for each).
    std::vector<FormRef> playerLeft(PlayerId p);

    bool isCompanion(const FormRef& npc) const { return list_.count(npc) != 0; }
    PlayerId ownerOf(const FormRef& npc) const;   // kNoPlayer = not a companion
    const Companion* find(const FormRef& npc) const;
    std::vector<FormRef> companionsOf(PlayerId owner) const;
    size_t countOf(PlayerId owner) const;
    size_t countOf(PlayerId owner, CompanionKind kind) const;
    size_t total() const { return list_.size(); }
    // What counts against the follower caps: everything except party companions.
    size_t capCountOf(PlayerId owner, CompanionKind kind) const;
    size_t capTotal() const;
    // true when `owner` holds more than the caps allow (humanoids, critters) or the party total is exceeded.
    bool overCap(PlayerId owner) const;
    bool partyOverCap() const { return capTotal() > caps_.partyTotal; }

    // The owner's mode / stance / range. Accepted only when state.rev is HIGHER than the stored one. false = unknown NPC or stale.
    bool setState(const FormRef& npc, const CompanionState& s);
    // Order-independent hash of (npc, owner, kind, state) over the whole book: equal digests = equal books (desync check).
    uint64_t digest() const;
    std::vector<Companion> roster() const;
    void replaceRoster(const std::vector<Companion>& all);   // a join / resync: the host's table wins
    void clear() { list_.clear(); }

private:
    FollowerCaps caps_;
    std::map<FormRef, Companion> list_;
    std::function<bool(const FormRef&)> oracle_;
    std::set<FormRef> fixedParty_;
};

// ---- commands ------------------------------------------------------------------------------------------------------------------------
enum class CmdKind : uint8_t {
    Wait = 1, Follow, SetStance, SetRange,         // change the shared state
    OpenWheel, OpenInventory, Equip, Heal, Talk,   // actions on the owner's machine
    Summon,                                        // move the companion next to its owner
    Dismiss,                                       // release it (becomes a normal NPC)
    Count_
};
const char* toString(CmdKind k);
inline bool changesState(CmdKind k) { return k == CmdKind::Wait || k == CmdKind::Follow || k == CmdKind::SetStance || k == CmdKind::SetRange; }

struct CompanionCmd {
    uint64_t id = 0;                // (issuer slot << 48 | counter): the idempotency key
    PlayerId from = kNoPlayer;      // who says they issued it
    FormRef npc;
    CmdKind kind = CmdKind::Wait;
    uint8_t arg = 0;                // SetStance: Stance, SetRange: Range
};
void io(Archive& a, CompanionCmd& c);

enum class CmdReason : uint8_t { Ok = 0, Duplicate, NotACompanion, NotOwner, BadSender, BadArg, UnknownKind, TooFast, Count_ };
const char* toString(CmdReason r);

struct CmdRules {
    uint32_t minGapMs = 100;        // per issuer: faster than this is refused (a stuck key, a flood)
    size_t remember = 4096;         // command ids kept for idempotency
};
struct CmdVerdict {
    bool apply = false;
    CmdReason reason = CmdReason::Ok;
    bool stateChanged = false;      // newState differs from the stored one
    CompanionState newState;        // state after the command (rev already raised when it changed)
    bool dismiss = false;           // CmdKind::Dismiss accepted
};

// The host's judge of a command message. It never changes the book: the caller commits `newState` with CompanionBook::setState / `dismiss`.
class CommandJudge {
public:
    explicit CommandJudge(CmdRules r = {}) : rules_(r) {}
    // `sender` = the machine the message came from (its author); `nowMs` = a monotonic clock.
    CmdVerdict judge(const CompanionCmd& c, PlayerId sender, uint64_t nowMs, const CompanionBook& book);
    bool seen(uint64_t id) const { return ids_.count(id) != 0; }
    void reset() { ids_.clear(); order_.clear(); last_.clear(); }
private:
    CmdRules rules_;
    std::set<uint64_t> ids_;
    std::vector<uint64_t> order_;
    std::map<PlayerId, uint64_t> last_;
};

// ---- the wheel: who may open it -------------------------------------------------------------------------------------------------------
enum class ForeignPolicy : uint8_t { Deny = 0, Allow = 1 };   // somebody else's companion: refuse the activation, or let it through
enum class ActivateAction : uint8_t { NotACompanion = 0, OwnerOpens, DenyForeign, AllowForeign, PartyOpens };   // PartyOpens: a party companion, anybody may
struct ActivationVerdict {
    ActivateAction action = ActivateAction::NotACompanion;
    PlayerId owner = kNoPlayer;
    bool suppress() const { return action == ActivateAction::DenyForeign; }
};
// `who` = the player whose machine (or whose forwarded intent) activates `npc`.
ActivationVerdict gateActivation(const CompanionBook& book, PlayerId who, const FormRef& npc, ForeignPolicy policy);

// ---- inventory ---------------------------------------------------------------------------------------------------------------------------
struct InvDelta {
    uint64_t txn = 0;               // (owner slot << 48 | counter): idempotent
    FormRef npc;
    FormRef item;
    int32_t delta = 0;              // + added to the companion, - taken from it
};
void io(Archive& a, InvDelta& d);
// Signed: the engine keeps a companion's inventory as CHANGES against its base container (ExtraContainerChanges countDelta), so a count is
// positive for an item it was given and negative for a base item it gave away. Zero means "as the base".
using ItemCounts = std::map<FormRef, int32_t>;
constexpr int32_t kMaxItemCount = 1000000;   // a count beyond this (either way) is a runaway, not an inventory
struct InvFull {
    FormRef npc;
    std::vector<std::pair<FormRef, int32_t>> items;   // every item with a non-zero count
};
void io(Archive& a, InvFull& f);

enum class InvReason : uint8_t { Ok = 0, Duplicate, NotACompanion, NotOwner, OutOfRange, ZeroDelta, Count_ };
const char* toString(InvReason r);
struct InvVerdict {
    bool apply = false;
    InvReason reason = InvReason::Ok;
    bool resync = false;            // the sender's picture is wrong (a runaway count): send it the full table
};

// The per-owner companion inventory ledger. One per machine. The OWNER samples its engine copy and turns changes into deltas; the host judges
// them (owner only, once each, within range); every machine mirrors accepted deltas.
class CompanionInventory {
public:
    explicit CompanionInventory(size_t remember = 8192) : cap_(remember) {}
    // Owner side: the companion's inventory as the engine shows it NOW. Returns the deltas against the last sample (empty on the first sample of
    // an NPC: it seeds the baseline and the caller sends fullOf()). `counter` must rise per call (next txn id = slot << 48 | ++counter).
    std::vector<InvDelta> sample(const FormRef& npc, const ItemCounts& now, PlayerId owner, uint64_t& counter);
    // Host / mirror: judge one delta from `sender`.
    InvVerdict judge(const InvDelta& d, PlayerId sender, const CompanionBook& book);
    // Applies an ACCEPTED delta to the model (host and mirrors). Returns the new count.
    int32_t apply(const InvDelta& d);
    // A full table replaces the model of one NPC; returns the deltas that turn the OLD model into the new one (what a mirror applies to its copy).
    std::vector<InvDelta> replace(const InvFull& f, PlayerId asOwner, uint64_t& counter);
    InvFull fullOf(const FormRef& npc) const;
    int32_t count(const FormRef& npc, const FormRef& item) const;
    uint64_t digestOf(const FormRef& npc) const;
    void forget(const FormRef& npc) { model_.erase(npc); last_.erase(npc); }
    void reset() { model_.clear(); last_.clear(); ids_.clear(); order_.clear(); }
private:
    bool remember(uint64_t id);
    size_t cap_;
    std::map<FormRef, ItemCounts> model_;   // authoritative picture (host) / mirrored picture
    std::map<FormRef, ItemCounts> last_;    // owner side: the last engine sample
    std::set<uint64_t> ids_;
    std::vector<uint64_t> order_;
};

// ---- teammate flags --------------------------------------------------------------------------------------------------------------------
struct TeammateFix {
    std::vector<FormRef> unset;     // flagged as teammate here but not this player's: SetPlayerTeammate 0
    std::vector<FormRef> set;       // this player's companion, loaded here, but not flagged: SetPlayerTeammate 1
    bool empty() const { return unset.empty() && set.empty(); }
};
// `flagged` = NPCs the local engine has as teammates (the player's teammate list / the actor flag); `loaded` = NPCs of the book that exist here as
// REAL actors (not stand-ins, not unloaded). An NPC the book does not know and that is flagged is ALSO unset (a leftover of a script run for a guest).
TeammateFix reconcileTeammates(const CompanionBook& book, PlayerId self, const std::set<FormRef>& flagged, const std::set<FormRef>& loaded);

// ---- the party: friendly fire and VATS -----------------------------------------------------------------------------------------------------
// A combatant as the party sees it: a player, somebody's companion, or anything else (an enemy, a bystander, scenery).
struct Combatant {
    enum Kind : uint8_t { Other = 0, Player, Companion } kind = Other;
    PlayerId slot = kNoPlayer;      // Player: its slot; Companion: its recruiter
    static Combatant player(PlayerId s) { return {Player, s}; }
    static Combatant companion(PlayerId recruiter) { return {Companion, recruiter}; }
    static Combatant other() { return {Other, kNoPlayer}; }
    bool party() const { return kind != Other; }
    // The player answerable for what this combatant does (a companion's kills and crimes are its recruiter's).
    PlayerId responsible() const { return kind == Other ? kNoPlayer : slot; }
};
enum class HitPolicy : uint8_t { Allow = 0, Neutralize };
// Hits inside the party (player <-> player, player <-> companion, companion <-> companion) do nothing unless `friendlyFire`. Everything that
// involves an outsider is Allow: enemies hurt companions, companions hurt enemies.
HitPolicy companionHitPolicy(const Combatant& attacker, const Combatant& victim, bool friendlyFire);
// May the local player pick this as a VATS target? Never a party member (VATS is real-time here; an ally is not an enemy).
bool vatsTargetAllowed(const Combatant& target);

}  // namespace fnvcoop::social
