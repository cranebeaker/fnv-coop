// PlayerPort: the player half of ActorSync's engine port (docs/entity-core-plan.md S5, S10, S11). ActorSync routes every call for a player entity
// (live::isPlayerNet) here instead of to the NPC engine port; this class answers them from the avatar layer through a narrow endpoint
// (implemented by AvatarNet), so plugin/avatar does not depend on plugin/actors and the NPC port never sees a player.
//
//   parts   : readPart  = the local player's look / equipment / weapon / facts / pack in the canonical bytes of the part
//             writePart = a remote player's value, applied to its puppet (look, equipment) or stored on the host (facts, pack)
//   stream  : sample    = the local player's newest snapshot (the avatar sampler's, 30 Hz) that ActorSync has not taken yet
//             onSnapshot = a remote player's snapshot, judged by ActorSync, for the avatar layer's low-latency playback buffer
//
// A player is never an AI actor: setAi, setProtected, spawnStandIn, setBase, kill and revive are refused/ignored here (and counted, so a test or
// the status can see a call that was routed wrongly). Game-free.
#pragma once
#include <cstdint>

#include "actors/actor_types.h"

namespace nvcoop::livewire {

using nvcoop::actors::Bytes;
using nvcoop::actors::NetId;
using nvcoop::actors::PartKind;
using nvcoop::actors::PlayerId;

// What the port needs from the avatar network service. Slots are the players' slots (live::slotOfPlayerNet).
class IPlayerEndpoint {
public:
    virtual ~IPlayerEndpoint() = default;
    virtual PlayerId playerSelf() const = 0;
    virtual bool playerSessionUp() const = 0;
    virtual bool readLocalPart(PartKind kind, Bytes& out) = 0;                                  // false = unreadable / nothing to say yet
    virtual bool writeRemotePart(PlayerId slot, PartKind kind, const Bytes& value) = 0;        // false = malformed (the replicator retries a few times)
    virtual bool takeLocalSample(fnvcoop::live::ActorSnapshot& out) = 0;                       // the newest local snapshot not taken yet
    virtual void remoteSnapshot(PlayerId slot, const fnvcoop::live::ActorSnapshot& s, uint32_t arrivalMs) = 0;
};

class PlayerPort : public nvcoop::actors::IActorPort {
public:
    explicit PlayerPort(IPlayerEndpoint& ep) : ep_(ep) {}

    bool loaded(NetId id) override;
    bool cellLoaded(nvcoop::actors::CellId) override { return true; }
    bool sample(NetId id, fnvcoop::live::ActorSnapshot& out) override;
    bool pose(NetId, fnvcoop::live::ActualPose&) override { return false; }
    void drive(NetId, const fnvcoop::live::InterpOut&) override { ++wrongCalls_; }
    void applyAv(NetId, const std::vector<fnvcoop::live::AvValue>&) override {}   // health and limbs of a puppet stay the real player's (Config.healMs)
    void place(NetId, const fnvcoop::live::ActorSnapshot&) override { ++wrongCalls_; }
    void setAi(NetId, bool) override { ++wrongCalls_; }
    void setProtected(NetId, bool) override { ++wrongCalls_; }
    bool spawnStandIn(const nvcoop::actors::ActorMeta&, const fnvcoop::live::ActorSnapshot&) override { ++wrongCalls_; return false; }
    void removeStandIn(NetId) override { ++wrongCalls_; }
    void setBase(NetId, const nvcoop::actors::FormRef&) override { ++wrongCalls_; }
    void kill(NetId) override { ++wrongCalls_; }
    void revive(NetId) override { ++wrongCalls_; }
    bool readPart(NetId id, PartKind kind, Bytes& out) override;
    bool writePart(NetId id, PartKind kind, const Bytes& value) override;
    void onSnapshot(NetId id, const fnvcoop::live::ActorSnapshot& s, uint32_t arrivalMs) override;

    uint64_t wrongCalls() const { return wrongCalls_; }   // calls an NPC-only path made on a player entity (must stay 0)

private:
    IPlayerEndpoint& ep_;
    uint64_t wrongCalls_ = 0;
};

}  // namespace nvcoop::livewire
