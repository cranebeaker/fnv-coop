// What plugin/coop/coop_module.cpp calls (the only caller outside plugin/markers). Everything is a no-op while the module is off or has no session.
//
//   ApplyOrder  (guest side)   "downed"  -> onLocalDowned(respawnMs)     "respawn" -> onLocalUp(byName)    "party" -> applyGuestOrder(json)
//   HandleOrder (host side)    "revive" / "beacon" -> handleHostOrder(from, json)
//
// The module itself (mk_glue.cpp) is registered with NVCOOP_MODULE and reads nvcoop.ini [markers] and [revive]; see docs/markers.md and docs/revive.md.
#pragma once
#include <cstdint>
#include <functional>
#include <string>

#include "mk_wire.h"   // nvmk::readInPlaceRespawn / inPlaceDamageLine: the in-place respawn order

namespace nvcoop::markers {

bool isGuestOp(const std::string& op);   // "party"
bool isHostOp(const std::string& op);    // "beacon", "revive"
std::string applyGuestOrder(const std::string& json);
std::string handleHostOrder(uint8_t from, const std::string& json);

// This machine's own player was downed by the host's order / got up (revived in place, or respawned at the checkpoint).
void onLocalDowned(uint32_t respawnMs);
void onLocalUp(const std::string& revivedBy);

// The in-place respawn order and its console line live in mk_wire.h (nvmk::encodeInPlaceRespawn, readInPlaceRespawn, inPlaceDamageLine).

// The seam for the downed pose of a friend's puppet (docs/revive.md "Downed pose"): it is called on this machine whenever a friend (not the local player)
// goes down or gets up, with his slot. mk_glue.cpp sets it to the avatar module's pose override when that module is built (NVCOOP_WITH_AVATAR_ENGINE);
// another owner of the puppets may set its own.
using DownedPoseHook = std::function<void(uint8_t slot, bool downed)>;
void setDownedPoseHook(DownedPoseHook hook);

}  // namespace nvcoop::markers
