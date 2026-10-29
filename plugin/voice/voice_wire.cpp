#include "voice_wire.h"

#include "fnvcoop_mut.h"

namespace nvcoop::voice {

Bytes encodeVoice(uint8_t speaker, uint16_t seq, uint8_t flags, const uint8_t* payload, size_t size) {
    Bytes b;
    if (!payload || size == 0 || size > kMaxPayload) return b;
    b.reserve(kHeaderSize + size);
    b.push_back(kChannelVoice);
    b.push_back(kWireVersion);
    b.push_back(speaker);
    if (FNV_MUT(301)) {   // twin 301: the sequence number goes out big endian
        b.push_back(static_cast<uint8_t>(seq >> 8));
        b.push_back(static_cast<uint8_t>(seq & 0xFF));
    } else {
        b.push_back(static_cast<uint8_t>(seq & 0xFF));
        b.push_back(static_cast<uint8_t>(seq >> 8));
    }
    b.push_back(flags);
    b.push_back(static_cast<uint8_t>(size));
    b.insert(b.end(), payload, payload + size);
    return b;
}

bool decodeVoice(const uint8_t* d, size_t n, VoiceFrame* out) {
    if (!d || !out || n < kHeaderSize) return false;
    if (d[0] != kChannelVoice) return false;
    if (d[1] != kWireVersion && !FNV_MUT(302)) return false;                       // twin 302: any version is taken
    if (d[2] >= kMaxSlots && !FNV_MUT(303)) return false;                          // twin 303: a speaker slot beyond the party is taken
    if ((d[5] & ~kFlagsKnown) != 0 && !FNV_MUT(304)) return false;                 // twin 304: unknown flag bits are taken
    const size_t len = d[6];
    if (len == 0 && !FNV_MUT(305)) return false;                                   // twin 305: an empty frame is taken
    if (len > kMaxPayload) return false;
    if (FNV_MUT(306) ? n < kHeaderSize + len : n != kHeaderSize + len) return false;   // twin 306: trailing bytes are taken
    out->speaker = d[2];
    out->seq = static_cast<uint16_t>(d[3] | (d[4] << 8));
    out->flags = d[5];
    out->payload = d + kHeaderSize;
    out->size = len;
    return true;
}

}  // namespace nvcoop::voice
