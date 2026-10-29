// Jitter buffer of one speaker's voice stream. Game-free.
//
// Frames come over the network unevenly, out of order, doubled or not at all; the ear wants one frame every 20 ms.
// The buffer waits a little before a burst starts (startFrames), puts frames back in order, drops doubles and frames that came
// too late, reports a missing frame as Lost (the decoder then conceals it) and NEVER lets the stream lag more than maxFrames
// behind the newest frame: past that the oldest frames are skipped, so a stall on the network is not heard as a delay later.
#pragma once
#include <cstdint>
#include <map>

#include "voice_types.h"

namespace nvcoop::voice {

struct JitterConfig {
    uint32_t startFrames = 3;       // frames to hold before a burst starts playing (60 ms)
    uint32_t maxFrames = 12;        // the stream spans at most this many frames from the next one to play to the newest (240 ms)
    uint32_t maxLostRun = 3;        // this many missing frames in a row: the gap is skipped instead of concealed further
    uint32_t startWaitTicks = 10;   // a burst whose end never came plays after this many pop() calls even when short
    uint32_t idleTicks = 25;        // an empty, starved stream goes idle after this many pop() calls
    uint32_t resyncLate = 8;        // this many late frames in a row: the sender restarted its numbers, follow it
};

enum class PushResult { Queued, Duplicate, Late };

enum class PopKind {
    None,    // nothing to play now (idle, waiting to start, or starved)
    Frame,   // `payload` is the next frame
    Lost,    // the next frame is missing: conceal it
};

struct JitterOut {
    PopKind kind = PopKind::None;
    Bytes payload;
    bool last = false;   // this frame ends the burst (the stream is idle after it)
};

struct JitterStats {
    uint64_t pushed = 0, played = 0, duplicates = 0, late = 0, skipped = 0, lost = 0, underruns = 0, resyncs = 0, bursts = 0;
};

class JitterBuffer {
public:
    explicit JitterBuffer(JitterConfig cfg = {}) : cfg_(cfg) {}

    PushResult push(uint16_t seq, uint8_t flags, const uint8_t* data, size_t size);
    JitterOut pop();                         // call once per 20 ms of playback
    void reset();
    bool idle() const { return state_ == State::Idle; }
    size_t depth() const { return frames_.size(); }              // frames waiting
    uint32_t span() const;                   // frames from the next one to play to the newest one waiting (0 when empty)
    const JitterStats& stats() const { return stats_; }

private:
    enum class State { Idle, Buffering, Playing };
    struct Entry { uint8_t flags = 0; Bytes data; };
    void begin(uint16_t seq);
    void trim();

    JitterConfig cfg_;
    State state_ = State::Idle;
    std::map<int64_t, Entry> frames_;    // by unwrapped sequence number
    int64_t next_ = 0;                   // the sequence number to play next
    uint32_t waitTicks_ = 0, lostRun_ = 0, lateRun_ = 0;
    int32_t lastStart_ = -1;             // sequence number of the start frame of the burst now playing (-1: none)
    int32_t lastPlayed_ = -1;            // sequence number (16 bit) of the last frame played, kept across bursts (-1: none yet)
    JitterStats stats_;
};

}  // namespace nvcoop::voice
