// The speech codec behind a small interface, so the game-free logic can be tested with a fake and the real one (Opus) is only
// linked where it is needed. One encoder per local microphone, one decoder per remote speaker.
#pragma once
#include <memory>

#include "voice_types.h"

namespace nvcoop::voice {

class IVoiceEncoder {
public:
    virtual ~IVoiceEncoder() = default;
    // kFrameSamples samples in; the encoded frame is written to `out` (at most `cap` bytes). Returns the size, 0 or less on error.
    virtual int encode(const int16_t* pcm, uint8_t* out, size_t cap) = 0;
};

class IVoiceDecoder {
public:
    virtual ~IVoiceDecoder() = default;
    // kFrameSamples samples out. data == nullptr: the frame was lost, conceal it. Returns the number of samples, 0 or less on error.
    virtual int decode(const uint8_t* data, size_t size, int16_t* pcm) = 0;
    virtual void reset() = 0;   // a new burst: forget the old one's state
};

class IVoiceCodecFactory {
public:
    virtual ~IVoiceCodecFactory() = default;
    virtual std::unique_ptr<IVoiceEncoder> makeEncoder() = 0;
    virtual std::unique_ptr<IVoiceDecoder> makeDecoder() = 0;
};

// Opus (third_party/opus): speech mode, 16 kHz mono, 20 ms frames, constant bit rate off (variable), no dtx. Null when the library
// refuses to start.
std::unique_ptr<IVoiceCodecFactory> makeOpusFactory(int bitrate);

}  // namespace nvcoop::voice
