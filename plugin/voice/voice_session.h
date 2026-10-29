// One machine's voice chat for one co-op session: sends its own microphone, relays on the host, and hears the others.
// Game-free: the datagrams go out through SendFn and come in through onDatagram(); the engine side (microphone, speakers, key,
// positions) is the module's job.
//
//   host   : receives a guest's frames (the speaker byte must be that guest's slot), plays them and relays them to the other
//            guests; its own frames go to every guest.
//   guest  : sends its frames to the host only; plays what the host sends (the host relays every other speaker).
//   Nobody hears their own voice: a frame whose speaker is this machine's own slot is dropped.
#pragma once
#include <functional>
#include <map>
#include <vector>

#include "voice_codec.h"
#include "voice_rx.h"
#include "voice_tx.h"
#include "voice_wire.h"

namespace nvcoop::voice {

// Spam limit for one player's voice (a normal talker sends 50 frames and a few KB a second).
class VoiceLimiter {
public:
    static constexpr uint32_t kPacketsPerSecond = 80;
    static constexpr uint32_t kBytesPerSecond = 24000;
    bool allow(uint64_t nowMs, size_t bytes);

private:
    uint64_t second_ = 0;
    uint32_t packets_ = 0, bytes_ = 0;
    bool started_ = false;
};

struct SessionConfig {
    bool isHost = false;
    uint8_t self = 0;             // this machine's slot
    TxConfig tx;
    JitterConfig jitter;
};

struct SessionStats {
    uint64_t sent = 0, received = 0, relayed = 0, forged = 0, ownEcho = 0, malformed = 0, wrongSource = 0, limited = 0, played = 0;
};

class VoiceSession {
public:
    // sendTo(slot, datagram): host: any guest slot; guest: slot 0 (the host). clients(): host: the joined guests' slots.
    using SendFn = std::function<void(uint8_t slot, const Bytes& datagram)>;
    using SlotsFn = std::function<std::vector<uint8_t>()>;

    VoiceSession(const SessionConfig& cfg, IVoiceCodecFactory& codecs, SendFn send, SlotsFn clients);

    // A datagram from the transport peer that is `fromSlot` (the host sees guests' slots; a guest only ever hears slot 0).
    void onDatagram(uint8_t fromSlot, const uint8_t* data, size_t size, uint64_t nowMs);
    // One 20 ms frame from the microphone (kFrameSamples samples).
    void feedMic(const int16_t* pcm, bool keyDown, bool selfMuted);
    // The speaker side (the audio thread calls render under the caller's lock).
    void render(int16_t* stereo, size_t frames) { rx_.render(stereo, frames); }

    VoiceReceiver& rx() { return rx_; }
    const VoiceReceiver& rx() const { return rx_; }
    VoiceTransmitter& tx() { return tx_; }
    const VoiceTransmitter& tx() const { return tx_; }
    const SessionStats& stats() const { return stats_; }
    uint8_t self() const { return cfg_.self; }
    bool isHost() const { return cfg_.isHost; }
    void playerLeft(uint8_t slot);

private:
    SessionConfig cfg_;
    IVoiceCodecFactory& codecs_;
    SendFn send_;
    SlotsFn clients_;
    std::unique_ptr<IVoiceEncoder> enc_;
    VoiceReceiver rx_;
    VoiceTransmitter tx_;
    std::map<uint8_t, VoiceLimiter> limiters_;
    SessionStats stats_;
};

}  // namespace nvcoop::voice
