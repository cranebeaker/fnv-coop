// The hearing half of voice chat: one jitter buffer and one decoder per remote speaker, mixed into one stereo stream at the volume
// and side of where each speaker is (placement.h), changed smoothly. Game-free. NOT thread safe: the caller locks (the audio
// thread calls render(), the game thread calls the rest).
#pragma once
#include <map>
#include <memory>

#include "voice_jitter.h"
#include "voice_mix.h"
#include "voice_placement.h"
#include "voice_codec.h"
#include "voice_types.h"

namespace nvcoop::voice {

struct RxStats {
    uint64_t framesIn = 0, duplicates = 0, late = 0, concealed = 0, decodeErrors = 0, skipped = 0, droppedInaudible = 0;
};

class VoiceReceiver {
public:
    VoiceReceiver(IVoiceCodecFactory& codecs, JitterConfig jitter = {}) : codecs_(codecs), jitterCfg_(jitter) {}

    // One datagram's frame of speaker `who` (a slot). Unknown speakers get a decoder and a buffer on first use (at most kMaxSlots).
    PushResult push(uint8_t who, uint16_t seq, uint8_t flags, const uint8_t* data, size_t size);
    // The mix for the next `frames` output frames (interleaved left, right). Always writes all of them.
    void render(int16_t* stereo, size_t frames);

    void setPlacement(uint8_t who, const Placement& p);   // where the speaker is now; the mix moves there smoothly
    void setMuted(uint8_t who, bool muted);               // this listener's mute of that player
    bool muted(uint8_t who) const;
    void setMasterVolume(float v) { master_ = v < 0 ? 0 : v > 2 ? 2 : v; }
    float masterVolume() const { return master_; }
    void forget(uint8_t who) { speakers_.erase(who); }
    void clear() { speakers_.clear(); }

    bool talking(uint8_t who) const;                      // a frame of theirs was played within the last 300 ms
    float gainOf(uint8_t who) const;                      // the gain the mix uses for them right now (after smoothing, both sides averaged)
    size_t speakerCount() const { return speakers_.size(); }
    size_t depthOf(uint8_t who) const;                    // frames waiting in their jitter buffer
    const JitterStats* jitterOf(uint8_t who) const;
    const RxStats& stats() const { return stats_; }

private:
    struct Speaker {
        explicit Speaker(JitterConfig j) : jitter(j) {}
        JitterBuffer jitter;
        std::unique_ptr<IVoiceDecoder> decoder;
        SampleFifo pcm;               // decoded, not played yet
        VoiceGain gain;               // the gains in use now, moved towards the target a sample at a time
        uint32_t quiet = 1u << 30;    // samples since the last decoded frame was taken for playing
    };
    Speaker* find(uint8_t who);
    const Speaker* find(uint8_t who) const;
    void fill(Speaker& s, size_t frames, bool heard);

    IVoiceCodecFactory& codecs_;
    JitterConfig jitterCfg_;
    std::map<uint8_t, Speaker> speakers_;
    MixBus bus_;                   // scratch for render(), kept so the audio thread allocates nothing once warm
    std::vector<int16_t> block_;   // one speaker's samples for the block being mixed
    Placement place_[kMaxSlots];   // by slot, kept when a speaker is forgotten
    bool muted_[kMaxSlots] = {};
    float master_ = 1.0f;
    RxStats stats_;
};

}  // namespace nvcoop::voice
