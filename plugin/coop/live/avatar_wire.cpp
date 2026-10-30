#include "avatar_wire.h"

#include "look_codec.h"    // the field codecs of Look, Facts and Pack (shared with the parts of the player entity)
#include "fnvcoop_mut.h"   // FNV_MUT(n): twins 31, 33 of the live wiring suite, 103 of the avatar suite (plugin/avatar/full); 32, 67, 101, 102, 104, 951 sit in look_codec.cpp

namespace nvcoop::livewire {

namespace {
void put8(AvBytes& b, uint8_t v) { b.push_back(v); }
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
        a.appearance.faceCoeffs != b.appearance.faceCoeffs || a.appearance.headParts != b.appearance.headParts || a.equipment.size() != b.equipment.size())
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

// Facts: [ch][0x44][ver][slot] then the Facts fields (look_codec.h)
AvBytes encodeFacts(const AvFacts& f) {
    AvBytes b;
    put8(b, kChannelAvatars);
    put8(b, kAvKindFacts);
    put8(b, kAvFactsVersion);
    put8(b, f.slot);
    nvcoop::avatar::encodeFactsFields(b, f.facts);
    return b;
}

bool decodeFacts(const uint8_t* data, size_t size, AvFacts& out) {
    Rd r{data, size};
    if (r.u8() != kChannelAvatars || r.u8() != kAvKindFacts || r.u8() != kAvFactsVersion) return false;
    AvFacts f;
    f.slot = r.u8();
    if (f.slot >= 8) r.ok = false;   // fnvcoop::kMaxSlots
    if (!r.ok) return false;
    size_t pos = r.i;
    if (!nvcoop::avatar::decodeFactsFields(data, size, pos, f.facts) || pos != size) return false;
    out = std::move(f);
    return true;
}

// Pack: [ch][0x45][ver][slot] then the Pack fields (look_codec.h). Entries are sorted by form with a count above zero; anything else is rejected as a whole.
AvBytes encodePack(const AvPack& p) {
    AvBytes b;
    put8(b, kChannelAvatars);
    put8(b, kAvKindPack);
    put8(b, kAvPackVersion);
    put8(b, p.slot);
    nvcoop::avatar::encodePackFields(b, p.pack);
    return b;
}

bool decodePack(const uint8_t* data, size_t size, AvPack& out) {
    Rd r{data, size};
    if (r.u8() != kChannelAvatars || r.u8() != kAvKindPack || r.u8() != kAvPackVersion) return false;
    AvPack p;
    p.slot = r.u8();
    if (p.slot >= 8) r.ok = false;   // fnvcoop::kMaxSlots
    if (!r.ok) return false;
    size_t pos = r.i;
    if (!nvcoop::avatar::decodePackFields(data, size, pos, p.pack) || pos != size) return false;
    out = std::move(p);
    return true;
}

// Look v2: [ch][0x43][ver=2][slot] then the Appearance fields and the equipment list (look_codec.h)
AvBytes encodeLook(const AvLook& l) {
    AvBytes b;
    put8(b, kChannelAvatars);
    put8(b, kAvKindLook2);
    put8(b, kAvatarWireVersion);
    put8(b, l.slot);
    nvcoop::avatar::encodeAppearanceFields(b, l.appearance);
    nvcoop::avatar::encodeEquipEntries(b, l.equipment);
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
    if (!r.ok) return false;
    size_t pos = r.i;
    if (!nvcoop::avatar::decodeAppearanceFields(r.p, r.n, pos, l.appearance)) return false;
    if (!nvcoop::avatar::decodeEquipEntries(r.p, r.n, pos, l.equipment)) return false;
    if (pos != r.n) return false;
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
