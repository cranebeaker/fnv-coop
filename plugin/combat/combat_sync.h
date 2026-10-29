// CombatSync: combat on one machine (FULL-SYNC-PLAN 3.4). Game-free; one instance per machine; the engine, the ownership picture and the
// network are reached through combat_ports.h. The rules are synccore/combat (combat-core); this file is the choreography.
//
//   A HIT is resolved ONCE, by the machine that owns the victim:
//     attacker's machine  : the engine hit gate (pre-damage) calls onEngineHit(). The attacker is simulated here (the local player, or an NPC
//                           this machine owns). Victim simulated here -> Proceed (the engine applies it natively, the owner path).
//                           Victim is a copy/puppet/stand-in -> a HitEvent goes to the victim's owner and the local damage is CLAMPED so the
//                           copy never dies from a prediction (it keeps >= 1 health, no dismember/explode, no fatigue knock-out).
//     remote-driven attacker : a copy of an NPC (or a puppet) that the local engine lets "hit" something is NEUTRALISED completely: its owner
//                           reports that hit to the victim's owner. That is what keeps visual-only projectiles and copies harmless.
//     victim's owner      : receive() -> HitValidator::judge (owner? new? alive? in range? rate? attacker who the sender says?) -> applyHit()
//                           (health + limb crippling + named/essential protection) -> the engine takes the health off -> MVitals to everyone.
//                           A hit it cannot resolve (handoff in flight, ownership table behind, victim unknown yet) is NACKed; the attacker
//                           re-sends it under a fresh id a few times. The player as victim: the player's own machine applies what the NPC's
//                           owner reports.
//   DEATH: ActorSync/the actor port decides THAT an actor dies (owner, or a host KillActor for a named NPC). onActorDied() adds the declaration
//     (killer, weapon, crippled limbs) exactly once (DeathLedger) and awards kill XP exactly once (XpLedger).
//   PROJECTILES / EXPLOSIONS: declared by the shooter's owner; everyone else gets a visual-only copy. Explosion damage goes out as one hit per
//     victim, to that victim's owner, with an id derived from (explosion, victim) so a second pass cannot double it.
//   CRIME / HOSTILITY: host-authoritative (HostilityBook), off by default (Config::crimeSync).
#pragma once
#include <deque>
#include <functional>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>
#include "combat/combat_wire.h"
#include "social/rewards.h"   // KillReport, CrimeEvent: plain data for the social hooks below (header only; nothing of synccore/social is linked here)

