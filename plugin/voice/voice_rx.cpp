#include "voice_rx.h"

#include <algorithm>
#include <cmath>
#include <vector>

#include "fnvcoop_mut.h"

namespace nvcoop::voice {

namespace {
constexpr float kPi4 = 0.78539816339f;
constexpr uint32_t kTalkingSamples = kSampleRate * 300 / 1000;
// One-pole smoothing of the gains: a time constant of 40 ms, so a voice that moves or fades never clicks.
const float kSlew = 1.0f - std::exp(-1.0f / (static_cast<float>(kSampleRate) * 0.04f));
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
    return s ? 0.5f * (s->gl + s->gr) : 0.0f;
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
    const bool heard = heardNow || s.gl > 1e-4f || s.gr > 1e-4f;
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
        s.pcm.insert(s.pcm.end(), buf, buf + kFrameSamples);
    }
}

void VoiceReceiver::render(int16_t* stereo, size_t frames) {
    if (!stereo || frames == 0) return;
    std::vector<float> sum(frames * 2, 0.0f);
    for (auto& kv : speakers_) {
        Speaker& s = kv.second;
        const Placement& place = place_[kv.first];
        const bool audible = place.audible && place.gain > 0 && !muted_[kv.first];
        fill(s, frames, audible);
        float tl = 0, tr = 0;
        if (audible || FNV_MUT(343)) {   // twin 343: a muted speaker is only turned down by the placement, not by the mute
            const float angle = (std::clamp(place.pan, -1.0f, 1.0f) + 1.0f) * kPi4;   // equal power
            tl = master_ * place.gain * std::cos(angle);
            tr = master_ * place.gain * std::sin(angle);
        }
        const size_t have = std::min(frames, s.pcm.size());
        for (size_t i = 0; i < frames; ++i) {
            if (FNV_MUT(341)) {   // twin 341: the gains jump to the target (clicks when a voice moves or fades)
                s.gl = tl;
                s.gr = tr;
            } else {
                s.gl += (tl - s.gl) * kSlew;
                s.gr += (tr - s.gr) * kSlew;
            }
            const float x = i < have ? static_cast<float>(s.pcm[i]) : 0.0f;
            sum[i * 2] += x * s.gl;
            sum[i * 2 + 1] += x * s.gr;
        }
        s.pcm.erase(s.pcm.begin(), s.pcm.begin() + static_cast<std::ptrdiff_t>(have));
        if (have > 0) s.quiet = 0;
        else s.quiet = s.quiet > (1u << 29) ? s.quiet : s.quiet + static_cast<uint32_t>(frames);
    }
    for (size_t i = 0; i < frames * 2; ++i) {
        const float v = FNV_MUT(344) ? sum[i] : std::clamp(sum[i], -32768.0f, 32767.0f);   // twin 344: a loud mix wraps around
        stereo[i] = static_cast<int16_t>(v);
    }
}

}  // namespace nvcoop::voice
