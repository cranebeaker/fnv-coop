// Windows sound for voice chat: the microphone (waveIn) and the speakers (waveOut), each on a thread of its own (nvreload::ManagedThread,
// so a hot reload can stop them). Only the system's own audio API; the game's sound engine is not touched. 16 kHz, 16 bit, 20 ms blocks.
// Nothing here has run in a game.
#pragma once
#include <atomic>
#include <functional>

#include "voice_pcm_queue.h"
#include "reload/reload_state.h"

namespace nvcoop::voice {

// Microphone: pushes captured samples into a PcmQueue.
class AudioCapture {
public:
    ~AudioCapture() { stop(); }
    // device -1 = the system default. Returns once the thread is started; opening the device can still fail later (failed()).
    void start(int device, PcmQueue* sink);
    void stop();
    bool failed() const { return failed_.load(); }
    bool running() const { return running_.load(); }

private:
    nvreload::ManagedThread thread_;
    std::atomic<bool> failed_{false}, running_{false};
};

// Speakers: asks `render` for each 20 ms block (stereo, interleaved) on the audio thread.
class AudioPlayback {
public:
    using RenderFn = std::function<void(int16_t* stereo, size_t frames)>;
    ~AudioPlayback() { stop(); }
    void start(int device, RenderFn render);
    void stop();
    bool failed() const { return failed_.load(); }
    bool running() const { return running_.load(); }

private:
    nvreload::ManagedThread thread_;
    std::atomic<bool> failed_{false}, running_{false};
};

}  // namespace nvcoop::voice
