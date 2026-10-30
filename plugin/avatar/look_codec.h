// The byte forms of a player's look, facts and pack, shared by the avatar wire messages (plugin/coop/live/avatar_wire: Look 0x43, Facts 0x44, Pack 0x45)
// and by the parts of the player entity (docs/entity-core-plan.md 2.2: PartKind Look, Facts, Pack). One codec, so the two carriers cannot drift apart:
// the Look v2 bytes between the slot and the end of the message are exactly the bytes of encodeAppearanceFields + encodeEquipEntries (a golden test in
// plugin/coop/live/tests pins them against the codec as it was before the move). Game-free.
//
// The decoders take (data, size, pos): they read from `pos`, advance it past what they consumed and return false on anything malformed (the caller rejects
// the whole message; nothing is half applied). Twins 32, 33, 67, 101, 102, 104 and 951 of the live wiring and avatar suites live in look_codec.cpp.
#pragma once
#include <cstddef>
#include <cstdint>
#include <vector>

#include "avatar_types.h"

namespace nvcoop::avatar {

constexpr size_t kMaxLookEquip = 64;
constexpr size_t kMaxLookName = 48;
using LookBytes = std::vector<uint8_t>;

// Appearance: [flags][race u32][hair u32][eyes u32][faceHash u32][hairColor u32 if flags&2][height u16, weight u16 if flags&4][n u8, n * i8 if flags&8][n u8, n * u32 if flags&16 (head parts)][nameLen u8][name]
void encodeAppearanceFields(LookBytes& out, const Appearance& a);
bool decodeAppearanceFields(const uint8_t* data, size_t size, size_t& pos, Appearance& out);

// Equipment list of the Look message: [n u8][form u32, flags u8 (1 = weapon, 2 = mods), mods u8 if flags&2] * n
void encodeEquipEntries(LookBytes& out, const std::vector<EquipEntry>& e);
bool decodeEquipEntries(const uint8_t* data, size_t size, size_t& pos, std::vector<EquipEntry>& out);

// The Look part's value: the appearance fields alone, to the end of the buffer.
LookBytes encodeAppearance(const Appearance& a);
bool decodeAppearance(const LookBytes& bytes, Appearance& out);

// Facts: [level u16][sitting u8][actions u32][n u8][region u32 * n]
void encodeFactsFields(LookBytes& out, const PlayerFacts& f);
bool decodeFactsFields(const uint8_t* data, size_t size, size_t& pos, PlayerFacts& out);
LookBytes encodeFacts(const PlayerFacts& f);
bool decodeFacts(const LookBytes& bytes, PlayerFacts& out);

// Pack: [flags u8 (1 = weight)][watchId u32][weight u32 if flags&1][n u16][(form u32, count i32) * n], sorted by form, counts above zero
void encodePackFields(LookBytes& out, const PlayerPack& p);
bool decodePackFields(const uint8_t* data, size_t size, size_t& pos, PlayerPack& out);
LookBytes encodePack(const PlayerPack& p);
bool decodePack(const LookBytes& bytes, PlayerPack& out);

}  // namespace nvcoop::avatar