namespace nvcoop::combat {

// CombatRules tuned for the co-op game (anti-cheat is a non-goal; the rules exist to stop echoes, forgeries by accident and double hits).
cc::CombatRules coopRules();

// What plugin/companions (companions, crime, shared kill rewards) asks of combat. All optional; an empty function is the old behaviour. The module wires them
// to nvcoop::social::glue* when it is built with NVCOOP_WITH_COMPANIONS (combat_module.cpp); the tests hand in fakes.
struct SocialHooks {
    std::function<bool()> xpHandled;                                       // true: the social module pays kill XP / karma for every kill; combat's own awardXp() stands down
    std::function<void(const fnvcoop::social::KillReport&)> reportKill;    // once per death, from the machine that declared it
    std::function<void(const fnvcoop::social::CrimeEvent&)> reportCrime;   // a witnessed attack / a kill of a faction member by a player or a companion
    std::function<bool(uint32_t attackerRef, uint32_t victimRef)> neutralizeHit;   // true: a hit inside the party (HitPolicy::Neutralize): the hit gate zeroes it
    // G40 (rules FriendlyFireGuard): true = this hit by a PLAYER on a protected story actor is an accident and is dropped (the attacker has been told). Asked on the machine
    // that holds the rules (the host); a machine without them answers false. `splash` = an explosion.
    std::function<bool(uint32_t victimRef, PlayerId attacker, bool splash)> blockProtectedHit;
};

struct Config {
    PlayerId self = kHostPlayer;
    bool isHost = true;
    cc::CombatRules rules = coopRules();
    cc::XpMode xpMode = cc::XpMode::KillingBlow;
    bool friendlyFire = false;            // player-vs-player damage (off: such hits are neutralised on the attacker's machine)
    bool crimeSync = false;               // host: turn witnessed crimes into SetEnemy/SetAlly effects
    uint32_t vitalsMinMs = 100;           // coalesce the vitals broadcast of natural (engine-applied) hits on one victim
    uint32_t retryDelayMs = 150;          // attacker: re-send a NACKed hit after this long
    uint8_t retryMax = 3;                 // ... at most this many times
    uint32_t inflightTtlMs = 4000;        // attacker: forget a forwarded hit after this long
    uint32_t lastHitTtlMs = 15000;        // kill credit: the last attacker counts for this long
    uint32_t blastSettleMs = 120;         // explosions: plan hits for victims the engine did not iterate this long after the first hit
    uint32_t deathDecorTtlMs = 5000;      // a declaration waits this long for its corpse to exist here
    uint32_t deferDeathMs = 1500;         // a declaration the ownership table was behind for is retried this long
    uint32_t lethalSettleMs = 1500;       // owner: a victim a NATURAL (engine-applied) hit left at health <= 0 gets this long to be dead by the engine's own life state (a VATS kill cam), then the owner kills it
    uint32_t killConfirmMs = 400;         // owner: a hit the rules call lethal must show the victim dead after this long, or the owner kills it
    SocialHooks social;
    // plugin/threat: every hit that lands on an actor this machine simulates (the engine's own, an accepted forwarded one, a light blow), so an enemy's damage ledger is exact
    std::function<void(uint32_t victimRef, uint32_t attackerRef, float damage)> onHitLanded;
};

// One hit as the engine hit gate sees it, BEFORE the damage is applied. All raw; the glue fills what it can read.
struct EngineHit {
    uint32_t attackerRef = 0, victimRef = 0;     // engine refs; 0 = none (environment, trap, fall)
    float healthDmg = 0, limbDmg = 0, fatigueDmg = 0;
    int32_t hitLocation = -1;                    // engine body-part index (ActorHitData+0x10); limbOfLocation() maps it
    cc::Vec3 impact;
    FormRef weapon, ammo;                        // empty = unknown
    uint64_t projectileKey = 0;                  // identity of the projectile object that carried it (0 = hitscan / melee)
    uint64_t explosionKey = 0;                   // identity of the explosion object (0 = not an explosion hit)
    cc::Vec3 blastCenter;                        // explosion: centre (the explosion object's position) and radius (0 = unknown: plain hits, no planning)
    float blastRadius = 0;
    FormRef explosionBase;                       // the explosion object's base form (empty = unknown)
    FormRef projectileBase;                      // the projectile object's base form (empty = unknown)
    bool melee = false, critical = false, vats = false;
    float weaponRange = 0;                       // 0 = unknown
};
cc::Limb limbOfLocation(int32_t hitLocation);    // GECK body part 0..14 -> combat limb (torso for "none"/unknown)

enum class GateAction : uint8_t {
    Proceed,       // let the engine apply the hit as it computed it
    Clamp,         // let the engine play the hit but leave the victim at least 1 health: health <= maxHealth, fatigue/limb 0, no lethal flags
    Neutralize,    // zero every damage field: the hit only plays its visuals
};
struct GateDecision {
    GateAction action = GateAction::Proceed;
    int32_t maxHealthDamage = 0;     // Clamp: health damage the engine may still apply
    const char* why = "";            // short reason (diagnostics)
};

struct Stats {
    uint64_t engineHits = 0, proceeded = 0, clamped = 0, neutralized = 0, ghostHitsNeutralized = 0, envHitsNeutralized = 0, friendlyNeutralized = 0;
    uint64_t hitsSent = 0, hitsRelayed = 0, hitsReceived = 0, hitsApplied = 0, hitsRejected = 0, hitDuplicates = 0, hitsForwarded = 0, hopLimit = 0;
    uint64_t nacksSent = 0, nacksReceived = 0, hitRetries = 0, hitsAbandoned = 0, resyncHints = 0, hitsResolvedLate = 0, killsForced = 0, naturalLethal = 0, naturalLethalStuck = 0, killsAttributed = 0;
    uint64_t damageApplied = 0, damageRefused = 0, limbsCrippled = 0, provoked = 0, engineRefused = 0, naturalHits = 0;
    // A blow by a combatant simulated here on a copy: how many were light (health damage under one point, sent exactly), how many did no health damage at all (a touch:
    // sent and replayed as a reaction only), and what the owner did with the light blows and touches it received (touchesEngine: the engine took the replay).
    uint64_t lightHitsSent = 0, lightHitsApplied = 0, touchesSent = 0, touchesApplied = 0, touchesEngine = 0, hitsOnCopies = 0;
    uint64_t vitalsSent = 0, vitalsApplied = 0, vitalsStale = 0;
    uint64_t deathsDeclared = 0, deathsAccepted = 0, deathsRejected = 0, deathsDeferred = 0, deathsRefusedLocal = 0, corpsesDecorated = 0;
    uint64_t xpAwards = 0, xpTotal = 0, xpUnknown = 0, xpEnginePaid = 0;
    uint64_t projectilesLocal = 0, projectilesVisual = 0, projectilesRejected = 0;
    uint64_t explosionsLocal = 0, explosionsVisual = 0, explosionsRejected = 0, blastHitsPlanned = 0, blastHitsSkipped = 0;
    uint64_t crimesReported = 0, hostilityApplied = 0, hostilityDropped = 0;
    uint64_t socialKillsReported = 0, socialXpHandled = 0, socialCrimesReported = 0, socialHitsNeutralized = 0, protectedHitsBlocked = 0;
    uint64_t undecodable = 0, spoofed = 0, unroutable = 0;
    uint64_t rejectedByReason[static_cast<size_t>(cc::HitReason::Count_)] = {};
};

class CombatSync {
public:
    CombatSync(const Config& cfg, IWorldView& world, ICombatEngine& engine, ICombatNet& net);
    ~CombatSync();

