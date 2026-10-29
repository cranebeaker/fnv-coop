#include "voice_session.h"

#include "fnvcoop_mut.h"

namespace nvcoop::voice {

namespace {
class NullEncoder : public IVoiceEncoder {
public:
    int encode(const int16_t*, uint8_t*, size_t) override { return 0; }
};
NullEncoder g_null;
}  // namespace

bool VoiceLimiter::allow(uint64_t nowMs, size_t bytes) {
    if (!started_ || nowMs - second_ >= 1000) {
        started_ = true;
        second_ = nowMs;
        packets_ = 0;
        bytes_ = 0;
    }
    if (FNV_MUT(351)) return true;   // twin 351: no limit, one guest can flood the host and everybody it relays to
    if (packets_ >= kPacketsPerSecond || bytes_ + bytes > kBytesPerSecond) return false;
    ++packets_;
    bytes_ += static_cast<uint32_t>(bytes);
    return true;
}

VoiceSession::VoiceSession(const SessionConfig& cfg, IVoiceCodecFactory& codecs, SendFn send, SlotsFn clients)
    : cfg_(cfg),
      codecs_(codecs),
      send_(std::move(send)),
      clients_(std::move(clients)),
      enc_(codecs.makeEncoder()),
      rx_(codecs, cfg.jitter),
      tx_(cfg.tx, enc_ ? *enc_ : static_cast<IVoiceEncoder&>(g_null), cfg.self) {}

void VoiceSession::feedMic(const int16_t* pcm, bool keyDown, bool selfMuted) {
    std::vector<Bytes> out;
    tx_.feed(pcm, keyDown, selfMuted, &out);
    for (const Bytes& b : out) {
        ++stats_.sent;
        if (cfg_.isHost) {
            if (clients_) for (uint8_t slot : clients_()) if (slot != cfg_.self && send_) send_(slot, b);
        } else if (send_) {
            send_(0, b);
        }
    }
}

void VoiceSession::onDatagram(uint8_t fromSlot, const uint8_t* data, size_t size, uint64_t nowMs) {
    VoiceFrame f;
    if (!decodeVoice(data, size, &f)) {
        ++stats_.malformed;
        return;
    }
    if (cfg_.isHost) {
        // A guest may only speak as itself.
        if (f.speaker != fromSlot && !FNV_MUT(352)) {   // twin 352: a guest can speak in another player's name
            ++stats_.forged;
            return;
        }
        if (fromSlot == cfg_.self) return;
        if (!limiters_[fromSlot].allow(nowMs, size)) {
            ++stats_.limited;
            return;
        }
    } else if (fromSlot != 0 && !FNV_MUT(353)) {   // twin 353: a guest takes voice from anybody but the host
        ++stats_.wrongSource;
        return;
    }
    if (f.speaker == cfg_.self && !FNV_MUT(354)) {   // twin 354: your own voice comes back to you
        ++stats_.ownEcho;
        return;
    }
    ++stats_.received;
    rx_.push(f.speaker, f.seq, f.flags, f.payload, f.size);
    if (cfg_.isHost && clients_ && !FNV_MUT(355)) {   // twin 355: the host plays a guest's voice but never relays it
        Bytes raw(data, data + size);
        for (uint8_t slot : clients_()) {
            if ((slot == fromSlot && !FNV_MUT(356)) || slot == cfg_.self) continue;   // twin 356: not back to the speaker
            if (send_) send_(slot, raw);
            ++stats_.relayed;
        }
    }
}

void VoiceSession::playerLeft(uint8_t slot) {
    rx_.forget(slot);
    limiters_.erase(slot);
}

}  // namespace nvcoop::voice
