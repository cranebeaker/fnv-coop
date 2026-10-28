#include "avatar_wire.h"

#include "fnvcoop_mut.h"   // FNV_MUT(n): twins 31..33 of the live wiring suite, 101..104 of the avatar suite (plugin/avatar/full)

namespace nvcoop::livewire {

namespace {
void put8(AvBytes& b, uint8_t v) { b.push_back(v); }
void put32(AvBytes& b, uint32_t v) { for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(v >> (8 * i))); }
void put16(AvBytes& b, uint16_t v) { b.push_back(static_cast<uint8_t>(v)); b.push_back(static_cast<uint8_t>(v >> 8)); }
struct Rd {
    const uint8_t* p;
    size_t n, i = 0;
    bool ok = true;
    uint8_t u8() { if (i + 1 > n) { ok = false; return 0; } return p[i++]; }
    uint16_t u16() { if (i + 2 > n) { ok = false; return 0; } uint16_t v = uint16_t(p[i] | (p[i + 1] << 8)); i += 2; return v; }
    uint32_t u32() { if (i + 4 > n) { ok = false; return 0; } uint32_t v = 0; for (int k = 0; k < 4; ++k) v |= uint32_t(p[i + k]) << (8 * k); i += 4; return v; }
    std::string str(size_t len) { if (i + len > n) { ok = false; return {}; } std::string s(reinterpret_cast<const char*>(p + i), len); i += len; return s; }
    bool done() const { return ok && i == n; }
};
}  // namespace

bool operator==(const AvLook& a, const AvLook& b) {
    if (a.slot != b.slot || a.appearance.race != b.appearance.race || a.appearance.female != b.appearance.female || a.appearance.hair != b.appearance.hair ||
        a.appearance.eyes != b.appearance.eyes || a.appearance.faceHash != b.appearance.faceHash || a.appearance.name != b.appearance.name ||
        a.appearance.hairColor != b.appearance.hairColor || a.appearance.height != b.appearance.height || a.appearance.weight != b.appearance.weight ||
        a.appearance.faceCoeffs != b.appearance.faceCoeffs || a.equipment.size() != b.equipment.size())
        return false;
    for (size_t i = 0; i < a.equipment.size(); ++i)
        if (a.equipment[i].form != b.equipment[i].form || a.equipment[i].weapon != b.equipment[i].weapon ||
            (!FNV_MUT(103) && a.equipment[i].mods != b.equipment[i].mods))   // twin 103: a changed weapon modification is not a changed look
            return false;
    return true;
}

AvBytes encodeRoster(const std::vector<uint8_t>& slots) {
    AvBytes b;
    put8(b, kChannelAvatars);
    put8(b, kAvKindRoster);
    put8(b, static_cast<uint8_t>(slots.size() > 16 ? 16 : slots.size()));
    for (size_t i = 0; i < slots.size() && i < 16; ++i) put8(b, slots[i]);
    return b;
}

bool decodeRoster(const uint8_t* data, size_t size, std::vector<uint8_t>& slots) {
    Rd r{data, size};
    if (r.u8() != kChannelAvatars || r.u8() != kAvKindRoster) return false;
    uint8_t n = r.u8();
    if (n > 16) return false;
    std::vector<uint8_t> out;
    for (uint8_t i = 0; i < n; ++i) {
        uint8_t s = r.u8();
        if (s >= 8) r.ok = false;   // fnvcoop::kMaxSlots
        out.push_back(s);
    }
    if (FNV_MUT(31) ? !r.ok : !r.done()) return false;   // MUTANT 31: trailing garbage after the roster is accepted
    slots = std::move(out);
    return true;
}

AvBytes encodeFacts(const AvFacts& f) {
    AvBytes b;
    put8(b, kChannelAvatars);
    put8(b, kAvKindFacts);
    put8(b, kAvFactsVersion);
    put8(b, f.slot);
    put16(b, f.facts.level);
    put8(b, f.facts.sitting);
    put32(b, f.facts.actions);
    const size_t n = f.facts.regions.size() > nvcoop::avatar::kMaxFactRegions ? nvcoop::avatar::kMaxFactRegions : f.facts.regions.size();
    put8(b, static_cast<uint8_t>(n));
    for (size_t i = 0; i < n; ++i) put32(b, f.facts.regions[i]);
    return b;
}

