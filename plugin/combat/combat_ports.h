// plugin/combat: the three ports CombatSync talks through (FULL-SYNC-PLAN 3.4). Game-free: no xNVSE, no Windows header.
//   IWorldView    who is who (engine ref <-> combatant), who owns it, where it is, how healthy it is. Production: combat_glue.cpp over
//                 plugin/actors/engine/actor_engine_api.h; tests: a scripted world.
//   ICombatEngine the effects CombatSync asks the game for (apply an accepted hit, cripple a limb, award XP ...). Production: console lines
//                 run on a calling reference (the bridge's exec{cmd, ref} path) plus the verified engine reads; tests: a recording fake.
//   ICombatNet    datagram out (the coop MuxTransport; the first byte is already kChannelCombat).
// The rules themselves (validation, limbs, death ledger, projectiles, explosions, XP, hostility) are synccore/combat (combat-core).
#pragma once
#include <cstdint>
#include <string>
#include <vector>
#include "combat.h"
#include "fnvcoop/transport.h"

namespace nvcoop::combat {

namespace cc = fnvcoop::combat;
using fnvcoop::Bytes;
using fnvcoop::FormRef;
using fnvcoop::OwnerEpoch;
using fnvcoop::PlayerId;
using fnvcoop::kHostPlayer;
using fnvcoop::kNoPlayer;
using fnvcoop::kMaxSlots;

// First byte of every datagram this module sends through the coop MuxTransport (0xFC story, 0xD1 dialogue, 0xE5 slides, 0xAC actors,
// 0xA1 reserved by the handshake). Registered in docs/protocol.md section 14.
constexpr uint8_t kChannelCombat = 0xCB;
constexpr uint8_t kCombatModuleWire = 2;   // 2: entities are bare NetIds (synccore/combat wire v2)

// ---- identity and facts --------------------------------------------------------------------------------------------------------
// Everything CombatSync asks about one combatant (a player slot or a synced actor). Refreshed on every call: nothing is cached by the caller.
struct EntityView {
    bool known = false;
    cc::Entity who;
    uint32_t ref = 0;                // engine runtime ref of the copy/real actor that exists HERE; 0 = not loaded on this machine
    PlayerId owner = kNoPlayer;      // actor: committed owner (kNoPlayer = nobody yet); player: its own slot
    OwnerEpoch epoch = 0;            // actor: owner epoch; player: 0
    OwnerEpoch entityEpoch = 0;      // player: the epoch of its pinned entity (S7; rises with every rejoin), 0 = not known. Judges a player's hits only
    bool simulatedHere = false;      // this machine runs its AI / is that player's machine. False for a copy, a stand-in, a puppet
    bool pending = false;            // an ownership handoff is in flight
    bool standIn = false;
    bool alive = true;
    bool named = false;              // story NPC: only a host KillActor kills it
    bool essential = false;
    cc::CellId cell = 0;
    cc::Vec3 pos;
    float radius = 0;
    int32_t health = 0, maxHealth = 0;
    uint8_t limbMask = 0;            // crippled limbs as the engine shows them here
    PlayerId recruiter = kNoPlayer;  // companions
    FormRef faction;                 // empty = unknown (crime reports are skipped)
};

class IWorldView {
public:
    virtual ~IWorldView() = default;
    virtual PlayerId self() const = 0;
    // The local player's ref and every puppet map to Entity::player(slot); a tracked actor (real or stand-in) maps to Entity::actor(netId).
    // false = scenery, an untracked actor, an unloaded ref.
    virtual bool entityOfRef(uint32_t ref, cc::Entity& out) const = 0;
    virtual bool entity(const cc::Entity& e, EntityView& out) const = 0;
    virtual std::vector<cc::Entity> entitiesNear(cc::CellId /*cell*/, const cc::Vec3& /*pos*/, float /*radius*/) const { return {}; }
    virtual std::vector<PlayerId> presentPlayers() const = 0;
    // Rule tables the validator may use (all optional).
    virtual bool cellsAdjacent(cc::CellId a, cc::CellId b) const { return a == b; }
    virtual const cc::WeaponInfo* weapon(const FormRef& /*id*/) const { return nullptr; }
    virtual bool lineOfFire(const cc::Vec3& /*from*/, const cc::Vec3& /*to*/) const { return true; }
};

// ---- engine effects ------------------------------------------------------------------------------------------------------------
// Every call returns true when the game accepted it. false is an ORDINARY event (ref gone, console refused): the caller counts it.
class ICombatEngine {
public:
    virtual ~ICombatEngine() = default;
    // The owner applies an ACCEPTED hit: `amount` health points off the victim (already limited by named/essential protection).
    // `attackerRef` is the attacker's ref on THIS machine (the puppet of a remote player, a local actor), 0 = none. Production:
    // DamageActorValue Health on the victim (or the engine's own hit path when the native replay is enabled).
    virtual bool damageHealth(const cc::Entity& victim, uint32_t victimRef, int32_t amount, uint32_t attackerRef, const cc::HitEvent& hit) = 0;
    // damageHealth with the exact (fractional) amount the attacker's engine computed. Default: the nearest whole point (an engine without float health).
    virtual bool damageHealthExact(const cc::Entity& victim, uint32_t victimRef, float amount, uint32_t attackerRef, const cc::HitEvent& hit) {
        return damageHealth(victim, victimRef, amount > 0 && amount < 1 ? 1 : static_cast<int32_t>(amount + 0.5f), attackerRef, hit);
    }
    // The owner replays an ACCEPTED blow that took under one point of health, or none (armour took it all, a glancing fist): the victim's hit reaction and its notice of
    // the attacker, with exactly `amount` points off its float health (0 = nothing; production: the native replay, a token amount for 0). The rules' whole-point health
    // is not touched by it. `attackerRef` as for damageHealth. Default false: only provoke() then runs.
    virtual bool touchVictim(uint32_t /*victimRef*/, uint32_t /*attackerRef*/, const cc::HitEvent& /*hit*/, float /*amount*/) { return false; }
    virtual bool setLimbCrippled(const cc::Entity& victim, uint32_t victimRef, cc::Limb limb) = 0;
    // A copy mirrors the owner's published health (never raises a dead actor, never kills: the death arrives as a declaration).
    virtual bool mirrorHealth(uint32_t ref, int32_t health) = 0;
    // The victim's owner makes the victim fight back / react to the attacker (StartCombat attackerRef).
    virtual bool provoke(uint32_t victimRef, uint32_t attackerRef) = 0;
    // Corpse decoration from a death declaration (crippled limbs show on the body). Only called once the actor is dead here.
    virtual bool decorateCorpse(uint32_t ref, uint8_t limbMask) = 0;
    virtual bool isDead(uint32_t ref) = 0;
    // The engine's LIFE STATE says dying / dead (Actor+0x108 = 1 or 2). isDead() also answers true for an actor whose health is <= 0 but whose life state
    // never flipped (a lethal VATS hit: the engine takes the health off and runs the death later, or not at all); only a life state death is declared by
    // the actor port's poll, so only this one lets the owner skip the kill. Default: isDead (an engine without the distinction).
    virtual bool lifeDead(uint32_t ref) { return isDead(ref); }
    // The owner makes a victim dead that an accepted lethal hit left standing (the engine's float health kept a sliver the rounded health did not
    // show, the replay was reduced by armour ...). The actor port then sees the death and declares it. false = could not (default: no engine).
    virtual bool killActor(uint32_t /*victimRef*/) { return false; }
    // G04 kill attribution (host): the same kill credited to the puppet of the REMOTE player that shot the victim, so the engine stores it as the killer before OnDeath and
    // `IsKiller player` is true for that guest (production: nvhooks::killWithAttribution, `KillActor <killer>` on the victim). Default: no attribution, the plain kill.
    virtual bool killActorBy(uint32_t victimRef, uint32_t /*killerRef*/) { return killActor(victimRef); }
    // ... and for a corpse that is already here (its death reached this machine before the declaration did): writes the killer only (nvhooks::setKillerOf). false = cannot.
    virtual bool setKiller(uint32_t /*victimRef*/, uint32_t /*killerRef*/) { return false; }
    // Kill XP. killXpOf returns the XP value of the victim, -1 = unknown (then no award; TODO-RE: the engine's per-victim XP).
    virtual int32_t killXpOf(uint32_t victimRef) = 0;
    virtual bool awardXp(int32_t xp) = 0;              // the LOCAL player only
    // Visual-only copies of another machine's projectile / explosion. They must never hurt anybody here (the hit gate neutralises their hits).
    virtual bool spawnVisualProjectile(const cc::ProjectileSpawn& p) = 0;
    virtual bool spawnVisualExplosion(const cc::ExplosionEvent& e) = 0;
    // SetEnemy / SetAlly / StopCombat from the host (crime and hostility).
    virtual bool applyHostility(const cc::HostilityEffect& e) = 0;
    virtual bool vatsActive() = 0;                     // the local player is in VATS (flags the captured hit)
};

// ---- network ---------------------------------------------------------------------------------------------------------------------
class ICombatNet {
public:
    virtual ~ICombatNet() = default;
    // Client: `peer` is always the host. Host: any joined client. `data` already starts with kChannelCombat. Everything is reliable-ordered.
    virtual void send(PlayerId peer, Bytes data) = 0;
};

}  // namespace nvcoop::combat
