#include "voice_tx.h"

#include <algorithm>
#include <cmath>

#include "fnvcoop_mut.h"
#include "voice_wire.h"

namespace nvcoop::voice {

float frameLoudness(const int16_t* pcm, size_t n) {
    if (!pcm || n == 0) return 0;
    double sum = 0;
    for (size_t i = 0; i < n; ++i) sum += std::fabs(static_cast<double>(pcm[i]));
    return static_cast<float>(sum / static_cast<double>(n));
}

void VoiceTransmitter::emit(const int16_t* pcm, uint8_t flags, std::vector<Bytes>* out) {
    uint8_t buf[kMaxPayload];
    const int n = enc_.encode(pcm, buf, sizeof buf);
    if (n <= 0 || static_cast<size_t>(n) > kMaxPayload) {
        ++stats_.encodeErrors;
        return;
    }
    Bytes b = encodeVoice(speaker_, seq_, flags, buf, static_cast<size_t>(n));
    if (b.empty()) {
        ++stats_.encodeErrors;
        return;
    }
    if (!FNV_MUT(331)) ++seq_;   // twin 331: every frame of a burst carries the same sequence number
    ++stats_.frames;
    if (out) out->push_back(std::move(b));
}

void VoiceTransmitter::feed(const int16_t* pcm, bool keyDown, bool muted, std::vector<Bytes>* out) {
    std::vector<int16_t> frame(pcm, pcm + kFrameSamples);
    if (cfg_.micGain != 1.0f && !FNV_MUT(332)) {   // twin 332: the microphone gain is ignored
        for (int16_t& s : frame) {
            const float v = static_cast<float>(s) * cfg_.micGain;
            s = static_cast<int16_t>(std::clamp(v, -32768.0f, 32767.0f));
        }
    }
    level_ = frameLoudness(frame.data(), frame.size());

    bool want = false;
    if (!muted || FNV_MUT(333)) {   // twin 333: a muted microphone still goes on the air
        if (cfg_.mode == Mode::PushToTalk) want = keyDown;
        else want = keyDown || level_ >= cfg_.openThreshold;
    }
    if (muted) ++stats_.muted;
    const uint32_t holdFrames = cfg_.mode == Mode::PushToTalk ? cfg_.tailFrames : cfg_.hangFrames;

    if (!talking_) {
        if (!want) {
            if (cfg_.mode == Mode::OpenMic && cfg_.prerollFrames > 0) {
                preroll_.push_back(frame);
                while (preroll_.size() > cfg_.prerollFrames) preroll_.pop_front();
            }
            return;
        }
        talking_ = true;
        hold_ = holdFrames;
        ++stats_.bursts;
        uint8_t flags = kFlagStart;
        if (cfg_.mode == Mode::OpenMic && !FNV_MUT(334)) {   // twin 334: the first syllable of an open mic is cut off
            for (const auto& p : preroll_) {
                emit(p.data(), flags, out);
                flags = 0;
            }
        }
        preroll_.clear();
        emit(frame.data(), flags, out);
        return;
    }
    if (want) {
        hold_ = holdFrames;
        emit(frame.data(), 0, out);
        return;
    }
    // The talking stopped: the tail frames still go out, the last of them says so.
    if (muted && !FNV_MUT(335)) hold_ = 0;   // twin 335: muting in the middle of a sentence lets the tail run on
    if (hold_ > 0) --hold_;
    const bool last = hold_ == 0;
    emit(frame.data(), last ? kFlagEnd : 0, out);
    if (last) talking_ = false;
}

}  // namespace nvcoop::voice
