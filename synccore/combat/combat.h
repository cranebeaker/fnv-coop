// combat-core: game-free combat rules (FULL-SYNC-PLAN 3.4). C++17, no engine, no network, no clock: time and world facts are passed in.
//
// The model (one sentence per rule, all enforced by code in combat.cpp and by unit tests with mutation twins):
//   * A HIT is resolved ONCE, by the machine that OWNS THE VICTIM (an NPC's owner, or a player's own machine). Everyone else forwards it.
//   * The victim's owner runs HitValidator: it is the owner, the hit is new (idempotent by hitId), the victim is alive, damage and range are
//     in bounds, the attacker is who the sender says (a player hit comes from that player; an NPC hit from that NPC's owner at the NPC's
//     CURRENT epoch), same or adjacent cell, within weapon range, impact point near the victim, rate of fire per weapon, not too old.
//   * The accepted hit becomes a DamageOutcome (health, limb crippling, death or protection) that the owner publishes as one event.
//   * DEATH is declared once, by the owner (or by the story host for named NPCs), idempotent by actor id and by declaration id (DeathLedger).
//   * PROJECTILES are spawned by the shooter's owner; every other machine gets a VISUAL-ONLY copy that must never deal damage.
//   * EXPLOSIONS are declared by the shooter's owner; the damage goes out as one HitEvent per victim to THAT VICTIM'S owner (planExplosionHits).
//   * Kill XP goes to the killing blow, or is shared by the party (setting). A death awards XP once (XpLedger).
//   * Crime / hostility is host-authoritative: HostilityBook decides SetEnemy / SetAlly effects, clients only report.
//
// Used by: synccore/enginemodel (the fake engine, so the fuzzer exercises it) and plugin/combat (engine glue, uses these types and
// plugin/actors/engine/actor_engine_api.h for "who owns this ref"). Everything engine-facing is behind ICombatWorld.
//
// Wire: every struct below has a symmetric io(Archive&, T&) (synccore/include/fnvcoop/wire.h). kCombatWireVersion is bumped on any change.
#pragma once
#include <cstdint>
#include <deque>
#include <map>
#include <set>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <variant>
#include <vector>
#include "fnvcoop/ownership.h"
#include "fnvcoop/types.h"
#include "fnvcoop/wire.h"
#include "../live/entity.h"   // one NetId space: players are playerNet(slot) (docs/entity-core-plan.md S7)