    PlayerId self() const { return cfg_.self; }
    bool isHost() const { return cfg_.isHost; }
    // Player-vs-player damage on or off while the session runs (the campaign's choice, plugin/rebalance: the host's word, sent to every guest).
    void setFriendlyFire(bool on);
    bool friendlyFire() const { return cfg_.friendlyFire; }

    // ---- session ----
    void playerJoined(PlayerId slot);    // host
    void playerLeft(PlayerId slot);      // host: forget what is in flight from/to it
    void hostLost();                     // client: drop in-flight state (a baseline follows)

    // ---- engine events (game main thread) ----
    GateDecision onEngineHit(const EngineHit& h);
    // The actor port saw an actor die here. authorised/mine/fromNetwork come from actors::engine::DiedEvent. killerRef: the attacker the engine
    // credits (0 = unknown; then the last accepted hit decides).
    void onActorDied(uint32_t ref, bool authorised, bool fromNetwork, bool mine, uint32_t killerRef);
    void onPlayerDied(uint32_t killerRef);     // the LOCAL player died
    void onPlayerAlive();                      // the LOCAL player is alive again (respawn/load): they can die again
    void onActorLost(const cc::Entity& who);   // cell unloaded / destroyed: forget per-actor combat state
    // G04 (host only): the engine ref of the puppet of the REMOTE player who killed the actor `victimRef` (the last accepted hit, or the death declaration that named a remote
    // player), 0 = nobody remote. The actor port asks it when it kills the host's copy (actors::engine::setKillerResolver), so a guest's kill is a kill BY that guest.
    uint32_t killerPuppetOf(uint32_t victimRef) const;
    // A projectile left a weapon fired by a combatant this machine simulates. TODO-RE: the engine feed (no verified projectile-create hook yet).
    void onProjectileFired(const cc::ProjectileSpawn& p);
    // Story effects applied on this machine (ResurrectActor makes the actor killable again).
    void onResurrect(const cc::Entity& who);

    // ---- network ----
    void receive(PlayerId from, const uint8_t* data, size_t size);   // one datagram from the mux
    void tick(uint64_t nowMs);

