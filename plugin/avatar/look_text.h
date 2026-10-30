// One-line descriptions of a player's look for nvcoop.log, and the comparison of two looks. Header only (the avatar library, the live wiring and the full chain all use it).
//
// describeLook: race, sex, hair, eyes, hair colour, height, weight, head parts and a hash of each of the three FaceGen blocks (FGGS geometry, FGGA asymmetry, FGTS skin tone), so two
// lines can be compared at a glance. The blocks are split the way the engine lays them out (50 + 30 + 50); a face vector of another length is hashed whole.
// describeLookChange: which parts differ ("race, skin tone, hair"); empty when nothing the puppet is built from differs. lookDiffers: the same as a yes / no (the name is not part of it).
#pragma once
#include <cstdint>
#include <cstdio>
#include <string>

#include "avatar_types.h"

namespace nvcoop::avatar {

namespace detail {
inline uint32_t fnvBytes(const int8_t* p, size_t n) {
    uint32_t h = 2166136261u;
    for (size_t i = 0; i < n; ++i) { h ^= static_cast<uint8_t>(p[i]); h *= 16777619u; }
    return h;
}
inline std::string hex8(uint32_t v) {
    char b[16];
    std::snprintf(b, sizeof b, "%08X", v);
    return b;
}
}  // namespace detail

inline std::string describeLook(const Appearance& a) {
    std::string s = "race " + std::to_string(a.race) + (a.female ? " female" : " male") + ", hair " + std::to_string(a.hair) + ", eyes " + std::to_string(a.eyes) +
                    ", hair colour " + detail::hex8(a.hairColor) + ", height " + std::to_string(a.height) + ", weight " + std::to_string(a.weight) + ", face data " +
                    std::to_string(a.faceCoeffs.size());
    if (a.faceCoeffs.size() == 130) {   // FGGS 50 + FGGA 30 + FGTS 50
        const int8_t* c = reinterpret_cast<const int8_t*>(a.faceCoeffs.data());
        s += " (shape " + detail::hex8(detail::fnvBytes(c, 50)) + ", asymmetry " + detail::hex8(detail::fnvBytes(c + 50, 30)) + ", skin tone " + detail::hex8(detail::fnvBytes(c + 80, 50)) + ")";
    } else if (!a.faceCoeffs.empty()) {
        s += " (hash " + detail::hex8(detail::fnvBytes(reinterpret_cast<const int8_t*>(a.faceCoeffs.data()), a.faceCoeffs.size())) + ")";
    }
    s += ", head parts " + std::to_string(a.headParts.size());
    if (!a.headParts.empty()) {
        s += " [";
        for (size_t i = 0; i < a.headParts.size(); ++i) s += (i ? " " : "") + detail::hex8(a.headParts[i]);
        s += "]";
    }
    return s;
}

inline std::string describeLookChange(const Appearance& a, const Appearance& b) {
    std::string out;
    auto add = [&](const char* w) { out += out.empty() ? "" : ", "; out += w; };
    if (a.race != b.race) add("race");
    if (a.female != b.female) add("sex");
    if (a.hair != b.hair) add("hair");
    if (a.eyes != b.eyes) add("eyes");
    if (a.hairColor != b.hairColor) add("hair colour");
    if (a.height != b.height) add("height");
    if (a.weight != b.weight) add("weight");
    if (a.faceCoeffs != b.faceCoeffs) {
        if (a.faceCoeffs.size() == 130 && b.faceCoeffs.size() == 130) {
            const int8_t* x = reinterpret_cast<const int8_t*>(a.faceCoeffs.data());
            const int8_t* y = reinterpret_cast<const int8_t*>(b.faceCoeffs.data());
            if (detail::fnvBytes(x, 50) != detail::fnvBytes(y, 50)) add("face shape");
            if (detail::fnvBytes(x + 50, 30) != detail::fnvBytes(y + 50, 30)) add("face asymmetry");
            if (detail::fnvBytes(x + 80, 50) != detail::fnvBytes(y + 80, 50)) add("skin tone");
        } else {
            add("face");
        }
    }
    if (a.headParts != b.headParts) add("head parts");
    return out;
}

inline bool lookDiffers(const Appearance& a, const Appearance& b) {
    return a.race != b.race || a.female != b.female || a.hair != b.hair || a.eyes != b.eyes || a.hairColor != b.hairColor || a.height != b.height ||
           a.weight != b.weight || a.faceCoeffs != b.faceCoeffs || a.headParts != b.headParts;
}

}  // namespace nvcoop::avatar
