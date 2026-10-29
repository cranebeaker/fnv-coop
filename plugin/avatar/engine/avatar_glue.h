// What plugin/coop (dll-wiring) calls to put the player avatars on the network. The module (win/avatar_module.cpp) owns the
// AvatarRuntime; it is created on the first frame in which the player exists and only when nvcoop.ini says [avatar] enabled=1.
//
//   at session start     avatar::glue::SetSnapshotSink([](const live::ActorSnapshot& s){ send unreliable-sequenced });
//                        avatar::glue::SetEquipSink([](const std::vector<EquipEntry>& e){ send reliable });
//                        avatar::glue::SetLocalIdentity(netIdOfMyAvatar, ownerEpoch);
//   a player joins       avatar::glue::AddRemote(slot, appearance);          // leaves: RemoveRemote(slot)
//   a snapshot arrives   avatar::glue::OnRemoteSnapshot(slot, snap, arrivalMs);
//   equipment event      avatar::glue::OnRemoteEquipment(slot, entries);
//   ghost-ref map        avatar::glue::PuppetRef(slot) / avatar::glue::PuppetSlot(ref)   (replaces nvmp::GhostRefOfPlayer)
//
// Every call is a no-op (returns false / 0) while the avatar layer is disabled or not created yet, so the co-op layer needs no
// #ifdef beyond including this header; calls made BEFORE the runtime exists are remembered (sinks, identity, remotes) and replayed.
#pragma once
#include <cstdint>
#include <functional>
#include <optional>
#include <vector>

#include "../avatar_types.h"
#include "../live_bridge.h"

namespace nvcoop::avatar::glue {

bool Enabled();       // nvcoop.ini [avatar] enabled=1 and the module loaded
bool Running();       // the runtime exists (the first frame with a player has passed)
// Why Running() is false, for the status line: "" when it runs, else what it waits for. The runtime needs a player in a loaded cell, so a copy
// that sits at the main menu (nothing loaded yet) is enabled but not running; that is not a wiring fault.
const char* NotRunningBecause();

void SetSnapshotSink(std::function<void(const live::ActorSnapshot&)> sink);
void SetEquipSink(std::function<void(const std::vector<EquipEntry>&)> sink);
void SetLocalIdentity(live::NetId netId, live::OwnerEpoch epoch);
// This machine's slot in the session (a client is 1, 2, ...). Overrides [avatar] localSlot (default 0, right for the host only): without it a
// client refuses the HOST's puppet as "the local slot". Call at the session start, before AddRemote.
void SetLocalSlot(Slot slot);

bool AddRemote(Slot slot, const Appearance& a);
void RemoveRemote(Slot slot);
// A changed look of a known remote player: updates the live puppet (a new puppet only when race or sex pick another template).
bool SetRemoteAppearance(Slot slot, const Appearance& a);
// The local player's own look (race, sex, hair, eyes, body, face data), read from the game. False while unreadable.
bool ReadLocalAppearance(Appearance& out);
// The local player's level / sit state / actions / regions (W3a item 3) and its weight and watched item counts (guest pack facts). False: not readable now.
bool ReadLocalFacts(PlayerFacts& out);
bool ReadLocalPack(const std::vector<FormId>& watch, PlayerPack& out);
void OnRemoteSnapshot(Slot slot, const live::ActorSnapshot& s, uint32_t arrivalMs);
// A timing figure of the network layer for avatar.status "stages" (see NetStage in avatar_types.h). No-op while the runtime does not exist.
void NoteStage(NetStage st, double ms);
void OnRemoteEquipment(Slot slot, const std::vector<EquipEntry>& wanted);

// Around a save the game itself writes (NVSE's SaveGame message arrives AFTER the .fos is written, so the module cannot see it coming):
// call PreSave() right before a save you start yourself (bridge saveGame, a quicksave hook), PostSave() once it finished. The puppets are
// deleted and stay away in between. A save that nobody announced is still safe: puppets are purged on the next load.
void PreSave();
void PostSave();

// The ghost-ref map lives in avatar_api.h (puppetRefOf / slotOfPuppet / actingRefOf); these two answer from the same live puppets.
RefId PuppetRef(Slot slot);                  // 0 = no usable puppet right now
std::optional<Slot> PuppetSlot(RefId ref);

// The downed pose of a friend's puppet (docs/revive.md "Downed pose"; plugin/markers sets it from its downed hook). While it holds, the puppet of that
// slot plays the lying group instead of its planned one, is not replaced when the engine calls it dead, and stands again at once when it is cleared.
// Remembered for a runtime or a puppet that does not exist yet; RemoveRemote forgets it.
void SetDownedPose(Slot slot, bool downed);
void SetPoseOverride(Slot slot, int baseGroup);   // any base group; kNoGroup clears

}  // namespace nvcoop::avatar::glue
