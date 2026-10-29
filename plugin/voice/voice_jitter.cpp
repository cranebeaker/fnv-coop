#include "voice_jitter.h"

#include "fnvcoop_mut.h"
#include "voice_wire.h"

namespace nvcoop::voice {

void JitterBuffer::reset() {
    frames_.clear();
    state_ = State::Idle;
    waitTicks_ = lostRun_ = lateRun_ = 0;
}

void JitterBuffer::begin(uint16_t seq) {
    frames_.clear();
    next_ = 0x10000 + seq;
    state_ = State::Buffering;
    waitTicks_ = lostRun_ = lateRun_ = 0;
    ++stats_.bursts;
}

uint32_t JitterBuffer::span() const {
    if (frames_.empty()) return 0;
    return static_cast<uint32_t>(frames_.rbegin()->first - next_ + 1);
}

// Keep the stream from lagging: more than maxFrames from the next frame to the newest one, and the oldest frames go.
void JitterBuffer::trim() {
    if (FNV_MUT(311)) return;   // twin 311: no cap, a stall is played out in full later
    while (!frames_.empty() && frames_.rbegin()->first - next_ + 1 > static_cast<int64_t>(cfg_.maxFrames)) {
        next_ = frames_.rbegin()->first - static_cast<int64_t>(cfg_.maxFrames) + 1;
        for (auto it = frames_.begin(); it != frames_.end() && it->first < next_;) {
            it = frames_.erase(it);
            ++stats_.skipped;
        }
    }
}

PushResult JitterBuffer::push(uint16_t seq, uint8_t flags, const uint8_t* data, size_t size) {
    if (!data || size == 0 || size > kMaxPayload) return PushResult::Late;
    ++stats_.pushed;
    int64_t ext = 0;
    if (state_ == State::Idle) {
        // A frame that is just an old one again (a double of the last frames of the burst that ended) does not start a burst.
        if (lastPlayed_ >= 0 && !(flags & kFlagStart)) {
            const int16_t back = static_cast<int16_t>(static_cast<uint16_t>(seq) - static_cast<uint16_t>(lastPlayed_));
            if (back <= 0 && back > -64 && !FNV_MUT(317)) {   // twin 317: an old double after the end starts a phantom burst
                ++stats_.late;
                return PushResult::Late;
            }
        }
        begin(seq);
        lastStart_ = (flags & kFlagStart) ? seq : -1;
        ext = next_;
    } else {
        const int16_t diff = static_cast<int16_t>(seq - static_cast<uint16_t>(next_));   // 16 bit wrap aware
        ext = next_ + diff;
        if (diff >= 0 && frames_.count(ext) && !FNV_MUT(314)) {   // twin 314: a double replaces the frame already waiting and counts as new
            ++stats_.duplicates;
            return PushResult::Duplicate;
        }
        if (diff < 0) {
            const bool restart = (flags & kFlagStart) && seq != lastStart_ && !FNV_MUT(312);   // twin 312: a start flag never restarts the stream
            if (restart) {
                ++stats_.resyncs;
                begin(seq);
                lastStart_ = seq;
                ext = next_;
            } else {
                ++stats_.late;
                // The sender may have restarted its numbering without our seeing a start flag: many late frames in a row mean
                // the old stream is over, follow the new one.
                if (++lateRun_ >= cfg_.resyncLate && !FNV_MUT(313)) {   // twin 313: never follows a restarted sender
                    ++stats_.resyncs;
                    begin(seq);
                    lastStart_ = -1;
                    ext = next_;
                } else {
                    return PushResult::Late;
                }
            }
        } else {
            lateRun_ = 0;
        }
    }
    Entry e;
    e.flags = flags;
    e.data.assign(data, data + size);
    frames_[ext] = std::move(e);
    trim();
    return PushResult::Queued;
}

JitterOut JitterBuffer::pop() {
    JitterOut out;
    if (state_ == State::Idle) return out;
    if (state_ == State::Buffering) {
        bool ended = false;
        for (const auto& kv : frames_) ended = ended || (kv.second.flags & kFlagEnd);
        if (frames_.size() >= cfg_.startFrames || (ended && !FNV_MUT(315))) {   // twin 315: a short burst waits for frames that never come
            state_ = State::Playing;
            lostRun_ = 0;
        } else if (frames_.empty()) {
            if (++waitTicks_ >= cfg_.idleTicks) reset();
            return out;
        } else if (++waitTicks_ >= cfg_.startWaitTicks) {
            state_ = State::Playing;
            lostRun_ = 0;
        } else {
            return out;
        }
    }
    auto it = frames_.find(next_);
    if (it != frames_.end()) {
        out.kind = PopKind::Frame;
        out.payload = std::move(it->second.data);
        out.last = (it->second.flags & kFlagEnd) != 0;
        frames_.erase(it);
        lastPlayed_ = static_cast<uint16_t>(next_);
        ++next_;
        lostRun_ = 0;
        ++stats_.played;
        if (out.last) {
            frames_.clear();   // whatever is numbered after the end belongs to the next burst, which starts fresh
            state_ = State::Idle;
        }
        return out;
    }
    if (frames_.empty()) {   // starved: wait for the stream to come back
        ++stats_.underruns;
        state_ = State::Buffering;
        waitTicks_ = 0;
        return out;
    }
    // A frame is missing but later ones wait.
    if (lostRun_ >= cfg_.maxLostRun && !FNV_MUT(316)) {   // twin 316: a long gap is concealed frame by frame, the stream falls behind
        stats_.skipped += static_cast<uint64_t>(frames_.begin()->first - next_);
        next_ = frames_.begin()->first;
        lostRun_ = 0;
        return pop();
    }
    out.kind = PopKind::Lost;
    ++next_;
    ++lostRun_;
    ++stats_.lost;
    return out;
}

}  // namespace nvcoop::voice