    // ---- queries (tests, diagnostics) ----
    const Stats& stats() const { return stats_; }
    const cc::HitValidator& validator() const { return validator_; }
    const cc::DeathLedger& deaths() const { return deaths_; }
    const cc::ProjectileRegistry& projectiles() const { return projectiles_; }
    const cc::HostilityBook& hostility() const { return hostility_; }
    size_t inflightHits() const { return inflight_.size(); }
    uint8_t crippledMask(const cc::Entity& e) const;
    std::string lastRejections() const;     // newest rejection lines
    const std::string& lastGate() const { return lastGate_; }   // newest gate decision on a synced victim: "clamp: forwarded to the victim's owner (victim 7, owner 0)"
    uint64_t nowMs() const { return now_; }

private:
    class Adapter;     // ICombatWorld over IWorldView + the projectile registry
    struct Inflight { cc::HitEvent hit; uint64_t sentAt = 0; uint64_t retryAt = 0; uint8_t attempts = 0; bool nacked = false; bool keepId = false; bool touch = false; uint32_t centi = 0; };
    struct ExpId { uint64_t id = 0; uint64_t at = 0; };
    struct LastHit { cc::Entity attacker; FormRef weapon; cc::Limb limb = cc::Limb::Torso; uint64_t at = 0; std::set<PlayerId> players; bool natural = false; };   // natural: the local engine applied it (and pays kill XP itself)
    struct Blast { cc::ExplosionEvent ev; uint64_t dueAt = 0; bool plan = true; std::set<cc::Entity> handled; };
    struct VisualBlast { cc::Vec3 pos; FormRef base; uint64_t until = 0; };
    struct VitalsDirty { uint64_t dueAt = 0; uint64_t cause = 0; };
    struct DeferredDeath { cc::DeathDecl d; PlayerId sender = kNoPlayer; uint64_t until = 0; };
    struct PendingDecor { cc::DeathDecl d; uint64_t until = 0; };

    // outbound
    void send(Envelope e);                                  // client: to the host; host: to env.dest, or fan-out when kNoPlayer
    void sendTo(PlayerId peer, const Envelope& e);
    void fanOut(const Envelope& e, PlayerId except);
    std::vector<PlayerId> peers() const;
    uint64_t nextId() { return (static_cast<uint64_t>(cfg_.self) << 48) | ++counter_; }
    // centi: the engine's health damage in hundredths, exactly (what the owner applies). The rules' own damage is whole points, at least 1. touch: the blow did no health
    // damage but is still a blow on an actor (centi 0; the owner replays it as a reaction only).
    bool buildHit(const EngineHit& e, const cc::Entity& atk, const EntityView& av, const cc::Entity& vic, const EntityView& vv, cc::HitEvent& out, bool& touch, uint32_t& centi);
    void forwardHit(const cc::HitEvent& h, PlayerId owner, bool touch = false, uint32_t centi = 0);
    void noteNaturalLethal(const cc::Entity& who);   // owner: after a natural hit's vitals went out, a victim at health <= 0 must get a life state death

    // inbound
    void dispatch(PlayerId from, const Envelope& e);
    void routeAsHost(PlayerId from, const Envelope& e);
    void onHit(PlayerId from, const Envelope& e, const MHit& m);
    void onNack(const MHitNack& m);
    void onVitals(const Envelope& e, const MVitals& m);
    void onDeathMsg(PlayerId from, const Envelope& e, const MDeath& m);
    void onProjectile(PlayerId from, const Envelope& e, const MProjectile& m);
    void onExplosion(PlayerId from, const Envelope& e, const MExplosion& m);
    void onCrime(PlayerId from, const MCrime& m);
    void onHostility(PlayerId from, const MHostility& m);

