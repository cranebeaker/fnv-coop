#include "combat/combat_sync.h"
#include <algorithm>
#include <cmath>
#include <cstdio>
#include "../../tools/mut/fnvcoop_mut.h"

// Mutation twins of this file are 701..760 and 801..812 (runtime style: FNVCOOP_MUT=<n>, tools/mut/run_twins.py). Each FNV_MUT(n) is a deliberately broken
// variant that tests/test_combat.cpp must catch. synccore/combat owns 301..362; plugin/actors owns its compile-time twins.

namespace nvcoop::combat {

cc::CombatRules coopRules() {
    cc::CombatRules r = cc::defaultRules();
    r.maxWeaponRange = 40000;      // snipers and scoped hitscan are not in a weapon table yet
    r.maxDamage = 5000;            // Fat Man, crits and VATS multipliers
    r.maxHitsPerSec = 240;         // minigun pellets and shotgun spreads arrive one engine hit at a time
    r.rateBurstSlack = 8;
    r.maxExplosionRadius = 4000;
    return r;
}

cc::Limb limbOfLocation(int32_t loc) {
    // GECK humanoid body-part order (ActorHitData+0x10); the engine's own category table at 0x1196DAC agrees: 1,2,13 head, 3,4 left arm,
    // 5,6 right arm, 7..12 legs, 14 none. Creatures use other part lists: unknown indexes count as torso.
    switch (loc) {
        case 1: case 2: case 13: return cc::Limb::Head;
        case 3: case 4: return cc::Limb::LeftArm;
        case 5: case 6: return cc::Limb::RightArm;
        case 7: case 8: case 9: return cc::Limb::LeftLeg;
        case 10: case 11: case 12: return cc::Limb::RightLeg;
        default: return cc::Limb::Torso;
    }
}

// ------------------------------------------------------------------------------------------------ ICombatWorld over IWorldView
class CombatSync::Adapter : public cc::ICombatWorld {
public:
    Adapter(IWorldView& w, const cc::ProjectileRegistry& p) : w_(w), proj_(p) {}
    // A death declaration is judged while the world may ALREADY show the actor dead (the actor port kills the copy as soon as ActorSync says so,
    // and the owner's own engine killed it before it declared). The ledger's own record decides "already dead", so the world's flag is masked.
    bool maskAlive = false;
    bool facts(const cc::Entity& e, cc::ActorFacts& out) const override {
        EntityView v;
        if (!w_.entity(e, v) || !v.known) { out = cc::ActorFacts{}; return false; }
        out = cc::ActorFacts{};
        out.known = true; out.who = e; out.alive = maskAlive ? true : v.alive; out.named = v.named; out.essential = v.essential;
        out.cell = v.cell; out.pos = v.pos; out.radius = v.radius; out.owner = v.owner; out.epoch = v.epoch;
        out.handingOff = v.pending; out.health = v.health; out.maxHealth = v.maxHealth; out.limbMask = v.limbMask; out.recruiter = v.recruiter;
        return true;
    }
    bool cellsAdjacent(cc::CellId a, cc::CellId b) const override { return w_.cellsAdjacent(a, b); }
    const cc::WeaponInfo* weapon(const FormRef& id) const override { return w_.weapon(id); }
    bool projectile(uint64_t id, cc::Entity& shooter) const override { return proj_.shooterOf(id, shooter); }
    bool lineOfFire(const cc::Vec3& a, const cc::Vec3& b) const override { return w_.lineOfFire(a, b); }
private:
    IWorldView& w_;
    const cc::ProjectileRegistry& proj_;
};

void CombatSync::setFriendlyFire(bool on) {
    if (FNV_MUT(813)) return;   // MUTANT 813: the campaign's friendly fire choice never reaches a running session
    cfg_.friendlyFire = on;
}

CombatSync::CombatSync(const Config& cfg, IWorldView& world, ICombatEngine& engine, ICombatNet& net)
    : cfg_(cfg), world_(world), eng_(engine), net_(net), validator_(cfg.rules), projectiles_(cfg.rules), explosions_(cfg.rules) {
    adapter_.reset(new Adapter(world_, projectiles_));
}

CombatSync::~CombatSync() = default;

// ------------------------------------------------------------------------------------------------ plumbing
void CombatSync::remember(const char* what, const std::string& detail) {
    rejections_.push_back(std::string(what) + ": " + detail);
    while (rejections_.size() > 32) rejections_.pop_front();
}

std::string CombatSync::lastRejections() const {
    std::string s;
    for (auto& r : rejections_) s += r + "\n";
    return s;
}

uint8_t CombatSync::crippledMask(const cc::Entity& e) const {
    auto it = limbs_.find(e);
    return it == limbs_.end() ? 0 : it->second.mask();
}

cc::LimbState& CombatSync::limbsOf(const cc::Entity& e, const EntityView& v) {
    auto it = limbs_.find(e);
    if (it == limbs_.end()) {
        cc::LimbState s;
        s.crippled = v.limbMask;
        it = limbs_.emplace(e, s).first;
    } else {
        it->second.crippled = cc::mergeLimbMask(it->second.crippled, v.limbMask);   // the engine may have crippled it natively since
    }
    return it->second;
}

std::vector<PlayerId> CombatSync::peers() const {
    if (!cfg_.isHost) return {kHostPlayer};
    return std::vector<PlayerId>(joined_.begin(), joined_.end());
}

void CombatSync::sendTo(PlayerId peer, const Envelope& e) {
    if (peer == cfg_.self || peer == kNoPlayer) return;
    if (cfg_.isHost && !joined_.count(peer)) { ++stats_.unroutable; return; }
    net_.send(peer, encodeEnvelope(e));
}

void CombatSync::fanOut(const Envelope& e, PlayerId except) {
    for (PlayerId p : peers()) {
        if (p == except) continue;
        sendTo(p, e);
    }
}

void CombatSync::send(Envelope e) {
    if (e.src == kNoPlayer) e.src = cfg_.self;
    if (!cfg_.isHost) { sendTo(kHostPlayer, e); return; }
    if (e.dest == kNoPlayer) fanOut(e, e.src);
    else sendTo(e.dest, e);
}

void CombatSync::playerJoined(PlayerId slot) { if (cfg_.isHost && slot != cfg_.self && slot < kMaxSlots) joined_.insert(slot); }

void CombatSync::playerLeft(PlayerId slot) {
    joined_.erase(slot);
    // hits that player was about to resolve for us are gone: they re-route when the actors are re-owned
    for (auto it = inflight_.begin(); it != inflight_.end();) {
        if (it->second.hit.victim.isPlayer() && it->second.hit.victim.id == slot) it = inflight_.erase(it);
        else ++it;
    }
}

void CombatSync::hostLost() {
    inflight_.clear();
    blasts_.clear();
    vitalsDirty_.clear();
    deferredDeaths_.clear();
}

// ------------------------------------------------------------------------------------------------ the hit gate (attacker's / any machine)
bool CombatSync::buildHit(const EngineHit& e, const cc::Entity& atk, const EntityView& av, const cc::Entity& vic, const EntityView& vv, cc::HitEvent& h, bool& touch, uint32_t& centi) {
    touch = false;
    centi = 0;
    // The engine's blows are fractions (a first-level fist is a few tenths of a point) and the owner must apply exactly what the attacker's engine computed: the exact
    // amount travels in hundredths. The rules' health is whole points and they refuse a hit of 0, so their own damage is the nearest point, at least 1 for a blow that
    // hurt at all; it only gets the hit judged. Rounding a light blow to nothing made it vanish at the gate (a guest's punch on a host-owned settler was neutralised here
    // and nobody ever heard of it).
    if (e.healthDmg > 0) centi = static_cast<uint32_t>(std::min<long long>(std::llround(static_cast<double>(e.healthDmg) * 100.0), kMaxCenti));
    if (FNV_MUT(809) && centi > 0) centi = static_cast<uint32_t>(std::lround(e.healthDmg)) * 100u;   // 809: the exact amount is rounded to whole points before it is sent
    int32_t dmg = static_cast<int32_t>(std::lround(e.healthDmg));
    if (centi > 0 && centi < 100) ++stats_.lightHitsSent;
    if (dmg <= 0 && centi > 0 && !FNV_MUT(801)) dmg = 1;   // 801: a light blow stays at 0 for the rules and is dropped as a touch with no amount
    if (dmg <= 0) {
        // No health damage at all (armour, a glancing hit; a hundredth of a point counts as none): on an actor it is still a blow. The victim reels and turns on its
        // attacker, so the owner must hear of it. Reported as damage 1 (the rules accept nothing less) and flagged, so the owner takes no health off. Explosions and
        // negative "damage" stay out of it.
        if (!vic.isActor() || e.explosionKey != 0 || e.healthDmg < 0 || FNV_MUT(802)) return false;   // 802: a blow with no health damage is dropped
        dmg = 1;
        touch = true;
        centi = 0;
    }
    h = cc::HitEvent{};
    h.hitId = nextId();
    h.attacker = atk;
    h.victim = vic;
    h.damage = dmg;
    h.limb = limbOfLocation(e.hitLocation);
    h.impactPoint = e.impact;
    // The impact vector is read from an engine struct whose semantics are only known from a header: a point farther from the victim than the validator
    // tolerates is not a world position (a local-space vector, an uninitialised field). Sending it would get a good hit rejected as "impact not near the
    // victim", so it is sent as unknown (the validator then skips that one check).
    if (!h.impactPoint.zero() && cc::distance(h.impactPoint, vv.pos) > validator_.rules().impactSlack + vv.radius && !FNV_MUT(779)) h.impactPoint = cc::Vec3{};
    h.weapon = e.weapon;
    h.ammo = e.ammo;
    h.projectile = e.projectileKey;
    h.firedAtTick = 0;   // unstamped: machines have no shared clock yet (rules.requireTick stays false)
    h.attackerEpoch = atk.isActor() ? av.epoch : 0;
    h.claimedRange = e.weaponRange;
    // A weapon the table does not know (there is no weapon table in the real world view yet) still needs a plausible reach: the validator rejects 0.
    if (!(h.claimedRange > 0) && !FNV_MUT(778)) h.claimedRange = e.melee ? 300.0f : validator_.rules().maxWeaponRange;
    h.critical = e.critical;
    h.vats = e.vats || (atk.isPlayer() && eng_.vatsActive());
    return true;
}

uint64_t CombatSync::explosionIdFor(const EngineHit& e, const cc::Entity& atk, const EntityView& av) {
    auto it = explosionIds_.find(e.explosionKey);
    if (it != explosionIds_.end()) return it->second.id;
    const bool radiusKnown = e.blastRadius > 0;
    cc::ExplosionEvent ev;
    ev.id = nextId();
    ev.source = atk;
    ev.sourceEpoch = atk.isActor() ? av.epoch : 0;
    ev.projectile = e.projectileKey;
    ev.weapon = e.weapon;
    ev.explosionBase = e.explosionBase;
    ev.cell = av.cell;
    ev.pos = e.blastCenter;
    ev.radius = radiusKnown ? e.blastRadius : 1.0f;   // unknown radius: a visual-only declaration (the ledger needs a positive one), never planned
    ev.damage = std::max<int32_t>(1, static_cast<int32_t>(std::lround(e.healthDmg)));   // centre damage unknown: this victim's hit is the best proxy
    ev.tick = static_cast<uint32_t>(now_);
    explosionIds_[e.explosionKey] = ExpId{ev.id, now_};
    const cc::ExplosionVerdict v = explosions_.judge(ev, cfg_.self, cfg_.self, *adapter_);
    if (!v.accept || !v.authoritative) { ++stats_.explosionsRejected; return ev.id; }
    ++stats_.explosionsLocal;
    Envelope env; env.src = cfg_.self; env.dest = kNoPlayer; env.body = MExplosion{ev};
    send(env);
    Blast b; b.ev = ev; b.dueAt = now_ + cfg_.blastSettleMs; b.plan = radiusKnown;
    blasts_[ev.id] = b;
    return ev.id;
}

bool CombatSync::isVisualBlast(const EngineHit& e) const {
    if (!e.explosionKey) return false;
    for (const VisualBlast& b : visualBlasts_) {
        if (b.until < now_) continue;
        if (!b.base.plugin.empty() && !e.explosionBase.plugin.empty() && b.base != e.explosionBase) continue;
        if (cc::distance(b.pos, e.blastCenter) <= 120.0f) return true;
    }
    return false;
}

void CombatSync::recordHit(const cc::Entity& victim, const cc::Entity& attacker, const FormRef& weapon, cc::Limb limb, bool natural) {
    if (attacker.none() || FNV_MUT(731)) return;   // 731: no kill credit is ever remembered
    LastHit& l = lastHit_[victim];
    l.attacker = attacker; l.weapon = weapon; l.limb = limb; l.at = now_; l.natural = natural;
    if (attacker.isPlayer() && attacker.id < kMaxSlots) l.players.insert(static_cast<PlayerId>(attacker.id));
}

GateDecision CombatSync::onEngineHit(const EngineHit& e) {
    ++stats_.engineHits;
    auto decide = [&](GateAction a, const char* why, int32_t maxDmg = 0) {
        GateDecision d; d.action = a; d.why = why; d.maxHealthDamage = maxDmg;
        if (a == GateAction::Proceed) ++stats_.proceeded;
        else if (a == GateAction::Clamp) ++stats_.clamped;
        else ++stats_.neutralized;
        // what the gate decided and why, for a hit on a synced victim: a failed fight shows here whether the shot was forwarded or resolved natively
        if (e.victimRef && (e.attackerRef || a != GateAction::Proceed)) {
            char b[160];
            std::snprintf(b, sizeof b, "%s: %s (victim ref %08X, attacker ref %08X, damage %.1f)", a == GateAction::Proceed ? "proceed" : a == GateAction::Clamp ? "clamp" : "neutralize",
                          why, e.victimRef, e.attackerRef, static_cast<double>(e.healthDmg));
            lastGate_ = b;
        }
        return d;
    };
    cc::Entity atk, vic;
    EntityView av, vv;
    const bool haveAtk = e.attackerRef && world_.entityOfRef(e.attackerRef, atk) && world_.entity(atk, av) && av.known;
    const bool haveVic = e.victimRef && world_.entityOfRef(e.victimRef, vic) && world_.entity(vic, vv) && vv.known;

    // 1. a copy of somebody else's combatant "hits" something here (its animation, a visual-only projectile or explosion): never real damage
    if (haveAtk && !av.simulatedHere && !FNV_MUT(701)) {
        ++stats_.ghostHitsNeutralized;
        return decide(GateAction::Neutralize, "the attacker is a copy here; its owner reports the hit");
    }
    // 1b. the blast / the bullet is a visual-only copy of another machine's: it only plays its effects here
    if (!FNV_MUT(722) && isVisualBlast(e)) { ++stats_.ghostHitsNeutralized; return decide(GateAction::Neutralize, "a visual-only explosion"); }
    if (!FNV_MUT(723) && !haveAtk && e.projectileKey && visualProjectileUntil_ >= now_) {
        ++stats_.ghostHitsNeutralized;
        return decide(GateAction::Neutralize, "a visual-only projectile");
    }
    // 1c. an explosion made by a combatant simulated here is declared once (visual elsewhere); its victims are hit through their owners
    uint64_t xid = 0;
    if (e.explosionKey != 0 && haveAtk && !FNV_MUT(703)) {
        xid = explosionIdFor(e, atk, av);
        auto b = blasts_.find(xid);
        if (b != blasts_.end() && haveVic) b->second.handled.insert(vic);
    }
    if (!haveVic) return decide(GateAction::Proceed, "victim is not a synced combatant");

    // 1d. plugin/companions: a hit inside the party (player <-> player, player <-> companion, companion <-> companion) does nothing unless friendly fire is on
    if (haveAtk && socialNeutralizes(e.attackerRef, e.victimRef)) return decide(GateAction::Neutralize, "a hit inside the party");
    // 1e. a player's accidental hit on a protected story actor (Joshua, Moore ...) would fail the line for everybody: dropped, the attacker is warned (rules FriendlyFireGuard)
    if (haveAtk && atk.isPlayer() && vic.isActor() && atk.id < kMaxSlots && protectedBlocks(e.victimRef, static_cast<PlayerId>(atk.id), e.explosionKey != 0))
        return decide(GateAction::Neutralize, "an accidental hit on a protected story actor");

    // 2. the victim is simulated here (an NPC this machine owns, or the local player): the engine applies the hit natively
    if (vv.simulatedHere) {
        ++stats_.naturalHits;
        if (haveAtk) recordHit(vic, atk, e.weapon, limbOfLocation(e.hitLocation), true);
        if (haveAtk && e.healthDmg > 0 && cfg_.onHitLanded && !FNV_MUT(820)) cfg_.onHitLanded(e.victimRef, e.attackerRef, e.healthDmg);   // 820: threat never hears of a natural hit
        // the local engine handles an attack by the local player on its own (reputation, the victim's reaction): the other machines still need to learn of it
        if (haveAtk && vic.isActor() && e.healthDmg > 0) socialCrime(atk, av, vic, vv, false, atk == cc::Entity::player(cfg_.self) ? static_cast<uint8_t>(1u << cfg_.self) : uint8_t(0));
        if (vic.isActor() && e.healthDmg > 0) {
            VitalsDirty& d = vitalsDirty_[vic];
            if (d.dueAt == 0) d.dueAt = now_ + cfg_.vitalsMinMs;
        }
        return decide(GateAction::Proceed, "victim is resolved here");
    }
    if (vic.isActor() && vv.owner == kNoPlayer && !FNV_MUT(702)) return decide(GateAction::Proceed, "victim has no owner yet");

    // 3. the victim is a copy here. Only a combatant simulated HERE may report a hit on it
    if (!haveAtk) {
        ++stats_.envHitsNeutralized;
        return decide(GateAction::Neutralize, "no simulated attacker: the victim's owner resolves it");
    }
    if (vic.isPlayer() && atk.isPlayer() && !cfg_.friendlyFire) {
        ++stats_.friendlyNeutralized;
        return decide(GateAction::Neutralize, "player-vs-player damage is off");
    }
    cc::HitEvent h;
    bool touch = false;
    uint32_t centi = 0;
    if (vic.isActor()) ++stats_.hitsOnCopies;
    if (!buildHit(e, atk, av, vic, vv, h, touch, centi)) return decide(GateAction::Neutralize, "no health damage to report");
    if (xid != 0 && e.blastRadius > 0) {   // a real radius: the hit takes the id every planning pass would give it, so it can never land twice
        h.explosionId = xid;
        h.hitId = cc::explosionHitId(xid, vic);
        h.impactPoint = e.blastCenter;
        h.claimedRange = e.blastRadius;
    }
    forwardHit(h, vv.owner, touch, centi);
    if (vic.isActor()) socialCrime(atk, av, vic, vv, false, 0);   // the victim's owner resolves the hit; the attacker's machine is the one that can vouch for who attacked
    // keep the copy alive: it may take what leaves it at least 1 health; the owner decides the real outcome
    int32_t room = vv.health > 1 ? vv.health - 1 : 0;
    if (FNV_MUT(704)) room = vv.health + 1000000;   // 704: the copy takes the full local damage (and can die from a prediction)
    return decide(GateAction::Clamp, touch ? "forwarded to the victim's owner (no health damage: a touch)" : "forwarded to the victim's owner", room);
}

void CombatSync::noteNaturalLethal(const cc::Entity& who) {
    EntityView v;
    if (!who.isActor() || !world_.entity(who, v) || !v.known || !v.ref || !v.simulatedHere || v.alive) return;
    // The engine applied a hit here and the victim is at health <= 0. A normal hit runs the death in the same call, a VATS kill (slow motion, kill
    // cam) can leave the life state alive for a long time or for good, and the actor port declares only a life state death.
    ++stats_.naturalLethal;
    if (FNV_MUT(792)) return;                                  // 792: a natural lethal hit is never followed up
    if ((v.named || v.essential) && !FNV_MUT(794)) return;     // a protected actor does not die; the engine knocks it down
    if (eng_.lifeDead(v.ref)) return;                          // the engine's own death ran: the actor port declares it
    for (const PendingKill& k : pendingKills_) if (k.who == who) return;
    pendingKills_.push_back({who, now_ + cfg_.lethalSettleMs, true});
}

void CombatSync::forwardHit(const cc::HitEvent& h, PlayerId owner, bool touch, uint32_t centi) {
    Inflight f; f.hit = h; f.sentAt = now_; f.touch = touch; f.centi = centi;
    inflight_[h.hitId] = f;
    ++stats_.hitsSent;
    if (touch) ++stats_.touchesSent;
    Envelope env; env.src = cfg_.self; env.dest = owner; env.body = MHit{h, touch, centi};
    send(env);
}

// ------------------------------------------------------------------------------------------------ inbound
void CombatSync::receive(PlayerId from, const uint8_t* data, size_t size) {
    Envelope e;
    if (!decodeEnvelope(data, size, e)) { ++stats_.undecodable; return; }
    if (cfg_.isHost) {
        if (e.src != from) { ++stats_.spoofed; e.src = from; }   // a client speaks only for itself
    } else if (from != kHostPlayer) {
        ++stats_.spoofed;
        return;
    }
    dispatch(from, e);
}

void CombatSync::dispatch(PlayerId from, const Envelope& e) {
    if (const auto* m = std::get_if<MHit>(&e.body)) onHit(from, e, *m);
    else if (const auto* m = std::get_if<MHitNack>(&e.body)) {
        if (cfg_.isHost && e.dest != cfg_.self && e.dest != kNoPlayer) { sendTo(e.dest, e); return; }   // relay to the attacker
        onNack(*m);
    }
    else if (const auto* m = std::get_if<MVitals>(&e.body)) onVitals(e, *m);
    else if (const auto* m = std::get_if<MDeath>(&e.body)) onDeathMsg(from, e, *m);
    else if (const auto* m = std::get_if<MProjectile>(&e.body)) onProjectile(from, e, *m);
    else if (const auto* m = std::get_if<MExplosion>(&e.body)) onExplosion(from, e, *m);
    else if (const auto* m = std::get_if<MCrime>(&e.body)) onCrime(from, *m);
    else if (const auto* m = std::get_if<MHostility>(&e.body)) onHostility(from, *m);
}

void CombatSync::onHit(PlayerId from, const Envelope& env, const MHit& m) {
    ++stats_.hitsReceived;
    const cc::HitEvent& h = m.h;
    if (cfg_.isHost) {
        // The host knows the current owner: a hit aimed at a stale owner is re-routed (the attacker's table may be behind).
        EntityView vv;
        PlayerId owner = kNoPlayer;
        if (h.victim.isPlayer()) owner = static_cast<PlayerId>(h.victim.id);
        else if (world_.entity(h.victim, vv) && vv.known) owner = vv.owner;
        if (FNV_MUT(705)) owner = env.dest;   // 705: trust the sender's idea of the owner
        if (owner != kNoPlayer && owner != cfg_.self && owner != env.src) {
            if (env.hops >= kMaxHops) { ++stats_.hopLimit; return; }
            Envelope f = env;
            ++f.hops;
            f.dest = owner;
            ++stats_.hitsForwarded;
            sendTo(owner, f);
            return;
        }
    }
    cc::HitVerdict v = validator_.judge(h, env.src, cfg_.self, now_, *adapter_);
    if (FNV_MUT(720)) { v.apply = true; v.reason = cc::HitReason::Ok; }   // 720: every received hit is applied (double damage, non-owner, stale epoch)
    if (v.apply) { applyAccepted(h, env.src, m.touch, m.centi); return; }
    ++stats_.rejectedByReason[static_cast<size_t>(v.reason)];
    switch (v.reason) {
        case cc::HitReason::Duplicate:
            ++stats_.hitDuplicates;
            return;
        case cc::HitReason::NotVictimOwner:
            // Never forwarded from here: a client may not re-send a message under somebody else's name, and the host already routed by ITS table.
            // The author hears a NACK, refreshes its own table, and sends the same hit again (the id is not consumed yet).
            break;
        default:
            break;
    }
    ++stats_.hitsRejected;
    remember(cc::toString(v.reason), v.detail);
    const bool retryable = v.reason == cc::HitReason::HandoffInProgress || v.reason == cc::HitReason::FutureAttackerEpoch ||
                           v.reason == cc::HitReason::UnknownVictim || v.reason == cc::HitReason::AttackerUnknown ||
                           v.reason == cc::HitReason::NotVictimOwner;
    if (v.resync) ++stats_.resyncHints;
    if (retryable && !FNV_MUT(706) && env.src != cfg_.self) {
        MHitNack n; n.hitId = h.hitId; n.reason = v.reason; n.resync = v.resync;
        Envelope r; r.src = cfg_.self; r.dest = env.src; r.body = n;
        ++stats_.nacksSent;
        send(r);
    }
}

void CombatSync::onNack(const MHitNack& m) {
    ++stats_.nacksReceived;
    auto it = inflight_.find(m.hitId);
    if (it == inflight_.end()) return;
    Inflight& f = it->second;
    if (f.attempts >= cfg_.retryMax) { ++stats_.hitsAbandoned; inflight_.erase(it); return; }
    f.nacked = true;
    f.retryAt = now_ + cfg_.retryDelayMs;
    // reasons that fire before the validator remembers the id keep it; the others consumed it and need a fresh one
    f.keepId = (m.reason == cc::HitReason::HandoffInProgress || m.reason == cc::HitReason::UnknownVictim || m.reason == cc::HitReason::NotVictimOwner);
}

// ------------------------------------------------------------------------------------------------ the owner applies an accepted hit
cc::VictimVitals CombatSync::vitalsOf(const cc::Entity& e, const EntityView& v) {
    cc::VictimVitals vit;
    vit.health = v.health; vit.maxHealth = v.maxHealth; vit.dead = !v.alive; vit.named = v.named; vit.essential = v.essential;
    vit.limbs = limbsOf(e, v);
    return vit;
}

// A blow of under one point of health, or none, reached the owner: the engine's reaction and the victim's notice of the attacker, with exactly `amount` points off
// its float health (nothing for a touch). The whole-point model of the rules is left alone; the coalesced vitals follow what the engine really shows.
void CombatSync::applyLight(const cc::HitEvent& h, const EntityView& vv, const EntityView& av, bool haveAtk, float amount) {
    if (!vv.alive || !vv.ref) return;
    if (amount > 0) ++stats_.lightHitsApplied; else ++stats_.touchesApplied;
    const uint32_t attackerRef = haveAtk ? av.ref : 0;
    if (!FNV_MUT(804) && eng_.touchVictim(vv.ref, attackerRef, h, amount)) ++stats_.touchesEngine;   // 804: the owner's engine never sees the blow
    if (h.victim.isActor() && haveAtk && av.ref && !FNV_MUT(805)) {                                   // 805: the victim does not turn on its attacker
        if (eng_.provoke(vv.ref, av.ref)) ++stats_.provoked;
    }
    if (amount > 0 && h.victim.isActor() && !FNV_MUT(812)) {   // 812: the light blow's result is never published (the copies keep the old health)
        VitalsDirty& d = vitalsDirty_[h.victim];
        if (d.dueAt == 0) d.dueAt = now_ + cfg_.vitalsMinMs;
    }
    if (amount > 0 && haveAtk && av.ref && cfg_.onHitLanded && !FNV_MUT(821)) cfg_.onHitLanded(vv.ref, av.ref, amount);   // 821: threat never hears of a light blow
    recordHit(h.victim, h.attacker, h.weapon, h.limb, false);
}

void CombatSync::applyAccepted(const cc::HitEvent& h, PlayerId sender, bool touch, uint32_t centi) {
    (void)sender;
    EntityView vv, av;
    if (!world_.entity(h.victim, vv) || !vv.known) return;
    if (vv.maxHealth <= 0) { ++stats_.damageRefused; remember("health unknown", cc::toString(h.victim)); return; }
    const bool haveAtk = world_.entity(h.attacker, av) && av.known;
    // a REMOTE player's hit that reached the victim's owner: the same story-actor protection as at the attacker's own gate (this machine holds the rules)
    if (h.attacker.isPlayer() && h.attacker.id != cfg_.self && h.attacker.id < kMaxSlots && h.victim.isActor() && vv.ref &&
        protectedBlocks(vv.ref, static_cast<PlayerId>(h.attacker.id), h.explosionId != 0)) return;
    // exact: the attacker's engine damage; a hit from a build that did not send it (centi 0, not a touch) falls back to the rules' whole points
    const float exact = centi > 0 ? static_cast<float>(centi) / 100.0f : static_cast<float>(h.damage);
    if (touch && !FNV_MUT(803)) { applyLight(h, vv, av, haveAtk, 0.0f); return; }       // 803: a touch is applied as a point of damage
    if (!touch && centi > 0 && centi < 100 && !FNV_MUT(810)) { applyLight(h, vv, av, haveAtk, exact); return; }   // 810: a light blow is applied as the point the rules carry
    cc::VictimVitals vit = vitalsOf(h.victim, vv);
    const cc::DamageOutcome o = cc::applyHit(vit, h, validator_.rules());
    limbs_[h.victim] = vit.limbs;
    if (!o.applied) return;
    ++stats_.hitsApplied;
    int32_t amount = o.healthBefore - o.healthAfter;
    // A lethal hit replays the WHOLE blow (and one point more), not the rounded remainder: the model's health is an integer, the engine's a float
    // (12.4 reads 12; a replay of 12 leaves a 0.4 sliver standing and the owner's gecko "lives" while every other machine was told it died).
    if (o.died && !FNV_MUT(790)) amount = std::max(amount, h.damage) + 1;
    // G04: the host owns the victim and a REMOTE player's shot is lethal: the kill is credited to that player's puppet (the engine stores the killer before OnDeath),
    // not left to a damage line that kills without a killer.
    bool attributed = false;
    if (o.died && h.victim.isActor() && vv.ref && !FNV_MUT(800)) {   // 800: a lethal hit of a remote player is plain damage (the victim dies killer-less)
        if (const uint32_t kr = remoteKillerRef(h.attacker)) {
            attributed = eng_.killActorBy(vv.ref, kr);
            if (attributed) { ++stats_.killsAttributed; stats_.damageApplied += static_cast<uint64_t>(std::max(amount, 0)); }
        }
    }
    if (!attributed && amount > 0 && vv.ref && !FNV_MUT(707)) {
        // the whole blow as the attacker's engine computed it, unless the rules changed the amount (a protected floor, a lethal replay): then their points
        const bool asComputed = centi > 0 && !o.died && amount == static_cast<int32_t>(std::lround(exact)) && !FNV_MUT(811);   // 811: fractions are rounded away on the owner
        const bool ok = asComputed ? eng_.damageHealthExact(h.victim, vv.ref, exact, haveAtk ? av.ref : 0, h) : eng_.damageHealth(h.victim, vv.ref, amount, haveAtk ? av.ref : 0, h);
        if (ok) stats_.damageApplied += static_cast<uint64_t>(amount);
        else { ++stats_.engineRefused; ++stats_.damageRefused; }
    }
    if (haveAtk && av.ref && vv.ref && cfg_.onHitLanded && !FNV_MUT(822)) cfg_.onHitLanded(vv.ref, av.ref, exact);   // 822: threat never hears of a forwarded hit
    for (unsigned i = 0; i < cc::kLimbCount; ++i) {
        if (!((o.newlyCrippled >> i) & 1u)) continue;
        if (vv.ref && eng_.setLimbCrippled(h.victim, vv.ref, static_cast<cc::Limb>(i))) ++stats_.limbsCrippled;
    }
    if (h.victim.isActor() && haveAtk && av.ref && vv.ref && !o.died && !FNV_MUT(708)) {
        if (eng_.provoke(vv.ref, av.ref)) ++stats_.provoked;
    }
    if (o.died && vv.ref && !FNV_MUT(791)) pendingKills_.push_back({h.victim, now_ + cfg_.killConfirmMs});   // tick() checks that it really died
    recordHit(h.victim, h.attacker, h.weapon, h.limb, false);
    vitalsDirty_.erase(h.victim);
    MVitals mv;
    mv.who = h.victim; mv.epoch = vv.epoch; mv.health = vit.health; mv.maxHealth = vit.maxHealth; mv.limbMask = vit.limbs.mask(); mv.dead = vit.dead; mv.cause = h.hitId;
    auto& sq = vitalsSeq_[h.victim];
    if (sq.first != mv.epoch) sq = {mv.epoch, 0};
    mv.seq = ++sq.second;
    ++stats_.vitalsSent;
    Envelope env; env.src = cfg_.self; env.dest = kNoPlayer; env.body = mv;
    send(env);
    maybeReportCrime(h, vv, o.died);
}

void CombatSync::publishVitals(const cc::Entity& e, uint64_t cause) {
    EntityView v;
    if (!world_.entity(e, v) || !v.known || !v.simulatedHere) return;
    limbsOf(e, v);
    MVitals mv;
    mv.who = e; mv.epoch = v.epoch; mv.health = v.health; mv.maxHealth = v.maxHealth; mv.limbMask = limbs_[e].mask(); mv.dead = !v.alive; mv.cause = cause;
    auto& sq = vitalsSeq_[e];
    if (sq.first != mv.epoch) sq = {mv.epoch, 0};
    mv.seq = ++sq.second;
    ++stats_.vitalsSent;
    Envelope env; env.src = cfg_.self; env.dest = kNoPlayer; env.body = mv;
    send(env);
}

void CombatSync::onVitals(const Envelope& e, const MVitals& m) {
    EntityView v;
    const bool known = world_.entity(m.who, v) && v.known;
    if (known) {
        // only the owner speaks for an actor; only a player speaks for themselves
        const PlayerId owner = m.who.isPlayer() ? static_cast<PlayerId>(m.who.id) : v.owner;
        if (!FNV_MUT(709) && owner != e.src) { ++stats_.spoofed; return; }
        if (!FNV_MUT(710) && m.who.isActor() && m.epoch < v.epoch) { ++stats_.vitalsStale; return; }
    }
    auto& seen = vitalsSeen_[m.who];
    const bool newer = m.epoch > seen.first || (m.epoch == seen.first && m.seq > seen.second);
    if (!newer && !FNV_MUT(711)) { ++stats_.vitalsStale; return; }
    if (newer) seen = {m.epoch, m.seq};
    if (cfg_.isHost) fanOut(e, e.src);
    if (!known || v.simulatedHere || !v.ref) return;   // an echo of what we resolved ourselves, or nothing loaded to show it on
    ++stats_.vitalsApplied;
    if (!m.dead) eng_.mirrorHealth(v.ref, std::max<int32_t>(1, m.health));   // never kills: a death arrives as a declaration
    const uint8_t fresh = static_cast<uint8_t>(m.limbMask & ~v.limbMask);
    for (unsigned i = 0; i < cc::kLimbCount; ++i)
        if ((fresh >> i) & 1u) eng_.setLimbCrippled(m.who, v.ref, static_cast<cc::Limb>(i));
}

// ------------------------------------------------------------------------------------------------ deaths
cc::DeathVerdict CombatSync::judgeDeath(const cc::DeathDecl& d, PlayerId sender, bool senderIsHost) {
    adapter_->maskAlive = !FNV_MUT(724);   // 724: the world's flag is believed (a declaration for an already killed copy is thrown away: no credit, no XP)
    const cc::DeathVerdict v = deaths_.judge(d, sender, senderIsHost, *adapter_);
    adapter_->maskAlive = false;
    return v;
}

void CombatSync::onActorDied(uint32_t ref, bool authorised, bool fromNetwork, bool mine, uint32_t killerRef) {
    cc::Entity who;
    if (!world_.entityOfRef(ref, who) || !who.isActor()) return;
    if (fromNetwork) {
        auto it = decor_.find(who);
        if (it != decor_.end()) { decorateIfDead(who, it->second.d); }
        return;
    }
    EntityView v;
    if (!world_.entity(who, v) || !v.known) return;
    if (v.named && !FNV_MUT(712)) {
        // A story NPC dies only through the host's KillActor effect: the HOST declares it (whoever owns the actor), nobody else does.
        if (!cfg_.isHost || !authorised) { ++stats_.deathsRefusedLocal; return; }
    } else if (!mine || !authorised) {
        ++stats_.deathsRefusedLocal;
        return;
    }
    cc::DeathDecl d;
    d.deathId = nextId();
    d.who = who;
    d.source = v.named ? cc::DeathSource::StoryHost : cc::DeathSource::Owner;
    d.declarer = cfg_.self;
    d.epoch = v.epoch;
    auto lh = lastHit_.find(who);
    if (lh != lastHit_.end() && now_ - lh->second.at <= cfg_.lastHitTtlMs) { d.killer = lh->second.attacker; d.weapon = lh->second.weapon; }
    else if (killerRef) { cc::Entity k; if (world_.entityOfRef(killerRef, k)) d.killer = k; }
    d.limbMask = static_cast<uint8_t>(v.limbMask | crippledMask(who));
    d.cell = v.cell; d.where = v.pos; d.tick = static_cast<uint32_t>(now_);
    const cc::DeathVerdict dv = judgeDeath(d, cfg_.self, cfg_.isHost);
    if (!dv.accept) { ++stats_.deathsRejected; remember(cc::toString(dv.reason), cc::toString(who)); return; }
    ++stats_.deathsDeclared;
    Envelope env; env.src = cfg_.self; env.dest = kNoPlayer; env.body = MDeath{d};
    send(env);
    handleDeathAccepted(d);
}

void CombatSync::onPlayerDied(uint32_t killerRef) {
    if (localPlayerDead_) return;
    localPlayerDead_ = true;
    cc::DeathDecl d;
    d.deathId = nextId();
    d.who = cc::Entity::player(cfg_.self);
    d.source = cc::DeathSource::SelfPlayer;
    d.declarer = cfg_.self;
    auto lh = lastHit_.find(d.who);
    if (lh != lastHit_.end() && now_ - lh->second.at <= cfg_.lastHitTtlMs) { d.killer = lh->second.attacker; d.weapon = lh->second.weapon; }
    else if (killerRef) { cc::Entity k; if (world_.entityOfRef(killerRef, k)) d.killer = k; }
    EntityView v;
    if (world_.entity(d.who, v)) { d.cell = v.cell; d.where = v.pos; d.limbMask = v.limbMask; }
    d.tick = static_cast<uint32_t>(now_);
    const cc::DeathVerdict dv = judgeDeath(d, cfg_.self, cfg_.isHost);
    if (!dv.accept) { ++stats_.deathsRejected; return; }
    ++stats_.deathsDeclared;
    Envelope env; env.src = cfg_.self; env.dest = kNoPlayer; env.body = MDeath{d};
    send(env);
    handleDeathAccepted(d);
}

void CombatSync::onPlayerAlive() {
    if (!localPlayerDead_) return;
    localPlayerDead_ = false;
    deaths_.resurrect(cc::Entity::player(cfg_.self));
    limbs_.erase(cc::Entity::player(cfg_.self));
    lastHit_.erase(cc::Entity::player(cfg_.self));
}

void CombatSync::onResurrect(const cc::Entity& who) {
    deaths_.resurrect(who);
    limbs_.erase(who);
    lastHit_.erase(who);
    vitalsSeen_.erase(who);
    decor_.erase(who);
}

void CombatSync::onActorLost(const cc::Entity& who) {
    lastHit_.erase(who);
    vitalsDirty_.erase(who);
    // limbs_ and the ledgers are kept: the actor may load again, and a corpse must not be killed twice
}

void CombatSync::onDeathMsg(PlayerId from, const Envelope& e, const MDeath& m) {
    (void)from;
    cc::DeathVerdict dv = judgeDeath(m.d, e.src, e.src == kHostPlayer);
    if (FNV_MUT(721)) { dv.accept = true; dv.reason = cc::DeathReason::Ok; }   // 721: every declaration counts (twice, named by message)
    if (!dv.accept) {
        if ((dv.reason == cc::DeathReason::FutureEpoch || dv.reason == cc::DeathReason::UnknownActor) && !FNV_MUT(714)) {
            DeferredDeath d; d.d = m.d; d.sender = e.src; d.until = now_ + cfg_.deferDeathMs;
            deferredDeaths_.push_back(d);
            ++stats_.deathsDeferred;
            if (dv.resync) ++stats_.resyncHints;
            return;
        }
        ++stats_.deathsRejected;
        remember(cc::toString(dv.reason), cc::toString(m.d.who));
        return;
    }
    if (cfg_.isHost) fanOut(e, e.src);
    handleDeathAccepted(m.d);
}

uint32_t CombatSync::remoteKillerRef(const cc::Entity& killer) const {
    if (!cfg_.isHost || !killer.isPlayer() || killer.id == cfg_.self) return 0;
    EntityView kv;
    return world_.entity(killer, kv) && kv.known ? kv.ref : 0;
}

uint32_t CombatSync::killerPuppetOf(uint32_t victimRef) const {
    if (FNV_MUT(798)) return 0;   // 798: nobody is ever credited (a guest's kill has no killer on the host)
    cc::Entity who;
    if (!victimRef || !world_.entityOfRef(victimRef, who) || !who.isActor()) return 0;
    auto lh = lastHit_.find(who);
    if (lh != lastHit_.end() && now_ - lh->second.at <= cfg_.lastHitTtlMs)
        if (const uint32_t r = remoteKillerRef(lh->second.attacker)) return r;
    auto kc = killCredit_.find(who);
    return kc != killCredit_.end() && now_ <= kc->second.until ? kc->second.killerRef : 0;
}

void CombatSync::handleDeathAccepted(const cc::DeathDecl& d) {
    ++stats_.deathsAccepted;
    decorateIfDead(d.who, d);
    if (d.who.isActor() && !FNV_MUT(799)) {   // 799: the death declaration never credits a remote player
        // G04: a remote player killed it. If the host's copy is already a corpse the killer is written now; else the actor port asks killerPuppetOf() when it kills the copy.
        if (const uint32_t kr = remoteKillerRef(d.killer)) {
            EntityView vv;
            if (world_.entity(d.who, vv) && vv.known && vv.ref && eng_.isDead(vv.ref)) { if (eng_.setKiller(vv.ref, kr)) ++stats_.killsAttributed; }
            else killCredit_[d.who] = KillCredit{kr, now_ + cfg_.deathDecorTtlMs};
        }
    }
    socialDeath(d);   // before awardXp / the erase below: both read the last hits
    // plugin/companions pays kill XP and karma once, to the right players, from the report above; combat's own award would pay the killer a second time.
    if (cfg_.social.xpHandled && cfg_.social.xpHandled() && !FNV_MUT(783)) ++stats_.socialXpHandled;   // 783: combat pays as well (the killer is paid twice)
    else awardXp(d);
    lastHit_.erase(d.who);
    vitalsDirty_.erase(d.who);
    if (cfg_.crimeSync && d.killer.isPlayer()) {
        EntityView v;
        if (d.who.isActor() && world_.entity(d.who, v) && !v.faction.plugin.empty() && d.killer.id == cfg_.self) {
            cc::CrimeReport r;
            r.criminal = cfg_.self; r.victim = d.who; r.victimFaction = v.faction;
            r.criminalFaction = FormRef("falloutnv.esm", 0x1BF3E);   // PlayerFaction
            r.killed = true; r.witnessed = true; r.reporter = cfg_.self;
            ++stats_.crimesReported;
            if (cfg_.isHost) onCrime(cfg_.self, MCrime{r});
            else { Envelope env; env.src = cfg_.self; env.dest = kHostPlayer; env.body = MCrime{r}; send(env); }
        }
    }
}

void CombatSync::decorateIfDead(const cc::Entity& who, const cc::DeathDecl& d) {
    if (!who.isActor()) return;
    EntityView v;
    if (!world_.entity(who, v) || !v.known || !v.ref) { decor_[who] = PendingDecor{d, now_ + cfg_.deathDecorTtlMs}; return; }
    if (eng_.isDead(v.ref)) {
        if (d.limbMask) eng_.decorateCorpse(v.ref, d.limbMask);
        ++stats_.corpsesDecorated;
        decor_.erase(who);
    } else {
        decor_[who] = PendingDecor{d, now_ + cfg_.deathDecorTtlMs};   // the kill (ActorSync) has not reached this machine yet
    }
}

void CombatSync::awardXp(const cc::DeathDecl& d) {
    cc::KillRecord k;
    k.deathId = d.deathId; k.victim = d.who; k.killer = d.killer;
    if (d.killer.isActor()) { EntityView kv; if (world_.entity(d.killer, kv) && kv.known) k.killerRecruiter = kv.recruiter; }
    EntityView v;
    int32_t xp = -1;
    if (d.who.isActor() && world_.entity(d.who, v) && v.known && v.ref) xp = eng_.killXpOf(v.ref);
    if (xp < 0) { ++stats_.xpUnknown; return; }
    k.xp = xp;
    auto it = lastHit_.find(d.who);   // contributors are informational (PartyShare splits among the players present)
    if (it != lastHit_.end()) k.contributors.assign(it->second.players.begin(), it->second.players.end());
    // One death pays once. The DeathLedger already stops a second declaration; the XpLedger is the second line (same death id => no second payout).
    const auto awards = xp_.award(k, cfg_.xpMode, world_.presentPlayers(), validator_.rules());
    // The local engine already paid the kill XP when the local player's own hit killed an actor this machine simulates (a native hit): paying again doubles it.
    const bool enginePaid = killWasNative(d);
    for (const auto& a : awards) {
        if (a.slot != cfg_.self || a.xp <= 0) continue;
        if (enginePaid) { ++stats_.xpEnginePaid; continue; }
        if (eng_.awardXp(a.xp)) { ++stats_.xpAwards; stats_.xpTotal += static_cast<uint64_t>(a.xp); }
    }
}

bool CombatSync::killWasNative(const cc::DeathDecl& d) const {
    auto it = lastHit_.find(d.who);
    return it != lastHit_.end() && it->second.natural && d.killer == cc::Entity::player(cfg_.self) && !FNV_MUT(780);
}

// ---- plugin/companions hooks (Config::social). Every one is a no-op while its function is empty. -------------------------------------------------
void CombatSync::socialDeath(const cc::DeathDecl& d) {
    if (!d.who.isActor()) return;
    EntityView v;
    const bool haveVictim = world_.entity(d.who, v) && v.known;
    if (cfg_.social.reportKill && (d.declarer == cfg_.self || FNV_MUT(784))) {   // 784: every machine that accepts the declaration reports it (the same kill, N times)
        fnvcoop::social::KillReport r;
        r.deathId = d.deathId;
        r.victim.who = d.who;
        if (haveVictim) {
            r.victim.faction = v.faction;
            r.victim.named = v.named;
            r.victim.hostileToParty = !v.faction.plugin.empty() && hostility_.hostile(FormRef("falloutnv.esm", 0x1BF3E), v.faction);
            if (v.ref) { const int32_t xp = eng_.killXpOf(v.ref); r.victim.xp = xp > 0 ? xp : 0; }
        }
        r.killer = d.killer;
        if (d.killer.isActor()) { EntityView kv; if (world_.entity(d.killer, kv) && kv.known) r.killerRecruiter = kv.recruiter; }
        auto it = lastHit_.find(d.who);
        if (it != lastHit_.end()) r.contributors.assign(it->second.players.begin(), it->second.players.end());
        if (killWasNative(d) && !FNV_MUT(785)) r.nativeMask = static_cast<uint8_t>(1u << cfg_.self);   // 785: forgotten; otherwise the local engine paid this player's XP and karma itself
        r.cell = d.cell;
        r.where = d.where;
        ++stats_.socialKillsReported;
        cfg_.social.reportKill(r);
    }
    // The murder: reported by the machine that simulates the killer (the killing player, or the player a killing companion follows).
    if (cfg_.social.reportCrime && haveVictim && !d.killer.none()) {
        EntityView kv;
        if (d.killer.isPlayer()) {
            if (d.killer.id == cfg_.self || FNV_MUT(788)) {   // 788: every machine that accepts the death reports the murder
                world_.entity(d.killer, kv);
                socialCrime(d.killer, kv, d.who, v, true, killWasNative(d) ? static_cast<uint8_t>(1u << cfg_.self) : uint8_t(0));
            }
        } else if (world_.entity(d.killer, kv) && kv.known && kv.recruiter == cfg_.self) {
            socialCrime(d.killer, kv, d.who, v, true, 0);
        }
    }
}

void CombatSync::socialCrime(const cc::Entity& attacker, const EntityView& av, const cc::Entity& victim, const EntityView& vv, bool killed, uint8_t nativeMask) {
    if (!cfg_.social.reportCrime || !victim.isActor() || vv.faction.plugin.empty()) return;
    fnvcoop::social::CrimeEvent e;
    if (attacker.isPlayer() && attacker.id < kMaxSlots) {
        e.criminal = static_cast<PlayerId>(attacker.id);
    } else if (attacker.isActor() && av.known && av.recruiter != kNoPlayer) {
        e.criminal = av.recruiter;   // a companion's crime is its recruiter's
        e.byCompanion = true;
    } else {
        return;
    }
    e.kind = killed ? fnvcoop::social::CrimeKind::Murder : fnvcoop::social::CrimeKind::Assault;
    if (!FNV_MUT(786)) {   // 786: no throttle (an automatic weapon is one report per bullet over the wire)
        auto& last = socialCrimeAt_[std::make_tuple(e.criminal, static_cast<uint8_t>(e.kind), vv.faction)];
        if (last != 0 && now_ - last < 3000) return;
        last = now_ ? now_ : 1;
    }
    e.victim = victim;
    e.faction = vv.faction;
    e.cell = vv.cell;
    e.where = vv.pos;
    e.victimHostile = hostility_.hostile(FormRef("falloutnv.esm", 0x1BF3E), vv.faction);   // already an enemy of the player faction: fighting it is no crime
    e.victimDead = killed;
    e.nativeMask = FNV_MUT(787) ? uint8_t(0) : nativeMask;   // 787: the engine's own reputation is forgotten (the local player is charged twice)
    e.tick = static_cast<uint32_t>(now_);
    ++stats_.socialCrimesReported;
    cfg_.social.reportCrime(e);
}

bool CombatSync::protectedBlocks(uint32_t victimRef, PlayerId attacker, bool splash) {
    if (!cfg_.social.blockProtectedHit || !victimRef || FNV_MUT(796)) return false;   // 796: the story-actor protection is never asked
    if (!cfg_.social.blockProtectedHit(victimRef, attacker, splash) && !FNV_MUT(797)) return false;   // 797: its answer is ignored (every accidental hit lands)
    ++stats_.protectedHitsBlocked;
    return true;
}

bool CombatSync::socialNeutralizes(uint32_t attackerRef, uint32_t victimRef) {
    if (!cfg_.social.neutralizeHit || !attackerRef || !victimRef || FNV_MUT(781)) return false;   // 781: the policy is never asked
    if (!cfg_.social.neutralizeHit(attackerRef, victimRef) && !FNV_MUT(782)) return false;   // 782: the answer is ignored (every hit with an attacker becomes harmless)
    ++stats_.socialHitsNeutralized;
    return true;
}

void CombatSync::maybeReportCrime(const cc::HitEvent& h, const EntityView& vv, bool killed) {
    if (!cfg_.crimeSync || !h.attacker.isPlayer() || !h.victim.isActor() || vv.faction.plugin.empty()) return;
    cc::CrimeReport r;
    r.criminal = static_cast<PlayerId>(h.attacker.id); r.victim = h.victim; r.victimFaction = vv.faction;
    r.criminalFaction = FormRef("falloutnv.esm", 0x1BF3E);
    r.killed = killed; r.witnessed = true; r.reporter = cfg_.self;
    ++stats_.crimesReported;
    if (cfg_.isHost) onCrime(cfg_.self, MCrime{r});
    else { Envelope env; env.src = cfg_.self; env.dest = kHostPlayer; env.body = MCrime{r}; send(env); }
}

void CombatSync::onCrime(PlayerId from, const MCrime& m) {
    if (!cfg_.isHost) return;
    const auto effects = hostility_.onCrime(m.r, cfg_.self, from);
    for (const auto& fx : effects) {
        hostility_.accept(fx, cfg_.self);   // the host books its own decision (and its replay guard)
        if (eng_.applyHostility(fx)) ++stats_.hostilityApplied;
        Envelope env; env.src = cfg_.self; env.dest = kNoPlayer; env.body = MHostility{fx};
        send(env);
    }
}

void CombatSync::onHostility(PlayerId from, const MHostility& m) {
    if (cfg_.isHost) return;   // the host decided it
    if (!hostility_.accept(m.e, from)) { ++stats_.hostilityDropped; return; }
    if (eng_.applyHostility(m.e)) ++stats_.hostilityApplied;
}

// ------------------------------------------------------------------------------------------------ projectiles and explosions
void CombatSync::onProjectileFired(const cc::ProjectileSpawn& in) {
    cc::ProjectileSpawn p = in;
    if (p.id == 0) p.id = nextId();
    EntityView sv;
    if (!world_.entity(p.shooter, sv) || !sv.known || !sv.simulatedHere) { ++stats_.projectilesRejected; return; }
    p.shooterEpoch = p.shooter.isActor() ? sv.epoch : 0;
    const cc::ProjectileVerdict v = projectiles_.onSpawn(p, cfg_.self, cfg_.self, now_, *adapter_);
    if (!v.accept) { ++stats_.projectilesRejected; remember(cc::toString(v.reason), "own projectile"); return; }
    ++stats_.projectilesLocal;
    Envelope env; env.src = cfg_.self; env.dest = kNoPlayer; env.body = MProjectile{p};
    send(env);
}

void CombatSync::onProjectile(PlayerId from, const Envelope& e, const MProjectile& m) {
    (void)from;
    const cc::ProjectileVerdict v = projectiles_.onSpawn(m.p, e.src, cfg_.self, now_, *adapter_);
    if (!v.accept) { ++stats_.projectilesRejected; remember(cc::toString(v.reason), "projectile " + std::to_string(m.p.id)); return; }
    if (cfg_.isHost && !FNV_MUT(716)) fanOut(e, e.src);   // 716: the host does not pass projectiles on
    if (cc::mayDealDamage(v.role)) return;   // cannot happen for a message from another machine: a remote shot is a visual copy
    ++stats_.projectilesVisual;
    visualProjectileUntil_ = now_ + 3000;
    eng_.spawnVisualProjectile(m.p);
}

void CombatSync::onExplosion(PlayerId from, const Envelope& e, const MExplosion& m) {
    (void)from;
    const cc::ExplosionVerdict v = explosions_.judge(m.e, e.src, cfg_.self, *adapter_);
    if (!v.accept) { ++stats_.explosionsRejected; remember(cc::toString(v.reason), "explosion " + std::to_string(m.e.id)); return; }
    if (cfg_.isHost && !FNV_MUT(719)) fanOut(e, e.src);   // 719: the host does not pass explosions on
    if (v.authoritative) return;   // our own (cannot arrive from the network)
    ++stats_.explosionsVisual;
    visualBlasts_.push_back(VisualBlast{m.e.pos, m.e.explosionBase, now_ + 2000});
    if (visualBlasts_.size() > 32) visualBlasts_.erase(visualBlasts_.begin());
    eng_.spawnVisualExplosion(m.e);
}

void CombatSync::planBlast(Blast& b) {
    std::vector<cc::BlastCandidate> cands;
    if (!b.plan) return;
    for (const cc::Entity& c : world_.entitiesNear(b.ev.cell, b.ev.pos, b.ev.radius + 300.0f)) {
        if (b.handled.count(c)) { ++stats_.blastHitsSkipped; continue; }
        EntityView v;
        if (!world_.entity(c, v) || !v.known || !v.alive) continue;
        cands.push_back({c, v.pos, v.cell, v.radius});
    }
    for (cc::HitEvent& h : cc::planExplosionHits(b.ev, cands, validator_.rules())) {
        EntityView vv, av;
        if (!world_.entity(h.victim, vv) || !vv.known) continue;
        ++stats_.blastHitsPlanned;
        if (vv.simulatedHere) continue;   // the local engine resolved (or will resolve) its own victims natively
        if (h.victim.isPlayer() && !cfg_.friendlyFire) continue;
        if (vv.owner == kNoPlayer) continue;
        if (!world_.entity(h.attacker, av)) av = EntityView{};
        if (socialNeutralizes(av.ref, vv.ref)) continue;   // a party member's blast does not hurt the party
        forwardHit(h, vv.owner);
        if (h.victim.isActor()) socialCrime(h.attacker, av, h.victim, vv, false, 0);
    }
}

// ------------------------------------------------------------------------------------------------ time
void CombatSync::tick(uint64_t nowMs) {
    now_ = nowMs;
    // attacker: re-send NACKed hits, forget old ones
    std::vector<uint64_t> resend;
    for (auto it = inflight_.begin(); it != inflight_.end();) {
        Inflight& f = it->second;
        if (now_ - f.sentAt > cfg_.inflightTtlMs) { it = inflight_.erase(it); continue; }
        if (f.nacked && f.retryAt <= now_) resend.push_back(it->first);
        ++it;
    }
    for (uint64_t id : resend) {
        auto it = inflight_.find(id);
        if (it == inflight_.end()) continue;
        Inflight f = it->second;
        inflight_.erase(it);
        EntityView vv;
        if (!world_.entity(f.hit.victim, vv) || !vv.known) { ++stats_.hitsAbandoned; continue; }
        PlayerId owner = f.hit.victim.isPlayer() ? static_cast<PlayerId>(f.hit.victim.id) : vv.owner;
        if (owner == cfg_.self && !FNV_MUT(789)) {
            // The handoff that NACKed this hit finished in OUR favour: we own the victim now and nobody else will ever resolve the hit. Resolve it
            // here, as the owner does for a hit it receives (the copy was clamped when the hit was fired, so the damage is still owed).
            const cc::HitEvent& h = f.hit;
            const cc::HitVerdict v = validator_.judge(h, cfg_.self, cfg_.self, now_, *adapter_);
            if (v.apply) { ++stats_.hitsResolvedLate; applyAccepted(h, cfg_.self, f.touch, f.centi); }
            else { ++stats_.hitsAbandoned; remember(cc::toString(v.reason), v.detail); }
            continue;
        }
        if (owner == kNoPlayer || owner == cfg_.self) { ++stats_.hitsAbandoned; continue; }
        cc::HitEvent h = f.hit;
        if (!f.keepId && !FNV_MUT(717)) h.hitId = (h.explosionId ? h.hitId ^ (static_cast<uint64_t>(f.attempts + 1) << 40) : nextId());
        Inflight n; n.hit = h; n.sentAt = f.sentAt; n.attempts = static_cast<uint8_t>(f.attempts + 1); n.touch = f.touch; n.centi = f.centi;
        inflight_[h.hitId] = n;
        ++stats_.hitRetries;
        Envelope env; env.src = cfg_.self; env.dest = owner; env.body = MHit{h, f.touch, f.centi};
        send(env);
    }
    // owner: a hit that the rules called lethal must have left the victim dead; if the engine still shows it standing, the owner kills it
    for (size_t i = 0; i < pendingKills_.size();) {
        if (pendingKills_[i].dueAt > now_) { ++i; continue; }
        const cc::Entity who = pendingKills_[i].who;
        const bool natural = pendingKills_[i].natural;
        pendingKills_.erase(pendingKills_.begin() + static_cast<long>(i));
        EntityView v;
        if (!world_.entity(who, v) || !v.known || !v.ref || !v.simulatedHere) continue;
        // lifeDead, not isDead: a health <= 0 actor whose life state never flipped (a lethal VATS hit) reads "dead" to isDead and nobody would ever declare it
        if (FNV_MUT(793) ? eng_.isDead(v.ref) : eng_.lifeDead(v.ref)) continue;
        if (natural && v.alive && !FNV_MUT(795)) continue;                                      // it was healed or resurrected in the meantime
        if (natural && (v.named || v.essential) && !FNV_MUT(794)) continue;   // a protected actor does not die: the engine knocks it down
        if (eng_.killActor(v.ref)) { ++stats_.killsForced; if (natural) ++stats_.naturalLethalStuck; }
        else { ++stats_.engineRefused; remember("kill refused", cc::toString(who)); }
    }
    // owner: natural hits publish their result (coalesced)
    for (auto it = vitalsDirty_.begin(); it != vitalsDirty_.end();) {
        if (it->second.dueAt <= now_) {
            const cc::Entity e = it->first;
            it = vitalsDirty_.erase(it);
            publishVitals(e, 0);
            noteNaturalLethal(e);
        } else ++it;
    }
    // explosions: plan hits for victims the local engine did not iterate
    for (auto it = blasts_.begin(); it != blasts_.end();) {
        if (it->second.dueAt <= now_) {
            if (!FNV_MUT(718)) planBlast(it->second);
            it = blasts_.erase(it);
        } else ++it;
    }
    for (auto it = explosionIds_.begin(); it != explosionIds_.end();) it = (now_ - it->second.at > 3000) ? explosionIds_.erase(it) : std::next(it);
    // declarations the ownership table was behind for
    for (size_t i = 0; i < deferredDeaths_.size();) {
        DeferredDeath& d = deferredDeaths_[i];
        if (now_ > d.until) { ++stats_.deathsRejected; deferredDeaths_.erase(deferredDeaths_.begin() + i); continue; }
        const cc::DeathVerdict dv = judgeDeath(d.d, d.sender, d.sender == kHostPlayer);
        if (dv.accept) {
            Envelope relay; relay.src = d.sender; relay.dest = kNoPlayer; relay.body = MDeath{d.d};
            if (cfg_.isHost) fanOut(relay, d.sender);
            handleDeathAccepted(d.d);
            deferredDeaths_.erase(deferredDeaths_.begin() + i);
            continue;
        }
        if (dv.reason != cc::DeathReason::FutureEpoch && dv.reason != cc::DeathReason::UnknownActor) { deferredDeaths_.erase(deferredDeaths_.begin() + i); continue; }
        ++i;
    }
    // corpses that did not exist here yet
    for (auto it = decor_.begin(); it != decor_.end();) {
        if (now_ > it->second.until) { it = decor_.erase(it); continue; }
        EntityView v;
        if (world_.entity(it->first, v) && v.known && v.ref && eng_.isDead(v.ref)) {
            if (it->second.d.limbMask) eng_.decorateCorpse(v.ref, it->second.d.limbMask);
            ++stats_.corpsesDecorated;
            it = decor_.erase(it);
        } else ++it;
    }
    for (auto it = lastHit_.begin(); it != lastHit_.end();) it = (now_ - it->second.at > cfg_.lastHitTtlMs) ? lastHit_.erase(it) : std::next(it);
    for (auto it = killCredit_.begin(); it != killCredit_.end();) it = (now_ > it->second.until) ? killCredit_.erase(it) : std::next(it);
    projectiles_.expire(now_);
}

}  // namespace nvcoop::combat
