// Windows sound for voice chat on WASAPI (shared mode): the microphone and the speakers, each on a thread of its own
// (nvreload::ManagedThread, so a hot reload can stop them). Only the system's own audio API; the game's sound engine is not touched.
// The codec side is 16 kHz, 16 bit, 20 ms blocks; the devices run at their own rate and format and are converted (voice_resample.h).
// A device that is missing, unplugged or reset does not crash or end the thread: it is reported through failed() and opened again
// as soon as it is back. Nothing here has run in a game.
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
    // device -1 = the system default, otherwise the position in the list of active microphones (the log names them on start).
    // Returns once the thread is started; opening the device can still fail later (failed()), and is retried every two seconds.
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
    // device: as for AudioCapture, but counted in the list of active speakers.
    void start(int device, RenderFn render);
    void stop();
    bool failed() const { return failed_.load(); }     // no working device right now
    bool running() const { return running_.load(); }   // a device is open and being served

private:
    nvreload::ManagedThread thread_;
    std::atomic<bool> failed_{false}, running_{false};
};

}  // namespace nvcoop::voice
