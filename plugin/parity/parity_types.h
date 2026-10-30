// Live sync parity checker: shared value types (game-free). One Sample is what one machine sees around its own player at one moment: the actors
// near the local player, every remote player's puppet, and the local player itself. Two machines' samples of the same moment are compared
// (parity_compare.h) to find what the two screens disagree about. Read-only: nothing here ever changes game state. docs/tools/parity.md.
#pragma once
#include <cstdint>
#include <string>
#include <vector>

namespace nvcoop::parity {

constexpr uint8_t kChannelParity = 0x9E;   // first byte of every datagram of this module (docs/protocol.md section 2)
constexpr uint8_t kWireVersion = 1;
constexpr uint8_t kNoGroup = 255;
constexpr uint8_t kNoHp = 255;
constexpr uint8_t kNoSlot = 255;
constexpr size_t kMaxEnts = 64;            // wire decoder limit; the sampler caps well below this

enum class EntKind : uint8_t { Npc = 0, Puppet = 1, Self = 2 };   // Puppet = a remote player's copy on this machine, Self = this machine's own player

struct Look {
    bool known = false;
    uint32_t race = 0;
    bool female = false;
    uint32_t hair = 0, eyes = 0, faceHash = 0;
    bool operator==(const Look& o) const { return known == o.known && race == o.race && female == o.female && hair == o.hair && eyes == o.eyes && faceHash == o.faceHash; }
};

struct Ent {
    EntKind kind = EntKind::Npc;
    uint8_t slot = kNoSlot;      // Puppet / Self: the player slot this entity stands for (the same number on both machines)
    uint32_t ref = 0;            // the local reference id (Npc: the same on both machines when it is a placed actor; Puppet: differs per machine)
    uint32_t base = 0;           // file only
    int32_t x = 0, y = 0, z = 0; // rounded world position, game units
    uint8_t speed16 = 0;         // speed since the previous sample of this entity, units per second / 16 (0 also means "not known yet")
    bool seated = false;         // GetSitting != 0
    bool dead = false;
    bool disabled = false;
    bool weaponOut = false;      // file only
    uint8_t hp = kNoHp;          // health bucket (hpBucket), kNoHp = unreadable
    uint8_t group = kNoGroup;    // primary animation group id (primaryGroup in parity_anim.h), kNoGroup = unreadable
    uint32_t acts = 0;           // Puppet / Self: how many attack animations this machine saw start on this entity (monotonic)
    Look look;                   // Puppet / Self only
};

struct Sample {
    uint8_t slot = kNoSlot;      // the machine that took it
    uint32_t seq = 0;            // per machine, counts up
    uint64_t ms = 0;             // that machine's monotonic clock
    uint64_t wallMs = 0;         // unix time, file only
    double gameTime = 0;         // GameDaysPassed, file only
    uint32_t cell = 0;           // the local player's cell
    uint32_t ws = 0;             // worldspace (0 = interior)
    uint32_t rad = 0;            // listed radius, game units
    uint32_t cov = 0;            // how far around the player this sample is complete: rad, or the distance of the farthest listed actor when the cap cut the list
    bool menu = false;           // a menu was open (a paused or loading game compares badly)
    std::vector<Ent> ents;       // Npc entries nearest first, then Puppet / Self entries
    const Ent* bySlot(uint8_t s) const { for (const Ent& e : ents) if (e.kind != EntKind::Npc && e.slot == s) return &e; return nullptr; }
    const Ent* byRef(uint32_t r) const { for (const Ent& e : ents) if (e.kind == EntKind::Npc && e.ref == r) return &e; return nullptr; }
};

}  // namespace nvcoop::parity
