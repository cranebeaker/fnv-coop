#include "look_codec.h"

#include <string>

#include "fnvcoop_mut.h"   // FNV_MUT(n): twins 32, 33, 67, 101, 102, 104, 951 (moved here from avatar_wire.cpp with the code they sit in)

namespace nvcoop::avatar {

namespace {
void put8(LookBytes& b, uint8_t v) { b.push_back(v); }
void put16(LookBytes& b, uint16_t v) { b.push_back(static_cast<uint8_t>(v)); b.push_back(static_cast<uint8_t>(v >> 8)); }
void put32(LookBytes& b, uint32_t v) { for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(v >> (8 * i))); }

struct Rd {
    const uint8_t* p;
    size_t n, i;
    bool ok = true;
    uint8_t u8() { if (i + 1 > n) { ok = false; return 0; } return p[i++]; }
    uint16_t u16() { if (i + 2 > n) { ok = false; return 0; } uint16_t v = uint16_t(p[i] | (p[i + 1] << 8)); i += 2; return v; }
    uint32_t u32() { if (i + 4 > n) { ok = false; return 0; } uint32_t v = 0; for (int k = 0; k < 4; ++k) v |= uint32_t(p[i + k]) << (8 * k); i += 4; return v; }
    std::string str(size_t len) { if (i + len > n) { ok = false; return {}; } std::string s(reinterpret_cast<const char*>(p + i), len); i += len; return s; }
};

// Look v2 flags byte
constexpr uint8_t kLkFemale = 0x01, kLkHairColor = 0x02, kLkBody = 0x04, kLkFace = 0x08, kLkReserved = 0xF0;
// per equipment entry flags byte
constexpr uint8_t kEqWeapon = 0x01, kEqMods = 0x02, kEqReserved = 0xFC;
// Pack flags
constexpr uint8_t kPkWeight = 0x01, kPkReserved = 0xFE;
}  // namespace

// ---- appearance ----------------------------------------------------------------------------------------------------------------------
void encodeAppearanceFields(LookBytes& b, const Appearance& a) {
    const size_t nFace = a.faceCoeffs.size() > kMaxFaceCoeffs ? kMaxFaceCoeffs : a.faceCoeffs.size();
    uint8_t flags = a.female ? kLkFemale : 0;
    if (a.hairColor) flags |= kLkHairColor;
    if (a.height || a.weight) flags |= kLkBody;
    if (nFace) flags |= kLkFace;
    put8(b, flags);
    put32(b, a.race);
    put32(b, a.hair);
    put32(b, a.eyes);
    put32(b, a.faceHash);
    if (flags & kLkHairColor) put32(b, a.hairColor);
    if (flags & kLkBody) {
        put16(b, FNV_MUT(1178) ? a.weight : a.height);   // MUTANT 1178: height and weight swapped on the wire (the golden bytes of Look v2 differ)
        put16(b, FNV_MUT(1178) ? a.height : a.weight);
    }
    if (flags & kLkFace) {
        put8(b, static_cast<uint8_t>(nFace));
        for (size_t i = 0; i < nFace; ++i) put8(b, static_cast<uint8_t>(a.faceCoeffs[i]));
    }
    std::string name = a.name.substr(0, kMaxLookName);
    put8(b, static_cast<uint8_t>(name.size()));
    b.insert(b.end(), name.begin(), name.end());
}

bool decodeAppearanceFields(const uint8_t* data, size_t size, size_t& pos, Appearance& out) {
    Rd r{data, size, pos};
    Appearance a;
    const uint8_t flags = r.u8();
    if ((flags & kLkReserved) && !FNV_MUT(102)) r.ok = false;   // twin 102: reserved flag bits are ignored (a future field would be misread)
    a.female = (flags & kLkFemale) != 0;
    a.race = r.u32();
    a.hair = r.u32();
    a.eyes = r.u32();
    a.faceHash = r.u32();
    if (flags & kLkHairColor) a.hairColor = r.u32();
    if (flags & kLkBody) {
        a.height = r.u16();
        a.weight = r.u16();
    }
    if (flags & kLkFace) {
        const uint8_t nf = r.u8();
        if ((nf == 0 || nf > kMaxFaceCoeffs) && !FNV_MUT(101)) r.ok = false;   // twin 101: an oversized face block is accepted
        for (uint8_t i = 0; i < nf && r.ok; ++i) a.faceCoeffs.push_back(static_cast<int8_t>(r.u8()));
    }
    const uint8_t nl = r.u8();
    if (nl > kMaxLookName) r.ok = false;
    a.name = r.str(r.ok ? nl : 0);
    if (!r.ok) return false;
    out = std::move(a);
    pos = r.i;
    return true;
}

void encodeEquipEntries(LookBytes& b, const std::vector<EquipEntry>& equipment) {
    const size_t n = equipment.size() > kMaxLookEquip ? kMaxLookEquip : equipment.size();
    put8(b, static_cast<uint8_t>(n));   // the count is the LAST byte of a look without equipment (the wire tests rely on it)
    for (size_t i = 0; i < n; ++i) {
        const EquipEntry& e = equipment[i];
        const uint8_t mods = static_cast<uint8_t>(e.mods & kWeaponModMask);
        put32(b, e.form);
        put8(b, static_cast<uint8_t>((e.weapon ? kEqWeapon : 0) | (mods ? kEqMods : 0)));
        if (mods) put8(b, mods);
    }
}

