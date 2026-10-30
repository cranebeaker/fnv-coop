#include "voice_rx.h"

#include <algorithm>
#include <cmath>
#include <vector>

#include "fnvcoop_mut.h"

namespace nvcoop::voice {

namespace {
constexpr uint32_t kTalkingSamples = kSampleRate * 300 / 1000;
// The gains of a voice glide with a time constant of 40 ms, so a voice that moves or fades never clicks.
const float kGlide = glideCoefficient(40.0f, static_cast<float>(kSampleRate));
}  // namespace

VoiceReceiver::Speaker* VoiceReceiver::find(uint8_t who) {
    auto it = speakers_.find(who);
    return it == speakers_.end() ? nullptr : &it->second;
}
const VoiceReceiver::Speaker* VoiceReceiver::find(uint8_t who) const {
    auto it = speakers_.find(who);
    return it == speakers_.end() ? nullptr : &it->second;
}

PushResult VoiceReceiver::push(uint8_t who, uint16_t seq, uint8_t flags, const uint8_t* data, size_t size) {
    Speaker* s = find(who);
    if (!s) {
        if (who >= kMaxSlots || speakers_.size() >= kMaxSlots) return PushResult::Late;
        auto dec = codecs_.makeDecoder();
        if (!dec) return PushResult::Late;
        s = &speakers_.emplace(who, Speaker(jitterCfg_)).first->second;
        s->decoder = std::move(dec);
    }
    const PushResult r = s->jitter.push(seq, flags, data, size);
    if (r == PushResult::Queued) ++stats_.framesIn;
    else if (r == PushResult::Duplicate) ++stats_.duplicates;
    else ++stats_.late;
    return r;
}

void VoiceReceiver::setPlacement(uint8_t who, const Placement& p) {
    if (who < kMaxSlots) place_[who] = p;
}

void VoiceReceiver::setMuted(uint8_t who, bool muted) {
    if (who < kMaxSlots) muted_[who] = muted;
}

bool VoiceReceiver::muted(uint8_t who) const { return who < kMaxSlots && muted_[who]; }

bool VoiceReceiver::talking(uint8_t who) const {
    const Speaker* s = find(who);
    return s && s->quiet < kTalkingSamples;
}

float VoiceReceiver::gainOf(uint8_t who) const {
    const Speaker* s = find(who);
    return s ? 0.5f * (s->gain.l + s->gain.r) : 0.0f;
}

size_t VoiceReceiver::depthOf(uint8_t who) const {
    const Speaker* s = find(who);
    return s ? s->jitter.depth() : 0;
}

const JitterStats* VoiceReceiver::jitterOf(uint8_t who) const {
    const Speaker* s = find(who);
    return s ? &s->jitter.stats() : nullptr;
}

// Takes frames out of the jitter buffer until `frames` samples are ready (or the stream has nothing more to give).
void VoiceReceiver::fill(Speaker& s, size_t frames, bool heardNow) {
    const bool heard = heardNow || s.gain.l > 1e-4f || s.gain.r > 1e-4f;
    while (s.pcm.size() < frames) {
        JitterOut o = s.jitter.pop();
        if (o.kind == PopKind::None) break;
        int16_t buf[kFrameSamples] = {};
        if (heard || FNV_MUT(342)) {   // twin 342: voices nobody can hear are decoded anyway
            const bool lost = o.kind == PopKind::Lost;
            const int n = s.decoder->decode(lost ? nullptr : o.payload.data(), o.payload.size(), buf);
            if (n <= 0) {
                ++stats_.decodeErrors;
                std::fill(buf, buf + kFrameSamples, int16_t(0));
            }
            if (lost) ++stats_.concealed;
        } else {
            ++stats_.droppedInaudible;   // still taken out of the buffer, so the stream keeps its time
        }
        if (o.last) s.decoder->reset();
        s.pcm.push(buf, kFrameSamples);
    }
}

void VoiceReceiver::render(int16_t* stereo, size_t frames) {
    if (!stereo || frames == 0) return;
    bus_.begin(frames);
    block_.resize(frames);
    for (auto& entry : speakers_) {
        const uint8_t who = entry.first;
        Speaker& s = entry.second;
        const Placement& place = place_[who];
        const bool audible = place.audible && place.gain > 0 && !muted_[who];
        fill(s, frames, audible);
        float targetL = 0, targetR = 0;
        if (audible || FNV_MUT(343)) {   // twin 343: a muted speaker is only turned down by the placement, not by the mute
            panGains(master_ * place.gain, place.pan, targetL, targetR);
        }
        const size_t have = s.pcm.take(block_.data(), frames);
        bus_.addVoice(block_.data(), have, s.gain, targetL, targetR, kGlide);
        if (have > 0) s.quiet = 0;
        else s.quiet = s.quiet > (1u << 29) ? s.quiet : s.quiet + static_cast<uint32_t>(frames);
    }
    bus_.finish(stereo);
}

}  // namespace nvcoop::voice
