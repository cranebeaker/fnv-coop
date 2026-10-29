#include "chat/chat_wire.h"

#include "fnvcoop_mut.h"

// Twins: 1060 = the decoder accepts bytes after the message; 1061 = the decoder takes an over-long text; 1062 = the decoder does not check the channel byte;
//        1063 = the decoder does not check the version; 1064 = the encoder writes the text length as the full size_t (a long text overflows the u8)
namespace nvcoop::chat {

namespace {

void put8(Bytes& b, uint8_t v) { b.push_back(v); }
void put32(Bytes& b, uint32_t v) { for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(v >> (8 * i))); }
void putStr(Bytes& b, const std::string& s, size_t max) {
    const size_t n = s.size() < max ? s.size() : max;
    b.push_back(static_cast<uint8_t>(FNV_MUT(1064) ? s.size() : n));
    b.insert(b.end(), s.begin(), s.begin() + static_cast<std::ptrdiff_t>(n));
}
Bytes head(Kind k) { return Bytes{kChannelChat, kWireVersion, static_cast<uint8_t>(k)}; }

struct Reader {
    const uint8_t* p; size_t n; size_t i = 0; bool ok = true;
    uint8_t u8() { if (i + 1 > n) { ok = false; return 0; } return p[i++]; }
    uint32_t u32() {
        if (i + 4 > n) { ok = false; return 0; }
        uint32_t v = 0;
        for (int k = 0; k < 4; ++k) v |= uint32_t(p[i + k]) << (8 * k);
        i += 4;
        return v;
    }
    std::string str(size_t max) {
        const size_t len = u8();
        if (!ok || (len > max && !FNV_MUT(1061)) || i + len > n) { ok = false; return {}; }
        std::string s(reinterpret_cast<const char*>(p + i), len);
        i += len;
        return s;
    }
};

}  // namespace

Bytes encodeSay(uint32_t msgId, const std::string& text) {
    Bytes b = head(Kind::Say);
    put32(b, msgId);
    putStr(b, text, kMaxTextBytes);
    return b;
}

Bytes encodeLine(Slot slot, uint32_t msgId, LineKind lineKind, const std::string& name, const std::string& text) {
    Bytes b = head(Kind::Line);
    put8(b, slot);
    put32(b, msgId);
    put8(b, static_cast<uint8_t>(lineKind));
    putStr(b, name, kMaxNameBytes);
    putStr(b, text, kMaxTextBytes);
    return b;
}

Bytes encodeHello(const std::string& name) {
    Bytes b = head(Kind::Hello);
    putStr(b, name, kMaxNameBytes);
    return b;
}

Bytes encodeNotice(uint8_t code) {
    Bytes b = head(Kind::Notice);
    put8(b, code);
    return b;
}

bool decode(const uint8_t* data, size_t size, Message& out) {
    if (!data || size < 3 || size > kMaxDatagramBytes) return false;
    if (data[0] != kChannelChat && !FNV_MUT(1062)) return false;
    if (data[1] != kWireVersion && !FNV_MUT(1063)) return false;
    Reader r{data, size, 3};
    Message m;
    switch (data[2]) {
        case static_cast<uint8_t>(Kind::Say):
            m.kind = Kind::Say;
            m.msgId = r.u32();
            m.text = r.str(kMaxTextBytes);
            break;
        case static_cast<uint8_t>(Kind::Line): {
            m.kind = Kind::Line;
            m.slot = r.u8();
            m.msgId = r.u32();
            const uint8_t lk = r.u8();
            if (lk > static_cast<uint8_t>(LineKind::System)) return false;
            m.lineKind = static_cast<LineKind>(lk);
            m.name = r.str(kMaxNameBytes);
            m.text = r.str(kMaxTextBytes);
            break;
        }
        case static_cast<uint8_t>(Kind::Hello):
            m.kind = Kind::Hello;
            m.name = r.str(kMaxNameBytes);
            break;
        case static_cast<uint8_t>(Kind::Notice):
            m.kind = Kind::Notice;
            m.code = r.u8();
            break;
        default: return false;
    }
    if (!r.ok) return false;
    if (r.i != size && !FNV_MUT(1060)) return false;
    out = std::move(m);
    return true;
}

}  // namespace nvcoop::chat
