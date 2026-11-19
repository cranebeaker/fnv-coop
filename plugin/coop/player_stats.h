// A guest's SPECIAL and skills for the host. The host runs some of a guest's actions itself (scripted activators, conversation lines) with its
// own player standing in, and their scripts read `player.GetAV Explosives` and the like: the host's skill decided whether the GUEST could disarm
// Megaton's bomb. Each guest sends its current values every few seconds on mux channel kChannel; while the host runs something for that guest
// it shifts its own player's values to the guest's (ModAV) and back (coop_module.cpp StatSwap). Game-free: the wire.
//   [0xD8][ver=1][n u8] n x ([actor value u8][value f32 LE])
#pragma once
#include <cstdint>
#include <cstring>
#include <map>
#include <vector>

namespace nvcoop { namespace coop { namespace pstats {

constexpr uint8_t kChannel = 0xD8;   // 0xD5 subtitles, 0xD6 shared XP, 0xD7 containers
constexpr uint8_t kVersion = 1;

using Values = std::map<uint8_t, float>;   // actor value code -> current value

// The ones scripts check: SPECIAL (5..11) and the skills (32 Barter .. 45 Unarmed).
inline std::vector<uint8_t> trackedValues() {
    std::vector<uint8_t> v;
    for (uint8_t a = 5; a <= 11; ++a) v.push_back(a);
    for (uint8_t a = 32; a <= 45; ++a) v.push_back(a);
    return v;
}

// Console name of a tracked actor value (ModAV takes names), nullptr for others.
inline const char* avName(uint8_t av) {
    static const char* special[] = {"Strength", "Perception", "Endurance", "Charisma", "Intelligence", "Agility", "Luck"};
    static const char* skills[] = {"Barter", "BigGuns", "EnergyWeapons", "Explosives", "Lockpick", "Medicine", "MeleeWeapons",
                                   "Repair", "Science", "Guns", "Sneak", "Speech", "Survival", "Unarmed"};
    if (av >= 5 && av <= 11) return special[av - 5];
    if (av >= 32 && av <= 45) return skills[av - 32];
    return nullptr;
}

inline std::vector<uint8_t> encode(const Values& v) {
    std::vector<uint8_t> b{kChannel, kVersion, static_cast<uint8_t>(v.size() < 255 ? v.size() : 255)};
    size_t k = 0;
    for (const auto& kv : v) {
        if (k++ == 255) break;
        b.push_back(kv.first);
        uint32_t bits;
        std::memcpy(&bits, &kv.second, 4);
        for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(bits >> (8 * i)));
    }
    return b;
}

inline bool decode(const uint8_t* p, size_t n, Values* out) {
    if (n < 3 || p[0] != kChannel || p[1] != kVersion) return false;
    const size_t count = p[2];
    if (n != 3 + count * 5) return false;
    Values v;
    for (size_t i = 0; i < count; ++i) {
        const uint8_t* e = p + 3 + i * 5;
        if (!avName(e[0])) return false;
        const uint32_t bits = uint32_t(e[1]) | uint32_t(e[2]) << 8 | uint32_t(e[3]) << 16 | uint32_t(e[4]) << 24;
        float f;
        std::memcpy(&f, &bits, 4);
        if (!(f >= -1000.0f && f <= 1000.0f)) return false;   // also refuses NaN
        v[e[0]] = f;
    }
    *out = std::move(v);
    return true;
}

}}}  // namespace nvcoop::coop::pstats
