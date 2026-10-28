// What the host knows about ONE remote player for the party conditions (W3a item 3). Plain data, no engine, no network: the avatar service
// (plugin/coop/live/avatar_net.h) fills it from the player's Look (sex, worn items), its newest snapshot (sneaking) and its Facts message
// (level, sit state, active actions, regions); plugin/coop/party_state.h turns it into the StateProvider answers. Each group is only valid when its
// have* flag is set: "unknown" makes the condition hooks skip that member.
#pragma once
#include <cstdint>
#include <vector>

namespace nvcoop {

struct SlotFacts {
    bool haveSex = false;
    int sex = 0;                         // 0 male / 1 female
    bool haveEquip = false;
    std::vector<uint32_t> equipped;      // worn items and the held weapon (runtime form ids; every machine runs the same load order)
    bool haveSneak = false;
    bool sneaking = false;
    bool haveFacts = false;              // level, sitting, actions and regions below
    uint16_t level = 0;                  // 0 = unknown
    uint8_t sitting = 0;
    uint32_t actions = 0;                // bit n = IsPlayerActionActive n (ids 0..31)
    std::vector<uint32_t> regions;
};

}  // namespace nvcoop