bool decodeEquipEntries(const uint8_t* data, size_t size, size_t& pos, std::vector<EquipEntry>& out) {
    Rd r{data, size, pos};
    std::vector<EquipEntry> list;
    const uint8_t ne = r.u8();
    if (ne > kMaxLookEquip && !FNV_MUT(32)) r.ok = false;   // MUTANT 32: an oversized equipment list is accepted up to the datagram size
    for (uint8_t i = 0; i < ne && r.ok; ++i) {
        EquipEntry e;
        e.form = r.u32();
        const uint8_t ef = r.u8();
        if (ef & kEqReserved) r.ok = false;
        e.weapon = (ef & kEqWeapon) != 0;
        if (ef & kEqMods) {
            e.mods = r.u8();
            if ((e.mods == 0 || (e.mods & ~kWeaponModMask) || !e.weapon) && !FNV_MUT(104)) r.ok = false;   // twin 104: junk mod flags are accepted
        }
        list.push_back(e);
    }
    if (!r.ok) return false;
    out = std::move(list);
    pos = r.i;
    return true;
}

LookBytes encodeAppearance(const Appearance& a) {
    LookBytes b;
    encodeAppearanceFields(b, a);
    return b;
}

bool decodeAppearance(const LookBytes& bytes, Appearance& out) {
    size_t pos = 0;
    Appearance a;
    if (!decodeAppearanceFields(bytes.data(), bytes.size(), pos, a) || pos != bytes.size()) return false;
    out = std::move(a);
    return true;
}

// ---- facts ---------------------------------------------------------------------------------------------------------------------------
void encodeFactsFields(LookBytes& b, const PlayerFacts& f) {
    put16(b, f.level);
    put8(b, f.sitting);
    put32(b, f.actions);
    const size_t n = f.regions.size() > kMaxFactRegions ? kMaxFactRegions : f.regions.size();
    put8(b, static_cast<uint8_t>(n));
    for (size_t i = 0; i < n; ++i) put32(b, f.regions[i]);
}

bool decodeFactsFields(const uint8_t* data, size_t size, size_t& pos, PlayerFacts& out) {
    Rd r{data, size, pos};
    PlayerFacts f;
    f.level = r.u16();
    f.sitting = r.u8();
    f.actions = r.u32();
    const uint8_t n = r.u8();
    if (n > kMaxFactRegions && !FNV_MUT(67)) r.ok = false;   // twin 67: an oversized region list is accepted
    for (uint8_t i = 0; i < n && r.ok; ++i) f.regions.push_back(r.u32());
    if (!r.ok) return false;
    out = std::move(f);
    pos = r.i;
    return true;
}

LookBytes encodeFacts(const PlayerFacts& f) {
    LookBytes b;
    encodeFactsFields(b, f);
    return b;
}

bool decodeFacts(const LookBytes& bytes, PlayerFacts& out) {
    size_t pos = 0;
    PlayerFacts f;
    if (!decodeFactsFields(bytes.data(), bytes.size(), pos, f) || pos != bytes.size()) return false;
    out = std::move(f);
    return true;
}

// ---- pack ----------------------------------------------------------------------------------------------------------------------------
void encodePackFields(LookBytes& b, const PlayerPack& p) {
    put8(b, p.haveWeight ? kPkWeight : 0);
    put32(b, p.watchId);
    if (p.haveWeight) put32(b, p.weightMilli);
    const size_t n = p.items.size() > kMaxPackItems ? kMaxPackItems : p.items.size();
    put16(b, static_cast<uint16_t>(n));
    for (size_t i = 0; i < n; ++i) {
        put32(b, p.items[i].form);
        put32(b, static_cast<uint32_t>(p.items[i].count));
    }
}

bool decodePackFields(const uint8_t* data, size_t size, size_t& pos, PlayerPack& out) {
    Rd r{data, size, pos};
    PlayerPack p;
    const uint8_t flags = r.u8();
    if (flags & kPkReserved) r.ok = false;
    p.watchId = r.u32();
    p.haveWeight = (flags & kPkWeight) != 0;
    if (p.haveWeight) p.weightMilli = r.u32();
    const uint16_t n = r.u16();
    if (n > kMaxPackItems && !FNV_MUT(951)) r.ok = false;   // twin 951: an oversized item list is accepted
    uint32_t prev = 0;
    for (uint16_t i = 0; i < n && r.ok; ++i) {
        PackEntry e;
        e.form = r.u32();
        e.count = static_cast<int32_t>(r.u32());
        if (e.count <= 0 || (i > 0 && e.form <= prev)) r.ok = false;   // sorted, de-duplicated, only counts above zero
        prev = e.form;
        p.items.push_back(e);
    }
    if (!r.ok) return false;
    out = std::move(p);
    pos = r.i;
    return true;
}

LookBytes encodePack(const PlayerPack& p) {
    LookBytes b;
    encodePackFields(b, p);
    return b;
}

bool decodePack(const LookBytes& bytes, PlayerPack& out) {
    size_t pos = 0;
    PlayerPack p;
    if (!decodePackFields(bytes.data(), bytes.size(), pos, p) || pos != bytes.size()) return false;
    out = std::move(p);
    return true;
}

}  // namespace nvcoop::avatar
