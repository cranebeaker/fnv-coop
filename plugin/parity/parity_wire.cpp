#include "parity/parity_wire.h"

#include <cmath>

#include "fnvcoop/wire.h"
#include "fnvcoop_mut.h"

// Twins: 1800 = the decoder does not check the crc; 1801 = the decoder accepts bytes after the sample; 1802 = the encoder writes positions as unsigned (a negative
//        coordinate comes back wrong); 1803 = the look is not written; 1804-type twins are in parity_compare.cpp
namespace nvcoop::parity {

namespace {

void put8(fnvcoop::Bytes& b, uint8_t v) { b.push_back(v); }
void putVar(fnvcoop::Bytes& b, uint32_t v) {
    while (v >= 0x80) { b.push_back(static_cast<uint8_t>(v | 0x80)); v >>= 7; }
    b.push_back(static_cast<uint8_t>(v));
}
void putSigned(fnvcoop::Bytes& b, int32_t v) {
    if (FNV_MUT(1802)) { putVar(b, static_cast<uint32_t>(v)); return; }
    putVar(b, (static_cast<uint32_t>(v) << 1) ^ static_cast<uint32_t>(v >> 31));
}
void put32(fnvcoop::Bytes& b, uint32_t v) { for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(v >> (8 * i))); }

struct Reader {
    const uint8_t* p; size_t n; size_t i = 0; bool ok = true;
    uint8_t u8() { if (i + 1 > n) { ok = false; return 0; } return p[i++]; }
    uint32_t var() {
        uint32_t v = 0;
        for (int sh = 0; sh < 35; sh += 7) {
            const uint8_t c = u8();
            if (!ok) return 0;
            v |= static_cast<uint32_t>(c & 0x7F) << sh;
            if (!(c & 0x80)) return v;
        }
        ok = false;
        return 0;
    }
    int32_t sgn() {
        const uint32_t u = var();
        return static_cast<int32_t>((u >> 1) ^ (~(u & 1) + 1));
    }
    uint32_t u32() {
        if (i + 4 > n) { ok = false; return 0; }
        uint32_t v = 0;
        for (int k = 0; k < 4; ++k) v |= uint32_t(p[i + k]) << (8 * k);
        i += 4;
        return v;
    }
};

}  // namespace

fnvcoop::Bytes encodeSample(const Sample& s) {
    fnvcoop::Bytes b{kChannelParity, kWireVersion, 1};
    std::vector<const Ent*> out;
    for (const Ent& e : s.ents) out.push_back(&e);
    if (out.size() > kMaxEnts) out.resize(kMaxEnts);
    const uint32_t cov = s.cov;
    put8(b, s.slot);
    putVar(b, s.seq);
    putVar(b, s.cell);
    putVar(b, s.ws);
    putVar(b, cov);
    put8(b, s.menu ? 1 : 0);
    put8(b, static_cast<uint8_t>(out.size()));
    for (const Ent* e : out) {
        const bool player = e->kind != EntKind::Npc;
        const bool look = player && e->look.known && !FNV_MUT(1803);
        uint8_t head = static_cast<uint8_t>(e->kind) & 3;
        if (e->seated) head |= 4;
        if (e->dead) head |= 8;
        if (e->disabled) head |= 16;
        if (look) head |= 32;
        if (player) head |= 64;
        put8(b, head);
        if (player) put8(b, e->slot);
        putVar(b, e->ref);
        putSigned(b, e->x);
        putSigned(b, e->y);
        putSigned(b, e->z);
        put8(b, e->speed16);
        put8(b, e->hp);
        put8(b, e->group);
        if (player) putVar(b, e->acts);
        if (look) {
            putVar(b, e->look.race);
            put8(b, e->look.female ? 1 : 0);
            putVar(b, e->look.hair);
            putVar(b, e->look.eyes);
            put32(b, e->look.faceHash);
        }
    }
    put32(b, fnvcoop::crc32(b.data(), b.size()));
    return b;
}

bool decodeSample(const uint8_t* data, size_t size, Sample& out) {
    if (!data || size < 3 + 7 + 4 || size > kMaxDatagramBytes) return false;
    if (data[0] != kChannelParity || data[1] != kWireVersion || data[2] != 1) return false;
    if (!FNV_MUT(1800)) {
        uint32_t want = 0;
        for (int k = 0; k < 4; ++k) want |= uint32_t(data[size - 4 + k]) << (8 * k);
        if (fnvcoop::crc32(data, size - 4) != want) return false;
    }
    Reader r{data, size - 4, 3};
    Sample s;
    s.slot = r.u8();
    s.seq = r.var();
    s.cell = r.var();
    s.ws = r.var();
    s.cov = r.var();
    s.rad = s.cov;
    s.menu = (r.u8() & 1) != 0;
    const unsigned n = r.u8();
    if (!r.ok || n > kMaxEnts) return false;
    for (unsigned k = 0; k < n; ++k) {
        Ent e;
        const uint8_t head = r.u8();
        const unsigned kind = head & 3;
        if (kind > 2) return false;
        e.kind = static_cast<EntKind>(kind);
        e.seated = (head & 4) != 0;
        e.dead = (head & 8) != 0;
        e.disabled = (head & 16) != 0;
        const bool player = (head & 64) != 0;
        if (player != (e.kind != EntKind::Npc)) return false;
        if (player) e.slot = r.u8();
        e.ref = r.var();
        e.x = r.sgn();
        e.y = r.sgn();
        e.z = r.sgn();
        e.speed16 = r.u8();
        e.hp = r.u8();
        e.group = r.u8();
        if (player) e.acts = r.var();
        if (head & 32) {
            e.look.known = true;
            e.look.race = r.var();
            e.look.female = r.u8() != 0;
            e.look.hair = r.var();
            e.look.eyes = r.var();
            e.look.faceHash = r.u32();
        }
        if (!r.ok) return false;
        s.ents.push_back(e);
    }
    if (!r.ok || (r.i != r.n && !FNV_MUT(1801))) return false;
    out = std::move(s);
    return true;
}

}  // namespace nvcoop::parity