bool decodeFacts(const uint8_t* data, size_t size, AvFacts& out) {
    Rd r{data, size};
    if (r.u8() != kChannelAvatars || r.u8() != kAvKindFacts || r.u8() != kAvFactsVersion) return false;
    AvFacts f;
    f.slot = r.u8();
    if (f.slot >= 8) r.ok = false;   // fnvcoop::kMaxSlots
    f.facts.level = r.u16();
    f.facts.sitting = r.u8();
    f.facts.actions = r.u32();
    const uint8_t n = r.u8();
    if (n > nvcoop::avatar::kMaxFactRegions && !FNV_MUT(67)) r.ok = false;   // twin 67: an oversized region list is accepted
    for (uint8_t i = 0; i < n && r.ok; ++i) f.facts.regions.push_back(r.u32());
    if (!r.done()) return false;
    out = std::move(f);
    return true;
}

// Pack: [0x45][ver][slot][flags][watchId][weight?][n u16][(form, count) * n]. Entries are sorted by form with a count above zero; anything else is rejected as a whole.
constexpr uint8_t kPkWeight = 0x01, kPkReserved = 0xFE;

AvBytes encodePack(const AvPack& p) {
    AvBytes b;
    put8(b, kChannelAvatars);
    put8(b, kAvKindPack);
    put8(b, kAvPackVersion);
    put8(b, p.slot);
    put8(b, p.pack.haveWeight ? kPkWeight : 0);
    put32(b, p.pack.watchId);
    if (p.pack.haveWeight) put32(b, p.pack.weightMilli);
    const size_t n = p.pack.items.size() > nvcoop::avatar::kMaxPackItems ? nvcoop::avatar::kMaxPackItems : p.pack.items.size();
    put16(b, static_cast<uint16_t>(n));
    for (size_t i = 0; i < n; ++i) {
        put32(b, p.pack.items[i].form);
        put32(b, static_cast<uint32_t>(p.pack.items[i].count));
    }
    return b;
}

bool decodePack(const uint8_t* data, size_t size, AvPack& out) {
    Rd r{data, size};
    if (r.u8() != kChannelAvatars || r.u8() != kAvKindPack || r.u8() != kAvPackVersion) return false;
    AvPack p;
    p.slot = r.u8();
    if (p.slot >= 8) r.ok = false;   // fnvcoop::kMaxSlots
    const uint8_t flags = r.u8();
    if (flags & kPkReserved) r.ok = false;
    p.pack.watchId = r.u32();
    p.pack.haveWeight = (flags & kPkWeight) != 0;
    if (p.pack.haveWeight) p.pack.weightMilli = r.u32();
    const uint16_t n = r.u16();
    if (n > nvcoop::avatar::kMaxPackItems && !FNV_MUT(951)) r.ok = false;   // twin 951: an oversized item list is accepted
    uint32_t prev = 0;
    for (uint16_t i = 0; i < n && r.ok; ++i) {
        nvcoop::avatar::PackEntry e;
        e.form = r.u32();
        e.count = static_cast<int32_t>(r.u32());
        if (e.count <= 0 || (i > 0 && e.form <= prev)) r.ok = false;   // sorted, de-duplicated, only counts above zero
        prev = e.form;
        p.pack.items.push_back(e);
    }
    if (!r.done()) return false;
    out = std::move(p);
    return true;
}

// Look v2 flags byte
constexpr uint8_t kLkFemale = 0x01, kLkHairColor = 0x02, kLkBody = 0x04, kLkFace = 0x08, kLkReserved = 0xF0;
// per equipment entry flags byte
constexpr uint8_t kEqWeapon = 0x01, kEqMods = 0x02, kEqReserved = 0xFC;

