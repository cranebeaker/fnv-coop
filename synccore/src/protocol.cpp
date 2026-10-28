#include "fnvcoop/protocol.h"

#include <utility>

#if defined(_MSC_VER)
#define FNVCOOP_NOINLINE __declspec(noinline)
#else
#define FNVCOOP_NOINLINE __attribute__((noinline))
#endif

namespace fnvcoop {

namespace {
constexpr uint8_t kMagic0 = 0xFC, kMagic1 = 0x4E;


void ioM(Archive& a, HandshakeInfo& h) {
    uint32_t p = h.protocol;
    a.varu32(p);
    if (a.reading()) { if (p > 0xFFFF) { a.fail(); return; } h.protocol = static_cast<uint16_t>(p); }
    a.varu32(h.buildNumber); a.u64(h.buildHash); a.u64(h.rulesHash); a.u64(h.volatileHash);
    a.str(h.buildName, 64);
    size_t n = h.plugins.size();
    if (!a.count(n, 1024)) { h.plugins.clear(); return; }
    if (a.reading()) {
        h.plugins.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) { std::string s; a.str(s, 255); if (a.ok()) h.plugins.push_back(std::move(s)); }
    } else {
        for (auto& s : h.plugins) a.str(s, 255);
    }
}
void ioM(Archive& a, Hello& m) {
    a.varu32(m.session); a.u64(m.token); a.varu(m.lastSeq); a.u8(m.flags); a.u64(m.contentHash); a.varu32(m.lastEpoch);
    ioM(a, m.hs);
    if (a.reading() && a.atEnd()) m.cursorCheck = 0;   // a Hello from a build before the field
    else a.u64(m.cursorCheck);
}
void ioM(Archive& a, Welcome& m) {
    a.varu32(m.session); a.u8(m.slot); a.varu(m.headSeq); io(a, m.policy);
    if (a.reading() && m.slot >= kMaxSlots) { a.fail(); return; }
    a.boolean(m.hasSnapshot);
    if (m.hasSnapshot) io(a, m.snapshot);
    a.varu32(m.intentEpoch);
    a.varu32(m.lastIntent);
    a.varu32(m.baselineId);
    a.u8(m.baselineAbs);
    a.varu32(m.worldEpoch);
    size_t n = m.rewards.size();
    if (!a.count(n, kMaxRewardsPerWelcome)) { m.rewards.clear(); return; }
    if (a.reading()) {
        m.rewards.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) { RewardGrant g; a.varu(g.id); io(a, g.effect); if (a.ok()) m.rewards.push_back(std::move(g)); }
    } else {
        for (auto& g : m.rewards) { uint64_t id = g.id; a.varu(id); io(a, g.effect); }
    }
}
void ioM(Archive& a, Reject& m) { a.varu32(m.session); a.u8(m.reason); a.str(m.text, 255); }
void ioM(Archive& a, TxnBatch& m) {
    a.varu32(m.session);
    a.varu32(m.worldEpoch);
    size_t n = m.txns.size();
    if (!a.count(n, kMaxTxnsPerBatch)) { m.txns.clear(); return; }
    if (a.reading()) {
        m.txns.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) { EffectTxn t; io(a, t); if (a.ok()) m.txns.push_back(std::move(t)); }
    } else {
        for (auto& t : m.txns) io(a, t);
    }
}
void ioM(Archive& a, Ack& m) { a.varu32(m.session); a.varu(m.upTo); a.varu32(m.worldEpoch); }
void ioM(Archive& a, Rebase& m) { a.varu32(m.session); a.varu32(m.epoch); }
void ioM(Archive& a, Intent& m) {
    a.varu32(m.session); a.varu32(m.epoch); a.varu32(m.id);
    uint8_t kind = static_cast<uint8_t>(m.kind);
    a.u8(kind);
    if (a.reading()) {
        if (kind >= static_cast<uint8_t>(IntentKind::Count_)) { a.fail(); return; }
        m.kind = static_cast<IntentKind>(kind);
    }
    io(a, m.cause);
    size_t n = m.effects.size();
    if (!a.count(n, kMaxEffectsPerTxn)) { m.effects.clear(); return; }
    if (a.reading()) {
        m.effects.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) { Effect e; io(a, e); if (a.ok()) m.effects.push_back(std::move(e)); }
    } else {
        for (auto& e : m.effects) io(a, e);
    }
    if (m.kind != IntentKind::AppliedEffects) io(a, m.ref);
}
void ioM(Archive& a, AppData& m) {
    a.varu32(m.session);
    uint32_t ch = m.channel; a.varu32(ch);
    if (a.reading()) { if (ch > 0xFFFF) { a.fail(); return; } m.channel = static_cast<uint16_t>(ch); }
    a.u8(m.from);
    a.bytes(m.payload, kMaxAppPayload);
}
void ioM(Archive& a, BaselineSummaryChunk& m) {
    a.varu32(m.session); a.varu32(m.id); a.varu32(m.index); a.varu32(m.total);
    if (a.reading() && (m.total == 0 || m.index >= m.total)) { a.fail(); return; }
    size_t n = m.entries.size();
    if (!a.count(n, 4096)) { m.entries.clear(); return; }
    if (a.reading()) {
        m.entries.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) { uint64_t k = 0; uint32_t v = 0; a.u64(k); a.u32(v); if (a.ok()) m.entries.emplace_back(k, v); }
    } else {
        for (auto& e : m.entries) { uint64_t k = e.first; uint32_t v = e.second; a.u64(k); a.u32(v); }
    }
    a.boolean(m.hasAbs);
    a.vari(m.karma);
    size_t r = m.rep.size();
    if (!a.count(r, 4096)) { m.rep.clear(); return; }
    if (a.reading()) {
        m.rep.clear();
        for (size_t i = 0; i < r && a.ok(); ++i) { RepAbs x; io(a, x.faction); a.vari(x.fame); a.vari(x.infamy); if (a.ok()) m.rep.push_back(std::move(x)); }
    } else {
        for (auto& x : m.rep) { io(a, x.faction); a.vari(x.fame); a.vari(x.infamy); }
    }
}
void ioM(Archive& a, BaselineDiffChunk& m) {
    a.varu32(m.session); a.varu32(m.id); a.varu32(m.index); a.varu32(m.total);
    if (a.reading() && (m.total == 0 || m.index >= m.total)) { a.fail(); return; }
    size_t n = m.effects.size();
    if (!a.count(n, kMaxEffectsPerTxn)) { m.effects.clear(); return; }
    if (a.reading()) {
        m.effects.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) { Effect e; io(a, e); if (a.ok()) m.effects.push_back(std::move(e)); }
    } else {
        for (auto& e : m.effects) io(a, e);
    }
    size_t r = m.revertKeys.size();
    if (!a.count(r, 4096)) { m.revertKeys.clear(); return; }
    if (a.reading()) {
        m.revertKeys.clear();
        for (size_t i = 0; i < r && a.ok(); ++i) { uint64_t k = 0; a.u64(k); if (a.ok()) m.revertKeys.push_back(k); }
    } else {
        for (auto k : m.revertKeys) a.u64(k);
    }
    a.varu32(m.kept);
}
void ioM(Archive& a, BaselineAck& m) {
    a.varu32(m.session); a.varu32(m.id); a.u8(m.phase); a.varu32(m.next);
    if (a.reading() && m.phase > 1) a.fail();
}
void ioM(Archive& a, SyncDigest& m) {
    a.varu32(m.session); a.varu32(m.epoch); a.varu(m.seq); a.varu32(m.digestId);
    size_t n = m.shards.size();
    if (!a.count(n, kSyncShards)) { m.shards.clear(); return; }
    if (a.reading()) {
        if (n != kSyncShards) { a.fail(); return; }
        m.shards.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) { uint32_t v = 0; a.u32(v); m.shards.push_back(v); }
    } else {
        for (auto v : m.shards) a.u32(v);
    }
}
void ioM(Archive& a, ShardRequest& m) { a.varu32(m.session); a.varu32(m.digestId); a.u64(m.maskLo); a.u64(m.maskHi); }
void ioM(Archive& a, ShardEntries& m) {
    a.varu32(m.session); a.varu32(m.digestId); a.boolean(m.truncated);
    size_t n = m.entries.size();
    if (!a.count(n, kMaxShardEntries)) { m.entries.clear(); return; }
    if (a.reading()) {
        m.entries.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) { Effect e; io(a, e); if (a.ok()) m.entries.push_back(std::move(e)); }
    } else {
        for (auto& e : m.entries) io(a, e);
    }
}
void ioM(Archive& a, SyncRepair& m) {
    a.varu32(m.session); a.varu32(m.epoch); a.varu32(m.digestId);
    size_t n = m.effects.size();
    if (!a.count(n, kMaxEffectsPerTxn)) { m.effects.clear(); return; }
    if (a.reading()) {
        m.effects.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) { Effect e; io(a, e); if (a.ok()) m.effects.push_back(std::move(e)); }
    } else {
        for (auto& e : m.effects) io(a, e);
    }
    size_t r = m.revertKeys.size();
    if (!a.count(r, 4096)) { m.revertKeys.clear(); return; }
    if (a.reading()) {
        m.revertKeys.clear();
        for (size_t i = 0; i < r && a.ok(); ++i) { uint64_t k = 0; a.u64(k); if (a.ok()) m.revertKeys.push_back(k); }
    } else {
        for (auto k : m.revertKeys) a.u64(k);
    }
}
void ioM(Archive& a, DigestRequest& m) { a.varu32(m.session); a.varu32(m.epoch); }
void ioM(Archive& a, IntentAck& m) { a.varu32(m.session); a.varu32(m.epoch); a.varu32(m.id); }

