// Wire messages of plugin/combat (docs/protocol.md section 14). One Envelope per datagram, always reliable-ordered.
//
// Datagram: [kChannelCombat 0xCB][kCombatModuleWire][kind u8][Archive payload (string table + body)]
// The payload structs (HitEvent, DeathDecl, ProjectileSpawn, ExplosionEvent, HostilityEffect) are synccore/combat's own wire (combat.h).
// Star topology like the actor module: a client sends to the host only (dest = who it is meant for); the host judges the routing and relays.
#pragma once
#include <variant>
#include "combat/combat_ports.h"

namespace nvcoop::combat {

enum class Kind : uint8_t { Hit = 1, HitNack, Vitals, Death, Projectile, Explosion, Crime, Hostility, Count_ };

// attacker's machine -> victim's owner (via the host). The rules' `h.damage` is whole points (at least 1, so they accept it); `centi` is the engine's health damage
// EXACTLY, in hundredths (a fist's 0.5 is 50), and is what the owner applies natively. `touch`: the blow did no health damage at all (armour took it all, a glancing
// fist; centi is 0): the owner replays it as a reaction only (no health off, the victim turns on the attacker).
struct MHit { cc::HitEvent h; bool touch = false; uint32_t centi = 0; };
constexpr uint32_t kMaxCenti = 100000000;   // 1,000,000 points: far beyond any real blow, keeps a hostile varint from becoming a float nobody meant
// victim's owner -> attacker: "not applied here, ask again" (handoff in flight, the ownership table was behind, victim not known yet)
struct MHitNack { uint64_t hitId = 0; cc::HitReason reason = cc::HitReason::Ok; bool resync = false; };
// victim's owner -> everyone: the result of an applied hit (health + crippled limbs). `seq` rises per (victim, epoch); stale ones are ignored.
struct MVitals { cc::Entity who; OwnerEpoch epoch = 0; uint32_t seq = 0; int32_t health = 0, maxHealth = 0; uint8_t limbMask = 0; bool dead = false; uint64_t cause = 0; };
struct MDeath { cc::DeathDecl d; };                     // the one declaration of a death (credit, weapon, limbs). The kill itself is ActorSync's
struct MProjectile { cc::ProjectileSpawn p; };          // shooter's owner -> everyone: a visual-only copy elsewhere
struct MExplosion { cc::ExplosionEvent e; };            // source's owner -> everyone: visual elsewhere, damage went out as hits
struct MCrime { cc::CrimeReport r; };                   // anybody -> host
struct MHostility { cc::HostilityEffect e; };           // host -> everyone

using Body = std::variant<MHit, MHitNack, MVitals, MDeath, MProjectile, MExplosion, MCrime, MHostility>;

constexpr uint8_t kMaxHops = 3;

struct Envelope {
    PlayerId src = kNoPlayer;     // author of the message (not the relaying host); filled by the receiver from the transport on first hop
    PlayerId dest = kNoPlayer;    // kNoPlayer = everyone except src (host fan-out) or "the host decides" for a hit
    uint8_t hops = 0;
    Body body;
};

const char* bodyName(const Body& b);
Kind kindOf(const Body& b);
Bytes encodeEnvelope(const Envelope& e);                                    // full datagram, channel and version included
bool decodeEnvelope(const uint8_t* data, size_t size, Envelope& out);      // data = the whole datagram; false on any malformed input

void io(fnvcoop::Archive& a, cc::CrimeReport& r);
void io(fnvcoop::Archive& a, MVitals& v);

}  // namespace nvcoop::combat