namespace fnvcoop::combat {

constexpr uint8_t kCombatWireVersion = 2;   // 2: an Entity is written as its bare NetId (players playerNet(slot))

using CellId = uint32_t;
using NetId = ActorNetId;   // network id of an actor (placed ref: plugin+local id mapped by the actor layer; created ref: 0xFF prefix)

// ------------------------------------------------------------------------------------------------ basic types
struct Vec3 {
    float x = 0, y = 0, z = 0;
    friend bool operator==(const Vec3& a, const Vec3& b) { return a.x == b.x && a.y == b.y && a.z == b.z; }
    friend bool operator!=(const Vec3& a, const Vec3& b) { return !(a == b); }
    bool zero() const { return x == 0 && y == 0 && z == 0; }
};
float distance(const Vec3& a, const Vec3& b);

// Body parts of an actor. Values are the wire values (also the bit index in a crippled-limb mask). Torso cannot be crippled.
enum class Limb : uint8_t { Torso = 0, Head, LeftArm, RightArm, LeftLeg, RightLeg, Count_ };
constexpr uint8_t kLimbCount = static_cast<uint8_t>(Limb::Count_);
const char* toString(Limb l);

enum class EntityKind : uint8_t { None = 0, Player = 1, Actor = 2 };
// A combatant: a player (id = slot) or an NPC / creature (id = NetId).
struct Entity {
    EntityKind kind = EntityKind::None;
    uint32_t id = 0;
    static Entity player(PlayerId slot) { return {EntityKind::Player, slot}; }
    static Entity actor(NetId id) { return {EntityKind::Actor, id}; }
    bool isPlayer() const { return kind == EntityKind::Player; }
    bool isActor() const { return kind == EntityKind::Actor; }
    bool none() const { return kind == EntityKind::None; }
    // The wire and the EntityMap name everything by NetId: a player is fnvcoop::live::playerNet(slot), an actor its own id, 0 nobody.
    static Entity fromNet(uint32_t net) {
        if (!net) return {};
        if (fnvcoop::live::isPlayerNet(net)) return player(fnvcoop::live::slotOfPlayerNet(net));
        return actor(net);
    }
    uint32_t netId() const { return isPlayer() ? fnvcoop::live::playerNet(static_cast<PlayerId>(id)) : isActor() ? id : 0; }
    PlayerId slot() const { return isPlayer() && id < kMaxSlots ? static_cast<PlayerId>(id) : kNoPlayer; }
    friend bool operator==(const Entity& a, const Entity& b) { return a.kind == b.kind && a.id == b.id; }
    friend bool operator!=(const Entity& a, const Entity& b) { return !(a == b); }
    friend bool operator<(const Entity& a, const Entity& b) { return a.kind != b.kind ? a.kind < b.kind : a.id < b.id; }
};
std::string toString(const Entity& e);
void io(Archive& a, Entity& e);   // the wire form inside every combat message (v2: the bare NetId)

// ------------------------------------------------------------------------------------------------ rules (tunable) and weapon table
struct WeaponInfo {
    float range = 6000;           // longest distance a hit with this weapon can land from (units; melee: reach)
    int32_t maxDamage = 500;      // largest single hit (after the shooter's perks; exceeding it is a forgery)
    uint32_t minIntervalMs = 0;   // fastest time between shots/swings; 0 = unknown (the global per-attacker cap applies)
    uint32_t pellets = 1;         // hits one shot can produce (shotgun pellets, burst per trigger pull)
    bool melee = false;
    bool explosive = false;       // detonates: its hits arrive as explosion hits
};

struct CombatRules {
    float maxWeaponRange = 6000;       // upper bound for any claimed range when the weapon is unknown
    float rangeSlack = 700;            // attacker and victim positions are from different frames: allow this much extra distance
    float replicaSlack = 400;          // ... plus this when the attacker is not simulated here (its position is an interpolated copy)
    float impactSlack = 400;           // impact point may be this far from the victim's position (plus the victim's radius)
    int32_t maxDamage = 500;           // upper bound for any hit when the weapon is unknown
    uint32_t maxHitsPerSec = 20;       // per attacker, all weapons
    uint32_t rateBurstSlack = 2;       // extra hits tolerated per window on top of the weapon's own rate
    uint32_t maxHitAgeMs = 3000;       // a hit stamped older than this (host-time ms) is stale
    uint32_t futureSlackMs = 500;      // ... or newer than this ahead of now
    bool requireTick = false;          // true: a hit with firedAtTick == 0 is rejected (production); false: 0 = unstamped, skip the age check
    bool requireKnownProjectile = false;   // true: a hit that names a projectile the world does not know is rejected
    bool allowSelfHit = false;         // direct hits on yourself (explosion self damage is always allowed)
    // projectiles
    float maxProjectileSpeed = 20000;  // units / s
    float projectileOriginSlack = 600; // spawn point may be this far from the shooter
    uint32_t maxProjectilesPerSec = 40;
    // explosions
    float maxExplosionRadius = 3000;
    float explosionFalloff = 1.0f;     // damage at the edge = damage * (1 - falloff); 1 = linear to zero, 0 = no falloff
    // limbs
    int32_t crippleDamage[kLimbCount] = {0, 30, 25, 25, 25, 25};   // accumulated limb damage that cripples it; 0 = cannot be crippled (torso)
    // xp
    int32_t partyXpMinShare = 0;       // PartyShare: every present player gets at least this
};
CombatRules defaultRules();

// ------------------------------------------------------------------------------------------------ world facts (what the engine glue / fake engine answers)
struct ActorFacts {
    bool known = false;
    Entity who;
    bool alive = true;
    bool named = false;            // story NPC: only a story KillActor may kill it
    bool essential = false;        // cannot die from damage (health floors at 1) but is not story-killable by combat either
    CellId cell = 0;
    Vec3 pos;
    float radius = 0;              // body size added to impact checks
    PlayerId owner = kNoPlayer;    // actor: its owner; player: its own slot
    OwnerEpoch epoch = 0;          // actor: owner epoch; player: the epoch of its pinned entity (0 = not known)
    bool handingOff = false;       // an ownership handoff is in flight (the new owner decides, not us)
    int32_t health = 0, maxHealth = 0;
    uint8_t limbMask = 0;
    PlayerId recruiter = kNoPlayer;   // companions: kill XP and crime go to the recruiter
};

class ICombatWorld {
public:
    virtual ~ICombatWorld() = default;
    virtual bool facts(const Entity& e, ActorFacts& out) const = 0;
    virtual bool cellsAdjacent(CellId a, CellId b) const { return a == b; }
    virtual const WeaponInfo* weapon(const FormRef& /*id*/) const { return nullptr; }
    // Is there a projectile with this id fired by someone? (fills `shooter`). Default: unknown.
    virtual bool projectile(uint64_t /*id*/, Entity& /*shooter*/) const { return false; }
    // Is the straight line between two points free? (engine raycast; the fake says yes)
    virtual bool lineOfFire(const Vec3& /*from*/, const Vec3& /*to*/) const { return true; }
};

// ------------------------------------------------------------------------------------------------ hits
// Captured on the ATTACKER's machine at fire time (xNVSE onhit gives attacker, victim, damage, limb, impact, weapon, ammo).
struct HitEvent {
    uint64_t hitId = 0;            // unique per hit (attacker slot << 48 | counter), the idempotency key
    Entity attacker;
    Entity victim;
    int32_t damage = 0;            // health damage as the attacker's machine computed it (perks, weapon, limb multiplier); armor of the VICTIM is applied by the victim's owner
    Limb limb = Limb::Torso;
    Vec3 impactPoint;              // zero = unknown (skips the impact check)
    FormRef weapon;                // empty plugin = unknown weapon
    FormRef ammo;
    uint64_t projectile = 0;       // id of the projectile that carried it (0 = hitscan / melee)
    uint64_t explosionId = 0;      // non-zero: this hit is explosion damage (impact point = blast centre, range = radius)
    uint32_t firedAtTick = 0;      // host-time ms when the shot was fired (0 = unstamped)
    OwnerEpoch attackerEpoch = 0;  // the attacker's owner epoch as the reporter knows it (a player: its pinned entity's; 0 = not known)
    float claimedRange = 0;        // weapon range the attacker claims (used only when the weapon is not in the table; 0 = none)
    bool critical = false;         // informational (kill-cam, crit chance already inside `damage`)
    bool vats = false;             // informational: VATS shot resolved in real time like any other
};
void io(Archive& a, HitEvent& h);

enum class HitReason : uint8_t {
    Ok = 0,
    Duplicate,          // hitId already judged
    UnknownVictim,
    NotVictimOwner,     // this machine does not own the victim: forward to verdict.forwardTo
    HandoffInProgress,  // the victim is changing owner: the new owner decides
    VictimDead,
    DamageOutOfBounds,
    WeaponRangeOutOfBounds,
    TooOld,             // firedAtTick older than maxHitAgeMs
    FromFuture,         // firedAtTick ahead of now (beyond slack)
    AttackerNotSender,  // a player attacker that is not the sender
    AttackerUnknown,
    AttackerDead,
    ForgedAttacker,     // an NPC attacker the sender does not own
    StaleAttackerEpoch, // an NPC attacker reported under an older owner epoch
    FutureAttackerEpoch,// the reporter knows a newer epoch than we do (ask for a resync, do not apply)
    OtherCell,
    OutOfRange,
    ImpactNotNearVictim,
    NoLineOfFire,
    RateOfFire,
    SelfHit,
    UnknownProjectile,
    ProjectileShooterMismatch,
    ExplosionOutOfRadius,
    Count_
};
const char* toString(HitReason r);

struct HitVerdict {
    bool apply = false;
    HitReason reason = HitReason::Ok;
    PlayerId forwardTo = kNoPlayer;   // NotVictimOwner: the owner the hit should be sent to
    bool resync = false;              // FutureAttackerEpoch: the ownership table here is behind
    int32_t damage = 0;               // what to apply (== hit.damage when accepted)
    std::string detail;               // human readable (first problem)
};

// The victim owner's judge. Stateful: it remembers hit ids (idempotency) and per-attacker/weapon fire times (rate of fire).
class HitValidator {
public:
    explicit HitValidator(CombatRules rules = defaultRules(), size_t rememberIds = 8192) : rules_(rules), cap_(rememberIds) {}
    // `sender` = the machine/player the hit message came from (its author, even when the host relayed it); `self` = this machine's slot;
    // `nowMs` = host-time estimate on this machine (same clock as firedAtTick).
    HitVerdict judge(const HitEvent& h, PlayerId sender, PlayerId self, uint64_t nowMs, const ICombatWorld& w);
    const CombatRules& rules() const { return rules_; }
    void setRules(const CombatRules& r) { rules_ = r; }
    void reset();   // new session: forget ids and rates
    uint64_t judged() const { return judged_; }
    uint64_t rejected() const { return rejected_; }
    bool seen(uint64_t hitId) const { return seen_.count(hitId) != 0; }
private:
    bool remember(uint64_t id);   // false if already known
    bool rateOk(const Entity& attacker, const FormRef& weapon, uint64_t nowMs, const ICombatWorld& w);
    CombatRules rules_;
    size_t cap_;
    std::unordered_set<uint64_t> seen_;
    std::deque<uint64_t> order_;
    std::map<std::pair<Entity, uint64_t>, std::deque<uint64_t>> times_;   // (attacker, weapon key) -> hit times (ms) in the last second
    uint64_t judged_ = 0, rejected_ = 0;
};

// ------------------------------------------------------------------------------------------------ limbs and damage application
// Crippled limbs only grow (a union is idempotent and order independent); healing is an explicit story/host effect (restore()).
struct LimbState {
    uint8_t crippled = 0;                   // bit i = Limb i crippled
    int32_t damage[kLimbCount] = {0, 0, 0, 0, 0, 0};   // accumulated damage per limb (owner side only; not sent)
    bool isCrippled(Limb l) const { return (crippled >> static_cast<unsigned>(l)) & 1u; }
    uint8_t mask() const { return crippled; }
    static uint8_t bit(Limb l) { return static_cast<uint8_t>(1u << static_cast<unsigned>(l)); }
    void restore(uint8_t mask) { crippled = static_cast<uint8_t>(crippled & ~mask); for (unsigned i = 0; i < kLimbCount; ++i) if ((mask >> i) & 1u) damage[i] = 0; }
    friend bool operator==(const LimbState& a, const LimbState& b) { return a.crippled == b.crippled; }
};
// Adds limb damage; returns the bits that became crippled by this call.
uint8_t applyLimbDamage(LimbState& s, Limb limb, int32_t damage, const CombatRules& rules);
// Union of crippled masks (what a receiver does with a published limb mask). Never un-cripples.
uint8_t mergeLimbMask(uint8_t have, uint8_t incoming);

struct VictimVitals {
    int32_t health = 100, maxHealth = 100;
    bool dead = false;
    bool named = false;        // story NPC
    bool essential = false;
    LimbState limbs;
};
struct DamageOutcome {
    bool applied = false;      // false: the victim was already dead
    int32_t healthBefore = 0, healthAfter = 0;
    uint8_t newlyCrippled = 0;
    bool died = false;         // health reached zero and nothing protected the victim: the owner must now declare death
    bool protectedFromDeath = false;   // named / essential: health floored at 1 instead
};
// Applies one ACCEPTED hit to the victim's vitals. Pure and deterministic.
DamageOutcome applyHit(VictimVitals& v, const HitEvent& h, const CombatRules& rules);

// ------------------------------------------------------------------------------------------------ death
enum class DeathSource : uint8_t { Owner = 0, StoryHost = 1, SelfPlayer = 2 };
struct DeathDecl {
    uint64_t deathId = 0;          // unique per declaration (declarer slot << 48 | counter); replays are dropped
    Entity who;
    DeathSource source = DeathSource::Owner;
    PlayerId declarer = kNoPlayer; // author of the declaration
    OwnerEpoch epoch = 0;          // owner epoch the declarer acted under (Owner source)
    Entity killer;                 // last attacker (none = environment / script)
    FormRef weapon;
    uint8_t limbMask = 0;          // crippled limbs at death (the corpse shows them)
    CellId cell = 0;
    Vec3 where;                    // bodies are placed where they fell; ragdolls are visual only
    uint32_t tick = 0;
};
void io(Archive& a, DeathDecl& d);

enum class DeathReason : uint8_t {
    Ok = 0,
    AlreadyDead,        // idempotent: this actor's death was already recorded (not an error, nothing to do)
    ReplayedDeclaration,// this exact declaration id was already seen
    UnknownActor,
    NotOwner,           // declarer does not own the actor
    StaleEpoch,
    FutureEpoch,        // declarer knows a newer epoch (resync)
    NamedNeedsStory,    // a story NPC may only be killed by the story host
    StoryNeedsHost,     // StoryHost source from a machine that is not the host
    SelfOnly,           // a player's death may only be declared by that player (or the host)
    Count_
};
const char* toString(DeathReason r);
struct DeathVerdict {
    bool accept = false;           // true only for the FIRST valid declaration: spawn corpse/loot, award XP, release ownership exactly once
    DeathReason reason = DeathReason::Ok;
    bool resync = false;
};
class DeathLedger {
public:
    explicit DeathLedger(size_t rememberIds = 8192) : cap_(rememberIds) {}
    DeathVerdict judge(const DeathDecl& d, PlayerId sender, bool senderIsHost, const ICombatWorld& w);
    bool isDead(const Entity& e) const { return dead_.count(e) != 0; }
    const DeathDecl* record(const Entity& e) const { auto it = dead_.find(e); return it == dead_.end() ? nullptr : &it->second; }
    // Story resurrect / respawn / new player life: the entity can die again.
    void resurrect(const Entity& e) { dead_.erase(e); }
    // Records a death that is already a fact (a story KillActor that was applied, a corpse found in a loaded world) without judging it.
    void markDead(const DeathDecl& d) { dead_[d.who] = d; remember(d.deathId); }
    size_t deaths() const { return dead_.size(); }
    void reset() { dead_.clear(); ids_.clear(); order_.clear(); }
private:
    bool remember(uint64_t id);
    std::map<Entity, DeathDecl> dead_;
    std::unordered_set<uint64_t> ids_;
    std::deque<uint64_t> order_;
    size_t cap_;
};

// ------------------------------------------------------------------------------------------------ projectiles
struct ProjectileSpawn {
    uint64_t id = 0;               // (shooter's owner slot << 48 | counter)
    Entity shooter;
    OwnerEpoch shooterEpoch = 0;
    FormRef weapon, ammo, projectileBase;
    CellId cell = 0;
    Vec3 origin;
    Vec3 velocity;                 // units / s
    uint32_t firedAtTick = 0;
    uint32_t lifetimeMs = 10000;
    bool explosive = false;        // grenade / rocket / mine: detonates into an ExplosionEvent
};
void io(Archive& a, ProjectileSpawn& p);

enum class ProjectileRole : uint8_t {
    Rejected = 0,
    Authoritative,   // this machine owns the shooter: the real projectile, real hits, real explosion
    VisualOnly       // another machine's shot: a copy for the eyes; it must NEVER deal damage here (mayDealDamage == false)
};
inline bool mayDealDamage(ProjectileRole r) { return r == ProjectileRole::Authoritative; }
enum class ProjectileReason : uint8_t { Ok = 0, Duplicate, UnknownShooter, NotShooterOwner, StaleEpoch, FutureEpoch, SpeedOutOfBounds, OriginTooFar, RateOfFire, Expired, Count_ };
const char* toString(ProjectileReason r);
struct ProjectileVerdict {
    bool accept = false;
    ProjectileRole role = ProjectileRole::Rejected;
    ProjectileReason reason = ProjectileReason::Ok;
    bool resync = false;
};
class ProjectileRegistry {
public:
    explicit ProjectileRegistry(CombatRules rules = defaultRules(), size_t rememberIds = 4096) : rules_(rules), cap_(rememberIds) {}
    // Local fire (sender == self) or a spawn event received from `sender`.
    ProjectileVerdict onSpawn(const ProjectileSpawn& p, PlayerId sender, PlayerId self, uint64_t nowMs, const ICombatWorld& w);
    bool live(uint64_t id) const { return live_.count(id) != 0; }
    const ProjectileSpawn* find(uint64_t id) const { auto it = live_.find(id); return it == live_.end() ? nullptr : &it->second.spawn; }
    ProjectileRole roleOf(uint64_t id) const { auto it = live_.find(id); return it == live_.end() ? ProjectileRole::Rejected : it->second.role; }
    void expire(uint64_t nowMs);          // drop projectiles past their lifetime
    void remove(uint64_t id) { live_.erase(id); }
    size_t liveCount() const { return live_.size(); }
    void reset() { live_.clear(); seen_.clear(); order_.clear(); times_.clear(); }
    // ICombatWorld::projectile() helper
    bool shooterOf(uint64_t id, Entity& out) const { auto it = live_.find(id); if (it == live_.end()) return false; out = it->second.spawn.shooter; return true; }
private:
    struct Live { ProjectileSpawn spawn; ProjectileRole role; uint64_t diesAtMs; };
    CombatRules rules_;
    size_t cap_;
    std::unordered_map<uint64_t, Live> live_;
    std::unordered_set<uint64_t> seen_;
    std::deque<uint64_t> order_;
    std::map<Entity, std::deque<uint64_t>> times_;
};

// ------------------------------------------------------------------------------------------------ explosions
struct ExplosionEvent {
    uint64_t id = 0;               // (shooter's owner slot << 48 | counter): one detonation = one id
    Entity source;                 // who gets credit
    OwnerEpoch sourceEpoch = 0;
    uint64_t projectile = 0;       // 0 = placed mine / scripted
    FormRef weapon, explosionBase;
    CellId cell = 0;
    Vec3 pos;
    float radius = 0;
    int32_t damage = 0;            // at the centre
    uint32_t tick = 0;
    uint32_t firedAtTick = 0;
};
void io(Archive& a, ExplosionEvent& e);

// Who a blast may hurt (the declaring machine lists the actors/players it knows within the radius).
struct BlastCandidate { Entity victim; Vec3 pos; CellId cell = 0; float radius = 0; };
// One HitEvent per victim in range, damage falling off with distance. hitId is derived from (explosion id, victim), so a replay or a second
// planning pass produces the SAME ids and is dropped as a duplicate by every HitValidator. Each hit goes to the owner of its victim.
std::vector<HitEvent> planExplosionHits(const ExplosionEvent& e, const std::vector<BlastCandidate>& candidates, const CombatRules& rules);
uint64_t explosionHitId(uint64_t explosionId, const Entity& victim);

enum class ExplosionReason : uint8_t { Ok = 0, Duplicate, UnknownSource, NotSourceOwner, StaleEpoch, FutureEpoch, RadiusOutOfBounds, DamageOutOfBounds, Count_ };
const char* toString(ExplosionReason r);
struct ExplosionVerdict {
    bool accept = false;
    ExplosionReason reason = ExplosionReason::Ok;
    bool authoritative = false;     // this machine is the source's owner: it plans the damage. Others: visual only, no damage here.
    bool resync = false;
};
class ExplosionLedger {
public:
    explicit ExplosionLedger(CombatRules rules = defaultRules(), size_t rememberIds = 4096) : rules_(rules), cap_(rememberIds) {}
    ExplosionVerdict judge(const ExplosionEvent& e, PlayerId sender, PlayerId self, const ICombatWorld& w);
    bool seen(uint64_t id) const { return ids_.count(id) != 0; }
    void reset() { ids_.clear(); order_.clear(); }
private:
    CombatRules rules_;
    size_t cap_;
    std::unordered_set<uint64_t> ids_;
    std::deque<uint64_t> order_;
};

// ------------------------------------------------------------------------------------------------ kill XP
enum class XpMode : uint8_t { KillingBlow = 0, PartyShare = 1 };
struct KillRecord {
    uint64_t deathId = 0;
    Entity victim;
    Entity killer;                 // last attacker (player or NPC; a companion's kill credits its recruiter)
    PlayerId killerRecruiter = kNoPlayer;   // killer is a companion: the player it follows
    int32_t xp = 0;                // XP value of the victim
    std::vector<PlayerId> contributors;   // players who damaged the victim (informational, used by PartyShare only through `present`)
};
struct XpAward { PlayerId slot = kNoPlayer; int32_t xp = 0; };
// Who gets how much. KillingBlow: the killing player (or the recruiter of a killing companion). No player killer: nobody (an NPC killed it).
// PartyShare: `present` split equally, the remainder going to the killer (or the lowest slot). Sum of awards <= xp, never negative.
std::vector<XpAward> attributeKillXp(const KillRecord& k, XpMode mode, const std::vector<PlayerId>& present, const CombatRules& rules);
// One kill awards once, even if the death message is replayed or two machines both report it.
class XpLedger {
public:
    explicit XpLedger(size_t remember = 8192) : cap_(remember) {}
    std::vector<XpAward> award(const KillRecord& k, XpMode mode, const std::vector<PlayerId>& present, const CombatRules& rules);
    void reset() { ids_.clear(); order_.clear(); }
private:
    std::unordered_set<uint64_t> ids_;
    std::deque<uint64_t> order_;
    size_t cap_;
};

// ------------------------------------------------------------------------------------------------ crime and hostility (host-authoritative)
enum class HostilityKind : uint8_t { SetEnemy = 0, SetAlly = 1, StopCombat = 2 };
struct HostilityEffect {
    uint64_t id = 0;               // idempotency id (derived from the decision, identical on replay)
    HostilityKind kind = HostilityKind::SetEnemy;
    FormRef factionA, factionB;    // SetEnemy / SetAlly: the two factions (player's faction first)
    Entity actor;                  // StopCombat: the actor to calm
    PlayerId cause = kNoPlayer;    // the player whose action caused it
    friend bool operator==(const HostilityEffect& a, const HostilityEffect& b) {
        return a.id == b.id && a.kind == b.kind && a.factionA == b.factionA && a.factionB == b.factionB && a.actor == b.actor && a.cause == b.cause;
    }
};
void io(Archive& a, HostilityEffect& e);
// A client's (or the host's own) report: "player X attacked/killed something of faction F, seen by witnesses".
struct CrimeReport {
    PlayerId criminal = kNoPlayer;
    Entity victim;
    FormRef victimFaction;         // empty = no faction
    FormRef criminalFaction;       // the player's faction (usually the player faction)
    bool victimWasHostile = false; // attacking a hostile is not a crime
    bool killed = false;
    bool witnessed = false;        // any player's detection counts
    PlayerId reporter = kNoPlayer;
};
class HostilityBook {
public:
    // Host only. Returns the effects to commit (empty when nothing changes). Escalation is idempotent: the same (criminal faction, victim faction)
    // turns hostile once; later crimes against that faction produce nothing new.
    std::vector<HostilityEffect> onCrime(const CrimeReport& r, PlayerId self, PlayerId sender);
    // Peace (quest, bribe, reputation): turns an enemy faction pair back into allies. Host only.
    std::vector<HostilityEffect> makePeace(const FormRef& a, const FormRef& b, PlayerId cause, PlayerId self);
    bool hostile(const FormRef& a, const FormRef& b) const;
    // Everyone: a received effect is accepted only from the host; replays are dropped. Returns true if it should be applied now.
    bool accept(const HostilityEffect& e, PlayerId sender);
    void reset() { pairs_.clear(); applied_.clear(); gen_.clear(); }
private:
    static std::pair<FormRef, FormRef> key(const FormRef& a, const FormRef& b) { return a < b ? std::make_pair(a, b) : std::make_pair(b, a); }
    std::map<std::pair<FormRef, FormRef>, bool> pairs_;   // true = enemies
    std::set<uint64_t> applied_;
    std::map<std::pair<FormRef, FormRef>, uint64_t> gen_;   // how often a pair flipped: part of the effect id, so a later flip is not mistaken for a replay
};

// ------------------------------------------------------------------------------------------------ wire bundle and the per-machine node
// One datagram payload of the combat channel: [kCombatWireByte][kCombatWireVersion][kind u8][Archive body]. The first byte keeps it apart from the
// other modules on the coop mux (actors use 0xAC). Routing is the glue's job: a HitEvent goes to the owner of its victim, a DeathDecl / ProjectileSpawn /
// ExplosionEvent / HostilityEffect to everyone (via the host).
constexpr uint8_t kCombatWireByte = 0xCB;
using CombatMsg = std::variant<HitEvent, DeathDecl, ProjectileSpawn, ExplosionEvent, HostilityEffect>;
std::vector<uint8_t> encodeMsg(const CombatMsg& m);                         // empty if a value cannot be encoded (non finite float)
bool decodeMsg(const uint8_t* data, size_t size, CombatMsg& out);           // false on anything malformed, wrong version or trailing bytes
const char* msgName(const CombatMsg& m);

// Everything one machine keeps for combat, so the glue owns exactly one object.
struct CombatNode {
    CombatRules rules;
    HitValidator hits;
    DeathLedger deaths;
    ProjectileRegistry projectiles;
    ExplosionLedger explosions;
    XpLedger xp;
    HostilityBook hostility;
    explicit CombatNode(CombatRules r = defaultRules()) : rules(r), hits(r), projectiles(r), explosions(r) {}
    void setRules(const CombatRules& r) { rules = r; hits.setRules(r); projectiles = ProjectileRegistry(r); explosions = ExplosionLedger(r); }
    void reset() { hits.reset(); deaths.reset(); projectiles.reset(); explosions.reset(); xp.reset(); hostility.reset(); }   // a new world (load, rejoin)
};

// Console lines the engine glue can run (ExecOn) to apply a hostility effect when no direct engine call is known (TODO-RE: direct faction relation call).
std::string consoleLineFor(const HostilityEffect& e, const std::string& factionAEditorId, const std::string& factionBEditorId);

}  // namespace fnvcoop::combat
