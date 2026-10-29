// Wire messages of the actor module. Snapshot streams (delta encoded, unreliable) are live-core's SnapshotBatch / SnapshotAck bytes carried
// verbatim; everything else is one Envelope per datagram, always reliable-ordered.
//
// Datagram layout: [kChannelActors][sub][payload]
//   sub 0  SubBatch     payload = live::SnapshotSender::flush() bytes     (unreliable)
//   sub 1  SubAck       payload = live::SnapshotReceiver::takeAcks() bytes (unreliable)
//   sub 2  SubEnvelope  payload = fnvcoop::Archive of an Envelope          (reliable)
//   sub 3  SubBatchPlayer / sub 4 SubAckPlayer: the same two packets for the player stream's own sender and receiver (StreamProfile, S10)
#pragma once
#include <variant>
#include "actors/actor_types.h"

namespace fnvcoop { class Archive; }

namespace nvcoop::actors {

// 2: ActorSnapshot carries the live schema v2 animation extension (synccore/live/live.h), + MAnimEvent. 3: MPart (a state part, state_parts.h)
// took the place of MInventory (inventory is one part now, equipment another).
constexpr uint8_t kActorWireVersion = 3;
enum Sub : uint8_t { SubBatch = kSubBatchNpc, SubAck = kSubAckNpc, SubEnvelope = 2, SubBatchPlayer = kSubBatchPlayer, SubAckPlayer = kSubAckPlayer };

struct MOwn { fnvcoop::OwnMsg m; };                                   // host -> all; client -> host only for Ack
struct MTrack { ActorMeta meta; };                                    // client -> host: "I have this actor loaded"
// Full (non-delta) state, reliable: the old owner's last state in a handoff (final) or the one reliable copy of a settled actor.
struct MSnapFull { live::ActorSnapshot s; bool final = false; bool settle = false; };
struct MDeath { NetId id = 0; OwnerEpoch epoch = 0; uint64_t eventId = 0; CellId cell = 0; live::Vec3 pos; };   // owner's death declaration
struct MPick { NetId id = 0; FormRef pick; };                         // host -> all: the one leveled pick of this actor
struct MPickReport { NetId id = 0; FormRef base; };                   // owner -> host: the base form the engine rolled
struct MResync { NetId id = 0; };                                     // client -> host: "my view is behind, tell me the owner"

// Opaque reliable event from the actor's owner to everyone (enemy-anim: the settled corpse pose). ActorSync only judges the sender (owner,
// current epoch), de-duplicates by eventId and hands `payload` to the registered AnimEventHandler; it never looks inside.
struct MAnimEvent { NetId id = 0; OwnerEpoch epoch = 0; uint64_t eventId = 0; Bytes payload; };

// One state part of an actor (state_parts.h): the owner's value in its canonical bytes. Owner to everyone, reliable.
struct MPart { NetId id = 0; OwnerEpoch epoch = 0; uint64_t eventId = 0; PartKind kind = PartKind::Inventory; Bytes value; };

// Host -> all: this record is gone for everybody (a player left; docs/entity-core-plan.md S3). Clients drop the record and its receiver state.
struct MUntrack { NetId id = 0; };

// The event bus (docs/entity-core-plan.md S6): one reliable envelope for everything that happens to an entity.
//   Owner    the entity's owner tells everyone; the host judges owner + epoch, relays, everyone de-duplicates by eventId
//   ToOwner  anyone asks the entity's CURRENT owner (a hit on it, an activation); the host routes it and re-routes when the owner changed
//   Nack     host -> origin: the entity has no owner to take the request (the origin may retry or drop it)
enum class EventRoute : uint8_t { Owner = 0, ToOwner = 1, Nack = 2 };
struct MEvent {
    NetId id = 0; OwnerEpoch epoch = 0; uint64_t eventId = 0;
    uint8_t kind = 0;                     // what the payload is; ActorSync never looks inside (handlers per kind)
    EventRoute route = EventRoute::Owner;
    uint8_t hops = 0;                     // ToOwner: re-routes so far (a limit stops a request that chases a moving owner)
    PlayerId origin = kNoPlayer;          // who sent it first (a ToOwner's answer and a Nack go there)
    Bytes payload;
};

using Body = std::variant<MOwn, MTrack, MSnapFull, MDeath, MPick, MPickReport, MResync, MAnimEvent, MPart, MUntrack, MEvent>;   // append only: the index is on the wire

struct Envelope {
    PlayerId src = kNoPlayer;
    PlayerId dest = kNoPlayer;   // kNoPlayer = everyone except src (a client's broadcast is relayed by the host)
    Body body;
};

const char* bodyName(const Body& b);
Bytes encodeEnvelope(const Envelope& e);                                    // full datagram, kChannelActors and sub included
bool decodeEnvelope(const uint8_t* payload, size_t size, Envelope& out);    // payload = after the sub byte; false on any malformed input
Bytes wrapLive(Sub sub, const Bytes& liveBytes);                            // full datagram around a live-core packet

void io(fnvcoop::Archive& a, live::ActorSnapshot& s);
void io(fnvcoop::Archive& a, ActorMeta& m);

}  // namespace nvcoop::actors
