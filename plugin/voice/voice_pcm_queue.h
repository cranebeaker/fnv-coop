// Hand-over of microphone samples from the capture thread to the game thread. Thread safe, bounded: if the game thread stalls
// (a loading screen) the oldest audio is dropped, so the first words after the stall are not seconds late.
#pragma once
#include <algorithm>
#include <deque>
#include <mutex>

#include "fnvcoop_mut.h"
#include "voice_types.h"

namespace nvcoop::voice {

class PcmQueue {
public:
    explicit PcmQueue(size_t capSamples = kSampleRate / 5) : cap_(capSamples) {}   // 200 ms

    void push(const int16_t* samples, size_t n) {
        if (!samples || n == 0) return;
        std::lock_guard<std::mutex> lock(m_);
        q_.insert(q_.end(), samples, samples + n);
        if (FNV_MUT(371)) return;   // twin 371: no bound, a stall is played back late
        if (q_.size() > cap_) {
            dropped_ += q_.size() - cap_;
            // drop whole frames from the front so the frames that remain stay aligned
            size_t drop = q_.size() - cap_;
            drop = (drop + kFrameSamples - 1) / kFrameSamples * kFrameSamples;
            drop = (std::min)(drop, q_.size());
            q_.erase(q_.begin(), q_.begin() + static_cast<std::ptrdiff_t>(drop));
        }
    }
    // One frame (kFrameSamples) into `out`; false when less than a frame is waiting.
    bool popFrame(int16_t* out) {
        std::lock_guard<std::mutex> lock(m_);
        if (q_.size() < kFrameSamples) return false;
        std::copy(q_.begin(), q_.begin() + kFrameSamples, out);
        q_.erase(q_.begin(), q_.begin() + kFrameSamples);
        return true;
    }
    size_t size() const {
        std::lock_guard<std::mutex> lock(m_);
        return q_.size();
    }
    uint64_t dropped() const {
        std::lock_guard<std::mutex> lock(m_);
        return dropped_;
    }
    void clear() {
        std::lock_guard<std::mutex> lock(m_);
        q_.clear();
    }

private:
    mutable std::mutex m_;
    std::deque<int16_t> q_;
    size_t cap_;
    uint64_t dropped_ = 0;
};

}  // namespace nvcoop::voice
