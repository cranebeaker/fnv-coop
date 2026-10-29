// The sending half of voice chat: decides when the microphone is on the air (push-to-talk, or open mic with a loudness gate),
// encodes the frames that go out and wraps them in datagrams. Game-free; fed one 20 ms frame at a time.
#pragma once
#include <deque>
#include <vector>

#include "voice_codec.h"
#include "voice_types.h"

namespace nvcoop::voice {

enum class Mode { PushToTalk, OpenMic };

struct TxConfig {
    Mode mode = Mode::PushToTalk;
    uint32_t tailFrames = 12;        // push-to-talk: keep sending this long after the key goes up (240 ms: word endings are not cut)
    uint32_t hangFrames = 20;        // open mic: keep sending this long after the voice dropped below the gate (400 ms)
    float openThreshold = 600.0f;    // open mic: the loudness (average absolute sample, 0 to 32767) that opens the gate
    uint32_t prerollFrames = 3;      // open mic: this many frames from just before the gate opened are sent first (60 ms)
    float micGain = 1.0f;            // multiplier on the microphone
};

struct TxStats {
    uint64_t frames = 0, bursts = 0, encodeErrors = 0, muted = 0;
};

class VoiceTransmitter {
public:
    VoiceTransmitter(const TxConfig& cfg, IVoiceEncoder& enc, uint8_t speaker) : cfg_(cfg), enc_(enc), speaker_(speaker) {}

    // One frame (kFrameSamples) from the microphone. `keyDown`: the talk key is held. `muted`: the player muted their own
    // microphone. The datagrams to send (0 or more) are appended to `out`.
    void feed(const int16_t* pcm, bool keyDown, bool muted, std::vector<Bytes>* out);
    bool talking() const { return talking_; }
    float level() const { return level_; }       // loudness of the last frame, after the gain
    uint16_t nextSeq() const { return seq_; }
    void setConfig(const TxConfig& c) { cfg_ = c; }
    const TxStats& stats() const { return stats_; }

private:
    void emit(const int16_t* pcm, uint8_t flags, std::vector<Bytes>* out);

    TxConfig cfg_;
    IVoiceEncoder& enc_;
    uint8_t speaker_;
    bool talking_ = false;
    uint32_t hold_ = 0;
    uint16_t seq_ = 0;
    float level_ = 0;
    std::deque<std::vector<int16_t>> preroll_;
    TxStats stats_;
};

// Average absolute sample of `n` samples (0 to 32767).
float frameLoudness(const int16_t* pcm, size_t n);

}  // namespace nvcoop::voice
