// Test rig for plugin/voice: a fake codec whose "speech" is a constant level (so a test can tell exactly which frame was heard),
// and a network of 1 host + guests that carries datagrams between VoiceSession objects.
#pragma once
#include <cmath>
#include <cstring>
#include <deque>
#include <memory>
#include <vector>

#include "voice_session.h"

namespace vt {

using namespace nvcoop::voice;

// Frame payload = [level lo][level hi][marker]. Decoding gives kFrameSamples samples of that level; a lost frame decodes to 0.
struct FakeCounters {
    int encodes = 0, decodes = 0, concealed = 0, resets = 0;
    int failEncode = 0;   // 1: encode returns 0
};

class FakeEncoder : public IVoiceEncoder {
public:
    explicit FakeEncoder(FakeCounters* c) : c_(c) {}
    int encode(const int16_t* pcm, uint8_t* out, size_t cap) override {
        ++c_->encodes;
        if (c_->failEncode || cap < 3) return 0;
        out[0] = static_cast<uint8_t>(pcm[0] & 0xFF);
        out[1] = static_cast<uint8_t>((pcm[0] >> 8) & 0xFF);
        out[2] = 0x5A;
        return 3;
    }
private:
    FakeCounters* c_;
};

class FakeDecoder : public IVoiceDecoder {
public:
    explicit FakeDecoder(FakeCounters* c) : c_(c) {}
    int decode(const uint8_t* data, size_t size, int16_t* pcm) override {
        ++c_->decodes;
        int16_t v = 0;
        if (!data) {
            ++c_->concealed;
        } else {
            if (size != 3) return -1;
            v = static_cast<int16_t>(data[0] | (data[1] << 8));
        }
        for (size_t i = 0; i < kFrameSamples; ++i) pcm[i] = v;
        return static_cast<int>(kFrameSamples);
    }
    void reset() override { ++c_->resets; }
private:
    FakeCounters* c_;
};

class FakeCodecs : public IVoiceCodecFactory {
public:
    std::unique_ptr<IVoiceEncoder> makeEncoder() override { return std::make_unique<FakeEncoder>(&counters); }
    std::unique_ptr<IVoiceDecoder> makeDecoder() override { return std::make_unique<FakeDecoder>(&counters); }
    FakeCounters counters;
};

inline std::vector<int16_t> level(int16_t v) { return std::vector<int16_t>(kFrameSamples, v); }

// A frame as the fake encoder makes it.
inline Bytes fakeFrame(int16_t v) { return Bytes{static_cast<uint8_t>(v & 0xFF), static_cast<uint8_t>((v >> 8) & 0xFF), 0x5A}; }

// One machine plus the datagrams waiting for it.
struct Machine {
    std::unique_ptr<VoiceSession> s;
    FakeCodecs codecs;
    std::vector<uint8_t> joined;   // host only: the guests' slots
};

// 1 host (slot 0) and N guests (slots 1..N), joined by a loop: host sends to slot k go to guest k; a guest's sends go to the host.
class Net {
public:
    struct Pkt { int from; int to; Bytes data; };
    explicit Net(int guests, const SessionConfig& base = {}) {
        for (int i = 0; i <= guests; ++i) {
            m_.emplace_back(new Machine());
            Machine& m = *m_.back();
            SessionConfig c = base;
            c.isHost = i == 0;
            c.self = static_cast<uint8_t>(i);
            if (i == 0) for (int g = 1; g <= guests; ++g) m.joined.push_back(static_cast<uint8_t>(g));
            const int me = i;
            m.s.reset(new VoiceSession(c, m.codecs,
                [this, me](uint8_t slot, const Bytes& b) { pending_.push_back({me, slot, b}); log.push_back({me, slot, b}); },
                [&m]() { return m.joined; }));
        }
    }
    Machine& at(int i) { return *m_[i]; }
    VoiceSession& s(int i) { return *m_[i]->s; }
    // Hands every waiting datagram to its receiver. The sender's slot as the receiver sees it: a guest only ever receives from slot 0.
    int deliver(uint64_t nowMs = 1000) {
        int n = 0;
        while (!pending_.empty()) {
            Pkt p = std::move(pending_.front());
            pending_.pop_front();
            if (drop_ && drop_(p.from, p.to, p.data)) continue;
            m_[p.to]->s->onDatagram(static_cast<uint8_t>(p.from), p.data.data(), p.data.size(), nowMs);
            ++n;
        }
        return n;
    }
    size_t waiting() const { return pending_.size(); }
    // Everything sent from `from` to `to` so far (kept for inspection when `keep` is on).
    std::function<bool(int from, int to, const Bytes&)> drop_;
    std::deque<Pkt>& pending() { return pending_; }
    std::vector<Pkt> log;   // every datagram ever sent

private:
    std::vector<std::unique_ptr<Machine>> m_;
    std::deque<Pkt> pending_;
};

// Renders `blocks` x kFrameSamples frames; returns the peak absolute left / right values of the last block (after any ramp).
struct Peak { int l = 0, r = 0; };
inline Peak renderBlocks(VoiceSession& s, int blocks, std::vector<int16_t>* all = nullptr) {
    Peak p;
    std::vector<int16_t> buf(kFrameSamples * 2);
    for (int b = 0; b < blocks; ++b) {
        s.render(buf.data(), kFrameSamples);
        if (all) all->insert(all->end(), buf.begin(), buf.end());
        p = Peak{};
        for (size_t i = 0; i < kFrameSamples; ++i) {
            p.l = std::max(p.l, std::abs(static_cast<int>(buf[i * 2])));
            p.r = std::max(p.r, std::abs(static_cast<int>(buf[i * 2 + 1])));
        }
    }
    return p;
}

inline Placement nearPlace(float pan = 0) {
    Placement p;
    p.audible = true;
    p.gain = 1.0f;
    p.pan = pan;
    return p;
}

}  // namespace vt
