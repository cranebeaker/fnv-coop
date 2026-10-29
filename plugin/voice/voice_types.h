// Voice chat: shared constants. Game-free (no NVSE, no Win32).
//
// One voice frame is 20 ms of 16 kHz mono speech. A frame travels as one datagram on the co-op mux (first byte kChannelVoice),
// unreliable, host relayed. See docs/voice.md.
#pragma once
#include <cstddef>
#include <cstdint>
#include <vector>

namespace nvcoop::voice {

using Bytes = std::vector<uint8_t>;

constexpr uint8_t kChannelVoice = 0x56;     // first byte of every datagram of this module (docs/voice.md; not used by any other channel)
constexpr uint8_t kWireVersion = 1;
constexpr uint8_t kMaxSlots = 8;            // fnvcoop::kMaxSlots: slot 0 is the host's player

constexpr uint32_t kSampleRate = 16000;     // capture, codec and playback rate (mono in, stereo out)
constexpr uint32_t kFrameMs = 20;
constexpr size_t kFrameSamples = kSampleRate * kFrameMs / 1000;   // 320
constexpr size_t kMaxPayload = 200;         // codec bytes in one frame (a 24 kbit/s frame is about 60)

enum : uint8_t {
    kFlagStart = 1,   // first frame of a talk burst: the receiver starts a fresh stream
    kFlagEnd = 2,     // last frame of a talk burst: the receiver plays what is left and goes quiet
    kFlagsKnown = kFlagStart | kFlagEnd,
};

}  // namespace nvcoop::voice
