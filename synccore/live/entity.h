// The one identity of everything the sync moves (docs/entity-core-plan.md 2.1). Header only, game-free.
//
// NetId space:
//   0x00000001 .. 0xFDFFFFFF   placed refs, deriveNetId(plugin, localId), the same on every machine
//   0xFE000000 .. 0xFE0000FF   player entities, low byte = slot
//   0xFF000000 and up          host-issued ids (created actors, loot items, spawn arbitration)
//
// A player is named on every wire as its entity: playerNet(slot), or playerFormRef(slot) where a FormRef is expected (ActorMeta.ref,
// effect payloads). Each machine translates the entity to its own ref through the EntityMap (entity_map.h): 0x14 on the player's own
// machine, that player's puppet everywhere else.
#pragma once
#include <cstdint>
#include <string>

#include "fnvcoop/types.h"

namespace fnvcoop::live {

using EntityNetId = uint32_t;   // the same type as live::NetId (live.h)

constexpr EntityNetId kPlayerNetBase = 0xFE000000u;
constexpr EntityNetId kCreatedNetBase = 0xFF000000u;
constexpr uint32_t kLocalPlayerRef = 0x14;          // the engine's player reference on every machine
constexpr const char* kPlayerPlugin = "@player";    // the plugin part of playerFormRef: no real plugin can have this name

constexpr EntityNetId playerNet(PlayerId slot) { return kPlayerNetBase | slot; }
constexpr bool isPlayerNet(EntityNetId id) { return (id >> 8) == (kPlayerNetBase >> 8); }
constexpr PlayerId slotOfPlayerNet(EntityNetId id) { return static_cast<PlayerId>(id & 0xFF); }

enum class EntityKind : uint8_t { None, Placed, Created, Player };

constexpr EntityKind kindOf(EntityNetId id) {
    if (id == 0) return EntityKind::None;
    if (isPlayerNet(id)) return EntityKind::Player;
    if ((id >> 24) == (kPlayerNetBase >> 24)) return EntityKind::None;   // the rest of 0xFE..: reserved, never issued
    if (id >= kCreatedNetBase) return EntityKind::Created;
    return EntityKind::Placed;
}

// A derived id for a placed ref must never land in the player or the host-issued range.
constexpr EntityNetId maskPlacedNetId(EntityNetId id) {
    const uint32_t top = id >> 24;
    if (top == 0xFF || top == 0xFE) id &= 0x7FFFFFFFu;
    return id ? id : 1u;
}

inline FormRef playerFormRef(PlayerId slot) { return FormRef(kPlayerPlugin, slot); }
inline bool isPlayerFormRef(const FormRef& r) { return r.plugin == kPlayerPlugin && r.localId < kMaxSlots; }
inline PlayerId slotOfPlayerFormRef(const FormRef& r) { return isPlayerFormRef(r) ? static_cast<PlayerId>(r.localId) : kNoPlayer; }

}  // namespace fnvcoop::live
