// Voice datagram: [0x56][version][speaker slot][seq u16 LE][flags][len][len codec bytes]. Exact length, no trailing bytes.
#pragma once
#include "voice_types.h"

namespace nvcoop::voice {

constexpr size_t kHeaderSize = 7;

struct VoiceFrame {
    uint8_t speaker = 0;
    uint16_t seq = 0;
    uint8_t flags = 0;
    const uint8_t* payload = nullptr;   // points into the datagram that was decoded
    size_t size = 0;
};

Bytes encodeVoice(uint8_t speaker, uint16_t seq, uint8_t flags, const uint8_t* payload, size_t size);
// false for anything that is not exactly one well-formed frame (wrong channel / version, unknown flag bits, speaker out of range,
// empty or oversized payload, a length that does not match the datagram).
bool decodeVoice(const uint8_t* data, size_t size, VoiceFrame* out);

}  // namespace nvcoop::voice
