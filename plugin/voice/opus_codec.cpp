// Opus behind IVoiceEncoder / IVoiceDecoder (third_party/opus, BSD-3-Clause).
#include <opus.h>

#include "voice_codec.h"

namespace nvcoop::voice {

namespace {

class OpusVoiceEncoder : public IVoiceEncoder {
public:
    explicit OpusVoiceEncoder(int bitrate) {
        int err = 0;
        enc_ = opus_encoder_create(static_cast<opus_int32>(kSampleRate), 1, OPUS_APPLICATION_VOIP, &err);
        if (err != OPUS_OK) {
            enc_ = nullptr;
            return;
        }
        opus_encoder_ctl(enc_, OPUS_SET_BITRATE(bitrate));
        opus_encoder_ctl(enc_, OPUS_SET_VBR(1));
        opus_encoder_ctl(enc_, OPUS_SET_COMPLEXITY(5));
        opus_encoder_ctl(enc_, OPUS_SET_SIGNAL(OPUS_SIGNAL_VOICE));
        opus_encoder_ctl(enc_, OPUS_SET_DTX(0));
        opus_encoder_ctl(enc_, OPUS_SET_PACKET_LOSS_PERC(8));
        opus_encoder_ctl(enc_, OPUS_SET_INBAND_FEC(0));
    }
    ~OpusVoiceEncoder() override {
        if (enc_) opus_encoder_destroy(enc_);
    }
    bool ok() const { return enc_ != nullptr; }
    int encode(const int16_t* pcm, uint8_t* out, size_t cap) override {
        if (!enc_ || !pcm || !out || cap == 0) return 0;
        return opus_encode(enc_, pcm, static_cast<int>(kFrameSamples), out, static_cast<opus_int32>(cap));
    }

private:
    OpusEncoder* enc_ = nullptr;
};

class OpusVoiceDecoder : public IVoiceDecoder {
public:
    OpusVoiceDecoder() {
        int err = 0;
        dec_ = opus_decoder_create(static_cast<opus_int32>(kSampleRate), 1, &err);
        if (err != OPUS_OK) dec_ = nullptr;
    }
    ~OpusVoiceDecoder() override {
        if (dec_) opus_decoder_destroy(dec_);
    }
    bool ok() const { return dec_ != nullptr; }
    int decode(const uint8_t* data, size_t size, int16_t* pcm) override {
        if (!dec_ || !pcm) return 0;
        return opus_decode(dec_, data, data ? static_cast<opus_int32>(size) : 0, pcm, static_cast<int>(kFrameSamples), 0);
    }
    void reset() override {
        if (dec_) opus_decoder_ctl(dec_, OPUS_RESET_STATE);
    }

private:
    OpusDecoder* dec_ = nullptr;
};

class OpusFactory : public IVoiceCodecFactory {
public:
    explicit OpusFactory(int bitrate) : bitrate_(bitrate) {}
    std::unique_ptr<IVoiceEncoder> makeEncoder() override {
        auto e = std::make_unique<OpusVoiceEncoder>(bitrate_);
        if (!e->ok()) return nullptr;
        return e;
    }
    std::unique_ptr<IVoiceDecoder> makeDecoder() override {
        auto d = std::make_unique<OpusVoiceDecoder>();
        if (!d->ok()) return nullptr;
        return d;
    }

private:
    int bitrate_;
};

}  // namespace

std::unique_ptr<IVoiceCodecFactory> makeOpusFactory(int bitrate) {
    if (bitrate < 6000) bitrate = 6000;
    if (bitrate > 64000) bitrate = 64000;
    auto f = std::make_unique<OpusFactory>(bitrate);
    if (!f->makeEncoder() || !f->makeDecoder()) return nullptr;   // the library must start before anything depends on it
    return f;
}

}  // namespace nvcoop::voice
