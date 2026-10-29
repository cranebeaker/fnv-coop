// combat-core implementation. See combat.h for the model. Mutation twins (runtime style, tools/mut/fnvcoop_mut.h) use numbers 301..362:
// each FNV_MUT(n) below is a deliberately broken variant that the unit tests in tests/ must catch.
#include "combat.h"
#include <algorithm>
#include <cmath>
#include <functional>
#include "../../tools/mut/fnvcoop_mut.h"

namespace fnvcoop::combat {

// ------------------------------------------------------------------------------------------------ basics
float distance(const Vec3& a, const Vec3& b) {
    const float dx = a.x - b.x, dy = a.y - b.y, dz = a.z - b.z;
    return std::sqrt(dx * dx + dy * dy + dz * dz);
}

const char* toString(Limb l) {
    switch (l) {
        case Limb::Torso: return "torso";
        case Limb::Head: return "head";
        case Limb::LeftArm: return "left_arm";
        case Limb::RightArm: return "right_arm";
        case Limb::LeftLeg: return "left_leg";
        case Limb::RightLeg: return "right_leg";
        default: return "?";
    }
}

std::string toString(const Entity& e) {
    switch (e.kind) {
        case EntityKind::Player: return "player#" + std::to_string(e.id);
        case EntityKind::Actor: return "actor#" + std::to_string(e.id);
        default: return "none";
    }
}

CombatRules defaultRules() { return CombatRules{}; }

namespace {

uint64_t weaponKey(const FormRef& w) {
    if (w.plugin.empty()) return 0;
    return hash64(w.plugin.data(), w.plugin.size(), 1469598103934665603ull) ^ (static_cast<uint64_t>(w.localId) * 0x9E3779B97F4A7C15ull);
}

// Small helpers for io of enums / entities / vectors.
template <class E>
void ioEnum(Archive& a, E& e, uint8_t maxExclusive) {
    uint8_t v = static_cast<uint8_t>(e);
    a.u8(v);
    if (a.reading()) {
        if (v >= maxExclusive) { a.fail(); return; }
        e = static_cast<E>(v);
    }
}

void ioEntity(Archive& a, Entity& e) {   // wire v2: the bare NetId (players playerNet(slot)), 0 = nobody
    uint32_t net = e.netId();
    a.varu32(net);
    if (!a.reading()) return;
    if (fnvcoop::live::kindOf(net) == fnvcoop::live::EntityKind::None && net != 0) { a.fail(); return; }   // the reserved part of 0xFE..
    if (fnvcoop::live::isPlayerNet(net) && fnvcoop::live::slotOfPlayerNet(net) >= kMaxSlots) { a.fail(); return; }
    e = Entity::fromNet(net);
}

void ioVec(Archive& a, Vec3& v) {
    a.f32(v.x);
    a.f32(v.y);
    a.f32(v.z);
}

// Remember an id in a bounded set. Returns false if it was already known.
bool rememberId(std::unordered_set<uint64_t>& set, std::deque<uint64_t>& order, size_t cap, uint64_t id) {
    if (!set.insert(id).second) return false;
    order.push_back(id);
    while (order.size() > cap) { set.erase(order.front()); order.pop_front(); }
    return true;
}

// Serial comparison of the 32-bit host-time stamp with now.
int32_t ageMs(uint64_t nowMs, uint32_t stamp) { return static_cast<int32_t>(static_cast<uint32_t>(nowMs) - stamp); }

}  // namespace

// ------------------------------------------------------------------------------------------------ wire
void io(Archive& a, Entity& e) { ioEntity(a, e); }

void io(Archive& a, HitEvent& h) {
    a.u64(h.hitId);
    ioEntity(a, h.attacker);
    ioEntity(a, h.victim);
    a.vari32(h.damage);
    ioEnum(a, h.limb, kLimbCount);
    ioVec(a, h.impactPoint);
    io(a, h.weapon);
    io(a, h.ammo);
    a.u64(h.projectile);
    a.u64(h.explosionId);
    a.u32(h.firedAtTick);
    a.varu32(h.attackerEpoch);
    a.f32(h.claimedRange);
    a.boolean(h.critical);
    a.boolean(h.vats);
}

void io(Archive& a, DeathDecl& d) {
    a.u64(d.deathId);
    ioEntity(a, d.who);
    ioEnum(a, d.source, 3);
    a.u8(d.declarer);
    a.varu32(d.epoch);
    ioEntity(a, d.killer);
    io(a, d.weapon);
    a.u8(d.limbMask);
    a.varu32(d.cell);
    ioVec(a, d.where);
    a.u32(d.tick);
}

void io(Archive& a, ProjectileSpawn& p) {
    a.u64(p.id);
    ioEntity(a, p.shooter);
    a.varu32(p.shooterEpoch);
    io(a, p.weapon);
    io(a, p.ammo);
    io(a, p.projectileBase);
    a.varu32(p.cell);
    ioVec(a, p.origin);
    ioVec(a, p.velocity);
    a.u32(p.firedAtTick);
    a.varu32(p.lifetimeMs);
    a.boolean(p.explosive);
}

void io(Archive& a, ExplosionEvent& e) {
    a.u64(e.id);
    ioEntity(a, e.source);
    a.varu32(e.sourceEpoch);
    a.u64(e.projectile);
    io(a, e.weapon);
    io(a, e.explosionBase);
    a.varu32(e.cell);
    ioVec(a, e.pos);
    a.f32(e.radius);
    a.vari32(e.damage);
    a.u32(e.tick);
    a.u32(e.firedAtTick);
}

void io(Archive& a, HostilityEffect& e) {
    a.u64(e.id);
    ioEnum(a, e.kind, 3);
    io(a, e.factionA);
    io(a, e.factionB);
    ioEntity(a, e.actor);
    a.u8(e.cause);
}

// ------------------------------------------------------------------------------------------------ strings
const char* toString(HitReason r) {
    switch (r) {
        case HitReason::Ok: return "ok";
        case HitReason::Duplicate: return "duplicate";
        case HitReason::UnknownVictim: return "unknown victim";
        case HitReason::NotVictimOwner: return "not the victim's owner";
        case HitReason::HandoffInProgress: return "handoff in progress";
        case HitReason::VictimDead: return "victim already dead";
        case HitReason::DamageOutOfBounds: return "damage out of bounds";
        case HitReason::WeaponRangeOutOfBounds: return "weapon range out of bounds";
        case HitReason::TooOld: return "hit too old";
        case HitReason::FromFuture: return "hit from the future";
        case HitReason::AttackerNotSender: return "attacker is not the sender";
        case HitReason::AttackerUnknown: return "attacker unknown";
        case HitReason::AttackerDead: return "attacker dead";
        case HitReason::ForgedAttacker: return "npc attacker not owned by the sender";
        case HitReason::StaleAttackerEpoch: return "stale attacker epoch";
        case HitReason::FutureAttackerEpoch: return "attacker epoch ahead of our table";
        case HitReason::OtherCell: return "attacker in another cell";
        case HitReason::OutOfRange: return "out of weapon range";
        case HitReason::ImpactNotNearVictim: return "impact point not near victim";
        case HitReason::NoLineOfFire: return "no line of fire";
        case HitReason::RateOfFire: return "rate of fire";
        case HitReason::SelfHit: return "self hit";
        case HitReason::UnknownProjectile: return "unknown projectile";
        case HitReason::ProjectileShooterMismatch: return "projectile shooter mismatch";
        case HitReason::ExplosionOutOfRadius: return "victim outside the blast";
        default: return "?";
    }
}
const char* toString(DeathReason r) {
    switch (r) {
        case DeathReason::Ok: return "ok";
        case DeathReason::AlreadyDead: return "already dead";
        case DeathReason::ReplayedDeclaration: return "replayed declaration";
        case DeathReason::UnknownActor: return "unknown actor";
        case DeathReason::NotOwner: return "declarer is not the owner";
        case DeathReason::StaleEpoch: return "stale epoch";
        case DeathReason::FutureEpoch: return "epoch ahead of our table";
        case DeathReason::NamedNeedsStory: return "named npc needs the story host";
        case DeathReason::StoryNeedsHost: return "story source from a non-host";
        case DeathReason::SelfOnly: return "player death declared by someone else";
        default: return "?";
    }
}
const char* toString(ProjectileReason r) {
    switch (r) {
        case ProjectileReason::Ok: return "ok";
        case ProjectileReason::Duplicate: return "duplicate";
        case ProjectileReason::UnknownShooter: return "unknown shooter";
        case ProjectileReason::NotShooterOwner: return "sender is not the shooter's owner";
        case ProjectileReason::StaleEpoch: return "stale epoch";
        case ProjectileReason::FutureEpoch: return "epoch ahead of our table";
        case ProjectileReason::SpeedOutOfBounds: return "speed out of bounds";
        case ProjectileReason::OriginTooFar: return "origin too far from the shooter";
        case ProjectileReason::RateOfFire: return "rate of fire";
        case ProjectileReason::Expired: return "expired";
        default: return "?";
    }
}
const char* toString(ExplosionReason r) {
    switch (r) {
        case ExplosionReason::Ok: return "ok";
        case ExplosionReason::Duplicate: return "duplicate";
        case ExplosionReason::UnknownSource: return "unknown source";
        case ExplosionReason::NotSourceOwner: return "sender is not the source's owner";
        case ExplosionReason::StaleEpoch: return "stale epoch";
        case ExplosionReason::FutureEpoch: return "epoch ahead of our table";
        case ExplosionReason::RadiusOutOfBounds: return "radius out of bounds";
        case ExplosionReason::DamageOutOfBounds: return "damage out of bounds";
        default: return "?";
    }
}

// ------------------------------------------------------------------------------------------------ hits
void HitValidator::reset() {
    seen_.clear();
    order_.clear();
    times_.clear();
}

bool HitValidator::remember(uint64_t id) { return rememberId(seen_, order_, cap_, id); }

bool HitValidator::rateOk(const Entity& attacker, const FormRef& weapon, uint64_t nowMs, const ICombatWorld& w) {
    auto& q = times_[{attacker, weaponKey(weapon)}];
    while (!q.empty() && q.front() + 1000 <= nowMs) q.pop_front();
    uint32_t allowed = rules_.maxHitsPerSec;
    if (const WeaponInfo* wi = weapon.plugin.empty() ? nullptr : w.weapon(weapon)) {
        if (wi->minIntervalMs > 0) {
            const uint32_t perSec = (1000u + wi->minIntervalMs - 1u) / wi->minIntervalMs;
            allowed = perSec * std::max<uint32_t>(1, wi->pellets) + rules_.rateBurstSlack;
        }
    }
    if (FNV_MUT(313)) return true;   // twin: rate of fire not enforced
    if (q.size() >= allowed) return false;
    q.push_back(nowMs);
    return true;
}

HitVerdict HitValidator::judge(const HitEvent& h, PlayerId sender, PlayerId self, uint64_t nowMs, const ICombatWorld& w) {
    ++judged_;
    HitVerdict v;
    auto reject = [&](HitReason r, const std::string& detail) {
        ++rejected_;
        v.apply = false;
        v.reason = r;
        v.detail = detail;
        return v;
    };

    // 1. the victim, and whether this machine is the one that resolves the hit
    ActorFacts victim;
    if (!w.facts(h.victim, victim) || !victim.known) return reject(HitReason::UnknownVictim, "victim " + toString(h.victim));
    if (!FNV_MUT(302) && victim.owner != self) {   // twin 302: any machine resolves any hit
        v.forwardTo = victim.owner;
        return reject(HitReason::NotVictimOwner, toString(h.victim) + " is owned by " + std::to_string(victim.owner));
    }
    if (!FNV_MUT(317) && victim.handingOff) return reject(HitReason::HandoffInProgress, toString(h.victim));

    // 2. idempotency: a hit resolves once, however often the network delivers it
    if (!FNV_MUT(301) && !remember(h.hitId)) return reject(HitReason::Duplicate, "hit " + std::to_string(h.hitId));

    // 3. victim state, bounds, age
    if (!FNV_MUT(303) && (!victim.alive || victim.health < 0)) return reject(HitReason::VictimDead, toString(h.victim));
    const WeaponInfo* wi = h.weapon.plugin.empty() ? nullptr : w.weapon(h.weapon);
    const int32_t dmgCap = wi ? std::min(rules_.maxDamage, wi->maxDamage) : rules_.maxDamage;
    if (!FNV_MUT(304) && (h.damage <= 0 || h.damage > dmgCap)) return reject(HitReason::DamageOutOfBounds, std::to_string(h.damage) + " > " + std::to_string(dmgCap));
    if (h.firedAtTick == 0) {
        if (rules_.requireTick) return reject(HitReason::TooOld, "unstamped hit");
    } else {
        const int32_t age = ageMs(nowMs, h.firedAtTick);
        if (!FNV_MUT(306) && age > static_cast<int32_t>(rules_.maxHitAgeMs)) return reject(HitReason::TooOld, std::to_string(age) + " ms old");
        if (!FNV_MUT(315) && age < -static_cast<int32_t>(rules_.futureSlackMs)) return reject(HitReason::FromFuture, std::to_string(-age) + " ms ahead");
    }

    // 4. who the attacker is, and whether the sender may speak for it
    const bool explosion = h.explosionId != 0;
    ActorFacts atk;
    const bool haveAtk = w.facts(h.attacker, atk) && atk.known;
    if (h.attacker.isPlayer()) {
        if (!FNV_MUT(307) && (h.attacker.id != sender || h.attacker.id >= kMaxSlots)) return reject(HitReason::AttackerNotSender, toString(h.attacker) + " from " + std::to_string(sender));
        if (!haveAtk) return reject(HitReason::AttackerUnknown, toString(h.attacker));
        if (!FNV_MUT(316) && !atk.alive && !explosion) return reject(HitReason::AttackerDead, toString(h.attacker));   // a grenade already in the air still goes off
        // a player's entity epoch rises with every join of that slot: a hit sent before a rejoin is the old session's (twin 366)
        if (!FNV_MUT(366) && h.attackerEpoch != 0 && atk.epoch != 0 && h.attackerEpoch < atk.epoch)
            return reject(HitReason::StaleAttackerEpoch, toString(h.attacker) + " " + std::to_string(h.attackerEpoch) + " < " + std::to_string(atk.epoch));
    } else if (h.attacker.isActor()) {
        if (!haveAtk) return reject(HitReason::AttackerUnknown, toString(h.attacker));
        if (!FNV_MUT(308) && atk.owner != sender) return reject(HitReason::ForgedAttacker, toString(h.attacker) + " is owned by " + std::to_string(atk.owner) + ", sender " + std::to_string(sender));
        if (!FNV_MUT(309) && h.attackerEpoch < atk.epoch) return reject(HitReason::StaleAttackerEpoch, std::to_string(h.attackerEpoch) + " < " + std::to_string(atk.epoch));
        if (!FNV_MUT(318) && h.attackerEpoch > atk.epoch) { v.resync = true; return reject(HitReason::FutureAttackerEpoch, std::to_string(h.attackerEpoch) + " > " + std::to_string(atk.epoch)); }
        if (!FNV_MUT(316) && !atk.alive && !explosion) return reject(HitReason::AttackerDead, toString(h.attacker));
    } else if (!(explosion && sender == kHostPlayer)) {   // an attacker-less hit exists only as host scripted explosion damage
        return reject(HitReason::AttackerUnknown, "no attacker");
    }

    if (!FNV_MUT(314) && !rules_.allowSelfHit && !explosion && h.attacker == h.victim) return reject(HitReason::SelfHit, toString(h.victim));

    // 5. projectile consistency
    if (h.projectile != 0) {
        Entity shooter;
        const bool known = w.projectile(h.projectile, shooter);
        if (known) {
            if (!FNV_MUT(320) && shooter != h.attacker) return reject(HitReason::ProjectileShooterMismatch, "projectile " + std::to_string(h.projectile) + " was fired by " + toString(shooter));
        } else if (rules_.requireKnownProjectile) {
            return reject(HitReason::UnknownProjectile, std::to_string(h.projectile));
        }
    }

    // 6. geometry
    if (explosion) {
        if (!(h.claimedRange > 0) || h.claimedRange > rules_.maxExplosionRadius) return reject(HitReason::WeaponRangeOutOfBounds, "blast radius " + std::to_string(h.claimedRange));
        if (!FNV_MUT(321) && distance(h.impactPoint, victim.pos) > h.claimedRange + victim.radius + rules_.impactSlack) return reject(HitReason::ExplosionOutOfRadius, toString(h.victim));
    } else {
        float range = h.claimedRange;
        if (wi) range = (FNV_MUT(319) && h.claimedRange > 0) ? h.claimedRange : wi->range;   // the table wins over what the attacker claims
        else if (!FNV_MUT(305) && (!(range > 0) || range > rules_.maxWeaponRange)) return reject(HitReason::WeaponRangeOutOfBounds, "range " + std::to_string(range));
        const float slack = rules_.rangeSlack + (atk.owner != self ? rules_.replicaSlack : 0.0f);
        if (!FNV_MUT(310) && !w.cellsAdjacent(atk.cell, victim.cell)) return reject(HitReason::OtherCell, toString(h.attacker) + " cell " + std::to_string(atk.cell) + ", victim cell " + std::to_string(victim.cell));
        if (!FNV_MUT(311) && distance(atk.pos, victim.pos) > range + slack + victim.radius) return reject(HitReason::OutOfRange, std::to_string(distance(atk.pos, victim.pos)) + " > " + std::to_string(range));
        if (!FNV_MUT(312) && !h.impactPoint.zero() && distance(h.impactPoint, victim.pos) > rules_.impactSlack + victim.radius) return reject(HitReason::ImpactNotNearVictim, toString(h.victim));
        if (!FNV_MUT(322) && !w.lineOfFire(atk.pos, h.impactPoint.zero() ? victim.pos : h.impactPoint)) return reject(HitReason::NoLineOfFire, toString(h.attacker));
    }

    // 7. rate of fire (last, so a hit rejected for another reason does not eat the budget). Explosion damage is limited per explosion, not per swing.
    if (!explosion && !rateOk(h.attacker, h.weapon, nowMs, w)) return reject(HitReason::RateOfFire, toString(h.attacker));

    v.apply = true;
    v.reason = HitReason::Ok;
    v.damage = h.damage;
    return v;
}

// ------------------------------------------------------------------------------------------------ limbs and damage
uint8_t applyLimbDamage(LimbState& s, Limb limb, int32_t damage, const CombatRules& rules) {
    const unsigned i = static_cast<unsigned>(limb);
    if (i >= kLimbCount || damage <= 0) return 0;
    if (FNV_MUT(324)) s.damage[i] = 0;   // twin: nothing accumulates, only a single big hit cripples
    s.damage[i] += damage;
    int32_t need = rules.crippleDamage[i];
    if (FNV_MUT(323) && i == 0) need = 30;   // twin: the torso can be crippled
    if (need <= 0 || s.damage[i] < need) return 0;
    const uint8_t b = LimbState::bit(limb);
    const uint8_t fresh = static_cast<uint8_t>(b & ~s.crippled);
    s.crippled = static_cast<uint8_t>(s.crippled | b);
    return fresh;
}

uint8_t mergeLimbMask(uint8_t have, uint8_t incoming) {
    if (FNV_MUT(325)) return incoming;   // twin: the newest mask replaces the old one, so a stale message un-cripples a limb
    return static_cast<uint8_t>(have | incoming);
}

DamageOutcome applyHit(VictimVitals& v, const HitEvent& h, const CombatRules& rules) {
    DamageOutcome o;
    o.healthBefore = o.healthAfter = v.health;
    if (v.dead && !FNV_MUT(327)) return o;
    o.applied = true;
    o.newlyCrippled = applyLimbDamage(v.limbs, h.limb, h.damage, rules);
    v.health -= h.damage;
    const bool lethal = FNV_MUT(328) ? v.health < 0 : v.health <= 0;
    if (lethal) {
        if ((v.named || v.essential) && !FNV_MUT(326)) {
            v.health = 1;
            o.protectedFromDeath = true;
        } else {
            v.health = 0;
            v.dead = true;
            o.died = true;
        }
    }
    o.healthAfter = v.health;
    return o;
}

// ------------------------------------------------------------------------------------------------ death
bool DeathLedger::remember(uint64_t id) { return rememberId(ids_, order_, cap_, id); }

DeathVerdict DeathLedger::judge(const DeathDecl& d, PlayerId sender, bool senderIsHost, const ICombatWorld& w) {
    DeathVerdict v;
    if (!FNV_MUT(333) && ids_.count(d.deathId)) { v.reason = DeathReason::ReplayedDeclaration; return v; }
    if (!FNV_MUT(331) && dead_.count(d.who)) {   // idempotent by actor id: it already died
        remember(d.deathId);
        v.reason = DeathReason::AlreadyDead;
        return v;
    }
    ActorFacts f;
    const bool known = w.facts(d.who, f) && f.known;
    if (known && !f.alive && !FNV_MUT(363)) {   // dead in the world though this ledger never heard of it (a world loaded with corpses, a late joiner)
        remember(d.deathId);
        v.reason = DeathReason::AlreadyDead;
        return v;
    }
    switch (d.source) {
        case DeathSource::StoryHost:
            if (!FNV_MUT(334) && !senderIsHost) { v.reason = DeathReason::StoryNeedsHost; return v; }
            break;   // the story may kill anything it names, loaded here or not
        case DeathSource::SelfPlayer:
            if (!FNV_MUT(335) && (!d.who.isPlayer() || d.declarer != d.who.id || sender != d.who.id)) { v.reason = DeathReason::SelfOnly; return v; }
            break;
        case DeathSource::Owner:
            if (!known) { v.reason = DeathReason::UnknownActor; return v; }
            if (!FNV_MUT(330) && f.named) { v.reason = DeathReason::NamedNeedsStory; return v; }
            if (!FNV_MUT(329) && f.owner != sender) { v.reason = DeathReason::NotOwner; return v; }
            if (d.who.isActor()) {
                if (!FNV_MUT(332) && d.epoch < f.epoch) { v.reason = DeathReason::StaleEpoch; return v; }
                if (d.epoch > f.epoch) { v.reason = DeathReason::FutureEpoch; v.resync = true; return v; }
            }
            break;
    }
    remember(d.deathId);
    dead_[d.who] = d;
    v.accept = true;
    v.reason = DeathReason::Ok;
    return v;
}

// ------------------------------------------------------------------------------------------------ projectiles
ProjectileVerdict ProjectileRegistry::onSpawn(const ProjectileSpawn& p, PlayerId sender, PlayerId self, uint64_t nowMs, const ICombatWorld& w) {
    ProjectileVerdict v;
    auto reject = [&](ProjectileReason r) { v.accept = false; v.role = ProjectileRole::Rejected; v.reason = r; return v; };

    ActorFacts s;
    if (!w.facts(p.shooter, s) || !s.known) return reject(ProjectileReason::UnknownShooter);
    if (!FNV_MUT(336) && s.owner != sender) return reject(ProjectileReason::NotShooterOwner);
    if (!FNV_MUT(338) && !rememberId(seen_, order_, cap_, p.id)) return reject(ProjectileReason::Duplicate);
    if (p.shooter.isActor()) {
        if (!FNV_MUT(341) && p.shooterEpoch < s.epoch) return reject(ProjectileReason::StaleEpoch);
        if (p.shooterEpoch > s.epoch) { v.resync = true; return reject(ProjectileReason::FutureEpoch); }
    }
    const float speed = std::sqrt(p.velocity.x * p.velocity.x + p.velocity.y * p.velocity.y + p.velocity.z * p.velocity.z);
    if (!FNV_MUT(339) && speed > rules_.maxProjectileSpeed) return reject(ProjectileReason::SpeedOutOfBounds);
    if (!FNV_MUT(340) && distance(p.origin, s.pos) > rules_.projectileOriginSlack + s.radius) return reject(ProjectileReason::OriginTooFar);
    if (p.firedAtTick != 0 && ageMs(nowMs, p.firedAtTick) > static_cast<int32_t>(p.lifetimeMs + rules_.maxHitAgeMs)) return reject(ProjectileReason::Expired);
    auto& q = times_[p.shooter];
    while (!q.empty() && q.front() + 1000 <= nowMs) q.pop_front();
    if (!FNV_MUT(342)) {
        if (q.size() >= rules_.maxProjectilesPerSec) return reject(ProjectileReason::RateOfFire);
    }
    q.push_back(nowMs);

    v.accept = true;
    v.reason = ProjectileReason::Ok;
    // The shooter's owner runs the real projectile; every other machine shows a copy that cannot hurt anybody.
    v.role = (sender == self && s.owner == self) ? ProjectileRole::Authoritative : ProjectileRole::VisualOnly;
    if (FNV_MUT(337)) v.role = ProjectileRole::Authoritative;   // twin: visual copies are live
    live_[p.id] = Live{p, v.role, nowMs + p.lifetimeMs};
    return v;
}

void ProjectileRegistry::expire(uint64_t nowMs) {
    for (auto it = live_.begin(); it != live_.end();) it = it->second.diesAtMs <= nowMs ? live_.erase(it) : std::next(it);
}

// ------------------------------------------------------------------------------------------------ explosions
uint64_t explosionHitId(uint64_t explosionId, const Entity& victim) {
    uint64_t key[3] = {explosionId, static_cast<uint64_t>(victim.kind), FNV_MUT(347) ? 0 : static_cast<uint64_t>(victim.id)};   // twin 347: victims share one id
    return (hash64(key, sizeof key) & ((1ull << 55) - 1)) | (1ull << 55);   // bit 55 marks a derived id; normal ids (slot << 48 | counter) never reach it
}

std::vector<HitEvent> planExplosionHits(const ExplosionEvent& e, const std::vector<BlastCandidate>& candidates, const CombatRules& rules) {
    std::vector<HitEvent> out;
    if (e.radius <= 0 || e.damage <= 0) return out;
    std::vector<const BlastCandidate*> order;
    for (auto& c : candidates) order.push_back(&c);
    std::sort(order.begin(), order.end(), [](const BlastCandidate* a, const BlastCandidate* b) { return a->victim < b->victim; });
    for (const BlastCandidate* c : order) {
        if (!FNV_MUT(348) && c->cell != e.cell) continue;
        const float d = distance(c->pos, e.pos);
        if (!FNV_MUT(345) && d > e.radius + c->radius) continue;
        float f = 1.0f - rules.explosionFalloff * std::min(1.0f, d / e.radius);
        if (FNV_MUT(346)) f = 1.0f;   // twin: no falloff
        HitEvent h;
        h.hitId = explosionHitId(e.id, c->victim);
        h.attacker = e.source;
        h.victim = c->victim;
        h.damage = std::max<int32_t>(1, static_cast<int32_t>(std::lround(static_cast<float>(e.damage) * f)));
        h.limb = Limb::Torso;
        h.impactPoint = e.pos;
        h.weapon = e.weapon;
        h.projectile = e.projectile;
        h.explosionId = e.id;
        h.firedAtTick = e.tick;
        h.attackerEpoch = e.sourceEpoch;
        h.claimedRange = e.radius;
        out.push_back(h);
    }
    return out;
}

ExplosionVerdict ExplosionLedger::judge(const ExplosionEvent& e, PlayerId sender, PlayerId self, const ICombatWorld& w) {
    ExplosionVerdict v;
    auto reject = [&](ExplosionReason r) { v.accept = false; v.reason = r; v.authoritative = false; return v; };
    PlayerId owner = kHostPlayer;   // a scripted blast (no source) is the host's
    OwnerEpoch epoch = 0;
    if (!e.source.none()) {
        ActorFacts s;
        if (!w.facts(e.source, s) || !s.known) return reject(ExplosionReason::UnknownSource);
        owner = s.owner;
        epoch = s.epoch;
    }
    if (!FNV_MUT(343) && owner != sender) return reject(ExplosionReason::NotSourceOwner);
    if (e.source.isActor()) {
        if (e.sourceEpoch < epoch) return reject(ExplosionReason::StaleEpoch);
        if (e.sourceEpoch > epoch) { v.resync = true; return reject(ExplosionReason::FutureEpoch); }
    }
    if (!FNV_MUT(350) && (!(e.radius > 0) || e.radius > rules_.maxExplosionRadius)) return reject(ExplosionReason::RadiusOutOfBounds);
    if (e.damage <= 0 || e.damage > rules_.maxDamage) return reject(ExplosionReason::DamageOutOfBounds);
    if (!FNV_MUT(349) && !rememberId(ids_, order_, cap_, e.id)) return reject(ExplosionReason::Duplicate);
    v.accept = true;
    v.reason = ExplosionReason::Ok;
    v.authoritative = FNV_MUT(344) ? true : (sender == self && owner == self);   // twin 344: every machine plans (and applies) the damage
    return v;
}

// ------------------------------------------------------------------------------------------------ kill XP
std::vector<XpAward> attributeKillXp(const KillRecord& k, XpMode mode, const std::vector<PlayerId>& present, const CombatRules& rules) {
    std::vector<XpAward> out;
    if (k.xp <= 0) return out;
    PlayerId credit = kNoPlayer;
    if (k.killer.isPlayer() && k.killer.id < kMaxSlots) credit = static_cast<PlayerId>(k.killer.id);
    else if (!FNV_MUT(352) && k.killerRecruiter != kNoPlayer && k.killerRecruiter < kMaxSlots) credit = k.killerRecruiter;   // a companion's kill belongs to its player
    if (credit == kNoPlayer) return out;   // killed by an NPC or the environment: nobody earns it

    std::vector<PlayerId> party;
    for (PlayerId p : present) if (p < kMaxSlots && std::find(party.begin(), party.end(), p) == party.end()) party.push_back(p);
    std::sort(party.begin(), party.end());

    if (mode == XpMode::KillingBlow) {
        if (FNV_MUT(351)) {   // twin: everyone present is paid in full
            for (PlayerId p : party) out.push_back({p, k.xp});
            return out;
        }
        out.push_back({credit, k.xp});
        return out;
    }
    if (party.empty()) { out.push_back({credit, k.xp}); return out; }
    const int32_t n = static_cast<int32_t>(party.size());
    int32_t base = k.xp / n;
    int32_t rem = k.xp % n;
    if (FNV_MUT(353)) { base = k.xp; rem = 0; }   // twin: nothing is split
    const PlayerId remTo = std::find(party.begin(), party.end(), credit) != party.end() ? credit : party.front();
    for (PlayerId p : party) {
        int32_t xp = std::max(base, rules.partyXpMinShare);
        if (p == remTo && !FNV_MUT(354)) xp += rem;   // twin 354: the remainder is dropped, so xp is lost
        out.push_back({p, xp});
    }
    return out;
}

std::vector<XpAward> XpLedger::award(const KillRecord& k, XpMode mode, const std::vector<PlayerId>& present, const CombatRules& rules) {
    const uint64_t key = k.deathId ? k.deathId : ((static_cast<uint64_t>(k.victim.kind) << 40) ^ k.victim.id ^ (1ull << 62));
    if (!FNV_MUT(355) && !rememberId(ids_, order_, cap_, key)) return {};
    return attributeKillXp(k, mode, present, rules);
}

// ------------------------------------------------------------------------------------------------ hostility
std::vector<HostilityEffect> HostilityBook::onCrime(const CrimeReport& r, PlayerId self, PlayerId sender) {
    std::vector<HostilityEffect> out;
    if (!FNV_MUT(357) && self != kHostPlayer) return out;   // clients report, the host decides
    if (r.criminal >= kMaxSlots || sender >= kMaxSlots) return out;
    if (!FNV_MUT(360) && !r.witnessed) return out;
    if (!FNV_MUT(358) && r.victimWasHostile) return out;
    if (r.victimFaction.plugin.empty() || r.criminalFaction.plugin.empty() || r.victimFaction == r.criminalFaction) return out;
    auto k = key(r.criminalFaction, r.victimFaction);
    auto it = pairs_.find(k);
    if (!FNV_MUT(359) && it != pairs_.end() && it->second) return out;   // already enemies: nothing new
    pairs_[k] = true;
    HostilityEffect e;
    e.kind = HostilityKind::SetEnemy;
    e.factionA = r.criminalFaction;
    e.factionB = r.victimFaction;
    e.cause = r.criminal;
    uint64_t gen = ++gen_[k];
    if (FNV_MUT(359)) gen = 1;
    uint64_t parts[4] = {static_cast<uint64_t>(e.kind), weaponKey(k.first), weaponKey(k.second), gen};
    e.id = hash64(parts, sizeof parts);
    out.push_back(e);
    return out;
}

std::vector<HostilityEffect> HostilityBook::makePeace(const FormRef& a, const FormRef& b, PlayerId cause, PlayerId self) {
    std::vector<HostilityEffect> out;
    if (self != kHostPlayer) return out;
    auto k = key(a, b);
    auto it = pairs_.find(k);
    if (it == pairs_.end() || !it->second) return out;
    if (!FNV_MUT(361)) it->second = false;
    HostilityEffect e;
    e.kind = HostilityKind::SetAlly;
    e.factionA = a;
    e.factionB = b;
    e.cause = cause;
    uint64_t parts[4] = {static_cast<uint64_t>(e.kind), weaponKey(k.first), weaponKey(k.second), ++gen_[k]};
    e.id = hash64(parts, sizeof parts);
    out.push_back(e);
    return out;
}

bool HostilityBook::hostile(const FormRef& a, const FormRef& b) const {
    auto it = pairs_.find(key(a, b));
    return it != pairs_.end() && it->second;
}

bool HostilityBook::accept(const HostilityEffect& e, PlayerId sender) {
    if (!FNV_MUT(356) && sender != kHostPlayer) return false;
    if (!FNV_MUT(362) && !applied_.insert(e.id).second) return false;
    if (e.kind == HostilityKind::SetEnemy) pairs_[key(e.factionA, e.factionB)] = true;
    else if (e.kind == HostilityKind::SetAlly) pairs_[key(e.factionA, e.factionB)] = false;
    return true;
}

// ------------------------------------------------------------------------------------------------ wire bundle
std::vector<uint8_t> encodeMsg(const CombatMsg& m) {
    Archive a = Archive::writer();
    CombatMsg copy = m;
    uint8_t kind = static_cast<uint8_t>(copy.index());
    a.u8(kind);
    std::visit([&](auto& v) { io(a, v); }, copy);
    if (!a.ok()) return {};
    std::vector<uint8_t> body = a.finish();
    std::vector<uint8_t> out;
    out.reserve(body.size() + 2);
    out.push_back(kCombatWireByte);
    out.push_back(kCombatWireVersion);
    out.insert(out.end(), body.begin(), body.end());
    return out;
}

namespace {
template <size_t I = 0>
bool readMsg(Archive& a, size_t idx, CombatMsg& out) {
    if constexpr (I == std::variant_size_v<CombatMsg>) {
        (void)a; (void)idx; (void)out;
        return false;
    } else {
        if (idx == I) {
            std::variant_alternative_t<I, CombatMsg> v{};
            io(a, v);
            if (!a.ok()) return false;
            out = std::move(v);
            return true;
        }
        return readMsg<I + 1>(a, idx, out);
    }
}
}  // namespace

bool decodeMsg(const uint8_t* data, size_t size, CombatMsg& out) {
    if (size < 3) return false;
    if (!FNV_MUT(364) && (data[0] != kCombatWireByte || data[1] != kCombatWireVersion)) return false;
    Archive a = Archive::reader(data + 2, size - 2);
    if (!a.ok()) return false;
    uint8_t kind = 0;
    a.u8(kind);
    if (!a.ok() || kind >= std::variant_size_v<CombatMsg>) return false;
    if (!readMsg(a, kind, out)) return false;
    return a.ok() && (FNV_MUT(365) || a.atEnd());
}

const char* msgName(const CombatMsg& m) {
    static const char* names[] = {"Hit", "Death", "Projectile", "Explosion", "Hostility"};
    return names[m.index()];
}

std::string consoleLineFor(const HostilityEffect& e, const std::string& a, const std::string& b) {
    // TODO-RE: the optional flag arguments of SetEnemy / SetAlly are not verified in game; the two-faction form is the documented minimum.
    switch (e.kind) {
        case HostilityKind::SetEnemy: return "SetEnemy " + a + " " + b;
        case HostilityKind::SetAlly: return "SetAlly " + a + " " + b;
        case HostilityKind::StopCombat: return "StopCombat";
    }
    return {};
}

}  // namespace fnvcoop::combat