AvBytes encodeLook(const AvLook& l) {
    AvBytes b;
    put8(b, kChannelAvatars);
    put8(b, kAvKindLook2);
    put8(b, kAvatarWireVersion);
    put8(b, l.slot);
    const nvcoop::avatar::Appearance& a = l.appearance;
    const size_t nFace = a.faceCoeffs.size() > nvcoop::avatar::kMaxFaceCoeffs ? nvcoop::avatar::kMaxFaceCoeffs : a.faceCoeffs.size();
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
        put16(b, a.height);
        put16(b, a.weight);
    }
    if (flags & kLkFace) {
        put8(b, static_cast<uint8_t>(nFace));
        for (size_t i = 0; i < nFace; ++i) put8(b, static_cast<uint8_t>(a.faceCoeffs[i]));
    }
    std::string name = a.name.substr(0, kMaxLookName);
    put8(b, static_cast<uint8_t>(name.size()));
    b.insert(b.end(), name.begin(), name.end());
    size_t n = l.equipment.size() > kMaxLookEquip ? kMaxLookEquip : l.equipment.size();
    put8(b, static_cast<uint8_t>(n));   // the count is the LAST byte of a look without equipment (the wire tests rely on it)
    for (size_t i = 0; i < n; ++i) {
        const nvcoop::avatar::EquipEntry& e = l.equipment[i];
        const uint8_t mods = static_cast<uint8_t>(e.mods & nvcoop::avatar::kWeaponModMask);
        put32(b, e.form);
        put8(b, static_cast<uint8_t>((e.weapon ? kEqWeapon : 0) | (mods ? kEqMods : 0)));
        if (mods) put8(b, mods);
    }
    return b;
}

namespace {
// v1: [slot][race][female][hair][eyes][face][name][equip form + weapon]
bool decodeLookV1(Rd& r, AvLook& out) {
    AvLook l;
    l.slot = r.u8();
    if (l.slot >= 8 && !FNV_MUT(33)) r.ok = false;   // MUTANT 33: a slot outside the table is accepted
    l.appearance.race = r.u32();
    l.appearance.female = r.u8() != 0;
    l.appearance.hair = r.u32();
    l.appearance.eyes = r.u32();
    l.appearance.faceHash = r.u32();
    uint8_t nl = r.u8();
    if (nl > kMaxLookName) r.ok = false;
    l.appearance.name = r.str(r.ok ? nl : 0);
    uint8_t ne = r.u8();
    if (ne > kMaxLookEquip && !FNV_MUT(32)) r.ok = false;   // MUTANT 32: an oversized equipment list is accepted up to the datagram size
    for (uint8_t i = 0; i < ne && r.ok; ++i) {
        nvcoop::avatar::EquipEntry e;
        e.form = r.u32();
        e.weapon = r.u8() != 0;
        l.equipment.push_back(e);
    }
    if (!r.done()) return false;
    out = std::move(l);
    return true;
}

bool decodeLookV2(Rd& r, AvLook& out) {
    if (r.u8() != kAvatarWireVersion) return false;
    AvLook l;
    l.slot = r.u8();
    if (l.slot >= 8 && !FNV_MUT(33)) r.ok = false;
    const uint8_t flags = r.u8();
    if ((flags & kLkReserved) && !FNV_MUT(102)) r.ok = false;   // twin 102: reserved flag bits are ignored (a future field would be misread)
    nvcoop::avatar::Appearance& a = l.appearance;
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
        if ((nf == 0 || nf > nvcoop::avatar::kMaxFaceCoeffs) && !FNV_MUT(101)) r.ok = false;   // twin 101: an oversized face block is accepted
        for (uint8_t i = 0; i < nf && r.ok; ++i) a.faceCoeffs.push_back(static_cast<int8_t>(r.u8()));
    }
    uint8_t nl = r.u8();
    if (nl > kMaxLookName) r.ok = false;
    a.name = r.str(r.ok ? nl : 0);
    uint8_t ne = r.u8();
    if (ne > kMaxLookEquip && !FNV_MUT(32)) r.ok = false;
    for (uint8_t i = 0; i < ne && r.ok; ++i) {
        nvcoop::avatar::EquipEntry e;
        e.form = r.u32();
        const uint8_t ef = r.u8();
        if (ef & kEqReserved) r.ok = false;
        e.weapon = (ef & kEqWeapon) != 0;
        if (ef & kEqMods) {
            e.mods = r.u8();
            if ((e.mods == 0 || (e.mods & ~nvcoop::avatar::kWeaponModMask) || !e.weapon) && !FNV_MUT(104)) r.ok = false;   // twin 104: junk mod flags are accepted
        }
        l.equipment.push_back(e);
    }
    if (!r.done()) return false;
    out = std::move(l);
    return true;
}
}  // namespace

bool decodeLook(const uint8_t* data, size_t size, AvLook& out) {
    Rd r{data, size};
    if (r.u8() != kChannelAvatars) return false;
    const uint8_t kind = r.u8();
    if (kind == kAvKindLook2) return decodeLookV2(r, out);
    if (kind == kAvKindLook) return decodeLookV1(r, out);
    return false;
}

}  // namespace nvcoop::livewire
