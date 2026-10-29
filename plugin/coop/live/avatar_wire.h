// Wire format of the avatar channel (first byte kChannelAvatars on the coop MuxTransport). Game-free.
//
//   [kChannelAvatars][live datagram]            kind 1 = SnapshotBatch, kind 2 = SnapshotAck (synccore/live codec, CRC protected)
//   [kChannelAvatars][0x40][n][slot * n]        Roster : host -> clients, reliable. Every slot that has a player right now (host included).
//   [kChannelAvatars][0x41][slot][look][equip]  Look   : owner -> host -> others, reliable. Appearance + what the player wears. (v1, still decoded)
//   [kChannelAvatars][0x43][ver=2][slot][flags][race][hair][eyes][face][hairColor?][height,weight?][faceCoeffs?][name][equip + mods]
//                                               Look v2: the same message with body data (height / weight / hair colour), FaceGen coefficients and the
//                                               weapon modification flags of the held weapon. encodeLook always writes this one.
//   [kChannelAvatars][0x44][ver=1][slot][level u16][sitting u8][actions u32][n u8][region u32 * n]
//                                               Facts  : client -> host ONLY, reliable, on change (W3a item 3). What a host-run condition asks about the guest that no snapshot
//                                               carries: GetLevel, GetSitting, IsPlayerActionActive (bit n = action id n, 0..31), IsPlayerInRegion (the region forms the player is in).
//                                               Sex, worn items and sneaking are already in Look / the snapshot.
//   [kChannelAvatars][0x45][ver=1][slot][flags u8][watchId u32][weight u32 if flags&1][n u16][(form u32, count i32) * n]
//                                               Pack   : client -> host ONLY, reliable, on change and at most once per packMinGapMs (guest pack facts). What a guest carries that
//                                               the host cannot see on a puppet: the InventoryWeight actor value (thousandths) and the counts of the WATCHED item forms (only counts above zero;
//                                               the watch list is nvcoop_pack_watch.json, generated from the rules and the quest conditions; watchId identifies it, so the host reads an absent form as
//                                               "zero" only when its own list has the same id). GetItemCount / GetActorValue InventoryWeight / PooledItems of a guest are answered from it.
//   [kChannelAvatars][0x42]                     Resync : client -> host, reliable. Answered with the Roster and every known Look for that client.
//
// Under [entities] parts=unified (docs/entity-core-plan.md) Roster, Look, Facts, Pack and Resync are no longer SENT: the player entities' parts carry them (ActorSync). They are
// always decoded, whichever mode is set. The field codecs (Look, Facts, Pack) are shared with those parts: plugin/avatar/look_codec.h.
//
// Roster and Look are decoded defensively: a truncated or oversized message is rejected as a whole, never half applied.
#pragma once
#include <cstdint>
#include <string>
#include <vector>

#include "avatar_types.h"
#include "look_codec.h"

namespace nvcoop::livewire {

constexpr uint8_t kChannelAvatars = 0xAD;   // 0xAC actors, 0xFC story, 0xD1 dialogue, 0xE5 slides
constexpr uint8_t kAvKindRoster = 0x40;
constexpr uint8_t kAvKindLook = 0x41;
constexpr uint8_t kAvKindLook2 = 0x43;    // Look v2 (kAvatarWireVersion = 2 inside the message)
constexpr uint8_t kAvKindFacts = 0x44;    // Facts (kAvFactsVersion inside the message); 0x45 and up are free
constexpr uint8_t kAvFactsVersion = 1;
constexpr uint8_t kAvKindPack = 0x45;     // Pack (kAvPackVersion inside the message)
constexpr uint8_t kAvPackVersion = 1;
constexpr uint8_t kAvatarWireVersion = 2;
constexpr uint8_t kAvKindResync = 0x42;   // client -> host, reliable: "my host link came back, send me the roster and every look again" (2 bytes)
constexpr size_t kMaxLookEquip = nvcoop::avatar::kMaxLookEquip;   // the field codecs moved to plugin/avatar/look_codec.h; the limits stay under their old names
constexpr size_t kMaxLookName = nvcoop::avatar::kMaxLookName;

using AvBytes = std::vector<uint8_t>;

struct AvLook {
    uint8_t slot = 0xFF;
    nvcoop::avatar::Appearance appearance;
    std::vector<nvcoop::avatar::EquipEntry> equipment;
    friend bool operator==(const AvLook& a, const AvLook& b);
};

struct AvFacts {
    uint8_t slot = 0xFF;
    nvcoop::avatar::PlayerFacts facts;
    friend bool operator==(const AvFacts& a, const AvFacts& b) { return a.slot == b.slot && a.facts == b.facts; }
};
AvBytes encodeFacts(const AvFacts& f);
bool decodeFacts(const uint8_t* data, size_t size, AvFacts& out);                   // `data` starts at the channel byte

struct AvPack {
    uint8_t slot = 0xFF;
    nvcoop::avatar::PlayerPack pack;
    friend bool operator==(const AvPack& a, const AvPack& b) { return a.slot == b.slot && a.pack == b.pack; }
};
AvBytes encodePack(const AvPack& p);
bool decodePack(const uint8_t* data, size_t size, AvPack& out);                     // `data` starts at the channel byte

AvBytes encodeRoster(const std::vector<uint8_t>& slots);
bool decodeRoster(const uint8_t* data, size_t size, std::vector<uint8_t>& slots);   // `data` starts at the channel byte

AvBytes encodeLook(const AvLook& l);
bool decodeLook(const uint8_t* data, size_t size, AvLook& out);                     // `data` starts at the channel byte

// NetId of a player's avatar stream: never in the derived-actor space (top byte 0xFE, low byte = slot).
inline uint32_t playerNetId(uint8_t slot) { return 0xFE000000u | slot; }
inline bool isPlayerNetId(uint32_t id) { return (id >> 8) == 0xFE0000u; }
inline uint8_t slotOfPlayerNetId(uint32_t id) { return static_cast<uint8_t>(id & 0xFF); }

}  // namespace nvcoop::livewire
