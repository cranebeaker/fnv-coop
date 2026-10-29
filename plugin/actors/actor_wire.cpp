#include "actors/actor_wire.h"
#include "fnvcoop/wire.h"

namespace nvcoop::actors {

namespace {
constexpr size_t kMaxAv = 64;
constexpr size_t kMaxPartBytes = 8192;   // one part's value (state_parts.h)

void ioVec(fnvcoop::Archive& a, live::Vec3& v) { a.f32(v.x); a.f32(v.y); a.f32(v.z); }

template <size_t I = 0>
bool readBody(fnvcoop::Archive& a, size_t idx, Body& out);
}  // namespace

void io(fnvcoop::Archive& a, live::ActorSnapshot& s) {
    a.varu32(s.netId); a.varu32(s.ownerEpoch); a.varu32(s.seq); a.varu32(s.tick); a.varu32(s.cell);
    ioVec(a, s.pos); ioVec(a, s.rot); ioVec(a, s.vel);
    a.u8(s.anim.locomotion);
    // Same packing as live-core's datagram (live.h, schema v2): five flag bits, bit 7 = the animation extension follows.
    uint8_t f = static_cast<uint8_t>(live::packAnimFlags(s.anim) | (live::animHasExt(s.anim) ? live::kAnimFlagExt : 0));
    a.u8(f);
    uint8_t action = s.anim.action, actionSeq = s.anim.actionSeq, ext = live::packAnimExt(s.anim);
    if (a.reading() && !(f & live::kAnimFlagExt)) { action = 0; actionSeq = 0; ext = 0; }
    if (f & live::kAnimFlagExt) { a.u8(action); a.u8(actionSeq); a.u8(ext); }
    if (a.reading()) {
        if ((f & live::kAnimFlagsReserved) || (ext & 0xC0)) a.fail();
        const uint8_t loco = s.anim.locomotion;
        s.anim = live::unpackAnim(loco, static_cast<uint8_t>(f & ~live::kAnimFlagExt), action, actionSeq, ext);
    }
    // The furniture block (live schema v3): a handoff or a resync must not stand a seated actor up until the next stream snapshot. Always one
    // sit byte (0 = not in furniture), then the furniture reference only when it is non-zero.
    uint8_t sit = live::packAnimSit(s.anim);
    uint32_t furniture = sit ? s.anim.furniture : 0;
    a.u8(sit);
    if (sit) a.varu32(furniture);
    if (a.reading()) {
        const uint32_t sitFurniture = sit ? furniture : 0;
        if (!live::unpackAnimSit(s.anim, sit, sitFurniture)) a.fail();
    }
    a.varu32(s.ai.combatTarget); a.varu32(s.ai.package); a.u8(s.ai.alert);
    size_t n = s.avDeltas.size();
    if (!a.count(n, kMaxAv)) return;
    if (a.reading()) s.avDeltas.assign(n, {});
    for (auto& v : s.avDeltas) { a.u8(v.av); a.vari32(v.value); }
}

void io(fnvcoop::Archive& a, ActorMeta& m) {
    a.varu32(m.netId); fnvcoop::io(a, m.ref); fnvcoop::io(a, m.base);
    a.boolean(m.named); a.boolean(m.created); a.varu32(m.cell); ioVec(a, m.pos);
}

namespace {
void io(fnvcoop::Archive& a, MOwn& m) { fnvcoop::io(a, m.m); }
void io(fnvcoop::Archive& a, MTrack& m) { nvcoop::actors::io(a, m.meta); }
void io(fnvcoop::Archive& a, MSnapFull& m) { nvcoop::actors::io(a, m.s); a.boolean(m.final); a.boolean(m.settle); }
void io(fnvcoop::Archive& a, MDeath& m) { a.varu32(m.id); a.varu32(m.epoch); a.varu(m.eventId); a.varu32(m.cell); ioVec(a, m.pos); }
void io(fnvcoop::Archive& a, MPick& m) { a.varu32(m.id); fnvcoop::io(a, m.pick); }
void io(fnvcoop::Archive& a, MPickReport& m) { a.varu32(m.id); fnvcoop::io(a, m.base); }
void io(fnvcoop::Archive& a, MResync& m) { a.varu32(m.id); }
void io(fnvcoop::Archive& a, MPart& m) {
    a.varu32(m.id); a.varu32(m.epoch); a.varu(m.eventId);
    uint8_t k = static_cast<uint8_t>(m.kind);
    a.u8(k);
    if (a.reading()) {
        if (k >= kPartCount) { a.fail(); return; }
        m.kind = static_cast<PartKind>(k);
    }
    a.bytes(m.value, kMaxPartBytes);
}
void io(fnvcoop::Archive& a, MUntrack& m) { a.varu32(m.id); }
void io(fnvcoop::Archive& a, MEvent& m) {
    a.varu32(m.id); a.varu32(m.epoch); a.varu(m.eventId); a.u8(m.kind);
    uint8_t r = static_cast<uint8_t>(m.route);
    a.u8(r);
    if (a.reading()) { if (r > 2) { a.fail(); return; } m.route = static_cast<EventRoute>(r); }
    a.u8(m.hops); a.u8(m.origin);
    a.bytes(m.payload, 4096);
}
void io(fnvcoop::Archive& a, MAnimEvent& m) { a.varu32(m.id); a.varu32(m.epoch); a.varu(m.eventId); a.bytes(m.payload, 256); }

template <size_t I>
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

const char* bodyName(const Body& b) {
    static const char* names[] = {"Own", "Track", "SnapFull", "Death", "Pick", "PickReport", "Resync", "AnimEvent", "Part", "Untrack", "Event"};
    return names[b.index()];
}

Bytes encodeEnvelope(const Envelope& e) {
    fnvcoop::Archive a = fnvcoop::Archive::writer();
    uint8_t ver = kActorWireVersion, src = e.src, dest = e.dest, idx = static_cast<uint8_t>(e.body.index());
    a.u8(ver); a.u8(src); a.u8(dest); a.u8(idx);
    Body copy = e.body;
    std::visit([&](auto& m) { io(a, m); }, copy);
    Bytes body = a.finish();
    Bytes out;
    out.reserve(body.size() + 2);
    out.push_back(kChannelActors);
    out.push_back(SubEnvelope);
    out.insert(out.end(), body.begin(), body.end());
    return out;
}

bool decodeEnvelope(const uint8_t* payload, size_t size, Envelope& out) {
    if (!payload || size == 0) return false;
    fnvcoop::Archive a = fnvcoop::Archive::reader(payload, size);
    uint8_t ver = 0, idx = 0;
    a.u8(ver);
    if (!a.ok() || ver != kActorWireVersion) return false;
    a.u8(out.src); a.u8(out.dest); a.u8(idx);
    if (!a.ok() || idx >= std::variant_size_v<Body>) return false;
    if (!readBody(a, idx, out.body)) return false;
    return a.ok() && a.atEnd();
}

Bytes wrapLive(Sub sub, const Bytes& liveBytes) {
    Bytes out;
    out.reserve(liveBytes.size() + 2);
    out.push_back(kChannelActors);
    out.push_back(sub);
    out.insert(out.end(), liveBytes.begin(), liveBytes.end());
    return out;
}

}  // namespace nvcoop::actors
