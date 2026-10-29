#include "combat/combat_wire.h"
#include "fnvcoop/wire.h"
#include "../../tools/mut/fnvcoop_mut.h"

namespace nvcoop::combat {

namespace {

void ioEntity(fnvcoop::Archive& a, cc::Entity& e) {
    uint8_t k = static_cast<uint8_t>(e.kind);
    a.u8(k);
    if (a.reading()) {
        if (k > static_cast<uint8_t>(cc::EntityKind::Actor)) { a.fail(); return; }
        e.kind = static_cast<cc::EntityKind>(k);
    }
    if (e.kind == cc::EntityKind::None) { if (a.reading()) e.id = 0; return; }
    a.varu32(e.id);
}

void io(fnvcoop::Archive& a, MHit& m) {
    cc::io(a, m.h);
    if (!FNV_MUT(806)) a.boolean(m.touch);   // 806: the touch flag never travels (the owner takes a point of health off for a blow that did none)
    if (!FNV_MUT(807)) a.varu32(m.centi);    // 807: the exact damage never travels (the owner falls back to the rules' whole points: a 0.5 fist lands as 1)
    if (a.reading() && m.centi > kMaxCenti) a.fail();
}
void io(fnvcoop::Archive& a, MHitNack& m) {
    a.u64(m.hitId);
    uint8_t r = static_cast<uint8_t>(m.reason);
    a.u8(r);
    if (a.reading()) {
        if (r >= static_cast<uint8_t>(cc::HitReason::Count_)) { a.fail(); return; }
        m.reason = static_cast<cc::HitReason>(r);
    }
    a.boolean(m.resync);
}
void io(fnvcoop::Archive& a, MDeath& m) { cc::io(a, m.d); }
void io(fnvcoop::Archive& a, MProjectile& m) { cc::io(a, m.p); }
void io(fnvcoop::Archive& a, MExplosion& m) { cc::io(a, m.e); }
void io(fnvcoop::Archive& a, MCrime& m) { nvcoop::combat::io(a, m.r); }
void io(fnvcoop::Archive& a, MHostility& m) { cc::io(a, m.e); }

template <size_t I = 0>
bool readBody(fnvcoop::Archive& a, size_t idx, Body& out) {
    if constexpr (I == std::variant_size_v<Body>) {
        (void)a; (void)idx; (void)out;
        return false;
    } else {
        if (idx == I) {
            std::variant_alternative_t<I, Body> v{};
            io(a, v);
            if (!a.ok()) return false;
            out = std::move(v);
            return true;
        }
        return readBody<I + 1>(a, idx, out);
    }
}

}  // namespace

void io(fnvcoop::Archive& a, MVitals& v) {
    ioEntity(a, v.who);
    a.varu32(v.epoch);
    a.varu32(v.seq);
    a.vari32(v.health);
    a.vari32(v.maxHealth);
    a.u8(v.limbMask);
    a.boolean(v.dead);
    a.u64(v.cause);
}

void io(fnvcoop::Archive& a, cc::CrimeReport& r) {
    a.u8(r.criminal);
    ioEntity(a, r.victim);
    fnvcoop::io(a, r.victimFaction);
    fnvcoop::io(a, r.criminalFaction);
    a.boolean(r.victimWasHostile);
    a.boolean(r.killed);
    a.boolean(r.witnessed);
    a.u8(r.reporter);
}

const char* bodyName(const Body& b) {
    static const char* names[] = {"Hit", "HitNack", "Vitals", "Death", "Projectile", "Explosion", "Crime", "Hostility"};
    return names[b.index()];
}

Kind kindOf(const Body& b) { return static_cast<Kind>(b.index() + 1); }

Bytes encodeEnvelope(const Envelope& e) {
    fnvcoop::Archive a = fnvcoop::Archive::writer();
    uint8_t src = e.src, dest = e.dest, hops = e.hops, idx = static_cast<uint8_t>(e.body.index());
    a.u8(src); a.u8(dest); a.u8(hops); a.u8(idx);
    Body copy = e.body;
    std::visit([&](auto& m) { io(a, m); }, copy);
    Bytes body = a.finish();
    Bytes out;
    out.reserve(body.size() + 2);
    out.push_back(kChannelCombat);
    out.push_back(kCombatModuleWire);
    out.insert(out.end(), body.begin(), body.end());
    return out;
}

bool decodeEnvelope(const uint8_t* data, size_t size, Envelope& out) {
    if (!data || size < 3 || data[0] != kChannelCombat || data[1] != kCombatModuleWire) return false;
    fnvcoop::Archive a = fnvcoop::Archive::reader(data + 2, size - 2);
    uint8_t idx = 0;
    a.u8(out.src); a.u8(out.dest); a.u8(out.hops); a.u8(idx);
    if (!a.ok() || idx >= std::variant_size_v<Body>) return false;
    if (!readBody(a, idx, out.body)) return false;
    return a.ok() && a.atEnd();
}

}  // namespace nvcoop::combat
