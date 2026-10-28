// plugin/companions: the ports SocialSync talks through (FULL-SYNC-PLAN 3.4 crime, 3.8 XP, 3.10 companions). Game-free: no xNVSE, no Windows header.
//   ISocialWorld   who is who on THIS machine (an NPC by wire id, the players' places, the engine's teammate list, which NPCs of a faction this machine runs).
//                  Production: social_module.cpp over plugin/actors/engine/actor_engine_api.h and the xNVSE layouts; tests: a scripted world.
//   ISocialEngine  what SocialSync asks the game to DO or READ (flag a teammate, set a faction flag, alert an NPC, give XP ...). Production: console lines
//                  run on a calling reference plus the verified reads; tests: a recording fake.
//   ISocialNet     datagram out (the coop MuxTransport; the first byte is already kChannelSocial).
//   ICommitter     host only: commit effects through the story session (ModActorValue / ModKarma / ModReputation / CompanionOwner).
// The rules themselves are synccore/social (companions.h, crime.h, rewards.h); this layer is wiring, ordering and the "who applies it" decisions.
#pragma once
#include <cstdint>
#include <string>
#include <vector>
#include "fnvcoop/transport.h"
#include "social/wire.h"

namespace nvcoop::social {

namespace sc = fnvcoop::social;
namespace cc = fnvcoop::combat;
using fnvcoop::Bytes;
using fnvcoop::Effect;
using fnvcoop::FormRef;
using fnvcoop::PlayerId;
using fnvcoop::kHostPlayer;
using fnvcoop::kMaxSlots;
using fnvcoop::kNoPlayer;

constexpr uint8_t kChannelSocial = sc::kChannelSocial;

// One NPC as this machine sees it. `ref` is the engine reference that exists HERE (0 = not loaded on this machine).
struct ActorView {
    bool known = false;
    FormRef npc;                     // the placed reference's wire id
    uint32_t netId = 0;              // the actor layer's net id (0 = not tracked)
    uint32_t ref = 0;
    PlayerId owner = kNoPlayer;      // who simulates it (the actor layer's committed owner)
    bool simulatedHere = false;      // this machine runs its AI (owner, no handoff in flight, not a stand-in)
    bool standIn = false;
    bool alive = true;
    bool waiting = false;            // told to wait (the companion script's `Waiting` variable); only read for companions
    uint32_t cell = 0;
    cc::Vec3 pos;
    FormRef faction;                 // its crime faction (empty = none)
    bool critter = false;            // a creature (counts against the critter follower cap)
};

class ISocialWorld {
public:
    virtual ~ISocialWorld() = default;
    virtual bool actor(const FormRef& npc, ActorView& out) const = 0;
    virtual bool actorByNet(uint32_t netId, ActorView& out) const = 0;
    // Every player this machine knows with the cell and position it last saw (the local one first-hand, remote ones from avatar snapshots).
    virtual std::vector<sc::PlayerPlace> players() const = 0;
    virtual uint32_t refOfPlayer(PlayerId slot) const = 0;          // the local player (0x14) or the puppet; 0 = none
    // The local engine's teammate list (the local player's followers), as wire ids. Entries that have no wire id are left out.
    virtual std::vector<FormRef> teammatesHere() const = 0;
    // NPCs of `faction` within `radius` of `where` in `cell` that THIS machine simulates (it alerts those; other machines alert theirs).
    virtual std::vector<FormRef> factionActorsHere(const FormRef& faction, uint32_t cell, const cc::Vec3& where, float radius) const = 0;
};

class ISocialEngine {
public:
    virtual ~ISocialEngine() = default;
    // ---- companions
    virtual bool setTeammate(const FormRef& npc, bool on) = 0;                              // SetPlayerTeammate 1/0 ON the NPC
    virtual bool applyCompanionState(const FormRef& npc, const sc::CompanionState& s) = 0;   // the owner's machine only; false = no engine mapping
    virtual bool summonCompanion(const FormRef& npc, PlayerId owner) = 0;                    // MoveTo the owner's player
    virtual bool readInventory(const FormRef& npc, sc::ItemCounts& out) = 0;                 // changes against the base container; false = not readable now
    virtual bool applyInventoryDelta(const FormRef& npc, const FormRef& item, int32_t delta) = 0;   // AddItem / RemoveItem ON the NPC (no capture)
    // ---- crime
    virtual bool applyStanding(const FormRef& faction, const sc::FactionStanding& st) = 0;  // SetPCFactionAttack / SetPCFactionMurder / SetPCEnemyOfFaction
    virtual bool applyHostility(const cc::HostilityEffect& e) = 0;                          // SetEnemy / SetAlly
    virtual bool alertActor(const FormRef& npc, uint32_t criminalRef) = 0;                  // StartCombat <criminal> ON the NPC
    // NPCs this machine simulates that see a player right now (owner side detections). Cheap; called a few times a second.
    virtual void observeDetections(std::vector<sc::Detection>& out) = 0;
    // The LOCAL player is trespassing right now (IsTrespassing): fills the facts and where.
    virtual bool localTrespass(sc::InteractionFacts& facts, uint32_t& cell, cc::Vec3& where) = 0;
    // The HOST reads what a guest's interaction touches: `pickup` = a take / pick up, else an activation. false = nothing crime-relevant about the target.
    virtual bool readInteraction(const FormRef& target, bool pickup, sc::InteractionFacts& facts, uint32_t& cell, cc::Vec3& where) = 0;
    // ---- rewards (the LOCAL player)
    virtual bool awardXp(int32_t xp) = 0;
    virtual bool rewardKarma(int32_t delta) = 0;
    virtual bool modReputation(const FormRef& faction, int32_t fame, int32_t infamy) = 0;
};

class ISocialNet {
public:
    virtual ~ISocialNet() = default;
    // Client: `peer` is always the host. Host: any joined client. `data` already starts with kChannelSocial. Reliable-ordered.
    virtual void send(PlayerId peer, Bytes data) = 0;
};

class ICommitter {
public:
    virtual ~ICommitter() = default;
    // Host only: commit effects as ONE txn (origin = the host). Returns the seq, 0 = nothing committed. `what` is a short tag for logs.
    virtual uint64_t commit(const std::vector<Effect>& effects, const char* what) = 0;
};

}  // namespace nvcoop::social