// One alternative per function, picked by a flat fold: only the matching reader's frame is ever on the stack. The old self-recursive
// readMsg<I> -> readMsg<I + 1> kept one ~1.4 KB frame per message type below the match (MSVC reserves the whole frame on entry), and a
// client copy overflowed its stack decoding a late message type (Goodsprings story run S02, 2026-10-28: C0000005 on the first push after
// `sub esp, 55Ch` in readMsg<1>, three dumps).
template <size_t I>
FNVCOOP_NOINLINE bool readOne(Archive& a, Message& out) {
    std::variant_alternative_t<I, Message> m{};
    ioM(a, m);
    out = std::move(m);
    return a.ok();
}
template <size_t... I>
bool readMsgIn(Archive& a, uint8_t type, Message& out, std::index_sequence<I...>) {
    bool ok = false, hit = false;
    ((type == I + 1 ? (hit = true, ok = readOne<I>(a, out), true) : false) || ...);
    return hit && ok;
}
bool readMsg(Archive& a, uint8_t type, Message& out) {
    return readMsgIn(a, type, out, std::make_index_sequence<std::variant_size_v<Message>>{});
}
}  // namespace

bool isHandshakeType(uint8_t type) { return type == 1 || type == 3; }   // index of Hello / Reject in Message, plus one

