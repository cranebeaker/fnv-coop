// The orders plugin/markers puts on the session's order channel (JSON, kAppOrderChannel). Game-free.
//
//   guest -> host   {"op":"beacon", ...}   where this player is (twice a second)
//   host  -> guest  {"op":"party", ...}    the whole table: everyone's place, state and revive progress (twice a second)
//   guest -> host   {"op":"revive","ph":"begin|cancel|done","t":<slot>}   a friend holds the activate key on a downed player
//
// Everything a guest sends is checked here before the host uses it (slot range, finite numbers, text length and characters).
// Mutation twins 1331..1345.
#pragma once
#include <string>
#include <vector>

#include "common/minijson.h"
#include "mk_types.h"

namespace nvmk {

constexpr const char* kOpBeacon = "beacon";
constexpr const char* kOpParty = "party";
constexpr const char* kOpRevive = "revive";

enum class RevivePhase : uint8_t { Begin, Cancel, Done };
const char* phaseName(RevivePhase p);

struct Beacon {
    std::string name;
    Place at;
    Place ext;
};
struct ReviveMsg {
    RevivePhase phase = RevivePhase::Begin;
    Slot target = kNoSlot;
};
struct PartyTable {
    uint32_t seq = 0;
    std::vector<Member> members;
};

bool isHostOp(const std::string& op);    // beacon, revive: what a guest sends
bool isGuestOp(const std::string& op);   // party: what the host sends

std::string encodeBeacon(const Beacon& b);
bool decodeBeacon(const std::string& json, Beacon& out);
std::string encodeRevive(const ReviveMsg& m);
bool decodeRevive(const std::string& json, ReviveMsg& out);
std::string encodeParty(const PartyTable& t);
bool decodeParty(const std::string& json, PartyTable& out);

// The respawn order that gets a downed player up where he lies (a plain {"op":"respawn"} sends him to the checkpoint instead):
//   {"op":"respawn","inPlace":true,"frac":0.3,"by":"<name>"}
std::string encodeInPlaceRespawn(float healthFraction, const std::string& byName);
// Reads the in-place fields of a respawn order the machine received (false = a checkpoint respawn). frac is clamped to 0.05 .. 1.
bool readInPlaceRespawn(const nvcoop::json::Value& order, float& healthFraction, std::string& byName);
// The console line (run on the player) that takes a restored player down to `frac` of his health after ResurrectActor + RestoreAV.
std::string inPlaceDamageLine(float healthFraction);

}  // namespace nvmk