    // owner side
    void applyAccepted(const cc::HitEvent& h, PlayerId sender, bool touch = false, uint32_t centi = 0);
    void applyLight(const cc::HitEvent& h, const EntityView& vv, const EntityView& av, bool haveAtk, float amount);   // owner: a blow of under one point of health, or none
    cc::VictimVitals vitalsOf(const cc::Entity& e, const EntityView& v);
    void publishVitals(const cc::Entity& e, uint64_t cause);
    void recordHit(const cc::Entity& victim, const cc::Entity& attacker, const FormRef& weapon, cc::Limb limb, bool natural);
    cc::DeathVerdict judgeDeath(const cc::DeathDecl& d, PlayerId sender, bool senderIsHost);
    void handleDeathAccepted(const cc::DeathDecl& d);
    void awardXp(const cc::DeathDecl& d);
    bool killWasNative(const cc::DeathDecl& d) const;    // the local engine already paid this kill (the local player's own hit on an actor simulated here)
    void socialDeath(const cc::DeathDecl& d);            // plugin/companions: the kill report (declarer only) and the murder (the killer's machine only)
    // plugin/companions: an attack by a player or a companion on a faction member, reported by the machine that simulates the attacker. Bursts are throttled.
    void socialCrime(const cc::Entity& attacker, const EntityView& av, const cc::Entity& victim, const EntityView& vv, bool killed, uint8_t nativeMask);
    bool protectedBlocks(uint32_t victimRef, PlayerId attacker, bool splash);   // G40
    bool socialNeutralizes(uint32_t attackerRef, uint32_t victimRef);   // plugin/companions: the hit stays inside the party
    void maybeReportCrime(const cc::HitEvent& h, const EntityView& vv, bool killed);
    void decorateIfDead(const cc::Entity& who, const cc::DeathDecl& d);

    // explosions
    uint64_t explosionIdFor(const EngineHit& e, const cc::Entity& atk, const EntityView& av);
    bool isVisualBlast(const EngineHit& e) const;
    void planBlast(Blast& b);

    // plumbing
    void remember(const char* what, const std::string& detail);
    cc::LimbState& limbsOf(const cc::Entity& e, const EntityView& v);

    Config cfg_;
    IWorldView& world_;
    ICombatEngine& eng_;
    ICombatNet& net_;
    std::unique_ptr<Adapter> adapter_;
    cc::HitValidator validator_;
    cc::DeathLedger deaths_;
    cc::ProjectileRegistry projectiles_;
    cc::ExplosionLedger explosions_;
    cc::XpLedger xp_;
    cc::HostilityBook hostility_;
    std::map<uint64_t, Inflight> inflight_;
    std::map<cc::Entity, LastHit> lastHit_;
    std::map<cc::Entity, cc::LimbState> limbs_;                 // owner side: accumulated limb damage + crippled bits
    std::map<cc::Entity, std::pair<OwnerEpoch, uint32_t>> vitalsSeq_;     // sender side counter (epoch, seq)
    std::map<cc::Entity, std::pair<OwnerEpoch, uint32_t>> vitalsSeen_;    // receiver side newest
    std::map<cc::Entity, VitalsDirty> vitalsDirty_;
    std::map<uint64_t, ExpId> explosionIds_;                    // engine explosion identity -> explosion id (kept a few seconds: late hits of the same blast)
    std::map<uint64_t, Blast> blasts_;                          // explosion id -> pending plan
    std::map<cc::Entity, PendingDecor> decor_;
    struct KillCredit { uint32_t killerRef = 0; uint64_t until = 0; };
    std::map<cc::Entity, KillCredit> killCredit_;               // host: victim -> the remote player's puppet that killed it (until the copy's kill happened here)
    uint32_t remoteKillerRef(const cc::Entity& killer) const;   // host: the puppet of a remote player, 0 for the local player / actors / unknown
    std::map<std::tuple<PlayerId, uint8_t, FormRef>, uint64_t> socialCrimeAt_;   // (criminal, murder?, faction) -> when it was last reported (burst throttle)
    std::vector<DeferredDeath> deferredDeaths_;
    struct PendingKill { cc::Entity who; uint64_t dueAt = 0; bool natural = false; };   // natural: the engine's own hit took the health, not our replay
    std::vector<PendingKill> pendingKills_;                     // owner side: victims an accepted lethal hit must have left dead
    std::vector<VisualBlast> visualBlasts_;                     // explosions shown here for another machine: their hits are harmless
    uint64_t visualProjectileUntil_ = 0;                        // a visual projectile was spawned recently: sourceless projectile hits are harmless
    std::set<PlayerId> joined_;
    std::deque<std::string> rejections_;
    std::string lastGate_;                                      // the last gate decision that was not a plain proceed on a non-actor (diagnostics)
    uint64_t now_ = 0, counter_ = 0;
    bool localPlayerDead_ = false;
    Stats stats_;
};

}  // namespace nvcoop::combat