const char* toString(DecodeStatus s) {
    switch (s) {
        case DecodeStatus::Ok: return "ok";
        case DecodeStatus::TooShort: return "too short";
        case DecodeStatus::BadMagic: return "bad magic";
        case DecodeStatus::BadVersion: return "bad version";
        case DecodeStatus::BadCrc: return "bad crc";
        case DecodeStatus::Malformed: return "malformed";
        case DecodeStatus::Trailing: return "trailing bytes";
    }
    return "?";
}

Bytes encodeMessage(const Message& msg) {
    Message m = msg;
    Archive a = Archive::writer();
    std::visit([&](auto& x) { ioM(a, x); }, m);
    Bytes body = a.finish();
    if (!a.ok() || body.empty() || body.size() > kMaxMessageBytes) return {};
    const uint8_t type = static_cast<uint8_t>(m.index() + 1);
    Bytes out{kMagic0, kMagic1, isHandshakeType(type) ? kHandshakeEnvelope : kWireVersion, type};
    out.insert(out.end(), body.begin(), body.end());
    uint32_t c = crc32(out.data(), out.size());
    for (int i = 0; i < 4; ++i) out.push_back(static_cast<uint8_t>(c >> (8 * i)));
    return out;
}

DecodeStatus decodeMessage(const uint8_t* data, size_t size, Message& out, bool verifyCrc) {
    if (size < 4 + 1 + 4) return DecodeStatus::TooShort;
    if (size > kMaxMessageBytes) return DecodeStatus::Malformed;
    if (data[0] != kMagic0 || data[1] != kMagic1) return DecodeStatus::BadMagic;
    const bool hs = isHandshakeType(data[3]);
    if (data[2] != (hs ? kHandshakeEnvelope : kWireVersion)) return DecodeStatus::BadVersion;
    if (verifyCrc) {
        uint32_t want = 0;
        for (int i = 0; i < 4; ++i) want |= static_cast<uint32_t>(data[size - 4 + i]) << (8 * i);
        if (crc32(data, size - 4) != want) return DecodeStatus::BadCrc;
    }
    Archive a = Archive::reader(data + 4, size - 8);
    if (!a.ok()) return DecodeStatus::Malformed;
    Message m;
    if (!readMsg(a, data[3], m)) return DecodeStatus::Malformed;
    if (!a.atEnd() && !hs) return DecodeStatus::Trailing;   // handshake messages may carry fields from a newer build
    out = std::move(m);
    return DecodeStatus::Ok;
}

}  // namespace fnvcoop
