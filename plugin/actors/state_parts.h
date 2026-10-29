// State parts: the pieces of an actor's state that are not motion and that the owner's engine changes by itself (its AI drinks a Stimpak,
// draws the rifle from its pack). The owner reads each part from its engine and every other machine writes the owner's value into its own
// copy. ActorSync runs one generic replicator for all of them (read, settle, publish reliably, de-duplicate, judge the sender, retry a
// refused write, catch a joining client up, flush on handoff and death); a part itself only says how its value is read, written and
// compared, through IActorPort::readPart / writePart and the canonical byte form below. The kinds and timings are in actor_types.h.
//
// Why read-and-compare and not engine events: the engine changes these in many places (native AI, scripts, the player's menus, a load),
// and the events it reports miss some of them (xNVSE OnActorEquip does not fire for a melee weapon or a weapon without ammo). Reading the
// engine's own state is the truth; an event can only make the read come sooner.
#pragma once
#include "actors/actor_types.h"

namespace nvcoop::actors {

const char* partName(PartKind k);

// Canonical byte forms: sorted, so equal state is equal bytes. Decoding fails on anything malformed.
Bytes encodeInventory(InvList items);
bool decodeInventory(const Bytes& b, InvList& out);
Bytes encodeEquipment(EquipList forms);
bool decodeEquipment(const Bytes& b, EquipList& out);

}  // namespace nvcoop::actors
